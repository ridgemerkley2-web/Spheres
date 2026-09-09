/* Independent formation IFV geometry. No simulation or renderer substitutions. */
const {test}=require('node:test'),assert=require('node:assert/strict'),fs=require('node:fs'),path=require('node:path'),vm=require('node:vm'),crypto=require('node:crypto');
const models=require('../../spheres-web/ui/arsenal-models.js'),raycast=require('../../spheres-web/ui/equipment-model.js').raycast;
const baseline=require('./fixtures/inf-mech-preservation.json').models;
const source=fs.readFileSync(path.join(__dirname,'../../spheres-web/ui/arsenal-models.js'),'utf8');
const hash=mesh=>{const h=crypto.createHash('sha256');for(const a of [mesh.positions,mesh.normals,mesh.colors])h.update(Buffer.from(a.buffer,a.byteOffset,a.byteLength));return h.digest('hex');};
function probe(lod='near'){
  const calls={tracks:[],wheels:[],louvres:[],lofts:[],bars:[]},context={module:{exports:{}},record:(kind,o,m)=>calls[kind].push({o:JSON.parse(JSON.stringify(o)),matrix:Array.from(m.m)})};
  let code=source;for(const [fn,key] of [['linkedTrack','tracks'],['dishedWheel','wheels'],['louvre','louvres']]){const marker=`function ${fn}(m, o) {`;assert.equal(code.split(marker).length,2);code=code.replace(marker,`${marker} record('${key}',o,m);`);}
  vm.runInNewContext(code,context);const api=context.module.exports,loft=api.Mesh.prototype.loft,bar=api.Mesh.prototype.bar;
  api.Mesh.prototype.loft=function(sections,...args){calls.lofts.push({sections:JSON.parse(JSON.stringify(sections)),matrix:Array.from(this.m)});return loft.call(this,sections,...args);};
  api.Mesh.prototype.bar=function(...args){calls.bars.push({args:args.slice(),matrix:Array.from(this.m)});return bar.apply(this,args);};
  const mesh=api.build('inf_mech',null,{lod});return {calls,mesh};
}
const near=probe(),far=probe('far');
const point=(m,p)=>[0,1,2].map(i=>m[i*4]*p[0]+m[i*4+1]*p[1]+m[i*4+2]*p[2]+m[i*4+3]);

test('only mechanised infantry near changes; all 46 far meshes and catalogue identities are byte-identical',()=>{
  assert.deepEqual(models.ids(),baseline.map(row=>row.id));
  for(const row of baseline){const n=models.build(row.id),f=models.build(row.id,null,{lod:'far'});assert.deepEqual(models.meta(row.id),row.meta);assert.deepEqual(n.min,row.min,row.id+' minimum');assert.deepEqual(n.max,row.max,row.id+' maximum');assert.deepEqual(f.min,row.farMin);assert.deepEqual(f.max,row.farMax);assert.equal(hash(f),row.farHash,row.id+' far geometry');
    if(row.id==='inf_mech')assert.notEqual(hash(n),row.nearHash);else assert.equal(hash(n),row.nearHash,row.id+' near geometry');}
});
test('new formation shape stays within existing budgets while replacing small hardware with structural geometry',()=>{
  const old=baseline.find(row=>row.id==='inf_mech'),mesh=models.build('inf_mech');assert(mesh.count/3<old.nearTriangles,'structural improvement must not pad the already full near budget');assert(mesh.count/3<=16000);assert.equal(models.build('inf_mech',null,{lod:'far'}).count/3,632);
  let bytes=0;for(const id of models.ids())for(const lod of ['near','far']){const m=models.build(id,null,{lod});for(const key of ['positions','normals','colors'])bytes+=m[key].byteLength;}assert(bytes<40*1024*1024);
});
test('six paired road-wheel stations are connected to hull-mounted suspension arms',()=>{
  assert.equal(near.calls.tracks.length,2);assert(near.calls.tracks.every(row=>row.o.wheels===6));assert(far.calls.tracks.every(row=>row.o.wheels===5));
  const wheels=near.calls.wheels.filter(row=>Math.abs(row.o.r-.3528)<1e-7&&Math.abs(row.o.y-.3528)<1e-7);assert.equal(wheels.length,12);const stations=[...new Set(wheels.map(row=>row.o.z))].sort((a,b)=>a-b);assert.equal(stations.length,6);
  for(let i=1;i<stations.length;i++)assert(Math.abs(stations[i]-stations[i-1]-.9188)<1e-6,'regular suspension stations');
  const arms=near.calls.bars.filter(row=>row.args[0]===-.055&&row.args[1]===.055&&row.args[4]===0&&row.args[5]===.35);assert.equal(arms.length,12);
  for(const arm of arms){const mount=point(arm.matrix,[0,0,0]),end=point(arm.matrix,[0,0,.35]);assert(Math.abs(mount[1]-.59)<1e-6);assert(Math.abs(Math.abs(mount[0])-1.17)<1e-6);assert(Math.abs(end[1]-.3528)<.01);assert(stations.some(z=>Math.abs(z-end[2])<.065),'arm endpoint lies on its road-wheel hub');}
});
test('a continuous rear crew bay sits behind the low turret and front engine bank',()=>{
  const hull=near.calls.lofts[0].sections,roof=row=>Math.max(...row.pts.map(p=>p[1]));assert(hull.slice(0,4).every(row=>roof(row)>1.77));assert(roof(hull.at(-2))<1.2,'long forward glacis slopes below the occupied bay');
  const turret=near.calls.lofts.find(row=>Math.abs(row.matrix[7]-1.80)<1e-8&&Math.abs(row.matrix[11]-3.22)<1e-8&&row.sections.length===4);assert(turret,'turret is ahead of the rear crew exits');assert(Math.max(...turret.sections.flatMap(row=>row.pts.map(p=>p[1])))<=.48);
  assert.equal(near.calls.louvres.length,1);const bank=near.calls.louvres[0];assert(bank.matrix[3]>.5,'power pack is beside the driver');assert(bank.matrix[11]>4.4,'power pack is ahead of the turret');assert(bank.o.y1-bank.o.y0>.8);assert(Math.abs(bank.matrix[5])>.2,'grille follows the inclined engine deck');
});
test('rear ramp and troop roof exits are exposed physical faces, not buried intersecting boxes',()=>{
  const ramp=raycast(near.mesh,[0,1.15,-2],[0,0,1]);assert(ramp);assert(Math.abs(ramp.point[2]+.045)<1e-6,'rear approach meets the external ramp plate');
  const door=raycast(near.mesh,[.42,1.16,-2],[0,0,1]);assert(door);assert(door.point[2]<ramp.point[2],'the inset personnel door has a separate accessible face');
  for(const x of [-.6,.6]){const hatch=raycast(near.mesh,[x,3,1.55],[0,-1,0]);assert(hatch);assert(Math.abs(hatch.point[1]-1.832)<1e-6,'roof exit sits just above the crew-bay roof');}
});
test('the redesigned near model has no collapsed faces or reversed smoothed normals',()=>{
  const {positions:p,normals:n}=near.mesh;let smooth=0;
  for(let i=0;i<p.length;i+=9){const ax=p[i+3]-p[i],ay=p[i+4]-p[i+1],az=p[i+5]-p[i+2],bx=p[i+6]-p[i],by=p[i+7]-p[i+1],bz=p[i+8]-p[i+2],face=[ay*bz-az*by,az*bx-ax*bz,ax*by-ay*bx],length=Math.hypot(...face);assert(length>1e-12,'triangle '+i/9+' has area');
    for(let j=0;j<9;j+=3){const norm=[n[i+j],n[i+j+1],n[i+j+2]],facing=face.reduce((v,f,k)=>v+f*norm[k],0)/length;assert(Math.abs(Math.hypot(...norm)-1)<1e-5);assert(facing>0,'normal shares its own winding hemisphere');if(facing<.999)smooth++;}}
  assert(smooth>1000,'wheels and barrel keep meaningful curved shading');
});
