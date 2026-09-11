'use strict';
const {test}=require('node:test');
const assert=require('node:assert/strict');
const crypto=require('node:crypto');
const {build}=require('../../spheres-web/ui/equipment-mesh.js');

// Inspect assembled triangles, not feature names or generator source. The
// scene coordinates belong to Spheres' fictional aircraft, not engineering data.
const shift=m=>m.bounds.max[1]-4.1;
const make=(components={},lod=1)=>build({platform:'air_tactical_strike',components,lod});
function axialHits(mesh,origin,axis,direction,accept=()=>true){
  const a=(axis+1)%3,b=(axis+2)%3,p=mesh.positions,hits=[];
  for(const part of mesh.parts){
    if(!accept(part))continue;
    for(let i=part.first*3;i<(part.first+part.count)*3;i+=9){
      const ax=p[i+a],ay=p[i+b],ux=p[i+3+a]-ax,uy=p[i+3+b]-ay;
      const vx=p[i+6+a]-ax,vy=p[i+6+b]-ay,det=ux*vy-uy*vx;
      if(Math.abs(det)<1e-10)continue;
      const px=origin[a]-ax,py=origin[b]-ay,u=(px*vy-py*vx)/det,v=(ux*py-uy*px)/det;
      if(u<-1e-7||v<-1e-7||u+v>1+1e-7)continue;
      const at=p[i+axis]+u*(p[i+3+axis]-p[i+axis])+v*(p[i+6+axis]-p[i+axis]);
      const distance=direction*(at-origin[axis]);if(distance<1e-6)continue;
      const vertex=i/3,glass=(mesh.surfaces||[]).some(s=>s.material==='glass'&&vertex>=s.first&&vertex<s.first+s.count);
      hits.push({distance,at,part,glass});
    }
  }
  return hits.sort((a,b)=>a.distance-b.distance).filter((h,i,all)=>!i||Math.abs(h.distance-all[i-1].distance)>1e-5);
}

test('all tactical exhaust choices reveal a deep interior beyond the old casing cap',()=>{
  for(const lod of [0,1])for(const [air_engine,r,xs] of [
    ['air_engine_twin',.54,[-.84,.84]],['air_engine_economical',.43,[0]],['air_engine_efficient',.52,[0]]
  ]){
    const mesh=make({air_engine},lod),y=1.73+shift(mesh);
    for(const x of xs)for(const [dx,dy] of [[0,0],[r*.25,0],[-r*.25,0],[0,r*.25],[0,-r*.25]]){
      const first=axialHits(mesh,[x+dx,y+dy,-8.60],2,1)[0];
      const context=`${air_engine}, LOD${lod}, ${x}/${dx}/${dy}`;
      assert(first,`no exhaust interior: ${context}`);
      assert.equal(first.part.slot,'air_engine',`airframe blocks the exhaust: ${context}`);
      assert(first.at>-7.90&&first.at<-7.20,`a shallow cap hides the turbine or the nozzle has no end: ${context}`);
    }
  }
});

function closedSolids(mesh,cx,cy,r,minZ=.934,maxZ=1.001){
  const points=[],ids=new Map(),triangles=[],parent=[];
  const id=p=>{const key=p.map(v=>v.toFixed(6)).join(',');if(ids.has(key))return ids.get(key);const n=points.length;ids.set(key,n);points.push(p);parent.push(n);return n;};
  const root=i=>parent[i]===i?i:(parent[i]=root(parent[i]));
  const p=mesh.positions;
  for(const part of mesh.parts.filter(p=>p.slot==='air_engine'))for(let i=part.first*3;i<(part.first+part.count)*3;i+=9){
    const ps=[0,3,6].map(j=>Array.from(p.subarray(i+j,i+j+3)));
    if(!ps.every(q=>q[2]>minZ&&q[2]<maxZ&&Math.hypot(q[0]-cx,q[1]-cy)<r*.723))continue;
    const t=ps.map(id);if(new Set(t).size!==3)continue;
    parent[root(t[1])]=root(t[0]);parent[root(t[2])]=root(t[0]);triangles.push(t);
  }
  const components=new Map();
  for(const t of triangles){const n=root(t[0]);if(!components.has(n))components.set(n,[]);components.get(n).push(t);}
  return [...components.values()].filter(ts=>{
    if(ts.length<12)return false;
    const edges=new Map(),vertices=new Set();
    for(const t of ts)for(let j=0;j<3;j++){
      vertices.add(t[j]);const a=t[j],b=t[(j+1)%3],key=a<b?`${a}/${b}`:`${b}/${a}`;
      edges.set(key,(edges.get(key)||0)+1);
    }
    const zs=[...vertices].map(i=>points[i][2]);
    return [...edges.values()].every(n=>n===2)&&Math.max(...zs)-Math.min(...zs)>.030;
  }).length;
}

test('both intakes contain separate solid swept fan blades instead of flat spokes',()=>{
  for(const lod of [0,1])for(const [air_engine,r,mouth] of [
    ['air_engine_twin',.378,1.3457],['air_engine_economical',.3225,1.309625],['air_engine_efficient',.390,1.3535]
  ]){
    const mesh=make({air_engine},lod),y=1.73+shift(mesh);
    for(const side of [-1,1]){
      assert(closedSolids(mesh,side*(mouth-.030),y,r)>=(lod===0?24:18),`${air_engine} LOD${lod}: fan vanes need closed thickness and curved axial sections`);
      const first=axialHits(mesh,[side*mouth,y,1.70],2,-1)[0];
      assert.equal(first?.part.slot,'air_engine');
      assert(first.distance>.45&&first.distance<.95,'new fan fittings must not seal the inlet at its mouth');
    }
  }
});

test('recessed exhaust turbines also retain solid vanes in both close detail levels',()=>{
  for(const lod of [0,1])for(const [air_engine,r,xs] of [
    ['air_engine_twin',.54,[-.84,.84]],['air_engine_economical',.43,[0]],['air_engine_efficient',.52,[0]]
  ]){
    const mesh=make({air_engine},lod),y=1.73+shift(mesh);
    for(const x of xs)assert(closedSolids(mesh,x,y,r,-7.420,-7.352)>=(lod===0?20:12),`${air_engine} LOD${lod}: the exhaust turbine must retain volumetric vanes`);
  }
});

test('tactical nozzle petals taper toward the outlet rather than forming a straight drum',()=>{
  for(const lod of [0,1])for(const [air_engine,r,xs] of [
    ['air_engine_twin',.54,[-.84,.84]],['air_engine_economical',.43,[0]],['air_engine_efficient',.52,[0]]
  ]){
    const mesh=make({air_engine},lod),y=1.73+shift(mesh);
    for(const x of xs){
      const radii=[[],[]];
      for(const part of mesh.parts.filter(p=>p.slot==='air_engine'))for(let i=part.first*3;i<(part.first+part.count)*3;i+=3){
        const px=mesh.positions[i],py=mesh.positions[i+1],z=mesh.positions[i+2],radius=Math.hypot(px-x,py-y);
        if(radius>r*1.10)continue;
        if(Math.abs(z+8.542)<.002)radii[0].push(radius);
        if(Math.abs(z+8.002)<.002)radii[1].push(radius);
      }
      assert(radii.every(a=>a.length>16),'both nozzle outlet and collar must have actual surface geometry');
      const outlet=Math.max(...radii[0]),collar=Math.max(...radii[1]);
      assert(outlet<collar*.90&&outlet>r*.75,'converging nozzle keeps a broad, visibly open outlet');
    }
  }
});

test('cockpit side controls sit visibly inside the glass with space above the consoles',()=>{
  for(const lod of [0,1])for(const air_avionics of ['air_avionics_analog','air_avionics_digital']){
    const mesh=make({air_avionics},lod),d=shift(mesh);
    for(const seatZ of [3.03,4.43]){
      const hits=axialHits(mesh,[-.31,4+d,seatZ-.015],1,-1),glass=hits[0],control=hits.find(h=>!h.glass);
      assert(glass?.glass,'the throttle is covered by the transparent canopy');
      assert.equal(control?.part.slot,'air_avionics','fuselage skin must not obscure the throttle');
      assert(control.at>2.58+d&&control.at<2.72+d,'throttle grip must stand above the console, inside the cockpit');
      assert(control.distance-glass.distance>.09,'controls must clear the canopy');
    }
  }
});

const loaded={air_engine:'air_engine_efficient',air_wing:'air_wing_stable',air_radar:'air_radar_mapping',air_avionics:'air_avionics_digital',air_countermeasures:'air_countermeasures_ecm',air_payload:'air_payload_guided',air_fuel:'air_fuel_extended'};
function digest(m){const h=crypto.createHash('sha256');for(const a of [m.positions,m.normals,m.colors])h.update(Buffer.from(a.buffer,a.byteOffset,a.byteLength));h.update(JSON.stringify([m.parts,m.surfaces,m.bounds,m.specification]));return h.digest('hex');}
const unchanged={
  'air_light_attack/default/0':'e6d5a29bd8786c6f7fb9692c49356ebaf2a025d47ea33e7eaab6383a42b4d791',
  'air_light_attack/default/1':'badd55452dec5aa92860e63cd024f4980856c56735a3422236d11d24b41b7ec1',
  'air_light_attack/default/2':'e1629439e693aebe0d99d4794724a66ad8f7874251ebf59189e70cd8ecf59679',
  'air_light_attack/loaded/0':'4ffbafb0c0759ec6339fd46cec2d39aa2693fa3265c27eabb35324d6e5715b18',
  'air_light_attack/loaded/1':'899341af7ec1e5cb049c32c60a1fc5fcaec3253ea7005e0f0997cb7714757d65',
  'air_light_attack/loaded/2':'3c3fc77f6b562359f4c9c7290ed25408df00d4ac3366e9a761521cdcd345f1fa',
  'air_tactical_strike/default/2':'98e82e0f3ab981894d492fcf948a351f0b79f12fb6bc4570abc1ba233da7cb35',
  'air_tactical_strike/loaded/2':'df95e2dfd8d1b3164a10f594b7c4129c77aa403fbcde76c55749854b5e848d9b'
};
test('light aircraft and tactical map meshes are unchanged by inspection detail work',()=>{
  for(const [key,expected] of Object.entries(unchanged)){
    const [platform,preset,lod]=key.split('/');
    assert.equal(digest(build({platform,components:preset==='loaded'?loaded:{},lod:Number(lod)})),expected,key);
  }
});

test('all engine and cockpit combinations retain inspection and catalogue budgets',()=>{
  for(const lod of [0,1])for(const air_engine of ['air_engine_twin','air_engine_economical','air_engine_efficient'])for(const air_avionics of ['air_avionics_analog','air_avionics_digital']){
    const m=make({air_engine,air_avionics},lod);
    assert(m.triangleCount>=(lod===0?100000:1000));
    assert(m.triangleCount<(lod===0?250000:18000),`${air_engine}/${air_avionics}/LOD${lod}: ${m.triangleCount}`);
    assert.equal(m.parts.filter(p=>p.slot==='air_engine').length,air_engine==='air_engine_twin'?2:1);
    assert.equal(m.parts.filter(p=>p.slot==='air_avionics').length,1);
    assert(m.positions.every(Number.isFinite)&&m.normals.every(Number.isFinite));
  }
});
