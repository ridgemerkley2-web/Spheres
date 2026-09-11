// S07: real fresh France/Tonga campaigns, ordinary visible controls throughout.
// Requests/evaluate are read-only observations except save snapshots for purity.
// No injected money, materials, completed capacity, workers or campaign responses.
const assert=require('node:assert/strict'),fs=require('node:fs'),path=require('node:path');
const cp=require('node:child_process'),crypto=require('node:crypto'),net=require('node:net');
const {chromium}=require('playwright');
const integrated=require('./ci-integrated.cjs');
const root=path.resolve(__dirname,'../..');
const hash=b=>crypto.createHash('sha256').update(b).digest('hex');
const copy=value=>JSON.parse(JSON.stringify(value));
const withoutSession=value=>{const next=copy(value);delete next.session_id;return next;};
const routeIs=(response,route)=>new URL(response.url()).pathname===route&&response.request().method()==='POST';
async function withinPanel(page,selector,label){
  const box=await page.locator(selector).evaluate(e=>({width:e.clientWidth,scroll:e.scrollWidth}));
  assert(box.width>0&&box.scroll<=box.width+1,label+' overflows: '+JSON.stringify(box));
  assert(!/\bNaN\b|\bundefined\b/.test(await page.locator(selector).innerText()));
}
async function port(){const s=net.createServer();await new Promise(r=>s.listen(0,'127.0.0.1',r));const p=s.address().port;await new Promise(r=>s.close(r));return p;}
async function read(page,url,route){const r=await page.request.get(url+route);assert(r.ok(),route+': '+r.status());return r.json();}
async function idle(page){await page.waitForFunction(()=>!advancing&&!pendingAdvance&&!PROD.busy&&!SESSION.busy&&!COMMAND_CHANNEL.busy&&!COMMAND_CHANNEL.pending);}
async function construction(page){
  if(await page.locator('#cabinetDrawer').isVisible())await page.locator('#cabinetDrawer [data-close-drawers]').click();
  if(!await page.locator('#productionPanel').isVisible())await page.locator('#productionDockBtn').click();
  await page.waitForFunction(()=>PROD.open&&PROD.mode==='build'&&PROD.data&&!PROD.loading&&!PROD.stale&&!PROD.busy&&!PROD.error);
}
async function closePanels(page){
  if(await page.locator('#productionPanel').isVisible())await page.locator('#productionClose').click();
  if(await page.locator('#cabinetDrawer').isVisible())await page.locator('#cabinetDrawer [data-close-drawers]').click();
}
async function command(page,action,kind){
  const wait=page.waitForResponse(r=>routeIs(r,'/api/command'));await action();const response=await wait;
  assert(response.ok());const result=await response.json();assert.deepEqual(result.errors||[],[]);assert(!result.command_pending);
  const payload=response.request().postDataJSON();assert(payload.commands.some(c=>c.kind===kind));await idle(page);await construction(page);return payload;
}
async function budget(page,millions){
  await construction(page);await page.locator('#constructionDailyBudget').fill(String(millions));
  return command(page,()=>page.locator('#constructionBudgetForm button[type="submit"]').click(),'construction_budget');
}
async function day(page,url){
  await closePanels(page);const wait=page.waitForResponse(r=>routeIs(r,'/api/advance'));
  await page.locator('#stepBtn').click();const response=await wait;assert(response.ok());
  assert.equal(response.request().postDataJSON().days,1);const result=await response.json();await idle(page);
  assert(!result.advance_pending);return read(page,url,'/api/production');
}
async function preview(page,url,kind,district,workshop=false){
  await construction(page);await page.locator('[data-prod-new]').click();
  if(workshop){
    await page.locator('[data-construction-small]').click();await page.locator('#constructionWorkshopProvince').selectOption(district);
    await page.locator('#constructionWorkshopForm button').click();
    await page.locator('[data-construction-workshop="0"]').waitFor({state:'visible'});
    await page.locator('[data-construction-workshop="0"]').click();
  }else{
    await page.locator('[data-prod-kind="'+kind+'"]').click();
    await page.locator('[data-prod-province="'+district+'"]').click();
  }
  await page.waitForFunction(()=>constructionPreviewCurrent()&&PROD.preview.can_start===true);
  return page.evaluate(()=>JSON.parse(JSON.stringify(PROD.preview)));
}
async function add(page,url,kind,district,workshop=false){
  const before=await read(page,url,'/api/state'),beforeBoard=await read(page,url,'/api/production');
  const quote=await preview(page,url,kind,district,workshop);
  assert.deepEqual(await read(page,url,'/api/state'),before,'Reviewing a construction quote changed the campaign');
  assert.deepEqual(await read(page,url,'/api/production'),beforeBoard,'A preview created work or receipts');
  const payload=await command(page,()=>page.locator('[data-construction-confirm]').click(),workshop?'start_industry_module':'start_project');
  const board=await read(page,url,'/api/production');const previous=new Set(beforeBoard.queue.map(p=>p.id));
  const added=board.queue.filter(p=>!previous.has(p.id));assert.equal(added.length,1);
  assert.equal(added[0].kind,kind);assert.equal(added[0].province.id,district);assert.deepEqual(added[0].requirements,[]);
  assert.equal(added[0].finance.spent_bn,0,'Starting construction charged undelivered work');
  assert(Math.abs(added[0].finance.cost_bn-quote.cost_bn)<1e-10,'Approved and contracted cost differ');
  return {id:added[0].id,quote,payload};
}
async function campaigns(page){
  await closePanels(page);if(await page.locator('.arc-time-menu').getAttribute('open')===null)await page.locator('.arc-time-menu > summary').click();
  await page.locator('#campaignsBtn').click();await page.locator('#campaignHome').waitFor({state:'visible'});
  await page.locator('#openSavesBtn').click();await page.locator('#savedCampaigns').waitFor({state:'visible'});
}
async function saveLoad(page,url,slot){
  const state=await read(page,url,'/api/state'),board=await read(page,url,'/api/production');
  await campaigns(page);await page.locator('#saveName').fill(slot);
  const save=page.waitForResponse(r=>routeIs(r,'/api/save'));await page.locator('#saveNamedBtn').click();assert((await save).ok());
  await page.locator('#saveSlots option[value="'+slot+'"]').waitFor({state:'attached'});await page.locator('#saveSlots').selectOption(slot);
  await page.locator('#loadBtn').click();await page.locator('#campaignConfirmCancel').click();
  assert.deepEqual(await read(page,url,'/api/state'),state,'Cancelling load changed the campaign');
  await page.locator('#loadBtn').click();const load=page.waitForResponse(r=>routeIs(r,'/api/load'));
  await page.locator('#campaignConfirmAccept').click();assert((await load).ok());
  await page.locator('#app').waitFor({state:'visible'});await idle(page);
  const after=await read(page,url,'/api/state');assert.notEqual(after.session_id,state.session_id);assert.equal(after.date,state.date);
  assert.deepEqual(withoutSession(await read(page,url,'/api/production')),withoutSession(board),'Named load changed construction progress, payments or outcomes');
  return {slot,date:state.date,new_session:true,exact_construction:true,cancel_pure:true};
}
async function assets(page,url,revision){
  const result={};for(const name of ['industry-ui.js','industry-ui.css']){
    const checkout=fs.readFileSync(path.join(root,'spheres-web/ui',name));
    const committed=cp.spawnSync('git',['show',revision+':spheres-web/ui/'+name],{cwd:root,windowsHide:true,maxBuffer:4*1024*1024});assert.equal(committed.status,0);
    assert(Buffer.from(checkout.toString('utf8').replace(/\r\n/g,'\n')).equals(committed.stdout));
    const response=await page.request.get(url+'/'+name);assert(response.ok());const served=await response.body();assert(served.equals(checkout));
    result[name]={served_sha256:hash(served),committed_sha256:hash(committed.stdout)};
  }return result;
}
async function journey(nation,binary,output){
  const run=fs.mkdtempSync(path.join(output,nation.toLowerCase()+'-')),url='http://127.0.0.1:'+await port();
  const server=cp.spawn(binary,['--port',new URL(url).port,'--no-open'],{cwd:run,windowsHide:true,stdio:['ignore','pipe','pipe']});
  const log=fs.createWriteStream(path.join(run,'server.log'));server.stdout.pipe(log);server.stderr.pipe(log);
  let browser,page,stage='launch',launchError;server.on('error',e=>launchError=e);
  const evidence={passed:false,nation,run,url,actions:[],screenshots:[],errors:[],advance_count:0,commands:[],observations:[],fixture:'Fresh campaign through the normal nation selector; no seeded funds, materials, workforce, completed capacity or synthetic responses'};
  const write=(name,value)=>fs.writeFileSync(path.join(run,name),JSON.stringify(value,null,2)+'\n');
  const shot=async(name,selector)=>{if(selector)await page.locator(selector).scrollIntoViewIfNeeded();await page.screenshot({path:path.join(run,name+'.png')});evidence.screenshots.push(name+'.png');};
  let totalConstructionPaid=0;
  const takeDay=async()=>{const board=await day(page,url);const paid=board.construction_budget.spent_today_bn;
    assert(Number.isFinite(paid)&&paid>=0);totalConstructionPaid+=paid;return board;};
  try{
    for(let attempt=0;;attempt++){
      if(launchError)throw launchError;if(server.exitCode!==null)throw Error('Server exited '+server.exitCode);
      try{if((await fetch(url+'/api/state')).ok)break;}catch(_){}
      if(attempt>300)throw Error('Server start timeout');await new Promise(r=>setTimeout(r,100));
    }
    browser=await chromium.launch({headless:true,channel:process.env.SPHERES_BROWSER_CHANNEL||'chrome'});evidence.browser_version=browser.version();
    page=await browser.newPage({viewport:{width:1440,height:1000},reducedMotion:'reduce'});page.setDefaultTimeout(45000);
    page.on('pageerror',e=>evidence.errors.push(e.message));page.on('request',r=>{
      if(r.method()!=='POST')return;const route=new URL(r.url()).pathname;
      if(route==='/api/advance')evidence.advance_count++;
      if(route==='/api/command')evidence.commands.push(r.postDataJSON());
    });
    evidence.build=await integrated.verifyBuild({page,url,root,run,binary});evidence.assets=await assets(page,url,evidence.build.revision);
    stage='country selection';await page.goto(url,{waitUntil:'domcontentloaded'});
    await page.locator('#campaignHome').waitFor({state:'visible'});await page.waitForFunction(()=>!!SESSION.live?.session_id);
    await page.locator('#newCampaignBtn').click();await page.locator('#nationPick [aria-label^="'+nation+';"]').click();await page.locator('#startBtn').click();
    await page.locator('#app').waitFor({state:'visible'});await idle(page);
    const opening=await read(page,url,'/api/state');assert.equal(opening.player,nation);assert.equal(opening.simulation_cadence,'daily');
    await construction(page);const initial=await read(page,url,'/api/production');write('initial-construction.json',initial);
    const district=(nation==='France'?initial.provinces.find(p=>/le-de-france/i.test(p.id)):null)?.id||initial.provinces[0].id;
    evidence.district=district;stage='funding and previews';
    const funded=initial.construction_budget.daily_budget_bn*4;
    assert(funded>0&&Number.isFinite(funded));await budget(page,funded*1000);
    const workshop=await add(page,url,'starter_industry',district,true);evidence.actions.push(workshop);
    let ordinary;if(nation==='France'){ordinary=await add(page,url,'office_district',district);evidence.actions.push(ordinary);}
    // A province cannot queue the same building kind twice. The cancelled
    // warehouse is a distinct financially funded job, including for Tonga.
    const cancelled=await add(page,url,'warehouse',district);
    evidence.actions.push({cancel_candidate:cancelled});
    await command(page,()=>page.locator('[data-prod-priority="high"][data-prod-id="'+cancelled.id+'"]').click(),'set_project_priority');
    await shot('funded-queue','#productionBody');
    stage='paid work and pause';await takeDay();await construction(page);
    // A tiny country may only fund one site per day. Give each reviewed job
    // its turn through actual priority controls before testing paid cancellation.
    await command(page,()=>page.locator('[data-prod-priority="low"][data-prod-id="'+cancelled.id+'"]').click(),'set_project_priority');
    await command(page,()=>page.locator('[data-prod-priority="high"][data-prod-id="'+workshop.id+'"]').click(),'set_project_priority');
    await takeDay();
    const worked=await read(page,url,'/api/production');assert(worked.queue.find(p=>p.id===workshop.id).finance.spent_bn>0);
    const cancelledPaid=worked.queue.find(p=>p.id===cancelled.id).finance.spent_bn;assert(cancelledPaid>0);
    await budget(page,0);const paused=await read(page,url,'/api/production');await takeDay();
    const pausedNext=await read(page,url,'/api/production');
    for(const p of paused.queue){const after=pausedNext.queue.find(q=>q.id===p.id);assert.equal(after.progress,p.progress);assert.equal(after.finance.spent_bn,p.finance.spent_bn);}
    assert.equal(pausedNext.construction_budget.spent_today_bn,0,'Paused work was charged');
    evidence.pause={date:(await read(page,url,'/api/state')).date,progress_retained:true,no_spending:true};
    await construction(page);await shot('paused-queue','#productionBody');
    stage='review cancellation';const beforeCancel=await read(page,url,'/api/state');
    await page.locator('[data-prod-cancel="'+cancelled.id+'"]').click();await page.locator('#campaignConfirmDialog').waitFor({state:'visible'});
    assert.match(await page.locator('#campaignConfirmMessage').innerText(),/not refunded/);await page.locator('#campaignConfirmCancel').click();
    assert.deepEqual(await read(page,url,'/api/state'),beforeCancel);
    await page.locator('[data-prod-cancel="'+cancelled.id+'"]').click();
    await command(page,()=>page.locator('#campaignConfirmAccept').click(),'cancel_project');
    const afterCancel=await read(page,url,'/api/state');
    const balances=s=>{const n=s.nations.find(n=>n.id===nation);return [n.treasury,n.debt_bn];};
    assert.deepEqual(balances(afterCancel),balances(beforeCancel),'Paid construction cancellation invented a cash refund');
    assert(!(await read(page,url,'/api/production')).queue.some(p=>p.id===cancelled.id));
    evidence.cancellation={id:cancelled.id,spent_bn:cancelledPaid,cancel_review_pure:true,no_refund:true};
    await budget(page,funded*1000);stage='paid save and reload';evidence.paid_save=await saveLoad(page,url,'s07-'+nation.toLowerCase()+'-paid');
    stage='finish through visible daily controls';let board;
    for(let days=0;days<800;days++){
      board=await takeDay();
      if(days%15===0||!board.queue.length){
        evidence.observations.push({date:(await read(page,url,'/api/state')).date,queue:board.queue.map(p=>({id:p.id,progress:p.progress,spent_bn:p.finance.spent_bn,status:p.status,reason:p.reason})),spent_ytd_bn:board.construction_budget.spent_ytd_bn});
        if(!board.queue.some(p=>p.id===workshop.id||p.id===ordinary?.id))break;
        if(days%60===0)console.log(nation+': '+evidence.observations.at(-1).date+' '+JSON.stringify(evidence.observations.at(-1).queue));
      }
      assert(days<799,'Construction did not finish within the declared 800-day bound');
    }
    board=await read(page,url,'/api/production');assert(!board.queue.some(p=>p.id===workshop.id||p.id===ordinary?.id));
    const reviewedTotal=workshop.quote.cost_bn+(ordinary?.quote.cost_bn||0)+cancelledPaid;
    assert(Math.abs(totalConstructionPaid-reviewedTotal)<1e-9,'Actual daily construction charges differ from completed reviewed contracts plus paid cancelled work');
    evidence.construction_accounting={actual_paid_bn:totalConstructionPaid,reviewed_completed_plus_cancelled_work_bn:reviewedTotal,exact_within_bn:1e-9};
    evidence.completed_date=(await read(page,url,'/api/state')).date;write('completed-construction.json',board);
    // The connected outcome assertions below use the same server reading that
    // the visible facility view consumes; no capacity is converted into output.
    await construction(page);await page.locator('[data-prod-built]').click();await shot('completed-construction','#productionBody');
    stage='connected operating outcome';
    const outcomes=await page.locator('[data-construction-outcome]').all();let outcome;
    for(const button of outcomes){const action=JSON.parse(await button.getAttribute('data-construction-outcome'));
      if(action.district===district&&action.kind==='starter_industry'){outcome=button;break;}}
    assert(outcome,'The completed workshop needs a direct operating-outcome link');await outcome.click();
    await page.waitForFunction(()=>industryCurrent()&&IDESK.data.date===S.date);
    const native=await read(page,url,'/api/industry?session_id='+encodeURIComponent((await read(page,url,'/api/state')).session_id));
    const accepted=await page.evaluate(()=>JSON.parse(JSON.stringify(IDESK.data)));assert.deepEqual(accepted,copy(native));write('completed-industry.json',native);
    const site=native.sites.find(s=>s.district===district&&s.kind==='starter_industry');assert(site);
    assert.equal(site.lifecycle.installed_capacity,workshop.quote.capacity_micros/1e6);
    const op=site.lifecycle.operation;assert(op,'Completed paid work must reach a dated operating attempt');
    assert.equal(op.owner_verified,true);assert(Number.isFinite(op.output)&&op.output>=0);assert(Number.isFinite(op.cash_spent_bn)&&op.cash_spent_bn>=0);
    assert(Number.isFinite(op.used_workers)&&op.used_workers>=0);assert(op.used_workers<=op.required_workers+1e-8);
    if(op.assigned_workers!==null)assert(op.used_workers<=op.assigned_workers+1e-8);
    if(op.output===0)assert(op.reason,'A blocked completed site needs a reason');
    const siteId='site:'+district+':starter_industry',siteSelector='[data-industry-site="'+siteId+'"]';
    assert(await page.locator(siteSelector).evaluate(e=>e===document.activeElement),'Outcome navigation must focus the exact completed facility');
    assert((await page.locator(siteSelector+' .industry-operation').innerText()).includes(op.date));
    const details=page.locator('details[data-industry-detail="'+siteId+':requirements"]');await details.locator('summary').click();
    evidence.operating={site:siteId,operation:op,staffing:site.lifecycle.staffing,readiness:site.lifecycle.readiness,exact_response:true,matched_focus:true};
    for(const size of [{width:1440,height:1000,name:'desktop'},{width:390,height:844,name:'mobile'}]){
      await page.setViewportSize({width:size.width,height:size.height});await withinPanel(page,'#industryRoot','Industry '+size.name);
      await withinPanel(page,siteSelector,'Facility '+size.name);await shot('operating-'+size.name,siteSelector);
      await shot('requirements-'+size.name,'details[data-industry-detail="'+siteId+':requirements"]');
    }
    await page.setViewportSize({width:1440,height:1000});
    stage='completed save and Continue';evidence.completed_save=await saveLoad(page,url,'s07-'+nation.toLowerCase()+'-complete');
    const loadedSession=(await read(page,url,'/api/state')).session_id;
    assert.deepEqual(withoutSession(await read(page,url,'/api/industry?session_id='+encodeURIComponent(loadedSession))),withoutSession(native),'Completed named load changed operating receipts or staffing');
    evidence.completed_save.exact_industry=true;
    await page.reload();await page.locator('#continueBtn').click();await page.locator('#app').waitFor({state:'visible'});await idle(page);
    assert.deepEqual(withoutSession(await read(page,url,'/api/production')),withoutSession(board),'Continue changed completed assets');
    assert.deepEqual(withoutSession(await read(page,url,'/api/industry?session_id='+encodeURIComponent(loadedSession))),withoutSession(native),'Continue changed operating receipts');
    assert.deepEqual(evidence.errors,[]);evidence.passed=true;evidence.finished_utc=new Date().toISOString();write('result.json',evidence);
    console.log('S07 '+nation+' journey passed: '+path.join(run,'result.json'));return {nation,result:path.join(run,'result.json'),sha256:hash(fs.readFileSync(path.join(run,'result.json')))};
  }catch(error){evidence.failed_stage=stage;evidence.failure=error.stack||String(error);write('result.json',evidence);
    if(page)try{await page.screenshot({path:path.join(run,'failure.png')});fs.writeFileSync(path.join(run,'failure.html'),await page.content());}catch(_){}
    console.error('S07 failed evidence: '+path.join(run,'result.json'));throw error;
  }finally{if(browser)await browser.close();server.kill();log.end();}
}
(async()=>{
  assert(process.env.SPHERES_BINARY,'Set SPHERES_BINARY');assert.match(process.env.SPHERES_EXPECTED_REVISION||'',/^[a-f0-9]{40}$/);
  const binary=path.resolve(process.env.SPHERES_BINARY),output=path.resolve(process.env.SPHERES_CONSTRUCTION_OUTPUT||'artifacts/browser-construction-ci');fs.mkdirSync(output,{recursive:true});
  const results=[];for(const nation of (process.env.SPHERES_CONSTRUCTION_NATION?[process.env.SPHERES_CONSTRUCTION_NATION]:['France','Tonga']))results.push(await journey(nation,binary,output));
  fs.writeFileSync(path.join(output,'latest-results.json'),JSON.stringify(results,null,2)+'\n');
})().catch(error=>{console.error(error);process.exitCode=1;});
