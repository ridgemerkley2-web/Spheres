'use strict';
const {test}=require('node:test');
const assert=require('node:assert/strict');
const {BUDGETS,verdict,budgetFailed,verifyBudgets,gradeExport}=require('./bench_art.cjs');

test('aircraft inspection gate enforces the requested quality floor as well as the ceiling',()=>{
  const budget=BUDGETS.aircraft_lod0;
  for(const [tris,failed] of [[99999,true],[100000,false],[250000,false],[250001,true]]) {
    assert.equal(budgetFailed({...verdict(tris,budget),budget}),failed,`${tris} aircraft triangles`);
  }
  assert.equal(budgetFailed({...verdict(99,BUDGETS.building_far),budget:BUDGETS.building_far}),false,
    'ordinary density guidance must not force unnecessary geometry');
});

test('inspection reconciliation does not relax the building or scene ceilings',()=>{
  verifyBudgets();
  for(const [budget,tris] of [[BUDGETS.building_near,12001],[BUDGETS.building_far,801],
    [BUDGETS.scene,150001],[BUDGETS.vehicle_lod1,12001],[BUDGETS.vehicle_lod2,1501],
    [BUDGETS.specialist_lod0,48001],[BUDGETS.vehicle_lod0,150001]]) {
    assert.equal(budgetFailed({...verdict(tris,budget),budget}),true,`${budget.row}: ${tris}`);
  }
});

test('serialized export budgets use the actual strict byte limits for each family',()=>{
  for(const [file,limit] of [['spheres-tank-heavy.glb',12000000],
    ['spheres-ground-ifv.glb',5000000],['spheres-air-fighter.glb',28000000]]) {
    assert.equal(gradeExport({file,bytes:limit-1}).state,'PASS');
    assert.equal(gradeExport({file,bytes:limit}).state,'OVER');
  }
  assert.throws(()=>gradeExport({file:'unknown.glb',bytes:100}),/unclassified/);
  assert.throws(()=>gradeExport({file:'spheres-tank-heavy.glb',bytes:0}),/invalid/);
});
