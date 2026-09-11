const {test}=require('node:test');
const assert=require('node:assert/strict');
const {build}=require('../../spheres-web/ui/equipment-mesh.js');
const platforms=['air_light_attack','air_tactical_strike'];
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
