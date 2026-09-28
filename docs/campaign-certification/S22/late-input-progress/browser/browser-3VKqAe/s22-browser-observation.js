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
    known.set(gl,row);contexts.push(row);const phases=new Map();
    const inspectionCandidate=gl.canvas.matches('canvas[data-equipment-focus="model-canvas"]')&&!!gl.canvas.closest('[data-model-canvas]');
    for(const name of ['drawArrays','drawElements','drawArraysInstanced','drawElementsInstanced']){
      if(typeof gl[name]!=='function')continue;
      const draw=gl[name];gl[name]=function(...args){
        const count=name.startsWith('drawArrays')?args[2]:args[1];
        const instances=name==='drawArraysInstanced'?args[3]:name==='drawElementsInstanced'?args[4]:1;
        // The released viewer appends its actual GL canvas INSIDE this slot;
        // the slot itself is a div. Arsenal3D's offscreen thumbnail context and
        // the globe are not inspection canvases, even while a fighter is selected.
        const inspection=inspectionCandidate&&typeof EQUIPMENT_VIEWER!=='undefined'&&
          !!EQUIPMENT_VIEWER.host?.contains(gl.canvas);
        const modelKey=inspection?EQUIPMENT_VIEWER.key:null;
        // One canvas can render several specifications without being recreated.
        // A ground-model draw while the family click is pending must not consume
        // the fighter's first-draw observation for this phase.
        const first=count>0&&instances>0&&!phases.get(probe.phase)?.has(modelKey)&&!gl.isContextLost();
        const started=first?performance.now():null,result=draw.apply(this,args);
        if(first){
          gl.finish();const completed=performance.now();
          if(!phases.has(probe.phase))phases.set(probe.phase,new Set());
          phases.get(probe.phase).add(modelKey);
          let renderedPlatform=null;
          if(modelKey){try{renderedPlatform=JSON.parse(modelKey).platform||null;}catch{}}
          const event={phase:probe.phase,method:name,count,started_ms:started,completed_ms:completed,
            canvas_id:gl.canvas.id,inspection_canvas:inspection,connected:gl.canvas.isConnected,width:gl.canvas.width,height:gl.canvas.height,
            rendered_platform:renderedPlatform,model_key:modelKey,
            design_platform:typeof EQUIP!=='undefined'?EQUIP.draft?.platform||null:null};
          row.first_draw??=event;row.first_draw_by_phase.push(event);
        }
        return result;
      };
    }
    return gl;
  };
})();
