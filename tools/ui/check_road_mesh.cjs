// Exercise the transport network kit. The contract this file defends is not
// "the segments look like roads": it is that a SEGMENT KIT is a kit — that
// every piece declares where it can be joined, that the declaration matches the
// geometry it came from, and that two compatible pieces meet with no gap and no
// overlap however long the chain gets. A road that is one metre out at the
// hundredth segment is not a road, and nothing but an assertion catches that.
//
// The budgets are the ones a repeated segment has to live inside: 200..1,200
// triangles near, 20..120 at map range, and under 5,000 for an assembled
// kilometre at map range. They are asserted at every piece, every detail level,
// every deflection, every hand, every gradient and several indices, because a
// budget asked at one variant is a budget that cannot fail.
const {test}=require('node:test');
const assert=require('node:assert/strict');
const crypto=require('node:crypto');
const fs=require('node:fs');
const path=require('node:path');
const vm=require('node:vm');
const file=path.resolve(__dirname,'../../spheres-web/ui/road-mesh.js');
const road=require(file);
const source=fs.readFileSync(file,'utf8');

const NEAR_MIN=200,NEAR_MAX=1200,MAP_MIN=20,MAP_MAX=120,KM_MAX=5000;
// The crease limit the shading contract rests on, WRITTEN HERE rather than read
// off the module. Reading it from the thing under test made the winding
// assertion below self-referential — setting the module's limit to -1, which
// welds a top face to the bottom face beneath it, left the check green because
// the bar moved with the defect. The bar is stated, and the module's value is
// asserted to still be it.
const CREASE=0.35;
// A millimetre. The seam tolerance the brief asks for, and the tolerance every
// exact-join assertion below uses.
const MM=0.001;
// The parts that carry the CROSS-SECTION — the swept surface a neighbour has to
// meet. Markings, furniture, track and safety hardware are dressing: paint may
// start and stop at a seam, a catenary wire may exist on one side and not the
// other, and neither is a hole in the road.
const SURFACE=new Set(['carriageway','formation','approach','deck']);
// Pieces with more than two ports reconcile two alignments (a junction mouth,
// a turnout's widening ballast pad, a crossing) and cannot hold every port to
// the exact cross-section contract. They are held to the weaker one, and the
// bound is asserted so a regression that made it worse still goes red.
const MULTI=new Set(['road_junction','road_crossroads','rail_switch','level_crossing']);
const MULTI_OVERHANG=0.45;

const digest=m=>crypto.createHash('sha256').update(Buffer.from(m.positions.buffer)).update(Buffer.from(m.colors.buffer)).update(Buffer.from(m.normals.buffer)).digest('hex');

/// Every option combination the kit can be asked for, per piece. Written once
/// so a defect that only appears on a left-hand one-degree bend at map detail
/// is exercised rather than hoped about.
function variants(piece){
  const info=road.info(piece),out=[];
  for(const lod of ['near','map']){
    for(const index of [0,1,2,3,4,5,6,7])out.push({lod,index});
    if(info.straight){
      for(const length of [4,10,25,40,info.maxLength])for(const rise of [0,1,-1,3])out.push({lod,length,rise,base:2});
      // A FALL DEEPER THAN THE FORMATION IT STARTS ON. Every gradient above is
      // pinned at base 2 and never asks for more than a metre of fall, so the
      // ground-contact assertion below could not fire: base 0 with rise -8 was
      // authored eight metres under grade and `finish` lifted the whole piece,
      // ports and all. These are the variants that catch that.
      for(const rise of [-0.2,-2,-8])for(const base of [0,1,8])out.push({lod,rise,base,length:25});
      out.push({lod,sign:true});
    }
    if(info.deflections)for(const turn of info.deflections)for(const hand of ['left','right'])out.push({lod,turn,hand,base:1.5});
    if(info.structure)for(const base of [2.5,5,12,20])out.push({lod,base});
    if(info.atGrade)out.push({lod,base:3});
    if(info.electrified)out.push({lod,catenary:false});
    if(info.modes)for(const mode of info.modes)out.push({lod,mode});
    if(!info.straight&&!info.structure&&!info.atGrade)out.push({lod,base:3});
  }
  return out;
}

function validate(mesh,label){
  const map=mesh.lod==='map';
  assert(mesh.positions instanceof Float32Array,`${label}: positions buffer`);
  assert(mesh.normals instanceof Float32Array,`${label}: normals buffer`);
  assert(mesh.colors instanceof Float32Array,`${label}: colors buffer`);
  assert.equal(mesh.positions.length,mesh.normals.length,`${label}: normal alignment`);
  assert.equal(mesh.positions.length,mesh.colors.length,`${label}: color alignment`);
  assert.equal(mesh.positions.length,mesh.triangleCount*9,`${label}: complete triangles`);
  for(let i=0;i<mesh.positions.length;i+=3){
    for(let j=0;j<3;j++){
      assert(Number.isFinite(mesh.positions[i+j]),`${label}: finite position at ${i+j}`);
      assert(Number.isFinite(mesh.normals[i+j]),`${label}: finite normal at ${i+j}`);
      assert(mesh.colors[i+j]>=0&&mesh.colors[i+j]<=1,`${label}: color in 0..1 at ${i+j}`);
      assert(mesh.positions[i+j]>=mesh.bounds.min[j]-1e-5&&mesh.positions[i+j]<=mesh.bounds.max[j]+1e-5,`${label}: vertex inside bounds`);
    }
    assert(Math.abs(Math.hypot(mesh.normals[i],mesh.normals[i+1],mesh.normals[i+2])-1)<1e-5,`${label}: unit normal at ${i}`);
  }
  for(let i=0;i<mesh.positions.length;i+=9){
    const a=mesh.positions.subarray(i,i+3),b=mesh.positions.subarray(i+3,i+6),c=mesh.positions.subarray(i+6,i+9);
    const u=[b[0]-a[0],b[1]-a[1],b[2]-a[2]],v=[c[0]-a[0],c[1]-a[1],c[2]-a[2]];
    const n=[u[1]*v[2]-u[2]*v[1],u[2]*v[0]-u[0]*v[2],u[0]*v[1]-u[1]*v[0]],area=Math.hypot(...n);
    assert(area>1e-9,`${label}: nondegenerate triangle ${i/9}`);
    // Smoothing is crease-limited, so a folded corner normal can lean away from
    // its own face but never past the limit. That bound is the shading contract
    // and it is what makes a smoothed vertex safe to light with.
    for(let k=0;k<3;k++){
      const d=n.reduce((sum,value,j)=>sum+value/area*mesh.normals[i+k*3+j],0);
      assert(d>CREASE-1e-6,`${label}: normal follows winding at ${i/9}.${k} (${d.toFixed(3)})`);
    }
  }
  // Ground contact is exact and not a tolerance: a piece is placed on terrain
  // by its base. `groundDrop` is the correction `finish` WOULD have applied, so
  // asserting it is zero catches a mis-authored piece instead of letting the
  // module silently shift it and move its ports with it.
  assert.equal(mesh.bounds.min[1],0,`${label}: ground contact at Y=0`);
  assert.equal(mesh.groundDrop,0,`${label}: authored at grade, no silent correction`);
  assert(mesh.bounds.max[1]>0.05,`${label}: has height`);
  const budget=map?[MAP_MIN,MAP_MAX]:[NEAR_MIN,NEAR_MAX];
  assert(mesh.triangleCount>=budget[0]&&mesh.triangleCount<=budget[1],`${label}: ${mesh.triangleCount} triangles outside ${budget[0]}..${budget[1]}`);
  let previous=0;const names=new Set();
  for(const part of mesh.parts){
    assert(!names.has(part.name),`${label}: unique part ${part.name}`);names.add(part.name);
    assert.equal(part.first,previous,`${label}: contiguous part ${part.name}`);
    assert(part.count>0&&part.count%3===0,`${label}: whole triangles in ${part.name}`);
    assert(part.group&&part.label,`${label}: part semantics on ${part.name}`);
    previous=part.first+part.count;
  }
  assert.equal(previous,mesh.positions.length/3,`${label}: parts cover every vertex`);
  assert(mesh.parts.length>=1,`${label}: at least one named part`);
  assert(SURFACE.has(mesh.parts[0].group),`${label}: first part is the swept cross-section, not ${mesh.parts[0].group}`);
}

/// Ports as geometry rather than as a promise: is there material at the port
/// plane, does it cover the width the port claims, and does anything stick out
/// past it into whatever gets mated there.
function planeReport(mesh,port,frame){
  const f=port.forward,o=port.position,right=[f[2],0,-f[0]];
  let beyond=0,on=0,lo=Infinity,hi=-Infinity;
  for(let i=0;i<mesh.positions.length;i+=3){
    let p=[mesh.positions[i],mesh.positions[i+1],mesh.positions[i+2]];
    if(frame)p=road.apply(frame,p);
    const d=(p[0]-o[0])*f[0]+(p[1]-o[1])*f[1]+(p[2]-o[2])*f[2];
    if(d>beyond)beyond=d;
    if(Math.abs(d)<=MM){on++;const u=(p[0]-o[0])*right[0]+(p[2]-o[2])*right[2];if(u<lo)lo=u;if(u>hi)hi=u;}
  }
  return {beyond,on,span:on?hi-lo:0};
}
/// The set of cross-section vertices sitting exactly on a port plane, keyed to
/// a tenth of a millimetre. Two pieces join with no gap and no overlap when
/// these two sets are EQUAL — not when the two port records agree, which is a
/// claim about the table rather than about the geometry.
function seamSet(mesh,port,frame){
  const f=port.forward,o=port.position,out=new Set();
  for(const part of mesh.parts){
    if(!SURFACE.has(part.group))continue;
    for(let v=part.first;v<part.first+part.count;v++){
      const i=v*3;
      let p=[mesh.positions[i],mesh.positions[i+1],mesh.positions[i+2]];
      if(frame)p=road.apply(frame,p);
      const d=(p[0]-o[0])*f[0]+(p[1]-o[1])*f[1]+(p[2]-o[2])*f[2];
      if(Math.abs(d)<=1e-4)out.add(p.map(x=>Math.round(x*10000)).join(','));
    }
  }
  return out;
}
function findPort(mesh,id){
  const p=mesh.parts&&mesh.ports.find(entry=>entry.id===id);
  assert(p,`port ${id} exists`);
  return p;
}

test('every piece builds at both detail levels, at every option the kit offers, inside its budget',()=>{
  assert.equal(road.pieces().length,13,'thirteen authored pieces');
  let count=0;
  for(const piece of road.pieces()){
    for(const options of variants(piece)){
      const mesh=road.build(piece,options);
      validate(mesh,`${piece}/${JSON.stringify(options)}`);
      assert.equal(mesh.piece,piece);
      assert.equal(mesh.lod,options.lod);
      count+=1;
    }
  }
  assert(count>200,`only ${count} variants exercised`);
});

test('the shading contract this file checks against is the one the module smooths to',()=>{
  assert.equal(road.crease,CREASE,'the crease limit moved: the winding assertion is checking a different contract from the one the module keeps');
  assert(road.crease>0&&road.crease<1,'a crease limit outside 0..1 welds faces that should stay hard');
  assert.equal(road.segmentLength,25,'the nominal segment length the kilometre budget is derived from');
  assert.equal(road.gauge,1.435,'standard gauge');
  assert.deepEqual(road.lods,['near','map']);
});

test('the map level is cheaper than the near level for every piece, and both are authored not decimated',()=>{
  for(const piece of road.pieces()){
    const near=road.build(piece,{lod:'near'}),map=road.build(piece,{lod:'map'});
    assert(map.triangleCount<near.triangleCount/2,`${piece}: map ${map.triangleCount} is not materially cheaper than near ${near.triangleCount}`);
    // Both LODs must still be the SAME OBJECT in the same place: the map level
    // is a coarser cross-section, not a smaller road.
    for(const axis of [0,2]){
      const a=near.bounds.max[axis]-near.bounds.min[axis],b=map.bounds.max[axis]-map.bounds.min[axis];
      assert(Math.abs(a-b)<0.6,`${piece}: axis ${axis} extent moves ${(a-b).toFixed(2)} m between detail levels`);
    }
    assert.deepEqual(near.ports.map(p=>p.id),map.ports.map(p=>p.id),`${piece}: same ports at both levels`);
    for(let i=0;i<near.ports.length;i++){
      for(let k=0;k<3;k++){
        assert(Math.abs(near.ports[i].position[k]-map.ports[i].position[k])<1e-9,`${piece}: port ${i} moves between detail levels`);
        assert(Math.abs(near.ports[i].forward[k]-map.ports[i].forward[k])<1e-9,`${piece}: port ${i} turns between detail levels`);
      }
    }
  }
});

test('every port is a well formed declaration and the geometry actually reaches it',()=>{
  for(const piece of road.pieces()){
    const multi=MULTI.has(piece);
    for(const options of variants(piece)){
      const mesh=road.build(piece,options);
      const label=`${piece}/${JSON.stringify(options)}`;
      assert(mesh.ports.length>=1,`${label}: declares a port`);
      const ids=new Set();
      for(const port of mesh.ports){
        assert(!ids.has(port.id),`${label}: unique port id ${port.id}`);ids.add(port.id);
        assert(['road','highway','rail'].includes(port.network),`${label}: known network on ${port.id}`);
        assert(port.position.every(Number.isFinite),`${label}: finite port position`);
        assert(Math.abs(Math.hypot(...port.forward)-1)<1e-9,`${label}: unit forward on ${port.id}`);
        assert.equal(port.forward[1],0,`${label}: forward is horizontal on ${port.id}`);
        assert.deepEqual(port.up,[0,1,0],`${label}: up is +Y on ${port.id}`);
        assert(port.width>0&&port.gauge>0,`${label}: port carries a width and a gauge`);
        assert(port.position[1]>=-1e-9,`${label}: port is not below ground`);
        const seen=planeReport(mesh,port);
        assert(seen.on>=3,`${label}: no geometry on the ${port.id} plane`);
        const allow=multi?MULTI_OVERHANG:MM;
        assert(seen.beyond<=allow+1e-9,`${label}: ${seen.beyond.toFixed(3)} m of geometry past the ${port.id} plane (allowed ${allow})`);
        if(!multi)assert(seen.span>=port.width*0.98,`${label}: ${port.id} covers only ${seen.span.toFixed(2)} m of a declared ${port.width} m`);
      }
    }
  }
});

// Which joins the kit claims are exact. Everything here must meet vertex for
// vertex; a pair that is only nominally compatible does not belong on this
// list, which is the point of having the list.
const EXACT=[
  ['road_straight','out',{},'road_straight','in',{}],
  ['road_straight','out',{},'road_bend','in',{turn:45,hand:'left'}],
  ['road_bend','out',{turn:90},'road_straight','in',{}],
  ['road_bend','out',{turn:1,hand:'left'},'road_bend','in',{turn:30}],
  ['road_straight','out',{rise:1,base:4},'road_bridge','in',{base:5}],
  ['road_bridge','out',{base:5},'road_straight','in',{base:5,rise:-1}],
  ['road_straight','out',{},'tunnel_portal','in',{mode:'road'}],
  ['road_straight','out',{},'road_junction','in',{}],
  ['road_junction','out',{},'road_straight','in',{}],
  ['road_straight','out',{},'road_crossroads','in',{}],
  ['road_straight','out',{},'level_crossing','in',{}],
  ['level_crossing','out',{},'road_straight','in',{}],
  ['rail_straight','out',{},'rail_curve','in',{turn:2}],
  ['rail_curve','out',{turn:3.5,hand:'left'},'rail_straight','in',{}],
  ['rail_straight','out',{base:4.6},'rail_bridge','in',{base:4.6}],
  ['rail_bridge','out',{base:4.6},'rail_straight','in',{base:4.6}],
  ['rail_straight','out',{},'tunnel_portal','in',{mode:'rail'}],
  ['rail_straight','out',{},'rail_switch','in',{}],
  ['rail_straight','out',{},'level_crossing','rail_west',{}],
  ['level_crossing','rail_east',{},'rail_straight','in',{}],
  ['highway_straight','out',{},'highway_bend','in',{turn:5}],
  ['highway_bend','out',{turn:10,hand:'left'},'highway_straight','in',{}],
];

test('two compatible pieces join with no gap and no overlap, vertex for vertex',()=>{
  for(const lod of ['near','map']){
    for(const [an,aid,ao,bn,bid,bo] of EXACT){
      const label=`${an}.${aid} -> ${bn}.${bid} @${lod}`;
      const a=road.build(an,{...ao,lod}),b=road.build(bn,{...bo,lod});
      const ap=findPort(a,aid),bp=findPort(b,bid);
      const frame=road.mateFrame(ap,bp);
      const placed=road.placePort(frame,bp);
      const fit=road.compatible(ap,placed);
      assert(fit.ok,`${label}: ${fit.why}`);
      assert(fit.gap<MM,`${label}: seam ${fit.gap} m apart`);
      assert(fit.dot<-1+1e-6,`${label}: seam faces do not oppose (${fit.dot})`);
      const sa=seamSet(a,ap,null),sb=seamSet(b,placed,frame);
      assert(sa.size>=5,`${label}: only ${sa.size} cross-section vertices at the seam`);
      const onlyA=[...sa].filter(k=>!sb.has(k)),onlyB=[...sb].filter(k=>!sa.has(k));
      assert.equal(onlyA.length,0,`${label}: ${onlyA.length} cross-section vertices with nothing to meet (${onlyA[0]})`);
      assert.equal(onlyB.length,0,`${label}: ${onlyB.length} cross-section vertices meeting nothing (${onlyB[0]})`);
    }
  }
});

/// Lay an explicit chain by the endpoint contract alone and hand back every
/// seam it made. Nothing here measures a gap and closes it: the frame of each
/// piece is derived from the port it is joining, which is the property under
/// test.
function chain(steps,lod){
  const seams=[],placed=[];
  let cursor=null,frame={c:1,s:0,x:0,y:0,z:0};
  for(const step of steps){
    const mesh=road.build(step[0],{...(step[1]||{}),lod});
    const entry=findPort(mesh,step[2]||'in');
    if(cursor)frame=road.mateFrame(cursor,entry);
    if(cursor)seams.push({label:`${step[0]}.${entry.id}`,fit:road.compatible(cursor,road.placePort(frame,entry))});
    placed.push({mesh,frame,entryY:road.placePort(frame,entry).position[1]});
    const exitId=step[3]||'out';
    const exit=mesh.ports.find(p=>p.id===exitId);
    if(!exit){cursor=null;break;}
    cursor=road.placePort(frame,exit);
  }
  return {seams,placed};
}

test('a chain of pieces climbs to a bridge, crosses a railway and ends in a tunnel, meeting every seam within a millimetre',()=>{
  const climb=[];
  for(let i=0;i<5;i++)climb.push(['road_straight',{base:i,rise:1}]);
  const fall=[];
  for(let i=0;i<5;i++)fall.push(['road_straight',{base:5-i,rise:-1}]);
  const steps=[['road_straight',{}],...climb,['road_bridge',{base:5}],...fall,
    ['road_junction',{}],['road_straight',{}],['level_crossing',{}],['road_straight',{}],
    ['road_bend',{turn:45}],['road_bend',{turn:45}],['road_straight',{}],['tunnel_portal',{mode:'road'}]];
  for(const lod of ['near','map']){
    const built=chain(steps,lod);
    assert.equal(built.placed.length,steps.length,`${lod}: whole chain placed`);
    assert.equal(built.seams.length,steps.length-1,`${lod}: a seam between every pair`);
    for(const seam of built.seams){
      assert(seam.fit.ok,`${lod}: ${seam.label} ${seam.fit.why}`);
      assert(seam.fit.gap<MM,`${lod}: ${seam.label} seam ${seam.fit.gap} m apart`);
    }
    // And the chain really climbed and came back down, so the height carried by
    // the port table is doing work rather than sitting at zero.
    const heights=built.placed.map(p=>p.entryY);
    assert(Math.max(...heights)>=5-1e-9,`${lod}: chain reached deck level`);
    assert(Math.abs(heights[heights.length-1])<1e-9,`${lod}: chain came back to grade`);
  }
  const rail=[];
  for(let i=0;i<12;i++)rail.push(['rail_straight',{base:i*4.6/12,rise:4.6/12}]);
  const down=[];
  for(let i=0;i<12;i++)down.push(['rail_straight',{base:4.6-i*4.6/12,rise:-4.6/12}]);
  const railSteps=[['rail_straight',{}],...rail,['rail_bridge',{base:4.6}],...down,
    ['rail_switch',{}],['rail_curve',{turn:3.5}],['rail_curve',{turn:3.5,hand:'left'}],
    ['rail_straight',{}],['tunnel_portal',{mode:'rail'}]];
  for(const lod of ['near','map']){
    const built=chain(railSteps,lod);
    assert.equal(built.placed.length,railSteps.length,`${lod}: whole rail chain placed`);
    for(const seam of built.seams){
      assert(seam.fit.ok,`${lod}: rail ${seam.label} ${seam.fit.why}`);
      assert(seam.fit.gap<MM,`${lod}: rail ${seam.label} seam ${seam.fit.gap} m apart`);
    }
  }
});

test('a long chain does not drift: two hundred segments end where arithmetic says they should',()=>{
  // Placement is multiply-and-add and never atan2-then-cos, so a long run must
  // accumulate rounding and nothing else. Two hundred 25 m straights are 5 km
  // and the endpoint has to land on it to well inside a millimetre.
  const steps=[];
  for(let i=0;i<200;i++)steps.push(['road_straight',{}]);
  const built=chain(steps,'map');
  for(const seam of built.seams)assert(seam.fit.gap<MM,`drifted to ${seam.fit.gap} m`);
  const last=built.placed[built.placed.length-1];
  assert(Math.abs(last.frame.z-199*25)<MM,`ended at ${last.frame.z} rather than ${199*25}`);
  assert(Math.abs(last.frame.x)<MM,`wandered ${last.frame.x} m sideways`);
  // And a run that turns: forty right bends of nine degrees each is a full
  // circle, which has to close on its own start.
  const ring=[];
  for(let i=0;i<24;i++)ring.push(['road_bend',{turn:15}]);
  const closed=chain(ring,'map');
  const start=closed.placed[0].frame,end=closed.placed[closed.placed.length-1];
  const exit=road.placePort(end.frame,end.mesh.ports[1]);
  assert(Math.hypot(exit.position[0]-start.x,exit.position[2]-start.z)<0.01,'a full circle of bends does not close');
});

test('an incompatible join is refused rather than fudged',()=>{
  const roadOut=findPort(road.build('road_straight',{}),'out');
  const railIn=findPort(road.build('rail_straight',{}),'in');
  const hwIn=findPort(road.build('highway_straight',{}),'in');
  const deckIn=findPort(road.build('road_bridge',{base:5}),'in');
  // A rail port sitting exactly where the road port is, facing it squarely.
  const railHere={...railIn,position:roadOut.position.slice(),forward:[0,0,-1]};
  assert.equal(road.compatible(roadOut,railHere).ok,false);
  assert.match(road.compatible(roadOut,railHere).why,/network/);
  const wide={...hwIn,network:'road',position:roadOut.position.slice(),forward:[0,0,-1]};
  assert.equal(road.compatible(roadOut,wide).ok,false);
  assert.match(road.compatible(roadOut,wide).why,/width/);
  // Same network, same width, five metres of air between them: a bridge deck
  // does not mate with a road at grade, and the contract says so instead of
  // hiding a step inside the seam.
  const high={...deckIn,forward:[0,0,-1]};
  const fit=road.compatible(roadOut,high);
  assert.equal(fit.ok,false);
  assert.match(fit.why,/apart/);
  assert(fit.gap>4.9,`${fit.gap} m apart`);
  // And two ports in the same place both facing the same way are back to back,
  // not joined.
  const backwards={...railIn,network:'road',width:roadOut.width,gauge:roadOut.gauge,
    position:roadOut.position.slice(),forward:roadOut.forward.slice()};
  assert.equal(road.compatible(roadOut,backwards).ok,false);
  assert.match(road.compatible(roadOut,backwards).why,/face/);
});

test('a run laid along a polyline meets every seam and stays inside the kilometre budget',()=>{
  for(const network of road.networks()){
    for(const lod of ['near','map']){
      const run=road.assemble({network,lod,path:[[0,0],[1000,0]]});
      validateRun(run,`${network}/${lod}`);
      assert.equal(run.run.segmentCount,40,`${network}/${lod}: 40 segments to the kilometre`);
      assert(run.run.maxSeamGap<MM,`${network}/${lod}: worst seam ${run.run.maxSeamGap} m`);
      assert.equal(run.run.seamsOk,true,`${network}/${lod}: every seam compatible`);
      assert(run.run.deviation<MM,`${network}/${lod}: straight run deviates ${run.run.deviation} m`);
      if(lod==='map')assert(run.triangleCount<KM_MAX,`${network}: a kilometre at map detail costs ${run.triangleCount}, over ${KM_MAX}`);
    }
  }
});

function validateRun(run,label){
  assert(run.triangleCount>0,`${label}: run has geometry`);
  assert.equal(run.groundDrop,0,`${label}: run authored at grade`);
  assert.equal(run.bounds.min[1],0,`${label}: run touches Y=0`);
  let previous=0;const names=new Set();
  for(const part of run.parts){
    assert(!names.has(part.name),`${label}: unique part ${part.name}`);names.add(part.name);
    assert.equal(part.first,previous,`${label}: contiguous part ${part.name}`);
    previous=part.first+part.count;
  }
  assert.equal(previous,run.positions.length/3,`${label}: parts cover every vertex of the run`);
  for(let i=0;i<run.positions.length;i++)assert(Number.isFinite(run.positions[i]),`${label}: finite run position`);
  for(let i=0;i<run.colors.length;i++)assert(run.colors[i]>=0&&run.colors[i]<=1,`${label}: run colour in range`);
}

test('a run says how far it fell short instead of pretending it followed the path',()=>{
  // A kit has the deflections it has and the segment length it has. The honest
  // reading is the residual, and it has to be REPORTED, not zero.
  const run=road.assemble({network:'road',lod:'map',path:[[0,0],[300,0],[300,400],[700,700]]});
  validateRun(run,'dogleg');
  assert(run.run.maxSeamGap<MM,`dogleg seam ${run.run.maxSeamGap} m`);
  assert.equal(run.run.seamsOk,true);
  assert(run.run.segmentCount>40,'a 1.2 km dogleg needs more than forty pieces');
  assert(run.run.corners.length===2,'two corners planned');
  for(const corner of run.run.corners){
    assert(corner.turns.length>=1,'a corner is built from real pieces');
    assert(Math.abs(corner.placed-corner.want)<=1.0,`corner wanted ${corner.want} and got ${corner.placed}`);
    for(const turn of corner.turns)assert(road.info('road_bend').deflections.includes(turn),`${turn} is a piece the kit has`);
  }
  assert(Number.isFinite(run.run.deviation)&&run.run.deviation>0,'the deviation from the path is measured and non-zero');
  assert(run.run.deviation<5,`deviation ${run.run.deviation} m is worse than the kit should manage`);
  assert(Number.isFinite(run.run.headingResidualDeg),'the unspent heading is reported');
  assert(run.description.includes(run.run.deviation.toFixed(2)),'the description quotes the deviation it measured');
  // A run with heights on the path climbs on graded straights.
  const hill=road.assemble({network:'road',lod:'map',path:[[0,0,0],[0,6,200]]});
  validateRun(hill,'hill');
  assert(hill.run.maxSeamGap<MM);
  assert(hill.run.pieces.some(p=>p.rise>0),'the climb is carried by graded straights');
  assert(Math.abs(hill.run.pieces.reduce((t,p)=>t+p.rise,0)-6)<1e-6,'the whole climb is delivered');
});

test('a run that goes DOWNHILL joins as exactly as one that goes up',()=>{
  // The climbing run above passes for a reason that does not generalise: on a
  // rising path every formation height is positive, and `base` is clamped to
  // 0..40. A DESCENDING path handed `assemble` a negative base, watched the
  // clamp swallow it, and then placed the next piece against a frame built for
  // the number before the clamp. Every seam on the descent was out by the whole
  // per-segment fall — 2.5 m at all twelve seams of a 300 m, 30 m descent, on
  // all three networks — and the run then went through `finish`'s ground
  // correction, which is the one repair this module says it never makes.
  // Nothing above asked a path to go down, so nothing above could see it.
  const paths={
    'straight descent':[[0,0,0],[0,-30,300]],
    'shallow descent':[[0,0,0],[0,-3,200]],
    'valley':[[0,0,0],[0,-12,250],[0,0,500]],
    'starts high, ends below the origin':[[0,20,0],[0,-5,600]],
    'descends round a corner':[[0,0,0],[0,-8,200],[200,-14,200]],
  };
  for(const network of road.networks()){
    for(const lod of ['near','map']){
      for(const name of Object.keys(paths)){
        const label=`${network}/${lod}/${name}`;
        const run=road.assemble({network,lod,path:paths[name]});
        validateRun(run,label);
        assert(run.run.segmentCount>0,`${label}: laid something`);
        assert(run.run.maxSeamGap<MM,`${label}: worst seam ${run.run.maxSeamGap} m`);
        assert.equal(run.run.seamsOk,true,`${label}: every seam compatible`);
        // The seating is DECLARED, not discovered: `datum` is what was taken off
        // every height, and the mesh still touches Y=0 without a correction.
        assert.equal(run.groundDrop,0,`${label}: seated by the path, not by finish`);
        assert.equal(run.bounds.min[1],0,`${label}: touches Y=0`);
        const low=Math.min(...paths[name].map(point=>point[1]));
        assert.equal(run.run.datum,low,`${label}: datum is the lowest point of the path asked for`);
        // Ports are reported in the SAME frame as the geometry, or a caller
        // placing the next thing against them places it in mid air.
        const start=run.ports.find(port=>port.id==='start');
        const end=run.ports.find(port=>port.id==='end');
        assert(Math.abs(start.position[1]-(paths[name][0][1]-low))<MM,`${label}: start port seated with the mesh`);
        assert(end.position[1]>=-MM,`${label}: end port not below the ground the mesh sits on`);
      }
    }
  }
  // The descent is actually delivered, not clamped away, and it is reported.
  const dive=road.assemble({network:'road',lod:'map',path:[[0,0,0],[0,-30,300]]});
  assert(Math.abs(dive.run.pieces.reduce((total,piece)=>total+piece.rise,0)+30)<1e-6,'the whole descent is delivered');
  assert(dive.run.pieces.every(piece=>piece.rise<0),'every straight on a pure descent falls');
  assert(dive.description.includes('seated on its'),'the description says the run was seated');
  assert(dive.description.includes('30.00 m higher'),'the description says by how much');
  // A path whose lowest point is already the origin has nothing to seat, so the
  // seating must not be a shift that fires on every path.
  for(const path of [[[0,0],[500,0]],[[0,0,0],[0,6,200]],[[0,0,0],[0,9,150],[0,3,300]]]){
    assert.equal(road.assemble({network:'road',lod:'map',path}).run.datum,0,`${JSON.stringify(path)}: nothing to seat`);
  }
  // And the datum is the lowest point WHEREVER the caller put its origin: a run
  // lying entirely four metres up is a run at grade, not four metres of fill.
  const raised=road.assemble({network:'road',lod:'map',path:[[0,4,0],[0,9,150]]});
  assert.equal(raised.run.datum,4,'the datum is the lowest point of the path, not zero');
  assert.equal(raised.run.maxSeamGap<MM,true,'a raised run still joins');
  assert.equal(raised.groundDrop,0,'a raised run is seated, not corrected');
  assert(Math.abs(raised.run.pieces[0].rise-road.assemble({network:'road',lod:'map',path:[[0,0,0],[0,5,150]]}).run.pieces[0].rise)<1e-9,
    'the same climb from a different origin is the same climb');
});

test('no piece is authored below the ground it stands on',()=>{
  // `base` is the height of the FORMATION above natural ground and the
  // cross-section draws fill: a skirt going down and out by twice the height.
  // At a negative height that skirt rises and turns inward, which is not a
  // cutting, so a fall deeper than the formation is refused the way a formation
  // height on an at-grade piece is refused — and the mesh reports the gradient
  // it was actually given rather than the one it was asked for.
  for(const piece of road.pieces()){
    if(!road.info(piece).straight)continue;
    for(const base of [0,0.5,2,8]){
      for(const asked of [-12,-8,-2,-0.2,0,0.2,2,8,12]){
        const mesh=road.build(piece,{lod:'near',base,rise:asked,length:25});
        const label=`${piece} base ${base} rise ${asked}`;
        assert.equal(mesh.groundDrop,0,`${label}: authored at grade`);
        assert.equal(mesh.bounds.min[1],0,`${label}: ground contact`);
        assert(mesh.rise>=-mesh.base-1e-9,`${label}: falls ${mesh.rise} from a formation only ${mesh.base} high`);
        assert(mesh.rise<=8+1e-9&&mesh.rise>=-8-1e-9,`${label}: gradient inside the range the kit offers`);
        if(asked>=-base&&asked>=-8&&asked<=8)assert(Math.abs(mesh.rise-asked)<1e-9,`${label}: a legal gradient is given as asked`);
        // The exit port is where the piece says it is, measured against the
        // formation and the gradient the mesh actually reports.
        const out=mesh.ports.find(port=>port.id==='out');
        assert(Math.abs(out.position[1]-(mesh.base+mesh.rise))<1e-6,`${label}: out port at base + rise`);
        assert(out.position[1]>=-1e-9,`${label}: out port underground`);
      }
    }
  }
  assert(Object.is(road.build('road_straight',{rise:-8}).rise,0),'a refused fall reads as zero, not as negative zero');
});

test('a level crossing is a surface a car can cross, not one buried in another',()=>{
  // The crossing panels were six flat triangles at a nominal rail height with a
  // CROWNED carriageway sweeping over them: 20 to 105 mm of road on top of them
  // at every point of the crossing, so the part existed, cost triangles and
  // could not be seen from any angle above. A part named in the table has to be
  // a part on the surface.
  const mesh=road.build('level_crossing',{lod:'near'});
  const panels=mesh.parts.find(part=>part.name.includes('crossing panels'));
  const carriageway=mesh.parts.find(part=>part.group==='carriageway');
  assert(panels&&carriageway,'the crossing has panels and a carriageway');
  // The height of a named part directly beneath a point, or null if it is not
  // over that part at all.
  function surfaceUnder(part,x,z){
    let best=null;
    for(let i=part.first*3;i<(part.first+part.count)*3;i+=9){
      const a=[mesh.positions[i],mesh.positions[i+1],mesh.positions[i+2]];
      const b=[mesh.positions[i+3],mesh.positions[i+4],mesh.positions[i+5]];
      const c=[mesh.positions[i+6],mesh.positions[i+7],mesh.positions[i+8]];
      const area=(b[0]-a[0])*(c[2]-a[2])-(c[0]-a[0])*(b[2]-a[2]);
      if(Math.abs(area)<1e-12)continue;
      const w1=((b[0]-x)*(c[2]-z)-(c[0]-x)*(b[2]-z))/area;
      const w2=((c[0]-x)*(a[2]-z)-(a[0]-x)*(c[2]-z))/area;
      const w3=1-w1-w2;
      if(w1<-1e-6||w2<-1e-6||w3<-1e-6)continue;
      const y=w1*a[1]+w2*b[1]+w3*c[1];
      if(best===null||y>best)best=y;
    }
    return best;
  }
  let checked=0;
  for(let v=panels.first;v<panels.first+panels.count;v++){
    const i=v*3,x=mesh.positions[i],y=mesh.positions[i+1],z=mesh.positions[i+2];
    const under=surfaceUnder(carriageway,x,z);
    if(under===null)continue;
    checked+=1;
    assert(y>under-1e-6,`a crossing panel at (${x.toFixed(2)}, ${z.toFixed(2)}) sits ${(under-y).toFixed(3)} m INSIDE the carriageway above it`);
    assert(y-under<0.05,`a crossing panel at (${x.toFixed(2)}, ${z.toFixed(2)}) floats ${(y-under).toFixed(3)} m over the road`);
    // Panels are the road between the rails; they do not run over the kerb.
    assert(Math.abs(x)<=3.5+1e-6,`a crossing panel reaches x=${x.toFixed(2)}, past the carriageway edge`);
  }
  assert(checked>=12,`only ${checked} panel vertices sat over the carriageway`);
  // And the crossing is FLUSH: a car meets the rail head, not a step up to it.
  let crown=-Infinity;
  for(let v=carriageway.first;v<carriageway.first+carriageway.count;v++){
    const i=v*3;
    if(Math.abs(mesh.positions[i])<1e-6&&Math.abs(mesh.positions[i+2]-6)<1.0)crown=Math.max(crown,mesh.positions[i+1]);
  }
  assert(crown>0,'the road is humped over the track');
  assert(Math.abs(crown-0.76)<0.01,`the crown sits at ${crown.toFixed(3)} m and the rail head at 0.760 m`);
  assert(mesh.bounds.max[1]>crown,'the rail head is the proud thing, as it is on a real crossing');
});

test('the road cross-section is the same road on both sides of the centre line',()=>{
  // Both kerbs are the same kerb. The left gutter — the 280 mm channel between
  // the kerb face and the carriageway edge — was authored in asphalt and the
  // right one in kerb grey, so a plain straight read with a 200 mm kerb on one
  // side and a 500 mm kerb on the other. Nothing about a two-lane road is
  // handed, and a kit whose two edges differ shows it down the length of a run.
  //
  // Sampled at offsets INSIDE the bands rather than at the profile's own
  // vertices: a vertex on a band boundary belongs to the band each side of it
  // and carries one of the two colours, which says nothing about the material.
  const OFFSETS=[0.875,1.20,2.60,3.56,3.75,3.94,4.04,4.35,5.30,5.80];
  for(const lod of ['near','map']){
    const mesh=road.build('road_straight',{lod,length:25});
    // The highest point of the SWEPT CROSS-SECTION at a point in plan, and the
    // colour it is painted. Scoped to parts[0] on purpose: gullies, marker
    // posts, sign posts and the lamp column are deliberately one-sided and
    // hashed onto a side, and a road is not asymmetric because its drain is.
    const section=mesh.parts[0];
    assert(SURFACE.has(section.group),'parts[0] is the swept cross-section');
    function surfaceAt(x,z){
      let best=null;
      for(let i=section.first*3;i<(section.first+section.count)*3;i+=9){
        const a=[mesh.positions[i],mesh.positions[i+1],mesh.positions[i+2]];
        const b=[mesh.positions[i+3],mesh.positions[i+4],mesh.positions[i+5]];
        const c=[mesh.positions[i+6],mesh.positions[i+7],mesh.positions[i+8]];
        const area=(b[0]-a[0])*(c[2]-a[2])-(c[0]-a[0])*(b[2]-a[2]);
        if(Math.abs(area)<1e-12)continue;
        const w1=((b[0]-x)*(c[2]-z)-(c[0]-x)*(b[2]-z))/area;
        const w2=((c[0]-x)*(a[2]-z)-(a[0]-x)*(c[2]-z))/area;
        const w3=1-w1-w2;
        if(w1<-1e-9||w2<-1e-9||w3<-1e-9)continue;
        const y=w1*a[1]+w2*b[1]+w3*c[1];
        if(best===null||y>best.y){
          best={y,r:w1*mesh.colors[i]+w2*mesh.colors[i+3]+w3*mesh.colors[i+6],
            g:w1*mesh.colors[i+1]+w2*mesh.colors[i+4]+w3*mesh.colors[i+7],
            b:w1*mesh.colors[i+2]+w2*mesh.colors[i+5]+w3*mesh.colors[i+8]};
        }
      }
      return best;
    }
    let compared=0;
    for(const off of OFFSETS){
      const left=surfaceAt(-off,12.5),right=surfaceAt(off,12.5);
      if(!left||!right)continue;
      compared+=1;
      assert(Math.abs(left.y-right.y)<1e-4,
        `${lod}: the section stands ${left.y.toFixed(4)} m at x=-${off} and ${right.y.toFixed(4)} m at +${off}`);
      // Colour is compared by HUE, because the baked sun is brighter on one
      // side of a crown than the other and that is the light, not the material.
      const l=Math.hypot(left.r,left.g,left.b)||1,r=Math.hypot(right.r,right.g,right.b)||1;
      const apart=Math.hypot(left.r/l-right.r/r,left.g/l-right.g/r,left.b/l-right.b/r);
      assert(apart<0.02,`${lod}: x=-${off} is a different material from x=+${off} (hue apart by ${apart.toFixed(3)})`);
    }
    assert(compared>=6,`${lod}: only ${compared} offsets found on the surface to compare`);
  }
});

test('malformed input resolves to a stable, honest default and nothing is mutated',()=>{
  const fallback=road.build('road_straight',{});
  for(const input of ['not_a_piece','__proto__','',null,undefined,{},0]){
    const mesh=road.build(input,{});
    assert.equal(mesh.piece,'road_straight',`${String(input)}: falls back`);
    assert.equal(digest(mesh),digest(fallback),`${String(input)}: falls back to the same mesh`);
  }
  for(const options of [undefined,null,'nonsense',0,{lod:'nonsense'},{index:Number.NaN},{turn:Number.NaN},{base:Number.NaN},{rise:Infinity},{length:-5}]){
    const mesh=road.build('road_straight',options);
    validate(mesh,`options ${JSON.stringify(options)}`);
  }
  const options={lod:'near',index:3,length:18,rise:1.5,base:2,sign:true};
  const snapshot=JSON.stringify(options);
  road.build('road_straight',options);
  assert.equal(JSON.stringify(options),snapshot,'build does not mutate its options');
  const spec={network:'road',lod:'map',path:[[0,0],[100,0]]};
  const specSnapshot=JSON.stringify(spec);
  road.assemble(spec);
  assert.equal(JSON.stringify(spec),specSnapshot,'assemble does not mutate its spec');
  assert(road.assemble({}).triangleCount>0,'an empty spec still builds something');
  assert(road.assemble({network:'nonsense',path:[[0,0]]}).network==='road','an unknown network falls back');
  assert.equal(road.info('not_a_piece'),null);
  // A deflection the kit does not have is snapped to one it does, and the mesh
  // says which it was given rather than which it was asked for.
  assert.equal(road.build('road_bend',{turn:37}).turn,30);
  assert.equal(road.build('road_bend',{turn:-45}).turn,45);
});

test('geometry is deterministic in one process and across a fresh load',()=>{
  delete require.cache[require.resolve(file)];
  const fresh=require(file);
  assert.deepEqual(fresh.pieces(),road.pieces());
  for(const piece of road.pieces()){
    for(const lod of ['near','map']){
      for(const index of [0,5]){
        const options={lod,index};
        const a=road.build(piece,options),b=fresh.build(piece,options),c=road.build(piece,{...options});
        for(const field of ['positions','normals','colors'])assert.deepEqual(a[field],b[field],`${piece}/${lod}/${index}: deterministic ${field}`);
        assert.equal(digest(a),digest(c),`${piece}/${lod}/${index}: repeat build is identical`);
        assert.deepEqual(a.bounds,b.bounds);
        assert.deepEqual(a.parts,b.parts);
        assert.deepEqual(a.ports,b.ports);
      }
    }
  }
  const runA=road.assemble({network:'rail',lod:'near',path:[[0,0],[200,0],[200,120]]});
  const runB=fresh.assemble({network:'rail',lod:'near',path:[[0,0],[200,0],[200,120]]});
  assert.equal(digest(runA),digest(runB),'an assembled run is identical across a fresh load');
});

test('variation comes only from the declared inputs',()=>{
  // The index is the only thing that may change a segment, and it must actually
  // change one — a hash that always answers the same way is not variation, it
  // is a constant with extra steps.
  const seen=new Set();
  for(let index=0;index<16;index++)seen.add(digest(road.build('road_straight',{lod:'near',index})));
  assert(seen.size>1,'the index never changes a road segment');
  assert(seen.size<16,'every index gives a different segment, so the variation is not drawn from a small set');
  for(let index=0;index<8;index++){
    assert.equal(digest(road.build('road_straight',{lod:'near',index})),digest(road.build('road_straight',{lod:'near',index})),`index ${index} is stable`);
  }
  // Map detail carries no dressing, so it must not vary at all: forty identical
  // segments in a kilometre is the whole reason the map level is cheap.
  const mapSeen=new Set();
  for(let index=0;index<16;index++)mapSeen.add(digest(road.build('road_straight',{lod:'map',index})));
  assert.equal(mapSeen.size,1,'the map level varies with the index and should not');
});

test('no wall clock, no frame counter and no second source of entropy can reach this geometry',()=>{
  // Iron rule one, and art gets no exemption. The cheapest way to keep it true
  // is for the vocabulary not to exist in the file at all.
  for(const pattern of [/\bDate\b/,/\bnow\s*\(/,/performance/i,/hrtime/,/requestAnimationFrame/i,
    /\bsetTimeout\b/,/\bsetInterval\b/,/Math\s*\.\s*random/,/\brandom\b/i,/getTime/,/process\s*\./,
    /\bglobalThis\s*\.\s*/])
    assert(!pattern.test(source),`road-mesh.js must not contain ${pattern}`);
  // And no DOM, so the geometry stays checkable outside a browser.
  for(const pattern of [/\bdocument\b/,/\bwindow\b/,/canvas/i,/WebGL/i,/require\s*\(/,/\bimport\b/,/fetch\s*\(/])
    assert(!pattern.test(source),`road-mesh.js must not contain ${pattern}`);
});

test('the browser global exports the same contract with no dependencies',()=>{
  const context=vm.createContext({});
  vm.runInContext(source,context);
  assert.equal(typeof context.RoadMesh.build,'function');
  assert.equal(typeof context.RoadMesh.assemble,'function');
  assert.equal(context.RoadMesh.pieces().join(','),road.pieces().join(','));
  const sandboxed=context.RoadMesh.build('rail_switch',{lod:'near'});
  assert.equal(sandboxed.triangleCount,road.build('rail_switch',{lod:'near'}).triangleCount);
  assert.equal(context.RoadMesh.assemble({network:'highway',lod:'map',path:[[0,0],[500,0]]}).triangleCount,
    road.assemble({network:'highway',lod:'map',path:[[0,0],[500,0]]}).triangleCount);
});

test('every piece says what it is and refuses to claim it is a real route',()=>{
  for(const piece of road.pieces()){
    const info=road.info(piece);
    assert(info.label&&info.blurb,`${piece}: described`);
    assert(['road','highway','rail'].includes(info.network),`${piece}: belongs to a network`);
    const mesh=road.build(piece,{lod:'near'});
    assert(mesh.description.includes('not a survey of any real'),`${piece}: refuses to claim a real route`);
    assert(mesh.description.includes('no capability of its'),`${piece}: grants nothing`);
    assert(mesh.description.includes('Original game art'),`${piece}: says whose art it is`);
    for(const port of mesh.ports)assert(mesh.description.includes(port.id),`${piece}: description lists ${port.id}`);
  }
  const run=road.assemble({network:'road',lod:'map',path:[[0,0],[500,0]]});
  assert(run.description.includes('asserts nothing about whether a route exists'),'a run refuses to claim a route exists');
  assert(run.description.includes('guide, not a promise'),'a run says the path is a guide');
  for(const network of road.networks()){
    const table=road.info(road.assemble({network,path:[[0,0],[50,0]]}).run.pieces[0].piece);
    assert.equal(table.network,network,`${network}: assembled from its own pieces`);
  }
});
