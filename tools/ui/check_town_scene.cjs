"use strict";
const test = require('node:test'), assert = require('node:assert/strict');
const town = require('../../spheres-web/ui/town-mesh.js');
const renderer = require('../../spheres-web/ui/arsenal3d.js');
const cases = [{id:1997,district:'mixed'}, {id:1994,district:'residential'},
  ...town.districts().map(district=>({id:'1990',district}))];

test('scene lots reassemble unchanged close geometry, finishes and exact placements',()=>{
  for(const options of cases){
    const full=town.block(options),scene=town.scene(options);
    assert.equal(scene.rawCloseTriangles,full.triangleCount);
    assert.deepEqual(scene.bounds,full.bounds);
    assert.equal(scene.lots.length,full.lots.length);
    for(const lot of scene.lots){
      const original=full.parts.find(p=>p.lot===lot.index),mesh=lot.mesh('close');
      assert.equal(original.count,mesh.positions.length/3);
      for(let i=0;i<mesh.positions.length;i++){
        const j=original.first*3+i;
        assert(Math.abs(mesh.positions[i]+lot.offset[i%3]-full.positions[j])<1e-5,`${scene.id}/${lot.id} position ${i}`);
        assert.equal(mesh.normals[i],full.normals[j]);
        assert.equal(mesh.colors[i],full.colors[j]);
      }
      for(const tier of ['close','mid','map']){
        const mesh=lot.mesh(tier);
        assert.equal(mesh.triangleCount,lot.triangles[tier]);
        assert.equal(mesh.id,lot.meshKey(tier));
        for(let i=0;i<3;i++){
          assert(mesh.bounds.min[i]+lot.offset[i]>=lot.bounds.min[i]-1e-5);
          assert(mesh.bounds.max[i]+lot.offset[i]<=lot.bounds.max[i]+1e-5);
        }
      }
    }
    const start=full.parts.find(p=>p.lot===-1).first*3;
    assert.deepEqual(scene.background.positions,full.positions.slice(start));
    assert.deepEqual(scene.background.normals,full.normals.slice(start));
    assert.deepEqual(scene.background.colors,full.colors.slice(start));
  }
});

test('the camera governs real visible lots across viewport, density, orbit and zoom',()=>{
  let demotions=0,culled=0,close=0,map=0;
  for(const options of cases.slice(0,2)){
    const scene=town.scene(options);
    for(const [w,h] of [[320,280],[640,400],[1280,800],[1100,1100]])for(const dpr of [1,2])
      for(const yaw of [0,34,90,145,270])for(const zoom of [.5,1,1.6,3,6]){
        const plan=renderer.scenePlan(scene,{width:Math.min(2048,w*dpr),height:Math.min(2048,h*dpr),yaw,zoom});
        assert(plan.triangles<=150000);
        assert.equal(plan.rawCloseTriangles,scene.rawCloseTriangles);
        assert.equal(plan.triangles,scene.background.triangleCount+plan.draws.reduce((n,d)=>n+d.lot.triangles[d.tier],0));
        assert.equal(plan.draws.length+plan.culled.length,scene.lots.length);
        assert.equal(new Set([...plan.draws.map(d=>d.id),...plan.culled]).size,scene.lots.length);
        for(const lot of scene.lots) assert.equal(!!renderer.projectBox(lot.bounds,plan.mvp,plan.width,plan.height),plan.draws.some(d=>d.id===lot.id));
        demotions+=plan.demotions;culled+=plan.culled.length;
        close+=plan.draws.filter(d=>d.tier==='close').length;map+=plan.draws.filter(d=>d.tier==='map').length;
      }
    for(const lot of scene.lots)for(const yaw of [0,90,210]){
      const plan=renderer.scenePlan(scene,{width:640,height:400,selected:lot.id,yaw});
      assert.equal(plan.draws.find(d=>d.id===lot.id)?.tier,'close');
      assert(plan.triangles<=150000);
    }
  }
  assert(demotions>0,'must exercise actual budget demotion');
  assert(culled>0&&close>0&&map>0,'must exercise frustum rejection and all detail tiers');
});

test('LOD hysteresis and homogeneous clipping are conservative',()=>{
  assert.equal(renderer.sceneTier(235,'close'),'close');
  assert.equal(renderer.sceneTier(235,'mid'),'mid');
  assert.equal(renderer.sceneTier(42,'map'),'map');
  assert.equal(renderer.sceneTier(42,'mid'),'mid');
  assert.equal(renderer.sceneTier(209,'close'),'mid');
  assert.equal(renderer.sceneTier(35,'mid'),'map');
  const identity=[1,0,0,0,0,1,0,0,0,0,1,0,0,0,0,1];
  assert.equal(renderer.projectBox({min:[2,0,0],max:[3,1,1]},identity,640,400),null);
  assert(renderer.projectBox({min:[-2,-2,-2],max:[2,2,2]},identity,640,400));
});

test('city browsing bounds CPU variants and eviction does not alter deterministic scenes',()=>{
  const before=town.scene(cases[0]);
  const first=before.lots[0].mesh('close');
  for(let id=30;id<45;id++) town.scene({id,district:town.districts()[id%town.districts().length]});
  const stats=town.cacheStats();assert(stats.triangles<=stats.cap);
  const after=town.scene(cases[0]);
  assert.deepEqual(after.lots[0].mesh('close').positions,first.positions);
  assert.equal(before.rawCloseTriangles,after.rawCloseTriangles);
});
