/* Test-tool instrumentation only. Not loaded by the game. Measures API buffer
 * payload storage and draw submissions, not driver VRAM, visibility or FPS. */
(function(root){
  'use strict';
  function attach(gl) {
    const sizes=new Map(),originals=new Map(),diagnostics=new Set();
    let peak=0,resident=0,uploads=0,uploaded=0,draws=0,triangles=0,offscreen=0,losses=0;
    const patch=(name,after)=>{
      if(typeof gl[name]!=='function')return;
      const original=gl[name];originals.set(name,original);
      gl[name]=function(...args){const result=original.apply(this,args);after(args,result);return result;};
    };
    patch('createBuffer',(_,buffer)=>{if(buffer)sizes.set(buffer,0);});
    patch('bufferData',([target])=>{
      if(gl.isContextLost())return;
      const name=Object.entries({ARRAY_BUFFER: 'ARRAY_BUFFER_BINDING',ELEMENT_ARRAY_BUFFER: 'ELEMENT_ARRAY_BUFFER_BINDING',
        COPY_READ_BUFFER: 'COPY_READ_BUFFER_BINDING',COPY_WRITE_BUFFER: 'COPY_WRITE_BUFFER_BINDING',
        PIXEL_PACK_BUFFER: 'PIXEL_PACK_BUFFER_BINDING',PIXEL_UNPACK_BUFFER: 'PIXEL_UNPACK_BUFFER_BINDING',
        TRANSFORM_FEEDBACK_BUFFER: 'TRANSFORM_FEEDBACK_BUFFER_BINDING',UNIFORM_BUFFER: 'UNIFORM_BUFFER_BINDING'})
        .find(([key])=>gl[key]!==undefined&&gl[key]===target)?.[1];
      if(!name)throw new Error(`Unmeasured buffer target ${target}`);
      const buffer=gl.getParameter(gl[name]);if(!buffer)return;
      const bytes=gl.getBufferParameter(target,gl.BUFFER_SIZE);
      resident+=bytes-(sizes.get(buffer)||0);sizes.set(buffer,bytes);peak=Math.max(peak,resident);uploads++;uploaded+=bytes;
    });
    patch('deleteBuffer',([buffer])=>{resident-=sizes.get(buffer)||0;sizes.delete(buffer);});
    function submit(mode,count,instances=1){
      if(gl.isContextLost())return;
      const n=(mode===gl.TRIANGLES?Math.floor(count/3):mode===gl.TRIANGLE_STRIP||mode===gl.TRIANGLE_FAN?Math.max(0,count-2):0)*instances;
      draws++;triangles+=n;if(gl.getParameter(gl.FRAMEBUFFER_BINDING))offscreen+=n;
    }
    patch('drawArrays',([mode,first,count])=>submit(mode,count));
    patch('drawElements',([mode,count])=>submit(mode,count));
    patch('drawArraysInstanced',([mode,first,count,instances])=>submit(mode,count,instances));
    patch('drawElementsInstanced',([mode,count,type,offset,instances])=>submit(mode,count,instances));
    for(const name of ['getShaderInfoLog','getProgramInfoLog'])patch(name,(_,message)=>{if(message)diagnostics.add(`${name}: ${message}`);});
    const lost=()=>{sizes.clear();resident=0;losses++;};
    gl.canvas?.addEventListener('webglcontextlost',lost);
    return {
      snapshot(){return {resident_buffer_payload_bytes:resident,peak_buffer_payload_bytes:peak,live_buffers:sizes.size,
        buffer_data_calls:uploads,buffer_data_payload_bytes:uploaded,draw_calls:draws,
        submitted_triangles:triangles,offscreen_submitted_triangles:offscreen,context_losses:losses};},
      diagnostics(){return [...diagnostics];},
      stop(){for(const [name,original] of originals)gl[name]=original;gl.canvas?.removeEventListener('webglcontextlost',lost);}
    };
  }
  function interval(before,after){
    return {resident_buffer_payload_bytes:after.resident_buffer_payload_bytes,
      peak_buffer_payload_bytes:after.peak_buffer_payload_bytes,live_buffers:after.live_buffers,
      buffer_data_calls:after.buffer_data_calls-before.buffer_data_calls,
      draw_calls:after.draw_calls-before.draw_calls,
      submitted_triangles:after.submitted_triangles-before.submitted_triangles,
      offscreen_submitted_triangles:after.offscreen_submitted_triangles-before.offscreen_submitted_triangles};
  }
  const api={attach,interval};
  if(typeof module==='object'&&module.exports)module.exports=api;else root.WebGLMeasurement=api;
})(typeof globalThis==='undefined'?this:globalThis);
