// Isolated S22 preparation bench: current renderer and meshes, no campaign data.
// SPHERES_ART_OUTPUT selects the parent for a fresh, retained evidence folder.
const {chromium}=require('playwright'),assert=require('node:assert/strict');
const fs=require('node:fs'),path=require('node:path'),http=require('node:http'),crypto=require('node:crypto'),cp=require('node:child_process'),os=require('node:os');
const root=path.resolve(__dirname,'../..'),ui=path.join(root,'spheres-web/ui');
const modules=['equipment-mesh.js','tank-surface.js','military-surface.js','equipment-export.js','equipment-model.js'];
const scripts=['webgl-measurement.js','art-memory-browser.cjs','mesh-accounting.cjs'];
const sha=bytes=>crypto.createHash('sha256').update(bytes).digest('hex');
const {build}=require(path.join(ui,'equipment-mesh.js'));
const platforms=['tank_standard','tank_heavy','tank_light','tank_destroyer','ground_ifv','ground_apc','ground_recon','ground_artillery','ground_air_defense','air_fighter','air_light_attack','air_tactical_strike'];
const html=`<!doctype html><html lang="en"><meta charset="utf-8"><title>Model memory measurement</title>
<style>body{margin:0;background:#101920;color:white;font:16px system-ui}#host{width:100%;max-width:960px} [data-model-canvas]{height:500px}</style>
<div id="host"><div data-model-canvas></div><p data-model-status></p><select data-model-part></select></div>
<script src="/probe.js"></script><script>
window.probes=[];const originalGetContext=HTMLCanvasElement.prototype.getContext;
HTMLCanvasElement.prototype.getContext=function(...args){const gl=originalGetContext.apply(this,args);if(gl&&args[0]==='webgl'&&!probes.some(p=>p.gl===gl))probes.push({gl,metrics:WebGLMeasurement.attach(gl)});return gl;};
window.settle=()=>new Promise(r=>requestAnimationFrame(()=>requestAnimationFrame(()=>requestAnimationFrame(r))));
</script>${modules.map(name=>`<script src="/${name}"></script>`).join('')}</html>`;
(async()=>{
  const parent=path.resolve(process.env.SPHERES_ART_OUTPUT||path.join(root,'../evidence/S22-preparation'));
  fs.mkdirSync(parent,{recursive:true});const out=fs.mkdtempSync(path.join(parent,'buffers-'));
  const git=args=>cp.execFileSync('git',args,{cwd:root,encoding:'utf8'}).trim();
  const result={passed:false,source_revision:git(['rev-parse','HEAD']),source_dirty:!!git(['status','--porcelain']),
    measured_at:new Date().toISOString(),platform:process.platform,os:os.release(),node:process.version,
    method:'Isolated actual EquipmentModel WebGL buffers queried via BUFFER_SIZE. Controlled redraw intervals count API submissions, including repeat passes. No FPS or driver VRAM claim. Textures, renderbuffers, default framebuffer, CPU heap and driver overhead excluded from buffer totals.',
    source_sha256:Object.fromEntries([...modules.map(n=>path.join('spheres-web/ui',n)),...scripts.map(n=>path.join('tools/ui',n))].map(file=>[file.replaceAll('\\','/'),sha(fs.readFileSync(path.join(root,file)))])),cases:[],errors:[]};
  const server=http.createServer((req,res)=>{
    const pathname=new URL(req.url,'http://localhost').pathname;
    if(pathname==='/'){res.setHeader('Content-Type','text/html');res.end(html);return;}
    const file=pathname==='/probe.js'?path.join(__dirname,'webgl-measurement.js'):path.resolve(ui,'.'+decodeURIComponent(pathname));
    if(pathname!=='/probe.js'&&!file.startsWith(ui+path.sep)){res.writeHead(403).end();return;}
    fs.readFile(file,(error,data)=>{if(error){res.writeHead(404).end();return;}res.setHeader('Content-Type',path.extname(file)==='.js'?'text/javascript':path.extname(file)==='.jpg'?'image/jpeg':'application/octet-stream');res.end(data);});
  });
  await new Promise(r=>server.listen(0,'127.0.0.1',r));let browser;
  try{
    browser=await chromium.launch({headless:true,channel:process.env.SPHERES_BROWSER_CHANNEL||'chrome'});result.browser=browser.version();
    const page=await browser.newPage({viewport:{width:1100,height:800},reducedMotion:'reduce'});page.on('pageerror',e=>result.errors.push(e.message));
    const snapshot=()=>page.evaluate(()=>probes.at(-1).metrics.snapshot());
    const settle=()=>page.evaluate(()=>settle());
    for(const platform of platforms){
      await page.goto(`http://127.0.0.1:${server.address().port}/`);
      await page.evaluate(platform=>{window.controller=EquipmentModel.mount(document.getElementById('host'),{platform});},platform);
      await page.waitForLoadState('networkidle');await settle();
      assert.match(await page.locator('[data-model-status]').textContent(),/3D model ready/);
      assert((await snapshot()).offscreen_submitted_triangles>0,'A real shadow-map pass must run; projected fallback is not evidence of working shadows');
      const gpu=await page.evaluate(()=>{const gl=probes.at(-1).gl,ext=gl.getExtension('WEBGL_debug_renderer_info');return {vendor:gl.getParameter(gl.VENDOR),renderer:gl.getParameter(gl.RENDERER),unmasked:ext?gl.getParameter(ext.UNMASKED_RENDERER_WEBGL):null};});
      const row={platform,gpu,lods:[]};result.cases.push(row);
      for(const lod of [0,1,2,0]){
        await page.evaluate(({platform,lod})=>controller.update({platform,lod}),{platform,lod});await settle();
        const mesh=build({platform,lod}),ground=!platform.startsWith('air_'),expected=mesh.triangleCount*(ground?156:108)+216;
        const current=await snapshot();assert.equal(current.resident_buffer_payload_bytes,expected,`${platform} LOD${lod} buffer payload`);
        assert.equal(current.live_buffers,ground?7:6);
        const interval=await page.evaluate(async()=>{const metrics=probes.at(-1).metrics,before=metrics.snapshot();controller.rotate(.08,0);await settle();return WebGLMeasurement.interval(before,metrics.snapshot());});
        assert.equal(interval.buffer_data_calls,0,'Orbit reuses uploaded buffers');assert.equal(interval.resident_buffer_payload_bytes,expected);
        assert(interval.submitted_triangles>=mesh.triangleCount+2,'Main mesh and floor submitted');assert.equal(interval.offscreen_submitted_triangles,0,'Orbit requires no new offscreen shadow pass');
        row.lods.push({lod,stored_model_triangles:mesh.triangleCount,expected_buffer_payload_bytes:expected,current,orbit_interval:interval});
      }
      const before=await snapshot();await page.evaluate(()=>{controller.selectPart(document.querySelector('[data-model-part] option[value]:not([value=""])').value);controller.setFinish('sand');});await settle();
      const painted=await snapshot();assert.equal(painted.resident_buffer_payload_bytes,before.resident_buffer_payload_bytes,'Paint/highlight do not duplicate stored geometry');
      row.paint_and_selection=painted;
      row.shader_diagnostics=await page.evaluate(()=>probes.at(-1).metrics.diagnostics());
      assert.deepEqual(row.shader_diagnostics,[],'Real shader compilation must succeed');
      if(platform==='tank_standard'||platform==='air_fighter'){
        assert(await page.evaluate(()=>!!(window.lossExtension=probes.at(-1).gl.getExtension('WEBGL_lose_context'))));
        await page.evaluate(()=>lossExtension.loseContext());await page.waitForFunction(()=>probes.at(-1).metrics.snapshot().context_losses===1);
        row.lost=await snapshot();assert.equal(row.lost.resident_buffer_payload_bytes,0);assert.equal(row.lost.live_buffers,0);
        await page.evaluate(()=>lossExtension.restoreContext());await page.waitForFunction(()=>document.querySelector('[data-model-status]').textContent.includes('3D model ready'));await settle();
        row.restored=await snapshot();assert.equal(row.restored.resident_buffer_payload_bytes,before.resident_buffer_payload_bytes);
        assert(row.restored.offscreen_submitted_triangles>before.offscreen_submitted_triangles,'Restore rebuilds the actual shadow target');
      }
      if(platform==='air_fighter'){
        await page.setViewportSize({width:390,height:844});await settle();assert(await page.evaluate(()=>document.documentElement.scrollWidth<=innerWidth));
        await page.screenshot({path:path.join(out,'fighter-narrow.png')});await page.setViewportSize({width:1100,height:800});
      }
      await page.evaluate(()=>controller.dispose());row.disposed=await snapshot();assert.equal(row.disposed.resident_buffer_payload_bytes,0);assert.equal(row.disposed.live_buffers,0);
      // A fresh visit must not resurrect disposed buffers on the previous context.
      await page.evaluate(platform=>{window.controller=EquipmentModel.mount(document.getElementById('host'),{platform,lod:2});},platform);await settle();
      assert.equal(await page.evaluate(()=>probes[0].metrics.snapshot().live_buffers),0);
      await page.evaluate(()=>controller.dispose());assert((await page.evaluate(()=>probes.map(p=>p.metrics.snapshot()))).every(p=>p.live_buffers===0));
      console.log(`${platform}: LOD replacement, redraw, disposal and revisit passed`);
    }
    assert.deepEqual(result.errors,[]);result.passed=true;
  }catch(error){result.failure=String(error.stack||error);throw error;}
  finally{fs.writeFileSync(path.join(out,'result.json'),JSON.stringify(result,null,2)+'\n');if(browser)await browser.close();await new Promise(r=>server.close(r));console.log(path.join(out,'result.json'));}
})().catch(error=>{console.error(error);process.exitCode=1;});
