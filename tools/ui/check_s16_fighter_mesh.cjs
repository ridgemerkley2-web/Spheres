#!/usr/bin/env node
"use strict";
const assert = require("node:assert/strict");
const crypto = require("node:crypto");
const fs = require("node:fs");
const path = require("node:path");
const root = path.resolve(__dirname, "../..");
const meshPath = path.join(root, "spheres-web/ui/equipment-mesh.js");
const baselinePath = path.join(__dirname, "s16_inherited_mesh_hashes.json");
const sourceRevision = "ab17714";
const legacySpecs = [
  ...["tank_standard", "tank_heavy", "tank_light", "tank_destroyer", "ground_ifv", "ground_apc", "ground_recon", "ground_artillery", "ground_air_defense", "air_light_attack", "air_tactical_strike"].map(platform => ({platform})),
  {platform:"air_light_attack",components:{air_engine:"air_engine_efficient",air_wing:"air_wing_stable",air_radar:"air_radar_mapping",air_avionics:"air_avionics_digital",air_countermeasures:"air_countermeasures_ecm",air_payload:"air_payload_guided",air_fuel:"air_fuel_extended"}},
  {platform:"air_tactical_strike",components:{air_engine:"air_engine_efficient",air_wing:"air_wing_stable",air_radar:"air_radar_mapping",air_avionics:"air_avionics_digital",air_countermeasures:"air_countermeasures_ecm",air_payload:"air_payload_guided",air_fuel:"air_fuel_extended"}}
];
function digest(mesh) {
  const hash = crypto.createHash("sha256"), metadata = {};
  for (const [key, value] of Object.entries(mesh)) {
    if (ArrayBuffer.isView(value)) hash.update(key).update(Buffer.from(value.buffer, value.byteOffset, value.byteLength));
    else metadata[key] = value;
  }
  return hash.update(JSON.stringify(metadata)).digest("hex");
}
// Explicit maintainer command only. The normal portable regression below reads
// the retained hashes and never needs a repository or a git executable.
if (process.argv.includes("--record-baseline")) {
  const source = require("node:child_process").execFileSync("git", ["show", `${sourceRevision}:spheres-web/ui/equipment-mesh.js`], {cwd:root, encoding:"utf8", maxBuffer:4*1024*1024});
  const context = {module:{exports:{}}};
  require("node:vm").runInNewContext(source, context, {filename:"S15-equipment-mesh.js"});
  const rows = legacySpecs.flatMap(spec => [0,1,2].map(lod => ({spec:{...spec,lod},sha256:digest(context.module.exports.build({...spec,lod}))})));
  process.stdout.write(JSON.stringify({source_revision:sourceRevision,source_path:"spheres-web/ui/equipment-mesh.js",hash_contract:"All typed geometry buffers plus exact enumerable metadata, in build order.",rows},null,2)+"\n");
  process.exit(0);
}

const {build} = require(meshPath);
let checks=0;
function check(label, run) {run();checks++;process.stdout.write(`PASS ${label}\n`);}
const defaults = {
  air_engine:"air_engine_interceptor",air_wing:"air_wing_interceptor",air_radar:"air_radar_interceptor",air_avionics:"air_avionics_interceptor",
  air_countermeasures:"air_countermeasures_basic",air_hardpoints:"air_hardpoints_interceptor",air_payload:"air_payload_short_range",air_fuel:"air_fuel_standard"
};
const slots=Object.keys(defaults).sort();
const spec={platform:"air_fighter",components:defaults};
function validate(mesh,lod) {
  assert.equal(mesh.assetKind,"aircraft");
  assert.equal(mesh.specification.platform,"air_fighter");
  assert.equal(mesh.lod,lod);
  assert.equal(mesh.positions.length,mesh.triangleCount*9);
  assert.equal(mesh.normals.length,mesh.positions.length);
  assert.equal(mesh.colors.length,mesh.positions.length);
  const bands=[[100000,250000],[4000,18000],[300,2500]];
  assert.ok(mesh.triangleCount>=bands[lod][0]&&mesh.triangleCount<=bands[lod][1],`LOD${lod}: ${mesh.triangleCount} triangles`);
  assert.deepEqual([...new Set(mesh.parts.map(p=>p.slot))].sort(),slots);
  assert.ok(!mesh.parts.some(p=>/\b(tank|bombs?|turret|tracks?)\b|targeting pod/.test(p.name)),"Fighter has no tank or bomber fittings");
  let end=0;
  for(const part of mesh.parts){assert.equal(part.first,end);assert.ok(part.count>0&&part.count%3===0);end+=part.count;}
  assert.equal(end,mesh.positions.length/3);
  for(let i=0;i<mesh.positions.length;i+=9){
    const p=mesh.positions,u=[p[i+3]-p[i],p[i+4]-p[i+1],p[i+5]-p[i+2]],v=[p[i+6]-p[i],p[i+7]-p[i+1],p[i+8]-p[i+2]];
    const n=[u[1]*v[2]-u[2]*v[1],u[2]*v[0]-u[0]*v[2],u[0]*v[1]-u[1]*v[0]],area=Math.hypot(...n);
    assert.ok(area>1e-9,`Nondegenerate fighter triangle ${i/9}`);
    for(let k=0;k<3;k++){
      const at=i+k*3;
      for(let a=0;a<3;a++){assert.ok(Number.isFinite(p[at+a])&&Number.isFinite(mesh.normals[at+a]));assert.ok(mesh.colors[at+a]>=0&&mesh.colors[at+a]<=1);}
      const normal=[mesh.normals[at],mesh.normals[at+1],mesh.normals[at+2]];
      assert.ok(Math.abs(Math.hypot(...normal)-1)<1e-5);
      assert.ok(normal.reduce((sum,value,a)=>sum+value*n[a]/area,0)>=.40,"Fighter normals follow triangle winding");
    }
  }
  assert.ok(mesh.bounds.min[1]>=-.02&&mesh.bounds.min[1]<.06,"Landing gear meets the turntable");
  assert.ok(mesh.bounds.max[2]-mesh.bounds.min[2]>14,"Long fighter fuselage");
  assert.ok(mesh.bounds.max[0]-mesh.bounds.min[0]<11,"Compact swept fighter span");
}

check("fighter inspection/card/map keep all eight native slots and valid geometry",()=>{
  const meshes=[0,1,2].map(lod=>build({...spec,lod}));
  for(let lod=0;lod<3;lod++){
    validate(meshes[lod],lod);
    assert.deepEqual(meshes[lod].specification.components,defaults);
    assert.deepEqual(meshes[lod].parts.map(p=>[p.name,p.slot]),meshes[0].parts.map(p=>[p.name,p.slot]));
    assert.equal(digest(build({...spec,lod})),digest(meshes[lod]));
    if(lod>0)assert.ok(meshes[lod].triangleCount<meshes[lod-1].triangleCount);
  }
  process.stdout.write(`FIGHTER_TRIANGLES=${meshes.map(m=>m.triangleCount).join(",")}\n`);
});
check("fighter payload is two separated slender air-to-air missiles",()=>{
  const mesh=build(spec),part=mesh.parts.find(p=>p.slot==="air_payload");
  assert.ok(part.name.includes("air-to-air missile"));
  const bounds=[{min:[Infinity,Infinity,Infinity],max:[-Infinity,-Infinity,-Infinity]},{min:[Infinity,Infinity,Infinity],max:[-Infinity,-Infinity,-Infinity]}];
  for(let i=part.first*3;i<(part.first+part.count)*3;i+=9){
    const sign=mesh.positions[i]>0;
    for(let k=0;k<3;k++){
      const at=i+k*3;assert.equal(mesh.positions[at]>0,sign,"Each missile stays on its own mount");
      const b=bounds[Number(sign)];for(let a=0;a<3;a++){b.min[a]=Math.min(b.min[a],mesh.positions[at+a]);b.max[a]=Math.max(b.max[a],mesh.positions[at+a]);}
    }
  }
  for(const b of bounds){assert.ok(b.max[2]-b.min[2]>2.3);assert.ok(b.max[0]-b.min[0]<.6);assert.ok(b.max[1]-b.min[1]<.6);}
});
check("every researched fighter fitting is retained and changes visible geometry",()=>{
  const original=build(spec),before=digest(original);
  for(const [slot,id] of [["air_engine","air_engine_interceptor_efficient"],["air_radar","air_radar_interceptor_tracking"],["air_avionics","air_avionics_interceptor_digital"],["air_countermeasures","air_countermeasures_ecm"],["air_fuel","air_fuel_extended"]]){
    const chosen={...spec,components:{...defaults,[slot]:id}},inspection=build(chosen),card=build({...chosen,lod:1});
    assert.equal(inspection.specification.components[slot],id);assert.notEqual(digest(inspection),before);
    assert.notEqual(digest(card),digest(build({...spec,lod:1})));
    validate(inspection,0);validate(card,1);
  }
});
check("fighter resolver refuses bombs and ground parts without mutating the requested spec",()=>{
  const raw={platform:"air_fighter",components:{air_engine:"air_engine_twin",air_wing:"air_wing_straight",air_radar:"air_radar_mapping",air_avionics:"air_avionics_digital",air_hardpoints:"air_hardpoints_heavy",air_payload:"air_payload_guided",turret:"turret_heavy",armament:"gun_125"}};
  const before=JSON.stringify(raw),mesh=build(raw);
  assert.deepEqual(mesh.specification.components,defaults);
  assert.equal(digest(mesh),digest(build(spec)));assert.equal(JSON.stringify(raw),before);
});
check("all inherited S15 family buffers and metadata remain byte-identical",()=>{
  const baseline=JSON.parse(fs.readFileSync(baselinePath,"utf8"));
  assert.equal(baseline.source_revision,sourceRevision);
  assert.equal(baseline.rows.length,legacySpecs.length*3);
  for(const row of baseline.rows)assert.equal(digest(build(row.spec)),row.sha256,`${row.spec.platform} LOD${row.spec.lod} inherited geometry`);
});
process.stdout.write(`${checks} S16 fighter mesh checks passed.\n`);
