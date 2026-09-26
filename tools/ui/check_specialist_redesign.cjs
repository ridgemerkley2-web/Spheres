'use strict';
const {test}=require('node:test'),assert=require('node:assert/strict'),crypto=require('node:crypto');
// The override permits a real red run against the archived previous source.
const {build}=require(process.env.SPHERES_SPECIALIST_AUDIT_MESH||'../../spheres-web/ui/equipment-mesh.js');
const {raycast}=require('../../spheres-web/ui/equipment-model.js');
const ground=['ground_ifv','ground_apc','ground_recon','ground_artillery','ground_air_defense'];
const hash=m=>{const h=crypto.createHash('sha256');for(const key of ['positions','normals','colors','materialClasses'])h.update(Buffer.from(m[key].buffer));return h.digest('hex');};
// Tactical LOD1/2 pins reflect the reviewed September 22 coarse optimization;
// aircraft inspection sidecars remain unchanged. Tank LOD0 pins record removal
// of sealed track-pin caps, verified by check_equipment_connector_optimization.
const approved={
  "tank_standard/0": "67232b2d303fe76c37827b1f1e65876e3347e8c840934f889bab4fd076175848",
  "tank_standard/1": "b274d23ccb22110a5d130207f2d0ad118553f2e3b4cff650d50e935dd35ea214",
  "tank_standard/2": "e6b413a7db639a76c42d7343f8fb45325c7b3717c4371389994cd010c3e1c2c7",
  "tank_heavy/0": "77ecf283c8cb60d9542498e2fb5e90fc07fb5f497e0bbfd8a15a8c9e82673a60",
  "tank_heavy/1": "630d16da3f2e389021b2aec6790bd8600561a8e89cb9c1c43fa24b628e69ccd4",
  "tank_heavy/2": "3d685892766222155637764cb2281fb9e31c1efabd4407c4bf010aa54df243d0",
  "tank_light/0": "63762adfc5ebefb6213f31813bd2b935e078bc58c2e60d5265713391ad101615",
  "tank_light/1": "d882ac1f41def90481cb6e32ce15d43ad5e20c58c2763eac94e337d76aaa2ee5",
  "tank_light/2": "8dd91b1dad7688e70f2a4c67a6cde5ee4e2018102c550141e91a882963bec8dc",
  "tank_destroyer/0": "67232b2d303fe76c37827b1f1e65876e3347e8c840934f889bab4fd076175848",
  "tank_destroyer/1": "b274d23ccb22110a5d130207f2d0ad118553f2e3b4cff650d50e935dd35ea214",
  "tank_destroyer/2": "e6b413a7db639a76c42d7343f8fb45325c7b3717c4371389994cd010c3e1c2c7",
  "air_light_attack/0": "626cb63c944adf487b6d3746c100aa7b7d8c3523531ac27eb6b3f54f59cd0801",
  "air_light_attack/1": "d6701d02d852080cba870e1de5a706ec1b13c486081f6d780b6b22a47b951fae",
  "air_light_attack/2": "c1902656dea412d725693a8c4b3b52118d42749c06482e15b20a2d42de905d88",
  "air_tactical_strike/0": "9ff8123ae78043163cd78d154f054daad2f11c077138efabd37eaa2bff2c7569",
  "air_tactical_strike/1": "b5f018a05f291035da6bf020669eb6c96630d61e6958c26d7dcdacd37f063655",
  "air_tactical_strike/2": "a7b1dbc4dbae7abe1786640c1bec7b5c81e27dff82954ab2420db710c739b17e"
};
function vertices(m,name,color){const p=m.parts.find(p=>typeof name==='string'?p.name===name:name(p));assert(p,String(name));const out=[];for(let i=p.first;i<p.first+p.count;i++)if(!color||color.every((v,k)=>Math.abs(m.colors[i*3+k]-v)<1e-6))out.push({p:Array.from(m.positions.subarray(i*3,i*3+3)),n:Array.from(m.normals.subarray(i*3,i*3+3)),material:m.materialClasses[i]});return out;}
function span(v,k){const nums=v.map(v=>v.p[k]);return [Math.min(...nums),Math.max(...nums)];}

test('approved tanks and aircraft retain exact geometry, colors and authored material classes',()=>{
  for(const [key,want]of Object.entries(approved)){const [platform,lod]=key.split('/');assert.equal(hash(build({platform,lod:Number(lod)})),want,key);}
});
test('specialist primary hull proportions are long enough for their distinct vehicle roles at every LOD',()=>{
  const minimum={ground_ifv:6.40,ground_apc:6.25,ground_recon:4.80,ground_artillery:7.00,ground_air_defense:6.30};
  for(const platform of ground)for(const lod of [0,1,2]){
    const m=build({platform,lod}),v=vertices(m,'protection / specialist sloped hull'),z=span(v,2);
    assert(z[1]-z[0]>minimum[platform],`${platform} LOD${lod} retains its authored long hull`);
  }
});
test('IFV fighting compartment is a low continuous shell with converging cheeks',()=>{
  const m=build({platform:'ground_ifv'}),v=vertices(m,'turret / autocannon turret'),y=span(v,1),z=span(v,2);
  assert((z[1]-z[0])/(y[1]-y[0])>2.45,'Turret has a broad low body instead of a tall octagonal box');
  const roof=vertices(m,'turret / autocannon turret',[.35*1.08,.40*1.08,.27*1.08]).filter(v=>v.n[1]>.999);
  const front=Math.max(...roof.map(v=>v.p[2])),wide=Math.max(...roof.map(v=>Math.abs(v.p[0]))),mouth=Math.max(...roof.filter(v=>Math.abs(v.p[2]-front)<1e-5).map(v=>Math.abs(v.p[0])));
  assert(wide>.6&&mouth/wide<.4,'The broad crew roof tapers to a narrow gun joint');
});
test('tracked specialists have distinct belt return gears beyond their paired road wheels',()=>{
  for(const platform of ['ground_ifv','ground_artillery','ground_air_defense']){
    const m=build({platform}),belt=span(vertices(m,'tracks / starboard continuous articulated belt'),2),last=platform==='ground_artillery'?7:6;
    const front=span(vertices(m,`running gear / starboard road wheel ${last}`),2),rear=span(vertices(m,'running gear / starboard road wheel 1'),2);
    assert(belt[1]-front[1]>.35&&rear[0]-belt[0]>.35,`${platform} road tyres leave real space for both end gears`);
    const wheelRanges=Array.from({length:last},(_,i)=>span(vertices(m,`running gear / starboard road wheel ${i+1}`),2));
    for(let i=1;i<wheelRanges.length;i++)assert(wheelRanges[i][0]-wheelRanges[i-1][1]>.004,`${platform} adjacent road tyres must not intersect`);
    const tireX=span(vertices(m,'running gear / starboard road wheel 1'),0),discPlane=tireX[1]-(tireX[1]-tireX[0])*.12+.018;
    const gear=vertices(m,'tracks / starboard continuous articulated belt',[.20,.22,.20]).filter(v=>v.p[2]>0&&Math.abs(v.p[0]-discPlane)<1e-5);
    assert(gear.length>20,'The clearance check must inspect the outer rim of the physical end disc');
    assert(span(gear,2)[0]>front[1]+.015,`${platform} front drive disc clears the last road tyre`);
  }
});
test('wheeled tire shoulder treads are rubber and open beads expose the actual painted wheel centers',()=>{
  for(const platform of ['ground_apc','ground_recon'])for(const wheels of ['ground_wheels_standard','ground_wheels_runflat']){
    const m=build({platform,components:{wheels}}),count=wheels==='ground_wheels_runflat'?4:3;
    for(const side of [-1,1])for(let j=1;j<=count;j++){
      const name=`running gear / ${side<0?'port':'starboard'} road tire ${j}`,v=vertices(m,name),y=span(v,1),z=span(v,2);
      assert(!v.some(v=>v.material===2),'Road tire tread must never receive tracked steel material');
      const hit=raycast(m,[side*5,(y[0]+y[1])/2,(z[0]+z[1])/2],[-side,0,0]);assert(hit);assert.equal(hit.part.name,name);
      assert.equal(m.materialClasses[hit.vertex],0,'A side view must see the painted wheel center through the bead opening');
    }
  }
});
test('specialist bow plates remain flat at every LOD without a coarse cylindrical bumper',()=>{
  for(const platform of ground)for(const lod of [0,1,2]){
    const m=build({platform,lod}),v=vertices(m,'protection / specialist sloped hull',[.32,.37,.25]);assert(v.length>=96);
    for(let i=0;i<v.length;i+=3)for(let j=1;j<3;j++)assert(v[i].n.every((n,k)=>Math.abs(n-v[i+j].n[k])<1e-6),'Folded armor is a planar surface');
  }
});
test('air-defense guns sit in outboard cradles and radar panels are non-optical surfaces',()=>{
  const m=build({platform:'ground_air_defense'}),gun=vertices(m,p=>p.slot==='armament'),x=span(gun,0);
  assert(x[0]<-.95&&x[1]>.95,'Twin cannon cradles are physically distinct on either side of the mount');
  for(const radar of ['ground_radar_search','ground_radar_tracking']){
    const model=build({platform:'ground_air_defense',components:{radar}}),array=vertices(model,p=>p.slot==='radar');
    assert(!array.some(v=>v.material===4),'Radar antenna/radome must not be rendered as a giant sight lens');
  }
});
