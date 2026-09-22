const {chromium}=require('playwright'),assert=require('node:assert/strict'),fs=require('node:fs'),path=require('node:path');
const base=__dirname,repo=path.join(base,'integration'),info=JSON.parse(fs.readFileSync(path.join(base,'S22-prep-review-launch.json'),'utf8'));
const out=path.join(base,'evidence/S22-prep-native-review');assert(!fs.existsSync(out));fs.mkdirSync(out);
(async()=>{
 const browser=await chromium.launch({channel:'chrome',headless:true}),page=await browser.newPage({viewport:{width:1440,height:1000},reducedMotion:'reduce'});
 const proof={passed:false,revision:info.runtime_revision,url:info.url,scope:'Native embedded-asset smoke test through existing equipment entry function; not a first-hour usability or campaign qualification.',commands:[],errors:[]};
 try{
  await page.addInitScript({path:path.join(repo,'tools/ui/webgl-measurement.js')});
  await page.addInitScript(()=>{window.equipmentProbes=[];const original=HTMLCanvasElement.prototype.getContext;HTMLCanvasElement.prototype.getContext=function(...args){const gl=original.apply(this,args);if(gl&&args[0]==='webgl'&&this.closest('[data-equipment-model]')&&!equipmentProbes.some(p=>p.gl===gl))equipmentProbes.push({gl,metrics:WebGLMeasurement.attach(gl)});return gl;};});
  page.on('pageerror',e=>proof.errors.push(e.message));page.on('request',r=>{if(r.method()==='POST'&&['/api/command','/api/advance'].includes(new URL(r.url()).pathname))proof.commands.push(new URL(r.url()).pathname);});
  await page.goto(info.url,{waitUntil:'domcontentloaded'});await page.locator('#continueBtn').click();await page.locator('#app').waitFor({state:'visible'});
  await page.evaluate(()=>openEquipment());await page.waitForFunction(()=>document.querySelector('#equipmentRoom [data-model-status]')?.textContent.includes('3D model ready'));
  await page.locator('#equipmentRoom [data-equipment-model]').scrollIntoViewIfNeeded();await page.evaluate(()=>new Promise(r=>requestAnimationFrame(()=>requestAnimationFrame(r))));
  proof.graphics=await page.evaluate(()=>({metrics:equipmentProbes.at(-1).metrics.snapshot(),diagnostics:equipmentProbes.at(-1).metrics.diagnostics()}));
  assert(proof.graphics.metrics.offscreen_submitted_triangles>0);assert.deepEqual(proof.graphics.diagnostics,[]);
  await page.locator('#equipmentRoom [data-equipment-model]').screenshot({path:path.join(out,'native-equipment-shadows.png')});
  assert.deepEqual(proof.commands,[]);assert.deepEqual(proof.errors,[]);proof.passed=true;
 }catch(error){proof.failure=String(error.stack||error);throw error;}
 finally{fs.writeFileSync(path.join(out,'result.json'),JSON.stringify(proof,null,2));await browser.close();console.log(path.join(out,'result.json'));}
})().catch(e=>{console.error(e);process.exitCode=1;});
