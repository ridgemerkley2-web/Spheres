/* Construction sites as geometry: one staged model for every project kind in
   `spheres-sim/src/production.rs::PROJECT_KINDS`.

   WHY THIS EXISTS. The construction queue is the most legible thing the
   simulation does — a province spends real days and real commodities and gets a
   capability at the end of it — and the browser has been showing that as a
   progress bar. This file turns the same record into something you can look at:
   what is standing, what is still a hole in the ground, and whether anybody is
   working today.

   THE STAGE IS A PURE FUNCTION OF SERVER PROGRESS, and that is the whole
   contract. production.rs has no stage, phase or milestone concept. It has
   `progress_days` over `total_days`, and a status recomputed at serialization
   (Building | Slowed | Paused | Blocked). So the thresholds live here, they read
   only that record, and there is no clock in this file of any kind — no wall
   clock, no frame counter, no elapsed-seconds animation. A site left open in a
   browser tab for an hour is exactly as built as when it opened, and a paused
   site shows parked plant instead of a crane swinging over work nobody is
   paying for. That is the honest reading and it is also the only one the sim
   can support.

   WHY BUILT AND NOT LOADED. Same reason as arsenal-models.js: no build step, no
   CDN, no third-party runtime (CLAUDE.md). Sixteen kinds at five stages is
   eighty meshes; as source they are one file, and the stages share one kit
   rather than being five unrelated copies per kind.

   WHY NO DOM IN HERE. Geometry is the half that has to be right. It is checked
   under node by `tools/ui/check_site_mesh.cjs`, which cannot run a browser and
   should not have to. Whoever draws this owns the GL context and imports
   nothing but the buffers `build` returns.

   MODEL SPACE. Metres. +X starboard, +Y up, +Z forward. Y=0 is ground contact —
   the underside of the formation platform every site sits on, and grade is
   `GRADE` above that, because a real site is built up on fill before anything
   else happens and a foundation pit has to have somewhere to go. No shared
   vertices, no index buffer, per-vertex RGB in 0..1 with no alpha, no UVs and no
   textures — this renderer has no texture path at all, so everything you can see
   here is geometry or vertex colour, which is why a rib, a chamfer and a louvre
   slat are all drawn rather than painted.

   SHADING. Normals are per triangle EXCEPT inside a smoothing group, where the
   faces meeting at a corner are averaged under a crease limit. Curved
   primitives — every loft, tube, column, rod and heap — open a group and come
   out round; plate, sheeting, panelling and concrete never do and keep their
   exact face normal. It costs no triangles and it is the single largest thing
   separating this from the faceted prisms it used to draw.

   TRUTHFULNESS (roadmap section 3). These are ORIGINAL generic industrial
   designs. Nothing here is a real named plant, no dimension is a measured
   dimension of anything real, and the art grants the province no capability the
   simulation has not already recorded. `generation` in particular stays GENERIC:
   a machine hall, a conversion block, heat rejection and consumables handling,
   with no named technology anywhere in it, because roadmap section E puts
   technology-specific variants in a later pass and this one is not entitled to
   invent one and present it as sourced.

   ALL SIXTEEN HAVE A COMPOSITION. `arms_plant` has a bespoke builder; the
   other fifteen have a function each in KIND_WORKS, built to what section E says
   that kind IS — a corridor and a structure over it, three factory modules on
   one grid, a switchyard, a laboratory campus, two machine bays under a
   travelling crane, a generic generation facility, an open process structure, a
   rail-and-road transfer yard, a racked high bay, and the two RETROFITS, which
   are installed into a host facility that already stands rather than being new
   standalone buildings. They still share one stage kit — hoarding, huts,
   materials, crane, scaffold, footings, cladding, roof, fence, lighting —
   because section E asks for exactly that, and because those are the same
   objects on all these sites. Offices add a framed service block, shipyards a
   dock and lifting gantry, and advanced industry a controlled assembly hall.
   `meta(key).placeholder` is a per-kind fact and it is false on all sixteen;
   the check asserts the count, so a kind
   cannot be quietly marked finished without one. */
(function (root, factory) {
  const api = factory();
  if (typeof module === "object" && module.exports) module.exports = api;
  else root.SiteMesh = api;
})(typeof globalThis !== "undefined" ? globalThis : this, function () {
  "use strict";

  const DEG = Math.PI / 180;

  /// Grade. Everything a site does happens on a made-up platform, and the
  /// platform is what touches Y=0. Without it an excavation would have to go
  /// negative, which would either break `bounds.min[1] === 0` or shift the whole
  /// model up by the depth of its deepest hole — so the building would sit at a
  /// different height at every stage. 0.6 m reads as a hardstand plinth when the
  /// site is finished and gives the dig somewhere to be while it is not.
  const GRADE = 0.6;
  const PIT = 0.05;

  // ---------------------------------------------------------------- matrices
  // Row-major 4x4, v' = M v, same convention and the same reason as
  // arsenal-models.js: the only reader is `xf` and it is easier to check by eye.
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
  function rgb(hex) {
    return [((hex >> 16) & 255) / 255, ((hex >> 8) & 255) / 255, (hex & 255) / 255];
  }
  function shade(c, k) {
    return [
      Math.max(0, Math.min(1, c[0] * k)),
      Math.max(0, Math.min(1, c[1] * k)),
      Math.max(0, Math.min(1, c[2] * k)),
    ];
  }

  /// The site palette. Deliberately drab: earth, fill, concrete, primed steel
  /// and sheet cladding are what a building site is, and the only saturated
  /// things on one are the safety colours and the hoarding. Industry accents
  /// live per kind in KINDS and are used sparingly — a factory should read as a
  /// factory, not as a pavilion (roadmap section 3).
  const P = {
    earth: rgb(0x6b5c46),
    earthCut: rgb(0x50442f),
    fill: rgb(0x8b8478),
    hardcore: rgb(0x9c968a),
    concrete: rgb(0xb4b0a6),
    concreteWet: rgb(0x8f8c85),
    kerb: rgb(0xc2beb4),
    asphalt: rgb(0x4b4b4d),
    lane: rgb(0xc8c29a),
    steel: rgb(0x717c86),
    primer: rgb(0x9c5c38),        // red-oxide primed frame, before it is clad
    galv: rgb(0xa2a9ae),
    clad: rgb(0xa9b0b4),
    roof: rgb(0x798186),
    glass: rgb(0x3d5f74),
    door: rgb(0x5e6a72),
    dark: rgb(0x33383c),
    timber: rgb(0x8d7247),
    rebar: rgb(0x6d6a63),
    hoard: rgb(0x2f5d7a),         // painted site hoarding
    hut: rgb(0xc7c3b6),
    safety: rgb(0xb8862c),        // plant and barriers
    stopped: rgb(0x9d4a34),       // the honest overlay's colour, on the site
    grass: rgb(0x51603f),
  };

  /// How far two faces may disagree and still share a corner normal. cos 69.5°.
  /// WHY A LIMIT AT ALL: a smooth cylinder still has a sharp rim, a spoil heap
  /// still has a hard edge where its side meets the ground, and a smoothing pass
  /// that ignores that turns every drum into a blob. At 0.35 the ring of an
  /// 8-segment tube merges (adjacent faces are 45° apart, dot 0.707) and its end
  /// cap does not (dot 0), which is exactly the split wanted.
  ///
  /// It also PROVES the shading contract the checks assert. Every face folded
  /// into a corner is within the limit of that corner's own face, so the sum has
  /// dot >= 0.35 * count with it and the normalised sum has dot >= 0.35. A
  /// smoothed vertex can therefore never point behind the triangle it belongs
  /// to, which is the property the old flat-normal assertion was really
  /// defending.
  const CREASE = 0.35;

  // ------------------------------------------------------------------- build
  // A mesh is a triangle soup with a per-vertex colour, a transform stack and a
  // list of named part ranges. Normals are NOT accumulated at emit time: they
  // are derived per triangle in `finish` from the vertices AS TRANSFORMED, so a
  // mirrored wall gets the normal its geometry actually has. Mirroring is how
  // half the walls here are placed, so that is worth the two lines it costs.
  //
  // SMOOTH SHADING, and why it is the first thing this pass did. Every triangle
  // in this file used to take the normal of the face it sat on, so a tank, a
  // pipe, a bollard, a drum and a spoil heap all read as faceted prisms however
  // many segments they were given. Smoothing costs ZERO triangles. Curved
  // primitives now open a SMOOTHING GROUP, and `finish` averages the face
  // normals that meet at each corner inside one, crease-limited so rims and
  // caps stay sharp. Flat plate, concrete, panelling and sheeting never open a
  // group and keep the exact face normal they always had.
  function Mesh() {
    this.pos = [];
    this.col = [];
    this.grp = [];
    this.parts = [];
    this.m = IDENT.slice();
    this.stack = [];
    this.sg = 0;
    this.sgNext = 0;
  }
  /// Everything emitted inside `fn` shares one smoothing group. A FRESH id per
  /// call, deliberately: two drums standing shoulder to shoulder touch at their
  /// skins, and a shared group would weld their normals into one lumpy surface.
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
  Mesh.prototype.tri = function (a, b, c, col) {
    const flip = this.detSign() < 0;
    const A = this.xf(a), B = this.xf(flip ? c : b), C = this.xf(flip ? b : c);
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
  Mesh.prototype.both = function (fn) {
    fn(this, 1);
    this.save().scale(-1, 1, 1);
    fn(this, -1);
    this.restore();
    return this;
  };

  /// Named ranges, in the same shape equipment-mesh.js uses, because the picker
  /// and the card renderer already speak it. A part that draws nothing is not
  /// recorded — an empty range would break the contiguity the checks assert, and
  /// a stage that has no crane should not carry a crane entry.
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

  // -------------------------------------------------------------- primitives
  /// A box by extents, not by centre and size: nearly everything on a site is
  /// positioned by the face it sits flush against — a slab top, a wall line, a
  /// platform edge — and extents say that without arithmetic at the call site.
  Mesh.prototype.bar = function (x0, x1, y0, y1, z0, z1, col) {
    const t = shade(col, 1.07), s = shade(col, 0.9), u = shade(col, 0.76);
    this.quad([x1, y0, z0], [x1, y1, z0], [x1, y1, z1], [x1, y0, z1], s);
    this.quad([x0, y0, z0], [x0, y0, z1], [x0, y1, z1], [x0, y1, z0], s);
    this.quad([x0, y1, z0], [x0, y1, z1], [x1, y1, z1], [x1, y1, z0], t);
    this.quad([x0, y0, z0], [x1, y0, z0], [x1, y0, z1], [x0, y0, z1], u);
    this.quad([x0, y0, z1], [x1, y0, z1], [x1, y1, z1], [x0, y1, z1], col);
    this.quad([x0, y0, z0], [x0, y1, z0], [x1, y1, z0], [x1, y0, z0], u);
    return this;
  };
  Mesh.prototype.box = function (cx, cy, cz, sx, sy, sz, col) {
    return this.bar(cx - sx / 2, cx + sx / 2, cy - sy / 2, cy + sy / 2, cz - sz / 2, cz + sz / 2, col);
  };

  /// The shading `bar` gives its six faces, as a function of an arbitrary
  /// outward normal, so a chamfer or a folded sheet lands on the same key as the
  /// box beside it. Top 1.07, bottom 0.76, front 1.00, back 0.76, sides 0.90 —
  /// read straight off `bar` so the two cannot drift apart.
  /// The 0.02 on nx is deliberate and small: it gives the two long faces of a
  /// section slightly different values so a beam does not read as symmetrical
  /// under a light that is not.
  function faceTone(nx, ny, nz) {
    return 0.90 + 0.17 * Math.max(0, ny) - 0.14 * Math.max(0, -ny)
      + 0.10 * Math.max(0, nz) - 0.14 * Math.max(0, -nz) + 0.02 * nx;
  }

  /// A CHAMFERED box: the same extents as `bar`, with the four edges parallel to
  /// its longest axis cut back by `ch`.
  ///
  /// WHY THIS EXISTS AND WHY IT IS WORTH 8 EXTRA TRIANGLES. Nothing manufactured
  /// has a knife edge. A rolled section, a kerb, a concrete upstand, a ballast
  /// block all carry a small arris, and that arris is what catches the light and
  /// separates a machined object from a folded piece of card — in a renderer
  /// with no textures it is most of what "machined" even means. 20 mm is the
  /// house default: enough to catch a highlight at inspection range, small
  /// enough to vanish honestly at map range, which is why the coarse path never
  /// calls this. The ends are left square because they are almost always buried
  /// in another member or standing on the ground.
  Mesh.prototype.beam = function (x0, x1, y0, y1, z0, z1, col, ch0) {
    const dx = x1 - x0, dy = y1 - y0, dz = z1 - z0;
    // Along the longest axis, because that is the edge a section is extruded
    // along and the one a viewer reads as its length.
    const axis = dx >= dy && dx >= dz ? 0 : (dy >= dz ? 1 : 2);
    const u = axis === 0 ? dy : dx, v = axis === 2 ? dy : dz;
    const ch = Math.max(0.004, Math.min(ch0 == null ? 0.02 : ch0, u * 0.34, v * 0.34));
    // The octagonal section, counter-clockwise in the two axes that are not the
    // extrusion axis. `a` is the first of them, `b` the second.
    const a0 = axis === 0 ? y0 : x0, a1 = axis === 0 ? y1 : x1;
    const b0 = axis === 2 ? y0 : z0, b1 = axis === 2 ? y1 : z1;
    const sec = [
      [a0 + ch, b0], [a1 - ch, b0], [a1, b0 + ch], [a1, b1 - ch],
      [a1 - ch, b1], [a0 + ch, b1], [a0, b1 - ch], [a0, b0 + ch],
    ];
    // A section listed counter-clockwise in (a, b) winds the wrong way when the
    // extrusion axis is Y, because (x, z) seen from +Y is the mirror of the
    // plane the list was written in. Reversing it once here fixes both the side
    // quads and the two caps, and `sgn` keeps the shading normal with them.
    const sgn = axis === 1 ? -1 : 1;
    if (axis === 1) sec.reverse();
    const e0 = axis === 0 ? x0 : (axis === 1 ? y0 : z0);
    const e1 = axis === 0 ? x1 : (axis === 1 ? y1 : z1);
    const at = (a, b, e) => (axis === 0 ? [e, a, b] : axis === 1 ? [a, e, b] : [a, b, e]);
    for (let i = 0; i < 8; i += 1) {
      const j = (i + 1) % 8;
      const na = sec[j][1] - sec[i][1], nb = sec[i][0] - sec[j][0];
      const nl = Math.hypot(na, nb) || 1;
      const n = at((sgn * na) / nl, (sgn * nb) / nl, 0);
      this.quad(at(sec[i][0], sec[i][1], e0), at(sec[j][0], sec[j][1], e0),
        at(sec[j][0], sec[j][1], e1), at(sec[i][0], sec[i][1], e1),
        shade(col, faceTone(n[0], n[1], n[2])));
    }
    this.fan(sec.map((p) => at(p[0], p[1], e1)), shade(col, faceTone(...at(0, 0, 1))));
    this.fan(sec.map((p) => at(p[0], p[1], e0)).reverse(), shade(col, faceTone(...at(0, 0, -1))));
    return this;
  };

  function ring(rx, ry, seg) {
    const pts = [];
    for (let i = 0; i < seg; i += 1) {
      const t = (i / seg) * Math.PI * 2;
      pts.push([Math.cos(t) * rx, Math.sin(t) * ry]);
    }
    return pts;
  }
  /// Lofted surfaces are the curved half of this file — pipes, tanks, silos,
  /// drums, bollards, downpipes, reinforcement, scaffold tube, masts — so the
  /// whole loft opens ONE smoothing group and every one of them comes out round
  /// for no triangles at all. The end caps sit in the same group and stay sharp
  /// on the crease limit, which is what a rolled rim actually looks like.
  Mesh.prototype.loft = function (sections, col, caps) {
    this.smooth(() => {
      const n = sections[0].pts.length;
      for (let s = 0; s + 1 < sections.length; s += 1) {
        const a = sections[s], b = sections[s + 1];
        for (let i = 0; i < n; i += 1) {
          const j = (i + 1) % n;
          const lit = 1 + 0.15 * Math.sin((i / n) * Math.PI * 2 + 1.2);
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
        this.fan(l.pts.map((p) => [p[0], p[1], l.z]), shade(col, 0.88));
      }
    });
    return this;
  };
  Mesh.prototype.tube = function (r0, r1, len, seg, col, caps) {
    return this.loft([
      { z: 0, pts: ring(r0 || 1e-4, r0 || 1e-4, seg) },
      { z: len, pts: ring(r1 || 1e-4, r1 || 1e-4, seg) },
    ], col, caps);
  };
  /// A vertical cylinder — stacks, tanks, silos, masts, piles. Standing a tube
  /// on its end is common enough here that the rotation belongs in one place.
  Mesh.prototype.column = function (x, y, z, h, r0, r1, seg, col, caps) {
    this.save().move(x, y, z).rotX(-90);
    this.tube(r0, r1 == null ? r0 : r1, h, seg, col, caps);
    this.restore();
    return this;
  };
  /// A conical heap. Spoil, aggregate, sand. Cheaper than a lofted cone and it
  /// is the right shape: tipped material stands at its angle of repose.
  /// A conical heap. Spoil, aggregate, sand, topsoil, grass verge. Smoothed
  /// around the ring, which also rounds the apex: tipped material does not come
  /// to a point, and the crease limit still holds the hard line where the side
  /// meets the ground.
  Mesh.prototype.mound = function (x, y, z, r, h, seg, col) {
    this.smooth(() => {
      const pts = ring(r, r, seg).map((p) => [x + p[0], y, z + p[1]]);
      for (let i = 0; i < seg; i += 1) {
        const j = (i + 1) % seg;
        // WOUND j-THEN-i. Listed the other way the whole cone is inside out —
        // every spoil heap, topsoil bund, aggregate pile and grass verge in this
        // file was, and it hid because a tall narrow heap's sides are nearly
        // vertical and a nearly vertical face lit from the wrong side is only
        // slightly wrong. Flatten the verge to the 0.55 m it should always have
        // been and it is unmistakable.
        this.tri(pts[j], pts[i], [x, y + h, z], shade(col, 1 + 0.1 * Math.sin(i * 1.3)));
      }
      // The base looks DOWN. It is buried either way, but a base facing up
      // shares the sides' sky-ward normal closely enough to pass the crease
      // limit, and the smoothing pass would then round the one edge on a tipped
      // heap that is genuinely hard.
      this.fan(pts, shade(col, 0.8));
    });
    return this;
  };
  /// A flat planform extruded in Y — slabs, aprons, canopies, gable tympana.
  /// Points are [x, z] and the winding is fixed here rather than asked of the
  /// caller, exactly as in arsenal-models.js: the signed area says which way the
  /// list runs and a clockwise one is reversed before anything is emitted.
  Mesh.prototype.plate = function (pts, thick, col) {
    let area = 0;
    for (let i = 0; i < pts.length; i += 1) {
      const j = (i + 1) % pts.length;
      area += pts[i][0] * pts[j][1] - pts[j][0] * pts[i][1];
    }
    const ordered = area < 0 ? pts.slice().reverse() : pts;
    const top = ordered.map((p) => [p[0], thick, p[1]]);
    const bot = ordered.map((p) => [p[0], 0, p[1]]);
    this.fan(top.slice().reverse(), shade(col, 1.08));
    for (let i = 0; i < ordered.length; i += 1) {
      const j = (i + 1) % ordered.length;
      this.quad(bot[j], bot[i], top[i], top[j], shade(col, 0.9));
    }
    return this;
  };

  /// A flat plate standing in the XY plane between two Z faces: haunches,
  /// gussets, gable tympana, sign boards, dock bumpers. It exists because
  /// hand-winding both faces of a plate is how normals end up inside out — the
  /// signed area of the outline decides the handedness once, here, and both
  /// faces and the rim follow from it.
  Mesh.prototype.webPlate = function (pts, z0, z1, col) {
    let area = 0;
    for (let i = 0; i < pts.length; i += 1) {
      const j = (i + 1) % pts.length;
      area += pts[i][0] * pts[j][1] - pts[j][0] * pts[i][1];
    }
    const o = area < 0 ? pts.slice().reverse() : pts;
    this.fan(o.map((p) => [p[0], p[1], z1]), shade(col, 1.02));
    this.fan(o.map((p) => [p[0], p[1], z0]).reverse(), shade(col, 0.88));
    for (let i = 0; i < o.length; i += 1) {
      const j = (i + 1) % o.length;
      this.quad([o[i][0], o[i][1], z0], [o[j][0], o[j][1], z0],
        [o[j][0], o[j][1], z1], [o[i][0], o[i][1], z1], shade(col, 0.84));
    }
    return this;
  };

  /// A round bar between two points, at any angle. Reinforcement, scaffold
  /// tube, lattice bracing, handrail and pendant ties are all this. Doing it
  /// with a matrix rather than by hand is what lets a brace actually be a brace
  /// instead of an axis-aligned box pretending to be one, and it comes out
  /// smooth-shaded for free because `loft` opens the group.
  Mesh.prototype.rod = function (a, b, r, seg, col, caps) {
    const dx = b[0] - a[0], dy = b[1] - a[1], dz = b[2] - a[2];
    const len = Math.hypot(dx, dy, dz);
    if (len < 1e-6) return this;
    this.save().move(a[0], a[1], a[2])
      .rotY(Math.atan2(dx, dz) / DEG)
      .rotX(-Math.asin(Math.max(-1, Math.min(1, dy / len))) / DEG);
    this.tube(r, r, len, seg, col, caps);
    this.restore();
    return this;
  };

  /// Ground you can tell from the platform it sits on. A site is not a clean
  /// plane: there is hardstanding where the plant runs, churned mud off the
  /// gate, and hardcore where the piling mat was left. With no textures the
  /// ONLY way to say that is to break the surface into faces that differ, so
  /// this lays a grid and varies the tone across it. `wear` is deliberately
  /// small — the brief asks for restrained, and mud that reads as camouflage is
  /// worse than no mud.
  function groundPatch(m, x, z, w, d, nx, nz, y, col, wear) {
    for (let i = 0; i < nx; i += 1) {
      for (let j = 0; j < nz; j += 1) {
        const x0 = x - w / 2 + (i * w) / nx, x1 = x - w / 2 + ((i + 1) * w) / nx;
        const z0 = z - d / 2 + (j * d) / nz, z1 = z - d / 2 + ((j + 1) * d) / nz;
        const t = 1 + wear * (0.6 * Math.sin(i * 1.7 + j * 2.3) + 0.4 * Math.sin(i * 0.62 - j * 1.13));
        deck(m, x0, x1, y, z0, z1, shade(col, t));
      }
    }
  }

  /// Profiled sheeting, drawn as the fold it actually is. Trapezoidal cladding
  /// is a continuous zig-zag of crest, web, valley, web, and every one of those
  /// facets takes a different amount of light — which is precisely the read this
  /// renderer can afford and a flat quad cannot fake. Spans x in [x0, x1], y in
  /// [0, h], sheeting the +Z face.
  function ribbedPanel(m, x0, x1, h, pitch, depth, col) {
    const span = x1 - x0;
    const ribs = Math.max(1, Math.round(span / pitch));
    const step = span / ribs;
    // crest 0.30, fall 0.20, valley 0.30, rise 0.20 of each rib
    const profile = [[0, 0], [0.30, 0], [0.50, depth], [0.80, depth], [1.0, 0]];
    for (let r = 0; r < ribs; r += 1) {
      for (let s = 0; s + 1 < profile.length; s += 1) {
        const ax = x0 + (r + profile[s][0]) * step, az = profile[s][1];
        const bx = x0 + (r + profile[s + 1][0]) * step, bz = profile[s + 1][1];
        const fl = Math.hypot(bz - az, bx - ax) || 1;
        m.quad([ax, 0, az], [bx, 0, bz], [bx, h, bz], [ax, h, az],
          shade(col, faceTone((az - bz) / fl, 0, (bx - ax) / fl)));
      }
    }
  }

  /// A louvre bank with slats and a hole behind them. WHY NOT A DARK RECTANGLE:
  /// a dark rectangle is what a texture would be for, and there is no texture
  /// path here. Ventilation openings are one of the few places an industrial
  /// building has real depth on its elevation, so they get real depth.
  function louvreBank(m, x0, x1, y0, y1, z, out, slats, col) {
    m.bar(x0, x1, y0, y1, z - 0.02, z + 0.03, shade(P.dark, 0.55));            // the opening
    for (let i = 0; i < slats; i += 1) {
      const y = y0 + 0.06 + (i * (y1 - y0 - 0.12)) / slats;
      const t = (y1 - y0 - 0.12) / slats;
      // Tilted down and out, so the top face catches sky and the underside does
      // not — the alternation is the whole silhouette of a louvre.
      m.quad([x0, y + t * 0.75, z], [x1, y + t * 0.75, z], [x1, y, z + out], [x0, y, z + out], shade(col, 0.66));
      m.quad([x0, y, z + out], [x1, y, z + out], [x1, y + t * 0.75, z], [x0, y + t * 0.75, z], shade(col, 1.08));
    }
    m.bar(x0 - 0.08, x0 + 0.02, y0 - 0.06, y1 + 0.06, z - 0.02, z + out + 0.04, shade(col, 0.86));
    m.bar(x1 - 0.02, x1 + 0.08, y0 - 0.06, y1 + 0.06, z - 0.02, z + out + 0.04, shade(col, 0.86));
    m.bar(x0 - 0.08, x1 + 0.08, y1 + 0.02, y1 + 0.14, z - 0.02, z + out + 0.1, shade(col, 1.1));
  }

  /// A reinforcement cage that reads as a cage: vertical bars at the corners and
  /// on the faces, closed links at four levels, and starters left projecting for
  /// the column that lands on it. Round bar, because it is round, and because
  /// the smoothing pass makes that free.
  function rebarCage(m, x, z, hw, hd, y0, y1, seg) {
    const r = 0.028;
    const verts = [
      [-hw, -hd], [0, -hd], [hw, -hd], [hw, 0],
      [hw, hd], [0, hd], [-hw, hd], [-hw, 0],
    ];
    for (const v of verts) {
      m.rod([x + v[0], y0, z + v[1]], [x + v[0], y1, z + v[1]], r, seg, P.rebar, false);
    }
    for (let l = 0; l < 4; l += 1) {
      const y = y0 + 0.16 + (l * (y1 - y0 - 0.32)) / 3;
      const link = [[-hw, -hd], [hw, -hd], [hw, hd], [-hw, hd]];
      for (let i = 0; i < 4; i += 1) {
        const a = link[i], b = link[(i + 1) % 4];
        m.rod([x + a[0], y, z + a[1]], [x + b[0], y, z + b[1]], r * 0.8, 6, shade(P.rebar, 1.08), false);
      }
    }
  }

  /// A lattice bay: four chords already standing, this adds the horizontals and
  /// the K-bracing that make a mast or a jib a lattice rather than a stick. The
  /// crane and the pipe gantry both want it, so it lives out here.
  function latticeBay(m, y0, y1, hx, hz, r, seg, col) {
    const ym = (y0 + y1) / 2;
    for (const s of [-1, 1]) {
      m.rod([-hx, y1, s * hz], [hx, y1, s * hz], r, seg, col, false);
      m.rod([-hx, y0, s * hz], [0, ym, s * hz], r * 0.85, seg, shade(col, 0.9), false);
      m.rod([0, ym, s * hz], [hx, y0, s * hz], r * 0.85, seg, shade(col, 0.9), false);
      m.rod([s * hx, y1, -hz], [s * hx, y1, hz], r, seg, col, false);
      m.rod([s * hx, y0, -hz], [s * hx, ym, 0], r * 0.85, seg, shade(col, 0.88), false);
      m.rod([s * hx, ym, 0], [s * hx, y0, hz], r * 0.85, seg, shade(col, 0.88), false);
    }
  }

  /// A bollard: shaft, reflective band and a domed cap. Three tubes rather than
  /// one because the band is the thing that says "bollard" and not "post", and
  /// the smoothing pass rounds all three for nothing.
  function bollard(m, x, z, h, r, seg, col) {
    m.column(x, GRADE, z, h * 0.62, r, r, seg, col, true);
    m.column(x, GRADE + h * 0.62, z, h * 0.16, r * 1.04, r * 1.04, seg, shade(P.hut, 1.06), false);
    m.column(x, GRADE + h * 0.78, z, h * 0.16, r, r, seg, col, false);
    m.column(x, GRADE + h * 0.94, z, h * 0.06, r, r * 0.55, seg, shade(col, 1.1), true);
  }

  // ------------------------------------------------------------------ stages
  // Five stages, and which one a site is in is a pure function of recorded work.
  // The thresholds are art decisions and live here; the sim is not asked to
  // carry them and would refuse if it were (iron rule 2). `complete` is reached
  // only at full progress because a completed project LEAVES the queue and lives
  // on as a capability — stage five is the hand-off state, not a late stage of
  // building.
  const STAGE_TABLE = [
    { key: "site", name: "Site establishment", from: 0.0 },
    { key: "foundation", name: "Foundations", from: 0.12 },
    { key: "frame", name: "Structural frame", from: 0.34 },
    { key: "enclosed", name: "Enclosed and commissioning", from: 0.62 },
    { key: "complete", name: "Complete", from: 1.0 },
  ];
  const STAGE_INDEX = {};
  STAGE_TABLE.forEach((s, i) => { STAGE_INDEX[s.key] = i; });

  const STAGE_WORK = [
    "hoarding up, ground stripped, huts and materials on site",
    "excavation open, pad footings, reinforcement and formwork",
    "steel frame erected, purlins and bracing, crane in service",
    "roof and cladding going on, services and commissioning",
    "handed over and in service",
  ];

  const STATUSES = ["building", "slowed", "paused", "blocked"];
  function normStatus(v) {
    const s = String(v == null ? "building" : v).toLowerCase();
    return STATUSES.indexOf(s) < 0 ? "building" : s;
  }
  function clamp01(v) { return v < 0 ? 0 : v > 1 ? 1 : v; }

  /// Progress in, stage out, and nothing else in between. Accepts the three
  /// shapes a caller actually has: a fraction, a stage key, or the project
  /// record `production_project_json` already serves (progress_days /
  /// total_days, plus status). A stage key is allowed because a catalogue or a
  /// gallery wants to show all five without inventing fractions for them.
  function stageFor(kindKey, input, opts) {
    const o = opts || {};
    let progress = null, forced = null, status = o.status;
    if (typeof input === "number" && isFinite(input)) progress = clamp01(input);
    else if (typeof input === "string") forced = input;
    else if (input && typeof input === "object") {
      if (typeof input.stage === "string") forced = input.stage;
      else if (typeof input.stage === "number" && isFinite(input.stage)) {
        forced = STAGE_TABLE[Math.max(0, Math.min(4, Math.round(input.stage)))].key;
      }
      if (progress == null && typeof input.total_days === "number" && input.total_days > 0
        && typeof input.progress_days === "number" && isFinite(input.progress_days)) {
        progress = clamp01(input.progress_days / input.total_days);
      }
      if (progress == null && typeof input.progress === "number" && isFinite(input.progress)) {
        progress = clamp01(input.progress);
      }
      if (status == null) status = input.status;
    }
    let index;
    if (forced != null && Object.prototype.hasOwnProperty.call(STAGE_INDEX, forced)) {
      index = STAGE_INDEX[forced];
      if (progress == null) progress = index === 4 ? 1 : STAGE_TABLE[index].from;
    } else {
      if (progress == null) progress = 0;
      index = 0;
      for (let i = 0; i < STAGE_TABLE.length; i += 1) {
        if (progress + 1e-9 >= STAGE_TABLE[i].from) index = i;
      }
    }
    const row = STAGE_TABLE[index];
    const st = normStatus(status);
    return {
      kind: KINDS[kindKey] ? kindKey : FALLBACK_KIND,
      index,
      stage: row.key,
      name: row.name,
      from: row.from,
      to: index + 1 < STAGE_TABLE.length ? STAGE_TABLE[index + 1].from : 1,
      progress,
      status: st,
      // Slowed work is still work. Paused and blocked are not, and the site has
      // to say so rather than let a viewer read a moving-looking scene as
      // progress that is not happening.
      active: st === "building" || st === "slowed",
    };
  }

  function stages(kindKey) {
    const k = KINDS[kindKey] || KINDS[FALLBACK_KIND];
    return STAGE_TABLE.map((row, i) => ({
      index: i,
      key: row.key,
      name: row.name,
      from: row.from,
      to: i + 1 < STAGE_TABLE.length ? STAGE_TABLE[i + 1].from : 1,
      work: i === 4 ? `${k.name.toLowerCase()} in service` : STAGE_WORK[i],
    }));
  }

  // ------------------------------------------------------------------- kinds
  // One row per key in PROJECT_KINDS. Every one of the sixteen carries a
  // composition — `arms_plant` a bespoke builder, the other fifteen a function
  // in KIND_WORKS — and this table holds only what a composition is placed
  // against: the compound, the lead block, the two colours and the sentence the
  // card prints.
  //
  // `pad` is the compound at province level one, and it grows with the level.
  // `block` is the LEAD building, and for most of these it is not the whole
  // facility: it is a small control building on `power_grid`, a link annex on
  // `automation`, a depot shed on `infrastructure`, and the high bay itself
  // only on `warehouse`. `home` seats it where its own composition needs the
  // room. `grid` is the footing pattern under it, which has to suit the block
  // rather than being four by three whatever is standing on it.
  //
  // The accents are what makes an industry legible at map zoom without turning
  // a province into a theme park (roadmap section 3): a stack, a tank farm, a
  // container stack, a pylon — the thing you would actually recognise from a
  // distance, and not much else.
  const FALLBACK_KIND = "civilian_industry";
  const KINDS = {
    infrastructure: {
      name: "Infrastructure works",
      pad: [64, 46], home: [-0.10, 0.20], grid: [3, 2],
      block: { w: 20, dz: 12, eaves: 6.6, ridge: 8.0, bays: 8, ends: 5 },
      body: rgb(0xa7ada8), accent: rgb(0xb8862c),
      blurb: "Carriageway, a structure carrying it, a duct route, and the depot beside them",
    },
    civilian_industry: {
      name: "Civilian industry",
      pad: [66, 46], home: [-0.22, 0.06], grid: [4, 3],
      block: { w: 16, dz: 17, eaves: 8.5, ridge: 10.6, bays: 7, ends: 7 },
      body: rgb(0xa9b0b4), accent: rgb(0x5c7f8c),
      blurb: "Three production modules on one grid, linked at high level, with the service yard behind",
    },
    power_grid: {
      name: "Power grid",
      pad: [66, 48], home: [-0.34, 0.24], grid: [2, 2],
      block: { w: 14, dz: 10, eaves: 5.4, ridge: 6.4, bays: 6, ends: 4 },
      body: rgb(0xbcbdb6), accent: rgb(0x8b9096),
      blurb: "A stone-surfaced switchyard with busbar gantries, transformer bays and a small control building",
    },
    research_center: {
      name: "Research centre",
      pad: [66, 46], home: [-0.28, 0.08], grid: [3, 2],
      block: { w: 20, dz: 14, eaves: 7.4, ridge: 8.6, bays: 9, ends: 6 },
      body: rgb(0xc3bfb2), accent: rgb(0x46708a),
      blurb: "A framed laboratory block and a workshop, linked and set round a courtyard",
    },
    arms_plant: {
      name: "Arms plant",
      pad: [66, 48], grid: [4, 3],
      block: { w: 34, dz: 24, eaves: 11.0, ridge: 14.0, bays: 14, ends: 10 },
      body: rgb(0x9aa2a3), accent: rgb(0x4e5a52),
      bespoke: true,
      blurb: "Assembly hall, test and service yard, secure loading area",
    },
    machinery_works: {
      name: "Machinery works",
      pad: [66, 48], home: [-0.06, 0.08], grid: [4, 3],
      block: { w: 28, dz: 18, eaves: 11.0, ridge: 13.2, bays: 12, ends: 7 },
      body: rgb(0xa4a8a5), accent: rgb(0x7a6a4a),
      blurb: "A machining bay under a travelling crane, a fitting bay behind it, and loading in the open",
    },
    generation: {
      name: "Generation plant",
      pad: [70, 50], home: [-0.20, 0.06], grid: [4, 3],
      block: { w: 24, dz: 19, eaves: 12.5, ridge: 14.0, bays: 10, ends: 8 },
      body: rgb(0xb0b2ac), accent: rgb(0x8d5f45),
      blurb: "A machine hall, a conversion block, heat rejection and consumables handling, kept deliberately generic with technology variants left to a later pass",
    },
    processing_plant: {
      name: "Processing plant",
      pad: [68, 48], home: [-0.22, 0.06], grid: [3, 3],
      block: { w: 22, dz: 16, eaves: 9.0, ridge: 10.6, bays: 9, ends: 6 },
      body: rgb(0xb6b1a4), accent: rgb(0x8f8a7c),
      blurb: "An open process structure carrying vessels, a bunded tank farm, pipework and tanker loading",
    },
    freight_terminal: {
      name: "Freight terminal",
      pad: [72, 50], home: [-0.02, 0.10], grid: [4, 2],
      block: { w: 30, dz: 12, eaves: 7.6, ridge: 8.6, bays: 12, ends: 5 },
      body: rgb(0xa8adb2), accent: rgb(0x51697f),
      blurb: "Two ballasted roads under a transfer gantry, container stacks, and a transit shed with a covered edge",
    },
    warehouse: {
      name: "Warehouse",
      pad: [70, 48], home: [0.02, 0.14], grid: [5, 3],
      block: { w: 34, dz: 20, eaves: 11.5, ridge: 12.8, bays: 14, ends: 8 },
      body: rgb(0xb2b6b8), accent: rgb(0x6b7a84),
      blurb: "A racked high bay, a full-length dock elevation under canopy, and the sprinkler tank standing outside it",
    },
    automation: {
      name: "Automation retrofit",
      pad: [62, 44], home: [-0.30, 0.06], grid: [2, 2],
      block: { w: 11, dz: 9, eaves: 5.4, ridge: 6.2, bays: 5, ends: 4 },
      body: rgb(0xa6acae), accent: rgb(0x66788a),
      retrofit: true,
      blurb: "A machine-cell line installed in a facility that already stands, with a link annex on its gable, and not a new building",
    },
    efficiency: {
      name: "Efficiency retrofit",
      pad: [62, 44], home: [-0.30, 0.06], grid: [2, 2],
      block: { w: 10, dz: 8, eaves: 4.8, ridge: 5.6, bays: 4, ends: 3 },
      body: rgb(0xb0aca0), accent: rgb(0x7f6a4c),
      retrofit: true,
      blurb: "Heat recovery, over-cladding and controls fitted to a facility that already stands, and not a new building",
    },
    starter_industry: {
      name: "Starter industry",
      pad: [50, 38], home: [-0.10, 0.04], grid: [3, 2],
      block: { w: 16, dz: 11, eaves: 6.0, ridge: 7.4, bays: 7, ends: 4 },
      body: rgb(0xb4b0a4), accent: rgb(0x7b8a6a),
      blurb: "A single-span workshop with an open lean-to, a blockwork store, an office pod and a small open yard",
    },
    office_district: {
      name: "Office district",
      pad: [66, 46], home: [-0.28, 0.08], grid: [3, 2],
      block: { w: 16, dz: 12, eaves: 6.6, ridge: 7.8, bays: 7, ends: 5 },
      body: rgb(0xbebcb3), accent: rgb(0x4f7384),
      blurb: "A multi-storey office block, shared service building, shaded entrance and pedestrian forecourt",
    },
    shipyard: {
      name: "Shipyard",
      pad: [72, 50], home: [-0.27, 0.12], grid: [3, 3],
      block: { w: 20, dz: 20, eaves: 10.5, ridge: 12.6, bays: 9, ends: 8 },
      body: rgb(0xa0a9ac), accent: rgb(0x6b787b),
      blurb: "A coastal fabrication shed beside an empty dry dock, lifting gantry, quay services and repair access",
    },
    advanced_industry: {
      name: "Advanced industry",
      pad: [66, 48], home: [-0.14, 0.10], grid: [4, 3],
      block: { w: 28, dz: 18, eaves: 8.6, ridge: 10.2, bays: 12, ends: 7 },
      body: rgb(0xbcc3c2), accent: rgb(0x527d7e),
      blurb: "A controlled assembly hall with filtered-air services, process supply cabinets and a separate clean loading vestibule",
    },
  };
  const KIND_KEYS = Object.keys(KINDS);
  // The row knows its own key. `build` has it, but the composition dispatch and
  // the retrofit host both want it from the row itself.
  for (const key of KIND_KEYS) KINDS[key].key = key;

  /// Deterministic per-kind variation. Two provinces building the same thing
  /// should not be pixel-identical, and a seeded generator is not available to
  /// this file and should not be — the sim owns the one RNG (iron rule 1). A
  /// hash of the key plus the caller's `variant` integer, fed through index
  /// arithmetic and trig, gives stable variety with no state at all.
  function keyHash(s) {
    let h = 7;
    for (let i = 0; i < s.length; i += 1) h = (h * 31 + s.charCodeAt(i)) % 9973;
    return h;
  }

  // ------------------------------------------------------------- shared kit
  // The pieces every kind reuses. Roadmap section E asks for one small shared
  // stage kit rather than five unrelated copies per facility, and this is it:
  // hoarding, spoil, excavation, reinforcement and formwork, frame, cladding,
  // crane, huts, stacked materials and lighting.

  /// The made-up platform, with a hole left where the works are. The hole is
  /// how an excavation exists without going below Y=0 — the apron is a ring,
  /// and the dig is what you see through it.
  /// DECK WINDING. These four quads run x-then-z, which winds a face looking at
  /// the ground rather than at the sky — the made-up platform every site stands
  /// on was lit from underneath. `deck` fixes the order in one place so the
  /// platform, the pit floor and the hardstanding all agree, and the checks now
  /// assert that a surface you walk on has a normal with a positive Y.
  function deck(m, x0, x1, y, z0, z1, col) {
    m.quad([x0, y, z0], [x0, y, z1], [x1, y, z1], [x1, y, z0], col);
  }

  function platform(m, W, D, hole, col) {
    const hw = W / 2, hd = D / 2;
    if (!hole) {
      deck(m, -hw, hw, GRADE, -hd, hd, col);
    } else {
      const aw = hole.w / 2, ad = hole.d / 2, cx = hole.x || 0, cz = hole.z || 0;
      const bands = [
        [-hw, hw, -hd, cz - ad], [-hw, hw, cz + ad, hd],
        [-hw, cx - aw, cz - ad, cz + ad], [cx + aw, hw, cz - ad, cz + ad],
      ];
      for (const b of bands) {
        if (b[1] - b[0] < 1e-6 || b[3] - b[2] < 1e-6) continue;
        deck(m, b[0], b[1], GRADE, b[2], b[3], col);
      }
    }
    // Outer skirt. The undersides are never seen and are not drawn; the skirt
    // bottom edge is what touches Y=0 and it is what `bounds.min[1]` reads.
    m.quad([-hw, 0, hd], [hw, 0, hd], [hw, GRADE, hd], [-hw, GRADE, hd], shade(col, 0.86));
    m.quad([hw, 0, -hd], [-hw, 0, -hd], [-hw, GRADE, -hd], [hw, GRADE, -hd], shade(col, 0.8));
    m.quad([hw, 0, hd], [hw, 0, -hd], [hw, GRADE, -hd], [hw, GRADE, hd], shade(col, 0.83));
    m.quad([-hw, 0, -hd], [-hw, 0, hd], [-hw, GRADE, hd], [-hw, GRADE, -hd], shade(col, 0.83));
  }

  /// The dig. Battered sides from the apron edge down to the pit floor, seen
  /// from inside, plus the cut floor. Depth is visual: a foundation pit at this
  /// scale is a shallow dish, and the spoil bund beside it does most of the work
  /// of saying how much was moved.
  function excavation(m, cx, cz, w, d, depth, benches, d0) {
    const fine = !!(d0 && d0.fine);
    const floorY = Math.max(PIT, GRADE - depth);
    const aw = w / 2, ad = d / 2, batter = Math.min(1.6, depth * 2.2);
    const iw = Math.max(1, aw - batter), id = Math.max(1, ad - batter);
    const outer = [[cx - aw, GRADE, cz - ad], [cx + aw, GRADE, cz - ad], [cx + aw, GRADE, cz + ad], [cx - aw, GRADE, cz + ad]];
    const inner = [[cx - iw, floorY, cz - id], [cx + iw, floorY, cz - id], [cx + iw, floorY, cz + id], [cx - iw, floorY, cz + id]];
    for (let i = 0; i < 4; i += 1) {
      const j = (i + 1) % 4;
      m.quad(outer[i], inner[i], inner[j], outer[j], shade(P.earthCut, 0.94 + 0.06 * i));
    }
    // The floor of a dig is churned, not swept. A grid of faces at slightly
    // different tones is the only way to say that with no textures, and it also
    // gives the pit a scale to read against.
    if (fine) groundPatch(m, cx, cz, iw * 2, id * 2, 9, 7, floorY, P.earthCut, 0.075);
    else deck(m, cx - iw, cx + iw, floorY, cz - id, cz + id, P.earthCut);
    // Trench sheeting on the two long sides once the dig is at depth. It is the
    // detail that separates a hole from a formed excavation at a glance.
    if (benches) {
      for (const s of [-1, 1]) {
        for (let i = 0; i < benches; i += 1) {
          const x = cx - iw + (i + 0.5) * (iw * 2 / benches);
          m.bar(x - 0.5, x + 0.5, floorY, GRADE + 0.35, cz + s * id - 0.08, cz + s * id + 0.08, P.rebar);
        }
        if (!fine) continue;
        // Waling and struts. Sheet piles hold nothing up on their own — the
        // horizontal waling across their faces is the member doing the work,
        // and it is what makes a dig read as supported rather than as a hole
        // somebody hopes will stand.
        for (const y of [floorY + 0.5, GRADE - 0.35]) {
          m.beam(cx - iw - 0.2, cx + iw + 0.2, y, y + 0.26, cz + s * id - 0.2, cz + s * id + 0.02, shade(P.primer, 0.95), 0.02);
        }
      }
      for (let i = 0; i < 3; i += 1) {
        const x = cx - iw * 0.7 + i * iw * 0.7;
        m.rod([x, GRADE - 0.22, cz - id], [x, GRADE - 0.22, cz + id], 0.14, 6, shade(P.primer, 1.05), false);
      }
    }
    // A haul ramp into the pit. Every excavation this size has one; without it
    // the plant that dug it could not have got out.
    if (fine) {
      const rw = Math.min(4.2, iw * 0.6);
      m.quad([cx - rw, GRADE, cz - ad], [cx - rw, floorY, cz - id * 0.2], [cx + rw, floorY, cz - id * 0.2], [cx + rw, GRADE, cz - ad],
        shade(P.hardcore, 0.92));
      for (const s of [-1, 1]) {
        m.quad([cx + s * rw, GRADE, cz - ad], [cx + s * rw, floorY, cz - id * 0.2],
          [cx + s * rw, floorY - 0.02, cz - id * 0.2], [cx + s * rw, GRADE - 0.02, cz - ad], shade(P.hardcore, 0.8));
      }
    }
  }

  /// Pad footings, reinforcement cages and formwork. Three different things in
  /// one call because on site they are one thing: a grid of bases at three
  /// different states of being poured, which is what a foundation stage looks
  /// like and why it is worth drawing at all.
  function footings(m, cx, cz, w, d, cols, rows, d0) {
    const floorY = Math.max(PIT, GRADE - 1.1);
    for (let i = 0; i < cols; i += 1) {
      for (let j = 0; j < rows; j += 1) {
        const x = cx + (i - (cols - 1) / 2) * (w / cols);
        const z = cz + (j - (rows - 1) / 2) * (d / rows);
        const state = (i * 3 + j * 2) % 5;           // poured, formed, or caged
        m.bar(x - 1.1, x + 1.1, floorY, floorY + 0.35, z - 1.1, z + 1.1, P.concreteWet);
        if (!d0.fine) continue;
        if (state === 0) {
          // Poured and struck. What is left is the kicker, the holding-down
          // bolts cast into it and the starter bars for the column — the three
          // things that say a base is finished rather than a slab of nothing.
          m.beam(x - 0.55, x + 0.55, floorY + 0.35, floorY + 0.62, z - 0.55, z + 0.55, P.concrete, 0.025);
          for (let b = 0; b < 4; b += 1) {
            const bx = x + (b % 2 ? 0.34 : -0.34), bz = z + (b < 2 ? 0.34 : -0.34);
            m.rod([bx, floorY + 0.6, bz], [bx, floorY + 0.82, bz], 0.035, 6, P.steel, true);
          }
          for (let s = 0; s < 4; s += 1) {
            const sx = x + (s % 2 ? 0.7 : -0.7), sz = z + (s < 2 ? 0.7 : -0.7);
            m.rod([sx, floorY + 0.3, sz], [sx, floorY + 1.35, sz], 0.026, 6, P.rebar, false);
          }
        } else if (state < 3) {
          // Formwork: face panels, soldiers standing against them, a waling
          // round the top and raking props holding the lot plumb. A box of four
          // thin planks is not formwork, it is a picture of formwork.
          for (const s of [-1, 1]) {
            m.bar(x - 1.2, x + 1.2, floorY + 0.28, floorY + 1.08, z + s * 1.18 - 0.05, z + s * 1.18 + 0.05, P.timber);
            m.bar(x + s * 1.18 - 0.05, x + s * 1.18 + 0.05, floorY + 0.28, floorY + 1.08, z - 1.2, z + 1.2, P.timber);
            m.bar(x - 1.26, x + 1.26, floorY + 0.86, floorY + 0.98, z + s * 1.24 - 0.06, z + s * 1.24 + 0.06, shade(P.timber, 0.82));
            m.bar(x + s * 1.24 - 0.06, x + s * 1.24 + 0.06, floorY + 0.86, floorY + 0.98, z - 1.26, z + 1.26, shade(P.timber, 0.82));
            for (const t of [-0.6, 0.6]) {
              m.bar(x + t - 0.05, x + t + 0.05, floorY + 0.28, floorY + 1.12, z + s * 1.24 - 0.05, z + s * 1.24 + 0.05, shade(P.timber, 1.1));
            }
            m.rod([x + s * 1.24, floorY + 1.0, z], [x + s * 2.3, floorY + 0.05, z], 0.05, 6, P.safety, false);
          }
        } else {
          // Blinding, spacers and a cage. Eight bars round the perimeter with
          // closed links at four levels is what one actually looks like from
          // above, and it is the difference between "reinforcement" and "four
          // sticks in a hole".
          // Blinding, and it stops AT Y=0 rather than a centimetre under it.
          // `finish` re-seats the lowest vertex on the ground, so a blinding
          // slab that dipped below zero lifted the whole model by 10 mm at the
          // foundation stage and nowhere else — every object on the site sat a
          // centimetre higher for one stage of five.
          m.bar(x - 1.25, x + 1.25, Math.max(0, floorY - 0.06), floorY, z - 1.25, z + 1.25, shade(P.concreteWet, 0.88));
          rebarCage(m, x, z, 0.72, 0.72, floorY + 0.12, floorY + 1.95, 6);
          for (const c of [-1, 1]) {
            m.rod([x + c * 0.85, floorY + 0.12, z - 0.85], [x + c * 0.85, floorY + 0.12, z + 0.85], 0.03, 6, shade(P.rebar, 0.9), false);
          }
        }
      }
    }
  }

  /// The slab. From the frame stage on it is the thing everything else stands
  /// on, and its edge is the only place the made-up ground is still visible.
  function slab(m, cx, cz, w, d, col, d0) {
    const top = GRADE + 0.18;
    m.bar(cx - w / 2, cx + w / 2, PIT, top, cz - d / 2, cz + d / 2, col);
    if (d0 && d0.fine) {
      // Construction joints, saw-cut on the grid the slab was poured in. They
      // are the only marking a bare industrial floor has, and this renderer
      // cannot draw a line, so they are 8 mm of real recess.
      for (let i = 1; i < 4; i += 1) {
        m.bar(cx - w / 2 + (i * w) / 4 - 0.03, cx - w / 2 + (i * w) / 4 + 0.03, top - 0.02, top,
          cz - d / 2 + 0.2, cz + d / 2 - 0.2, shade(col, 0.78));
      }
      for (let i = 1; i < 3; i += 1) {
        m.bar(cx - w / 2 + 0.2, cx + w / 2 - 0.2, top - 0.02, top,
          cz - d / 2 + (i * d) / 3 - 0.03, cz - d / 2 + (i * d) / 3 + 0.03, shade(col, 0.78));
      }
      // The chamfered arris round the slab edge, which is where the shutter was
      // and the one edge of a floor slab anybody ever sees.
      m.beam(cx - w / 2 - 0.04, cx + w / 2 + 0.04, top - 0.14, top, cz - d / 2 - 0.04, cz - d / 2 + 0.1, shade(col, 0.94), 0.022);
      m.beam(cx - w / 2 - 0.04, cx + w / 2 + 0.04, top - 0.14, top, cz + d / 2 - 0.1, cz + d / 2 + 0.04, shade(col, 0.94), 0.022);
    }
    return top;
  }

  /// One portal frame: two columns, two rafters, a haunch each side. Primed
  /// steel, because a frame that has not been clad has not been painted either.
  ///
  /// WHAT CHANGED AND WHY. This used to be four boxes and a stepped ribbon, with
  /// the left half wound inside out — the frame that carries the whole building
  /// was lit from behind on one side of the ridge. It is now built once in the
  /// right-hand half and MIRRORED, so `tri` re-winds it and both halves face out
  /// by construction. The members are I-sections rather than solid boxes,
  /// because the flange-web-flange read is what tells a viewer this is rolled
  /// steel and not a length of timber, and it is only three primitives.
  function portal(m, cz, span, depth, eaves, ridge, col) {
    const hw = span / 2, t = 0.22, fl = 0.055, dep = 0.34;
    const steps = 3;
    for (const s of [-1, 1]) {
      m.save();
      if (s < 0) m.scale(-1, 1, 1);
      // Column, as flange / web / flange.
      m.beam(hw - t, hw - t + fl, depth + 0.2, eaves, cz - t, cz + t, col, 0.016);
      m.beam(hw + t - fl, hw + t, depth + 0.2, eaves, cz - t, cz + t, shade(col, 0.94), 0.016);
      m.bar(hw - t + fl, hw + t - fl, depth + 0.2, eaves, cz - 0.05, cz + 0.05, shade(col, 0.8));
      // Grout bed, base plate, holding-down bolts. The bolts are the only place
      // a steel frame touches its foundation and they are four of the cheapest
      // triangles on the model.
      m.bar(hw - 0.42, hw + 0.42, depth, depth + 0.1, cz - 0.42, cz + 0.42, P.concrete);
      m.beam(hw - 0.5, hw + 0.5, depth + 0.1, depth + 0.16, cz - 0.5, cz + 0.5, shade(col, 0.8), 0.02);
      for (let b = 0; b < 4; b += 1) {
        const bx = hw + (b % 2 ? 0.34 : -0.34), bz = cz + (b < 2 ? 0.34 : -0.34);
        m.rod([bx, depth + 0.16, bz], [bx, depth + 0.3, bz], 0.038, 6, P.steel, true);
      }
      // Haunch: the deepened wedge under the rafter at the eaves, and the pair
      // of stiffeners welded either side of it.
      const hx = hw - span * 0.09, hy = eaves + (ridge - eaves) * (span * 0.09) / hw;
      m.webPlate([[hw - t, eaves + dep], [hw - t, eaves - 0.85], [hx, hy]], cz - t, cz + t, shade(col, 0.96));
      for (const zs of [-1, 1]) {
        m.bar(hw - t, hw + t - fl, eaves - 0.5, eaves, cz + zs * 0.14 - 0.02, cz + zs * 0.14 + 0.02, shade(col, 1.06));
      }
      // Rafter, stepped from eaves to ridge. Four faces per step: the underside
      // used to be left open, and a rafter you can see the inside of is a
      // rafter that is not there.
      for (let i = 0; i < steps; i += 1) {
        const x0 = hw * (1 - i / steps), x1 = hw * (1 - (i + 1) / steps);
        const y0 = eaves + (ridge - eaves) * (i / steps), y1 = eaves + (ridge - eaves) * ((i + 1) / steps);
        m.quad([x0, y0, cz - t], [x1, y1, cz - t], [x1, y1 + dep, cz - t], [x0, y0 + dep, cz - t], shade(col, 1.05));
        m.quad([x1, y1, cz + t], [x0, y0, cz + t], [x0, y0 + dep, cz + t], [x1, y1 + dep, cz + t], shade(col, 0.92));
        m.quad([x0, y0 + dep, cz - t], [x1, y1 + dep, cz - t], [x1, y1 + dep, cz + t], [x0, y0 + dep, cz + t], shade(col, 1.12));
        m.quad([x1, y1, cz - t], [x0, y0, cz - t], [x0, y0, cz + t], [x1, y1, cz + t], shade(col, 0.72));
      }
      m.restore();
    }
    // Apex splice plate, bolted through both rafters. One per frame, not one
    // per side, which is why it sits outside the mirror.
    m.beam(-0.55, 0.55, ridge - 0.12, ridge + dep + 0.12, cz - t - 0.03, cz + t + 0.03, shade(col, 1.08), 0.018);
  }

  /// A clad wall in local space: spans x in [-w/2, w/2], y in [0, h], facing +Z.
  /// `clad` is the fraction of bays actually sheeted, which is what turns one
  /// function into both the enclosing stage and the finished building.
  function claddedWall(m, w, h, bays, col, d0, o) {
    const opt = o || {};
    const bw = w / bays;
    const done = Math.round(bays * (opt.clad == null ? 1 : clamp01(opt.clad)));
    for (let i = 0; i < bays; i += 1) {
      if (i >= done) continue;
      const x0 = -w / 2 + i * bw, x1 = x0 + bw;
      const tone = 1 + 0.05 * Math.sin(i * 1.7 + (opt.phase || 0));
      // Profiled at inspection range, one flat quad at map range. A rib is
      // 35 mm deep and there is no distance at which drawing it costs less than
      // it earns close up or more than it earns far away.
      if (d0.fine) ribbedPanel(m, x0, x1, h, opt.pitch || 0.75, 0.05, shade(col, tone));
      else m.quad([x0, 0, 0], [x1, 0, 0], [x1, h, 0], [x0, h, 0], shade(col, tone));
      // The lining. A sheeted wall is one quad thick, and a part-clad building
      // is the one time you look at the back of it: without this you see
      // straight through the far elevation and the panel shades from its
      // outward normal, which is worse than a plain grey inside face.
      m.quad([x1, 0, -0.14], [x0, 0, -0.14], [x0, h, -0.14], [x1, h, -0.14], shade(P.dark, 1.5));
    }
    if (done > 0 && d0.fine) {
      const edge = -w / 2 + done * bw;
      // Flashings are what close a sheeted envelope, and they are chamfered
      // because a folded flashing has an arris on every bend — at eaves height
      // that line is the top edge of the whole building.
      m.beam(-w / 2, edge, h - 0.3, h, 0.0, 0.18, shade(col, 1.1), 0.018);       // eaves trim
      m.beam(-w / 2, edge, 0, 0.32, 0.0, 0.18, shade(col, 0.72), 0.018);         // base flashing
      m.bar(-w / 2, edge, 0.32, 0.4, 0.02, 0.14, shade(P.galv, 0.86));           // base drip
      // Corner flashing on the finished end only, so a part-clad wall still
      // shows the sheeting running out mid-elevation.
      if (done === bays) {
        m.beam(edge - 0.18, edge, 0, h, 0.0, 0.2, shade(col, 0.94), 0.018);
      }
      // Fixing rows: a sheeted wall is screwed to rails at three levels and the
      // shadow line of that row is visible on any real one.
      for (let r = 1; r <= 3; r += 1) {
        m.bar(-w / 2, edge, (h * r) / 4, (h * r) / 4 + 0.05, 0.05, 0.1, shade(col, 0.8));
      }
    }
  }

  /// Side rails and sheeting rails: the horizontal steel a wall is fixed to.
  /// Visible only while the wall is open, which is exactly when it should be.
  function sideRails(m, w, h, levels, cz, col) {
    for (let i = 1; i <= levels; i += 1) {
      const y = (h * i) / (levels + 1);
      m.bar(-w / 2, w / 2, y - 0.1, y + 0.1, cz - 0.09, cz + 0.09, col);
    }
  }

  /// Roof: purlins first, then sheets, and a stage can have the first without
  /// the second. Local space matches the walls — ridge on the x=0 line, slopes
  /// running along z.
  function roof(m, w, d, eaves, ridge, sheets, col, d0, o) {
    const opt = o || {};
    const hw = w / 2, hd = d / 2;
    const done = Math.round(sheets * (opt.clad == null ? 1 : clamp01(opt.clad)));
    // Slope length and pitch, so anything laid ON the roof is laid on the roof
    // rather than floating above it at a guessed height.
    const run = Math.hypot(hw, ridge - eaves);
    const pitch = Math.atan2(ridge - eaves, hw) / DEG;
    /// One standing seam, eaves to ridge, on slope `s`. WHY IT IS WORTH THE
    /// TRIANGLES: a roof that is two flat quads reads as a folded card from any
    /// angle that shows the ridge. The seam lines are the only thing that can
    /// say "sheets, laid side by side and lapped" in a renderer with no
    /// textures, and they are what makes the eye read a length.
    const onSlope = (s, fn) => {
      m.save().move(s * hw, eaves, 0);
      if (s > 0) m.scale(-1, 1, 1);   // mirrored, and `tri` re-winds it for us
      m.rotZ(pitch);
      fn();
      m.restore();
    };
    for (let i = 0; i < done; i += 1) {
      const z0 = -hd + (i * d) / sheets, z1 = -hd + ((i + 1) * d) / sheets;
      const tone = 1 + 0.045 * Math.sin(i * 2.1);
      // WINDING. These four used to be listed x-then-z, which put the weather
      // face of the roof looking at the floor and the dark inner lining looking
      // at the sky — the whole envelope was lit inside out. The sheet faces up
      // and out; the lining, 200 mm under it, faces down and in.
      m.quad([-hw, eaves, z1], [0, ridge, z1], [0, ridge, z0], [-hw, eaves, z0], shade(col, tone));
      m.quad([0, ridge, z1], [hw, eaves, z1], [hw, eaves, z0], [0, ridge, z0], shade(col, tone * 0.93));
      m.quad([-hw, eaves - 0.2, z0], [0, ridge - 0.2, z0], [0, ridge - 0.2, z1], [-hw, eaves - 0.2, z1], shade(P.dark, 1.5));
      m.quad([0, ridge - 0.2, z0], [hw, eaves - 0.2, z0], [hw, eaves - 0.2, z1], [0, ridge - 0.2, z1], shade(P.dark, 1.4));
      if (d0.ribs) {
        for (const s of [-1, 1]) {
          onSlope(s, () => m.bar(run * 0.02, run * 0.98, 0, 0.08, z1 - 0.07, z1 + 0.07, shade(col, 0.9)));
        }
      }
    }
    if (opt.purlins) {
      for (const s of [-1, 1]) {
        for (let i = 1; i <= 4; i += 1) {
          const t = i / 5, x = s * hw * (1 - t), y = eaves + (ridge - eaves) * t;
          m.bar(x - 0.11, x + 0.11, y, y + 0.16, -hd, hd, P.primer);
        }
      }
      m.bar(-0.14, 0.14, ridge, ridge + 0.18, -hd, hd, P.primer);
    }
    if (done > 0 && d0.fine) {
      // A folded ridge capping has arrises down both sides of it and they are
      // the brightest line on the building, so this one is chamfered.
      m.beam(-0.4, 0.4, ridge + 0.04, ridge + 0.32, -hd - 0.25, hd + 0.25, shade(col, 1.12), 0.035);
      for (const s of [-1, 1]) {
        m.bar(s * hw - 0.22, s * hw + 0.22, eaves - 0.28, eaves, -hd, hd, shade(col, 0.78));
        m.bar(s * hw - 0.3, s * hw + 0.06, eaves - 0.42, eaves - 0.26, -hd, hd, shade(P.galv, 0.9));   // drip
        // Barge board along each gable verge, which is where a sheeted roof is
        // closed off and the one place its thickness is on show.
        for (const g of [-1, 1]) {
          onSlope(s, () => m.bar(run * 0.01, run * 0.99, -0.14, 0.1, g * hd - 0.06, g * hd + 0.16, shade(col, 0.82)));
        }
      }
    }
  }

  /// Perimeter hoarding: painted panels on posts with a gate. The one saturated
  /// colour on the site, and the thing that says "this is a site" from further
  /// away than any of the rest of it.
  function hoarding(m, W, D, d0, col) {
    const hw = W / 2 - 0.6, hd = D / 2 - 0.6, h = 2.4;
    const runs = [
      [-hw, hd, hw, hd], [hw, -hd, -hw, -hd],
      [hw, hd, hw, -hd], [-hw, -hd, -hw, hd],
    ];
    const step = d0.fine ? 4.0 : 12.0;
    runs.forEach((r, ri) => {
      const dx = r[2] - r[0], dz = r[3] - r[1], len = Math.hypot(dx, dz);
      const n = Math.max(2, Math.round(len / step));
      const ux = dx / len, uz = dz / len;
      for (let i = 0; i < n; i += 1) {
        // The gate: one bay of the front run is left open and hung with a leaf.
        const gate = ri === 1 && i === Math.floor(n / 2);
        const t0 = (i * len) / n, t1 = ((i + 1) * len) / n;
        const x0 = r[0] + ux * t0, z0 = r[1] + uz * t0;
        const x1 = r[0] + ux * t1, z1 = r[1] + uz * t1;
        const tone = 1 + 0.06 * Math.sin(ri * 2 + i);
        if (!gate) {
          m.quad([x0, GRADE, z0], [x1, GRADE, z1], [x1, GRADE + h, z1], [x0, GRADE + h, z0], shade(col, tone));
          m.quad([x1, GRADE, z1], [x0, GRADE, z0], [x0, GRADE + h, z0], [x1, GRADE + h, z1], shade(col, 0.7));
        } else {
          m.quad([x0, GRADE, z0], [x0 + (x1 - x0) * 0.45, GRADE, z0 + (z1 - z0) * 0.45],
            [x0 + (x1 - x0) * 0.45, GRADE + h, z0 + (z1 - z0) * 0.45], [x0, GRADE + h, z0], shade(P.safety, 1));
        }
        if (d0.fine) {
          // Post, capping rail and a raking prop back into the site. A hoarding
          // is a fence panel screwed to posts that are held up by something, and
          // the props are the part that says it is temporary.
          m.beam(x0 - 0.12, x0 + 0.12, GRADE, GRADE + h + 0.18, z0 - 0.12, z0 + 0.12, P.steel, 0.016);
          const mx = (x0 + x1) / 2, mz = (z0 + z1) / 2;
          m.bar(Math.min(x0, x1) - 0.06, Math.max(x0, x1) + 0.06, GRADE + h, GRADE + h + 0.14,
            Math.min(z0, z1) - 0.06, Math.max(z0, z1) + 0.06, shade(col, 1.14));
          if (!gate) {
            const inx = uz, inz = -ux;   // into the compound, whichever way the run points
            m.rod([mx, GRADE + h * 0.82, mz], [mx + inx * 1.1, GRADE + 0.05, mz + inz * 1.1], 0.05, 6, P.steel, false);
            m.bar(mx + inx * 1.05 - 0.2, mx + inx * 1.05 + 0.2, GRADE, GRADE + 0.24,
              mz + inz * 1.05 - 0.2, mz + inz * 1.05 + 0.2, P.concreteWet);
          }
        }
      }
    });
  }

  /// The permanent line at hand-over: palisade on concrete, not painted board.
  function fence(m, W, D, d0) {
    const hw = W / 2 - 0.8, hd = D / 2 - 0.8, h = 2.6;
    // Close up, palisade centres are shorter than a hoarding panel and the
    // posts are the thing you read.
    //
    // AT MAP SCALE A PERIMETER IS A LINE. A 0.2 m post is far under a pixel
    // there, and drawing it anyway is how a map budget gets eaten: the coarse
    // fence used to cost 264 of the 800 triangles a whole site is allowed, and
    // it bought four dozen marks nobody can resolve. The coarse path now draws
    // each run as one continuous rail — which is what the eye reads at that
    // size anyway — plus a post at each corner so the compound has a shape.
    // That is 96 triangles for the whole perimeter, and the 168 it gave back
    // are what paid for every kind having its own silhouette on the map.
    const step = d0.fine ? 4.0 : 1e9;
    if (!d0.fine) {
      for (const [a, b] of [[[-hw, hd], [hw, hd]], [[hw, -hd], [-hw, -hd]], [[hw, hd], [hw, -hd]], [[-hw, -hd], [-hw, hd]]]) {
        m.bar(Math.min(a[0], b[0]) - 0.07, Math.max(a[0], b[0]) + 0.07, GRADE, GRADE + h,
          Math.min(a[1], b[1]) - 0.07, Math.max(a[1], b[1]) + 0.07, shade(P.galv, 0.94));
      }
      for (const cx of [-hw, hw]) {
        for (const cz of [-hd, hd]) m.bar(cx - 0.22, cx + 0.22, GRADE, GRADE + h + 0.2, cz - 0.22, cz + 0.22, P.galv);
      }
      return;
    }
    for (const [a, b] of [[[-hw, hd], [hw, hd]], [[hw, -hd], [-hw, -hd]], [[hw, hd], [hw, -hd]], [[-hw, -hd], [-hw, hd]]]) {
      const dx = b[0] - a[0], dz = b[1] - a[1], len = Math.hypot(dx, dz);
      const n = Math.max(2, Math.round(len / step));
      for (let i = 0; i <= n; i += 1) {
        const x = a[0] + (dx * i) / n, z = a[1] + (dz * i) / n;
        if (d0.fine) m.beam(x - 0.1, x + 0.1, GRADE, GRADE + h + 0.16, z - 0.1, z + 0.1, P.galv, 0.018);
        else m.bar(x - 0.1, x + 0.1, GRADE, GRADE + h, z - 0.1, z + 0.1, P.galv);
      }
      // The pales. A palisade with no pales is two wires and a row of posts,
      // which is a cattle fence, not a compound line — and the vertical rhythm
      // is the only thing that gives the perimeter a scale to read the building
      // against. Close range only: at map range these are far under a pixel and
      // the two rails carry the whole silhouette.
      if (d0.fine) {
        const px = dx / len, pz = dz / len;
        for (let i = 0; i < n; i += 1) {
          for (let p = 1; p <= 3; p += 1) {
            const t = ((i + (p / 4)) * len) / n;
            const x = a[0] + px * t, z = a[1] + pz * t;
            m.bar(x - 0.045, x + 0.045, GRADE + 0.12, GRADE + h + 0.1, z - 0.045, z + 0.045, shade(P.galv, 0.92 + 0.05 * (p % 2)));
          }
        }
      }
      const ux = dx / len, uz = dz / len;
      for (const y of [GRADE + 0.5, GRADE + h - 0.35]) {
        m.bar(a[0] - Math.abs(uz) * 0.05, b[0] + Math.abs(uz) * 0.05, y, y + 0.12,
          a[1] - Math.abs(ux) * 0.05, b[1] + Math.abs(ux) * 0.05, shade(P.galv, 0.9));
      }
    }
  }

  /// Site accommodation: stacked cabins on blocks with a stair. Two-high because
  /// that is what a compound this size gets, and it gives the site a second
  /// silhouette that is not the building.
  function siteHuts(m, x, z, rot, count, d0) {
    m.save().move(x, GRADE, z).rotY(rot);
    for (let i = 0; i < count; i += 1) {
      const level = i % 2, run = Math.floor(i / 2);
      const y = level * 2.7, zz = run * 3.4;
      m.bar(-3.0, 3.0, y + 0.25, y + 2.65, zz - 1.4, zz + 1.4, shade(P.hut, 1 - level * 0.04));
      m.bar(-3.05, 3.05, y + 2.6, y + 2.75, zz - 1.5, zz + 1.5, shade(P.hut, 0.8));
      if (d0.fine) {
        // A site cabin is a pressed steel box: ribbed sides, a rolled roof edge,
        // corner castings it is craned and stacked by, and a chassis under it.
        // Those four things are the whole reason one reads as a cabin and not as
        // a shipping crate, and they are what a stacked pair needs to look
        // STACKED rather than balanced.
        m.beam(-3.1, 3.1, y + 2.72, y + 2.86, zz - 1.56, zz + 1.56, shade(P.hut, 0.9), 0.02);
        m.beam(-3.06, 3.06, y + 0.18, y + 0.34, zz - 1.46, zz + 1.46, shade(P.hut, 0.72), 0.02);
        for (const cx of [-2.86, 2.86]) {
          for (const cz of [-1.3, 1.3]) {
            m.bar(cx - 0.16, cx + 0.16, y + 0.2, y + 2.78, zz + cz - 0.16, zz + cz + 0.16, shade(P.steel, 1.02));
          }
        }
        for (let r = 0; r < 5; r += 1) {
          const rx = -2.3 + r * 1.15;
          for (const fz of [zz - 1.44, zz + 1.38]) {
            m.bar(rx - 0.07, rx + 0.07, y + 0.34, y + 2.6, fz, fz + 0.06, shade(P.hut, 0.86));
          }
        }
        // Glazing with a frame and a sill, and a door with a frame and a handle.
        for (const wx of [-1.6, 0.2]) {
          m.bar(wx, wx + 1.1, y + 1.4, y + 2.15, zz + 1.4, zz + 1.44, P.glass);
          m.beam(wx - 0.09, wx + 1.19, y + 1.31, y + 2.24, zz + 1.4, zz + 1.52, shade(P.hut, 1.06), 0.015);
          m.bar(wx - 0.09, wx + 1.19, y + 1.24, y + 1.33, zz + 1.4, zz + 1.6, shade(P.hut, 0.78));
        }
        m.bar(1.9, 2.7, y + 0.3, y + 2.3, zz + 1.4, zz + 1.46, P.door);
        m.beam(1.8, 2.8, y + 0.26, y + 2.4, zz + 1.42, zz + 1.54, shade(P.hut, 0.94), 0.015);
        m.rod([2.6, y + 1.2, zz + 1.5], [2.6, y + 1.5, zz + 1.5], 0.035, 6, P.dark, true);
        for (let b = 0; b < 3; b += 1) m.bar(-2.6 + b * 2.4, -2.2 + b * 2.4, 0, 0.25, zz - 1.2, zz + 1.2, P.concrete);
      }
    }
    if (d0.fine) {
      // Steps to the upper cabin: two stringers, treads on them, a handrail with
      // posts, and a landing at the door. The old six floating slabs were a
      // stair-shaped gesture.
      const rise = 0.44, tread = 0.27, steps = 6;
      const zTop = -1.4 + steps * tread, yTop = rise * steps;
      for (const sx of [3.12, 4.28]) {
        m.rod([sx, 0.1, -1.4], [sx, yTop + 0.1, zTop], 0.065, 6, shade(P.galv, 0.9), false);
      }
      for (let s = 0; s < steps; s += 1) {
        m.beam(3.05, 4.35, rise * s + 0.14, rise * s + 0.22, -1.4 + s * tread, -0.78 + s * tread, P.galv, 0.012);
      }
      m.beam(3.0, 4.6, yTop + 0.06, yTop + 0.18, zTop - 0.1, zTop + 1.3, shade(P.galv, 0.96), 0.015);
      for (let p = 0; p <= 3; p += 1) {
        const pz = -1.4 + (p * (zTop + 1.2 + 1.4)) / 3;
        const py = Math.min(yTop, Math.max(0, ((pz + 1.4) / (zTop + 1.4)) * yTop));
        m.rod([4.45, py + 0.1, pz], [4.45, py + 1.15, pz], 0.035, 6, P.galv, false);
      }
      m.rod([4.45, 1.25, -1.4], [4.45, yTop + 1.25, zTop], 0.035, 6, shade(P.galv, 1.04), false);
      m.rod([4.45, yTop + 1.25, zTop], [4.45, yTop + 1.25, zTop + 1.2], 0.035, 6, shade(P.galv, 1.04), false);
    }
    m.restore();
  }

  /// Stacked materials. Four things a site keeps in the open — sheet pallets,
  /// pipe bundles, block packs and steel sections — chosen by index so a site
  /// does not have four of the same pile.
  function materialStack(m, x, z, kind, d0) {
    const y = GRADE;
    // WHAT MAKES A STACK A STACK. Material on a site is banded, chocked, sat on
    // bearers and never quite square. Four boxes of decreasing size read as a
    // diagram of a stack; the strapping and the bearers are what make it one,
    // and they are the cheapest detail in the file.
    // The map path must not grow by a triangle: the worst-case far mesh has
    // under a hundred to spare against a hard 800, so every addition below is
    // behind `fine` and the coarse counts are exactly what they were.
    if (kind === 0) {                                   // clad sheet packs
      if (d0.fine) for (const bz of [-0.9, 0.9]) m.bar(x - 3.0, x + 3.0, y, y + 0.14, z + bz - 0.14, z + bz + 0.14, P.timber);
      for (let i = 0; i < 3; i += 1) {
        const skew = d0.fine ? (i === 2 ? 0.22 : 0) : 0;
        m.bar(x - 3.2 + skew, x + 3.2 + skew, y + 0.14 + i * 0.34, y + 0.14 + i * 0.34 + 0.3,
          z - 1.1 + i * 0.12, z + 1.1 - i * 0.12, shade(P.clad, 0.9 + i * 0.06));
        if (!d0.fine) continue;
        for (const sx of [-2.1, 0.2, 2.4]) {           // strapping over each pack
          m.bar(x + sx + skew - 0.04, x + sx + skew + 0.04, y + 0.12 + i * 0.34, y + 0.46 + i * 0.34,
            z - 1.16 + i * 0.12, z + 1.16 - i * 0.12, shade(P.dark, 1.25));
        }
      }
      if (d0.fine) {
        for (const cx of [-3.0, 3.0]) m.beam(x + cx - 0.1, x + cx + 0.1, y + 0.14, y + 1.16, z - 1.0, z + 1.0, P.safety, 0.012);
      }
    } else if (kind === 1) {                            // pipe bundle
      // PIPE LIES DOWN. These were standing on end on a bearer mat laid out for
      // pipe lying along it, so a bundle read as a picket fence. Nested rows at
      // the offsets a stack of 1 m pipe actually nests at, on bearers, chocked
      // at both ends.
      m.bar(x - 3.4, x + 3.4, y, y + 0.16, z - 2.8, z + 2.8, P.timber);
      for (let r = 0; r < (d0.fine ? 2 : 1); r += 1) {
        for (let i = 0; i < 4 - r; i += 1) {
          const px = x - 2.4 + i * 1.05 + r * 0.53, py = y + 0.68 + r * 0.91;
          m.rod([px, py, z - 2.6], [px, py, z + 2.6], 0.5, d0.seg, P.steel, false);
          if (d0.fine) {
            // A bore you can see into. A capped tube is a bar; the ring at the
            // end is what says pipe.
            for (const cz of [z - 2.6, z + 2.58]) {
              m.rod([px, py, cz], [px, py, cz + 0.02], 0.5, d0.seg, shade(P.dark, 1.2), false);
              m.rod([px, py, cz], [px, py, cz + 0.02], 0.36, d0.seg, shade(P.dark, 0.7), false);
            }
          }
        }
      }
      if (d0.fine) {
        for (const cz of [z - 1.9, z + 1.9]) {          // chocks and bundle strap
          m.bar(x - 3.3, x + 3.3, y + 0.16, y + 0.26, cz - 0.12, cz + 0.12, shade(P.timber, 0.86));
          m.bar(x - 3.3, x - 3.1, y + 0.16, y + 1.9, cz - 0.1, cz + 0.1, P.timber);
          m.bar(x + 3.1, x + 3.3, y + 0.16, y + 1.9, cz - 0.1, cz + 0.1, P.timber);
        }
      }
    } else if (kind === 2) {                            // block packs
      for (let i = 0; i < 4; i += 1) {
        const ox = (i % 2) * 2.5, oz = Math.floor(i / 2) * 1.7;
        m.bar(x + ox - 1.1, x + ox + 1.1, y + 0.12, y + 1.2, z + oz - 0.75, z + oz + 0.75, shade(P.concrete, 0.88 + 0.06 * i));
        if (!d0.fine) continue;
        m.bar(x + ox - 1.15, x + ox + 1.15, y + 1.2, y + 1.26, z + oz - 0.8, z + oz + 0.8, P.hardcore);
        m.bar(x + ox - 1.16, x + ox + 1.16, y, y + 0.12, z + oz - 0.8, z + oz + 0.8, P.timber);  // pallet
        for (const sx of [-0.6, 0.6]) {                 // banding
          m.bar(x + ox + sx - 0.04, x + ox + sx + 0.04, y + 0.1, y + 1.24, z + oz - 0.79, z + oz + 0.79, shade(P.dark, 1.35));
        }
        // The half-used pack, in courses, because a compound this old has one.
        if (i === 3) {
          for (let c = 0; c < 3; c += 1) {
            m.bar(x + ox - 1.1 + c * 0.42, x + ox - 0.78 + c * 0.42, y + 1.26, y + 1.48,
              z + oz - 0.72 + (c % 2) * 0.2, z + oz + 0.1 + (c % 2) * 0.2, shade(P.hardcore, 0.94 + 0.05 * c));
          }
        }
      }
    } else {                                            // steel sections
      for (const oz of [-2.2, 2.2]) m.bar(x - 4.2, x + 4.2, y, y + 0.2, z + oz - 0.16, z + oz + 0.16, P.timber);
      for (let i = 0; i < 4; i += 1) {
        const cz = z - 0.9 + (i % 2) * 0.55;
        // Rolled sections, chamfered, because the arris down a flange is the
        // brightest line on a stack of steel in the open.
        if (d0.fine) {
          m.beam(x - 4.5, x + 4.5, y + 0.2 + i * 0.42, y + 0.55 + i * 0.42, cz, cz + 0.55, P.primer, 0.018);
          m.bar(x + 4.3, x + 4.52, y + 0.24 + i * 0.42, y + 0.51 + i * 0.42, cz + 0.06, cz + 0.49, shade(P.safety, 1.1));
        } else {
          m.bar(x - 4.5, x + 4.5, y + 0.2 + i * 0.42, y + 0.55 + i * 0.42, cz, cz + 0.55, P.primer);
        }
      }
      if (d0.fine) {
        for (const sx of [-2.6, 2.6]) {
          m.bar(x + sx - 0.04, x + sx + 0.04, y + 0.18, y + 2.1, z - 0.96, z - 0.88, shade(P.dark, 1.3));
        }
      }
    }
  }

  /// Tower crane. The one piece of site plant with a real silhouette, and the
  /// one place the status has to be honest: a site that is not being worked
  /// parks its jib over the compound and lowers the hook to the deck instead of
  /// leaving a load hanging. Nothing here moves — the pose is a function of the
  /// record, not of anything elapsing.
  function towerCrane(m, x, z, h, jib, working, d0, slew) {
    const t = 0.85;
    // At map scale a crane is a vertical line with a horizontal one across the
    // top of it, and spending 250 triangles on lattice that resolves to two
    // pixels is how a map budget gets eaten. The pose still reads: a parked jib
    // sits over the compound and the hook is on the deck.
    if (!d0.fine) {
      m.save().move(x, GRADE, z);
      m.bar(-1.9, 1.9, 0, 0.7, -1.9, 1.9, P.concreteWet);
      m.bar(-0.7, 0.7, 0.7, h, -0.7, 0.7, P.safety);
      m.save().move(0, h, 0).rotY(slew);
      m.bar(-0.55, 0.55, 0, 1.8, -8.6, jib, P.safety);
      m.bar(-1.4, 1.4, 0.5, 2.1, -8.6, -7.2, P.concreteWet);
      const trolleyFar = working ? jib * 0.62 : jib * 0.3;
      m.bar(-0.09, 0.09, working ? -h * 0.42 : -h + 1.0, 1.0, trolleyFar - 0.09, trolleyFar + 0.09, P.dark);
      m.restore();
      m.restore();
      return;
    }
    // A LATTICE, not a stick. A tower crane is the tallest thing on any site
    // here and the one object a viewer will look at against the sky, where a
    // solid box reads as a chimney and a lattice reads as a crane from any
    // distance the mast subtends more than a couple of pixels. Chords are round
    // tube and come out smooth-shaded for nothing; the bracing is real bracing,
    // put in by `latticeBay` so the mast and the jib are braced the same way.
    const seg = 5, chord = 0.13;
    m.save().move(x, GRADE, z);
    for (let i = 0; i < 4; i += 1) {                              // ballast
      const bx = (i % 2 ? 1 : -1) * 2.6, bz = (i < 2 ? 1 : -1) * 2.6;
      m.beam(bx - 1.6, bx + 1.6, 0, 0.7, bz - 1.6, bz + 1.6, P.concreteWet, 0.04);
    }
    m.beam(-1.5, 1.5, 0.7, 0.95, -1.5, 1.5, shade(P.concreteWet, 1.06), 0.03);   // base frame
    for (const sx of [-1, 1]) {
      for (const sz of [-1, 1]) {
        m.rod([sx * t, 0.7, sz * t], [sx * t, h, sz * t], chord, seg, P.safety, false);
      }
    }
    const bays = 10;
    for (let i = 0; i < bays; i += 1) {
      const y0 = 0.95 + ((h - 1.2) * i) / bays, y1 = 0.95 + ((h - 1.2) * (i + 1)) / bays;
      latticeBay(m, y0, y1, t, t, 0.06, 6, shade(P.safety, 0.95));
      // Climbing ladder, one face of the mast, in a cage of hoops.
      m.rod([-0.09, y0, -t - 0.22], [-0.09, y1, -t - 0.22], 0.035, 6, P.galv, false);
      m.rod([0.09, y0, -t - 0.22], [0.09, y1, -t - 0.22], 0.035, 6, P.galv, false);
      m.rod([-0.09, (y0 + y1) / 2, -t - 0.22], [0.09, (y0 + y1) / 2, -t - 0.22], 0.025, 6, P.galv, false);
    }
    // The slew: a fixed angle, chosen per site from its own index. Two sites do
    // not point the same way, and one site points the same way every time.
    m.save().move(0, h, 0).rotY(slew);
    m.column(0, -0.35, 0, 0.5, 1.15, 1.15, 10, shade(P.dark, 1.3), true);        // slew bearing
    m.beam(-1.1, 1.1, 0.15, 1.5, -1.1, 1.1, P.safety, 0.03);                     // turntable
    m.bar(-0.9, 0.9, 0.2, 1.9, 1.1, 2.4, shade(P.hut, 0.95));                    // cab
    m.beam(-0.98, 0.98, 1.86, 2.02, 1.02, 2.5, shade(P.hut, 0.8), 0.02);
    m.bar(-0.7, 0.7, 0.8, 1.7, 2.4, 2.46, P.glass);
    m.bar(-0.86, 0.86, 0.75, 0.82, 1.15, 2.5, shade(P.galv, 0.9));
    // A-frame over the turntable, and the pendant ties that hang the jib and the
    // counter-jib off it. Without them a jib is a plank balanced on a post.
    const apex = 5.2;
    for (const sx of [-1, 1]) {
      m.rod([sx * 0.75, 1.5, -0.75], [0, apex, 0], 0.085, 6, shade(P.safety, 1.04), false);
      m.rod([sx * 0.75, 1.5, 0.75], [0, apex, 0], 0.085, 6, shade(P.safety, 1.04), false);
    }
    m.rod([0, apex, 0], [0, 1.72, jib * 0.42], 0.05, 6, P.dark, false);
    m.rod([0, apex, 0], [0, 1.72, jib * 0.86], 0.05, 6, P.dark, false);
    m.rod([0, apex, 0], [0, 1.9, -7.6], 0.05, 6, P.dark, false);
    // Jib: two bottom chords and a top chord, braced bay by bay.
    for (const sx of [-0.45, 0.45]) {
      m.rod([sx, 1.4, 0], [sx, 1.4, jib], 0.075, seg, shade(P.safety, 1.05), false);
    }
    m.rod([0, 1.72, 0], [0, 1.72, jib], 0.075, seg, shade(P.safety, 0.98), false);
    const webs = 12;
    for (let i = 0; i < webs; i += 1) {
      const za = (jib * i) / webs, zb = (jib * (i + 1)) / webs;
      m.rod([-0.45, 1.4, zb], [0.45, 1.4, zb], 0.045, 6, shade(P.safety, 0.88), false);
      m.rod([-0.45, 1.4, za], [0, 1.72, zb], 0.04, 6, shade(P.safety, 0.84), false);
      m.rod([0.45, 1.4, za], [0, 1.72, zb], 0.04, 6, shade(P.safety, 0.84), false);
      m.rod([-0.45, 1.4, za], [0.45, 1.4, zb], 0.04, 6, shade(P.safety, 0.8), false);
    }
    m.column(0, 1.56, jib - 0.1, 0.16, 0.34, 0.34, 8, P.dark, true);              // jib-head sheave
    // Counter-jib, braced the same way, with the counterweight in slabs.
    for (const sx of [-1.15, 1.15]) {
      m.rod([sx, 1.35, -1.1], [sx, 1.35, -7.9], 0.08, seg, shade(P.safety, 0.92), false);
      m.rod([sx, 2.05, -1.1], [sx, 2.05, -7.9], 0.08, seg, shade(P.safety, 1.0), false);
    }
    for (let i = 0; i < 4; i += 1) {
      const zz = -1.6 - i * 1.6;
      m.rod([-1.15, 1.35, zz], [1.15, 1.35, zz], 0.05, 6, shade(P.safety, 0.86), false);
      m.rod([-1.15, 2.05, zz], [-1.15, 1.35, zz - 1.4], 0.04, 6, shade(P.safety, 0.82), false);
      m.rod([1.15, 2.05, zz], [1.15, 1.35, zz - 1.4], 0.04, 6, shade(P.safety, 0.82), false);
    }
    for (let i = 0; i < 3; i += 1) {
      m.beam(-1.5, 1.5, 0.6, 2.2, -8.6 + i * 0.42, -8.24 + i * 0.42, shade(P.concreteWet, 0.94 + 0.04 * i), 0.03);
    }
    m.bar(-1.0, 1.0, 2.1, 2.5, -6.6, -4.2, shade(P.galv, 0.94));                  // hoist machinery
    // Trolley, rope and hook. Working: hook up over the work, with a load on it.
    // Not working: hook down on the deck and nothing hanging.
    const trolley = working ? jib * 0.62 : jib * 0.3;
    const hookY = working ? -h * 0.42 : -h + 1.0;
    m.beam(-0.35, 0.35, 1.08, 1.45, trolley - 0.5, trolley + 0.5, P.dark, 0.02);
    for (const sx of [-0.24, 0.24]) m.column(sx, 1.14, trolley, 0.1, 0.19, 0.19, 8, shade(P.steel, 0.9), true);
    m.rod([0, hookY, trolley], [0, 1.1, trolley], 0.035, 6, P.dark, false);
    m.column(0, hookY - 0.12, trolley, 0.52, 0.26, 0.26, 8, shade(P.steel, 0.9), true);
    m.webPlate([[-0.16, hookY - 0.6], [0.16, hookY - 0.6], [0.16, hookY - 0.12], [-0.16, hookY - 0.12]],
      trolley - 0.05, trolley + 0.05, shade(P.steel, 0.8));
    if (working) {
      m.beam(-1.2, 1.2, hookY - 1.1, hookY - 0.5, trolley - 1.2, trolley + 1.2, P.timber, 0.03);
      for (const sx of [-1, 1]) {
        m.rod([0, hookY - 0.55, trolley], [sx * 1.1, hookY - 0.5, trolley + sx * 1.1], 0.03, 6, P.dark, false);
      }
    }
    m.restore();
    m.restore();
  }

  /// Lighting masts. A site works past dark; a finished plant lights its yard.
  /// Same mast at both ends of the job, which is the point of a shared kit.
  function lightMast(m, x, z, h, d0) {
    m.column(x, GRADE, z, h, 0.22, 0.14, d0.seg, P.galv, false);
    m.bar(x - 1.5, x + 1.5, GRADE + h, GRADE + h + 0.18, z - 0.2, z + 0.2, P.galv);
    // At map scale a mast is a stem, a crosshead and one lamp: the three
    // floodlights on it are 0.6 m apart and resolve to one mark. That is
    // twelve triangles a mast rather than twenty-four, on six masts.
    const lamps = d0.fine ? 3 : 1;
    for (let i = 0; i < lamps; i += 1) {
      const lx = x - 1.1 + (i * 2.2) / Math.max(1, lamps - 1 || 1);
      m.bar(lx - 0.32, lx + 0.32, GRADE + h - 0.35, GRADE + h, z - 0.45, z + 0.45, P.dark);
      if (!d0.fine) continue;
      m.bar(lx - 0.26, lx + 0.26, GRADE + h - 0.38, GRADE + h - 0.34, z - 0.4, z + 0.4, shade(P.lane, 1.15));
      // A hood over the lamp and a bracket under it. A floodlight without a
      // hood is a glowing brick, and the hood is what throws the shadow that
      // tells you which way the thing is pointing.
      m.bar(lx - 0.36, lx + 0.36, GRADE + h - 0.02, GRADE + h + 0.1, z - 0.62, z + 0.5, shade(P.dark, 1.5));
      m.rod([lx, GRADE + h, z + 0.4], [lx, GRADE + h - 0.3, z - 0.2], 0.04, 6, P.galv, false);
    }
    if (d0.fine) {
      // Base flange, holding-down bolts and the cable duct up the mast. The
      // three things that stop a mast being a stick pushed into the ground.
      m.beam(x - 0.42, x + 0.42, GRADE, GRADE + 0.09, z - 0.42, z + 0.42, shade(P.galv, 0.9), 0.02);
      for (let b = 0; b < 4; b += 1) {
        const bx = x + (b % 2 ? 0.29 : -0.29), bz = z + (b < 2 ? 0.29 : -0.29);
        m.rod([bx, GRADE + 0.09, bz], [bx, GRADE + 0.2, bz], 0.032, 6, P.steel, true);
      }
      m.bar(x - 0.09, x + 0.09, GRADE + 0.2, GRADE + 1.5, z + 0.14, z + 0.3, shade(P.galv, 0.84));
      m.rod([x, GRADE + h * 0.52, z], [x, GRADE + h * 0.52 + 0.14, z], 0.21, d0.seg, shade(P.galv, 0.86), false);
    }
  }

  /// The honest overlay, on the ground rather than in the HUD: a stop board at
  /// the gate. Paused and blocked are different words in the sim and different
  /// boards here, and neither of them is a crane that looks busy.
  function stopBoard(m, x, z, blocked, d0) {
    m.save().move(x, GRADE, z);
    // One post at map range, not two. A 150 mm sign leg is under a pixel there
    // and the BOARD is the whole of what the stop notice has to say; the twelve
    // triangles the second leg cost came off the one stage that has to leave
    // room for a finished site to be bigger than it.
    for (const sx of d0.fine ? [-1.1, 1.1] : [0]) {
      if (d0.fine) m.rod([sx, 0, 0], [sx, 2.3, 0], 0.075, 6, P.galv, true);
      else m.bar(sx - 0.09, sx + 0.09, 0, 2.3, -0.09, 0.09, P.galv);
    }
    if (d0.fine) {
      m.webPlate([[-1.5, 1.3], [1.5, 1.3], [1.5, 2.5], [-1.5, 2.5]], -0.05, 0.05, blocked ? P.stopped : P.safety);
      for (let i = 0; i < 3; i += 1) m.bar(-1.2, 1.2, 1.5 + i * 0.3, 1.62 + i * 0.3, 0.05, 0.08, shade(P.hut, 1));
      m.beam(-0.7, 0.7, 0, 0.34, -0.7, 0.7, P.concreteWet, 0.03);
      // A stopped site is coned off as well as signed. Two cones and a barrier
      // in front of the board, so it reads as closed from behind as well.
      for (const cx of [-2.2, 2.2]) {
        m.column(cx, 0, 0.9, 0.7, 0.34, 0.09, 8, P.safety, true);
        m.bar(cx - 0.42, cx + 0.42, 0, 0.07, 0.9 - 0.42, 0.9 + 0.42, shade(P.safety, 0.82));
      }
      m.beam(-2.2, 2.2, 0.55, 0.75, 0.85, 0.95, shade(P.hut, 1.02), 0.015);
    } else {
      m.bar(-1.5, 1.5, 1.3, 2.5, -0.06, 0.06, blocked ? P.stopped : P.safety);
    }
    m.restore();
  }

  /// Concrete batching skid and a bowser. Small, and it is the difference
  /// between a foundation stage that reads as work and one that reads as a
  /// tidy diagram.
  function sitePlant(m, x, z, d0) {
    // A batching skid is a silo standing on legs over a weigh hopper, with a
    // chute into whatever is under it. The silo is a smooth cylinder over a
    // cone, which is the shape stored powder actually needs and is now free to
    // shade correctly.
    m.column(x, GRADE + 1.8, z, 3.4, 1.5, 1.5, d0.seg, P.galv, true);
    m.column(x, GRADE + 1.0, z, 0.8, 0.45, 1.5, d0.seg, shade(P.galv, 0.92), false);
    m.bar(x - 1.8, x + 1.8, GRADE + 5.2, GRADE + 5.6, z - 1.8, z + 1.8, shade(P.galv, 0.86));
    for (let i = 0; i < 4; i += 1) {
      const a = i * 90 * DEG;
      const lx = x + Math.cos(a) * 1.55, lz = z + Math.sin(a) * 1.55;
      m.rod([lx, GRADE, lz], [lx, GRADE + 2.2, lz], 0.11, 6, P.steel, false);
      if (d0.fine) {
        m.bar(lx - 0.24, lx + 0.24, GRADE, GRADE + 0.12, lz - 0.24, lz + 0.24, shade(P.steel, 0.9));
        m.rod([lx, GRADE + 2.0, lz], [x + Math.cos(a + 1.57) * 1.55, GRADE + 0.4, z + Math.sin(a + 1.57) * 1.55],
          0.06, 5, shade(P.steel, 0.9), false);
      }
    }
    if (d0.fine) {
      m.column(x, GRADE + 0.55, z, 0.5, 0.7, 0.45, d0.seg, shade(P.safety, 0.94), true);   // weigh hopper
      m.rod([x, GRADE + 1.0, z], [x + 2.0, GRADE + 0.4, z], 0.22, 8, shade(P.galv, 1.02), true);
      // Filter head and access ladder, which is how the top of a silo is
      // reached and the only thing that gives it a scale.
      m.column(x + 0.6, GRADE + 5.6, z, 0.9, 0.42, 0.42, d0.seg, shade(P.galv, 1.04), true);
      for (const sx of [-0.22, 0.22]) m.rod([x + sx, GRADE + 0.2, z - 1.62], [x + sx, GRADE + 5.7, z - 1.62], 0.035, 6, P.galv, false);
      for (let r = 0; r < 9; r += 1) {
        m.rod([x - 0.22, GRADE + 0.5 + r * 0.6, z - 1.62], [x + 0.22, GRADE + 0.5 + r * 0.6, z - 1.62], 0.025, 6, P.galv, false);
      }
    }
    // The bowser beside it.
    m.bar(x + 3.4, x + 7.0, GRADE + 0.55, GRADE + 1.9, z - 1.1, z + 1.1, P.safety);
    if (d0.fine) {
      m.beam(x + 3.3, x + 7.1, GRADE + 1.85, GRADE + 2.0, z - 1.2, z + 1.2, shade(P.safety, 0.8), 0.02);
      m.rod([x + 3.5, GRADE + 1.25, z], [x + 6.9, GRADE + 1.25, z], 0.62, 10, shade(P.galv, 1.0), true);
      for (const wx of [x + 4.1, x + 6.3]) {
        for (const wz of [z - 1.05, z + 1.05]) {
          m.rod([wx, GRADE + 0.45, wz - 0.1], [wx, GRADE + 0.45, wz + 0.1], 0.45, 10, P.dark, true);
        }
      }
      m.bar(x + 2.6, x + 3.5, GRADE + 0.9, GRADE + 1.2, z - 0.12, z + 0.12, shade(P.steel, 0.95));
    }
  }

  /// Scaffold on one gable while the sheeting goes up. Present only in the
  /// enclosing stage, which is the only time it would be there.
  /// Scaffold, in the words it is actually built out of: standards up, ledgers
  /// along, transoms across, boards on the transoms, a guardrail and a toe board
  /// at every lift, facade bracing on the diagonal and a base plate under every
  /// standard. WHY THAT MUCH: scaffold is the one thing on a site made almost
  /// entirely of thin round tube, and thin round tube is where flat shading and
  /// square section were costing the most — it read as a stack of planks. Every
  /// member here is a `rod`, so it is round, and the smoothing pass makes it so
  /// for no triangles. The map path keeps the old plank-and-post massing.
  function scaffold(m, x, z, w, h, d0) {
    // The map scaffold is one lift on one bay: at that size it is a texture
    // against the gable, and the gable is what carries the stage.
    const lifts = d0.fine ? 4 : 1, bays = d0.fine ? 5 : 1;
    if (!d0.fine) {
      for (let i = 0; i <= bays; i += 1) {
        const px = x - w / 2 + (i * w) / bays;
        for (const pz of [z, z + 1.3]) m.bar(px - 0.06, px + 0.06, GRADE, GRADE + h, pz - 0.06, pz + 0.06, P.galv);
      }
      for (let i = 1; i <= lifts; i += 1) {
        const y = (h * i) / lifts;
        m.bar(x - w / 2, x + w / 2, GRADE + y, GRADE + y + 0.08, z - 0.06, z + 1.36, P.timber);
        m.bar(x - w / 2, x + w / 2, GRADE + y + 0.9, GRADE + y + 0.98, z + 1.24, z + 1.36, P.galv);
      }
      return;
    }
    const rows = [z, z + 1.3], tube = 0.033, seg = 5;
    for (let i = 0; i <= bays; i += 1) {
      const px = x - w / 2 + (i * w) / bays;
      for (const pz of rows) {
        m.rod([px, GRADE + 0.1, pz], [px, GRADE + h + 1.05, pz], tube, seg, P.galv, false);
        m.bar(px - 0.13, px + 0.13, GRADE, GRADE + 0.1, pz - 0.13, pz + 0.13, shade(P.steel, 0.95));  // sole plate
      }
    }
    for (let i = 1; i <= lifts; i += 1) {
      const y = GRADE + (h * i) / lifts;
      for (const pz of rows) {
        m.rod([x - w / 2, y, pz], [x + w / 2, y, pz], tube, seg, shade(P.galv, 0.94), false);
        m.rod([x - w / 2, y + 0.95, pz], [x + w / 2, y + 0.95, pz], tube, seg, shade(P.galv, 1.02), false);
      }
      for (let b = 0; b <= bays; b += 1) {
        const px = x - w / 2 + (b * w) / bays;
        m.rod([px, y, rows[0] - 0.16], [px, y, rows[1] + 0.16], tube, 6, shade(P.galv, 0.88), false);
      }
      // Four boards to the lift, laid on the transoms, and the toe board that
      // stops anything rolling off them.
      for (let b = 0; b < 4; b += 1) {
        const bz = rows[0] - 0.1 + b * 0.4;
        m.bar(x - w / 2, x + w / 2, y + 0.04, y + 0.09, bz, bz + 0.37, shade(P.timber, 0.94 + 0.05 * (b % 2)));
      }
      m.bar(x - w / 2, x + w / 2, y + 0.09, y + 0.34, rows[1] + 0.14, rows[1] + 0.2, shade(P.timber, 0.86));
      // Facade brace, one diagonal a lift, alternating hand as a braced facade
      // actually does.
      const d = i % 2 ? 1 : -1;
      m.rod([x - (d * w) / 2, y, rows[1]], [x + (d * w) / 2, y + h / lifts, rows[1]], tube, 6, shade(P.galv, 0.82), false);
    }
    // Ties back to the building, without which the whole thing is a ladder
    // leaning on nothing.
    for (const px of [x - w * 0.3, x + w * 0.3]) {
      for (let i = 1; i <= lifts; i += 2) {
        m.rod([px, GRADE + (h * i) / lifts, rows[0]], [px, GRADE + (h * i) / lifts, z - 1.4], tube, 6, shade(P.galv, 0.9), false);
      }
    }
  }

  // ------------------------------------------------ groundworks, plant, yard
  // THE SECOND DETAIL PASS, and every function in it is CLOSE RANGE ONLY: each
  // one returns on its first line unless `d0.fine`, so the map mesh is byte for
  // byte the mesh it was. The far LOD is what the globe overlay pays for, its
  // worst kind sits forty-four triangles under a hard eight hundred, and it did
  // not ask for any of this.
  //
  // WHAT THESE BUY. The first pass drew a building and a fence round it. A site
  // is not that: it is a drainage trench with pipe bedded in it, a machine
  // standing over the dig, a skip line, pipework and ducting hung off the
  // structure, a stair somebody climbs to reach the roof, and paint on the
  // ground telling a lorry where to stop. Every one of those changes the
  // SILHOUETTE rather than the surface, which is the only kind of detail that
  // survives on a 1124x102 strip — a rib you cannot resolve is wasted, a stack
  // against the sky is not.
  //
  // AND THEY ARE STAGE-APPROPRIATE, which is the harder half of the brief. The
  // trench, the excavator and the dumper belong to the two stages before
  // anything stands and are gone by the frame stage; the access stair arrives
  // with the steel; the pipework and ducting arrive with commissioning; the
  // markings and the bunded store arrive at hand-over. Nothing here gives an
  // early site a finished building.

  /// Below-ground drainage, open. The first fortnight of every one of these
  /// jobs is a trench, and a trench with bedding in it, pipe on the bedding, a
  /// manhole every so often and a barrier down the open side is the most
  /// legible thing an early site has to show.
  ///
  /// The cut sides are battered and drawn ONCE, then mirrored in Z: `tri`
  /// re-winds under a negative determinant, so the far bank faces into the
  /// trench by construction instead of by a hand-listed quad somebody has to
  /// get right twice.
  function drainage(m, x0, x1, z, d0) {
    if (!d0.fine) return;
    const bot = Math.max(PIT + 0.04, GRADE - 1.0), hw = 0.62, lip = 1.05;
    m.save().move(0, 0, z);
    for (const s of [-1, 1]) {
      m.save();
      if (s < 0) m.scale(1, 1, -1);
      m.quad([x0, GRADE, lip], [x1, GRADE, lip], [x1, bot, hw], [x0, bot, hw], shade(P.earthCut, 0.98));
      m.restore();
    }
    // The floor of a trench is churned, same as the floor of a dig, and for the
    // same reason: with no textures a single quad reads as a painted stripe.
    groundPatch(m, (x0 + x1) / 2, 0, x1 - x0, hw * 2, 10, 2, bot, P.earthCut, 0.08);
    m.bar(x0, x1, bot, bot + 0.16, -0.44, 0.44, P.hardcore);                  // bedding
    // Pipe in lengths with a collar at every joint. One continuous tube is a
    // rod; the collars are what make it drainage.
    const runs = Math.max(3, Math.round((x1 - x0) / 3.4));
    for (let i = 0; i < runs; i += 1) {
      const a = x0 + (i * (x1 - x0)) / runs, b = x0 + ((i + 1) * (x1 - x0)) / runs;
      m.rod([a + 0.08, bot + 0.44, 0], [b - 0.08, bot + 0.44, 0], 0.26, 10, shade(P.concreteWet, 1.0), true);
      m.rod([b - 0.26, bot + 0.44, 0], [b + 0.04, bot + 0.44, 0], 0.32, 10, shade(P.concreteWet, 0.84), false);
    }
    // Manholes: precast rings, a cover slab across them, and the frame and
    // cover sitting proud of the fill where a wheel will find it.
    for (let i = 0; i < 2; i += 1) {
      const mx = x0 + ((i + 0.7) * (x1 - x0)) / 2.4;
      for (let r = 0; r < 3; r += 1) {
        const y = bot + 0.1 + r * ((GRADE - bot - 0.42) / 3);
        m.column(mx, y, 0, (GRADE - bot - 0.42) / 3 - 0.04, 0.82 - r * 0.02, 0.82 - r * 0.02, 12,
          shade(P.concreteWet, 0.92 + 0.05 * r), false);
      }
      m.beam(mx - 0.92, mx + 0.92, GRADE - 0.32, GRADE - 0.1, -0.92, 0.92, shade(P.concrete, 0.96), 0.03);
      m.beam(mx - 0.46, mx + 0.46, GRADE - 0.1, GRADE + 0.06, -0.46, 0.46, shade(P.dark, 1.18), 0.025);
      m.bar(mx - 0.34, mx + 0.34, GRADE + 0.06, GRADE + 0.08, -0.34, 0.34, shade(P.steel, 0.9));
      for (let s = 0; s < 3; s += 1) {                                        // step irons down the shaft
        m.rod([mx - 0.2, bot + 0.5 + s * 0.32, -0.62], [mx + 0.2, bot + 0.5 + s * 0.32, -0.62], 0.03, 6, P.galv, false);
      }
    }
    // Arisings along the far bank, and pipe stock waiting to go in on the near
    // one. A trench with no spoil beside it came from nowhere.
    for (let i = 0; i < 7; i += 1) {
      m.mound(x0 + 1.2 + (i * (x1 - x0 - 2.4)) / 6, GRADE, 2.4, 1.5, 0.95, 6, i % 2 ? P.earth : P.earthCut);
    }
    for (const bz of [-3.2, -2.2]) m.bar(x0 + 1.0, x0 + 8.0, GRADE, GRADE + 0.16, bz - 0.16, bz + 0.16, P.timber);
    for (let i = 0; i < 3; i += 1) {
      const py = GRADE + 0.42 + (i > 1 ? 0.5 : 0), px = -2.7 + (i % 2) * 0.56 + (i > 1 ? 0.28 : 0);
      m.rod([x0 + 1.1, py, px], [x0 + 7.9, py, px], 0.26, 10, shade(P.concreteWet, 0.96), false);
    }
    // Pedestrian barrier down the open side. This is a hole somebody could fall
    // into and the barrier is what says the site knows it.
    for (let i = 0; i < 8; i += 1) {
      const bx = x0 + 0.6 + (i * (x1 - x0 - 1.2)) / 7;
      m.rod([bx, GRADE, -1.55], [bx, GRADE + 1.1, -1.55], 0.045, 6, P.safety, false);
      m.bar(bx - 0.28, bx + 0.28, GRADE, GRADE + 0.08, -1.8, -1.3, shade(P.steel, 0.92));
    }
    for (const y of [GRADE + 0.55, GRADE + 1.06]) {
      m.bar(x0 + 0.5, x1 - 0.5, y, y + 0.11, -1.6, -1.5, shade(P.safety, y > GRADE + 0.8 ? 1.1 : 0.86));
    }
    m.restore();
  }

  /// A tracked excavator. The one machine every one of these sites has, and the
  /// one object that says WORK rather than "a building will be here". Tracks
  /// with shoes on them, a slewing house with a counterweight and a cab, and a
  /// boom, dipper and bucket carried on rams.
  ///
  /// THE POSE IS THE RECORDED STATUS AND NOTHING ELSE. A working machine has
  /// the boom up and the bucket held over the dig; a stopped one has it lowered
  /// with the bucket flat on the ground and the house squared up, which is how
  /// plant is left when nobody is paying for it. Both poses cost exactly the
  /// same triangles, so the stop cannot move a budget either.
  function excavator(m, x, z, rot, working, d0) {
    if (!d0.fine) return;
    const seg = 8;
    m.save().move(x, GRADE, z).rotY(rot);
    // Undercarriage. A track is a frame with an idler at one end, a sprocket at
    // the other and shoes wrapped round both; the shoe rhythm is the whole read
    // of it and it is the cheapest thing on the machine.
    for (const s of [-1, 1]) {
      const tx = s * 1.18;
      m.beam(tx - 0.26, tx + 0.26, 0.34, 0.84, -2.2, 2.2, shade(P.safety, 0.84), 0.03);
      for (const ez of [-2.2, 2.2]) {
        m.rod([tx - 0.32, 0.5, ez], [tx + 0.32, 0.5, ez], 0.46, seg, shade(P.dark, 1.12), true);
      }
      for (let r = 0; r < 4; r += 1) {
        m.rod([tx - 0.24, 0.26, -1.35 + r * 0.9], [tx + 0.24, 0.26, -1.35 + r * 0.9], 0.2, 6, shade(P.steel, 0.9), true);
      }
      for (let i = 0; i < 11; i += 1) {
        const sz = -2.5 + (i * 5.0) / 10;
        m.bar(tx - 0.36, tx + 0.36, 0, 0.1, sz - 0.17, sz + 0.17, shade(P.dark, 0.94 + 0.14 * (i % 2)));
        m.bar(tx - 0.36, tx + 0.36, 0.9, 1.0, sz - 0.17, sz + 0.17, shade(P.dark, 0.86 + 0.14 * (i % 2)));
      }
    }
    m.beam(-1.25, 1.25, 0.46, 0.94, -1.3, 1.3, shade(P.steel, 0.84), 0.03);
    m.column(0, 0.94, 0, 0.2, 0.84, 0.84, 12, shade(P.dark, 1.08), true);
    // The house, slewed off the tracks. Counterweight at the back, engine cover
    // beside the cab, glazing on two faces and a walkway rail round the deck.
    m.save().move(0, 1.14, 0).rotY(working ? 24 : -6);
    m.beam(-1.28, 1.28, 0, 1.0, -2.1, 1.45, shade(P.safety, 0.96), 0.04);
    m.beam(-1.32, 1.32, -0.42, 0.96, -2.6, -2.0, shade(P.dark, 1.02), 0.05);
    m.beam(-1.2, 1.2, 1.0, 1.46, -1.95, -0.15, shade(P.safety, 1.06), 0.035);
    m.bar(-1.24, -0.18, 1.0, 2.44, 0.05, 1.45, shade(P.safety, 1.02));
    m.bar(-1.22, -0.2, 1.34, 2.24, 1.45, 1.5, P.glass);
    m.bar(-1.28, -1.24, 1.34, 2.24, 0.15, 1.45, P.glass);
    m.beam(-1.32, -0.1, 2.44, 2.56, -0.02, 1.54, shade(P.safety, 0.84), 0.02);
    m.column(0.78, 1.46, -1.45, 0.66, 0.1, 0.085, 8, shade(P.dark, 1.2), true);
    for (const rz of [-1.9, -0.4]) {
      m.rod([1.28, 1.0, rz], [1.28, 1.9, rz], 0.035, 6, P.galv, false);
    }
    m.rod([1.28, 1.9, -1.9], [1.28, 1.9, -0.4], 0.035, 6, shade(P.galv, 1.05), false);
    // Boom, dipper and bucket. The angles are the only thing the status moves.
    const boomUp = working ? 42 : 12, dipAt = working ? 66 : 34, bucketAt = working ? 34 : -22;
    m.save().move(0.3, 0.36, 1.35).rotX(-boomUp);
    m.beam(-0.28, 0.28, -0.3, 0.3, 0, 4.4, shade(P.safety, 1.0), 0.035);
    for (const hs of [-1, 1]) {
      m.rod([hs * 0.32, 0.24, 0.3], [hs * 0.32, 0.24, 4.1], 0.05, 6, shade(P.dark, 1.1), false);
    }
    m.rod([0, -0.62, 0.1], [0, -0.28, 2.3], 0.19, 8, shade(P.steel, 0.94), true);
    m.rod([0, -0.3, 2.1], [0, -0.1, 3.1], 0.095, 8, shade(P.galv, 1.12), true);
    m.save().move(0, 0, 4.4).rotX(dipAt);
    m.beam(-0.22, 0.22, -0.26, 0.26, 0, 2.4, shade(P.safety, 0.94), 0.03);
    m.rod([0, 0.5, 0.15], [0, 0.24, 1.5], 0.15, 8, shade(P.steel, 0.94), true);
    m.rod([0, 0.26, 1.35], [0, 0.12, 2.2], 0.078, 8, shade(P.galv, 1.12), true);
    m.save().move(0, 0, 2.4).rotX(bucketAt);
    // The bucket, as its own section extruded across the machine. `rotY(90)`
    // puts the profile plane where `webPlate` wants it, which fixes the
    // handedness once instead of asking the call site to wind a shell by hand.
    m.save().rotY(90);
    m.webPlate([[0.0, 0.0], [-0.1, -0.98], [-0.7, -1.28], [-1.28, -0.95], [-1.32, -0.15]],
      -0.62, 0.62, shade(P.safety, 0.86));
    m.restore();
    for (let t = 0; t < 5; t += 1) {
      const tx = -0.5 + t * 0.25;
      m.beam(tx - 0.06, tx + 0.06, -1.42, -1.2, 0.55, 0.85, shade(P.steel, 1.02), 0.012);
    }
    m.restore();
    m.restore();
    m.restore();
    m.restore();
    m.restore();
  }

  /// A site dumper. The other half of an earthworks job: something has to take
  /// what the excavator digs out. Skip forward over the front axle, four wheels
  /// that are round, a roll-over frame and a beacon.
  function dumper(m, x, z, rot, d0) {
    if (!d0.fine) return;
    m.save().move(x, GRADE, z).rotY(rot);
    m.beam(-0.92, 0.92, 0.52, 0.84, -1.9, 2.0, shade(P.steel, 0.9), 0.03);
    for (const wz of [-1.2, 1.35]) {
      for (const s of [-1, 1]) {
        m.rod([s * 0.82, 0.6, wz], [s * 1.16, 0.6, wz], 0.6, 10, P.dark, true);
        m.rod([s * 0.98, 0.6, wz], [s * 1.14, 0.6, wz], 0.28, 8, shade(P.steel, 1.02), true);
      }
      m.rod([-0.88, 0.6, wz], [0.88, 0.6, wz], 0.11, 6, shade(P.steel, 0.86), false);
    }
    // The skip. Tapered, because a straight-sided box is a crate.
    m.save().rotY(90);
    m.webPlate([[-2.35, 0.92], [-0.55, 0.92], [-0.25, 2.1], [-2.6, 2.1]], -1.02, 1.02, shade(P.safety, 1.04));
    m.restore();
    m.beam(-1.08, 1.08, 2.06, 2.2, -2.66, -0.2, shade(P.safety, 0.82), 0.025);
    m.bar(-0.86, 0.86, 0.84, 1.5, 0.6, 1.9, shade(P.safety, 0.94));            // engine cover
    m.bar(-0.5, 0.5, 1.5, 2.02, 1.0, 1.5, shade(P.dark, 1.1));                 // seat
    for (const s of [-1, 1]) {
      m.rod([s * 0.76, 1.5, 0.7], [s * 0.76, 3.0, 0.7], 0.058, 6, P.safety, false);
      m.rod([s * 0.76, 1.5, 1.9], [s * 0.76, 3.0, 1.9], 0.058, 6, P.safety, false);
      m.rod([s * 0.76, 3.0, 0.7], [s * 0.76, 3.0, 1.9], 0.058, 6, shade(P.safety, 1.08), false);
    }
    m.rod([-0.76, 3.0, 1.3], [0.76, 3.0, 1.3], 0.058, 6, shade(P.safety, 1.08), false);
    m.column(0.58, 3.0, 1.9, 0.24, 0.11, 0.11, 8, shade(P.stopped, 1.15), true);
    m.restore();
  }

  /// Skips. Waste segregation is three open containers, chained down, one of
  /// them full. Built as five thin boxes rather than one solid, because a skip
  /// is OPEN and a solid wedge with a rail on top is a skip-shaped paperweight —
  /// you can see into these, which is the entire difference.
  function skipLine(m, x, z, d0) {
    if (!d0.fine) return;
    for (let i = 0; i < 3; i += 1) {
      const sz = z + i * 3.1, front = 1.02, back = 1.62;
      const col = shade(i === 1 ? P.safety : P.hoard, 0.88 + 0.09 * i);
      m.bar(x - 1.6, x + 1.6, GRADE + 0.16, GRADE + 0.26, sz - 1.15, sz + 1.15, shade(col, 0.8));
      m.bar(x - 1.64, x - 1.56, GRADE + 0.16, GRADE + back, sz - 1.2, sz + 1.2, col);
      m.bar(x + 1.56, x + 1.64, GRADE + 0.16, GRADE + back, sz - 1.2, sz + 1.2, shade(col, 0.94));
      m.bar(x - 1.64, x + 1.64, GRADE + 0.16, GRADE + back, sz + 1.14, sz + 1.22, shade(col, 0.9));
      m.bar(x - 1.64, x + 1.64, GRADE + 0.16, GRADE + front, sz - 1.22, sz - 1.14, shade(col, 1.06));
      m.beam(x - 1.7, x + 1.7, GRADE + back, GRADE + back + 0.12, sz - 1.26, sz + 1.26, shade(col, 1.14), 0.018);
      for (const hz of [sz - 0.5, sz + 0.5]) {                                 // lifting eyes and runners
        m.beam(x - 1.72, x - 1.6, GRADE + 0.6, GRADE + 1.05, hz - 0.07, hz + 0.07, shade(P.steel, 0.94), 0.012);
        m.beam(x + 1.6, x + 1.72, GRADE + 0.6, GRADE + 1.05, hz - 0.07, hz + 0.07, shade(P.steel, 0.94), 0.012);
      }
      m.beam(x - 1.66, x + 1.66, GRADE, GRADE + 0.16, sz - 1.0, sz - 0.7, shade(P.steel, 0.86), 0.02);
      m.beam(x - 1.66, x + 1.66, GRADE, GRADE + 0.16, sz + 0.7, sz + 1.0, shade(P.steel, 0.86), 0.02);
      if (i !== 1) continue;
      for (let f = 0; f < 4; f += 1) {                                          // the full one
        m.mound(x - 0.9 + (f % 2) * 1.5, GRADE + 1.1, sz - 0.5 + Math.floor(f / 2) * 1.0, 0.85, 0.75, 6,
          shade(f % 2 ? P.timber : P.hardcore, 0.9 + 0.08 * f));
      }
    }
  }

  /// The external access stair. Every industrial building on this list has one,
  /// nothing on it was drawn before, and it is the single cheapest thing that
  /// changes a GABLE SILHOUETTE from a triangle to a building somebody works
  /// in: two flights with a half landing between them, stringers at the pitch,
  /// level treads, a handrail with real posts, and a hooped ladder off the top
  /// landing to the roof. It arrives with the steel — primed at the frame
  /// stage, galvanised once the building is clad — and it never leaves.
  function accessStair(m, x, z, y0, top, d0, col) {
    if (!d0.fine) return;
    const wide = 1.15, rise = (top - y0) / 2, len = rise * 1.5;
    for (let f = 0; f < 2; f += 1) {
      const dir = f ? -1 : 1;
      const zBase = z + (f ? len : 0), yBase = y0 + f * rise;
      // Stringers, laid along the pitch. Drawn in a rotated frame so the member
      // is a real raking section and not a stepped stack of boxes.
      m.save().move(x, yBase, zBase).rotX(-dir * Math.atan2(rise, len) / DEG);
      const run = Math.hypot(rise, len);
      for (const s of [-1, 1]) {
        m.beam(s * (wide / 2) - 0.055, s * (wide / 2) + 0.055, -0.32, -0.02, 0, dir * run, col, 0.016);
      }
      m.restore();
      const steps = 7;
      for (let i = 0; i < steps; i += 1) {
        const t = (i + 0.5) / steps;
        const ty = yBase + rise * t, tz = zBase + dir * len * t;
        m.beam(x - wide / 2, x + wide / 2, ty, ty + 0.05, tz - 0.13, tz + 0.13, shade(col, 1.08), 0.011);
        m.bar(x - wide / 2 + 0.03, x + wide / 2 - 0.03, ty - 0.16, ty, tz + dir * 0.12, tz + dir * 0.16, shade(col, 0.82));
      }
      // Handrail: a raking top rail and a knee rail on posts, both sides.
      for (const s of [-1, 1]) {
        const px = x + s * (wide / 2 + 0.06);
        for (const hy of [1.02, 0.55]) {
          m.rod([px, yBase + hy, zBase], [px, yBase + rise + hy, zBase + dir * len], 0.036, 6,
            shade(P.galv, hy > 0.8 ? 1.06 : 0.92), false);
        }
        for (let p = 0; p <= 2; p += 1) {
          const t = p / 2;
          m.rod([px, yBase + rise * t, zBase + dir * len * t], [px, yBase + rise * t + 1.02, zBase + dir * len * t],
            0.036, 6, P.galv, false);
        }
      }
    }
    // Half landing at the head of the first flight, top landing at the head of
    // the second, both grated and both railed.
    for (const [ly, lz] of [[y0 + rise, z + len + 0.7], [top, z - 0.7]]) {
      gratedDeck(m, x, lz, wide + 0.3, 1.4, ly, d0, true);
      for (const s of [-1, 1]) {
        m.beam(x + s * (wide / 2 + 0.14) - 0.06, x + s * (wide / 2 + 0.14) + 0.06, ly - 0.42, ly - 0.26,
          lz - 0.7, lz + 0.7, shade(col, 0.9), 0.014);
        m.beam(x + s * (wide / 2 + 0.14) - 0.08, x + s * (wide / 2 + 0.14) + 0.08, GRADE, ly - 0.4,
          lz + 0.5, lz + 0.66, col, 0.018);
      }
    }
    const lad = z - 1.28;
    // Hooped cat ladder off the top landing. A roof nobody can reach is a roof
    // with nothing on it, and this file puts plant on every one of them.
    for (const s of [-1, 1]) {
      m.rod([x + s * 0.24, top, lad], [x + s * 0.24, top + 2.6, lad], 0.036, 6, P.galv, false);
    }
    for (let r = 0; r < 8; r += 1) {
      m.rod([x - 0.24, top + 0.24 + r * 0.3, lad], [x + 0.24, top + 0.24 + r * 0.3, lad], 0.026, 6,
        shade(P.galv, 0.96), false);
    }
    for (let hp = 0; hp < 3; hp += 1) {
      const hy = top + 0.8 + hp * 0.8;
      m.rod([x - 0.42, hy, lad], [x - 0.42, hy, lad - 0.56], 0.03, 6, shade(P.galv, 1.04), false);
      m.rod([x + 0.42, hy, lad], [x + 0.42, hy, lad - 0.56], 0.03, 6, shade(P.galv, 1.04), false);
      m.rod([x - 0.42, hy, lad - 0.56], [x + 0.42, hy, lad - 0.56], 0.03, 6, shade(P.galv, 1.04), false);
    }
  }

  /// External pipework, cable ladder and rainwater goods on an elevation. This
  /// is what an industrial building actually looks like from the yard and the
  /// first pass drew none of it: a bank of pipes on brackets at high level with
  /// flanged joints and one line turning up and over the eaves, a cable ladder
  /// above them with rungs you can count, downpipes into shoes, and a valve set
  /// standing on the ground where somebody can reach it.
  function wallServices(m, o, d0, k) {
    if (!d0.fine) return;
    const b = o.block, z = o.z + b.dz / 2, y = o.deck + Math.max(2.6, b.eaves - 2.6);
    const x0 = o.x - b.w * 0.42, x1 = o.x + b.w * 0.42;
    const lines = [[0.0, 0.19, shade(P.galv, 1.0)], [0.52, 0.15, shade(k.accent, 1.1)], [0.98, 0.12, shade(P.safety, 0.9)]];
    for (const [dy, r, col] of lines) {
      m.rod([x0, y + dy, z + 0.62], [x1, y + dy, z + 0.62], r, 10, col, false);
      for (let j = 0; j < 4; j += 1) {                                          // flanged joints
        const jx = x0 + ((j + 0.5) * (x1 - x0)) / 4;
        m.rod([jx - 0.07, y + dy, z + 0.62], [jx + 0.07, y + dy, z + 0.62], r * 1.4, 10, shade(col, 0.8), true);
      }
    }
    // Brackets. Pipework hanging on nothing is a stripe painted on a wall.
    for (let i = 0; i <= 5; i += 1) {
      const bx = x0 + (i * (x1 - x0)) / 5;
      m.beam(bx - 0.07, bx + 0.07, y - 0.34, y + 1.2, z + 0.04, z + 0.82, shade(P.steel, 0.92), 0.015);
      m.rod([bx, y - 0.3, z + 0.8], [bx, y - 0.9, z + 0.1], 0.045, 6, shade(P.steel, 0.88), false);
    }
    // One line turns up the gable and over the eaves onto the roof, which is
    // the bit that changes the outline against the sky.
    m.rod([x1, y, z + 0.62], [x1 + 0.9, y, z + 0.62], 0.19, 10, shade(P.galv, 1.0), false);
    m.rod([x1 + 0.9, y - 0.1, z + 0.62], [x1 + 0.9, o.deck + b.eaves + 1.2, z + 0.62], 0.19, 10, shade(P.galv, 1.06), false);
    m.rod([x1 + 0.9, o.deck + b.eaves + 1.2, z + 0.62], [x1 + 0.9, o.deck + b.eaves + 1.2, z - 1.6], 0.19, 10,
      shade(P.galv, 0.94), false);
    // Cable ladder: two side rails and rungs at 300, which is the one detail
    // that separates it from a duct.
    for (const s of [-1, 1]) {
      m.beam(x0, x1, y + 1.42, y + 1.56, z + 0.4 + s * 0.26, z + 0.5 + s * 0.26, shade(P.galv, 0.94), 0.014);
    }
    const rungs = Math.max(6, Math.round((x1 - x0) / 0.9));
    for (let i = 0; i < rungs; i += 1) {
      const rx = x0 + ((i + 0.5) * (x1 - x0)) / rungs;
      m.bar(rx - 0.05, rx + 0.05, y + 1.45, y + 1.53, z + 0.16, z + 0.74, shade(P.galv, 1.06));
    }
    // Rainwater goods: hopper, pipe, bracket, shoe. Four parts, and a building
    // without them sheds its roof onto its own wall.
    for (let i = 0; i < 3; i += 1) {
      const dx = o.x - b.w * 0.34 + (i * b.w * 0.68) / 2;
      m.rod([dx, GRADE + 0.34, z + 0.2], [dx, o.deck + b.eaves - 0.34, z + 0.2], 0.09, 8, shade(P.galv, 0.98), false);
      m.beam(dx - 0.3, dx + 0.3, o.deck + b.eaves - 0.34, o.deck + b.eaves + 0.04, z + 0.02, z + 0.42,
        shade(P.galv, 1.08), 0.02);
      m.rod([dx, GRADE + 0.34, z + 0.2], [dx, GRADE + 0.2, z + 0.62], 0.09, 8, shade(P.galv, 0.9), true);
      for (const by of [GRADE + 1.6, GRADE + 3.4]) {
        m.bar(dx - 0.14, dx + 0.14, by, by + 0.09, z, z + 0.26, shade(P.galv, 0.86));
      }
    }
    // A valve set at ground level: two risers, a manifold, gate bodies and
    // handwheels, on a plinth with a bollard in front of it.
    const vx = o.x + b.w * 0.30;
    m.beam(vx - 1.3, vx + 1.3, GRADE, GRADE + 0.26, z + 0.3, z + 1.5, shade(P.concrete, 0.94), 0.025);
    for (let i = 0; i < 2; i += 1) {
      const rx = vx - 0.7 + i * 1.4;
      m.rod([rx, GRADE + 0.26, z + 0.9], [rx, y - 0.1, z + 0.9], 0.14, 10, shade(P.galv, 1.02), false);
      m.beam(rx - 0.24, rx + 0.24, GRADE + 1.16, GRADE + 1.62, z + 0.66, z + 1.14, shade(k.accent, 0.9), 0.02);
      m.column(rx, GRADE + 1.62, z + 0.9, 0.1, 0.34, 0.34, 12, shade(P.stopped, 1.0), true);
      for (let s = 0; s < 3; s += 1) {
        const a = (s * 60) * DEG;
        m.rod([rx - Math.cos(a) * 0.32, GRADE + 1.67, z + 0.9 - Math.sin(a) * 0.32],
          [rx + Math.cos(a) * 0.32, GRADE + 1.67, z + 0.9 + Math.sin(a) * 0.32], 0.035, 6, shade(P.stopped, 1.1), false);
      }
    }
    m.rod([vx - 0.7, GRADE + 0.7, z + 0.9], [vx + 0.7, GRADE + 0.7, z + 0.9], 0.14, 10, shade(P.galv, 0.92), false);
    bollard(m, vx, z + 2.1, 1.0, 0.16, 8, P.safety);
  }

  /// Extract ducting and vent stacks. The top of an industrial building is
  /// never empty and it is the half of the silhouette a wide strip actually
  /// shows: a rectangular duct run on stools with a flanged joint every few
  /// metres, an elbow into a fan casing with a weather cowl, and two vent
  /// stacks with rain caps and stays.
  ///
  /// `stage` splits it honestly. The duct route goes in with the services at
  /// the enclosing stage; the fan and the stacks are commissioned and only
  /// stand once the building is handed over.
  function roofServices(m, o, stage, d0) {
    if (!d0.fine) return;
    const b = o.block, hw = b.w / 2;
    const off = Math.min(4.0, b.w * 0.30);
    const ry = o.deck + b.eaves + (b.ridge - b.eaves) * (1 - off / hw);
    const z0 = o.z - b.dz * 0.36, z1 = o.z + b.dz * 0.36;
    const dx0 = o.x - off - 0.8, dx1 = o.x - off + 0.8;
    // Stools, then the duct on them. A duct lying on a roof is a duct that
    // leaks; the stools are what hold it off the sheeting.
    const stools = Math.max(3, Math.round((z1 - z0) / 3.4));
    for (let i = 0; i <= stools; i += 1) {
      const sz = z0 + (i * (z1 - z0)) / stools;
      for (const sx of [dx0 + 0.16, dx1 - 0.16]) {
        m.beam(sx - 0.07, sx + 0.07, ry, ry + 0.52, sz - 0.12, sz + 0.12, shade(P.steel, 0.92), 0.014);
      }
      m.beam(dx0, dx1, ry + 0.46, ry + 0.58, sz - 0.14, sz + 0.14, shade(P.steel, 0.88), 0.016);
    }
    const runs = Math.max(3, Math.round((z1 - z0) / 3.0));
    for (let i = 0; i < runs; i += 1) {
      const a = z0 + (i * (z1 - z0)) / runs, c = z0 + ((i + 1) * (z1 - z0)) / runs;
      m.beam(dx0, dx1, ry + 0.58, ry + 1.5, a + 0.05, c - 0.05, shade(P.galv, 0.98 + 0.04 * (i % 2)), 0.03);
      m.beam(dx0 - 0.09, dx1 + 0.09, ry + 0.5, ry + 1.58, c - 0.08, c + 0.08, shade(P.galv, 1.08), 0.018);
    }
    // The elbow up out of the run, and the fan on top of it. Commissioned kit.
    if (stage >= 5) {
      m.beam(dx0, dx1, ry + 1.5, ry + 2.5, z1 - 1.5, z1 - 0.1, shade(P.galv, 1.02), 0.03);
      m.beam(dx0 - 0.12, dx1 + 0.12, ry + 2.5, ry + 2.66, z1 - 1.62, z1 + 0.02, shade(P.galv, 1.1), 0.02);
      m.column((dx0 + dx1) / 2, ry + 2.66, z1 - 0.8, 0.82, 0.66, 0.66, 14, shade(P.clad, 1.0), true);
      m.column((dx0 + dx1) / 2, ry + 3.48, z1 - 0.8, 0.34, 0.78, 0.5, 14, shade(P.dark, 1.15), true);
      for (const sx of [-0.5, 0.5]) {
        m.rod([(dx0 + dx1) / 2 + sx, ry + 2.72, z1 - 1.5], [(dx0 + dx1) / 2 + sx, ry + 3.3, z1 - 0.9], 0.04, 6,
          shade(P.galv, 0.94), false);
      }
      // Vent stacks over the ridge, with caps and stays. Two of them, because a
      // single stack reads as a chimney and a pair reads as process extract.
      for (let s = 0; s < 2; s += 1) {
        const sx = o.x + b.w * 0.16, sz = o.z + (s ? 1 : -1) * b.dz * 0.28;
        const base = o.deck + b.eaves + (b.ridge - b.eaves) * (1 - Math.abs(b.w * 0.16) / hw);
        m.column(sx, base, sz, 4.6, 0.42, 0.36, 14, shade(P.galv, 1.0), false);
        m.column(sx, base + 4.6, sz, 0.24, 0.5, 0.5, 14, shade(P.galv, 1.08), false);
        m.column(sx, base + 4.9, sz, 0.34, 0.54, 0.16, 14, shade(P.dark, 1.1), true);
        m.column(sx, base + 0.5, sz, 0.14, 0.52, 0.52, 14, shade(P.galv, 0.86), false);
        for (let g = 0; g < 3; g += 1) {
          const a = (g * 120 + 30) * DEG;
          m.rod([sx, base + 3.4, sz], [sx + Math.cos(a) * 2.2, base + 0.1, sz + Math.sin(a) * 2.2], 0.035, 5,
            shade(P.dark, 1.05), false);
        }
      }
    }
  }

  /// Hardstanding markings and a walkway. A yard with no paint on it is a grey
  /// rectangle, and paint is the one thing a renderer with no textures has to
  /// build out of geometry: the walkway route from the gate to the door in its
  /// own surface tone with edge lines, a hatched keep-clear box, two direction
  /// arrows as real polygons, and the bay numbers on posts.
  ///
  /// All of it flat, deliberately: none of it opens a smoothing group, so the
  /// check that plate and paint keep their exact face normal still holds here.
  function yardMarkings(m, W, D, d0) {
    if (!d0.fine) return;
    const y = GRADE + 0.11, wz = -D * 0.22;
    m.save().move(0, y, 0);
    m.plate([[-W * 0.44, wz - 0.7], [W * 0.16, wz - 0.7], [W * 0.16, wz + 0.7], [-W * 0.44, wz + 0.7]], 0.012,
      shade(P.asphalt, 1.22));
    m.plate([[W * 0.12, wz - 0.7], [W * 0.18, wz - 0.7], [W * 0.18, -D * 0.02], [W * 0.12, -D * 0.02]], 0.014,
      shade(P.asphalt, 1.22));
    m.restore();
    for (const s of [-1, 1]) {
      m.bar(-W * 0.44, W * 0.17, y + 0.012, y + 0.03, wz + s * 0.7 - 0.06, wz + s * 0.7 + 0.06, P.lane);
    }
    // Hatched keep-clear, on the diagonal, in front of the gate.
    for (let i = 0; i < 7; i += 1) {
      const hx = -W * 0.14 + i * 1.15;
      m.save().move(0, y, 0);
      m.plate([[hx, -D * 0.38], [hx + 0.22, -D * 0.38], [hx + 1.02, -D * 0.30], [hx + 0.8, -D * 0.30]], 0.016, P.lane);
      m.restore();
    }
    // Two direction arrows, as a shaft and a head rather than as one outline:
    // an arrow is concave, `plate` fans from its first vertex, and a fan across
    // a concave outline lays triangles outside the shape and degenerate ones
    // along the notch. Two convex pieces are the honest way to draw it.
    for (let a = 0; a < 2; a += 1) {
      const ax = -W * 0.30 + a * W * 0.30, az = -D * 0.36;
      m.save().move(0, y, 0);
      m.plate([[ax - 0.26, az], [ax + 0.26, az], [ax + 0.26, az + 1.55], [ax - 0.26, az + 1.55]], 0.016, P.lane);
      m.plate([[ax - 0.64, az + 1.4], [ax + 0.64, az + 1.4], [ax, az + 2.42]], 0.016, P.lane);
      m.restore();
    }
    // A speed roundel and the bay numbers. Both are things a yard has and both
    // are cheap; the posts also give the apron something vertical to read the
    // marked bays against.
    m.save().move(0, y, 0);
    m.plate([[W * 0.10, -D * 0.14], [W * 0.10 + 1.4, -D * 0.14], [W * 0.10 + 1.4, -D * 0.14 + 1.4],
      [W * 0.10, -D * 0.14 + 1.4]], 0.016, P.lane);
    m.restore();
    m.bar(W * 0.10 + 0.28, W * 0.10 + 1.12, y + 0.016, y + 0.032, -D * 0.14 + 0.6, -D * 0.14 + 0.8,
      shade(P.asphalt, 1.1));
    for (let i = 0; i < 4; i += 1) {
      const px = -W * 0.12 + i * 2.6, pz = -D * 0.27;
      m.beam(px - 0.05, px + 0.05, GRADE + 0.1, GRADE + 1.15, pz - 0.05, pz + 0.05, shade(P.galv, 0.94), 0.012);
      m.webPlate([[px - 0.24, GRADE + 0.86], [px + 0.24, GRADE + 0.86], [px + 0.24, GRADE + 1.24], [px - 0.24, GRADE + 1.24]],
        pz - 0.04, pz + 0.02, shade(P.hut, 1.08));
    }
  }

  /// The bunded store. Every facility on this list keeps fuel, oil or gas
  /// outside in a bund, and a bund is three unmistakable things: a kerbed slab
  /// that will hold a spill, a horizontal tank on saddles with a fill point,
  /// and a caged rack of bottles beside it. Small, permanent, and it is the
  /// piece that stops a finished yard being an empty car park.
  function bundedStore(m, x, z, d0, k) {
    if (!d0.fine) return;
    const w = 8.4, d = 5.2;
    m.bar(x - w / 2, x + w / 2, GRADE + 0.04, GRADE + 0.16, z - d / 2, z + d / 2, shade(P.concrete, 0.96));
    for (const s of [-1, 1]) {
      m.beam(x - w / 2 - 0.16, x + w / 2 + 0.16, GRADE + 0.04, GRADE + 0.7, z + s * (d / 2) - 0.16, z + s * (d / 2) + 0.16,
        shade(P.concrete, 1.02), 0.03);
      m.beam(x + s * (w / 2) - 0.16, x + s * (w / 2) + 0.16, GRADE + 0.04, GRADE + 0.7, z - d / 2 - 0.16, z + d / 2 + 0.16,
        shade(P.concrete, 0.98), 0.03);
    }
    // The tank: rolled shell on two saddles, a dished end, a fill cabinet and a
    // vent standing off the top of it.
    const tz = z - 1.0;
    m.rod([x - 2.6, GRADE + 1.35, tz], [x + 2.6, GRADE + 1.35, tz], 0.85, 14, shade(k.accent, 1.04), true);
    for (const sx of [-1.6, 1.6]) {
      m.beam(x + sx - 0.36, x + sx + 0.36, GRADE + 0.12, GRADE + 0.72, tz - 0.78, tz + 0.78, shade(P.concrete, 0.94), 0.03);
      m.webPlate([[x + sx - 0.4, GRADE + 0.72], [x + sx + 0.4, GRADE + 0.72], [x + sx + 0.4, GRADE + 0.86],
        [x + sx - 0.4, GRADE + 0.86]], tz - 0.8, tz + 0.8, shade(P.steel, 0.94));
    }
    m.column(x + 0.4, GRADE + 2.2, tz, 0.9, 0.09, 0.09, 8, shade(P.galv, 1.04), true);
    m.bar(x + 2.7, x + 3.5, GRADE + 0.72, GRADE + 1.9, tz - 0.42, tz + 0.42, shade(P.clad, 1.0));
    m.beam(x + 2.64, x + 3.56, GRADE + 1.9, GRADE + 2.02, tz - 0.48, tz + 0.48, shade(P.roof, 1.04), 0.018);
    m.rod([x - 2.6, GRADE + 1.35, tz], [x - 3.3, GRADE + 1.35, tz], 0.16, 8, shade(P.galv, 0.96), true);
    // The bottle cage: a framed rack with mesh on three sides and bottles in it.
    const cx = x + 1.0, cz = z + 1.6;
    for (const px of [cx - 1.8, cx + 1.8]) {
      for (const pz of [cz - 0.8, cz + 0.8]) {
        m.beam(px - 0.06, px + 0.06, GRADE + 0.12, GRADE + 2.2, pz - 0.06, pz + 0.06, shade(P.safety, 0.9), 0.014);
      }
    }
    m.beam(cx - 1.86, cx + 1.86, GRADE + 2.2, GRADE + 2.32, cz - 0.86, cz + 0.86, shade(P.safety, 1.06), 0.018);
    for (let i = 0; i < 9; i += 1) {
      const mx = cx - 1.7 + (i * 3.4) / 8;
      m.bar(mx - 0.025, mx + 0.025, GRADE + 0.2, GRADE + 2.18, cz - 0.83, cz - 0.79, shade(P.galv, 0.98));
      m.bar(mx - 0.025, mx + 0.025, GRADE + 0.2, GRADE + 2.18, cz + 0.79, cz + 0.83, shade(P.galv, 0.9));
    }
    for (let i = 0; i < 5; i += 1) {
      const bx = cx - 1.4 + i * 0.7;
      m.column(bx, GRADE + 0.12, cz, 1.5, 0.22, 0.22, 10, shade(i % 2 ? P.stopped : P.hoard, 0.96), true);
      m.column(bx, GRADE + 1.62, cz, 0.2, 0.22, 0.12, 10, shade(P.galv, 1.06), true);
      m.column(bx, GRADE + 1.82, cz, 0.16, 0.07, 0.07, 8, shade(P.galv, 0.9), true);
    }
    for (const bx of [x - w / 2 - 0.9, x + w / 2 + 0.9]) bollard(m, bx, z - d / 2 - 0.9, 1.0, 0.16, 8, P.safety);
  }

  /// A packaged substation and the cable route out of it. Every one of these
  /// facilities is fed from somewhere and the feed is a physical object: a
  /// kiosk with louvred ventilation and a locked door, a metering pillar beside
  /// it, and a cable trench under precast covers running to the building with
  /// marker posts along it. It goes in with the permanent supply at the frame
  /// stage and it is still there at hand-over — unlike the site kit, which is
  /// the distinction the stage gate exists to make.
  function substationKiosk(m, x, z, toX, d0) {
    if (!d0.fine) return;
    const w = 4.4, d = 2.8, h = 2.9;
    m.beam(x - w / 2 - 0.4, x + w / 2 + 0.4, GRADE, GRADE + 0.22, z - d / 2 - 0.4, z + d / 2 + 0.4,
      shade(P.concrete, 0.96), 0.025);
    m.bar(x - w / 2, x + w / 2, GRADE + 0.22, GRADE + h, z - d / 2, z + d / 2, shade(P.clad, 0.98));
    m.beam(x - w / 2 - 0.16, x + w / 2 + 0.16, GRADE + h, GRADE + h + 0.2, z - d / 2 - 0.16, z + d / 2 + 0.16,
      shade(P.roof, 1.04), 0.025);
    // Louvres on the long face and a door on the end. A kiosk that cannot
    // breathe is a box, and the louvre bank is the same object the halls use.
    louvreBank(m, x - w / 2 + 0.5, x - 0.3, GRADE + 1.1, GRADE + 2.4, z + d / 2, 0.13, 4, shade(P.galv, 0.96));
    louvreBank(m, x + 0.3, x + w / 2 - 0.5, GRADE + 1.1, GRADE + 2.4, z + d / 2, 0.13, 4, shade(P.galv, 0.96));
    m.bar(x + w / 2, x + w / 2 + 0.05, GRADE + 0.3, GRADE + 2.4, z - 0.5, z + 0.5, P.door);
    m.webPlate([[x - 1.1, GRADE + 2.6], [x + 1.1, GRADE + 2.6], [x + 1.1, GRADE + 2.9], [x - 1.1, GRADE + 2.9]],
      z - d / 2 - 0.06, z - d / 2, shade(P.safety, 1.05));
    // The metering pillar beside it, and the duct sweeping out of its base.
    m.bar(x - w / 2 - 1.5, x - w / 2 - 0.5, GRADE + 0.22, GRADE + 1.9, z - 0.4, z + 0.4, shade(P.clad, 0.9));
    m.beam(x - w / 2 - 1.56, x - w / 2 - 0.44, GRADE + 1.9, GRADE + 2.02, z - 0.46, z + 0.46, shade(P.roof, 1.02), 0.018);
    m.rod([x - w / 2 - 1.0, GRADE + 0.9, z - 0.42], [x - w / 2 - 1.0, GRADE + 0.9, z - 1.0], 0.09, 8, shade(P.galv, 1.0), true);
    // The cable route: covers on the trench, marker posts along it, and a duct
    // stub standing where it turns in under the building.
    const covers = Math.max(3, Math.round(Math.abs(toX - x) / 1.7));
    const cz = z - d / 2 - 1.9;
    for (let i = 0; i < covers; i += 1) {
      const cx = x + ((i + 0.5) * (toX - x)) / covers;
      m.beam(cx - 0.72, cx + 0.72, GRADE + 0.04, GRADE + 0.17, cz - 0.6, cz + 0.6,
        shade(P.concrete, 0.9 + 0.07 * (i % 2)), 0.02);
    }
    for (let i = 0; i <= 2; i += 1) {
      const px = x + (i * (toX - x)) / 2;
      m.beam(px - 0.07, px + 0.07, GRADE + 0.1, GRADE + 0.95, cz + 0.72, cz + 0.86, shade(P.safety, 0.94), 0.014);
      m.webPlate([[px - 0.2, GRADE + 0.72], [px + 0.2, GRADE + 0.72], [px + 0.2, GRADE + 0.98], [px - 0.2, GRADE + 0.98]],
        cz + 0.86, cz + 0.9, shade(P.hut, 1.06));
    }
    m.rod([toX, GRADE + 0.17, cz], [toX, GRADE + 0.9, cz], 0.13, 8, shade(P.galv, 0.94), true);
    for (const bx of [x - w / 2 - 2.4, x + w / 2 + 1.2]) bollard(m, bx, cz - 0.9, 1.0, 0.16, 8, P.safety);
  }

  /// A pipe bridge across the yard. Services do not stop at a building's wall:
  /// they cross the compound six metres up on trestles, and that horizontal
  /// line with the trestles hanging off it and a walkway beside it is one of
  /// the very few things on an industrial site that still reads at 1124 x 102,
  /// where the strip is a hundred pixels tall and nothing survives but the
  /// outline.
  ///
  /// `full` is the hand-over state and it is an honest split rather than a
  /// budget trick: the steel and the two main lines go up with the services,
  /// and the remaining lines, the lagging and the lighting run under the
  /// walkway are commissioning work.
  function pipeBridge(m, x0, x1, z, y, d0, k, full) {
    if (!d0.fine) return;
    const bays = Math.max(4, Math.round((x1 - x0) / 9));
    for (let i = 0; i <= bays; i += 1) {
      const px = x0 + (i * (x1 - x0)) / bays;
      for (const s of [-1, 1]) {
        m.beam(px - 0.15, px + 0.15, GRADE, y, z + s * 0.85 - 0.15, z + s * 0.85 + 0.15, shade(P.steel, 0.94), 0.022);
        m.beam(px - 0.34, px + 0.34, GRADE - 0.02, GRADE + 0.32, z + s * 0.85 - 0.34, z + s * 0.85 + 0.34,
          shade(P.concrete, 0.94), 0.03);
        m.rod([px, y - 0.35, z + s * 0.85], [px, y - 1.9, z], 0.055, 6, shade(P.steel, 0.9), false);
      }
      m.beam(px - 0.19, px + 0.19, y, y + 0.28, z - 1.15, z + 1.15, shade(P.steel, 1.02), 0.022);
    }
    // The lines. Two while the bridge is being built, four once it is in
    // service, and a shoe under every line at every trestle.
    const lines = full
      ? [[-0.62, 0.24, shade(P.galv, 1.0)], [-0.12, 0.19, shade(k.accent, 1.08)],
        [0.34, 0.15, shade(P.safety, 0.92)], [0.72, 0.12, shade(P.hut, 0.92)]]
      : [[-0.62, 0.24, shade(P.galv, 1.0)], [-0.12, 0.19, shade(k.accent, 1.08)]];
    for (const line of lines) {
      const dz = line[0], r = line[1];
      m.rod([x0 - 0.7, y + 0.28 + r, z + dz], [x1 + 0.7, y + 0.28 + r, z + dz], r, 10, line[2], false);
      for (let i = 0; i <= bays; i += 1) {
        const px = x0 + (i * (x1 - x0)) / bays;
        m.beam(px - 0.11, px + 0.11, y + 0.28, y + 0.28 + r * 0.7, z + dz - 0.15, z + dz + 0.15, shade(P.steel, 0.9), 0.012);
      }
    }
    // The maintenance walkway. Somebody has to be able to reach a flange, and
    // the grating rhythm is what gives a bridge a length to read against.
    m.bar(x0, x1, y + 0.2, y + 0.28, z + 0.98, z + 1.36, shade(P.steel, 0.86));
    const slats = Math.max(8, Math.round((x1 - x0) / 1.15));
    for (let i = 0; i < slats; i += 1) {
      const sx = x0 + (i * (x1 - x0)) / slats;
      m.bar(sx + 0.08, sx + (x1 - x0) / slats - 0.08, y + 0.28, y + 0.34, z + 0.98, z + 1.36,
        shade(P.steel, 0.94 + 0.09 * (i % 2)));
    }
    const posts = Math.max(6, bays * 2);
    for (let i = 0; i <= posts; i += 1) {
      const px = x0 + (i * (x1 - x0)) / posts;
      m.rod([px, y + 0.34, z + 1.36], [px, y + 1.42, z + 1.36], 0.035, 6, P.galv, false);
    }
    m.rod([x0, y + 1.42, z + 1.36], [x1, y + 1.42, z + 1.36], 0.035, 6, shade(P.galv, 1.06), false);
    m.rod([x0, y + 0.9, z + 1.36], [x1, y + 0.9, z + 1.36], 0.032, 6, shade(P.galv, 0.92), false);
    m.bar(x0, x1, y + 0.34, y + 0.48, z + 1.32, z + 1.38, shade(P.safety, 0.92));
    // Both ends drop to grade, so the bridge comes from somewhere and goes
    // somewhere instead of floating over the yard on nothing.
    for (const s of [-1, 1]) {
      const ex = s < 0 ? x0 : x1;
      m.rod([ex + s * 0.7, y + 0.52, z - 0.62], [ex + s * 0.7, GRADE + 1.3, z - 0.62], 0.24, 10, shade(P.galv, 0.98), false);
      m.rod([ex + s * 0.7, GRADE + 1.3, z - 0.62], [ex + s * 2.1, GRADE + 1.3, z - 0.62], 0.24, 10, shade(P.galv, 0.9), false);
      m.beam(ex + s * 1.8 - 0.5, ex + s * 1.8 + 0.5, GRADE, GRADE + 0.3, z - 1.12, z - 0.12, shade(P.concrete, 0.94), 0.025);
    }
    if (!full) return;
    // Lagging bands on the hot line and a lighting run under the walkway. Both
    // are commissioning work and neither is there while the bridge is going up.
    for (let i = 0; i < bays * 2; i += 1) {
      const bx = x0 + ((i + 0.5) * (x1 - x0)) / (bays * 2);
      m.rod([bx - 0.14, y + 0.43, z + 0.34], [bx + 0.14, y + 0.43, z + 0.34], 0.2, 10, shade(P.hut, 1.04), false);
    }
    for (let i = 0; i < bays; i += 1) {
      const lx = x0 + ((i + 0.5) * (x1 - x0)) / bays;
      m.bar(lx - 0.34, lx + 0.34, y + 0.06, y + 0.2, z + 1.02, z + 1.3, shade(P.dark, 1.3));
      m.bar(lx - 0.28, lx + 0.28, y + 0.02, y + 0.06, z + 1.04, z + 1.28, shade(P.lane, 1.2));
    }
  }

  /// Where the drainage run goes, and it is ONE function because the trench and
  /// the plant working beside it have to agree: clear of the dig on the approach
  /// side of the compound, and never so far forward that it lands in the gate,
  /// the wheel wash or the haul route. Every kind seats its block differently,
  /// so this is read off the block rather than off the pad.
  function serviceRunZ(o, b, D) {
    return Math.max(o.z - (b.dz + 3.2) / 2 - 7.0, -D / 2 + 10.5);
  }

  /// The services that hang off the building itself, gathered so `buildNear`
  /// calls them once and the stage gate lives in one place. Nothing here exists
  /// before the steel is up, which is the honest reading: you cannot bracket
  /// pipework to a frame that is not standing.
  function buildingServices(m, k, o, W, D, stage, d0) {
    if (!d0.fine) return;
    const b = o.block;
    if (stage >= 3) {
      m.part("services / external stair and roof access", () => {
        accessStair(m, o.x + b.w / 2 + 1.0, o.z - b.dz * 0.30, o.deck, o.deck + b.eaves, d0,
          stage >= 4 ? shade(P.galv, 0.96) : P.primer);
      });
      m.part("services / packaged substation and cable route", () => {
        substationKiosk(m, o.x + b.w / 2 + 3.4, o.z + b.dz * 0.30, o.x + b.w * 0.20, d0);
      });
    }
    if (stage >= 4) {
      m.part("services / external pipework and cable ladder", () => wallServices(m, o, d0, k));
      m.part("services / extract ducting and vent stacks", () => roofServices(m, o, stage, d0));
      m.part("services / yard pipe bridge", () => {
        pipeBridge(m, -W * 0.34, W * 0.34, D * 0.36, GRADE + 6.2, d0, k, stage >= 5);
      });
    }
  }

  // ------------------------------------------------------------- arms plant
  // The one kind worked through in full (roadmap section E: assembly hall, test
  // and service yard, secure loading area). Everything below is original
  // generic industrial design — no real plant, no measured dimensions, and no
  // claim about what it can produce.

  /// The assembly hall, at whatever completeness the stage asks for. One
  /// function, four appearances: bare frame, frame with rails, part-clad, clad.
  function hall(m, o, d0) {
    const { w, dz, eaves, ridge, bays, ends } = o.block;
    const clad = o.clad;
    // A TAGGED BLOCK IS A SECONDARY OBJECT and is drawn a step plainer: five
    // frames rather than seven, six roof sheets rather than ten. It is the same
    // call this function already makes for a tagged block's rib pitch, its
    // hidden frames and its rainwater goods, and it is what lets a composition
    // stand three halls on one pad without spending three lead blocks' worth of
    // the close budget on them.
    const frames = d0.fine ? (o.tag ? 5 : 7) : 3;
    const sheets = d0.fine ? (o.tag ? 6 : 10) : 3;
    // Part names are the picker's identity, so a compound with four wings in it
    // cannot have four parts called "wall cladding". The tag is the block's own
    // name and it is what a selection reads back.
    const tag = o.tag ? ` (${o.tag})` : "";
    // Red-oxide primer is what erected steel looks like before it is clad, and
    // it is the fastest read there is for "this is not finished". Once the
    // envelope is on, the columns that stay proud of it are painted trim, so
    // the same geometry carries a different colour rather than a different
    // model.
    const steel = clad > 0.9 ? shade(o.col, 0.66) : P.primer;
    // A delivered wing passes `frame: false`. Its steel is inside a finished
    // envelope and cannot be seen from any angle this renderer offers, and at
    // five levels those hidden frames were most of a level-5 compound's budget.
    if (o.frame !== false) {
    m.part(`structure / portal frames and haunches${tag}`, () => {
      for (let i = 0; i < frames; i += 1) {
        const z = -dz / 2 + (i * dz) / (frames - 1);
        m.save().move(o.x, 0, o.z);
        portal(m, z, w, o.deck, eaves, ridge, steel);
        m.restore();
      }
    });
    m.part(`structure / purlins, rails and bracing${tag}`, () => {
      m.save().move(o.x, 0, o.z);
      roof(m, w, dz, eaves, ridge, sheets, P.roof, d0, { clad: 0, purlins: true });
      for (const s of [-1, 1]) {
        m.save().move(s * (w / 2), 0, 0).rotY(90 * s);
        sideRails(m, dz, eaves, d0.fine ? 4 : 2, 0, steel);
        m.restore();
      }
      for (const s of [-1, 1]) {
        m.save().move(0, 0, s * (dz / 2)).rotY(s > 0 ? 0 : 180);
        sideRails(m, w, eaves, d0.fine ? 3 : 1, 0, steel);
        m.restore();
      }
      if (d0.fine) {
        for (const s of [-1, 1]) {
          m.save().move(s * (w / 2 - 0.3), o.deck + eaves / 2, -dz / 2 + dz / (frames - 1) / 2).rotX(38);
          m.bar(-0.09, 0.09, -eaves * 0.62, eaves * 0.62, -0.09, 0.09, steel);
          m.restore();
        }
      }
      m.restore();
    });
    }
    if (clad <= 0) return;
    m.part(`envelope / wall cladding${tag}`, () => {
      m.save().move(o.x, o.deck, o.z);
      // A delivered wing is a secondary object and is drawn a step plainer —
      // the same call already made for its hidden frames and its rainwater
      // goods. Wider rib pitch, not fewer flashings: the flashings are the
      // silhouette and the ribs are the surface.
      const pitch = o.tag ? 1.25 : 0.75;
      for (const s of [-1, 1]) {
        m.save().move(0, 0, s * (dz / 2)).rotY(s > 0 ? 0 : 180);
        claddedWall(m, w, eaves, bays, o.col, d0, { clad, phase: s, pitch });
        m.restore();
      }
      for (const s of [-1, 1]) {
        m.save().move(s * (w / 2), 0, 0).rotY(90 * s);
        claddedWall(m, dz, eaves, ends, o.col, d0, { clad, phase: s * 2, pitch });
        m.restore();
      }
      m.restore();
    });
    m.part(`envelope / roof sheeting and ridge${tag}`, () => {
      m.save().move(o.x, o.deck, o.z);
      roof(m, w, dz, eaves, ridge, sheets, P.roof, d0, { clad });
      // Gable tympanum, only where the wall below it is sheeted.
      if (clad > 0.9) {
        for (const s of [-1, 1]) {
          m.save().move(0, 0, s * (dz / 2)).rotY(s > 0 ? 0 : 180);
          m.tri([-w / 2, eaves, 0], [w / 2, eaves, 0], [0, ridge, 0], shade(o.col, 1.03));
          m.restore();
        }
      }
      m.restore();
    });
    if (clad > 0.9 && d0.fine) {
      m.part(`envelope / ridge ventilator${tag}`, () => {
        m.save().move(o.x, o.deck, o.z);
        m.bar(-1.2, 1.2, ridge + 0.25, ridge + 1.5, -dz * 0.3, dz * 0.3, shade(P.roof, 1.06));
        for (let i = 0; i < 5; i += 1) {
          const zz = -dz * 0.28 + (i * dz * 0.56) / 4;
          m.bar(-1.3, 1.3, ridge + 0.55, ridge + 0.72, zz - 0.18, zz + 0.18, P.dark);
        }
        m.bar(-1.45, 1.45, ridge + 1.5, ridge + 1.62, -dz * 0.32, dz * 0.32, shade(P.roof, 0.86));
        m.restore();
      });
      m.part(`envelope / rainwater goods${tag}`, () => {
        m.save().move(o.x, o.deck, o.z);
        for (const s of [-1, 1]) {
          m.bar(s * (w / 2) - 0.2, s * (w / 2) + 0.24, eaves, eaves + 0.32, -dz / 2, dz / 2, shade(P.galv, 0.94));
          for (let i = 0; i < 3; i += 1) {
            const zz = -dz / 2 + 1.5 + (i * (dz - 3)) / 2;
            m.column(s * (w / 2 + 0.2), 0, zz, eaves, 0.13, 0.13, d0.seg, P.galv, false);
            // The shoe at the bottom and the hopper at the top. Only on the
            // lead block: a delivered wing is a secondary object and is drawn a
            // step plainer, the same call already made for its hidden frames.
            if (o.tag) continue;
            m.rod([s * (w / 2 + 0.2), 0.42, zz], [s * (w / 2 + 0.55), 0.18, zz], 0.13, 8, shade(P.galv, 0.9), true);
            m.column(s * (w / 2 + 0.2), eaves - 0.34, zz, 0.34, 0.24, 0.14, 8, shade(P.galv, 1.02), false);
          }
        }
        m.restore();
      });
    }
  }

  /// The doors are what says "assembly hall" rather than "shed": full-height
  /// sliding leaves on the gable a finished vehicle would come out of.
  function hallDoors(m, o, d0) {
    const { w, dz, eaves } = o.block;
    m.part("envelope / assembly doors and personnel access", () => {
      m.save().move(o.x, o.deck, o.z + dz / 2 + 0.02);
      for (let i = 0; i < 3; i += 1) {
        const cx = (i - 1) * (w / 3.4), dw = w / 4.6, dh = eaves * 0.66;
        m.bar(cx - dw / 2, cx + dw / 2, 0.05, dh, 0, 0.14, P.door);
        if (d0.fine) {
          // Two leaves that meet on the centre line, ribbed, on a track with a
          // bottom guide — a sliding door is a pair of panels that move, and the
          // meeting stile and the track are what say so. Vision panels at head
          // height, because every large industrial door has them.
          for (const ls of [-1, 1]) {
            m.beam(cx + (ls < 0 ? -dw / 2 : 0.03), cx + (ls < 0 ? -0.03 : dw / 2), 0.05, dh, 0.14, 0.24,
              shade(P.door, ls < 0 ? 1.04 : 0.96), 0.02);
            for (let r = 0; r < 4; r += 1) {
              const rx = cx + ls * (0.2 + r * (dw / 2 - 0.35) / 3);
              m.bar(rx - 0.05, rx + 0.05, 0.1, dh - 0.05, 0.24, 0.3, shade(P.door, 0.86));
            }
            m.bar(cx + ls * dw * 0.24 - 0.42, cx + ls * dw * 0.24 + 0.42, dh - 1.35, dh - 0.75, 0.22, 0.26, P.glass);
          }
          m.beam(cx - dw / 2 - 0.3, cx + dw / 2 + 0.3, dh, dh + 0.3, 0, 0.36, P.safety, 0.02);   // track and hood
          m.bar(cx - dw / 2 - 0.3, cx + dw / 2 + 0.3, 0.02, 0.09, 0.16, 0.3, shade(P.steel, 0.94));
          for (const gs of [-1, 1]) {
            m.beam(cx + gs * (dw / 2 + 0.18) - 0.09, cx + gs * (dw / 2 + 0.18) + 0.09, 0.05, dh + 0.3, 0.12, 0.32, P.galv, 0.016);
          }
        }
      }
      if (d0.fine) {
        // Personnel door beside the last leaf, with a frame, a handle and a
        // canopy: the one human-sized thing on a 34 metre elevation, and the
        // only reference a viewer has for how big the rest of it is.
        m.bar(w / 2 - 2.6, w / 2 - 1.4, 0.05, 2.3, 0, 0.12, shade(P.door, 0.85));
        m.beam(w / 2 - 2.72, w / 2 - 1.28, 0.02, 2.42, 0.1, 0.2, shade(P.galv, 1.02), 0.015);
        m.rod([w / 2 - 1.62, 1.05, 0.14], [w / 2 - 1.62, 1.05, 0.3], 0.04, 6, P.dark, true);
        m.bar(w / 2 - 3.0, w / 2 - 1.0, 2.42, 2.58, 0, 1.4, shade(P.clad, 1.06));
        for (const bs of [-0.9, 0.9]) m.rod([w / 2 - 2.0 + bs, 2.42, 0.1], [w / 2 - 2.0 + bs, 1.9, 0.05], 0.04, 6, P.galv, false);
        // Guide posts either side of the outer opening — what stops a vehicle
        // taking the door off its track. Placed at local -0.18 so they stand on
        // the yard, not on the deck this block is drawn relative to.
        for (const gs of [-1, 1]) {
          const gx = gs * (w / 3.4 + w / 9.2 + 0.8);
          m.column(gx, -0.18, 1.1, 0.72, 0.2, 0.2, 8, P.safety, true);
          m.column(gx, 0.54, 1.1, 0.19, 0.21, 0.21, 8, shade(P.hut, 1.06), false);
          m.column(gx, 0.73, 1.1, 0.2, 0.2, 0.13, 8, P.safety, true);
        }
      }
      m.restore();
    });
  }

  /// Secure loading area: a raised dock with levellers, a canopy over it and a
  /// gatehouse on the compound line. "Secure" here is a fence, a gate and a
  /// controlled dock — it is not a claim about anything the sim models.
  function loadingArea(m, o, d0) {
    const x = o.x + 8, z = o.z - o.block.dz / 2 - 8.0;
    m.part("yard / loading dock and levellers", () => {
      m.bar(x - 11, x + 11, GRADE, GRADE + 1.25, z - 3.2, z + 3.2, P.concrete);
      if (d0.fine) m.beam(x - 11.2, x + 11.2, GRADE + 1.2, GRADE + 1.34, z - 3.4, z + 3.4, P.kerb, 0.025);
      else m.bar(x - 11.2, x + 11.2, GRADE + 1.2, GRADE + 1.32, z - 3.4, z + 3.4, P.kerb);
      const docks = d0.fine ? 4 : 2;
      for (let i = 0; i < docks; i += 1) {
        const dx = x - 8.2 + (i * 16.4) / (docks - 1);
        m.bar(dx - 1.6, dx + 1.6, GRADE + 1.32, GRADE + 1.44, z - 3.4, z - 1.4, shade(P.steel, 1.05));
        if (!d0.fine) continue;
        // A dock is a hole in a wall with a leveller in it, rubber buffers
        // either side, a shelter round it and a light on an arm. Each of those
        // is a thing a driver needs, and together they are the difference
        // between a loading dock and a slot.
        m.beam(dx - 1.55, dx + 1.55, GRADE + 1.36, GRADE + 1.46, z - 3.5, z - 3.16, shade(P.steel, 1.12), 0.015);
        for (const s of [-1, 1]) {
          m.beam(dx + s * 1.75 - 0.18, dx + s * 1.75 + 0.18, GRADE + 0.55, GRADE + 1.28, z - 3.56, z - 3.3, shade(P.dark, 1.15), 0.02);
          m.bar(dx + s * 1.75 - 0.26, dx + s * 1.75 + 0.26, GRADE + 1.32, GRADE + 3.5, z - 3.62, z - 3.28, shade(P.dark, 1.0));
        }
        m.bar(dx - 2.05, dx + 2.05, GRADE + 3.5, GRADE + 3.86, z - 3.66, z - 3.24, shade(P.dark, 1.0));
        m.bar(dx - 1.5, dx + 1.5, GRADE + 1.46, GRADE + 3.44, z - 3.36, z - 3.3, P.door);
        for (let s = 0; s < 5; s += 1) {
          m.bar(dx - 1.44, dx + 1.44, GRADE + 1.6 + s * 0.38, GRADE + 1.88 + s * 0.38, z - 3.42, z - 3.36, shade(P.door, 1.08));
        }
        m.rod([dx + 1.9, GRADE + 3.3, z - 3.3], [dx + 2.7, GRADE + 3.5, z - 3.9], 0.05, 6, P.galv, false);
        m.column(dx + 2.7, GRADE + 3.34, z - 3.9, 0.3, 0.24, 0.2, 8, shade(P.lane, 1.1), true);
      }
      if (d0.fine) {
        // Steps off the end of the dock, and the guard rail along the open edge.
        for (let s = 0; s < 4; s += 1) {
          m.beam(x + 11.2, x + 12.4, GRADE + (s * 1.25) / 4, GRADE + ((s + 1) * 1.25) / 4,
            z + 1.0 + s * 0.3, z + 3.2, shade(P.concrete, 0.94 + 0.03 * s), 0.02);
        }
        for (const rz of [z + 3.3]) {
          for (let p = 0; p < 5; p += 1) {
            const px = x - 10 + p * 5;
            m.rod([px, GRADE + 1.34, rz], [px, GRADE + 2.44, rz], 0.045, 6, P.safety, false);
          }
          m.rod([x - 10, GRADE + 2.44, rz], [x + 10, GRADE + 2.44, rz], 0.045, 6, P.safety, false);
          m.rod([x - 10, GRADE + 1.9, rz], [x + 10, GRADE + 1.9, rz], 0.045, 6, shade(P.safety, 0.9), false);
        }
      }
    });
    m.part("yard / dock canopy", () => {
      const posts = d0.fine ? 5 : 2;
      for (let i = 0; i < posts; i += 1) {
        const px = x - 10 + (i * 20) / (posts - 1);
        if (d0.fine) {
          m.beam(px - 0.16, px + 0.16, GRADE, GRADE + 5.2, z - 3.6, z - 3.28, P.steel, 0.018);
          m.bar(px - 0.3, px + 0.3, GRADE, GRADE + 0.1, z - 3.74, z - 3.14, shade(P.steel, 0.9));
          m.webPlate([[px - 0.16, GRADE + 4.4], [px - 0.16, GRADE + 5.2], [px - 1.5, GRADE + 5.2]],
            z - 3.52, z - 3.36, shade(P.steel, 0.96));
        } else {
          m.bar(px - 0.16, px + 0.16, GRADE, GRADE + 5.2, z - 3.6, z - 3.28, P.steel);
        }
      }
      m.save().move(0, GRADE + 5.2, 0);
      m.plate([[x - 11.5, z - 5.6], [x + 11.5, z - 5.6], [x + 11.5, z + 0.6], [x - 11.5, z + 0.6]], 0.22, shade(P.roof, 1.04));
      m.restore();
      if (d0.fine) {
        // Gutter along the outer edge and two downpipes off it. Water has to go
        // somewhere and a canopy with no rainwater goods reads as a card.
        m.beam(x - 11.6, x + 11.6, GRADE + 5.2, GRADE + 5.5, z - 5.72, z - 5.44, shade(P.galv, 0.96), 0.02);
        for (const px of [x - 10, x + 10]) m.rod([px, GRADE, z - 5.58], [px, GRADE + 5.2, z - 5.58], 0.11, 8, P.galv, false);
        for (const px of [x - 10, x + 10]) m.rod([px, GRADE + 0.15, z - 5.58], [px, GRADE + 0.15, z - 5.1], 0.11, 8, P.galv, false);
      }
    });
    m.part("yard / bollards and dock markings", () => {
      const n = d0.fine ? 8 : 3;
      for (let i = 0; i < n; i += 1) {
        const bx = x - 10.5 + (i * 21) / (n - 1);
        if (d0.fine) bollard(m, bx, z - 4.6, 1.1, 0.22, 8, P.safety);
        else m.column(bx, GRADE, z - 4.6, 1.05, 0.22, 0.22, d0.seg, P.safety, true);
      }
      if (d0.fine) {
        for (let i = 0; i < 4; i += 1) {
          const lx = x - 7.5 + i * 5;
          // Stops 7.8 m short of the dock rather than 11: the compound line at
          // level one sits at 23.2 m and the old run put painted lane markings
          // straight through the perimeter fence.
          m.bar(lx - 0.12, lx + 0.12, GRADE + 0.01, GRADE + 0.04, z - 7.8, z - 5, P.lane);
        }
        // The approach itself, worn where the vehicles turn.
        groundPatch(m, x, z - 4.4, 24, 6.8, 10, 5, GRADE + 0.008, P.asphalt, 0.055);
      }
    });
  }

  function gatehouse(m, x, z, d0) {
    m.part("yard / gatehouse and controlled entry", () => {
      m.bar(x - 2.4, x + 2.4, GRADE, GRADE + 3.1, z - 2.0, z + 2.0, shade(P.clad, 1.02));
      if (d0.fine) m.beam(x - 2.7, x + 2.7, GRADE + 3.1, GRADE + 3.38, z - 2.3, z + 2.3, P.roof, 0.025);
      else m.bar(x - 2.7, x + 2.7, GRADE + 3.1, GRADE + 3.35, z - 2.3, z + 2.3, P.roof);
      if (d0.fine) {
        // Glazing in a frame, a door, and a barrier with a counterweight. A
        // controlled entry is a person in a box who can stop a vehicle, and the
        // barrier is the part that does the stopping.
        for (const s of [-1, 1]) {
          m.bar(x - 1.7, x + 1.7, GRADE + 1.5, GRADE + 2.5, z + s * 2.0, z + s * 2.03, P.glass);
          m.beam(x - 1.82, x + 1.82, GRADE + 1.4, GRADE + 2.6, z + s * 2.0, z + s * 2.1, shade(P.galv, 1.0), 0.015);
          m.beam(x - 0.06, x + 0.06, GRADE + 1.5, GRADE + 2.5, z + s * 2.0, z + s * 2.1, shade(P.galv, 1.02), 0.012);
        }
        m.bar(x - 2.4, x - 1.4, GRADE + 0.05, GRADE + 2.1, z + 2.0, z + 2.04, P.door);
        m.beam(x - 2.52, x - 1.28, GRADE, GRADE + 2.22, z + 2.0, z + 2.09, shade(P.galv, 0.96), 0.015);
        m.column(x + 2.6, GRADE, z, 1.5, 0.18, 0.18, 10, P.dark, true);
        m.beam(x + 2.6, x + 9.2, GRADE + 1.36, GRADE + 1.52, z - 0.1, z + 0.1, P.safety, 0.02);
        for (let b = 0; b < 5; b += 1) {
          m.bar(x + 3.2 + b * 1.2, x + 3.8 + b * 1.2, GRADE + 1.35, GRADE + 1.53, z - 0.11, z + 0.11, shade(P.hut, 1.04));
        }
        m.beam(x + 1.9, x + 2.6, GRADE + 1.28, GRADE + 1.6, z - 0.16, z + 0.16, P.concreteWet, 0.02);
        for (const bs of [-1, 1]) bollard(m, x + 2.6, z + bs * 1.6, 0.95, 0.18, 8, P.safety);
        m.beam(x - 2.9, x + 2.9, GRADE + 3.38, GRADE + 3.72, z - 1.0, z - 0.86, shade(P.hoard, 1.1), 0.02);
      }
    });
  }

  /// Test and service yard: hardstand, ramps and lane markings. A place to run
  /// a finished machine, not a proving ground — and labelled as the former.
  function testYard(m, x, z, w, d, d0) {
    m.part("yard / test and service hardstand", () => {
      m.save().move(0, GRADE, 0);
      m.plate([[x - w / 2, z - d / 2], [x + w / 2, z - d / 2], [x + w / 2, z + d / 2], [x - w / 2, z + d / 2]], 0.12, P.asphalt);
      m.restore();
      if (d0.fine) {
        for (let i = 0; i < 5; i += 1) {
          const lz = z - d / 2 + 2 + (i * (d - 4)) / 4;
          m.bar(x - w / 2 + 1, x + w / 2 - 1, GRADE + 0.12, GRADE + 0.15, lz - 0.11, lz + 0.11, P.lane);
        }
        for (const s of [-1, 1]) {
          m.beam(x + s * (w / 2) - 0.3, x + s * (w / 2) + 0.3, GRADE + 0.12, GRADE + 0.42, z - d / 2, z + d / 2, P.kerb, 0.025);
        }
        // Ramp with kerbs, and an inspection pit with a grating over it rather
        // than a plate: the slats are what say there is a hole under it.
        m.beam(x - 4, x + 4, GRADE + 0.12, GRADE + 0.9, z - d / 2 - 3, z - d / 2 - 0.4, P.concrete, 0.03);
        for (const s of [-1, 1]) {
          m.beam(x + s * 4 - 0.22, x + s * 4 + 0.22, GRADE + 0.9, GRADE + 1.12, z - d / 2 - 3, z - d / 2 - 0.4, P.kerb, 0.02);
        }
        m.bar(x - 2.3, x + 2.3, GRADE + 0.05, GRADE + 0.13, z + 1.9, z + 8.1, shade(P.dark, 0.7));
        for (let g = 0; g < 14; g += 1) {
          const gz = 2.0 + g * 0.44;
          m.bar(x - 2.24, x + 2.24, GRADE + 0.13, GRADE + 0.2, z + gz, z + gz + 0.26, shade(P.steel, 0.92 + 0.06 * (g % 2)));
        }
        for (const s of [-1, 1]) {
          m.beam(x + s * 2.32, x + s * 2.44, GRADE + 0.12, GRADE + 0.22, z + 1.85, z + 8.15, shade(P.steel, 1.02), 0.015);
        }
        // Wash-down bay in the far corner, and the tyre wear across the middle
        // of the hardstand where everything turns.
        groundPatch(m, x, z, w - 2.4, d - 2.4, 10, 7, GRADE + 0.125, P.asphalt, 0.05);
        m.bar(x + w / 2 - 5.4, x + w / 2 - 0.6, GRADE + 0.12, GRADE + 0.2, z - d / 2 + 0.6, z - d / 2 + 4.4, shade(P.concrete, 0.92));
        m.rod([x + w / 2 - 3.0, GRADE + 0.2, z - d / 2 + 2.5], [x + w / 2 - 3.0, GRADE + 2.6, z - d / 2 + 2.5], 0.09, 8, P.galv, false);
        m.beam(x + w / 2 - 3.6, x + w / 2 - 2.4, GRADE + 2.6, GRADE + 2.74, z - d / 2 + 2.4, z - d / 2 + 3.4, shade(P.galv, 0.94), 0.02);
      }
    });
  }

  function utilities(m, x, z, d0, tall) {
    m.part("services / substation, tanks and stack", () => {
      m.bar(x - 3.0, x + 3.0, GRADE, GRADE + 3.4, z - 2.2, z + 2.2, shade(P.concrete, 0.96));
      if (d0.fine) m.beam(x - 3.2, x + 3.2, GRADE + 3.4, GRADE + 3.62, z - 2.4, z + 2.4, P.roof, 0.025);
      else m.bar(x - 3.2, x + 3.2, GRADE + 3.4, GRADE + 3.6, z - 2.4, z + 2.4, P.roof);
      for (const s of [-1, 1]) {
        m.bar(x + 4.2, x + 6.6, GRADE, GRADE + 2.4, z + s * 1.6 - 1.0, z + s * 1.6 + 1.0, P.galv);
        if (!d0.fine) continue;
        // A transformer is a tank with radiator banks bolted to it and bushings
        // on top. Both are on every one, both are unmistakable, and both are a
        // handful of triangles.
        m.column(x + 5.4, GRADE + 2.4, z + s * 1.6, 1.6, 0.3, 0.3, d0.seg, P.dark, true);
        for (let r = 0; r < 6; r += 1) {
          m.bar(x + 4.4 + r * 0.36, x + 4.6 + r * 0.36, GRADE + 0.5, GRADE + 2.1,
            z + s * 1.6 - 1.42, z + s * 1.6 - 1.0, shade(P.galv, 0.86 + 0.06 * (r % 2)));
        }
        for (const bx of [x + 4.7, x + 5.4, x + 6.1]) {
          m.column(bx, GRADE + 2.42, z + s * 1.6 + 0.7, 0.85, 0.16, 0.13, 8, shade(P.hardcore, 1.02), true);
        }
        m.bar(x + 4.0, x + 6.8, GRADE, GRADE + 0.16, z + s * 1.6 - 1.2, z + s * 1.6 + 1.2, shade(P.concrete, 0.9));
      }
      // Storage tanks: smooth shells with a dished top, a stair round one of
      // them and a rail at the top. A tank without an access stair is a drum.
      for (const tx of [x - 6.5, x - 10.5]) {
        m.column(tx, GRADE + 0.35, z, 6.5, 1.9, 1.9, d0.fine ? 14 : d0.seg, shade(P.galv, tx < x - 8 ? 0.9 : 0.94), true);
        if (!d0.fine) continue;
        m.column(tx, GRADE + 6.85, z, 0.45, 1.9, 1.35, 14, shade(P.galv, 1.02), true);
        m.beam(tx - 2.1, tx + 2.1, GRADE, GRADE + 0.35, z - 2.1, z + 2.1, shade(P.concrete, 0.94), 0.03);
        for (let hp = 0; hp < 8; hp += 1) {
          const a = (hp / 8) * Math.PI * 2;
          m.rod([tx + Math.cos(a) * 1.86, GRADE + 7.3, z + Math.sin(a) * 1.86],
            [tx + Math.cos(a) * 1.86, GRADE + 8.3, z + Math.sin(a) * 1.86], 0.035, 4, P.galv, false);
          const b = ((hp + 1) / 8) * Math.PI * 2;
          m.rod([tx + Math.cos(a) * 1.86, GRADE + 8.3, z + Math.sin(a) * 1.86],
            [tx + Math.cos(b) * 1.86, GRADE + 8.3, z + Math.sin(b) * 1.86], 0.035, 4, shade(P.galv, 1.04), false);
        }
      }
      if (d0.fine) {
        for (let s = 0; s < 12; s += 1) {
          m.beam(x - 6.5 + 2.1, x - 6.5 + 3.3, GRADE + 0.35 + s * 0.55, GRADE + 0.45 + s * 0.55,
            z - 2.2 + s * 0.4, z - 1.6 + s * 0.4, P.galv, 0.012);
        }
        m.rod([x - 3.6, GRADE + 1.4, z - 2.2], [x - 3.6, GRADE + 7.4, z + 2.6], 0.035, 6, shade(P.galv, 1.04), false);
      }
      if (tall) {
        m.column(x - 1.0, GRADE, z - 5.5, 17.0, 0.95, 0.75, d0.fine ? 14 : d0.seg, shade(P.clad, 0.88), false);
        if (d0.fine) {
          // Stiffening bands and a ladder. A flue with neither is a cone.
          for (let bnd = 0; bnd < 4; bnd += 1) {
            const by = GRADE + 3.4 + bnd * 3.4, br = 0.95 - (0.2 * (by - GRADE)) / 17;
            m.column(x - 1.0, by, z - 5.5, 0.22, br + 0.07, br + 0.07, 14, shade(P.clad, 0.78), false);
          }
          for (const lx of [-0.16, 0.16]) {
            m.rod([x - 1.0 + lx, GRADE + 1.0, z - 6.6], [x - 1.0 + lx, GRADE + 16.4, z - 6.35], 0.035, 6, P.galv, false);
          }
          for (let r = 0; r < 14; r += 1) {
            const ry = GRADE + 1.4 + r * 1.1;
            m.rod([x - 1.16, ry, z - 6.58], [x - 0.84, ry, z - 6.58], 0.026, 6, P.galv, false);
          }
        }
      }
      if (d0.fine) {
        // The pipe bridge back to the hall, on its own trestles.
        for (let i = 0; i < 4; i += 1) {
          m.rod([x - 11.0, GRADE + 5.0 + i * 0.5, z - 0.2], [x - 6.0, GRADE + 5.0 + i * 0.5, z - 0.2], 0.16, 8, shade(P.steel, 0.96 + 0.04 * (i % 2)), false);
        }
        for (const px of [x - 10.4, x - 6.6]) {
          m.beam(px - 0.12, px + 0.12, GRADE, GRADE + 4.9, z - 0.34, z - 0.06, P.steel, 0.016);
        }
      }
    });
  }

  // ------------------------------------------------------------ generic kit
  // The other twelve. Deliberately plain massing on the same stage kit: a
  // correctly staged shed with the kind's one recognisable feature beside it.
  // They are placeholders and the description says so.

  /// The seat a yard prop is placed from: the middle of the pad, not the middle
  /// of the building. Every one of them positions itself as a fraction of W and
  /// D from the point it is handed, so handing it the lead block's seat adds
  /// that kind's `home` offset a second time — which is how a switchyard's
  /// feeder gantry ended up sixteen metres past its own perimeter fence.
  const YARD = { x: 0, z: 0 };

  // ------------------------------------------------------------- shared props
  // The industrial fittings the kinds are composed from. Each was a branch of
  // one long `if (prop === ...)` chain that every kind walked with a list of
  // names; they are functions now because a kind no longer has "a list of
  // props" — it has a composition, and a composition has to be able to put a
  // tank farm somewhere other than where the chain happened to put it. The
  // bodies are unchanged: same geometry, same names, same triangles.

  function propStack(m, k, x, z, W, D, d0, built) {
    m.part("services / flue stack", () => {
      const sx = x + W * 0.26, sz = z - D * 0.18, sh = built ? 22 : 6;
      m.column(sx, GRADE, sz, sh, 1.3, 1.0, d0.fine ? 14 : d0.seg, shade(P.clad, 0.86), false);
      if (!d0.fine) return;
      // Stiffening bands, a caged ladder and a coping ring. A flue with none
      // of them is a traffic cone with the point cut off.
      m.beam(sx - 1.7, sx + 1.7, GRADE, GRADE + 0.5, sz - 1.7, sz + 1.7, shade(P.concrete, 0.94), 0.03);
      if (built) {
        for (let i = 0; i < 3; i += 1) m.column(sx, GRADE + 8 + i * 4.5, sz, 0.4, 1.5, 1.5, 14, P.safety, true);
        m.column(sx, GRADE + sh - 0.4, sz, 0.5, 1.12, 1.12, 14, shade(P.clad, 0.7), false);
        for (const lx of [-0.16, 0.16]) m.rod([sx + lx, GRADE + 1.0, sz - 1.4], [sx + lx, GRADE + sh - 0.8, sz - 1.12], 0.035, 6, P.galv, false);
        for (let r = 0; r < 12; r += 1) {
          m.rod([sx - 0.16, GRADE + 1.4 + r * 1.6, sz - 1.38], [sx + 0.16, GRADE + 1.4 + r * 1.6, sz - 1.38], 0.026, 6, P.galv, false);
        }
      }
    });
  }

  function propTanks(m, k, x, z, W, D, d0, built) {
    m.part("services / tank farm", () => {
      const n = d0.fine ? 3 : 2, th = built ? 9 : 2.5;
      for (let i = 0; i < n; i += 1) {
        const tz = z + D * 0.1 + i * 6.5, tx = x + W * 0.3;
        m.column(tx, GRADE + (d0.fine ? 0.4 : 0), tz, th, 2.6, 2.6, d0.fine ? 14 : d0.seg, shade(P.galv, 0.95 + i * 0.03), true);
        if (!d0.fine) continue;
        // Plinth, dished top, top rail. A storage tank is a rolled shell you
        // walk on the top of, and a flat lid with no rail is a bin.
        m.beam(tx - 2.9, tx + 2.9, GRADE, GRADE + 0.4, tz - 2.9, tz + 2.9, shade(P.concrete, 0.94), 0.03);
        if (built) {
          m.column(tx, GRADE + 0.4 + th, tz, 0.55, 2.6, 1.8, 14, shade(P.galv, 1.04), true);
          for (let hp = 0; hp < 8; hp += 1) {
            const a = (hp / 8) * Math.PI * 2, b2 = ((hp + 1) / 8) * Math.PI * 2;
            m.rod([tx + Math.cos(a) * 2.5, GRADE + 0.4 + th, tz + Math.sin(a) * 2.5],
              [tx + Math.cos(a) * 2.5, GRADE + 1.4 + th, tz + Math.sin(a) * 2.5], 0.035, 4, P.galv, false);
            m.rod([tx + Math.cos(a) * 2.5, GRADE + 1.4 + th, tz + Math.sin(a) * 2.5],
              [tx + Math.cos(b2) * 2.5, GRADE + 1.4 + th, tz + Math.sin(b2) * 2.5], 0.035, 4, shade(P.galv, 1.04), false);
          }
          m.rod([tx - 2.6, GRADE + 1.2, tz], [tx - 4.6, GRADE + 1.2, tz], 0.16, 8, shade(P.steel, 1.0), true);
        }
      }
      // The bund wall the farm stands in, which is what makes it a farm.
      if (d0.fine) {
        const bz0 = z + D * 0.1 - 3.6, bz1 = z + D * 0.1 + (n - 1) * 6.5 + 3.6;
        for (const s of [-1, 1]) {
          m.beam(x + W * 0.3 + s * 3.8, x + W * 0.3 + s * 4.1, GRADE, GRADE + 1.1, bz0, bz1, shade(P.concrete, 0.9), 0.03);
        }
        for (const bz of [bz0, bz1]) {
          m.beam(x + W * 0.3 - 4.1, x + W * 0.3 + 4.1, GRADE, GRADE + 1.1, bz - 0.15, bz + 0.15, shade(P.concrete, 0.9), 0.03);
        }
      }
    });
  }

  function propPipeRack(m, k, x, z, W, D, d0, built) {
    m.part("services / pipe rack", () => {
      const rz = z - D * 0.3;
      for (let i = 0; i < (d0.fine ? 5 : 2); i += 1) {
        const px = x - W * 0.3 + (i * W * 0.6) / 4;
        if (d0.fine) m.beam(px - 0.16, px + 0.16, GRADE, GRADE + 5.5, rz - 0.16, rz + 0.16, P.steel, 0.018);
        else m.bar(px - 0.16, px + 0.16, GRADE, GRADE + 5.5, rz - 0.16, rz + 0.16, P.steel);
      }
      for (let i = 0; i < (d0.fine ? 3 : 1); i += 1) {
        m.bar(x - W * 0.32, x + W * 0.32, GRADE + 4.4 + i * 0.5, GRADE + 4.7 + i * 0.5, rz - 0.25, rz + 0.25, shade(P.galv, 0.94));
      }
      if (!d0.fine) return;
      // The pipes themselves, which the rack existed to carry and did not
      // have. Round, lagged in two diameters, on shoes, with a loop at one
      // end for the expansion a hot line needs.
      const runs = [[0.34, 4.95, 0.9], [0.24, 5.45, 1.02], [0.18, 5.95, 0.88]];
      for (let r = 0; r < runs.length; r += 1) {
        const rr = runs[r][0], ry = GRADE + runs[r][1];
        m.rod([x - W * 0.32, ry + rr, rz - 0.12 + r * 0.14], [x + W * 0.28, ry + rr, rz - 0.12 + r * 0.14],
          rr, 10, shade(P.galv, runs[r][2]), false);
        m.rod([x + W * 0.28, ry + rr, rz - 0.12 + r * 0.14], [x + W * 0.28, ry + rr + 1.6, rz - 0.12 + r * 0.14],
          rr, 10, shade(P.galv, runs[r][2] * 0.96), true);
      }
    });
  }

  function propTransformers(m, k, x, z, W, D, d0, built) {
    m.part("services / transformer bays", () => {
      for (let i = 0; i < (d0.fine ? 3 : 2); i += 1) {
        const tx = x - W * 0.22 + i * 7.5, tz = z + D * 0.16;
        m.bar(tx - 2.2, tx + 2.2, GRADE + (d0.fine ? 0.2 : 0), GRADE + 3.2, tz - 1.6, tz + 1.6, shade(P.steel, 1.05));
        if (!d0.fine) continue;
        // Bushings, radiator banks, a conservator and the blast wall between
        // bays. All four are on every transformer and none of them is more
        // than a dozen triangles.
        for (const s of [-1, 1]) m.column(tx + s * 1.2, GRADE + 3.2, tz, 1.5, 0.35, 0.28, 10, P.hardcore, true);
        m.rod([tx - 1.6, GRADE + 3.5, tz - 1.2], [tx + 1.6, GRADE + 3.5, tz - 1.2], 0.34, 10, shade(P.galv, 0.98), true);
        for (let r = 0; r < 7; r += 1) {
          m.bar(tx - 1.9 + r * 0.55, tx - 1.72 + r * 0.55, GRADE + 0.6, GRADE + 2.8, tz + 1.6, tz + 2.05, shade(P.steel, 0.88 + 0.06 * (r % 2)));
        }
        m.beam(tx - 2.6, tx + 2.6, GRADE, GRADE + 0.2, tz - 2.2, tz + 2.3, shade(P.concrete, 0.92), 0.03);
        m.bar(tx - 2.6, tx + 2.6, GRADE, GRADE + 4.2, tz + 2.2, tz + 2.3, P.galv);
      }
    });
  }

  function propPylon(m, k, x, z, W, D, d0, built) {
    m.part("services / feeder gantry", () => {
      const px = x - W * 0.3, py = built ? 14 : 5, pz = z - D * 0.2;
      for (const sx of [-1, 1]) {
        for (const sz of [-1, 1]) {
          if (d0.fine) m.rod([px + sx * 1.5, GRADE, pz + sz * 1.5], [px + sx * 1.5, GRADE + py, pz + sz * 1.5], 0.14, 6, P.galv, false);
          else m.bar(px + sx * 1.5 - 0.14, px + sx * 1.5 + 0.14, GRADE, GRADE + py, pz + sz * 1.5 - 0.14, pz + sz * 1.5 + 0.14, P.galv);
        }
      }
      m.bar(px - 4.5, px + 4.5, GRADE + py, GRADE + py + 0.4, pz - 0.25, pz + 0.25, P.galv);
      if (!d0.fine) return;
      // A braced lattice and real insulator strings. A gantry is four legs
      // and the bracing between them; without the bracing it is four poles,
      // and the discs hanging off the crosshead are what say "live".
      m.save().move(px, 0, pz);
      for (let bay = 0; bay < 5; bay += 1) {
        latticeBay(m, GRADE + (py * bay) / 5, GRADE + (py * (bay + 1)) / 5, 1.5, 1.5, 0.07, 6, shade(P.galv, 0.92));
      }
      m.restore();
      for (let i = 0; i < 3; i += 1) {
        const ax = px - 3.4 + i * 3.4;
        for (let d = 0; d < 5; d += 1) {
          m.column(ax, GRADE + py - 0.28 - d * 0.24, pz, 0.16, 0.19, 0.19, 8, shade(P.hardcore, 1.0 - 0.03 * d), false);
        }
        m.rod([ax, GRADE + py - 1.5, pz], [ax + 3.0, GRADE + py - 2.4, pz - 6.0], 0.05, 6, P.dark, false);
      }
    });
  }

  function propContainers(m, k, x, z, W, D, d0, built) {
    m.part("yard / container stacks", () => {
      const n = d0.fine ? 6 : 2;
      for (let i = 0; i < n; i += 1) {
        const cx = x - W * 0.3 + (i % 3) * 7.0, cz = z + D * 0.18 + Math.floor(i / 3) * 3.2;
        const high = built ? 2 : 1;
        for (let h = 0; h < high; h += 1) {
          const y0 = GRADE + h * 2.6, col = shade(k.accent, 0.82 + 0.12 * ((i + h) % 3));
          m.bar(cx - 3.0, cx + 3.0, y0, y0 + 2.5, cz - 1.2, cz + 1.2, col);
          if (!d0.fine) continue;
          // Corner castings, corrugation, and doors with locking bars on one
          // end. A container is the most recognisable object in a freight
          // yard and it is recognisable by exactly those three things.
          for (const ex of [-3.0, 3.0]) {
            for (const ez of [-1.2, 1.2]) {
              m.bar(cx + ex - 0.17, cx + ex + 0.17, y0, y0 + 2.5, cz + ez - 0.17, cz + ez + 0.17, shade(P.steel, 1.0));
            }
          }
          for (let r = 0; r < 6; r += 1) {
            const rx = cx - 2.3 + r * 0.92;
            for (const fz of [cz - 1.26, cz + 1.2]) {
              m.bar(rx - 0.08, rx + 0.08, y0 + 0.1, y0 + 2.4, fz, fz + 0.06, shade(col, 0.8));
            }
          }
          for (const bx of [cx + 2.4, cx + 2.72]) {
            m.rod([bx, y0 + 0.14, cz + 1.24], [bx, y0 + 2.36, cz + 1.24], 0.05, 6, shade(P.steel, 0.94), false);
          }
        }
      }
    });
  }

  function propGantry(m, k, x, z, W, D, d0, built) {
    m.part("yard / transfer gantry", () => {
      const gz = z + D * 0.18, h = built ? 12 : 4;
      for (const sx of [-1, 1]) {
        if (d0.fine) {
          // Each leg is an A-frame of four chords with bracing, not a post.
          m.save().move(x + sx * 13, 0, gz);
          for (const cx of [-0.5, 0.5]) {
            for (const cz of [-0.5, 0.5]) m.rod([cx, GRADE, cz], [cx, GRADE + h, cz], 0.12, 6, P.safety, false);
          }
          for (let bay = 0; bay < 4; bay += 1) {
            latticeBay(m, GRADE + (h * bay) / 4, GRADE + (h * (bay + 1)) / 4, 0.5, 0.5, 0.06, 6, shade(P.safety, 0.92));
          }
          m.restore();
          m.beam(x + sx * 13 - 1.2, x + sx * 13 + 1.2, GRADE, GRADE + 0.6, gz - 1.2, gz + 1.2, P.concrete, 0.03);
          for (const wz of [gz - 0.8, gz + 0.8]) {
            m.rod([x + sx * 13 - 0.5, GRADE + 0.3, wz], [x + sx * 13 + 0.5, GRADE + 0.3, wz], 0.3, 8, P.dark, true);
          }
        } else {
          m.bar(x + sx * 13 - 0.5, x + sx * 13 + 0.5, GRADE, GRADE + h, gz - 0.5, gz + 0.5, P.safety);
        }
      }
      m.bar(x - 14, x + 14, GRADE + h, GRADE + h + 1.3, gz - 0.9, gz + 0.9, P.safety);
      if (d0.fine) {
        // Trolley and spreader on the beam, and the rails the legs run on.
        m.beam(x - 2, x + 2, GRADE + h - 0.5, GRADE + h, gz - 0.8, gz + 0.8, shade(P.safety, 1.06), 0.025);
        for (const rx of [-1.4, 1.4]) m.rod([x + rx, GRADE + h - 3.4, gz], [x + rx, GRADE + h - 0.5, gz], 0.05, 6, P.dark, false);
        m.beam(x - 3.0, x + 3.0, GRADE + h - 3.9, GRADE + h - 3.4, gz - 1.3, gz + 1.3, P.dark, 0.03);
        for (const rs of [-1, 1]) {
          m.bar(x - 15, x + 15, GRADE, GRADE + 0.22, gz + rs * 0.62 - 0.1, gz + rs * 0.62 + 0.1, shade(P.steel, 1.0));
        }
      }
    });
  }

  function propYardCrane(m, k, x, z, W, D, d0, built) {
    m.part("yard / outdoor crane rail", () => {
      const rz = z - D * 0.28;
      for (const s of [-1, 1]) {
        m.bar(x - W * 0.34, x + W * 0.34, GRADE, GRADE + 0.25, rz + s * 4 - 0.3, rz + s * 4 + 0.3, P.steel);
        // Sleepers under the rail. A crane rail laid straight on the yard is
        // a stripe; the sleepers are what make it track.
        if (d0.fine) {
          for (let t = 0; t < 9; t += 1) {
            const tx = x - W * 0.32 + (t * W * 0.64) / 8;
            m.bar(tx - 0.3, tx + 0.3, GRADE, GRADE + 0.12, rz + s * 4 - 0.7, rz + s * 4 + 0.7, shade(P.timber, 0.86));
          }
        }
      }
      if (built) {
        for (const s of [-1, 1]) {
          if (d0.fine) {
            m.save().move(x, 0, rz + s * 4);
            for (const cx of [-0.34, 0.34]) {
              for (const cz of [-0.34, 0.34]) m.rod([cx, GRADE + 0.25, cz], [cx, GRADE + 7.5, cz], 0.1, 6, P.safety, false);
            }
            for (let bay = 0; bay < 3; bay += 1) {
              latticeBay(m, GRADE + 0.25 + (7.25 * bay) / 3, GRADE + 0.25 + (7.25 * (bay + 1)) / 3, 0.34, 0.34, 0.05, 6, shade(P.safety, 0.92));
            }
            m.restore();
          } else {
            m.bar(x - 0.4, x + 0.4, GRADE + 0.25, GRADE + 7.5, rz + s * 4 - 0.4, rz + s * 4 + 0.4, P.safety);
          }
        }
        m.bar(x - 1.0, x + 1.0, GRADE + 7.5, GRADE + 8.4, rz - 4.6, rz + 4.6, P.safety);
        if (d0.fine) {
          m.beam(x - 1.2, x + 1.2, GRADE + 6.9, GRADE + 7.5, rz - 1.0, rz + 1.0, P.dark, 0.025);
          m.rod([x, GRADE + 4.4, rz], [x, GRADE + 6.9, rz], 0.05, 6, P.dark, false);
          m.column(x, GRADE + 3.8, rz, 0.6, 0.3, 0.3, 8, shade(P.steel, 0.9), true);
        }
      }
    });
  }

  function propDocks(m, k, x, z, W, D, d0, built) {
    m.part("yard / loading docks", () => {
      const dz0 = z - D * 0.24;
      m.bar(x - W * 0.3, x + W * 0.3, GRADE, GRADE + 1.25, dz0 - 2.4, dz0 + 2.4, P.concrete);
      const n = d0.fine ? 5 : 2;
      for (let i = 0; i < n; i += 1) {
        const dx = x - W * 0.26 + (i * W * 0.52) / (n - 1);
        m.bar(dx - 1.5, dx + 1.5, GRADE + 1.25, GRADE + 1.36, dz0 - 2.5, dz0 - 0.9, shade(P.steel, 1.05));
        if (!d0.fine) continue;
        // Shutter, buffers and a leveller lip on every bay, so a dock reads
        // as somewhere a lorry backs onto rather than a step in a wall.
        m.bar(dx - 1.7, dx + 1.7, GRADE + 1.36, GRADE + 3.9, dz0 - 2.58, dz0 - 2.46, P.dark);
        for (let sl = 0; sl < 5; sl += 1) {
          m.bar(dx - 1.62, dx + 1.62, GRADE + 1.5 + sl * 0.48, GRADE + 1.86 + sl * 0.48, dz0 - 2.62, dz0 - 2.58, shade(P.door, 1.06));
        }
        for (const s of [-1, 1]) {
          m.beam(dx + s * 1.82 - 0.16, dx + s * 1.82 + 0.16, GRADE + 0.6, GRADE + 1.3, dz0 - 2.6, dz0 - 2.34, shade(P.dark, 1.15), 0.02);
        }
        m.beam(dx - 1.45, dx + 1.45, GRADE + 1.36, GRADE + 1.46, dz0 - 2.66, dz0 - 2.4, shade(P.steel, 1.12), 0.015);
      }
      if (d0.fine) {
        m.beam(x - W * 0.31, x + W * 0.31, GRADE + 1.2, GRADE + 1.34, dz0 - 2.5, dz0 + 2.5, P.kerb, 0.025);
        groundPatch(m, x, dz0 - 7.5, W * 0.6, 9, 9, 5, GRADE + 0.01, P.asphalt, 0.055);
      }
    });
  }

  function propRoadStrip(m, k, x, z, W, D, d0, built) {
    m.part("yard / access road", () => {
      m.save().move(0, GRADE, 0);
      m.plate([[x - W * 0.42, z - D * 0.44], [x + W * 0.42, z - D * 0.44], [x + W * 0.42, z - D * 0.30], [x - W * 0.42, z - D * 0.30]], 0.1, P.asphalt);
      m.restore();
      if (d0.fine) {
        for (let i = 0; i < 7; i += 1) {
          const lx = x - W * 0.36 + (i * W * 0.72) / 6;
          m.bar(lx - 1.1, lx + 1.1, GRADE + 0.1, GRADE + 0.13, z - D * 0.375, z - D * 0.365, P.lane);
        }
        // Kerbs and a run-in of worn hardstanding either side. A road with no
        // edge is a grey rectangle.
        for (const s of [-1, 1]) {
          const kz = z - D * (s < 0 ? 0.445 : 0.295);
          m.beam(x - W * 0.42, x + W * 0.42, GRADE + 0.1, GRADE + 0.32, kz - 0.11, kz + 0.11, P.kerb, 0.022);
        }
        groundPatch(m, x, z - D * 0.375, W * 0.8, D * 0.11, 12, 3, GRADE + 0.105, P.asphalt, 0.05);
      }
    });
  }

  function propPipeStack(m, k, x, z, W, D, d0, built) {
    m.part("yard / stacked sections", () => materialStack(m, x - W * 0.3, z + D * 0.26, 1, d0));
  }

  function propKiosk(m, k, x, z, W, D, d0, built) {
    m.part("services / control kiosk", () => {
      const kx = x + W * 0.32, kz = z - D * 0.3;
      m.bar(kx - 2.2, kx + 2.2, GRADE, GRADE + 2.9, kz - 1.6, kz + 1.6, shade(P.clad, 1.04));
      if (d0.fine) m.beam(kx - 2.4, kx + 2.4, GRADE + 2.9, GRADE + 3.12, kz - 1.8, kz + 1.8, P.roof, 0.025);
      else m.bar(kx - 2.4, kx + 2.4, GRADE + 2.9, GRADE + 3.1, kz - 1.8, kz + 1.8, P.roof);
      if (d0.fine) {
        m.bar(kx - 0.6, kx + 0.6, GRADE + 0.1, GRADE + 2.2, kz + 1.6, kz + 1.64, P.door);
        m.beam(kx - 0.72, kx + 0.72, GRADE + 0.04, GRADE + 2.32, kz + 1.6, kz + 1.72, shade(P.galv, 0.98), 0.015);
        m.bar(kx + 0.7, kx + 1.9, GRADE + 1.5, GRADE + 2.3, kz + 1.6, kz + 1.63, P.glass);
        m.beam(kx + 0.58, kx + 2.02, GRADE + 1.4, GRADE + 2.4, kz + 1.6, kz + 1.7, shade(P.galv, 1.0), 0.015);
        m.beam(kx - 1.0, kx + 1.0, GRADE, GRADE + 0.1, kz + 1.6, kz + 2.2, shade(P.concrete, 0.94), 0.02);
        for (const lv of [-1.5, -0.9]) {
          louvreBank(m, kx + lv, kx + lv + 0.5, GRADE + 1.6, GRADE + 2.3, kz + 1.6, 0.1, 3, shade(P.galv, 0.96));
        }
      }
    });
  }

  function propGlazing(m, k, x, z, W, D, d0, built) {
    m.part("envelope / strip glazing", () => {
      const b = k.block;
      const n = d0.fine ? 8 : 3;
      for (const s of [-1, 1]) {
        for (let i = 0; i < n; i += 1) {
          const gx = -b.w / 2 + 1.5 + (i * (b.w - 3)) / (n - 1);
          m.bar(x + gx - 0.8, x + gx + 0.8, GRADE + 3.2, GRADE + 5.4, z + s * (b.dz / 2) - 0.05, z + s * (b.dz / 2) + 0.05, P.glass);
          if (!d0.fine) continue;
          // Frame, transom and a sill with a drip. Strip glazing without a
          // surround is a row of blue rectangles painted on a wall.
          m.beam(x + gx - 0.92, x + gx + 0.92, GRADE + 3.08, GRADE + 5.52, z + s * (b.dz / 2) - 0.09, z + s * (b.dz / 2) + 0.09, shade(P.galv, 1.0), 0.015);
          m.beam(x + gx - 0.05, x + gx + 0.05, GRADE + 3.2, GRADE + 5.4, z + s * (b.dz / 2) - 0.09, z + s * (b.dz / 2) + 0.09, shade(P.galv, 1.02), 0.012);
          m.bar(x + gx - 0.98, x + gx + 0.98, GRADE + 2.94, GRADE + 3.08, z + s * (b.dz / 2) - 0.16, z + s * (b.dz / 2) + 0.16, shade(P.galv, 0.84));
        }
      }
    });
  }

  function propPlantRoom(m, k, x, z, W, D, d0, built) {
    m.part("services / roof plant", () => {
      const b = k.block;
      m.bar(x - 4, x + 4, GRADE + b.eaves + 0.2, GRADE + b.eaves + 2.6, z - 3, z + 3, shade(P.galv, 0.95));
      if (!d0.fine) return;
      for (let i = 0; i < 3; i += 1) m.column(x - 2.4 + i * 2.4, GRADE + b.eaves + 2.6, z, 1.1, 0.7, 0.7, 12, P.dark, true);
      // A plinth, a louvre face and a handrail round the edge, which is what
      // is on the roof of every plant room ever built.
      m.beam(x - 4.3, x + 4.3, GRADE + b.eaves, GRADE + b.eaves + 0.2, z - 3.3, z + 3.3, shade(P.concrete, 0.92), 0.03);
      louvreBank(m, x - 3.2, x + 3.2, GRADE + b.eaves + 0.6, GRADE + b.eaves + 2.2, z + 3.0, 0.16, 5, shade(P.galv, 0.98));
      for (const rx of [-4.4, 4.4]) {
        m.rod([x + rx, GRADE + b.eaves + 0.2, z - 3.4], [x + rx, GRADE + b.eaves + 1.3, z - 3.4], 0.035, 6, P.galv, false);
        m.rod([x + rx, GRADE + b.eaves + 0.2, z + 3.4], [x + rx, GRADE + b.eaves + 1.3, z + 3.4], 0.035, 6, P.galv, false);
        m.rod([x + rx, GRADE + b.eaves + 1.3, z - 3.4], [x + rx, GRADE + b.eaves + 1.3, z + 3.4], 0.035, 6, shade(P.galv, 1.04), false);
      }
    });
  }

  function propAnnex(m, k, x, z, W, D, d0, built) {
    m.part("structure / service annex", () => {
      const b = k.block;
      m.bar(x + b.w / 2, x + b.w / 2 + 9, GRADE, GRADE + 5.2, z - 5, z + 5, shade(k.body, 0.95));
      if (d0.fine) m.beam(x + b.w / 2 - 0.2, x + b.w / 2 + 9.2, GRADE + 5.2, GRADE + 5.54, z - 5.3, z + 5.3, P.roof, 0.03);
      else m.bar(x + b.w / 2 - 0.2, x + b.w / 2 + 9.2, GRADE + 5.2, GRADE + 5.5, z - 5.3, z + 5.3, P.roof);
      if (!d0.fine) return;
      m.bar(x + b.w / 2 + 3.4, x + b.w / 2 + 5.6, GRADE + 0.05, GRADE + 2.4, z + 5.0, z + 5.04, P.door);
      m.beam(x + b.w / 2 + 3.28, x + b.w / 2 + 5.72, GRADE, GRADE + 2.52, z + 5.0, z + 5.12, shade(P.galv, 0.98), 0.015);
      for (let i = 0; i < 3; i += 1) {
        m.bar(x + b.w / 2 + 0.9 + i * 3.0, x + b.w / 2 + 2.3 + i * 3.0, GRADE + 3.2, GRADE + 4.4, z - 5.04, z - 5.0, P.glass);
      }
      for (const py of [1.6, 3.2, 4.8]) {
        m.bar(x + b.w / 2 + 8.9, x + b.w / 2 + 9.06, GRADE, GRADE + py, z - 4.4, z - 4.2, shade(P.galv, 0.9));
      }
    });
  }

  function propLeanTo(m, k, x, z, W, D, d0, built) {
    m.part("structure / open lean-to", () => {
      const b = k.block;
      const n = d0.fine ? 4 : 2;
      for (let i = 0; i < n; i += 1) {
        const px = x - b.w / 2 + (i * b.w) / (n - 1);
        if (d0.fine) m.beam(px - 0.14, px + 0.14, GRADE, GRADE + 3.6, z + b.dz / 2 + 5.2, z + b.dz / 2 + 5.5, P.steel, 0.016);
        else m.bar(px - 0.14, px + 0.14, GRADE, GRADE + 3.6, z + b.dz / 2 + 5.2, z + b.dz / 2 + 5.5, P.steel);
      }
      m.save().move(0, GRADE + 3.6, 0);
      m.plate([[x - b.w / 2 - 0.4, z + b.dz / 2], [x + b.w / 2 + 0.4, z + b.dz / 2], [x + b.w / 2 + 0.4, z + b.dz / 2 + 5.8], [x - b.w / 2 - 0.4, z + b.dz / 2 + 5.8]], 0.2, P.roof);
      m.restore();
      if (!d0.fine) return;
      // Purlins under the sheet, a rail across the open front, and something
      // actually stored under it — an empty canopy reads as unfinished.
      for (let p = 0; p < 4; p += 1) {
        const pz = z + b.dz / 2 + 0.6 + p * 1.5;
        m.bar(x - b.w / 2 - 0.3, x + b.w / 2 + 0.3, GRADE + 3.4, GRADE + 3.58, pz - 0.09, pz + 0.09, shade(P.steel, 0.9));
      }
      m.beam(x - b.w / 2, x + b.w / 2, GRADE + 3.6, GRADE + 3.82, z + b.dz / 2 + 5.6, z + b.dz / 2 + 5.86, shade(P.roof, 1.1), 0.02);
      materialStack(m, x - b.w * 0.22, z + b.dz / 2 + 3.0, 2, d0);
    });
  }

  function propRobotCell(m, k, x, z, W, D, d0, built) {
    m.part("structure / machine cell annex", () => {
      const b = k.block;
      m.bar(x - b.w / 2 - 8, x - b.w / 2, GRADE, GRADE + 4.6, z - 4, z + 4, shade(k.accent, 1.05));
      if (!d0.fine) return;
      for (let i = 0; i < 3; i += 1) m.column(x - b.w / 2 - 6 + i * 2.4, GRADE + 4.6, z, 1.4, 0.5, 0.4, 12, P.galv, true);
      // Extract stacks want a weather cowl; a roll shutter and a fire escape
      // are what the rest of the elevation has.
      for (let i = 0; i < 3; i += 1) {
        m.column(x - b.w / 2 - 6 + i * 2.4, GRADE + 6.0, z, 0.3, 0.62, 0.42, 12, shade(P.dark, 1.2), true);
      }
      m.beam(x - b.w / 2 - 8.2, x - b.w / 2 + 0.2, GRADE + 4.6, GRADE + 4.84, z - 4.2, z + 4.2, P.roof, 0.025);
      m.bar(x - b.w / 2 - 6.4, x - b.w / 2 - 3.6, GRADE + 0.05, GRADE + 3.4, z + 4.0, z + 4.04, P.door);
      for (let s = 0; s < 6; s += 1) {
        m.bar(x - b.w / 2 - 6.32, x - b.w / 2 - 3.68, GRADE + 0.2 + s * 0.52, GRADE + 0.58 + s * 0.52, z + 4.04, z + 4.1, shade(P.door, 1.08));
      }
      louvreBank(m, x - b.w / 2 - 2.8, x - b.w / 2 - 0.8, GRADE + 2.4, GRADE + 3.8, z + 4.0, 0.14, 4, shade(P.galv, 0.96));
    });
  }

  function propDuctRun(m, k, x, z, W, D, d0, built) {
    m.part("services / heat recovery ducts", () => {
      const b = k.block;
      const n = d0.fine ? 4 : 2;
      for (let i = 0; i < n; i += 1) {
        const dx = x - b.w * 0.3 + (i * b.w * 0.6) / (n - 1);
        m.column(dx, GRADE + b.eaves, z, 3.0, 0.75, 0.75, d0.fine ? 12 : d0.seg, shade(P.galv, 1.02), true);
        if (!d0.fine) continue;
        // Flanged joints and a saddle at the roof line. Ductwork is made in
        // lengths, and the flange rings are how anyone knows that.
        for (const fy of [0.1, 1.5, 2.9]) {
          m.column(dx, GRADE + b.eaves + fy, z, 0.12, 0.88, 0.88, 12, shade(P.galv, 0.86), false);
        }
        m.beam(dx - 0.95, dx + 0.95, GRADE + b.eaves - 0.25, GRADE + b.eaves + 0.05, z - 0.95, z + 0.95, shade(P.steel, 0.92), 0.02);
      }
      m.bar(x - b.w * 0.34, x + b.w * 0.34, GRADE + b.eaves + 2.6, GRADE + b.eaves + 3.4, z - 0.8, z + 0.8, shade(P.galv, 0.95));
      if (d0.fine) {
        m.beam(x - b.w * 0.36, x + b.w * 0.36, GRADE + b.eaves + 3.4, GRADE + b.eaves + 3.62, z - 0.95, z + 0.95, shade(P.galv, 1.06), 0.02);
        m.column(x + b.w * 0.30, GRADE + b.eaves + 3.62, z, 1.1, 0.55, 0.55, 12, shade(P.galv, 1.0), false);
        m.column(x + b.w * 0.30, GRADE + b.eaves + 4.72, z, 0.3, 0.72, 0.5, 12, shade(P.dark, 1.2), true);
      }
    });
  }

  function propHost(m, k, x, z, W, D, d0, built) {
    // A retrofit is installed INTO something that already exists, and the
    // roadmap is explicit that automation and efficiency are not new
    // standalone buildings. The host shed is therefore drawn complete at
    // every stage: the staged work is the kit going into it.
    m.part("structure / existing host facility", () => {
      // BUTTED, not standing nearby. A retrofit is installed INTO a
      // facility that already exists, and the annex carrying the new work
      // shares a wall with it: the host's near face lands exactly on the
      // annex's far face, whatever the annex is. Twenty metres away it
      // read as two separate buildings, which is the one thing roadmap
      // section E says this kind must not be.
      const b = k.block, hx = x + b.w / 2 + 11;
      m.bar(hx - 11, hx + 11, GRADE, GRADE + 7.0, z - 8, z + 8, shade(P.clad, 0.9));
      m.save().move(hx, GRADE, z);
      roof(m, 22, 16, 7.0, 8.8, d0.fine ? 6 : 2, shade(P.roof, 0.92), d0, { clad: 1 });
      m.restore();
      if (d0.fine) {
        m.bar(hx - 4, hx + 4, GRADE + 0.1, GRADE + 4.4, z - 8.05, z - 7.95, P.door);
        for (let s = 0; s < 8; s += 1) {
          m.bar(hx - 3.9, hx + 3.9, GRADE + 0.25 + s * 0.52, GRADE + 0.63 + s * 0.52, z - 8.11, z - 8.05, shade(P.door, 1.07));
        }
        for (let i = 0; i < 4; i += 1) {
          m.bar(hx - 9 + i * 6, hx - 7 + i * 6, GRADE + 4.8, GRADE + 6.2, z + 7.95, z + 8.05, P.glass);
          m.beam(hx - 9.12 + i * 6, hx - 6.88 + i * 6, GRADE + 4.68, GRADE + 6.32, z + 7.95, z + 8.09, shade(P.galv, 1.0), 0.015);
        }
        // The host is an OLDER building, and it says so: ribbed sheeting, a
        // rusted base flashing, a dock at one end and a raked apron. That is
        // the point of a retrofit — the plant already exists and the staged
        // work is the kit going into it.
        for (let r = 0; r < 9; r += 1) {
          const rx = hx - 10 + r * 2.5;
          for (const fz of [z - 8.02, z + 7.96]) m.bar(rx - 0.08, rx + 0.08, GRADE + 0.2, GRADE + 6.9, fz, fz + 0.07, shade(P.clad, 0.8));
        }
        m.beam(hx - 11.1, hx + 11.1, GRADE, GRADE + 0.36, z - 8.15, z + 8.15, shade(P.primer, 0.86), 0.02);
        m.beam(hx - 11.2, hx + 11.2, GRADE + 6.86, GRADE + 7.06, z - 8.2, z + 8.2, shade(P.clad, 1.08), 0.025);
        m.bar(hx + 4.6, hx + 10.4, GRADE, GRADE + 1.2, z - 10.4, z - 8.0, P.concrete);
        groundPatch(m, hx, z - 12.5, 24, 8, 9, 4, GRADE + 0.01, P.asphalt, 0.055);
      }
      void b;
    });
  }


  // ---------------------------------------------------------------- kind kit
  // Pieces more than one composition needs. Nothing here decides how built it
  // is: the caller passes the stage in, because the stage is a pure function of
  // recorded work and this file has exactly one place that reads it.

  /// Ballasted track. A freight yard and a works siding are the same object: a
  /// ballast shoulder, sleepers across it, two rails on the sleepers, and
  /// something at the end that stops a wagon. The rails are chamfered because
  /// the running edge of a rail is the brightest line in a yard, and the
  /// sleeper rhythm is the only thing that gives a siding a length.
  function railTrack(m, x0, x1, z, d0, buffer) {
    m.bar(x0 - 1.4, x1 + 1.4, GRADE, GRADE + 0.34, z - 2.3, z + 2.3, shade(P.hardcore, 0.94));
    if (d0.fine) {
      const n = Math.max(4, Math.round((x1 - x0) / 2.6));
      for (let i = 0; i < n; i += 1) {
        const sx = x0 + ((i + 0.5) * (x1 - x0)) / n;
        m.bar(sx - 0.14, sx + 0.14, GRADE + 0.34, GRADE + 0.46, z - 1.3, z + 1.3, shade(P.timber, 0.86 + 0.08 * (i % 2)));
      }
    }
    if (d0.fine) {
      for (const s of [-1, 1]) {
        m.beam(x0, x1, GRADE + 0.46, GRADE + 0.62, z + s * 0.72 - 0.07, z + s * 0.72 + 0.07, P.steel, 0.014);
      }
    } else {
      // Two rails 1.44 m apart are one line at map range, and the ballast under
      // them is the part that is actually a metre and a half wide.
      m.bar(x0, x1, GRADE + 0.34, GRADE + 0.5, z - 0.8, z + 0.8, shade(P.steel, 0.96));
    }
    if (!buffer) return;
    // The stop block: a ballasted mass with a rubbing plate on the front and
    // two rails run up into it, which is what a friction stop actually is.
    m.bar(x1 - 1.6, x1 + 0.3, GRADE + 0.46, GRADE + 1.5, z - 1.05, z + 1.05, shade(P.safety, 0.9));
    if (!d0.fine) return;
    m.beam(x1 + 0.2, x1 + 0.34, GRADE + 0.7, GRADE + 1.6, z - 1.1, z + 1.1, shade(P.dark, 1.1), 0.02);
    for (const s of [-1, 1]) {
      m.rod([x1 - 1.7, GRADE + 0.62, z + s * 0.72], [x1 - 0.3, GRADE + 1.32, z + s * 0.72], 0.085, 6, shade(P.safety, 1.05), false);
    }
  }

  /// A grating deck with a handrail: the floor of a process structure, the
  /// platform a fan bank stands on, the walkway round a vessel. Grating is
  /// drawn as slats, because a solid plate at deck level reads as a shadow and
  /// the slats are what say a person could stand on it.
  function gratedDeck(m, x, z, w, d, y, d0, rails) {
    m.bar(x - w / 2, x + w / 2, y - 0.26, y - 0.06, z - d / 2, z + d / 2, shade(P.steel, 0.88));
    if (d0.fine) {
      const n = Math.max(3, Math.round(d / 1.1));
      for (let i = 0; i < n; i += 1) {
        const z0 = z - d / 2 + (i * d) / n;
        m.bar(x - w / 2, x + w / 2, y - 0.06, y, z0 + 0.07, z0 + d / n - 0.07, shade(P.steel, 0.96 + 0.08 * (i % 2)));
      }
    } else {
      deck(m, x - w / 2, x + w / 2, y, z - d / 2, z + d / 2, shade(P.steel, 0.98));
    }
    // A 40 mm handrail tube is invisible at map range and a deck full of them
    // is 70 triangles that resolve to nothing, so the coarse path stops at the
    // deck. Every process structure in this file is several decks high.
    if (!rails || !d0.fine) return;
    const posts = 5;
    for (const s of [-1, 1]) {
      for (let i = 0; i < posts; i += 1) {
        const px = x - w / 2 + (i * w) / (posts - 1);
        m.rod([px, y, z + s * (d / 2)], [px, y + 1.1, z + s * (d / 2)], 0.04, 6, P.galv, false);
      }
      m.rod([x - w / 2, y + 1.1, z + s * (d / 2)], [x + w / 2, y + 1.1, z + s * (d / 2)], 0.04, 6, shade(P.galv, 1.05), false);
      if (d0.fine) {
        m.rod([x - w / 2, y + 0.55, z + s * (d / 2)], [x + w / 2, y + 0.55, z + s * (d / 2)], 0.032, 6, shade(P.galv, 0.92), false);
        m.bar(x - w / 2, x + w / 2, y, y + 0.14, z + s * (d / 2) - 0.05, z + s * (d / 2) + 0.05, shade(P.safety, 0.9));
      }
    }
  }

  /// A vertical vessel: rolled shell, dished head, skirt and a nozzle or two.
  /// Smooth by construction, because `column` lofts and `loft` opens a group.
  function vessel(m, x, z, base, h, r, d0, col) {
    const seg = d0.fine ? 14 : d0.seg;
    m.column(x, base, z, h, r, r, seg, col, true);
    if (!d0.fine) return;
    m.column(x, base + h, z, r * 0.42, r, r * 0.62, 14, shade(col, 1.05), true);
    m.column(x, base - 0.5, z, 0.5, r * 0.92, r * 0.92, 14, shade(col, 0.82), false);
    m.beam(x - r - 0.5, x + r + 0.5, base - 0.85, base - 0.5, z - r - 0.5, z + r + 0.5, shade(P.concrete, 0.94), 0.03);
    for (const s of [-1, 1]) {
      m.rod([x + s * r, base + h * 0.24, z], [x + s * (r + 1.1), base + h * 0.24, z], 0.16, 8, shade(P.galv, 1.0), true);
    }
    for (let b = 0; b < 3; b += 1) {
      m.column(x, base + (h * (b + 1)) / 4, z, 0.1, r * 1.04, r * 1.04, 14, shade(col, 0.86), false);
    }
  }

  /// A horizontal drum on two saddles. The saddle is the part that says the
  /// thing is heavy and the shell is the part that says it holds pressure.
  function drum(m, x, z, len, r, d0, col) {
    const seg = d0.fine ? 14 : d0.seg;
    m.rod([x - len / 2, GRADE + r + 1.0, z], [x + len / 2, GRADE + r + 1.0, z], r, seg, col, true);
    if (!d0.fine) return;
    for (const s of [-1, 1]) {
      const sx = x + s * len * 0.3;
      m.beam(sx - 0.45, sx + 0.45, GRADE, GRADE + 1.0, z - r * 0.9, z + r * 0.9, shade(P.concrete, 0.94), 0.03);
      m.webPlate([[sx - 0.5, GRADE + 1.0], [sx + 0.5, GRADE + 1.0], [sx + 0.5, GRADE + 1.12], [sx - 0.5, GRADE + 1.12]],
        z - r * 0.92, z + r * 0.92, shade(P.steel, 0.94));
    }
    m.rod([x, GRADE + r * 2 + 1.0, z], [x, GRADE + r * 2 + 1.9, z], 0.16, 8, shade(P.galv, 1.02), true);
    m.rod([x - len / 2 - 0.6, GRADE + r + 1.0, z], [x - len / 2, GRADE + r + 1.0, z], 0.22, 8, shade(P.galv, 0.95), true);
  }

  /// A covered conveyor on trestles, running between two points. Material
  /// handling is the one thing a bulk facility has that nothing else does, and
  /// a sloped enclosed gallery on legs is unmistakable from any distance.
  function conveyor(m, a, b, wide, d0) {
    const dx = b[0] - a[0], dy = b[1] - a[1], dz = b[2] - a[2];
    const len = Math.hypot(dx, dy, dz);
    if (len < 1e-6) return;
    m.save().move(a[0], a[1], a[2]).rotY(Math.atan2(dx, dz) / DEG)
      .rotX(-Math.asin(Math.max(-1, Math.min(1, dy / len))) / DEG);
    // The gallery: a folded hood over a belt, drawn as its own section rather
    // than as a box, because the roof of a conveyor gallery is pitched and that
    // pitch is most of what is ever seen of it.
    m.bar(-wide / 2, wide / 2, -0.35, 0.35, 0, len, shade(P.galv, 0.94));
    if (d0.fine) {
      m.quad([-wide / 2, 0.35, 0], [0, 0.72, 0], [0, 0.72, len], [-wide / 2, 0.35, len], shade(P.galv, 1.06));
      m.quad([0, 0.72, 0], [wide / 2, 0.35, 0], [wide / 2, 0.35, len], [0, 0.72, len], shade(P.galv, 0.98));
      for (let i = 0; i < 6; i += 1) {
        const t = ((i + 0.5) * len) / 6;
        m.bar(-wide / 2 - 0.06, wide / 2 + 0.06, -0.4, 0.4, t - 0.07, t + 0.07, shade(P.galv, 0.84));
      }
    }
    m.restore();
    // Trestles, plumb, under the run. They are what stops a gallery being a
    // ramp drawn in the air.
    const legs = d0.fine ? 4 : 1;
    for (let i = 1; i <= legs; i += 1) {
      const t = i / (legs + 1);
      const lx = a[0] + dx * t, ly = a[1] + dy * t, lz = a[2] + dz * t;
      for (const s of [-1, 1]) {
        if (d0.fine) m.beam(lx + s * (wide / 2) - 0.14, lx + s * (wide / 2) + 0.14, GRADE, ly - 0.3, lz - 0.16, lz + 0.16, P.steel, 0.016);
        else m.bar(lx + s * (wide / 2) - 0.14, lx + s * (wide / 2) + 0.14, GRADE, ly - 0.3, lz - 0.16, lz + 0.16, P.steel);
      }
      if (d0.fine) {
        m.rod([lx - wide / 2, GRADE + 0.9, lz], [lx + wide / 2, ly - 0.9, lz], 0.05, 6, shade(P.steel, 0.9), false);
        m.beam(lx - wide / 2 - 0.3, lx + wide / 2 + 0.3, GRADE, GRADE + 0.22, lz - 0.4, lz + 0.4, shade(P.concrete, 0.94), 0.025);
      }
    }
  }

  /// Mesh guarding round a machine cell. A robot line is fenced, and the fence
  /// is the single most recognisable thing about one — welded mesh in framed
  /// panels on feet, with a bay left open to get in. Drawn as frame plus a
  /// coarse grid, because a solid panel reads as a wall and a wall is the wrong
  /// object entirely.
  function guardFence(m, x, z, w, d, d0, h0) {
    // Welded mesh is 4 mm wire in a 50 mm grid. There is no map zoom at which
    // any of that resolves, and the posts carrying it are 60 mm square, so the
    // coarse path draws none of it — the cells inside are what reads there.
    if (!d0.fine) return;
    const h = h0 || 2.1, hw = w / 2, hd = d / 2;
    const runs = [[-hw, -hd, hw, -hd], [hw, hd, -hw, hd], [hw, -hd, hw, hd], [-hw, hd, -hw, -hd]];
    for (let r = 0; r < runs.length; r += 1) {
      const a = runs[r];
      const dx = a[2] - a[0], dz = a[3] - a[1], len = Math.hypot(dx, dz);
      const n = Math.max(2, Math.round(len / (d0.fine ? 2.4 : 6)));
      const ux = dx / len, uz = dz / len;
      for (let i = 0; i <= n; i += 1) {
        const px = x + a[0] + ux * ((i * len) / n), pz = z + a[1] + uz * ((i * len) / n);
        m.bar(px - 0.06, px + 0.06, GRADE, GRADE + h, pz - 0.06, pz + 0.06, shade(P.safety, 0.86));
        if (d0.fine) m.bar(px - 0.18, px + 0.18, GRADE, GRADE + 0.06, pz - 0.18, pz + 0.18, shade(P.steel, 0.92));
      }
      if (!d0.fine) continue;
      for (const y of [GRADE + 0.35, GRADE + h - 0.12]) {
        m.bar(x + Math.min(a[0], a[2]) - 0.04, x + Math.max(a[0], a[2]) + 0.04, y, y + 0.08,
          z + Math.min(a[1], a[3]) - 0.04, z + Math.max(a[1], a[3]) + 0.04, shade(P.galv, 0.92));
      }
      // The front run is left as rails only: that is the way in, and a cell you
      // cannot get into is a cell nobody can load.
      if (r === 0) continue;
      for (let i = 0; i < n * 3; i += 1) {
        const t = ((i + 0.5) * len) / (n * 3);
        const px = x + a[0] + ux * t, pz = z + a[1] + uz * t;
        m.bar(px - 0.022, px + 0.022, GRADE + 0.35, GRADE + h - 0.12, pz - 0.022, pz + 0.022, shade(P.galv, 0.98));
      }
    }
  }

  /// A row of control cabinets against a wall, with a cable tray over them.
  /// This is what "controls" physically IS. The roadmap is explicit that
  /// invisible software must not be handed a giant external box, so the entire
  /// control content of a retrofit is this: a metre of cabinet per panel and
  /// the tray that feeds it.
  function cabinets(m, x, z, n, d0, col) {
    if (!d0.fine) {
      m.bar(x, x + n * 0.94, GRADE + 0.12, GRADE + 2.2, z - 0.42, z + 0.42, shade(col || P.clad, 0.98));
      return;
    }
    for (let i = 0; i < n; i += 1) {
      const cx = x + i * 0.94;
      m.bar(cx, cx + 0.9, GRADE + 0.12, GRADE + 2.2, z - 0.42, z + 0.42, shade(col || P.clad, 0.96 + 0.05 * (i % 2)));
      if (!d0.fine) continue;
      m.beam(cx - 0.03, cx + 0.93, GRADE, GRADE + 0.12, z - 0.46, z + 0.46, shade(P.steel, 0.9), 0.015);
      m.beam(cx - 0.03, cx + 0.93, GRADE + 2.2, GRADE + 2.3, z - 0.46, z + 0.46, shade(col || P.clad, 1.08), 0.015);
      m.bar(cx + 0.08, cx + 0.82, GRADE + 1.35, GRADE + 1.85, z + 0.42, z + 0.45, P.dark);
      m.rod([cx + 0.78, GRADE + 1.1, z + 0.44], [cx + 0.78, GRADE + 1.1, z + 0.56], 0.035, 6, shade(P.galv, 1.05), true);
      for (let l = 0; l < 3; l += 1) {
        m.bar(cx + 0.14 + l * 0.2, cx + 0.24 + l * 0.2, GRADE + 1.95, GRADE + 2.05, z + 0.42, z + 0.44,
          shade(l === 1 ? P.safety : P.lane, 1.1));
      }
    }
    if (!d0.fine) return;
    m.bar(x - 0.2, x + n * 0.94, GRADE + 2.6, GRADE + 2.72, z - 0.3, z + 0.3, shade(P.galv, 0.9));
    for (let i = 0; i < n * 4; i += 1) {
      m.bar(x + i * 0.24, x + i * 0.24 + 0.08, GRADE + 2.72, GRADE + 2.78, z - 0.3, z + 0.3, shade(P.galv, 1.04));
    }
    for (let i = 0; i <= n; i += 2) {
      m.rod([x + i * 0.94, GRADE + 2.72, z], [x + i * 0.94, GRADE + 3.4, z], 0.045, 6, P.galv, false);
    }
  }

  /// A lattice suspension tower, off the pad and marching away. The one thing
  /// that says "grid" from further off than any building on this list.
  function pylonTower(m, x, z, h, d0) {
    const base = 3.2, top = 0.9;
    if (!d0.fine) {
      m.bar(x - 0.5, x + 0.5, GRADE, GRADE + h, z - 0.5, z + 0.5, P.galv);
      for (let a = 0; a < 2; a += 1) {
        m.bar(x - 5.5, x + 5.5, GRADE + h - 3.5 - a * 4.2, GRADE + h - 3.1 - a * 4.2, z - 0.35, z + 0.35, P.galv);
      }
      return;
    }
    const bays = 6;
    for (let i = 0; i < bays; i += 1) {
      const t0 = i / bays, t1 = (i + 1) / bays;
      const r0 = base + (top - base) * t0, r1 = base + (top - base) * t1;
      const y0 = GRADE + h * t0, y1 = GRADE + h * t1;
      for (const sx of [-1, 1]) {
        for (const sz of [-1, 1]) {
          m.rod([x + sx * r0 * 0.5, y0, z + sz * r0 * 0.5], [x + sx * r1 * 0.5, y1, z + sz * r1 * 0.5], 0.075, 5, P.galv, false);
        }
      }
      m.save().move(x, 0, z);
      latticeBay(m, y0, y1, r1 * 0.5, r1 * 0.5, 0.045, 5, shade(P.galv, 0.92));
      m.restore();
      if (i) continue;
      for (const sx of [-1, 1]) {
        for (const sz of [-1, 1]) {
          m.beam(x + sx * r0 * 0.5 - 0.6, x + sx * r0 * 0.5 + 0.6, GRADE - 0.02, GRADE + 0.5,
            z + sz * r0 * 0.5 - 0.6, z + sz * r0 * 0.5 + 0.6, shade(P.concrete, 0.94), 0.03);
        }
      }
    }
    // Two crossarms and the insulator strings hanging off them. The discs are
    // what say the tower is live rather than a radio mast.
    for (let a = 0; a < 2; a += 1) {
      const ay = GRADE + h - 3.4 - a * 4.2;
      m.rod([x - 6.0, ay, z], [x + 6.0, ay, z], 0.11, 6, P.galv, false);
      m.rod([x - 5.6, ay + 1.5, z], [x + 5.6, ay + 1.5, z], 0.07, 6, shade(P.galv, 1.04), false);
      for (const sx of [-1, 1]) {
        m.rod([x + sx * 5.8, ay, z], [x + sx * 1.2, ay + 1.5, z], 0.055, 5, shade(P.galv, 0.9), false);
        for (let d = 0; d < 5; d += 1) {
          m.column(x + sx * 5.4, ay - 0.3 - d * 0.26, z, 0.17, 0.2, 0.2, 8, shade(P.hardcore, 1.02 - 0.03 * d), false);
        }
      }
    }
    m.rod([x, GRADE + h, z], [x, GRADE + h + 1.6, z], 0.06, 5, shade(P.galv, 1.06), false);
  }

  /// A block on the same stage clock as the lead one: nothing before the frame
  /// goes up, primed steel at three, part-sheeted at four, finished at five.
  /// Every kind that is more than one building uses this, so no composition can
  /// invent its own idea of what half built looks like.
  function stagedBlock(m, o, stage, d0) {
    if (stage < 3) return;
    hall(m, Object.assign({}, o, { clad: CLAD_BY_STAGE[stage - 1] }), d0);
  }

  /// A flat-roofed framed block with a parapet, floor bands and strip glazing:
  /// the shape an office, a laboratory or a control building actually is, and
  /// the one thing a portal-framed shed cannot pretend to be. Same stage clock
  /// as `stagedBlock` — frame and floor slabs at three, envelope from four.
  function deckedBlock(m, o, stage, d0) {
    if (stage < 3) return;
    const b = o.block, hw = b.w / 2, hd = b.dz / 2;
    const floors = b.floors || 2;
    const fh = b.eaves / floors;
    const clad = CLAD_BY_STAGE[stage - 1];
    const steel = clad > 0.9 ? shade(o.col, 0.7) : P.primer;
    const tag = o.tag ? ` (${o.tag})` : "";
    m.part(`structure / framed floors and columns${tag}`, () => {
      m.save().move(o.x, o.deck, o.z);
      const cols = d0.fine ? 5 : 3, rows = d0.fine ? 3 : 2;
      for (let i = 0; i < cols; i += 1) {
        for (let j = 0; j < rows; j += 1) {
          const cx = -hw + 0.6 + (i * (b.w - 1.2)) / (cols - 1), cz = -hd + 0.6 + (j * (b.dz - 1.2)) / (rows - 1);
          if (d0.fine) m.beam(cx - 0.22, cx + 0.22, 0, b.eaves, cz - 0.22, cz + 0.22, steel, 0.018);
          else m.bar(cx - 0.22, cx + 0.22, 0, b.eaves, cz - 0.22, cz + 0.22, steel);
        }
      }
      for (let f = 1; f <= floors; f += 1) {
        const y = fh * f;
        // The slab edge. On a framed building it is the one horizontal line
        // that survives the cladding, and it is what makes a block read as
        // storeys rather than as a tall box.
        m.bar(-hw, hw, y - 0.34, y, -hd, hd, shade(P.concrete, 0.9));
        if (!d0.fine) continue;
        for (let j = 0; j < rows; j += 1) {
          const cz = -hd + 0.6 + (j * (b.dz - 1.2)) / (rows - 1);
          m.beam(-hw + 0.4, hw - 0.4, y - 0.78, y - 0.34, cz - 0.16, cz + 0.16, steel, 0.016);
        }
      }
      m.restore();
    });
    if (clad <= 0) return;
    m.part(`envelope / spandrel panels and strip glazing${tag}`, () => {
      m.save().move(o.x, o.deck, o.z);
      const done = clamp01(clad);
      const x1 = -hw + b.w * done;
      for (const s of [-1, 1]) {
        for (let f = 0; f < floors; f += 1) {
          const y0 = fh * f, sill = y0 + fh * 0.36, head = y0 + fh * 0.82;
          if (x1 <= -hw + 0.4) continue;
          m.bar(-hw, x1, y0, sill, s * hd - 0.06, s * hd + 0.06, shade(o.col, 1.0 + 0.04 * f));
          m.bar(-hw, x1, head, y0 + fh, s * hd - 0.06, s * hd + 0.06, shade(o.col, 0.92));
          m.bar(-hw + 0.3, x1 - 0.3, sill, head, s * hd - 0.04, s * hd + 0.04, P.glass);
          if (!d0.fine) continue;
          // Mullions and a sill with a drip. Strip glazing without a grid is a
          // blue stripe painted on a wall, and the grid is the only thing that
          // gives a glazed elevation a scale.
          const n = Math.max(2, Math.round((x1 + hw) / 2.4));
          for (let i = 0; i <= n; i += 1) {
            const gx = -hw + 0.3 + (i * (x1 - 0.6 + hw)) / n;
            m.beam(gx - 0.05, gx + 0.05, sill, head, s * hd - 0.09, s * hd + 0.09, shade(P.galv, 1.02), 0.012);
          }
          m.beam(-hw, x1, sill - 0.12, sill, s * hd - 0.14, s * hd + 0.14, shade(P.galv, 0.86), 0.015);
        }
      }
      // The two ends stay solid: a laboratory or an office block is glazed on
      // its long faces and blank on its short ones, where the stairs and the
      // risers are.
      for (const s of [-1, 1]) {
        if (s > 0 && x1 < hw - 0.2) continue;
        m.bar(s * hw - 0.09, s * hw + 0.09, 0, b.eaves, -hd, hd, shade(o.col, 0.94));
      }
      m.restore();
    });
    m.part(`envelope / roof deck and parapet${tag}`, () => {
      m.save().move(o.x, o.deck, o.z);
      deck(m, -hw, hw, b.eaves, -hd, hd, shade(P.roof, 0.98));
      for (const s of [-1, 1]) {
        m.bar(-hw - 0.12, hw + 0.12, b.eaves, b.eaves + 0.75, s * hd - 0.12, s * hd + 0.12, shade(o.col, 1.05));
        m.bar(s * hw - 0.12, s * hw + 0.12, b.eaves, b.eaves + 0.75, -hd - 0.12, hd + 0.12, shade(o.col, 1.02));
      }
      if (!d0.fine) { m.restore(); return; }
      for (const s of [-1, 1]) {
        m.beam(-hw - 0.18, hw + 0.18, b.eaves + 0.75, b.eaves + 0.87, s * hd - 0.18, s * hd + 0.18, shade(P.galv, 1.06), 0.018);
        m.beam(s * hw - 0.18, s * hw + 0.18, b.eaves + 0.75, b.eaves + 0.87, -hd - 0.18, hd + 0.18, shade(P.galv, 1.02), 0.018);
      }
      // Falls to a rainwater outlet, and the outlet. A flat roof is not flat,
      // and the one place that shows is the sump at the low corner.
      m.bar(-hw + 1.0, -hw + 2.4, b.eaves - 0.06, b.eaves + 0.02, -hd + 1.0, -hd + 2.4, shade(P.dark, 1.15));
      m.restore();
      m.column(o.x - hw + 1.7, GRADE, o.z - hd + 1.7, o.deck - GRADE + b.eaves, 0.13, 0.13, d0.seg, P.galv, false);
    });
  }

  // ------------------------------------------------------------ compositions
  // One function per project kind. What each of them IS comes from roadmap
  // section E and nowhere else, and what each of them is NOT is in the
  // description `build` returns: these are representative generic facilities
  // and none of them claims a capacity, a throughput or a real company's plant.
  //
  // Every one of them takes the same context and reads `stage` for what is
  // standing. There is no clock here either — a composition cannot know how
  // long anything has taken, only how much work the server has recorded.
  //
  // THE SHARED STAGE KIT STAYS SHARED (roadmap section E asks for exactly
  // that): platform, dig, footings, slab, hoarding, huts, materials, crane,
  // scaffold and lighting are the same objects on all thirteen sites, because
  // they are the same objects on all thirteen real ones. What differs is the
  // permanent work, and that is what these functions are.

  /// Road, bridge and utility corridor works. The only kind on this list whose
  /// subject is not a building: the permanent work is a length of carriageway,
  /// a structure carrying it over something, and a duct route beside it. The
  /// depot shed is the smallest object here rather than the largest, which is
  /// the right way round for a corridor job.
  function worksInfrastructure(m, k, c) {
    const { o, W, D, d0, stage } = c;
    const built = stage >= 4, done = stage >= 5;
    const rz = -D * 0.16, half = 6.0;                 // corridor centre line and half width
    const bx = W * 0.22;                              // where the structure crosses it
    if (d0.fine && stage >= 3) genericBlock(m, k, o, d0);
    // The carriageway, in the layers it is actually built in. Capping at the
    // start, subbase, base, binder and surface: each stage lays one and each
    // one is 60-80 mm proud of the last, so the edge of the road reads as a
    // build-up rather than as a painted stripe.
    m.part("corridor / carriageway formation", () => {
      const layers = [
        [P.hardcore, half + 1.6, 0.10],
        [shade(P.hardcore, 0.88), half + 1.2, 0.18],
        [shade(P.asphalt, 1.25), half + 0.8, 0.26],
        [shade(P.asphalt, 1.05), half + 0.4, 0.34],
        [P.asphalt, half, 0.42],
      ];
      m.save().move(0, GRADE, 0);
      for (let i = 0; i < Math.min(stage + 1, layers.length); i += 1) {
        const l = layers[i];
        m.plate([[-W * 0.46, rz - l[1]], [W * 0.46, rz - l[1]], [W * 0.46, rz + l[1]], [-W * 0.46, rz + l[1]]], l[2], l[0]);
      }
      m.restore();
      if (!d0.fine) return;
      groundPatch(m, 0, rz, W * 0.9, half * 2 - 0.4, 12, 4, GRADE + 0.425, P.asphalt, 0.05);
      if (stage >= 3) {
        for (const s of [-1, 1]) {
          m.beam(-W * 0.46, W * 0.46, GRADE + 0.42, GRADE + 0.66, rz + s * (half + 0.2) - 0.14, rz + s * (half + 0.2) + 0.14, P.kerb, 0.022);
        }
      }
      if (!done) return;
      // Lane markings, gullies and a footway behind the kerb. A road with no
      // drainage is a picture of a road.
      for (let i = 0; i < 9; i += 1) {
        const lx = -W * 0.40 + (i * W * 0.80) / 8;
        m.bar(lx - 1.5, lx + 1.5, GRADE + 0.425, GRADE + 0.45, rz - 0.1, rz + 0.1, P.lane);
      }
      for (let i = 0; i < 4; i += 1) {
        const gx = -W * 0.34 + (i * W * 0.68) / 3;
        m.bar(gx - 0.34, gx + 0.34, GRADE + 0.4, GRADE + 0.44, rz - half - 0.05, rz - half + 0.6, shade(P.dark, 1.2));
      }
      m.save().move(0, GRADE + 0.42, 0);
      m.plate([[-W * 0.46, rz + half + 0.34], [W * 0.46, rz + half + 0.34], [W * 0.46, rz + half + 2.4], [-W * 0.46, rz + half + 2.4]], 0.1, P.kerb);
      m.restore();
    });
    // The duct route. Open with a bedded duct bank while the job is live,
    // backfilled with chamber covers once it is not: a utility corridor is the
    // one kind whose permanent work ends up under the ground.
    // TWO OBJECTS, NOT ONE STATE FLAG. An open trench with a duct bank bedded
    // in it and a reinstated route with chamber covers on it are different
    // things, and naming them the same made the corridor's own work look
    // identical at the frame and commissioning stages to anything reading the
    // part list — which is exactly what a picker and the checks do.
    m.part(built ? "corridor / duct route reinstated" : "corridor / utility duct trench", () => {
      const tz = rz + half + 3.6;
      if (!built) {
        m.bar(-W * 0.42, W * 0.42, PIT, GRADE, tz - 1.1, tz + 1.1, P.earthCut);
        for (let i = 0; i < (d0.fine ? 4 : 2); i += 1) {
          const dy = PIT + 0.28 + i * 0.34;
          for (let j = 0; j < (d0.fine ? 3 : 1); j += 1) {
            m.rod([-W * 0.42, dy, tz - 0.5 + j * 0.5], [W * 0.42, dy, tz - 0.5 + j * 0.5], 0.13, d0.fine ? 8 : 4,
              shade(j === 1 ? P.safety : P.hoard, 0.94 + 0.06 * i), false);
          }
        }
        if (d0.fine) {
          for (let i = 0; i < 6; i += 1) {
            const sx = -W * 0.36 + (i * W * 0.72) / 5;
            m.bar(sx - 0.1, sx + 0.1, PIT, GRADE + 0.4, tz - 1.15, tz + 1.15, P.timber);
          }
        }
      } else {
        m.bar(-W * 0.42, W * 0.42, GRADE - 0.1, GRADE + 0.06, tz - 1.1, tz + 1.1, shade(P.hardcore, 0.96));
        for (let i = 0; i < (d0.fine ? 4 : 2); i += 1) {
          const cx = -W * 0.34 + (i * W * 0.68) / 3;
          // A 1.4 m chamber cover has an arris worth drawing at inspection
          // range and nothing worth drawing at map range.
          if (d0.fine) {
            m.beam(cx - 0.7, cx + 0.7, GRADE + 0.06, GRADE + 0.14, tz - 0.7, tz + 0.7, shade(P.steel, 0.96), 0.02);
            m.bar(cx - 0.62, cx + 0.62, GRADE + 0.14, GRADE + 0.17, tz - 0.62, tz + 0.62, shade(P.dark, 1.1));
          } else {
            m.bar(cx - 0.7, cx + 0.7, GRADE + 0.06, GRADE + 0.14, tz - 0.7, tz + 0.7, shade(P.steel, 0.96));
          }
        }
      }
    });
    if (stage >= 2) {
      // Abutments each side of the carriageway and a pier between them. At the
      // foundation stage they are bases only; the stems come with the frame.
      m.part("structure / abutments and pier", () => {
        for (const s of [-1, 1]) {
          const az = rz + s * (half + 5.5);
          m.bar(bx - 5.5, bx + 5.5, PIT, GRADE + (stage >= 3 ? 5.4 : 0.9), az - 1.6, az + 1.6, shade(P.concrete, 0.94));
          if (stage >= 3 && d0.fine) {
            // Wing walls and a bearing shelf. The shelf is what a beam lands on
            // and the wing walls are what hold the approach embankment back.
            for (const w of [-1, 1]) {
              m.webPlate([[bx + w * 5.5, GRADE], [bx + w * 8.4, GRADE], [bx + w * 5.5, GRADE + 5.4]],
                az - 1.0, az + 1.0, shade(P.concrete, 0.9));
            }
            m.beam(bx - 5.7, bx + 5.7, GRADE + 5.4, GRADE + 5.7, az - 1.8, az + 1.8, P.kerb, 0.03);
            for (const b of [-3.2, 0, 3.2]) {
              m.bar(bx + b - 0.42, bx + b + 0.42, GRADE + 5.7, GRADE + 6.0, az - 0.5, az + 0.5, shade(P.dark, 1.15));
            }
          }
        }
        if (stage >= 3) {
          m.bar(bx - 3.4, bx + 3.4, PIT, GRADE + 0.7, rz - 1.4, rz + 1.4, shade(P.concrete, 0.92));
          m.column(bx, GRADE + 0.7, rz, 5.0, 1.15, 1.0, d0.fine ? 14 : d0.seg, shade(P.concrete, 0.98), true);
          if (d0.fine) m.beam(bx - 3.6, bx + 3.6, GRADE + 5.7, GRADE + 6.0, rz - 1.0, rz + 1.0, P.kerb, 0.03);
        } else if (d0.fine) {
          rebarCage(m, bx, rz, 1.5, 0.9, PIT + 0.1, GRADE + 1.8, 6);
        }
      });
    }
    if (stage >= 3) {
      // Precast beams, landed. Two of them at the frame stage and the full set
      // once the deck is on, so the span is visibly incomplete while it is.
      m.part("structure / deck beams", () => {
        const beams = stage >= 4 ? 5 : 2;
        for (let i = 0; i < beams; i += 1) {
          const px = bx - 4.0 + (i * 8.0) / 4;
          if (d0.fine) m.beam(px - 0.5, px + 0.5, GRADE + 6.0, GRADE + 7.1, rz - half - 6.2, rz + half + 6.2, shade(P.concrete, 0.96), 0.03);
          else m.bar(px - 0.5, px + 0.5, GRADE + 6.0, GRADE + 7.1, rz - half - 6.2, rz + half + 6.2, shade(P.concrete, 0.96));
        }
      });
    }
    if (stage >= 4) {
      // The deck slab is its own operation and its own part: the beams are
      // landed by crane in a morning and the slab is poured on them weeks
      // later, with the approach embankments built up to meet it.
      m.part("structure / deck slab and approaches", () => {
        {
          m.bar(bx - 5.0, bx + 5.0, GRADE + 7.1, GRADE + 7.35, rz - half - 6.4, rz + half + 6.4, shade(P.concrete, 1.02));
          if (d0.fine) {
            for (const s of [-1, 1]) {
              m.beam(bx + s * 5.0 - 0.34, bx + s * 5.0 + 0.02, GRADE + 6.7, GRADE + 7.4, rz - half - 6.4, rz + half + 6.4, shade(P.concrete, 0.9), 0.025);
            }
            // The approach embankment either end, so the deck goes somewhere.
            for (const s of [-1, 1]) {
              m.quad([bx - 5.0, GRADE + 7.35, rz + s * (half + 6.4)], [bx + 5.0, GRADE + 7.35, rz + s * (half + 6.4)],
                [bx + 6.6, GRADE, rz + s * (half + 9.0)], [bx - 6.6, GRADE, rz + s * (half + 9.0)], shade(P.asphalt, 1.1));
            }
          }
        }
      });
    }
    if (done) {
      m.part("corridor / parapets, barrier and sign gantry", () => {
        for (const s of [-1, 1]) {
          if (d0.fine) m.beam(bx + s * 5.05 - 0.16, bx + s * 5.05 + 0.16, GRADE + 7.35, GRADE + 8.55, rz - half - 6.4, rz + half + 6.4, P.kerb, 0.025);
          else m.bar(bx + s * 5.05 - 0.16, bx + s * 5.05 + 0.16, GRADE + 7.35, GRADE + 8.55, rz - half - 6.4, rz + half + 6.4, P.kerb);
          if (!d0.fine) continue;
          for (let i = 0; i < 8; i += 1) {
            const pz = rz - half - 5.6 + (i * (half * 2 + 11.2)) / 7;
            m.bar(bx + s * 5.05 - 0.2, bx + s * 5.05 + 0.2, GRADE + 8.55, GRADE + 9.6, pz - 0.09, pz + 0.09, P.galv);
          }
          m.rod([bx + s * 5.05, GRADE + 9.6, rz - half - 6.0], [bx + s * 5.05, GRADE + 9.6, rz + half + 6.0], 0.06, 6, shade(P.galv, 1.05), false);
        }
        // Vehicle restraint along the near verge, and a sign gantry over the
        // road. Both are what a finished corridor has and a half-built one
        // cannot: they go on last and they are what says the road is open.
        // Barrier posts are 90 mm and 3 m apart: at map range the rail above
        // them is the whole of what a safety barrier looks like.
        for (let i = 0; d0.fine && i < 10; i += 1) {
          const px = -W * 0.42 + (i * W * 0.84) / 9;
          m.bar(px - 0.09, px + 0.09, GRADE + 0.42, GRADE + 1.15, rz - half - 1.0, rz - half - 0.85, P.galv);
        }
        m.bar(-W * 0.43, W * 0.43, GRADE + 0.95, GRADE + 1.28, rz - half - 1.05, rz - half - 0.85, shade(P.galv, 1.06));
        const gx = -W * 0.22;
        for (const s of [-1, 1]) {
          if (d0.fine) m.beam(gx - 0.26, gx + 0.26, GRADE + 0.42, GRADE + 6.6, rz + s * (half + 1.4) - 0.26, rz + s * (half + 1.4) + 0.26, P.galv, 0.02);
          else m.bar(gx - 0.26, gx + 0.26, GRADE + 0.42, GRADE + 6.6, rz + s * (half + 1.4) - 0.26, rz + s * (half + 1.4) + 0.26, P.galv);
        }
        m.bar(gx - 0.3, gx + 0.3, GRADE + 6.6, GRADE + 7.0, rz - half - 1.6, rz + half + 1.6, P.galv);
        m.bar(gx - 0.06, gx + 0.06, GRADE + 4.5, GRADE + 6.6, rz - 3.6, rz + 3.6, shade(P.hoard, 1.05));
        if (d0.fine) {
          m.beam(gx - 0.2, gx + 0.2, GRADE + 4.35, GRADE + 6.75, rz - 3.8, rz + 3.8, shade(P.galv, 0.94), 0.02);
          for (const s of [-1, 1]) {
            m.rod([gx, GRADE + 6.4, rz + s * (half + 1.2)], [gx, GRADE + 5.0, rz + s * 3.6], 0.05, 6, P.galv, false);
          }
        }
      });
    }
    // The stockpile: culvert rings, kerb packs and a headwall unit. Precast is
    // what a corridor job keeps in the open, and the rings are round.
    m.part("yard / precast and culvert stockpile", () => {
      const sx = W * 0.34, sz = D * 0.26;
      for (let i = 0; i < (d0.fine ? 4 : 2); i += 1) {
        const cx = sx - 2.4 + (i % 2) * 3.4, cz = sz + Math.floor(i / 2) * 3.0;
        m.rod([cx, GRADE + 1.2, cz - 0.9], [cx, GRADE + 1.2, cz + 0.9], 1.2, d0.fine ? 12 : d0.seg, shade(P.concrete, 0.94 + 0.05 * i), false);
        if (!d0.fine) continue;
        m.rod([cx, GRADE + 1.2, cz - 0.92], [cx, GRADE + 1.2, cz + 0.92], 0.95, 12, shade(P.dark, 0.9), false);
        for (const cs of [-1, 1]) {
          m.bar(cx + cs * 1.25 - 0.16, cx + cs * 1.25 + 0.16, GRADE, GRADE + 0.34, cz - 0.9, cz + 0.9, P.timber);
        }
      }
      if (!d0.fine) return;
      for (let i = 0; i < 3; i += 1) {
        m.beam(sx - 4.6, sx - 1.4, GRADE + i * 0.34, GRADE + 0.28 + i * 0.34, sz - 4.6 + i * 0.1, sz - 3.4 + i * 0.1, P.kerb, 0.02);
      }
      m.webPlate([[sx + 2.6, GRADE], [sx + 6.2, GRADE], [sx + 6.2, GRADE + 2.2], [sx + 4.4, GRADE + 3.0], [sx + 2.6, GRADE + 2.2]],
        sz - 5.4, sz - 4.9, shade(P.concrete, 0.98));
    });
    if (stage >= 3 && d0.fine) propKiosk(m, k, YARD.x, YARD.z, W, D, d0, built);
  }

  /// Modular factory and service yard. The composition IS the modularity: three
  /// identical production modules on one grid with movement joints between
  /// them, linked at high level, with the service yard behind. The third module
  /// runs a stage behind the other two, which is what a modular building is FOR
  /// and the only honest way to draw one being built.
  function worksCivilianIndustry(m, k, c) {
    const { o, W, D, d0, stage } = c;
    const built = stage >= 4, done = stage >= 5;
    // Three modules of 16 m on 17.4 m centres span 50 m, which is what fits
    // between the perimeters of the smallest compound this kind is built on.
    // A module wide enough to look impressive on its own is a module that puts
    // the third one through the fence.
    const b = k.block, gap = 1.4;
    const mods = [1, 2];
    if (d0.fine) {
      if (stage >= 3) genericBlock(m, k, o, d0);
      for (const i of mods) {
        // The third module runs one stage behind the other two. That is what a
        // modular building is FOR — the next module goes up beside the working
        // ones — and it is the only honest way to draw one being built.
        stagedBlock(m, {
          x: o.x + i * (b.w + gap), z: o.z, block: b, col: shade(k.body, 1 - i * 0.03),
          deck: GRADE + 0.18, tag: `module ${i + 1}`,
        }, i === 2 ? stage - 1 : stage, d0);
      }
    } else if (stage >= 4) {
      m.part("structure / repeat modules", () => {
        for (const i of mods) {
          const mx = o.x + i * (b.w + gap);
          if (i === 2 && stage < 5) continue;
          m.bar(mx - b.w / 2, mx + b.w / 2, GRADE, GRADE + b.eaves, o.z - b.dz / 2, o.z + b.dz / 2, shade(k.body, done ? 0.97 : 0.88));
        }
      });
    }
    if (stage >= 3) {
      // The module bases the frames stand on, and the movement joint between
      // them. A modular building is a row of separate structures that touch,
      // and the joint is the only thing that says so from outside.
      m.part("structure / module bases and movement joints", () => {
        for (const i of mods) {
          const mx = o.x + i * (b.w + gap);
          m.bar(mx - b.w / 2 - 1.1, mx + b.w / 2 + 1.1, PIT, GRADE + 0.18, o.z - b.dz / 2 - 1.1, o.z + b.dz / 2 + 1.1, P.concrete);
        }
        if (!d0.fine) return;
        for (let i = 0; i < 3; i += 1) {
          const jx = o.x + b.w / 2 + gap / 2 + i * (b.w + gap);
          if (i > 1) continue;
          m.bar(jx - 0.34, jx + 0.34, GRADE + 0.18, GRADE + 0.26, o.z - b.dz / 2, o.z + b.dz / 2, shade(P.dark, 1.1));
          m.bar(jx - 0.5, jx + 0.5, GRADE + b.eaves - 0.3, GRADE + b.eaves + 0.2, o.z - b.dz / 2, o.z + b.dz / 2, shade(P.galv, 0.92));
        }
      });
    }
    if (stage >= 4) {
      // Link bridges at high level between modules. A modular factory moves
      // work between its modules and the bridge is where that happens.
      m.part("structure / high-level module links", () => {
        for (let i = 0; i < 2; i += 1) {
          const jx = o.x + b.w / 2 + gap / 2 + i * (b.w + gap);
          const y = GRADE + b.eaves * 0.62;
          m.bar(jx - (b.w / 2 + gap / 2), jx + (b.w / 2 + gap / 2), y, y + 2.6, o.z + b.dz / 2 + 1.4, o.z + b.dz / 2 + 4.2, shade(P.clad, 1.02));
          if (!d0.fine) continue;
          m.bar(jx - (b.w / 2 + gap / 2) + 0.6, jx + (b.w / 2 + gap / 2) - 0.6, y + 1.1, y + 2.1, o.z + b.dz / 2 + 4.2, o.z + b.dz / 2 + 4.24, P.glass);
          m.beam(jx - (b.w / 2 + gap / 2) - 0.2, jx + (b.w / 2 + gap / 2) + 0.2, y + 2.6, y + 2.8, o.z + b.dz / 2 + 1.2, o.z + b.dz / 2 + 4.4, P.roof, 0.02);
          for (const s of [-1, 1]) {
            m.beam(jx + s * (b.w / 2 + gap / 2) - 0.2, jx + s * (b.w / 2 + gap / 2) + 0.2, GRADE, y, o.z + b.dz / 2 + 2.6, o.z + b.dz / 2 + 3.0, P.steel, 0.018);
          }
        }
      });
    }
    // The service yard: a low service block, an external plant compound and the
    // pair of tanks a factory keeps outside its own envelope.
    if (stage >= 3) {
      m.part("structure / service block", () => {
        const sx = o.x + b.w * 1.1, sz = o.z + b.dz / 2 + 7.6;
        m.bar(sx - 8, sx + 8, GRADE, GRADE + 4.2, sz - 4, sz + 4, shade(k.body, 0.93));
        if (d0.fine) {
          m.beam(sx - 8.3, sx + 8.3, GRADE + 4.2, GRADE + 4.5, sz - 4.3, sz + 4.3, P.roof, 0.025);
          m.bar(sx - 1.2, sx + 1.2, GRADE + 0.05, GRADE + 2.4, sz + 4.0, sz + 4.04, P.door);
          for (let i = 0; i < 4; i += 1) {
            m.bar(sx - 6.6 + i * 3.4, sx - 4.8 + i * 3.4, GRADE + 2.5, GRADE + 3.6, sz + 4.0, sz + 4.04, P.glass);
            m.beam(sx - 6.72 + i * 3.4, sx - 4.68 + i * 3.4, GRADE + 2.4, GRADE + 3.7, sz + 3.98, sz + 4.1, shade(P.galv, 1.0), 0.015);
          }
          louvreBank(m, sx + 4.4, sx + 7.2, GRADE + 1.4, GRADE + 3.2, sz + 4.0, 0.14, 4, shade(P.galv, 0.96));
        } else {
          m.bar(sx - 8.3, sx + 8.3, GRADE + 4.2, GRADE + 4.45, sz - 4.3, sz + 4.3, P.roof);
        }
      });
    }
    if (stage >= 4) {
      m.part("services / external plant compound", () => {
        const px = o.x + b.w * 2.2, pz = o.z + b.dz / 2 + 7.4;
        m.bar(px - 6, px + 6, GRADE, GRADE + 0.3, pz - 4, pz + 4, shade(P.concrete, 0.94));
        for (let i = 0; i < (d0.fine ? 3 : 1); i += 1) {
          const cx = px - 4 + i * 4;
          m.bar(cx - 1.5, cx + 1.5, GRADE + 0.3, GRADE + 2.6, pz - 1.6, pz + 1.6, shade(P.galv, 0.96 + 0.04 * i));
          if (!d0.fine) continue;
          // Fan cowls on top and a coil face on the side: a chiller is a box
          // that moves air and both of those are what say it does.
          m.column(cx, GRADE + 2.6, pz, 0.34, 0.72, 0.72, 12, shade(P.dark, 1.2), true);
          for (let f = 0; f < 5; f += 1) {
            m.bar(cx - 1.42, cx + 1.42, GRADE + 0.6 + f * 0.36, GRADE + 0.84 + f * 0.36, pz + 1.6, pz + 1.72, shade(P.steel, 0.9 + 0.06 * (f % 2)));
          }
        }
        if (!d0.fine) return;
        // A kerb and bollards, not a palisade. The compound already has a
        // perimeter; a second fence inside it is 1,300 triangles of line that
        // says nothing the first one did not, and an external plant slab is
        // actually protected by exactly this.
        for (const s of [-1, 1]) {
          m.beam(px - 6.4, px + 6.4, GRADE + 0.3, GRADE + 0.52, pz + s * 4.2 - 0.16, pz + s * 4.2 + 0.16, P.kerb, 0.02);
        }
        for (let i = 0; i < 5; i += 1) {
          bollard(m, px - 5.6 + i * 2.8, pz - 4.8, 0.95, 0.16, 8, P.safety);
        }
      });
      propTanks(m, k, YARD.x, YARD.z, W, D, d0, built);
    }
  }

  /// Substation, pylons and feeder equipment. Almost none of this kind is a
  /// building: it is a stone-surfaced yard with steel in it, and the control
  /// building is the smallest object on the pad. The staging follows how a
  /// switchyard is actually built — earth grid first, then plinths and the
  /// cable trench, then the steel, then the equipment, and the conductors last.
  function worksPowerGrid(m, k, c) {
    const { o, W, D, d0, stage } = c;
    const built = stage >= 4, done = stage >= 5;
    // The switchyard takes the MIDDLE of the pad and the control building sits
    // on its edge, which is the way round a substation is actually laid out —
    // and the only way a 34 m yard fits inside a 66 m compound whose control
    // building is seated hard against one side.
    const yx = -W * 0.02, yz = o.z - 2;
    if (d0.fine && stage >= 3) genericBlock(m, k, o, d0);
    m.part("yard / earth grid and stone surfacing", () => {
      // A switchyard stands on stone over a buried earth grid. Before the stone
      // goes down the grid is a set of open trenches with bare conductor in
      // them, and that is the whole of what a substation site looks like at
      // stage one.
      if (stage <= 2) {
        for (let i = 0; i < (d0.fine ? 4 : 2); i += 1) {
          const gz = yz - 9 + (i * 18) / 3;
          m.bar(yx - 16, yx + 16, PIT, GRADE, gz - 0.45, gz + 0.45, P.earthCut);
          if (d0.fine) m.rod([yx - 15.5, PIT + 0.12, gz], [yx + 15.5, PIT + 0.12, gz], 0.06, 6, shade(P.primer, 1.1), false);
        }
        for (let i = 0; i < (d0.fine ? 3 : 1); i += 1) {
          const gx = yx - 12 + i * 12;
          m.bar(gx - 0.45, gx + 0.45, PIT, GRADE, yz - 9.5, yz + 9.5, P.earthCut);
        }
      } else {
        m.save().move(0, GRADE, 0);
        m.plate([[yx - 17, yz - 11], [yx + 17, yz - 11], [yx + 17, yz + 11], [yx - 17, yz + 11]], 0.12, P.hardcore);
        m.restore();
        if (d0.fine) groundPatch(m, yx, yz, 33, 21, 12, 8, GRADE + 0.125, P.hardcore, 0.05);
      }
    });
    if (stage >= 2) {
      m.part("services / equipment plinths and cable trench", () => {
        for (let i = 0; i < (d0.fine ? 6 : 2); i += 1) {
          const n = d0.fine ? 6 : 2;
          const px = yx - 13 + (i * 26) / (n - 1);
          if (d0.fine) m.beam(px - 1.1, px + 1.1, PIT, GRADE + (stage >= 3 ? 0.55 : 0.2), yz - 5.4, yz - 3.2, shade(P.concrete, 0.94 + 0.03 * (i % 2)), 0.03);
          else m.bar(px - 1.1, px + 1.1, PIT, GRADE + (stage >= 3 ? 0.55 : 0.2), yz - 5.4, yz - 3.2, shade(P.concrete, 0.94));
          if (stage === 2 && d0.fine) rebarCage(m, px, yz - 4.3, 0.7, 0.7, PIT + 0.1, GRADE + 1.2, 6);
        }
        // The cable trench, which is how a substation is wired and the one
        // permanent feature that survives every equipment change.
        m.bar(yx - 15, yx + 15, PIT, GRADE + 0.1, yz - 1.2, yz - 0.2, shade(P.concrete, 0.9));
        if (!d0.fine) return;
        for (let i = 0; i < 14; i += 1) {
          const cx = yx - 14.5 + (i * 29) / 13;
          if (stage >= 3) m.beam(cx - 0.95, cx + 0.95, GRADE + 0.1, GRADE + 0.2, yz - 1.25, yz - 0.15, shade(P.steel, 0.94 + 0.05 * (i % 2)), 0.02);
          else m.rod([cx - 0.9, PIT + 0.2, yz - 0.7], [cx + 0.9, PIT + 0.2, yz - 0.7], 0.05, 6, shade(P.dark, 1.2), false);
        }
      });
    }
    if (stage >= 3) {
      // Busbar gantries. Steel first and conductors last: a yard with its
      // structures up and nothing strung is exactly what stage three is.
      m.part("structure / busbar gantries", () => {
        for (let g = 0; g < (d0.fine ? 3 : 2); g += 1) {
          const n = d0.fine ? 3 : 2;
          const gx = yx - 11 + (g * 22) / (n - 1);
          const h = 9.5;
          for (const s of [-1, 1]) {
            if (d0.fine) {
              m.save().move(gx, 0, yz + s * 7);
              for (const cx of [-0.55, 0.55]) {
                for (const cz of [-0.55, 0.55]) m.rod([cx, GRADE, cz], [cx, GRADE + h, cz], 0.09, 5, P.galv, false);
              }
              for (let bay = 0; bay < 4; bay += 1) {
                latticeBay(m, GRADE + (h * bay) / 4, GRADE + (h * (bay + 1)) / 4, 0.55, 0.55, 0.045, 5, shade(P.galv, 0.92));
              }
              m.restore();
              m.beam(gx - 1.0, gx + 1.0, GRADE - 0.02, GRADE + 0.45, yz + s * 7 - 1.0, yz + s * 7 + 1.0, shade(P.concrete, 0.94), 0.03);
            } else {
              m.bar(gx - 0.55, gx + 0.55, GRADE, GRADE + h, yz + s * 7 - 0.55, yz + s * 7 + 0.55, P.galv);
            }
          }
          m.bar(gx - 0.4, gx + 0.4, GRADE + h, GRADE + h + 0.55, yz - 7.6, yz + 7.6, P.galv);
          if (!d0.fine || !built) continue;
          // Insulator strings and the conductor between them. The strings are
          // the discs; the conductor is a rod with a sag in it drawn as two.
          for (const s of [-1, 1]) {
            for (let d = 0; d < 4; d += 1) {
              m.column(gx, GRADE + h - 0.5 - d * 0.28, yz + s * 5.4, 0.19, 0.22, 0.22, 8, shade(P.hardcore, 1.02 - 0.03 * d), false);
            }
          }
          m.rod([gx, GRADE + h - 1.7, yz - 5.4], [gx, GRADE + h - 2.3, yz], 0.045, 5, P.dark, false);
          m.rod([gx, GRADE + h - 2.3, yz], [gx, GRADE + h - 1.7, yz + 5.4], 0.045, 5, P.dark, false);
        }
      });
    }
    if (built) {
      // Circuit breakers and disconnectors on the plinths. Three porcelain
      // stacks on a tank is what a breaker looks like and it is unmistakable.
      m.part("services / circuit breakers and disconnectors", () => {
        for (let i = 0; i < (d0.fine ? 4 : 2); i += 1) {
          const n = d0.fine ? 4 : 2;
          const px = yx - 12 + (i * 24) / (n - 1);
          m.bar(px - 1.0, px + 1.0, GRADE + 0.55, GRADE + 2.2, yz - 5.2, yz - 3.4, shade(P.galv, 0.96));
          // The porcelain stacks are 340 mm across. Close up they are the whole
          // silhouette of a breaker; at map range they are the tank under them.
          for (let p = 0; d0.fine && p < 3; p += 1) {
            const cx = px - 0.7 + p * 0.7;
            m.column(cx, GRADE + 2.2, yz - 4.3, 2.4, 0.17, 0.17, d0.fine ? 10 : d0.seg, shade(P.hardcore, 1.0), true);
            if (!d0.fine) continue;
            for (let sh = 0; sh < 6; sh += 1) {
              m.column(cx, GRADE + 2.5 + sh * 0.36, yz - 4.3, 0.1, 0.28, 0.28, 10, shade(P.hardcore, 0.92), false);
            }
            m.rod([cx, GRADE + 4.6, yz - 4.3], [cx, GRADE + 5.2, yz - 4.3], 0.05, 6, P.galv, false);
          }
          if (!d0.fine) continue;
          m.beam(px - 1.15, px + 1.15, GRADE + 2.2, GRADE + 2.34, yz - 5.35, yz - 3.25, shade(P.galv, 1.06), 0.02);
          m.bar(px - 0.3, px + 0.3, GRADE + 0.9, GRADE + 1.8, yz - 3.4, yz - 3.32, P.dark);
        }
      });
      propTransformers(m, k, YARD.x, YARD.z, W, D, d0, built);
    }
    propPylon(m, k, YARD.x, YARD.z, W, D, d0, built);
    if (done) {
      m.part("services / line termination tower", () => {
        // ONE tower, not two. A second lattice costs 1,500 close triangles and
        // adds a repeat of the same silhouette; the line it carries is what
        // says the yard is connected to something, and one tower carries it.
        pylonTower(m, -W * 0.30, D * 0.40, 22, d0);
      });
    }
  }

  /// Laboratory campus with service buildings. A campus is not a shed: it is a
  /// framed block with floors and a parapet, a lower workshop beside it, a
  /// glazed link where the two meet, and a courtyard between them. The workshop
  /// is the portal-framed half and the laboratory is the flat-roofed half, and
  /// the two together are the composition.
  function worksResearchCenter(m, k, c) {
    const { o, W, D, d0, stage } = c;
    const built = stage >= 4, done = stage >= 5;
    const b = k.block;
    const lab = {
      x: o.x + b.w / 2 + 13.5, z: o.z + 1.5, deck: GRADE + 0.18, col: shade(k.body, 1.02),
      block: { w: 24, dz: 15, eaves: 8.8, floors: 2 },
    };
    if (d0.fine) {
      if (stage >= 3) genericBlock(m, k, o, d0);
      deckedBlock(m, lab, stage, d0);
    } else {
      if (stage >= 4) {
        m.part("structure / laboratory block", () => {
          m.bar(lab.x - 12, lab.x + 12, GRADE, GRADE + 8.8, lab.z - 7.5, lab.z + 7.5, shade(k.body, done ? 1.02 : 0.92));
          deck(m, lab.x - 12, lab.x + 12, GRADE + 9.6, lab.z - 7.5, lab.z + 7.5, shade(P.roof, 0.98));
          for (const s of [-1, 1]) {
            m.bar(lab.x - 12, lab.x + 12, GRADE + 8.8, GRADE + 9.6, lab.z + s * 7.5 - 0.14, lab.z + s * 7.5 + 0.14, shade(k.body, 1.05));
            m.bar(lab.x + s * 12 - 0.14, lab.x + s * 12 + 0.14, GRADE + 8.8, GRADE + 9.6, lab.z - 7.5, lab.z + 7.5, shade(k.body, 1.02));
          }
        });
      } else if (stage === 3) {
        m.part("structure / laboratory frames", () => {
          for (let i = 0; i < 3; i += 1) {
            const fx = lab.x - 10 + i * 10;
            m.bar(fx - 0.3, fx + 0.3, GRADE, GRADE + 8.8, lab.z - 7, lab.z + 7, P.primer);
          }
        });
      }
    }
    if (stage >= 3) {
      // The link. Two campus buildings that do not touch are two buildings; the
      // glazed link is what makes them a campus.
      m.part("structure / glazed link and entrance", () => {
        const lx = (o.x + b.w / 2 + lab.x - 12) / 2, lz = o.z - 1.5;
        const gapw = (lab.x - 12) - (o.x + b.w / 2);
        m.bar(lx - gapw / 2 - 0.2, lx + gapw / 2 + 0.2, GRADE, GRADE + 4.6, lz - 4.2, lz + 4.2, shade(P.galv, 0.9));
        if (!d0.fine) return;
        for (const s of [-1, 1]) {
          for (let i = 0; i < 5; i += 1) {
            const gx = lx - gapw / 2 + 0.6 + (i * (gapw - 1.2)) / 4;
            m.bar(gx - 0.75, gx + 0.75, GRADE + 0.6, GRADE + 4.0, lz + s * 4.2 - 0.04, lz + s * 4.2 + 0.04, P.glass);
            m.beam(gx - 0.88, gx + 0.88, GRADE + 0.5, GRADE + 4.1, lz + s * 4.2 - 0.09, lz + s * 4.2 + 0.09, shade(P.galv, 1.02), 0.014);
          }
        }
        m.beam(lx - gapw / 2 - 0.6, lx + gapw / 2 + 0.6, GRADE + 4.6, GRADE + 4.9, lz - 4.6, lz + 4.6, P.roof, 0.025);
        // The entrance, on the link, with a canopy and steps. A campus is
        // entered at the join between its two halves.
        m.bar(lx - 2.0, lx + 2.0, GRADE + 0.05, GRADE + 2.6, lz - 4.24, lz - 4.2, P.glass);
        m.beam(lx - 3.2, lx + 3.2, GRADE + 3.2, GRADE + 3.44, lz - 7.4, lz - 4.0, shade(k.accent, 1.08), 0.025);
        for (const s of [-1, 1]) m.rod([lx + s * 2.8, GRADE, lz - 7.0], [lx + s * 2.8, GRADE + 3.2, lz - 7.0], 0.12, 8, P.galv, false);
      });
    }
    if (built) {
      // Fume extract. A laboratory's roof is covered in it and nothing else on
      // this list has anything like it: a run of stacks with weather cowls on a
      // plant deck, ducted down into the block.
      m.part("services / fume extract and roof plant", () => {
        const ry = lab.deck + lab.block.eaves;
        if (d0.fine) gratedDeck(m, lab.x + 3, lab.z, 13, 8, ry + 1.1, d0, true);
        for (let i = 0; i < (d0.fine ? 5 : 2); i += 1) {
          const n = d0.fine ? 5 : 2;
          const sx = lab.x - 1.5 + (i * 12) / (n - 1);
          m.column(sx, ry + 1.1, lab.z - 2.2, 4.6, 0.42, 0.42, d0.fine ? 12 : d0.seg, shade(P.galv, 1.0), false);
          if (!d0.fine) continue;
          m.column(sx, ry + 5.7, lab.z - 2.2, 0.42, 0.62, 0.42, 12, shade(P.dark, 1.2), true);
          m.column(sx, ry + 1.1, lab.z - 2.2, 0.14, 0.52, 0.52, 12, shade(P.galv, 0.86), false);
        }
        m.bar(lab.x - 3, lab.x + 6, ry + 1.1, ry + 3.4, lab.z + 1.0, lab.z + 3.6, shade(P.galv, 0.95));
        if (d0.fine) {
          for (let f = 0; f < 6; f += 1) {
            m.bar(lab.x - 2.9 + f * 1.4, lab.x - 2.0 + f * 1.4, ry + 1.4, ry + 3.1, lab.z + 3.6, lab.z + 3.72, shade(P.steel, 0.9 + 0.06 * (f % 2)));
          }
        }
      });
    }
    if (done) {
      m.part("yard / courtyard and planting", () => {
        const cx = (o.x + lab.x) / 2, cz = o.z + b.dz / 2 + 7.5;
        m.save().move(0, GRADE, 0);
        m.plate([[cx - 13, cz - 5], [cx + 13, cz - 5], [cx + 13, cz + 5], [cx - 13, cz + 5]], 0.12, P.kerb);
        m.restore();
        for (let i = 0; i < (d0.fine ? 6 : 3); i += 1) {
          const n = d0.fine ? 6 : 3;
          const px = cx - 10 + (i * 20) / (n - 1);
          m.mound(px, GRADE + 0.12, cz + 3.2, 1.5, 0.5, d0.fine ? 7 : 4, shade(P.grass, 0.96 + 0.05 * (i % 3)));
          if (!d0.fine) continue;
          m.beam(px - 1.7, px + 1.7, GRADE + 0.12, GRADE + 0.46, cz + 1.3, cz + 1.7, P.kerb, 0.02);
          m.bar(px - 1.6, px + 1.6, GRADE + 0.46, GRADE + 0.52, cz + 1.3, cz + 1.72, shade(P.timber, 0.94));
        }
      });
      if (d0.fine) propAnnex(m, k, o.x, o.z, W, D, d0, built);
    }
  }

  /// Machine-tool halls and equipment loading. Two parallel bays with a valley
  /// between them — a tall machining bay and a lower fitting bay — and the one
  /// thing a machine shop has that a warehouse does not: an overhead travelling
  /// crane on rail beams down the length of the tall bay, standing before the
  /// walls do and still visible through the gable once they are up.
  function worksMachineryWorks(m, k, c) {
    const { o, W, D, d0, stage } = c;
    const built = stage >= 4, done = stage >= 5;
    const b = k.block;
    // ALONGSIDE, not behind. Two bays sharing a valley gutter is what a machine
    // shop is, and it is also the only way a second 20 m bay fits on the pad:
    // behind the machining hall there are 24 m of fill and the bay is 14 deep.
    const bay2 = {
      x: o.x + b.w / 2 + 1.6 + b.w * 0.36, z: o.z, deck: GRADE + 0.18, col: shade(k.body, 0.95),
      block: { w: b.w * 0.72, dz: b.dz * 0.78, eaves: b.eaves * 0.62, ridge: b.ridge * 0.58, bays: 9, ends: 5 },
      tag: "fitting bay",
    };
    if (d0.fine) {
      if (stage >= 3) genericBlock(m, k, o, d0);
      stagedBlock(m, bay2, stage, d0);
    } else if (stage >= 4) {
      m.part("structure / fitting bay", () => {
        m.bar(bay2.x - bay2.block.w / 2, bay2.x + bay2.block.w / 2, GRADE, GRADE + bay2.block.eaves,
          bay2.z - bay2.block.dz / 2, bay2.z + bay2.block.dz / 2, shade(k.body, done ? 0.95 : 0.86));
      });
    }
    if (stage >= 3) {
      // The travelling crane. Corbels off the columns carry a crane rail beam
      // down each side of the tall bay, and the bridge spans between them with
      // a crab on it. It is drawn before the cladding because that is when it
      // is installed, and it is why the bay is as tall as it is.
      m.part("structure / overhead travelling crane", () => {
        const cy = GRADE + 0.18 + b.eaves * 0.68;
        for (const s of [-1, 1]) {
          const cx = o.x + s * (b.w / 2 - 1.1);
          if (d0.fine) {
            m.beam(cx - 0.42, cx + 0.42, cy, cy + 0.95, o.z - b.dz / 2, o.z + b.dz / 2, shade(P.primer, 1.02), 0.025);
            m.bar(cx - 0.2, cx + 0.2, cy + 0.95, cy + 1.08, o.z - b.dz / 2, o.z + b.dz / 2, shade(P.steel, 1.05));
            for (let i = 0; i < 5; i += 1) {
              const bz = o.z - b.dz / 2 + 1.4 + (i * (b.dz - 2.8)) / 4;
              m.webPlate([[cx + s * 0.42, cy], [cx + s * 1.5, cy], [cx + s * 0.42, cy - 1.3]], bz - 0.16, bz + 0.16, shade(P.primer, 0.94));
            }
          } else {
            m.bar(cx - 0.42, cx + 0.42, cy, cy + 0.95, o.z - b.dz / 2, o.z + b.dz / 2, P.primer);
          }
        }
        const gz = o.z + (built ? b.dz * 0.16 : -b.dz * 0.22);
        m.bar(o.x - b.w / 2 + 0.7, o.x + b.w / 2 - 0.7, cy + 1.08, cy + 2.2, gz - 0.9, gz + 0.9, P.safety);
        if (!d0.fine) return;
        m.beam(o.x - b.w / 2 + 0.7, o.x + b.w / 2 - 0.7, cy + 2.2, cy + 2.42, gz - 1.05, gz + 1.05, shade(P.safety, 1.08), 0.02);
        // The crab, its rope and the hook block. A gantry with nothing hanging
        // off it is a beam.
        const crab = o.x + (built ? b.w * 0.18 : -b.w * 0.1);
        m.beam(crab - 1.1, crab + 1.1, cy + 2.2, cy + 3.0, gz - 1.0, gz + 1.0, shade(P.dark, 1.1), 0.025);
        m.rod([crab, cy - 2.6, gz], [crab, cy + 2.2, gz], 0.045, 6, P.dark, false);
        m.column(crab, cy - 3.2, gz, 0.6, 0.3, 0.3, 8, shade(P.steel, 0.92), true);
        m.webPlate([[crab - 0.18, cy - 3.9], [crab + 0.18, cy - 3.9], [crab + 0.18, cy - 3.2], [crab - 0.18, cy - 3.2]],
          gz - 0.06, gz + 0.06, shade(P.steel, 0.82));
      });
    }
    if (built) {
      // What a machine shop keeps outside: stillages of finished work, swarf
      // skips and a compressor house. All three are on every one of them.
      m.part("yard / stillages and swarf skips", () => {
        for (let i = 0; i < (d0.fine ? 6 : 2); i += 1) {
          const sx = W * 0.14 + (i % 3) * 3.4, sz = -D * 0.10 + Math.floor(i / 3) * 3.4;
          m.bar(sx - 1.4, sx + 1.4, GRADE + 0.16, GRADE + 1.3, sz - 1.2, sz + 1.2, shade(P.primer, 0.94 + 0.06 * (i % 3)));
          if (!d0.fine) continue;
          for (const cx of [-1.35, 1.35]) {
            for (const cz of [-1.15, 1.15]) {
              m.bar(sx + cx - 0.11, sx + cx + 0.11, GRADE, GRADE + 1.35, sz + cz - 0.11, sz + cz + 0.11, shade(P.steel, 1.0));
            }
          }
          m.beam(sx - 1.45, sx + 1.45, GRADE + 1.3, GRADE + 1.42, sz - 1.25, sz + 1.25, shade(P.steel, 0.9), 0.02);
        }
        if (!d0.fine) return;
        for (let i = 0; i < 2; i += 1) {
          const kx = W * 0.14 + i * 5.0, kz = -D * 0.19;
          // A skip is a tapered open box: the taper is what says it tips.
          m.webPlate([[kx - 1.9, GRADE], [kx + 1.9, GRADE], [kx + 2.3, GRADE + 1.5], [kx - 2.3, GRADE + 1.5]],
            kz - 1.1, kz + 1.1, shade(P.safety, 0.86));
          m.bar(kx - 2.2, kx + 2.2, GRADE + 1.4, GRADE + 1.55, kz - 1.16, kz + 1.16, shade(P.safety, 1.06));
          m.rod([kx - 1.4, GRADE + 0.9, kz - 1.12], [kx + 1.4, GRADE + 0.9, kz - 1.12], 0.07, 6, shade(P.dark, 1.1), false);
        }
      });
      m.part("services / compressor house", () => {
        const hx = o.x - b.w / 2 - 6.5, hz = o.z + b.dz * 0.3;
        m.bar(hx - 3.2, hx + 3.2, GRADE, GRADE + 3.4, hz - 2.6, hz + 2.6, shade(P.clad, 0.98));
        if (!d0.fine) return;
        m.beam(hx - 3.4, hx + 3.4, GRADE + 3.4, GRADE + 3.64, hz - 2.8, hz + 2.8, P.roof, 0.025);
        louvreBank(m, hx - 2.4, hx + 0.6, GRADE + 1.4, GRADE + 2.8, hz + 2.6, 0.16, 4, shade(P.galv, 0.96));
        m.bar(hx + 1.2, hx + 2.4, GRADE + 0.05, GRADE + 2.3, hz + 2.6, hz + 2.64, P.door);
        m.column(hx + 4.4, GRADE, hz, 4.2, 0.85, 0.85, 12, shade(P.galv, 1.0), true);        // air receiver
        m.beam(hx + 3.5, hx + 5.3, GRADE, GRADE + 0.3, hz - 0.9, hz + 0.9, shade(P.concrete, 0.94), 0.03);
        m.rod([hx + 3.2, GRADE + 2.6, hz], [hx + 4.4, GRADE + 2.6, hz], 0.14, 8, shade(P.galv, 0.94), true);
      });
    }
    propYardCrane(m, k, YARD.x, D * 0.66, W, D, d0, built);
    if (stage >= 3) propDocks(m, k, YARD.x, YARD.z, W, D, d0, built);
    if (d0.fine) propPipeStack(m, k, YARD.x, YARD.z, W, D, d0, built);
  }

  /// A generic generation facility, and it stays generic on purpose: a machine
  /// hall, an energy-conversion block, a heat-rejection bank, consumables
  /// handling into it and a switchyard out of it. There is no named reactor
  /// here, no named turbine and no fuel: the roadmap puts technology-specific
  /// variants in a later pass and this pass is not allowed to invent one and
  /// present it as sourced.
  function worksGeneration(m, k, c) {
    const { o, W, D, d0, stage } = c;
    const built = stage >= 4, done = stage >= 5;
    const b = k.block;
    const conv = {
      x: o.x + b.w / 2 + 9.5, z: o.z, deck: GRADE + 0.18, col: shade(k.body, 0.92),
      block: { w: 17, dz: b.dz * 0.85, eaves: b.eaves + 5.5, ridge: b.eaves + 6.2, bays: 7, ends: 7 },
      tag: "conversion block",
    };
    if (d0.fine) {
      if (stage >= 3) genericBlock(m, k, o, d0);
      stagedBlock(m, conv, stage, d0);
    } else if (stage >= 4) {
      m.part("structure / conversion block", () => {
        m.bar(conv.x - 8.5, conv.x + 8.5, GRADE, GRADE + conv.block.eaves, conv.z - conv.block.dz / 2, conv.z + conv.block.dz / 2,
          shade(k.body, done ? 0.92 : 0.84));
      });
    }
    if (stage >= 3) {
      // Heat rejection. Every generation technology on the roadmap's later list
      // rejects heat, and a raised bank of fan cells is the shape that is
      // common to all of them — which is exactly why it is the one drawn here.
      m.part("services / heat rejection bank", () => {
        const hx = o.x - b.w * 0.1, hz = o.z + b.dz / 2 + 7.6, hy = GRADE + (built ? 7.4 : 3.0);
        const cells = d0.fine ? 4 : 2;
        for (let i = 0; i < cells; i += 1) {
          const cx = hx - 9 + (i * 18) / (cells - 1);
          for (const s of [-1, 1]) {
            if (d0.fine) m.beam(cx - 1.9, cx - 1.5, GRADE, hy, hz + s * 3.4 - 0.2, hz + s * 3.4 + 0.2, P.steel, 0.018);
            else m.bar(cx - 1.9, cx - 1.5, GRADE, hy, hz + s * 3.4 - 0.2, hz + s * 3.4 + 0.2, P.steel);
          }
        }
        gratedDeck(m, hx, hz, 20, 7.6, hy, d0, true);
        if (!built) return;
        for (let i = 0; i < cells; i += 1) {
          const cx = hx - 7.5 + (i * 15) / (cells - 1);
          m.column(cx, hy, hz, 1.5, 2.4, 2.4, d0.fine ? 14 : d0.seg, shade(P.galv, 0.94), false);
          m.column(cx, hy + 1.5, hz, 0.4, 2.4, 2.0, d0.fine ? 14 : d0.seg, shade(P.dark, 1.1), false);
          if (!d0.fine) continue;
          // The fan itself, as a hub and four blades. A ring with nothing in it
          // is a hole in the deck.
          m.column(cx, hy + 0.9, hz, 0.5, 0.4, 0.4, 10, shade(P.steel, 0.9), true);
          for (let bl = 0; bl < 4; bl += 1) {
            const a = bl * 90 * DEG;
            m.save().move(cx, hy + 1.1, hz).rotY(a / DEG).rotZ(14);
            m.beam(0.35, 2.15, -0.05, 0.05, -0.32, 0.32, shade(P.hut, 1.0), 0.012);
            m.restore();
          }
          for (let g = 0; g < 5; g += 1) {
            m.rod([cx - 2.3, hy + 1.9, hz - 2.3 + g * 1.15], [cx + 2.3, hy + 1.9, hz - 2.3 + g * 1.15], 0.03, 5, shade(P.galv, 1.02), false);
          }
        }
      });
    }
    if (stage >= 3) {
      // Consumables handling: a covered gallery from a ground hopper up into
      // the conversion block. Deliberately unlabelled as to what it carries —
      // the sim records no fuel and the art will not invent one.
      m.part("services / consumables handling gallery", () => {
        const bxp = o.x + b.w * 0.1, bz = o.z - b.dz / 2 - 13.0;
        m.bar(bxp - 5.5, bxp + 5.5, GRADE, GRADE + 3.2, bz - 4.4, bz + 4.4, shade(P.concrete, 0.94));
        if (d0.fine) {
          m.beam(bxp - 5.7, bxp + 5.7, GRADE + 3.2, GRADE + 3.5, bz - 4.6, bz + 4.6, P.kerb, 0.03);
          m.column(bxp, GRADE + 3.5, bz, 3.2, 3.4, 2.0, 14, shade(P.galv, 0.94), false);
          m.column(bxp, GRADE + 6.7, bz, 0.4, 2.0, 2.0, 14, shade(P.galv, 1.04), false);
        }
        conveyor(m, [bxp, GRADE + 7.4, bz], [conv.x - 6.0, GRADE + conv.block.eaves * 0.8, conv.z - conv.block.dz * 0.3], 2.6, d0);
      });
    }
    propStack(m, k, YARD.x, YARD.z, W, D, d0, built);
    if (built) {
      propTransformers(m, k, YARD.x, YARD.z, W, D, d0, built);
      propTanks(m, k, YARD.x, YARD.z, W, D, d0, built);
    }
    if (done) {
      m.part("services / takeoff gantry", () => {
        const gx = -W * 0.34, gz = D * 0.36;
        for (const s of [-1, 1]) {
          if (d0.fine) {
            m.save().move(gx + s * 5.5, 0, gz);
            for (const cx of [-0.5, 0.5]) {
              for (const cz of [-0.5, 0.5]) m.rod([cx, GRADE, cz], [cx, GRADE + 11, cz], 0.085, 5, P.galv, false);
            }
            for (let bay = 0; bay < 4; bay += 1) latticeBay(m, GRADE + bay * 2.75, GRADE + (bay + 1) * 2.75, 0.5, 0.5, 0.045, 5, shade(P.galv, 0.92));
            m.restore();
          } else {
            m.bar(gx + s * 5.5 - 0.5, gx + s * 5.5 + 0.5, GRADE, GRADE + 11, gz - 0.5, gz + 0.5, P.galv);
          }
        }
        m.bar(gx - 6.2, gx + 6.2, GRADE + 11, GRADE + 11.5, gz - 0.35, gz + 0.35, P.galv);
        if (!d0.fine) return;
        for (const px of [-4, 0, 4]) {
          for (let d = 0; d < 4; d += 1) {
            m.column(gx + px, GRADE + 10.5 - d * 0.28, gz, 0.19, 0.22, 0.22, 8, shade(P.hardcore, 1.0 - 0.03 * d), false);
          }
          m.rod([gx + px, GRADE + 9.3, gz], [gx + px + 1.6, GRADE + 8.0, gz - 7.0], 0.045, 5, P.dark, false);
        }
      });
    }
  }

  /// Industrial processing hall, tanks and pipework, material handling. The
  /// signature is that most of the plant is OUTSIDE the building: an open steel
  /// process structure with vessels stacked through it on grating decks, a
  /// bunded tank farm, a drum on saddles, a pipe rack tying it together and a
  /// road tanker loading bay at the end of it.
  function worksProcessingPlant(m, k, c) {
    const { o, W, D, d0, stage } = c;
    const built = stage >= 4, done = stage >= 5;
    const b = k.block;
    const tx = o.x + b.w / 2 + 11, tz = o.z + 1;
    if (d0.fine && stage >= 3) genericBlock(m, k, o, d0);
    if (stage >= 2) {
      m.part("earthworks / vessel bases and bund slab", () => {
        // A process structure sits on its own raft, and the tank farm sits in a
        // bund. Both are cast before any steel arrives, and at the foundation
        // stage they are the only thing on this side of the site.
        m.bar(tx - 7.5, tx + 7.5, PIT, GRADE + (stage >= 3 ? 0.5 : 0.22), tz - 6, tz + 6, shade(P.concrete, 0.94));
        if (stage === 2 && d0.fine) {
          for (let i = 0; i < 4; i += 1) {
            rebarCage(m, tx - 5 + (i % 2) * 10, tz - 4 + Math.floor(i / 2) * 8, 0.8, 0.8, PIT + 0.1, GRADE + 1.6, 6);
          }
        }
      });
    }
    if (stage >= 3) {
      // The process structure. Four columns, decks between them, vessels
      // standing through the decks and a stair up the outside. The vessels
      // arrive with the envelope, so at stage three the structure is empty and
      // says so.
      m.part("structure / process tower and access decks", () => {
        // Four decks close up; two at map range, where the structure reads as
        // its outline and the decks only have to say it has more than one.
        const levels = d0.fine ? 4 : 2, lh = d0.fine ? 4.2 : 8.4;
        for (const sx of [-1, 1]) {
          for (const sz of [-1, 1]) {
            const cx = tx + sx * 5.5, cz = tz + sz * 4.2;
            if (d0.fine) m.beam(cx - 0.26, cx + 0.26, GRADE + 0.5, GRADE + 0.5 + levels * lh, cz - 0.26, cz + 0.26, P.primer, 0.02);
            else m.bar(cx - 0.26, cx + 0.26, GRADE + 0.5, GRADE + 0.5 + levels * lh, cz - 0.26, cz + 0.26, P.primer);
          }
        }
        for (let l = 1; l <= levels; l += 1) {
          gratedDeck(m, tx, tz, 11.6, 8.9, GRADE + 0.5 + l * lh, d0, true);
        }
        if (!d0.fine) return;
        for (let l = 0; l < levels; l += 1) {
          const y0 = GRADE + 0.5 + l * lh;
          for (const sz of [-1, 1]) {
            m.rod([tx - 5.5, y0, tz + sz * 4.2], [tx + 5.5, y0 + lh, tz + sz * 4.2], 0.075, 5, shade(P.primer, 0.92), false);
          }
          // The stair up the outside, in flights with a landing. A structure
          // nobody can climb is a sculpture.
          for (let s = 0; s < 6; s += 1) {
            m.beam(tx + 6.0, tx + 7.6, y0 + (s * lh) / 6, y0 + 0.12 + (s * lh) / 6,
              tz - 4.0 + s * 0.55, tz - 3.4 + s * 0.55, P.galv, 0.012);
          }
          m.rod([tx + 7.7, y0 + 0.9, tz - 4.0], [tx + 7.7, y0 + lh + 0.9, tz - 0.7], 0.035, 6, shade(P.galv, 1.04), false);
        }
      });
    }
    if (built) {
      m.part("services / process vessels", () => {
        vessel(m, tx - 3.4, tz - 1.6, GRADE + 1.2, 13.5, 1.9, d0, shade(P.galv, 0.96));
        vessel(m, tx + 3.2, tz + 2.0, GRADE + 5.6, 8.6, 1.5, d0, shade(P.galv, 1.02));
        if (!d0.fine) return;
        // The lines between them: a process structure is vessels plus the pipe
        // that connects them, and pipe drawn as straight runs with an elbow at
        // each end is what that looks like.
        m.rod([tx - 3.4, GRADE + 14.4, tz - 1.6], [tx - 3.4, GRADE + 16.0, tz - 1.6], 0.24, 8, shade(P.galv, 1.0), true);
        m.rod([tx - 3.4, GRADE + 16.0, tz - 1.6], [tx + 3.2, GRADE + 16.0, tz - 1.6], 0.24, 8, shade(P.galv, 0.94), true);
        m.rod([tx + 3.2, GRADE + 16.0, tz - 1.6], [tx + 3.2, GRADE + 14.2, tz + 2.0], 0.24, 8, shade(P.galv, 0.98), true);
        m.rod([tx - 5.2, GRADE + 2.4, tz - 1.6], [tx - 3.4, GRADE + 2.4, tz - 1.6], 0.3, 8, shade(P.galv, 0.92), true);
      });
      processDrum(m, tx, tz, d0);
    }
    propTanks(m, k, YARD.x, YARD.z, W, D, d0, built);
    if (stage >= 3) propPipeRack(m, k, YARD.x, YARD.z, W, D, d0, built);
    propStack(m, k, YARD.x, YARD.z, W, D, d0, built);
    if (done) {
      m.part("yard / road tanker loading bay", () => {
        const lx = -W * 0.30, lz = D * 0.30;
        m.save().move(0, GRADE, 0);
        m.plate([[lx - 8, lz - 4], [lx + 8, lz - 4], [lx + 8, lz + 4], [lx - 8, lz + 4]], 0.14, shade(P.concrete, 0.96));
        m.restore();
        for (const s of [-1, 1]) {
          if (d0.fine) m.beam(lx + s * 6.4 - 0.2, lx + s * 6.4 + 0.2, GRADE + 0.14, GRADE + 6.4, lz - 2.6, lz - 2.2, P.steel, 0.018);
          else m.bar(lx + s * 6.4 - 0.2, lx + s * 6.4 + 0.2, GRADE + 0.14, GRADE + 6.4, lz - 2.6, lz - 2.2, P.steel);
        }
        m.bar(lx - 7, lx + 7, GRADE + 6.4, GRADE + 6.9, lz - 2.7, lz - 2.1, shade(P.galv, 0.96));
        if (!d0.fine) return;
        gratedDeck(m, lx, lz - 1.0, 12, 1.9, GRADE + 4.4, d0, true);
        for (let i = 0; i < 3; i += 1) {
          const ax = lx - 4 + i * 4;
          m.rod([ax, GRADE + 6.4, lz - 2.4], [ax, GRADE + 5.1, lz - 0.2], 0.16, 8, shade(P.galv, 1.0), true);
          m.rod([ax, GRADE + 5.1, lz - 0.2], [ax, GRADE + 3.6, lz + 0.4], 0.13, 8, shade(P.dark, 1.2), true);
          m.column(ax, GRADE + 0.14, lz + 2.6, 1.1, 0.22, 0.22, 8, P.safety, true);
        }
        for (let i = 0; i < 6; i += 1) {
          m.bar(lx - 7 + i * 2.6, lx - 5.8 + i * 2.6, GRADE + 0.14, GRADE + 0.17, lz + 3.0, lz + 3.2, P.lane);
        }
      });
    }
  }

  /// The drum on saddles beside the process structure, and the pump skid under
  /// it. Split out of the vessels so the picker can name the two apart, which
  /// is what a part is for.
  function processDrum(m, tx, tz, d0) {
    m.part("services / horizontal drum and pumps", () => {
      drum(m, tx + 1.5, tz + 8.5, 9.5, 1.5, d0, shade(P.galv, 0.98));
      if (!d0.fine) return;
      for (let i = 0; i < 3; i += 1) {
        const px = tx - 2.5 + i * 3.0;
        m.beam(px - 0.9, px + 0.9, GRADE, GRADE + 0.4, tz + 11.4, tz + 12.8, shade(P.concrete, 0.94), 0.025);
        m.rod([px - 0.7, GRADE + 0.75, tz + 12.1], [px + 0.5, GRADE + 0.75, tz + 12.1], 0.35, 10, shade(P.safety, 0.92), true);
        m.column(px + 0.5, GRADE + 0.4, tz + 12.1, 0.9, 0.42, 0.42, 10, shade(P.steel, 0.95), true);
        m.rod([px - 0.7, GRADE + 0.75, tz + 12.1], [px - 1.1, GRADE + 0.75, tz + 12.1], 0.16, 8, shade(P.galv, 1.0), true);
      }
    });
  }

  /// Rail, road and container transfer yard. The yard is the facility here and
  /// the shed is a detail on it: two ballasted roads with a stop block, a
  /// rail-mounted gantry spanning them, container stacks in rows and a long low
  /// transit shed with a canopy down its side. The staging is the yard's, not a
  /// building's — formation and ballast first, then track, then the gantry.
  function worksFreightTerminal(m, k, c) {
    const { o, W, D, d0, stage } = c;
    const built = stage >= 4, done = stage >= 5;
    const b = k.block;
    const rz = o.z + b.dz / 2 + 7.5;
    if (d0.fine && stage >= 3) genericBlock(m, k, o, d0);
    m.part("yard / rail formation and sidings", () => {
      // Stage one is a graded formation with a capping layer on it; the ballast
      // and the track come later, because that is the order a siding is laid in
      // and because a terminal with track already down is not a terminal being
      // built.
      if (stage <= 1) {
        m.save().move(0, GRADE, 0);
        m.plate([[-W * 0.42, rz - 6.5], [W * 0.42, rz - 6.5], [W * 0.42, rz + 6.5], [-W * 0.42, rz + 6.5]], 0.1, P.hardcore);
        m.restore();
        if (d0.fine) groundPatch(m, 0, rz, W * 0.8, 12, 12, 4, GRADE + 0.105, P.hardcore, 0.07);
        return;
      }
      for (let t = 0; t < 2; t += 1) {
        railTrack(m, -W * 0.40, W * 0.36, rz + t * 5.2 - 2.6, d0, stage >= 3);
      }
    });
    if (stage >= 3) propGantry(m, k, o.x, rz - 2.6 - D * 0.18, W, D, d0, built);
    if (built) propContainers(m, k, YARD.x, YARD.z, W, D, d0, built);
    if (built) {
      // The canopy along the shed, and the doors under it. A transit shed is a
      // long building whose entire point is the covered edge where road meets
      // rail, and that edge is what makes this kind read as transfer rather
      // than as storage.
      m.part("structure / transfer canopy and doors", () => {
        const cz = o.z - b.dz / 2 - 3.4;
        const posts = d0.fine ? 7 : 3;
        for (let i = 0; i < posts; i += 1) {
          const px = o.x - b.w / 2 + (i * b.w) / (posts - 1);
          if (d0.fine) m.beam(px - 0.16, px + 0.16, GRADE, GRADE + 5.4, cz - 0.16, cz + 0.16, P.steel, 0.018);
          else m.bar(px - 0.16, px + 0.16, GRADE, GRADE + 5.4, cz - 0.16, cz + 0.16, P.steel);
        }
        m.save().move(0, GRADE + 5.4, 0);
        m.plate([[o.x - b.w / 2 - 0.8, cz - 1.2], [o.x + b.w / 2 + 0.8, cz - 1.2],
          [o.x + b.w / 2 + 0.8, o.z - b.dz / 2], [o.x - b.w / 2 - 0.8, o.z - b.dz / 2]], 0.24, shade(P.roof, 1.04));
        m.restore();
        for (let i = 0; i < (d0.fine ? 6 : 2); i += 1) {
          const n = d0.fine ? 6 : 2;
          const dx = o.x - b.w * 0.4 + (i * b.w * 0.8) / (n - 1);
          m.bar(dx - 1.6, dx + 1.6, GRADE + 0.24, GRADE + 4.4, o.z - b.dz / 2 - 0.06, o.z - b.dz / 2 + 0.02, P.door);
          if (!d0.fine) continue;
          for (let s = 0; s < 7; s += 1) {
            m.bar(dx - 1.52, dx + 1.52, GRADE + 0.4 + s * 0.56, GRADE + 0.82 + s * 0.56,
              o.z - b.dz / 2 - 0.12, o.z - b.dz / 2 - 0.06, shade(P.door, 1.08));
          }
          m.beam(dx - 1.8, dx + 1.8, GRADE, GRADE + 0.24, o.z - b.dz / 2 - 1.4, o.z - b.dz / 2 + 0.1, P.kerb, 0.02);
        }
        if (d0.fine) {
          m.beam(o.x - b.w / 2 - 0.9, o.x + b.w / 2 + 0.9, GRADE + 5.4, GRADE + 5.72, cz - 1.32, cz - 1.0, shade(P.galv, 0.96), 0.02);
          groundPatch(m, o.x, cz + 1.4, b.w, 5.6, 12, 4, GRADE + 0.012, P.asphalt, 0.05);
        }
      });
    }
    if (done) {
      m.part("yard / trailer park and running lanes", () => {
        const px = -W * 0.24, pz = D * 0.34;
        m.save().move(0, GRADE, 0);
        m.plate([[px - 14, pz - 5], [px + 14, pz - 5], [px + 14, pz + 5], [px - 14, pz + 5]], 0.12, P.asphalt);
        m.restore();
        for (let i = 0; i < (d0.fine ? 9 : 3); i += 1) {
          const n = d0.fine ? 9 : 3;
          const lx = px - 13 + (i * 26) / (n - 1);
          m.bar(lx - 0.1, lx + 0.1, GRADE + 0.12, GRADE + 0.15, pz - 4.4, pz + 4.4, P.lane);
        }
        if (!d0.fine) return;
        for (let i = 0; i < 3; i += 1) {
          const tx2 = px - 9 + i * 9;
          m.bar(tx2 - 5.4, tx2 + 5.4, GRADE + 1.2, GRADE + 3.9, pz - 1.4, pz + 1.4, shade(P.clad, 1.0 - 0.03 * i));
          m.beam(tx2 - 5.2, tx2 + 5.2, GRADE + 1.0, GRADE + 1.2, pz - 1.2, pz + 1.2, shade(P.dark, 1.15), 0.02);
          for (const wz of [pz - 1.1, pz + 1.1]) {
            for (const wx of [tx2 - 3.4, tx2 - 2.4, tx2 + 4.0]) {
              m.rod([wx, GRADE + 0.52, wz], [wx, GRADE + 0.52, wz + 0.16], 0.52, 8, P.dark, true);
            }
          }
          m.beam(tx2 - 5.5, tx2 - 4.6, GRADE + 0.2, GRADE + 1.0, pz - 0.5, pz + 0.5, shade(P.steel, 0.94), 0.02);
        }
      });
    }
    if (d0.fine) propRoadStrip(m, k, YARD.x, YARD.z, W, D, d0, built);
  }

  /// Storage halls and loading docks. The one kind whose building IS the
  /// facility: a long high-bay hall with racking in it, a full-length dock
  /// elevation under a continuous canopy, and the sprinkler tank and pump house
  /// that every one of them has standing outside in the open.
  function worksWarehouse(m, k, c) {
    const { o, W, D, d0, stage } = c;
    const built = stage >= 4, done = stage >= 5;
    const b = k.block;
    if (d0.fine && stage >= 3) genericBlock(m, k, o, d0);
    if (built) {
      // Racking, installed once the envelope is on. It is the reason the hall
      // is as tall as it is and it is visible through the open dock doors, so
      // it is worth its triangles even after the walls close.
      m.part("structure / pallet racking", () => {
        // Racking is inside a sheeted building. At map range the building is
        // opaque, so this is one of the few parts that draws nothing at all
        // there rather than a cheaper version of itself.
        if (!d0.fine) return;
        const runs = 4, bays = 6;
        for (let r = 0; r < runs; r += 1) {
          const rz = o.z - b.dz * 0.3 + (r * b.dz * 0.6) / (runs - 1);
          for (let i = 0; i <= bays; i += 1) {
            const rx = o.x - b.w * 0.36 + (i * b.w * 0.72) / bays;
            for (const sz of [-1.2, 1.2]) {
              m.bar(rx - 0.1, rx + 0.1, GRADE + 0.18, GRADE + b.eaves * 0.82, rz + sz - 0.1, rz + sz + 0.1, shade(P.safety, 0.88));
            }
          }
          if (!d0.fine) continue;
          for (let l = 0; l < 4; l += 1) {
            const y = GRADE + 1.7 + l * 2.1;
            for (const sz of [-1.2, 1.2]) {
              m.beam(o.x - b.w * 0.36, o.x + b.w * 0.36, y, y + 0.16, rz + sz - 0.09, rz + sz + 0.09, shade(P.steel, 1.02), 0.014);
            }
            for (let p = 0; p < 5; p += 1) {
              const px = o.x - b.w * 0.32 + (p * b.w * 0.64) / 4;
              if ((p + l + r) % 4 === 3) continue;                     // a warehouse is never full
              m.bar(px - 0.55, px + 0.55, y + 0.16, y + 1.5, rz - 1.15, rz + 1.15, shade(P.hardcore, 0.9 + 0.05 * ((p + l) % 3)));
            }
          }
        }
      });
      m.part("structure / dock elevation and canopy", () => {
        const dz0 = o.z - b.dz / 2;
        const docks = d0.fine ? 7 : 3;
        m.bar(o.x - b.w / 2, o.x + b.w / 2, GRADE, GRADE + 1.25, dz0 - 3.2, dz0, P.concrete);
        for (let i = 0; i < docks; i += 1) {
          const dx = o.x - b.w * 0.40 + (i * b.w * 0.80) / (docks - 1);
          m.bar(dx - 1.7, dx + 1.7, GRADE + 1.36, GRADE + 4.4, dz0 - 0.12, dz0 - 0.02, P.dark);
          if (!d0.fine) continue;
          for (let s = 0; s < 7; s += 1) {
            m.bar(dx - 1.62, dx + 1.62, GRADE + 1.5 + s * 0.42, GRADE + 1.84 + s * 0.42, dz0 - 0.18, dz0 - 0.12, shade(P.door, 1.07));
          }
          for (const s of [-1, 1]) {
            m.beam(dx + s * 1.86 - 0.16, dx + s * 1.86 + 0.16, GRADE + 0.6, GRADE + 1.3, dz0 - 0.24, dz0 + 0.02, shade(P.dark, 1.15), 0.02);
          }
          m.beam(dx - 1.5, dx + 1.5, GRADE + 1.25, GRADE + 1.4, dz0 - 3.3, dz0 - 1.5, shade(P.steel, 1.08), 0.015);
        }
        const posts = d0.fine ? 8 : 3;
        for (let i = 0; i < posts; i += 1) {
          const px = o.x - b.w / 2 + (i * b.w) / (posts - 1);
          if (d0.fine) m.beam(px - 0.15, px + 0.15, GRADE, GRADE + 5.6, dz0 - 3.5, dz0 - 3.2, P.steel, 0.018);
          else m.bar(px - 0.15, px + 0.15, GRADE, GRADE + 5.6, dz0 - 3.5, dz0 - 3.2, P.steel);
        }
        m.save().move(0, GRADE + 5.6, 0);
        m.plate([[o.x - b.w / 2 - 0.6, dz0 - 4.4], [o.x + b.w / 2 + 0.6, dz0 - 4.4],
          [o.x + b.w / 2 + 0.6, dz0], [o.x - b.w / 2 - 0.6, dz0]], 0.22, shade(P.roof, 1.03));
        m.restore();
        if (d0.fine) {
          m.beam(o.x - b.w / 2 - 0.7, o.x + b.w / 2 + 0.7, GRADE + 5.6, GRADE + 5.92, dz0 - 4.52, dz0 - 4.2, shade(P.galv, 0.96), 0.02);
          groundPatch(m, o.x, dz0 - 8.0, b.w, 7.0, 12, 4, GRADE + 0.012, P.asphalt, 0.055);
        }
      });
    }
    if (stage >= 3) {
      // Sprinkler tank and pump house. The tank is the tallest round thing on a
      // distribution site and it goes up with the frame, because the hall
      // cannot be handed over without it.
      m.part("services / sprinkler tank and pump house", () => {
        const sx = o.x - b.w / 2 - 8.5, sz = o.z + b.dz * 0.22;
        m.column(sx, GRADE + 0.5, sz, built ? 9.5 : 4.0, 3.4, 3.4, d0.fine ? 14 : d0.seg, shade(P.galv, 0.94), true);
        if (d0.fine) {
          m.beam(sx - 3.9, sx + 3.9, GRADE, GRADE + 0.5, sz - 3.9, sz + 3.9, shade(P.concrete, 0.94), 0.03);
          if (built) {
            m.column(sx, GRADE + 10.0, sz, 0.6, 3.4, 2.4, 14, shade(P.galv, 1.04), true);
            for (let hp = 0; hp < 8; hp += 1) {
              const a = (hp / 8) * Math.PI * 2, a2 = ((hp + 1) / 8) * Math.PI * 2;
              m.rod([sx + Math.cos(a) * 3.3, GRADE + 10.0, sz + Math.sin(a) * 3.3],
                [sx + Math.cos(a) * 3.3, GRADE + 11.1, sz + Math.sin(a) * 3.3], 0.035, 4, P.galv, false);
              m.rod([sx + Math.cos(a) * 3.3, GRADE + 11.1, sz + Math.sin(a) * 3.3],
                [sx + Math.cos(a2) * 3.3, GRADE + 11.1, sz + Math.sin(a2) * 3.3], 0.035, 4, shade(P.galv, 1.04), false);
            }
            for (let r = 0; r < 12; r += 1) {
              m.rod([sx - 0.2, GRADE + 1.2 + r * 0.8, sz - 3.5], [sx + 0.2, GRADE + 1.2 + r * 0.8, sz - 3.5], 0.026, 6, P.galv, false);
            }
          }
        }
        if (!built) return;
        m.bar(sx - 2.6, sx + 2.6, GRADE, GRADE + 3.0, sz - 7.6, sz - 4.4, shade(P.clad, 1.0));
        if (!d0.fine) return;
        m.beam(sx - 2.8, sx + 2.8, GRADE + 3.0, GRADE + 3.24, sz - 7.8, sz - 4.2, P.roof, 0.025);
        m.bar(sx - 0.8, sx + 0.8, GRADE + 0.05, GRADE + 2.3, sz - 7.62, sz - 7.58, P.door);
        m.rod([sx, GRADE + 1.4, sz - 4.4], [sx, GRADE + 1.4, sz - 3.6], 0.22, 8, shade(P.safety, 0.96), true);
        m.rod([sx, GRADE + 1.4, sz - 3.6], [sx, GRADE + 1.4, sz - 3.0], 0.22, 8, shade(P.galv, 1.0), true);
      });
    }
    if (done) {
      m.part("yard / trailer standing", () => {
        const px = W * 0.26, pz = D * 0.34;
        m.save().move(0, GRADE, 0);
        m.plate([[px - 9, pz - 6], [px + 9, pz - 6], [px + 9, pz + 6], [px - 9, pz + 6]], 0.12, P.asphalt);
        m.restore();
        for (let i = 0; i < (d0.fine ? 7 : 3); i += 1) {
          const n = d0.fine ? 7 : 3;
          const lx = px - 8 + (i * 16) / (n - 1);
          m.bar(lx - 0.1, lx + 0.1, GRADE + 0.12, GRADE + 0.15, pz - 5.2, pz + 5.2, P.lane);
        }
      });
    }
    if (d0.fine) propRoadStrip(m, k, YARD.x, YARD.z, W, D, d0, built);
  }

  /// A robot and machine-cell RETROFIT installed in an existing facility. The
  /// roadmap is explicit that this is not a new standalone building, so the
  /// host shed stands finished at every stage and the staged work is the line
  /// going into it: floor broken out, machine bases cast, cell steel and guard
  /// posts up, robots and cabinets landed, line fenced and running. The only
  /// new structure is the low link annex butted onto the host's gable, which is
  /// where the line comes out of the building — and it is the smallest object
  /// on the pad, not the largest.
  function worksAutomation(m, k, c) {
    const { o, W, D, d0, stage } = c;
    const built = stage >= 4, done = stage >= 5;
    const b = k.block;
    propHost(m, k, o.x, o.z, W, D, d0, built);
    if (d0.fine && stage >= 3) genericBlock(m, k, o, d0);
    const lx = o.x + b.w / 2 + 12.5, lz = o.z - 5.5;    // inside the host's footprint
    if (stage <= 2) {
      m.part("site / floor broken out for machine bases", () => {
        // A retrofit's excavation is a saw-cut in a floor that is already
        // there, not a hole in a field. The cut edge, the broken slab and the
        // spoil beside it are the whole of it.
        m.bar(lx - 7, lx + 7, GRADE - 0.55, GRADE + 0.02, lz - 3.4, lz + 3.4, P.earthCut);
        if (!d0.fine) return;
        for (const s of [-1, 1]) {
          m.beam(lx - 7.2, lx + 7.2, GRADE - 0.06, GRADE + 0.06, lz + s * 3.5 - 0.14, lz + s * 3.5 + 0.14, shade(P.concrete, 1.02), 0.015);
        }
        groundPatch(m, lx, lz, 13.6, 6.6, 8, 4, GRADE - 0.02, P.earthCut, 0.08);
        for (let i = 0; i < 3; i += 1) {
          m.mound(lx - 4 + i * 4, GRADE, lz + 6.0, 1.5, 0.8, d0.fine ? 6 : 4, i % 2 ? P.hardcore : P.earth);
        }
      });
    }
    if (stage === 2) {
      m.part("earthworks / machine bases and isolation pads", () => {
        for (let i = 0; i < 3; i += 1) {
          const bx = lx - 4.4 + i * 4.4;
          m.bar(bx - 1.5, bx + 1.5, PIT, GRADE + 0.35, lz - 1.6, lz + 1.6, P.concreteWet);
          if (!d0.fine) continue;
          rebarCage(m, bx, lz, 0.9, 0.9, GRADE - 0.4, GRADE + 0.9, 6);
          // The isolation joint round a machine base. A machine that shares a
          // slab with the building it stands in transmits everything it does
          // into the building, which is why the joint is there.
          for (const s of [-1, 1]) {
            m.bar(bx - 1.62, bx + 1.62, GRADE + 0.02, GRADE + 0.35, lz + s * 1.66 - 0.06, lz + s * 1.66 + 0.06, shade(P.dark, 1.2));
          }
        }
      });
    }
    if (stage >= 3) {
      m.part("structure / machine cell line", () => {
        for (let i = 0; i < 3; i += 1) {
          const bx = lx - 4.4 + i * 4.4;
          m.bar(bx - 1.5, bx + 1.5, PIT, GRADE + 0.42, lz - 1.6, lz + 1.6, shade(P.concrete, 0.98));
          // Cell frame: four posts and a rail across the top, which is what a
          // machine cell is before anything is installed in it. A 110 mm post
          // is nothing at map range, so the coarse path keeps the head rail
          // only — three lines over three plinths, which is the read.
          if (d0.fine) {
            for (const sx of [-1, 1]) {
              for (const sz of [-1, 1]) {
                m.beam(bx + sx * 1.3 - 0.11, bx + sx * 1.3 + 0.11, GRADE + 0.42, GRADE + 3.2, lz + sz * 1.4 - 0.11, lz + sz * 1.4 + 0.11, P.primer, 0.014);
              }
            }
          }
          m.bar(bx - 1.45, bx + 1.45, GRADE + 3.2, GRADE + 3.4, lz - 1.5, lz + 1.5, shade(P.primer, 1.06));
          if (!built) continue;
          // The machine, and the robot serving it. A pedestal, a slewing base,
          // two links and a wrist: that is the shape of an articulated arm and
          // the reason it is the one object on this site nothing else looks
          // like.
          m.bar(bx - 1.1, bx + 1.1, GRADE + 0.42, GRADE + 2.2, lz - 1.1, lz + 1.1, shade(k.accent, 1.02));
          if (!d0.fine) continue;
          m.bar(bx - 0.85, bx + 0.85, GRADE + 1.3, GRADE + 2.0, lz + 1.1, lz + 1.14, P.dark);
          m.column(bx + 1.7, GRADE + 0.42, lz - 0.4, 0.55, 0.42, 0.42, 10, shade(P.safety, 0.94), true);
          m.column(bx + 1.7, GRADE + 0.97, lz - 0.4, 0.42, 0.34, 0.34, 10, shade(P.safety, 1.06), true);
          m.save().move(bx + 1.7, GRADE + 1.39, lz - 0.4).rotZ(-38);
          m.beam(-0.16, 0.16, 0, 1.5, -0.2, 0.2, shade(P.safety, 1.0), 0.012);
          m.save().move(0, 1.5, 0).rotZ(76);
          m.beam(-0.13, 0.13, 0, 1.25, -0.16, 0.16, shade(P.safety, 0.92), 0.012);
          m.column(0, 1.25, 0, 0.28, 0.16, 0.11, 8, shade(P.dark, 1.2), true);
          m.restore();
          m.restore();
        }
        if (!d0.fine) return;
        // The conveyor spine the cells sit along, and the transfer out through
        // the annex. A line of machines with nothing between them is a row of
        // machines, not a line.
        m.bar(lx - 7.4, lx + 6.8, GRADE + 0.85, GRADE + 1.05, lz - 2.7, lz - 2.1, shade(P.steel, 1.0));
        for (let i = 0; i < 9; i += 1) {
          const rx = lx - 7.0 + i * 1.7;
          m.rod([rx, GRADE + 1.12, lz - 2.7], [rx, GRADE + 1.12, lz - 2.1], 0.11, 8, shade(P.galv, 1.02), true);
          if (i % 2) continue;
          m.bar(rx - 0.09, rx + 0.09, GRADE + 0.42, GRADE + 0.85, lz - 2.5, lz - 2.3, shade(P.steel, 0.9));
        }
      });
    }
    if (stage >= 3) {
      m.part("services / guarding and interlocks", () => {
        guardFence(m, lx, lz, 15.5, 8.6, d0, 2.2);
        if (!d0.fine || !built) return;
        // Light curtain posts on the open bay and a stop button at the corner.
        for (const s of [-1, 1]) {
          m.beam(lx + s * 2.6 - 0.09, lx + s * 2.6 + 0.09, GRADE, GRADE + 1.9, lz - 4.4, lz - 4.22, shade(P.safety, 1.1), 0.012);
        }
        m.column(lx - 7.4, GRADE, lz - 4.6, 1.2, 0.09, 0.09, 8, P.galv, false);
        m.column(lx - 7.4, GRADE + 1.2, lz - 4.6, 0.22, 0.2, 0.2, 10, P.stopped, true);
      });
    }
    if (built) {
      m.part("services / control cabinets and cable tray", () => {
        cabinets(m, lx - 6.5, lz + 5.2, d0.fine ? 5 : 2, d0, shade(k.accent, 1.1));
      });
      m.part("services / compressed air and chiller skid", () => {
        const sx = o.x - b.w / 2 - 3.2, sz = o.z + 3.0;
        m.bar(sx - 2.4, sx + 2.4, GRADE + 0.25, GRADE + 2.3, sz - 1.5, sz + 1.5, shade(P.galv, 0.96));
        if (!d0.fine) return;
        m.beam(sx - 2.6, sx + 2.6, GRADE, GRADE + 0.25, sz - 1.7, sz + 1.7, shade(P.concrete, 0.94), 0.025);
        m.column(sx - 1.0, GRADE + 2.3, sz, 0.32, 0.62, 0.62, 12, shade(P.dark, 1.15), true);
        m.column(sx + 1.4, GRADE + 0.25, sz, 3.2, 0.62, 0.62, 12, shade(P.galv, 1.02), true);   // air receiver
        for (let f = 0; f < 4; f += 1) {
          m.bar(sx - 2.3 + f * 0.6, sx - 1.95 + f * 0.6, GRADE + 0.6, GRADE + 2.0, sz + 1.5, sz + 1.6, shade(P.steel, 0.9 + 0.06 * (f % 2)));
        }
        m.rod([sx + 1.4, GRADE + 3.0, sz], [sx + 1.4, GRADE + 3.4, sz], 0.1, 8, shade(P.galv, 1.04), true);
      });
    }
    if (done) {
      // HAND-OVER IS ITS OWN STATE, even for a retrofit. A commissioned line is
      // marked out on the floor it runs on, and the pallets of parts it feeds
      // on are standing beside it: without them the finished installation and
      // the one still being commissioned are the same picture.
      m.part("yard / line markings and part store", () => {
        for (let i = 0; i < (d0.fine ? 4 : 2); i += 1) {
          const n = d0.fine ? 4 : 2;
          const mx = lx - 7.0 + (i * 14) / (n - 1);
          m.bar(mx - 0.12, mx + 0.12, GRADE + 0.42, GRADE + 0.46, lz - 5.4, lz + 5.4, P.lane);
        }
        m.bar(lx - 7.6, lx + 7.6, GRADE + 0.42, GRADE + 0.46, lz - 5.5, lz - 5.2, shade(P.safety, 1.1));
        for (let i = 0; i < (d0.fine ? 3 : 1); i += 1) {
          const px = lx - 5.0 + i * 3.4;
          m.bar(px - 1.1, px + 1.1, GRADE + 0.42, GRADE + 1.5, lz + 7.0, lz + 8.4, shade(P.hardcore, 0.94 + 0.05 * i));
          if (!d0.fine) continue;
          m.bar(px - 1.16, px + 1.16, GRADE + 0.42, GRADE + 0.54, lz + 6.94, lz + 8.46, P.timber);
          m.bar(px - 0.06, px + 0.06, GRADE + 0.5, GRADE + 1.56, lz + 6.9, lz + 8.5, shade(P.dark, 1.3));
        }
      });
      if (d0.fine) propKiosk(m, k, YARD.x, YARD.z, W, D, d0, built);
    }
  }

  /// A heat-recovery, insulation and control RETROFIT in an existing facility.
  /// Same rule as automation and for the same reason: the host stands complete
  /// at every stage and the staged work is what is being fitted to it. The
  /// controls are a kiosk and a row of cabinets — the roadmap says not to
  /// pretend invisible software needs a giant external box, and it does not.
  /// The only new structure is a lean-to plant house on the host's gable.
  function worksEfficiency(m, k, c) {
    const { o, W, D, d0, stage } = c;
    const built = stage >= 4, done = stage >= 5;
    const b = k.block;
    propHost(m, k, o.x, o.z, W, D, d0, built);
    if (d0.fine && stage >= 3) genericBlock(m, k, o, d0);
    const hx = o.x + b.w / 2 + 12.5;                    // the host's centre line
    if (stage >= 2) {
      m.part("earthworks / skid plinth and trestle bases", () => {
        m.bar(o.x - 6.5, o.x - 1.0, PIT, GRADE + (stage >= 3 ? 0.45 : 0.2), o.z + 4.0, o.z + 8.4, P.concreteWet);
        if (stage === 2 && d0.fine) rebarCage(m, o.x - 3.7, o.z + 6.2, 1.9, 1.6, PIT + 0.1, GRADE + 1.3, 6);
        for (let i = 0; i < (d0.fine ? 4 : 2); i += 1) {
          const bx = o.x - 0.5 + (i * 11) / 3;
          // A 1.2 m pad has an arris worth 16 extra triangles at inspection
          // range and nothing at all at map range, where this kind is already
          // the closest of the thirteen to the ceiling.
          if (d0.fine) m.beam(bx - 0.6, bx + 0.6, PIT, GRADE + 0.3, o.z + 5.8, o.z + 7.0, shade(P.concrete, 0.94), 0.025);
          else m.bar(bx - 0.6, bx + 0.6, PIT, GRADE + 0.3, o.z + 5.8, o.z + 7.0, shade(P.concrete, 0.94));
        }
      });
    }
    if (stage >= 3) {
      // The heat recovery skid: a finned recovery block on a frame with a flue
      // duct spliced into the host's existing flue. This is the physical half
      // of an efficiency project and it is the only large object in it.
      m.part("services / heat recovery skid and economiser", () => {
        const sx = o.x - 3.7, sz = o.z + 6.2;
        m.bar(sx - 2.6, sx + 2.6, GRADE + 0.45, GRADE + (built ? 4.6 : 1.4), sz - 2.0, sz + 2.0, shade(P.galv, 0.95));
        if (!built) return;
        if (d0.fine) {
          // The fin block, drawn as fins. A heat exchanger with a smooth face
          // is a box, and the fin pitch is the only thing that says what it is
          // for.
          for (let f = 0; f < 12; f += 1) {
            m.bar(sx - 2.4 + f * 0.4, sx - 2.16 + f * 0.4, GRADE + 1.2, GRADE + 4.2, sz + 2.0, sz + 2.34, shade(P.steel, 0.88 + 0.08 * (f % 2)));
          }
          m.beam(sx - 2.8, sx + 2.8, GRADE + 4.6, GRADE + 4.86, sz - 2.2, sz + 2.5, shade(P.galv, 1.06), 0.02);
          m.bar(sx - 1.0, sx + 1.0, GRADE + 0.5, GRADE + 2.4, sz - 2.04, sz - 2.0, P.door);
        }
        // The flue duct up to the host's stack. A recovery unit that is not
        // connected to anything recovers nothing.
        m.column(sx + 1.6, GRADE + 4.6, sz, 5.4, 0.72, 0.72, d0.fine ? 12 : d0.seg, shade(P.galv, 1.0), false);
        conveyor(m, [sx + 1.6, GRADE + 10.2, sz], [hx - 6.0, GRADE + 8.2, o.z + 3.0], 1.5, d0);
        if (d0.fine) {
          for (const fy of [0.2, 2.6, 5.0]) {
            m.column(sx + 1.6, GRADE + 4.6 + fy, sz, 0.14, 0.86, 0.86, 12, shade(P.galv, 0.86), false);
          }
        }
      });
      m.part("services / insulated pipe run", () => {
        // Lagged flow and return on trestles, from the skid into the host. The
        // lagging is why the pipe is fat, and the fat pipe is what says the
        // circuit is hot.
        const y = GRADE + 2.4;
        for (let i = 0; i < 2; i += 1) {
          m.rod([o.x - 1.0, y + i * 0.75, o.z + 6.2], [hx - 6.5, y + i * 0.75, o.z + 6.2], 0.28, d0.fine ? 10 : 4,
            shade(P.hut, 1.0 - i * 0.06), true);
        }
        if (!d0.fine) return;
        for (let i = 0; i < 4; i += 1) {
          const bx = o.x - 0.5 + (i * 11) / 3;
          for (const yy of [y, y + 0.75]) {
            m.column(bx, yy - 0.34, o.z + 6.2, 0.12, 0.34, 0.34, 10, shade(P.steel, 0.94), false);
          }
          m.beam(bx - 0.16, bx + 0.16, GRADE + 0.3, y - 0.34, o.z + 6.04, o.z + 6.36, P.steel, 0.016);
        }
        for (let i = 0; i < 6; i += 1) {
          const px = o.x - 0.6 + (i * 11.6) / 5;
          m.column(px, y - 0.36, o.z + 6.2, 0.1, 0.33, 0.33, 10, shade(P.galv, 1.05), false);
        }
      });
    }
    if (stage >= 4) {
      // Over-cladding the host. This IS the insulation half of the project and
      // it is drawn where insulation actually goes: onto the walls of the
      // building that already exists, half done at the commissioning stage and
      // closed at hand-over. Nothing here is a new building.
      m.part("envelope / wall cladding (over-clad to host)", () => {
        const clad = CLAD_BY_STAGE[stage - 1];
        for (const s of [-1, 1]) {
          m.save().move(hx, GRADE, o.z + s * 8.25).rotY(s > 0 ? 0 : 180);
          claddedWall(m, 22, 7.0, 9, shade(k.accent, 1.12), d0, { clad, phase: s, pitch: 0.9 });
          m.restore();
        }
        if (!d0.fine) return;
        for (const s of [-1, 1]) {
          m.beam(hx - 11.3, hx + 11.3, GRADE + 7.0, GRADE + 7.26, o.z + s * 8.35 - 0.16, o.z + s * 8.35 + 0.16, shade(P.galv, 1.04), 0.02);
        }
      });
      m.part("envelope / roof insulation boards", () => {
        // Boards stacked on the roof and part laid, which is what the job looks
        // like on the day it is being done and the reason the stack shrinks
        // rather than the building growing.
        const laid = built && done ? 1 : 0.5;
        m.save().move(hx, GRADE, o.z);
        const n = d0.fine ? 6 : 2;
        for (let i = 0; i < Math.max(1, Math.round(n * laid)); i += 1) {
          const z0 = -8 + (i * 16) / n;
          m.quad([-10.6, 7.02, z0 + 0.2], [0, 8.82, z0 + 0.2], [0, 8.82, z0 + 16 / n - 0.2], [-10.6, 7.02, z0 + 16 / n - 0.2],
            shade(k.accent, 1.02 + 0.04 * (i % 2)));
        }
        m.restore();
        if (!d0.fine) return;
        for (let i = 0; i < 3; i += 1) {
          m.bar(hx + 3.4, hx + 8.6, GRADE + 8.0 + i * 0.16, GRADE + 8.14 + i * 0.16, o.z - 2.4 + i * 0.2, o.z + 2.0 + i * 0.2,
            shade(k.accent, 0.94 + 0.05 * i));
        }
      });
    }
    if (built) {
      m.part("services / controls kiosk and panel line", () => {
        cabinets(m, o.x + 2.0, o.z - 6.4, d0.fine ? 4 : 2, d0, shade(P.clad, 1.02));
      });
      m.part("services / thermal buffer vessel", () => {
        vessel(m, o.x - 8.5, o.z + 6.2, GRADE + 0.5, 7.0, 1.4, d0, shade(P.hut, 0.98));
        if (!d0.fine) return;
        for (let i = 0; i < 5; i += 1) {
          m.column(o.x - 8.5, GRADE + 1.2 + i * 1.3, o.z + 6.2, 0.1, 1.48, 1.48, 12, shade(P.hut, 0.88), false);
        }
      });
    }
    if (done) {
      // The metering and valve station: two isolating valves, a meter set and
      // the insulated header between them. An efficiency project is handed over
      // ON its metering — that is how anyone knows what it did — and until it
      // is there the circuit is not finished, which is the difference between
      // this stage and the one before it.
      m.part("services / metering and valve station", () => {
        const vx = o.x + 4.4, vz = o.z + 6.2, y = GRADE + 2.4;
        m.rod([vx - 2.2, y, vz], [vx + 2.2, y, vz], 0.3, d0.fine ? 10 : 4, shade(P.hut, 1.02), true);
        for (const s2 of [-1, 1]) {
          m.column(vx + s2 * 1.3, y + 0.3, vz, 0.55, 0.2, 0.2, d0.fine ? 10 : d0.seg, shade(P.safety, 1.0), true);
          if (!d0.fine) continue;
          m.rod([vx + s2 * 1.3, y + 0.85, vz - 0.28], [vx + s2 * 1.3, y + 0.85, vz + 0.28], 0.34, 10, shade(P.safety, 0.9), true);
          m.column(vx + s2 * 1.3, y - 0.42, vz, 0.16, 0.36, 0.36, 10, shade(P.galv, 0.9), false);
        }
        if (!d0.fine) return;
        m.bar(vx - 0.5, vx + 0.5, y - 1.5, y - 0.5, vz + 0.32, vz + 0.36, shade(P.clad, 1.06));
        m.beam(vx - 0.62, vx + 0.62, y - 1.62, y - 0.38, vz + 0.28, vz + 0.44, shade(P.galv, 1.0), 0.015);
        m.bar(vx - 0.34, vx + 0.34, y - 1.24, y - 0.86, vz + 0.36, vz + 0.38, P.dark);
        m.beam(vx - 0.2, vx + 0.2, GRADE, y - 1.62, vz + 0.28, vz + 0.44, P.steel, 0.016);
      });
      if (d0.fine) propKiosk(m, k, YARD.x, YARD.z, W, D, d0, built);
    }
  }

  /// A smaller workshop and light-industry compound. Everything here is one
  /// size down: a single-span workshop with an open lean-to on its flank, a
  /// blockwork store with a mono-pitch roof, an office pod at the gate, and an
  /// open yard with a stock rack and a compressor. The point of this kind is
  /// that it is SMALL, so it gets fewer objects and smaller ones rather than
  /// the same compound at a different colour.
  function worksStarterIndustry(m, k, c) {
    const { o, W, D, d0, stage } = c;
    const built = stage >= 4, done = stage >= 5;
    const b = k.block;
    if (d0.fine && stage >= 3) genericBlock(m, k, o, d0);
    if (stage >= 3) {
      // The blockwork store. A light-industry compound is not all sheeted
      // steel: the small permanent store is laid in blockwork with a mono-pitch
      // roof, and the course lines are the only thing on this site that says
      // masonry.
      m.part("structure / blockwork store", () => {
        const sx = o.x + b.w / 2 + 7.5, sz = o.z + 1.0, h0 = 3.0, h1 = 4.2;
        m.bar(sx - 4.5, sx + 4.5, GRADE, GRADE + h0, sz - 3.2, sz + 3.2, shade(P.hardcore, 1.0));
        m.webPlate([[sx - 4.5, GRADE + h0], [sx + 4.5, GRADE + h0], [sx + 4.5, GRADE + h1]], sz - 3.2, sz + 3.2, shade(P.hardcore, 0.96));
        // The mono-pitch sheet, one plane from the low eaves to the high one.
        m.quad([sx - 4.7, GRADE + h0 + 0.1, sz - 3.4], [sx + 4.7, GRADE + h1 + 0.1, sz - 3.4],
          [sx + 4.7, GRADE + h1 + 0.1, sz + 3.4], [sx - 4.7, GRADE + h0 + 0.1, sz + 3.4], shade(P.roof, 1.04));
        if (!d0.fine) return;
        for (let i = 0; i < 12; i += 1) {
          m.bar(sx - 4.52, sx + 4.52, GRADE + 0.22 + i * 0.22, GRADE + 0.26 + i * 0.22, sz + 3.2, sz + 3.23, shade(P.hardcore, 0.86));
        }
        m.bar(sx - 1.3, sx + 1.3, GRADE + 0.05, GRADE + 2.3, sz + 3.2, sz + 3.26, P.door);
        m.beam(sx - 1.44, sx + 1.44, GRADE, GRADE + 2.44, sz + 3.2, sz + 3.34, shade(P.galv, 0.98), 0.015);
        m.bar(sx + 2.0, sx + 3.4, GRADE + 1.6, GRADE + 2.5, sz + 3.2, sz + 3.24, P.glass);
        m.beam(sx - 4.8, sx + 4.8, GRADE + h1 + 0.02, GRADE + h1 + 0.2, sz - 3.5, sz - 3.3, shade(P.roof, 0.86), 0.02);
      });
    }
    if (built) {
      m.part("structure / office pod and canopy", () => {
        const px = o.x - b.w / 2 - 6.0, pz = o.z - b.dz * 0.2;
        m.bar(px - 3.4, px + 3.4, GRADE + 0.15, GRADE + 3.0, pz - 2.4, pz + 2.4, shade(P.hut, 1.02));
        if (!d0.fine) return;
        m.beam(px - 3.6, px + 3.6, GRADE + 3.0, GRADE + 3.22, pz - 2.6, pz + 2.6, P.roof, 0.02);
        m.beam(px - 3.5, px + 3.5, GRADE, GRADE + 0.15, pz - 2.5, pz + 2.5, shade(P.concrete, 0.94), 0.02);
        for (const wx of [-2.4, -0.6]) {
          m.bar(px + wx, px + wx + 1.4, GRADE + 1.3, GRADE + 2.4, pz - 2.44, pz - 2.4, P.glass);
          m.beam(px + wx - 0.12, px + wx + 1.52, GRADE + 1.2, GRADE + 2.5, pz - 2.5, pz - 2.38, shade(P.hut, 1.08), 0.014);
        }
        m.bar(px + 1.6, px + 2.6, GRADE + 0.2, GRADE + 2.3, pz - 2.44, pz - 2.4, P.door);
        m.beam(px + 0.6, px + 3.6, GRADE + 3.22, GRADE + 3.4, pz - 4.2, pz - 2.3, shade(k.accent, 1.1), 0.02);
        for (const cx of [1.0, 3.2]) m.rod([px + cx, GRADE + 3.22, pz - 4.0], [px + cx, GRADE + 0.15, pz - 4.0], 0.09, 8, P.galv, false);
      });
      m.part("yard / open stock rack and compressor", () => {
        const rx = o.x + b.w * 0.1, rz = o.z + b.dz / 2 + 9.5;
        for (let i = 0; i < (d0.fine ? 4 : 2); i += 1) {
          const n = d0.fine ? 4 : 2;
          const px = rx - 4.5 + (i * 9) / (n - 1);
          m.bar(px - 0.11, px + 0.11, GRADE, GRADE + 3.2, rz - 0.11, rz + 0.11, shade(P.safety, 0.9));
          if (!d0.fine) continue;
          for (let a = 0; a < 4; a += 1) {
            m.rod([px, GRADE + 0.8 + a * 0.7, rz], [px, GRADE + 0.8 + a * 0.7, rz + 1.5], 0.075, 6, shade(P.safety, 1.02), false);
          }
        }
        if (!d0.fine) return;
        for (let a = 0; a < 3; a += 1) {
          for (let i = 0; i < 3; i += 1) {
            m.rod([rx - 4.7, GRADE + 0.86 + a * 0.7, rz + 0.35 + i * 0.4], [rx + 4.7, GRADE + 0.86 + a * 0.7, rz + 0.35 + i * 0.4],
              0.14, 8, shade(P.steel, 0.92 + 0.05 * i), false);
          }
        }
        m.bar(rx + 6.4, rx + 8.4, GRADE + 0.2, GRADE + 1.7, rz - 1.0, rz + 1.0, shade(P.safety, 0.94));
        m.beam(rx + 6.2, rx + 8.6, GRADE, GRADE + 0.2, rz - 1.2, rz + 1.2, shade(P.concrete, 0.94), 0.02);
        m.rod([rx + 6.6, GRADE + 2.0, rz], [rx + 8.2, GRADE + 2.0, rz], 0.42, 10, shade(P.galv, 1.0), true);
        for (const wz of [rz - 0.9, rz + 0.9]) {
          m.rod([rx + 6.9, GRADE + 0.35, wz], [rx + 7.05, GRADE + 0.35, wz], 0.35, 8, P.dark, true);
          m.rod([rx + 7.9, GRADE + 0.35, wz], [rx + 8.05, GRADE + 0.35, wz], 0.35, 8, P.dark, true);
        }
      });
    }
    if (done) propLeanTo(m, k, o.x, o.z, W, D, d0, built);
    if (stage >= 3 && d0.fine) propPipeStack(m, k, YARD.x, YARD.z, W, D, d0, built);
  }

  /// Service economy: the low shared-services hall uses the same site kit as
  /// the original buildings; the offices have storeys, strip glazing and an
  /// entrance forecourt. These are representative spaces, not measured assets.
  function worksOfficeDistrict(m, k, c) {
    const { o, W, d0, stage } = c;
    if (d0.fine && stage >= 3) genericBlock(m, k, o, d0);
    const x = W * 0.12, z = o.z + 1.0;
    if (d0.fine) {
      deckedBlock(m, { x, z, deck: GRADE + 0.18, col: k.body, tag: "office floors",
        block: { w: 24, dz: 15, eaves: 13.2, floors: 3 } }, stage, d0);
    } else if (stage >= 3) {
      m.part("structure / office floor silhouette", () => {
        if (stage === 3) {
          for (const dx of [-10, 0, 10]) m.bar(x + dx - 0.3, x + dx + 0.3, GRADE, GRADE + 13.2, z - 7.5, z + 7.5, P.primer);
        } else {
          m.bar(x - 12, x + 12, GRADE, GRADE + 13.2, z - 7.5, z + 7.5, k.body);
          for (let floor = 0; floor < 3; floor += 1) m.bar(x - 11.6, x + 11.6, GRADE + floor * 4.4 + 1.6, GRADE + floor * 4.4 + 3.5, z + 7.5, z + 7.6, P.glass);
        }
      });
    }
    if (stage >= 4) {
      m.part("services / office air handling terrace", () => {
        for (const dx of [-5, 5]) {
          m.bar(x + dx - 2.0, x + dx + 2.0, GRADE + 13.4, GRADE + 15.1, z - 2, z + 2, P.galv);
          if (d0.fine) {
            for (let i = 0; i < 8; i += 1) m.beam(x + dx - 1.8, x + dx + 1.8, GRADE + 13.6 + i * 0.16, GRADE + 13.65 + i * 0.16, z + 2, z + 2.14, P.dark, 0.01);
            m.rod([x + dx, GRADE + 14.1, z - 2], [x + dx, GRADE + 14.1, z - 4.0], 0.38, 12, P.galv, true);
          }
        }
      });
      m.part("envelope / office entrance canopy", () => {
        m.bar(x - 4, x + 4, GRADE + 3.5, GRADE + 3.8, z + 7.5, z + 11.5, k.accent);
        for (const dx of [-3.5, 3.5]) m.column(x + dx, GRADE, z + 11, 3.5, 0.12, 0.12, d0.fine ? 12 : 4, P.galv, false);
        if (d0.fine) m.bar(x - 2.2, x + 2.2, GRADE, GRADE + 3.1, z + 7.61, z + 7.75, P.glass);
      });
    }
    if (stage >= 5) m.part("yard / office pedestrian forecourt", () => {
      m.bar(x - 13, x + 13, GRADE, GRADE + 0.15, z + 9, z + 15, P.kerb);
      for (const dx of [-10, 10]) {
        m.bar(x + dx - 1.5, x + dx + 1.5, GRADE + 0.15, GRADE + 0.6, z + 10.5, z + 13.5, P.concrete);
        m.mound(x + dx, GRADE + 0.6, z + 12, 1.4, 0.7, d0.fine ? 10 : 4, P.grass);
      }
    });
  }

  /// No vessel is drawn: the project creates a place to build and service
  /// ships, while actual ships remain equipment owned by the simulation.
  function worksShipyard(m, k, c) {
    const { o, W, d0, stage } = c;
    if (d0.fine && stage >= 3) genericBlock(m, k, o, d0);
    // The dock occupies the clear centre of the compound. Shared stored units
    // stay on the eastern hardstand, clear of this basin and its repair access.
    const x = W * 0.02, z = 1.0;
    if (stage >= 2) m.part("dock / dry dock base and retaining walls", () => {
      m.bar(x - 6, x + 6, GRADE + 0.02, GRADE + 0.10, z - 14, z + 14, shade(P.concrete, 0.66));
      for (const dx of [-6.6, 6.6]) m.bar(x + dx - 0.6, x + dx + 0.6, GRADE, GRADE + (stage >= 3 ? 1.9 : 0.4), z - 14, z + 14, P.concrete);
      if (d0.fine && stage >= 3) {
        for (let i = 0; i < 10; i += 1) m.beam(x - 1.0, x + 1.0, GRADE + 0.10, GRADE + 0.7, z - 12 + i * 2.5, z - 11.4 + i * 2.5, P.timber, 0.025);
      }
    });
    if (stage >= 3) m.part("structure / shipyard lifting gantry", () => {
      for (const dx of [-8, 8]) {
        m.bar(x + dx - 0.32, x + dx + 0.32, GRADE + 0.12, GRADE + 0.30, z - 15, z + 15, P.steel);
        m.bar(x + dx - 0.55, x + dx + 0.55, GRADE + 0.3, GRADE + 20.0, z - 1.0, z + 1.0, k.accent);
        if (d0.fine) {
          for (const dz of [-1.5, 1.5]) m.rod([x + dx, GRADE + 0.7, z + dz - 0.2], [x + dx, GRADE + 0.7, z + dz + 0.2], 0.6, 12, P.dark, true);
          m.rod([x + dx, GRADE + 3, z], [x + dx * 0.65, GRADE + 19, z], 0.16, 8, P.steel, false);
        }
      }
      m.bar(x - 9.3, x + 9.3, GRADE + 19.0, GRADE + 21.0, z - 1.1, z + 1.1, k.accent);
      m.bar(x - 1.5, x + 1.5, GRADE + 18.0, GRADE + 19.0, z - 1.4, z + 1.4, P.steel);
      if (d0.fine) {
        for (const dx of [-0.45, 0.45]) m.rod([x + dx, GRADE + 18, z], [x + dx, GRADE + 10, z], 0.05, 6, P.dark, false);
        m.beam(x - 0.8, x + 0.8, GRADE + 9.4, GRADE + 10.2, z - 0.4, z + 0.4, P.safety, 0.03);
      }
    });
    if (stage >= 4) m.part("services / dock pump house and shore cabinets", () => {
      m.bar(x + 9.5, x + 14.5, GRADE, GRADE + 3.2, z + 7, z + 13, k.body);
      for (let i = 0; i < (d0.fine ? 4 : 1); i += 1) m.bar(x + 9.5, x + 10.6, GRADE + 0.1, GRADE + 1.8, z - 10 + i * 3, z - 9 + i * 3, P.galv);
      if (d0.fine) m.rod([x + 11, GRADE + 1.2, z + 7], [x + 11, GRADE + 1.2, z - 12], 0.15, 10, P.galv, true);
    });
    if (stage >= 5) m.part("dock / caisson gate and quay bollards", () => {
      m.bar(x - 6, x + 6, GRADE + 0.1, GRADE + 1.8, z + 13.4, z + 14.0, P.steel);
      for (const dx of [-7.2, 7.2]) for (let i = 0; i < (d0.fine ? 5 : 2); i += 1) {
        const bz = z - 11 + i * (d0.fine ? 5.5 : 22);
        m.column(x + dx, GRADE + 1.9, bz, 0.8, 0.25, 0.32, d0.fine ? 12 : 4, P.dark, true);
      }
    });
  }

  /// A controlled production environment: air handling, process supplies and
  /// an enclosed material entrance distinguish it from the ordinary factory.
  function worksAdvancedIndustry(m, k, c) {
    const { o, d0, stage } = c;
    if (d0.fine && stage >= 3) genericBlock(m, k, o, d0);
    const x = o.x + 20, z = o.z;
    if (stage >= 3) m.part("structure / controlled process service gallery", () => {
      for (const dz of [-6, 0, 6]) {
        m.bar(x - 2.8, x + 2.8, GRADE + 4.5, GRADE + 4.8, z + dz - 0.2, z + dz + 0.2, P.steel);
        for (const dx of [-2.6, 2.6]) m.bar(x + dx - 0.16, x + dx + 0.16, GRADE, GRADE + 4.8, z + dz - 0.16, z + dz + 0.16, P.steel);
      }
      if (d0.fine) {
        for (const dx of [-1.4, 0, 1.4]) m.rod([x + dx, GRADE + 4.9, z - 7], [x + dx, GRADE + 4.9, z + 7], 0.17, 10, P.galv, true);
      }
    });
    if (stage >= 4) {
      m.part("services / filtered air supply bank", () => {
        for (let i = 0; i < 3; i += 1) {
          const fx = o.x - 8 + i * 8;
          m.bar(fx - 2.6, fx + 2.6, GRADE + k.block.ridge + 0.15, GRADE + k.block.ridge + 2.0, o.z - 3, o.z + 3, P.galv);
          if (d0.fine) for (let s = 0; s < 9; s += 1) m.beam(fx - 2.3, fx + 2.3, GRADE + k.block.ridge + 0.3 + s * 0.17, GRADE + k.block.ridge + 0.36 + s * 0.17, o.z + 3, o.z + 3.18, P.dark, 0.01);
        }
      });
      m.part("services / enclosed process supply cabinets", () => {
        for (const dz of [-4.5, 0, 4.5]) {
          m.bar(x - 2.0, x + 2.0, GRADE + 0.15, GRADE + 3.3, z + dz - 1.5, z + dz + 1.5, shade(k.body, 0.94));
          if (d0.fine) {
            m.beam(x + 2.0, x + 2.1, GRADE + 0.45, GRADE + 3.0, z + dz - 1.2, z + dz + 1.2, P.door, 0.015);
            m.column(x - 0.7, GRADE + 0.2, z + dz, 2.5, 0.4, 0.4, 12, P.galv, true);
          }
        }
      });
    }
    if (stage >= 5) m.part("envelope / clean loading vestibule", () => {
      const front = o.z + k.block.dz / 2;
      m.bar(o.x - 4.5, o.x + 4.5, GRADE, GRADE + 4.4, front + 0.1, front + 5.0, k.body);
      m.bar(o.x - 2.0, o.x + 2.0, GRADE + 0.2, GRADE + 3.8, front + 5.0, front + 5.1, P.door);
      if (d0.fine) {
        for (const dx of [-2.2, 2.2]) m.beam(o.x + dx - 0.15, o.x + dx + 0.15, GRADE, GRADE + 4.0, front + 5.1, front + 5.3, P.galv, 0.02);
        m.bar(o.x - 4.8, o.x + 4.8, GRADE + 4.4, GRADE + 4.6, front, front + 5.3, P.roof);
      }
    });
  }

  // The dispatch. One entry per key in PROJECT_KINDS except `arms_plant`, which
  // has its own builder above and is not composed from this table.
  const KIND_WORKS = {
    infrastructure: worksInfrastructure,
    civilian_industry: worksCivilianIndustry,
    power_grid: worksPowerGrid,
    research_center: worksResearchCenter,
    machinery_works: worksMachineryWorks,
    generation: worksGeneration,
    processing_plant: worksProcessingPlant,
    freight_terminal: worksFreightTerminal,
    warehouse: worksWarehouse,
    automation: worksAutomation,
    efficiency: worksEfficiency,
    starter_industry: worksStarterIndustry,
    office_district: worksOfficeDistrict,
    shipyard: worksShipyard,
    advanced_industry: worksAdvancedIndustry,
  };

  /// The plain shed, staged. Everything the twelve share.
  function genericBlock(m, k, o, d0) {
    hall(m, o, d0);
    if (o.clad > 0.9) {
      m.part("envelope / roller doors", () => {
        m.save().move(o.x, o.deck, o.z + o.block.dz / 2 + 0.02);
        for (let i = 0; i < 2; i += 1) {
          const cx = (i - 0.5) * (o.block.w / 2.6), dw = o.block.w / 5.2, dh = o.block.eaves * 0.55;
          m.bar(cx - dw / 2, cx + dw / 2, 0.05, dh, 0, 0.12, P.door);
          if (d0.fine) for (let s = 0; s < 4; s += 1) {
            const y = 0.2 + (s * (dh - 0.4)) / 4;
            m.bar(cx - dw / 2 + 0.06, cx + dw / 2 - 0.06, y, y + 0.2, 0.12, 0.17, shade(P.door, 1.09));
          }
        }
        m.restore();
      });
    }
  }

  /// Hand-over. Everything the site kit was standing in for gets replaced by the
  /// thing it was standing in for: marked bays instead of a churned apron, kerbs
  /// and verge instead of spoil, roof lights and louvres instead of open bays,
  /// yard lighting instead of site lighting. It is also where most of the
  /// completed model's triangles are, which is the right place for them — the
  /// finished plant is what a player looks at for the rest of the campaign,
  /// while a stage is looked at for a few dozen days.
  function finishedPlant(m, k, o, W, D, d0) {
    const b = o.block;
    m.part("envelope / roof lights", () => {
      const pitch = Math.atan2(b.ridge - b.eaves, b.w / 2) / DEG;
      for (const s of [-1, 1]) {
        m.save().move(o.x, o.deck, o.z).rotY(s > 0 ? 0 : 180).move(0, 0, 0);
        for (let i = 0; i < (d0.fine ? 8 : 3); i += 1) {
          const n = d0.fine ? 8 : 3;
          const zz = -b.dz / 2 + 1.6 + (i * (b.dz - 3.2)) / (n - 1);
          m.save().move(-b.w * 0.28, b.eaves + (b.ridge - b.eaves) * 0.44, zz).rotZ(-pitch);
          if (d0.fine) {
            // Kerb, glazing on top of it, and glazing bars across. A rooflight
            // that is a flat bright patch reads as a stain; the upstand is what
            // makes it a hole in the roof with something in it.
            m.beam(-2.5, 2.5, 0.0, 0.16, -0.8, 0.8, shade(P.roof, 0.86), 0.02);
            m.bar(-2.4, 2.4, 0.16, 0.22, -0.7, 0.7, shade(P.glass, 1.35));
            for (let g = 1; g < 3; g += 1) {
              m.bar(-2.4 + (g * 4.8) / 3 - 0.05, -2.4 + (g * 4.8) / 3 + 0.05, 0.2, 0.27, -0.7, 0.7, shade(P.galv, 0.94));
            }
          } else {
            m.bar(-2.4, 2.4, 0.02, 0.2, -0.7, 0.7, shade(P.glass, 1.35));
          }
          m.restore();
        }
        m.restore();
      }
    });
    m.part("envelope / wall louvres and vents", () => {
      for (const s of [-1, 1]) {
        for (let i = 0; i < (d0.fine ? 8 : 3); i += 1) {
          const n = d0.fine ? 8 : 3;
          const lx = o.x - b.w * 0.36 + (i * b.w * 0.72) / (n - 1);
          const y0 = o.deck + b.eaves - 2.4, y1 = o.deck + b.eaves - 1.0;
          if (d0.fine) {
            m.save().move(0, 0, o.z + s * (b.dz / 2));
            if (s < 0) m.scale(1, 1, -1);
            louvreBank(m, lx - 0.9, lx + 0.9, y0, y1, 0.0, 0.16, 4, shade(P.galv, 0.98));
            m.restore();
          } else {
            m.bar(lx - 0.9, lx + 0.9, y0, y1, o.z + s * (b.dz / 2) - 0.02, o.z + s * (b.dz / 2) + 0.18, P.dark);
          }
        }
      }
    });
    m.part("services / roof plant and access walkway", () => {
      // Air handling, extract cowls and a condenser bank on a frame over the
      // ridge, with a walkway and a handrail to reach them. This is the one
      // thing an industrial building always has on top of it and the model
      // never had, and the silhouette against the sky is where it is read.
      const ry = o.deck + b.ridge;
      const units = d0.fine ? 3 : 1;
      for (let i = 0; i < units; i += 1) {
        const zz = o.z - b.dz * 0.22 + (i * b.dz * 0.44) / Math.max(1, units - 1 || 1);
        m.beam(o.x - 2.6, o.x + 2.6, ry + 0.34, ry + 0.5, zz - 1.5, zz + 1.5, shade(P.steel, 0.9), 0.02);
        m.beam(o.x - 2.3, o.x + 2.3, ry + 0.5, ry + 2.05, zz - 1.3, zz + 1.3, shade(P.galv, 0.98), 0.03);
        if (!d0.fine) continue;
        for (const cx of [-1.2, 1.2]) {
          m.column(o.x + cx, ry + 2.05, zz, 0.5, 0.62, 0.62, d0.seg, shade(P.galv, 1.04), false);
          m.column(o.x + cx, ry + 2.55, zz, 0.26, 0.72, 0.5, d0.seg, shade(P.dark, 1.2), true);
        }
        for (let f = 0; f < 5; f += 1) {
          m.bar(o.x - 2.26, o.x + 2.26, ry + 0.7 + f * 0.24, ry + 0.86 + f * 0.24, zz + 1.3, zz + 1.36, shade(P.dark, 1.1));
        }
        for (const px of [-2.4, 2.4]) {
          m.rod([o.x + px, ry + 0.1, zz - 1.4], [o.x + px, ry + 0.42, zz - 1.4], 0.07, 6, P.steel, false);
          m.rod([o.x + px, ry + 0.1, zz + 1.4], [o.x + px, ry + 0.42, zz + 1.4], 0.07, 6, P.steel, false);
        }
      }
      if (d0.fine) {
        m.bar(o.x - 0.6, o.x + 0.6, ry + 0.3, ry + 0.36, o.z - b.dz * 0.34, o.z + b.dz * 0.34, shade(P.galv, 0.92));
        for (const rs of [-1, 1]) {
          m.rod([o.x + rs * 0.6, ry + 0.36, o.z - b.dz * 0.34], [o.x + rs * 0.6, ry + 1.36, o.z - b.dz * 0.34], 0.035, 6, P.galv, false);
          m.rod([o.x + rs * 0.6, ry + 0.36, o.z + b.dz * 0.34], [o.x + rs * 0.6, ry + 1.36, o.z + b.dz * 0.34], 0.035, 6, P.galv, false);
          m.rod([o.x + rs * 0.6, ry + 1.36, o.z - b.dz * 0.34], [o.x + rs * 0.6, ry + 1.36, o.z + b.dz * 0.34], 0.035, 6, P.galv, false);
        }
      }
    });
    m.part("envelope / entrance canopy and reception", () => {
      const ex = o.x - b.w * 0.36, ez = o.z - b.dz / 2;
      for (const px of [ex - 2.4, ex + 2.4]) {
        if (d0.fine) m.rod([px, GRADE, ez - 3.06], [px, GRADE + 3.4, ez - 3.06], 0.15, 8, P.galv, false);
        else m.bar(px - 0.14, px + 0.14, GRADE, GRADE + 3.4, ez - 3.2, ez - 2.92, P.galv);
      }
      m.save().move(0, GRADE + 3.4, 0);
      m.plate([[ex - 3.0, ez - 3.6], [ex + 3.0, ez - 3.6], [ex + 3.0, ez + 0.2], [ex - 3.0, ez + 0.2]], 0.18, shade(P.roof, 1.06));
      m.restore();
      m.bar(ex - 1.1, ex + 1.1, o.deck + 0.05, o.deck + 2.4, ez - 0.06, ez + 0.02, P.glass);
      if (d0.fine) {
        // Mullions, a transom and a fascia. Curtain walling is a grid of frames
        // with glass in it, and the grid is the only thing that gives a glazed
        // elevation any scale at all.
        for (let i = 0; i < 4; i += 1) {
          const gx = ex - 3.0 + i * 1.6;
          m.bar(gx - 0.55, gx + 0.55, o.deck + 2.7, o.deck + 4.2, ez - 0.06, ez + 0.02, P.glass);
          m.beam(gx + 0.55, gx + 0.68, o.deck + 2.62, o.deck + 4.28, ez - 0.1, ez + 0.06, shade(P.galv, 1.0), 0.015);
        }
        m.beam(ex - 3.14, ex + 3.14, o.deck + 2.56, o.deck + 2.7, ez - 0.1, ez + 0.06, shade(P.galv, 0.9), 0.015);
        for (const mx of [ex - 1.14, ex, ex + 1.14]) {
          m.beam(mx - 0.05, mx + 0.05, o.deck + 0.05, o.deck + 2.44, ez - 0.1, ez + 0.06, shade(P.galv, 1.02), 0.012);
        }
        m.beam(ex - 3.1, ex + 3.1, GRADE + 3.58, GRADE + 3.8, ez - 3.7, ez + 0.3, shade(k.accent, 1.1), 0.025);
        // Steps up to the door, because the deck stands above the yard and a
        // door opening onto a 180 mm drop is a detail somebody forgot.
        for (let s = 0; s < 3; s += 1) {
          m.beam(ex - 1.6, ex + 1.6, GRADE + (s * (o.deck - GRADE)) / 3, GRADE + ((s + 1) * (o.deck - GRADE)) / 3,
            ez - 0.36 - (2 - s) * 0.32, ez, shade(P.concrete, 0.94 + 0.04 * s), 0.02);
        }
      }
    });
    m.part("yard / marked bays and kerbs", () => {
      const bays = d0.fine ? 10 : 3, bx = -W * 0.42, bz = -D * 0.30;
      for (let i = 0; i <= bays; i += 1) {
        const lz = bz + (i * 20) / bays;
        m.bar(bx, bx + 7.5, GRADE + 0.1, GRADE + 0.13, lz - 0.09, lz + 0.09, P.lane);
      }
      for (let i = 0; i < (d0.fine ? 6 : 2); i += 1) {
        if (d0.fine) m.beam(-W * 0.46 + i * 3, -W * 0.46 + i * 3 + 2.4, GRADE + 0.1, GRADE + 0.32, -D * 0.40 - 0.2, -D * 0.40 + 0.2, P.kerb, 0.022);
        else m.bar(-W * 0.46 + i * 3, -W * 0.46 + i * 3 + 2.4, GRADE + 0.1, GRADE + 0.32, -D * 0.40 - 0.2, -D * 0.40 + 0.2, P.kerb);
      }
      m.bar(-W * 0.46, W * 0.46, GRADE + 0.1, GRADE + 0.32, -D * 0.455, -D * 0.435, P.kerb);
      if (d0.fine) {
        // Hardstanding that can be told apart from the platform it is laid on,
        // and the gullies it drains to. Restrained: the tone spread is six per
        // cent, which is wear rather than camouflage.
        groundPatch(m, 0, D * 0.06, W * 0.86, D * 0.28, 12, 6, GRADE + 0.02, P.asphalt, 0.06);
        for (let g = 0; g < 4; g += 1) {
          const gx = -W * 0.32 + (g * W * 0.64) / 3;
          m.bar(gx - 0.32, gx + 0.32, GRADE + 0.02, GRADE + 0.06, D * 0.06 - 0.32, D * 0.06 + 0.32, shade(P.dark, 1.2));
        }
      }
    });
    m.part("yard / parked vehicles and stored units", () => {
      // A yard parks what fits in it. The row used to be four vehicles at fixed
      // 9.2 m centres whatever the compound was, which ran the last one seven
      // metres off the end of the hardstanding on the smaller kinds — and an
      // articulated unit is eleven metres long, so 9.2 m centres had them
      // inside one another as well. Count and spacing now come off the pad.
      const lorries = d0.fine ? Math.max(2, Math.min(4, Math.floor((D - 8) / 11))) : 1;
      for (let i = 0; i < lorries; i += 1) {
        // 0.40, not 0.415: at 0.415 a 4.4 m wide vehicle hung 0.6 m over the
        // edge of the fill on the smallest compound on the list.
        const tx = -W * 0.40, tz = (i - (lorries - 1) / 2) * 11 + 0.7;
        m.bar(tx - 2.2, tx + 2.2, GRADE + 1.1, GRADE + 3.6, tz - 6.0, tz + 2.2, shade(P.clad, 1.02));
        m.bar(tx - 1.9, tx + 1.9, GRADE + 0.55, GRADE + 1.1, tz - 5.4, tz - 3.6, P.dark);
        m.bar(tx - 1.9, tx + 1.9, GRADE, GRADE + 0.6, tz + 0.4, tz + 2.0, P.dark);
        if (!d0.fine) continue;
        // A box on two blocks is not a lorry. Chassis, landing legs, a tandem
        // bogie on wheels that are round, a tractor unit with a cab and a rear
        // marker board: the parts a viewer counts without knowing they are
        // counting them.
        m.beam(tx - 1.7, tx + 1.7, GRADE + 0.95, GRADE + 1.12, tz - 5.8, tz + 2.0, shade(P.dark, 1.2), 0.02);
        for (const lz of [tz - 3.2, tz - 2.7]) {
          for (const ls of [-1, 1]) {
            m.beam(tx + ls * 1.6, tx + ls * 1.86, GRADE + 0.2, GRADE + 0.98, lz - 0.12, lz + 0.12, shade(P.steel, 0.94), 0.015);
          }
        }
        for (const wz of [tz - 5.1, tz - 4.0]) {
          for (const ws of [-1, 1]) {
            m.rod([tx + ws * 1.7, GRADE + 0.52, wz], [tx + ws * 1.96, GRADE + 0.52, wz], 0.52, 8, P.dark, true);
            m.rod([tx + ws * 1.76, GRADE + 0.52, wz], [tx + ws * 1.9, GRADE + 0.52, wz], 0.27, 6, shade(P.steel, 1.0), true);
          }
        }
        m.bar(tx - 1.9, tx + 1.9, GRADE + 1.1, GRADE + 3.0, tz + 2.2, tz + 4.6, shade(k.accent, 1.06));
        m.bar(tx - 1.72, tx + 1.72, GRADE + 2.0, GRADE + 2.9, tz + 4.6, tz + 4.66, P.glass);
        for (const wz of [tz + 2.8, tz + 4.2]) {
          for (const ws of [-1, 1]) {
            m.rod([tx + ws * 1.58, GRADE + 0.5, wz], [tx + ws * 1.86, GRADE + 0.5, wz], 0.5, 8, P.dark, true);
          }
        }
        m.bar(tx - 2.0, tx + 2.0, GRADE + 3.4, GRADE + 3.62, tz - 6.12, tz - 5.98, shade(P.safety, 1.1));
      }
      // Six stored units, not eight. The last pair sat behind the delivered
      // wings where nothing can see them, and at 260 close triangles each they
      // were the cheapest 520 triangles in the file to give back to the twelve
      // compositions that needed them.
      for (let i = 0; i < (d0.fine ? 6 : 2); i += 1) {
        // Same reason as the vehicles: at D*0.30 the back row of stored units
        // stood three metres past the edge of the fill on a small compound.
        const cx = W * 0.32 + (i % 2) * 6.4, cz = D * 0.18 + Math.floor(i / 2) * 3.0;
        m.bar(cx - 3.0, cx + 3.0, GRADE + (d0.fine ? 0.14 : 0), GRADE + 2.5, cz - 1.2, cz + 1.2, shade(k.accent, 0.84 + 0.1 * (i % 3)));
        if (!d0.fine) continue;
        // Corner castings, corrugation and a base rail. A stored unit is a
        // pressed steel box and reads as one from thirty metres.
        for (const ex2 of [-3.0, 3.0]) {
          for (const ez2 of [-1.2, 1.2]) {
            m.bar(cx + ex2 - 0.16, cx + ex2 + 0.16, GRADE + 0.14, GRADE + 2.5, cz + ez2 - 0.16, cz + ez2 + 0.16, shade(P.steel, 1.0));
          }
        }
        for (let r = 0; r < 5; r += 1) {
          const rx = cx - 2.2 + r * 1.1;
          for (const fz of [cz - 1.26, cz + 1.2]) {
            m.bar(rx - 0.08, rx + 0.08, GRADE + 0.24, GRADE + 2.42, fz, fz + 0.06, shade(k.accent, 0.78));
          }
        }
        m.beam(cx - 3.05, cx + 3.05, GRADE, GRADE + 0.16, cz - 1.24, cz + 1.24, shade(P.steel, 0.86), 0.02);
      }
    });
    m.part("yard / hardstanding markings and walkway", () => yardMarkings(m, W, D, d0));
    m.part("yard / bunded store and gas cage", () => bundedStore(m, W * 0.30, -D * 0.02, d0, k));
    m.part("yard / bollards and gate island", () => {
      for (let i = 0; i < (d0.fine ? 10 : 3); i += 1) {
        const n = d0.fine ? 10 : 3;
        const bx = -W * 0.42 + (i * W * 0.84) / (n - 1);
        if (d0.fine) bollard(m, bx, -D * 0.415, 1.05, 0.2, 8, P.safety);
        else m.column(bx, GRADE, -D * 0.415, 1.0, 0.2, 0.2, d0.seg, P.safety, true);
      }
    });
    m.part("yard / weighbridge and external storage racks", () => {
      const wx = -W * 0.47, wz = D * 0.10;
      if (d0.fine) {
        // A weighbridge is a plate deck in a pit with a kerb round it and a
        // ticket kiosk beside it. The plate joint down the middle is the one
        // thing that says "deck" instead of "patch of yard".
        m.beam(wx, wx + 9, GRADE + 0.08, GRADE + 0.26, wz, wz + 3.6, shade(P.steel, 1.02), 0.02);
        m.bar(wx + 4.4, wx + 4.6, GRADE + 0.24, GRADE + 0.27, wz, wz + 3.6, shade(P.dark, 1.2));
        for (const kz of [wz - 0.22, wz + 3.6]) m.beam(wx - 0.2, wx + 9.2, GRADE + 0.06, GRADE + 0.3, kz, kz + 0.22, P.kerb, 0.02);
        m.bar(wx + 9.6, wx + 11.6, GRADE, GRADE + 2.7, wz + 0.6, wz + 2.4, shade(P.clad, 1.02));
        m.beam(wx + 9.4, wx + 11.8, GRADE + 2.7, GRADE + 2.9, wz + 0.4, wz + 2.6, P.roof, 0.02);
        m.bar(wx + 9.56, wx + 11.5, GRADE + 1.3, GRADE + 2.2, wz + 0.56, wz + 0.6, P.glass);
      } else {
        m.bar(wx, wx + 9, GRADE + 0.08, GRADE + 0.24, wz, wz + 3.6, shade(P.steel, 1.02));
      }
      for (const s of [-1, 1]) {
        if (d0.fine) bollard(m, wx + 4.5, wz + 1.8 + s * 2.6, 1.45, 0.18, 8, P.safety);
        else m.column(wx + 4.5, GRADE, wz + 1.8 + s * 2.6, 1.4, 0.18, 0.18, d0.seg, P.safety, true);
      }
      const uprights = d0.fine ? 5 : 3;
      for (let i = 0; i < uprights; i += 1) {
        const rx = -W * 0.16 + (i * 11) / (uprights - 1);
        for (const rz of [D * 0.42, D * 0.42 + 2.4]) {
          if (d0.fine) m.beam(rx - 0.12, rx + 0.12, GRADE, GRADE + 4.4, rz - 0.12, rz + 0.12, shade(P.safety, 0.9), 0.016);
          else m.bar(rx - 0.12, rx + 0.12, GRADE, GRADE + 4.4, rz - 0.12, rz + 0.12, shade(P.safety, 0.9));
        }
        // Frame bracing between the two uprights of a rack frame. Without it a
        // pallet rack is a row of poles.
        if (d0.fine) {
          for (let b = 0; b < 3; b += 1) {
            m.rod([rx, GRADE + 0.5 + b * 1.3, D * 0.42], [rx, GRADE + 1.8 + b * 1.3, D * 0.42 + 2.4], 0.05, 6, shade(P.safety, 0.82), false);
          }
          m.bar(rx - 0.2, rx + 0.2, GRADE, GRADE + 0.1, D * 0.42 - 0.2, D * 0.42 + 2.6, shade(P.steel, 0.9));
        }
      }
      for (let i = 0; i < (d0.fine ? 3 : 1); i += 1) {
        const y = GRADE + 1.3 + i * 1.5;
        if (d0.fine) {
          for (const rz of [D * 0.42 - 0.2, D * 0.42 + 2.4]) {
            m.beam(-W * 0.16, -W * 0.16 + 11, y, y + 0.16, rz, rz + 0.2, shade(P.steel, 1.04), 0.016);
          }
          // Pallets on the beams, banded, at slightly different depths.
          for (let p = 0; p < 3; p += 1) {
            const px = -W * 0.16 + 0.7 + p * 3.5, off = 0.1 * ((p + i) % 3);
            m.bar(px, px + 2.4, y + 0.16, y + 0.28, D * 0.42 - 0.1 + off, D * 0.42 + 2.3 + off, shade(P.timber, 0.9));
            m.bar(px + 0.1, px + 2.3, y + 0.28, y + 1.05, D * 0.42 + off, D * 0.42 + 2.2 + off, shade(P.hardcore, 0.95 + 0.05 * p));
            m.bar(px + 0.9, px + 1.02, y + 0.26, y + 1.1, D * 0.42 - 0.06 + off, D * 0.42 + 2.26 + off, shade(P.dark, 1.3));
          }
        } else {
          m.bar(-W * 0.16, -W * 0.16 + 11, y, y + 0.16, D * 0.42 - 0.2, D * 0.42 + 2.6, shade(P.steel, 1.04));
        }
      }
    });
  }

  // -------------------------------------------------------------- assembly
  /// The site around whatever is being built. Presence is per stage and the
  /// pose is per status; neither is per anything that elapses.
  function siteWorks(m, k, o, W, D, st, d0, h) {
    const stage = st.index + 1;
    const working = st.active;
    const b = k.block;
    m.part("site / perimeter hoarding and gate", () => hoarding(m, W, D, d0, P.hoard));
    m.part("site / accommodation and welfare", () => {
      siteHuts(m, -W * 0.42, -D * 0.20, 8 + (h % 5) * 4, stage >= 3 ? 4 : 2, d0);
    });
    m.part("site / stacked materials", () => {
      const kinds = stage === 1 ? [2, 3, 1] : stage === 2 ? [3, 2, 1] : stage === 3 ? [0, 3, 1] : [0, 1];
      kinds.forEach((kind, i) => {
        materialStack(m, W * 0.40 - i * 0.4, -D * 0.26 + i * 7.5, kind, d0);
      });
    });
    // Wheel wash, rumble strip and the cone line off the gate. Small, permanent
    // for the life of the job, and the reason a site does not put mud on the
    // province's road: worth the eighty triangles because it is the piece that
    // says these gates are used.
    m.part("site / wheel wash and traffic control", () => {
      const gz = -D / 2 + 6.0;
      m.bar(-4.4, 4.4, GRADE, GRADE + 0.22, gz - 3.0, gz + 3.0, P.concreteWet);
      for (let i = 0; i < 6; i += 1) {
        const rz = gz - 2.4 + i * 0.95;
        m.bar(-4.2, 4.2, GRADE + 0.22, GRADE + 0.34, rz - 0.2, rz + 0.2, shade(P.steel, 0.95));
      }
      for (let i = 0; i < (d0.fine ? 8 : 3); i += 1) {
        const cx = -6.5 + (i * 13) / ((d0.fine ? 8 : 3) - 1);
        m.column(cx, GRADE, gz + 4.6, 0.75, 0.36, 0.1, d0.fine ? 6 : 4, P.safety, true);
      }
    });
    // Waste segregation. Three skips for the life of the job, and gone at
    // hand-over with everything else that belongs to the job rather than to the
    // facility. A site with no skips on it is a site nobody is working.
    m.part("site / skips and waste segregation", () => skipLine(m, -W * 0.40, -D * 0.04, d0));
    if (stage <= 2) {
      // GROUNDWORKS, which is what an early site should gain and the first pass
      // gave it none of: the drainage going in before anything stands on top of
      // it, and the plant that puts it there. Both are gone by the frame stage,
      // which is when they would actually leave.
      m.part("earthworks / drainage runs and manholes", () => {
        drainage(m, -W * 0.30, W * 0.34, serviceRunZ(o, b, D), d0);
      });
      m.part("plant / earthmoving plant", () => {
        const tz = serviceRunZ(o, b, D);
        const pz = Math.max(tz + 3.4, Math.min(tz + 5.2, o.z - (b.dz + 3.2) / 2 - 3.0));
        excavator(m, -W * 0.26, pz, 6, working, d0);
        dumper(m, W * 0.24, pz - 1.2, 196, d0);
      });
      m.part("earthworks / topsoil bund", () => {
        // Stripped topsoil, stockpiled along the boundary. It goes back as the
        // verge at hand-over, which is why the finished site has one and the
        // stages in between do not.
        for (let i = 0; i < (d0.fine ? 8 : 4); i += 1) {
          const n = d0.fine ? 8 : 4;
          m.mound(-W * 0.42 + (i * W * 0.84) / (n - 1), GRADE, D * 0.44, 2.6, 1.5, d0.fine ? 6 : 4, P.earth);
        }
      });
      m.part("earthworks / spoil heaps", () => {
        for (let i = 0; i < (stage === 1 ? 2 : 4); i += 1) {
          const a = ((i * 97 + h) % 360) * DEG;
          m.mound(o.x + (b.w / 2 + 5.0) * Math.cos(a), GRADE, o.z + (b.dz / 2 + 5.0) * Math.sin(a),
            3.4 + (i % 3) * 0.7, 1.9 + (i % 2) * 0.6, d0.fine ? 7 : 5,
            i % 2 ? P.earth : P.earthCut);
        }
      });
    }
    if (stage === 2) m.part("plant / batching skid and bowser", () => sitePlant(m, -W * 0.42, D * 0.30, d0));
    if (stage >= 3 && stage <= 4) {
      m.part("plant / tower crane", () => {
        towerCrane(m, -W * 0.42, -D * 0.06, 26, 22, working, d0, (h % 8) * 45 + (working ? 0 : 20));
      });
    }
    if (stage === 4) m.part("plant / gable scaffold", () => scaffold(m, o.x, o.z + b.dz / 2 + 1.4, b.w * 0.55, b.eaves, d0));
    m.part("services / lighting masts", () => {
      const n = stage >= 4 ? 4 : 2;
      for (let i = 0; i < n; i += 1) {
        const sx = i % 2 ? 1 : -1, sz = i < 2 ? 1 : -1;
        lightMast(m, sx * W * 0.40, sz * D * 0.38, 11, d0);
      }
    });
    if (!working) {
      m.part("site / work stopped notice", () => stopBoard(m, 0, -D / 2 + 3.2, st.status === "blocked", d0));
    }
  }

  const CLAD_BY_STAGE = [0, 0, 0, 0.62, 1];

  /// The compound at a given province level, and where the lead block sits on
  /// it. Both builders read this, because a near mesh and a far mesh that
  /// disagree about where the building is are two different buildings.
  ///
  /// WHY THE PAD GROWS FASTER THAN IT USED TO. A level upgrade delivers a wing,
  /// and four of them plus the works have to fit INSIDE the fence: at four
  /// metres a level the wings ran off the end of the fill by level three and
  /// hung in the air, which the checks could not see because the wings were
  /// themselves what made the bounding box grow. Five by four a level puts them
  /// on the ground, `wingSeat` places them where there is room, and the growth
  /// stops there because the perimeter is metred: every extra metre of fence is
  /// four hundred close triangles and the fence is already the single most
  /// expensive object on a finished site.
  function compound(k, level) {
    return { W: k.pad[0] + (level - 1) * 5, D: k.pad[1] + (level - 1) * 4 };
  }
  function homeSeat(k, W, D) {
    const home = k.home || [-0.06, 0.10];
    return { x: W * home[0], z: D * home[1] };
  }
  /// Delivered wings. Four of them at level five, and they have to be ON THE
  /// FILL: the old seat marched them off the lead block's flank at thirteen
  /// metres a level, which put wing four sixty metres out and hanging in the
  /// air over nothing. The check could not see it, because the wings were
  /// themselves the thing making the bounding box grow.
  ///
  /// They now stand in the clear strip across the front of the compound,
  /// between the apron and the works, two to a side and paired outwards from
  /// the centre line. Every seat is a fraction of the pad, so the row spreads
  /// as the compound grows instead of walking off the edge of it, and every
  /// composition keeps that strip clear. `l` counts from one.
  function wingSeat(k, o, W, D, l) {
    const side = l <= 2 ? 1 : -1, out = (l - 1) % 2;
    return { x: side * (W * 0.16 + out * 11.5), z: -D * 0.24 };
  }

  function buildNear(m, k, st, level, variant) {
    const d0 = { fine: true, ribs: true, seg: 12 };
    const h = (keyHash(k.name) + variant * 17) % 360;
    const { W, D } = compound(k, level);
    const stage = st.index + 1;
    const b = k.block;
    const seat = homeSeat(k, W, D);
    const o = { x: seat.x, z: seat.z, block: b, col: k.body, deck: GRADE + 0.18, clad: CLAD_BY_STAGE[st.index] };

    m.part("site / formation platform", () => {
      platform(m, W, D, stage <= 2 ? { x: o.x, z: o.z, w: b.w + 3.2, d: b.dz + 3.2 } : null, P.fill);
    });
    if (stage <= 2) {
      m.part("earthworks / excavation", () => {
        excavation(m, o.x, o.z, b.w + 3.2, b.dz + 3.2, stage === 1 ? 0.35 : 1.1, stage === 2 && d0.fine ? 5 : 0);
      });
      if (stage === 2) {
        m.part("earthworks / pad footings, reinforcement and formwork", () => {
          footings(m, o.x, o.z, b.w, b.dz, (k.grid || [4, 3])[0], (k.grid || [4, 3])[1], d0);
        });
        m.part("earthworks / blinding and formed slab edge", () => {
          m.bar(o.x - b.w / 2 - 1, o.x + b.w / 2 + 1, PIT, PIT + 0.12, o.z - b.dz / 2 - 1, o.z - b.dz / 2 + 3, P.concreteWet);
          m.bar(o.x - b.w / 2 - 1, o.x - b.w / 2 + 3, PIT, PIT + 0.12, o.z - b.dz / 2 - 1, o.z + b.dz / 2 + 1, P.concreteWet);
        });
      }
    } else {
      m.part("structure / ground slab", () => slab(m, o.x, o.z, b.w + 2.2, b.dz + 2.2, P.concrete, d0));
    }
    if (stage === 1) {
      m.part("site / setting out and access", () => {
        // Profiles, not pegs. Setting out is a pair of posts with a board
        // across them and a line pulled between boards, and the line is the
        // thing the building is actually built to — it is the one part of this
        // stage that says what is about to be here.
        for (let i = 0; i < 10; i += 1) {
          const px = o.x - b.w / 2 - 3 + (i * (b.w + 6)) / 9;
          for (const s of [-1, 1]) {
            const pz = o.z + s * (b.dz / 2 + 3);
            m.bar(px - 0.08, px + 0.08, GRADE, GRADE + 1.0, pz - 0.08, pz + 0.08, P.safety);
            m.bar(px - 0.55, px + 0.55, GRADE + 0.78, GRADE + 0.95, pz - 0.04, pz + 0.04, P.timber);
            if (i) {
              const qx = o.x - b.w / 2 - 3 + ((i - 1) * (b.w + 6)) / 9;
              m.bar(qx, px, GRADE + 0.95, GRADE + 0.98, pz - 0.015, pz + 0.015, shade(P.hut, 1.1));
            }
          }
        }
        m.save().move(0, GRADE, 0);
        m.plate([[-W * 0.44, -D * 0.42], [W * 0.1, -D * 0.42], [W * 0.1, -D * 0.30], [-W * 0.44, -D * 0.30]], 0.1, P.hardcore);
        m.restore();
        // The haul route churned into the fill, and the site board at the gate.
        groundPatch(m, -W * 0.17, -D * 0.36, W * 0.5, D * 0.11, 10, 3, GRADE + 0.105, P.hardcore, 0.09);
        for (const sx of [-W * 0.30, -W * 0.20]) m.rod([sx, GRADE, -D / 2 + 5.2], [sx, GRADE + 2.5, -D / 2 + 5.2], 0.07, 6, P.timber, false);
        m.webPlate([[-W * 0.31, GRADE + 1.2], [-W * 0.19, GRADE + 1.2], [-W * 0.19, GRADE + 2.4], [-W * 0.31, GRADE + 2.4]],
          -D / 2 + 5.14, -D / 2 + 5.2, shade(P.hoard, 1.1));
      });
    }

    // Levels already delivered stand finished beside the works, because
    // MAX_PROVINCE_LEVEL is five and five identical buildings is not what a
    // level upgrade is (roadmap section E). Each completed level is a wing.
    for (let l = 1; l < level; l += 1) {
      const at = wingSeat(k, o, W, D, l);
      const wing = {
        x: at.x,
        z: at.z,
        block: {
          w: 9,
          dz: 8,
          eaves: Math.max(4.2, Math.min(7.5, b.eaves * 0.62)),
          ridge: Math.max(5.0, Math.min(8.6, b.ridge * 0.60)),
          bays: 4, ends: 4,
        },
        col: shade(k.body, 0.96),
        deck: GRADE + 0.18,
        clad: 1,
        frame: false,
        tag: `wing ${l}`,
      };
      m.part(`structure / delivered wing ${l}`, () => {
        m.bar(wing.x - wing.block.w / 2 - 1, wing.x + wing.block.w / 2 + 1, PIT, GRADE + 0.18,
          wing.z - wing.block.dz / 2 - 1, wing.z + wing.block.dz / 2 + 1, P.concrete);
      });
      hall(m, wing, d0);
    }

    if (k.bespoke) {
      if (stage >= 3) hall(m, o, d0);
      if (stage >= 5) hallDoors(m, o, d0);
      if (stage >= 4) loadingArea(m, o, d0);
      if (stage >= 4) utilities(m, W * 0.40, D * 0.22, d0, stage >= 5);
      if (stage >= 5) {
        testYard(m, W * 0.34, -D * 0.24, 18, 14, d0);
        gatehouse(m, -W * 0.28, -D / 2 + 4.5, d0);
      }
    } else {
      // The kind's own composition. It draws its own buildings (including the
      // lead one) because for most of the twelve the lead block is a part of
      // the facility rather than the whole of it, and only the composition
      // knows what else is standing beside it at this stage.
      KIND_WORKS[k.key](m, k, { o, W, D, d0, stage, h, level });
    }
    // The services hung off whatever is standing. Stage-gated in one place, so
    // no composition can decide on its own that pipework arrives before the
    // steel it is bracketed to.
    buildingServices(m, k, o, W, D, stage, d0);

    if (stage >= 5) {
      m.part("yard / permanent perimeter", () => fence(m, W, D, d0));
      m.part("yard / apron and verge", () => {
        m.save().move(0, GRADE, 0);
        m.plate([[-W * 0.44, -D * 0.44], [W * 0.44, -D * 0.44], [W * 0.44, -D * 0.28], [-W * 0.44, -D * 0.28]], 0.1, P.asphalt);
        m.restore();
        // A verge is a low mound of reinstated topsoil, not a row of pointed
        // cones: wide, 0.55 m proud, at overlapping centres so it reads as one
        // strip of planting along the boundary rather than a dozen objects. The
        // radius is tied to the spacing so the strip stays closed at every
        // province level, and pulled in from the platform edge so it does not
        // hang off the fill — a verge floating past the hardstanding is the
        // detail that gives a model away.
        for (let i = 0; i < (d0.fine ? 16 : 5); i += 1) {
          const n = d0.fine ? 16 : 5;
          m.mound(-W * 0.46 + (i * W * 0.92) / (n - 1), GRADE, -D * 0.45,
            ((W * 0.92) / (n - 1)) * 0.56, 0.55, d0.fine ? 7 : 4, shade(P.grass, 0.96 + 0.06 * (i % 3)));
        }
      });
      m.part("yard / signage and lighting", () => {
        // A totem at the gate: two posts on base plates, a framed board between
        // them, and a low plinth. The accent colour appears here and almost
        // nowhere else, which is the point — a factory should read as a factory
        // and not as a pavilion.
        m.bar(-W * 0.36, -W * 0.36 + 5.4, GRADE + 1.6, GRADE + 3.0, -D / 2 + 2.0, -D / 2 + 2.16, shade(k.accent, 1.12));
        if (d0.fine) {
          m.beam(-W * 0.36 - 0.16, -W * 0.36 + 5.56, GRADE + 1.44, GRADE + 3.16, -D / 2 + 1.96, -D / 2 + 2.2, shade(P.galv, 1.02), 0.02);
          m.beam(-W * 0.36 + 0.2, -W * 0.36 + 5.2, GRADE + 1.5, GRADE + 1.62, -D / 2 + 1.94, -D / 2 + 2.22, shade(P.galv, 0.86), 0.015);
          m.beam(-W * 0.36 - 0.4, -W * 0.36 + 5.8, GRADE, GRADE + 0.34, -D / 2 + 1.7, -D / 2 + 2.5, shade(P.concrete, 0.94), 0.025);
          for (const sx of [-W * 0.36 + 0.3, -W * 0.36 + 5.1]) {
            m.bar(sx - 0.26, sx + 0.26, GRADE + 0.34, GRADE + 0.44, -D / 2 + 1.82, -D / 2 + 2.34, shade(P.galv, 0.88));
          }
        }
        for (const sx of [-W * 0.36 + 0.3, -W * 0.36 + 5.1]) m.bar(sx - 0.12, sx + 0.12, GRADE, GRADE + 3.1, -D / 2 + 1.9, -D / 2 + 2.26, P.galv);
        for (let i = 0; i < 6; i += 1) {
          lightMast(m, (i % 2 ? 1 : -1) * W * 0.40, (i < 2 ? 1 : i < 4 ? -1 : 0) * D * 0.30, 11, d0);
        }
      });
      finishedPlant(m, k, o, W, D, d0);
    } else {
      siteWorks(m, k, o, W, D, st, d0, h);
    }
    return { W, D };
  }

  /// Map view. A separate coarse path rather than the same one with the detail
  /// turned down: at 100..800 triangles the question is only "what is this and
  /// how far along is it", and the answer is massing plus the two silhouettes
  /// that carry at that size — the crane and the stack.
  function buildFar(m, k, st, level, variant) {
    const d0 = { fine: false, ribs: false, seg: 4 };
    const h = (keyHash(k.name) + variant * 17) % 360;
    const { W, D } = compound(k, level);
    const stage = st.index + 1;
    const b = k.block;
    const seat = homeSeat(k, W, D);
    const x = seat.x, z = seat.z;
    const o = { x, z, block: b, col: k.body, deck: GRADE + 0.18, clad: CLAD_BY_STAGE[st.index] };

    m.part("site / formation platform", () => platform(m, W, D, null, P.fill));
    m.part("site / perimeter", () => {
      if (stage >= 5) fence(m, W, D, d0); else hoarding(m, W, D, d0, P.hoard);
    });
    if (stage <= 2) {
      m.part("earthworks / dig and spoil", () => {
        // 20 mm PROUD of the platform deck, not flush with it. The coarse
        // platform is drawn without a hole (the hole costs triangles this budget
        // does not have), so a dig whose top face sits exactly at GRADE is
        // coplanar with the deck and z-fights across the whole excavation at
        // every map zoom. Two centimetres settles it and costs nothing.
        m.bar(x - b.w / 2, x + b.w / 2, GRADE - 0.4, GRADE + 0.02, z - b.dz / 2, z + b.dz / 2, P.earthCut);
        for (let i = 0; i < (stage === 1 ? 2 : 4); i += 1) {
          const a = ((i * 97 + h) % 360) * DEG;
          m.mound(W * 0.30 * Math.cos(a) - W * 0.06, GRADE, D * 0.34 * Math.sin(a), 3.4, 2.0, 5, P.earth);
        }
      });
      if (stage === 2) {
        m.part("earthworks / footings", () => {
          for (let i = 0; i < 6; i += 1) {
            const fx = x + ((i % 3) - 1) * (b.w / 3), fz = z + (i < 3 ? -1 : 1) * (b.dz / 4);
            m.bar(fx - 1.4, fx + 1.4, GRADE - 0.4, GRADE + 0.3, fz - 1.4, fz + 1.4, P.concreteWet);
          }
        });
      }
    } else {
      m.part("structure / slab", () => slab(m, x, z, b.w + 2, b.dz + 2, P.concrete));
    }
    if (stage === 3) {
      m.part("structure / frame", () => {
        for (let i = 0; i < 3; i += 1) {
          const fz = z - b.dz / 2 + (i * b.dz) / 2;
          for (const s of [-1, 1]) m.bar(x + s * b.w / 2 - 0.4, x + s * b.w / 2 + 0.4, GRADE, GRADE + b.eaves, fz - 0.4, fz + 0.4, P.primer);
          m.bar(x - b.w / 2, x + b.w / 2, GRADE + b.eaves, GRADE + b.eaves + 0.5, fz - 0.35, fz + 0.35, P.primer);
        }
        m.bar(x - 0.35, x + 0.35, GRADE + b.ridge - 0.5, GRADE + b.ridge, z - b.dz / 2, z + b.dz / 2, P.primer);
      });
    }
    if (stage >= 4) {
      m.part("envelope / massing", () => {
        m.bar(x - b.w / 2, x + b.w / 2, GRADE, GRADE + b.eaves, z - b.dz / 2, z + b.dz / 2,
          stage >= 5 ? k.body : shade(k.body, 0.9));
        m.save().move(x, GRADE, z);
        roof(m, b.w, b.dz, b.eaves, b.ridge, 1, P.roof, d0, { clad: stage >= 5 ? 1 : 0.62 });
        // The gable above the eaves is a hole in a box-and-roof massing, and at
        // map scale a hole reads as a hole. Two triangles close it.
        for (const s of [-1, 1]) {
          m.save().rotY(s > 0 ? 0 : 180);
          m.tri([-b.w / 2, b.eaves, b.dz / 2], [b.w / 2, b.eaves, b.dz / 2], [0, b.ridge, b.dz / 2],
            shade(stage >= 5 ? k.body : shade(k.body, 0.9), 1.04));
          m.restore();
        }
        m.restore();
      });
      if (stage === 4) {
        // The unclad end is the whole story of this stage at map scale: an
        // outline of frame where the sheeting has not reached, and scaffold
        // against it. Without them stage four reads as a finished building.
        m.part("structure / open bays and scaffold", () => {
          for (const s of [-1, 1]) {
            m.bar(x + s * b.w / 2 - 0.4, x + s * b.w / 2 + 0.4, GRADE, GRADE + b.eaves + (b.ridge - b.eaves) * 0.3, z - b.dz / 2, z - b.dz / 2 + 0.8, P.primer);
          }
          m.bar(x - b.w / 2, x + b.w / 2, GRADE + b.eaves * 0.55, GRADE + b.eaves * 0.55 + 0.3, z - b.dz / 2 - 0.35, z - b.dz / 2 + 0.05, P.primer);
          scaffold(m, x, z + b.dz / 2 + 0.6, b.w * 0.6, b.eaves, d0);
          // No second material stack here. `site / huts and materials` already
          // puts one on the map mesh, and a duplicate cost 36 triangles at the
          // one stage that could least afford them: a stopped site adds a stop
          // board at four and nowhere else, so stage four is the stage that
          // decides whether the finished site can still be the biggest mesh.
        });
      }
    }
    for (let l = 1; l < level; l += 1) {
      m.part(`structure / delivered wing ${l}`, () => {
        const at = wingSeat(k, o, W, D, l);
        m.bar(at.x - 4.5, at.x + 4.5, GRADE, GRADE + Math.max(4.2, Math.min(7.5, b.eaves * 0.62)),
          at.z - 4, at.z + 4, shade(k.body, 0.96));
      });
    }
    // The kind's own composition, coarse. It names its own parts, so it is
    // called at the top level: a part opened inside another one would silently
    // break the contiguous ranges the picker indexes by. At map scale most of
    // a composition draws nothing at all and what is left is the two or three
    // objects that still read at a hundred metres.
    if (!k.bespoke) KIND_WORKS[k.key](m, k, { o, W, D, d0, stage, h, level });
    if (stage >= 3 && stage <= 4) {
      m.part("plant / tower crane", () => towerCrane(m, -W * 0.42, -D * 0.06, 26, 22, st.active, d0, (h % 8) * 45));
    }
    if (stage < 5) {
      m.part("site / huts and materials", () => {
        // One cabin at map range. A stacked pair is 5.4 m of silhouette against
        // a 70 m compound and the second one adds nothing you can see.
        siteHuts(m, -W * 0.34, D * 0.3, 8 + (h % 5) * 4, 1, d0);
        materialStack(m, W * 0.30, -D * 0.30, stage <= 2 ? 2 : 0, d0);
      });
      if (!st.active) m.part("site / work stopped notice", () => stopBoard(m, 0, -D / 2 + 3.2, st.status === "blocked", d0));
    } else {
      m.part("yard / apron", () => {
        m.save().move(0, GRADE, 0);
        m.plate([[-W * 0.44, -D * 0.44], [W * 0.44, -D * 0.44], [W * 0.44, -D * 0.28], [-W * 0.44, -D * 0.28]], 0.1, P.asphalt);
        m.restore();
      });
      if (k.bespoke) {
        m.part("yard / loading dock and stack", () => {
          m.bar(x - 11, x + 11, GRADE, GRADE + 1.3, z - b.dz / 2 - 12, z - b.dz / 2 - 6, P.concrete);
          m.column(W * 0.28, GRADE, D * 0.26, 17, 0.95, 0.75, 4, shade(P.clad, 0.88), false);
        });
      }
      // The fixed hand-over dressing. Every kind gets the same, so a completed
      // site never comes out cheaper than the stage before it just because its
      // props happen to be plain.
      // THE HAND-OVER DRESSING IS NOT A GARNISH, it is the reason a finished
      // site is not cheaper to draw than the building site it replaced. Strip
      // the hoarding, the huts, the crane and the scaffold out at stage five
      // and put nothing back and the completed mesh comes out with fewer
      // triangles than the stage before it — which is both a monotonicity
      // failure and a lie about which of the two a viewer is looking at. What
      // goes back is what is actually there: stored units in the yard, marked
      // bays, the gate island and the lighting.
      m.part("yard / hand-over dressing", () => {
        for (let i = 0; i < 5; i += 1) {
          m.bar(W * 0.20 + (i % 2) * 6.4, W * 0.20 + (i % 2) * 6.4 + 5.6, GRADE, GRADE + 2.5,
            D * 0.02 + Math.floor(i / 2) * 3.0 - 1.2, D * 0.02 + Math.floor(i / 2) * 3.0 + 1.2, shade(k.accent, 0.86 + 0.1 * (i % 3)));
        }
        m.bar(-W * 0.44, W * 0.44, GRADE + 0.1, GRADE + 0.32, -D * 0.29, -D * 0.27, P.kerb);
        m.bar(-W * 0.36, -W * 0.36 + 5.4, GRADE + 1.6, GRADE + 3.0, -D / 2 + 2.0, -D / 2 + 2.16, shade(k.accent, 1.12));
        // Marked bays on the apron and the gate island between them, which is
        // the difference between a yard in use and a grey rectangle.
        for (let i = 0; i < 3; i += 1) {
          const bx = -W * 0.40 + (i * W * 0.30) / 2;
          m.bar(bx - 0.14, bx + 0.14, GRADE + 0.1, GRADE + 0.14, -D * 0.42, -D * 0.30, P.lane);
        }
        m.bar(-W * 0.06, W * 0.06, GRADE + 0.1, GRADE + 0.4, -D / 2 + 1.6, -D / 2 + 4.4, P.kerb);
        for (let i = 0; i < 2; i += 1) {
          m.bar(-W * 0.03 + i * W * 0.04, -W * 0.02 + i * W * 0.04, GRADE + 0.4, GRADE + 1.5,
            -D / 2 + 2.6, -D / 2 + 3.4, P.safety);
        }
        // Three masts, not four. The map budget's worst case is the finished
        // corridor and it was inside twelve triangles of a hard ceiling; a
        // fourth mast on the far corner is the least of what is being read.
        for (let i = 0; i < 3; i += 1) lightMast(m, (i % 2 ? 1 : -1) * W * 0.40, (i < 2 ? 1 : -1) * D * 0.30, 11, d0);
      });
    }
    return { W, D };
  }

  Mesh.prototype.finish = function (info) {
    const positions = new Float32Array(this.pos);
    const colors = new Float32Array(this.col);
    const normals = new Float32Array(positions.length);
    const count = positions.length / 9;
    const face = new Float64Array(count * 3);
    for (let t = 0; t < count; t += 1) {
      const i = t * 9;
      const ax = positions[i], ay = positions[i + 1], az = positions[i + 2];
      const ux = positions[i + 3] - ax, uy = positions[i + 4] - ay, uz = positions[i + 5] - az;
      const vx = positions[i + 6] - ax, vy = positions[i + 7] - ay, vz = positions[i + 8] - az;
      let nx = uy * vz - uz * vy, ny = uz * vx - ux * vz, nz = ux * vy - uy * vx;
      const len = Math.hypot(nx, ny, nz);
      if (len > 1e-12) { nx /= len; ny /= len; nz /= len; } else { nx = 0; ny = 1; nz = 0; }
      face[t * 3] = nx; face[t * 3 + 1] = ny; face[t * 3 + 2] = nz;
      for (let k = 0; k < 3; k += 1) {
        normals[i + k * 3] = nx; normals[i + k * 3 + 1] = ny; normals[i + k * 3 + 2] = nz;
      }
    }
    // THE SMOOTHING PASS. Corners are gathered per smoothing group by their
    // position on a 1 mm lattice — the same corner reached by two different
    // primitives is computed by the same expression from the same matrix, so it
    // lands on the same key exactly rather than nearly. Nothing here is a lookup
    // into anything outside the buffer, and Map iteration is insertion order, so
    // two processes given the same inputs sum the same numbers in the same order
    // and hand back byte-identical normals.
    const corners = new Map();
    for (let t = 0; t < count; t += 1) {
      const g = this.grp[t];
      if (!g) continue;
      for (let c = 0; c < 3; c += 1) {
        const i = t * 9 + c * 3;
        const key = `${g}|${Math.round(positions[i] * 1000)}|${Math.round(positions[i + 1] * 1000)}`
          + `|${Math.round(positions[i + 2] * 1000)}`;
        const at = corners.get(key);
        if (at) at.push(t * 3 + c); else corners.set(key, [t * 3 + c]);
      }
    }
    let smoothCorners = 0;
    for (const list of corners.values()) {
      if (list.length < 2) continue;
      for (let a = 0; a < list.length; a += 1) {
        const ta = (list[a] - (list[a] % 3)) / 3;
        const ax = face[ta * 3], ay = face[ta * 3 + 1], az = face[ta * 3 + 2];
        let sx = 0, sy = 0, sz = 0, folded = 0;
        for (let b = 0; b < list.length; b += 1) {
          const tb = (list[b] - (list[b] % 3)) / 3;
          const bx = face[tb * 3], by = face[tb * 3 + 1], bz = face[tb * 3 + 2];
          if (ax * bx + ay * by + az * bz < CREASE) continue;
          sx += bx; sy += by; sz += bz; folded += 1;
        }
        if (folded < 2) continue;
        const len = Math.hypot(sx, sy, sz);
        if (len < 1e-9) continue;
        const o = ta * 9 + (list[a] % 3) * 3;
        normals[o] = sx / len; normals[o + 1] = sy / len; normals[o + 2] = sz / len;
        smoothCorners += 1;
      }
    }
    let smoothTriangles = 0;
    for (let t = 0; t < count; t += 1) {
      const i = t * 9;
      if (normals[i] !== normals[i + 3] || normals[i + 1] !== normals[i + 4] || normals[i + 2] !== normals[i + 5]
        || normals[i] !== normals[i + 6] || normals[i + 1] !== normals[i + 7] || normals[i + 2] !== normals[i + 8]) {
        smoothTriangles += 1;
      }
    }
    const min = [Infinity, Infinity, Infinity], max = [-Infinity, -Infinity, -Infinity];
    for (let i = 0; i < positions.length; i += 3) {
      for (let k = 0; k < 3; k += 1) {
        if (positions[i + k] < min[k]) min[k] = positions[i + k];
        if (positions[i + k] > max[k]) max[k] = positions[i + k];
      }
    }
    // Seat the lowest vertex on Y=0. It should already be there — the platform
    // skirt is authored at zero — but a part that dips below grade would
    // otherwise put the site underground, and a renderer placing this on terrain
    // has one contract and this is it.
    const ground = min[1];
    if (ground !== 0) {
      for (let i = 1; i < positions.length; i += 3) positions[i] -= ground;
      max[1] -= ground;
      min[1] = 0;
    }
    const out = {
      positions, normals, colors,
      bounds: { min, max },
      parts: this.parts,
      triangleCount: count,
      // What the smoothing pass actually did, so a caller (and the checks) can
      // see it regress instead of taking it on trust.
      shading: { smoothTriangles, flatTriangles: count - smoothTriangles, smoothCorners, crease: CREASE },
    };
    for (const key of Object.keys(info)) out[key] = info[key];
    return out;
  };

  // --------------------------------------------------------------------- api
  const STATUS_NOTE = {
    building: "work in progress",
    slowed: "work continuing at a reduced rate",
    paused: "work stopped: plant parked, nothing on the hook",
    blocked: "work stopped and blocked: plant parked, nothing on the hook",
  };

  /// `kindKey`, a stage or a progress reading, and options. The options are
  /// `lod` (0 close, 1 map), `level` (which province level is being delivered,
  /// 1..MAX_PROVINCE_LEVEL), `status`, and `variant` — an integer, usually the
  /// project id, which only ever reaches index arithmetic and trig, so the same
  /// inputs give byte-identical output every call.
  function build(kindKey, stageOrProgress, opts) {
    const o = opts && typeof opts === "object" ? opts : {};
    const key = Object.prototype.hasOwnProperty.call(KINDS, kindKey) ? kindKey : FALLBACK_KIND;
    const k = KINDS[key];
    const st = stageFor(key, stageOrProgress, o);
    // ONE LOD VOCABULARY ACROSS THE ART MODULES. This file grew up saying
    // "far" and town-mesh.js grew up saying "map" for the same thing, and each
    // silently fell back to its expensive path when handed the other module's
    // word — a caller asking for the cheap mesh got the dear one and no error.
    // Both words (and the numeric forms) now mean the coarse level in both.
    const lod = o.lod === 1 || o.lod === 2 || o.lod === "far" || o.lod === "map" || o.lod === "lod1" || o.lod === "lod2" ? 1 : 0;
    const rawLevel = typeof o.level === "number" && isFinite(o.level) ? Math.round(o.level) : 1;
    const level = Math.max(1, Math.min(5, rawLevel));
    const variant = typeof o.variant === "number" && isFinite(o.variant) ? Math.abs(Math.round(o.variant)) % 1000 : 0;
    const m = new Mesh();
    const pad = lod ? buildFar(m, k, st, level, variant) : buildNear(m, k, st, level, variant);
    // PLACEHOLDER IS A PER-KIND FACT. It used to be `!k.bespoke`, which meant
    // "anything without its own builder", and that was accurate exactly while
    // twelve of the thirteen were plain massing. They are not any more: each
    // one has a composition in KIND_WORKS built to what roadmap section E says
    // it is. A kind that has NOT had that work carries `placeholder: true` in
    // its own row and says so on its own card, so the count is a number the
    // check can assert rather than a thing anyone has to take on trust.
    const placeholder = k.placeholder === true;
    // Stage five is the hand-off state, so an active site there is finished, not
    // working. Quoting "work in progress" beside 100% was the description
    // disagreeing with itself, and a card that reads a finished plant as still
    // building is the same class of lie as elapsed time completing a building.
    // A stopped status still prints its own note: if the sim ever flags a
    // delivered site blocked, the card should say so rather than paper over it.
    const note = st.index === 4 && st.active ? "handed over" : STATUS_NOTE[st.status];
    const description = `${k.name}, ${st.name.toLowerCase()} (${Math.round(st.progress * 100)}% of recorded work, `
      + `${note}). ${k.blurb}. Original game art: a generic industrial design, not a real `
      + `facility, with no measured dimensions and no capability of its own`
      + `${placeholder ? "; plain placeholder massing pending its own art pass" : ""}.`;
    return m.finish({
      kind: key,
      requestedKind: kindKey,
      stage: st.stage,
      stageIndex: st.index,
      stageName: st.name,
      progress: st.progress,
      status: st.status,
      active: st.active,
      level,
      lod,
      variant,
      placeholder,
      compound: { width: pad.W, depth: pad.D },
      // What a caller should print over the site when work is not happening.
      // The words are the sim's own status, not an art invention.
      overlay: st.active ? null : { status: st.status, label: STATUS_NOTE[st.status] },
      description,
    });
  }

  function kinds() { return KIND_KEYS.slice(); }
  function meta(kindKey) {
    const k = KINDS[kindKey];
    return k ? {
      key: kindKey, name: k.name, blurb: k.blurb,
      placeholder: k.placeholder === true, retrofit: !!k.retrofit,
      compound: { width: k.pad[0], depth: k.pad[1] },
    } : null;
  }

  return Object.freeze({
    build, stages, kinds, meta, stageFor,
    grade: GRADE,
    palette: P,
    statuses: STATUSES.slice(),
    maxLevel: 5,
  });
});
