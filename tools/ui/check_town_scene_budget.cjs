'use strict';
const test=require('node:test'),assert=require('node:assert/strict');
const town=require('../../spheres-web/ui/town-mesh.js');
const renderer=require('../../spheres-web/ui/arsenal3d.js');
const {measureTownScene,BUDGETS,budgetFailed}=require('./bench_art.cjs');

test('town budget sweep measures the renderer, includes actual default framing, and retains every selection',()=>{
  const scene=town.scene({id:'1990',district:'residential'}),original=renderer.scenePlan;
  const vertexCounts=new Map();
  for(const lot of scene.lots)for(const tier of ['close','mid','map'])
    vertexCounts.set(lot.meshKey(tier),lot.mesh(tier).positions.length/9);
  const selected=new Set(),pitches=new Set(),viewports=new Set();let maximum=0,calls=0;
  renderer.scenePlan=function(model,options){
    assert.equal(model,scene);
    selected.add(options.selected);pitches.add(options.pitch);viewports.add(options.width+'x'+options.height);
    const plan=original(model,options);
    const submitted=model.background.positions.length/9+plan.draws.reduce((n,item)=>n+vertexCounts.get(item.lot.meshKey(item.tier)),0);
    assert.equal(submitted,plan.triangles,'budget must include the complete background and every submitted lot');
    assert.equal(plan.drawCalls,plan.draws.length+1);
    assert.equal(plan.draws.length+plan.culled.length,scene.lots.length);
    if(options.selected)assert.equal(plan.draws.find(d=>d.id===options.selected)?.tier,'close');
    maximum=Math.max(maximum,submitted);calls++;return plan;
  };
  let measured;try{measured=measureTownScene(scene);}finally{renderer.scenePlan=original;}
  assert.equal(measured.hi.tris,maximum);assert.equal(measured.views,calls);
  assert.equal(measured.budget,BUDGETS.scene);assert.equal(budgetFailed({...measured.v,budget:measured.budget}),false);
  assert.deepEqual(selected,new Set([null,...scene.lots.map(lot=>lot.id)]));
  assert(pitches.has(35),'gallery scene default pitch must be sampled');
  assert(viewports.has('2048x1440'),'largest sampled device viewport must be retained');
});

test('unrenderable background cannot be silently removed or accepted by the draw gate',()=>{
  const scene=town.scene({id:1997,district:'mixed'});
  assert.throws(()=>measureTownScene({...scene,background:{...scene.background,triangleCount:150001}}),/cannot fit its draw budget/);
  assert(scene.rawCloseTriangles>150000,'raw complete close scene remains visible diagnostic evidence');
});
