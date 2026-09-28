'use strict';
const orderedControls=Object.freeze(['west','east','zoom-in','zoom-out']);
function requestedCameraChange(key,before,after,beforeMap,afterMap){
  if(!before||!after||![before.yaw,before.pitch,before.zoom,after.yaw,after.pitch,after.zoom].every(Number.isFinite))return false;
  const yaw=Math.atan2(Math.sin(after.yaw-before.yaw),Math.cos(after.yaw-before.yaw));
  if(key==='west'||key==='east')return (key==='west'?yaw<0:yaw>0)&&after.zoom===before.zoom&&after.pitch===before.pitch;
  if(key==='zoom-in'||key==='zoom-out'){
    // mapZoom preserves the Robinson map centre; unprojectFree recovers its
    // latitude with 24 bisections over 172.8 degrees. The first round trip can
    // change the globe angle by <1e-7 radians despite the exact same centre.
    // Require that unchanged finite centre as well as bounded angular drift.
    const exact=after.yaw===before.yaw&&after.pitch===before.pitch;
    const sameCentre=beforeMap&&afterMap&&['cx','cy'].every(k=>Number.isFinite(beforeMap[k])&&beforeMap[k]===afterMap[k]);
    return (key==='zoom-in'?after.zoom>before.zoom:after.zoom<before.zoom)&&
      (exact||(sameCentre&&Math.abs(yaw)<=1e-7&&Math.abs(after.pitch-before.pitch)<=1e-7));
  }
  return false;
}
function validInputs(rows){return Array.isArray(rows)&&rows.length===31&&rows.every((row,i)=>
  row.key===orderedControls[i%4]&&row.trusted===true&&Number.isFinite(row.elapsed_ms)&&row.elapsed_ms>=0&&
  requestedCameraChange(row.key,row.before_globe,row.after_globe,row.before_camera,row.after_camera));}
function referenceGpu(gpu){return !!gpu&&typeof gpu.version==='string'&&gpu.version.startsWith('WebGL')&&
  /ANGLE/i.test(gpu.unmasked||'')&&/NVIDIA.*RTX\s*5070\b/i.test(gpu.unmasked||'')&&
  !/SwiftShader|llvmpipe|software|WARP|Microsoft Basic/i.test(gpu.unmasked||'');}
function completeTrace(trace){return !!trace&&trace.file==='chrome-trace.json.gz'&&trace.raw_bytes>0&&trace.compressed_bytes>0&&/^[a-f0-9]{64}$/.test(trace.sha256||'');}
function markFailure(evidence,error){evidence.passed=false;evidence.failure=String(error?.stack||error);}
module.exports={orderedControls,requestedCameraChange,validInputs,referenceGpu,completeTrace,markFailure};
