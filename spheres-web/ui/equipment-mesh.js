/* Original, component-driven game geometry. These shapes are visual concepts,
   not historical vehicle specifications. +Z is forward; Y=0 is the ground. */
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
  const add = (a, b) => [a[0] + b[0], a[1] + b[1], a[2] + b[2]];
  const sub = (a, b) => [a[0] - b[0], a[1] - b[1], a[2] - b[2]];
  const mul = (a, n) => [a[0] * n, a[1] * n, a[2] * n];
  const dot = (a, b) => a[0] * b[0] + a[1] * b[1] + a[2] * b[2];
  const cross = (a, b) => [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]];
  const normal = a => { const length = Math.hypot(...a); return length > 1e-10 ? mul(a, 1 / length) : [0, 1, 0]; };
  const mean = points => mul(points.reduce((sum, point) => add(sum, point), [0, 0, 0]), 1 / points.length);
  const shade = (color, amount) => color.map(value => Math.max(0, Math.min(1, value * amount)));

  function createBuilder() {
    const positions = [], normals = [], colors = [], parts = [];
    const bounds = { min: [Infinity, Infinity, Infinity], max: [-Infinity, -Infinity, -Infinity] };
    function triangle(a, b, c, color) {
      const perpendicular = cross(sub(b, a), sub(c, a));
      if (dot(perpendicular, perpendicular) < 1e-18) return;
      const n = normal(perpendicular);
      for (const point of [a, b, c]) {
        positions.push(...point); normals.push(...n); colors.push(...color);
        for (let axis = 0; axis < 3; axis++) {
          bounds.min[axis] = Math.min(bounds.min[axis], point[axis]);
          bounds.max[axis] = Math.max(bounds.max[axis], point[axis]);
        }
      }
    }
    function face(points, color, interior) {
      let ordered = points;
      if (interior && dot(cross(sub(points[1], points[0]), sub(points[2], points[0])), sub(mean(points), interior)) < 0) ordered = [...points].reverse();
      for (let i = 1; i < ordered.length - 1; i++) triangle(ordered[0], ordered[i], ordered[i + 1], color);
    }
    function part(name, draw, slot) {
      const first = positions.length / 3;
      draw();
      const count = positions.length / 3 - first;
      if (count) {
        const prefix=name.split(' / ')[0];
        slot=slot||(prefix==='running gear'?(name.includes('suspension')?'suspension':'tracks'):prefix==='chassis'?(name.includes('periscopes')?'sensors':'protection'):prefix==='stowage'?'ammunition':prefix);
        parts.push({ name, first, count, slot, label:name.split(' / ').slice(1).join(' / ')||name });
      }
    }
    function box(center, size, color, basis) {
      const axes = basis || [[1, 0, 0], [0, 1, 0], [0, 0, 1]];
      const corner = (x, y, z) => add(center, add(mul(axes[0], x * size[0] / 2), add(mul(axes[1], y * size[1] / 2), mul(axes[2], z * size[2] / 2))));
      const p = [corner(-1, -1, -1), corner(1, -1, -1), corner(1, 1, -1), corner(-1, 1, -1),
        corner(-1, -1, 1), corner(1, -1, 1), corner(1, 1, 1), corner(-1, 1, 1)];
      for (const indices of [[0, 1, 2, 3], [4, 7, 6, 5], [0, 4, 5, 1], [3, 2, 6, 7], [0, 3, 7, 4], [1, 5, 6, 2]]) face(indices.map(i => p[i]), color, center);
    }
    // Convex cross sections create continuous bevels and sloped armor faces.
    function loft(rings, color) {
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
    function cylinder(a, b, radius, color, segments = 24, radiusEnd = radius, caps = true) {
      const axis = normal(sub(b, a));
      const u = normal(cross(axis, Math.abs(axis[1]) > 0.9 ? [1, 0, 0] : [0, 1, 0]));
      const v = cross(axis, u), center = mul(add(a, b), 0.5), starts = [], ends = [];
      for (let i = 0; i < segments; i++) {
        const theta = TAU * i / segments, radial = add(mul(u, Math.cos(theta)), mul(v, Math.sin(theta)));
        starts.push(add(a, mul(radial, radius))); ends.push(add(b, mul(radial, radiusEnd)));
      }
      for (let i = 0; i < segments; i++) {
        const next = (i + 1) % segments;
        face([starts[i], starts[next], ends[next], ends[i]], color, center);
      }
      if (caps) { face(starts, color, center); face(ends, color, center); }
    }
    function tube(a, b, radius, innerRadius, color, segments = 32) {
      const axis = normal(sub(b, a)), u = normal(cross(axis, Math.abs(axis[1]) > 0.9 ? [1, 0, 0] : [0, 1, 0])), v = cross(axis, u);
      const ring = (center, r) => Array.from({ length: segments }, (_, i) => add(center, add(mul(u, r * Math.cos(TAU * i / segments)), mul(v, r * Math.sin(TAU * i / segments)))));
      const outerA = ring(a, radius), outerB = ring(b, radius), innerA = ring(a, innerRadius), innerB = ring(b, innerRadius), center = mul(add(a, b), 0.5);
      for (let i = 0; i < segments; i++) {
        const j = (i + 1) % segments;
        face([outerA[i], outerA[j], outerB[j], outerB[i]], color, center);
        const inward = mul(add(innerA[i], innerA[j]), 0.5);
        const nearWall = add(inward, mul(sub(inward, a), 0.5));
        face([innerA[i], innerB[i], innerB[j], innerA[j]], PALETTE.black, nearWall);
        face([outerB[i], outerB[j], innerB[j], innerB[i]], PALETTE.bright, a);
        face([outerA[i], innerA[i], innerA[j], outerA[j]], color, b);
      }
    }
    function rod(a, b, radius, color, segments = 10) { cylinder(a, b, radius, color, segments); }
    return {
      triangle, face, part, box, loft, cylinder, tube, rod,
      finish(description) {
        // Flat tread shoes meet a curved belt at their corners. Seat the actual
        // lowest vertex on the ground, rather than clipping the shoe geometry.
        const ground = bounds.min[1];
        for (let i = 1; i < positions.length; i += 3) positions[i] -= ground;
        bounds.max[1] -= ground; bounds.min[1] = 0;
        return { positions: new Float32Array(positions), normals: new Float32Array(normals), colors: new Float32Array(colors), bounds, parts, triangleCount: positions.length / 9, description };
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
  function buildGround(spec) {
    const s=spec.components,b=createBuilder(),c=PALETTE;
    const scout=spec.platform==='ground_recon',carrier=spec.platform==='ground_apc',ifv=spec.platform==='ground_ifv',artillery=spec.platform==='ground_artillery',aa=spec.platform==='ground_air_defense';
    const wheeled=carrier||scout,w=scout?1.03:carrier?1.16:artillery?1.43:1.28,length=scout?4.45:carrier?5.50:artillery?6.55:5.75;
    const rear=-length/2,front=length/2,deck=scout?1.67:carrier?1.98:ifv?1.78:1.52;
    const modular=s.protection==='ground_armor_modular',hydro=s.suspension==='suspension_hydro';
    const mountZ=artillery?-.63:aa?.12:scout?.15:.73;
    const mg=s.turret==='ground_station_mg',howitzer=s.turret==='ground_turret_howitzer',aaMount=s.turret==='ground_turret_aa';
    const mountTop=deck+(mg?.37:howitzer?1.42:aaMount?.91:.76),gunY=mountTop-(howitzer?.52:mg?.08:.24);
    const labels={engine_diesel_600:'compact diesel',engine_diesel_900:'standard diesel',engine_diesel_1200:'high-output diesel',ground_engine_750:'managed diesel',ground_gun_25:'25 mm autocannon',ground_gun_35:'35 mm autocannon',ground_mg_127:'12.7 mm machine gun',ground_howitzer_122:'122 mm howitzer',ground_howitzer_155:'155 mm howitzer',ground_aa_gun:'twin air-defense cannon',ground_aa_missiles:'short-range missile launcher',ground_ammo_autocannon:'autocannon',ground_ammo_ball:'machine gun',ground_ammo_he:'artillery',ground_ammo_guided:'guided artillery',ground_ammo_aa:'air-defense cannon',ground_ammo_missiles:'missile',optics_day:'daylight',optics_night:'night',optics_thermal:'thermal',fcs_basic:'basic',fcs_stabilized:'stabilized',fcs_digital:'digital',comms_radio:'field radio',comms_data:'tactical data',ground_comms_secure:'secure radio',ground_comms_network:'networked command',aps_soft:'soft-kill',aps_hard:'active interception'};
    const label=id=>labels[id]||String(id).replace(/^ground_/,'').replace(/_/g,' ');
    function hatch(x,y,z,r=.25) {
      b.cylinder([x,y,z],[x,y+.045,z],r,c.shade,24);
      b.cylinder([x,y+.045,z],[x,y+.08,z],r*.90,c.upper,24);
      b.rod([x-r*.30,y+.10,z],[x+r*.30,y+.10,z],.021,c.bright);
    }
    function wheel(side,z,r,x,width,label) {
      b.part(`running gear / ${label}`,()=>{
        b.cylinder([side*(x-width/2),r,z],[side*(x+width/2),r,z],r,c.rubber,32);
        b.cylinder([side*(x+width/2),r,z],[side*(x+width/2+.025),r,z],r*.72,c.steel,28);
        b.cylinder([side*(x+width/2+.026),r,z],[side*(x+width/2+.055),r,z],r*.57,c.upper,24);
        b.cylinder([side*(x+width/2+.056),r,z],[side*(x+width/2+.10),r,z],r*.23,c.shade,16);
        for(let j=0;j<8;j++) {
          const a=TAU*j/8,cy=r+Math.cos(a)*r*.39,cz=z+Math.sin(a)*r*.39;
          b.cylinder([side*(x+width/2+.05),cy,cz],[side*(x+width/2+.075),cy,cz],.022,c.bright,6);
        }
        // A smooth cylinder is the fastest way to make a wheeled hull look
        // unfinished. Two shoulder rows, a centre row staggered off their pitch,
        // and a bead ring on each sidewall, for 376 triangles a tyre.
        if(wheeled) {
          for(let j=0;j<32;j++) {
            const a=TAU*j/32,radial=[0,Math.cos(a),Math.sin(a)],tangent=[0,-Math.sin(a),Math.cos(a)];
            for(const strip of [-1,1])b.box([side*x+strip*width*.23,r+Math.cos(a)*(r+.006),z+Math.sin(a)*(r+.006)],[width*.43,.035,.075],c.track,[[1,0,0],radial,tangent]);
          }
          for(let j=0;j<16;j++) {
            const a=TAU*(j+.5)/16,radial=[0,Math.cos(a),Math.sin(a)],tangent=[0,-Math.sin(a),Math.cos(a)];
            b.box([side*x,r+Math.cos(a)*(r+.004),z+Math.sin(a)*(r+.004)],[width*.30,.031,.11],c.rubber,[[1,0,0],radial,tangent]);
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
      for(let i=-3;i<=3;i++)b.box([i*w*.32,1.03,front+.055],[.05,.05,.03],c.bright);
      for(const side of [-1,1]) {
        b.rod([side*w*.44,.86,front-.04],[side*w*.44,.86,front+.14],.055,c.bright,8);
        b.box([side*(w-.055),shoulder,0],[.05,.045,length-.70],c.shade);
        beveled(b,[side*(w+.06),wheeled?1.33:1.18,0],[.25,.075,length-.40],c.edge,.028);
        for(const z of [rear+.21,front-.12]) {
          b.rod([side*w*.66,.74,z],[side*w*.66,.74,z+(z>0?.16:-.16)],.063,c.bright,12);
          b.box([side*w*.77,1.08,z],[.15,.14,.07],c.shade);
          b.cylinder([side*w*.77,1.08,z],[side*w*.77,1.08,z+(z>0?.055:-.055)],.045,z>0?c.lens:c.amber,12);
          if(z>0) {
            for(const dx of [-.085,.085])b.rod([side*w*.77+dx,1.00,z+.09],[side*w*.77+dx,1.18,z+.09],.012,c.steel,6);
            b.rod([side*w*.77-.085,1.18,z+.09],[side*w*.77+.085,1.18,z+.09],.012,c.steel,6);
          }
        }
        if(modular)for(let j=0;j<6;j++) {
          const z=rear+.58+j*(length-.97)/6;
          b.box([side*(w+.065),deck-.27,z],[.17,.44,(length-1.10)/6],c.armor);
          for(const dz of [-.15,.15])b.cylinder([side*(w+.15),deck-.13,z+dz],[side*(w+.18),deck-.13,z+dz],.027,c.bright,6);
        }
      }
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
          b.rod([side*(w-.30),.87,z-.15],[side*x,r,z],hydro?.066:.044,c.steel,12);
          if(hydro)b.cylinder([side*(w-.24),.80,z-.12],[side*(x-.10),r+.10,z-.025],.104,c.bright,14);
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
          for(let j=0;j<4;j++)b.box([side*(w+.19),binY,-length*.12+(j-1.5)*length*.10],[.03,.10,.05],c.bright);
        }
        for(let j=0;j<3;j++) {
          const z=rear+.85+j*(length-1.9)/2;
          b.rod([railX,deck+.07,z-.16],[railX,deck+.16,z-.16],.017,c.steel,6);
          b.rod([railX,deck+.07,z+.16],[railX,deck+.16,z+.16],.017,c.steel,6);
          b.rod([railX,deck+.16,z-.16],[railX,deck+.16,z+.16],.017,c.steel,6);
        }
        for(const z of [rear+.60,front-.80])b.box([side*(w-.06),deck-.02,z],[.13,.09,.10],c.steel);
        for(let j=0;j<2;j++)b.rod([side*(w-.06),deck-.16-j*.14,front-1.55],[side*(w-.06),deck-.16-j*.14,front-.78],.024,j?c.steel:c.cable,8);
      });
      if(wheeled)b.part(`running gear / ${lane} wheel arches and mud flaps`,()=>{
        const R=r+.14,arc=6;
        for(let j=0;j<count;j++) {
          const z=-half+2*half*j/(count-1);
          for(let k=0;k<arc;k++) {
            const t0=Math.PI*(.16+.68*k/arc),t1=Math.PI*(.16+.68*(k+1)/arc),t=(t0+t1)/2;
            const radial=[0,Math.sin(t),Math.cos(t)],tangent=[0,Math.cos(t),-Math.sin(t)],span=R*(t1-t0)*1.08;
            const cy=r+Math.sin(t)*R,cz=z+Math.cos(t)*R;
            b.box([side*x,cy,cz],[width+.20,.07,span],weathered(c.upper,j+k),[[1,0,0],radial,tangent]);
            b.box([side*(x+width/2+.085),cy,cz],[.05,.13,span],c.edge,[[1,0,0],radial,tangent]);
          }
          // The flap hangs from the arch's trailing end, not from thin air: it has
          // to reach that exact point or it reads as a detached black rectangle.
          b.box([side*x,r*.89,z-R+.085],[width+.10,r*1.42,.035],c.rubber);
        }
      },'wheels');
      if(!wheeled)b.part(`tracks / ${lane} continuous articulated belt`,()=>{
        const radius=.53,centerY=.53,span=half,perimeter=span*4+TAU*radius,steps=Math.ceil(perimeter/.18);
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
          b.box([side*x,p.y,p.z],[width+.025,.048,perimeter/steps*.82],c.steel,[[1,0,0],radial,p.tangent]);
          b.box([side*x,p.y+radial[1]*.032,p.z+radial[2]*.032],[width*.63,.034,.07],s.tracks==='tracks_padded'?c.rubber:c.bright,[[1,0,0],radial,p.tangent]);
        }
      });
    }
    b.part(`mobility / ${label(s.mobility)} engine deck`,()=>{
      const z=front-1.12,x=w*.44,managed=s.mobility==='ground_engine_750',power=s.mobility==='engine_diesel_1200';
      b.box([x,deck+.105,z],[.66,.07,.75],c.shade);
      const vents=power?14:managed?11:s.mobility==='engine_diesel_900'?9:7;
      for(let i=0;i<vents;i++)b.box([x,deck+.151,z-.31+i*.62/(vents-1)],[.57,.022,.027],c.bright);
      // Framed grille rather than louvres painted on a lid. From above, the deck
      // is most of what a card crop shows of a low hull.
      for(const dx of [-.315,.315])b.box([x+dx,deck+.145,z],[.055,.070,.80],c.upper);
      for(const dz of [-.375,.375])b.box([x,deck+.145,z+dz],[.72,.070,.055],c.upper);
      for(const dz of [-.30,.30])b.box([x-.40,deck+.10,z+dz],[.10,.075,.10],c.steel);
      if(managed)b.box([x,deck+.17,z+.44],[.49,.14,.16],c.upper);
      for(let j=0;j<(power?2:1);j++) {
        b.cylinder([w-.14,deck-.34,front-1.18-j*.30],[w+.14,deck-.34,front-1.18-j*.30],.09,c.steel,16);
        b.tube([w+.14,deck-.34,front-1.18-j*.30],[w+.24,deck-.23,front-1.18-j*.30],.095,.066,c.steel,16);
      }
    });
    b.part('transmission / forward drive access',()=>{
      const automatic=s.transmission!=='transmission_manual',managed=s.transmission==='ground_transmission_electric';
      b.box([0,.91,front-.02],[automatic?1.0:.73,.22,automatic?.19:.12],c.shade);
      for(const x of [-.26,.26])b.box([x,.94,front+.08],[.07,.06,.045],c.bright);
      if(managed)b.box([-.43,1.075,front-.16],[.25,.16,.26],c.upper);
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
        for(let j=0;j<8;j++){const a=TAU*j/8;b.box([Math.sin(a)*.50,deck+.185,mountZ+Math.cos(a)*.50],[.07,.042,.07],c.bright);}
        for(const side of [-1,1])for(let j=0;j<3;j++)
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
          b.box([x,y,z],[.25,.25,1.50],c.shade);
          b.box([x,y,z+.765],[.215,.215,.045],c.black);
          for(const dz of [-.40,.42])b.box([x,y,z+dz],[.275,.275,.035],c.bright);
        }
      } else {
        const thick=s.armament==='ground_howitzer_155'||s.armament==='ground_gun_35';
        const extent=indirect?(thick?4.35:3.82):small?1.02:twin?2.55:thick?2.64:2.23;
        const radius=indirect?(thick?.112:.087):small?.027:twin?.040:thick?.052:.039;
        const start=mountZ+(indirect?.70:small?.12:.39),rise=indirect?.36:twin?.20:0;
        const offsets=twin?[-.46,.46]:[0];
        for(const x of offsets) {
          b.cylinder([x,gunY,start-.08],[x,gunY,start+.30],radius*2.6,c.shade,24);
          b.cylinder([x,gunY,start+.25],[x,gunY+rise,start+extent-.22],radius*1.18,c.upper,24,radius);
          b.tube([x,gunY+rise,start+extent-.23],[x,gunY+rise,start+extent],radius*1.08,radius*.68,c.steel,24);
          if(indirect) {
            b.cylinder([x,gunY+rise*.53,start+extent*.50],[x,gunY+rise*.62,start+extent*.60],radius*1.53,c.shade,24);
            b.box([x,gunY+rise,start+extent-.065],[radius*3.6,radius*2.6,.25],c.steel);
            for(const side of [-1,1])b.box([x+side*radius*1.82,gunY+rise,start+extent-.05],[.009,radius*1.2,.13],c.black);
            b.rod([x+.19,gunY+.13,start],[x+.19,gunY+.13,start+.91],.060,c.steel,16);
          } else if(small) {
            b.box([.12,gunY-.05,start+.12],[.13,.13,.28],c.steel);
            b.box([0,gunY,start+.10],[.105,.095,.42],c.black);
          } else for(let j=0;j<4;j++)b.cylinder([x,gunY+rise*j/4,start+.50+j*.16],[x,gunY+rise*j/4,start+.535+j*.16],radius*1.45,c.steel,16);
        }
      }
    });
    b.part(`ammunition / ${label(s.ammunition)} protected stowage`,()=>{
      // Exterior lockers identify the ammunition bay; payload quantities remain
      // simulation metadata, not invented visible rounds or performance claims.
      const x=howitzer?.91:mg?.27:.55,z=mountZ-(howitzer?1.09:mg?.18:.53);
      b.box([x,mg?mountTop-.09:deck+.40,z],[.30,.27,.40],c.shade);
      b.box([x,mg?mountTop+.052:deck+.542,z],[.32,.025,.42],c.edge);
      b.rod([x-.06,mg?mountTop+.078:deck+.565,z],[x+.06,mg?mountTop+.078:deck+.565,z],.014,c.bright);
    });
    b.part(`sensors / ${label(s.sensors)} observation fittings`,()=>{
      hatch(-w*.42,deck+.065,front-1.12,.25);
      for(let j=-1;j<=1;j++)b.box([-w*.42+j*.16,deck+.19,front-.90],[.13,.085,.065],c.glass);
      // Coaming and guard bar over the driver's blocks. From above they are the
      // only fitting that says which end of a low flat hull is the front.
      b.box([-w*.42,deck+.135,front-.90],[.52,.055,.11],c.shade);
      for(const dx of [-.25,.25])b.rod([-w*.42+dx,deck+.24,front-.86],[-w*.42+dx,deck+.30,front-.86],.016,c.steel,6);
      b.rod([-w*.42-.25,deck+.30,front-.86],[-w*.42+.25,deck+.30,front-.86],.016,c.steel,6);
      const night=s.sensors==='optics_night',thermal=s.sensors==='optics_thermal',y=mg?deck+.17:mountTop;
      b.box([.32,y+.075,mountZ+.15],[thermal?.28:.21,thermal?.16:.12,.24],c.shade);
      b.box([.32,y+.077,mountZ+.276],[thermal?.19:.13,.075,.016],c.lens);
      if(night)b.cylinder([.40,y+.09,mountZ+.20],[.40,y+.09,mountZ+.37],.083,c.black,20);
      if(thermal){b.cylinder([.32,y+.15,mountZ+.15],[.32,y+.38,mountZ+.15],.085,c.steel,20);b.box([.32,y+.44,mountZ+.15],[.32,.15,.24],c.upper);b.box([.32,y+.44,mountZ+.278],[.22,.09,.02],c.lens);}
    });
    b.part(`fire_control / ${label(s.fire_control)} sight and stabilization`,()=>{
      const digital=s.fire_control==='fcs_digital',stabilized=s.fire_control==='fcs_stabilized';
      const x=mg?-.30:-.40,y=mg?mountTop-.12:mountTop-.10;
      b.box([x,y,mountZ+.30],[.14,digital?.19:.10,.24],c.shade);
      b.box([x,y,mountZ+.43],[.095,.065,.018],c.glass);
      if(stabilized||digital)b.rod([x,gunY-.12,mountZ+.35],[x,gunY-.12,mountZ+.82],.038,c.bright,12);
      if(digital)b.box([x-.18,y+.06,mountZ+.22],[.18,.13,.24],c.upper);
    });
    b.part(`communications / ${label(s.communications)} aerial installation`,()=>{
      const network=s.communications==='ground_comms_network',secure=s.communications==='ground_comms_secure',data=s.communications==='comms_data';
      for(const side of network||data?[-1,1]:[-1]) {
        const x=side*(w-.35),z=rear+.54;
        b.cylinder([x,deck,z],[x,deck+.14,z],.075,c.black,16);
        b.rod([x,deck+.14,z],[x,deck+(network?1.31:1.01),z-.08],.013,c.steel,8);
      }
      if(secure||network)b.box([w-.43,deck+.14,rear+.65],[.32,.19,.37],c.shade);
      if(data||network)b.cylinder([w-.43,deck+.20,rear+.65],[w-.43,deck+.32,rear+.65],.19,c.upper,24);
    });
    if(s.active_protection!=='aps_none')b.part(`active_protection / ${label(s.active_protection)} perimeter system`,()=>{
      for(const side of [-1,1]) {
        const x=side*(w-.20),z=front-.82;
        b.box([x,deck+.09,z],[.16,.17,.18],c.shade);
        b.box([x,deck+.09,z+.095],[.10,.095,.015],c.lens);
        const count=s.active_protection==='aps_hard'?4:2;
        for(let j=0;j<count;j++)b.cylinder([x,deck+.06,z-.30-j*.095],[x+side*.13,deck+.24,z-.21-j*.095],.041,c.steel,12);
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
      for(let j=0;j<2;j++)b.rod([-w*.55,deck+height+.05,rear+.30+j*.44],[w*.55,deck+height+.05,rear+.30+j*.44],.018,c.steel,6);
      for(let j=0;j<5;j++)b.rod([(j-2)*w*.275,deck+height+.05,rear+.28],[(j-2)*w*.275,deck+height+.05,rear+.76],.014,c.steel,6);
      beveled(b,[0,deck+height+.17,rear+.52],[w*.66,.22,.40],c.canvas,.055);
      for(const side of [-1,1]) {
        hatch(side*w*.37,deck+height,rear+1.01,.25);
        for(let j=0;j<3;j++)b.box([side*(w-.09),deck-.17,rear+.48+j*.43],[.045,.075,.14],c.glass);
      }
      // The ramp is the entire aft silhouette on a carrier and it was one flat
      // panel with five ribs. Personnel door, hinges, actuators and lights give
      // the rear view something to resolve into at card size.
      beveled(b,[0,deck-.46,rear-.035],[w*1.18,.90,.085],protectedBay?c.armor:c.shade,.07);
      for(const x of [-w*.50,w*.50])b.rod([x,.79,rear-.092],[x,deck-.16,rear-.092],protectedBay?.038:.023,c.bright);
      for(let j=0;j<5;j++)b.box([0,.81+j*.135,rear-.085],[w*.90,.020,.025],c.steel);
      b.box([0,deck-.27,rear-.10],[.17,.055,.035],c.bright);
      b.box([w*.32,deck-.46,rear-.090],[.44,.72,.028],c.edge);
      b.box([w*.32,deck-.25,rear-.106],[.19,.12,.016],c.glass);
      b.box([w*.32+.18,deck-.46,rear-.108],[.05,.10,.042],c.bright);
      for(const side of [-1,1]) {
        b.rod([side*w*.63,.84,rear+.03],[side*w*.63,.84,rear-.10],.052,c.steel,8);
        b.cylinder([side*w*.74,deck-.66,rear+.18],[side*w*.66,deck-.28,rear-.04],.050,c.bright,10);
        b.box([side*w*.82,deck-.09,rear-.02],[.16,.15,.09],c.shade);
        b.box([side*w*.82,deck-.09,rear-.070],[.10,.09,.012],side>0?c.amber:c.lens);
      }
      b.box([0,.63,rear-.13],[w*.66,.070,.18],c.steel);
    });
    if(s.recon_package)b.part(`recon_package / ${s.recon_package==='ground_recon_mast'?'elevated observation mast':'scout observation station'}`,()=>{
      const elevated=s.recon_package==='ground_recon_mast',z=rear+.94,y=deck+(elevated?1.72:.39);
      b.cylinder([.12,deck,z],[.12,deck+.21,z],.20,c.shade,24);
      b.cylinder([.12,deck+.19,z],[.12,y-.15,z],elevated?.06:.095,c.bright,20,elevated?.036:.095);
      if(elevated)b.cylinder([.12,deck+.26,z],[.12,deck+.77,z],.095,c.steel,20);
      b.box([.12,y,z],[elevated?.46:.34,.25,.32],c.upper);
      for(const x of [-.01,.22])b.box([x,y,z+.168],[.13,.15,.016],c.lens);
      hatch(-.39,deck+.065,z-.25,.25);
    });
    if(s.artillery_loader)b.part(`artillery_loader / ${s.artillery_loader==='ground_loader_assisted'?'assisted loading installation':'manual loading access'}`,()=>{
      const assisted=s.artillery_loader==='ground_loader_assisted',z=mountZ-1.27;
      b.box([0,deck+.65,z],[assisted?1.35:.86,assisted?.71:.56,assisted?.49:.13],c.shade);
      b.box([0,deck+.65,z-(assisted?.26:.08)],[.56,.40,.025],c.upper);
      for(const side of [-1,1]) {
        b.rod([side*.42,deck+.41,z-.12],[side*.42,deck+.93,z-.12],.024,c.bright);
        b.rod([side*.60,.64,rear-.04],[side*.74,.26,rear-.27],.066,c.steel);
        b.box([side*.74,.24,rear-.27],[.34,.085,.26],c.shade);
      }
      if(assisted)for(let j=0;j<4;j++)b.box([0,deck+.37,z-.28-j*.10],[.59,.045,.052],c.bright);
    });
    if(s.radar)b.part(`radar / ${s.radar==='ground_radar_tracking'?'search and tracking array':'rotating search array'}`,()=>{
      const tracking=s.radar==='ground_radar_tracking',z=mountZ-.43,y=mountTop+.93;
      b.cylinder([0,mountTop,z],[0,y-.16,z],.11,c.steel,24);
      b.cylinder([0,mountTop+.03,z],[0,mountTop+.18,z],.28,c.shade,24);
      b.box([0,y,z],[tracking?1.24:1.02,tracking?.55:.38,.14],c.upper);
      b.box([0,y,z+.079],[tracking?1.13:.91,tracking?.44:.27,.024],c.glass);
      for(let i=-4;i<=4;i++)b.box([i*.105,y,z+.095],[.014,tracking?.44:.27,.012],c.shade);
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
    if(Object.hasOwn(GROUND_DEFAULTS,spec.platform))return buildGround(spec);
    const mobile = ["drive_mobile","engine_diesel_1200","engine_turbine_1500"].includes(chosen.mobility), reinforced = chosen.protection === "protection_heavy", active = chosen.protection === "protection_active" || chosen.active_protection === "aps_hard";
    const heavyGun = ["armament_heavy","gun_120","gun_125"].includes(chosen.armament), integrated = ["sensors_integrated","optics_thermal"].includes(chosen.sensors), data = chosen.communications === "comms_data";
    const compact=chosen.turret==='turret_compact',casemate=chosen.turret==='turret_casemate',autoload=chosen.turret==='turret_autoload',large=chosen.turret==='turret_heavy';
    const b = createBuilder(), c = PALETTE;
    const hullWidth = heavy ? 1.64 : 1.43, rear = heavy ? -3.32 : -2.91, front = heavy ? 3.04 : 2.76;
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
      for (let i = -3; i <= 3; i++) b.box([i * hullWidth * 0.30, 0.99, front + 0.125], [0.055, 0.055, 0.032], c.bright);
      beveled(b, [0, 1.06, rear - 0.07], [hullWidth * 1.60, 0.60, 0.11], c.upper, 0.07);
      for (let i = -4; i <= 4; i++) b.box([i * hullWidth * 0.21, 1.32, rear - 0.13], [0.05, 0.05, 0.028], c.bright);
      for (const x of [-hullWidth * 0.72, hullWidth * 0.72]) {
        b.rod([x, 0.78, front - 0.04], [x, 0.78, front + 0.19], 0.095, c.bright, 12);
        b.tube([x, 0.80, front + 0.16], [x, 0.80, front + 0.23], 0.115, 0.063, c.steel, 16);
        b.tube([x, 0.79, rear - 0.15], [x, 0.79, rear - 0.05], 0.11, 0.060, c.steel, 16);
      }
    });

    b.part("chassis / glacis applique, splash guard and spare track links", () => {
      // Applique is spaced off the glacis on purpose — flush plate would just be
      // a colour change. The basis is the measured glacis slope so the plates lie
      // on the surface rather than floating at their own angle.
      const up = [0, 0.7407, -0.6745], out = [0, 0.6745, 0.7407], slope = [[1, 0, 0], up, out];
      for (let i = 0; i < 5; i++) {
        const x = (i - 2) * hullWidth * 0.38;
        beveled(b, [x, 1.234, front - 0.131], [hullWidth * 0.35, 0.42, 0.09], weathered(c.armor, i), 0.06, slope);
      }
      // Spare links live on the glacis on real vehicles because that is the face
      // that gets hit; here they also break the largest flat plane on the model.
      for (let i = 0; i < 4; i++) beveled(b, [(i - 1.5) * 0.40, 1.02, front - 0.06], [0.34, 0.13, 0.10], c.steel, 0.03, slope);
      b.box([0, 1.515, front - 0.455], [hullWidth * 1.70, 0.05, 0.17], c.edge);
      for (const x of [-hullWidth * 0.55, hullWidth * 0.55]) {
        b.rod([x, 1.05, front + 0.10], [x, 1.16, front + 0.06], 0.036, c.bright, 8);
        b.rod([x, 1.42, front - 0.34], [x, 1.53, front - 0.44], 0.030, c.steel, 8);
      }
    });

    for (const side of [-1, 1]) {
      const label = side < 0 ? "port" : "starboard", x = side * trackX;
      b.part(`running gear / ${label} continuous track belt`, () => {
        const steps = heavy ? 112 : 104;
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
        const count = heavy ? 92 : 84, step = pathLength / count;
        for (let i = 0; i < count; i++) {
          const p = trackPoint(i * step, trackRadius - 0.033), tangent = p.tangent, outward = [0, -tangent[2], tangent[1]];
          const basis = [[1, 0, 0], outward, tangent], center = [x, p.y, p.z];
          b.box(center, [trackWidth + 0.035, 0.045, step * 0.88], shade(c.track, 0.95 + (i % 5) * 0.018), basis);
          b.box(add(center, mul(outward, 0.034)), [trackWidth * (chosen.tracks==='tracks_padded'?.90:.71), chosen.tracks==='tracks_padded'?.06:.027, step * 0.65], c.rubber, basis);
          b.box(add(center, mul(outward, 0.032)), [trackWidth + 0.070, 0.023, step * 0.14], c.bright, basis);
          for (const rim of [-1, 1]) {
            const pinCenter = add(center, mul(tangent, step * 0.40));
            b.cylinder(add(pinCenter, [rim * (trackWidth / 2 - 0.025), 0, 0]), add(pinCenter, [rim * (trackWidth / 2 + 0.050), 0, 0]), 0.025, c.bright, 8);
          }
          if (i % 2 === 0) b.box(add(center, mul(outward, -0.095)), [0.070, 0.14, step * 0.33], c.steel, basis);
        }
      });
      b.part(`running gear / ${label} suspension, road wheels and sprockets`, () => {
        const wheels = heavy ? 7 : 6, first = -wheelHalfSpan + 0.62, last = wheelHalfSpan - 0.62;
        for (let i = 0; i < wheels; i++) {
          const z = first + (last - first) * i / (wheels - 1), wheelY = 0.48, wheelRadius = 0.335;
          b.rod([side * (hullWidth - 0.14), 0.90, z - 0.24], [x, wheelY, z], 0.085, c.steel, 12);
          // Bump stop above each arm. It is the fitting that tells the eye the
          // arm swings, and it survives the shrink to card size as a shadow.
          b.box([side * (hullWidth - 0.05), 1.02, z - 0.19], [0.17, 0.13, 0.16], c.shade);
          if(chosen.suspension==='suspension_hydro') {b.cylinder([x+side*.25,.86,z-.18],[x+side*.25,.53,z],.065,c.bright,12);b.cylinder([x+side*.25,.99,z-.25],[x+side*.25,.76,z-.13],.10,c.shade,12);}
          b.cylinder([x - 0.27, wheelY, z], [x + 0.27, wheelY, z], wheelRadius, c.rubber, 36);
          for (const rim of [-1, 1]) {
            const near = x + rim * 0.273, out = x + rim * 0.301;
            b.cylinder([near, wheelY, z], [out, wheelY, z], wheelRadius * 0.81, c.upper, 32, wheelRadius * 0.76);
            b.cylinder([out, wheelY, z], [out + rim * 0.027, wheelY, z], 0.148, c.shade, 24, 0.115);
            b.cylinder([out + rim * 0.026, wheelY, z], [out + rim * 0.043, wheelY, z], 0.064, c.bright, 12);
            for (let bolt = 0; bolt < 8; bolt++) {
              const angle = TAU * bolt / 8, by = wheelY + Math.cos(angle) * 0.208, bz = z + Math.sin(angle) * 0.208;
              b.cylinder([out, by, bz], [out + rim * 0.017, by, bz], 0.021, c.bright, 6);
            }
          }
        }
        // Drive sprocket forward, idler aft. Toothed wheels at both ends is the
        // tell that one track end was drawn twice rather than designed, and the
        // spoked idler web gives the rear of the running gear its own read.
        for (const end of [-1, 1]) {
          const z = end * wheelHalfSpan, y = trackCenterY + 0.21, r = 0.36;
          b.cylinder([x - 0.28, y, z], [x + 0.28, y, z], r, c.steel, 28);
          b.cylinder([x + side * 0.281, y, z], [x + side * 0.326, y, z], r * 0.75, c.hull, 28);
          b.cylinder([x + side * 0.325, y, z], [x + side * 0.355, y, z], 0.13, c.bright, 16);
          if (end > 0) for (let tooth = 0; tooth < 14; tooth++) {
            const angle = TAU * tooth / 14, radial = [0, Math.sin(angle), Math.cos(angle)], tangent = [0, Math.cos(angle), -Math.sin(angle)];
            b.box([x, y + radial[1] * r, z + radial[2] * r], [0.49, 0.075, 0.10], c.bright, [[1, 0, 0], radial, tangent]);
          } else {
            b.cylinder([x + side * 0.283, y, z], [x + side * 0.302, y, z], r * 0.98, c.steel, 28);
            for (let spoke = 0; spoke < 8; spoke++) {
              const angle = TAU * spoke / 8, radial = [0, Math.sin(angle), Math.cos(angle)], tangent = [0, Math.cos(angle), -Math.sin(angle)];
              b.box([x + side * 0.316, y + radial[1] * r * 0.50, z + radial[2] * r * 0.50], [0.030, r * 0.60, 0.075], c.shade, [[1, 0, 0], radial, tangent]);
            }
            b.box([side * (hullWidth - 0.02), 1.02, z + end * 0.22], [0.34, 0.15, 0.36], c.shade);
          }
        }
        for (let i = 0; i < 3; i++) {
          const z = -wheelHalfSpan * 0.66 + i * wheelHalfSpan * 0.66;
          b.cylinder([x - 0.18, 1.13, z], [x + 0.18, 1.13, z], 0.135, c.rubber, 16);
          b.box([side * (hullWidth - 0.03), 1.16, z], [0.30, 0.10, 0.13], c.steel);
        }
      });
      b.part(`chassis / ${label} fenders, segmented skirts and fixtures`, () => {
        b.box([x, 1.43, -0.08], [trackWidth + 0.11, 0.085, (wheelHalfSpan + 0.54) * 2], c.shade);
        const outerX = x + side * (trackWidth / 2 + 0.055), panels = heavy ? 7 : 6;
        for (let i = 0; i < panels; i++) {
          const z = -wheelHalfSpan + (i + 0.5) * (wheelHalfSpan * 2 / panels), panelLength = wheelHalfSpan * 2 / panels - 0.035;
          beveled(b, [outerX, 1.23, z], [0.065, reinforced ? 0.55 : 0.36, panelLength], weathered(i % 2 ? c.hull : c.armor, i), 0.022);
          b.box([outerX + side * 0.039, 1.40, z], [0.018, 0.045, panelLength * 0.76], c.edge);
          for (const dz of [-panelLength * 0.34, panelLength * 0.34]) b.cylinder([outerX + side * 0.030, 1.33, z + dz], [outerX + side * 0.048, 1.33, z + dz], 0.025, c.bright, 6);
          // Hinge lugs at the top of every panel: skirts swing up for track work,
          // and without the lugs the row reads as one painted stripe.
          for (const dz of [-panelLength * 0.37, panelLength * 0.37]) b.box([outerX - side * 0.026, 1.418, z + dz], [0.052, 0.075, 0.052], c.steel);
        }
        for (const end of [-1, 1]) b.box([x, 1.08, end * (wheelHalfSpan + 0.55)], [trackWidth + 0.06, 0.57, 0.046], c.rubber);
        b.box([side * (hullWidth - 0.16), 1.53, front - 0.48], [0.25, 0.15, 0.21], c.shade);
        b.box([side * (hullWidth - 0.16), 1.55, front - 0.37], [0.16, 0.078, 0.012], c.amber);
        for (const dx of [-0.12, 0.12]) b.rod([side * (hullWidth - 0.16) + dx, 1.53, front - 0.35], [side * (hullWidth - 0.16) + dx, 1.72, front - 0.35], 0.012, c.steel, 6);
        b.rod([side * (hullWidth - 0.16) - 0.12, 1.72, front - 0.35], [side * (hullWidth - 0.16) + 0.12, 1.72, front - 0.35], 0.012, c.steel, 6);
      });
      b.part(`chassis / ${label} sponson stowage bins and tool stowage`, () => {
        // Bins ride the fender rather than the hull side, because the hull side is
        // armour. They are also the layer that stops the flank reading as one long
        // green plate between the skirt line and the turret.
        const top = 1.4725;
        for (let i = 0; i < 3; i++) {
          const z = (i - 1) * wheelHalfSpan * 0.70, length = i === 1 ? 1.06 : 0.88;
          beveled(b, [x, top + 0.17, z], [trackWidth + 0.03, 0.33, length], weathered(c.hull, i + (side > 0 ? 1 : 0)), 0.06);
          b.box([x, top + 0.345, z], [trackWidth + 0.06, 0.026, length + 0.03], c.edge);
          for (const dz of [-length * 0.30, length * 0.30]) b.box([x + side * (trackWidth / 2 + 0.005), top + 0.20, z + dz], [0.030, 0.090, 0.055], c.bright);
          b.rod([x - 0.15, top + 0.372, z - length * 0.35], [x + 0.15, top + 0.372, z - length * 0.35], 0.016, c.steel, 6);
        }
        // Pioneer tools clamped outboard: two long thin runs that catch the light
        // along the whole flank for the price of two rods.
        for (let i = 0; i < 2; i++) b.rod([x + side * (trackWidth / 2 + 0.055), top + 0.11 + i * 0.115, -0.62], [x + side * (trackWidth / 2 + 0.055), top + 0.11 + i * 0.115, 0.66], 0.027, i ? c.steel : c.cable, 8);
      });
    }

    b.part("chassis / driver hatch, periscopes and glacis detail", () => {
      b.loft([octagon(1.545, 0.37, front - 1.27, front - 0.70, 0.10), octagon(1.60, 0.33, front - 1.24, front - 0.73, 0.09)], c.edge);
      for (const x of [-0.26, 0, 0.26]) {
        b.box([x, 1.64, front - 0.77], [0.19, 0.12, 0.12], c.shade);
        b.box([x, 1.65, front - 0.705], [0.14, 0.053, 0.012], c.glass);
      }
      b.rod([-0.12, 1.65, front - 1.10], [-0.12, 1.70, front - 1.10], 0.019, c.steel);
      b.rod([0.12, 1.65, front - 1.10], [0.12, 1.70, front - 1.10], 0.019, c.steel);
      b.rod([-0.12, 1.70, front - 1.10], [0.12, 1.70, front - 1.10], 0.019, c.steel);
      for (const x of [-0.78, 0.78]) {
        b.rod([x, 1.40, front - 0.20], [x, 1.23, front + 0.005], 0.025, c.bright);
        for (let i = 0; i < 5; i++) b.box([x + (i - 2) * 0.11, 1.487, front - 0.73], [0.063, 0.018, 0.11], c.shade);
      }
    });

    b.part(`mobility / ${mobile ? "high-output powertrain exhaust and cooling" : "standard powertrain exhaust and cooling"}`, () => {
      const engineRear = rear + 0.37, engineFront = -1.51, mid = (engineRear + engineFront) / 2;
      b.box([0, 1.563, mid], [hullWidth * 1.55, 0.045, engineFront - engineRear], c.shade);
      const louvres = mobile ? 18 : 12;
      for (let i = 0; i < louvres; i++) {
        const z = engineRear + 0.06 + (engineFront - engineRear - 0.12) * i / (louvres - 1);
        for (const x of [-hullWidth * 0.40, hullWidth * 0.40]) b.box([x, 1.608, z], [hullWidth * 0.67, 0.040, mobile ? 0.032 : 0.045], c.edge);
      }
      // Raised frames around each louvre bank. Bare stripes on a flat lid read as
      // paint from above, and the deck is most of the plan view on a card.
      for (const x of [-hullWidth * 0.40, hullWidth * 0.40]) {
        for (const dx of [-hullWidth * 0.365, hullWidth * 0.365]) b.box([x + dx, 1.601, mid], [0.055, 0.078, engineFront - engineRear - 0.02], c.upper);
        for (const dz of [engineRear + 0.028, engineFront - 0.028]) b.box([x, 1.601, dz], [hullWidth * 0.79, 0.078, 0.055], c.upper);
      }
      // Bolted powerpack access panel forward of the grilles, clear of the ring.
      beveled(b, [0, 1.585, engineFront + 0.30], [0.86, 0.075, 0.44], c.edge, 0.07);
      for (const dx of [-0.24, 0.24]) b.box([dx, 1.632, engineFront + 0.30], [0.072, 0.048, 0.28], c.bright);
      // Pintle and light clusters. The rear plate is the second view a card crop
      // shows after the three-quarter, and it was blank.
      beveled(b, [0, 1.02, rear - 0.20], [0.34, 0.26, 0.22], c.steel, 0.05);
      b.tube([0, 1.02, rear - 0.30], [0, 1.02, rear - 0.255], 0.105, 0.058, c.bright, 12);
      for (const side of [-1, 1]) {
        b.box([side * (hullWidth - 0.42), 1.40, rear - 0.10], [0.20, 0.17, 0.10], c.shade);
        b.box([side * (hullWidth - 0.42), 1.40, rear - 0.152], [0.13, 0.10, 0.012], side > 0 ? c.amber : c.lens);
      }
      if (mobile) {
        for (const x of [-0.60, 0.60]) {
          b.cylinder([x, 1.57, rear + 0.79], [x, 1.69, rear + 0.79], 0.32, c.steel, 32);
          for (let stripe = -3; stripe <= 3; stripe++) {
            const z = rear + 0.79 + stripe * 0.074, length = 2 * Math.sqrt(0.29 * 0.29 - Math.pow(stripe * 0.074, 2));
            b.box([x, 1.703, z], [length, 0.026, 0.025], c.edge);
          }
        }
        for (const x of [-0.67, 0.67]) {
          b.box([x, 1.16, rear - 0.08], [0.61, 0.31, 0.25], c.steel);
          b.box([x, 1.16, rear - 0.213], [0.48, 0.21, 0.018], c.black);
          for (let i = -2; i <= 2; i++) b.box([x, 1.16 + i * 0.040, rear - 0.227], [0.50, 0.014, 0.030], c.bright);
        }
      } else {
        for (const x of [-0.74, 0.74]) {
          b.cylinder([x, 1.11, rear - 0.04], [x, 1.11, rear - 0.26], 0.19, c.steel, 24);
          b.tube([x, 1.11, rear - 0.13], [x, 1.11, rear - 0.31], 0.165, 0.125, c.bright, 24);
          b.cylinder([x, 1.11, rear - 0.08], [x, 1.11, rear - 0.14], 0.12, c.black, 20);
        }
      }
      for (const x of [-hullWidth + 0.18, hullWidth - 0.18]) {
        b.box([x, 1.635, rear + 0.72], [0.21, 0.18, 0.75], c.hull);
        for (const z of [rear + 0.41, rear + 1.01]) b.box([x, 1.733, z], [0.22, 0.02, 0.053], c.bright);
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
        for (let i = 0; i < 12; i++) {
          const angle = TAU * i / 12;
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
      }
      beveled(b, [0, turretY + 0.41, -1.59], [turretWidth * 1.56, 0.38, 0.27], c.shade, 0.07);
      for (const x of [-turretWidth * 0.57, 0, turretWidth * 0.57]) b.box([x, turretY + 0.56, -1.739], [0.27, 0.09, 0.024], c.steel);
    });

    b.part(`armament / ${heavyGun ? "heavy weapon, enlarged mantlet and sleeved barrel" : "standard weapon, mantlet and barrel"}`, () => {
      const gunY = turretY + 0.49, radius = chosen.armament==='gun_90'?.085:chosen.armament==='gun_125'?.15:heavyGun ? 0.132 : 0.105, muzzle = chosen.armament==='gun_90'?4.78:chosen.armament==='gun_125'?6.67:heavyGun ? 6.36 : 5.68;
      beveled(b, [0, gunY, 1.20], [heavyGun ? 0.90 : 0.75, heavyGun ? 0.68 : 0.55, 0.61], c.shade, 0.10);
      b.cylinder([-0.45, gunY, 1.45], [0.45, gunY, 1.45], heavyGun ? 0.35 : 0.29, c.armor, 32);
      b.cylinder([0, gunY, 1.46], [0, gunY, 1.99], radius * 1.85, c.steel, 32, radius * 1.34);
      // Canvas boot over the mantlet gap. It is the only soft material on the
      // model, and it stops the barrel reading as a rod pushed into a box.
      b.cylinder([0, gunY, 1.63], [0, gunY, 1.94], radius * 2.42, c.canvas, 16, radius * 1.54);
      for (const [z, scale] of [[1.66, 2.44], [1.89, 1.70]]) b.cylinder([0, gunY, z], [0, gunY, z + 0.028], radius * scale, c.steel, 16);
      b.cylinder([0, gunY, 1.94], [0, gunY, muzzle - 0.23], radius * 1.10, c.upper, 36, radius);
      b.cylinder([0, gunY, 3.30], [0, gunY, 3.87], radius * (heavyGun ? 1.60 : 1.45), c.shade, 36, radius * 1.28);
      const sleeveCount = heavyGun ? 7 : 4;
      for (let i = 0; i < sleeveCount; i++) {
        const z = 2.31 + (muzzle - 2.80) * i / (sleeveCount - 1);
        b.cylinder([0, gunY, z], [0, gunY, z + 0.063], radius * 1.19, c.bright, 32);
      }
      b.tube([0, gunY, muzzle - 0.42], [0, gunY, muzzle], radius * 1.06, radius * 0.74, c.steel, 40);
      b.cylinder([0, gunY, muzzle - 0.455], [0, gunY, muzzle - 0.425], radius * 0.73, c.black, 32);
      b.box([0, gunY + radius * 1.12, muzzle - 0.21], [0.088, 0.052, 0.18], c.shade);
      b.box([0.37, gunY - 0.02, 1.54], [0.14, 0.13, 0.12], c.black);
      b.tube([0.37, gunY - 0.02, 1.55], [0.37, gunY - 0.02, 1.76], 0.041, 0.025, c.steel, 16);
    });

    b.part("turret / crew hatches, cupola and fittings", () => {
      for (const hatch of [{ x: -0.48, z: -0.36, radius: 0.35 }, { x: 0.48, z: -0.24, radius: 0.30 }]) {
        b.cylinder([hatch.x, turretTop - 0.018, hatch.z], [hatch.x, turretTop + 0.13, hatch.z], hatch.radius, c.shade, 32);
        b.cylinder([hatch.x, turretTop + 0.129, hatch.z], [hatch.x, turretTop + 0.19, hatch.z], hatch.radius * 0.96, c.edge, 32, hatch.radius * 0.91);
        b.box([hatch.x, turretTop + 0.205, hatch.z - 0.02], [0.20, 0.043, 0.039], c.steel);
        for (const dx of [-0.15, 0.15]) b.box([hatch.x + dx, turretTop + 0.13, hatch.z - hatch.radius], [0.065, 0.085, 0.105], c.bright);
        for (let i = 0; i < 5; i++) {
          const angle = -0.84 + i * 0.42, x = hatch.x + Math.sin(angle) * (hatch.radius + 0.02), z = hatch.z + Math.cos(angle) * (hatch.radius + 0.02);
          b.box([x, turretTop + 0.065, z], [0.085, 0.055, 0.060], c.glass);
        }
      }
      // Pintle machine gun beside the commander's hatch. At card size this is the
      // single detail that separates a turret from a smooth casting; it is a crew
      // fitting, not the platform's armament, and carries no game capability.
      const pintle = [-0.18, turretTop, -0.20];
      b.cylinder([pintle[0], pintle[1] + 0.18, pintle[2]], [pintle[0], pintle[1] + 0.44, pintle[2]], 0.045, c.steel, 12);
      b.cylinder([pintle[0], pintle[1] + 0.44, pintle[2]], [pintle[0], pintle[1] + 0.50, pintle[2]], 0.075, c.shade, 12);
      b.box([pintle[0], pintle[1] + 0.545, pintle[2] + 0.16], [0.12, 0.14, 0.32], c.shade);
      b.box([pintle[0], pintle[1] + 0.46, pintle[2] + 0.10], [0.19, 0.16, 0.22], c.black);
      b.cylinder([pintle[0], pintle[1] + 0.565, pintle[2] + 0.30], [pintle[0], pintle[1] + 0.565, pintle[2] + 0.76], 0.032, c.steel, 12);
      b.box([pintle[0], pintle[1] + 0.625, pintle[2] + 0.40], [0.048, 0.042, 0.28], c.bright);
      b.box([pintle[0] + 0.15, pintle[1] + 0.50, pintle[2] + 0.08], [0.17, 0.20, 0.20], c.armor);
      for (const side of [-1, 1]) {
        const x = side * turretWidth * 0.79;
        b.rod([x, turretTop - 0.12, -0.55], [x, turretTop + 0.035, -0.55], 0.022, c.steel);
        b.rod([x, turretTop - 0.12, 0.16], [x, turretTop + 0.035, 0.16], 0.022, c.steel);
        b.rod([x, turretTop + 0.035, -0.55], [x, turretTop + 0.035, 0.16], 0.022, c.steel);
        for (let i = 0; i < 5; i++) {
          const z = -0.44 + i * 0.20;
          b.cylinder([side * (turretWidth + 0.035), turretY + 0.35, z], [side * (turretWidth + 0.047), turretY + 0.35, z], 0.024, c.bright, 6);
        }
        for (let tube = 0; tube < 4; tube++) {
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
        b.cylinder([0.65, turretTop - 0.24, 1.002], [0.65, turretTop - 0.24, 1.17], 0.090, c.steel, 20);
        b.cylinder([0.65, turretTop - 0.24, 1.171], [0.65, turretTop - 0.24, 1.177], 0.069, c.glass, 20);
      }
    });

    b.part(`communications / ${data ? "tactical data aerials and terminal" : "field radio whip aerial"}`, () => {
      const aerials = data ? [{ x: -0.83, z: -0.92, height: 1.30 }, { x: 0.84, z: -1.01, height: 0.92 }] : [{ x: 0.75, z: -0.97, height: 1.45 }];
      for (const antenna of aerials) {
        const bottom = turretTop - 0.05;
        b.cylinder([antenna.x, bottom, antenna.z], [antenna.x, bottom + 0.11, antenna.z], 0.084, c.steel, 16);
        b.cylinder([antenna.x, bottom + 0.11, antenna.z], [antenna.x, bottom + 0.26, antenna.z], 0.040, c.rubber, 16, 0.029);
        b.cylinder([antenna.x, bottom + 0.25, antenna.z], [antenna.x + 0.045, bottom + antenna.height, antenna.z - 0.12], 0.013, c.steel, 8, 0.007);
        for (let turn = 0; turn < 4; turn++) b.cylinder([antenna.x, bottom + 0.115 + turn * 0.027, antenna.z], [antenna.x, bottom + 0.130 + turn * 0.027, antenna.z], 0.046, c.bright, 12);
      }
      if (data) {
        b.box([0.27, turretTop + 0.045, -0.96], [0.35, 0.13, 0.30], c.shade);
        b.cylinder([0.27, turretTop + 0.10, -0.96], [0.27, turretTop + 0.18, -0.96], 0.175, c.edge, 24, 0.14);
        b.box([turretWidth - 0.12, turretY + 0.59, -0.93], [0.18, 0.22, 0.49], c.hull);
        b.rod([0.82, turretTop - 0.10, -1.0], [0.68, turretTop + 0.015, -0.81], 0.018, c.cable);
      }
    });

    b.part(`protection / ${reinforced ? "reinforced modular armor blocks" : active ? "active-protection sensors and intercept modules" : "standard armor fixtures"}`, () => {
      if (reinforced) {
        for (const side of [-1, 1]) {
          for (let row = 0; row < 2; row++) for (let i = 0; i < 4; i++) {
            const z = -0.92 + i * 0.47, y = turretY + 0.34 + row * 0.235, x = side * (turretWidth + 0.05 - row * 0.075);
            b.box([x, y, z], [0.23, 0.21, 0.40], i % 2 ? c.armor : c.hull);
            b.box([x + side * 0.122, y, z], [0.018, 0.13, 0.30], c.edge);
          }
          const cheekX = side * 0.89;
          for (let i = 0; i < 3; i++) {
            const y = turretY + 0.28 + i * 0.17, z = 1.50 - i * 0.16;
            b.box([cheekX, y, z], [0.65, 0.14, 0.18], c.armor);
          }
          for (let i = 0; i < 6; i++) {
            const z = -wheelHalfSpan + 0.42 + i * (wheelHalfSpan * 2 - 0.84) / 5;
            b.box([side * (trackX + trackWidth / 2 + 0.13), 1.18, z], [0.12, 0.43, 0.61], c.armor);
            b.box([side * (trackX + trackWidth / 2 + 0.198), 1.18, z], [0.014, 0.33, 0.50], c.edge);
          }
        }
        for (let row = 0; row < 2; row++) for (let i = -2; i <= 2; i++) b.box([i * 0.40, 1.37 - row * 0.15, front - 0.16 + row * 0.19], [0.36, 0.15, 0.19], c.armor);
      }
      if(!active&&!reinforced) {
        for (const side of [-1, 1]) for (let i = 0; i < 3; i++) {
          const x = side * (turretWidth - 0.02), z = -0.93 + i * 0.47;
          b.box([x, turretY + 0.43, z], [0.052, 0.17, 0.30], c.armor);
        }
      }
    });
    if(active)b.part(`${chosen.active_protection==='aps_hard'?'active_protection':'protection'} / active-protection sensors and intercept modules`,()=>{
        for (const side of [-1, 1]) {
          for (const z of [-1.06, 0.56]) {
            b.box([side * (turretWidth - 0.075), turretTop - 0.19, z], [0.23, 0.28, 0.29], c.shade);
            b.box([side * (turretWidth + 0.05), turretTop - 0.17, z], [0.018, 0.19, 0.22], c.glass);
            for (let stripe = -2; stripe <= 2; stripe++) b.box([side * (turretWidth + 0.063), turretTop - 0.17 + stripe * 0.033, z], [0.009, 0.008, 0.205], c.steel);
          }
          const x = side * (turretWidth + 0.035);
          b.cylinder([x, turretY + 0.30, -0.26], [x, turretY + 0.48, -0.26], 0.17, c.steel, 20);
          b.box([x, turretY + 0.58, -0.24], [0.30, 0.21, 0.48], c.armor);
          for (const offset of [-0.11, 0.11]) b.cylinder([x + side * 0.04, turretY + 0.59, -0.24 + offset], [x + side * 0.23, turretY + 0.67, -0.24 + offset], 0.068, c.steel, 14);
          b.rod([side * (turretWidth - 0.07), turretTop - 0.27, -1.12], [side * (turretWidth - 0.07), turretTop - 0.27, 0.53], 0.018, c.cable);
        }
        b.box([0, turretTop + 0.02, -1.05], [0.36, 0.12, 0.29], c.shade);
    });

    b.part("stowage / bustle rack, canvas rolls, tow cable and tools", () => {
      const rackRear = -2.03, rackFront = -1.50, rackY = turretY + 0.24, rackHalf = turretWidth * 0.82;
      b.box([0, rackY, (rackRear + rackFront) / 2], [rackHalf * 2, 0.047, rackFront - rackRear], c.steel);
      for (const x of [-rackHalf, rackHalf]) {
        b.rod([x, rackY, rackRear], [x, rackY + 0.43, rackRear], 0.023, c.bright);
        b.rod([x, rackY + 0.43, rackRear], [x, rackY + 0.43, rackFront], 0.023, c.bright);
        b.rod([x, rackY, rackFront], [x, rackY + 0.43, rackFront], 0.023, c.bright);
      }
      for (const height of [0.15, 0.40]) b.rod([-rackHalf, rackY + height, rackRear], [rackHalf, rackY + height, rackRear], 0.022, c.bright);
      for (let i = 0; i < 9; i++) {
        const x = -rackHalf + i * rackHalf / 4;
        b.rod([x, rackY, rackRear], [x, rackY + 0.40, rackRear], 0.013, c.steel, 6);
      }
      for (const x of [-0.53, 0.11, 0.60]) {
        b.box([x, rackY + 0.15, -1.77], [0.40, 0.23, 0.37], x < 0 ? c.canvas : c.shade);
        b.box([x, rackY + 0.279, -1.77], [0.41, 0.025, 0.38], c.edge);
        b.box([x, rackY + 0.289, -1.77], [0.045, 0.012, 0.40], c.steel);
      }
      b.cylinder([-0.60, turretTop + 0.065, -1.03], [0.32, turretTop + 0.065, -1.03], 0.105, c.canvas, 16);
      for (const x of [-0.43, 0.16]) b.cylinder([x, turretTop + 0.065, -1.03], [x + 0.04, turretTop + 0.065, -1.03], 0.110, c.steel, 16);
      const cablePoints = [[-hullWidth + 0.10, 1.56, -0.70], [-hullWidth + 0.10, 1.59, -1.60], [-hullWidth + 0.19, 1.60, rear + 0.21],
        [0, 1.61, rear + 0.16], [hullWidth - 0.19, 1.60, rear + 0.21], [hullWidth - 0.10, 1.59, -1.60], [hullWidth - 0.10, 1.56, -0.70]];
      for (let i = 0; i < cablePoints.length - 1; i++) b.rod(cablePoints[i], cablePoints[i + 1], 0.031, c.cable, 10);
      b.rod([hullWidth - 0.14, 1.59, 0.06], [hullWidth - 0.14, 1.59, 1.21], 0.027, c.canvas, 10);
      b.box([hullWidth - 0.14, 1.59, 1.24], [0.18, 0.052, 0.25], c.steel);
      for (const z of [0.20, 0.95]) b.box([hullWidth - 0.14, 1.626, z], [0.12, 0.035, 0.052], c.bright);
    });

    b.part('mobility / powerpack installation',()=>{
      if(chosen.mobility==='engine_turbine_1500') {b.box([0,1.12,rear-.18],[1.35,.46,.33],c.shade);for(let i=-4;i<=4;i++)b.box([i*.14,1.12,rear-.36],[.035,.35,.04],c.bright);}
      if(chosen.mobility==='engine_diesel_600') b.box([0,1.64,rear+.65],[.62,.16,.38],c.hull);
    });
    b.part('transmission / final drive housing',()=>{
      if(chosen.transmission==='transmission_auto') {b.box([0,.63,rear-.13],[.93,.30,.18],c.hull);for(let i=-2;i<=2;i++)b.box([i*.16,.64,rear-.23],[.04,.2,.02],c.edge);}
    });
    b.part('turret / autoloader bustle',()=>{
      if(autoload){b.box([0,turretY+.37,-1.76],[turretWidth*1.62,.51,.59],c.armor);for(let i=-2;i<=2;i++)b.box([i*.30,turretY+.635,-1.75],[.24,.025,.44],c.edge);}
    });
    b.part('active_protection / soft-kill sensors',()=>{
      if(chosen.active_protection==='aps_soft')for(const side of [-1,1]){b.box([side*turretWidth,turretTop-.10,-.7],[.16,.18,.2],c.shade);b.box([side*(turretWidth+.09),turretTop-.10,-.7],[.025,.10,.12],c.glass);}
    });
    b.part('sensors / night observation housing',()=>{
      if(chosen.sensors==='optics_night'){b.cylinder([-.45,turretTop+.05,.55],[-.45,turretTop+.26,.55],.17,c.shade,20);b.box([-.45,turretTop+.19,.73],[.22,.13,.03],c.lens);}
    });
    b.part('fire_control / stabilization and rangefinding equipment',()=>{
      if(chosen.fire_control==='fcs_stabilized')b.cylinder([0,turretY+.49,2.02],[0,turretY+.49,2.20],.20,c.bright,24);
      if(chosen.fire_control==='fcs_digital'){b.box([.64,turretTop-.12,.87],[.33,.27,.34],c.shade);b.box([.64,turretTop-.12,1.05],[.25,.14,.025],c.lens);b.rod([.64,turretTop-.24,.7],[.80,turretY+.37,.38],.022,c.cable);}
    });
    const result=b.finish(`Original ${spec.platform.replace('tank_','')} tank game model. Visual interpretation of the selected specifications. Internal ammunition loads affect game ratings and are recorded in the exported specification metadata.`);
    if(spec.platform==='tank_light'){for(let i=0;i<result.positions.length;i++)result.positions[i]*=.80;result.bounds.min=result.bounds.min.map(v=>v*.80);result.bounds.max=result.bounds.max.map(v=>v*.80);}
    result.specification={platform:spec.platform,components:Object.fromEntries(Object.entries(chosen).filter(([,value])=>value))};
    return result;
  }
  return Object.freeze({ build });
});
