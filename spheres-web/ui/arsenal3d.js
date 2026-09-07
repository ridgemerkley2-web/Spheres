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
  const FRAG = `#version 300 es
  precision highp float;
  in vec3 vNrm; in vec3 vCol; in vec3 vPos;
  uniform vec3 uEye;
  out vec4 outColor;
  void main() {
    vec3 N = normalize(vNrm);
    vec3 V = normalize(uEye - vPos);
    if (dot(N, V) < 0.0) N = -N;
    vec3 key = normalize(vec3(-0.45, 0.82, 0.55));
    vec3 fil = normalize(vec3(0.7, 0.15, -0.5));
    float sky = 0.5 + 0.5 * N.y;
    vec3 c = vCol * (0.26 + 0.24 * sky);
    c += vCol * max(dot(N, key), 0.0) * 0.86;
    c += vCol * max(dot(N, fil), 0.0) * 0.22 * vec3(0.75, 0.85, 1.0);
    float rim = pow(1.0 - max(dot(N, V), 0.0), 3.0);
    c += rim * 0.30 * vec3(0.62, 0.76, 0.95);
    float spec = pow(max(dot(reflect(-key, N), V), 0.0), 24.0);
    c += spec * 0.16;
    outColor = vec4(c, 1.0);
  }`;

  let gl = null, prog = null, uMVP = null, uEye = null, glCanvas = null;
  let available = null;
  const vaos = new Map();

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
      const vs = compile(gl.VERTEX_SHADER, VERT), fs = compile(gl.FRAGMENT_SHADER, FRAG);
      if (!vs || !fs) return available;
      prog = gl.createProgram();
      gl.attachShader(prog, vs); gl.attachShader(prog, fs);
      gl.bindAttribLocation(prog, 0, "aPos");
      gl.bindAttribLocation(prog, 1, "aNrm");
      gl.bindAttribLocation(prog, 2, "aCol");
      gl.linkProgram(prog);
      if (!gl.getProgramParameter(prog, gl.LINK_STATUS)) return available;
      uMVP = gl.getUniformLocation(prog, "uMVP");
      uEye = gl.getUniformLocation(prog, "uEye");
      gl.enable(gl.DEPTH_TEST);
      // No back-face culling, deliberately. Three parts of the deck are open
      // shells — a dish is a paraboloid with no back, a rotodome is a disc, a
      // solar array is a sheet — and culling would make each of them vanish
      // from one side. The shader is two-sided for the same reason.
      gl.disable(gl.CULL_FACE);
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
  function bufferFor(id, cls) {
    if (vaos.has(id)) return vaos.get(id);
    const geom = resolveMesh(id, cls);
    if (!geom || !geom.positions || !geom.positions.length) return null;
    const key = geom.id || id;
    if (vaos.has(key)) { vaos.set(id, vaos.get(key)); return vaos.get(key); }
    const vao = gl.createVertexArray();
    gl.bindVertexArray(vao);
    [[geom.positions, 0], [geom.normals, 1], [geom.colors, 2]].forEach((pair) => {
      const b = gl.createBuffer();
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
    const entry = { vao, count, centre, radius: r || 1, fits: new Map(), geom };
    vaos.set(key, entry);
    if (key !== id) vaos.set(id, entry);
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
  function fitDistance(entry, aspect, pitch, yaw) {
    const key = `${aspect.toFixed(3)}|${pitch}|${yaw == null ? "turn" : yaw}`;
    if (entry.fits.has(key)) return entry.fits.get(key);
    const tanV = Math.tan((FOV * Math.PI) / 360), tanH = tanV * aspect;
    const rp = (pitch * Math.PI) / 180;
    const p = entry.geom.positions;
    const ox = entry.centre[0], oy = entry.centre[1], oz = entry.centre[2];
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
    const d = Math.max(worst * 1.03, entry.radius * 0.1);
    entry.fits.set(key, d);
    return d;
  }
  /// The two distances a card ever uses: tight on the angle it rests at, and
  /// far enough that nothing leaves the frame at any angle it can turn to.
  function fitsFor(id, cls, aspect, pitch, restYaw) {
    const entry = bufferFor(id, cls);
    if (!entry) return null;
    return [fitDistance(entry, aspect, pitch, restYaw), fitDistance(entry, aspect, pitch, null)];
  }

  /// Draw one model into the shared GL canvas at w x h device pixels, then hand
  /// the caller the canvas to copy. Nothing is retained between calls except
  /// the buffers.
  function renderTo(id, cls, w, h, yaw, pitch, dist) {
    if (!init()) return null;
    const entry = bufferFor(id, cls);
    if (!entry) return null;
    if (glCanvas.width < w || glCanvas.height < h) {
      glCanvas.width = Math.max(glCanvas.width, w);
      glCanvas.height = Math.max(glCanvas.height, h);
    }
    const d = dist || fitDistance(entry, w / h, pitch, yaw);
    const ry = (yaw * Math.PI) / 180, rp = (pitch * Math.PI) / 180;
    const eye = [
      Math.sin(ry) * Math.cos(rp) * d,
      Math.sin(rp) * d,
      Math.cos(ry) * Math.cos(rp) * d,
    ];
    const view = lookAt(eye, [0, 0, 0], [0, 1, 0]);
    const proj = perspective(FOV, w / h, d * 0.02, d + entry.radius * 2.5);
    const mvp = mul4(mul4(proj, view), translate([-entry.centre[0], -entry.centre[1], -entry.centre[2]]));
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
    gl.uniform3f(uEye, eye[0] + entry.centre[0], eye[1] + entry.centre[1], eye[2] + entry.centre[2]);
    gl.bindVertexArray(entry.vao);
    gl.drawArrays(gl.TRIANGLES, 0, entry.count);
    gl.bindVertexArray(null);
    gl.disable(gl.SCISSOR_TEST);
    return { w, h, top: glCanvas.height - h };
  }

  // ------------------------------------------------------------ the cards
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
      if (!state.dist) state.dist = fits[0];
    }
    const out = renderTo(state.id, state.cls, w, h, state.yaw, state.pitch, state.dist);
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
      aspect: null, dist: 0, distRest: 0, distTurn: 0, spinning: false,
    };
    if (!bufferFor(id, state.cls)) return false;
    mounted.set(canvas, state);
    canvas.setAttribute("aria-hidden", "true");
    return paint(canvas, state);
  }

  /// Mount everything under `root` that asked for a model. Called after each
  /// panel render, because the panels replace their own innerHTML and a mounted
  /// canvas does not survive that.
  function scan(where) {
    if (!init()) return 0;
    const host = where || document;
    const list = host.querySelectorAll ? host.querySelectorAll("canvas[data-kit3d]") : [];
    let n = 0;
    list.forEach((canvas) => {
      if (mounted.has(canvas)) return;
      const ok = mount(canvas, canvas.getAttribute("data-kit3d"),
        { cls: canvas.getAttribute("data-kit3d-class") || "" });
      if (ok) {
        n += 1;
        // The card, whatever the card turns out to be. The three named
        // selectors are the panel's own; anything else -- the bench in
        // tools/arsenal, a future card type -- gets its immediate parent, so a
        // container never has to know it is holding a model to make one turn.
        const holder = canvas.closest("[data-class-mark], .work-card, .factory-ledger-row")
          || canvas.parentElement;
        if (holder) holder.classList.add("has-kit3d");
      } else {
        canvas.remove();
      }
    });
    // Anything that scrolled away and was replaced is no longer in the document.
    if (mounted.size > 400) {
      mounted.forEach((v, k) => { if (!k.isConnected) mounted.delete(k); });
    }
    return n;
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

  root.Arsenal3D = {
    mount, scan, dataURL, renderTo,
    get available() { return init(); },
    register,
    canvasHtml(id, cls) {
      // Provider ids carry ':' '/' '.' and '-'. Everything else still goes, so
      // this stays an attribute value that cannot break out of its quotes.
      return `<canvas class="kit3d" data-kit3d="${String(id).replace(/[^a-z0-9_:/.-]/gi, "")}"`
        + ` data-kit3d-class="${String(cls || "").replace(/[^a-z]/gi, "")}"></canvas>`;
    },
  };
})(typeof window !== "undefined" ? window : globalThis);
