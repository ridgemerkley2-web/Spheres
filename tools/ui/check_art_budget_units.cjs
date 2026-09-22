'use strict';
const {test}=require('node:test');
const assert=require('node:assert/strict');
const {measureBuildingUnits}=require('./art-budget-units.cjs');

function fixture() {
  return {positions:new Float32Array(6*9), budgetBuildings:[{id:'west',label:'West dwelling'},{id:'east',label:'East dwelling'}],
    budgetUnits:[
      {id:'roof',kind:'shared',buildingIds:['west','east'],first:0,count:6},
      {id:'west-wall',kind:'building',buildingIds:['west'],first:6,count:3},
      {id:'east-wall',kind:'building',buildingIds:['east'],first:9,count:3},
      {id:'yard',kind:'terrain',buildingIds:[],first:12,count:3},
      {id:'car',kind:'prop',buildingIds:[],first:15,count:3}
    ]};
}

test('shared envelope is charged fully to each dwelling and once to the scene',()=>{
  const r=measureBuildingUnits(fixture());
  assert.equal(r.assemblyTriangles,6);
  assert.equal(r.physicalBuildings,2);
  assert.deepEqual(r.buildings.map(b=>[b.triangles,b.ownedTriangles,b.sharedTriangles]),[[3,1,2],[3,1,2]]);
  assert.deepEqual(r.totals,{building:2,shared:2,terrain:1,prop:1});
});

test('constituent accounting rejects hidden costs, overlap and invented owners',()=>{
  for (const [edit,pattern] of [
    [m=>m.budgetUnits.pop(),/unassigned geometry/],
    [m=>m.budgetUnits[1].first=3,/overlapping/],
    [m=>m.budgetUnits[0].buildingIds.push('unknown'),/unknown building/],
    [m=>m.budgetUnits[0].buildingIds=[],/no owners/],
    [m=>m.budgetUnits[1].buildingIds=['west','east'],/one owner/],
    [m=>m.budgetUnits[4].buildingIds=['west'],/nonbuilding/],
    [m=>m.budgetUnits[1].count=2,/invalid vertex/],
    [m=>m.budgetUnits[1].id='roof',/duplicate/],
    [m=>m.budgetBuildings.push({id:'empty',label:'Empty'}),/no geometry/],
  ]) {
    const m=fixture(); edit(m); assert.throws(()=>measureBuildingUnits(m),pattern);
  }
});
