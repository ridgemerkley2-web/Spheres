'use strict';
const {test}=require('node:test');
const assert=require('node:assert/strict');
const fs=require('node:fs');
const path=require('node:path');
const {maxedSpec}=require('./bench_art.cjs');
const {build}=require('../../spheres-web/ui/equipment-mesh.js');
const root=path.resolve(__dirname,'../..');
const components=['equipment.rs','equipment_specs.rs','equipment_ground.rs'].flatMap(file=>
  [...fs.readFileSync(path.join(root,'spheres-sim/src',file),'utf8')
    .matchAll(/component!\("([^"]+)","[^"]+","([^"]+)"/g)].map(m=>[m[1],m[2]]));
assert(components.length>=60,'The test needs the real component catalogue');
const slots=['mobility','transmission','tracks','suspension','turret','armament','ammunition',
  'protection','active_protection','sensors','fire_control','communications'].sort();
const loaded={mobility:'engine_turbine_1500',transmission:'transmission_auto',tracks:'tracks_wide',
  suspension:'suspension_hydro',turret:'turret_heavy',armament:'gun_125',ammunition:'ammo_penetrator',
  protection:'protection_heavy',active_protection:'aps_hard',sensors:'optics_thermal',
  fire_control:'fcs_digital',communications:'comms_data'};

test('worst-spec search includes every accepted tank slot omitted by the default',()=>{
  const catalogueBefore=JSON.stringify(components);
  for(const platform of ['tank_standard','tank_heavy','tank_light','tank_destroyer']){
    const result=maxedSpec(platform,components);
    assert.deepEqual(Object.keys(result.spec.components).sort(),slots,platform+' searches all twelve slots');
    assert.deepEqual(result.mesh.specification.components,result.spec.components,'Search output must contain only normalized accepted choices');
    assert.equal(result.base.triangleCount,build({platform}).triangleCount,'Baseline remains the actual unconfigured model');
    assert(result.mesh.triangleCount>=result.base.triangleCount);
    for(const [slot,id] of Object.entries(result.spec.components))assert(
      components.some(row=>row[0]===id&&row[1]===slot)||result.base.specification.components[slot]===id,
      `${platform}/${slot}: only native catalogue entries or the generator's existing default aliases`);
    if(platform==='tank_heavy'){
      const full=build({platform,components:loaded});
      assert.equal(full.triangleCount,79828,'Reviewed fully loaded inspection fixture');
      assert(result.mesh.triangleCount>=full.triangleCount,'Search must not under-report the known fully loaded fixture');
    }
  }
  assert.equal(JSON.stringify(components),catalogueBefore);
});

test('worst-spec search rejects incompatible generator slots instead of fabricating components',()=>{
  for(const platform of ['ground_recon','ground_ifv','ground_artillery']){
    const result=maxedSpec(platform,components),resolved=build(result.spec).specification;
    assert.deepEqual(result.spec.components,resolved.components);
    assert.equal(Object.keys(resolved.components).length,13);
    assert.equal('wheels' in resolved.components,platform==='ground_recon');
    assert.equal('tracks' in resolved.components,platform!=='ground_recon');
    assert.equal('artillery_loader' in resolved.components,platform==='ground_artillery');
    assert.equal('troop_compartment' in resolved.components,platform==='ground_ifv');
    assert(result.mesh.triangleCount>=result.base.triangleCount);
  }
});
