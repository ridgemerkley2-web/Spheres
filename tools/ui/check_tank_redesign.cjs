'use strict';
const {test}=require('node:test'),assert=require('node:assert/strict'),crypto=require('node:crypto'),path=require('node:path');
// Optional override permits a real red run against the archived pre-redesign
// generator, without changing production code or the assertions below.
const {build}=require(process.env.SPHERES_TANK_AUDIT_MESH||'../../spheres-web/ui/equipment-mesh.js');
const tanks=['tank_standard','tank_heavy','tank_light','tank_destroyer'];
const hash=m=>crypto.createHash('sha256').update(Buffer.from(m.positions.buffer)).update(Buffer.from(m.normals.buffer)).update(Buffer.from(m.colors.buffer)).digest('hex');
function vertices(mesh,name,color){const p=mesh.parts.find(p=>p.name===name);assert(p,name);const out=[];for(let i=p.first*3;i<(p.first+p.count)*3;i+=3)if(!color||color.every((v,k)=>Math.abs(mesh.colors[i+k]-v)<1e-6))out.push({p:Array.from(mesh.positions.subarray(i,i+3)),n:Array.from(mesh.normals.subarray(i,i+3))});return out;}
function span(v,axis){const values=v.map(v=>v.p[axis]);return [Math.min(...values),Math.max(...values)];}
// Specialist pins intentionally updated by the later specialist proportion pass;
// all aircraft pins retain the pre-tank-redesign bytes. The dedicated specialist
// test additionally freezes approved tank/air material sidecars.
const nonTankReviewed={
  "ground_ifv/0": "4e6ebec21677943fcbbda5eab877c430809ad10f55f1a475bfa95969a9967047",
  "ground_ifv/1": "35bae9a60e93112a15e554f7177fbecdd1e84024ec4580c98ed5970d18b61c11",
  "ground_ifv/2": "e92e4ccd6e2ee26da60662a0b1a30bd9b75e46eb3580da978326dcc77c128611",
  "ground_apc/0": "09aca7cdb83190111eb9764d5c40c24c6b871e635d1c73d6486f5b006a92cb78",
  "ground_apc/1": "c2bb341e5c551bba3d2e067ea083ba23c58df57922fcfa76d4955750141461af",
  "ground_apc/2": "b01c4f24487a5de0f5cbe7edf5a50c8b21560434dee78d522c7d6065a78ad37c",
  "ground_recon/0": "a08518bf42fb38a8274a3afecef879f667c54fa89ba53c7d8a6c74e5f2fb8d25",
  "ground_recon/1": "2e3009c7bc5c3704813cee2aa6f34bc45c1261c8a317778ab4056371f7c05cc2",
  "ground_recon/2": "7a5bd0559ddebc51842140f0beea3be6c78cd735fabff792eeab596250eaffd3",
  "ground_artillery/0": "b1b58a04631c2655df0343e1ff2dcd5d7de019dd44a6b33da67caec80c767ff7",
  "ground_artillery/1": "f78b20e414d4d9036de6044c59d5cddfecd543c5ea1c59d5cd3148aed0861001",
  "ground_artillery/2": "3c731bf952b76bb78bf0e90deb6511ee77d9263ae5bf63cc5fcf3af384c5c411",
  "ground_air_defense/0": "48f8fb87e23fff8d3eeac21715797c1046ce637fb0f00a271c130b0a46e36846",
  "ground_air_defense/1": "36c715bf0fcb113a2eebb6f393706aeda342db8b93e24eec3b531a9b17bdf774",
  "ground_air_defense/2": "d897cdd61f676e5b10ddc35f2838ee5ea7c43e71d05fede729d856f708f1bb4e",
  "air_light_attack/0": "269fdb652e4e1df8fc34b22b4bca1801a3cd7a38877b7da0fbd172d9a7ef3e1e",
  "air_light_attack/1": "269fdb652e4e1df8fc34b22b4bca1801a3cd7a38877b7da0fbd172d9a7ef3e1e",
  "air_light_attack/2": "269fdb652e4e1df8fc34b22b4bca1801a3cd7a38877b7da0fbd172d9a7ef3e1e",
  "air_tactical_strike/0": "84f36ee62aecf2702c36e1b2bd37eca3bd61bef3a21d2d0185e18ce4864424e6",
  "air_tactical_strike/1": "84f36ee62aecf2702c36e1b2bd37eca3bd61bef3a21d2d0185e18ce4864424e6",
  "air_tactical_strike/2": "84f36ee62aecf2702c36e1b2bd37eca3bd61bef3a21d2d0185e18ce4864424e6"
};

test('every aircraft and specialist default matches its reviewed geometry and colors at every LOD',()=>{
  for(const [key,want]of Object.entries(nonTankReviewed)){const [platform,lod]=key.split('/');assert.equal(hash(build({platform,lod:Number(lod)})),want,key);}
});

test('tank hulls have a long chassis envelope at all three LODs',()=>{
  for(const platform of tanks)for(const lod of [0,1,2]){
    const m=build({platform,lod}),z=span(vertices(m,'chassis / sloped lower hull'),2),ratio=(z[1]-z[0])/(m.bounds.max[0]-m.bounds.min[0]);
    assert(ratio>1.75&&ratio<2.1,platform+' LOD'+lod+' hull length / overall width '+ratio);
  }
});

test('rotating turrets form a broad continuous low body instead of a tall narrow stack',()=>{
  for(const platform of ['tank_standard','tank_heavy'])for(const turret of ['turret_standard','turret_compact','turret_heavy','turret_autoload']){
    const m=build({platform,components:{turret,armament:'gun_120'}}),v=vertices(m,'turret / ring and faceted armor shell'),y=span(v,1),z=span(v,2);
    assert((z[1]-z[0])/(y[1]-y[0])>4,'Broad turret body '+platform+' '+turret);
    assert(z[0]<-1.9,'Rear armored body extends into its bustle');
  }
});

test('each main gun is a slender continuous barrel with a recessed compact mount',()=>{
  for(const armament of ['gun_90','gun_105','gun_120','gun_125']){
    const m=build({platform:'tank_standard',components:{armament}}),part=m.parts.find(p=>p.slot==='armament'),v=vertices(m,part.name),muzzle=span(v,2)[1];
    const barrel=v.filter(v=>v.p[2]>muzzle-1.5);assert(barrel.length>100);
    const radius=Math.max(...barrel.map(v=>Math.abs(v.p[0]))),width=m.bounds.max[0]-m.bounds.min[0];
    assert(radius/width<.036,armament+' barrel radius / vehicle width '+radius/width);
    const mount=v.filter(v=>v.p[2]<1.72),mountY=span(mount,1);assert(mountY[1]-mountY[0]<.52,'No oversized transverse drum '+armament);
  }
});

test('heavy cheek armor has a single wedge top and a sloped face instead of repeated staircase ledges',()=>{
  for(const platform of ['tank_standard','tank_heavy'])for(const turret of ['turret_standard','turret_compact','turret_heavy','turret_autoload','turret_casemate']){
    const m=build({platform,components:{turret,protection:'protection_heavy',armament:'gun_120'}}),v=vertices(m,'protection / reinforced modular armor blocks');
    const tops=v.filter(v=>v.n[1]>.999&&v.p[2]>.95&&v.p[1]>1.6&&Math.abs(v.p[0])>.35);
    assert(tops.length>=6,'The test must inspect both real cheek tops');
    assert.equal(new Set(tops.map(v=>v.p[1].toFixed(5))).size,1,'Only the top of a continuous wedge may be horizontal');
    assert(v.some(v=>v.p[2]>1.3&&v.p[1]>1.65&&v.n[1]>.25&&v.n[1]<.95),'The wedge has a genuine sloped front face');
  }
});

test('road wheels fill the track envelope and end gears share the belt return centers',()=>{
  const m=build({platform:'tank_standard'}),name='running gear / starboard suspension, road wheels and sprockets';
  const road=vertices(m,name,[.055,.068,.060]).filter(v=>Math.abs(v.p[2]+2.32)<.405&&v.p[1]<1),roadY=span(road,1);
  assert(roadY[1]-roadY[0]>.77&&roadY[1]-roadY[0]<.82,'Full-sized circular road tyres');
  const beltY=span(vertices(m,'running gear / starboard continuous track belt'),1),beltCenter=(beltY[0]+beltY[1])/2;
  for(const end of [-1,1]){const endY=span(vertices(m,name,[.29,.34,.22]).filter(v=>Math.abs(v.p[2]-end*2.78)<.52),1);assert(Math.abs((endY[0]+endY[1])/2-beltCenter)<.01,'End wheel sits concentrically in the track return');}
});

test('tank material classes are explicit aligned deterministic metadata across selected configurations and LODs',()=>{
  for(const platform of tanks)for(const lod of [0,1,2]){
    const spec={platform,lod,components:{protection:'protection_heavy',armament:'gun_125',active_protection:'aps_hard',suspension:'suspension_hydro',sensors:'optics_thermal',ammunition:'ammo_penetrator'}};
    const m=build(spec);assert(m.materialClasses instanceof Uint8Array);assert.equal(m.materialClasses.length,m.positions.length/3);assert(m.materialClasses.every(c=>Number.isInteger(c)&&c>=0&&c<=7));assert.deepEqual(m.materialClasses,build(spec).materialClasses);
  }
});
