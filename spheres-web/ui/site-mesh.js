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
   CDN, no third-party runtime (CLAUDE.md). Thirteen kinds at five stages is
   sixty-five meshes; as source they are one file, and the stages share one kit
   rather than being five unrelated copies per kind.

   WHY NO DOM IN HERE. Geometry is the half that has to be right. It is checked
   under node by `tools/ui/check_site_mesh.cjs`, which cannot run a browser and
   should not have to. Whoever draws this owns the GL context and imports
   nothing but the buffers `build` returns.

   MODEL SPACE. Metres. +X starboard, +Y up, +Z forward. Y=0 is ground contact —
   the underside of the formation platform every site sits on, and grade is
   `GRADE` above that, because a real site is built up on fill before anything
   else happens and a foundation pit has to have somewhere to go. Flat
   per-triangle normals, no shared vertices, no index buffer, per-vertex RGB in
   0..1 with no alpha.

   TRUTHFULNESS (roadmap section 3). These are ORIGINAL generic industrial
   designs. Nothing here is a real named plant, no dimension is a measured
   dimension of anything real, and the art grants the province no capability the
   simulation has not already recorded. Twelve of the thirteen kinds are
   deliberately plain massing — legible at map zoom, correct in stage and scale,
   and not finished art. `arms_plant` is the one worked through in full. */
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

  // ------------------------------------------------------------------- build
  // A mesh is a triangle soup with a per-vertex colour, a transform stack and a
  // list of named part ranges. Normals are NOT accumulated: they are derived per
  // triangle in `finish` from the vertices AS TRANSFORMED, so a mirrored wall
  // gets the normal its geometry actually has. Mirroring is how half the walls
  // here are placed, so that is worth the two lines it costs.
  function Mesh() {
    this.pos = [];
    this.col = [];
    this.parts = [];
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

  function ring(rx, ry, seg) {
    const pts = [];
    for (let i = 0; i < seg; i += 1) {
      const t = (i / seg) * Math.PI * 2;
      pts.push([Math.cos(t) * rx, Math.sin(t) * ry]);
    }
    return pts;
  }
  Mesh.prototype.loft = function (sections, col, caps) {
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
  Mesh.prototype.mound = function (x, y, z, r, h, seg, col) {
    const pts = ring(r, r, seg).map((p) => [x + p[0], y, z + p[1]]);
    for (let i = 0; i < seg; i += 1) {
      const j = (i + 1) % seg;
      this.tri(pts[i], pts[j], [x, y + h, z], shade(col, 1 + 0.1 * Math.sin(i * 1.3)));
    }
    this.fan(pts.slice().reverse(), shade(col, 0.8));
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
  // One row per key in PROJECT_KINDS. `arms_plant` carries a bespoke builder;
  // the rest are plain massing driven from these numbers plus a short prop list.
  // The accents are what makes an industry legible at map zoom without turning
  // a province into a theme park (roadmap section 3): a stack, a tank farm, a
  // container stack, a pylon — the thing you would actually recognise from a
  // distance, and not much else.
  const FALLBACK_KIND = "civilian_industry";
  const KINDS = {
    infrastructure: {
      name: "Infrastructure works",
      pad: [58, 40], block: { w: 22, dz: 13, eaves: 7.0, ridge: 8.4, bays: 9, ends: 6 },
      body: rgb(0xa7ada8), accent: rgb(0xb8862c),
      props: ["roadStrip", "pipeStack", "kiosk"],
      blurb: "Road, bridge and utility corridor depot",
    },
    civilian_industry: {
      name: "Civilian industry",
      pad: [60, 42], block: { w: 30, dz: 20, eaves: 8.5, ridge: 10.8, bays: 12, ends: 8 },
      body: rgb(0xa9b0b4), accent: rgb(0x5c7f8c),
      props: ["yardCrane", "tanks", "kiosk"],
      blurb: "Modular factory and service yard",
    },
    power_grid: {
      name: "Power grid",
      pad: [56, 40], block: { w: 16, dz: 11, eaves: 6.0, ridge: 7.0, bays: 7, ends: 5 },
      body: rgb(0xbcbdb6), accent: rgb(0x8b9096),
      props: ["pylon", "transformers", "kiosk"],
      blurb: "Substation, pylons and feeder equipment",
    },
    research_center: {
      name: "Research centre",
      pad: [58, 42], block: { w: 26, dz: 16, eaves: 9.0, ridge: 9.6, bays: 11, ends: 7 },
      body: rgb(0xc3bfb2), accent: rgb(0x46708a),
      props: ["glazing", "plantRoom", "annex"],
      blurb: "Laboratory campus with service buildings",
    },
    arms_plant: {
      name: "Arms plant",
      pad: [66, 48], block: { w: 34, dz: 24, eaves: 11.0, ridge: 14.0, bays: 14, ends: 10 },
      body: rgb(0x9aa2a3), accent: rgb(0x4e5a52),
      props: [],
      bespoke: true,
      blurb: "Assembly hall, test and service yard, secure loading area",
    },
    machinery_works: {
      name: "Machinery works",
      pad: [62, 42], block: { w: 32, dz: 18, eaves: 9.5, ridge: 11.6, bays: 13, ends: 7 },
      body: rgb(0xa4a8a5), accent: rgb(0x7a6a4a),
      props: ["yardCrane", "pipeStack", "docks"],
      blurb: "Machine-tool halls and equipment loading",
    },
    generation: {
      name: "Generation plant",
      pad: [62, 44], block: { w: 26, dz: 20, eaves: 12.0, ridge: 13.4, bays: 11, ends: 8 },
      body: rgb(0xb0b2ac), accent: rgb(0x8d5f45),
      props: ["stack", "transformers", "tanks"],
      blurb: "Generic generation facility; technology variants are later work",
    },
    processing_plant: {
      name: "Processing plant",
      pad: [62, 44], block: { w: 24, dz: 18, eaves: 9.0, ridge: 10.6, bays: 10, ends: 7 },
      body: rgb(0xb6b1a4), accent: rgb(0x8f8a7c),
      props: ["tanks", "pipeRack", "stack"],
      blurb: "Processing hall, tanks and pipework, material handling",
    },
    freight_terminal: {
      name: "Freight terminal",
      pad: [66, 44], block: { w: 20, dz: 12, eaves: 7.5, ridge: 8.4, bays: 8, ends: 5 },
      body: rgb(0xa8adb2), accent: rgb(0x51697f),
      props: ["containers", "gantry", "roadStrip"],
      blurb: "Rail, road and container transfer yard",
    },
    warehouse: {
      name: "Warehouse",
      pad: [64, 42], block: { w: 36, dz: 20, eaves: 9.0, ridge: 10.2, bays: 15, ends: 8 },
      body: rgb(0xb2b6b8), accent: rgb(0x6b7a84),
      props: ["docks", "roadStrip"],
      blurb: "Storage halls and loading docks",
    },
    automation: {
      name: "Automation retrofit",
      pad: [58, 40], block: { w: 18, dz: 14, eaves: 8.0, ridge: 9.0, bays: 8, ends: 6 },
      body: rgb(0xa6acae), accent: rgb(0x66788a),
      props: ["host", "robotCell", "kiosk"],
      retrofit: true,
      blurb: "Machine cells retrofitted into an existing facility",
    },
    efficiency: {
      name: "Efficiency retrofit",
      pad: [58, 40], block: { w: 16, dz: 12, eaves: 7.0, ridge: 8.0, bays: 7, ends: 5 },
      body: rgb(0xb0aca0), accent: rgb(0x7f6a4c),
      props: ["host", "ductRun", "kiosk"],
      retrofit: true,
      blurb: "Heat recovery, insulation and controls in an existing facility",
    },
    starter_industry: {
      name: "Starter industry",
      pad: [46, 34], block: { w: 18, dz: 12, eaves: 6.5, ridge: 8.0, bays: 8, ends: 5 },
      body: rgb(0xb4b0a4), accent: rgb(0x7b8a6a),
      props: ["leanTo", "pipeStack"],
      blurb: "Small workshop and light-industry compound",
    },
  };
  const KIND_KEYS = Object.keys(KINDS);

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
  function platform(m, W, D, hole, col) {
    const hw = W / 2, hd = D / 2;
    if (!hole) {
      m.quad([-hw, GRADE, -hd], [hw, GRADE, -hd], [hw, GRADE, hd], [-hw, GRADE, hd], col);
    } else {
      const aw = hole.w / 2, ad = hole.d / 2, cx = hole.x || 0, cz = hole.z || 0;
      const bands = [
        [-hw, hw, -hd, cz - ad], [-hw, hw, cz + ad, hd],
        [-hw, cx - aw, cz - ad, cz + ad], [cx + aw, hw, cz - ad, cz + ad],
      ];
      for (const b of bands) {
        if (b[1] - b[0] < 1e-6 || b[3] - b[2] < 1e-6) continue;
        m.quad([b[0], GRADE, b[2]], [b[1], GRADE, b[2]], [b[1], GRADE, b[3]], [b[0], GRADE, b[3]], col);
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
  function excavation(m, cx, cz, w, d, depth, benches) {
    const floorY = Math.max(PIT, GRADE - depth);
    const aw = w / 2, ad = d / 2, batter = Math.min(1.6, depth * 2.2);
    const iw = Math.max(1, aw - batter), id = Math.max(1, ad - batter);
    const outer = [[cx - aw, GRADE, cz - ad], [cx + aw, GRADE, cz - ad], [cx + aw, GRADE, cz + ad], [cx - aw, GRADE, cz + ad]];
    const inner = [[cx - iw, floorY, cz - id], [cx + iw, floorY, cz - id], [cx + iw, floorY, cz + id], [cx - iw, floorY, cz + id]];
    for (let i = 0; i < 4; i += 1) {
      const j = (i + 1) % 4;
      m.quad(outer[i], inner[i], inner[j], outer[j], shade(P.earthCut, 0.94 + 0.06 * i));
    }
    m.quad(inner[0], inner[1], inner[2], inner[3], P.earthCut);
    // Trench sheeting on the two long sides once the dig is at depth. It is the
    // detail that separates a hole from a formed excavation at a glance.
    if (benches) {
      for (const s of [-1, 1]) {
        for (let i = 0; i < benches; i += 1) {
          const x = cx - iw + (i + 0.5) * (iw * 2 / benches);
          m.bar(x - 0.5, x + 0.5, floorY, GRADE + 0.35, cz + s * id - 0.08, cz + s * id + 0.08, P.rebar);
        }
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
        if (state === 0 || !d0.fine) continue;
        if (state < 3) {
          for (const s of [-1, 1]) {
            m.bar(x - 1.15, x + 1.15, floorY + 0.3, floorY + 1.05, z + s * 1.15 - 0.06, z + s * 1.15 + 0.06, P.timber);
            m.bar(x + s * 1.15 - 0.06, x + s * 1.15 + 0.06, floorY + 0.3, floorY + 1.05, z - 1.15, z + 1.15, P.timber);
          }
        } else {
          for (let r = 0; r < 4; r += 1) {
            const rx = x + (r % 2 ? 0.62 : -0.62), rz = z + (r < 2 ? 0.62 : -0.62);
            m.bar(rx - 0.05, rx + 0.05, floorY + 0.3, floorY + 1.9, rz - 0.05, rz + 0.05, P.rebar);
          }
          for (let h = 0; h < 3; h += 1) {
            const y = floorY + 0.6 + h * 0.55;
            m.bar(x - 0.68, x + 0.68, y, y + 0.05, z - 0.68, z - 0.6, P.rebar);
            m.bar(x - 0.68, x + 0.68, y, y + 0.05, z + 0.6, z + 0.68, P.rebar);
          }
        }
      }
    }
  }

  /// The slab. From the frame stage on it is the thing everything else stands
  /// on, and its edge is the only place the made-up ground is still visible.
  function slab(m, cx, cz, w, d, col) {
    const top = GRADE + 0.18;
    m.bar(cx - w / 2, cx + w / 2, PIT, top, cz - d / 2, cz + d / 2, col);
    return top;
  }

  /// One portal frame: two columns, two rafters, a haunch each side. Primed
  /// steel, because a frame that has not been clad has not been painted either.
  function portal(m, cz, span, depth, eaves, ridge, col) {
    const hw = span / 2, t = 0.22;
    for (const s of [-1, 1]) {
      m.bar(s * hw - t, s * hw + t, depth, eaves, cz - t, cz + t, col);
      m.bar(s * hw - 0.5, s * hw + 0.5, depth, depth + 0.16, cz - 0.5, cz + 0.5, shade(col, 0.8));
    }
    const steps = 3;
    for (const s of [-1, 1]) {
      for (let i = 0; i < steps; i += 1) {
        const x0 = s * hw * (1 - i / steps), x1 = s * hw * (1 - (i + 1) / steps);
        const y0 = eaves + (ridge - eaves) * (i / steps), y1 = eaves + (ridge - eaves) * ((i + 1) / steps);
        m.quad([x0, y0, cz - t], [x1, y1, cz - t], [x1, y1 + 0.34, cz - t], [x0, y0 + 0.34, cz - t], shade(col, 1.05));
        m.quad([x1, y1, cz + t], [x0, y0, cz + t], [x0, y0 + 0.34, cz + t], [x1, y1 + 0.34, cz + t], shade(col, 0.92));
        m.quad([x0, y0 + 0.34, cz - t], [x1, y1 + 0.34, cz - t], [x1, y1 + 0.34, cz + t], [x0, y0 + 0.34, cz + t], shade(col, 1.12));
      }
    }
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
      m.quad([x0, 0, 0], [x1, 0, 0], [x1, h, 0], [x0, h, 0], shade(col, tone));
      // The lining. A sheeted wall is one quad thick, and a part-clad building
      // is the one time you look at the back of it: without this you see
      // straight through the far elevation and the panel shades from its
      // outward normal, which is worse than a plain grey inside face.
      m.quad([x1, 0, -0.14], [x0, 0, -0.14], [x0, h, -0.14], [x1, h, -0.14], shade(P.dark, 1.5));
    }
    if (done > 0 && d0.fine) {
      const edge = -w / 2 + done * bw;
      m.bar(-w / 2, edge, h - 0.3, h, 0.0, 0.16, shade(col, 1.1));      // eaves trim
      m.bar(-w / 2, edge, 0, 0.3, 0.0, 0.16, shade(col, 0.72));          // base flashing
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
    for (let i = 0; i < done; i += 1) {
      const z0 = -hd + (i * d) / sheets, z1 = -hd + ((i + 1) * d) / sheets;
      const tone = 1 + 0.045 * Math.sin(i * 2.1);
      m.quad([-hw, eaves, z0], [0, ridge, z0], [0, ridge, z1], [-hw, eaves, z1], shade(col, tone));
      m.quad([0, ridge, z0], [hw, eaves, z0], [hw, eaves, z1], [0, ridge, z1], shade(col, tone * 0.93));
      m.quad([0, ridge - 0.2, z0], [-hw, eaves - 0.2, z0], [-hw, eaves - 0.2, z1], [0, ridge - 0.2, z1], shade(P.dark, 1.5));
      m.quad([hw, eaves - 0.2, z0], [0, ridge - 0.2, z0], [0, ridge - 0.2, z1], [hw, eaves - 0.2, z1], shade(P.dark, 1.4));
      if (d0.ribs) {
        m.bar(-hw + 0.2, -0.2, eaves + (ridge - eaves) * 0.5, eaves + (ridge - eaves) * 0.5 + 0.09, z1 - 0.07, z1 + 0.07, shade(col, 0.86));
        m.bar(0.2, hw - 0.2, eaves + (ridge - eaves) * 0.5, eaves + (ridge - eaves) * 0.5 + 0.09, z1 - 0.07, z1 + 0.07, shade(col, 0.82));
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
      m.bar(-0.35, 0.35, ridge + 0.06, ridge + 0.3, -hd, hd, shade(col, 1.12));       // ridge cap
      for (const s of [-1, 1]) m.bar(s * hw - 0.22, s * hw + 0.22, eaves - 0.28, eaves, -hd, hd, shade(col, 0.78));
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
          m.bar(x0 - 0.12, x0 + 0.12, GRADE, GRADE + h + 0.18, z0 - 0.12, z0 + 0.12, P.steel);
        }
      }
    });
  }

  /// The permanent line at hand-over: palisade on concrete, not painted board.
  function fence(m, W, D, d0) {
    const hw = W / 2 - 0.8, hd = D / 2 - 0.8, h = 2.6;
    // Close up, palisade centres are shorter than a hoarding panel and the
    // posts are the thing you read. At map scale a 0.2 m post is well under a
    // pixel, so the coarse path spaces them far enough apart to stay a line of
    // marks rather than a solid band — the two continuous rails carry the
    // silhouette there, and the posts only say what kind of line it is.
    // The coarse step is a budget decision, not a taste one: at 12 m centres a
    // finished level-three compound spent 384 of its 800 map triangles on
    // fence posts and machinery_works blew the ceiling at 830. 22 m holds the
    // worst case at 710 with the compound still growing four metres per level.
    const step = d0.fine ? 3.0 : 22.0;
    for (const [a, b] of [[[-hw, hd], [hw, hd]], [[hw, -hd], [-hw, -hd]], [[hw, hd], [hw, -hd]], [[-hw, -hd], [-hw, hd]]]) {
      const dx = b[0] - a[0], dz = b[1] - a[1], len = Math.hypot(dx, dz);
      const n = Math.max(2, Math.round(len / step));
      for (let i = 0; i <= n; i += 1) {
        const x = a[0] + (dx * i) / n, z = a[1] + (dz * i) / n;
        m.bar(x - 0.1, x + 0.1, GRADE, GRADE + h, z - 0.1, z + 0.1, P.galv);
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
        for (const wx of [-1.6, 0.2]) m.bar(wx, wx + 1.1, y + 1.4, y + 2.15, zz + 1.4, zz + 1.46, P.glass);
        m.bar(1.9, 2.7, y + 0.3, y + 2.3, zz + 1.4, zz + 1.46, P.door);
        for (let b = 0; b < 3; b += 1) m.bar(-2.6 + b * 2.4, -2.2 + b * 2.4, 0, 0.25, zz - 1.2, zz + 1.2, P.concrete);
      }
    }
    if (d0.fine) {
      for (let s = 0; s < 6; s += 1) {
        m.bar(3.0, 4.2, 0.45 * s, 0.45 * s + 0.09, -1.2 + s * 0.22, -0.9 + s * 0.22, P.galv);
      }
    }
    m.restore();
  }

  /// Stacked materials. Four things a site keeps in the open — sheet pallets,
  /// pipe bundles, block packs and steel sections — chosen by index so a site
  /// does not have four of the same pile.
  function materialStack(m, x, z, kind, d0) {
    const y = GRADE;
    if (kind === 0) {                                   // clad sheet packs
      for (let i = 0; i < 3; i += 1) {
        m.bar(x - 3.2, x + 3.2, y + i * 0.34, y + i * 0.34 + 0.3, z - 1.1 + i * 0.12, z + 1.1 - i * 0.12, shade(P.clad, 0.9 + i * 0.06));
      }
    } else if (kind === 1) {                            // pipe bundle
      for (let r = 0; r < (d0.fine ? 2 : 1); r += 1) {
        for (let i = 0; i < 4 - r; i += 1) {
          m.column(x - 2.6 + i * 1.3 + r * 0.65, y + 0.55 + r * 1.05, z, 5.2, 0.5, 0.5, d0.seg, P.steel, false);
        }
      }
      m.bar(x - 3.4, x + 3.4, y, y + 0.16, z - 2.8, z + 2.8, P.timber);
    } else if (kind === 2) {                            // block packs
      for (let i = 0; i < 4; i += 1) {
        const ox = (i % 2) * 2.5, oz = Math.floor(i / 2) * 1.7;
        m.bar(x + ox - 1.1, x + ox + 1.1, y, y + 1.2, z + oz - 0.75, z + oz + 0.75, shade(P.concrete, 0.88 + 0.06 * i));
        if (d0.fine) m.bar(x + ox - 1.15, x + ox + 1.15, y + 1.2, y + 1.26, z + oz - 0.8, z + oz + 0.8, P.hardcore);
      }
    } else {                                            // steel sections
      for (let i = 0; i < 4; i += 1) {
        m.bar(x - 4.5, x + 4.5, y + 0.2 + i * 0.42, y + 0.55 + i * 0.42, z - 0.9 + (i % 2) * 0.55, z - 0.35 + (i % 2) * 0.55, P.primer);
      }
      for (const oz of [-2.2, 2.2]) m.bar(x - 4.2, x + 4.2, y, y + 0.2, z + oz - 0.16, z + oz + 0.16, P.timber);
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
    m.save().move(x, GRADE, z);
    for (let i = 0; i < 4; i += 1) {                              // ballast
      const bx = (i % 2 ? 1 : -1) * 2.6, bz = (i < 2 ? 1 : -1) * 2.6;
      m.bar(bx - 1.6, bx + 1.6, 0, 0.7, bz - 1.6, bz + 1.6, P.concreteWet);
    }
    for (const sx of [-1, 1]) {
      for (const sz of [-1, 1]) {
        m.bar(sx * t - 0.12, sx * t + 0.12, 0.7, h, sz * t - 0.12, sz * t + 0.12, P.safety);
      }
    }
    const ties = d0.fine ? 6 : 2;
    for (let i = 1; i <= ties; i += 1) {
      const y = 0.7 + (h - 0.7) * (i / (ties + 1));
      m.bar(-t, t, y, y + 0.11, -t - 0.1, -t + 0.1, shade(P.safety, 0.9));
      m.bar(-t, t, y, y + 0.11, t - 0.1, t + 0.1, shade(P.safety, 0.9));
      if (d0.fine) {
        m.save().move(0, y, -t).rotZ(28);
        m.bar(-t * 1.2, t * 1.2, 0, 0.09, -0.08, 0.08, shade(P.safety, 0.8));
        m.restore();
      }
    }
    // The slew: a fixed angle, chosen per site from its own index. Two sites do
    // not point the same way, and one site points the same way every time.
    m.save().move(0, h, 0).rotY(slew);
    m.bar(-1.1, 1.1, 0, 1.5, -1.1, 1.1, P.safety);                 // slew ring and cab
    m.bar(-0.9, 0.9, 0.2, 1.9, 1.1, 2.4, shade(P.hut, 0.95));
    if (d0.fine) m.bar(-0.7, 0.7, 0.8, 1.7, 2.4, 2.46, P.glass);
    for (const sx of [-0.45, 0.45]) {
      m.bar(sx - 0.09, sx + 0.09, 1.4, 1.6, 0, jib, shade(P.safety, 1.05));
    }
    m.bar(-0.09, 0.09, 2.5, 2.7, 0, jib * 0.55, shade(P.safety, 0.95));
    const webs = d0.fine ? 7 : 2;
    for (let i = 0; i < webs; i += 1) {
      const zz = (jib * (i + 0.5)) / webs;
      m.bar(-0.5, 0.5, 1.4, 1.55, zz - 0.08, zz + 0.08, shade(P.safety, 0.88));
      if (d0.fine && zz < jib * 0.55) m.bar(-0.06, 0.06, 1.5, 2.6, zz - 0.07, zz + 0.07, shade(P.safety, 0.8));
    }
    m.bar(-1.3, 1.3, 1.2, 2.0, -7.5, -1.1, shade(P.safety, 0.92));  // counter-jib
    m.bar(-1.5, 1.5, 0.6, 2.2, -8.6, -7.4, P.concreteWet);          // counterweight
    // Trolley, rope and hook. Working: hook up over the work. Not working: hook
    // down on the deck and no load on it.
    const trolley = working ? jib * 0.62 : jib * 0.3;
    const hookY = working ? -h * 0.42 : -h + 1.0;
    m.bar(-0.35, 0.35, 1.1, 1.45, trolley - 0.5, trolley + 0.5, P.dark);
    m.bar(-0.05, 0.05, hookY, 1.1, trolley - 0.05, trolley + 0.05, P.dark);
    m.bar(-0.3, 0.3, hookY - 0.5, hookY, trolley - 0.3, trolley + 0.3, shade(P.steel, 0.9));
    if (working) m.bar(-1.2, 1.2, hookY - 1.1, hookY - 0.5, trolley - 1.2, trolley + 1.2, P.timber);
    m.restore();
    m.restore();
  }

  /// Lighting masts. A site works past dark; a finished plant lights its yard.
  /// Same mast at both ends of the job, which is the point of a shared kit.
  function lightMast(m, x, z, h, d0) {
    m.column(x, GRADE, z, h, 0.22, 0.14, d0.seg, P.galv, false);
    m.bar(x - 1.5, x + 1.5, GRADE + h, GRADE + h + 0.18, z - 0.2, z + 0.2, P.galv);
    const lamps = d0.fine ? 3 : 1;
    for (let i = 0; i < lamps; i += 1) {
      const lx = x - 1.1 + (i * 2.2) / Math.max(1, lamps - 1 || 1);
      m.bar(lx - 0.32, lx + 0.32, GRADE + h - 0.35, GRADE + h, z - 0.45, z + 0.45, P.dark);
      m.bar(lx - 0.26, lx + 0.26, GRADE + h - 0.38, GRADE + h - 0.34, z - 0.4, z + 0.4, shade(P.lane, 1.15));
    }
  }

  /// The honest overlay, on the ground rather than in the HUD: a stop board at
  /// the gate. Paused and blocked are different words in the sim and different
  /// boards here, and neither of them is a crane that looks busy.
  function stopBoard(m, x, z, blocked, d0) {
    m.save().move(x, GRADE, z);
    for (const sx of [-1.1, 1.1]) m.bar(sx - 0.09, sx + 0.09, 0, 2.3, -0.09, 0.09, P.galv);
    m.bar(-1.5, 1.5, 1.3, 2.5, -0.06, 0.06, blocked ? P.stopped : P.safety);
    if (d0.fine) {
      for (let i = 0; i < 3; i += 1) m.bar(-1.2, 1.2, 1.5 + i * 0.3, 1.62 + i * 0.3, 0.06, 0.09, shade(P.hut, 1));
      m.bar(-0.6, 0.6, 0, 0.3, -0.6, 0.6, P.concreteWet);
    }
    m.restore();
  }

  /// Concrete batching skid and a bowser. Small, and it is the difference
  /// between a foundation stage that reads as work and one that reads as a
  /// tidy diagram.
  function sitePlant(m, x, z, d0) {
    m.column(x, GRADE, z, 4.6, 1.5, 1.0, d0.seg, P.galv, true);
    m.bar(x - 1.8, x + 1.8, GRADE + 4.6, GRADE + 5.6, z - 1.8, z + 1.8, shade(P.galv, 0.86));
    for (let i = 0; i < 4; i += 1) {
      const a = i * 90 * DEG;
      m.bar(x + Math.cos(a) * 1.9 - 0.1, x + Math.cos(a) * 1.9 + 0.1, GRADE, GRADE + 4.6,
        z + Math.sin(a) * 1.9 - 0.1, z + Math.sin(a) * 1.9 + 0.1, P.steel);
    }
    m.bar(x + 3.4, x + 7.0, GRADE, GRADE + 1.9, z - 1.1, z + 1.1, P.safety);
    if (d0.fine) m.bar(x + 3.3, x + 7.1, GRADE + 1.85, GRADE + 2.0, z - 1.2, z + 1.2, shade(P.safety, 0.8));
  }

  /// Scaffold on one gable while the sheeting goes up. Present only in the
  /// enclosing stage, which is the only time it would be there.
  function scaffold(m, x, z, w, h, d0) {
    const lifts = d0.fine ? 4 : 2, bays = d0.fine ? 5 : 2;
    for (let i = 0; i <= bays; i += 1) {
      const px = x - w / 2 + (i * w) / bays;
      for (const pz of [z, z + 1.3]) m.bar(px - 0.06, px + 0.06, GRADE, GRADE + h, pz - 0.06, pz + 0.06, P.galv);
    }
    for (let i = 1; i <= lifts; i += 1) {
      const y = (h * i) / lifts;
      m.bar(x - w / 2, x + w / 2, GRADE + y, GRADE + y + 0.08, z - 0.06, z + 1.36, P.timber);
      m.bar(x - w / 2, x + w / 2, GRADE + y + 0.9, GRADE + y + 0.98, z + 1.24, z + 1.36, P.galv);
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
    const frames = d0.fine ? 7 : 3;
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
      roof(m, w, dz, eaves, ridge, d0.fine ? 10 : 3, P.roof, d0, { clad: 0, purlins: true });
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
      for (const s of [-1, 1]) {
        m.save().move(0, 0, s * (dz / 2)).rotY(s > 0 ? 0 : 180);
        claddedWall(m, w, eaves, bays, o.col, d0, { clad, phase: s });
        m.restore();
      }
      for (const s of [-1, 1]) {
        m.save().move(s * (w / 2), 0, 0).rotY(90 * s);
        claddedWall(m, dz, eaves, ends, o.col, d0, { clad, phase: s * 2 });
        m.restore();
      }
      m.restore();
    });
    m.part(`envelope / roof sheeting and ridge${tag}`, () => {
      m.save().move(o.x, o.deck, o.z);
      roof(m, w, dz, eaves, ridge, d0.fine ? 10 : 3, P.roof, d0, { clad });
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
          const slats = 6;
          for (let s = 0; s < slats; s += 1) {
            const y = 0.15 + (s * (dh - 0.3)) / slats;
            m.bar(cx - dw / 2 + 0.05, cx + dw / 2 - 0.05, y, y + (dh - 0.3) / slats - 0.06, 0.14, 0.2, shade(P.door, 1.08));
          }
          m.bar(cx - dw / 2 - 0.16, cx + dw / 2 + 0.16, dh, dh + 0.28, 0, 0.24, P.safety);
        }
      }
      if (d0.fine) {
        m.bar(w / 2 - 2.6, w / 2 - 1.4, 0.05, 2.3, 0, 0.12, shade(P.door, 0.85));
        m.bar(w / 2 - 3.0, w / 2 - 1.0, 2.3, 2.5, 0, 1.4, shade(P.clad, 1.06));
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
      m.bar(x - 11.2, x + 11.2, GRADE + 1.2, GRADE + 1.32, z - 3.4, z + 3.4, P.kerb);
      const docks = d0.fine ? 4 : 2;
      for (let i = 0; i < docks; i += 1) {
        const dx = x - 8.2 + (i * 16.4) / (docks - 1);
        m.bar(dx - 1.6, dx + 1.6, GRADE + 1.32, GRADE + 1.44, z - 3.4, z - 1.4, shade(P.steel, 1.05));
        if (d0.fine) {
          for (const s of [-1, 1]) m.bar(dx + s * 1.75 - 0.18, dx + s * 1.75 + 0.18, GRADE + 1.32, GRADE + 2.6, z - 3.5, z - 3.1, P.dark);
          m.bar(dx - 1.9, dx + 1.9, GRADE + 2.6, GRADE + 2.85, z - 3.55, z - 3.05, P.dark);
        }
      }
    });
    m.part("yard / dock canopy", () => {
      const posts = d0.fine ? 5 : 2;
      for (let i = 0; i < posts; i += 1) {
        const px = x - 10 + (i * 20) / (posts - 1);
        m.bar(px - 0.16, px + 0.16, GRADE, GRADE + 5.2, z - 3.6, z - 3.28, P.steel);
      }
      m.save().move(0, GRADE + 5.2, 0);
      m.plate([[x - 11.5, z - 5.6], [x + 11.5, z - 5.6], [x + 11.5, z + 0.6], [x - 11.5, z + 0.6]], 0.22, shade(P.roof, 1.04));
      m.restore();
    });
    m.part("yard / bollards and dock markings", () => {
      const n = d0.fine ? 8 : 3;
      for (let i = 0; i < n; i += 1) {
        const bx = x - 10.5 + (i * 21) / (n - 1);
        m.column(bx, GRADE, z - 4.6, 1.05, 0.22, 0.22, d0.seg, P.safety, true);
      }
      if (d0.fine) {
        for (let i = 0; i < 4; i += 1) {
          const lx = x - 7.5 + i * 5;
          m.bar(lx - 0.12, lx + 0.12, GRADE + 0.01, GRADE + 0.04, z - 11, z - 5, P.lane);
        }
      }
    });
  }

  function gatehouse(m, x, z, d0) {
    m.part("yard / gatehouse and controlled entry", () => {
      m.bar(x - 2.4, x + 2.4, GRADE, GRADE + 3.1, z - 2.0, z + 2.0, shade(P.clad, 1.02));
      m.bar(x - 2.7, x + 2.7, GRADE + 3.1, GRADE + 3.35, z - 2.3, z + 2.3, P.roof);
      if (d0.fine) {
        for (const s of [-1, 1]) m.bar(x - 1.7, x + 1.7, GRADE + 1.5, GRADE + 2.5, z + s * 2.0, z + s * 2.04, P.glass);
        m.bar(x + 2.6, x + 9.0, GRADE + 1.05, GRADE + 1.2, z - 0.16, z + 0.16, P.safety);
        m.column(x + 2.6, GRADE, z, 1.4, 0.16, 0.16, d0.seg, P.dark, true);
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
          m.bar(x + s * (w / 2) - 0.3, x + s * (w / 2) + 0.3, GRADE + 0.12, GRADE + 0.42, z - d / 2, z + d / 2, P.kerb);
        }
        // Ramp and inspection pit cover, which is what a service yard has.
        m.bar(x - 4, x + 4, GRADE + 0.12, GRADE + 0.9, z - d / 2 - 3, z - d / 2 - 0.4, P.concrete);
        m.bar(x - 2.2, x + 2.2, GRADE + 0.13, GRADE + 0.2, z + 2, z + 8, shade(P.steel, 0.9));
      }
    });
  }

  function utilities(m, x, z, d0, tall) {
    m.part("services / substation, tanks and stack", () => {
      m.bar(x - 3.0, x + 3.0, GRADE, GRADE + 3.4, z - 2.2, z + 2.2, shade(P.concrete, 0.96));
      m.bar(x - 3.2, x + 3.2, GRADE + 3.4, GRADE + 3.6, z - 2.4, z + 2.4, P.roof);
      for (const s of [-1, 1]) {
        m.bar(x + 4.2, x + 6.6, GRADE, GRADE + 2.4, z + s * 1.6 - 1.0, z + s * 1.6 + 1.0, P.galv);
        if (d0.fine) m.column(x + 5.4, GRADE + 2.4, z + s * 1.6, 1.6, 0.3, 0.3, d0.seg, P.dark, true);
      }
      m.column(x - 6.5, GRADE, z, 7.2, 1.9, 1.9, d0.seg, shade(P.galv, 0.94), true);
      m.column(x - 10.5, GRADE, z, 7.2, 1.9, 1.9, d0.seg, shade(P.galv, 0.9), true);
      if (tall) m.column(x - 1.0, GRADE, z - 5.5, 17.0, 0.95, 0.75, d0.seg, shade(P.clad, 0.88), false);
      if (d0.fine) {
        for (let i = 0; i < 4; i += 1) {
          m.bar(x - 11.0, x - 6.0, GRADE + 5.0 + i * 0.5, GRADE + 5.3 + i * 0.5, z - 0.2, z + 0.2, P.steel);
        }
      }
    });
  }

  // ------------------------------------------------------------ generic kit
  // The other twelve. Deliberately plain massing on the same stage kit: a
  // correctly staged shed with the kind's one recognisable feature beside it.
  // They are placeholders and the description says so.

  function genericProps(m, k, x, z, W, D, d0, stage) {
    const built = stage >= 4;
    for (const prop of k.props) {
      if (prop === "stack") {
        m.part("services / flue stack", () => {
          m.column(x + W * 0.26, GRADE, z - D * 0.18, built ? 22 : 6, 1.3, 1.0, d0.seg, shade(P.clad, 0.86), false);
          if (d0.fine && built) for (let i = 0; i < 3; i += 1) m.column(x + W * 0.26, GRADE + 8 + i * 4.5, z - D * 0.18, 0.4, 1.5, 1.5, d0.seg, P.safety, true);
        });
      } else if (prop === "tanks") {
        m.part("services / tank farm", () => {
          for (let i = 0; i < (d0.fine ? 3 : 2); i += 1) {
            m.column(x + W * 0.3, GRADE, z + D * 0.1 + i * 6.5, built ? 9 : 2.5, 2.6, 2.6, d0.seg, shade(P.galv, 0.95 + i * 0.03), true);
          }
        });
      } else if (prop === "pipeRack") {
        m.part("services / pipe rack", () => {
          for (let i = 0; i < (d0.fine ? 5 : 2); i += 1) {
            const px = x - W * 0.3 + (i * W * 0.6) / 4;
            m.bar(px - 0.16, px + 0.16, GRADE, GRADE + 5.5, z - D * 0.3 - 0.16, z - D * 0.3 + 0.16, P.steel);
          }
          for (let i = 0; i < (d0.fine ? 3 : 1); i += 1) {
            m.bar(x - W * 0.32, x + W * 0.32, GRADE + 4.4 + i * 0.5, GRADE + 4.7 + i * 0.5, z - D * 0.3 - 0.25, z - D * 0.3 + 0.25, shade(P.galv, 0.94));
          }
        });
      } else if (prop === "transformers") {
        m.part("services / transformer bays", () => {
          for (let i = 0; i < (d0.fine ? 3 : 2); i += 1) {
            const tx = x - W * 0.22 + i * 7.5;
            m.bar(tx - 2.2, tx + 2.2, GRADE, GRADE + 3.2, z + D * 0.16 - 1.6, z + D * 0.16 + 1.6, shade(P.steel, 1.05));
            if (d0.fine) {
              for (const s of [-1, 1]) m.column(tx + s * 1.2, GRADE + 3.2, z + D * 0.16, 1.5, 0.35, 0.28, d0.seg, P.hardcore, true);
              m.bar(tx - 2.6, tx + 2.6, GRADE, GRADE + 4.2, z + D * 0.16 + 2.2, z + D * 0.16 + 2.3, P.galv);
            }
          }
        });
      } else if (prop === "pylon") {
        m.part("services / feeder gantry", () => {
          const px = x - W * 0.3, py = built ? 14 : 5;
          for (const sx of [-1, 1]) {
            for (const sz of [-1, 1]) {
              m.bar(px + sx * 1.5 - 0.14, px + sx * 1.5 + 0.14, GRADE, GRADE + py, z - D * 0.2 + sz * 1.5 - 0.14, z - D * 0.2 + sz * 1.5 + 0.14, P.galv);
            }
          }
          m.bar(px - 4.5, px + 4.5, GRADE + py, GRADE + py + 0.4, z - D * 0.2 - 0.25, z - D * 0.2 + 0.25, P.galv);
          if (d0.fine) {
            for (let i = 0; i < 3; i += 1) {
              const ax = px - 3.4 + i * 3.4;
              m.column(ax, GRADE + py - 1.2, z - D * 0.2, 1.2, 0.16, 0.16, d0.seg, P.hardcore, false);
            }
            for (let i = 1; i <= 3; i += 1) {
              const y = GRADE + (py * i) / 4;
              m.bar(px - 1.6, px + 1.6, y, y + 0.12, z - D * 0.2 - 1.6, z - D * 0.2 - 1.5, shade(P.galv, 0.9));
              m.bar(px - 1.6, px + 1.6, y, y + 0.12, z - D * 0.2 + 1.5, z - D * 0.2 + 1.6, shade(P.galv, 0.9));
            }
          }
        });
      } else if (prop === "containers") {
        m.part("yard / container stacks", () => {
          const n = d0.fine ? 6 : 2;
          for (let i = 0; i < n; i += 1) {
            const cx = x - W * 0.3 + (i % 3) * 7.0, cz = z + D * 0.18 + Math.floor(i / 3) * 3.2;
            const high = built ? 2 : 1;
            for (let h = 0; h < high; h += 1) {
              m.bar(cx - 3.0, cx + 3.0, GRADE + h * 2.6, GRADE + 2.5 + h * 2.6, cz - 1.2, cz + 1.2,
                shade(k.accent, 0.82 + 0.12 * ((i + h) % 3)));
            }
          }
        });
      } else if (prop === "gantry") {
        m.part("yard / transfer gantry", () => {
          const gz = z + D * 0.18, h = built ? 12 : 4;
          for (const sx of [-1, 1]) {
            m.bar(x + sx * 13 - 0.5, x + sx * 13 + 0.5, GRADE, GRADE + h, gz - 0.5, gz + 0.5, P.safety);
            if (d0.fine) m.bar(x + sx * 13 - 1.2, x + sx * 13 + 1.2, GRADE, GRADE + 0.6, gz - 1.2, gz + 1.2, P.concrete);
          }
          m.bar(x - 14, x + 14, GRADE + h, GRADE + h + 1.3, gz - 0.9, gz + 0.9, P.safety);
          if (d0.fine) m.bar(x - 2, x + 2, GRADE + h - 3.4, GRADE + h, gz - 0.7, gz + 0.7, P.dark);
        });
      } else if (prop === "yardCrane") {
        m.part("yard / outdoor crane rail", () => {
          for (const s of [-1, 1]) {
            m.bar(x - W * 0.34, x + W * 0.34, GRADE, GRADE + 0.25, z - D * 0.28 + s * 4 - 0.3, z - D * 0.28 + s * 4 + 0.3, P.steel);
          }
          if (built) {
            for (const s of [-1, 1]) m.bar(x - 0.4, x + 0.4, GRADE + 0.25, GRADE + 7.5, z - D * 0.28 + s * 4 - 0.4, z - D * 0.28 + s * 4 + 0.4, P.safety);
            m.bar(x - 1.0, x + 1.0, GRADE + 7.5, GRADE + 8.4, z - D * 0.28 - 4.6, z - D * 0.28 + 4.6, P.safety);
          }
        });
      } else if (prop === "docks") {
        m.part("yard / loading docks", () => {
          const dz0 = z - D * 0.24;
          m.bar(x - W * 0.3, x + W * 0.3, GRADE, GRADE + 1.25, dz0 - 2.4, dz0 + 2.4, P.concrete);
          const n = d0.fine ? 5 : 2;
          for (let i = 0; i < n; i += 1) {
            const dx = x - W * 0.26 + (i * W * 0.52) / (n - 1);
            m.bar(dx - 1.5, dx + 1.5, GRADE + 1.25, GRADE + 1.36, dz0 - 2.5, dz0 - 0.9, shade(P.steel, 1.05));
            if (d0.fine) m.bar(dx - 1.7, dx + 1.7, GRADE + 1.36, GRADE + 3.9, dz0 - 2.58, dz0 - 2.46, P.dark);
          }
        });
      } else if (prop === "roadStrip") {
        m.part("yard / access road", () => {
          m.save().move(0, GRADE, 0);
          m.plate([[x - W * 0.42, z - D * 0.44], [x + W * 0.42, z - D * 0.44], [x + W * 0.42, z - D * 0.30], [x - W * 0.42, z - D * 0.30]], 0.1, P.asphalt);
          m.restore();
          if (d0.fine) {
            for (let i = 0; i < 7; i += 1) {
              const lx = x - W * 0.36 + (i * W * 0.72) / 6;
              m.bar(lx - 1.1, lx + 1.1, GRADE + 0.1, GRADE + 0.13, z - D * 0.375, z - D * 0.365, P.lane);
            }
          }
        });
      } else if (prop === "pipeStack") {
        m.part("yard / stacked sections", () => materialStack(m, x - W * 0.3, z + D * 0.26, 1, d0));
      } else if (prop === "kiosk") {
        m.part("services / control kiosk", () => {
          m.bar(x + W * 0.32 - 2.2, x + W * 0.32 + 2.2, GRADE, GRADE + 2.9, z - D * 0.3 - 1.6, z - D * 0.3 + 1.6, shade(P.clad, 1.04));
          m.bar(x + W * 0.32 - 2.4, x + W * 0.32 + 2.4, GRADE + 2.9, GRADE + 3.1, z - D * 0.3 - 1.8, z - D * 0.3 + 1.8, P.roof);
          if (d0.fine) m.bar(x + W * 0.32 - 0.6, x + W * 0.32 + 0.6, GRADE + 0.1, GRADE + 2.2, z - D * 0.3 + 1.6, z - D * 0.3 + 1.64, P.door);
        });
      } else if (prop === "glazing" && built) {
        m.part("envelope / strip glazing", () => {
          const b = k.block;
          for (const s of [-1, 1]) {
            for (let i = 0; i < (d0.fine ? 8 : 3); i += 1) {
              const gx = -b.w / 2 + 1.5 + (i * (b.w - 3)) / ((d0.fine ? 8 : 3) - 1);
              m.bar(x + gx - 0.8, x + gx + 0.8, GRADE + 3.2, GRADE + 5.4, z + s * (b.dz / 2) - 0.05, z + s * (b.dz / 2) + 0.05, P.glass);
            }
          }
        });
      } else if (prop === "plantRoom" && built) {
        m.part("services / roof plant", () => {
          const b = k.block;
          m.bar(x - 4, x + 4, GRADE + b.eaves + 0.2, GRADE + b.eaves + 2.6, z - 3, z + 3, shade(P.galv, 0.95));
          if (d0.fine) for (let i = 0; i < 3; i += 1) m.column(x - 2.4 + i * 2.4, GRADE + b.eaves + 2.6, z, 1.1, 0.7, 0.7, d0.seg, P.dark, true);
        });
      } else if (prop === "annex" && built) {
        m.part("structure / service annex", () => {
          const b = k.block;
          m.bar(x + b.w / 2, x + b.w / 2 + 9, GRADE, GRADE + 5.2, z - 5, z + 5, shade(k.body, 0.95));
          m.bar(x + b.w / 2 - 0.2, x + b.w / 2 + 9.2, GRADE + 5.2, GRADE + 5.5, z - 5.3, z + 5.3, P.roof);
        });
      } else if (prop === "leanTo" && built) {
        m.part("structure / open lean-to", () => {
          const b = k.block;
          for (let i = 0; i < (d0.fine ? 4 : 2); i += 1) {
            const px = x - b.w / 2 + (i * b.w) / ((d0.fine ? 4 : 2) - 1);
            m.bar(px - 0.14, px + 0.14, GRADE, GRADE + 3.6, z + b.dz / 2 + 5.2, z + b.dz / 2 + 5.5, P.steel);
          }
          m.save().move(0, GRADE + 3.6, 0);
          m.plate([[x - b.w / 2 - 0.4, z + b.dz / 2], [x + b.w / 2 + 0.4, z + b.dz / 2], [x + b.w / 2 + 0.4, z + b.dz / 2 + 5.8], [x - b.w / 2 - 0.4, z + b.dz / 2 + 5.8]], 0.2, P.roof);
          m.restore();
        });
      } else if (prop === "robotCell" && built) {
        m.part("structure / machine cell annex", () => {
          const b = k.block;
          m.bar(x - b.w / 2 - 8, x - b.w / 2, GRADE, GRADE + 4.6, z - 4, z + 4, shade(k.accent, 1.05));
          if (d0.fine) {
            for (let i = 0; i < 3; i += 1) m.column(x - b.w / 2 - 6 + i * 2.4, GRADE + 4.6, z, 1.4, 0.5, 0.4, d0.seg, P.galv, true);
          }
        });
      } else if (prop === "ductRun" && built) {
        m.part("services / heat recovery ducts", () => {
          const b = k.block;
          for (let i = 0; i < (d0.fine ? 4 : 2); i += 1) {
            const dx = x - b.w * 0.3 + (i * b.w * 0.6) / ((d0.fine ? 4 : 2) - 1);
            m.column(dx, GRADE + b.eaves, z, 3.0, 0.75, 0.75, d0.seg, shade(P.galv, 1.02), true);
          }
          m.bar(x - b.w * 0.34, x + b.w * 0.34, GRADE + b.eaves + 2.6, GRADE + b.eaves + 3.4, z - 0.8, z + 0.8, shade(P.galv, 0.95));
        });
      } else if (prop === "host") {
        // A retrofit is installed INTO something that already exists, and the
        // roadmap is explicit that automation and efficiency are not new
        // standalone buildings. The host shed is therefore drawn complete at
        // every stage: the staged work is the kit going into it.
        m.part("structure / existing host facility", () => {
          const hx = x + 20, b = k.block;
          m.bar(hx - 11, hx + 11, GRADE, GRADE + 7.0, z - 8, z + 8, shade(P.clad, 0.9));
          m.save().move(hx, GRADE, z);
          roof(m, 22, 16, 7.0, 8.8, d0.fine ? 6 : 2, shade(P.roof, 0.92), d0, { clad: 1 });
          m.restore();
          if (d0.fine) {
            m.bar(hx - 4, hx + 4, GRADE + 0.1, GRADE + 4.4, z - 8.05, z - 7.95, P.door);
            for (let i = 0; i < 4; i += 1) m.bar(hx - 9 + i * 6, hx - 7 + i * 6, GRADE + 4.8, GRADE + 6.2, z + 7.95, z + 8.05, P.glass);
          }
          void b;
        });
      }
    }
  }

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
          m.bar(-2.4, 2.4, 0.02, 0.2, -0.7, 0.7, shade(P.glass, 1.35));
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
          m.bar(lx - 0.9, lx + 0.9, o.deck + b.eaves - 2.4, o.deck + b.eaves - 1.0,
            o.z + s * (b.dz / 2) - 0.02, o.z + s * (b.dz / 2) + 0.18, P.dark);
        }
      }
    });
    m.part("envelope / entrance canopy and reception", () => {
      const ex = o.x - b.w * 0.36, ez = o.z - b.dz / 2;
      for (const px of [ex - 2.4, ex + 2.4]) m.bar(px - 0.14, px + 0.14, GRADE, GRADE + 3.4, ez - 3.2, ez - 2.92, P.galv);
      m.save().move(0, GRADE + 3.4, 0);
      m.plate([[ex - 3.0, ez - 3.6], [ex + 3.0, ez - 3.6], [ex + 3.0, ez + 0.2], [ex - 3.0, ez + 0.2]], 0.18, shade(P.roof, 1.06));
      m.restore();
      m.bar(ex - 1.1, ex + 1.1, o.deck + 0.05, o.deck + 2.4, ez - 0.06, ez + 0.02, P.glass);
      if (d0.fine) {
        for (let i = 0; i < 4; i += 1) {
          const gx = ex - 3.0 + i * 1.6;
          m.bar(gx - 0.55, gx + 0.55, o.deck + 2.7, o.deck + 4.2, ez - 0.06, ez + 0.02, P.glass);
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
        m.bar(-W * 0.46 + i * 3, -W * 0.46 + i * 3 + 2.4, GRADE + 0.1, GRADE + 0.32, -D * 0.40 - 0.2, -D * 0.40 + 0.2, P.kerb);
      }
      m.bar(-W * 0.46, W * 0.46, GRADE + 0.1, GRADE + 0.32, -D * 0.455, -D * 0.435, P.kerb);
    });
    m.part("yard / parked vehicles and stored units", () => {
      for (let i = 0; i < (d0.fine ? 4 : 1); i += 1) {
        const tx = -W * 0.415, tz = -D * 0.24 + i * 9.2;
        m.bar(tx - 2.2, tx + 2.2, GRADE + 1.1, GRADE + 3.6, tz - 6.0, tz + 2.2, shade(P.clad, 1.02));
        m.bar(tx - 1.9, tx + 1.9, GRADE + 0.55, GRADE + 1.1, tz - 5.4, tz - 3.6, P.dark);
        m.bar(tx - 1.9, tx + 1.9, GRADE, GRADE + 0.6, tz + 0.4, tz + 2.0, P.dark);
      }
      for (let i = 0; i < (d0.fine ? 8 : 2); i += 1) {
        const cx = W * 0.32 + (i % 2) * 6.4, cz = D * 0.30 + Math.floor(i / 2) * 3.0;
        m.bar(cx - 3.0, cx + 3.0, GRADE, GRADE + 2.5, cz - 1.2, cz + 1.2, shade(k.accent, 0.84 + 0.1 * (i % 3)));
      }
    });
    m.part("yard / bollards and gate island", () => {
      for (let i = 0; i < (d0.fine ? 10 : 3); i += 1) {
        const n = d0.fine ? 10 : 3;
        m.column(-W * 0.42 + (i * W * 0.84) / (n - 1), GRADE, -D * 0.415, 1.0, 0.2, 0.2, d0.seg, P.safety, true);
      }
    });
    m.part("yard / weighbridge and external storage racks", () => {
      m.bar(-W * 0.47, -W * 0.47 + 9, GRADE + 0.08, GRADE + 0.24, D * 0.10, D * 0.10 + 3.6, shade(P.steel, 1.02));
      for (const s of [-1, 1]) {
        m.column(-W * 0.47 + 4.5, GRADE, D * 0.10 + 1.8 + s * 2.6, 1.4, 0.18, 0.18, d0.seg, P.safety, true);
      }
      const uprights = d0.fine ? 7 : 3;
      for (let i = 0; i < uprights; i += 1) {
        const rx = -W * 0.16 + (i * 11) / (uprights - 1);
        for (const rz of [D * 0.42, D * 0.42 + 2.4]) m.bar(rx - 0.12, rx + 0.12, GRADE, GRADE + 4.4, rz - 0.12, rz + 0.12, shade(P.safety, 0.9));
      }
      for (let i = 0; i < (d0.fine ? 3 : 1); i += 1) {
        const y = GRADE + 1.3 + i * 1.5;
        m.bar(-W * 0.16, -W * 0.16 + 11, y, y + 0.16, D * 0.42 - 0.2, D * 0.42 + 2.6, shade(P.steel, 1.04));
        if (d0.fine) m.bar(-W * 0.16 + 1 + i * 2, -W * 0.16 + 4 + i * 2, y + 0.16, y + 1.0, D * 0.42 - 0.1, D * 0.42 + 2.5, shade(P.timber, 1.05));
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
    if (stage <= 2) {
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

  function buildNear(m, k, st, level, variant) {
    const d0 = { fine: true, ribs: true, seg: 8 };
    const h = (keyHash(k.name) + variant * 17) % 360;
    const W = k.pad[0] + (level - 1) * 4, D = k.pad[1] + (level - 1) * 2;
    const stage = st.index + 1;
    const b = k.block;
    const o = { x: -W * 0.06, z: D * 0.10, block: b, col: k.body, deck: GRADE + 0.18, clad: CLAD_BY_STAGE[st.index] };

    m.part("site / formation platform", () => {
      platform(m, W, D, stage <= 2 ? { x: o.x, z: o.z, w: b.w + 3.2, d: b.dz + 3.2 } : null, P.fill);
    });
    if (stage <= 2) {
      m.part("earthworks / excavation", () => {
        excavation(m, o.x, o.z, b.w + 3.2, b.dz + 3.2, stage === 1 ? 0.35 : 1.1, stage === 2 && d0.fine ? 5 : 0);
      });
      if (stage === 2) {
        m.part("earthworks / pad footings, reinforcement and formwork", () => {
          footings(m, o.x, o.z, b.w, b.dz, 4, 3, d0);
        });
        m.part("earthworks / blinding and formed slab edge", () => {
          m.bar(o.x - b.w / 2 - 1, o.x + b.w / 2 + 1, PIT, PIT + 0.12, o.z - b.dz / 2 - 1, o.z - b.dz / 2 + 3, P.concreteWet);
          m.bar(o.x - b.w / 2 - 1, o.x - b.w / 2 + 3, PIT, PIT + 0.12, o.z - b.dz / 2 - 1, o.z + b.dz / 2 + 1, P.concreteWet);
        });
      }
    } else {
      m.part("structure / ground slab", () => slab(m, o.x, o.z, b.w + 2.2, b.dz + 2.2, P.concrete));
    }
    if (stage === 1) {
      m.part("site / setting out and access", () => {
        for (let i = 0; i < 10; i += 1) {
          const px = o.x - b.w / 2 - 3 + (i * (b.w + 6)) / 9;
          for (const s of [-1, 1]) {
            m.bar(px - 0.08, px + 0.08, GRADE, GRADE + 1.0, o.z + s * (b.dz / 2 + 3) - 0.08, o.z + s * (b.dz / 2 + 3) + 0.08, P.safety);
          }
        }
        m.save().move(0, GRADE, 0);
        m.plate([[-W * 0.44, -D * 0.42], [W * 0.1, -D * 0.42], [W * 0.1, -D * 0.30], [-W * 0.44, -D * 0.30]], 0.1, P.hardcore);
        m.restore();
      });
    }

    // Levels already delivered stand finished beside the works, because
    // MAX_PROVINCE_LEVEL is five and five identical buildings is not what a
    // level upgrade is (roadmap section E). Each completed level is a wing.
    for (let l = 1; l < level; l += 1) {
      const wing = {
        x: o.x + b.w / 2 + 7 + (l - 1) * 13,
        z: o.z - 3 + ((l % 2) ? 4 : -4),
        block: { w: 11, dz: b.dz * 0.62, eaves: b.eaves * 0.78, ridge: b.ridge * 0.74, bays: 5, ends: 4 },
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
      if (stage >= 3) genericBlock(m, k, o, d0);
      genericProps(m, k, o.x, o.z, W, D, d0, stage);
    }

    if (stage >= 5) {
      m.part("yard / permanent perimeter", () => fence(m, W, D, d0));
      m.part("yard / apron and verge", () => {
        m.save().move(0, GRADE, 0);
        m.plate([[-W * 0.44, -D * 0.44], [W * 0.44, -D * 0.44], [W * 0.44, -D * 0.28], [-W * 0.44, -D * 0.28]], 0.1, P.asphalt);
        m.restore();
        for (let i = 0; i < (d0.fine ? 12 : 5); i += 1) {
          const n = d0.fine ? 12 : 5;
          m.mound(-W * 0.46 + (i * W * 0.92) / (n - 1), GRADE, -D * 0.47, 1.5, 2.4, d0.fine ? 6 : 4, P.grass);
        }
      });
      m.part("yard / signage and lighting", () => {
        m.bar(-W * 0.36, -W * 0.36 + 5.4, GRADE + 1.6, GRADE + 3.0, -D / 2 + 2.0, -D / 2 + 2.16, shade(k.accent, 1.12));
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
    const W = k.pad[0] + (level - 1) * 4, D = k.pad[1] + (level - 1) * 2;
    const stage = st.index + 1;
    const b = k.block;
    const x = -W * 0.06, z = D * 0.10;

    m.part("site / formation platform", () => platform(m, W, D, null, P.fill));
    m.part("site / perimeter", () => {
      if (stage >= 5) fence(m, W, D, d0); else hoarding(m, W, D, d0, P.hoard);
    });
    if (stage <= 2) {
      m.part("earthworks / dig and spoil", () => {
        m.bar(x - b.w / 2, x + b.w / 2, GRADE - 0.4, GRADE, z - b.dz / 2, z + b.dz / 2, P.earthCut);
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
            m.bar(x + s * b.w * 0.25 - 0.35, x + s * b.w * 0.25 + 0.35, GRADE, GRADE + b.eaves, z - b.dz / 2 - 0.3, z - b.dz / 2 + 0.3, P.primer);
          }
          m.bar(x - b.w / 2, x + b.w / 2, GRADE + b.eaves * 0.55, GRADE + b.eaves * 0.55 + 0.3, z - b.dz / 2 - 0.35, z - b.dz / 2 + 0.05, P.primer);
          scaffold(m, x, z + b.dz / 2 + 0.6, b.w * 0.6, b.eaves, d0);
          materialStack(m, x - b.w * 0.55, z + b.dz * 0.6, 0, d0);
        });
      }
    }
    for (let l = 1; l < level; l += 1) {
      m.part(`structure / delivered wing ${l}`, () => {
        const wx = x + b.w / 2 + 7 + (l - 1) * 13, wz = z - 3 + ((l % 2) ? 4 : -4);
        m.bar(wx - 5.5, wx + 5.5, GRADE, GRADE + b.eaves * 0.78, wz - b.dz * 0.31, wz + b.dz * 0.31, shade(k.body, 0.96));
      });
    }
    if (stage >= 3 && stage <= 4) {
      m.part("plant / tower crane", () => towerCrane(m, -W * 0.42, -D * 0.06, 26, 22, st.active, d0, (h % 8) * 45));
    }
    if (stage < 5) {
      m.part("site / huts and materials", () => {
        siteHuts(m, -W * 0.34, D * 0.3, 8 + (h % 5) * 4, 2, d0);
        materialStack(m, W * 0.30, -D * 0.30, stage <= 2 ? 2 : 0, d0);
      });
      if (!st.active) m.part("site / work stopped notice", () => stopBoard(m, 0, -D / 2 + 3.2, st.status === "blocked", d0));
    } else {
      m.part("yard / apron", () => {
        m.save().move(0, GRADE, 0);
        m.plate([[-W * 0.44, -D * 0.44], [W * 0.44, -D * 0.44], [W * 0.44, -D * 0.28], [-W * 0.44, -D * 0.28]], 0.1, P.asphalt);
        m.restore();
      });
      // `genericProps` names its own parts, so it is called at the top level:
      // a part opened inside another one would silently break the contiguous
      // ranges the picker indexes by.
      if (k.bespoke) {
        m.part("yard / loading dock and stack", () => {
          m.bar(x - 11, x + 11, GRADE, GRADE + 1.3, z - b.dz / 2 - 12, z - b.dz / 2 - 6, P.concrete);
          m.column(W * 0.28, GRADE, D * 0.26, 17, 0.95, 0.75, 4, shade(P.clad, 0.88), false);
        });
      } else {
        genericProps(m, k, x, z, W, D, d0, stage);
      }
      // The fixed hand-over dressing. Every kind gets the same, so a completed
      // site never comes out cheaper than the stage before it just because its
      // props happen to be plain.
      m.part("yard / hand-over dressing", () => {
        for (let i = 0; i < 4; i += 1) {
          m.bar(W * 0.20 + (i % 2) * 6.4, W * 0.20 + (i % 2) * 6.4 + 5.6, GRADE, GRADE + 2.5,
            D * 0.02 + Math.floor(i / 2) * 3.0 - 1.2, D * 0.02 + Math.floor(i / 2) * 3.0 + 1.2, shade(k.accent, 0.86 + 0.1 * (i % 3)));
        }
        m.bar(-W * 0.44, W * 0.44, GRADE + 0.1, GRADE + 0.32, -D * 0.29, -D * 0.27, P.kerb);
        m.bar(-W * 0.36, -W * 0.36 + 5.4, GRADE + 1.6, GRADE + 3.0, -D / 2 + 2.0, -D / 2 + 2.16, shade(k.accent, 1.12));
        for (let i = 0; i < 3; i += 1) lightMast(m, (i % 2 ? 1 : -1) * W * 0.40, (i < 2 ? 1 : -1) * D * 0.30, 11, d0);
      });
    }
    return { W, D };
  }

  Mesh.prototype.finish = function (info) {
    const positions = new Float32Array(this.pos);
    const colors = new Float32Array(this.col);
    const normals = new Float32Array(positions.length);
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
      triangleCount: positions.length / 9,
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
    const placeholder = !k.bespoke;
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
      + `${placeholder ? "; plain placeholder massing pending its own art pass" : ""}.`
    ;
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
      placeholder: !k.bespoke, retrofit: !!k.retrofit,
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
