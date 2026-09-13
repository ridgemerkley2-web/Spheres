// S09: a fresh France campaign and the shipped research/designer controls.
// Requires a clean committed release build. No granted research, funds or models.
const assert=require('node:assert/strict'),fs=require('node:fs'),path=require('node:path');
const cp=require('node:child_process'),net=require('node:net'),crypto=require('node:crypto');
const {chromium}=require('playwright'),integrated=require('./ci-integrated.cjs');
const audit=require('./supplier-archive-audit.cjs');
const root=path.resolve(__dirname,'../..'),copy=x=>JSON.parse(JSON.stringify(x));
const quoted=x=>JSON.stringify(String(x));
const route=(r,p)=>new URL(r.url()).pathname===p&&r.request().method()==='POST';
async function freePort(){const s=net.createServer();await new Promise(r=>s.listen(0,'127.0.0.1',r));const p=s.address().port;await new Promise(r=>s.close(r));return p;}
async function get(page,url,p){const r=await page.request.get(url+p);try{assert(r.ok(),p);return await r.json();}finally{await r.dispose();}}
async function ready(page){await page.waitForFunction(()=>equipmentCurrent()&&equipmentPreviewCurrent()&&!EQUIP.review&&!EQUIP.replacement);}
async function idle(page){await page.waitForFunction(()=>!SESSION.busy&&!COMMAND_CHANNEL.busy&&!COMMAND_CHANNEL.pending&&!advancing&&!pendingAdvance&&!EQUIP.busy);}
async function designer(page){
  if(!await page.locator('#equipmentRoom').isVisible()){
    await page.locator('#techBtn').click();await page.locator('#techMenu [data-equipment-bureau]').click();
  }
  await page.locator('[data-equipment-tab="designer"]').first().click();await ready(page);
}
async function draft(page){return page.evaluate(()=>JSON.parse(JSON.stringify(EQUIP.draft)));}
async function captures(page,url,run,slot){
  const slots=new Set(['s09-before-guidance','s09-after-guidance','s09-after-draft-save','s09-after-load','s09-after-continue']);
  assert(slots.has(slot));const base=fs.realpathSync(run),saves=path.join(base,'saves'),source=path.join(saves,slot+'.json');
  assert.equal(fs.realpathSync(saves),saves);assert(!fs.existsSync(source));
  const response=await page.request.post(url+'/api/save',{data:{slot}});
  try{assert(response.ok());await response.body();}finally{await response.dispose();}
  assert(fs.lstatSync(source).isFile()&&!fs.lstatSync(source).isSymbolicLink());
  assert.equal(fs.realpathSync(source),source);
  const output=path.join(base,'audit-captures');if(!fs.existsSync(output))fs.mkdirSync(output);
  assert.equal(fs.realpathSync(output),output);assert(!fs.lstatSync(output).isSymbolicLink());
  const target=path.join(fs.mkdtempSync(path.join(output,slot+'-')),slot+'.json');assert(!fs.existsSync(target));
  fs.renameSync(source,target);return audit.inspect(target,'France');
}
async function campaigns(page){
  if(await page.locator('#equipmentRoom').isVisible())await page.locator('[data-equipment-close]').click();
  if(await page.locator('#cabinetDrawer').isVisible())await page.locator('#cabinetDrawer [data-close-drawers]').click();
  if(await page.locator('#app').isVisible()){
    if(await page.locator('.arc-time-menu').getAttribute('open')===null)await page.locator('.arc-time-menu > summary').click();
    await page.locator('#campaignsBtn').click();
  }
  if(!await page.locator('#savedCampaigns').isVisible())await page.locator('#openSavesBtn').click();
  await page.locator('#savedCampaigns').waitFor({state:'visible'});
}
async function reopenSaved(page,name){
  await designer(page);await page.locator('[data-equipment-tab="library"]').first().click();
  const id='draft:'+name;await page.locator('[data-equipment-edit='+quoted(id)+']').click();
  if(await page.locator('[data-equipment-replacement]').isVisible())await page.locator('[data-equipment-replacement-accept]').click();
  await ready(page);await page.locator('[data-equipment-draft-status="saved"]').waitFor({state:'visible'});
  return draft(page);
}
async function main(){
  assert(process.env.SPHERES_BINARY,'Set the already built SPHERES_BINARY');
  assert.match(process.env.SPHERES_EXPECTED_REVISION||'',/^[a-f0-9]{40}$/);
  const git=args=>cp.execFileSync('git',['-c','core.longpaths=true',...args],{cwd:root,encoding:'utf8',windowsHide:true});
  const sourceRevision=git(['rev-parse','HEAD']).trim();assert.equal(git(['status','--porcelain']).trim(),'','Browser driver needs a clean committed checkout');
  assert.equal(git(['diff','--name-only',process.env.SPHERES_EXPECTED_REVISION,sourceRevision,'--','spheres-sim','spheres-cli','spheres-web','Cargo.toml','Cargo.lock']).trim(),'','A newer browser-driver commit must preserve the complete runtime source');
  const binary=path.resolve(process.env.SPHERES_BINARY);assert(fs.statSync(binary).isFile());
  const output=path.resolve(process.env.SPHERES_RESEARCH_OUTPUT||path.join(root,'artifacts/browser-research-design-ci'));
  fs.mkdirSync(output,{recursive:true});const out=fs.mkdtempSync(path.join(output,'france-')),run=path.join(out,'server');fs.mkdirSync(run);fs.mkdirSync(path.join(run,'saves'));
  const port=await freePort(),url='http://127.0.0.1:'+port;
  const server=cp.spawn(binary,['--port',String(port),'--no-open'],{cwd:run,windowsHide:true,stdio:['ignore','pipe','pipe']});
  const log=fs.createWriteStream(path.join(out,'server.log'));server.stdout.pipe(log);server.stderr.pipe(log);
  let browser,page,launchError,stage='launch';server.on('error',e=>{launchError=e;});
  const e={passed:false,run,url,started_utc:new Date().toISOString(),test_source:{revision:sourceRevision,driver_sha256:crypto.createHash('sha256').update(fs.readFileSync(__filename)).digest('hex'),runtime_source_equal:true},fixture:'Fresh France, ordinary menus and controls; no state grants or substituted campaign responses',screenshots:[],commands:[],errors:[],checks:[]};
  const write=(name,value)=>fs.writeFileSync(path.join(out,name),JSON.stringify(value,null,2)+'\n');
  const telemetry=(event,details)=>fs.appendFileSync(path.join(out,'progress.jsonl'),JSON.stringify({utc:new Date().toISOString(),stage,event,...details})+'\n');
  const shot=async(name,selector)=>{if(selector)await page.locator(selector).scrollIntoViewIfNeeded();await page.screenshot({path:path.join(out,name+'.png')});e.screenshots.push(name+'.png');};
  audit.configure(out,telemetry);
  try{
    const until=Date.now()+20000;
    for(;;){
      if(launchError)throw launchError;assert.equal(server.exitCode,null);
      try{const r=await fetch(url+'/api/build',{headers:{Connection:'close'},signal:AbortSignal.timeout(2000)});await r.arrayBuffer();if(r.ok)break;}catch(error){if(Date.now()>=until)throw error;}
      assert(Date.now()<until,'Server startup exceeded 20 seconds');await new Promise(r=>setTimeout(r,100));
    }
    browser=await chromium.launch({headless:true,...(process.env.SPHERES_BROWSER_CHANNEL?{channel:process.env.SPHERES_BROWSER_CHANNEL}:{})});
    e.browser_version=browser.version();page=await browser.newPage({viewport:{width:1440,height:1000},reducedMotion:'reduce'});page.setDefaultTimeout(30000);
    const cdp=await page.context().newCDPSession(page);await cdp.send('Network.enable');await cdp.send('Page.enable');await cdp.send('Page.setLifecycleEventsEnabled',{enabled:true});
    const network=path.join(out,'browser-lifecycle.jsonl'),observe=(kind,value)=>fs.appendFileSync(network,JSON.stringify({utc:new Date().toISOString(),stage,kind,value})+'\n');
    for(const event of ['Network.requestWillBeSent','Network.responseReceived','Network.loadingFinished','Network.loadingFailed','Page.lifecycleEvent'])cdp.on(event,value=>observe(event,value));
    page.on('console',message=>observe('console',{type:message.type(),text:message.text()}));page.on('crash',()=>observe('page-crash',{}));
    page.on('pageerror',error=>e.errors.push(error.message));page.on('request',r=>{if(new URL(r.url()).pathname==='/api/command'&&r.method()==='POST')e.commands.push(r.postDataJSON());});
    e.build=await integrated.verifyBuild({page,url,root,run,binary});
    stage='fresh France';await page.goto(url,{waitUntil:'domcontentloaded'});await page.locator('#campaignHome').waitFor();await page.waitForFunction(()=>!!SESSION.live?.session_id);
    await page.locator('#newCampaignBtn').click();await page.locator('#nationPick [aria-label^="France;"]').click();await page.locator('#startBtn').click();
    await page.locator('#app').waitFor();await idle(page);assert.equal(await page.evaluate(()=>clock.running),false);
    const initial=await get(page,url,'/api/state');assert.equal(initial.player,'France');e.capabilities=await integrated.capabilities({page,url,player:'France'});
    const before=await captures(page,url,run,'s09-before-guidance');

    stage='recommendations and cost reading';await designer(page);
    const initialDraft=await draft(page);const guidance=await page.evaluate(()=>JSON.parse(JSON.stringify(EQUIP.preview.guidance)));
    assert.equal(guidance.platform,initialDraft.platform);assert(guidance.recommendations.length>0,'The fresh platform needs a known legal recommendation');
    for(const row of guidance.recommendations){
      const p=await page.request.post(url+'/api/equipment-preview',{data:{session_id:initial.session_id,name:'Read-only recommendation check',...row.spec}});
      try{assert(p.ok());const v=await p.json();assert.equal(v.valid,true);assert.equal(v.nation,'France');}finally{await p.dispose();}
      assert(await page.locator('[data-equipment-recommendation='+quoted(row.id)+']').isVisible());
    }
    const acquisition=guidance.costs.find(row=>/purchase|acquisition/i.test(row.label));assert(acquisition&&Number.isFinite(acquisition.amount_bn)&&acquisition.amount_bn>0,'Native guidance must supply a conditional acquisition estimate');
    assert(guidance.conditions.length>0);write('initial-guidance.json',guidance);
    const art=await page.request.get(url+'/art/pages/military-research-v1.webp');try{assert(art.ok(),'Existing research background art must be served');await art.body();}finally{await art.dispose();}
    // Declared in the S09 protocol before the first measurement. Sequential
    // full HTTP/JSON reads, no response replacement or cache installation.
    const board=await page.evaluate(()=>JSON.parse(JSON.stringify(EQUIP.data)));
    const timings=[];
    for(const item of [{id:'board'},...board.presets.map(p=>({id:p.platform,spec:{platform:p.platform,components:p.components}}))]){
      const samples=[];
      for(let i=0;i<24;i++){
        const start=performance.now();let r;
        try{
          r=item.spec?await page.request.post(url+'/api/equipment-preview',{data:{session_id:initial.session_id,name:'Performance read',...item.spec}}):await page.request.get(url+'/api/equipment?session_id='+encodeURIComponent(initial.session_id));
          assert(r.ok());const value=await r.json();assert.equal(value.session_id,initial.session_id);
        }finally{if(r)await r.dispose();}
        if(i>=3)samples.push(performance.now()-start);
      }
      const sorted=[...samples].sort((a,b)=>a-b),p95=sorted[Math.ceil(sorted.length*.95)-1],max=sorted.at(-1);
      const row={id:item.id,samples_ms:samples,p95_ms:p95,max_ms:max,p95_limit_ms:item.spec?250:300,max_limit_ms:item.spec?500:750};
      row.passed=p95<=row.p95_limit_ms&&max<=row.max_limit_ms;timings.push(row);write('read-performance.json',{warmups:3,samples:21,rows:timings});
    }
    assert.equal(new Set(timings.map(r=>r.id)).size,12,'Board and all eleven platform reads');
    assert(timings.every(r=>r.passed),'Read performance exceeded the predeclared S09 limits');e.read_performance=timings.map(({samples_ms,...r})=>r);
    await shot('design-guidance-desktop','[data-equipment-guidance]');
    await page.setViewportSize({width:390,height:844});await integrated.withinPanel(page,'#equipmentRoot','Designer mobile');await shot('design-guidance-mobile','[data-equipment-guidance]');
    await page.setViewportSize({width:1440,height:1000});
    const name='S09 Atlas';await page.locator('#equipmentName').fill(name);await ready(page);const edited=await draft(page);
    await page.locator('[data-equipment-draft-status="unsaved"]').waitFor();

    stage='research and return without losing edits';await page.locator('[data-equipment-tab="research"]').first().click();
    const research=await page.evaluate(()=>JSON.parse(JSON.stringify(EQUIP.data.research)));
    const compatible=research.flatMap((row,index)=>(row.unlock_components||[]).map(c=>({row,index,c}))).find(({c})=>c.compatible_platforms?.some(p=>p.id===edited.platform)&&c.id!==edited.components[c.slot]);
    assert(compatible,'Research must expose another compatible component to explore');
    await page.locator('[data-equipment-branch='+quoted(compatible.row.branch)+']').click();
    const unlock=page.locator('[data-equipment-unlock='+quoted(compatible.c.id)+']');
    // Expand the real component detail if its natural control is in a details section.
    const ancestors=await unlock.evaluate(el=>{const keys=[];for(let p=el.parentElement;p;p=p.parentElement)if(p.tagName==='DETAILS')keys.unshift(p.dataset.equipmentDetail);return keys;});
    for(const key of ancestors){const details=page.locator('details[data-equipment-detail='+quoted(key)+']');if(await details.getAttribute('open')===null)await details.locator(':scope > summary').click();}
    await unlock.click();await page.locator('[data-equipment-replacement]').waitFor();
    assert.deepEqual(await draft(page),edited,'Exploration silently replaced unsaved work');
    await shot('research-part-review','[data-equipment-replacement]');
    await page.locator('[data-equipment-replacement-cancel]').click();assert.deepEqual(await draft(page),edited);
    await page.locator('[data-equipment-return-draft]').click();await ready(page);assert.deepEqual(await draft(page),edited);
    e.checks.push('Research exploration/cancel/return preserves the exact unsaved draft');
    const external=research.flatMap((row,index)=>(row.prerequisites||[]).map((p,pi)=>({row,index,p,pi}))).find(({p})=>p.domain&&!research.some(r=>r.id===p.id));
    assert(external,'A genuine prerequisite route to the general research screen must be available');
    await page.locator('[data-equipment-tab="research"]').first().click();await page.locator('[data-equipment-branch='+quoted(external.row.branch)+']').click();
    await page.locator('[data-equipment-prerequisite='+quoted(external.index+':'+external.pi)+']').click();
    await page.waitForFunction(id=>tech.open&&!equipmentActive()&&tech.data?.[tech.sel]?.id===id,external.p.id);assert.deepEqual(await draft(page),edited);
    await page.getByRole('button',{name:'Equipment designer',exact:true}).click();await designer(page);assert.deepEqual(await draft(page),edited);
    e.checks.push('External prerequisite research and return retain the unsaved draft');

    stage='explicit suggested configuration';
    const suggestion=await page.evaluate(()=>JSON.parse(JSON.stringify(EQUIP.preview.guidance.recommendations.find(r=>r.id==='advanced')||EQUIP.preview.guidance.recommendations[0])));
    await page.locator('[data-equipment-recommendation-use='+quoted(suggestion.id)+']').click();
    if(await page.locator('[data-equipment-replacement]').isVisible()){
      assert.deepEqual(await draft(page),edited);await page.locator('[data-equipment-replacement-cancel]').click();assert.deepEqual(await draft(page),edited);
      await page.locator('[data-equipment-recommendation-use='+quoted(suggestion.id)+']').click();await page.locator('[data-equipment-replacement-accept]').click();
    }
    await ready(page);let chosen=await draft(page);assert.equal(chosen.platform,suggestion.spec.platform);assert.deepEqual(chosen.components,suggestion.spec.components);
    await page.locator('#equipmentName').fill(name);await ready(page);chosen=await draft(page);
    assert.equal(await page.evaluate(()=>EQUIP.preview.valid),true);assert.equal(e.commands.length,0,'Advice must not issue orders');
    await shot('chosen-design-desktop','[data-equipment-guidance]');
    audit.compare(await captures(page,url,run,'s09-after-guidance'),before,'Research, local drafts and recommendations changed the native campaign');
    e.checks.push('Every displayed recommendation recompiles legally; all exploration is pure');

    stage='save exact design draft';
    await page.locator('[data-equipment-action][data-equipment-scope="preview"]').filter({hasText:'Save design draft'}).click();
    await page.locator('#equipmentActionReviewTitle').waitFor();await page.locator('[data-equipment-confirm]').click();
    await idle(page);await ready(page);await page.locator('[data-equipment-draft-status="saved"]').waitFor();
    assert.equal(e.commands.length,1);assert.deepEqual(e.commands.flatMap(r=>r.commands).map(c=>c.kind),['equipment_save']);
    const saved=await page.evaluate(name=>JSON.parse(JSON.stringify(EQUIP.data.designs.find(r=>r.id==='draft:'+name))),name);
    assert(saved);assert.equal(saved.name,name);assert.deepEqual(saved.spec,{platform:chosen.platform,components:chosen.components});
    e.saved_draft=saved;e.checks.push('One explicit save command stores the reviewed name and complete specification');
    const afterSave=await captures(page,url,run,'s09-after-draft-save');
    assert.equal(afterSave.buyer.learned_sha256,before.buyer.learned_sha256);assert.deepEqual(afterSave.buyer.revision_ids,before.buyer.revision_ids);assert.deepEqual(afterSave.buyer.held_by_revision,before.buyer.held_by_revision);
    await shot('saved-draft-desktop','[data-equipment-draft-status]');

    stage='named save and load';const slot='s09-saved-design',beforeLoad=await get(page,url,'/api/state');await campaigns(page);
    await page.locator('#saveName').fill(slot);const saveResponse=page.waitForResponse(r=>route(r,'/api/save'));await page.locator('#saveNamedBtn').click();assert((await saveResponse).ok());
    await page.locator('#saveSlots option[value='+quoted(slot)+']').waitFor({state:'attached'});await page.locator('#saveSlots').selectOption(slot);
    await page.locator('#loadBtn').click();await page.locator('#campaignConfirmCancel').click();assert.deepEqual(await get(page,url,'/api/state'),beforeLoad);
    const loadResponse=page.waitForResponse(r=>route(r,'/api/load'));await page.locator('#loadBtn').click();await page.locator('#campaignConfirmAccept').click();assert((await loadResponse).ok());
    await page.locator('#app').waitFor();await idle(page);const loaded=await get(page,url,'/api/state');assert.notEqual(loaded.session_id,beforeLoad.session_id);assert.equal(loaded.date,beforeLoad.date);
    const reloaded=await reopenSaved(page,name);assert.equal(reloaded.name,name);assert.deepEqual(reloaded.components,chosen.components);assert.equal(reloaded.platform,chosen.platform);
    audit.compare(await captures(page,url,run,'s09-after-load'),afterSave,'Named Save/Load changed the saved native design or other campaign property');
    await page.reload();await page.locator('#continueBtn').click();await page.locator('#app').waitFor();await idle(page);
    const continued=await reopenSaved(page,name);assert.equal(continued.name,name);assert.deepEqual(continued.components,chosen.components);assert.equal(continued.platform,chosen.platform);
    audit.compare(await captures(page,url,run,'s09-after-continue'),afterSave,'Continue changed the saved native campaign');
    await page.setViewportSize({width:390,height:844});await integrated.withinPanel(page,'#equipmentRoot','Reopened designer mobile');await shot('reopened-draft-mobile','[data-equipment-draft-status]');
    assert.equal(e.commands.length,1);assert.deepEqual(e.errors,[]);e.checks.push('Named Save/Load cancellation, new session and Continue retain exact saved draft and native world');
    e.passed=true;e.finished_utc=new Date().toISOString();write('result.json',e);console.log(JSON.stringify({passed:true,result:path.join(out,'result.json')}));
  }catch(error){e.failed_stage=stage;e.failure=String(error.stack||error);write('result.json',e);if(page)try{await page.screenshot({path:path.join(out,'failure.png')});fs.writeFileSync(path.join(out,'failure.html'),await page.content());}catch{}throw error;}
  finally{if(browser)await browser.close();server.kill();log.end();}
}
main().catch(error=>{console.error(error);process.exitCode=1;});
