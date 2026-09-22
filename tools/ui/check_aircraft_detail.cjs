const {test}=require('node:test');
const assert=require('node:assert/strict');
const {build}=require('../../spheres-web/ui/equipment-mesh.js');
const platforms=['air_light_attack','air_fighter','air_tactical_strike'];
const loaded={air_engine:'air_engine_efficient',air_wing:'air_wing_stable',air_radar:'air_radar_mapping',air_avionics:'air_avionics_digital',air_countermeasures:'air_countermeasures_ecm',air_payload:'air_payload_guided',air_fuel:'air_fuel_extended'};
test('default and loaded aircraft meet the 100k inspection contract and bounded cheaper LODs',()=>{
  for(const platform of platforms)for(const components of [{},loaded]){
    const levels=[0,1,2].map(lod=>build({platform,components,lod}));
    assert(levels[0].triangleCount>=100000&&levels[0].triangleCount<=250000);
    assert(levels[1].triangleCount<18000&&levels[1].triangleCount<levels[0].triangleCount/8);
    assert(levels[2].triangleCount<2500&&levels[2].triangleCount<levels[1].triangleCount/4);
    for(const [lod,m] of levels.entries()){
      assert.equal(m.lod,lod);
      assert.deepEqual(m.specification,levels[0].specification);
      assert.deepEqual(m.parts.map(p=>[p.name,p.slot,p.label]),levels[0].parts.map(p=>[p.name,p.slot,p.label]));
      assert(m.parts.every(p=>p.count>0),'no missing selectable assembly');
      let end=0;for(const p of m.parts){assert.equal(p.first,end);end+=p.count;}assert.equal(end,m.positions.length/3);
      for(const values of [m.positions,m.normals,m.colors])assert(values.every(Number.isFinite));
      for(let i=0;i<m.normals.length;i+=3)assert(Math.abs(Math.hypot(m.normals[i],m.normals[i+1],m.normals[i+2])-1)<1e-5);
    }
  }
});
test('aircraft LOD clamps malformed requests and never edits the design',()=>{
  const spec={platform:platforms[0],components:loaded,lod:2},before=JSON.stringify(spec);
  build(spec);assert.equal(JSON.stringify(spec),before);
  assert.equal(build({...spec,lod:-5}).lod,0);assert.equal(build({...spec,lod:99}).lod,2);assert.equal(build({...spec,lod:'bad'}).lod,0);
});

test('tactical default and fully loaded designs meet the unchanged catalogue and map ceilings',()=>{
  // Pre-optimization bounds protect the full wing span, radome, tail and
  // nozzle silhouette while cheaper details use fewer surface samples.
  const bounds={
    default:[{min:[-5.859448726356644,0,-8.542],max:[5.859448726356644,4.074999999999999,9.188]},
      {min:[-5.854956224390981,0,-8.232],max:[5.859448726356644,3.8395395575739943,8.568000000000001]}],
    loaded:[{min:[-6.6274999999999995,0,-8.542],max:[6.6274999999999995,4.074999999999999,9.438]},
      {min:[-6.6028790987816155,0,-8.232],max:[6.604293532306935,3.8395395575739943,8.818000000000001]}]
  };
  for(const [preset,components] of [['default',{}],['loaded',loaded],['loaded',{...loaded,air_engine:'air_engine_twin'}],['loaded',{...loaded,air_engine:'air_engine_twin',air_avionics:'air_avionics_analog'}]]){
    for(const [lod,ceiling] of [[1,12000],[2,1500]]){
      const mesh=build({platform:'air_tactical_strike',components,lod});
      assert.equal(mesh.triangleCount,mesh.positions.length/9,'measure the actual emitted triangles');
      assert(mesh.triangleCount<=ceiling,`${preset}/${components.air_engine||'default'}/LOD${lod}: ${mesh.triangleCount} > ${ceiling}`);
      assert.deepEqual(mesh.bounds,bounds[preset][lod-1]);
      assert.equal(new Set(mesh.parts.map(part=>part.slot)).size,8,'all design slots remain selectable');
    }
  }
});
