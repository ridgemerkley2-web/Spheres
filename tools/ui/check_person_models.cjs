const {test}=require('node:test'),assert=require('node:assert/strict');
const fs=require('node:fs'),path=require('node:path'),crypto=require('node:crypto');
const models=require('../../spheres-web/ui/person-models.js');
const {glb}=require('../../spheres-web/ui/equipment-export.js');
const viewer=require('../../spheres-web/ui/person-3d.js');
const {render}=require('../../spheres-web/ui/government-ui.js');
const registry=require('../../spheres-sim/data/party_leaders.json');
const source=require('../../spheres-web/data/person_models.json');
const seen=new Set();
for(const id of models.ids())test('physical character, identity, era and downloadable geometry: '+id,()=>{
  const meta=models.meta(id),person=registry.people.find(p=>p.id===meta.person_id);
  assert.ok(person);assert.equal(meta.name,person.name);assert.equal(meta.status,'likeness-study');
  assert.ok(meta.from<meta.to);assert.ok(meta.sources.every(url=>url.startsWith('https://')));
  for(const other of source.characters.filter(p=>p.id!==id&&p.person_id===meta.person_id))assert.ok(other.to<=meta.from||meta.to<=other.from,'appearance eras cannot overlap');
  const mesh=models.build(id);assert.ok(mesh.triangleCount>10000&&mesh.triangleCount<35000);
  assert.equal(mesh.assetKind,'character');assert.equal(mesh.id,'person:'+id);
  assert.equal(mesh.positions.length,mesh.normals.length);assert.equal(mesh.positions.length,mesh.colors.length);
  assert.ok(mesh.parts.some(p=>p.name==='Face and ears'));assert.ok(mesh.parts.some(p=>p.name==='Sculpted hair'));
  let next=0;for(const part of mesh.parts){assert.equal(part.first,next);next+=part.count;}assert.equal(next,mesh.positions.length/3);
  assert.ok(mesh.bounds.max[1]-mesh.bounds.min[1]>4.5);assert.ok(mesh.bounds.max[2]-mesh.bounds.min[2]>1.2,'real volume, not a flat portrait');
  const exported=Buffer.from(glb(mesh,meta.name)),disk=fs.readFileSync(path.resolve(__dirname,'../../spheres-web/ui/person-models',id+'.glb'));
  assert.deepEqual(disk,exported);assert.equal(disk.toString('ascii',0,4),'glTF');assert.equal(disk.readUInt32LE(4),2);assert.equal(disk.readUInt32LE(8),disk.length);
  const doc=JSON.parse(disk.toString('utf8',20,20+disk.readUInt32LE(12)).trim());
  assert.equal(doc.asset.generator,'Spheres Character Studio');assert.equal(doc.meshes[0].extras.specification.person_id,person.id);
  assert.equal(doc.accessors[0].count,mesh.positions.length/3);assert.equal(doc.materials[0].name,'Sculpted character');
  const hash=crypto.createHash('sha256').update(Buffer.from(mesh.positions.buffer)).digest('hex');assert.ok(!seen.has(hash),'each named character needs distinct physical features');seen.add(hash);
});
test('unknown or malicious IDs never borrow a person or build a generic character',()=>{
  for(const id of ['UK','george_bush','constructor','__proto__','../margaret_thatcher_1990_v1'])assert.equal(models.build(id),null);
});
test('camera is manually controlled, bounded and resettable without changing its input',()=>{
  const start=viewer.initial();assert.deepEqual(viewer.change(start,'left'),{yaw:347,pitch:5,zoom:1});assert.deepEqual(start,viewer.initial());
  let v=start;for(let i=0;i<100;i++)v=viewer.change(viewer.change(v,'in'),'up');assert.equal(v.zoom,1.8);assert.equal(v.pitch,35);
  for(let i=0;i<100;i++)v=viewer.change(viewer.change(v,'out'),'down');assert.equal(v.zoom,.75);assert.equal(v.pitch,-20);
  assert.deepEqual(viewer.change(v,'reset'),start);assert.equal(viewer.change(start,'drag',100,100).yaw,67);
  assert.deepEqual(viewer.change(start,'drag',NaN,Infinity),start);
});
test('Government shows the actual bound executive model and provides accessible manual controls',()=>{
  const person={id:'margaret_thatcher',name:'Margaret Thatcher',portrait:{method:'procedural_3d',model_id:'margaret_thatcher_1990_v1'}};
  const data={nation:'UK',leader:{name:person.name},party_leadership:{executive_person:person,parties:[]},actions:[]};
  const html=render(data,{tab:'overview'});
  assert.match(html,/data-person-model="margaret_thatcher_1990_v1"/);assert.match(html,/Open 3D character of Margaret Thatcher/);
  assert.match(html,/Rotate Margaret Thatcher left/);assert.match(html,/3D likeness study/);assert.doesNotMatch(html,/src="[^"]*Thatcher/);
  person.portrait.model_id='x" onerror="alert(1)';assert.doesNotMatch(render(data,{tab:'overview'}),/data-person-model=/);
  delete data.party_leadership.executive_person;assert.doesNotMatch(render(data,{tab:'overview'}),/data-person-model=/);
});
