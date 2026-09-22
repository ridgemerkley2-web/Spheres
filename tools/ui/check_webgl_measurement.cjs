const {test}=require('node:test'),assert=require('node:assert/strict');
const {attach,interval}=require('./webgl-measurement.js');
function fixture(){
  const allocated=new Map(),listeners=new Map();let bound=null,offscreen=null,lost=false;
  const gl={ARRAY_BUFFER:1,ARRAY_BUFFER_BINDING:2,BUFFER_SIZE:3,FRAMEBUFFER_BINDING:4,TRIANGLES:5,TRIANGLE_STRIP:6,TRIANGLE_FAN:7,
    createBuffer(){const b={};allocated.set(b,0);return b;},bindBuffer(t,b){bound=b;},
    bufferData(t,data,usage,offset=0,length){allocated.set(bound,typeof data==='number'?data:(length===undefined?data.length-offset:length||data.length-offset)*data.BYTES_PER_ELEMENT);},
    getParameter(p){return p===2?bound:offscreen;},getBufferParameter(){return allocated.get(bound);},
    deleteBuffer(b){allocated.delete(b);},drawArrays(){},drawElements(){},drawArraysInstanced(){},isContextLost:()=>lost,
    canvas:{addEventListener:(name,fn)=>listeners.set(name,fn),removeEventListener:name=>listeners.delete(name)}};
  return {gl,allocated,listeners,shadow:value=>offscreen=value,lose(){lost=true;allocated.clear();listeners.get('webglcontextlost')();},restore(){lost=false;}};
}
test('buffer replacement and deletion count residency; repeated passes count submissions without inventing storage',()=>{
  const f=fixture(),m=attach(f.gl),gl=f.gl,b=gl.createBuffer();gl.bindBuffer(gl.ARRAY_BUFFER,b);gl.bufferData(1,new Float32Array(9));
  const before=m.snapshot();gl.drawArrays(5,0,3);f.shadow({});gl.drawArrays(5,0,3);f.shadow(null);gl.drawArrays(5,0,3);
  assert.deepEqual(interval(before,m.snapshot()),{resident_buffer_payload_bytes:36,peak_buffer_payload_bytes:36,live_buffers:1,buffer_data_calls:0,draw_calls:3,submitted_triangles:3,offscreen_submitted_triangles:1});
  gl.bufferData(1,12);assert.equal(m.snapshot().resident_buffer_payload_bytes,12);assert.equal(m.snapshot().peak_buffer_payload_bytes,36);
  gl.deleteBuffer(b);gl.deleteBuffer(b);assert.equal(m.snapshot().resident_buffer_payload_bytes,0);assert.equal(m.snapshot().live_buffers,0);
});
test('WebGL2 ranged uploads use actual BUFFER_SIZE; indexed and instanced calls count submitted primitives',()=>{
  const {gl}=fixture(),m=attach(gl),b=gl.createBuffer();gl.bindBuffer(1,b);gl.bufferData(1,new Float32Array(100),0,2,6);
  assert.equal(m.snapshot().resident_buffer_payload_bytes,24);gl.drawElements(5,6);gl.drawArraysInstanced(6,0,4,5);
  assert.equal(m.snapshot().submitted_triangles,12);m.stop();assert.equal(gl.createBuffer.name,'createBuffer');
});
test('context loss drops live buffers and restore starts from zero, preserving a cumulative peak',()=>{
  const f=fixture(),m=attach(f.gl),gl=f.gl;gl.bindBuffer(1,gl.createBuffer());gl.bufferData(1,100);f.lose();
  assert.equal(m.snapshot().resident_buffer_payload_bytes,0);gl.drawArrays(5,0,3);assert.equal(m.snapshot().draw_calls,0);
  f.restore();gl.bindBuffer(1,gl.createBuffer());gl.bufferData(1,40);
  assert.equal(m.snapshot().resident_buffer_payload_bytes,40);assert.equal(m.snapshot().peak_buffer_payload_bytes,100);assert.equal(m.snapshot().context_losses,1);
  m.stop();assert.equal(f.listeners.size,0);
});
