// The equipment models on screen. One WebGL2 context for the whole page, and
// every card gets a picture out of it.
//
// WHY ONE CONTEXT. The equipment catalogue draws forty-odd cards at once and a
// browser gives a page about sixteen live WebGL contexts before it starts
// dropping the oldest — a canvas-per-card design does not fail on a small
// screen and then fails on a big one, which is the worst way for a thing to
// fail. So there is a single hidden GL canvas, models are drawn into it one at
// a time, and each card's ordinary 2D canvas takes a `drawImage` copy. A card
// that is never redrawn costs nothing after its first frame.
//
// WHY A CARD IS STILL AND A HOVERED CARD SPINS. A grid of forty spinning models
// is a slot machine, and the panel it sits in is where the player spends money.
// Everything rests at the same three-quarter view — which is the view that
// shows a planform AND a profile — and only the one under the pointer turns.
//
// The geometry is arsenal-models.js's and this file does not know what any of
// it is: it asks for an id, gets buffers and a bounding box back, and frames
// them. That is the whole contract.
(function (root) {
  "use strict";

  const MAX_DPR = 2;
  const REST_YAW = 34;      // degrees, front-three-quarter from the starboard bow
  const REST_PITCH = 20;    // looking down, which is how a map looks at things
  const FOV = 26;           // narrow: a long lens flatters a small object

  const VERT = `#version 300 es
  in vec3 aPos; in vec3 aNrm; in vec3 aCol;
  uniform mat4 uMVP;
  out vec3 vNrm; out vec3 vCol; out vec3 vPos;
  void main() {
    vNrm = aNrm; vCol = aCol; vPos = aPos;
    gl_Position = uMVP * vec4(aPos, 1.0);
  }`;

  // Two-sided on purpose. These meshes are open in a few places by design — a
  // rotodome is a disc, a solar array is a sheet, a wing is a plate — and a
  // sheet lit from behind should be lit, not black.
  const FRAG_TEMPLATE = `#version 300 es
  precision highp float;
  in vec3 vNrm; in vec3 vCol; in vec3 vPos;
  uniform vec3 uEye;
  uniform float uHeight;
  uniform float uCharacter;
  __SURFACE__
  out vec4 outColor;
  void main() {
    vec3 N = normalize(vNrm);
    vec3 V = normalize(uEye - vPos);
    if (dot(N, V) < 0.0) N = -N;
    if (uCharacter > 0.5) {
      // A broad portrait key and warm fill keep skin soft and tailoring matte.
      // Equipment retains its own harder lighting and installed surface shader.
      vec3 portraitKey = normalize(vec3(-0.55, 0.7, 1.0));
      vec3 portraitFill = normalize(vec3(0.75, 0.2, 0.7));
      float hemisphere = 0.5 + 0.5 * N.y;
      vec3 c = vCol * (0.34 + 0.10 * hemisphere);
      c += vCol * (0.52 * max(dot(N, portraitKey), 0.0));
      c += vCol * (0.19 * max(dot(N, portraitFill), 0.0)) * vec3(1.0, 0.96, 0.91);
      float rim = pow(1.0 - max(dot(N, V), 0.0), 3.0);
      c += rim * 0.055 * vec3(0.74, 0.84, 0.92);
      c += 0.018 * pow(max(dot(N, normalize(portraitKey + V)), 0.0), 32.0);
      outColor = vec4(c, 1.0);
      return;
    }
    vec3 key = normalize(vec3(-0.45, 0.82, 0.55));
    vec3 fil = normalize(vec3(0.7, 0.15, -0.5));
    float sky = 0.5 + 0.5 * N.y;
    // AMBIENT IS THE ENEMY OF A BEVEL. The models carry chamfers, panel lines
    // and weld beads now, and none of them read if a flat ambient term is doing
    // most of the lighting: a 47,000-triangle tank came out looking like a
    // 5,000-triangle one. Ambient is down and the key is up, which costs
    // nothing and is what makes the geometry visible at all.
    // The albedo a fragment shades with, after whatever surface treatment is
    // installed. The default is the identity, so an uninstalled renderer is
    // byte-for-byte the one that shipped.
    vec3 alb = uCharacter > 0.5 ? vCol : surface(vCol, N, vPos, V);
    vec3 c = alb * (0.19 + 0.19 * sky);
    c += alb * max(dot(N, key), 0.0) * 1.06;
    c += alb * max(dot(N, fil), 0.0) * 0.20 * vec3(0.75, 0.85, 1.0);
    // Contact darkening. Not real occlusion — there is no neighbour
    // information here — but everything in this deck stands on y=0, so the
    // bottom of a hull IS the shadowed part, and grounding it stops a vehicle
    // looking like it is floating over its own tracks.
    float low = 1.0 - smoothstep(0.0, max(uHeight, 0.001) * 0.42, vPos.y);
    c *= 1.0 - 0.22 * low;
    float rim = pow(1.0 - max(dot(N, V), 0.0), 3.0);
    c += rim * 0.34 * vec3(0.62, 0.76, 0.95);
    // Specular by brightness rather than by material, because there is no
    // material channel: rubber, track and tyre are the dark colours in every
    // palette here and they should stay matte, while steel, glass and lens are
    // the bright ones and should catch a highlight.
    // vCol, not alb, and deliberately: this asks WHAT MATERIAL THIS IS, and
    // the untouched palette value is the answer. A surface treatment that
    // muddies a steel panel has not turned the steel into rubber, so the
    // highlight should not go with the dirt.
    float lum = dot(vCol, vec3(0.299, 0.587, 0.114));
    float spec = pow(max(dot(reflect(-key, N), V), 0.0), 42.0);
    c += spec * (0.06 + 0.34 * lum);
    outColor = vec4(c, 1.0);
  }`;

  /// SURFACE TREATMENT, INSTALLABLE AT RUNTIME.
  ///
  /// This renderer has no textures and no UVs and is not getting either: the
  /// generators emit position, normal and colour, and an atlas would cost the
  /// megabytes that the whole procedural approach exists to avoid. Surface
  /// detail is therefore CODE spliced into the fragment shader — 3D noise read
  /// from model-space position, which needs no parameterisation at all.
  ///
  /// It is swappable at runtime because the only honest way to choose between
  /// treatments is to look at them on the same mesh in the same frame.
  const DEFAULT_SURFACE = "vec3 surface(vec3 albedo, vec3 N, vec3 P, vec3 V) { return albedo; }";
  let surfaceGlsl = DEFAULT_SURFACE;
  function fragSource() { return FRAG_TEMPLATE.replace("__SURFACE__", surfaceGlsl); }

  let gl = null, prog = null, uMVP = null, uEye = null, uHeight = null, uCharacter = null, glCanvas = null;
  let lost = false;
  const sprites = new Map();
  let available = null;
  /// THE MODEL CACHE IS BOUNDED, AND IT HAS TO BE.
  ///
  /// Every distinct id ever mounted used to keep its GPU buffers for the life
  /// of the page, and the ONLY thing that ever cleared them was losing the
  /// context — which is to say the eviction policy was the failure. Measured
  /// on the live game: the ids a player can actually reach are 40 close town
  /// blocks (5,926,602 triangles, 610 MiB) and 325 site configurations
  /// (6,890,594 triangles, 710 MiB). A session that browses cities and
  /// construction projects walks to 1,320 MiB of buffers nothing frees, and
  /// the driver drops the context long before that.
  ///
  /// Bounded by TRIANGLES rather than by entry count, because these differ by
  /// a hundredfold: a far-LOD site is 164 triangles and a close town block is
  /// 214,044. 1.2M triangles is about 124 MiB at the 108 bytes a triangle
  /// costs here, which holds any view this game builds — the heaviest
  /// realistic working set is roughly 900k (a city card at 214k, a dozen site
  /// cards at ~35k each, and the whole 46-model deck at 272k) — with room to
  /// spare and no thrash.
  const CACHE_TRIANGLES = 1200000;
  const vaos = new Map();
  let cachedTriangles = 0;

  /// Building the program is separate from creating the context, because a
  /// context can come BACK. On `webglcontextrestored` every shader, program and
  /// buffer the driver held is gone but the canvas and its listeners are not, so
  /// the restore path re-runs exactly this and repaints, rather than making a
  /// second canvas and orphaning the first.
  function setupGl() {
    const vs = compile(gl.VERTEX_SHADER, VERT), fs = compile(gl.FRAGMENT_SHADER, fragSource());
    if (!vs || !fs) return false;
    prog = gl.createProgram();
    gl.attachShader(prog, vs); gl.attachShader(prog, fs);
    gl.bindAttribLocation(prog, 0, "aPos");
    gl.bindAttribLocation(prog, 1, "aNrm");
    gl.bindAttribLocation(prog, 2, "aCol");
    gl.linkProgram(prog);
    if (!gl.getProgramParameter(prog, gl.LINK_STATUS)) return false;
    uMVP = gl.getUniformLocation(prog, "uMVP");
    uEye = gl.getUniformLocation(prog, "uEye");
    uHeight = gl.getUniformLocation(prog, "uHeight");
    uCharacter = gl.getUniformLocation(prog, "uCharacter");
    gl.enable(gl.DEPTH_TEST);
    // No back-face culling, deliberately. Three parts of the deck are open
    // shells — a dish is a paraboloid with no back, a rotodome is a disc, a
    // solar array is a sheet — and culling would make each of them vanish
    // from one side. The shader is two-sided for the same reason.
    gl.disable(gl.CULL_FACE);
    return true;
  }
  function init() {
    if (available !== null) return available;
    available = false;
    if (typeof document === "undefined" || !root.ArsenalModels) return available;
    try {
      glCanvas = document.createElement("canvas");
      glCanvas.width = 256; glCanvas.height = 256;
      gl = glCanvas.getContext("webgl2", {
        alpha: true, antialias: true, premultipliedAlpha: true, depth: true,
      });
      if (!gl) return available;
      // A lost context is not an error to swallow: preventDefault is what makes
      // the browser promise a restore, and everything cached is dead the moment
      // it fires. Cards go quiet rather than drawing from freed handles.
      glCanvas.addEventListener("webglcontextlost", (event) => {
        event.preventDefault();
        lost = true;
        vaos.clear();
        cachedTriangles = 0;
        sprites.clear();
      }, false);
      glCanvas.addEventListener("webglcontextrestored", () => {
        lost = false;
        if (!setupGl()) { available = false; return; }
        mounted.forEach((state, canvas) => { if (canvas.isConnected) { state.aspect = null; paint(canvas, state); } });
      }, false);
      if (!setupGl()) return available;
      available = true;
    } catch (e) { available = false; }
    return available;
  }
  function compile(kind, src) {
    const s = gl.createShader(kind);
    gl.shaderSource(s, src); gl.compileShader(s);
    return gl.getShaderParameter(s, gl.COMPILE_STATUS) ? s : null;
  }

  /// WHERE A MESH COMES FROM. This file was written when the equipment deck
  /// was the only source of card art. It is not any more: construction sites
  /// and town blocks are the same kind of object — three parallel Float32Arrays
  /// and a bounding box — and they want the same single context, for exactly
  /// the reason in this file's header. So an id may carry a provider prefix,
  /// `site:arms_plant/frame/2`, and a bare id still means the deck. Providers
  /// register from the page rather than being imported here, because the
  /// geometry modules are DOM-free by contract and must not learn about GL.
  const providers = new Map();
  function register(prefix, build) {
    if (typeof prefix !== "string" || typeof build !== "function") return false;
    providers.set(prefix, build);
    return true;
  }
  function resolveMesh(id, cls) {
    const text = String(id);
    const cut = text.indexOf(":");
    if (cut > 0) {
      const build = providers.get(text.slice(0, cut));
      if (!build) return null;
      try { return build(text.slice(cut + 1), cls) || null; } catch (e) { return null; }
    }
    return root.ArsenalModels ? root.ArsenalModels.build(text, cls) : null;
  }

  /// One VAO per mesh, built on first sight and kept. The whole deck is about
  /// twenty thousand triangles, so keeping all of it costs less than a single
  /// one of the baked map textures this page already holds. Sites and towns
  /// cache under the id STRING, which is what makes a card cheap on the second
  /// render of a panel that rebuilds its own innerHTML.
  /// Free one entry outright. Deleting a vertex array does NOT delete the
  /// buffers attached to it, so those are tracked and deleted by hand; and an
  /// entry is reachable under more than one key (its own id and the geometry
  /// id it resolved to), so every alias goes at once. Dropping only one alias
  /// would leave the other pointing at a deleted vertex array, which draws
  /// nothing and reports no error.
  function release(entry) {
    for (const alias of entry.keys) vaos.delete(alias);
    cachedTriangles -= entry.tris;
    if (gl && !lost) {
      for (const b of entry.bufs) gl.deleteBuffer(b);
      gl.deleteVertexArray(entry.vao);
    }
    entry.geom = null;            // and let the CPU-side arrays go too
  }

  /// Least-recently-USED, not least-recently-built: a Map iterates in
  /// insertion order, so touching an entry on every hit and re-inserting it
  /// keeps the order honest. Without the touch this would evict whatever was
  /// oldest, which on a panel that repaints every card is the one being drawn.
  function touch(entry) {
    for (const alias of entry.keys) { vaos.delete(alias); vaos.set(alias, entry); }
  }

  function trim(protect) {
    for (const entry of [...vaos.values()]) {
      if (cachedTriangles <= CACHE_TRIANGLES) return;
      if (entry === protect || !vaos.has(entry.keys[0])) continue;
      release(entry);
    }
  }

  function bufferFor(id, cls) {
    if (vaos.has(id)) { const hit = vaos.get(id); touch(hit); return hit; }
    const geom = resolveMesh(id, cls);
    if (!geom || !geom.positions || !geom.positions.length) return null;
    const key = geom.id || id;
    if (vaos.has(key)) {
      const hit = vaos.get(key);
      hit.keys.push(id);          // a new alias for the same geometry
      vaos.set(id, hit);
      touch(hit);
      return hit;
    }
    const vao = gl.createVertexArray();
    const bufs = [];
    gl.bindVertexArray(vao);
    [[geom.positions, 0], [geom.normals, 1], [geom.colors, 2]].forEach((pair) => {
      const b = gl.createBuffer();
      bufs.push(b);
      gl.bindBuffer(gl.ARRAY_BUFFER, b);
      gl.bufferData(gl.ARRAY_BUFFER, pair[0], gl.STATIC_DRAW);
      gl.enableVertexAttribArray(pair[1]);
      gl.vertexAttribPointer(pair[1], 3, gl.FLOAT, false, 0, 0);
    });
    gl.bindVertexArray(null);
    // Two mesh shapes reach here. The deck's carries `centre` and `count`; the
    // equipment/site/town generators carry `bounds` and `triangleCount`. Neither
    // is wrong, so read whichever is present rather than making four generators
    // agree on a field name after the fact.
    const count = typeof geom.count === "number" ? geom.count : geom.positions.length / 3;
    const centre = geom.centre || (geom.bounds
      ? [(geom.bounds.min[0] + geom.bounds.max[0]) / 2,
        (geom.bounds.min[1] + geom.bounds.max[1]) / 2,
        (geom.bounds.min[2] + geom.bounds.max[2]) / 2]
      : [0, 0, 0]);
    // The framing radius is the bounding sphere about the box centre, so a
    // task group and a hand-launched drone are both fitted by the same rule and
    // neither is clipped when it turns.
    let r = 0;
    for (let i = 0; i < geom.positions.length; i += 3) {
      r = Math.max(r, Math.hypot(
        geom.positions[i] - centre[0],
        geom.positions[i + 1] - centre[1],
        geom.positions[i + 2] - centre[2],
      ));
    }
    const keys = key === id ? [key] : [key, id];
    const entry = { vao, bufs, keys, tris: count / 3,
      count, centre, radius: r || 1, fits: new Map(), geom };
    vaos.set(key, entry);
    if (key !== id) vaos.set(id, entry);
    cachedTriangles += entry.tris;
    // Trimmed AFTER inserting, and the new entry is protected: the caller is
    // about to draw it, so evicting it here would rebuild it immediately.
    trim(entry);
    return entry;
  }

  // ------------------------------------------------------------- matrices
  function perspective(fovDeg, aspect, near, far) {
    const f = 1 / Math.tan((fovDeg * Math.PI) / 360);
    return [f / aspect, 0, 0, 0, 0, f, 0, 0, 0, 0, (far + near) / (near - far), -1,
      0, 0, (2 * far * near) / (near - far), 0];
  }
  function lookAt(eye, at, up) {
    const z = norm([eye[0] - at[0], eye[1] - at[1], eye[2] - at[2]]);
    const x = norm(cross(up, z)), y = cross(z, x);
    return [x[0], y[0], z[0], 0, x[1], y[1], z[1], 0, x[2], y[2], z[2], 0,
      -dot(x, eye), -dot(y, eye), -dot(z, eye), 1];
  }
  function cross(a, b) {
    return [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]];
  }
  function dot(a, b) { return a[0] * b[0] + a[1] * b[1] + a[2] * b[2]; }
  function norm(v) {
    const l = Math.hypot(v[0], v[1], v[2]) || 1;
    return [v[0] / l, v[1] / l, v[2] / l];
  }
  function mul4(a, b) {
    const o = new Float32Array(16);
    for (let c = 0; c < 4; c += 1) {
      for (let r = 0; r < 4; r += 1) {
        o[c * 4 + r] = a[r] * b[c * 4] + a[4 + r] * b[c * 4 + 1]
          + a[8 + r] * b[c * 4 + 2] + a[12 + r] * b[c * 4 + 3];
      }
    }
    return o;
  }
  function translate(t) { return [1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, t[0], t[1], t[2], 1]; }

  /// FRAME THE BOX, NOT THE SPHERE — AND FRAME IT FOR THE WHOLE TURN.
  ///
  /// Almost everything in this deck is long and thin: a frigate is 133 m by 15,
  /// a Tomahawk is 6.25 by 0.5. The sphere that contains a frigate is 133 across
  /// in every direction, so fitting it throws away four fifths of the card and
  /// the ship becomes a grey hyphen. Fitting the BOX at the resting angle fixes
  /// that and breaks something else: the moment a card is hovered and starts to
  /// turn, the long axis swings towards the camera and the model grows out of
  /// frame. So the distance is the worst case over a full revolution at this
  /// card's aspect ratio — a few hundred dot products, once per model per
  /// aspect, and then it is a lookup. The model fills the card and stays inside
  /// it all the way round.
  /// FIT THE SILHOUETTE, NOT THE BOX. The box that contains a tank has corners
  /// the tank does not reach, and framing to those corners leaves nearly half
  /// the card empty — measured at 56% fill before this read the vertices
  /// instead. Every vertex is walked, which sounds expensive and is not: the
  /// largest model in the deck is 3,432 of them, the answer is cached per model
  /// per card shape, and a full revolution of samples costs about a millisecond
  /// once.
  /// WHERE TO STAND, AND WHAT TO LOOK AT.
  ///
  /// The distance alone is not enough, and framing on the model's bounding-box
  /// centre is what left every card wasting a third of itself. A site is wide,
  /// flat and seen from above, so under perspective its near ground projects
  /// further from centre than its far ground does and the silhouette lands LOW.
  /// Measured on the shipped cards: a starter industry site in a 3.66:1 strip
  /// spanned NDC -0.942 to +0.298 -- flush against the bottom edge with 37% of
  /// the card empty above it -- and every site and vehicle tested had an offset
  /// of the same sign.
  ///
  /// So: fit, look at where the silhouette actually IS, fit again. Three passes
  /// converge because the correction is perpendicular to the view axis and so
  /// barely moves the depths it was derived from.
  ///
  /// The pivot is measured at ONE angle and then used for the turning distance
  /// too. Recomputing it per angle would make a spinning card wobble; sharing
  /// it keeps the turn distance a bound on every angle and merely lets the card
  /// drift a little off-centre as it comes round, which is what a turntable
  /// does anyway.
  /// FIVE PASSES, because the exact solve below only settles the extremes it
  /// can currently SEE. Each pass solves outright for the two vertices that
  /// are extreme right now, but moving the pivot can hand that role to a
  /// different pair, so the answer is approached rather than reached in one
  /// step. Four is the measured minimum at which every shipped model settles
  /// -- three leaves the wide thin airframes off centre -- and this keeps one
  /// pass in hand. The fits are cached per model and aspect, so they cost
  /// nothing after the first paint of a given card shape.
  const PASSES = 5;
  function fitFrame(entry, aspect, pitch, yaw, fixed) {
    const key = `${aspect.toFixed(3)}|${pitch}|${yaw == null ? "turn" : yaw}|${fixed ? "fixed" : ""}`;
    if (entry.fits.has(key)) return entry.fits.get(key);
    const tanV = Math.tan((FOV * Math.PI) / 360), tanH = tanV * aspect;
    const rp = (pitch * Math.PI) / 180;
    const p = entry.geom.positions;
    let ox = fixed ? fixed[0] : entry.centre[0];
    let oy = fixed ? fixed[1] : entry.centre[1];
    let oz = fixed ? fixed[2] : entry.centre[2];
    const ry0 = ((yaw == null ? 0 : yaw) * Math.PI) / 180;
    const f0 = [Math.sin(ry0) * Math.cos(rp), Math.sin(rp), Math.cos(ry0) * Math.cos(rp)];
    const r0 = norm(cross([0, 1, 0], f0));
    const u0 = cross(f0, r0);
    let d = 0;
    for (let pass = 0; pass < PASSES; pass += 1) {
      let worst = 0;
      for (let a = 0; a < 360; a += 10) {
        const ry = ((yaw == null ? a : yaw) * Math.PI) / 180;
        const f = [Math.sin(ry) * Math.cos(rp), Math.sin(rp), Math.cos(ry) * Math.cos(rp)];
        const r = norm(cross([0, 1, 0], f));
        const u = cross(f, r);
        for (let i = 0; i < p.length; i += 3) {
          const x = p[i] - ox, y = p[i + 1] - oy, z = p[i + 2] - oz;
          const depth = x * f[0] + y * f[1] + z * f[2];
          const lx = Math.abs(x * r[0] + y * r[1] + z * r[2]);
          const ly = Math.abs(x * u[0] + y * u[1] + z * u[2]);
          const need = depth + Math.max(lx / tanH, ly / tanV);
          if (need > worst) worst = need;
        }
        if (yaw != null) break;
      }
      d = Math.max(worst * 1.03, entry.radius * 0.1);
      // A pivot handed in is not ours to move: the turning fit must bound
      // every angle about the SAME point the resting card is drawn around,
      // or the model clips the moment it comes round to an angle whose own
      // pivot sat elsewhere.
      if (fixed || pass === PASSES - 1) break;
      // The two extremes in each axis, WITH the camera distance of the vertex
      // that achieved each. That distance is the whole difficulty: a shift of
      // the pivot moves a near vertex across the frame further than a far one,
      // so a correction scaled by the viewing distance overshoots on a deep
      // model and the pivot oscillates instead of settling. Five passes made
      // the raven WORSE than three -- 0.239 off centre against 0.145 -- which
      // is what sent me looking for this rather than adding more passes.
      let mnx = Infinity, mxx = -Infinity, mny = Infinity, mxy = -Infinity;
      let lxMin = 0, lxMax = 0, zxMin = 1, zxMax = 1;
      let lyMin = 0, lyMax = 0, zyMin = 1, zyMax = 1;
      for (let i = 0; i < p.length; i += 3) {
        const x = p[i] - ox, y = p[i + 1] - oy, z = p[i + 2] - oz;
        const zc = d - (x * f0[0] + y * f0[1] + z * f0[2]);
        if (zc <= 1e-6) continue;
        const lx = x * r0[0] + y * r0[1] + z * r0[2];
        const ly = x * u0[0] + y * u0[1] + z * u0[2];
        const nx = lx / (tanH * zc), ny = ly / (tanV * zc);
        if (nx < mnx) { mnx = nx; lxMin = lx; zxMin = zc; }
        if (nx > mxx) { mxx = nx; lxMax = lx; zxMax = zc; }
        if (ny < mny) { mny = ny; lyMin = ly; zyMin = zc; }
        if (ny > mxy) { mxy = ny; lyMax = ly; zyMax = zc; }
      }
      if (!(mxx > mnx) || !(mxy > mny)) break;
      // Solve for the sideways shift that makes the two extremes symmetric.
      // A shift is perpendicular to the view axis, so it does not change any
      // zc, and (lMin - s)/zMin = -(lMax - s)/zMax has the exact solution
      // below. It lands in one pass instead of creeping toward the answer.
      const cx = (lxMin * zxMax + lxMax * zxMin) / (zxMin + zxMax);
      const cy = (lyMin * zyMax + lyMax * zyMin) / (zyMin + zyMax);
      ox += cx * r0[0] + cy * u0[0];
      oy += cx * r0[1] + cy * u0[1];
      oz += cx * r0[2] + cy * u0[2];
    }
    const fit = { d, pivot: [ox, oy, oz] };
    entry.fits.set(key, fit);
    return fit;
  }
  /// The two distances a card ever uses: tight on the angle it rests at, and
  /// far enough that nothing leaves the frame at any angle it can turn to.
  function fitsFor(id, cls, aspect, pitch, restYaw) {
    const entry = bufferFor(id, cls);
    if (!entry) return null;
    const rest = fitFrame(entry, aspect, pitch, restYaw);
    // The RESTING pivot is the one that ships -- it is the angle the card
    // actually sits at -- so the turning distance is fitted about THAT
    // point rather than one of its own. Reusing it also stops the model
    // sliding in the frame the moment the pointer arrives.
    return [rest.d, fitFrame(entry, aspect, pitch, null, rest.pivot).d, rest.pivot];
  }

  /// Draw one model into the shared GL canvas at w x h device pixels, then hand
  /// the caller the canvas to copy. Nothing is retained between calls except
  /// the buffers.
  function renderTo(id, cls, w, h, yaw, pitch, dist, pivot) {
    if (lost) return null;
    if (!init()) return null;
    const entry = bufferFor(id, cls);
    if (!entry) return null;
    if (glCanvas.width < w || glCanvas.height < h) {
      glCanvas.width = Math.max(glCanvas.width, w);
      glCanvas.height = Math.max(glCanvas.height, h);
    }
    const own = dist && pivot ? null : fitFrame(entry, w / h, pitch, yaw);
    const d = dist || own.d;
    const at = pivot || own.pivot;
    const ry = (yaw * Math.PI) / 180, rp = (pitch * Math.PI) / 180;
    const eye = [
      Math.sin(ry) * Math.cos(rp) * d,
      Math.sin(rp) * d,
      Math.cos(ry) * Math.cos(rp) * d,
    ];
    const view = lookAt(eye, [0, 0, 0], [0, 1, 0]);
    // The pivot can sit off the bounding-box centre, so the far plane gets
    // the radius twice over rather than assuming the model is centred on it.
    const proj = perspective(FOV, w / h, d * 0.02, d + entry.radius * 3.5);
    const mvp = mul4(mul4(proj, view), translate([-at[0], -at[1], -at[2]]));
    // The frame is drawn into the BOTTOM-left of the shared canvas and copied
    // from the bottom-left of it. Those are the same corner and two different
    // numbers: GL measures its viewport from the bottom, `drawImage` measures
    // its source rectangle from the top. Rendering at `height - h` and copying
    // from `height - h` -- the obvious pairing -- reads the one band of the
    // canvas nothing was ever drawn into, and every card comes out empty.
    gl.viewport(0, 0, w, h);
    gl.enable(gl.SCISSOR_TEST);
    gl.scissor(0, 0, w, h);
    gl.clearColor(0, 0, 0, 0);
    gl.clear(gl.COLOR_BUFFER_BIT | gl.DEPTH_BUFFER_BIT);
    gl.useProgram(prog);
    gl.uniformMatrix4fv(uMVP, false, mvp);
    gl.uniform3f(uEye, eye[0] + at[0], eye[1] + at[1], eye[2] + at[2]);
    gl.uniform1f(uHeight, entry.geom.bounds ? (entry.geom.bounds.max[1] - entry.geom.bounds.min[1])
      : (entry.geom.max ? entry.geom.max[1] - entry.geom.min[1] : 2.0));
    gl.uniform1f(uCharacter, entry.geom.assetKind === "character" ? 1 : 0);
    gl.bindVertexArray(entry.vao);
    gl.drawArrays(gl.TRIANGLES, 0, entry.count);
    gl.bindVertexArray(null);
    gl.disable(gl.SCISSOR_TEST);
    return { w, h, top: glCanvas.height - h };
  }

  // ------------------------------------------------------------ the cards
  // An explicitly controlled view shares the renderer without joining the
  // equipment hover animation or retaining its canvas in the card registry.
  function draw(canvas, id, opts) {
    if (!canvas || !init() || lost) return false;
    const o = opts || {}, entry = bufferFor(id, "");
    if (!entry) return false;
    const rect = canvas.getBoundingClientRect();
    if (rect.width <= 0 || rect.height <= 0) return false;
    const dpr = Math.min(MAX_DPR, root.devicePixelRatio || 1);
    const w = Math.max(8, Math.min(2048, Math.round(rect.width * dpr)));
    const h = Math.max(8, Math.min(2048, Math.round(rect.height * dpr)));
    if (canvas.width !== w) canvas.width = w;
    if (canvas.height !== h) canvas.height = h;
    const yaw = Number.isFinite(o.yaw) ? o.yaw : 12;
    // Quantize the manual pitch so dragging cannot accumulate unbounded fit
    // entries, or refit every vertex for sub-pixel camera changes.
    const pitch = Number.isFinite(o.pitch) ? Math.round(Math.max(-20,Math.min(34,o.pitch))/2)*2 : 6;
    // A portrait phone canvas needs more horizontal room for both ears. The
    // face preset must remain useful when the modal becomes tall and narrow.
    const zoomLimit = entry.geom.assetKind === "character" ? Math.min(3,Math.max(1.8,3*w/h)) : 1.8;
    const zoom = Number.isFinite(o.zoom) ? Math.max(.75,Math.min(zoomLimit,o.zoom)) : 1;
    const fit = fitFrame(entry,w/h,pitch,null);
    const pivot = fit.pivot.slice();
    // Close inspection moves toward the face, keeping the head visible while
    // the lower body leaves the frame as the player zooms in.
    if (entry.geom.assetKind === "character" && zoom > 1) {
      const t = Math.min(1,(zoom-1)/1.3);
      if (entry.geom.portraitPivot) {
        for (let i=0;i<3;i++) pivot[i] += (entry.geom.portraitPivot[i]-pivot[i])*t;
      } else {
        pivot[1] += t * (entry.geom.bounds.max[1]-entry.geom.bounds.min[1]) * .37;
      }
    }
    const out = renderTo(id,"",w,h,yaw,pitch,fit.d/zoom,pivot);
    const ctx = canvas.getContext("2d");
    if (!out || !ctx) return false;
    ctx.clearRect(0,0,w,h);
    ctx.drawImage(glCanvas,0,out.top,w,h,0,0,w,h);
    return true;
  }

  const mounted = new Map();   // canvas -> state
  let active = null;           // the one card that is turning or settling back
  let frame = 0;

  function paint(canvas, state) {
    const dpr = Math.min(MAX_DPR, root.devicePixelRatio || 1);
    const rect = canvas.getBoundingClientRect();
    const cssW = Math.max(8, Math.round(rect.width || canvas.clientWidth || 48));
    const cssH = Math.max(8, Math.round(rect.height || canvas.clientHeight || 48));
    const w = Math.round(cssW * dpr), h = Math.round(cssH * dpr);
    if (canvas.width !== w || canvas.height !== h) {
      canvas.width = w; canvas.height = h;
      state.aspect = null;
    }
    // The two fits depend on the card's shape, so they are recomputed only when
    // that changes -- a resize, or the first paint.
    if (state.aspect !== w / h) {
      const fits = fitsFor(state.id, state.cls, w / h, state.pitch, state.rest);
      if (!fits) return false;
      state.aspect = w / h;
      state.distRest = fits[0];
      state.distTurn = fits[1];
      state.pivot = fits[2];
      // A RESIZE MUST MOVE THE CAMERA, and this kept the old distance for
      // ever: `dist` was assigned only while it was still zero, so a card
      // whose frame changed shape went on being drawn from the distance
      // fitted to its PREVIOUS shape. Only a card mid-turn keeps its eased
      // value, because that ease already heads for the new goal.
      if (!state.dist || canvas !== active) state.dist = state.spinning ? fits[1] : fits[0];
    }
    const out = renderTo(state.id, state.cls, w, h, state.yaw, state.pitch, state.dist, state.pivot);
    const ctx = canvas.getContext("2d");
    if (!ctx) return false;
    ctx.clearRect(0, 0, w, h);
    if (!out) return false;
    ctx.drawImage(glCanvas, 0, out.top, w, h, 0, 0, w, h);
    return true;
  }

  /// A CARD FILLS ITS FRAME AT REST AND PULLS BACK TO TURN.
  ///
  /// The distance that keeps a 133 m frigate inside the card at EVERY angle
  /// leaves a third of the card empty at the one angle it actually sits at,
  /// because a hull swung along the view axis is much taller on screen than a
  /// hull seen broadside. Two distances and an ease between them gets both: the
  /// still card is framed tight, and the hovered one draws back as it starts to
  /// turn, which is also what a hand does when it picks something up to look at
  /// it. Off the pointer, the model turns back to the angle every other card is
  /// resting at, so a grid never ends up a field of randomly-rotated models.
  function tick() {
    frame = 0;
    const canvas = active;
    if (!canvas) return;
    const state = mounted.get(canvas);
    if (!state || !canvas.isConnected) { active = null; return; }
    const goal = state.spinning ? state.distTurn : state.distRest;
    state.dist += (goal - state.dist) * 0.18;
    if (state.spinning) {
      state.yaw = (state.yaw + 0.85) % 360;
    } else {
      const delta = ((state.rest - state.yaw + 540) % 360) - 180;
      state.yaw += delta * 0.18;
      if (Math.abs(delta) < 0.4 && Math.abs(goal - state.dist) < goal * 0.003) {
        state.yaw = state.rest;
        state.dist = goal;
        paint(canvas, state);
        active = null;
        return;
      }
    }
    paint(canvas, state);
    frame = root.requestAnimationFrame(tick);
  }
  /// Turning is opt-out. A player who has asked their system for less motion
  /// gets the same still three-quarter view they get when the pointer is
  /// elsewhere, which is already the resting state -- so there is nothing to
  /// design twice, only a spin not to start.
  function reducedMotion() {
    return typeof matchMedia === "function"
      && matchMedia("(prefers-reduced-motion: reduce)").matches;
  }
  function startSpin(canvas) {
    const state = mounted.get(canvas);
    if (!state || reducedMotion()) return;
    if (active && active !== canvas) settle(active);
    state.spinning = true;
    active = canvas;
    if (!frame) frame = root.requestAnimationFrame(tick);
  }
  function settle(canvas) {
    const state = mounted.get(canvas);
    if (!state) return;
    state.spinning = false;
    if (active === canvas && !frame) frame = root.requestAnimationFrame(tick);
  }

  /// Mount one canvas. Returns false when there is no GL, and the caller is
  /// expected to leave whatever it had in place — the catalogue keeps its
  /// class glyph, which is why the glyph was never removed from the markup.
  function mount(canvas, id, opts) {
    if (!canvas || !init()) return false;
    const o = opts || {};
    const state = {
      id, cls: o.cls || "",
      rest: o.yaw == null ? REST_YAW : o.yaw,
      yaw: o.yaw == null ? REST_YAW : o.yaw,
      pitch: o.pitch == null ? REST_PITCH : o.pitch,
      aspect: null, dist: 0, distRest: 0, distTurn: 0, pivot: null, spinning: false,
    };
    if (!bufferFor(id, state.cls)) return false;
    mounted.set(canvas, state);
    canvas.setAttribute("aria-hidden", "true");
    return paint(canvas, state);
  }

  /// Mount everything under `root` that asked for a model. Called after each
  /// panel render, because the panels replace their own innerHTML and a mounted
  /// canvas does not survive that.
  /// The mount half of `scan`, shared by the immediate and the deferred path.
  const pending = new Set();
  function attach(canvas) {
    pending.delete(canvas);
    const ok = mount(canvas, canvas.getAttribute("data-kit3d"),
      { cls: canvas.getAttribute("data-kit3d-class") || "" });
    if (!ok) { canvas.remove(); return false; }
    const holder = canvas.closest("[data-class-mark], .work-card, .factory-ledger-row")
      || canvas.parentElement;
    if (holder) holder.classList.add("has-kit3d");
    return true;
  }
  const observer = typeof IntersectionObserver === "function"
    ? new IntersectionObserver((entries) => {
      entries.forEach((entry) => {
        if (!entry.isIntersecting) return;
        observer.unobserve(entry.target);
        if (entry.target.isConnected) attach(entry.target);
      });
    }, { rootMargin: "200px" })
    : null;

  /// MOUNT WHAT CAN BE SEEN, DEFER THE REST.
  ///
  /// Measured on this machine: a 46-card equipment catalogue cost 298 ms to
  /// open — 6.5 ms a card to build, upload and draw — because scan() mounted
  /// every canvas it found whether or not anyone could see it. The deck went
  /// from 20,780 triangles to 272,491 in the fidelity pass, and that turned a
  /// hitch nobody noticed into one they would. Per-FRAME cost was never the
  /// problem: the designer submits 94,576 triangles a frame and a card that is
  /// not being hovered submits none at all.
  ///
  /// So a card in view mounts now, synchronously, exactly as before — nothing
  /// that is visible waits on an observer callback, which also keeps the
  /// bench and the tests deterministic. A card below the fold is observed and
  /// mounts when it scrolls in, and keeps its class glyph until it does.
  function scan(where) {
    if (!init()) return 0;
    const host = where || document;
    const list = host.querySelectorAll ? host.querySelectorAll("canvas[data-kit3d]") : [];
    let n = 0;
    const near = (canvas) => {
      const r = canvas.getBoundingClientRect();
      const h = root.innerHeight || 0, w = root.innerWidth || 0;
      // A margin of one viewport, so scrolling meets meshes that are ready.
      return r.bottom > -h && r.top < h * 2 && r.right > -w && r.left < w * 2;
    };
    list.forEach((canvas) => {
      if (mounted.has(canvas) || pending.has(canvas)) return;
      if (near(canvas)) { if (attach(canvas)) n += 1; return; }
      if (!observer) { if (attach(canvas)) n += 1; return; }
      pending.add(canvas);
      observer.observe(canvas);
    });
    if (mounted.size > 400) {
      mounted.forEach((v, k) => { if (!k.isConnected) mounted.delete(k); });
    }
    return n;
  }

  /// Install a surface treatment, or pass nothing to go back to flat albedo.
  /// Relinks the program and drops the sprite cache, because a sprite is a
  /// baked picture and would otherwise keep the old surface forever.
  function setSurface(glsl) {
    if (!init()) return false;
    surfaceGlsl = (typeof glsl === "string" && glsl.trim()) ? glsl : DEFAULT_SURFACE;
    const vs = compile(gl.VERTEX_SHADER, VERT), fs = compile(gl.FRAGMENT_SHADER, fragSource());
    if (!vs || !fs) { surfaceGlsl = DEFAULT_SURFACE; return false; }
    const next = gl.createProgram();
    gl.attachShader(next, vs); gl.attachShader(next, fs);
    gl.bindAttribLocation(next, 0, "aPos");
    gl.bindAttribLocation(next, 1, "aNrm");
    gl.bindAttribLocation(next, 2, "aCol");
    gl.linkProgram(next);
    if (!gl.getProgramParameter(next, gl.LINK_STATUS)) { surfaceGlsl = DEFAULT_SURFACE; return false; }
    prog = next;
    uMVP = gl.getUniformLocation(prog, "uMVP");
    uEye = gl.getUniformLocation(prog, "uEye");
    uHeight = gl.getUniformLocation(prog, "uHeight");
    uCharacter = gl.getUniformLocation(prog, "uCharacter");
    sprites.clear();
    mounted.forEach((state, canvas) => { if (canvas.isConnected) paint(canvas, state); });
    return true;
  }

  /// A MODEL BAKED FLAT, ONCE, FOR THE 2D LAYERS.
  ///
  /// The globe's marker pass is a 2D context. It cannot hold a GL model and it
  /// redraws on every camera move, so it needs something it can `drawImage` in
  /// microseconds — and it needs it SYNCHRONOUSLY, which rules out `dataURL`
  /// and an Image that has to load. A map-LOD site is a few hundred triangles
  /// and bakes in well under a millisecond; after the first bake this is a Map
  /// lookup. Failures cache as null too, so a kind with no mesh is not retried
  /// on every frame of a camera drag.
  function sprite(id, sizePx, opts) {
    if (!init() || lost) return null;
    const size = Math.max(8, Math.min(256, Math.round(sizePx || 64)));
    const key = `${id}@${size}`;
    if (sprites.has(key)) return sprites.get(key);
    const o = opts || {};
    const out = renderTo(id, o.cls || "", size, size,
      o.yaw == null ? REST_YAW : o.yaw, o.pitch == null ? REST_PITCH : o.pitch);
    if (!out) { sprites.set(key, null); return null; }
    const flat = document.createElement("canvas");
    flat.width = size; flat.height = size;
    const ctx = flat.getContext("2d");
    if (!ctx) { sprites.set(key, null); return null; }
    ctx.drawImage(glCanvas, 0, out.top, size, size, 0, 0, size, size);
    sprites.set(key, flat);
    return flat;
  }

  /// A still of one model as a data URL, for anything that wants a picture
  /// rather than a canvas — a tooltip, a report, the chronicle.
  function dataURL(id, size, opts) {
    if (!init()) return null;
    const o = opts || {};
    const s = size || 128;
    const out = renderTo(id, o.cls || "", s, s,
      o.yaw == null ? REST_YAW : o.yaw, o.pitch == null ? REST_PITCH : o.pitch);
    if (!out) return null;
    const c = document.createElement("canvas");
    c.width = s; c.height = s;
    c.getContext("2d").drawImage(glCanvas, 0, out.top, s, s, 0, 0, s, s);
    return c.toDataURL("image/png");
  }

  if (typeof document !== "undefined") {
    document.addEventListener("pointerover", (e) => {
      const holder = e.target && e.target.closest && e.target.closest(".has-kit3d");
      const canvas = holder && holder.querySelector("canvas[data-kit3d]");
      if (canvas && mounted.has(canvas)) startSpin(canvas);
    }, { passive: true });
    document.addEventListener("pointerout", (e) => {
      const holder = e.target && e.target.closest && e.target.closest(".has-kit3d");
      const canvas = holder && holder.querySelector("canvas[data-kit3d]");
      if (canvas && mounted.has(canvas) && !holder.contains(e.relatedTarget)) settle(canvas);
    }, { passive: true });
    document.addEventListener("focusin", (e) => {
      const holder = e.target && e.target.closest && e.target.closest(".has-kit3d");
      const canvas = holder && holder.querySelector("canvas[data-kit3d]");
      if (canvas && mounted.has(canvas)) startSpin(canvas);
    });
    document.addEventListener("focusout", (e) => {
      const holder = e.target && e.target.closest && e.target.closest(".has-kit3d");
      const canvas = holder && holder.querySelector("canvas[data-kit3d]");
      if (canvas && mounted.has(canvas)) settle(canvas);
    });
  }

  /// THE FRAMING MATHS, WITHOUT A GPU.
  ///
  /// `fitFrame` is a pure function of the geometry -- no GL, no DOM, no
  /// canvas -- and it decides how every card in the game is composed, so it is
  /// worth testing on every shipped model rather than on whatever happens to
  /// be on screen. This exposes it over a plain mesh: pass anything with
  /// `positions` and `bounds` and get back the distance and the pivot a card
  /// would use.
  function frameOf(geom, aspect, pitch, yaw, fixed) {
    // The kits disagree about where they keep their extents: the mesh kits
    // hand back `bounds.min/max`, the arsenal deck puts `min`/`max` at the top
    // level. The renderer copes with both, so this must too, or a whole family
    // silently drops out of any sweep written against it.
    if (!geom || !geom.positions) return null;
    const b = geom.bounds || geom;
    if (!b.min || !b.max) return null;
    const centre = [0, 1, 2].map((i) => (b.min[i] + b.max[i]) / 2);
    const p = geom.positions;
    let radius = 0;
    for (let i = 0; i < p.length; i += 3) {
      const d = Math.hypot(p[i] - centre[0], p[i + 1] - centre[1], p[i + 2] - centre[2]);
      if (d > radius) radius = d;
    }
    return fitFrame({ geom, centre, radius: radius || 1, fits: new Map() },
      aspect, pitch == null ? REST_PITCH : pitch,
      yaw === undefined ? REST_YAW : yaw, fixed);
  }

  root.Arsenal3D = {
    /// What the cache is holding right now, so the bound can be asserted
    /// rather than believed. `triangles` is the live total, `cap` the ceiling
    /// it is trimmed to, and `models` the number of distinct geometries (not
    /// keys: an entry reachable under two aliases is one model).
    cacheStats() {
      const seen = new Set();
      for (const e of vaos.values()) seen.add(e);
      return { triangles: cachedTriangles, cap: CACHE_TRIANGLES,
        models: seen.size, keys: vaos.size };
    },
    mount, scan, dataURL, renderTo, sprite, setSurface, frameOf, draw,
    REST_YAW, REST_PITCH, FOV,
    get available() { return init(); },
    register,
    canvasHtml(id, cls) {
      // Provider ids carry ':' '/' '.' and '-'. Everything else still goes, so
      // this stays an attribute value that cannot break out of its quotes.
      return `<canvas class="kit3d" data-kit3d="${String(id).replace(/[^a-z0-9_:/.-]/gi, "")}"`
        + ` data-kit3d-class="${String(cls || "").replace(/[^a-z]/gi, "")}"></canvas>`;
    },
  };
  // Node sees the same object, so the framing can be checked headlessly.
  if (typeof module === "object" && module.exports) module.exports = root.Arsenal3D;
})(typeof window !== "undefined" ? window : globalThis);
