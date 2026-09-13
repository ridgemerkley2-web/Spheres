const fs=require('node:fs'),path=require('node:path'),cp=require('node:child_process'),net=require('node:net'),assert=require('node:assert/strict');
const base=__dirname,repo=path.join(base,'integration');
const {chromium}=require(path.join(repo,'tools/ui/node_modules/playwright'));
const binary=path.join(base,'integration-target/release/spheres-web.exe');
const expected='94d2c094b2c69a70613af2258acbf49e10d9e871';
const root=path.join(base,'evidence/S09-visual-review');fs.mkdirSync(root,{recursive:true});
const out=fs.mkdtempSync(path.join(root,'france-')),run=path.join(out,'server');fs.mkdirSync(run);
const result={kind:'visual-only-review',qualification:false,expected_revision:expected,run,started_utc:new Date().toISOString(),screenshots:[],page_errors:[],http_errors:[],commands:[],post_paths:[],observations:[]};
const write=()=>fs.writeFileSync(path.join(out,'result.json'),JSON.stringify(result,null,2)+'\n');
async function main(){
 const reservation=net.createServer();await new Promise(r=>reservation.listen(0,'127.0.0.1',r));const port=reservation.address().port;await new Promise(r=>reservation.close(r));
 const url='http://127.0.0.1:'+port;result.url=url;
 const server=cp.spawn(binary,['--port',String(port),'--no-open'],{cwd:run,windowsHide:true,stdio:['ignore','pipe','pipe']});result.owned_server_pid=server.pid;
 const log=fs.createWriteStream(path.join(out,'server.log'));server.stdout.pipe(log);server.stderr.pipe(log);
 let browser,page,stage='startup';console.log(JSON.stringify({out,url,pid:server.pid}));
 const shot=async(name,selector)=>{if(selector)await page.locator(selector).scrollIntoViewIfNeeded();const filename=name+'.png';await page.screenshot({path:path.join(out,filename)});result.screenshots.push({filename,viewport:page.viewportSize(),selector:selector||null});write();};
 const dimension=async(label)=>result.observations.push(await page.evaluate(label=>({label,viewport:{width:innerWidth,height:innerHeight},document:{scroll:document.documentElement.scrollWidth,client:document.documentElement.clientWidth},room:(()=>{const e=document.querySelector('#equipmentRoom');return e?{scroll:e.scrollWidth,client:e.clientWidth}:null;})(),overflow:[...document.querySelectorAll('#equipmentRoom *')].filter(e=>{const r=e.getBoundingClientRect();return r.width>0&&r.height>0&&(r.right>innerWidth+1||r.left<-1)&&getComputedStyle(e).position!=='absolute';}).slice(0,18).map(e=>({tag:e.tagName,class:e.className,text:e.textContent.slice(0,70),left:e.getBoundingClientRect().left,right:e.getBoundingClientRect().right}))}),label));
 try{
  for(let i=0;;i++){assert.equal(server.exitCode,null,'Owned server ended unexpectedly');try{const r=await fetch(url+'/api/build',{signal:AbortSignal.timeout(1000)});if(r.ok){result.build=await r.json();break;}}catch(e){if(i>=99)throw e;}await new Promise(r=>setTimeout(r,100));}
  assert.equal(result.build.revision,expected.slice(0,12));
  browser=await chromium.launch({headless:true,channel:'chrome'});result.browser=browser.version();page=await browser.newPage({viewport:{width:1440,height:1000},reducedMotion:'reduce'});page.setDefaultTimeout(30000);
  page.on('pageerror',e=>result.page_errors.push(e.message));page.on('response',r=>{if(r.status()>=400)result.http_errors.push({url:r.url(),status:r.status()});});page.on('request',r=>{if(r.method()==='POST'){const p=new URL(r.url()).pathname;result.post_paths.push(p);if(p==='/api/command')result.commands.push(r.postDataJSON());}});
  stage='ordinary new France';await page.goto(url,{waitUntil:'domcontentloaded'});await page.locator('#campaignHome').waitFor();await page.waitForFunction(()=>!!SESSION.live?.session_id);await page.locator('#newCampaignBtn').click();await page.locator('#nationPick [aria-label^="France;"]').click();await page.locator('#startBtn').click();await page.locator('#app').waitFor();
  await page.waitForFunction(()=>!SESSION.busy&&!COMMAND_CHANNEL.busy&&!COMMAND_CHANNEL.pending&&!advancing&&!pendingAdvance&&!EQUIP.busy);
  const stateResponse=await page.request.get(url+'/api/state');const state=await stateResponse.json();await stateResponse.dispose();assert.equal(state.player,'France');result.campaign={player:state.player,date:state.date,session_id:state.session_id};
  stage='designer desktop';await page.locator('#techBtn').click();await page.locator('#techMenu [data-equipment-bureau]').click();await page.locator('[data-equipment-tab="designer"]').first().click();await page.waitForFunction(()=>equipmentCurrent()&&equipmentPreviewCurrent()&&!EQUIP.review&&!EQUIP.replacement);
  try{await page.waitForFunction(()=>!document.querySelector('[data-model-status]')?.textContent.includes('Preparing'),{},{timeout:15000});}catch(e){result.observations.push({label:'model readiness',note:e.message});}
  result.model_status=await page.locator('[data-model-status]').allTextContents();result.guidance_text=await page.locator('[data-equipment-guidance]').innerText();
  await shot('desktop-designer-top','[data-equipment-draft-status]');await shot('desktop-model','[data-equipment-model]');await shot('desktop-guidance','[data-equipment-recommendation="economical"]');await shot('desktop-current-costs','.eq-guidance-current');await dimension('desktop designer');
  stage='designer mobile';await page.setViewportSize({width:390,height:844});await shot('mobile-designer-top','[data-equipment-draft-status]');await shot('mobile-model','[data-equipment-model]');await shot('mobile-lower-cost','[data-equipment-recommendation="economical"]');await shot('mobile-advanced','[data-equipment-recommendation="advanced"]');await shot('mobile-current-costs','.eq-guidance-current');await dimension('mobile designer');
  stage='research detail';await page.setViewportSize({width:1440,height:1000});await page.locator('[data-equipment-tab="research"]').first().click();await page.locator('[data-equipment-branch="optics"]').click();
  const detail=page.locator('details[data-equipment-detail^="unlock:"]').first();await detail.waitFor();result.component_detail=await detail.getAttribute('data-equipment-detail');
  if(await detail.getAttribute('open')===null)await detail.locator(':scope > summary').click();
  await shot('desktop-research-component','details[data-equipment-detail="'+result.component_detail+'"]');result.component_text=await detail.innerText();await dimension('desktop research');
  await page.setViewportSize({width:390,height:844});await shot('mobile-research-component','details[data-equipment-detail="'+result.component_detail+'"]');await dimension('mobile research');
  assert.equal(result.commands.length,0,'Visual review must issue no game orders');result.review_capture_completed=true;console.log(JSON.stringify({completed:true,out,build:result.build,errors:result.page_errors,http_errors:result.http_errors,commands:result.commands.length}));
 }catch(error){result.review_capture_completed=false;result.failure={stage,message:error.message,stack:error.stack};if(page)try{await shot('failure');}catch{}console.log(JSON.stringify({completed:false,out,stage,error:error.message}));process.exitCode=1;}
 finally{if(browser)await browser.close();if(server.exitCode===null){server.kill();await Promise.race([new Promise(r=>server.once('exit',r)),new Promise(r=>setTimeout(r,5000))]);}log.end();result.owned_server_exit={exit_code:server.exitCode,signal:server.signalCode};result.finished_utc=new Date().toISOString();write();}
}
main().catch(e=>{console.error(e);process.exitCode=1;});
