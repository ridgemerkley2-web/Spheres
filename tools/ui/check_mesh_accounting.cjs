const {test}=require('node:test'),assert=require('node:assert/strict');
const {measureMesh,sumPayloads}=require('./mesh-accounting.cjs');
function mesh(){return {triangleCount:2,positions:new Float32Array(18),normals:new Float32Array(18),colors:new Float32Array(18),materialClasses:new Uint8Array(6)};}
test('CPU material tags are counted without being mistaken for an uploaded float attribute',()=>{
  const m=measureMesh(mesh());assert.equal(m.cpu_attribute_view_bytes,222);assert.equal(m.cpu_backing_buffer_bytes,222);assert.equal(m.base_attribute_upload_bytes,216);
  const classic=mesh();delete classic.materialClasses;assert.equal(measureMesh(classic).cpu_attribute_view_bytes,216);
});
test('shared backing allocation is counted once, while each GPU attribute upload has its own payload',()=>{
  const m=mesh(),backing=new ArrayBuffer(400);m.positions=new Float32Array(backing,0,18);m.normals=new Float32Array(backing,72,18);m.colors=m.positions;
  const measured=measureMesh(m);assert.equal(measured.cpu_attribute_view_bytes,222);assert.equal(measured.cpu_backing_buffer_bytes,406);assert.equal(measured.base_attribute_upload_bytes,216);
});
test('changed or corrupt layouts refuse measurement rather than silently excluding bytes',()=>{
  for(const change of [m=>m.uvs=new Float32Array(12),m=>m.indices=new Uint16Array(6),m=>m.colors=new Float64Array(18),m=>m.materialClasses=new Uint8Array(5),m=>m.triangleCount=1.5,m=>delete m.normals]){
    const m=mesh();change(m);assert.throws(()=>measureMesh(m,'changed model'),/changed model:/);
  }
});
test('inventory totals sum measured payloads without assuming one byte rate per triangle',()=>{
  const rows=[{tris:2,bytes:216,accounting:{cpu_backing_buffer_bytes:222}},{tris:1,bytes:156,accounting:{cpu_backing_buffer_bytes:111}}];
  assert.deepEqual(sumPayloads(rows),{n:2,tris:3,bytes:372,cpuBytes:333});
});
