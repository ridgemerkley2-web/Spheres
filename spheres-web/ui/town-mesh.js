// A representative temperate town block, and the masonry kit it is assembled
// from. Roadmap section G, asset row `urban.temperate_block.v1`.
//
// WHY THIS EXISTS. Section G says make towns visible before authoring another
// prestige military model, and P0 asks this file exactly one question: does a
// settlement read well at useful map zoom? Everything here is aimed at that
// question. It is ONE regional kit — temperate masonry, the brick-and-slate
// town — not the eight kits the roadmap eventually wants, and one assembled
// block, not a city.
//
// WHY BUILT AND NOT LOADED. Same reason as arsenal-models.js: no build step,
// no CDN, no third-party code. A town is also the asset class where authoring
// wins by the widest margin, because a town is repetition with variation and
// repetition with variation is what a function is. A block costs nothing on
// the wire and a few milliseconds of arithmetic.
//
// WHAT IT IS NOT. Representative scenery. Not a real town, no street-accurate
// layout, no measured building. Roadmap section 3: a generic design stays
// original game art, and nothing here grants a nation housing, capacity or
// output. The 1990 baseline is why there is no glass tower and no solar roof
// in the kit — a temperate town in January 1990 is brick, render, slate and
// clay tile, with one system-built slab block if the block is unlucky.
//
// MODEL SPACE. Metres. +X starboard, +Y up, +Z the direction a building faces
// (its street elevation). Y=0 is ground contact — the carriageway for a block,
// the pad for a single building — and a building root sits on the centre of
// its own footprint at grade, which is what the shared production contract
// asks of one.
//
// DETERMINISM. No clock and no entropy source in this file, at all. Every
// choice a lot makes — kind, frontage width, storey count, finish scheme, the
// lean of a street tree — is an integer hash of (block id, lot index, a salt
// naming the decision). The same id builds the same town on every machine
// forever, which is the only reason a town can be SAVED as an id rather than
// as geometry.
//
// COLOUR IS RESOLVED LATE, and that is what makes the reuse real. A baked
// variant stores, per vertex, a material SLOT (wall, roof, trim, glass,
// ground, foliage, metal, dark) and a multiplier — never a colour. The colour
// is worked out when the variant is stamped into a block, from that lot's
// finish scheme and the stamped normal. So a street of houses can share three
// baked meshes and still be three bricks and two slates under one sun, and the
// variant cache stays small enough to be worth having.
//
// WHAT THE DETAIL PASS CHANGED, and why. The first cut of this kit was honest
// massing with roofs on it, and it read as massing. Two things were capping it
// and both are fixed here.
//
// One: EVERY TRIANGLE WAS FLAT SHADED. A drainpipe, a chimney pot, a tree
// trunk and a lamp column were all faceted prisms, because a face normal is
// what `tri` computes and nothing ever averaged them. Smooth normals across a
// curved surface cost NOTHING — not one triangle — and they are the single
// largest gain available to a vertex-coloured renderer with no textures. See
// `Builder.smoothed`. Brick, render, glazing and roof planes stay flat,
// because they are flat.
//
// Two: THE OPENINGS WERE NOT OPEN. Every window, shopfront and fanlight was
// authored as a reveal running back from the wall plane to a pane of glass
// behind it, and every one of them was set into a SOLID box whose own face was
// drawn straight across the front of it. The file's own comment — "what makes
// a box read as a building is the rhythm of its holes" — was describing
// geometry no camera could reach. Walls are cut now; see `Builder.wall` and
// `Builder.opening`.
//
// Three: THE DETAIL CEILING. A street reads from the things below eaves level —
// window reveals with real cills and lintels, door surrounds, gutters and
// downpipes, slate courses, dormers, kerbs, hedges, parked cars — and none of
// them existed. They do now, and they are paid for with a third LOD: `close`
// for inspection, `mid` for a card, `map` for the map. `mid` is not a separate
// authoring of anything; it is the same call graph with the small stuff turned
// off by `Builder.detail`, which is why it cannot drift away from the building
// it is a cheaper view of.
(function (root, factory) {
  const api = factory();
  if (typeof module === "object" && module.exports) module.exports = api;
  else root.TownMesh = api;
})(typeof globalThis !== "undefined" ? globalThis : this, function () {
  "use strict";

  // Floor to floor. Three metres is the number a temperate flat or house
  // actually has; retail and offices get four, because a shopfront only three
  // metres tall reads as a garage.
  const STOREY = 3.0;
  const TALL_STOREY = 4.0;

  const SLOT = {
    WALL: 0, ROOF: 1, TRIM: 2, GLASS: 3, GROUND: 4, FOLIAGE: 5, METAL: 6, DARK: 7,
    // Three car paints. They are slots of their own rather than a reuse of
    // TRIM because TRIM moves with the finish scheme, and a parked car does
    // not change colour when the house behind it changes from brick to render.
    CAR_A: 8, CAR_B: 9, CAR_C: 10,
  };

  // Six finish schemes. Wall, roof and trim move; everything else is the same
  // town in all six, because tarmac and leaves do not change when the bricks
  // do. This is the temperate palette only — an arid courtyard kit or a
  // continental estate kit would be a different table in a later kit.
  const SCHEMES = [
    { id: "red_brick_slate", wall: [0.54, 0.33, 0.27], roof: [0.27, 0.29, 0.32], trim: [0.85, 0.84, 0.79] },
    { id: "brown_brick_tile", wall: [0.45, 0.34, 0.27], roof: [0.50, 0.27, 0.20], trim: [0.79, 0.77, 0.71] },
    { id: "cream_render_slate", wall: [0.77, 0.73, 0.63], roof: [0.29, 0.30, 0.33], trim: [0.55, 0.52, 0.47] },
    { id: "grey_stone_slate", wall: [0.60, 0.59, 0.55], roof: [0.30, 0.32, 0.34], trim: [0.72, 0.70, 0.65] },
    { id: "pale_brick_tile", wall: [0.67, 0.59, 0.49], roof: [0.46, 0.29, 0.23], trim: [0.83, 0.81, 0.77] },
    { id: "painted_render_tile", wall: [0.71, 0.70, 0.66], roof: [0.41, 0.28, 0.23], trim: [0.31, 0.36, 0.34] },
  ];
  // THE REPAINT PREDICATE CONSTRAINS THIS TABLE. A sand or winter finish
  // rewrites a vertex only where g > r*1.025 && g > b*1.06 && g > 0.12, so
  // FOLIAGE is green-dominant deliberately: leaves and grass are exactly what
  // should go straw or snow-white when the town moves climate, and they are
  // the only slot here that does. Masonry, tarmac, glass, steel and the three
  // car paints all fail the predicate on purpose — a brick town does not turn
  // sand-coloured, and a red car stays red in the snow.
  const FIXED = [
    null, null, null,
    [0.20, 0.26, 0.30],   // GLASS  — 1990 glazing is a dark hole, not a mirror
    [0.25, 0.25, 0.26],   // GROUND — carriageway, paving, yards
    [0.28, 0.40, 0.23],   // FOLIAGE
    [0.53, 0.55, 0.57],   // METAL
    [0.10, 0.11, 0.12],   // DARK   — openings, rubber, shadow gaps
    [0.44, 0.14, 0.13],   // CAR_A  — a dull red saloon
    [0.44, 0.48, 0.54],   // CAR_B  — pale blue-grey
    [0.74, 0.72, 0.68],   // CAR_C  — cream
  ];

  // One sun, fixed, high and to the right. Lighting is baked into the vertex
  // colour because the renderer this feeds is the flat vertex-colour path the
  // rest of the game already uses; separating albedo from light properly is a
  // renderer milestone and not this file's.
  const SUN = (function () {
    const v = [0.42, 0.80, 0.40], n = Math.hypot(v[0], v[1], v[2]);
    return [v[0] / n, v[1] / n, v[2] / n];
  })();
  const AMBIENT = 0.52, DIFFUSE = 0.58;

  // ------------------------------------------------------------------- hash
  // The only source of variety here. A lot asks a question by salt — "which
  // kind", "how wide", "which finish" — and gets the same answer on every
  // machine and every reload. Murmur3's finaliser with a stir in front of it:
  // not cryptographic, does not need to be, and it decorrelates small integers,
  // which is the whole job.
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

  // ---------------------------------------------------------------- builder
  // Triangle soup with a yaw-and-translate stack. YAW IS RESTRICTED TO THE
  // FOUR CARDINALS on purpose: a town on a street grid never needs another
  // angle, and cos/sin at 0/90/180/270 taken from a table rather than from
  // Math.cos are exactly 0 and +/-1 — so a stamped instance keeps exactly unit
  // normals, and two blocks built from the same variant are bitwise identical
  // rather than nearly identical.
  const YAW = [[1, 0], [0, 1], [-1, 0], [0, -1]];

  function Builder(detail) {
    this.pos = []; this.nrm = []; this.slot = []; this.mat = []; this.sch = [];
    this.tf = { c: 1, s: 0, x: 0, y: 0, z: 0 };
    this.stack = [];
    this.scheme = 0;
    this.parts = [];
    // Pending elevations and the openings that will be cut out of them; see
    // the punched-wall note below.
    this.faces = []; this.holes = [];
    // 2 close, 1 card, 0 map. Read by the shared kit — a chamfer, a slate
    // course, a glazing bar, a tree limb and a parked car all ask it whether
    // they are worth drawing — so the three LODs are one call graph and a kind
    // never has to know which one it is being built at.
    this.detail = detail == null ? 2 : detail | 0;
  }
  /// How many sides a round thing gets. Curvature is the one place where
  /// segments are worth real triangles, so close view is generous; the map is
  /// where a six-sided pipe is not only acceptable but invisible.
  Builder.prototype.segs = function (close, mid, map) {
    return this.detail >= 2 ? close : this.detail >= 1 ? (mid == null ? close : mid) : (map == null ? 4 : map);
  };
  Builder.prototype.push = function (yawIdx, tx, ty, tz) {
    const t = this.tf, r = YAW[((yawIdx | 0) % 4 + 4) % 4];
    this.stack.push(t);
    this.tf = {
      c: t.c * r[0] - t.s * r[1],
      s: t.c * r[1] + t.s * r[0],
      x: t.c * (tx || 0) + t.s * (tz || 0) + t.x,
      y: (ty || 0) + t.y,
      z: -t.s * (tx || 0) + t.c * (tz || 0) + t.z,
    };
    return this;
  };
  Builder.prototype.pop = function () {
    this.tf = this.stack.pop() || { c: 1, s: 0, x: 0, y: 0, z: 0 };
    return this;
  };
  Builder.prototype.xf = function (p) {
    const t = this.tf;
    return [t.c * p[0] + t.s * p[2] + t.x, p[1] + t.y, -t.s * p[0] + t.c * p[2] + t.z];
  };

  // A degenerate triangle is DROPPED, not emitted with a fallback normal. A
  // cone tip or a zero-width mullion is a modelling slip rather than a shape,
  // and letting one through costs either a NaN in an attribute buffer or a
  // black facet nobody can explain three months later.
  Builder.prototype.tri = function (a, b, c, slot, mat) {
    const A = this.xf(a), B = this.xf(b), C = this.xf(c);
    const ux = B[0] - A[0], uy = B[1] - A[1], uz = B[2] - A[2];
    const vx = C[0] - A[0], vy = C[1] - A[1], vz = C[2] - A[2];
    let nx = uy * vz - uz * vy, ny = uz * vx - ux * vz, nz = ux * vy - uy * vx;
    const len = Math.hypot(nx, ny, nz);
    if (!(len > 1e-7)) return this;
    nx /= len; ny /= len; nz /= len;
    const m = mat == null ? 1 : mat;
    const tri = [A, B, C];
    for (let k = 0; k < 3; k += 1) {
      this.pos.push(tri[k][0], tri[k][1], tri[k][2]);
      this.nrm.push(nx, ny, nz);
      this.slot.push(slot | 0);
      this.mat.push(m);
      this.sch.push(this.scheme);
    }
    return this;
  };
  Builder.prototype.quad = function (a, b, c, d, slot, mat) {
    return this.tri(a, b, c, slot, mat).tri(a, c, d, slot, mat);
  };
  Builder.prototype.fan = function (pts, slot, mat) {
    for (let i = 1; i + 1 < pts.length; i += 1) this.tri(pts[0], pts[i], pts[i + 1], slot, mat);
    return this;
  };

  /// An axis-aligned box by extents, because almost every part of a building
  /// is positioned by a face it must sit flush against — a floor level, an
  /// eaves line, a wall plane — and extents say that directly.
  Builder.prototype.box = function (x0, x1, y0, y1, z0, z1, slot, mat) {
    this.quad([x1, y0, z0], [x1, y1, z0], [x1, y1, z1], [x1, y0, z1], slot, mat);
    this.quad([x0, y0, z0], [x0, y0, z1], [x0, y1, z1], [x0, y1, z0], slot, mat);
    this.quad([x0, y1, z0], [x0, y1, z1], [x1, y1, z1], [x1, y1, z0], slot, mat);
    this.quad([x0, y0, z0], [x1, y0, z0], [x1, y0, z1], [x0, y0, z1], slot, mat);
    this.quad([x0, y0, z1], [x1, y0, z1], [x1, y1, z1], [x0, y1, z1], slot, mat);
    this.quad([x0, y0, z0], [x0, y1, z0], [x1, y1, z0], [x1, y0, z0], slot, mat);
    return this;
  };
  /// A flat slab on a ground plane — carriageway, footway, yard, pitch. Two
  /// triangles, because ground is the surface a town has most of and giving it
  /// thickness buys nothing you can see.
  ///
  /// IT FACES UP. It used to be wound the other way, and every square metre of
  /// ground in the kit — carriageway, footways, gardens, yards, flat roofs,
  /// the stadium pitch — was therefore lit as a downward face: `colourOf` saw
  /// a negative lambert and baked pure ambient, so the whole town stood on
  /// something half as bright as it should be. The renderer flips a normal
  /// towards the eye before lighting it, which is exactly why nobody caught
  /// it: the ground was not black, only wrong. Measured on the mixed block,
  /// 56,000 m^2 of surface faced down against 17,000 m^2 up.
  Builder.prototype.slab = function (x0, x1, z0, z1, y, slot, mat) {
    return this.quad([x0, y, z0], [x0, y, z1], [x1, y, z1], [x1, y, z0], slot, mat);
  };
  /// The same two triangles facing DOWN. A lamp lens, a canopy underside and a
  /// balcony soffit are seen from below and want the ambient reading `slab`
  /// used to give everything by accident.
  Builder.prototype.soffit = function (x0, x1, z0, z1, y, slot, mat) {
    return this.quad([x0, y, z0], [x1, y, z0], [x1, y, z1], [x0, y, z1], slot, mat);
  };

  // ------------------------------------------------------- punched walls
  // THE HOLES WERE NEVER HOLES. Every opening in this kit — sash, ribbon,
  // shopfront, fanlight — was authored as a reveal running back from the wall
  // plane to a pane of glass 170 mm behind it, and every one of them was
  // INVISIBLE, because the wall it was set into is a solid box and the box's
  // own face was drawn across the front of it. The depth test did the rest. So
  // the file's own comment — "what makes a box read as a building is the
  // rhythm of its holes" — was describing geometry the player could not see:
  // what showed was the cill, the lintel and the jambs, and between them a
  // rectangle of bare brick. Every pane, every glazing bar, every reveal and
  // every fanlight in the town was buried.
  //
  // The repair is to cut the wall rather than to move the window forward. An
  // elevation registers itself as a PENDING FACE instead of drawing straight
  // away; each opening registers a world-space box; and at the end of the
  // building the faces are emitted as a grid of rectangles with the openings
  // left out. Bands first, then columns inside a band, so a five-by-six
  // elevation costs about sixty rectangles rather than the nine hundred a
  // naive full grid would produce.

  /// Record an opening, in world space, so any face it lands on is cut. The
  /// thin axis is opened out by 120 mm either way: a wall is a plane and an
  /// opening is a rectangle on it, and no two wall planes in this kit are
  /// closer together than that.
  Builder.prototype.opening = function (x0, x1, y0, y1, zf) {
    const a = this.xf([x0, y0, zf]), b = this.xf([x1, y1, zf]);
    const box = [Math.min(a[0], b[0]), Math.max(a[0], b[0]), Math.min(a[1], b[1]), Math.max(a[1], b[1]),
      Math.min(a[2], b[2]), Math.max(a[2], b[2])];
    for (const k of [0, 2, 4]) {
      if (box[k + 1] - box[k] < 0.02) { box[k] -= 0.12; box[k + 1] += 0.12; }
    }
    this.holes.push(box);
    return this;
  };
  /// A box whose four vertical faces are punchable. Top and bottom go in
  /// straight away — nothing is ever cut out of them — and the elevations wait
  /// until the openings are known.
  Builder.prototype.wall = function (x0, x1, y0, y1, z0, z1, slot, mat) {
    this.quad([x0, y1, z0], [x0, y1, z1], [x1, y1, z1], [x1, y1, z0], slot, mat);
    this.quad([x0, y0, z0], [x1, y0, z0], [x1, y0, z1], [x0, y0, z1], slot, mat);
    for (const f of [["z+", z1, x0, x1], ["z-", z0, x0, x1], ["x+", x1, z0, z1], ["x-", x0, z0, z1]]) {
      this.faces.push({ tf: this.tf, side: f[0], at: f[1], u0: f[2], u1: f[3], v0: y0, v1: y1, slot, mat });
    }
    return this;
  };
  /// Local coordinates of a world box under a cardinal transform. Cardinal
  /// yaw is what makes this exact: an axis-aligned box stays axis-aligned, so
  /// the two corners are enough and no clipping is needed.
  function toLocal(tf, box) {
    const c = tf.c, s = tf.s;
    const pts = [];
    for (const X of [box[0], box[1]]) {
      for (const Z of [box[4], box[5]]) {
        pts.push([c * (X - tf.x) - s * (Z - tf.z), s * (X - tf.x) + c * (Z - tf.z)]);
      }
    }
    let x0 = Infinity, x1 = -Infinity, z0 = Infinity, z1 = -Infinity;
    for (const p of pts) {
      if (p[0] < x0) x0 = p[0];
      if (p[0] > x1) x1 = p[0];
      if (p[1] < z0) z0 = p[1];
      if (p[1] > z1) z1 = p[1];
    }
    return [x0, x1, box[2] - tf.y, box[3] - tf.y, z0, z1];
  }
  Builder.prototype.flushFaces = function () {
    const faces = this.faces;
    if (!faces.length) { this.holes = []; return this; }
    this.faces = [];
    const saved = this.tf;
    for (const f of faces) {
      this.tf = f.tf;
      const cuts = [];
      for (const world of this.holes) {
        const h = toLocal(f.tf, world);
        const onFace = f.side[0] === "z" ? (h[4] < f.at + 0.13 && h[5] > f.at - 0.13)
          : (h[0] < f.at + 0.13 && h[1] > f.at - 0.13);
        if (!onFace) continue;
        const u0 = f.side[0] === "z" ? h[0] : h[4], u1 = f.side[0] === "z" ? h[1] : h[5];
        if (u1 <= f.u0 + 1e-4 || u0 >= f.u1 - 1e-4 || h[3] <= f.v0 + 1e-4 || h[2] >= f.v1 - 1e-4) continue;
        cuts.push([Math.max(u0, f.u0), Math.min(u1, f.u1), Math.max(h[2], f.v0), Math.min(h[3], f.v1)]);
      }
      emitFace(this, f, cuts);
    }
    this.tf = saved;
    this.holes = [];
    return this;
  };
  function sortedBounds(values, lo, hi) {
    const out = [lo];
    values.slice().sort((a, b) => a - b).forEach((v) => { if (v > out[out.length - 1] + 1e-4 && v < hi - 1e-4) out.push(v); });
    out.push(hi);
    return out;
  }
  function emitFace(b, f, cuts) {
    const quadAt = (u0, u1, v0, v1) => {
      if (!(u1 - u0 > 1e-4 && v1 - v0 > 1e-4)) return;
      const a = f.at;
      if (f.side === "z+") b.quad([u0, v0, a], [u1, v0, a], [u1, v1, a], [u0, v1, a], f.slot, f.mat);
      else if (f.side === "z-") b.quad([u0, v0, a], [u0, v1, a], [u1, v1, a], [u1, v0, a], f.slot, f.mat);
      else if (f.side === "x+") b.quad([a, v0, u0], [a, v1, u0], [a, v1, u1], [a, v0, u1], f.slot, f.mat);
      else b.quad([a, v0, u0], [a, v0, u1], [a, v1, u1], [a, v1, u0], f.slot, f.mat);
    };
    if (!cuts.length) { quadAt(f.u0, f.u1, f.v0, f.v1); return; }
    const vs = [];
    for (const c of cuts) { vs.push(c[2], c[3]); }
    const bands = sortedBounds(vs, f.v0, f.v1);
    for (let i = 0; i + 1 < bands.length; i += 1) {
      const v0 = bands[i], v1 = bands[i + 1], mid = (v0 + v1) / 2;
      const active = cuts.filter((c) => c[2] < mid && c[3] > mid);
      if (!active.length) { quadAt(f.u0, f.u1, v0, v1); continue; }
      const us = [];
      for (const c of active) { us.push(c[0], c[1]); }
      const cols = sortedBounds(us, f.u0, f.u1);
      for (let j = 0; j + 1 < cols.length; j += 1) {
        const u0 = cols[j], u1 = cols[j + 1], um = (u0 + u1) / 2;
        if (active.some((c) => c[0] < um && c[1] > um)) continue;
        quadAt(u0, u1, v0, v1);
      }
    }
  }

  /// Weld vertex normals inside ONE drawn group, so a curved surface shades as
  /// a curve instead of as a prism. This is the cheapest realism in the file:
  /// it moves no vertex and adds no triangle, and it is the difference between
  /// a hexagonal drainpipe and a drainpipe.
  ///
  /// SCOPED TO THE GROUP ON PURPOSE. A whole-mesh weld would find the downpipe
  /// where it meets the wall behind it and tilt the corner of a four-metre
  /// wall quad to match a 60 mm pipe — a shading fault you cannot find by
  /// looking at the pipe. Only what one call draws is welded together.
  ///
  /// The crease is what keeps a cap flat. Faces sharing a position join a
  /// cluster only while they stay inside CREASE of that cluster's running
  /// average, so the eight sides of a chimney pot merge into one smooth barrel
  /// and the disc on top of it stays a disc. No sorting, no set arithmetic:
  /// insertion order is the iteration order, which is what keeps a welded mesh
  /// byte-identical from one process to the next.
  const CREASE = 0.62;    // cos ~51.7 deg. An eight-sided tube merges; a cap does not.
  Builder.prototype.smoothed = function (draw, crease) {
    const first = this.pos.length / 3;
    draw();
    const last = this.pos.length / 3;
    const cos = crease == null ? CREASE : crease;
    const at = new Map();
    for (let v = first; v < last; v += 1) {
      const i = v * 3;
      // Quantised to 1/4096 m. Coincident vertices are usually bit-identical
      // because they come from the same `oval` call, but a ring stamped
      // through two different transform pushes need not be, and a quarter of a
      // millimetre is far below anything the geometry means.
      const key = Math.round(this.pos[i] * 4096) + "|" + Math.round(this.pos[i + 1] * 4096)
        + "|" + Math.round(this.pos[i + 2] * 4096);
      let clusters = at.get(key);
      if (!clusters) { clusters = []; at.set(key, clusters); }
      const nx = this.nrm[i], ny = this.nrm[i + 1], nz = this.nrm[i + 2];
      let joined = false;
      for (let c = 0; c < clusters.length; c += 1) {
        const acc = clusters[c], len = Math.hypot(acc[0], acc[1], acc[2]);
        if (len > 1e-9 && (nx * acc[0] + ny * acc[1] + nz * acc[2]) / len > cos) {
          acc[0] += nx; acc[1] += ny; acc[2] += nz; acc[3].push(v); joined = true; break;
        }
      }
      if (!joined) clusters.push([nx, ny, nz, [v]]);
    }
    for (const clusters of at.values()) {
      for (let c = 0; c < clusters.length; c += 1) {
        const acc = clusters[c];
        if (acc[3].length < 2) continue;
        const len = Math.hypot(acc[0], acc[1], acc[2]);
        if (!(len > 1e-6)) continue;
        const nx = acc[0] / len, ny = acc[1] / len, nz = acc[2] / len;
        for (let k = 0; k < acc[3].length; k += 1) {
          const i = acc[3][k] * 3;
          this.nrm[i] = nx; this.nrm[i + 1] = ny; this.nrm[i + 2] = nz;
        }
      }
    }
    return this;
  };

  /// A box with every edge chamfered. A 20 mm bevel is the whole difference
  /// between a machined stone cill and a cardboard rectangle: it catches the
  /// sun on an edge that would otherwise be a single hard line between two
  /// flat tones, and on a vertex-lit mesh that highlight is the only edge cue
  /// there is. Sixty triangles against twelve, so it goes on the pieces a hand
  /// would reach — cills, copings, kerbs, string courses, gate piers, nosings
  /// — and not on the wall behind them.
  ///
  /// Built as three lofted bands over an OCTAGONAL ring, which chamfers the
  /// four vertical arrises for free and lets `loft` settle the winding. Below
  /// close detail it is a plain box, which is most of what the card LOD saves.
  Builder.prototype.bevelBox = function (x0, x1, y0, y1, z0, z1, slot, mat, chamf) {
    const c = Math.min(chamf == null ? 0.022 : chamf, (x1 - x0) / 2.5, (y1 - y0) / 2.5, (z1 - z0) / 2.5);
    if (!(c > 0.003) || this.detail < 2) return this.box(x0, x1, y0, y1, z0, z1, slot, mat);
    const ring = (k) => [
      [x0 + c + k, z0 + k], [x1 - c - k, z0 + k], [x1 - k, z0 + c + k], [x1 - k, z1 - c - k],
      [x1 - c - k, z1 - k], [x0 + c + k, z1 - k], [x0 + k, z1 - c - k], [x0 + k, z0 + c + k],
    ];
    const outer = ring(0), inner = ring(c);
    this.loft(inner, outer, y0, y0 + c, slot, mat, false);
    this.loft(outer, outer, y0 + c, y1 - c, slot, mat, false);
    this.loft(outer, inner, y1 - c, y1, slot, mat, false);
    this.fan(reversed(inner).map((p) => [p[0], y1, p[1]]), slot, mat);
    this.fan(inner.map((p) => [p[0], y0, p[1]]), slot, mat);
    return this;
  };

  /// A lofted prism between two matching point rings. Orientation is FIXED
  /// HERE rather than asked of the caller: the signed area says which way the
  /// list runs, and a counter-clockwise one is reversed, so a ring written by
  /// hand and a ring from `oval` both come out with walls facing outward and
  /// caps facing up.
  Builder.prototype.loft = function (lower, upper, y0, y1, slot, mat, caps) {
    let area = 0;
    for (let i = 0; i < lower.length; i += 1) {
      const j = (i + 1) % lower.length;
      area += lower[i][0] * lower[j][1] - lower[j][0] * lower[i][1];
    }
    const lo = area > 0 ? lower.slice().reverse() : lower;
    const up = area > 0 ? upper.slice().reverse() : upper;
    const m = mat == null ? 1 : mat;
    for (let i = 0; i < lo.length; i += 1) {
      const j = (i + 1) % lo.length;
      this.quad([lo[i][0], y0, lo[i][1]], [lo[j][0], y0, lo[j][1]],
        [up[j][0], y1, up[j][1]], [up[i][0], y1, up[i][1]], slot, m);
    }
    if (caps !== false) {
      this.fan(up.map((p) => [p[0], y1, p[1]]), slot, m);
      this.fan(lo.slice().reverse().map((p) => [p[0], y0, p[1]]), slot, m);
    }
    return this;
  };
  /// A quad strip between two rings with the winding TAKEN AS GIVEN. `loft`
  /// forces walls outward, which is right for a solid and wrong for a bowl:
  /// terracing is seen from the inside, and the concourse deck between two
  /// rings has to face up rather than down. Reverse a ring to flip it.
  Builder.prototype.band = function (lower, upper, y0, y1, slot, mat) {
    for (let i = 0; i < lower.length; i += 1) {
      const j = (i + 1) % lower.length;
      this.quad([lower[i][0], y0, lower[i][1]], [lower[j][0], y0, lower[j][1]],
        [upper[j][0], y1, upper[j][1]], [upper[i][0], y1, upper[i][1]], slot, mat);
    }
    return this;
  };
  function reversed(pts) { return pts.slice().reverse(); }
  Builder.prototype.cyl = function (cx, cz, y0, y1, r0, r1, seg, slot, mat, caps) {
    return this.loft(oval(cx, cz, r0, r0, seg), oval(cx, cz, r1, r1, seg), y0, y1, slot, mat, caps);
  };

  /// A ring in the x-z plane, clockwise as plotted — the orientation `loft`
  /// wants for an outward wall, though `loft` would fix it either way.
  function oval(cx, cz, rx, rz, seg) {
    const pts = [];
    for (let i = 0; i < seg; i += 1) {
      const t = -(i / seg) * Math.PI * 2;
      pts.push([cx + Math.cos(t) * rx, cz + Math.sin(t) * rz]);
    }
    return pts;
  }

  // ------------------------------------------------------- round primitives
  // Everything below is `cyl`, `loft` and `oval` with the normals welded
  // afterwards. They exist as their own names so that a call site says what it
  // meant — this is a pipe, this is a dome, this is a leaf mass — and so that
  // adding a segment to every pipe in the town is one edit.

  /// A pipe, a pot, a bollard, a column. Smooth around the axis and flat
  /// across the caps.
  function tube(b, cx, cz, y0, y1, r0, r1, seg, slot, mat, caps) {
    return b.smoothed(() => { b.cyl(cx, cz, y0, y1, r0, r1, seg, slot, mat, caps); });
  }
  /// A half-round section swept along X: gutters and ridge tiles, which are
  /// the two curved profiles a temperate roof actually has. `turn` rotates the
  /// open side — 1 opens upward for a gutter, -1 downward for a ridge cap.
  function halfRound(b, x0, x1, cy, cz, r, seg, turn, slot, mat) {
    const n = Math.max(3, seg | 0), up = turn < 0 ? -1 : 1;
    return b.smoothed(() => {
      for (let i = 0; i < n; i += 1) {
        const a0 = Math.PI * (i / n), a1 = Math.PI * ((i + 1) / n);
        const p0 = [cz - Math.cos(a0) * r, cy + up * Math.sin(a0) * r];
        const p1 = [cz - Math.cos(a1) * r, cy + up * Math.sin(a1) * r];
        // Wound so the CONVEX side faces out in both turns: the outside of a
        // gutter is what the street sees, and the outside of a ridge cap is
        // what the sun catches.
        if (up > 0) b.quad([x0, p1[1], p1[0]], [x1, p1[1], p1[0]], [x1, p0[1], p0[0]], [x0, p0[1], p0[0]], slot, mat);
        else b.quad([x0, p0[1], p0[0]], [x1, p0[1], p0[0]], [x1, p1[1], p1[0]], [x0, p1[1], p1[0]], slot, mat);
      }
    });
  }
  /// A squashed spheroid in `rings` latitude bands. Leaf masses, hedge lobes
  /// and the odd finial. Poles are collapsed, so `tri` drops the degenerate
  /// quad halves at the top and bottom by itself.
  function lobe(b, cx, cy, cz, rx, ry, rz, seg, rings, slot, mat) {
    return b.smoothed(() => {
      let prev = null, prevY = 0;
      for (let r = 0; r <= rings; r += 1) {
        const a = Math.PI * (r / rings);
        const ring = oval(cx, cz, Math.sin(a) * rx, Math.sin(a) * rz, seg);
        const y = cy + Math.cos(a) * ry;
        // Wound BOTTOM-UP. The latitude loop runs from the top pole down, so
        // handing `band` the rings in loop order made every leaf mass in the
        // town inside-out: the crown was lit from underneath and the smooth
        // normals pointed into the tree.
        if (prev) b.band(ring, prev, y, prevY, slot, mat);
        prev = ring; prevY = y;
      }
    }, 0.2);
  }

  /// Semantic ranges, so a click on a roof can say which building it hit.
  /// Counted in vertices, matching the convention equipment-mesh.js already
  /// set for selectable parts.
  Builder.prototype.part = function (name, kind, lot, draw) {
    const first = this.pos.length / 3;
    draw();
    const count = this.pos.length / 3 - first;
    if (count > 0) this.parts.push({ name, kind, lot, first, count });
    return this;
  };

  /// Bake without resolving colour — this is all a cached variant holds.
  Builder.prototype.bake = function () {
    return {
      pos: new Float32Array(this.pos),
      nrm: new Float32Array(this.nrm),
      slot: new Uint8Array(this.slot),
      mat: new Float32Array(this.mat),
      tris: this.pos.length / 9,
    };
  };

  function colourOf(schemeIdx, slot, mat, nx, ny, nz) {
    const s = SCHEMES[schemeIdx] || SCHEMES[0];
    const base = slot === SLOT.WALL ? s.wall
      : slot === SLOT.ROOF ? s.roof
        : slot === SLOT.TRIM ? s.trim
          : FIXED[slot] || FIXED[SLOT.DARK];
    const lambert = nx * SUN[0] + ny * SUN[1] + nz * SUN[2];
    const k = mat * (AMBIENT + DIFFUSE * (lambert > 0 ? lambert : 0));
    const r = base[0] * k, g = base[1] * k, b = base[2] * k;
    return [r > 1 ? 1 : r < 0 ? 0 : r, g > 1 ? 1 : g < 0 ? 0 : g, b > 1 ? 1 : b < 0 ? 0 : b];
  }

  /// Resolve to render buffers and seat the whole thing on Y=0. Grade comes
  /// from the LOWEST vertex rather than a nominal datum, because a kerb
  /// upstand, a sunken loading dock and a garden step all disagree about where
  /// the ground is and the renderer only needs nothing to float.
  Builder.prototype.finish = function (description, extra) {
    const n = this.pos.length / 3;
    const positions = new Float32Array(this.pos);
    const normals = new Float32Array(this.nrm);
    const colors = new Float32Array(n * 3);
    for (let v = 0; v < n; v += 1) {
      const i = v * 3;
      const c = colourOf(this.sch[v], this.slot[v], this.mat[v], normals[i], normals[i + 1], normals[i + 2]);
      colors[i] = c[0]; colors[i + 1] = c[1]; colors[i + 2] = c[2];
    }
    const min = [Infinity, Infinity, Infinity], max = [-Infinity, -Infinity, -Infinity];
    for (let i = 0; i < positions.length; i += 3) {
      for (let k = 0; k < 3; k += 1) {
        if (positions[i + k] < min[k]) min[k] = positions[i + k];
        if (positions[i + k] > max[k]) max[k] = positions[i + k];
      }
    }
    const drop = min[1];
    if (drop !== 0) {
      for (let i = 1; i < positions.length; i += 3) positions[i] -= drop;
      min[1] -= drop; max[1] -= drop;
    }
    const out = {
      positions, normals, colors,
      bounds: { min, max },
      size: [max[0] - min[0], max[1] - min[1], max[2] - min[2]],
      triangleCount: n / 3,
      parts: this.parts,
      description,
    };
    if (extra) for (const key of Object.keys(extra)) out[key] = extra[key];
    return out;
  };

  /// Copy a baked variant into a block under a cardinal yaw and a ground
  /// translation, carrying the lot's finish scheme with it. This is the whole
  /// of the reuse: a dozen-odd lots, a handful of baked meshes, one pass of
  /// arithmetic per vertex and no re-authoring.
  function stampInto(out, baked, yawIdx, tx, tz, schemeIdx) {
    const r = YAW[((yawIdx | 0) % 4 + 4) % 4], c = r[0], s = r[1];
    const p = baked.pos, nn = baked.nrm;
    for (let i = 0; i < p.length; i += 3) {
      out.pos.push(c * p[i] + s * p[i + 2] + tx, p[i + 1], -s * p[i] + c * p[i + 2] + tz);
      out.nrm.push(c * nn[i] + s * nn[i + 2], nn[i + 1], -s * nn[i] + c * nn[i + 2]);
    }
    for (let v = 0; v < baked.slot.length; v += 1) {
      out.slot.push(baked.slot[v]);
      out.mat.push(baked.mat[v]);
      out.sch.push(schemeIdx);
    }
    return baked.tris;
  }

  // ------------------------------------------------------- window and door
  // Openings are where the triangles go, and that is the correct place for
  // them: what makes a box read as a building is the rhythm of its holes. A
  // sash window here costs about ninety triangles — surround, reveal, glass
  // and glazing bars — which is why a two-storey house lands near two thousand
  // and a fourteen-storey slab needs the cheaper ribbon below instead.

  /// One opening on the +Z wall plane at z = zf, centred on cx. Authored only
  /// for +Z; the other three elevations get it by pushing a cardinal yaw, so
  /// there is one of this function and not four.
  ///
  /// `dress` is the difference between a principal elevation and a back one,
  /// and it is a real distinction rather than a budget dodge: a building of
  /// this period got stone cills and lintels where the street could see them
  /// and plain openings in the return and the rear. It is also where the
  /// triangles for the fronts come from.
  function sash(b, cx, y0, w, h, zf, cols, rows, dress) {
    const hw = w / 2, y1 = y0 + h, dep = 0.17, full = b.detail >= 2;
    const shown = dress !== false;
    // A CILL IS A PROJECTING STONE, not a painted line. It oversails the jambs,
    // stands 100 mm proud of the brickwork and is chamfered on its outer
    // arris, so it takes a highlight along its whole length and throws the
    // shadow that reads, at any distance, as "there is a hole here".
    if (shown) {
      b.bevelBox(cx - hw - 0.16, cx + hw + 0.16, y0 - 0.14, y0 + 0.01, zf - 0.03, zf + 0.11, SLOT.TRIM, 1.04, 0.022);
      // Lintel over the head. Same idea, less projection, because it is
      // carrying brickwork rather than throwing water clear of it.
      b.bevelBox(cx - hw - 0.16, cx + hw + 0.16, y1 - 0.01, y1 + 0.18, zf - 0.03, zf + 0.07, SLOT.TRIM, 1.0, 0.02);
    } else {
      b.box(cx - hw - 0.14, cx + hw + 0.14, y0 - 0.1, y0 + 0.01, zf - 0.02, zf + 0.07, SLOT.TRIM, 1.04);
      b.box(cx - hw - 0.14, cx + hw + 0.14, y1 - 0.01, y1 + 0.14, zf - 0.02, zf + 0.05, SLOT.TRIM, 1.0);
    }
    b.box(cx - hw - 0.13, cx - hw, y0, y1, zf - 0.02, zf + 0.05, SLOT.TRIM, 0.98);                        // jambs
    b.box(cx + hw, cx + hw + 0.13, y0, y1, zf - 0.02, zf + 0.05, SLOT.TRIM, 0.98);
    b.opening(cx - hw, cx + hw, y0, y1, zf);
    // Reveal: four inward-facing quads from the wall plane back to the glass.
    b.quad([cx - hw, y0, zf], [cx + hw, y0, zf], [cx + hw, y0, zf - dep], [cx - hw, y0, zf - dep], SLOT.WALL, 0.72);
    b.quad([cx - hw, y1, zf - dep], [cx + hw, y1, zf - dep], [cx + hw, y1, zf], [cx - hw, y1, zf], SLOT.WALL, 0.72);
    b.quad([cx - hw, y0, zf - dep], [cx - hw, y1, zf - dep], [cx - hw, y1, zf], [cx - hw, y0, zf], SLOT.WALL, 0.8);
    b.quad([cx + hw, y0, zf], [cx + hw, y1, zf], [cx + hw, y1, zf - dep], [cx + hw, y0, zf - dep], SLOT.WALL, 0.8);
    b.quad([cx - hw, y0, zf - dep], [cx + hw, y0, zf - dep], [cx + hw, y1, zf - dep], [cx - hw, y1, zf - dep], SLOT.GLASS, 1.0);
    if (full) {
      // The frame itself, sitting in the reveal in front of the glass. Four
      // sections and a meeting rail: the window, rather than the hole.
      const fz0 = zf - dep + 0.005, fz1 = zf - dep + 0.075;
      b.box(cx - hw, cx - hw + 0.07, y0, y1, fz0, fz1, SLOT.TRIM, 1.02);
      b.box(cx + hw - 0.07, cx + hw, y0, y1, fz0, fz1, SLOT.TRIM, 1.02);
      b.box(cx - hw, cx + hw, y0, y0 + 0.07, fz0, fz1, SLOT.TRIM, 1.06);
      b.box(cx - hw, cx + hw, y1 - 0.07, y1, fz0, fz1, SLOT.TRIM, 1.06);
      b.box(cx - hw, cx + hw, y0 + h * 0.5 - 0.05, y0 + h * 0.5 + 0.05, fz0, fz1 + 0.02, SLOT.TRIM, 1.08);
    }
    if (!full) return b;
    const nc = Math.max(1, cols | 0), nr = Math.max(1, rows | 0);
    for (let i = 1; i < nc; i += 1) {
      const x = cx - hw + (w * i) / nc;
      b.box(x - 0.026, x + 0.026, y0, y1, zf - dep, zf - dep + 0.05, SLOT.TRIM, 1.05);
    }
    for (let j = 1; j < nr; j += 1) {
      const y = y0 + (h * j) / nr;
      b.box(cx - hw, cx + hw, y - 0.028, y + 0.028, zf - dep, zf - dep + 0.05, SLOT.TRIM, 1.05);
    }
    return b;
  }

  /// A horizontal glazing band with mullions at a fixed pitch. This is how a
  /// system-built block, an office or a school elevation is glazed, and it is
  /// also the reason those buildings stay inside the twelve-thousand ceiling:
  /// one band covers a whole floor for the price of a couple of sashes.
  function ribbon(b, x0, x1, y0, h, zf, pitch) {
    const y1 = y0 + h, dep = 0.14;
    if (b.detail >= 2) {
      b.bevelBox(x0 - 0.12, x1 + 0.12, y0 - 0.14, y0 + 0.01, zf - 0.02, zf + 0.1, SLOT.TRIM, 1.02, 0.02);
      b.bevelBox(x0 - 0.12, x1 + 0.12, y1 - 0.01, y1 + 0.15, zf - 0.02, zf + 0.06, SLOT.TRIM, 1.0, 0.02);
    } else {
      b.box(x0 - 0.1, x1 + 0.1, y0 - 0.12, y0, zf - 0.02, zf + 0.08, SLOT.TRIM, 1.02);
      b.box(x0 - 0.1, x1 + 0.1, y1, y1 + 0.14, zf - 0.02, zf + 0.05, SLOT.TRIM, 1.0);
    }
    b.quad([x0, y0, zf], [x1, y0, zf], [x1, y0, zf - dep], [x0, y0, zf - dep], SLOT.WALL, 0.72);
    b.quad([x0, y1, zf - dep], [x1, y1, zf - dep], [x1, y1, zf], [x0, y1, zf], SLOT.WALL, 0.72);
    b.quad([x0, y0, zf - dep], [x0, y1, zf - dep], [x0, y1, zf], [x0, y0, zf], SLOT.WALL, 0.8);
    b.quad([x1, y0, zf], [x1, y1, zf], [x1, y1, zf - dep], [x1, y0, zf - dep], SLOT.WALL, 0.8);
    b.quad([x0, y0, zf - dep], [x1, y0, zf - dep], [x1, y1, zf - dep], [x0, y1, zf - dep], SLOT.GLASS, 1.0);
    b.opening(x0, x1, y0, y1, zf);
    // Mullions at a fixed pitch, thinned out below close range. A ribbon is
    // read by its rhythm rather than by its count, so halving the mullions on
    // a card costs the elevation nothing and is most of what the card saves on
    // a building that is nothing but ribbons.
    const n = Math.max(1, Math.round((x1 - x0) / ((pitch || 1.8) * (b.detail >= 2 ? 1 : 2))));
    for (let i = 1; i < n; i += 1) {
      const x = x0 + ((x1 - x0) * i) / n;
      b.box(x - 0.05, x + 0.05, y0, y1, zf - dep, zf - dep + 0.06, SLOT.TRIM, 1.05);
    }
    if (b.detail >= 2) {
      // A transom at the head of the opening light, which is what turns a
      // glazed slot into a window at close range.
      b.box(x0, x1, y0 + h * 0.62 - 0.04, y0 + h * 0.62 + 0.04, zf - dep, zf - dep + 0.055, SLOT.TRIM, 1.06);
    }
    return b;
  }

  /// A front door with its surround, fanlight and step. Doors are worth their
  /// triangles because a door is the only part of an elevation that gives the
  /// building a human scale to be read against.
  function frontDoor(b, cx, zf, h) {
    const w = 0.95, hw = w / 2, top = h || 2.1, full = b.detail >= 2;
    b.box(cx - hw - 0.14, cx + hw + 0.14, 0, top + 0.62, zf - 0.02, zf + 0.09, SLOT.TRIM, 1.02);
    b.box(cx - hw, cx + hw, 0, top, zf - 0.06, zf + 0.02, SLOT.DARK, 1.0);
    b.box(cx - hw + 0.09, cx + hw - 0.09, 0.28, top - 0.9, zf + 0.015, zf + 0.05, SLOT.TRIM, 0.95);
    b.box(cx - hw + 0.09, cx + hw - 0.09, top - 0.78, top - 0.18, zf + 0.015, zf + 0.05, SLOT.TRIM, 0.95);
    b.box(cx - hw, cx + hw, top + 0.08, top + 0.52, zf - 0.1, zf - 0.04, SLOT.GLASS, 1.0);
    b.opening(cx - hw, cx + hw, 0.02, top + 0.52, zf);
    // Step with a chamfered nosing. A door is where the eye goes to read the
    // scale of the whole building, so it is worth the extra fifty triangles.
    b.bevelBox(cx - hw - 0.3, cx + hw + 0.3, 0, 0.15, zf, zf + 0.55, SLOT.TRIM, 1.04, 0.025);
    b.bevelBox(cx - hw - 0.34, cx + hw + 0.34, top + 0.66, top + 0.84, zf - 0.02, zf + 0.44, SLOT.TRIM, 1.05, 0.022); // hood
    if (full) {
      for (const sx of [-1, 1]) {                                     // hood brackets
        b.box(cx + sx * (hw + 0.16), cx + sx * (hw + 0.26), top + 0.2, top + 0.66, zf, zf + 0.3, SLOT.TRIM, sx > 0 ? 1.06 : 0.9);
      }
      tube(b, cx + hw - 0.18, zf + 0.06, top - 1.02, top - 0.9, 0.05, 0.05, 8, SLOT.METAL, 1.05);      // knob
      b.box(cx - 0.13, cx + 0.13, top - 1.24, top - 1.16, zf + 0.02, zf + 0.05, SLOT.METAL, 1.08);     // letter plate
      b.box(cx - hw - 0.02, cx + hw + 0.02, -0.005, 0.02, zf - 0.05, zf + 0.1, SLOT.TRIM, 1.1);        // threshold
    } else {
      tube(b, cx + hw - 0.18, zf + 0.06, top - 1.02, top - 0.9, 0.05, 0.05, 5, SLOT.METAL, 1.05);
    }
    return b;
  }

  // --------------------------------------------------------------- roofing
  /// SLATE AND TILE COURSES, as geometry. There is no texture path in this
  /// renderer, so the only way a roof can say "slate" rather than "a plane" is
  /// to be built as the courses it is made of: a run of shallow steps, each
  /// one a tread lying on the roof plane and a lip standing proud of the
  /// course below it. The lip is what does the work — it takes the sun on its
  /// top arris and drops a shadow line onto the course underneath, and a dozen
  /// of those lines are what the eye reads as a tiled roof from a street away.
  ///
  /// Four triangles a course, so a whole two-sided roof is under a hundred.
  /// Below close detail the slope is one quad, because at card and map size
  /// the courses are smaller than a pixel and all they would cost is fill.
  function slopeCourses(b, x0, x1, yEave, zEave, yRidge, zRidge, slot, mat, thk) {
    const dy = yRidge - yEave, dz = zRidge - zEave, L = Math.hypot(dy, dz);
    if (!(L > 0.05)) return b;
    let ny = -dz / L, nz = dy / L;
    if (ny < 0) { ny = -ny; nz = -nz; }                 // a roof slope always faces up
    const flip = dz > 0;                                 // the -Z slope runs the other way
    const face = (ay, az, by, bz) => {
      if (flip) b.quad([x0, by, bz], [x1, by, bz], [x1, ay, az], [x0, ay, az], slot, mat);
      else b.quad([x0, ay, az], [x1, ay, az], [x1, by, bz], [x0, by, bz], slot, mat);
    };
    if (b.detail < 2) { face(yEave, zEave, yRidge, zRidge); return b; }
    const t = thk == null ? 0.035 : thk;
    const n = Math.max(4, Math.min(20, Math.round(L / 0.34)));
    // THE COURSE HAS TO TAPER OR IT IS NOT A COURSE. The first cut of this
    // offset BOTH ends of every tread by the same 35 mm, which put all twenty
    // treads on one plane 35 mm above the roof and buried every lip underneath
    // it: measured on the house, 19 course quads sharing a single plane, and
    // the roof rendered as the bare sheet the courses exist to replace. It cost
    // 160 triangles a roof and bought nothing.
    //
    // What a slate actually does is stand proud at its LOWER edge, where it
    // laps the course below, and die back into the roof under the course above.
    // So the tread runs from `t` proud at t0 down to the plane at t1, and the
    // lip is the 35 mm step up onto it. That is the same four triangles, it is
    // continuous with the course below it, and it gives every course line a lit
    // arris and a shadow — which is the whole point.
    for (let i = 0; i < n; i += 1) {
      const t0 = i / n, t1 = (i + 1) / n;
      const ay = yEave + dy * t0 + ny * t, az = zEave + dz * t0 + nz * t;
      const by = yEave + dy * t1, bz = zEave + dz * t1;
      face(ay, az, by, bz);                                     // the course, dying back into the roof
      face(yEave + dy * t0, zEave + dz * t0, ay, az);           // its lip, standing on the one below
    }
    return b;
  }
  /// A barge board along one rake of a gable: a board of `drop` depth hanging
  /// under the verge, swept from the eaves corner up to the ridge.
  function barge(b, x0, x1, yEave, zEave, yTop, zTop, drop, slot, mat) {
    const d = drop == null ? 0.24 : drop;
    b.quad([x0, yEave - d, zEave], [x0, yEave, zEave], [x0, yTop, zTop], [x0, yTop - d, zTop], slot, mat);
    b.quad([x1, yTop - d, zTop], [x1, yTop, zTop], [x1, yEave, zEave], [x1, yEave - d, zEave], slot, mat);
    b.quad([x0, yEave - d, zEave], [x0, yTop - d, zTop], [x1, yTop - d, zTop], [x1, yEave - d, zEave], slot, mat * 0.78);
    return b;
  }
  /// Half-round ridge caps, bedded over the apex. Smooth around the section,
  /// which is the whole point of using a round profile at all.
  function ridgeCap(b, x0, x1, y, z, r, slot, mat) {
    if (b.detail < 2) return b.box(x0, x1, y - r * 0.5, y + r * 0.6, z - r, z + r, slot, mat);
    halfRound(b, x0, x1, y, z, r, b.segs(7, 5, 3), -1, slot, mat);
    b.tri([x0, y, z - r], [x0, y, z + r], [x0, y + r * 0.02, z], slot, mat * 0.9);
    b.tri([x1, y, z + r], [x1, y, z - r], [x1, y + r * 0.02, z], slot, mat * 0.9);
    return b;
  }
  /// A pitched roof with its ridge running along X, so the gables face +/-X.
  /// Eaves overhang, soffit, fascia, barge boards, slate courses and a
  /// half-round ridge: between them they are all of what tells a pitched roof
  /// from a wedge, and they are why a house looks like a house at close range.
  function gableRoof(b, hw, hd, eave, rise, over) {
    const o = over == null ? 0.32 : over;
    const X0 = -hw - o, X1 = hw + o, Z0 = -hd - o, Z1 = hd + o, top = eave + rise;
    slopeCourses(b, X0, X1, eave, Z1, top, 0, SLOT.ROOF, 1.0);
    slopeCourses(b, X0, X1, eave, Z0, top, 0, SLOT.ROOF, 0.86);
    // THE GABLE IS MASONRY, not slate. It was drawn in the roof slot, so every
    // gable end in the town was a triangle of dark grey sitting on a brick
    // wall, which is the one thing a gable never is.
    b.tri([X0, eave, Z0], [X0, eave, Z1], [X0, top, 0], SLOT.WALL, 0.82);
    b.tri([X1, eave, Z1], [X1, eave, Z0], [X1, top, 0], SLOT.WALL, 0.9);
    b.bevelBox(X0, X1, eave - 0.19, eave + 0.02, Z1 - 0.09, Z1, SLOT.TRIM, 1.02, 0.02);    // fascia
    b.bevelBox(X0, X1, eave - 0.19, eave + 0.02, Z0, Z0 + 0.09, SLOT.TRIM, 0.95, 0.02);
    // BARGE BOARDS FOLLOW THE RAKE. They used to be drawn as a full-height
    // rectangular plate from eaves to ridge across the whole depth of the
    // roof, which put a pair of two-metre fins above the roof line at each
    // gable end of every house, terrace, shop and school in the kit. Flat
    // shading and a dark roof hid them; a ridge cap and a lit slope did not.
    for (const sx of [-1, 1]) {
      const x = sx > 0 ? X1 - 0.07 : X0, xb = sx > 0 ? X1 : X0 + 0.07;
      barge(b, x, xb, eave, Z1, top, 0, 0.26, SLOT.TRIM, sx > 0 ? 1.02 : 0.94);
      barge(b, x, xb, eave, Z0, top, 0, 0.26, SLOT.TRIM, sx > 0 ? 1.0 : 0.92);
    }
    ridgeCap(b, X0, X1, top, 0, 0.15, SLOT.ROOF, 1.08);
    // Soffit — the underside of the overhang, which is seen from the pavement
    // on every one of these buildings and used to be wound facing the sky.
    b.soffit(X0, X1, Z1 - 0.09, hd, eave - 0.19, SLOT.TRIM, 0.72);
    b.soffit(X0, X1, -hd, Z0 + 0.09, eave - 0.19, SLOT.TRIM, 0.7);
    return b;
  }
  /// A hipped roof: four slopes to a short ridge. The civic and low-rise
  /// residential kinds use it because a hip reads as a deliberate building and
  /// a gable reads as a house, at any zoom. The hips get their own courses and
  /// a hip roll along each arris.
  function hipRoof(b, hw, hd, eave, rise, over) {
    const o = over == null ? 0.3 : over;
    const X0 = -hw - o, X1 = hw + o, Z0 = -hd - o, Z1 = hd + o, top = eave + rise;
    const rx = Math.max(0.4, hw - hd * 0.85);
    slopeCourses(b, X0, X1, eave, Z1, top, 0, SLOT.ROOF, 1.0);
    slopeCourses(b, X0, X1, eave, Z0, top, 0, SLOT.ROOF, 0.86);
    b.tri([X0, eave, Z0], [X0, eave, Z1], [-rx, top, 0], SLOT.ROOF, 0.82);
    b.tri([X1, eave, Z1], [X1, eave, Z0], [rx, top, 0], SLOT.ROOF, 0.9);
    for (const z of [[Z1 - 0.09, Z1, 1.02], [Z0, Z0 + 0.09, 0.95]]) {
      b.bevelBox(X0, X1, eave - 0.19, eave + 0.02, z[0], z[1], SLOT.TRIM, z[2], 0.02);
    }
    b.bevelBox(X0, X0 + 0.09, eave - 0.19, eave + 0.02, Z0, Z1, SLOT.TRIM, 0.95, 0.02);
    b.bevelBox(X1 - 0.09, X1, eave - 0.19, eave + 0.02, Z0, Z1, SLOT.TRIM, 1.0, 0.02);
    ridgeCap(b, -rx, rx, top, 0, 0.14, SLOT.ROOF, 1.08);
    if (b.detail >= 2) {
      // Hip rolls, one per arris. Four short half-rounds, and they are what
      // stops the four slopes meeting in a bare crease.
      for (const sx of [-1, 1]) {
        for (const sz of [-1, 1]) {
          const n = 5;
          for (let i = 0; i < n; i += 1) {
            const t0 = i / n, t1 = (i + 1) / n;
            const p = (t) => [sx * (rx + (hw + o - rx) * t), top + (eave - top) * t, sz * (Z1 * t)];
            // The roll sits INBOARD of the hip line, not astride it: astride,
            // it put 100 mm of roof past the eaves on each side, and a lot
            // boundary is not the place to discover that a roll is 200 mm wide.
            const a = p(t0), c = p(t1), inset = sx * 0.1;
            b.quad([a[0] - 0.1 - inset, a[1] + 0.05, a[2]], [c[0] - 0.1 - inset, c[1] + 0.05, c[2]],
              [c[0] + 0.1 - inset, c[1] + 0.05, c[2]], [a[0] + 0.1 - inset, a[1] + 0.05, a[2]], SLOT.ROOF, sx > 0 ? 1.1 : 0.92);
          }
        }
      }
    }
    b.soffit(X0, X1, Z1 - 0.09, hd, eave - 0.19, SLOT.TRIM, 0.72);
    b.soffit(X0, X1, -hd, Z0 + 0.09, eave - 0.19, SLOT.TRIM, 0.7);
    return b;
  }
  /// A gabled dormer on the +Z slope: cheeks, a pitched roof of its own, lead
  /// flashing at the abutment and a window in the face. Dormers are the reason
  /// a two-storey roof reads as lived in rather than as a lid, and they are
  /// the cheapest way to break a long unbroken slope on a terrace.
  function dormer(b, cx, w, yBase, sill, h, zFace, slope) {
    const hw = w / 2, top = sill + h;
    const zBack = zFace - (slope || 1.6);
    b.wall(cx - hw, cx + hw, yBase, top + 0.1, zBack, zFace, SLOT.WALL, 0.98);
    b.bevelBox(cx - hw - 0.07, cx + hw + 0.07, top + 0.1, top + 0.22, zBack, zFace + 0.1, SLOT.TRIM, 1.04, 0.02);
    // Its own little roof, ridged along Z so the gable faces the street.
    b.quad([cx - hw - 0.08, top + 0.22, zFace + 0.1], [cx, top + 0.62, zFace + 0.1],
      [cx, top + 0.62, zBack], [cx - hw - 0.08, top + 0.22, zBack], SLOT.ROOF, 0.88);
    b.quad([cx, top + 0.62, zFace + 0.1], [cx + hw + 0.08, top + 0.22, zFace + 0.1],
      [cx + hw + 0.08, top + 0.22, zBack], [cx, top + 0.62, zBack], SLOT.ROOF, 1.06);
    b.tri([cx - hw - 0.08, top + 0.22, zFace + 0.1], [cx + hw + 0.08, top + 0.22, zFace + 0.1], [cx, top + 0.62, zFace + 0.1], SLOT.ROOF, 1.0);
    if (b.detail >= 2) {
      // Lead soakers where the cheeks die into the main slope.
      for (const sx of [-1, 1]) {
        b.box(cx + sx * hw - 0.05, cx + sx * hw + 0.05, yBase, yBase + 0.28, zBack, zFace - 0.1, SLOT.METAL, sx > 0 ? 1.05 : 0.9);
      }
    }
    sash(b, cx, sill, w - 0.62, h - 0.28, zFace, 2, 3);
    return b;
  }
  /// Dormers on the +Z slope of a pitched roof, sized to the roof they are
  /// standing on. The caller says where along the frontage it wants them; the
  /// height comes from the room between the roof surface and the ridge, so a
  /// shallow roof gets a squat dormer and a steep one gets a full-height
  /// window, and neither ever pokes out of the ridge.
  function roofDormers(b, hw, hd, eave, rise, over, xs, w) {
    if (b.detail < 2) return b;                         // a dormer is a close-range read
    const Z1 = hd + (over == null ? 0.32 : over), depth = 1.4;
    const zFace = hd * 0.9;
    const roofY = (z) => eave + rise * (1 - z / Z1);
    const yBase = roofY(zFace) - 0.12, sill = yBase + 0.36;
    const h = Math.min(1.25, eave + rise - sill - 0.78);
    if (!(h > 0.72) || roofY(zFace - depth) + 0.3 > sill + h) return b;
    for (const x of xs) if (Math.abs(x) + w / 2 < hw - 0.35) dormer(b, x, w, yBase, sill, h, zFace, depth);
    return b;
  }
  /// A flat roof with a parapet, coping and whatever plant the building needs.
  /// Roof clutter matters more than it looks: from a map camera the roof IS
  /// the building, and an empty rectangle is the thing that reads as a box.
  function flatRoof(b, hw, hd, y, para, plant) {
    b.slab(-hw, hw, -hd, hd, y, SLOT.GROUND, 0.86);
    b.box(-hw - 0.12, hw + 0.12, y, y + para, hd - 0.14, hd + 0.12, SLOT.WALL, 1.0);
    b.box(-hw - 0.12, hw + 0.12, y, y + para, -hd - 0.12, -hd + 0.14, SLOT.WALL, 0.86);
    b.box(-hw - 0.12, -hw + 0.14, y, y + para, -hd, hd, SLOT.WALL, 0.9);
    b.box(hw - 0.14, hw + 0.12, y, y + para, -hd, hd, SLOT.WALL, 1.0);
    b.box(-hw - 0.16, hw + 0.16, y + para, y + para + 0.09, -hd - 0.16, hd + 0.16, SLOT.TRIM, 1.06);
    if (plant !== false) {
      b.box(-hw * 0.42, -hw * 0.06, y, y + 2.5, -hd * 0.34, hd * 0.2, SLOT.WALL, 0.94);   // lift overrun / stair head
      b.box(hw * 0.16, hw * 0.6, y, y + 1.35, -hd * 0.5, -hd * 0.06, SLOT.METAL, 0.9);    // plant
      for (let i = 0; i < 4; i += 1) {
        b.box(hw * 0.2 + i * (hw * 0.09), hw * 0.24 + i * (hw * 0.09), y + 1.35, y + 1.5, -hd * 0.44, -hd * 0.12, SLOT.METAL, 1.05);
      }
      tube(b, -hw * 0.62, hd * 0.42, y, y + 1.8, 0.42, 0.42, b.segs(10, 7, 5), SLOT.METAL, 0.95);   // water tank
    }
    return b;
  }
  /// A stack with a corbelled head, lead flashing at the roof line and clay
  /// pots. Chimneys are the single cheapest thing that says "temperate, and
  /// not new": the kit puts one on every pitched roof and one per party wall
  /// on a terrace. The pots are smoothed and have a rim and a hollow throat,
  /// because a pot is the most obviously round object on the skyline and a
  /// hexagonal one gives the whole roof away.
  function chimney(b, cx, cz, base, top, w, d, pots) {
    b.box(cx - w / 2, cx + w / 2, base, top - 0.32, cz - d / 2, cz + d / 2, SLOT.WALL, 1.0);
    b.bevelBox(cx - w / 2 - 0.09, cx + w / 2 + 0.09, top - 0.32, top, cz - d / 2 - 0.09, cz + d / 2 + 0.09, SLOT.WALL, 1.06, 0.025);
    if (b.detail >= 2) {
      // Flashing: a lead apron and two soakers where the stack passes through
      // the slope. Thirty centimetres of grey at the base is what stops a
      // chimney reading as a box pushed into a roof.
      b.bevelBox(cx - w / 2 - 0.06, cx + w / 2 + 0.06, base + 0.02, base + 0.26, cz - d / 2 - 0.06, cz + d / 2 + 0.06, SLOT.METAL, 0.95, 0.02);
    }
    const n = pots || 2;
    for (let i = 0; i < n; i += 1) {
      const x = cx + (n === 1 ? 0 : (i / (n - 1) - 0.5) * (w - 0.4));
      tube(b, x, cz, top, top + 0.58, 0.145, 0.125, b.segs(9, 6, 4), SLOT.ROOF, 1.1);
      if (b.detail >= 2) {
        tube(b, x, cz, top + 0.58, top + 0.66, 0.155, 0.15, 9, SLOT.ROOF, 1.14);   // rim
        tube(b, x, cz, top + 0.5, top + 0.62, 0.09, 0.09, 9, SLOT.DARK, 0.7);      // throat
      }
    }
    return b;
  }
  /// Eaves gutter, brackets and downpipes. A half-round gutter on brackets and
  /// the vertical line of a pipe with a swan neck off the eaves are two of the
  /// half-dozen details that separate a modelled building from a massing
  /// study, and they read from further away than they have any right to
  /// because they are the only near-vertical lines on an elevation.
  function rainwater(b, hw, hd, eave) {
    const z = hd + 0.26, y = eave - 0.24;
    halfRound(b, -hw - 0.3, hw + 0.3, y, z, 0.11, b.segs(7, 5, 3), 1, SLOT.TRIM, 1.02);
    if (b.detail >= 2) {
      const n = Math.max(2, Math.round(hw / 1.5));
      for (let i = 0; i <= n; i += 1) {
        const x = -hw - 0.2 + ((hw + 0.2) * 2 * i) / n;
        b.box(x - 0.02, x + 0.02, y - 0.02, y + 0.13, z - 0.16, z + 0.13, SLOT.METAL, 0.95);
      }
    }
    for (const sx of [-1, 1]) {
      const px = sx * (hw - 0.18), seg = b.segs(8, 5, 4);
      if (b.detail >= 2) {
        b.box(px - 0.11, px + 0.11, y - 0.06, y + 0.16, z - 0.13, z + 0.12, SLOT.TRIM, 1.06);   // hopper
        tube(b, px, z, y - 0.34, y - 0.06, 0.055, 0.055, seg, SLOT.TRIM, 1.0);                  // swan neck
        tube(b, px, hd + 0.07, y - 0.52, y - 0.3, 0.055, 0.055, seg, SLOT.TRIM, 1.0);
        b.box(px - 0.055, px + 0.055, y - 0.36, y - 0.3, hd + 0.05, z, SLOT.TRIM, 0.98);
        tube(b, px, hd + 0.07, 0.16, y - 0.5, 0.055, 0.055, seg, SLOT.TRIM, 1.0);
        tube(b, px, hd + 0.07, 0.0, 0.16, 0.07, 0.06, seg, SLOT.TRIM, 0.95);                    // shoe
        for (let k = 1; k * 1.9 < y - 0.6; k += 1) {
          b.box(px - 0.085, px + 0.085, k * 1.9, k * 1.9 + 0.05, hd + 0.02, hd + 0.09, SLOT.METAL, 0.95);
        }
      } else {
        tube(b, px, hd + 0.07, 0, y - 0.1, 0.055, 0.055, seg, SLOT.TRIM, 1.0);
      }
    }
    return b;
  }

  // ------------------------------------------------------ small site pieces
  /// Boundary railings: two rails and a run of balusters. The balusters are
  /// the reason a front garden costs real triangles, and they are also the
  /// reason it reads as a front garden rather than a green rectangle.
  function railing(b, x0, x1, z, y, h, pitch) {
    // Baluster pitch doubles below close range. A railing is read as a texture
    // of verticals and half of them at twice the spacing reads the same at
    // card size, which matters because a block is full of railings.
    const n = Math.max(2, Math.round((x1 - x0) / ((pitch || 0.24) * (b.detail >= 2 ? 1 : 2))));
    b.box(x0, x1, y + h - 0.09, y + h, z - 0.03, z + 0.03, SLOT.METAL, 1.05);
    b.box(x0, x1, y + h * 0.42, y + h * 0.42 + 0.05, z - 0.025, z + 0.025, SLOT.METAL, 0.95);
    for (let i = 0; i <= n; i += 1) {
      const x = x0 + ((x1 - x0) * i) / n;
      b.box(x - 0.018, x + 0.018, y, y + h, z - 0.018, z + 0.018, SLOT.METAL, 1.0);
    }
    return b;
  }
  function lowWall(b, x0, x1, z0, z1, h) {
    b.box(x0, x1, 0, h, z0, z1, SLOT.WALL, 0.96);
    b.bevelBox(x0 - 0.05, x1 + 0.05, h, h + 0.09, z0 - 0.05, z1 + 0.05, SLOT.TRIM, 1.06, 0.02);   // coping
    return b;
  }
  /// A clipped hedge. A box of leaves reads as a box, so this is a box with
  /// every arris taken off and a few lobes settled on top of it: the silhouette
  /// a hedge actually has is soft on the corners and lumpy along the top, and
  /// those are the only two things about it the eye checks.
  function hedge(b, x0, x1, z0, z1, h, mat) {
    const m = mat == null ? 0.95 : mat;
    b.bevelBox(x0, x1, 0, h, z0, z1, SLOT.FOLIAGE, m, Math.min(0.16, (z1 - z0) / 2.6));
    if (b.detail < 2) return b;
    const run = x1 - x0, n = Math.max(1, Math.min(7, Math.round(run / 1.6)));
    const r = Math.min(0.42, (z1 - z0) * 0.55);
    for (let i = 0; i < n; i += 1) {
      const x = x0 + (run * (i + 0.5)) / n;
      lobe(b, x, h - r * 0.35, (z0 + z1) / 2, run / n * 0.52, r * 0.7, r, 6, 3, SLOT.FOLIAGE, m * (i % 2 ? 1.06 : 0.94));
    }
    return b;
  }
  /// A deciduous street tree. Tapered trunk with a root flare, three limbs
  /// into the crown, and a crown made of overlapping lobes rather than one
  /// blob — a lollipop is what a single mass always reads as, and the branches
  /// are what stop the canopy floating.
  ///
  /// Deterministic in its arguments alone: the crown arrangement is hashed
  /// from the height and the lean it was given, never from a counter or a
  /// clock, so the same tree in two towns is the same tree.
  function tree(b, cx, cz, h, lean, seg) {
    const s = b.segs(Math.max(7, seg || 7), 5, 4), r = h * 0.055;
    const salt = mix32(Math.round(h * 97) ^ Math.imul(Math.round(lean * 211) | 0, 0x2545f491));
    b.smoothed(() => {
      b.loft(oval(cx, cz, r * 1.35, r * 1.35, s), oval(cx, cz, r, r, s), 0, h * 0.09, SLOT.WALL, 0.5, false);
      b.loft(oval(cx, cz, r, r, s), oval(cx + lean * 0.3, cz, r * 0.62, r * 0.62, s), h * 0.09, h * 0.42, SLOT.WALL, 0.55, false);
    });
    if (b.detail >= 2) {
      // Three limbs off the crotch. Short, tapered and smoothed, and they cost
      // about seventy triangles for the whole tree.
      for (let i = 0; i < 3; i += 1) {
        const a = ((salt >>> (i * 5)) % 6) / 6 * Math.PI * 2;
        const dx = Math.cos(a) * h * 0.16, dz = Math.sin(a) * h * 0.16;
        b.smoothed(() => {
          b.loft(oval(cx + lean * 0.3, cz, r * 0.5, r * 0.5, 5),
            oval(cx + lean * 0.4 + dx, cz + dz, r * 0.22, r * 0.22, 5), h * 0.4, h * 0.62, SLOT.WALL, 0.6, false);
        });
      }
    }
    // THE CROWN STAYS INSIDE ITS OLD ENVELOPE. The outer lobes are offset by
    // 0.34 of the radius and are 0.6 of it across, so the widest reach is
    // 0.94 rad — narrower than the single blob this replaced. That is not
    // fussiness: a street tree sits a metre and a half off a lot boundary, and
    // a crown that grew 18% wider put a civic building's trees through the
    // school next door and failed the neighbour check rather than looking bad.
    const rad = h * 0.3, cy = h * 0.72, lobes = b.detail >= 2 ? 4 : 1;
    for (let i = 0; i < lobes; i += 1) {
      const a = ((salt >>> (i * 4 + 3)) % 8) / 8 * Math.PI * 2;
      const off = i === 0 ? 0 : rad * 0.34;
      lobe(b, cx + lean * 0.6 + Math.cos(a) * off, cy + (i === 0 ? rad * 0.1 : ((salt >>> (i * 3)) % 5 - 2) * rad * 0.16),
        cz + Math.sin(a) * off, rad * (i === 0 ? 0.94 : 0.6), rad * (i === 0 ? 0.82 : 0.56), rad * (i === 0 ? 0.9 : 0.58),
        s, b.detail >= 2 ? 4 : 3, SLOT.FOLIAGE, 0.86 + (i % 3) * 0.09);
    }
    return b;
  }
  /// A lamp column: cast base, tapered smooth column, swan neck and a lantern
  /// with the lens facing DOWN, which is the direction a lantern points.
  function lampColumn(b, cx, cz, out) {
    const s = b.segs(8, 6, 4);
    b.bevelBox(cx - 0.22, cx + 0.22, 0, 0.17, cz - 0.22, cz + 0.22, SLOT.GROUND, 1.0, 0.025);
    tube(b, cx, cz, 0.1, 0.62, 0.15, 0.115, s, SLOT.METAL, 0.9);
    tube(b, cx, cz, 0.62, 7.2, 0.11, 0.07, s, SLOT.METAL, 0.95);
    if (b.detail >= 2) {
      // Swan neck: two short tubes and a knuckle, which is what a 1990 column
      // has where the bracket leaves the shaft.
      tube(b, cx + out * 0.3, cz, 7.05, 7.24, 0.06, 0.06, 6, SLOT.METAL, 1.0);
      b.box(cx, cx + out * 1.1, 7.06, 7.2, cz - 0.05, cz + 0.05, SLOT.METAL, 1.0);
    } else {
      b.box(cx, cx + out * 1.1, 7.05, 7.2, cz - 0.05, cz + 0.05, SLOT.METAL, 1.0);
    }
    // A column with no bracket (out === 0) carries the lantern on its head
    // rather than nowhere: the civic forecourt asks for exactly that, and the
    // old code quietly emitted a zero-width lantern that every triangle of got
    // dropped as degenerate.
    const a0 = out ? out * 0.72 : -0.28, a1 = out ? out * 1.34 : 0.28;
    const l0 = cx + Math.min(a0, a1), l1 = cx + Math.max(a0, a1);
    b.loft([[l0, cz - 0.2], [l1, cz - 0.2], [l1, cz + 0.2], [l0, cz + 0.2]],
      [[l0 + 0.06, cz - 0.15], [l1 - 0.06, cz - 0.15], [l1 - 0.06, cz + 0.15], [l0 + 0.06, cz + 0.15]],
      6.72, 7.08, SLOT.METAL, 1.05);
    b.soffit(l0 + 0.05, l1 - 0.05, cz - 0.16, cz + 0.16, 6.715, SLOT.GLASS, 1.14);
    return b;
  }
  function bench(b, cx, cz, yaw) {
    b.push(yaw, cx, 0, cz);
    for (const x of [-0.7, 0.7]) {
      b.box(x - 0.06, x + 0.06, 0, 0.44, -0.25, 0.25, SLOT.METAL, 0.95);
      b.box(x - 0.05, x + 0.05, 0.44, 0.92, -0.26, -0.14, SLOT.METAL, 0.95);
    }
    for (let i = 0; i < 3; i += 1) b.box(-0.85, 0.85, 0.44, 0.5, -0.24 + i * 0.16, -0.14 + i * 0.16, SLOT.TRIM, 1.02);
    for (let i = 0; i < 3; i += 1) b.box(-0.85, 0.85, 0.55 + i * 0.13, 0.65 + i * 0.13, -0.27, -0.21, SLOT.TRIM, 1.0);
    b.pop();
    return b;
  }
  function bollard(b, cx, cz) {
    const s = b.segs(9, 6, 4);
    tube(b, cx, cz, 0, 0.08, 0.115, 0.1, s, SLOT.METAL, 0.85);        // spread foot
    tube(b, cx, cz, 0.08, 0.9, 0.09, 0.08, s, SLOT.METAL, 1.0);
    tube(b, cx, cz, 0.9, 0.96, 0.105, 0.09, s, SLOT.METAL, 1.06);     // collar
    if (b.detail >= 2) lobe(b, cx, 0.96, cz, 0.09, 0.09, 0.09, s, 3, SLOT.METAL, 1.1);
    else tube(b, cx, cz, 0.96, 1.02, 0.09, 0.04, s, SLOT.METAL, 1.08);
    return b;
  }
  function litterBin(b, cx, cz) {
    const s = b.segs(10, 7, 5);
    tube(b, cx, cz, 0, 0.14, 0.2, 0.24, s, SLOT.METAL, 0.8);
    tube(b, cx, cz, 0.14, 0.88, 0.24, 0.26, s, SLOT.METAL, 0.9);
    tube(b, cx, cz, 0.88, 0.98, 0.28, 0.27, s, SLOT.DARK, 1.0);
    if (b.detail >= 2) {
      tube(b, cx, cz, 0.98, 1.06, 0.27, 0.2, s, SLOT.DARK, 1.05);
      for (let i = 0; i < 3; i += 1) {                                 // banding
        b.cyl(cx, cz, 0.3 + i * 0.2, 0.34 + i * 0.2, 0.265, 0.265, s, SLOT.METAL, 1.06, false);
      }
    }
    return b;
  }
  function roadSign(b, cx, cz, h) {
    tube(b, cx, cz, 0, h, 0.055, 0.05, b.segs(8, 6, 4), SLOT.METAL, 1.0);
    b.bevelBox(cx - 0.34, cx + 0.34, h - 0.5, h, cz - 0.035, cz + 0.035, SLOT.TRIM, 1.08, 0.02);
    return b;
  }
  /// A wheel with its axle along Z, which is across a car whose length runs
  /// along X: tread, two walls and a dished rim. Round things are where smooth
  /// normals earn their keep, and a wheel is the roundest thing in a street.
  function wheel(b, cx, cy, cz, r, halfW, seg, slot, mat) {
    const n = Math.max(5, seg | 0);
    const P = (a, rr) => [cx + Math.cos(a) * rr, cy + Math.sin(a) * rr];
    b.smoothed(() => {
      for (let i = 0; i < n; i += 1) {
        const p0 = P((i / n) * Math.PI * 2, r), p1 = P(((i + 1) / n) * Math.PI * 2, r);
        b.quad([p0[0], p0[1], cz + halfW], [p0[0], p0[1], cz - halfW],
          [p1[0], p1[1], cz - halfW], [p1[0], p1[1], cz + halfW], slot, mat);
      }
    });
    if (b.detail < 2) return b;
    const hub = r * 0.56;
    for (const sz of [-1, 1]) {
      const z = cz + sz * halfW;
      for (let i = 0; i < n; i += 1) {
        const a0 = (i / n) * Math.PI * 2, a1 = ((i + 1) / n) * Math.PI * 2;
        const o0 = P(a0, r), o1 = P(a1, r), i0 = P(a0, hub), i1 = P(a1, hub);
        if (sz > 0) b.quad([o0[0], o0[1], z], [o1[0], o1[1], z], [i1[0], i1[1], z], [i0[0], i0[1], z], slot, mat * 0.88);
        else b.quad([i0[0], i0[1], z], [i1[0], i1[1], z], [o1[0], o1[1], z], [o0[0], o0[1], z], slot, mat * 0.88);
        // The rim inside the tyre wall, a shade brighter and set in.
        const rz = z - sz * 0.012;
        if (sz > 0) b.tri([i0[0], i0[1], rz], [i1[0], i1[1], rz], [cx, cy, rz], SLOT.METAL, 1.0);
        else b.tri([i1[0], i1[1], rz], [i0[0], i0[1], rz], [cx, cy, rz], SLOT.METAL, 1.0);
      }
    }
    return b;
  }
  /// A parked car. Generic three-box saloon of the period: no badge, no grille
  /// pattern, no maker. It is here because an empty kerb is the single loudest
  /// way a generated street says nobody lives there, and because a car is a
  /// known-size object the eye measures the whole street against.
  function parkedCar(b, cx, cz, yaw, paint, len) {
    const L = len == null ? 4.1 : len, hl = L / 2, hwid = 0.82, s = b.segs(12, 7, 5);
    b.push(yaw, cx, 0, cz);
    if (b.detail < 1) {
      b.box(-hl, hl, 0.2, 1.32, -hwid, hwid, paint, 1.0);
      b.pop();
      return b;
    }
    if (b.detail < 2) {
      b.box(-hl, hl, 0.28, 0.92, -hwid, hwid, paint, 1.0);
      b.box(-hl * 0.42, hl * 0.5, 0.92, 1.38, -hwid + 0.09, hwid - 0.09, SLOT.GLASS, 0.95);
      for (const sx of [-1, 1]) for (const sz of [-1, 1]) {
        b.box(sx * hl * 0.62 - 0.28, sx * hl * 0.62 + 0.28, 0.02, 0.6, sz * (hwid - 0.02) - 0.11, sz * (hwid - 0.02) + 0.11, SLOT.DARK, 0.9);
      }
      b.pop();
      return b;
    }
    // Lower body, bonnet and boot as three chamfered masses. The chamfers are
    // the point: a car with square arrises reads as a brick, and 20 mm off
    // every edge is what gives it the long highlight down the flank.
    b.bevelBox(-hl, hl, 0.34, 0.9, -hwid, hwid, paint, 1.0, 0.055);
    b.bevelBox(-hl, -hl * 0.38, 0.86, 1.03, -hwid + 0.03, hwid - 0.03, paint, 1.05, 0.045);   // bonnet
    b.bevelBox(hl * 0.46, hl, 0.86, 1.06, -hwid + 0.03, hwid - 0.03, paint, 0.98, 0.045);     // boot
    // Cabin, tapered in on all four sides, with the glazing set inside it.
    b.loft([[-hl * 0.4, -hwid + 0.04], [hl * 0.48, -hwid + 0.04], [hl * 0.48, hwid - 0.04], [-hl * 0.4, hwid - 0.04]],
      [[-hl * 0.2, -hwid + 0.2], [hl * 0.34, -hwid + 0.2], [hl * 0.34, hwid - 0.2], [-hl * 0.2, hwid - 0.2]],
      0.9, 1.45, paint, 1.06);
    for (const sz of [-1, 1]) {
      // Wound per side. A quad mirrored across an axis reverses, and the near
      // side's window would otherwise face into the car.
      const p = [[-hl * 0.34, 0.98, sz * (hwid - 0.02)], [hl * 0.42, 0.98, sz * (hwid - 0.02)],
        [hl * 0.3, 1.4, sz * (hwid - 0.17)], [-hl * 0.16, 1.4, sz * (hwid - 0.17)]];
      if (sz > 0) b.quad(p[0], p[1], p[2], p[3], SLOT.GLASS, 1.0);
      else b.quad(p[3], p[2], p[1], p[0], SLOT.GLASS, 0.86);
    }
    b.quad([-hl * 0.38, 0.95, -hwid + 0.05], [-hl * 0.38, 0.95, hwid - 0.05],
      [-hl * 0.19, 1.42, hwid - 0.19], [-hl * 0.19, 1.42, -hwid + 0.19], SLOT.GLASS, 1.05);   // windscreen
    b.quad([hl * 0.46, 0.95, hwid - 0.05], [hl * 0.46, 0.95, -hwid + 0.05],
      [hl * 0.33, 1.42, -hwid + 0.19], [hl * 0.33, 1.42, hwid - 0.19], SLOT.GLASS, 0.9);      // backlight
    for (const sx of [-1, 1]) {
      // Bumper, lamps and an arch liner at each end.
      b.bevelBox(sx * hl - (sx > 0 ? 0.14 : 0), sx * hl + (sx > 0 ? 0 : 0.14), 0.42, 0.62, -hwid - 0.02, hwid + 0.02, SLOT.DARK, 1.0, 0.02);
      for (const sz of [-1, 1]) {
        b.box(sx * hl - 0.06, sx * hl + 0.06, 0.68, 0.86, sz * (hwid - 0.42), sz * (hwid - 0.12),
          sx < 0 ? SLOT.GLASS : SLOT.CAR_A, sx < 0 ? 1.18 : 1.1);
        wheel(b, sx * hl * 0.62, 0.32, sz * (hwid - 0.06), 0.32, 0.1, s, SLOT.DARK, 1.0);
        b.box(sx * hl * 0.62 - 0.36, sx * hl * 0.62 + 0.36, 0.34, 0.68, sz * (hwid - 0.12) - 0.02, sz * (hwid - 0.12) + 0.02, SLOT.DARK, 0.7);
      }
    }
    for (const sz of [-1, 1]) {                                                     // mirrors
      b.box(-hl * 0.3, -hl * 0.22, 1.0, 1.12, sz * (hwid + 0.02) - 0.05, sz * (hwid + 0.02) + 0.05, SLOT.DARK, 0.95);
    }
    b.pop();
    return b;
  }

  /// A bus shelter and a glazed kiosk. Both are GENERIC: they carry no
  /// operator, no livery and no claim to be any particular manufactured unit,
  /// which is roadmap section 3's line on original game art.
  function busShelter(b, cx, cz, yaw) {
    b.push(yaw, cx, 0, cz);
    b.box(-2.0, 2.0, 0, 0.08, -0.7, 0.7, SLOT.GROUND, 1.0);
    for (const x of [-1.94, 1.94]) b.box(x - 0.06, x + 0.06, 0.08, 2.4, -0.66, -0.54, SLOT.METAL, 1.0);
    b.box(-2.0, 2.0, 0.4, 2.3, -0.62, -0.58, SLOT.GLASS, 1.0);
    b.box(-2.0, 2.0, 2.3, 2.42, -0.75, 0.72, SLOT.METAL, 1.05);
    b.box(-2.0, -1.4, 0.4, 2.3, -0.62, 0.66, SLOT.GLASS, 0.95);
    b.box(-0.9, 0.9, 0.42, 0.5, -0.55, -0.1, SLOT.TRIM, 1.0);
    b.pop();
    return b;
  }
  function kiosk(b, cx, cz, yaw) {
    b.push(yaw, cx, 0, cz);
    b.box(-0.6, 0.6, 0, 0.14, -0.6, 0.6, SLOT.GROUND, 1.0);
    b.box(-0.6, 0.6, 0.14, 2.55, -0.6, 0.6, SLOT.TRIM, 0.95);
    for (const f of [[0.58, 0.62], [-0.62, -0.58]]) b.box(-0.48, 0.48, 0.55, 2.25, f[0], f[1], SLOT.GLASS, 1.0);
    b.box(-0.7, 0.7, 2.55, 2.78, -0.7, 0.7, SLOT.TRIM, 1.08);
    b.pop();
    return b;
  }
  function garage(b, cx, cz, yaw, w) {
    b.push(yaw, cx, 0, cz);
    const hw = (w || 3.0) / 2;
    b.box(-hw, hw, 0, 2.35, -2.7, 2.7, SLOT.WALL, 0.95);
    b.box(-hw - 0.1, hw + 0.1, 2.35, 2.52, -2.8, 2.8, SLOT.ROOF, 1.05);
    b.box(-hw + 0.22, hw - 0.22, 0.05, 2.05, 2.62, 2.72, SLOT.METAL, 1.0);
    for (let i = 0; i < 6; i += 1) b.box(-hw + 0.22, hw - 0.22, 0.2 + i * 0.3, 0.25 + i * 0.3, 2.72, 2.75, SLOT.METAL, 0.88);
    b.pop();
    return b;
  }

  // ------------------------------------------------------------------ kits
  // Fifteen kinds, the set roadmap section G names. Each one owns a lot, not
  // just a building: the mesh covers its garden, forecourt or yard out to the
  // lot boundary, so a block is laid out by butting lots together and the
  // ground between the buildings is already right.
  //
  // Dimensions are plausible temperate massing in metres and nothing more. No
  // number here is a measurement of anything that exists — see the file
  // header, and roadmap section 3.

  // The pieces every kind shares are the OPENING and the ROOF, not the
  // elevation: a terrace, a town hall and a hospital put their windows in
  // places that have nothing in common, and an elevation function general
  // enough for all three ended up longer than the fifteen calls it replaced.
  // So `sash`, `ribbon`, `frontDoor`, `shell` and the roof functions above are
  // the shared kit, and each kind lays them out itself.

  /// Masonry shell with plinth, floor string courses and an eaves cornice.
  /// The courses cost twelve triangles each and they are what stops a
  /// four-storey wall reading as one tall blank slab.
  function shell(b, hw, hd, h, courses, courseAt) {
    b.wall(-hw, hw, 0, h, -hd, hd, SLOT.WALL, 1.0);
    // Plinth and string courses get their arris taken off. They are the two
    // horizontal lines a masonry elevation has, they run the whole width of
    // the building, and a chamfer is what makes them read as a projecting
    // course rather than as a change of paint.
    b.bevelBox(-hw - 0.14, hw + 0.14, 0, 0.62, -hd - 0.14, hd + 0.14, SLOT.TRIM, 0.9, 0.028);
    for (let i = 1; i <= (courses || 0); i += 1) {
      const y = courseAt(i);
      b.bevelBox(-hw - 0.08, hw + 0.08, y - 0.09, y + 0.09, -hd - 0.08, hd + 0.08, SLOT.TRIM, 1.04, 0.022);
    }
    return b;
  }
  function grass(b, x0, x1, z0, z1, y) { return b.slab(x0, x1, z0, z1, y == null ? 0.11 : y, SLOT.FOLIAGE, 0.82); }
  function paving(b, x0, x1, z0, z1, y) { return b.slab(x0, x1, z0, z1, y == null ? 0.1 : y, SLOT.GROUND, 1.02); }

  const KINDS = {};

  // --------------------------------------------------------------- house
  KINDS.house = {
    label: "Detached house",
    widths: [9.0, 10.4, 11.8], depth: 20.0, storeys: [2, 2, 3],
    ground: "grass",
    close(b, p) {
      const hw = p.w / 2, hd = 4.9, h = p.storeys * STOREY, back = -p.d / 2 + 5.6;
      const zc = back + hd;                                     // body sits at the rear of the lot
      grass(b, -p.w / 2, p.w / 2, -p.d / 2, p.d / 2);
      b.push(0, 0, 0, zc);
      shell(b, hw, hd, h, p.storeys - 1, (i) => i * STOREY);
      gableRoof(b, hw, hd, h, hd * 0.82);
      chimney(b, -hw + 0.5, 0, h, h + hd * 0.82 + 1.05, 1.05, 0.75, 2);
      rainwater(b, hw, hd, h);
      // One dormer in the front slope. A temperate house of this age with a
      // roof this steep has a room in it, and the dormer is what says so.
      roofDormers(b, hw, hd, h, hd * 0.82, 0.32, [hw - 3.4], 1.45);
      // Front elevation: a canted bay on the ground floor, sashes above.
      const bayW = Math.min(3.1, p.w * 0.32), bx = -hw + bayW * 0.62 + 0.5;
      b.wall(bx - bayW / 2, bx + bayW / 2, 0, 2.75, hd, hd + 1.05, SLOT.WALL, 1.02);
      b.box(bx - bayW / 2 - 0.14, bx + bayW / 2 + 0.14, 2.75, 2.95, hd - 0.05, hd + 1.2, SLOT.ROOF, 1.08);
      sash(b, bx, 0.72, bayW - 0.9, 1.65, hd + 1.05, 3, 2);
      sash(b, bx - bayW / 2 - 0.02, 0.72, 0.7, 1.65, hd + 0.5, 1, 2);
      frontDoor(b, hw - 1.5, hd, 2.1);
      for (let s = 1; s < p.storeys; s += 1) {
        sash(b, bx, s * STOREY + 0.85, 1.35, 1.55, hd, 2, 3);
        sash(b, hw - 1.5, s * STOREY + 0.85, 1.15, 1.55, hd, 2, 3);
      }
      // Rear and sides.
      b.push(2, 0, 0, 0);
      for (let s = 0; s < p.storeys; s += 1) {
        sash(b, -hw + 1.5, s * STOREY + 0.85, 1.2, 1.55, hd, 2, 3, false);
        sash(b, hw - 1.6, s * STOREY + 0.85, 1.5, 1.55, hd, 2, 3, false);
      }
      b.pop();
      for (const yaw of [1, 3]) {
        b.push(yaw, 0, 0, 0);
        for (let s = 0; s < p.storeys; s += 1) sash(b, 0.6, s * STOREY + 1.0, 0.95, 1.35, hw, 2, 2, false);
        b.pop();
      }
      // Single-storey rear addition — every temperate house of this age has one.
      b.box(-hw + 0.4, hw - 2.4, 0, 2.5, -hd - 2.9, -hd + 0.1, SLOT.WALL, 0.95);
      b.box(-hw + 0.3, hw - 2.3, 2.5, 2.72, -hd - 3.1, -hd + 0.1, SLOT.ROOF, 1.06);
      b.pop();
      // Front garden: dwarf wall, railings, gate piers and a path to the door.
      const fz = p.d / 2;
      lowWall(b, -p.w / 2, -0.9, fz - 0.4, fz - 0.05, 0.55);
      lowWall(b, 0.9, p.w / 2, fz - 0.4, fz - 0.05, 0.55);
      railing(b, -p.w / 2 + 0.1, -1.0, fz - 0.22, 0.63, 0.55, 0.26);
      railing(b, 1.0, p.w / 2 - 0.1, fz - 0.22, 0.63, 0.55, 0.26);
      for (const x of [-1.0, 1.0]) b.box(x - 0.19, x + 0.19, 0, 1.25, fz - 0.5, fz - 0.12, SLOT.WALL, 1.0);
      paving(b, -0.85, 0.85, zc + hd, fz - 0.2);
      // Drive down the side of the house, with the car that goes on it. A
      // driveway with nothing parked on it reads as a slab of grey; a car is
      // also the object that tells the eye how big the house is.
      paving(b, hw - 2.9, hw - 0.5, -p.d / 2 + 3.2, fz - 0.6);
      parkedCar(b, hw - 1.7, fz - 3.4, 1, SLOT.CAR_B, 4.0);
      hedge(b, -p.w / 2, -1.6, zc + hd + 1.2, zc + hd + 1.9, 1.15);
      tree(b, -p.w / 2 + 1.5, fz - 3.2, 5.4, 0.25, 6);
      railing(b, -0.95, 0.95, fz - 0.3, 0, 1.05, 0.2);                        // gate
      b.box(-p.w / 2 + 0.8, -p.w / 2 + 3.0, 0, 2.05, -p.d / 2 + 0.8, -p.d / 2 + 2.6, SLOT.WALL, 0.94);
      b.box(-p.w / 2 + 0.65, -p.w / 2 + 3.15, 2.05, 2.22, -p.d / 2 + 0.65, -p.d / 2 + 2.75, SLOT.ROOF, 1.05);
      b.box(-p.w / 2 + 1.5, -p.w / 2 + 2.3, 0, 1.85, -p.d / 2 + 2.55, -p.d / 2 + 2.64, SLOT.DARK, 1.0);
      litterBin(b, p.w / 2 - 1.2, -p.d / 2 + 1.4);
      return "Original game art: representative temperate detached house, brick or render with a pitched roof, front garden and rear addition. Massing only; not a measured building.";
    },
    massing(p) {
      const hd = 4.9, zc = -p.d / 2 + 5.6 + hd;
      return [{ x0: -p.w / 2, x1: p.w / 2, z0: zc - hd, z1: zc + hd, h: p.storeys * STOREY, roof: "gable",
        rise: hd * 0.82, bands: p.storeys, chimneys: [-p.w / 2 + 0.5] }];
    },
  };

  // ----------------------------------------------------------- row_house
  KINDS.row_house = {
    label: "Row houses",
    widths: [5.9 * 4, 5.9 * 5, 5.9 * 6], depth: 17.0, storeys: [2, 2, 3],
    ground: "grass",
    close(b, p) {
      const units = Math.round(p.w / 5.9), uw = p.w / units;
      const hw = p.w / 2, hd = 4.4, h = p.storeys * STOREY;
      const zc = -p.d / 2 + 5.0 + hd;
      grass(b, -hw, hw, -p.d / 2, p.d / 2);
      b.push(0, 0, 0, zc);
      shell(b, hw, hd, h, p.storeys - 1, (i) => i * STOREY);
      gableRoof(b, hw, hd, h, hd * 0.8, 0.22);
      roofDormers(b, hw, hd, h, hd * 0.8, 0.22,
        (function () { const xs = []; for (let u = 0; u < units; u += 1) xs.push(-hw + uw * (u + 0.5)); return xs; })(), 1.25);
      // A stack on every party wall and both ends: the signature of a terrace.
      for (let i = 0; i <= units; i += 1) {
        const inset = i === 0 ? 0.62 : i === units ? -0.62 : 0;
        chimney(b, -hw + uw * i + inset, 0, h, h + hd * 0.8 + 0.95, 0.95, 0.7, i === 0 || i === units ? 2 : 3);
      }
      rainwater(b, hw, hd, h);
      for (let u = 0; u < units; u += 1) {
        const cx = -hw + uw * (u + 0.5);
        frontDoor(b, cx + uw * 0.28, hd, 2.05);
        sash(b, cx - uw * 0.2, 0.85, uw * 0.42, 1.6, hd, 2, 3);
        for (let s = 1; s < p.storeys; s += 1) {
          sash(b, cx - uw * 0.2, s * STOREY + 0.85, uw * 0.4, 1.5, hd, 2, 3);
          sash(b, cx + uw * 0.28, s * STOREY + 0.85, uw * 0.26, 1.5, hd, 2, 2);
        }
      }
      b.push(2, 0, 0, 0);
      for (let u = 0; u < units; u += 1) {
        const cx = -hw + uw * (u + 0.5);
        // The ground-floor back window sits BESIDE the addition, not behind it,
        // which is both where a terrace actually puts it and the only place it
        // can be seen from. The upper floors have the whole wall to themselves.
        for (let s = 1; s < p.storeys; s += 1) sash(b, cx, s * STOREY + 0.9, uw * 0.38, 1.45, hd, 2, 2, false);
        sash(b, cx + uw * 0.32, 0.9, uw * 0.24, 1.35, hd, 2, 2, false);
        // THE BACK ADDITION GOES OUT THE BACK. This frame is pushed at yaw 2,
        // which mirrors BOTH x and z, so the rear wall the sashes above sit in
        // is at +hd here and everything behind the house is at z greater than
        // that. Authored with -hd the outrigger landed on the other side of the
        // building entirely: 2.4 m of blank brick standing in the FRONT garden
        // of every unit, across the ground-floor sashes and the front doors and
        // through the dwarf wall. Measured on the street elevation before the
        // repair, 6 of 32 ground-floor panes could be seen from outside; the
        // upper floors, which nothing stood in front of, were 36 of 36.
        b.box(cx - uw * 0.3, cx + uw * 0.16, 0, 2.4, hd - 0.1, hd + 2.4, SLOT.WALL, 0.94);
        b.box(cx - uw * 0.34, cx + uw * 0.2, 2.4, 2.6, hd - 0.1, hd + 2.55, SLOT.ROOF, 1.05);
      }
      b.pop();
      b.pop();
      // Forecourt: a shallow strip, dwarf wall and railings, no front garden.
      const fz = p.d / 2;
      paving(b, -hw, hw, zc + hd, fz - 1.0);
      lowWall(b, -hw, hw, fz - 1.2, fz - 0.85, 0.5);
      railing(b, -hw, hw, fz - 1.02, 0.58, 0.5, 0.24);
      for (let u = 0; u < units; u += 1) grass(b, -hw + uw * u + 0.4, -hw + uw * (u + 1) - 0.4, -p.d / 2 + 0.5, zc - hd - 3.0);
      // Two cars nose to tail at the kerb. A terrace with an empty frontage is
      // the single clearest tell that a street was generated rather than lived
      // in, and this is four hundred triangles against that.
      parkedCar(b, -hw + uw * 0.9, fz - 1.9, 0, SLOT.CAR_A, 4.0);
      if (units > 3) parkedCar(b, -hw + uw * (units - 1.1), fz - 1.9, 0, SLOT.CAR_C, 4.2);
      return "Original game art: representative temperate terrace, continuous eaves and party-wall stacks over " +
        units + " dwellings. Massing only; not a measured street.";
    },
    massing(p) {
      const units = Math.round(p.w / 5.9), uw = p.w / units, hw = p.w / 2, hd = 4.4;
      const zc = -p.d / 2 + 5.0 + hd, ch = [];
      for (let i = 0; i <= units; i += 1) ch.push(-hw + uw * i);
      return [{ x0: -hw, x1: hw, z0: zc - hd, z1: zc + hd, h: p.storeys * STOREY, roof: "gable",
        rise: hd * 0.8, bands: p.storeys, chimneys: ch }];
    },
  };

  // ------------------------------------------------------ low_apartment
  KINDS.low_apartment = {
    label: "Low-rise apartments",
    widths: [16.5, 19.0, 22.0], depth: 21.0, storeys: [3, 3, 4],
    ground: "grass",
    close(b, p) {
      const hw = p.w / 2, hd = 6.2, h = p.storeys * STOREY, zc = -p.d / 2 + 6.4 + hd;
      grass(b, -hw, hw, -p.d / 2, p.d / 2);
      b.push(0, 0, 0, zc);
      shell(b, hw, hd, h, p.storeys - 1, (i) => i * STOREY);
      hipRoof(b, hw, hd, h, hd * 0.5);
      rainwater(b, hw, hd, h);
      // Communal entrance in a shallow projecting bay, flats either side.
      b.wall(-1.7, 1.7, 0, STOREY - 0.2, hd, hd + 0.9, SLOT.WALL, 1.03);
      b.box(-1.95, 1.95, STOREY - 0.2, STOREY, hd - 0.05, hd + 1.1, SLOT.TRIM, 1.08);
      frontDoor(b, 0, hd + 0.9, 2.2);
      b.box(-1.4, 1.4, STOREY + 0.4, h - 0.4, hd + 0.02, hd + 0.1, SLOT.GLASS, 0.9);
      for (let s = 0; s < p.storeys; s += 1) {
        const y = s * STOREY;
        for (const cx of [-hw + 2.4, -hw + 5.2, hw - 5.2, hw - 2.4]) sash(b, cx, y + 0.95, 1.5, 1.6, hd, 2, 2);
        if (s > 0) {
          for (const cx of [-hw + 3.8, hw - 3.8]) {
            b.box(cx - 1.5, cx + 1.5, y - 0.16, y + 0.02, hd, hd + 1.25, SLOT.TRIM, 1.06);
            railing(b, cx - 1.5, cx + 1.5, hd + 1.2, y + 0.02, 1.05, 0.22);
            b.box(cx - 1.5, cx - 1.44, y + 0.02, y + 1.07, hd, hd + 1.2, SLOT.METAL, 0.95);
            b.box(cx + 1.44, cx + 1.5, y + 0.02, y + 1.07, hd, hd + 1.2, SLOT.METAL, 0.95);
          }
        }
      }
      b.push(2, 0, 0, 0);
      for (let s = 0; s < p.storeys; s += 1) {
        for (const cx of [-hw + 3.0, -hw + 6.4, hw - 6.4, hw - 3.0]) sash(b, cx, s * STOREY + 0.95, 1.35, 1.55, hd, 2, 2, false);
      }
      b.pop();
      for (const yaw of [1, 3]) {
        b.push(yaw, 0, 0, 0);
        for (let s = 0; s < p.storeys; s += 1) sash(b, 0, s * STOREY + 1.0, 1.1, 1.4, hw, 2, 2, false);
        b.pop();
      }
      b.pop();
      const fz = p.d / 2;
      paving(b, -2.6, 2.6, zc + hd, fz - 0.8);
      lowWall(b, -hw, hw, fz - 1.0, fz - 0.65, 0.6);
      for (const x of [-hw + 3.0, hw - 3.0]) tree(b, x, fz - 3.4, 6.2, -0.2, 6);
      paving(b, -hw, hw, -p.d / 2 + 0.6, zc - hd - 0.4);
      return "Original game art: representative temperate low-rise flats, hipped roof, communal stair and balconies. Massing only.";
    },
    massing(p) {
      const hd = 6.2, zc = -p.d / 2 + 6.4 + hd;
      return [{ x0: -p.w / 2, x1: p.w / 2, z0: zc - hd, z1: zc + hd, h: p.storeys * STOREY, roof: "hip",
        rise: hd * 0.5, bands: p.storeys }];
    },
  };

  // ------------------------------------------------------ mid_apartment
  KINDS.mid_apartment = {
    label: "Mid-rise apartments",
    widths: [21.0, 24.0, 27.0], depth: 24.0, storeys: [5, 6, 7],
    ground: "paved",
    close(b, p) {
      const hw = p.w / 2, hd = 7.0, h = p.storeys * STOREY, zc = -p.d / 2 + 7.5 + hd;
      paving(b, -hw, hw, -p.d / 2, p.d / 2);
      b.push(0, 0, 0, zc);
      shell(b, hw, hd, h, 0, () => 0);
      flatRoof(b, hw, hd, h, 1.0, true);
      // Stair and lift core expressed on the elevation — the 1970s temperate
      // block's one honest piece of composition.
      b.wall(-2.2, 2.2, 0, h + 1.3, hd, hd + 1.1, SLOT.WALL, 0.94);
      b.box(-2.45, 2.45, h + 1.3, h + 1.45, hd - 0.05, hd + 1.3, SLOT.TRIM, 1.06);
      for (let s = 0; s < p.storeys; s += 1) b.box(-1.6, 1.6, s * STOREY + 0.6, s * STOREY + 2.5, hd + 1.05, hd + 1.14, SLOT.GLASS, 0.95);
      frontDoor(b, 0, hd + 1.1, 2.3);
      for (let s = 0; s < p.storeys; s += 1) {
        const y = s * STOREY;
        ribbon(b, -hw + 1.1, -2.7, y + 0.95, 1.65, hd, 1.7);
        ribbon(b, 2.7, hw - 1.1, y + 0.95, 1.65, hd, 1.7);
        b.box(-hw + 0.9, -2.6, y - 0.18, y, hd, hd + 1.3, SLOT.TRIM, 1.06);
        b.box(2.6, hw - 0.9, y - 0.18, y, hd, hd + 1.3, SLOT.TRIM, 1.06);
        railing(b, -hw + 0.9, -2.6, hd + 1.25, y, 1.05, 0.28);
        railing(b, 2.6, hw - 0.9, hd + 1.25, y, 1.05, 0.28);
      }
      b.push(2, 0, 0, 0);
      for (let s = 0; s < p.storeys; s += 1) ribbon(b, -hw + 1.2, hw - 1.2, s * STOREY + 1.0, 1.5, hd, 2.0);
      b.pop();
      for (const yaw of [1, 3]) {
        b.push(yaw, 0, 0, 0);
        for (let s = 0; s < p.storeys; s += 1) sash(b, 0, s * STOREY + 1.0, 1.2, 1.45, hw, 2, 2, false);
        b.pop();
      }
      b.pop();
      const fz = p.d / 2;
      paving(b, -3.0, 3.0, zc + hd + 1.1, fz - 0.5);
      grass(b, -hw, -3.4, zc + hd + 1.2, fz - 1.0);
      grass(b, 3.4, hw, zc + hd + 1.2, fz - 1.0);
      for (const x of [-hw + 2.6, hw - 2.6]) tree(b, x, fz - 3.0, 6.8, 0.2, 6);
      return "Original game art: representative temperate mid-rise flats, expressed stair core and access balconies. Massing only.";
    },
    massing(p) {
      const hd = 7.0, zc = -p.d / 2 + 7.5 + hd;
      return [{ x0: -p.w / 2, x1: p.w / 2, z0: zc - hd, z1: zc + hd, h: p.storeys * STOREY, roof: "flat",
        para: 1.0, bands: 4, plant: true }];
    },
  };

  // ----------------------------------------------------- high_apartment
  KINDS.high_apartment = {
    label: "High-rise apartments",
    widths: [22.0, 26.0], depth: 34.0, storeys: [11, 14, 16],
    ground: "grass",
    close(b, p) {
      const hw = p.w / 2, hd = 8.5, h = p.storeys * STOREY, zc = -p.d / 2 + 11.0 + hd;
      grass(b, -p.w / 2, p.w / 2, -p.d / 2, p.d / 2);
      b.push(0, 0, 0, zc);
      shell(b, hw, hd, h, 0, () => 0);
      flatRoof(b, hw, hd, h, 1.2, true);
      b.box(-2.6, 2.6, 0, h + 3.4, -hd - 1.6, -hd + 0.2, SLOT.WALL, 0.92);      // stair and tank tower
      b.box(-2.85, 2.85, h + 3.4, h + 3.6, -hd - 1.85, -hd + 0.2, SLOT.TRIM, 1.06);
      b.wall(-hw - 0.3, hw + 0.3, 0, TALL_STOREY, hd - 0.2, hd + 0.9, SLOT.WALL, 1.02);  // ground-floor podium
      b.box(-hw - 0.45, hw + 0.45, TALL_STOREY, TALL_STOREY + 0.22, hd - 0.3, hd + 1.15, SLOT.TRIM, 1.08);
      ribbon(b, -hw + 0.6, -2.4, 0.9, 2.2, hd + 0.9, 1.9);
      ribbon(b, 2.4, hw - 0.6, 0.9, 2.2, hd + 0.9, 1.9);
      frontDoor(b, 0, hd + 0.9, 2.4);
      // Bands of glazing and slab-edge balconies. Sashes would cost four times
      // as much here and would also be the wrong century.
      for (let s = 1; s < p.storeys; s += 1) {
        const y = s * STOREY + (TALL_STOREY - STOREY);
        if (y + 2.0 > h) break;
        ribbon(b, -hw + 0.8, hw - 0.8, y + 0.8, 1.55, hd, 2.4);
        b.box(-hw - 0.25, hw + 0.25, y - 0.2, y, hd, hd + 1.2, SLOT.TRIM, 1.06);
        b.box(-hw - 0.25, hw + 0.25, y, y + 1.02, hd + 1.06, hd + 1.2, SLOT.WALL, 1.02);
        b.box(-hw - 0.3, hw + 0.3, y + 1.02, y + 1.12, hd + 1.0, hd + 1.26, SLOT.TRIM, 1.08);
      }
      b.push(2, 0, 0, 0);
      for (let s = 0; s < p.storeys; s += 1) {
        const y = s * STOREY;
        if (y + 2.0 > h) break;
        ribbon(b, -hw + 3.2, hw - 3.2, y + 0.9, 1.35, hd, 2.6);
      }
      b.pop();
      for (const yaw of [1, 3]) {
        b.push(yaw, 0, 0, 0);
        for (let s = 0; s < p.storeys; s += 2) {
          if (s * STOREY + 2.2 > h) break;
          sash(b, 0, s * STOREY + 1.1, 1.1, 1.3, hw, 1, 2, false);
        }
        b.pop();
      }
      b.pop();
      const fz = p.d / 2;
      paving(b, -3.4, 3.4, zc + hd + 1.0, fz - 0.6);
      paving(b, -p.w / 2, p.w / 2, -p.d / 2 + 0.5, zc - hd - 2.5);
      for (const x of [-hw + 2.0, hw - 2.0, 0]) tree(b, x, fz - 4.0, 7.4, -0.3, 6);
      for (const x of [-6, 0, 6]) bollard(b, x, fz - 1.2);
      return "Original game art: representative temperate system-built residential slab, banded glazing and slab-edge balconies. Massing only.";
    },
    massing(p) {
      const hd = 8.5, zc = -p.d / 2 + 11.0 + hd, hw = p.w / 2;
      return [
        { x0: -hw, x1: hw, z0: zc - hd, z1: zc + hd, h: p.storeys * STOREY, roof: "flat", para: 1.2, bands: 4, plant: true },
        { x0: -hw - 0.3, x1: hw + 0.3, z0: zc + hd - 0.2, z1: zc + hd + 0.9, h: TALL_STOREY, roof: "flat", para: 0.3, bands: 1 },
      ];
    },
  };

  // ---------------------------------------------------------------- office
  KINDS.office = {
    label: "Office block",
    widths: [24.0, 28.0, 32.0], depth: 24.0, storeys: [4, 5, 6],
    ground: "paved",
    close(b, p) {
      const hw = p.w / 2, hd = 8.0, h = TALL_STOREY + (p.storeys - 1) * STOREY;
      const zc = -p.d / 2 + 7.0 + hd;
      paving(b, -hw, hw, -p.d / 2, p.d / 2);
      b.push(0, 0, 0, zc);
      shell(b, hw, hd, h, 0, () => 0);
      flatRoof(b, hw, hd, h, 1.1, true);
      // Ground floor is glazed the full width and set back behind a colonnade;
      // above it, a mullioned band per floor. That is the 1970s-80s temperate
      // commercial block, and it is deliberately not a glass tower.
      b.box(-hw, hw, 0, TALL_STOREY, hd - 1.0, hd - 0.85, SLOT.GLASS, 0.95);
      // The colonnade is a real recess: cut the ground floor out of the wall
      // and leave the piers standing in front of the glass line.
      b.opening(-hw + 0.45, hw - 0.45, 0.62, TALL_STOREY - 0.06, hd);
      for (let i = 0; i < 7; i += 1) {
        const x = -hw + 0.7 + (i * (p.w - 1.4)) / 6;
        b.box(x - 0.22, x + 0.22, 0, TALL_STOREY, hd - 0.95, hd, SLOT.TRIM, 1.0);
      }
      b.box(-hw - 0.3, hw + 0.3, TALL_STOREY, TALL_STOREY + 0.55, hd - 0.1, hd + 0.55, SLOT.TRIM, 1.08);
      b.box(-3.4, 3.4, 0, 3.2, hd, hd + 1.55, SLOT.TRIM, 1.05);                 // entrance canopy
      b.box(-3.4, -3.2, 0, 3.2, hd + 1.25, hd + 1.55, SLOT.METAL, 1.0);
      b.box(3.2, 3.4, 0, 3.2, hd + 1.25, hd + 1.55, SLOT.METAL, 1.0);
      frontDoor(b, 0, hd - 0.85, 2.4);
      for (let s = 1; s < p.storeys; s += 1) {
        const y = TALL_STOREY + (s - 1) * STOREY;
        ribbon(b, -hw + 0.9, hw - 0.9, y + 0.85, 1.75, hd, 1.6);
        b.box(-hw - 0.1, hw + 0.1, y + 2.75, y + 2.95, hd - 0.05, hd + 0.22, SLOT.TRIM, 1.06);
      }
      b.push(2, 0, 0, 0);
      for (let s = 0; s < p.storeys; s += 1) {
        const y = s === 0 ? 0 : TALL_STOREY + (s - 1) * STOREY;
        ribbon(b, -hw + 1.2, hw - 1.2, y + 1.0, 1.5, hd, 1.9);
      }
      b.box(hw - 5.0, hw - 1.6, 0, 3.4, hd - 0.02, hd + 0.06, SLOT.METAL, 0.9);  // service door
      b.pop();
      for (const yaw of [1, 3]) {
        b.push(yaw, 0, 0, 0);
        for (let s = 0; s < p.storeys; s += 1) {
          const y = s === 0 ? 0 : TALL_STOREY + (s - 1) * STOREY;
          ribbon(b, -hd + 1.0, hd - 1.0, y + 1.0, 1.5, hw, 2.1);
        }
        b.pop();
      }
      b.pop();
      const fz = p.d / 2;
      paving(b, -hw, hw, zc + hd + 1.55, fz);
      for (const x of [-hw + 3.0, 0, hw - 3.0]) bollard(b, x, fz - 1.4);
      for (const x of [-hw + 6.0, hw - 6.0]) tree(b, x, fz - 3.6, 6.6, 0.2, 6);
      b.box(-hw + 1.0, -hw + 3.6, 0, 1.9, zc - hd - 3.0, zc - hd - 1.0, SLOT.METAL, 0.85);  // bin store
      return "Original game art: representative temperate commercial office block, glazed ground floor and banded upper glazing. Massing only.";
    },
    massing(p) {
      const hd = 8.0, zc = -p.d / 2 + 7.0 + hd, hw = p.w / 2;
      return [{ x0: -hw, x1: hw, z0: zc - hd, z1: zc + hd, h: TALL_STOREY + (p.storeys - 1) * STOREY,
        roof: "flat", para: 1.1, bands: 4, plant: true }];
    },
  };

  // ------------------------------------------------------------------ shop
  KINDS.shop = {
    label: "Storefront with flats over",
    widths: [11.0, 13.5, 16.0], depth: 18.0, storeys: [2, 3, 3],
    ground: "paved",
    close(b, p) {
      const hw = p.w / 2, hd = 6.5, h = TALL_STOREY + (p.storeys - 1) * STOREY;
      const zc = -p.d / 2 + 4.6 + hd;
      paving(b, -hw, hw, -p.d / 2, p.d / 2);
      b.push(0, 0, 0, zc);
      shell(b, hw, hd, h, p.storeys - 1, (i) => TALL_STOREY + (i - 1) * STOREY);
      gableRoof(b, hw, hd, h, hd * 0.62, 0.26);
      chimney(b, hw - 0.9, 0, h, h + hd * 0.62 + 0.9, 0.85, 0.7, 2);
      chimney(b, -hw + 0.9, 0, h, h + hd * 0.62 + 0.75, 0.8, 0.7, 2);
      rainwater(b, hw, hd, h);
      // Shopfront: stallriser, deep glazing, mullions, fascia and a blind.
      b.box(-hw + 0.35, hw - 0.35, 0, 0.62, hd - 0.05, hd + 0.05, SLOT.TRIM, 0.94);
      b.box(-hw + 0.35, hw - 0.35, 0.62, 3.1, hd - 0.35, hd - 0.25, SLOT.GLASS, 1.0);
      b.opening(-hw + 0.35, hw - 0.35, 0.62, 3.12, hd);
      const mull = Math.max(2, Math.round(p.w / 2.4));
      for (let i = 0; i <= mull; i += 1) {
        const x = -hw + 0.35 + ((p.w - 0.7) * i) / mull;
        b.box(x - 0.08, x + 0.08, 0.62, 3.1, hd - 0.35, hd - 0.02, SLOT.TRIM, 1.05);
      }
      b.box(-hw + 0.35, hw - 0.35, 3.1, 3.28, hd - 0.4, hd, SLOT.TRIM, 1.0);
      b.box(-hw + 0.1, hw - 0.1, 3.3, TALL_STOREY - 0.15, hd - 0.02, hd + 0.14, SLOT.TRIM, 1.09);   // fascia board
      b.quad([-hw + 0.2, TALL_STOREY - 0.2, hd + 0.12], [hw - 0.2, TALL_STOREY - 0.2, hd + 0.12],
        [hw - 0.2, TALL_STOREY - 0.95, hd + 0.85], [-hw + 0.2, TALL_STOREY - 0.95, hd + 0.85], SLOT.TRIM, 1.02);
      b.box(-hw + 0.2, hw - 0.2, TALL_STOREY - 1.08, TALL_STOREY - 0.95, hd + 0.77, hd + 0.9, SLOT.METAL, 1.0);
      b.box(-hw + 1.1, -hw + 1.3, TALL_STOREY - 0.2, TALL_STOREY + 0.9, hd + 0.02, hd + 0.9, SLOT.METAL, 1.0);
      b.box(-hw + 1.1, -hw + 1.9, TALL_STOREY - 1.1, TALL_STOREY - 0.3, hd + 0.78, hd + 0.86, SLOT.TRIM, 1.06);
      b.box(hw - 2.35, hw - 2.2, 0, 3.1, hd - 0.9, hd - 0.3, SLOT.TRIM, 0.95);   // recessed doorway
      b.box(hw - 1.05, hw - 0.9, 0, 3.1, hd - 0.9, hd - 0.3, SLOT.TRIM, 0.95);
      b.box(hw - 2.35, hw - 0.9, 3.0, 3.12, hd - 0.95, hd - 0.25, SLOT.TRIM, 1.02);
      b.box(hw - 2.35, hw - 0.9, 0, 0.09, hd - 0.95, hd - 0.25, SLOT.GROUND, 1.1);
      b.opening(hw - 2.35, hw - 0.9, 0, 3.02, hd);
      b.box(hw - 2.2, hw - 1.05, 0.09, 2.35, hd - 0.96, hd - 0.86, SLOT.DARK, 1.0);   // shop door
      b.box(hw - 2.2, hw - 1.05, 2.4, 2.95, hd - 0.96, hd - 0.9, SLOT.GLASS, 1.0);
      for (let s = 1; s < p.storeys; s += 1) {
        const y = TALL_STOREY + (s - 1) * STOREY;
        for (let i = 0; i < 4; i += 1) sash(b, -hw + (p.w * (i + 0.5)) / 4, y + 0.7, 1.1, 1.7, hd, 2, 3);
      }
      b.push(2, 0, 0, 0);
      for (let s = 0; s < p.storeys; s += 1) {
        const y = s === 0 ? 0 : TALL_STOREY + (s - 1) * STOREY;
        for (let i = 0; i < 2; i += 1) sash(b, -hw + (p.w * (i + 0.5)) / 2, y + 1.0, 1.15, 1.5, hd, 2, 2, false);
      }
      b.box(-hw + 0.6, -hw + 2.4, 0, 2.2, -hd - 0.06, -hd + 0.02, SLOT.DARK, 1.0);
      b.pop();
      for (const yaw of [1, 3]) {
        b.push(yaw, 0, 0, 0);
        for (let s = 1; s < p.storeys; s += 1) sash(b, 0, TALL_STOREY + (s - 1) * STOREY + 1.0, 1.0, 1.4, hw, 2, 2, false);
        b.pop();
      }
      b.pop();
      const fz = p.d / 2;
      paving(b, -hw, hw, zc + hd, fz);
      b.box(-hw + 1.0, -hw + 2.6, 0, 0.9, zc + hd + 0.4, zc + hd + 1.1, SLOT.TRIM, 1.02);   // pavement display
      litterBin(b, hw - 1.4, fz - 1.4);
      bollard(b, -hw + 1.2, fz - 1.0);
      bollard(b, hw - 4.0, fz - 1.0);
      roadSign(b, -hw + 3.4, fz - 1.0, 2.4);
      b.push(2, 0, 0, zc);
      rainwater(b, hw, hd, h);
      b.pop();
      paving(b, -hw, hw, -p.d / 2 + 0.4, zc - hd - 0.2);
      b.box(-hw + 1.2, -hw + 3.2, 0, 1.5, -p.d / 2 + 1.2, -p.d / 2 + 3.0, SLOT.METAL, 0.85);
      // Rear yard wall with piers, and a gate onto the service lane.
      lowWall(b, -hw, hw, -p.d / 2 + 0.35, -p.d / 2 + 0.7, 2.1);
      for (let i = 0; i < 5; i += 1) {
        const x = -hw + (p.w * i) / 4;
        b.box(x - 0.22, x + 0.22, 0, 2.45, -p.d / 2 + 0.25, -p.d / 2 + 0.8, SLOT.WALL, 1.03);
      }
      railing(b, hw - 3.4, hw - 0.9, -p.d / 2 + 0.52, 0, 2.0, 0.24);
      railing(b, -hw + 0.2, hw - 0.2, zc + hd + 0.95, 0, 0.95, 0.85);
      return "Original game art: representative temperate storefront with flats over, stallriser, mullioned shopfront and fascia. No real trade name is carried.";
    },
    massing(p) {
      const hd = 6.5, zc = -p.d / 2 + 4.6 + hd, hw = p.w / 2;
      return [{ x0: -hw, x1: hw, z0: zc - hd, z1: zc + hd, h: TALL_STOREY + (p.storeys - 1) * STOREY,
        roof: "gable", rise: hd * 0.62, bands: p.storeys, chimneys: [hw - 0.6] }];
    },
  };

  // ------------------------------------------------------------- warehouse
  KINDS.warehouse = {
    label: "Urban warehouse",
    widths: [26.0, 32.0], depth: 30.0, storeys: [1, 1],
    ground: "paved",
    close(b, p) {
      const hw = p.w / 2, hd = 9.5, h = 7.6, zc = -p.d / 2 + 9.0 + hd;
      paving(b, -hw, hw, -p.d / 2, p.d / 2);
      b.push(0, 0, 0, zc);
      b.wall(-hw, hw, 0, h, -hd, hd, SLOT.WALL, 1.0);
      b.bevelBox(-hw - 0.15, hw + 0.15, 0, 1.2, -hd - 0.15, hd + 0.15, SLOT.TRIM, 0.9, 0.03);
      // Profiled cladding ribs — cheap, and the vertical rhythm is what makes
      // a big blank shed read as clad steel rather than as a solid.
      for (let i = 0; i <= 16; i += 1) {
        const x = -hw + (p.w * i) / 16;
        b.box(x - 0.05, x + 0.05, 1.2, h, hd, hd + 0.07, SLOT.METAL, 1.02);
        b.box(x - 0.05, x + 0.05, 1.2, h, -hd - 0.07, -hd, SLOT.METAL, 0.88);
      }
      gableRoof(b, hw, hd, h, hd * 0.19, 0.45);
      for (let i = 0; i < 5; i += 1) {                       // roof lights
        const x = -hw + (p.w * (i + 0.5)) / 5;
        b.box(x - 1.5, x + 1.5, h + hd * 0.09, h + hd * 0.12, 1.0, 4.2, SLOT.GLASS, 1.08);
        b.box(x - 1.5, x + 1.5, h + hd * 0.09, h + hd * 0.12, -4.2, -1.0, SLOT.GLASS, 1.02);
      }
      // Three loading docks with roller shutters under a canopy.
      for (let i = 0; i < 3; i += 1) {
        const cx = -hw + (p.w * (i + 0.5)) / 3;
        b.box(cx - 1.9, cx + 1.9, 0, 4.6, hd - 0.35, hd, SLOT.DARK, 1.0);
        b.opening(cx - 1.9, cx + 1.9, 0, 4.6, hd);
        for (let k = 0; k < 12; k += 1) b.box(cx - 1.85, cx + 1.85, 0.2 + k * 0.36, 0.42 + k * 0.36, hd - 0.32, hd - 0.24, SLOT.METAL, 0.95);
        b.box(cx - 2.2, cx + 2.2, 4.6, 4.85, hd - 0.4, hd + 1.6, SLOT.METAL, 1.06);
        b.box(cx - 2.3, cx + 2.3, 0, 1.1, hd, hd + 1.5, SLOT.GROUND, 1.04);   // dock apron
        bollard(b, cx - 2.6, hd + 1.7);
        bollard(b, cx + 2.6, hd + 1.7);
      }
      b.box(hw - 3.2, hw - 2.0, 0, 2.2, hd - 0.06, hd + 0.03, SLOT.DARK, 1.0);
      b.opening(hw - 3.2, hw - 2.0, 0, 2.2, hd);
      b.box(hw - 3.5, hw - 1.7, 2.2, 2.45, hd - 0.1, hd + 1.0, SLOT.METAL, 1.05);
      // Eaves gutter and downpipes. A shed this size is mostly roof, and the
      // rainwater goods are the only thing on the elevation at eaves level.
      rainwater(b, hw, hd, h);
      if (b.detail >= 2) {
        // A stack of pallets and a skip on the apron. A yard with nothing in
        // it is the tell that a building was drawn and never used.
        for (let i = 0; i < 4; i += 1) {
          b.bevelBox(-hw + 2.0, -hw + 3.2, i * 0.16, i * 0.16 + 0.13, -hd - 3.4, -hd - 2.2, SLOT.TRIM, 0.85 + i * 0.04, 0.015);
        }
        b.loft([[-hw + 5.2, -hd - 3.6], [-hw + 8.2, -hd - 3.6], [-hw + 8.2, -hd - 2.0], [-hw + 5.2, -hd - 2.0]],
          [[-hw + 4.9, -hd - 3.8], [-hw + 8.5, -hd - 3.8], [-hw + 8.5, -hd - 1.8], [-hw + 4.9, -hd - 1.8]],
          0, 1.35, SLOT.METAL, 0.88);
      }
      for (const yaw of [1, 3, 2]) {
        b.push(yaw, 0, 0, 0);
        const run = yaw === 2 ? hw : hd;
        for (let i = 0; i < 4; i += 1) ribbon(b, -run + 1.4 + (i * (run * 2 - 2.8)) / 4, -run + 1.4 + ((i + 0.72) * (run * 2 - 2.8)) / 4, 4.4, 1.5, yaw === 2 ? hd : hw, 1.5);
        b.pop();
      }
      b.pop();
      const fz = p.d / 2;
      for (let i = 0; i < 8; i += 1) b.box(-hw + 0.6 + i * (p.w / 8), -hw + 0.75 + i * (p.w / 8), 0.1, 0.12, fz - 9.5, fz - 3.5, SLOT.TRIM, 1.08);  // yard bay markings
      railing(b, -hw, hw, fz - 0.6, 0, 2.1, 0.4);
      for (const x of [-hw + 0.2, hw - 0.2]) b.box(x - 0.12, x + 0.12, 0, 2.4, fz - 0.75, fz - 0.45, SLOT.METAL, 1.0);
      return "Original game art: representative temperate urban warehouse, clad steel frame, three loading docks and a yard. Massing only; no stated capacity.";
    },
    massing(p) {
      const hd = 9.5, zc = -p.d / 2 + 9.0 + hd, hw = p.w / 2;
      return [{ x0: -hw, x1: hw, z0: zc - hd, z1: zc + hd, h: 7.6, roof: "gable", rise: hd * 0.19, bands: 2 }];
    },
  };

  // ----------------------------------------------------------------- civic
  KINDS.civic = {
    label: "Civic building",
    widths: [26.0, 30.0], depth: 30.0, storeys: [3, 3],
    ground: "paved",
    close(b, p) {
      const hw = p.w / 2, hd = 8.5, h = TALL_STOREY + (p.storeys - 1) * 3.6;
      const zc = -p.d / 2 + 8.0 + hd;
      paving(b, -hw, hw, -p.d / 2, p.d / 2);
      b.push(0, 0, 0, zc);
      shell(b, hw, hd, h, p.storeys - 1, (i) => TALL_STOREY + (i - 1) * 3.6);
      b.box(-hw - 0.3, hw + 0.3, h, h + 0.6, -hd - 0.3, hd + 0.3, SLOT.TRIM, 1.08);   // cornice
      hipRoof(b, hw + 0.3, hd + 0.3, h + 0.6, hd * 0.42, 0.08);
      // Quoins at the corners, a portico on axis, steps down to the pavement.
      for (const sx of [-1, 1]) {
        for (let i = 0; i < 9; i += 1) {
          b.box(sx * (hw - 0.9), sx * (hw + 0.09), i * 0.85, i * 0.85 + 0.42, hd - 0.02, hd + 0.09, SLOT.TRIM, 1.05);
        }
      }
      // The portico is a PORCH, not a solid block with columns inside it. It
      // used to project 2.6 m and the colonnade stood at 2.2 m — buried in its
      // own masonry, invisible from every angle, on the one elevation the kind
      // exists to deliver. The wall comes forward 1.8 m and the columns stand
      // clear in front of it, under the pediment, where a portico's columns go.
      b.wall(-4.6, 4.6, 0, TALL_STOREY + 1.4, hd, hd + 1.8, SLOT.WALL, 1.03);
      b.box(-5.0, 5.0, TALL_STOREY + 1.4, TALL_STOREY + 1.85, hd - 0.1, hd + 3.0, SLOT.TRIM, 1.1);
      b.tri([-5.0, TALL_STOREY + 1.85, hd + 3.0], [5.0, TALL_STOREY + 1.85, hd + 3.0], [0, TALL_STOREY + 3.3, hd + 3.0], SLOT.TRIM, 1.06);
      b.quad([-5.0, TALL_STOREY + 1.85, hd + 3.0], [0, TALL_STOREY + 3.3, hd + 3.0], [0, TALL_STOREY + 3.3, hd], [-5.0, TALL_STOREY + 1.85, hd], SLOT.ROOF, 0.95);
      b.quad([0, TALL_STOREY + 3.3, hd + 3.0], [5.0, TALL_STOREY + 1.85, hd + 3.0], [5.0, TALL_STOREY + 1.85, hd], [0, TALL_STOREY + 3.3, hd], SLOT.ROOF, 1.05);
      for (let i = 0; i < 4; i += 1) {
        const x = -3.4 + i * 2.27;
        tube(b, x, hd + 2.45, 0.52, 0.74, 0.38, 0.33, b.segs(12, 8, 5), SLOT.TRIM, 1.02);     // base
        tube(b, x, hd + 2.45, 0.74, TALL_STOREY + 1.32, 0.31, 0.27, b.segs(12, 8, 5), SLOT.TRIM, 1.04);
        tube(b, x, hd + 2.45, TALL_STOREY + 1.32, TALL_STOREY + 1.5, 0.38, 0.31, b.segs(12, 8, 5), SLOT.TRIM, 1.09);
      }
      for (let i = 0; i < 4; i += 1) b.box(-5.6 + i * 0.0, 5.6, i * 0.2, i * 0.2 + 0.2, hd + 2.6 + i * 0.42, hd + 4.4, SLOT.TRIM, 1.04);
      frontDoor(b, 0, hd + 1.8, 3.0);
      // Clock cupola. It carries no maker's name and no dial reading.
      b.box(-1.9, 1.9, h + hd * 0.42 - 0.5, h + hd * 0.42 + 3.4, -1.9, 1.9, SLOT.WALL, 1.04);
      b.box(-2.15, 2.15, h + hd * 0.42 + 3.4, h + hd * 0.42 + 3.65, -2.15, 2.15, SLOT.TRIM, 1.1);
      for (const f of [[1.9, 1.98, 0], [-1.98, -1.9, 0]]) tube(b, 0, (f[0] + f[1]) / 2, h + hd * 0.42 + 1.1, h + hd * 0.42 + 2.5, 0.8, 0.8, b.segs(14, 10, 6), SLOT.TRIM, 1.08);
      tube(b, 0, 0, h + hd * 0.42 + 3.65, h + hd * 0.42 + 6.1, 1.75, 0.12, b.segs(12, 8, 5), SLOT.ROOF, 1.06);
      tube(b, 0, 0, h + hd * 0.42 + 6.1, h + hd * 0.42 + 7.4, 0.07, 0.07, 6, SLOT.METAL, 1.1);
      for (let s = 0; s < p.storeys; s += 1) {
        const y = s === 0 ? 0 : TALL_STOREY + (s - 1) * 3.6;
        const fh = s === 0 ? TALL_STOREY : 3.6;
        for (const sx of [-1, 1]) {
          for (let i = 0; i < 3; i += 1) sash(b, sx * (5.8 + i * 3.1), y + fh * 0.26, 1.5, fh * 0.56, hd, 2, 3);
        }
      }
      b.push(2, 0, 0, 0);
      for (let s = 0; s < p.storeys; s += 1) {
        const y = s === 0 ? 0 : TALL_STOREY + (s - 1) * 3.6;
        for (let i = 0; i < 7; i += 1) sash(b, -hw + (p.w * (i + 0.5)) / 7, y + 1.0, 1.3, 1.9, hd, 2, 3, false);
      }
      b.pop();
      for (const yaw of [1, 3]) {
        b.push(yaw, 0, 0, 0);
        for (let s = 0; s < p.storeys; s += 1) {
          for (let i = 0; i < 3; i += 1) sash(b, -hd + (hd * 2 * (i + 0.5)) / 3, (s === 0 ? 0 : TALL_STOREY + (s - 1) * 3.6) + 1.0, 1.2, 1.85, hw, 2, 3, false);
        }
        b.pop();
      }
      b.pop();
      const fz = p.d / 2;
      paving(b, -hw, hw, zc + hd + 4.4, fz);
      for (const x of [-8.5, 8.5]) { bollard(b, x, fz - 2.0); lampColumn(b, x, fz - 3.4, 0); }
      for (const x of [-hw + 1.6, hw - 1.6]) tree(b, x, fz - 3.0, 7.2, 0.2, 6);
      return "Original game art: representative temperate civic building — portico, quoins, hipped roof and clock cupola. Original design; not any existing town hall.";
    },
    massing(p) {
      const hd = 8.5, zc = -p.d / 2 + 8.0 + hd, hw = p.w / 2, h = TALL_STOREY + (p.storeys - 1) * 3.6;
      return [
        { x0: -hw, x1: hw, z0: zc - hd, z1: zc + hd, h, roof: "hip", rise: hd * 0.42, bands: p.storeys },
        { x0: -4.6, x1: 4.6, z0: zc + hd, z1: zc + hd + 1.8, h: TALL_STOREY + 1.85, roof: "flat", para: 0.4, bands: 1 },
        { x0: -1.9, x1: 1.9, z0: -1.9, z1: 1.9, y0: h + hd * 0.42 - 0.5, h: h + hd * 0.42 + 3.65, roof: "flat", para: 0.3, bands: 1 },
      ];
    },
  };

  // ---------------------------------------------------------------- school
  KINDS.school = {
    label: "School",
    widths: [38.0, 44.0], depth: 34.0, storeys: [2, 2],
    ground: "grass",
    close(b, p) {
      const hw = p.w / 2, hd = 6.5, h = p.storeys * 3.3, zc = -p.d / 2 + 15.0 + hd;
      grass(b, -hw, hw, -p.d / 2, p.d / 2);
      b.push(0, 0, 0, zc);
      shell(b, hw, hd, h, 1, () => 3.3);
      gableRoof(b, hw, hd, h, hd * 0.28, 0.5);
      rainwater(b, hw, hd, h);
      // Classroom glazing is banded and generous: the one feature that makes a
      // school read as a school from above the rooftops.
      for (let s = 0; s < p.storeys; s += 1) {
        const y = s * 3.3;
        for (let i = 0; i < 5; i += 1) {
          const x0 = -hw + 1.3 + (i * (p.w - 2.6)) / 5, x1 = x0 + (p.w - 2.6) / 5 - 1.5;
          ribbon(b, x0, x1, y + 1.05, 1.75, hd, 1.35);
        }
      }
      b.push(2, 0, 0, 0);
      for (let s = 0; s < p.storeys; s += 1) {
        for (let i = 0; i < 4; i += 1) {
          const x0 = -hw + 1.6 + (i * (p.w - 3.2)) / 4, x1 = x0 + (p.w - 3.2) / 4 - 2.0;
          ribbon(b, x0, x1, s * 3.3 + 1.1, 1.6, hd, 1.5);
        }
      }
      b.pop();
      // Assembly hall wing, taller and blanker, at one end.
      const wx = hw - 8.0;
      b.box(wx - 8.0, wx + 8.0, 0, 6.6, -hd - 13.0, -hd + 0.2, SLOT.WALL, 0.97);
      b.box(wx - 8.2, wx + 8.2, 0, 0.6, -hd - 13.2, -hd + 0.2, SLOT.TRIM, 0.9);
      b.push(0, wx, 0, -hd - 6.4);
      gableRoof(b, 8.0, 6.6, 6.6, 1.5, 0.4);
      b.pop();
      for (let i = 0; i < 3; i += 1) b.box(wx - 6.0 + i * 4.4, wx - 3.4 + i * 4.4, 3.6, 5.6, -hd - 13.06, -hd - 12.94, SLOT.GLASS, 1.0);
      // Covered walkway linking the two.
      for (let i = 0; i < 5; i += 1) tube(b, -hw + 3.0 + i * 4.2, -hd - 3.2, 0, 2.9, 0.09, 0.09, b.segs(8, 6, 4), SLOT.METAL, 1.0);
      b.box(-hw + 2.4, -hw + 20.0, 2.9, 3.08, -hd - 4.1, -hd - 2.3, SLOT.METAL, 1.06);
      b.pop();
      const fz = p.d / 2;
      paving(b, -hw, hw, zc + hd, fz - 1.2);
      b.box(-hw + 4.0, -hw + 18.0, 0.11, 0.13, zc + hd + 1.4, zc + hd + 5.4, SLOT.TRIM, 1.06);   // playground markings
      b.box(-hw + 10.5, -hw + 11.5, 0.11, 0.13, zc + hd + 1.4, zc + hd + 5.4, SLOT.TRIM, 1.06);
      b.cyl(-hw + 11.0, zc + hd + 3.4, 0.11, 0.13, 1.9, 1.9, 12, SLOT.TRIM, 1.06);
      railing(b, -hw, hw, fz - 0.7, 0, 1.9, 0.32);
      for (const x of [-hw + 2, hw - 2]) b.box(x - 0.14, x + 0.14, 0, 2.3, fz - 0.9, fz - 0.5, SLOT.METAL, 1.0);
      for (const x of [hw - 6, hw - 1.6]) tree(b, x, fz - 4.5, 7.0, -0.3, 6);
      bench(b, -hw + 6, zc + hd + 1.4, 0);
      bench(b, -hw + 12, zc + hd + 1.4, 0);
      return "Original game art: representative temperate school, banded classroom glazing, hall wing and covered walkway. Massing only; no roll or capacity implied.";
    },
    massing(p) {
      const hd = 6.5, zc = -p.d / 2 + 15.0 + hd, hw = p.w / 2, wx = hw - 8.0;
      return [
        { x0: -hw, x1: hw, z0: zc - hd, z1: zc + hd, h: p.storeys * 3.3, roof: "gable", rise: hd * 0.28, bands: 2 },
        { x0: wx - 8.0, x1: wx + 8.0, z0: zc - hd - 13.0, z1: zc - hd + 0.2, h: 6.6, roof: "gable", rise: 1.5, bands: 2 },
      ];
    },
  };

  // -------------------------------------------------------------- hospital
  KINDS.hospital = {
    label: "Hospital",
    widths: [34.0, 40.0], depth: 42.0, storeys: [5, 6],
    ground: "paved",
    close(b, p) {
      const hw = p.w / 2, hd = 8.0, h = p.storeys * STOREY, zc = -p.d / 2 + 10.0 + hd;
      paving(b, -hw, hw, -p.d / 2, p.d / 2);
      b.push(0, 0, 0, zc);
      shell(b, hw, hd, h, 0, () => 0);
      flatRoof(b, hw, hd, h, 1.1, true);
      // Two-storey podium in front of the ward slab, with the ambulance
      // canopy on it — the composition every temperate district hospital of
      // this period ended up with.
      b.wall(-hw + 2.0, hw - 2.0, 0, 2 * TALL_STOREY, hd, hd + 7.5, SLOT.WALL, 1.02);
      b.box(-hw + 1.7, hw - 1.7, 2 * TALL_STOREY, 2 * TALL_STOREY + 0.35, hd - 0.1, hd + 7.8, SLOT.TRIM, 1.08);
      for (let s = 0; s < 2; s += 1) ribbon(b, -hw + 3.0, hw - 3.0, s * TALL_STOREY + 1.2, 2.0, hd + 7.5, 1.8);
      b.box(-5.5, 5.5, 0, 4.4, hd + 7.5, hd + 12.5, SLOT.TRIM, 1.05);
      b.box(-5.5, 5.5, 4.4, 4.75, hd + 7.3, hd + 12.9, SLOT.TRIM, 1.1);
      for (const x of [-5.2, 5.2]) tube(b, x, hd + 12.2, 0, 4.4, 0.19, 0.19, b.segs(10, 7, 5), SLOT.METAL, 1.0);
      frontDoor(b, 0, hd + 7.5, 2.6);
      for (let s = 2; s < p.storeys; s += 1) {
        ribbon(b, -hw + 1.0, hw - 1.0, s * STOREY + 0.9, 1.6, hd, 1.7);
        b.box(-hw - 0.1, hw + 0.1, s * STOREY + 2.6, s * STOREY + 2.78, hd - 0.05, hd + 0.2, SLOT.TRIM, 1.06);
      }
      b.push(2, 0, 0, 0);
      for (let s = 0; s < p.storeys; s += 1) ribbon(b, -hw + 1.2, hw - 1.2, s * STOREY + 1.0, 1.5, hd, 1.9);
      b.box(hw - 8.0, hw - 3.0, 0, 4.2, hd - 0.06, hd + 0.04, SLOT.METAL, 0.88);
      b.pop();
      for (const yaw of [1, 3]) {
        b.push(yaw, 0, 0, 0);
        for (let s = 0; s < p.storeys; s += 1) ribbon(b, -hd + 1.4, hd - 1.4, s * STOREY + 1.0, 1.4, hw, 2.0);
        b.pop();
      }
      b.pop();
      const fz = p.d / 2;
      paving(b, -hw, hw, zc + hd + 12.5, fz);
      for (let i = 0; i < 6; i += 1) b.box(-hw + 2 + i * 5.2, -hw + 2.15 + i * 5.2, 0.1, 0.12, fz - 5.0, fz - 0.6, SLOT.TRIM, 1.08);
      for (const x of [-hw + 1, hw - 1]) lampColumn(b, x, fz - 1.6, x < 0 ? 1 : -1);
      for (const x of [-hw + 8, hw - 8]) tree(b, x, fz - 2.2, 6.4, 0.2, 6);
      bollard(b, -6.5, zc + hd + 13.0); bollard(b, 6.5, zc + hd + 13.0);
      return "Original game art: representative temperate hospital — ward slab over a two-storey podium with an ambulance canopy. Massing only; no bed count implied.";
    },
    massing(p) {
      const hd = 8.0, zc = -p.d / 2 + 10.0 + hd, hw = p.w / 2;
      return [
        { x0: -hw, x1: hw, z0: zc - hd, z1: zc + hd, h: p.storeys * STOREY, roof: "flat", para: 1.1, bands: 4, plant: true },
        { x0: -hw + 2.0, x1: hw - 2.0, z0: zc + hd, z1: zc + hd + 7.5, h: 2 * TALL_STOREY, roof: "flat", para: 0.4, bands: 2 },
      ];
    },
  };

  // ------------------------------------------------------------ university
  KINDS.university = {
    label: "University",
    widths: [36.0, 42.0], depth: 40.0, storeys: [3, 4],
    ground: "grass",
    close(b, p) {
      const hw = p.w / 2, hd = 7.0, h = TALL_STOREY + (p.storeys - 1) * 3.5;
      const zc = -p.d / 2 + 24.0 + hd;
      grass(b, -hw, hw, -p.d / 2, p.d / 2);
      // A U around a courtyard: main range at the rear, two wings coming
      // forward. It costs three volumes and it is the shape that says campus.
      b.push(0, 0, 0, zc);
      shell(b, hw, hd, h, p.storeys - 1, (i) => TALL_STOREY + (i - 1) * 3.5);
      gableRoof(b, hw, hd, h, hd * 0.55, 0.4);
      for (const x of [-hw + 4, 0, hw - 4]) chimney(b, x, 0, h, h + hd * 0.55 + 0.8, 1.0, 0.8, 3);
      rainwater(b, hw, hd, h);
      roofDormers(b, hw, hd, h, hd * 0.55, 0.4, [-hw + 9, -hw + 15, hw - 15, hw - 9], 1.35);
      b.wall(-4.4, 4.4, 0, h + 2.2, hd, hd + 1.4, SLOT.WALL, 1.03);
      b.box(-4.8, 4.8, h + 2.2, h + 2.45, hd - 0.1, hd + 1.7, SLOT.TRIM, 1.1);
      b.box(-2.1, 2.1, 0, 4.4, hd + 1.36, hd + 1.46, SLOT.DARK, 1.0);                       // arched entry
      for (let i = 0; i < 8; i += 1) {
        const a0 = Math.PI * (i / 8), a1 = Math.PI * ((i + 1) / 8);
        b.tri([0, 4.4, hd + 1.46], [Math.cos(a1) * 2.1, 4.4 + Math.sin(a1) * 2.1, hd + 1.46],
          [Math.cos(a0) * 2.1, 4.4 + Math.sin(a0) * 2.1, hd + 1.46], SLOT.DARK, 1.0);
        b.box(Math.cos(a0) * 2.1 - 0.16, Math.cos(a0) * 2.1 + 0.16, 4.4 + Math.sin(a0) * 2.1 - 0.16,
          4.4 + Math.sin(a0) * 2.1 + 0.16, hd + 1.34, hd + 1.5, SLOT.TRIM, 1.05);
      }
      frontDoor(b, 0, hd + 1.4, 2.6);
      for (let s = 0; s < p.storeys; s += 1) {
        const y = s === 0 ? 0 : TALL_STOREY + (s - 1) * 3.5;
        const fh = s === 0 ? TALL_STOREY : 3.5;
        for (const sx of [-1, 1]) for (let i = 0; i < 3; i += 1) sash(b, sx * (6.2 + i * 3.5), y + fh * 0.24, 1.45, fh * 0.58, hd, 2, 3);
      }
      b.push(2, 0, 0, 0);
      for (let s = 0; s < p.storeys; s += 1) {
        for (let i = 0; i < 6; i += 1) sash(b, -hw + (p.w * (i + 0.5)) / 6, (s === 0 ? 0 : TALL_STOREY + (s - 1) * 3.5) + 1.1, 1.35, 1.9, hd, 2, 3, false);
      }
      b.pop();
      b.pop();
      const wingH = TALL_STOREY + 3.5, wd = 8.0;
      for (const sx of [-1, 1]) {
        const cx = sx * (hw - 6.0);
        b.push(0, cx, 0, zc - hd - wd);
        shell(b, 6.0, wd, wingH, 1, () => TALL_STOREY);
        gableRoof(b, 6.0, wd, wingH, wd * 0.5, 0.36);
        chimney(b, sx * 4.6, 0, wingH, wingH + wd * 0.5 + 0.8, 0.9, 0.7, 2);
        for (const yaw of [1, 3]) {
          b.push(yaw, 0, 0, 0);
          for (let s = 0; s < 2; s += 1) for (let i = 0; i < 3; i += 1) sash(b, -wd + (wd * 2 * (i + 0.5)) / 3, (s ? TALL_STOREY : 0) + 1.15, 1.3, 1.8, 6.0, 2, 3, false);
          b.pop();
        }
        b.push(2, 0, 0, 0);
        for (let s = 0; s < 2; s += 1) for (let i = 0; i < 3; i += 1) sash(b, -6.0 + (12 * (i + 0.5)) / 3, (s ? TALL_STOREY : 0) + 1.15, 1.25, 1.8, wd, 2, 3, false);
        b.pop();
        b.pop();
      }
      // Courtyard between the wings.
      const cyz = zc - hd - wd;
      paving(b, -hw + 12.5, hw - 12.5, cyz - wd, cyz + wd);
      for (const x of [-hw + 15, hw - 15]) tree(b, x, cyz, 7.6, 0.2, 6);
      bench(b, 0, cyz - wd + 2.0, 0); bench(b, 0, cyz + wd - 2.0, 2);
      const fz = p.d / 2;
      railing(b, -hw, hw, fz - 0.8, 0, 1.9, 0.3);
      paving(b, -3.0, 3.0, zc + hd + 1.4, fz - 0.9);
      for (const x of [-hw + 5, hw - 5]) tree(b, x, fz - 5.0, 8.2, -0.2, 6);
      return "Original game art: representative temperate university range around a courtyard. Original design; not any existing institution.";
    },
    massing(p) {
      const hd = 7.0, zc = -p.d / 2 + 24.0 + hd, hw = p.w / 2;
      const h = TALL_STOREY + (p.storeys - 1) * 3.5, wingH = TALL_STOREY + 3.5, wd = 8.0;
      const vols = [{ x0: -hw, x1: hw, z0: zc - hd, z1: zc + hd, h, roof: "gable", rise: hd * 0.55, bands: 3,
        chimneys: [-hw + 4, 0, hw - 4] }];
      for (const sx of [-1, 1]) {
        const cx = sx * (hw - 6.0);
        vols.push({ x0: cx - 6.0, x1: cx + 6.0, z0: zc - hd - wd * 2, z1: zc - hd, h: wingH, roof: "gable", rise: wd * 0.5, bands: 2 });
      }
      return vols;
    },
  };

  // --------------------------------------------------------------- stadium
  KINDS.stadium = {
    label: "Stadium",
    widths: [178.0], depth: 140.0, storeys: [1],
    ground: "paved",
    close(b, p) {
      // A standard association-football pitch is 105 x 68 metres by the laws
      // of the game, so the bowl is sized off that rather than off any real
      // ground. Everything outside the touchline is original scenery.
      // The bowl's segment count is the one thing that falls with detail here:
      // the rake, the height and the plan stay exactly what they are, so the
      // card is the same ground seen with a coarser curve rather than a
      // smaller stadium.
      const seg = b.segs(24, 16, 14), rows = 12;
      const inRx = 62, inRz = 44, step = 1.55, rise = 0.95;
      b.slab(-52.5, 52.5, -34, 34, 0.12, SLOT.FOLIAGE, 0.95);
      for (let i = 0; i < 6; i += 1) b.slab(-52.5, 52.5, -34 + i * 11.33, -34 + i * 11.33 + 5.6, 0.13, SLOT.FOLIAGE, 1.05);
      b.box(-52.5, 52.5, 0.13, 0.15, -34, -33.88, SLOT.TRIM, 1.08);
      b.box(-52.5, 52.5, 0.13, 0.15, 33.88, 34, SLOT.TRIM, 1.08);
      b.box(-0.06, 0.06, 0.13, 0.15, -34, 34, SLOT.TRIM, 1.08);
      b.cyl(0, 0, 0.13, 0.15, 9.15, 9.15, 16, SLOT.TRIM, 1.08, false);
      for (const sx of [-1, 1]) b.box(sx * 52.5 - sx * 16.5, sx * 52.5, 0.13, 0.15, -20.16, -20.04, SLOT.TRIM, 1.08);
      // Raked terracing, twelve rows: a riser facing IN toward the pitch and a
      // tread facing up. `band` and not `loft`, because a bowl is seen from
      // inside and `loft` forces walls outward.
      let ry0 = 1.2;
      for (let r = 0; r < rows; r += 1) {
        const inner = oval(0, 0, inRx + r * step, inRz + r * step, seg);
        const outer = oval(0, 0, inRx + (r + 1) * step, inRz + (r + 1) * step, seg);
        const y1 = ry0 + rise;
        b.band(reversed(inner), reversed(inner), ry0, y1, SLOT.GROUND, 0.8);
        b.band(outer, inner, y1, y1, SLOT.WALL, r % 3 === 0 ? 1.06 : 0.9);
        ry0 = y1;
      }
      const outRx = inRx + rows * step, outRz = inRz + rows * step;
      const rim = oval(0, 0, outRx, outRz, seg), skin = oval(0, 0, outRx + 3.0, outRz + 3.0, seg);
      b.band(skin, rim, ry0, ry0, SLOT.GROUND, 0.95);                       // concourse deck
      // THE OUTER SKIN IS SMOOTHED and the terracing is not, and that is the
      // whole rule in one object: a continuous curved wall shades as a curve,
      // a flight of steps shades as steps. Left flat, a 24-sided bowl read as
      // a drum built out of flat panels from any distance at all.
      b.smoothed(() => {
        b.band(skin, skin, 0, ry0 + 2.4, SLOT.WALL, 1.0);                   // outer skin
      });
      b.band(skin, oval(0, 0, outRx + 3.2, outRz + 3.2, seg), ry0 + 2.4, ry0 + 2.7, SLOT.TRIM, 1.08);
      // Crush barriers across the terracing, and a goal at each end. A pitch
      // with no goals on it is the one thing that stops a bowl reading as a
      // ground; the frames are generic and carry no markings.
      if (b.detail >= 2) {
        for (let r = 2; r < rows; r += 3) {
          const ring = oval(0, 0, inRx + (r + 0.5) * step, inRz + (r + 0.5) * step, seg);
          const y = 1.2 + r * rise;
          for (let i = 0; i < ring.length; i += 2) {
            b.box(ring[i][0] - 0.9, ring[i][0] + 0.9, y + 0.9, y + 1.02, ring[i][1] - 0.05, ring[i][1] + 0.05, SLOT.METAL, 1.02);
          }
        }
        for (const sx of [-1, 1]) {
          const gx = sx * 52.5;
          for (const sz of [-1, 1]) tube(b, gx, sz * 3.66, 0.12, 2.56, 0.06, 0.06, 7, SLOT.TRIM, 1.1);
          b.box(gx - 0.06, gx + 0.06, 2.44, 2.56, -3.72, 3.72, SLOT.TRIM, 1.12);
          for (const sz of [-1, 1]) b.box(gx - 0.05, gx + sx * 1.9, 0.12, 0.2, sz * 3.66 - 0.04, sz * 3.66 + 0.04, SLOT.TRIM, 1.0);
        }
        // Hoardings round the touchline: blank boards, no advertiser.
        for (const sz of [-1, 1]) {
          for (let i = 0; i < 14; i += 1) {
            const x = -49 + i * 7.2;
            b.box(x, x + 6.6, 0.12, 1.05, sz * 37.2 - 0.08, sz * 37.2 + 0.08, SLOT.TRIM, sz > 0 ? 1.06 : 0.88);
          }
        }
      }
      // Turnstile blocks around the concourse.
      const gate = oval(0, 0, outRx + 3.6, outRz + 3.6, 8);
      for (const g of gate) b.box(g[0] - 2.2, g[0] + 2.2, 0, 3.2, g[1] - 2.2, g[1] + 2.2, SLOT.WALL, 0.95);
      // Cantilever roofs over the two side stands.
      for (const sz of [-1, 1]) {
        for (let i = 0; i < 9; i += 1) {
          const x = -34 + i * 8.5;
          b.quad([x - 4.0, ry0 + 3.0, sz * (inRz + 2)], [x + 4.0, ry0 + 3.0, sz * (inRz + 2)],
            [x + 4.0, ry0 + 7.0, sz * (outRz + 4)], [x - 4.0, ry0 + 7.0, sz * (outRz + 4)], SLOT.METAL, sz > 0 ? 1.06 : 0.86);
          b.box(x - 0.5, x + 0.5, ry0 + 2.6, ry0 + 7.0, sz * (outRz + 2.5), sz * (outRz + 3.5), SLOT.METAL, 1.0);
        }
      }
      // Four floodlight masts: legs, ring frames, braces and a lamp head.
      for (const sx of [-1, 1]) for (const sz of [-1, 1]) {
        const cx = sx * (outRx - 4), cz = sz * (outRz - 2);
        b.push(0, cx, 0, cz);
        for (const lx of [-1.5, 1.5]) for (const lz of [-1.5, 1.5]) {
          b.loft([[lx - 0.16, lz - 0.16], [lx + 0.16, lz - 0.16], [lx + 0.16, lz + 0.16], [lx - 0.16, lz + 0.16]],
            [[lx * 0.3 - 0.1, lz * 0.3 - 0.1], [lx * 0.3 + 0.1, lz * 0.3 - 0.1], [lx * 0.3 + 0.1, lz * 0.3 + 0.1], [lx * 0.3 - 0.1, lz * 0.3 + 0.1]],
            0, 34, SLOT.METAL, 1.0, false);
        }
        for (let k = 0; k <= 5; k += 1) {
          const y = k * 6.2, f = 1.5 - (1.5 - 0.45) * (y / 34);
          b.box(-f - 0.1, f + 0.1, y, y + 0.16, -f - 0.1, -f + 0.1, SLOT.METAL, 1.0);
          b.box(-f - 0.1, f + 0.1, y, y + 0.16, f - 0.1, f + 0.1, SLOT.METAL, 1.0);
          b.box(-f - 0.1, -f + 0.1, y, y + 0.16, -f, f, SLOT.METAL, 0.95);
          b.box(f - 0.1, f + 0.1, y, y + 0.16, -f, f, SLOT.METAL, 1.05);
          if (k < 5) {
            const f2 = 1.5 - (1.5 - 0.45) * ((y + 6.2) / 34);
            b.quad([-f, y, -f - 0.05], [f2, y + 6.2, -f2 - 0.05], [f2, y + 6.2, -f2 + 0.05], [-f, y, -f + 0.05], SLOT.METAL, 0.9);
            b.quad([-f, y, f - 0.05], [f2, y + 6.2, f2 - 0.05], [f2, y + 6.2, f2 + 0.05], [-f, y, f + 0.05], SLOT.METAL, 1.02);
          }
        }
        b.box(-3.2, 3.2, 34, 40.0, -0.7, 0.7, SLOT.METAL, 1.0);
        for (let r = 0; r < 3; r += 1) for (let c = 0; c < 6; c += 1) {
          b.box(-3.0 + c * 1.0, -2.3 + c * 1.0, 34.4 + r * 1.8, 35.6 + r * 1.8, -sz * 0.75, -sz * 0.68, SLOT.GLASS, 1.1);
        }
        b.pop();
      }
      return "Original game art: representative temperate stadium bowl around a standard-size pitch. Original design; not any existing ground, and no attendance implied.";
    },
    map(b) {
      const seg = 14, outRx = 62 + 12 * 1.55 + 3, outRz = 44 + 12 * 1.55 + 3;
      b.slab(-52.5, 52.5, -34, 34, 0.12, SLOT.FOLIAGE, 0.95);
      const inner = oval(0, 0, 62, 44, seg), outer = oval(0, 0, outRx, outRz, seg);
      b.band(reversed(inner), reversed(outer), 1.2, 16.0, SLOT.WALL, 0.95);       // the rake, as one surface
      b.band(outer, outer, 0, 18.4, SLOT.WALL, 1.0);
      b.band(outer, oval(0, 0, outRx + 0.3, outRz + 0.3, seg), 18.4, 18.9, SLOT.TRIM, 1.08);
      b.band(reversed(inner), reversed(inner), 0.12, 1.4, SLOT.GROUND, 0.85);
      for (const sz of [-1, 1]) b.quad([-40, 19.0, sz * 46], [40, 19.0, sz * 46], [40, 22.0, sz * (outRz + 1)], [-40, 22.0, sz * (outRz + 1)], SLOT.METAL, sz > 0 ? 1.06 : 0.86);
      for (const sx of [-1, 1]) for (const sz of [-1, 1]) {
        b.cyl(sx * (outRx - 4), sz * (outRz - 2), 0, 34, 1.6, 0.5, 4, SLOT.METAL, 1.0);
        b.box(sx * (outRx - 4) - 3.2, sx * (outRx - 4) + 3.2, 34, 40, sz * (outRz - 2) - 0.7, sz * (outRz - 2) + 0.7, SLOT.METAL, 1.05);
      }
      return b;
    },
  };

  // ------------------------------------------------------------------ park
  KINDS.park = {
    label: "Park and public space",
    widths: [30.0, 38.0, 46.0], depth: 30.0, storeys: [1],
    ground: "grass",
    close(b, p) {
      const hw = p.w / 2, hd = p.d / 2;
      grass(b, -hw, hw, -hd, hd);
      // Two paths crossing on a small circle, which is the shape of every
      // temperate municipal park laid out in the nineteenth century and never
      // replanned since.
      paving(b, -hw + 0.4, hw - 0.4, -1.4, 1.4, 0.12);
      paving(b, -1.4, 1.4, -hd + 0.4, hd - 0.4, 0.12);
      b.cyl(0, 0, 0.12, 0.14, 4.6, 4.6, 14, SLOT.GROUND, 1.02, false);
      tube(b, 0, 0, 0, 0.55, 2.3, 2.1, b.segs(16, 10, 6), SLOT.TRIM, 1.05);
      tube(b, 0, 0, 0.55, 0.75, 1.9, 1.7, b.segs(16, 10, 6), SLOT.WALL, 1.02);
      for (let i = 0; i < 6; i += 1) {                                    // the bandstand columns
        const a = (i / 6) * Math.PI * 2;
        tube(b, Math.cos(a) * 1.55, Math.sin(a) * 1.42, 0.75, 2.55, 0.09, 0.075, b.segs(8, 6, 4), SLOT.METAL, 1.02);
      }
      tube(b, 0, 0, 0.75, 2.6, 0.35, 0.28, b.segs(10, 7, 5), SLOT.TRIM, 1.06);
      tube(b, 0, 0, 2.5, 3.15, 2.0, 0.1, b.segs(16, 10, 6), SLOT.ROOF, 1.08);
      const s = p.seed || 0;
      for (let i = 0; i < 11; i += 1) {
        const a = (i / 11) * Math.PI * 2;
        const rx = hw * askSpan(s, 300 + i, 0.42, 0.86), rz = hd * askSpan(s, 340 + i, 0.42, 0.86);
        tree(b, Math.cos(a) * rx, Math.sin(a) * rz, askSpan(s, 380 + i, 6.2, 9.4), askSpan(s, 420 + i, -0.4, 0.4), 6);
      }
      for (let i = 0; i < 4; i += 1) {
        const x = i < 2 ? -6.5 : 6.5, z = i % 2 ? -2.6 : 2.6;
        bench(b, x, z, i % 2 ? 0 : 2);
      }
      for (const x of [-hw + 4, hw - 4]) { lampColumn(b, x, 2.4, x < 0 ? 1 : -1); litterBin(b, x + (x < 0 ? 2.0 : -2.0), -2.4); }
      // Hedge and railings to the street, with a gap on the path.
      for (const seg2 of [[-hw, -2.2], [2.2, hw]]) {
        b.box(seg2[0], seg2[1], 0, 1.0, hd - 1.0, hd - 0.35, SLOT.FOLIAGE, 0.95);
        railing(b, seg2[0], seg2[1], hd - 0.25, 0, 1.15, 0.28);
      }
      railing(b, -hw, hw, -hd + 0.25, 0, 1.15, 0.4);
      for (const x of [-2.2, 2.2]) b.box(x - 0.2, x + 0.2, 0, 1.6, hd - 0.55, hd - 0.15, SLOT.WALL, 1.02);
      b.box(-hw + 2, -hw + 6, 0.12, 0.14, -hd + 3, -hd + 8, SLOT.GROUND, 1.0);
      return "Original game art: representative temperate municipal park — crossing paths, bandstand, mature trees, railings. Scenery only.";
    },
    massing() { return []; },
    map(b, p) {
      const hw = p.w / 2, hd = p.d / 2;
      b.slab(-hw, hw, -hd, hd, 0.1, SLOT.FOLIAGE, 0.82);
      b.slab(-hw + 0.4, hw - 0.4, -1.4, 1.4, 0.13, SLOT.GROUND, 1.02);
      b.slab(-1.4, 1.4, -hd + 0.4, hd - 0.4, 0.13, SLOT.GROUND, 1.02);
      b.cyl(0, 0, 0.13, 2.8, 2.2, 1.4, 8, SLOT.TRIM, 1.05);
      for (let i = 0; i < 9; i += 1) {
        const a = (i / 9) * Math.PI * 2, x = Math.cos(a) * hw * 0.66, z = Math.sin(a) * hd * 0.66;
        b.cyl(x, z, 1.4, 7.6, 2.6, 0.4, 6, SLOT.FOLIAGE, 1.0);
        b.cyl(x, z, 0, 1.4, 0.35, 0.35, 4, SLOT.WALL, 0.6);
      }
      for (const sz of [-1, 1]) b.box(-hw, hw, 0, 1.1, sz * hd - 0.5, sz * hd, SLOT.FOLIAGE, 0.95);
      return b;
    },
  };

  // --------------------------------------------------------------- utility
  KINDS.utility = {
    label: "Neighbourhood utilities",
    widths: [14.0, 17.0], depth: 15.0, storeys: [1],
    ground: "paved",
    close(b, p) {
      const hw = p.w / 2, hd = p.d / 2;
      paving(b, -hw, hw, -hd, hd);
      // Brick switch house, a transformer with its radiators, a cable gantry
      // and the compound fence. Nothing here is a specification of anything.
      b.box(-hw + 0.6, -hw + 7.0, 0, 4.2, -hd + 5.6, hd - 1.2, SLOT.WALL, 1.0);
      b.box(-hw + 0.45, -hw + 7.15, 0, 0.5, -hd + 5.45, hd - 1.05, SLOT.TRIM, 0.9);
      b.push(0, -hw + 3.8, 0, (hd - 1.2 + (-hd + 5.6)) / 2);
      gableRoof(b, 3.2, (hd - 1.2 - (-hd + 5.6)) / 2, 4.2, 0.9, 0.3);
      b.pop();
      b.box(-hw + 2.4, -hw + 3.9, 0, 2.2, hd - 1.26, hd - 1.14, SLOT.DARK, 1.0);
      for (let i = 0; i < 2; i += 1) b.box(-hw + 1.0 + i * 3.6, -hw + 2.1 + i * 3.6, 2.5, 3.5, hd - 1.26, hd - 1.18, SLOT.METAL, 0.9);
      const tx = hw - 5.2, tz = 1.0;
      b.box(tx - 2.1, tx + 2.1, 0.35, 3.3, tz - 1.6, tz + 1.6, SLOT.METAL, 0.92);
      b.box(tx - 2.4, tx + 2.4, 0, 0.35, tz - 1.9, tz + 1.9, SLOT.GROUND, 1.0);
      for (let i = 0; i < 9; i += 1) b.box(tx - 2.2, tx + 2.2, 0.5, 3.1, tz + 1.6 + i * 0.14, tz + 1.66 + i * 0.14, SLOT.METAL, 1.06);
      for (let i = 0; i < 3; i += 1) {
        const x = tx - 1.2 + i * 1.2;
        tube(b, x, tz - 0.9, 3.3, 4.5, 0.24, 0.18, b.segs(10, 7, 5), SLOT.TRIM, 1.05);
        tube(b, x, tz - 0.9, 4.5, 4.7, 0.3, 0.3, b.segs(10, 7, 5), SLOT.DARK, 1.0);
      }
      for (const x of [tx - 3.2, tx + 3.2]) tube(b, x, tz + 3.0, 0, 7.0, 0.16, 0.13, b.segs(9, 6, 4), SLOT.METAL, 1.0);
      b.box(tx - 3.4, tx + 3.4, 7.0, 7.25, tz + 2.85, tz + 3.15, SLOT.METAL, 1.05);
      for (let i = 0; i < 3; i += 1) tube(b, tx - 2.2 + i * 2.2, tz + 3.0, 6.2, 7.0, 0.09, 0.09, b.segs(8, 6, 4), SLOT.DARK, 0.9);
      // A lattice pylon in the corner, four legs braced in six panels.
      b.push(0, hw - 5.0, 0, -hd + 3.0);
      for (const lx of [-1, 1]) for (const lz of [-1, 1]) {
        b.loft([[lx * 1.5 - 0.12, lz * 1.5 - 0.12], [lx * 1.5 + 0.12, lz * 1.5 - 0.12], [lx * 1.5 + 0.12, lz * 1.5 + 0.12], [lx * 1.5 - 0.12, lz * 1.5 + 0.12]],
          [[lx * 0.55 - 0.08, lz * 0.55 - 0.08], [lx * 0.55 + 0.08, lz * 0.55 - 0.08], [lx * 0.55 + 0.08, lz * 0.55 + 0.08], [lx * 0.55 - 0.08, lz * 0.55 + 0.08]],
          0, 15.0, SLOT.METAL, 1.0, false);
      }
      for (let k = 0; k <= 5; k += 1) {
        const y = k * 3.0, f = 1.5 - (1.5 - 0.55) * (y / 15.0);
        b.box(-f - 0.08, f + 0.08, y, y + 0.13, -f - 0.08, -f + 0.08, SLOT.METAL, 1.0);
        b.box(-f - 0.08, f + 0.08, y, y + 0.13, f - 0.08, f + 0.08, SLOT.METAL, 1.0);
        b.box(-f - 0.08, -f + 0.08, y, y + 0.13, -f, f, SLOT.METAL, 0.95);
        b.box(f - 0.08, f + 0.08, y, y + 0.13, -f, f, SLOT.METAL, 1.05);
      }
      for (let k = 0; k < 5; k += 1) {
        const y = k * 3.0, f = 1.5 - (1.5 - 0.55) * (y / 15.0), f2 = 1.5 - (1.5 - 0.55) * ((y + 3.0) / 15.0);
        for (const sz of [-1, 1]) {
          b.quad([-f, y, sz * f - 0.05], [f2, y + 3.0, sz * f2 - 0.05], [f2, y + 3.0, sz * f2 + 0.05], [-f, y, sz * f + 0.05], SLOT.METAL, sz > 0 ? 1.02 : 0.9);
        }
      }
      for (const k of [0, 1]) {
        const y = 11.0 + k * 3.2;
        b.box(-4.6, 4.6, y, y + 0.16, -0.12, 0.12, SLOT.METAL, 1.0);
        for (const x of [-4.3, 0, 4.3]) tube(b, x, 0, y - 0.9, y, 0.07, 0.07, b.segs(8, 6, 4), SLOT.DARK, 0.9);
      }
      b.pop();
      // Palisade fence around the compound, with warning plates.
      for (const sz of [-1, 1]) railing(b, -hw, hw, sz * (hd - 0.4), 0, 2.4, 0.22);
      for (const sx of [-1, 1]) {
        b.push(1, sx * (hw - 0.4), 0, 0);
        railing(b, -hd, hd, 0, 0, 2.4, 0.22);
        b.pop();
      }
      b.box(-1.2, -0.2, 1.3, 1.75, hd - 0.5, hd - 0.42, SLOT.TRIM, 1.08);
      b.box(0.4, 1.4, 1.3, 1.75, hd - 0.5, hd - 0.42, SLOT.TRIM, 1.08);
      return "Original game art: representative temperate distribution substation and compound. Scenery only — carries no rating, no capacity and no connection to the power model.";
    },
    massing(p) {
      const hw = p.w / 2, hd = p.d / 2;
      return [{ x0: -hw + 0.6, x1: -hw + 7.0, z0: -hd + 5.6, z1: hd - 1.2, h: 4.2, roof: "gable", rise: 0.9, bands: 1 }];
    },
    map(b, p) {
      const hw = p.w / 2, hd = p.d / 2;
      b.slab(-hw, hw, -hd, hd, 0.1, SLOT.GROUND, 1.0);
      drawMassing(b, KINDS.utility.massing(p), 1);
      const tx = hw - 5.2;
      b.box(tx - 2.4, tx + 2.4, 0, 3.3, -0.9, 2.9, SLOT.METAL, 0.95);
      b.box(tx - 3.4, tx + 3.4, 6.9, 7.25, 3.85, 4.15, SLOT.METAL, 1.05);
      for (const x of [tx - 3.2, tx + 3.2]) b.cyl(x, 4.0, 0, 7.0, 0.2, 0.16, 4, SLOT.METAL, 1.0);
      b.cyl(hw - 5.0, -hd + 3.0, 0, 15.0, 1.6, 0.6, 4, SLOT.METAL, 1.0);
      for (const k of [0, 1]) b.box(hw - 9.6, hw - 0.4, 11.0 + k * 3.2, 11.2 + k * 3.2, -hd + 2.88, -hd + 3.12, SLOT.METAL, 1.0);
      for (const sz of [-1, 1]) b.box(-hw, hw, 0, 2.3, sz * hd - 0.16, sz * hd, SLOT.METAL, 0.9);
      return b;
    },
  };

  // ------------------------------------------------------------- map scale
  // The map LOD is not a decimated LOD0 — it is authored from the SAME
  // massing table the close view is built around, so the silhouette a player
  // learns at map zoom is the silhouette they get when they zoom in. What it
  // keeps is what survives at that size: mass, roof form, storey rhythm and
  // one piece of roof furniture. What it drops is every opening, every piece
  // of trim and the whole streetscape below waist height.

  /// One glazing band at map scale, standing 50 mm PROUD of the wall rather
  /// than recessed into it. Six triangles against the punched recess's
  /// seventy-two, and at map size the two are the same dark line with the same
  /// shadow under it — the difference between a 50 mm projection and a 220 mm
  /// reveal is well under a pixel when a whole town fits on screen.
  ///
  /// It is proud and not recessed for a reason worth writing down: the recess
  /// this replaced was drawn BEHIND the massing box's own face and was
  /// therefore invisible, exactly like the close view's windows were. The two
  /// honest repairs are to cut the wall or to stand the band in front of it,
  /// and the map is the one level where the cheap one is also the right one,
  /// because the map budget is multiplied by every settlement on the screen.
  function mapBand(b, x0, x1, y0, h, zf) {
    const y1 = y0 + h, p = 0.05;
    b.quad([x0, y0, zf + p], [x1, y0, zf + p], [x1, y1, zf + p], [x0, y1, zf + p], SLOT.GLASS, 1.0);
    b.quad([x0, y1, zf + p], [x1, y1, zf + p], [x1, y1, zf], [x0, y1, zf], SLOT.TRIM, 1.06);
    b.quad([x0, y0, zf], [x1, y0, zf], [x1, y0, zf + p], [x0, y0, zf + p], SLOT.TRIM, 0.8);
    return b;
  }

  /// Detail level falls with the number of volumes a kind has, which is the
  /// only way a three-wing university and a single house both land inside the
  /// 100..800 triangle map budget without hand-tuning fifteen numbers.
  function drawMassing(b, vols, forceLevel) {
    const level = forceLevel != null ? forceLevel : (vols.length <= 1 ? 3 : vols.length <= 2 ? 2 : 1);
    const faces = level >= 2 ? 4 : 2;
    for (const v of vols) {
      const y0 = v.y0 || 0, y1 = v.h;
      const hw = (v.x1 - v.x0) / 2, hd = (v.z1 - v.z0) / 2;
      const cx = (v.x0 + v.x1) / 2, cz = (v.z0 + v.z1) / 2;
      b.push(0, cx, 0, cz);
      b.box(-hw, hw, y0, y1, -hd, hd, SLOT.WALL, 1.0);
      b.box(-hw - 0.18, hw + 0.18, y0, y0 + 0.55, -hd - 0.18, hd + 0.18, SLOT.TRIM, 0.9);
      const bands = Math.max(2, Math.min(v.bands || 2, level + 1));
      const fh = (y1 - y0) / bands;
      for (let f = 0; f < faces; f += 1) {
        b.push(f, 0, 0, 0);
        const run = (f % 2 === 0 ? hw : hd) - 0.9, zf = f % 2 === 0 ? hd : hw;
        if (run > 0.6) for (let s = 0; s < bands; s += 1) mapBand(b, -run, run, y0 + s * fh + fh * 0.3, fh * 0.42, zf);
        b.pop();
      }
      if (v.roof === "gable" || v.roof === "hip") {
        const top = y1 + (v.rise || hd * 0.6), rx = v.roof === "hip" ? Math.max(0.4, hw - hd * 0.85) : hw + 0.3;
        b.quad([-hw - 0.3, y1, hd + 0.3], [hw + 0.3, y1, hd + 0.3], [rx, top, 0], [-rx, top, 0], SLOT.ROOF, 1.0);
        b.quad([hw + 0.3, y1, -hd - 0.3], [-hw - 0.3, y1, -hd - 0.3], [-rx, top, 0], [rx, top, 0], SLOT.ROOF, 0.86);
        b.tri([-hw - 0.3, y1, -hd - 0.3], [-hw - 0.3, y1, hd + 0.3], [-rx, top, 0], SLOT.ROOF, 0.82);
        b.tri([hw + 0.3, y1, hd + 0.3], [hw + 0.3, y1, -hd - 0.3], [rx, top, 0], SLOT.ROOF, 0.9);
        b.box(-hw - 0.34, hw + 0.34, y1 - 0.2, y1, -hd - 0.34, hd + 0.34, SLOT.TRIM, 1.02);
        // A ridge line. Twelve triangles, and at map size it is the difference
        // between a roof that has a direction and a roof that is a wedge — the
        // ridge is the one line on a pitched roof that survives being two
        // pixels tall.
        b.box(-rx - 0.14, rx + 0.14, top - 0.13, top + 0.13, -0.2, 0.2, SLOT.ROOF, 1.12);
      } else {
        b.slab(-hw, hw, -hd, hd, y1, SLOT.GROUND, 0.86);
        b.box(-hw - 0.22, hw + 0.22, y1, y1 + (v.para || 0.8), -hd - 0.22, hd + 0.22, SLOT.WALL, 1.02);
      }
      // Where the building is entered, as a dark recessed panel on the street
      // elevation. At map zoom this is the only cue for which way a building
      // faces, and a settlement where every block faces nowhere reads as a
      // pattern rather than as a town.
      if (y1 - y0 > 3) {
        const dw = Math.min(1.6, hw * 0.4);
        b.box(-dw, dw, y0, y0 + Math.min(2.6, (y1 - y0) * 0.55), hd - 0.16, hd + 0.02, SLOT.DARK, 1.0);
      }
      if (v.chimneys) {
        for (let i = 0; i < Math.min(4, v.chimneys.length); i += 1) {
          const x = v.chimneys[Math.floor((i * v.chimneys.length) / Math.min(4, v.chimneys.length))] - cx;
          b.box(x - 0.5, x + 0.5, y1, y1 + (v.rise || 2) + 0.9, -0.4, 0.4, SLOT.WALL, 1.04);
        }
      }
      if (v.plant) {
        b.box(-hw * 0.42, -hw * 0.06, y1, y1 + 2.6, -hd * 0.34, hd * 0.2, SLOT.WALL, 0.94);
        b.box(hw * 0.16, hw * 0.6, y1, y1 + 1.5, -hd * 0.5, -hd * 0.06, SLOT.METAL, 0.9);
      }
      b.pop();
    }
    return b;
  }

  // ------------------------------------------------------------- variants
  // The cache is the reuse. A key names everything the geometry depends on,
  // so two lots that agree on kind, footprint, storeys and LOD share one baked
  // mesh and differ only in where they stand and what they are faced in.
  // Nothing here depends on call order, so a cache hit and a cache miss
  // produce the same block.
  const SEEDED = { park: true };
  const variants = new Map();

  function normaliseLod(v) {
    // See the matching note in site-mesh.js: "far" is that module's word for the
    // coarse mesh and "map" is this one's. Accepting both in both is the whole
    // fix; a caller that says the wrong word used to get the 20x heavier mesh
    // with no complaint.
    //
    // THE NUMERIC FORMS STILL MEAN THE COARSE MESH. `mid` is a third level and
    // it had to be given a word of its own rather than take 1 off `map`: the
    // two art modules agreed that 1, 2, "lod1", "lod2", "far" and "map" all
    // name the cheap mesh, and quietly redefining 1 here would have handed
    // every existing caller a mesh eight times heavier than the one it asked
    // for, silently, which is the exact failure that agreement was written to
    // end.
    if (v === "map" || v === "far" || v === 1 || v === 2 || v === "lod1" || v === "lod2") return "map";
    if (v === "mid" || v === "card") return "mid";
    return "close";
  }
  const DETAIL = { close: 2, mid: 1, map: 0 };
  function quantise(x) { return Math.round(x * 10) / 10; }
  // How far a roof may legitimately hang past the land it was allotted. Eaves,
  // cornices and window surrounds all do it, a terrace where they did not would
  // read as a row of separate boxes, and the layout leaves gaps for it. It is
  // the allowance for TRIM, not a licence for a wing: anything past it is a
  // building standing somewhere it was not given.
  const EAVE = 1.5;

  function paramsFor(kind, seed, o) {
    const spec = KINDS[kind];
    const w = quantise(o && o.width != null ? o.width : askOne(seed, 3, spec.widths));
    const d = quantise(o && o.depth != null ? o.depth : spec.depth);
    const storeys = Math.max(1, (o && o.storeys != null ? o.storeys : askOne(seed, 5, spec.storeys)) | 0);
    return { w, d, storeys, seed: SEEDED[kind] ? seed : 0 };
  }

  function variantFor(kind, p, lod) {
    const key = kind + "|" + p.w + "|" + p.d + "|" + p.storeys + "|" + lod + "|" + p.seed;
    const hit = variants.get(key);
    if (hit) return hit;
    const b = new Builder(DETAIL[lod]);
    const spec = KINDS[kind];
    let description;
    if (lod === "map") {
      // The lot ground goes in at map scale too. It is two triangles, it keeps
      // the map root on the same footprint centre as the close view so the two
      // swap in place rather than sliding, and the colour of the plot is what
      // separates a residential street from a commercial one at that size.
      if (spec.map) spec.map(b, p);
      else {
        if (spec.ground === "grass") grass(b, -p.w / 2, p.w / 2, -p.d / 2, p.d / 2, 0.08);
        else paving(b, -p.w / 2, p.w / 2, -p.d / 2, p.d / 2, 0.08);
        drawMassing(b, spec.massing(p));
      }
      description = "Original game art: " + spec.label.toLowerCase()
        + " at map scale — representative temperate scenery reduced to massing, roof form and storey rhythm.";
    } else {
      description = spec.close(b, p);
      // The card LOD is the SAME building, drawn by the same code with the
      // small stuff switched off, so it can never become a different building
      // by drifting. Say so rather than let a card claim the close view's
      // description word for word.
      if (lod === "mid") {
        description = description.replace(/Massing only/i, "Card scale")
          + " Card LOD: chamfers, glazing bars, slate courses, tree limbs and window frames are dropped; "
          + "the massing, openings and roof form are the close view's own.";
      }
    }
    b.flushFaces();
    const baked = b.bake();
    baked.key = key; baked.kind = kind; baked.label = spec.label; baked.description = description;
    baked.footprint = [p.w, p.d]; baked.storeys = p.storeys;
    // `footprint` is what the layout ASKED for. Eight of the fifteen kinds draw
    // at their own fixed depth and quietly ignore the depth in `p` — a
    // university is 40 m deep whatever room a block has for it. So measure what
    // was actually drawn and carry it too, because a caller deciding whether a
    // building fits must ask the mesh rather than the wish. Without this the
    // block anchor recorded a clamped request as the lot while the mesh ran
    // 22 m past it and through the buildings behind.
    let ex0 = Infinity, ex1 = -Infinity, ez0 = Infinity, ez1 = -Infinity;
    for (let i = 0; i < baked.pos.length; i += 3) {
      const x = baked.pos[i], z = baked.pos[i + 2];
      if (x < ex0) ex0 = x;
      if (x > ex1) ex1 = x;
      if (z < ez0) ez0 = z;
      if (z > ez1) ez1 = z;
    }
    baked.extent = baked.tris ? [ex1 - ex0, ez1 - ez0] : [0, 0];
    variants.set(key, baked);
    return baked;
  }

  // -------------------------------------------------------------- district
  // Which kinds a frontage may draw from. A block is not a random bag: a
  // temperate high street is shops with flats over and the odd civic front,
  // and a residential street is houses and terraces with one utility corner.
  // The tables ARE the urbanism, and they are the cheapest thing here to
  // change when a settlement does not look right.
  const DISTRICTS = {
    mixed: {
      front: ["row_house", "row_house", "shop", "shop", "house", "low_apartment"],
      flank: ["house", "shop", "utility", "row_house"],
      anchor: ["park", "school"],
    },
    residential: {
      front: ["house", "row_house", "row_house", "house", "low_apartment"],
      flank: ["house", "row_house", "utility"],
      anchor: ["park", "school"],
    },
    commercial: {
      front: ["shop", "shop", "office", "mid_apartment", "shop"],
      flank: ["shop", "office", "utility"],
      anchor: ["office", "park"],
    },
    civic: {
      front: ["civic", "low_apartment", "shop", "school"],
      flank: ["park", "shop", "utility"],
      anchor: ["university", "hospital", "park"],
    },
    industrial: {
      front: ["warehouse", "warehouse", "office", "utility"],
      flank: ["utility", "office"],
      anchor: ["warehouse", "utility"],
    },
  };
  const DEFAULT_SIZE = [148, 104];

  function clamp(v, lo, hi) { return v < lo ? lo : v > hi ? hi : v; }

  /// Draw a kind that fits. Six hashed attempts and then give up — a frontage
  /// that cannot be filled ends short rather than shrinking a building to fit,
  /// because a squeezed building is the thing that makes a generated town look
  /// generated.
  function pickKind(seed, salt, table, depthCap, runLeft) {
    for (let attempt = 0; attempt < 6; attempt += 1) {
      const kind = askOne(seed, salt + attempt * 31, table);
      const spec = KINDS[kind];
      if (spec.depth <= depthCap && Math.min.apply(null, spec.widths) <= runLeft) return kind;
    }
    return null;
  }

  /// A frontage gap is not decoration, it is clearance, and it has to be wider
  /// than the trim standing on both sides of it. Measured across 225 blocks,
  /// the widest anything in this kit hangs past its own lot line is 0.70 m —
  /// the shop's fascia — with the civic portico at 0.59 and the school's eaves
  /// at 0.55, so 1.6 m of gap leaves at least 0.20 m of daylight in the worst
  /// pairing this kit can produce.
  ///
  /// WHY THE FLOOR EXISTS. Before it, the gap was only ever the leftover run
  /// divided up, and a tightly packed frontage could close it to 1.0 m: block
  /// id 9's civic district stood a school and a civic building 50 mm into each
  /// other over a 21 m run. The neighbour check would have caught it — its bar
  /// is 50 mm and this was 50.4 — and it stayed green only because id 9 is not
  /// one of the eleven ids it samples. The repair is the geometry, not the bar.
  const MIN_GAP = 1.6;

  /// One frontage, subdivided along its run. Leftover is spread as equal gaps
  /// and the whole row is centred, so a run that does not divide evenly reads
  /// as side entries and alley gates rather than as a mistake at one end. When
  /// the run is too tight for that to clear MIN_GAP, the interior gaps are paid
  /// first and the two end margins take what is left: a building standing hard
  /// against the block corner is a corner shop, two buildings sharing the same
  /// air is a defect.
  function subdivide(seed, salt, run, table, depthCap) {
    const picks = [];
    let used = 0;
    for (let i = 0; i < 24; i += 1) {
      // The reserve is one MIN_GAP per interior gap the row will have once this
      // pick joins it, which is exactly what the floor below needs to be payable.
      const left = run - used - picks.length * MIN_GAP;
      const kind = pickKind(seed, salt + i * 101, table, depthCap, left);
      if (!kind) break;
      const spec = KINDS[kind];
      let w = 0;
      for (const cand of spec.widths) if (cand <= left && cand > w) w = cand;
      if (w <= 0) break;
      picks.push({ kind, w, storeys: askOne(seed, salt + i * 101 + 7, spec.storeys), depth: spec.depth });
      used += w;
    }
    if (!picks.length) return picks;
    const spare = run - used;
    const gap = picks.length > 1 ? clamp(spare / (picks.length + 1), MIN_GAP, 7.0) : 0;
    // Never negative: the packer above reserved MIN_GAP per interior gap, so
    // `spare >= gap * (picks.length - 1)` whichever arm of the clamp won.
    let cursor = (spare - gap * (picks.length - 1)) / 2;
    for (const pick of picks) { pick.at = cursor + pick.w / 2; cursor += pick.w + gap; }
    return picks;
  }

  // ----------------------------------------------------------- streetscape
  // Kerbs, footways, markings and furniture. Drawn straight into the block
  // rather than cached, because none of it repeats exactly and all of it is
  // cheap. The finish scheme is pinned to zero here: tarmac and a white edge
  // line do not change when the bricks do.
  function streetscape(b, g, lod) {
    b.scheme = 0;
    const X0 = g.X0, X1 = g.X1, Z0 = g.Z0, Z1 = g.Z1, hw = g.w / 2, hd = g.d / 2, foot = g.foot;
    b.slab(-hw, hw, -hd, hd, 0, SLOT.GROUND, 1.0);                                   // carriageway
    b.slab(X0 - foot, X1 + foot, Z0 - foot, Z1 + foot, 0.06, SLOT.FOLIAGE, 0.66);     // block land
    // Footways, and the kerb upstand that separates them from the road.
    b.slab(X0 - foot, X1 + foot, Z1, Z1 + foot, 0.14, SLOT.GROUND, 1.18);
    b.slab(X0 - foot, X1 + foot, Z0 - foot, Z0, 0.14, SLOT.GROUND, 1.18);
    b.slab(X0 - foot, X0, Z0, Z1, 0.14, SLOT.GROUND, 1.18);
    b.slab(X1, X1 + foot, Z0, Z1, 0.14, SLOT.GROUND, 1.18);
    // KERBS ARE A BULLNOSE, not a step. The chamfer along the top arris is
    // 25 mm of geometry that runs the whole perimeter of the block, and it is
    // the line the eye follows to read where the road stops — the one edge in
    // a street that is always lit from one side and shadowed on the other.
    b.bevelBox(X0 - foot, X1 + foot, 0, 0.15, Z1 + foot - 0.17, Z1 + foot, SLOT.TRIM, 1.02, 0.025);
    b.bevelBox(X0 - foot, X1 + foot, 0, 0.15, Z0 - foot, Z0 - foot + 0.17, SLOT.TRIM, 1.02, 0.025);
    b.bevelBox(X0 - foot, X0 - foot + 0.17, 0, 0.15, Z0 - foot, Z1 + foot, SLOT.TRIM, 1.02, 0.025);
    b.bevelBox(X1 + foot - 0.17, X1 + foot, 0, 0.15, Z0 - foot, Z1 + foot, SLOT.TRIM, 1.02, 0.025);
    // The gutter channel: one course of setts laid flat against the kerb,
    // slightly darker and slightly proud. It is what stops the road and the
    // footway meeting as one flat tone.
    if (b.detail >= 2) {
      for (const z of [Z1 + foot - 0.17, Z0 - foot + 0.17]) {
        b.slab(X0 - foot, X1 + foot, z - (z > 0 ? 0.42 : 0), z + (z > 0 ? 0 : 0.42), 0.012, SLOT.GROUND, 0.86);
      }
      for (const x of [X0 - foot + 0.17, X1 + foot - 0.17]) {
        b.slab(x - (x > 0 ? 0.42 : 0), x + (x > 0 ? 0 : 0.42), Z0 - foot, Z1 + foot, 0.012, SLOT.GROUND, 0.86);
      }
    }
    // Edge lines, inset from the tile boundary so two abutting tiles do not
    // paint the same line twice down the middle of one carriageway.
    for (const z of [hd - 0.6, -hd + 0.6]) b.box(X0 - foot, X1 + foot, 0.001, 0.02, z - 0.06, z + 0.06, SLOT.TRIM, 1.1);
    for (const x of [hw - 0.6, -hw + 0.6]) b.box(x - 0.06, x + 0.06, 0.001, 0.02, Z0 - foot, Z1 + foot, SLOT.TRIM, 1.1);
    if (lod === "map") {
      for (let i = 0; i < 10; i += 1) {
        const x = X0 + ((X1 - X0) * (i + 0.5)) / 10;
        for (const z of [Z1 + foot * 0.55, Z0 - foot * 0.55]) {
          b.cyl(x, z, 1.1, 6.6, 2.1, 0.35, 6, SLOT.FOLIAGE, 1.0);
          b.cyl(x, z, 0, 1.1, 0.3, 0.3, 4, SLOT.WALL, 0.6);
        }
      }
      return b;
    }
    // A pedestrian crossing on the north side, which is also the thing that
    // gives the block a legible front.
    for (let i = 0; i < 7; i += 1) {
      const x = -3.6 + i * 1.15;
      b.box(x - 0.34, x + 0.34, 0.001, 0.02, Z1 + foot, hd, SLOT.TRIM, 1.1);
    }
    // Dropped kerb across the crossing, tactile paving each side of it, and a
    // pair of guard rails. A crossing that runs into a 150 mm upstand is the
    // sort of thing that is invisible until you notice it and then is all you
    // can see.
    if (b.detail >= 2) {
      b.bevelBox(-4.4, 4.4, 0, 0.055, Z1 + foot - 0.17, Z1 + foot, SLOT.TRIM, 1.06, 0.02);
      for (const sx of [-1, 1]) {
        b.slab(sx * 4.4 - (sx > 0 ? 1.2 : 0), sx * 4.4 + (sx > 0 ? 0 : 1.2), Z1 + foot - 1.3, Z1 + foot - 0.2, 0.152, SLOT.GROUND, 0.94);
        railing(b, Math.min(sx * 4.6, sx * 7.4), Math.max(sx * 4.6, sx * 7.4), Z1 + foot - 0.32, 0.15, 1.05, 0.3);
      }
    }
    for (let i = 0; i < 10; i += 1) {
      const x = X0 + ((X1 - X0) * (i + 0.5)) / 10;
      if (Math.abs(x) > 6) {
        tree(b, x, Z1 + foot * 0.55, 6.8 + (i % 3) * 0.7, ((i % 5) - 2) * 0.12, 6);
        b.slab(x - 0.7, x + 0.7, Z1 + foot * 0.55 - 0.7, Z1 + foot * 0.55 + 0.7, 0.15, SLOT.DARK, 1.0);
      }
      tree(b, x, Z0 - foot * 0.55, 6.4 + (i % 4) * 0.6, ((i % 3) - 1) * 0.15, 6);
      b.slab(x - 0.7, x + 0.7, Z0 - foot * 0.55 - 0.7, Z0 - foot * 0.55 + 0.7, 0.15, SLOT.DARK, 1.0);
    }
    for (let i = 0; i < 6; i += 1) {
      const x = X0 + ((X1 - X0) * (i + 0.5)) / 6;
      lampColumn(b, x, Z1 + foot - 0.7, -1);
      lampColumn(b, x, Z0 - foot + 0.7, 1);
    }
    for (let i = 0; i < 3; i += 1) {
      const z = Z0 + ((Z1 - Z0) * (i + 0.5)) / 3;
      lampColumn(b, X1 + foot - 0.7, z, -1);
      lampColumn(b, X0 - foot + 0.7, z, 1);
    }
    busShelter(b, X0 + 16, Z1 + foot * 0.5, 2);
    kiosk(b, X1 - 8, Z1 + foot * 0.55, 2);
    bench(b, -12, Z1 + foot * 0.5, 2);
    bench(b, 12, Z1 + foot * 0.5, 2);
    bench(b, X0 + 6, Z0 - foot * 0.5, 0);
    litterBin(b, -8.6, Z1 + foot * 0.5);
    litterBin(b, 8.6, Z1 + foot * 0.5);
    litterBin(b, X0 + 3, Z0 - foot * 0.5);
    for (const x of [-5.4, 5.4]) { bollard(b, x, Z1 + foot - 0.5); bollard(b, x, Z1 + 0.6); }
    roadSign(b, X0 + 1.4, Z1 + foot - 0.6, 2.6);
    roadSign(b, X1 - 1.4, Z0 - foot + 0.6, 2.6);
    roadSign(b, X1 - 1.4, Z1 + foot - 0.6, 2.2);
    // Cars at the kerb, staggered so the two sides do not line up. Six of
    // them is what a hundred-metre frontage in 1990 holds, and they are the
    // only moving-scale object in the whole block: everything else here is
    // furniture, and furniture is the same size everywhere.
    const PAINT = [SLOT.CAR_A, SLOT.CAR_B, SLOT.CAR_C];
    for (let i = 0; i < 4; i += 1) {
      const x = X0 + 9 + i * ((X1 - X0 - 18) / 3);
      if (Math.abs(x) > 8) parkedCar(b, x, Z1 + foot + 1.15, 0, PAINT[i % 3], 3.9 + (i % 2) * 0.35);
    }
    for (let i = 0; i < 3; i += 1) {
      const x = X0 + 14 + i * ((X1 - X0 - 28) / 2);
      parkedCar(b, x, Z0 - foot - 1.15, 0, PAINT[(i + 2) % 3], 4.0 + (i % 2) * 0.3);
    }
    // A utility cabinet and a grit bin, because a pavement is never only
    // lamps and trees. Both are generic boxes with no operator on them.
    b.bevelBox(X0 + 5.4, X0 + 6.5, 0.15, 1.42, Z1 + foot - 0.95, Z1 + foot - 0.55, SLOT.METAL, 0.95, 0.025);
    b.bevelBox(X1 - 7.2, X1 - 6.2, 0.15, 0.86, Z0 - foot + 0.5, Z0 - foot + 1.15, SLOT.TRIM, 0.9, 0.03);
    return b;
  }

  /// The block core: a service lane and lock-up garages behind the frontages,
  /// used when the leftover land is too small or the wrong shape for an
  /// anchor. Rear land that is simply left as grass is what makes a generated
  /// block look hollow from a low camera.
  function serviceCore(b, x0, x1, z0, z1, seed) {
    b.scheme = ask(seed, 771) % SCHEMES.length;
    const cz = (z0 + z1) / 2;
    b.slab(x0, x1, cz - 3.0, cz + 3.0, 0.09, SLOT.GROUND, 1.05);
    const n = Math.max(1, Math.floor((x1 - x0 - 4) / 3.2));
    for (let i = 0; i < n; i += 1) {
      const x = x0 + 2.0 + i * 3.2 + 1.6;
      if (x + 1.6 > x1) break;
      garage(b, x, z0 + 2.9, 2, 3.1);
    }
    for (let i = 0; i < 4; i += 1) {
      const x = x0 + ((x1 - x0) * (i + 0.5)) / 4;
      if (z1 - z0 > 14) tree(b, x, z1 - 2.6, 6.0 + (i % 2) * 1.2, (i % 3 - 1) * 0.2, 6);
    }
    for (const x of [x0 + 1.2, x1 - 1.2]) b.box(x - 0.2, x + 0.2, 0, 1.9, z0, z1, SLOT.WALL, 0.95);
    return b;
  }

  // ------------------------------------------------------------------ block
  /// Assemble one block. The tile carries HALF a street corridor on each edge,
  /// so two tiles laid side by side make one full carriageway between them and
  /// a single tile still reads correctly on its own. That is a deliberate
  /// compromise for a P0 whose job is to be looked at as one block.
  function block(opts) {
    const o = opts || {};
    const seed = seedOf(o.id == null ? 0 : o.id);
    const district = DISTRICTS[o.district] ? o.district : "mixed";
    const table = DISTRICTS[district];
    const size = o.size || DEFAULT_SIZE;
    const w = clamp(Number(size[0]) || DEFAULT_SIZE[0], 70, 420);
    const d = clamp(Number(size[1]) || DEFAULT_SIZE[1], 60, 320);
    const street = clamp(o.street == null ? 13 : Number(o.street) || 13, 9, 26);
    const lod = normaliseLod(o.lod);
    const margin = street / 2, foot = margin * 0.42;
    const X0 = -w / 2 + margin, X1 = w / 2 - margin, Z0 = -d / 2 + margin, Z1 = d / 2 - margin;

    const out = new Builder(DETAIL[lod]);
    const lots = [];
    const used = new Map();
    let tris = 0, lotIndex = 0;

    function place(kind, params, yaw, x, z, label) {
      const p = { w: quantise(params.w), d: quantise(params.d), storeys: params.storeys,
        seed: SEEDED[kind] ? mix32(seed ^ Math.imul(lotIndex + 1, 0x27d4eb2f)) : 0 };
      const baked = variantFor(kind, p, lod);
      const scheme = ask(seed, 900 + lotIndex * 5) % SCHEMES.length;
      out.part(label + " / " + baked.label, kind, lotIndex, () => {
        tris += stampInto(out, baked, yaw, x, z, scheme);
      });
      used.set(baked.key, (used.get(baked.key) || 0) + 1);
      lots.push({ index: lotIndex, kind, label: baked.label, x: quantise(x), z: quantise(z), yaw,
        width: p.w, depth: p.d, storeys: p.storeys, scheme: SCHEMES[scheme].id, variant: baked.key });
      lotIndex += 1;
    }

    // North and south frontages first: they set how much depth the flanks and
    // the core have left to work with.
    const north = subdivide(seed, 11, X1 - X0, table.front, (Z1 - Z0) * 0.42);
    const south = subdivide(seed, 4007, X1 - X0, table.front, (Z1 - Z0) * 0.42);
    let deepN = 0, deepS = 0;
    for (const pick of north) deepN = Math.max(deepN, pick.depth);
    for (const pick of south) deepS = Math.max(deepS, pick.depth);
    for (const pick of north) place(pick.kind, { w: pick.w, d: pick.depth, storeys: pick.storeys }, 0, X0 + pick.at, Z1 - pick.depth / 2, "north frontage");
    for (const pick of south) place(pick.kind, { w: pick.w, d: pick.depth, storeys: pick.storeys }, 2, X0 + pick.at, Z0 + pick.depth / 2, "south frontage");

    const coreZ0 = Z0 + deepS, coreZ1 = Z1 - deepN, coreRun = coreZ1 - coreZ0;
    let deepE = 0, deepW = 0;
    if (coreRun > 22) {
      const east = subdivide(seed, 8009, coreRun, table.flank, (X1 - X0) * 0.3);
      const west = subdivide(seed, 9011, coreRun, table.flank, (X1 - X0) * 0.3);
      for (const pick of east) deepE = Math.max(deepE, pick.depth);
      for (const pick of west) deepW = Math.max(deepW, pick.depth);
      for (const pick of east) place(pick.kind, { w: pick.w, d: pick.depth, storeys: pick.storeys }, 1, X1 - pick.depth / 2, coreZ0 + pick.at, "east flank");
      for (const pick of west) place(pick.kind, { w: pick.w, d: pick.depth, storeys: pick.storeys }, 3, X0 + pick.depth / 2, coreZ0 + pick.at, "west flank");
    }

    // Whatever is left in the middle. An anchor if one fits; a service lane
    // and lock-ups if not.
    const inX0 = X0 + deepW + 2.5, inX1 = X1 - deepE - 2.5;
    const inZ0 = coreZ0 + 2.5, inZ1 = coreZ1 - 2.5;
    let anchored = null;
    if (inX1 - inX0 > 16 && inZ1 - inZ0 > 14) {
      const roomX = inX1 - inX0, roomZ = inZ1 - inZ0;
      for (const kind of table.anchor) {
        const spec = KINDS[kind];
        let best = 0;
        for (const cand of spec.widths) if (cand <= roomX && cand > best) best = cand;
        if (best <= 0) continue;
        const depth = quantise(Math.min(spec.depth, roomZ));
        if (depth <= 12) continue;
        const storeys = askOne(seed, 613, spec.storeys);
        // Bake the candidate and measure it before committing the lot. The gate
        // here used to be the CLAMPED REQUEST — `min(spec.depth, room)` — which
        // let a 40 m university onto 18 m of interior and recorded the lot as
        // 18 m deep, so the lot table said no overlap while the mesh ploughed
        // 22 m into the school behind it (540 m^2 of interpenetration on the
        // worst civic block). A kind that cannot honour the depth asked of it
        // is not a smaller building, it is the same building in the wrong
        // place, so reject it and try the next anchor the district offers.
        //
        // AND THE PROBE IS ALWAYS THE CLOSE VARIANT, whatever level is being
        // built. It used to be the level in hand, which made the LAYOUT a
        // function of the LOD: a map-scale hospital has no gutters, no eaves
        // overhang and no portico, so it measures a little smaller and passes a
        // gate the close-scale one fails. Measured over 1,000 blocks that put a
        // hospital in the middle of civic blocks 2, 142 and 198 at map zoom and
        // a park in the same ground the moment the camera came in — the exact
        // pop this kit exists to avoid. The close variant is the biggest the
        // building ever gets, so a fit measured there is a fit at every level.
        // It costs one extra bake per distinct anchor candidate per process,
        // shared through the same variant cache as everything else.
        const probe = variantFor(kind, { w: quantise(best), d: depth, storeys,
          seed: SEEDED[kind] ? mix32(seed ^ Math.imul(lotIndex + 1, 0x27d4eb2f)) : 0 }, "close");
        if (probe.extent[0] > roomX + EAVE || probe.extent[1] > roomZ + EAVE) continue;
        place(kind, { w: best, d: depth, storeys },
          0, (inX0 + inX1) / 2, (inZ0 + inZ1) / 2, "block interior");
        anchored = kind;
        break;
      }
    }
    if (!anchored && inX1 - inX0 > 12 && inZ1 - inZ0 > 8) {
      out.part("block interior / service lane and lock-ups", "service", -1, () => {
        serviceCore(out, inX0, inX1, inZ0, inZ1, seed);
      });
    }
    // Rear boundaries. The land behind a frontage is gardens and yards, and
    // left as one unbroken sheet of grass it is the thing that makes a block
    // read as a model of a block rather than as a place: from any camera above
    // the eaves the middle of the tile is most of what you see. A hedge line
    // behind each frontage, with a gap for the path, divides it for about two
    // thousand triangles. It stays clear of the flank lots and of whatever the
    // interior holds, so it cannot clash with either.
    const bandX0 = X0 + deepW + 1.0, bandX1 = X1 - deepE - 1.0;
    if (out.detail >= 1 && bandX1 - bandX0 > 16 && coreRun > 10) {
      out.part("block interior / rear boundaries", "boundary", -1, () => {
        out.scheme = ask(seed, 881) % SCHEMES.length;
        for (const side of [1, -1]) {
          const z = side > 0 ? coreZ1 - 1.05 : coreZ0 + 1.05;
          const runs = 5, span = (bandX1 - bandX0) / runs;
          for (let i = 0; i < runs; i += 1) {
            if (i === 2) continue;                                  // the way through
            const x0 = bandX0 + i * span + 0.5, x1 = bandX0 + (i + 1) * span - 0.5;
            if (ask(seed, 1200 + i * 7 + (side > 0 ? 0 : 3)) % 3 === 0) {
              lowWall(out, x0, x1, z - 0.16, z + 0.16, 0.95);
            } else {
              hedge(out, x0, x1, z - 0.34, z + 0.34, 1.15 + (i % 2) * 0.18);
            }
          }
          for (const x of [bandX0 + span * 1.5, bandX0 + span * 3.5]) {
            tree(out, x, z - side * 2.2, 5.4 + (Math.abs(x) % 3) * 0.4, side * 0.2, 6);
          }
        }
      });
    }
    out.part("streetscape / carriageway, footways and furniture", "street", -1, () => {
      streetscape(out, { X0, X1, Z0, Z1, w, d, foot }, lod);
    });

    const variantList = [];
    for (const [key, count] of used) {
      const baked = variants.get(key);
      variantList.push({ key, kind: baked.kind, triangleCount: baked.tris, uses: count });
    }
    variantList.sort((a, c) => (a.key < c.key ? -1 : a.key > c.key ? 1 : 0));

    return out.finish(
      "Original game art: representative temperate town block, " + district + " district, " + lots.length
      + " lots assembled from " + variantList.length + " shared building meshes. Layout is representative scenery, "
      + "not a reconstruction of any real street, and grants no simulation capability.",
      {
        blockId: o.id == null ? 0 : o.id,
        // `tile` and not `size`: `size` is the mesh extent every other mesh in
        // this game reports, and the tile is the land the block occupies.
        district, lod, tile: [w, d], streetWidth: street,
        lots, variants: variantList,
        lotCount: lots.length,
        variantCount: variantList.length,
        anchor: anchored,
        buildingTriangles: tris,
      },
    );
  }

  // --------------------------------------------------------------- exports
  /// One building on its own lot, centred on its footprint at grade. Same
  /// variant cache as the block, so inspecting a building and then looking at
  /// the block it came from costs the geometry once.
  function building(kind, opts) {
    if (!Object.prototype.hasOwnProperty.call(KINDS, kind)) return null;
    const o = opts || {};
    const seed = mix32(seedOf(o.id == null ? 0 : o.id) ^ Math.imul((o.lot | 0) + 1, 0x27d4eb2f));
    const p = paramsFor(kind, seed, o);
    const lod = normaliseLod(o.lod);
    const baked = variantFor(kind, p, lod);
    const scheme = o.scheme != null && SCHEMES[o.scheme | 0] ? (o.scheme | 0) : ask(seed, 900) % SCHEMES.length;
    const out = new Builder(DETAIL[lod]);
    out.part(baked.label, kind, 0, () => { stampInto(out, baked, 0, 0, 0, scheme); });
    return out.finish(baked.description, {
      kind, lod, scheme: SCHEMES[scheme].id,
      storeys: p.storeys, footprint: baked.footprint,
      variantKey: baked.key, label: baked.label,
    });
  }

  function kinds() { return Object.keys(KINDS); }
  function districtNames() { return Object.keys(DISTRICTS); }
  function kindInfo(kind) {
    const spec = KINDS[kind];
    if (!spec) return null;
    return { kind, label: spec.label, widths: spec.widths.slice(), depth: spec.depth, storeys: spec.storeys.slice() };
  }

  return Object.freeze({
    block, building, kinds, kindInfo,
    districts: districtNames,
    schemes: SCHEMES.map((s) => s.id),
    version: "urban.temperate_block.v1",
    kit: "temperate_masonry",
    era: 1990,
    STOREY,
    defaultTile: DEFAULT_SIZE.slice(),
  });
});
