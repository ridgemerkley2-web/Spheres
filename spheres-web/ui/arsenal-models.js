// The equipment deck as geometry. One low-poly mesh for every kit id in
// `spheres-sim/src/arsenal.rs`'s DECK, built in code rather than loaded.
//
// WHY BUILT AND NOT LOADED. This page has no build step, no CDN and no
// third-party code (CLAUDE.md, and the guard tests in main.rs that enforce it),
// so a glTF loader plus forty-six binary payloads was never on the table. What
// IS on the table is the same trick mapgen.rs plays with the map: author the
// thing procedurally, ship the recipe, and let the client bake it. A tank here
// costs about forty lines of source and nothing on the wire.
//
// WHY THIS FILE HAS NO DOM IN IT. Geometry is the half that has to be right;
// drawing it is the half that has to be fast. They are separated so the meshes
// can be built and checked outside a browser — `node tools/arsenal-obj.js`
// exports every model to Wavefront OBJ using exactly this file, and the Rust
// suite asserts this file covers the deck. arsenal3d.js owns the GL context and
// imports nothing but the buffers this returns.
//
// MODEL SPACE, and it is the same for all forty-six: +X right, +Y up, +Z the
// direction the thing points (nose, bow, muzzle, gun front). Units are metres,
// taken from the real vehicle where the real vehicle is a real vehicle, so an
// F-15E next to an M1 is the size an F-15E is next to an M1. The renderer
// normalises to a unit box for a card; the metres are kept so a future map
// layer can place them against a terrain scale instead of re-guessing.
//
// SHADING IS A DECISION PER PART, not a setting for the file. Every triangle
// still derives its normal from its own winding by default, because faceting is
// the look for a Nighthawk, a Spirit and a Sentinel and those three silhouettes
// are the aircraft. What has changed is that a part may ask to be smooth:
// `Mesh.soft` puts its triangles in one smoothing group and `finish` averages
// their face normals where they meet, so a fuselage, a wing section, a nozzle
// and a radome read as the curved objects they are instead of as prisms. Hard
// edges — chines, facets, chamfers, control-surface gaps — stay outside the
// group and stay hard, which is the whole reason it is opt-in.
//
// TWO LEVELS OF DETAIL, one recipe. `build(id, cls)` is the catalogue mesh, in
// the thousands of triangles a card can afford; `build(id, cls, {lod:"far"})`
// is the same model with coarser rings and none of the greebles, for a map pin.
// Nothing about the first signature moved.
(function (root) {
  "use strict";

  const DEG = Math.PI / 180;

  // ---------------------------------------------------------------- matrices
  // Row-major 4x4, applied as v' = M v. Row-major because the only reader is
  // `xf` below and it is easier to check by eye than a column-major one; the
  // renderer builds its own projection separately and never sees these.
  const IDENT = [1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1];

  function mul(a, b) {
    const o = new Array(16);
    for (let r = 0; r < 4; r += 1) {
      for (let c = 0; c < 4; c += 1) {
        o[r * 4 + c] = a[r * 4] * b[c] + a[r * 4 + 1] * b[4 + c]
          + a[r * 4 + 2] * b[8 + c] + a[r * 4 + 3] * b[12 + c];
      }
    }
    return o;
  }
  function mTranslate(x, y, z) { return [1, 0, 0, x, 0, 1, 0, y, 0, 0, 1, z, 0, 0, 0, 1]; }
  function mScale(x, y, z) { return [x, 0, 0, 0, 0, y, 0, 0, 0, 0, z, 0, 0, 0, 0, 1]; }
  function mRotX(d) {
    const c = Math.cos(d * DEG), s = Math.sin(d * DEG);
    return [1, 0, 0, 0, 0, c, -s, 0, 0, s, c, 0, 0, 0, 0, 1];
  }
  function mRotY(d) {
    const c = Math.cos(d * DEG), s = Math.sin(d * DEG);
    return [c, 0, s, 0, 0, 1, 0, 0, -s, 0, c, 0, 0, 0, 0, 1];
  }
  function mRotZ(d) {
    const c = Math.cos(d * DEG), s = Math.sin(d * DEG);
    return [c, -s, 0, 0, s, c, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1];
  }

  // ------------------------------------------------------------------ colour
  // Hex in, [r,g,b] out, and a shade() that darkens or lightens a face so a
  // single-material vehicle still reads as a vehicle. Everything is authored in
  // sRGB by eye; the shader does not gamma-correct, which is the right call for
  // a 90px card where the alternative is washed-out mud.
  function rgb(hex) {
    return [((hex >> 16) & 255) / 255, ((hex >> 8) & 255) / 255, (hex & 255) / 255];
  }
  function shade(c, k) {
    return [Math.min(1, c[0] * k), Math.min(1, c[1] * k), Math.min(1, c[2] * k)];
  }

  // The palette. Six service finishes plus the details that sit on all of them.
  // NATO green and Soviet olive are deliberately different so a legacy tank and
  // a modern one are not the same object in two sizes.
  const P = {
    olive: rgb(0x4d5540),        // legacy ground kit, Warsaw-Pact green
    green: rgb(0x3f4a38),        // modern ground kit
    tan: rgb(0x9a8a68),          // desert armour
    track: rgb(0x2b2c28),
    grey: rgb(0x8e9aa4),         // air-superiority grey
    greyDark: rgb(0x5f6b76),
    stealth: rgb(0x33373c),      // low-observable charcoal
    stealthLit: rgb(0x474c53),
    navy: rgb(0x6d7883),         // haze grey
    navyDeck: rgb(0x3d444b),
    hull: rgb(0x2f3439),         // below the waterline / submarine black
    white: rgb(0xd8dde2),
    canopy: rgb(0x2a4b63),
    glass: rgb(0x38607a),
    metal: rgb(0xa8aeb4),
    exhaust: rgb(0x54585c),
    warhead: rgb(0xb0b6bc),
    missile: rgb(0xcfd4d8),
    red: rgb(0x9d3b32),
    gold: rgb(0xb08a3c),         // satellite foil
    panel: rgb(0x1f3f6b),        // solar cell
    skin: rgb(0x8a6a4e),
    cloth: rgb(0x5c6148),
    black: rgb(0x1c1e21),
    sensor: rgb(0x232a30),
    rubber: rgb(0x26282b),
    rust: rgb(0x6b5a44),
  };

  // ------------------------------------------------------------------- build
  // A mesh is a triangle soup with a per-vertex colour and a transform stack.
  // Normals are NOT accumulated here. They are derived per triangle at the end
  // (`finish`), from the vertices as transformed — which means a mirrored or
  // non-uniformly scaled part gets the normal its geometry actually has rather
  // than one carried over from the primitive's local space. That single choice
  // removes the whole class of inverted-normal bugs that comes with mirroring,
  // and mirroring is how half of these models are built.
  function Mesh(lod) {
    this.pos = [];
    this.col = [];
    // One smoothing group id per triangle, 0 meaning "keep your own face
    // normal". See `soft` below; the default is still 0, so a part that says
    // nothing is flat-shaded exactly as it was.
    this.grp = [];
    this.group = 0;
    this.groups = 0;
    // Which level of detail is being built. `far` is the map-and-sprite mesh:
    // same recipe, coarser rings, and the greebles that only exist to be seen
    // at card size are skipped rather than shrunk.
    this.far = lod === "far";
    this.m = IDENT.slice();
    this.stack = [];
  }
  /// The number this level of detail wants. Written as one call so a segment
  /// count reads as a single decision — `m.lod(24, 8)` — instead of an `if`
  /// wrapped round every ring in the file.
  Mesh.prototype.lod = function (near, far) { return this.far ? far : near; };

  /// SMOOTH SHADING, AND WHY IT IS OPT-IN. The file header's original claim —
  /// faceting IS the look — is true of a Nighthawk and false of a Sentry. A
  /// fuselage, a wing section, a nozzle, a radome and a wheel are curved
  /// objects, and one normal per triangle turns every one of them into a prism.
  /// `soft` marks every triangle emitted inside it as belonging to ONE
  /// smoothing group; `finish` averages those triangles' face normals wherever
  /// they share a corner and hands the average to all three vertices. The
  /// averaging is by UNNORMALISED face normal, so it is area-weighted for free
  /// and a sliver cannot shout down the panel beside it.
  ///
  /// It is deliberately not global. A chine, a facet, a chamfer and a
  /// control-surface gap are hard edges that a global smooth would erase, and
  /// erasing them would cost the F-117, the B-2 and the RQ-170 their identity.
  Mesh.prototype.soft = function (fn) {
    const prev = this.group;
    this.groups += 1;
    this.group = this.groups;
    fn(this);
    this.group = prev;
    return this;
  };
  Mesh.prototype.save = function () { this.stack.push(this.m.slice()); return this; };
  Mesh.prototype.restore = function () { this.m = this.stack.pop() || IDENT.slice(); return this; };
  Mesh.prototype.move = function (x, y, z) { this.m = mul(this.m, mTranslate(x, y, z)); return this; };
  Mesh.prototype.scale = function (x, y, z) {
    this.m = mul(this.m, mScale(x, y == null ? x : y, z == null ? x : z));
    return this;
  };
  Mesh.prototype.rotX = function (d) { this.m = mul(this.m, mRotX(d)); return this; };
  Mesh.prototype.rotY = function (d) { this.m = mul(this.m, mRotY(d)); return this; };
  Mesh.prototype.rotZ = function (d) { this.m = mul(this.m, mRotZ(d)); return this; };

  Mesh.prototype.xf = function (p) {
    const m = this.m, x = p[0], y = p[1], z = p[2];
    return [
      m[0] * x + m[1] * y + m[2] * z + m[3],
      m[4] * x + m[5] * y + m[6] * z + m[7],
      m[8] * x + m[9] * y + m[10] * z + m[11],
    ];
  };

  Mesh.prototype.tri = function (a, b, c, col) {
    const flip = this.detSign() < 0;
    const A = this.xf(a), B = this.xf(flip ? c : b), C = this.xf(flip ? b : c);
    this.pos.push(A[0], A[1], A[2], B[0], B[1], B[2], C[0], C[1], C[2]);
    for (let i = 0; i < 3; i += 1) this.col.push(col[0], col[1], col[2]);
    this.grp.push(this.group);
    return this;
  };
  Mesh.prototype.quad = function (a, b, c, d, col) {
    return this.tri(a, b, c, col).tri(a, c, d, col);
  };
  /// A convex polygon as a fan. Concave planforms are not authored anywhere in
  /// this file; where one was wanted (the B-2's sawtooth trailing edge) it is
  /// built as several convex plates instead, which is also how it is panelled.
  Mesh.prototype.fan = function (pts, col) {
    for (let i = 1; i + 1 < pts.length; i += 1) this.tri(pts[0], pts[i], pts[i + 1], col);
    return this;
  };

  /// Mirroring is how a wing, a tail, a track or a hull side gets its other
  /// half, and a mirror REVERSES WINDING — the same three vertices that faced
  /// out now face in. Rather than ask forty-six models to remember that, `tri`
  /// checks the sign of the transform's determinant and swaps two vertices when
  /// it is negative. `both` can then be a two-line function and every part
  /// authored for the right-hand side is correct on the left.
  Mesh.prototype.detSign = function () {
    const m = this.m;
    const d = m[0] * (m[5] * m[10] - m[6] * m[9])
      - m[1] * (m[4] * m[10] - m[6] * m[8])
      + m[2] * (m[4] * m[9] - m[5] * m[8]);
    return d < 0 ? -1 : 1;
  };
  Mesh.prototype.both = function (fn) {
    fn(this, 1);
    this.save().scale(-1, 1, 1);
    fn(this, -1);
    this.restore();
    return this;
  };

  // ------------------------------------------------------------- primitives
  /// An axis-aligned box by extents. Extents rather than centre-and-size
  /// because almost every part here is positioned by a face it must sit flush
  /// against — a hull roof, a deck, a waterline — and extents say that directly.
  Mesh.prototype.bar = function (x0, x1, y0, y1, z0, z1, col) {
    const t = shade(col, 1.06), s = shade(col, 0.9), u = shade(col, 0.78);
    this.quad([x1, y0, z0], [x1, y1, z0], [x1, y1, z1], [x1, y0, z1], s);
    this.quad([x0, y0, z0], [x0, y0, z1], [x0, y1, z1], [x0, y1, z0], s);
    this.quad([x0, y1, z0], [x0, y1, z1], [x1, y1, z1], [x1, y1, z0], t);
    this.quad([x0, y0, z0], [x1, y0, z0], [x1, y0, z1], [x0, y0, z1], u);
    this.quad([x0, y0, z1], [x1, y0, z1], [x1, y1, z1], [x0, y1, z1], col);
    this.quad([x0, y0, z0], [x0, y1, z0], [x1, y1, z0], [x1, y0, z0], u);
    return this;
  };

  /// A ring of points at one station. Radii may differ in x and y, which is
  /// what makes a fuselage oval and a submarine round from the same call.
  function ring(rx, ry, seg) {
    const pts = [];
    for (let i = 0; i < seg; i += 1) {
      const t = (i / seg) * Math.PI * 2;
      pts.push([Math.cos(t) * rx, Math.sin(t) * ry]);
    }
    return pts;
  }

  /// Sections lofted along +Z. Every section carries the same point count, so
  /// a fuselage is written as a handful of stations and the sides come out
  /// closed by construction. `caps` closes the ends with fans.
  ///
  /// ONE PANEL OF THE RING MAY BE DECLARED HARD, and the argument is optional
  /// so every caller written before it reads unchanged. `hard` is the contour
  /// index whose panel keeps its own face normal even when the loft is inside a
  /// smoothing group — the trailing edge of an aerofoil is the only one in the
  /// file, and it needs it: upper and lower surfaces meet there at about ten
  /// degrees, so averaging across the fold makes the two sides cancel and the
  /// residue point INTO the wing. Every triangle along every trailing edge then
  /// shades as if lit from underneath, which is a dark seam down the one line
  /// of a wing a card actually reads. The panel is emitted in its original
  /// place in the order, so nothing but its smoothing group changes.
  Mesh.prototype.loft = function (sections, col, caps, hard) {
    const n = sections[0].pts.length;
    for (let s = 0; s + 1 < sections.length; s += 1) {
      const a = sections[s], b = sections[s + 1];
      for (let i = 0; i < n; i += 1) {
        const j = (i + 1) % n;
        const lit = 1 + 0.16 * Math.sin((i / n) * Math.PI * 2 + 1.2);
        const held = hard === i ? this.group : -1;
        if (held >= 0) this.group = 0;
        this.quad(
          [a.pts[i][0], a.pts[i][1], a.z], [a.pts[j][0], a.pts[j][1], a.z],
          [b.pts[j][0], b.pts[j][1], b.z], [b.pts[i][0], b.pts[i][1], b.z],
          shade(col, lit),
        );
        if (held >= 0) this.group = held;
      }
    }
    if (caps !== false) {
      const f = sections[0], l = sections[sections.length - 1];
      this.fan(f.pts.map((p) => [p[0], p[1], f.z]).slice().reverse(), shade(col, 0.8));
      this.fan(l.pts.map((p) => [p[0], p[1], l.z]), shade(col, 0.86));
    }
    return this;
  };

  /// A cone, cylinder or tube along +Z, from z=0 to z=len.
  Mesh.prototype.tube = function (r0, r1, len, seg, col, caps) {
    return this.loft([
      { z: 0, pts: ring(r0 || 1e-4, r0 || 1e-4, seg) },
      { z: len, pts: ring(r1 || 1e-4, r1 || 1e-4, seg) },
    ], col, caps);
  };

  Mesh.prototype.ball = function (r, seg, rings, col) {
    const sec = [];
    for (let i = 0; i <= rings; i += 1) {
      const a = -Math.PI / 2 + (i / rings) * Math.PI;
      const rr = Math.max(1e-4, Math.cos(a) * r);
      sec.push({ z: Math.sin(a) * r, pts: ring(rr, rr, seg) });
    }
    return this.loft(sec, col, false);
  };

  /// A flat planform extruded in Y: wings, tails, fins, rudders, blades, solar
  /// panels, deck plates. Points are [x, z], ordered counter-clockwise as
  /// plotted with x right and z up. Convex only — see `fan`.
  Mesh.prototype.plate = function (pts, thick, col) {
    const h = thick / 2;
    // ORIENTATION IS FIXED HERE, NOT ASKED OF THE CALLER. A wing planform is
    // naturally written leading edge first — nose, tip, tip trailing, root
    // trailing — which traces clockwise, while a deck outline is usually
    // written counter-clockwise, and neither author should have to know which
    // way the normals then point. The signed area says which one this is, and
    // a clockwise list is reversed before anything is emitted, so the faces
    // come out with the top on top in both cases and the exported OBJ is right.
    let area = 0;
    for (let i = 0; i < pts.length; i += 1) {
      const j = (i + 1) % pts.length;
      area += pts[i][0] * pts[j][1] - pts[j][0] * pts[i][1];
    }
    const ordered = area < 0 ? pts.slice().reverse() : pts;
    const top = ordered.map((p) => [p[0], h, p[1]]);
    const bot = ordered.map((p) => [p[0], -h, p[1]]);
    this.fan(top.slice().reverse(), shade(col, 1.08));
    this.fan(bot, shade(col, 0.82));
    for (let i = 0; i < ordered.length; i += 1) {
      const j = (i + 1) % ordered.length;
      this.quad(bot[j], bot[i], top[i], top[j], shade(col, 0.94));
    }
    return this;
  };

  /// A wing panel with the four numbers a wing actually has. Sweep is the
  /// leading edge's rearward travel across the span, which is how a planform is
  /// quoted and how it is recognised: nearly zero on a Predator, most of the
  /// root chord on a delta.
  Mesh.prototype.wing = function (o, col) {
    const span = o.span, root = o.root;
    const tip = o.tip == null ? root * 0.4 : o.tip;
    const sweep = o.sweep == null ? root * 0.5 : o.sweep;
    const thick = o.thick == null ? 0.12 : o.thick;
    this.save().move(o.x0 || 0, o.y || 0, o.z || 0).rotZ(-(o.dihedral || 0));
    this.plate([[0, 0], [span, -sweep], [span, -sweep - tip], [0, -root]], thick, col);
    this.restore();
    return this;
  };

  /// A plate with its rim chamfered off, and the reason it exists is light. A
  /// flat plate's edge is one quad standing at ninety degrees to both faces, so
  /// under a key light it is either black or blown out and never reads as an
  /// edge at all. A chamfer puts two narrow bands at grazing angles there, and
  /// those bands are what make a door, an access panel, a control surface or a
  /// gear door look machined instead of printed on. Convex only, like `plate`.
  Mesh.prototype.slab = function (pts, thick, cham, col) {
    let area = 0;
    for (let i = 0; i < pts.length; i += 1) {
      const j = (i + 1) % pts.length;
      area += pts[i][0] * pts[j][1] - pts[j][0] * pts[i][1];
    }
    const ord = area < 0 ? pts.slice().reverse() : pts;
    let cx = 0, cy = 0;
    for (const p of ord) { cx += p[0] / ord.length; cy += p[1] / ord.length; }
    // A CHAMFER CANNOT BE DEEPER THAN THE PLATE IS HALF THICK. Asked for one
    // that is — a thin control surface whose hinge gap sets the chamfer, which
    // is exactly what `liftSurface` does — the upright band of the rim closed
    // to nothing and shipped eight zero-area triangles per plate that draw no
    // pixels and still cost their place in the budget. Clamping is also the
    // honest geometry: past the half-thickness it has stopped being a chamfer
    // and become a knife, and leaving a fifth of the rim upright is what keeps
    // the two grazing bands reading as an edge rather than a fold.
    const h = thick / 2, cm = Math.min(cham, h * 0.8), hi = h - cm;
    // Each rim point walks the chamfer toward the centroid — a fixed distance
    // rather than a fixed fraction, so a long thin control surface gets the
    // same chamfer width at its root as at its tip.
    const inset = ord.map((p) => {
      const dx = cx - p[0], dy = cy - p[1], d = Math.hypot(dx, dy) || 1;
      const s = Math.min(0.72, cm / d);
      return [p[0] + dx * s, p[1] + dy * s];
    });
    const top = inset.map((p) => [p[0], h, p[1]]), bot = inset.map((p) => [p[0], -h, p[1]]);
    const rimT = ord.map((p) => [p[0], hi, p[1]]), rimB = ord.map((p) => [p[0], -hi, p[1]]);
    this.fan(top.slice().reverse(), shade(col, 1.08));
    this.fan(bot, shade(col, 0.82));
    for (let i = 0; i < ord.length; i += 1) {
      const j = (i + 1) % ord.length;
      this.quad(rimT[j], rimT[i], top[i], top[j], shade(col, 1.02));
      this.quad(rimB[j], rimB[i], rimT[i], rimT[j], shade(col, 0.94));
      this.quad(bot[j], bot[i], rimB[i], rimB[j], shade(col, 0.87));
    }
    return this;
  };

  /// A symmetric aerofoil section: NACA four-digit thickness law, closed, in
  /// [distance aft of the leading edge, thickness]. Cosine spacing puts the
  /// points where the curvature is — at the leading edge — because that is the
  /// one part of a wing section a 90px card can resolve, and it is what turns
  /// the edge from a knife into something that catches the key light.
  function foilRing(chord, thick, n) {
    const half = (x) => 5 * thick * chord * (0.2969 * Math.sqrt(x) - 0.1260 * x
      - 0.3516 * x * x + 0.2843 * x * x * x - 0.1015 * x * x * x * x);
    const xs = [];
    for (let i = 0; i <= n; i += 1) xs.push(0.5 - 0.5 * Math.cos((i / n) * Math.PI));
    const pts = [];
    // Lower surface first, leading edge to trailing, then the upper surface
    // back: counter-clockwise as plotted, which is what `loft` wants for
    // outward faces.
    for (let i = 0; i <= n; i += 1) pts.push([xs[i] * chord, -half(xs[i])]);
    for (let i = n - 1; i >= 1; i -= 1) pts.push([xs[i] * chord, half(xs[i])]);
    return pts;
  }

  /// A lifting surface as a LOFTED AEROFOIL rather than an extruded outline,
  /// and this is the single largest change this pass makes to an aircraft. The
  /// planform arguments are `wing`'s, unchanged, so a table written for one
  /// reads for the other: root leading edge at (x0, y, z), span outboard in +x,
  /// chord running aft in -z, sweep the leading edge's rearward travel.
  ///
  /// Built as sections along local +z after `rotY(90)`, which maps span to +x
  /// and chord aft to -z. The two ribs are emitted outside the smoothing group
  /// so the tip stays a hard edge — a wing whose tip rib is smoothed reads as a
  /// blob, and the tip is where a card sees the wing's thickness.
  Mesh.prototype.foil = function (o, col) {
    const n = o.n || this.lod(9, 3), bays = o.bays || this.lod(6, 1);
    const thick = o.thick == null ? 0.1 : o.thick;
    const secs = [];
    for (let i = 0; i <= bays; i += 1) {
      const t = i / bays;
      const chord = Math.max(1e-3, o.root + ((o.tip == null ? o.root * 0.4 : o.tip) - o.root) * t);
      const aft = (o.sweep == null ? o.root * 0.5 : o.sweep) * t;
      // Thickness/chord falls outboard, as it does on every wing built since
      // 1950: the tip is the thin end structurally and visually.
      const tr = thick * (1 - 0.28 * t);
      secs.push({ z: (o.span || 1) * t, pts: foilRing(chord, tr, n).map((p) => [p[0] + aft, p[1]]) });
    }
    // Dihedral is a rotation of the whole panel in the world frame and
    // incidence is a rotation of the SECTION about the span axis — which is
    // the innermost rotZ, after rotY(90) has already turned local +z into
    // span. Positive incidence puts the leading edge down, which is how a
    // trimmed stabiliser sits and how a propeller blade is pitched.
    this.save().move(o.x0 || 0, o.y || 0, o.z || 0)
      .rotZ(-(o.dihedral || 0)).rotY(90).rotZ(o.incidence || 0);
    // The contour is one continuous curve — trailing edge, forward along the
    // lower surface, round the leading edge, back along the upper — and it is
    // smooth everywhere except where it closes. `foilRing` lays the lower
    // surface down as indices 0..n, so index n is the trailing edge and panel n
    // is the wedge that shuts it: the one hard edge on a wing, and the one the
    // smoothing group must not average across. See `loft`'s note on `hard`.
    this.soft((mm) => mm.loft(secs, col, false, n));
    const root = secs[0], tip = secs[secs.length - 1];
    this.fan(root.pts.map((p) => [p[0], p[1], root.z]).slice().reverse(), shade(col, 0.84));
    this.fan(tip.pts.map((p) => [p[0], p[1], tip.z]), shade(col, 0.9));
    this.restore();
    return this;
  };

  /// The whole surface: the fixed box, its separate control surfaces with real
  /// hinge gaps, and an optional fence. Control surfaces are geometry a card
  /// can actually show — the shadow line of a hinge gap is visible at 90px
  /// where a panel line is not — and they are the difference between a wing and
  /// a wing-shaped object. `ctrl` is the aft fraction of chord they take.
  Mesh.prototype.liftSurface = function (o, col) {
    const cf = o.ctrl == null ? 0.24 : o.ctrl;
    const span = o.span, root = o.root, tip = o.tip == null ? root * 0.4 : o.tip;
    const sweep = o.sweep == null ? root * 0.5 : o.sweep;
    this.foil({
      x0: o.x0, y: o.y, z: o.z, span, root: root * (1 - cf), tip: tip * (1 - cf),
      sweep, dihedral: o.dihedral, thick: o.thick, n: o.n, bays: o.bays,
      incidence: o.incidence,
    }, col);
    if (this.far || !o.surfaces) return this;
    const gap = o.gap == null ? Math.min(0.09, root * 0.02) : o.gap;
    const chordAt = (t) => root + (tip - root) * t;
    this.save().move(o.x0 || 0, o.y || 0, o.z || 0).rotZ(-(o.dihedral || 0));
    o.surfaces.forEach((s) => {
      const hinge = (t) => -(sweep * t + chordAt(t) * (1 - cf) + gap);
      const back = (t) => -(sweep * t + chordAt(t));
      const mid = hinge((s[0] + s[1]) / 2);
      this.save().move(0, 0, mid).rotX(s[2] || 0).move(0, 0, -mid);
      this.slab([
        [span * s[0], hinge(s[0])], [span * s[1], hinge(s[1])],
        [span * s[1], back(s[1])], [span * s[0], back(s[0])],
      ], (o.thick || 0.1) * chordAt((s[0] + s[1]) / 2) * 0.62, gap * 0.5, shade(col, 0.97));
      this.restore();
    });
    this.restore();
    // A fence is a small plate standing on the upper surface, chordwise, at one
    // spanwise station. rotZ(90) stands the planform on edge — the same trick
    // `jet` plays with a fin — so the outline is written [height, chord].
    if (o.fence != null) {
      const t = o.fence, c = chordAt(t), h = c * 0.12;
      this.save().move((o.x0 || 0) + span * t, o.y || 0, o.z || 0)
        .rotZ(-(o.dihedral || 0)).move(0, 0, -sweep * t).rotZ(90);
      this.slab([[0, c * 0.02], [h, -c * 0.06], [h, -c * 0.5], [0, -c * 0.62]],
        (o.thick || 0.1) * c * 0.3, (o.thick || 0.1) * c * 0.08, shade(col, 1.06));
      this.restore();
    }
    return this;
  };

  /// Triangle soup to buffers. Normals are computed here, per face, from the
  /// vertices AS TRANSFORMED — see the note on `Mesh`. A degenerate triangle
  /// (a cone tip, a zero-chord tip rib) gets +Y rather than a NaN, because one
  /// NaN in an attribute buffer takes the whole draw call with it.
  Mesh.prototype.finish = function () {
    const positions = new Float32Array(this.pos);
    const colors = new Float32Array(this.col);
    const normals = new Float32Array(positions.length);
    const min = [Infinity, Infinity, Infinity], max = [-Infinity, -Infinity, -Infinity];
    const faces = this.groups > 0 ? new Float64Array(positions.length / 3) : null;
    for (let i = 0; i < positions.length; i += 9) {
      const ax = positions[i], ay = positions[i + 1], az = positions[i + 2];
      const ux = positions[i + 3] - ax, uy = positions[i + 4] - ay, uz = positions[i + 5] - az;
      const vx = positions[i + 6] - ax, vy = positions[i + 7] - ay, vz = positions[i + 8] - az;
      let nx = uy * vz - uz * vy, ny = uz * vx - ux * vz, nz = ux * vy - uy * vx;
      if (faces) { const f = i / 3; faces[f] = nx; faces[f + 1] = ny; faces[f + 2] = nz; }
      const len = Math.hypot(nx, ny, nz);
      if (len > 1e-12) { nx /= len; ny /= len; nz /= len; } else { nx = 0; ny = 1; nz = 0; }
      for (let k = 0; k < 3; k += 1) {
        normals[i + k * 3] = nx; normals[i + k * 3 + 1] = ny; normals[i + k * 3 + 2] = nz;
      }
    }
    // The smoothing pass. Two triangles share a vertex when they are in the
    // SAME group and put a corner in the same place to a hundredth of a
    // millimetre — a tolerance far below anything authored here and far above
    // float32's step at these sizes, so the join is exact for vertices that
    // came out of the same arithmetic and never guesses for ones that did not.
    // The key carries the group id, so two parts that touch stay two parts.
    if (faces) {
      const q = (v) => { const k = Math.round(v * 1e5); return k === 0 ? 0 : k; };
      const acc = new Map();
      for (let t = 0; t < this.grp.length; t += 1) {
        const g = this.grp[t];
        if (!g) continue;
        const f = t * 9;
        for (let c = 0; c < 3; c += 1) {
          const i = f + c * 3;
          const key = `${g}|${q(positions[i])}|${q(positions[i + 1])}|${q(positions[i + 2])}`;
          const sum = acc.get(key);
          if (sum) { sum[0] += faces[f / 3]; sum[1] += faces[f / 3 + 1]; sum[2] += faces[f / 3 + 2]; }
          else acc.set(key, [faces[f / 3], faces[f / 3 + 1], faces[f / 3 + 2]]);
        }
      }
      for (let t = 0; t < this.grp.length; t += 1) {
        if (!this.grp[t]) continue;
        const f = t * 9, fn = t * 3;
        const fl = Math.hypot(faces[fn], faces[fn + 1], faces[fn + 2]);
        for (let c = 0; c < 3; c += 1) {
          const i = f + c * 3;
          const key = `${this.grp[t]}|${q(positions[i])}|${q(positions[i + 1])}|${q(positions[i + 2])}`;
          const sum = acc.get(key);
          const len = Math.hypot(sum[0], sum[1], sum[2]);
          // A cone tip, or two faces that cancel exactly, leaves nothing to
          // average. Keep the face normal there rather than emit a zero: one
          // non-unit normal in the buffer is a black triangle on the card.
          if (len <= 1e-12) continue;
          // AND A CORNER MAY NOT BE HANDED A NORMAL ITS OWN FACE POINTS AWAY
          // FROM. Where a smoothing group closes round a fold of nearly half a
          // turn — the nose of a wing panel swept back further than its own
          // chord, where six faces meet a leading edge three up and three down
          // — the up and down faces cancel and what survives is the residue,
          // which lies flat along the surface or behind it. Every triangle
          // there then shades as though the light were under the wing. The
          // average has to be in the same half-space as the face wearing it or
          // it is not a smoothed normal, it is noise, so that corner keeps the
          // face normal instead. The test is against zero rather than some
          // chosen crease angle on purpose: a coarse ring smoothed on a map-LOD
          // cylinder sits at 0.5 to 0.8 here and must not be broken up, while
          // a genuine cancellation lands at or below nothing. This can only
          // ever replace a normal that already contradicted its geometry.
          if (fl > 1e-12
            && sum[0] * faces[fn] + sum[1] * faces[fn + 1] + sum[2] * faces[fn + 2] <= 0) continue;
          normals[i] = sum[0] / len; normals[i + 1] = sum[1] / len; normals[i + 2] = sum[2] / len;
        }
      }
    }
    for (let i = 0; i < positions.length; i += 3) {
      for (let k = 0; k < 3; k += 1) {
        if (positions[i + k] < min[k]) min[k] = positions[i + k];
        if (positions[i + k] > max[k]) max[k] = positions[i + k];
      }
    }
    return {
      positions, normals, colors,
      count: positions.length / 3,
      min, max,
      size: [max[0] - min[0], max[1] - min[1], max[2] - min[2]],
      centre: [(max[0] + min[0]) / 2, (max[1] + min[1]) / 2, (max[2] + min[2]) / 2],
    };
  };

  /// A body as a run of stations along +Z: `[t, widthFactor, heightFactor]`,
  /// t running 0 at the tail to 1 at the nose. Fuselages, ship hulls, missile
  /// bodies and submarine pressure hulls are all this call with a different
  /// table, which is why there is one of it and not four.
  ///
  /// TWO OPTIONAL KNOBS, BOTH OFF BY DEFAULT, so every table already written
  /// against this call still produces the same triangles it always did.
  /// `sharp` swaps the ellipse for a superellipse of that exponent, which is
  /// how a chined fuselage — flat sides, flat deck, a hard corner between them
  /// — comes out of the same loft as a round one. `soft` says the body is a
  /// curved surface and its normals should be averaged; a chined body says
  /// nothing and keeps its facets.
  function ringSuper(rx, ry, seg, e) {
    const pts = [];
    for (let i = 0; i < seg; i += 1) {
      const t = (i / seg) * Math.PI * 2;
      const c = Math.cos(t), s = Math.sin(t);
      const k = Math.pow(Math.pow(Math.abs(c), e) + Math.pow(Math.abs(s), e), -1 / e);
      pts.push([c * k * rx, s * k * ry]);
    }
    return pts;
  }
  function bodyLoft(m, o) {
    const seg = o.seg || 10;
    const secs = o.stations.map((s) => {
      const rx = Math.max(1e-3, s[1] * o.w / 2);
      const ry = Math.max(1e-3, (s[2] == null ? s[1] : s[2]) * (o.h == null ? o.w : o.h) / 2);
      return { z: s[0] * o.len, pts: o.sharp ? ringSuper(rx, ry, seg, o.sharp) : ring(rx, ry, seg) };
    });
    if (o.soft) m.soft((mm) => mm.loft(secs, o.col, o.caps));
    else m.loft(secs, o.col, o.caps);
  }

  // ------------------------------------------------------------- aircraft kit
  // WHAT BUYS REALISM IN A RENDERER WITH NO TEXTURES. Everything below is
  // geometry or vertex colour, because that is all there is: no UVs, no maps,
  // no decals. So a panel is a panel, a hinge gap is a gap, a nozzle has
  // petals, and an intake has a duct you can see down. Each of these is small;
  // together they are the difference between a shape and an aircraft.

  /// A skin panel, hatch or service door, laid on a body of revolution at the
  /// station and clock angle given. Sitting slightly proud with a chamfered
  /// rim, because a chamfer is what catches the key light — an inset rectangle
  /// with no thickness is invisible from every angle but one.
  function skinPanel(m, o) {
    const t = o.th * DEG, rx = o.rx, ry = o.ry;
    const d = o.depth == null ? Math.min(rx, ry) * 0.05 : o.depth;
    // The outward normal of an ellipse is not the radius: it is (cos/rx,
    // sin/ry) normalised, and using the radius instead is what makes a panel
    // hover off a flat-sided body.
    const phi = Math.atan2(Math.sin(t) / ry, Math.cos(t) / rx) / DEG;
    m.save().move(Math.cos(t) * rx, Math.sin(t) * ry, o.z).rotZ(phi - 90).move(0, d * 0.2, 0);
    m.slab([[-o.w / 2, -o.len / 2], [o.w / 2, -o.len / 2], [o.w / 2, o.len / 2],
      [-o.w / 2, o.len / 2]], d, d * 0.35, o.col);
    m.restore();
  }
  /// A run of them: the line of access panels down a fuselage side, which on a
  /// real airframe is where the avionics bays are and on a card is what tells
  /// the eye how long the thing is.
  function panelRun(m, o) {
    const n = o.n || 4;
    for (let i = 0; i < n; i += 1) {
      const f = n === 1 ? 0.5 : i / (n - 1);
      skinPanel(m, {
        rx: o.rx, ry: o.ry, th: o.th, z: o.z0 + (o.z1 - o.z0) * f,
        w: o.w, len: o.len, depth: o.depth,
        // A hair of variation between panels, so a row of them is a row of
        // panels and not a comb. Deterministic: it is a function of the index.
        col: shade(o.col, 1 + ((i % 3) - 1) * 0.035),
      });
    }
  }

  /// An exhaust can with its petals. A nozzle drawn as a smooth tube is a pipe;
  /// what the back of an afterburning engine actually shows is a ring of
  /// overlapping convergent flaps, each a flat facet at its own angle. Built as
  /// a fluted loft — alternate segments in and out — and left hard-edged,
  /// because those facets ARE the detail. Then the wall runs back down the
  /// throat to a dark disc, so the hole reads as a hole and not as a cap.
  function nozzle(m, o) {
    const seg = m.lod(18, 8), R = o.r, L = o.len;
    const flute = (r, amp) => {
      const pts = [];
      for (let i = 0; i < seg; i += 1) {
        const t = (i / seg) * Math.PI * 2;
        const f = r * (1 + amp * (i % 2 ? -1 : 1));
        pts.push([Math.cos(t) * f, Math.sin(t) * f]);
      }
      return pts;
    };
    m.save().move(o.x || 0, o.y || 0, o.z || 0);
    m.soft((mm) => mm.loft([
      { z: 0, pts: ring(R * 1.16, R * 1.16, seg) },
      { z: L * 0.28, pts: ring(R * 1.12, R * 1.12, seg) },
      { z: L * 0.45, pts: ring(R * 1.04, R * 1.04, seg) },
    ], P.exhaust, false));
    m.loft([{ z: L * 0.45, pts: flute(R * 1.02, 0.015) },
      { z: L * 0.74, pts: flute(R * 0.94, 0.045) },
      { z: L, pts: flute(R * 0.86, 0.075) }], shade(P.exhaust, 1.14), false);
    m.loft([{ z: L, pts: ring(R * 0.8, R * 0.8, seg) },
      { z: L * 0.5, pts: ring(R * 0.68, R * 0.68, seg) },
      { z: L * 0.1, pts: ring(R * 0.6, R * 0.6, seg) }], shade(P.exhaust, 0.45), false);
    m.save().move(0, 0, L * 0.1);
    m.fan(ring(R * 0.6, R * 0.6, seg).map((p) => [p[0], p[1], 0]), P.black);
    m.restore();
    m.restore();
  }

  /// An intake with a real mouth, facing +Z. A rectangular hole in the side of
  /// a fuselage is a black quad; what an intake has is a lip with thickness, a
  /// duct that goes somewhere, and — on everything but a Lightning — a splitter
  /// plate standing off the skin to keep the boundary layer out of the engine.
  /// The splitter is diagnostic: its absence is exactly what "diverterless"
  /// means, and it is the one thing about an F-35's intake a card can show.
  function intakeDuct(m, o) {
    const seg = m.lod(14, 8), W = o.w, H = o.h, L = o.len, e = o.sharp || 2.6;
    const rect = (k) => ringSuper(W / 2 * k, H / 2 * k, seg, e);
    m.save().move(o.x || 0, o.y || 0, o.z || 0).rotY(o.yaw || 0);
    // The cowl, from the fairing back on the fuselage forward to the lip.
    m.soft((mm) => mm.loft([
      { z: -L, pts: rect(0.86) }, { z: -L * 0.45, pts: rect(1.02) },
      { z: -L * 0.08, pts: rect(1.06) }, { z: 0, pts: rect(1.0) },
    ], o.col, false));
    // The lip rolls over and the duct runs in, narrowing. Two sections of wall
    // are enough for the eye to follow it into the dark.
    m.loft([{ z: 0, pts: rect(1.0) }, { z: -0.02 * L, pts: rect(0.88) }],
      shade(o.col, 1.2), false);
    m.soft((mm) => mm.loft([
      { z: -L * 0.02, pts: rect(0.86) }, { z: -L * 0.5, pts: rect(0.74) },
      { z: -L * 1.1, pts: rect(0.62) },
    ], shade(P.black, 1.5), false));
    m.save().move(0, 0, -L * 1.1);
    m.fan(rect(0.62).map((p) => [p[0], p[1], 0]), P.black);
    m.restore();
    if (o.splitter) {
      m.save().move(-o.splitter * (W / 2 + 0.02), 0, -L * 0.5).rotZ(90);
      m.slab([[-H * 0.44, L * 0.62], [H * 0.44, L * 0.55], [H * 0.44, -L * 0.5],
        [-H * 0.44, -L * 0.5]], W * 0.1, W * 0.03, shade(o.col, 0.86));
      m.restore();
    }
    m.restore();
  }

  /// A canopy: glass, frames and the rail they sit in. The glass is smoothed
  /// and the frames are not, which is the same decision the real thing makes in
  /// aluminium — and the frames are what stop a canopy reading as a blister.
  function canopyGlass(m, o) {
    const L = o.len, W = o.w, H = o.h;
    const seg = m.lod(14, 6);
    const stations = o.stations || [[0, .3, .28], [.1, .68, .66], [.24, .94, .92],
      [.42, 1, 1], [.62, .98, .94], [.8, .82, .74], [.92, .5, .48], [1, .16, .18]];
    m.save().move(o.x || 0, o.y, o.z);
    bodyLoft(m, { len: L, w: W, h: H, col: o.col || P.canopy, seg, soft: true, stations });
    if (!m.far) {
      // Frame bands: the same section a touch larger over a short run, which is
      // exactly what a canopy bow is. Two of them — the windscreen arch and the
      // aft bow — plus the sill rail down each side.
      (o.frames || [[.2, .26], [.74, .8]]).forEach((f) => {
        const at = (t) => {
          let a = stations[0], b = stations[stations.length - 1];
          for (let i = 0; i + 1 < stations.length; i += 1) {
            if (t >= stations[i][0] && t <= stations[i + 1][0]) { a = stations[i]; b = stations[i + 1]; }
          }
          const k = (t - a[0]) / Math.max(1e-6, b[0] - a[0]);
          return [t, a[1] + (b[1] - a[1]) * k, a[2] + (b[2] - a[2]) * k];
        };
        bodyLoft(m, { len: L, w: W * 1.07, h: H * 1.07, col: shade(o.frame || P.greyDark, 1.0),
          seg, stations: [at(f[0]), at(f[1])] });
      });
      m.both((mm) => {
        mm.save().move(W * 0.44, -H * 0.06, L * 0.5);
        mm.slab([[-W * 0.06, -L * 0.44], [W * 0.06, -L * 0.44], [W * 0.06, L * 0.4],
          [-W * 0.06, L * 0.4]], H * 0.12, H * 0.03, shade(o.frame || P.greyDark, 0.9));
        mm.restore();
      });
      // The seat, dark, under the glass. Nothing here is transparent, so it is
      // drawn as a headrest breaking the canopy line rather than as furniture.
      m.save().move(0, H * 0.1, L * 0.34);
      m.slab([[-W * 0.24, -L * 0.1], [W * 0.24, -L * 0.1], [W * 0.24, L * 0.06],
        [-W * 0.24, L * 0.06]], H * 0.44, H * 0.08, P.black);
      m.restore();
    }
    m.restore();
  }

  /// A pitot boom or a refuelling probe: a thin tapered tube with a step in it,
  /// smoothed, because at this diameter faceting reads as a bent wire.
  function boom(m, o) {
    m.save().move(o.x || 0, o.y || 0, o.z || 0).rotY(o.yaw || 0).rotX(o.pitch || 0);
    m.soft((mm) => {
      mm.tube(o.r, o.r * 0.86, o.len * 0.55, mm.lod(8, 5), o.col || P.greyDark, false);
      mm.save().move(0, 0, o.len * 0.55);
      mm.tube(o.r * 0.62, o.r * 0.34, o.len * 0.45, mm.lod(8, 5), o.col || P.metal, false);
      mm.restore();
    });
    m.restore();
  }
  /// A blade aerial. Half a dozen of these along a spine is what an operational
  /// aircraft looks like and a model kit's box art does not.
  function bladeAerial(m, o) {
    m.save().move(o.x || 0, o.y, o.z).rotZ(90).rotY(o.roll || 0);
    m.slab([[0, o.len * 0.5], [o.h, o.len * 0.1], [o.h, -o.len * 0.16], [0, -o.len * 0.5]],
      o.t || o.h * 0.14, (o.t || o.h * 0.14) * 0.3, o.col || P.greyDark);
    m.restore();
  }

  const JET_STATIONS = [[0, .72, .74], [.07, 1, 1], [.4, 1, 1], [.62, .96, .92],
    [.82, .66, .68], [.94, .34, .36], [1, .06, .06]];
  /// The same fuselage at the resolution a catalogue card deserves. The seven
  /// stations above are the shape and these do not move them: they fill in
  /// where a real airframe changes section — over the intakes, under the
  /// cockpit, into the boat-tail — so the loft has something to interpolate
  /// besides a straight line. Authored rather than subdivided, because a
  /// fuselage is a set of decisions and not a spline.
  const JET_STATIONS_FINE = [[0, .72, .74], [.03, .84, .85], [.07, 1, 1], [.13, 1.02, 1.0],
    [.22, 1.03, 1.0], [.31, 1.02, 1.0], [.4, 1, 1], [.48, 1, .99], [.55, .99, .96],
    [.62, .96, .92], [.7, .88, .86], [.76, .78, .78], [.82, .66, .68], [.88, .5, .53],
    [.94, .34, .36], [.975, .19, .2], [1, .06, .06]];

  /// The fighter family. Nine of the deck's aircraft are this function with a
  /// different table: what separates an F-15E from an F-22 at the size a card
  /// draws them is planform, tail cant and whether the section is oval or
  /// chined, and all of those are arguments. What it does NOT cover is the
  /// three aircraft whose whole identity is that they are not this shape — the
  /// F-117, the B-2 and the RQ-170 — which are built below on their own.
  ///
  /// EVERY OPTION THE TABLES ALREADY USED STILL MEANS WHAT IT MEANT. What is
  /// new is what the function does with them: aerofoil wings with separate
  /// control surfaces instead of extruded outlines, an intake with a duct you
  /// can see down, a nozzle with petals, a framed canopy, and the panels,
  /// booms and doors an operational aircraft carries and a silhouette does not.
  function jet(m, o) {
    const L = o.len, col = o.col, w = o.w, h = o.h;
    const faceted = !!o.facet;
    const seg = faceted ? m.lod(12, 6) : m.lod(22, 8);
    // The fuselage. A chined low-observable body is a superellipse and a
    // conventional one is an oval; one loft draws both. Only the oval is
    // smoothed: averaging a chine away is how a Raptor stops being a Raptor.
    bodyLoft(m, {
      len: L, w, h, col, seg, sharp: faceted ? 3.2 : null, soft: !faceted,
      stations: o.stations || (m.far ? JET_STATIONS : JET_STATIONS_FINE),
    });

    const wingY = o.wingY == null ? -h * 0.1 : o.wingY;
    const halfSpan = Math.max(0.2, o.span / 2 - w * 0.44);
    const foilT = o.foil == null ? 0.06 : o.foil;
    // Wings, from the fuselage side out, the root chord placed by its leading
    // edge so a delta and a swept panel are the same two numbers. The panel is
    // now a lofted aerofoil, and the flaperon and the aileron are separate
    // surfaces on real hinge gaps — deflected a couple of degrees, because a
    // gap that is only a line vanishes at 90px and a gap with a shadow does not.
    m.both((mm) => {
      mm.liftSurface({
        x0: w * 0.44, y: wingY, z: o.wingZ,
        span: halfSpan, root: o.root, tip: o.tip, sweep: o.sweep,
        thick: foilT, dihedral: o.dihedral || 0, ctrl: o.wingCtrl == null ? 0.2 : o.wingCtrl,
        surfaces: [[0.05, 0.48, 4], [0.54, 0.96, -5]], fence: o.fence,
      }, col);
      // A leading-edge root extension where the aircraft has one. It is the
      // vortex generator that made a 1970s fighter manoeuvre, and it is a
      // straight line on the plan view, so a card can show it.
      if (o.lerx) {
        mm.save().move(w * 0.4, wingY + h * 0.06, o.wingZ);
        mm.slab([[0, o.lerx], [w * 0.3, o.lerx * 0.16], [w * 0.3, -o.root * 0.2],
          [0, -o.root * 0.35]], h * 0.09, h * 0.03, shade(col, 1.05));
        mm.restore();
      }
      // Pylons and their stores, where the table hangs them. A clean fighter
      // and a loaded one are the same aircraft doing different jobs, and the
      // stores are the only way a card can say which.
      (o.pylons || []).forEach((p) => {
        store(mm, w * 0.44 + halfSpan * p[0], wingY - h * 0.06,
          o.wingZ - o.root * 0.35 + (p[2] || 0), o.root * (p[1] || 0.5),
          o.root * (p[1] || 0.5) * 0.055, p[3]);
      });
    });

    // Horizontal stabilisers. Absent on the tailless designs, where the wing
    // trailing edge carries the control surfaces instead. All-moving on
    // everything here since the 1960s, so there is no separate elevator: the
    // whole slab is the control surface, and it sits a degree nose-down.
    if (o.stab) {
      m.both((mm) => {
        mm.foil({
          x0: w * 0.4, y: o.stab.y || -h * 0.05, z: o.stab.z,
          span: o.stab.span, root: o.stab.root, tip: o.stab.tip, sweep: o.stab.sweep,
          thick: foilT * 0.85, dihedral: o.stab.dihedral || 0, incidence: 1.5,
          n: mm.lod(7, 3), bays: mm.lod(4, 1),
        }, shade(col, 0.96));
      });
    }

    // Fins. `cant` is the outward lean that distinguishes a low-observable twin
    // tail from a conventional one at a glance, and it is the only part of the
    // F-22's shape a 90px card can actually show. rotZ(90) stands a planform on
    // its edge and the cant is taken off that angle; `both` gives the port fin
    // the opposite lean for free.
    const fin = o.fin;
    if (fin && fin.kind !== "none") {
      const finSweep = fin.sweep == null ? fin.root * 0.7 : fin.sweep;
      const one = (mm, x) => {
        mm.save().move(x, fin.y == null ? h * 0.3 : fin.y, fin.z).rotZ(90 - (fin.cant || 0));
        mm.liftSurface({
          x0: 0, y: 0, z: 0, span: fin.height, root: fin.root, tip: fin.tip,
          sweep: finSweep, thick: foilT * 0.8, ctrl: 0.26,
          n: mm.lod(7, 3), bays: mm.lod(4, 1), surfaces: [[0.06, 0.92, 2]],
        }, shade(col, 1.02));
        // The tip cap. Every fin on a modern aircraft carries an antenna
        // fairing or a jammer up there, and it squares off the silhouette.
        if (!mm.far) {
          mm.save().move(fin.height * 0.99, 0, -finSweep);
          mm.slab([[0, fin.tip * 0.3], [h * 0.13, fin.tip * 0.18], [h * 0.13, -fin.tip * 0.86],
            [0, -fin.tip * 1.02]], foilT * fin.tip * 1.6, foilT * fin.tip * 0.45,
          shade(col, 0.8));
          mm.restore();
        }
        mm.restore();
      };
      if (fin.kind === "twin") m.both((mm) => one(mm, fin.x == null ? w * 0.32 : fin.x));
      else one(m, 0);
    }

    // Exhaust. Two nozzles or one, with petals, sunk into the tail.
    const nz = o.nozzles == null ? 2 : o.nozzles;
    for (let i = 0; i < nz; i += 1) {
      const x = nz === 1 ? 0 : (i === 0 ? 1 : -1) * w * 0.22;
      nozzle(m, { x, y: -h * 0.02, z: -L * 0.02, r: h * 0.27, len: L * 0.085 });
    }

    // Canopy. Cockpit glass is the one place a saturated colour is allowed: it
    // is what tells you which end is the front. A two-seater gets a longer one
    // and a middle bow, which is the whole visible difference between a fighter
    // and the strike variant built on it.
    if (o.canopy !== false) {
      const cz = o.canopyZ == null ? L * 0.66 : o.canopyZ;
      const two = !!o.twoSeat;
      canopyGlass(m, {
        y: h * 0.4, z: cz, len: L * (two ? 0.25 : 0.19), w: w * 0.56, h: h * 0.36,
        frame: shade(col, 0.7), frames: two ? [[.15, .2], [.48, .53], [.76, .82]] : null,
      });
    }

    // Intakes, and they are diagnostic: a nose inlet is 1960s, cheek inlets are
    // 1970s, a chin inlet is an F-16, a diverterless bump is an F-35 — and the
    // splitter plate standing off the skin is the tell for all of them except
    // the last, whose entire point is that it does not have one.
    if (o.intake === "nose") {
      intakeDuct(m, { x: 0, y: 0, z: L * 0.995, w: h * 0.46, h: h * 0.46,
        len: L * 0.12, sharp: 2, col: shade(col, 0.94) });
      // The shock cone. A supersonic nose inlet without its centrebody is a
      // hoover, and the cone is half of what makes this shape read as 1960s.
      m.save().move(0, 0, L * 0.95);
      m.soft((mm) => mm.tube(h * 0.02, h * 0.17, L * 0.05, mm.lod(12, 6), P.metal, false));
      m.restore();
    } else if (o.intake === "side" || o.intake === "dsi") {
      const iz = (o.intakeZ == null ? L * 0.52 : o.intakeZ) + L * 0.16;
      m.both((mm) => {
        intakeDuct(mm, {
          x: w * 0.52, y: -h * 0.1, z: iz, w: w * 0.36, h: h * 0.36, len: L * 0.17,
          sharp: o.intake === "dsi" ? 2.4 : 3.4, col: shade(col, 0.92),
          splitter: o.intake === "side" ? 1 : 0, yaw: -4,
        });
      });
    } else if (o.intake === "chin") {
      intakeDuct(m, { x: 0, y: -h * 0.34, z: L * 0.72, w: w * 0.5, h: h * 0.26,
        len: L * 0.14, sharp: 3.0, col: shade(col, 0.9) });
    }
    if (m.far) return m;

    // What is left is what an operational airframe carries and a silhouette
    // does not: the avionics-bay panels down each side, the spine hatches, the
    // gun port, the gear doors, a pitot boom and the aerials. None of it is
    // more than a few dozen triangles and all of it is the difference between
    // a shape and an aeroplane.
    panelRun(m, { rx: w * 0.5, ry: h * 0.5, th: 18, z0: L * 0.42, z1: L * 0.78,
      n: 4, w: w * 0.3, len: L * 0.05, col: shade(col, 0.94) });
    panelRun(m, { rx: w * 0.5, ry: h * 0.5, th: 162, z0: L * 0.42, z1: L * 0.78,
      n: 4, w: w * 0.3, len: L * 0.05, col: shade(col, 0.94) });
    panelRun(m, { rx: w * 0.5, ry: h * 0.5, th: 90, z0: L * 0.16, z1: L * 0.4,
      n: 3, w: w * 0.34, len: L * 0.05, col: shade(col, 1.03) });
    panelRun(m, { rx: w * 0.5, ry: h * 0.5, th: 270, z0: L * 0.3, z1: L * 0.6,
      n: 3, w: w * 0.4, len: L * 0.06, col: shade(col, 0.9) });
    // Gear doors: the nose bay under the cockpit, a main bay each side of the
    // wing root. Outlined proud rather than cut in, because this file has no
    // booleans and a chamfered outline throws the same shadow a cut would.
    skinPanel(m, { rx: w * 0.5, ry: h * 0.5, th: 270, z: L * 0.62,
      w: w * 0.3, len: L * 0.1, depth: h * 0.04, col: shade(col, 0.86) });
    m.both((mm) => {
      skinPanel(mm, { rx: w * 0.5, ry: h * 0.5, th: 250, z: o.wingZ - o.root * 0.3,
        w: w * 0.34, len: o.root * 0.5, depth: h * 0.04, col: shade(col, 0.86) });
    });
    if (o.gun !== false) {
      skinPanel(m, { rx: w * 0.5, ry: h * 0.5, th: 34, z: (o.canopyZ || L * 0.66) - L * 0.03,
        w: w * 0.12, len: L * 0.04, depth: h * 0.05, col: P.sensor });
    }
    // Air data. A 1960s interceptor carries a metre and a half of nose boom and
    // that boom is part of its recognisable length; everything since carries a
    // short pitot on the side of the radome, where it costs the deck's quoted
    // length nothing. Getting this wrong makes an F-15E measure 21 metres.
    if (o.intake === "nose") {
      boom(m, { x: w * 0.24, y: 0, z: L * 0.88, r: h * 0.018, len: L * 0.075, pitch: -8 });
    } else {
      m.both((mm) => boom(mm, { x: w * 0.15, y: h * 0.05, z: L * 0.9,
        r: h * 0.012, len: L * 0.05 }));
    }
    bladeAerial(m, { y: h * 0.46, z: L * 0.34, h: h * 0.12, len: L * 0.035 });
    bladeAerial(m, { y: -h * 0.44, z: L * 0.46, h: h * 0.09, len: L * 0.03, roll: 180 });
    // Formation-light strips and the refuelling receptacle: flush panels, and
    // the sort of thing the eye reads as "a real aeroplane" without naming.
    m.both((mm) => {
      skinPanel(mm, { rx: w * 0.5, ry: h * 0.5, th: 62, z: L * 0.5,
        w: w * 0.06, len: L * 0.14, depth: h * 0.02, col: shade(P.white, 0.9) });
    });
    skinPanel(m, { rx: w * 0.5, ry: h * 0.5, th: 90, z: L * 0.56,
      w: w * 0.16, len: L * 0.04, depth: h * 0.03, col: shade(col, 0.8) });
    return m;
  }

  // --------------------------------------------------------- ground vehicles
  /// A hull cross-section: wide at the belly, tapered at the roof, which is the
  /// shape every armoured vehicle in this file has in section.
  function trap(halfW, y0, y1, taper) {
    return [[halfW, y0], [halfW * taper, y1], [-halfW * taper, y1], [-halfW, y0]];
  }

  // ------------------------------------------------------------- the armour kit
  // EVERYTHING FROM HERE TO `roadWheel` BELONGS TO THE ARMOUR CLASS ALONE, and
  // that is a decision rather than an accident. `trap` above is shared with the
  // truck, the IFV and two satellites; `linkedTrack` and `dishedWheel` further
  // down are the mechanised formation's. So the three tanks get their own hull
  // section, their own belt and their own turret shells here, and nothing
  // outside this block changes shape because a tank did.

  /// An armoured hull in section. `trap` gives a hull ONE fold per side, and a
  /// vehicle with one fold is a wedge with tracks on it; a rolled-plate hull
  /// has four — the belly plate, the flare out to the sponson, the vertical
  /// flank and the roof taper — and every one of them is a line the key light
  /// catches from the three-quarter view a card is always drawn at. Eleven
  /// contour points, and the shoulder is placed a long way outboard on purpose:
  /// put it where the plate ANGLES suggest and it lands within a thousandth of
  /// the line between its neighbours, which is a fold that reads as nothing and
  /// a cap fan that ships slivers. Convex, so `loft` can still shut the ends.
  ///
  /// LEFT FACETED, DELIBERATELY. A tank hull is cut plate welded at its folds,
  /// and smoothing those folds would cost these three what averaging a chine
  /// costs a Raptor. The curved parts of an armoured vehicle are its wheels,
  /// its gun and — on one of the three — its turret, and those are smoothed
  /// one at a time below.
  function hullSect(hw, y0, y1, y2, taper) {
    const flare = y0 + (y1 - y0) * 0.34;
    const shoulder = hw * (1 - (1 - taper) * 0.30);
    const my = y1 + (y2 - y1) * 0.42;
    return [
      [hw * 0.72, y0], [hw, flare], [hw, y1], [shoulder, my], [hw * taper, y2],
      [0, y2 + (y2 - y1) * 0.045],
      [-hw * taper, y2], [-shoulder, my], [-hw, y1], [-hw, flare], [-hw * 0.72, y0],
    ];
  }

  /// A cast turret in section: a flat ring floor with a half-ellipse crown over
  /// it. A second-generation turret is a CASTING and a third-generation one is
  /// welded plate, and round against faceted is most of what a 90 px card can
  /// tell about a tank's decade — so the two are different shells below rather
  /// than one shell with a flag on it.
  function castSect(hw, h, n) {
    const pts = [];
    for (let i = 0; i <= n; i += 1) {
      const a = (i / n) * Math.PI;
      pts.push([Math.cos(a) * hw, Math.sin(a) * h]);
    }
    return pts;
  }

  /// A welded turret in section: a vertical lower flank, a sloped upper cheek
  /// and a flat roof. Seven points and not one of them is on a curve, which is
  /// the entire idea.
  function weldSect(hw, y1, y2, crown, taper) {
    return [
      [hw, 0], [hw, y1], [hw * taper, y2], [0, y2 + crown],
      [-hw * taper, y2], [-hw, y1], [-hw, 0],
    ];
  }

  /// The Armour class's running gear, and it is where the triangles go. At card
  /// size the eye finds the wheel line before it finds the gun; these three
  /// models used to carry a painted band and nine bare cylinders, which is a
  /// crate on blocks. `linkedTrack` below is the mechanised formation's belt
  /// and its output may not move, so a tank keeps its own here — a heavier shoe
  /// with a grouser pad on the outside and a guide horn on the inside, a
  /// torsion arm and a bump stop at every station, a TOOTHED sprocket at the
  /// drive end, a SPOKED idler at the other, and the return rollers holding the
  /// top run up off the road wheels.
  ///
  /// Drawing both ends the same is the tell that one wheel was drawn twice
  /// rather than a vehicle designed, so they are not the same.
  function trackRun(m, o) {
    const x = o.x, t = o.thick, R = o.r;
    // The shoe's half-thickness. The belt path is walked along the SHOE
    // CENTRELINE and the grouser pad stands proud of it, so the loop is lifted
    // by the pad rather than by the shoe — otherwise the tank stands on its
    // shoe plates with its grousers buried, which is the one arrangement no
    // tracked vehicle has ever had.
    const g = R * 0.075;
    const path = { z0: o.z0 + R, z1: o.z1 - R, r: R, cy: R + g * 1.95 };
    const wheelR = R * 0.74, wheelW = t * 0.92;
    const wheelY = path.cy - R + g + wheelR;
    const n = o.wheels == null ? 6 : o.wheels;
    const first = path.z0 + R * 0.5, last = path.z1 - R * 0.5;
    const at = (i) => (n === 1 ? (first + last) / 2 : first + (i / (n - 1)) * (last - first));
    const endR = R * 0.86;
    if (m.far) {
      // Nine pixels hold the loop and five wheels in it. The wheel line is the
      // last thing to go: a tracked hull with a blank band under it reads as a
      // crate, which is the one thing a map pin must not do.
      m.bar(x - t / 2, x + t / 2, 0, path.cy + R + g, path.z0 - R, path.z1 + R, P.track);
      [path.z0, path.z1].forEach((z) => {
        dishedWheel(m, { x: x - wheelW / 2, y: path.cy, z, r: endR, w: wheelW,
          col: shade(P.track, 1.35) });
      });
      for (let i = 0; i < 3; i += 1) {
        dishedWheel(m, { x: x - wheelW / 2, y: wheelY, r: wheelR, w: wheelW,
          z: at(Math.round((i * (n - 1)) / 2)) });
      }
      return;
    }
    // The belt. `trackStation` walks the stadium as one parameter, so the links
    // come out evenly spaced round the corners instead of bunching there, and
    // the shade walks with the index so a run of them is a run and not a
    // stripe. Deterministic: it is a function of i and nothing else.
    const links = o.links == null ? 30 : o.links;
    const pitch = (2 * ((path.z1 - path.z0) + Math.PI * R)) / links;
    for (let i = 0; i < links; i += 1) {
      const s = trackStation((i + 0.5) / links, path);
      m.save().move(x, s[1], s[0]).rotX(s[2]);
      m.bar(-t / 2, t / 2, -g, g, -pitch * 0.42, pitch * 0.42,
        shade(P.track, 1 + ((i % 5) - 2) * 0.055));
      // The grouser pad on the outer face and the guide horn on the inner one.
      // The horn rides between the two halves of every road wheel and is what
      // keeps the track on the vehicle; without it a belt is a rubber ribbon.
      m.bar(-t * 0.38, t * 0.38, -g * 1.95, -g * 0.85, -pitch * 0.3, pitch * 0.3,
        shade(P.rubber, 1.2 + (i % 2) * 0.12));
      m.bar(-t * 0.1, t * 0.1, g * 0.8, g * 3.0, -pitch * 0.2, pitch * 0.2,
        shade(P.track, 1.5));
      m.restore();
    }
    for (let i = 0; i < n; i += 1) {
      const z = at(i);
      dishedWheel(m, { x: x - wheelW / 2, y: wheelY, z, r: wheelR, w: wheelW });
      // The torsion arm is the fitting that says the wheel MOVES, and the bump
      // stop above it is what limits the travel. Both are a shadow under the
      // sponson at card size, and a wheel line with neither is a row of discs
      // glued to a flank.
      m.save().move(x - t * 0.66, wheelY + R * 0.52, z - R * 0.52).rotX(45);
      m.bar(-t * 0.13, t * 0.13, -R * 0.09, R * 0.09, 0, R * 0.74, shade(P.metal, 0.5));
      m.restore();
      m.bar(x - t * 0.9, x - t * 0.5, wheelY + R * 0.5, wheelY + R * 0.8,
        z - R * 0.66, z - R * 0.3, shade(P.track, 1.15));
    }
    // Drive sprocket aft, idler forward, as they sit on nearly every tank built
    // since 1945 — the final drive is at the transmission end and the
    // transmission is behind the engine.
    const sprocket = path.z0, idler = path.z1;
    dishedWheel(m, { x: x - wheelW / 2, y: path.cy, z: sprocket, r: endR, w: wheelW,
      col: shade(P.track, 1.4) });
    for (let i = 0; i < 11; i += 1) {
      m.save().move(x, path.cy, sprocket).rotX((i * 360) / 11).move(0, endR * 0.95, 0);
      m.bar(-t * 0.2, t * 0.2, 0, R * 0.16, -pitch * 0.16, pitch * 0.16, shade(P.metal, 0.55));
      m.restore();
    }
    dishedWheel(m, { x: x - wheelW / 2, y: path.cy, z: idler, r: endR, w: wheelW,
      col: shade(P.track, 1.3) });
    for (let i = 0; i < 8; i += 1) {
      m.save().move(x - t * 0.08, path.cy, idler).rotX(i * 45);
      m.bar(-t * 0.055, t * 0.055, endR * 0.26, endR * 0.82, -endR * 0.09, endR * 0.09,
        shade(P.track, 1.6));
      m.restore();
    }
    // Return rollers. Small, smoothed, and the reason the top run is a straight
    // line held up rather than a band sagging onto the road wheels.
    const rollR = R * 0.24, rollY = path.cy + R - g - rollR;
    for (let i = 0; i < 3; i += 1) {
      const z = path.z0 + (path.z1 - path.z0) * (0.22 + i * 0.28);
      m.save().move(x - t * 0.3, rollY, z).rotY(90);
      m.soft((mm) => mm.tube(rollR, rollR, t * 0.55, mm.lod(9, 5), shade(P.rubber, 1.35), false));
      discCap(m, rollR, m.lod(9, 5), shade(P.metal, 0.6), true);
      m.save().move(0, 0, t * 0.55);
      discCap(m, rollR, m.lod(9, 5), shade(P.metal, 0.8), false);
      m.restore();
      m.restore();
    }
  }

  /// The armour family: hull, running gear, turret, gun. Three of the deck's
  /// entries are this call — second-generation armour is a cast round turret on
  /// a short five-wheel hull, third is welded and angular on a long seven-wheel
  /// one, and Trophy is the third with an active protection suite bolted round
  /// its turret. The tables did not move; what the function does with them did.
  function armour(m, o) {
    const col = o.col, L = o.len, W = o.w, H = o.h;
    const belly = o.belly == null ? H * 0.3 : o.belly;
    const roof = belly + H * 0.55;
    const fender = belly + H * 0.3;
    const tx = W / 2 * 0.97, tw = W * 0.17, tr = H * 0.2;
    // The mudguard datum, and it is NOT the sponson line. `fender` above is a
    // fold in the hull plate; the shelf a skirt hangs from sits on top of the
    // TRACK, which `trackRun` closes at 2.22 times its own radius — walk that
    // number rather than guess it, or the guards float half a metre above the
    // belt and the skirt hangs in the air beside it.
    const shelf = tr * 2.22 + H * 0.02;
    // The hull, station by station. `a`, `b` and `c` shift the belly, the
    // sponson line and the roof in units of H, which is how the glacis, the
    // rear plate and the nose wedge come out of one table instead of three
    // lofts stitched together.
    const sec = (z, k, a, b, c, tp) => ({
      z: z * L,
      pts: hullSect(W / 2 * k, belly + a * H, fender + b * H, roof + c * H, tp),
    });
    const hull = [
      sec(0, 0.9, 0.03, -0.02, -0.02, 0.86),
      sec(0.05, 0.98, 0.01, 0, 0, 0.9),
      sec(0.16, 1, 0, 0, 0, 0.92),
      sec(0.42, 1, 0, 0, 0, 0.92),
      sec(0.62, 1, 0, 0, 0, 0.92),
      sec(0.74, 1, 0, 0, -0.01, 0.92),
      sec(0.82, 1, 0.02, -0.02, -0.06, 0.92),
      sec(0.9, 0.99, 0.06, -0.05, -0.15, 0.9),
      sec(0.96, 0.95, 0.11, -0.1, -0.24, 0.88),
      sec(1, 0.88, 0.15, -0.12, -0.29, 0.86),
    ];
    m.loft(m.far ? [hull[0], hull[3], hull[7], hull[9]] : hull, col);

    m.both((mm) => trackRun(mm, {
      x: tx, thick: tw, r: tr, z0: L * 0.02, z1: L * 0.98,
      wheels: o.wheels, links: o.links,
    }));

    if (!m.far) {
      // THE GLACIS, and everything bolted to it. The angle is measured off the
      // two stations that make the plate rather than guessed, so the applique
      // lies ON the armour instead of floating at its own pitch — which is the
      // difference between an added plate and a decal.
      const gz0 = L * 0.82, gz1 = L * 0.995;
      const gy0 = roof - H * 0.06, gy1 = roof - H * 0.29;
      const gAng = Math.atan2(gy0 - gy1, gz1 - gz0) / DEG;
      const gLen = Math.hypot(gz1 - gz0, gy0 - gy1);
      m.save().move(0, (gy0 + gy1) / 2, (gz0 + gz1) / 2).rotX(gAng);
      for (let i = 0; i < 5; i += 1) {
        m.save().move((i - 2) * W * 0.185, H * 0.02, 0);
        m.slab([[-W * 0.084, -gLen * 0.4], [W * 0.084, -gLen * 0.4],
          [W * 0.084, gLen * 0.4], [-W * 0.084, gLen * 0.4]],
        H * 0.05, H * 0.014, shade(col, 1 + ((i % 3) - 1) * 0.05));
        m.restore();
      }
      // Spare track links, which live on the glacis on the real vehicle because
      // that is the face that gets hit, and here also break the largest flat
      // plane on the model.
      for (let i = 0; i < 4; i += 1) {
        m.bar((i - 1.5) * W * 0.14 - W * 0.05, (i - 1.5) * W * 0.14 + W * 0.05,
          H * 0.05, H * 0.085, -gLen * 0.46, -gLen * 0.36, shade(P.track, 1.3));
      }
      m.restore();
      boltRun(m, { n: 5, r: H * 0.012, x0: -W * 0.36, x1: W * 0.36,
        y0: gy1 + H * 0.03, z0: L * 0.975, pitch: -(90 - gAng), col: shade(P.metal, 0.62) });
      // The bolted nose beam and the towing eyes. A hull that ends in a taper
      // ends in nothing; the beam is the hard horizontal that says where the
      // front of the vehicle is, and a recovery shackle pins to the eyes.
      m.bar(-W * 0.42, W * 0.42, gy1 - H * 0.05, gy1 + H * 0.02, L * 0.985, L * 1.02,
        shade(col, 0.84));
      m.both((mm) => {
        mm.bar(W * 0.24, W * 0.33, gy1 - H * 0.04, gy1 + H * 0.01, L * 1.0, L * 1.05,
          shade(P.metal, 0.6));
        // Headlamp in its guard, out on the fender where the driver can see it.
        mm.save().move(W * 0.4, roof - H * 0.2, L * 0.93);
        mm.soft((s) => s.tube(H * 0.062, H * 0.072, H * 0.06, s.lod(9, 5), shade(col, 0.9), false));
        discCap(mm, H * 0.062, mm.lod(9, 5), shade(col, 0.7), true);
        mm.save().move(0, 0, H * 0.06);
        discCap(mm, H * 0.072, mm.lod(9, 5), shade(P.glass, 1.2), false);
        mm.restore();
        mm.restore();
        // Fender shelf over the track, with a mudflap hanging at each end. The
        // flap is rubber and hangs nearly to the road, which is the one part of
        // a tank that is allowed to look soft.
        mm.bar(tx - tw * 0.62, tx + tw * 0.66, shelf - H * 0.015, shelf + H * 0.015,
          L * 0.03, L * 0.97, shade(col, 0.9));
        [L * 0.045, L * 0.955].forEach((z) => {
          mm.bar(tx - tw * 0.6, tx + tw * 0.62, H * 0.05, shelf - H * 0.01,
            z - L * 0.012, z + L * 0.012, shade(P.rubber, 1.25));
        });
        // Skirt panels down the flank, each on its own hinge lug so the row
        // reads as plates that swing up for track work rather than as one
        // painted stripe. Armour plate where the table asks for it, and the
        // shallow dust shield of an earlier decade where it does not.
        // How far down the panel reaches is the era. A modern skirt hangs past
        // the road-wheel centres because it is armour; the earlier decade's is
        // a dust shield over the top of the belt and nothing more.
        const panels = 5, deep = shelf - (o.skirt ? H * 0.32 : H * 0.16);
        for (let i = 0; i < panels; i += 1) {
          const z0 = L * 0.07 + (i * L * 0.84) / panels;
          const z1 = z0 + (L * 0.84) / panels - L * 0.014;
          mm.save().move(tx + tw * 0.56, 0, 0).rotZ(90);
          mm.slab([[deep, z0], [shelf + H * 0.012, z0],
            [shelf + H * 0.012, z1], [deep, z1]],
          H * 0.026, H * 0.008, shade(col, i % 2 ? 1.07 : 0.92));
          mm.restore();
          mm.bar(tx + tw * 0.48, tx + tw * 0.58, shelf - H * 0.01, shelf + H * 0.03,
            (z0 + z1) / 2 - L * 0.018, (z0 + z1) / 2 + L * 0.018, shade(P.metal, 0.6));
        }
      });
      // The engine deck: the bank of grille slats over the powerpack and the
      // exhaust louvre on each flank. Shadow is the only way a renderer with no
      // textures has of saying there is air moving through here.
      for (let i = 0; i < 7; i += 1) {
        m.save().move(0, roof + H * 0.008, L * (0.13 + i * 0.042)).rotX(34);
        m.bar(-W * 0.3, W * 0.3, -H * 0.012, H * 0.012, -H * 0.03, H * 0.03,
          shade(col, 0.74 + (i % 2) * 0.08));
        m.restore();
      }
      m.both((mm) => louvre(mm, { x: W / 2 * 1.005, y0: fender + H * 0.02, y1: fender + H * 0.2,
        z: L * 0.09, w: W * 0.16, t: H * 0.01, d: H * 0.035, yaw: 90,
        col: shade(P.exhaust, 1.05), n: 5 }));
      // The driver sits forward and left, with his hatch and the three
      // periscopes he closes it and drives on.
      m.save().move(-W * 0.17, roof + H * 0.005, L * 0.78);
      m.slab([[-W * 0.1, -L * 0.042], [W * 0.1, -L * 0.042], [W * 0.1, L * 0.042],
        [-W * 0.1, L * 0.042]], H * 0.045, H * 0.014, shade(col, 1.14));
      m.restore();
      for (let i = 0; i < 3; i += 1) {
        m.bar(-W * 0.25 + i * W * 0.08, -W * 0.21 + i * W * 0.08,
          roof + H * 0.005, roof + H * 0.035, L * 0.815, L * 0.838, P.sensor);
      }
      // The rear plate: towing pintle, the two convoy lamps and the crew's
      // stowage box, which every tank in service carries and none of them
      // leave empty.
      m.bar(-W * 0.1, W * 0.1, belly + H * 0.16, belly + H * 0.26, -L * 0.02, L * 0.02,
        shade(P.metal, 0.58));
      m.both((mm) => {
        mm.bar(W * 0.26, W * 0.34, roof - H * 0.14, roof - H * 0.06, -L * 0.01, L * 0.015,
          shade(P.red, 1.05));
      });
      m.bar(-W * 0.3, W * 0.3, roof - H * 0.04, roof + H * 0.08, -L * 0.005, L * 0.075,
        shade(col, 0.88));
      boltRun(m, { n: 5, r: H * 0.012, x0: -W * 0.34, x1: W * 0.34,
        y0: roof - H * 0.02, z0: L * 0.001, pitch: 0, col: shade(P.metal, 0.6) });
    }

    const t = o.turret;
    if (!t) return m;
    const ty = roof, tz = o.turretZ == null ? L * 0.42 : o.turretZ;
    m.save().move(0, ty, tz);
    // The turret ring. Without a collar the shell floats a centimetre off the
    // roof and every three-quarter view shows the gap.
    m.save().move(0, -H * 0.03, 0).rotX(-90);
    m.soft((mm) => mm.tube(t.w * 0.4, t.w * 0.43, H * 0.05, mm.lod(16, 8), shade(col, 0.76), false));
    m.restore();
    if (t.round) {
      // A CASTING. Lofted as a run of half-ellipse sections and SMOOTHED,
      // because a cast turret has no facets in it — the one place in this class
      // where averaging the normals is the honest answer rather than the lazy
      // one. The floor panel that closes each ring is declared hard: it is a
      // full-width lid the model never draws, and left inside the smoothing
      // group it drags the flanks it shares a rim with round onto itself.
      const cn = m.lod(11, 6);
      const cast = [
        [-0.5, 0.3, 0.68], [-0.42, 0.44, 0.92], [-0.2, 0.49, 1], [0.02, 0.5, 1],
        [0.24, 0.47, 0.96], [0.4, 0.38, 0.86], [0.5, 0.26, 0.7],
      ].map((s) => ({ z: t.len * s[0], pts: castSect(t.w * s[1], t.h * s[2], cn) }));
      m.soft((mm) => mm.loft(cast, shade(col, 1.04), false, cn));
      const cf = cast[0], cl = cast[cast.length - 1];
      m.fan(cf.pts.map((p) => [p[0], p[1], cf.z]).slice().reverse(), shade(col, 0.86));
      m.fan(cl.pts.map((p) => [p[0], p[1], cl.z]), shade(col, 0.94));
    } else {
      // WELDED PLATE, AND IT STAYS FACETED. A third-generation turret is a box
      // of spaced composite arrays with a sloped front and a squared bustle,
      // and its identity is the folds; a smoothing group across them would cost
      // it exactly what averaging a chine costs a Raptor. The two stations a
      // hair apart at the bustle join are the step up onto the fighting
      // compartment, and that step is the silhouette from the side.
      m.loft([
        [-0.5, 0.4, 0.6, 0.66, 0.92], [-0.42, 0.46, 0.62, 0.7, 0.92],
        [-0.16, 0.46, 0.64, 0.72, 0.92], [-0.13, 0.5, 0.92, 1, 0.88],
        [0.1, 0.5, 0.92, 1, 0.88], [0.26, 0.47, 0.86, 0.96, 0.86],
        [0.42, 0.36, 0.6, 0.74, 0.84], [0.5, 0.26, 0.44, 0.54, 0.82],
      ].map((s) => ({
        z: t.len * s[0],
        pts: weldSect(t.w * s[1], t.h * s[2], t.h * s[3], t.h * 0.02, s[4]),
      })), shade(col, 1.05));
    }

    // Mantlet, then gun. The barrel runs past the hull nose on a modern tank
    // and that overhang IS the silhouette, so it is lofted rather than tubed:
    // a real one is fatter at the chamber, steps at the trunnions and tapers
    // to the muzzle, and it is smoothed because at this diameter a facet reads
    // as a bend in the pipe.
    const gy = t.round ? t.h * 0.34 : t.h * 0.55;
    const bs = m.lod(14, 7), GL = t.gunLen, GR = t.gunR;
    m.save().move(0, gy, t.len * 0.4);
    m.soft((mm) => mm.loft([
      { z: -t.h * 0.5, pts: ring(t.h * 0.3, t.h * 0.26, bs) },
      { z: -t.h * 0.2, pts: ring(t.h * 0.42, t.h * 0.36, bs) },
      { z: t.h * 0.1, pts: ring(t.h * 0.4, t.h * 0.34, bs) },
      { z: t.h * 0.2, pts: ring(t.h * 0.3, t.h * 0.26, bs) },
    ], shade(col, 0.9), false));
    m.save().move(0, 0, t.h * 0.18);
    m.soft((mm) => mm.loft([
      { z: 0, pts: ring(GR * 1.5, GR * 1.5, bs) },
      { z: GL * 0.06, pts: ring(GR * 1.22, GR * 1.22, bs) },
      { z: GL * 0.3, pts: ring(GR * 1.05, GR * 1.05, bs) },
      { z: GL * 0.62, pts: ring(GR * 0.98, GR * 0.98, bs) },
      { z: GL, pts: ring(GR * 0.92, GR * 0.92, bs) },
    ], P.exhaust, false));
    if (!m.far) {
      if (t.brake) {
        // The fume extractor two thirds down the tube. It is the one lump on a
        // modern tank gun and the fastest way for a card to say which century
        // this vehicle belongs to.
        m.save().move(0, 0, GL * 0.5);
        m.soft((mm) => mm.loft([
          { z: -GL * 0.09, pts: ring(GR * 1.02, GR * 1.02, bs) },
          { z: -GL * 0.06, pts: ring(GR * 1.85, GR * 1.85, bs) },
          { z: GL * 0.06, pts: ring(GR * 1.85, GR * 1.85, bs) },
          { z: GL * 0.09, pts: ring(GR * 1.02, GR * 1.02, bs) },
        ], shade(P.exhaust, 1.12), false));
        m.restore();
      }
      [0.2, 0.74].forEach((f) => jointBand(m, { z: GL * f, r: GR * 1.02, w: GR * 0.9,
        k: 1.14, seg: bs, col: shade(P.exhaust, 0.82) }));
    }
    m.save().move(0, 0, GL);
    if (t.brake) {
      m.soft((mm) => mm.loft([
        { z: -GL * 0.045, pts: ring(GR * 1.5, GR * 1.5, bs) },
        { z: GL * 0.02, pts: ring(GR * 1.58, GR * 1.58, bs) },
        { z: GL * 0.045, pts: ring(GR * 1.34, GR * 1.34, bs) },
      ], shade(P.exhaust, 0.94), false));
    }
    // The bore. A gun that ends in a disc is a rod; the wall running back down
    // the tube to a dark disc is what makes the muzzle a hole.
    m.soft((mm) => mm.loft([
      { z: GL * 0.045, pts: ring(GR * 0.56, GR * 0.56, bs) },
      { z: -GL * 0.09, pts: ring(GR * 0.5, GR * 0.5, bs) },
    ], shade(P.black, 1.6), false));
    m.move(0, 0, -GL * 0.09);
    m.fan(ring(GR * 0.5, GR * 0.5, bs).map((p) => [p[0], p[1], 0]), P.black);
    m.restore();
    m.restore();
    m.restore();

    if (!m.far) {
      // The gunner's sight in its armoured hood, and on the modern turrets the
      // commander's independent viewer standing above it. Two apertures at
      // different heights is what says this vehicle can look and shoot at once.
      m.save().move(t.w * 0.24, t.h * 0.9, t.len * 0.16);
      m.slab([[-t.w * 0.11, -t.len * 0.06], [t.w * 0.11, -t.len * 0.06],
        [t.w * 0.11, t.len * 0.06], [-t.w * 0.11, t.len * 0.06]],
      t.h * 0.24, t.h * 0.05, shade(col, 1.12));
      m.save().move(0, t.h * 0.02, t.len * 0.062);
      m.slab([[-t.w * 0.08, -t.h * 0.07], [t.w * 0.08, -t.h * 0.07],
        [t.w * 0.08, t.h * 0.07], [-t.w * 0.08, t.h * 0.07]],
      t.h * 0.2, t.h * 0.03, shade(P.glass, 0.95));
      m.restore();
      m.restore();
      if (!t.round) {
        m.save().move(-t.w * 0.16, t.h * 1.02, -t.len * 0.02);
        m.bar(-t.w * 0.09, t.w * 0.09, 0, t.h * 0.16, -t.w * 0.09, t.w * 0.09,
          shade(col, 0.9));
        m.move(0, t.h * 0.3, 0);
        m.slab([[-t.w * 0.12, -t.w * 0.11], [t.w * 0.12, -t.w * 0.11],
          [t.w * 0.12, t.w * 0.11], [-t.w * 0.12, t.w * 0.11]],
        t.h * 0.3, t.h * 0.05, shade(col, 1.1));
        m.save().move(0, 0, t.w * 0.11);
        m.slab([[-t.w * 0.07, -t.h * 0.06], [t.w * 0.07, -t.h * 0.06],
          [t.w * 0.07, t.h * 0.06], [-t.w * 0.07, t.h * 0.06]],
        t.h * 0.06, t.h * 0.02, shade(P.glass, 1.0));
        m.restore();
        m.restore();
      }
      // The two roof hatches and the commander's periscope ring. A turret has
      // three men in it and the hatches are the only thing on a card that says
      // so; the ring of blocks round one of them is what makes it a cupola.
      [[t.w * 0.2, -t.len * 0.08], [-t.w * 0.22, -t.len * 0.16]].forEach((h, k) => {
        m.save().move(h[0], t.h * (t.round ? 0.94 : 1.02), h[1]);
        m.slab([[-t.w * 0.15, -t.w * 0.15], [t.w * 0.15, -t.w * 0.15],
          [t.w * 0.15, t.w * 0.15], [-t.w * 0.15, t.w * 0.15]],
        t.h * 0.07, t.h * 0.022, shade(col, k ? 1.16 : 1.1));
        m.restore();
      });
      for (let i = 0; i < 6; i += 1) {
        m.save().move(t.w * 0.2, t.h * (t.round ? 0.97 : 1.05), -t.len * 0.08)
          .rotY(i * 60).move(0, 0, t.w * 0.17);
        m.bar(-t.w * 0.05, t.w * 0.05, 0, t.h * 0.05, -t.w * 0.025, t.w * 0.025, P.sensor);
        m.restore();
      }
      // Smoke dischargers, two banks of four, angled out and up the way they
      // are aimed. Every one is a tube with a hole in it rather than a peg.
      m.both((mm) => {
        for (let i = 0; i < 4; i += 1) {
          mm.save()
            .move(t.w * (0.34 + (i % 2) * 0.1), t.h * (0.5 + Math.floor(i / 2) * 0.16),
              t.len * 0.24)
            .rotY(26).rotX(-16);
          mm.soft((s) => s.tube(t.h * 0.052, t.h * 0.052, t.h * 0.2, s.lod(8, 5),
            shade(P.black, 1.5), false));
          mm.save().move(0, 0, t.h * 0.2);
          discCap(mm, t.h * 0.048, mm.lod(8, 5), shade(P.metal, 0.7), false);
          mm.restore();
          mm.restore();
        }
        // Applique on the turret cheek, on its own bolt line, standing proud of
        // the shell. A flush panel is a colour change.
        for (let i = 0; i < 3; i += 1) {
          mm.save().move(t.w * (t.round ? 0.47 : 0.51), 0, 0).rotZ(90);
          mm.slab([[t.h * (0.18 + i * 0.24), -t.len * 0.3],
            [t.h * (0.38 + i * 0.24), -t.len * 0.3],
            [t.h * (0.38 + i * 0.24), t.len * 0.2],
            [t.h * (0.18 + i * 0.24), t.len * 0.2]],
          t.h * 0.05, t.h * 0.014, shade(col, i % 2 ? 1.1 : 0.94));
          mm.restore();
        }
      });
      // The bustle rack. The frame is what turns a clean casting into a vehicle
      // somebody lives out of, and every tank in service carries one.
      const bz = -t.len * 0.5, bw = t.w * 0.42, bh = t.h * 0.5;
      m.bar(-bw, bw, bh * 0.24, bh * 0.32, bz - t.len * 0.16, bz, shade(P.metal, 0.6));
      m.bar(-bw, bw, bh, bh * 1.08, bz - t.len * 0.16, bz, shade(P.metal, 0.6));
      for (let i = 0; i < 5; i += 1) {
        const bx = -bw + 2 * bw * (i / 4);
        m.bar(bx - t.w * 0.012, bx + t.w * 0.012, bh * 0.24, bh * 1.08,
          bz - t.len * 0.155, bz - t.len * 0.14, shade(P.metal, 0.62));
      }
      // Two whips on the bustle. Kept SHORT on purpose: a real one is two
      // metres of steel and the card renderer fits a bounding box, so an
      // honest antenna costs the tank a third of its own height on every
      // card it appears on. These are the stub the eye needs and no more.
      whip(m, { x: -t.w * 0.4, y: t.h * (t.round ? 0.7 : 0.9), z: -t.len * 0.34,
        r: H * 0.008, len: H * 0.3, lean: 7 });
      whip(m, { x: t.w * 0.4, y: t.h * (t.round ? 0.68 : 0.88), z: -t.len * 0.38,
        r: H * 0.007, len: H * 0.24, lean: -9 });
    }
    if (t.mg) {
      // The commander's machine gun on its pintle: a mount, a receiver, a
      // barrel with a bore and the ammunition box beside it.
      m.save().move(t.w * 0.3, t.h * (t.round ? 0.96 : 1.04), -t.len * 0.06);
      m.bar(-t.w * 0.035, t.w * 0.035, 0, t.h * 0.14, -t.w * 0.035, t.w * 0.035,
        shade(P.metal, 0.5));
      m.save().move(0, t.h * 0.14, 0);
      m.bar(-t.w * 0.045, t.w * 0.045, 0, t.h * 0.11, -t.len * 0.09, t.len * 0.05, P.black);
      if (!m.far) {
        m.bar(t.w * 0.05, t.w * 0.14, t.h * 0.01, t.h * 0.09, -t.len * 0.07, -t.len * 0.01,
          shade(P.olive, 0.9));
        m.save().move(0, t.h * 0.055, t.len * 0.05);
        m.soft((mm) => mm.tube(t.w * 0.017, t.w * 0.014, t.len * 0.16, mm.lod(9, 5),
          shade(P.black, 1.4), false));
        m.save().move(0, 0, t.len * 0.16);
        discCap(m, t.w * 0.013, m.lod(9, 5), P.black, false);
        m.restore();
        m.restore();
      }
      m.restore();
      m.restore();
    }
    if (t.aps && m.far) {
      // THE MAP PIN NEEDS THE SUITE TOO, and this is the one place in the class
      // where the rule below — gate a greeble block whole rather than shrink it
      // — had to be broken, because measurement said so. Gated out, this mesh
      // came back position- and normal-IDENTICAL to a Third-Generation Armour,
      // float for float; the only thing separating a Trophy pin from a bare
      // tank was tan paint. That is the one read a kit called Active Protection
      // may not lose, and a plan view is exactly the view a map gets, so what
      // survives here is the roof clutter: four faces canted out on the turret
      // corners and a launcher block on each side. Boxes, not arrays — the
      // elements, the frame chamfers and the countermeasure tubes are all
      // sub-pixel at map zoom and cost 356 triangles to say nothing. These cost
      // 72 and put the far mesh at 1,153 against a 1,500 ceiling, where letting
      // the detailed suite through would have spent all but 63 of the headroom.
      // Flat-shaded on purpose: every one of them is a plate or a box.
      m.both((mm) => {
        // The same frames the near mesh builds, in the same places, at the same
        // yaw and cant — one box each in place of a chamfered slab and its grid.
        mm.save().move(t.w * 0.54, t.h * 0.64, t.len * 0.1).rotY(58).rotX(90 - 12);
        mm.bar(-t.h * 0.3, t.h * 0.3, -t.h * 0.035, t.h * 0.035, -t.h * 0.24, t.h * 0.24,
          shade(shade(P.sensor, 1.1), 0.86));
        mm.restore();
        mm.save().move(t.w * 0.46, t.h * 0.6, -t.len * 0.34).rotY(122).rotX(90 - 12);
        mm.bar(-t.h * 0.25, t.h * 0.25, -t.h * 0.035, t.h * 0.035, -t.h * 0.21, t.h * 0.21,
          shade(shade(P.sensor, 1.1), 0.86));
        mm.restore();
        // Pedestal and launcher in one box: at this size they are one lump, and
        // the lump standing proud of the roof is the whole of the silhouette.
        mm.save().move(t.w * 0.5, t.h * 0.84, -t.len * 0.1).rotY(64);
        mm.bar(-t.h * 0.2, t.h * 0.2, -t.h * 0.3, t.h * 0.14, -t.h * 0.14, t.h * 0.14,
          shade(col, 0.85));
        mm.restore();
      });
    }
    if (t.aps && !m.far) {
      // The four radar faces and the two launchers, which is the ENTIRE visible
      // difference between a protected tank and an unprotected one — so they
      // are modelled rather than suggested: a framed array with its radiating
      // elements, and a trainable launcher with the countermeasure tubes in it.
      // Trophy throws a pattern of fragments and not a missile, so the tubes
      // are short and fat and canted outboard.
      m.both((mm) => {
        arrayGrid(mm, { x: t.w * 0.54, y: t.h * 0.64, z: t.len * 0.1,
          w: t.h * 0.6, h: t.h * 0.48, t: t.h * 0.07, yaw: 58, tilt: 12, cols: 2, rows: 2,
          col: shade(P.sensor, 1.1) });
        arrayGrid(mm, { x: t.w * 0.46, y: t.h * 0.6, z: -t.len * 0.34,
          w: t.h * 0.5, h: t.h * 0.42, t: t.h * 0.07, yaw: 122, tilt: 12, cols: 2, rows: 2,
          col: shade(P.sensor, 1.1) });
        mm.save().move(t.w * 0.5, t.h * 0.84, -t.len * 0.1).rotY(64);
        mm.bar(-t.h * 0.1, t.h * 0.1, -t.h * 0.3, 0, -t.h * 0.09, t.h * 0.09,
          shade(col, 0.78));
        mm.slab([[-t.h * 0.2, -t.h * 0.14], [t.h * 0.2, -t.h * 0.14],
          [t.h * 0.2, t.h * 0.14], [-t.h * 0.2, t.h * 0.14]],
        t.h * 0.26, t.h * 0.05, shade(col, 0.92));
        for (let i = 0; i < 2; i += 1) {
          mm.save().move(-t.h * 0.09 + i * t.h * 0.18, 0, t.h * 0.12).rotX(-8);
          mm.soft((s) => s.tube(t.h * 0.068, t.h * 0.072, t.h * 0.1, s.lod(9, 5),
            shade(P.metal, 0.6), false));
          mm.save().move(0, 0, t.h * 0.1);
          discCap(mm, t.h * 0.07, mm.lod(9, 5), shade(P.black, 1.4), false);
          mm.restore();
          mm.restore();
        }
        mm.restore();
      });
    }
    m.restore();
    return m;
  }

  /// A road wheel with its tyre, rim and hub as three separate surfaces, and
  /// WHY THAT IS WORTH THE TRIANGLES: a launcher is a lorry underneath, and at
  /// card size the eye finds the wheel line before it finds anything on the
  /// deck. One dark cylinder per axle reads as a cylinder. A tyre with rolled
  /// shoulders and a crowned tread — so exactly one band is square to the key
  /// light, which is where a real tyre's highlight sits — a bright rim flange
  /// standing proud of it, a hub with its stud circle and eight tread lugs
  /// reads as running gear. Smoothed on the tyre and the rim because both are
  /// bodies of revolution; the lugs and the studs stay hard, because they are
  /// what casts the shadow that says the tread is not painted on.
  function roadWheel(m, o) {
    const R = o.r, W = o.w, seg = m.lod(14, 7);
    m.save().move(o.x, o.y, o.z).rotY(90);
    if (m.far) {
      // A map pin gets the silhouette and stops. Everything above exists to be
      // read at 90 px and none of it survives being drawn at nine.
      m.soft((mm) => mm.tube(R, R, W, seg, P.rubber));
      m.restore();
      return;
    }
    const rimR = R * 0.62, hubR = R * 0.3;
    m.soft((mm) => mm.loft([
      { z: 0, pts: ring(rimR, rimR, seg) },
      { z: W * 0.09, pts: ring(R * 0.94, R * 0.94, seg) },
      { z: W * 0.22, pts: ring(R, R, seg) },
      { z: W * 0.78, pts: ring(R, R, seg) },
      { z: W * 0.91, pts: ring(R * 0.94, R * 0.94, seg) },
      { z: W, pts: ring(rimR, rimR, seg) },
    ], P.rubber, false));
    // The rim as a closed spool the tyre is mounted on. Only its two flanges
    // are ever visible, which is exactly how much of a real rim you see.
    m.soft((mm) => mm.loft([
      { z: -W * 0.02, pts: ring(hubR, hubR, seg) },
      { z: W * 0.05, pts: ring(rimR * 1.03, rimR * 1.03, seg) },
      { z: W * 0.95, pts: ring(rimR * 1.03, rimR * 1.03, seg) },
      { z: W * 1.02, pts: ring(hubR, hubR, seg) },
    ], shade(P.metal, 0.66)));
    // Hub cap and stud circle, outboard face only: the inboard one faces the
    // chassis and nothing ever sees it.
    m.save().move(0, 0, W);
    m.soft((mm) => mm.tube(hubR * 0.74, hubR * 0.6, W * 0.14, mm.lod(10, 5), shade(P.metal, 0.82)));
    for (let i = 0; i < 6; i += 1) {
      m.save().rotZ(i * 60).move(hubR * 0.86, 0, 0);
      m.bar(-R * 0.036, R * 0.036, -R * 0.036, R * 0.036, 0, W * 0.06, shade(P.metal, 0.5));
      m.restore();
    }
    m.restore();
    for (let i = 0; i < 8; i += 1) {
      m.save().rotZ(i * 45).move(0, R * 0.99, W * 0.5);
      m.bar(-R * 0.1, R * 0.1, -R * 0.05, R * 0.05, -W * 0.34, W * 0.34, shade(P.rubber, 1.5));
      m.restore();
    }
    m.restore();
  }

  /// Wheels on an axle line, for everything that is a truck underneath: a
  /// missile transporter-erector, a Patriot battery, a counter-UAS mount, a
  /// command post.
  ///
  /// `detail` IS OPT-IN AND DEFAULTS OFF, so every table written against this
  /// call before the running-gear pass emits the triangles it always did.
  function wheels(m, o) {
    const r = o.r, n = o.axles, z0 = o.z0, z1 = o.z1;
    for (let i = 0; i < n; i += 1) {
      const z = n === 1 ? z0 : z0 + (i / (n - 1)) * (z1 - z0);
      m.both((mm) => {
        if (o.detail) {
          roadWheel(mm, { x: o.x - o.w * 0.5, y: r, z, r, w: o.w });
          return;
        }
        mm.save().move(o.x - o.w * 0.5, r, z).rotY(90);
        mm.tube(r, r, o.w, 10, P.rubber);
        mm.save().move(0, 0, o.w * 0.5);
        mm.tube(r * 0.45, r * 0.45, o.w * 0.55, 8, shade(P.metal, 0.7));
        mm.restore();
        mm.restore();
      });
    }
  }

  /// What a lorry has that a box on wheels does not: frame rails under the bed
  /// with cross members between them, an arched mudguard over every wheel, the
  /// cab's window and door frames, mirrors on their arms, a grille, headlamps,
  /// the exhaust stack up the back of the cab and the stowage lockers down the
  /// flanks. Every piece is a handful of triangles; together they are the
  /// difference between a launcher and a crate with a rocket on it.
  function truckKit(m, o, bedY) {
    if (m.far) return;
    const L = o.len, W = o.w, r = o.wheelR, cab = o.cab, col = o.col;
    const front = L - cab * 0.62;
    m.both((mm) => {
      mm.bar(W * 0.22, W * 0.33, r * 0.74, r * 1.1, r * 0.5, front + r * 0.6, shade(P.black, 1.5));
      // Stowage lockers along the flank: a lorry in the field carries its own
      // spares, and the locker line is what breaks a bare bed side.
      for (let i = 0; i < 3; i += 1) {
        const z0 = r * 1.9 + i * (front - r * 3.2) / 3;
        mm.bar(W * 0.43, W * 0.5, bedY - r * 0.72, bedY - r * 0.1, z0, z0 + (front - r * 3.2) / 3 * 0.82,
          shade(col, i % 2 ? 0.92 : 1.02));
      }
    });
    for (let i = 0; i < 5; i += 1) {
      const z = r * 1.1 + (front - r * 1.6) * (i / 4);
      m.bar(-W * 0.3, W * 0.3, r * 0.82, r * 1.02, z - W * 0.045, z + W * 0.045, shade(P.black, 1.3));
    }
    // Mudguards. Four flat plates round the top of each wheel, which is what a
    // pressed-steel guard is; a smooth arc here would cost twice the triangles
    // and read the same at the size this is drawn.
    const n = o.axles;
    for (let i = 0; i < n; i += 1) {
      const z = n === 1 ? r * 1.3 : r * 1.3 + (i / (n - 1)) * (front - r * 1.3);
      m.both((mm) => {
        for (let k = 0; k < 4; k += 1) {
          const a = -54 + k * 36;
          mm.save().move(W / 2 * 0.99 - W * 0.09, r, z).rotX(a).move(0, r * 1.28, 0);
          mm.bar(-W * 0.1, W * 0.1, 0, r * 0.07, -r * 0.42, r * 0.42, shade(col, 0.78));
          mm.restore();
        }
      });
    }
    // The cab. Window frames round the glass, a door outline each side, the
    // grille and lamps on the nose, mirrors, and the stack.
    const cy = bedY, ch = o.cabH;
    m.both((mm) => {
      mm.save().move(W * 0.47, cy + ch * 0.34, L - cab * 0.5);
      mm.slab([[-ch * 0.16, -cab * 0.2], [ch * 0.16, -cab * 0.2], [ch * 0.16, cab * 0.2],
        [-ch * 0.16, cab * 0.2]], W * 0.04, W * 0.012, shade(col, 0.84));
      mm.restore();
      mm.save().move(W * 0.44, cy + ch * 0.52, L - cab * 0.2);
      mm.slab([[-ch * 0.12, -cab * 0.12], [ch * 0.12, -cab * 0.12], [ch * 0.12, cab * 0.12],
        [-ch * 0.12, cab * 0.12]], W * 0.05, W * 0.014, P.glass);
      mm.restore();
      // Mirror: an arm and a head, and nothing says "cab" faster.
      mm.save().move(W * 0.5, cy + ch * 0.62, L - cab * 0.16);
      mm.bar(0, W * 0.11, -ch * 0.02, ch * 0.02, -W * 0.02, W * 0.02, shade(P.black, 1.4));
      mm.save().move(W * 0.1, 0, 0);
      mm.bar(-W * 0.015, W * 0.015, -ch * 0.13, ch * 0.13, -W * 0.03, W * 0.03, shade(P.black, 1.2));
      mm.restore();
      mm.restore();
      mm.save().move(W * 0.28, cy + ch * 0.12, L - W * 0.02);
      mm.bar(-W * 0.07, W * 0.07, -ch * 0.06, ch * 0.06, 0, W * 0.05, shade(P.white, 0.94));
      mm.restore();
    });
    m.bar(-W * 0.24, W * 0.24, cy + ch * 0.16, cy + ch * 0.38, L - W * 0.03, L + W * 0.01,
      shade(P.black, 1.35));
    m.save().move(W * 0.36, cy, L - cab * 0.86).rotX(-90);
    m.soft((mm) => mm.tube(W * 0.045, W * 0.04, ch * 1.05, mm.lod(8, 5), P.exhaust));
    m.restore();
  }

  /// A truck chassis with a cab. `bedY` is the deck height everything else is
  /// bolted to, so a launcher and a radar can be written against the same datum.
  function truck(m, o) {
    const col = o.col, L = o.len, W = o.w, r = o.wheelR;
    const bedY = r * 2.05;
    m.bar(-W / 2 * 0.86, W / 2 * 0.86, r * 1.1, bedY, 0, L, shade(col, 0.86));
    m.loft([
      { z: L - o.cab, pts: trap(W / 2, bedY, bedY + o.cabH, 0.96) },
      { z: L - o.cab * 0.35, pts: trap(W / 2, bedY, bedY + o.cabH, 0.96) },
      { z: L - o.cab * 0.1, pts: trap(W / 2, bedY, bedY + o.cabH * 0.72, 0.94) },
      { z: L, pts: trap(W / 2 * 0.95, bedY, bedY + o.cabH * 0.5, 0.94) },
    ], col);
    m.bar(-W / 2 * 0.86, W / 2 * 0.86, bedY + o.cabH * 0.3, bedY + o.cabH * 0.72,
      L - o.cab * 0.34, L - o.cab * 0.08, P.glass);
    // `detail` is opt-in and absent by default, so the command post and the
    // counter-UAS trailer written against this call before the launcher pass
    // still build the lorry they always built.
    wheels(m, { r, w: W * 0.16, x: W / 2 * 0.99, axles: o.axles, z0: r * 1.3, z1: L - o.cab * 0.62,
      detail: o.detail });
    if (o.detail) truckKit(m, o, bedY);
    return bedY;
  }

  // ------------------------------------------------------------------ ships
  // WHAT A SHIP IS READ FROM AT CARD SIZE, and it is not the silhouette. It is
  // the plating strakes running the length of the side, the knuckle where that
  // side turns over into the deck edge, the bulwark and the guard rails
  // standing above the deck, the superstructure stepping back tier by tier as
  // it rises, and the see-through of a lattice mast against the sky. Every one
  // of those is geometry or vertex colour, which is all this renderer has.
  //
  // THE HULL IS SMOOTH AND THE DECK EDGE IS NOT. A ship's side is plate rolled
  // over frames — a developable surface, and averaging its normals is simply
  // correct, which is why all six of these used to read as faceted prisms. But
  // the waterline, the knuckle under the deck edge and the transom are folds,
  // and they are the three lines a card reads a hull's sheer from. So a hull is
  // TWO lofts sharing their waterline points exactly, each smoothed inside
  // itself and neither smoothed into the other: the boot topping falls out of
  // the geometry instead of out of a texture this file does not have, and every
  // fold stays a fold.

  /// The lines plan, as one table. `[t along the length, half-beam factor,
  /// sheer rise in multiples of `sheer`, forefoot rise in multiples of the
  /// draft]`. One table rather than three because on a real hull they are one
  /// decision — the bow narrows, rises and lifts its forefoot together — and a
  /// transom does none of the three, which is why t=0 is nearly flat.
  const HULL_LINES = [
    [0, 0.76, 0.26, 0.05], [0.025, 0.90, 0.19, 0.01], [0.06, 0.965, 0.13, 0],
    [0.12, 0.995, 0.07, 0], [0.22, 1, 0.03, 0], [0.34, 1, 0, 0],
    [0.46, 1, 0.01, 0], [0.56, 0.995, 0.05, 0], [0.66, 0.97, 0.12, 0.01],
    [0.75, 0.92, 0.22, 0.05], [0.83, 0.82, 0.36, 0.16], [0.89, 0.68, 0.52, 0.34],
    [0.94, 0.50, 0.72, 0.56], [0.975, 0.30, 0.92, 0.78], [1, 0.06, 1.12, 0.95],
  ];
  /// The same hull at map size: the five stations where the plan actually
  /// turns. It is that short because the task group is THREE of these inside
  /// one map-pin budget, and a pin that spends it all on the carrier's frames
  /// has nothing left for the ships in company.
  const HULL_LINES_FAR = [[0, 0.76, 0.26, 0.05], [0.08, 0.97, 0.12, 0],
    [0.45, 1, 0, 0], [0.8, 0.84, 0.32, 0.12], [0.94, 0.50, 0.72, 0.56],
    [1, 0.06, 1.12, 0.95]];
  /// And the same hull for a ship that is one of several in the same frame. The
  /// task group is three hulls and eight aeroplanes inside one catalogue
  /// budget, so its escorts come off this table rather than the full one. The
  /// alternative is to spend the budget on a single fictional super-ship, which
  /// is the one thing a task group must never become.
  const HULL_LINES_LEAN = [[0, 0.76, 0.26, 0.05], [0.06, 0.965, 0.13, 0],
    [0.22, 1, 0.03, 0], [0.46, 1, 0.01, 0], [0.66, 0.97, 0.12, 0.01],
    [0.83, 0.82, 0.36, 0.16], [0.94, 0.50, 0.72, 0.56], [1, 0.06, 1.12, 0.95]];

  /// A frame, in the XY plane. Written as a starboard profile in fractions —
  /// `[half-beam factor, height factor]` from the bottom up — and mirrored, so
  /// the two halves cannot drift apart. Counter-clockwise as plotted, which is
  /// what an outward face wants.
  ///
  /// IT STARTS AND ENDS AT THE BAND'S TOP EDGE, and that is not cosmetic. The
  /// contour runs down the PORT side, across the bottom and up the starboard
  /// side, so the one panel that would close the ring is the lid across the
  /// band's top — the waterline on the underwater band, the deck line on the
  /// topside one. Both of those are shared with the band above, so the lid is
  /// a face inside the ship that is never drawn; and because it is a full-beam
  /// flat panel sitting in the same smoothing group as the near-vertical
  /// sides, area-weighted averaging let it drag their normals as much as
  /// ninety degrees off, which is a black band down the side of every hull in
  /// this class. `bandLoft` leaves that panel out. Its absence is also why the
  /// contour is ordered this way and not the obvious way: skipping the
  /// wrap-around of a bottom-up contour would take out the flat of bottom,
  /// which is a face the card can see.
  function hullRing(hw, y0, y1, prof) {
    const pts = [];
    for (let i = prof.length - 1; i >= 0; i -= 1) {
      pts.push([-hw * prof[i][0], y0 + (y1 - y0) * prof[i][1]]);
    }
    for (let i = 0; i < prof.length; i += 1) {
      pts.push([hw * prof[i][0], y0 + (y1 - y0) * prof[i][1]]);
    }
    return pts;
  }
  /// `loft` for a contour that is NOT closed: every panel but the wrap-around.
  /// See `hullRing` for why a hull band must not have that one.
  function bandLoft(m, secs, col) {
    const n = secs[0].pts.length;
    for (let k = 0; k + 1 < secs.length; k += 1) {
      const a = secs[k], b = secs[k + 1];
      for (let i = 0; i + 1 < n; i += 1) {
        const j = i + 1;
        const lit = 1 + 0.16 * Math.sin((i / n) * Math.PI * 2 + 1.2);
        m.quad([a.pts[i][0], a.pts[i][1], a.z], [a.pts[j][0], a.pts[j][1], a.z],
          [b.pts[j][0], b.pts[j][1], b.z], [b.pts[i][0], b.pts[i][1], b.z],
          shade(col, lit));
      }
    }
  }
  /// The section below the waterline: a flat of bottom, the turn of the bilge
  /// and then a near-vertical side up to the waterline. The bilge gets all the
  /// points because it is the only part of a hull a card can see curving, and
  /// with one normal per triangle it used to be a chamfer.
  function bilgeProfile(n, flat) {
    const p = [[flat, 0]];
    for (let i = 1; i <= n; i += 1) {
      const a = (i / n) * Math.PI * 0.5;
      p.push([flat + (1 - flat) * Math.sin(a), 0.58 * (1 - Math.cos(a))]);
    }
    p.push([1, 0.82]);
    p.push([1, 1]);
    return p;
  }
  /// Above the waterline: a little flare, then the knuckle that tucks the plate
  /// in under the deck edge. The knuckle is the last two points on purpose —
  /// the deck-edge coaming lands on it, and the deck plan is quoted at the same
  /// 0.955 so the two cannot disagree.
  const TOPSIDE_PROFILE = [[1, 0], [1.012, 0.36], [1.02, 0.70], [1.008, 0.90], [0.955, 1]];
  const DECK_K = 0.955;
  /// The topside's half-beam factor at a height fraction, so a strake can be
  /// laid ON the side rather than inside it.
  function topsideK(v) {
    const p = TOPSIDE_PROFILE;
    for (let i = 0; i + 1 < p.length; i += 1) {
      if (v >= p[i][1] && v <= p[i + 1][1]) {
        const k = (v - p[i][1]) / Math.max(1e-6, p[i + 1][1] - p[i][1]);
        return p[i][0] + (p[i + 1][0] - p[i][0]) * k;
      }
    }
    return 1;
  }

  /// The shell: two lofts sharing a waterline, plus the four folds that end
  /// them. The caps sit OUTSIDE both smoothing groups — a transom averaged into
  /// the quarters rounds off the one corner that says where the ship stops.
  function hullShell(m, o, st) {
    const L = o.len, B = o.beam, fb = o.freeboard, d = o.draft, sheer = o.sheer || 0;
    const boot = o.boot || P.hull, col = o.col || P.navy;
    const under = bilgeProfile(m.lod(o.bilge || 5, 2), o.flat == null ? 0.36 : o.flat);
    const wet = st.map((s) => ({ z: s[0] * L,
      pts: hullRing(B / 2 * s[1], -d * (1 - s[3]), 0, under) }));
    const dry = st.map((s) => ({ z: s[0] * L,
      pts: hullRing(B / 2 * s[1], 0, fb + sheer * s[2], TOPSIDE_PROFILE) }));
    m.soft((mm) => bandLoft(mm, wet, boot));
    m.soft((mm) => bandLoft(mm, dry, col));
    // A TRANSOM AND A STEM ARE FANS, AND THE HUB MATTERS. `fan` takes its hub
    // from the first point, and since `hullRing` now starts at the band's top
    // edge that point sits in a run of three collinear contour points up the
    // same near-vertical side — so the first two triangles of every cap
    // enclosed no area at all and drew nothing while costing their place in the
    // budget. Rotating the contour to start at the keel corner puts the hub
    // where the contour actually turns. The frame is convex, so a fan from any
    // corner of it is still the whole face.
    const cap = (sec, c, fwd) => {
      const k = sec.pts.length / 2 - 1;
      const p = sec.pts.slice(k).concat(sec.pts.slice(0, k))
        .map((q) => [q[0], q[1], sec.z]);
      m.fan(fwd ? p : p.slice().reverse(), c);
    };
    cap(wet[0], shade(boot, 0.78), false);
    cap(dry[0], shade(col, 0.80), false);
    cap(wet[wet.length - 1], shade(boot, 0.86), true);
    cap(dry[dry.length - 1], shade(col, 0.88), true);
  }

  /// The deck plan at the knuckle, as `[x, z, sheer rise]`, traced starboard
  /// bow to starboard stern and back up the port side. That order is what makes
  /// `strip`'s outward normal outboard, and it carries the sheer with it so a
  /// bulwark and a guard rail follow the deck instead of cutting through it.
  function deckOutline(o, st, k) {
    const L = o.len, B = o.beam, sheer = o.sheer || 0;
    const pts = [];
    for (let i = st.length - 1; i >= 0; i -= 1) {
      pts.push([B / 2 * st[i][1] * k, st[i][0] * L, sheer * st[i][2]]);
    }
    for (let i = 0; i < st.length; i += 1) {
      pts.push([-B / 2 * st[i][1] * k, st[i][0] * L, sheer * st[i][2]]);
    }
    return pts;
  }

  /// A strip of plate standing off a curve, and it is the one call behind a
  /// bulwark, a rubbing strake, a deck-edge coaming, a bridge window band and a
  /// roof lip. Given a polyline in the `[x, z]` plane — optionally carrying a
  /// third number, the local rise — a height and an outward thickness, it emits
  /// the outer face, the inner face and the capping rail along the top. Corners
  /// are mitred by averaging the two adjacent segment normals, so a run round a
  /// bow does not open a gap at the stem.
  function strip(m, pts, o) {
    const n = pts.length, closed = !!o.closed, t = o.t, col = o.col;
    if (n < 2) return;
    const nrm = [];
    for (let i = 0; i < n; i += 1) {
      const a = pts[i > 0 ? i - 1 : (closed ? n - 1 : 0)];
      const b = pts[i + 1 < n ? i + 1 : (closed ? 0 : n - 1)];
      const dx = b[0] - a[0], dz = b[1] - a[1], d = Math.hypot(dx, dz) || 1;
      nrm.push([-dz / d, dx / d]);
    }
    const yOf = (p) => o.y + (p.length > 2 ? p[2] : 0);
    const lim = closed ? n : n - 1;
    for (let i = 0; i < lim; i += 1) {
      const j = (i + 1) % n;
      const A = pts[i], Bp = pts[j];
      const Ao = [A[0] + nrm[i][0] * t, A[1] + nrm[i][1] * t];
      const Bo = [Bp[0] + nrm[j][0] * t, Bp[1] + nrm[j][1] * t];
      const a0 = yOf(A), a1 = a0 + o.h, b0 = yOf(Bp), b1 = b0 + o.h;
      m.quad([Ao[0], a0, Ao[1]], [Bo[0], b0, Bo[1]], [Bo[0], b1, Bo[1]], [Ao[0], a1, Ao[1]], col);
      m.quad([Bp[0], b0, Bp[1]], [A[0], a0, A[1]], [A[0], a1, A[1]], [Bp[0], b1, Bp[1]],
        shade(col, 0.84));
      m.quad([A[0], a1, A[1]], [Ao[0], a1, Ao[1]], [Bo[0], b1, Bo[1]], [Bp[0], b1, Bp[1]],
        shade(col, 1.14));
    }
  }

  /// A DRUM, AND WHY `tube` WITH CAPS IS NOT ONE. Half the naval kit is a short
  /// wide cylinder — a barbette, a bearing race, a bollard, a capstan, a launch
  /// tube's muzzle, a bolt head — and a smoothed `tube` closes those with caps
  /// emitted INSIDE the smoothing group. The cap is then one flat face sharing
  /// every rim vertex with the wall, and because the averaging is area-weighted,
  /// on anything shorter than it is wide the cap simply wins: the rim's normal
  /// swings round until it lies nearly along the cap, ninety degrees off the
  /// wall it belongs to, and the whole side of the drum shades as though it were
  /// edge-on to the light. The fix costs nothing — the same triangles with the
  /// caps emitted outside the group, which is also what a machined end face IS:
  /// a hard edge against a curved side.
  function drum(m, o) {
    const seg = o.seg, r0 = o.r, r1 = o.r1 == null ? o.r : o.r1, L = o.len;
    const col = o.col;
    m.save().move(o.x || 0, o.y || 0, o.z || 0);
    m.soft((mm) => mm.tube(r0, r1, L, seg, col, false));
    if (o.caps !== false) {
      m.fan(ring(r0, r0, seg).map((q) => [q[0], q[1], 0]).reverse(), shade(col, 0.82));
      m.fan(ring(r1, r1, seg).map((q) => [q[0], q[1], L]), shade(col, 1.06));
    }
    m.restore();
  }

  /// Guard rails: stanchions and the wires between them. Three wires on posts
  /// round a deck edge is the most recognisable thing about a ship's profile
  /// that is not the ship, and it is the only feature that tells the eye how
  /// high the deck stands. Real bars rather than a painted line, because a
  /// painted line needs a texture and there are none.
  function railRun(m, pts, o) {
    const h = o.h, t = o.t, col = o.col || P.metal, step = o.step || 1;
    const wires = o.wires || [0.44, 0.74, 1];
    for (let i = 0; i + step < pts.length; i += step) {
      const a = pts[i], b = pts[i + step];
      const ya = o.y + (a.length > 2 ? a[2] : 0), yb = o.y + (b.length > 2 ? b[2] : 0);
      const dx = b[0] - a[0], dz = b[1] - a[1], len = Math.hypot(dx, dz);
      if (len < t * 6) continue;
      m.save().move(a[0], ya, a[1]).rotY(Math.atan2(dx, dz) / DEG);
      m.bar(-t * 1.4, t * 1.4, 0, h, -t * 1.4, t * 1.4, shade(col, 0.86));
      wires.forEach((f) => {
        m.save().move(0, h * f, 0).rotX(-Math.atan2(yb - ya, len) / DEG);
        m.bar(-t, t, -t, t, 0, len, col);
        m.restore();
      });
      m.restore();
    }
  }

  /// The weather deck, crowned. Decks are cambered so water runs off them, and
  /// the camber is what stops one reading as a lid: two strips a side rather
  /// than one plate, each following the sheer station by station.
  function weatherDeck(m, o, st, k, y, col) {
    const L = o.len, B = o.beam, sheer = o.sheer || 0;
    const cam = o.camber == null ? B * 0.014 : o.camber;
    for (let i = 0; i + 1 < st.length; i += 1) {
      const a = st[i], b = st[i + 1];
      const ax = B / 2 * a[1] * k, bx = B / 2 * b[1] * k;
      const az = a[0] * L, bz = b[0] * L;
      const ay = y + sheer * a[2], by = y + sheer * b[2];
      const c = shade(col, 1 + ((i % 3) - 1) * 0.04);
      m.quad([ax, ay, az], [0, ay + cam, az], [0, by + cam, bz], [bx, by, bz], c);
      m.quad([0, ay + cam, az], [-ax, ay, az], [-bx, by, bz], [0, by + cam, bz], c);
    }
  }

  /// A superstructure block. A real one is a box with its corners knocked off,
  /// a band of bridge windows near the top, doors on coamings and a lip where
  /// the deck above overhangs. The canted corner is not decoration: it is what
  /// makes a 2010s frigate low-observable and its absence is what dates a 1980s
  /// one, so `tumble` — the inward lean of the sides — is an argument.
  function deckHouse(m, o) {
    const hw = o.w / 2, z0 = o.z0, z1 = o.z1, H = o.h, y0 = o.y, y1 = o.y + H;
    const col = o.col || shade(P.navy, 1.07);
    const cham = o.cham == null ? Math.min(hw, H) * 0.26 : o.cham;
    const tum = o.tumble == null ? 0.93 : o.tumble;
    const sect = (f) => {
      const w = hw * f;
      return [[w, y0], [w * tum, y1 - cham], [w * tum * 0.66, y1],
        [-w * tum * 0.66, y1], [-w * tum, y1 - cham], [-w, y0]];
    };
    const d = z1 - z0;
    m.loft([
      { z: z0, pts: sect(o.aft == null ? 0.86 : o.aft) },
      { z: z0 + d * 0.1, pts: sect(1) },
      { z: z0 + d * 0.68, pts: sect(1) },
      { z: z1 - d * 0.08, pts: sect(0.97) },
      { z: z1, pts: sect(o.fwd == null ? 0.78 : o.fwd) },
    ], col, true);
    if (m.far) return;
    const plan = (f) => {
      const w = hw * f * tum, c = Math.min(w, d / 2) * 0.34;
      const a = z0 + d * 0.08, b = z1 - d * 0.05;
      return [[w, b - c], [w, a + c], [w - c, a], [-w + c, a],
        [-w, a + c], [-w, b - c], [-w + c, b], [w - c, b]];
    };
    // The bridge window band and the roof lip, both right round the block. A
    // deckhouse without a window line is a shipping container.
    strip(m, plan(0.995), { y: y1 - cham - H * 0.19, h: H * 0.15, t: hw * 0.02,
      closed: true, col: o.glass || P.glass });
    strip(m, plan(1.0), { y: y1 - H * 0.05, h: H * 0.06, t: hw * 0.045,
      closed: true, col: shade(col, 0.88) });
    // Watertight doors port and starboard on their coamings, and the vertical
    // stiffeners a plated side is welded to. A block on a ship that is one of
    // several in the frame stops here: at that size the window line is the
    // whole of what a superstructure says.
    if (o.plain) return;
    m.both((mm) => {
      mm.save().move(hw * tum * 0.99, y0, z0 + d * 0.3).rotZ(90);
      mm.slab([[H * 0.03, -H * 0.15], [H * 0.44, -H * 0.13], [H * 0.44, H * 0.13],
        [H * 0.03, H * 0.15]], hw * 0.06, hw * 0.016, shade(col, 0.8));
      mm.restore();
      for (let i = 0; i < 3; i += 1) {
        mm.save().move(hw * tum * 0.985, y0, z0 + d * (0.16 + i * 0.24)).rotZ(90);
        mm.slab([[H * 0.06, -H * 0.035], [H * 0.86, -H * 0.035], [H * 0.86, H * 0.035],
          [H * 0.06, H * 0.035]], hw * 0.03, hw * 0.009, shade(col, 1.1));
        mm.restore();
      }
    });
  }

  /// A rotating air-search array: the face, the ribbed back it is braced by and
  /// the pedestal it turns on. Drawn as a bare plate it reads as a road sign;
  /// the ribs, the feed and the pedestal are what make it machinery.
  function radarArray(m, o) {
    const w = o.w, h = o.h, t = o.t == null ? w * 0.05 : o.t;
    m.save().move(o.x || 0, o.y, o.z || 0).rotY(o.yaw == null ? 24 : o.yaw);
    m.save().rotX(-90);
    drum(m, { r: w * 0.09, r1: w * 0.075, len: o.ped, seg: m.lod(10, 5), col: P.white });
    m.restore();
    m.save().move(0, o.ped, 0).rotX(-(o.tilt == null ? 9 : o.tilt)).rotX(-90);
    m.slab([[-w / 2, -h / 2], [w / 2, -h / 2], [w / 2, h / 2], [-w / 2, h / 2]],
      t, t * 0.3, o.col || P.white);
    if (!m.far) {
      for (let i = 0; i < 3; i += 1) {
        m.save().move(-w * 0.3 + w * 0.3 * i, -t * 0.9, 0);
        m.bar(-w * 0.022, w * 0.022, -t * 0.8, 0, -h * 0.46, h * 0.46, shade(P.greyDark, 1.0));
        m.restore();
      }
      m.save().move(0, -t * 1.05, 0);
      m.bar(-w * 0.46, w * 0.46, -t * 0.55, 0, -h * 0.12, h * 0.12, shade(P.greyDark, 0.9));
      m.restore();
      m.save().move(0, t * 0.9, 0);
      m.slab([[-w * 0.4, -h * 0.17], [w * 0.4, -h * 0.17], [w * 0.4, h * 0.17],
        [-w * 0.4, h * 0.17]], t * 0.5, t * 0.16, shade(P.sensor, 1.1));
      m.restore();
    }
    m.restore();
    m.restore();
  }

  /// A mast, and it is a LATTICE, not a pole. A real one is an open frame
  /// because it has to hold a tonne of array up in a fifty-knot wind without
  /// acting as a sail, and the see-through of it is the whole of what reads as
  /// a mast at card size rather than as a stick. On the frame go the platform,
  /// the yard with its navigation lights, the air-search array, the ESM cans
  /// and the whip aerials an operational ship is covered in.
  function mast(m, o) {
    const h = o.h, r = o.r, col = o.col || P.metal;
    m.save().move(0, o.y, o.z);
    m.save().rotX(-90);
    lattice(m, { len: h * 0.86, w: r * 1.5, h: r * 1.5, t: r * 0.26,
      n: m.lod(o.bays || 5, 1), col });
    m.restore();
    m.save().move(0, h * 0.86, 0);
    m.slab([[-r * 2.4, -r * 2.4], [r * 2.4, -r * 2.4], [r * 2.4, r * 2.6], [-r * 2.4, r * 2.6]],
      r * 0.5, r * 0.16, shade(col, 0.9));
    m.save().rotX(-90);
    m.soft((mm) => mm.tube(r * 0.6, r * 0.3, h * 0.3, mm.lod(9, 5), col, false));
    m.restore();
    m.restore();
    m.save().move(0, h * 0.62, 0);
    m.bar(-h * 0.2, h * 0.2, -r * 0.3, r * 0.3, -r * 0.3, r * 0.3, col);
    if (!m.far) {
      m.both((mm) => {
        mm.save().move(h * 0.2, r * 0.4, 0).rotX(-90);
        drum(mm, { r: r * 0.34, r1: r * 0.28, len: r * 1.2, seg: 8, col: P.white });
        mm.restore();
      });
    }
    m.restore();
    if (o.radar) {
      radarArray(m, { y: h * 0.86 + r * 0.25, w: o.arrayW == null ? h * 0.44 : o.arrayW,
        h: o.arrayH == null ? h * 0.15 : o.arrayH, ped: h * 0.16, yaw: 24 });
    }
    if (!m.far) {
      m.both((mm) => {
        mm.save().move(r * 1.9, h * 0.42, r * 0.6).rotX(-84).rotY(9);
        mm.soft((s) => s.tube(r * 0.12, r * 0.05, h * 0.42, s.lod(7, 4), shade(col, 0.7), false));
        mm.restore();
        mm.save().move(r * 2.1, h * 0.9, 0).rotX(-90);
        drum(mm, { r: r * 0.5, r1: r * 0.44, len: h * 0.13, seg: mm.lod(9, 5),
          col: shade(P.white, 0.94) });
        mm.restore();
      });
    }
    m.restore();
  }

  /// A gun mount: the barbette it trains on, the faceted shield, the slot the
  /// barrel elevates in with its blast bag, and a barrel with a real bore. A
  /// tapered tube stuck in a wedge is a gun-shaped object; the slot, the bag
  /// and the hole down the muzzle are what make it a mount.
  function deckGun(m, o) {
    const R = o.r, col = o.col || P.white;
    m.save().move(0, o.y, o.z);
    m.save().rotX(-90);
    drum(m, { r: R * 0.86, r1: R * 0.8, len: R * 0.24, seg: m.lod(14, 6),
      col: shade(col, 0.78) });
    m.restore();
    m.save().move(0, R * 0.24, 0);
    const sect = (f, y1) => {
      const w = R * 0.78 * f;
      return [[w, 0], [w * 0.9, y1 * 0.72], [w * 0.5, y1], [-w * 0.5, y1],
        [-w * 0.9, y1 * 0.72], [-w, 0]];
    };
    m.loft([
      { z: -R * 1.05, pts: sect(0.72, R * 0.72) },
      { z: -R * 0.8, pts: sect(0.95, R * 0.92) },
      { z: R * 0.35, pts: sect(1, R * 0.98) },
      { z: R * 0.95, pts: sect(0.72, R * 0.66) },
      { z: R * 1.15, pts: sect(0.44, R * 0.5) },
    ], col, true);
    // The blast bag round the trunnion slot, and the barrel through it: a
    // smoothed tube stepping down to a muzzle with the bore drilled out.
    m.save().move(0, R * 0.56, R * 0.9);
    m.soft((mm) => mm.loft([
      { z: -R * 0.34, pts: ring(R * 0.33, R * 0.31, mm.lod(12, 6)) },
      { z: R * 0.22, pts: ring(R * 0.26, R * 0.24, mm.lod(12, 6)) },
    ], shade(P.black, 1.5), false));
    m.soft((mm) => mm.loft([
      { z: 0, pts: ring(R * 0.19, R * 0.19, mm.lod(12, 6)) },
      { z: R * 1.7, pts: ring(R * 0.14, R * 0.14, mm.lod(12, 6)) },
      { z: R * 2.35, pts: ring(R * 0.125, R * 0.125, mm.lod(12, 6)) },
    ], P.metal, false));
    if (!m.far) {
      // The muzzle ring, and the bore behind it. A gun with no hole in the end
      // is a pipe, and the hole is three rings and a disc.
      m.save().move(0, 0, R * 2.35);
      m.soft((mm) => mm.loft([
        { z: 0, pts: ring(R * 0.17, R * 0.17, mm.lod(12, 6)) },
        { z: R * 0.22, pts: ring(R * 0.17, R * 0.17, mm.lod(12, 6)) },
        { z: R * 0.27, pts: ring(R * 0.13, R * 0.13, mm.lod(12, 6)) },
      ], shade(P.metal, 0.9), false));
      m.save().move(0, 0, R * 0.27);
      m.soft((mm) => mm.loft([
        { z: 0, pts: ring(R * 0.1, R * 0.1, mm.lod(12, 6)) },
        { z: -R * 0.55, pts: ring(R * 0.075, R * 0.075, mm.lod(12, 6)) },
      ], shade(P.black, 1.3), false));
      m.move(0, 0, -R * 0.55);
      m.fan(ring(R * 0.075, R * 0.075, m.lod(12, 6)).map((p) => [p[0], p[1], 0]), P.black);
      m.restore();
      m.restore();
      m.both((mm) => {
        mm.save().move(R * 0.72, R * 0.1, -R * 0.2).rotZ(90);
        mm.slab([[R * 0.16, -R * 0.32], [R * 0.62, -R * 0.3], [R * 0.62, R * 0.3],
          [R * 0.16, R * 0.32]], R * 0.07, R * 0.022, shade(col, 0.88));
        mm.restore();
      });
    }
    m.restore();
    m.restore();
    m.restore();
  }

  /// A close-in weapon system: the drum, the radome that sits on it and the
  /// six-barrel cluster slung on the front. It is the most recognisable object
  /// on a warship's upper deck and it used to be a cone with a stick in it.
  function ciws(m, o) {
    const R = o.r;
    m.save().move(o.x || 0, o.y, o.z).rotY(o.yaw || 0);
    m.save().rotX(-90);
    drum(m, { r: R, r1: R * 0.94, len: R * 1.1, seg: m.lod(14, 6), col: P.white });
    m.restore();
    m.save().move(0, R * 1.1, 0).scale(1, 1.3, 1);
    m.soft((mm) => mm.ball(R * 0.86, mm.lod(14, 6), mm.lod(7, 4), shade(P.white, 1.06)));
    m.restore();
    m.save().move(0, R * 0.5, R * 0.62).rotX(-24);
    drum(m, { r: R * 0.34, r1: R * 0.3, len: R * 0.9, seg: m.lod(10, 5),
      col: shade(P.metal, 0.9) });
    m.save().move(0, 0, R * 0.9);
    const barrels = m.lod(6, 3);
    for (let i = 0; i < barrels; i += 1) {
      m.save().rotZ(i * (360 / barrels)).move(0, R * 0.16, 0);
      drum(m, { r: R * 0.05, r1: R * 0.045, len: R * 1.2, seg: m.lod(6, 4), col: P.metal });
      m.restore();
    }
    m.restore();
    m.restore();
    m.restore();
  }

  /// A vertical launch system as what it is from above: a raft of cell hatches
  /// let into the deck inside a coaming, each a chamfered lid with its hinge
  /// down one edge. A flat dark rectangle is a hatch drawn as a decal; these
  /// are lids, and the shadow under the chamfer is what says so.
  function vls(m, o) {
    // A map pin shows THAT there is a launcher raft, not how many cells are in
    // it, so the coarse grid is half as fine in both directions.
    const cols = m.lod(o.cols || 4, Math.max(2, Math.round((o.cols || 4) / 2)));
    const rows = m.lod(o.rows || 6, Math.max(2, Math.round((o.rows || 6) / 2)));
    const W = o.w, D = o.z1 - o.z0, col = o.col || shade(P.navy, 0.92);
    m.bar(-W / 2, W / 2, o.y, o.y + o.h * 0.34, o.z0, o.z1, col);
    strip(m, [[W / 2, o.z1], [W / 2, o.z0], [-W / 2, o.z0], [-W / 2, o.z1]],
      { y: o.y + o.h * 0.34, h: o.h * 0.22, t: W * 0.016, closed: true,
        col: shade(col, 1.14) });
    const cw = (W / cols) * 0.68, cd = (D / rows) * 0.68;
    for (let r = 0; r < rows; r += 1) {
      for (let c = 0; c < cols; c += 1) {
        const x = -W / 2 + (W / cols) * (c + 0.5);
        const z = o.z0 + (D / rows) * (r + 0.5);
        m.save().move(x, o.y + o.h * 0.4, z);
        m.slab([[-cw / 2, -cd / 2], [cw / 2, -cd / 2], [cw / 2, cd / 2], [-cw / 2, cd / 2]],
          o.h * 0.2, o.h * 0.07, shade(P.black, 1 + ((r * 2 + c) % 3) * 0.14));
        if (!m.far) {
          m.save().move(0, o.h * 0.1, -cd * 0.52);
          m.bar(-cw * 0.4, cw * 0.4, -o.h * 0.03, o.h * 0.03, -cd * 0.05, cd * 0.05,
            shade(P.metal, 0.7));
          m.restore();
        }
        m.restore();
      }
    }
  }

  /// Shafts, brackets, screws and rudders. Nothing on a warship is photographed
  /// less and nothing says "this floats" faster — a hull with a bare bottom is
  /// a bath toy. The blades are lofted aerofoils with real pitch, because a
  /// propeller drawn as flat paddles is the one part of a ship that is meant to
  /// be moving. Mirrored by `both`, which hands the port screw the opposite
  /// handedness for free — and outward-turning twin screws is what a real one
  /// has.
  function running(m, o) {
    // A map pin is a plan view of a ship on water and the running gear is
    // under it, so the coarse level does not pay for a screw it cannot show.
    if (m.far) return;
    const R = o.r, col = o.col || shade(P.metal, 0.62);
    m.both((mm) => {
      mm.save().move(o.x, o.y, o.z1);
      mm.soft((s) => s.tube(R * 0.3, R * 0.34, o.z0 - o.z1, s.lod(10, 5), col, false));
      if (!mm.far) {
        // The A-bracket: two legs carrying the shaft up to the hull, which is
        // the detail that makes an exposed shaft read as a shaft.
        [-18, 26].forEach((deg) => {
          mm.save().move(0, 0, (o.z0 - o.z1) * 0.34).rotZ(deg);
          mm.slab([[R * 0.3, -R * 0.5], [o.brace, -R * 0.34], [o.brace, R * 0.34],
            [R * 0.3, R * 0.5]], R * 0.3, R * 0.09, shade(col, 1.1));
          mm.restore();
        });
      }
      // The hub, tapering aft: authored in a frame turned half a turn so the
      // cone runs in increasing local z and the winding stays outward.
      mm.save().rotY(180);
      mm.soft((s) => s.loft([
        { z: 0, pts: ring(R * 0.42, R * 0.42, s.lod(12, 6)) },
        { z: R * 0.5, pts: ring(R * 0.34, R * 0.34, s.lod(12, 6)) },
        { z: R * 0.85, pts: ring(R * 0.1, R * 0.1, s.lod(12, 6)) },
      ], shade(col, 1.2), false));
      mm.fan(ring(R * 0.42, R * 0.42, mm.lod(12, 6)).map((q) => [q[0], q[1], 0]).reverse(),
        shade(col, 0.9));
      mm.save().move(0, 0, R * 0.85);
      mm.fan(ring(R * 0.1, R * 0.1, mm.lod(12, 6)).map((q) => [q[0], q[1], 0]),
        shade(col, 1.3));
      mm.restore();
      mm.restore();
      const blades = mm.far ? 4 : (o.blades || 5);
      for (let i = 0; i < blades; i += 1) {
        mm.save().move(0, 0, -R * 0.16).rotZ(i * (360 / blades));
        mm.foil({ x0: R * 0.36, y: 0, z: 0, span: R * 0.66, root: R * 0.72, tip: R * 0.44,
          sweep: R * 0.34, thick: 0.16, incidence: 32, n: mm.lod(5, 3), bays: mm.lod(3, 1) },
        shade(col, 1.26));
        mm.restore();
      }
      mm.restore();
      if (o.rudder) {
        mm.save().move(o.x * 0.9, o.y + R * 0.3, o.z1 - o.rudder * 1.1);
        mm.save().rotX(-90);
        drum(mm, { r: R * 0.18, len: R * 1.1, seg: mm.lod(8, 5), col });
        mm.restore();
        mm.foil({ x0: 0, y: 0, z: 0, span: R * 1.9, root: o.rudder, tip: o.rudder * 0.72,
          sweep: o.rudder * 0.16, thick: 0.15, dihedral: 90,
          n: mm.lod(7, 3), bays: mm.lod(3, 1) }, shade(col, 1.05));
        mm.restore();
      }
    });
  }

  /// The furniture a weather deck actually carries: bollards in pairs, mushroom
  /// vents over the machinery spaces, liferaft canisters in their cradles and
  /// the windlass on the forecastle head. Each is a dozen triangles; together
  /// they are the difference between a deck and a lid.
  function deckFittings(m, o) {
    if (m.far) return;
    const L = o.len, B = o.beam, y = o.y, s = o.scale;
    m.both((mm) => {
      (o.bollards || [0.14, 0.32, 0.62, 0.86]).forEach((t, i) => {
        const x = B * 0.42 * (1 - Math.abs(t - 0.5) * 0.55);
        const rise = (o.sheer || 0) * (t > 0.8 ? 0.4 : 0.03);
        mm.save().move(x, y + rise, L * t);
        for (let k = 0; k < 2; k += 1) {
          mm.save().move(0, 0, (k ? 1 : -1) * s * 0.6).rotX(-90);
          drum(mm, { r: s * 0.2, r1: s * 0.24, len: s * 0.62, seg: mm.lod(8, 5),
            col: P.greyDark });
          mm.restore();
        }
        mm.restore();
        if (i % 2 === 0) {
          mm.save().move(x * 0.72, y + rise, L * t + s * 2.4).rotX(-90);
          mm.soft((q) => q.tube(s * 0.34, s * 0.3, s * 1.1, q.lod(9, 5), shade(P.navy, 1.1), false));
          mm.restore();
          mm.save().move(x * 0.72, y + rise + s * 1.1, L * t + s * 2.4);
          mm.soft((q) => q.ball(s * 0.4, q.lod(9, 5), q.lod(4, 3), shade(P.navy, 1.16)));
          mm.restore();
        }
      });
      (o.rafts || [0.4, 0.52]).forEach((t) => {
        mm.save().move(B * 0.44, y + s * 0.7, L * t).rotY(90).rotZ(14);
        drum(mm, { r: s * 0.5, len: s * 1.5, seg: mm.lod(10, 5), col: shade(P.white, 0.96) });
        mm.restore();
        mm.save().move(B * 0.44, y, L * t);
        mm.bar(-s * 0.9, s * 0.9, 0, s * 0.4, -s * 0.3, s * 0.3, P.greyDark);
        mm.restore();
      });
    });
    m.save().move(0, y + (o.sheer || 0) * 0.8, L * 0.9);
    m.bar(-s * 1.3, s * 1.3, 0, s * 0.7, -s * 1.1, s * 1.1, shade(P.navy, 0.9));
    m.save().move(0, s * 0.7, 0).rotX(-90);
    drum(m, { r: s * 0.5, r1: s * 0.42, len: s * 0.7, seg: m.lod(10, 5), col: P.greyDark });
    m.restore();
    m.restore();
  }

  /// The whole ship. Every argument the tables already used still means what it
  /// meant — `len`, `beam`, `draft`, `freeboard`, `sheer`, `col` and `houses`
  /// as `[z0, z1, width factor, height, base rise]` up the stack, which is how
  /// a warship's profile is actually described. What is new is what comes out
  /// of them: a hull with a turn of bilge, a cambered deck inside a bulwark and
  /// its rails, plating strakes, tiered houses with window bands and doors,
  /// running gear under the counter and the fittings a deck carries.
  function ship(m, o) {
    const L = o.len, B = o.beam, fb = o.freeboard, sheer = o.sheer || 0;
    const st = o.lines || (m.far ? HULL_LINES_FAR : HULL_LINES);
    const lean = !!o.lean;
    hullShell(m, o, st);
    weatherDeck(m, o, st, DECK_K, fb, o.deck || P.navyDeck);
    const edge = deckOutline(o, st, DECK_K);
    const bw = o.bulwark == null ? B * 0.075 : o.bulwark;
    if (!m.far) {
      strip(m, edge, { y: fb - B * 0.007, h: B * 0.02, t: B * 0.013,
        closed: true, col: shade(o.col || P.navy, 0.82) });
    }
    strip(m, edge, { y: fb, h: bw, t: B * 0.009, closed: true,
      col: shade(o.col || P.navy, 1.05) });
    if (!m.far && !lean) {
      // Rubbing strakes: the laps between courses of shell plating, laid ON the
      // side by the topside profile rather than at the deck plan's own width,
      // and the one detail that tells the eye how long a hull is.
      const side = edge.filter((p, i) => i % 2 === 0);
      [0.34, 0.66].forEach((f, i) => {
        const k = topsideK(f) / DECK_K * 1.004;
        strip(m, side.map((p) => [p[0] * k, p[1], p[2] * f]),
          { y: fb * f, h: B * 0.013, t: B * 0.006, closed: true,
            col: shade(o.col || P.navy, i ? 0.9 : 1.1) });
      });
      railRun(m, edge, { y: fb + bw, h: B * 0.07, t: B * 0.005,
        step: o.railStep || 2 });
    }
    (o.houses || []).forEach((h, i) => {
      deckHouse(m, { w: B * h[2], z0: L * h[0], z1: L * h[1], y: fb + (h[4] || 0),
        h: h[3], col: shade(o.col || P.navy, 1.07 - i * 0.035), tumble: o.tumble,
        plain: lean });
    });
    if (o.running) {
      running(m, { x: B * 0.24, y: -o.draft * 0.6, z0: L * 0.15, z1: L * 0.035,
        r: o.running, brace: B * 0.19,
        rudder: o.rudder == null ? o.running * 1.5 : o.rudder });
    }
    if (o.fittings !== false) {
      deckFittings(m, { len: L, beam: B, y: fb, sheer, scale: B * 0.045,
        bollards: o.bollards, rafts: o.rafts });
    }
    return m;
  }

  /// A boat: a rigid inflatable on its cradle. A patrol craft's whole job is
  /// boarding and this is what it does it with, so on that hull the boat is not
  /// furniture — it is the armament.
  function seaBoat(m, o) {
    const L = o.len, W = o.w;
    const sec = (hw, base, top) => [[hw, base], [hw * 1.06, top], [-hw * 1.06, top], [-hw, base]];
    m.save().move(o.x, o.y, o.z).rotY(o.yaw || 0);
    const hull = [
      { z: -L * 0.5, pts: sec(W * 0.3, W * 0.1, W * 0.52) },
      { z: -L * 0.2, pts: sec(W * 0.48, -W * 0.16, W * 0.5) },
      { z: L * 0.2, pts: sec(W * 0.5, -W * 0.16, W * 0.5) },
      { z: L * 0.42, pts: sec(W * 0.36, -W * 0.04, W * 0.5) },
      { z: L * 0.5, pts: sec(W * 0.08, W * 0.16, W * 0.52) },
    ];
    // THE SAME CLOSED-RING TRAP THE HULL BANDS WERE FIXED FOR, AND IT WAS STILL
    // HERE. This section is a chine boat: two near-vertical topsides, a flat of
    // bottom and — because a `loft` ring has to close — a full-beam LID across
    // the gunwales that is never drawn from outside. Lofted as one closed ring
    // inside one smoothing group, that lid is a horizontal face sharing every
    // rim vertex with the two sides, and the area weighting hands the sides its
    // normal: measured at 0.0329 against its own face on nav_patrol and 0.0253
    // on nav_escort, the worst corners in either model. The four panel runs are
    // emitted separately instead, at exactly the same triangle count and in the
    // same winding — the two topsides smoothed ALONG the boat, where a hull is
    // actually curved, and the lid and the bottom left hard, because a chine is
    // the one line that says this is a boat and not a bar of soap.
    const col0 = o.col || shade(P.black, 1.7);
    const run = (a, b) => hull.map((s) => ({ z: s.z, pts: [s.pts[a], s.pts[b]] }));
    m.soft((mm) => bandLoft(mm, run(0, 1), col0));
    m.soft((mm) => bandLoft(mm, run(2, 3), col0));
    bandLoft(m, run(1, 2), shade(col0, 0.94));
    bandLoft(m, run(3, 0), shade(col0, 0.86));
    m.fan(hull[0].pts.map((q) => [q[0], q[1], hull[0].z]).slice().reverse(),
      shade(o.col || shade(P.black, 1.7), 0.8));
    m.fan(hull[4].pts.map((q) => [q[0], q[1], hull[4].z]),
      shade(o.col || shade(P.black, 1.7), 0.9));
    if (!m.far) {
      m.save().move(0, W * 0.5, -L * 0.04);
      m.bar(-W * 0.22, W * 0.22, 0, W * 0.42, -W * 0.24, W * 0.24, shade(P.grey, 0.9));
      m.restore();
      m.save().move(0, W * 0.1, -L * 0.54).rotX(10);
      m.bar(-W * 0.12, W * 0.12, -W * 0.5, W * 0.3, -W * 0.2, W * 0.14, P.black);
      m.restore();
      m.save().move(0, W * 0.5, L * 0.12);
      m.slab([[-W * 0.4, -L * 0.22], [W * 0.4, -L * 0.22], [W * 0.4, L * 0.22],
        [-W * 0.4, L * 0.22]], W * 0.06, W * 0.02, shade(P.grey, 1.1));
      m.restore();
    }
    m.restore();
  }

  // -------------------------------------------------------------- submarines
  /// The submarine family. A modern hull is a body of revolution with a
  /// parallel mid-body, so it is one smoothed loft plus the six things that
  /// break the cylinder: the casing along its back, the sail and the fillet
  /// that fairs it in, the planes, the free-flood holes down the flank, the
  /// towed-array fairing and the screw.
  ///
  /// EVERY CURVE HERE IS SMOOTHED AND THE CASING EDGE IS NOT. A pressure hull
  /// is a spun cylinder and one normal per triangle turned it into a twelve-
  /// sided pencil; the casing that runs along its back is a separate structure
  /// with a hard edge, and that edge is the line a card reads the length from.
  const SUB_STATIONS = [[0, .10, .10], [.015, .30, .30], [.035, .48, .48],
    [.07, .68, .68], [.12, .84, .84], [.18, .94, .94], [.26, .99, .99],
    [.34, 1, 1], [.56, 1, 1], [.70, .995, .995], [.79, .96, .96],
    [.86, .88, .88], [.92, .74, .74], [.96, .56, .56], [.985, .34, .34],
    [1, .09, .09]];
  const SUB_STATIONS_FAR = [[0, .10, .10], [.035, .48, .48], [.12, .84, .84],
    [.34, 1, 1], [.70, .995, .995], [.86, .88, .88], [.96, .56, .56], [1, .09, .09]];

  /// The sail in section: a symmetric streamlined body with rounded shoulders
  /// and a FLAT TOP, written as a closed ring so `loft` can sweep it fore and
  /// aft. A sail with square corners is a 1950s boat and neither of these is
  /// one — but a sail that comes to a point at the crown is worse than either.
  /// The crown carries the bridge cockpit and it is a deck people stand on, so
  /// it is a real panel a fifth of the beam wide; taken to a knife instead, the
  /// smoothing group closes round it, the two sides cancel, and the residue
  /// hands the whole top of the sail a normal lying flat along itself. That
  /// measured 0.0009 against its own face and it drew as a black crease down
  /// the one line of a submarine a card can see.
  function subSailRing(m, hw, y0, y1) {
    const n = m.lod(4, 2), pts = [[hw, y0]];
    for (let i = 1; i <= n; i += 1) {
      const a = (i / n) * Math.PI * 0.5;
      pts.push([hw * Math.cos(a * 0.62), y0 + (y1 - y0) * (0.7 + 0.3 * Math.sin(a))]);
    }
    for (let i = n; i >= 1; i -= 1) {
      const a = (i / n) * Math.PI * 0.5;
      pts.push([-hw * Math.cos(a * 0.62), y0 + (y1 - y0) * (0.7 + 0.3 * Math.sin(a))]);
    }
    pts.push([-hw, y0]);
    return pts;
  }

  function submarine(m, o) {
    const L = o.len, D = o.dia, col = o.col || P.hull;
    const seg = m.lod(o.seg || 26, 10);
    const st = m.far ? SUB_STATIONS_FAR : SUB_STATIONS;
    bodyLoft(m, { len: L, w: D, h: D, col, seg, soft: true, caps: false, stations: st });
    m.fan(ring(D * st[0][1] / 2, D * st[0][1] / 2, seg)
      .map((q) => [q[0], q[1], 0]).reverse(), shade(col, 0.78));
    m.fan(ring(D * st[st.length - 1][1] / 2, D * st[st.length - 1][1] / 2, seg)
      .map((q) => [q[0], q[1], L]), shade(col, 0.92));
    // The casing: the flat walking deck along the top of the pressure hull,
    // with the hard edge that makes a submarine a submarine and not a torpedo.
    const sb = [], pt = [];
    st.forEach((s) => {
      if (s[0] < 0.05 || s[0] > 0.95) return;
      const hw = D * 0.17 * Math.min(1, s[1] * 1.2);
      sb.push([hw, s[0] * L]);
      pt.push([-hw, s[0] * L]);
    });
    strip(m, sb.slice().reverse().concat(pt), { y: D * 0.455, h: D * 0.05,
      t: D * 0.012, closed: true, col: shade(col, 1.22) });
    for (let i = 0; i + 1 < sb.length; i += 1) {
      m.quad([sb[i][0], D * 0.505, sb[i][1]], [-sb[i][0], D * 0.505, sb[i][1]],
        [-sb[i + 1][0], D * 0.505, sb[i + 1][1]], [sb[i + 1][0], D * 0.505, sb[i + 1][1]],
        shade(col, 1.3));
    }
    // The sail, streamlined in section rather than a wedge, with the bridge
    // cockpit cut into its top and the fillet that fairs its foot into the
    // casing — a butt joint there is what makes a sail read as a fin taped on.
    const SH = o.sailH, SL = o.sailLen;
    m.save().move(0, D * 0.46, L * 0.62);
    const sail = [
      { z: -SL * 0.5, pts: subSailRing(m, D * 0.055, 0, SH * 0.96) },
      { z: -SL * 0.3, pts: subSailRing(m, D * 0.115, 0, SH) },
      { z: SL * 0.1, pts: subSailRing(m, D * 0.135, 0, SH) },
      { z: SL * 0.38, pts: subSailRing(m, D * 0.115, 0, SH * 0.95) },
      { z: SL * 0.5, pts: subSailRing(m, D * 0.05, 0, SH * 0.88) },
    ];
    m.soft((mm) => mm.loft(sail, shade(col, 1.2), false));
    m.fan(sail[0].pts.map((q) => [q[0], q[1], sail[0].z]).slice().reverse(),
      shade(col, 1.0));
    m.fan(sail[4].pts.map((q) => [q[0], q[1], sail[4].z]), shade(col, 1.1));
    if (!m.far) {
      m.soft((mm) => mm.loft([
        { z: -SL * 0.6, pts: subSailRing(mm, D * 0.085, D * 0.02, D * 0.07) },
        { z: -SL * 0.32, pts: subSailRing(mm, D * 0.175, D * 0.02, D * 0.13) },
        { z: SL * 0.4, pts: subSailRing(mm, D * 0.175, D * 0.02, D * 0.13) },
        { z: SL * 0.6, pts: subSailRing(mm, D * 0.08, D * 0.02, D * 0.06) },
      ], shade(col, 1.1), false));
      m.save().move(0, SH * 0.98, SL * 0.16);
      m.bar(-D * 0.07, D * 0.07, -SH * 0.12, 0, -SL * 0.13, SL * 0.13, P.black);
      m.restore();
    }
    // Masts: the search and attack periscopes, the ESM head and the snorkel.
    // A sail drawn bare reads as a fin; the mast line is what says this is a
    // boat that is looking at something.
    [[0, 0, 0.5, D * 0.03], [D * 0.05, -SL * 0.15, 0.34, D * 0.024],
      [-D * 0.05, SL * 0.1, 0.26, D * 0.02]].forEach((k) => {
      m.save().move(k[0], SH * 0.94, k[1]).rotX(-90);
      drum(m, { r: k[3], r1: k[3] * 0.8, len: SH * k[2], seg: m.lod(8, 5), col: P.metal });
      m.restore();
    });
    if (o.sailPlanes) {
      // Fairwater planes on the sail: real aerofoils, because they are the one
      // control surface on a boat that a photograph ever shows.
      m.both((mm) => {
        mm.foil({ x0: D * 0.12, y: SH * 0.56, z: -SL * 0.02, span: D * 0.55,
          root: SL * 0.44, tip: SL * 0.3, sweep: SL * 0.06, thick: 0.14,
          n: mm.lod(7, 3), bays: mm.lod(3, 1) }, shade(col, 1.14));
      });
    }
    m.restore();
    if (!o.sailPlanes) {
      // Bow planes on the hull instead, set low and well forward — which is
      // where a boat that has to sit on the bottom puts them.
      m.both((mm) => {
        mm.foil({ x0: D * 0.42, y: -D * 0.06, z: L * 0.79, span: D * 0.5,
          root: L * 0.05, tip: L * 0.034, sweep: L * 0.014, thick: 0.15,
          n: mm.lod(7, 3), bays: mm.lod(3, 1) }, shade(col, 1.14));
      });
    }
    // Stern surfaces: cruciform on the nuclear boat, X-planes on the AIP one,
    // which is the same four aerofoils rolled forty-five degrees.
    for (let i = 0; i < 4; i += 1) {
      m.save().move(0, 0, L * 0.9).rotZ((o.xstern ? 45 : 0) + i * 90);
      m.foil({ x0: D * 0.2, y: 0, z: 0, span: D * 0.5, root: L * 0.075,
        tip: L * 0.038, sweep: L * 0.03, thick: 0.14,
        n: m.lod(7, 3), bays: m.lod(3, 1) }, shade(col, 1.16));
      m.restore();
    }
    // The screw: seven skewed blades on a fairwater cone. An odd blade count
    // and heavy skew are how a quiet screw is built and both are visible.
    m.save().move(0, 0, L * 0.962);
    m.save().rotY(180);
    m.soft((mm) => mm.loft([
      { z: 0, pts: ring(D * 0.02, D * 0.02, mm.lod(14, 7)) },
      { z: -L * 0.018, pts: ring(D * 0.085, D * 0.085, mm.lod(14, 7)) },
      { z: -L * 0.034, pts: ring(D * 0.105, D * 0.105, mm.lod(14, 7)) },
    ], shade(col, 0.9), false));
    m.restore();
    m.fan(ring(D * 0.105, D * 0.105, m.lod(14, 7)).map((q) => [q[0], q[1], -L * 0.034]),
      shade(col, 1.1));
    m.fan(ring(D * 0.02, D * 0.02, m.lod(14, 7)).map((q) => [q[0], q[1], 0]),
      shade(col, 0.86));
    const blades = m.far ? 5 : 7;
    for (let i = 0; i < blades; i += 1) {
      m.save().rotZ(i * (360 / blades)).move(0, 0, -L * 0.004);
      m.foil({ x0: D * 0.09, y: 0, z: 0, span: D * 0.23, root: L * 0.028,
        tip: L * 0.016, sweep: L * 0.02, thick: 0.13, incidence: 30,
        n: m.lod(5, 3), bays: m.lod(3, 1) }, shade(P.metal, 0.72));
      m.restore();
    }
    m.restore();
    if (m.far) return m;
    // The free-flood holes down the casing edge, the towed-array fairing on the
    // quarter, the torpedo shutters on the bow flanks and the runs of access
    // hatches along the back. All flush, all shallow, all shadow — which is the
    // only surface detail a renderer with no textures can have.
    m.both((mm) => {
      for (let i = 0; i < 7; i += 1) {
        mm.save().move(D * 0.16, D * 0.47, L * (0.2 + i * 0.085));
        mm.slab([[-D * 0.03, -L * 0.012], [D * 0.03, -L * 0.012], [D * 0.03, L * 0.012],
          [-D * 0.03, L * 0.012]], D * 0.022, D * 0.007, shade(P.black, 1.2));
        mm.restore();
      }
      mm.save().move(D * 0.45, -D * 0.05, L * 0.28);
      mm.soft((s) => s.loft([
        { z: 0, pts: ring(D * 0.028, D * 0.05, s.lod(8, 5)) },
        { z: L * 0.24, pts: ring(D * 0.045, D * 0.075, s.lod(8, 5)) },
        { z: L * 0.44, pts: ring(D * 0.018, D * 0.035, s.lod(8, 5)) },
      ], shade(col, 1.3), false));
      // Both ends of the fairing are blunt closures against a curved side —
      // folds, and outside the group for the same reason a transom is.
      mm.fan(ring(D * 0.028, D * 0.05, mm.lod(8, 5)).map((q) => [q[0], q[1], 0]).reverse(),
        shade(col, 1.1));
      mm.save().move(0, 0, L * 0.44);
      mm.fan(ring(D * 0.018, D * 0.035, mm.lod(8, 5)).map((q) => [q[0], q[1], 0]),
        shade(col, 1.2));
      mm.restore();
      mm.restore();
      [0.76, 0.83].forEach((t) => {
        skinPanel(mm, { rx: D * 0.46, ry: D * 0.46, th: -14, z: L * t,
          w: D * 0.3, len: D * 0.3, depth: D * 0.016, col: shade(col, 1.28) });
      });
    });
    panelRun(m, { rx: D * 0.48, ry: D * 0.48, th: 90, z0: L * 0.2, z1: L * 0.55, n: 4,
      w: D * 0.16, len: L * 0.022, depth: D * 0.012, col: shade(col, 1.32) });
    panelRun(m, { rx: D * 0.48, ry: D * 0.48, th: 270, z0: L * 0.3, z1: L * 0.6, n: 3,
      w: D * 0.18, len: L * 0.022, depth: D * 0.012, col: shade(col, 1.18) });
    // The circumferential seams. A pressure hull is built in cans welded end to
    // end and the boat is painted over the welds, but the frame line still
    // shows as a shallow band — and it is the only thing on a hundred metres of
    // unbroken cylinder that gives the eye a scale. The forwardmost one is the
    // sonar dome joint, which is where the pressure hull actually stops.
    [0.24, 0.46, 0.66, 0.775].forEach((t, i) => {
      jointBand(m, { z: L * t, r: D * 0.5, w: L * 0.012, k: i === 3 ? 1.018 : 1.008,
        seg, col: shade(col, i === 3 ? 1.2 : 1.06) });
    });
    // Cleats and the capstan on the casing: a boat comes alongside and has to
    // be made fast to something, and a casing drawn bare is a casing nobody has
    // ever moored.
    m.both((mm) => {
      [0.3, 0.42, 0.7, 0.78].forEach((t) => {
        mm.save().move(D * 0.1, D * 0.5, L * t).rotX(-90);
        drum(mm, { r: D * 0.018, r1: D * 0.022, len: D * 0.05, seg: mm.lod(8, 4),
          col: shade(P.metal, 0.7) });
        mm.restore();
      });
      // The escape trunk hatch: a raised coaming with its own ring of dogs,
      // and there is one over each compartment because that is the point.
      [0.36, 0.74].forEach((t) => {
        mm.save().move(D * 0.06, D * 0.505, L * t);
        mm.slab([[-D * 0.05, -D * 0.05], [D * 0.05, -D * 0.05], [D * 0.05, D * 0.05],
          [-D * 0.05, D * 0.05]], D * 0.024, D * 0.008, shade(col, 1.36));
        mm.restore();
      });
    });
    if (o.tubes) {
      // Vertical launch tubes forward of the pressure hull's forward bulkhead:
      // a double row of muzzle hatches let into the casing, each on its bolt
      // ring. It is the one feature that tells an improved boat from an early
      // one at a glance, so it is worth its four hundred triangles.
      const rows = Math.ceil(o.tubes / 2);
      for (let i = 0; i < rows; i += 1) {
        const t = o.tubeZ[0] + (o.tubeZ[1] - o.tubeZ[0]) * (i / Math.max(1, rows - 1));
        m.both((mm) => {
          mm.save().move(D * 0.085, D * 0.5, L * t).rotX(-90);
          drum(mm, { r: D * 0.042, len: D * 0.014, seg: mm.lod(10, 5),
            col: shade(col, 1.34) });
          mm.restore();
        });
      }
    }
    if (o.flankArray) {
      // The flank sonar: a long shallow panel down each side, which on a
      // conventional boat is most of the sensor fit and all of its silhouette
      // below the casing.
      m.both((mm) => {
        mm.save().move(D * 0.465, -D * 0.08, L * 0.44).rotZ(90);
        mm.slab([[-D * 0.13, -L * 0.12], [D * 0.13, -L * 0.12], [D * 0.13, L * 0.12],
          [-D * 0.13, L * 0.12]], D * 0.045, D * 0.014, shade(col, 1.24));
        mm.restore();
        // And the diesel exhaust, which is the other thing only this boat has.
        mm.save().move(D * 0.14, D * 0.5, L * 0.6).rotX(-70);
        drum(mm, { r: D * 0.04, r1: D * 0.036, len: D * 0.16, seg: mm.lod(9, 5),
          col: shade(P.black, 1.4) });
        mm.restore();
      });
    }
    return m;
  }

  // --------------------------------------------------------------- ordnance
  /// A missile, a bomb, an interceptor or a glide body: one tube, one nose and
  /// as many fin sets as it has. Eleven entries in the deck come through here.
  function missile(m, o) {
    const L = o.len, R = o.r, col = o.col || P.missile;
    const nose = o.nose == null ? L * 0.16 : o.nose;
    bodyLoft(m, {
      len: L, w: R * 2, h: R * 2, col, seg: o.seg || 10, soft: o.soft,
      stations: o.stations || [[0, o.tail ? 0.7 : 1, o.tail ? 0.7 : 1], [0.02, 1, 1],
        [1 - nose / L, 1, 1], [1 - (nose / L) * 0.42, 0.72, 0.72],
        [1, o.blunt ? 0.34 : 0.06, o.blunt ? 0.34 : 0.06]],
    });
    (o.fins || []).forEach((f) => {
      const count = f.n || 4;
      for (let i = 0; i < count; i += 1) {
        m.save().move(0, 0, f.z * L).rotZ((f.roll || 0) + i * (360 / count));
        m.plate([[R * 0.9, 0], [R + f.span, -(f.sweep || 0)],
          [R + f.span, -(f.sweep || 0) - (f.tip == null ? f.chord * 0.6 : f.tip)],
          [R * 0.9, -f.chord]], f.thick || R * 0.16, shade(col, 0.92));
        m.restore();
      }
    });
    if (o.wings) {
      m.both((mm) => {
        mm.save().move(R * 0.9, 0, o.wings.z * L);
        mm.plate([[0, 0], [o.wings.span, -(o.wings.sweep || 0)],
          [o.wings.span, -(o.wings.sweep || 0) - o.wings.tip], [0, -o.wings.chord]],
        R * 0.18, shade(col, 0.96));
        mm.restore();
      });
    }
    if (o.seeker) {
      m.save().move(0, 0, L * 0.985);
      m.ball(R * 0.6, 8, 4, P.glass);
      m.restore();
    }
    if (o.motor !== false) {
      m.save().move(0, 0, -L * 0.02);
      m.tube(R * (o.tail ? 0.62 : 0.82), R * (o.tail ? 0.5 : 0.66), L * 0.03, 8, P.exhaust);
      m.restore();
    }
    return m;
  }

  // ------------------------------------------------------------- ordnance kit
  // WHAT MAKES A ROUND A ROUND, in a renderer with no textures. A missile is a
  // tube, so it is the one shape in this deck where a smooth loft is nearly
  // right and completely characterless: every real one is built in cans that
  // bolt together, wired down a raceway on the outside, hung from lugs, fitted
  // with fins that fillet into the skin, and ended by a nozzle with a throat.
  // Each piece below is a few dozen triangles and every one of them is a line
  // of shadow the eye reads as engineering. NONE of them are wired into
  // `missile` above: that call is shared with the Air and Infantry classes and
  // its output must not move, so these are composed beside it instead.

  /// The raised hoop where two sections of a round bolt together. Smoothed,
  /// because it is a band of a cylinder and faceting it puts a crease across
  /// the one detail that is meant to be a step.
  function jointBand(m, o) {
    const seg = o.seg, r = o.r, w = o.w, k = o.k == null ? 1.045 : o.k;
    m.save().move(0, 0, o.z);
    m.soft((mm) => mm.loft([
      { z: -w * 0.5, pts: ring(r, r, seg) },
      { z: -w * 0.28, pts: ring(r * k, r * k, seg) },
      { z: w * 0.28, pts: ring(r * k, r * k, seg) },
      { z: w * 0.5, pts: ring(r, r, seg) },
    ], o.col, false));
    m.restore();
  }

  /// The cable raceway: the conduit that carries the wiring from the tail
  /// section up to the guidance section on the OUTSIDE of the skin, because
  /// the inside is full of propellant. It is on every real rocket, it is the
  /// detail that tells the eye which way up the round is clocked, and it is
  /// what stops a body of revolution reading as a pencil.
  function raceway(m, o) {
    const seg = m.lod(8, 5), h = o.h, taper = (o.z1 - o.z0) * 0.06;
    m.save().rotZ(o.roll || 0).move(0, o.r * 0.94, o.z0);
    m.soft((mm) => mm.loft([
      { z: 0, pts: ring(h * 0.2, h * 0.2, seg) },
      { z: taper, pts: ring(h * 1.7, h, seg) },
      { z: o.z1 - o.z0 - taper, pts: ring(h * 1.7, h, seg) },
      { z: o.z1 - o.z0, pts: ring(h * 0.2, h * 0.2, seg) },
    ], o.col, false));
    m.restore();
  }

  /// A fin set with root fillets. A control fin bolted flat to a cylinder reads
  /// as a card pushed into a tube; what a real one has is a fairing where the
  /// root meets the skin, and that fairing is what makes the fin and the body
  /// look like one object rather than two. The blade is a `slab` rather than a
  /// `plate` so its rim is chamfered and catches light — see `slab`'s note.
  function finSet(m, o) {
    const n = o.n || 4, R = o.r, col = o.col, t = o.thick == null ? R * 0.16 : o.thick;
    const sweep = o.sweep || 0, tip = o.tip == null ? o.chord * 0.55 : o.tip;
    const seg = m.lod(8, 4);
    for (let i = 0; i < n; i += 1) {
      m.save().move(0, 0, o.z).rotZ((o.roll || 0) + i * (360 / n));
      m.slab([[R * 0.9, 0], [R + o.span, -sweep], [R + o.span, -sweep - tip],
        [R * 0.9, -o.chord]], t, t * 0.32, col);
      if (!m.far) {
        m.save().move(R * 0.84, 0, -o.chord * 0.5);
        m.soft((mm) => mm.loft([
          { z: -o.chord * 0.62, pts: ring(1e-3, 1e-3, seg) },
          { z: -o.chord * 0.2, pts: ring(t * 0.8, t * 2.4, seg) },
          { z: o.chord * 0.3, pts: ring(t * 0.8, t * 2.4, seg) },
          { z: o.chord * 0.6, pts: ring(1e-3, 1e-3, seg) },
        ], shade(col, 1.06), false));
        m.restore();
      }
      m.restore();
    }
  }

  /// A solid-motor bell: the aft closure, the convergent waist, the exit cone
  /// and — the part that matters — the wall running back UP the inside to a
  /// dark throat disc, so the hole reads as a hole. `nozzle` above is an
  /// afterburning turbofan's ring of petals; a rocket has neither petals nor a
  /// turbine, and drawing one with the other's tail is the sort of mistake a
  /// card shows. Built in a frame rolled half a turn, so the whole bell is
  /// authored running aft in increasing local z and the winding stays outward.
  function motorBell(m, o) {
    const seg = m.lod(16, 7), R = o.r, L = o.len;
    m.save().move(0, 0, o.z).rotX(180);
    m.soft((mm) => mm.loft([
      { z: 0, pts: ring(R, R, seg) },
      { z: L * 0.2, pts: ring(R * 0.7, R * 0.7, seg) },
      { z: L * 0.44, pts: ring(R * 0.46, R * 0.46, seg) },
      { z: L, pts: ring(R * 0.86, R * 0.86, seg) },
    ], shade(P.exhaust, 1.06), false));
    m.save().move(0, 0, L);
    m.soft((mm) => mm.loft([
      { z: 0, pts: ring(R * 0.8, R * 0.8, seg) },
      { z: -L * 0.52, pts: ring(R * 0.38, R * 0.38, seg) },
      { z: -L * 0.66, pts: ring(R * 0.22, R * 0.22, seg) },
    ], shade(P.exhaust, 0.44), false));
    m.move(0, 0, -L * 0.66);
    m.fan(ring(R * 0.22, R * 0.22, seg).map((p) => [p[0], p[1], 0]), P.black);
    m.restore();
    m.restore();
  }

  /// A seeker head: the window, the lip it is recessed behind and the ring the
  /// gimbal turns in. Three parts because a guided round's nose is three
  /// parts — and because a single bright ball on the end of a tube reads as a
  /// bead, which is what every seeker in this file used to be.
  function seekerHead(m, o) {
    const seg = m.lod(14, 6), R = o.r;
    m.save().move(0, 0, o.z);
    m.soft((mm) => mm.loft([
      { z: -R * 0.5, pts: ring(R * 1.16, R * 1.16, seg) },
      { z: -R * 0.1, pts: ring(R * 1.2, R * 1.2, seg) },
      { z: 0, pts: ring(R * 1.08, R * 1.08, seg) },
    ], shade(o.col || P.greyDark, 0.9), false));
    m.soft((mm) => {
      const rings = mm.lod(5, 2);
      const secs = [];
      for (let i = 0; i <= rings; i += 1) {
        const a = (i / rings) * Math.PI * 0.5;
        // The dome's base is a hair WIDER than the lip's inner ring, so the two
        // overlap rather than leave an annulus you can see the body through.
        const rr = Math.max(1e-3, Math.cos(a) * R * 1.09);
        secs.push({ z: Math.sin(a) * R * (o.dome == null ? 0.9 : o.dome), pts: ring(rr, rr, seg) });
      }
      mm.loft(secs, o.glass || P.glass, false);
    });
    m.restore();
  }

  /// A suspension lug. Two of these at fourteen inches apart is how every
  /// air-carried store in the world hangs off a rack, and their absence is why
  /// a bomb drawn without them looks like a bomb-shaped object.
  function hoistLug(m, o) {
    m.save().move(0, o.r * 0.92, o.z);
    m.bar(-o.r * 0.09, o.r * 0.09, 0, o.r * 0.3, -o.r * 0.16, o.r * 0.16, o.col);
    m.restore();
  }

  /// A launch canister: the tube, its reinforcing hoops, the frangible cover
  /// across the mouth and the base plate the round sits on. A canister drawn as
  /// a plain pipe IS a plain pipe — the hoops and the lid are the whole of what
  /// makes four of them read as ordnance rather than as scaffolding.
  function canister(m, o) {
    const seg = m.lod(16, 7), R = o.r, L = o.len, col = o.col;
    m.soft((mm) => mm.tube(R, R, L, seg, col, false));
    m.save().move(0, 0, L);
    m.soft((mm) => mm.loft([
      { z: 0, pts: ring(R, R, seg) },
      { z: R * 0.16, pts: ring(R * 1.05, R * 1.05, seg) },
      { z: R * 0.3, pts: ring(R * 0.95, R * 0.95, seg) },
    ], shade(col, 1.14), false));
    // The cover, dished, and a shade the tube is not: a sealed round shows a
    // lid and a fired one shows a hole, and this deck prices sealed rounds.
    m.move(0, 0, R * 0.3);
    m.fan(ring(R * 0.95, R * 0.95, seg).map((p) => [p[0], p[1], 0]), shade(P.black, 1.6));
    m.restore();
    m.save().move(0, 0, -R * 0.22);
    m.soft((mm) => mm.tube(R * 1.06, R * 1.06, R * 0.22, seg, shade(col, 0.8)));
    m.restore();
    if (m.far) return;
    const hoops = o.hoops == null ? 5 : o.hoops;
    for (let i = 0; i < hoops; i += 1) {
      jointBand(m, { z: L * (0.1 + 0.8 * (i / Math.max(1, hoops - 1))), r: R,
        w: R * 0.34, k: 1.07, seg, col: shade(col, 0.84) });
    }
  }

  /// A levelling jack: the leg, its ram and the foot pad. An emplaced launcher
  /// stands on its jacks with the wheels unloaded, and a battery drawn without
  /// them is a lorry that happens to be parked.
  function jack(m, o) {
    m.save().move(o.x, o.y, o.z);
    m.bar(-o.r, o.r, -o.len * 0.35, 0, -o.r, o.r, shade(P.metal, 0.55));
    m.save().rotX(90);
    m.soft((mm) => mm.tube(o.r * 0.6, o.r * 0.6, o.len, mm.lod(9, 5), shade(P.metal, 0.78)));
    m.restore();
    m.save().move(0, -o.len, 0);
    m.bar(-o.r * 1.9, o.r * 1.9, -o.r * 0.5, o.r * 0.5, -o.r * 1.9, o.r * 1.9, shade(P.metal, 0.5));
    m.restore();
    m.restore();
  }

  /// A lattice bay: four longerons and the diagonals between them. This is how
  /// an erector arm, a launcher frame and a radar mast are actually built, and
  /// the see-through of it is the only thing that distinguishes structure from
  /// a solid beam at card size.
  function lattice(m, o) {
    const n = m.lod(o.n || 5, 2), L = o.len, w = o.w, h = o.h, t = o.t, col = o.col;
    m.save().move(o.x || 0, o.y || 0, o.z || 0);
    [[w, h], [w, -h], [-w, h], [-w, -h]].forEach((c) => {
      m.bar(c[0] - t, c[0] + t, c[1] - t, c[1] + t, 0, L, col);
    });
    if (!m.far) {
      for (let i = 0; i < n; i += 1) {
        const z = L * ((i + 0.5) / n);
        m.bar(-w - t, w + t, h - t, h + t, z - t, z + t, shade(col, 0.9));
        m.bar(-w - t, w + t, -h - t, -h + t, z - t, z + t, shade(col, 0.9));
        m.save().move(0, 0, z).rotX(i % 2 ? 34 : -34);
        m.bar(w - t, w + t, -h * 1.3, h * 1.3, -t, t, shade(col, 1.08));
        m.bar(-w - t, -w + t, -h * 1.3, h * 1.3, -t, t, shade(col, 1.08));
        m.restore();
      }
    }
    m.restore();
  }

  // ------------------------------------------------------------------ space
  /// A dish as a paraboloid opening along +Z, with its feed on a boom. Used on
  /// satellites, warships, command vehicles and the counter-UAS mount.
  function dish(m, o) {
    const R = o.r, depth = o.depth == null ? R * 0.36 : o.depth;
    const rings = 5, secs = [];
    for (let i = 0; i <= rings; i += 1) {
      const t = i / rings, r = Math.max(1e-3, R * t);
      secs.push({ z: -depth * t * t, pts: ring(r, r, 12) });
    }
    m.loft(secs, o.col || P.white, false);
    m.save().move(0, 0, R * 0.5);
    m.tube(R * 0.05, R * 0.05, -R * 0.5, 6, P.metal);
    m.ball(R * 0.1, 6, 3, P.sensor);
    m.restore();
  }
  // ------------------------------------------------------------- the space kit
  // WHAT MAKES A SPACECRAFT A SPACECRAFT, in a renderer with no textures and no
  // sky to put it against. This class shipped a gold box, four blue slabs and a
  // dish between them — 542 triangles a model, every one of them a flat facet,
  // which is a DIAGRAM of a satellite rather than a satellite. Everything below
  // is geometry or vertex colour because that is all there is, and every piece
  // of it is something the real article carries for a reason this file can
  // name: a blanket quilted over a frame, an array built out of cells with a
  // millimetre of shadow between them, a reflector with a rim and ribs across
  // its back, and a bore you can see down to the mirror at the bottom of it.
  //
  // THE PIVOTS ARE REAL AND THEY ARE ARGUMENTS. A solar wing turns on its
  // drive, a high-gain antenna trains in azimuth and elevates on a yoke, a boom
  // unfolds about a hinge — so each is built INSIDE its own rotation, and the
  // joint it turns in is drawn as the canister, bearing ring or hinge it
  // actually is. A pose is then one number, and an articulated satellite later
  // is a caller change rather than a rewrite of the geometry.
  //
  // SHADING IS SPLIT HARDER HERE THAN ANYWHERE ELSE IN THE DECK, and down a
  // line the hardware itself draws: barrels, reflectors, baffles, bells and
  // drive cans are TURNED parts and are smoothed; the bus, the blanket panels,
  // the arrays, the ribs, the yokes and every hinge fitting are folded plate
  // and stay faceted. A spacecraft is exactly that mixture — machined cylinders
  // bolted to a sheet-metal box — and smoothing the box is what would turn the
  // one angular shape in the class into a can.

  /// Multi-layer insulation, quilted over the flats of an n-sided body. WHY IT
  /// IS THE FIRST THING BUILT: a spacecraft is mostly blanket, the blanket is
  /// gold, and what a photograph of one actually shows is the QUILTING — the
  /// tape line between two panels and the way each bags a little proud of the
  /// frame under it. A gold box with no seams is a gold box. Each panel is laid
  /// on its FACET's outward normal rather than on the vertex radius, which is
  /// the difference between a blanket lying on a bus and one hovering off its
  /// corners; `skinPanel` above does the same sum for an ellipse.
  function mliQuilt(m, o) {
    if (m.far) return;
    const n = o.n, rows = o.rows == null ? 3 : o.rows, R = o.r;
    const t = o.t == null ? R * 0.05 : o.t;
    const ap = R * Math.cos(Math.PI / n), w = 2 * R * Math.sin(Math.PI / n);
    const span = (o.z1 - o.z0) / rows;
    for (let i = 0; i < n; i += 1) {
      const a = ((i + 0.5) / n) * Math.PI * 2;
      m.save().move(Math.cos(a) * ap, Math.sin(a) * ap, 0).rotZ(a / DEG - 90);
      for (let k = 0; k < rows; k += 1) {
        m.save().move(0, t * 0.4, o.z0 + span * (k + 0.5));
        // A hair of shade between neighbours, as a function of the two indices
        // and nothing else, so a bus comes out the same way in every process
        // that builds it and a wall of panels is a wall of panels, not a grid.
        m.slab([[-w * 0.42, -span * 0.42], [w * 0.42, -span * 0.42],
          [w * 0.42, span * 0.42], [-w * 0.42, span * 0.42]], t, t * 0.34,
        shade(o.col, 1 + (((i * 2 + k) % 5) - 2) * 0.035));
        m.restore();
      }
      m.restore();
    }
  }

  /// One bay of a solar wing: the substrate, the two spars it is built round,
  /// and the cells laid on the sun side. THE CELLS ARE GEOMETRY AND NOT PAINT,
  /// because paint is the one thing this renderer does not have — the
  /// interconnect gap between two cells is a millimetre of shadow, and a field
  /// of them is the entire reason an array reads as an array instead of as a
  /// blue slab. Cells on one face only, which is also true of the article: the
  /// back of a panel is substrate and harness and it is a different colour.
  function arrayBay(m, o) {
    const L = o.len, W = o.w, t = o.t;
    m.save().move(o.x, 0, 0);
    m.slab([[0, -W / 2], [L, -W / 2], [L, W / 2], [0, W / 2]], t, t * 0.34, o.col);
    if (m.far) { m.restore(); return; }
    [-W / 2 + t, W / 2 - t].forEach((z) => {
      m.bar(t * 0.5, L - t * 0.5, -t * 0.95, t * 0.95, z - t, z + t, shade(o.col, 0.8));
    });
    const cols = o.cols, rows = o.rows;
    const cw = (L * 0.9) / cols, ch = (W * 0.88) / rows;
    for (let c = 0; c < cols; c += 1) {
      for (let r = 0; r < rows; r += 1) {
        m.save().move(L * 0.05 + cw * (c + 0.5), t * 0.5, -W * 0.44 + ch * (r + 0.5));
        m.plate([[-cw * 0.43, -ch * 0.41], [cw * 0.43, -ch * 0.41],
          [cw * 0.43, ch * 0.41], [-cw * 0.43, ch * 0.41]], t * 0.5,
        shade(o.cell, 1 + (((c * 2 + r) % 4) - 1.5) * 0.06));
        m.restore();
      }
    }
    m.restore();
  }

  /// A deployable solar wing, running outboard in +X from the bus face.
  ///
  /// THE DRIVE IS THE PIVOT AND IT IS DRAWN. An array turns to keep its cells
  /// square to the sun and it turns about ONE axis — the wing's own — so the
  /// whole wing is built inside that rotation and the joint is the canister it
  /// really is rather than a weld. Everything outboard of it follows for free.
  function solarWing(m, o) {
    const seg = m.lod(12, 5), r = o.r;
    m.save().move(o.x, o.y || 0, o.z || 0).rotX(o.rot || 0);
    // The drive canister. Smoothed, and closed OUTSIDE the smoothing group by
    // `discCap` — a cap lofted inside one shares every rim vertex with the wall
    // and rolls the very step the can was drawn to show.
    m.save().rotY(90);
    m.soft((mm) => mm.tube(r, r * 0.94, o.drive, seg, shade(P.metal, 0.82), false));
    discCap(m, r, seg, shade(P.metal, 0.56), true);
    m.save().move(0, 0, o.drive);
    discCap(m, r * 0.94, seg, shade(P.metal, 1.0), false);
    m.restore();
    // `plain` is the caller saying this wing is drawn small enough that the
    // bearing races, the hinge fittings and the tension bar would each be under
    // a pixel — see `satellite`'s note on `trim`. It is not a level of detail:
    // the same wing at the same LOD is either worth them or it is not.
    if (!m.far && !o.plain) {
      jointBand(m, { z: o.drive * 0.22, r: r * 0.985, w: o.drive * 0.28, k: 1.26,
        seg, col: shade(P.metal, 0.58) });
      jointBand(m, { z: o.drive * 0.78, r: r * 0.955, w: o.drive * 0.28, k: 1.26,
        seg, col: shade(P.metal, 0.58) });
    }
    m.restore();
    // The yoke. A blanket that starts at the bus wall has nothing to fold
    // against and nowhere for the drive to sit, so a real wing stands off on
    // two booms and the gap between bus and first panel is diagnostic.
    [-1, 1].forEach((s) => {
      m.bar(o.drive * 0.85, o.inner + o.t * 3, -r * 0.34, r * 0.34,
        s * o.w * 0.3 - r * 0.34, s * o.w * 0.3 + r * 0.34, shade(P.metal, 0.7));
    });
    const bayLen = (o.span - o.inner) / o.bays;
    for (let i = 0; i < o.bays; i += 1) {
      const x0 = o.inner + i * bayLen;
      arrayBay(m, { x: x0 + bayLen * 0.035, len: bayLen * 0.93, w: o.w, t: o.t,
        col: o.frame, cell: i % 2 ? shade(o.cell, 1.14) : o.cell,
        cols: o.cols, rows: o.rows });
      if (m.far || o.plain || i === 0) continue;
      // The hinge line between two bays: two lugs and the pin they fold about,
      // which is how an array of this size gets into a launch shroud at all.
      [-1, 1].forEach((s) => {
        m.save().move(x0, 0, s * o.w * 0.36);
        m.bar(-bayLen * 0.018, bayLen * 0.018, -o.t * 1.5, o.t * 1.5,
          -o.t * 1.7, o.t * 1.7, shade(P.metal, 0.64));
        m.restore();
      });
      m.save().move(x0, 0, -o.w * 0.42);
      const pin = m.lod(7, 4);
      m.soft((mm) => mm.tube(o.t * 0.6, o.t * 0.6, o.w * 0.84, pin,
        shade(P.metal, 0.92), false));
      discCap(m, o.t * 0.6, pin, shade(P.metal, 0.7), true);
      m.save().move(0, 0, o.w * 0.84);
      discCap(m, o.t * 0.6, pin, shade(P.metal, 0.7), false);
      m.restore();
      m.restore();
    }
    // The tension bar across the tip: what the deployment cable pulls on, and
    // the reason the outboard end of an array is not a raw plate edge.
    if (!m.far && !o.plain) {
      m.bar(o.span - o.t * 1.4, o.span + o.t * 2.6, -o.t * 1.7, o.t * 1.7,
        -o.w * 0.45, o.w * 0.45, shade(P.metal, 0.74));
    }
    m.restore();
  }

  /// The bus: eight equipment panels bolted into a frame, wrapped in blanket,
  /// standing on the ring it flew up on. EIGHT SIDES RATHER THAN A CUBE because
  /// that is what a real one is — the panels ARE the structure, and eight of
  /// them let the radiators face away from the sun while the instruments face
  /// the ground. Faceted on purpose: see the kit note above.
  function spaceBus(m, o) {
    const n = 8, R = o.r, L = o.len, col = o.col, seg = m.lod(16, 8);
    m.save().move(o.x || 0, o.y || 0, o.z || 0);
    m.loft([
      { z: -L / 2, pts: ring(R * 0.9, R * 0.9, n) },
      { z: -L * 0.43, pts: ring(R, R, n) },
      { z: L * 0.43, pts: ring(R, R, n) },
      { z: L / 2, pts: ring(R * 0.93, R * 0.93, n) },
    ], col, true);
    if (o.quilt !== false) {
      mliQuilt(m, { n, r: R, z0: -L * 0.41, z1: L * 0.41,
        rows: o.rows == null ? 3 : o.rows, t: R * 0.05, col: shade(col, 1.05) });
    }
    if (m.far) { m.restore(); return; }
    // The longerons at the panel joints: the frame the blanket is laid over,
    // showing at every corner, which is how the eye knows there is one.
    for (let i = 0; i < n; i += 1) {
      const a = (i / n) * Math.PI * 2;
      m.save().move(Math.cos(a) * R, Math.sin(a) * R, 0).rotZ(a / DEG);
      m.bar(-R * 0.035, R * 0.05, -R * 0.05, R * 0.05, -L * 0.46, L * 0.46,
        shade(col, 0.76));
      m.restore();
    }
    // The separation ring, authored in INCREASING z and then hung off the aft
    // face — a band lofted in decreasing z winds inward, which is the same trap
    // `motorBell` sidesteps with a half turn. It is the one fitting on a bus
    // that says which end pointed at the launcher.
    m.save().move(0, 0, -L / 2 - R * 0.3);
    m.soft((mm) => mm.loft([
      { z: 0, pts: ring(R * 0.58, R * 0.58, seg) },
      { z: R * 0.14, pts: ring(R * 0.64, R * 0.64, seg) },
      { z: R * 0.3, pts: ring(R * 0.6, R * 0.6, seg) },
    ], shade(P.metal, 0.68), false));
    discCap(m, R * 0.58, seg, shade(P.metal, 0.46), true);
    m.restore();
    m.restore();
  }

  /// A steerable high-gain antenna, on its own TWO-AXIS pivot. `dish` above is
  /// shared with two Infantry kits and may not move, so the spacecraft's own is
  /// composed here — the same argument `arrayGrid` makes against `arrayFace`.
  /// What a real one has that a bare paraboloid does not: a stiffening rim that
  /// catches the key light, ribs and a hoop across the BACK (which is the half
  /// a card sees most of the time), a subreflector on its tripod, a feed
  /// looking up into it, and the azimuth bearing and elevation yoke it trains
  /// on. `az` and `el` are those two joints and they default to zero.
  function spaceDish(m, o) {
    const seg = o.seg || m.lod(20, 8), rings = o.rings || m.lod(6, 2);
    const R = o.r, depth = o.depth == null ? R * 0.34 : o.depth;
    const col = o.col || shade(P.white, 0.96), th = R * 0.05;
    m.save().move(o.x || 0, o.y || 0, o.z || 0).rotY(o.az || 0);
    // The pedestal, in the azimuth frame: it stands still while the dish
    // elevates, which is the whole reason the two rotations are nested.
    m.save().move(0, -R * 0.66, 0).rotX(-90);
    m.soft((mm) => mm.tube(R * 0.17, R * 0.13, R * 0.66, m.lod(12, 6),
      shade(P.metal, 0.7), false));
    discCap(m, R * 0.17, m.lod(12, 6), shade(P.metal, 0.5), true);
    if (!m.far) {
      jointBand(m, { z: R * 0.6, r: R * 0.135, w: R * 0.1, k: 1.4, seg: m.lod(12, 6),
        col: shade(P.metal, 0.55) });
    }
    m.restore();
    m.save().rotX(-(o.el || 0));
    // The elevation yoke: two arms from the trunnions back to the reflector's
    // hub. Without them the dish floats above its mast.
    m.both((mm) => {
      mm.bar(R * 0.24, R * 0.38, -R * 0.14, R * 0.03, -depth * 0.9, -depth * 0.1,
        shade(P.metal, 0.66));
    });
    const front = [], back = [];
    for (let i = 0; i <= rings; i += 1) {
      const t = i / rings, r = Math.max(1e-3, R * t);
      front.push({ z: depth * t * t, pts: ring(r, r, seg) });
      back.push({ z: depth * t * t - th, pts: ring(r, r, seg) });
    }
    // The face, lofted RIM TO VERTEX so it winds concave, and the back lofted
    // the other way so it winds convex. Both smoothed: a paraboloid is the one
    // genuinely curved surface on a spacecraft and faceting it makes a bowl
    // into a lampshade.
    m.soft((mm) => mm.loft(front.slice().reverse(), shade(col, 1.1), false));
    m.soft((mm) => mm.loft(back, shade(col, 0.78), false));
    // The rim, hard and outside both groups. It is the edge the key light
    // catches and a smoothed one loses it.
    m.loft([
      { z: depth - th, pts: ring(R, R, seg) },
      { z: depth + R * 0.02, pts: ring(R * 1.035, R * 1.035, seg) },
      { z: depth, pts: ring(R * 0.985, R * 0.985, seg) },
    ], shade(col, 0.9), false);
    // The hub the ribs run to, and the feed and rotary joint live in.
    m.save().move(0, 0, -th - R * 0.2);
    m.soft((mm) => mm.tube(R * 0.17, R * 0.17, R * 0.24, m.lod(12, 6),
      shade(P.metal, 0.72), false));
    discCap(m, R * 0.17, m.lod(12, 6), shade(P.metal, 0.5), true);
    m.restore();
    const ribs = m.far ? 0 : (o.ribs == null ? 6 : o.ribs);
    for (let i = 0; i < ribs; i += 1) {
      // A rib stands in the plane containing the axis and its own radius, so
      // the frame is just rotZ: local x runs out along the radius, local y is
      // tangential, and `slab` extrudes in y. Straight rather than following
      // the parabola, because a shallow dish's ribs are, and because a curved
      // ribbon is not convex and `slab` is convex only.
      //
      // THE OFFSET IS NOT A GUESS. A chord across a convex curve lies in FRONT
      // of it in the middle, so a rib pinned to the back surface at both ends
      // cuts through the reflector between them — measured at 1.5 cm of blade
      // showing through the dish face, which a card at three-quarter view sees
      // as a bright spoke on the wrong side. The sag of a chord across
      // z = depth·t² from t0 to t1 is depth·(t1-t0)²/4, so the whole rib is set
      // back by more than that and never surfaces.
      const t0 = 0.15, t1 = 0.95;
      const off = th + depth * (t1 - t0) * (t1 - t0) / 4 + R * 0.01;
      m.save().rotZ(i * (360 / ribs));
      m.slab([[R * t0, depth * t0 * t0 - off], [R * t1, depth * t1 * t1 - off],
        [R * t1, depth * t1 * t1 - off - R * 0.1],
        [R * t0, depth * t0 * t0 - off - R * 0.16]],
      R * 0.03, R * 0.008, shade(col, 0.66));
      m.restore();
    }
    // The hoop the ribs are tied into, set back to sit on their blades rather
    // than on the shell — same sag arithmetic as the ribs, one radius in.
    if (!m.far) {
      jointBand(m, { z: depth * 0.3136 - th - depth * 0.16 - R * 0.05,
        r: R * 0.56, w: R * 0.07, k: 1.12, seg, col: shade(col, 0.72) });
    }
    if (o.feed === false || m.far) { m.restore(); m.restore(); return; }
    // The Cassegrain pair: a subreflector at the focus and the feed at the
    // vertex looking up into it. The three struts across the aperture are what
    // say the geometry is folded rather than prime-focus, and they are the
    // detail a card reads first on a spacecraft antenna.
    const f = R * 0.62;
    for (let i = 0; i < 3; i += 1) {
      const dx = -R * 0.88, dz = f - depth, len = Math.hypot(dx, dz);
      m.save().rotZ(i * 120 + 30).move(R * 0.88, 0, depth)
        .rotY(Math.atan2(dx, dz) / DEG);
      m.soft((mm) => mm.tube(R * 0.022, R * 0.016, len, mm.lod(6, 4),
        shade(P.metal, 0.8), false));
      m.restore();
    }
    m.save().move(0, 0, f - R * 0.09);
    m.soft((mm) => mm.loft([
      { z: 0, pts: ring(1e-3, 1e-3, seg) },
      { z: R * 0.04, pts: ring(R * 0.11, R * 0.11, seg) },
      { z: R * 0.09, pts: ring(R * 0.17, R * 0.17, seg) },
    ], shade(col, 1.16), false));
    m.save().move(0, 0, R * 0.09);
    discCap(m, R * 0.17, seg, shade(P.metal, 0.66), false);
    m.restore();
    m.restore();
    m.soft((mm) => mm.loft([
      { z: -R * 0.04, pts: ring(R * 0.07, R * 0.07, seg) },
      { z: R * 0.11, pts: ring(R * 0.09, R * 0.09, seg) },
      { z: R * 0.2, pts: ring(R * 0.14, R * 0.14, seg) },
    ], shade(P.metal, 0.88), false));
    m.save().move(0, 0, R * 0.2);
    m.soft((mm) => mm.loft([
      { z: 0, pts: ring(R * 0.13, R * 0.13, seg) },
      { z: -R * 0.15, pts: ring(R * 0.07, R * 0.07, seg) },
    ], shade(P.black, 1.5), false));
    m.move(0, 0, -R * 0.15);
    discCap(m, R * 0.07, seg, P.black, false);
    m.restore();
    m.restore();
    m.restore();
  }

  /// A thruster cluster: the mounting block, two bells and the dark throat down
  /// each of them. Attitude control is the one system on a spacecraft that has
  /// an exhaust, and four of these round a bus are what say the thing
  /// manoeuvres rather than merely falls.
  function thrusterPod(m, o) {
    if (m.far) return;
    const seg = m.lod(9, 5), r = o.r;
    m.save().move(o.x, o.y, o.z).rotY(o.yaw || 0).rotX(o.pitch || 0);
    m.bar(-r * 1.7, r * 1.7, -r * 1.1, r * 1.1, -r * 1.1, 0, shade(P.metal, 0.55));
    [-r * 0.85, r * 0.85].forEach((dx) => {
      m.save().move(dx, 0, 0);
      m.soft((mm) => mm.loft([
        { z: 0, pts: ring(r * 0.34, r * 0.34, seg) },
        { z: r * 0.7, pts: ring(r * 0.26, r * 0.26, seg) },
        { z: r * 1.5, pts: ring(r * 0.62, r * 0.62, seg) },
      ], shade(P.exhaust, 1.12), false));
      m.save().move(0, 0, r * 1.5);
      m.soft((mm) => mm.loft([
        { z: 0, pts: ring(r * 0.56, r * 0.56, seg) },
        { z: -r * 0.5, pts: ring(r * 0.22, r * 0.22, seg) },
      ], shade(P.exhaust, 0.44), false));
      m.move(0, 0, -r * 0.5);
      discCap(m, r * 0.22, seg, P.black, false);
      m.restore();
      m.restore();
    });
    m.restore();
  }

  /// A star tracker in its sun baffle, on the wedge that aims it away from
  /// everything bright. Three of them is how a spacecraft knows which way it is
  /// pointing, and the baffle — a tube far longer than the optic behind it —
  /// is what makes each read as an instrument rather than as a bolt head.
  function starTracker(m, o) {
    if (m.far) return;
    const seg = m.lod(10, 5), r = o.r;
    m.save().move(o.x, o.y, o.z).rotY(o.yaw || 0).rotX(o.pitch || 0);
    m.bar(-r * 1.2, r * 1.2, -r * 1.2, r * 1.2, -r * 1.1, 0, shade(P.metal, 0.6));
    m.soft((mm) => mm.loft([
      { z: 0, pts: ring(r, r, seg) },
      { z: r * 2.2, pts: ring(r * 0.9, r * 0.9, seg) },
      { z: r * 2.6, pts: ring(r * 1.06, r * 1.06, seg) },
    ], shade(P.white, 0.84), false));
    m.save().move(0, 0, r * 2.6);
    m.soft((mm) => mm.loft([
      { z: 0, pts: ring(r * 0.98, r * 0.98, seg) },
      { z: -r * 1.2, pts: ring(r * 0.6, r * 0.6, seg) },
    ], shade(P.black, 1.35), false));
    m.move(0, 0, -r * 1.2);
    discCap(m, r * 0.6, seg, P.sensor, false);
    m.restore();
    m.restore();
  }

  /// A deployable mast, ON ITS OWN HINGE. A magnetometer boom, a gravity-
  /// gradient boom and an antenna mast are the same object: a root hinge with a
  /// trunnion either side of it, a truss that unfolds out of the bus, and
  /// whatever is on the end. The hinge is the pivot and it is drawn, because a
  /// mast growing straight out of a flat panel is a spike.
  function deployMast(m, o) {
    const t = o.t;
    m.save().move(o.x, o.y, o.z).rotY(o.yaw || 0).rotX(o.pitch || 0);
    m.bar(-t * 2.4, t * 2.4, -t * 2.2, t * 2.2, -t * 2, t * 0.6,
      shade(P.metal, 0.56));
    if (!m.far) {
      m.both((mm) => {
        mm.save().move(t * 2.4, 0, -t * 0.6).rotY(90);
        mm.soft((k) => k.tube(t * 1.3, t * 1.3, t * 0.7, k.lod(9, 5),
          shade(P.metal, 0.82), false));
        discCap(mm, t * 1.3, mm.lod(9, 5), shade(P.metal, 0.6), true);
        mm.save().move(0, 0, t * 0.7);
        discCap(mm, t * 1.3, mm.lod(9, 5), shade(P.metal, 0.95), false);
        mm.restore();
        mm.restore();
      });
    }
    lattice(m, { z: t * 0.6, len: o.len, w: t * 1.4, h: t * 1.4, t: t * 0.3,
      n: o.n || 5, col: o.col || shade(P.metal, 0.66) });
    if (!m.far) {
      m.save().move(0, 0, o.len + t * 0.6);
      m.bar(-t * 1.9, t * 1.9, -t * 1.9, t * 1.9, 0, t * 2.8, o.tip || P.sensor);
      m.restore();
    }
    m.restore();
  }

  /// The optical assembly, and on one of these models it IS the model: a
  /// sunshade with a scalloped lip, the barrel with its metering hoops and
  /// blanket, the stack of baffle vanes down the bore, the secondary on its
  /// spider and the primary at the bottom of it.
  ///
  /// THE BORE IS MODELLED AND NOT CAPPED, which is the single decision that
  /// makes this read as a telescope. A card at three-quarter view CAN see down
  /// an aperture this size; a disc across the mouth would have been four
  /// triangles and a lie. The same call serves the small nadir barrel a recon
  /// bus carries — the scallops, hoops and blanket are counts, and zero is a
  /// legal count.
  function telescope(m, o) {
    const seg = o.seg || m.lod(24, 10), R = o.r, L = o.len, S = o.hood;
    const col = o.col || P.gold, bore = R * 0.86;
    const mz = L * 0.2, dep = R * 0.34;
    m.save().move(o.x || 0, o.y || 0, o.z || 0).rotY(o.yaw || 0).rotX(o.pitch || 0);
    // The barrel. A metering structure IS a cylinder and it is the one shape in
    // this class that must not read as a prism, so it is smoothed — and its aft
    // face is closed outside the group, for the reason `discCap` exists.
    m.soft((mm) => mm.loft([
      { z: 0, pts: ring(bore, bore, seg) },
      { z: L * 0.05, pts: ring(R, R, seg) },
      { z: L * 0.55, pts: ring(R, R, seg) },
      { z: L * 0.74, pts: ring(R * 0.985, R * 0.985, seg) },
      { z: L, pts: ring(R * 0.985, R * 0.985, seg) },
    ], col, false));
    discCap(m, bore, seg, shade(col, 0.66), true);
    // The sunshade. A third of the length of one of these is light shade, and
    // it is what makes the silhouette a telescope rather than a tank.
    m.save().move(0, 0, L);
    m.soft((mm) => mm.loft([
      { z: 0, pts: ring(R * 0.985, R * 0.985, seg) },
      { z: S * 0.45, pts: ring(R * 1.012, R * 1.012, seg) },
      { z: S, pts: ring(R * 1.05, R * 1.05, seg) },
    ], shade(col, 1.1), false));
    m.restore();
    // The bore, run back down the inside to the primary.
    m.save().move(0, 0, L + S);
    m.soft((mm) => mm.loft([
      { z: 0, pts: ring(R * 1.05, R * 1.05, seg) },
      { z: -S * 0.9, pts: ring(R * 0.93, R * 0.93, seg) },
      { z: -(S + L - mz), pts: ring(bore, bore, seg) },
    ], shade(P.black, 1.3), false));
    m.restore();
    // The baffle vanes: annular rings that stop skylight walking down the wall
    // onto the mirror. Two sections at the same station, so each is one flat
    // annulus facing the aperture — the cheapest ring in the file and the one
    // that gives the bore its depth.
    const vanes = m.far ? 0 : (o.vanes == null ? 3 : o.vanes);
    for (let i = 0; i < vanes; i += 1) {
      const f = vanes === 1 ? 0.5 : i / (vanes - 1);
      const z = mz + (L - mz) * (0.24 + 0.6 * f), ir = bore * (0.74 - i * 0.04);
      m.loft([{ z, pts: ring(bore * 0.995, bore * 0.995, seg) },
        { z, pts: ring(ir, ir, seg) }], shade(P.black, 1.05 + i * 0.07), false);
    }
    // The primary, rim first so it winds concave. Its rim IS the bore radius,
    // so the two meet with no annulus for the eye to see the barrel's own back
    // face through.
    const mr = m.lod(4, 2), sec = [];
    for (let i = 0; i <= mr; i += 1) {
      const t = i / mr, r = Math.max(1e-3, bore * t);
      sec.push({ z: mz - dep * (1 - t * t), pts: ring(r, r, seg) });
    }
    m.soft((mm) => mm.loft(sec.slice().reverse(), o.mirror || shade(P.glass, 1.25),
      false));
    if (!m.far) {
      // The baffle round the hole in the middle of the primary, which is where
      // the light actually leaves a Cassegrain, and which closes the vertex.
      m.save().move(0, 0, mz - dep);
      m.soft((mm) => mm.tube(bore * 0.18, bore * 0.15, dep * 0.55, m.lod(12, 6),
        shade(P.black, 1.5), false));
      m.save().move(0, 0, dep * 0.55);
      discCap(m, bore * 0.15, m.lod(12, 6), P.black, false);
      m.restore();
      m.restore();
    }
    // The secondary on its four-vane spider, up the bore where a card can see
    // it. This is the part of a telescope that is unmistakably a telescope, and
    // it costs a hundred and forty triangles. `spider: false` is for a barrel
    // drawn small enough that nothing down the bore resolves at all.
    if (!m.far && o.spider !== false) {
      const sz = mz + (L - mz) * 0.9;
      for (let i = 0; i < 4; i += 1) {
        m.save().move(0, 0, sz).rotZ(i * 90);
        m.bar(0, bore, -R * 0.012, R * 0.012, -R * 0.018, R * 0.018,
          shade(P.metal, 0.72));
        m.restore();
      }
      m.save().move(0, 0, sz);
      m.soft((mm) => mm.loft([
        { z: -R * 0.05, pts: ring(bore * 0.26, bore * 0.26, seg) },
        { z: 0, pts: ring(bore * 0.28, bore * 0.28, seg) },
        { z: R * 0.09, pts: ring(bore * 0.22, bore * 0.22, seg) },
      ], shade(P.metal, 0.8), false));
      m.save().move(0, 0, -R * 0.05);
      discCap(m, bore * 0.26, seg, o.mirror || shade(P.glass, 1.25), true);
      m.restore();
      m.save().move(0, 0, R * 0.09);
      discCap(m, bore * 0.22, seg, shade(P.metal, 0.58), false);
      m.restore();
      m.restore();
    }
    const hoops = m.far ? 0 : (o.hoops == null ? 3 : o.hoops);
    for (let i = 0; i < hoops; i += 1) {
      const f = hoops === 1 ? 0.5 : i / (hoops - 1);
      jointBand(m, { z: L * (0.12 + 0.7 * f), r: R, w: R * 0.28, k: 1.06, seg,
        col: shade(col, 0.86) });
    }
    if (o.quilt) {
      // The blanket, laid on an eight-sided figure whose APOTHEM is the barrel
      // radius, so the panels lie tangent to the tube instead of hovering off
      // it. Same sum as the bus, one line of trigonometry apart.
      mliQuilt(m, { n: 8, r: R / Math.cos(Math.PI / 8), z0: L * 0.07, z1: L * 0.7,
        rows: o.quilt, t: R * 0.05, col: shade(col, 1.04) });
    }
    const teeth = m.far ? 0 : (o.teeth == null ? 12 : o.teeth);
    for (let i = 0; i < teeth; i += 1) {
      // The scalloped lip. A light shade's forward edge is cut into petals so
      // the rim never presents one continuous bright line to the optic, and it
      // is the most recognisable thing about the front of one of these.
      const a = ((i + 0.5) / teeth) * Math.PI * 2;
      const ap = R * 1.05 * Math.cos(Math.PI / teeth);
      const w = 2 * R * 1.05 * Math.sin(Math.PI / teeth);
      m.save().move(Math.cos(a) * ap, Math.sin(a) * ap, L + S).rotZ(a / DEG - 90);
      m.slab([[-w * 0.46, 0], [w * 0.46, 0], [0, R * 0.3]], R * 0.035, R * 0.01,
        shade(col, 1.14));
      m.restore();
    }
    if (!m.far) {
      for (let i = 0; i < 4; i += 1) {
        m.save().rotZ(i * 90 + 45);
        m.bar(R * 0.985, R * 1.07, -R * 0.05, R * 0.05, L * 0.07, L * 0.7,
          shade(col, 0.74));
        m.restore();
      }
    }
    m.restore();
  }

  /// A spacecraft: the bus, its two solar wings on their drives, and the
  /// fittings that make a box a vehicle. THE ARGUMENTS DID NOT MOVE — `bus`,
  /// `span`, `panelW`, `bays` and `col` still mean what they meant, so a table
  /// written against the old call puts the wings exactly where it meant to.
  /// What changed is what the call builds out of them.
  ///
  /// `trim` is the one addition and it is an ART decision rather than a level
  /// of detail: the two craft flying in company on the constellation card are
  /// drawn at a third of the lead ship's size, where a blanket panel would be
  /// two centimetres across on a twelve-metre model. They carry the structure
  /// and not the quilting, and the triangles that buys go into the ship the
  /// card is actually about.
  function satellite(m, o) {
    const B = o.bus, span = o.span, R = Math.max(B[0], B[1]) / 2;
    spaceBus(m, { r: R, len: B[2], col: o.col || P.gold,
      quilt: !o.trim, rows: o.rows == null ? 3 : o.rows });
    const drive = span * 0.03;
    m.both((mm, s) => {
      solarWing(mm, {
        x: R * 0.94, drive, inner: drive + span * 0.05,
        span: span / 2 - R * 0.94, w: o.panelW, bays: o.bays || 3,
        t: span * 0.005, r: span * 0.016,
        // BOTH WINGS TRACK THE SAME SUN, so both get the SAME number — and
        // that is worth a line, because the reflex is to negate it. `both`
        // mirrors in x and a mirror usually reverses a rotation, but this one
        // is ABOUT the mirrored axis: diag(-1,1,1) commutes with rotX, so the
        // mirrored wing comes out at the angle it was given rather than at
        // minus it. Negating here was the first draft and it splayed the pair
        // forty degrees apart — visible the moment the model was looked at
        // down its own axis, and invisible to every assertion in the checks.
        rot: o.rot || 0,
        cols: o.cols || 4, rows: o.cells || 5, plain: !!o.trim,
        frame: o.frame || shade(P.metal, 0.6), cell: o.cell || P.panel,
      });
    });
    return m;
  }

  /// One craft of the reconnaissance constellation, written once and flown
  /// three times — a constellation is identical satellites in different
  /// attitudes, and drawing three different spacecraft would be drawing three
  /// different programmes. `lead` is the one the card is about.
  function reconCraft(m, lead) {
    satellite(m, { bus: [1.6, 1.9, 3.4], span: 11.0, panelW: 2.1, bays: 3,
      col: P.gold, rot: 15, trim: !lead, cols: lead ? 4 : 3, cells: lead ? 5 : 3 });
    // At MAP size the whole constellation is a pin, and the two in company are
    // three pixels of bus and array between them. They keep the silhouette
    // everything else in this class is recognised by and drop the rest.
    if (!lead && m.far) return;
    // The payload looks DOWN, because what this thing is for is the ground, and
    // the optical axis is the one line on it that says so.
    telescope(m, { y: -0.9, z: -0.5, pitch: 90, r: 0.5, len: 1.7, hood: 0.55,
      seg: m.lod(lead ? 16 : 11, 8), hoops: lead ? 2 : 0, teeth: 0,
      quilt: lead ? 2 : 0, vanes: lead ? 2 : 0, spider: lead,
      col: shade(P.gold, 0.94), mirror: shade(P.glass, 1.2) });
    // The high-gain antenna off the forward face, trained off-axis: a relay
    // satellite is somewhere else in the sky, and a dish aimed straight ahead
    // is a dish nobody aimed.
    spaceDish(m, { y: 0.25, z: 1.85, r: 0.8, az: -24, el: 30,
      seg: m.lod(lead ? 16 : 11, 8), rings: m.lod(lead ? 5 : 3, 2),
      ribs: lead ? 5 : 0, feed: lead });
    if (!lead) return;
    deployMast(m, { x: -0.7, y: 0.66, z: -1.4, pitch: -30, yaw: -22, t: 0.05,
      len: 2.4, n: 4, col: shade(P.metal, 0.7) });
    [[1, 1], [-1, 1], [1, -1], [-1, -1]].forEach((c) => {
      thrusterPod(m, { x: c[0] * 0.62, y: c[1] * 0.62, z: -1.7, r: 0.14,
        pitch: 0, yaw: 180 });
    });
    starTracker(m, { x: 0.62, y: -0.5, z: 0.9, r: 0.09, yaw: 40, pitch: 30 });
    starTracker(m, { x: -0.62, y: -0.5, z: 0.9, r: 0.09, yaw: -40, pitch: 30 });
    whip(m, { x: 0, y: 0.92, z: -1.1, r: 0.014, len: 0.9, lean: 12 });
    bladeAerial(m, { x: 0.5, y: -0.86, z: 1.1, h: 0.16, len: 0.3, t: 0.03,
      col: shade(P.gold, 0.8) });
  }

  // ------------------------------------------------------- people & sensors
  // --------------------------------------------------------- the infantry kit
  // WHAT AN INFANTRY CARD HAS TO SHOW THAT NO OTHER CLASS DOES: people, the
  // load they carry, running gear that runs inside a track rather than on a
  // road, and — for the two entries that are not vehicles at all — a rack of
  // boxes that is honestly a rack of boxes.
  //
  // EVERY PIECE BELOW IS COMPOSED BESIDE THE SHARED KIT, NEVER INSIDE IT.
  // `armour`, `trackRun`, `truck`, `wheels`, `roadWheel`, `dish` and
  // `arrayFace` are each shared with the Armour, Missile, Naval or Space
  // classes, and those classes are raised by other passes; moving one of them
  // would move models this pass is not allowed to touch. So where this class
  // wants a track with links in it, a wheel that shows its dish, or an array
  // face with elements on it, it gets its own here and the shared call stays
  // exactly where it was.

  /// A flat disc closing a ring, emitted OUTSIDE any smoothing group. `loft`'s
  /// own caps join whatever group they were lofted in, which rounds the rim off
  /// a can that is supposed to show a lip; this keeps the rim hard.
  function discCap(m, r, seg, col, back) {
    const pts = ring(r, r, seg).map((p) => [p[0], p[1], 0]);
    m.fan(back ? pts.slice().reverse() : pts, col);
  }

  /// A limb segment: a smoothed tapered tube running down from the joint at the
  /// local origin, with a ball at each end. WHY THE BALLS ARE WORTH THEIR
  /// TRIANGLES — an arm built as two pipes that stop where they meet shows the
  /// gap between them from every angle a card is ever drawn at, and a shoulder,
  /// an elbow and a knee are round. Pitch is measured from straight down, so a
  /// leg is a small number and an arm bent up onto a weapon is a large one.
  function limb(m, o) {
    const seg = m.lod(10, 5), rings = m.lod(3, 2);
    m.save().move(o.x, o.y, o.z).rotZ(o.roll || 0).rotX(90 + (o.pitch || 0));
    m.soft((mm) => mm.tube(o.r0, o.r1, o.len, seg, o.col, m.far));
    // A map pin is nine pixels of person: the taper is the limb and the joints
    // it turns about are below the size of a single one.
    if (!m.far) {
      m.soft((mm) => mm.ball(o.r0, seg, rings, o.col));
      m.save().move(0, 0, o.len);
      m.soft((mm) => mm.ball(o.r1, seg, rings, o.col));
      m.restore();
    }
    m.restore();
  }

  /// The weapon, modelled rather than suggested. At 90 px the barrel line and
  /// the magazine hanging under it are what say "rifleman"; a single black bar
  /// says "stick". Receiver, handguard, barrel, muzzle device, magazine, grip,
  /// stock and optic — none of them more than a dozen triangles, and together
  /// they are a rifle. Points down local +Z.
  function rifle(m, o) {
    const col = o.col || P.black, seg = m.lod(8, 5);
    m.save().move(o.x, o.y, o.z).rotY(o.yaw || 0).rotX(o.pitch || 0).rotZ(o.roll || 0);
    m.bar(-0.024, 0.024, -0.028, 0.046, -0.1, 0.16, shade(col, 1.35));
    if (m.far) { m.restore(); return; }
    // Handguard, then the barrel out of the front of it and the flash hider on
    // the end. Three diameters in a row is what a muzzle actually looks like.
    m.save().move(0, 0.012, 0.16);
    m.soft((mm) => mm.loft([
      { z: 0, pts: ringSuper(0.027, 0.027, seg, 3.2) },
      { z: 0.2, pts: ringSuper(0.025, 0.025, seg, 3.2) },
    ], shade(col, 1.15), false));
    m.restore();
    m.save().move(0, 0.012, 0.36);
    m.soft((mm) => mm.tube(0.0095, 0.0085, 0.13, seg, shade(P.metal, 0.42), false));
    m.restore();
    m.save().move(0, 0.012, 0.49);
    m.soft((mm) => mm.tube(0.017, 0.015, 0.055, seg, shade(P.metal, 0.38), false));
    discCap(m, 0.008, seg, P.black, false);
    m.restore();
    // Magazine, raked forward as a curved one is, and the pistol grip behind it.
    m.save().move(0, -0.028, 0.02).rotX(-14);
    m.bar(-0.017, 0.017, -0.15, 0, -0.042, 0.042, shade(col, 0.88));
    m.restore();
    m.save().move(0, -0.026, -0.05).rotX(20);
    m.bar(-0.017, 0.017, -0.11, 0, -0.026, 0.026, shade(col, 1.05));
    m.restore();
    // Stock and cheek piece aft, optic and its mount on the rail.
    m.save().move(0, 0.004, -0.1);
    m.slab([[-0.026, -0.19], [0.026, -0.19], [0.026, 0], [-0.026, 0]], 0.062, 0.014,
      shade(col, 0.95));
    m.restore();
    m.save().move(0, 0.05, 0.09);
    m.bar(-0.014, 0.014, 0, 0.026, -0.05, 0.05, shade(col, 1.2));
    m.save().move(0, 0.026, 0);
    m.soft((mm) => mm.tube(0.021, 0.021, 0.11, seg, shade(P.sensor, 1.2), false));
    m.save().move(0, 0, 0.11);
    discCap(m, 0.019, seg, shade(P.glass, 1.1), false);
    m.restore();
    m.restore();
    m.restore();
    m.restore();
  }

  /// One soldier, about 1.8 m, and the most-repeated object in this class:
  /// three of them ARE the light formation and two more ride on the mechanised
  /// one. Infantry formations are the only entries in the deck whose unit is
  /// people, and a card that draws them as a vehicle is lying about what the
  /// money buys — so what has to read at 90 px is a person under a load, not a
  /// cross of boxes.
  ///
  /// SHADING IS SPLIT THE WAY THE KIT IS, and the line falls between a SHELL
  /// and a THING STOOD ON ONE — not between skin and fabric, which is where an
  /// earlier draft of this comment put it and where the code never did. Every
  /// shell lofted round the figure carries averaged normals: trunk, hips, neck,
  /// head, limbs and their joint balls, the helmet and its brim band, the plate
  /// carrier's own shell, the daysack, and the rifle's handguard, barrel, flash
  /// hider and optic body. All of those are bodies of revolution drawn at
  /// twelve segments, so one normal per triangle makes each of them a prism
  /// sitting on a smooth man — which reads worse than either choice made
  /// consistently. Everything that stands PROUD of a shell stays hard: the
  /// pouches, the night-vision mount, the pack straps, the knee pads, the
  /// boots, and the rifle's receiver, magazine, grip and stock. That split is
  /// the load-carriage story — the shadow where a pouch stands off the chest is
  /// the whole of it, and smoothing across that join would wipe it off.
  function soldier(m, o) {
    const c = o.col || P.cloth;
    const gear = shade(c, 0.74), seg = m.lod(12, 5), s = o.side == null ? 1 : o.side;
    m.save().move(o.x || 0, 0, o.z || 0).rotY(o.face || 0);
    // The trunk, lofted up +Y in a frame rolled a quarter turn, so one table of
    // half-widths and half-depths gives hips, waist, chest and shoulders in the
    // order a body has them. A superellipse rather than an ellipse because a
    // chest is deeper across than it is round, and a round one is a barrel.
    m.save().rotX(-90);
    m.soft((mm) => mm.loft([[0.84, .152, .112], [0.95, .140, .100], [1.06, .157, .112],
      [1.18, .183, .124], [1.26, .195, .128], [1.34, .188, .118], [1.41, .118, .086]]
      .map((k) => ({ z: k[0], pts: ringSuper(k[1], k[2], seg, 2.5) })), c, false));
    m.restore();
    // Hips and thighs, then the shins and the boots. The knee ball is what
    // makes a leg bend instead of kink.
    m.save().rotX(-90);
    m.soft((mm) => mm.loft([[0.62, .175, .118], [0.78, .172, .118], [0.9, .158, .114]]
      .map((k) => ({ z: k[0], pts: ringSuper(k[1], k[2], seg, 2.6) })), shade(c, 0.94), false));
    m.restore();
    m.both((mm, sg) => {
      limb(mm, { x: 0.085, y: 0.66, z: sg > 0 ? 0.02 : -0.02, len: 0.3, r0: 0.075, r1: 0.06,
        pitch: sg > 0 ? -4 : 5, col: shade(c, 0.98) });
      limb(mm, { x: 0.085, y: 0.36, z: sg > 0 ? 0.04 : -0.05, len: 0.28, r0: 0.058, r1: 0.045,
        pitch: sg > 0 ? 3 : -3, col: shade(c, 0.9) });
      mm.save().move(0.085, 0.0, sg > 0 ? 0.04 : -0.03);
      mm.bar(-0.055, 0.055, 0, 0.09, -0.075, 0.13, shade(P.black, 1.25));
      mm.restore();
    });
    // Arms, brought up onto the weapon. Shoulder, elbow and hand are three
    // balls and two tapers; nothing else about a person is as easy to get wrong.
    limb(m, { x: 0.19 * s, y: 1.3, z: 0.0, len: 0.27, r0: 0.062, r1: 0.05, pitch: 16 * s,
      roll: -14 * s, col: shade(c, 1.04) });
    limb(m, { x: 0.19 * s + 0.07 * s, y: 1.03, z: 0.02, len: 0.25, r0: 0.05, r1: 0.042,
      pitch: 62, roll: -6 * s, col: shade(c, 1.0) });
    limb(m, { x: -0.19 * s, y: 1.3, z: 0.0, len: 0.26, r0: 0.062, r1: 0.05, pitch: 26 * s,
      roll: 10 * s, col: shade(c, 1.04) });
    limb(m, { x: -0.19 * s - 0.05 * s, y: 1.06, z: 0.06, len: 0.22, r0: 0.05, r1: 0.04,
      pitch: 74, roll: 8 * s, col: shade(c, 1.0) });
    // Neck, head, and the helmet over it. The helmet is a shell of revolution
    // with a separate brim band, which is the line a card reads as a helmet.
    m.save().rotX(-90);
    m.soft((mm) => mm.loft([[1.38, .052, .05], [1.48, .056, .054]]
      .map((k) => ({ z: k[0], pts: ring(k[1], k[2], seg) })), P.skin, false));
    m.restore();
    m.save().move(0, 1.575, 0.008).scale(1, 1.1, 1.06);
    m.soft((mm) => mm.ball(0.087, seg, m.lod(6, 3), P.skin));
    m.restore();
    m.save().move(0, 1.6, 0.004).rotX(-90);
    m.soft((mm) => mm.loft([
      { z: -0.03, pts: ringSuper(0.113, 0.125, seg, 2.3) },
      { z: 0.02, pts: ringSuper(0.116, 0.128, seg, 2.3) },
      { z: 0.07, pts: ringSuper(0.104, 0.114, seg, 2.3) },
      { z: 0.105, pts: ringSuper(0.072, 0.079, seg, 2.3) },
      { z: 0.125, pts: ringSuper(0.026, 0.028, seg, 2.3) },
    ], shade(c, 1.16), false));
    m.restore();
    if (m.far) { m.restore(); return; }
    // The helmet's brim band and its night-vision mount: two parts, and the
    // mount is the one silhouette detail that dates the figure to this century.
    m.save().move(0, 1.57, 0.004).rotX(-90);
    m.soft((mm) => mm.loft([
      { z: 0, pts: ringSuper(0.113, 0.125, seg, 2.3) },
      { z: 0.014, pts: ringSuper(0.122, 0.134, seg, 2.3) },
      { z: 0.03, pts: ringSuper(0.118, 0.13, seg, 2.3) },
    ], shade(c, 0.86), false));
    m.restore();
    m.save().move(0, 1.66, 0.115);
    m.slab([[-0.03, -0.03], [0.03, -0.03], [0.03, 0.03], [-0.03, 0.03]], 0.05, 0.014,
      shade(P.sensor, 1.2));
    m.restore();
    // The plate carrier: the same section a touch larger over the chest run, so
    // the armour sits ON the man rather than being painted across him. Then the
    // pouch line across the front, which is what a load looks like.
    m.save().rotX(-90);
    m.soft((mm) => mm.loft([[1.0, .168, .128], [1.06, .176, .136], [1.24, .2, .146],
      [1.31, .19, .132]].map((k) => ({ z: k[0], pts: ringSuper(k[1], k[2], seg, 2.4) })),
    gear, false));
    m.restore();
    [[-0.105, 1.06], [0, 1.06], [0.105, 1.06], [-0.06, 1.17], [0.06, 1.17]].forEach((p, i) => {
      m.save().move(p[0], p[1], 0.144).rotX(90);
      m.slab([[-0.045, -0.05], [0.045, -0.05], [0.045, 0.05], [-0.045, 0.05]], 0.052, 0.014,
        shade(gear, 1 + ((i % 3) - 1) * 0.07));
      m.restore();
    });
    // The daysack on the back, its two straps over the shoulders, and the
    // knee pads. The pack is a shell like the carrier and is smoothed with it;
    // the straps and the pads are what stand off it, so they stay hard and keep
    // their own edge against the pack behind them.
    m.save().move(0, 1.02, -0.13).rotX(-90);
    m.soft((mm) => mm.loft([
      { z: 0, pts: ringSuper(0.13, 0.05, seg, 3.0) },
      { z: 0.08, pts: ringSuper(0.15, 0.085, seg, 3.0) },
      { z: 0.32, pts: ringSuper(0.15, 0.085, seg, 3.0) },
      { z: 0.4, pts: ringSuper(0.12, 0.05, seg, 3.0) },
    ], shade(gear, 0.9), false));
    m.restore();
    m.both((mm) => {
      mm.save().move(0.1, 1.18, -0.02).rotX(-8);
      mm.bar(-0.032, 0.032, -0.14, 0.14, -0.075, -0.03, shade(gear, 1.1));
      mm.restore();
      mm.save().move(0.085, 0.36, 0.055);
      mm.slab([[-0.05, -0.06], [0.05, -0.06], [0.05, 0.06], [-0.05, 0.06]], 0.05, 0.014,
        shade(gear, 0.94));
      mm.restore();
    });
    rifle(m, { x: 0.12 * s, y: 1.09, z: 0.16, yaw: -10 * s, pitch: -6, roll: 8 * s });
    m.restore();
  }

  /// A sandbag: a squashed loft with both ends pinched, laid in a course. A bar
  /// is a brick; a bag sags, and the sag is the whole reason a hasty position
  /// looks hand-built rather than poured.
  function sandbag(m, o) {
    const seg = m.lod(8, 5);
    m.save().move(o.x, o.y, o.z).rotY(o.yaw || 0);
    m.soft((mm) => mm.loft([
      { z: -o.len / 2, pts: ringSuper(o.w * 0.34, o.h * 0.42, seg, 3.0) },
      { z: -o.len * 0.28, pts: ringSuper(o.w * 0.95, o.h * 0.94, seg, 3.0) },
      { z: o.len * 0.28, pts: ringSuper(o.w, o.h, seg, 3.0) },
      { z: o.len / 2, pts: ringSuper(o.w * 0.36, o.h * 0.44, seg, 3.0) },
    ], o.col, true));
    m.restore();
  }

  /// A run of dome-head fasteners. A panel that meets another panel with
  /// nothing at the join is printed on; a line of bolt heads, each with its own
  /// little highlight, is bolted on — and at the size these are drawn that is
  /// the entire difference. Skipped whole at the far level of detail.
  function boltRun(m, o) {
    if (m.far) return;
    const n = o.n, seg = m.lod(6, 4), r = o.r;
    for (let i = 0; i < n; i += 1) {
      const t = n === 1 ? 0.5 : i / (n - 1);
      m.save().move(o.x0 + ((o.x1 == null ? o.x0 : o.x1) - o.x0) * t,
        o.y0 + ((o.y1 == null ? o.y0 : o.y1) - o.y0) * t,
        o.z0 + ((o.z1 == null ? o.z0 : o.z1) - o.z0) * t)
        .rotY(o.yaw || 0).rotX(o.pitch == null ? -90 : o.pitch);
      m.soft((mm) => mm.loft([
        { z: 0, pts: ring(r, r, seg) },
        { z: r * 0.72, pts: ring(r * 0.88, r * 0.88, seg) },
        { z: r, pts: ring(r * 0.42, r * 0.42, seg) },
      ], o.col, false));
      m.restore();
    }
  }

  /// A louvre stack: the angled slats over a generator's air intake or a radio
  /// rack's fan. Every slat is a shadow, and shadow is the only way a renderer
  /// with no textures has of saying there is air moving through here.
  function louvre(m, o) {
    if (m.far) return;
    const n = o.n || 6;
    for (let i = 0; i < n; i += 1) {
      m.save().move(o.x || 0, o.y0 + (o.y1 - o.y0) * ((i + 0.5) / n), o.z || 0)
        .rotY(o.yaw || 0).rotX(32);
      m.bar(-o.w / 2, o.w / 2, -o.t, o.t, -o.d, o.d, shade(o.col, 1 - (i % 2) * 0.1));
      m.restore();
    }
  }

  /// A circular bulkhead connector: the shell, its coupling ring and the dark
  /// face of the pins. A rack with no connectors on the back of it is a stack
  /// of bricks, and connectors are how the eye knows which face is the back.
  function connector(m, o) {
    const seg = m.lod(8, 5), r = o.r;
    m.save().move(o.x, o.y, o.z).rotY(o.yaw || 0).rotX(o.pitch || 0);
    m.soft((mm) => mm.tube(r, r * 0.94, o.len, seg, o.col || shade(P.metal, 0.68), false));
    m.save().move(0, 0, o.len * 0.35);
    m.soft((mm) => mm.loft([
      { z: 0, pts: ring(r * 1.2, r * 1.2, seg) },
      { z: o.len * 0.34, pts: ring(r * 1.2, r * 1.2, seg) },
    ], shade(P.metal, 0.92), false));
    m.restore();
    m.save().move(0, 0, o.len);
    discCap(m, r * 0.9, seg, P.black, false);
    m.restore();
    m.restore();
  }

  /// A whip antenna on its insulator base: a tapered smoothed rod, the coil at
  /// the foot that lets it whip, and the shoulder it stands on. At this
  /// diameter a faceted rod reads as a bent wire, so the rod is smoothed; the
  /// coil bands stay hard because their steps are what say it is sprung.
  function whip(m, o) {
    const seg = m.lod(7, 4);
    m.save().move(o.x, o.y, o.z).rotZ(o.lean || 0).rotX(-90);
    m.soft((mm) => mm.loft([
      { z: 0, pts: ring(o.r * 2.0, o.r * 2.0, seg) },
      { z: o.len * 0.05, pts: ring(o.r * 1.5, o.r * 1.5, seg) },
      { z: o.len * 0.14, pts: ring(o.r * 1.05, o.r * 1.05, seg) },
      { z: o.len, pts: ring(o.r * 0.3, o.r * 0.3, seg) },
    ], o.col || P.black, false));
    if (!m.far) {
      for (let i = 0; i < 3; i += 1) {
        jointBand(m, { z: o.len * (0.03 + i * 0.026), r: o.r * 1.5, w: o.len * 0.014,
          k: 1.4, seg, col: shade(P.metal, 0.7) });
      }
    }
    m.restore();
  }

  /// A sensor ball with real apertures. The recognition happens BEHIND the
  /// windows, so the windows are geometry: each is a recessed lip cut into the
  /// sphere with a domed window sitting proud inside it, which is what makes a
  /// turret read as an optic instead of as a bowling ball. Apertures are given
  /// as [yaw, pitch, radius fraction, colour] and clocked round the front face.
  function gimbalBall(m, o) {
    const seg = m.lod(24, 8), rings = m.lod(12, 4), R = o.r;
    const base = o.col || P.sensor;
    m.save().move(o.x || 0, o.y || 0, o.z || 0);
    m.soft((mm) => mm.ball(R, seg, rings, base));
    // The azimuth seam. The ball is two castings bolted together and the joint
    // between them is the one line on it that says which way it turns.
    if (!m.far) {
      m.save().rotX(90);
      m.soft((mm) => mm.loft([
        { z: -R * 0.05, pts: ring(R * 0.999, R * 0.999, seg) },
        { z: 0, pts: ring(R * 1.025, R * 1.025, seg) },
        { z: R * 0.05, pts: ring(R * 0.999, R * 0.999, seg) },
      ], shade(base, 0.68), false));
      m.restore();
    }
    (o.apertures || []).forEach((a) => {
      const r = R * a[2], dr = m.lod(5, 2), secs = [];
      m.save().rotY(a[0]).rotX(-a[1]).move(0, 0, R * 0.9);
      m.soft((mm) => mm.loft([
        { z: 0, pts: ring(r * 1.26, r * 1.26, seg) },
        { z: R * 0.1, pts: ring(r * 1.2, r * 1.2, seg) },
        { z: R * 0.13, pts: ring(r * 1.04, r * 1.04, seg) },
      ], shade(base, 1.35), false));
      m.save().move(0, 0, R * 0.13);
      for (let i = 0; i <= dr; i += 1) {
        const t = (i / dr) * Math.PI * 0.5;
        const rr = Math.max(1e-3, Math.cos(t) * r * 1.02);
        secs.push({ z: Math.sin(t) * r * 0.38, pts: ring(rr, rr, seg) });
      }
      m.soft((mm) => mm.loft(secs, a[3] || P.glass, false));
      m.restore();
      m.restore();
    });
    m.restore();
  }

  /// A pyramidal horn: the aperture a jammer radiates through, and a cluster of
  /// them is what a directional electronic-attack head actually is. Rectangular
  /// sections rather than round, because the flare of a horn is rectangular and
  /// that is exactly what separates it from a dish feed at card size. The wall
  /// runs back down the inside to a dark disc, so the mouth reads as a mouth.
  function horn(m, o) {
    const seg = 8, W = o.w, L = o.len;
    const rect = (k) => ringSuper(W * k, W * k * 0.84, seg, 5);
    m.save().move(o.x || 0, o.y || 0, o.z || 0).rotY(o.yaw || 0).rotX(o.pitch || 0);
    m.loft([{ z: 0, pts: rect(0.2) }, { z: L * 0.22, pts: rect(0.24) },
      { z: L, pts: rect(0.5) }], o.col || shade(P.metal, 0.72), false);
    m.loft([{ z: L, pts: rect(0.46) }, { z: L * 0.3, pts: rect(0.2) },
      { z: L * 0.1, pts: rect(0.17) }], shade(P.sensor, 0.9), false);
    m.save().move(0, 0, L * 0.1);
    m.fan(rect(0.17).map((p) => [p[0], p[1], 0]), P.black);
    m.restore();
    m.restore();
  }

  /// A phased-array face with its elements. `arrayFace` above is shared with
  /// the AESA refit and two warships and may not move, so the counter-UAS
  /// battery's own face is composed here: a chamfered frame and the grid of
  /// radiating elements that is the entire difference between an array and a
  /// painted rectangle. The face is built lying down and then stood up, so the
  /// outline is written [across, up] as an array is quoted.
  function arrayGrid(m, o) {
    const W = o.w, H = o.h, t = o.t;
    m.save().move(o.x || 0, o.y || 0, o.z || 0).rotY(o.yaw || 0).rotX(90 - (o.tilt || 0));
    m.slab([[-W / 2, -H / 2], [W / 2, -H / 2], [W / 2, H / 2], [-W / 2, H / 2]],
      t, t * 0.3, shade(o.col || P.sensor, 0.86));
    const cols = m.lod(o.cols || 6, 1), rows = m.lod(o.rows || 8, 1);
    const cw = W * 0.88 / cols, rh = H * 0.88 / rows;
    for (let r = 0; r < rows; r += 1) {
      for (let k = 0; k < cols; k += 1) {
        m.save().move(-W * 0.44 + cw * (k + 0.5), t * 0.4, -H * 0.44 + rh * (r + 0.5));
        m.slab([[-cw * 0.36, -rh * 0.36], [cw * 0.36, -rh * 0.36],
          [cw * 0.36, rh * 0.36], [-cw * 0.36, rh * 0.36]], t * 0.55, t * 0.16,
        shade(P.glass, 0.82 + ((r + k) % 3) * 0.08));
        m.restore();
      }
    }
    m.restore();
  }

  /// One line-replaceable unit in a rack: the chassis, its front panel, the two
  /// pull handles a fitter grabs, the card slots down the face and the status
  /// lamps beside them. THE TWO ABSTRACT FITS IN THIS CLASS ARE EQUIPMENT AND
  /// NOT VEHICLES — a data link and a recognition stack are things bolted into
  /// something else — and the honest way to draw equipment is as the boxes it
  /// ships in. The face is toward +Z.
  function rackUnit(m, o) {
    const W = o.w, H = o.h, D = o.d, col = o.col || shade(P.greyDark, 1.05);
    m.save().move(o.x, o.y, o.z);
    m.bar(-W / 2, W / 2, 0, H, -D, 0, shade(col, 0.88));
    if (m.far) { m.restore(); return; }
    m.save().move(0, H / 2, D * 0.04).rotX(90);
    m.slab([[-W / 2, -H / 2], [W / 2, -H / 2], [W / 2, H / 2], [-W / 2, H / 2]],
      D * 0.08, D * 0.025, col);
    m.restore();
    // Handles at the outer ends of the panel, slots and lamps between them.
    m.both((mm) => {
      mm.save().move(W * 0.42, H * 0.5, D * 0.1);
      mm.bar(-W * 0.045, W * 0.045, -H * 0.3, H * 0.3, -D * 0.03, D * 0.03,
        shade(P.metal, 0.66));
      mm.restore();
    });
    const slots = o.slots == null ? 4 : o.slots;
    for (let i = 0; i < slots; i += 1) {
      const x = -W * 0.28 + (W * 0.56) * ((i + 0.5) / slots);
      m.save().move(x, H * 0.5, D * 0.07);
      m.bar(-W * 0.55 / slots * 0.34, W * 0.55 / slots * 0.34, -H * 0.32, H * 0.32,
        -D * 0.02, D * 0.02, shade(col, 1.14));
      m.restore();
    }
    [0.2, 0.42, 0.62].forEach((f, i) => {
      m.save().move(-W * 0.36, H * f, D * 0.09);
      m.bar(-W * 0.018, W * 0.018, -H * 0.05, H * 0.05, -D * 0.012, D * 0.012,
        i === 1 ? shade(P.red, 1.3) : shade(P.glass, 1.2));
      m.restore();
    });
    m.restore();
  }

  /// A dished road wheel. `roadWheel` above is the LORRY's — rim flange, stud
  /// circle, tread lugs — and this is the tracked vehicle's, which is a
  /// different object: a road wheel runs inside a track, so it never shows a
  /// tread and always shows its dish and its hub. Smoothed on the rubber and
  /// the dish, hard at the two rims, which is where the light catches it.
  function dishedWheel(m, o) {
    const seg = m.lod(12, 6), R = o.r, W = o.w;
    m.save().move(o.x, o.y, o.z).rotY(90);
    m.soft((mm) => mm.loft([
      { z: 0, pts: ring(R * 0.5, R * 0.5, seg) },
      { z: W * 0.16, pts: ring(R * 0.96, R * 0.96, seg) },
      { z: W * 0.3, pts: ring(R, R, seg) },
      { z: W * 0.7, pts: ring(R, R, seg) },
      { z: W * 0.84, pts: ring(R * 0.96, R * 0.96, seg) },
      { z: W, pts: ring(R * 0.5, R * 0.5, seg) },
    ], o.col || P.rubber, false));
    discCap(m, R * 0.5, seg, shade(P.track, 1.35), true);
    m.save().move(0, 0, W);
    discCap(m, R * 0.5, seg, shade(P.track, 1.35), false);
    if (!m.far) {
      m.soft((mm) => mm.tube(R * 0.26, R * 0.2, W * 0.18, mm.lod(9, 5),
        shade(P.metal, 0.6), false));
      m.save().move(0, 0, W * 0.18);
      discCap(m, R * 0.2, m.lod(9, 5), shade(P.metal, 0.85), false);
      m.restore();
    }
    m.restore();
    m.restore();
  }

  /// Where one track link sits on the loop, as [z, y, roll]. The path is a
  /// stadium: the bottom run, the arc over the sprocket, the top run and the
  /// arc over the idler, walked as one parameter so the links come out evenly
  /// spaced round the whole loop instead of bunching at the corners.
  function trackStation(t, o) {
    const straight = o.z1 - o.z0, arc = Math.PI * o.r;
    let d = t * 2 * (straight + arc);
    if (d < straight) return [o.z0 + d, o.cy - o.r, 0];
    d -= straight;
    if (d < arc) {
      const p = d / o.r;
      return [o.z1 + Math.sin(p) * o.r, o.cy - Math.cos(p) * o.r, -p / DEG];
    }
    d -= arc;
    if (d < straight) return [o.z1 - d, o.cy + o.r, -180];
    const p = Math.PI + (d - straight) / o.r;
    return [o.z0 + Math.sin(p) * o.r, o.cy - Math.cos(p) * o.r, -p / DEG];
  }

  /// A track run built out of links. `trackRun` above draws the band as a
  /// single box and is shared with the Armour class, so the mechanised
  /// formation gets its own running gear here: individual links with the guide
  /// horns that hold them on the sprocket, a dished road wheel at every
  /// station, a toothed sprocket and idler at the ends and the return rollers
  /// along the top. THE LINK LINE IS THE DETAIL — at card size the eye finds
  /// the running gear before it finds the turret, and a smooth band is a rubber
  /// band on a toy.
  function linkedTrack(m, o) {
    const path = { z0: o.z0, z1: o.z1, r: o.r, cy: o.r };
    if (m.far) {
      // Nine pixels hold the loop and three wheels in it. The wheel line is
      // the last thing to go: a tracked hull with a blank band under it reads
      // as a crate, which is the one thing a map pin must not do.
      m.bar(o.x - o.w / 2, o.x + o.w / 2, 0, o.r * 2, o.z0, o.z1, P.track);
      [o.z0, o.z1].forEach((z) => {
        m.save().move(o.x - o.w / 2, o.r, z).rotY(90);
        m.soft((mm) => mm.tube(o.r, o.r, o.w, 7, shade(P.track, 1.2), true));
        m.restore();
      });
      for (let i = 0; i < 3; i += 1) {
        dishedWheel(m, { x: o.x - o.w * 0.46, y: o.r * 0.84,
          z: o.z0 + o.r + (i / 2) * (o.z1 - o.z0 - o.r * 2), r: o.r * 0.84, w: o.w * 0.9 });
      }
      return;
    }
    const n = o.links == null ? 34 : o.links;
    for (let i = 0; i < n; i += 1) {
      const s = trackStation((i + 0.5) / n, path);
      m.save().move(o.x, s[1], s[0]).rotX(s[2]);
      m.bar(-o.w / 2, o.w / 2, -o.t, o.t, -o.pitch * 0.42, o.pitch * 0.42,
        shade(P.track, 1 + ((i % 3) - 1) * 0.14));
      // The guide horn stands on the INSIDE of the link, between the two
      // halves of the road wheel, and is what keeps the track on the vehicle.
      m.bar(-o.w * 0.11, o.w * 0.11, o.t, o.t * 3.0, -o.pitch * 0.2, o.pitch * 0.2,
        shade(P.track, 1.45));
      m.restore();
    }
    // Sprocket and idler, with teeth: a smooth disc at the end of a track is a
    // pulley, and the teeth are what make it drive.
    [[o.z1, 1], [o.z0, -1]].forEach((e) => {
      dishedWheel(m, { x: o.x - o.w * 0.42, y: o.r, z: e[0], r: o.r * 0.86, w: o.w * 0.84,
        col: shade(P.track, 1.5) });
      for (let i = 0; i < 10; i += 1) {
        m.save().move(o.x, o.r, e[0]).rotX(i * 36 + e[1] * 9).move(0, o.r * 0.9, 0);
        m.bar(-o.w * 0.14, o.w * 0.14, 0, o.r * 0.16, -o.pitch * 0.16, o.pitch * 0.16,
          shade(P.metal, 0.55));
        m.restore();
      }
    });
    const wheels = o.wheels == null ? 6 : o.wheels;
    for (let i = 0; i < wheels; i += 1) {
      const z = o.z0 + o.r * 0.9 + (i / Math.max(1, wheels - 1)) * (o.z1 - o.z0 - o.r * 1.8);
      dishedWheel(m, { x: o.x - o.w * 0.46, y: o.r * 0.84, z, r: o.r * 0.84, w: o.w * 0.9 });
    }
    for (let i = 0; i < 3; i += 1) {
      const z = o.z0 + (o.z1 - o.z0) * (0.24 + i * 0.26);
      dishedWheel(m, { x: o.x - o.w * 0.36, y: o.r * 2 - o.t * 2, z, r: o.r * 0.3,
        w: o.w * 0.6, col: shade(P.rubber, 1.3) });
    }
  }
  /// A flat phased-array face on a mount. This is the AESA refit, and it is
  /// also the face on the Patriot radar and the counter-UAS trailer.
  function arrayFace(m, o) {
    const R = o.r;
    m.save().move(0, o.y || 0, o.z || 0).rotX(-(o.tilt == null ? 22 : o.tilt));
    const oct = [];
    for (let i = 0; i < 8; i += 1) {
      const a = (i + 0.5) * 45 * DEG;
      oct.push([Math.cos(a) * R, Math.sin(a) * R * (o.squash || 1)]);
    }
    m.plate(oct.slice().reverse(), R * 0.16, shade(o.col || P.sensor, 1.0));
    // The face itself, a shade brighter and proud of the frame, so it reads as
    // an aperture rather than a plate.
    m.save().move(0, R * 0.1, 0);
    m.plate(oct.map((p) => [p[0] * 0.86, p[1] * 0.86]).reverse(), R * 0.05, shade(P.glass, 0.9));
    m.restore();
    m.restore();
  }

  /// A faceted section: flat belly, two chines, a ridge. This is the shape of
  /// every low-observable body in the deck, and it is why the F-117 does not
  /// come out of `jet` — a Nighthawk drawn with a round fuselage is a Nighthawk
  /// drawn wrong.
  function pent(halfW, y0, y1, ridge) {
    return [[halfW, y0], [halfW * 0.62, y1], [0, y1 + ridge], [-halfW * 0.62, y1], [-halfW, y0]];
  }
  function facetBody(m, o) {
    m.loft(o.stations.map((s) => ({
      z: s[0] * o.len,
      pts: pent(Math.max(1e-3, s[1] * o.w / 2), -s[2] * o.h * 0.35, s[2] * o.h * 0.3, s[2] * o.h * 0.35),
    })), o.col);
  }

  /// A mosaic of skin panels over a quadrilateral patch, laid in the local
  /// [x, z] plane and standing a few centimetres proud. This is what a
  /// low-observable skin actually is: not a smooth surface but a field of flat
  /// panels with radar-absorbent tape along every seam, and at card size those
  /// seams are the only texture a renderer with no textures will ever have.
  /// The shade variation between tiles is a function of the row and column
  /// index and nothing else, so the same aircraft comes out the same way in
  /// every process that builds it.
  function facetMosaic(m, o) {
    const rows = m.lod(o.rows || 4, 1), cols = m.lod(o.cols || 3, 1);
    const c = o.corners;
    const at = (u, v) => {
      const a0 = c[0][0] + (c[1][0] - c[0][0]) * u, a1 = c[0][1] + (c[1][1] - c[0][1]) * u;
      const b0 = c[3][0] + (c[2][0] - c[3][0]) * u, b1 = c[3][1] + (c[2][1] - c[3][1]) * u;
      return [a0 + (b0 - a0) * v, a1 + (b1 - a1) * v];
    };
    const gap = o.gap == null ? 0.14 : o.gap;
    for (let r = 0; r < rows; r += 1) {
      for (let k = 0; k < cols; k += 1) {
        const u0 = (k + gap * 0.5) / cols, u1 = (k + 1 - gap * 0.5) / cols;
        const v0 = (r + gap * 0.5) / rows, v1 = (r + 1 - gap * 0.5) / rows;
        m.slab([at(u0, v0), at(u1, v0), at(u1, v1), at(u0, v1)], o.t, o.t * 0.35,
          shade(o.col, 1 + (((r * 3 + k) % 5) - 2) * 0.03));
      }
    }
  }

  /// A pylon and a store under a wing, which is what turns a fighter into a
  /// strike fighter and is the only difference a card can show between them.
  /// The pylon is a chamfered wedge with its two sway braces, because that is
  /// what a store hangs from and a bare post reads as a mistake; the store
  /// itself is smooth-lofted with a seeker window, fin roots and a nozzle
  /// throat. Skipped whole at the far level of detail — a map pin does not
  /// carry ordnance.
  function store(m, x, y, z, len, r, col) {
    if (m.far) return;
    m.save().move(x, y, z);
    m.slab([[-r * 0.34, -len * 0.2], [r * 0.34, -len * 0.16], [r * 0.34, len * 0.16],
      [-r * 0.34, len * 0.2]], r * 1.5, r * 0.3, P.greyDark);
    // Sway braces: two small collars gripping the store at the ends of the
    // pylon, and the only reason a store does not float under a wing.
    [-len * 0.12, len * 0.12].forEach((dz) => {
      m.save().move(0, -r * 0.7, dz);
      m.slab([[-r * 0.7, -r * 0.16], [r * 0.7, -r * 0.16], [r * 0.7, r * 0.16],
        [-r * 0.7, r * 0.16]], r * 0.7, r * 0.14, shade(P.greyDark, 0.86));
      m.restore();
    });
    m.save().move(0, -r * 1.1, -len / 2);
    missile(m, { len, r, col: col || P.warhead, nose: len * 0.24, blunt: true, motor: false,
      seg: 12, soft: true,
      fins: [{ n: 4, z: 0.08, span: r * 1.5, chord: len * 0.2, tip: len * 0.1, roll: 45 },
        { n: 4, z: 0.66, span: r * 1.1, chord: len * 0.12, tip: len * 0.07, roll: 45 }] });
    // The seeker window and the umbilical fairing down the spine: two details,
    // both flush, and between them they say which end guides and which end
    // was bolted to the aeroplane.
    m.save().move(0, 0, len * 0.94);
    m.soft((mm) => mm.tube(r * 0.62, r * 0.5, len * 0.05, mm.lod(10, 5), P.glass, false));
    m.restore();
    m.save().move(0, r * 0.94, len * 0.42);
    m.slab([[-r * 0.24, -len * 0.2], [r * 0.24, -len * 0.2], [r * 0.24, len * 0.2],
      [-r * 0.24, len * 0.2]], r * 0.3, r * 0.1, shade(col || P.warhead, 0.86));
    m.restore();
    m.restore();
    m.restore();
  }

  /// A serrated edge: the row of triangular teeth a low-observable door, panel
  /// or intake lip is cut with, so its edge returns radar in two directions
  /// instead of in every one. It is also the most recognisable surface detail
  /// on a Raptor, a Lightning or a Spirit, and it is geometry — which is the
  /// only kind of detail this renderer has.
  function sawEdge(m, o) {
    const n = m.lod(o.n || 7, 3), step = o.len / n, t = o.t;
    for (let i = 0; i < n; i += 1) {
      m.save().move(o.x || 0, o.y || 0, o.z0 + step * (i + 0.5)).rotZ(o.roll || 0);
      m.slab([[0, -step * 0.5], [o.tooth, 0], [0, step * 0.5]], t, t * 0.3, o.col);
      m.restore();
    }
  }

  // ===================================================================== deck
  // One entry per id in `spheres-sim/src/arsenal.rs`. `span` is the largest
  // real dimension in metres and is carried so a future map layer can size
  // these against terrain instead of guessing; the card renderer ignores it and
  // fits the bounding box.
  const MODELS = {
    // ---------------------------------------------------------- the legacy tier
    inf_light: {
      name: "Light Infantry Formation", cls: "Infantry", span: 4,
      build(m) {
        // A FIRE TEAM IN A HASTY POSITION, because that is what the deck is
        // buying: three riflemen, the sandbag wall they built in twenty
        // minutes, and the section stores behind it. The soldiers are the
        // model — nothing else in this deck is priced in people — so they are
        // where the triangles go, and the position is what tells the eye they
        // are holding ground rather than standing in a field.
        soldier(m, { x: -1.05, z: 0.1, face: 10 });
        soldier(m, { x: 0.15, z: 1.15, face: -8, col: shade(P.cloth, 0.88), side: -1 });
        soldier(m, { x: 1.15, z: -0.2, face: 16, col: shade(P.cloth, 1.1) });
        // The sangar: two courses, the upper one offset half a bag so the
        // joints break, which is the one thing a sandbag wall has to get right
        // and the one thing that makes it read as a wall and not a hedge.
        const bags = m.lod(8, 4);
        for (let course = 0; course < 2; course += 1) {
          for (let i = 0; i < bags - course; i += 1) {
            const t = (i + course * 0.5) / bags;
            sandbag(m, {
              x: -1.75 + t * 3.5, y: 0.115 + course * 0.2, z: -1.55 + (course ? 0.03 : 0),
              w: 0.26, h: 0.115, len: 0.48, yaw: 4 - i * 1.6,
              col: shade(P.rust, 0.9 + ((i + course) % 3) * 0.07),
            });
          }
        }
        // The return down the flank, so the position has a corner in it.
        for (let i = 0; i < m.lod(3, 1); i += 1) {
          sandbag(m, { x: 1.72, y: 0.115, z: -1.2 + i * 0.5, w: 0.26, h: 0.115, len: 0.48,
            yaw: 90, col: shade(P.rust, 0.94 + (i % 2) * 0.08) });
        }
        if (m.far) return;
        // Section stores behind the wall: a linked-ammunition box, a jerry can
        // with its handles, and the radio with its whip up. Every one of them
        // is something a real fire team is carrying and a silhouette is not.
        m.save().move(-1.6, 0, 0.6).rotY(-14);
        m.bar(-0.19, 0.19, 0, 0.26, -0.13, 0.13, shade(P.green, 0.92));
        m.bar(-0.15, 0.15, 0.26, 0.3, -0.1, 0.1, shade(P.green, 1.1));
        m.save().move(0, 0.3, 0).rotZ(90);
        m.slab([[0, 0.07], [0.06, 0.07], [0.06, -0.07], [0, -0.07]], 0.02, 0.006,
          shade(P.metal, 0.6));
        m.restore();
        boltRun(m, { n: 4, r: 0.014, x0: -0.15, x1: 0.15, y0: 0.24, z0: 0.13,
          pitch: 0, col: shade(P.metal, 0.7) });
        m.restore();
        m.save().move(-1.25, 0, 0.72).rotY(8);
        m.bar(-0.09, 0.09, 0, 0.44, -0.15, 0.15, shade(P.olive, 1.0));
        m.bar(-0.06, 0.06, 0.44, 0.5, -0.09, 0.09, shade(P.olive, 0.8));
        m.both((mm) => {
          mm.bar(0.055, 0.075, 0.36, 0.44, -0.11, -0.02, shade(P.olive, 1.2));
        });
        m.restore();
        m.save().move(1.62, 0, 0.55).rotY(22);
        m.bar(-0.13, 0.13, 0, 0.34, -0.09, 0.09, shade(P.sensor, 1.15));
        louvre(m, { n: 4, y0: 0.06, y1: 0.24, z: 0.09, w: 0.16, t: 0.008, d: 0.012,
          col: shade(P.sensor, 1.4) });
        connector(m, { x: -0.09, y: 0.28, z: 0.09, r: 0.022, len: 0.03 });
        whip(m, { x: 0.08, y: 0.34, z: -0.03, r: 0.011, len: 0.95, lean: -7 });
        m.restore();
      },
    },
    inf_mech: {
      name: "Mechanised Infantry Formation", cls: "Infantry", span: 6.7,
      build(m) {
        // NOT A TANK WITH A SMALL TURRET, which is what this used to be. An
        // infantry fighting vehicle is a different machine and the difference
        // is legible: a long flat roof with a ramp under it because the
        // section rides inside, a one- or two-man turret set well back so the
        // troop compartment can be forward of the engine, a cannon rather than
        // a gun, and missiles bolted on the outside of the turret because
        // there is no room for them in it. `armour` is shared with the three
        // Armour entries and may not move, so the hull is lofted here.
        const L = 6.7, W = 2.9, belly = 0.6;
        const sec = (t, hw, y0, y1, y2, k) => ({
          z: t * L,
          pts: [[hw, y0], [hw, y1], [hw * k, y2], [0, y2 + 0.05],
            [-hw * k, y2], [-hw, y1], [-hw, y0]],
        });
        const hull = [
          sec(0, 1.3, belly, 1.28, 1.74, 0.95), sec(0.05, 1.4, belly, 1.3, 1.76, 0.95),
          sec(0.24, 1.45, belly, 1.3, 1.76, 0.95), sec(0.46, 1.45, belly, 1.3, 1.76, 0.95),
          sec(0.62, 1.45, belly, 1.28, 1.72, 0.95), sec(0.74, 1.42, belly, 1.16, 1.5, 0.94),
          sec(0.86, 1.32, belly + 0.06, 1.0, 1.16, 0.92),
          sec(0.94, 1.16, belly + 0.12, 0.86, 0.98, 0.9),
          sec(1, 0.92, belly + 0.2, 0.76, 0.86, 0.88),
        ];
        m.loft(m.far ? [hull[0], hull[3], hull[6], hull[8]] : hull, P.olive);
        // Running gear, built out of links rather than drawn as a band. This is
        // where the triangles go and it is where the eye goes: at 90 px the
        // wheel line and the link line are found before the turret is.
        m.both((mm) => linkedTrack(mm, {
          x: W / 2 * 0.96, w: W * 0.17, r: 0.42, z0: 0.6, z1: L - 0.75,
          t: 0.045, pitch: 0.28, links: 26, wheels: 5,
        }));
        if (!m.far) {
          // Applique armour: the panels a modern IFV is uparmoured with, each
          // standing proud of the flank on its own bolt line. The bolts are
          // the detail — a plate with nothing round its edge is a decal.
          m.both((mm) => {
            for (let i = 0; i < 4; i += 1) {
              const z0 = 0.55 + i * 1.28;
              mm.save().move(W / 2 * 0.99, 0, 0).rotZ(90);
              mm.slab([[1.36, z0], [1.68, z0 + 0.05], [1.68, z0 + 1.1], [1.36, z0 + 1.05]],
                0.09, 0.03, shade(P.olive, i % 2 ? 1.08 : 0.94));
              mm.restore();
              boltRun(mm, { n: 4, r: 0.028, x0: W / 2 * 1.03, y0: 1.4, y1: 1.62,
                z0: z0 + 0.12, z1: z0 + 0.92, pitch: 0, yaw: 90,
                col: shade(P.metal, 0.62) });
            }
            // Skirt plates over the top run, hinged so they can be lifted.
            for (let i = 0; i < 3; i += 1) {
              mm.save().move(W / 2 * 1.02, 0, 0).rotZ(90);
              mm.slab([[0.72, 0.75 + i * 1.6], [1.34, 0.75 + i * 1.6],
                [1.34, 2.25 + i * 1.6], [0.72, 2.25 + i * 1.6]], 0.07, 0.025,
              shade(P.olive, 0.86));
              mm.restore();
            }
          });
          // The trim vane folded back on the glacis, and the driver's three
          // periscopes in the roof beside his hatch.
          m.save().move(0, 1.06, L * 0.9).rotX(-16);
          m.slab([[-1.16, -0.36], [1.16, -0.36], [1.16, 0.36], [-1.16, 0.36]], 0.07, 0.024,
            shade(P.olive, 1.12));
          m.restore();
          m.save().move(-0.62, 1.76, L * 0.76);
          m.slab([[-0.3, -0.3], [0.3, -0.3], [0.3, 0.3], [-0.3, 0.3]], 0.08, 0.026,
            shade(P.olive, 1.14));
          m.restore();
          for (let i = 0; i < 3; i += 1) {
            m.save().move(-0.86 + i * 0.24, 1.8, L * 0.79);
            m.bar(-0.07, 0.07, 0, 0.06, -0.045, 0.045, P.sensor);
            m.restore();
          }
          boltRun(m, { n: 5, r: 0.03, x0: -1.0, x1: 1.0, y0: 1.14, z0: L * 0.9 + 0.3,
            pitch: -60, col: shade(P.metal, 0.66) });
        }
        // The turret, set back over the engine deck. Faceted, low and offset —
        // an IFV turret is a box for a cannon and two sights, not a casting.
        m.save().move(0.16, 1.74, L * 0.36);
        m.loft([
          { z: -0.78, pts: trap(0.6, 0, 0.52, 0.8) },
          { z: -0.5, pts: trap(0.7, 0, 0.62, 0.82) },
          { z: 0.3, pts: trap(0.7, 0, 0.62, 0.82) },
          { z: 0.62, pts: trap(0.56, 0, 0.5, 0.8) },
        ], shade(P.olive, 1.06));
        // The mantlet, the cannon and its thermal sleeve. A 25 mm chain gun is
        // a thin barrel with a fat sleeve on it, which is exactly how a card
        // tells it apart from a 120 mm tank gun.
        m.save().move(0, 0.3, 0.5);
        m.soft((mm) => mm.loft([
          { z: 0, pts: ring(0.24, 0.2, mm.lod(12, 6)) },
          { z: 0.16, pts: ring(0.2, 0.17, mm.lod(12, 6)) },
        ], shade(P.olive, 0.86), false));
        m.save().move(0, 0, 0.16);
        m.soft((mm) => mm.tube(0.062, 0.05, 1.55, mm.lod(12, 6), P.exhaust, false));
        m.restore();
        if (!m.far) {
          [0.3, 0.52, 0.74].forEach((f) => {
            jointBand(m, { z: 0.16 + f * 1.3, r: 0.058, w: 0.07, k: 1.5, seg: 12,
              col: shade(P.exhaust, 1.2) });
          });
          m.save().move(0, 0, 1.66);
          m.soft((mm) => mm.tube(0.052, 0.052, 0.12, 12, shade(P.metal, 0.5), false));
          discCap(m, 0.03, 12, P.black, false);
          m.restore();
        }
        m.restore();
        if (!m.far) {
          // Twin anti-tank missiles in their launch tubes on the turret flank,
          // the commander's sight above the roof, and the smoke dischargers.
          m.save().move(0.78, 0.16, -0.14);
          for (let i = 0; i < 2; i += 1) {
            m.save().move(0, i * 0.3, 0);
            canister(m, { r: 0.13, len: 1.0, col: shade(P.olive, 0.9), hoops: 3 });
            m.restore();
          }
          m.bar(-0.05, 0.05, -0.04, 0.42, -0.5, 0.2, shade(P.olive, 0.8));
          m.restore();
          m.save().move(-0.3, 0.62, 0.12);
          m.bar(-0.16, 0.16, 0, 0.3, -0.16, 0.16, shade(P.sensor, 1.15));
          m.save().move(0, 0.14, 0.16);
          m.soft((mm) => mm.tube(0.1, 0.1, 0.05, 10, shade(P.glass, 1.05), false));
          discCap(m, 0.09, 10, shade(P.glass, 1.2), false);
          m.restore();
          m.restore();
          m.both((mm) => {
            for (let i = 0; i < 4; i += 1) {
              mm.save().move(0.52 + (i % 2) * 0.11, 0.24 + Math.floor(i / 2) * 0.16, 0.36)
                .rotY(24).rotX(-12);
              mm.soft((s) => s.tube(0.045, 0.045, 0.16, s.lod(8, 5), shade(P.black, 1.5), false));
              mm.save().move(0, 0, 0.16);
              discCap(mm, 0.042, mm.lod(8, 5), shade(P.metal, 0.7), false);
              mm.restore();
              mm.restore();
            }
          });
          // Roof hatch and the crew's periscope ring: the turret has two men in
          // it and the hatch is the only thing on a card that says so.
          m.save().move(-0.16, 0.62, -0.2);
          m.slab([[-0.28, -0.28], [0.28, -0.28], [0.28, 0.28], [-0.28, 0.28]], 0.09, 0.03,
            shade(P.olive, 1.16));
          m.restore();
          for (let i = 0; i < 6; i += 1) {
            m.save().move(-0.16, 0.63, -0.2).rotY(i * 60).move(0, 0, 0.34);
            m.bar(-0.06, 0.06, 0, 0.07, -0.04, 0.04, P.sensor);
            m.restore();
          }
        }
        m.restore();
        if (m.far) return;
        // The ramp is the whole point of the vehicle: it is a bus that shoots.
        // Hinged at the bottom, with its own door in it and the grab handles
        // the section pulls itself out by.
        m.save().move(0, 1.15, 0.05);
        m.slab([[-1.12, -0.06], [1.12, -0.06], [1.12, 0.06], [-1.12, 0.06]], 1.1, 0.05,
          shade(P.olive, 0.82));
        m.restore();
        m.save().move(0.42, 1.06, 0.13).rotX(90);
        m.slab([[-0.28, -0.42], [0.28, -0.42], [0.28, 0.42], [-0.28, 0.42]], 0.07, 0.024,
          shade(P.olive, 0.9));
        m.restore();
        m.both((mm) => {
          mm.bar(0.86, 0.96, 0.9, 1.0, 0.12, 0.3, shade(P.metal, 0.7));
          mm.bar(0.86, 0.96, 1.34, 1.44, 0.12, 0.3, shade(P.metal, 0.7));
        });
        boltRun(m, { n: 5, r: 0.03, x0: -1.0, x1: 1.0, y0: 0.66, z0: 0.14,
          pitch: 0, col: shade(P.metal, 0.6) });
        // Engine louvres over the deck, the exhaust outlet on the flank, the
        // stowage basket across the back and the two aerials.
        louvre(m, { n: 6, y0: 1.78, y1: 1.78, z: L * 0.62, w: 1.5, t: 0.035, d: 0.11,
          col: shade(P.olive, 0.78) });
        m.save().move(W / 2 * 0.99, 0, L * 0.66).rotZ(90);
        m.slab([[1.0, -0.1], [1.36, -0.1], [1.36, 0.5], [1.0, 0.5]], 0.08, 0.026, P.exhaust);
        m.restore();
        for (let i = 0; i < 5; i += 1) {
          m.bar(-1.2 + i * 0.6, -1.14 + i * 0.6, 1.76, 2.06, -0.08, 0.02,
            shade(P.metal, 0.62));
        }
        m.bar(-1.24, 1.24, 1.98, 2.04, -0.1, -0.02, shade(P.metal, 0.6));
        m.bar(-1.24, 1.24, 1.76, 1.82, -0.1, -0.02, shade(P.metal, 0.6));
        whip(m, { x: -1.1, y: 1.78, z: 1.1, r: 0.016, len: 1.05, lean: 6 });
        whip(m, { x: 1.1, y: 1.78, z: 0.9, r: 0.014, len: 0.86, lean: -8 });
        m.both((mm) => {
          mm.save().move(0.72, 0.82, L - 0.04);
          mm.soft((s) => s.tube(0.11, 0.11, 0.08, s.lod(10, 5), shade(P.white, 0.94), false));
          discCap(mm, 0.1, mm.lod(10, 5), shade(P.glass, 1.2), false);
          mm.restore();
          mm.bar(0.5, 0.66, 0.62, 0.78, L - 0.12, L + 0.1, shade(P.metal, 0.55));
        });
        // The section that rides in it, dismounted at the ramp. The formation
        // is people plus a vehicle and the card has to price both.
        soldier(m, { x: 2.15, z: 1.0, face: -104, col: shade(P.cloth, 0.95) });
      },
    },
    arm_gen2: {
      name: "Second-Generation Armour", cls: "Armour", span: 9.0,
      build(m) {
        armour(m, {
          len: 6.45, w: 3.27, h: 2.3, col: P.olive, wheels: 5, turretZ: 2.7,
          turret: { w: 2.5, len: 2.6, h: 0.72, round: true, gunR: 0.075, gunLen: 3.9, mg: true },
        });
      },
    },
    arm_gen3: {
      name: "Third-Generation Armour", cls: "Armour", span: 11.0,
      build(m) {
        armour(m, {
          len: 7.9, w: 3.66, h: 2.44, col: P.green, wheels: 7, skirt: true, turretZ: 3.0,
          turret: { w: 2.9, len: 3.6, h: 0.86, gunR: 0.085, gunLen: 5.3, brake: true, mg: true },
        });
      },
    },
    trophy: {
      name: "Trophy Active Protection", cls: "Armour", span: 11.0,
      build(m) {
        armour(m, {
          len: 7.9, w: 3.66, h: 2.44, col: P.tan, wheels: 7, skirt: true, turretZ: 3.0,
          turret: { w: 2.9, len: 3.6, h: 0.86, gunR: 0.085, gunLen: 5.3, brake: true, mg: true, aps: true },
        });
      },
    },

    // ------------------------------------------------------------------- air
    air_gen2: {
      name: "Second-Generation Combat Aircraft", cls: "Air", span: 15,
      build(m) {
        jet(m, {
          len: 15, w: 1.55, h: 1.75, span: 7.15, col: P.greyDark,
          wingZ: 7.0, root: 5.8, tip: 0.25, sweep: 4.7, dihedral: -2,
          stab: { z: 2.9, span: 2.1, root: 1.9, tip: 0.6, sweep: 1.3 },
          fin: { kind: "single", z: 4.4, root: 3.6, tip: 1.1, height: 2.5, sweep: 2.4 },
          nozzles: 1, intake: "nose", canopyZ: 9.4,
          // A wing fence and two rails. Both are 1960s: the fence is how a
          // swept wing of that decade was stopped from stalling at the tip,
          // and the rails carry the two beam-riding missiles it was built for.
          fence: 0.56, foil: 0.055,
          pylons: [[0.34, 0.3, 0.4], [0.62, 0.26, 0.2]],
        });
        // The brake chute fairing at the fin root, which every aircraft of this
        // generation has and nothing since does.
        m.save().move(0, 1.3, 1.2);
        m.slab([[-0.22, -0.9], [0.22, -0.9], [0.22, 0.9], [-0.22, 0.9]], 0.4, 0.12,
          shade(P.greyDark, 0.86));
        m.restore();
      },
    },
    air_gen3: {
      name: "Third-Generation Combat Aircraft", cls: "Air", span: 17.7,
      build(m) {
        jet(m, {
          len: 17.7, w: 2.05, h: 2.2, span: 11.7, col: P.greyDark,
          wingZ: 8.0, root: 5.0, tip: 1.5, sweep: 3.8, dihedral: 6,
          stab: { z: 2.9, span: 2.6, root: 2.2, tip: 0.8, sweep: 1.6, dihedral: -18 },
          fin: { kind: "single", z: 4.0, root: 4.2, tip: 1.3, height: 3.1, sweep: 2.7 },
          nozzles: 2, intake: "side", intakeZ: 9.5, canopyZ: 11.2,
          fence: 0.5, foil: 0.06,
          pylons: [[0.3, 0.36, 0.5], [0.58, 0.3, 0.3]],
        });
        // A ventral strake each side of the nozzles: this generation's answer
        // to the directional stability its own intakes cost it.
        m.both((mm) => {
          mm.save().move(0.5, -0.95, 2.4).rotZ(-24);
          mm.slab([[0, 1.5], [0.9, 0.4], [0.9, -0.5], [0, -1.4]], 0.14, 0.05,
            shade(P.greyDark, 0.9));
          mm.restore();
        });
      },
    },
    air_gen4: {
      name: "Fourth-Generation Combat Aircraft", cls: "Air", span: 19.4,
      build(m) {
        jet(m, {
          len: 19.4, w: 2.7, h: 2.35, span: 13.05, col: P.grey,
          wingZ: 9.0, root: 6.6, tip: 1.4, sweep: 4.1,
          stab: { z: 2.9, span: 3.0, root: 3.3, tip: 1.1, sweep: 2.4 },
          fin: { kind: "twin", x: 1.1, z: 4.6, root: 4.4, tip: 1.6, height: 3.1, sweep: 3.0, cant: 3 },
          nozzles: 2, intake: "side", intakeZ: 10.4, canopyZ: 12.6,
          // The leading-edge root extension is what makes this generation the
          // one that turns, and it is visible from every angle a card uses.
          lerx: 4.2, foil: 0.058,
          pylons: [[0.26, 0.34, 0.6], [0.5, 0.3, 0.3], [0.74, 0.2, 0.1]],
        });
      },
    },
    f15e: {
      name: "F-15E Strike Eagle", cls: "Air", span: 19.4,
      build(m) {
        jet(m, {
          len: 19.4, w: 2.7, h: 2.35, span: 13.05, col: shade(P.greyDark, 0.86),
          wingZ: 9.0, root: 6.6, tip: 1.4, sweep: 4.1,
          stab: { z: 2.9, span: 3.0, root: 3.3, tip: 1.1, sweep: 2.4 },
          fin: { kind: "twin", x: 1.1, z: 4.6, root: 4.4, tip: 1.6, height: 3.1, sweep: 3.0, cant: 3 },
          nozzles: 2, intake: "side", intakeZ: 10.4, canopyZ: 12.4,
          // Two seats, and the bow between them is the difference a card can
          // see between an Eagle and a Strike Eagle from the side.
          twoSeat: true, foil: 0.058,
        });
        // Conformal fuel tanks along the intakes and four bombs under the wing.
        // The E is a two-seat bomb truck and that is the entire difference.
        m.both((mm) => {
          mm.save().move(1.45, 0.18, 9.8);
          // The tank is a lofted fairing, not a box: it wraps the intake
          // shoulder, and the tangential launchers along its lower corner are
          // the shape everybody recognises.
          mm.soft((s) => bodyLoft(s, {
            len: 5.6, w: 0.62, h: 1.35, col: shade(P.greyDark, 0.8), seg: s.lod(12, 6),
            stations: [[0, .3, .3], [.08, .9, .9], [.5, 1, 1], [.86, .92, .94], [1, .4, .5]],
          }));
          mm.move(0, 0, -2.8);
          mm.restore();
          if (!mm.far) {
            mm.save().move(1.45, -0.5, 7.2);
            mm.slab([[-0.24, -1.2], [0.24, -1.2], [0.24, 1.2], [-0.24, 1.2]], 0.3, 0.1,
              shade(P.greyDark, 0.72));
            mm.restore();
          }
          store(mm, 2.6, -0.55, 7.4, 3.4, 0.19);
          store(mm, 3.9, -0.35, 7.0, 3.4, 0.19);
          store(mm, 5.3, -0.2, 7.6, 2.2, 0.14, P.missile);
        });
        // The targeting and navigation pods under the intake ducts. Two pods
        // is what makes the aeroplane a night strike aircraft rather than a
        // fighter with bombs on it.
        m.both((mm) => {
          mm.save().move(0.95, -1.25, 10.6);
          mm.soft((s) => bodyLoft(s, {
            len: 2.6, w: 0.36, h: 0.36, col: P.greyDark, seg: s.lod(12, 6),
            stations: [[0, .5, .5], [.1, 1, 1], [.8, 1, 1], [1, .55, .55]],
          }));
          mm.save().move(0, 0, 2.55);
          mm.soft((s) => s.tube(0.16, 0.14, 0.1, s.lod(10, 5), P.glass, false));
          mm.restore();
          mm.restore();
        });
      },
    },
    f117: {
      name: "F-117 Nighthawk", cls: "Air", span: 20.1,
      build(m) {
        const L = 20.1, half = 6.6;
        // NOT ONE CURVE ANYWHERE, AND THAT IS THE POINT. Every other aircraft
        // in this class now carries smoothed normals on the parts that are
        // genuinely curved; this one carries none, because the Nighthawk has no
        // curved parts. It was faceted so a 1970s computer could predict its
        // radar return, and the facets are what the aeroplane IS. Detail here
        // is added by SUBDIVIDING the faceting, never by softening it.
        facetBody(m, {
          len: L, w: 4.6, h: 3.6, col: P.stealth,
          stations: [[0, .84, .9], [.04, .9, .96], [.08, .95, 1], [.18, .99, 1],
            [.34, 1, 1], [.48, .94, .94], [.62, .82, .82], [.74, .64, .68],
            [.86, .46, .5], [.94, .26, .3], [1, .1, .12]],
        });
        // The planform: sixty-seven degrees of leading-edge sweep, straight
        // from the nose to the tip, and a straight trailing edge back to the
        // tail. Built as a slab so the leading edge carries a real chamfer —
        // on the real aircraft that chamfer is the radar-absorbent edge.
        m.both((mm) => {
          mm.save().move(1.6, -0.35, 0);
          mm.slab([[0, L * 0.96], [half, L * 0.28], [half, L * 0.2], [0, L * 0.06]],
            0.42, 0.14, P.stealth);
          mm.restore();
          // The skin is a mosaic of flat panels with absorbent tape along every
          // seam. At card size those seams are the only texture this renderer
          // will ever have, so they are laid in as geometry.
          mm.save().move(1.7, -0.14, 0);
          facetMosaic(mm, { rows: 4, cols: 7, t: 0.06, col: shade(P.stealth, 1.12),
            corners: [[0.2, L * 0.88], [half * 0.72, L * 0.4], [half * 0.72, L * 0.26],
              [0.2, L * 0.12]] });
          mm.restore();
          mm.save().move(1.7, -0.56, 0);
          facetMosaic(mm, { rows: 3, cols: 6, t: 0.06, col: shade(P.stealth, 0.9),
            corners: [[0.2, L * 0.84], [half * 0.68, L * 0.42], [half * 0.68, L * 0.3],
              [0.2, L * 0.14]] });
          mm.restore();
          // V-tails, canted well out, which is the other half of the shape.
          mm.save().move(1.1, 0.5, L * 0.2).rotZ(90 - 42);
          mm.slab([[0, 2.6], [3.3, 0.6], [3.3, -0.2], [0, -1.0]], 0.24, 0.08,
            shade(P.stealth, 1.12));
          mm.restore();
          // The weapons bay door, serrated on both long edges. A Nighthawk
          // carries everything internally and the doors are the belly.
          mm.save().move(0.55, -1.02, L * 0.46);
          mm.slab([[-0.42, -2.1], [0.42, -2.1], [0.42, 2.1], [-0.42, 2.1]], 0.12, 0.04,
            shade(P.stealth, 0.82));
          mm.restore();
          sawEdge(mm, { x: 1.0, y: -1.0, z0: L * 0.46 - 2.1, len: 4.2, n: 8,
            tooth: 0.3, t: 0.1, col: shade(P.stealth, 0.94) });
          // Two of the four nose booms, mirrored into four.
          boom(mm, { x: 0.55, y: 0.42, z: L * 0.94, r: 0.05, len: 1.5, yaw: 14 });
          boom(mm, { x: 0.5, y: -0.2, z: L * 0.95, r: 0.045, len: 1.3, yaw: 16, pitch: -10 });
        });
        // The faceted canopy, five flat panes, gold-tinted on the real thing
        // because the glass is coated to keep the cockpit from being a corner
        // reflector.
        m.save().move(0, 1.15, L * 0.6);
        facetBody(m, { len: 3.4, w: 2.2, h: 1.4, col: P.sensor,
          stations: [[0, .5, .5], [.2, .8, .8], [.4, 1, 1], [.72, .9, .9], [1, .3, .4]] });
        m.restore();
        // The platypus exhaust: a wide flat slot with a row of vanes across it,
        // which is how the plume is spread thin enough to cool before it leaves
        // the aeroplane. Nothing else in the deck looks like this.
        m.bar(-2.2, 2.2, -0.35, 0.2, -0.3, 0.3, P.black);
        for (let i = 0; i < m.lod(11, 3); i += 1) {
          const n = m.lod(11, 3);
          m.save().move(-2.0 + (4.0 / (n - 1)) * i, -0.08, -0.05);
          m.slab([[-0.05, -0.3], [0.05, -0.3], [0.05, 0.3], [-0.05, 0.3]], 0.42, 0.06,
            shade(P.exhaust, 0.7));
          m.restore();
        }
        m.save().move(0, 0.28, -0.1);
        m.slab([[-2.3, -0.34], [2.3, -0.34], [2.3, 0.34], [-2.3, 0.34]], 0.2, 0.07,
          shade(P.stealth, 1.2));
        m.restore();
        // FLIR above the nose and DLIR below it, both behind a grille of fine
        // bars — a Nighthawk's two apertures are the only holes in it.
        [[0.62, L * 0.86, 1], [-0.72, L * 0.8, -1]].forEach((g) => {
          m.save().move(0, g[0], g[1]);
          m.slab([[-0.44, -0.44], [0.44, -0.44], [0.44, 0.44], [-0.44, 0.44]], 0.16, 0.05,
            P.black);
          for (let i = 0; i < m.lod(6, 2); i += 1) {
            m.save().move(-0.34 + i * 0.136, g[2] * 0.1, 0);
            m.slab([[-0.02, -0.4], [0.02, -0.4], [0.02, 0.4], [-0.02, 0.4]], 0.06, 0.02,
              shade(P.metal, 0.6));
            m.restore();
          }
          m.restore();
        });
        // Panel mosaic over the upper body, and the gear doors.
        m.save().move(0, 1.02, 0);
        facetMosaic(m, { rows: 5, cols: 3, t: 0.05, col: shade(P.stealth, 1.06),
          corners: [[-1.5, L * 0.78], [1.5, L * 0.78], [1.9, L * 0.16], [-1.9, L * 0.16]] });
        m.restore();
      },
    },
    e3: {
      name: "E-3 Sentry AWACS", cls: "Air", span: 44.4,
      build(m) {
        const L = 46.6;
        // A 707 is a smooth aluminium tube and the single biggest thing that
        // was wrong with this model is that it was a prism. Twenty-four
        // segments and a smoothing group, and it is an airliner again.
        bodyLoft(m, {
          len: L, w: 3.8, h: 4.0, col: P.white, seg: m.lod(24, 10), soft: true,
          stations: [[0, .3, .34], [.03, .58, .62], [.06, .82, .86], [.11, .95, .96],
            [.16, 1, 1], [.4, 1, 1], [.62, 1, 1], [.78, 1, 1], [.84, .95, .96],
            [.9, .84, .86], [.94, .7, .74], [.97, .5, .54], [1, .12, .14]],
        });
        m.both((mm) => {
          mm.liftSurface({ x0: 1.7, y: -0.9, z: L * 0.52, span: 20.5, root: 8.4, tip: 2.4,
            sweep: 8.0, thick: 0.115, dihedral: 6, ctrl: 0.24,
            surfaces: [[0.06, 0.42, 6], [0.48, 0.7, 5], [0.74, 0.97, -4]],
          }, shade(P.white, 0.94));
          // Two engines a side, hung ahead of and below the wing, which is what
          // a 707 looks like and nothing else does. Each is a smooth nacelle
          // with a fan face you can see, a pylon and a hot core nozzle.
          [[6.2, L * 0.5], [11.6, L * 0.44]].forEach((e, i) => {
            mm.save().move(e[0], -2.1 - i * 0.15, e[1]);
            mm.slab([[-0.28, -0.4], [0.28, -0.4], [0.28, 2.2], [-0.28, 2.2]], 1.9, 0.22,
              P.greyDark);
            mm.soft((s) => bodyLoft(s, {
              len: 4.6, w: 2.2, h: 2.2, col: shade(P.white, 0.9), seg: s.lod(16, 8),
              stations: [[0, .82, .82], [.06, .96, .96], [.3, 1, 1], [.72, .98, .98],
                [.94, .92, .92], [1, .86, .86]],
            }));
            // Down the intake: the lip, the duct and the spinner. An engine
            // drawn as a closed cylinder is a fuel tank — but a map pin cannot
            // see down an intake, so none of this is built at far range.
            if (mm.far) { mm.restore(); return; }
            mm.save().move(0, 0, 4.6);
            mm.soft((s) => s.loft([{ z: 0, pts: ring(1.1, 1.1, s.lod(16, 8)) },
              { z: -0.34, pts: ring(0.94, 0.94, s.lod(16, 8)) },
              { z: -1.5, pts: ring(0.86, 0.86, s.lod(16, 8)) }], shade(P.black, 1.6), false));
            mm.save().move(0, 0, -1.5);
            mm.fan(ring(0.86, 0.86, mm.lod(16, 8)).map((p) => [p[0], p[1], 0]), P.sensor);
            mm.move(0, 0, 0.1);
            mm.soft((s) => s.tube(0.02, 0.3, 0.8, s.lod(12, 6), shade(P.metal, 0.8), false));
            mm.restore();
            mm.restore();
            mm.save().move(0, 0, -0.1);
            nozzle(mm, { r: 0.62, len: 0.9 });
            mm.restore();
            mm.restore();
          });
          mm.liftSurface({ x0: 1.2, y: 0.6, z: L * 0.1, span: 6.6, root: 4.4, tip: 1.6,
            sweep: 3.2, thick: 0.1, dihedral: 8, ctrl: 0.3,
            surfaces: [[0.06, 0.94, 3]] }, shade(P.white, 0.94));
        });
        // The fin, with a rudder. `foil` roots the chord AT the anchor where
        // the plate this replaced straddled it, so the anchor moves forward by
        // the old leading-edge offset — otherwise the fin trails five metres
        // off the back of the aeroplane and the quoted length is a lie.
        m.save().move(0, 1.9, L * 0.06 + 4.4).rotZ(90 - 2);
        m.liftSurface({ x0: 0, y: 0, z: 0, span: 7.6, root: 6.0, tip: 2.2, sweep: 3.2,
          thick: 0.1, ctrl: 0.28, surfaces: [[0.05, 0.94, 2]] }, shade(P.white, 1.05));
        m.restore();
        // The rotodome. Eleven metres across, on two struts, and the reason the
        // airframe is in the deck at all. Smoothed round its rim, because it is
        // a disc and not a nut.
        m.save().move(0, 4.2, L * 0.33);
        m.both((mm) => {
          mm.save().move(1.5, -1.5, 0).rotZ(-8);
          mm.slab([[-0.26, -1.1], [0.26, -1.1], [0.26, 1.1], [-0.26, 1.1]], 1.7, 0.16,
            P.greyDark);
          mm.restore();
        });
        m.save().rotX(-90);
        m.soft((s) => s.loft([
          { z: -0.5, pts: ring(5.2, 5.2, s.lod(22, 10)) },
          { z: -0.4, pts: ring(5.5, 5.5, s.lod(22, 10)) },
          { z: 0.4, pts: ring(5.5, 5.5, s.lod(22, 10)) },
          { z: 0.5, pts: ring(5.2, 5.2, s.lod(22, 10)) },
        ], P.white, false));
        [-0.5, 0.5].forEach((z) => {
          m.save().move(0, 0, z);
          m.fan(ring(5.2, 5.2, m.lod(22, 10)).map((p) => [p[0], p[1], 0]),
            shade(P.white, z > 0 ? 1.04 : 0.86));
          m.restore();
        });
        m.restore();
        m.save().move(0, 0.16, 0).rotX(-90).scale(1, 1, 0.1);
        m.tube(5.6, 5.6, 0.6, m.lod(22, 10), shade(P.red, 1.1));
        m.restore();
        m.restore();
        if (m.far) return;
        // Cabin windows, doors and the escape hatches: a run of small flush
        // panels down each side, which is how the eye reads the length of an
        // airliner and the one thing a bare tube cannot say.
        [30, -30, 150, -150].forEach((th) => {
          panelRun(m, { rx: 1.9, ry: 2.0, th, z0: L * 0.24, z1: L * 0.8, n: m.lod(9, 3),
            w: 0.34, len: 0.34, depth: 0.06, col: P.sensor });
        });
        [[24, L * 0.78], [156, L * 0.78], [24, L * 0.3], [156, L * 0.3]].forEach((d) => {
          skinPanel(m, { rx: 1.9, ry: 2.0, th: d[0], z: d[1], w: 0.9, len: 1.7,
            depth: 0.07, col: shade(P.white, 0.9) });
        });
        // Gear fairings and the ventral fin, then the aerials.
        m.both((mm) => {
          mm.save().move(1.5, -1.7, L * 0.46);
          mm.soft((s) => bodyLoft(s, {
            len: 5.0, w: 1.5, h: 1.2, col: shade(P.white, 0.92), seg: s.lod(12, 6),
            stations: [[0, .4, .4], [.12, 1, 1], [.7, 1, 1], [1, .5, .6]],
          }));
          mm.restore();
        });
        bladeAerial(m, { y: 2.1, z: L * 0.66, h: 0.5, len: 1.1, col: shade(P.white, 0.86) });
        bladeAerial(m, { y: 2.1, z: L * 0.2, h: 0.4, len: 0.9, col: shade(P.white, 0.86) });
        bladeAerial(m, { y: -2.1, z: L * 0.6, h: 0.36, len: 0.8, roll: 180,
          col: shade(P.white, 0.86) });

        // The flight deck windows, which are where the front is.
        m.save().move(0, 1.5, L * 0.92).rotX(-16);
        m.slab([[-1.1, -0.5], [1.1, -0.5], [0.8, 0.6], [-0.8, 0.6]], 0.24, 0.08, P.glass);
        m.restore();
      },
    },
    b2: {
      name: "B-2 Spirit", cls: "Air", span: 52.4,
      build(m) {
        const L = 21.0, half = 26.2;
        // A FLYING WING, AND IT STAYS ONE. The silhouette is the aeroplane, so
        // nothing here rounds it off: the panels are flat, the edges are
        // chamfered rather than filleted, and the trailing edge keeps its
        // sawtooth. What is added is what the real aircraft has and this model
        // did not — the double-W trailing edge as real teeth, the buried
        // intakes with serrated lips, the exhaust troughs, and a skin that is a
        // mosaic of panels instead of one grey sheet.
        m.both((mm) => {
          mm.slab([[0, L], [half * 0.55, L * 0.44], [half * 0.55, L * 0.1], [0, L * 0.02]],
            1.0, 0.3, P.stealth);
          mm.slab([[half * 0.55, L * 0.44], [half, L * 0.06], [half, -L * 0.02],
            [half * 0.55, L * 0.1]], 0.55, 0.18, shade(P.stealth, 0.94));
          // The trailing edge. Four big teeth a side, and they are the reason a
          // Spirit is recognisable from below at any distance. The tooth base
          // has to sit ON the two panels' trailing edges — those step at
          // half*0.55 and the inner one sweeps FORWARD going outboard, so a
          // straight line through them leaves the teeth hanging in the air.
          const back = (x) => (x <= half * 0.55
            ? L * 0.02 + (x / (half * 0.55)) * L * 0.08
            : L * 0.1 - ((x - half * 0.55) / (half * 0.45)) * L * 0.12);
          for (let i = 0; i < mm.lod(4, 1); i += 1) {
            const n = mm.lod(4, 1);
            const x0 = half * (0.12 + 0.2 * i), x1 = half * (0.12 + 0.2 * (i + 1));
            const xm = (x0 + x1) / 2;
            mm.slab([[x0, back(x0) + L * 0.03], [x1, back(x1) + L * 0.03],
              [xm, back(xm) - L * 0.06]],
            0.7 - i * 0.1, 0.16, shade(P.stealth, 1 + ((i % 2) - 0.5) * 0.08));
            if (n === 1) break;
          }
          // Panel mosaic, upper and lower.
          mm.save().move(0, 0.52, 0);
          facetMosaic(mm, { rows: 6, cols: 4, t: 0.09, col: shade(P.stealth, 1.1),
            corners: [[1.2, L * 0.92], [half * 0.52, L * 0.46], [half * 0.52, L * 0.14],
              [1.2, L * 0.08]] });
          mm.restore();
          mm.save().move(0, -0.52, 0);
          facetMosaic(mm, { rows: 5, cols: 3, t: 0.09, col: shade(P.stealth, 0.88),
            corners: [[1.2, L * 0.88], [half * 0.5, L * 0.44], [half * 0.5, L * 0.16],
              [1.2, L * 0.1]] });
          mm.restore();
          // Buried intakes: a serrated lip, a duct that turns out of sight, and
          // the boundary-layer diverter slot behind it.
          mm.save().move(2.6, 1.1, L * 0.52);
          mm.slab([[-1.2, -1.5], [1.2, -1.5], [1.2, 1.5], [-1.2, 1.5]], 0.72, 0.22,
            shade(P.stealth, 1.2));
          mm.restore();
          mm.save().move(2.6, 1.5, L * 0.52).rotZ(0);
          mm.soft((s) => s.loft([
            { z: 1.4, pts: ringSuper(0.95, 0.42, s.lod(14, 6), 3.0) },
            { z: 0.2, pts: ringSuper(0.86, 0.36, s.lod(14, 6), 3.0) },
            { z: -1.6, pts: ringSuper(0.7, 0.3, s.lod(14, 6), 3.0) },
          ], shade(P.black, 1.5), false));
          mm.save().move(0, 0, -1.6);
          mm.fan(ringSuper(0.7, 0.3, mm.lod(14, 6), 3.0).map((p) => [p[0], p[1], 0]), P.black);
          mm.restore();
          mm.restore();
          sawEdge(mm, { x: 2.6, y: 1.52, z0: L * 0.52 + 0.9, len: 1.9, n: 6, tooth: 0.34,
            t: 0.12, col: shade(P.stealth, 1.26), roll: 90 });
          // The exhaust trough: a shielded slot let into the upper surface,
          // with vanes, so the plume is spread and shaded from below.
          mm.save().move(2.9, 0.5, L * 0.11);
          mm.slab([[-1.5, -1.4], [1.5, -1.4], [1.5, 1.4], [-1.5, 1.4]], 0.24, 0.08, P.black);
          mm.restore();
          for (let i = 0; i < mm.lod(5, 2); i += 1) {
            mm.save().move(1.5 + i * 0.7, 0.58, L * 0.11);
            mm.slab([[-0.06, -1.3], [0.06, -1.3], [0.06, 1.3], [-0.06, 1.3]], 0.3, 0.08,
              shade(P.exhaust, 0.8));
            mm.restore();
          }
          // Main gear door, serrated on its outboard edge.
          mm.save().move(3.4, -0.56, L * 0.42);
          mm.slab([[-1.1, -2.0], [1.1, -2.0], [1.1, 2.0], [-1.1, 2.0]], 0.14, 0.05,
            shade(P.stealth, 0.84));
          mm.restore();
          sawEdge(mm, { x: 4.5, y: -0.56, z0: L * 0.42 - 2.0, len: 4.0, n: 7, tooth: 0.34,
            t: 0.12, col: shade(P.stealth, 0.9) });
        });
        facetBody(m, {
          len: L, w: 9.0, h: 3.4, col: shade(P.stealth, 1.08),
          stations: [[.02, .7, .5], [.08, .86, .8], [.16, 1, 1], [.34, .99, .99],
            [.5, .96, .96], [.64, .82, .84], [.78, .6, .62], [.9, .34, .38], [1, .12, .16]],
        });
        // The cockpit: four flat windscreen panes in a frame, set into the
        // leading-edge hump. Flat, because on this aeroplane even the glass is.
        m.save().move(0, 1.5, L * 0.78);
        m.slab([[-1.5, -1.3], [1.5, -1.3], [1.1, 1.3], [-1.1, 1.3]], 0.56, 0.16,
          shade(P.stealth, 1.24));
        m.restore();
        m.save().move(0, 1.62, L * 0.79).rotX(-22);
        m.slab([[-1.2, -0.9], [1.2, -0.9], [0.86, 0.9], [-0.86, 0.9]], 0.2, 0.07, P.canopy);
        m.restore();
        if (m.far) return;
        m.save().move(0, 1.8, L * 0.5);
        facetMosaic(m, { rows: 4, cols: 3, t: 0.07, col: shade(P.stealth, 1.16),
          corners: [[-2.2, L * 0.3], [2.2, L * 0.3], [3.4, -L * 0.06], [-3.4, -L * 0.06]] });
        m.restore();
        // Nose gear door and the refuelling receptacle, both on the centreline.
        m.save().move(0, -1.2, L * 0.66);
        m.slab([[-0.7, -1.5], [0.7, -1.5], [0.7, 1.5], [-0.7, 1.5]], 0.14, 0.05,
          shade(P.stealth, 0.82));
        m.restore();
        m.save().move(0, 1.86, L * 0.62);
        m.slab([[-0.5, -0.5], [0.5, -0.5], [0.5, 0.5], [-0.5, 0.5]], 0.12, 0.05,
          shade(P.stealth, 0.86));
        m.restore();
      },
    },
    predator: {
      name: "RQ-1 Predator", cls: "Air", span: 14.8,
      build(m) { predatorAirframe(m, false); },
    },
    mq1b: {
      name: "MQ-1B Predator", cls: "Air", span: 16.8,
      build(m) { predatorAirframe(m, true); },
    },
    f22: {
      name: "F-22 Raptor", cls: "Air", span: 13.56,
      build(m) {
        jet(m, {
          len: 18.92, w: 3.0, h: 2.3, span: 13.56, col: P.stealth, facet: true,
          wingZ: 9.4, root: 6.9, tip: 1.3, sweep: 5.6,
          stab: { z: 2.6, span: 2.7, root: 2.9, tip: 0.8, sweep: 2.3 },
          fin: { kind: "twin", x: 1.25, z: 4.0, root: 3.4, tip: 1.1, height: 3.0, sweep: 2.7, cant: 28 },
          nozzles: 2, intake: "side", intakeZ: 10.2, canopyZ: 12.4,
          // Nothing hangs outside: a Raptor's weapons are in three bays and
          // the clean wing is the whole argument for the aeroplane.
          foil: 0.05, gun: true,
        });
        // Chines. The straight line from the radome to the intake lip is the
        // single most recognisable thing about the aircraft after the tails,
        // and it is a hard edge — no smoothing group touches it.
        m.both((mm) => {
          mm.save().move(1.0, 0.1, 0);
          mm.slab([[0, 17.2], [1.5, 11.0], [1.5, 9.6], [0, 12.0]], 0.2, 0.07,
            shade(P.stealth, 1.18));
          mm.restore();
          // The side weapons bay and the main bay doors, every edge serrated.
          mm.save().move(1.35, -0.5, 9.4);
          mm.slab([[-0.3, -1.5], [0.3, -1.5], [0.3, 1.5], [-0.3, 1.5]], 0.18, 0.06,
            shade(P.stealth, 0.86));
          mm.restore();
          mm.save().move(0.72, -1.06, 8.6);
          mm.slab([[-0.62, -2.4], [0.62, -2.4], [0.62, 2.4], [-0.62, 2.4]], 0.14, 0.05,
            shade(P.stealth, 0.82));
          mm.restore();
          sawEdge(mm, { x: 1.36, y: -1.04, z0: 6.2, len: 4.8, n: 9, tooth: 0.26, t: 0.1,
            col: shade(P.stealth, 0.92) });
          sawEdge(mm, { x: 1.5, y: 0.62, z0: 4.4, len: 3.6, n: 7, tooth: 0.24, t: 0.1,
            col: shade(P.stealth, 1.1) });
          // The two-dimensional thrust-vectoring nozzle's shroud: a Raptor's
          // exhaust is a slot between two flat paddles, not a round can.
          mm.save().move(0.66, 0, 0.5);
          mm.slab([[-0.42, -1.3], [0.42, -1.3], [0.42, 1.3], [-0.42, 1.3]], 1.5, 0.24,
            shade(P.stealth, 0.9));
          mm.restore();
        });
      },
    },
    ea18g: {
      name: "EA-18G Growler", cls: "Air", span: 13.62,
      build(m) {
        jet(m, {
          len: 18.31, w: 2.85, h: 2.4, span: 13.62, col: P.grey,
          wingZ: 8.6, root: 5.8, tip: 1.6, sweep: 3.4,
          stab: { z: 2.6, span: 2.9, root: 3.0, tip: 1.0, sweep: 2.1 },
          fin: { kind: "twin", x: 1.2, z: 4.2, root: 3.7, tip: 1.4, height: 2.9, sweep: 2.5, cant: 20 },
          nozzles: 2, intake: "side", intakeZ: 9.8, canopyZ: 12.0,
          twoSeat: true, lerx: 3.4, foil: 0.055, gun: false,
        });
        // Five jamming pods, and they are the aircraft: a Growler is a Super
        // Hornet that carries transmitters where the bombs would be. Each pod
        // is a body with a ram-air turbine on the nose and a radiating aperture
        // at the back, which is what a jammer looks like close up — and none of
        // that survives being drawn as a map pin, so at far range they go.
        if (m.far) return;
        m.both((mm) => {
          [[2.9, 8.4], [4.6, 8.0]].forEach((p) => {
            mm.save().move(p[0], -0.7, p[1]);
            mm.slab([[-0.3, -0.3], [0.3, -0.3], [0.3, 0.3], [-0.3, 0.3]], 0.72, 0.16,
              P.greyDark);
            mm.save().move(0, -0.62, -1.9);
            mm.soft((s) => bodyLoft(s, {
              len: 3.8, w: 0.56, h: 0.56, col: P.white, seg: s.lod(14, 6),
              stations: [[0, .5, .5], [.06, .92, .92], [.3, 1, 1], [.78, 1, 1],
                [.92, .86, .86], [1, .5, .5]],
            }));
            mm.save().move(0, 0, 3.8);
            mm.soft((s) => s.tube(0.26, 0.1, 0.6, s.lod(10, 5), shade(P.white, 0.9), false));
            // The turbine that powers it, spinning in the airstream.
            for (let i = 0; i < mm.lod(4, 2); i += 1) {
              mm.save().move(0, 0, 0.32).rotZ(i * (360 / mm.lod(4, 2)) + 18);
              mm.slab([[0.03, 0.05], [0.24, 0.09], [0.24, -0.09], [0.03, -0.05]], 0.03, 0.01,
                P.black);
              mm.restore();
            }
            mm.restore();
            mm.save().move(0, -0.1, -0.02);
            mm.soft((s) => s.tube(0.24, 0.16, -0.5, s.lod(10, 5), P.sensor, false));
            mm.restore();
            mm.restore();
            mm.restore();
          });
          // The wingtip receiver pods, which is where a Growler's ears are.
          // Anchored on the TIP CHORD — the strake this replaced sat at z 12.6,
          // which is fine for a strake on the forebody and puts a pod three
          // metres in front of the wing.
          mm.save().move(6.4, 0.15, 2.9);
          mm.soft((s) => bodyLoft(s, {
            len: 2.4, w: 0.34, h: 0.34, col: shade(P.grey, 1.06), seg: s.lod(12, 6),
            stations: [[0, .6, .6], [.1, 1, 1], [.8, 1, 1], [1, .55, .55]],
          }));
          mm.restore();
        });
        m.save().move(0, -0.9, 7.1);
        m.slab([[-0.32, -0.4], [0.32, -0.4], [0.32, 0.4], [-0.32, 0.4]], 0.6, 0.14, P.greyDark);
        m.save().move(0, -0.6, -1.9);
        m.soft((s) => bodyLoft(s, {
          len: 3.8, w: 0.56, h: 0.56, col: P.white, seg: s.lod(14, 6),
          stations: [[0, .5, .5], [.06, .92, .92], [.3, 1, 1], [.78, 1, 1],
            [.92, .86, .86], [1, .5, .5]],
        }));
        m.restore();
        m.restore();
      },
    },
    rq170: {
      name: "RQ-170 Sentinel", cls: "Air", span: 20.0,
      build(m) {
        const L = 4.5, half = 10.0;
        // A FLYING WING, LIKE THE SPIRIT AND FOR THE SAME REASON. Nothing here
        // is rounded; the detail is added as more flat panels, not fewer edges.
        m.both((mm) => {
          mm.slab([[0, L], [half * 0.5, L * 0.5], [half * 0.5, L * 0.06], [0, -L * 0.12]],
            0.42, 0.12, P.stealthLit);
          mm.slab([[half * 0.5, L * 0.5], [half, L * 0.12], [half, -L * 0.04],
            [half * 0.5, L * 0.06]], 0.26, 0.08, shade(P.stealthLit, 0.94));
          mm.save().move(0, 0.23, 0);
          facetMosaic(mm, { rows: 6, cols: 4, t: 0.04, col: shade(P.stealthLit, 1.1),
            corners: [[0.8, L * 0.9], [half * 0.46, L * 0.53], [half * 0.46, L * 0.1],
              [0.8, -L * 0.06]] });
          mm.restore();
          mm.save().move(0, -0.23, 0);
          facetMosaic(mm, { rows: 5, cols: 3, t: 0.04, col: shade(P.stealthLit, 0.9),
            corners: [[0.9, L * 0.86], [half * 0.44, L * 0.52], [half * 0.44, L * 0.14],
              [0.9, -L * 0.02]] });
          mm.restore();
          // The outer panel is skin too, and skin on this aircraft means
          // mosaic: the seam pattern is the only thing about a Sentinel that
          // has ever been photographed clearly.
          mm.save().move(0, 0.15, 0);
          facetMosaic(mm, { rows: 4, cols: 3, t: 0.035, col: shade(P.stealthLit, 1.04),
            corners: [[half * 0.54, L * 0.47], [half * 0.94, L * 0.16],
              [half * 0.94, -L * 0.02], [half * 0.54, L * 0.07]] });
          mm.restore();
          mm.save().move(0, -0.15, 0);
          facetMosaic(mm, { rows: 3, cols: 2, t: 0.035, col: shade(P.stealthLit, 0.86),
            corners: [[half * 0.58, L * 0.44], [half * 0.92, L * 0.17],
              [half * 0.92, L * 0.0], [half * 0.58, L * 0.09]] });
          mm.restore();
          // Elevons on the trailing edge, each on its own gap, and the two
          // sensor fairings that make this airframe worth the money.
          mm.save().move(0, 0, -L * 0.02);
          mm.slab([[half * 0.16, 0], [half * 0.46, -L * 0.02], [half * 0.46, -L * 0.14],
            [half * 0.16, -L * 0.12]], 0.16, 0.05, shade(P.stealthLit, 0.86));
          mm.slab([[half * 0.52, -L * 0.02], [half * 0.86, -L * 0.06], [half * 0.86, -L * 0.16],
            [half * 0.52, -L * 0.14]], 0.12, 0.04, shade(P.stealthLit, 0.82));
          mm.restore();
          mm.save().move(2.6, -0.2, L * 0.42);
          mm.slab([[-0.6, -0.9], [0.6, -0.9], [0.6, 0.9], [-0.6, 0.9]], 0.24, 0.08,
            shade(P.stealthLit, 0.86));
          mm.restore();
          sawEdge(mm, { x: 1.2, y: -0.22, z0: L * 0.2, len: 1.6, n: 5, tooth: 0.18, t: 0.07,
            col: shade(P.stealthLit, 0.92) });
        });
        facetBody(m, {
          len: L, w: 3.0, h: 1.3, col: P.stealthLit,
          stations: [[0, .68, .6], [.1, .84, .82], [.2, 1, 1], [.42, .98, .98],
            [.62, .9, .92], [.82, .62, .68], [1, .24, .3]],
        });
        // The dorsal inlet and the SATCOM blister behind it, which together are
        // the only things standing above the wing.
        m.save().move(0, 0.5, L * 0.5);
        m.slab([[-0.82, -0.92], [0.82, -0.92], [0.82, 0.92], [-0.82, 0.92]], 0.36, 0.11,
          shade(P.stealthLit, 1.15));
        m.restore();
        m.save().move(0, 0.68, L * 0.62).rotZ(0);
        m.soft((s) => s.loft([
          { z: 0.9, pts: ringSuper(0.56, 0.2, s.lod(14, 6), 2.8) },
          { z: 0.1, pts: ringSuper(0.5, 0.17, s.lod(14, 6), 2.8) },
          { z: -0.8, pts: ringSuper(0.4, 0.14, s.lod(14, 6), 2.8) },
        ], shade(P.black, 1.5), false));
        m.save().move(0, 0, -0.8);
        m.fan(ringSuper(0.4, 0.14, m.lod(14, 6), 2.8).map((p) => [p[0], p[1], 0]), P.black);
        m.restore();
        m.restore();
        m.save().move(0, 0.6, L * 0.2).scale(1, 0.5, 1.3);
        m.soft((s) => s.ball(0.5, s.lod(14, 6), s.lod(7, 3), shade(P.stealthLit, 1.2)));
        m.restore();
      },
    },
    f35a: {
      name: "F-35A Lightning II", cls: "Air", span: 10.7,
      build(m) {
        jet(m, {
          len: 15.7, w: 2.9, h: 2.4, span: 10.7, col: P.stealth, facet: true,
          wingZ: 7.6, root: 5.4, tip: 1.3, sweep: 4.2,
          stab: { z: 2.2, span: 2.3, root: 2.5, tip: 0.8, sweep: 1.9 },
          fin: { kind: "twin", x: 1.1, z: 3.6, root: 3.1, tip: 1.0, height: 2.6, sweep: 2.2, cant: 25 },
          nozzles: 1, intake: "dsi", intakeZ: 8.6, canopyZ: 10.4,
          foil: 0.052,
        });
        // The diverterless bump. It is a lump on the side of an intake and it
        // is also the reason this airframe cost what it cost: no splitter
        // plate, no diverter slot, and the bump does the work instead.
        m.both((mm) => {
          mm.save().move(1.45, -0.25, 9.6).scale(0.7, 0.9, 1.4);
          mm.soft((s) => s.ball(0.62, s.lod(14, 8), s.lod(8, 5), shade(P.stealth, 1.2)));
          mm.restore();
          // Weapons bay doors, serrated, and the DAS apertures — six flat
          // windows let into the skin, which on this aircraft do the job a
          // radar warning receiver used to.
          mm.save().move(0.66, -0.98, 7.4);
          mm.slab([[-0.56, -1.9], [0.56, -1.9], [0.56, 1.9], [-0.56, 1.9]], 0.14, 0.05,
            shade(P.stealth, 0.84));
          mm.restore();
          sawEdge(mm, { x: 1.24, y: -0.96, z0: 5.5, len: 3.8, n: 8, tooth: 0.22, t: 0.09,
            col: shade(P.stealth, 0.92) });
          [[1.0, 0.55, 11.6], [1.05, -0.7, 11.0], [0.9, 0.3, 4.2]].forEach((d) => {
            mm.save().move(d[0], d[1], d[2]).rotZ(24);
            mm.slab([[-0.2, -0.24], [0.2, -0.24], [0.2, 0.24], [-0.2, 0.24]], 0.1, 0.035,
              shade(P.glass, 0.8));
            mm.restore();
          });
        });
        // The EOTS window under the nose: a faceted sapphire prism, and the one
        // asymmetric thing on the aeroplane.
        m.save().move(0, -1.05, 12.4).rotX(12);
        m.slab([[-0.42, -0.9], [0.42, -0.9], [0.3, 0.7], [-0.3, 0.7]], 0.34, 0.1,
          shade(P.glass, 0.9));
        m.restore();
        // The gun fairing on the port shoulder, which is the A model's tell.
        m.save().move(-0.95, 0.55, 10.2);
        m.slab([[-0.24, -0.8], [0.24, -0.8], [0.24, 0.8], [-0.24, 0.8]], 0.3, 0.1,
          shade(P.stealth, 1.14));
        m.restore();
        m.save().move(0, -1.05, 8.0);
        m.slab([[-0.5, -1.1], [0.5, -1.1], [0.5, 1.1], [-0.5, 1.1]], 0.35, 0.1, P.black);
        m.restore();
      },
    },
    cca: {
      name: "Collaborative Combat Aircraft", cls: "Air", span: 8.0,
      build(m) {
        jet(m, {
          len: 9.0, w: 1.6, h: 1.35, span: 8.0, col: P.stealthLit, facet: true,
          wingZ: 4.6, root: 3.6, tip: 0.9, sweep: 2.9,
          fin: { kind: "twin", x: 0.62, z: 1.9, root: 1.7, tip: 0.5, height: 1.2, sweep: 1.2, cant: 40 },
          nozzles: 1, intake: "none", canopy: false, foil: 0.055, gun: false,
        });
        // No cockpit, a dorsal inlet, and a size that says this one is bought
        // by the dozen. All three are the point of the programme.
        m.save().move(0, 0.6, 5.0);
        m.slab([[-0.52, -0.92], [0.52, -0.92], [0.52, 0.92], [-0.52, 0.92]], 0.44, 0.13,
          shade(P.stealthLit, 1.16));
        m.restore();
        m.save().move(0, 0.78, 5.1);
        m.soft((s) => s.loft([
          { z: 0.9, pts: ringSuper(0.4, 0.16, s.lod(14, 6), 2.6) },
          { z: 0.1, pts: ringSuper(0.36, 0.14, s.lod(14, 6), 2.6) },
          { z: -0.9, pts: ringSuper(0.28, 0.12, s.lod(14, 6), 2.6) },
        ], shade(P.black, 1.5), false));
        m.save().move(0, 0, -0.9);
        m.fan(ringSuper(0.28, 0.12, m.lod(14, 6), 2.6).map((p) => [p[0], p[1], 0]), P.black);
        m.restore();
        m.restore();
        sawEdge(m, { x: 0, y: 0.82, z0: 5.6, len: 0.9, n: 5, tooth: 0.16, t: 0.08,
          col: shade(P.stealthLit, 1.24), roll: 90 });
        // A weapons bay, because an attritable aircraft that carries nothing is
        // a target drone, and the two of them are priced very differently.
        m.both((mm) => {
          mm.save().move(0.38, -0.56, 4.4);
          mm.slab([[-0.32, -1.2], [0.32, -1.2], [0.32, 1.2], [-0.32, 1.2]], 0.1, 0.035,
            shade(P.stealthLit, 0.84));
          mm.restore();
          sawEdge(mm, { x: 0.72, y: -0.55, z0: 3.2, len: 2.4, n: 6, tooth: 0.16, t: 0.07,
            col: shade(P.stealthLit, 0.92) });
          mm.save().move(0.5, 0.34, 6.4);
          mm.slab([[-0.16, -0.5], [0.16, -0.5], [0.16, 0.5], [-0.16, 0.5]], 0.08, 0.03,
            shade(P.glass, 0.85));
          mm.restore();
        });
        // Panelled wings and a panelled deck. An attritable airframe is built
        // out of flat panels because flat panels are what is cheap, and the
        // seams between them are the only surface detail it has.
        m.both((mm) => {
          mm.save().move(0, 0.1, 0);
          facetMosaic(mm, { rows: 4, cols: 3, t: 0.035, col: shade(P.stealthLit, 1.04),
            corners: [[0.85, 4.45], [3.75, 1.85], [3.75, 1.0], [0.85, 1.2]] });
          mm.restore();
          mm.save().move(0, -0.1, 0);
          facetMosaic(mm, { rows: 3, cols: 2, t: 0.035, col: shade(P.stealthLit, 0.88),
            corners: [[0.9, 4.35], [3.65, 1.83], [3.65, 1.1], [0.9, 1.3]] });
          mm.restore();
        });
        m.save().move(0, 0.42, 0);
        facetMosaic(m, { rows: 5, cols: 3, t: 0.04, col: shade(P.stealthLit, 1.08),
          corners: [[-0.6, 4.2], [0.6, 4.2], [0.72, 1.2], [-0.72, 1.2]] });
        m.restore();
      },
    },
    sixthgen: {
      name: "Sixth-Generation Fighter", cls: "Air", span: 17.0,
      build(m) {
        jet(m, {
          len: 22.0, w: 3.6, h: 2.7, span: 17.0, col: shade(P.stealth, 0.88), facet: true,
          wingZ: 10.8, root: 9.6, tip: 1.7, sweep: 7.6,
          nozzles: 2, intake: "dsi", intakeZ: 11.6, canopyZ: 14.6,
          fin: { kind: "none" }, foil: 0.05, wingCtrl: 0.24, gun: false,
        });
        // Tailless, so the wing's trailing edge does the work — and a deep
        // body, because range is the requirement this generation is actually
        // built to. The chine runs the whole length for the same reason.
        m.both((mm) => {
          mm.save().move(1.2, 0.2, 0);
          mm.slab([[0, 20.0], [2.0, 12.6], [2.0, 11.0], [0, 13.8]], 0.24, 0.08,
            shade(P.stealth, 1.2));
          mm.restore();
          // Two deep weapons bays and a serrated door line down each side: a
          // fighter with this much internal volume is carrying it internally.
          mm.save().move(0.9, -1.24, 10.0);
          mm.slab([[-0.72, -3.4], [0.72, -3.4], [0.72, 3.4], [-0.72, 3.4]], 0.16, 0.06,
            shade(P.stealth, 0.8));
          mm.restore();
          sawEdge(mm, { x: 1.66, y: -1.2, z0: 6.4, len: 7.0, n: 10, tooth: 0.3, t: 0.11,
            col: shade(P.stealth, 0.9) });
          sawEdge(mm, { x: 1.7, y: 0.9, z0: 4.0, len: 5.0, n: 8, tooth: 0.28, t: 0.11,
            col: shade(P.stealth, 1.1) });
          // The flat exhaust shroud, as on the Raptor: the plume leaves through
          // a slot between two paddles and never shows the turbine.
          mm.save().move(0.8, 0, 0.6);
          mm.slab([[-0.5, -1.6], [0.5, -1.6], [0.5, 1.6], [-0.5, 1.6]], 1.8, 0.3,
            shade(P.stealth, 0.92));
          mm.restore();
        });
        // The dorsal spine: fuel and cooling, and on this generation the reason
        // the aeroplane is as deep as it is.
        m.save().move(0, 1.32, 0);
        facetMosaic(m, { rows: 4, cols: 2, t: 0.06, col: shade(P.stealth, 1.02),
          corners: [[-1.3, 12.6], [1.3, 12.6], [1.6, 2.6], [-1.6, 2.6]] });
        m.restore();
      },
    },
    aesa: {
      name: "AESA Radar Refit", cls: "Air", span: 1.4,
      build(m) {
        // NOT A PLATFORM: the thing that gets bolted into one, and the model is
        // therefore the hardware itself rather than an aeroplane with a radar
        // in it. What makes an active array active is that every element is its
        // own transmitter, so the face is drawn as elements — a lattice of
        // radiators behind the radome line — and not as a painted disc. The
        // liquid cold plate and its manifold sit behind, because cooling is the
        // engineering problem this refit actually buys.
        const R = 0.62;
        m.bar(-0.62, 0.62, 0, 0.12, -0.5, 0.5, P.greyDark);
        m.save().move(0, 0.12, 0);
        m.bar(-0.1, 0.1, 0, 0.5, -0.08, 0.08, P.metal);
        m.restore();
        m.save().move(0, 0.78, 0);
        arrayFace(m, { r: R, tilt: -74, col: P.sensor });
        // The elements. A triangular lattice inside the aperture, which is how
        // they are actually packed at half a wavelength; every one that falls
        // outside the circle is simply not emitted, so the pattern is round
        // without a mask.
        m.save().rotX(74).move(0, R * 0.1 + 0.03, 0);
        const step = R * 0.128;
        for (let r = -8; r <= 8; r += 1) {
          for (let c = -9; c <= 9; c += 1) {
            const x = (c + (r % 2 ? 0.5 : 0)) * step, z = r * step * 0.87;
            if (Math.hypot(x, z) > R * 0.82) continue;
            // At map range the lattice thins to every third element rather
            // than vanishing: an array face with no elements is a dish.
            if (m.far && (r % 3 || c % 3)) continue;
            m.slab([[x - step * 0.36, z - step * 0.3], [x + step * 0.36, z - step * 0.3],
              [x + step * 0.36, z + step * 0.3], [x - step * 0.36, z + step * 0.3]],
            0.022, 0.006, shade(P.glass, 1.05 + (((r + c) % 3) - 1) * 0.06));
          }
        }
        m.restore();
        m.restore();
        // The cold plate behind the array, its coolant manifold, and the two
        // bundles of waveguide that leave it.
        m.save().move(0, 0.62, -0.34).rotX(18);
        m.bar(-0.44, 0.44, -0.3, 0.3, -0.34, 0, shade(P.greyDark, 0.9));
        for (let i = 0; i < m.lod(11, 3); i += 1) {
          const n = m.lod(11, 3);
          m.save().move(-0.4 + (0.8 / (n - 1)) * i, 0, -0.36);
          m.slab([[-0.018, -0.28], [0.018, -0.28], [0.018, 0.28], [-0.018, 0.28]], 0.16, 0.04,
            shade(P.metal, 0.8));
          m.restore();
        }
        m.restore();
        if (m.far) return;
        m.save().move(0, 0.5, -0.5).rotX(24);
        m.soft((s) => s.tube(0.09, 0.09, 0.4, s.lod(12, 6), shade(P.metal, 0.7), false));
        m.restore();
        m.save().move(0.22, 0.42, -0.46).rotX(30);
        m.soft((s) => s.tube(0.06, 0.06, 0.34, s.lod(12, 6), shade(P.red, 0.9), false));
        m.restore();
        m.save().move(-0.22, 0.42, -0.46).rotX(30);
        m.soft((s) => s.tube(0.06, 0.06, 0.34, s.lod(12, 6), shade(P.glass, 0.8), false));
        m.restore();
        // The bulkhead ring it bolts to, and the bolts. A refit is a thing with
        // a mounting interface; without one it is a prop.
        m.save().move(0, 0.78, -0.02).rotX(74);
        for (let i = 0; i < m.lod(16, 4); i += 1) {
          const n = m.lod(16, 4), a = (i / n) * Math.PI * 2;
          m.save().move(Math.cos(a) * R * 0.95, 0, Math.sin(a) * R * 0.95);
          m.slab([[-0.026, -0.026], [0.026, -0.026], [0.026, 0.026], [-0.026, 0.026]],
            0.05, 0.014, shade(P.metal, 0.9));
          m.restore();
        }
        m.restore();
      },
    },
  };

  /// The Predator airframe, twice: unarmed as the RQ-1 and with two Hellfires
  /// as the MQ-1B. They are the same aircraft and the deck prices them apart
  /// only because of what hangs under the wing, so the models differ only there.
  ///
  /// Everything on this aeroplane is curved and almost nothing on it is
  /// straight — it is a composite airframe laid up in a mould, not a machined
  /// one — so the fuselage, the SATCOM fairing, the sensor ball and the
  /// spinner all carry smoothed normals. It is also always photographed on the
  /// ground, which is why the undercarriage is modelled: a Predator floating
  /// with its wheels retracted is a picture nobody has ever taken.
  function predatorAirframe(m, armed) {
    const L = 8.22, col = shade(P.white, 0.92);
    bodyLoft(m, {
      len: L, w: 0.9, h: 1.0, col, seg: m.lod(22, 9), soft: true,
      stations: [[0, .3, .3], [.05, .46, .48], [.1, .62, .66], [.2, .72, .76],
        [.34, .8, .84], [.48, .86, .92], [.62, .92, 1.0], [.74, .97, 1.1],
        [.86, 1, 1.15], [.94, .84, 1.02], [1, .5, .72]],
    });
    m.both((mm) => {
      // A long straight wing at a Reynolds number nothing else here flies at.
      // The aileron is most of the outboard trailing edge, because roll
      // authority on a wing this slender has to come from somewhere.
      mm.liftSurface({
        x0: 0.4, y: 0.35, z: L * 0.52, span: 6.9, root: 0.9, tip: 0.55, sweep: 0.12,
        thick: 0.14, dihedral: 2, ctrl: 0.26,
        surfaces: [[0.06, 0.46, 8], [0.5, 0.94, -4]], n: mm.lod(13, 4), bays: mm.lod(9, 1),
      }, col);
      // The inverted V-tail, which is the silhouette everybody knows. Anchored
      // at the leading edge, which is 1.5 m ahead of where the old plate's
      // straddling outline put it — same tail, same place, quoted honestly.
      mm.save().move(0.28, 0.1, L * 0.1 + 1.5).rotZ(-(90 - 43));
      mm.liftSurface({ x0: 0, y: 0, z: 0, span: 2.1, root: 1.6, tip: 0.55, sweep: 1.0,
        thick: 0.12, ctrl: 0.3, surfaces: [[0.08, 0.9, 3]],
        n: mm.lod(11, 3), bays: mm.lod(7, 1) }, col);
      mm.restore();
      if (!mm.far) {
        // Pitot, wingtip light and the aerial the aircraft is flown through.
        boom(mm, { x: 7.2, y: 0.34, z: L * 0.56, r: 0.02, len: 0.42, yaw: 0 });
        mm.save().move(2.4, 0.24, L * 0.42);
        mm.slab([[-0.1, -0.34], [0.1, -0.34], [0.1, 0.34], [-0.1, 0.34]], 0.14, 0.04,
          shade(P.white, 0.84));
        mm.restore();
      }
      if (armed) {
        // The rail, the launcher and the missile on it. On the MQ-1B this is
        // the entire difference from the RQ-1 and the entire reason the kit
        // costs what it costs.
        mm.save().move(1.9, 0.24, L * 0.5);
        mm.slab([[-0.06, -0.2], [0.06, -0.2], [0.06, 0.2], [-0.06, 0.2]], 0.3, 0.08,
          P.greyDark);
        mm.save().move(0, -0.3, 0);
        mm.slab([[-0.07, -0.5], [0.07, -0.5], [0.07, 0.5], [-0.07, 0.5]], 0.12, 0.04,
          shade(P.greyDark, 1.1));
        mm.restore();
        mm.save().move(0, -0.42, -0.8);
        missile(mm, { len: 1.63, r: 0.09, col: P.warhead, seeker: true, nose: 0.3,
          seg: mm.lod(12, 6), soft: true,
          fins: [{ n: 4, z: 0.06, span: 0.16, chord: 0.3, tip: 0.16, roll: 45 },
            { n: 4, z: 0.6, span: 0.14, chord: 0.22, tip: 0.14 }] });
        mm.restore();
        mm.restore();
      }
    });
    // Pusher prop and the SATCOM hump, in that order of importance. The blades
    // are aerofoils with real twist, because a propeller drawn as two flat
    // paddles is the one part of this aircraft that is always moving.
    m.save().move(0, 0.16, -0.05);
    m.soft((s) => s.tube(0.09, 0.05, 0.22, s.lod(12, 6), P.black, false));
    for (let i = 0; i < 2; i += 1) {
      m.save().rotZ(i * 180 + 18);
      m.foil({ x0: 0.05, y: 0, z: 0.04, span: 0.81, root: 0.15, tip: 0.09, sweep: 0.03,
        thick: 0.14, incidence: 24, n: m.lod(8, 3), bays: m.lod(5, 1) }, P.black);
      m.restore();
    }
    m.restore();
    m.save().move(0, 0.52, L * 0.86).scale(1, 0.72, 1.2);
    m.soft((s) => s.ball(0.42, s.lod(16, 8), s.lod(8, 4), shade(P.white, 1.0)));
    m.restore();
    // The sensor ball: the aperture plate and two windows, which is where the
    // aeroplane's whole job happens.
    m.save().move(0, -0.32, L * 0.78).scale(1, 0.9, 1);
    m.soft((s) => s.ball(0.28, s.lod(16, 8), s.lod(8, 4), P.sensor));
    m.restore();
    if (!m.far) {
      m.save().move(0, -0.34, L * 0.78 + 0.26).rotX(6);
      m.slab([[-0.16, -0.16], [0.16, -0.16], [0.16, 0.16], [-0.16, 0.16]], 0.06, 0.02,
        shade(P.glass, 0.9));
      m.restore();
      m.save().move(0.1, -0.46, L * 0.78 + 0.2);
      m.slab([[-0.07, -0.07], [0.07, -0.07], [0.07, 0.07], [-0.07, 0.07]], 0.05, 0.016,
        shade(P.glass, 1.2));
      m.restore();
      // Undercarriage: a nose leg and two mains, each a smooth oleo with a fork
      // and a wheel. Three legs is about two hundred triangles and it is what
      // stops the model reading as a plastic toy hanging on a wire.
      [[0, L * 0.78, 0.62, 0.11], [0.62, L * 0.42, 0.72, 0.13], [-0.62, L * 0.42, 0.72, 0.13]]
        .forEach((g) => {
          m.save().move(g[0], -0.3, g[1]);
          m.soft((s) => s.save().rotX(90).tube(0.035, 0.03, g[2], s.lod(8, 4),
            shade(P.metal, 0.7), false).restore());
          m.save().move(0, -g[2], 0).rotY(90);
          m.soft((s) => s.tube(g[3], g[3], 0.08, s.lod(12, 6), P.rubber, false));
          m.save().move(0, 0, 0.04);
          m.soft((s) => s.tube(g[3] * 0.45, g[3] * 0.45, 0.03, s.lod(10, 5),
            shade(P.metal, 0.8), false));
          m.restore();
          m.restore();
          m.restore();
        });
      // The engine's cooling intake and exhaust, and the fuselage panel line
      // that says where the avionics bay opens.
      m.save().move(0, -0.42, L * 0.16);
      m.slab([[-0.16, -0.3], [0.16, -0.3], [0.16, 0.3], [-0.16, 0.3]], 0.14, 0.05,
        shade(P.sensor, 1.2));
      m.restore();
      panelRun(m, { rx: 0.42, ry: 0.5, th: 0, z0: L * 0.3, z1: L * 0.72, n: 4,
        w: 0.2, len: 0.3, depth: 0.03, col: shade(col, 0.94) });
      panelRun(m, { rx: 0.42, ry: 0.5, th: 180, z0: L * 0.3, z1: L * 0.72, n: 4,
        w: 0.2, len: 0.3, depth: 0.03, col: shade(col, 0.94) });
      panelRun(m, { rx: 0.42, ry: 0.5, th: 90, z0: L * 0.2, z1: L * 0.46, n: 3,
        w: 0.22, len: 0.26, depth: 0.03, col: shade(col, 1.04) });
      bladeAerial(m, { y: 0.5, z: L * 0.3, h: 0.1, len: 0.24, col: shade(P.white, 0.8) });
      bladeAerial(m, { y: -0.44, z: L * 0.62, h: 0.08, len: 0.2, roll: 180,
        col: shade(P.white, 0.8) });
    }
  }
  /// A bolt circle. A ring of fasteners round a bearing race, a hatch or an
  /// access panel is the cheapest honest engineering detail there is — a dozen
  /// short cylinders, twenty triangles each, and a panel stops looking printed
  /// on and starts looking bolted down.
  function boltRing(m, o) {
    const n = m.lod(o.n || 12, 4), R = o.r, r = o.br;
    const col = o.col || shade(P.metal, 0.82);
    for (let i = 0; i < n; i += 1) {
      const a = (i / n) * Math.PI * 2 + (o.phase || 0);
      m.save().move((o.x || 0) + Math.cos(a) * R, o.y || 0, (o.z || 0) + Math.sin(a) * R)
        .rotX(-90);
      drum(m, { r, r1: r * 0.84, len: o.h, seg: m.lod(6, 4), col });
      m.restore();
    }
  }
  /// A line of them down the two long edges of a panel, which is how a hatch
  /// cover on a weather deck is actually held on.
  function boltRow(m, o) {
    const n = m.lod(o.n || 6, 2), col = o.col || shade(P.metal, 0.82);
    for (let i = 0; i < n; i += 1) {
      const t = n === 1 ? 0.5 : i / (n - 1);
      for (let s = -1; s <= 1; s += 2) {
        m.save().move(o.x + s * o.w, o.y, o.z0 + (o.z1 - o.z0) * t).rotX(-90);
        drum(m, { r: o.br, r1: o.br * 0.84, len: o.h, seg: m.lod(6, 4), col });
        m.restore();
      }
    }
  }

  /// A parked aircraft on a carrier deck. A `jet` call is five thousand
  /// triangles and there are eight of these up there, so this is the same
  /// picture at a fortieth of the price: a smoothed fuselage, wings folded the
  /// way a deck-parked aircraft's are, canted fins and a dark canopy. It is
  /// still a real aeroplane and not a paper dart, which is what the flight deck
  /// needs it to be — eight of them ARE the reason the carrier exists.
  function deckPlane(m, x, z, face, col) {
    m.save().move(x, 1.0, z).rotY(face).move(0, 0, -8.4);
    bodyLoft(m, { len: 16.8, w: 2.5, h: 2.1, col, seg: m.lod(9, 5), soft: true,
      stations: m.far
        ? [[0, .34, .38], [.42, 1, 1], [.88, .52, .56], [1, .14, .16]]
        : [[0, .34, .38], [.09, .92, .96], [.42, 1, 1], [.66, .94, .88],
          [.88, .52, .56], [1, .14, .16]] });
    m.both((mm) => {
      // The wing, folded up outboard of the crank, because that is how one
      // stands on a deck and it is the only aircraft pose a carrier ever shows.
      mm.save().move(1.1, 0.2, 9.0);
      mm.slab([[0, 0], [3.2, -1.6], [3.2, -4.0], [0, -4.6]], 0.28, 0.09, col);
      mm.restore();
      mm.save().move(4.2, 0.3, 7.6).rotZ(-72);
      mm.plate([[0, 1.2], [2.6, 0.4], [2.6, -1.6], [0, -2.0]], 0.24, shade(col, 1.06));
      mm.restore();
      if (mm.far) return;
      // Tailplane and a canted fin, which is the silhouette everybody knows.
      mm.save().move(1.0, 0.1, 2.6);
      mm.plate([[0, 0.6], [2.6, -0.6], [2.6, -1.9], [0, -2.2]], 0.2, shade(col, 0.96));
      mm.restore();
      mm.save().move(0.9, 0.7, 3.4).rotZ(-70);
      mm.plate([[0, 1.4], [2.8, -0.4], [2.8, -1.5], [0, -2.0]], 0.2, shade(col, 1.04));
      mm.restore();
    });
    if (!m.far) {
      m.save().move(0, 1.1, 12.2);
      m.soft((mm) => mm.ball(0.9, mm.lod(8, 5), mm.lod(4, 3), P.canopy));
      m.restore();
    }
    m.restore();
  }

  Object.assign(MODELS, {
    // ----------------------------------------------------------------- naval
    // FIVE ENTRIES OF HULL AND ONE OF MOUNT, AND WHAT EACH HAS TO SAY. A patrol
    // craft is a boat with a gun and two RIBs; a frigate is a radar with a hull
    // under it; a task group is three ships in company and never one
    // super-ship; an SSN is a cylinder whose whole character is its casing edge
    // and its sail; an AIP boat is the same shape smaller with X-planes; and
    // the laser mount is a telescope on a gun ring. All six come off the kit
    // above and differ only in what is bolted to them, which is also the truth
    // about warships: a frigate and a destroyer ARE the same hull with a
    // different fit, and that is why the deck prices them as one entry.
    nav_patrol: {
      name: "Patrol and Coastal Craft", cls: "Naval", span: 35,
      build(m) {
        const L = 35, fb = 2.4;
        ship(m, {
          len: L, beam: 7.2, draft: 1.9, freeboard: fb, sheer: 0.9,
          col: shade(P.navy, 0.94), tumble: 0.9, bulwark: 0.5, railStep: 2,
          houses: [[0.28, 0.66, 0.78, 2.6], [0.36, 0.58, 0.56, 1.9, 2.6]],
          running: 0.55, rudder: 0.8,
          bollards: [0.13, 0.3, 0.68, 0.88], rafts: [0.22],
        });
        mast(m, { y: fb + 4.5, z: L * 0.46, r: 0.22, h: 4.2, radar: true,
          arrayW: 2.0, arrayH: 0.52 });
        deckGun(m, { y: fb, z: L * 0.79, r: 0.8 });
        // Two rigid inflatables on the after deck. A patrol craft's job is
        // boarding, so on this hull the boats are not furniture — they are the
        // armament, and they are why the transom is cut down.
        m.both((mm) => {
          seaBoat(mm, { x: 2.0, y: fb + 0.35, z: L * 0.19, len: 6.4, w: 2.2, yaw: 3 });
          mm.save().move(2.0, fb, L * 0.19);
          mm.bar(-1.0, 1.0, 0, 0.34, -2.6, 2.6, P.greyDark);
          mm.restore();
        });
        if (!m.far) {
          // A remote weapon station over the bridge and a searchlight beside
          // it: the two things every constabulary hull carries and no warship
          // model ever draws.
          m.save().move(0, fb + 4.5, L * 0.56);
          m.save().rotX(-90);
          drum(m, { r: 0.3, r1: 0.26, len: 0.34, seg: m.lod(12, 6),
            col: shade(P.navy, 1.2) });
          m.restore();
          m.save().move(0, 0.34, 0.1).rotX(-12);
          m.bar(-0.24, 0.24, -0.12, 0.22, -0.3, 0.36, P.greyDark);
          m.save().move(0, 0.06, 0.36);
          drum(m, { r: 0.05, r1: 0.045, len: 0.9, seg: m.lod(8, 4), col: P.metal });
          m.restore();
          m.restore();
          m.restore();
          m.both((mm) => {
            mm.save().move(1.5, fb + 4.4, L * 0.5).rotX(-8);
            mm.soft((s) => s.tube(0.22, 0.24, 0.26, s.lod(12, 6), P.white, false));
            mm.save().move(0, 0, 0.26);
            mm.fan(ring(0.24, 0.24, mm.lod(12, 6)).map((p) => [p[0], p[1], 0]),
              shade(P.glass, 1.3));
            mm.restore();
            mm.restore();
          });
          // The exhaust outlets in the transom and the boarding ladder on the
          // quarter. Both are holes, and holes are what a card can read.
          m.both((mm) => {
            mm.save().move(1.6, 0.62, 0.05);
            mm.soft((s) => s.loft([{ z: 0, pts: ring(0.34, 0.3, s.lod(10, 5)) },
              { z: -0.7, pts: ring(0.26, 0.24, s.lod(10, 5)) }], shade(P.black, 1.4), false));
            mm.restore();
            for (let i = 0; i < 5; i += 1) {
              mm.save().move(3.35, fb - 0.3 - i * 0.34, L * 0.34);
              mm.bar(-0.16, 0.16, -0.03, 0.03, -0.22, 0.22, shade(P.metal, 0.9));
              mm.restore();
            }
          });
          boltRing(m, { r: 0.86, br: 0.035, h: 0.05, n: 12, y: fb + 0.02,
            z: L * 0.79, col: shade(P.metal, 0.9) });
        }
      },
    },
    nav_escort: {
      name: "Escort Frigate or Destroyer", cls: "Naval", span: 133,
      build(m) {
        const L = 133, fb = 6.2, B = 14.6;
        ship(m, {
          len: L, beam: B, draft: 4.6, freeboard: fb, sheer: 2.0, tumble: 0.9,
          // Three tiers, each standing on the one below rather than all three
          // rising from the deck: that is what a superstructure IS, and the
          // step between tiers is the shadow a card reads the profile from.
          houses: [[0.2, 0.68, 0.8, 5.2], [0.4, 0.63, 0.58, 3.4, 5.2],
            [0.46, 0.6, 0.42, 2.6, 8.6]],
          running: 1.5, rudder: 2.6, railStep: 2,
          bollards: [0.1, 0.28, 0.66, 0.88], rafts: [0.34, 0.44],
        });
        deckGun(m, { y: fb, z: L * 0.79, r: 2.0 });
        // Two strike-length launcher groups, forward of the bridge and abaft
        // the funnel, which is where they sit on every escort built since 1985.
        vls(m, { w: 6.6, y: fb, h: 1.4, z0: L * 0.685, z1: L * 0.745, cols: 4, rows: 4 });
        vls(m, { w: 5.4, y: fb, h: 1.4, z0: L * 0.28, z1: L * 0.325, cols: 3, rows: 3 });
        mast(m, { y: fb + 11.2, z: L * 0.53, r: 0.62, h: 10.4, radar: true,
          arrayW: 4.8, arrayH: 1.5 });
        // The funnel, raked, with its uptake caps. A warship whose funnel is
        // vertical is a warship drawn from memory.
        m.save().move(0, fb + 5.2, L * 0.36).rotX(-9);
        m.loft([
          { z: -3.6, pts: [[2.5, 0], [2.2, 5.2], [-2.2, 5.2], [-2.5, 0]] },
          { z: 2.6, pts: [[2.4, 0], [2.1, 5.4], [-2.1, 5.4], [-2.4, 0]] },
          { z: 3.2, pts: [[1.9, 0], [1.7, 5.0], [-1.7, 5.0], [-1.9, 0]] },
        ], shade(P.navy, 0.9), true);
        m.restore();
        if (!m.far) {
          m.both((mm) => {
            // An uptake cap is 1.4 m across and 0.6 m tall, which is the exact
            // shape `drum` exists for: a capped `tube` inside a smoothing group
            // lets the flat cap outvote the wall it shares its rim with, and
            // this one measured 0.0693 — 86 degrees off its own face — the
            // worst corner on the frigate outside the boat. Same triangles.
            mm.save().move(1.1, fb + 10.6, L * 0.36).rotX(-9).rotX(-90);
            drum(mm, { r: 0.7, r1: 0.62, len: 0.6, seg: mm.lod(12, 6),
              col: shade(P.black, 1.3) });
            mm.restore();
          });
        }
        // Hangar, flight deck and the landing circle. A frigate without a pad
        // cannot do the thing frigates are bought for.
        deckHouse(m, { w: B * 0.86, z0: L * 0.135, z1: L * 0.245, y: fb, h: 5.4,
          col: shade(P.navy, 1.04), tumble: 0.97, fwd: 0.97, aft: 0.97 });
        m.bar(-6.1, 6.1, fb, fb + 0.3, L * 0.02, L * 0.135, P.navyDeck);
        m.save().move(0, fb + 0.33, L * 0.075).rotX(-90).scale(1, 1, 0.03);
        m.tube(4.3, 4.3, 1, m.lod(20, 10), shade(P.white, 0.95));
        m.restore();
        if (!m.far) {
          // The hangar doors, and the flight-deck grid the helicopter is
          // hauled down onto.
          m.save().move(0, fb + 0.4, L * 0.134);
          m.slab([[-4.6, -0.4], [4.6, -0.4], [4.6, 0.4], [-4.6, 0.4]], 4.4, 0.22,
            shade(P.navy, 1.14));
          m.restore();
          m.save().move(0, fb + 0.35, L * 0.075);
          m.slab([[-1.5, -1.5], [1.5, -1.5], [1.5, 1.5], [-1.5, 1.5]], 0.16, 0.05,
            shade(P.navyDeck, 0.8));
          m.restore();
        }
        // Close-in mounts over the hangar and on the forward superstructure,
        // the four fixed array faces on the bridge block, and the satcom domes.
        if (!m.far) {
          ciws(m, { y: fb + 5.4, z: L * 0.19, r: 1.0 });
          ciws(m, { y: fb + 8.6, z: L * 0.62, r: 1.0, yaw: 180 });
        }
        m.both((mm) => {
          if (mm.far) return;
          mm.save().move(B * 0.36, fb + 4.0, L * 0.615);
          arrayFace(mm, { r: 1.7, tilt: 12, squash: 0.9 });
          mm.restore();
          mm.save().move(B * 0.36, fb + 4.0, L * 0.245).rotY(180);
          arrayFace(mm, { r: 1.6, tilt: 12, squash: 0.9 });
          mm.restore();
          mm.save().move(3.2, fb + 9.2, L * 0.45).scale(1, 1.1, 1);
          mm.soft((s) => s.ball(1.5, s.lod(14, 6), s.lod(7, 4), shade(P.white, 1.02)));
          mm.restore();
          // Torpedo tubes on the waist, angled outboard, and the boats above
          // them in their davits.
          mm.save().move(B * 0.4, fb + 1.1, L * 0.33).rotY(-32);
          for (let i = 0; i < 3; i += 1) {
            mm.save().move(0, i === 2 ? 0.9 : 0, (i - 0.5) * 0.95);
            mm.save().rotY(90);
            drum(mm, { r: 0.44, len: 3.2, seg: mm.lod(10, 5),
              col: shade(P.white, 0.92) });
            mm.restore();
            mm.restore();
          }
          mm.restore();
          seaBoat(mm, { x: B * 0.42, y: fb + 4.4, z: L * 0.42, len: 7.2, w: 2.6, yaw: 2 });
        });
      },
    },
    nav_blue: {
      name: "Blue-Water Task Group", cls: "Naval", span: 333,
      build(m) {
        const L = 333, fb = 17.0;
        // THE CARRIER FIRST, AND IT IS ONLY THE FIRST. This entry is a task
        // group and a task group is not one ship; the two escorts below are in
        // company at the station they would actually keep, and the budget is
        // split three ways on purpose. A single fictional super-ship would be
        // cheaper to draw and would be a lie about how navies work.
        ship(m, {
          len: L, beam: 41, draft: 11.3, freeboard: fb, sheer: 2.0,
          col: shade(P.navy, 0.9), bulwark: 1.2, lean: true, fittings: false,
          running: 3.4, rudder: 5.6,
        });
        // The flight deck: wider than the hull, overhanging to port, and the
        // reason the ship exists. Everything else up here is furniture.
        m.save().move(0, fb + 0.9, 0);
        m.slab([[38.5, L * 0.02], [38.5, L * 0.86], [24, L * 0.99], [-24, L * 0.99],
          [-38.5, L * 0.86], [-38.5, L * 0.02]], 1.6, 0.5, P.navyDeck);
        if (!m.far) {
          // The angled deck, nine degrees to port, and the two bow catapults.
          m.save().move(-6, 0.9, L * 0.45).rotY(9);
          m.slab([[-11, -L * 0.3], [11, -L * 0.3], [11, L * 0.3], [-11, L * 0.3]], 0.4, 0.12,
            shade(P.navyDeck, 1.14));
          m.restore();
          [[-8, 0.62, 0.98, 0], [11, 0.62, 0.98, 0], [-19, 0.16, 0.44, 9]].forEach((c) => {
            m.save().move(c[0], 0.9, L * (c[1] + c[2]) / 2).rotY(c[3]);
            m.slab([[-0.7, -L * (c[2] - c[1]) / 2], [0.7, -L * (c[2] - c[1]) / 2],
              [0.7, L * (c[2] - c[1]) / 2], [-0.7, L * (c[2] - c[1]) / 2]], 0.3, 0.1,
            shade(P.navyDeck, 0.7));
            m.restore();
          });
          // Jet blast deflectors, up behind each catapult, and the deck-edge
          // lift on the starboard quarter. Both are plate standing proud, both
          // are what a flight deck has instead of markings.
          [[-8, 0.55], [11, 0.55]].forEach((d) => {
            m.save().move(d[0], 0.9, L * d[1]).rotX(-42);
            m.slab([[-5.0, -0.3], [5.0, -0.3], [5.0, 2.6], [-5.0, 2.6]], 0.5, 0.16,
              shade(P.navyDeck, 1.3));
            m.restore();
          });
          m.save().move(38.5, 0.4, L * 0.2);
          m.slab([[0, -9], [11, -9], [11, 9], [0, 9]], 1.4, 0.4, shade(P.navyDeck, 1.08));
          m.restore();
        }
        m.restore();
        // The island, starboard side and well aft of amidships, with its own
        // mast, its fixed array faces and the funnel uptakes through it.
        m.save().move(21, fb + 1.8, L * 0.42);
        deckHouse(m, { w: 12.4, z0: -15, z1: 13, y: 0, h: 13.5,
          col: shade(P.navy, 1.05), tumble: 0.92 });
        if (!m.far) {
          deckHouse(m, { w: 9.0, z0: -8, z1: 8, y: 13.5, h: 5.0, plain: true,
            col: shade(P.navy, 1.02), tumble: 0.94 });
        }
        mast(m, { y: 18.5, z: 0, r: 0.85, h: 12.0, radar: true, arrayW: 5.2, arrayH: 1.6 });
        if (!m.far) {
          m.both((mm) => {
            mm.save().move(5.6, 9.0, 4.0);
            arrayFace(mm, { r: 2.6, tilt: 8, squash: 0.95 });
            mm.restore();
            mm.save().move(5.6, 9.0, -8.0).rotY(180);
            arrayFace(mm, { r: 2.4, tilt: 8, squash: 0.95 });
            mm.restore();
          });
          ciws(m, { x: 0, y: 18.5, z: -13.0, r: 1.1, yaw: 180 });
        }
        m.restore();
        // The air group, parked the way a deck park actually is: packed on the
        // starboard side and along the round-down, wings folded, noses out.
        // A map pin of a task group shows ships, not the deck park, so the
        // coarse mesh spends nothing on aircraft and everything on the three
        // hulls that make it a group.
        if (!m.far) {
          m.save().move(0, fb + 1.7, 0);
          [[-14, 0.62, 24], [-24, 0.5, 12], [-13, 0.36, -6], [-26, 0.24, 4],
            [16, 0.12, -170], [6, 0.2, 178], [26, 0.7, 30], [10, 0.9, 6]]
            .forEach((p) => { deckPlane(m, p[0], L * p[1], p[2], P.grey); });
          m.restore();
        }
        // Two escorts in company. They come off the lean lines table so all
        // three hulls fit one catalogue budget — a task group that spends it
        // all on the carrier is a carrier.
        [[-96, -L * 0.2, 1], [88, L * 0.12, -1]].forEach((e) => {
          m.save().move(e[0], 0, e[1]).rotY(e[2] * 4);
          const EL = 150;
          ship(m, { len: EL, beam: 16, draft: 5.0, freeboard: 6.6, sheer: 2.0,
            lines: m.far ? HULL_LINES_FAR : HULL_LINES_LEAN, lean: true, fittings: false,
            houses: m.far ? [[0.26, 0.66, 0.78, 5.4]]
              : [[0.26, 0.66, 0.78, 5.4], [0.44, 0.62, 0.56, 3.4, 5.4]] });
          if (!m.far) {
            mast(m, { y: 6.6 + 8.8, z: EL * 0.52, r: 0.62, h: 9.0, radar: true,
              arrayW: 4.2, arrayH: 1.3, bays: 3 });
            deckGun(m, { y: 6.6, z: EL * 0.79, r: 2.0 });
            vls(m, { w: 6.0, y: 6.6, h: 1.3, z0: EL * 0.69, z1: EL * 0.745,
              cols: 3, rows: 3 });
          }
          m.restore();
        });
      },
    },
    la_ssn: {
      name: "Los Angeles-class SSN", cls: "Naval", span: 110,
      build(m) {
        // The improved boats carry twelve vertical launch tubes forward of the
        // pressure hull's forward bulkhead, and the ring of muzzle hatches in
        // the casing is the one feature that tells a 688I from a 688 at a
        // glance. Fairwater planes on the sail, cruciform stern.
        submarine(m, { len: 110, dia: 10, sailLen: 16, sailH: 6.2, sailPlanes: true,
          tubes: 12, tubeZ: [0.72, 0.86] });
      },
    },
    aip_ssk: {
      name: "Air-Independent Propulsion Submarine", cls: "Naval", span: 56,
      build(m) {
        // A smaller, lighter boat: X-planes aft, bow planes on the hull, flank
        // arrays down each side and the snort induction on the sail. The
        // lighter finish is the anechoic coating rather than bare steel.
        submarine(m, { len: 56, dia: 6.2, sailLen: 8.4, sailH: 3.6, xstern: true,
          col: shade(P.hull, 1.15), flankArray: true });
      },
    },
    laws: {
      name: "Shipboard Directed-Energy Mount", cls: "Naval", span: 3.4,
      build(m) {
        // A DIRECTED-ENERGY MOUNT IS A TELESCOPE ON A GUN RING, and that is the
        // whole of the design brief. Everything below the trunnions is a
        // training base bolted to a deck; everything above them is optics in a
        // tube with a coolant plant hanging off it. Drawn as a cone with a
        // stick in it — which is what it was — it reads as a searchlight, so
        // every fastener, louvre and cable run below is there to say machinery.
        const seg = m.lod(20, 8);
        // The deck ring, its bearing race and the bolt circle that holds the
        // whole mount to the ship.
        m.save().rotX(-90);
        drum(m, { r: 0.95, r1: 0.9, len: 0.22, seg, col: P.navyDeck });
        m.restore();
        m.save().move(0, 0.22, 0).rotX(-90);
        drum(m, { r: 0.82, r1: 0.8, len: 0.14, seg, col: shade(P.metal, 0.66) });
        m.restore();
        boltRing(m, { r: 0.9, br: 0.04, h: 0.07, n: 24, y: 0.19,
          col: shade(P.metal, 0.9) });
        boltRing(m, { r: 0.79, br: 0.028, h: 0.05, n: 12, y: 0.34,
          phase: 0.26, col: shade(P.metal, 0.72) });
        // The gratings round the base and the handrails on them. A mount this
        // size is maintained by people standing on it, and the platform and its
        // rail are the two things that give a card the mount's scale — without
        // them a laser director is the same picture at any size.
        // Traced CLOCKWISE in [x, z], which is the same order the deck outline
        // above uses and the order `strip` reads as outboard: a ring walked the
        // other way would hand every face on it a normal pointing into the
        // mount.
        const ringAt = (r, n) => {
          const out = [];
          for (let i = 0; i < n; i += 1) {
            const a = (i / n) * Math.PI * 2;
            out.push([Math.cos(a) * r, -Math.sin(a) * r]);
          }
          return out;
        };
        strip(m, ringAt(0.98, m.lod(16, 8)), { y: 0.02, h: 0.1, t: 0.3, closed: true,
          col: shade(P.navyDeck, 1.12) });
        strip(m, ringAt(1.28, m.lod(16, 8)), { y: 0.12, h: 0.05, t: 0.04, closed: true,
          col: shade(P.metal, 0.62) });
        for (let i = 0; i < m.lod(8, 4); i += 1) {
          const a = (i / m.lod(8, 4)) * Math.PI * 2 + 0.4;
          m.save().move(Math.cos(a) * 1.26, 0.11, Math.sin(a) * 1.26);
          m.bar(-0.028, 0.028, 0, 0.55, -0.028, 0.028, shade(P.metal, 0.7));
          m.restore();
        }
        for (let i = 0; i < m.lod(8, 4); i += 1) {
          const n0 = m.lod(8, 4);
          const a0 = (i / n0) * Math.PI * 2 + 0.4, a1 = ((i + 1) / n0) * Math.PI * 2 + 0.4;
          const p0 = [Math.cos(a0) * 1.26, Math.sin(a0) * 1.26];
          const p1 = [Math.cos(a1) * 1.26, Math.sin(a1) * 1.26];
          const len = Math.hypot(p1[0] - p0[0], p1[1] - p0[1]);
          m.save().move(p0[0], 0.11, p0[1])
            .rotY(Math.atan2(p1[0] - p0[0], p1[1] - p0[1]) / DEG);
          [0.62, 1].forEach((f) => {
            m.save().move(0, 0.55 * f, 0);
            m.bar(-0.022, 0.022, -0.022, 0.022, 0, len, P.metal);
            m.restore();
          });
          m.restore();
        }
        // The training house: a chamfered box, because everything on a modern
        // upper deck has its corners knocked off to keep radar off it.
        const hs = (f, top) => {
          const w = 0.8 * f, c = 0.18;
          return [[w, 0], [w, top - c], [w - c, top], [-w + c, top], [-w, top - c], [-w, 0]];
        };
        m.save().move(0, 0.36, 0).rotY(16);
        m.loft([
          { z: -0.98, pts: hs(0.68, 0.86) },
          { z: -0.74, pts: hs(0.94, 1.0) },
          { z: 0.42, pts: hs(1, 1.04) },
          { z: 0.8, pts: hs(0.86, 0.96) },
          { z: 0.98, pts: hs(0.58, 0.8) },
        ], P.white, true);
        if (!m.far) {
          // The equipment access panels, each on its own bolt line, and the
          // cooling louvres for the laser's chiller. A blank white box is a
          // fridge; the louvres are what make it a weapon.
          m.both((mm) => {
            mm.save().move(0.79, 0.3, -0.2).rotZ(90);
            mm.slab([[-0.02, -0.42], [0.6, -0.42], [0.6, 0.42], [-0.02, 0.42]],
              0.06, 0.02, shade(P.white, 0.9));
            mm.restore();
            mm.save().move(0.8, 0.59, -0.2).rotZ(-90);
            boltRow(mm, { x: 0, w: 0.27, y: 0, z0: -0.38, z1: 0.38, n: 4,
              br: 0.02, h: 0.035 });
            mm.restore();
            for (let i = 0; i < 11; i += 1) {
              mm.save().move(0.8, 0.58 + i * 0.042, 0.5).rotZ(90).rotX(-24);
              mm.slab([[0, -0.3], [0.04, -0.3], [0.04, 0.3], [0, 0.3]], 0.05, 0.014,
                shade(P.greyDark, 1.1));
              mm.restore();
            }
          });
          // The chiller skid on the back of the house, its intake grille and
          // the two coolant runs up to the director. A laser is a heat engine
          // that throws away almost all of it, so the plumbing is not detail —
          // it is most of the machine, and drawing it is the difference between
          // a weapon and a spotlight.
          m.save().move(0, 0.62, -1.24);
          m.loft([
            { z: -0.16, pts: [[0.5, -0.36], [0.42, 0.4], [-0.42, 0.4], [-0.5, -0.36]] },
            { z: 0.12, pts: [[0.56, -0.4], [0.46, 0.46], [-0.46, 0.46], [-0.56, -0.4]] },
            { z: 0.3, pts: [[0.48, -0.34], [0.4, 0.4], [-0.4, 0.4], [-0.48, -0.34]] },
          ], shade(P.white, 0.9), true);
          m.restore();
          m.save().move(0, 0.5, -0.99);
          m.slab([[-0.5, -0.34], [0.5, -0.34], [0.5, 0.34], [-0.5, 0.34]], 0.1, 0.03,
            shade(P.sensor, 1.2));
          m.restore();
          m.both((mm) => {
            mm.save().move(0.44, 1.02, -1.1).rotX(-64);
            drum(mm, { r: 0.07, r1: 0.065, len: 0.62, seg: mm.lod(10, 5),
              col: shade(P.metal, 0.78) });
            mm.restore();
            mm.save().move(0.34, 1.2, -0.5).rotZ(-8);
            drum(mm, { r: 0.055, len: 0.34, seg: mm.lod(9, 5),
              col: shade(P.metal, 0.66) });
            mm.restore();
          });
          for (let i = 0; i < 4; i += 1) {
            m.save().move(-0.5 + i * 0.33, 0.16, -1.02).rotX(-80);
            drum(m, { r: 0.05, len: 0.5, seg: m.lod(8, 4), col: P.black });
            m.restore();
          }
        }
        m.restore();
        // The trunnions and the beam director between them. The director is a
        // coelostat: a big tube with a window recessed behind a lip, tilted up
        // the way a mount tracking something is tilted up.
        m.save().move(0, 1.44, 0).rotY(16);
        m.both((mm) => {
          mm.save().move(0.72, 0, 0.1).rotY(90);
          drum(mm, { r: 0.24, r1: 0.2, len: 0.16, seg: mm.lod(12, 6),
            col: shade(P.white, 0.86) });
          mm.restore();
          if (mm.far) return;
          boltRing(mm, { x: 0.8, y: 0, z: 0.1, r: 0.16, br: 0.018, h: 0.03, n: 8 });
          // The elevation drive: a gearcase on the starboard trunnion with its
          // motor hanging off it, which is what actually points the thing.
          mm.save().move(0.86, -0.22, 0.1);
          mm.bar(-0.1, 0.1, -0.24, 0.24, -0.22, 0.22, shade(P.white, 0.82));
          mm.restore();
          mm.save().move(0.96, -0.32, 0.1).rotY(90);
          drum(mm, { r: 0.11, r1: 0.1, len: 0.26, seg: mm.lod(10, 5),
            col: shade(P.sensor, 1.1) });
          mm.restore();
        });
        m.save().rotX(-17);
        m.soft((mm) => mm.loft([
          { z: -0.66, pts: ring(0.3, 0.3, mm.lod(18, 8)) },
          { z: -0.4, pts: ring(0.42, 0.42, mm.lod(18, 8)) },
          { z: 0.5, pts: ring(0.44, 0.44, mm.lod(18, 8)) },
          { z: 0.86, pts: ring(0.4, 0.4, mm.lod(18, 8)) },
        ], shade(P.white, 0.94), false));
        m.fan(ring(0.3, 0.3, m.lod(18, 8)).map((q) => [q[0], q[1], -0.66]).reverse(),
          shade(P.white, 0.8));
        // The aperture: the shade ring, the recessed window and the ring of
        // fasteners that holds it in. A flat bright disc on the end of a tube
        // reads as a bead; three parts read as optics.
        m.soft((mm) => mm.loft([
          { z: 0.86, pts: ring(0.42, 0.42, mm.lod(18, 8)) },
          { z: 0.98, pts: ring(0.44, 0.44, mm.lod(18, 8)) },
          { z: 1.02, pts: ring(0.38, 0.38, mm.lod(18, 8)) },
        ], shade(P.greyDark, 1.05), false));
        m.save().move(0, 0, 0.96);
        m.soft((mm) => {
          const rings = mm.lod(4, 2), secs = [];
          for (let i = 0; i <= rings; i += 1) {
            const a = (i / rings) * Math.PI * 0.5;
            secs.push({ z: Math.sin(a) * 0.1, pts: ring(Math.max(1e-3, Math.cos(a) * 0.38),
              Math.max(1e-3, Math.cos(a) * 0.38), mm.lod(18, 8)) });
          }
          mm.loft(secs, P.glass, false);
        });
        m.restore();
        if (!m.far) {
          boltRing(m, { r: 0.41, br: 0.016, h: 0.025, n: 14, y: 0, z: 0.94,
            col: shade(P.metal, 0.8) });
          // The boresight camera and the coarse tracker, slung under the
          // director where the crew can align them.
          m.save().move(0.22, -0.44, 0.4);
          drum(m, { r: 0.1, r1: 0.09, len: 0.5, seg: m.lod(10, 5), col: shade(P.sensor, 1.2) });
          drum(m, { z: 0.5, r: 0.08, len: 0.03, seg: m.lod(10, 5), col: P.glass });
          m.restore();
          m.save().move(-0.24, -0.42, 0.3);
          drum(m, { r: 0.07, r1: 0.065, len: 0.62, seg: m.lod(9, 5), col: shade(P.sensor, 1.0) });
          m.restore();
          // Panel seams down the barrel, which is how an optical bench that has
          // to come apart in a dockyard is actually built.
          for (let i = 0; i < 4; i += 1) {
            jointBand(m, { z: -0.24 + i * 0.3, r: 0.44, w: 0.07, k: 1.05,
              seg: m.lod(18, 8), col: shade(P.white, 0.86) });
          }
        }
        // The beam. Nothing else in this file emits, and a directed-energy
        // mount drawn cold is indistinguishable from a searchlight.
        m.save().move(0, 0, 1.06);
        m.soft((mm) => mm.tube(0.07, 0.02, 2.3, mm.lod(8, 5), [1, 0.86, 0.55], false));
        m.restore();
        m.restore();
        m.restore();
      },
    },

    // --------------------------------------------------------------- missile
    msl_sam: {
      name: "Area Air-Defence System", cls: "Missile", span: 13.1,
      build(m) {
        // A LAUNCHER IS A LORRY FIRST. Everything below the deck is running
        // gear — eight wheels with crowned tyres and stud circles, frame rails,
        // mudguards, a cab with door and window frames — because that is what
        // this vehicle mostly IS, and drawing it as a box on four black discs
        // threw away three quarters of the object.
        const bed = truck(m, { len: 13.1, w: 3.2, wheelR: 0.62, axles: 4, cab: 2.8, cabH: 1.9,
          col: P.olive, detail: true });
        // The slew ring. A cold-launch area system stands its rounds up on a
        // deck that turns, and the ring is the joint that lets it — it is also
        // what tells this apart from the theatre battery two entries down,
        // which slants its box at the horizon instead.
        m.save().move(0, bed, 3.6).rotX(-90);
        m.soft((mm) => mm.tube(1.55, 1.44, 0.36, mm.lod(20, 8), shade(P.olive, 0.82)));
        m.restore();
        m.save().move(0, bed + 0.36, 3.6).rotX(-84);
        // Four sealed canisters. The hoops and the lid are the detail: a
        // canister drawn as a plain pipe is a plain pipe, and four plain pipes
        // standing in a square read as scaffolding.
        for (let i = 0; i < 4; i += 1) {
          m.save().move(((i % 2) - 0.5) * 1.5, (Math.floor(i / 2) - 0.5) * 1.5, 0);
          canister(m, { r: 0.62, len: 7.4, col: shade(P.olive, 1.1), hoops: 5 });
          m.restore();
        }
        // The frame that holds the four as one bundle. Without it they float in
        // formation, which is what the old model did.
        lattice(m, { z: 0.5, len: 6.2, w: 1.48, h: 1.48, t: 0.085, n: 4,
          col: shade(P.olive, 0.74) });
        m.restore();
        // The erector rams: two of them, off the deck to the base of the frame,
        // and they are the reason the bundle is up rather than lying down.
        m.both((mm) => {
          mm.save().move(1.15, bed + 0.2, 1.5).rotX(-58);
          mm.soft((s) => s.tube(0.14, 0.14, 2.5, s.lod(10, 5), shade(P.metal, 0.7)));
          mm.save().move(0, 0, 2.5);
          mm.soft((s) => s.tube(0.09, 0.09, 0.7, s.lod(8, 5), P.metal));
          mm.restore();
          mm.restore();
        });
        // The generator and the crew shelter behind the cab: an area battery
        // needs power on the vehicle and somebody sitting next to it.
        m.save().move(0, bed, 8.4);
        m.bar(-1.34, 1.34, 0, 1.25, 0, 1.9, shade(P.olive, 0.9));
        m.restore();
        if (!m.far) {
          m.both((mm) => {
            mm.save().move(1.36, bed + 0.62, 9.3);
            mm.slab([[-0.34, -0.62], [0.34, -0.62], [0.34, 0.62], [-0.34, 0.62]], 0.1, 0.03,
              shade(P.olive, 0.76));
            mm.restore();
          });
          // Four jacks, down. An emplaced battery stands on them with the
          // wheels unloaded; a launcher without them is a lorry that parked.
          [[1.62, 1.5], [-1.62, 1.5], [1.62, 6.4], [-1.62, 6.4]].forEach((p) => {
            jack(m, { x: p[0], y: bed - 0.5, z: p[1], r: 0.16, len: bed - 0.62 });
          });
          // The cable reel and the junction box the battery is wired into.
          m.save().move(-1.3, bed + 0.3, 11.0).rotY(90);
          m.soft((mm) => mm.tube(0.3, 0.3, 0.44, mm.lod(12, 6), shade(P.olive, 1.16)));
          m.restore();
          m.save().move(1.2, bed + 0.2, 11.0);
          m.bar(-0.24, 0.24, 0, 0.5, -0.3, 0.3, P.sensor);
          m.restore();
        }
      },
    },
    msl_brm: {
      name: "Theatre Ballistic Missile", cls: "Missile", span: 13.4,
      build(m) {
        const bed = truck(m, { len: 13.4, w: 3.0, wheelR: 0.66, axles: 4, cab: 3.0, cabH: 2.0,
          col: shade(P.olive, 0.92), detail: true });
        // Erected. A transporter-erector-launcher with the round down is a
        // lorry; with the round up it is the thing the deck is pricing.
        const R = 0.44, ML = 11.25, seg = m.lod(24, 8);
        m.save().move(0, bed, 2.6).rotX(-74);
        // The launch table the round stands on while the arm holds it.
        m.soft((mm) => mm.tube(0.86, 0.78, 0.3, mm.lod(16, 7), shade(P.metal, 0.6)));
        m.save().move(0, 0, 0.3);
        missile(m, {
          len: ML, r: R, col: shade(P.warhead, 0.94), nose: 2.6, seg, soft: true, motor: false,
          // Stations at every place the round actually changes section: the aft
          // skirt, the parallel motor case, the interstage step and the
          // re-entry body's ogive. A missile lofted from five stations is a
          // pencil with a point on it.
          stations: [[0, .72, .72], [.012, .95, .95], [.03, 1, 1], [.18, 1, 1], [.34, 1, 1],
            [.5, 1, 1], [.62, 1, 1], [.7, 1, 1], [.755, 1, 1], [.775, .97, .97],
            [.83, .87, .87], [.89, .71, .71], [.94, .5, .5], [.975, .29, .29], [1, .05, .05]],
        });
        // Where the cans bolt together, the wiring that runs between them and
        // the fins that steer it, each filleted into the skin.
        [0.03, 0.29, 0.545, 0.755].forEach((t) => {
          jointBand(m, { z: t * ML, r: R, w: R * 0.44, seg, col: shade(P.warhead, 0.8) });
        });
        raceway(m, { roll: 0, r: R, h: R * 0.16, z0: ML * 0.05, z1: ML * 0.74,
          col: shade(P.warhead, 0.86) });
        raceway(m, { roll: 180, r: R, h: R * 0.1, z0: ML * 0.06, z1: ML * 0.5,
          col: shade(P.warhead, 0.86) });
        finSet(m, { z: ML * 0.02, r: R, n: 4, span: 0.55, chord: 1.5, tip: 0.7, sweep: 0.5,
          roll: 45, thick: R * 0.2, col: shade(P.warhead, 0.9) });
        motorBell(m, { z: 0, r: R * 0.78, len: ML * 0.055 });
        if (!m.far) {
          // The umbilical fairing: the plug the launch vehicle pulls out of the
          // round at first motion, and the one asymmetry on an otherwise
          // perfectly turned object.
          m.save().move(0, R * 0.98, ML * 0.66);
          m.slab([[-R * 0.3, -0.5], [R * 0.3, -0.44], [R * 0.3, 0.44], [-R * 0.3, 0.5]],
            R * 0.4, R * 0.12, shade(P.warhead, 0.72));
          m.restore();
        }
        m.restore();
        m.restore();
        // The erector arm, in lattice: it is see-through, and the see-through
        // is what makes it read as structure rather than as a beam.
        m.save().move(0, bed + 0.1, 1.2).rotX(-74);
        lattice(m, { y: -0.62, z: 0.4, len: 8.2, w: 0.62, h: 0.3, t: 0.075, n: 6,
          col: shade(P.olive, 0.76) });
        m.restore();
        m.save().move(0, bed - 0.1, 1.2);
        m.bar(-1.3, 1.3, -0.5, 0.2, -2.6, 3.4, shade(P.olive, 0.8));
        m.restore();
        if (!m.far) {
          m.both((mm) => {
            // The erector ram, and the four jacks the vehicle stands on.
            mm.save().move(1.05, bed - 0.1, 5.2).rotX(-38);
            mm.soft((s) => s.tube(0.15, 0.15, 2.4, s.lod(10, 5), shade(P.metal, 0.7)));
            mm.restore();
          });
          [[1.5, 1.0], [-1.5, 1.0], [1.5, 6.2], [-1.5, 6.2]].forEach((p) => {
            jack(m, { x: p[0], y: bed - 0.55, z: p[1], r: 0.17, len: bed - 0.66 });
          });
        }
      },
    },
    msl_deterrent: {
      name: "Strategic Deterrent Force", cls: "Missile", span: 21,
      build(m) {
        // A silo, its hatch thrown clear, and the round leaving it. The PAD IS
        // AS MUCH OF THE MODEL AS THE MISSILE: a deterrent is a fixed
        // installation somebody has to find, and what they find is a concrete
        // collar, a hatch on rails, and a hole with a depth to it. The old
        // model drew the hole as a black disc, which is a manhole cover.
        const seg = m.lod(30, 9);
        m.save().rotX(-90);
        // The apron: the hardstanding the collar sits in, sloped to drain. It
        // is the flattest thing in the deck and it is what puts the silo in a
        // place rather than in the air.
        m.soft((mm) => mm.loft([
          { z: -0.34, pts: ring(6.3, 6.3, seg) },
          { z: -0.28, pts: ring(5.4, 5.4, seg) },
          { z: -0.25, pts: ring(4.7, 4.7, seg) },
        ], shade(P.metal, 0.46), false));
        m.soft((mm) => mm.loft([
          { z: -0.25, pts: ring(4.7, 4.7, seg) },
          { z: 0.72, pts: ring(4.4, 4.4, seg) },
          { z: 1.12, pts: ring(3.6, 3.6, seg) },
          { z: 1.3, pts: ring(3.15, 3.15, seg) },
        ], shade(P.metal, 0.55), false));
        // The muzzle ring and the liner running down into the dark. Listed in
        // DECREASING z on purpose: that reverses the winding, which is what
        // makes a bore face inward the way the inside of a hole does.
        m.soft((mm) => mm.loft([
          { z: 1.3, pts: ring(3.15, 3.15, seg) },
          { z: 0.3, pts: ring(2.72, 2.72, seg) },
          { z: -2.4, pts: ring(2.62, 2.62, seg) },
        ], shade(P.black, 1.55), false));
        m.save().move(0, 0, -2.4);
        m.fan(ring(2.62, 2.62, seg).map((p) => [p[0], p[1], 0]), P.black);
        m.restore();
        m.restore();
        // The hatch, thrown clear on its rails, ribbed underneath the way a
        // slab that has to stop a near miss is ribbed.
        m.save().move(5.3, 0.55, 0).rotZ(-22);
        m.slab([[-3.0, -3.0], [3.0, -3.0], [3.0, 3.0], [-3.0, 3.0]], 0.62, 0.18,
          shade(P.metal, 0.62));
        if (!m.far) {
          for (let i = 0; i < 5; i += 1) {
            m.bar(-2.8, 2.8, -0.62, -0.28, -2.4 + i * 1.2, -2.12 + i * 1.2, shade(P.metal, 0.44));
          }
        }
        m.restore();
        if (!m.far) {
          // The rail beams the hatch runs out on, and the ram that drives it.
          m.both((mm) => {
            mm.bar(1.9, 2.5, 0, 0.42, 3.1, 6.1, shade(P.metal, 0.5));
          });
          m.save().move(4.0, 0.5, 0).rotZ(-90).rotX(-90);
          m.soft((mm) => mm.tube(0.22, 0.22, 2.2, mm.lod(10, 5), shade(P.metal, 0.74)));
          m.restore();
          // Personnel hatch, cable trays and the bollards round the apron: the
          // furniture of a launch facility, and what says it is manned.
          m.save().move(-4.6, 0.1, 3.0);
          m.bar(-0.8, 0.8, 0, 0.34, -0.8, 0.8, shade(P.metal, 0.66));
          m.restore();
          m.save().move(-4.9, 0.1, -1.0);
          m.bar(-0.5, 0.5, 0, 0.26, -2.6, 2.6, shade(P.metal, 0.48));
          m.restore();
          for (let i = 0; i < 8; i += 1) {
            m.save().rotY(i * 45).move(0, 0, 5.5).rotX(-90);
            m.soft((mm) => mm.tube(0.16, 0.14, 0.9, mm.lod(8, 5), shade(P.white, 0.7)));
            m.restore();
          }
        }
        // The round, on its way out of the canister that ejected it.
        const R = 1.05, ML = 21;
        m.save().move(0, 1.2, 0).rotX(-90);
        missile(m, {
          len: ML, r: R, col: P.white, nose: 4.2, seg, soft: true, motor: false,
          // Three stages and a shroud, with a step at every interstage, because
          // a strategic round is four objects bolted end to end and the steps
          // are the only place a card can see the joins.
          stations: [[0, .86, .86], [.012, .98, .98], [.03, 1, 1], [.18, 1, 1], [.335, 1, 1],
            [.35, .95, .95], [.365, .91, .91], [.5, .91, .91], [.62, .91, .91],
            [.635, .84, .84], [.65, .8, .8], [.78, .8, .8], [.815, .79, .79],
            [.86, .71, .71], [.92, .5, .5], [.965, .27, .27], [1, .045, .045]],
        });
        [0.032, 0.348, 0.633, 0.813].forEach((t) => {
          jointBand(m, { z: t * ML, r: R * (t > 0.6 ? (t > 0.7 ? 0.8 : 0.86) : 1),
            w: R * 0.36, seg, col: shade(P.white, 0.78) });
        });
        raceway(m, { roll: 0, r: R, h: R * 0.12, z0: ML * 0.04, z1: ML * 0.8,
          col: shade(P.white, 0.82) });
        raceway(m, { roll: 152, r: R, h: R * 0.08, z0: ML * 0.05, z1: ML * 0.61,
          col: shade(P.white, 0.82) });
        finSet(m, { z: ML * 0.005, r: R, n: 4, span: 0.5, chord: 1.6, tip: 0.8, sweep: 0.55,
          roll: 45, thick: R * 0.14, col: shade(P.white, 0.86) });
        motorBell(m, { z: 0, r: R * 0.82, len: ML * 0.05 });
        if (!m.far) {
          // Access doors down the motor cases: a stage is a pressure vessel and
          // every one of them is opened somewhere.
          [0.09, 0.14, 0.24, 0.29, 0.44, 0.49, 0.56, 0.68, 0.73].forEach((t, i) => {
            skinPanel(m, { rx: R * (t > 0.635 ? 0.8 : t > 0.35 ? 0.91 : 1), ry: R * (t > 0.635 ? 0.8 : t > 0.35 ? 0.91 : 1),
              th: i % 2 ? 108 : 252, z: t * ML,
              w: R * 0.7, len: ML * 0.028, depth: R * 0.05, col: shade(P.white, 0.84) });
          });
          // Roll-control thruster ports round the interstage, and the four
          // blast ports the second stage lights through. Both are holes with a
          // lip, which is the only way this renderer can draw a hole.
          for (let i = 0; i < 4; i += 1) {
            m.save().rotZ(45 + i * 90).move(0, R * 0.9, ML * 0.352).rotX(-90);
            m.soft((mm) => mm.tube(R * 0.14, R * 0.1, R * 0.22, mm.lod(9, 5), P.sensor));
            m.restore();
            m.save().rotZ(i * 90).move(0, R * 0.83, ML * 0.645).rotX(-90);
            m.soft((mm) => mm.tube(R * 0.1, R * 0.08, R * 0.18, mm.lod(8, 5), P.sensor));
            m.restore();
          }
          // The dark band round the shroud is the separation joint, and the
          // only marking a white round carries that is not a stencil.
          jointBand(m, { z: ML * 0.845, r: R * 0.76, w: R * 0.5, k: 1.02, seg,
            col: shade(P.sensor, 1.2) });
        }
        m.restore();
        // The eject canister still in the silo, mouth OPEN round the round —
        // which is why this is built here rather than through `canister`, whose
        // whole job is to put a frangible lid across a sealed one.
        m.save().move(0, -1.7, 0).rotX(-90);
        const CR = R * 1.24;
        m.soft((mm) => mm.tube(CR, CR, 3.5, seg, shade(P.metal, 0.7), false));
        m.save().move(0, 0, 3.5);
        m.soft((mm) => mm.loft([
          { z: 0, pts: ring(CR, CR, seg) },
          { z: 0.16, pts: ring(CR * 1.06, CR * 1.06, seg) },
          { z: 0.26, pts: ring(CR * 0.94, CR * 0.94, seg) },
        ], shade(P.metal, 0.86), false));
        m.restore();
        if (!m.far) {
          for (let i = 0; i < 3; i += 1) {
            jointBand(m, { z: 0.5 + i * 1.2, r: CR, w: CR * 0.3, k: 1.07, seg,
              col: shade(P.metal, 0.56) });
          }
        }
        m.restore();
      },
    },
    paveway: {
      name: "GBU-24 Paveway III", cls: "Missile", span: 4.4,
      build(m) {
        // THREE OBJECTS BOLTED TOGETHER, and a laser-guided bomb is only
        // recognisable when a card can see the joins: a guidance section with a
        // gimballed seeker and four canards, a penetrator warhead in the
        // middle, and the airfoil group on the back with the long low-drag
        // wings that give this mark its glide. The old model was one tube with
        // four cards stuck in it.
        const L = 4.4, R = 0.185, seg = m.lod(40, 10);
        missile(m, {
          len: L, r: R, col: P.warhead, nose: 0.5, seg, soft: true, motor: false,
          stations: [[0, .82, .82], [.01, .96, .96], [.03, 1, 1], [.14, 1, 1], [.3, 1, 1],
            [.46, 1, 1], [.6, 1, 1], [.72, 1, 1], [.8, 1, 1], [.86, .98, .98],
            [.9, .92, .92], [.945, .78, .78], [.975, .55, .55], [1, .3, .3]],
        });
        // The seeker: the window in its lip, on the bearing ring it gimbals in.
        seekerHead(m, { z: L * 0.995, r: R * 0.56, dome: 1.0, col: shade(P.metal, 0.86) });
        jointBand(m, { z: L * 0.9, r: R * 0.9, w: R * 0.5, k: 1.06, seg,
          col: shade(P.metal, 0.8) });
        // The joins: guidance to warhead, warhead to airfoil group, and the
        // band at the back of the fuze well.
        [0.03, 0.24, 0.8].forEach((t) => {
          jointBand(m, { z: t * L, r: R, w: R * 0.4, seg, col: shade(P.warhead, 0.8) });
        });
        // Canards on the guidance section, each with the actuator fairing that
        // drives it. A Paveway III's canards are the front half of its name.
        finSet(m, { z: L * 0.845, r: R * 0.97, n: 4, span: R * 1.5, chord: L * 0.095,
          tip: L * 0.055, sweep: L * 0.02, roll: 45, thick: R * 0.13,
          col: shade(P.warhead, 0.94) });
        // The airfoil group: the carrier the wings bolt to, four long wings, and
        // the four small fixed fins between them. The wing span IS this
        // weapon's silhouette, and the carrier is why the wings are attached to
        // the bomb rather than growing out of it.
        m.save().move(0, 0, L * 0.03);
        m.soft((mm) => mm.loft([
          { z: 0, pts: ring(R, R, seg) },
          { z: L * 0.03, pts: ring(R * 1.07, R * 1.07, seg) },
          { z: L * 0.26, pts: ring(R * 1.07, R * 1.07, seg) },
          { z: L * 0.29, pts: ring(R, R, seg) },
        ], shade(P.warhead, 0.92), false));
        m.restore();
        [0.075, 0.27].forEach((t) => {
          jointBand(m, { z: t * L, r: R * 1.07, w: R * 0.34, k: 1.05, seg,
            col: shade(P.metal, 0.74) });
        });
        finSet(m, { z: L * 0.3, r: R * 1.05, n: 4, span: R * 3.4, chord: L * 0.3,
          tip: L * 0.19, sweep: L * 0.055, roll: 0, thick: R * 0.11,
          col: shade(P.warhead, 0.9) });
        finSet(m, { z: L * 0.11, r: R * 0.96, n: 4, span: R * 1.4, chord: L * 0.1,
          tip: L * 0.06, sweep: L * 0.02, roll: 45, thick: R * 0.12,
          col: shade(P.warhead, 0.96) });
        if (!m.far) {
          // The hinge fittings the wings pivot on when the group is folded in a
          // bomb bay: four brackets, and the reason the wing has a root.
          for (let i = 0; i < 4; i += 1) {
            m.save().rotZ(i * 90).move(0, R * 1.1, L * 0.2);
            m.bar(-R * 0.16, R * 0.16, 0, R * 0.14, -L * 0.06, L * 0.06,
              shade(P.metal, 0.66));
            m.restore();
          }
        }
        // The two suspension lugs, fourteen inches apart, which is how every
        // air-carried store in the world hangs off a rack — and the conduit
        // that runs the seeker's wiring back to the tail.
        hoistLug(m, { z: L * 0.42, r: R, col: shade(P.metal, 0.7) });
        hoistLug(m, { z: L * 0.5, r: R, col: shade(P.metal, 0.7) });
        raceway(m, { roll: 0, r: R, h: R * 0.16, z0: L * 0.13, z1: L * 0.79,
          col: shade(P.warhead, 0.85) });
        if (m.far) return;
        // The fuze well in the base, and the arming lanyard boss beside it.
        m.save().move(0, 0, L * 0.008).rotX(180);
        m.soft((mm) => mm.tube(R * 0.5, R * 0.42, R * 0.3, mm.lod(14, 6), P.sensor));
        m.restore();
        [0.2, 0.34, 0.56, 0.68].forEach((t, i) => {
          skinPanel(m, { rx: R, ry: R, th: i % 2 ? 120 : 240, z: t * L,
            w: R * 0.66, len: L * 0.05, depth: R * 0.06, col: shade(P.warhead, 0.88) });
        });
        m.save().move(0, R * 0.94, L * 0.68);
        m.slab([[-R * 0.2, -L * 0.03], [R * 0.2, -L * 0.03], [R * 0.2, L * 0.03],
          [-R * 0.2, L * 0.03]], R * 0.24, R * 0.08, P.sensor);
        m.restore();
      },
    },
    tomahawk: {
      name: "BGM-109 Tomahawk", cls: "Missile", span: 6.25,
      build(m) {
        // A CRUISE MISSILE IS AN AEROPLANE, and the two things that say so are
        // the wings it unfolds and the hole it breathes through. The old model
        // drew the scoop as a box and the exhaust as a stub of pipe; a duct you
        // can see down and a nozzle with a dark throat are what turn a white
        // tube into an air-breathing weapon. The 6.25 m the deck quotes is the
        // round WITH its launch booster, so the airframe is 5.56 of it and the
        // booster is the rest — which is also why the tail looks the way it
        // does on a ship-launched round and not on an air-launched one.
        const R = 0.26, BL = 0.69, L = 6.25 - BL, seg = m.lod(40, 10);
        m.save().move(0, 0, BL);
        missile(m, {
          len: L, r: R, col: shade(P.white, 0.9), nose: 0.7, blunt: true, seg, soft: true,
          motor: false,
          stations: [[0, .84, .84], [.012, .97, .97], [.03, 1, 1], [.16, 1, 1], [.32, 1, 1],
            [.46, 1, 1], [.58, 1, 1], [.7, 1, 1], [.8, 1, 1], [.87, .99, .99],
            [.92, .93, .93], [.96, .78, .78], [.985, .55, .55], [1, .34, .34]],
        });
        // The wing slot fairing down the spine, and the two wings out of it.
        // A Tomahawk's wings live inside the body until launch, so the slot is
        // as much of the shape as the wing.
        m.save().move(0, 0, L * 0.3);
        m.soft((mm) => mm.loft([
          { z: -L * 0.12, pts: ring(R * 0.4, R * 0.24, seg) },
          { z: -L * 0.07, pts: ring(R * 1.12, R * 1.06, seg) },
          { z: L * 0.12, pts: ring(R * 1.12, R * 1.06, seg) },
          { z: L * 0.17, pts: ring(R * 0.4, R * 0.24, seg) },
        ], shade(P.white, 0.86), false));
        m.restore();
        m.both((mm) => {
          mm.save().move(R * 1.0, R * 0.1, L * 0.42);
          mm.slab([[0, 0], [1.35, -0.3], [1.35, -0.72], [0, -0.62]], 0.07, 0.024,
            shade(P.white, 0.94));
          mm.restore();
        });
        // Four tail fins with fillets, and the joins between the sections.
        finSet(m, { z: L * 0.045, r: R, n: 4, span: 0.34, chord: 0.55, tip: 0.3,
          sweep: 0.12, roll: 45, thick: R * 0.13, col: shade(P.white, 0.9) });
        [0.035, 0.2, 0.62].forEach((t) => {
          jointBand(m, { z: t * L, r: R, w: R * 0.34, seg, col: shade(P.white, 0.8) });
        });
        raceway(m, { roll: 22, r: R, h: R * 0.13, z0: L * 0.1, z1: L * 0.66,
          col: shade(P.white, 0.84) });
        // The ventral inlet. A real mouth with a lip and a duct behind it: this
        // is how a cruise missile breathes and it is the one detail that stops
        // the whole object reading as a rocket.
        intakeDuct(m, { x: 0, y: -R * 0.92, z: L * 0.24, w: R * 1.0, h: R * 0.66,
          len: L * 0.12, sharp: 2.6, col: shade(P.white, 0.8) });
        // The exhaust: the wall runs back UP the pipe to a dark disc, so the
        // hole reads as a hole rather than as a cap.
        m.save().move(0, 0, L * 0.006).rotX(180);
        m.soft((mm) => mm.loft([
          { z: 0, pts: ring(R * 0.36, R * 0.36, seg) },
          { z: R * 0.5, pts: ring(R * 0.4, R * 0.4, seg) },
        ], P.exhaust, false));
        m.save().move(0, 0, R * 0.5);
        m.soft((mm) => mm.loft([
          { z: 0, pts: ring(R * 0.34, R * 0.34, seg) },
          { z: -R * 0.7, pts: ring(R * 0.24, R * 0.24, seg) },
        ], shade(P.exhaust, 0.44), false));
        m.move(0, 0, -R * 0.7);
        m.fan(ring(R * 0.24, R * 0.24, seg).map((p) => [p[0], p[1], 0]), P.black);
        m.restore();
        m.restore();
        if (!m.far) {
          // The navigation antennas along the spine — a terrain-following radar
          // needs to see the ground and a satellite fix needs to see up — and
          // the access doors down the fuel section.
          m.save().move(0, R * 0.95, L * 0.72);
          m.slab([[-R * 0.42, -L * 0.05], [R * 0.42, -L * 0.05], [R * 0.42, L * 0.05],
            [-R * 0.42, L * 0.05]], R * 0.2, R * 0.06, P.sensor);
          m.restore();
          m.save().move(0, -R * 0.96, L * 0.82);
          m.slab([[-R * 0.3, -L * 0.03], [R * 0.3, -L * 0.03], [R * 0.3, L * 0.03],
            [-R * 0.3, L * 0.03]], R * 0.16, R * 0.05, P.glass);
          m.restore();
          [0.36, 0.44, 0.52, 0.6, 0.68, 0.76, 0.13, 0.19].forEach((t, i) => {
            skinPanel(m, { rx: R, ry: R, th: i % 2 ? 130 : 230, z: t * L,
              w: R * 0.6, len: L * 0.045, depth: R * 0.05, col: shade(P.white, 0.84) });
          });
          raceway(m, { roll: 200, r: R, h: R * 0.09, z0: L * 0.12, z1: L * 0.58,
            col: shade(P.white, 0.84) });
          // The wing-root blisters: the fairings over the hinge and the screw
          // jack that drives it, one each side, and the reason the wing has a
          // place to come out of.
          m.both((mm) => {
            mm.save().move(R * 0.86, R * 0.1, L * 0.42);
            mm.soft((s) => s.loft([
              { z: -L * 0.09, pts: ring(R * 0.04, R * 0.04, s.lod(8, 4)) },
              { z: -L * 0.04, pts: ring(R * 0.3, R * 0.22, s.lod(8, 4)) },
              { z: L * 0.05, pts: ring(R * 0.3, R * 0.22, s.lod(8, 4)) },
              { z: L * 0.09, pts: ring(R * 0.04, R * 0.04, s.lod(8, 4)) },
            ], shade(P.white, 0.88), false));
            mm.restore();
          });
        }
        m.restore();
        // The booster: its own case, its own fins and its own nozzle. It is a
        // rocket bolted to an aeroplane and it should look like one.
        m.soft((mm) => mm.loft([
          { z: 0.02, pts: ring(R * 0.78, R * 0.78, seg) },
          { z: 0.1, pts: ring(R * 0.92, R * 0.92, seg) },
          { z: BL - 0.06, pts: ring(R * 0.92, R * 0.92, seg) },
          { z: BL, pts: ring(R * 0.86, R * 0.86, seg) },
        ], shade(P.greyDark, 1.1), false));
        finSet(m, { z: 0.44, r: R * 0.9, n: 4, span: 0.3, chord: 0.4, tip: 0.2,
          sweep: 0.1, roll: 0, thick: R * 0.12, col: shade(P.greyDark, 1.2) });
        [0.1, BL - 0.06].forEach((z) => {
          jointBand(m, { z, r: R * 0.92, w: R * 0.3, k: 1.07, seg,
            col: shade(P.greyDark, 0.86) });
        });
        motorBell(m, { z: 0.36, r: R * 0.74, len: 0.36 });
      },
    },
    patriot: {
      name: "MIM-104 Patriot", cls: "Missile", span: 12.0,
      build(m) {
        // The launching station, not the missile: a semitrailer with the box at
        // the elevation it sits at when the battery is emplaced. A LAUNCHER IS
        // A TRAILER FIRST — the running gear gets the same treatment as the
        // ground kit, because at card size the wheel line and the four canister
        // mouths are the two things anybody reads.
        const seg = m.lod(20, 8);
        m.bar(-1.5, 1.5, 0.7, 1.0, 0, 9.2, shade(P.green, 0.8));
        m.both((mm) => {
          mm.bar(1.02, 1.36, 0.44, 0.76, 0.4, 8.4, shade(P.black, 1.45));
          for (let i = 0; i < 3; i += 1) {
            mm.bar(1.5, 1.62, 0.72, 1.24, 3.6 + i * 1.7, 4.9 + i * 1.7,
              shade(P.green, i % 2 ? 0.92 : 1.02));
          }
        });
        for (let i = 0; i < 4; i += 1) {
          const z = 0.6 + i * 2.6;
          m.bar(-1.2, 1.2, 0.5, 0.7, z - 0.14, z + 0.14, shade(P.black, 1.3));
        }
        wheels(m, { r: 0.6, w: 0.5, x: 1.55, axles: 2, z0: 1.1, z1: 2.4, detail: true });
        if (!m.far) {
          m.both((mm) => {
            for (let a = 0; a < 2; a += 1) {
              for (let k = 0; k < 4; k += 1) {
                mm.save().move(1.32, 0.6, 1.1 + a * 1.3).rotX(-54 + k * 36).move(0, 0.8, 0);
                mm.bar(-0.15, 0.15, 0, 0.06, -0.34, 0.34, shade(P.green, 0.76));
                mm.restore();
              }
            }
          });
          // The landing legs the trailer stands on with no tractor under it,
          // and the rear levelling jacks. A launching station is emplaced, and
          // what emplaces it is these four.
          m.both((mm) => jack(mm, { x: 1.2, y: 0.62, z: 7.9, r: 0.13, len: 0.62 }));
          m.both((mm) => jack(mm, { x: 1.35, y: 0.62, z: 0.5, r: 0.13, len: 0.62 }));
        }
        // The trunnion the launcher pivots on: two bearing housings, not a box.
        m.both((mm) => {
          mm.save().move(0.6, 1.5, 1.2).rotY(90);
          mm.soft((s) => s.tube(0.34, 0.34, 0.5, s.lod(14, 6), shade(P.green, 0.9)));
          mm.restore();
        });
        m.save().move(0, 1.0, 1.2).rotX(-38);
        // The launcher frame is a TRUSS, not a slab. Four longerons and the
        // bracing between them, with the canisters visible inside it, is what
        // a Patriot launcher actually looks like from any angle a card uses.
        lattice(m, { y: 1.05, z: 0.1, len: 6.1, w: 1.5, h: 1.02, t: 0.1, n: 5,
          col: shade(P.green, 0.82) });
        for (let i = 0; i < 4; i += 1) {
          const x = ((i % 2) - 0.5) * 1.5, y = 0.5 + Math.floor(i / 2) * 1.05;
          m.save().move(x, y, 0.2);
          // A Patriot canister is a rounded SQUARE section, not a tube — which
          // is what the superellipse in `bodyLoft` is for, and what tells this
          // apart from the round cold-launch cans two entries up.
          bodyLoft(m, {
            len: 5.9, w: 0.78, h: 0.78, col: shade(P.green, 1.06), seg, sharp: 6,
            stations: [[0, .96, .96], [.02, 1, 1], [.5, 1, 1], [.97, 1, 1], [1, .96, .96]],
          });
          if (!m.far) {
            for (let k = 0; k < 5; k += 1) {
              const z = 0.6 + k * 1.2;
              m.bar(-0.42, 0.42, -0.42, 0.42, z - 0.06, z + 0.06, shade(P.green, 0.84));
            }
          }
          // The frangible mouth cover, dished and dark: a sealed round shows a
          // lid, and this deck prices sealed rounds.
          m.save().move(0, 0, 5.9);
          m.soft((mm) => mm.tube(0.4, 0.4, 0.1, mm.lod(14, 7), shade(P.metal, 0.62)));
          m.move(0, 0, 0.1);
          m.fan(ring(0.4, 0.4, m.lod(14, 7)).map((p) => [p[0], p[1], 0]), shade(P.black, 1.7));
          m.restore();
          m.restore();
        }
        m.restore();
        // The two elevation rams, off the deck to the back of the frame.
        m.both((mm) => {
          mm.save().move(1.15, 1.0, 3.2).rotX(-64);
          mm.soft((s) => s.tube(0.16, 0.16, 1.5, s.lod(10, 5), shade(P.metal, 0.66)));
          mm.save().move(0, 0, 1.5);
          mm.soft((s) => s.tube(0.1, 0.1, 0.5, s.lod(8, 5), P.metal));
          mm.restore();
          mm.restore();
        });
        // The launcher electronics and the generator at the head of the
        // trailer, with the mast that carries the data link to the battery.
        m.save().move(0, 1.0, 8.6);
        m.bar(-1.2, 1.2, 0, 1.4, -0.9, 0.9, shade(P.green, 1.06));
        m.restore();
        if (!m.far) {
          m.save().move(0, 2.4, 8.6).rotX(-90);
          m.soft((mm) => mm.tube(0.09, 0.06, 2.4, mm.lod(8, 5), P.metal));
          m.restore();
          m.save().move(0.7, 2.4, 8.9);
          m.slab([[-0.34, -0.3], [0.34, -0.3], [0.34, 0.3], [-0.34, 0.3]], 0.12, 0.04, P.sensor);
          m.restore();
          m.save().move(-1.0, 1.4, 7.4).rotY(90);
          m.soft((mm) => mm.tube(0.3, 0.3, 0.4, mm.lod(12, 6), shade(P.green, 1.14)));
          m.restore();
        }
      },
    },
    jdam: {
      name: "JDAM Guidance Kit", cls: "Missile", span: 3.84,
      build(m) {
        // THE KIT IS THE TAIL, and the deck is pricing the kit rather than the
        // bomb it is bolted to — so the tail section, the two body strakes and
        // the satellite antenna behind them are the parts that have to read.
        // Everything forward of the kit is a plain iron bomb, and drawing it as
        // one is the point: a dumb weapon becomes a guided one by bolting this
        // on, and the card should show exactly which parts were bought.
        const L = 3.84, R = 0.23, seg = m.lod(52, 10);
        missile(m, {
          len: L, r: R, col: P.warhead, nose: 0.75, blunt: true, motor: false, seg, soft: true,
          stations: [[0, .86, .86], [.02, .94, .94], [.06, 1, 1], [.12, 1, 1], [.2, 1, 1],
            [.28, 1, 1], [.36, 1, 1], [.44, 1, 1], [.52, 1, 1], [.6, 1, 1], [.67, 1, 1],
            [.735, 1, 1], [.775, .99, .99], [.81, .965, .965], [.85, .915, .915],
            [.885, .86, .86], [.915, .79, .79], [.94, .71, .71], [.962, .62, .62],
            [.982, .5, .5], [1, .34, .34]],
        });
        // The tail kit itself: a boat-tailed control section in its own finish,
        // three fins on real actuators, and the ring where it clamps to the
        // bomb. The clamp ring is the seam a card can actually see.
        m.save().move(0, 0, -0.02);
        m.soft((mm) => mm.loft([
          { z: 0, pts: ring(R * 0.86, R * 0.86, seg) },
          { z: 0.08, pts: ring(R * 1.02, R * 1.02, seg) },
          { z: 0.56, pts: ring(R * 1.02, R * 1.02, seg) },
          { z: 0.66, pts: ring(R * 0.98, R * 0.98, seg) },
        ], shade(P.green, 1.1), false));
        m.restore();
        jointBand(m, { z: 0.62, r: R * 1.0, w: R * 0.4, k: 1.06, seg,
          col: shade(P.green, 0.82) });
        finSet(m, { z: 0.2, r: R * 1.02, n: 3, span: 0.42, chord: 0.55, tip: 0.32,
          sweep: 0.13, roll: 30, thick: R * 0.14, col: shade(P.green, 1.16) });
        // The strakes: two of them, full length, and they are the whole of the
        // difference between a bomb that flies straight and one that tumbles.
        m.both((mm) => {
          mm.save().move(R * 0.94, 0, 1.22);
          mm.slab([[0, 1.44], [0.12, 1.3], [0.12, -1.3], [0, -1.44]], 0.06, 0.02,
            shade(P.green, 1.05));
          mm.restore();
        });
        [0.05, 0.28, 0.74].forEach((t) => {
          jointBand(m, { z: t * L, r: R, w: R * 0.34, seg, col: shade(P.warhead, 0.82) });
        });
        // The suspension band the lugs are welded to, the lugs themselves, and
        // the GPS antenna behind them: that aerial is what makes the whole kit
        // worth its price, and it sits on the spine where it can see up.
        jointBand(m, { z: L * 0.44, r: R, w: R * 1.5, k: 1.03, seg,
          col: shade(P.metal, 0.72) });
        hoistLug(m, { z: L * 0.4, r: R, col: shade(P.metal, 0.7) });
        hoistLug(m, { z: L * 0.48, r: R, col: shade(P.metal, 0.7) });
        m.save().move(0, R * 0.96, 0.5);
        m.slab([[-R * 0.5, -0.22], [R * 0.5, -0.2], [R * 0.5, 0.2], [-R * 0.5, 0.22]],
          R * 0.28, R * 0.08, P.sensor);
        m.restore();
        if (m.far) return;
        // The umbilical the aircraft talks to the kit through, the tail
        // actuator bulges the fins turn on, and the fuze well in the nose.
        raceway(m, { roll: 150, r: R, h: R * 0.13, z0: 0.66, z1: L * 0.66,
          col: shade(P.warhead, 0.86) });
        for (let i = 0; i < 3; i += 1) {
          m.save().rotZ(30 + i * 120).move(0, R * 0.98, 0.3);
          m.soft((mm) => mm.loft([
            { z: -0.2, pts: ring(0.005, 0.005, mm.lod(8, 4)) },
            { z: -0.1, pts: ring(R * 0.3, R * 0.14, mm.lod(8, 4)) },
            { z: 0.1, pts: ring(R * 0.3, R * 0.14, mm.lod(8, 4)) },
            { z: 0.2, pts: ring(0.005, 0.005, mm.lod(8, 4)) },
          ], shade(P.green, 0.96), false));
          m.restore();
        }
        m.save().move(0, 0, L * 0.99);
        m.soft((mm) => mm.tube(R * 0.3, R * 0.24, R * 0.2, mm.lod(14, 6), P.sensor));
        m.restore();
        raceway(m, { roll: 212, r: R, h: R * 0.08, z0: 0.7, z1: L * 0.5,
          col: shade(P.warhead, 0.86) });
        [0.2, 0.33, 0.53, 0.6, 0.66, 0.72].forEach((t, i) => {
          skinPanel(m, { rx: R, ry: R, th: i % 2 ? 118 : 242, z: t * L,
            w: R * 0.5, len: L * 0.04, depth: R * 0.05, col: shade(P.warhead, 0.88) });
        });
      },
    },
    gbi: {
      name: "Ground-Based Interceptor", cls: "Missile", span: 16.8,
      build(m) {
        // Three stages of booster and, on the top of them, the part that costs
        // the money. THE KILL VEHICLE IS THE PRODUCT: everything below it is a
        // lift, and the card should spend its triangles accordingly — divert
        // thrusters that are holes, a telescope with a sun shade over a real
        // aperture, and an attitude cluster, rather than two grey cylinders.
        const L = 16.8, R = 0.66, seg = m.lod(34, 9);
        m.save().rotX(-90);
        missile(m, {
          len: L, r: R, col: P.white, nose: 1.4, seg, soft: true, motor: false,
          stations: [[0, .88, .88], [.012, .97, .97], [.03, 1, 1], [.16, 1, 1], [.3, 1, 1],
            [.41, 1, 1], [.425, .93, .93], [.44, .86, .86], [.58, .86, .86],
            [.715, .86, .86], [.728, .73, .73], [.742, .62, .62], [.84, .61, .61],
            [.9, .6, .6], [.93, .58, .58], [.955, .5, .5], [.98, .34, .34], [1, .17, .17]],
        });
        [0.032, 0.432, 0.735, 0.94].forEach((t) => {
          jointBand(m, { z: t * L, r: R * (t > 0.9 ? 0.58 : t > 0.5 ? 0.72 : t > 0.2 ? 0.93 : 1),
            w: R * 0.4, seg, col: shade(P.white, 0.78) });
        });
        raceway(m, { roll: 0, r: R, h: R * 0.13, z0: L * 0.04, z1: L * 0.42,
          col: shade(P.white, 0.84) });
        raceway(m, { roll: 0, r: R * 0.86, h: R * 0.1, z0: L * 0.45, z1: L * 0.72,
          col: shade(P.white, 0.84) });
        motorBell(m, { z: 0, r: R * 0.84, len: L * 0.055 });
        if (!m.far) {
          [0.08, 0.15, 0.22, 0.29, 0.36, 0.5, 0.55, 0.6, 0.65, 0.7].forEach((t, i) => {
            const rr = R * (t > 0.44 ? 0.86 : 1);
            skinPanel(m, { rx: rr, ry: rr, th: i % 2 ? 112 : 248, z: t * L,
              w: R * 0.6, len: L * 0.024, depth: R * 0.05, col: shade(P.white, 0.85) });
          });
          // Roll-control ports round each interstage, which is how a booster
          // holds its heading between burns.
          for (let i = 0; i < 4; i += 1) {
            m.save().rotZ(45 + i * 90).move(0, R * 0.84, L * 0.45).rotX(-90);
            m.soft((mm) => mm.tube(R * 0.1, R * 0.075, R * 0.16, mm.lod(9, 5), P.sensor));
            m.restore();
          }
        }
        // The kill vehicle.
        m.save().move(0, 0, L * 0.975);
        m.soft((mm) => mm.loft([
          { z: 0, pts: ring(0.38, 0.38, seg) },
          { z: 0.2, pts: ring(0.42, 0.42, seg) },
          { z: 0.74, pts: ring(0.42, 0.42, seg) },
          { z: 0.9, pts: ring(0.34, 0.34, seg) },
        ], P.sensor, false));
        // The telescope: a shade over an aperture, because a kill vehicle is a
        // camera that hits things and the camera has to be pointing somewhere.
        m.save().move(0, 0, 0.9);
        m.soft((mm) => mm.loft([
          { z: 0, pts: ring(0.24, 0.24, seg) },
          { z: 0.38, pts: ring(0.22, 0.22, seg) },
          { z: 0.5, pts: ring(0.28, 0.28, seg) },
        ], shade(P.metal, 0.7), false));
        m.save().move(0, 0, 0.5);
        m.soft((mm) => mm.loft([
          { z: 0, pts: ring(0.26, 0.26, seg) },
          { z: -0.22, pts: ring(0.19, 0.19, seg) },
        ], shade(P.glass, 0.7), false));
        m.move(0, 0, -0.22);
        m.fan(ring(0.19, 0.19, seg).map((p) => [p[0], p[1], 0]), P.glass);
        m.restore();
        m.restore();
        if (!m.far) {
          // Four divert thrusters round the waist and two attitude clusters
          // above them: this is the whole of how the thing steers, and they are
          // holes with lips, which is the only way this file draws a hole.
          for (let i = 0; i < 4; i += 1) {
            m.save().rotZ(i * 90).move(0, 0.42, 0.44).rotX(-90);
            m.soft((mm) => mm.tube(0.11, 0.14, 0.12, mm.lod(10, 5), shade(P.exhaust, 1.1)));
            m.move(0, 0, 0.12);
            m.fan(ring(0.14, 0.14, m.lod(10, 5)).map((p) => [p[0], p[1], 0]), P.black);
            m.restore();
            m.save().rotZ(45 + i * 90).move(0, 0.4, 0.78).rotX(-90);
            m.soft((mm) => mm.tube(0.05, 0.06, 0.08, mm.lod(8, 4), shade(P.exhaust, 1.2)));
            m.restore();
          }
          // The propellant tanks strapped round the bus, which is why a kill
          // vehicle is wider at the waist than at either end.
          for (let i = 0; i < 3; i += 1) {
            m.save().rotZ(60 + i * 120).move(0, 0.44, 0.34);
            m.soft((mm) => mm.ball(0.13, mm.lod(10, 5), mm.lod(6, 3), shade(P.gold, 1.05)));
            m.restore();
          }
        }
        m.restore();
        m.restore();
        // The silo the round is leaving: a collar, a lip and a bore with a
        // depth to it. A hole drawn as a disc is a manhole cover.
        m.save().rotX(-90);
        m.soft((mm) => mm.loft([
          { z: -0.35, pts: ring(2.4, 2.4, seg) },
          { z: 0.1, pts: ring(2.1, 2.1, seg) },
          { z: 0.42, pts: ring(0.92, 0.92, seg) },
        ], shade(P.metal, 0.5), false));
        m.soft((mm) => mm.loft([
          { z: 0.42, pts: ring(0.92, 0.92, seg) },
          { z: -0.1, pts: ring(0.8, 0.8, seg) },
          { z: -0.95, pts: ring(0.78, 0.78, seg) },
        ], shade(P.black, 1.5), false));
        m.save().move(0, 0, -0.95);
        m.fan(ring(0.78, 0.78, seg).map((p) => [p[0], p[1], 0]), P.black);
        m.restore();
        m.restore();
      },
    },
    x51: {
      name: "Scramjet Test Vehicle", cls: "Missile", span: 7.6,
      build(m) {
        // A waverider on its booster, which is the only way one of these has
        // ever flown: the test article is the front four metres. NOTHING HERE
        // IS SMOOTHED, and that is the same decision the F-117 makes for the
        // same reason — a waverider is a set of flat compression surfaces that
        // exist to put a shock exactly where the inlet is, and a rounded one is
        // not a waverider. Detail is bought by SUBDIVIDING the facets and by
        // the thermal tiles over them, never by softening either.
        const BL = 3.3;
        facetBody(m, {
          len: 4.3, w: 0.66, h: 0.6, col: shade(P.white, 0.86),
          stations: [[0, .9, .9], [.06, .96, .97], [.12, 1, 1], [.24, 1, 1], [.36, 1, 1],
            [.48, 1, 1], [.62, 1, 1], [.72, .9, .86], [.86, .7, .66], [.94, .44, .4],
            [1, .18, .16]],
        });
        // The scramjet: a cowl with a real lip, an isolator behind it and a
        // duct that goes somewhere dark. A hypersonic test vehicle whose engine
        // is a black rectangle is a model of the fairing, not of the engine.
        intakeDuct(m, { x: 0, y: -0.28, z: 2.55, w: 0.52, h: 0.24, len: 0.95,
          sharp: 4.0, col: shade(P.white, 0.8) });
        m.save().move(0, -0.34, 0.9);
        m.bar(-0.3, 0.3, -0.2, 0.06, -0.6, 0.72, P.sensor);
        // The nozzle ramp: the aft end of a scramjet is the vehicle's own
        // underside, and the step where it starts is the whole engine cycle.
        m.save().move(0, -0.2, -0.62).rotX(9);
        m.slab([[-0.26, -0.5], [0.26, -0.5], [0.3, 0.5], [-0.3, 0.5]], 0.08, 0.026,
          shade(P.exhaust, 1.15));
        m.restore();
        m.restore();
        // Tail: one fin above and two below, all flat plates on a flat body.
        m.save().move(0, 0.26, 0.5).rotZ(90);
        m.slab([[0, 1.0], [0.7, 0.2], [0.7, -0.2], [0, -0.5]], 0.08, 0.026,
          shade(P.white, 1.05));
        m.restore();
        m.both((mm) => {
          mm.save().move(0.24, -0.2, 0.45).rotZ(-118);
          mm.slab([[0, 0.6], [0.42, 0.16], [0.42, -0.1], [0, -0.34]], 0.06, 0.02,
            shade(P.white, 0.94));
          mm.restore();
        });
        if (!m.far) {
          // The thermal protection. A vehicle that flies at Mach 5 is tiled,
          // the tiles are laid in courses with a gap at every seam, and in a
          // renderer with no textures that field of chamfered plates is the
          // only surface texture this object will ever have.
          m.save().move(0, -0.219, 0);
          facetMosaic(m, { rows: 10, cols: 4, t: 0.014, gap: 0.16, col: shade(P.white, 0.78),
            corners: [[-0.29, 0.2], [0.29, 0.2], [0.2, 3.9], [-0.2, 3.9]] });
          m.restore();
          m.save().move(0, 0.2, 0);
          facetMosaic(m, { rows: 7, cols: 3, t: 0.014, gap: 0.16, col: shade(P.white, 0.9),
            corners: [[-0.2, 0.35], [0.2, 0.35], [0.13, 3.7], [-0.13, 3.7]] });
          m.restore();
          // The compression flanks between the chine and the belly carry their
          // own courses, laid the long way because that is the direction the
          // heat runs.
          m.both((mm) => {
            mm.save().move(0.235, -0.09, 0).rotZ(-38);
            facetMosaic(mm, { rows: 8, cols: 1, t: 0.012, gap: 0.18,
              col: shade(P.white, 0.84),
              corners: [[-0.075, 0.3], [0.075, 0.3], [0.055, 3.8], [-0.055, 3.8]] });
            mm.restore();
          });
          // The leading-edge strips down both chines: carbon, darker than the
          // skin, and the hottest thing on the aeroplane.
          m.both((mm) => {
            mm.save().move(0.3, -0.02, 0).rotY(-4);
            mm.slab([[-0.03, 0.4], [0.03, 0.5], [0.03, 3.85], [-0.03, 3.9]], 0.05, 0.016,
              shade(P.stealth, 1.4));
            mm.restore();
          });
          // Instrumentation ports along the flank: this is a TEST vehicle, and
          // what makes it one is that it is covered in sensors.
          for (let i = 0; i < 8; i += 1) {
            m.both((mm) => {
              mm.save().move(0.31, 0.02, 0.55 + i * 0.42).rotZ(90);
              mm.slab([[-0.05, -0.07], [0.05, -0.07], [0.05, 0.07], [-0.05, 0.07]],
                0.05, 0.016, i % 2 ? P.sensor : shade(P.metal, 0.8));
              mm.restore();
            });
          }
        }
        // The booster, and the interstage that mates a round motor to a flat
        // aeroplane. Smoothed, because unlike everything above it a solid
        // rocket motor IS a cylinder.
        const seg = m.lod(30, 8);
        m.save().move(0, -0.05, -BL);
        m.soft((mm) => mm.loft([
          { z: 0.05, pts: ring(0.36, 0.36, seg) },
          { z: 0.2, pts: ring(0.42, 0.42, seg) },
          { z: BL - 0.5, pts: ring(0.42, 0.42, seg) },
          { z: BL - 0.16, pts: ring(0.42, 0.42, seg) },
          { z: BL, pts: ring(0.38, 0.38, seg) },
        ], P.greyDark, false));
        motorBell(m, { z: 0.5, r: 0.38, len: 0.5 });
        [0.4, BL * 0.5, BL - 0.3].forEach((z) => {
          jointBand(m, { z, r: 0.42, w: 0.14, k: 1.06, seg, col: shade(P.greyDark, 0.86) });
        });
        finSet(m, { z: 1.0, r: 0.42, n: 4, span: 0.6, chord: 1.0, tip: 0.5, sweep: 0.42,
          roll: 45, thick: 0.07, col: shade(P.greyDark, 1.1) });
        m.restore();
        // The interstage adapter: round at the motor, flat at the aeroplane.
        m.save().move(0, -0.05, -0.5);
        m.soft((mm) => mm.loft([
          { z: 0, pts: ring(0.38, 0.38, seg) },
          { z: 0.26, pts: ringSuper(0.36, 0.3, seg, 3.2) },
          { z: 0.5, pts: ringSuper(0.34, 0.28, seg, 4.5) },
        ], shade(P.greyDark, 1.2), false));
        m.restore();
      },
    },
    hgv: {
      name: "Hypersonic Glide Vehicle", cls: "Missile", span: 5.0,
      build(m) {
        // A flat-bottomed cone that rides its own shock. No motor, four flaps,
        // and a nose radius small enough to be the hard part of the programme.
        // FLAT, LIKE THE X-51 AND FOR THE SAME REASON: the compression surface
        // is the vehicle, and a smoothed one is a re-entry capsule. What buys
        // the detail here is the thermal protection — a glide body spends its
        // whole flight inside its own shock layer, so it is tiled everywhere,
        // and a field of chamfered courses is the only surface texture a
        // renderer with no textures can have.
        const S = [[0, 1.0], [0.45, 0.995], [1.2, 0.93], [1.9, 0.85], [2.6, 0.72],
          [3.2, 0.545], [3.75, 0.395], [4.25, 0.25], [4.6, 0.15], [4.85, 0.08],
          [5.0, 0.032]];
        m.loft(S.map((s) => ({ z: s[0],
          pts: pent(1.15 * s[1], -0.34 * s[1], 0.24 * s[1], 0.28 * s[1]) })),
        shade(P.stealth, 1.3));
        // The nose cap: carbon-carbon, darker than the skin, and a separate
        // part on every real one because it is the part that erodes.
        m.loft([
          { z: 4.6, pts: pent(1.15 * 0.155, -0.34 * 0.155, 0.24 * 0.155, 0.28 * 0.155) },
          { z: 4.85, pts: pent(1.15 * 0.083, -0.34 * 0.083, 0.24 * 0.083, 0.28 * 0.083) },
          { z: 5.02, pts: pent(1.15 * 0.03, -0.34 * 0.03, 0.24 * 0.03, 0.28 * 0.03) },
        ], shade(P.black, 1.5), false);
        // The four control flaps, on real hinge gaps, each with the actuator
        // fairing that drives it. A glide vehicle steers with nothing else.
        m.both((mm) => {
          mm.save().move(0.5, -0.3, 0.02).rotX(14);
          mm.slab([[0, 0.04], [0.62, 0.04], [0.62, -0.72], [0, -0.74]], 0.07, 0.024, P.sensor);
          mm.restore();
          mm.save().move(1.06, 0.02, 0.6).rotZ(64);
          mm.slab([[0, 0.5], [0.5, 0.2], [0.5, -0.3], [0, -0.6]], 0.07, 0.024,
            shade(P.stealth, 1.5));
          mm.restore();
          // The hinge fairings stay at BOTH levels of detail. They are the step
          // between the body and the flap, so they are what stops a map pin
          // from reading as one continuous wedge.
          mm.save().move(0.5, -0.24, 0.16);
          mm.bar(-0.09, 0.09, -0.06, 0.06, -0.14, 0.34, shade(P.metal, 0.7));
          mm.restore();
          mm.save().move(0.92, 0.06, 0.72);
          mm.bar(-0.08, 0.08, -0.07, 0.07, -0.16, 0.3, shade(P.metal, 0.7));
          mm.restore();
        });
        // The base: a recessed plate rather than a disc, so the aft end reads
        // as the open end of a structure and not as a lid.
        m.save().move(0, -0.05, 0);
        m.soft((mm) => mm.tube(0.52, 0.5, 0.06, mm.lod(16, 8), P.black));
        m.restore();
        m.save().move(0, -0.05, 0.12);
        m.soft((mm) => mm.loft([
          { z: 0, pts: ring(0.46, 0.3, mm.lod(16, 8)) },
          { z: -0.18, pts: ring(0.34, 0.22, mm.lod(16, 8)) },
        ], shade(P.black, 1.3), false));
        m.restore();
        // The chine leading edges stay at BOTH levels of detail. They are the
        // hottest line on the vehicle and its own material, and they are also
        // the two lines that give a map pin its planform.
        m.both((mm) => {
          mm.save().move(1.14, -0.05, 0).rotZ(-14);
          mm.slab([[-0.05, 0.1], [0.05, 0.1], [0.05, 3.9], [-0.05, 3.9]], 0.07, 0.022,
            shade(P.black, 1.7));
          mm.restore();
        });
        if (m.far) return;
        // Reaction control ports round the base: above the atmosphere the flaps
        // do nothing and these are what holds the attitude.
        for (let i = 0; i < 6; i += 1) {
          m.save().rotZ(30 + i * 60).move(0, 0.4, 0.34).rotX(-90);
          m.soft((mm) => mm.tube(0.05, 0.04, 0.07, mm.lod(8, 4), P.sensor));
          m.restore();
        }
        // The thermal courses. Belly first, because it is the surface that
        // faces the flow and the one a card sees from any angle above it.
        m.save().move(0, -0.352, 0);
        facetMosaic(m, { rows: 13, cols: 5, t: 0.02, gap: 0.14, col: shade(P.stealth, 1.16),
          corners: [[-1.08, 0.12], [1.08, 0.12], [0.42, 3.9], [-0.42, 3.9]] });
        m.restore();
        m.save().move(0, 0.25, 0);
        facetMosaic(m, { rows: 10, cols: 4, t: 0.02, gap: 0.14, col: shade(P.stealth, 1.42),
          corners: [[-0.68, 0.2], [0.68, 0.2], [0.26, 3.8], [-0.26, 3.8]] });
        m.restore();
        m.both((mm) => {
          mm.save().move(0.92, -0.05, 0).rotZ(-42);
          facetMosaic(mm, { rows: 10, cols: 1, t: 0.018, gap: 0.16,
            col: shade(P.stealth, 1.28),
            corners: [[-0.14, 0.15], [0.14, 0.15], [0.06, 3.85], [-0.06, 3.85]] });
          mm.restore();
        });
      },
    },
    owa: {
      name: "One-Way Attack Drone", cls: "Missile", span: 2.5,
      build(m) {
        // CHEAP ENOUGH TO BE EXPENDABLE IS THE WHOLE DESIGN, and the way that
        // shows is not by drawing less of it — it is by drawing what a cheap
        // aeroplane is made of. A delta wing with a real aerofoil and two
        // elevons on hinge gaps, a warhead in the nose behind a joint band, a
        // piston engine with cooling fins and an exhaust pipe pushing a
        // two-blade propeller, a satellite aerial on the spine, and the rocket
        // bottle that throws it off its rail. Every one of those is a thing
        // somebody bolted on in a shed, and together they are why this costs
        // what a car costs and not what a cruise missile costs.
        const L = 3.5, seg = m.lod(34, 9);
        m.both((mm) => {
          mm.liftSurface({
            x0: 0.24, y: 0, z: L * 0.92, span: 1.0, root: L * 0.9, tip: 0.49,
            sweep: L * 0.8, thick: 0.045, ctrl: 0.16,
            surfaces: [[0.1, 0.5, 3], [0.55, 0.95, -4]],
          }, shade(P.cloth, 0.9));
          // The winglet, which on this airframe is a fin: it is all the
          // directional stability a tailless delta has.
          mm.save().move(1.22, 0.0, L * 0.06).rotZ(76);
          mm.slab([[0, 0.42], [0.42, 0.2], [0.42, -0.16], [0, -0.28]], 0.06, 0.02,
            shade(P.cloth, 1.1));
          mm.restore();
        });
        bodyLoft(m, {
          len: L, w: 0.5, h: 0.42, col: shade(P.cloth, 1.05), seg, soft: true,
          stations: [[0, .5, .5], [.04, .74, .74], [.08, .9, .9], [.16, .96, .96],
            [.26, 1, 1], [.38, 1, 1], [.5, 1, 1], [.62, .98, .99], [.74, .9, .95],
            [.86, .8, .86], [.94, .55, .64], [1, .24, .3]],
        });
        // The wing-body fairing. A wing that meets a fuselage at a hard corner
        // is a wing pushed through a tube; the fillet is the two seconds of
        // glass tape that makes it an aeroplane, and it is on the real one.
        m.both((mm) => {
          mm.save().move(0.24, -0.02, 0);
          mm.soft((s) => s.loft([
            { z: 0.1, pts: ring(0.004, 0.004, s.lod(10, 5)) },
            { z: 0.5, pts: ring(0.1, 0.09, s.lod(10, 5)) },
            { z: 2.1, pts: ring(0.1, 0.09, s.lod(10, 5)) },
            { z: 3.2, pts: ring(0.004, 0.004, s.lod(10, 5)) },
          ], shade(P.cloth, 0.96), false));
          mm.restore();
        });
        // Where the warhead section bolts to the airframe, and the impact fuze
        // probe on the nose. Both are the difference between this and a target
        // drone, which is otherwise the same aeroplane.
        jointBand(m, { z: L * 0.78, r: 0.25 * 0.86, w: 0.1, k: 1.07, seg,
          col: shade(P.cloth, 0.82) });
        boom(m, { x: 0, y: 0.02, z: L * 0.995, r: 0.018, len: 0.16 });
        // The engine, hung on the back: crankcase, a finned cylinder standing
        // up out of it, the exhaust down the flank, and the propeller.
        m.save().move(0, 0.02, -0.16);
        m.soft((mm) => mm.loft([
          { z: 0, pts: ring(0.11, 0.11, mm.lod(16, 7)) },
          { z: 0.1, pts: ring(0.15, 0.15, mm.lod(16, 7)) },
          { z: 0.36, pts: ring(0.15, 0.15, mm.lod(16, 7)) },
          { z: 0.44, pts: ring(0.12, 0.12, mm.lod(16, 7)) },
        ], shade(P.metal, 0.6), false));
        m.restore();
        if (!m.far) {
          m.save().move(0, 0.16, 0.06).rotX(-78);
          m.soft((mm) => mm.tube(0.085, 0.08, 0.2, mm.lod(14, 7), shade(P.metal, 0.5)));
          // The cooling fins. An air-cooled two-stroke is a stack of discs and
          // nothing else in this deck looks remotely like it.
          for (let k = 0; k < 5; k += 1) {
            jointBand(m, { z: 0.05 + k * 0.035, r: 0.084, w: 0.016, k: 1.5,
              seg: m.lod(14, 7), col: shade(P.metal, 0.72) });
          }
          m.save().move(0, 0, 0.2);
          m.soft((mm) => mm.tube(0.075, 0.06, 0.06, mm.lod(12, 6), shade(P.metal, 0.8)));
          m.restore();
          m.restore();
          // The exhaust: a bent pipe and a can, down the right flank.
          m.save().move(0.14, 0.0, 0.1).rotY(-24);
          m.soft((mm) => mm.tube(0.035, 0.035, 0.34, mm.lod(10, 5), P.exhaust));
          m.restore();
          m.save().move(0.2, -0.02, 0.42).rotY(-12);
          m.soft((mm) => mm.tube(0.055, 0.045, 0.3, mm.lod(12, 6), shade(P.exhaust, 1.2)));
          m.restore();
        }
        // The propeller: two blades with an aerofoil section and pitch, and a
        // spinner. A pusher drawn with flat paddles reads as a toy.
        m.save().move(0, 0.02, -0.2);
        m.soft((mm) => mm.ball(0.075, mm.lod(12, 6), mm.lod(6, 3), shade(P.black, 1.6)));
        m.restore();
        for (let i = 0; i < 2; i += 1) {
          m.save().move(0, 0.02, -0.2).rotZ(i * 180).rotY(180);
          m.foil({ x0: 0.05, y: 0, z: 0, span: 0.44, root: 0.14, tip: 0.07,
            sweep: 0.03, thick: 0.12, incidence: 26, n: m.lod(7, 3), bays: m.lod(4, 1) },
          shade(P.black, 1.5));
          m.restore();
        }
        if (m.far) return;
        // The aerial farm: a satellite navigation array on the spine, the
        // command link blade under it, and the access hatches down both
        // flanks — which on the real thing are plywood, screwed on.
        m.save().move(0, 0.22, L * 0.44);
        m.slab([[-0.11, -0.11], [0.11, -0.11], [0.11, 0.11], [-0.11, 0.11]], 0.05, 0.016,
          P.sensor);
        m.restore();
        for (let i = 0; i < 4; i += 1) {
          m.save().rotZ(i * 90 + 45).move(0, 0.235, L * 0.44);
          m.bar(-0.026, 0.026, 0, 0.02, -0.026, 0.026, shade(P.metal, 0.9));
          m.restore();
        }
        bladeAerial(m, { y: -0.21, z: L * 0.56, h: 0.09, len: 0.12, roll: 180,
          col: shade(P.cloth, 0.7) });
        [0.22, 0.3, 0.42, 0.56, 0.68, 0.74].forEach((t, i) => {
          skinPanel(m, { rx: 0.25, ry: 0.21, th: i % 2 ? 118 : 242, z: t * L,
            w: 0.16, len: 0.2, depth: 0.014, col: shade(P.cloth, 0.9) });
        });
        skinPanel(m, { rx: 0.25, ry: 0.21, th: 270, z: L * 0.5,
          w: 0.2, len: 0.5, depth: 0.016, col: shade(P.cloth, 0.84) });
        // The rail lugs and the rocket bottle that throws it off the ramp. This
        // aeroplane has no undercarriage and never lands; the bottle is how it
        // gets airborne and it is part of what the deck is buying.
        m.both((mm) => {
          mm.save().move(0.1, -0.2, L * 0.5);
          mm.bar(-0.03, 0.03, -0.07, 0, -0.3, 0.3, shade(P.metal, 0.6));
          mm.restore();
        });
        m.save().move(0, -0.34, 0.28);
        m.soft((mm) => mm.loft([
          { z: 0, pts: ring(0.1, 0.1, mm.lod(16, 7)) },
          { z: 0.08, pts: ring(0.13, 0.13, mm.lod(16, 7)) },
          { z: 1.0, pts: ring(0.13, 0.13, mm.lod(16, 7)) },
          { z: 1.1, pts: ring(0.1, 0.1, mm.lod(16, 7)) },
        ], shade(P.greyDark, 1.05), false));
        [0.2, 0.55, 0.9].forEach((z) => {
          jointBand(m, { z, r: 0.13, w: 0.05, k: 1.08, seg: m.lod(16, 7),
            col: shade(P.greyDark, 0.84) });
        });
        finSet(m, { z: 0.16, r: 0.13, n: 4, span: 0.11, chord: 0.16, tip: 0.09,
          sweep: 0.04, roll: 45, thick: 0.02, col: shade(P.greyDark, 1.16) });
        motorBell(m, { z: 0.04, r: 0.1, len: 0.14 });
        m.restore();
      },
    },

    // -------------------------------------------------------------- infantry
    raven: {
      name: "RQ-11 Raven", cls: "Infantry", span: 1.4,
      build(m) {
        // A 1.9 kg AEROPLANE, and everything on it is at 1.9 kg scale: a foam
        // fuselage that breaks into four pieces to go in a rucksack, a straight
        // wing on a carbon spar, a V-tail with two ruddervators, a pusher motor
        // and a folding propeller, and a nose gimbal that is most of the price.
        // The old model was a lofted pod with four flat plates; what makes this
        // read as an aircraft rather than a dart is the aerofoil sections, the
        // break lines the pieces actually part on, and the camera.
        const L = 0.92, seg = m.lod(24, 8);
        bodyLoft(m, {
          len: L, w: 0.105, h: 0.125, col: shade(P.cloth, 1.3), seg, soft: true,
          stations: m.far
            ? [[0, .4, .4], [.1, .8, .8], [.55, 1, 1], [.86, .9, .9], [1, .5, .55]]
            : [[0, .34, .34], [.04, .58, .58], [.1, .8, .8], [.18, .92, .92],
              [.28, .98, .98], [.4, 1, 1], [.52, 1, 1], [.62, .99, 1],
              [.72, .96, .98], [.82, .92, .94], [.9, .84, .88], [.96, .68, .74],
              [1, .46, .52]],
        });
        // The wing. A straight constant-chord panel with real aerofoil section
        // and an aileron on a hinge gap — a Raven's wing is a foam plank, but a
        // foam plank still has a leading edge that catches light and a flat
        // plate does not.
        m.both((mm) => {
          mm.liftSurface({
            x0: 0.045, y: 0.042, z: L * 0.62, span: 0.62, root: 0.165, tip: 0.135,
            sweep: 0.018, thick: 0.11, ctrl: 0.26,
            surfaces: [[0.35, 0.92, 3]], n: mm.lod(11, 3), bays: mm.lod(6, 1),
          }, shade(P.cloth, 1.24));
          // The V-tail, standing at 35 degrees off vertical, each panel with its
          // own ruddervator: this aircraft has no rudder and no elevator, only
          // these two surfaces, and that is what the V is FOR.
          mm.liftSurface({
            x0: 0.02, y: 0.022, z: L * 0.15, span: 0.24, root: 0.14, tip: 0.1,
            sweep: 0.05, thick: 0.1, dihedral: 55, ctrl: 0.34,
            surfaces: [[0.08, 0.94, -4]], n: mm.lod(9, 3), bays: mm.lod(4, 1),
          }, shade(P.cloth, 1.24));
        });
        // The camera turret under the nose. This is the payload and it is what
        // the money buys: a gimbal with a forward window and a side window,
        // which is exactly the two the real one carries.
        gimbalBall(m, { y: -0.03, z: L * 0.74, r: 0.055, col: shade(P.sensor, 1.1),
          apertures: [[0, 8, 0.42], [78, 0, 0.3, shade(P.glass, 1.2)]] });
        // The pusher motor and its folding two-blade propeller, on the tail.
        m.save().move(0, 0.018, -0.055);
        m.soft((mm) => mm.loft([
          { z: 0, pts: ring(0.019, 0.019, mm.lod(12, 6)) },
          { z: 0.012, pts: ring(0.026, 0.026, mm.lod(12, 6)) },
          { z: 0.055, pts: ring(0.026, 0.026, mm.lod(12, 6)) },
          { z: 0.07, pts: ring(0.02, 0.02, mm.lod(12, 6)) },
        ], shade(P.metal, 0.62), false));
        m.restore();
        m.save().move(0, 0.018, -0.062);
        m.soft((mm) => mm.ball(0.021, mm.lod(12, 6), mm.lod(5, 3), shade(P.black, 1.5)));
        m.restore();
        for (let i = 0; i < 2; i += 1) {
          m.save().move(0, 0.018, -0.062).rotZ(i * 180 + 14).rotY(180);
          m.foil({ x0: 0.012, y: 0, z: 0, span: 0.16, root: 0.038, tip: 0.02,
            sweep: 0.012, thick: 0.13, incidence: 24, n: m.lod(9, 3), bays: m.lod(5, 1) },
          shade(P.black, 1.45));
          m.restore();
        }
        if (m.far) return;
        // THE BREAK LINES ARE THE AIRCRAFT. A Raven comes out of the bag in
        // pieces and clips together in the field, and the joints where the nose,
        // the payload bay and the tail boom meet are the only surface detail a
        // foam airframe has. Two raised collars and the wing saddle across the
        // spine.
        [0.3, 0.62].forEach((t) => {
          jointBand(m, { z: L * t, r: 0.055, w: 0.016, k: 1.09, seg,
            col: shade(P.cloth, 1.12) });
        });
        m.save().move(0, 0.052, L * 0.55);
        m.slab([[-0.05, -0.075], [0.05, -0.075], [0.05, 0.075], [-0.05, 0.075]], 0.022, 0.007,
          shade(P.cloth, 1.16));
        m.restore();
        // The spar the wing panels slide onto, showing where they part company
        // with the fuselage, and the fillet where each panel meets the pod. A
        // wing pushed through a tube at a hard corner is a wing pushed through
        // a tube; the fillet is what makes the two one aeroplane.
        m.both((mm) => {
          mm.save().move(0.045, 0.042, L * 0.55).rotY(90);
          mm.soft((s) => s.tube(0.006, 0.006, 0.06, s.lod(8, 4), shade(P.black, 1.4), false));
          mm.restore();
          mm.save().move(0.048, 0.036, L * 0.62);
          mm.soft((s) => s.loft([
            { z: 0.03, pts: ring(0.0015, 0.0015, s.lod(10, 5)) },
            { z: -0.02, pts: ring(0.014, 0.012, s.lod(10, 5)) },
            { z: -0.12, pts: ring(0.014, 0.012, s.lod(10, 5)) },
            { z: -0.18, pts: ring(0.0015, 0.0015, s.lod(10, 5)) },
          ], shade(P.cloth, 1.2), false));
          mm.restore();
          mm.save().move(0.022, 0.02, L * 0.15).rotZ(-55);
          mm.soft((s) => s.loft([
            { z: 0.02, pts: ring(0.0015, 0.0015, s.lod(10, 5)) },
            { z: -0.02, pts: ring(0.012, 0.01, s.lod(10, 5)) },
            { z: -0.1, pts: ring(0.012, 0.01, s.lod(10, 5)) },
            { z: -0.15, pts: ring(0.0015, 0.0015, s.lod(10, 5)) },
          ], shade(P.cloth, 1.2), false));
          mm.restore();
          // The servo fairing on each wing panel, over the aileron horn.
          mm.save().move(0.3, 0.03, L * 0.56);
          mm.slab([[-0.016, -0.03], [0.016, -0.03], [0.016, 0.03], [-0.016, 0.03]],
            0.016, 0.005, shade(P.cloth, 1.1));
          mm.restore();
        });
        jointBand(m, { z: L * 0.86, r: 0.05, w: 0.014, k: 1.1, seg,
          col: shade(P.cloth, 1.06) });
        m.save().move(0, -0.058, L * 0.44);
        m.slab([[-0.036, -0.09], [0.036, -0.09], [0.036, 0.09], [-0.036, 0.09]], 0.014, 0.005,
          shade(P.cloth, 1.18));
        m.restore();
        boltRun(m, { n: 4, r: 0.0055, x0: -0.03, x1: 0.03, y0: -0.062, z0: L * 0.36,
          pitch: 90, col: shade(P.greyDark, 1.1) });
        // Aerials and the air-data probe: the command-link blade under the
        // fuselage, the satellite-navigation patch on the spine, and the pitot.
        bladeAerial(m, { y: -0.06, z: L * 0.24, h: 0.05, len: 0.06, roll: 180,
          col: shade(P.cloth, 0.78) });
        m.save().move(0, 0.062, L * 0.68);
        m.slab([[-0.022, -0.022], [0.022, -0.022], [0.022, 0.022], [-0.022, 0.022]],
          0.012, 0.004, P.sensor);
        m.restore();
        boom(m, { x: 0, y: 0.03, z: L * 0.995, r: 0.005, len: 0.075 });
        // The hand grip moulded under the wing centre section. This aircraft is
        // thrown, and the grip is the one feature that says so.
        m.save().move(0, -0.05, L * 0.6);
        m.slab([[-0.016, -0.05], [0.016, -0.05], [0.016, 0.05], [-0.016, 0.05]], 0.04, 0.012,
          shade(P.cloth, 0.86));
        m.restore();
      },
    },
    switchblade: {
      name: "Switchblade Loitering Munition", cls: "Infantry", span: 1.2,
      build(m) {
        // DEPLOYED, BESIDE ITS TUBE. Folded it is a tube with a tube in it and
        // the picture that says what the money bought is the wings out — so
        // both states are on the card at once, which is also how the thing is
        // photographed. The round is a 2.5 kg aeroplane with a warhead in the
        // nose: the wings pop out of slots in the body, the tail fins pop out
        // of the aft ring, and the propeller unfolds behind them.
        const L = 0.5, R = 0.037, seg = m.lod(26, 9);
        bodyLoft(m, {
          len: L, w: R * 2, h: R * 2, col: P.sensor, seg, soft: true,
          stations: m.far
            ? [[0, .8, .8], [.04, 1, 1], [.76, 1, 1], [.9, .78, .78], [1, .34, .34]]
            : [[0, .62, .62], [.02, .82, .82], [.05, 1, 1], [.14, 1, 1], [.26, 1, 1],
              [.38, 1, 1], [.5, 1, 1], [.62, 1, 1], [.72, 1, 1], [.8, .99, .99],
              [.87, .92, .92], [.93, .78, .78], [.97, .58, .58], [1, .3, .3]],
        });
        // The wings, swept forward out of their slots as they sit when they
        // unfold, with a real aerofoil section. The hinge fairing at each root
        // is the slot they came out of and is what makes them read as folding
        // rather than as glued on.
        m.both((mm) => {
          mm.foil({ x0: R * 0.9, y: 0.008, z: L * 0.62, span: 0.27, root: 0.085, tip: 0.05,
            sweep: -0.03, thick: 0.09, dihedral: 4, n: mm.lod(11, 3), bays: mm.lod(6, 1) },
          shade(P.sensor, 1.5));
          if (mm.far) return;
          mm.save().move(R * 0.82, 0.006, L * 0.62);
          mm.soft((s) => s.loft([
            { z: 0.05, pts: ring(0.0015, 0.0015, s.lod(9, 5)) },
            { z: 0.0, pts: ring(0.009, 0.007, s.lod(9, 5)) },
            { z: -0.07, pts: ring(0.009, 0.007, s.lod(9, 5)) },
            { z: -0.11, pts: ring(0.0015, 0.0015, s.lod(9, 5)) },
          ], shade(P.sensor, 1.3), false));
          mm.restore();
        });
        // The four tail fins with root fillets, on the aft ring.
        finSet(m, { z: L * 0.2, r: R, n: 4, span: 0.062, chord: 0.075, tip: 0.045,
          sweep: 0.022, roll: 45, thick: 0.007, col: shade(P.sensor, 1.42) });
        // The nose. A loitering munition's whole value is that it sees what it
        // is about to hit, so the seeker window is the payload and it is drawn
        // as a window with a lip and not as a bead.
        seekerHead(m, { z: L * 0.985, r: R * 0.6, col: shade(P.greyDark, 1.05),
          glass: shade(P.glass, 1.15), dome: 1.0 });
        // The motor and the folding two-blade propeller behind it.
        m.save().move(0, 0, -0.012);
        m.soft((mm) => mm.loft([
          { z: 0, pts: ring(R * 0.5, R * 0.5, mm.lod(14, 6)) },
          { z: 0.008, pts: ring(R * 0.62, R * 0.62, mm.lod(14, 6)) },
          { z: 0.03, pts: ring(R * 0.62, R * 0.62, mm.lod(14, 6)) },
          { z: 0.04, pts: ring(R * 0.46, R * 0.46, mm.lod(14, 6)) },
        ], shade(P.metal, 0.6), false));
        m.restore();
        m.save().move(0, 0, -0.018);
        m.soft((mm) => mm.ball(R * 0.4, mm.lod(12, 6), mm.lod(5, 3), shade(P.black, 1.5)));
        m.restore();
        for (let i = 0; i < 2; i += 1) {
          m.save().move(0, 0, -0.018).rotZ(i * 180 + 26).rotY(180);
          m.foil({ x0: 0.008, y: 0, z: 0, span: 0.1, root: 0.026, tip: 0.014,
            sweep: 0.008, thick: 0.13, incidence: 22, n: m.lod(9, 3), bays: m.lod(4, 1) },
          shade(P.black, 1.45));
          m.restore();
        }
        // The launch tube it is fired from, laid beside it on its bipod. The
        // tube IS half the system — the round is sealed in it until the moment
        // it is thrown out — so it is drawn as ordnance, with hoops and a
        // frangible cover, and not as a length of pipe.
        m.save().move(0.3, 0.075, -0.04).rotY(8).rotX(3);
        canister(m, { r: 0.075, len: 0.6, col: shade(P.cloth, 0.85), hoops: 4 });
        if (!m.far) {
          // Shoulder pad, carry sling swivels, the sight rail across the top
          // and the umbilical to the fire-control unit.
          m.save().move(0, 0.02, 0.12);
          m.slab([[-0.06, -0.08], [0.06, -0.08], [0.06, 0.08], [-0.06, 0.08]], 0.09, 0.02,
            shade(P.black, 1.4));
          m.restore();
          m.save().move(0, 0.082, 0.34);
          m.bar(-0.016, 0.016, 0, 0.022, -0.14, 0.14, shade(P.black, 1.6));
          m.restore();
          boltRun(m, { n: 4, r: 0.008, x0: 0, y0: 0.088, z0: 0.22, z1: 0.46,
            col: shade(P.metal, 0.72) });
          m.both((mm) => {
            mm.save().move(0.076, 0, 0.16).rotY(90);
            mm.soft((s) => s.tube(0.009, 0.009, 0.016, s.lod(8, 5), shade(P.metal, 0.6), true));
            mm.restore();
          });
          m.save().move(-0.05, -0.05, 0.08).rotZ(30).rotX(8);
          m.soft((mm) => mm.tube(0.008, 0.008, 0.16, mm.lod(8, 5), shade(P.black, 1.2), true));
          m.restore();
          // The bipod the tube stands on. A launcher lying on the ground with
          // nothing under it is a pipe somebody dropped.
          m.both((mm) => {
            mm.save().move(0.03, -0.06, 0.5).rotZ(26).rotX(-10);
            mm.soft((s) => s.tube(0.006, 0.005, 0.1, s.lod(7, 4), shade(P.metal, 0.6), true));
            mm.save().move(0, 0, 0.1);
            mm.bar(-0.018, 0.018, -0.006, 0.006, -0.012, 0.012, shade(P.metal, 0.5));
            mm.restore();
            mm.restore();
          });
        }
        m.restore();
        if (m.far) return;
        // The joint bands where the warhead, guidance and battery sections bolt
        // together, the raceway that wires them, and the two access covers.
        [0.14, 0.44, 0.76].forEach((t) => {
          jointBand(m, { z: L * t, r: R, w: 0.012, k: 1.07, seg,
            col: shade(P.sensor, 1.25) });
        });
        raceway(m, { r: R, z0: L * 0.16, z1: L * 0.74, h: 0.005, roll: 200,
          col: shade(P.sensor, 1.35) });
        // The fin actuator fairings on the aft ring. Four pop-out fins need
        // four servos and there is no room for them inside a 37 mm body, so
        // they live in bumps on the outside — which is exactly what the eye
        // reads as "this thing unfolds".
        for (let i = 0; i < 4; i += 1) {
          m.save().rotZ(45 + i * 90).move(0, R * 0.92, L * 0.28);
          m.soft((mm) => mm.loft([
            { z: -0.03, pts: ring(0.0015, 0.0015, mm.lod(9, 5)) },
            { z: -0.014, pts: ring(0.009, 0.006, mm.lod(9, 5)) },
            { z: 0.02, pts: ring(0.009, 0.006, mm.lod(9, 5)) },
            { z: 0.038, pts: ring(0.0015, 0.0015, mm.lod(9, 5)) },
          ], shade(P.sensor, 1.4), false));
          m.restore();
        }
        [[0.3, 90], [0.58, 250]].forEach((p) => {
          skinPanel(m, { rx: R, ry: R, th: p[1], z: L * p[0], w: 0.03, len: 0.05,
            depth: 0.004, col: shade(P.sensor, 1.3) });
        });
        boom(m, { x: 0, y: 0, z: L * 1.02, r: 0.0035, len: 0.03, col: shade(P.metal, 0.8) });
      },
    },
    cuas: {
      name: "Counter-UAS Battery", cls: "Infantry", span: 6.4,
      build(m) {
        // A LAYERED BATTERY IS FOUR THINGS ON ONE TRAILER: it has to see, it
        // has to identify, it has to jam, and when jamming does not work it has
        // to shoot. All four are on the mount, and the trailer under them is a
        // real trailer — a frame with rails and crossmembers, running gear that
        // shows its rims, a drawbar with a hitch on it and the four jacks it
        // stands on with the wheels unloaded. A battery drawn without those is
        // a radar floating over a plank.
        const bedY = 0.8;
        m.bar(-1.1, 1.1, 0.62, bedY, 0, 4.2, shade(P.green, 0.8));
        m.both((mm) => mm.bar(0.82, 1.02, 0.46, 0.66, 0.1, 4.1, shade(P.black, 1.35)));
        if (!m.far) {
          for (let i = 0; i < 6; i += 1) {
            const z = 0.25 + i * 0.75;
            m.bar(-0.92, 0.92, 0.5, 0.62, z - 0.06, z + 0.06, shade(P.black, 1.25));
          }
          // Mudguards over each wheel: four flat plates round the arc, which is
          // what a pressed-steel guard is.
          [1.1, 2.5].forEach((z) => {
            m.both((mm) => {
              for (let k = 0; k < 4; k += 1) {
                mm.save().move(1.0, 0.5, z).rotX(-54 + k * 36).move(0, 0.66, 0);
                mm.bar(-0.02, 0.28, 0, 0.04, -0.24, 0.24, shade(P.green, 0.76));
                mm.restore();
              }
            });
          });
        }
        wheels(m, { r: 0.5, w: 0.34, x: 1.15, axles: 2, z0: 1.1, z1: 2.5, detail: true });
        // The drawbar, its lunette eye and the jockey wheel folded up.
        m.loft([
          { z: 4.1, pts: trap(0.42, 0.46, 0.68, 0.9) },
          { z: 4.7, pts: trap(0.19, 0.5, 0.66, 0.9) },
          { z: 5.35, pts: trap(0.12, 0.52, 0.64, 0.9) },
        ], shade(P.green, 0.74));
        m.save().move(0, 0.58, 5.35).rotX(90);
        m.soft((mm) => mm.loft([
          { z: 0, pts: ring(0.11, 0.09, mm.lod(14, 6)) },
          { z: 0.06, pts: ring(0.11, 0.09, mm.lod(14, 6)) },
        ], shade(P.metal, 0.6), false));
        m.restore();
        [[-1.02, 0.3], [1.02, 0.3], [-1.02, 3.9], [1.02, 3.9]].forEach((p) => {
          jack(m, { x: p[0], y: 0.5, z: p[1], r: 0.07, len: 0.5 });
        });
        // The mount: a slew ring on a pedestal, and the head that turns on it.
        m.save().move(0, bedY, 2.35);
        m.soft((mm) => mm.loft([
          { z: 0, pts: ring(0.42, 0.42, mm.lod(16, 7)) },
          { z: 0.34, pts: ring(0.38, 0.38, mm.lod(16, 7)) },
          { z: 0.44, pts: ring(0.44, 0.44, mm.lod(16, 7)) },
          { z: 0.5, pts: ring(0.4, 0.4, mm.lod(16, 7)) },
        ], P.green, false));
        boltRun(m, { n: 8, r: 0.035, x0: 0, y0: 0.5, z0: 0, col: shade(P.metal, 0.62) });
        m.save().move(0, 0.5, 0);
        m.bar(-0.62, 0.62, 0, 0.7, -0.6, 0.6, shade(P.green, 1.06));
        // The search-and-track array, on the front face of the head. Its own
        // grid rather than `arrayFace`, which is shared with the AESA refit and
        // two warships and must not move — see the note on `arrayGrid`.
        arrayGrid(m, { x: -0.16, y: 0.74, z: 0.24, w: 0.86, h: 1.02, t: 0.09, tilt: 16,
          cols: 5, rows: 7, col: shade(P.sensor, 1.05) });
        // The electro-optical ball that identifies what the array found. It is
        // the difference between engaging a quadcopter and engaging a bird.
        gimbalBall(m, { x: 0.46, y: 0.86, z: 0.16, r: 0.22, col: shade(P.sensor, 1.2),
          apertures: [[0, 4, 0.44], [26, -8, 0.3, shade(P.glass, 1.15)],
            [-30, 6, 0.22, shade(P.red, 1.1)]] });
        m.save().move(0.46, 0.7, 0.16);
        m.bar(-0.26, 0.26, -0.34, 0.02, -0.1, 0.1, shade(P.green, 0.9));
        m.restore();
        // The jammer head: four horns on a common plate, pointing where the
        // array is pointing. This is the soft kill and it is most of the
        // engagements a battery like this actually wins.
        m.save().move(0, 0.28, 0.58);
        m.bar(-0.4, 0.4, -0.12, 0.12, -0.06, 0.02, shade(P.green, 0.86));
        for (let i = 0; i < 4; i += 1) {
          horn(m, { x: -0.24 + (i % 2) * 0.48, y: -0.06 + Math.floor(i / 2) * 0.12,
            z: 0.02, w: 0.2, len: 0.32, pitch: -6 });
        }
        m.restore();
        // The hard kill: a 30 mm effector on its cradle with the feed chute and
        // the ammunition box behind it.
        m.save().move(-0.02, 0.78, 0.3).rotX(-22);
        m.bar(-0.13, 0.13, -0.11, 0.11, -0.34, 0.1, shade(P.green, 0.94));
        m.save().move(0, 0, 0.1);
        m.soft((mm) => mm.tube(0.062, 0.05, 1.3, mm.lod(12, 6), P.exhaust, false));
        if (!m.far) {
          [0.3, 0.56].forEach((f) => {
            jointBand(m, { z: 0.1 + f * 1.2, r: 0.058, w: 0.06, k: 1.4, seg: 12,
              col: shade(P.exhaust, 1.2) });
          });
          m.save().move(0, 0, 1.4);
          m.soft((mm) => mm.tube(0.052, 0.052, 0.1, 12, shade(P.metal, 0.5), false));
          discCap(m, 0.03, 12, P.black, false);
          m.restore();
        }
        m.restore();
        m.bar(-0.19, 0.19, -0.3, 0.06, -0.56, -0.3, shade(P.green, 0.82));
        m.restore();
        m.restore();
        m.restore();
        // The tracking dish, forward on the bed and looking up: the fine track
        // that hands the effector its firing solution.
        m.save().move(0, bedY + 0.34, 0.75).rotX(-74);
        dish(m, { r: 0.62, col: shade(P.white, 0.92) });
        m.restore();
        m.save().move(0, bedY, 0.75);
        m.soft((mm) => mm.tube(0.13, 0.11, 0.36, mm.lod(12, 6), shade(P.green, 0.9), true));
        m.restore();
        if (m.far) return;
        // Three dish support struts, so the reflector stands on something.
        for (let i = 0; i < 3; i += 1) {
          m.save().move(0, bedY + 0.32, 0.75).rotY(i * 120 + 30).move(0, 0, 0.4)
            .rotX(24);
          m.bar(-0.022, 0.022, -0.022, 0.022, -0.2, 0.2, shade(P.metal, 0.66));
          m.restore();
        }
        // The generator set that runs all of it, at the back of the bed: a box
        // with a louvred intake, an exhaust stack and its fuel tank. A radar
        // with no power source on the trailer is a radar somebody forgot.
        m.save().move(0, bedY, 3.35);
        m.bar(-0.72, 0.72, 0, 0.66, -0.5, 0.5, shade(P.green, 1.02));
        louvre(m, { n: 6, y0: 0.12, y1: 0.54, z: 0.5, w: 0.9, t: 0.02, d: 0.03,
          col: shade(P.sensor, 1.3) });
        boltRun(m, { n: 5, r: 0.022, x0: -0.6, x1: 0.6, y0: 0.66, z0: -0.4,
          col: shade(P.metal, 0.66) });
        m.save().move(0.5, 0.66, -0.3);
        m.soft((mm) => mm.tube(0.05, 0.045, 0.5, mm.lod(10, 5), P.exhaust, false));
        m.save().move(0, 0, 0.5);
        discCap(m, 0.043, m.lod(10, 5), P.black, false);
        m.restore();
        m.restore();
        m.save().move(-0.42, 0.02, -0.62).rotX(90);
        m.soft((mm) => mm.tube(0.19, 0.19, 0.5, mm.lod(12, 6), shade(P.green, 0.86), true));
        m.restore();
        m.restore();
        // Cable runs from the generator to the mount, the operator's console on
        // the flank, and the two whip aerials for the command net.
        m.both((mm) => {
          mm.save().move(0.7, 0.78, 2.9).rotY(6);
          mm.soft((s) => s.tube(0.035, 0.035, 0.5, s.lod(8, 5), shade(P.black, 1.3), true));
          mm.restore();
        });
        m.save().move(-0.86, 0.62, 3.0).rotY(-90).rotX(-14);
        m.bar(-0.24, 0.24, 0, 0.34, -0.04, 0.04, shade(P.sensor, 1.2));
        m.save().move(0, 0.17, 0.04);
        m.slab([[-0.2, -0.12], [0.2, -0.12], [0.2, 0.12], [-0.2, 0.12]], 0.02, 0.006,
          shade(P.glass, 1.05));
        m.restore();
        m.restore();
        whip(m, { x: -1.02, y: bedY, z: 3.95, r: 0.016, len: 1.5, lean: 8 });
        whip(m, { x: 1.02, y: bedY, z: 3.95, r: 0.014, len: 1.2, lean: -10 });
        connector(m, { x: 0.6, y: 0.7, z: 4.15, r: 0.03, len: 0.06 });
        connector(m, { x: 0.46, y: 0.7, z: 4.15, r: 0.03, len: 0.06 });
      },
    },
    link16: {
      name: "Tactical Data Link Fit", cls: "Infantry", span: 2.2,
      build(m) {
        // THE FIT IS NOT A VEHICLE AND MUST NOT PRETEND TO BE ONE. A tactical
        // data link is a terminal, an antenna group and the rack they bolt
        // into, fitted to something that already exists — so the model is that
        // hardware on its mounting plate, and every detail on it is a detail
        // equipment has: rack rails, unit handles, card slots, status lamps,
        // circular connectors, a cable loom and the bolt line round the plate.
        // Drawing it as a truck would be a lie about what the money buys.
        m.save().move(0, 0.055, 0);
        m.slab([[-0.9, -0.7], [0.9, -0.7], [0.9, 0.7], [-0.9, 0.7]], 0.11, 0.035,
          P.greyDark);
        m.restore();
        boltRun(m, { n: m.lod(7, 2), r: 0.03, x0: -0.82, x1: 0.82, y0: 0.11, z0: -0.62,
          col: shade(P.metal, 0.66) });
        boltRun(m, { n: m.lod(7, 2), r: 0.03, x0: -0.82, x1: 0.82, y0: 0.11, z0: 0.62,
          col: shade(P.metal, 0.66) });
        boltRun(m, { n: 4, r: 0.03, x0: -0.82, y0: 0.11, z0: -0.42, z1: 0.42,
          col: shade(P.metal, 0.66) });
        boltRun(m, { n: 4, r: 0.03, x0: 0.82, y0: 0.11, z0: -0.42, z1: 0.42,
          col: shade(P.metal, 0.66) });
        // The rack. Four uprights, the pair of drilled rails the units screw
        // to, and the units themselves sliding in from the front.
        const rx = 0.34, rz = -0.28, rh = 0.98;
        m.save().move(-0.3, 0.11, rz);
        [[-rx, -0.26], [rx, -0.26], [-rx, 0.26], [rx, 0.26]].forEach((c) => {
          m.bar(c[0] - 0.028, c[0] + 0.028, 0, rh, c[1] - 0.028, c[1] + 0.028,
            shade(P.greyDark, 0.86));
        });
        m.bar(-rx - 0.03, rx + 0.03, rh, rh + 0.05, -0.3, 0.3, shade(P.greyDark, 1.12));
        m.bar(-rx - 0.03, rx + 0.03, 0, 0.05, -0.3, 0.3, shade(P.greyDark, 0.8));
        if (!m.far) {
          m.both((mm) => {
            mm.save().move(0.3, 0, 0);
            mm.bar(-0.012, 0.012, 0.05, rh, 0.2, 0.24, shade(P.metal, 0.68));
            mm.restore();
          });
          louvre(m, { n: 8, y0: 0.1, y1: 0.92, z: -0.3, w: 0.56, t: 0.012, d: 0.02,
            col: shade(P.sensor, 1.2) });
        }
        const units = m.lod(6, 3);
        for (let i = 0; i < units; i += 1) {
          rackUnit(m, { x: 0, y: 0.07 + i * (rh - 0.1) / units, z: 0.27,
            w: 0.6, h: (rh - 0.1) / units * 0.88, d: 0.5,
            slots: 3 + (i % 2), col: shade(P.greyDark, 1 + ((i % 3) - 1) * 0.07) });
        }
        if (!m.far) {
          // The back of the rack: the circular connectors every box on it is
          // wired through, and the loom that leaves for the antennas.
          for (let i = 0; i < 8; i += 1) {
            connector(m, { x: -0.22 + (i % 4) * 0.15, y: 0.2 + Math.floor(i / 4) * 0.5,
              z: -0.3, r: 0.035, len: 0.07, yaw: 180 });
          }
          for (let i = 0; i < 3; i += 1) {
            m.save().move(-0.18 + i * 0.18, 0.24 + i * 0.06, -0.4).rotY(-24 - i * 8);
            m.soft((mm) => mm.tube(0.022, 0.022, 0.34, mm.lod(8, 5),
              shade(P.black, 1.2 + i * 0.1), true));
            m.restore();
          }
        }
        m.restore();
        // The antenna group on the front half of the plate: the low-profile
        // radome that hides the steered array, the blade that carries the
        // voice net, two whips for the legacy radios and the navigation patch
        // they are all timed against.
        m.save().move(0.42, 0.11, 0.22);
        m.soft((mm) => {
          const rings = mm.lod(6, 3), seg = mm.lod(18, 7), secs = [];
          for (let i = 0; i <= rings; i += 1) {
            const t = (i / rings) * Math.PI * 0.5;
            const rr = Math.max(1e-3, Math.cos(t) * 0.3);
            secs.push({ z: Math.sin(t) * 0.19, pts: ring(rr, rr * 0.86, seg) });
          }
          mm.save().rotX(-90);
          mm.loft(secs, shade(P.white, 1.0), false);
          mm.restore();
        });
        m.soft((mm) => {
          mm.save().rotX(-90);
          mm.loft([{ z: -0.02, pts: ring(0.315, 0.271, mm.lod(18, 7)) },
            { z: 0.02, pts: ring(0.3, 0.258, mm.lod(18, 7)) }], shade(P.greyDark, 1.0), false);
          mm.restore();
        });
        boltRun(m, { n: m.lod(8, 3), r: 0.018, x0: 0, y0: 0.01, z0: 0,
          col: shade(P.metal, 0.7) });
        m.restore();
        bladeAerial(m, { x: -0.62, y: 0.11, z: 0.34, h: 0.42, len: 0.3, t: 0.05,
          col: shade(P.white, 0.95) });
        whip(m, { x: 0.78, y: 0.11, z: -0.5, r: 0.02, len: 1.05, lean: -7 });
        whip(m, { x: -0.78, y: 0.11, z: -0.5, r: 0.018, len: 0.86, lean: 7 });
        // The auxiliary stack beside the main rack: the crypto unit, the power
        // supply and the frequency reference, with the heat-sink fins that are
        // the whole reason a radio fit needs a plate this size.
        m.save().move(-0.62, 0.11, -0.22);
        [[-0.2, -0.2], [0.2, -0.2], [-0.2, 0.2], [0.2, 0.2]].forEach((c) => {
          m.bar(c[0] - 0.024, c[0] + 0.024, 0, 0.62, c[1] - 0.024, c[1] + 0.024,
            shade(P.greyDark, 0.84));
        });
        m.bar(-0.22, 0.22, 0.62, 0.67, -0.24, 0.24, shade(P.greyDark, 1.1));
        m.bar(-0.22, 0.22, 0, 0.05, -0.24, 0.24, shade(P.greyDark, 0.78));
        for (let i = 0; i < m.lod(3, 2); i += 1) {
          rackUnit(m, { x: 0, y: 0.06 + i * 0.185, z: 0.21, w: 0.38, h: 0.16, d: 0.4,
            slots: 2, col: shade(P.metal, 0.74 + i * 0.06) });
        }
        if (!m.far) {
          for (let i = 0; i < 9; i += 1) {
            m.bar(-0.21 + i * 0.047, -0.192 + i * 0.047, 0.67, 0.78, -0.22, 0.22,
              shade(P.metal, 0.88));
          }
          connector(m, { x: -0.08, y: 0.3, z: -0.25, r: 0.03, len: 0.06, yaw: 180 });
          connector(m, { x: 0.08, y: 0.3, z: -0.25, r: 0.03, len: 0.06, yaw: 180 });
        }
        m.restore();
        if (m.far) return;
        // The waveguide from the rack to the radome. A steered array is fed by
        // a rigid run with a flange at every break, not by a wire, and the
        // flanges are what say this carries power rather than signal.
        [[-0.24, 0.9, -0.16, 34], [0.06, 0.66, 0.02, 18], [0.3, 0.4, 0.14, 8]]
          .forEach((p, i) => {
            m.save().move(p[0], p[1], p[2]).rotY(-56 + i * 14).rotX(p[3]);
            m.soft((mm) => mm.tube(0.035, 0.033, 0.3, mm.lod(10, 5),
              shade(P.metal, 0.78), true));
            jointBand(m, { z: 0.02, r: 0.037, w: 0.02, k: 1.5, seg: m.lod(10, 5),
              col: shade(P.metal, 0.6) });
            jointBand(m, { z: 0.28, r: 0.037, w: 0.02, k: 1.5, seg: m.lod(10, 5),
              col: shade(P.metal, 0.6) });
            m.restore();
          });
        // The navigation patch on its ground plane, and the terminal head the
        // operator actually touches: a display and a keypad, because a fit with
        // no controls on it anywhere is a prop.
        m.save().move(-0.2, 0.11, 0.5);
        m.slab([[-0.14, -0.14], [0.14, -0.14], [0.14, 0.14], [-0.14, 0.14]], 0.045, 0.014,
          shade(P.greyDark, 1.1));
        m.save().move(0, 0.04, 0);
        m.slab([[-0.09, -0.09], [0.09, -0.09], [0.09, 0.09], [-0.09, 0.09]], 0.03, 0.01,
          shade(P.white, 0.9));
        m.restore();
        m.restore();
        m.save().move(0.16, 0.11, 0.56).rotX(-24);
        m.bar(-0.22, 0.22, 0, 0.26, -0.06, 0.06, shade(P.sensor, 1.2));
        m.save().move(0, 0.19, 0.06);
        m.slab([[-0.17, -0.05], [0.17, -0.05], [0.17, 0.05], [-0.17, 0.05]], 0.02, 0.006,
          shade(P.glass, 1.05));
        m.restore();
        for (let i = 0; i < 12; i += 1) {
          m.save().move(-0.15 + (i % 6) * 0.06, 0.06 + Math.floor(i / 6) * 0.05, 0.06);
          m.bar(-0.022, 0.022, -0.018, 0.018, -0.008, 0.008,
            shade(P.greyDark, 1.1 + ((i % 3) - 1) * 0.12));
          m.restore();
        }
        m.restore();
      },
    },
    c4isr: {
      name: "Networked C4ISR", cls: "Infantry", span: 7.4,
      build(m) {
        // A COMMAND POST IS A ROOM THAT DROVE THERE, and both halves of that
        // sentence have to be on the card. The lorry gets its running gear,
        // mudguards, frame rails, cab furniture and stack — `truck`'s `detail`
        // arm, which is opt-in and already there, so nothing the Missile class
        // built against this call moves. The room gets what a shelter body
        // actually is: corner extrusions, panel seams with the bolt line down
        // them, a door with hinges and a handle, an air-conditioning pack, the
        // cable entry with its connectors, and the aerial farm on the roof.
        const L = 7.4, W = 2.5;
        const bed = truck(m, { len: L, w: W, wheelR: 0.55, axles: 2, cab: 2.0, cabH: 1.7,
          col: P.green, detail: true });
        const top = bed + 2.1, z0 = 0.3, z1 = 5.2;
        m.loft([{ z: z0, pts: trap(1.25, bed, top, 0.98) },
          { z: z1, pts: trap(1.25, bed, top, 0.98) }], shade(P.green, 1.05));
        if (!m.far) {
          // The corner extrusions the panels are riveted into. A shelter body
          // is a frame with skins on it and the corners are where that shows.
          m.both((mm) => {
            [[bed + 0.02, 0.14], [top - 0.16, 0.14]].forEach((c) => {
              mm.bar(1.16, 1.28, c[0], c[0] + c[1], z0 - 0.02, z1 + 0.02,
                shade(P.green, 0.82));
            });
            mm.bar(1.16, 1.3, bed, top, z0 - 0.02, z0 + 0.1, shade(P.green, 0.86));
            mm.bar(1.16, 1.3, bed, top, z1 - 0.1, z1 + 0.02, shade(P.green, 0.86));
            // Panel seams down the flank, three bays, each with its bolt line.
            for (let i = 0; i < 3; i += 1) {
              const z = z0 + 0.35 + i * 1.55;
              mm.bar(1.2, 1.3, bed + 0.1, top - 0.1, z - 0.05, z + 0.05,
                shade(P.green, 0.9));
              boltRun(mm, { n: 5, r: 0.028, x0: 1.3, y0: bed + 0.28, y1: top - 0.28,
                z0: z, pitch: 0, yaw: 90, col: shade(P.metal, 0.64) });
            }
          });
          // The door at the back, with its two hinges, the handle and the step
          // under it. A shelter with no way into it is a crate.
          m.save().move(0.42, bed + 1.0, z0 - 0.03).rotX(90);
          m.slab([[-0.38, -0.9], [0.38, -0.9], [0.38, 0.9], [-0.38, 0.9]], 0.09, 0.03,
            shade(P.green, 1.14));
          m.restore();
          [bed + 0.35, bed + 1.65].forEach((y) => {
            m.bar(0.02, 0.14, y - 0.09, y + 0.09, z0 - 0.11, z0 - 0.01,
              shade(P.metal, 0.66));
          });
          m.bar(0.68, 0.78, bed + 0.86, bed + 1.14, z0 - 0.12, z0 - 0.02,
            shade(P.metal, 0.8));
          m.bar(0.06, 0.78, bed - 0.36, bed - 0.28, z0 - 0.36, z0 - 0.06,
            shade(P.metal, 0.6));
          m.both((mm) => {
            mm.bar(0.7, 0.76, bed - 0.32, bed, z0 - 0.32, z0 - 0.24, shade(P.metal, 0.55));
          });
          // The air-conditioning pack on the front wall and the cable entry
          // panel low on the flank. A room full of radios is mostly a cooling
          // problem, and the pack is the honest way to say so.
          m.save().move(-0.5, top - 0.72, z1 + 0.02);
          m.bar(-0.42, 0.42, 0, 0.56, 0, 0.24, shade(P.green, 0.92));
          louvre(m, { n: 6, y0: 0.08, y1: 0.48, z: 0.24, w: 0.62, t: 0.02, d: 0.03,
            col: shade(P.sensor, 1.25) });
          m.restore();
          m.save().move(-1.26, bed + 0.44, 1.4);
          m.slab([[-0.34, -0.22], [0.34, -0.22], [0.34, 0.22], [-0.34, 0.22]], 0.08, 0.024,
            shade(P.greyDark, 1.0));
          m.restore();
          for (let i = 0; i < 6; i += 1) {
            connector(m, { x: -1.32, y: bed + 0.3 + Math.floor(i / 3) * 0.28,
              z: 1.24 + (i % 3) * 0.16, r: 0.045, len: 0.08, yaw: -90 });
          }
        }
        // The satellite terminal on the roof: the reflector, its feed, the
        // pedestal it steers on and the three struts that hold the rim.
        m.save().move(0, top + 0.42, 2.6).rotX(-58);
        dish(m, { r: 1.0, col: shade(P.white, 0.94) });
        m.restore();
        m.save().move(0, top, 2.6);
        m.soft((mm) => mm.loft([
          { z: 0, pts: ring(0.24, 0.24, mm.lod(14, 6)) },
          { z: 0.3, pts: ring(0.2, 0.2, mm.lod(14, 6)) },
          { z: 0.4, pts: ring(0.26, 0.26, mm.lod(14, 6)) },
        ], shade(P.green, 0.94), false));
        m.restore();
        // Two telescoping masts, sectioned, with a collar at every break.
        [[1.0, 0.52, 2.4, 0.055], [-1.0, 0.6, 1.8, 0.045]].forEach((p) => {
          m.save().move(p[0], top, p[2]).rotX(-90);
          for (let i = 0; i < 3; i += 1) {
            const r = p[3] * (1 - i * 0.22);
            m.save().move(0, 0, p[1] * i * 0.9);
            m.soft((mm) => mm.tube(r, r * 0.96, p[1], mm.lod(9, 5),
              shade(P.metal, 0.72 + i * 0.08), true));
            if (!m.far) {
              jointBand(m, { z: 0.03, r: r * 1.02, w: 0.05, k: 1.4, seg: m.lod(9, 5),
                col: shade(P.metal, 0.55) });
            }
            m.restore();
          }
          m.restore();
        });
        // The lattice mast that carries the line-of-sight relay. Lattice is how
        // a real mast is built and the see-through of it is what tells the eye
        // it is structure and not a post.
        lattice(m, { x: -0.7, y: top, z: 4.4, len: 1.6, w: 0.16, h: 0.16, t: 0.03, n: 4,
          col: shade(P.metal, 0.68) });
        if (m.far) return;
        // The aerial farm across the roof, the ladder up the back, the spare
        // wheel on the flank and the awning frame over the door.
        [0.9, 1.4, 3.6, 4.1].forEach((z, i) => {
          bladeAerial(m, { x: (i % 2 ? 0.75 : -0.75), y: top, z, h: 0.28, len: 0.22,
            t: 0.04, col: shade(P.green, 0.86) });
        });
        whip(m, { x: 1.12, y: top, z: 1.0, r: 0.018, len: 1.5, lean: 6 });
        whip(m, { x: -1.12, y: top, z: 1.2, r: 0.016, len: 1.2, lean: -6 });
        m.save().move(0.5, top, 3.0);
        m.slab([[-0.16, -0.16], [0.16, -0.16], [0.16, 0.16], [-0.16, 0.16]], 0.06, 0.018,
          P.sensor);
        m.restore();
        m.both((mm) => {
          mm.bar(0.36, 0.42, bed - 0.3, top + 0.05, z0 - 0.16, z0 - 0.08,
            shade(P.metal, 0.7));
        });
        for (let i = 0; i < 6; i += 1) {
          m.bar(-0.42, 0.42, bed - 0.24 + i * 0.44, bed - 0.18 + i * 0.44,
            z0 - 0.16, z0 - 0.08, shade(P.metal, 0.62));
        }
        roadWheel(m, { x: -1.34, y: bed + 0.62, z: 4.3, r: 0.55, w: 0.4 });
        m.save().move(1.28, bed + 0.3, 3.2);
        m.bar(-0.1, 0.1, 0, 0.5, -0.4, 0.4, shade(P.green, 0.88));
        louvre(m, { n: 5, y0: 0.08, y1: 0.42, z: 0.1, w: 0.16, t: 0.014, d: 0.02,
          yaw: 90, col: shade(P.sensor, 1.2) });
        m.restore();
        m.both((mm) => {
          mm.bar(1.16, 1.24, top - 0.06, top + 0.02, z0 + 0.2, z1 - 0.2,
            shade(P.metal, 0.72));
        });
      },
    },
    atr: {
      name: "Autonomous Target Recognition", cls: "Infantry", span: 0.8,
      build(m) {
        // AN ALGORITHM IS NOT A VEHICLE EITHER. What the deck is buying is the
        // sensor that feeds the algorithm and the box that runs it, so the
        // model is a gimballed head on a yoke and the processing stack behind
        // it — not a new machine, and not a turret with a gun in it. The
        // recognition happens BEHIND the windows, which is why the windows are
        // real geometry: four apertures, each a lip cut into the sphere with a
        // domed window inside it, because a bright ball on a stick reads as a
        // bead and a bead says nothing about what this costs.
        const R = 0.24, plateY = 0.05;
        m.save().move(0, plateY / 2, 0);
        m.slab([[-0.3, -0.3], [0.3, -0.3], [0.3, 0.3], [-0.3, 0.3]], plateY, 0.018,
          P.greyDark);
        m.restore();
        boltRun(m, { n: m.lod(7, 2), r: 0.022, x0: -0.26, x1: 0.26, y0: plateY, z0: -0.26,
          col: shade(P.metal, 0.66) });
        boltRun(m, { n: m.lod(7, 2), r: 0.022, x0: -0.26, x1: 0.26, y0: plateY, z0: 0.26,
          col: shade(P.metal, 0.66) });
        boltRun(m, { n: 5, r: 0.022, x0: -0.26, y0: plateY, z0: -0.17, z1: 0.17,
          col: shade(P.metal, 0.66) });
        boltRun(m, { n: 5, r: 0.022, x0: 0.26, y0: plateY, z0: -0.17, z1: 0.17,
          col: shade(P.metal, 0.66) });
        // The azimuth ring the whole head turns on, and its bolt circle.
        m.save().move(0, plateY, 0).rotX(-90);
        m.soft((mm) => mm.loft([
          { z: 0, pts: ring(0.19, 0.19, mm.lod(16, 7)) },
          { z: 0.07, pts: ring(0.185, 0.185, mm.lod(16, 7)) },
          { z: 0.1, pts: ring(0.2, 0.2, mm.lod(16, 7)) },
          { z: 0.14, pts: ring(0.17, 0.17, mm.lod(16, 7)) },
        ], shade(P.greyDark, 1.08), false));
        m.restore();
        if (!m.far) {
          for (let i = 0; i < 12; i += 1) {
            m.save().rotY(i * 30).move(0, plateY + 0.1, 0.185);
            m.soft((mm) => mm.loft([
              { z: 0, pts: ring(0.016, 0.016, mm.lod(6, 4)) },
              { z: 0.012, pts: ring(0.014, 0.014, mm.lod(6, 4)) },
              { z: 0.018, pts: ring(0.007, 0.007, mm.lod(6, 4)) },
            ], shade(P.metal, 0.6), false));
            m.restore();
          }
        }
        // The yoke: two arms and the bearing housings the ball hangs between.
        m.both((mm) => {
          mm.save().move(0.2, plateY + 0.14, 0).rotX(-90);
          mm.soft((s) => s.loft([
            { z: 0, pts: ringSuper(0.055, 0.045, s.lod(10, 5), 3.0) },
            { z: 0.1, pts: ringSuper(0.05, 0.042, s.lod(10, 5), 3.0) },
            { z: 0.22, pts: ringSuper(0.048, 0.05, s.lod(10, 5), 3.0) },
            { z: 0.3, pts: ringSuper(0.055, 0.06, s.lod(10, 5), 3.0) },
          ], shade(P.greyDark, 1.14), false));
          mm.restore();
          mm.save().move(0.19, plateY + 0.44, 0).rotY(90);
          mm.soft((s) => s.tube(0.06, 0.055, 0.045, s.lod(12, 6),
            shade(P.metal, 0.72), false));
          mm.save().move(0, 0, -0.02);
          discCap(mm, 0.058, mm.lod(12, 6), shade(P.metal, 0.9), true);
          mm.restore();
          mm.restore();
        });
        gimbalBall(m, { y: plateY + 0.44, r: R, col: P.sensor,
          apertures: [[0, 6, 0.46], [30, -10, 0.3, shade(P.glass, 1.15)],
            [-32, -8, 0.26, shade(P.sensor, 0.7)], [8, 34, 0.16, shade(P.red, 1.05)],
            [-16, -34, 0.15, shade(P.glass, 0.8)]] });
        // The sun shade over the forward window: a short hood that keeps the
        // sun off the optic, and the one part of the head that is not a sphere.
        if (!m.far) {
          m.save().move(0, plateY + 0.47, R * 0.86).rotX(-6);
          m.soft((mm) => mm.loft([
            { z: 0, pts: ring(R * 0.52, R * 0.52, mm.lod(18, 8)) },
            { z: 0.05, pts: ring(R * 0.56, R * 0.56, mm.lod(18, 8)) },
          ], shade(P.sensor, 0.72), false));
          m.restore();
        }
        // The processing stack behind it. This is where the recognition
        // actually runs, and it looks like what it is: a sealed box with more
        // heat-sink fin than box, a front panel with its lamps, and the
        // connectors the sensor and the host platform plug into.
        m.save().move(0, plateY, -0.42);
        m.bar(-0.22, 0.22, 0, 0.26, -0.16, 0.16, shade(P.sensor, 1.25));
        if (!m.far) {
          for (let i = 0; i < 20; i += 1) {
            m.bar(-0.213 + i * 0.021, -0.204 + i * 0.021, 0.26, 0.36, -0.15, 0.15,
              shade(P.metal, 0.8 + (i % 2) * 0.1));
          }
          m.save().move(0, 0.13, 0.16).rotX(90);
          m.slab([[-0.22, -0.13], [0.22, -0.13], [0.22, 0.13], [-0.22, 0.13]], 0.03, 0.01,
            shade(P.sensor, 1.45));
          m.restore();
          [0.06, 0.13, 0.2].forEach((y, i) => {
            m.save().move(-0.16, y, 0.18);
            m.bar(-0.016, 0.016, -0.014, 0.014, -0.008, 0.008,
              i === 0 ? shade(P.red, 1.25) : shade(P.glass, 1.2));
            m.restore();
          });
          for (let i = 0; i < 4; i += 1) {
            connector(m, { x: -0.06 + (i % 2) * 0.14, y: 0.07 + Math.floor(i / 2) * 0.12,
              z: 0.17, r: 0.028, len: 0.05 });
          }
          for (let i = 0; i < 3; i += 1) {
            connector(m, { x: -0.1 + i * 0.1, y: 0.13, z: -0.17, r: 0.026, len: 0.05,
              yaw: 180 });
          }
          louvre(m, { n: 5, y0: 0.05, y1: 0.22, z: 0, x: 0.22, w: 0.24, t: 0.012, d: 0.016,
            yaw: 90, col: shade(P.sensor, 1.5) });
          // The rack ears and the slide rails: this box lives in something
          // else's rack, and the ears are what says so.
          m.both((mm) => {
            mm.bar(0.22, 0.29, 0.04, 0.22, 0.12, 0.16, shade(P.metal, 0.66));
            mm.bar(0.2, 0.24, 0.09, 0.13, -0.16, 0.16, shade(P.metal, 0.58));
          });
          boltRun(m, { n: 3, r: 0.016, x0: 0.255, y0: 0.06, y1: 0.2, z0: 0.17,
            pitch: 0, yaw: 90, col: shade(P.metal, 0.55) });
        }
        m.restore();
        if (m.far) return;
        // The loom from the head to the box, and the inertial reference beside
        // it: the head has to know which way it is pointing before anything it
        // sees can be a coordinate.
        [[-0.07, 0.2], [0.07, 0.24]].forEach((p, i) => {
          m.save().move(p[0], plateY + p[1], -0.16).rotX(58 + i * 8).rotY(i ? 8 : -8);
          m.soft((mm) => mm.tube(0.019, 0.019, 0.28, mm.lod(9, 5),
            shade(P.black, 1.2 + i * 0.15), true));
          jointBand(m, { z: 0.03, r: 0.021, w: 0.016, k: 1.5, seg: m.lod(9, 5),
            col: shade(P.metal, 0.62) });
          jointBand(m, { z: 0.25, r: 0.021, w: 0.016, k: 1.5, seg: m.lod(9, 5),
            col: shade(P.metal, 0.62) });
          m.restore();
        });
        m.save().move(-0.2, plateY, -0.14);
        m.bar(-0.08, 0.08, 0, 0.09, -0.07, 0.07, shade(P.greyDark, 1.16));
        m.save().move(0, 0.09, 0);
        m.slab([[-0.06, -0.05], [0.06, -0.05], [0.06, 0.05], [-0.06, 0.05]], 0.02, 0.006,
          shade(P.metal, 0.8));
        m.restore();
        m.restore();
        m.save().move(0.22, plateY, -0.15);
        m.slab([[-0.05, -0.05], [0.05, -0.05], [0.05, 0.05], [-0.05, 0.05]], 0.03, 0.009,
          P.sensor);
        m.restore();
      },
    },

    // ------------------------------------------------------------------ space
    spc_recon: {
      name: "Reconnaissance Satellite Constellation", cls: "Space", span: 12,
      build(m) {
        // A constellation is not one satellite, and the deck charges for the
        // constellation. Three fly here: the lead ship at full size and two in
        // company at the sizes perspective would give them, each in its own
        // attitude — a plane of identical craft is identical craft pointed
        // differently, and three in the same pose would read as one drawn
        // thrice. See `satellite`'s note on `trim` for what the two behind
        // spend their triangles on instead of quilting.
        reconCraft(m, true);
        [[-5.2, 2.6, -6.0, 0.42, 28, 14], [5.6, -2.2, -7.4, 0.34, -36, -11]]
          .forEach((s) => {
            m.save().move(s[0], s[1], s[2]).rotY(s[4]).rotZ(s[5]).scale(s[3]);
            reconCraft(m, false);
            m.restore();
          });
      },
    },
    kh11: {
      name: "Electro-Optical Reconnaissance Satellite", cls: "Space", span: 15,
      build(m) {
        // A TELESCOPE WITH A SPACECRAFT BOLTED TO THE BACK OF IT, which is what
        // one of these is: the tube sets the size and everything else follows.
        //
        // DRAWN ALONG +Z, WHICH IS THIS FILE'S OWN CONVENTION AND WAS NOT BEING
        // KEPT HERE. The header says +Z is the direction the thing points —
        // nose, bow, muzzle — and for this one that is the aperture; it was
        // lying on its side along +Y while its own class-mate pointed forward,
        // so the two Space models disagreed with each other and with every
        // other model in the deck. Pointing it correctly also puts the arrays
        // across the card, which is the axis a three-quarter view has most of.
        telescope(m, { z: -1.4, r: 1.35, len: 8.2, hood: 2.6, hoops: 4, teeth: 12,
          quilt: 3, vanes: 3, col: P.gold, mirror: shade(P.glass, 1.3) });
        m.save().move(0, 0, -3.2);
        satellite(m, { bus: [2.6, 2.4, 3.6], span: 15.0, panelW: 2.6, bays: 4,
          col: shade(P.gold, 0.9), rot: 20, cols: 4, cells: 5 });
        m.restore();
        // Two antennas, and they are aimed at different things: the big one at
        // a relay satellite somewhere astern, the small one down at a ground
        // station. Both trained off-axis, because a dish that agrees with the
        // spacecraft's own axis is a dish nobody pointed.
        spaceDish(m, { x: 0, y: -1.5, z: -4.4, r: 1.05, az: 34, el: -46,
          rings: m.lod(5, 2) });
        spaceDish(m, { x: 1.15, y: 0.9, z: -3.6, r: 0.6, az: -46, el: 24,
          seg: m.lod(14, 8), rings: m.lod(4, 2), ribs: 4 });
        if (m.far) return;
        // The rest of what an operational one carries: the attitude-control
        // clusters at the corners of the bus, the trackers that tell it which
        // way it is facing, the boom that gets the magnetometer clear of the
        // spacecraft's own field, and the aerials.
        [[1, 1], [-1, 1], [1, -1], [-1, -1]].forEach((c) => {
          thrusterPod(m, { x: c[0] * 0.85, y: c[1] * 0.85, z: -4.9, r: 0.19,
            yaw: 180 });
        });
        starTracker(m, { x: 0.9, y: -0.72, z: -2.4, r: 0.12, yaw: 38, pitch: 34 });
        starTracker(m, { x: -0.9, y: -0.72, z: -2.4, r: 0.12, yaw: -38, pitch: 34 });
        starTracker(m, { x: 0, y: -1.15, z: -3.4, r: 0.12, pitch: 62 });
        deployMast(m, { x: -1.05, y: 0.95, z: -3.0, pitch: -34, yaw: -26, t: 0.07,
          len: 3.2, n: 5, col: shade(P.metal, 0.7) });
        whip(m, { x: 0.9, y: 1.0, z: -4.4, r: 0.02, len: 1.4, lean: 14 });
        bladeAerial(m, { x: -0.6, y: -1.24, z: -1.9, h: 0.24, len: 0.44, t: 0.05,
          col: shade(P.gold, 0.78) });
        bladeAerial(m, { x: 0.55, y: -1.24, z: -4.2, h: 0.2, len: 0.38, t: 0.045,
          col: shade(P.gold, 0.78) });
      },
    },
  });

  // ------------------------------------------------------------------- api
  /// Which model stands in when an id is not in the table. The deck is edited
  /// in Rust and this file is JavaScript, so the two CAN drift — `main.rs`
  /// asserts they do not, but a browser that has been handed a kit it has never
  /// heard of should draw the right KIND of thing rather than nothing at all.
  const CLASS_FALLBACK = {
    infantry: "inf_light", armour: "arm_gen3", air: "air_gen4",
    naval: "nav_escort", missile: "msl_brm", space: "spc_recon",
  };

  const cache = new Map();

  function has(id) { return Object.prototype.hasOwnProperty.call(MODELS, id); }
  function resolve(id, cls) {
    if (has(id)) return id;
    const key = String(cls || "").toLowerCase();
    return CLASS_FALLBACK[key] || null;
  }
  /// Build once, keep. A card that scrolls in and out of view rebuilds nothing,
  /// and the same id always yields the same mesh.
  ///
  /// WHAT THAT NOW COSTS, MEASURED RATHER THAN REMEMBERED. This comment used to
  /// say the whole deck was under a megabyte of Float32Array, and at 452
  /// triangles a model it was. ALL SIX CLASSES HAVE NOW HAD THEIR DETAIL PASS —
  /// Air, Missile, Infantry, Naval, Armour and now Space — and every one of the
  /// forty-six models is between four and twelve thousand triangles, so the
  /// deck's detailed meshes come to 272,491 triangles: 28.07 MiB of attribute
  /// arrays if every one of them is built, plus 4.07 MiB for the 39,508
  /// triangles of coarse variant behind them if every one of those is built
  /// too. Both figures are read off `byteLength` rather than derived: three
  /// Float32Arrays a mesh, nine floats a triangle, so a triangle is 108 bytes
  /// wherever it sits. The cache only fills with what is actually drawn, so a
  /// page showing a dozen cards pays for a dozen; a page that builds all
  /// forty-six pays the lot. This was the last line in the file waiting on the
  /// last class, and it is now a measurement of the finished deck rather than
  /// of a deck mid-pass: the Space pass added 19,846 triangles on two models,
  /// and it is the pass that gave the coarse level its first real work here —
  /// those two had no coarse variant at all before it, being 542 triangles at
  /// both levels, and they are 1,354 and 876 now.
  ///
  /// THE THIRD ARGUMENT IS OPTIONAL AND THE SIGNATURE DID NOT MOVE.
  /// `build(id, cls)` returns the detailed mesh it always returned; only
  /// `build(id, cls, { lod: "far" })` asks for the coarse one, which is the
  /// mesh for a map pin or a sprite — the same recipe with coarser rings and
  /// none of the greebles that exist to be read at card size. The two are
  /// cached in separate slots under the same id, so a page that draws both
  /// pays for each once.
  function build(id, cls, opts) {
    const key = resolve(id, cls);
    if (!key) return null;
    const far = !!(opts && opts.lod === "far");
    const slot = far ? `${key}|far` : key;
    if (cache.has(slot)) return cache.get(slot);
    const m = new Mesh(far ? "far" : "near");
    MODELS[key].build(m);
    const geom = m.finish();
    geom.id = key;
    geom.name = MODELS[key].name;
    geom.cls = MODELS[key].cls;
    geom.span = MODELS[key].span;
    geom.lod = far ? "far" : "near";
    cache.set(slot, geom);
    return geom;
  }
  function ids() { return Object.keys(MODELS); }
  function meta(id) {
    return has(id)
      ? { id, name: MODELS[id].name, cls: MODELS[id].cls, span: MODELS[id].span }
      : null;
  }

  /// Wavefront OBJ, with the colour carried on the vertex line — the extension
  /// Blender, MeshLab and three.js all read. Written so these meshes are not
  /// trapped in this page: `node tools/arsenal-obj.js` dumps the whole deck to
  /// files an artist can open, and if one of them is ever replaced by something
  /// hand-modelled, the OBJ is the thing to match.
  function toOBJ(id) {
    const g = build(id);
    if (!g) return null;
    const out = [`# SPHERES arsenal model: ${g.id} — ${g.name}`,
      `# class ${g.cls}, ${g.count / 3} triangles, metres, +Z forward +Y up`,
      `o ${g.id}`];
    const p = g.positions, c = g.colors, n = g.normals;
    for (let i = 0; i < p.length; i += 3) {
      out.push(`v ${p[i].toFixed(4)} ${p[i + 1].toFixed(4)} ${p[i + 2].toFixed(4)} `
        + `${c[i].toFixed(3)} ${c[i + 1].toFixed(3)} ${c[i + 2].toFixed(3)}`);
    }
    // ONE NORMAL PER VERTEX, NOT PER FACE. It used to be per face, which was
    // exactly right while every triangle in this file was flat-shaded and is
    // wrong now that fuselages, wings and nozzles carry averaged normals: an
    // export that collapsed those three corners back to one would throw away
    // the whole of the smooth-shading pass on the way out of the door, and the
    // OBJ is the thing a hand-modelled replacement would have to match.
    for (let i = 0; i < n.length; i += 3) {
      out.push(`vn ${n[i].toFixed(4)} ${n[i + 1].toFixed(4)} ${n[i + 2].toFixed(4)}`);
    }
    for (let t = 0; t < g.count / 3; t += 1) {
      const a = t * 3 + 1;
      out.push(`f ${a}//${a} ${a + 1}//${a + 1} ${a + 2}//${a + 2}`);
    }
    return out.join("\n") + "\n";
  }

  const ArsenalModels = {
    MODELS, build, ids, meta, has, toOBJ, palette: P,
    Mesh, primitives: { ring, trap, pent, bodyLoft, missile, dish, satellite, soldier },
  };
  if (typeof module === "object" && module.exports) module.exports = ArsenalModels;
  root.ArsenalModels = ArsenalModels;
})(typeof window !== "undefined" ? window : globalThis);
