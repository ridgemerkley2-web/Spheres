'use strict';
const {test}=require('node:test');
const assert=require('node:assert/strict');
const fs=require('node:fs');
const path=require('node:path');
const vm=require('node:vm');
const crypto=require('node:crypto');
const {measureBuildingUnits}=require('./art-budget-units.cjs');
const town=require('../../spheres-web/ui/town-mesh.js');
const site=require('../../spheres-web/ui/site-mesh.js');
const source=fs.readFileSync(path.join(__dirname,'../../spheres-web/ui/site-mesh.js'),'utf8');
function load(text){const context={module:{exports:{}}};vm.runInNewContext(text,context);return context.module.exports;}
function buffers(mesh, first=0, count=mesh.positions.length/3){
  const hash=crypto.createHash('sha256');
  for(const a of [mesh.positions,mesh.normals,mesh.colors])hash.update(Buffer.from(a.buffer,a.byteOffset+first*12,count*12));
  return hash.digest('hex');
}

test('industrial compounds own every triangle and retain each physical building across stages',()=>{
  for(const kind of site.kinds())for(const stage of site.stages(kind))for(const level of [1,5])for(const status of ['building','blocked']){
    const mesh=site.build(kind,stage.key,{level,status}),at=measureBuildingUnits(mesh,kind+'/'+stage.key);
    assert.equal(at.assemblyTriangles,mesh.triangleCount);
    for(let i=1;i<level;i++)assert(at.buildings.some(b=>b.id==='wing-'+i),kind+': missing delivered wing');
    assert(at.buildings.every(b=>b.triangles<=12000),kind+': actual building exceeds the unchanged 12k limit');
    const props=mesh.budgetUnits.filter(u=>u.kind==='prop');
    assert(props.length>0,kind+': yard and construction props are accounted separately');
    if(stage.key==='complete'){
      const lead=at.buildings.find(b=>b.id==='lead');
      assert(lead && lead.triangles>0,kind+': keep the full lead building, its envelope and attached roof plant');
      const roof=mesh.parts.find(p=>p.name==='services / roof plant and access walkway');
      assert(mesh.budgetUnits.some(u=>u.first===roof.first&&u.count===roof.count&&u.buildingIds.includes('lead')));
    }
  }
});

test('terraces and campuses keep exact pre-accounting geometry and conservative shared ownership',()=>{
  const hash=crypto.createHash('sha256');
  for(const kind of ['row_house','university'])for(const width of town.kindInfo(kind).widths)
    for(const storeys of new Set(town.kindInfo(kind).storeys))for(const lod of ['close','mid','map'])for(const id of [0,1]){
      const mesh=town.building(kind,{width,storeys,lod,id});
      for(const a of [mesh.positions,mesh.normals,mesh.colors])hash.update(Buffer.from(a.buffer));
      hash.update(JSON.stringify([mesh.bounds,mesh.parts]));
      if(lod==='map'){assert.equal(mesh.budgetUnits,undefined);continue;}
      const at=measureBuildingUnits(mesh,kind);
      assert.equal(at.physicalBuildings,kind==='row_house'?Math.round(width/5.9):3);
      assert(at.buildings.every(b=>b.triangles<=12000));
      if(kind==='row_house'){
        const shared=mesh.budgetUnits.filter(u=>u.kind==='shared'&&u.buildingIds.length===at.physicalBuildings);
        assert(shared.length>0,'the continuous envelope must retain all dwelling owners');
        const envelope=shared.reduce((n,u)=>n+u.count/3,0);
        assert(at.buildings.every(b=>b.sharedTriangles>=envelope),'charge the whole common roof to each dwelling');
        assert(at.buildings.reduce((n,b)=>n+b.triangles,0)>at.totals.building+at.totals.shared,'do not divide shared cost');
      }
    }
  // Pinned before budget ownership was authored; metadata cannot change art.
  assert.equal(hash.digest('hex'),'27adf4678fb861172ba892d85f50869b86707f5c4abfe548d9a136af5f92cc8d');
});

test('only completely enclosed main portal interiors disappear, retaining every other part byte',()=>{
  const marker='const sealedInterior = d0.fine';
  assert(source.includes(marker));
  const reference=load(source.replace(marker,'const sealedInterior = false && d0.fine'));
  for(const kind of site.kinds())for(const stage of site.stages(kind))for(const lod of [0,1]){
    const options={level:5,status:'blocked',lod},a=reference.build(kind,stage.key,options),b=site.build(kind,stage.key,options);
    if(stage.key!=='complete'||lod){assert.equal(buffers(a),buffers(b),kind+'/'+stage.key+'/'+lod);continue;}
    assert.equal(a.triangleCount-b.triangleCount,980,kind+': five enclosed interior portals');
    assert.deepEqual(JSON.parse(JSON.stringify(a.bounds)),b.bounds,kind+': preserve the silhouette bounds');
    assert.equal(a.parts.length,b.parts.length);
    for(let i=0;i<a.parts.length;i++){
      const pa=a.parts[i],pb=b.parts[i];assert.equal(pa.name,pb.name);
      if(pa.name==='structure / portal frames and haunches')continue;
      assert.equal(pa.count,pb.count,kind+'/'+pa.name);
      assert.equal(buffers(a,pa.first,pa.count),buffers(b,pb.first,pb.count),kind+'/'+pa.name);
    }
  }
});

test('every removed portal triangle lies inside its real opaque wall, roof and slab envelope',()=>{
  const exposed=load(source.replace('    build, stages, kinds, meta, stageFor,','    build, stages, kinds, meta, stageFor, _test: {Mesh, portal, KINDS, P},'));
  const {Mesh,portal,KINDS,P}=exposed._test;
  const triangles=m=>{const out=new Map();for(let i=0;i<m.pos.length;i+=9){const p=m.pos.slice(i,i+9),c=m.col.slice(i,i+9);out.set(JSON.stringify([p,c]),p);}return out;};
  for(const k of Object.values(KINDS)){
    const {w,dz,eaves,ridge}=k.block,deck=.78,a=new Mesh(),b=new Mesh();
    portal(a,0,w,deck,eaves,ridge,P.steel,false);portal(b,0,w,deck,eaves,ridge,P.steel,true);
    const old=triangles(a),current=triangles(b);
    for(const key of current.keys())assert(old.has(key),'retained portal surface must be byte-identical');
    let removed=0;
    for(const [key,points] of old){if(current.has(key))continue;removed++;
      for(let i=0;i<9;i+=3){const x=points[i],y=points[i+1],z=points[i+2];
        assert(Math.abs(x)<=w/2+1e-9,'behind the opaque side walls');
        assert(Math.abs(z)<dz/2-.14,'inside both opaque gables');
        assert(y>=deck,'above the solid ground slab');
        assert(y<deck+ridge-(ridge-eaves)*Math.abs(x)/(w/2),'below both opaque roof slopes');
      }
    }
    assert.equal(removed,196,'only the authored inner flange, bolts, haunch, rafters and apex plate');
  }
});
