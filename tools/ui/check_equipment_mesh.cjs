#!/usr/bin/env node
"use strict";
const assert = require("node:assert/strict");
const crypto = require("node:crypto");
const fs = require("node:fs");
const path = require("node:path");
const vm = require("node:vm");
const meshPath = path.resolve(__dirname, "../../spheres-web/ui/equipment-mesh.js");
const { build } = require(meshPath);
const baseline = { platform: "tank_standard", components: { mobility: "drive_standard", protection: "protection_standard", armament: "armament_standard", sensors: "sensors_optical", communications: "comms_radio" } };
let checks = 0;
function check(label, fn) { fn(); checks++; process.stdout.write(`PASS ${label}\n`); }
function digest(mesh) { return crypto.createHash("sha256").update(Buffer.from(mesh.positions.buffer)).update(Buffer.from(mesh.colors.buffer)).digest("hex"); }
function validate(mesh,ground=false) {
  const air=ground==='air';
  assert.ok(mesh.positions instanceof Float32Array);
  assert.ok(mesh.normals instanceof Float32Array);
  assert.ok(mesh.colors instanceof Float32Array);
  assert.equal(mesh.positions.length, mesh.normals.length);
  assert.equal(mesh.positions.length, mesh.colors.length);
  assert.equal(mesh.positions.length, mesh.triangleCount * 9);
  assert.ok(mesh.triangleCount >= (ground?8000:20000) && mesh.triangleCount <= 60000, `${mesh.triangleCount} triangles`);
  let previous = 0;
  const names = new Set();
  for (const part of mesh.parts) {
    assert.ok(!names.has(part.name), `unique semantic part ${part.name}`); names.add(part.name);
    assert.equal(part.first, previous); assert.equal(part.count % 3, 0); assert.ok(part.count > 0);
    previous = part.first + part.count;
  }
  assert.equal(previous, mesh.positions.length / 3);
  assert.ok(mesh.parts.length >= (air?16:18));
  for (let i = 0; i < mesh.positions.length; i += 3) {
    for (let j = 0; j < 3; j++) {
      assert.ok(Number.isFinite(mesh.positions[i + j])); assert.ok(Number.isFinite(mesh.normals[i + j]));
      assert.ok(mesh.colors[i + j] >= 0 && mesh.colors[i + j] <= 1);
      assert.ok(mesh.positions[i + j] >= mesh.bounds.min[j] - 1e-5 && mesh.positions[i + j] <= mesh.bounds.max[j] + 1e-5);
    }
    assert.ok(Math.abs(Math.hypot(...mesh.normals.subarray(i, i + 3)) - 1) < 1e-5);
  }
  for (let i = 0; i < mesh.positions.length; i += 9) {
    const a = mesh.positions.subarray(i, i + 3), b = mesh.positions.subarray(i + 3, i + 6), c = mesh.positions.subarray(i + 6, i + 9);
    const u = b.map((v, j) => v - a[j]), v = c.map((v, j) => v - a[j]);
    const n = [u[1] * v[2] - u[2] * v[1], u[2] * v[0] - u[0] * v[2], u[0] * v[1] - u[1] * v[0]], area = Math.hypot(...n);
    assert.ok(area > 1e-9, `nondegenerate triangle ${i / 9}`);
    const facing = n.reduce((sum, value, j) => sum + value / area * mesh.normals[i + j], 0);
    assert.ok(facing > 0.999, `normal follows winding at ${i / 9}`);
  }
  assert.ok(mesh.bounds.min[1] >= -0.02 && mesh.bounds.min[1] < 0.06, "track soles meet ground");
  assert.ok(mesh.bounds.max[2] > (ground?1.7:3) && mesh.bounds.min[2] < -1.7, "forward and aft chassis bounds");
  assert.ok(mesh.description.includes("Visual interpretation"));
}
const base = build(baseline);
check("baseline is bounded finite geometry with normalized winding and semantic parts", () => validate(base));
check("mesh is deterministic and does not mutate a draft", () => {
  const before = JSON.stringify(baseline), next = build(baseline);
  assert.equal(digest(base), digest(next)); assert.equal(JSON.stringify(next.parts), JSON.stringify(base.parts));
  assert.equal(JSON.stringify(baseline), before);
});
check("standard chassis has full track loops, individual links, wheels, muzzle and fittings", () => {
  for (const term of ["continuous track belt", "individual tread links", "suspension", "faceted armor", "barrel", "crew hatches", "stowage"]) assert.ok(base.parts.some(part => part.name.includes(term)), term);
});
const variants = [
  ["mobility", "drive_mobile"], ["protection", "protection_heavy"], ["protection", "protection_active"],
  ["armament", "armament_heavy"], ["sensors", "sensors_integrated"], ["communications", "comms_data"]
];
for (const [slot, id] of variants) {
  check(`${id} visibly changes geometry in its named component part`, () => {
    const next = build({ ...baseline, components: { ...baseline.components, [slot]: id } }); validate(next);
    assert.notEqual(digest(base), digest(next));
    const beforePart = base.parts.find(part => part.name.startsWith(`${slot} /`));
    const afterPart = next.parts.find(part => part.name.startsWith(`${slot} /`));
    assert.ok(beforePart && afterPart); assert.notEqual(beforePart.name, afterPart.name);
    assert.notDeepEqual(base.positions.slice(beforePart.first * 3, (beforePart.first + beforePart.count) * 3), next.positions.slice(afterPart.first * 3, (afterPart.first + afterPart.count) * 3));
  });
}
check("heavy chassis has larger width, longer hull and additional running gear", () => {
  const next = build({ ...baseline, platform: "tank_heavy" }); validate(next);
  assert.ok(next.bounds.max[0] > base.bounds.max[0]); assert.ok(next.bounds.min[2] < base.bounds.min[2]);
  assert.ok(next.triangleCount > base.triangleCount); assert.notEqual(digest(base), digest(next));
});
check("fully upgraded concept stays inside the renderer geometry budget", () => validate(build({ platform: "tank_heavy", components: { mobility: "drive_mobile", protection: "protection_active", armament: "armament_heavy", sensors: "sensors_integrated", communications: "comms_data" } })));
check("missing and unsupported values use stable safe visual defaults", () => {
  for (const input of [undefined, null, {}, { platform: "__proto__", components: { mobility: "<script>" } }]) assert.equal(digest(build(input)), digest(base));
});
check("browser global exports the same build contract without dependencies", () => {
  const context = vm.createContext({}); vm.runInContext(fs.readFileSync(meshPath, "utf8"), context);
  assert.equal(typeof context.EquipmentMesh.build, "function");
  assert.equal(context.EquipmentMesh.build(baseline).triangleCount, base.triangleCount);
});
const detailed={platform:'tank_standard',components:{mobility:'engine_diesel_900',transmission:'transmission_manual',tracks:'tracks_standard',suspension:'suspension_torsion',turret:'turret_standard',armament:'gun_105',ammunition:'ammo_mixed',protection:'protection_standard',active_protection:'aps_none',sensors:'optics_day',fire_control:'fcs_basic',communications:'comms_radio'}};
check('every independent catalogue specification survives model resolution and GLB metadata',()=>{
  const catalogue=fs.readFileSync(path.resolve(__dirname,'../../spheres-sim/src/equipment_specs.rs'),'utf8');
  const rows=[...catalogue.matchAll(/component!\("([^"]+)","[^"]+","([^"]+)"/g)];assert(rows.length>=30);
  for(const [,id,slot] of rows){const mesh=build({...detailed,components:{...detailed.components,[slot]:id}});assert.equal(mesh.specification.components[slot],id);validate(mesh);}
});
check('light chassis, casemate and separate exterior upgrades produce different valid meshes',()=>{
  const original=build(detailed);validate(original);
  for(const [slot,id] of [['tracks','tracks_wide'],['suspension','suspension_hydro'],['turret','turret_compact'],['turret','turret_autoload'],['transmission','transmission_auto'],['fire_control','fcs_digital'],['active_protection','aps_soft'],['sensors','optics_night']]){const next=build({...detailed,components:{...detailed.components,[slot]:id}});assert.notEqual(digest(next),digest(original),id);validate(next);}
  const light=build({...detailed,platform:'tank_light'});assert(light.bounds.max[0]<original.bounds.max[0]);validate(light);
  const destroyer=build({...detailed,platform:'tank_destroyer',components:{...detailed.components,turret:'turret_casemate',armament:'gun_120'}});assert.notEqual(digest(destroyer),digest(original));validate(destroyer);
});
check('reinforced armor and independent active protection are both represented',()=>{
  const armor=build({...detailed,components:{...detailed.components,protection:'protection_heavy'}}),both=build({...detailed,components:{...detailed.components,protection:'protection_heavy',active_protection:'aps_hard'}});
  assert(both.triangleCount>armor.triangleCount);validate(both);
  assert(both.parts.some(p=>p.slot==='active_protection'&&p.name.includes('intercept')));
});
const groundPlatforms=['ground_ifv','ground_apc','ground_recon','ground_artillery','ground_air_defense'];
const ground=Object.fromEntries(groundPlatforms.map(platform=>[platform,build({platform})]));
check('all five specialist chassis are distinct complete geometry with their thirteen role specifications',()=>{
  const hashes=new Set();
  for(const [platform,mesh] of Object.entries(ground)){
    validate(mesh,true);hashes.add(digest(mesh));assert.equal(mesh.specification.platform,platform);
    assert.equal(Object.keys(mesh.specification.components).length,13);
    const original=JSON.stringify(mesh.specification);assert.equal(digest(build(mesh.specification)),digest(mesh));assert.equal(JSON.stringify(mesh.specification),original);
    for(const part of mesh.parts){assert(part.slot in mesh.specification.components,part.name);assert(part.label&&!part.label.includes('ground_'));}
  }
  assert.equal(hashes.size,5);assert(ground.ground_recon.bounds.max[0]<ground.ground_apc.bounds.max[0]);
  assert(ground.ground_artillery.bounds.min[2]<ground.ground_ifv.bounds.min[2]);
  for(const platform of ['ground_apc','ground_recon']){
    assert.equal(ground[platform].parts.filter(p=>p.name.includes('road tire')).length,6);
    assert(!('tracks' in ground[platform].specification.components));assert(!ground[platform].parts.some(p=>p.slot==='tracks'));
  }
  for(const platform of ['ground_ifv','ground_artillery','ground_air_defense'])assert.equal(ground[platform].parts.filter(p=>p.name.includes('continuous articulated belt')).length,2);
});
check('every new simulation component resolves and exterior choices change the actual triangles',()=>{
  const catalogue=fs.readFileSync(path.resolve(__dirname,'../../spheres-sim/src/equipment_ground.rs'),'utf8');
  const rows=[...catalogue.matchAll(/component!\("([^"]+)","[^"]+","([^"]+)"/g)];assert(rows.length>=30);
  for(const [,id,slot] of rows){
    let platform='ground_ifv';
    if(/howitzer|loader|ammo_he|ammo_guided/.test(id))platform='ground_artillery';
    else if(/radar|_aa|missile/.test(id))platform='ground_air_defense';
    else if(/wheels|recon/.test(id))platform='ground_recon';
    else if(/station_mg|mg_127|ammo_ball/.test(id))platform='ground_apc';
    const original=ground[platform],spec={platform,components:{...original.specification.components,[slot]:id}},snapshot=JSON.stringify(spec),next=build(spec);
    assert.equal(next.specification.components[slot],id);validate(next,true);assert.equal(JSON.stringify(spec),snapshot);
    if(slot!=='ammunition'&&original.specification.components[slot]!==id)assert.notEqual(digest(next),digest(original),id);
  }
});
check('mission upgrades alter observable silhouette, running gear and weapons',()=>{
  const upgrade=(platform,components)=>build({platform,components:{...ground[platform].specification.components,...components}});
  const wheels=upgrade('ground_recon',{wheels:'ground_wheels_runflat'});assert.equal(wheels.parts.filter(p=>p.name.includes('road tire')).length,8);
  const mast=upgrade('ground_recon',{recon_package:'ground_recon_mast'});assert(mast.bounds.max[1]>ground.ground_recon.bounds.max[1]+.5);
  const gun=upgrade('ground_artillery',{armament:'ground_howitzer_155'});assert(gun.bounds.max[2]>ground.ground_artillery.bounds.max[2]+.4);
  const missiles=upgrade('ground_air_defense',{armament:'ground_aa_missiles',radar:'ground_radar_tracking',ammunition:'ground_ammo_missiles',fire_control:'fcs_digital'});validate(missiles,true);
  assert(missiles.parts.some(p=>p.slot==='armament'&&p.label.includes('missile')));assert(missiles.parts.some(p=>p.slot==='radar'&&p.label.includes('tracking')));
  const scout=upgrade('ground_recon',{turret:'ground_turret_autocannon',armament:'ground_gun_35',ammunition:'ground_ammo_autocannon'});assert(scout.bounds.max[2]>ground.ground_recon.bounds.max[2]);validate(scout,true);
  for(const platform of groundPlatforms){
    const original=ground[platform];
    for(const [slot,id] of [['mobility','ground_engine_750'],['transmission','transmission_auto'],['suspension','suspension_hydro'],['protection','ground_armor_modular'],['sensors','optics_night'],['sensors','optics_thermal'],['fire_control','fcs_stabilized'],['fire_control','fcs_digital'],['active_protection','aps_soft'],['active_protection','aps_hard']])assert.notEqual(digest(upgrade(platform,{[slot]:id})),digest(original),`${platform} ${id}`);
    if(original.specification.components.tracks)for(const tracks of ['tracks_wide','tracks_padded'])assert.notEqual(digest(upgrade(platform,{tracks})),digest(original),`${platform} ${tracks}`);
  }
});
const aircraft=Object.fromEntries(['air_light_attack','air_tactical_strike'].map(platform=>[platform,build({platform})]));
check('light attack and tactical strike are distinct complete aircraft with eight pickable specifications',()=>{
  const sim=fs.readFileSync(path.resolve(__dirname,'../../spheres-sim/src/equipment_aviation.rs'),'utf8');
  const slots=[...sim.slice(sim.indexOf('pub const AVIATION_SLOTS'),sim.indexOf('];')+2).matchAll(/"(air_\w+)"/g)].map(m=>m[1]);assert.equal(slots.length,8);
  for(const [platform,mesh] of Object.entries(aircraft)){
    validate(mesh,'air');assert.equal(mesh.specification.platform,platform);assert.deepEqual(Object.keys(mesh.specification.components).sort(),slots.slice().sort());
    for(const slot of slots)assert(mesh.parts.some(p=>p.slot===slot),slot);
    assert(mesh.parts.every(p=>slots.includes(p.slot)));assert.equal(mesh.parts.filter(p=>p.label.includes('landing gear')).length,3);
    assert(mesh.bounds.max[0]-mesh.bounds.min[0]>9);assert(mesh.bounds.max[2]-mesh.bounds.min[2]>12);
    const spec={name:'Test aircraft',platform,components:mesh.specification.components},before=JSON.stringify(spec);assert.equal(digest(build(spec)),digest(mesh));assert.equal(JSON.stringify(spec),before);
  }
  assert.notEqual(digest(aircraft.air_light_attack),digest(aircraft.air_tactical_strike));
  assert(aircraft.air_tactical_strike.bounds.max[2]>aircraft.air_light_attack.bounds.max[2]+2);
  assert.equal(aircraft.air_light_attack.parts.filter(p=>p.slot==='air_engine').length,1);assert.equal(aircraft.air_tactical_strike.parts.filter(p=>p.slot==='air_engine').length,2);
});
check('every aviation component resolves from the live catalogue and every selectable upgrade changes its own visible assembly',()=>{
  const sim=fs.readFileSync(path.resolve(__dirname,'../../spheres-sim/src/equipment_aviation.rs'),'utf8');
  const rows=[...sim.matchAll(/component!\("(air_\w+)","[^"]+","(air_\w+)"/g)];assert.equal(rows.length,18);
  for(const [,id,slot] of rows){
    const original=aircraft.air_tactical_strike,next=build({platform:'air_tactical_strike',components:{...original.specification.components,[slot]:id}});validate(next,'air');assert.equal(next.specification.components[slot],id);
    if(original.specification.components[slot]!==id){
      const selected=m=>m.parts.filter(p=>p.slot===slot).flatMap(p=>Array.from(m.positions.slice(p.first*3,(p.first+p.count)*3)));
      assert.notDeepEqual(selected(next),selected(original),id+' must change its selected assembly');
    }
  }
});
check('light aircraft reject heavy-only visuals and omit stale ground slots',()=>{
  const mesh=build({platform:'air_light_attack',components:{air_engine:'air_engine_twin',air_wing:'air_wing_swept',air_hardpoints:'air_hardpoints_heavy',armament:'gun_125',mobility:'drive_mobile'}});
  assert.deepEqual(mesh.specification,aircraft.air_light_attack.specification);assert.equal(digest(mesh),digest(aircraft.air_light_attack));
});
process.stdout.write(`${checks} equipment mesh checks passed; baseline ${base.triangleCount.toLocaleString("en-US")} triangles.\n`);
