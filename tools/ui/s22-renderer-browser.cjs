'use strict';
// Native campaign renderer qualification with a separately labeled viewer
// orbit sample. The caller
// owns a disposable native server and a copied campaign. Call install(page)
// BEFORE goto, load through ordinary controls, then run({...}). No command,
// save, load, fixture invention or simulation mutation occurs in this helper.
const assert=require('node:assert/strict'),fs=require('node:fs'),path=require('node:path'),crypto=require('node:crypto');
const root=path.resolve(__dirname,'../..'),hash=bytes=>crypto.createHash('sha256').update(bytes).digest('hex');
const files=['index.html','city-mesh.js','city-layer.js','arsenal3d.js','arsenal-models.js','equipment-model.js',
  'equipment-mesh.js','equipment-ui.js','map-controls.js','terrain-surface.js','globe3d.js','cities.js','rivers.js',
  'water-detail.js','height-detail.js','coast.png','lake.png','relief.png','tank-surface.js','military-surface.js'];

async function install(page) {
  await page.addInitScript({content:fs.readFileSync(path.join(__dirname,'webgl-measurement.js'),'utf8')});
  await page.addInitScript({content:fs.readFileSync(path.join(__dirname,'webgl-texture-measurement.js'),'utf8')});
  await page.addInitScript(()=>{
    const contexts=[],original=HTMLCanvasElement.prototype.getContext;
    const counters={resident:0,peak:0,uploads:0,uploaded:0,deletions:0,draws:0,triangles:0,losses:0};
    const cities=new Map();
    window.__s22Renderer={contexts,counters,cities,buildingCity:false,drawingCity:false};
    HTMLCanvasElement.prototype.getContext=function(type,...args) {
      const gl=original.call(this,type,...args);
      if(!gl||!['webgl','webgl2'].includes(type)||contexts.some(row=>row.gl===gl))return gl;
      const metrics=WebGLMeasurement.attach(gl),textures=WebGLTextureMeasurement.attach(gl),row={id:contexts.length,type,canvas:this,gl,metrics,textures};contexts.push(row);
      const create=gl.createBuffer,upload=gl.bufferData,remove=gl.deleteBuffer,draw=gl.drawArrays;
      gl.createBuffer=function(...args){const buffer=create.apply(this,args);if(buffer&&__s22Renderer.buildingCity)cities.set(buffer,{gl,bytes:0});return buffer;};
      gl.bufferData=function(target,...args){
        const result=upload.call(this,target,...args);
        if(target===gl.ARRAY_BUFFER&&!gl.isContextLost()){
          const buffer=gl.getParameter(gl.ARRAY_BUFFER_BINDING),entry=cities.get(buffer);
          if(entry){const bytes=gl.getBufferParameter(target,gl.BUFFER_SIZE);counters.resident+=bytes-entry.bytes;entry.bytes=bytes;
            counters.peak=Math.max(counters.peak,counters.resident);counters.uploads++;counters.uploaded+=bytes;}
        }
        return result;
      };
      gl.deleteBuffer=function(buffer){const entry=cities.get(buffer);if(entry){counters.resident-=entry.bytes;cities.delete(buffer);counters.deletions++;}return remove.call(this,buffer);};
      gl.drawArrays=function(mode,first,count){const result=draw.call(this,mode,first,count);
        if(__s22Renderer.drawingCity&&!gl.isContextLost()){counters.draws++;if(mode===gl.TRIANGLES)counters.triangles+=Math.floor(count/3);}return result;};
      gl.canvas.addEventListener('webglcontextlost',()=>{
        let affected=false;for(const [buffer,entry]of cities)if(entry.gl===gl){counters.resident-=entry.bytes;cities.delete(buffer);affected=true;}
        if(affected||gl.canvas.id==='glmap')counters.losses++;
      });return gl;
    };
  });
}

async function run({page,tap,state,shot,evidence,expectedRevision}) {
  const out=evidence.out;assert(out);assert.match(expectedRevision||evidence.runtime_revision||'',/^[a-f0-9]{40}$/);
  const proof=evidence.renderer={passed:false,memory_complete:false,method:'Actual native campaign rendering, queried WebGL buffer payload and separate declared texture texel payload. Texture requests are not verified physical allocations; unknown layouts remain null. City attribution wraps released functions without changing their logic. Renderbuffers, framebuffer surfaces, driver overhead and CPU/GPU process memory are excluded. Fighter orbit timing measures submitted/completed draw frames, not compositor presentation or map FPS. Lazy-card fixture is isolated DOM using released models, not campaign catalogue usability.',
    observations:[],sources:[],checks:[],errors:[],requests:[],started_utc:new Date().toISOString()};
  const onError=error=>proof.errors.push(error.message);
  const onRequest=request=>{if(request.method()==='POST')proof.requests.push({path:new URL(request.url()).pathname,payload:request.postDataJSON()});};
  page.on('pageerror',onError);page.on('request',onRequest);
  const record=(name,data)=>{const file=name+'.json',bytes=Buffer.from(JSON.stringify(data,null,2)+'\n');fs.writeFileSync(path.join(out,file),bytes);
    proof.observations.push({file,bytes:bytes.length,sha256:hash(bytes)});return data;};
  const settle=()=>page.evaluate(()=>new Promise(resolve=>requestAnimationFrame(()=>requestAnimationFrame(resolve))));
  const idle=()=>page.waitForFunction(()=>!SESSION.busy&&!advancing&&!pendingAdvance&&!COMMAND_CHANNEL.busy&&!COMMAND_CHANNEL.pending&&!EQUIP.busy);
  const snapshot=label=>page.evaluate(label=>{
    const probe=__s22Renderer,cache=typeof GLR!=='undefined'&&GLR?.cityCache;
    const cityRows=cache?[...cache].map(([index,row])=>({index,name:CITIES[index]?.name,triangles:row.tris,span:row.span,
      retained_model_position_bytes:row.model?.positions?.byteLength||0})):[];
    return {label,date:S.date,session_id:S.session_id,viewport:{width:innerWidth,height:innerHeight,dpr:devicePixelRatio},
      map:{ready:GL.ready,ok:GL.ok,reason:GL.reason,zoom:GLOBE.zoom,details:{...ui.mapDetails}},
      city:{...probe.counters,entries:cityRows,cap_triangles:CITY_LAYER.cacheTriangles,cap_count:CITY_LAYER.cache},
      arsenal:Arsenal3D.cacheStats(),contexts:probe.contexts.map(row=>({id:row.id,type:row.type,canvas_id:row.canvas.id,
        connected:row.canvas.isConnected,metrics:row.metrics.snapshot(),diagnostics:row.metrics.diagnostics(),
        declared_texture_payload:row.textures.snapshot(),texture_diagnostics:row.textures.diagnostics(),
        renderer:row.gl.getParameter(row.gl.RENDERER),vendor:row.gl.getParameter(row.gl.VENDOR)})),
      js_heap:performance.memory?{used:performance.memory.usedJSHeapSize,total:performance.memory.totalJSHeapSize,limit:performance.memory.jsHeapSizeLimit}:null};
  },label);
  function assertCity(row) {
    const triangles=row.city.entries.reduce((sum,entry)=>sum+entry.triangles,0);
    assert.equal(row.city.cap_triangles,800000);assert.equal(row.city.cap_count,8);
    assert(row.city.entries.length<=8);assert(triangles<=800000);
    assert.equal(row.city.resident,triangles*108,'queried city buffers agree with the live native cache');
    assert(row.city.peak<=86400000,'city payload cap holds during upload');
    assert(row.arsenal.triangles<=row.arsenal.cap);assert.equal(row.arsenal.cap,1200000);
  }
  function assertTextures(row){for(const context of row.contexts){const t=context.declared_texture_payload;
    assert.equal(t.probe_errors,0,'texture measurement errors');assert.equal(t.unmeasured_allocation_events,0,'all texture allocations measured');
    assert.equal(t.unmeasured_texture_allocations,0,'all live texture layouts known');assert.notEqual(t.declared_texture_texel_payload_bytes,null);
    assert.notEqual(t.peak_declared_texture_texel_payload_bytes,null);}}
  async function observe(label){const row=record(label,await snapshot(label));assertCity(row);assertTextures(row);return row;}
  async function closeRooms(){
    for(const [panel,close]of [['#guidanceDialog','#guidanceDialog [data-guidance-close]'],['#equipmentRoom','[data-equipment-close]'],
      ['#productionPanel','#productionClose'],['#cabinetDrawer','#cabinetDrawer [data-close-drawers]'],
      ['#provinceDossier','#provinceDossier .province-close'],['#mapCityCard','[data-map-detail-focus="city-close"]']])
      if(await page.locator(panel).isVisible())await tap(close);
  }
  async function findCity(name){
    await closeRooms();await tap('#worldFindBtn');await page.locator('#worldFindInput').fill(name);
    const result=page.locator('[data-find-kind="city"]').filter({has:page.getByText(name,{exact:true})});
    await result.waitFor({state:'visible'});await result.click();await page.locator('#mapCityCard').waitFor({state:'visible'});
    for(let step=0;step<24&&(await page.evaluate(()=>GLOBE.zoom))<1499;step++)await tap('[data-map-action="zoom-in"]');
    assert((await page.evaluate(()=>GLOBE.zoom))>=1499,'exercise the released close-city span');
    await page.waitForFunction(name=>GL.ready&&GL.ok&&GLR?.cityCache&&[...GLR.cityCache.keys()].some(index=>CITIES[index].name===name),name,{timeout:120000});
    await settle();
  }
  async function detail(key,on){
    if(await page.locator(`button[data-map-detail="${key}"]`).getAttribute('aria-pressed')===String(on))return;
    const menu=page.locator('#mapControls .map-detail-menu');
    if(await menu.getAttribute('open')===null)await tap('#mapControls .map-detail-menu > summary');
    await tap(`button[data-map-detail="${key}"]`);
    assert.equal(await page.locator(`button[data-map-detail="${key}"]`).getAttribute('aria-pressed'),String(on));
    if(await menu.getAttribute('open')!==null)await tap('#mapControls .map-detail-menu > summary');await settle();
  }
  async function loseRestore(kind){
    const before=await page.evaluate(kind=>{
      const rows=__s22Renderer.contexts;
      const row=kind==='globe'?rows.find(row=>row.canvas.id==='glmap'):
        kind==='inspection'?rows.findLast(row=>row.type==='webgl'&&row.canvas.isConnected):
        rows.find(row=>row.type==='webgl2'&&row.canvas.id!=='glmap');
      if(!row)throw Error('Missing actual '+kind+' context');
      const extension=row.gl.getExtension('WEBGL_lose_context');if(!extension)throw Error('Context-loss injection unavailable');
      __s22Renderer.loss={id:row.id,extension};const metrics=row.metrics.snapshot(),textures=row.textures.snapshot();extension.loseContext();return {id:row.id,metrics,textures};
    },kind);
    await page.waitForFunction(({id,losses})=>__s22Renderer.contexts[id].metrics.snapshot().context_losses>losses,{id:before.id,losses:before.metrics.context_losses});
    const lost=await page.evaluate(id=>({buffers:__s22Renderer.contexts[id].metrics.snapshot(),textures:__s22Renderer.contexts[id].textures.snapshot()}),before.id);
    assert.equal(lost.buffers.live_buffers,0);assert.equal(lost.textures.live_textures,0);assert.equal(lost.textures.allocated_texture_levels,0);
    assert.equal(lost.textures.declared_texture_texel_payload_bytes,0);
    await page.evaluate(()=>__s22Renderer.loss.extension.restoreContext());
    if(kind==='globe')await page.waitForFunction(()=>GL.ready&&GL.ok&&GLCV.getClientRects().length&&!document.querySelector('#globeFail'),null,{timeout:120000});
    else await page.waitForFunction(id=>__s22Renderer.contexts[id].metrics.snapshot().live_buffers>0,before.id,{timeout:120000});
    if(before.textures.declared_texture_texel_payload_bytes>0)await page.waitForFunction(id=>__s22Renderer.contexts[id].textures.snapshot().declared_texture_texel_payload_bytes>0,before.id,{timeout:120000});
    await settle();const after=await snapshot(kind+' restored');assertTextures(after);
    record('s22-'+kind+'-context-recovery',{before,lost,after,texture_scope:'Loss clears every tracked texture allocation. Restoration recreates payload when the original context had textures; exact before/after sizes are observations, not asserted equal while asynchronous material uploads settle.'});
  }
  async function measureFighterOrbit(){
    const measured=await page.evaluate(async()=>{
      const controller=EQUIPMENT_VIEWER.controller;
      const row=__s22Renderer.contexts.findLast(row=>row.type==='webgl'&&row.canvas.isConnected);
      if(!controller||!row||EQUIP.draft?.platform!=='air_fighter')throw Error('Released fighter viewer is not active');
      if(document.hidden)throw Error('Viewer timing requires a visible document');
      const {gl,metrics}=row,nativeRaf=window.requestAnimationFrame.bind(window);
      const summary=values=>{const sorted=[...values].sort((a,b)=>a-b);return {samples:sorted.length,
        min_ms:sorted[0]??null,median_ms:sorted[Math.floor(sorted.length/2)]??null,
        p95_ms:sorted[Math.ceil(sorted.length*.95)-1]??null,max_ms:sorted.at(-1)??null};};
      async function phase(durationMs){
        const started=performance.now(),frames=[],ticks=[];let finished=false;
        return new Promise((resolve,reject)=>{
          function scheduleDraw(){
            // Capture only the callback scheduled synchronously by this
            // controller.rotate call. Other game/requestAnimationFrame users
            // retain their original scheduling and are never counted as draws.
            const original=window.requestAnimationFrame;
            window.requestAnimationFrame=callback=>nativeRaf(timestamp=>{
              if(finished)return;
              const callbackStarted=performance.now(),before=metrics.snapshot();
              try {
                callback(timestamp);
                const finishStarted=performance.now();gl.finish();const completed=performance.now();
                const after=metrics.snapshot(),drawCalls=after.draw_calls-before.draw_calls;
                if(drawCalls>0)frames.push({started_ms:callbackStarted-started,completed_ms:completed-started,
                  callback_and_gpu_ms:completed-callbackStarted,gpu_finish_wait_ms:completed-finishStarted,
                  draw_calls:drawCalls,submitted_triangles:after.submitted_triangles-before.submitted_triangles});
              }catch(error){finished=true;reject(error);}
            });
            try {controller.rotate(.008,0);} finally {window.requestAnimationFrame=original;}
          }
          function tick(){
            if(finished)return;
            try {
              if(document.hidden||gl.isContextLost()||!row.canvas.isConnected||EQUIPMENT_VIEWER.controller!==controller)
                throw Error('Viewer changed, became hidden or lost its context during orbit timing');
              const now=performance.now();ticks.push({elapsed_ms:now-started,completed_draw_frames:frames.length});
              if(now-started>=durationMs){
                finished=true;
                const elapsed=now-started,frameIntervals=frames.slice(1).map((frame,index)=>frame.completed_ms-frames[index].completed_ms);
                const drawWork=summary(frames.map(frame=>frame.callback_and_gpu_ms));
                resolve({requested_ms:durationMs,elapsed_ms:elapsed,draw_frames:frames.length,animation_callbacks:ticks.length,
                  completed_draw_fps:frames.length*1000/elapsed,
                  between_completed_draws_fps:frames.length>1?(frames.length-1)*1000/(frames.at(-1).completed_ms-frames[0].completed_ms):null,
                  callback_and_gpu_completion:drawWork,completed_draw_intervals:summary(frameIntervals),
                  draw_work_p95_upper_bound_fps:drawWork.p95_ms>0?1000/drawWork.p95_ms:null,
                  raw_frames:frames,raw_animation_ticks:ticks});return;
              }
              scheduleDraw();nativeRaf(tick);
            }catch(error){finished=true;reject(error);}
          }
          scheduleDraw();nativeRaf(tick);
        });
      }
      const warmup=await phase(3000),sample=await phase(12000);
      const target=60;
      return {platform:EQUIP.draft.platform,context_id:row.id,
        viewport:{width:innerWidth,height:innerHeight,dpr:devicePixelRatio},
        canvas:{width:row.canvas.width,height:row.canvas.height},
        gpu:{vendor:gl.getParameter(gl.VENDOR),renderer:gl.getParameter(gl.RENDERER),
          unmasked_renderer:(()=>{const ext=gl.getExtension('WEBGL_debug_renderer_info');return ext?gl.getParameter(ext.UNMASKED_RENDERER_WEBGL):null;})()},
        target_fps:target,passed:sample.completed_draw_fps>=target,warmup,sample,
        method:'3 seconds of separate orbit warm-up, then at least 12 seconds of continuous native controller.rotate calls. Only controller animation callbacks submitting one or more draw calls count. Each finishes GPU work with gl.finish(). Full elapsed time, including scheduling gaps and the first frame, is the acceptance denominator; no rounding or vsync tolerance is applied. Between-frame cadence excludes startup and is diagnostic only. Callback plus GPU-completion p95 yields a draw-work upper bound, not observed FPS. Instrumented submission/completion is not proof of compositor-presented frames.'};
    });
    proof.viewer_orbit=record('s22-fighter-orbit-performance',measured);
  }
  try {
    const response=await page.request.get(new URL('/api/build',page.url()).href);assert(response.ok());proof.build=await response.json();
    assert.equal(proof.build.revision,(expectedRevision||evidence.runtime_revision).slice(0,12),'native build reports the exact clean 12-character revision');
    for(const name of files){const response=await page.request.get(new URL(name==='index.html'?'/':'/'+name,page.url()).href);
      assert(response.ok());const bytes=await response.body(),local=fs.readFileSync(path.join(root,'spheres-web/ui',name));
      assert(bytes.equals(local),'exact native served '+name);proof.sources.push({path:'spheres-web/ui/'+name,sha256:hash(bytes),bytes:bytes.length});}
    for(const name of ['s22-renderer-browser.cjs','webgl-measurement.js','webgl-texture-measurement.js']){const bytes=fs.readFileSync(path.join(__dirname,name));
      fs.writeFileSync(path.join(out,name),bytes);proof.sources.push({path:'tools/ui/'+name,sha256:hash(bytes),bytes:bytes.length});}
    await idle();await closeRooms();proof.before_state=record('s22-renderer-state-before',await state());
    await page.waitForFunction(()=>window.__s22Renderer?.contexts.length&&GL.ready&&GL.ok,null,{timeout:120000});
    await page.evaluate(()=>{
      const probe=__s22Renderer;if(probe.wrapped)throw Error('Renderer qualification cannot be installed twice');probe.wrapped=true;
      // Any cities already uploaded during startup are adopted by querying the
      // real buffers. Subsequent create/upload/delete calls are observed live.
      for(const entry of GLR.cityCache.values())for(const buffer of entry.bufs){
        GLR.gl.bindBuffer(GLR.gl.ARRAY_BUFFER,buffer);const bytes=GLR.gl.getBufferParameter(GLR.gl.ARRAY_BUFFER,GLR.gl.BUFFER_SIZE);
        probe.cities.set(buffer,{gl:GLR.gl,bytes});probe.counters.resident+=bytes;
      }
      probe.counters.peak=probe.counters.resident;
      const build=cityBuffers,draw=drawCityLayer;
      cityBuffers=function(...args){probe.buildingCity=true;try{return build.apply(this,args);}finally{probe.buildingCity=false;}};
      drawCityLayer=function(...args){probe.drawingCity=true;try{return draw.apply(this,args);}finally{probe.drawingCity=false;}};
    });
    await page.setViewportSize({width:1920,height:1080});await detail('relief',true);await detail('cities',true);
    for(const name of ['Tokyo','New York','Mexico City','Mumbai','Sao Paulo','Delhi']){await findCity(name);await observe('s22-city-'+name.replaceAll(' ','-').toLowerCase());}
    const toured=await observe('s22-city-tour-complete');assert(toured.city.deletions>0,'tour actually exercised native city eviction');
    assert(toured.city.uploaded>86400000,'tour exceeds one cache worth of uploads');
    const beforeReuse=toured.city.uploads;await tap('[data-map-action="east"]');await tap('[data-map-action="west"]');await settle();
    const reused=await observe('s22-city-revisit');assert.equal(reused.city.uploads,beforeReuse,'same close view reuses city attributes');
    await detail('cities',false);const disabled=await snapshot('cities disabled');
    await tap('[data-map-action="east"]');await tap('[data-map-action="west"]');await settle();
    const hidden=await observe('s22-cities-disabled');assert.equal(hidden.city.draws,disabled.city.draws);assert.equal(hidden.city.uploads,disabled.city.uploads);
    await detail('relief',false);await observe('s22-low-detail');await shot('s22-low-detail');
    await detail('relief',true);await detail('cities',true);await findCity('Delhi');
    await loseRestore('globe');await page.waitForFunction(()=>GLR.cityCache.size>0);await observe('s22-globe-restored');
    await loseRestore('arsenal');await observe('s22-city-card-restored');await shot('s22-city-card-restored');
    // Repeated actual city cards must not retain canvases removed by room/map redraw.
    for(let visit=0;visit<6;visit++){
      await findCity(visit%2?'Paris':'Delhi');await tap('[data-map-detail-focus="city-close"]');await settle();
      const row=await observe('s22-city-room-closed-'+visit);assert.equal(row.arsenal.mounted,0);assert.equal(row.arsenal.pending,0);
    }
    proof.checks.push('Actual main CityMesh transient cache ceiling, LRU reuse, Cities Off, low-detail view, context restoration and repeated city cards');

    // Native designer visits. The family controls change only the local draft;
    // no commissioning, procurement or research command is sent.
    for(let visit=0;visit<6;visit++){
      const visitStarted=performance.now();
      if(await page.locator('#intelDrawer').getAttribute('aria-hidden')!=='false')await tap('[data-drawer="intelDrawer"]');
      await tap('#warsCard [data-ground-equipment-tab="service"]');await page.waitForFunction(()=>equipmentCurrent());
      await tap('[role="tab"][data-equipment-tab="designer"]');await page.waitForFunction(()=>equipmentCurrent()&&EQUIP.tab==='designer'&&!equipmentPending());
      const family=await page.evaluate(()=>equipmentFamily(equipmentRows('platforms').find(row=>row.id==='air_fighter')));
      assert(family);await tap(`[data-equipment-family=${JSON.stringify(family)}]`);
      await page.waitForFunction(()=>!equipmentPending());
      if(await page.locator('#equipmentPlatform').inputValue()!=='air_fighter')await page.locator('#equipmentPlatform').selectOption('air_fighter');
      await page.locator('[data-model-canvas]').scrollIntoViewIfNeeded();
      await page.waitForFunction(()=>document.querySelector('[data-model-status]')?.textContent.includes('3D model ready'));
      await settle();const inspection=await observe('s22-aircraft-visit-'+visit);
      const mesh=await page.evaluate(()=>({triangles:EquipmentMesh.build(EQUIP.draft).triangleCount,platform:EQUIP.draft.platform,spec:structuredClone(EQUIP.draft)}));
      assert.equal(mesh.platform,'air_fighter');assert(mesh.triangles>=100000&&mesh.triangles<=250000);record('s22-aircraft-mesh-'+visit,mesh);
      const current=inspection.contexts.filter(row=>row.type==='webgl'&&row.connected);assert.equal(current.length,1,'one live native inspection canvas');
      assert(current[0].metrics.draw_calls>0,'Ready inspection must have submitted actual geometry');
      proof.inspection_loads??=[];proof.inspection_loads.push({visit,elapsed_ms:performance.now()-visitStarted,kind:visit===0?'first native inspection visit':'repeat visit',method:'Visible room and family navigation to ready released fighter with actual GPU draw submissions; includes Playwright interaction/observation overhead, not compositor presentation time.'});
      if(visit===0){await measureFighterOrbit();await loseRestore('inspection');await shot('s22-aircraft-restored');}
      if(visit===5){await page.setViewportSize({width:390,height:844});await settle();
        assert(await page.evaluate(()=>document.documentElement.scrollWidth<=innerWidth));await shot('s22-aircraft-390');await page.setViewportSize({width:1920,height:1080});}
      await tap('[data-equipment-close]');await settle();
      const closed=await observe('s22-aircraft-closed-'+visit);
      for(const row of closed.contexts.filter(row=>row.type==='webgl')){
        assert.equal(row.metrics.live_buffers,0,'closed viewers dispose every buffer');
        assert.equal(row.declared_texture_payload.live_textures,0,'closed viewers dispose every texture');
        assert.equal(row.declared_texture_payload.allocated_texture_levels,0);
        assert.equal(row.declared_texture_payload.declared_texture_texel_payload_bytes,0);
      }
    }
    proof.checks.push('Six native aircraft designer visits, one 100k+ model at a time, context recovery, 390px layout and complete buffer/texture disposal');

    // Explicit controlled lazy-card fixture on the real native page. This uses
    // the released deck and scanner; it is not evidence of a generated army or
    // of an otherwise-unreachable legacy manufacturing room.
    await page.evaluate(()=>{
      const host=document.createElement('div');host.id='s22LazyProbe';host.style.cssText='position:fixed;top:20px;left:20px;width:240px;height:160px;overflow:auto;z-index:99999;background:#18242b';
      host.innerHTML='<div style="height:12000px"></div>'+Arsenal3D.canvasHtml('arm_gen2','armour');
      const canvas=host.querySelector('canvas');canvas.style.cssText='display:block;width:200px;height:140px';document.body.append(host);Arsenal3D.scan(host);
    });
    await settle();const pending=await observe('s22-lazy-card-pending');assert.equal(pending.arsenal.pending,1);assert.equal(pending.arsenal.mounted,0);
    await page.locator('#s22LazyProbe canvas').scrollIntoViewIfNeeded();
    await page.waitForFunction(()=>Arsenal3D.cacheStats().pending===0&&Arsenal3D.cacheStats().mounted===1);
    await observe('s22-lazy-card-visible');await page.evaluate(()=>document.getElementById('s22LazyProbe').remove());await settle();
    const cleaned=await observe('s22-lazy-card-removed');assert.equal(cleaned.arsenal.pending,0);assert.equal(cleaned.arsenal.mounted,0);
    proof.checks.push('Controlled native-page lazy-card fixture defers below-fold geometry, mounts on intersection and releases detached references');
    proof.after_state=record('s22-renderer-state-after',await state());assert.deepEqual(proof.after_state,proof.before_state,'renderer checks are simulation-read-only');
    assert.deepEqual(proof.requests.filter(row=>row.path!=='/api/equipment-preview'),[],'only read-only local draft previews may POST');
    assert.deepEqual(proof.errors,[]);const final=await observe('s22-renderer-final');
    for(const row of final.contexts)assert.deepEqual(row.diagnostics,[],'shader compilation diagnostics');
    assert(proof.viewer_orbit.passed,'Released fighter orbit completed-draw FPS '+proof.viewer_orbit.sample.completed_draw_fps+' is below the unchanged 60 FPS target');
    proof.memory_complete=true;proof.passed=true;return proof;
  } catch(error) {proof.failure=String(error.stack||error);throw error;}
  finally {proof.finished_utc=new Date().toISOString();page.off('pageerror',onError);page.off('request',onRequest);
    fs.writeFileSync(path.join(out,'s22-renderer-result.json'),JSON.stringify(proof,null,2)+'\n');}
}
module.exports=run;module.exports.install=install;
