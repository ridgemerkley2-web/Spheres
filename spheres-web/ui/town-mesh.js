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

  const SLOT = { WALL: 0, ROOF: 1, TRIM: 2, GLASS: 3, GROUND: 4, FOLIAGE: 5, METAL: 6, DARK: 7 };

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
  const FIXED = [
    null, null, null,
    [0.20, 0.26, 0.30],   // GLASS  — 1990 glazing is a dark hole, not a mirror
    [0.25, 0.25, 0.26],   // GROUND — carriageway, paving, yards
    [0.28, 0.40, 0.23],   // FOLIAGE
    [0.53, 0.55, 0.57],   // METAL
    [0.10, 0.11, 0.12],   // DARK   — openings, rubber, shadow gaps
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

  function Builder() {
    this.pos = []; this.nrm = []; this.slot = []; this.mat = []; this.sch = [];
    this.tf = { c: 1, s: 0, x: 0, y: 0, z: 0 };
    this.stack = [];
    this.scheme = 0;
    this.parts = [];
  }
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
  Builder.prototype.slab = function (x0, x1, z0, z1, y, slot, mat) {
    return this.quad([x0, y, z0], [x1, y, z0], [x1, y, z1], [x0, y, z1], slot, mat);
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
  function sash(b, cx, y0, w, h, zf, cols, rows) {
    const hw = w / 2, y1 = y0 + h, dep = 0.17;
    b.box(cx - hw - 0.13, cx + hw + 0.13, y0 - 0.13, y0, zf - 0.02, zf + 0.10, SLOT.TRIM, 1.02);          // sill
    b.box(cx - hw - 0.13, cx + hw + 0.13, y1, y1 + 0.16, zf - 0.02, zf + 0.06, SLOT.TRIM, 1.0);           // head
    b.box(cx - hw - 0.13, cx - hw, y0, y1, zf - 0.02, zf + 0.05, SLOT.TRIM, 0.98);                        // jambs
    b.box(cx + hw, cx + hw + 0.13, y0, y1, zf - 0.02, zf + 0.05, SLOT.TRIM, 0.98);
    // Reveal: four inward-facing quads from the wall plane back to the glass.
    b.quad([cx - hw, y0, zf], [cx + hw, y0, zf], [cx + hw, y0, zf - dep], [cx - hw, y0, zf - dep], SLOT.WALL, 0.72);
    b.quad([cx - hw, y1, zf - dep], [cx + hw, y1, zf - dep], [cx + hw, y1, zf], [cx - hw, y1, zf], SLOT.WALL, 0.72);
    b.quad([cx - hw, y0, zf - dep], [cx - hw, y1, zf - dep], [cx - hw, y1, zf], [cx - hw, y0, zf], SLOT.WALL, 0.8);
    b.quad([cx + hw, y0, zf], [cx + hw, y1, zf], [cx + hw, y1, zf - dep], [cx + hw, y0, zf - dep], SLOT.WALL, 0.8);
    b.quad([cx - hw, y0, zf - dep], [cx + hw, y0, zf - dep], [cx + hw, y1, zf - dep], [cx - hw, y1, zf - dep], SLOT.GLASS, 1.0);
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
    b.box(x0 - 0.1, x1 + 0.1, y0 - 0.12, y0, zf - 0.02, zf + 0.08, SLOT.TRIM, 1.02);
    b.box(x0 - 0.1, x1 + 0.1, y1, y1 + 0.14, zf - 0.02, zf + 0.05, SLOT.TRIM, 1.0);
    b.quad([x0, y0, zf], [x1, y0, zf], [x1, y0, zf - dep], [x0, y0, zf - dep], SLOT.WALL, 0.72);
    b.quad([x0, y1, zf - dep], [x1, y1, zf - dep], [x1, y1, zf], [x0, y1, zf], SLOT.WALL, 0.72);
    b.quad([x0, y0, zf - dep], [x0, y1, zf - dep], [x0, y1, zf], [x0, y0, zf], SLOT.WALL, 0.8);
    b.quad([x1, y0, zf], [x1, y1, zf], [x1, y1, zf - dep], [x1, y0, zf - dep], SLOT.WALL, 0.8);
    b.quad([x0, y0, zf - dep], [x1, y0, zf - dep], [x1, y1, zf - dep], [x0, y1, zf - dep], SLOT.GLASS, 1.0);
    const n = Math.max(1, Math.round((x1 - x0) / (pitch || 1.8)));
    for (let i = 1; i < n; i += 1) {
      const x = x0 + ((x1 - x0) * i) / n;
      b.box(x - 0.05, x + 0.05, y0, y1, zf - dep, zf - dep + 0.06, SLOT.TRIM, 1.05);
    }
    return b;
  }

  /// A front door with its surround, fanlight and step. Doors are worth their
  /// triangles because a door is the only part of an elevation that gives the
  /// building a human scale to be read against.
  function frontDoor(b, cx, zf, h) {
    const w = 0.95, hw = w / 2, top = h || 2.1;
    b.box(cx - hw - 0.14, cx + hw + 0.14, 0, top + 0.62, zf - 0.02, zf + 0.09, SLOT.TRIM, 1.02);
    b.box(cx - hw, cx + hw, 0, top, zf - 0.06, zf + 0.02, SLOT.DARK, 1.0);
    b.box(cx - hw + 0.09, cx + hw - 0.09, 0.28, top - 0.9, zf + 0.015, zf + 0.05, SLOT.TRIM, 0.95);
    b.box(cx - hw + 0.09, cx + hw - 0.09, top - 0.78, top - 0.18, zf + 0.015, zf + 0.05, SLOT.TRIM, 0.95);
    b.box(cx - hw, cx + hw, top + 0.08, top + 0.52, zf - 0.1, zf - 0.04, SLOT.GLASS, 1.0);
    b.box(cx - hw - 0.3, cx + hw + 0.3, 0, 0.14, zf, zf + 0.55, SLOT.TRIM, 1.04);          // step
    b.box(cx - hw - 0.34, cx + hw + 0.34, top + 0.66, top + 0.82, zf - 0.02, zf + 0.42, SLOT.TRIM, 1.05); // hood
    b.cyl(cx + hw - 0.18, zf + 0.06, top - 1.02, top - 0.92, 0.05, 0.05, 6, SLOT.METAL, 1.05);
    return b;
  }

  // --------------------------------------------------------------- roofing
  /// A pitched roof with its ridge running along X, so the gables face +/-X.
  /// Eaves overhang, fascia, barge boards and a ridge cap are all in here
  /// because between them they are most of what tells a pitched roof from a
  /// wedge, and they cost about fifty triangles.
  function gableRoof(b, hw, hd, eave, rise, over) {
    const o = over == null ? 0.32 : over;
    const X0 = -hw - o, X1 = hw + o, Z0 = -hd - o, Z1 = hd + o, top = eave + rise;
    b.quad([X0, eave, Z1], [X1, eave, Z1], [X1, top, 0], [X0, top, 0], SLOT.ROOF, 1.0);
    b.quad([X1, eave, Z0], [X0, eave, Z0], [X0, top, 0], [X1, top, 0], SLOT.ROOF, 0.86);
    b.tri([X0, eave, Z0], [X0, eave, Z1], [X0, top, 0], SLOT.ROOF, 0.82);
    b.tri([X1, eave, Z1], [X1, eave, Z0], [X1, top, 0], SLOT.ROOF, 0.9);
    b.box(X0, X1, eave - 0.17, eave, Z1 - 0.08, Z1, SLOT.TRIM, 1.02);                      // fascia
    b.box(X0, X1, eave - 0.17, eave, Z0, Z0 + 0.08, SLOT.TRIM, 0.95);
    b.box(X0, X0 + 0.09, eave - 0.05, top, Z0, Z1, SLOT.TRIM, 0.98);                       // barge boards
    b.box(X1 - 0.09, X1, eave - 0.05, top, Z0, Z1, SLOT.TRIM, 0.98);
    b.box(X0, X1, top - 0.05, top + 0.11, -0.14, 0.14, SLOT.ROOF, 1.08);                   // ridge
    b.quad([X0, eave - 0.17, Z1 - 0.08], [X1, eave - 0.17, Z1 - 0.08], [X1, eave - 0.17, hd], [X0, eave - 0.17, hd], SLOT.TRIM, 0.7);
    return b;
  }
  /// A hipped roof: four slopes to a short ridge. The civic and low-rise
  /// residential kinds use it because a hip reads as a deliberate building and
  /// a gable reads as a house, at any zoom.
  function hipRoof(b, hw, hd, eave, rise, over) {
    const o = over == null ? 0.3 : over;
    const X0 = -hw - o, X1 = hw + o, Z0 = -hd - o, Z1 = hd + o, top = eave + rise;
    const rx = Math.max(0.4, hw - hd * 0.85);
    b.quad([X0, eave, Z1], [X1, eave, Z1], [rx, top, 0], [-rx, top, 0], SLOT.ROOF, 1.0);
    b.quad([X1, eave, Z0], [X0, eave, Z0], [-rx, top, 0], [rx, top, 0], SLOT.ROOF, 0.86);
    b.tri([X0, eave, Z0], [X0, eave, Z1], [-rx, top, 0], SLOT.ROOF, 0.82);
    b.tri([X1, eave, Z1], [X1, eave, Z0], [rx, top, 0], SLOT.ROOF, 0.9);
    for (const z of [[Z1 - 0.08, Z1, 1.02], [Z0, Z0 + 0.08, 0.95]]) b.box(X0, X1, eave - 0.17, eave, z[0], z[1], SLOT.TRIM, z[2]);
    b.box(X0, X0 + 0.08, eave - 0.17, eave, Z0, Z1, SLOT.TRIM, 0.95);
    b.box(X1 - 0.08, X1, eave - 0.17, eave, Z0, Z1, SLOT.TRIM, 1.0);
    b.box(-rx, rx, top - 0.05, top + 0.1, -0.13, 0.13, SLOT.ROOF, 1.08);
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
      b.cyl(-hw * 0.62, hd * 0.42, y, y + 1.8, 0.42, 0.42, 8, SLOT.METAL, 0.95);          // water tank
    }
    return b;
  }
  /// A stack with corbelled head and pots. Chimneys are the single cheapest
  /// thing that says "temperate, and not new": the kit puts one on every
  /// pitched roof and one per party wall on a terrace.
  function chimney(b, cx, cz, base, top, w, d, pots) {
    b.box(cx - w / 2, cx + w / 2, base, top, cz - d / 2, cz + d / 2, SLOT.WALL, 1.0);
    b.box(cx - w / 2 - 0.09, cx + w / 2 + 0.09, top - 0.3, top, cz - d / 2 - 0.09, cz + d / 2 + 0.09, SLOT.WALL, 1.06);
    const n = pots || 2;
    for (let i = 0; i < n; i += 1) {
      const x = cx + (n === 1 ? 0 : (i / (n - 1) - 0.5) * (w - 0.4));
      b.cyl(x, cz, top, top + 0.55, 0.14, 0.12, 6, SLOT.ROOF, 1.1);
    }
    return b;
  }
  /// Eaves gutter and one downpipe. Cheap, and the vertical line of a
  /// downpipe is what breaks up a blank gable at close range.
  function rainwater(b, hw, hd, eave) {
    b.box(-hw - 0.3, hw + 0.3, eave - 0.3, eave - 0.18, hd + 0.2, hd + 0.34, SLOT.TRIM, 1.02);
    b.cyl(hw - 0.18, hd + 0.28, 0, eave - 0.28, 0.06, 0.06, 6, SLOT.TRIM, 1.0);
    b.cyl(-hw + 0.18, hd + 0.28, 0, eave - 0.28, 0.06, 0.06, 6, SLOT.TRIM, 1.0);
    return b;
  }

  // ------------------------------------------------------ small site pieces
  /// Boundary railings: two rails and a run of balusters. The balusters are
  /// the reason a front garden costs real triangles, and they are also the
  /// reason it reads as a front garden rather than a green rectangle.
  function railing(b, x0, x1, z, y, h, pitch) {
    const n = Math.max(2, Math.round((x1 - x0) / (pitch || 0.24)));
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
    b.box(x0 - 0.04, x1 + 0.04, h, h + 0.08, z0 - 0.04, z1 + 0.04, SLOT.TRIM, 1.06);
    return b;
  }
  /// A deciduous street tree: tapered trunk, two canopy masses. Two masses
  /// rather than one because a single blob reads as a lollipop, and the
  /// second one is twenty-four triangles.
  function tree(b, cx, cz, h, lean, seg) {
    const s = seg || 6, r = h * 0.055;
    b.loft(oval(cx, cz, r, r, s), oval(cx + lean * 0.3, cz, r * 0.62, r * 0.62, s), 0, h * 0.42, SLOT.WALL, 0.55);
    const c1 = h * 0.62, c2 = h * 0.86, rad = h * 0.3;
    b.loft(oval(cx + lean * 0.4, cz, rad * 0.55, rad * 0.55, s), oval(cx + lean * 0.6, cz, rad, rad * 0.92, s), h * 0.38, c1, SLOT.FOLIAGE, 0.9);
    b.loft(oval(cx + lean * 0.6, cz, rad, rad * 0.92, s), oval(cx + lean * 0.8, cz + lean * 0.2, rad * 0.72, rad * 0.7, s), c1, c2, SLOT.FOLIAGE, 1.05);
    b.loft(oval(cx + lean * 0.8, cz + lean * 0.2, rad * 0.72, rad * 0.7, s), oval(cx + lean, cz + lean * 0.3, rad * 0.16, rad * 0.16, s), c2, h, SLOT.FOLIAGE, 1.1);
    return b;
  }
  function lampColumn(b, cx, cz, out) {
    b.box(cx - 0.22, cx + 0.22, 0, 0.16, cz - 0.22, cz + 0.22, SLOT.GROUND, 1.0);
    b.cyl(cx, cz, 0.1, 7.2, 0.11, 0.07, 6, SLOT.METAL, 0.95);
    b.box(cx, cx + out * 1.1, 7.05, 7.2, cz - 0.05, cz + 0.05, SLOT.METAL, 1.0);
    b.box(cx + out * 0.75, cx + out * 1.3, 6.72, 7.06, cz - 0.19, cz + 0.19, SLOT.METAL, 1.05);
    b.slab(cx + out * 0.78, cx + out * 1.27, cz - 0.16, cz + 0.16, 6.71, SLOT.GLASS, 1.1);
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
    b.cyl(cx, cz, 0, 0.92, 0.09, 0.08, 6, SLOT.METAL, 1.0);
    b.cyl(cx, cz, 0.92, 1.0, 0.11, 0.05, 6, SLOT.METAL, 1.08);
    return b;
  }
  function litterBin(b, cx, cz) {
    b.cyl(cx, cz, 0, 0.9, 0.24, 0.26, 8, SLOT.METAL, 0.9);
    b.cyl(cx, cz, 0.9, 1.0, 0.28, 0.22, 8, SLOT.DARK, 1.0);
    return b;
  }
  function roadSign(b, cx, cz, h) {
    b.cyl(cx, cz, 0, h, 0.05, 0.05, 6, SLOT.METAL, 1.0);
    b.box(cx - 0.34, cx + 0.34, h - 0.5, h, cz - 0.03, cz + 0.03, SLOT.TRIM, 1.08);
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
    b.box(-hw, hw, 0, h, -hd, hd, SLOT.WALL, 1.0);
    b.box(-hw - 0.13, hw + 0.13, 0, 0.6, -hd - 0.13, hd + 0.13, SLOT.TRIM, 0.9);
    for (let i = 1; i <= (courses || 0); i += 1) {
      const y = courseAt(i);
      b.box(-hw - 0.07, hw + 0.07, y - 0.09, y + 0.09, -hd - 0.07, hd + 0.07, SLOT.TRIM, 1.04);
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
      // Front elevation: a canted bay on the ground floor, sashes above.
      const bayW = Math.min(3.1, p.w * 0.32), bx = -hw + bayW * 0.62 + 0.5;
      b.box(bx - bayW / 2, bx + bayW / 2, 0, 2.75, hd, hd + 1.05, SLOT.WALL, 1.02);
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
        sash(b, -hw + 1.5, s * STOREY + 0.85, 1.2, 1.55, hd, 2, 3);
        sash(b, hw - 1.6, s * STOREY + 0.85, 1.5, 1.55, hd, 2, 3);
      }
      b.pop();
      for (const yaw of [1, 3]) {
        b.push(yaw, 0, 0, 0);
        for (let s = 0; s < p.storeys; s += 1) sash(b, 0.6, s * STOREY + 1.0, 0.95, 1.35, hw, 2, 2);
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
      paving(b, hw - 2.4, hw - 0.6, zc + hd - 0.1, zc + hd + 0.1);
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
        for (let s = 0; s < p.storeys; s += 1) sash(b, cx, s * STOREY + 0.9, uw * 0.38, 1.45, hd, 2, 2);
        b.box(cx - uw * 0.3, cx + uw * 0.16, 0, 2.4, -hd - 2.4, -hd + 0.1, SLOT.WALL, 0.94);
        b.box(cx - uw * 0.34, cx + uw * 0.2, 2.4, 2.6, -hd - 2.55, -hd + 0.1, SLOT.ROOF, 1.05);
      }
      b.pop();
      b.pop();
      // Forecourt: a shallow strip, dwarf wall and railings, no front garden.
      const fz = p.d / 2;
      paving(b, -hw, hw, zc + hd, fz - 1.0);
      lowWall(b, -hw, hw, fz - 1.2, fz - 0.85, 0.5);
      railing(b, -hw, hw, fz - 1.02, 0.58, 0.5, 0.24);
      for (let u = 0; u < units; u += 1) grass(b, -hw + uw * u + 0.4, -hw + uw * (u + 1) - 0.4, -p.d / 2 + 0.5, zc - hd - 3.0);
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
      b.box(-1.7, 1.7, 0, STOREY - 0.2, hd, hd + 0.9, SLOT.WALL, 1.03);
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
        for (const cx of [-hw + 3.0, -hw + 6.4, hw - 6.4, hw - 3.0]) sash(b, cx, s * STOREY + 0.95, 1.35, 1.55, hd, 2, 2);
      }
      b.pop();
      for (const yaw of [1, 3]) {
        b.push(yaw, 0, 0, 0);
        for (let s = 0; s < p.storeys; s += 1) sash(b, 0, s * STOREY + 1.0, 1.1, 1.4, hw, 2, 2);
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
      b.box(-2.2, 2.2, 0, h + 1.3, hd, hd + 1.1, SLOT.WALL, 0.94);
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
        for (let s = 0; s < p.storeys; s += 1) sash(b, 0, s * STOREY + 1.0, 1.2, 1.45, hw, 2, 2);
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
      b.box(-hw - 0.3, hw + 0.3, 0, TALL_STOREY, hd - 0.2, hd + 0.9, SLOT.WALL, 1.02);   // ground-floor podium
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
          sash(b, 0, s * STOREY + 1.1, 1.1, 1.3, hw, 1, 2);
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
      b.box(hw - 2.2, hw - 1.05, 0.09, 2.35, hd - 0.96, hd - 0.86, SLOT.DARK, 1.0);   // shop door
      b.box(hw - 2.2, hw - 1.05, 2.4, 2.95, hd - 0.96, hd - 0.9, SLOT.GLASS, 1.0);
      for (let s = 1; s < p.storeys; s += 1) {
        const y = TALL_STOREY + (s - 1) * STOREY;
        for (let i = 0; i < 4; i += 1) sash(b, -hw + (p.w * (i + 0.5)) / 4, y + 0.7, 1.1, 1.7, hd, 2, 3);
      }
      b.push(2, 0, 0, 0);
      for (let s = 0; s < p.storeys; s += 1) {
        const y = s === 0 ? 0 : TALL_STOREY + (s - 1) * STOREY;
        for (let i = 0; i < 2; i += 1) sash(b, -hw + (p.w * (i + 0.5)) / 2, y + 1.0, 1.15, 1.5, hd, 2, 2);
      }
      b.box(-hw + 0.6, -hw + 2.4, 0, 2.2, -hd - 0.06, -hd + 0.02, SLOT.DARK, 1.0);
      b.pop();
      for (const yaw of [1, 3]) {
        b.push(yaw, 0, 0, 0);
        for (let s = 1; s < p.storeys; s += 1) sash(b, 0, TALL_STOREY + (s - 1) * STOREY + 1.0, 1.0, 1.4, hw, 2, 2);
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
      b.box(-hw, hw, 0, h, -hd, hd, SLOT.WALL, 1.0);
      b.box(-hw - 0.14, hw + 0.14, 0, 1.2, -hd - 0.14, hd + 0.14, SLOT.TRIM, 0.9);
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
        for (let k = 0; k < 12; k += 1) b.box(cx - 1.85, cx + 1.85, 0.2 + k * 0.36, 0.42 + k * 0.36, hd - 0.32, hd - 0.24, SLOT.METAL, 0.95);
        b.box(cx - 2.2, cx + 2.2, 4.6, 4.85, hd - 0.4, hd + 1.6, SLOT.METAL, 1.06);
        b.box(cx - 2.3, cx + 2.3, 0, 1.1, hd, hd + 1.5, SLOT.GROUND, 1.04);   // dock apron
        bollard(b, cx - 2.6, hd + 1.7);
        bollard(b, cx + 2.6, hd + 1.7);
      }
      b.box(hw - 3.2, hw - 2.0, 0, 2.2, hd - 0.06, hd + 0.03, SLOT.DARK, 1.0);
      b.box(hw - 3.5, hw - 1.7, 2.2, 2.45, hd - 0.1, hd + 1.0, SLOT.METAL, 1.05);
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
      b.box(-4.6, 4.6, 0, TALL_STOREY + 1.4, hd, hd + 2.6, SLOT.WALL, 1.03);
      b.box(-5.0, 5.0, TALL_STOREY + 1.4, TALL_STOREY + 1.85, hd - 0.1, hd + 3.0, SLOT.TRIM, 1.1);
      b.tri([-5.0, TALL_STOREY + 1.85, hd + 3.0], [5.0, TALL_STOREY + 1.85, hd + 3.0], [0, TALL_STOREY + 3.3, hd + 3.0], SLOT.TRIM, 1.06);
      b.quad([-5.0, TALL_STOREY + 1.85, hd + 3.0], [0, TALL_STOREY + 3.3, hd + 3.0], [0, TALL_STOREY + 3.3, hd], [-5.0, TALL_STOREY + 1.85, hd], SLOT.ROOF, 0.95);
      b.quad([0, TALL_STOREY + 3.3, hd + 3.0], [5.0, TALL_STOREY + 1.85, hd + 3.0], [5.0, TALL_STOREY + 1.85, hd], [0, TALL_STOREY + 3.3, hd], SLOT.ROOF, 1.05);
      for (let i = 0; i < 4; i += 1) {
        const x = -3.4 + i * 2.27;
        b.cyl(x, hd + 2.2, 0.6, TALL_STOREY + 1.35, 0.31, 0.27, 8, SLOT.TRIM, 1.04);
        b.cyl(x, hd + 2.2, TALL_STOREY + 1.35, TALL_STOREY + 1.5, 0.37, 0.31, 8, SLOT.TRIM, 1.09);
      }
      for (let i = 0; i < 4; i += 1) b.box(-5.6 + i * 0.0, 5.6, i * 0.2, i * 0.2 + 0.2, hd + 2.6 + i * 0.42, hd + 4.4, SLOT.TRIM, 1.04);
      frontDoor(b, 0, hd + 2.6, 3.0);
      // Clock cupola. It carries no maker's name and no dial reading.
      b.box(-1.9, 1.9, h + hd * 0.42 - 0.5, h + hd * 0.42 + 3.4, -1.9, 1.9, SLOT.WALL, 1.04);
      b.box(-2.15, 2.15, h + hd * 0.42 + 3.4, h + hd * 0.42 + 3.65, -2.15, 2.15, SLOT.TRIM, 1.1);
      for (const f of [[1.9, 1.98, 0], [-1.98, -1.9, 0]]) b.cyl(0, (f[0] + f[1]) / 2, h + hd * 0.42 + 1.1, h + hd * 0.42 + 2.5, 0.8, 0.8, 10, SLOT.TRIM, 1.08);
      b.cyl(0, 0, h + hd * 0.42 + 3.65, h + hd * 0.42 + 6.1, 1.75, 0.12, 8, SLOT.ROOF, 1.06);
      b.cyl(0, 0, h + hd * 0.42 + 6.1, h + hd * 0.42 + 7.4, 0.07, 0.07, 6, SLOT.METAL, 1.1);
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
        for (let i = 0; i < 7; i += 1) sash(b, -hw + (p.w * (i + 0.5)) / 7, y + 1.0, 1.3, 1.9, hd, 2, 3);
      }
      b.pop();
      for (const yaw of [1, 3]) {
        b.push(yaw, 0, 0, 0);
        for (let s = 0; s < p.storeys; s += 1) {
          for (let i = 0; i < 3; i += 1) sash(b, -hd + (hd * 2 * (i + 0.5)) / 3, (s === 0 ? 0 : TALL_STOREY + (s - 1) * 3.6) + 1.0, 1.2, 1.85, hw, 2, 3);
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
        { x0: -4.6, x1: 4.6, z0: zc + hd, z1: zc + hd + 2.6, h: TALL_STOREY + 1.85, roof: "flat", para: 0.4, bands: 1 },
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
      for (let i = 0; i < 5; i += 1) b.cyl(-hw + 3.0 + i * 4.2, -hd - 3.2, 0, 2.9, 0.09, 0.09, 6, SLOT.METAL, 1.0);
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
      b.box(-hw + 2.0, hw - 2.0, 0, 2 * TALL_STOREY, hd, hd + 7.5, SLOT.WALL, 1.02);
      b.box(-hw + 1.7, hw - 1.7, 2 * TALL_STOREY, 2 * TALL_STOREY + 0.35, hd - 0.1, hd + 7.8, SLOT.TRIM, 1.08);
      for (let s = 0; s < 2; s += 1) ribbon(b, -hw + 3.0, hw - 3.0, s * TALL_STOREY + 1.2, 2.0, hd + 7.5, 1.8);
      b.box(-5.5, 5.5, 0, 4.4, hd + 7.5, hd + 12.5, SLOT.TRIM, 1.05);
      b.box(-5.5, 5.5, 4.4, 4.75, hd + 7.3, hd + 12.9, SLOT.TRIM, 1.1);
      for (const x of [-5.2, 5.2]) b.cyl(x, hd + 12.2, 0, 4.4, 0.19, 0.19, 8, SLOT.METAL, 1.0);
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
      b.box(-4.4, 4.4, 0, h + 2.2, hd, hd + 1.4, SLOT.WALL, 1.03);
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
        for (let i = 0; i < 6; i += 1) sash(b, -hw + (p.w * (i + 0.5)) / 6, (s === 0 ? 0 : TALL_STOREY + (s - 1) * 3.5) + 1.1, 1.35, 1.9, hd, 2, 3);
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
          for (let s = 0; s < 2; s += 1) for (let i = 0; i < 3; i += 1) sash(b, -wd + (wd * 2 * (i + 0.5)) / 3, (s ? TALL_STOREY : 0) + 1.15, 1.3, 1.8, 6.0, 2, 3);
          b.pop();
        }
        b.push(2, 0, 0, 0);
        for (let s = 0; s < 2; s += 1) for (let i = 0; i < 3; i += 1) sash(b, -6.0 + (12 * (i + 0.5)) / 3, (s ? TALL_STOREY : 0) + 1.15, 1.25, 1.8, wd, 2, 3);
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
      const seg = 24, rows = 12;
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
      b.band(skin, skin, 0, ry0 + 2.4, SLOT.WALL, 1.0);                     // outer skin
      b.band(skin, oval(0, 0, outRx + 3.2, outRz + 3.2, seg), ry0 + 2.4, ry0 + 2.7, SLOT.TRIM, 1.08);
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
      b.cyl(0, 0, 0, 0.55, 2.3, 2.1, 12, SLOT.TRIM, 1.05);
      b.cyl(0, 0, 0.55, 0.75, 1.9, 1.7, 12, SLOT.WALL, 1.02);
      b.cyl(0, 0, 0.75, 2.6, 0.35, 0.28, 8, SLOT.TRIM, 1.06);
      b.cyl(0, 0, 2.6, 3.1, 0.9, 0.1, 8, SLOT.TRIM, 1.08);
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
        b.cyl(x, tz - 0.9, 3.3, 4.5, 0.24, 0.18, 8, SLOT.TRIM, 1.05);
        b.cyl(x, tz - 0.9, 4.5, 4.7, 0.3, 0.3, 8, SLOT.DARK, 1.0);
      }
      for (const x of [tx - 3.2, tx + 3.2]) b.cyl(x, tz + 3.0, 0, 7.0, 0.16, 0.13, 6, SLOT.METAL, 1.0);
      b.box(tx - 3.4, tx + 3.4, 7.0, 7.25, tz + 2.85, tz + 3.15, SLOT.METAL, 1.05);
      for (let i = 0; i < 3; i += 1) b.cyl(tx - 2.2 + i * 2.2, tz + 3.0, 6.2, 7.0, 0.09, 0.09, 6, SLOT.DARK, 0.9);
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
        for (const x of [-4.3, 0, 4.3]) b.cyl(x, 0, y - 0.9, y, 0.07, 0.07, 6, SLOT.DARK, 0.9);
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

  /// One recessed glazing band. Ten triangles, and it is what stops a map
  /// building reading as an untextured block.
  function mapBand(b, x0, x1, y0, h, zf) {
    const y1 = y0 + h, dep = 0.22;
    b.quad([x0, y0, zf], [x1, y0, zf], [x1, y0, zf - dep], [x0, y0, zf - dep], SLOT.WALL, 0.75);
    b.quad([x0, y1, zf - dep], [x1, y1, zf - dep], [x1, y1, zf], [x0, y1, zf], SLOT.WALL, 0.75);
    b.quad([x0, y0, zf - dep], [x0, y1, zf - dep], [x0, y1, zf], [x0, y0, zf], SLOT.WALL, 0.82);
    b.quad([x1, y0, zf], [x1, y1, zf], [x1, y1, zf - dep], [x1, y0, zf - dep], SLOT.WALL, 0.82);
    b.quad([x0, y0, zf - dep], [x1, y0, zf - dep], [x1, y1, zf - dep], [x0, y1, zf - dep], SLOT.GLASS, 1.0);
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
      const bands = Math.max(1, Math.min(v.bands || 2, level + 1));
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
      } else {
        b.slab(-hw, hw, -hd, hd, y1, SLOT.GROUND, 0.86);
        b.box(-hw - 0.22, hw + 0.22, y1, y1 + (v.para || 0.8), -hd - 0.22, hd + 0.22, SLOT.WALL, 1.02);
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
    if (v === "map" || v === "far" || v === 1 || v === 2 || v === "lod1" || v === "lod2") return "map";
    return "close";
  }
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
    const b = new Builder();
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
    }
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

  /// One frontage, subdivided along its run. Leftover is spread as equal gaps
  /// and the whole row is centred, so a run that does not divide evenly reads
  /// as side entries and alley gates rather than as a mistake at one end.
  function subdivide(seed, salt, run, table, depthCap) {
    const picks = [];
    let used = 0;
    for (let i = 0; i < 24; i += 1) {
      const left = run - used - picks.length * 1.6;
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
    const gap = picks.length > 1 ? Math.min(7.0, (run - used) / (picks.length + 1)) : 0;
    let cursor = (run - used - gap * (picks.length - 1)) / 2;
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
    b.box(X0 - foot, X1 + foot, 0, 0.14, Z1 + foot - 0.16, Z1 + foot, SLOT.TRIM, 1.02);
    b.box(X0 - foot, X1 + foot, 0, 0.14, Z0 - foot, Z0 - foot + 0.16, SLOT.TRIM, 1.02);
    b.box(X0 - foot, X0 - foot + 0.16, 0, 0.14, Z0 - foot, Z1 + foot, SLOT.TRIM, 1.02);
    b.box(X1 + foot - 0.16, X1 + foot, 0, 0.14, Z0 - foot, Z1 + foot, SLOT.TRIM, 1.02);
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

    const out = new Builder();
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
        const probe = variantFor(kind, { w: quantise(best), d: depth, storeys,
          seed: SEEDED[kind] ? mix32(seed ^ Math.imul(lotIndex + 1, 0x27d4eb2f)) : 0 }, lod);
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
    const out = new Builder();
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
