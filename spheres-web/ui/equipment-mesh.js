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
  function buildAircraft(spec) {
    const s=spec.components,b=createBuilder(),strike=spec.platform==='air_tactical_strike';
    const c={paint:[.37,.41,.36],upper:[.45,.49,.42],edge:[.53,.56,.48],dark:[.19,.23,.21],steel:PALETTE.steel,black:PALETTE.black,glass:[.09,.22,.28],rubber:PALETTE.rubber};
    const length=strike?16.8:11.8,y=strike?2.10:1.72,w=strike?.77:.58,rear=-length*.49,front=length*.51;
    const swept=s.air_wing==='air_wing_swept',stable=s.air_wing==='air_wing_stable',span=(strike?6.15:5.15)+(stable?.75:0),wingZ=strike?.15:-.1;
    const twin=s.air_engine==='air_engine_twin',efficient=s.air_engine==='air_engine_efficient',mapping=s.air_radar==='air_radar_mapping',digital=s.air_avionics==='air_avionics_digital',ecm=s.air_countermeasures==='air_countermeasures_ecm',heavy=s.air_hardpoints==='air_hardpoints_heavy',guided=s.air_payload==='air_payload_guided',extended=s.air_fuel==='air_fuel_extended';
    const ring=(z,rx,ry,cy=y,cx=0,n=32)=>Array.from({length:n},(_,i)=>[cx+rx*Math.cos(TAU*i/n),cy+ry*Math.sin(TAU*i/n),z]);
    const body=(rows,color,cx=0)=>b.loft(rows.map(([z,rx,ry,cy])=>ring(z,rx,ry,cy??y,cx)),color);
    // Closed tapered plates support both horizontal swept wings and vertical fins.
    const plate=(outline,thickness,color,vertical=false)=> {
      const d=vertical?[thickness/2,0,0]:[0,thickness/2,0];
      b.loft([outline.map(p=>sub(p,d)),outline.map(p=>add(p,d))],color);
    };
    const panel=(a,z,width,depth)=>b.box([a,y+w*.90,z],[width,.015,depth],c.dark);
    b.part('air_wing / airframe and wing roots',()=>{
      body([[rear,.16,.19],[rear+length*.08,w*.56,w*.55],[rear+length*.20,w*.83,w*.83],[rear+length*.37,w,w*.90],[rear+length*.55,w,w*.95],[rear+length*.72,w*.81,w*.86],[front-length*.15,w*.56,w*.59],[front-length*.10,w*.43,w*.47]],c.paint);
      for(const side of [-1,1]) {
        plate([[side*w*.5,y,wingZ+1.85],[side*w*2,y-.03,wingZ+.6],[side*w*2,y-.08,wingZ-1.3],[side*w*.5,y,wingZ-2.0]],.22,c.paint);
        for(let i=0;i<5;i++)b.box([side*w*.99,y-.02,rear+2.3+i*.18],[.017,.29,.045],c.dark);
      }
    },'air_wing');
    b.part(`air_radar / ${mapping?'terrain-mapping radome and sensor fairing':'basic ranging radome'}`,()=>{
      const offset=mapping?.25:0;
      body([[front-length*.10,w*.43,w*.47],[front-length*.055,w*.27,w*.30],[front+offset,.016,.022]],mapping?c.dark:shade(c.paint,.85));
      b.rod([0,y,front+offset],[0,y,front+offset+.62],.018,c.steel,12);
      if(mapping){body([[front-1.65,.17,.12,y-.43],[front-1.14,.20,.16,y-.40],[front-.85,.04,.05,y-.36]],c.dark);b.cylinder([0,y-.5,front-1.05],[0,y-.52,front-.96],.10,PALETTE.lens,24);}
    },'air_radar');
    b.part(`air_avionics / ${digital?'digital mission cockpit and targeting pod':'analog cockpit and radio aerials'}`,()=>{
      const z=front-length*.29,canopyLength=strike?2.95:2.35;
      const rows=[[z-canopyLength*.52,w*.28,.12,y+w*.8],[z-canopyLength*.3,w*.53,.41,y+w*.91],[z+canopyLength*.19,w*.49,.49,y+w*.91],[z+canopyLength*.46,w*.22,.20,y+w*.87]];
      body(rows,c.glass);
      for(const [rz,rx,ry,cy] of [rows[1],rows[2]])for(let i=0;i<16;i++){const t=TAU*i/32,t2=TAU*(i+1)/32;b.rod([rx*Math.cos(t),cy+ry*Math.sin(t),rz],[rx*Math.cos(t2),cy+ry*Math.sin(t2),rz],.027,c.edge,8);}
      for(const side of [-1,1])b.rod([side*w*.49,y+w*.89,z-canopyLength*.3],[side*w*.22,y+w*.87,z+canopyLength*.46],.025,c.edge);
      plate([[0,y+w*.8,rear+length*.46],[0,y+w*.8+.47,rear+length*.43],[0,y+w*.8,rear+length*.40]],.035,c.dark,true);
      if(digital){body([[.35,.21,.2,y-.73],[1.55,.21,.20,y-.73],[1.87,.11,.14,y-.73]],c.dark,.58);b.cylinder([.58,y-.73,1.86],[.58,y-.73,1.89],.105,PALETTE.lens,28);b.box([0,y+w*.96,z-1.3],[.25,.13,.5],c.edge);}
      else for(const side of [-1,1])b.rod([side*.22,y+w*.85,z-1.1],[side*.28,y+w*.85+.35,z-1.55],.012,c.steel);
    },'air_avionics');
    for(const side of [-1,1]) {
      const lead=wingZ+(swept?-1.20:stable?.1:.55),trail=lead-(swept?1.05:1.45);
      b.part(`air_wing / ${side<0?'port':'starboard'} ${swept?'swept':stable?'high-stability':'straight'} wing`,()=>{
        const root=side*w*.76,tip=side*span,dihedral=stable?.32:.12;
        plate([[root,y,wingZ+1.7],[tip,y+dihedral,lead],[tip,y+dihedral,trail],[root,y,wingZ-1.95]],.15,c.paint);
        // Separate flaps, leading-edge strips, panel joins and navigation lenses.
        const outline=[[side*(w+.25),y-.04,wingZ-1.82],[side*(span-.23),y+dihedral-.04,trail+.10],[side*(span-.23),y+dihedral-.04,trail-.10],[side*(w+.25),y-.04,wingZ-2.02]];
        plate(outline,.085,c.upper);
        b.rod([root,y+.086,wingZ+1.65],[tip,y+dihedral+.086,lead-.02],.017,c.edge,8);
        for(let k=1;k<=3;k++){const f=k/4,xx=side*(w+(span-w)*f),zz=wingZ+1.7+(lead-wingZ-1.7)*f;b.rod([xx,y+dihedral*f+.083,zz-.18],[xx,y+dihedral*f+.083,zz-1.02],.011,c.dark,8);}
        if(stable)plate([[tip,y+.2,trail+.2],[tip,y+.8,trail+.35],[tip,y+.8,lead-.1],[tip,y+.2,lead]],.055,c.upper,true);
        b.cylinder([tip,y+dihedral,lead-.08],[tip+side*.07,y+dihedral,lead-.08],.064,side<0?[.60,.07,.06]:[.06,.47,.22],14);
      },'air_wing');
      b.part(`air_wing / ${side<0?'port':'starboard'} tailplane`,()=>{
        const tz=rear+1.3;
        plate([[side*.17,y+.12,tz+1.5],[side*(strike?2.85:2.23),y+.23,tz-.02],[side*(strike?2.8:2.2),y+.23,tz-.71],[side*.17,y+.12,tz-.37]],.10,c.upper);
        b.rod([side*.30,y+.18,tz-.28],[side*(strike?2.7:2.1),y+.29,tz-.63],.015,c.dark,8);
      },'air_wing');
    }
    b.part('air_wing / vertical stabilizers and rudders',()=>{
      for(const side of strike?[-1,1]:[0]){
        const x=side*.62,tz=rear+1.3,h=strike?2.0:1.65;
        plate([[x,y+.16,tz+1.75],[x+side*.4,y+h,tz+.55],[x+side*.5,y+h,tz-.22],[x,y+.16,tz-.57]],.12,c.paint,true);
        b.rod([x+side*.44,y+h-.11,tz-.10],[x+side*.02,y+.3,tz-.44],.018,c.dark,8);
      }
    },'air_wing');
    const engineXs=twin?[-.73,.73]:[0],engineRadius=twin?.54:efficient?.52:.43;
    engineXs.forEach((x,index)=>{
      b.part(`air_engine / ${twin?(index?'starboard':'port')+' twin turbofan':efficient?'efficient turbofan':'economical turbine'} nacelle`,()=>{
        const ey=y-.15,inlet=strike?1.0:.35,exhaust=rear+.03;
        body([[exhaust,engineRadius*.87,engineRadius*.87,ey],[rear+1.1,engineRadius,engineRadius,ey],[rear+2.8,engineRadius*1.13,engineRadius*1.04,ey],[inlet-.45,engineRadius*.95,engineRadius,ey],[inlet,engineRadius*.89,engineRadius*.89,ey]],c.paint,x);
        b.tube([x,ey,inlet-.42],[x,ey,inlet+.04],engineRadius*.87,engineRadius*.73,c.edge,36);
        b.cylinder([x,ey,inlet-.43],[x,ey,inlet-.39],engineRadius*.71,c.black,36);
        for(let k=0;k<20;k++){const t=TAU*k/20;plate([[x+Math.cos(t)*.07,ey+Math.sin(t)*.07,inlet-.37],[x+Math.cos(t+.12)*engineRadius*.69,ey+Math.sin(t+.12)*engineRadius*.69,inlet-.36],[x+Math.cos(t+.23)*engineRadius*.69,ey+Math.sin(t+.23)*engineRadius*.69,inlet-.34]],.008,c.steel);}
        b.cylinder([x,ey,inlet-.4],[x,ey,inlet-.17],.075,c.edge,20,.02);
        b.tube([x,ey,exhaust+.28],[x,ey,exhaust-.34],engineRadius*.86,engineRadius*.68,c.steel,40);
        b.cylinder([x,ey,exhaust+.29],[x,ey,exhaust+.27],engineRadius*.67,c.black,36);
        for(let k=0;k<24;k++){const a=TAU*k/24;b.rod([x+Math.cos(a)*engineRadius*.86,ey+Math.sin(a)*engineRadius*.86,exhaust+.12],[x+Math.cos(a)*engineRadius*.86,ey+Math.sin(a)*engineRadius*.86,exhaust-.34],.013,c.edge,8);}
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
      for(const side of [-1,1]){panel(side*.25,-.55,.23,.32);b.cylinder([side*.25,y+w*.91,-.55],[side*.25,y+w*.925,-.55],.075,c.edge,20);}
      if(extended)for(const side of [-1,1]){
        const x=side*(span-.55),z=wingZ-.10,fy=y+.10;
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
    return mesh;
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
        if(wheeled)for(let j=0;j<32;j++) {
          const a=TAU*j/32,radial=[0,Math.cos(a),Math.sin(a)],tangent=[0,-Math.sin(a),Math.cos(a)];
          for(const strip of [-1,1])b.box([side*x+strip*width*.23,r+Math.cos(a)*(r+.006),z+Math.sin(a)*(r+.006)],[width*.43,.035,.075],c.track,[[1,0,0],radial,tangent]);
        }
      },wheeled?'wheels':'tracks');
    }
    b.part('protection / specialist sloped hull',()=>{
      b.loft([octagon(.44,w-.24,rear+.25,front-.44,.25),octagon(.94,w,rear,front,.27),octagon(deck,w-.14,rear+.10,front-.62,.30)],c.hull);
      b.loft([octagon(deck,w-.14,rear+.10,front-.62,.30),octagon(deck+.065,w-.20,rear+.16,front-.69,.28)],c.upper);
      for(const side of [-1,1]) {
        b.box([side*(w+.06),wheeled?1.33:1.18,0],[.25,.075,length-.40],c.edge);
        for(const z of [rear+.21,front-.12]) {
          b.rod([side*w*.66,.74,z],[side*w*.66,.74,z+(z>0?.16:-.16)],.063,c.bright,12);
          b.box([side*w*.77,1.08,z],[.15,.14,.07],c.shade);
          b.cylinder([side*w*.77,1.08,z],[side*w*.77,1.08,z+(z>0?.055:-.055)],.045,z>0?c.lens:c.amber,12);
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
        b.box([0,mountTop-.03,mountZ+.10],[.49,.32,.085],c.armor);
        for(const side of [-1,1])b.box([side*.235,mountTop-.015,mountZ-.03],[.05,.29,.32],c.upper);
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
      for(const side of [-1,1]) {
        hatch(side*w*.37,deck+height,rear+1.01,.25);
        for(let j=0;j<3;j++)b.box([side*(w-.09),deck-.17,rear+.48+j*.43],[.045,.075,.14],c.glass);
      }
      b.box([0,deck-.46,rear-.035],[w*1.18,.90,.085],protectedBay?c.armor:c.shade);
      for(const x of [-w*.50,w*.50])b.rod([x,.79,rear-.092],[x,deck-.16,rear-.092],protectedBay?.038:.023,c.bright);
      for(let j=0;j<5;j++)b.box([0,.81+j*.135,rear-.085],[w*.90,.020,.025],c.steel);
      b.box([0,deck-.27,rear-.10],[.17,.055,.035],c.bright);
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
    if(Object.hasOwn(AIR_DEFAULTS,spec.platform))return buildAircraft(spec);
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
      b.loft([octagon(0.39, hullWidth - 0.29, rear + 0.16, front - 0.49, 0.28),
        octagon(0.91, hullWidth - 0.08, rear, front + 0.08, 0.27),
        octagon(1.47, hullWidth, rear + 0.06, front - 0.43, 0.32)], c.hull);
      b.loft([octagon(1.475, hullWidth + 0.015, rear + 0.05, front - 0.43, 0.32),
        octagon(1.54, hullWidth - 0.025, rear + 0.11, front - 0.49, 0.32)], c.upper);
      b.box([0, 0.58, rear - 0.03], [hullWidth * 1.5, 0.22, 0.12], c.shade);
      for (const x of [-hullWidth * 0.72, hullWidth * 0.72]) {
        b.rod([x, 0.78, front - 0.04], [x, 0.78, front + 0.19], 0.095, c.bright, 12);
        b.tube([x, 0.80, front + 0.16], [x, 0.80, front + 0.23], 0.115, 0.063, c.steel, 16);
        b.tube([x, 0.79, rear - 0.15], [x, 0.79, rear - 0.05], 0.11, 0.060, c.steel, 16);
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
        for (const end of [-1, 1]) {
          const z = end * wheelHalfSpan, y = trackCenterY + 0.21, r = 0.36;
          b.cylinder([x - 0.28, y, z], [x + 0.28, y, z], r, c.steel, 28);
          b.cylinder([x + side * 0.281, y, z], [x + side * 0.326, y, z], r * 0.75, c.hull, 28);
          b.cylinder([x + side * 0.325, y, z], [x + side * 0.355, y, z], 0.13, c.bright, 16);
          for (let tooth = 0; tooth < 14; tooth++) {
            const angle = TAU * tooth / 14, radial = [0, Math.sin(angle), Math.cos(angle)], tangent = [0, Math.cos(angle), -Math.sin(angle)];
            b.box([x, y + radial[1] * r, z + radial[2] * r], [0.49, 0.075, 0.10], c.bright, [[1, 0, 0], radial, tangent]);
          }
        }
        for (let i = 0; i < 3; i++) {
          const z = -wheelHalfSpan * 0.66 + i * wheelHalfSpan * 0.66;
          b.cylinder([x - 0.18, 1.13, z], [x + 0.18, 1.13, z], 0.135, c.rubber, 16);
        }
      });
      b.part(`chassis / ${label} fenders, segmented skirts and fixtures`, () => {
        b.box([x, 1.43, -0.08], [trackWidth + 0.11, 0.085, (wheelHalfSpan + 0.54) * 2], c.shade);
        const outerX = x + side * (trackWidth / 2 + 0.055), panels = heavy ? 7 : 6;
        for (let i = 0; i < panels; i++) {
          const z = -wheelHalfSpan + (i + 0.5) * (wheelHalfSpan * 2 / panels), panelLength = wheelHalfSpan * 2 / panels - 0.035;
          b.box([outerX, 1.23, z], [0.065, reinforced ? 0.55 : 0.36, panelLength], i % 2 ? c.hull : c.armor);
          b.box([outerX + side * 0.039, 1.40, z], [0.018, 0.045, panelLength * 0.76], c.edge);
          for (const dz of [-panelLength * 0.34, panelLength * 0.34]) b.cylinder([outerX + side * 0.030, 1.33, z + dz], [outerX + side * 0.048, 1.33, z + dz], 0.025, c.bright, 6);
        }
        for (const end of [-1, 1]) b.box([x, 1.08, end * (wheelHalfSpan + 0.55)], [trackWidth + 0.06, 0.57, 0.046], c.rubber);
        b.box([side * (hullWidth - 0.16), 1.53, front - 0.48], [0.25, 0.15, 0.21], c.shade);
        b.box([side * (hullWidth - 0.16), 1.55, front - 0.37], [0.16, 0.078, 0.012], c.amber);
        for (const dx of [-0.12, 0.12]) b.rod([side * (hullWidth - 0.16) + dx, 1.53, front - 0.35], [side * (hullWidth - 0.16) + dx, 1.72, front - 0.35], 0.012, c.steel, 6);
        b.rod([side * (hullWidth - 0.16) - 0.12, 1.72, front - 0.35], [side * (hullWidth - 0.16) + 0.12, 1.72, front - 0.35], 0.012, c.steel, 6);
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
      else b.loft([octagon(turretY, turretWidth * 0.88, -1.56, 1.20, 0.35, turretZ),
        octagon(turretY + 0.23, turretWidth, -1.64, 1.37, 0.47, turretZ),
        octagon(turretTop - 0.08, turretWidth * 0.84, -1.36, 0.83, 0.37, turretZ),
        octagon(turretTop, turretWidth * 0.77, -1.25, 0.73, 0.32, turretZ)], c.upper);
      // Separate cheek castings belong to the rotating turret, not the fixed hull mounting.
      for (const side of casemate?[]:[-1, 1]) {
        const cheek = [[side * 0.43, turretY + 0.25, 1.49], [side * (turretWidth - 0.27), turretY + 0.28, 1.45],
          [side * turretWidth, turretY + 0.29, 0.94], [side * (turretWidth - 0.16), turretTop - 0.10, 0.73],
          [side * 0.46, turretTop - 0.13, 0.89]];
        const inside = cheek.map(point => [point[0], point[1] - 0.10, point[2] - 0.055]), center = mean([...cheek, ...inside]);
        b.face(cheek, c.armor, center); b.face(inside, c.shade, center);
        for (let i = 0; i < cheek.length; i++) { const next = (i + 1) % cheek.length; b.face([cheek[i], cheek[next], inside[next], inside[i]], c.edge, center); }
      }
      b.box([0, turretY + 0.41, -1.59], [turretWidth * 1.56, 0.38, 0.27], c.shade);
      for (const x of [-turretWidth * 0.57, 0, turretWidth * 0.57]) b.box([x, turretY + 0.56, -1.739], [0.27, 0.09, 0.024], c.steel);
    });

    b.part(`armament / ${heavyGun ? "heavy weapon, enlarged mantlet and sleeved barrel" : "standard weapon, mantlet and barrel"}`, () => {
      const gunY = turretY + 0.49, radius = chosen.armament==='gun_90'?.085:chosen.armament==='gun_125'?.15:heavyGun ? 0.132 : 0.105, muzzle = chosen.armament==='gun_90'?4.78:chosen.armament==='gun_125'?6.67:heavyGun ? 6.36 : 5.68;
      b.box([0, gunY, 1.20], [heavyGun ? 0.90 : 0.75, heavyGun ? 0.68 : 0.55, 0.61], c.shade);
      b.cylinder([-0.45, gunY, 1.45], [0.45, gunY, 1.45], heavyGun ? 0.35 : 0.29, c.armor, 32);
      b.cylinder([0, gunY, 1.46], [0, gunY, 1.99], radius * 1.85, c.steel, 32, radius * 1.34);
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
