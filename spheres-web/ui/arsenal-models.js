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
// Every triangle is flat-shaded from its own winding — no vertex normals are
// stored, and none are needed: faceting IS the look, and it makes a mesh half
// the size of one that carries them.
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
  function Mesh() {
    this.pos = [];
    this.col = [];
    this.m = IDENT.slice();
    this.stack = [];
  }
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
  Mesh.prototype.loft = function (sections, col, caps) {
    const n = sections[0].pts.length;
    for (let s = 0; s + 1 < sections.length; s += 1) {
      const a = sections[s], b = sections[s + 1];
      for (let i = 0; i < n; i += 1) {
        const j = (i + 1) % n;
        const lit = 1 + 0.16 * Math.sin((i / n) * Math.PI * 2 + 1.2);
        this.quad(
          [a.pts[i][0], a.pts[i][1], a.z], [a.pts[j][0], a.pts[j][1], a.z],
          [b.pts[j][0], b.pts[j][1], b.z], [b.pts[i][0], b.pts[i][1], b.z],
          shade(col, lit),
        );
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

  /// Triangle soup to buffers. Normals are computed here, per face, from the
  /// vertices AS TRANSFORMED — see the note on `Mesh`. A degenerate triangle
  /// (a cone tip, a zero-chord tip rib) gets +Y rather than a NaN, because one
  /// NaN in an attribute buffer takes the whole draw call with it.
  Mesh.prototype.finish = function () {
    const positions = new Float32Array(this.pos);
    const colors = new Float32Array(this.col);
    const normals = new Float32Array(positions.length);
    const min = [Infinity, Infinity, Infinity], max = [-Infinity, -Infinity, -Infinity];
    for (let i = 0; i < positions.length; i += 9) {
      const ax = positions[i], ay = positions[i + 1], az = positions[i + 2];
      const ux = positions[i + 3] - ax, uy = positions[i + 4] - ay, uz = positions[i + 5] - az;
      const vx = positions[i + 6] - ax, vy = positions[i + 7] - ay, vz = positions[i + 8] - az;
      let nx = uy * vz - uz * vy, ny = uz * vx - ux * vz, nz = ux * vy - uy * vx;
      const len = Math.hypot(nx, ny, nz);
      if (len > 1e-12) { nx /= len; ny /= len; nz /= len; } else { nx = 0; ny = 1; nz = 0; }
      for (let k = 0; k < 3; k += 1) {
        normals[i + k * 3] = nx; normals[i + k * 3 + 1] = ny; normals[i + k * 3 + 2] = nz;
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
  function bodyLoft(m, o) {
    const seg = o.seg || 10;
    const secs = o.stations.map((s) => ({
      z: s[0] * o.len,
      pts: ring(
        Math.max(1e-3, s[1] * o.w / 2),
        Math.max(1e-3, (s[2] == null ? s[1] : s[2]) * (o.h == null ? o.w : o.h) / 2),
        seg,
      ),
    }));
    m.loft(secs, o.col, o.caps);
  }

  const JET_STATIONS = [[0, .72, .74], [.07, 1, 1], [.4, 1, 1], [.62, .96, .92],
    [.82, .66, .68], [.94, .34, .36], [1, .06, .06]];

  /// The fighter family. Nine of the deck's aircraft are this function with a
  /// different table: what separates an F-15E from an F-22 at the size a card
  /// draws them is planform, tail cant and how many facets the nose has, and
  /// all three are arguments. What it does NOT cover is the three aircraft
  /// whose whole identity is that they are not this shape — the F-117, the B-2
  /// and the RQ-170 — which are built below on their own.
  function jet(m, o) {
    const L = o.len, col = o.col, w = o.w, h = o.h;
    const seg = o.facet ? 6 : 10;
    bodyLoft(m, { len: L, w, h, col, seg, stations: o.stations || JET_STATIONS });

    // Wings, from the fuselage side out. The root chord is placed by its
    // leading edge so a delta and a swept panel are the same two numbers.
    m.both((mm) => {
      mm.wing({
        x0: w * 0.44, y: o.wingY == null ? -h * 0.1 : o.wingY, z: o.wingZ,
        span: o.span / 2 - w * 0.44, root: o.root, tip: o.tip, sweep: o.sweep,
        thick: h * 0.14, dihedral: o.dihedral || 0,
      }, col);
    });

    // Horizontal stabilisers. Absent on the tailless designs, where the wing
    // trailing edge carries the control surfaces instead.
    if (o.stab) {
      m.both((mm) => {
        mm.wing({
          x0: w * 0.40, y: o.stab.y || -h * 0.05, z: o.stab.z,
          span: o.stab.span, root: o.stab.root, tip: o.stab.tip, sweep: o.stab.sweep,
          thick: h * 0.09, dihedral: o.stab.dihedral || 0,
        }, shade(col, 0.96));
      });
    }

    // Fins. `cant` is the outward lean that distinguishes a low-observable
    // twin tail from a conventional one at a glance, and it is the only part of
    // the F-22's shape a 90px card can actually show.
    const fin = o.fin;
    if (fin && fin.kind !== "none") {
      const sweep = fin.sweep == null ? fin.root * 0.7 : fin.sweep;
      const shape = [[0, 0], [fin.height, -sweep], [fin.height, -sweep - fin.tip], [0, -fin.root]];
      // rotZ(90) stands a planform on its edge; the cant is taken off that
      // angle, and `both` gives the port fin the opposite lean for free.
      const one = (mm, x) => {
        mm.save().move(x, fin.y == null ? h * 0.3 : fin.y, fin.z).rotZ(90 - (fin.cant || 0));
        mm.plate(shape, h * 0.08, shade(col, 1.02));
        mm.restore();
      };
      if (fin.kind === "twin") m.both((mm) => one(mm, fin.x == null ? w * 0.32 : fin.x));
      else one(m, 0);
    }

    // Exhaust. Two nozzles or one, sunk into the tail so the dark ring reads
    // as a hole rather than a stub.
    const nz = o.nozzles == null ? 2 : o.nozzles;
    for (let i = 0; i < nz; i += 1) {
      const x = nz === 1 ? 0 : (i === 0 ? 1 : -1) * w * 0.22;
      m.save().move(x, -h * 0.02, -0.1);
      m.tube(h * 0.29, h * 0.24, L * 0.06, 8, P.exhaust);
      m.tube(h * 0.21, h * 0.21, L * 0.03, 8, P.black);
      m.restore();
    }

    // Canopy. Cockpit glass is the one place a saturated colour is allowed:
    // it is what tells you which end is the front.
    if (o.canopy !== false) {
      const cz = o.canopyZ == null ? L * 0.66 : o.canopyZ;
      m.save().move(0, h * 0.42, cz);
      bodyLoft(m, {
        len: L * 0.19, w: w * 0.55, h: h * 0.34, col: P.canopy, seg: 6,
        stations: [[0, .3, .3], [.25, 1, 1], [.7, .9, .8], [1, .18, .2]],
      });
      m.restore();
    }

    // Intakes, and they are diagnostic: a nose inlet is 1960s, cheek inlets
    // are 1970s, a chin inlet is an F-16, a diverterless bump is an F-35.
    if (o.intake === "nose") {
      m.save().move(0, 0, L * 0.955).rotY(0);
      m.tube(h * 0.16, h * 0.05, L * 0.05, 8, P.black);
      m.restore();
    } else if (o.intake === "side" || o.intake === "dsi") {
      m.both((mm) => {
        mm.save().move(w * 0.5, -h * 0.12, o.intakeZ == null ? L * 0.52 : o.intakeZ);
        mm.bar(-w * 0.06, w * 0.16, -h * 0.16, h * 0.16, 0, L * 0.16, shade(col, 0.88));
        mm.save().move(0, 0, L * 0.16);
        mm.tube(h * 0.15, h * 0.15, 0.02, 6, P.black);
        mm.restore();
        mm.restore();
      });
    } else if (o.intake === "chin") {
      m.save().move(0, -h * 0.42, L * 0.58);
      m.bar(-w * 0.28, w * 0.28, -h * 0.1, h * 0.14, 0, L * 0.14, shade(col, 0.88));
      m.restore();
    }
    return m;
  }

  // --------------------------------------------------------- ground vehicles
  /// A hull cross-section: wide at the belly, tapered at the roof, which is the
  /// shape every armoured vehicle in this file has in section.
  function trap(halfW, y0, y1, taper) {
    return [[halfW, y0], [halfW * taper, y1], [-halfW * taper, y1], [-halfW, y0]];
  }

  /// A track run: the box, the sprockets at each end and the road wheels
  /// between them. Wheels are cheap and they are what stops a tracked hull
  /// reading as a crate — at 90px the eye finds the wheel line before it finds
  /// the gun.
  function trackRun(m, o) {
    const y0 = o.y0, y1 = o.y1, z0 = o.z0, z1 = o.z1, x = o.x, t = o.thick;
    m.bar(x - t / 2, x + t / 2, y0 + o.r * 0.5, y1, z0, z1, P.track);
    const wheels = o.wheels == null ? 6 : o.wheels;
    const cy = y0 + o.r;
    for (let i = 0; i < wheels; i += 1) {
      const z = z0 + o.r + (i / Math.max(1, wheels - 1)) * (z1 - z0 - o.r * 2);
      m.save().move(x - t * 0.62, cy, z).rotY(90);
      m.tube(o.r, o.r, t * 1.24, 9, shade(P.track, 1.5));
      m.restore();
    }
    // Sprocket and idler sit a little higher and larger, as they do.
    [[z0 + o.r * 0.7, 1.18], [z1 - o.r * 0.7, 1.18]].forEach((s) => {
      m.save().move(x - t * 0.7, cy + o.r * 0.25, s[0]).rotY(90);
      m.tube(o.r * s[1], o.r * s[1], t * 1.4, 9, shade(P.track, 1.25));
      m.restore();
    });
  }

  /// The armour family: hull, tracks, turret, gun. Three of the deck's entries
  /// are this call — second-generation armour is rounded and low, third is
  /// angular and long-gunned, Trophy is third with the panels bolted on — and
  /// the mechanised infantry formation is the same hull with a small turret.
  function armour(m, o) {
    const col = o.col, L = o.len, W = o.w, H = o.h;
    const belly = o.belly == null ? H * 0.3 : o.belly;
    const roof = belly + H * 0.55;
    m.loft([
      { z: 0, pts: trap(W / 2 * 0.94, belly, roof, 0.92) },
      { z: L * 0.12, pts: trap(W / 2, belly, roof, 0.92) },
      { z: L * 0.72, pts: trap(W / 2, belly, roof, 0.92) },
      { z: L * 0.86, pts: trap(W / 2, belly, roof - H * 0.1, 0.9) },
      { z: L, pts: trap(W / 2 * 0.9, belly + H * 0.16, roof - H * 0.26, 0.86) },
    ], col);
    m.both((mm) => trackRun(mm, {
      x: W / 2 * 0.97, thick: W * 0.17, r: H * 0.2, y0: 0, y1: belly + H * 0.12,
      z0: L * 0.02, z1: L * 0.98, wheels: o.wheels,
    }));
    if (o.skirt) {
      m.both((mm) => mm.bar(W / 2 * 0.86, W / 2 * 1.02, belly - H * 0.02, belly + H * 0.2,
        L * 0.08, L * 0.8, shade(col, 0.92)));
    }

    const t = o.turret;
    if (!t) return m;
    const ty = roof, tz = o.turretZ == null ? L * 0.42 : o.turretZ;
    m.save().move(0, ty, tz);
    if (t.round) {
      // A cast turret is a squashed dome. Ten segments, because at eight the
      // dome starts to look like a nut and the whole point of gen-2 armour is
      // that its turret is round.
      m.save().scale(1, 0.62, 1.25).rotX(-90);
      m.ball(t.w / 2, 10, 5, shade(col, 1.04));
      m.restore();
    } else {
      m.loft([
        { z: -t.len / 2, pts: trap(t.w / 2 * 0.8, 0, t.h, 0.78) },
        { z: 0, pts: trap(t.w / 2, 0, t.h, 0.8) },
        { z: t.len * 0.3, pts: trap(t.w / 2, 0, t.h, 0.8) },
        { z: t.len / 2, pts: trap(t.w / 2 * 0.62, 0, t.h * 0.78, 0.8) },
      ], shade(col, 1.05));
    }
    // Mantlet then barrel, and the barrel runs past the hull nose on a modern
    // tank. That overhang is the silhouette.
    const gy = t.round ? t.h * 0.34 : t.h * 0.55;
    m.save().move(0, gy, t.len * 0.4);
    m.tube(t.h * 0.3, t.h * 0.26, t.gunLen * 0.1, 8, shade(col, 0.9));
    m.tube(t.gunR, t.gunR * 0.92, t.gunLen, 9, P.exhaust);
    if (t.brake) {
      m.save().move(0, 0, t.gunLen * 0.86);
      m.tube(t.gunR * 1.5, t.gunR * 1.5, t.gunLen * 0.1, 8, shade(P.exhaust, 0.9));
      m.restore();
    }
    m.restore();
    if (t.mg) {
      m.save().move(t.w * 0.28, t.h, -t.len * 0.1);
      m.bar(-0.09, 0.09, 0, 0.22, -0.12, 0.12, P.black);
      m.save().move(0, 0.2, 0);
      m.tube(0.04, 0.04, 0.85, 6, P.black);
      m.restore();
      m.restore();
    }
    if (t.aps) {
      // Trophy's four radar faces and the two launchers, which is the entire
      // visible difference between a protected tank and an unprotected one.
      m.both((mm) => {
        mm.save().move(t.w * 0.5, t.h * 0.62, t.len * 0.1).rotY(28).rotZ(-14);
        mm.bar(-0.04, 0.04, -0.26, 0.26, -0.3, 0.3, P.sensor);
        mm.restore();
        mm.save().move(t.w * 0.46, t.h * 0.9, -t.len * 0.18).rotZ(-24);
        mm.bar(-0.14, 0.14, -0.1, 0.1, -0.18, 0.18, shade(col, 0.8));
        mm.restore();
      });
    }
    m.restore();
    return m;
  }

  /// Wheels on an axle line, for everything that is a truck underneath: a
  /// missile transporter-erector, a Patriot battery, a counter-UAS mount, a
  /// command post.
  function wheels(m, o) {
    const r = o.r, n = o.axles, z0 = o.z0, z1 = o.z1;
    for (let i = 0; i < n; i += 1) {
      const z = n === 1 ? z0 : z0 + (i / (n - 1)) * (z1 - z0);
      m.both((mm) => {
        mm.save().move(o.x - o.w * 0.5, r, z).rotY(90);
        mm.tube(r, r, o.w, 10, P.rubber);
        mm.save().move(0, 0, o.w * 0.5);
        mm.tube(r * 0.45, r * 0.45, o.w * 0.55, 8, shade(P.metal, 0.7));
        mm.restore();
        mm.restore();
      });
    }
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
    wheels(m, { r, w: W * 0.16, x: W / 2 * 0.99, axles: o.axles, z0: r * 1.3, z1: L - o.cab * 0.62 });
    return bedY;
  }

  // ------------------------------------------------------------------ ships
  /// A hull as a plan outline swept in Y, built twice: once below the waterline
  /// in anti-fouling dark, once above it in haze grey. Two bands rather than
  /// one because the boot topping is then a consequence of the geometry instead
  /// of a texture, and this file has no textures.
  function hullBand(m, o, y0, y1, col, taper) {
    const L = o.len, B = o.beam;
    const w = (f) => B / 2 * f;
    m.loft([
      { z: 0, pts: trap(w(0.72), y0, y1, taper) },
      { z: L * 0.08, pts: trap(w(0.94), y0, y1, taper) },
      { z: L * 0.35, pts: trap(w(1), y0, y1, taper) },
      { z: L * 0.62, pts: trap(w(0.97), y0, y1, taper) },
      { z: L * 0.85, pts: trap(w(0.7), y0, y1 + (o.sheer || 0), taper) },
      { z: L * 0.96, pts: trap(w(0.32), y0, y1 + (o.sheer || 0) * 1.6, taper) },
      { z: L, pts: trap(w(0.05), y0 + (y1 - y0) * 0.45, y1 + (o.sheer || 0) * 1.8, taper) },
    ], col);
  }
  function ship(m, o) {
    const L = o.len, B = o.beam, fb = o.freeboard;
    hullBand(m, o, -o.draft, 0, P.hull, 0.55);
    hullBand(m, o, 0, fb, o.col || P.navy, 0.97);
    // Deck houses, given as [z0, z1, widthFactor, height] up the stack, which
    // is how a warship's profile is actually described.
    (o.houses || []).forEach((h) => {
      m.loft([
        { z: L * h[0], pts: trap(B / 2 * h[2], fb, fb + h[3], 0.94) },
        { z: L * (h[0] + (h[1] - h[0]) * 0.85), pts: trap(B / 2 * h[2], fb, fb + h[3], 0.94) },
        { z: L * h[1], pts: trap(B / 2 * h[2] * 0.9, fb, fb + h[3] * 0.92, 0.94) },
      ], shade(o.col || P.navy, 1.07));
    });
    return m;
  }
  /// A mast: a tapered pole, a yard and an air-search array. Radars are what a
  /// warship is for, so every surface combatant in the deck carries one.
  function mast(m, o) {
    m.save().move(0, o.y, o.z).rotX(-90);
    m.tube(o.r, o.r * 0.4, o.h, 6, P.metal);
    m.restore();
    m.save().move(0, o.y + o.h * 0.62, o.z);
    m.bar(-o.h * 0.16, o.h * 0.16, -o.r * 0.4, o.r * 0.4, -o.r * 0.4, o.r * 0.4, P.metal);
    m.restore();
    if (o.radar) {
      m.save().move(0, o.y + o.h * 0.88, o.z).rotY(22);
      m.bar(-o.h * 0.22, o.h * 0.22, -o.h * 0.12, o.h * 0.12, -0.05, 0.05, P.white);
      m.restore();
    }
  }
  /// A single-barrel gun in a shielded mount, laid forward.
  function deckGun(m, o) {
    m.save().move(0, o.y, o.z);
    m.loft([
      { z: -o.r, pts: trap(o.r, 0, o.r * 1.1, 0.7) },
      { z: o.r * 0.9, pts: trap(o.r * 0.8, 0, o.r, 0.6) },
    ], P.white);
    m.save().move(0, o.r * 0.62, o.r * 0.8);
    m.tube(o.r * 0.16, o.r * 0.13, o.r * 2.6, 7, P.metal);
    m.restore();
    m.restore();
  }
  /// A vertical launch system as what it looks like from above: a raft of dark
  /// cell hatches let into the deck.
  function vls(m, o) {
    const cols = o.cols || 4, rows = o.rows || 4;
    m.bar(-o.w / 2, o.w / 2, o.y, o.y + o.h * 0.3, o.z0, o.z1, shade(P.navy, 0.9));
    for (let r = 0; r < rows; r += 1) {
      for (let c = 0; c < cols; c += 1) {
        const x = -o.w / 2 + (o.w / cols) * (c + 0.5);
        const z = o.z0 + ((o.z1 - o.z0) / rows) * (r + 0.5);
        const cw = (o.w / cols) * 0.62, cd = ((o.z1 - o.z0) / rows) * 0.62;
        m.bar(x - cw / 2, x + cw / 2, o.y + o.h * 0.3, o.y + o.h * 0.34, z - cd / 2, z + cd / 2, P.black);
      }
    }
  }

  /// The submarine family. A modern hull is a body of revolution with a
  /// parallel mid-body, so it is one loft plus the four things that break the
  /// cylinder: the sail, the bow planes, the stern surfaces and the screw.
  function submarine(m, o) {
    const L = o.len, D = o.dia, col = o.col || P.hull;
    bodyLoft(m, {
      len: L, w: D, h: D, col, seg: 12,
      stations: [[0, .12, .12], [.04, .5, .5], [.12, .86, .86], [.28, 1, 1],
        [.74, 1, 1], [.9, .82, .82], [1, .1, .1]],
    });
    m.save().move(0, D * 0.42, L * 0.62);
    m.loft([
      { z: -o.sailLen / 2, pts: trap(D * 0.13, 0, o.sailH, 0.8) },
      { z: o.sailLen * 0.1, pts: trap(D * 0.15, 0, o.sailH, 0.8) },
      { z: o.sailLen / 2, pts: trap(D * 0.1, 0, o.sailH * 0.88, 0.8) },
    ], shade(col, 1.25));
    if (o.sailPlanes) {
      m.both((mm) => {
        mm.save().move(D * 0.12, o.sailH * 0.55, 0);
        mm.plate([[0, o.sailLen * 0.22], [D * 0.62, o.sailLen * 0.16],
          [D * 0.62, -o.sailLen * 0.16], [0, -o.sailLen * 0.24]], D * 0.05, shade(col, 1.15));
        mm.restore();
      });
    }
    // Two masts up. A sail drawn bare reads as a fin, and the periscope line is
    // what says this is a boat that is looking at something.
    m.save().move(0, o.sailH, 0);
    m.tube(D * 0.028, D * 0.02, o.sailH * 0.5, 6, P.metal);
    m.move(D * 0.07, 0, -o.sailLen * 0.16);
    m.tube(D * 0.022, D * 0.016, o.sailH * 0.34, 6, P.metal);
    m.restore();
    m.restore();
    if (!o.sailPlanes) {
      m.both((mm) => {
        mm.save().move(D * 0.45, 0, L * 0.8);
        mm.plate([[0, L * 0.03], [D * 0.58, L * 0.02], [D * 0.58, -L * 0.03], [0, -L * 0.04]],
          D * 0.06, shade(col, 1.14));
        mm.restore();
      });
    }
    // Stern surfaces: cruciform on the nuclear boat, X-planes on the AIP one,
    // which is the same four plates rolled 45 degrees.
    for (let i = 0; i < 4; i += 1) {
      m.save().move(0, 0, L * 0.9).rotZ((o.xstern ? 45 : 0) + i * 90);
      m.plate([[D * 0.18, L * 0.045], [D * 0.72, L * 0.02], [D * 0.72, -L * 0.03],
        [D * 0.18, -L * 0.055]], D * 0.055, shade(col, 1.16));
      m.restore();
    }
    m.save().move(0, 0, L * 0.965);
    m.tube(D * 0.12, D * 0.04, L * 0.03, 8, shade(col, 0.9));
    for (let i = 0; i < 7; i += 1) {
      m.save().rotZ(i * (360 / 7)).move(0, 0, L * 0.008).rotY(-26);
      m.plate([[D * 0.05, L * 0.013], [D * 0.3, L * 0.019], [D * 0.32, -L * 0.013],
        [D * 0.05, -L * 0.015]], D * 0.02, P.metal);
      m.restore();
    }
    m.restore();
    return m;
  }

  // --------------------------------------------------------------- ordnance
  /// A missile, a bomb, an interceptor or a glide body: one tube, one nose and
  /// as many fin sets as it has. Eleven entries in the deck come through here.
  function missile(m, o) {
    const L = o.len, R = o.r, col = o.col || P.missile;
    const nose = o.nose == null ? L * 0.16 : o.nose;
    bodyLoft(m, {
      len: L, w: R * 2, h: R * 2, col, seg: o.seg || 10,
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
  /// A spacecraft: a bus, two solar wings, and whatever it is for on the front.
  /// The wings are drawn bay by bay so an array reads as an array rather than
  /// as a blue slab.
  function satellite(m, o) {
    const B = o.bus;
    m.bar(-B[0] / 2, B[0] / 2, -B[1] / 2, B[1] / 2, -B[2] / 2, B[2] / 2, o.col || P.gold);
    const bays = o.bays || 3;
    const inner = B[0] / 2 + o.span * 0.06;
    const bayLen = (o.span / 2 - inner) / bays;
    m.both((mm) => {
      mm.save().move(B[0] / 2, 0, 0).rotX(90);
      mm.tube(o.span * 0.012, o.span * 0.012, o.span * 0.06, 6, P.metal);
      mm.restore();
      for (let i = 0; i < bays; i += 1) {
        const x0 = inner + i * bayLen, x1 = x0 + bayLen * 0.93;
        mm.plate([[x0, o.panelW / 2], [x1, o.panelW / 2], [x1, -o.panelW / 2], [x0, -o.panelW / 2]],
          o.span * 0.006, i % 2 ? shade(P.panel, 1.2) : P.panel);
      }
    });
    return m;
  }

  // ------------------------------------------------------- people & sensors
  /// One soldier, about 1.8 m. Infantry formations are the only entries in the
  /// deck whose unit is people, and a card that draws them as a vehicle is
  /// lying about what the money buys.
  function soldier(m, o) {
    const c = o.col || P.cloth;
    m.save().move(o.x || 0, 0, o.z || 0).rotY(o.face || 0);
    m.bar(-0.11, 0.11, 0, 0.44, -0.07, 0.07, shade(c, 0.86));
    m.bar(-0.24, -0.02, 0, 0.44, -0.07, 0.07, shade(c, 0.92));
    m.loft([
      { z: -0.11, pts: trap(0.2, 0.44, 1.12, 0.86) },
      { z: 0.11, pts: trap(0.2, 0.44, 1.12, 0.86) },
    ], c);
    m.bar(-0.26, 0.26, 0.92, 1.15, -0.14, 0.03, shade(c, 1.08));
    m.bar(-0.1, 0.1, 1.15, 1.34, -0.09, 0.09, P.skin);
    m.save().move(0, 1.42, 0).scale(1, 0.8, 1.05);
    m.ball(0.14, 8, 4, shade(c, 1.15));
    m.restore();
    m.save().move(0.14, 1.02, 0.1).rotX(84);
    m.bar(-0.03, 0.03, -0.02, 0.02, -0.2, 0.62, P.black);
    m.bar(-0.02, 0.02, -0.09, -0.02, 0.02, 0.16, P.black);
    m.restore();
    m.restore();
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

  /// A pylon and a store under a wing, which is what turns a fighter into a
  /// strike fighter and is the only difference a card can show between them.
  function store(m, x, y, z, len, r, col) {
    m.save().move(x, y, z);
    m.bar(-r * 0.3, r * 0.3, 0, r * 1.5, -len * 0.16, len * 0.16, P.greyDark);
    m.save().move(0, -r, -len / 2);
    missile(m, { len, r, col: col || P.warhead, nose: len * 0.24, blunt: true, motor: false,
      fins: [{ n: 4, z: 0.1, span: r * 1.5, chord: len * 0.2, tip: len * 0.1, roll: 45 }] });
    m.restore();
    m.restore();
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
        soldier(m, { x: -1.05, z: 0.1, face: 10 });
        soldier(m, { x: 0.15, z: 1.15, face: -8, col: shade(P.cloth, 0.88) });
        soldier(m, { x: 1.15, z: -0.2, face: 16, col: shade(P.cloth, 1.1) });
        for (let i = 0; i < 4; i += 1) {
          m.save().move(-1.5 + i * 0.85, 0, -1.5).rotY(6 - i * 4);
          m.bar(-0.42, 0.42, 0, 0.3, -0.22, 0.22, shade(P.rust, 1 - i * 0.04));
          m.bar(-0.34, 0.34, 0.3, 0.56, -0.18, 0.18, shade(P.rust, 0.9 + i * 0.04));
          m.restore();
        }
      },
    },
    inf_mech: {
      name: "Mechanised Infantry Formation", cls: "Infantry", span: 6.7,
      build(m) {
        armour(m, {
          len: 6.7, w: 2.9, h: 2.1, col: P.olive, belly: 0.55, wheels: 5, turretZ: 4.2,
          turret: { w: 1.5, len: 1.5, h: 0.5, round: true, gunR: 0.055, gunLen: 1.1, mg: true },
        });
        // The ramp is the whole point of the vehicle: it is a bus that shoots.
        m.bar(-1.0, 1.0, 0.6, 1.7, -0.06, 0.06, shade(P.olive, 0.8));
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
        });
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
        });
        // Conformal fuel tanks along the intakes and four bombs under the wing.
        // The E is a two-seat bomb truck and that is the entire difference.
        m.both((mm) => {
          mm.bar(1.15, 1.75, -0.5, 0.85, 7.0, 12.6, shade(P.greyDark, 0.8));
          store(mm, 2.6, -0.55, 7.4, 3.4, 0.19);
          store(mm, 3.9, -0.35, 7.0, 3.4, 0.19);
        });
        m.save().move(0, 1.05, 11.0);
        m.bar(-0.5, 0.5, 0, 0.5, -1.2, 1.2, P.canopy);
        m.restore();
      },
    },
    f117: {
      name: "F-117 Nighthawk", cls: "Air", span: 20.1,
      build(m) {
        const L = 20.1;
        facetBody(m, {
          len: L, w: 4.6, h: 3.6, col: P.stealth,
          stations: [[0, .84, .9], [.08, .95, 1], [.34, 1, 1], [.62, .82, .82],
            [.86, .46, .5], [1, .1, .12]],
        });
        // The planform IS the aircraft. Sixty-seven degrees of leading-edge
        // sweep, straight from the nose to the tip, and a straight trailing
        // edge back to the tail.
        m.both((mm) => {
          mm.save().move(1.6, -0.35, 0);
          mm.plate([[0, L * 0.96], [6.6, L * 0.28], [6.6, L * 0.2], [0, L * 0.06]], 0.42, P.stealth);
          mm.restore();
          // V-tails, canted well out, which is the other half of the shape.
          mm.save().move(1.1, 0.5, L * 0.2).rotZ(90 - 42);
          mm.plate([[0, 2.6], [3.3, 0.6], [3.3, -0.2], [0, -1.0]], 0.24, shade(P.stealth, 1.12));
          mm.restore();
        });
        // Faceted canopy and the flat exhaust slot.
        m.save().move(0, 1.15, L * 0.6);
        facetBody(m, { len: 3.4, w: 2.2, h: 1.4, col: P.sensor,
          stations: [[0, .5, .5], [.4, 1, 1], [1, .3, .4]] });
        m.restore();
        m.bar(-2.2, 2.2, -0.35, 0.2, -0.3, 0.3, P.black);
      },
    },
    e3: {
      name: "E-3 Sentry AWACS", cls: "Air", span: 44.4,
      build(m) {
        const L = 46.6;
        bodyLoft(m, {
          len: L, w: 3.8, h: 4.0, col: P.white, seg: 12,
          stations: [[0, .3, .34], [.06, .82, .86], [.16, 1, 1], [.78, 1, 1],
            [.9, .84, .86], [.97, .5, .54], [1, .12, .14]],
        });
        m.both((mm) => {
          mm.wing({ x0: 1.7, y: -0.9, z: L * 0.52, span: 20.5, root: 8.4, tip: 2.4, sweep: 8.0,
            thick: 0.7, dihedral: 6 }, shade(P.white, 0.94));
          // Two engines a side, hung ahead of and below the wing, which is what
          // a 707 looks like and nothing else does.
          [[6.2, L * 0.5], [11.6, L * 0.44]].forEach((e, i) => {
            mm.save().move(e[0], -2.1 - i * 0.15, e[1]);
            mm.bar(-0.5, 0.5, 0.6, 1.5, -0.4, 2.2, P.greyDark);
            mm.tube(1.1, 1.05, 4.6, 10, shade(P.white, 0.9));
            mm.save().move(0, 0, 0.2);
            mm.tube(0.92, 0.92, 0.3, 10, P.black);
            mm.restore();
            mm.restore();
          });
          mm.wing({ x0: 1.2, y: 0.6, z: L * 0.1, span: 6.6, root: 4.4, tip: 1.6, sweep: 3.2,
            thick: 0.4, dihedral: 8 }, shade(P.white, 0.94));
        });
        m.save().move(0, 1.9, L * 0.06).rotZ(90 - 2);
        m.plate([[0, 4.4], [7.6, 1.2], [7.6, -1.0], [0, -1.6]], 0.4, shade(P.white, 1.05));
        m.restore();
        // The rotodome. Eleven metres across, on two struts, and the reason the
        // airframe is in the deck at all.
        m.save().move(0, 4.2, L * 0.33);
        m.both((mm) => {
          mm.save().move(1.5, -1.5, 0).rotZ(-8);
          mm.bar(-0.24, 0.24, 0, 1.7, -1.1, 1.1, P.greyDark);
          mm.restore();
        });
        m.save().rotX(-90).scale(1, 1, 0.16);
        m.tube(5.5, 5.5, 1.0, 14, P.white);
        m.restore();
        m.save().move(0, 0.2, 0).rotX(-90).scale(1, 1, 0.1);
        m.tube(5.6, 5.6, 0.6, 14, shade(P.red, 1.1));
        m.restore();
        m.restore();
      },
    },
    b2: {
      name: "B-2 Spirit", cls: "Air", span: 52.4,
      build(m) {
        const L = 21.0, half = 26.2;
        // A flying wing is authored as two convex panels a side: the outer
        // panel and the inner one, whose trailing edges step. `plate` is convex
        // only, and the step is how the sawtooth is honestly built rather than
        // faked with a texture this file does not have.
        m.both((mm) => {
          mm.plate([[0, L], [half * 0.55, L * 0.44], [half * 0.55, L * 0.1], [0, L * 0.02]],
            1.0, P.stealth);
          mm.plate([[half * 0.55, L * 0.44], [half, L * 0.06], [half, -L * 0.02],
            [half * 0.55, L * 0.1]], 0.55, shade(P.stealth, 0.94));
        });
        facetBody(m, {
          len: L, w: 9.0, h: 3.4, col: shade(P.stealth, 1.08),
          stations: [[.02, .7, .5], [.16, 1, 1], [.5, .96, .96], [.78, .6, .62], [1, .12, .16]],
        });
        // Buried intakes and the shielded exhaust troughs.
        m.both((mm) => {
          mm.save().move(2.6, 1.1, L * 0.52);
          mm.bar(-1.1, 1.1, 0, 0.7, -1.4, 1.4, shade(P.stealth, 1.2));
          mm.bar(-0.85, 0.85, 0.7, 0.78, -1.0, 1.0, P.black);
          mm.restore();
          mm.bar(1.4, 4.4, 0.2, 0.5, L * 0.06, L * 0.16, P.black);
        });
        m.save().move(0, 1.5, L * 0.78);
        m.bar(-1.5, 1.5, 0, 0.55, -1.3, 1.3, P.canopy);
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
        });
        // Chines. The straight line from the radome to the intake lip is the
        // single most recognisable thing about the aircraft after the tails.
        m.both((mm) => {
          mm.save().move(1.0, 0.1, 0);
          mm.plate([[0, 17.2], [1.5, 11.0], [1.5, 9.6], [0, 12.0]], 0.2, shade(P.stealth, 1.18));
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
        });
        // Five jamming pods, and they are the aircraft: a Growler is a Super
        // Hornet that carries transmitters where the bombs would be.
        m.both((mm) => {
          [[2.9, 8.4], [4.6, 8.0]].forEach((p) => {
            mm.save().move(p[0], -0.7, p[1]);
            mm.bar(-0.28, 0.28, -0.1, 0.6, -1.9, 1.9, P.white);
            mm.save().move(0, -0.1, 1.9).rotX(0);
            mm.tube(0.28, 0.1, 0.7, 8, shade(P.white, 0.9));
            mm.restore();
            mm.save().move(0, -0.1, -1.9);
            mm.tube(0.28, 0.16, -0.6, 8, P.sensor);
            mm.restore();
            mm.restore();
          });
          mm.wing({ x0: 1.35, y: 0.15, z: 12.6, span: 1.0, root: 3.4, tip: 1.4, sweep: 2.6,
            thick: 0.2 }, shade(P.grey, 1.05));
        });
        m.save().move(0, -0.9, 9.0);
        m.bar(-0.3, 0.3, -0.2, 0.6, -1.9, 1.9, P.white);
        m.restore();
      },
    },
    rq170: {
      name: "RQ-170 Sentinel", cls: "Air", span: 20.0,
      build(m) {
        const L = 4.5, half = 10.0;
        m.both((mm) => {
          mm.plate([[0, L], [half * 0.5, L * 0.5], [half * 0.5, L * 0.06], [0, -L * 0.12]],
            0.42, P.stealthLit);
          mm.plate([[half * 0.5, L * 0.5], [half, L * 0.12], [half, -L * 0.04],
            [half * 0.5, L * 0.06]], 0.26, shade(P.stealthLit, 0.94));
        });
        facetBody(m, {
          len: L, w: 3.0, h: 1.3, col: P.stealthLit,
          stations: [[0, .68, .6], [.2, 1, 1], [.62, .9, .92], [1, .24, .3]],
        });
        m.save().move(0, 0.5, L * 0.5);
        m.bar(-0.8, 0.8, 0, 0.34, -0.9, 0.9, shade(P.stealthLit, 1.15));
        m.bar(-0.6, 0.6, 0.34, 0.38, -0.6, 0.6, P.black);
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
        });
        // The diverterless bump. It is a lump on the side of an intake and it
        // is also the reason this airframe cost what it cost.
        m.both((mm) => {
          mm.save().move(1.45, -0.25, 9.6).scale(0.7, 0.9, 1.4);
          mm.ball(0.62, 8, 5, shade(P.stealth, 1.2));
          mm.restore();
        });
        m.save().move(0, -1.05, 8.0);
        m.bar(-0.5, 0.5, 0, 0.35, -1.1, 1.1, P.black);
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
          nozzles: 1, intake: "none", canopy: false,
        });
        // No cockpit, a dorsal inlet, and a size that says this one is bought
        // by the dozen. All three are the point of the programme.
        m.save().move(0, 0.6, 5.0);
        m.bar(-0.5, 0.5, 0, 0.42, -0.9, 0.9, shade(P.stealthLit, 1.16));
        m.bar(-0.38, 0.38, 0.42, 0.46, -0.6, 0.6, P.black);
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
          fin: { kind: "none" },
        });
        // Tailless, so the wing's trailing edge does the work — and a deep body,
        // because range is the requirement this generation is actually built to.
        m.both((mm) => {
          mm.save().move(1.2, 0.2, 0);
          mm.plate([[0, 20.0], [2.0, 12.6], [2.0, 11.0], [0, 13.8]], 0.24, shade(P.stealth, 1.2));
          mm.restore();
        });
      },
    },
    aesa: {
      name: "AESA Radar Refit", cls: "Air", span: 1.4,
      build(m) {
        // Not a platform: the thing that gets bolted into one. A tilted array
        // face on its bulkhead frame, with the liquid-cooled back end behind it.
        m.bar(-0.62, 0.62, 0, 0.12, -0.5, 0.5, P.greyDark);
        m.save().move(0, 0.12, 0);
        m.bar(-0.1, 0.1, 0, 0.5, -0.08, 0.08, P.metal);
        m.restore();
        m.save().move(0, 0.78, 0);
        arrayFace(m, { r: 0.62, tilt: 18, col: P.sensor });
        m.restore();
        m.save().move(0, 0.62, -0.34).rotX(18);
        m.bar(-0.44, 0.44, -0.3, 0.3, -0.34, 0, shade(P.greyDark, 0.9));
        m.restore();
      },
    },
  };

  /// The Predator airframe, twice: unarmed as the RQ-1 and with two Hellfires
  /// as the MQ-1B. They are the same aircraft and the deck prices them apart
  /// only because of what hangs under the wing, so the models differ only there.
  function predatorAirframe(m, armed) {
    const L = 8.22, col = shade(P.white, 0.92);
    bodyLoft(m, {
      len: L, w: 0.9, h: 1.0, col, seg: 10,
      stations: [[0, .3, .3], [.1, .62, .66], [.34, .8, .84], [.62, .92, 1.0],
        [.86, 1, 1.15], [1, .5, .72]],
    });
    m.both((mm) => {
      mm.wing({ x0: 0.4, y: 0.35, z: L * 0.52, span: 6.9, root: 0.9, tip: 0.55, sweep: 0.12,
        thick: 0.12, dihedral: 2 }, col);
      // The inverted V-tail, which is the silhouette everybody knows.
      mm.save().move(0.28, 0.1, L * 0.1).rotZ(-(90 - 43));
      mm.plate([[0, 1.5], [2.1, 0.5], [2.1, 0.05], [0, -0.1]], 0.1, col);
      mm.restore();
      if (armed) {
        mm.save().move(1.9, 0.24, L * 0.5);
        mm.bar(-0.05, 0.05, -0.28, 0, -0.16, 0.16, P.greyDark);
        mm.save().move(0, -0.28, -0.8);
        missile(m, { len: 1.63, r: 0.09, col: P.warhead, seeker: true, nose: 0.3,
          fins: [{ n: 4, z: 0.06, span: 0.16, chord: 0.3, tip: 0.16, roll: 45 },
            { n: 4, z: 0.6, span: 0.14, chord: 0.22, tip: 0.14 }] });
        mm.restore();
        mm.restore();
      }
    });
    // Pusher prop and the SATCOM hump, in that order of importance.
    m.save().move(0, 0.16, -0.05).rotY(0);
    for (let i = 0; i < 2; i += 1) {
      m.save().rotZ(i * 90 + 18);
      m.plate([[0.05, 0.06], [0.86, 0.08], [0.86, -0.08], [0.05, -0.06]], 0.04, P.black);
      m.restore();
    }
    m.restore();
    m.save().move(0, 0.52, L * 0.86).scale(1, 0.72, 1.2);
    m.ball(0.42, 10, 5, shade(P.white, 1.0));
    m.restore();
    m.save().move(0, -0.32, L * 0.78).scale(1, 0.9, 1);
    m.ball(0.28, 8, 4, P.sensor);
    m.restore();
  }

  // A parked aircraft on a carrier deck, at the four triangles it is worth. A
  // real `jet` call costs five hundred triangles and there are eight of them up
  // there; at the size a task group draws, this is the same picture.
  function deckPlane(m, x, z, face, col) {
    m.save().move(x, 0, z).rotY(face);
    m.plate([[0, 8], [5.5, -1.5], [5.5, -3.4], [0, -6]], 0.5, col);
    m.plate([[-5.5, -1.5], [0, 8], [0, -6], [-5.5, -3.4]], 0.5, col);
    m.bar(-0.9, 0.9, 0.25, 1.5, -6, 8, shade(col, 1.1));
    m.save().move(0, 1.4, -3.6).rotZ(90);
    m.plate([[0, 2.2], [2.4, 0.2], [2.4, -0.6], [0, -1.2]], 0.16, shade(col, 1.15));
    m.restore();
    m.restore();
  }

  Object.assign(MODELS, {
    // ----------------------------------------------------------------- naval
    nav_patrol: {
      name: "Patrol and Coastal Craft", cls: "Naval", span: 35,
      build(m) {
        ship(m, {
          len: 35, beam: 7.2, draft: 1.9, freeboard: 2.4, sheer: 0.9, col: shade(P.navy, 0.94),
          houses: [[0.3, 0.62, 0.68, 2.6]],
        });
        m.bar(-1.4, 1.4, 5.0, 5.3, 35 * 0.34, 35 * 0.56, shade(P.navy, 1.12));
        m.bar(-1.1, 1.1, 3.6, 4.9, 35 * 0.36, 35 * 0.55, P.glass);
        mast(m, { y: 5.3, z: 35 * 0.42, r: 0.16, h: 4.6, radar: true });
        deckGun(m, { y: 2.4, z: 35 * 0.78, r: 0.8 });
        // Two rigid inflatables on the after deck, because a patrol craft's job
        // is boarding and that is what it is carrying to do it with.
        m.both((mm) => {
          mm.save().move(1.9, 2.6, 35 * 0.2).rotY(2);
          mm.loft([{ z: -2.2, pts: trap(0.5, 0, 0.6, 0.8) }, { z: 2.2, pts: trap(0.7, 0, 0.7, 0.8) }],
            shade(P.black, 1.6));
          mm.restore();
        });
      },
    },
    nav_escort: {
      name: "Escort Frigate or Destroyer", cls: "Naval", span: 133,
      build(m) {
        const L = 133;
        ship(m, {
          len: L, beam: 14.6, draft: 4.6, freeboard: 6.2, sheer: 2.0,
          houses: [[0.24, 0.66, 0.72, 7.2], [0.3, 0.56, 0.52, 4.4], [0.34, 0.48, 0.34, 3.0]],
        });
        deckGun(m, { y: 6.2, z: L * 0.79, r: 2.0 });
        vls(m, { w: 6.4, y: 6.2, h: 1.2, z0: L * 0.66, z1: L * 0.74, cols: 4, rows: 6 });
        mast(m, { y: 13.4, z: L * 0.52, r: 0.6, h: 11.0, radar: true });
        // Funnel, hangar and pad. A frigate without a flight deck is a frigate
        // that cannot do the thing frigates are bought for.
        m.save().move(0, 13.4, L * 0.36).rotX(-8);
        m.loft([{ z: -3.4, pts: trap(2.6, 0, 5.4, 0.8) }, { z: 3.0, pts: trap(2.4, 0, 5.0, 0.8) }],
          shade(P.navy, 0.9));
        m.restore();
        m.loft([{ z: L * 0.12, pts: trap(5.4, 6.2, 12.6, 0.94) },
          { z: L * 0.22, pts: trap(5.6, 6.2, 12.6, 0.94) }], shade(P.navy, 1.04));
        m.bar(-6.0, 6.0, 6.2, 6.5, L * 0.02, L * 0.12, P.navyDeck);
        m.save().move(0, 6.55, L * 0.07).rotX(-90).scale(1, 1, 0.02);
        m.tube(4.2, 4.2, 1, 16, shade(P.white, 0.95));
        m.restore();
        // The two things every modern escort shows above the hangar: a pair of
        // close-in mounts and the after array face.
        m.save().move(0, 12.8, L * 0.2);
        m.tube(1.0, 0.9, 1.4, 10, P.white);
        m.save().move(0, 0.4, 1.4).rotX(-70);
        m.tube(0.5, 0.42, 1.6, 8, P.metal);
        m.restore();
        m.restore();
        m.both((mm) => {
          mm.save().move(3.9, 10.2, L * 0.6);
          arrayFace(mm, { r: 1.7, tilt: 12, squash: 0.9 });
          mm.restore();
        });
      },
    },
    nav_blue: {
      name: "Blue-Water Task Group", cls: "Naval", span: 333,
      build(m) {
        const L = 333;
        ship(m, { len: L, beam: 41, draft: 11.3, freeboard: 17.0, sheer: 2.0, col: shade(P.navy, 0.9) });
        // The flight deck is wider than the hull, overhangs to port, and is the
        // reason the ship exists; everything else on it is furniture.
        m.plate([[38.5, L * 0.02], [38.5, L * 0.86], [24, L * 0.99], [-24, L * 0.99],
          [-38.5, L * 0.86], [-38.5, L * 0.02]], 1.6, P.navyDeck);
        m.save().move(-6, 1.5, L * 0.45).rotY(9);
        m.plate([[-11, -L * 0.3], [11, -L * 0.3], [11, L * 0.3], [-11, L * 0.3]], 0.3,
          shade(P.navyDeck, 1.14));
        m.restore();
        // Island, starboard side, well aft of amidships.
        m.save().move(21, 2.4, L * 0.42);
        m.loft([{ z: -14, pts: trap(6.0, 0, 14.0, 0.9) }, { z: 8, pts: trap(6.4, 0, 15.0, 0.9) },
          { z: 14, pts: trap(5.2, 0, 12.0, 0.9) }], shade(P.navy, 1.05));
        mast(m, { y: 15.0, z: 0, r: 0.9, h: 13.0, radar: true });
        m.both((mm) => {
          mm.save().move(5.0, 9.0, 2.0);
          arrayFace(mm, { r: 2.6, tilt: 8, squash: 0.95 });
          mm.restore();
        });
        m.restore();
        [[-14, 0.62, 24], [-24, 0.5, 12], [-13, 0.36, -6], [-26, 0.24, 4],
          [16, 0.12, -170], [6, 0.2, 178], [26, 0.7, 30], [10, 0.9, 6]].forEach((p) => {
          deckPlane(m, p[0], L * p[1], p[2], P.grey);
        });
        // A task group is not one ship. Two escorts in company, at the station
        // they would actually keep, and the picture stops being a carrier and
        // starts being a group.
        [[-96, -L * 0.2, 1], [88, L * 0.12, -1]].forEach((e) => {
          m.save().move(e[0], 0, e[1]).rotY(e[2] * 4);
          ship(m, { len: 150, beam: 16, draft: 5.0, freeboard: 6.6, sheer: 2.0,
            houses: [[0.26, 0.64, 0.72, 8.0], [0.32, 0.54, 0.5, 4.6]] });
          mast(m, { y: 14.6, z: 150 * 0.5, r: 0.7, h: 11.0, radar: true });
          deckGun(m, { y: 6.6, z: 150 * 0.8, r: 2.0 });
          m.restore();
        });
      },
    },
    la_ssn: {
      name: "Los Angeles-class SSN", cls: "Naval", span: 110,
      build(m) {
        submarine(m, { len: 110, dia: 10, sailLen: 16, sailH: 6.2, sailPlanes: true });
      },
    },
    aip_ssk: {
      name: "Air-Independent Propulsion Submarine", cls: "Naval", span: 56,
      build(m) {
        submarine(m, { len: 56, dia: 6.2, sailLen: 8.4, sailH: 3.6, xstern: true,
          col: shade(P.hull, 1.15) });
      },
    },
    laws: {
      name: "Shipboard Directed-Energy Mount", cls: "Naval", span: 3.4,
      build(m) {
        m.save().rotX(-90);
        m.tube(0.86, 0.78, 0.9, 12, P.navyDeck);
        m.restore();
        m.save().move(0, 0.9, 0).rotY(18);
        m.loft([{ z: -0.9, pts: trap(0.72, 0, 1.05, 0.8) }, { z: 0.5, pts: trap(0.78, 0, 1.15, 0.82) },
          { z: 0.95, pts: trap(0.6, 0, 0.95, 0.8) }], P.white);
        // The beam director: a coelostat window on a trunnion, tilted up the way
        // a mount tracking something is tilted up.
        m.save().move(0, 0.8, 0.6).rotX(-16);
        m.tube(0.42, 0.4, 1.1, 12, shade(P.white, 0.92));
        m.save().move(0, 0, 1.1);
        m.tube(0.34, 0.34, 0.06, 12, P.glass);
        m.restore();
        // The beam itself. Nothing else in this file emits, and a directed
        // energy mount drawn cold is indistinguishable from a searchlight.
        m.save().move(0, 0, 1.16);
        m.tube(0.06, 0.02, 2.2, 6, [1, 0.86, 0.55]);
        m.restore();
        m.restore();
        m.restore();
      },
    },

    // --------------------------------------------------------------- missile
    msl_sam: {
      name: "Area Air-Defence System", cls: "Missile", span: 13.1,
      build(m) {
        const bed = truck(m, { len: 13.1, w: 3.2, wheelR: 0.62, axles: 4, cab: 2.8, cabH: 1.9,
          col: P.olive });
        // Four canisters, near vertical. A cold-launch area system stands its
        // rounds up, and that is what tells it apart from the theatre battery
        // two entries down, which slants its box at the horizon.
        m.save().move(0, bed, 3.6).rotX(-84);
        for (let i = 0; i < 4; i += 1) {
          m.save().move(((i % 2) - 0.5) * 1.5, (Math.floor(i / 2) - 0.5) * 1.5, 0);
          m.tube(0.62, 0.62, 7.4, 10, shade(P.olive, 1.1));
          m.save().move(0, 0, 7.4);
          m.tube(0.56, 0.56, 0.12, 10, P.black);
          m.restore();
          m.restore();
        }
        m.restore();
        m.save().move(0, bed + 0.4, 1.4);
        m.bar(-1.5, 1.5, -0.4, 0, -1.6, 1.6, shade(P.olive, 0.86));
        m.restore();
      },
    },
    msl_brm: {
      name: "Theatre Ballistic Missile", cls: "Missile", span: 13.4,
      build(m) {
        const bed = truck(m, { len: 13.4, w: 3.0, wheelR: 0.66, axles: 4, cab: 3.0, cabH: 2.0,
          col: shade(P.olive, 0.92) });
        // Erected. A transporter-erector-launcher with the round down is a
        // lorry; with the round up it is the thing the deck is pricing.
        m.save().move(0, bed, 2.6).rotX(-74);
        missile(m, {
          len: 11.25, r: 0.44, col: shade(P.warhead, 0.94), nose: 2.6,
          fins: [{ n: 4, z: 0.02, span: 0.55, chord: 1.5, tip: 0.7, roll: 45 }],
        });
        m.restore();
        m.save().move(0, bed - 0.1, 1.2);
        m.bar(-1.3, 1.3, -0.5, 0.2, -2.6, 3.4, shade(P.olive, 0.8));
        m.restore();
      },
    },
    msl_deterrent: {
      name: "Strategic Deterrent Force", cls: "Missile", span: 21,
      build(m) {
        // A silo, its hatch thrown clear, and the round leaving it. The pad is
        // as much of the model as the missile: a deterrent is a fixed
        // installation somebody has to find.
        m.save().rotX(-90);
        m.tube(3.4, 3.4, 1.1, 16, shade(P.metal, 0.55));
        m.save().move(0, 0, 1.1);
        m.tube(2.5, 2.5, 0.3, 16, P.black);
        m.restore();
        m.restore();
        m.save().move(5.6, 0.6, 0).rotZ(-24);
        m.bar(-3.2, 3.2, 0, 0.5, -3.2, 3.2, shade(P.metal, 0.62));
        m.restore();
        m.save().move(0, 1.2, 0).rotX(-90);
        missile(m, {
          len: 21, r: 1.05, col: P.white, nose: 4.2, seg: 12,
          stations: [[0, 1, 1], [0.02, 1, 1], [0.36, 1, 1], [0.38, 0.9, 0.9], [0.66, 0.9, 0.9],
            [0.68, 0.78, 0.78], [0.8, 0.78, 0.78], [1, 0.1, 0.1]],
          fins: [{ n: 4, z: 0.0, span: 0.5, chord: 1.6, tip: 0.8, roll: 45 }],
        });
        m.restore();
      },
    },
    paveway: {
      name: "GBU-24 Paveway III", cls: "Missile", span: 4.4,
      build(m) {
        missile(m, {
          len: 4.4, r: 0.185, col: P.warhead, nose: 0.5, seeker: true, motor: false,
          fins: [{ n: 4, z: 0.72, span: 0.28, chord: 0.42, tip: 0.24, roll: 45 },
            { n: 4, z: 0.06, span: 0.62, chord: 1.3, tip: 0.8 }],
        });
        m.bar(-0.06, 0.06, 0.19, 0.3, 1.4, 2.8, shade(P.warhead, 0.85));
      },
    },
    tomahawk: {
      name: "BGM-109 Tomahawk", cls: "Missile", span: 6.25,
      build(m) {
        missile(m, {
          len: 6.25, r: 0.26, col: shade(P.white, 0.9), nose: 0.7, blunt: true,
          wings: { z: 0.42, span: 1.35, chord: 0.62, tip: 0.42, sweep: 0.3 },
          fins: [{ n: 3, z: 0.03, span: 0.34, chord: 0.55, tip: 0.3, roll: 60 }],
        });
        // The ventral scoop, which is how a cruise missile breathes and the one
        // detail that stops it reading as a rocket.
        m.save().move(0, -0.26, 4.3);
        m.bar(-0.14, 0.14, -0.22, 0.05, -0.5, 0.5, shade(P.white, 0.8));
        m.restore();
        m.save().move(0, 0, -0.55);
        m.tube(0.2, 0.24, 0.55, 10, P.exhaust);
        m.restore();
      },
    },
    patriot: {
      name: "MIM-104 Patriot", cls: "Missile", span: 12.0,
      build(m) {
        // The launching station, not the missile: a semitrailer with the box at
        // the elevation it sits at when the battery is emplaced.
        m.bar(-1.5, 1.5, 0.7, 1.0, 0, 9.2, shade(P.green, 0.8));
        wheels(m, { r: 0.6, w: 0.5, x: 1.55, axles: 2, z0: 1.1, z1: 2.4 });
        m.save().move(0, 1.5, 1.2);
        m.bar(-0.4, 0.4, -0.5, 0.5, -0.5, 0.5, shade(P.green, 0.9));
        m.restore();
        m.save().move(0, 1.0, 1.2).rotX(-38);
        m.bar(-1.55, 1.55, 0, 2.1, 0, 6.2, P.green);
        for (let i = 0; i < 4; i += 1) {
          m.save().move(((i % 2) - 0.5) * 1.5, 0.5 + Math.floor(i / 2) * 1.05, 6.2);
          m.tube(0.42, 0.42, 0.12, 10, P.black);
          m.restore();
        }
        m.restore();
        m.save().move(0, 1.0, 8.6);
        m.bar(-1.2, 1.2, 0, 1.4, -0.9, 0.9, shade(P.green, 1.06));
        m.restore();
      },
    },
    jdam: {
      name: "JDAM Guidance Kit", cls: "Missile", span: 3.84,
      build(m) {
        missile(m, {
          len: 3.84, r: 0.23, col: P.warhead, nose: 0.75, blunt: true, motor: false,
          fins: [{ n: 3, z: 0.02, span: 0.42, chord: 0.6, tip: 0.34, roll: 30 }],
        });
        // The kit IS the tail. Strakes down the body and a boat-tailed control
        // section: a dumb bomb becomes a guided one by bolting this on.
        m.save().move(0, 0, -0.02);
        m.tube(0.235, 0.2, 0.62, 10, shade(P.green, 1.1));
        m.restore();
        m.both((mm) => {
          mm.save().move(0.2, 0, 1.1);
          mm.plate([[0, 1.5], [0.12, 1.4], [0.12, -1.4], [0, -1.5]], 0.06, shade(P.green, 1.05));
          mm.restore();
        });
      },
    },
    gbi: {
      name: "Ground-Based Interceptor", cls: "Missile", span: 16.8,
      build(m) {
        m.save().rotX(-90);
        missile(m, {
          len: 16.8, r: 0.66, col: P.white, nose: 1.4, seg: 12, motor: true,
          stations: [[0, 1, 1], [0.02, 1, 1], [0.42, 1, 1], [0.44, 0.86, 0.86],
            [0.72, 0.86, 0.86], [0.74, 0.62, 0.62], [0.9, 0.6, 0.6], [1, 0.16, 0.16]],
        });
        // The kill vehicle on top, which is the part that costs the money: a
        // squat body with its own divert thrusters and a telescope.
        m.save().move(0, 0, 16.4);
        m.tube(0.38, 0.3, 0.9, 8, P.sensor);
        m.save().move(0, 0, 0.9);
        m.tube(0.22, 0.2, 0.5, 8, P.glass);
        m.restore();
        m.restore();
        m.restore();
        m.save().rotX(-90).move(0, 0, -0.1);
        m.tube(0.7, 0.7, 0.5, 12, shade(P.metal, 0.5));
        m.restore();
      },
    },
    x51: {
      name: "Scramjet Test Vehicle", cls: "Missile", span: 7.6,
      build(m) {
        // A waverider on its booster, which is the only way one of these has
        // ever flown: the test article is the front two metres.
        facetBody(m, {
          len: 4.3, w: 0.66, h: 0.6, col: shade(P.white, 0.86),
          stations: [[0, .9, .9], [.12, 1, 1], [.62, 1, 1], [.86, .7, .66], [1, .18, .16]],
        });
        m.save().move(0, -0.32, 1.5);
        m.bar(-0.3, 0.3, -0.24, 0.06, -1.1, 1.1, P.sensor);
        m.bar(-0.24, 0.24, -0.26, -0.2, -1.1, 1.1, P.black);
        m.restore();
        m.save().move(0, 0.26, 0.5).rotZ(90);
        m.plate([[0, 1.0], [0.7, 0.2], [0.7, -0.2], [0, -0.5]], 0.08, shade(P.white, 1.05));
        m.restore();
        m.save().move(0, -0.05, -3.3);
        m.tube(0.42, 0.42, 3.3, 10, P.greyDark);
        m.save().move(0, 0, -0.2);
        m.tube(0.34, 0.42, 0.3, 10, P.exhaust);
        m.restore();
        for (let i = 0; i < 4; i += 1) {
          m.save().move(0, 0, 0.3).rotZ(45 + i * 90);
          m.plate([[0.4, 0.9], [1.0, 0.45], [1.0, -0.1], [0.4, -0.2]], 0.07, shade(P.greyDark, 1.1));
          m.restore();
        }
        m.restore();
      },
    },
    hgv: {
      name: "Hypersonic Glide Vehicle", cls: "Missile", span: 5.0,
      build(m) {
        // A flat-bottomed cone that rides its own shock. No motor, four flaps,
        // and a nose radius small enough to be the hard part of the programme.
        m.loft([
          { z: 0, pts: pent(1.15, -0.34, 0.24, 0.28) },
          { z: 1.2, pts: pent(1.05, -0.3, 0.22, 0.26) },
          { z: 3.2, pts: pent(0.6, -0.18, 0.14, 0.16) },
          { z: 4.6, pts: pent(0.2, -0.06, 0.05, 0.06) },
          { z: 5.0, pts: pent(0.04, -0.012, 0.01, 0.012) },
        ], shade(P.stealth, 1.3));
        m.both((mm) => {
          mm.save().move(0.5, -0.28, 0.05).rotX(14);
          mm.plate([[0, 0.02], [0.62, 0.02], [0.62, -0.72], [0, -0.72]], 0.07, P.sensor);
          mm.restore();
          mm.save().move(1.05, 0.0, 0.6).rotZ(64);
          mm.plate([[0, 0.5], [0.5, 0.2], [0.5, -0.3], [0, -0.6]], 0.07, shade(P.stealth, 1.5));
          mm.restore();
        });
        m.save().move(0, -0.05, 0);
        m.tube(0.5, 0.5, 0.06, 10, P.black);
        m.restore();
      },
    },
    owa: {
      name: "One-Way Attack Drone", cls: "Missile", span: 2.5,
      build(m) {
        const L = 3.5;
        // A delta wing with the warhead in the nose and a two-stroke pushing it.
        // Cheap enough to be expendable is the whole design, and it shows.
        m.both((mm) => {
          mm.plate([[0, L * 0.92], [1.25, L * 0.12], [1.25, -L * 0.02], [0, L * 0.02]], 0.14,
            shade(P.cloth, 0.9));
          mm.save().move(1.2, 0.0, L * 0.06).rotZ(76);
          mm.plate([[0, 0.42], [0.42, 0.2], [0.42, -0.16], [0, -0.28]], 0.06, shade(P.cloth, 1.1));
          mm.restore();
        });
        bodyLoft(m, {
          len: L, w: 0.5, h: 0.42, col: shade(P.cloth, 1.05), seg: 8,
          stations: [[0, .5, .5], [.08, .9, .9], [.5, 1, 1], [.86, .8, .86], [1, .24, .3]],
        });
        m.save().move(0, 0.05, -0.02);
        for (let i = 0; i < 2; i += 1) {
          m.save().rotZ(i * 90 + 24);
          m.plate([[0.04, 0.05], [0.5, 0.07], [0.5, -0.07], [0.04, -0.05]], 0.03, P.black);
          m.restore();
        }
        m.restore();
      },
    },

    // -------------------------------------------------------------- infantry
    raven: {
      name: "RQ-11 Raven", cls: "Infantry", span: 1.4,
      build(m) {
        const L = 0.92;
        bodyLoft(m, {
          len: L, w: 0.1, h: 0.12, col: shade(P.cloth, 1.3), seg: 8,
          stations: [[0, .4, .4], [.1, .8, .8], [.55, 1, 1], [.86, .9, .9], [1, .5, .55]],
        });
        m.both((mm) => {
          mm.wing({ x0: 0.04, y: 0.04, z: L * 0.62, span: 0.66, root: 0.16, tip: 0.14,
            sweep: 0.02, thick: 0.02 }, shade(P.cloth, 1.24));
          mm.save().move(0.02, 0.02, L * 0.06).rotZ(-(90 - 35));
          mm.plate([[0, 0.14], [0.2, 0.06], [0.2, -0.02], [0, -0.06]], 0.014, shade(P.cloth, 1.24));
          mm.restore();
        });
        m.save().move(0, 0.02, L * 0.98);
        for (let i = 0; i < 2; i += 1) {
          m.save().rotZ(i * 90 + 30);
          m.plate([[0.01, 0.012], [0.19, 0.016], [0.19, -0.016], [0.01, -0.012]], 0.008, P.black);
          m.restore();
        }
        m.restore();
        // Hand-launched, so the model is drawn with the hand's grip on it: the
        // camera nose and the fuselage plug are all there is.
        m.save().move(0, -0.02, L * 0.72).scale(1, 0.9, 1);
        m.ball(0.055, 8, 4, P.sensor);
        m.restore();
      },
    },
    switchblade: {
      name: "Switchblade Loitering Munition", cls: "Infantry", span: 1.2,
      build(m) {
        // Deployed, beside its tube. Folded it is a tube with a tube in it, and
        // the picture that says what the money bought is the wings out.
        const L = 0.5;
        missile(m, { len: L, r: 0.037, col: P.sensor, nose: 0.12, blunt: true, motor: false, seg: 8 });
        m.both((mm) => {
          mm.save().move(0.03, 0.01, L * 0.6);
          mm.plate([[0, 0.08], [0.28, 0.04], [0.28, -0.02], [0, -0.06]], 0.008, shade(P.sensor, 1.5));
          mm.restore();
          mm.save().move(0.03, -0.01, L * 0.2).rotZ(-64);
          mm.plate([[0, 0.06], [0.16, 0.03], [0.16, -0.02], [0, -0.05]], 0.008, shade(P.sensor, 1.4));
          mm.restore();
        });
        m.save().move(0, 0.0, -0.02);
        for (let i = 0; i < 2; i += 1) {
          m.save().rotZ(i * 90 + 20);
          m.plate([[0.01, 0.008], [0.1, 0.012], [0.1, -0.012], [0.01, -0.008]], 0.005, P.black);
          m.restore();
        }
        m.restore();
        m.save().move(0.28, 0.06, 0.1).rotY(9).rotX(4);
        m.tube(0.07, 0.07, 0.62, 10, shade(P.cloth, 0.85));
        m.restore();
      },
    },
    cuas: {
      name: "Counter-UAS Battery", cls: "Infantry", span: 6.4,
      build(m) {
        m.bar(-1.1, 1.1, 0.55, 0.8, 0, 4.2, shade(P.green, 0.8));
        wheels(m, { r: 0.5, w: 0.34, x: 1.15, axles: 2, z0: 1.0, z1: 2.6 });
        m.bar(-0.14, 0.14, 0.3, 0.62, 3.9, 4.9, shade(P.green, 0.7));
        // A layered battery is three things on one trailer: it has to see, it
        // has to jam, and it has to shoot. All three are on the mount.
        m.save().move(0, 0.8, 2.6);
        m.tube(0.4, 0.36, 0.5, 10, P.green);
        m.save().move(0, 0.5, 0);
        m.bar(-0.62, 0.62, 0, 0.7, -0.6, 0.6, shade(P.green, 1.06));
        m.save().move(0, 0.42, 0.6).rotX(-24);
        m.tube(0.1, 0.09, 1.5, 8, P.black);
        m.restore();
        m.save().move(0.42, 0.72, 0).rotX(-20);
        m.bar(-0.2, 0.2, 0, 0.86, -0.05, 0.05, P.metal);
        m.restore();
        m.save().move(-0.34, 0.7, 0.1);
        arrayFace(m, { r: 0.44, tilt: 16 });
        m.restore();
        m.restore();
        m.restore();
        m.save().move(0, 0.8, 0.7).rotX(-90);
        dish(m, { r: 0.62, col: shade(P.white, 0.92) });
        m.restore();
      },
    },
    link16: {
      name: "Tactical Data Link Fit", cls: "Infantry", span: 2.2,
      build(m) {
        // An antenna group on its mounting plate. The fit is not a vehicle; it
        // is what is added to one, so the model is the hardware and its plate.
        m.bar(-0.9, 0.9, 0, 0.1, -0.7, 0.7, P.greyDark);
        m.save().move(0, 0.1, 0.2).rotZ(90);
        m.plate([[0, 0.16], [0.86, 0.1], [0.86, -0.06], [0, -0.2]], 0.07, shade(P.white, 0.95));
        m.restore();
        m.save().move(-0.5, 0.1, -0.2).scale(1, 0.8, 1);
        m.ball(0.3, 10, 5, shade(P.white, 1.0));
        m.restore();
        m.both((mm) => {
          mm.save().move(0.66, 0.1, -0.36).rotX(-6);
          mm.tube(0.022, 0.012, 1.15, 6, P.black);
          mm.restore();
        });
        m.save().move(0.2, 0.1, -0.5);
        m.bar(-0.24, 0.24, 0, 0.3, -0.16, 0.16, P.sensor);
        m.restore();
      },
    },
    c4isr: {
      name: "Networked C4ISR", cls: "Infantry", span: 7.4,
      build(m) {
        const bed = truck(m, { len: 7.4, w: 2.5, wheelR: 0.55, axles: 2, cab: 2.0, cabH: 1.7,
          col: P.green });
        // A box body with a dish on the roof and two masts up: a command post
        // is a room that drove there.
        m.loft([{ z: 0.3, pts: trap(1.25, bed, bed + 2.1, 0.98) },
          { z: 5.0, pts: trap(1.25, bed, bed + 2.1, 0.98) }], shade(P.green, 1.05));
        m.save().move(0, bed + 2.1, 2.6).rotX(-58);
        dish(m, { r: 1.0, col: shade(P.white, 0.94) });
        m.restore();
        m.save().move(1.0, bed + 2.1, 0.6);
        m.tube(0.05, 0.03, 2.6, 6, P.metal);
        m.restore();
        m.save().move(-1.0, bed + 2.1, 0.7);
        m.tube(0.04, 0.02, 1.9, 6, P.metal);
        m.restore();
        m.save().move(0, bed + 0.4, 0.28);
        m.bar(-0.8, 0.8, 0, 1.2, -0.06, 0.06, shade(P.green, 0.82));
        m.restore();
      },
    },
    atr: {
      name: "Autonomous Target Recognition", cls: "Infantry", span: 0.8,
      build(m) {
        // The turret ball, because that is where the recognition happens: two
        // apertures, a yoke, and the processing box it feeds.
        m.bar(-0.3, 0.3, 0.52, 0.62, -0.3, 0.3, P.greyDark);
        m.both((mm) => {
          mm.save().move(0.26, 0.24, 0);
          mm.bar(-0.05, 0.05, 0, 0.3, -0.12, 0.12, shade(P.greyDark, 1.1));
          mm.restore();
        });
        m.save().move(0, 0.24, 0);
        m.ball(0.26, 12, 7, P.sensor);
        m.save().move(0, 0, 0.24).rotY(0);
        m.tube(0.11, 0.11, 0.04, 10, P.glass);
        m.restore();
        m.save().move(0.11, 0.06, 0.22);
        m.tube(0.055, 0.055, 0.05, 8, shade(P.glass, 1.3));
        m.restore();
        m.restore();
        m.save().move(0, 0.62, 0);
        m.bar(-0.24, 0.24, 0, 0.2, -0.24, 0.24, shade(P.sensor, 1.3));
        m.restore();
      },
    },

    // ------------------------------------------------------------------ space
    spc_recon: {
      name: "Reconnaissance Satellite Constellation", cls: "Space", span: 12,
      build(m) {
        // A constellation is not one satellite, and the deck charges for the
        // constellation. Three, at the sizes perspective would give them.
        satellite(m, { bus: [1.6, 1.9, 3.4], span: 11.0, panelW: 2.1, bays: 3, col: P.gold });
        m.save().move(0, 0, 1.7).rotX(180);
        dish(m, { r: 0.7, col: shade(P.white, 0.96) });
        m.restore();
        m.save().move(0, -1.0, -1.9);
        m.tube(0.34, 0.34, 0.7, 10, P.sensor);
        m.restore();
        [[-5.2, 2.6, -6.0, 0.42], [5.6, -2.2, -7.4, 0.34]].forEach((s) => {
          m.save().move(s[0], s[1], s[2]).rotY(28).rotZ(14).scale(s[3]);
          satellite(m, { bus: [1.6, 1.9, 3.4], span: 11.0, panelW: 2.1, bays: 3, col: P.gold });
          m.restore();
        });
      },
    },
    kh11: {
      name: "Electro-Optical Reconnaissance Satellite", cls: "Space", span: 15,
      build(m) {
        // A telescope with a spacecraft bolted to the back of it, which is what
        // one of these is: the tube sets the size and everything else follows.
        m.save().rotX(-90);
        m.tube(1.35, 1.35, 8.2, 14, P.gold);
        m.save().move(0, 0, 8.2);
        m.tube(1.45, 1.45, 2.6, 14, shade(P.black, 1.4));
        m.save().move(0, 0, 2.6);
        m.tube(1.4, 1.4, 0.1, 14, P.black);
        m.restore();
        m.restore();
        m.restore();
        m.save().move(0, -4.6, 0);
        satellite(m, { bus: [2.6, 2.4, 3.0], span: 15.0, panelW: 2.6, bays: 4, col: shade(P.gold, 0.9) });
        m.restore();
        m.save().move(0, -5.4, 1.9).rotX(28);
        dish(m, { r: 0.95, col: shade(P.white, 0.96) });
        m.restore();
        m.save().move(1.4, -6.2, 0).rotZ(-40);
        m.tube(0.05, 0.04, 2.2, 6, P.metal);
        m.restore();
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
  /// Build once, keep. A card that scrolls in and out of view rebuilds nothing;
  /// forty-six meshes together are under a megabyte of Float32Array and they
  /// are all the same forty-six every time.
  function build(id, cls) {
    const key = resolve(id, cls);
    if (!key) return null;
    if (cache.has(key)) return cache.get(key);
    const m = new Mesh();
    MODELS[key].build(m);
    const geom = m.finish();
    geom.id = key;
    geom.name = MODELS[key].name;
    geom.cls = MODELS[key].cls;
    geom.span = MODELS[key].span;
    cache.set(key, geom);
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
    for (let i = 0; i < n.length; i += 9) {
      out.push(`vn ${n[i].toFixed(4)} ${n[i + 1].toFixed(4)} ${n[i + 2].toFixed(4)}`);
    }
    for (let t = 0; t < g.count / 3; t += 1) {
      const a = t * 3 + 1;
      out.push(`f ${a}//${t + 1} ${a + 1}//${t + 1} ${a + 2}//${t + 1}`);
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
