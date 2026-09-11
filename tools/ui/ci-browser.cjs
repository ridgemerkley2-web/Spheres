// Disposable real-browser CI smoke, including a committed response lost in
// transit. Local agent UI verification uses the computer-use browser instead.
const assert=require('node:assert/strict'),fs=require('node:fs'),path=require('node:path'),net=require('node:net'),cp=require('node:child_process');
const {chromium}=require('playwright');
const integrated=require('./ci-integrated.cjs');
const root=path.resolve(__dirname,'../..'),out=path.join(root,'artifacts/browser-ci');
async function port(){const s=net.createServer();await new Promise(r=>s.listen(0,'127.0.0.1',r));const p=s.address().port;await new Promise(r=>s.close(r));return p;}
(async()=>{
  fs.mkdirSync(out,{recursive:true});const run=fs.mkdtempSync(path.join(out,'campaign-')),p=await port(),url=`http://127.0.0.1:${p}`;
  const binary=path.resolve(process.env.SPHERES_BINARY||path.join(root,'target/release/spheres-web'+(process.platform==='win32'?'.exe':'')));
  const server=cp.spawn(binary,['--port',String(p),'--no-open'],{cwd:run,windowsHide:true,stdio:['ignore','pipe','pipe']});
  const log=fs.createWriteStream(path.join(run,'server.log'));server.stdout.pipe(log);server.stderr.pipe(log);
  let browser;
  try{
    const startupDeadline=Date.now()+20000;
    for(let n=0;;n++){try{const response=await fetch(url+'/api/build',{headers:{Connection:'close'},signal:AbortSignal.timeout(Math.max(1,Math.min(2000,startupDeadline-Date.now())))});await response.arrayBuffer();if(response.ok)break;}catch(_){}if(n>=200||Date.now()>=startupDeadline)throw Error('Disposable server failed to start');await new Promise(r=>setTimeout(r,100));}
    browser=await chromium.launch({headless:true});const page=await browser.newPage({viewport:{width:1440,height:1000},reducedMotion:'reduce'});
    page.setDefaultTimeout(30000);const errors=[];page.on('pageerror',e=>errors.push(e.message));
    const buildEvidence=await integrated.verifyBuild({page,url,root,run,binary});
    // Delay only the first real discovery response; menu choices must survive
    // its completion. No synthetic state or response body is supplied.
    let releaseBoot,bootHeld=false;const bootGate=new Promise(resolve=>{releaseBoot=resolve;});
    await page.route('**/api/state',async route=>{if(bootHeld)return route.continue();bootHeld=true;const response=await route.fetch();await bootGate;await route.fulfill({response});});
    await page.goto(url,{waitUntil:'domcontentloaded'});await page.locator('#campaignHome').waitFor({state:'visible'});
    assert(await page.locator('#newCampaignPicker').isHidden());
    await page.locator('#openSavesBtn').click();await page.locator('#savedCampaigns').waitFor({state:'visible'});
    await page.locator('#menuSaveEmpty').waitFor({state:'visible'});
    releaseBoot();await page.waitForFunction(()=>!!SESSION.live?.session_id);
    assert.equal(await page.locator('#setup').getAttribute('data-menu-view'),'saves','Delayed live-world discovery must retain the player-selected Saves view');
    assert(await page.locator('#savedCampaigns').isVisible());assert(await page.locator('#campaignHome').isHidden());
    await page.unroute('**/api/state');
    assert(await page.locator('#loadBtn').isDisabled(),'A fresh disposable folder has no campaign to load');
    await page.locator('#savedCampaigns [data-menu-back]').click();
    await page.locator('#newCampaignBtn').click();await page.locator('#newCampaignPicker').waitFor({state:'visible'});
    await page.locator('#nationPick [aria-label^="United States;"]').click();await page.locator('#startBtn').click();await page.locator('#app').waitFor({state:'visible'});
    const state=async()=> (await page.request.get(url+'/api/state')).json();
    const initial=await state();assert.equal(initial.player,'USA');assert.equal(initial.simulation_cadence,'daily');
    const adviceResponse=page.waitForResponse(response=>new URL(response.url()).pathname==='/api/guidance'&&response.request().method()==='GET');
    await page.getByRole('button',{name:'Advisors',exact:true}).click();
    await page.locator('#guidanceDialog').getByRole('heading',{name:'What needs your attention?',exact:true}).waitFor();
    assert((await adviceResponse).ok(),'The current advisory council must load its native campaign reading');
    await page.locator('#guidanceDialog .guidance-cards[aria-busy="false"]').waitFor();
    await page.getByRole('button',{name:'Close tutorial and advisors',exact:true}).click();
    await page.getByRole('button',{name:'Find',exact:true}).click();await page.locator('#worldFindInput').fill('California');await page.locator('[data-find-id="US-CA"]').click();await page.locator('#provinceDossier').waitFor({state:'visible'});
    await page.evaluate(()=>closeProvince());
    const integrationEvidence=await integrated.panels({page,url,run,out,player:initial.player});
    // Let the normal command channel create the receipt. Lose only the first
    // already-committed response, then recover via the visible receipt button.
    let lost=false;const commandRequests=[];await page.route('**/api/command',async route=>{commandRequests.push(route.request().postDataJSON());if(lost)return route.continue();lost=true;await route.fetch();await route.abort('failed');});
    const lostResponse=await page.evaluate(()=>api('/api/command',{commands:[{kind:'tax',value:0.29}]}));
    // Legacy panels receive an unchanged state with errors, not an exception.
    assert.equal(lostResponse.command_pending,true,'A committed response loss must retain uncertainty');
    assert(Array.isArray(lostResponse.errors)&&lostResponse.errors.some(message=>typeof message==='string'&&message.length>0),'The pending legacy-panel response must explain its error');
    await page.locator('#retryCommandBtn').waitFor({state:'visible'});const committed=await state();
    const pendingReceipt=await page.evaluate(()=>JSON.parse(JSON.stringify(COMMAND_CHANNEL.pending)));
    assert(pendingReceipt&&pendingReceipt.session_id===committed.session_id);
    assert.equal(commandRequests.length,1);
    await page.reload();await page.locator('#continueBtn').waitFor({state:'visible'});await page.locator('#continueBtn').click();await page.locator('#app').waitFor({state:'visible'});
    await page.locator('#retryCommandBtn').waitFor({state:'visible'});
    assert.deepEqual(await page.evaluate(()=>JSON.parse(JSON.stringify(COMMAND_CHANNEL.pending))),pendingReceipt,'Reload must retain the exact frozen receipt');
    assert.equal(commandRequests.length,1,'Boot and Continue must not retry an uncertain command automatically');
    assert.deepEqual(await state(),committed,'Reload and Continue must not change the committed campaign');
    await page.locator('#retryCommandBtn').click();await page.locator('#pendingCommand').waitFor({state:'hidden'});
    assert.equal(commandRequests.length,2);assert.deepEqual(commandRequests[1],commandRequests[0],'Visible retry must send the identical receipt and payload');
    const recovered=await state();assert.equal(recovered.nations.find(n=>n.id==='USA').political_capital,committed.nations.find(n=>n.id==='USA').political_capital);
    assert.equal(recovered.nations.find(n=>n.id==='USA').tax,0.29);
    await page.unroute('**/api/command');
    const saved=await page.request.post(url+'/api/save',{data:{slot:'ci-smoke'}});assert(saved.ok());
    const history=await (await page.request.get(url+'/api/history?nations=USA')).json();
    if(await page.locator('.arc-time-menu').getAttribute('open')===null)await page.locator('.arc-time-menu > summary').click();
    const initialSaveList=page.waitForResponse(response=>new URL(response.url()).pathname==='/api/saves');
    await page.locator('#campaignsBtn').click();await page.locator('#campaignHome').waitFor({state:'visible'});
    assert((await initialSaveList).ok());
    await page.locator('#saveSlots option[value="ci-smoke"]').waitFor({state:'attached'});
    // Hold the next actual list response until the player has reviewed a
    // different selection. Its original body must not restore the old choice.
    let releaseSaveList;const saveListGate=new Promise(resolve=>{releaseSaveList=resolve;});
    await page.route('**/api/saves',async route=>{const response=await route.fetch();await saveListGate;await route.fulfill({response});});
    await page.locator('#openSavesBtn').click();await page.locator('#savedCampaigns').waitFor({state:'visible'});
    await page.locator('#saveSlots').selectOption('ci-smoke');
    await page.locator('#loadBtn').click();await page.locator('#campaignConfirmDialog').waitFor({state:'visible'});
    assert(await page.locator('#campaignConfirmCancel').evaluate(element=>element===document.activeElement),'Load confirmation must initially focus Cancel');
    assert((await page.locator('#campaignConfirmMessage').innerText()).includes('ci-smoke'));
    // Observe the real select rebuild, not a timeout or a synthetic response.
    await page.evaluate(()=>{window.__s05SaveListApplied=new Promise(resolve=>{
      const observer=new MutationObserver(()=>{observer.disconnect();resolve();});
      observer.observe(document.getElementById('saveSlots'),{childList:true});
    });});
    releaseSaveList();
    await page.evaluate(async()=>{await window.__s05SaveListApplied;delete window.__s05SaveListApplied;});
    await page.unroute('**/api/saves');
    assert.equal(await page.locator('#saveSlots').inputValue(),'ci-smoke','A delayed native list response must preserve the latest selected save');
    await page.locator('#campaignConfirmCancel').click();await page.locator('#campaignConfirmDialog').waitFor({state:'hidden'});
    assert.deepEqual(await state(),recovered,'Cancelling a named load must preserve the live campaign');
    assert.equal(await page.locator('#saveSlots').inputValue(),'ci-smoke','Cancelling must retain the reviewed selection');
    await page.locator('#loadBtn').click();await page.locator('#campaignConfirmDialog').waitFor({state:'visible'});
    assert((await page.locator('#campaignConfirmMessage').innerText()).includes('ci-smoke'));
    const loadResponse=page.waitForResponse(response=>response.url()===url+'/api/load'&&response.request().method()==='POST');
    await page.locator('#campaignConfirmAccept').click();const loaded=await loadResponse;assert(loaded.ok());
    const loadRequest=loaded.request().postDataJSON();assert.equal(loadRequest.slot,'ci-smoke');assert.equal(loadRequest.backup,false);
    await page.locator('#app').waitFor({state:'visible'});const restoredState=await state();
    assert.equal(restoredState.player,recovered.player);assert.equal(restoredState.date,recovered.date);assert.notEqual(restoredState.session_id,recovered.session_id);
    const loadedCapabilities=await integrated.capabilities({page,url,player:initial.player});
    assert.deepEqual(loadedCapabilities,integrationEvidence.capabilities);
    const restored=await(await page.request.get(url+'/api/history?nations=USA')).json();
    delete restored.session_id;delete history.session_id;assert.deepEqual(restored,history);
    await page.reload();await page.locator('#continueBtn').click();
    await page.locator('#techBtn').click();await page.locator('#techMenu').getByRole('button',{name:/^Explore technology/}).click();await page.getByRole('button',{name:'Research list',exact:true}).click();await page.locator('#researchListQuery').fill('');await page.locator('[data-research-id]').first().waitFor();
    await page.screenshot({path:path.join(out,'research-desktop.png')});await page.setViewportSize({width:414,height:896});
    assert(await page.evaluate(()=>document.querySelector('#decisionDialog').scrollWidth<=document.querySelector('#decisionDialog').clientWidth+1));
    await page.screenshot({path:path.join(out,'research-mobile.png')});assert.deepEqual(errors,[]);
    fs.writeFileSync(path.join(out,'result.json'),JSON.stringify({passed:true,build:initial.build,build_evidence:buildEvidence,integrated:integrationEvidence,loaded_capabilities:loadedCapabilities,delayed_boot_navigation_preserved:true,lost_committed_response_recovered:lost,pending_receipt_reload_recovered:true,identical_retry_requests:commandRequests.length,visible_named_load:{slot:loadRequest.slot,cancel_preserved_state:true,new_session:true},save_history_roundtrip:true},null,2));
  }finally{if(browser)await browser.close();server.kill();log.end();}
})().catch(e=>{console.error(e);process.exitCode=1;});
