// The logistics and civilian prop kit. Roadmap section F (civilian transport
// and logistics) and the "initial logistics props" of P2, built as one kit
// rather than as twenty-five unrelated meshes.
//
// WHY THIS EXISTS. A freight yard, a quay, a goods siding and a depot are the
// places where the simulation's trade, production and construction numbers
// would become something you can look at, and the browser has none of the
// objects those places are made of. Section F names them: cars, buses, vans,
// articulated trucks, tankers, locomotives, wagons, cranes. This file is that
// list, at the two levels of detail the game actually draws.
//
// REUSE IS THE POINT, and it is the reason this is one file. A tanker trailer
// is a semi-trailer chassis plus a barrel; a tank wagon is an underframe plus
// the SAME barrel; a fuel bowser is a rigid chassis plus a cab plus a shorter
// one. A container flat wagon, a yard gantry and a container stack all hold
// the same ISO box. Seventeen shared part builders assemble twenty-five
// pieces, and `reuse()` MEASURES that rather than asserting it: it builds
// every piece with a trace switched on and reports which builder each one
// actually called. A claim about reuse that nobody can re-derive is marketing.
//
// WHAT IT IS NOT. Representative scenery. Every design here is generic and
// original: no licensed vehicle, no named manufacturer, no badge, no model
// designation, and no dimension below is a measured dimension of any real
// machine. The sizes are chosen to be plausible for the 1990 baseline and to
// sit correctly beside each other — a 40 ft container next to a semi-trailer
// next to a bogie flat wagon — not to reproduce a specification. NOTHING HERE
// GRANTS A CAPABILITY: a modelled tanker is not a fuel system, a silo is not
// storage, a crane does not move cargo, and a wagon carries nothing the
// simulation has recorded. This module is NOT WIRED INTO THE GAME.
//
// THE 1990 BASELINE shows up as omissions as much as shapes. Flat dark glazing
// rather than mirrored glass, steel wheels rather than alloys, a boxy cab with
// a small roof fairing rather than a moulded one, no aerodynamic trailer
// skirts, no LED lamp clusters, no reach-stacker silhouette. Where a shape
// would read as 2010s it was cut.
//
// MODEL SPACE. Metres. +X right, +Y up, +Z forward — a vehicle faces +Z, so
// its length runs in Z and its track in X. Y=0 is ground contact and
// `bounds.min[1]` is exactly 0 on every piece, because that is the only
// contract a renderer placing a prop on terrain has. A prop root is centred
// horizontally on its own footprint.
//
// DETERMINISM. There is no clock and no entropy source in this file. Every
// choice a piece makes — which livery, which container colours, how the crates
// are stacked — is an integer hash of explicit inputs: the piece key and the
// caller's `variant`. Same inputs, byte-identical buffers — in this process, in
// a freshly required copy, and in a SEPARATE NODE PROCESS, which is the arm of
// the claim a module-level cache would satisfy for free and which
// `check_prop_mesh.cjs` therefore spawns a child to test. That is the project's
// first iron rule and art gets no exemption from it.
//
// WHAT IS A PIECE AND WHAT IS A PART. A PIECE is something `build` returns and
// something the near budget applies to. A single shipping pallet is not one:
// at 1.2 x 0.8 m it cannot honestly reach the 300-triangle floor, and padding
// it to get there would be the budget writing the art. So the pallet is a
// shared PART, and `pallet_stack` and `crate_stack` are the pieces that use it.
// Same reasoning the other way for `silo_group` and `container_stack`, which
// are assemblies because a single silo or a single stacked box is not what a
// yard shows you.
//
// THE API, in one place. `build(piece, {lod, variant, coupled})` is one prop.
// `rig(tractor, trailer, opts)` couples two of them at the fifth wheel.
// `yard(opts)` places forty into a freight-yard scene, which is what the map
// budget is actually asked about. `reuse()` measures the shared-part matrix.
// `pieces()`, `meta()` and `yardRoster()` are the catalogue.
//
// COLOUR IS RESOLVED FROM THE FINISHED NORMAL, which is what makes the
// smoothing worth having. There are no textures and no UVs in this renderer,
// so a cylinder's only chance of reading as a cylinder is a colour gradient
// across it. Vertices carry a base colour and a multiplier; `finish` averages
// face normals inside a smoothing group, crease-limited so a rim stays a rim,
// and only then works out the colour from one fixed sun. A tank barrel and a
// wheel therefore cost exactly the triangles they always did and stop looking
// like folded card.
//
// LODS. `lod: 0` is the near/inspection mesh, 300..2,500 triangles; `lod: 1` is
// the map mesh, 30..200. They are ONE call graph with `detail` gating the small
// stuff and the segment counts, not two authorings, so the map mesh cannot
// drift away from the prop it is a cheap view of.
(function (root, factory) {
  const api = factory();
  if (typeof module === "object" && module.exports) module.exports = api;
  else root.PropMesh = api;
})(typeof globalThis !== "undefined" ? globalThis : this, function () {
  "use strict";

  const DEG = Math.PI / 180;

  /// The fifth-wheel height, and the whole of the coupling contract. A tractor
  /// carries its coupler plate at exactly this height and a semi-trailer
  /// carries its kingpin at exactly this height, so `rig()` couples the two
  /// with a translation in Z alone and the coupled vehicle still touches the
  /// ground at Y=0. Pick a different number for either half and the trailer
  /// either floats or drags.
  const COUPLING_Y = 1.15;

  /// How far two faces may disagree and still share a corner normal. cos 69.5°.
  /// A limit is needed because a smooth barrel still has a hard rim where it
  /// meets its dished end, a wheel has a hard rim where tread meets sidewall,
  /// and a smoothing pass that ignores that turns every cylinder into a blob.
  /// At 0.35 the ring of an 8-segment tube merges (adjacent faces 45 degrees
  /// apart, dot 0.707) and its end cap does not (dot 0).
  ///
  /// It also PROVES the shading contract the checks assert. A corner averages
  /// only the faces within the limit OF THAT FACE, so every summand has dot
  /// >= 0.35 with the face being shaded and the normalised sum therefore does
  /// too. A smoothed vertex can never point behind the triangle it belongs to.
  const CREASE = 0.35;

  // One sun, fixed, high and to the right. Lighting is baked into the vertex
  // colour because the path this feeds is the flat vertex-colour path the rest
  // of the game already uses. Separating albedo from light properly is a
  // renderer milestone and not this file's; the roadmap says so and this is the
  // approximation it asks for in the meantime.
  const SUN = (function () {
    const v = [0.42, 0.80, 0.40], n = Math.hypot(v[0], v[1], v[2]);
    return [v[0] / n, v[1] / n, v[2] / n];
  })();
  const AMBIENT = 0.54, DIFFUSE = 0.52;

  // ------------------------------------------------------------------- hash
  // The only source of variety here. A piece asks a question by salt — "which
  // livery", "which container colour", "how tall is crate three" — and gets the
  // same answer on every machine and every reload. Murmur3's finaliser with a
  // stir in front of it: not cryptographic, does not need to be, and it
  // decorrelates small integers, which is the whole job.
  function mix32(x) {
    let h = x | 0;
    h = Math.imul(h ^ (h >>> 16), 0x7feb352d);
    h = Math.imul(h ^ (h >>> 15), 0x846ca68b);
    h ^= h >>> 16;
    return h >>> 0;
  }
  function seedOf(id) {
    if (typeof id === "number" && Number.isFinite(id)) return mix32(Math.round(id));
    let h = 0x811c9dc5;
    const s = String(id == null ? "" : id);
    for (let i = 0; i < s.length; i += 1) h = Math.imul(h ^ s.charCodeAt(i), 0x01000193);
    return mix32(h);
  }
  function ask(seed, salt) { return mix32((seed ^ Math.imul((salt | 0) + 1, 0x9e3779b1)) | 0); }
  function askUnit(seed, salt) { return ask(seed, salt) / 4294967296; }
  function askOne(seed, salt, list) { return list[ask(seed, salt) % list.length]; }
  function askSpan(seed, salt, lo, hi) { return lo + (hi - lo) * askUnit(seed, salt); }

  // ----------------------------------------------------------------- colour
  function rgb(hex) {
    return [((hex >> 16) & 255) / 255, ((hex >> 8) & 255) / 255, (hex & 255) / 255];
  }

  /// The yard palette. A working yard is grey, drab and scuffed, and the only
  /// saturated things on one are containers, safety paint and liveries — so
  /// those are the only saturated entries here. 1990 again: no pearlescent
  /// paint, no anodised trim, glazing is a dark hole and not a mirror.
  const P = {
    tyre: rgb(0x24262a),
    hub: rgb(0x8f9499),
    chassis: rgb(0x3b4046),
    steel: rgb(0x6f7a84),
    galv: rgb(0x9aa2a8),
    chrome: rgb(0xb2b8bd),
    glass: rgb(0x2b3a44),
    lamp: rgb(0xc9a83e),
    lampRed: rgb(0x8e2f26),
    panel: rgb(0xc9cbc8),
    dark: rgb(0x1b1d20),
    timber: rgb(0x9b7d4e),
    crate: rgb(0xa4855a),
    board: rgb(0x8a7a5c),
    wrap: rgb(0x8f96a0),
    concrete: rgb(0xb0aca2),
    asphalt: rgb(0x45474a),
    ballast: rgb(0x7a766c),
    railhead: rgb(0x8b8f92),
    rust: rgb(0x6b5546),
    safety: rgb(0xc09427),
    tankShell: rgb(0xc9cbc6),
    siloShell: rgb(0xa8adb1),
    craneBody: rgb(0xb4b9bb),
    belt: rgb(0x2f3134),
    // Cab and body liveries. Six plain fleet colours, the sort a 1990 haulier
    // actually painted a cab, and none of them is a real operator's scheme.
    livery: [
      rgb(0xd0d2cf), rgb(0x2f5f8c), rgb(0x8f3128),
      rgb(0x2f6046), rgb(0xb26a22), rgb(0xc4b795),
    ],
    // Container colours. Weathered, because a 1990 box has been at sea.
    box: [
      rgb(0x9c4a34), rgb(0x2f5d7a), rgb(0x3d6b47),
      rgb(0x8a8d90), rgb(0xa8712a), rgb(0x7b5f52),
    ],
    // Car paints. Dull on purpose; a yard is not a showroom.
    car: [rgb(0x8d3b30), rgb(0x5a6672), rgb(0xbcb6a6), rgb(0x33503f), rgb(0x8d8f92)],
  };

  // -------------------------------------------------------------- matrices
  // Row-major 4x4, v' = M v — the same convention and the same reason as
  // site-mesh.js and arsenal-models.js: the only reader is `xf` and it is
  // easier to check by eye than a column-major one.
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

  // ---------------------------------------------------------------- builder
  // Triangle soup, a transform stack, a smoothing-group id and a list of named
  // vertex ranges. Normals are NOT stored at emit time: `finish` derives a face
  // normal from the vertices AS TRANSFORMED and then smooths, so a mirrored
  // half gets the normal its geometry actually has. Mirroring is how most of
  // the near-side detail on these vehicles is placed, so that is worth the two
  // lines it costs.
  function Mesh(detail) {
    this.pos = [];
    this.base = [];
    this.mat = [];
    this.grp = [];
    this.parts = [];
    this.m = IDENT.slice();
    this.stack = [];
    this.sg = 0;
    this.sgNext = 0;
    // 1 near, 0 map. Read by the shared kit — a chamfer, a mirror arm, a
    // handrail, a corner casting and a wheel's segment count all ask it whether
    // they are worth drawing — so the two LODs are one call graph and a piece
    // never has to know which one it is being built at.
    this.detail = detail == null ? 1 : detail | 0;
  }
  /// Near value or map value. Reads at the call site like the decision it is.
  Mesh.prototype.d = function (near, map) { return this.detail >= 1 ? near : map; };
  /// How many sides a round thing gets. Curvature is the one place where
  /// segments buy real shape, so the near mesh is generous; on the map a
  /// four-sided drum is not merely acceptable, it is invisible.
  Mesh.prototype.segs = function (near, map) { return this.detail >= 1 ? near : (map == null ? 4 : map); };

  /// Everything emitted inside `fn` shares one smoothing group, and each call
  /// takes a FRESH id: two drums standing shoulder to shoulder touch at their
  /// skins and a shared group would weld their normals into one lumpy surface.
  /// Group 0 means "never smoothed" and is where flat plate, panelling, glass
  /// and every box in the file stays.
  Mesh.prototype.smooth = function (fn) {
    const prev = this.sg;
    this.sgNext += 1;
    this.sg = this.sgNext;
    fn(this);
    this.sg = prev;
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
  Mesh.prototype.detSign = function () {
    const m = this.m;
    const d = m[0] * (m[5] * m[10] - m[6] * m[9])
      - m[1] * (m[4] * m[10] - m[6] * m[8])
      + m[2] * (m[4] * m[9] - m[5] * m[8]);
    return d < 0 ? -1 : 1;
  };

  // A DEGENERATE TRIANGLE IS DROPPED, not emitted with a fallback normal. A
  // cone tip, a zero-width mullion or a wheel scaled to nothing is a modelling
  // slip rather than a shape, and letting one through costs either a NaN in an
  // attribute buffer or a black facet nobody can explain three months later.
  // Dropping happens at emit, so the part ranges counted around a draw call
  // stay exact.
  Mesh.prototype.tri = function (a, b, c, col, mat) {
    const flip = this.detSign() < 0;
    const A = this.xf(a), B = this.xf(flip ? c : b), C = this.xf(flip ? b : c);
    const ux = B[0] - A[0], uy = B[1] - A[1], uz = B[2] - A[2];
    const vx = C[0] - A[0], vy = C[1] - A[1], vz = C[2] - A[2];
    const nx = uy * vz - uz * vy, ny = uz * vx - ux * vz, nz = ux * vy - uy * vx;
    if (!(Math.hypot(nx, ny, nz) > 1e-9)) return this;
    const k = mat == null ? 1 : mat;
    const tri = [A, B, C];
    for (let i = 0; i < 3; i += 1) {
      this.pos.push(tri[i][0], tri[i][1], tri[i][2]);
      this.base.push(col[0], col[1], col[2]);
      this.mat.push(k);
    }
    this.grp.push(this.sg);
    return this;
  };
  Mesh.prototype.quad = function (a, b, c, d, col, mat) {
    return this.tri(a, b, c, col, mat).tri(a, c, d, col, mat);
  };
  Mesh.prototype.fan = function (pts, col, mat) {
    for (let i = 1; i + 1 < pts.length; i += 1) this.tri(pts[0], pts[i], pts[i + 1], col, mat);
    return this;
  };
  /// Draw `fn` once, then again mirrored in X. Half of every vehicle here is
  /// authored on the +X side only, which is both cheaper to write and the only
  /// way the two sides cannot drift apart.
  Mesh.prototype.both = function (fn) {
    fn(this, 1);
    this.save().scale(-1, 1, 1);
    fn(this, -1);
    this.restore();
    return this;
  };

  /// Named ranges, in the shape equipment-mesh.js already set, because the
  /// picker and the card renderer speak it. A part that draws nothing is not
  /// recorded: an empty range would break the contiguity the checks assert, and
  /// a map-LOD prop that has no handrails should not carry a handrail entry.
  Mesh.prototype.part = function (name, fn) {
    const first = this.pos.length / 3;
    fn(this);
    const count = this.pos.length / 3 - first;
    if (count > 0) {
      const cut = name.indexOf(" / ");
      this.parts.push({
        name,
        first,
        count,
        group: cut < 0 ? name : name.slice(0, cut),
        label: cut < 0 ? name : name.slice(cut + 3),
      });
    }
    return this;
  };

  // ------------------------------------------------------------- primitives
  /// A box by EXTENTS, not by centre and size. Almost everything on a vehicle
  /// is positioned by the face it sits flush against — a chassis rail top, a
  /// body side, a deck level — and extents say that without arithmetic at the
  /// call site.
  Mesh.prototype.bar = function (x0, x1, y0, y1, z0, z1, col, mat) {
    // Extents are NORMALISED here rather than trusted. Half the call sites in
    // this file compute an extent from a direction (`zz + d * 0.14`, a mirrored
    // offset, a stack index) and a reversed pair silently inverts the winding,
    // which is the defect that shows up as a hole in a solid object only when
    // the camera moves. One swap costs nothing and removes the whole class.
    if (x0 > x1) { const t = x0; x0 = x1; x1 = t; }
    if (y0 > y1) { const t = y0; y0 = y1; y1 = t; }
    if (z0 > z1) { const t = z0; z0 = z1; z1 = t; }
    const k = mat == null ? 1 : mat;
    this.quad([x1, y0, z0], [x1, y1, z0], [x1, y1, z1], [x1, y0, z1], col, k);
    this.quad([x0, y0, z0], [x0, y0, z1], [x0, y1, z1], [x0, y1, z0], col, k);
    this.quad([x0, y1, z0], [x0, y1, z1], [x1, y1, z1], [x1, y1, z0], col, k);
    this.quad([x0, y0, z0], [x1, y0, z0], [x1, y0, z1], [x0, y0, z1], col, k);
    this.quad([x0, y0, z1], [x1, y0, z1], [x1, y1, z1], [x0, y1, z1], col, k);
    this.quad([x0, y0, z0], [x0, y1, z0], [x1, y1, z0], [x1, y0, z0], col, k);
    return this;
  };
  Mesh.prototype.box = function (cx, cy, cz, sx, sy, sz, col, mat) {
    return this.bar(cx - sx / 2, cx + sx / 2, cy - sy / 2, cy + sy / 2, cz - sz / 2, cz + sz / 2, col, mat);
  };

  /// A CHAMFERED box: the same extents as `bar`, with the four edges parallel
  /// to its longest axis cut back by `ch`.
  ///
  /// WHY IT IS WORTH 16 EXTRA TRIANGLES. Nothing manufactured has a knife edge.
  /// A rolled chassis rail, a crane chord, a container corner post and a fork
  /// tine all carry a small arris, and that arris is what catches the light and
  /// separates a machined object from a folded piece of card — in a renderer
  /// with no textures it is most of what "machined" even means. 20 mm is the
  /// house default. ON THE MAP MESH THIS FALLS BACK TO `bar`: at that range the
  /// chamfer is well under a pixel and 16 triangles is a sixth of the whole
  /// budget, so the same call site pays for it only where it shows.
  Mesh.prototype.beam = function (x0, x1, y0, y1, z0, z1, col, ch0, mat) {
    if (x0 > x1) { const t = x0; x0 = x1; x1 = t; }
    if (y0 > y1) { const t = y0; y0 = y1; y1 = t; }
    if (z0 > z1) { const t = z0; z0 = z1; z1 = t; }
    if (this.detail < 1) return this.bar(x0, x1, y0, y1, z0, z1, col, mat);
    const dx = x1 - x0, dy = y1 - y0, dz = z1 - z0;
    const axis = dx >= dy && dx >= dz ? 0 : (dy >= dz ? 1 : 2);
    const u = axis === 0 ? dy : dx, v = axis === 2 ? dy : dz;
    const ch = Math.max(0.004, Math.min(ch0 == null ? 0.02 : ch0, u * 0.34, v * 0.34));
    const a0 = axis === 0 ? y0 : x0, a1 = axis === 0 ? y1 : x1;
    const b0 = axis === 2 ? y0 : z0, b1 = axis === 2 ? y1 : z1;
    const sec = [
      [a0 + ch, b0], [a1 - ch, b0], [a1, b0 + ch], [a1, b1 - ch],
      [a1 - ch, b1], [a0 + ch, b1], [a0, b1 - ch], [a0, b0 + ch],
    ];
    const e0 = axis === 0 ? x0 : (axis === 1 ? y0 : z0);
    const e1 = axis === 0 ? x1 : (axis === 1 ? y1 : z1);
    const at = (a, b, e) => (axis === 0 ? [e, a, b] : axis === 1 ? [a, e, b] : [a, b, e]);
    for (let i = 0; i < 8; i += 1) {
      const j = (i + 1) % 8;
      this.quad(at(sec[i][0], sec[i][1], e0), at(sec[j][0], sec[j][1], e0),
        at(sec[j][0], sec[j][1], e1), at(sec[i][0], sec[i][1], e1), col, mat);
    }
    this.fan(sec.map((p) => at(p[0], p[1], e1)), col, mat);
    this.fan(sec.map((p) => at(p[0], p[1], e0)).reverse(), col, mat);
    return this;
  };

  function ringPts(rx, ry, seg) {
    const pts = [];
    for (let i = 0; i < seg; i += 1) {
      const t = (i / seg) * Math.PI * 2;
      pts.push([Math.cos(t) * rx, Math.sin(t) * ry]);
    }
    return pts;
  }
  /// A lofted tube along +Z between two rings. Walls open a smoothing group;
  /// caps deliberately do not, so the rim between them stays a hard rim without
  /// having to rely on the crease limit to notice.
  Mesh.prototype.tube = function (r0, r1, len, seg, col, caps, mat) {
    const a = ringPts(r0 || 1e-4, r0 || 1e-4, seg), b = ringPts(r1 || 1e-4, r1 || 1e-4, seg);
    this.smooth((m) => {
      for (let i = 0; i < seg; i += 1) {
        const j = (i + 1) % seg;
        m.quad([a[i][0], a[i][1], 0], [a[j][0], a[j][1], 0],
          [b[j][0], b[j][1], len], [b[i][0], b[i][1], len], col, mat);
      }
    });
    if (caps !== false) {
      this.fan(b.map((p) => [p[0], p[1], len]), col, mat);
      this.fan(a.map((p) => [p[0], p[1], 0]).reverse(), col, mat);
    }
    return this;
  };
  /// A vertical cylinder — masts, columns, silos, bollards, stacks.
  Mesh.prototype.column = function (x, y, z, h, r0, r1, seg, col, caps, mat) {
    this.save().move(x, y, z).rotX(-90);
    this.tube(r0, r1 == null ? r0 : r1, h, seg, col, caps, mat);
    this.restore();
    return this;
  };
  /// A cylinder lying along Z, centred on (x, y) — a barrel, a roller, a
  /// silencer, a hydraulic ram.
  Mesh.prototype.drum = function (x, y, z, len, r0, r1, seg, col, caps, mat) {
    this.save().move(x, y, z);
    this.tube(r0, r1 == null ? r0 : r1, len, seg, col, caps, mat);
    this.restore();
    return this;
  };
  /// A cylinder lying along X — a wheel, an axle, a cross-shaft, a roller. The
  /// same loft as `drum` turned a quarter turn, given its own name because
  /// "which way does this cylinder point" is the question that goes wrong when
  /// a call site has to remember a rotation.
  Mesh.prototype.roll = function (x, y, z, len, r0, r1, seg, col, caps, mat) {
    this.save().move(x, y, z).rotY(90);
    this.tube(r0, r1 == null ? r0 : r1, len, seg, col, caps, mat);
    this.restore();
    return this;
  };
  /// A flat planform extruded in Y — decks, aprons, roof panels, tailgates.
  /// Points are [x, z] and the winding is fixed HERE rather than asked of the
  /// caller: the signed area says which way the list runs and a clockwise one
  /// is reversed before anything is emitted.
  Mesh.prototype.plate = function (pts, y0, y1, col, mat) {
    let area = 0;
    for (let i = 0; i < pts.length; i += 1) {
      const j = (i + 1) % pts.length;
      area += pts[i][0] * pts[j][1] - pts[j][0] * pts[i][1];
    }
    const o = area < 0 ? pts.slice().reverse() : pts;
    const top = o.map((p) => [p[0], y1, p[1]]);
    const bot = o.map((p) => [p[0], y0, p[1]]);
    this.fan(top.slice().reverse(), col, mat);
    this.fan(bot, col, mat == null ? 0.72 : mat * 0.72);
    for (let i = 0; i < o.length; i += 1) {
      const j = (i + 1) % o.length;
      this.quad(bot[j], bot[i], top[i], top[j], col, mat);
    }
    return this;
  };

  // ------------------------------------------------------- shared registry
  // WHY A REGISTRY AND A TRACE. The brief for this kit is reuse, and "we reused
  // things" is the easiest claim in art to make and the hardest to check. Every
  // shared builder is registered here and wrapped so that, when a trace is
  // running, calling it records its name. `reuse()` builds all twenty-five
  // pieces at both LODs with the trace on and reports the measured matrix, so
  // the number in the report is read off the code rather than counted by hand.
  //
  // The trace NEVER touches geometry. It is a Set of strings that no builder
  // reads; switch it off and the buffers are identical, which the checks
  // assert rather than take on trust.
  const SHARED = {};
  let TRACE = null;
  function share(name, fn) {
    const wrapped = function () {
      if (TRACE) TRACE.add(name);
      return fn.apply(null, arguments);
    };
    SHARED[name] = wrapped;
    return wrapped;
  }

  // -------------------------------------------------------- shared builders
  // These ARE the reuse. None of them opens a part range: a shared builder is
  // always called from inside a piece's own `part`, and a part opened inside
  // another one would silently break the contiguous ranges the picker indexes
  // by. Each is registered through `share` so `reuse()` can measure who calls
  // what instead of the report asserting it.

  /// One road wheel, axis along X, centred on `x` and occupying `w` of track,
  /// standing on Y=0 when `y` is the radius. `twin` draws the paired tyres a
  /// heavy axle actually carries — the most recognisable thing about a truck at
  /// close range.
  ///
  /// ON THE MAP MESH a wheel is a six-sided open drum: twelve triangles, no
  /// caps, no hub, no twin. At that range a wheel is a dark smudge under an
  /// arch, and paying forty triangles for one is exactly how a forty-prop yard
  /// blows its budget.
  const roadWheel = share("roadWheel", function (m, x, y, z, r, w, twin, col) {
    const tyre = col || P.tyre;
    if (m.detail < 1) return m.roll(x - w / 2, y, z, w, r, r, 4, tyre, false);
    const seg = r > 0.45 ? 12 : 10;
    if (twin) {
      m.roll(x - w / 2, y, z, w * 0.44, r, r, seg, tyre, true);
      m.roll(x + w * 0.06, y, z, w * 0.44, r, r, seg, tyre, true);
    } else {
      m.roll(x - w / 2, y, z, w, r, r, seg, tyre, true);
    }
    const s = x < 0 ? -1 : 1;
    m.save().move(x + s * (w / 2 + 0.006), y, z).rotY(90 * s);
    m.fan(ringPts(r * 0.52, r * 0.52, 8).map((p) => [p[0], p[1], 0]), P.hub, 1.04);
    m.restore();
    return m;
  });

  /// An axle: the beam and the wheel at each end of it. Every road vehicle here
  /// is a list of these plus a body, which is the whole argument for a kit.
  const axleSet = share("axleSet", function (m, z, y, halfTrack, r, w, twin, col) {
    if (m.detail >= 1) {
      m.beam(-halfTrack + w * 0.7, halfTrack - w * 0.7, y - 0.085, y + 0.085,
        z - 0.085, z + 0.085, col || P.chassis, 0.02);
    }
    roadWheel(m, halfTrack - w / 2, y, z, r, w, twin);
    roadWheel(m, -halfTrack + w / 2, y, z, r, w, twin);
    return m;
  });

  /// The ladder frame a rigid road vehicle is built on: two channel rails and
  /// the crossmembers between them. Visible under every truck in the kit from
  /// any angle below the waistline, which is why it is geometry and not a
  /// painted shadow.
  const ladderChassis = share("ladderChassis", function (m, z0, z1, halfW, y, h, col) {
    const t = (h == null ? 0.26 : h) / 2, c = col || P.chassis;
    m.both((mm) => mm.beam(halfW - 0.055, halfW + 0.055, y - t, y + t, z0, z1, c, 0.02));
    const n = m.d(4, 0);
    for (let i = 0; i < n; i += 1) {
      const cz = z0 + ((i + 0.5) * (z1 - z0)) / n;
      m.bar(-halfW + 0.05, halfW - 0.05, y - t * 0.55, y + t * 0.55, cz - 0.055, cz + 0.055, c, 0.86);
    }
    return m;
  });

  /// The semi-trailer chassis, and the piece of reuse the brief asks for by
  /// name: the box, the flatbed and the tanker are this plus a load-carrying
  /// body, not three trailers. It owns the kingpin, the landing legs, the
  /// tri-axle bogie, the mudguards and the rear underrun bar, because those are
  /// the same on all three.
  ///
  /// `coupled` raises the landing legs. A trailer standing in a yard has them
  /// down and a trailer under a tractor has them up; drawing one state for both
  /// is the kind of small lie that makes a yard read as a diagram.
  const semiChassis = share("semiChassis", function (m, z0, z1, halfW, deckY, opts) {
    const o = opts || {};
    const kz = z1 - 1.55;
    // The edge rails of the deck frame, tucked just UNDER the body side rather
    // than proud of it. A frame that oversails the body it carries is the
    // silhouette of a flat wagon, not of a road trailer, and it also stops the
    // body from being the widest thing on the vehicle, which on a box trailer
    // it has to be.
    m.both((mm) => mm.beam(halfW - 0.11, halfW + 0.04, COUPLING_Y, deckY, z0 + 0.05, z1, P.chassis, 0.02));
    const cross = m.d(5, 0);
    for (let i = 0; i < cross; i += 1) {
      const cz = z0 + 0.6 + ((i + 0.5) * (z1 - z0 - 1.2)) / cross;
      m.bar(-halfW + 0.06, halfW - 0.06, COUPLING_Y + 0.02, deckY - 0.03, cz - 0.06, cz + 0.06, P.chassis, 0.85);
    }
    // Kingpin plate and pin. The plate's UNDERSIDE is exactly COUPLING_Y; that
    // one number is the whole of the coupling contract and `rig` reads it back.
    m.bar(-1.02, 1.02, COUPLING_Y, COUPLING_Y + 0.06, kz - 1.05, kz + 1.35, P.steel);
    if (m.detail >= 1) m.column(0, COUPLING_Y - 0.13, kz, 0.13, 0.05, 0.05, 8, P.chrome);
    // Landing legs.
    const lz = z1 - 3.6, top = COUPLING_Y + 0.05;
    const foot = o.coupled ? 0.62 : 0.0;
    m.both((mm) => {
      mm.beam(halfW - 0.42, halfW - 0.30, foot + 0.10, top, lz - 0.06, lz + 0.06, P.galv, 0.015);
      if (m.detail >= 1) mm.bar(halfW - 0.50, halfW - 0.22, foot, foot + 0.10, lz - 0.16, lz + 0.16, P.steel);
    });
    if (m.detail >= 1) m.bar(-halfW + 0.30, -halfW + 0.42, top - 0.30, top - 0.18, lz - 0.28, lz - 0.06, P.safety);
    // Tri-axle bogie. Three axles is what a 1990 European semi-trailer runs.
    //
    // THE TRACK IS THE TRACTOR'S TRACK, deliberately and not by coincidence:
    // `tractorUnit` passes the same 1.24 to `axleSet`, so the twin tyres of the
    // unit and of the trailer stand on the same lines. That is what a coupled
    // vehicle built to one road standard looks like from behind, and it is the
    // cheapest way to keep the two halves reading as one lorry. It was briefly
    // `halfW + 0.16` here, which threw the bogie out to 2.80 m over the tyres
    // and — with the mudguards below at `halfW + 0.36` — made the trailer
    // 3.20 m wide overall: wider than its own 2.58 m body, wider than the
    // tractor across its mirrors, and half a metre outside any road envelope
    // 1990 had. `check_prop_mesh.cjs` now measures the envelope so it cannot
    // drift back.
    const bz = z0 + 1.30;
    for (let i = 0; i < 3; i += 1) axleSet(m, bz + i * 1.31, 0.50, halfW, 0.50, 0.34, true);
    if (m.detail >= 1) {
      m.both((mm) => {
        // Mudguards: inboard edge past the inner tyre of the twin, outboard
        // edge flush with the body side. A mudguard proud of the body is not a
        // mudguard, it is the first thing a gatepost takes off.
        for (let i = 0; i < 3; i += 1) {
          mm.bar(halfW - 0.40, halfW + 0.01, 1.02, 1.10, bz + i * 1.31 - 0.62, bz + i * 1.31 + 0.62, P.chassis, 0.95);
        }
        mm.bar(halfW - 0.10, halfW - 0.02, 0.62, 0.98, bz - 0.70, bz + 2.72, P.chassis, 0.8);
      });
    }
    // Rear underrun bar and lamp cluster. Legally mandatory on a 1990 trailer
    // and the thing you actually see when one is parked nose-in.
    m.bar(-halfW - 0.02, halfW + 0.02, 0.46, 0.58, z0 - 0.12, z0 + 0.02, P.steel);
    if (m.detail >= 1) {
      m.both((mm) => mm.bar(halfW - 0.30, halfW - 0.06, 0.58, 0.95, z0 - 0.06, z0 + 0.02, P.steel, 0.9));
      m.both((mm) => {
        mm.bar(halfW - 0.62, halfW - 0.28, 0.72, 0.94, z0 - 0.10, z0 - 0.04, P.lampRed);
        mm.bar(halfW - 0.62, halfW - 0.28, 0.98, 1.10, z0 - 0.10, z0 - 0.04, P.lamp);
      });
    }
    return { kingpin: [0, COUPLING_Y, kz] };
  });

  /// A road cab: shell, glazing, doors, grille, bumper and mirrors, occupying
  /// z0..z1 and rising `h` from `y0`. The tractor unit, the rigid box van and
  /// the fuel bowser are the same cab at three widths, which is what "shared
  /// part" has to mean if it is to mean anything.
  const roadCab = share("roadCab", function (m, y0, z0, z1, halfW, h, col, opts) {
    const o = opts || {};
    const y1 = y0 + h, dep = z1 - z0;
    m.bar(-halfW, halfW, y0, y1, z0, z1, col);
    // Windscreen: a recessed dark pane, not a shiny one. 1990 glass is a hole.
    const wsY0 = y0 + h * 0.50, wsY1 = y1 - h * 0.10;
    m.bar(-halfW + 0.10, halfW - 0.10, wsY0, wsY1, z1 - 0.02, z1 + 0.015, P.glass, 0.92);
    // Side glass and door shut lines.
    if (m.detail >= 1) {
      m.both((mm) => {
        mm.bar(halfW - 0.015, halfW + 0.012, wsY0, wsY1 - 0.06, z1 - dep * 0.52, z1 - 0.14, P.glass, 0.9);
        // Door shut line, waist rail and handle. Below eaves level is where a
        // cab reads from, the same argument town-mesh.js makes about openings.
        mm.bar(halfW - 0.01, halfW + 0.016, y0 + 0.06, wsY1 - 0.04, z1 - dep * 0.56, z1 - dep * 0.53, P.dark, 0.7);
        mm.bar(halfW - 0.01, halfW + 0.016, y0 + 0.06, y0 + 0.10, z1 - dep * 0.56, z1 - 0.12, P.dark, 0.7);
        mm.bar(halfW - 0.02, halfW + 0.02, wsY0 - 0.16, wsY0 - 0.08, z1 - dep * 0.30, z1 - dep * 0.22, P.dark);
      });
    }
    // Grille, bumper and lamps.
    if (m.detail >= 1) m.bar(-halfW + 0.22, halfW - 0.22, y0 + h * 0.16, wsY0 - 0.10, z1 + 0.005, z1 + 0.035, P.dark, 0.9);
    m.bar(-halfW - 0.02, halfW + 0.02, y0 - 0.30, y0 + h * 0.10, z1 + 0.01, z1 + 0.10, P.steel);
    if (m.detail >= 1) {
      m.both((mm) => {
        mm.bar(halfW - 0.62, halfW - 0.26, y0 - 0.22, y0 + 0.02, z1 + 0.08, z1 + 0.11, P.lamp);
        mm.bar(halfW - 0.24, halfW - 0.06, y0 - 0.22, y0 - 0.04, z1 + 0.08, z1 + 0.11, P.lampRed);
        // Mirror arm and head. A 1990 truck wears two big flat mirrors on a
        // tubular arm; it is most of the silhouette above the door.
        mm.bar(halfW - 0.02, halfW + 0.24, wsY1 - 0.28, wsY1 - 0.22, z1 - 0.24, z1 - 0.18, P.dark);
        mm.bar(halfW + 0.20, halfW + 0.28, wsY0 - 0.10, wsY1 - 0.18, z1 - 0.26, z1 - 0.14, P.dark);
        // Step into the cab.
        mm.bar(halfW - 0.44, halfW - 0.06, y0 - 0.52, y0 - 0.44, z1 - dep * 0.50, z1 - dep * 0.30, P.galv);
      });
    }
    // Roof fairing. Small and square: a 1990 add-on deflector, not a moulded
    // 2010s roof.
    if (o.fairing) {
      m.bar(-halfW + 0.06, halfW - 0.06, y1, y1 + o.fairing, z1 - dep * 0.62, z1 - dep * 0.10, col, 1.03);
      m.quad([-halfW + 0.06, y1 + o.fairing, z1 - dep * 0.10], [halfW - 0.06, y1 + o.fairing, z1 - dep * 0.10],
        [halfW - 0.06, y1, z1 - 0.02], [-halfW + 0.06, y1, z1 - 0.02], col, 1.06);
    }
    return m;
  });

  /// A closed box body: shell, side ribs, roof, rear doors with their locking
  /// bars, and the rub rail down each side. The box semi-trailer, the rigid box
  /// van and the panel van's load space are all this.
  const boxBody = share("boxBody", function (m, z0, z1, halfW, y0, y1, col, opts) {
    const o = opts || {};
    m.bar(-halfW, halfW, y0, y1, z0, z1, col);
    if (m.detail >= 1) {
      const ribs = Math.max(3, Math.round((z1 - z0) / 1.35));
      m.both((mm) => {
        for (let i = 1; i < ribs; i += 1) {
          const rz = z0 + ((z1 - z0) * i) / ribs;
          mm.bar(halfW, halfW + 0.025, y0 + 0.08, y1 - 0.05, rz - 0.035, rz + 0.035, col, 0.94);
        }
        mm.bar(halfW, halfW + 0.04, y0 + 0.10, y0 + 0.20, z0 + 0.06, z1 - 0.06, P.steel);
        mm.bar(halfW - 0.02, halfW + 0.03, y1 - 0.10, y1, z0, z1, col, 1.05);
      });
    }
    // Rear doors: two leaves, a shut line and four locking bars.
    const dz = z0 - 0.02;
    if (m.detail >= 1) {
      m.bar(-halfW + 0.04, -0.02, y0 + 0.05, y1 - 0.05, dz - 0.02, dz + 0.01, col, 0.88);
      m.bar(0.02, halfW - 0.04, y0 + 0.05, y1 - 0.05, dz - 0.02, dz + 0.01, col, 0.88);
      for (const bx of [-halfW + 0.28, -0.26, 0.26, halfW - 0.28]) {
        m.bar(bx - 0.03, bx + 0.03, y0 + 0.10, y1 - 0.10, dz - 0.06, dz - 0.02, P.steel);
      }
      if (o.tailLift) {
        m.bar(-halfW + 0.10, halfW - 0.10, y0 - 0.10, y0 - 0.02, z0 - 1.35, z0 - 0.04, P.galv);
        m.both((mm) => mm.bar(halfW - 0.30, halfW - 0.18, y0 - 0.02, y0 + 0.34, z0 - 0.34, z0 - 0.20, P.steel));
      }
    }
    return m;
  });

  /// The ISO box, and the single most reused shape in the kit: it is the 20 ft
  /// piece, the 40 ft piece, every box in the stack, the load on the flat
  /// wagon and the thing both cranes are holding.
  ///
  /// The corrugation is a FOLDED SHEET, two triangles a fold, rather than ribs
  /// standing off a flat panel — same read, a quarter of the cost, and cost is
  /// the whole reason a stack of six fits in one prop budget. `lite` drops the
  /// corner castings, hinges and locking bars for boxes inside a stack, where
  /// they are occluded by the box in front.
  const isoContainer = share("isoContainer", function (m, cx, y0, cz, len, col, opts) {
    const o = opts || {};
    const w = 2.44, h = o.height == null ? 2.59 : o.height;
    const hw = w / 2, hl = len / 2, y1 = y0 + h;
    const x0 = cx - hw, x1 = cx + hw, z0 = cz - hl, z1 = cz + hl;
    // `named` lets the container open its OWN part ranges, and it is set only
    // when the container IS the whole prop. Inside a stack, a wagon or a crane
    // it is a sub-assembly and the caller has already opened a part around it —
    // a part opened inside another one would break the contiguous ranges the
    // picker indexes by.
    const grp = o.named ? (name, fn) => { m.part(name, fn); } : (name, fn) => { fn(); };
    if (m.detail < 1 || o.plain) {
      grp("container / box", () => {
        m.bar(x0, x1, y0, y1, z0, z1, col);
        if (m.detail >= 1) m.bar(x0 + 0.05, x1 - 0.05, y0 + 0.08, y1 - 0.08, z0 - 0.02, z0 + 0.01, col, 0.86);
      });
      return m;
    }
    const c = 0.14;                       // corner post and rail width
    grp("container / corner posts and rails", () => {
      for (const sx of [x0, x1]) {
        for (const sz of [z0, z1]) {
          m.bar(sx === x0 ? sx : sx - c, sx === x0 ? sx + c : sx,
            y0, y1, sz === z0 ? sz : sz - c, sz === z0 ? sz + c : sz, P.steel, 0.96);
        }
      }
      for (const sx of [x0, x1]) {
        const a = sx === x0 ? sx : sx - 0.09, b = sx === x0 ? sx + 0.09 : sx;
        m.bar(a, b, y1 - 0.11, y1, z0, z1, P.steel, 1.0);
        m.bar(a, b, y0, y0 + 0.13, z0, z1, P.steel, 0.9);
      }
      m.bar(x0, x1, y1 - 0.11, y1, z1 - 0.09, z1, P.steel, 1.0);
      m.bar(x0, x1, y1 - 0.11, y1, z0, z0 + 0.09, P.steel, 1.0);
      m.bar(x0, x1, y0, y0 + 0.13, z1 - 0.09, z1, P.steel, 0.9);
      m.bar(x0, x1, y0, y0 + 0.13, z0, z0 + 0.09, P.steel, 0.9);
    });
    const amp = 0.035, ya = y0 + 0.13, yb = y1 - 0.11;
    grp("container / corrugated sides", () => {
      // Authored on +X and mirrored, so the two sides cannot drift apart.
      const folds = Math.max(6, Math.round((len - 2 * c) / (o.lite ? 1.1 : 0.56)));
      m.both((mm) => {
        for (let i = 0; i < folds; i += 1) {
          const za = z0 + c + ((len - 2 * c) * i) / folds;
          const zb = z0 + c + ((len - 2 * c) * (i + 1)) / folds;
          const xa = hw - (i % 2 ? amp : 0), xb = hw - (i % 2 ? 0 : amp);
          mm.quad([xa, ya, za], [xa, yb, za], [xb, yb, zb], [xb, ya, zb], col);
        }
      });
    });
    grp("container / roof, floor and front", () => {
      m.bar(x0, x1, y1 - 0.10, y1 - 0.04, z0 + c, z1 - c, col, 1.05);
      m.quad([x0, y0 + 0.02, z0], [x1, y0 + 0.02, z0], [x1, y0 + 0.02, z1], [x0, y0 + 0.02, z1], P.dark, 0.7);
      m.bar(x0 + c, x1 - c, ya, yb, z1 - 0.05, z1 - 0.01, col, 1.02);
    });
    grp("container / doors and locking bars", () => {
      m.bar(x0 + c, cx - 0.02, ya, yb, z0 + 0.01, z0 + 0.05, col, 0.86);
      m.bar(cx + 0.02, x1 - c, ya, yb, z0 + 0.01, z0 + 0.05, col, 0.86);
      if (o.lite) return;
      for (const bx of [x0 + 0.34, cx - 0.24, cx + 0.24, x1 - 0.34]) {
        m.bar(bx - 0.028, bx + 0.028, ya + 0.05, yb - 0.05, z0 - 0.02, z0 + 0.012, P.steel, 1.05);
        m.bar(bx - 0.05, bx + 0.05, ya + 0.55, ya + 0.68, z0 - 0.05, z0 + 0.01, P.steel, 1.1);
      }
    });
    if (!o.lite) {
      grp("container / corner castings", () => {
        // The eight steel boxes everything on a quay actually grips.
        for (const sx of [x0, x1 - 0.18]) {
          for (const sy of [y0, y1 - 0.16]) {
            for (const sz of [z0, z1 - 0.18]) m.bar(sx, sx + 0.18, sy, sy + 0.16, sz, sz + 0.18, P.dark, 1.0);
          }
        }
      });
    }
    return m;
  });

  /// A horizontal barrel with dished ends, stiffener rings and a walkway. The
  /// tanker semi-trailer, the rail tank wagon and the fuel bowser are the same
  /// barrel at three lengths and two radii.
  ///
  /// TRUTHFULNESS, stated where the geometry is: this shape carries nothing.
  /// The simulation has no fuel, no liquid inventory and no vessel capacity for
  /// it to represent, and a compartment line painted on the side would be
  /// inventing one.
  const tankBarrel = share("tankBarrel", function (m, y, z0, z1, r, col, opts) {
    const o = opts || {};
    const seg = m.segs(r > 0.9 ? 16 : 12, 6);
    const len = z1 - z0;
    m.drum(0, y, z0, len, r, r, seg, col, m.detail < 1);
    // Dished ends: one ring pulled in and capped, which is what a pressed end
    // looks like from any distance that matters here. The far end is drawn by
    // turning the whole thing round rather than by extruding a negative length,
    // because a negative length reverses the winding. The map mesh takes flat
    // caps instead: a 220 mm dish is well under a pixel at that range.
    if (m.detail >= 1) {
      for (const [zz, yaw] of [[z0, 180], [z1, 0]]) {
        m.save().move(0, y, zz).rotY(yaw);
        m.tube(r, r * 0.55, 0.22, seg, col, true);
        m.restore();
      }
    }
    if (m.detail >= 1) {
      const rings = Math.max(2, Math.round(len / 2.6));
      for (let i = 1; i < rings; i += 1) {
        const rz = z0 + (len * i) / rings;
        m.drum(0, y, rz - 0.035, 0.07, r + 0.03, r + 0.03, seg, P.steel, false);
      }
      // Walkway and manlids on the crown.
      m.bar(-0.30, 0.30, y + r - 0.01, y + r + 0.04, z0 + 0.4, z1 - 0.4, P.galv);
      const lids = o.lids == null ? Math.max(1, Math.round(len / 3.4)) : o.lids;
      for (let i = 0; i < lids; i += 1) {
        const lz = z0 + (len * (i + 0.5)) / lids;
        m.column(0.42, y + r - 0.06, lz, 0.14, 0.24, 0.22, 8, P.steel, true);
      }
    }
    return m;
  });

  /// A vertical cylindrical vessel: shell, conical roof, stiffener rings, and
  /// optionally a discharge hopper on legs. A bulk storage tank has the flat
  /// bottom; a silo has the hopper. Same builder, one flag.
  const verticalVessel = share("verticalVessel", function (m, x, z, y0, h, r, col, opts) {
    const o = opts || {};
    const seg = m.segs(16, 5);
    m.column(x, y0, z, h, r, r, seg, col, false);
    // Conical roof. A shallow cone is what sheds rain off a tank and it reads
    // as a tank from further away than a flat lid does.
    m.save().move(x, y0 + h, z).rotX(-90);
    m.tube(r, r * 0.10, o.roof == null ? r * 0.28 : o.roof, seg, col, m.detail >= 1, 1.05);
    m.restore();
    if (o.hopper) {
      // rotX(+90) aims the loft down, which is how a discharge cone is drawn
      // without extruding a negative length past `tube`.
      m.save().move(x, y0, z).rotX(90);
      m.tube(r, r * 0.22, o.hopper, seg, col, m.detail >= 1, 0.9);
      m.restore();
      const legs = m.d(4, 2);
      for (let i = 0; i < legs; i += 1) {
        const a = (i / legs) * Math.PI * 2;
        m.beam(x + Math.cos(a) * r - 0.09, x + Math.cos(a) * r + 0.09, 0, y0,
          z + Math.sin(a) * r - 0.09, z + Math.sin(a) * r + 0.09, P.steel, 0.02);
      }
    } else if (y0 > 0.02) {
      // A flat-bottomed vessel stands on a concrete ring, which is also what
      // puts the piece on Y=0 without the shell itself being buried.
      m.column(x, 0, z, y0, r + 0.12, r + 0.12, seg, P.concrete, true);
    }
    if (m.detail >= 1) {
      for (const f of [0.34, 0.68]) {
        m.column(x, y0 + h * f, z, 0.09, r + 0.035, r + 0.035, seg, P.steel, false);
      }
    }
    return m;
  });

  /// A two-axle rail bogie. The locomotive and all three wagons ride on these,
  /// which is the reuse that makes a rail siding affordable at all.
  const railBogie = share("railBogie", function (m, z, col) {
    const c = col || P.chassis, r = 0.46, halfT = 0.76, w = 0.14;
    if (m.detail >= 1) m.both((mm) => mm.beam(halfT - 0.24, halfT - 0.10, 0.34, 0.72, z - 1.05, z + 1.05, c, 0.02));
    m.bar(-halfT + 0.10, halfT - 0.10, 0.40, 0.66, z - 0.95, z + 0.95, c, 0.9);
    for (const az of [z - 0.90, z + 0.90]) {
      if (m.detail >= 1) m.roll(-halfT + 0.02, r, az, halfT * 2 - 0.04, 0.06, 0.06, 8, P.steel, false);
      for (const s of [-1, 1]) {
        const wx = s * (halfT - 0.09);
        m.roll(wx - 0.07, r, az, 0.14, r, r, m.segs(12, 4), P.railhead, m.detail >= 1);
        if (m.detail >= 1) {
          // The flange, on the inboard side of the tread where a rail wheel
          // actually carries it.
          m.roll(wx - s * 0.10, r, az, 0.03, r + 0.045, r + 0.045, 12, P.railhead, true, 0.92);
          m.bar(wx - 0.13, wx + 0.13, 0.60, 0.80, az - 0.16, az + 0.16, c, 0.95);
        }
      }
    }
    return m;
  });

  /// The rail underframe: solebars, headstocks, buffers and a centre coupler.
  /// Shared by the locomotive and every wagon, at four lengths.
  const railUnderframe = share("railUnderframe", function (m, z0, z1, halfW, deckY) {
    m.both((mm) => mm.beam(halfW - 0.09, halfW, deckY - 0.34, deckY, z0, z1, P.chassis, 0.02));
    for (const zz of [z0, z1]) {
      const d = zz === z0 ? 1 : -1;
      m.bar(-halfW, halfW, deckY - 0.34, deckY, zz, zz + d * 0.14, P.chassis, 0.95);
      if (m.detail >= 1) {
        m.both((mm) => mm.bar(halfW - 0.52, halfW - 0.22, deckY - 0.30, deckY - 0.10, zz, zz - d * 0.22, P.steel));
        m.column(0, deckY - 0.40, zz - d * 0.05, 0.22, 0.10, 0.10, 8, P.dark, true);
        m.both((mm) => mm.column(halfW - 0.37, deckY - 0.20, zz - d * 0.30, 0.30, 0.17, 0.17, 8, P.steel, true));
      }
    }
    m.bar(-halfW, halfW, deckY, deckY + 0.05, z0 + 0.14, z1 - 0.14, P.chassis, 1.02);
    return m;
  });

  /// A lattice structural run along +Z from the origin, section `w` by `h`:
  /// four chords and a zig-zag of diagonals in each side face. Crane legs,
  /// crane girders, the conveyor trestle tops and the tall light masts are all
  /// this at different proportions.
  ///
  /// ON THE MAP MESH IT COLLAPSES TO ONE SOLID BAR. A lattice at map range is a
  /// grey line; drawing the diagonals there would cost more than the entire map
  /// budget of the crane they belong to.
  const lattice = share("lattice", function (m, len, w, h, bays, col) {
    const c = col || P.craneBody, hw = w / 2, hh = h / 2;
    if (m.detail < 1) return m.bar(-hw, hw, -hh, hh, 0, len, c);
    // Chords are plain boxes and NOT chamfered, which is the one place in the
    // file where the arris was not worth its price: a crane carries eight to
    // sixteen lattice runs and 16 extra triangles a chord is 500 off a 2,500
    // budget for an edge highlight on a member 60 mm wide.
    for (const sx of [-hw, hw]) {
      for (const sy of [-hh, hh]) {
        m.bar(sx - 0.055, sx + 0.055, sy - 0.055, sy + 0.055, 0, len, c);
      }
    }
    const n = Math.max(2, bays | 0), step = len / n;
    for (const sx of [-hw, hw]) {
      for (let i = 0; i < n; i += 1) {
        const za = i * step, zb = (i + 1) * step;
        const ya = i % 2 ? hh : -hh, yb = i % 2 ? -hh : hh;
        m.quad([sx - 0.035, ya, za], [sx - 0.035, yb, zb], [sx + 0.035, yb, zb], [sx + 0.035, ya, za], c, 0.92);
        m.quad([sx + 0.035, ya, za], [sx + 0.035, yb, zb], [sx - 0.035, yb, zb], [sx - 0.035, ya, za], c, 0.82);
        m.bar(sx - 0.04, sx + 0.04, -hh, hh, zb - 0.04, zb + 0.04, c, 0.88);
      }
    }
    return m;
  });

  /// A vertical ladder or stair stringer with rungs, climbing y0..y1 at (x, z).
  /// Tanks, silos, cranes, the conveyor gantry and the light masts all need one
  /// and it is the same one.
  const ladderRun = share("ladderRun", function (m, x, z, y0, y1, w, col) {
    if (m.detail < 1) return m;
    const c = col || P.galv, hw = w / 2;
    m.both((mm) => mm.bar(x + hw - 0.03, x + hw + 0.03, y0, y1, z - 0.03, z + 0.03, c));
    const rungs = Math.max(2, Math.round((y1 - y0) / 0.32));
    for (let i = 1; i < rungs; i += 1) {
      const ry = y0 + ((y1 - y0) * i) / rungs;
      m.bar(x - hw, x + hw, ry - 0.022, ry + 0.022, z - 0.022, z + 0.022, c, 1.05);
    }
    return m;
  });

  /// A floodlight head: a frame and a row of lamps aimed down. Light masts, the
  /// yard gantry and the quay crane all wear one.
  const floodHead = share("floodHead", function (m, x, y, z, n, span, col) {
    const c = col || P.galv;
    m.bar(x - span / 2, x + span / 2, y, y + 0.09, z - 0.06, z + 0.06, c);
    if (m.detail < 1) return m;
    for (let i = 0; i < n; i += 1) {
      const lx = x - span / 2 + (span * (i + 0.5)) / n;
      m.bar(lx - 0.17, lx + 0.17, y - 0.28, y - 0.02, z - 0.13, z + 0.13, P.dark, 0.95);
      if (m.detail >= 1) {
        m.quad([lx - 0.15, y - 0.27, z + 0.13], [lx + 0.15, y - 0.27, z + 0.13],
          [lx + 0.15, y - 0.04, z + 0.14], [lx - 0.15, y - 0.04, z + 0.14], P.lamp, 1.12);
      }
    }
    return m;
  });

  /// One shipping pallet. Three bearers, five top boards, three bottom boards —
  /// the timber pallet a 1990 yard is knee-deep in. Never a piece of its own:
  /// at 1.2 x 0.8 m it cannot fill the near budget honestly, so it exists as a
  /// shared part and the PIECES are the stacks it makes.
  const palletDeck = share("palletDeck", function (m, x, y, z, yaw, col) {
    const c = col || P.timber;
    m.save().move(x, y, z).rotY(yaw || 0);
    if (m.detail < 1) {
      m.bar(-0.60, 0.60, 0, 0.145, -0.40, 0.40, c);
      m.restore();
      return m;
    }
    for (const bz of [-0.335, 0, 0.335]) {
      m.bar(-0.60, 0.60, 0.022, 0.100, bz - 0.048, bz + 0.048, c, 0.9);
      m.bar(-0.60, 0.60, 0, 0.022, bz - 0.058, bz + 0.058, P.board, 0.82);
    }
    for (let i = 0; i < 5; i += 1) {
      const bx = -0.60 + 0.06 + (i * 1.08) / 4;
      m.bar(bx - 0.058, bx + 0.058, 0.100, 0.122, -0.40, 0.40, P.board, 1.04);
    }
    m.restore();
    return m;
  });

  /// A container spreader: the head block, the beam and four twistlocks. The
  /// yard gantry and the quay crane both hang one, and it is the part that
  /// tells you what a crane is for.
  const spreader = share("spreader", function (m, x, y, z, len, col) {
    const c = col || P.safety, hl = len / 2;
    m.bar(x - 1.24, x + 1.24, y, y + 0.34, z - hl + 0.2, z + hl - 0.2, c);
    m.bar(x - 1.24, x + 1.24, y + 0.34, y + 0.46, z - 0.9, z + 0.9, c, 1.05);
    if (m.detail >= 1) {
      for (const sx of [x - 1.16, x + 1.16]) {
        for (const sz of [z - hl + 0.14, z + hl - 0.14]) {
          m.bar(sx - 0.11, sx + 0.11, y - 0.16, y, sz - 0.11, sz + 0.11, P.dark);
        }
      }
      m.both((mm) => mm.bar(x + 0.6, x + 0.9, y + 0.46, y + 0.62, z - 0.16, z + 0.16, P.steel));
    }
    return m;
  });

  /// A crane machinery house: the box that holds the hoist, its door, its
  /// louvres and its roof. Both cranes carry one and they are the same box.
  const machineHouse = share("machineHouse", function (m, x, y, z, w, h, dep, col) {
    const c = col || P.craneBody;
    m.bar(x - w / 2, x + w / 2, y, y + h, z - dep / 2, z + dep / 2, c);
    if (m.detail >= 1) {
      m.bar(x - w / 2 - 0.06, x + w / 2 + 0.06, y + h, y + h + 0.08, z - dep / 2 - 0.06, z + dep / 2 + 0.06, P.steel, 1.05);
      m.bar(x - 0.42, x + 0.42, y + 0.05, y + 2.10, z + dep / 2, z + dep / 2 + 0.02, P.dark, 0.9);
      for (let i = 0; i < 3; i += 1) {
        m.bar(x + w / 2, x + w / 2 + 0.02, y + 0.6 + i * 0.42, y + 0.9 + i * 0.42, z - dep / 2 + 0.2, z + dep / 2 - 0.2, P.dark, 0.85);
      }
    }
    return m;
  });

  // --------------------------------------------------------------- pieces
  // Each builder assembles shared parts and adds only what is genuinely its
  // own. Every emission is inside a `part`, because the ranges have to cover
  // the buffer with no gap for the picker to be able to index them at all.

  // ------------------------------------------------------------ road: trucks
  function tractorUnit(m, seed, o) {
    const cab = askOne(seed, 1, P.livery);
    const halfW = 1.24, fwZ = -1.60;
    m.part("running gear / chassis", () => ladderChassis(m, -3.05, 2.45, 0.44, 1.00, 0.26));
    m.part("running gear / steer axle", () => axleSet(m, 1.85, 0.52, halfW, 0.52, 0.34, false));
    m.part("running gear / drive axles", () => {
      axleSet(m, -1.20, 0.52, halfW, 0.52, 0.34, true);
      axleSet(m, -2.55, 0.52, halfW, 0.52, 0.34, true);
    });
    m.part("coupling / fifth wheel", () => {
      if (m.detail < 1) {
        m.bar(-0.74, 0.74, 1.02, COUPLING_Y, fwZ - 0.75, fwZ + 0.80, P.steel);
        return;
      }
      // The throat is a real gap between two plates rather than a dark stripe
      // painted on one: a kingpin has to have somewhere to be.
      m.bar(-0.74, 0.74, 1.02, COUPLING_Y, fwZ + 0.05, fwZ + 0.80, P.steel);
      m.bar(-0.74, -0.09, 1.02, COUPLING_Y, fwZ - 0.75, fwZ + 0.05, P.steel);
      m.bar(0.09, 0.74, 1.02, COUPLING_Y, fwZ - 0.75, fwZ + 0.05, P.steel);
      m.both((mm) => mm.bar(0.30, 0.62, 0.86, 1.02, fwZ - 0.35, fwZ + 0.45, P.chassis, 0.85));
    });
    m.part("cab / shell and glazing", () => roadCab(m, 1.20, 0.30, 2.50, 1.22, 2.05, cab, { fairing: 0.42 }));
    m.part("cab / tanks, stack and catwalk", () => {
      if (m.detail < 1) return;
      m.drum(-0.62, 0.72, -0.55, 1.55, 0.34, 0.34, m.segs(12, 6), P.chrome, true);
      m.bar(0.48, 0.90, 0.60, 0.94, -0.55, 0.55, P.chassis, 0.9);
      m.column(0.68, 1.14, 0.20, m.d(1.95, 1.20), 0.085, 0.075, m.segs(8, 4), P.chrome, true);
      if (m.detail >= 1) {
        m.bar(-0.80, 0.80, 1.13, 1.19, -0.30, 0.28, P.galv, 1.05);
        m.both((mm) => mm.bar(0.86, 1.28, 1.06, 1.14, -3.00, -0.62, P.chassis, 0.95));
      }
    });
    m.part("lighting / rear cluster and marker board", () => {
      if (m.detail < 1) return;
      m.both((mm) => {
        mm.bar(0.44, 0.78, 0.72, 1.00, -3.10, -3.03, P.lampRed);
        if (m.detail >= 1) mm.bar(0.44, 0.78, 1.04, 1.16, -3.10, -3.03, P.lamp);
      });
      m.bar(-0.34, 0.34, 0.56, 0.72, -3.12, -3.06, P.panel, 0.9);
    });
    return { sockets: { fifth_wheel: [0, COUPLING_Y, fwZ] }, livery: cab };
  }

  function trailerCommon(m, seed, o, deckY) {
    const z0 = -6.80, z1 = 6.80, halfW = 1.24;
    let sock = null;
    m.part("running gear / chassis, bogie and legs", () => {
      sock = semiChassis(m, z0, z1, halfW, deckY, { coupled: !!o.coupled });
    });
    return { z0, z1, halfW, sock };
  }

  function trailerBox(m, seed, o) {
    const body = askOne(seed, 3, [P.panel, P.livery[0], P.livery[1], P.livery[5]]);
    const t = trailerCommon(m, seed, o, 1.30);
    m.part("body / box", () => boxBody(m, t.z0, t.z1, 1.25, 1.30, 4.02, body, {}));
    m.part("body / nose fairing and front bulkhead", () => {
      m.bar(-1.25, 1.25, 4.02, 4.06, t.z1 - 3.2, t.z1, body, 1.06);
      if (m.detail >= 1) {
        m.both((mm) => mm.bar(1.20, 1.27, 1.34, 1.52, t.z0 + 0.2, t.z1 - 0.2, P.safety, 1.0));
      }
    });
    return { sockets: { kingpin: t.sock.kingpin }, livery: body };
  }

  function trailerFlatbed(m, seed, o) {
    const t = trailerCommon(m, seed, o, 1.28);
    m.part("body / deck", () => {
      m.bar(-1.25, 1.25, 1.28, 1.40, t.z0, t.z1, P.timber, 0.98);
      if (m.detail >= 1) {
        for (let i = 0; i < 11; i += 1) {
          const bz = t.z0 + 0.3 + (i * (t.z1 - t.z0 - 0.6)) / 10;
          m.bar(-1.25, 1.25, 1.40, 1.405, bz - 0.03, bz + 0.03, P.board, 0.86);
        }
      }
    });
    m.part("body / headboard and stanchions", () => {
      m.bar(-1.25, 1.25, 1.40, 3.10, t.z1 - 0.14, t.z1, P.galv);
      if (m.detail < 1) return;
      const posts = m.d(8, 3);
      m.both((mm) => {
        for (let i = 0; i < posts; i += 1) {
          const pz = t.z0 + 0.6 + (i * (t.z1 - t.z0 - 1.6)) / (posts - 1);
          mm.bar(1.19, 1.27, 1.40, 1.62, pz - 0.05, pz + 0.05, P.steel);
        }
      });
    });
    m.part("body / lashing rings and spare wheel", () => {
      if (m.detail < 1) return;
      m.both((mm) => {
        for (let i = 0; i < 5; i += 1) {
          const rz = t.z0 + 1.0 + (i * (t.z1 - t.z0 - 2.4)) / 4;
          mm.bar(1.22, 1.29, 1.20, 1.30, rz - 0.07, rz + 0.07, P.dark);
        }
      });
      m.roll(-0.28, 0.86, t.z1 - 5.0, 0.30, 0.50, 0.50, 10, P.tyre, true);
    });
    return { sockets: { kingpin: t.sock.kingpin } };
  }

  function trailerTanker(m, seed, o) {
    const shell = askOne(seed, 5, [P.tankShell, P.galv, P.livery[0]]);
    const t = trailerCommon(m, seed, o, 1.34);
    m.part("body / barrel", () => tankBarrel(m, 2.42, t.z0 + 0.55, t.z1 - 0.95, 1.03, shell, { lids: 3 }));
    m.part("body / saddles and frame", () => {
      for (let i = 0; i < m.d(3, 1); i += 1) {
        const sz = t.z0 + 1.4 + i * 3.8;
        m.bar(-1.06, 1.06, 1.34, 1.62, sz - 0.14, sz + 0.14, P.chassis, 0.92);
      }
      m.bar(-1.24, 1.24, 1.34, 1.44, t.z0 + 0.5, t.z1 - 0.9, P.chassis, 0.88);
    });
    m.part("body / rear cabinet, ladder and handrail", () => {
      m.bar(-1.10, 1.10, 1.40, 2.42, t.z0 + 0.05, t.z0 + 0.62, P.galv, 0.95);
      ladderRun(m, 0.72, t.z0 + 0.02, 1.40, 3.52, 0.46, P.galv);
      if (m.detail >= 1) {
        m.both((mm) => {
          mm.bar(0.28, 0.34, 3.45, 3.52, t.z0 + 1.2, t.z1 - 1.2, P.galv, 1.06);
          for (let i = 0; i < 5; i += 1) {
            const hz = t.z0 + 1.4 + (i * (t.z1 - t.z0 - 3.2)) / 4;
            mm.bar(0.28, 0.34, 3.42, 3.52, hz - 0.03, hz + 0.03, P.galv);
          }
        });
      }
    });
    return { sockets: { kingpin: t.sock.kingpin } };
  }

  function rigidBoxVan(m, seed, o) {
    const cab = askOne(seed, 7, P.livery), body = askOne(seed, 8, [P.panel, P.livery[0], P.livery[3]]);
    const halfW = 1.14;
    m.part("running gear / chassis", () => ladderChassis(m, -3.60, 3.55, 0.40, 0.90, 0.22));
    m.part("running gear / axles", () => {
      axleSet(m, 2.55, 0.46, halfW, 0.46, 0.28, false);
      axleSet(m, -1.90, 0.46, halfW, 0.46, 0.28, true);
    });
    m.part("cab / shell and glazing", () => roadCab(m, 1.05, 1.30, 3.50, 1.10, 1.85, cab, {}));
    m.part("body / box", () => boxBody(m, -3.62, 1.22, 1.15, 1.12, 3.55, body, { tailLift: true }));
    m.part("body / mounting rails and mudguards", () => {
      m.bar(-1.16, 1.16, 1.02, 1.12, -3.60, 1.22, P.chassis, 0.86);
      if (m.detail >= 1) m.both((mm) => mm.bar(0.80, 1.20, 0.96, 1.04, -2.70, -1.05, P.chassis, 0.95));
    });
    return { livery: cab };
  }

  function serviceVan(m, seed) {
    const body = askOne(seed, 11, P.livery);
    const halfW = 0.88, r = 0.32;
    m.part("running gear / axles", () => {
      axleSet(m, 1.45, r, halfW, r, 0.20, false);
      axleSet(m, -1.35, r, halfW, r, 0.20, false);
    });
    m.part("body / load space", () => {
      m.bar(-0.94, 0.94, 0.44, 2.24, -2.42, 1.10, body);
      m.bar(-0.90, 0.90, 2.24, 2.30, -2.36, 1.02, body, 1.06);
      // Rear doors, split, with a shut line you can see.
      m.bar(-0.90, -0.02, 0.52, 2.16, -2.44, -2.41, body, 0.86);
      m.bar(0.02, 0.90, 0.52, 2.16, -2.44, -2.41, body, 0.86);
    });
    m.part("body / cab and glazing", () => {
      m.bar(-0.92, 0.92, 0.52, 1.30, 1.10, 2.36, body, 0.98);
      // A one-box van in 1990 has a short bonnet and a steeply raked screen.
      m.quad([-0.86, 2.16, 1.12], [0.86, 2.16, 1.12], [0.86, 1.32, 1.98], [-0.86, 1.32, 1.98], P.glass, 1.0);
      m.bar(-0.94, 0.94, 1.30, 2.18, 1.06, 1.14, body, 0.96);
      m.both((mm) => {
        mm.bar(0.90, 0.95, 1.30, 1.92, 0.20, 1.05, P.glass, 0.92);
        if (m.detail >= 1) mm.bar(0.90, 0.96, 0.60, 1.94, 0.16, 0.20, P.dark, 0.7);
      });
    });
    m.part("body / bumpers, arches and lamps", () => {
      m.bar(-0.94, 0.94, 0.30, 0.52, 2.30, 2.44, P.dark);
      m.bar(-0.94, 0.94, 0.30, 0.52, -2.50, -2.40, P.dark);
      if (m.detail >= 1) {
        m.both((mm) => {
          mm.bar(0.94, 0.99, 0.30, 0.66, 1.10, 1.80, P.dark, 0.9);
          mm.bar(0.94, 0.99, 0.30, 0.66, -1.70, -1.00, P.dark, 0.9);
          mm.bar(0.40, 0.82, 0.60, 0.82, 2.42, 2.46, P.lamp);
          mm.bar(0.40, 0.82, 1.55, 2.05, -2.47, -2.43, P.lampRed);
          mm.bar(0.92, 1.10, 1.70, 1.92, 1.20, 1.30, P.dark);
        });
      }
    });
    return { livery: body };
  }

  function bus(m, seed) {
    const body = askOne(seed, 13, [P.livery[2], P.livery[1], P.livery[3], P.livery[0]]);
    const halfW = 1.22, r = 0.50;
    m.part("running gear / axles", () => {
      axleSet(m, 3.55, r, halfW, r, 0.30, false);
      axleSet(m, -3.10, r, halfW, r, 0.30, true);
    });
    m.part("body / shell", () => {
      m.bar(-1.25, 1.25, 0.58, 3.02, -5.50, 5.50, body);
      m.bar(-1.20, 1.20, 3.02, 3.10, -5.40, 5.40, P.panel, 1.06);
      if (m.detail >= 1) m.bar(-1.27, 1.27, 0.30, 0.62, -5.30, 5.30, P.dark, 0.8);
    });
    m.part("body / glazing band", () => {
      const n = m.d(9, 3);
      m.both((mm) => {
        for (let i = 0; i < n; i += 1) {
          const a = -4.90 + (i * 9.6) / n, b = a + 9.6 / n - 0.16;
          mm.bar(1.24, 1.29, 1.76, 2.60, a, b, P.glass, 0.94);
        }
      });
      m.bar(-1.10, 1.10, 1.60, 2.62, 5.50, 5.54, P.glass, 1.0);
      m.bar(-1.10, 1.10, 1.76, 2.50, -5.54, -5.50, P.glass, 0.9);
    });
    m.part("body / doors, destination panel and lamps", () => {
      for (const dz of [3.95, -0.60]) {
        m.bar(1.24, 1.30, 0.62, 2.60, dz - 0.55, dz + 0.55, P.dark, 0.85);
        if (m.detail >= 1) m.bar(1.25, 1.31, 1.76, 2.52, dz - 0.50, dz + 0.50, P.glass, 0.92);
      }
      m.bar(-0.85, 0.85, 2.66, 2.96, 5.50, 5.53, P.dark, 0.9);
      if (m.detail >= 1) {
        m.both((mm) => {
          mm.bar(0.55, 1.05, 0.66, 1.00, 5.50, 5.56, P.lamp);
          mm.bar(0.55, 1.05, 0.66, 1.10, -5.56, -5.50, P.lampRed);
          mm.bar(1.26, 1.44, 2.20, 2.60, 4.90, 5.00, P.dark);
          mm.bar(1.25, 1.34, 0.58, 0.94, 3.10, 4.05, P.dark, 0.9);
          mm.bar(1.25, 1.34, 0.58, 0.94, -3.60, -2.60, P.dark, 0.9);
        });
      }
    });
    return { livery: body };
  }

  function car(m, seed) {
    const paint = askOne(seed, 17, P.car);
    const halfW = 0.82, r = 0.30;
    m.part("running gear / axles", () => {
      axleSet(m, 1.32, r, halfW, r, 0.19, false);
      axleSet(m, -1.36, r, halfW, r, 0.19, false);
    });
    m.part("body / lower body", () => {
      m.bar(-0.84, 0.84, 0.34, 1.02, -2.16, 2.16, paint);
      m.bar(-0.86, 0.86, 0.24, 0.40, -2.10, 2.10, P.dark, 0.8);
      // Bonnet and boot lids, a shade proud of the body, so a three-box saloon
      // reads as three boxes and not as one brick.
      m.bar(-0.80, 0.80, 1.02, 1.06, 0.92, 2.10, paint, 1.05);
      m.bar(-0.80, 0.80, 1.02, 1.06, -2.10, -1.10, paint, 1.05);
    });
    m.part("body / cabin and glass", () => {
      m.bar(-0.76, 0.76, 1.06, 1.44, -1.06, 0.88, paint, 1.02);
      m.quad([-0.72, 1.06, 0.90], [0.72, 1.06, 0.90], [0.70, 1.42, 0.62], [-0.70, 1.42, 0.62], P.glass, 1.05);
      m.quad([-0.70, 1.42, -0.86], [0.70, 1.42, -0.86], [0.72, 1.06, -1.10], [-0.72, 1.06, -1.10], P.glass, 0.95);
      m.both((mm) => {
        mm.bar(0.76, 0.79, 1.12, 1.40, -0.80, -0.06, P.glass, 0.94);
        mm.bar(0.76, 0.79, 1.12, 1.40, 0.02, 0.62, P.glass, 0.94);
        mm.bar(-0.74, 0.74, 1.44, 1.47, -1.00, 0.80, P.panel, 1.06);
      });
    });
    m.part("body / bumpers, lamps and trim", () => {
      m.bar(-0.86, 0.86, 0.52, 0.72, 2.16, 2.24, P.dark);
      m.bar(-0.86, 0.86, 0.52, 0.72, -2.24, -2.16, P.dark);
      if (m.detail >= 1) {
        m.bar(-0.52, 0.52, 0.76, 0.94, 2.14, 2.18, P.dark, 0.85);
        m.both((mm) => {
          mm.bar(0.54, 0.82, 0.76, 0.94, 2.14, 2.19, P.lamp);
          mm.bar(0.44, 0.82, 0.74, 0.98, -2.19, -2.14, P.lampRed);
          mm.bar(0.84, 0.98, 1.14, 1.28, 0.68, 0.80, P.dark);
          mm.bar(0.84, 0.88, 0.32, 0.62, 1.05, 1.60, P.dark, 0.85);
          mm.bar(0.84, 0.88, 0.32, 0.62, -1.64, -1.10, P.dark, 0.85);
        });
      }
    });
    return { livery: paint };
  }

  // ------------------------------------------------------- yard machinery
  function forklift(m, seed) {
    const body = askOne(seed, 19, [P.safety, P.livery[2], P.livery[4]]);
    m.part("running gear / wheels", () => {
      roadWheel(m, 0.50, 0.32, 0.18, 0.32, 0.20, false);
      roadWheel(m, -0.50, 0.32, 0.18, 0.32, 0.20, false);
      roadWheel(m, 0.38, 0.24, -1.18, 0.24, 0.16, false);
      roadWheel(m, -0.38, 0.24, -1.18, 0.24, 0.16, false);
    });
    m.part("body / chassis and counterweight", () => {
      m.bar(-0.56, 0.56, 0.20, 0.98, -1.48, 0.60, body);
      m.bar(-0.52, 0.52, 0.34, 1.12, -1.52, -0.86, P.dark, 0.9);
      m.bar(-0.58, 0.58, 0.14, 0.28, -1.20, 0.50, P.chassis, 0.85);
    });
    m.part("body / seat, controls and guard", () => {
      if (m.detail >= 1) {
        m.bar(-0.30, 0.30, 0.98, 1.36, -0.86, -0.48, P.dark, 0.95);
        m.bar(-0.30, 0.30, 0.90, 0.98, -0.90, -0.38, P.dark, 0.9);
        m.bar(-0.05, 0.05, 1.10, 1.42, -0.34, -0.26, P.dark);
      }
      m.both((mm) => {
        mm.bar(0.46, 0.54, 0.98, 2.14, -1.10, -1.02, P.steel);
        if (m.detail >= 1) mm.bar(0.46, 0.54, 0.98, 2.14, -0.16, -0.08, P.steel);
      });
      m.bar(-0.56, 0.56, 2.14, 2.22, -1.14, -0.04, P.steel, 1.05);
      if (m.detail >= 1) {
        for (let i = 0; i < 3; i += 1) {
          m.bar(-0.54, 0.54, 2.10, 2.14, -0.98 + i * 0.30, -0.92 + i * 0.30, P.steel, 0.9);
        }
      }
    });
    m.part("mast / rails, carriage and forks", () => {
      m.both((mm) => {
        mm.beam(0.30, 0.42, 0.16, 3.10, 0.62, 0.78, P.steel, 0.015);
        if (m.detail >= 1) mm.beam(0.24, 0.32, 0.30, 2.40, 0.66, 0.76, P.safety, 0.015);
      });
      m.bar(-0.44, 0.44, 0.16, 0.30, 0.60, 0.80, P.steel, 0.9);
      m.bar(-0.44, 0.44, 3.02, 3.10, 0.60, 0.80, P.steel, 1.05);
      m.bar(-0.46, 0.46, 0.20, 0.72, 0.78, 0.86, P.safety);
      m.both((mm) => {
        mm.bar(0.10, 0.24, 0.06, 0.16, 0.86, 1.94, P.steel);
        if (m.detail >= 1) mm.bar(0.10, 0.24, 0.16, 0.66, 0.84, 0.92, P.steel, 0.95);
      });
      if (m.detail >= 1) m.column(0, 0.30, 0.42, 1.60, 0.06, 0.06, 8, P.chrome, true);
    });
    return { livery: body };
  }

  function fuelBowser(m, seed) {
    const cab = askOne(seed, 23, P.livery);
    const halfW = 1.12;
    m.part("running gear / chassis", () => ladderChassis(m, -3.40, 3.40, 0.40, 0.92, 0.24));
    m.part("running gear / axles", () => {
      axleSet(m, 2.40, 0.46, halfW, 0.46, 0.28, false);
      axleSet(m, -1.95, 0.46, halfW, 0.46, 0.28, true);
    });
    m.part("cab / shell and glazing", () => roadCab(m, 1.06, 1.42, 3.36, 1.08, 1.80, cab, {}));
    m.part("body / barrel", () => tankBarrel(m, 1.92, -3.10, 1.28, 0.84, P.tankShell, { lids: 2 }));
    m.part("body / saddles, cabinet and ladder", () => {
      for (const sz of [-2.60, -0.70, 1.00]) m.bar(-0.88, 0.88, 1.04, 1.24, sz - 0.12, sz + 0.12, P.chassis, 0.92);
      m.bar(-1.02, 1.02, 1.00, 1.86, -3.36, -3.02, P.galv, 0.95);
      ladderRun(m, 0.60, -3.30, 1.00, 2.86, 0.40, P.galv);
      if (m.detail >= 1) {
        m.both((mm) => mm.bar(0.92, 1.14, 1.02, 1.42, -2.40, -1.10, P.dark, 0.9));
        m.bar(-0.30, 0.30, 2.76, 2.86, -3.00, 1.10, P.galv, 1.05);
      }
    });
    return { livery: cab };
  }

  // -------------------------------------------------------------- containers
  function containerPiece(len, salt) {
    return function (m, seed) {
      const col = askOne(seed, salt, P.box);
      isoContainer(m, 0, 0, 0, len, col, { named: true });
      m.part("ground / timber dunnage", () => {
        for (const dz of [-len / 2 + 0.6, len / 2 - 0.6]) {
          m.bar(-1.10, 1.10, 0, 0.001, dz - 0.10, dz + 0.10, P.timber, 0.7);
        }
      });
      return { livery: col };
    };
  }

  function containerStack(m, seed) {
    const cols = [-1.32, 1.32], tiers = 3, len = 6.06;
    for (let c = 0; c < cols.length; c += 1) {
      for (let t = 0; t < tiers; t += 1) {
        const col = askOne(seed, 40 + c * 8 + t, P.box);
        m.part(`stack / column ${c + 1} tier ${t + 1}`, () => {
          isoContainer(m, cols[c], t * 2.60, 0, len, col, { lite: true });
        });
      }
    }
    m.part("stack / hardstanding", () => {
      m.bar(-3.10, 3.10, 0, 0.02, -len / 2 - 0.6, len / 2 + 0.6, P.concrete, 0.94);
    });
    return {};
  }

  function palletStack(m, seed) {
    m.part("stack / tall stack", () => {
      const n = 6;
      for (let i = 0; i < n; i += 1) palletDeck(m, -0.95, i * 0.145, 0.10, 0, P.timber);
    });
    m.part("stack / short stack", () => {
      const n = 4;
      for (let i = 0; i < n; i += 1) palletDeck(m, 0.95, i * 0.145, -0.30, 0, P.timber);
    });
    m.part("stack / loose pallets", () => {
      palletDeck(m, 0.10, 0, 1.30, askOne(seed, 51, [0, 90]), P.board);
      palletDeck(m, -0.20, 0.145, 1.34, askOne(seed, 52, [0, 90]), P.timber);
    });
    return {};
  }

  function crateStack(m, seed) {
    const spots = [[-1.30, 0.10, 0], [0.10, 0.10, 0], [-0.60, 0.10, 1.10]];
    m.part("load / pallets", () => {
      for (let i = 0; i < spots.length; i += 1) palletDeck(m, spots[i][0], 0, spots[i][2], 0, P.timber);
    });
    m.part("load / timber crates", () => {
      for (let i = 0; i < spots.length; i += 1) {
        const h = askSpan(seed, 60 + i, 0.62, 1.05);
        const x = spots[i][0], z = spots[i][2], y0 = 0.145;
        m.bar(x - 0.55, x + 0.55, y0, y0 + h, z - 0.36, z + 0.36, P.crate);
        if (m.detail >= 1) {
          for (const bx of [x - 0.50, x + 0.50]) m.bar(bx - 0.045, bx + 0.045, y0, y0 + h, z - 0.38, z + 0.38, P.board, 1.05);
          m.bar(x - 0.55, x + 0.55, y0 + h * 0.52, y0 + h * 0.52 + 0.07, z - 0.38, z + 0.38, P.board, 1.05);
        }
      }
      // A second crate on the tallest pallet, because a yard stacks.
      m.bar(-1.72, -0.88, 0.90, 1.42, -0.28, 0.30, P.crate, 0.96);
    });
    m.part("load / wrapped load and banding", () => {
      m.bar(-0.55, 0.55, 0.145, 1.28, 1.85, 2.55, P.wrap, 0.98);
      palletDeck(m, 0, 0, 2.20, 0, P.timber);
      if (m.detail >= 1) {
        for (const by of [0.55, 1.05]) m.bar(-0.57, 0.57, by, by + 0.05, 1.83, 2.57, P.dark, 0.9);
      }
    });
    m.part("load / steel drums", () => {
      for (let i = 0; i < 3; i += 1) {
        const dx = 1.55 + (i % 2) * 0.66, dz = 1.70 + Math.floor(i / 2) * 0.68;
        m.column(dx, 0, dz, 0.88, 0.29, 0.29, m.segs(10, 5), askOne(seed, 70 + i, [P.livery[2], P.livery[3], P.steel]), true);
      }
    });
    return {};
  }

  // ------------------------------------------------------------------ cranes
  function verticalLattice(m, x, z, y0, y1, w, h, bays, col) {
    m.save().move(x, y0, z).rotX(-90);
    lattice(m, y1 - y0, w, h, bays, col);
    m.restore();
    return m;
  }

  function yardCrane(m, seed) {
    const legX = 5.60, legZ = 3.60, top = 10.60, wheelR = 0.55;
    m.part("portal / legs", () => {
      for (const sx of [-legX, legX]) {
        for (const sz of [-legZ, legZ]) verticalLattice(m, sx, sz, 1.10, top, 0.62, 0.62, 3, P.craneBody);
      }
    });
    m.part("portal / sill beams and wheels", () => {
      for (const sz of [-legZ, legZ]) {
        m.bar(-legX - 0.45, legX + 0.45, 0.92, 1.16, sz - 0.30, sz + 0.30, P.craneBody, 0.95);
        for (const sx of [-legX, legX]) {
          roadWheel(m, sx - 0.40, wheelR, sz, wheelR, 0.36, false);
          if (m.detail >= 1) roadWheel(m, sx + 0.40, wheelR, sz, wheelR, 0.36, false);
        }
      }
    });
    m.part("portal / head beams and runway girders", () => {
      for (const sz of [-legZ, legZ]) {
        m.beam(-legX - 0.35, legX + 0.35, top, top + 0.62, sz - 0.34, sz + 0.34, P.craneBody, 0.03);
      }
      if (m.detail < 1) return;
      for (const sx of [-legX, legX]) {
        m.save().move(sx, top + 0.62, -legZ - 0.9).rotY(0);
        lattice(m, 2 * legZ + 1.8, 0.52, 0.62, 4, P.craneBody);
        m.restore();
      }
    });
    m.part("hoist / trolley and spreader", () => {
      m.bar(-legX - 0.2, legX + 0.2, top + 0.94, top + 1.30, -0.90, 0.90, P.craneBody, 1.0);
      spreader(m, 0, 5.20, 0, 6.30, P.safety);
      // Four falls of rope, drawn as thin members. They are what makes a crane
      // read as a crane rather than as a gantry with a box under it — and they
      // are also four members 70 mm across, so the map mesh does without.
      if (m.detail < 1) return;
      for (const sx of [-1.16, 1.16]) {
        for (const sz of [-2.90, 2.90]) m.bar(sx - 0.035, sx + 0.035, 5.66, top + 0.94, sz - 0.035, sz + 0.035, P.dark);
      }
    });
    m.part("machinery / house and lighting", () => {
      machineHouse(m, 3.20, top + 1.30, 2.60, 3.00, 2.20, 2.40, P.craneBody);
      floodHead(m, -3.00, top + 1.10, legZ + 0.40, m.d(3, 2), 2.20, P.galv);
    });
    m.part("access / ladder and walkway", () => {
      ladderRun(m, legX - 0.90, legZ + 0.55, 1.20, top, 0.50, P.galv);
      if (m.detail >= 1) {
        m.bar(-legX, legX, top + 0.62, top + 0.68, legZ + 0.30, legZ + 0.90, P.galv, 1.04);
        for (let i = 0; i < 7; i += 1) {
          const hx = -legX + (i * 2 * legX) / 6;
          m.bar(hx - 0.03, hx + 0.03, top + 0.68, top + 1.68, legZ + 0.84, legZ + 0.90, P.galv);
        }
        m.bar(-legX, legX, top + 1.62, top + 1.68, legZ + 0.84, legZ + 0.90, P.galv, 1.06);
      }
    });
    return {};
  }

  function gantryCrane(m, seed) {
    const gauge = 8.20, portal = 19.0, boomZ = 22.0, backZ = -8.0;
    m.part("portal / legs", () => {
      for (const sx of [-gauge, gauge]) {
        for (const sz of [-3.20, 3.20]) verticalLattice(m, sx, sz, 1.30, portal, 0.86, 0.86, 4, P.craneBody);
      }
    });
    m.part("portal / sill beams, rails and bogies", () => {
      for (const sx of [-gauge, gauge]) {
        m.beam(sx - 0.70, sx + 0.70, 0.70, 1.34, -4.60, 4.60, P.craneBody, 0.03);
        if (m.detail < 1) continue;
        m.bar(sx - 0.09, sx + 0.09, 0, 0.16, -6.20, 6.20, P.railhead, 1.05);
        for (const sz of [-3.60, 3.60]) {
          m.bar(sx - 0.55, sx + 0.55, 0.16, 0.70, sz - 0.85, sz + 0.85, P.chassis, 0.9);
        }
      }
      m.beam(-gauge - 0.7, gauge + 0.7, portal - 0.9, portal, -0.42, 0.42, P.craneBody, 0.03);
    });
    m.part("boom / main girder and boom", () => {
      for (const sx of [-gauge, gauge]) {
        m.beam(sx - 0.55, sx + 0.55, portal, portal + 1.60, backZ, boomZ, P.craneBody, 0.04);
      }
      if (m.detail >= 1) {
        m.bar(-gauge, gauge, portal + 1.30, portal + 1.60, boomZ - 1.20, boomZ, P.craneBody, 1.02);
        m.bar(-gauge, gauge, portal + 1.30, portal + 1.60, backZ, backZ + 1.20, P.craneBody, 1.02);
        for (let i = 0; i < 6; i += 1) {
          const bz = backZ + 2.0 + (i * (boomZ - backZ - 4.0)) / 5;
          m.bar(-gauge, gauge, portal + 0.30, portal + 0.62, bz - 0.16, bz + 0.16, P.craneBody, 0.9);
        }
      }
    });
    m.part("boom / A-frame and backstays", () => {
      for (const sx of [-gauge, gauge]) {
        m.beam(sx - 0.30, sx + 0.30, portal + 1.60, portal + 7.40, -0.90, 1.60, P.craneBody, 0.03);
        if (m.detail < 1) continue;
        // Three ties: forward to the boom tip, back over the machinery house,
        // and along the boom. Members 280 mm across at twenty-odd metres, so
        // the map mesh keeps the A-frame and drops the rigging.
        m.bar(sx - 0.14, sx + 0.14, portal + 7.10, portal + 7.40, 1.40, 9.0, P.craneBody, 1.0);
        m.bar(sx - 0.14, sx + 0.14, portal + 4.30, portal + 7.40, -6.6, -0.60, P.craneBody, 1.0);
        m.bar(sx - 0.14, sx + 0.14, portal + 4.30, portal + 4.60, 9.0, 17.0, P.craneBody, 1.0);
      }
      if (m.detail >= 1) m.bar(-gauge, gauge, portal + 7.10, portal + 7.40, -0.40, 0.90, P.craneBody, 1.05);
    });
    m.part("hoist / trolley and spreader", () => {
      m.bar(-gauge + 0.2, gauge - 0.2, portal + 1.60, portal + 2.30, 10.4, 12.6, P.craneBody, 1.0);
      spreader(m, 0, 8.60, 11.50, 12.40, P.safety);
      if (m.detail < 1) return;
      for (const sx of [-1.16, 1.16]) {
        for (const sz of [11.50 - 5.8, 11.50 + 5.8]) {
          m.bar(sx - 0.045, sx + 0.045, 9.06, portal + 1.60, sz - 0.045, sz + 0.045, P.dark);
        }
      }
    });
    m.part("machinery / house and lighting", () => {
      machineHouse(m, 0, portal + 1.60, -4.60, 6.20, 3.20, 5.00, P.craneBody);
      floodHead(m, 0, portal + 1.20, boomZ - 0.4, 4, 4.40, P.galv);
      if (m.detail >= 1) floodHead(m, -gauge + 1.2, portal + 1.20, 4.0, 2, 1.60, P.galv);
    });
    m.part("access / stairs and walkways", () => {
      ladderRun(m, gauge - 1.30, 3.90, 1.34, portal, 0.60, P.galv);
      if (m.detail >= 1) {
        for (const sx of [-gauge, gauge]) {
          m.bar(sx - 1.10, sx - 0.55, portal + 1.60, portal + 1.66, backZ + 1.2, boomZ - 1.2, P.galv, 1.04);
          for (let i = 0; i < 8; i += 1) {
            const hz = backZ + 2.0 + (i * (boomZ - backZ - 4.0)) / 7;
            m.bar(sx - 1.12, sx - 1.06, portal + 1.66, portal + 2.66, hz - 0.03, hz + 0.03, P.galv);
          }
          m.bar(sx - 1.12, sx - 1.06, portal + 2.60, portal + 2.66, backZ + 1.2, boomZ - 1.2, P.galv, 1.06);
        }
      }
    });
    return {};
  }

  // -------------------------------------------------------------------- rail
  function locomotive(m, seed) {
    const body = askOne(seed, 31, [P.livery[3], P.livery[1], P.livery[2], P.steel]);
    const z0 = -8.40, z1 = 8.40, halfW = 1.42, deck = 1.16;
    m.part("running gear / bogies", () => { railBogie(m, -5.20, P.chassis); railBogie(m, 5.20, P.chassis); });
    m.part("running gear / underframe and couplers", () => railUnderframe(m, z0, z1, halfW, deck));
    m.part("body / long hood", () => {
      m.bar(-1.22, 1.22, deck + 0.05, deck + 2.55, z0 + 0.9, 2.10, body);
      if (m.detail >= 1) m.bar(-1.26, 1.26, deck + 2.55, deck + 2.68, z0 + 0.7, 2.20, P.panel, 1.06);
      if (m.detail >= 1) {
        m.both((mm) => {
          for (let i = 0; i < 5; i += 1) {
            const lz = z0 + 1.6 + i * 1.35;
            mm.bar(1.22, 1.26, deck + 1.10, deck + 2.00, lz - 0.36, lz + 0.36, P.dark, 0.86);
          }
          mm.bar(1.22, 1.27, deck + 0.20, deck + 0.90, z0 + 1.2, 1.90, body, 0.94);
        });
      }
    });
    m.part("body / cab", () => {
      m.bar(-1.34, 1.34, deck + 0.05, deck + 2.95, 2.10, 5.00, body);
      if (m.detail >= 1) m.bar(-1.38, 1.38, deck + 2.95, deck + 3.06, 2.00, 5.10, P.panel, 1.06);
      m.bar(-1.16, 1.16, deck + 1.65, deck + 2.55, 5.00, 5.04, P.glass, 1.0);
      if (m.detail >= 1) m.both((mm) => {
        mm.bar(1.34, 1.38, deck + 1.65, deck + 2.55, 2.40, 3.30, P.glass, 0.94);
        mm.bar(1.34, 1.38, deck + 1.65, deck + 2.55, 3.60, 4.60, P.glass, 0.94);
        if (m.detail >= 1) mm.bar(1.34, 1.40, deck + 0.15, deck + 2.60, 3.34, 3.42, P.dark, 0.8);
      });
    });
    m.part("body / short hood, radiator and fuel tank", () => {
      m.bar(-1.22, 1.22, deck + 0.05, deck + 2.05, 5.00, z1 - 0.9, body);
      if (m.detail >= 1) {
        // Roof cap, radiator grille and the fuel tank slung between the bogies.
        m.bar(-1.26, 1.26, deck + 2.05, deck + 2.16, 4.90, z1 - 0.8, P.panel, 1.06);
        m.bar(-1.18, 1.18, deck + 0.30, deck + 1.70, z1 - 0.94, z1 - 0.86, P.dark, 0.88);
        m.bar(-1.02, 1.02, 0.36, deck - 0.34, -2.60, 1.40, P.chassis, 0.9);
        m.both((mm) => {
          mm.column(1.06, 0.62, -3.20, 1.10, 0.26, 0.26, 8, P.steel, true);
          mm.bar(0.86, 1.10, deck + 2.68, deck + 2.86, z0 + 1.4, z0 + 1.8, P.dark);
        });
        m.column(0, deck + 2.68, -1.20, 0.42, 0.18, 0.14, 8, P.dark, true);
      }
    });
    m.part("body / walkway handrails and lamps", () => {
      if (m.detail < 1) return;
      m.both((mm) => {
        for (let i = 0; i < 6; i += 1) {
          const sz = z0 + 1.2 + i * 2.6;
          mm.bar(1.30, 1.36, deck + 0.05, deck + 1.05, sz - 0.03, sz + 0.03, P.galv);
        }
        mm.bar(1.30, 1.36, deck + 0.99, deck + 1.05, z0 + 1.2, z1 - 1.2, P.galv, 1.06);
      });
      m.bar(-0.34, 0.34, deck + 2.20, deck + 2.50, 5.02, 5.08, P.lamp);
      m.bar(-0.34, 0.34, deck + 1.30, deck + 1.62, z1 - 0.96, z1 - 0.90, P.lamp);
    });
    return { livery: body };
  }

  function wagonContainer(m, seed) {
    const z0 = -9.60, z1 = 9.60, halfW = 1.32, deck = 1.10;
    m.part("running gear / bogies", () => { railBogie(m, -6.60, P.chassis); railBogie(m, 6.60, P.chassis); });
    m.part("running gear / underframe and couplers", () => railUnderframe(m, z0, z1, halfW, deck));
    m.part("deck / spine and cross bearers", () => {
      m.bar(-0.44, 0.44, deck + 0.05, deck + 0.30, z0 + 0.2, z1 - 0.2, P.chassis, 0.98);
      const n = m.d(7, 2);
      for (let i = 0; i < n; i += 1) {
        const cz = z0 + 1.0 + (i * (z1 - z0 - 2.0)) / (n - 1);
        m.bar(-halfW + 0.06, halfW - 0.06, deck + 0.05, deck + 0.22, cz - 0.10, cz + 0.10, P.chassis, 0.9);
      }
    });
    m.part("deck / twistlocks and brake gear", () => {
      if (m.detail < 1) return;
      for (const sx of [-1.18, 1.18]) {
        for (const sz of [-9.10, -3.05, 3.05, 9.10]) {
          m.bar(sx - 0.12, sx + 0.12, deck + 0.05, deck + 0.24, sz - 0.12, sz + 0.12, P.safety, 1.05);
        }
      }
      if (m.detail >= 1) {
        m.bar(-0.60, 0.60, 0.52, deck - 0.36, -1.30, 0.60, P.chassis, 0.85);
        m.column(0.80, 0.60, -0.20, 0.70, 0.22, 0.22, 8, P.steel, true);
      }
    });
    return {};
  }

  function wagonTank(m, seed) {
    const z0 = -6.80, z1 = 6.80, halfW = 1.26, deck = 1.08;
    m.part("running gear / bogies", () => { railBogie(m, -4.30, P.chassis); railBogie(m, 4.30, P.chassis); });
    m.part("running gear / underframe and couplers", () => railUnderframe(m, z0, z1, halfW, deck));
    m.part("body / barrel", () => tankBarrel(m, 2.28, z0 + 0.90, z1 - 0.90, 1.10, P.tankShell, { lids: 1 }));
    m.part("body / saddles and anchors", () => {
      for (const sz of [-3.40, 3.40]) m.bar(-1.10, 1.10, deck + 0.05, 1.62, sz - 0.24, sz + 0.24, P.chassis, 0.92);
      if (m.detail >= 1) m.bar(-0.36, 0.36, deck + 0.05, 1.30, z0 + 1.0, z1 - 1.0, P.chassis, 0.86);
    });
    m.part("body / ladder, platform and valve", () => {
      if (m.detail < 1) return;
      ladderRun(m, 0.96, z0 + 1.10, deck + 0.05, 3.44, 0.44, P.galv);
      m.bar(0.30, 1.06, 3.38, 3.46, z0 + 0.90, z0 + 1.40, P.galv, 1.04);
      m.column(0, 1.02, 0, 0.28, 0.16, 0.16, 8, P.steel, true);
      m.both((mm) => {
        mm.bar(0.32, 0.38, 3.44, 4.24, z0 + 0.92, z0 + 0.98, P.galv);
        mm.bar(0.32, 0.38, 4.18, 4.24, z0 + 0.92, z0 + 1.42, P.galv, 1.06);
      });
    });
    return {};
  }

  function wagonHopper(m, seed) {
    const z0 = -6.40, z1 = 6.40, halfW = 1.30, deck = 1.06;
    m.part("running gear / bogies", () => { railBogie(m, -4.10, P.chassis); railBogie(m, 4.10, P.chassis); });
    m.part("running gear / underframe and couplers", () => railUnderframe(m, z0, z1, halfW, deck));
    m.part("body / hopper", () => {
      const top = 3.42, ya = deck + 0.62;
      m.bar(-1.28, 1.28, ya, top, z0 + 0.80, z1 - 0.80, P.steel, 0.98);
      // Two discharge cones, drawn as sloped end walls under the body: this is
      // the shape that says "hopper" and not "open box".
      for (const s of [-1, 1]) {
        const cz = s * 2.60;
        m.quad([-1.28, ya, cz + s * 1.90], [1.28, ya, cz + s * 1.90],
          [0.44, deck - 0.30, cz + s * 0.38], [-0.44, deck - 0.30, cz + s * 0.38], P.steel, 0.86);
        m.bar(-0.46, 0.46, deck - 0.44, deck - 0.28, cz - 0.42, cz + 0.42, P.dark, 0.9);
      }
      if (m.detail >= 1) m.bar(-1.32, 1.32, top, top + 0.10, z0 + 0.72, z1 - 0.72, P.steel, 1.06);
    });
    m.part("body / ribs and hatches", () => {
      if (m.detail < 1) return;
      m.both((mm) => {
        for (let i = 0; i < 5; i += 1) {
          const rz = z0 + 1.4 + i * 2.4;
          mm.bar(1.28, 1.33, deck + 0.66, 3.40, rz - 0.08, rz + 0.08, P.steel, 0.92);
        }
      });
      for (let i = 0; i < m.d(4, 2); i += 1) {
        const hz = z0 + 1.6 + (i * (z1 - z0 - 3.2)) / (m.d(4, 2) - 1);
        m.bar(-0.78, 0.78, 3.42, 3.56, hz - 0.42, hz + 0.42, P.dark, 0.95);
      }
    });
    return {};
  }

  // ------------------------------------------------------------ static plant
  function storageTank(m, seed) {
    const r = 4.60, h = 8.00;
    m.part("vessel / shell and roof", () => verticalVessel(m, 0, 0, 0.40, h, r, P.tankShell, { roof: 1.30 }));
    m.part("vessel / nozzles and manway", () => {
      m.drum(0, 1.30, r - 0.05, 0.70, 0.30, 0.30, m.segs(8, 4), P.steel, true);
      m.column(r - 0.9, 0.40 + h + 0.4, 0.6, 0.60, 0.22, 0.22, m.segs(8, 4), P.steel, true);
      m.bar(-0.55, 0.55, 1.90, 2.60, r - 0.06, r + 0.08, P.steel, 1.02);
    });
    m.part("access / stair and roof rail", () => {
      ladderRun(m, 0, r + 0.30, 0.40, 0.40 + h + 0.10, 0.62, P.galv);
      if (m.detail >= 1) {
        const seg = 12;
        for (let i = 0; i < seg; i += 1) {
          const a = (i / seg) * Math.PI * 2;
          m.bar(Math.cos(a) * (r - 0.20) - 0.035, Math.cos(a) * (r - 0.20) + 0.035,
            0.40 + h, 0.40 + h + 1.05, Math.sin(a) * (r - 0.20) - 0.035, Math.sin(a) * (r - 0.20) + 0.035, P.galv);
        }
        m.column(0, 0.40 + h + 1.00, 0, 0.06, r - 0.16, r - 0.16, seg, P.galv, false);
      }
    });
    m.part("containment / bund kerb", () => {
      m.column(0, 0, 0, 0.55, r + 3.10, r + 3.10, m.segs(16, 6), P.concrete, false);
      m.column(0, 0.52, 0, 0.05, r + 3.16, r + 3.16, m.segs(16, 6), P.concrete, true);
    });
    return {};
  }

  function siloGroup(m, seed) {
    const r = 1.72, spacing = 3.80;
    m.part("silos / vessels", () => {
      for (let i = 0; i < 3; i += 1) {
        verticalVessel(m, (i - 1) * spacing, 0, 3.30, 7.00, r, P.siloShell, { hopper: 1.70, roof: 0.60 });
      }
    });
    m.part("silos / support ring beams", () => {
      m.bar(-spacing - r, spacing + r, 3.10, 3.34, -0.10, 0.10, P.steel, 0.96);
      if (m.detail < 1) return;
      for (let i = 0; i < 3; i += 1) {
        m.bar((i - 1) * spacing - r, (i - 1) * spacing + r, 3.10, 3.34, -r, r, P.steel, 0.9);
      }
    });
    m.part("plant / elevator leg and head", () => {
      m.beam(spacing + r + 0.10, spacing + r + 0.90, 0, 12.20, -0.42, 0.38, P.galv, 0.03);
      if (m.detail < 1) return;
      m.bar(spacing + r - 0.10, spacing + r + 1.30, 12.20, 13.30, -0.62, 0.58, P.galv, 1.02);
      m.bar(-spacing, spacing + r + 0.30, 10.60, 11.10, -0.30, 0.30, P.galv, 0.98);
    });
    m.part("access / ladder and roof walkway", () => {
      ladderRun(m, spacing + r + 1.40, 0, 0, 10.60, 0.52, P.galv);
      if (m.detail >= 1) {
        m.bar(-spacing - 0.6, spacing + 0.6, 10.60, 10.66, -0.34, 0.34, P.galv, 1.04);
        for (let i = 0; i < 7; i += 1) {
          const hx = -spacing - 0.4 + (i * (2 * spacing + 0.8)) / 6;
          m.bar(hx - 0.03, hx + 0.03, 10.66, 11.66, 0.28, 0.34, P.galv);
        }
        m.bar(-spacing - 0.6, spacing + 0.6, 11.60, 11.66, 0.28, 0.34, P.galv, 1.06);
      }
    });
    return {};
  }

  function conveyor(m, seed) {
    const z0 = -10.0, z1 = 10.0, y0 = 1.10, rise = 4.20;
    const ang = Math.atan2(rise, z1 - z0) / DEG;
    const runLen = Math.hypot(z1 - z0, rise);
    m.part("support / trestles", () => {
      const n = m.d(5, 3);
      for (let i = 0; i < n; i += 1) {
        const tz = z0 + 1.2 + (i * (z1 - z0 - 2.4)) / (n - 1);
        const ty = y0 + ((tz - z0) / (z1 - z0)) * rise;
        m.both((mm) => {
          mm.beam(0.62, 0.78, 0, ty, tz - 0.09, tz + 0.09, P.steel, 0.02);
          if (m.detail >= 1) mm.bar(0.16, 0.66, ty * 0.42, ty * 0.52, tz - 0.06, tz + 0.06, P.steel, 0.9);
        });
        m.bar(-0.78, 0.78, ty - 0.16, ty, tz - 0.09, tz + 0.09, P.steel, 0.95);
      }
    });
    m.part("gantry / stringers and belt", () => {
      m.save().move(0, y0, z0).rotX(-ang);
      m.both((mm) => mm.beam(0.58, 0.72, 0, 0.36, 0, runLen, P.steel, 0.02));
      m.bar(-0.56, 0.56, 0.30, 0.36, 0.10, runLen - 0.10, P.belt, 1.02);
      if (m.detail >= 1) m.bar(-0.50, 0.50, -0.02, 0.02, 0.30, runLen - 0.30, P.belt, 0.8);
      if (m.detail >= 1) {
        const idlers = 9;
        for (let i = 0; i < idlers; i += 1) {
          const iz = 0.6 + (i * (runLen - 1.2)) / (idlers - 1);
          m.roll(-0.54, 0.24, iz, 1.08, 0.08, 0.08, 6, P.steel, false);
        }
        m.bar(-0.62, -0.56, 0.36, 1.32, 0.4, runLen - 0.4, P.galv, 1.0);
        for (let i = 0; i < 6; i += 1) {
          const hz = 0.6 + (i * (runLen - 1.2)) / 5;
          m.bar(-0.62, -0.56, 0.36, 1.32, hz - 0.03, hz + 0.03, P.galv);
        }
      }
      m.restore();
    });
    m.part("plant / head and tail pulleys, drive and chute", () => {
      m.roll(-0.62, y0 + rise + 0.10, z1 - 0.30, 1.24, 0.36, 0.36, m.segs(10, 5), P.steel, true);
      m.roll(-0.62, y0 + 0.10, z0 + 0.30, 1.24, 0.30, 0.30, m.segs(10, 5), P.steel, true);
      m.bar(0.66, 1.32, y0 + rise - 0.30, y0 + rise + 0.42, z1 - 0.90, z1 - 0.10, P.livery[4], 0.98);
      if (m.detail >= 1) m.bar(-0.62, 0.62, y0 + rise - 1.40, y0 + rise - 0.05, z1 - 0.70, z1 + 0.10, P.galv, 0.92);
    });
    // Somebody has to be able to reach the head pulley, and a six-metre head
    // end with no way up it reads as a diagram of a conveyor rather than one.
    m.part("access / head ladder", () => ladderRun(m, -0.90, z1 - 1.40, 0, y0 + rise + 0.30, 0.46, P.galv));
    return {};
  }

  function lightMast(m, seed) {
    const h1 = 6.60, h2 = 12.20;
    m.part("mast / column", () => {
      m.column(0, 0.34, 0, h1 - 0.34, 0.20, 0.155, m.segs(10, 4), P.galv, false);
      m.column(0, h1, 0, h2 - h1, 0.155, 0.105, m.segs(10, 4), P.galv, true);
    });
    m.part("mast / base plate and cabinet", () => {
      m.bar(-0.42, 0.42, 0.20, 0.34, -0.42, 0.42, P.steel, 1.02);
      m.bar(-0.50, 0.50, 0, 0.20, -0.50, 0.50, P.concrete, 0.96);
      m.bar(-0.28, 0.28, 0.34, 1.28, 0.20, 0.44, P.galv, 0.94);
      if (m.detail >= 1) {
        for (const bx of [-0.34, 0.34]) {
          for (const bz of [-0.34, 0.34]) m.column(bx, 0.20, bz, 0.16, 0.035, 0.035, 6, P.dark, true);
        }
      }
    });
    m.part("head / frame and floodlights", () => {
      m.bar(-1.55, 1.55, h2, h2 + 0.12, -0.09, 0.09, P.galv, 1.05);
      m.bar(-0.09, 0.09, h2 - 0.10, h2 + 0.12, -0.90, 0.90, P.galv, 1.0);
      floodHead(m, 0, h2, 0.80, m.d(3, 2), 2.90, P.galv);
      floodHead(m, 0, h2, -0.80, m.d(3, 2), 2.90, P.galv);
    });
    m.part("access / ladder", () => ladderRun(m, 0.30, -0.001, 1.30, h2 - 0.60, 0.42, P.galv));
    return {};
  }

  // ------------------------------------------------------------------- table
  const PIECES = {
    tractor_unit: {
      name: "Articulated tractor unit", kind: "road", build: tractorUnit,
      blurb: "a three-axle sleeper-cab tractor with a fifth-wheel coupler; it is the front half of an articulated lorry and carries nothing by itself",
    },
    trailer_box: {
      name: "Box semi-trailer", kind: "trailer", build: trailerBox,
      blurb: "a closed 13.6 m box trailer on a tri-axle bogie, with landing legs, rear doors and an underrun bar",
    },
    trailer_flatbed: {
      name: "Flatbed semi-trailer", kind: "trailer", build: trailerFlatbed,
      blurb: "the same chassis with a timber deck, a headboard, side stanchions and lashing rings, carrying no load",
    },
    trailer_tanker: {
      name: "Tanker semi-trailer", kind: "trailer", build: trailerTanker,
      blurb: "the same chassis under a cylindrical barrel with a crown walkway and a rear valve cabinet",
    },
    rigid_box_van: {
      name: "Rigid box lorry", kind: "road", build: rigidBoxVan,
      blurb: "a two-axle rigid with a box body and a tail lift, the medium-weight delivery vehicle of the period",
    },
    service_van: {
      name: "Panel van", kind: "road", build: serviceVan,
      blurb: "a one-box light commercial van with a short bonnet and split rear doors",
    },
    bus: {
      name: "Single-deck bus", kind: "road", build: bus,
      blurb: "an eleven-metre two-door city bus with a blank destination panel",
    },
    car: {
      name: "Saloon car", kind: "road", build: car,
      blurb: "a generic three-box family saloon, the shape of an ordinary 1990 car and not any particular one",
    },
    forklift: {
      name: "Counterbalance forklift", kind: "handling", build: forklift,
      blurb: "a yard forklift with a two-stage mast, forks, overhead guard and counterweight",
    },
    fuel_bowser: {
      name: "Fuel bowser", kind: "road", build: fuelBowser,
      blurb: "a rigid two-axle tanker with a small barrel and a rear pump cabinet; it holds no fuel the simulation knows about",
    },
    container_20ft: {
      name: "20 ft ISO container", kind: "container", build: containerPiece(6.06, 41),
      blurb: "a corrugated steel dry-freight box with corner castings, door leaves and locking bars, standing on timber dunnage",
    },
    container_40ft: {
      name: "40 ft ISO container", kind: "container", build: containerPiece(12.19, 43),
      blurb: "the same box at twice the length, which is the point of a shared builder",
    },
    container_stack: {
      name: "Container stack", kind: "container", build: containerStack,
      blurb: "six boxes stacked two wide and three high on a concrete hardstanding",
    },
    pallet_stack: {
      name: "Pallet stacks", kind: "handling", build: palletStack,
      blurb: "two stacks of empty timber pallets and a pair laid loose beside them",
    },
    crate_stack: {
      name: "Crate and drum stacks", kind: "handling", build: crateStack,
      blurb: "palletised timber crates, a banded wrapped load and three steel drums",
    },
    yard_crane: {
      name: "Yard gantry crane", kind: "crane", build: yardCrane,
      blurb: "a rubber-tyred container gantry with a trolley, a spreader and a machinery house; it moves nothing in the simulation",
    },
    gantry_crane: {
      name: "Quayside gantry crane", kind: "crane", build: gantryCrane,
      blurb: "a rail-mounted ship-to-shore crane with a boom, an A-frame and backstays; representative scenery, not a port capacity",
    },
    locomotive: {
      name: "Diesel locomotive", kind: "rail", build: locomotive,
      blurb: "a generic hood-unit diesel on two bogies, with a centre cab, walkways and buffers",
    },
    wagon_container: {
      name: "Container flat wagon", kind: "rail", build: wagonContainer,
      blurb: "a bogie flat with a spine, cross bearers and twistlocks, carrying no container",
    },
    wagon_tank: {
      name: "Tank wagon", kind: "rail", build: wagonTank,
      blurb: "a bogie underframe under the same barrel the road tanker uses, with a crown platform and a ladder",
    },
    wagon_hopper: {
      name: "Hopper wagon", kind: "rail", build: wagonHopper,
      blurb: "a bogie hopper with sloped discharge cones and top hatches, carrying nothing",
    },
    storage_tank: {
      name: "Bulk storage tank", kind: "static", build: storageTank,
      blurb: "a vertical shell tank with a cone roof, nozzles, an access stair and a bund kerb; it stores nothing the simulation records",
    },
    silo_group: {
      name: "Silo battery", kind: "static", build: siloGroup,
      blurb: "three hopper-bottomed silos on legs with an elevator leg and a roof walkway",
    },
    conveyor: {
      name: "Conveyor run", kind: "static", build: conveyor,
      blurb: "a twenty-metre inclined belt on trestles, with head and tail pulleys, a drive and a discharge chute",
    },
    light_mast: {
      name: "Yard light mast", kind: "static", build: lightMast,
      blurb: "a twelve-metre tapered mast with two floodlight banks, a base cabinet and a climbing ladder",
    },
  };
  const PIECE_KEYS = Object.keys(PIECES);
  const FALLBACK = "container_20ft";
  function has(o, k) { return Object.prototype.hasOwnProperty.call(o, k); }

  // ------------------------------------------------------------------ finish
  Mesh.prototype.finish = function (info) {
    const verts = this.pos.length / 3, tris = verts / 3;
    const positions = new Float32Array(this.pos);
    const normals = new Float32Array(verts * 3);
    const colors = new Float32Array(verts * 3);
    // Face normals first, from the DOUBLE positions and the vertices as
    // transformed, so a mirrored half gets the normal its geometry has.
    const face = new Float64Array(tris * 3);
    for (let t = 0; t < tris; t += 1) {
      const i = t * 9;
      const ax = this.pos[i], ay = this.pos[i + 1], az = this.pos[i + 2];
      const ux = this.pos[i + 3] - ax, uy = this.pos[i + 4] - ay, uz = this.pos[i + 5] - az;
      const vx = this.pos[i + 6] - ax, vy = this.pos[i + 7] - ay, vz = this.pos[i + 8] - az;
      let nx = uy * vz - uz * vy, ny = uz * vx - ux * vz, nz = ux * vy - uy * vx;
      const len = Math.hypot(nx, ny, nz);
      if (len > 1e-12) { nx /= len; ny /= len; nz /= len; } else { nx = 0; ny = 1; nz = 0; }
      face[t * 3] = nx; face[t * 3 + 1] = ny; face[t * 3 + 2] = nz;
    }
    // Which faces meet at each corner, per smoothing group. Quantised to a
    // tenth of a millimetre: two vertices authored at the same place agree to
    // far better than that and two authored a millimetre apart are a modelling
    // decision that should not be welded. Group 0 never enters the map, which
    // is what makes "flat unless asked" the default.
    const buckets = new Map();
    const keyAt = (i, g) => ((Math.round(this.pos[i] * 8192) | 0) + "," + (Math.round(this.pos[i + 1] * 8192) | 0)
      + "," + (Math.round(this.pos[i + 2] * 8192) | 0) + "," + g);
    for (let t = 0; t < tris; t += 1) {
      const g = this.grp[t];
      if (!g) continue;
      for (let c = 0; c < 3; c += 1) {
        const k = keyAt(t * 9 + c * 3, g);
        const list = buckets.get(k);
        if (list) list.push(t); else buckets.set(k, [t]);
      }
    }
    // Resolve the normal, then the colour FROM that normal. Iteration is over
    // triangles in emit order and the Map is only ever looked up by key, never
    // walked, so nothing here depends on insertion order.
    for (let t = 0; t < tris; t += 1) {
      const g = this.grp[t];
      const fx = face[t * 3], fy = face[t * 3 + 1], fz = face[t * 3 + 2];
      for (let c = 0; c < 3; c += 1) {
        const v = t * 3 + c, i = v * 3;
        let nx = fx, ny = fy, nz = fz;
        if (g) {
          const list = buckets.get(keyAt(i, g));
          let sx = 0, sy = 0, sz = 0;
          for (let q = 0; q < list.length; q += 1) {
            const o = list[q] * 3;
            // Crease limit measured against THIS face, not against the running
            // sum: that is what stops a cap fan with many triangles at one
            // corner from dragging the wall normals round with it.
            if (face[o] * fx + face[o + 1] * fy + face[o + 2] * fz >= CREASE) {
              sx += face[o]; sy += face[o + 1]; sz += face[o + 2];
            }
          }
          const len = Math.hypot(sx, sy, sz);
          if (len > 1e-9) { nx = sx / len; ny = sy / len; nz = sz / len; }
        }
        normals[i] = nx; normals[i + 1] = ny; normals[i + 2] = nz;
        const lambert = nx * SUN[0] + ny * SUN[1] + nz * SUN[2];
        const k = this.mat[v] * (AMBIENT + DIFFUSE * (lambert > 0 ? lambert : 0));
        for (let ch = 0; ch < 3; ch += 1) {
          const value = this.base[i + ch] * k;
          colors[i + ch] = value > 1 ? 1 : value < 0 ? 0 : value;
        }
      }
    }
    // Seat the lowest vertex on Y=0. Authoring puts wheels, feet and slabs
    // there already, but a prop that dips a millimetre below grade would sink
    // into terrain, and a renderer placing this has exactly one contract.
    let drop = Infinity;
    for (let i = 1; i < positions.length; i += 3) if (positions[i] < drop) drop = positions[i];
    if (drop !== 0 && isFinite(drop)) {
      for (let i = 1; i < positions.length; i += 3) positions[i] -= drop;
    }
    const min = [Infinity, Infinity, Infinity], max = [-Infinity, -Infinity, -Infinity];
    for (let i = 0; i < positions.length; i += 3) {
      for (let k = 0; k < 3; k += 1) {
        if (positions[i + k] < min[k]) min[k] = positions[i + k];
        if (positions[i + k] > max[k]) max[k] = positions[i + k];
      }
    }
    const out = {
      positions, normals, colors,
      bounds: { min, max },
      size: [max[0] - min[0], max[1] - min[1], max[2] - min[2]],
      parts: this.parts,
      triangleCount: tris,
    };
    for (const key of Object.keys(info)) out[key] = info[key];
    return out;
  };

  // --------------------------------------------------------------------- api
  function normLod(v) {
    return v === 1 || v === 2 || v === "far" || v === "map" || v === "lod1" || v === "lod2" ? 1 : 0;
  }
  function describe(p, extra) {
    return `${p.name}: ${p.blurb}. Representative scenery on the 1990 baseline — an original generic `
      + `design, not a licensed, named or measured vehicle, and it grants the simulation no cargo, `
      + `capacity or capability${extra || ""}.`;
  }

  /// One prop. `lod` picks the near (0) or map (1) mesh, `variant` is any
  /// integer and is the only source of variety, and `coupled` raises a
  /// trailer's landing legs. Nothing else reaches the geometry, which is why
  /// the same three arguments give the same bytes on every machine.
  function build(pieceKey, opts) {
    const o = opts && typeof opts === "object" ? opts : {};
    const key = typeof pieceKey === "string" && has(PIECES, pieceKey) ? pieceKey : FALLBACK;
    const p = PIECES[key];
    const lod = normLod(o.lod);
    const variant = typeof o.variant === "number" && isFinite(o.variant) ? Math.abs(Math.round(o.variant)) % 1000 : 0;
    const coupled = !!o.coupled;
    const seed = mix32(seedOf(key) ^ mix32(variant + 1));
    const m = new Mesh(lod ? 0 : 1);
    const extra = p.build(m, seed, { coupled }) || {};
    return m.finish({
      piece: key,
      requestedPiece: pieceKey,
      name: p.name,
      kind: p.kind,
      lod,
      variant,
      coupled,
      sockets: extra.sockets || {},
      description: describe(p, coupled && extra.sockets && extra.sockets.kingpin ? "; drawn coupled, with its landing legs raised" : ""),
    });
  }

  /// Concatenate meshes into one buffer under a cardinal yaw and a ground
  /// translation. Cardinal ONLY: cos/sin at 0/90/180/270 taken from a table are
  /// exactly 0 and +/-1, so a placed prop keeps exactly unit normals and two
  /// yards built from the same props are bitwise identical rather than nearly.
  const YAW4 = [[1, 0], [0, 1], [-1, 0], [0, -1]];
  /// `label` null means "carry the source mesh's own part ranges across,
  /// prefixed", which is what a coupled vehicle wants: a click on the trailer's
  /// bogie should still say bogie. A string means "one part for the whole
  /// placed prop", which is what a forty-prop yard wants.
  function placeInto(acc, mesh, yawIdx, tx, ty, tz, label, prefix) {
    const r = YAW4[((yawIdx | 0) % 4 + 4) % 4], c = r[0], s = r[1];
    const first = acc.pos.length / 3;
    for (let i = 0; i < mesh.positions.length; i += 3) {
      const x = mesh.positions[i], y = mesh.positions[i + 1], z = mesh.positions[i + 2];
      acc.pos.push(c * x + s * z + tx, y + ty, -s * x + c * z + tz);
      const nx = mesh.normals[i], ny = mesh.normals[i + 1], nz = mesh.normals[i + 2];
      acc.nrm.push(c * nx + s * nz, ny, -s * nx + c * nz);
      acc.col.push(mesh.colors[i], mesh.colors[i + 1], mesh.colors[i + 2]);
    }
    const count = mesh.positions.length / 3;
    if (label == null) {
      for (const part of mesh.parts) {
        acc.parts.push({
          name: `${prefix} / ${part.name}`, first: first + part.first, count: part.count,
          group: prefix, label: part.name,
        });
      }
    } else {
      const cut = label.indexOf(" / ");
      acc.parts.push({
        name: label, first, count,
        group: cut < 0 ? label : label.slice(0, cut),
        label: cut < 0 ? label : label.slice(cut + 3),
      });
    }
    return count / 3;
  }
  function assemble(acc, info) {
    const positions = new Float32Array(acc.pos);
    const normals = new Float32Array(acc.nrm);
    const colors = new Float32Array(acc.col);
    let drop = Infinity;
    for (let i = 1; i < positions.length; i += 3) if (positions[i] < drop) drop = positions[i];
    if (drop !== 0 && isFinite(drop)) for (let i = 1; i < positions.length; i += 3) positions[i] -= drop;
    const min = [Infinity, Infinity, Infinity], max = [-Infinity, -Infinity, -Infinity];
    for (let i = 0; i < positions.length; i += 3) {
      for (let k = 0; k < 3; k += 1) {
        if (positions[i + k] < min[k]) min[k] = positions[i + k];
        if (positions[i + k] > max[k]) max[k] = positions[i + k];
      }
    }
    const out = {
      positions, normals, colors,
      bounds: { min, max },
      size: [max[0] - min[0], max[1] - min[1], max[2] - min[2]],
      parts: acc.parts,
      triangleCount: positions.length / 9,
    };
    for (const key of Object.keys(info)) out[key] = info[key];
    return out;
  }

  /// COUPLE A TRACTOR TO A TRAILER. The whole articulated vehicle the roadmap
  /// asks for, assembled from two meshes that stand on their own, by matching
  /// the tractor's `fifth_wheel` socket to the trailer's `kingpin`. Both are
  /// authored at COUPLING_Y, so the offset is a translation in Z and the rig
  /// still touches the ground at Y=0 — which the checks assert rather than
  /// assume, because a fifth wheel and a kingpin at different heights is the
  /// classic way a coupled vehicle ends up hovering.
  function rig(tractorKey, trailerKey, opts) {
    const o = opts && typeof opts === "object" ? opts : {};
    const tKey = typeof tractorKey === "string" && has(PIECES, tractorKey) ? tractorKey : "tractor_unit";
    const rKey = typeof trailerKey === "string" && has(PIECES, trailerKey) ? trailerKey : "trailer_box";
    const head = build(tKey, { lod: o.lod, variant: o.variant });
    const tail = build(rKey, { lod: o.lod, variant: o.variant, coupled: true });
    const fw = head.sockets.fifth_wheel, kp = tail.sockets.kingpin;
    if (!fw || !kp) return head;
    const acc = { pos: [], nrm: [], col: [], parts: [] };
    placeInto(acc, head, 0, 0, 0, 0, null, "tractor");
    placeInto(acc, tail, 0, fw[0] - kp[0], fw[1] - kp[1], fw[2] - kp[2], null, "trailer");
    return assemble(acc, {
      piece: `${tKey}+${rKey}`,
      name: `${head.name} with ${tail.name.toLowerCase()}`,
      kind: "rig",
      lod: head.lod,
      variant: head.variant,
      coupled: true,
      tractor: tKey,
      trailer: rKey,
      coupling: { height: COUPLING_Y, offset: [fw[0] - kp[0], fw[1] - kp[1], fw[2] - kp[2]] },
      sockets: {},
      description: `${head.name} coupled to a ${tail.name.toLowerCase()}, joined at the fifth wheel. `
        + `Representative scenery on the 1990 baseline — original generic designs, not licensed, named or `
        + `measured vehicles, and the rig carries no load the simulation records.`,
    });
  }

  // ------------------------------------------------------------------- yard
  // An AUTHORED yard, not a scattered one: forty props placed by hand so the
  // scene reads as a freight yard with a rail siding, a container stack, a
  // quay crane and a tank farm, and so the map-LOD budget is measured against
  // something a game would actually draw rather than against forty copies of
  // the cheapest prop in the kit.
  const YARD = [
    { piece: "gantry_crane", x: -26, z: 6, yaw: 0 },
    { piece: "container_stack", x: -14, z: -8, yaw: 0 },
    { piece: "container_stack", x: -14, z: 2, yaw: 0 },
    { piece: "container_stack", x: -14, z: 12, yaw: 0 },
    { piece: "container_40ft", x: -6, z: -10, yaw: 0 },
    { piece: "container_40ft", x: -6, z: 4, yaw: 0 },
    { piece: "container_20ft", x: -6, z: 14, yaw: 0 },
    { piece: "container_20ft", x: -6, z: 21, yaw: 0 },
    { piece: "yard_crane", x: -10, z: 30, yaw: 1 },
    { piece: "tractor_unit", x: 4, z: -18, yaw: 2 },
    { piece: "trailer_box", x: 4, z: -8, yaw: 2 },
    { piece: "trailer_box", x: 9, z: -8, yaw: 2 },
    { piece: "trailer_flatbed", x: 14, z: -8, yaw: 2 },
    { piece: "trailer_tanker", x: 19, z: -8, yaw: 2 },
    { piece: "tractor_unit", x: 9, z: -18, yaw: 2 },
    { piece: "rigid_box_van", x: 15, z: -18, yaw: 2 },
    { piece: "fuel_bowser", x: 20, z: -18, yaw: 2 },
    { piece: "service_van", x: 25, z: -18, yaw: 2 },
    { piece: "car", x: 28, z: -18, yaw: 2 },
    { piece: "car", x: 28, z: -14, yaw: 2 },
    { piece: "bus", x: 28, z: -8, yaw: 2 },
    { piece: "forklift", x: 2, z: 8, yaw: 3 },
    { piece: "forklift", x: 6, z: 18, yaw: 1 },
    { piece: "pallet_stack", x: 2, z: 12, yaw: 0 },
    { piece: "pallet_stack", x: 4, z: 15, yaw: 1 },
    { piece: "crate_stack", x: 8, z: 12, yaw: 0 },
    { piece: "crate_stack", x: 12, z: 14, yaw: 2 },
    { piece: "locomotive", x: 22, z: 22, yaw: 0 },
    { piece: "wagon_container", x: 22, z: 42, yaw: 0 },
    { piece: "wagon_tank", x: 22, z: 60, yaw: 0 },
    { piece: "wagon_hopper", x: 22, z: 76, yaw: 0 },
    { piece: "storage_tank", x: 40, z: 4, yaw: 0 },
    { piece: "storage_tank", x: 40, z: 22, yaw: 0 },
    { piece: "silo_group", x: 38, z: -14, yaw: 1 },
    { piece: "conveyor", x: 32, z: 34, yaw: 1 },
    { piece: "light_mast", x: -2, z: -22, yaw: 0 },
    { piece: "light_mast", x: -2, z: 26, yaw: 0 },
    { piece: "light_mast", x: 30, z: -22, yaw: 0 },
    { piece: "light_mast", x: 30, z: 26, yaw: 0 },
    { piece: "light_mast", x: 16, z: 40, yaw: 0 },
  ];

  /// The whole yard as one mesh. Defaults to the MAP lod because that is the
  /// question the budget asks: what does a scene of this many props cost at the
  /// zoom a scene of this many props is drawn at.
  function yard(opts) {
    const o = opts && typeof opts === "object" ? opts : {};
    const lod = o.lod === undefined ? 1 : normLod(o.lod);
    const variant = typeof o.variant === "number" && isFinite(o.variant) ? Math.abs(Math.round(o.variant)) % 1000 : 0;
    const acc = { pos: [], nrm: [], col: [], parts: [] };
    const counts = {};
    for (let i = 0; i < YARD.length; i += 1) {
      const slot = YARD[i];
      const mesh = build(slot.piece, { lod, variant: variant + i });
      counts[slot.piece] = (counts[slot.piece] || 0) + 1;
      placeInto(acc, mesh, slot.yaw, slot.x, 0, slot.z, `prop ${i + 1} / ${slot.piece}`);
    }
    return assemble(acc, {
      piece: "yard",
      name: "Freight yard scene",
      kind: "scene",
      lod,
      variant,
      propCount: YARD.length,
      composition: counts,
      sockets: {},
      description: `A representative freight yard of ${YARD.length} props: a quay crane, container stacks, `
        + `a lorry park, a rail siding and a tank farm. Original generic designs on the 1990 baseline, `
        + `arranged for legibility; it is not a real terminal, has no measured layout, and grants the `
        + `simulation no throughput, storage or capacity.`,
    });
  }

  // ------------------------------------------------------------------- reuse
  /// MEASURED reuse, not claimed reuse. Builds every piece at both LODs with
  /// the trace running and reports which shared builder each one actually
  /// called. `builders` is every registered builder; `pieces` maps a piece to
  /// the builders it used; `byBuilder` inverts that.
  function reuse() {
    const pieces = {}, byBuilder = {}, triangles = {};
    const names = Object.keys(SHARED).sort();
    for (const name of names) byBuilder[name] = [];
    const outer = TRACE;
    for (const key of PIECE_KEYS) {
      const seen = new Set();
      TRACE = seen;
      let near, map;
      try {
        near = build(key, { lod: 0 }).triangleCount;
        map = build(key, { lod: 1 }).triangleCount;
      } finally {
        TRACE = outer;
      }
      // The counts are reported from the TRACED builds on purpose: quoted
      // beside a build taken with the trace off, they are the evidence that
      // measuring the reuse did not change the thing being measured.
      triangles[key] = [near, map];
      const used = Array.from(seen).sort();
      pieces[key] = used;
      for (const name of used) byBuilder[name].push(key);
    }
    let served = 0;
    for (const name of names) if (byBuilder[name].length > 1) served += 1;
    return {
      builders: names,
      builderCount: names.length,
      sharedByMoreThanOne: served,
      pieceCount: PIECE_KEYS.length,
      pieces,
      byBuilder,
      triangles,
    };
  }

  function pieces() { return PIECE_KEYS.slice(); }
  function meta(key) {
    const p = has(PIECES, key) ? PIECES[key] : null;
    return p ? { key, name: p.name, kind: p.kind, blurb: p.blurb } : null;
  }
  function yardRoster() { return YARD.map((slot) => ({ piece: slot.piece, x: slot.x, z: slot.z, yaw: slot.yaw })); }

  return Object.freeze({
    build, pieces, meta, rig, yard, yardRoster, reuse,
    palette: P,
    couplingHeight: COUPLING_Y,
    budgets: Object.freeze({ near: [300, 2500], map: [30, 200], yard: 8000 }),
  });
});
