const {test}=require('node:test');
const assert=require('node:assert/strict');
const {build}=require('../../spheres-web/ui/equipment-mesh.js');

// These probes inspect the exported triangles, rather than trusting authored
// feature labels or triangle totals. Coordinates describe the original Spheres
// concept in scene units; they are not real aircraft engineering dimensions.
const make=(components={},lod=1)=>build({platform:'air_tactical_strike',components,lod});
const boundsCache=new WeakMap();
function bounds(mesh,part){
  let entries=boundsCache.get(mesh);if(!entries){entries=new Map();boundsCache.set(mesh,entries);}
  if(entries.has(part))return entries.get(part);
  const min=[Infinity,Infinity,Infinity],max=[-Infinity,-Infinity,-Infinity];
  for(let i=part.first*3;i<(part.first+part.count)*3;i+=3)for(let a=0;a<3;a++){
    min[a]=Math.min(min[a],mesh.positions[i+a]);max[a]=Math.max(max[a],mesh.positions[i+a]);
  }
  const result={min,max};entries.set(part,result);return result;
}
function trace(mesh,origin,direction,accept=()=>true){
  const hits=[],p=mesh.positions,[ox,oy,oz]=origin,[dx,dy,dz]=direction;
  for(const part of mesh.parts){
    if(!accept(part))continue;
    const box=bounds(mesh,part);
    if(direction.some((d,a)=>d===0&&(origin[a]<box.min[a]-1e-6||origin[a]>box.max[a]+1e-6)))continue;
    for(let i=part.first*3;i<(part.first+part.count)*3;i+=9){
      const ax=p[i],ay=p[i+1],az=p[i+2],ux=p[i+3]-ax,uy=p[i+4]-ay,uz=p[i+5]-az;
      const vx=p[i+6]-ax,vy=p[i+7]-ay,vz=p[i+8]-az;
      const hx=dy*vz-dz*vy,hy=dz*vx-dx*vz,hz=dx*vy-dy*vx,det=ux*hx+uy*hy+uz*hz;
      if(Math.abs(det)<1e-10)continue;
      const sx=ox-ax,sy=oy-ay,sz=oz-az,u=(sx*hx+sy*hy+sz*hz)/det;
      if(u<-1e-7||u>1+1e-7)continue;
      const qx=sy*uz-sz*uy,qy=sz*ux-sx*uz,qz=sx*uy-sy*ux,v=(dx*qx+dy*qy+dz*qz)/det;
      if(v<-1e-7||u+v>1+1e-7)continue;
      const distance=(vx*qx+vy*qy+vz*qz)/det;if(distance<1e-6)continue;
      const vertex=i/3,glass=(mesh.surfaces||[]).some(s=>s.material==='glass'&&vertex>=s.first&&vertex<s.first+s.count);
      hits.push({distance,part,glass,position:[ox+dx*distance,oy+dy*distance,oz+dz*distance]});
    }
  }
  return hits.sort((a,b)=>a.distance-b.distance).filter((h,i,all)=>!i||Math.abs(h.distance-all[i-1].distance)>1e-5);
}
// Grounding differs slightly by LOD. The tips of the fixed vertical stabilizers
// give a stable datum without reaching into private geometry-builder state.
const datum=mesh=>mesh.bounds.max[1]-4.1;

test('every tactical engine option has two physically open, recessed intake throats',()=>{
  for(const lod of [0,1])for(const [air_engine,mouthX] of [
    ['air_engine_twin',1.3457],['air_engine_economical',1.309625],['air_engine_efficient',1.3535]
  ]){
    const mesh=make({air_engine},lod),y=1.73+datum(mesh);
    for(const side of [-1,1])for(const [xOffset,yOffset] of [[0,0],[-.16,0],[.16,0],[0,-.16],[0,.16]]){
      const first=trace(mesh,[side*mouthX+xOffset,y+yOffset,1.70],[0,0,-1])[0];
      const context=`${air_engine}, LOD${lod}, side ${side}, sample ${xOffset}/${yOffset}`;
      assert(first,`missing intake interior: ${context}`);
      assert.equal(first.part.slot,'air_engine',`unrelated geometry blocks intake: ${context}`);
      assert(first.distance>.45&&first.distance<.95,`intake is capped at its lip or has no recessed end: ${context}`);
    }
  }
});

test('both tactical seats remain visible through glass above a real cockpit cavity',()=>{
  for(const lod of [0,1])for(const air_avionics of ['air_avionics_analog','air_avionics_digital']){
    const mesh=make({air_avionics},lod),shift=datum(mesh);
    for(const z of [2.79,4.19])for(const x of [-.07,0,.07]){
      const hits=trace(mesh,[x,4+shift,z],[0,-1,0]),glass=hits[0],inside=hits.find(h=>!h.glass);
      const context=`${air_avionics}, LOD${lod}, seat z${z}, x${x}`;
      assert(glass?.glass,`seat is not covered by its clear canopy: ${context}`);
      assert(inside,`missing seat geometry: ${context}`);
      assert.equal(inside.part.slot,'air_avionics',`fuselage obscures seat: ${context}`);
      assert(inside.position[1]>2.80+shift,`probe misses the seat headrest: ${context}`);
      assert(inside.distance-glass.distance>.04,`seat clips the glass: ${context}`);
      const shell=hits.find(h=>h.part.label==='airframe and wing roots');
      assert(shell&&shell.distance>inside.distance+.05,`cockpit has no depth above the fuselage: ${context}`);
    }
  }
});

test('all tactical wing choices have distinct flaps, ailerons and open hinge seams',()=>{
  for(const [air_wing,span,tipTrail] of [
    ['air_wing_swept',5.85,-3.10],['air_wing_stable',6.60,-1.20],['air_wing_straight',5.85,-.75]
  ]){
    const mesh=make({air_wing}),root=.5852;
    for(const side of [-1,1]){
      const name=side<0?'port ':'starboard ',wing=p=>p.label.startsWith(name)&&p.label.endsWith(' wing');
      for(const fraction of [.40,.70]){
        const x=side*(root+(span-root)*fraction),trailing=-1.8+(tipTrail+1.8)*fraction;
        assert(trace(mesh,[x,4,trailing+.020],[0,-1,0],wing).length,'main wing skin is missing');
        assert.equal(trace(mesh,[x,4,trailing-.0125],[0,-1,0],wing).length,0,'wing skin and control surface overlap their hinge gap');
        assert(trace(mesh,[x,4,trailing-.100],[0,-1,0],wing).length,'separate control surface is missing');
      }
      const fraction=.54,x=side*(root+(span-root)*fraction),trailing=-1.8+(tipTrail+1.8)*fraction;
      assert.equal(trace(mesh,[x,4,trailing-.100],[0,-1,0],wing).length,0,'flap and aileron have no separating break');
    }
  }
});

test('tactical payloads clear wing undersides and both main wings retain mirrored geometry',()=>{
  for(const air_payload of ['air_payload_unguided','air_payload_guided']){
    const mesh=make({air_payload,air_fuel:'air_fuel_extended'});
    const wings=mesh.parts.filter(p=>/^(port|starboard) .* wing$/.test(p.label));
    assert.equal(wings.length,2);
    const a=bounds(mesh,wings[0]),b=bounds(mesh,wings[1]);
    for(const [left,right] of [[a.min[0],-b.max[0]],[a.max[0],-b.min[0]],[a.min[1],b.min[1]],[a.max[1],b.max[1]],[a.min[2],b.min[2]],[a.max[2],b.max[2]]]){
      assert(Math.abs(left-right)<1e-5,'main wing envelopes are asymmetric');
    }
    const payload=bounds(mesh,mesh.parts.find(p=>p.slot==='air_payload'));
    assert(payload.max[1]<Math.min(a.min[1],b.min[1])-.10,'external stores clip the wing underside');
    const fuel=mesh.parts.find(p=>p.slot==='air_fuel');
    for(const side of [-1,1]){
      const tank=trace(mesh,[side*3.75,1.3+datum(mesh),3],[0,0,-1],p=>p===fuel)[0];
      assert(tank&&tank.position[2]<.5&&tank.position[2]>-.5,'underwing fuel tank is missing or displaced');
    }
  }
});
