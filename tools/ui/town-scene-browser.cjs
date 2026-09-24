// Real WebGL2 submissions for the gallery's adaptive town path. No campaign,
// saves, downloads or native server. SPHERES_ART_OUTPUT holds compact evidence.
'use strict';
const {chromium}=require('playwright'),assert=require('node:assert/strict');
const fs=require('node:fs'),path=require('node:path'),http=require('node:http'),crypto=require('node:crypto'),cp=require('node:child_process');
const root=path.resolve(__dirname,'../..');
const files=['spheres-web/ui/town-mesh.js','spheres-web/ui/arsenal3d.js','tools/arsenal/art-gallery.html',
  'tools/ui/town-scene-browser.cjs','tools/ui/webgl-measurement.js'];
const html=`<!doctype html><meta charset="utf-8"><style>body{background:#16222c;color:white}canvas{display:block;width:640px;height:400px;touch-action:none}</style><canvas id="scene"></canvas>
<script src="/spheres-web/ui/arsenal-models.js"></script><script src="/spheres-web/ui/arsenal3d.js"></script><script src="/spheres-web/ui/town-mesh.js"></script>`;
(async()=>{
  const parent=path.resolve(process.env.SPHERES_ART_OUTPUT||path.join(root,'../evidence/town-scene'));
  fs.mkdirSync(parent,{recursive:true});const out=fs.mkdtempSync(path.join(parent,'scene-'));
  const result={passed:false,revision:cp.execFileSync('git',['rev-parse','HEAD'],{cwd:root,encoding:'utf8'}).trim(),
    source_dirty:!!cp.execFileSync('git',['status','--porcelain'],{cwd:root,encoding:'utf8'}).trim(),
    source_sha256:Object.fromEntries(files.map(f=>[f,crypto.createHash('sha256').update(fs.readFileSync(path.join(root,f))).digest('hex')])),
    canonical_lf_source_sha256:Object.fromEntries(files.map(f=>[f,crypto.createHash('sha256').update(fs.readFileSync(path.join(root,f),'utf8').replace(/\r\n/g,'\n')).digest('hex')])),
    method:'Actual WebGL2 drawArrays submissions and queried buffer payload sizes, not a triangle estimate, FPS or driver VRAM claim.',cases:[],errors:[]};
  const server=http.createServer((req,res)=>{
    const url=new URL(req.url,'http://localhost');
    if(url.pathname==='/'){res.setHeader('Content-Type','text/html');res.end(html);return;}
    const file=path.resolve(root,'.'+decodeURIComponent(url.pathname));
    if(!file.startsWith(root+path.sep)){res.writeHead(403).end();return;}
    fs.readFile(file,(e,b)=>{if(e){res.writeHead(404).end();return;}res.setHeader('Content-Type',file.endsWith('.html')?'text/html':'text/javascript');res.end(b);});
  });
  await new Promise(r=>server.listen(0,'127.0.0.1',r));let browser;
  try{
    browser=await chromium.launch({headless:true,channel:process.env.SPHERES_BROWSER_CHANNEL||'chrome'});result.browser=browser.version();
    for(const dpr of [1,2]){
      const context=await browser.newContext({viewport:{width:1280,height:900},deviceScaleFactor:dpr,reducedMotion:'reduce'});
      await context.addInitScript({content:fs.readFileSync(path.join(__dirname,'webgl-measurement.js'),'utf8')+`
        window.probes=[];const original=HTMLCanvasElement.prototype.getContext;
        HTMLCanvasElement.prototype.getContext=function(...args){const gl=original.apply(this,args);
        if(gl&&args[0]==='webgl2'&&!probes.some(p=>p.gl===gl))probes.push({gl,metrics:WebGLMeasurement.attach(gl)});return gl;};`});
      const page=await context.newPage();page.on('pageerror',e=>result.errors.push(e.message));
      await page.goto(`http://127.0.0.1:${server.address().port}/`);
      for(const [id,district] of [[1997,'mixed'],[1994,'residential'],['1990','residential']]){
        await page.evaluate(options=>{window.scene=TownMesh.scene(options);window.canvas=document.getElementById('scene');}, {id,district});
        for(const [width,height] of [[320,280],[640,400],[1280,800],[1100,1100]])for(const yaw of [0,34,90,145,270])for(const zoom of [.5,1,3]){
          const row=await page.evaluate(o=>{
            canvas.style.width=o.width+'px';canvas.style.height=o.height+'px';
            const ready=Arsenal3D.available,probe=probes[0],before=probe.metrics.snapshot();
            const plan=Arsenal3D.drawScene(canvas,scene,o),after=probe.metrics.snapshot();
            return {ready,options:o,triangles:plan.triangles,raw:plan.rawCloseTriangles,visible:plan.draws.length,culled:plan.culled.length,
              tiers:plan.draws.map(d=>d.tier),drawCalls:plan.drawCalls,demotions:plan.demotions,
              actual:WebGLMeasurement.interval(before,after),cache:Arsenal3D.cacheStats(),cpu:TownMesh.cacheStats(),glError:probe.gl.getError()};
          },{width,height,yaw,zoom});
          assert(row.ready);assert.equal(row.actual.submitted_triangles,row.triangles);assert.equal(row.actual.draw_calls,row.drawCalls);
          assert(row.triangles<=150000);assert.equal(row.glError,0);
          assert.equal(row.actual.resident_buffer_payload_bytes,row.cache.triangles*108);
          assert(row.cache.triangles<=row.cache.cap&&row.cpu.triangles<=row.cpu.cap);
          result.cases.push({dpr,id,district,...row});
        }
        const focus=await page.evaluate(()=>{
          canvas.style.width='640px';canvas.style.height='400px';
          window.controller=Arsenal3D.mountScene(canvas,scene);
          const rows=[];
          for(const lot of scene.lots){
            const probe=probes[0],before=probe.metrics.snapshot(),plan=controller.focus(lot.id);
            rows.push({id:lot.id,tier:plan.draws.find(d=>d.id===lot.id)?.tier,triangles:plan.triangles,
              actual:WebGLMeasurement.interval(before,probe.metrics.snapshot()).submitted_triangles});
          }
          controller.focus('lot-0');return rows;
        });
        for(const row of focus){assert.equal(row.tier,'close');assert.equal(row.actual,row.triangles);assert(row.triangles<=150000);}
        result.cases.push({dpr,id,district,focus});
        if(dpr===1&&typeof id==='number'){
          await page.locator('#scene').screenshot({path:path.join(out,`${district}-focus.png`)});
          await page.evaluate(()=>controller.reset());
          await page.locator('#scene').screenshot({path:path.join(out,`${district}-adaptive.png`)});
          const rawFrame=await page.evaluate(()=>{
            const plan=controller.plan,id=typeof scene.blockId+'-'+scene.blockId+'-'+scene.district;
            Arsenal3D.register('raw',()=>TownMesh.block({id:scene.blockId,district:scene.district}));
            const before=probes[0].metrics.snapshot();
            const raw=Arsenal3D.renderTo('raw:'+id,'',canvas.width,canvas.height,plan.yaw,plan.pitch,plan.camera.distance,plan.camera.pivot);
            const ctx=canvas.getContext('2d');ctx.clearRect(0,0,canvas.width,canvas.height);
            ctx.drawImage(probes[0].gl.canvas,0,raw.top,canvas.width,canvas.height,0,0,canvas.width,canvas.height);
            return {expected:scene.rawCloseTriangles,actual:WebGLMeasurement.interval(before,probes[0].metrics.snapshot()).submitted_triangles};
          });
          assert.equal(rawFrame.actual,rawFrame.expected,'raw comparison must use this exact block, not a cached previous seed');
          result.cases.push({dpr,id,district,rawComparison:rawFrame});
          await page.locator('#scene').screenshot({path:path.join(out,`${district}-raw-close.png`)});
        }
        await page.evaluate(()=>controller.dispose());
      }
      const worst=await page.evaluate(dpr=>{
        canvas.style.width=(2048/dpr)+'px';canvas.style.height=(1440/dpr)+'px';
        const before=probes[0].metrics.snapshot();
        const plan=Arsenal3D.drawScene(canvas,scene,{selected:'lot-15',yaw:0,zoom:.55});
        return {triangles:plan.triangles,width:plan.width,height:plan.height,selected:plan.selected,
          actual:WebGLMeasurement.interval(before,probes[0].metrics.snapshot())};
      },dpr);
      assert.equal(worst.width,2048);assert.equal(worst.height,1440);assert.equal(worst.selected,'lot-15');
      assert.equal(worst.actual.submitted_triangles,worst.triangles);assert(worst.triangles<=150000);
      result.cases.push({dpr,stringSeedWorst:worst});
      await page.evaluate(()=>{canvas.style.width='640px';canvas.style.height='400px';});
      const interaction=await page.evaluate(()=>{window.controller=Arsenal3D.mountScene(canvas,scene);return controller.plan.yaw;});
      await page.locator('#scene').focus();await page.keyboard.press('ArrowRight');
      assert.equal(await page.evaluate(()=>controller.plan.yaw),interaction+15);
      await page.keyboard.press('+');assert((await page.evaluate(()=>controller.plan.zoom))>1);
      await page.keyboard.press('Escape');assert.equal(await page.evaluate(()=>controller.plan.zoom),1);
      await page.locator('#scene').hover();await page.mouse.wheel(0,-100);
      await page.waitForFunction(()=>controller.plan.zoom>1);
      const pointerStart=await page.evaluate(()=>controller.plan.yaw);
      const box=await page.locator('#scene').boundingBox();
      await page.mouse.move(box.x+100,box.y+100);await page.mouse.down();await page.mouse.move(box.x+160,box.y+110);await page.mouse.up();
      assert((await page.evaluate(()=>controller.plan.yaw))>pointerStart);
      await page.evaluate(()=>controller.focus('lot-0'));
      await page.evaluate(()=>{window.loss=probes[0].gl.getExtension('WEBGL_lose_context');loss.loseContext();});
      await page.waitForFunction(()=>probes[0].metrics.snapshot().context_losses===1);
      assert.equal(await page.evaluate(()=>probes[0].metrics.snapshot().live_buffers),0);
      await page.evaluate(()=>loss.restoreContext());
      await page.waitForFunction(()=>probes[0].metrics.snapshot().live_buffers>0);
      const restored=await page.evaluate(()=>({selected:controller.plan.selected,tier:controller.plan.draws.find(d=>d.id==='lot-0').tier,
        cache:Arsenal3D.cacheStats(),metrics:probes[0].metrics.snapshot(),diagnostics:probes[0].metrics.diagnostics()}));
      assert.equal(restored.selected,'lot-0');assert.equal(restored.tier,'close');assert.equal(restored.metrics.resident_buffer_payload_bytes,restored.cache.triangles*108);
      assert.deepEqual(restored.diagnostics,[]);result.cases.push({dpr,contextRestore:restored});
      const stress=await page.evaluate(()=>{
        controller.dispose();let max=0;
        for(let id=10;id<50;id++){
          const model=TownMesh.scene({id,district:TownMesh.districts()[id%TownMesh.districts().length]});
          Arsenal3D.drawScene(canvas,model,{width:640,height:400,selected:'lot-0'});
          max=Math.max(max,Arsenal3D.cacheStats().triangles);
        }
        const first=probes[0].metrics.snapshot();
        Arsenal3D.drawScene(canvas,scene,{selected:'lot-0'});
        const before=probes[0].metrics.snapshot();Arsenal3D.drawScene(canvas,scene,{selected:'lot-0'});
        const same=WebGLMeasurement.interval(before,probes[0].metrics.snapshot());
        const disposed=Arsenal3D.mountScene(canvas,scene);disposed.dispose();
        const start=probes[0].metrics.snapshot();canvas.dispatchEvent(new KeyboardEvent('keydown',{key:'ArrowRight'}));
        const afterDispose=WebGLMeasurement.interval(start,probes[0].metrics.snapshot());
        return {max,cache:Arsenal3D.cacheStats(),cpu:TownMesh.cacheStats(),metrics:probes[0].metrics.snapshot(),same,afterDispose,
          evicted:first.buffer_data_payload_bytes>first.resident_buffer_payload_bytes};
      });
      assert(stress.max<=stress.cache.cap);assert(stress.cpu.triangles<=stress.cpu.cap);
      assert(stress.evicted);assert.equal(stress.same.buffer_data_calls,0,'same view reuses resident variant buffers');
      assert.equal(stress.afterDispose.draw_calls,0,'disposed controller removes input listeners');
      assert(stress.metrics.peak_buffer_payload_bytes<=stress.cache.cap*108,'GPU payload cap holds during uploads too');
      assert.equal(stress.metrics.resident_buffer_payload_bytes,stress.cache.triangles*108);result.cases.push({dpr,stress});
      await page.goto(`http://127.0.0.1:${server.address().port}/tools/arsenal/art-gallery.html`);
      await page.getByRole('button',{name:'Town budget cases',exact:true}).click();
      await page.waitForFunction(()=>[...document.querySelectorAll('[data-town-measure]')].every(e=>e.textContent.includes('submitted triangles')));
      await page.locator('[data-town-lot]').first().selectOption('lot-0');
      assert.match(await page.locator('[data-town-measure]').first().textContent(),/1 close|[2-9]\d* close/);
      await page.locator('[data-town-reset]').first().click();assert.equal(await page.locator('[data-town-lot]').first().inputValue(),'');
      result.cases.push({dpr,galleryCaptions:await page.locator('[data-town-measure]').allTextContents()});
      await context.close();
    }
    const fallback=await browser.newPage();
    await fallback.addInitScript(()=>{const original=HTMLCanvasElement.prototype.getContext;
      HTMLCanvasElement.prototype.getContext=function(type,...args){return type==='webgl2'?null:original.call(this,type,...args);};});
    await fallback.goto(`http://127.0.0.1:${server.address().port}/tools/arsenal/art-gallery.html`);
    assert.match(await fallback.locator('.warn').textContent(),/No WebGL2/);
    result.fallback=true;await fallback.close();
    assert.deepEqual(result.errors,[]);result.passed=true;
  }catch(error){result.failure=String(error.stack||error);throw error;}
  finally{fs.writeFileSync(path.join(out,'result.json'),JSON.stringify(result,null,2)+'\n');if(browser)await browser.close();await new Promise(r=>server.close(r));console.log(path.join(out,'result.json'));}
})().catch(error=>{console.error(error);process.exitCode=1;});
