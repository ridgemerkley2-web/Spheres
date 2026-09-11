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

// Triangle bands. LOD0 is the inspection mesh build(spec) returns; LOD1 is the
// catalogue card; LOD2 is the map pin. The upper LOD0 figures are the art budget
// for this pass, not the binding constraint — see the export-ceiling check near
// the bottom of this file, which is what actually limits LOD0 today.
// September inspection pass: 283 single-option specialists peak at 45,826;
// the loaded fixtures peak at 46,106. 48k reserves 4.1% for selected designs.
const BANDS = [{ tank: [20000, 150000], ground: [8000, 48000] }, { tank: [4000, 12000], ground: [4000, 12000] }, { tank: [300, 1500], ground: [300, 1500] }];

// The normals contract, and why it is not the one that used to be here.
//
// The old assertion was `facing > 0.999` at every vertex: each vertex normal had
// to equal its own face normal. That is not a correctness property, it is an
// assertion that the model is FLAT SHADED, and it made every cylinder, dome,
// tyre and barrel on nine platforms read as a faceted prism. It is replaced by
// four assertions. The old bar constrained one number on every vertex; these
// constrain flat vertices harder than it did, and constrain the smoothed ones
// in the only way a normal buffer can be checked without a second copy of the
// geometry — by what the surface itself implies:
//
//   1. every normal is unit length, and faces the same hemisphere as its winding
//      (dot(face, n) >= 0.40, so an inverted or garbage normal is still caught,
//      and the bound is tight enough that no legitimate smoothing reaches it);
//   2. a surface that is NOT smoothed is EXACTLY flat — the builder stores the
//      face normal bit for bit, and the recount here reads a drift of at most
//      1e-4 from float32 rounding, where the old bar allowed 2.5 degrees;
//   3. smoothing is DECLARED. mesh.smoothing names the parts that smooth and
//      how many of their vertices carry a normal of their own, and this
//      recounts it from the buffer. A part that smooths without declaring it
//      fails, and a part that declares smoothing it does not do fails too;
//   4. smoothing is SHARED across the vertices it smooths between, which is
//      what makes it smoothing rather than a per-triangle decoration. Written
//      up in full at the bottom of normalsContract, with the measured margins.
//
// Declaration rides beside `parts` rather than inside it because the GLB
// exporter copies part records field by field and check_equipment_import.cjs
// compares them for deep equality.
function normalsContract(mesh) {
  const declared = new Map();
  let previous = -1;
  for (const entry of mesh.smoothing) {
    const part = mesh.parts.find(p => p.name === entry.name);
    assert.ok(part, `smoothing declares an unknown part ${entry.name}`);
    assert.equal(entry.first, part.first); assert.equal(entry.count, part.count);
    assert.ok(entry.vertices > 0 && entry.vertices <= part.count, `smoothing count for ${entry.name}`);
    assert.ok(entry.first > previous, "smoothing is in part order"); previous = entry.first;
    declared.set(entry.name, entry.vertices);
  }
  let smoothTotal = 0, sharedTotal = 0;
  for (const part of mesh.parts) {
    let own = 0;
    // For assertion 4 below: every vertex position in this part, and which of
    // them carry a normal of their own. 0.1 mm buckets, which is two orders
    // below the smallest bevel in the file, so this can only ever group
    // vertices the builder really did place on top of each other.
    const coincident = new Map(), smoothedAt = [];
    for (let i = part.first * 3; i < (part.first + part.count) * 3; i += 9) {
      const a = mesh.positions.subarray(i, i + 3), b = mesh.positions.subarray(i + 3, i + 6), c = mesh.positions.subarray(i + 6, i + 9);
      const u = [b[0] - a[0], b[1] - a[1], b[2] - a[2]], v = [c[0] - a[0], c[1] - a[1], c[2] - a[2]];
      const n = [u[1] * v[2] - u[2] * v[1], u[2] * v[0] - u[0] * v[2], u[0] * v[1] - u[1] * v[0]], area = Math.hypot(...n);
      assert.ok(area > 1e-9, `nondegenerate triangle ${i / 9}`);
      const face = [n[0] / area, n[1] / area, n[2] / area];
      for (let vertex = 0; vertex < 3; vertex++) {
        const at = i + vertex * 3, normal = [mesh.normals[at], mesh.normals[at + 1], mesh.normals[at + 2]];
        assert.ok(Math.abs(Math.hypot(...normal) - 1) < 1e-5, `unit normal at vertex ${at / 3}`);
        const facing = face[0] * normal[0] + face[1] * normal[1] + face[2] * normal[2];
        assert.ok(facing >= 0.40, `normal follows its winding at vertex ${at / 3} (${facing})`);
        // Classification has to be unambiguous or this recount means nothing.
        // The builder stores anything within half a degree of the face normal AS
        // the face normal, so measured drift is either float32 noise or real
        // smoothing, with a gap between them. Measured across all nine platforms
        // and all three levels: flat vertices reach 5.1e-5, smoothed vertices
        // start at 1.0e-2. The band below is deliberately wider than that on the
        // flat side, because an ill-conditioned sliver recomputes its face normal
        // to about 1e-4 and the recount must not read rounding as art.
        const drift = Math.abs(normal[0] - face[0]) + Math.abs(normal[1] - face[1]) + Math.abs(normal[2] - face[2]);
        assert.ok(drift < 1e-3 || drift > 5e-3, `normal at vertex ${at / 3} is neither flat nor smoothed (${drift})`);
        const key = `${Math.round(mesh.positions[at] * 1e4)},${Math.round(mesh.positions[at + 1] * 1e4)},${Math.round(mesh.positions[at + 2] * 1e4)}`;
        const bucket = coincident.get(key); if (bucket) bucket.push(at); else coincident.set(key, [at]);
        if (drift > 2e-3) { own++; smoothedAt.push(at, i, key); }
      }
    }
    if (declared.has(part.name)) assert.equal(own, declared.get(part.name), `${part.name} smooths exactly what it declares`);
    else assert.equal(own, 0, `${part.name} declares no smoothing, so every normal must be its face normal`);
    smoothTotal += own;
    for (let k = 0; k < smoothedAt.length; k += 3) {
      const at = smoothedAt[k], triangle = smoothedAt[k + 1];
      for (const other of coincident.get(smoothedAt[k + 2])) {
        if (other >= triangle && other < triangle + 9) continue;      // its own triangle proves nothing
        if (Math.abs(mesh.normals[other] - mesh.normals[at]) <= 1e-6 && Math.abs(mesh.normals[other + 1] - mesh.normals[at + 1]) <= 1e-6
          && Math.abs(mesh.normals[other + 2] - mesh.normals[at + 2]) <= 1e-6) { sharedTotal++; break; }
      }
    }
  }
  // 4. SMOOTHING IS A PROPERTY OF THE SURFACE, NOT OF THE TRIANGLE.
  // Assertions 1-3 prove a declared normal left its face and that the count is
  // honest. They do NOT prove the departure means anything: give every smoothed
  // vertex a fixed skew off its OWN face normal and the unit length, the facing
  // floor, the drift bands and the declared count all still pass while the
  // shading is nonsense — verified, that corruption passed all 23 checks before
  // this assertion existed. What makes a normal a SMOOTH normal is that the
  // triangles meeting at that point on the surface agree on it; that agreement
  // is the entire reason smoothing removes the seam, and a per-triangle skew
  // cannot produce it. So a smoothed vertex must have a coincident sibling in
  // the same part, in a different triangle, carrying the same normal.
  // Measured: 96.5% of smoothed vertices do (worst mesh ground_artillery LOD0);
  // the rest are honest patch edges — a dome's apex fan, the closing seam of a
  // revolve, the end cap of a sweep — which have no sibling to agree with. The
  // per-face skew above scores at most 66.7%. Hence 90%: 6 points under the
  // real floor, 23 points over the forgery.
  // The bar is per mesh and not per part because on the smallest smoothed parts
  // the two bands overlap — the worst honest part is 85.3% (glacis applique)
  // and the best forged one is 83.3% — so a per-part bar would either go red on
  // real geometry or pass a fake, and neither is worth having.
  // HONEST LIMIT, and a warning for whoever tunes this next. One single rotation
  // applied to the whole smoothed field keeps the agreement intact, so this
  // assertion cannot see it. It is caught anyway — a rotation of even 1.1 deg
  // goes red — but by the facing floor above, and only just: the tightest honest
  // vertex in the whole model measures 0.4019 against a 0.40 bar (a three-segment
  // revolve at LOD2), so that catch has 0.0019 of slack and is luck rather than
  // design. Two things follow. A coarser LOD2, or one more segment removed from
  // a revolve, will go red on honest geometry there. And anyone who relieves that
  // by lowering the 0.40 floor removes the only thing standing between this
  // contract and a rotated normal buffer, so a real rotation test has to be
  // written in its place first.
  if (smoothTotal) assert.ok(sharedTotal / smoothTotal > 0.90,
    `smoothed normals are shared with the coincident vertices they smooth across (${(sharedTotal / smoothTotal * 100).toFixed(1)}%)`);
  return smoothTotal;
}

function validate(mesh, ground = false, level = 0) {
  const air = ground === "air";
  assert.ok(mesh.positions instanceof Float32Array);
  assert.ok(mesh.normals instanceof Float32Array);
  assert.ok(mesh.colors instanceof Float32Array);
  assert.equal(mesh.positions.length, mesh.normals.length);
  assert.equal(mesh.positions.length, mesh.colors.length);
  assert.equal(mesh.positions.length, mesh.triangleCount * 9);
  assert.equal(mesh.lod, level);
  // User-approved aircraft inspection budget: 100k+ triangles, with separate
  // cheaper catalogue/map recipes. Ground budgets and geometric checks stay fixed.
  const [low, high] = air ? [[100000,250000],[4000,18000],[300,2500]][level] : BANDS[level][ground ? "ground" : "tank"];
  assert.ok(mesh.triangleCount >= low && mesh.triangleCount <= high, `${mesh.triangleCount} triangles at LOD${level}`);
  let previous = 0;
  const names = new Set();
  for (const part of mesh.parts) {
    assert.ok(!names.has(part.name), `unique semantic part ${part.name}`); names.add(part.name);
    assert.equal(part.first, previous); assert.equal(part.count % 3, 0); assert.ok(part.count > 0);
    previous = part.first + part.count;
  }
  assert.equal(previous, mesh.positions.length / 3);
  assert.ok(mesh.parts.length >= (air ? 16 : 18));
  for (let i = 0; i < mesh.positions.length; i += 3) {
    for (let j = 0; j < 3; j++) {
      assert.ok(Number.isFinite(mesh.positions[i + j])); assert.ok(Number.isFinite(mesh.normals[i + j]));
      assert.ok(mesh.colors[i + j] >= 0 && mesh.colors[i + j] <= 1);
      assert.ok(mesh.positions[i + j] >= mesh.bounds.min[j] - 1e-5 && mesh.positions[i + j] <= mesh.bounds.max[j] + 1e-5);
    }
  }
  const smoothed = normalsContract(mesh);
  // Curved surfaces are most of a vehicle: wheels, tyres, barrels, drums, domes,
  // cable, hatch lids. If this fraction collapses the model has silently gone
  // back to flat shading, which is the regression the old contract enforced.
  if (level === 0 && !air) assert.ok(smoothed / (mesh.triangleCount * 3) > 0.30, `${(smoothed / (mesh.triangleCount * 3) * 100).toFixed(1)}% of vertices smoothed`);
  assert.ok(mesh.bounds.min[1] >= -0.02 && mesh.bounds.min[1] < 0.06, "track soles meet ground");
  assert.ok(mesh.bounds.max[2] > (ground ? 1.7 : 3) && mesh.bounds.min[2] < -1.7, "forward and aft chassis bounds");
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
    // THE AMMUNITION EXEMPTION IS GONE. This line used to read
    // `if(slot!=='ammunition'&&...)`, which held every slot but one to the bar
    // that a choice must change the triangles — and the one it let through was
    // the slot with the least to show for itself. Every ground ammunition id
    // drew the same locker in the same place and read its label off the
    // selection, so `docs/art/COMPONENT_COVERAGE.md` scored all six absent while
    // this file was green. A contract with a hole in it is a contract that
    // measures where the art already is.
    if(original.specification.components[slot]!==id)assert.notEqual(digest(next),digest(original),id);
  }
});
check('every tank ammunition load draws its own stowage, at the card level too',()=>{
  // The sweep above covers the ground families. The TANK families were never
  // swept for this at all, and not because anyone exempted them: the sweep two
  // checks down reads its slot list from `build({platform})`, whose legacy
  // five-slot resolution carries no `ammunition` key, so the slot fell out of
  // the loop before it could be measured. Between that and the exemption just
  // removed, the whole ammunition catalogue — nine ids across nine hulls — was
  // outside the contract. This is the other half of the repair.
  const loads=[...fs.readFileSync(path.resolve(__dirname,'../../spheres-sim/src/equipment_specs.rs'),'utf8')
    .matchAll(/component!\("([^"]+)","[^"]+","ammunition"/g)].map(m=>m[1]);
  assert.ok(loads.length>=3,`only ${loads.length} tank ammunition loads scraped`);
  const seen=new Set();
  for(const platform of ['tank_standard','tank_heavy','tank_light','tank_destroyer']){
    for(const load of loads){
      const spec={platform,components:{...detailed.components,ammunition:load}};
      const mesh=build(spec);validate(mesh);
      assert.equal(mesh.specification.components.ammunition,load);
      const part=mesh.parts.find(p=>p.slot==='ammunition'&&p.name.includes('ready rack'));
      assert.ok(part,`${platform} ${load} draws no ammunition stowage`);
      assert.ok(part.count>0&&part.label&&!part.label.includes('ammo_'),`${platform} ${load} label ${part.label}`);
      // Distinct from every other load on the same hull, and distinct at the
      // catalogue card too: a difference only the inspection mesh carries is a
      // difference the player never sees.
      for(const other of loads){
        if(other===load)continue;
        const rival={platform,components:{...detailed.components,ammunition:other}};
        assert.notEqual(digest(mesh),digest(build(rival)),`${platform} ${load} against ${other}`);
        assert.notEqual(digest(build({...spec,lod:1})),digest(build({...rival,lod:1})),`${platform} ${load} against ${other} at LOD1`);
      }
      seen.add(`${platform}/${load}`);
    }
  }
  assert.equal(seen.size,4*loads.length);
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

// ---------------------------------------------------------------------------
// Level of detail
// ---------------------------------------------------------------------------
const PLATFORMS = ['tank_standard','tank_heavy','tank_light','tank_destroyer',...groundPlatforms];
const isGround = platform => groundPlatforms.includes(platform);
check('build(spec) still returns the inspection mesh, and lod selects a coarser one',()=>{
  for(const platform of PLATFORMS){
    // The API did not change shape: an untouched spec is still LOD0. `lod` is an
    // extra key on the same object, so every existing caller keeps working.
    assert.equal(digest(build({platform})),digest(build({platform,lod:0})));
    assert.equal(build({platform}).lod,0);
    // Out-of-range, fractional and hostile values clamp instead of throwing,
    // because this value arrives from a renderer's distance heuristic.
    for(const [asked,expected] of [[-4,0],[0.4,0],[1.6,2],[9,2],['2',2],[null,0],[undefined,0],[NaN,0],['nonsense',0]])
      assert.equal(build({platform,lod:asked}).lod,expected,`lod ${String(asked)}`);
  }
});
check('every level keeps the same parts, slots, labels and order so picking survives the swap',()=>{
  for(const platform of PLATFORMS){
    const levels=[0,1,2].map(lod=>build({platform,lod}));
    const shape=mesh=>mesh.parts.map(p=>`${p.name}|${p.slot}|${p.label}`).join('\n');
    assert.equal(shape(levels[1]),shape(levels[0]),`${platform} LOD1 part list`);
    assert.equal(shape(levels[2]),shape(levels[0]),`${platform} LOD2 part list`);
    for(let lod=0;lod<3;lod++){
      validate(levels[lod],isGround(platform),lod);
      assert.deepEqual(build({platform,lod}).parts,levels[lod].parts);
      assert.equal(digest(build({platform,lod})),digest(levels[lod]),`${platform} LOD${lod} determinism`);
      assert.equal(levels[lod].bounds.min[1],0);
    }
    assert.ok(levels[1].triangleCount<levels[0].triangleCount/3,`${platform} LOD1 is a real reduction`);
    assert.ok(levels[2].triangleCount<levels[1].triangleCount/3,`${platform} LOD2 is a real reduction`);
    // A map pin still has to be the right size and stand on the ground: the
    // coarse levels must not shrink the vehicle or lift it off its tracks.
    for(let axis=0;axis<3;axis++)for(const lod of [1,2]){
      assert.ok(Math.abs(levels[lod].bounds.max[axis]-levels[0].bounds.max[axis])<0.35,`${platform} LOD${lod} max ${axis}`);
      assert.ok(Math.abs(levels[lod].bounds.min[axis]-levels[0].bounds.min[axis])<0.35,`${platform} LOD${lod} min ${axis}`);
    }
  }
});
check('a coarse level still answers every component choice with different geometry',()=>{
  // The catalogue card is drawn at LOD1. If a choice stopped being visible there
  // the designer would silently show the player the wrong vehicle.
  for(const [slot,id] of [['armament','gun_125'],['protection','protection_heavy'],['turret','turret_autoload'],['mobility','engine_turbine_1500'],['sensors','optics_thermal']]){
    const next=build({...detailed,components:{...detailed.components,[slot]:id},lod:1});
    assert.notEqual(digest(next),digest(build({...detailed,lod:1})),`${id} at LOD1`);
  }
  for(const [slot,id] of [['wheels','ground_wheels_runflat'],['recon_package','ground_recon_mast'],['protection','ground_armor_modular']]){
    const spec={platform:'ground_recon',components:{...ground.ground_recon.specification.components,[slot]:id}};
    assert.notEqual(digest(build({...spec,lod:1})),digest(build({platform:'ground_recon',lod:1})),`${id} at LOD1`);
  }
  assert.equal(build({platform:'ground_recon',components:{wheels:'ground_wheels_runflat'},lod:2}).parts.filter(p=>p.name.includes('road tire')).length,8);
});

// ---------------------------------------------------------------------------
// The coarse bands, over the whole option space
// ---------------------------------------------------------------------------
// WHY THIS EXISTS, and it is not a tidying-up. The band check above ran on nine
// DEFAULT builds and nothing else. Every other LOD assertion in this file reads
// a digest or a part list, and the one place a coarse level was built from a
// non-default spec — the run-flat scout two checks up, built at LOD2 to count
// its eight road tyres — never called validate() on what it built, so its
// triangle count was never looked at. It was 1,764 against a 1,500 ceiling, and
// a loaded scout reached 1,808: the map pin for every wheeled platform the
// player could actually field was over budget and the suite was green. A band
// that is only asked of the default configuration is not a band, because the
// default configuration is the one build a designer never ships.
//
// So: every catalogue id, on every platform that has its slot, at LOD1 and at
// LOD2, through the same validate() the default builds go through — which is
// the band, the winding, the flat/smooth classification, the declared smoothing
// count and the shared-normal floor, not just a triangle total. Part identity
// is checked across the two coarse levels at the same time, because a fitting
// that only appears at one distance is the same defect wearing a different hat.
const CATALOGUE = [...fs.readFileSync(path.resolve(__dirname,'../../spheres-sim/src/equipment_specs.rs'),'utf8').matchAll(/component!\("([^"]+)","[^"]+","([^"]+)"/g),
  ...fs.readFileSync(path.resolve(__dirname,'../../spheres-sim/src/equipment_ground.rs'),'utf8').matchAll(/component!\("([^"]+)","[^"]+","([^"]+)"/g)];
const BY_SLOT = new Map();
for (const [, id, slot] of CATALOGUE) { if (!BY_SLOT.has(slot)) BY_SLOT.set(slot, new Set()); BY_SLOT.get(slot).add(id); }
const shapeOf = mesh => mesh.parts.map(p => `${p.name}|${p.slot}|${p.label}`).join('\n');
check('every catalogue option holds the coarse triangle bands on every platform that fields it',()=>{
  let swept = 0;
  for (const platform of PLATFORMS) {
    const fitted = build({ platform }).specification.components;
    for (const [slot, ids] of BY_SLOT) {
      // A slot the platform does not carry resolves back to the default build,
      // which is already covered; sweeping it would only reread the same mesh.
      if (!(slot in fitted)) continue;
      for (const id of ids) {
        const components = { ...fitted, [slot]: id };
        const levels = [0, 1, 2].map(lod => build({ platform, components, lod }));
        for (const mesh of levels.slice(1)) validate(mesh, isGround(platform), mesh.lod);
        for (const lod of [1, 2]) {
          assert.equal(shapeOf(levels[lod]), shapeOf(levels[0]), `${platform} ${id} LOD${lod} part list`);
          assert.ok(levels[lod].triangleCount < levels[lod - 1].triangleCount / 3, `${platform} ${id} LOD${lod} is a real reduction`);
          // Reach is not detail. A coarse level may drop a fitting, but the
          // fitting that sets the vehicle's own extent has to survive, or the
          // pin is drawn at the wrong size and in the wrong place. This is how
          // a short 90 mm gun was caught losing 900 mm of barrel at LOD2 while
          // its triangle count sat comfortably inside the band.
          for (let axis = 0; axis < 3; axis++) {
            assert.ok(Math.abs(levels[lod].bounds.max[axis] - levels[0].bounds.max[axis]) < 0.35, `${platform} ${id} LOD${lod} max ${axis}`);
            assert.ok(Math.abs(levels[lod].bounds.min[axis] - levels[0].bounds.min[axis]) < 0.35, `${platform} ${id} LOD${lod} min ${axis}`);
          }
        }
        swept++;
      }
    }
  }
  // Guard the guard: if a rename ever empties the catalogue scrape this check
  // would pass by sweeping nothing at all.
  assert.ok(swept >= 200, `only ${swept} platform/option pairs swept`);
});
check('the coarse bands survive a fully loaded vehicle, not only one changed slot',()=>{
  // One option at a time is the cheap sweep and it misses the case that broke:
  // run-flats put a scout over the LOD2 ceiling on their own, and a scout also
  // carrying a mast, modular armour and an autocannon turret was further over
  // still. These are the heaviest combination each platform can be given.
  const loaded = [
    ['tank_heavy', { mobility:'engine_turbine_1500', transmission:'transmission_auto', tracks:'tracks_wide', suspension:'suspension_hydro', turret:'turret_heavy', armament:'gun_125', ammunition:'ammo_penetrator', protection:'protection_heavy', active_protection:'aps_hard', sensors:'optics_thermal', fire_control:'fcs_digital', communications:'comms_data' }],
    ['tank_standard', { turret:'turret_autoload', armament:'gun_125', protection:'protection_heavy', active_protection:'aps_hard', sensors:'sensors_integrated', communications:'comms_data', tracks:'tracks_padded' }],
    ['tank_destroyer', { turret:'turret_casemate', armament:'gun_120', protection:'protection_heavy', active_protection:'aps_hard', sensors:'optics_thermal' }],
    ['tank_light', { turret:'turret_compact', armament:'gun_90', protection:'protection_heavy', active_protection:'aps_hard', sensors:'optics_night' }],
    ['ground_recon', { wheels:'ground_wheels_runflat', recon_package:'ground_recon_mast', protection:'ground_armor_modular', turret:'ground_turret_autocannon', armament:'ground_gun_35', ammunition:'ground_ammo_autocannon', suspension:'suspension_hydro', sensors:'optics_thermal', active_protection:'aps_hard', communications:'ground_comms_network' }],
    ['ground_apc', { wheels:'ground_wheels_runflat', protection:'ground_armor_modular', troop_compartment:'ground_troops_protected', turret:'ground_turret_autocannon', armament:'ground_gun_35', suspension:'suspension_hydro', sensors:'optics_thermal', active_protection:'aps_hard', communications:'ground_comms_network' }],
    ['ground_ifv', { protection:'ground_armor_modular', troop_compartment:'ground_troops_protected', tracks:'tracks_wide', turret:'ground_turret_autocannon', armament:'ground_gun_35', suspension:'suspension_hydro', sensors:'optics_thermal', active_protection:'aps_hard', communications:'ground_comms_network' }],
    ['ground_artillery', { protection:'ground_armor_modular', tracks:'tracks_wide', armament:'ground_howitzer_155', artillery_loader:'ground_loader_assisted', suspension:'suspension_hydro', sensors:'optics_thermal', active_protection:'aps_hard', communications:'ground_comms_network' }],
    ['ground_air_defense', { protection:'ground_armor_modular', tracks:'tracks_wide', armament:'ground_aa_missiles', radar:'ground_radar_tracking', ammunition:'ground_ammo_missiles', suspension:'suspension_hydro', sensors:'optics_thermal', active_protection:'aps_hard', communications:'ground_comms_network' }]
  ];
  for (const [platform, components] of loaded) {
    const levels = [0, 1, 2].map(lod => build({ platform, components, lod }));
    for (const mesh of levels) validate(mesh, isGround(platform), mesh.lod);
    assert.equal(shapeOf(levels[1]), shapeOf(levels[0]), `${platform} loaded LOD1 part list`);
    assert.equal(shapeOf(levels[2]), shapeOf(levels[0]), `${platform} loaded LOD2 part list`);
    assert.ok(levels[1].triangleCount < levels[0].triangleCount / 3, `${platform} loaded LOD1 is a real reduction`);
    assert.ok(levels[2].triangleCount < levels[1].triangleCount / 3, `${platform} loaded LOD2 is a real reduction`);
    for (let axis = 0; axis < 3; axis++) for (const lod of [1, 2]) {
      assert.ok(Math.abs(levels[lod].bounds.max[axis] - levels[0].bounds.max[axis]) < 0.35, `${platform} loaded LOD${lod} max ${axis}`);
      assert.ok(Math.abs(levels[lod].bounds.min[axis] - levels[0].bounds.min[axis]) < 0.35, `${platform} loaded LOD${lod} min ${axis}`);
    }
  }
});
check('a wheeled platform pays for its extra road tyres inside the coarse budget',()=>{
  // The specific regression, named so it cannot be lost in the sweep above.
  // Eight run-flats instead of six is 33% more running gear, and at LOD2 the
  // segment counts are already on the floor, so the saving has to come from
  // fewer turned SURFACES per wheel and not from thinner ones.
  for (const platform of ['ground_recon', 'ground_apc']) {
    const runflat = build({ platform, components: { wheels: 'ground_wheels_runflat' }, lod: 2 });
    assert.equal(runflat.parts.filter(p => p.name.includes('road tire')).length, 8);
    validate(runflat, true, 2);
    const [, ceiling] = BANDS[2].ground;
    assert.ok(runflat.triangleCount < ceiling * 0.92,
      `${platform} run-flat map pin is ${runflat.triangleCount} triangles against a ${ceiling} ceiling, with no headroom left for the next art pass`);
  }
});

// ---------------------------------------------------------------------------
// The real LOD0 ceiling
// ---------------------------------------------------------------------------
check('the reference exports stay under the byte cap check_equipment_export.cjs enforces',()=>{
  // This is the constraint that actually limits LOD0, and it is not a triangle
  // budget. The exporter writes three attributes x three vertices x three floats
  // x four bytes = 108 bytes for every triangle, with no index buffer, and
  // The user-requested September inspection pass changes the shipping contract
  // to 12 MB per tank and 5 MB per specialist. The additional bytes carry paired
  // wheels, guide horns, cooling louvers and service fittings, not subdivisions.
  // The 32 KB JSON reserve stays intact, as do every catalogue/map LOD budget.
  // These are art delivery budgets, not loosened geometry correctness checks.
  const perTriangle=108,header=32768,tankCap=(12000000-header)/perTriangle,groundCap=(5000000-header)/perTriangle;
  const catalogue={mobility:'engine_diesel_900',transmission:'transmission_manual',tracks:'tracks_standard',suspension:'suspension_torsion',turret:'turret_standard',armament:'gun_105',ammunition:'ammo_mixed',protection:'protection_standard',active_protection:'aps_none',sensors:'optics_day',fire_control:'fcs_basic',communications:'comms_radio'};
  const exported=[
    {platform:'tank_standard',components:catalogue},
    {platform:'tank_standard',components:{...catalogue,mobility:'engine_diesel_1200'}},
    {platform:'tank_heavy',components:{...catalogue,mobility:'engine_diesel_1200',protection:'protection_heavy',turret:'turret_heavy',armament:'gun_120'}},
    {platform:'tank_light',components:{...catalogue,mobility:'engine_diesel_600',turret:'turret_compact',armament:'gun_90'}},
    {platform:'tank_destroyer',components:{...catalogue,turret:'turret_casemate',armament:'gun_120',ammunition:'ammo_penetrator'}}
  ];
  for(const spec of exported)assert.ok(build(spec).triangleCount<tankCap,`${spec.platform} exports ${build(spec).triangleCount} triangles against a ${Math.round(tankCap)} ceiling`);
  for(const platform of groundPlatforms)assert.ok(ground[platform].triangleCount<groundCap,`${platform} exports ${ground[platform].triangleCount} triangles against a ${Math.round(groundCap)} ceiling`);
});

check('a specialist that fields two ammunition loads shows which one it carries',()=>{
  // The removed exemption holds each load against the platform DEFAULT. That is
  // not the same question as whether two loads a player can actually choose
  // between look different from each other, which is the question the designer
  // asks: the artillery hull fields a high-explosive and a guided load, and both
  // could have differed from nothing while being identical to one another.
  const rival={ground_recon:['ground_ammo_ball','ground_ammo_autocannon'],
    ground_artillery:['ground_ammo_he','ground_ammo_guided'],
    ground_air_defense:['ground_ammo_aa','ground_ammo_missiles']};
  for(const [platform,loads] of Object.entries(rival)){
    const of=(ammunition,lod)=>build({platform,components:{...ground[platform].specification.components,ammunition},lod});
    for(const lod of [0,1]){
      const meshes=loads.map(load=>of(load,lod));
      assert.notEqual(digest(meshes[0]),digest(meshes[1]),`${platform} ammunition loads at LOD${lod}`);
    }
    for(const load of loads){
      const mesh=of(load,0);validate(mesh,true);
      const part=mesh.parts.find(p=>p.slot==='ammunition');
      assert.ok(part&&part.label&&!part.label.includes('ground_'),`${platform} ${load} stowage label`);
    }
  }
});
check('the committed component coverage report still matches the geometry',()=>{
  // docs/art/COMPONENT_COVERAGE.md is the only place that says which component
  // choices actually change the model. A generated document that nothing checks
  // is a stale document with a build command in its header, so this runs the
  // generator's own --check: if the art moved and the report did not, this goes
  // red here rather than being discovered by a player who selected a component
  // and saw nothing happen.
  const tool=path.resolve(__dirname,'build_component_coverage.cjs');
  assert.ok(fs.existsSync(tool),'the coverage generator is missing');
  const run=require('node:child_process').spawnSync(process.execPath,[tool,'--check'],{encoding:'utf8'});
  assert.equal(run.status,0,`node tools/ui/build_component_coverage.cjs --check failed:\n${run.stdout}${run.stderr}`);
  assert.match(run.stdout,/0 weak \/ 0 absent/,`the report still records holes:\n${run.stdout}`);
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

const smoothedShare = mesh => mesh.smoothing.reduce((sum, entry) => sum + entry.vertices, 0) / (mesh.triangleCount * 3);
process.stdout.write(`${checks} equipment mesh checks passed; baseline ${base.triangleCount.toLocaleString("en-US")} triangles, ` +
  `${(smoothedShare(base) * 100).toFixed(1)}% of its vertices smoothed, ` +
  `LOD1 ${build({ ...baseline, lod: 1 }).triangleCount.toLocaleString("en-US")}, LOD2 ${build({ ...baseline, lod: 2 }).triangleCount.toLocaleString("en-US")}.\n`);
