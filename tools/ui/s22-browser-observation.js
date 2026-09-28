/* Qualification-only observation installed before navigation. No game data is changed. */
(()=>{
  const contexts=[],known=new WeakMap();
  const probe=window.__s22PageObservation={time_origin_ms:performance.timeOrigin,phase:'uncached-page-entry',
    phases:[{name:'uncached-page-entry',at_ms:performance.now()}],contexts};
  probe.setPhase=name=>{probe.phase=name;probe.phases.push({name,at_ms:performance.now()});};
  const original=HTMLCanvasElement.prototype.getContext;
  HTMLCanvasElement.prototype.getContext=function(type,...args){
    const gl=original.call(this,type,...args);
    if(!gl||!['webgl','webgl2','experimental-webgl'].includes(type)||known.has(gl))return gl;
    const row={id:contexts.length,type,created_ms:performance.now(),first_draw:null,first_draw_by_phase:[]};
    known.set(gl,row);contexts.push(row);const phases=new Set();
    for(const name of ['drawArrays','drawElements','drawArraysInstanced','drawElementsInstanced']){
      if(typeof gl[name]!=='function')continue;
      const draw=gl[name];gl[name]=function(...args){
        const count=name.startsWith('drawArrays')?args[2]:args[1];
        const instances=name==='drawArraysInstanced'?args[3]:name==='drawElementsInstanced'?args[4]:1;
        const first=count>0&&instances>0&&!phases.has(probe.phase)&&!gl.isContextLost();
        const started=first?performance.now():null,result=draw.apply(this,args);
        if(first){
          gl.finish();const completed=performance.now();phases.add(probe.phase);
          const event={phase:probe.phase,method:name,count,started_ms:started,completed_ms:completed,
            canvas_id:gl.canvas.id,inspection_canvas:gl.canvas.matches('[data-model-canvas]'),connected:gl.canvas.isConnected,width:gl.canvas.width,height:gl.canvas.height,
            design_platform:typeof EQUIP!=='undefined'?EQUIP.draft?.platform||null:null};
          row.first_draw??=event;row.first_draw_by_phase.push(event);
        }
        return result;
      };
    }
    return gl;
  };
})();
