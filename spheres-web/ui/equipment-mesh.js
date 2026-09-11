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

   Level of detail. build(spec) returns the inspection mesh. spec.lod
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
  // Authored surface classes follow palette tokens through color shading.
  // Consumers must not reverse-engineer material identity from final RGB.
  const materialTokens=new WeakMap();
  const MATERIAL_CODES={hull:0,upper:0,edge:0,shade:0,armor:0,steel:1,bright:1,track:2,
    rubber:3,black:3,glass:4,lens:4,canvas:5,cable:6,amber:7};
  for(const [name,color] of Object.entries(PALETTE))materialTokens.set(color,MATERIAL_CODES[name]);
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
  const AIR_CHOICES = {
    air_engine:["air_engine_economical","air_engine_efficient","air_engine_twin"],
    air_wing:["air_wing_straight","air_wing_stable","air_wing_swept"],
    air_radar:["air_radar_basic","air_radar_mapping"],
    air_avionics:["air_avionics_analog","air_avionics_digital"],
    air_countermeasures:["air_countermeasures_basic","air_countermeasures_ecm"],
    air_hardpoints:["air_hardpoints_light","air_hardpoints_heavy"],
    air_payload:["air_payload_unguided","air_payload_guided"],
    air_fuel:["air_fuel_standard","air_fuel_extended"]
  };
  const AIR_DEFAULTS = {
    air_light_attack:{air_engine:"air_engine_economical",air_wing:"air_wing_straight",air_hardpoints:"air_hardpoints_light"},
    air_tactical_strike:{air_engine:"air_engine_twin",air_wing:"air_wing_swept",air_hardpoints:"air_hardpoints_heavy"}
  };
  const add = (a, b) => [a[0] + b[0], a[1] + b[1], a[2] + b[2]];
  const sub = (a, b) => [a[0] - b[0], a[1] - b[1], a[2] - b[2]];
  const mul = (a, n) => [a[0] * n, a[1] * n, a[2] * n];
  const dot = (a, b) => a[0] * b[0] + a[1] * b[1] + a[2] * b[2];
  const cross = (a, b) => [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]];
  const normal = a => { const length = Math.hypot(...a); return length > 1e-10 ? mul(a, 1 / length) : [0, 1, 0]; };
  const mean = points => mul(points.reduce((sum, point) => add(sum, point), [0, 0, 0]), 1 / points.length);
  const shade = (color, amount) => {
    const out=color.map(value=>Math.max(0,Math.min(1,value*amount)));
    materialTokens.set(out,materialTokens.get(color)??255);
    return out;
  };

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
    const positions = [], normals = [], colors = [], materialClasses = [], parts = [], smoothing = [];
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
      materialClasses.push(materialTokens.get(color)??255);
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
    function wheelBody(center, axis, radius, halfWidth, tyre, rim, hub, dual, paired=dual) {
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
      const tyreProfile = [
        { r: radius * 0.72, h: 0, s: 1 }, { r: radius * 0.955, h: halfWidth * 0.36, s: 1 },
        { r: radius, h: halfWidth * 0.76, s: 1 }, { r: radius, h: halfWidth * 1.24, s: 1 },
        { r: radius * 0.955, h: halfWidth * 1.64, s: 1 }, { r: radius * 0.72, h: halfWidth * 2, s: 1 }
      ];
      if (D.level===0&&paired) {
        // Two actual wheels, with daylight for the track's central guide horn.
        // The old single tyre crossed that channel. Outer wheel faces retain
        // their locations, so the running-gear silhouette and unit scale stay put.
        for (const side of [-1,1]) {
          const outward=mul(a,side), seat=add(center,mul(outward,halfWidth*.44));
          revolve(seat,outward,[{r:radius*.72,h:0},{r:radius*.94,h:halfWidth*.098,s:1},
            {r:radius,h:halfWidth*.28,s:1},{r:radius*.94,h:halfWidth*.462,s:1},{r:radius*.72,h:halfWidth*.56}],tyre,16,Infinity);
          // The inboard face is an annular pressed disc. It needs a true hole,
          // not several hidden concentric surfaces behind the suspension arm.
          revolve(seat,mul(outward,-1),[{r:radius*.72,h:0},{r:radius*.20,h:0}],rim,12,Infinity);
        }
        cylinder(add(center,mul(a,-halfWidth*.51)),add(center,mul(a,halfWidth*.51)),radius*.20,hub,16);
      } else revolve(add(center, mul(a, -halfWidth)), a, tyreProfile, tyre, 20, Infinity);
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
      cursor: () => positions.length / 3,
      triangle, turned, face, part, box, loft, cylinder, tube, socket, rod, revolve, studs, wheelBody,
      finish(description) {
        // Flat tread shoes meet a curved belt at their corners. Seat the actual
        // lowest vertex on the ground, rather than clipping the shoe geometry.
        const ground = bounds.min[1];
        for (let i = 1; i < positions.length; i += 3) positions[i] -= ground;
        bounds.max[1] -= ground; bounds.min[1] = 0;
        return { positions: new Float32Array(positions), normals: new Float32Array(normals), colors: new Float32Array(colors), materialClasses:new Uint8Array(materialClasses),
          bounds, parts, smoothing, lod: D.level, triangleCount: positions.length / 9, description };
      }
    };
  }
  function octagon(y, width, rear, front, bevel, offset = 0) {
    return [[-width + bevel, y, rear + offset], [width - bevel, y, rear + offset], [width, y, rear + bevel + offset],
      [width, y, front - bevel + offset], [width - bevel, y, front + offset], [-width + bevel, y, front + offset],
      [-width, y, front - bevel + offset], [-width, y, rear + bevel + offset]];
  }
  function inspectionBow(b,width,front,low,high,color) {
    // Folded upper/lower armor replaces the inspection-only cylindrical nose
    // bulge. Both ends sit on the existing bow, behind its applique and tow eyes.
    b.loft([octagon(low,width*.93,front-.11,front+.065,.045),
      octagon(high,width*.85,front-.50,front-.355,.045)],color);
    inspectionSeam(b,[-width*.73,high+.004,front-.355],[width*.73,high+.004,front-.355],
      normal([0,.70,.71]),color,true);
  }
  function cheekSection(y,width,rear,front,corner,offset=0){
    // A recessed gun opening and long oblique cheeks give the shell a fighting
    // compartment, instead of one octagonal slab with a rod through its front.
    return [[-width+corner,y,rear+offset],[width-corner,y,rear+offset],
      [width,y,rear+corner+offset],[width,y,front-.73+offset],
      [width*.77,y,front-.26+offset],[width*.29,y,front+offset],
      [-width*.29,y,front+offset],[-width*.77,y,front-.26+offset],
      [-width,y,front-.73+offset],[-width,y,rear+corner+offset]];
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
  // Inspection fittings stay inside the existing semantic part. In particular,
  // a hinge cannot create a new picking ID or leave a coarse part empty.
  // The surface frame is orthonormal: U/V lie on the plate, N points out of it.
  function inspectionHinge(b, center, axis, outward, length, radius, color) {
    if(b.level!==0)return;
    const u=normal(axis),n=normal(outward),v=normal(cross(n,u));
    for(const side of [-1,1])b.box(add(center,mul(v,side*radius*1.5)),[length*.90,radius*.35,radius*2.0],color,[u,n,v]);
    for(let k=0;k<3;k++){
      const start=add(center,mul(u,length*(-.5+k/3)));
      b.cylinder(start,add(start,mul(u,length*.30)),radius,k===1?shade(color,.82):color,10);
    }
    for(const side of [-1,1])b.cylinder(add(center,mul(u,side*length*.49)),add(center,mul(u,side*length*.55)),radius*.55,PALETTE.bright,8);
  }
  function inspectionLatch(b, center, u, n, size, color) {
    if(b.level!==0)return;
    const v=normal(cross(n,u)),at=(x,y,z)=>add(center,add(mul(u,x),add(mul(n,y),mul(v,z))));
    beveled(b,at(0,0,0),[size*.64,size*.16,size*1.25],shade(color,.8),size*.12,[u,n,v]);
    b.box(at(0,size*.13,0),[size*.30,size*.15,size*.72],color,[u,n,v]);
    b.rod(at(-size*.27,size*.14,size*.15),at(size*.27,size*.14,size*.15),size*.09,PALETTE.steel,8);
    b.rod(at(0,size*.16,-size*.16),at(0,size*.16,-size*.58),size*.07,PALETTE.bright,8);
  }
  function inspectionSeam(b, a, end, n, color, welded=false) {
    if(b.level!==0)return;
    const line=sub(end,a),length=Math.hypot(...line),u=normal(line),v=normal(cross(n,u));
    if(length<.05)return;
    // A narrow recessed-color joint sits on the armor; intermittent shallow
    // raised beads cover it only on welded joints. No free-standing wire seam.
    b.box(mean([a,end]),[length,.006,welded?.024:.013],shade(color,.62),[u,n,v]);
    if(welded){
      const count=Math.min(28,Math.ceil(length/.085));
      for(let i=0;i<count;i++)b.box(add(add(a,mul(u,length*(i+.5)/count)),mul(n,.006)),
        [length/count*.78,.009,.019],shade(color,1.08),[u,n,v]);
    }
  }
  function inspectionPanel(b, center, width, depth, u, n, color) {
    if(b.level!==0)return;
    const v=normal(cross(n,u)),at=(x,z)=>add(center,add(mul(u,x),mul(v,z)));
    b.box(add(center,mul(n,.007)),[width,.012,depth],shade(color,.94),[u,n,v]);
    for(const sign of [-1,1]){
      inspectionSeam(b,at(sign*width/2,-depth/2),at(sign*width/2,depth/2),n,color);
      inspectionSeam(b,at(-width/2,sign*depth/2),at(width/2,sign*depth/2),n,color);
    }
    for(const sign of [-1,1])inspectionHinge(b,add(at(sign*width*.29,-depth*.49),mul(n,.028)),u,n,Math.min(.19,width*.24),.022,color);
    inspectionLatch(b,add(at(0,depth*.40),mul(n,.025)),u,n,.12,PALETTE.bright);
  }
  function inspectionGuideHorn(b, center, inward, tangent, width, depth, height) {
    const u=[1,0,0],ring=(h,w,d)=>[-1,1].flatMap((x,i)=>(i?[1,-1]:[-1,1]).map(z=>add(center,add(mul(inward,h),add(mul(u,x*w/2),mul(tangent,z*d/2))))));
    // A tapered cast horn, broad foot seated in the shoe, clear of its pad.
    b.loft([ring(0,width,depth),ring(height*.65,width*.64,depth*.63),ring(height,width*.22,depth*.35)],PALETTE.steel);
  }
  function inspectionTrackConnector(b, center, tangent, inward, trackWidth, pitch) {
    if(b.level!==0)return;
    const pin=add(center,mul(tangent,pitch*.46)),basis=[[1,0,0],inward,tangent];
    for(const side of [-1,1]){
      const edge=add(pin,[side*(trackWidth/2-.018),0,0]);
      b.box(edge,[.074,.061,pitch*.44],PALETTE.steel,basis);
      b.cylinder(add(edge,[side*.027,0,0]),add(edge,[side*.050,0,0]),.027,PALETTE.bright,6);
    }
  }
  // Restrained wear, varied by loop index so the same input still gives the same
  // bytes. shade() scales all three channels together, so a weathered panel keeps
  // the green dominance the sand/winter repaint heuristic keys on.
  const weathered = (color, index) => shade(color, 0.945 + 0.05 * (index % 3) + 0.022 * Math.abs(Math.sin(index * 1.7)));
  function resolveSpec(spec) {
    const input = spec && typeof spec === "object" ? spec : {}, components = {};
    if (Object.hasOwn(AIR_DEFAULTS,input.platform)) {
      const defaults={air_radar:"air_radar_basic",air_avionics:"air_avionics_analog",air_countermeasures:"air_countermeasures_basic",air_payload:"air_payload_unguided",air_fuel:"air_fuel_standard",...AIR_DEFAULTS[input.platform]};
      const restricted=input.platform==='air_light_attack'?['air_engine_twin','air_wing_swept','air_hardpoints_heavy']:[];
      for(const [slot,choices] of Object.entries(AIR_CHOICES))components[slot]=choices.includes(input.components?.[slot])&&!restricted.includes(input.components[slot])?input.components[slot]:defaults[slot];
      return {platform:input.platform,components};
    }
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

  // Parked, original aircraft concepts. All selectable fittings have the same
  // eight semantic slots as the simulation; dimensions are visual art only.
  function buildAircraft(spec,detail) {
    const s=spec.components,b=createBuilder(detail),strike=spec.platform==='air_tactical_strike';
    // Spend inspection geometry on curved airframe/duct sections and wing
    // profiles. Catalogue and map builds sample the same authored surfaces.
    const level=detail.level,radial=[160,32,8][level],steps=[16,3,1][level];
    const wingSections=[40,5,2][level],wingChord=[80,16,4][level];
    const c=strike?{paint:[.48,.52,.54],upper:[.55,.58,.59],edge:[.39,.43,.45],dark:[.20,.24,.26],steel:[.25,.27,.28],black:PALETTE.black,glass:[.40,.53,.56],rubber:PALETTE.rubber}:{paint:[.37,.41,.36],upper:[.45,.49,.42],edge:[.53,.56,.48],dark:[.19,.23,.21],steel:PALETTE.steel,black:PALETTE.black,glass:[.09,.22,.28],rubber:PALETTE.rubber};
    const surfaces=[];
    const length=strike?16.8:11.8,y=strike?2.10:1.72,w=strike?.77:.58,rear=-length*.49,front=length*.51;
    const swept=s.air_wing==='air_wing_swept',stable=s.air_wing==='air_wing_stable',span=(strike?5.85:5.15)+(stable?.75:0),wingZ=strike?.15:-.1;
    const twin=s.air_engine==='air_engine_twin',efficient=s.air_engine==='air_engine_efficient',mapping=s.air_radar==='air_radar_mapping',digital=s.air_avionics==='air_avionics_digital',ecm=s.air_countermeasures==='air_countermeasures_ecm',heavy=s.air_hardpoints==='air_hardpoints_heavy',guided=s.air_payload==='air_payload_guided',extended=s.air_fuel==='air_fuel_extended';
    const ring=(z,rx,ry,cy=y,cx=0,n=radial)=>Array.from({length:n},(_,i)=>[cx+rx*Math.cos(TAU*i/n),cy+ry*Math.sin(TAU*i/n),z]);
    const roundedSections=input=>{
      const points=input.map(p=>[p[0],p[1],p[2],p[3]??y,p[4]??0]);
      const slopes=points.map((p,k)=>p.map((_,axis)=>{
        if(axis===0)return 1;
        const before=points[Math.max(0,k-1)],after=points[Math.min(points.length-1,k+1)];
        const a=k?(p[axis]-before[axis])/(p[0]-before[0]):(after[axis]-p[axis])/(after[0]-p[0]);
        const d=k<points.length-1?(after[axis]-p[axis])/(after[0]-p[0]):a;
        return a*d>0?2*a*d/(a+d):0;
      })),result=[];
      for(let k=0;k<points.length-1;k++)for(let j=0;j<steps;j++){
        const t=j/steps,t2=t*t,t3=t2*t,p=points[k],q=points[k+1],dz=q[0]-p[0],row=[p[0]+dz*t];
        for(let axis=1;axis<5;axis++){
          const value=(2*t3-3*t2+1)*p[axis]+(t3-2*t2+t)*dz*slopes[k][axis]+(-2*t3+3*t2)*q[axis]+(t3-t2)*dz*slopes[k+1][axis];
          row.push(Math.max(Math.min(p[axis],q[axis]),Math.min(Math.max(p[axis],q[axis]),value)));
        }
        result.push(row);
      }
      result.push(points[points.length-1]);return result;
    };
    const body=(input,color,cx=0,options={})=>{
      const rows=options.rounded?roundedSections(input):input;
      const power=options.squared?.65:1;
      const curve=a=>Math.sign(a)*Math.pow(Math.abs(a),power);
      const rings=rows.map(([z,rx,ry,cy,dx])=>options.squared?Array.from({length:radial},(_,i)=>[cx+(dx??0)+rx*curve(Math.cos(TAU*i/radial)),(cy??y)+ry*curve(Math.sin(TAU*i/radial)),z]):ring(z,rx,ry,cy??y,cx+(dx??0)));
      const center=mean(rings.flat());
      if(!options.open){b.face(rings[0],shade(color,.84),center);b.face(rings[rings.length-1],shade(color,1.08),center);}
      // Elliptical fuselage/canopy normals follow the actual ring radii and
      // taper. Shared normals remove the old longitudinal prism stripes without
      // subdividing a single triangle or rounding a wing's sharp trailing edge.
      const normals=rows.map((row,k)=>{
        const before=rows[Math.max(0,k-1)],after=rows[Math.min(rows.length-1,k+1)],dz=after[0]-before[0];
        const dx=(after[1]-before[1])/dz,dy=(after[2]-before[2])/dz,dc=((after[3]??y)-(before[3]??y))/dz,dcx=((after[4]??0)-(before[4]??0))/dz;
        return Array.from({length:radial},(_,i)=>{
          const angle=TAU*i/radial,co=Math.cos(angle),si=Math.sin(angle);
          if(options.squared){const nx=Math.sign(co)*Math.pow(Math.abs(co),2-power)/row[1],ny=Math.sign(si)*Math.pow(Math.abs(si),2-power)/row[2];return mul(normal([nx,ny,-nx*(dcx+dx*curve(co))-ny*(dc+dy*curve(si))]),options.inward?-1:1);}
          return mul(normal([row[2]*co,row[1]*si,-row[1]*si*(dc+dy*si)-row[2]*co*(dcx+dx*co)]),options.inward?-1:1);
        });
      });
      for(let k=0;k<rings.length-1;k++)for(let i=0;i<radial;i++){
        if(options.upper&&i>=radial/2)continue;
        const zmid=(rows[k][0]+rows[k+1][0])/2;
        if(options.cockpit&&zmid>2.20&&zmid<5.55&&Math.sin(TAU*(i+.5)/radial)>.38)continue;
        const j=(i+1)%radial,a=rings[k][i],d=rings[k][j],e=rings[k+1][j],f=rings[k+1][i];
        if(options.inward){
          b.turned(a,e,d,color,normals[k][i],normals[k+1][j],normals[k][j]);
          b.turned(a,f,e,color,normals[k][i],normals[k+1][i],normals[k+1][j]);
        }else{
          b.turned(a,d,e,color,normals[k][i],normals[k][j],normals[k+1][j]);
          b.turned(a,e,f,color,normals[k][i],normals[k+1][j],normals[k+1][i]);
        }
      }
    };
    // Closed tapered plates support both horizontal swept wings and vertical fins.
    const plate=(outline,thickness,color,vertical=false)=> {
      const d=vertical?[thickness/2,0,0]:[0,thickness/2,0];
      b.loft([outline.map(p=>sub(p,d)),outline.map(p=>add(p,d))],color);
    };
    const airfoil=(outline,thickness,color)=>{
      const sections=[];
      for(let j=0;j<=wingSections;j++){
        const spanFraction=j/wingSections,lead=add(outline[0],mul(sub(outline[1],outline[0]),spanFraction));
        const trail=add(outline[3],mul(sub(outline[2],outline[3]),spanFraction)),chord=sub(trail,lead),points=[];
        // A restrained original biconvex section: a rounded leading edge, a
        // deep root and a sharp trailing edge. No real airfoil rating implied.
        for(let i=0;i<=wingChord;i++){
          const f=i/wingChord,h=thickness*(1-spanFraction*.68)*Math.sin(Math.PI*Math.pow(f,.65));
          points.push(add(add(lead,mul(chord,f)),[0,h,0]));
        }
        for(let i=wingChord-1;i>=1;i--){const f=i/wingChord,h=thickness*(1-spanFraction*.68)*Math.sin(Math.PI*Math.pow(f,.65));points.push(add(add(lead,mul(chord,f)),[0,-h*.62,0]));}
        sections.push(points);
      }
      b.loft(sections,color);
    };
    const airframeRows=strike?[[rear,.17,.15],[rear+length*.08,.46,.27],[rear+length*.20,.94,.38],[rear+length*.37,1.04,.44],[rear+length*.55,.94,.49],[rear+length*.72,.63,.47],[front-length*.15,.47,.39],[front-length*.10,w*.43,w*.47]]:null;
    const skinRows=strike?roundedSections(airframeRows):null;
    const skin=(z,angle,offset=.006)=>{
      const k=Math.max(1,skinRows.findIndex(p=>p[0]>=z)),a=skinRows[k-1],q=skinRows[k],t=Math.max(0,Math.min(1,(z-a[0])/(q[0]-a[0])));
      return [(a[1]+(q[1]-a[1])*t+offset)*Math.cos(angle),y+(a[2]+(q[2]-a[2])*t+offset)*Math.sin(angle),z];
    };
    const panel=(a,z,width,depth)=>b.box([a,y+w*.90,z],[width,.015,depth],c.dark);
    b.part('air_wing / airframe and wing roots',()=>{
      body(strike?airframeRows:[[rear,.16,.19],[rear+length*.08,w*.56,w*.55],[rear+length*.20,w*.83,w*.83],[rear+length*.37,w,w*.90],[rear+length*.55,w,w*.95],[rear+length*.72,w*.81,w*.86],[front-length*.15,w*.56,w*.59],[front-length*.10,w*.43,w*.47]],c.paint,0,{rounded:true,cockpit:strike});
      for(const side of [-1,1]) {
        body(strike?[[wingZ-3.65,.12,.055,y,side*.10],[wingZ-2.0,.69,.24,y],[wingZ-.10,.78,.28,y+.035],[wingZ+2.35,.57,.12,y+.12,side*.04],[wingZ+4.25,.045,.035,y+.17,-side*.20]]:[[wingZ-2.05,.055,.035,y],[wingZ-1.30,w*.68,.125,y],
          [wingZ-.10,w*.80,.21,y+.012],[wingZ+.85,w*.53,.14,y+.015],[wingZ+1.86,.055,.035,y]],c.paint,side*w*.90,{rounded:true});
        if(!strike)for(let i=0;i<5;i++)b.box([side*w*.99,y-.02,rear+2.3+i*.18],[.017,.29,.045],c.dark);
        // Service bay doors lie on the nearly cylindrical waist, with their
        // hinges reaching the fuselage instead of detached decorative squares.
        if(!strike)inspectionPanel(b,[side*w*.991,y,wingZ+1.02],.58,.38,[0,0,1],[side,0,0],c.paint);
      }
      if(strike&&level<2){
        // Thin access seams follow the new curved skin, including the dorsal
        // engine covers. Their subdued edges stay attached at every sample.
        const n=level===0?24:8,seam=shade(c.paint,.80);
        for(const z of [-5.35,-2.45,.85,6.08])for(let j=0;j<n;j++){
          const a=.26+(Math.PI-.52)*j/n,q=.26+(Math.PI-.52)*(j+1)/n;
          b.rod(skin(z,a),skin(z,q),.006,seam,6);
        }
        for(const angle of [Math.PI*.30,Math.PI*.70])for(let j=0;j<n;j++)b.rod(skin(-5.35+6.20*j/n,angle),skin(-5.35+6.20*(j+1)/n,angle),.006,seam,6);
      }
    },'air_wing');
    b.part(`air_radar / ${mapping?'terrain-mapping radome and sensor fairing':'basic ranging radome'}`,()=>{
      const offset=mapping?.25:0;
      body([[front-length*.10,w*.43,w*.47],[front-length*.055,w*.27,w*.30],[front+offset,.016,.022]],mapping?c.dark:shade(c.paint,.85));
      b.rod([0,y,front+offset],[0,y,front+offset+.62],.018,c.steel,12);
      if(mapping){body([[front-1.65,.17,.12,y-.43],[front-1.14,.20,.16,y-.40],[front-.85,.04,.05,y-.36]],c.dark);b.cylinder([0,y-.5,front-1.05],[0,y-.52,front-.96],.10,PALETTE.lens,24);}
    },'air_radar');
    b.part(`air_avionics / ${digital?'digital mission cockpit and targeting pod':'analog cockpit and radio aerials'}`,()=>{
      if(strike){
        // The upper fuselage has a genuine opening. An open-bottom glazing shell
        // sits above the coaming and two seats instead of enclosing a solid tube.
        const cy=y+.40,rows=[[2.06,.03,.025,cy],[2.40,.39,.38,cy],[3.15,.49,.61,cy],[4.65,.45,.59,cy],[5.35,.29,.35,cy],[5.66,.03,.025,cy]];
        const upholstery=[.28,.29,.25],webbing=[.49,.46,.35],instrument=[.61,.71,.65];
        // Instrument graphics are physical inset faces on the panel. Their
        // straight strokes stay legible without inventing tiny rendered text.
        const panelFace=(x,yy,z,width,height,color)=>b.face([[x-width/2,yy-height/2,z],[x+width/2,yy-height/2,z],[x+width/2,yy+height/2,z],[x-width/2,yy+height/2,z]],color,[x,yy,z+.1]);
        const panelLine=(a,q,width,color)=>{
          const d=sub(q,a),across=mul(normal([-d[1],d[0],0]),width/2);
          b.face([sub(a,across),sub(q,across),add(q,across),add(a,across)],color,add(mean([a,q]),[0,0,.1]));
        };
        const belt=(a,q,width)=>{
          const along=normal(sub(q,a)),across=normal(cross([0,0,1],along)),out=normal(cross(across,along));
          b.box(mean([a,q]),[width,Math.hypot(...sub(q,a)),.007],webbing,[across,along,out]);
        };
        b.box([0,cy-.27,3.86],[.79,.08,3.18],c.black);
        for(const side of [-1,1]){
          b.box([side*.405,cy-.13,3.84],[.055,.28,2.86],c.dark);
          b.rod([side*.28,cy,2.26],[side*.47,cy,3.15],.027,c.edge,12);
          b.rod([side*.47,cy,3.15],[side*.43,cy,4.65],.027,c.edge,12);
          b.rod([side*.43,cy,4.65],[side*.25,cy,5.40],.027,c.edge,12);
        }
        for(const z of [3.03,4.43]){
          beveled(b,[0,cy-.10,z],[.46,.16,.51],[.17,.19,.17],.04);
          beveled(b,[0,cy+.10,z-.22],[.44,.46,.14],[.12,.14,.13],.04);
          b.box([0,cy+.385,z-.24],[.27,.14,.17],c.black);
          if(level===2)for(const side of [-1,1])b.rod([side*.09,cy+.33,z-.14],[side*.16,cy+.01,z+.06],.021,[.51,.48,.36],10);
          b.box([0,cy+.01,z+.36],[.54,.22,.14],c.dark);
          if(level<2){
            b.always(()=>{
              // Three rounded rectangular sections shape the seat pan; the
              // central cushion rises out of the supporting shell at its edge.
              const cushion=(width,depth,yy)=>[[-width*.40,yy,z-depth/2],[width*.40,yy,z-depth/2],[width/2,yy,z-depth*.38],[width/2,yy,z+depth*.38],[width*.40,yy,z+depth/2],[-width*.40,yy,z+depth/2],[-width/2,yy,z+depth*.38],[-width/2,yy,z-depth*.38]];
              b.loft([cushion(.40,.43,cy-.025),cushion(.405,.425,cy+.004),cushion(.35,.38,cy+.026)],upholstery);
              beveled(b,[0,cy+.115,z-.126],[.33,.33,.047],upholstery,.045);
              beveled(b,[0,cy+.385,z-.149],[.235,.108,.022],[.23,.25,.22],.023);
              for(const side of [-1,1]){
                belt([side*.10,cy+.29,z-.093],[side*.155,cy+.036,z+.11],.045);
                belt([side*.205,cy+.024,z+.085],[side*.025,cy+.038,z+.125],.036);
                b.box([side*.115,cy+.215,z-.040],[.060,.027,.013],c.steel);
                beveled(b,[side*.31,cy-.04,z+.025],[.15,.14,.63],c.dark,.018);
                // Pedals rest below the instrument panel, clear of the seat pan.
                b.box([side*.105,cy-.205,z+.34],[.14,.105,.025],c.steel,[[1,0,0],[0,.9,-.43589],[0,.43589,.9]]);
              }
              beveled(b,[0,cy+.041,z+.129],[.061,.045,.021],c.steel,.009);
              b.cylinder([-.31,cy+.03,z+.02],[-.31,cy+.125,z-.015],.012,c.steel,10);
              b.cylinder([-.35,cy+.125,z-.015],[-.27,cy+.125,z-.015],.025,c.black,12);
              b.cylinder([0,cy+.015,z+.13],[0,cy+.046,z+.13],.042,c.black,12);
              b.rod([0,cy+.035,z+.13],[0,cy+.127,z+.18],.014,c.steel,10);
              beveled(b,[0,cy+.146,z+.185],[.050,.071,.045],c.black,.011);

              if(digital){
                for(const [i,x] of [-.17,0,.17].entries()){
                  b.box([x,cy+.050,z+.278],[.147,.124,.024],c.black);
                  panelFace(x,cy+.050,z+.264,.126,.102,[.045,.105,.09]);
                  if(i===1){
                    panelFace(x,cy+.075,z+.263,.118,.047,[.15,.31,.39]);
                    panelFace(x,cy+.026,z+.263,.118,.047,[.32,.25,.17]);
                    panelLine([x-.044,cy+.049,z+.262],[x+.044,cy+.049,z+.262],.004,instrument);
                    panelLine([x,cy+.041,z+.261],[x,cy+.066,z+.261],.003,instrument);
                  }else if(i===0){
                    const n=level===0?24:8;
                    for(let k=0;k<n;k++){
                      const a=TAU*k/n,q=TAU*(k+1)/n;
                      panelLine([x+Math.cos(a)*.038,cy+.050+Math.sin(a)*.038,z+.263],[x+Math.cos(q)*.038,cy+.050+Math.sin(q)*.038,z+.263],.0025,[.17,.46,.30]);
                    }
                    panelLine([x,cy+.050,z+.262],[x+.023,cy+.078,z+.262],.003,[.47,.72,.43]);
                    panelFace(x-.013,cy+.070,z+.261,.007,.005,instrument);
                  }else for(let k=0;k<3;k++){
                    panelFace(x,cy+.077-k*.027,z+.263,.101,.014,[.08,.17,.13]);
                    panelFace(x-.018+k*.009,cy+.077-k*.027,z+.262,.065-k*.018,.010,[.28,.64,.40]);
                  }
                  if(level===0)for(const dx of [-.045,-.015,.015,.045])b.box([x+dx,cy-.021,z+.266],[.012,.009,.008],[.38,.41,.37]);
                }
              }else{
                const gauges=level===0?[[-.17,.064],[0,.064],[.17,.064],[-.085,-.040],[.085,-.040]]:[[-.17,.045],[0,.045],[.17,.045]];
                for(const [index,[x,yy]] of gauges.entries()){
                  b.cylinder([x,cy+yy,z+.284],[x,cy+yy,z+.271],.048,c.steel,24);
                  const n=level===0?24:8;
                  b.face(Array.from({length:n},(_,k)=>[x+Math.cos(TAU*k/n)*.039,cy+yy+Math.sin(TAU*k/n)*.039,z+.270]),c.black,[x,cy+yy,z+.4]);
                  if(level===0)for(let k=0;k<12;k++){
                    const a=TAU*k/12;panelLine([x+Math.cos(a)*.028,cy+yy+Math.sin(a)*.028,z+.269],[x+Math.cos(a)*.035,cy+yy+Math.sin(a)*.035,z+.269],.002,instrument);
                  }
                  const a=.6+index*.85;
                  panelLine([x,cy+yy,z+.268],[x+Math.cos(a)*.028,cy+yy+Math.sin(a)*.028,z+.268],.0035,instrument);
                  panelFace(x,cy+yy,z+.267,.006,.006,[.59,.32,.18]);
                }
              }
              if(level===0){
                // Console switches, throttle gate, pedal hinges and upholstered
                // seams reward a close view without cluttering catalogue LODs.
                for(const side of [-1,1]){
                  for(let k=0;k<4;k++){
                    const zz=z-.22+k*.14;
                    b.box([side*.31,cy+.034,zz],[.097,.008,.073],[.28,.30,.28]);
                    b.cylinder([side*.31,cy+.041,zz],[side*.31,cy+.061,zz-.006],.009,c.steel,10);
                    b.cylinder([side*.35,cy+.039,zz],[side*.35,cy+.046,zz],.010,k===3?[.65,.33,.15]:[.24,.43,.30],10);
                  }
                  b.rod([side*.105,cy-.252,z+.245],[side*.105,cy-.205,z+.34],.013,c.steel,12);
                  for(const dz of [-.032,0,.032])b.box([side*.105,cy-.164,z+.321+dz],[.12,.004,.009],c.dark);
                  b.rod([side*.17,cy+.032,z-.11],[side*.17,cy+.032,z+.11],.004,[.39,.39,.32],6);
                  b.cylinder([side*.18,cy+.265,z-.124],[side*.18,cy+.265,z-.107],.014,c.steel,12);
                }
                b.box([-.31,cy+.035,z+.12],[.042,.009,.16],c.black);
                b.box([0,cy+.157,z+.157],[.022,.021,.009],[.48,.15,.10]);
                for(const yy of [.054,.069,.084])b.tube([0,cy+yy,z+.152],[0,cy+yy+.006,z+.152],.022,.015,c.black,12);
              }
            });
          }
        }
        // Canopy frames follow sampled cross-sections. Only the clear shell is
        // translucent, so brackets, seats and instruments keep physical depth.
        for(const [z,rx,ry] of [[2.40,.39,.38],[3.70,.48,.61],[5.35,.29,.35]]){
          const n=[40,16,6][level];for(let k=0;k<n;k++){
            const a=Math.PI*k/n,q=Math.PI*(k+1)/n;
            b.rod([rx*Math.cos(a),cy+ry*Math.sin(a),z],[rx*Math.cos(q),cy+ry*Math.sin(q),z],.018,c.edge,10);
          }
        }
        if(level===0)for(const side of [-1,1]){
          for(const [a,q] of [[[side*.28,cy-.009,2.26],[side*.47,cy-.009,3.15]],[[side*.47,cy-.009,3.15],[side*.43,cy-.009,4.65]],[[side*.43,cy-.009,4.65],[side*.25,cy-.009,5.40]]])b.rod(a,q,.013,c.black,12);
          for(const [z,x] of [[2.67,.366],[3.31,.465],[4.18,.443],[4.94,.363]]){
            inspectionHinge(b,[side*x,cy-.010,z],[0,0,1],[side,0,0],.082,.010,c.steel);
            b.cylinder([side*x,cy-.004,z+.077],[side*x,cy+.013,z+.077],.012,c.steel,12);
          }
        }
        const first=b.cursor();body(rows,c.glass,0,{rounded:true,upper:true,open:true});
        surfaces.push({first,count:b.cursor()-first,material:'glass',opacity:.24});
      }else{
      const z=front-length*.29,canopyLength=strike?2.95:2.35;
      const rows=[[z-canopyLength*.52,w*.28,.12,y+w*.8],[z-canopyLength*.3,w*.53,.41,y+w*.91],[z+canopyLength*.19,w*.49,.49,y+w*.91],[z+canopyLength*.46,w*.22,.20,y+w*.87]];
      body(rows,c.glass,0,{rounded:true});
      for(const [rz,rx,ry,cy] of [rows[1],rows[2]])for(let i=0;i<16;i++){const t=TAU*i/32,t2=TAU*(i+1)/32;b.rod([rx*Math.cos(t),cy+ry*Math.sin(t),rz],[rx*Math.cos(t2),cy+ry*Math.sin(t2),rz],.027,c.edge,8);}
      for(const side of [-1,1])b.rod([side*w*.49,y+w*.89,z-canopyLength*.3],[side*w*.22,y+w*.87,z+canopyLength*.46],.025,c.edge);
      for(const side of [-1,1]){
        b.rod([side*w*.28,rows[0][3],rows[0][0]],[side*w*.53,rows[1][3],rows[1][0]],.024,c.black,10);
        b.rod([side*w*.53,rows[1][3],rows[1][0]],[side*w*.49,rows[2][3],rows[2][0]],.024,c.black,10);
        inspectionHinge(b,[side*w*.51,y+w*.91,z-.23],[0,0,1],[side,0,0],.27,.027,c.edge);
        inspectionLatch(b,[side*w*.33,y+w*.91,z+canopyLength*.32],[0,0,1],[side,0,0],.075,c.edge);
      }
      }
      const z=front-length*.29;
      if(strike)plate([[0,y+.46,-.32],[0,y+.73,-.64],[0,y+.46,-.88]],.025,c.dark,true);
      else plate([[0,y+w*.8,rear+length*.46],[0,y+w*.8+.47,rear+length*.43],[0,y+w*.8,rear+length*.40]],.035,c.dark,true);
      if(digital){body([[.35,.21,.2,y-.73],[1.55,.21,.20,y-.73],[1.87,.11,.14,y-.73]],c.dark,.58);b.cylinder([.58,y-.73,1.86],[.58,y-.73,1.89],.105,PALETTE.lens,28);b.box([0,strike?skin(1.55,Math.PI/2)[1]+.025:y+w*.96,strike?1.55:z-1.3],[.25,.13,.5],c.edge);}
      else for(const side of [-1,1])b.rod([side*.22,strike?y+.47:y+w*.85,strike?1.30:z-1.1],[side*.28,strike?y+.70:y+w*.85+.35,strike?.95:z-1.55],.012,c.steel);
    },'air_avionics');
    for(const side of [-1,1]) {
      const lead=wingZ+(swept?(strike?-1.85:-1.20):stable?.1:.55),trail=lead-(swept?(strike?1.40:1.05):1.45);
      b.part(`air_wing / ${side<0?'port':'starboard'} ${swept?'swept':stable?'high-stability':'straight'} wing`,()=>{
        const root=side*w*.76,tip=side*span,dihedral=stable?.32:.12;
        airfoil([[root,y,wingZ+1.7],[tip,y+dihedral,lead],[tip,y+dihedral,trail],[root,y,wingZ-1.95]],strike?.12:.145,c.paint);
        // Separate flaps, leading-edge strips, panel joins and navigation lenses.
        if(strike){
          // Separate inboard flap and outboard aileron live aft of the skin,
          // with a genuine hinge gap and a short break between the controls.
          for(const [a,q] of [[.08,.53],[.55,.95]]){
            const point=f=>[root+(tip-root)*f,y+dihedral*f,wingZ-1.95+(trail-wingZ+1.95)*f];
            const u=point(a),v=point(q),depth=.22;
            plate([add(u,[0,0,-.025]),add(v,[0,0,-.025]),add(v,[0,-.015,-depth]),add(u,[0,-.015,-depth])],.035,c.upper);
          }
        }else{
          const outline=[[side*(w+.25),y-.04,wingZ-1.82],[side*(span-.23),y+dihedral-.04,trail+.10],[side*(span-.23),y+dihedral-.04,trail-.10],[side*(w+.25),y-.04,wingZ-2.02]];
          plate(outline,.085,c.upper);
        }
        b.rod([root,y+.086,wingZ+1.65],[tip,y+dihedral+.086,lead-.02],.017,c.edge,8);
        for(let k=1;k<=3;k++){const f=k/4,xx=side*(w+(span-w)*f),zz=wingZ+1.7+(lead-wingZ-1.7)*f;b.rod([xx,y+dihedral*f+.083,zz-.18],[xx,y+dihedral*f+.083,zz-1.02],.011,c.dark,8);}
        for(const f of [.27,.55,.79]){
          const x=side*(w+(span-w)*f),trailing=wingZ-1.95+(trail-wingZ+1.95)*f;
          // Actuator fairing below each flap and a hinge at the actual join.
          beveled(b,[x,y+dihedral*f-.085,trailing+.20],[.12,.12,.48],c.upper,.035);
          inspectionHinge(b,[x,y+dihedral*f-.025,trailing+.035],[side,0,0],[0,-1,0],.23,.025,c.steel);
        }
        if(stable)plate([[tip,y+.2,trail+.2],[tip,y+.8,trail+.35],[tip,y+.8,lead-.1],[tip,y+.2,lead]],.055,c.upper,true);
        b.cylinder([tip,y+dihedral,lead-.08],[tip+side*.07,y+dihedral,lead-.08],.064,side<0?[.60,.07,.06]:[.06,.47,.22],14);
      },'air_wing');
      b.part(`air_wing / ${side<0?'port':'starboard'} tailplane`,()=>{
        const tz=rear+1.3;
        plate([[side*(strike?.70:.17),y+.12,tz+1.5],[side*(strike?3.05:2.23),y+.23,tz-.02],[side*(strike?2.75:2.2),y+.23,tz-(strike?.85:.71)],[side*(strike?.70:.17),y+.12,tz-(strike?.45:.37)]],strike?.065:.10,c.upper);
        b.rod([side*.30,y+.18,tz-.28],[side*(strike?2.7:2.1),y+.29,tz-.63],.015,c.dark,8);
      },'air_wing');
    }
    b.part('air_wing / vertical stabilizers and rudders',()=>{
      for(const side of strike?[-1,1]:[0]){
        const x=side*(strike?.96:.62),tz=rear+1.3,h=strike?2.0:1.65;
        plate([[x,y+.16,tz+1.75],[x+side*.4,y+h,tz+.55],[x+side*.5,y+h,tz-.22],[x,y+.16,tz-.57]],.12,c.paint,true);
        b.rod([x+side*.44,y+h-.11,tz-.10],[x+side*.02,y+.3,tz-.44],.018,c.dark,8);
      }
    },'air_wing');
    const engineXs=twin?(strike?[-.84,.84]:[-.73,.73]):[0],engineRadius=twin?.54:efficient?.52:.43;
    engineXs.forEach((x,index)=>{
      b.part(`air_engine / ${twin?(index?'starboard':'port')+' twin turbofan':efficient?'efficient turbofan':'economical turbine'} nacelle`,()=>{
        const ey=y-(strike?.37:.15),housingFront=wingZ+.03,exhaust=rear+.03;
        // The engine casing ends behind the duct throat. Extending a capped
        // casing to the lip would put a solid painted disc across the intake.
        body([[exhaust,engineRadius*.87,engineRadius*.87,ey],[rear+1.1,engineRadius,engineRadius,ey],[rear+2.8,engineRadius*1.13,engineRadius*1.04,ey],
          [housingFront-.60,engineRadius*.95,engineRadius,ey],[housingFront,engineRadius*.89,engineRadius*.89,ey]],c.paint,x,{open:strike&&level<2});
        // Closed swept vanes have actual front/back surfaces. Their curved
        // radial sections catch highlights through the inlet and nozzle.
        const fan=(cx,cz,r,direction,count)=>{
          const sections=level===0?7:2;
          for(let blade=0;blade<count;blade++){
            const phase=TAU*blade/count,rings=[];
            for(let j=0;j<=sections;j++){
              const t=j/sections,angle=phase+.25*t*t,reach=r*(.17+.55*t),half=.032+.075*t;
              const depth=cz+direction*(.017+.043*Math.sin(Math.PI*t)),thick=.006*(1-.42*t);
              const p=(a,z)=>[cx+Math.cos(a)*reach,ey+Math.sin(a)*reach,z];
              rings.push([p(angle-half,depth-thick),p(angle+half,depth-thick),p(angle+half,depth+thick),p(angle-half,depth+thick)]);
            }
            // Keep the thin fan vanes visible in catalogue detail as well.
            b.always(()=>b.loft(rings,blade%3?[.39,.42,.43]:[.45,.47,.47]));
          }
        };
        // Exposed side ducts feed the selected engine installation. A single
        // central circular mouth was hidden inside the fuselage; two exposed
        // intakes still belong to ONE engine part when a single engine is fitted.
        for(const side of twin?[Math.sign(x)]:[-1,1]){
          const r=engineRadius*(strike?(twin?.70:.75):(twin?.66:.65)),mouthX=side*(strike?1.10+r*.65:w+r+.045),mouthZ=wingZ+(strike?1.45:.95);
          body([[mouthZ-1.72,r*.51,r*.72,ey,x-mouthX],[mouthZ-.78,r*.89,r*.96,ey,-side*.115],
            [mouthZ-.14,r*1.02,r*1.02,ey,0],[mouthZ,r*1.03,r*1.03,ey,0]],c.paint,mouthX,{rounded:true,open:true,squared:strike});
          if(strike){
            const outside=Array.from({length:radial},(_,k)=>{const a=TAU*k/radial;return [mouthX+Math.sign(Math.cos(a))*Math.pow(Math.abs(Math.cos(a)),.65)*r*1.03,ey+Math.sign(Math.sin(a))*Math.pow(Math.abs(Math.sin(a)),.65)*r*1.03,mouthZ];});
            const inside=outside.map(p=>[mouthX+(p[0]-mouthX)*.84,ey+(p[1]-ey)*.84,mouthZ+.012]);
            for(let k=0;k<radial;k++){const q=(k+1)%radial;b.face([outside[k],outside[q],inside[q],inside[k]],c.edge,[mouthX,ey,mouthZ-.15]);}
          }else b.revolve([mouthX,ey,mouthZ-.08],[0,0,1],[{r:r*1.03,h:0,s:1},{r:r*1.075,h:.055,s:1},
            {r:r*1.04,h:.10,s:1},{r:r*.88,h:.105},{r:r*.84,h:.06,s:1},{r:r*.84,h:0,s:1}],c.edge,32);
          body([[mouthZ-.66,r*.72,r*.72,ey,-side*.030],[mouthZ-.31,r*.79,r*.80,ey,-side*.008],
            [mouthZ-.08,r*.84,r*.84,ey,0]],c.dark,mouthX,{rounded:true,open:true,inward:true,squared:strike});
          if(strike)body([[mouthZ-.08,r*.84,r*.84,ey],[mouthZ+.012,r*.8652,r*.8652,ey]],c.dark,mouthX,{open:true,inward:true,squared:true});
          const hubX=mouthX-side*.030,fanZ=mouthZ-.675;
          if(strike)b.face(Array.from({length:radial},(_,k)=>{const a=TAU*k/radial;return [hubX+Math.sign(Math.cos(a))*Math.pow(Math.abs(Math.cos(a)),.65)*r*.735,ey+Math.sign(Math.sin(a))*Math.pow(Math.abs(Math.sin(a)),.65)*r*.735,fanZ-.016];}),c.black,[hubX,ey,fanZ-.10]);
          b.cylinder([hubX,ey,fanZ-.015],[hubX,ey,fanZ],r*.73,c.black,28);
          if(strike&&level<2){
            fan(hubX,fanZ,r,1,level===0?28:20);
            b.revolve([hubX,ey,fanZ+.013],[0,0,1],[{r:r*.18,h:0,s:1},{r:r*.18,h:.026,s:1},{r:r*.14,h:.079,s:1},{r:r*.065,h:.119,s:1},{r:0,h:.145}],c.edge,level===0?64:24);
            // Fan containment collar, seated behind the duct exit. Small
            // peripheral fasteners remain outside the clear throat samples.
            b.revolve([hubX,ey,fanZ],[0,0,1],[{r:r*.725,h:-.015},{r:r*.77,h:-.015},{r:r*.77,h:.018},{r:r*.725,h:.018}],c.steel,level===0?96:28);
            if(level===0)for(let k=0;k<12;k++){
              const a=TAU*k/12;
              b.cylinder([hubX+Math.cos(a)*r*.748,ey+Math.sin(a)*r*.748,fanZ+.018],[hubX+Math.cos(a)*r*.748,ey+Math.sin(a)*r*.748,fanZ+.025],.006,c.edge,8);
            }
          }else{
          for(let k=0;k<16;k++){
            const t=TAU*k/16;
            b.face([[hubX+Math.cos(t)*r*.12,ey+Math.sin(t)*r*.12,fanZ+.012],
              [hubX+Math.cos(t+.13)*r*.68,ey+Math.sin(t+.13)*r*.68,fanZ+.026],
              [hubX+Math.cos(t+.30)*r*.67,ey+Math.sin(t+.30)*r*.67,fanZ+.014]],c.steel,[hubX,ey,fanZ-.1]);
          }
          b.cylinder([hubX,ey,fanZ],[hubX,ey,fanZ+.12],r*.15,c.edge,16,r*.04);
          }
        }
        if(strike&&level<2){
          const R=engineRadius,segments=level===0?96:32,metal=[.30,.29,.27],liner=[.16,.18,.19];
          // The outer collar converges to a thin outlet rim, then returns
          // inward along a deep liner. No flat casing cap crosses the cavity.
          b.revolve([x,ey,exhaust],[0,0,-1],[
            {r:R*.96,h:-.28,s:1},{r:R*.965,h:-.19,s:1},
            {r:R*.89,h:.02,s:1},{r:R*.79,h:.34},
            {r:R*.65,h:.34,c:liner},{r:R*.71,h:.12,s:1,c:liner},
            {r:R*.81,h:-.40,s:1,c:liner},{r:R*.72,h:-.90,c:liner},{r:0,h:-.90}
          ],metal,segments);
          const petals=level===0?20:12;
          for(let k=0;k<petals;k++){
            const a=TAU*k/petals,half=TAU/petals*.44,rings=[];
            for(const [z,r] of [[exhaust+.20,R*.967],[exhaust-.035,R*.89],[exhaust-.34,R*.795]]){
              const p=(angle,rr)=>[x+Math.cos(angle)*rr,ey+Math.sin(angle)*rr,z];
              rings.push([p(a-half,r-.005),p(a+half,r-.005),p(a+half,r+.010),p(a-half,r+.010)]);
            }
            b.loft(rings,k%3?[.36,.35,.32]:[.30,.31,.30]);
            if(level===0){
              const radial=[Math.cos(a),Math.sin(a),0],tangent=[-Math.sin(a),Math.cos(a),0];
              const p=add([x,ey,exhaust+.32],mul(radial,R*.98)),q=add([x,ey,exhaust-.035],mul(radial,R*.925));
              b.cylinder(p,q,.017,c.steel,12);
              const knuckle=add(q,mul(radial,.006));
              b.cylinder(add(knuckle,mul(tangent,-.028)),add(knuckle,mul(tangent,.028)),.027,c.edge,10);
              b.box(add(p,mul(radial,-.016)),[.045,.045,.09],c.steel,[tangent,radial,[0,0,1]]);
            }
          }
          // Recessed liner ribs, a turbine face and central exhaust cone are
          // visible through the mouth, with no fire effect on a parked model.
          const coreZ=exhaust+.855;
          fan(x,coreZ,R,-1,level===0?22:14);
          b.revolve([x,ey,coreZ],[0,0,-1],[{r:R*.18,h:0,s:1},{r:R*.19,h:.05,s:1},{r:R*.15,h:.14,s:1},{r:R*.07,h:.235,s:1},{r:0,h:.27}],c.steel,level===0?48:20);
          for(let j=0;j<(level===0?7:3);j++){
            const z=exhaust+.20+j*(level===0?.085:.235),radius=R*(z<exhaust+.40?.785:.81-(z-exhaust-.40)*.18);
            b.revolve([x,ey,z],[0,0,1],[{r:radius,h:-.012},{r:radius-.012,h:-.004},{r:radius-.012,h:.004},{r:radius,h:.012}],j%2?[.23,.24,.23]:[.19,.21,.22],segments);
          }
          if(level===0)for(const side of [-1,1]){
            // Service seams and access latches remain seated on each nacelle.
            const yy=ey+R*.67,xx=x+side*R*.69;
            for(const dz of [.66,1.62,2.70])b.rod([xx,yy,exhaust+dz-.26],[xx,yy,exhaust+dz+.26],.007,c.edge,8);
          }
        }else{
        b.tube([x,ey,exhaust+.28],[x,ey,exhaust-.34],engineRadius*.86,engineRadius*.68,c.steel,40);
        b.cylinder([x,ey,exhaust+.29],[x,ey,exhaust+.27],engineRadius*.67,c.black,36);
        for(let k=0;k<24;k++){const a=TAU*k/24;b.rod([x+Math.cos(a)*engineRadius*.86,ey+Math.sin(a)*engineRadius*.86,exhaust+.12],[x+Math.cos(a)*engineRadius*.86,ey+Math.sin(a)*engineRadius*.86,exhaust-.34],.013,c.edge,8);}
        // Overlapping tapered nozzle petals seat on the existing exhaust lip.
        // Each is a plate with real thickness, not a bright wire floating aft.
        for(let k=0;k<16;k++){
          const a=TAU*k/16,radial=[Math.cos(a),Math.sin(a),0],tangent=[-Math.sin(a),Math.cos(a),0];
          beveled(b,add([x,ey,exhaust-.10],mul(radial,engineRadius*.875)),[engineRadius*.24,.017,.43],shade(c.steel,1+(k%3)*.05),.008,[tangent,radial,[0,0,1]]);
        }
        if(level===0){
          // Recessed heat-shield corrugations sit inside the nozzle. The
          // actuator links attach each petal to its collar, leaving the throat open.
          for(let j=0;j<5;j++){
            const z=exhaust+.015+j*.047;
            b.tube([x,ey,z],[x,ey,z+.018],engineRadius*.695,engineRadius*.667,c.dark,96);
          }
          for(let k=0;k<16;k++){
            const a=TAU*k/16,radial=[Math.cos(a),Math.sin(a),0];
            const p=add([x,ey,exhaust+.21],mul(radial,engineRadius*.90));
            const q=add([x,ey,exhaust-.09],mul(radial,engineRadius*.895));
            b.cylinder(p,q,.021,c.steel,16);
            b.cylinder(add(q,mul(radial,-.015)),add(q,mul(radial,.026)),.038,c.edge,16);
          }
        }
        }
        if(efficient)for(const side of [-1,1])b.box([x+side*engineRadius,ey, rear+2.3],[.08,.22,.8],c.upper);
      },'air_engine');
    });
    // A tricycle undercarriage keeps aircraft seated naturally in the existing
    // turntable. Tire treads, hydraulic braces and wheel wells are real meshes.
    for(const [label,x,z,r] of [['nose',0,front-length*.22,.24],['port',-(strike?1.15:.95),-.6,.34],['starboard',strike?1.15:.95,-.6,.34]])b.part(`air_wing / ${label} landing gear`,()=>{
      const gy=r+.025,top=y-.28;
      b.box([x,top,z],[.45,.12,.78],c.dark);b.box([x+(x<0?-.24:.24),top-.15,z],[.035,.36,.79],c.upper);
      b.cylinder([x,gy,z],[x,top,z-.12],.059,c.steel,20);b.cylinder([x,gy+.20,z-.01],[x,top-.17,z-.10],.031,c.edge,16);
      b.rod([x,gy+.28,z],[x,top,z-.49],.035,c.steel,12);
      const scissor=[x+.13,gy+.25,z+.095];
      b.rod([x+.047,gy+.12,z-.012],scissor,.024,c.steel,12);
      b.rod(scissor,[x+.047,gy+.43,z-.045],.024,c.steel,12);
      b.cylinder(add(scissor,[-.035,0,0]),add(scissor,[.035,0,0]),.038,c.edge,14);
      b.rod([x-.072,top-.04,z-.105],[x-.080,gy+.43,z-.035],.012,c.black,10);
      b.rod([x-.080,gy+.43,z-.035],[x-.13,gy+.10,z+.12],.012,c.black,10);
      beveled(b,[x+.135,gy+r*.25,z+r*.26],[.09,r*.36,r*.24],c.steel,.017);
      inspectionHinge(b,[x+(x<0?-.24:.24),top-.025,z],[0,0,1],[1,0,0],.34,.025,c.steel);
      b.cylinder([x-.11,gy,z],[x+.11,gy,z],r,c.rubber,40);
      for(const side of [-1,1]){
        b.cylinder([x+side*.111,gy,z],[x+side*.128,gy,z],r*.50,c.edge,28);
        b.cylinder([x+side*.13,gy,z],[x+side*.14,gy,z],r*.22,c.dark,20);
        for(let k=0;k<8;k++){const a=TAU*k/8;b.cylinder([x+side*.131,gy+Math.cos(a)*r*.35,z+Math.sin(a)*r*.35],[x+side*.143,gy+Math.cos(a)*r*.35,z+Math.sin(a)*r*.35],.017,c.steel,8);}
      }
      for(let k=0;k<32;k++){const a=TAU*k/32; b.box([x,gy+Math.cos(a)*r,z+Math.sin(a)*r],[.19,.014,.021],c.dark,[[1,0,0],[0,Math.cos(a),Math.sin(a)],[0,-Math.sin(a),Math.cos(a)]]);}
    },'air_wing');
    const stations=[];
    for(const side of [-1,1])for(let i=0;i<(heavy?2:1);i++)stations.push({x:side*(1.65+i*.92),z:wingZ+(swept?-.35-i*.25:.15),i,side});
    b.part(`air_hardpoints / ${heavy?'four-store strike':'two-store attack'} external mounts`,()=>{
      for(const {x,z} of stations){plate([[x,y-.04,z+.45],[x,y-.46,z+.22],[x,y-.46,z-.50],[x,y-.04,z-.66]],.115,c.upper,true);b.box([x,y-.45,z-.08],[.24,.11,.66],c.steel);for(const dz of [-.3,.22])b.rod([x-.12,y-.51,z+dz],[x+.12,y-.51,z+dz],.026,c.edge,10);}
    },'air_hardpoints');
    b.part(`air_payload / ${guided?'precision-guided external bombs':'unguided external bombs'}`,()=>{
      for(const {x,z} of stations){
        const sy=y-.79,sz=z+.16,r=guided?.15:.18;
        body([[sz-.92,.055,.055,sy],[sz-.67,r,r,sy],[sz+.35,r,r,sy],[sz+.70,r*.45,r*.45,sy],[sz+.78,.018,.018,sy]],guided?c.dark:c.paint,x);
        for(const axis of [0,1])for(const sign of [-1,1]){
          const off=axis?[0,sign*.37,0]:[sign*.37,0,0],base=[x,sy,sz-.58];
          plate([add(base,[0,0,-.19]),add(add(base,off),[0,0,-.27]),add(add(base,off),[0,0,.10]),add(base,[0,0,.27])],.026,c.edge,axis===1);
          if(guided){const f=[x,sy,sz+.33],small=mul(off,.65);plate([add(f,[0,0,-.13]),add(add(f,small),[0,0,-.20]),add(add(f,small),[0,0,.06]),add(f,[0,0,.13])],.022,c.upper,axis===1);}
        }
        if(guided)b.cylinder([x,sy,sz+.77],[x,sy,sz+.80],.03,PALETTE.lens,16);
      }
    },'air_payload');
    b.part(`air_fuel / ${extended?'extended fuel tanks and refueling fitting':'internal fuel access panels'}`,()=>{
      for(const side of [-1,1]){
        if(strike){const p=skin(-.55,Math.PI/2-side*.26);b.cylinder(add(p,[0,-.025,0]),add(p,[0,.012,0]),.075,c.edge,20);}
        else {panel(side*.25,-.55,.23,.32);b.cylinder([side*.25,y+w*.91,-.55],[side*.25,y+w*.925,-.55],.075,c.edge,20);}
      }
      if(extended)for(const side of [-1,1]){
        const x=side*(strike?3.75:span-.55),z=wingZ-(strike?1.3:.10),fy=y+(strike?-.80:.10);
        if(strike)plate([[x,y-.01,z+.38],[x,fy+.29,z+.22],[x,fy+.29,z-.57],[x,y-.01,z-.68]],.10,c.upper,true);
        body([[z-1.75,.035,.035,fy],[z-1.21,.25,.25,fy],[z+.63,.27,.27,fy],[z+1.38,.08,.08,fy],[z+1.47,.025,.025,fy]],c.upper,x);
        for(const dz of [-.8,.3])b.tube([x,fy,z+dz-.018],[x,fy,z+dz+.018],.273,.261,c.edge,32);
      }
    },'air_fuel');
    b.part(`air_countermeasures / ${ecm?'electronic countermeasure fairings and dispensers':'flare and chaff dispensers'}`,()=>{
      for(const side of [-1,1]){
        const dx=side*(twin?1.30:.70),dy=y+.16;
        b.box([side*(twin?1.10:.50),y+.10,rear+2.0],[.46,.12,.55],c.paint);
        b.box([dx,dy,rear+2.0],[.32,.24,.55],c.steel);
        for(let i=0;i<3;i++)for(let j=0;j<4;j++)b.box([dx+(i-1)*.075,dy+.126,rear+1.84+j*.10],[.045,.018,.065],c.black);
        if(ecm){body([[rear+1.0,.075,.075,y+.47],[rear+1.26,.15,.15,y+.47],[rear+2.0,.15,.15,y+.47],[rear+2.30,.035,.035,y+.47]],c.dark,side*(w+.20));b.cylinder([side*(w+.20),y+.47,rear+2.28],[side*(w+.20),y+.47,rear+2.32],.035,PALETTE.lens,16);}
      }
    },'air_countermeasures');
    const mesh=b.finish(`Visual interpretation of an original ${strike?'twin-seat tactical strike':'light attack'} aircraft. Component-driven parked game model; performance and procurement values belong to the simulation.`);
    mesh.specification={platform:spec.platform,components:{...s}};
    if(strike){mesh.assetKind='aircraft';mesh.surfaces=surfaces;}
    return mesh;
  }

  // Specialist chassis share fittings and construction primitives, but their hulls,
  // running gear, weapon mounts and mission installations are separate geometry.
  function buildGround(spec, detail) {
    const s=spec.components,c=PALETTE;
    const scout=spec.platform==='ground_recon',carrier=spec.platform==='ground_apc',ifv=spec.platform==='ground_ifv',artillery=spec.platform==='ground_artillery',aa=spec.platform==='ground_air_defense';
    const wheeled=carrier||scout,w=scout?1.03:carrier?1.16:artillery?1.43:1.28,length=scout?4.95:carrier?6.40:artillery?7.15:ifv?6.55:6.45;
    const b=createBuilder(detail,length);
    const rear=-length/2,front=length/2,deck=scout?1.58:carrier?1.94:ifv?1.78:artillery?1.48:1.46;
    const roofFront=front-(scout?1.10:carrier?1.36:ifv?1.34:1.20),tireRadius=scout?.58:.63;
    const modular=s.protection==='ground_armor_modular',hydro=s.suspension==='suspension_hydro';
    const mountZ=artillery?-.76:aa?-.20:scout?.18:.22;
    const mg=s.turret==='ground_station_mg',howitzer=s.turret==='ground_turret_howitzer',aaMount=s.turret==='ground_turret_aa';
    const mountTop=deck+(mg?.37:howitzer?1.26:aaMount?.72:.56),gunY=mountTop-(howitzer?.47:mg?.08:.18);
    const mountWidth=howitzer?1.19:aaMount?.91:mg?.42:.80;
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
      inspectionHinge(b,[x,y+.082,z-r*.83],[1,0,0],[0,1,0],r*.76,.032,c.steel);
      inspectionLatch(b,[x,y+.13,z+r*.57],[1,0,0],[0,1,0],.10,c.bright);
    }
    function wheel(side,z,r,x,width,label) {
      b.part(`running gear / ${label}`,()=>{
        const centerY=r+(wheeled?0:.075);
        b.wheelBody([side*x,centerY,z],[b.level===0?side:1,0,0],r,width/2,c.rubber,c.upper,c.shade,false,b.level===0&&!wheeled);
        // A smooth cylinder is the fastest way to make a wheeled hull look
        // unfinished. Two shoulder rows, a centre row staggered off their pitch,
        // and a bead ring on each sidewall.
        if(wheeled&&b.keep(0.30)) {
          const shoulders=b.many(32,8),centres=b.many(16,4);
          for(let j=0;j<shoulders;j++) {
            const a=TAU*j/shoulders,radial=[0,Math.cos(a),Math.sin(a)],tangent=[0,-Math.sin(a),Math.cos(a)];
            for(const strip of [-1,1])b.box([side*x+strip*width*.23,r+Math.cos(a)*(r+.006),z+Math.sin(a)*(r+.006)],[width*.43,b.gauge(.035),.075],shade(c.rubber,1.28),[[1,0,0],radial,tangent]);
          }
          for(let j=0;j<centres;j++) {
            const a=TAU*(j+.5)/centres,radial=[0,Math.cos(a),Math.sin(a)],tangent=[0,-Math.sin(a),Math.cos(a)];
            b.box([side*x,r+Math.cos(a)*(r+.004),z+Math.sin(a)*(r+.004)],[width*.30,b.gauge(.031),.11],c.rubber,[[1,0,0],radial,tangent]);
          }
          // The bead is an annulus, never a filled cap hiding the painted rim
          // and wheel nuts. Its open inner edge follows the tire's bead seat.
          for(const bead of [-1,1])b.revolve([side*x+bead*width*.485,r,z],[bead,0,0],[
            {r:r*.735,h:0},{r:r*.885,h:-.036,s:1},{r:r*.948,h:-.060}],c.rubber,24);
        }
      },wheeled?'wheels':'tracks');
    }
    b.part('protection / specialist sloped hull',()=>{
      // Longitudinal sections give the carrier a continuous troop roof, the
      // scout a short, strongly raked bow and the tracked mission hulls low
      // sponsons. The roof and glacis meet; no roof slab or cross-hull drum is
      // laid over a generic octagon. These primary forms survive every LOD.
      const shoulder=wheeled?1.18:1.03;
      const section=(z,top,topWidth,sideWidth,chine=shoulder,bottom=.44)=>[
        [-topWidth,top,z],[topWidth,top,z],[sideWidth,chine,z],[sideWidth*.96,.79,z],
        [sideWidth*.72,bottom,z],[-sideWidth*.72,bottom,z],[-sideWidth*.96,.79,z],[-sideWidth,chine,z]];
      const aftTop=w-(scout?.29:carrier?.12:.18),roofWidth=w-(scout?.28:carrier?.16:.20);
      b.loft([section(rear+.08,deck-.045,aftTop,w-.08),section(rear+.32,deck+.045,roofWidth,w),
        section(roofFront,deck+.045,roofWidth,w),section(front-.30,1.07,w*.84,w*.94,.98,.53),
        section(front,1.005,w*.79,w*.87,.94,.58)],c.hull);
      // A thin roof skin shares the same rake endpoints, with a visible plate
      // seam instead of a freestanding second box.
      b.loft([octagon(deck+.045,roofWidth,rear+.32,roofFront,.10),
        octagon(deck+.065,roofWidth-.012,rear+.33,roofFront-.012,.10)],c.upper);
      b.loft([octagon(1.075,w*.85,front-.34,front-.20,.035),
        octagon(deck+.04,roofWidth,roofFront-.045,roofFront+.035,.035)],c.armor);
      // A shallow chin break carries tow loads visually and catches the lower
      // highlight; it follows the bow instead of protruding like a bumper.
      b.loft([octagon(.91,w*.86,front-.105,front+.025,.035),
        octagon(1.04,w*.83,front-.225,front-.10,.035)],c.armor);
      for(const side of [-1,1]){
        inspectionSeam(b,[side*(roofWidth-.045),deck+.069,rear+.47],[side*(roofWidth-.045),deck+.069,roofFront-.12],[0,1,0],c.upper,true);
        inspectionPanel(b,[side*.24,deck+.071,roofFront-.30],.30,.28,[1,0,0],[0,1,0],c.upper);
      }
      b.box([0,deck+.068,roofFront-.055],[roofWidth*1.72,.032,.070],c.edge);
      if(b.keep(0.30))for(let i=-3;i<=3;i++)b.box([i*w*.25,.98,front-.016],[.032,.032,.022],c.bright);
      for(const side of [-1,1]) {
        b.rod([side*w*.44,.86,front-.04],[side*w*.44,.86,front+.14],.055,c.bright,12);
        b.box([side*(w-.055),shoulder,0],[.05,.045,length-.70],c.shade);
        beveled(b,[side*(w+.06),wheeled?tireRadius*2+.14:1.18,0],[.25,.060,length-.40],c.edge,.022);
        for(const z of [rear+.21,front-.12]) {
          b.rod([side*w*.66,.74,z],[side*w*.66,.74,z+(z>0?.16:-.16)],.063,c.bright,12);
          if(b.keep(0.30)) {
            b.box([side*w*.77,1.08,z],[.15,.14,.07],c.shade);
            b.cylinder([side*w*.77,1.08,z],[side*w*.77,1.08,z+(z>0?.035:-.035)],.036,z>0?shade(c.bright,1.35):c.amber,12);
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
          const panelY=deck-.25,panelZ=Math.min(z,roofFront-.24);
          beveled(b,[side*(w-.015),panelY,panelZ],[.20,.39,(length-1.10)/6-.035],c.armor,.045);
          if(b.keep(0.30))for(const dz of [-.15,.15])b.cylinder([side*(w+.078),deck-.13,panelZ+dz],[side*(w+.101),deck-.13,panelZ+dz],.027,c.bright,6);
        }
      }
      if(modular)b.loft([octagon(1.13,w*.78,front-.40,front-.29,.04),
        octagon(deck+.075,roofWidth*.92,roofFront-.10,roofFront-.015,.04)],c.armor);
    });
    for(const side of [-1,1]) {
      const lane=side<0?'port':'starboard';
      const x=w+(wheeled?.02:.09),width=wheeled?.40:(s.tracks==='tracks_wide'?.60:.45),r=wheeled?tireRadius:artillery?.345:aa?.35:.36;
      const half=length/2-(wheeled?.69:.63),count=wheeled?(s.wheels==='ground_wheels_runflat'?4:3):artillery?7:6;
      // All six/seven paired tyres and the two independent end gears must fit
      // in the return span. Keep daylight between the projected wheel circles,
      // rather than concealing the first road wheel behind a sprocket disc.
      const axleHalf=wheeled?half:half-.82,axleY=r+(wheeled?0:.075);
      const axleZ=j=>-axleHalf+2*axleHalf*j/(count-1);
      for(let j=0;j<count;j++)wheel(side,axleZ(j),r,x,width,`${lane} ${wheeled?'road tire':'road wheel'} ${j+1}`);
      b.part(`suspension / ${lane} ${hydro?'hydropneumatic struts':'torsion arms'}`,()=>{
        for(let j=0;j<count;j++) {
          const z=axleZ(j);
          b.rod([side*(w-.30),.87,z-.15],[side*x,axleY,z],b.gauge(hydro?.066:.044),c.steel,12);
          if(hydro)b.cylinder([side*(w-.24),.80,z-.12],[side*(x-.10),axleY+.10,z-.025],.104,c.bright,14);
          // Trailing arm, damper and the bearing housing it swings on. A single
          // rod per wheel is what made the underside read as scaffolding.
          if(b.keep(0.30)) {
            b.cylinder([side*(w-.20),.94,z-.10],[side*(w-.04),.94,z-.10],.088,c.shade,12);
            b.revolve([side*(w-.05),.94,z-.10],[side,0,0],[
              {r:.052,h:0,s:1},{r:.052,h:.10,s:1},{r:.030,h:.13,s:1},{r:0,h:.15}],c.bright,12);
            b.cylinder([side*(w-.16),.88,z+.10],[side*(x-.12),axleY+.16,z+.03],.046,c.steel,6);
            b.box([side*(w-.10),.99,z-.10],[.13,.16,.14],c.steel);
            if(b.level===0){
              b.studs([side*(w-.033),.94,z-.10],[side,0,0],.067,6,.011,.012,c.bright);
              b.cylinder([side*(w-.19),.88,z+.10],[side*(w-.11),.81,z+.087],.061,c.shade,12);
              b.socket([side*(x-.11),axleY+.16,z+.03],[side*(x-.075),axleY+.16,z+.03],.059,.032,c.steel,10);
            }
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
          const binY=wheeled?deck-.18:1.40,binZ=rear+1.12,binLength=scout?.82:artillery?1.38:1.12;
          // Shallow rear tool lockers leave the fighting compartment's sloped
          // sides visible. A single enormous box used to hide half each hull.
          beveled(b,[side*(w-.035),binY,binZ],[.19,.22,binLength],weathered(c.hull,side>0?1:2),.035);
          b.box([side*(w-.035),binY+.125,binZ],[.21,.020,binLength+.02],c.edge);
          if(b.keep(0.30))for(const dz of [-binLength*.31,binLength*.31])b.box([side*(w+.063),binY,binZ+dz],[.023,.075,.04],c.bright);
          for(const dz of [-binLength*.31,binLength*.31]){
            inspectionHinge(b,[side*(w-.05),binY+.13,binZ+dz],[0,0,1],[0,1,0],.13,.018,c.steel);
            inspectionLatch(b,[side*(w+.075),binY+.04,binZ+dz],[0,0,1],[side,0,0],.085,c.bright);
          }
        }
        if(b.keep(0.30))for(let j=0;j<b.many(3,1);j++) {
          const z=rear+.85+j*(roofFront-rear-1.15)/2;
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
          const z=axleZ(j);
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
        const radius=.53,centerY=.53,span=half,perimeter=span*4+TAU*radius,steps=b.many(Math.ceil(perimeter/.205),12,8);
        // Separate end gears now turn the belt; the six/seven paired road
        // wheels sit between them. End fittings stay inside the existing belt
        // part so picking IDs and road-wheel count retain their meaning.
        if(b.level<2)for(const end of [-1,1]){
          const z=end*span,outer=side*x+side*width*.38;
          b.revolve([outer,centerY,z],[side,0,0],[{r:.36,h:-.07},{r:.43,h:-.035,s:1},
            {r:.43,h:.018,s:1},{r:.32,h:.040},{r:.12,h:.055},{r:0,h:.055}],c.steel,20);
          if(b.level===0){
            b.studs([outer+side*.042,centerY,z],[side,0,0],.23,8,.025,.024,c.bright);
            if(end>0)for(let j=0;j<12;j++){
              const a=TAU*j/12,radial=[0,Math.cos(a),Math.sin(a)],tangent=[0,-Math.sin(a),Math.cos(a)];
              b.box([outer,centerY+Math.cos(a)*.443,z+Math.sin(a)*.443],[.09,.055,.072],c.bright,[[1,0,0],radial,tangent]);
            }
          }
        }
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
          if(b.level===0){
            const center=[side*x,p.y,p.z],inward=mul(radial,-1),pitch=perimeter/steps;
            inspectionTrackConnector(b,center,p.tangent,inward,width,pitch);
            inspectionGuideHorn(b,add(center,mul(inward,.022)),inward,p.tangent,.075,pitch*.44,.13);
          }
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
      const z=scout?rear+.93:roofFront-.43,x=w*.44,managed=s.mobility==='ground_engine_750',power=s.mobility==='engine_diesel_1200';
      b.box([x,deck+.105,z],[.66,.07,.75],c.shade);
      const vents=power?14:managed?11:s.mobility==='engine_diesel_900'?9:7;
      if(b.keep(0.30))for(let i=0;i<b.many(vents,3);i++)b.box([x,deck+.151,z-.31+i*.62/(b.many(vents,3)-1)],[.57,.022,.027],c.bright);
      // Framed grille rather than louvres painted on a lid. From above, the deck
      // is most of what a card crop shows of a low hull.
      for(const dx of [-.315,.315])b.box([x+dx,deck+.145,z],[.055,.070,.80],c.upper);
      for(const dz of [-.375,.375])b.box([x,deck+.145,z+dz],[.72,.070,.055],c.upper);
      if(b.level===0){
        // Crossbars join the existing louvres to their frame, and the separate
        // service panel is seated on the deck alongside the cooling bank.
        for(const dx of [-.19,0,.19])b.box([x+dx,deck+.167,z],[.016,.025,.70],c.shade);
        inspectionPanel(b,[-w*.42,deck+.071,z-.04],.54,.61,[1,0,0],[0,1,0],c.upper);
        for(const dz of [-.25,.25])inspectionHinge(b,[x-.36,deck+.16,z+dz],[0,0,1],[0,1,0],.13,.022,c.steel);
      }
      if(b.keep(0.30))for(const dz of [-.30,.30])b.box([x-.40,deck+.10,z+dz],[.10,.075,.10],c.steel);
      if(managed)b.box([x,deck+.17,z+.44],[.49,.14,.16],c.upper);
      for(let j=0;j<(power?2:1);j++) {
        b.cylinder([w-.14,deck-.34,z-.06-j*.30],[w+.04,deck-.34,z-.06-j*.30],.075,c.steel,16);
        b.tube([w+.04,deck-.34,z-.06-j*.30],[w+.12,deck-.26,z-.06-j*.30],.080,.054,c.steel,16);
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
      const radius=mg?.34:howitzer?.97:aaMount?.74:.62;
      b.cylinder([0,deck+.05,mountZ],[0,deck+.105,mountZ],radius,c.steel,32);
      if(mg) {
        b.loft([octagon(deck+.105,.23,-.20,.23,.07,mountZ),octagon(mountTop-.07,.17,-.17,.16,.04,mountZ)],c.upper);
        beveled(b,[0,mountTop-.03,mountZ+.15],[.45,.28,.065],c.armor,.045);
        for(const side of [-1,1])b.box([side*.215,mountTop-.025,mountZ-.01],[.045,.25,.32],c.upper);
        // A bare pedestal reads as a pipe. The bolted ring hatch it stands on and
        // the discharger cluster behind it are what make it a fighting position.
        b.cylinder([0,deck+.07,mountZ],[0,deck+.115,mountZ],.42,c.shade,24);
        if(b.keep(0.30))for(let j=0;j<8;j++){const a=TAU*j/8;b.box([Math.sin(a)*.42,deck+.132,mountZ+Math.cos(a)*.42],[.045,.024,.045],c.bright);}
        if(b.keep(0.30))for(const side of [-1,1])for(let j=0;j<3;j++)
          b.cylinder([side*(.42+j*.10),deck+.24,mountZ-.34],[side*(.46+j*.11),deck+.46,mountZ-.44],.055,c.steel,10);
      } else {
        const tw=mountWidth,back=howitzer?-1.49:aaMount?-.92:-.94,ahead=howitzer?1.05:aaMount?.67:.75;
        // A broad fighting compartment tapers into a narrow gun opening. Its
        // bustle is part of the shell, rather than a high octagonal block
        // balanced on an exposed turntable. The howitzer keeps useful height
        // and rear loading volume; the IFV and radar mount stay low.
        b.loft([cheekSection(deck+.095,tw*.93,back+.10,ahead-.06,.16,mountZ),
          cheekSection(deck+(howitzer?.37:.23),tw,back,ahead,.17,mountZ),
          cheekSection(mountTop-.085,tw*.91,back+.06,ahead-.16,.17,mountZ),
          cheekSection(mountTop,tw*.84,back+.15,ahead-.25,.17,mountZ)],c.upper);
        for(const side of [-1,1])inspectionSeam(b,[side*tw*.69,mountTop+.004,mountZ+back+.35],
          [side*tw*.69,mountTop+.004,mountZ+ahead-.36],[0,1,0],c.upper,true);
        hatch(-tw*.40,mountTop,mountZ-.30,howitzer?.28:.21);
        for(const side of [-1,1]) {
          b.box([side*tw*.91,mountTop-.17,mountZ-.38],[.045,.065,.29],c.shade);
          if(b.keep(0.30))inspectionLatch(b,[side*tw*.68,mountTop+.012,mountZ+back+.42],[0,0,1],[0,1,0],.09,c.steel);
        }
        if(modular)b.loft([cheekSection(deck+.18,tw+.055,back+.20,ahead+.005,.16,mountZ),
          cheekSection(mountTop-.10,tw*.95,back+.28,ahead-.16,.16,mountZ)],c.armor);
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
        const start=mountZ+(indirect?.79:small?.12:.57),rise=indirect?.36:twin?.20:0;
        const offsets=twin?[-mountWidth-.10,mountWidth+.10]:[0];
        for(const x of offsets) {
          if(!small){
            const size=indirect?[.43,.42,.44]:twin?[.26,.27,.66]:[.28,.25,.33];
            beveled(b,[x,gunY,start-.05],size,c.armor,indirect?.085:.045);
            if(twin)b.cylinder([x-Math.sign(x)*.23,gunY,start-.21],[x+Math.sign(x)*.11,gunY,start-.21],.16,c.steel,18);
          }
          b.cylinder([x,gunY,start-.08],[x,gunY,start+.19],radius*1.75,c.shade,20);
          b.always(()=>{
            b.cylinder([x,gunY,start+.25],[x,gunY+rise,start+extent-.22],radius*1.18,c.upper,24,radius);
            b.tube([x,gunY+rise,start+extent-.23],[x,gunY+rise,start+extent],radius*1.08,radius*.68,c.steel,24);
          });
          if(indirect) {
            b.cylinder([x,gunY+rise*.53,start+extent*.50],[x,gunY+rise*.62,start+extent*.60],radius*1.53,c.shade,24);
            b.box([x,gunY+rise,start+extent-.065],[radius*3.6,radius*2.6,.25],c.steel);
            if(b.keep(0.30))for(const side of [-1,1])b.box([x+side*radius*1.82,gunY+rise,start+extent-.05],[.009,radius*1.2,.13],c.black);
            b.rod([x+.19,gunY+.13,start],[x+.19,gunY+.13,start+.91],.060,c.steel,16);
            if(b.level===0){
              b.cylinder([x+.19,gunY+.13,start+.03],[x+.19,gunY+.13,start+.52],.085,c.shade,16);
              for(const dz of [.10,.46])b.box([x+.095,gunY+.13,start+dz],[.19,.11,.05],c.steel);
              b.rod([x+.24,gunY+.14,start+.10],[x+.24,gunY+.14,start+.46],.012,c.cable,8);
            }
          } else if(small) {
            b.box([.12,gunY-.05,start+.12],[.13,.13,.28],c.steel);
            b.box([0,gunY,start+.10],[.105,.095,.42],c.black);
          } else if(b.keep(0.30))for(let j=0;j<2;j++)b.cylinder([x,gunY+rise*j*.28,start+.46+j*.43],[x,gunY+rise*j*.28,start+.485+j*.43],radius*1.28,c.steel,16);
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
      const ammo=s.ammunition,tw=mountWidth,dk=deck+.065;
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
          locker(side*(tw+.11),mountTop-.23,mountZ-.25,[.22,.29,.58],weathered(c.hull,side>0?1:2));
          if(b.keep(0.30)) {
            b.box([side*(tw-.005),mountTop-.14,mountZ-.03],[.15,.09,.30],c.steel);
            for(let j=0;j<b.many(5,2);j++)b.box([side*(tw-.005),mountTop-.14,mountZ-.14+j*.055],[.155,.04,.024],c.bright);
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
      hatch(-w*.42,deck+.065,roofFront-.40,.25);
      if(b.keep(0.30))for(let j=-1;j<=1;j++)b.box([-w*.42+j*.16,deck+.19,roofFront-.18],[.13,.085,.065],c.glass);
      // Coaming and guard bar over the driver's blocks. From above they are the
      // only fitting that says which end of a low flat hull is the front.
      b.box([-w*.42,deck+.135,roofFront-.18],[.52,.055,.11],c.shade);
      if(b.keep(0.30)) {
        for(const dx of [-.25,.25])b.rod([-w*.42+dx,deck+.24,roofFront-.14],[-w*.42+dx,deck+.30,roofFront-.14],.016,c.steel,6);
        b.rod([-w*.42-.25,deck+.30,roofFront-.14],[-w*.42+.25,deck+.30,roofFront-.14],.016,c.steel,6);
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
          b.box([-w*.42,deck+.26,roofFront-.18],[.30,.10,.14],c.shade);
          b.box([-w*.42,deck+.26,roofFront-.11],[.22,.055,.016],c.glass);
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
        const x=side*(w-.20),z=roofFront-.20;
        b.box([x,deck+.09,z],[.16,.17,.18],c.shade);
        b.box([x,deck+.09,z+.095],[.10,.095,.015],c.lens);
        const count=s.active_protection==='aps_hard'?4:2;
        if(b.keep(0.30))for(let j=0;j<count;j++)b.cylinder([x,deck+.06,z-.30-j*.095],[x+side*.13,deck+.24,z-.21-j*.095],.041,c.steel,12);
        if(count===4)b.box([side*(w-.10),deck+.28,rear+.47],[.21,.25,.27],c.armor);
      }
    });
    if(s.troop_compartment)b.part(`troop_compartment / ${s.troop_compartment==='ground_troops_protected'?'reinforced troop bay':'troop bay and rear egress'}`,()=>{
      const protectedBay=s.troop_compartment==='ground_troops_protected',height=protectedBay?.18:.065;
      if(protectedBay)b.loft([octagon(deck+.04,w*.77,rear+.30,rear+2.15,.11),
        octagon(deck+height,w*.66,rear+.36,rear+2.08,.12)],c.upper);
      // Roof edge lip and a stowage basket forward of the hatches. Crews stow on
      // the roof of a carrier, and the top view had nothing between the hatches.
      for(const dx of [-w*.67,w*.67])b.box([dx,deck+height+.018,rear+1.20],[.035,.035,1.70],c.edge);
      for(const dz of [rear+.36,rear+2.04])b.box([0,deck+height+.018,dz],[w*1.36,.035,.035],c.edge);
      if(b.keep(0.30)) {
        for(let j=0;j<2;j++)b.rod([-w*.55,deck+height+.05,rear+.30+j*.44],[w*.55,deck+height+.05,rear+.30+j*.44],.018,c.steel,6);
        for(let j=0;j<b.many(5,2);j++)b.rod([(j-2)*w*.275,deck+height+.05,rear+.28],[(j-2)*w*.275,deck+height+.05,rear+.76],.014,c.steel,6);
      }
      beveled(b,[0,deck+height+.09,rear+.56],[w*.52,.12,.30],c.canvas,.04);
      // Roof hatches, vision blocks and the periscope cluster each crewman gets.
      for(const side of [-1,1]) {
        hatch(side*w*.37,deck+height,rear+1.25,.25);
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
      inspectionHinge(b,[w*.32-.24,deck-.25,rear-.13],[0,1,0],[0,0,-1],.16,.025,c.steel);
      inspectionHinge(b,[w*.32-.24,deck-.66,rear-.13],[0,1,0],[0,0,-1],.16,.025,c.steel);
      inspectionSeam(b,[w*.32-.23,deck-.83,rear-.109],[w*.32+.23,deck-.83,rear-.109],[0,0,-1],c.edge);
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
      const elevated=s.recon_package==='ground_recon_mast',z=rear+1.65,y=deck+(elevated?1.72:.39);
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
      const assisted=s.artillery_loader==='ground_loader_assisted',z=mountZ-1.43;
      beveled(b,[0,deck+.65,z],[assisted?1.35:.86,assisted?.71:.56,assisted?.49:.13],c.shade,.075);
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
        beveled(b,[0,y,z],[tracking?1.24:1.02,tracking?.55:.38,.14],c.upper,.065);
        beveled(b,[0,y,z+.079],[tracking?1.13:.91,tracking?.44:.27,.024],c.steel,.055);
      });
      if(b.keep(0.30))for(let i=-4;i<=4;i++)b.box([i*.105,y,z+.095],[.014,tracking?.44:.27,.012],c.shade);
      if(tracking) {
        b.rod([.63,mountTop-.02,mountZ+.14],[.83,mountTop+.41,mountZ+.13],.07,c.steel,16);
        b.cylinder([.83,mountTop+.46,mountZ+.12],[.83,mountTop+.46,mountZ+.26],.27,c.upper,32,.31);
        b.revolve([.83,mountTop+.46,mountZ+.265],[0,0,1],[{r:.265,h:0},{r:.24,h:.07,s:1},{r:.13,h:.12,s:1},{r:0,h:.14}],c.upper,24);
      }
    });
    const names={ground_ifv:'tracked infantry fighting vehicle',ground_apc:'wheeled armored personnel carrier',ground_recon:'wheeled reconnaissance vehicle',ground_artillery:'self-propelled artillery',ground_air_defense:'mobile air-defense vehicle'};
    const result=b.finish(`Visual interpretation of a fictional ${names[spec.platform]}. Component-driven original game art; simulation values are separate.`);
    result.specification={platform:spec.platform,components:{...s}};
    return result;
  }

  function build(rawSpec) {
    const spec = resolveSpec(rawSpec), chosen = spec.components, heavy = spec.platform === "tank_heavy";
    if(Object.hasOwn(AIR_DEFAULTS,spec.platform))return buildAircraft(spec,levelOf(rawSpec));
    const detail = levelOf(rawSpec);
    if(Object.hasOwn(GROUND_DEFAULTS,spec.platform))return buildGround(spec, detail);
    const mobile = ["drive_mobile","engine_diesel_1200","engine_turbine_1500"].includes(chosen.mobility), reinforced = chosen.protection === "protection_heavy", active = chosen.protection === "protection_active" || chosen.active_protection === "aps_hard";
    const heavyGun = ["armament_heavy","gun_120","gun_125"].includes(chosen.armament), integrated = ["sensors_integrated","optics_thermal"].includes(chosen.sensors), data = chosen.communications === "comms_data";
    const compact=chosen.turret==='turret_compact',casemate=chosen.turret==='turret_casemate',autoload=chosen.turret==='turret_autoload',large=chosen.turret==='turret_heavy';
    // A tank's long load-bearing hull and broad, low turret are its primary
    // forms. These dimensions belong to all three tank LODs; stretching only
    // the close view would make the vehicle change size on the map.
    const hullWidth = heavy ? 1.48 : 1.34, rear = heavy ? -3.80 : -3.44, front = heavy ? 3.50 : 3.24;
    const b = createBuilder(detail, front - rear), c = PALETTE;
    const wheelHalfSpan = heavy ? 3.10 : 2.78, trackX = hullWidth + 0.15, trackWidth = (heavy ? 0.70 : 0.64)+(chosen.tracks==='tracks_wide'?.19:0);
    const trackRadius = 0.65, trackCenterY = 0.65, turretY = heavy ? 1.55 : 1.53, turretZ = 0.13;
    const turretWidth = (heavy ? 1.53 : 1.40)*(compact?.82:casemate?1.02:large?1.08:1), turretTop = turretY + (casemate?.65:compact?.63:autoload?.68:heavyGun?.84:.74);
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
      for(const side of [-1,1])inspectionSeam(b,[side*(hullWidth-.075),1.544,rear+.46],
        [side*(hullWidth-.075),1.544,front-.86],[0,1,0],c.upper,true);
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
      // The close-view glacis is folded armor, not a cylindrical bulge laid on
      // top of the bow. The distant mesh preserves its original silhouette.
      inspectionBow(b,hullWidth,front,1.02,1.42,c.armor);
    });

    b.part("chassis / glacis applique, splash guard and spare track links", () => {
      // Applique is spaced off the glacis on purpose — flush plate would just be
      // a colour change. The basis is the measured glacis slope so the plates lie
      // on the surface rather than floating at their own angle.
      const up = [0, 0.7407, -0.6745], out = [0, 0.6745, 0.7407], slope = [[1, 0, 0], up, out];
      if(!reinforced)for (let i = 0; i < b.many(5, 2); i++) {
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
        // Compact lamps stand on brackets attached to the upper deck edge. Their
        // clear faces use neutral metal-glass tones, distinct from sight optics.
        if(b.level===0){
          b.box([x*1.12,1.53,front-.55],[.12,.08,.15],c.steel);
          b.revolve([x*1.12,1.58,front-.55],[0,.08,1],[
            {r:0,h:0},{r:.077,h:.014},{r:.083,h:.095,s:1},
            {r:.069,h:.120,c:shade(c.bright,1.35)},{r:0,h:.123}],c.shade,10);
        }else if (b.keep(0.30)) b.revolve([x * 1.12, 1.44, front - 0.40], [0, 0.55, 0.83], [
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
            // The inspection horn tapers into the now-open paired-wheel channel;
            // distant levels keep their original readable block silhouettes.
            if(b.level===0){
              inspectionGuideHorn(b,add(center,mul(outward,-.022)),mul(outward,-1),tangent,.075,step*.42,.16);
              inspectionTrackConnector(b,center,tangent,mul(outward,-1),trackWidth,step);
            }else{
              b.box(add(center, mul(outward, -0.088)), [0.070, 0.13, step * 0.30], c.steel, basis);
              b.box(add(center, mul(outward, -0.155)), [0.060, 0.055, step * 0.22], c.bright, basis);
            }
          }
        }
      });
      b.part(`running gear / ${label} suspension, road wheels and sprockets`, () => {
        const wheels = heavy ? 7 : 6, first = -wheelHalfSpan + 0.46, last = wheelHalfSpan - 0.46;
        for (let i = 0; i < wheels; i++) {
          const z = first + (last - first) * i / (wheels - 1), wheelY = heavy?.54:.53, wheelRadius = heavy?.41:.40;
          b.rod([side * (hullWidth - 0.14), 0.90, z - 0.24], [x, wheelY, z], b.gauge(0.085), c.steel, 12);
          // Bump stop above each arm. It is the fitting that tells the eye the
          // arm swings, and it survives the shrink to card size as a shadow.
          b.box([side * (hullWidth - 0.05), 1.02, z - 0.19], [0.17, 0.13, 0.16], c.shade);
          if(b.level===0){
            const pivot=[side*(hullWidth-.03),.90,z-.24];
            b.cylinder(add(pivot,[-side*.11,0,0]),add(pivot,[side*.065,0,0]),.12,c.shade,16);
            b.studs(add(pivot,[side*.067,0,0]),[side,0,0],.087,6,.016,.015,c.bright);
            b.cylinder([x,.48,z],[x,.58,z-.07],.12,c.steel,12);
          }
          if(chosen.suspension==='suspension_hydro') {
            b.cylinder([x+side*.25,.86,z-.18],[x+side*.25,.53,z],.065,c.bright,12);
            b.cylinder([x+side*.25,.99,z-.25],[x+side*.25,.76,z-.13],.10,c.shade,12);
            if(b.level===0){
              // The pressure reservoir and its mount distinguish a hydraulic
              // unit from a torsion arm. A thin piston alone disappeared behind
              // the newly separated wheels. This assembly is carried by the
              // existing upper strut, below the fender and inside the track width.
              const reservoir=[x+side*.19,.925,z-.46];
              b.revolve(reservoir,[0,0,1],[{r:0,h:0},{r:.055,h:0},{r:.116,h:.045,s:1},
                {r:.128,h:.095,s:1},{r:.128,h:.285,s:1},{r:.105,h:.34,s:1},{r:.050,h:.385},{r:0,h:.385}],c.shade,16);
              for(const dz of [.105,.265]){
                b.cylinder(add(reservoir,[0,0,dz-.018]),add(reservoir,[0,0,dz+.018]),.135,c.steel,16);
                b.box([x+side*.135,.91,z-.46+dz],[.17,.095,.062],c.steel);
              }
              b.rod([x+side*.19,.925,z-.075],[x+side*.25,.90,z-.20],.026,c.cable,10);
              b.socket([x+side*.19,.925,z-.455],[x+side*.19,.925,z-.495],.036,.017,c.bright,10);
            }
          }
          b.wheelBody([x, wheelY, z], [1, 0, 0], wheelRadius, 0.27, c.rubber, c.upper, c.shade, true);
        }
        // Rear drive sprocket follows the rear powerpack; the front idler has a
        // distinct tensioning web. Both are concentric with the track return,
        // rather than tiny wheels floating inside a much larger belt curve.
        for (const end of [-1, 1]) {
          const z = end * wheelHalfSpan, y = trackCenterY, r = 0.51;
          b.cylinder([x - 0.28, y, z], [x + 0.28, y, z], r, c.steel, 18);
          b.cylinder([x + side * 0.281, y, z], [x + side * 0.326, y, z], r * 0.75, c.hull, 18);
          b.cylinder([x + side * 0.325, y, z], [x + side * 0.355, y, z], 0.13, c.bright, 12);
          if (end < 0) { if (b.keep(0.30)) for (let tooth = 0; tooth < b.many(14, 5); tooth++) {
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
          const z = (i - 1) * wheelHalfSpan * 0.70, length = i === 1 ? 1.32 : 1.12;
          beveled(b, [x, top + 0.10, z], [trackWidth + 0.03, 0.20, length], weathered(c.hull, i + (side > 0 ? 1 : 0)), 0.06);
          b.box([x, top + 0.212, z], [trackWidth + 0.06, 0.026, length + 0.03], c.edge);
          if (b.keep(0.30)) for (const dz of [-length * 0.30, length * 0.30]) b.box([x + side * (trackWidth / 2 + 0.005), top + 0.125, z + dz], [0.030, 0.055, 0.055], c.bright);
          if (b.keep(0.30)) b.rod([x - 0.15, top + 0.237, z - length * 0.35], [x + 0.15, top + 0.237, z - length * 0.35], 0.016, c.steel, 6);
          inspectionSeam(b,[x-trackWidth*.40,top+.230,z-length*.43],[x+trackWidth*.40,top+.230,z-length*.43],[0,1,0],c.upper);
          for(const dz of [-length*.29,length*.29]){
            inspectionHinge(b,[x-side*trackWidth*.43,top+.241,z+dz],[0,0,1],[0,1,0],.15,.025,c.steel);
            inspectionLatch(b,[x+side*(trackWidth*.5+.035),top+.125,z+dz],[0,0,1],[side,0,0],.10,c.bright);
          }
        }
        // Pioneer tools clamped outboard: two long thin runs that catch the light
        // along the whole flank for the price of two rods.
        if (b.keep(0.30)) for (let i = 0; i < 2; i++) b.rod([x + side * (trackWidth / 2 + 0.055), top + 0.07 + i * 0.085, -0.62], [x + side * (trackWidth / 2 + 0.055), top + 0.07 + i * 0.085, 0.66], 0.027, i ? c.steel : c.cable, 8);
        // Tie-down sockets along the bin line. Five per side, and they are what a
        // crew actually lashes stowage to.
        if (b.keep(0.30)) for (let i = 0; i < b.many(5, 2); i++)
          b.socket([x, top + 0.235, -1.30 + i * 0.65], [x, top + 0.295, -1.30 + i * 0.65], 0.070, 0.042, c.steel, 18);
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
      inspectionPanel(b,[0,1.625,engineFront+.30],.73,.34,[1,0,0],[0,1,0],c.upper);
      if(b.level===0)for(const x of [-hullWidth*.40,hullWidth*.40]){
        for(const dx of [-.23,.23])b.box([x+dx,1.637,mid],[.018,.021,engineFront-engineRear-.08],c.shade);
        for(const z of [engineRear+.25,engineFront-.25])inspectionHinge(b,[x-hullWidth*.365,1.65,z],[0,0,1],[0,1,0],.17,.026,c.steel);
      }
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
      // Low protected intake caps follow the engine-deck surface. The large
      // standing drums previously competed with the turret's main silhouette.
      for (const side of [-1, 1]) if (b.keep(0.30)) b.revolve([side * (hullWidth - 0.60), 1.585, rear + 1.30], [0, 1, 0], [
        { r: 0.11, h: 0 }, { r: 0.255, h: 0.018 }, { r: 0.270, h: 0.045, s: 1 },
        { r: 0.255, h: 0.075, s: 1 }, { r: 0.220, h: 0.095 }, { r: 0.110, h: 0.100 }], c.steel, 24);
      for (const x of [-hullWidth + 0.18, hullWidth - 0.18]) {
        b.box([x, 1.635, rear + 0.72], [0.21, 0.18, 0.75], c.hull);
        if (b.keep(0.30)) for (const z of [rear + 0.41, rear + 1.01]) b.box([x, 1.733, z], [0.22, 0.02, 0.053], c.bright);
      }
    });

    b.part("turret / ring and faceted armor shell", () => {
      // The ring lives under the overhang, not on a tall cylindrical pedestal.
      // A broad shoulder carries a continuous rear bustle and two wedge cheeks.
      // Roof and belly have only a narrow bevel, leaving large readable planes.
      if(!casemate){
        b.cylinder([0,1.49,turretZ],[0,turretY+.055,turretZ],turretWidth*.80,c.steel,40);
        const section=(y,w,back,nose)=>[
          [-w*.78,y,back],[w*.78,y,back],[w,y,back+.32],
          [w,y,nose-.94],[w*.76,y,nose-.30],[w*.30,y,nose],
          [-w*.30,y,nose],[-w*.76,y,nose-.30],[-w,y,nose-.94],[-w,y,back+.32]
        ];
        b.loft([
          section(turretY,turretWidth*.88,-1.89,1.42),
          section(turretY+.20,turretWidth,-1.98,1.78),
          section(turretTop-.055,turretWidth*.86,-1.87,1.17),
          section(turretTop,turretWidth*.835,-1.83,1.12)
        ],c.upper);
        b.cylinder([0,1.50,turretZ],[0,1.56,turretZ],turretWidth*.84,c.shade,24);
        for(const side of [-1,1]){
          // Edge joints separate the removable cheek package without placing
          // another shallow, disconnected plate in front of the turret volume.
          inspectionSeam(b,[side*turretWidth*.78,turretY+.225,1.38],
            [side*turretWidth*.68,turretTop-.065,.86],normal([side,.22,1]),c.armor,true);
          if(b.keep(.30))b.socket([side*(turretWidth*.86-.13),turretTop-.10,-.66],
            [side*(turretWidth*.86+.025),turretTop-.02,-.66],.065,.038,c.steel,16);
        }
        // The rear armor remains part of the rotating body. These shallow
        // service panels read as access in that body, rather than a second box.
        beveled(b,[0,turretY+.40,-1.948],[turretWidth*1.38,.30,.070],c.armor,.022);
        if(b.keep(.30))for(const x of [-turretWidth*.52,0,turretWidth*.52])
          b.box([x,turretY+.47,-1.986],[.23,.025,.016],c.steel);
      }else{
        // The fixed casemate grows directly from the hull shoulder and keeps
        // the lower gun opening broad; it is not a turret on a square plinth.
        b.loft([octagon(1.44,hullWidth*.97,-2.10,2.48,.20),
          octagon(1.44+(turretTop-1.44)*.56,turretWidth*.91,-1.94,1.51,.25),
          octagon(turretTop,turretWidth*.83,-1.72,1.05,.18)],c.upper);
      }
    });

    b.part(`armament / ${heavyGun ? "heavy weapon, enlarged mantlet and sleeved barrel" : "standard weapon, mantlet and barrel"}`, () => {
      const gunY=turretY+.43,radius=chosen.armament==='gun_90'?.070:chosen.armament==='gun_125'?.103:heavyGun?.096:.083;
      const bore=chosen.armament==='gun_90'?.045:chosen.armament==='gun_125'?.0625:heavyGun?.060:.0525;
      const muzzle=chosen.armament==='gun_90'?5.65:chosen.armament==='gun_125'?7.57:heavyGun?7.25:6.55;
      // The trunnion sits within a low armored opening. Only the short collar
      // and a restrained fabric seal project ahead of the turret cheeks.
      beveled(b,[0,gunY,1.43],[heavyGun?.76:.65,heavyGun?.49:.43,.50],c.shade,.09);
      b.cylinder([-.27,gunY,1.53],[.27,gunY,1.53],heavyGun?.225:.205,c.armor,28);
      b.cylinder([0,gunY,1.65],[0,gunY,1.83],radius*1.75,c.steel,28,radius*1.28);
      b.revolve([0,gunY,1.72],[0,0,1],[{r:radius*1.93,h:0},
        {r:radius*1.88,h:.06,s:1},{r:radius*1.48,h:.18,s:1},{r:radius*1.19,h:.24}],c.canvas,24);
      b.cylinder([0,gunY,1.94],[0,gunY,muzzle-.20],radius*1.08,c.upper,36,radius);
      const evacuator=2.05+(muzzle-2.05)*.41;
      b.revolve([0,gunY,evacuator],[0,0,1],[{r:radius*1.08,h:0},
        {r:radius*1.48,h:.13,s:1},{r:radius*1.52,h:.49,s:1},
        {r:radius*1.41,h:.64,s:1},{r:radius*1.07,h:.72}],c.upper,32);
      if(b.keep(.30)){
        // Thin sleeve joints follow the same continuous barrel. Their height
        // stays subordinate to the bore evacuator instead of forming beads.
        for(const z of [2.23,evacuator-.15,evacuator+.86,muzzle-.67])
          b.cylinder([0,gunY,z],[0,gunY,z+.032],radius*1.13,c.steel,24);
        const sleeve=muzzle-1.72;
        b.revolve([0,gunY,sleeve],[0,0,1],[{r:radius*1.04,h:0},
          {r:radius*1.12,h:.07,s:1},{r:radius*1.12,h:1.16,s:1},{r:radius*1.03,h:1.25}],c.upper,28);
        inspectionSeam(b,[radius*1.115,gunY,sleeve+.12],[radius*1.115,gunY,sleeve+1.06],[1,0,0],c.upper);
        for(const dz of [.22,.88])inspectionLatch(b,[radius*1.13,gunY,sleeve+dz],[0,0,1],[1,0,0],.037,c.steel);
      }
      b.always(()=>b.tube([0,gunY,muzzle-.30],[0,gunY,muzzle],radius*1.035,bore,c.steel,40));
      b.cylinder([0,gunY,muzzle-.325],[0,gunY,muzzle-.305],bore*.99,c.black,32);
      if(b.keep(.30)){
        b.box([0,gunY+radius*1.07,muzzle-.19],[.059,.034,.15],c.shade);
        b.box([.32,gunY-.025,1.62],[.095,.083,.10],c.black);
        b.tube([.32,gunY-.025,1.63],[.32,gunY-.025,1.79],.031,.018,c.steel,16);
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
        inspectionHinge(b,[hatch.x,turretTop+.17,hatch.z-hatch.radius*.88],[1,0,0],[0,1,0],hatch.radius*.90,.035,c.steel);
        inspectionLatch(b,[hatch.x,turretTop+.222,hatch.z+hatch.radius*.52],[1,0,0],[0,1,0],.105,c.bright);
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
        for(const side of [-1,1]){
          // The cheek is one deep wedge module. Its armor face sweeps back from
          // the gun opening into the flank; horizontal stair-step boxes cannot
          // approximate this surface without breaking its primary silhouette.
          const ring=(y,points)=>points.map(([x,z])=>[side*x,y,z]);
          const bottom=ring(turretY+.19,[[.39,1.86],[turretWidth*.83,1.75],
            [turretWidth*1.065,.66],[turretWidth*.93,.40],[.48,.90]]);
          const top=ring(turretTop-.065,[[.38,1.14],[turretWidth*.74,1.08],
            [turretWidth*.91,.43],[turretWidth*.82,.21],[.45,.67]]);
          b.loft([bottom,top],c.armor);
          inspectionSeam(b,[side*turretWidth*.62,turretY+.215,1.808],
            [side*turretWidth*.57,turretTop-.061,1.112],normal([side*.10,.76,.65]),c.armor);
          // Flank modules are a single aligned row, clear of the main wedge.
          for(let i=0;i<b.many(4,2);i++){
            const z=-1.55+i*.42,x=side*(turretWidth*.94+.055),y=turretY+.40;
            beveled(b,[x,y,z],[.18,.36,.385],weathered(c.armor,i),.025,
              [normal([1,side*.34,0]),normal([-side*.34,1,0]),[0,0,1]]);
          }
          for(let i=0;i<b.many(6,2);i++){
            const z=-wheelHalfSpan+.42+i*(wheelHalfSpan*2-.84)/5;
            b.box([side*(trackX+trackWidth/2+.13),1.18,z],[b.gauge(.12),.43,.61],c.armor);
            if(b.keep(.30))b.box([side*(trackX+trackWidth/2+.198),1.18,z],[.014,.33,.50],c.edge);
          }
        }
        // A continuous sloped glacis bank replaces the two horizontal rows.
        const up=[0,.7407,-.6745],out=[0,.6745,.7407],center=[0,1.25,front-.20];
        beveled(b,center,[hullWidth*1.62,.57,.12],c.armor,.045,[[1,0,0],up,out]);
        for(const x of [-hullWidth*.49,0,hullWidth*.49])
          inspectionSeam(b,add([x,center[1],center[2]],add(mul(up,-.235),mul(out,.063))),
            add([x,center[1],center[2]],add(mul(up,.235),mul(out,.063))),out,c.armor);
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
      const rackRear = -2.18, rackFront = -1.68, rackY = turretY + 0.30, rackHalf = turretWidth * 0.82;
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
      // Brackets return to the rotating bustle, never to the fixed engine deck.
      // A turret magazine must move with its parent assembly.
      if(b.keep(0.30))for(const side of [-1,1]) {
        b.rod([side*bayHalf*.78,bayY-bayH/2,bayFront-.05],[side*bayHalf*.86,turretY+.18,bayFront+.16],.030,c.steel,8);
        b.box([side*bayHalf*.86,turretY+.18,bayFront+.16],[.13,.055,.17],c.shade);
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
      if(chosen.fire_control==='fcs_stabilized')b.cylinder([0,turretY+.43,2.02],[0,turretY+.43,2.20],.16,c.bright,24);
      if(chosen.fire_control==='fcs_digital'){b.box([.64,turretTop-.12,.87],[.33,.27,.34],c.shade);b.box([.64,turretTop-.12,1.05],[.25,.14,.025],c.lens);if(b.keep(0.30))b.rod([.64,turretTop-.24,.7],[.80,turretY+.37,.38],.022,c.cable);}
    });
    const result=b.finish(`Original ${spec.platform.replace('tank_','')} tank game model. Visual interpretation of the selected specifications. Internal ammunition loads affect game ratings and are recorded in the exported specification metadata.`);
    if(spec.platform==='tank_light'){for(let i=0;i<result.positions.length;i++)result.positions[i]*=.80;result.bounds.min=result.bounds.min.map(v=>v*.80);result.bounds.max=result.bounds.max.map(v=>v*.80);}
    result.specification={platform:spec.platform,components:Object.fromEntries(Object.entries(chosen).filter(([,value])=>value))};
    return result;
  }
  return Object.freeze({ build });
});
