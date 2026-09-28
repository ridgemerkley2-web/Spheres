// Native S22 browser measurement. Every run owns a new server/save directory.
// Input and runtime identities are required; failed attempts are retained.
'use strict';
const assert=require('node:assert/strict'),fs=require('node:fs'),path=require('node:path');
const cp=require('node:child_process'),crypto=require('node:crypto'),net=require('node:net'),zlib=require('node:zlib');
const {chromium}=require('playwright');
const root=path.resolve(__dirname,'../..');
const hash=b=>crypto.createHash('sha256').update(b).digest('hex');
const git=args=>cp.execFileSync('git',args,{cwd:root,encoding:'utf8',windowsHide:true}).trim();
const summary=values=>{const a=[...values].sort((a,b)=>a-b);return {count:a.length,min:a[0],median:a[Math.ceil(a.length/2)-1],p95:a[Math.ceil(a.length*.95)-1],max:a.at(-1)};};

async function install(page){
  await page.addInitScript({path:path.join(__dirname,'webgl-measurement.js')});
  await page.addInitScript(()=>{
    window.s22Probes=[];
    const original=HTMLCanvasElement.prototype.getContext;
    HTMLCanvasElement.prototype.getContext=function(...args){
      const gl=original.apply(this,args);
      if(gl&&['webgl','webgl2','experimental-webgl'].includes(args[0])&&!s22Probes.some(p=>p.gl===gl))
        s22Probes.push({gl,metrics:WebGLMeasurement.attach(gl)});
      return gl;
    };
    window.s22Inputs=[];window.s22CaptureInputs=false;
    document.addEventListener('click',event=>{
      if(!s22CaptureInputs||!event.isTrusted||!event.target.closest('#mapControls button'))return;
      const button=event.target.closest('button'),row={key:button.dataset.mapFocus,start:performance.now(),event_timestamp:event.timeStamp};
      requestAnimationFrame(()=>requestAnimationFrame(()=>{
        if(typeof GL!=='undefined'&&GL.gl&&!GL.gl.isContextLost())GL.gl.finish();
        row.complete=performance.now();row.elapsed_ms=row.complete-row.event_timestamp;s22Inputs.push(row);
      }));
    },true);
  });
}

async function run(){
  const expected=process.env.SPHERES_EXPECTED_REVISION,checkpoint=path.resolve(process.env.SPHERES_S22_CHECKPOINT||'');
  const expectedHash=process.env.SPHERES_S22_CHECKPOINT_SHA256;
  assert.match(expected||'',/^[a-f0-9]{40}$/);assert.match(expectedHash||'',/^[a-f0-9]{64}$/);
  assert.equal(git(['status','--porcelain','--','spheres-web','spheres-sim']).trim(),'','Runtime must be committed');
  assert.equal(git(['diff','--name-only',expected,'HEAD','--','spheres-web','spheres-sim','Cargo.toml','Cargo.lock']),'');
  const binary=path.resolve(process.env.SPHERES_BINARY||path.join(root,'target/release/spheres-web.exe'));
  const bytes=fs.readFileSync(checkpoint),raw=bytes[0]===31&&bytes[1]===139?zlib.gunzipSync(bytes):bytes;
  assert.equal(hash(raw),expectedHash);
  const parent=path.resolve(process.env.SPHERES_S22_OUTPUT||path.join(root,'../evidence/s22-browser'));
  fs.mkdirSync(parent,{recursive:true});const out=fs.mkdtempSync(path.join(parent,'browser-')),serverRoot=path.join(out,'server');
  fs.mkdirSync(path.join(serverRoot,'saves'),{recursive:true});fs.writeFileSync(path.join(serverRoot,'saves/s22-input.json'),raw);
  const socket=net.createServer();await new Promise(r=>socket.listen(0,'127.0.0.1',r));const port=socket.address().port;await new Promise(r=>socket.close(r));
  const url=`http://127.0.0.1:${port}`;
  const evidence={passed:false,qualification:process.env.SPHERES_S22_QUALIFY==='1',started_utc:new Date().toISOString(),out,run:serverRoot,url,
    revision:expected,head:git(['rev-parse','HEAD']),binary_sha256:hash(fs.readFileSync(binary)),checkpoint_sha256:hash(raw),
    driver_sha256:hash(fs.readFileSync(__filename)),actions:[],requests:[],errors:[],screenshots:[],cells:[],checks:[],assets:[],
    method:{map:'A continuous changing camera drives the released Globe3D render path. Each counted frame must issue actual WebGL draws; gl.finish completes GPU work before its timestamp. 3s settling is recorded separately, then a full 12s window includes blocking and final frame cost. This is completed render throughput under synthetic continuous navigation, not idle RAF cadence or display presentation FPS. Instrumentation and forced GPU synchronization add cost.',
      input:'31 trusted visible map-control clicks per detail preset; event timestamp to a second animation-frame opportunity plus GPU completion. Includes input dispatch and main-thread work. This is a conservative paint-opportunity proxy, not compositor presentation latency or network command latency.',
      memory:'CDP JavaScript heap/DOM and measured WebGL buffer payload, separately labeled. Buffer payload excludes textures, renderbuffers, framebuffers, driver overhead and total VRAM. No headless native 1GiB ceiling is applied to browser counters.'}};
  fs.copyFileSync(__filename,path.join(out,'driver.cjs'));console.log(out);
  const serverStart=performance.now(),server=cp.spawn(binary,['--port',String(port),'--no-open'],{cwd:serverRoot,windowsHide:true,stdio:['ignore','pipe','pipe']});
  const serverLog=fs.createWriteStream(path.join(out,'server.log'));server.stdout.pipe(serverLog);server.stderr.pipe(serverLog);
  let browser,page,launchError;server.on('error',e=>{launchError=e;});
  try{
    let ready=false;
    for(let i=0;i<600;i++){
      if(launchError)throw launchError;if(server.exitCode!==null)throw Error('Owned server exited');
      try{if((await fetch(url+'/api/build')).ok){ready=true;break;}}catch{}
      await new Promise(r=>setTimeout(r,100));
    }
    assert(ready);evidence.server_start_ready_ms=performance.now()-serverStart;
    browser=await chromium.launch({headless:true,channel:process.env.SPHERES_BROWSER_CHANNEL||'msedge'});evidence.browser=browser.version();
    page=await browser.newPage({viewport:{width:1920,height:1080},deviceScaleFactor:1,reducedMotion:'reduce',hasTouch:true});
    if(process.env.SPHERES_S22_RENDERERS==='1')await require('./s22-renderer-browser.cjs').install(page);else await install(page);
    page.on('pageerror',e=>evidence.errors.push(e.message));
    page.on('request',r=>{if(r.method()==='POST')evidence.requests.push({path:new URL(r.url()).pathname,body:r.postDataJSON()});});
    const tap=async selector=>{evidence.actions.push({click:selector});await page.locator(selector).click();};
    const state=async()=>{const r=await page.request.get(url+'/api/state');assert(r.ok());return r.json();};
    const shot=async name=>{const file=name+'.png';await page.screenshot({path:path.join(out,file)});evidence.screenshots.push(file);};
    const cold=performance.now();await page.goto(url);await page.locator('#openSavesBtn').waitFor();evidence.cold_menu_ms=performance.now()-cold;
    evidence.navigation=await page.evaluate(()=>performance.getEntriesByType('navigation').map(e=>e.toJSON()));
    evidence.build=await(await page.request.get(url+'/api/build')).json();assert.equal(evidence.build.revision,expected);
    for(const name of ['index.html','map-controls.js','globe3d.js','arsenal3d.js','equipment-model.js']){
      const r=await page.request.get(url+(name==='index.html'?'/':'/'+name)),served=await r.body();
      assert(served.equals(fs.readFileSync(path.join(root,'spheres-web/ui',name))),'Served current '+name);
      evidence.assets.push({name,sha256:hash(served)});
    }
    await tap('#openSavesBtn');await page.locator('#saveSlots').selectOption('s22-input');
    const load=performance.now();await tap('#loadBtn');await page.waitForFunction(()=>S?.player&&!SESSION.busy&&GL.ok&&GL.ready&&GLOBE?.lastSize);
    evidence.cold_campaign_to_usable_map_ms=performance.now()-load;
    const before=await state();evidence.campaign={date:before.date,player:before.player,session_id:before.session_id};
    async function save(slot){
      if(await page.locator('.arc-time-menu').getAttribute('open')===null)await tap('.arc-time-menu > summary');
      await tap('#campaignsBtn');await tap('#openSavesBtn');await page.locator('#saveName').fill(slot);
      const response=page.waitForResponse(r=>new URL(r.url()).pathname==='/api/save'&&r.request().method()==='POST');
      await tap('#saveNamedBtn');assert((await response).ok());
      await page.locator(`#saveSlots option[value="${slot}"]`).waitFor({state:'attached'});
      const b=fs.readFileSync(path.join(serverRoot,'saves',slot+'.json')),v=JSON.parse(b);delete v.saved_unix;
      const compressed=zlib.gzipSync(b);fs.writeFileSync(path.join(out,slot+'.json.gz'),compressed);
      await tap('#savedCampaigns [data-menu-back]');await tap('#continueBtn');await page.waitForFunction(()=>!SESSION.busy&&S?.player);
      return hash(JSON.stringify(v));
    }
    evidence.before_save=await save('s22-before');
    if(process.env.SPHERES_S22_RENDERERS==='1'){
      await require('./s22-renderer-browser.cjs')({page,tap,state,shot,evidence,expectedRevision:expected});
    }else{
      const cdp=await page.context().newCDPSession(page);await cdp.send('Performance.enable');
      async function memory(){return {metrics:await cdp.send('Performance.getMetrics'),dom:await cdp.send('Memory.getDOMCounters'),
        contexts:await page.evaluate(()=>s22Probes.map(p=>({connected:p.gl.canvas.isConnected,id:p.gl.canvas.id,buffer_payload:p.metrics.snapshot()})))};}
      evidence.gpu=await page.evaluate(()=>{const gl=GL.gl,e=gl.getExtension('WEBGL_debug_renderer_info');return {vendor:gl.getParameter(gl.VENDOR),renderer:gl.getParameter(gl.RENDERER),unmasked:e?gl.getParameter(e.UNMASKED_RENDERER_WEBGL):null};});
      evidence.memory_before=await memory();
      const views=[{name:'world',zoom:1.25,amplitude:.3},{name:'national',zoom:8,amplitude:.07},{name:'regional',zoom:40,amplitude:.015},{name:'paris-city',zoom:181,amplitude:.003}];
      for(const detail of ['standard','low']){
        await page.locator('[data-map-mode="terrain"]').click();
        if(await page.locator('.map-detail-menu').getAttribute('open')===null)await tap('.map-detail-menu > summary');
        await tap(`[data-map-preset="${detail}"]`);await page.keyboard.press('Escape');
        const cell={detail,viewport:page.viewportSize(),dpr:await page.evaluate(()=>devicePixelRatio),views:[]};evidence.cells.push(cell);
        for(const view of views){
          const metrics=await page.evaluate(async v=>{
            const gl=GL.gl,probe=s22Probes.find(p=>p.gl===gl);if(!probe)throw Error('Missing actual map context');
            const yaw=-2.3522*Math.PI/180,pitch=48.8566*Math.PI/180;
            const measure=duration=>new Promise(resolve=>{
              const frames=[],start=performance.now();let previous=start;
              const frame=()=>{
                const a=performance.now(),t=(a-start)/1000;
                GLOBE.setView(yaw+Math.sin(t)*v.amplitude,pitch+Math.sin(t*.7)*v.amplitude*.35,v.zoom*(1+.04*Math.sin(t*.8)));
                const before=probe.metrics.snapshot();GLOBE.render();gl.finish();const end=performance.now(),after=probe.metrics.snapshot();
                frames.push({start_ms:a-start,complete_ms:end-start,interval_ms:end-previous,draw_ms:end-a,draw_calls:after.draw_calls-before.draw_calls,triangles:after.submitted_triangles-before.submitted_triangles});previous=end;
                if(end-start<duration)requestAnimationFrame(frame);else resolve({elapsed_ms:end-start,frames});
              };requestAnimationFrame(frame);
            });
            GLOBE.setView(yaw,pitch,v.zoom);GLOBE.render();gl.finish();
            const settle=await measure(3000),measured=await measure(12000);
            return {view:v,settle,measured,map_details:{...ui.mapDetails},buffers:probe.metrics.snapshot(),ready:GL.ok&&GL.ready,
              active_canvas:{width:GLCV.width,height:GLCV.height,css_width:GLCV.clientWidth,css_height:GLCV.clientHeight}};
          },view);
          metrics.fps=metrics.measured.frames.length*1000/metrics.measured.elapsed_ms;
          metrics.draw_summary=summary(metrics.measured.frames.map(f=>f.draw_ms));metrics.frame_interval_summary=summary(metrics.measured.frames.map(f=>f.interval_ms));
          metrics.passed=metrics.ready&&metrics.measured.frames.every(f=>f.draw_calls>0)&&metrics.fps>=30;
          cell.views.push(metrics);console.log(`${detail} ${view.name}: ${metrics.fps.toFixed(2)} completed FPS`);await shot(`${detail}-${view.name}`);
        }
        // Local controls return to a nation-scale camera; no world command is sent.
        await page.evaluate(()=>{GLOBE.lookAt(2.3522,48.8566,8);GLOBE.render();s22Inputs=[];s22CaptureInputs=true;});
        const actions=['west','east','zoom-in','zoom-out'];
        for(let i=0;i<31;i++){
          await tap(`[data-map-action="${actions[i%actions.length]}"]`);await page.waitForFunction(n=>s22Inputs.length===n,i+1);
        }
        cell.inputs=await page.evaluate(()=>{s22CaptureInputs=false;return s22Inputs;});cell.input_summary=summary(cell.inputs.map(i=>i.elapsed_ms));
        cell.input_passed=cell.input_summary.p95<=200;cell.memory=await memory();
      }
      evidence.layout=[];
      for(const viewport of [{width:390,height:844},{width:3440,height:1440}]){
        await page.setViewportSize(viewport);await tap('[data-map-action="home"]');
        if(await page.locator('.map-detail-menu').getAttribute('open')===null)await tap('.map-detail-menu > summary');
        await tap('[data-map-preset="low"]');await page.keyboard.press('Escape');
        const layout=await page.evaluate(()=>({width:innerWidth,scroll_width:document.documentElement.scrollWidth,controls:[...document.querySelectorAll('#mapControls [data-map-action]')].map(e=>{const r=e.getBoundingClientRect();return {action:e.dataset.mapAction,left:r.left,right:r.right,top:r.top,bottom:r.bottom,width:r.width,height:r.height};})}));
        layout.passed=layout.scroll_width<=viewport.width+1&&layout.controls.every(c=>c.left>=0&&c.right<=viewport.width+1&&c.width>0&&c.top>=0&&c.bottom<=viewport.height);
        evidence.layout.push(layout);await shot(`map-${viewport.width}`);
      }
      await page.setViewportSize({width:1920,height:1080});evidence.memory_after=await memory();
    }
    for(const [panel,close] of [['#equipmentRoom','[data-equipment-close]'],['#guidanceDialog','#guidanceDialog [data-guidance-close]'],['#productionPanel','#productionClose'],['#cabinetDrawer','#cabinetDrawer [data-close-drawers]']])
      if(await page.locator(panel).isVisible())await tap(close);
    evidence.after_save=await save('s22-after');assert.equal(evidence.after_save,evidence.before_save,'Complete saved campaign unchanged except wall-clock envelope timestamp');
    assert.equal((await state()).date,before.date);assert.deepEqual(evidence.errors,[]);
    assert(!evidence.requests.some(r=>['/api/advance','/api/command'].includes(r.path)),'Presentation measurement cannot settle or command the campaign');
    assert.equal(hash(fs.readFileSync(checkpoint)),hash(bytes));assert.equal(hash(fs.readFileSync(binary)),evidence.binary_sha256);
    evidence.passed=evidence.cells.every(c=>c.input_passed&&c.views.every(v=>v.passed))&&(evidence.layout||[]).every(r=>r.passed);
    if(evidence.qualification)assert(evidence.passed,'Frozen browser performance/layout limits failed; raw evidence retained');
  }catch(error){evidence.failure=error.stack;if(page)try{await page.screenshot({path:path.join(out,'failure.png')});fs.writeFileSync(path.join(out,'failure.txt'),await page.locator('body').innerText());}catch{}throw error;}
  finally{evidence.finished_utc=new Date().toISOString();fs.writeFileSync(path.join(out,'result.json'),JSON.stringify(evidence,null,2)+'\n');if(browser)await browser.close();if(server.exitCode===null)server.kill();serverLog.end();console.log(path.join(out,'result.json'));}
}
if(require.main===module)run().catch(e=>{console.error(e);process.exitCode=1;});
module.exports={install,summary};
