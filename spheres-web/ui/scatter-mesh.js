// The terrain scatter kit: the vegetation, rock, shore, bank and field dressing
// a map is detailed with. Roadmap section G, "Terrain detailing".
//
// WHY THIS EXISTS. Section G asks for biome trees, scrub, grasses, rocks,
// cliffs, snow-edge features, shoreline dressing, river banks and agricultural
// field patches. It also says the elevation surface stays in charge: nothing in
// this file displaces terrain, and nothing here decides where anything goes on
// a real map. It produces PIECES, and a seeded PLAN for arranging them. The
// caller owns the terrain, the exclusions and the draw call.
//
// WHY A PLAYER SHOULD CARE ABOUT SILHOUETTE. Climate is read off a map long
// before any label is. A palm, a spruce, an oak and a saguaro have to be four
// different shapes at a glance or the map says nothing about where you are, so
// each biome here gets its own species built by its own rules rather than one
// tree with a different green on it.
//
// TREES ARE NOT SPHERES ON STICKS, and that is the single largest thing this
// file spends triangles on. A trunk tapers, bends and FORKS; limbs leave the
// fork and carry the canopy; the canopy is four to six overlapping masses with
// hashed lumps on them, not one ball. Section 4 warns against alpha overdraw,
// so a leaf mass is geometry — no crossed cards, no cut-out sheets, nothing
// that costs fill rate twice. Six lobes of forty-two triangles each is cheaper
// than one screen of overdrawn foliage cards and it survives being looked at
// from below.
//
// BUDGETS come from roadmap section 4: a tree is 100..800 triangles near and
// 20..100 far. Scrub, grass and rock are cheaper. These are scattered in their
// THOUSANDS, so the far number is the one that decides whether a map is
// affordable — `treeCost(n)` prices a stand of n at both levels from the
// measured meshes rather than from an estimate.
//
// MODEL SPACE. Metres. +X right, +Y up, +Z forward. Ground contact at Y=0
// exactly: `finish` seats the lowest vertex there, so a caller drops a piece
// onto terrain by its base and nothing else.
//
// DETERMINISM. No clock, no entropy source, no mutable module state that
// reaches geometry. Every varying quantity — a trunk's lean, a lobe's lump, a
// cobble's size, where instance 300 of a scatter lands — is an integer hash of
// explicit inputs. Same inputs, byte-identical output, in separate processes.
// That is iron rule 1 and art gets no exemption from it: a forest that is a
// different forest after a reload cannot be SAVED as a seed, and saving a
// forest as a seed is the only reason a map can afford one.
//
// COLOUR AND LIGHT. Per-vertex RGB, no alpha, no textures and no UVs, because
// that is the whole of what this renderer has. Albedo is emitted per triangle;
// the lambert term is applied in `finish` against the FINAL normal, so smooth
// shading is visible in the colour rather than merely recorded in a buffer.
// The sun and the ambient/diffuse split match town-mesh.js so a stand of trees
// and the town beside it sit under the same light.
//
// SHADING. Curved things open a smoothing group and get crease-limited averaged
// normals — trunks, fronds, canopy lobes, dunes. Faceted things never open one:
// a boulder is flat-shaded on purpose, because facets are what make a rock read
// as rock in a renderer with no textures. Bark, sand and rock get no free
// detail from a texture, so the geometry has to carry it.
//
// WHAT THIS IS NOT. Representative scenery, and original game art. It is not a
// species record, not a survey, not a land-cover claim. A field patch does not
// grant a nation farmland; a cliff tile does not describe a real escarpment; a
// hundred trees on a province is not a forestry statistic. Every description
// this file returns says so in its own words.
//
// NOT WIRED IN. Nothing in the game calls this yet. It is a kit and a plan, and
// integration is somebody else's commit.
(function (root, factory) {
  const api = factory();
  if (typeof module === "object" && module.exports) module.exports = api;
  else root.ScatterMesh = api;
})(typeof globalThis !== "undefined" ? globalThis : this, function () {
  "use strict";

  // ------------------------------------------------------------------- hash
  // The only source of variety in this file. A caller asks a question by salt —
  // "how tall", "which way does it lean", "how lumpy is lobe three" — and gets
  // the same answer on every machine forever. Murmur3's finaliser with a stir
  // in front of it, the same one town-mesh.js uses, so the two kits decorrelate
  // small integers the same way.
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
  function askSpan(seed, salt, lo, hi) { return lo + (hi - lo) * askUnit(seed, salt); }
  function askSigned(seed, salt, amp) { return (askUnit(seed, salt) - 0.5) * 2 * amp; }
  function askOne(seed, salt, list) { return list[ask(seed, salt) % list.length]; }

  // ------------------------------------------------------------------ light
  // One sun, fixed, high and to the right — the same vector and the same
  // ambient/diffuse split as town-mesh.js. Baking light into vertex colour is
  // what the current path does; separating albedo from light is a renderer
  // milestone and not this file's to invent.
  const SUN = (function () {
    const v = [0.42, 0.80, 0.40], n = Math.hypot(v[0], v[1], v[2]);
    return [v[0] / n, v[1] / n, v[2] / n];
  })();
  const AMBIENT = 0.52, DIFFUSE = 0.58;

  /// How far two faces may disagree and still share a corner normal. cos 69.5°,
  /// the same limit site-mesh.js settled on. It merges the ring of an 8-sided
  /// tube (adjacent faces 45° apart, dot 0.707) and refuses its end cap (dot 0),
  /// which is exactly the split wanted: a trunk is round, the rim where it meets
  /// its cap is not. It also PROVES the shading contract the checks assert —
  /// every face folded into a corner is within the limit of that corner's own
  /// face, so a smoothed normal can never point behind the triangle it belongs
  /// to, and dot(vertexNormal, faceNormal) >= CREASE always holds.
  const CREASE = 0.35;
  /// Weld tolerance, 0.1 mm. Two corners closer than this are the same corner.
  const WELD = 1e4;

  function clamp01(v) { return v < 0 ? 0 : v > 1 ? 1 : v; }
  function rgb(hex) {
    return [((hex >> 16) & 255) / 255, ((hex >> 8) & 255) / 255, (hex & 255) / 255];
  }
  /// Scale an albedo. Used for bark rings, cobble variety and the dozen places
  /// where one material wants to be slightly not itself.
  function sh(c, k) { return [clamp01(c[0] * k), clamp01(c[1] * k), clamp01(c[2] * k)]; }
  function mixc(a, b, t) {
    const u = t < 0 ? 0 : t > 1 ? 1 : t;
    return [a[0] + (b[0] - a[0]) * u, a[1] + (b[1] - a[1]) * u, a[2] + (b[2] - a[2]) * u];
  }

  // --------------------------------------------------------------- 3x4 maths
  // Row-major 3x4; the implied last row is [0,0,0,1]. Angles are RADIANS, which
  // differs from site-mesh.js's degrees on purpose: everything here that rotates
  // is doing so by a hashed fraction of a turn, and a turn is 2*PI.
  const IDENT = [1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0];
  function mul(a, b) {
    const o = new Array(12);
    for (let r = 0; r < 3; r += 1) {
      for (let c = 0; c < 3; c += 1) {
        o[r * 4 + c] = a[r * 4] * b[c] + a[r * 4 + 1] * b[4 + c] + a[r * 4 + 2] * b[8 + c];
      }
      o[r * 4 + 3] = a[r * 4] * b[3] + a[r * 4 + 1] * b[7] + a[r * 4 + 2] * b[11] + a[r * 4 + 3];
    }
    return o;
  }
  function mTranslate(x, y, z) { return [1, 0, 0, x, 0, 1, 0, y, 0, 0, 1, z]; }
  function mScale(x, y, z) { return [x, 0, 0, 0, 0, y, 0, 0, 0, 0, z, 0]; }
  function mRotX(a) { const c = Math.cos(a), s = Math.sin(a); return [1, 0, 0, 0, 0, c, -s, 0, 0, s, c, 0]; }
  function mRotY(a) { const c = Math.cos(a), s = Math.sin(a); return [c, 0, s, 0, 0, 1, 0, 0, -s, 0, c, 0]; }
  function mRotZ(a) { const c = Math.cos(a), s = Math.sin(a); return [c, -s, 0, 0, s, c, 0, 0, 0, 0, 1, 0]; }

  function sub(a, b) { return [a[0] - b[0], a[1] - b[1], a[2] - b[2]]; }
  function dot3(a, b) { return a[0] * b[0] + a[1] * b[1] + a[2] * b[2]; }
  function cross(a, b) {
    return [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]];
  }
  function scaled(a, k) { return [a[0] * k, a[1] * k, a[2] * k]; }
  function norm3(a) {
    const l = Math.hypot(a[0], a[1], a[2]);
    return l > 1e-9 ? [a[0] / l, a[1] / l, a[2] / l] : [0, 1, 0];
  }
  function reversed(list) { return list.slice().reverse(); }

  // ---------------------------------------------------------------- builder
  // Triangle soup, a transform stack, smoothing groups and named part ranges.
  // Normals are NOT accumulated at emit: they are derived in `finish` from the
  // vertices AS TRANSFORMED, so a mirrored or negatively scaled instance gets
  // the normal its geometry actually has rather than the one its author meant.
  function Mesh() {
    this.pos = [];
    this.col = [];
    this.grp = [];
    this.parts = [];
    this.m = IDENT.slice();
    this.stack = [];
    this.sg = 0;
    this.sgNext = 0;
    this.open = false;
  }
  /// Everything drawn inside `fn` shares one smoothing group. A FRESH id every
  /// call, deliberately: two canopy lobes overlap where they meet, and a shared
  /// group would weld their skins into one lumpy surface instead of leaving the
  /// crease that makes them read as two masses.
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
  Mesh.prototype.rotX = function (a) { this.m = mul(this.m, mRotX(a)); return this; };
  Mesh.prototype.rotY = function (a) { this.m = mul(this.m, mRotY(a)); return this; };
  Mesh.prototype.rotZ = function (a) { this.m = mul(this.m, mRotZ(a)); return this; };
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

  /// A degenerate triangle is DROPPED, not emitted with a fallback normal. A
  /// blade tapering to a point and a lobe collapsing onto its pole both produce
  /// them by construction, and letting one through costs either a NaN in an
  /// attribute buffer or a black facet nobody can account for later. Dropping is
  /// deterministic, so a kind's triangle count stays fixed across seeds — which
  /// is what makes a per-instance budget a number rather than an average.
  Mesh.prototype.tri = function (a, b, c, col) {
    const flip = this.detSign() < 0;
    const A = this.xf(a), B = this.xf(flip ? c : b), C = this.xf(flip ? b : c);
    const ux = B[0] - A[0], uy = B[1] - A[1], uz = B[2] - A[2];
    const vx = C[0] - A[0], vy = C[1] - A[1], vz = C[2] - A[2];
    const nx = uy * vz - uz * vy, ny = uz * vx - ux * vz, nz = ux * vy - uy * vx;
    if (!(Math.hypot(nx, ny, nz) > 1e-9)) return this;
    this.pos.push(A[0], A[1], A[2], B[0], B[1], B[2], C[0], C[1], C[2]);
    for (let i = 0; i < 3; i += 1) this.col.push(col[0], col[1], col[2]);
    this.grp.push(this.sg);
    return this;
  };
  Mesh.prototype.quad = function (a, b, c, d, col) {
    return this.tri(a, b, c, col).tri(a, c, d, col);
  };
  Mesh.prototype.fan = function (pts, col) {
    for (let i = 1; i + 1 < pts.length; i += 1) this.tri(pts[0], pts[i], pts[i + 1], col);
    return this;
  };

  /// Named ranges in the shape equipment-mesh.js set and site-mesh.js kept, so
  /// the picker already speaks it. A part that draws nothing is not recorded: an
  /// empty range would break the contiguity the checks assert, and a far LOD
  /// that has no leaf litter should not carry a leaf-litter entry.
  ///
  /// NESTING IS FLATTENED, and the reason is a real defect this caught. A
  /// hedgerow draws an occasional hedgerow tree, and a tree opens parts of its
  /// own — trunk, limbs, canopy — so those ranges were being appended AFTER the
  /// hedge range that already contained them, and the part list stopped tiling
  /// the buffer in order. Only the outermost part records; an inner one draws
  /// into whatever range is already open.
  Mesh.prototype.part = function (name, fn) {
    if (this.open) { fn(this); return this; }
    const first = this.pos.length / 3;
    this.open = true;
    fn(this);
    this.open = false;
    const count = this.pos.length / 3 - first;
    if (count > 0) {
      const cut = name.indexOf(" / ");
      this.parts.push({
        name, first, count,
        group: cut < 0 ? name : name.slice(0, cut),
        label: cut < 0 ? name : name.slice(cut + 3),
      });
    }
    return this;
  };

  // -------------------------------------------------------------- primitives
  Mesh.prototype.bar = function (x0, x1, y0, y1, z0, z1, col) {
    this.quad([x1, y0, z0], [x1, y1, z0], [x1, y1, z1], [x1, y0, z1], col);
    this.quad([x0, y0, z0], [x0, y0, z1], [x0, y1, z1], [x0, y1, z0], col);
    this.quad([x0, y1, z0], [x0, y1, z1], [x1, y1, z1], [x1, y1, z0], col);
    this.quad([x0, y0, z0], [x1, y0, z0], [x1, y0, z1], [x0, y0, z1], col);
    this.quad([x0, y0, z1], [x1, y0, z1], [x1, y1, z1], [x0, y1, z1], col);
    this.quad([x0, y0, z0], [x0, y1, z0], [x1, y1, z0], [x1, y0, z0], col);
    return this;
  };
  Mesh.prototype.slab = function (x0, x1, z0, z1, y, col) {
    return this.quad([x0, y, z0], [x0, y, z1], [x1, y, z1], [x1, y, z0], col);
  };

  /// A ring in the XZ plane at `y`, in increasing angle. Every closed primitive
  /// below is built from these, and the winding convention is fixed once here:
  /// increasing angle runs +X towards +Z, which means a fan over the ring as
  /// plotted faces DOWN and a fan over the reversed ring faces UP. Band quads
  /// go (lower[k], upper[k], upper[k+1], lower[k+1]) to face outward. Getting
  /// this wrong is the classic inverted-normal defect and the checks test for it
  /// by signed volume, not by eye.
  function ring(cx, cy, cz, rx, rz, seg, phase) {
    const pts = [];
    for (let k = 0; k < seg; k += 1) {
      const a = ((k / seg) * Math.PI * 2) + (phase || 0);
      pts.push([cx + Math.cos(a) * rx, cy, cz + Math.sin(a) * rz]);
    }
    return pts;
  }

  Mesh.prototype.band = function (lower, upper, col, colUpper) {
    const n = lower.length;
    for (let k = 0; k < n; k += 1) {
      const j = (k + 1) % n;
      if (colUpper) {
        this.tri(lower[k], upper[k], upper[j], colUpper);
        this.tri(lower[k], upper[j], lower[j], col);
      } else {
        this.quad(lower[k], upper[k], upper[j], lower[j], col);
      }
    }
    return this;
  };

  /// A closed dome standing on the ground plane: base ring at `y0`, widening
  /// above it before it closes at the top. `bulge` below 0.5 puts the widest
  /// section above the base, which is what gives a boulder its overhang and
  /// stops it reading as a hemisphere. `rough` displaces each ring vertex by a
  /// hashed fraction of the radius, from a table so the surface stays closed.
  Mesh.prototype.dome = function (cx, y0, cz, rx, ry, rz, seg, stacks, seed, rough, col, colTop, bulge) {
    const start = Math.PI * (bulge == null ? 0.5 : bulge);
    const rings = [];
    for (let i = 0; i <= stacks; i += 1) {
      const t = i / stacks;
      const phi = start + (Math.PI - start) * t;
      const sr = Math.sin(phi), sy = -Math.cos(phi);
      const pts = [];
      for (let k = 0; k < seg; k += 1) {
        const a = (k / seg) * Math.PI * 2;
        const j = 1 + rough * askSigned(seed, i * 131 + k * 17 + 5, 1);
        pts.push([
          cx + Math.cos(a) * sr * rx * j,
          y0 + (sy - -Math.cos(start)) * ry * (1 + rough * askSigned(seed, i * 47 + k * 3 + 11, 0.5)),
          cz + Math.sin(a) * sr * rz * j,
        ]);
      }
      rings.push(pts);
    }
    const top = rings[stacks][0];
    const apex = [cx, top[1] + ry * 0.04, cz];
    this.fan(rings[0], sh(col, 0.7));
    for (let i = 0; i + 1 < stacks; i += 1) {
      const t = (i + 1) / stacks;
      this.band(rings[i], rings[i + 1], mixc(col, colTop || col, t * 0.8));
    }
    const last = rings[stacks - 1];
    for (let k = 0; k < seg; k += 1) this.tri(last[k], apex, last[(k + 1) % seg], colTop || col);
    return this;
  };

  /// A closed lumpy spheroid, for a canopy mass that floats clear of the ground.
  /// `squash` flattens it; `droop` shears the lower half sideways so a lobe hangs
  /// off its limb rather than balancing on it.
  Mesh.prototype.blob = function (cx, cy, cz, rx, ry, rz, seg, stacks, seed, rough, col, colTop, droop) {
    const rings = [];
    for (let i = 1; i < stacks; i += 1) {
      const phi = (i / stacks) * Math.PI;
      const sr = Math.sin(phi), sy = -Math.cos(phi);
      const pts = [];
      for (let k = 0; k < seg; k += 1) {
        const a = (k / seg) * Math.PI * 2;
        const j = 1 + rough * askSigned(seed, i * 131 + k * 17 + 5, 1);
        const dz = (droop || 0) * (1 - sy) * 0.5;
        pts.push([
          cx + Math.cos(a) * sr * rx * j + dz * rx,
          cy + sy * ry * (1 + rough * askSigned(seed, i * 53 + k * 7 + 3, 0.6)),
          cz + Math.sin(a) * sr * rz * j,
        ]);
      }
      rings.push(pts);
    }
    const bot = [cx + (droop || 0) * rx, cy - ry * (1 + rough * askSigned(seed, 991, 0.4)), cz];
    const topP = [cx, cy + ry * (1 + rough * askSigned(seed, 997, 0.4)), cz];
    const first = rings[0], last = rings[rings.length - 1];
    for (let k = 0; k < seg; k += 1) this.tri(bot, first[k], first[(k + 1) % seg], sh(col, 0.82));
    for (let i = 0; i + 1 < rings.length; i += 1) {
      this.band(rings[i], rings[i + 1], mixc(col, colTop || col, (i + 1) / rings.length));
    }
    for (let k = 0; k < seg; k += 1) this.tri(last[k], topP, last[(k + 1) % seg], colTop || col);
    return this;
  };

  /// A swept tube along a polyline: the trunk, every limb, every frond rib, the
  /// saguaro's arms and a river bank's exposed roots. The frame is parallel
  /// transported from node to node so a limb that curves through ninety degrees
  /// does not twist along its own axis, which is the defect that makes a hand
  /// rolled branch look like a corkscrew. `flutes` modulates the radius around
  /// the ring — the saguaro's ribs are real geometry, not a colour band.
  Mesh.prototype.tube = function (nodes, seg, colAt, opts) {
    const o = opts || {};
    const flute = o.flute || 0, flutes = o.flutes || 0, phase = o.phase || 0;
    const rings = [];
    let ref = [0, 0, 1];
    for (let i = 0; i < nodes.length; i += 1) {
      const p = nodes[i].p, prev = i > 0 ? nodes[i - 1].p : null, next = nodes[i + 1] ? nodes[i + 1].p : null;
      const t = norm3(next && prev ? sub(next, prev) : next ? sub(next, p) : sub(p, prev));
      let u = sub(ref, scaled(t, dot3(ref, t)));
      if (Math.hypot(u[0], u[1], u[2]) < 1e-4) u = sub([1, 0, 0], scaled(t, t[0]));
      u = norm3(u);
      ref = u;
      const v = cross(t, u);
      const pts = [];
      for (let k = 0; k < seg; k += 1) {
        const a = (k / seg) * Math.PI * 2 + phase;
        const rr = nodes[i].r * (1 + flute * Math.cos(a * flutes));
        const ca = Math.cos(a), sa = Math.sin(a);
        pts.push([
          p[0] + (ca * u[0] + sa * v[0]) * rr,
          p[1] + (ca * u[1] + sa * v[1]) * rr,
          p[2] + (ca * u[2] + sa * v[2]) * rr,
        ]);
      }
      rings.push(pts);
    }
    for (let i = 0; i + 1 < rings.length; i += 1) {
      this.band(rings[i], rings[i + 1], colAt(i), colAt(i + 1));
    }
    if (o.capBase) this.fan(rings[0], colAt(0));
    if (o.capTop !== false) this.fan(reversed(rings[rings.length - 1]), colAt(rings.length - 1));
    return this;
  };

  /// One blade of grass, or one palm leaflet: a shallow V in section so it is
  /// visible from both sides without a two-sided material, tapering to a point.
  /// Six triangles near, two far. This is the one place in the kit where a
  /// transparent card would be cheaper in triangles, and it is exactly the place
  /// roadmap section 4 says not to spend fill rate — a meadow of cards overdraws
  /// the same pixel forty times.
  Mesh.prototype.blade = function (x, z, h, w, yaw, arc, col, colTip, stations) {
    const along = [Math.cos(yaw), 0, Math.sin(yaw)], across = [-Math.sin(yaw), 0, Math.cos(yaw)];
    const n = Math.max(2, stations | 0);
    const rows = [];
    for (let i = 0; i < n; i += 1) {
      const t = i / (n - 1);
      const bend = arc * h * t * t;
      const cy = h * t * (1 - 0.22 * t * t);
      const hw = (w * 0.5) * (1 - t) * (1 - t * 0.4);
      const fold = w * 0.42 * (1 - t);
      const c = [x + along[0] * bend, cy, z + along[2] * bend];
      rows.push({
        l: [c[0] - across[0] * hw, c[1], c[2] - across[2] * hw],
        r: [c[0] + across[0] * hw, c[1], c[2] + across[2] * hw],
        m: [c[0], c[1] + fold, c[2]],
        t,
      });
    }
    for (let i = 0; i + 1 < n; i += 1) {
      const a = rows[i], b = rows[i + 1];
      const c0 = mixc(col, colTip || col, a.t), c1 = mixc(col, colTip || col, b.t);
      this.tri(a.l, a.m, b.m, c1).tri(a.l, b.m, b.l, c1);
      this.tri(a.m, a.r, b.r, c0).tri(a.m, b.r, b.m, c1);
    }
    return this;
  };

  /// A ploughed ridge: a long triangular section running along +X. Two faces,
  /// four triangles, and the pair of them is what makes a brown rectangle read
  /// as a worked field from the air.
  Mesh.prototype.ridge = function (x0, x1, zc, w, h, col, colLit) {
    const a = [x0, 0, zc - w], b = [x1, 0, zc - w], c = [x1, h, zc], d = [x0, h, zc];
    this.quad(a, d, c, b, colLit);
    const e = [x0, 0, zc + w], f = [x1, 0, zc + w];
    this.quad(e, f, c, d, col);
    return this;
  };

  /// A TILEABLE strip: `cols` panels across `width`, `rows` giving the profile
  /// away from the datum edge. This is the cliff face, all three shores, the
  /// river bank and the hedgerow bank, because all five are the same thing — a
  /// profile extruded along a contour with variation on it.
  ///
  /// WHY IT TILES EXACTLY. The height and offset of every point on a COLUMN
  /// BOUNDARY is a hash of that boundary's GLOBAL index, `index * cols + j`.
  /// Tile n's right edge is therefore tile n+1's left edge by construction and
  /// not by luck, and the check asserts it by comparing the two edges vertex for
  /// vertex. Props stay inset by a margin so they never straddle a seam.
  function stripPoint(seed, gj, r, spec, x) {
    const s = spec[r];
    return [
      x,
      s.y + (s.jy || 0) * askSigned(seed, gj * 64 + r * 7 + 1, 1),
      s.z + (s.jz || 0) * askSigned(seed, gj * 64 + r * 7 + 2, 1),
    ];
  }
  Mesh.prototype.strip = function (seed, index, cols, width, spec, colFn) {
    const step = width / cols;
    const grid = [];
    for (let j = 0; j <= cols; j += 1) {
      const gj = index * cols + j, col = [];
      for (let r = 0; r < spec.length; r += 1) col.push(stripPoint(seed, gj, r, spec, j * step));
      grid.push(col);
    }
    for (let j = 0; j < cols; j += 1) {
      for (let r = 0; r + 1 < spec.length; r += 1) {
        this.quad(grid[j][r], grid[j][r + 1], grid[j + 1][r + 1], grid[j + 1][r],
          colFn(r, j, seed, index));
      }
    }
    return grid;
  };

  // ------------------------------------------------------------------ finish
  /// Face normals, crease-limited smoothing, baked light, bounds, ground seat.
  ///
  /// SMOOTHING, and why it is worth the pass. Every triangle used to take the
  /// normal of the face it sat on, so a trunk, a frond, a dune and a canopy lobe
  /// all read as faceted prisms however many segments they were given. Averaging
  /// costs ZERO triangles and is the largest single gain available to a
  /// vertex-coloured renderer with no textures. A corner folds in only the faces
  /// within CREASE of its OWN face, so a tube's ring merges and the rim where it
  /// meets its cap stays sharp — and a boulder, which never opens a group, keeps
  /// every facet it was built with.
  Mesh.prototype.finish = function (info) {
    const positions = new Float32Array(this.pos);
    const normals = new Float32Array(positions.length);
    const faces = new Float32Array(positions.length / 3);
    const tris = positions.length / 9;
    for (let f = 0; f < tris; f += 1) {
      const i = f * 9;
      const ax = positions[i], ay = positions[i + 1], az = positions[i + 2];
      const ux = positions[i + 3] - ax, uy = positions[i + 4] - ay, uz = positions[i + 5] - az;
      const vx = positions[i + 6] - ax, vy = positions[i + 7] - ay, vz = positions[i + 8] - az;
      let nx = uy * vz - uz * vy, ny = uz * vx - ux * vz, nz = ux * vy - uy * vx;
      const len = Math.hypot(nx, ny, nz);
      if (len > 1e-12) { nx /= len; ny /= len; nz /= len; } else { nx = 0; ny = 1; nz = 0; }
      faces[f * 3] = nx; faces[f * 3 + 1] = ny; faces[f * 3 + 2] = nz;
      for (let k = 0; k < 3; k += 1) {
        normals[i + k * 3] = nx; normals[i + k * 3 + 1] = ny; normals[i + k * 3 + 2] = nz;
      }
    }
    const corners = new Map();
    for (let f = 0; f < tris; f += 1) {
      const g = this.grp[f];
      if (!g) continue;
      for (let k = 0; k < 3; k += 1) {
        const i = f * 9 + k * 3;
        const key = g + "|" + Math.round(positions[i] * WELD) + "|"
          + Math.round(positions[i + 1] * WELD) + "|" + Math.round(positions[i + 2] * WELD);
        const bucket = corners.get(key);
        if (bucket) bucket.push(f); else corners.set(key, [f]);
      }
    }
    for (let f = 0; f < tris; f += 1) {
      const g = this.grp[f];
      if (!g) continue;
      const fx = faces[f * 3], fy = faces[f * 3 + 1], fz = faces[f * 3 + 2];
      for (let k = 0; k < 3; k += 1) {
        const i = f * 9 + k * 3;
        const key = g + "|" + Math.round(positions[i] * WELD) + "|"
          + Math.round(positions[i + 1] * WELD) + "|" + Math.round(positions[i + 2] * WELD);
        const bucket = corners.get(key);
        if (!bucket || bucket.length < 2) continue;
        let sx = 0, sy = 0, sz = 0;
        for (let b = 0; b < bucket.length; b += 1) {
          const o = bucket[b] * 3, ox = faces[o], oy = faces[o + 1], oz = faces[o + 2];
          if (ox * fx + oy * fy + oz * fz < CREASE) continue;
          sx += ox; sy += oy; sz += oz;
        }
        const l = Math.hypot(sx, sy, sz);
        if (l > 1e-9) { normals[i] = sx / l; normals[i + 1] = sy / l; normals[i + 2] = sz / l; }
      }
    }
    const colors = new Float32Array(positions.length);
    for (let i = 0; i < positions.length; i += 3) {
      const lambert = normals[i] * SUN[0] + normals[i + 1] * SUN[1] + normals[i + 2] * SUN[2];
      const k = AMBIENT + DIFFUSE * (lambert > 0 ? lambert : 0);
      colors[i] = clamp01(this.col[i] * k);
      colors[i + 1] = clamp01(this.col[i + 1] * k);
      colors[i + 2] = clamp01(this.col[i + 2] * k);
    }
    const min = [Infinity, Infinity, Infinity], max = [-Infinity, -Infinity, -Infinity];
    for (let i = 0; i < positions.length; i += 3) {
      for (let k = 0; k < 3; k += 1) {
        if (positions[i + k] < min[k]) min[k] = positions[i + k];
        if (positions[i + k] > max[k]) max[k] = positions[i + k];
      }
    }
    // Seat the lowest vertex on Y=0. A caller places a piece on terrain by its
    // base and by nothing else, so this is exact rather than a tolerance: a
    // tree hovering four centimetres over a hillside is visible from orbit.
    const ground = min[1];
    if (ground !== 0) {
      for (let i = 1; i < positions.length; i += 3) positions[i] -= ground;
      max[1] -= ground;
      min[1] = 0;
    }
    const out = {
      positions, normals, colors,
      bounds: { min, max },
      size: [max[0] - min[0], max[1] - min[1], max[2] - min[2]],
      parts: this.parts,
      triangleCount: tris,
    };
    if (info) for (const key of Object.keys(info)) out[key] = info[key];
    return out;
  };

  // ---------------------------------------------------------------- palettes
  // One palette per biome. Authored as base colours; the lit, dark and dead
  // variants are derived, so a biome cannot drift into having a rock whose
  // highlight belongs to a different stone. These are ALBEDO — the sun is
  // applied later — which is why they look flat written down and do not look
  // flat on the mesh.
  //
  // 1990 baseline is not a constraint that bites here: a Scots pine in 1990 is
  // a Scots pine. What the era does decide is the field patch, which is small,
  // hedged and ploughed in ridges rather than laser-levelled.
  function palette(spec) {
    const p = {
      bark: rgb(spec.bark),
      leaf: rgb(spec.leaf),
      scrub: rgb(spec.scrub),
      grass: rgb(spec.grass),
      grassDry: rgb(spec.grassDry),
      rock: rgb(spec.rock),
      soil: rgb(spec.soil),
      sand: rgb(spec.sand),
      snow: rgb(spec.snow == null ? 0xdfe4e8 : spec.snow),
      water: rgb(spec.water == null ? 0x2f4a55 : spec.water),
      crop: rgb(spec.crop == null ? spec.grass : spec.crop),
    };
    p.barkLit = sh(p.bark, 1.24);
    p.barkDark = sh(p.bark, 0.72);
    p.leafLit = sh(p.leaf, 1.42);
    p.leafDark = sh(p.leaf, 0.66);
    p.rockLit = sh(p.rock, 1.18);
    p.rockDark = sh(p.rock, 0.7);
    p.soilDark = sh(p.soil, 0.74);
    p.sandWet = sh(p.sand, 0.68);
    p.dead = mixc(p.bark, p.grassDry, 0.55);
    p.litter = mixc(p.soil, p.leafDark, 0.4);
    return p;
  }

  // --------------------------------------------------------------- specimens
  // Structure per species. Sizes are metres and they are ORDINARY sizes for a
  // mature specimen of the type, not record trees and not measurements of any
  // particular one: a 14 m oak, a 20 m Scots pine, a 6 m saguaro. The point of
  // the numbers is that a palm and a spruce stand next to each other at the
  // right relative height, not that any of them is a survey.
  const TREES = {
    oak: {
      builder: "broadleaf", label: "oak", common: "temperate broadleaf",
      h: [10, 16], clear: 0.36, forks: 4, spread: 0.60, crown: 0.56, flat: 0.80,
      lobes: 6, trunkR: 0.048, lean: 0.10, gnarl: 0.22, roots: true,
    },
    holm_oak: {
      builder: "broadleaf", label: "holm oak", common: "mediterranean evergreen oak",
      h: [6, 9.5], clear: 0.20, forks: 3, spread: 0.72, crown: 0.60, flat: 0.94,
      lobes: 5, trunkR: 0.062, lean: 0.16, gnarl: 0.34, roots: true, leafShift: -0.12,
    },
    acacia: {
      builder: "broadleaf", label: "acacia", common: "savanna flat-crowned thorn tree",
      h: [6.5, 10], clear: 0.64, forks: 5, spread: 1.05, crown: 0.74, flat: 0.30,
      lobes: 6, trunkR: 0.040, lean: 0.08, gnarl: 0.16, roots: false, leafShift: 0.10,
    },
    pine: {
      builder: "conifer", label: "Scots pine", common: "temperate conifer",
      h: [15, 23], clear: 0.56, tiers: 5, spread: 0.28, droop: 0.10, trunkR: 0.030,
      lean: 0.08, taper: 0.55, plateTop: true, barkTop: true,
    },
    spruce: {
      builder: "conifer", label: "spruce", common: "boreal conifer",
      h: [13, 21], clear: 0.10, tiers: 9, spread: 0.20, droop: 0.22, trunkR: 0.026,
      lean: 0.04, taper: 0.10, plateTop: false, spire: true,
    },
    krummholz: {
      builder: "conifer", label: "krummholz", common: "alpine wind-flagged conifer",
      h: [1.3, 2.4], clear: 0.12, tiers: 4, spread: 0.55, droop: 0.30, trunkR: 0.055,
      lean: 0.55, taper: 0.30, plateTop: false, flag: 0.85, snag: true,
    },
    palm: { builder: "palm", label: "coconut palm", common: "tropical palm", h: [9, 15], fronds: 9 },
    saguaro: { builder: "saguaro", label: "saguaro", common: "arid columnar cactus", h: [4.5, 7.5], arms: 2 },
  };

  const SHRUBS = {
    hazel_scrub: { label: "hazel scrub", h: [1.6, 2.8], stems: 4, lobes: 4, wide: 0.85, style: "bush" },
    heather: { label: "heather and bilberry", h: [0.32, 0.6], stems: 3, lobes: 5, wide: 1.5, style: "bush", dense: true },
    maquis: { label: "maquis", h: [1.0, 1.9], stems: 5, lobes: 5, wide: 1.15, style: "bush", dense: true },
    creosote: { label: "creosote bush", h: [0.9, 1.8], stems: 6, lobes: 6, wide: 1.0, style: "open" },
    thorn_scrub: { label: "thorn scrub", h: [1.2, 2.2], stems: 5, lobes: 4, wide: 1.25, style: "open", flat: 0.45 },
    tropical_thicket: { label: "broadleaf thicket", h: [1.8, 3.0], leaves: 9, style: "leaves" },
    alpine_cushion: { label: "cushion plant", h: [0.16, 0.3], pads: 11, style: "cushion" },
    dwarf_birch: { label: "dwarf birch mat", h: [0.35, 0.7], stems: 5, lobes: 6, wide: 2.1, style: "mat" },
  };

  const GRASSES = {
    grass_tuft: { label: "grass tuft", h: [0.28, 0.55], blades: 11, spread: 0.30, arc: 0.55 },
    bunchgrass: { label: "bunchgrass", h: [0.35, 0.7], blades: 9, spread: 0.22, arc: 0.30, dry: true },
    sedge_tussock: { label: "sedge tussock", h: [0.25, 0.45], blades: 13, spread: 0.34, arc: 0.70, mound: true },
  };

  const BIOMES = {
    temperate_broadleaf: {
      label: "temperate broadleaf",
      note: "oak and hazel over grass, on brown earth with occasional erratics",
      tree: "oak", shrub: "hazel_scrub", grass: "grass_tuft", rock: "rock_cluster",
      mix: [["oak", 30], ["hazel_scrub", 18], ["grass_tuft", 38], ["boulder", 9], ["rock_cluster", 5]],
      pal: palette({ bark: 0x5b4a3a, leaf: 0x3d6029, scrub: 0x466a2f, grass: 0x5c7238, grassDry: 0x8d8b52, rock: 0x7c7b73, soil: 0x5d4c39, sand: 0x9b8e6f, crop: 0x76863c }),
    },
    temperate_conifer: {
      label: "temperate conifer",
      note: "Scots pine with a clear lower trunk over heather and grass",
      tree: "pine", shrub: "heather", grass: "grass_tuft", rock: "boulder",
      mix: [["pine", 32], ["heather", 22], ["grass_tuft", 32], ["boulder", 10], ["rock_cluster", 4]],
      pal: palette({ bark: 0x6d4a30, leaf: 0x33512f, scrub: 0x4a4d33, grass: 0x63713f, grassDry: 0x8f8a55, rock: 0x777a7c, soil: 0x4e4132, sand: 0x94886c, crop: 0x6f7f3d }),
    },
    boreal: {
      label: "boreal",
      note: "close-grown spruce spires, moss and lichen, granite erratics",
      tree: "spruce", shrub: "heather", grass: "sedge_tussock", rock: "boulder",
      mix: [["spruce", 42], ["heather", 20], ["sedge_tussock", 24], ["boulder", 10], ["rock_cluster", 4]],
      pal: palette({ bark: 0x4a3c30, leaf: 0x28442f, scrub: 0x40563a, grass: 0x63703f, grassDry: 0x8a8455, rock: 0x71757b, soil: 0x413528, sand: 0x8d876e, snow: 0xe4e9ed, crop: 0x6b7b3c }),
    },
    mediterranean: {
      label: "mediterranean",
      note: "holm oak and maquis over dry bunchgrass on pale limestone",
      tree: "holm_oak", shrub: "maquis", grass: "bunchgrass", rock: "rock_cluster",
      mix: [["holm_oak", 22], ["maquis", 28], ["bunchgrass", 30], ["boulder", 10], ["rock_cluster", 10]],
      pal: palette({ bark: 0x6a5b47, leaf: 0x3a5330, scrub: 0x4f6135, grass: 0x7d8149, grassDry: 0xab9d66, rock: 0xa79d87, soil: 0x8a6a4c, sand: 0xbfae87, crop: 0x8d9048 }),
    },
    arid: {
      label: "arid / desert",
      note: "saguaro and creosote at wide spacing, stone pavement and scree",
      tree: "saguaro", shrub: "creosote", grass: "bunchgrass", rock: "scree_patch",
      mix: [["saguaro", 10], ["creosote", 32], ["bunchgrass", 20], ["boulder", 14], ["rock_cluster", 12], ["scree_patch", 12]],
      pal: palette({ bark: 0x6b5a3e, leaf: 0x4f6c3c, scrub: 0x5f6f3f, grass: 0x93894f, grassDry: 0xb2a273, rock: 0xa17f5e, soil: 0x8b6d4c, sand: 0xc2aa7a, crop: 0x9a9450 }),
    },
    tropical: {
      label: "tropical",
      note: "palms over broadleaf thicket, deep green, red-brown soil",
      tree: "palm", shrub: "tropical_thicket", grass: "grass_tuft", rock: "boulder",
      mix: [["palm", 26], ["tropical_thicket", 30], ["grass_tuft", 32], ["boulder", 8], ["rock_cluster", 4]],
      pal: palette({ bark: 0x6f6152, leaf: 0x2f6b34, scrub: 0x39763a, grass: 0x40793a, grassDry: 0x8c8c46, rock: 0x6f6b5f, soil: 0x6b4b34, sand: 0xbfae8b, crop: 0x4f8438 }),
    },
    alpine: {
      label: "alpine",
      note: "krummholz below the treeline, cushion plants, scree and late snow",
      tree: "krummholz", shrub: "alpine_cushion", grass: "sedge_tussock", rock: "scree_patch",
      mix: [["krummholz", 16], ["alpine_cushion", 24], ["sedge_tussock", 22], ["boulder", 16], ["rock_cluster", 12], ["scree_patch", 10]],
      pal: palette({ bark: 0x4e4238, leaf: 0x35492f, scrub: 0x4b5a38, grass: 0x6c7b4a, grassDry: 0x968f5d, rock: 0x8b8a86, soil: 0x50483c, sand: 0x9d9890, snow: 0xe8edf1, crop: 0x74833f }),
    },
    tundra: {
      label: "tundra",
      note: "no trees: dwarf birch and heath over sedge, lichen-blotched rock",
      tree: null, shrub: "dwarf_birch", grass: "sedge_tussock", rock: "boulder",
      mix: [["dwarf_birch", 26], ["heather", 22], ["sedge_tussock", 30], ["boulder", 14], ["scree_patch", 8]],
      pal: palette({ bark: 0x594a3e, leaf: 0x566139, scrub: 0x6a5a37, grass: 0x7c7a4e, grassDry: 0x9d9260, rock: 0x86857e, soil: 0x4b4236, sand: 0x94896f, snow: 0xe4e9ee, crop: 0x7c7a4e }),
    },
    savanna: {
      label: "savanna",
      note: "flat-crowned acacia standing well apart over dry bunchgrass",
      tree: "acacia", shrub: "thorn_scrub", grass: "bunchgrass", rock: "rock_cluster",
      mix: [["acacia", 16], ["thorn_scrub", 24], ["bunchgrass", 42], ["boulder", 10], ["rock_cluster", 8]],
      pal: palette({ bark: 0x6e6252, leaf: 0x5c6d36, scrub: 0x6b6f3a, grass: 0x97904f, grassDry: 0xb69d60, rock: 0x8f7f66, soil: 0x7a5b3e, sand: 0xc0aa7c, crop: 0x9c9450 }),
    },
  };

  // ------------------------------------------------------------------ trees
  // FOUR BUILDERS, EIGHT SPECIES. A broadleaf, a conifer, a palm and a cactus
  // are four different plants structurally, and no amount of parameter on one
  // of them produces another. Within a builder the parameters are large enough
  // to matter: an oak forks at a third of its height into a rounded crown, an
  // acacia carries a bare trunk to two thirds and then goes flat, a holm oak
  // forks almost at the ground. Those are the same code and they are not the
  // same silhouette.
  //
  // The detail argument `d` is 1 near and 0 far. Far is not a separate
  // authoring of anything — it is this same call graph with the segment counts
  // dropped and the small parts switched off, which is the only way a far LOD
  // cannot drift away from the tree it is a cheaper view of.

  /// Trunk polyline: a base, a bend, and the fork. It leans and it is not
  /// straight, because a straight cylinder is the thing that makes a tree read
  /// as a stick.
  function trunkNodes(h, clearH, r, lean, gnarl, seed, d) {
    const dirX = Math.cos(askUnit(seed, 11) * Math.PI * 2), dirZ = Math.sin(askUnit(seed, 11) * Math.PI * 2);
    const n = d ? 4 : 2;
    const out = [];
    for (let i = 0; i < n; i += 1) {
      const t = i / (n - 1);
      const off = lean * clearH * t * t + gnarl * clearH * 0.12 * Math.sin(t * 4.1 + askUnit(seed, 13) * 6.2);
      out.push({
        p: [dirX * off, clearH * t, dirZ * off],
        r: r * (1 - 0.42 * t) * (1 + (d ? gnarl * 0.35 * askSigned(seed, 17 + i, 1) : 0)),
      });
    }
    return out;
  }

  function broadleaf(m, spec, pal, seed, d) {
    const h = askSpan(seed, 1, spec.h[0], spec.h[1]);
    const clearH = h * spec.clear;
    const trunkR = h * spec.trunkR;
    const forks = d ? spec.forks : 2;
    const seg = d ? 8 : 4;
    const nodes = trunkNodes(h, clearH, trunkR, spec.lean, spec.gnarl, seed, d);
    const top = nodes[nodes.length - 1];
    const barkAt = (i) => mixc(pal.bark, pal.barkLit, 0.25 + 0.5 * ((i * 37) % 7) / 7);

    if (d && spec.roots) {
      m.part("root flare", () => {
        for (let k = 0; k < 5; k += 1) {
          const a = (k / 5) * Math.PI * 2 + askUnit(seed, 21) * 3.1;
          const rr = trunkR * askSpan(seed, 23 + k, 1.5, 2.4);
          m.tri([0, trunkR * 1.5, 0],
            [Math.cos(a) * rr, 0, Math.sin(a) * rr],
            [Math.cos(a + 0.9) * rr * 0.8, 0, Math.sin(a + 0.9) * rr * 0.8], pal.barkDark);
        }
      });
    }
    m.part("trunk", () => {
      m.smooth(() => { m.tube(nodes, seg, barkAt, { capBase: true, capTop: false }); });
    });

    // Limbs leave the fork and CARRY the canopy — every lobe centre below is a
    // limb tip pushed a little further out. That is the difference between a
    // canopy and a ball balanced on a pole.
    const tips = [];
    m.part("limbs", () => {
      for (let f = 0; f < forks; f += 1) {
        const a = (f / forks) * Math.PI * 2 + askUnit(seed, 31) * 6.28;
        const reach = h * spec.spread * askSpan(seed, 41 + f, 0.72, 1.12);
        const rise = h * (1 - spec.clear) * askSpan(seed, 51 + f, 0.42, 0.78) * (0.4 + spec.flat * 0.9);
        const limb = [
          { p: top.p, r: top.r * 0.72 },
          { p: [top.p[0] + Math.cos(a) * reach * 0.45, top.p[1] + rise * 0.62, top.p[2] + Math.sin(a) * reach * 0.45], r: top.r * 0.42 },
          { p: [top.p[0] + Math.cos(a) * reach, top.p[1] + rise, top.p[2] + Math.sin(a) * reach], r: top.r * 0.16 },
        ];
        m.smooth(() => { m.tube(limb, d ? 5 : 3, () => pal.bark, {}); });
        tips.push({ p: limb[2].p, a, reach });
        if (d) {
          const bud = [
            { p: limb[1].p, r: top.r * 0.3 },
            { p: [limb[1].p[0] + Math.cos(a + 1.4) * reach * 0.35, limb[1].p[1] + rise * 0.4, limb[1].p[2] + Math.sin(a + 1.4) * reach * 0.35], r: top.r * 0.1 },
          ];
          m.smooth(() => { m.tube(bud, 4, () => pal.barkDark, {}); });
        }
      }
    });

    // Canopy: one mass per limb tip plus a couple filling the middle, each with
    // its own hashed lumps and its own smoothing group so the seams between
    // them stay visible. Six masses read as foliage; one reads as a lollipop.
    m.part("canopy", () => {
      const lobes = d ? spec.lobes : 2;
      const crownR = h * spec.crown;
      for (let i = 0; i < lobes; i += 1) {
        const t = tips[i % Math.max(1, tips.length)];
        const pull = i < tips.length ? 1 : 0.35;
        const a = t ? t.a : (i / lobes) * 6.28;
        const cx = (t ? t.p[0] : 0) * pull + Math.cos(a + 1.1) * crownR * 0.22 * askSigned(seed, 61 + i, 1);
        const cz = (t ? t.p[2] : 0) * pull + Math.sin(a + 1.1) * crownR * 0.22 * askSigned(seed, 71 + i, 1);
        const cy = (t ? t.p[1] : clearH + h * 0.3) + crownR * (spec.leafShift || 0)
          + crownR * 0.16 * askSigned(seed, 81 + i, 1);
        const rr = crownR * askSpan(seed, 91 + i, 0.46, 0.70) * (i < tips.length ? 1 : 1.2);
        const sd = mix32(seed ^ Math.imul(i + 1, 0x2545f491));
        m.smooth(() => {
          m.blob(cx, cy, cz, rr, rr * spec.flat, rr * askSpan(seed, 101 + i, 0.85, 1.1),
            d ? 7 : 4, d ? 4 : 3, sd, d ? 0.22 : 0.12,
            mixc(pal.leafDark, pal.leaf, 0.55), pal.leafLit, askSigned(seed, 111 + i, 0.16));
        });
      }
    });
    return h;
  }

  /// A conifer is a trunk with tiers of branches on it, and the whole art of it
  /// is that a tier must not read as a cone. Each tier's ring radius varies per
  /// segment and its droop varies with it, so the outline is a row of branch
  /// tips rather than a smooth edge; each tier gets its own phase so two tiers
  /// never line up. `flag` pushes every tier downwind, which is the alpine
  /// krummholz silhouette and nothing else in the kit looks like it.
  function conifer(m, spec, pal, seed, d) {
    const h = askSpan(seed, 1, spec.h[0], spec.h[1]);
    const clearH = h * spec.clear;
    const trunkR = h * spec.trunkR;
    const seg = d ? 7 : 4;
    const lean = spec.lean;
    const dirA = askUnit(seed, 11) * Math.PI * 2;
    const nodes = [];
    const nn = d ? 4 : 2;
    for (let i = 0; i < nn; i += 1) {
      const t = i / (nn - 1);
      const off = lean * h * 0.5 * t * t;
      nodes.push({
        p: [Math.cos(dirA) * off, h * t, Math.sin(dirA) * off],
        r: trunkR * (1 - 0.82 * t) + 0.008,
      });
    }
    m.part("trunk", () => {
      m.smooth(() => {
        m.tube(nodes, seg, (i) => mixc(pal.bark, pal.barkLit, spec.barkTop ? i / Math.max(1, nn - 1) : 0.2),
          { capBase: true });
      });
    });
    if (spec.snag && d) {
      m.part("dead snag", () => {
        const a = dirA + 2.4;
        m.smooth(() => {
          m.tube([
            { p: [0, h * 0.2, 0], r: trunkR * 0.4 },
            { p: [Math.cos(a) * h * 0.5, h * 0.55, Math.sin(a) * h * 0.5], r: trunkR * 0.1 },
          ], 4, () => pal.dead, {});
        });
      });
    }
    m.part("branch tiers", () => {
      const tiers = d ? spec.tiers : Math.max(2, Math.round(spec.tiers * 0.4));
      for (let i = 0; i < tiers; i += 1) {
        const t = tiers === 1 ? 0.5 : i / (tiers - 1);
        const y = clearH + (h - clearH) * (spec.spire ? t : 0.15 + 0.85 * t);
        const shrink = spec.spire ? (1 - t) : Math.sin(0.5 + t * 2.2);
        const rr = h * spec.spread * Math.max(0.12, shrink) * (1 - spec.taper * t);
        const off = lean * h * 0.5 * (y / h) * (y / h);
        const flag = (spec.flag || 0) * rr;
        const cx = Math.cos(dirA) * off + Math.cos(dirA) * flag;
        const cz = Math.sin(dirA) * off + Math.sin(dirA) * flag;
        const tierSeg = d ? 9 : 5;
        const phase = askUnit(seed, 121 + i) * Math.PI * 2;
        const pts = [];
        for (let k = 0; k < tierSeg; k += 1) {
          const a = (k / tierSeg) * Math.PI * 2 + phase;
          const jag = askSpan(seed, 131 + i * 16 + k, 0.62, 1.15);
          const wind = spec.flag ? (0.35 + 0.65 * (0.5 + 0.5 * Math.cos(a - dirA))) : 1;
          const rad = rr * jag * wind;
          pts.push([cx + Math.cos(a) * rad, y - spec.droop * rad * askSpan(seed, 141 + i * 16 + k, 0.6, 1.3), cz + Math.sin(a) * rad]);
        }
        const apex = [cx, y + rr * (spec.plateTop ? 0.16 : 0.34), cz];
        const skirt = [cx, y - rr * 0.10, cz];
        const dark = mixc(pal.leafDark, pal.leaf, 0.3);
        for (let k = 0; k < tierSeg; k += 1) {
          const j = (k + 1) % tierSeg;
          m.tri(pts[k], apex, pts[j], pal.leafLit);
          m.tri(pts[k], pts[j], skirt, dark);
        }
        // A second, smaller plate just under the first: a tier is a mass of
        // branches, and one plate per tier is the thing that reads as a stack
        // of cones. This costs a third of a tier and removes that read.
        if (d) {
          const inner = pts.map((p) => [cx + (p[0] - cx) * 0.62, p[1] - rr * 0.22, cz + (p[2] - cz) * 0.62]);
          const low = [cx, y - rr * 0.34, cz];
          const high = [cx, y - rr * 0.02, cz];
          // Closed, not a one-sided plate. An open fan is invisible from above
          // under backface culling and it also makes the signed volume of the
          // tier negative, which is the number the checks read to catch an
          // inverted winding. Nine triangles to keep that test meaningful.
          for (let k = 0; k < tierSeg; k += 1) {
            const j = (k + 1) % tierSeg;
            m.tri(inner[k], inner[j], low, pal.leafDark);
            m.tri(inner[k], high, inner[j], pal.leaf);
          }
        }
      }
    });
    return h;
  }

  /// A palm is a bare curved stem and a crown of arching fronds, and the fronds
  /// have to be RIBS WITH LEAFLETS or the tree reads as a mop. Each frond is a
  /// tapered rib tube with leaflet pairs stepped along it; the leaflets are the
  /// same V-section blade the grass uses, so they are visible from both sides
  /// and cost nothing in fill rate.
  function palm(m, spec, pal, seed, d) {
    const h = askSpan(seed, 1, spec.h[0], spec.h[1]);
    const bend = askSpan(seed, 3, 0.06, 0.19) * h;
    const dirA = askUnit(seed, 5) * Math.PI * 2;
    const seg = d ? 7 : 4;
    const nn = d ? 6 : 3;
    const nodes = [];
    for (let i = 0; i < nn; i += 1) {
      const t = i / (nn - 1);
      // Leaf-scar rings: the radius steps rather than tapering smoothly, which
      // is what a palm stem actually looks like and is free here.
      const step = d ? 1 + 0.06 * Math.cos(t * 11) : 1;
      nodes.push({
        p: [Math.cos(dirA) * bend * t * t, h * 0.86 * t, Math.sin(dirA) * bend * t * t],
        r: h * 0.019 * (1 - 0.22 * t) * step,
      });
    }
    const crown = nodes[nn - 1].p;
    m.part("stem", () => {
      m.smooth(() => {
        m.tube(nodes, seg, (i) => mixc(pal.bark, pal.barkLit, 0.2 + 0.5 * (i % 2)), { capBase: true, capTop: false });
      });
    });
    m.part("crown shaft", () => {
      m.smooth(() => {
        m.blob(crown[0], crown[1] + h * 0.02, crown[2], h * 0.026, h * 0.05, h * 0.026,
          d ? 6 : 4, 3, mix32(seed ^ 0x51), 0.08, pal.leafDark, pal.leaf, 0);
      });
    });
    m.part("fronds", () => {
      const count = d ? spec.fronds : 4;
      for (let f = 0; f < count; f += 1) {
        const a = (f / count) * Math.PI * 2 + askUnit(seed, 7) * 6.28;
        const reach = h * askSpan(seed, 61 + f, 0.28, 0.40);
        const lift = h * askSpan(seed, 71 + f, 0.06, 0.17);
        const rib = [
          { p: crown, r: h * 0.008 },
          { p: [crown[0] + Math.cos(a) * reach * 0.45, crown[1] + lift, crown[2] + Math.sin(a) * reach * 0.45], r: h * 0.005 },
          { p: [crown[0] + Math.cos(a) * reach, crown[1] + lift * 0.1 - h * 0.06, crown[2] + Math.sin(a) * reach], r: h * 0.0018 },
        ];
        m.smooth(() => { m.tube(rib, 3, () => pal.leafDark, { capTop: false }); });
        if (!d) {
          // Far: the frond is its own outline. Two triangles a side, and at map
          // range that is the entire difference between a palm and a post.
          const w = reach * 0.16;
          const midA = [rib[1].p[0] - Math.sin(a) * w, rib[1].p[1], rib[1].p[2] + Math.cos(a) * w];
          const midB = [rib[1].p[0] + Math.sin(a) * w, rib[1].p[1], rib[1].p[2] - Math.cos(a) * w];
          m.tri(rib[0].p, midA, rib[2].p, pal.leaf);
          m.tri(rib[0].p, rib[2].p, midB, pal.leafLit);
          continue;
        }
        for (let s = 1; s <= 4; s += 1) {
          const t = s / 5;
          const base = [
            rib[0].p[0] + (rib[2].p[0] - rib[0].p[0]) * t,
            rib[0].p[1] + lift * (t < 0.5 ? t * 2 : 1) - h * 0.06 * t * t,
            rib[0].p[2] + (rib[2].p[2] - rib[0].p[2]) * t,
          ];
          const leafLen = reach * 0.30 * (1 - 0.5 * Math.abs(t - 0.45));
          for (const side of [1, -1]) {
            m.save().move(base[0], base[1], base[2]).rotY(-a - side * 1.15).rotZ(-0.55);
            m.blade(0, 0, leafLen, leafLen * 0.30, 0, 0.35,
              mixc(pal.leaf, pal.leafDark, 0.35), pal.leafLit, 3);
            m.restore();
          }
        }
      }
    });
    if (d) {
      m.part("fruit", () => {
        for (let i = 0; i < 3; i += 1) {
          const a = (i / 3) * 6.28 + 0.7;
          m.smooth(() => {
            m.blob(crown[0] + Math.cos(a) * h * 0.03, crown[1] - h * 0.02, crown[2] + Math.sin(a) * h * 0.03,
              h * 0.016, h * 0.016, h * 0.016, 5, 3, mix32(seed ^ (i + 7)), 0.05, pal.dead, pal.barkLit, 0);
          });
        }
      });
    }
    return h;
  }

  /// A saguaro is a fluted column with elbowed arms, and the flutes are the
  /// whole silhouette: a smooth cylinder with two bends on it is a plumbing
  /// diagram. `flute` modulates the tube radius around the ring, so the ribs
  /// are geometry that catches the sun on one side, which is what a cactus
  /// looks like at any distance a player will see one.
  function saguaro(m, spec, pal, seed, d) {
    const h = askSpan(seed, 1, spec.h[0], spec.h[1]);
    const r = h * askSpan(seed, 3, 0.045, 0.062);
    const seg = d ? 16 : 6;
    const ribs = 8;
    const flute = d ? 0.11 : 0;
    const green = mixc(pal.leaf, [0.30, 0.44, 0.26], 0.5);
    const greenLit = sh(green, 1.3);
    const nn = d ? 4 : 3;
    const nodes = [];
    for (let i = 0; i < nn; i += 1) {
      const t = i / (nn - 1);
      nodes.push({ p: [0, h * 0.94 * t, 0], r: r * (1 - 0.18 * t * t) });
    }
    nodes.push({ p: [0, h, 0], r: r * 0.66 });
    m.part("column", () => {
      m.smooth(() => {
        m.tube(nodes, seg, () => green, { capBase: true, capTop: false, flute, flutes: ribs });
      });
      m.smooth(() => {
        m.dome(0, h - r * 0.1, 0, r * 0.68, r * 0.8, r * 0.68, d ? 10 : 4, 2,
          mix32(seed ^ 0x33), 0.05, green, greenLit, 0.5);
      });
    });
    m.part("arms", () => {
      const arms = d ? spec.arms : 1;
      for (let i = 0; i < arms; i += 1) {
        const a = askUnit(seed, 21 + i) * Math.PI * 2;
        const y0 = h * askSpan(seed, 31 + i, 0.42, 0.58);
        const out = r * askSpan(seed, 41 + i, 3.2, 5.0);
        const up = h * askSpan(seed, 51 + i, 0.24, 0.40);
        const ar = r * 0.62;
        const arm = [
          { p: [Math.cos(a) * r * 0.6, y0, Math.sin(a) * r * 0.6], r: ar * 0.9 },
          { p: [Math.cos(a) * out * 0.7, y0 + up * 0.12, Math.sin(a) * out * 0.7], r: ar },
          { p: [Math.cos(a) * out, y0 + up * 0.5, Math.sin(a) * out], r: ar * 0.95 },
          { p: [Math.cos(a) * out, y0 + up, Math.sin(a) * out], r: ar * 0.8 },
        ];
        m.smooth(() => {
          m.tube(d ? arm : [arm[0], arm[2], arm[3]], d ? 10 : 5, () => green,
            { capBase: true, capTop: false, flute: flute * 0.8, flutes: ribs });
        });
        m.smooth(() => {
          m.dome(Math.cos(a) * out, y0 + up - ar * 0.1, Math.sin(a) * out, ar * 0.8, ar * 0.9, ar * 0.8,
            d ? 8 : 4, 2, mix32(seed ^ (i + 3)), 0.04, green, greenLit, 0.5);
        });
      }
    });
    return h;
  }

  // ----------------------------------------------------------------- shrubs
  // Four styles, eight shrubs. A shrub is the cheapest thing that says which
  // biome you are in when the trees are too far apart to say it — a mediterranean
  // maquis is a dense round mound, a creosote bush is open and leggy with sky
  // through it, a cushion plant is a dome that hugs the rock, and a dwarf birch
  // mat is wider than it is tall. Those are different shapes, not different
  // greens.
  function shrub(m, spec, pal, seed, d) {
    const h = askSpan(seed, 1, spec.h[0], spec.h[1]);
    const wide = spec.wide || 1;

    if (spec.style === "cushion") {
      // A cushion plant is many small pads pressed together over stone. Built
      // from domes so it sits ON the ground rather than half inside it.
      m.part("cushion", () => {
        const pads = d ? spec.pads : 4;
        for (let i = 0; i < pads; i += 1) {
          const a = (i / pads) * 6.28 + askUnit(seed, 3) * 6.28;
          const rr = h * askSpan(seed, 11 + i, 1.1, 2.4);
          const dist = h * askSpan(seed, 21 + i, 0.0, 2.6);
          m.smooth(() => {
            m.dome(Math.cos(a) * dist, 0, Math.sin(a) * dist, rr, h * askSpan(seed, 31 + i, 0.6, 1.15), rr * 0.92,
              d ? 6 : 4, 2, mix32(seed ^ Math.imul(i + 1, 0x9e37)), 0.16,
              mixc(pal.scrub, pal.leafDark, 0.4), pal.leafLit, 0.62);
          });
        }
      });
      return h;
    }

    if (spec.style === "leaves") {
      // Tropical thicket: a few short stems carrying BIG leaves. The leaf is
      // the identifying feature, so it is a blade a metre long rather than a
      // lobe, and there is no canopy mass at all.
      m.part("stems", () => {
        const stems = d ? 4 : 2;
        for (let i = 0; i < stems; i += 1) {
          const a = (i / stems) * 6.28 + askUnit(seed, 3) * 6.28;
          m.smooth(() => {
            m.tube([
              { p: [0, 0, 0], r: h * 0.035 },
              { p: [Math.cos(a) * h * 0.12, h * 0.55, Math.sin(a) * h * 0.12], r: h * 0.018 },
            ], d ? 5 : 3, () => pal.bark, { capBase: true });
          });
        }
      });
      m.part("leaves", () => {
        const leaves = d ? spec.leaves : 4;
        for (let i = 0; i < leaves; i += 1) {
          const a = (i / leaves) * 6.28 + askUnit(seed, 5) * 6.28;
          const y = h * askSpan(seed, 41 + i, 0.30, 0.72);
          const len = h * askSpan(seed, 51 + i, 0.42, 0.66);
          m.save().move(Math.cos(a) * h * 0.08, y, Math.sin(a) * h * 0.08)
            .rotY(-a).rotZ(-askSpan(seed, 61 + i, 0.5, 1.05));
          m.blade(0, 0, len, len * 0.44, 0, 0.5, pal.leaf, pal.leafLit, d ? 3 : 2);
          m.restore();
        }
      });
      return h;
    }

    const stems = d ? spec.stems : 2;
    const flat = spec.flat || (spec.style === "mat" ? 0.34 : spec.dense ? 0.86 : 0.72);
    const tips = [];
    m.part("stems", () => {
      for (let i = 0; i < stems; i += 1) {
        const a = (i / stems) * 6.28 + askUnit(seed, 3) * 6.28;
        const reach = h * wide * askSpan(seed, 11 + i, 0.28, 0.62);
        const rise = h * askSpan(seed, 21 + i, 0.55, 0.95);
        const node = [
          { p: [0, 0, 0], r: h * 0.05 },
          { p: [Math.cos(a) * reach * 0.5, rise * 0.55, Math.sin(a) * reach * 0.5], r: h * 0.03 },
          { p: [Math.cos(a) * reach, rise, Math.sin(a) * reach], r: h * 0.014 },
        ];
        m.smooth(() => { m.tube(d ? node : [node[0], node[2]], d ? 4 : 3, () => pal.bark, { capBase: true }); });
        tips.push(node[2].p);
      }
    });
    m.part("foliage", () => {
      const lobes = d ? spec.lobes : 2;
      const open = spec.style === "open";
      for (let i = 0; i < lobes; i += 1) {
        const t = tips[i % tips.length];
        const rr = h * wide * askSpan(seed, 71 + i, open ? 0.20 : 0.34, open ? 0.34 : 0.56);
        m.smooth(() => {
          m.blob(t[0] * 0.85 + askSigned(seed, 81 + i, h * 0.12),
            t[1] + rr * (spec.style === "mat" ? 0.1 : 0.25),
            t[2] * 0.85 + askSigned(seed, 91 + i, h * 0.12),
            rr, rr * flat, rr * askSpan(seed, 101 + i, 0.85, 1.15),
            d ? 6 : 4, d ? 3 : 2, mix32(seed ^ Math.imul(i + 1, 0x85eb)), d ? 0.24 : 0.1,
            mixc(pal.scrub, pal.leafDark, 0.35), pal.leafLit, askSigned(seed, 111 + i, 0.2));
        });
      }
    });
    return h;
  }

  // ---------------------------------------------------------------- grasses
  /// A tuft is blades and a small mound of thatch under them. It is the piece
  /// that will exist in the largest numbers of anything in this kit, so it is
  /// also the one where the far LOD matters most: four blades and no mound.
  function tuft(m, spec, pal, seed, d) {
    const h = askSpan(seed, 1, spec.h[0], spec.h[1]);
    const base = spec.dry ? pal.grassDry : pal.grass;
    if (d && spec.mound) {
      m.part("thatch", () => {
        m.smooth(() => {
          m.dome(0, 0, 0, spec.spread * 0.9, h * 0.22, spec.spread * 0.9, 6, 2,
            mix32(seed ^ 0x77), 0.18, mixc(pal.litter, base, 0.4), base, 0.55);
        });
      });
    }
    m.part("blades", () => {
      const blades = d ? spec.blades : 4;
      for (let i = 0; i < blades; i += 1) {
        const a = askUnit(seed, 11 + i) * Math.PI * 2;
        const dist = spec.spread * askSpan(seed, 31 + i, 0, 0.8);
        const bh = h * askSpan(seed, 51 + i, 0.55, 1.05);
        m.blade(Math.cos(a) * dist, Math.sin(a) * dist, bh, h * 0.09,
          a + askSigned(seed, 71 + i, 1.2), spec.arc * askSpan(seed, 91 + i, 0.6, 1.4),
          mixc(base, pal.leafDark, 0.3), spec.dry ? sh(base, 1.25) : pal.leafLit, d ? 3 : 2);
      }
    });
    return h;
  }

  // ------------------------------------------------------------------ rocks
  /// A boulder is FLAT SHADED on purpose — it never opens a smoothing group.
  /// Facets are the only thing that makes stone read as stone in a renderer
  /// with no textures, and smoothing one turns it into a potato. `bulge` under
  /// a half puts the widest section above the base so the rock overhangs its
  /// own footprint, which is what stops it reading as a dome.
  function boulderAt(m, cx, cz, r, seed, d, pal, tone) {
    const seg = d ? 7 : 5;
    const stacks = d ? 3 : 2;
    const col = sh(pal.rock, tone == null ? 1 : tone);
    m.dome(cx, 0, cz, r, r * askSpan(seed, 3, 0.55, 0.95), r * askSpan(seed, 5, 0.8, 1.2),
      seg, stacks, seed, d ? 0.30 : 0.20, col, sh(pal.rockLit, tone == null ? 1 : tone),
      askSpan(seed, 7, 0.32, 0.46));
    return r;
  }

  function boulder(m, pal, seed, d) {
    const r = askSpan(seed, 1, 0.5, 1.4);
    m.part("boulder", () => { boulderAt(m, 0, 0, r, seed, d, pal, 1); });
    if (d) {
      m.part("lichen and grit", () => {
        for (let i = 0; i < 4; i += 1) {
          const a = askUnit(seed, 21 + i) * 6.28, dist = r * askSpan(seed, 31 + i, 1.05, 1.5);
          boulderAt(m, Math.cos(a) * dist, Math.sin(a) * dist, r * askSpan(seed, 41 + i, 0.10, 0.22),
            mix32(seed ^ (i + 11)), 0, pal, 0.86);
        }
      });
    }
    return r;
  }

  function rockCluster(m, pal, seed, d) {
    const big = askSpan(seed, 1, 0.8, 1.8);
    m.part("boulders", () => {
      const n = d ? 4 : 2;
      for (let i = 0; i < n; i += 1) {
        const a = (i / n) * 6.28 + askUnit(seed, 3) * 6.28;
        const dist = big * askSpan(seed, 11 + i, 0.0, 0.95);
        boulderAt(m, Math.cos(a) * dist, Math.sin(a) * dist, big * askSpan(seed, 21 + i, 0.45, 1.0),
          mix32(seed ^ Math.imul(i + 1, 0x1b873593)), d, pal, askSpan(seed, 31 + i, 0.86, 1.12));
      }
    });
    if (d) {
      m.part("broken scree", () => {
        for (let i = 0; i < 9; i += 1) {
          const a = askUnit(seed, 41 + i) * 6.28, dist = big * askSpan(seed, 51 + i, 1.0, 1.9);
          boulderAt(m, Math.cos(a) * dist, Math.sin(a) * dist, big * askSpan(seed, 61 + i, 0.08, 0.20),
            mix32(seed ^ (i + 31)), 0, pal, askSpan(seed, 71 + i, 0.78, 1.05));
        }
      });
    }
    return big;
  }

  /// A scree patch is angular chips on a dust apron, and it is the piece that
  /// makes an arid or alpine slope stop looking like painted ground. The chips
  /// are the same flat-shaded dome at small radius with a hard squash on them.
  function screePatch(m, pal, seed, d) {
    const rad = askSpan(seed, 1, 1.6, 3.0);
    m.part("apron", () => {
      const seg = d ? 10 : 6;
      const pts = [];
      for (let k = 0; k < seg; k += 1) {
        const a = (k / seg) * 6.28;
        const rr = rad * askSpan(seed, 11 + k, 0.75, 1.15);
        pts.push([Math.cos(a) * rr, 0, Math.sin(a) * rr]);
      }
      m.fan(reversed(pts), mixc(pal.rockDark, pal.soil, 0.4));
    });
    m.part("chips", () => {
      const chips = d ? 22 : 6;
      for (let i = 0; i < chips; i += 1) {
        const a = askUnit(seed, 101 + i) * 6.28, dist = rad * Math.sqrt(askUnit(seed, 131 + i));
        boulderAt(m, Math.cos(a) * dist, Math.sin(a) * dist, askSpan(seed, 161 + i, 0.07, 0.24),
          mix32(seed ^ Math.imul(i + 1, 0xcc9e2d51)), 0, pal, askSpan(seed, 191 + i, 0.8, 1.15));
      }
    });
    return rad;
  }

  // -------------------------------------------------------- terrain dressing
  // Six of these are TILES: they repeat along a contour, a coast, a bank or a
  // field edge, and `index` says which tile in the run this is. Everything that
  // touches a tile boundary comes from `strip`, so tile n's right edge is tile
  // n+1's left edge exactly; everything that does not — boulders, cobbles,
  // reeds, hedge bushes — is inset by a margin so it can never straddle a seam
  // and get cut in half.
  const TILE = 12;

  function cliffFace(m, pal, seed, index, d) {
    const cols = d ? 7 : 3;
    const rows = d ? [
      { y: 0.00, z: 0.00, jy: 0.00, jz: 0.20 },
      { y: 1.60, z: -0.35, jy: 0.30, jz: 0.45 },
      { y: 3.40, z: -0.10, jy: 0.40, jz: 0.55 },
      { y: 5.20, z: -0.75, jy: 0.40, jz: 0.50 },
      { y: 7.00, z: -0.40, jy: 0.45, jz: 0.55 },
      { y: 8.60, z: -1.10, jy: 0.35, jz: 0.40 },
      { y: 9.05, z: -2.30, jy: 0.30, jz: 0.35 },
    ] : [
      { y: 0.00, z: 0.00, jy: 0.00, jz: 0.20 },
      { y: 4.20, z: -0.30, jy: 0.40, jz: 0.50 },
      { y: 8.60, z: -1.00, jy: 0.35, jz: 0.40 },
      { y: 9.05, z: -2.30, jy: 0.30, jz: 0.35 },
    ];
    const top = rows.length - 1;
    m.part("rock face", () => {
      m.strip(seed, index, cols, TILE, rows, (r, j, s, ix) => {
        if (r === top - 1) return mixc(pal.rock, pal.grass, 0.35);
        const t = r / Math.max(1, top - 1);
        return sh(mixc(pal.rockDark, pal.rockLit, t * 0.8 + 0.1),
          askSpan(s, (ix * 97 + j * 13 + r) | 0, 0.88, 1.10));
      });
    });
    m.part("clifftop ground", () => {
      m.strip(seed, index, cols, TILE, [rows[top], { y: rows[top].y - 0.05, z: rows[top].z - 2.4, jy: 0.25, jz: 0.4 }],
        () => mixc(pal.grass, pal.soil, 0.3));
    });
    m.part("talus", () => {
      const n = d ? 7 : 2;
      for (let i = 0; i < n; i += 1) {
        // (i + 0.5) stations keep every boulder clear of both seams.
        const x = ((i + 0.5) / n) * TILE;
        const gz = index * 1000 + i;
        boulderAt(m, x + askSigned(seed, gz * 3 + 1, TILE / (n * 3)), askSpan(seed, gz * 3 + 2, 0.2, 1.5),
          askSpan(seed, gz * 3 + 3, 0.28, 0.95), mix32(seed ^ Math.imul(gz + 1, 0x27d4eb2f)), d, pal,
          askSpan(seed, gz * 3 + 4, 0.8, 1.1));
      }
    });
    return { tile: TILE, height: rows[top].y };
  }

  /// Snow-edge: a melting patch with a scalloped rim, a grit line where it has
  /// pulled back, and rock coming through it. This is the piece that makes a
  /// snow line read as a snow line rather than as a paint boundary.
  function snowEdge(m, pal, seed, d) {
    const rad = askSpan(seed, 1, 2.2, 4.4);
    const seg = d ? 16 : 7;
    const rim = [], grit = [];
    for (let k = 0; k < seg; k += 1) {
      const a = (k / seg) * 6.28;
      // Deeper on the lee side: a drift is not a circle.
      const lee = 0.72 + 0.42 * Math.cos(a - askUnit(seed, 3) * 6.28);
      const rr = rad * lee * askSpan(seed, 11 + k, 0.72, 1.18);
      rim.push([Math.cos(a) * rr, 0.05 + 0.04 * askUnit(seed, 41 + k), Math.sin(a) * rr]);
      grit.push([Math.cos(a) * rr * 1.14, 0.0, Math.sin(a) * rr * 1.14]);
    }
    m.part("melt grit", () => { m.band(grit, rim, mixc(pal.soil, pal.rockDark, 0.4)); });
    m.part("snow patch", () => {
      m.smooth(() => {
        const crown = [];
        for (let k = 0; k < seg; k += 1) {
          crown.push([rim[k][0] * 0.55, askSpan(seed, 71 + k, 0.22, 0.52), rim[k][2] * 0.55]);
        }
        m.band(rim, crown, pal.snow, sh(pal.snow, 1.04));
        m.fan(reversed(crown), sh(pal.snow, 1.06));
      });
    });
    m.part("rock through the snow", () => {
      const n = d ? 3 : 1;
      for (let i = 0; i < n; i += 1) {
        const a = askUnit(seed, 101 + i) * 6.28, dist = rad * askSpan(seed, 111 + i, 0.15, 0.75);
        boulderAt(m, Math.cos(a) * dist, Math.sin(a) * dist, askSpan(seed, 121 + i, 0.30, 0.8),
          mix32(seed ^ (i + 5)), d, pal, 0.9);
      }
    });
    return rad;
  }

  const SHORE_ROWS = {
    shore_sand: [
      // The datum row carries NO height jitter, in all six tiling kinds. `finish`
      // seats a mesh on its own lowest vertex, so a jittered datum would drop
      // each tile by a different amount and the seam that matches in model space
      // would stop matching in the buffer. Nothing else in the profile can reach
      // below zero, so this row is the minimum by construction.
      { y: 0.00, z: 0.0, jy: 0.00, jz: 0.20 },
      { y: 0.12, z: 1.6, jy: 0.05, jz: 0.30 },
      { y: 0.34, z: 3.4, jy: 0.07, jz: 0.35 },
      { y: 0.64, z: 5.0, jy: 0.10, jz: 0.35 },
      { y: 0.52, z: 6.6, jy: 0.10, jz: 0.35 },
      { y: 0.86, z: 8.6, jy: 0.22, jz: 0.45 },
    ],
    shore_shingle: [
      { y: 0.00, z: 0.0, jy: 0.00, jz: 0.20 },
      { y: 0.38, z: 1.4, jy: 0.10, jz: 0.30 },
      { y: 0.92, z: 2.8, jy: 0.14, jz: 0.35 },
      { y: 0.74, z: 4.4, jy: 0.14, jz: 0.35 },
      { y: 1.05, z: 6.2, jy: 0.16, jz: 0.40 },
      { y: 0.95, z: 8.0, jy: 0.18, jz: 0.45 },
    ],
    shore_shelf: [
      { y: 0.00, z: 0.0, jy: 0.00, jz: 0.15 },
      { y: 0.26, z: 1.2, jy: 0.06, jz: 0.20 },
      { y: 0.30, z: 2.6, jy: 0.06, jz: 0.25 },
      { y: 0.66, z: 4.0, jy: 0.08, jz: 0.25 },
      { y: 0.72, z: 6.0, jy: 0.08, jz: 0.30 },
      { y: 1.16, z: 8.0, jy: 0.16, jz: 0.35 },
    ],
  };

  function shore(m, kind, pal, seed, index, d) {
    const cols = d ? 7 : 3;
    const full = SHORE_ROWS[kind];
    const rows = d ? full : [full[0], full[2], full[4], full[5]];
    const bands = kind === "shore_sand"
      ? [pal.sandWet, pal.sandWet, pal.sand, sh(pal.sand, 1.08), pal.sand, mixc(pal.sand, pal.grass, 0.45)]
      : kind === "shore_shingle"
        ? [sh(pal.rockDark, 0.9), pal.rockDark, pal.rock, pal.rock, pal.rockLit, mixc(pal.rock, pal.grass, 0.4)]
        : [sh(pal.rockDark, 0.85), pal.rockDark, mixc(pal.rockDark, pal.leafDark, 0.35), pal.rock, pal.rockLit,
          mixc(pal.rock, pal.grass, 0.35)];
    const pick = d ? (r) => bands[r] : (r) => bands[[0, 2, 4, 5][r]];
    m.part("beach profile", () => {
      m.strip(seed, index, cols, TILE, rows, (r, j, s, ix) =>
        sh(pick(r), askSpan(s, (ix * 131 + j * 17 + r) | 0, 0.9, 1.08)));
    });
    if (kind === "shore_shingle") {
      m.part("cobbles", () => {
        const n = d ? 20 : 5;
        for (let i = 0; i < n; i += 1) {
          const x = ((i + 0.5) / n) * TILE, gz = index * 1000 + i;
          boulderAt(m, x + askSigned(seed, gz * 5 + 1, TILE / (n * 2.6)), askSpan(seed, gz * 5 + 2, 0.4, 7.4),
            askSpan(seed, gz * 5 + 3, 0.09, 0.26), mix32(seed ^ Math.imul(gz + 3, 0x85ebca6b)), 0, pal,
            askSpan(seed, gz * 5 + 4, 0.78, 1.15));
        }
      });
    }
    if (kind === "shore_shelf" && d) {
      m.part("tide pools", () => {
        for (let i = 0; i < 3; i += 1) {
          const x = ((i + 0.5) / 3) * TILE + askSigned(seed, 201 + i, 1.2);
          const z = askSpan(seed, 211 + i, 2.0, 5.4), rr = askSpan(seed, 221 + i, 0.5, 1.0);
          const lip = ring(x, 0.30, z, rr, rr * 0.8, 7, askUnit(seed, 231 + i) * 6.28);
          const bed = ring(x, 0.14, z, rr * 0.6, rr * 0.5, 7, askUnit(seed, 231 + i) * 6.28);
          m.band(reversed(lip), reversed(bed), pal.rockDark);
          m.fan(reversed(bed), sh(pal.water, 1.1));
        }
      });
    }
    if (kind === "shore_sand" && d) {
      m.part("strandline and marram", () => {
        for (let i = 0; i < 9; i += 1) {
          const x = ((i + 0.5) / 9) * TILE + askSigned(seed, 301 + i, 0.4);
          if (i % 3 === 0) {
            m.blade(x, askSpan(seed, 321 + i, 8.0, 9.0), askSpan(seed, 331 + i, 0.35, 0.7), 0.06,
              askUnit(seed, 341 + i) * 6.28, 0.5, mixc(pal.grass, pal.grassDry, 0.5), pal.leafLit, 3);
          } else {
            boulderAt(m, x, askSpan(seed, 351 + i, 2.4, 3.6), askSpan(seed, 361 + i, 0.05, 0.14),
              mix32(seed ^ (i + 61)), 0, pal, 0.75);
          }
        }
      });
    }
    return { tile: TILE, depth: full[full.length - 1].z };
  }

  /// A river bank is the one profile here that is NOT a heightfield: the cut
  /// bank undercuts, so row 3 sits back over row 2 towards the water. That
  /// overhang is what makes a bank read as a bank rather than as a ramp, and
  /// `strip` handles it because it lofts a profile rather than sampling heights.
  function riverBank(m, pal, seed, index, d) {
    const cols = d ? 7 : 3;
    const full = [
      { y: 0.00, z: 0.00, jy: 0.00, jz: 0.20 },
      { y: 0.20, z: 0.90, jy: 0.06, jz: 0.25 },
      { y: 0.26, z: 1.55, jy: 0.07, jz: 0.25 },
      { y: 1.00, z: 1.28, jy: 0.16, jz: 0.28 },
      { y: 1.75, z: 1.80, jy: 0.18, jz: 0.30 },
      { y: 2.10, z: 2.65, jy: 0.14, jz: 0.35 },
      { y: 2.16, z: 4.30, jy: 0.16, jz: 0.45 },
    ];
    const rows = d ? full : [full[0], full[2], full[3], full[5], full[6]];
    const bands = [sh(pal.rockDark, 0.9), pal.rock, mixc(pal.rock, pal.soil, 0.5),
      pal.soilDark, pal.soil, mixc(pal.soil, pal.grass, 0.55), pal.grass];
    const pick = d ? (r) => bands[r] : (r) => bands[[0, 2, 3, 5, 6][r]];
    m.part("bank profile", () => {
      m.strip(seed, index, cols, TILE, rows, (r, j, s, ix) =>
        sh(pick(r), askSpan(s, (ix * 149 + j * 19 + r) | 0, 0.9, 1.08)));
    });
    m.part("gravel bar", () => {
      const n = d ? 14 : 4;
      for (let i = 0; i < n; i += 1) {
        const x = ((i + 0.5) / n) * TILE, gz = index * 1000 + i;
        boulderAt(m, x + askSigned(seed, gz * 7 + 1, TILE / (n * 2.6)), askSpan(seed, gz * 7 + 2, 0.1, 1.2),
          askSpan(seed, gz * 7 + 3, 0.08, 0.24), mix32(seed ^ Math.imul(gz + 5, 0xc2b2ae35)), 0, pal,
          askSpan(seed, gz * 7 + 4, 0.82, 1.12));
      }
    });
    if (d) {
      m.part("exposed roots", () => {
        for (let i = 0; i < 3; i += 1) {
          const x = ((i + 0.5) / 3) * TILE + askSigned(seed, 401 + i, 0.8);
          const y = askSpan(seed, 411 + i, 1.15, 1.7);
          m.smooth(() => {
            m.tube([
              { p: [x, y, 2.0], r: 0.07 },
              { p: [x + askSigned(seed, 421 + i, 0.5), y - 0.25, 1.35], r: 0.05 },
              { p: [x + askSigned(seed, 431 + i, 0.9), y - 0.62, 1.1], r: 0.02 },
            ], 4, () => pal.dead, {});
          });
        }
      });
      m.part("reeds", () => {
        for (let i = 0; i < 12; i += 1) {
          const x = ((i + 0.5) / 12) * TILE + askSigned(seed, 501 + i, 0.3);
          m.blade(x, askSpan(seed, 511 + i, 0.5, 1.5), askSpan(seed, 521 + i, 0.55, 1.15), 0.05,
            askUnit(seed, 531 + i) * 6.28, 0.35, mixc(pal.grass, pal.leafDark, 0.3), pal.leafLit, 3);
        }
      });
    }
    return { tile: TILE, depth: full[full.length - 1].z };
  }

  /// A hedgerow on a bank with a ditch beside it: the 1990 field boundary in
  /// most of temperate Europe, and the reason a field patch reads as farmland
  /// rather than as a coloured rectangle. Bushes stand at (k+0.5) stations so
  /// two abutting tiles leave one clean gap rather than a doubled bush.
  function hedgerow(m, pal, seed, index, d, opt) {
    const cols = d ? 6 : 3;
    const rows = [
      { y: 0.30, z: 0.00, jy: 0.03, jz: 0.10 },
      { y: 0.42, z: 0.80, jy: 0.06, jz: 0.15 },
      { y: 0.78, z: 1.50, jy: 0.10, jz: 0.18 },
      { y: 0.50, z: 2.20, jy: 0.08, jz: 0.18 },
      { y: 0.00, z: 3.00, jy: 0.00, jz: 0.15 },
    ];
    if (!opt || opt.bank !== false) {
      m.part("bank and ditch", () => {
        m.strip(seed, index, cols, TILE, rows, (r) => r >= 3 ? mixc(pal.soil, pal.grass, 0.4)
          : mixc(pal.grass, pal.soil, r === 2 ? 0.15 : 0.3));
      });
    }
    m.part("hedge", () => {
      const n = d ? 6 : 3;
      for (let i = 0; i < n; i += 1) {
        const x = ((i + 0.5) / n) * TILE, gz = index * 1000 + i;
        const sd = mix32(seed ^ Math.imul(gz + 7, 0x9e3779b1));
        const lobes = d ? 3 : 1;
        for (let l = 0; l < lobes; l += 1) {
          const rr = askSpan(sd, 11 + l, 0.55, 0.95);
          m.smooth(() => {
            m.blob(x + askSigned(sd, 21 + l, 0.4), 1.05 + rr * 0.85 + askSigned(sd, 31 + l, 0.2),
              1.4 + askSigned(sd, 41 + l, 0.35), rr, rr * 1.15, rr * 0.85,
              d ? 6 : 4, d ? 3 : 2, mix32(sd ^ (l + 1)), d ? 0.26 : 0.12,
              mixc(pal.scrub, pal.leafDark, 0.45), pal.leafLit, 0);
          });
        }
      }
      if (d) {
        // One hedgerow tree per few tiles. Hashed on the tile index so the run
        // has trees in it without every tile carrying one.
        if (ask(seed, index * 13 + 3) % 3 === 0) {
          m.save().move(TILE * 0.5, 0.7, 1.45);
          broadleaf(m, { ...TREES.oak, h: [5, 8], lobes: 4, forks: 3 }, pal, mix32(seed ^ (index + 101)), 0);
          m.restore();
        }
      }
    });
    return { tile: TILE, depth: 3.0 };
  }

  const CROPS = {
    ploughed: { ridge: 0.16, spacing: 1.05, colA: "soil", colB: "soilDark", stubble: 0 },
    stubble: { ridge: 0.10, spacing: 1.25, colA: "grassDry", colB: "soil", stubble: 26 },
    cereal: { ridge: 0.22, spacing: 1.15, colA: "crop", colB: "grass", stubble: 0 },
    pasture: { ridge: 0.0, spacing: 0, colA: "grass", colB: "grass", stubble: 34 },
  };

  /// A field patch: worked ground with plough lines and a hedged boundary. The
  /// furrows are what carries it — a brown rectangle is a texture nobody has,
  /// and thirty ridges of four triangles each are what make it agricultural
  /// land from the air. The size is a smallholding rather than a prairie
  /// because the 1990 baseline in most of the world is a hedged field.
  function fieldPatch(m, pal, seed, d, opt) {
    const w = (opt && opt.width) || 48, dep = (opt && opt.depth) || 36;
    const cropKey = opt && CROPS[opt.crop] ? opt.crop : askOne(seed, 3, Object.keys(CROPS));
    const crop = CROPS[cropKey];
    const a = pal[crop.colA] || pal.soil, b = pal[crop.colB] || pal.soilDark;
    // The surface is a tonal grid rather than one quad. It is the same flat
    // ground either way, so this is not detail it does not have — it is the
    // patchiness a worked field has and a single quad cannot show, and it is
    // what keeps a pasture (which has no furrows to draw) from arriving at the
    // map level as eight triangles of nothing.
    m.part("field surface", () => {
      const gx = d ? 4 : 3, gz = d ? 4 : 3;
      for (let i = 0; i < gx; i += 1) {
        for (let j = 0; j < gz; j += 1) {
          m.slab((i / gx) * w, ((i + 1) / gx) * w, (j / gz) * dep, ((j + 1) / gz) * dep, 0.02,
            sh(mixc(a, b, 0.5), askSpan(seed, 601 + i * 8 + j, 0.92, 1.08)));
        }
      }
    });
    if (crop.spacing > 0) {
      m.part("plough lines", () => {
        const spacing = crop.spacing * (d ? 1 : 3.2);
        const n = Math.max(2, Math.floor((dep - 4) / spacing));
        for (let i = 0; i < n; i += 1) {
          const zc = 2 + (i + 0.5) * ((dep - 4) / n);
          m.ridge(1.5, w - 1.5, zc, spacing * 0.46,
            crop.ridge * askSpan(seed, 11 + i, 0.8, 1.2), b, a);
        }
      });
    }
    if (crop.stubble > 0 && d) {
      m.part("standing growth", () => {
        for (let i = 0; i < crop.stubble; i += 1) {
          const x = askSpan(seed, 101 + i, 2, w - 2), z = askSpan(seed, 201 + i, 2, dep - 2);
          m.blade(x, z, askSpan(seed, 301 + i, 0.12, 0.3), 0.05, askUnit(seed, 401 + i) * 6.28,
            0.4, a, sh(a, 1.2), 2);
        }
      });
    }
    m.part("headland track", () => {
      m.slab(0, w, dep - 1.4, dep, 0.05, mixc(pal.soil, pal.rock, 0.35));
    });
    // Boundary. At near detail the hedge is the hedge kit run along two edges;
    // at map range it is a dark band, which is all a hedge is at that size.
    m.part("boundary", () => {
      if (!d) {
        m.slab(0, w, -0.9, 0, 0.06, pal.leafDark);
        m.slab(0, w, dep, dep + 0.9, 0.06, pal.leafDark);
        return;
      }
      const runs = Math.max(1, Math.round(w / TILE));
      for (let i = 0; i < runs; i += 1) {
        m.save().move(i * TILE, 0, -3.0);
        hedgerow(m, pal, mix32(seed ^ 0x5bf0), i, 0, { bank: false });
        m.restore();
        m.save().move(i * TILE, 0, dep).rotY(0);
        hedgerow(m, pal, mix32(seed ^ 0x77a1), i, 0, { bank: false });
        m.restore();
      }
      m.bar(w * 0.45, w * 0.45 + 0.12, 0, 1.2, dep + 1.2, dep + 1.3, pal.dead);
      m.bar(w * 0.45 + 3.2, w * 0.45 + 3.32, 0, 1.2, dep + 1.2, dep + 1.3, pal.dead);
      for (let r = 0; r < 3; r += 1) {
        m.bar(w * 0.45, w * 0.45 + 3.32, 0.35 + r * 0.35, 0.42 + r * 0.35, dep + 1.22, dep + 1.28, pal.dead);
      }
    });
    return { crop: cropKey, width: w, depth: dep };
  }

  // ------------------------------------------------------------------ kinds
  // Budgets. The tree row is roadmap section 4 verbatim; the rest are this
  // file's own and are set where they are because these pieces exist in far
  // larger numbers than a tree does. A tuft of grass at 60 triangles is only
  // affordable because there is a 12-triangle version of it for the map, and
  // the check asserts BOTH ends of every band — a piece that quietly fell to
  // four triangles has stopped being the thing it claims to be.
  const BUDGET = {
    tree: { near: [100, 800], far: [20, 100] },
    shrub: { near: [30, 420], far: [6, 100] },
    grass: { near: [16, 240], far: [4, 60] },
    rock: { near: [30, 520], far: [6, 120] },
    dressing: { near: [80, 1800], far: [16, 360] },
  };

  const KINDS = {};
  function register(kind, family, biome, label, blurb, draw, extra) {
    KINDS[kind] = Object.assign({ kind, family, biome, label, blurb, draw }, extra || {});
  }
  for (const key of Object.keys(TREES)) {
    const spec = TREES[key];
    const home = Object.keys(BIOMES).find((b) => BIOMES[b].tree === key) || "temperate_broadleaf";
    register(key, "tree", home, spec.label, spec.common, (m, c) => {
      const fn = spec.builder === "broadleaf" ? broadleaf : spec.builder === "conifer" ? conifer
        : spec.builder === "palm" ? palm : saguaro;
      return fn(m, spec, c.pal, c.seed, c.d);
    }, { radius: 3.2 });
  }
  for (const key of Object.keys(SHRUBS)) {
    const spec = SHRUBS[key];
    const home = Object.keys(BIOMES).find((b) => BIOMES[b].shrub === key) || "temperate_broadleaf";
    register(key, "shrub", home, spec.label, "understorey and open scrub",
      (m, c) => shrub(m, spec, c.pal, c.seed, c.d), { radius: 1.3 });
  }
  for (const key of Object.keys(GRASSES)) {
    const spec = GRASSES[key];
    const home = Object.keys(BIOMES).find((b) => BIOMES[b].grass === key) || "temperate_broadleaf";
    register(key, "grass", home, spec.label, "ground cover",
      (m, c) => tuft(m, spec, c.pal, c.seed, c.d), { radius: 0.5 });
  }
  register("boulder", "rock", "temperate_broadleaf", "boulder", "a single erratic with grit around it",
    (m, c) => boulder(m, c.pal, c.seed, c.d), { radius: 1.6 });
  register("rock_cluster", "rock", "alpine", "rock cluster", "four boulders and broken scree",
    (m, c) => rockCluster(m, c.pal, c.seed, c.d), { radius: 2.6 });
  register("scree_patch", "rock", "alpine", "scree patch", "angular chips on a dust apron",
    (m, c) => screePatch(m, c.pal, c.seed, c.d), { radius: 3.0 });
  register("cliff_face", "dressing", "alpine", "cliff face", "a tiling section of rock face with talus at its foot",
    (m, c) => cliffFace(m, c.pal, c.seed, c.index, c.d), { tile: TILE, tiles: true, radius: 8 });
  register("snow_edge", "dressing", "alpine", "snow edge", "a melting patch with a grit rim and rock through it",
    (m, c) => snowEdge(m, c.pal, c.seed, c.d), { radius: 4.5 });
  register("shore_sand", "dressing", "temperate_broadleaf", "sand shore", "wet sand, beach face, berm and dune toe",
    (m, c) => shore(m, "shore_sand", c.pal, c.seed, c.index, c.d), { tile: TILE, tiles: true, radius: 9 });
  register("shore_shingle", "dressing", "temperate_broadleaf", "shingle shore", "a cobble bank with two storm ridges",
    (m, c) => shore(m, "shore_shingle", c.pal, c.seed, c.index, c.d), { tile: TILE, tiles: true, radius: 9 });
  register("shore_shelf", "dressing", "mediterranean", "rock shelf shore", "jointed bedrock stepping down to tide pools",
    (m, c) => shore(m, "shore_shelf", c.pal, c.seed, c.index, c.d), { tile: TILE, tiles: true, radius: 9 });
  register("river_bank", "dressing", "temperate_broadleaf", "river bank", "gravel bar, undercut cut bank, roots and reeds",
    (m, c) => riverBank(m, c.pal, c.seed, c.index, c.d), { tile: TILE, tiles: true, radius: 5 });
  register("hedgerow", "dressing", "temperate_broadleaf", "hedgerow", "hedge on a bank with a ditch beside it",
    (m, c) => hedgerow(m, c.pal, c.seed, c.index, c.d, c.o), { tile: TILE, tiles: true, radius: 3 });
  register("field_patch", "dressing", "temperate_broadleaf", "field patch", "worked ground with plough lines and a hedged boundary",
    (m, c) => fieldPatch(m, c.pal, c.seed, c.d, c.o), { radius: 30 });

  const FALLBACK = "boulder";
  const TILEABLE = Object.keys(KINDS).filter((k) => KINDS[k].tiles);
  const SCATTERABLE = Object.keys(KINDS).filter((k) => KINDS[k].family !== "dressing");

  function has(table, key) { return typeof key === "string" && Object.prototype.hasOwnProperty.call(table, key); }
  /// One LOD vocabulary across the art modules: site-mesh.js says "far",
  /// town-mesh.js says "map", and both mean the coarse level. Accept either
  /// here too, or a caller asking for the cheap mesh silently gets the dear one.
  function isFar(v) {
    return v === 1 || v === 2 || v === "far" || v === "map" || v === "lod1" || v === "lod2";
  }
  function intOf(v, dflt) {
    return typeof v === "number" && Number.isFinite(v) ? Math.round(v) : (dflt || 0);
  }

  // ---------------------------------------------------------------- descriptions
  /// Every description says what the piece IS and what it is NOT, because a
  /// scatter kit is exactly the sort of art that gets mistaken for data. A
  /// hundred trees on a province is a hundred trees of art.
  function describe(spec, biome, lod, tris) {
    const b = BIOMES[biome];
    return `${spec.label} (${spec.blurb}), ${b.label} palette, `
      + `${lod ? "map" : "near"} detail, ${tris} triangles. `
      + `Representative scenery and original game art: a generic ${spec.family} built from a seeded hash, `
      + `not a survey, not a species record and not a land-cover measurement. `
      + `It grants no forest cover, forage, timber, mineral or farmland of any kind.`;
  }

  /// Build one piece. `kind` picks the geometry, `biome` picks the palette (a
  /// boulder in the tundra is not the boulder in the desert), `seed` picks the
  /// specimen, `lod` picks near or map, and `index` says which tile of a run
  /// this is for the six kinds that tile.
  function piece(kind, opts) {
    const o = opts && typeof opts === "object" ? opts : {};
    const key = has(KINDS, kind) ? kind : FALLBACK;
    const spec = KINDS[key];
    const lod = isFar(o.lod) ? 1 : 0;
    const biome = has(BIOMES, o.biome) ? o.biome : spec.biome;
    const seed = seedOf(o.seed == null ? (o.id == null ? 0 : o.id) : o.seed);
    const index = Math.abs(intOf(o.index, 0)) % 100000;
    const m = new Mesh();
    const detail = spec.draw(m, { pal: BIOMES[biome].pal, seed, index, d: lod ? 0 : 1, lod, o });
    const tris = m.pos.length / 9;
    return m.finish({
      kind: key,
      requestedKind: kind,
      family: spec.family,
      label: spec.label,
      biome,
      lod,
      seed,
      index: spec.tiles ? index : null,
      tile: spec.tile || null,
      budget: BUDGET[spec.family][lod ? "far" : "near"].slice(),
      detail: detail && typeof detail === "object" ? detail : null,
      description: describe(spec, biome, lod, tris),
    });
  }

  // ------------------------------------------------------------------- cost
  // A kind's triangle count is FIXED per LOD: only sizes, angles and colours
  // vary with the seed, never the structure. That is a deliberate constraint
  // and it is what makes a budget a number instead of an average — a thousand
  // spruces cost exactly a thousand times one spruce, and the check asserts the
  // count does not move across seeds.
  const COST = new Map();
  /// The count at the canonical seed. EXACT for every scatterable kind (see
  /// `fixedCost`); representative for a dressing tile, whose crop and tile index
  /// legitimately change how much is on it.
  function cost(kind, lod) {
    const key = (has(KINDS, kind) ? kind : FALLBACK) + "/" + (isFar(lod) ? 1 : 0);
    if (!COST.has(key)) {
      const [k, l] = key.split("/");
      COST.set(key, piece(k, { lod: Number(l), seed: 1 }).triangleCount);
    }
    return COST.get(key);
  }
  function median(list) {
    const s = list.slice().sort((a, b) => a - b);
    const mid = s.length >> 1;
    return s.length % 2 ? s[mid] : Math.round((s[mid - 1] + s[mid]) / 2);
  }
  /// What a stand of `n` trees actually costs, measured rather than estimated.
  /// The far number is the one that decides whether a map can afford a forest;
  /// section 4's own note says the same thing about distant content.
  function treeCost(n) {
    const trees = Object.keys(KINDS).filter((k) => KINDS[k].family === "tree");
    const near = trees.map((k) => cost(k, 0)), far = trees.map((k) => cost(k, 1));
    const count = Math.max(0, intOf(n, 1000));
    return {
      instances: count,
      kinds: trees.length,
      nearMedian: median(near), farMedian: median(far),
      nearRange: [Math.min.apply(null, near), Math.max.apply(null, near)],
      farRange: [Math.min.apply(null, far), Math.max.apply(null, far)],
      nearTotal: median(near) * count,
      farTotal: median(far) * count,
      note: "Triangles only. Draw calls, instancing and culling are the caller's, "
        + "and this is not a frame-time promise.",
    };
  }

  // ---------------------------------------------------------------- scatter
  // WHERE THE INSTANCES GO. A stratified grid with hashed jitter inside each
  // cell, which is a Latin-square-ish placement: it fills the area evenly at
  // any count without the clumping a plain hashed position gives, and it costs
  // one pass. Kind comes from the biome's mix table, which is what actually
  // reads as climate — savanna is mostly dry grass with acacias standing well
  // apart, boreal is spruce shoulder to shoulder.
  //
  // EXCLUSIONS are the caller's data, not this file's opinion: roadmap section
  // G says terrain-following placement excludes water, excessive slope, labels
  // and transport corridors, and all four of those live outside this module.
  // Pass circles and rectangles in `exclude`, a `ground(x, z)` sampler to sit
  // instances on terrain, and a `slopeLimit` to refuse steep ground. What comes
  // back reports what it refused and why rather than silently thinning out.
  function pickKind(seedv, salt, mix, total) {
    let roll = (ask(seedv, salt) % total);
    for (let i = 0; i < mix.length; i += 1) {
      roll -= mix[i][1];
      if (roll < 0) return mix[i][0];
    }
    return mix[mix.length - 1][0];
  }
  function areaOf(area) {
    const a = area && typeof area === "object" ? area : {};
    const w = Math.abs(intOf(a.width != null ? a.width : (Array.isArray(area) ? area[0] : 100), 100)) || 100;
    const d = Math.abs(intOf(a.depth != null ? a.depth : (Array.isArray(area) ? area[1] : w), w)) || w;
    return { x0: intOf(a.x0, 0), z0: intOf(a.z0, 0), width: w, depth: d };
  }
  function blocked(x, z, list) {
    if (!Array.isArray(list)) return false;
    for (let i = 0; i < list.length; i += 1) {
      const e = list[i];
      if (!e) continue;
      if (e.r != null) {
        const dx = x - e.x, dz = z - e.z;
        if (dx * dx + dz * dz <= e.r * e.r) return true;
      } else if (e.x0 != null) {
        if (x >= e.x0 && x <= e.x1 && z >= e.z0 && z <= e.z1) return true;
      }
    }
    return false;
  }

  function scatter(seed, biome, count, area, opts) {
    const o = opts && typeof opts === "object" ? opts : {};
    const key = has(BIOMES, biome) ? biome : "temperate_broadleaf";
    const b = BIOMES[key];
    const base = seedOf(seed);
    const want = Math.max(0, Math.min(100000, intOf(count, 0)));
    const box = areaOf(area);
    const lod = isFar(o.lod) ? 1 : 0;
    const mix = Array.isArray(o.mix) && o.mix.length ? o.mix : b.mix;
    let total = 0;
    for (let i = 0; i < mix.length; i += 1) total += mix[i][1];
    const cols = Math.max(1, Math.round(Math.sqrt(Math.max(1, want) * (box.width / box.depth))));
    const rows = Math.max(1, Math.ceil(want / cols));
    const cw = box.width / cols, cd = box.depth / rows;
    const ground = typeof o.ground === "function" ? o.ground : null;
    const slopeLimit = typeof o.slopeLimit === "number" && Number.isFinite(o.slopeLimit) ? o.slopeLimit : null;
    const spacing = o.spacing === false ? false : true;
    const instances = [];
    const histogram = {};
    const refused = { excluded: 0, slope: 0, spacing: 0 };
    const placedGrid = new Map();
    for (let i = 0; i < want; i += 1) {
      const cx = i % cols, cz = Math.floor(i / cols);
      const rolled = pickKind(base, i * 4 + 1, mix, total);
      const kind = has(KINDS, rolled) ? rolled : FALLBACK;
      const spec = KINDS[kind];
      const x = box.x0 + (cx + 0.5 + askSigned(base, i * 4 + 2, 0.44)) * cw;
      const z = box.z0 + (cz + 0.5 + askSigned(base, i * 4 + 3, 0.44)) * cd;
      if (blocked(x, z, o.exclude)) { refused.excluded += 1; continue; }
      let y = 0;
      if (ground) {
        y = ground(x, z);
        if (!Number.isFinite(y)) { refused.excluded += 1; continue; }
        if (slopeLimit != null) {
          const h = 0.5;
          const gx = (ground(x + h, z) - ground(x - h, z)) / (2 * h);
          const gz = (ground(x, z + h) - ground(x, z - h)) / (2 * h);
          if (!(Math.hypot(gx, gz) <= slopeLimit)) { refused.slope += 1; continue; }
        }
      }
      const sc = askSpan(base, i * 4 + 4, 0.72, 1.34);
      if (spacing) {
        const rad = (spec.radius || 1) * sc * 0.45;
        const gk = Math.floor(x / 4) + ":" + Math.floor(z / 4);
        let clash = false;
        for (let dx = -1; dx <= 1 && !clash; dx += 1) {
          for (let dz = -1; dz <= 1 && !clash; dz += 1) {
            const near = placedGrid.get((Math.floor(x / 4) + dx) + ":" + (Math.floor(z / 4) + dz));
            if (!near) continue;
            for (let n = 0; n < near.length; n += 1) {
              const q = near[n];
              if (Math.hypot(q.x - x, q.z - z) < (rad + q.rad) * 0.75) { clash = true; break; }
            }
          }
        }
        if (clash) { refused.spacing += 1; continue; }
        const bucket = placedGrid.get(gk);
        if (bucket) bucket.push({ x, z, rad }); else placedGrid.set(gk, [{ x, z, rad }]);
      }
      instances.push({
        kind, family: spec.family, x, y, z,
        yaw: askUnit(base, i * 4 + 5) * Math.PI * 2,
        scale: sc,
        seed: mix32(base ^ Math.imul(i + 1, 0x27d4eb2f)),
        cell: i, lod,
      });
      histogram[kind] = (histogram[kind] || 0) + 1;
    }
    let near = 0, far = 0;
    for (const kind of Object.keys(histogram)) {
      near += cost(kind, 0) * histogram[kind];
      far += cost(kind, 1) * histogram[kind];
    }
    const hectares = (box.width * box.depth) / 10000;
    const parts = Object.keys(histogram).sort().map((k) => `${histogram[k]} ${KINDS[k].label}`);
    return {
      seed: base, requestedSeed: seed, biome: key, lod,
      area: box, requested: want, placed: instances.length,
      grid: { cols, rows, cellWidth: cw, cellDepth: cd },
      instances, histogram, refused,
      perHectare: hectares > 0 ? Math.round((instances.length / hectares) * 10) / 10 : 0,
      triangles: { near, far },
      description: `Seeded scatter of ${instances.length} instances over `
        + `${box.width} x ${box.depth} m of ${b.label} (${b.note}): ${parts.join(", ")}. `
        + `A placement plan, not a survey: it describes no real vegetation, and the terrain, `
        + `water, slope and corridor exclusions belong to the caller.`,
    };
  }

  /// The same plan, built. Pieces are baked once per (kind, variant) and stamped
  /// with a yaw and a scale, which is the whole of the reuse: a thousand trees
  /// cost eight variants of geometry and a pass of arithmetic. Parts are grouped
  /// BY KIND rather than per instance, because a scatter field is drawn, not
  /// picked, and a thousand named ranges would be a thousand nobody reads.
  function scatterMesh(seed, biome, count, area, opts) {
    const o = opts && typeof opts === "object" ? opts : {};
    const plan = scatter(seed, biome, count, area, o);
    const variants = Math.max(1, Math.min(24, intOf(o.variants, 6)));
    const cache = new Map();
    const bakedOf = (kind, sd) => {
      const slot = kind + "/" + (mix32(sd) % variants);
      if (!cache.has(slot)) {
        cache.set(slot, piece(kind, { biome: plan.biome, lod: plan.lod, seed: mix32(sd) % variants + 1 }));
      }
      return cache.get(slot);
    };
    const m = new Mesh();
    const byKind = new Map();
    for (const inst of plan.instances) {
      const list = byKind.get(inst.kind);
      if (list) list.push(inst); else byKind.set(inst.kind, [inst]);
    }
    for (const kind of Array.from(byKind.keys()).sort()) {
      const list = byKind.get(kind);
      m.part(KINDS[kind].label + " / " + list.length + " instances", () => {
        for (const inst of list) {
          const baked = bakedOf(kind, inst.seed);
          m.save().move(inst.x, inst.y, inst.z).rotY(inst.yaw).scale(inst.scale);
          // Stamping a baked buffer rather than re-running the generator: the
          // positions are already final, so this is one transform per vertex.
          const p = baked.positions;
          for (let i = 0; i < p.length; i += 9) {
            m.tri([p[i], p[i + 1], p[i + 2]], [p[i + 3], p[i + 4], p[i + 5]], [p[i + 6], p[i + 7], p[i + 8]],
              [baked.colors[i], baked.colors[i + 1], baked.colors[i + 2]]);
          }
          m.restore();
        }
      });
    }
    // The stamped colours already carry the sun from the baked piece, so the
    // second pass in `finish` would light them twice. Undo the first bake by
    // dividing it back out at emit is not worth the arithmetic; instead the
    // field is flat-shaded on purpose — every instance keeps the shading it was
    // baked with and no smoothing group crosses two plants.
    const out = m.finish({
      kind: "scatter_field", family: "field", biome: plan.biome, lod: plan.lod,
      plan: { seed: plan.seed, placed: plan.placed, requested: plan.requested, histogram: plan.histogram },
      instances: plan.instances.length,
      description: plan.description,
    });
    return out;
  }

  // -------------------------------------------------------------------- api
  function kinds() { return Object.keys(KINDS); }
  function kindInfo(kind) {
    if (!has(KINDS, kind)) return null;
    const s = KINDS[kind];
    return {
      kind: s.kind, family: s.family, label: s.label, blurb: s.blurb,
      biome: s.biome, tiles: !!s.tiles, tile: s.tile || null, radius: s.radius,
      // Whether the triangle count is a CONSTANT for this kind at a given LOD.
      // It is, for everything that gets scattered: only sizes, angles and
      // colours move with the seed, so a thousand of them cost a thousand times
      // one. It is not for a field patch, whose crop variant changes how many
      // plough ridges there are, or for a hedgerow, which carries a hedgerow
      // tree on some tile indices and not others. Those are placed in tens and
      // their count is bounded by the budget rather than fixed.
      fixedCost: s.family !== "dressing",
      budget: { near: BUDGET[s.family].near.slice(), far: BUDGET[s.family].far.slice() },
    };
  }
  function biomes() { return Object.keys(BIOMES); }
  function biomeInfo(biome) {
    if (!has(BIOMES, biome)) return null;
    const b = BIOMES[biome];
    return {
      biome, label: b.label, note: b.note,
      tree: b.tree, shrub: b.shrub, grass: b.grass, rock: b.rock,
      mix: b.mix.map((row) => row.slice()),
    };
  }
  function budgets() {
    const out = {};
    for (const family of Object.keys(BUDGET)) {
      out[family] = { near: BUDGET[family].near.slice(), far: BUDGET[family].far.slice() };
    }
    return out;
  }

  return {
    kinds, kindInfo, biomes, biomeInfo, budgets, cost, treeCost,
    piece, scatter, scatterMesh,
    families: ["tree", "shrub", "grass", "rock", "dressing"],
    tileable: TILEABLE.slice(),
    scatterable: SCATTERABLE.slice(),
    tileWidth: TILE,
    crops: Object.keys(CROPS),
  };
});
