'use strict';
const orderedControls=Object.freeze(['west','east','zoom-in','zoom-out']);
function requestedCameraChange(key,before,after){
  if(!before||!after||![before.yaw,before.pitch,before.zoom,after.yaw,after.pitch,after.zoom].every(Number.isFinite))return false;
  const yaw=Math.atan2(Math.sin(after.yaw-before.yaw),Math.cos(after.yaw-before.yaw));
  if(key==='west'||key==='east')return (key==='west'?yaw<0:yaw>0)&&after.zoom===before.zoom&&after.pitch===before.pitch;
  if(key==='zoom-in'||key==='zoom-out')return (key==='zoom-in'?after.zoom>before.zoom:after.zoom<before.zoom)&&after.yaw===before.yaw&&after.pitch===before.pitch;
  return false;
}
function validInputs(rows){return Array.isArray(rows)&&rows.length===31&&rows.every((row,i)=>
  row.key===orderedControls[i%4]&&row.trusted===true&&Number.isFinite(row.elapsed_ms)&&row.elapsed_ms>=0&&
  requestedCameraChange(row.key,row.before_globe,row.after_globe));}
function referenceGpu(gpu){return !!gpu&&typeof gpu.version==='string'&&gpu.version.startsWith('WebGL')&&
  /ANGLE/i.test(gpu.unmasked||'')&&/NVIDIA.*RTX\s*5070\b/i.test(gpu.unmasked||'')&&
  !/SwiftShader|llvmpipe|software|WARP|Microsoft Basic/i.test(gpu.unmasked||'');}
function completeTrace(trace){return !!trace&&trace.file==='chrome-trace.json.gz'&&trace.raw_bytes>0&&trace.compressed_bytes>0&&/^[a-f0-9]{64}$/.test(trace.sha256||'');}
function markFailure(evidence,error){evidence.passed=false;evidence.failure=String(error?.stack||error);}
module.exports={orderedControls,requestedCameraChange,validInputs,referenceGpu,completeTrace,markFailure};
