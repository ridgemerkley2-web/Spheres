'use strict';
const {test}=require('node:test');
const assert=require('node:assert/strict');
const crypto=require('node:crypto');
const site=require('../../spheres-web/ui/site-mesh.js');
const stages=['site','foundation','frame','enclosed','complete'];

test('site surface optimization preserves every original map mesh byte',()=>{
  const all=crypto.createHash('sha256');
  for(const kind of site.kinds())for(const stage of stages)for(let level=1;level<=5;level++)for(const status of ['building','paused']){
    const mesh=site.build(kind,stage,{lod:1,level,status,variant:11});
    const hash=crypto.createHash('sha256');
    for(const values of [mesh.positions,mesh.normals,mesh.colors])hash.update(Buffer.from(values.buffer,values.byteOffset,values.byteLength));
    hash.update(JSON.stringify([mesh.bounds,mesh.parts,mesh.shading]));
    all.update(hash.digest('hex'));
  }
  // All 800 far configurations, recorded before the near-surface optimization.
  assert.equal(all.digest('hex'),'65203b2a4d68a8d3131b924314bf63155c57435c77a97705f02d7ff43192de66');
});

test('completed perimeters retain four corner posts without coincident faces',()=>{
  for(const kind of site.kinds())for(const level of [1,5]){
    const mesh=site.build(kind,'complete',{level}),part=mesh.parts.find(p=>p.name==='yard / permanent perimeter');
    const faces=new Set(),vertices=[],p=mesh.positions;
    for(let i=part.first*3;i<(part.first+part.count)*3;i+=9){
      const points=[0,3,6].map(k=>Array.from(p.subarray(i+k,i+k+3)));
      const key=points.map(v=>v.map(x=>x.toFixed(5)).join(',')).sort().join('|');
      assert(!faces.has(key),`${kind}/L${level}: duplicate perimeter triangle`);
      faces.add(key);vertices.push(...points);
    }
    for(const sx of [-1,1])for(const sz of [-1,1]){
      const x=sx*(mesh.compound.width/2-.8),z=sz*(mesh.compound.depth/2-.8);
      assert(vertices.some(p=>Math.abs(p[0]-x)<=.101&&Math.abs(p[2]-z)<=.101&&Math.abs(p[1]-3.36)<1e-5),`${kind}/L${level}: missing corner post`);
    }
  }
});
