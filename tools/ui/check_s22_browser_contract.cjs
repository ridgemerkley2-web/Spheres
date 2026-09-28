'use strict';
const test=require('node:test'),assert=require('node:assert/strict'),fs=require('node:fs'),path=require('node:path'),vm=require('node:vm');
const {orderedControls,requestedCameraChange,validInputs,referenceGpu,completeTrace,markFailure}=require('./s22-browser-contract.cjs');
test('qualification requires every ordered trusted camera action and its specific effect',()=>{
  const before={yaw:0,pitch:.3,zoom:8};
  const rows=Array.from({length:31},(_,i)=>{const key=orderedControls[i%4];return {key,trusted:true,elapsed_ms:10,before_globe:{...before},after_globe:{...before,...(key==='west'?{yaw:-.1}:key==='east'?{yaw:.1}:key==='zoom-in'?{zoom:10.8}:{zoom:8/1.35})}};});
  assert(validInputs(rows));assert(!validInputs(rows.slice(1)));
  for(const edit of [row=>row.key='east',row=>row.trusted=false,row=>row.after_globe.yaw=.1,row=>row.after_globe={...row.before_globe},row=>row.elapsed_ms=NaN,row=>row.after_globe.zoom=9]){
    const bad=structuredClone(rows);edit(bad[0]);assert(!validInputs(bad));
  }
  assert(requestedCameraChange('east',{...before,yaw:Math.PI-.02},{...before,yaw:-Math.PI+.02}),'east through wrap');
  assert(!requestedCameraChange('zoom-in',before,{...before,yaw:.1}),'unrelated camera motion is not zoom completion');
});
test('qualification reference GPU gate accepts only the declared hardware and version',()=>{
  const gpu={version:'WebGL 2.0 (OpenGL ES 3.0 Chromium)',unmasked:'ANGLE (NVIDIA, NVIDIA GeForce RTX 5070 Direct3D11 vs_5_0 ps_5_0, D3D11)'};
  assert(referenceGpu(gpu));for(const value of [null,{...gpu,version:null},{...gpu,unmasked:null},{...gpu,unmasked:'ANGLE (Google, Vulkan SwiftShader)'},{...gpu,unmasked:gpu.unmasked.replace('5070','4090')},{...gpu,unmasked:gpu.unmasked+' software'}])assert(!referenceGpu(value));
});
test('missing or failed trace cannot retain a passing verdict',()=>{
  const trace={file:'chrome-trace.json.gz',raw_bytes:10,compressed_bytes:20,sha256:'a'.repeat(64)};assert(completeTrace(trace));
  for(const value of [null,{...trace,raw_bytes:0},{...trace,compressed_bytes:0},{...trace,sha256:''}])assert(!completeTrace(value));
  const evidence={passed:true};markFailure(evidence,new Error('IO.read failed'));assert.equal(evidence.passed,false);assert.match(evidence.failure,/IO.read failed/);
});
test('first-draw observer ignores zero-count calls and records actual GL completion once per phase',()=>{
  let time=0,draws=0,finishes=0;const canvas={id:'glmap',isConnected:true,width:100,height:80,matches:()=>false};
  const gl={canvas,isContextLost:()=>false,finish(){finishes++;time+=2;},drawArrays(){draws++;time++;}};
  function Canvas(){}Canvas.prototype.getContext=function(){return gl;};
  const context=vm.createContext({window:{},HTMLCanvasElement:Canvas,performance:{timeOrigin:1000,now:()=>time}});
  vm.runInContext(fs.readFileSync(path.join(__dirname,'s22-browser-observation.js'),'utf8'),context);
  const element=new Canvas();element.getContext('webgl2');gl.drawArrays(4,0,0);assert.equal(finishes,0);
  gl.drawArrays(4,0,3);gl.drawArrays(4,0,3);assert.equal(finishes,1);
  const probe=context.window.__s22PageObservation;assert.equal(probe.contexts.length,1);assert.equal(probe.contexts[0].first_draw.count,3);
  assert.equal(probe.contexts[0].first_draw.completed_ms,4);probe.setPhase('campaign-save-load');gl.drawArrays(4,0,6);
  assert.equal(finishes,2);assert.equal(probe.contexts[0].first_draw_by_phase.length,2);assert.equal(draws,4);
});
