// S06: a real France campaign, ordinary budget/tax controls and named save/load.
// Requires an already-built, committed release binary. No fixture endowments,
// synthetic response bodies or hidden game-state mutations are used here.
const assert=require('node:assert/strict');
const fs=require('node:fs'),path=require('node:path'),net=require('node:net');
const cp=require('node:child_process'),crypto=require('node:crypto');
const {chromium}=require('playwright');
const integrated=require('./ci-integrated.cjs');
const root=path.resolve(__dirname,'../..');
const hash=value=>crypto.createHash('sha256').update(value).digest('hex');
const copy=value=>JSON.parse(JSON.stringify(value));
const isRoute=(response,route,method='GET')=>new URL(response.url()).pathname===route&&response.request().method()===method;
const finite=(value,label)=>assert(Number.isFinite(value),label+' must be a recorded finite number');
function close(actual,expected,label){
  finite(actual,label);finite(expected,label);
  assert(Math.abs(actual-expected)<1e-9,label+': '+actual+' differs from '+expected);
}
async function freePort(){
  const socket=net.createServer();await new Promise(resolve=>socket.listen(0,'127.0.0.1',resolve));
  const port=socket.address().port;await new Promise(resolve=>socket.close(resolve));return port;
}
async function get(page,url,route){
  const response=await page.request.get(url+route);assert(response.ok(),route+' returned '+response.status());return response.json();
}
async function archive(page,url,run,slot){
  const response=await page.request.post(url+'/api/save',{data:{slot}});assert(response.ok(),'Could not capture '+slot);
  const value=JSON.parse(fs.readFileSync(path.join(run,'saves',slot+'.json'),'utf8'));
  delete value.saved_unix;return value;
}
async function currentReading(page){
  // Read existing state only, waiting for the production acceptance guards.
  await page.waitForFunction(()=>cashFlowCurrent()&&CFLOW.data.date===S.date);
  return page.evaluate(()=>JSON.parse(JSON.stringify(CFLOW.data)));
}
async function overview(page){
  if(!await page.locator('#cabinetDrawer').isVisible())await page.locator('[data-drawer="cabinetDrawer"]').click();
  await page.locator('#cab-tab-overview').click();
  await page.locator('#cashFlowTitle').waitFor({state:'visible'});
  assert.equal(await page.locator('#cashFlowTitle').innerText(),'Money and recovery');
  return currentReading(page);
}
async function verifyReading(page,url,reading){
  const state=await get(page,url,'/api/state');
  assert.equal(reading.session_id,state.session_id);assert.equal(reading.nation,'France');assert.equal(reading.date,state.date);
  const native=await get(page,url,'/api/cash-flow?session_id='+encodeURIComponent(state.session_id));
  // The browser observation uses JSON.stringify, which loses the sign of zero.
  // Normalize the native observation through that same serialization only.
  assert.deepEqual(reading,copy(native),'Accepted browser reading must be the native response');
  const country=state.nations.find(n=>n.id==='France');assert(country);
  assert.equal(reading.balances.treasury_bn,country.treasury);assert.equal(reading.balances.debt_bn,country.debt_bn);
  if(!reading.money.available){assert.equal(reading.money.reconciliation,null);return state;}
  const check=row=>{
    assert(Number.isInteger(row.day));assert(Array.isArray(row.entries));
    close(row.closing_treasury_bn-row.opening_treasury_bn,row.cash_change_bn,'Opening/closing cash');
    close(row.closing_debt_bn-row.opening_debt_bn,row.debt_change_bn,'Opening/closing debt');
    close(row.cash_change_bn-row.debt_change_bn,row.net_change_bn,'Net financing');
    for(const entry of row.entries){finite(entry.cash_change_bn,'Entry cash');finite(entry.debt_change_bn,'Entry debt');}
    close(row.entries.reduce((sum,entry)=>sum+entry.cash_change_bn,0),row.cash_change_bn,'Recorded cash posting sum');
    close(row.entries.reduce((sum,entry)=>sum+entry.debt_change_bn,0),row.debt_change_bn,'Recorded debt posting sum');
  };
  check(reading.money.reconciliation);
  for(const day of reading.money.recent_days){assert.equal(day.day,day.reconciliation.day);check(day.reconciliation);}
  close(reading.money.reconciliation.closing_treasury_bn,country.treasury,'Latest cash and actual treasury');
  close(reading.money.reconciliation.closing_debt_bn,country.debt_bn,'Latest debt and actual public debt');
  return state;
}
async function enact(page,url){
  const before=await get(page,url,'/api/state');
  const responsePromise=page.waitForResponse(response=>isRoute(response,'/api/advance','POST'));
  await page.locator('#cabinetEnact').click();const response=await responsePromise;assert(response.ok(),'Cabinet enact failed');
  const payload=response.request().postDataJSON();assert.equal(payload.days,1);assert.equal(payload.session_id,before.session_id);
  await page.waitForFunction(()=>!advancing&&!CAB.busy&&!pendingAdvance);
  const after=await get(page,url,'/api/state');assert.equal(after.session_id,before.session_id);assert.notEqual(after.date,before.date);
  assert.deepEqual((after.log||[]).filter(row=>row.date===before.date&&row.text.startsWith('[rejected]')),[],'An enacted order was refused');
  return {before_date:before.date,after_date:after.date,payload};
}
async function noOverflow(page){
  const dimensions={};
  for(const selector of ['#cabinetDrawer','#left','#cashFlowRoot','.cf-money','.cf-money-decisions']){
    const locator=page.locator(selector);if(!await locator.count())continue;
    const box=await locator.evaluate(element=>({width:element.clientWidth,scroll:element.scrollWidth}));
    assert(box.width>0&&box.scroll<=box.width+1,selector+' has horizontal overflow: '+JSON.stringify(box));dimensions[selector]=box;
  }
  const documentBox=await page.evaluate(()=>({width:document.documentElement.clientWidth,scroll:document.documentElement.scrollWidth}));
  assert(documentBox.scroll<=documentBox.width+1,'The campaign document overflows horizontally');
  assert(!/\bNaN\b|\bundefined\b/.test(await page.locator('#cashFlowRoot').innerText()),'Money screen shows invalid data');
  return {...dimensions,document:documentBox};
}
// Hold exactly one *real* response. Subsequent requests use their real server
// responses immediately, allowing a newer campaign date to overtake this one.
async function holdReading(page){
  let used=false,release,reportReady,reportDone;
  const gate=new Promise(resolve=>{release=resolve;});
  const ready=new Promise(resolve=>{reportReady=resolve;});
  const done=new Promise(resolve=>{reportDone=resolve;});
  const handler=async route=>{
    if(used)return route.continue();used=true;
    try{const response=await route.fetch();reportReady({body:await response.json(),ok:response.ok()});await gate;await route.fulfill({response});}
    finally{reportDone();}
  };
  await page.route('**/api/cash-flow?*',handler);
  return {ready,release,async finish(){release();await done;await page.unroute('**/api/cash-flow?*',handler);}};
}
async function verifyMoneyCss(page,url,revision){
  const name='cash-flow-ui.css',checkout=fs.readFileSync(path.join(root,'spheres-web/ui',name));
  const result=cp.spawnSync('git',['show',revision+':spheres-web/ui/'+name],{cwd:root,windowsHide:true,maxBuffer:1024*1024});
  assert.equal(result.status,0,'Cannot read committed money stylesheet');
  const text=checkout.toString('utf8');assert(Buffer.from(text).equals(checkout),'Money stylesheet must be UTF-8');
  assert(Buffer.from(text.replace(/\r\n/g,'\n')).equals(result.stdout),'Money stylesheet checkout differs from the committed source');
  const response=await page.request.get(url+'/'+name);assert(response.ok());const served=await response.body();
  assert(served.equals(checkout),'Served money stylesheet differs from the compiled checkout');
  return {name,served_sha256:hash(served),checkout_sha256:hash(checkout),committed_sha256:hash(result.stdout),
    newline_normalization:'Only CRLF to LF for checkout/commit comparison; served/checkout comparison is byte-exact'};
}
async function campaigns(page){
  if(await page.locator('#cabinetDrawer').isVisible())await page.locator('#cabinetDrawer [data-close-drawers]').click();
  if(await page.locator('.arc-time-menu').getAttribute('open')===null)await page.locator('.arc-time-menu > summary').click();
  await page.locator('#campaignsBtn').click();await page.locator('#campaignHome').waitFor({state:'visible'});
  await page.locator('#openSavesBtn').click();await page.locator('#savedCampaigns').waitFor({state:'visible'});
}

(async()=>{
  assert(process.env.SPHERES_BINARY,'Set SPHERES_BINARY to the already-built release executable');
  assert.match(process.env.SPHERES_EXPECTED_REVISION||'',/^[a-f0-9]{40}$/,'Set SPHERES_EXPECTED_REVISION to its exact committed source');
  const binary=path.resolve(process.env.SPHERES_BINARY);assert(fs.statSync(binary).isFile());
  const output=path.resolve(process.env.SPHERES_MONEY_OUTPUT||path.join(root,'artifacts/browser-money-ci'));
  fs.mkdirSync(output,{recursive:true});const run=fs.mkdtempSync(path.join(output,'france-'));
  const port=await freePort(),url=`http://127.0.0.1:${port}`;
  const server=cp.spawn(binary,['--port',String(port),'--no-open'],{cwd:run,windowsHide:true,stdio:['ignore','pipe','pipe']});
  const log=fs.createWriteStream(path.join(run,'server.log'));server.stdout.pipe(log);server.stderr.pipe(log);
  let launchError,browser,page,stage='launch';server.on('error',error=>{launchError=error;});
  const evidence={passed:false,run,url,fixture:'Fresh France selected through the normal nation menu; no seeded money or synthetic campaign responses',
    browser_channel:process.env.SPHERES_BROWSER_CHANNEL||'playwright-default',actions:[],screenshots:[],errors:[],requests:[]};
  const write=(name,value)=>fs.writeFileSync(path.join(run,name),JSON.stringify(value,null,2));
  const screenshot=async(name,locator)=>{if(locator)await locator.scrollIntoViewIfNeeded();await page.screenshot({path:path.join(run,name+'.png')});evidence.screenshots.push(name+'.png');};
  try{
    for(let attempt=0;;attempt++){
      if(launchError)throw launchError;if(server.exitCode!==null)throw Error('Disposable server exited with '+server.exitCode);
      try{if((await fetch(url+'/api/state')).ok)break;}catch(_){}
      if(attempt>=200)throw Error('Disposable server failed to start');await new Promise(resolve=>setTimeout(resolve,100));
    }
    browser=await chromium.launch({headless:true,...(process.env.SPHERES_BROWSER_CHANNEL?{channel:process.env.SPHERES_BROWSER_CHANNEL}:{})});
    evidence.browser_version=browser.version();
    page=await browser.newPage({viewport:{width:1440,height:1000},reducedMotion:'reduce'});page.setDefaultTimeout(30000);
    page.on('pageerror',error=>evidence.errors.push(error.message));
    page.on('request',request=>{if(request.method()==='POST'&&/^\/api\/(advance|command|new|save|load)$/.test(new URL(request.url()).pathname))
      evidence.requests.push({route:new URL(request.url()).pathname,payload:request.postDataJSON()});});
    evidence.build=await integrated.verifyBuild({page,url,root,run,binary});
    evidence.money_css=await verifyMoneyCss(page,url,evidence.build.revision);
    stage='France selection';
    await page.goto(url,{waitUntil:'domcontentloaded'});await page.locator('#campaignHome').waitFor({state:'visible'});
    await page.waitForFunction(()=>!!SESSION.live?.session_id);
    await page.locator('#newCampaignBtn').click();await page.locator('#newCampaignPicker').waitFor({state:'visible'});
    await page.locator('#nationPick [aria-label^="France;"]').click();await page.locator('#startBtn').click();
    await page.locator('#app').waitFor({state:'visible'});await page.waitForFunction(()=>!SESSION.busy);
    const initial=await get(page,url,'/api/state');assert.equal(initial.player,'France');assert.equal(initial.simulation_cadence,'daily');
    assert.equal(await page.evaluate(()=>clock.running),false,'The normal fresh campaign must be paused');
    stage='initial pure money reading';
    const initialArchive=await archive(page,url,run,'s06-read-before');
    const empty=await overview(page);await verifyReading(page,url,empty);
    assert.equal(empty.money.available,false);assert.equal(empty.money.reconciliation,null);assert.deepEqual(empty.money.decisions,[]);
    assert.match(await page.locator('#cashFlowMoneyTitle').innerText(),/not available yet/);
    assert.deepEqual(await archive(page,url,run,'s06-read-after'),initialArchive,'Opening and reading money must not create accounts, receipts or orders');
    assert.deepEqual(await get(page,url,'/api/state'),initial,'Reading money must preserve the campaign');
    assert.equal(evidence.requests.filter(request=>request.route==='/api/advance'||request.route==='/api/command').length,0);
    write('initial-money.json',empty);await screenshot('money-initial-desktop',page.locator('#cashFlowTitle'));
    evidence.initial_read={date:empty.date,unavailable:true,archive_pure:true};

    stage='initial budget enactment';
    await page.locator('#cab-tab-budget').click();await page.locator('#cabinet-budget').waitFor({state:'visible'});
    evidence.actions.push(await enact(page,url));
    const budget=await overview(page);await verifyReading(page,url,budget);assert.equal(budget.money.available,true);
    assert(budget.money.reconciliation.entries.some(entry=>entry.id==='budget_settlement'),'A real budget settlement must be recorded');
    write('budget-money.json',budget);

    stage='review and enact tax policy';
    const beforeTaxState=await get(page,url,'/api/state'),beforeTax=beforeTaxState.nations.find(n=>n.id==='France').tax;
    await page.locator('.cf-money-recovery').getByRole('button',{name:'Review taxes',exact:true}).click();
    await page.locator('#cabinet-policy').waitFor({state:'visible'});
    const tax=page.locator('#cabinet-policy .slider[data-kind="tax"] input');
    assert(await tax.evaluate(element=>element===document.activeElement),'Review taxes must focus the actual tax control');
    const startValue=Number(await tax.inputValue()),direction=startValue<=595?'ArrowRight':'ArrowLeft';
    for(let step=0;step<5;step++)await tax.press(direction);
    const approvedTax=Number(await tax.inputValue())/1000;assert.notEqual(approvedTax,beforeTax);
    assert.deepEqual(await get(page,url,'/api/state'),beforeTaxState,'Moving the slider must remain a draft');
    const draftReading=await get(page,url,'/api/cash-flow?session_id='+encodeURIComponent(beforeTaxState.session_id));
    assert.deepEqual(draftReading.money,budget.money,'A draft cannot invent receipts, decisions or future outcomes');
    await screenshot('tax-draft-desktop',page.locator('#cabinet-policy .slider[data-kind="tax"]'));
    evidence.actions.push(await enact(page,url));
    const approved=await overview(page),approvedState=await verifyReading(page,url,approved);
    assert.equal(approvedState.nations.find(n=>n.id==='France').tax,approvedTax);
    const taxDecision=approved.money.decisions.find(decision=>decision.label==='Tax rate reviewed');assert(taxDecision,'The approved tax decision must be retained');
    assert.equal(taxDecision.before_label,`Tax rate ${(beforeTax*100).toFixed(2)}%`);
    assert.equal(taxDecision.after_label,`Tax rate ${(approvedTax*100).toFixed(2)}%`);
    assert.match(taxDecision.note,/observed outcomes, not a forecast/);
    assert.match(taxDecision.note,/effect alone/);assert.match(taxDecision.observed_label,/Through /);
    write('approved-money.json',approved);
    evidence.tax_decision={id:taxDecision.id,before:beforeTax,approved:approvedTax,date:taxDecision.date,draft_pure:true};

    stage='dated details, responsive layout and refresh retention';
    const older=approved.money.recent_days.find(day=>day.day!==approved.money.reconciliation.day&&day.reconciliation.entries.length);
    assert(older,'Two real enacts must leave an earlier selectable paid day');
    const entryKey=`money:entry:${older.day}:${older.reconciliation.entries[0].id}`,decisionKey=`money:decision:${taxDecision.id}`;
    const detail=key=>page.locator('details[data-cash-flow-detail='+JSON.stringify(key)+']');
    evidence.views=[];
    for(const size of [{name:'desktop',width:1440,height:1000},{name:'mobile',width:390,height:844}]){
      await page.setViewportSize({width:size.width,height:size.height});
      await page.locator('#cashFlowDay').selectOption(String(older.day));
      assert.equal(await page.locator('.cf-money .cf-date').innerText(),older.reconciliation.label+' · '+older.reconciliation.period_note);
      for(const key of [entryKey,decisionKey])if(await detail(key).getAttribute('open')===null)await detail(key).locator('summary').click();
      const decisionText=await detail(decisionKey).innerText();assert(decisionText.includes(taxDecision.before_label));assert(decisionText.includes(taxDecision.after_label));
      assert(decisionText.includes(taxDecision.observed_label));assert(decisionText.includes(taxDecision.note));
      const held=await holdReading(page);
      try{
        await page.locator('[data-cash-flow-refresh]').click();const original=await held.ready;assert(original.ok);
        // During the real request, use the summary twice to leave it open and
        // focused. Refresh completion must preserve that actual user focus.
        await detail(entryKey).locator('summary').click();await detail(entryKey).locator('summary').click();
        held.release();await held.finish();await currentReading(page);
      }finally{held.release();}
      assert.equal(await page.locator('#cashFlowDay').inputValue(),String(older.day));
      for(const key of [entryKey,decisionKey])assert.notEqual(await detail(key).getAttribute('open'),null,'Refresh lost '+key);
      assert(await detail(entryKey).locator('summary').evaluate(element=>element===document.activeElement),'Refresh lost the user-focused receipt summary');
      const dimensions=await noOverflow(page);
      await screenshot('money-'+size.name,page.locator('#cashFlowTitle'));
      await screenshot('money-entry-'+size.name,detail(entryKey));
      await screenshot('money-decision-'+size.name,detail(decisionKey));
      await screenshot('money-recovery-'+size.name,page.locator('.cf-money-recovery'));
      evidence.views.push({...size,dimensions,selected_day:older.day,entry_open:true,decision_open:true,focus_retained:true});
    }
    assert.deepEqual(await get(page,url,'/api/state'),approvedState,'Details, layout and refresh must not change campaign state');

    stage='old dated response overtaken by a real next day';
    await page.setViewportSize({width:1440,height:1000});await page.locator('#cashFlowDay').selectOption('latest');
    const held=await holdReading(page);let current;
    try{
      await page.locator('[data-cash-flow-refresh]').click();const old=await held.ready;assert(old.ok);
      evidence.actions.push(await enact(page,url));current=await currentReading(page);
      assert.notEqual(current.date,old.body.date,'The second reading must come from an actual later day');
      held.release();await held.finish();
      // A read-only browser turn lets the held fetch completion settle.
      await page.evaluate(()=>new Promise(resolve=>requestAnimationFrame(()=>requestAnimationFrame(resolve))));
      assert.deepEqual(await currentReading(page),current,'The old native response replaced the later accepted reading');
      evidence.stale_response={held_date:old.body.date,accepted_date:current.date,old_response_discarded:true};
    }finally{held.release();}
    const beforeLoad=await verifyReading(page,url,current);write('before-load-money.json',current);

    stage='visible named save, cancel and confirmed load';
    const slot='s06-france-money';await campaigns(page);await page.locator('#saveName').fill(slot);
    const savedPromise=page.waitForResponse(response=>isRoute(response,'/api/save','POST'));
    await page.locator('#saveNamedBtn').click();const saved=await savedPromise;assert(saved.ok());assert.equal(saved.request().postDataJSON().slot,slot);
    await page.locator('#saveSlots option[value="'+slot+'"]').waitFor({state:'attached'});await page.locator('#saveSlots').selectOption(slot);
    await screenshot('named-save-desktop',page.locator('#saveSlots'));
    await page.locator('#loadBtn').click();await page.locator('#campaignConfirmDialog').waitFor({state:'visible'});
    assert((await page.locator('#campaignConfirmMessage').innerText()).includes(slot));
    assert(await page.locator('#campaignConfirmCancel').evaluate(element=>element===document.activeElement));
    await page.locator('#campaignConfirmCancel').click();assert.deepEqual(await get(page,url,'/api/state'),beforeLoad);
    assert.equal(await page.locator('#saveSlots').inputValue(),slot);
    await page.locator('#loadBtn').click();await page.locator('#campaignConfirmDialog').waitFor({state:'visible'});
    const loadPromise=page.waitForResponse(response=>isRoute(response,'/api/load','POST'));
    await page.locator('#campaignConfirmAccept').click();const loadedResponse=await loadPromise;assert(loadedResponse.ok());
    assert.equal(loadedResponse.request().postDataJSON().slot,slot);assert.equal(loadedResponse.request().postDataJSON().backup,false);
    await page.locator('#app').waitFor({state:'visible'});await page.waitForFunction(()=>!SESSION.busy);
    const loadedReading=await overview(page),loadedState=await verifyReading(page,url,loadedReading);
    assert.notEqual(loadedState.session_id,beforeLoad.session_id);assert.equal(loadedState.date,beforeLoad.date);
    assert.deepEqual(loadedReading.money,current.money,'Named load must retain the exact dated money and decision history');
    assert.deepEqual(loadedReading.balances,current.balances);write('loaded-money.json',loadedReading);
    await screenshot('money-loaded-desktop',page.locator('#cashFlowTitle'));
    stage='browser reload and normal Continue';
    await page.reload();await page.locator('#continueBtn').waitFor({state:'visible'});await page.locator('#continueBtn').click();
    await page.locator('#app').waitFor({state:'visible'});await page.waitForFunction(()=>!SESSION.busy);
    const resumed=await overview(page);assert.deepEqual(resumed.money,current.money);await verifyReading(page,url,resumed);
    assert.equal(resumed.session_id,loadedState.session_id,'Continue must keep the loaded session');
    const namedFile=fs.readFileSync(path.join(run,'saves',slot+'.json'));
    evidence.save_load={slot,cancel_pure:true,new_session:true,date:loadedState.date,exact_money:true,browser_reload_continue:true,archive_sha256:hash(namedFile)};
    assert.deepEqual(evidence.errors,[]);assert.equal(evidence.requests.filter(request=>request.route==='/api/advance').length,3,'Only the three visible enacts may advance time');
    assert.equal(evidence.requests.filter(request=>request.route==='/api/command').length,0,'The journey uses the reviewed Cabinet advance lane');
    evidence.passed=true;evidence.completed_at=new Date().toISOString();write('result.json',evidence);
    console.log('S06 money journey passed: '+path.join(run,'result.json'));
  }catch(error){
    evidence.failed_stage=stage;evidence.failure=error.stack||String(error);write('result.json',evidence);
    if(page)try{await page.screenshot({path:path.join(run,'failure.png')});fs.writeFileSync(path.join(run,'failure.html'),await page.content());}catch(_){}
    console.error('S06 money evidence: '+path.join(run,'result.json'));throw error;
  }finally{if(browser)await browser.close();server.kill();log.end();}
})().catch(error=>{console.error(error);process.exitCode=1;});
