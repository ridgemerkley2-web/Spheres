#!/usr/bin/env node
'use strict';
const {test}=require('node:test'),assert=require('node:assert/strict'),crypto=require('node:crypto');
const fs=require('node:fs'),path=require('node:path'),vm=require('node:vm');
const {build}=require('../../spheres-web/ui/equipment-mesh.js');
const platforms=['tank_standard','tank_heavy','tank_light','tank_destroyer','ground_ifv','ground_apc','ground_recon','ground_artillery','ground_air_defense','air_light_attack','air_tactical_strike'];
const meshes=new Map(platforms.map(platform=>[platform,build({platform})]));
const hash=m=>crypto.createHash('sha256').update(Buffer.from(m.positions.buffer)).update(Buffer.from(m.normals.buffer)).update(Buffer.from(m.colors.buffer)).digest('hex');
const shape=m=>m.parts.map(p=>[p.name,p.slot,p.label]);
const rubber=[.055,.068,.060],steel=[.20,.22,.20],airPaint=[.37,.41,.36];
function vertices(mesh,part,color){
  const found=mesh.parts.find(p=>typeof part==='string'?p.name===part:part(p));assert(found,`Missing part ${part}`);
  const points=[];
  for(let i=found.first*3;i<(found.first+found.count)*3;i+=3){
    if(!color||color.every((v,j)=>Math.abs(mesh.colors[i+j]-v)<1e-6))points.push(Array.from(mesh.positions.subarray(i,i+3)));
  }
  return points;
}
// Ground coarse hashes deliberately reflect each authored proportion redesign.
// The approved tank pins remain unchanged; specialist-before sources and GLBs
// are archived in ../specialist-reference-research. No geometry budget, normal
// requirement or cross-LOD bounds tolerance changed in either redesign.
const coarseHashes={
  tank_standard:["d66b71d005b4e25e376a8cb6411c3b961ef350a2a2411ac781c066c3003c206f","e5760810cb00879db26dcfc36f77827819638cc32a4f45a80117e1d7ec156601"],
  tank_heavy:["6a25e3d4840b0107b10f57b8edfa8e0731ae8262c221f8005e439c4e371d505c","8b8e7bca0b48db6a474aaf6b66b6a355cebe2e9f596fb412c0f95bffed795759"],
  tank_light:["3c1e3753147c54edbcf73c6124de406dc6881a51ede3d26c929ff5a51561bd29","de1f56f7b408a3e34b75bb9568d2417b0cd79acf1155c79284016dcbd46366e8"],
  tank_destroyer:["d66b71d005b4e25e376a8cb6411c3b961ef350a2a2411ac781c066c3003c206f","e5760810cb00879db26dcfc36f77827819638cc32a4f45a80117e1d7ec156601"],
  ground_ifv:["35bae9a60e93112a15e554f7177fbecdd1e84024ec4580c98ed5970d18b61c11","e92e4ccd6e2ee26da60662a0b1a30bd9b75e46eb3580da978326dcc77c128611"],
  ground_apc:["c2bb341e5c551bba3d2e067ea083ba23c58df57922fcfa76d4955750141461af","b01c4f24487a5de0f5cbe7edf5a50c8b21560434dee78d522c7d6065a78ad37c"],
  ground_recon:["2e3009c7bc5c3704813cee2aa6f34bc45c1261c8a317778ab4056371f7c05cc2","7a5bd0559ddebc51842140f0beea3be6c78cd735fabff792eeab596250eaffd3"],
  ground_artillery:["f78b20e414d4d9036de6044c59d5cddfecd543c5ea1c59d5cd3148aed0861001","3c731bf952b76bb78bf0e90deb6511ee77d9263ae5bf63cc5fcf3af384c5c411"],
  ground_air_defense:["36c715bf0fcb113a2eebb6f393706aeda342db8b93e24eec3b531a9b17bdf774","d897cdd61f676e5b10ddc35f2838ee5ea7c43e71d05fede729d856f708f1bb4e"]
};
test('all18 ground coarse meshes match their reviewed baselines and preserve part identities',()=>{
  for(const [platform,hashes] of Object.entries(coarseHashes))for(const lod of [1,2]){
    const mesh=build({platform,lod});assert.equal(hash(mesh),hashes[lod-1],`${platform} LOD${lod}`);
    assert.deepEqual(shape(mesh),shape(meshes.get(platform)),`${platform} picking survives LOD${lod}`);
  }
});
test('paired tank road wheels leave a real central rubber-free guide channel',()=>{
  const mesh=meshes.get('tank_standard'),part='running gear / starboard suspension, road wheels and sprockets';
  const wheel=vertices(mesh,part,rubber).filter(p=>Math.abs(p[2]+2.32)<.34&&p[1]<.86);
  assert(wheel.length>100,'The assertion must inspect a real road tyre');
  assert(!wheel.some(p=>Math.abs(p[0]-1.49)<.10),'Rubber may not bridge the central horn channel');
  assert(wheel.some(p=>p[0]<1.49-.15)&&wheel.some(p=>p[0]>1.49+.15),'Both paired tyres must exist');
  const coarse=vertices(build({platform:'tank_standard',lod:1}),part,rubber);
  assert(coarse.some(p=>Math.abs(p[0]-1.49)<.10),'The coarse silhouette remains its original inexpensive continuous tyre');
});
test('tracked specialists use paired tyres while wheeled families keep single road tires',()=>{
  for(const platform of ['ground_ifv','ground_artillery','ground_air_defense']){
    const mesh=meshes.get(platform),centerX=platform==='ground_artillery'?1.52:1.37;
    const tyre=vertices(mesh,'running gear / starboard road wheel 1',rubber);
    assert(tyre.length>100);assert(!tyre.some(p=>Math.abs(p[0]-centerX)<.09),`${platform} guide channel`);
    assert(tyre.some(p=>p[0]<centerX-.1)&&tyre.some(p=>p[0]>centerX+.1));
  }
  for(const platform of ['ground_apc','ground_recon']){
    const mesh=meshes.get(platform),centerX=platform==='ground_apc'?1.18:1.05;
    assert(vertices(mesh,'running gear / starboard road tire 1',rubber).some(p=>Math.abs(p[0]-centerX)<.09),`${platform} retains a single continuous tire`);
  }
});
test('tank guide horns taper toward their tips instead of hiding rectangular blocks between wheels',()=>{
  const mesh=meshes.get('tank_standard'),x=1.49,z=2.78;
  // The longer hull increases shoe pitch; inspect the complete authored foot.
  const halfDepth=(4*2.78+Math.PI*2*.65)/84*.42/2+.001;
  const horn=vertices(mesh,'running gear / starboard individual tread links and pins',steel)
    .filter(p=>Math.abs(p[0]-x)<.039&&Math.abs(p[2]-z)<halfDepth&&p[1]>1);
  assert(horn.length>12);
  const levels=new Map();for(const p of horn){const key=p[1].toFixed(5);levels.set(key,Math.max(levels.get(key)||0,Math.abs(p[0]-x)));}
  const heights=[...levels].map(([y,r])=>[Number(y),r]).sort((a,b)=>a[0]-b[0]);
  assert.equal(heights.length,3,'A seated foot, shoulder and narrow tip form the horn');
  assert(Math.abs(heights[2][0]-heights[0][0]-.16)<1e-4);
  assert(heights[0][1]<heights[1][1]&&heights[1][1]<heights[2][1]);
  assert(heights[0][1]/heights[2][1]<.25,'The tip must fit the central channel');
});
test('aircraft wings have curved span-varying airfoil volume rather than flat slabs',()=>{
  for(const platform of ['air_light_attack','air_tactical_strike']){
    const mesh=meshes.get(platform),strike=platform==='air_tactical_strike',w=strike?.77:.58,span=strike?6.15:5.15;
    const x=w*.76+(span-w*.76)*.2;
    const points=vertices(mesh,p=>p.name.startsWith('air_wing / starboard ')&&p.name.endsWith('wing'),airPaint).filter(p=>Math.abs(p[0]-x)<1e-5);
    assert(new Set(points.map(p=>p[1].toFixed(5))).size>=24,`${platform} upper and lower curved section`);
    const height=Math.max(...points.map(p=>p[1]))-Math.min(...points.map(p=>p[1]));assert(height>.18&&height<.24,'The wing has a useful but bounded root section');
  }
});
test('aircraft fuselage and canopy normals are shared curved surfaces',()=>{
  for(const platform of ['air_light_attack','air_tactical_strike']){
    const mesh=meshes.get(platform);
    for(const prefix of ['air_wing / airframe','air_avionics /']){
      const part=mesh.parts.find(p=>p.name.startsWith(prefix)),entry=mesh.smoothing.find(p=>p.name===part.name);
      assert(entry&&entry.vertices>part.count*.30,`${platform} ${prefix} must shade as a curved surface`);
    }
  }
});
test('every inspection triangle is finite, nondegenerate, grounded and faces its declared normals',()=>{
  for(const [platform,mesh] of meshes){
    assert.equal(mesh.bounds.min[1],0);assert.equal(mesh.positions.length,mesh.triangleCount*9);
    for(let i=0;i<mesh.positions.length;i+=9){
      const p=mesh.positions,a=[p[i+3]-p[i],p[i+4]-p[i+1],p[i+5]-p[i+2]],b=[p[i+6]-p[i],p[i+7]-p[i+1],p[i+8]-p[i+2]];
      const cross=[a[1]*b[2]-a[2]*b[1],a[2]*b[0]-a[0]*b[2],a[0]*b[1]-a[1]*b[0]],area=Math.hypot(...cross);
      assert(area>1e-9,`${platform} triangle${i/9} has usable area`);
      for(let j=0;j<9;j+=3){
        const n=Array.from(mesh.normals.subarray(i+j,i+j+3));assert(n.every(Number.isFinite));assert(Math.abs(Math.hypot(...n)-1)<1e-5);
        assert(cross.reduce((v,c,k)=>v+c*n[k],0)/area>=.40,`${platform} triangle${i/9} winding`);
        for(let k=0;k<3;k++){assert(Number.isFinite(p[i+j+k]));assert(p[i+j+k]>=mesh.bounds.min[k]-1e-5&&p[i+j+k]<=mesh.bounds.max[k]+1e-5);}
      }
    }
  }
});
test('inspection detail preserves immutable specifications and deterministic geometry',()=>{
  for(const [platform,mesh] of meshes){
    const input={platform,components:{...mesh.specification.components}};Object.freeze(input.components);Object.freeze(input);
    assert.equal(hash(build(input)),hash(mesh));assert.deepEqual(build(input).specification,mesh.specification);
    assert.equal(new Set(mesh.parts.map(p=>p.name)).size,mesh.parts.length);
    let end=0;for(const p of mesh.parts){assert.equal(p.first,end);assert(p.count>0&&p.count%3===0);end+=p.count;}assert.equal(end,mesh.positions.length/3);
  }
});
test('extra detail respects meter scale and the light tank remains the scaled platform',()=>{
  const standard=meshes.get('tank_standard'),light=meshes.get('tank_light');
  assert.equal(standard.positions.length,light.positions.length);
  for(let i=0;i<standard.positions.length;i++)assert(Math.abs(light.positions[i]-standard.positions[i]*.8)<1e-6);
  for(const [platform,mesh] of meshes){assert(mesh.bounds.max[0]-mesh.bounds.min[0]<14,`${platform} width in meters`);assert(mesh.bounds.max[2]-mesh.bounds.min[2]<20,`${platform} length in meters`);}
});
test('hydropneumatic tanks remain visibly distinct under the unchanged component coverage metric',()=>{
  // Exercise the actual shared occupancy measurement, not a private lower bar.
  // Stop before its file-writing report sweep: this test must remain read-only.
  const filename=path.join(__dirname,'build_component_coverage.cjs'),source=fs.readFileSync(filename,'utf8');
  const start=source.indexOf('const rows = [], baselines = [];');assert(start>0,'Coverage measurement export boundary');
  const context={require,Buffer,__dirname:path.dirname(filename)};
  vm.runInNewContext(source.slice(0,start)+'\nthis.measureOne = measure;',context,{filename});
  for(const platform of ['tank_standard','tank_heavy','tank_light','tank_destroyer']){
    const row=context.measureOne(platform,'suspension','suspension_hydro');
    assert.equal(row.verdict,'distinct',`${platform} hydro outline is only ${row.area}m²`);
    assert.equal(row.added,0,'Upgrade clarity must come from actual geometry, not new picking IDs');
    assert.equal(row.removed,0);
  }
});
