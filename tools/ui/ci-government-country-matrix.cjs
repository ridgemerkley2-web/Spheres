// S10.b: eight independent, ordinary CP1 starts. This is no-time browser
// coverage, not a long-run campaign or Russia-activation qualification.
const assert=require('node:assert/strict'),fs=require('node:fs'),path=require('node:path');
const cp=require('node:child_process'),net=require('node:net'),crypto=require('node:crypto');
const {chromium}=require('playwright'),integrated=require('./ci-integrated.cjs'),audit=require('./supplier-archive-audit.cjs');
const root=path.resolve(__dirname,'../..'),copy=value=>JSON.parse(JSON.stringify(value));
const hash=bytes=>crypto.createHash('sha256').update(bytes).digest('hex'),quoted=value=>JSON.stringify(String(value));
const CASES=[
  {nation:'France',name:'France',electoral:true},
  {nation:'Japan',name:'Japan',electoral:true},
  {nation:'India',name:'India',electoral:true},
  {nation:'Brazil',name:'Brazil',electoral:true},
  {nation:'SouthAfrica',name:'South Africa',electoral:false},
  {nation:'Tonga',name:'Tonga',electoral:false},
  {nation:'SaudiArabia',name:'Saudi Arabia',electoral:false},
  {nation:'USSR',name:'Soviet Union',electoral:false}
];
const AUDIT_SLOTS=new Set(['s10b-before-views','s10b-after-cancel','s10b-after-action','s10b-after-load-continue']);
const post=(response,endpoint)=>new URL(response.url()).pathname===endpoint&&response.request().method()==='POST';
const write=(file,value)=>fs.writeFileSync(file,JSON.stringify(value,null,2)+'\n');
async function freePort(){const s=net.createServer();await new Promise(resolve=>s.listen(0,'127.0.0.1',resolve));const port=s.address().port;await new Promise(resolve=>s.close(resolve));return port;}
async function get(page,url,endpoint){const r=await page.request.get(url+endpoint);try{assert(r.ok(),endpoint+' returned '+r.status());return await r.json();}finally{await r.dispose();}}
async function idle(page){await page.waitForFunction(()=>!SESSION.busy&&!COMMAND_CHANNEL.busy&&!COMMAND_CHANNEL.pending&&!advancing&&!pendingAdvance&&!AGENCY.busy&&!gov.busy);}
async function withinPanel(page,selector,label){
  const box=await page.locator(selector).evaluate(e=>({scroll:e.scrollWidth,width:e.clientWidth}));
  assert(box.width>0&&box.scroll<=box.width+1,label+' overflows horizontally: '+JSON.stringify(box));
  assert(!/\bNaN\b|\bundefined\b/.test(await page.locator(selector).innerText()),label+' has an invalid label');
}
async function capture(page,url,run,nation,slot){
  assert(AUDIT_SLOTS.has(slot),'Only fixed audit-only names may leave the save picker');
  const base=fs.realpathSync(run),saves=path.join(base,'saves'),source=path.join(saves,slot+'.json');
  assert.equal(fs.realpathSync(saves),saves);assert(!fs.existsSync(source));
  const r=await page.request.post(url+'/api/save',{data:{slot}});
  try{assert(r.ok(),'Native audit save failed');await r.body();}finally{await r.dispose();}
  assert(fs.lstatSync(source).isFile()&&!fs.lstatSync(source).isSymbolicLink());assert.equal(fs.realpathSync(source),source);
  const directory=path.join(base,'audit-captures');if(!fs.existsSync(directory))fs.mkdirSync(directory);
  assert.equal(fs.realpathSync(directory),directory);assert(!fs.lstatSync(directory).isSymbolicLink());
  const target=path.join(fs.mkdtempSync(path.join(directory,slot+'-')),slot+'.json');assert(!fs.existsSync(target));
  fs.renameSync(source,target);return audit.inspect(target,nation);
}
// These are deliberately fresh saves, with a hard size bound. The generic
// audit worker still compares every world field; this small projection only
// lets the browser assertion check the native financial/government outcomes.
function politicalFacts(archive,nation){
  assert(fs.statSync(archive.input.path).size<=32*1024*1024,'Fresh matrix archive exceeds the declared 32 MiB projection bound');
  let world=JSON.parse(fs.readFileSync(archive.input.path,'utf8').replace(/^\uFEFF/,''));
  while(world&&world.world&&typeof world.world==='object')world=world.world;
  const ns=world.nations.filter(n=>n.id===nation),gs=world.governments.states.filter(g=>g.nation===nation);
  assert.equal(ns.length,1);assert.equal(gs.length,1);const n=ns[0];
  return {nation:Object.fromEntries(['id','political_capital','treasury_bn','debt_bn','debt_gdp','gdp','stability','authoritarianism','separatism'].map(k=>[k,n[k]])),government:copy(gs[0])};
}
const parties=board=>board.groups.flatMap(group=>group.parties);
function compareBoardNative(board,facts){
  assert.equal(board.political_capital,facts.nation.political_capital);
  assert.deepEqual(parties(board).filter(p=>p.in_government).map(p=>p.id).sort(),[...facts.government.coalition].sort());
  for(const pillar of board.pillars){
    const found=facts.government.pillars.find(([key])=>key.toLowerCase()===pillar.key);
    assert(found,'Native pillar missing: '+pillar.key);assert.equal(pillar.loyalty,found[1]);
  }
}
function quotedReading(key,board,facts){
  const n=facts.nation,g=facts.government,pc=x=>x.toFixed(1),pct=x=>(x*100).toFixed(1)+'%',money=x=>'$'+(x*1000).toFixed(2)+' million';
  if(key==='political_capital')return pc(n.political_capital);
  if(key==='treasury')return money(n.treasury_bn);
  if(key==='debt')return Number.isFinite(n.debt_bn)&&Number.isFinite(n.treasury_bn)?money(n.debt_bn):pct(n.debt_gdp);
  if(key==='cabinet'){const names=new Map(parties(board).map(p=>[p.id,p.name]));return g.coalition.map(id=>names.get(id)||id).join(' + ')||'No seated cabinet';}
  if(key==='seats')return pct(board.government_seats);
  if(key==='strain')return pc(board.strain);
  if(key==='upkeep')return pc(board.upkeep)+' PC / month';
  if(key==='composition'){
    const detail=board.briefing.stats.find(row=>row.key==='upkeep')?.detail||'';
    const found=detail.match(/standing target by ([+-]\d+\.\d) points/);assert(found,'Native composition reading is missing');return found[1];
  }
  if(key==='coup_pressure')return pc(g.coup_pressure);
  if(key.startsWith('loyalty:')){const found=g.pillars.find(([id])=>id.toLowerCase()===key.slice(8));assert(found);return pct(found[1]);}
  assert.fail('Unmapped native effect in this bounded invite/secure-pillar case: '+key);
}
function verifyOutcome(quote,beforeBoard,afterBoard,before,after){
  compareBoardNative(beforeBoard,before);compareBoardNative(afterBoard,after);
  assert(Math.abs((before.nation.political_capital-after.nation.political_capital)-quote.price_pc)<1e-9,'Native political cost differs from review');
  const balance=n=>Number.isFinite(n.treasury_bn)&&Number.isFinite(n.debt_bn)?n.treasury_bn-n.debt_bn:-n.debt_gdp*n.gdp;
  assert(Math.abs(Math.max(0,balance(before.nation)-balance(after.nation))-quote.money_cost_bn)<1e-9,'Native financial cost differs from review');
  assert(quote.changes.length>1,'The chosen decision must change more than its political cost');
  for(const row of quote.changes){
    assert.equal(quotedReading(row.key,beforeBoard,before),row.before,'Before reading: '+row.key);
    assert.equal(quotedReading(row.key,afterBoard,after),row.after,'After reading: '+row.key);
  }
  if(quote.command.kind==='invite_to_government'){
    assert(!before.government.coalition.includes(quote.command.party));
    assert.deepEqual(after.government.coalition,[...before.government.coalition,quote.command.party]);
    assert(quote.changes.some(row=>row.key==='cabinet'));assert.equal(quote.money_cost_bn,0);
  }else{
    assert.equal(quote.command.kind,'secure_pillar');
    const key=quote.command.pillar,old=before.government.pillars.find(([id])=>id.toLowerCase()===key),now=after.government.pillars.find(([id])=>id.toLowerCase()===key);
    assert(now[1]>old[1]);assert(quote.changes.some(row=>row.key==='loyalty:'+key));assert(quote.money_cost_bn>0);
  }
  assert.deepEqual(afterBoard.leader,beforeBoard.leader,'A coalition invitation or loyalty payment must not replace the recorded incumbent');
}
async function government(page,nation,tab='overview'){
  if(!await page.locator('#govScreen').isVisible())await page.locator('#govBtn').click();
  await page.waitForFunction(()=>gov.open&&gov.data&&gov.dataState===S&&!gov.data.error);
  if(await page.locator('#govPick').inputValue()!==nation)await page.locator('#govPick').selectOption(nation);
  await page.waitForFunction(id=>gov.nation===id&&gov.data?.nation===id&&gov.dataState===S&&!gov.data.error,nation);
  await page.locator('#gov-tab-'+tab).click();
  return page.evaluate(()=>JSON.parse(JSON.stringify(gov.data)));
}
async function review(page,nation,command){
  const board=await government(page,nation,'decisions'),index=board.actions.findIndex(a=>JSON.stringify(a.command)===JSON.stringify(command));
  assert(index>=0&&!board.actions[index].refusal,'The selected native action is not available');
  const response=page.waitForResponse(r=>post(r,'/api/government/preview'));
  await page.locator('#govScreen [data-gov-review='+quoted(index)+']').click();
  const received=await response;assert(received.ok());const quote=await received.json();
  await page.waitForFunction(()=>gov.review&&!gov.review.loading);assert.equal(quote.valid,true);
  assert.equal(quote.nation,nation);assert.deepEqual(quote.command,command);assert(quote.review_token);
  assert.equal(await page.locator('#govReview [data-gov-confirm]').isEnabled(),true);return quote;
}
async function campaigns(page){
  if(await page.locator('#govScreen').isVisible())await page.locator('#govScreen .tbar .x').click();
  if(await page.locator('.arc-time-menu').getAttribute('open')===null)await page.locator('.arc-time-menu > summary').click();
  await page.locator('#campaignsBtn').click();
  if(!await page.locator('#savedCampaigns').isVisible())await page.locator('#openSavesBtn').click();
  await page.locator('#savedCampaigns').waitFor({state:'visible'});
}
async function extraAssets(page,url,expected,git,build){
  for(const name of ['agency-ui.js','agency.css']){
    const checkout=fs.readFileSync(path.join(root,'spheres-web/ui',name)),text=checkout.toString('utf8'),committed=git(['show',expected+':spheres-web/ui/'+name]);
    assert(Buffer.from(text).equals(checkout));const canonical=Buffer.from(text.replace(/\r\n/g,'\n'));assert(canonical.equals(committed));
    const r=await page.request.get(url+'/'+name);try{assert(r.ok());const served=await r.body();assert(served.equals(checkout));build.assets[name]={served_sha256:hash(served),checkout_sha256:hash(checkout),committed_sha256:hash(committed),canonical_checkout_sha256:hash(canonical),served_bytes:served.length,newline_normalization:'CRLF to LF for source comparison only; served bytes equal checkout exactly'};}finally{await r.dispose();}
  }
}
async function runCountry(test,browser,options){
  const {output,binary,expected,git,source}=options,out=fs.mkdtempSync(path.join(output,test.nation.toLowerCase()+'-')),run=path.join(out,'server');
  fs.mkdirSync(run);fs.mkdirSync(path.join(run,'saves'));const port=await freePort(),url='http://127.0.0.1:'+port;
  const server=cp.spawn(binary,['--port',String(port),'--no-open'],{cwd:run,windowsHide:true,stdio:['ignore','pipe','pipe']});
  const log=fs.createWriteStream(path.join(out,'server.log'));server.stdout.pipe(log,{end:false});server.stderr.pipe(log,{end:false});
  let launchError,page,context,stage='launch';server.on('error',error=>{launchError=error;});
  const started=performance.now(),e={passed:false,nation:test.nation,name:test.name,run,url,started_utc:new Date().toISOString(),scope:'Fresh CP1 startup, no elapsed days, no grants, no response substitutions; not a long-run or G2 qualification',test_source:source,commands:[],errors:[],screenshots:[],checks:[],timings_ms:{}};
  const telemetry=(event,details)=>fs.appendFileSync(path.join(out,'progress.jsonl'),JSON.stringify({utc:new Date().toISOString(),stage,event,...details})+'\n');
  const timed=async(name,fn)=>{stage=name;const start=performance.now();try{return await fn();}finally{e.timings_ms[name]=performance.now()-start;telemetry('stage-complete',{duration_ms:e.timings_ms[name]});}};
  const shot=async(name,selector)=>{if(selector)await page.locator(selector).scrollIntoViewIfNeeded();const file=name+'.png';await page.screenshot({path:path.join(out,file)});e.screenshots.push({file,sha256:hash(fs.readFileSync(path.join(out,file))),viewport:page.viewportSize()});};
  audit.configure(out,telemetry);
  try{
    await timed('launch and exact build',async()=>{
      const until=Date.now()+20000;for(;;){
        if(launchError)throw launchError;assert.equal(server.exitCode,null);
        try{const r=await fetch(url+'/api/build',{headers:{Connection:'close'},signal:AbortSignal.timeout(2000)});await r.arrayBuffer();if(r.ok)break;}catch(error){if(Date.now()>=until)throw error;}
        assert(Date.now()<until,'Server startup exceeded 20 seconds');await new Promise(resolve=>setTimeout(resolve,100));
      }
      context=await browser.newContext({viewport:{width:1440,height:1000},reducedMotion:'reduce'});page=await context.newPage();page.setDefaultTimeout(30000);
      page.on('pageerror',error=>e.errors.push(error.message));page.on('request',r=>{if(new URL(r.url()).pathname==='/api/command'&&r.method()==='POST')e.commands.push(r.postDataJSON());});
      e.build=await integrated.verifyBuild({page,url,root,run,binary});await extraAssets(page,url,expected,git,e.build);
    });
    const initial=await timed('ordinary fresh campaign',async()=>{
      await page.goto(url,{waitUntil:'domcontentloaded'});await page.locator('#campaignHome').waitFor();await page.waitForFunction(()=>!!SESSION.live?.session_id);
      await page.locator('#newCampaignBtn').click();await page.locator('#nationSearch').fill(test.name);
      await page.locator('#nationPick [aria-label^='+quoted(test.name+';')+']').click();await page.locator('#startBtn').click();
      await page.locator('#app').waitFor();await idle(page);assert.equal(await page.evaluate(()=>clock.running),false);
      const state=await get(page,url,'/api/state');assert.equal(state.player,test.nation);assert.equal(state.date,'1 Jan 1990');
      e.capabilities=await integrated.capabilities({page,url,player:test.nation});return state;
    });
    const before=await capture(page,url,run,test.nation,'s10b-before-views'),beforeFacts=politicalFacts(before,test.nation);e.archives={before};
    const board=await timed('own government and authored structure',async()=>{
      const data=await government(page,test.nation);assert.equal(data.mine,true);assert.equal(data.electoral,test.electoral);
      assert.equal(data.briefing.date_label,initial.date);compareBoardNative(data,beforeFacts);
      assert.equal(await page.locator('#govScreen .gov-ui-hero h1').innerText(),test.name);
      assert((await page.locator('#gov-panel-overview').innerText()).includes(data.leader?.name||data.leader?.described||data.ruling_institution),'Actual native officeholder/institution must be visible');
      await withinPanel(page,'#govScreen .gov-ui','Own overview');await shot('overview-desktop','#govScreen .gov-ui');
      await page.locator('#gov-tab-politics').click();assert.equal(await page.locator('#gov-tab-politics').innerText(),test.electoral?'Parliament & parties':'Regime & institutions');
      if(test.electoral)assert(parties(data).length>0);else{
        assert(data.pillars.length>0);const text=await page.locator('#gov-panel-politics').innerText();for(const pillar of data.pillars)assert(text.includes(pillar.name));
      }
      await page.setViewportSize({width:390,height:844});await withinPanel(page,'#govScreen .gov-ui','Political structure narrow');await shot('structure-narrow','#gov-panel-politics');await page.setViewportSize({width:1440,height:1000});
      e.opening_government=data;e.checks.push('Own overview and parliamentary or institutional structure match native records');return data;
    });
    await timed('party leadership and historical browsing',async()=>{
      assert(board.party_leadership,'Shipped party leadership board is required');await page.locator('#gov-tab-leadership').click();
      await withinPanel(page,'#govScreen .gov-ui','Party leadership');await shot('leadership-desktop','#gov-panel-leadership');
      await page.locator('[data-gov-leadership-mode="reference"]').click();
      await page.waitForFunction(()=>gov.leadershipReference&&!gov.leadershipReference.loading&&!!gov.leadershipReference.data&&!gov.leadershipReference.error);
      const referenceDate='2000-01-01';await page.locator('[data-gov-leadership-date]').fill(referenceDate);await page.locator('[data-gov-leadership-date]').press('Tab');
      await page.waitForFunction(date=>gov.leadershipReference?.data?.date===date&&!gov.leadershipReference.loading&&!gov.leadershipReference.error,referenceDate);
      e.historical_reference={date:referenceDate,nation:await page.evaluate(()=>gov.leadershipReference.data.nation)};assert.equal(e.historical_reference.nation,test.nation);
      await page.locator('[data-gov-leadership-mode="campaign"]').click();
      assert.deepEqual((await get(page,url,'/api/government?nation='+test.nation)).leader,board.leader);
      e.checks.push('Historical browsing changes the reference reading while retaining the campaign incumbent');
    });
    await timed('foreign inspection and served refusal',async()=>{
      const foreign=test.nation==='France'?'SaudiArabia':'France',data=await government(page,foreign,'decisions');
      assert.equal(data.mine,false);assert.equal(data.nation,foreign);await page.locator('[data-gov-filter="all"]').click();
      assert.match(await page.locator('#gov-panel-decisions').innerText(),/foreign government.*inspection/i);
      const controls=page.locator('#gov-panel-decisions [data-gov-review]');assert(await controls.count()>0);for(const button of await controls.all())assert(await button.isDisabled());
      e.foreign_inspection={nation:foreign,mine:data.mine,disabled_review_buttons:await controls.count(),leader:data.leader};
      const own=await government(page,test.nation,'decisions');await page.locator('[data-gov-filter="all"]').click();
      e.served_refusals=own.actions.map((action,index)=>({index,label:action.label,command:action.command,reason:action.refusal})).filter(row=>row.reason);
      for(const row of e.served_refusals){
        const card=page.locator('[data-gov-decision='+quoted(row.index)+']');assert((await card.innerText()).includes(row.reason));assert(await card.locator('[data-gov-review]').isDisabled());
      }
      if(e.served_refusals.length)await shot('served-refusal-desktop','[data-gov-decision='+quoted(e.served_refusals[0].index)+']');
      e.checks.push('Foreign actions remain view-only; actual served refusal text and disabled controls agree');
    });
    // Avoid inventing opening research, money or eligibility. These two shipped
    // action families have explicit coalition/loyalty outcomes without electing
    // or replacing an incumbent. Absence is a failure, never a silent skip.
    const kind=board.electoral?'invite':'secure_pillar',legal=board.actions.filter(a=>a.kind===kind&&!a.refusal&&a.affordable!==false&&Number.isFinite(a.price)&&a.price>0).sort((a,b)=>a.price-b.price);
    assert(legal.length,'No ordinary paid '+kind+' available: '+JSON.stringify(board.actions));const command=copy(legal[0].command);
    const cancelled=await timed('legal review and cancellation',async()=>{
      const quote=await review(page,test.nation,command);assert.equal(quote.date_label,initial.date);
      await withinPanel(page,'#govReview','Decision review');await shot('review-desktop','#govReview');
      await page.setViewportSize({width:390,height:844});await withinPanel(page,'#govReview','Decision review narrow');await shot('review-narrow','#govReview');
      await page.locator('#govReview [data-gov-review-close]').last().click();await page.locator('#govReview').waitFor({state:'hidden'});await page.setViewportSize({width:1440,height:1000});
      return quote;
    });
    const afterCancel=await capture(page,url,run,test.nation,'s10b-after-cancel');e.archives.after_cancel=afterCancel;audit.compare(afterCancel,before,'Views, historical/foreign browsing, native refusals, review and cancellation changed '+test.nation);
    assert.equal(e.commands.length,0);assert.deepEqual((await get(page,url,'/api/government?nation='+test.nation)).leader,board.leader);e.cancelled_review=cancelled;
    e.checks.push('All viewing and review/cancellation preserve every native world field and issue no command');
    const applied=await timed('one exact confirmed native decision',async()=>{
      const quote=await review(page,test.nation,command);assert.deepEqual(quote,cancelled,'Pure cancelled review must remain identical');
      const response=page.waitForResponse(r=>post(r,'/api/command'));await page.locator('#govReview [data-gov-confirm]').click();
      const r=await response;assert(r.ok());const state=await r.json();assert.deepEqual(state.errors,[]);await idle(page);
      const current=await get(page,url,'/api/state'),government=await get(page,url,'/api/government?nation='+test.nation);
      assert.equal(current.date,quote.date_label);assert.equal(current.date,initial.date);assert(current.dispatch_count>initial.dispatch_count);
      const events=current.log.slice(0,current.dispatch_count-initial.dispatch_count);assert(events.length&&events.every(row=>row.date===quote.date_label));
      await shot('result-desktop','#govScreen .gov-ui');return {quote,state:current,government,events};
    });
    const afterAction=await capture(page,url,run,test.nation,'s10b-after-action'),afterFacts=politicalFacts(afterAction,test.nation);e.archives.after_action=afterAction;
    assert.notEqual(afterAction.canonical.sha256,before.canonical.sha256);verifyOutcome(applied.quote,board,applied.government,beforeFacts,afterFacts);
    e.confirmed_review=applied.quote;e.native_outcome={before:beforeFacts,after:afterFacts,events:applied.events,government:applied.government};
    assert.equal(e.commands.length,1);assert.deepEqual(e.commands[0].commands,[command]);assert.equal(e.commands[0].review_kind,'government');assert.equal(e.commands[0].review_token,applied.quote.review_token);
    e.checks.push('One reviewed confirmation matches exact native political/financial costs, every quoted effect and a dated result; incumbent retained');
    await timed('ordinary named Save Load and Continue',async()=>{
      const slot='s10b-'+test.nation.toLowerCase()+'-government';await campaigns(page);await page.locator('#saveName').fill(slot);
      const saved=page.waitForResponse(r=>post(r,'/api/save'));await page.locator('#saveNamedBtn').click();assert((await saved).ok());
      await page.locator('#saveSlots option[value='+quoted(slot)+']').waitFor({state:'attached'});await page.locator('#saveSlots').selectOption(slot);
      const loaded=page.waitForResponse(r=>post(r,'/api/load'));await page.locator('#loadBtn').click();await page.locator('#campaignConfirmAccept').click();assert((await loaded).ok());
      await page.locator('#app').waitFor();await idle(page);const state=await get(page,url,'/api/state');assert.notEqual(state.session_id,applied.state.session_id);assert.equal(state.date,initial.date);
      assert.deepEqual((await get(page,url,'/api/government?nation='+test.nation)),applied.government,'Loaded government reading differs');
      await page.reload({waitUntil:'domcontentloaded'});await page.locator('#continueBtn').click();await page.locator('#app').waitFor();await idle(page);
      const continued=await get(page,url,'/api/state');assert.equal(continued.date,initial.date);assert.equal(continued.player,test.nation);assert.equal(await page.evaluate(()=>clock.running),false);
      const afterLoad=await capture(page,url,run,test.nation,'s10b-after-load-continue');e.archives.after_load_continue=afterLoad;audit.compare(afterLoad,afterAction,'Named Save/Load/Continue changed complete native '+test.nation+' campaign');
      const resumed=await government(page,test.nation);assert.deepEqual(resumed,applied.government);
      await page.setViewportSize({width:390,height:844});await withinPanel(page,'#govScreen .gov-ui','Continued overview narrow');await shot('continued-overview-narrow','#govScreen .gov-ui');
      e.saved_slot=slot;e.final_state={player:continued.player,date:continued.date,political_capital:resumed.political_capital,leader:resumed.leader};
    });
    assert.equal(e.commands.length,1);assert.deepEqual(e.errors,[]);e.checks.push('Ordinary named Save/Load/Continue retains the complete native campaign, costs and incumbent');e.passed=true;
  }catch(error){e.failed_stage=stage;e.failure=String(error.stack||error);if(page)try{await page.screenshot({path:path.join(out,'failure.png')});fs.writeFileSync(path.join(out,'failure.html'),await page.content());}catch(captureError){e.failure_capture_error=String(captureError);}}
  finally{
    try{if(context)await context.close();}catch(error){e.passed=false;e.context_cleanup_error=String(error);}
    try{
      if(server.pid&&server.exitCode===null){
        await new Promise((resolve,reject)=>{const timer=setTimeout(()=>reject(new Error('Owned matrix server did not stop within 5 seconds')),5000);server.once('close',()=>{clearTimeout(timer);resolve();});server.kill();});
      }
      e.owned_server_stopped=server.exitCode!==null||server.signalCode!==null||!server.pid;
      assert(e.owned_server_stopped,'Owned matrix server is still running');
    }catch(error){e.passed=false;e.server_cleanup_error=String(error);}
    await new Promise(resolve=>log.end(resolve));
    e.finished_utc=new Date().toISOString();e.total_ms=performance.now()-started;write(path.join(out,'result.json'),e);
  }
  return {nation:test.nation,passed:e.passed,result:path.join(out,'result.json'),result_sha256:hash(fs.readFileSync(path.join(out,'result.json'))),duration_ms:e.total_ms,failed_stage:e.failed_stage};
}
async function main(){
  assert(process.env.SPHERES_BINARY,'Set the already built SPHERES_BINARY');const expected=process.env.SPHERES_EXPECTED_REVISION||'';assert.match(expected,/^[a-f0-9]{40}$/);
  const git=args=>cp.execFileSync('git',['-c','core.longpaths=true',...args],{cwd:root,windowsHide:true,maxBuffer:32*1024*1024});
  const revision=git(['rev-parse','HEAD']).toString().trim();assert.equal(git(['status','--porcelain']).toString().trim(),'','Require a clean committed checkout');
  assert.equal(git(['diff','--name-only',expected,revision,'--','spheres-sim','spheres-cli','spheres-web','Cargo.toml','Cargo.lock']).toString().trim(),'','A newer driver must preserve all runtime source');
  const binary=path.resolve(process.env.SPHERES_BINARY);assert(fs.statSync(binary).isFile());const binaryHash=hash(fs.readFileSync(binary));
  const requested=process.env.SPHERES_COUNTRY_MATRIX_NATIONS?process.env.SPHERES_COUNTRY_MATRIX_NATIONS.split(','):CASES.map(row=>row.nation);
  assert(requested.length&&new Set(requested).size===requested.length);for(const nation of requested)assert(CASES.some(row=>row.nation===nation),'Unsupported matrix country: '+nation);
  const selected=CASES.filter(row=>requested.includes(row.nation));
  const base=path.resolve(process.env.SPHERES_COUNTRY_MATRIX_OUTPUT||path.join(root,'artifacts/browser-government-country-matrix-ci'));fs.mkdirSync(base,{recursive:true});
  const output=fs.mkdtempSync(path.join(base,'matrix-')),source={revision,driver_sha256:hash(fs.readFileSync(__filename)),runtime_source_equal:true};
  const result={passed:false,scope:'S10.b ordinary Jan-1990 startup matrix; zero elapsed days; not G2, a long-run campaign or a Russia birth test',runtime_revision:expected,binary_sha256:binaryHash,test_source:source,requested_nations:requested,full_matrix_requested:selected.length===CASES.length,full_eight_country_matrix:false,started_utc:new Date().toISOString(),cases:[]};
  let browser;
  try{
    browser=await chromium.launch({headless:true,args:['--log-net-log='+path.join(output,'chrome-netlog.json')],...(process.env.SPHERES_BROWSER_CHANNEL?{channel:process.env.SPHERES_BROWSER_CHANNEL}:{})});result.browser_version=browser.version();
    for(const test of selected){const outcome=await runCountry(test,browser,{output,binary,expected,git,source});result.cases.push(outcome);write(path.join(output,'result.json'),result);console.log(JSON.stringify(outcome));}
    assert.equal(hash(fs.readFileSync(binary)),binaryHash);assert.equal(git(['rev-parse','HEAD']).toString().trim(),revision);assert.equal(git(['status','--porcelain']).toString().trim(),'');
    result.passed=result.cases.length===selected.length&&result.cases.every(row=>row.passed);
    result.full_eight_country_matrix=result.passed&&selected.length===CASES.length;
  }catch(error){result.failure=String(error.stack||error);}
  finally{if(browser)await browser.close();result.finished_utc=new Date().toISOString();write(path.join(output,'result.json'),result);}
  console.log(JSON.stringify({passed:result.passed,full_eight_country_matrix:result.full_eight_country_matrix,result:path.join(output,'result.json')}));if(!result.passed)process.exitCode=1;
}
main().catch(error=>{console.error(error);process.exitCode=1;});
