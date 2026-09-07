/* The transport network kit: road, divided highway and railway as a SEGMENT
   KIT with mating endpoints, plus the layout function that drives a run of
   segments along a polyline. Roadmap section F, "Networks".

   WHY A KIT AND NOT A MODEL. Section F is explicit: use spline-driven segment
   kits with compatible endpoints rather than individually modelling every road.
   A road is the asset class where that matters most, because a road is the same
   25 metres several thousand times over. So nothing here models a named route.
   Thirteen pieces are authored; a network is those pieces laid end to end by
   `assemble`, and the only thing that makes two pieces line up is the endpoint
   contract below.

   THE ENDPOINT CONTRACT is the load-bearing part of this file, so it is stated
   before the geometry. Every piece declares its connection points as ports:

     { id, network, position:[x,y,z], forward:[x,y,z], up:[0,1,0], width, gauge }

   `forward` is the OUTWARD normal of the port face — it points out of the
   piece, along the direction a neighbour arrives from. Two ports mate when
   their world positions coincide, their forwards are exact opposites, their
   networks match and their widths and gauges agree. `mate` returns the
   placement frame that makes that true, and `assemble` uses nothing else to
   place a segment. That is why a run has no seam gaps: the seam is not measured
   after the fact, it is the thing the placement is derived from.

   PLACEMENT IS MULTIPLY-AND-ADD, NEVER TRIGONOMETRY. A placement is a yaw and a
   translation carried as {c, s, x, y, z} — the cosine and sine of the yaw kept
   as a pair and never converted back to an angle. Composing two placements is
   one complex multiplication; mating a port to a port solves a 2x2 system whose
   matrix is its own inverse. So a chain of two hundred segments accumulates
   only the rounding of multiplication and addition, never the rounding of atan2
   followed by cos and sin. Arc authoring inside a piece does use Math.cos and
   Math.sin, which is the one place a segment's own shape is trigonometric.

   DETERMINISM. No clock and no entropy source of any kind, which is iron rule
   one and gets no art exemption. Variation — which side a marker post stands
   on, whether a segment carries a gully — is a 32-bit hash of explicit inputs
   (the piece name and the caller's `index`), so the same call gives the same
   bytes in a fresh process forever. `tools/ui/check_road_mesh.cjs` asserts that
   in one process and across a fresh load, and greps this source for the
   vocabulary that would break it.

   MODEL SPACE. Metres. +X right, +Y up, +Z forward. Y=0 is ground contact,
   exactly: every piece is authored so its lowest vertex is natural ground, and
   the checks assert the correction `finish` would apply is zero, which is how a
   mis-authored piece gets caught rather than silently shifted.

   THE ROOT IS THE ENTRY PORT, NOT THE FOOTPRINT CENTRE, and that is a departure
   from the shared production contract worth stating rather than hiding. A
   segment kit is placed by its seam. Putting the origin anywhere else means
   every placement carries a half-length offset that has to agree with the port
   table, and the first time those two disagree the run develops a gap. So
   `ports.in.position` is [0, base, 0] on every piece and the body runs to +Z.

   PER-VERTEX COLOUR, NO TEXTURES, NO UVS. Lane markings, kerb faces, ballast,
   rail heads and grass verges are geometry with a colour on it, because that is
   the whole of what this renderer has. A dashed centre line is four small quads
   sitting 8 mm proud of the carriageway, not a decal.

   WHAT THIS IS NOT. Representative scenery. No piece is a survey of a real
   road, no dimension is a measurement of a specific route, and laying a run
   between two places asserts nothing about whether a road exists there, what it
   carries, or what it cost. The cross-sections are ordinary 1990 practice —
   3.5 m lanes, 1.435 m track gauge, a 4 m central reserve — chosen because they
   are unremarkable. Nothing here grants a province capacity, and nothing here
   is wired into the game: this module builds buffers and stops. */
(function (root, factory) {
  const api = factory();
  if (typeof module === "object" && module.exports) module.exports = api;
  else root.RoadMesh = api;
})(typeof globalThis !== "undefined" ? globalThis : this, function () {
  "use strict";

  // ------------------------------------------------------------- dimensions
  // Ordinary 1990 practice, in metres. None of these is a measured figure from
  // a real route; they are the plain numbers that make a cross-section read as
  // what it is.
  const SEG = 25;              // nominal segment length: 40 to the kilometre
  const LANE = 3.5;            // two-lane carriageway lane
  const ROAD_HALF = 6.0;       // two-lane corridor half width, verge to verge
  const HW_HALF = 13.5;        // dual two-lane corridor half width
  const RAIL_HALF = 3.1;       // single-track ballast shoulder half width
  const GAUGE = 1.435;         // standard gauge
  const SLEEPER_PITCH = 0.65;
  const ROAD_RADIUS = 20;      // the road bend piece's own radius
  const HW_RADIUS = 120;
  const RAIL_RADIUS = 400;
  const DECK_RISE = 5.0;       // road bridge deck above the bed it crosses
  const RAIL_DECK_RISE = 4.6;
  const MARK = 0.008;          // how far a painted marking stands proud
  const SKIRT = 0.05;          // minimum width of the fill skirt band, so a
                               // piece at grade still emits it and every
                               // cross-section keeps the same point count

  const DEG = Math.PI / 180;

  // ------------------------------------------------------------------- hash
  // The only source of variation here. A piece asks a question by salt and gets
  // the same answer on every machine and every reload: murmur3's finaliser with
  // an FNV stir in front of it. Not cryptographic, does not need to be, and it
  // decorrelates small integers, which is the whole job.
  function mix32(x) {
    let h = x | 0;
    h = Math.imul(h ^ (h >>> 16), 0x7feb352d);
    h = Math.imul(h ^ (h >>> 15), 0x846ca68b);
    h ^= h >>> 16;
    return h >>> 0;
  }
  function seedOf(text, index) {
    let h = 0x811c9dc5;
    const s = String(text == null ? "" : text);
    for (let i = 0; i < s.length; i += 1) h = Math.imul(h ^ s.charCodeAt(i), 0x01000193);
    return mix32(h ^ Math.imul((index | 0) + 1, 0x9e3779b1));
  }
  function ask(seed, salt) { return mix32((seed ^ Math.imul((salt | 0) + 1, 0x27d4eb2f)) | 0); }

  // ----------------------------------------------------------------- colour
  function rgb(hex) {
    return [((hex >> 16) & 255) / 255, ((hex >> 8) & 255) / 255, (hex & 255) / 255];
  }

  /// Deliberately drab. A transport corridor is bitumen, ballast, galvanised
  /// steel and grass; the only saturated things on one are the paint and the
  /// signs, and both are saturated because they have to be seen.
  const P = {
    asphalt: rgb(0x3e4044),
    asphaltWorn: rgb(0x4a4c50),
    kerb: rgb(0xb0aca2),
    concrete: rgb(0xa8a49a),
    paint: rgb(0xe4e0d4),
    paintYellow: rgb(0xcaa63a),
    grass: rgb(0x4e5c3a),
    earth: rgb(0x6b5c46),
    ballast: rgb(0x7a746a),
    sleeper: rgb(0x6c6459),
    railHead: rgb(0xb6b9bb),
    railWeb: rgb(0x6a5647),
    steel: rgb(0x808890),
    galv: rgb(0xa2a9ae),
    signFace: rgb(0xd8dbdd),
    post: rgb(0x9aa0a4),
    dark: rgb(0x1b1d1f),
    water: rgb(0x33505e),
    timber: rgb(0x6f5a3e),
  };

  /// One sun, fixed, high and to the right, baked into the vertex colour
  /// because the path this feeds is the flat vertex-colour path the rest of the
  /// browser art already uses. Separating albedo from light properly is a
  /// renderer milestone and not this file's.
  const SUN = (function () {
    const v = [0.40, 0.82, 0.41], n = Math.hypot(v[0], v[1], v[2]);
    return [v[0] / n, v[1] / n, v[2] / n];
  })();
  const AMBIENT = 0.54, DIFFUSE = 0.50;

  /// How far two faces may disagree and still share a corner normal. cos 69.5°.
  /// A rail head is curved and its foot is not; a lamp column is smooth and its
  /// base flange is not. At 0.35 the ring of an 8-segment tube merges (adjacent
  /// faces 45° apart, dot 0.707) and its end cap does not (dot 0), which is the
  /// split wanted. It also bounds the shading contract the checks assert: every
  /// face folded into a corner is within the limit of that corner's own face,
  /// so the normalised sum still has dot >= 0.35 with it, and a smoothed vertex
  /// can never point behind the triangle it belongs to.
  const CREASE = 0.35;

  // ------------------------------------------------------------------ frame
  // A placement is a yaw and a translation. The yaw is carried as its cosine
  // and sine so that composing, mating and inverting are multiplication and
  // addition only — see the header. `apply` maps a local point into the frame;
  // `direction` maps a local direction, which is the same map without the
  // translation.
  const IDENT = { c: 1, s: 0, x: 0, y: 0, z: 0 };

  function apply(f, p) {
    return [f.c * p[0] + f.s * p[2] + f.x, p[1] + f.y, -f.s * p[0] + f.c * p[2] + f.z];
  }
  function direction(f, d) {
    return [f.c * d[0] + f.s * d[2], d[1], -f.s * d[0] + f.c * d[2]];
  }
  /// F after G: the frame that maps p to F(G(p)).
  function compose(f, g) {
    return {
      c: f.c * g.c - f.s * g.s,
      s: f.c * g.s + f.s * g.c,
      x: f.c * g.x + f.s * g.z + f.x,
      y: g.y + f.y,
      z: -f.s * g.x + f.c * g.z + f.z,
    };
  }
  /// The yaw that takes local direction `q` to world direction `w`, both unit
  /// and both in the X-Z plane. The 2x2 system here is a reflection, so it is
  /// its own inverse and the solution is two dot products — no angle is ever
  /// formed, which is what keeps a long chain from drifting.
  function yawTaking(q, w) {
    return { c: q[0] * w[0] + q[2] * w[2], s: q[2] * w[0] - q[0] * w[2] };
  }

  // ------------------------------------------------------------------- mesh
  // Triangle soup with a per-vertex albedo, a yaw-and-translate stack, named
  // part ranges, and a smoothing group per emitted surface. Normals are NOT
  // accumulated at emit time; they are derived in `finish` from the vertices as
  // transformed, then folded together inside a smoothing group under CREASE.
  // Flat things never open a group and keep their exact face normal.
  function Mesh() {
    this.pos = [];
    this.col = [];
    this.grp = [];
    this.parts = [];
    this.f = IDENT;
    this.stack = [];
    this.sg = 0;
    this.sgNext = 0;
    this.prefix = "";
  }
  Mesh.prototype.push = function (c, s, x, y, z) {
    this.stack.push(this.f);
    this.f = compose(this.f, { c, s, x: x || 0, y: y || 0, z: z || 0 });
    return this;
  };
  Mesh.prototype.pushFrame = function (g) {
    this.stack.push(this.f);
    this.f = compose(this.f, g);
    return this;
  };
  Mesh.prototype.pop = function () { this.f = this.stack.pop() || IDENT; return this; };
  Mesh.prototype.xf = function (p) { return apply(this.f, p); };

  /// Everything emitted inside `fn` shares one smoothing group, and a FRESH id
  /// per call deliberately: two rails lying side by side touch nothing, but two
  /// lamp columns standing shoulder to shoulder would have their normals welded
  /// into one lumpy surface if they shared a group.
  Mesh.prototype.smooth = function (fn) {
    const prev = this.sg;
    this.sgNext += 1;
    this.sg = this.sgNext;
    fn(this);
    this.sg = prev;
    return this;
  };

  /// A degenerate triangle is DROPPED rather than emitted with a made-up
  /// normal. This is not tidiness: the junction cross-section is authored by
  /// collapsing the kerb and verge points onto the corridor edge, so the mouth
  /// of a junction emits a dozen zero-area bands by design and they have to go
  /// somewhere. Letting one through costs either a NaN in an attribute buffer
  /// or a black facet nobody can explain three months later.
  Mesh.prototype.tri = function (a, b, c, col) {
    const A = this.xf(a), B = this.xf(b), C = this.xf(c);
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
  /// the pickers already speak it. A part that draws nothing is not recorded —
  /// an empty range would break the contiguity the checks assert, and a map-LOD
  /// segment that has no gully should not carry a gully entry.
  Mesh.prototype.part = function (name, fn) {
    const first = this.pos.length / 3;
    fn(this);
    const count = this.pos.length / 3 - first;
    if (count > 0) {
      const full = this.prefix + name;
      const cut = full.indexOf(" / ");
      this.parts.push({
        name: full,
        first,
        count,
        group: cut < 0 ? full : full.slice(0, cut),
        label: cut < 0 ? full : full.slice(cut + 3),
      });
    }
    return this;
  };

  // ------------------------------------------------------------- primitives
  /// A box by extents, not by centre and size: almost everything beside a road
  /// is positioned by the face it sits flush against — a formation level, a
  /// kerb line, a deck soffit — and extents say that without arithmetic at the
  /// call site.
  Mesh.prototype.bar = function (x0, x1, y0, y1, z0, z1, col) {
    this.quad([x1, y0, z0], [x1, y1, z0], [x1, y1, z1], [x1, y0, z1], col);
    this.quad([x0, y0, z0], [x0, y0, z1], [x0, y1, z1], [x0, y1, z0], col);
    this.quad([x0, y1, z0], [x0, y1, z1], [x1, y1, z1], [x1, y1, z0], col);
    this.quad([x0, y0, z0], [x1, y0, z0], [x1, y0, z1], [x0, y0, z1], col);
    this.quad([x0, y0, z1], [x1, y0, z1], [x1, y1, z1], [x0, y1, z1], col);
    this.quad([x0, y0, z0], [x0, y1, z0], [x1, y1, z0], [x1, y0, z0], col);
    return this;
  };
  /// Five faces, no underside. A concrete sleeper is half buried in ballast and
  /// there are forty of them in a segment; the face nobody can see is 20% of
  /// the cost of the one thing this kit emits most of.
  Mesh.prototype.buried = function (x0, x1, y0, y1, z0, z1, col) {
    this.quad([x1, y0, z0], [x1, y1, z0], [x1, y1, z1], [x1, y0, z1], col);
    this.quad([x0, y0, z0], [x0, y0, z1], [x0, y1, z1], [x0, y1, z0], col);
    this.quad([x0, y1, z0], [x0, y1, z1], [x1, y1, z1], [x1, y1, z0], col);
    this.quad([x0, y0, z1], [x1, y0, z1], [x1, y1, z1], [x0, y1, z1], col);
    this.quad([x0, y0, z0], [x0, y1, z0], [x1, y1, z0], [x1, y0, z0], col);
    return this;
  };

  /// A CHAMFERED box: the same extents as `bar` with the four edges parallel to
  /// its longest axis cut back by `ch`. Worth its eight extra triangles for the
  /// same reason site-mesh.js pays for them — nothing manufactured has a knife
  /// edge, and in a renderer with no textures the arris catching the light is
  /// most of what "manufactured" even means. A crash barrier rail, a sign post
  /// and a marker post all get one; the map path never calls this.
  Mesh.prototype.beam = function (x0, x1, y0, y1, z0, z1, col, ch0) {
    const dx = x1 - x0, dy = y1 - y0, dz = z1 - z0;
    const axis = dx >= dy && dx >= dz ? 0 : (dy >= dz ? 1 : 2);
    const u = axis === 0 ? dy : dx, v = axis === 2 ? dy : dz;
    const ch = Math.max(0.003, Math.min(ch0 == null ? 0.02 : ch0, u * 0.34, v * 0.34));
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
        at(sec[j][0], sec[j][1], e1), at(sec[i][0], sec[i][1], e1), col);
    }
    this.fan(sec.map((p) => at(p[0], p[1], e1)), col);
    this.fan(sec.map((p) => at(p[0], p[1], e0)).reverse(), col);
    return this;
  };

  /// A vertical cylinder — lamp columns, catenary masts, bridge piers, gate
  /// posts. Smoothed as one group, so the crease limit merges the barrel and
  /// keeps the end caps sharp.
  Mesh.prototype.column = function (x, z, y0, y1, r0, r1, seg, col) {
    const n = Math.max(3, seg | 0);
    const ring = (r, y) => {
      const pts = [];
      for (let i = 0; i < n; i += 1) {
        const t = (i / n) * Math.PI * 2;
        pts.push([x + Math.cos(t) * r, y, z + Math.sin(t) * r]);
      }
      return pts;
    };
    const lo = ring(r0, y0), up = ring(r1 == null ? r0 : r1, y1);
    this.smooth((m) => {
      for (let i = 0; i < n; i += 1) {
        const j = (i + 1) % n;
        m.quad(lo[i], lo[j], up[j], up[i], col);
      }
      m.fan(up, col);
      m.fan(lo.slice().reverse(), col);
    });
    return this;
  };

  /// A flat quad on a plane of constant Y — a lane marking, a crossing panel, a
  /// bed slab. Two triangles, because a marking has no thickness worth paying
  /// for; it is lifted MARK above the surface it sits on instead.
  Mesh.prototype.patch = function (x0, x1, z0, z1, y, col) {
    return this.quad([x0, y, z0], [x1, y, z0], [x1, y, z1], [x0, y, z1], col);
  };

  // ------------------------------------------------------------------ sweep
  // The kit's workhorse. A station is a point on the alignment plus the unit
  // heading there; a cross-section is a list of {x, y, col} in the plane normal
  // to that heading, where `col` belongs to the BAND running from that point to
  // the next one outward-to-inward. `profileAt` is a function of the station
  // rather than a constant list, because the fill skirt widens with the height
  // of the formation, the deck of a bridge morphs, and a junction mouth is a
  // cross-section with its kerb collapsed onto the corridor edge.
  //
  // The right-hand vector is (hz, -hx), so a profile written left to right
  // faces up: h x r = +Y, which is what fixes the winding once for every
  // surface in the file rather than at each call.
  Mesh.prototype.sweep = function (stations, profileAt, closed) {
    const secs = [];
    for (let j = 0; j < stations.length; j += 1) {
      const st = stations[j];
      const prof = profileAt(st, j);
      const rx = st.hz, rz = -st.hx;
      const row = [];
      for (let i = 0; i < prof.length; i += 1) {
        row.push({
          p: [st.x + prof[i].x * rx, st.y + prof[i].y, st.z + prof[i].x * rz],
          col: prof[i].col,
        });
      }
      secs.push(row);
    }
    const n = secs[0].length;
    const bands = closed === false ? n - 1 : n;
    this.smooth((m) => {
      for (let j = 0; j + 1 < secs.length; j += 1) {
        for (let i = 0; i < bands; i += 1) {
          const k = (i + 1) % n;
          m.quad(secs[j][i].p, secs[j + 1][i].p, secs[j + 1][k].p, secs[j][k].p, secs[j][i].col);
        }
      }
    });
    return this;
  };

  // --------------------------------------------------------------- stations
  /// A straight run of `len` metres, rising `rise` metres from `base`.
  function lineStations(len, steps, base, rise) {
    const out = [];
    for (let j = 0; j <= steps; j += 1) {
      const t = j / steps;
      out.push({ x: 0, y: base + rise * t, z: len * t, hx: 0, hz: 1, t, s: len * t });
    }
    return out;
  }
  /// A circular arc of `defl` degrees at radius `r`, turning right for hand +1
  /// and left for hand -1, flat at `base`. This is the one place a piece's own
  /// shape is trigonometric; the placement of the piece never is.
  function arcStations(r, defl, hand, steps, base) {
    const out = [], total = defl * DEG, h = hand < 0 ? -1 : 1;
    for (let j = 0; j <= steps; j += 1) {
      const t = j / steps, a = total * t;
      const ca = Math.cos(a), sa = Math.sin(a);
      out.push({
        x: h * (r - r * ca), y: base, z: r * sa,
        hx: h * sa, hz: ca, t, s: r * total * t,
      });
    }
    return out;
  }
  /// The exit port of an arc, derived from the same two calls the last station
  /// uses so the port table and the geometry cannot disagree.
  function arcExit(r, defl, hand, base) {
    const a = defl * DEG, h = hand < 0 ? -1 : 1;
    const ca = Math.cos(a), sa = Math.sin(a);
    return { position: [h * (r - r * ca), base, r * sa], forward: [h * sa, 0, ca] };
  }

  // ----------------------------------------------------------------- finish
  Mesh.prototype.finish = function (info) {
    const positions = new Float32Array(this.pos);
    const colors = new Float32Array(this.col);
    const normals = new Float32Array(positions.length);
    const faces = new Float32Array(positions.length / 3);
    const tris = positions.length / 9;
    // Face normals first. The winding is already correct everywhere because
    // `sweep`, `bar` and `beam` fix it at the primitive rather than at the call
    // site, so this is a read and not a repair.
    for (let f = 0; f < tris; f += 1) {
      const i = f * 9;
      const ax = positions[i], ay = positions[i + 1], az = positions[i + 2];
      const ux = positions[i + 3] - ax, uy = positions[i + 4] - ay, uz = positions[i + 5] - az;
      const vx = positions[i + 6] - ax, vy = positions[i + 7] - ay, vz = positions[i + 8] - az;
      let nx = uy * vz - uz * vy, ny = uz * vx - ux * vz, nz = ux * vy - uy * vx;
      const len = Math.hypot(nx, ny, nz);
      if (len > 1e-12) { nx /= len; ny /= len; nz /= len; } else { nx = 0; ny = 1; nz = 0; }
      faces[f * 3] = nx; faces[f * 3 + 1] = ny; faces[f * 3 + 2] = nz;
    }
    // Fold the faces that meet at a corner INSIDE one smoothing group. The key
    // is quantised to a tenth of a millimetre because two primitives that mean
    // to touch are authored from the same numbers and land bitwise equal, while
    // an arc's two sides of a station are the same numbers by construction too.
    const buckets = new Map();
    for (let f = 0; f < tris; f += 1) {
      const g = this.grp[f];
      if (!g) continue;
      for (let k = 0; k < 3; k += 1) {
        const i = f * 9 + k * 3;
        const key = g + "|" + Math.round(positions[i] * 10000) + "|"
          + Math.round(positions[i + 1] * 10000) + "|" + Math.round(positions[i + 2] * 10000);
        const list = buckets.get(key);
        if (list) list.push(f); else buckets.set(key, [f]);
      }
    }
    for (let f = 0; f < tris; f += 1) {
      const g = this.grp[f];
      const fx = faces[f * 3], fy = faces[f * 3 + 1], fz = faces[f * 3 + 2];
      for (let k = 0; k < 3; k += 1) {
        const i = f * 9 + k * 3;
        let nx = fx, ny = fy, nz = fz;
        if (g) {
          const key = g + "|" + Math.round(positions[i] * 10000) + "|"
            + Math.round(positions[i + 1] * 10000) + "|" + Math.round(positions[i + 2] * 10000);
          const list = buckets.get(key);
          nx = 0; ny = 0; nz = 0;
          for (let q = 0; q < list.length; q += 1) {
            const o = list[q] * 3;
            const ox = faces[o], oy = faces[o + 1], oz = faces[o + 2];
            if (ox * fx + oy * fy + oz * fz < CREASE) continue;
            nx += ox; ny += oy; nz += oz;
          }
          const l = Math.hypot(nx, ny, nz);
          if (l > 1e-9) { nx /= l; ny /= l; nz /= l; } else { nx = fx; ny = fy; nz = fz; }
        }
        normals[i] = nx; normals[i + 1] = ny; normals[i + 2] = nz;
        const k2 = AMBIENT + DIFFUSE * Math.max(0, nx * SUN[0] + ny * SUN[1] + nz * SUN[2]);
        for (let c = 0; c < 3; c += 1) {
          const v = colors[i + c] * k2;
          colors[i + c] = v > 1 ? 1 : (v < 0 ? 0 : v);
        }
      }
    }
    const min = [Infinity, Infinity, Infinity], max = [-Infinity, -Infinity, -Infinity];
    for (let i = 0; i < positions.length; i += 3) {
      for (let k = 0; k < 3; k += 1) {
        if (positions[i + k] < min[k]) min[k] = positions[i + k];
        if (positions[i + k] > max[k]) max[k] = positions[i + k];
      }
    }
    // Seat the lowest vertex on Y=0. It should ALREADY be there — every piece
    // authors its fill skirt or its pier feet at natural ground — so the drop
    // is reported rather than swallowed, and the checks assert it is zero. A
    // silent correction here would move the ports relative to the terrain and
    // the first symptom would be a run that floats.
    const drop = min[1];
    const ports = (info.ports || []).map((port) => ({
      id: port.id,
      network: port.network,
      position: [port.position[0], port.position[1] - drop, port.position[2]],
      forward: port.forward.slice(),
      up: [0, 1, 0],
      width: port.width,
      gauge: port.gauge,
    }));
    if (drop !== 0) {
      for (let i = 1; i < positions.length; i += 3) positions[i] -= drop;
      max[1] -= drop;
      min[1] = 0;
    }
    const out = {
      positions, normals, colors,
      bounds: { min, max },
      size: [max[0] - min[0], max[1] - min[1], max[2] - min[2]],
      parts: this.parts,
      triangleCount: tris,
      groundDrop: drop,
    };
    for (const key of Object.keys(info)) out[key] = info[key];
    out.ports = ports;
    return out;
  };

  // ------------------------------------------------------------- profiles
  // A cross-section is written outward-to-inward on the left, across the
  // centre, then inward-to-outward on the right, so the list runs in increasing
  // local x and `sweep` gets the winding it wants. Each point carries the
  // colour of the BAND that starts at it. That is why the same physical kerb
  // reads `kerb` on the way in and `grass` on the way out: the band leaving the
  // kerb's back edge on the right is verge, and the band leaving it on the left
  // is the kerb's own back face.
  //
  // `h` is the height of the formation above natural ground, so the fill skirt
  // is the first and last point of every section and the underside band closes
  // the ring at natural ground. At h = 0 the skirt is SKIRT wide and flat,
  // which keeps the point count identical at grade and on an embankment — a
  // sweep whose two ends disagree about how many points it has is a sweep that
  // cannot be written at all.
  function pt(x, y, col) { return { x, y, col }; }

  /// The two-lane cross-section. `openL`/`openR` in 0..1 morph the kerb and
  /// verge on that side out to the corridor edge and down to carriageway level;
  /// at 1 the side is a junction mouth, at 0 it is an ordinary kerbed edge.
  /// Everything between is the kerb radius, which is why a junction needs no
  /// second mesh and no corner geometry of its own.
  function roadProfile(h, openL, openR) {
    const sk = ROAD_HALF + 2 * h + SKIRT;
    const L = [
      [-sk, -h, P.earth], [-ROAD_HALF, 0, P.grass], [-4.60, 0.14, P.grass],
      [-4.10, 0.10, P.kerb], [-4.10, 0.135, P.kerb], [-3.98, 0.135, P.kerb],
      [-3.90, 0.105, P.kerb], [-3.90, 0.005, P.kerb], [-3.62, 0.005, P.asphalt],
      [-3.50, 0.02, P.asphalt], [-1.75, 0.06, P.asphalt],
    ];
    const R = [
      [1.75, 0.06, P.asphalt], [3.50, 0.02, P.asphalt], [3.62, 0.005, P.kerb],
      [3.90, 0.005, P.kerb], [3.90, 0.105, P.kerb], [3.98, 0.135, P.kerb],
      [4.10, 0.135, P.grass], [4.10, 0.10, P.grass], [4.60, 0.14, P.grass],
      [ROAD_HALF, 0, P.earth], [sk, -h, P.earth],
    ];
    // Where each of those points goes when its side is a junction mouth: the
    // skirt stays put, the carriageway edge runs out to the corridor edge and
    // every kerb and verge point collapses onto it. The collapsed bands have
    // zero area and `tri` drops them.
    const openL2 = [
      [-sk, -h, P.earth], [-ROAD_HALF, 0.02, P.asphalt], [-ROAD_HALF, 0.02, P.asphalt],
      [-ROAD_HALF, 0.02, P.asphalt], [-ROAD_HALF, 0.02, P.asphalt], [-ROAD_HALF, 0.02, P.asphalt],
      [-ROAD_HALF, 0.02, P.asphalt], [-ROAD_HALF, 0.02, P.asphalt], [-ROAD_HALF, 0.02, P.asphalt],
      [-ROAD_HALF, 0.02, P.asphalt], [-1.75, 0.06, P.asphalt],
    ];
    const openR2 = [
      [1.75, 0.06, P.asphalt], [ROAD_HALF, 0.02, P.asphalt], [ROAD_HALF, 0.02, P.asphalt],
      [ROAD_HALF, 0.02, P.asphalt], [ROAD_HALF, 0.02, P.asphalt], [ROAD_HALF, 0.02, P.asphalt],
      [ROAD_HALF, 0.02, P.asphalt], [ROAD_HALF, 0.02, P.asphalt], [ROAD_HALF, 0.02, P.asphalt],
      [ROAD_HALF, 0.02, P.asphalt], [sk, -h, P.earth],
    ];
    const out = [];
    for (let i = 0; i < L.length; i += 1) out.push(morph(L[i], openL2[i], openL));
    out.push(pt(0, 0.09, P.asphalt));
    for (let i = 0; i < R.length; i += 1) out.push(morph(R[i], openR2[i], openR));
    return out;
  }
  function morph(a, b, t) {
    if (!(t > 0)) return pt(a[0], a[1], a[2]);
    if (t >= 1) return pt(b[0], b[1], b[2]);
    return pt(a[0] + (b[0] - a[0]) * t, a[1] + (b[1] - a[1]) * t, t < 0.5 ? a[2] : b[2]);
  }

  /// The same road at map range. Nine points instead of twenty-three: the kerb
  /// upstand is 130 mm and disappears honestly below about a metre per pixel,
  /// so the verge meets the carriageway directly and the crown is the only
  /// shape left. This is authored from the same numbers rather than decimated,
  /// which is why the two cannot drift apart.
  function roadProfileMap(h, openL, openR) {
    const sk = ROAD_HALF + 2 * h + SKIRT;
    const eL = openL > 0.5 ? -ROAD_HALF : -3.50, eR = openR > 0.5 ? ROAD_HALF : 3.50;
    return [
      pt(-sk, -h, P.earth), pt(-ROAD_HALF, 0, P.grass),
      pt(openL > 0.5 ? -ROAD_HALF : -4.05, openL > 0.5 ? 0.02 : 0.12, P.asphalt),
      pt(eL, 0.02, P.asphalt), pt(0, 0.09, P.asphalt), pt(eR, 0.02, P.asphalt),
      pt(openR > 0.5 ? ROAD_HALF : 4.05, openR > 0.5 ? 0.02 : 0.12, P.grass),
      pt(ROAD_HALF, 0, P.earth), pt(sk, -h, P.earth),
    ];
  }

  /// Dual two-lane with a kerbed central reserve, hard shoulders and a fall
  /// away from the reserve on each carriageway. The reserve is grass with a
  /// kerb each side because that is what a 1990 dual carriageway has; the
  /// tensioned steel barrier standing in it is drawn separately so it can be
  /// left off at map range.
  function highwayProfile(h) {
    const sk = HW_HALF + 2 * h + SKIRT;
    return [
      pt(-sk, -h, P.earth), pt(-HW_HALF, 0, P.grass), pt(-12.20, 0.35, P.grass),
      pt(-11.10, 0.22, P.kerb), pt(-11.10, 0.24, P.kerb), pt(-11.00, 0.24, P.kerb),
      pt(-11.00, 0.03, P.asphalt), pt(-10.80, 0.035, P.asphalt), pt(-9.30, 0.09, P.asphalt),
      pt(-5.65, 0.19, P.asphalt), pt(-2.30, 0.27, P.kerb), pt(-2.20, 0.24, P.kerb),
      pt(-2.10, 0.30, P.grass), pt(-1.95, 0.30, P.grass), pt(0, 0.34, P.grass),
      pt(1.95, 0.30, P.grass), pt(2.10, 0.30, P.kerb), pt(2.20, 0.24, P.kerb),
      pt(2.30, 0.27, P.asphalt), pt(5.65, 0.19, P.asphalt), pt(9.30, 0.09, P.asphalt),
      pt(10.80, 0.035, P.asphalt), pt(11.00, 0.03, P.kerb), pt(11.00, 0.24, P.kerb),
      pt(11.10, 0.24, P.grass), pt(11.10, 0.22, P.grass), pt(12.20, 0.35, P.grass),
      pt(HW_HALF, 0, P.earth), pt(sk, -h, P.earth),
    ];
  }
  function highwayProfileMap(h) {
    const sk = HW_HALF + 2 * h + SKIRT;
    return [
      pt(-sk, -h, P.earth), pt(-HW_HALF, 0, P.grass), pt(-11.05, 0.23, P.asphalt),
      pt(-11.00, 0.03, P.asphalt), pt(-2.30, 0.27, P.grass), pt(-2.10, 0.31, P.grass),
      pt(2.10, 0.31, P.asphalt), pt(2.30, 0.27, P.asphalt), pt(11.00, 0.03, P.grass),
      pt(11.05, 0.23, P.grass), pt(HW_HALF, 0, P.earth), pt(sk, -h, P.earth),
    ];
  }

  /// Single-track ballast: a trapezoid with a 1 in 2 shoulder, 600 mm deep
  /// under the sleeper. The sleepers and rails sit on top of this and are drawn
  /// as their own parts, because at map range the ballast is the only part of a
  /// railway that is more than one pixel wide.
  function railProfile(h) {
    const sk = RAIL_HALF + 2 * h + SKIRT;
    return [
      pt(-sk, -h, P.earth), pt(-RAIL_HALF, 0, P.ballast), pt(-2.05, 0.52, P.ballast),
      pt(-1.60, 0.58, P.ballast), pt(1.60, 0.58, P.ballast), pt(2.05, 0.52, P.ballast),
      pt(RAIL_HALF, 0, P.earth), pt(sk, -h, P.earth),
    ];
  }
  function railProfileMap(h) {
    const sk = RAIL_HALF + 2 * h + SKIRT;
    return [
      pt(-sk, -h, P.earth), pt(-RAIL_HALF, 0, P.ballast), pt(-1.60, 0.58, P.ballast),
      pt(1.60, 0.58, P.ballast), pt(RAIL_HALF, 0, P.earth), pt(sk, -h, P.earth),
    ];
  }

  /// A flat-bottom rail section, ten points around foot, web and head, at
  /// `off` metres from the track centre. Smoothing rounds the head and the web
  /// fillets and leaves the foot edges sharp, which is the only reason a rail
  /// reads as steel rather than as a strip of card.
  const RAIL_SECTION = [
    [-0.070, 0.000], [0.070, 0.000], [0.070, 0.022], [0.018, 0.045], [0.033, 0.140],
    [0.028, 0.160], [-0.028, 0.160], [-0.033, 0.140], [-0.018, 0.045], [-0.070, 0.022],
  ];
  function railSection(off, top) {
    const out = [];
    for (let i = 0; i < RAIL_SECTION.length; i += 1) {
      const p = RAIL_SECTION[i];
      out.push(pt(off + p[0], top + p[1], p[1] > 0.12 ? P.railHead : P.railWeb));
    }
    return out;
  }

  /// The bridge deck: an edge beam and fascia, a cornice, a steel parapet, a
  /// raised footway each side and the same carriageway crown the road has. The
  /// deck does NOT try to become the road cross-section at the abutment — the
  /// piece sweeps the road profile over its approaches and this profile over
  /// its span, and the abutment block covers the change. That is how the
  /// bridge's ports stay bit-identical to a straight's, which is the thing the
  /// seam assertion actually needs.
  function deckProfile() {
    return [
      pt(-5.20, -0.95, P.concrete), pt(-5.20, -0.20, P.concrete), pt(-4.95, 0, P.concrete),
      pt(-4.95, 0.10, P.steel), pt(-4.80, 1.05, P.steel), pt(-4.65, 1.05, P.steel),
      pt(-4.65, 0.14, P.concrete), pt(-4.30, 0.16, P.concrete), pt(-4.10, 0.16, P.kerb),
      pt(-4.10, 0.02, P.asphalt), pt(-3.62, 0.005, P.asphalt), pt(-3.50, 0.02, P.asphalt),
      pt(0, 0.09, P.asphalt), pt(3.50, 0.02, P.asphalt), pt(3.62, 0.005, P.kerb),
      pt(4.10, 0.02, P.kerb), pt(4.10, 0.16, P.concrete), pt(4.30, 0.16, P.concrete),
      pt(4.65, 0.14, P.steel), pt(4.65, 1.05, P.steel), pt(4.80, 1.05, P.steel),
      pt(4.95, 0.10, P.concrete), pt(4.95, 0, P.concrete), pt(5.20, -0.20, P.concrete),
      pt(5.20, -0.95, P.concrete),
    ];
  }
  function deckProfileMap() {
    return [
      pt(-5.20, -0.95, P.concrete), pt(-5.20, 0, P.concrete), pt(-4.80, 1.00, P.steel),
      pt(-4.65, 0.10, P.asphalt), pt(0, 0.09, P.asphalt), pt(4.65, 0.10, P.steel),
      pt(4.80, 1.00, P.concrete), pt(5.20, 0, P.concrete), pt(5.20, -0.95, P.concrete),
    ];
  }
  /// The rail equivalent: a ballasted trough between two plate girders. Ballast
  /// on a bridge is ordinary practice on a 1990 main line and it keeps the
  /// track cross-section continuous across the abutment.
  function railDeckProfile() {
    return [
      pt(-2.40, -1.30, P.steel), pt(-2.40, 0.62, P.steel), pt(-2.20, 0.62, P.steel),
      pt(-2.20, 0.05, P.concrete), pt(-1.85, 0.05, P.ballast), pt(-1.60, 0.58, P.ballast),
      pt(1.60, 0.58, P.ballast), pt(1.85, 0.05, P.concrete), pt(2.20, 0.05, P.steel),
      pt(2.20, 0.62, P.steel), pt(2.40, 0.62, P.steel), pt(2.40, -1.30, P.steel),
    ];
  }
  function railDeckProfileMap() {
    return [
      pt(-2.40, -1.30, P.steel), pt(-2.40, 0.62, P.steel), pt(-1.60, 0.58, P.ballast),
      pt(1.60, 0.58, P.ballast), pt(2.40, 0.62, P.steel), pt(2.40, -1.30, P.steel),
    ];
  }

  // ------------------------------------------------------------- alignment
  /// The station at arc length `s`, interpolated between the authored ones.
  /// Markings, posts and sleepers are placed by distance along the piece rather
  /// than by station index, so a dash is 3 m long on a bend as well as on a
  /// straight and a sleeper pitch does not change with the step count.
  function alignAt(stations, s) {
    const last = stations[stations.length - 1];
    if (s <= 0) return stations[0];
    if (s >= last.s) return last;
    for (let i = 0; i + 1 < stations.length; i += 1) {
      const a = stations[i], b = stations[i + 1];
      if (s > b.s) continue;
      const span = b.s - a.s;
      const t = span > 1e-9 ? (s - a.s) / span : 0;
      const hx = a.hx + (b.hx - a.hx) * t, hz = a.hz + (b.hz - a.hz) * t;
      const l = Math.hypot(hx, hz) || 1;
      return {
        x: a.x + (b.x - a.x) * t, y: a.y + (b.y - a.y) * t, z: a.z + (b.z - a.z) * t,
        hx: hx / l, hz: hz / l, t: a.t + (b.t - a.t) * t, s,
      };
    }
    return last;
  }
  /// The surface height of a cross-section at lateral offset x. Markings sit on
  /// the surface that is actually authored rather than on a nominal plane, so a
  /// centre line follows the crown and an edge line follows the crossfall
  /// instead of hovering over one and sinking into the other.
  function profileY(prof, x) {
    for (let i = 0; i + 1 < prof.length; i += 1) {
      const a = prof[i], b = prof[i + 1];
      if (x < Math.min(a.x, b.x) - 1e-9 || x > Math.max(a.x, b.x) + 1e-9) continue;
      const span = b.x - a.x;
      if (Math.abs(span) < 1e-9) continue;
      return a.y + (b.y - a.y) * ((x - a.x) / span);
    }
    return prof[0].y;
  }
  /// A longitudinal strip of paint between lateral offsets x0 and x1, from arc
  /// length s0 to s1, lifted MARK above the surface. Sampled at the piece's own
  /// stations so the strip follows an arc without a second discretisation.
  Mesh.prototype.stripe = function (stations, prof, x0, x1, s0, s1, col) {
    const cuts = [s0];
    for (let i = 0; i < stations.length; i += 1) {
      if (stations[i].s > s0 + 1e-6 && stations[i].s < s1 - 1e-6) cuts.push(stations[i].s);
    }
    cuts.push(s1);
    const y0 = profileY(prof, x0) + MARK, y1 = profileY(prof, x1) + MARK;
    for (let i = 0; i + 1 < cuts.length; i += 1) {
      const a = alignAt(stations, cuts[i]), b = alignAt(stations, cuts[i + 1]);
      const ar = [a.hz, -a.hx], br = [b.hz, -b.hx];
      this.quad(
        [a.x + x0 * ar[0], a.y + y0, a.z + x0 * ar[1]],
        [b.x + x0 * br[0], b.y + y0, b.z + x0 * br[1]],
        [b.x + x1 * br[0], b.y + y1, b.z + x1 * br[1]],
        [a.x + x1 * ar[0], a.y + y1, a.z + x1 * ar[1]],
        col,
      );
    }
    return this;
  };
  /// Stand at a station with local +Z along the direction of travel and local
  /// +X to its right, so a post, a sign or a sleeper is authored once in plain
  /// coordinates and placed on a straight and an arc by the same call.
  Mesh.prototype.atStation = function (st, fn) {
    this.push(st.hz, st.hx, st.x, st.y, st.z);
    fn(this);
    this.pop();
    return this;
  };

  // --------------------------------------------------------------- dressing
  const DASH = 3.0, DASH_CYCLE = 6.25;   // 6.25 divides the 25 m segment, so
                                         // the dash phase carries across a
                                         // straight seam. It does not carry
                                         // across a bend, whose arc length is
                                         // not a multiple of the cycle; that is
                                         // a real limit of a fixed-piece kit
                                         // and not something this file hides.

  function centreLine(m, stations, prof, len, dashed) {
    if (!dashed) { m.stripe(stations, prof, -0.05, 0.05, 0, len, P.paint); return; }
    for (let s = 0; s + DASH <= len + 1e-6; s += DASH_CYCLE) {
      m.stripe(stations, prof, -0.05, 0.05, s, s + DASH, P.paint);
    }
  }
  function edgeLines(m, stations, prof, len, off) {
    m.stripe(stations, prof, -off - 0.05, -off + 0.05, 0, len, P.paint);
    m.stripe(stations, prof, off - 0.05, off + 0.05, 0, len, P.paint);
  }
  /// Reflective verge posts. One every twelve metres or so, sides alternating
  /// on a hash of the piece and the caller's index, because a road with a post
  /// facing the same way every 25 m for a kilometre reads as wallpaper.
  function markerPosts(m, stations, len, seed) {
    const n = Math.max(1, Math.round(len / 12.5));
    for (let i = 0; i < n; i += 1) {
      const s = len * (i + 0.5) / n;
      const side = (ask(seed, 40 + i) & 1) ? 1 : -1;
      m.atStation(alignAt(stations, s), (b) => {
        b.beam(side * 5.25 - 0.05, side * 5.25 + 0.05, 0.10, 1.05, -0.03, 0.03, P.signFace, 0.012);
      });
    }
  }
  /// A gully in the channel with its grating. Not every segment has one; a
  /// hash of the piece and index decides, which is the whole of the variation a
  /// straight gets and is enough to stop a run reading as a repeated stamp.
  function gully(m, stations, len, seed, side) {
    if (ask(seed, 7) % 3 === 0) return;
    m.atStation(alignAt(stations, len * 0.5), (b) => {
      b.bar(side * 3.78 - 0.24, side * 3.78 + 0.24, 0, 0.04, -0.24, 0.24, P.concrete);
      b.patch(side * 3.78 - 0.20, side * 3.78 + 0.20, -0.20, 0.20, 0.045, P.dark);
    });
  }
  /// A route sign on a single post. The plate carries no legend — this renderer
  /// has no textures, so a sign that claimed to say anything would be a lie
  /// told in geometry. It is a plate of the right size in the right place.
  function signPost(m, st, side, height, w, h, col, off) {
    const x = side * (off == null ? 5.6 : off);
    m.atStation(st, (b) => {
      b.column(x, 0, 0, height, 0.055, 0.05, 6, P.post);
      b.beam(x - w / 2, x + w / 2, height - h, height, -0.04, 0.04, col || P.signFace, 0.015);
    });
  }
  /// A lamp column with an outreach arm. Four segments on the barrel, because
  /// at inspection range the barrel is 140 mm across and any more is spent on
  /// something narrower than the marking beside it.
  function lampColumn(m, st, side) {
    m.atStation(st, (b) => {
      b.column(side * 5.4, 0, 0, 8.0, 0.09, 0.07, 6, P.post);
      b.beam(side * 4.2, side * 5.4, 7.85, 8.0, -0.06, 0.06, P.post, 0.02);
      b.bar(side * 4.0, side * 4.5, 7.62, 7.86, -0.16, 0.16, P.galv);
    });
  }
  /// Tensioned steel barrier: a rail on posts. The rail is a chamfered section
  /// rather than the real corrugated W profile — the corrugation is 80 mm deep
  /// and would cost four times the triangles to say something that vanishes at
  /// any range this kit is looked at from.
  function crashBarrier(m, stations, len, off, y0, y1) {
    // Posts sit at the MIDDLE of each 4 m bay rather than at its ends. A post
    // straddling the end of a segment would stand 60 mm past the port plane and
    // then interpenetrate the first post of the next segment; spacing them
    // inside the piece keeps the run continuous and every port plane clean.
    const n = Math.max(2, Math.round(len / 4));
    for (let i = 0; i < n; i += 1) {
      m.atStation(alignAt(stations, len * (i + 0.5) / n), (b) => {
        b.bar(off - 0.05, off + 0.05, y0, y1, -0.06, 0.06, P.steel);
      });
    }
    for (let i = 0; i + 1 < stations.length; i += 1) {
      const a = stations[i], c = stations[i + 1];
      const ar = [a.hz, -a.hx], cr = [c.hz, -c.hx];
      const lo = y1 - 0.30, hi = y1 - 0.02;
      m.quad(
        [a.x + off * ar[0], a.y + lo, a.z + off * ar[1]],
        [c.x + off * cr[0], c.y + lo, c.z + off * cr[1]],
        [c.x + off * cr[0], c.y + hi, c.z + off * cr[1]],
        [a.x + off * ar[0], a.y + hi, a.z + off * ar[1]], P.galv);
    }
  }

  // ------------------------------------------------------------ rail dressing
  const SLEEPER = { half: 1.30, wide: 0.125, top: 0.60, bottom: 0.40 };
  const RAIL_TOP = SLEEPER.top;

  function sleepers(m, stations, len, pitch, half) {
    const n = Math.max(1, Math.floor(len / pitch));
    for (let i = 0; i < n; i += 1) {
      const s = (i + 0.5) * len / n;
      m.atStation(alignAt(stations, s), (b) => {
        b.buried(-(half || SLEEPER.half), half || SLEEPER.half,
          SLEEPER.bottom, SLEEPER.top, -SLEEPER.wide, SLEEPER.wide, P.sleeper);
      });
    }
  }
  /// A pair of running rails swept along the alignment. Two sweeps rather than
  /// one so each rail gets its own smoothing group and the crease limit does
  /// not weld the two heads together across the four-foot.
  function railPair(m, stations, off, top) {
    for (const side of [-1, 1]) {
      m.sweep(stations, () => railSection(side * off, top));
    }
  }
  /// Overhead line equipment: a mast, a cantilever bracket, a registration arm
  /// and the contact wire itself as a ribbon. The wire is drawn as a ribbon
  /// because it is 12 mm of copper and a tube for it would be four times the
  /// cost of the mast holding it up.
  function catenary(m, stations, len, seed) {
    const side = (ask(seed, 11) & 1) ? 1 : -1;
    m.atStation(alignAt(stations, len * 0.5), (b) => {
      b.column(side * 2.85, 0, 0, 6.6, 0.13, 0.10, 6, P.galv);
      b.beam(side * 0.2, side * 2.85, 6.25, 6.40, -0.05, 0.05, P.galv, 0.02);
      b.bar(side * 0.10, side * 0.30, 5.30, 6.28, -0.03, 0.03, P.galv);
    });
    for (let i = 0; i + 1 < stations.length; i += 1) {
      const a = stations[i], c = stations[i + 1];
      const ar = [a.hz, -a.hx], cr = [c.hz, -c.hx];
      m.quad(
        [a.x - 0.012 * ar[0], a.y + 5.30, a.z - 0.012 * ar[1]],
        [c.x - 0.012 * cr[0], c.y + 5.30, c.z - 0.012 * cr[1]],
        [c.x + 0.012 * cr[0], c.y + 5.30, c.z + 0.012 * cr[1]],
        [a.x + 0.012 * ar[0], a.y + 5.30, a.z + 0.012 * ar[1]], P.steel);
    }
  }

  // ------------------------------------------------------------------ ports
  function port(id, network, position, forward, width, gauge) {
    return { id, network, position, forward, up: [0, 1, 0], width, gauge };
  }
  const IN_FWD = [0, 0, -1];

  // ------------------------------------------------------------ road pieces
  function clampInt(v, lo, hi) { return v < lo ? lo : (v > hi ? hi : v); }

  function roadStraight(m, o) {
    const map = o.lod === "map";
    const len = o.length, base = o.base, rise = o.rise;
    const stations = lineStations(len, map ? 2 : 4, base, rise);
    const shape = map ? roadProfileMap : roadProfile;
    const flat = shape(0, 0, 0);
    m.part("carriageway / formation, kerbs and verges", (b) => {
      b.sweep(stations, (st) => shape(base + rise * st.t, 0, 0));
    });
    m.part("markings / centre line", (b) => centreLine(b, stations, flat, len, true));
    if (!map) {
      m.part("markings / edge lines", (b) => edgeLines(b, stations, flat, len, 3.35));
      m.part("furniture / verge marker posts", (b) => markerPosts(b, stations, len, o.seed));
      m.part("drainage / gully and grating", (b) => gully(b, stations, len, o.seed, -1));
      if (o.sign) {
        m.part("furniture / route sign", (b) => signPost(b, alignAt(stations, len * 0.72), 1, 2.6, 1.5, 0.9));
      }
    }
    return [
      port("in", "road", [0, base, 0], IN_FWD, 2 * ROAD_HALF, 2 * LANE),
      port("out", "road", [0, base + rise, len], [0, 0, 1], 2 * ROAD_HALF, 2 * LANE),
    ];
  }

  function roadBend(m, o) {
    const map = o.lod === "map";
    const defl = o.turn, hand = o.hand, base = o.base;
    const steps = map ? clampInt(Math.ceil(defl / 30), 2, 4) : clampInt(Math.ceil(defl / 11.25), 4, 8);
    const stations = arcStations(ROAD_RADIUS, defl, hand, steps, base);
    const len = stations[stations.length - 1].s;
    const shape = map ? roadProfileMap : roadProfile;
    const flat = shape(0, 0, 0);
    m.part("carriageway / formation, kerbs and verges", (b) => b.sweep(stations, () => shape(base, 0, 0)));
    m.part("markings / centre line", (b) => centreLine(b, stations, flat, len, true));
    if (!map) {
      m.part("markings / edge lines", (b) => edgeLines(b, stations, flat, len, 3.35));
      m.part("furniture / verge marker posts", (b) => markerPosts(b, stations, len, o.seed));
    }
    const ex = arcExit(ROAD_RADIUS, defl, hand, base);
    return [
      port("in", "road", [0, base, 0], IN_FWD, 2 * ROAD_HALF, 2 * LANE),
      port("out", "road", ex.position, ex.forward, 2 * ROAD_HALF, 2 * LANE),
    ];
  }

  /// How far open the kerb line is at distance `z` along a junction tile whose
  /// mouth is centred at `at`. Fully open across the 7 m carriageway, closing
  /// over a 1.5 m kerb radius each side with a smoothstep. This IS the kerb
  /// radius — there is no corner geometry in a junction, only a cross-section
  /// whose kerb walks out to the corridor edge and back.
  function mouthAt(z, at) {
    const d = Math.abs(z - at);
    if (d <= 3.5) return 1;
    if (d >= 5.0) return 0;
    const t = (5.0 - d) / 1.5;
    return t * t * (3 - 2 * t);
  }

  /// A junction tile: the through route swept across a 12 m square with its
  /// kerb opened on one side (T) or both (crossroads). The through ports carry
  /// the road cross-section unchanged, so a straight mated to either of them
  /// meets it vertex for vertex. THE BRANCH PORT DOES NOT: the tile's surface
  /// at the branch mouth is the through carriageway run out to the corridor
  /// edge, which is flat where a straight's verge would be crowned. That is a
  /// real limit of building a junction from one cross-section rather than from
  /// authored corner quadrants, it is worth about 100 mm of grass at the mouth,
  /// and the checks hold the branch to the weaker contract because of it.
  function roadJunction(m, o, both) {
    const map = o.lod === "map";
    const base = o.base, side = 2 * ROAD_HALF;
    const steps = map ? 4 : 10;
    const stations = lineStations(side, steps, base, 0);
    const shape = map ? roadProfileMap : roadProfile;
    m.part("carriageway / junction tile and kerb radii", (b) => {
      b.sweep(stations, (st) => {
        const open = mouthAt(st.z, ROAD_HALF);
        return shape(base, both ? open : 0, open);
      });
    });
    const flat = shape(0, 0, 0);
    m.part("markings / approach centre lines", (b) => {
      centreLine(b, stations, flat, 2.5, true);
      for (let s = side - 2.5; s + DASH <= side + 1e-6; s += DASH_CYCLE) {
        b.stripe(stations, flat, -0.05, 0.05, s, s + DASH, P.paint);
      }
    });
    m.part("markings / give way lines", (b) => {
      const y = 0.02 + MARK;
      b.patch(3.45, 3.75, 2.5, 9.5, y, P.paint);
      if (both) b.patch(-3.75, -3.45, 2.5, 9.5, y, P.paint);
    });
    if (!map) {
      m.part("furniture / junction signs", (b) => {
        signPost(b, { x: 0, y: base, z: 1.2, hx: 0, hz: 1 }, 1, 2.4, 1.3, 0.8, P.signFace, 4.9);
        if (both) signPost(b, { x: 0, y: base, z: side - 1.2, hx: 0, hz: 1 }, -1, 2.4, 1.3, 0.8, P.signFace, 4.9);
      });
      m.part("furniture / lamp column", (b) => {
        lampColumn(b, { x: 0, y: base, z: 1.6, hx: 0, hz: 1 }, both ? 1 : -1);
      });
    }
    const ports = [
      port("in", "road", [0, base, 0], IN_FWD, side, 2 * LANE),
      port("out", "road", [0, base, side], [0, 0, 1], side, 2 * LANE),
      port("branch", "road", [ROAD_HALF, base, ROAD_HALF], [1, 0, 0], side, 2 * LANE),
    ];
    if (both) ports.push(port("branch_left", "road", [-ROAD_HALF, base, ROAD_HALF], [-1, 0, 0], side, 2 * LANE));
    return ports;
  }

  // --------------------------------------------------------- highway pieces
  function highwayDressing(m, stations, len, map, base) {
    const flat = map ? highwayProfileMap(0) : highwayProfile(0);
    m.part("markings / lane and edge lines", (b) => {
      b.stripe(stations, flat, -2.40, -2.30, 0, len, P.paint);
      b.stripe(stations, flat, 2.30, 2.40, 0, len, P.paint);
      b.stripe(stations, flat, -9.35, -9.25, 0, len, P.paint);
      b.stripe(stations, flat, 9.25, 9.35, 0, len, P.paint);
      if (map) return;
      for (let s = 0; s + DASH <= len + 1e-6; s += DASH_CYCLE) {
        b.stripe(stations, flat, -5.70, -5.60, s, s + DASH, P.paint);
        b.stripe(stations, flat, 5.60, 5.70, s, s + DASH, P.paint);
      }
    });
    if (map) return;
    m.part("safety / central reserve barrier", (b) => crashBarrier(b, stations, len, 0, 0.34, 1.09));
    m.part("safety / verge barriers", (b) => {
      crashBarrier(b, stations, len, -11.90, 0.28, 1.03);
      crashBarrier(b, stations, len, 11.90, 0.28, 1.03);
    });
  }

  function highwayStraight(m, o) {
    const map = o.lod === "map";
    const len = o.length, base = o.base, rise = o.rise;
    const stations = lineStations(len, map ? 2 : 4, base, rise);
    const shape = map ? highwayProfileMap : highwayProfile;
    m.part("carriageway / formation, shoulders and central reserve", (b) => {
      b.sweep(stations, (st) => shape(base + rise * st.t));
    });
    highwayDressing(m, stations, len, map, base);
    return [
      port("in", "highway", [0, base, 0], IN_FWD, 2 * HW_HALF, 8.7),
      port("out", "highway", [0, base + rise, len], [0, 0, 1], 2 * HW_HALF, 8.7),
    ];
  }

  function highwayBend(m, o) {
    const map = o.lod === "map";
    const defl = o.turn, hand = o.hand, base = o.base;
    const steps = map ? 2 : clampInt(Math.ceil(defl / 3), 2, 6);
    const stations = arcStations(HW_RADIUS, defl, hand, steps, base);
    const len = stations[stations.length - 1].s;
    const shape = map ? highwayProfileMap : highwayProfile;
    m.part("carriageway / formation, shoulders and central reserve", (b) => {
      b.sweep(stations, () => shape(base));
    });
    highwayDressing(m, stations, len, map, base);
    const ex = arcExit(HW_RADIUS, defl, hand, base);
    return [
      port("in", "highway", [0, base, 0], IN_FWD, 2 * HW_HALF, 8.7),
      port("out", "highway", ex.position, ex.forward, 2 * HW_HALF, 8.7),
    ];
  }

  // ------------------------------------------------------------ rail pieces
  function railDressing(m, stations, len, map, seed, catenaryOn) {
    if (map) {
      m.part("track / running rails", (b) => {
        for (const side of [-1, 1]) {
          b.sweep(stations, () => [
            pt(side * GAUGE / 2 - 0.07, RAIL_TOP + 0.14, P.railHead),
            pt(side * GAUGE / 2 + 0.07, RAIL_TOP + 0.14, P.railHead),
          ], false);
        }
      });
      return;
    }
    m.part("track / sleepers", (b) => sleepers(b, stations, len, SLEEPER_PITCH));
    m.part("track / running rails", (b) => railPair(b, stations, GAUGE / 2, RAIL_TOP));
    if (catenaryOn) m.part("electrification / catenary mast and contact wire", (b) => catenary(b, stations, len, seed));
  }

  function railStraight(m, o) {
    const map = o.lod === "map";
    const len = o.length, base = o.base, rise = o.rise;
    const stations = lineStations(len, map ? 2 : 4, base, rise);
    const shape = map ? railProfileMap : railProfile;
    m.part("formation / ballast and shoulders", (b) => {
      b.sweep(stations, (st) => shape(base + rise * st.t));
    });
    railDressing(m, stations, len, map, o.seed, o.catenary);
    return [
      port("in", "rail", [0, base, 0], IN_FWD, 2 * RAIL_HALF, GAUGE),
      port("out", "rail", [0, base + rise, len], [0, 0, 1], 2 * RAIL_HALF, GAUGE),
    ];
  }

  function railCurve(m, o) {
    const map = o.lod === "map";
    const defl = o.turn, hand = o.hand, base = o.base;
    const steps = map ? 2 : clampInt(Math.ceil(defl / 2), 2, 4);
    const stations = arcStations(RAIL_RADIUS, defl, hand, steps, base);
    const len = stations[stations.length - 1].s;
    const shape = map ? railProfileMap : railProfile;
    m.part("formation / ballast and shoulders", (b) => b.sweep(stations, () => shape(base)));
    railDressing(m, stations, len, map, o.seed, o.catenary);
    const ex = arcExit(RAIL_RADIUS, defl, hand, base);
    return [
      port("in", "rail", [0, base, 0], IN_FWD, 2 * RAIL_HALF, GAUGE),
      port("out", "rail", ex.position, ex.forward, 2 * RAIL_HALF, GAUGE),
    ];
  }

  /// A turnout. The diverging route leaves the toe as a STRAIGHT at the
  /// crossing angle rather than as the curve a real turnout uses, which is the
  /// honest limit of doing this with one sweep: a 1 in 8 turnout's lead curve
  /// is 200 m radius over 20 m and the difference across the piece is under
  /// 250 mm. The switch blades are modelled, the crossing nose is not.
  const SWITCH_ANGLE = 7.125;   // 1 in 8
  const SWITCH_LEN = 30;

  function railSwitch(m, o) {
    const map = o.lod === "map";
    const base = o.base, hand = o.hand < 0 ? -1 : 1;
    const steps = map ? 2 : 6;
    const stations = lineStations(SWITCH_LEN, steps, base, 0);
    const a = SWITCH_ANGLE * DEG, ta = Math.tan(a);
    // The diverging alignment, as its own station list so the rails, the
    // ballast widening and the exit port all read from the same numbers.
    // The diverging rails run from the switch blades to the branch port. The
    // near end is inset because the rail section is rotated by the crossing
    // angle and, swept flush to z = 0, its outer foot would dip 98 mm behind
    // the toe plane and into whatever is mated there. The far end is NOT inset:
    // it has to land exactly in the branch's own plane, and the price is that
    // the same 98 mm stands past the square through exit — which is why the
    // turnout is one of the pieces held to the looser end contract.
    const div = [];
    const z0 = 0.6, z1 = SWITCH_LEN;
    for (let j = 0; j <= steps; j += 1) {
      const t = j / steps, z = z0 + (z1 - z0) * t;
      div.push({ x: hand * z * ta, y: base, z, hx: hand * Math.sin(a), hz: Math.cos(a), t, s: (z - z0) / Math.cos(a) });
    }
    const shape = map ? railProfileMap : railProfile;
    // The ballast pad widens with the diverging track. Adding the divergence to
    // the outer shoulder of the through cross-section keeps it one sweep.
    m.part("formation / turnout ballast", (b) => {
      b.sweep(stations, (st) => {
        const spread = hand * st.z * ta;
        const prof = shape(base);
        return prof.map((p, i) => {
          const outer = hand > 0 ? i >= prof.length - 3 : i <= 2;
          return outer ? pt(p.x + spread, p.y, p.col) : p;
        });
      });
    });
    if (!map) {
      m.part("track / bearers", (b) => sleepers(b, stations, SWITCH_LEN, 0.75, 1.30 + Math.abs(hand) * 0.9));
      m.part("track / through rails", (b) => railPair(b, stations, GAUGE / 2, RAIL_TOP));
      m.part("track / diverging rails", (b) => railPair(b, div, GAUGE / 2, RAIL_TOP));
      m.part("track / switch blades", (b) => {
        for (const s of [-1, 1]) {
          b.atStation(stations[0], (c) => {
            c.beam(s * GAUGE / 2 - 0.03, s * GAUGE / 2 + 0.03, RAIL_TOP + 0.02, RAIL_TOP + 0.15, 0.4, 5.2, P.railHead, 0.01);
          });
        }
      });
    } else {
      m.part("track / running rails", (b) => {
        for (const list of [stations, div]) {
          for (const side of [-1, 1]) {
            b.sweep(list, () => [
              pt(side * GAUGE / 2 - 0.07, RAIL_TOP + 0.14, P.railHead),
              pt(side * GAUGE / 2 + 0.07, RAIL_TOP + 0.14, P.railHead),
            ], false);
          }
        }
      });
    }
    return [
      port("in", "rail", [0, base, 0], IN_FWD, 2 * RAIL_HALF, GAUGE),
      port("out", "rail", [0, base, SWITCH_LEN], [0, 0, 1], 2 * RAIL_HALF, GAUGE),
      port("branch", "rail", [hand * SWITCH_LEN * ta, base, SWITCH_LEN],
        [hand * Math.sin(a), 0, Math.cos(a)], 2 * RAIL_HALF, GAUGE),
    ];
  }

  // ------------------------------------------------------- structure pieces
  /// Stations along +Z beginning at z0. Bridges and portals are built from
  /// several sweeps butted together inside one piece, so they need to start
  /// somewhere other than the origin.
  function lineFrom(z0, len, steps, base) {
    const out = [];
    for (let j = 0; j <= steps; j += 1) {
      const t = j / steps;
      out.push({ x: 0, y: base, z: z0 + len * t, hx: 0, hz: 1, t, s: len * t });
    }
    return out;
  }
  /// Stations along +X at a fixed z. The rail arm of a level crossing runs
  /// across the road, and running it as its own alignment rather than as a
  /// rotated copy keeps the crossing's four ports in one coordinate system.
  function crossFrom(x0, len, steps, z, base) {
    const out = [];
    for (let j = 0; j <= steps; j += 1) {
      const t = j / steps;
      out.push({ x: x0 + len * t, y: base, z, hx: 1, hz: 0, t, s: len * t });
    }
    return out;
  }

  /// A bridge over a gap. The road crosses at DECK_RISE above the bed, on
  /// embankment approaches at each end, so the piece's PORTS ARE AT DECK LEVEL
  /// and its ground contact is the bed of the gap — that is the whole reason
  /// the port table carries a height at all. A run reaches a bridge by climbing
  /// on graded straights (`rise`), which is what `assemble` does when the path
  /// carries heights, and a straight at base 0 will not mate with a bridge at
  /// base 5 by design: the contract refuses the join instead of hiding a five
  /// metre step inside it.
  function roadBridge(m, o) {
    const map = o.lod === "map";
    const base = o.base, len = 40, app = 5, span = len - 2 * app;
    const shape = map ? roadProfileMap : roadProfile;
    const deck = map ? deckProfileMap : deckProfile;
    m.part("approach / embankment and carriageway", (b) => {
      b.sweep(lineFrom(0, app, map ? 1 : 2, base), () => shape(base, 0, 0));
      b.sweep(lineFrom(len - app, app, map ? 1 : 2, base), () => shape(base, 0, 0));
    });
    m.part("deck / slab, footways and parapets", (b) => {
      b.sweep(lineFrom(app, span, map ? 2 : 6, base), () => deck());
    });
    m.part("substructure / piers", (b) => {
      for (const z of [len * 0.375, len * 0.625]) {
        b.column(0, z, 0, base - 1.05, 1.15, 1.0, map ? 4 : 8, P.concrete);
        if (!map) b.bar(-1.6, 1.6, base - 1.05, base - 0.92, z - 1.5, z + 1.5, P.concrete);
      }
    });
    if (!map) {
      m.part("substructure / abutments and wing walls", (b) => {
        for (const z of [app - 0.8, len - app + 0.05]) b.bar(-5.6, 5.6, 0, base - 0.05, z, z + 0.75, P.concrete);
        for (const z of [app - 0.8, len - app + 0.05]) {
          b.bar(-6.4, -5.6, 0, base - 0.4, z - 2.6, z + 3.3, P.concrete);
          b.bar(5.6, 6.4, 0, base - 0.4, z - 2.6, z + 3.3, P.concrete);
        }
      });
      m.part("gap / bed", (b) => b.patch(-14, 14, app - 1, len - app + 1, 0.02, P.water));
    }
    return [
      port("in", "road", [0, base, 0], IN_FWD, 2 * ROAD_HALF, 2 * LANE),
      port("out", "road", [0, base, len], [0, 0, 1], 2 * ROAD_HALF, 2 * LANE),
    ];
  }

  /// The rail equivalent, shorter because a segment's cost is dominated by
  /// sleepers and a 40 m ballasted deck would spend 600 triangles on them
  /// alone. Plate girders each side, ballast carried across, ordinary 1990
  /// practice for a short main-line underbridge.
  function railBridge(m, o) {
    const map = o.lod === "map";
    const base = o.base, len = 30, app = 5, span = len - 2 * app;
    const shape = map ? railProfileMap : railProfile;
    const deck = map ? railDeckProfileMap : railDeckProfile;
    const whole = lineFrom(0, len, map ? 2 : 6, base);
    m.part("approach / embankment and ballast", (b) => {
      b.sweep(lineFrom(0, app, map ? 1 : 2, base), () => shape(base));
      b.sweep(lineFrom(len - app, app, map ? 1 : 2, base), () => shape(base));
    });
    m.part("deck / plate girders and ballast trough", (b) => {
      b.sweep(lineFrom(app, span, map ? 2 : 4, base), () => deck());
    });
    m.part("substructure / piers and abutments", (b) => {
      b.column(0, len * 0.5, 0, base - 1.35, 1.2, 1.05, map ? 4 : 8, P.concrete);
      if (!map) {
        for (const z of [app - 0.8, len - app + 0.05]) b.bar(-3.0, 3.0, 0, base - 0.05, z, z + 0.75, P.concrete);
      }
    });
    if (map) {
      m.part("track / running rails", (b) => {
        for (const side of [-1, 1]) {
          b.sweep(whole, () => [
            pt(side * GAUGE / 2 - 0.07, RAIL_TOP + 0.14, P.railHead),
            pt(side * GAUGE / 2 + 0.07, RAIL_TOP + 0.14, P.railHead),
          ], false);
        }
      });
    } else {
      m.part("track / sleepers", (b) => sleepers(b, whole, len, SLEEPER_PITCH));
      m.part("track / running rails", (b) => railPair(b, whole, GAUGE / 2, RAIL_TOP));
      m.part("gap / bed", (b) => b.patch(-9, 9, app - 1, len - app + 1, 0.02, P.water));
    }
    return [
      port("in", "rail", [0, base, 0], IN_FWD, 2 * RAIL_HALF, GAUGE),
      port("out", "rail", [0, base, len], [0, 0, 1], 2 * RAIL_HALF, GAUGE),
    ];
  }

  /// A tunnel portal: the one piece in this kit with a single port, because a
  /// tunnel is where a route STOPS being modelled. The bore runs a few metres
  /// in and ends in a dark face; nothing behind that is authored, and nothing
  /// here says how long the tunnel is or that one exists.
  function tunnelPortal(m, o) {
    const map = o.lod === "map";
    const rail = o.mode === "rail";
    const app = 6, face = 6.6, seg = map ? 5 : 10;
    const half = rail ? RAIL_HALF : ROAD_HALF;
    const springing = rail ? 1.4 : 1.2;
    const radius = rail ? 3.0 : 4.6;
    const wallTop = springing + radius + 1.6;
    const shape = rail ? (map ? railProfileMap : railProfile) : (map ? roadProfileMap : roadProfile);
    m.part("approach / formation", (b) => {
      b.sweep(lineFrom(0, app, map ? 1 : 2, 0), () => (rail ? shape(0) : shape(0, 0, 0)));
    });
    if (rail && !map) {
      m.part("track / sleepers and rails", (b) => {
        const st = lineFrom(0, app, 2, 0);
        sleepers(b, st, app, SLEEPER_PITCH);
        railPair(b, st, GAUGE / 2, RAIL_TOP);
      });
    }
    // The arch, as a list of points from one springing to the other.
    const arch = [];
    for (let i = 0; i <= seg; i += 1) {
      const a = Math.PI * (i / seg);
      arch.push([-radius * Math.cos(a), springing + radius * Math.sin(a)]);
    }
    m.part("portal / headwall and arch", (b) => {
      b.bar(-half - 1.5, -radius, 0, wallTop, face, face + 1.4, P.concrete);
      b.bar(radius, half + 1.5, 0, wallTop, face, face + 1.4, P.concrete);
      for (let i = 0; i < arch.length - 1; i += 1) {
        const a = arch[i], c = arch[i + 1];
        b.quad([a[0], a[1], face], [c[0], c[1], face], [c[0], wallTop, face], [a[0], wallTop, face], P.concrete);
      }
      b.beam(-half - 1.6, half + 1.6, wallTop, wallTop + 0.45, face - 0.12, face + 1.5, P.concrete, 0.04);
    });
    m.part("portal / bore", (b) => {
      b.smooth((c) => {
        for (let i = 0; i < arch.length - 1; i += 1) {
          const a = arch[i], d = arch[i + 1];
          c.quad([a[0], a[1], face + 14], [d[0], d[1], face + 14], [d[0], d[1], face], [a[0], a[1], face], P.dark);
        }
      });
      b.fan(arch.map((p) => [p[0], p[1], face + 14]), P.dark);
    });
    m.part("cutting / hillside", (b) => {
      const berm = [
        pt(-(half + 10), 0, P.grass), pt(-(half + 3), wallTop * 0.62, P.grass),
        pt(0, wallTop + 2.4, P.grass), pt(half + 3, wallTop * 0.62, P.grass),
        pt(half + 10, 0, P.earth),
      ];
      const st = lineFrom(face + 1.4, 16, map ? 1 : 3, 0);
      b.sweep(st, () => berm);
      b.fan(berm.map((p) => [p.x, p.y, face + 1.4]).reverse(), P.grass);
      b.fan(berm.map((p) => [p.x, p.y, face + 17.4]), P.grass);
    });
    return [port("in", rail ? "rail" : "road", [0, 0, 0], IN_FWD,
      2 * (rail ? RAIL_HALF : ROAD_HALF), rail ? GAUGE : 2 * LANE)];
  }

  /// Where a road crosses a railway at grade. FOUR PORTS, two networks, and the
  /// only piece here that has to reconcile two formations: the track keeps its
  /// full 600 mm ballast so its ports match a rail straight vertex for vertex,
  /// and the ROAD is humped over it, returning to grade at its own ports. A
  /// hump is what a level crossing actually is, and it is also the only way
  /// both seams can be exact at once.
  function levelCrossing(m, o) {
    const map = o.lod === "map";
    const tile = 2 * ROAD_HALF, reach = 8, at = ROAD_HALF;
    // The crest is set so the CROWN of the carriageway lands on the rail head:
    // RAIL_TOP + 0.16 is the top of the rail, and the crown stands 0.09 above
    // the formation, so the crest is the difference. At 0.62 the road sat 50 mm
    // below the rail and a car crossed a ridge; a real crossing is flush and
    // the rail head is proud by a few millimetres, which is what this gives.
    const CREST = RAIL_TOP + 0.16 - 0.09;
    const hump = (z) => {
      const d = Math.abs(z - at);
      if (d <= 2.2) return CREST;
      if (d >= at) return 0;
      const t = (at - d) / (at - 2.2);
      return CREST * t * t * (3 - 2 * t);
    };
    const shape = map ? roadProfileMap : roadProfile;
    const railShape = map ? railProfileMap : railProfile;
    const roadSt = [];
    const steps = map ? 4 : 8;
    for (let j = 0; j <= steps; j += 1) {
      const t = j / steps, z = tile * t;
      roadSt.push({ x: 0, y: hump(z), z, hx: 0, hz: 1, t, s: z });
    }
    m.part("carriageway / humped crossing approach", (b) => {
      b.sweep(roadSt, (st) => shape(hump(st.z), 0, 0));
    });
    m.part("formation / ballast outside the carriageway", (b) => {
      b.sweep(crossFrom(-reach, reach - 3.9, map ? 1 : 2, at, 0), () => railShape(0));
      b.sweep(crossFrom(3.9, reach - 3.9, map ? 1 : 2, at, 0), () => railShape(0));
    });
    const railSt = crossFrom(-reach, 2 * reach, map ? 2 : 4, at, 0);
    if (map) {
      m.part("track / running rails", (b) => {
        for (const side of [-1, 1]) {
          b.sweep(railSt, () => [
            pt(side * GAUGE / 2 - 0.07, RAIL_TOP + 0.14, P.railHead),
            pt(side * GAUGE / 2 + 0.07, RAIL_TOP + 0.14, P.railHead),
          ], false);
        }
      });
    } else {
      m.part("track / running rails", (b) => railPair(b, railSt, GAUGE / 2, RAIL_TOP));
      m.part("track / sleepers", (b) => {
        for (const x0 of [-reach, 4.2]) {
          sleepers(b, crossFrom(x0, reach - 4.2, 2, at, 0), reach - 4.2, SLEEPER_PITCH);
        }
      });
      // The panels are the surface a car actually drives on across the track,
      // so they have to sit ON the carriageway, not at a nominal rail height.
      // As flat patches at RAIL_TOP + 0.005 all six of their triangles were
      // buried 20 to 105 mm INSIDE the crowned road above them — never the top
      // surface at any point of the crossing, which is geometry paid for and
      // never seen — and they ran out to x = 4.2, over the kerb line at 3.90
      // and into the verge. Drawn as stripes on the road's own cross-section
      // they follow the crown and the hump, and they stop at the carriageway
      // edge. Banded at the profile's own vertices so each band is a chord of
      // a surface that is straight between them, not across the crown.
      m.part("track / crossing panels", (b) => {
        const flat = shape(0, 0, 0);
        const bands = [-3.50, -1.75, 0, 1.75, 3.50];
        const lanes = [
          [at - 1.35, at - GAUGE / 2 - 0.08],
          [at - GAUGE / 2 + 0.08, at + GAUGE / 2 - 0.08],
          [at + GAUGE / 2 + 0.08, at + 1.35],
        ];
        for (const lane of lanes) {
          for (let i = 0; i + 1 < bands.length; i += 1) {
            b.stripe(roadSt, flat, bands[i], bands[i + 1], lane[0], lane[1], P.concrete);
          }
        }
      });
      m.part("safety / half barriers", (b) => {
        for (const s of [-1, 1]) {
          const z = at - s * 3.4;
          b.column(s * 4.6, z, 0.05, 1.15, 0.16, 0.14, 6, P.galv);
          // The boom lies ACROSS the carriageway from its post to the centre
          // line — a half barrier, which is what a 1990 automatic crossing has.
          b.beam(Math.min(0.1, s * 4.5), Math.max(0.1, s * 4.5), 0.98, 1.12, z - 0.09, z + 0.09, P.signFace, 0.03);
          b.bar(s * 4.6 - 0.3, s * 4.6 + 0.3, 0, 0.12, z - 0.4, z + 0.4, P.concrete);
        }
      });
      m.part("safety / warning lights", (b) => {
        for (const s of [-1, 1]) {
          const z = at - s * 4.4;
          b.column(s * 5.2, z, 0, 2.6, 0.07, 0.06, 6, P.post);
          b.bar(s * 5.2 - 0.42, s * 5.2 + 0.42, 2.35, 2.75, z - 0.12, z + 0.12, P.dark);
        }
      });
    }
    return [
      port("in", "road", [0, 0, 0], IN_FWD, tile, 2 * LANE),
      port("out", "road", [0, 0, tile], [0, 0, 1], tile, 2 * LANE),
      port("rail_west", "rail", [-reach, 0, at], [-1, 0, 0], 2 * RAIL_HALF, GAUGE),
      port("rail_east", "rail", [reach, 0, at], [1, 0, 0], 2 * RAIL_HALF, GAUGE),
    ];
  }

  // --------------------------------------------------------------- the kit
  // Thirteen pieces. Everything else a network needs is an option on one of
  // them (a gradient, a deflection, a hand, a length) rather than a fourteenth
  // piece, because an option cannot drift away from the piece it varies and a
  // near-copy can.
  const PIECES = {
    road_straight: {
      label: "Two-lane road, straight", network: "road", draw: roadStraight,
      straight: true, length: SEG, signs: true,
      blurb: "7 m carriageway on 3.5 m lanes, kerbed both sides with grass verges",
    },
    road_bend: {
      label: "Two-lane road, bend", network: "road", draw: roadBend,
      deflections: [1, 2, 5, 10, 15, 30, 45, 90], radius: ROAD_RADIUS,
      blurb: "the same cross-section swept round a 20 m radius",
    },
    road_junction: {
      label: "Two-lane road, T junction", network: "road", draw: (m, o) => roadJunction(m, o, false), atGrade: true,
      blurb: "a 12 m tile with the kerb opened to a side road on one side",
    },
    road_crossroads: {
      label: "Two-lane road, crossroads", network: "road", draw: (m, o) => roadJunction(m, o, true), atGrade: true,
      blurb: "the same tile with both kerbs opened",
    },
    highway_straight: {
      label: "Divided highway, straight", network: "highway", draw: highwayStraight,
      straight: true, length: SEG,
      blurb: "two two-lane carriageways, hard shoulders and a kerbed central reserve",
    },
    highway_bend: {
      label: "Divided highway, bend", network: "highway", draw: highwayBend,
      deflections: [1, 2, 5, 10], radius: HW_RADIUS,
      blurb: "the same cross-section swept round a 120 m radius",
    },
    rail_straight: {
      label: "Single track railway, straight", network: "rail", draw: railStraight,
      straight: true, length: SEG, electrified: true, maxLength: 50,
      blurb: "standard gauge on concrete sleepers and ballast, with overhead line",
    },
    rail_curve: {
      label: "Single track railway, curve", network: "rail", draw: railCurve,
      deflections: [0.5, 1, 2, 3.5], radius: RAIL_RADIUS, electrified: true,
      blurb: "the same track swept round a 400 m radius",
    },
    rail_switch: {
      label: "Single track railway, turnout", network: "rail", draw: railSwitch,
      handed: true, atGrade: true, blurb: "a 1 in 8 turnout with switch blades and long bearers",
    },
    road_bridge: {
      label: "Road bridge", network: "road", draw: roadBridge, base: DECK_RISE, structure: true,
      blurb: "a 30 m span on two piers with embankment approaches and steel parapets",
    },
    rail_bridge: {
      label: "Rail bridge", network: "rail", draw: railBridge, base: RAIL_DECK_RISE, structure: true,
      blurb: "a 20 m plate-girder underbridge carrying ballasted track",
    },
    tunnel_portal: {
      label: "Tunnel portal", network: "road", draw: tunnelPortal, modes: true,
      blurb: "an arched masonry portal in a cutting, for road or for rail",
    },
    level_crossing: {
      label: "Level crossing", network: "road", draw: levelCrossing, atGrade: true,
      blurb: "a road humped over a railway at grade, with half barriers",
    },
  };
  const PIECE_KEYS = Object.keys(PIECES);
  const FALLBACK = "road_straight";

  const NETWORKS = {
    road: { straight: "road_straight", bend: "road_bend", radius: ROAD_RADIUS, segment: SEG },
    highway: { straight: "highway_straight", bend: "highway_bend", radius: HW_RADIUS, segment: SEG },
    rail: { straight: "rail_straight", bend: "rail_curve", radius: RAIL_RADIUS, segment: SEG },
  };

  /// ONE LOD VOCABULARY ACROSS THE ART MODULES. site-mesh.js grew up saying
  /// "far" and town-mesh.js saying "map" for the same thing, and each used to
  /// fall back silently to its expensive path when handed the other's word.
  /// Both words and the numeric forms mean the coarse level here too.
  function normaliseLod(v) {
    if (v === 1 || v === 2 || v === "map" || v === "far" || v === "lod1" || v === "lod2") return "map";
    return "near";
  }
  function clampNum(v, lo, hi, fallback) {
    if (typeof v !== "number" || !isFinite(v)) return fallback;
    return v < lo ? lo : (v > hi ? hi : v);
  }
  /// The allowed deflection nearest the one asked for. A kit has the bends it
  /// has; asking for 37 degrees gets 30 and says so in `turn`, so a caller can
  /// see what it was actually given rather than believing what it asked.
  function nearestDeflection(list, want) {
    if (typeof want !== "number" || !isFinite(want)) return list[list.length - 1];
    const a = Math.abs(want);
    let best = list[0], gap = Math.abs(a - list[0]);
    for (let i = 1; i < list.length; i += 1) {
      const g = Math.abs(a - list[i]);
      if (g < gap) { gap = g; best = list[i]; }
    }
    return best;
  }

  function options(name, spec, opts) {
    const o = opts && typeof opts === "object" ? opts : {};
    const index = typeof o.index === "number" && isFinite(o.index)
      ? Math.abs(Math.round(o.index)) % 100000 : 0;
    const straight = !!spec.straight;
    // A junction, a turnout and a level crossing are GRADE pieces and refuse a
    // formation height. Their ports face sideways as well as along the route,
    // and an embankment's fill skirt widens by twice its height — at three
    // metres of fill a junction's own toe stands six metres past its side
    // road's port plane. There is no honest cross-section for that, so the
    // option is refused rather than half-honoured.
    const baseHeight = spec.atGrade ? 0
      : (spec.structure ? clampNum(o.base, 2.5, 20, spec.base)
        : (spec.base != null ? spec.base : clampNum(o.base, 0, 40, 0)));
    return {
      lod: normaliseLod(o.lod),
      index,
      seed: seedOf(name, index),
      base: baseHeight,
      // A FALL MAY NOT BE DEEPER THAN THE FORMATION IT STARTS ON. The
      // cross-section draws a fill skirt going DOWN and OUT from the formation
      // by 2h; at a negative formation height that skirt would rise and turn
      // inward, which is not a cutting, it is nonsense. Before this clamp a
      // straight asked for base 0 and rise -8 was authored eight metres below
      // grade and `finish` quietly lifted the whole piece — the one correction
      // this file says it never applies — carrying its ports with it. The
      // refusal is the same one `base` already makes, and `rise` on the
      // returned mesh reports what was actually given.
      rise: straight ? clampNum(o.rise, baseHeight > 0 ? Math.max(-8, -baseHeight) : 0, 8, 0) : 0,
      // A rail straight's cost is dominated by SLEEPERS, one every 650 mm, so
      // its length clamp is tighter than a road's: at 60 m a single segment
      // spends 920 triangles on sleepers alone and leaves the near budget.
      length: straight ? clampNum(o.length, 4, spec.maxLength || 60, spec.length) : 0,
      turn: spec.deflections ? nearestDeflection(spec.deflections, o.turn) : 0,
      hand: (o.hand === "left" || o.hand === -1) ? -1 : 1,
      catenary: spec.electrified ? o.catenary !== false : false,
      sign: spec.signs ? o.sign === true : false,
      mode: spec.modes && o.mode === "rail" ? "rail" : "road",
    };
  }

  function describe(key, spec, o, ports) {
    const bits = [spec.label];
    if (spec.straight) bits.push(o.length.toFixed(1) + " m");
    if (spec.deflections) bits.push(o.turn + " deg " + (o.hand < 0 ? "left" : "right")
      + " at " + spec.radius + " m radius");
    if (spec.handed) bits.push(o.hand < 0 ? "left hand" : "right hand");
    if (spec.modes) bits.push(o.mode);
    if (o.rise) bits.push((o.rise > 0 ? "rising " : "falling ") + Math.abs(o.rise).toFixed(2) + " m");
    if (o.base) bits.push("formation " + o.base.toFixed(2) + " m above natural ground");
    bits.push(o.lod === "map" ? "map detail" : "near detail");
    return bits.join(", ") + ". " + spec.blurb.charAt(0).toUpperCase() + spec.blurb.slice(1)
      + ". Original game art: a segment of a generic transport kit, not a survey of any real "
      + "road or railway, with no measured dimension of a real route and no capability of its "
      + "own. Mates at " + ports.map((p) => p.id).join(", ") + ".";
  }

  function drawPiece(m, name, opts) {
    const key = Object.prototype.hasOwnProperty.call(PIECES, name) ? name : FALLBACK;
    const spec = PIECES[key];
    const o = options(key, spec, opts);
    const ports = spec.draw(m, o);
    return { key, spec, o, ports };
  }

  /// One piece, on its own, rooted at its entry port.
  function build(name, opts) {
    const m = new Mesh();
    const drawn = drawPiece(m, name, opts);
    const o = drawn.o, spec = drawn.spec;
    const len = spec.straight ? o.length
      : (spec.deflections ? spec.radius * o.turn * DEG : null);
    return m.finish({
      piece: drawn.key,
      requestedPiece: name,
      label: spec.label,
      network: spec.network,
      lod: o.lod,
      base: o.base,
      rise: o.rise,
      turn: spec.deflections ? o.turn : 0,
      hand: o.hand,
      index: o.index,
      mode: spec.modes ? o.mode : null,
      runLength: len,
      ports: drawn.ports,
      description: describe(drawn.key, spec, o, drawn.ports),
    });
  }

  // ------------------------------------------------------------- the mating
  /// The frame that places a piece so that its `local` port meets `world`. The
  /// two forwards end up exact opposites and the two positions coincide, which
  /// is the whole of the endpoint contract — nothing downstream measures a seam
  /// and nudges it.
  function mateFrame(world, local) {
    const w = [-world.forward[0], -world.forward[1], -world.forward[2]];
    const yaw = yawTaking(local.forward, w);
    const f = { c: yaw.c, s: yaw.s, x: 0, y: 0, z: 0 };
    const p = apply(f, local.position);
    f.x = world.position[0] - p[0];
    f.y = world.position[1] - p[1];
    f.z = world.position[2] - p[2];
    return f;
  }
  function placePort(f, p) {
    return {
      id: p.id, network: p.network,
      position: apply(f, p.position),
      forward: direction(f, p.forward),
      up: [0, 1, 0], width: p.width, gauge: p.gauge,
    };
  }
  /// Whether two ports may be joined, and why not when they may not. Height is
  /// part of it: a straight at grade and a bridge deck five metres up are both
  /// road ports of the same width and they still do not mate.
  function compatible(a, b, tol) {
    const t = tol == null ? 0.001 : tol;
    if (a.network !== b.network) return { ok: false, why: "different networks" };
    if (Math.abs(a.width - b.width) > 0.001) return { ok: false, why: "different corridor widths" };
    if (Math.abs(a.gauge - b.gauge) > 0.001) return { ok: false, why: "different gauges" };
    const gap = Math.hypot(a.position[0] - b.position[0], a.position[1] - b.position[1],
      a.position[2] - b.position[2]);
    if (gap > t) return { ok: false, why: "ports " + gap.toFixed(4) + " m apart", gap };
    const dot = a.forward[0] * b.forward[0] + a.forward[1] * b.forward[1] + a.forward[2] * b.forward[2];
    if (dot > -1 + 1e-4) return { ok: false, why: "ports do not face each other", gap, dot };
    return { ok: true, why: "", gap, dot };
  }

  // ---------------------------------------------------------------- the run
  /// Lay a run of pieces along a polyline. THE PATH IS A GUIDE AND NOT A
  /// PROMISE, and the return value says by how much: a kit has the deflections
  /// it has, so a 37 degree corner is built from 30 plus 5 and two degrees of
  /// heading are left over, and a 61 m leg is two 25 m straights plus an 11 m
  /// filler. `deviation` is measured against the path that was asked for, not
  /// against the one the pieces produced, because the second number would
  /// always be zero and would tell a caller nothing.
  function assemble(spec) {
    const s = spec && typeof spec === "object" ? spec : {};
    const netKey = Object.prototype.hasOwnProperty.call(NETWORKS, s.network) ? s.network : "road";
    const net = NETWORKS[netKey];
    const lod = normaliseLod(s.lod);
    const raw = Array.isArray(s.path) ? s.path : [];
    const pts = [];
    for (let i = 0; i < raw.length; i += 1) {
      const p = raw[i];
      if (!Array.isArray(p) || p.length < 2) continue;
      const x = Number(p[0]), z = Number(p.length >= 3 ? p[2] : p[1]);
      const y = p.length >= 3 ? Number(p[1]) : 0;
      if (!isFinite(x) || !isFinite(y) || !isFinite(z)) continue;
      if (pts.length && Math.hypot(x - pts[pts.length - 1][0], z - pts[pts.length - 1][2]) < 1e-6) continue;
      pts.push([x, y, z]);
    }
    if (pts.length < 2) pts.splice(0, pts.length, [0, 0, 0], [0, 0, net.segment]);
    // SEAT THE PATH ON ITS OWN LOWEST POINT. `base` is the height of the
    // formation above natural ground and cannot be negative — the cross-section
    // draws fill, not a cutting — so a path that descends below where it
    // started used to hand `put` a negative base, watch `options` clamp it to
    // zero, and place every following piece against a frame built for the
    // unclamped value. The result was a run that STEPPED: a 300 m descent of
    // 30 m reported a 2.5 m gap at all twelve seams and `seamsOk` false, on
    // every network. Subtracting the datum here makes the whole run fill above
    // its lowest point, which is the same road, and is reported rather than
    // hidden: a caller that asked to end 30 m down gets a mesh whose start is
    // 30 m up and `datum` saying so.
    let datum = Infinity;
    for (let i = 0; i < pts.length; i += 1) if (pts[i][1] < datum) datum = pts[i][1];
    if (datum !== 0) for (let i = 0; i < pts.length; i += 1) pts[i][1] -= datum;

    const legs = [];
    for (let i = 0; i + 1 < pts.length; i += 1) {
      const dx = pts[i + 1][0] - pts[i][0], dz = pts[i + 1][2] - pts[i][2];
      const len = Math.hypot(dx, dz);
      legs.push({ len, hx: dx / len, hz: dz / len, dy: pts[i + 1][1] - pts[i][1] });
    }
    // Corner plans: which bends, which way, and how much tangent each eats out
    // of the legs either side of it.
    const corners = [];
    let headingResidual = 0;
    for (let i = 1; i < legs.length; i += 1) {
      const a = legs[i - 1], b = legs[i];
      const cross = a.hz * b.hx - a.hx * b.hz;
      const dot = a.hx * b.hx + a.hz * b.hz;
      const want = Math.atan2(Math.abs(cross), dot) / DEG;
      const hand = cross >= 0 ? 1 : -1;
      const list = PIECES[net.bend].deflections.slice().sort((p, q) => q - p);
      const turns = [];
      let rem = want;
      for (let k = 0; k < list.length; k += 1) {
        while (rem >= list[k] - 1e-9 && turns.length < 48) { turns.push(list[k]); rem -= list[k]; }
      }
      const placed = turns.reduce((t, v) => t + v, 0);
      headingResidual += rem;
      corners.push({ hand, turns, placed, want, tangent: net.radius * Math.tan(placed * DEG / 2) });
    }

    const m = new Mesh();
    const placed = [];
    const seams = [];
    let maxGap = 0, lengthResidual = 0;
    // The run starts from a synthetic EXIT port at the first path point — an
    // exit port's forward points along travel, the same as every real `out`
    // port — so the FIRST piece is placed by exactly the same mating arithmetic
    // as every piece after it. A start placed some other way is a start whose
    // seam contract was never tested.
    let cursor = {
      id: "origin", network: netKey, position: pts[0].slice(),
      forward: [legs[0].hx, 0, legs[0].hz], up: [0, 1, 0], width: 0, gauge: 0,
    };
    let height = pts[0][1];
    let frame = IDENT;

    function put(name, opts) {
      // The entry port of every piece is [0, base, 0], so the frame that mates
      // it has to know the base the piece is about to be built with. Deriving
      // it here rather than after the previous piece is what makes a graded run
      // join: at base zero the two are the same and the defect hides.
      frame = mateFrame(cursor, { position: [0, opts.base || 0, 0], forward: IN_FWD });
      m.prefix = "segment " + placed.length + " / ";
      m.pushFrame(frame);
      const drawn = drawPiece(m, name, opts);
      m.pop();
      m.prefix = "";
      const world = drawn.ports.map((p) => placePort(frame, p));
      const entry = world[0];
      if (placed.length) {
        const fit = compatible(cursor, entry, 0.001);
        const gap = fit.gap == null ? 0 : fit.gap;
        if (gap > maxGap) maxGap = gap;
        seams.push({ at: entry.position.slice(), gap, ok: fit.ok, why: fit.why });
      }
      const exit = world[1];
      placed.push({
        piece: drawn.key, index: placed.length, at: entry.position.slice(),
        turn: drawn.spec.deflections ? drawn.o.turn : 0,
        hand: drawn.o.hand, length: drawn.o.length, rise: drawn.o.rise,
        triangleCount: m.pos.length / 9,
      });
      cursor = exit;
      height = exit.position[1];
    }

    for (let i = 0; i < legs.length; i += 1) {
      const inTan = i > 0 ? corners[i - 1].tangent : 0;
      const outTan = i < corners.length ? corners[i].tangent : 0;
      const usable = legs[i].len - inTan - outTan;
      const whole = Math.max(0, Math.floor(usable / net.segment + 1e-9));
      const rest = usable - whole * net.segment;
      const filler = rest >= 4 ? rest : 0;
      if (rest > 0 && !filler) lengthResidual += rest;
      if (usable < 0) lengthResidual += -usable;
      const total = whole * net.segment + filler;
      for (let k = 0; k < whole; k += 1) {
        put(net.straight, {
          lod, index: placed.length, length: net.segment,
          base: height, rise: total > 0 ? legs[i].dy * (net.segment / total) : 0,
        });
      }
      if (filler) {
        put(net.straight, {
          lod, index: placed.length, length: filler,
          base: height, rise: total > 0 ? legs[i].dy * (filler / total) : 0,
        });
      }
      if (i < corners.length) {
        for (const turn of corners[i].turns) {
          put(net.bend, { lod, index: placed.length, turn, hand: corners[i].hand, base: height });
        }
      }
    }

    const endpoint = placed.length ? cursor.position : pts[0];
    const target = pts[pts.length - 1];
    const deviation = Math.hypot(endpoint[0] - target[0], endpoint[1] - target[1], endpoint[2] - target[2]);
    let pathLength = 0;
    for (const leg of legs) pathLength += leg.len;

    return m.finish({
      network: netKey,
      lod,
      ports: placed.length
        ? [
          { id: "start", network: netKey, position: pts[0].slice(), forward: [-legs[0].hx, 0, -legs[0].hz],
            up: [0, 1, 0], width: cursor.width, gauge: cursor.gauge },
          { id: "end", network: netKey, position: endpoint.slice(), forward: cursor.forward.slice(),
            up: [0, 1, 0], width: cursor.width, gauge: cursor.gauge },
        ] : [],
      run: {
        pieces: placed,
        segmentCount: placed.length,
        seamCount: seams.length,
        maxSeamGap: maxGap,
        seamsOk: seams.every((seam) => seam.ok),
        deviation,
        headingResidualDeg: headingResidual,
        lengthResidual,
        pathLength,
        // What was subtracted from every path height to seat the run. Zero for
        // a level run or one that only climbs; the depth of the descent for one
        // that goes down. Add it back to read a port in the caller's own frame.
        datum,
        corners: corners.map((c) => ({ want: c.want, placed: c.placed, hand: c.hand, turns: c.turns.slice() })),
      },
      description: "A " + Math.round(pathLength) + " m " + netKey + " run of " + placed.length
        + " kit segments at " + lod + " detail, laid along a polyline by the endpoint contract. "
        + "The path is a guide, not a promise: the kit has a fixed set of deflections and a fixed "
        + "segment length, so this run ends " + deviation.toFixed(2) + " m from the point asked for "
        + "with " + headingResidual.toFixed(2) + " degrees of heading unspent."
        + (datum !== 0 ? " The path descends below where it starts, so the run is seated on its"
          + " lowest point and every height in it reads " + (-datum).toFixed(2)
          + " m higher than the one asked for." : "")
        + " Representative "
        + "scenery: it asserts nothing about whether a route exists here, what it carries or what "
        + "it cost.",
    });
  }

  // ------------------------------------------------------------------- api
  function pieces() { return PIECE_KEYS.slice(); }
  function info(name) {
    const spec = PIECES[name];
    if (!spec) return null;
    return {
      piece: name, label: spec.label, network: spec.network, blurb: spec.blurb,
      straight: !!spec.straight,
      segmentLength: spec.straight ? spec.length : null,
      maxLength: spec.straight ? (spec.maxLength || 60) : null,
      deflections: spec.deflections ? spec.deflections.slice() : null,
      radius: spec.radius == null ? null : spec.radius,
      structure: !!spec.structure,
      atGrade: !!spec.atGrade,
      deckHeight: spec.structure ? spec.base : null,
      electrified: !!spec.electrified,
      modes: spec.modes ? ["road", "rail"] : null,
    };
  }
  function networks() { return Object.keys(NETWORKS); }

  return Object.freeze({
    build, assemble, pieces, info, networks,
    mateFrame, placePort, compatible, compose, apply, direction, yawTaking,
    segmentLength: SEG,
    gauge: GAUGE,
    lods: ["near", "map"],
    palette: P,
    crease: CREASE,
  });
});
