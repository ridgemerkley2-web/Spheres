/* Original, component-driven game geometry. These shapes are visual concepts,
   not historical vehicle specifications. +Z is forward; Y=0 is the ground.

   Shading. Curved surfaces here are TURNED, not faceted: revolve() lays a
   profile of {r, h} rings around an axis and marks the rings that belong to a
   continuous surface with s:true. Those rings carry one normal per (ring,
   angle), computed from the profile and SHARED by every triangle that meets
   there, which is what makes the seam disappear. Rings without s keep the exact
   face normal of their own triangle, so a chamfer, a cap and a bolt head stay
   as hard as they were. tools/ui/check_equipment_mesh.cjs recounts both from
   the buffer and rejects a per-face skew that only looks smooth.

   Level of detail. build(spec) still returns the inspection mesh. spec.lod
   selects 0 (inspection), 1 (catalogue card) or 2 (map pin); the part list,
   its order, its slots and its labels are identical at every level so a
   selection survives the swap. */
(function (root, factory) {
  const api = factory();
  if (typeof module === "object" && module.exports) module.exports = api;
  else root.EquipmentMesh = api;
})(typeof globalThis !== "undefined" ? globalThis : this, function () {
  "use strict";

  const TAU = Math.PI * 2;
  const PALETTE = {
    hull: [0.29, 0.34, 0.22], upper: [0.35, 0.40, 0.27], edge: [0.40, 0.44, 0.31],
    shade: [0.22, 0.27, 0.18], armor: [0.32, 0.37, 0.25], steel: [0.20, 0.22, 0.20],
    bright: [0.39, 0.42, 0.38], track: [0.13, 0.15, 0.14], rubber: [0.055, 0.068, 0.060],
    black: [0.035, 0.045, 0.040], glass: [0.13, 0.33, 0.34], lens: [0.20, 0.44, 0.48],
    amber: [0.62, 0.37, 0.16], canvas: [0.30, 0.31, 0.21], cable: [0.25, 0.27, 0.24]
  };
  const DEFAULT_COMPONENTS = {
    mobility: "drive_standard", protection: "protection_standard", armament: "armament_standard",
    sensors: "sensors_optical", communications: "comms_radio"
  };
  const CHOICES = {
    mobility: ["drive_standard", "drive_mobile", "engine_diesel_600", "engine_diesel_900", "engine_diesel_1200", "engine_turbine_1500"],
    protection: ["protection_standard", "protection_heavy", "protection_active"],
    armament: ["armament_standard", "armament_heavy", "gun_90", "gun_105", "gun_120", "gun_125"],
    sensors: ["sensors_optical", "sensors_integrated", "optics_day", "optics_night", "optics_thermal"],
    communications: ["comms_radio", "comms_data"],
    transmission:["transmission_manual","transmission_auto"], tracks:["tracks_standard","tracks_wide","tracks_padded"],
    suspension:["suspension_torsion","suspension_hydro"], turret:["turret_compact","turret_standard","turret_heavy","turret_autoload","turret_casemate"],
    ammunition:["ammo_mixed","ammo_penetrator","ammo_support"], active_protection:["aps_none","aps_soft","aps_hard"],
    fire_control:["fcs_basic","fcs_stabilized","fcs_digital"]
  };
  const GROUND_CHOICES = {
    mobility:["ground_engine_750"], transmission:["ground_transmission_electric"], wheels:["ground_wheels_standard","ground_wheels_runflat"],
    turret:["ground_turret_autocannon","ground_station_mg","ground_turret_howitzer","ground_turret_aa"],
    armament:["ground_gun_25","ground_gun_35","ground_mg_127","ground_howitzer_122","ground_howitzer_155","ground_aa_gun","ground_aa_missiles"],
    ammunition:["ground_ammo_autocannon","ground_ammo_ball","ground_ammo_he","ground_ammo_guided","ground_ammo_aa","ground_ammo_missiles"],
    protection:["ground_armor_light","ground_armor_modular"], troop_compartment:["ground_troops_standard","ground_troops_protected"],
    recon_package:["ground_recon_observer","ground_recon_mast"], artillery_loader:["ground_loader_manual","ground_loader_assisted"],
    radar:["ground_radar_search","ground_radar_tracking"], communications:["ground_comms_network","ground_comms_secure"]
  };
  const GROUND_DEFAULTS = {
    ground_ifv:{turret:"ground_turret_autocannon",armament:"ground_gun_25",ammunition:"ground_ammo_autocannon",troop_compartment:"ground_troops_standard"},
    ground_apc:{turret:"ground_station_mg",armament:"ground_mg_127",ammunition:"ground_ammo_ball",wheels:"ground_wheels_standard",troop_compartment:"ground_troops_standard"},
    ground_recon:{turret:"ground_station_mg",armament:"ground_mg_127",ammunition:"ground_ammo_ball",wheels:"ground_wheels_standard",recon_package:"ground_recon_observer"},
    ground_artillery:{mobility:"engine_diesel_900",turret:"ground_turret_howitzer",armament:"ground_howitzer_122",ammunition:"ground_ammo_he",artillery_loader:"ground_loader_manual"},
    ground_air_defense:{turret:"ground_turret_aa",armament:"ground_aa_gun",ammunition:"ground_ammo_aa",radar:"ground_radar_search"}
  };
  // The tank ammunition loads name their own stowage. The label is the part's, so
  // it must read as a thing a crew stows and not as a catalogue id.
  const AMMO_LABEL = { ammo_mixed: "mixed-purpose", ammo_penetrator: "penetrator", ammo_support: "fire-support" };
  const add = (a, b) => [a[0] + b[0], a[1] + b[1], a[2] + b[2]];
  const sub = (a, b) => [a[0] - b[0], a[1] - b[1], a[2] - b[2]];
  const mul = (a, n) => [a[0] * n, a[1] * n, a[2] * n];
  const dot = (a, b) => a[0] * b[0] + a[1] * b[1] + a[2] * b[2];
  const cross = (a, b) => [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]];
  const normal = a => { const length = Math.hypot(...a); return length > 1e-10 ? mul(a, 1 / length) : [0, 1, 0]; };
  const mean = points => mul(points.reduce((sum, point) => add(sum, point), [0, 0, 0]), 1 / points.length);
  const shade = (color, amount) => color.map(value => Math.max(0, Math.min(1, value * amount)));

  // The three levels. segScale thins every turned surface, dense thins every
  // repeated run (track links, louvres, skirt panels), and cull drops a feature
  // whose largest dimension is under the threshold. cull is multiplied by the
  // hull length so a 4.45 m scout is not decimated as hard as a 6.55 m gun
  // carrier. retry relaxes the cull when a part would otherwise vanish, because
  // the part list has to be identical at every level; block thickens what
  // survives at LOD2 so the silhouette does not thin out around the gaps.
  const LEVELS = [
    { level: 0, segScale: 1,    segFloor: 6, cull: 0,     dense: 1 },
    { level: 1, segScale: 0.31, segFloor: 6, cull: 0.265, dense: 0.145 },
    { level: 2, segScale: 0.12, segFloor: 3, cull: 1.6,   dense: 0.03, retry: 0.50, block: 1.15 }
  ];
  const levelOf = spec => {
    const asked = Math.round(Number(spec && spec.lod));
    return LEVELS[Number.isFinite(asked) ? Math.min(LEVELS.length - 1, Math.max(0, asked)) : 0];
  };

  // A normal is stored as the face normal unless it departs from it by more
  // than this, so the recount in the contract can tell rounding from art.
  const SMOOTH_FLOOR = 6e-3;
  // A vertex normal never leaves its own winding hemisphere by more than this.
  const FACING_FLOOR = 0.44;
  // Two surfaces meeting at a sharper crease than this are a hard edge, not one
  // curved surface, and the ring between them keeps its face normals. cos 27.5
  // degrees each side: with the worst angular error a three-segment revolve can
  // carry (60 degrees) the product still clears FACING_FLOOR.
  const CREASE_FLOOR = 0.887;

  function createBuilder(detail, hull) {
    const D = detail || LEVELS[0];
    const span = Math.max(1, Number(hull) || 6) / 6;
    let cull = D.cull * span;
    const positions = [], normals = [], colors = [], parts = [], smoothing = [];
    const bounds = { min: [Infinity, Infinity, Infinity], max: [-Infinity, -Infinity, -Infinity] };
    let smoothed = 0;

    const seg = n => {
      const asked = Math.max(3, Math.round(Number(n) || 3));
      return Math.max(3, Math.min(asked, Math.max(D.segFloor, Math.round(asked * D.segScale))));
    };
    // A repeated run thins by dense, but never below `floor` on the catalogue
    // card, where a row of one reads as a mistake. The map pin takes `coarse`
    // instead, which is one unless the run is a closed loop that needs corners.
    const many = (n, floor = 1, coarse = 1) => {
      const bar = D.level >= 2 ? coarse : floor;
      return Math.max(Math.min(n, bar), Math.min(n, Math.round(n * D.dense)));
    };
    const keep = size => !(size < cull);
    // Every turned or extruded primitive measures itself and drops out when its
    // largest dimension is under the threshold, so a coarse level loses the
    // fittings and keeps the volumes without a hand-written rule per fitting.
    const small = size => cull > 0 && size < cull;
    const gauge = value => value * (D.block || 1);
    // Geometry that sets the vehicle's own extent is drawn at every level, size
    // or no size: a coarse level may lose fittings, but it may not lose reach.
    // The contract holds every level's bounds within 35 mm of the inspection
    // mesh, and this is what keeps a gun carrier's recoil spades inside it.
    const always = draw => { const original = cull; cull = 0; try { draw(); } finally { cull = original; } };

    function store(point, n, color) {
      positions.push(point[0], point[1], point[2]);
      normals.push(n[0], n[1], n[2]);
      colors.push(color[0], color[1], color[2]);
      for (let axis = 0; axis < 3; axis++) {
        bounds.min[axis] = Math.min(bounds.min[axis], point[axis]);
        bounds.max[axis] = Math.max(bounds.max[axis], point[axis]);
      }
    }
    function triangle(a, b, c, color) {
      const perpendicular = cross(sub(b, a), sub(c, a));
      if (dot(perpendicular, perpendicular) < 1e-18) return;
      const n = normal(perpendicular);
      for (const point of [a, b, c]) store(point, n, color);
    }
    // The one place a vertex is allowed a normal of its own. A wanted normal is
    // taken only when it has really left the face normal and still faces the
    // same way as the winding; otherwise the face normal is stored, so the
    // declared smoothing count is exactly what the buffer carries.
    function turned(a, b, c, color, na, nb, nc) {
      const perpendicular = cross(sub(b, a), sub(c, a));
      if (dot(perpendicular, perpendicular) < 1e-18) return;
      const face = normal(perpendicular);
      const points = [a, b, c], wanted = [na, nb, nc];
      for (let i = 0; i < 3; i++) {
        const want = wanted[i];
        let n = face;
        if (want) {
          const drift = Math.abs(want[0] - face[0]) + Math.abs(want[1] - face[1]) + Math.abs(want[2] - face[2]);
          if (drift > SMOOTH_FLOOR && dot(want, face) >= FACING_FLOOR) { n = want; smoothed++; }
        }
        store(points[i], n, color);
      }
    }
    function face(points, color, interior) {
      let ordered = points;
      if (interior && dot(cross(sub(points[1], points[0]), sub(points[2], points[0])), sub(mean(points), interior)) < 0) ordered = [...points].reverse();
      for (let i = 1; i < ordered.length - 1; i++) triangle(ordered[0], ordered[i], ordered[i + 1], color);
    }
    function part(name, draw, slot) {
      const first = positions.length / 3, before = smoothed;
      draw();
      // The part table is the same at every level, so a part that culled itself
      // away gets its cull threshold relaxed until it has something to show.
      // Every culling level needs this, not only the one that names a factor:
      // a part that disappears is a part whose name, slot and label disappear
      // with it, and picking has to survive the swap.
      if (positions.length / 3 === first && cull > 0) {
        const original = cull, relax = D.retry || 0.5;
        for (let attempt = 0; attempt < 10 && positions.length / 3 === first; attempt++) { cull *= relax; draw(); }
        cull = original;
      }
      const count = positions.length / 3 - first;
      if (count) {
        const prefix=name.split(' / ')[0];
        slot=slot||(prefix==='running gear'?(name.includes('suspension')?'suspension':'tracks'):prefix==='chassis'?(name.includes('periscopes')?'sensors':'protection'):prefix==='stowage'?'ammunition':prefix);
        parts.push({ name, first, count, slot, label:name.split(' / ').slice(1).join(' / ')||name });
        if (smoothed > before) smoothing.push({ name, first, count, vertices: smoothed - before });
      }
    }
    function box(center, size, color, basis) {
      if (small(Math.hypot(size[0], size[1], size[2]))) return;
      const axes = basis || [[1, 0, 0], [0, 1, 0], [0, 0, 1]];
      const corner = (x, y, z) => add(center, add(mul(axes[0], x * size[0] / 2), add(mul(axes[1], y * size[1] / 2), mul(axes[2], z * size[2] / 2))));
      const p = [corner(-1, -1, -1), corner(1, -1, -1), corner(1, 1, -1), corner(-1, 1, -1),
        corner(-1, -1, 1), corner(1, -1, 1), corner(1, 1, 1), corner(-1, 1, 1)];
      for (const indices of [[0, 1, 2, 3], [4, 7, 6, 5], [0, 4, 5, 1], [3, 2, 6, 7], [0, 3, 7, 4], [1, 5, 6, 2]]) face(indices.map(i => p[i]), color, center);
    }
    // Convex cross sections create continuous bevels and sloped armor faces.
    function loft(rings, color) {
      if (cull > 0) {
        const low = [Infinity, Infinity, Infinity], high = [-Infinity, -Infinity, -Infinity];
        for (const point of rings.flat()) for (let axis = 0; axis < 3; axis++) { low[axis] = Math.min(low[axis], point[axis]); high[axis] = Math.max(high[axis], point[axis]); }
        if (small(Math.hypot(high[0] - low[0], high[1] - low[1], high[2] - low[2]))) return;
      }
      // At the map-pin level a five-ring casting and a three-ring one are the
      // same shape. Keep the two ends and the widest section between them, so
      // the silhouette that survives is the one the eye actually reads.
      if (D.level >= 2 && rings.length > 3) {   // eslint-disable-line no-param-reassign
        let widest = 1, best = -1;
        for (let level = 1; level < rings.length - 1; level++) {
          let reach = 0;
          for (const point of rings[level]) reach = Math.max(reach, Math.abs(point[0]) + Math.abs(point[2]));
          if (reach > best) { best = reach; widest = level; }
        }
        rings = [rings[0], rings[widest], rings[rings.length - 1]];
      }
      const center = mean(rings.flat());
      face(rings[0], shade(color, 0.84), center);
      face(rings[rings.length - 1], shade(color, 1.08), center);
      for (let level = 0; level < rings.length - 1; level++) {
        for (let i = 0; i < rings[level].length; i++) {
          const j = (i + 1) % rings[level].length;
          face([rings[level][i], rings[level][j], rings[level + 1][j], rings[level + 1][i]], color, center);
        }
      }
    }
    // The turned surface. profile entries are {r, h, s?, c?}: r is the radius
    // about the axis, h the distance along it from origin, s marks a ring whose
    // normals are smoothed, c recolours the band that starts at that ring.
    function revolve(origin, axis, profile, color, segments = 24, extent) {
      const rings = profile.filter(ring => ring && Number.isFinite(ring.r) && Number.isFinite(ring.h));
      if (rings.length < 2) return;
      if (cull > 0) {
        let wide = 0, low = Infinity, high = -Infinity;
        for (const ring of rings) { wide = Math.max(wide, ring.r); low = Math.min(low, ring.h); high = Math.max(high, ring.h); }
        if (small(extent !== undefined ? extent : Math.hypot(wide * 2, high - low))) return;
      }
      const a = normal(axis);
      const u = normal(cross(a, Math.abs(a[1]) > 0.9 ? [1, 0, 0] : [0, 1, 0])), v = cross(a, u);
      const S = seg(segments);
      // Band k runs from ring k to ring k+1. Its surface normal is constant in
      // the (radial, axial) frame, so it is stored once and rotated per angle.
      const bands = [];
      for (let k = 0; k < rings.length - 1; k++) {
        const dh = rings[k + 1].h - rings[k].h, dr = rings[k + 1].r - rings[k].r;
        const length = Math.hypot(dh, dr);
        if (length < 1e-9 || (rings[k].r < 1e-9 && rings[k + 1].r < 1e-9)) { bands.push(null); continue; }
        bands.push({ radial: dh / length, axial: -dr / length });
      }
      const ringNormal = rings.map((ring, k) => {
        if (!ring.s) return null;
        const near = [k > 0 ? bands[k - 1] : null, k < bands.length ? bands[k] : null].filter(Boolean);
        if (!near.length) return null;
        let radial = 0, axial = 0;
        for (const band of near) { radial += band.radial; axial += band.axial; }
        const length = Math.hypot(radial, axial);
        if (length < 1e-6) return null;
        radial /= length; axial /= length;
        for (const band of near) if (band.radial * radial + band.axial * axial < CREASE_FLOOR) return null;
        return { radial, axial };
      });
      const base = add(origin, [0, 0, 0]);
      const radials = [], point = (ring, i) => add(add(base, mul(a, ring.h)), mul(radials[i], ring.r));
      for (let i = 0; i < S; i++) {
        const theta = TAU * i / S;
        radials.push(add(mul(u, Math.cos(theta)), mul(v, Math.sin(theta))));
      }
      const at = (k, i) => {
        const spin = ringNormal[k];
        return spin ? normal(add(mul(radials[i], spin.radial), mul(a, spin.axial))) : null;
      };
      for (let k = 0; k < bands.length; k++) {
        if (!bands[k]) continue;
        const lower = rings[k], upper = rings[k + 1], tint = rings[k].c || color;
        for (let i = 0; i < S; i++) {
          const j = (i + 1) % S;
          const a0 = point(lower, i), a1 = point(lower, j), b0 = point(upper, i), b1 = point(upper, j);
          const na0 = at(k, i), na1 = at(k, j), nb0 = at(k + 1, i), nb1 = at(k + 1, j);
          if (lower.r < 1e-9) turned(a0, b1, b0, tint, na0, nb1, nb0);
          else if (upper.r < 1e-9) turned(a0, a1, b1, tint, na0, na1, nb1);
          else { turned(a0, a1, b1, tint, na0, na1, nb1); turned(a0, b1, b0, tint, na0, nb1, nb0); }
        }
      }
    }
    // 4S triangles capped, 2S open. The wall is one continuous surface; the two
    // caps meet it at a hard rim, which is why they are separate rings.
    function cylinder(a, b, radius, color, segments = 24, radiusEnd = radius, caps = true) {
      const axis = sub(b, a), length = Math.hypot(...axis);
      if (length < 1e-9) return;
      const profile = [];
      if (caps && radius > 1e-9) profile.push({ r: 0, h: 0 }, { r: radius, h: 0 });
      profile.push({ r: radius, h: 0, s: 1 }, { r: radiusEnd, h: length, s: 1 });
      if (caps && radiusEnd > 1e-9) profile.push({ r: radiusEnd, h: length }, { r: 0, h: length });
      revolve(a, axis, profile, color, segments, Math.hypot(Math.max(radius, radiusEnd) * 2, length));
    }
    // A bore. 8S triangles: the outer wall and the inner wall are both turned,
    // the two annular rims between them stay flat.
    function tube(a, b, radius, innerRadius, color, segments = 32) {
      const axis = sub(b, a), length = Math.hypot(...axis);
      if (length < 1e-9) return;
      const reach = Math.hypot(radius * 2, length);
      revolve(a, axis, [{ r: radius, h: 0, s: 1 }, { r: radius, h: length, s: 1 }], color, segments, reach);
      revolve(a, axis, [{ r: radius, h: length }, { r: innerRadius, h: length }], PALETTE.bright, segments, reach);
      revolve(a, axis, [{ r: innerRadius, h: length, s: 1 }, { r: innerRadius, h: 0, s: 1 }], PALETTE.black, segments, reach);
      revolve(a, axis, [{ r: innerRadius, h: 0 }, { r: radius, h: 0 }], color, segments, reach);
    }
    // A blind socket: lifting eye, tie-down, tow fitting, filler cap. 7S. It is
    // the fitting that reads at card size wherever a bare rod used to sit.
    function socket(a, b, radius, bore, color, segments = 18) {
      const axis = sub(b, a), length = Math.hypot(...axis);
      if (length < 1e-9) return;
      const reach = Math.hypot(radius * 2, length);
      revolve(a, axis, [{ r: radius, h: 0, s: 1 }, { r: radius, h: length, s: 1 }], color, segments, reach);
      revolve(a, axis, [{ r: radius, h: length }, { r: bore, h: length }], color, segments, reach);
      revolve(a, axis, [{ r: bore, h: length, s: 1 }, { r: bore, h: length * 0.25, s: 1 }], color, segments, reach);
      revolve(a, axis, [{ r: bore, h: length * 0.25 }, { r: 0, h: length * 0.25 }], PALETTE.track, segments, reach);
    }
    function rod(a, b, radius, color, segments = 10) { cylinder(a, b, radius, color, segments); }
    // Fastener heads, six triangles each, laid on a face rather than sunk into
    // it. They are the smallest thing on the model that survives to LOD1.
    function studs(center, axis, radius, count, size, rise, color, force) {
      if (!force && small(size * 2)) return;
      const a = normal(axis);
      const u = normal(cross(a, Math.abs(a[1]) > 0.9 ? [1, 0, 0] : [0, 1, 0])), v = cross(a, u);
      for (let i = 0; i < count; i++) {
        const theta = TAU * i / count;
        const seat = add(center, mul(add(mul(u, Math.cos(theta)), mul(v, Math.sin(theta))), radius));
        revolve(seat, a, [{ r: size, h: 0, s: 1 }, { r: 0, h: rise }], color, 6, Infinity);
      }
    }
    // A road wheel or road tyre: tyre, dished rim, hub and nuts as separate
    // turned surfaces. At the map-pin level the turned surfaces collapse into
    // the tyre alone, which is the whole of the wheel that a pin can show and
    // the reason eight run-flats still fit the coarse budget.
    function wheelBody(center, axis, radius, halfWidth, tyre, rim, hub, dual) {
      const a = normal(axis);
      // THE MAP PIN. Segment counts are already on the floor at LOD2, so the
      // only saving left on a wheel is fewer turned SURFACES, not thinner ones:
      // a tyre, and nothing else. That is what keeps an eight-wheel run-flat
      // scout inside the coarse budget where the old five-surface wheel put it
      // 264 triangles over the ceiling.
      if (D.level >= 2) {
        revolve(add(center, mul(a, -halfWidth)), a, [
          { r: radius * 0.80, h: 0, s: 1 }, { r: radius, h: halfWidth * 0.6, s: 1 },
          { r: radius, h: halfWidth * 1.4, s: 1 }, { r: radius * 0.80, h: halfWidth * 2, s: 1 }
        ], tyre, 20, Infinity);
        return;
      }
      // The tyre: crown, two shoulders and two bead flanks, one continuous
      // turned surface from bead to bead.
      revolve(add(center, mul(a, -halfWidth)), a, [
        { r: radius * 0.72, h: 0, s: 1 }, { r: radius * 0.955, h: halfWidth * 0.36, s: 1 },
        { r: radius, h: halfWidth * 0.76, s: 1 }, { r: radius, h: halfWidth * 1.24, s: 1 },
        { r: radius * 0.955, h: halfWidth * 1.64, s: 1 }, { r: radius * 0.72, h: halfWidth * 2, s: 1 }
      ], tyre, 20, Infinity);
      for (const side of dual ? [-1, 1] : [1]) {
        const outer = add(center, mul(a, side * halfWidth)), inward = mul(a, -side);
        if (dual) {
          // A road wheel is a pair of pressed discs bolted back to back, so both
          // faces are turned and both carry a hub and its nuts.
          revolve(outer, inward, [
            { r: 0, h: -0.001 }, { r: radius * 0.760, h: -0.001 },
            { r: radius * 0.810, h: 0.028, s: 1 }, { r: 0, h: 0.028 }
          ], rim, 18, Infinity);
          revolve(outer, mul(a, side), [
            { r: 0, h: 0.002 }, { r: radius * 0.442, h: 0.002 }, { r: radius * 0.376, h: 0.023, s: 1 },
            { r: radius * 0.191, h: 0.036, s: 1 }, { r: radius * 0.155, h: 0.054, s: 1 }, { r: 0, h: 0.062 }
          ], hub, 14, Infinity);
          if (D.level < 1) studs(outer, mul(a, side), radius * 0.621, 8, radius * 0.078, 0.030, PALETTE.bright);
          continue;
        }
        // A road tyre on a rim: one dished face with the hub turned into it, a
        // plain cone behind, the wheel nuts, and the valve stem.
        revolve(outer, inward, [
          { r: 0, h: 0 }, { r: radius * 0.210, h: radius * 0.035, s: 1 }, { r: radius * 0.260, h: radius * 0.087, s: 1 },
          { r: radius * 0.300, h: radius * 0.130 }, { r: radius * 0.570, h: radius * 0.130 },
          { r: radius * 0.570, h: radius * 0.209 }, { r: radius * 0.720, h: radius * 0.209 },
          { r: radius * 0.720, h: radius * 0.265 }
        ], rim, 14, Infinity);
        const back = add(center, mul(a, -halfWidth));
        revolve(back, a, [{ r: radius * 0.720, h: 0 }, { r: radius * 0.620, h: radius * 0.087, s: 1 }, { r: 0, h: radius * 0.104 }], hub, 14, Infinity);
        studs(add(outer, mul(a, -side * radius * 0.120)), mul(a, side), radius * 0.390, 8, radius * 0.065, 0.034, PALETTE.bright, true);
        if (D.level < 1) revolve(add(outer, mul(a, -side * radius * 0.150)), mul(a, side), [
          { r: 0.020, h: 0, s: 1 }, { r: 0.020, h: 0.030, s: 1 }, { r: 0.012, h: 0.038 }
        ], PALETTE.black, 5, Infinity);
      }
    }
    return {
      level: D.level, seg, many, keep, gauge, always, dense: D.dense,
      triangle, face, part, box, loft, cylinder, tube, socket, rod, revolve, studs, wheelBody,
      finish(description) {
        // Flat tread shoes meet a curved belt at their corners. Seat the actual
        // lowest vertex on the ground, rather than clipping the shoe geometry.
        const ground = bounds.min[1];
        for (let i = 1; i < positions.length; i += 3) positions[i] -= ground;
        bounds.max[1] -= ground; bounds.min[1] = 0;
        return { positions: new Float32Array(positions), normals: new Float32Array(normals), colors: new Float32Array(colors),
          bounds, parts, smoothing, lod: D.level, triangleCount: positions.length / 9, description };
      }
    };
  }
  function octagon(y, width, rear, front, bevel, offset = 0) {
    return [[-width + bevel, y, rear + offset], [width - bevel, y, rear + offset], [width, y, rear + bevel + offset],
      [width, y, front - bevel + offset], [width - bevel, y, front + offset], [-width + bevel, y, front + offset],
      [-width, y, front - bevel + offset], [-width, y, rear + bevel + offset]];
  }
  // A chamfered prism reads as a machined fitting where a raw box reads as a toy:
  // the cut edge catches its own flat normal and draws a highlight line. It costs
  // 28 triangles against box()'s 12, so it is spent on volumes big enough to see
  // on a 90px card and never on fasteners. The chamfer runs along the basis Z.
  function beveled(b, center, size, color, cut, basis) {
    const axes = basis || [[1, 0, 0], [0, 1, 0], [0, 0, 1]];
    const hx = size[0] / 2, hy = size[1] / 2, k = Math.min(cut, hx * 0.8, hy * 0.8);
    const section = [[-hx + k, -hy], [hx - k, -hy], [hx, -hy + k], [hx, hy - k],
      [hx - k, hy], [-hx + k, hy], [-hx, hy - k], [-hx, -hy + k]];
    const ring = end => section.map(([u, v]) => add(center, add(mul(axes[0], u), add(mul(axes[1], v), mul(axes[2], end * size[2] / 2)))));
    b.loft([ring(-1), ring(1)], color);
  }
  // Restrained wear, varied by loop index so the same input still gives the same
  // bytes. shade() scales all three channels together, so a weathered panel keeps
  // the green dominance the sand/winter repaint heuristic keys on.
  const weathered = (color, index) => shade(color, 0.945 + 0.05 * (index % 3) + 0.022 * Math.abs(Math.sin(index * 1.7)));
  function resolveSpec(spec) {
    const input = spec && typeof spec === "object" ? spec : {}, components = {};
    if (Object.hasOwn(GROUND_DEFAULTS,input.platform)) {
      const defaults={mobility:"engine_diesel_600",transmission:"transmission_manual",tracks:"tracks_standard",suspension:"suspension_torsion",protection:"ground_armor_light",active_protection:"aps_none",sensors:"optics_day",fire_control:"fcs_basic",communications:"comms_radio",...GROUND_DEFAULTS[input.platform]};
      if(defaults.wheels)delete defaults.tracks;
      for(const [slot,fallback] of Object.entries(defaults)) {
        const choices=[...(CHOICES[slot]||[]),...(GROUND_CHOICES[slot]||[])];
        components[slot]=choices.includes(input.components?.[slot])?input.components[slot]:fallback;
      }
      return {platform:input.platform,components};
    }
    for (const [slot, choices] of Object.entries(CHOICES)) components[slot] = choices.includes(input.components?.[slot]) ? input.components[slot] : DEFAULT_COMPONENTS[slot];
    return { platform: ["tank_standard","tank_heavy","tank_light","tank_destroyer"].includes(input.platform) ? input.platform : "tank_standard", components };
  }

  // Specialist chassis share fittings and construction primitives, but their hulls,
  // running gear, weapon mounts and mission installations are separate geometry.
  function buildGround(spec, detail) {
    const s=spec.components,c=PALETTE;
    const scout=spec.platform==='ground_recon',carrier=spec.platform==='ground_apc',ifv=spec.platform==='ground_ifv',artillery=spec.platform==='ground_artillery',aa=spec.platform==='ground_air_defense';
    const wheeled=carrier||scout,w=scout?1.03:carrier?1.16:artillery?1.43:1.28,length=scout?4.45:carrier?5.50:artillery?6.55:5.75;
    const b=createBuilder(detail,length);
    const rear=-length/2,front=length/2,deck=scout?1.67:carrier?1.98:ifv?1.78:1.52;
    const modular=s.protection==='ground_armor_modular',hydro=s.suspension==='suspension_hydro';
    const mountZ=artillery?-.63:aa?.12:scout?.15:.73;
    const mg=s.turret==='ground_station_mg',howitzer=s.turret==='ground_turret_howitzer',aaMount=s.turret==='ground_turret_aa';
    const mountTop=deck+(mg?.37:howitzer?1.42:aaMount?.91:.76),gunY=mountTop-(howitzer?.52:mg?.08:.24);
    const labels={engine_diesel_600:'compact diesel',engine_diesel_900:'standard diesel',engine_diesel_1200:'high-output diesel',ground_engine_750:'managed diesel',ground_gun_25:'25 mm autocannon',ground_gun_35:'35 mm autocannon',ground_mg_127:'12.7 mm machine gun',ground_howitzer_122:'122 mm howitzer',ground_howitzer_155:'155 mm howitzer',ground_aa_gun:'twin air-defense cannon',ground_aa_missiles:'short-range missile launcher',ground_ammo_autocannon:'autocannon',ground_ammo_ball:'machine gun',ground_ammo_he:'artillery',ground_ammo_guided:'guided artillery',ground_ammo_aa:'air-defense cannon',ground_ammo_missiles:'missile',optics_day:'daylight',optics_night:'night',optics_thermal:'thermal',fcs_basic:'basic',fcs_stabilized:'stabilized',fcs_digital:'digital',comms_radio:'field radio',comms_data:'tactical data',ground_comms_secure:'secure radio',ground_comms_network:'networked command',aps_soft:'soft-kill',aps_hard:'active interception'};
    const label=id=>labels[id]||String(id).replace(/^ground_/,'').replace(/_/g,' ');
    function hatch(x,y,z,r=.25) {
      // A stepped seating ring and a domed lid, which is the shape a pressed
      // hatch actually has and the reason it catches a highlight from above.
      b.revolve([x,y,z],[0,1,0],[{r:0,h:0},{r:r,h:0},{r:r*1.02,h:.030,s:1},{r:r*.97,h:.048}],c.shade,24,Infinity);
      b.revolve([x,y+.046,z],[0,1,0],[{r:r*.955,h:0},{r:r*.955,h:.020,s:1},{r:r*.905,h:.046,s:1},
        {r:r*.760,h:.068,s:1},{r:r*.420,h:.082,s:1},{r:0,h:.088}],c.upper,20,Infinity);
      if(b.keep(0.30)) {
        b.rod([x-r*.30,y+.13,z],[x+r*.30,y+.13,z],.021,c.bright,10);
        b.studs([x,y+.048,z],[0,1,0],r*.99,8,r*.075,.022,c.bright);
      }
    }
    function wheel(side,z,r,x,width,label) {
      b.part(`running gear / ${label}`,()=>{
        b.wheelBody([side*x,r,z],[1,0,0],r,width/2,c.rubber,c.upper,c.shade,false);
        // A smooth cylinder is the fastest way to make a wheeled hull look
        // unfinished. Two shoulder rows, a centre row staggered off their pitch,
        // and a bead ring on each sidewall.
        if(wheeled&&b.keep(0.30)) {
          const shoulders=b.many(32,8),centres=b.many(16,4);
          for(let j=0;j<shoulders;j++) {
            const a=TAU*j/shoulders,radial=[0,Math.cos(a),Math.sin(a)],tangent=[0,-Math.sin(a),Math.cos(a)];
            for(const strip of [-1,1])b.box([side*x+strip*width*.23,r+Math.cos(a)*(r+.006),z+Math.sin(a)*(r+.006)],[width*.43,b.gauge(.035),.075],c.track,[[1,0,0],radial,tangent]);
          }
          for(let j=0;j<centres;j++) {
            const a=TAU*(j+.5)/centres,radial=[0,Math.cos(a),Math.sin(a)],tangent=[0,-Math.sin(a),Math.cos(a)];
            b.box([side*x,r+Math.cos(a)*(r+.004),z+Math.sin(a)*(r+.004)],[width*.30,b.gauge(.031),.11],c.rubber,[[1,0,0],radial,tangent]);
          }
          for(const bead of [-1,1])b.cylinder([side*x+bead*width*.47,r,z],[side*x+bead*width*.50,r,z],r*.985,c.black,24);
        }
      },wheeled?'wheels':'tracks');
    }
    b.part('protection / specialist sloped hull',()=>{
      // Two extra rings for the price of 32 triangles. The old three-ring loft ran
      // one unbroken plane from the belly to the deck, which is what made this hull
      // read as a wedge of soap next to the tank's folded plate.
      const shoulder=.94+(deck-.94)*.42;
      b.loft([octagon(.44,w-.24,rear+.25,front-.44,.25),octagon(.78,w-.06,rear+.06,front-.11,.26),
        octagon(.94,w,rear,front,.27),octagon(shoulder,w-.045,rear+.03,front-.24,.29),
        octagon(deck,w-.14,rear+.10,front-.62,.30)],c.hull);
      b.loft([octagon(deck,w-.14,rear+.10,front-.62,.30),octagon(deck+.065,w-.20,rear+.16,front-.69,.28)],c.upper);
      // Splash plate and bolted nose beam: a wheeled hull needs a hard line where
      // the glacis meets the deck or the whole front is one continuous slope.
      b.box([0,deck-.10,front-.60],[w*1.62,.055,.19],c.edge);
      beveled(b,[0,1.03,front-.02],[w*1.52,.22,.16],c.armor,.05);
      if(b.keep(0.30))for(let i=-3;i<=3;i++)b.box([i*w*.32,1.03,front+.055],[.05,.05,.03],c.bright);
      for(const side of [-1,1]) {
        b.rod([side*w*.44,.86,front-.04],[side*w*.44,.86,front+.14],.055,c.bright,12);
        b.box([side*(w-.055),shoulder,0],[.05,.045,length-.70],c.shade);
        beveled(b,[side*(w+.06),wheeled?1.33:1.18,0],[.25,.075,length-.40],c.edge,.028);
        for(const z of [rear+.21,front-.12]) {
          b.rod([side*w*.66,.74,z],[side*w*.66,.74,z+(z>0?.16:-.16)],.063,c.bright,12);
          if(b.keep(0.30)) {
            b.box([side*w*.77,1.08,z],[.15,.14,.07],c.shade);
            b.cylinder([side*w*.77,1.08,z],[side*w*.77,1.08,z+(z>0?.055:-.055)],.045,z>0?c.lens:c.amber,12);
          }
          if(z>0&&b.keep(0.30)) {
            for(const dx of [-.085,.085])b.rod([side*w*.77+dx,1.00,z+.09],[side*w*.77+dx,1.18,z+.09],.012,c.steel,6);
            b.rod([side*w*.77-.085,1.18,z+.09],[side*w*.77+.085,1.18,z+.09],.012,c.steel,6);
          }
        }
        // Tow fittings on the hull corners, and a filler cap on the sponson. The
        // socket is the fitting a bare rod could never be: it has a rim.
        if(b.keep(0.30))for(const z of [rear+.30,front-.36])
          b.socket([side*(w-.10),.98,z],[side*(w+.12),.98,z],.085,.050,c.steel,18);
        if(modular)for(let j=0;j<b.many(6,2);j++) {
          const j6=j*6/b.many(6,2),z=rear+.58+j6*(length-.97)/6;
          b.box([side*(w+.065),deck-.27,z],[.17,.44,(length-1.10)/6],c.armor);
          if(b.keep(0.30))for(const dz of [-.15,.15])b.cylinder([side*(w+.15),deck-.13,z+dz],[side*(w+.18),deck-.13,z+dz],.027,c.bright,6);
        }
      }
      // The cast transition between the glacis and the lower plate, turned
      // rather than folded: on a real hull this is one poured casting.
      if(b.keep(0.30))b.revolve([0,1.02,front-.30],[1,0,0],[
        {r:.26,h:-w*.98},{r:.34,h:-w*.72,s:1},{r:.38,h:-w*.30,s:1},
        {r:.38,h:w*.30,s:1},{r:.34,h:w*.72,s:1},{r:.26,h:w*.98}],c.armor,24);
      if(modular)b.box([0,1.17,front-.22],[w*1.37,.22,.20],c.armor);
    });
    for(const side of [-1,1]) {
      const lane=side<0?'port':'starboard';
      const x=w+(wheeled?.02:.09),width=wheeled?.40:(s.tracks==='tracks_wide'?.60:.45),r=wheeled?.56:.46;
      const half=length/2-(wheeled?.69:.63),count=wheeled?(s.wheels==='ground_wheels_runflat'?4:3):artillery?7:6;
      for(let j=0;j<count;j++)wheel(side,-half+2*half*j/(count-1),r,x,width,`${lane} ${wheeled?'road tire':'road wheel'} ${j+1}`);
      b.part(`suspension / ${lane} ${hydro?'hydropneumatic struts':'torsion arms'}`,()=>{
        for(let j=0;j<count;j++) {
          const z=-half+2*half*j/(count-1);
          b.rod([side*(w-.30),.87,z-.15],[side*x,r,z],b.gauge(hydro?.066:.044),c.steel,12);
          if(hydro)b.cylinder([side*(w-.24),.80,z-.12],[side*(x-.10),r+.10,z-.025],.104,c.bright,14);
          // Trailing arm, damper and the bearing housing it swings on. A single
          // rod per wheel is what made the underside read as scaffolding.
          if(b.keep(0.30)) {
            b.cylinder([side*(w-.20),.94,z-.10],[side*(w-.04),.94,z-.10],.088,c.shade,12);
            b.revolve([side*(w-.05),.94,z-.10],[side,0,0],[
              {r:.052,h:0,s:1},{r:.052,h:.10,s:1},{r:.030,h:.13,s:1},{r:0,h:.15}],c.bright,12);
            b.cylinder([side*(w-.16),.88,z+.10],[side*(x-.12),r+.16,z+.03],.046,c.steel,6);
            b.box([side*(w-.10),.99,z-.10],[.13,.16,.14],c.steel);
            // A steered axle needs a track rod; a torsion bar does not.
            if(wheeled)b.cylinder([side*(w-.12),.70,z],[side*(x-.06),.70,z+.19],.038,c.bright,12);
          }
        }
      });
      // Arches, not a flat slab with tyres poking out. On a wheeled hull this is
      // the single largest silhouette gain available: the sweep is kept under the
      // axle pitch so four-axle kit does not fuse its arches into one blister.
      b.part(`protection / ${lane} flank stowage, rails and lifting eyes`,()=>{
        // The long locker only appears where modular armour is not fitted: on this
        // hull the two want the same 340 mm of flank, and the armour wins. Rails
        // and eyes sit on the deck edge above both, so they are always present.
        const railX=side*(w-.17);
        if(!modular) {
          const binY=(wheeled?1.3675:1.2175)+.18;
          beveled(b,[side*(w+.07),binY,-length*.12],[.24,.34,length*.44],weathered(c.hull,side>0?1:2),.06);
          b.box([side*(w+.07),binY+.185,-length*.12],[.27,.028,length*.44+.03],c.edge);
          if(b.keep(0.30))for(let j=0;j<4;j++)b.box([side*(w+.19),binY,-length*.12+(j-1.5)*length*.10],[.03,.10,.05],c.bright);
        }
        if(b.keep(0.30))for(let j=0;j<b.many(3,1);j++) {
          const z=rear+.85+j*(length-1.9)/2;
          b.rod([railX,deck+.07,z-.16],[railX,deck+.16,z-.16],.017,c.steel,6);
          b.rod([railX,deck+.07,z+.16],[railX,deck+.16,z+.16],.017,c.steel,6);
          b.rod([railX,deck+.16,z-.16],[railX,deck+.16,z+.16],.017,c.steel,6);
        }
        if(b.keep(0.30))for(const z of [rear+.60,front-.80])b.box([side*(w-.06),deck-.02,z],[.13,.09,.10],c.steel);
        if(b.keep(0.30))for(let j=0;j<2;j++)b.rod([side*(w-.06),deck-.16-j*.14,front-1.55],[side*(w-.06),deck-.16-j*.14,front-.78],.024,j?c.steel:c.cable,8);
      });
      if(wheeled)b.part(`running gear / ${lane} wheel arches and mud flaps`,()=>{
        const R=r+.14,arc=b.many(6,3);
        for(let j=0;j<count;j++) {
          const z=-half+2*half*j/(count-1);
          for(let k=0;k<arc;k++) {
            const t0=Math.PI*(.16+.68*k/arc),t1=Math.PI*(.16+.68*(k+1)/arc),t=(t0+t1)/2;
            const radial=[0,Math.sin(t),Math.cos(t)],tangent=[0,Math.cos(t),-Math.sin(t)],span=R*(t1-t0)*1.08;
            const cy=r+Math.sin(t)*R,cz=z+Math.cos(t)*R;
            b.box([side*x,cy,cz],[width+.20,b.gauge(.07),span],weathered(c.upper,j+k),[[1,0,0],radial,tangent]);
            b.box([side*(x+width/2+.085),cy,cz],[b.gauge(.05),.13,span],c.edge,[[1,0,0],radial,tangent]);
          }
          // The flap hangs from the arch's trailing end, not from thin air: it has
          // to reach that exact point or it reads as a detached black rectangle.
          b.box([side*x,r*.89,z-R+.085],[width+.10,r*1.42,b.gauge(.035)],c.rubber);
          if(b.keep(0.30))b.rod([side*(x-width*.4),r*1.55,z-R+.06],[side*(x+width*.4),r*1.55,z-R+.06],.020,c.steel,6);
        }
      },'wheels');
      if(!wheeled)b.part(`tracks / ${lane} continuous articulated belt`,()=>{
        const radius=.53,centerY=.53,span=half,perimeter=span*4+TAU*radius,steps=b.many(Math.ceil(perimeter/.18),12,8);
        function point(distance,rr) {
          let t=distance%perimeter;
          if(t<2*span)return {y:centerY+rr,z:span-t,tangent:[0,0,-1]};t-=2*span;
          if(t<Math.PI*radius){const a=Math.PI/2+t/radius;return {y:centerY+rr*Math.sin(a),z:-span+rr*Math.cos(a),tangent:[0,Math.cos(a),-Math.sin(a)]};}t-=Math.PI*radius;
          if(t<2*span)return {y:centerY-rr,z:-span+t,tangent:[0,0,1]};t-=2*span;
          const a=-Math.PI/2+t/radius;return {y:centerY+rr*Math.sin(a),z:span+rr*Math.cos(a),tangent:[0,Math.cos(a),-Math.sin(a)]};
        }
        for(let j=0;j<steps;j++) {
          const p=point(perimeter*j/steps,radius),q=point(perimeter*(j+1)/steps,radius),a=point(perimeter*j/steps,radius-.07),d=point(perimeter*(j+1)/steps,radius-.07);
          const inner=[side*x,a.y,a.z];
          for(const edge of [-1,1])b.face([[side*x+edge*width/2,p.y,p.z],[side*x+edge*width/2,q.y,q.z],[side*x+edge*width/2,d.y,d.z],[side*x+edge*width/2,a.y,a.z]],c.track,inner);
          b.face([[side*x-width/2,p.y,p.z],[side*x+width/2,p.y,p.z],[side*x+width/2,q.y,q.z],[side*x-width/2,q.y,q.z]],c.track,inner);
          const radial=cross([1,0,0],p.tangent);
          b.box([side*x,p.y,p.z],[width+.025,b.gauge(.048),perimeter/steps*.82],c.steel,[[1,0,0],radial,p.tangent]);
          // The shoe face. A padded track is not a steel grouser in a different
          // colour — it is a rubber block that covers nearly the whole shoe and
          // stands twice as proud, which is why a padded hull sits higher on its
          // own belt and reads with a softer edge. The tank belt has drawn the
          // difference this way since it was written; this is the specialist
          // hulls catching up with it, and it is the only change here that moves
          // the whole vehicle rather than adding a fitting to it.
          const padded=s.tracks==='tracks_padded';
          b.box([side*x,p.y+radial[1]*(padded?.049:.032),p.z+radial[2]*(padded?.049:.032)],
            [width*(padded?.94:.63),b.gauge(padded?.064:.034),padded?.105:.07],padded?c.rubber:c.bright,[[1,0,0],radial,p.tangent]);
        }
      });
    }
    b.part(`mobility / ${label(s.mobility)} engine deck`,()=>{
      const z=front-1.12,x=w*.44,managed=s.mobility==='ground_engine_750',power=s.mobility==='engine_diesel_1200';
      b.box([x,deck+.105,z],[.66,.07,.75],c.shade);
      const vents=power?14:managed?11:s.mobility==='engine_diesel_900'?9:7;
      if(b.keep(0.30))for(let i=0;i<b.many(vents,3);i++)b.box([x,deck+.151,z-.31+i*.62/(b.many(vents,3)-1)],[.57,.022,.027],c.bright);
      // Framed grille rather than louvres painted on a lid. From above, the deck
      // is most of what a card crop shows of a low hull.
      for(const dx of [-.315,.315])b.box([x+dx,deck+.145,z],[.055,.070,.80],c.upper);
      for(const dz of [-.375,.375])b.box([x,deck+.145,z+dz],[.72,.070,.055],c.upper);
      if(b.keep(0.30))for(const dz of [-.30,.30])b.box([x-.40,deck+.10,z+dz],[.10,.075,.10],c.steel);
      if(managed)b.box([x,deck+.17,z+.44],[.49,.14,.16],c.upper);
      for(let j=0;j<(power?2:1);j++) {
        b.cylinder([w-.14,deck-.34,front-1.18-j*.30],[w+.14,deck-.34,front-1.18-j*.30],.09,c.steel,16);
        b.tube([w+.14,deck-.34,front-1.18-j*.30],[w+.24,deck-.23,front-1.18-j*.30],.095,.066,c.steel,16);
      }
      // AIR CLEANERS, and the reason they are here. Four engines are legal on
      // these hulls and the only thing that used to separate them was how many
      // louvres the grille had — a pitch, not a shape, and the coverage report
      // scored every one of them weak. An air cleaner is the fitting that stands
      // proud of an engine deck, so the count and the height of the drums is what
      // now answers the choice: none on the compact, one on the standard, a low
      // heat exchanger and a fan stack on the managed one, two tall drums on the
      // high-output. They are drawn under the deck's own maximum, so a coarse
      // level may cull them without moving the vehicle's extent.
      const drums=power?2:s.mobility==='engine_diesel_900'?1:0;
      for(let j=0;j<drums;j++)b.revolve([x-.44,deck+.09,z-.26+j*.52],[0,1,0],[
        {r:.09,h:0},{r:.185,h:.05,s:1},{r:.20,h:.13,s:1},{r:.20,h:.40,s:1},{r:.175,h:.46,s:1},{r:.085,h:.49}],c.steel,20,.62);
      if(drums&&b.keep(0.30))for(let j=0;j<drums;j++)
        b.tube([x-.44,deck+.50,z-.26+j*.52],[x-.30,deck+.52,z-.26+j*.52],.070,.046,c.shade,12);
      if(managed) {
        b.revolve([x-.42,deck+.11,z-.10],[0,1,0],[{r:.14,h:0},{r:.21,h:.045,s:1},{r:.21,h:.30,s:1},{r:.17,h:.34}],c.upper,18,.44);
        b.box([x-.42,deck+.13,z+.36],[.40,.18,.30],c.shade);
        if(b.keep(0.30))for(let j=0;j<b.many(6,2);j++)b.box([x-.42,deck+.13,z+.24+j*.048],[.36,.13,.020],c.bright);
      }
      // Cooling-fan cowl and a filler socket. From directly above, the deck used
      // to be a lid with stripes on it and nothing that stood proud.
      if(b.keep(0.30)) {
        b.revolve([x-.02,deck+.152,z-.02],[0,1,0],[
          {r:.245,h:0},{r:.245,h:.055,s:1},{r:.215,h:.085,s:1},{r:.215,h:.10},{r:0,h:.10}],c.steel,12);
        b.socket([x+.30,deck+.14,z-.34],[x+.30,deck+.24,z-.34],.070,.042,c.steel,8);
      }
    });
    b.part('transmission / forward drive access',()=>{
      // Three gearboxes, three front ends. The old part changed one plate's width
      // and depth, which the coverage report scored as a panel growing in place;
      // a cross-drive and a manual box do not share a nose casting, so they do
      // not share one here either.
      const automatic=s.transmission!=='transmission_manual',managed=s.transmission==='ground_transmission_electric';
      b.box([0,.91,front-.02],[automatic?1.0:.73,.22,automatic?.19:.12],c.shade);
      if(b.keep(0.30))for(const x of [-.26,.26])b.box([x,.94,front+.08],[.07,.06,.045],c.bright);
      if(!automatic) {
        // Manual: a bolted inspection plate, the shift linkage that runs back
        // through it, and the hand-crank boss beside it.
        beveled(b,[0,.99,front+.055],[.62,.26,.10],c.upper,.045);
        if(b.keep(0.30)) {
          for(const dx of [-.20,.20])b.rod([dx,.99,front+.06],[dx,.99,front+.13],.030,c.steel,8);
          b.socket([.31,.86,front+.02],[.31,.86,front+.16],.075,.044,c.steel,14);
        }
      } else {
        // Automatic: a cross-drive housing with a turned final-drive bulge at
        // each end of it, which is the shape that tells the eye the drive splits
        // to both tracks at the front.
        for(const side of [-1,1])b.revolve([side*.60,.88,front-.10],[side,0,0],[
          {r:.20,h:0},{r:.26,h:.055,s:1},{r:.28,h:.16,s:1},{r:.26,h:.245,s:1},{r:.19,h:.28},{r:0,h:.28}],c.armor,20,.60);
        if(b.keep(0.30))for(const side of [-1,1])b.studs([side*.60,.88,front-.10],[side,0,0],.215,8,.032,.026,c.bright);
      }
      if(managed) {
        // Managed: the control unit, its cooling module and the loom that leaves
        // it. The plain automatic has none of this.
        b.box([-.43,1.075,front-.16],[.25,.16,.26],c.upper);
        b.box([.34,1.10,front-.24],[.44,.24,.34],c.shade);
        if(b.keep(0.30)) {
          for(let j=0;j<b.many(5,2);j++)b.box([.34,1.10,front-.40+j*.055],[.40,.17,.024],c.bright);
          b.rod([.14,1.05,front-.24],[-.30,1.02,front-.18],.026,c.cable,8);
        }
      }
    });
    b.part(`turret / ${mg?'protected weapon station':howitzer?'enclosed howitzer turret':aaMount?'air-defense cradle':'autocannon turret'}`,()=>{
      const radius=mg?.36:howitzer?1.07:aaMount?.78:.63;
      b.cylinder([0,deck+.066,mountZ],[0,deck+.18,mountZ],radius,c.steel,40);
      if(mg) {
        b.cylinder([0,deck+.18,mountZ],[0,mountTop-.05,mountZ],.16,c.upper,20);
        beveled(b,[0,mountTop-.03,mountZ+.10],[.49,.32,.085],c.armor,.05);
        for(const side of [-1,1])b.box([side*.235,mountTop-.015,mountZ-.03],[.05,.29,.32],c.upper);
        // A bare pedestal reads as a pipe. The bolted ring hatch it stands on and
        // the discharger cluster behind it are what make it a fighting position.
        b.cylinder([0,deck+.09,mountZ],[0,deck+.155,mountZ],.50,c.shade,28);
        if(b.keep(0.30))for(let j=0;j<8;j++){const a=TAU*j/8;b.box([Math.sin(a)*.50,deck+.185,mountZ+Math.cos(a)*.50],[.07,.042,.07],c.bright);}
        if(b.keep(0.30))for(const side of [-1,1])for(let j=0;j<3;j++)
          b.cylinder([side*(.42+j*.10),deck+.24,mountZ-.34],[side*(.46+j*.11),deck+.46,mountZ-.44],.055,c.steel,10);
      } else {
        const tw=howitzer?1.18:aaMount?.83:.75,back=howitzer?-1.31:aaMount?-.63:-.73,ahead=howitzer?.94:aaMount?.58:.61;
        b.loft([octagon(deck+.19,tw,back,ahead,.19,mountZ),octagon(mountTop,tw*(aaMount?.80:.85),back+.14,ahead-.17,.19,mountZ)],c.upper);
        hatch(-tw*.40,mountTop,mountZ-.24,howitzer?.31:.23);
        for(const side of [-1,1]) {
          b.box([side*tw*.93,mountTop-.20,mountZ-.20],[.075,.10,.38],c.shade);
          if(modular)b.box([side*(tw+.03),mountTop-.29,mountZ+.09],[.13,.30,.60],c.armor);
        }
      }
    });
    b.part(`armament / ${label(s.armament)} installation`,()=>{
      const missile=s.armament==='ground_aa_missiles',twin=s.armament==='ground_aa_gun',indirect=s.armament.startsWith('ground_howitzer'),small=s.armament==='ground_mg_127';
      if(missile) {
        for(const side of [-1,1])for(let row=0;row<2;row++)for(let col=0;col<2;col++) {
          const x=side*(.73+col*.28),y=mountTop-.02+row*.28,z=mountZ+.14;
          b.always(()=>{
            b.box([x,y,z],[.25,.25,1.50],c.shade);
            b.box([x,y,z+.765],[.215,.215,.045],c.black);
          });
          if(b.keep(0.30))for(const dz of [-.40,.42])b.box([x,y,z+dz],[.275,.275,.035],c.bright);
        }
      } else {
        const thick=s.armament==='ground_howitzer_155'||s.armament==='ground_gun_35';
        const extent=indirect?(thick?4.35:3.82):small?1.02:twin?2.55:thick?2.64:2.23;
        const radius=indirect?(thick?.112:.087):small?.027:twin?.040:thick?.052:.039;
        const start=mountZ+(indirect?.70:small?.12:.39),rise=indirect?.36:twin?.20:0;
        const offsets=twin?[-.46,.46]:[0];
        for(const x of offsets) {
          b.cylinder([x,gunY,start-.08],[x,gunY,start+.30],radius*2.6,c.shade,24);
          b.always(()=>{
            b.cylinder([x,gunY,start+.25],[x,gunY+rise,start+extent-.22],radius*1.18,c.upper,24,radius);
            b.tube([x,gunY+rise,start+extent-.23],[x,gunY+rise,start+extent],radius*1.08,radius*.68,c.steel,24);
          });
          if(indirect) {
            b.cylinder([x,gunY+rise*.53,start+extent*.50],[x,gunY+rise*.62,start+extent*.60],radius*1.53,c.shade,24);
            b.box([x,gunY+rise,start+extent-.065],[radius*3.6,radius*2.6,.25],c.steel);
            if(b.keep(0.30))for(const side of [-1,1])b.box([x+side*radius*1.82,gunY+rise,start+extent-.05],[.009,radius*1.2,.13],c.black);
            b.rod([x+.19,gunY+.13,start],[x+.19,gunY+.13,start+.91],.060,c.steel,16);
          } else if(small) {
            b.box([.12,gunY-.05,start+.12],[.13,.13,.28],c.steel);
            b.box([0,gunY,start+.10],[.105,.095,.42],c.black);
          } else if(b.keep(0.30))for(let j=0;j<4;j++)b.cylinder([x,gunY+rise*j/4,start+.50+j*.16],[x,gunY+rise*j/4,start+.535+j*.16],radius*1.45,c.steel,16);
        }
      }
    });
    b.part(`ammunition / ${label(s.ammunition)} ready stowage`,()=>{
      // Exterior stowage identifies the ammunition load; payload quantities remain
      // simulation metadata, not invented performance claims. What this draws is
      // what a resupply party handles — a ready box, an upright shell rack, a
      // charge bin, a canister cradle — and the SHAPE of it answers the choice.
      // The old part drew one locker in the same place for all six ids and read
      // its label off the selection, which is why the coverage report scored the
      // whole slot absent: a belt box and a rack of upright projectiles do not
      // look alike at any distance, and they should not have.
      const ammo=s.ammunition,tw=howitzer?1.18:aaMount?.83:mg?.50:.75,dk=deck+.065;
      // A lidded deck locker: body, raised lid, over-centre catches, grab rail.
      const locker=(cx,cy,cz,size,tint)=>{
        // The chamfer and the lid rim are inspection detail. At the map pin the
        // locker is one prism, because a locker's 28 triangles against a prism's
        // 12 is a quarter of what the LOD2 band has left over on a loaded hull.
        if(b.level<2)beveled(b,[cx,cy,cz],size,tint,.05);else b.box([cx,cy,cz],size,tint);
        if(b.keep(0.30))b.box([cx,cy+size[1]/2+.018,cz],[size[0]+.03,.028,size[2]+.03],c.edge);
        if(b.keep(0.30))for(const dz of [-size[2]*.30,size[2]*.30])b.box([cx+size[0]/2+.014,cy,cz+dz],[.028,.085,.05],c.bright);
        if(b.keep(0.30))b.rod([cx-size[0]*.22,cy+size[1]/2+.046,cz],[cx+size[0]*.22,cy+size[1]/2+.046,cz],.015,c.bright,8);
      };
      // A projectile: turned body, ogive nose. A rack of these reads as rounds;
      // a rack of plain cylinders reads as pegs.
      const round=(origin,axis,radius,reach,tint)=>b.revolve(origin,axis,[
        {r:0,h:0},{r:radius,h:.022},{r:radius,h:reach*.70,s:1},{r:radius*.86,h:reach*.84,s:1},
        {r:radius*.46,h:reach*.96,s:1},{r:0,h:reach}],tint,12,Math.hypot(radius*2,reach));
      if(ammo==='ground_ammo_autocannon') {
        // Belt-fed ready boxes on the turret flanks, with the feed chute to the
        // breech and a spent-link bag under it.
        for(const side of [-1,1]) {
          locker(side*(tw+.28),mountTop-.28,mountZ-.04,[.28,.38,.70],weathered(c.hull,side>0?1:2));
          if(b.keep(0.30)) {
            b.box([side*(tw+.10),mountTop-.14,mountZ+.16],[.16,.11,.34],c.steel);
            for(let j=0;j<b.many(5,2);j++)b.box([side*(tw+.10),mountTop-.14,mountZ+.03+j*.07],[.17,.055,.032],c.bright);
          }
        }
        if(b.keep(0.30)) {
          b.box([0,mountTop-.44,mountZ-.30],[.42,.26,.30],c.canvas);
          b.rod([-.20,mountTop-.30,mountZ-.30],[.20,mountTop-.30,mountZ-.30],.018,c.steel,8);
        }
      } else if(ammo==='ground_ammo_ball') {
        // A single belt box on the station bracket and two spare boxes stacked on
        // the deck: the smallest load in the catalogue, and it should look it.
        locker(.34,mountTop-.11,mountZ+.04,[.26,.22,.36],c.shade);
        if(b.keep(0.30))for(let j=0;j<b.many(4,2);j++)b.box([.34,mountTop-.02,mountZ-.13-j*.05],[.17,.045,.030],c.bright);
        for(let j=0;j<2;j++)locker(w*.56,dk+.13+j*.21,mountZ-.88,[.44,.19,.54],weathered(c.hull,j+1));
        if(b.keep(0.30))for(const dz of [-.20,.20])b.rod([w*.56-.24,dk+.06,mountZ-.88+dz],[w*.56+.24,dk+.06,mountZ-.88+dz],.020,c.steel,8);
      } else if(ammo==='ground_ammo_he') {
        // An upright projectile ready rack on the rear deck, behind the turret,
        // where a crew actually stands to load. Seven noses in a row is the one
        // silhouette in this slot that a map pin could still recognise.
        const z=rear+1.02;
        b.box([0,dk+.05,z],[1.62,.07,.46],c.steel);
        for(let i=0;i<b.many(7,3);i++) {
          const step=b.many(7,3),x=(i-(step-1)/2)*(1.42/Math.max(1,step-1)||0);
          round([x,dk+.09,z],[0,1,0],.077,.62,i%2?c.shade:c.bright);
        }
        if(b.keep(0.30))for(const dz of [-.20,.20]) {
          b.rod([-.78,dk+.44,z+dz],[.78,dk+.44,z+dz],.018,c.steel,8);
          for(const dx of [-.78,.78])b.rod([dx,dk+.09,z+dz],[dx,dk+.48,z+dz],.018,c.steel,8);
        }
        locker(-w*.60,dk+.20,z-.72,[.46,.34,.52],c.canvas);
      } else if(ammo==='ground_ammo_guided') {
        // Three padded canisters lying fore-and-aft in a cradle, with the setting
        // unit and its umbilical: long horizontal tubes against the high-explosive
        // load's short upright ones.
        const z=rear+1.16;
        b.box([0,dk+.06,z],[1.32,.09,.34],c.steel);
        for(let i=0;i<b.many(3,2);i++) {
          const step=b.many(3,2),x=(i-(step-1)/2)*(.82/Math.max(1,step-1)||0);
          b.cylinder([x,dk+.24,z-.54],[x,dk+.24,z+.50],.105,c.canvas,16);
          b.always(()=>b.tube([x,dk+.24,z+.48],[x,dk+.24,z+.56],.108,.070,c.steel,16));
          if(b.keep(0.30))for(const dz of [-.30,.26])b.cylinder([x,dk+.24,z+dz],[x,dk+.24,z+dz+.035],.122,c.bright,16);
        }
        if(b.keep(0.30))for(const dz of [-.34,.30])b.box([0,dk+.13,z+dz],[1.28,.10,.055],c.shade);
        b.box([w*.58,dk+.22,z-.74],[.34,.30,.36],c.upper);
        b.box([w*.58,dk+.22,z-.74+.19],[.22,.16,.020],c.glass);
        if(b.keep(0.30))b.rod([w*.58-.14,dk+.10,z-.74],[.44,dk+.10,z-.54],.020,c.cable,8);
      } else if(ammo==='ground_ammo_aa') {
        // Twin ready drums flanking the mount, each with its feed chute. A cannon
        // load is round where the missile load is square.
        for(const side of [-1,1]) {
          b.revolve([side*(tw+.16),mountTop-.32,mountZ-.20],[side,0,0],[
            {r:0,h:0},{r:.28,h:.02},{r:.30,h:.05,s:1},{r:.30,h:.24,s:1},{r:.28,h:.27},{r:0,h:.29}],c.shade,20,.62);
          if(b.keep(0.30)) {
            b.studs([side*(tw+.18),mountTop-.32,mountZ-.20],[side,0,0],.20,6,.030,.024,c.bright);
            b.box([side*(tw+.24),mountTop-.16,mountZ+.06],[.14,.30,.20],c.steel);
          }
        }
        locker(w*.46,dk+.17,mountZ-1.24,[.86,.30,.50],weathered(c.hull,1));
      } else if(ammo==='ground_ammo_missiles') {
        // A reload cradle carrying two spare launch canisters on the rear deck,
        // with the lifting frame that gets them onto the rails.
        const z=rear+1.34;
        b.box([0,dk+.07,z],[1.20,.09,1.30],c.steel);
        for(const side of [-1,1]) {
          b.box([side*.44,dk+.26,z],[.26,.26,1.46],c.shade);
          b.box([side*.44,dk+.26,z+.755],[.225,.225,.045],c.black);
          if(b.keep(0.30))for(const dz of [-.42,.44])b.box([side*.44,dk+.26,z+dz],[.29,.29,.035],c.bright);
          if(b.keep(0.30))b.rod([side*.44,dk+.41,z-.52],[side*.44,dk+.41,z+.52],.020,c.steel,8);
        }
        if(b.keep(0.30))for(const dz of [-.56,.56]) {
          for(const dx of [-.62,.62])b.rod([dx,dk+.11,z+dz],[dx,dk+.62,z+dz],.024,c.steel,8);
          b.rod([-.62,dk+.62,z+dz],[.62,dk+.62,z+dz],.024,c.steel,8);
        }
      } else {
        // Any load the mesh does not model yet keeps the plain deck locker rather
        // than silently drawing nothing.
        locker(w*.48,dk+.18,mountZ-.60,[.52,.30,.44],c.shade);
      }
    });
    b.part(`sensors / ${label(s.sensors)} observation fittings`,()=>{
      hatch(-w*.42,deck+.065,front-1.12,.25);
      if(b.keep(0.30))for(let j=-1;j<=1;j++)b.box([-w*.42+j*.16,deck+.19,front-.90],[.13,.085,.065],c.glass);
      // Coaming and guard bar over the driver's blocks. From above they are the
      // only fitting that says which end of a low flat hull is the front.
      b.box([-w*.42,deck+.135,front-.90],[.52,.055,.11],c.shade);
      if(b.keep(0.30)) {
        for(const dx of [-.25,.25])b.rod([-w*.42+dx,deck+.24,front-.86],[-w*.42+dx,deck+.30,front-.86],.016,c.steel,6);
        b.rod([-w*.42-.25,deck+.30,front-.86],[-w*.42+.25,deck+.30,front-.86],.016,c.steel,6);
      }
      const night=s.sensors==='optics_night',thermal=s.sensors==='optics_thermal',y=mg?deck+.17:mountTop;
      b.box([.32,y+.075,mountZ+.15],[thermal?.28:.21,thermal?.16:.12,.24],c.shade);
      b.box([.32,y+.077,mountZ+.276],[thermal?.19:.13,.075,.016],c.lens);
      // Night observation is a SEARCHLIGHT on these hulls, not a slightly larger
      // drum: a turned housing on its own bracket, with the emitter face and the
      // hood that keeps the driver's block usable beside it. The old 83 mm drum
      // scored weak because at any distance it was the same shape as the day
      // sight it sat next to.
      if(night) {
        b.revolve([.44,y+.11,mountZ+.16],[0,0,1],[
          {r:.09,h:0},{r:.185,h:.045,s:1},{r:.195,h:.20,s:1},{r:.185,h:.30,s:1},{r:.17,h:.32}],c.shade,20,.44);
        b.revolve([.44,y+.11,mountZ+.16],[0,0,1],[{r:.165,h:.315,s:1},{r:.13,h:.335,s:1},{r:0,h:.345}],c.lens,20,.44);
        b.box([.44,y-.09,mountZ+.20],[.11,.22,.13],c.steel);
        if(b.keep(0.30)) {
          b.studs([.44,y+.11,mountZ+.17],[0,0,1],.175,8,.026,.020,c.bright);
          b.box([-w*.42,deck+.26,front-.90],[.30,.10,.14],c.shade);
          b.box([-w*.42,deck+.26,front-.83],[.22,.055,.016],c.glass);
        }
      }
      if(thermal)b.always(()=>{b.cylinder([.32,y+.15,mountZ+.15],[.32,y+.38,mountZ+.15],.085,c.steel,20);b.box([.32,y+.44,mountZ+.15],[.32,.15,.24],c.upper);b.box([.32,y+.44,mountZ+.278],[.22,.09,.02],c.lens);})
    });
    b.part(`fire_control / ${label(s.fire_control)} sight and stabilization`,()=>{
      // Stabilisation is a machine with a stroke, so it is drawn as one: the
      // elevation actuator and its linkage stand outboard of the mount where the
      // eye can see them move. Digital control adds the computer case and the
      // crosswind sensor above it. The old part answered all three choices by
      // making one grey box 90 mm taller, which is the definition of weak.
      const digital=s.fire_control==='fcs_digital',stabilized=s.fire_control==='fcs_stabilized';
      const x=mg?-.30:-.40,y=mg?mountTop-.12:mountTop-.10;
      b.box([x,y,mountZ+.30],[.14,digital?.19:.10,.24],c.shade);
      b.box([x,y,mountZ+.43],[.095,.065,.018],c.glass);
      if(stabilized||digital) {
        b.rod([x,gunY-.12,mountZ+.35],[x,gunY-.12,mountZ+.82],.038,c.bright,12);
        const ax=x-.24,ay=(gunY+y)/2-.06;
        b.revolve([ax,ay-.22,mountZ+.14],[0,1,0],[
          {r:.055,h:0},{r:.098,h:.045,s:1},{r:.098,h:.34,s:1},{r:.072,h:.38,s:1},{r:.072,h:.50,s:1},{r:0,h:.52}],c.steel,18,.56);
        if(b.keep(0.30)) {
          b.box([ax,ay-.26,mountZ+.14],[.15,.11,.17],c.shade);
          b.rod([ax,ay+.30,mountZ+.14],[ax+.16,ay+.34,mountZ+.30],.030,c.bright,8);
        }
      }
      if(digital) {
        b.box([x-.18,y+.06,mountZ+.22],[.18,.13,.24],c.upper);
        // The ballistic computer, its display and the laser rangefinder head that
        // feeds it. All of it sits at mount height and under the vehicle's own
        // maximum: a mast would have read better and it was drawn and taken out
        // again, because on the howitzer hull it set the extent, and geometry
        // that sets the extent has to survive to the map pin, where 60 triangles
        // is most of what the LOD2 band has left on a loaded vehicle.
        b.box([x+.10,y+.19,mountZ-.30],[.34,.26,.32],c.shade);
        b.box([x+.10,y+.19,mountZ-.145],[.24,.15,.020],c.glass);
        if(b.keep(0.30)) {
          b.box([x+.10,y+.33,mountZ-.30],[.28,.030,.26],c.edge);
          b.rod([x+.10,y+.06,mountZ-.30],[x,y-.02,mountZ+.12],.026,c.cable,8);
        }
        b.revolve([x+.30,y+.10,mountZ+.28],[0,0,1],[
          {r:.06,h:0},{r:.115,h:.035,s:1},{r:.115,h:.19,s:1},{r:.098,h:.22}],c.steel,18,.26);
        b.revolve([x+.30,y+.10,mountZ+.28],[0,0,1],[{r:.092,h:.215,s:1},{r:.070,h:.235,s:1},{r:0,h:.245}],c.lens,18,.26);
      }
    });
    b.part(`communications / ${label(s.communications)} aerial installation`,()=>{
      const network=s.communications==='ground_comms_network',secure=s.communications==='ground_comms_secure',data=s.communications==='comms_data';
      for(const side of network||data?[-1,1]:[-1]) {
        const x=side*(w-.35),z=rear+.54;
        b.cylinder([x,deck,z],[x,deck+.14,z],.075,c.black,16);
        b.always(()=>b.rod([x,deck+.14,z],[x,deck+(network?1.31:1.01),z-.08],b.gauge(.013),c.steel,8));
      }
      // The secure set is a CASE on a bracket, standing off the deck where the
      // outline can see it, plus the tuning unit at the aerial foot and the loom
      // between them. Buried flat against the deck at 320x190x370 it was
      // measurably invisible — 0.02 m² of changed outline on a hull whose other
      // choices move a quarter of a square metre.
      if(secure||network) {
        beveled(b,[w-.46,deck+.31,rear+.66],[.44,.36,.48],c.shade,.055);
        b.box([w-.46,deck+.50,rear+.66],[.47,.030,.51],c.edge);
        if(b.keep(0.30)) {
          for(const dz of [-.17,.17])b.rod([w-.46,deck+.09,rear+.66+dz],[w-.46,deck+.14,rear+.66+dz],.028,c.steel,8);
          b.box([w-.35,deck+.20,rear+.58],[.19,.17,.21],c.upper);
          b.rod([w-.35,deck+.16,rear+.56],[w-.35,deck+.14,rear+.54],.022,c.cable,8);
        }
      }
      if(data||network)b.cylinder([w-.43,deck+.20,rear+.65],[w-.43,deck+.32,rear+.65],.19,c.upper,24);
    });
    if(s.active_protection!=='aps_none')b.part(`active_protection / ${label(s.active_protection)} perimeter system`,()=>{
      for(const side of [-1,1]) {
        const x=side*(w-.20),z=front-.82;
        b.box([x,deck+.09,z],[.16,.17,.18],c.shade);
        b.box([x,deck+.09,z+.095],[.10,.095,.015],c.lens);
        const count=s.active_protection==='aps_hard'?4:2;
        if(b.keep(0.30))for(let j=0;j<count;j++)b.cylinder([x,deck+.06,z-.30-j*.095],[x+side*.13,deck+.24,z-.21-j*.095],.041,c.steel,12);
        if(count===4)b.box([side*(w-.10),deck+.28,rear+.47],[.21,.25,.27],c.armor);
      }
    });
    if(s.troop_compartment)b.part(`troop_compartment / ${s.troop_compartment==='ground_troops_protected'?'reinforced troop bay':'troop bay and rear egress'}`,()=>{
      const protectedBay=s.troop_compartment==='ground_troops_protected',height=protectedBay?.19:.045;
      b.box([0,deck+height/2,rear+1.02],[w*1.34,height,1.33],c.upper);
      // Roof edge lip and a stowage basket forward of the hatches. Crews stow on
      // the roof of a carrier, and the top view had nothing between the hatches.
      for(const dx of [-w*.67,w*.67])b.box([dx,deck+height+.03,rear+1.02],[.06,.075,1.35],c.edge);
      for(const dz of [rear+.36,rear+1.68])b.box([0,deck+height+.03,dz],[w*1.36,.075,.06],c.edge);
      if(b.keep(0.30)) {
        for(let j=0;j<2;j++)b.rod([-w*.55,deck+height+.05,rear+.30+j*.44],[w*.55,deck+height+.05,rear+.30+j*.44],.018,c.steel,6);
        for(let j=0;j<b.many(5,2);j++)b.rod([(j-2)*w*.275,deck+height+.05,rear+.28],[(j-2)*w*.275,deck+height+.05,rear+.76],.014,c.steel,6);
      }
      beveled(b,[0,deck+height+.17,rear+.52],[w*.66,.22,.40],c.canvas,.055);
      // Roof hatches, vision blocks and the periscope cluster each crewman gets.
      for(const side of [-1,1]) {
        hatch(side*w*.37,deck+height,rear+1.01,.25);
        if(b.keep(0.30))for(let j=0;j<3;j++) {
          b.box([side*(w-.09),deck-.17,rear+.48+j*.43],[.045,.075,.14],c.glass);
          b.box([side*(w-.05),deck-.17,rear+.48+j*.43],[.045,.11,.19],c.armor);
          b.rod([side*(w-.10),deck-.40,rear+.48+j*.43],[side*(w+.04),deck-.40,rear+.48+j*.43],.030,c.steel,8);
        }
        // Tie-downs along the bay roof: four a side, and they are what a
        // section lashes its kit to.
        if(b.keep(0.30))for(let j=0;j<4;j++)
          b.socket([side*w*.86,deck+height+.02,rear+.42+j*.42],[side*w*.86,deck+height+.11,rear+.42+j*.42],.070,.042,c.steel,18);
      }
      // The ramp is the entire aft silhouette on a carrier and it was one flat
      // panel with five ribs. Personnel door, hinges, actuators and lights give
      // the rear view something to resolve into at card size.
      beveled(b,[0,deck-.46,rear-.035],[w*1.18,.90,.085],protectedBay?c.armor:c.shade,.07);
      if(b.keep(0.30))for(const x of [-w*.50,w*.50])b.rod([x,.79,rear-.092],[x,deck-.16,rear-.092],b.gauge(protectedBay?.038:.023),c.bright);
      if(b.keep(0.30))for(let j=0;j<b.many(5,2);j++)b.box([0,.81+j*.135,rear-.085],[w*.90,.020,.025],c.steel);
      b.box([0,deck-.27,rear-.10],[.17,.055,.035],c.bright);
      b.box([w*.32,deck-.46,rear-.090],[.44,.72,.028],c.edge);
      b.box([w*.32,deck-.25,rear-.106],[.19,.12,.016],c.glass);
      if(b.keep(0.30))b.box([w*.32+.18,deck-.46,rear-.108],[.05,.10,.042],c.bright);
      for(const side of [-1,1]) {
        b.rod([side*w*.63,.84,rear+.03],[side*w*.63,.84,rear-.10],.052,c.steel,8);
        b.cylinder([side*w*.74,deck-.66,rear+.18],[side*w*.66,deck-.28,rear-.04],.050,c.bright,10);
        if(b.keep(0.30)) {
          b.box([side*w*.82,deck-.09,rear-.02],[.16,.15,.09],c.shade);
          b.box([side*w*.82,deck-.09,rear-.070],[.10,.09,.012],side>0?c.amber:c.lens);
        }
        // Ramp hinge barrel and the door catch. The hinge line is what tells the
        // eye the whole aft plate swings rather than being welded shut.
        if(b.keep(0.30))b.revolve([side*w*.86,.72,rear-.02],[0,1,0],[
          {r:.055,h:0,s:1},{r:.055,h:.34,s:1},{r:.038,h:.38}],c.steel,12);
      }
      b.box([0,.63,rear-.13],[w*.66,.070,.18],c.steel);
    });
    if(s.recon_package)b.part(`recon_package / ${s.recon_package==='ground_recon_mast'?'elevated observation mast':'scout observation station'}`,()=>{
      const elevated=s.recon_package==='ground_recon_mast',z=rear+.94,y=deck+(elevated?1.72:.39);
      b.cylinder([.12,deck,z],[.12,deck+.21,z],.20,c.shade,24);
      b.cylinder([.12,deck+.19,z],[.12,y-.15,z],b.gauge(elevated?.06:.095),c.bright,20,b.gauge(elevated?.036:.095));
      if(elevated)b.cylinder([.12,deck+.26,z],[.12,deck+.77,z],.095,c.steel,20);
      b.always(()=>{
        b.box([.12,y,z],[elevated?.46:.34,.25,.32],c.upper);
        for(const x of [-.01,.22])b.box([x,y,z+.168],[.13,.15,.016],c.lens);
      });
      hatch(-.39,deck+.065,z-.25,.25);
    });
    if(s.artillery_loader)b.part(`artillery_loader / ${s.artillery_loader==='ground_loader_assisted'?'assisted loading installation':'manual loading access'}`,()=>{
      const assisted=s.artillery_loader==='ground_loader_assisted',z=mountZ-1.27;
      b.box([0,deck+.65,z],[assisted?1.35:.86,assisted?.71:.56,assisted?.49:.13],c.shade);
      b.box([0,deck+.65,z-(assisted?.26:.08)],[.56,.40,.025],c.upper);
      for(const side of [-1,1]) {
        if(b.keep(0.30))b.rod([side*.42,deck+.41,z-.12],[side*.42,deck+.93,z-.12],.024,c.bright);
        // The recoil spades are the aftmost thing on this vehicle, so they are
        // drawn at every level rather than culled with the rest of the fittings.
        b.always(()=>{
          b.rod([side*.60,.64,rear-.04],[side*.74,.26,rear-.27],.066,c.steel,10);
          b.box([side*.74,.24,rear-.27],[.34,.085,.26],c.shade);
        });
      }
      if(assisted)for(let j=0;j<b.many(4,2);j++)b.box([0,deck+.37,z-.28-j*.10],[.59,.045,.052],c.bright);
    });
    if(s.radar)b.part(`radar / ${s.radar==='ground_radar_tracking'?'search and tracking array':'rotating search array'}`,()=>{
      const tracking=s.radar==='ground_radar_tracking',z=mountZ-.43,y=mountTop+.93;
      b.cylinder([0,mountTop,z],[0,y-.16,z],b.gauge(.11),c.steel,24);
      b.cylinder([0,mountTop+.03,z],[0,mountTop+.18,z],.28,c.shade,24);
      b.always(()=>{
        b.box([0,y,z],[tracking?1.24:1.02,tracking?.55:.38,.14],c.upper);
        b.box([0,y,z+.079],[tracking?1.13:.91,tracking?.44:.27,.024],c.glass);
      });
      if(b.keep(0.30))for(let i=-4;i<=4;i++)b.box([i*.105,y,z+.095],[.014,tracking?.44:.27,.012],c.shade);
      if(tracking) {
        b.rod([.63,mountTop-.02,mountZ+.14],[.83,mountTop+.41,mountZ+.13],.07,c.steel,16);
        b.cylinder([.83,mountTop+.46,mountZ+.12],[.83,mountTop+.46,mountZ+.26],.27,c.upper,32,.31);
        b.cylinder([.83,mountTop+.46,mountZ+.265],[.83,mountTop+.46,mountZ+.28],.265,c.glass,32);
      }
    });
    const names={ground_ifv:'tracked infantry fighting vehicle',ground_apc:'wheeled armored personnel carrier',ground_recon:'wheeled reconnaissance vehicle',ground_artillery:'self-propelled artillery',ground_air_defense:'mobile air-defense vehicle'};
    const result=b.finish(`Visual interpretation of a fictional ${names[spec.platform]}. Component-driven original game art; simulation values are separate.`);
    result.specification={platform:spec.platform,components:{...s}};
    return result;
  }

  function build(rawSpec) {
    const spec = resolveSpec(rawSpec), chosen = spec.components, heavy = spec.platform === "tank_heavy";
    const detail = levelOf(rawSpec);
    if(Object.hasOwn(GROUND_DEFAULTS,spec.platform))return buildGround(spec, detail);
    const mobile = ["drive_mobile","engine_diesel_1200","engine_turbine_1500"].includes(chosen.mobility), reinforced = chosen.protection === "protection_heavy", active = chosen.protection === "protection_active" || chosen.active_protection === "aps_hard";
    const heavyGun = ["armament_heavy","gun_120","gun_125"].includes(chosen.armament), integrated = ["sensors_integrated","optics_thermal"].includes(chosen.sensors), data = chosen.communications === "comms_data";
    const compact=chosen.turret==='turret_compact',casemate=chosen.turret==='turret_casemate',autoload=chosen.turret==='turret_autoload',large=chosen.turret==='turret_heavy';
    const hullWidth = heavy ? 1.64 : 1.43, rear = heavy ? -3.32 : -2.91, front = heavy ? 3.04 : 2.76;
    const b = createBuilder(detail, front - rear), c = PALETTE;
    const wheelHalfSpan = heavy ? 2.65 : 2.31, trackX = hullWidth + 0.20, trackWidth = (heavy ? 0.77 : 0.68)+(chosen.tracks==='tracks_wide'?.19:0);
    const trackRadius = 0.67, trackCenterY = 0.67, turretY = heavy ? 1.68 : 1.58, turretZ = 0.13;
    const turretWidth = (heavy ? 1.37 : 1.18)*(compact?.80:casemate?1.13:large?1.08:1), turretTop = turretY + (casemate?.65:compact?.68:autoload?.73:heavyGun?.97:.87);
    const pathLength = wheelHalfSpan * 4 + TAU * trackRadius;
    function trackPoint(distance, radius = trackRadius) {
      // Both radii share the same angular path so the belt has consistent thickness.
      let s = ((distance % pathLength) + pathLength) % pathLength;
      if (s < wheelHalfSpan * 2) return { y: trackCenterY + radius, z: wheelHalfSpan - s, tangent: [0, 0, -1] };
      s -= wheelHalfSpan * 2;
      if (s < Math.PI * trackRadius) {
        const angle = Math.PI / 2 + s / trackRadius;
        return { y: trackCenterY + radius * Math.sin(angle), z: -wheelHalfSpan + radius * Math.cos(angle), tangent: [0, Math.cos(angle), -Math.sin(angle)] };
      }
      s -= Math.PI * trackRadius;
      if (s < wheelHalfSpan * 2) return { y: trackCenterY - radius, z: -wheelHalfSpan + s, tangent: [0, 0, 1] };
      s -= wheelHalfSpan * 2;
      const angle = -Math.PI / 2 + s / trackRadius;
      return { y: trackCenterY + radius * Math.sin(angle), z: wheelHalfSpan + radius * Math.cos(angle), tangent: [0, Math.cos(angle), -Math.sin(angle)] };
    }

    b.part("chassis / sloped lower hull", () => {
      // Five rings rather than three. The two added rings put a chine above the
      // belly and a break under the fender line, so the flank reads as folded
      // plate instead of one slab; the glacis gains its second angle the same way.
      b.loft([octagon(0.39, hullWidth - 0.29, rear + 0.16, front - 0.49, 0.28),
        octagon(0.66, hullWidth - 0.17, rear + 0.06, front - 0.24, 0.28),
        octagon(0.91, hullWidth - 0.08, rear, front + 0.08, 0.27),
        octagon(1.22, hullWidth - 0.05, rear + 0.02, front - 0.14, 0.30),
        octagon(1.47, hullWidth, rear + 0.06, front - 0.43, 0.32)], c.hull);
      b.loft([octagon(1.475, hullWidth + 0.015, rear + 0.05, front - 0.43, 0.32),
        octagon(1.54, hullWidth - 0.025, rear + 0.11, front - 0.49, 0.32)], c.upper);
      b.box([0, 0.58, rear - 0.03], [hullWidth * 1.5, 0.22, 0.12], c.shade);
      // Bolted nose beam and a rear plate. Without a hard horizontal at each end
      // the hull is a single smooth wedge from any three-quarter view.
      beveled(b, [0, 0.99, front + 0.05], [hullWidth * 1.44, 0.21, 0.15], c.armor, 0.05);
      if (b.keep(0.30)) for (let i = -3; i <= 3; i++) b.box([i * hullWidth * 0.30, 0.99, front + 0.125], [0.055, 0.055, 0.032], c.bright);
      beveled(b, [0, 1.06, rear - 0.07], [hullWidth * 1.60, 0.60, 0.11], c.upper, 0.07);
      if (b.keep(0.30)) for (let i = -4; i <= 4; i++) b.box([i * hullWidth * 0.21, 1.32, rear - 0.13], [0.05, 0.05, 0.028], c.bright);
      for (const x of [-hullWidth * 0.72, hullWidth * 0.72]) {
        b.rod([x, 0.78, front - 0.04], [x, 0.78, front + 0.19], b.gauge(0.095), c.bright, 12);
        b.tube([x, 0.80, front + 0.16], [x, 0.80, front + 0.23], 0.115, 0.063, c.steel, 16);
        b.tube([x, 0.79, rear - 0.15], [x, 0.79, rear - 0.05], 0.11, 0.060, c.steel, 16);
        // Towing sockets on the nose and the rear plate: the fitting a recovery
        // shackle actually pins to, and the reason the hull ends read as machined.
        if (b.keep(0.30)) for (const [z, out] of [[front + 0.02, 0.20], [rear - 0.02, -0.20]])
          b.socket([x * 0.62, 1.10, z], [x * 0.62, 1.10, z + out], 0.088, 0.052, c.steel, 18);
      }
      // The cast nose transition between the glacis and the lower plate. Turned
      // rather than folded, because on a real hull this is one poured casting.
      if (b.keep(0.30)) b.revolve([0, 1.14, front - 0.30], [1, 0, 0], [
        { r: 0.30, h: -hullWidth * 0.98 }, { r: 0.40, h: -hullWidth * 0.72, s: 1 },
        { r: 0.44, h: -hullWidth * 0.30, s: 1 }, { r: 0.44, h: hullWidth * 0.30, s: 1 },
        { r: 0.40, h: hullWidth * 0.72, s: 1 }, { r: 0.30, h: hullWidth * 0.98 }], c.armor, 18);
    });

    b.part("chassis / glacis applique, splash guard and spare track links", () => {
      // Applique is spaced off the glacis on purpose — flush plate would just be
      // a colour change. The basis is the measured glacis slope so the plates lie
      // on the surface rather than floating at their own angle.
      const up = [0, 0.7407, -0.6745], out = [0, 0.6745, 0.7407], slope = [[1, 0, 0], up, out];
      for (let i = 0; i < b.many(5, 2); i++) {
        const x = (i - 2) * hullWidth * 0.38;
        beveled(b, [x, 1.234, front - 0.131], [hullWidth * 0.35, 0.42, 0.09], weathered(c.armor, i), 0.06, slope);
      }
      // Spare links live on the glacis on real vehicles because that is the face
      // that gets hit; here they also break the largest flat plane on the model.
      if (b.keep(0.30)) for (let i = 0; i < b.many(4, 1); i++) beveled(b, [(i - 1.5) * 0.40, 1.02, front - 0.06], [0.34, 0.13, 0.10], c.steel, 0.03, slope);
      b.box([0, 1.515, front - 0.455], [hullWidth * 1.70, 0.05, 0.17], c.edge);
      for (const x of [-hullWidth * 0.55, hullWidth * 0.55]) {
        if (b.keep(0.30)) {
          b.rod([x, 1.05, front + 0.10], [x, 1.16, front + 0.06], 0.036, c.bright, 8);
          b.rod([x, 1.42, front - 0.34], [x, 1.53, front - 0.44], 0.030, c.steel, 8);
        }
        // Headlight guards, one turned housing each. The nose used to be armour
        // and nothing else from the front three-quarter view.
        if (b.keep(0.30)) b.revolve([x * 1.12, 1.44, front - 0.40], [0, 0.55, 0.83], [
          { r: 0.00, h: 0 }, { r: 0.118, h: 0.030 }, { r: 0.126, h: 0.115, s: 1 },
          { r: 0.092, h: 0.155, c: c.lens }, { r: 0, h: 0.158 }], c.shade, 10);
      }
    });

    for (const side of [-1, 1]) {
      const label = side < 0 ? "port" : "starboard", x = side * trackX;
      b.part(`running gear / ${label} continuous track belt`, () => {
        const steps = b.many(heavy ? 112 : 104, 12, 6);
        for (let i = 0; i < steps; i++) {
          const a = trackPoint(pathLength * i / steps, trackRadius - 0.015), d = trackPoint(pathLength * (i + 1) / steps, trackRadius - 0.015);
          const ai = trackPoint(pathLength * i / steps, trackRadius - 0.13), di = trackPoint(pathLength * (i + 1) / steps, trackRadius - 0.13);
          const p = (tx, point) => [tx, point.y, point.z], left = x - trackWidth / 2, right = x + trackWidth / 2;
          b.face([p(left, a), p(right, a), p(right, d), p(left, d)], c.track, [x, trackCenterY, (a.z + d.z) / 2]);
          // The inside of the belt faces the wheel volume.
          const innerMid = mean([p(left, ai), p(right, di)]), outerMid = mean([p(left, a), p(right, d)]);
          b.face([p(left, ai), p(left, di), p(right, di), p(right, ai)], c.rubber, add(outerMid, sub(outerMid, innerMid)));
          b.face([p(left, a), p(left, d), p(left, di), p(left, ai)], c.steel, [right + 1, trackCenterY, 0]);
          b.face([p(right, a), p(right, ai), p(right, di), p(right, d)], c.steel, [left - 1, trackCenterY, 0]);
        }
      });
      b.part(`running gear / ${label} individual tread links and pins`, () => {
        const count = b.many(heavy ? 92 : 84, 10, 6), step = pathLength / count;
        for (let i = 0; i < count; i++) {
          const p = trackPoint(i * step, trackRadius - 0.033), tangent = p.tangent, outward = [0, -tangent[2], tangent[1]];
          const basis = [[1, 0, 0], outward, tangent], center = [x, p.y, p.z];
          b.box(center, [trackWidth + 0.035, b.gauge(0.045), step * 0.88], shade(c.track, 0.95 + (i % 5) * 0.018), basis);
          if (b.keep(0.30)) {
            b.box(add(center, mul(outward, 0.034)), [trackWidth * (chosen.tracks==='tracks_padded'?.90:.71), chosen.tracks==='tracks_padded'?.06:.027, step * 0.65], c.rubber, basis);
            // End connectors and pins. The pin heads are what make a track read
            // as links rather than a rubber ribbon at any distance.
            for (const rim of [-1, 1]) {
              const pinCenter = add(center, mul(tangent, step * 0.40));
              b.cylinder(add(pinCenter, [rim * (trackWidth / 2 - 0.025), 0, 0]), add(pinCenter, [rim * (trackWidth / 2 + 0.050), 0, 0]), 0.025, c.bright, 6);
            }
            b.box(add(center, mul(outward, 0.032)), [trackWidth + 0.070, 0.023, step * 0.14], c.bright, basis);
            // Guide horn, inboard of the shoe, riding between the road wheels.
            b.box(add(center, mul(outward, -0.088)), [0.070, 0.13, step * 0.30], c.steel, basis);
            b.box(add(center, mul(outward, -0.155)), [0.060, 0.055, step * 0.22], c.bright, basis);
          }
        }
      });
      b.part(`running gear / ${label} suspension, road wheels and sprockets`, () => {
        const wheels = heavy ? 7 : 6, first = -wheelHalfSpan + 0.62, last = wheelHalfSpan - 0.62;
        for (let i = 0; i < wheels; i++) {
          const z = first + (last - first) * i / (wheels - 1), wheelY = 0.48, wheelRadius = 0.335;
          b.rod([side * (hullWidth - 0.14), 0.90, z - 0.24], [x, wheelY, z], b.gauge(0.085), c.steel, 12);
          // Bump stop above each arm. It is the fitting that tells the eye the
          // arm swings, and it survives the shrink to card size as a shadow.
          b.box([side * (hullWidth - 0.05), 1.02, z - 0.19], [0.17, 0.13, 0.16], c.shade);
          if(chosen.suspension==='suspension_hydro') {b.cylinder([x+side*.25,.86,z-.18],[x+side*.25,.53,z],.065,c.bright,12);b.cylinder([x+side*.25,.99,z-.25],[x+side*.25,.76,z-.13],.10,c.shade,12);}
          b.wheelBody([x, wheelY, z], [1, 0, 0], wheelRadius, 0.27, c.rubber, c.upper, c.shade, true);
        }
        // Drive sprocket forward, idler aft. Toothed wheels at both ends is the
        // tell that one track end was drawn twice rather than designed, and the
        // spoked idler web gives the rear of the running gear its own read.
        for (const end of [-1, 1]) {
          const z = end * wheelHalfSpan, y = trackCenterY + 0.21, r = 0.36;
          b.cylinder([x - 0.28, y, z], [x + 0.28, y, z], r, c.steel, 18);
          b.cylinder([x + side * 0.281, y, z], [x + side * 0.326, y, z], r * 0.75, c.hull, 18);
          b.cylinder([x + side * 0.325, y, z], [x + side * 0.355, y, z], 0.13, c.bright, 12);
          if (end > 0) { if (b.keep(0.30)) for (let tooth = 0; tooth < b.many(14, 5); tooth++) {
            const angle = TAU * tooth / b.many(14, 5), radial = [0, Math.sin(angle), Math.cos(angle)], tangent = [0, Math.cos(angle), -Math.sin(angle)];
            b.box([x, y + radial[1] * r, z + radial[2] * r], [0.49, 0.075, 0.10], c.bright, [[1, 0, 0], radial, tangent]);
          } } else {
            b.cylinder([x + side * 0.283, y, z], [x + side * 0.302, y, z], r * 0.98, c.steel, 18);
            if (b.keep(0.30)) for (let spoke = 0; spoke < b.many(8, 3); spoke++) {
              const angle = TAU * spoke / b.many(8, 3), radial = [0, Math.sin(angle), Math.cos(angle)], tangent = [0, Math.cos(angle), -Math.sin(angle)];
              b.box([x + side * 0.316, y + radial[1] * r * 0.50, z + radial[2] * r * 0.50], [0.030, r * 0.60, 0.075], c.shade, [[1, 0, 0], radial, tangent]);
            }
            b.box([side * (hullWidth - 0.02), 1.02, z + end * 0.22], [0.34, 0.15, 0.36], c.shade);
          }
        }
        if (b.keep(0.30)) for (let i = 0; i < b.many(3, 1); i++) {
          const z = -wheelHalfSpan * 0.66 + i * wheelHalfSpan * 0.66;
          b.cylinder([x - 0.18, 1.13, z], [x + 0.18, 1.13, z], 0.135, c.rubber, 12);
          b.box([side * (hullWidth - 0.03), 1.16, z], [0.30, 0.10, 0.13], c.steel);
        }
      });
      b.part(`chassis / ${label} fenders, segmented skirts and fixtures`, () => {
        b.box([x, 1.43, -0.08], [trackWidth + 0.11, 0.085, (wheelHalfSpan + 0.54) * 2], c.shade);
        const outerX = x + side * (trackWidth / 2 + 0.055), panels = b.many(heavy ? 7 : 6, 2);
        for (let i = 0; i < panels; i++) {
          const z = -wheelHalfSpan + (i + 0.5) * (wheelHalfSpan * 2 / panels), panelLength = wheelHalfSpan * 2 / panels - 0.035;
          beveled(b, [outerX, 1.23, z], [b.gauge(0.065), reinforced ? 0.55 : 0.36, panelLength], weathered(i % 2 ? c.hull : c.armor, i), 0.022);
          b.box([outerX + side * 0.039, 1.40, z], [0.018, 0.045, panelLength * 0.76], c.edge);
          if (b.keep(0.30)) for (const dz of [-panelLength * 0.34, panelLength * 0.34]) b.cylinder([outerX + side * 0.030, 1.33, z + dz], [outerX + side * 0.048, 1.33, z + dz], 0.025, c.bright, 6);
          // Hinge lugs at the top of every panel: skirts swing up for track work,
          // and without the lugs the row reads as one painted stripe.
          if (b.keep(0.30)) for (const dz of [-panelLength * 0.37, panelLength * 0.37]) b.box([outerX - side * 0.026, 1.418, z + dz], [0.052, 0.075, 0.052], c.steel);
        }
        for (const end of [-1, 1]) b.box([x, 1.08, end * (wheelHalfSpan + 0.55)], [trackWidth + 0.06, 0.57, b.gauge(0.046)], c.rubber);
        if (b.keep(0.30)) {
          b.box([side * (hullWidth - 0.16), 1.53, front - 0.48], [0.25, 0.15, 0.21], c.shade);
          b.box([side * (hullWidth - 0.16), 1.55, front - 0.37], [0.16, 0.078, 0.012], c.amber);
        }
        if (b.keep(0.30)) {
          for (const dx of [-0.12, 0.12]) b.rod([side * (hullWidth - 0.16) + dx, 1.53, front - 0.35], [side * (hullWidth - 0.16) + dx, 1.72, front - 0.35], 0.012, c.steel, 6);
          b.rod([side * (hullWidth - 0.16) - 0.12, 1.72, front - 0.35], [side * (hullWidth - 0.16) + 0.12, 1.72, front - 0.35], 0.012, c.steel, 6);
        }
        // Mudguard brackets under the fender lip: three a side, and they are
        // the shadow that keeps the fender from reading as a floating plank.
        if (b.keep(0.30)) for (let i = 0; i < b.many(3, 1); i++)
          b.rod([x - trackWidth / 2, 1.385, (i - 1) * wheelHalfSpan * 0.80], [x + trackWidth / 2, 1.385, (i - 1) * wheelHalfSpan * 0.80], 0.026, c.steel, 6);
      });
      b.part(`chassis / ${label} sponson stowage bins and tool stowage`, () => {
        // Bins ride the fender rather than the hull side, because the hull side is
        // armour. They are also the layer that stops the flank reading as one long
        // green plate between the skirt line and the turret.
        const top = 1.4725;
        for (let i = 0; i < b.many(3, 1); i++) {
          const z = (i - 1) * wheelHalfSpan * 0.70, length = i === 1 ? 1.06 : 0.88;
          beveled(b, [x, top + 0.17, z], [trackWidth + 0.03, 0.33, length], weathered(c.hull, i + (side > 0 ? 1 : 0)), 0.06);
          b.box([x, top + 0.345, z], [trackWidth + 0.06, 0.026, length + 0.03], c.edge);
          if (b.keep(0.30)) for (const dz of [-length * 0.30, length * 0.30]) b.box([x + side * (trackWidth / 2 + 0.005), top + 0.20, z + dz], [0.030, 0.090, 0.055], c.bright);
          if (b.keep(0.30)) b.rod([x - 0.15, top + 0.372, z - length * 0.35], [x + 0.15, top + 0.372, z - length * 0.35], 0.016, c.steel, 6);
        }
        // Pioneer tools clamped outboard: two long thin runs that catch the light
        // along the whole flank for the price of two rods.
        if (b.keep(0.30)) for (let i = 0; i < 2; i++) b.rod([x + side * (trackWidth / 2 + 0.055), top + 0.11 + i * 0.115, -0.62], [x + side * (trackWidth / 2 + 0.055), top + 0.11 + i * 0.115, 0.66], 0.027, i ? c.steel : c.cable, 8);
        // Tie-down sockets along the bin line. Five per side, and they are what a
        // crew actually lashes stowage to.
        if (b.keep(0.30)) for (let i = 0; i < b.many(5, 2); i++)
          b.socket([x, top + 0.36, -1.30 + i * 0.65], [x, top + 0.44, -1.30 + i * 0.65], 0.070, 0.042, c.steel, 18);
      });
    }

    b.part("chassis / driver hatch, periscopes and glacis detail", () => {
      b.loft([octagon(1.545, 0.37, front - 1.27, front - 0.70, 0.10), octagon(1.60, 0.33, front - 1.24, front - 0.73, 0.09)], c.edge);
      if (b.keep(0.30)) for (const x of [-0.26, 0, 0.26]) {
        b.box([x, 1.64, front - 0.77], [0.19, 0.12, 0.12], c.shade);
        b.box([x, 1.65, front - 0.705], [0.14, 0.053, 0.012], c.glass);
      }
      if (b.keep(0.30)) {
        b.rod([-0.12, 1.65, front - 1.10], [-0.12, 1.70, front - 1.10], 0.019, c.steel);
        b.rod([0.12, 1.65, front - 1.10], [0.12, 1.70, front - 1.10], 0.019, c.steel);
        b.rod([-0.12, 1.70, front - 1.10], [0.12, 1.70, front - 1.10], 0.019, c.steel);
      }
      // The driver's hatch itself: a turned lid on a turned ring, which is the
      // one round thing on the whole glacis.
      if (b.keep(0.30)) {
        b.revolve([0, 1.575, front - 0.98], [0, 1, 0], [
          { r: 0.30, h: 0 }, { r: 0.30, h: 0.045, s: 1 }, { r: 0.275, h: 0.075, s: 1 },
          { r: 0.245, h: 0.088 }, { r: 0, h: 0.088 }], c.upper, 12);
        b.studs([0, 1.578, front - 0.98], [0, 1, 0], 0.285, 10, 0.028, 0.022, c.bright);
      }
      for (const x of [-0.78, 0.78]) {
        if (b.keep(0.30)) b.rod([x, 1.40, front - 0.20], [x, 1.23, front + 0.005], 0.025, c.bright);
        if (b.keep(0.30)) for (let i = 0; i < b.many(5, 2); i++) b.box([x + (i - 2) * 0.11, 1.487, front - 0.73], [0.063, 0.018, 0.11], c.shade);
      }
    });

    b.part(`mobility / ${mobile ? "high-output powertrain exhaust and cooling" : "standard powertrain exhaust and cooling"}`, () => {
      const engineRear = rear + 0.37, engineFront = -1.51, mid = (engineRear + engineFront) / 2;
      b.box([0, 1.563, mid], [hullWidth * 1.55, 0.045, engineFront - engineRear], c.shade);
      const louvres = b.many(mobile ? 18 : 12, 3);
      if (b.keep(0.30)) for (let i = 0; i < louvres; i++) {
        const z = engineRear + 0.06 + (engineFront - engineRear - 0.12) * i / (louvres - 1);
        for (const x of [-hullWidth * 0.40, hullWidth * 0.40]) b.box([x, 1.608, z], [hullWidth * 0.67, 0.040, mobile ? 0.032 : 0.045], c.edge);
      }
      // Raised frames around each louvre bank. Bare stripes on a flat lid read as
      // paint from above, and the deck is most of the plan view on a card.
      for (const x of [-hullWidth * 0.40, hullWidth * 0.40]) {
        for (const dx of [-hullWidth * 0.365, hullWidth * 0.365]) b.box([x + dx, 1.601, mid], [0.055, 0.078, engineFront - engineRear - 0.02], c.upper);
        for (const dz of [engineRear + 0.028, engineFront - 0.028]) b.box([x, 1.601, dz], [hullWidth * 0.79, 0.078, 0.055], c.upper);
        // The fan under each bank, turned, seen through the grille from above.
        if (b.keep(0.30)) b.revolve([x, 1.545, mid], [0, 1, 0], [
          { r: 0, h: 0 }, { r: 0.40, h: 0 }, { r: 0.40, h: 0.048, s: 1 },
          { r: 0.36, h: 0.070, s: 1 }, { r: 0, h: 0.070 }], c.steel, 20);
      }
      // Bolted powerpack access panel forward of the grilles, clear of the ring.
      beveled(b, [0, 1.585, engineFront + 0.30], [0.86, 0.075, 0.44], c.edge, 0.07);
      if (b.keep(0.30)) {
        for (const dx of [-0.24, 0.24]) b.box([dx, 1.632, engineFront + 0.30], [0.072, 0.048, 0.28], c.bright);
        b.studs([0, 1.610, engineFront + 0.30], [0, 1, 0], 0.34, 8, 0.030, 0.024, c.bright);
      }
      // Pintle and light clusters. The rear plate is the second view a card crop
      // shows after the three-quarter, and it was blank.
      beveled(b, [0, 1.02, rear - 0.20], [0.34, 0.26, 0.22], c.steel, 0.05);
      b.tube([0, 1.02, rear - 0.30], [0, 1.02, rear - 0.255], 0.105, 0.058, c.bright, 12);
      if (b.keep(0.30)) for (const side of [-1, 1]) {
        b.box([side * (hullWidth - 0.42), 1.40, rear - 0.10], [0.20, 0.17, 0.10], c.shade);
        b.box([side * (hullWidth - 0.42), 1.40, rear - 0.152], [0.13, 0.10, 0.012], side > 0 ? c.amber : c.lens);
      }
      if (mobile) {
        for (const x of [-0.60, 0.60]) {
          b.cylinder([x, 1.57, rear + 0.79], [x, 1.69, rear + 0.79], 0.32, c.steel, 24);
          if (b.keep(0.30)) for (let stripe = -3; stripe <= 3; stripe++) {
            const z = rear + 0.79 + stripe * 0.074, length = 2 * Math.sqrt(0.29 * 0.29 - Math.pow(stripe * 0.074, 2));
            b.box([x, 1.703, z], [length, 0.026, 0.025], c.edge);
          }
        }
        for (const x of [-0.67, 0.67]) {
          b.box([x, 1.16, rear - 0.08], [0.61, 0.31, 0.25], c.steel);
          b.box([x, 1.16, rear - 0.213], [0.48, 0.21, 0.018], c.black);
          if (b.keep(0.30)) for (let i = -2; i <= 2; i++) b.box([x, 1.16 + i * 0.040, rear - 0.227], [0.50, 0.014, 0.030], c.bright);
        }
      } else {
        for (const x of [-0.74, 0.74]) {
          b.cylinder([x, 1.11, rear - 0.04], [x, 1.11, rear - 0.26], 0.19, c.steel, 24);
          b.tube([x, 1.11, rear - 0.13], [x, 1.11, rear - 0.31], 0.165, 0.125, c.bright, 24);
          b.cylinder([x, 1.11, rear - 0.08], [x, 1.11, rear - 0.14], 0.12, c.black, 20);
          // Exhaust shroud: a turned jacket over the pipe, which is the shape a
          // silencer actually has and the only round thing on the rear plate.
          if (b.keep(0.30)) b.revolve([x, 1.11, rear - 0.02], [0, 0, -1], [
            { r: 0.205, h: 0 }, { r: 0.235, h: 0.055, s: 1 }, { r: 0.235, h: 0.20, s: 1 },
            { r: 0.205, h: 0.255 }, { r: 0.175, h: 0.255 }], c.upper, 20);
        }
      }
      // Air cleaners. Every engine deck on the model was flat plate and stripes
      // until these two drums stood up off it.
      for (const side of [-1, 1]) if (b.keep(0.30)) b.revolve([side * (hullWidth - 0.60), 1.585, rear + 1.30], [0, 1, 0], [
        { r: 0.11, h: 0 }, { r: 0.235, h: 0.055, s: 1 }, { r: 0.255, h: 0.150, s: 1 },
        { r: 0.255, h: 0.330, s: 1 }, { r: 0.225, h: 0.395, s: 1 }, { r: 0.110, h: 0.420 }], c.steel, 24);
      for (const x of [-hullWidth + 0.18, hullWidth - 0.18]) {
        b.box([x, 1.635, rear + 0.72], [0.21, 0.18, 0.75], c.hull);
        if (b.keep(0.30)) for (const z of [rear + 0.41, rear + 1.01]) b.box([x, 1.733, z], [0.22, 0.02, 0.053], c.bright);
      }
    });

    b.part("turret / ring and faceted armor shell", () => {
      if(!casemate)b.cylinder([0, 1.53, turretZ], [0, turretY + 0.06, turretZ], turretWidth * 0.88, c.steel, 48);
      else b.box([0,1.57,-.1],[turretWidth*1.95,.27,2.9],c.hull);
      if(casemate)b.loft([octagon(1.47,hullWidth*.97,-1.87,2.23,.16),octagon(turretTop,turretWidth*.82,-1.35,.87,.14)],c.upper);
      // The extra ring below the roof turns one long slab side into a shoulder
      // and a roof chamfer, which is what makes a turret read as cast armour.
      else b.loft([octagon(turretY, turretWidth * 0.88, -1.56, 1.20, 0.35, turretZ),
        octagon(turretY + 0.23, turretWidth, -1.64, 1.37, 0.47, turretZ),
        octagon(turretTop - 0.26, turretWidth * 0.92, -1.50, 1.04, 0.42, turretZ),
        octagon(turretTop - 0.08, turretWidth * 0.84, -1.36, 0.83, 0.37, turretZ),
        octagon(turretTop, turretWidth * 0.77, -1.25, 0.73, 0.32, turretZ)], c.upper);
      // A bolted collar at the ring. Without it the shell floats on the deck;
      // with it the two big volumes get a shadow line where they meet.
      if (!casemate) {
        b.cylinder([0, 1.495, turretZ], [0, 1.578, turretZ], turretWidth * 0.96, c.shade, 24);
        if (b.keep(0.30)) for (let i = 0; i < b.many(12, 4); i++) {
          const angle = TAU * i / b.many(12, 4);
          b.box([Math.sin(angle) * turretWidth * 0.96, 1.588, turretZ + Math.cos(angle) * turretWidth * 0.96], [0.075, 0.030, 0.075], c.bright);
        }
      }
      // Separate cheek castings belong to the rotating turret, not the fixed hull mounting.
      for (const side of casemate?[]:[-1, 1]) {
        const cheek = [[side * 0.43, turretY + 0.25, 1.49], [side * (turretWidth - 0.27), turretY + 0.28, 1.45],
          [side * turretWidth, turretY + 0.29, 0.94], [side * (turretWidth - 0.16), turretTop - 0.10, 0.73],
          [side * 0.46, turretTop - 0.13, 0.89]];
        const inside = cheek.map(point => [point[0], point[1] - 0.10, point[2] - 0.055]), center = mean([...cheek, ...inside]);
        b.face(cheek, c.armor, center); b.face(inside, c.shade, center);
        for (let i = 0; i < cheek.length; i++) { const next = (i + 1) % cheek.length; b.face([cheek[i], cheek[next], inside[next], inside[i]], c.edge, center); }
        // Lifting eye on each shoulder, which is where a turret is actually slung.
        if (b.keep(0.30)) b.socket([side * (turretWidth - 0.20), turretTop - 0.13, -0.62], [side * (turretWidth + 0.02), turretTop - 0.05, -0.62], 0.080, 0.048, c.steel, 18);
      }
      beveled(b, [0, turretY + 0.41, -1.59], [turretWidth * 1.56, 0.38, 0.27], c.shade, 0.07);
      if (b.keep(0.30)) for (const x of [-turretWidth * 0.57, 0, turretWidth * 0.57]) b.box([x, turretY + 0.56, -1.739], [0.27, 0.09, 0.024], c.steel);
    });

    b.part(`armament / ${heavyGun ? "heavy weapon, enlarged mantlet and sleeved barrel" : "standard weapon, mantlet and barrel"}`, () => {
      const gunY = turretY + 0.49, radius = chosen.armament==='gun_90'?.085:chosen.armament==='gun_125'?.15:heavyGun ? 0.132 : 0.105, muzzle = chosen.armament==='gun_90'?4.78:chosen.armament==='gun_125'?6.67:heavyGun ? 6.36 : 5.68;
      beveled(b, [0, gunY, 1.20], [heavyGun ? 0.90 : 0.75, heavyGun ? 0.68 : 0.55, 0.61], c.shade, 0.10);
      b.cylinder([-0.45, gunY, 1.45], [0.45, gunY, 1.45], heavyGun ? 0.35 : 0.29, c.armor, 32);
      b.cylinder([0, gunY, 1.46], [0, gunY, 1.99], radius * 1.85, c.steel, 32, radius * 1.34);
      // Canvas boot over the mantlet gap. It is the only soft material on the
      // model, and it stops the barrel reading as a rod pushed into a box.
      b.cylinder([0, gunY, 1.63], [0, gunY, 1.94], radius * 2.42, c.canvas, 16, radius * 1.54);
      if (b.keep(0.30)) for (const [z, scale] of [[1.66, 2.44], [1.89, 1.70]]) b.cylinder([0, gunY, z], [0, gunY, z + 0.028], radius * scale, c.steel, 16);
      b.cylinder([0, gunY, 1.94], [0, gunY, muzzle - 0.23], radius * 1.10, c.upper, 36, radius);
      // The bore evacuator is a turned drum, not a straight sleeve: it swells at
      // the middle and tapers back to the tube at each end.
      b.revolve([0, gunY, 3.24], [0, 0, 1], [
        { r: radius * 1.06, h: 0 }, { r: radius * (heavyGun ? 1.62 : 1.47), h: 0.15, s: 1 },
        { r: radius * (heavyGun ? 1.66 : 1.51), h: 0.42, s: 1 }, { r: radius * (heavyGun ? 1.62 : 1.47), h: 0.66, s: 1 },
        { r: radius * 1.06, h: 0.80 }], c.shade, 32);
      const sleeveCount = b.many(heavyGun ? 7 : 4, 2);
      if (b.keep(0.30)) for (let i = 0; i < sleeveCount; i++) {
        const z = 2.31 + (muzzle - 2.80) * i / (sleeveCount - 1);
        b.cylinder([0, gunY, z], [0, gunY, z + 0.063], radius * 1.19, c.bright, 32);
      }
      // The thermal sleeve is the barrel's outermost turned surface, and the
      // straps that clamp it are what break the tube into readable lengths.
      if (b.keep(0.30)) {
        const sleeve = muzzle - 1.90;
        b.revolve([0, gunY, sleeve], [0, 0, 1], [
          { r: radius * 1.12, h: 0 }, { r: radius * 1.30, h: 0.09, s: 1 }, { r: radius * 1.30, h: 0.52, s: 1 },
          { r: radius * 1.26, h: 0.90, s: 1 }, { r: radius * 1.26, h: 1.24, s: 1 }, { r: radius * 1.10, h: 1.33 }], c.upper, 32);
        b.cylinder([0, gunY, 1.24], [0, gunY, 1.42], radius * 2.30, c.steel, 24);
        for (let strap = 0; strap < 4; strap++) b.cylinder([0, gunY, sleeve + 0.12 + strap * 0.33], [0, gunY, sleeve + 0.16 + strap * 0.33], radius * 1.38, c.bright, 8);
        b.cylinder([0, gunY, muzzle - 0.62], [0, gunY, muzzle - 0.50], radius * 1.22, c.shade, 10);
      }
      b.always(() => b.tube([0, gunY, muzzle - 0.42], [0, gunY, muzzle], radius * 1.06, radius * 0.74, c.steel, 40));
      b.cylinder([0, gunY, muzzle - 0.455], [0, gunY, muzzle - 0.425], radius * 0.73, c.black, 32);
      if (b.keep(0.30)) {
        b.box([0, gunY + radius * 1.12, muzzle - 0.21], [0.088, 0.052, 0.18], c.shade);
        b.box([0.37, gunY - 0.02, 1.54], [0.14, 0.13, 0.12], c.black);
        b.tube([0.37, gunY - 0.02, 1.55], [0.37, gunY - 0.02, 1.76], 0.041, 0.025, c.steel, 16);
      }
    });

    b.part("turret / crew hatches, cupola and fittings", () => {
      for (const hatch of [{ x: -0.48, z: -0.36, radius: 0.35 }, { x: 0.48, z: -0.24, radius: 0.30 }]) {
        b.cylinder([hatch.x, turretTop - 0.018, hatch.z], [hatch.x, turretTop + 0.13, hatch.z], hatch.radius, c.shade, 32);
        b.cylinder([hatch.x, turretTop + 0.129, hatch.z], [hatch.x, turretTop + 0.19, hatch.z], hatch.radius * 0.96, c.edge, 32, hatch.radius * 0.91);
        if (b.keep(0.30)) {
          b.box([hatch.x, turretTop + 0.205, hatch.z - 0.02], [0.20, 0.043, 0.039], c.steel);
          for (const dx of [-0.15, 0.15]) b.box([hatch.x + dx, turretTop + 0.13, hatch.z - hatch.radius], [0.065, 0.085, 0.105], c.bright);
        }
        if (b.keep(0.30)) for (let i = 0; i < b.many(5, 2); i++) {
          const angle = -0.84 + i * 0.42, x = hatch.x + Math.sin(angle) * (hatch.radius + 0.02), z = hatch.z + Math.cos(angle) * (hatch.radius + 0.02);
          b.box([x, turretTop + 0.065, z], [0.085, 0.055, 0.060], c.glass);
        }
      }
      // Pintle machine gun beside the commander's hatch. At card size this is the
      // single detail that separates a turret from a smooth casting; it is a crew
      // fitting, not the platform's armament, and carries no game capability.
      const pintle = [-0.18, turretTop, -0.20];
      if (b.keep(0.30)) {
        b.cylinder([pintle[0], pintle[1] + 0.18, pintle[2]], [pintle[0], pintle[1] + 0.44, pintle[2]], b.gauge(0.045), c.steel, 12);
        b.cylinder([pintle[0], pintle[1] + 0.44, pintle[2]], [pintle[0], pintle[1] + 0.50, pintle[2]], 0.075, c.shade, 12);
        b.box([pintle[0], pintle[1] + 0.545, pintle[2] + 0.16], [0.12, 0.14, 0.32], c.shade);
        b.box([pintle[0], pintle[1] + 0.46, pintle[2] + 0.10], [0.19, 0.16, 0.22], c.black);
        b.cylinder([pintle[0], pintle[1] + 0.565, pintle[2] + 0.30], [pintle[0], pintle[1] + 0.565, pintle[2] + 0.76], b.gauge(0.032), c.steel, 12);
        b.box([pintle[0], pintle[1] + 0.625, pintle[2] + 0.40], [0.048, 0.042, 0.28], c.bright);
        b.box([pintle[0] + 0.15, pintle[1] + 0.50, pintle[2] + 0.08], [0.17, 0.20, 0.20], c.armor);
      }
      for (const side of [-1, 1]) {
        const x = side * turretWidth * 0.79;
        if (b.keep(0.30)) {
          b.rod([x, turretTop - 0.12, -0.55], [x, turretTop + 0.035, -0.55], 0.022, c.steel);
          b.rod([x, turretTop - 0.12, 0.16], [x, turretTop + 0.035, 0.16], 0.022, c.steel);
          b.rod([x, turretTop + 0.035, -0.55], [x, turretTop + 0.035, 0.16], 0.022, c.steel);
        }
        if (b.keep(0.30)) for (let i = 0; i < b.many(5, 2); i++) {
          const z = -0.44 + i * 0.20;
          b.cylinder([side * (turretWidth + 0.035), turretY + 0.35, z], [side * (turretWidth + 0.047), turretY + 0.35, z], 0.024, c.bright, 6);
        }
        if (b.keep(0.30)) for (let tube = 0; tube < b.many(4, 2); tube++) {
          const start = [side * (turretWidth - 0.035), turretY + 0.45, 0.34 + tube * 0.16];
          const end = add(start, [side * 0.23, 0.15, 0.10]);
          b.cylinder(start, end, 0.052, c.steel, 12);
          b.cylinder(end, add(end, [side * 0.018, 0.012, 0.008]), 0.054, c.black, 12);
        }
      }
    });

    b.part(`sensors / ${integrated ? "integrated panoramic sight and fire-control optics" : "optical sight and rangefinder housing"}`, () => {
      const sightX = integrated ? -0.53 : -0.45, sightZ = integrated ? 0.58 : 0.65;
      if (integrated) {
        b.cylinder([sightX, turretTop - 0.05, sightZ], [sightX, turretTop + 0.21, sightZ], 0.23, c.steel, 24);
        b.loft([octagon(turretTop + 0.17, 0.23, -0.22, 0.22, 0.07).map(p => add(p, [sightX, 0, sightZ])),
          octagon(turretTop + 0.51, 0.20, -0.19, 0.19, 0.06).map(p => add(p, [sightX, 0, sightZ]))], c.shade);
        b.box([sightX, turretTop + 0.36, sightZ + 0.203], [0.28, 0.19, 0.025], c.black);
        for (const dx of [-0.078, 0.078]) b.cylinder([sightX + dx, turretTop + 0.36, sightZ + 0.219], [sightX + dx, turretTop + 0.36, sightZ + 0.228], 0.063, c.lens, 20);
        b.box([0.54, turretTop - 0.16, 0.91], [0.36, 0.22, 0.17], c.shade);
        b.box([0.54, turretTop - 0.14, 1.006], [0.24, 0.115, 0.021], c.glass);
        b.box([0.54, turretTop - 0.008, 0.98], [0.39, 0.035, 0.26], c.edge);
      } else {
        b.box([sightX, turretTop + 0.015, sightZ], [0.34, 0.18, 0.33], c.shade);
        b.box([sightX, turretTop + 0.04, sightZ + 0.178], [0.25, 0.070, 0.025], c.glass);
        b.box([sightX, turretTop + 0.116, sightZ + 0.01], [0.39, 0.03, 0.37], c.edge);
        // Rangefinder head: a turned housing with a glass eye, rather than a peg.
        b.revolve([0.65, turretTop - 0.24, 1.002], [0, 0, 1], [
          { r: 0, h: 0 }, { r: 0.090, h: 0 }, { r: 0.098, h: 0.075, s: 1 },
          { r: 0.090, h: 0.169 }, { r: 0.069, h: 0.169 }], c.steel, 28);
        b.revolve([0.65, turretTop - 0.24, 1.002], [0, 0, 1], [
          { r: 0.067, h: 0.168, s: 1 }, { r: 0.055, h: 0.180, s: 1 }, { r: 0, h: 0.184 }], c.glass, 28);
      }
    });

    b.part(`communications / ${data ? "tactical data aerials and terminal" : "field radio whip aerial"}`, () => {
      const aerials = data ? [{ x: -0.83, z: -0.92, height: 1.30 }, { x: 0.84, z: -1.01, height: 0.92 }] : [{ x: 0.75, z: -0.97, height: 1.45 }];
      for (const antenna of aerials) {
        const bottom = turretTop - 0.05;
        b.cylinder([antenna.x, bottom, antenna.z], [antenna.x, bottom + 0.11, antenna.z], 0.084, c.steel, 16);
        b.cylinder([antenna.x, bottom + 0.11, antenna.z], [antenna.x, bottom + 0.26, antenna.z], 0.040, c.rubber, 16, 0.029);
        b.always(() => b.cylinder([antenna.x, bottom + 0.25, antenna.z], [antenna.x + 0.045, bottom + antenna.height, antenna.z - 0.12], b.gauge(0.013), c.steel, 8, b.gauge(0.007)));
        if (b.keep(0.30)) for (let turn = 0; turn < b.many(4, 2); turn++) b.cylinder([antenna.x, bottom + 0.115 + turn * 0.027, antenna.z], [antenna.x, bottom + 0.130 + turn * 0.027, antenna.z], 0.046, c.bright, 12);
      }
      if (data) {
        b.box([0.27, turretTop + 0.045, -0.96], [0.35, 0.13, 0.30], c.shade);
        b.cylinder([0.27, turretTop + 0.10, -0.96], [0.27, turretTop + 0.18, -0.96], 0.175, c.edge, 24, 0.14);
        b.box([turretWidth - 0.12, turretY + 0.59, -0.93], [0.18, 0.22, 0.49], c.hull);
        if (b.keep(0.30)) b.rod([0.82, turretTop - 0.10, -1.0], [0.68, turretTop + 0.015, -0.81], 0.018, c.cable);
      }
    });

    b.part(`protection / ${reinforced ? "reinforced modular armor blocks" : active ? "active-protection sensors and intercept modules" : "standard armor fixtures"}`, () => {
      if (reinforced) {
        for (const side of [-1, 1]) {
          for (let row = 0; row < 2; row++) for (let i = 0; i < b.many(4, 2); i++) {
            const z = -0.92 + i * 0.47, y = turretY + 0.34 + row * 0.235, x = side * (turretWidth + 0.05 - row * 0.075);
            b.box([x, y, z], [0.23, 0.21, 0.40], i % 2 ? c.armor : c.hull);
            if (b.keep(0.30)) b.box([x + side * 0.122, y, z], [0.018, 0.13, 0.30], c.edge);
          }
          const cheekX = side * 0.89;
          for (let i = 0; i < b.many(3, 1); i++) {
            const y = turretY + 0.28 + i * 0.17, z = 1.50 - i * 0.16;
            b.box([cheekX, y, z], [0.65, 0.14, 0.18], c.armor);
          }
          for (let i = 0; i < b.many(6, 2); i++) {
            const z = -wheelHalfSpan + 0.42 + i * (wheelHalfSpan * 2 - 0.84) / 5;
            b.box([side * (trackX + trackWidth / 2 + 0.13), 1.18, z], [b.gauge(0.12), 0.43, 0.61], c.armor);
            if (b.keep(0.30)) b.box([side * (trackX + trackWidth / 2 + 0.198), 1.18, z], [0.014, 0.33, 0.50], c.edge);
          }
        }
        for (let row = 0; row < 2; row++) for (let i = 0; i < b.many(5, 3); i++) b.box([(i - 2) * 0.40, 1.37 - row * 0.15, front - 0.16 + row * 0.19], [0.36, 0.15, 0.19], c.armor);
      }
      if(!active&&!reinforced) {
        for (const side of [-1, 1]) for (let i = 0; i < b.many(3, 1); i++) {
          const x = side * (turretWidth - 0.02), z = -0.93 + i * 0.47;
          b.box([x, turretY + 0.43, z], [b.gauge(0.052), 0.17, 0.30], c.armor);
        }
      }
    });
    if(active)b.part(`${chosen.active_protection==='aps_hard'?'active_protection':'protection'} / active-protection sensors and intercept modules`,()=>{
        for (const side of [-1, 1]) {
          for (const z of b.many(2, 2) > 1 ? [-1.06, 0.56] : [0.56]) {
            b.box([side * (turretWidth - 0.075), turretTop - 0.19, z], [0.23, 0.28, 0.29], c.shade);
            b.box([side * (turretWidth + 0.05), turretTop - 0.17, z], [0.018, 0.19, 0.22], c.glass);
            if (b.keep(0.30)) for (let stripe = -2; stripe <= 2; stripe++) b.box([side * (turretWidth + 0.063), turretTop - 0.17 + stripe * 0.033, z], [0.009, 0.008, 0.205], c.steel);
          }
          const x = side * (turretWidth + 0.035);
          b.cylinder([x, turretY + 0.30, -0.26], [x, turretY + 0.48, -0.26], 0.17, c.steel, 20);
          b.box([x, turretY + 0.58, -0.24], [0.30, 0.21, 0.48], c.armor);
          if (b.keep(0.30)) for (const offset of [-0.11, 0.11]) b.cylinder([x + side * 0.04, turretY + 0.59, -0.24 + offset], [x + side * 0.23, turretY + 0.67, -0.24 + offset], 0.068, c.steel, 14);
          if (b.keep(0.30)) b.rod([side * (turretWidth - 0.07), turretTop - 0.27, -1.12], [side * (turretWidth - 0.07), turretTop - 0.27, 0.53], 0.018, c.cable);
        }
        b.box([0, turretTop + 0.02, -1.05], [0.36, 0.12, 0.29], c.shade);
    });

    b.part("stowage / bustle rack, canvas rolls, tow cable and tools", () => {
      const rackRear = -2.03, rackFront = -1.50, rackY = turretY + 0.24, rackHalf = turretWidth * 0.82;
      b.box([0, rackY, (rackRear + rackFront) / 2], [rackHalf * 2, 0.047, rackFront - rackRear], c.steel);
      if (b.keep(0.30)) for (const x of [-rackHalf, rackHalf]) {
        b.rod([x, rackY, rackRear], [x, rackY + 0.43, rackRear], 0.023, c.bright);
        b.rod([x, rackY + 0.43, rackRear], [x, rackY + 0.43, rackFront], 0.023, c.bright);
        b.rod([x, rackY, rackFront], [x, rackY + 0.43, rackFront], 0.023, c.bright);
      }
      if (b.keep(0.30)) for (const height of [0.15, 0.40]) b.rod([-rackHalf, rackY + height, rackRear], [rackHalf, rackY + height, rackRear], 0.022, c.bright);
      if (b.keep(0.30)) for (let i = 0; i < b.many(9, 3); i++) {
        const x = -rackHalf + i * rackHalf / 4;
        b.rod([x, rackY, rackRear], [x, rackY + 0.40, rackRear], 0.013, c.steel, 6);
      }
      for (const x of [-0.53, 0.11, 0.60]) {
        b.box([x, rackY + 0.15, -1.77], [0.40, 0.23, 0.37], x < 0 ? c.canvas : c.shade);
        if (b.keep(0.30)) {
          b.box([x, rackY + 0.279, -1.77], [0.41, 0.025, 0.38], c.edge);
          b.box([x, rackY + 0.289, -1.77], [0.045, 0.012, 0.40], c.steel);
        }
      }
      b.cylinder([-0.60, turretTop + 0.065, -1.03], [0.32, turretTop + 0.065, -1.03], 0.105, c.canvas, 16);
      if (b.keep(0.30)) for (const x of [-0.43, 0.16]) b.cylinder([x, turretTop + 0.065, -1.03], [x + 0.04, turretTop + 0.065, -1.03], 0.110, c.steel, 16);
      const cablePoints = [[-hullWidth + 0.10, 1.56, -0.70], [-hullWidth + 0.10, 1.59, -1.60], [-hullWidth + 0.19, 1.60, rear + 0.21],
        [0, 1.61, rear + 0.16], [hullWidth - 0.19, 1.60, rear + 0.21], [hullWidth - 0.10, 1.59, -1.60], [hullWidth - 0.10, 1.56, -0.70]];
      if (b.keep(0.30)) for (let i = 0; i < cablePoints.length - 1; i++) b.rod(cablePoints[i], cablePoints[i + 1], b.gauge(0.031), c.cable, 10);
      // The cable ends in eyes, because a cable that ends in nothing reads as a
      // pipe. Two sockets, one at each end of the run.
      if (b.keep(0.30)) {
        for (const x of [-hullWidth + 0.10, -0.34, 0.34, hullWidth - 0.10])
          b.socket([x, 1.56, -0.70], [x, 1.56, -0.52], 0.075, 0.045, c.steel, 18);
        b.cylinder([hullWidth - 0.44, 1.60, -0.30], [hullWidth - 0.44, 1.60, 0.62], 0.062, c.canvas, 18);
      }
      if (b.keep(0.30)) {
        b.rod([hullWidth - 0.14, 1.59, 0.06], [hullWidth - 0.14, 1.59, 1.21], 0.027, c.canvas, 10);
        b.box([hullWidth - 0.14, 1.59, 1.24], [0.18, 0.052, 0.25], c.steel);
        for (const z of [0.20, 0.95]) b.box([hullWidth - 0.14, 1.626, z], [0.12, 0.035, 0.052], c.bright);
      }
    });

    // THE AMMUNITION LOAD, which until this pass had no geometry at all on any
    // tank: the three ids were metadata, the contract exempted the slot, and a
    // player who changed the load watched an unchanged model. Roadmap section A
    // says what it should be — "readable stowage/inspection representation, not
    // detailed weapon internals" — so this is the bustle magazine a resupply
    // party sees: a compartment hung on the back of the stowage rack, its roof
    // carrying the blow-off panels that vent a magazine fire upward, its door
    // swung open on the ready rack inside. What changes between the three loads
    // is the SHAPE of that stowage — the depth of the compartment, the run of the
    // panels, and what is racked in it — never its colour.
    if(chosen.ammunition)b.part(`ammunition / ${AMMO_LABEL[chosen.ammunition]} ready rack and blow-off panels`,()=>{
      const pen=chosen.ammunition==='ammo_penetrator',sup=chosen.ammunition==='ammo_support';
      const rackY=turretY+0.24,bayHalf=turretWidth*(sup?0.72:0.66);
      const bayBack=pen?-2.62:sup?-2.30:-2.44,bayFront=-2.00,depth=bayFront-bayBack;
      const bayH=sup?0.50:0.42,bayY=rackY+bayH/2,mid=(bayBack+bayFront)/2,roof=bayY+bayH/2;
      // THE MAP PIN. At LOD2 the compartment is the only thing here that clears
      // the cull, and a chamfered prism costs 28 triangles against a prism's 12
      // on a level whose whole vehicle budget is 1,500. The chamfer is what makes
      // it read as a machined box at inspection range; at map range nothing can
      // see it, so it is not drawn.
      if(b.level<2)beveled(b,[0,bayY,mid],[bayHalf*2,bayH,depth],c.armor,0.06);
      else b.box([0,bayY,mid],[bayHalf*2,bayH,depth],c.armor);
      // Blow-off panels. Two long ones over a rack of long rods, four short ones
      // over a charge bin, three otherwise: the run of the panels is the plan
      // view's answer to which load is aboard.
      // Two at the map pin, not one: a single panel spans the whole compartment
      // roof, which makes it as large as the compartment and pushes it back over
      // the cull — 12 triangles the LOD2 band does not have to spare.
      const panels=pen?2:sup?4:3,drawn=b.many(panels,2,2);
      for(let i=0;i<drawn;i++) {
        const pitch=bayHalf*2/drawn,x=(i-(drawn-1)/2)*pitch,wide=pitch-0.055;
        b.box([x,roof+0.028,mid],[wide,0.055,depth-0.10],c.upper);
        b.box([x,roof+0.062,mid],[wide-0.06,0.024,depth-0.17],c.edge);
        if(b.keep(0.30))for(const dz of [-depth*0.30,depth*0.30])b.box([x,roof+0.052,mid+dz],[wide*0.72,0.030,0.035],c.bright);
      }
      // The door, hinged open to port so the rack behind it is what the eye lands
      // on. A closed plate would say nothing about the load.
      const doorH=bayH*0.78,hinge=-bayHalf-0.02;
      b.box([hinge-doorH*0.42,bayY,bayBack-0.03-depth*0.28],[doorH*0.84,doorH,0.05],c.upper,[[0,0,1],[0,1,0],[1,0,0]]);
      if(b.keep(0.30)) {
        for(const dy of [-doorH*0.32,doorH*0.32])b.revolve([hinge,bayY+dy,bayBack-0.03],[0,1,0],[
          {r:0.042,h:-0.05,s:1},{r:0.042,h:0.05,s:1},{r:0.028,h:0.075}],c.steel,10,0.14);
        b.rod([hinge-doorH*0.74,bayY-0.09,bayBack-0.02-depth*0.28],[hinge-doorH*0.74,bayY+0.09,bayBack-0.02-depth*0.28],0.022,c.bright,8);
      }
      // The ready rack, through the opening. A penetrator load is four long rods
      // that stand out past the door frame; a mixed load is two tiers of five
      // shorter rounds; a fire-support load is three fat projectiles under a row
      // of upright charge canisters, which is the one that changes the roof line.
      const tiers=pen?[[4,0.055,0.86,0.0]]:sup?[[3,0.084,0.44,0.0]]:[[3,0.052,0.52,-0.10],[2,0.076,0.46,0.10]];
      for(const [count,radius,reach,lift] of tiers)for(let i=0;i<count;i++) {
        const x=(i-(count-1)/2)*(bayHalf*1.5/count);
        b.revolve([x,bayY+lift,bayBack+reach],[0,0,-1],[
          {r:0,h:0},{r:radius,h:0.022},{r:radius,h:reach*0.70,s:1},{r:radius*0.86,h:reach*0.84,s:1},
          {r:radius*0.46,h:reach*0.96,s:1},{r:0,h:reach}],pen?c.steel:i%2?c.bright:c.shade,12,Math.hypot(radius*2,reach));
      }
      if(sup)for(let i=0;i<b.many(4,2);i++) {
        const x=(i-1.5)*(bayHalf*0.62);
        b.revolve([x,roof+0.08,mid],[0,1,0],[{r:0.082,h:0},{r:0.082,h:0.26,s:1},{r:0.062,h:0.30},{r:0,h:0.30}],c.canvas,12,0.34);
      }
      // Two brackets down to the engine deck, so the compartment is carried and
      // not floating behind the turret.
      if(b.keep(0.30))for(const side of [-1,1]) {
        b.rod([side*bayHalf*0.78,bayY-bayH/2,bayFront-0.05],[side*bayHalf*0.86,1.60,bayFront-0.34],0.030,c.steel,8);
        b.box([side*bayHalf*0.86,1.60,bayFront-0.34],[0.13,0.055,0.17],c.shade);
      }
    });

    b.part('mobility / powerpack installation',()=>{
      if(chosen.mobility==='engine_turbine_1500') {b.box([0,1.12,rear-.18],[1.35,.46,.33],c.shade);if(b.keep(0.30))for(let i=-4;i<=4;i++)b.box([i*.14,1.12,rear-.36],[.035,.35,.04],c.bright);}
      if(chosen.mobility==='engine_diesel_600') b.box([0,1.64,rear+.65],[.62,.16,.38],c.hull);
    });
    b.part('transmission / final drive housing',()=>{
      if(chosen.transmission==='transmission_auto') {b.box([0,.63,rear-.13],[.93,.30,.18],c.hull);if(b.keep(0.30))for(let i=-2;i<=2;i++)b.box([i*.16,.64,rear-.23],[.04,.2,.02],c.edge);}
    });
    b.part('turret / autoloader bustle',()=>{
      if(autoload){b.box([0,turretY+.37,-1.76],[turretWidth*1.62,.51,.59],c.armor);if(b.keep(0.30))for(let i=-2;i<=2;i++)b.box([i*.30,turretY+.635,-1.75],[.24,.025,.44],c.edge);}
    });
    b.part('active_protection / soft-kill sensors',()=>{
      if(chosen.active_protection==='aps_soft')for(const side of [-1,1]){b.box([side*turretWidth,turretTop-.10,-.7],[b.gauge(.16),.18,.2],c.shade);if(b.keep(0.30))b.box([side*(turretWidth+.09),turretTop-.10,-.7],[.025,.10,.12],c.glass);}
    });
    b.part('sensors / night observation housing',()=>{
      if(chosen.sensors==='optics_night'){b.cylinder([-.45,turretTop+.05,.55],[-.45,turretTop+.26,.55],.17,c.shade,20);b.box([-.45,turretTop+.19,.73],[.22,.13,.03],c.lens);}
    });
    b.part('fire_control / stabilization and rangefinding equipment',()=>{
      if(chosen.fire_control==='fcs_stabilized')b.cylinder([0,turretY+.49,2.02],[0,turretY+.49,2.20],.20,c.bright,24);
      if(chosen.fire_control==='fcs_digital'){b.box([.64,turretTop-.12,.87],[.33,.27,.34],c.shade);b.box([.64,turretTop-.12,1.05],[.25,.14,.025],c.lens);if(b.keep(0.30))b.rod([.64,turretTop-.24,.7],[.80,turretY+.37,.38],.022,c.cable);}
    });
    const result=b.finish(`Original ${spec.platform.replace('tank_','')} tank game model. Visual interpretation of the selected specifications. Internal ammunition loads affect game ratings and are recorded in the exported specification metadata.`);
    if(spec.platform==='tank_light'){for(let i=0;i<result.positions.length;i++)result.positions[i]*=.80;result.bounds.min=result.bounds.min.map(v=>v*.80);result.bounds.max=result.bounds.max.map(v=>v*.80);}
    result.specification={platform:spec.platform,components:Object.fromEntries(Object.entries(chosen).filter(([,value])=>value))};
    return result;
  }
  return Object.freeze({ build });
});
