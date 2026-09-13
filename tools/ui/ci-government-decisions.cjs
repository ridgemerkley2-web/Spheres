// S10: ordinary France menus, a real second tab and native save comparisons.
// The driver grants no state, intercepts no responses and advances no clock.
const assert=require('node:assert/strict'),fs=require('node:fs'),path=require('node:path');
const cp=require('node:child_process'),net=require('node:net'),crypto=require('node:crypto');
const {chromium}=require('playwright'),integrated=require('./ci-integrated.cjs'),audit=require('./supplier-archive-audit.cjs');
const root=path.resolve(__dirname,'../..'),copy=x=>JSON.parse(JSON.stringify(x));
const hash=bytes=>crypto.createHash('sha256').update(bytes).digest('hex');
const quoted=x=>JSON.stringify(String(x));
const route=(response,p)=>new URL(response.url()).pathname===p&&response.request().method()==='POST';
const SLOTS=new Set(['s10-before-review','s10-after-cancel','s10-after-second-tab','s10-after-stale','s10-after-government','s10-after-load-cancel','s10-after-load','s10-after-continue']);
async function freePort(){const server=net.createServer();await new Promise(resolve=>server.listen(0,'127.0.0.1',resolve));const port=server.address().port;await new Promise(resolve=>server.close(resolve));return port;}
async function get(page,url,p){const response=await page.request.get(url+p);try{assert(response.ok(),p);return await response.json();}finally{await response.dispose();}}
async function idle(page){await page.waitForFunction(()=>!SESSION.busy&&!COMMAND_CHANNEL.busy&&!COMMAND_CHANNEL.pending&&!advancing&&!pendingAdvance&&!AGENCY.busy&&!gov.busy);}
async function withinPanel(page,selector,label){
  const box=await page.locator(selector).evaluate(e=>({scroll:e.scrollWidth,width:e.clientWidth}));
  assert(box.width>0&&box.scroll<=box.width+1,label+' overflows horizontally: '+JSON.stringify(box));
  assert(!/\bNaN\b|\bundefined\b/.test(await page.locator(selector).innerText()),label+' has an invalid label');
}
async function captures(page,url,run,slot){
  assert(SLOTS.has(slot),'Only audit-only slot names may move out of the menu');
  const base=fs.realpathSync(run),saves=path.join(base,'saves'),source=path.join(saves,slot+'.json');
  assert.equal(fs.realpathSync(saves),saves);assert(!fs.existsSync(source));
  const response=await page.request.post(url+'/api/save',{data:{slot}});
  try{assert(response.ok());await response.body();}finally{await response.dispose();}
  assert(fs.lstatSync(source).isFile()&&!fs.lstatSync(source).isSymbolicLink());assert.equal(fs.realpathSync(source),source);
  const output=path.join(base,'audit-captures');if(!fs.existsSync(output))fs.mkdirSync(output);
  assert.equal(fs.realpathSync(output),output);assert(!fs.lstatSync(output).isSymbolicLink());
  const target=path.join(fs.mkdtempSync(path.join(output,slot+'-')),slot+'.json');assert(!fs.existsSync(target));
  fs.renameSync(source,target);return audit.inspect(target,'France');
}
async function government(page){
  if(await page.locator('#agencyPanel').isVisible())await page.locator('#agencyClose').click();
  if(!await page.locator('#govScreen').isVisible())await page.locator('#govBtn').click();
  await page.waitForFunction(()=>gov.open&&gov.data&&gov.dataState===S&&!gov.data.error);
  await page.locator('#gov-tab-decisions').click();
  return page.evaluate(()=>JSON.parse(JSON.stringify(gov.data)));
}
async function reviewGovernment(page,command){
  const data=await government(page),index=data.actions.findIndex(a=>JSON.stringify(a.command)===JSON.stringify(command));
  assert(index>=0,'The same real government decision must still be served');assert(!data.actions[index].refusal);
  const response=page.waitForResponse(r=>route(r,'/api/government/preview'));
  await page.locator('#govScreen [data-gov-review='+quoted(index)+']').click();
  const received=await response;assert(received.ok());const quote=await received.json();
  await page.waitForFunction(()=>gov.review&&!gov.review.loading);
  assert.equal(quote.valid,true);assert.deepEqual(quote.command,command);assert.equal(typeof quote.review_token,'string');assert(quote.review_token);
  assert.equal(await page.locator('#govReview [data-gov-confirm]').isEnabled(),true);
  return quote;
}
async function policyReview(page,policy){
  await page.locator('#agencyBtn').click();await page.locator('#agencyPanel').waitFor({state:'visible'});
  for(const [key,value] of Object.entries(policy))await page.locator('#agencyPolicy select[name='+quoted(key)+']').selectOption(value);
  const response=page.waitForResponse(r=>route(r,'/api/decisions/preview'));
  await page.locator('#agencyPolicy button[type="submit"]').click();
  const received=await response;assert(received.ok());const quote=await received.json();
  await page.waitForFunction(()=>AGENCY.review&&!AGENCY.review.loading);
  assert.equal(quote.valid,true);assert.deepEqual(quote.command,{kind:'set_diplomatic_policy',policy});
  assert.equal(typeof quote.review_token,'string');assert(quote.review_token);
  assert.equal(await page.locator('#agencyReview [data-agency-confirm]').isEnabled(),true);return quote;
}
async function campaigns(page){
  if(await page.locator('#agencyPanel').isVisible())await page.locator('#agencyClose').click();
  if(await page.locator('#govScreen').isVisible())await page.locator('#govScreen .tbar .x').click();
  if(await page.locator('#app').isVisible()){
    if(await page.locator('.arc-time-menu').getAttribute('open')===null)await page.locator('.arc-time-menu > summary').click();
    await page.locator('#campaignsBtn').click();
  }
  if(!await page.locator('#savedCampaigns').isVisible())await page.locator('#openSavesBtn').click();
  await page.locator('#savedCampaigns').waitFor({state:'visible'});
}
async function main(){
  assert(process.env.SPHERES_BINARY,'Set the already built SPHERES_BINARY');
  const expected=process.env.SPHERES_EXPECTED_REVISION||'';assert.match(expected,/^[a-f0-9]{40}$/);
  const git=args=>cp.execFileSync('git',['-c','core.longpaths=true',...args],{cwd:root,windowsHide:true,maxBuffer:32*1024*1024});
  const driverRevision=git(['rev-parse','HEAD']).toString().trim();assert.equal(git(['status','--porcelain']).toString().trim(),'','Require a clean committed checkout');
  assert.equal(git(['diff','--name-only',expected,driverRevision,'--','spheres-sim','spheres-cli','spheres-web','Cargo.toml','Cargo.lock']).toString().trim(),'','A newer driver must preserve the entire runtime source');
  const binary=path.resolve(process.env.SPHERES_BINARY);assert(fs.statSync(binary).isFile());const binaryHash=hash(fs.readFileSync(binary));
  const output=path.resolve(process.env.SPHERES_DECISIONS_OUTPUT||path.join(root,'artifacts/browser-government-decisions-ci'));
  fs.mkdirSync(output,{recursive:true});const out=fs.mkdtempSync(path.join(output,'france-')),run=path.join(out,'server');fs.mkdirSync(run);fs.mkdirSync(path.join(run,'saves'));
  const port=await freePort(),url='http://127.0.0.1:'+port;
  const server=cp.spawn(binary,['--port',String(port),'--no-open'],{cwd:run,windowsHide:true,stdio:['ignore','pipe','pipe']});
  const log=fs.createWriteStream(path.join(out,'server.log'));server.stdout.pipe(log);server.stderr.pipe(log);
  let browser,page,second,launchError,stage='launch';server.on('error',error=>{launchError=error;});
  const e={passed:false,run,url,started_utc:new Date().toISOString(),test_source:{revision:driverRevision,driver_sha256:hash(fs.readFileSync(__filename)),runtime_source_equal:true},fixture:'Fresh France through ordinary controls, real same-campaign second tab; no state grants or substituted responses',screenshots:[],commands:[],errors:[],checks:[]};
  const write=(name,value)=>fs.writeFileSync(path.join(out,name),JSON.stringify(value,null,2)+'\n');
  const telemetry=(event,details)=>fs.appendFileSync(path.join(out,'progress.jsonl'),JSON.stringify({utc:new Date().toISOString(),stage,event,...details})+'\n');
  const observe=(p,label)=>{p.setDefaultTimeout(30000);p.on('pageerror',error=>e.errors.push(label+': '+error.message));p.on('request',request=>{if(new URL(request.url()).pathname==='/api/command'&&request.method()==='POST')e.commands.push({tab:label,payload:request.postDataJSON()});});};
  const shot=async(p,name,selector)=>{if(selector)await p.locator(selector).scrollIntoViewIfNeeded();await p.screenshot({path:path.join(out,name+'.png')});e.screenshots.push(name+'.png');};
  audit.configure(out,telemetry);
  try{
    const until=Date.now()+20000;
    for(;;){
      if(launchError)throw launchError;assert.equal(server.exitCode,null);
      try{const response=await fetch(url+'/api/build',{headers:{Connection:'close'},signal:AbortSignal.timeout(2000)});await response.arrayBuffer();if(response.ok)break;}catch(error){if(Date.now()>=until)throw error;}
      assert(Date.now()<until,'Server startup exceeded 20 seconds');await new Promise(resolve=>setTimeout(resolve,100));
    }
    browser=await chromium.launch({headless:true,args:['--log-net-log='+path.join(out,'chrome-netlog.json')],...(process.env.SPHERES_BROWSER_CHANNEL?{channel:process.env.SPHERES_BROWSER_CHANNEL}:{})});
    e.browser_version=browser.version();const context=await browser.newContext({viewport:{width:1440,height:1000},reducedMotion:'reduce'});page=await context.newPage();observe(page,'A');
    e.build=await integrated.verifyBuild({page,url,root,run,binary});
    for(const name of ['agency-ui.js','agency.css']){
      const checkout=fs.readFileSync(path.join(root,'spheres-web/ui',name)),text=checkout.toString('utf8'),committed=git(['show',expected+':spheres-web/ui/'+name]);
      assert(Buffer.from(text).equals(checkout));const canonical=Buffer.from(text.replace(/\r\n/g,'\n'));assert(canonical.equals(committed));
      const response=await page.request.get(url+'/'+name);try{assert(response.ok());const served=await response.body();assert(served.equals(checkout));e.build.assets[name]={served_sha256:hash(served),checkout_sha256:hash(checkout),committed_sha256:hash(committed),canonical_checkout_sha256:hash(canonical),served_bytes:served.length,newline_normalization:'CRLF to LF for source comparison only; served bytes equal checkout exactly'};}finally{await response.dispose();}
    }
    stage='fresh France';await page.goto(url,{waitUntil:'domcontentloaded'});await page.locator('#campaignHome').waitFor();await page.waitForFunction(()=>!!SESSION.live?.session_id);
    await page.locator('#newCampaignBtn').click();await page.locator('#nationPick [aria-label^="France;"]').click();await page.locator('#startBtn').click();
    await page.locator('#app').waitFor();await idle(page);assert.equal(await page.evaluate(()=>clock.running),false);
    const initial=await get(page,url,'/api/state');assert.equal(initial.player,'France');e.capabilities=await integrated.capabilities({page,url,player:'France'});
    const before=await captures(page,url,run,'s10-before-review');

    stage='standing policy review and cancellation';
    const policy={...initial.agency.policy,trade_treaties:initial.agency.policy.trade_treaties==='decline'?'review':'decline'};
    const policyQuote=await policyReview(page,policy);assert.equal(policyQuote.date_label,initial.date);e.cancelled_policy_review=policyQuote;
    await withinPanel(page,'#agencyPanel','Decisions desktop');await shot(page,'policy-review-desktop','#agencyReview');
    await page.setViewportSize({width:390,height:844});await withinPanel(page,'#agencyPanel','Decisions narrow');await shot(page,'policy-review-narrow','#agencyReview');
    await page.locator('#agencyReview [data-agency-review-cancel]').click();await page.locator('#agencyReview').waitFor({state:'hidden'});
    await page.locator('#agencyClose').click();await page.setViewportSize({width:1440,height:1000});
    audit.compare(await captures(page,url,run,'s10-after-cancel'),before,'Policy review and cancellation changed the native campaign');
    assert.equal(e.commands.length,0);e.checks.push('Desktop/narrow policy review and cancellation are whole-campaign pure');

    stage='first tab holds government review';
    const board=await government(page),rank=a=>a.kind==='invite'?0:a.kind==='stratagem'?1:2;
    const legal=board.actions.filter(a=>!a.refusal&&a.affordable!==false&&Number.isFinite(a.price)&&a.price>0&&['invite','stratagem'].includes(a.kind)).sort((a,b)=>rank(a)-rank(b)||a.price-b.price);
    assert(legal.length,'Fresh France must have an ordinary legal paid government decision');const command=copy(legal[0].command);
    const held=await reviewGovernment(page,command);e.held_government_review=held;
    await shot(page,'government-review-before-second-tab','#govReview');

    stage='second tab makes a real policy decision';
    second=await context.newPage();observe(second,'B');await second.goto(url,{waitUntil:'domcontentloaded'});
    await second.locator('#continueBtn').click();await second.locator('#app').waitFor();await idle(second);
    assert.equal(await second.evaluate(()=>S.session_id),initial.session_id,'Second tab must continue the same live campaign');
    const secondQuote=await policyReview(second,policy);e.confirmed_policy_review=secondQuote;
    const changedResponse=second.waitForResponse(r=>route(r,'/api/command'));await second.locator('#agencyReview [data-agency-confirm]').click();
    const changed=await changedResponse;assert(changed.ok());const changedState=await changed.json();assert.deepEqual(changedState.errors,[]);await idle(second);
    const afterPolicyState=await get(second,url,'/api/state');assert.deepEqual(afterPolicyState.agency.policy,policy);assert.equal(afterPolicyState.date,initial.date);
    await shot(second,'second-tab-policy-recorded','#agencyStatus');
    const afterPolicy=await captures(second,url,run,'s10-after-second-tab');assert.notEqual(afterPolicy.canonical.sha256,before.canonical.sha256,'Actual policy confirmation must change native state');
    assert.equal(e.commands.length,1);

    stage='old government confirmation refuses without effect';
    assert.equal(await page.evaluate(()=>gov.review?.data?.review_token),held.review_token,'First tab must still hold its actual old review');
    const staleResponse=page.waitForResponse(r=>route(r,'/api/command'));await page.locator('#govReview [data-gov-confirm]').click();
    const refused=await staleResponse;assert.equal(refused.status(),400);const refusal=await refused.json();
    assert.equal(refusal.requires_review,true);assert.match(refusal.error,/out of date|review.*again/i);e.stale_refusal=refusal;
    await idle(page);await page.waitForFunction(()=>gov.review===null&&gov.dataState===S);
    assert.deepEqual(await page.evaluate(()=>JSON.parse(JSON.stringify(S.agency.policy))),policy,'First tab must refresh real current figures after rejection');
    assert.match(await page.locator('#govScreen').innerText(),/out of date|review.*again/i);
    await shot(page,'government-stale-review-refused','#govScreen .gov-ui');
    audit.compare(await captures(page,url,run,'s10-after-stale'),afterPolicy,'Stale government confirmation changed the campaign after tab B');
    assert.equal(e.commands.length,2);e.checks.push('A genuine second-tab policy change invalidates the held government review; refusal has no native effect and refreshes current figures');
    await second.close();second=null;

    stage='fresh reviewed government decision';
    const beforeGovernment=await get(page,url,'/api/government?nation=France'),fresh=await reviewGovernment(page,command);
    assert.notEqual(fresh.review_token,held.review_token);assert.equal(fresh.date_label,initial.date);e.confirmed_government_review=fresh;
    await withinPanel(page,'#govReview','Government immediate review');await shot(page,'government-fresh-review','#govReview');
    const appliedResponse=page.waitForResponse(r=>route(r,'/api/command'));await page.locator('#govReview [data-gov-confirm]').click();
    const applied=await appliedResponse;assert(applied.ok());const appliedState=await applied.json();assert.deepEqual(appliedState.errors,[]);await idle(page);
    const afterGovernment=await get(page,url,'/api/government?nation=France'),afterState=await get(page,url,'/api/state');
    assert(Math.abs((beforeGovernment.political_capital-afterGovernment.political_capital)-fresh.price_pc)<1e-9,'Actual political-capital cost must equal the native review');
    assert.equal(afterState.date,fresh.date_label);assert(afterState.dispatch_count>afterPolicyState.dispatch_count,'An actual government result must be recorded');
    const newEvents=afterState.log.slice(0,afterState.dispatch_count-afterPolicyState.dispatch_count);assert(newEvents.length&&newEvents.every(row=>row.date===fresh.date_label));
    e.government_result={command,price_pc:fresh.price_pc,before_pc:beforeGovernment.political_capital,after_pc:afterGovernment.political_capital,date:afterState.date,events:newEvents};
    const afterAction=await captures(page,url,run,'s10-after-government');assert.notEqual(afterAction.canonical.sha256,afterPolicy.canonical.sha256);
    await shot(page,'government-result-desktop','#govScreen .gov-ui');
    assert.equal(e.commands.length,3);e.checks.push('A fresh government review applies once with the exact reviewed political cost and a dated native result');

    stage='named save, cancellation, load and Continue';const slot='s10-government-decisions',beforeLoad=await get(page,url,'/api/state');await campaigns(page);
    await page.locator('#saveName').fill(slot);const saveResponse=page.waitForResponse(r=>route(r,'/api/save'));await page.locator('#saveNamedBtn').click();assert((await saveResponse).ok());
    await page.locator('#saveSlots option[value='+quoted(slot)+']').waitFor({state:'attached'});await page.locator('#saveSlots').selectOption(slot);
    await page.locator('#loadBtn').click();await page.locator('#campaignConfirmCancel').click();assert.deepEqual(await get(page,url,'/api/state'),beforeLoad);
    audit.compare(await captures(page,url,run,'s10-after-load-cancel'),afterAction,'Cancelled loading changed the complete campaign');
    const loadResponse=page.waitForResponse(r=>route(r,'/api/load'));await page.locator('#loadBtn').click();await page.locator('#campaignConfirmAccept').click();assert((await loadResponse).ok());
    await page.locator('#app').waitFor();await idle(page);const loaded=await get(page,url,'/api/state');assert.notEqual(loaded.session_id,beforeLoad.session_id);assert.equal(loaded.date,beforeLoad.date);assert.deepEqual(loaded.agency.policy,policy);
    audit.compare(await captures(page,url,run,'s10-after-load'),afterAction,'Named Save/Load changed government, policy or other native campaign state');
    await page.reload({waitUntil:'domcontentloaded'});await page.locator('#continueBtn').click();await page.locator('#app').waitFor();await idle(page);
    const continued=await get(page,url,'/api/state');assert.deepEqual(continued.agency.policy,policy);assert.equal(continued.date,beforeLoad.date);
    audit.compare(await captures(page,url,run,'s10-after-continue'),afterAction,'Continue changed the complete native campaign');
    const resumedGovernment=await government(page);assert.equal(resumedGovernment.political_capital,afterGovernment.political_capital);assert.deepEqual(resumedGovernment.leader,afterGovernment.leader);
    await page.setViewportSize({width:390,height:844});await withinPanel(page,'#govScreen .gov-ui','Government continued narrow');await shot(page,'government-continued-narrow','#govScreen .gov-ui');
    e.saved_slot=slot;e.final_state={player:continued.player,date:continued.date,policy:continued.agency.policy,leader:resumedGovernment.leader,political_capital:resumedGovernment.political_capital};
    assert.equal(e.commands.length,3);assert.deepEqual(e.commands.map(x=>x.payload.review_kind),['decisions','government','government']);
    assert.deepEqual(e.commands.map(x=>x.payload.commands),[[{kind:'set_diplomatic_policy',policy}],[command],[command]]);
    assert.equal(e.commands[1].payload.review_token,held.review_token);assert.equal(e.commands[2].payload.review_token,fresh.review_token);
    assert.deepEqual(e.errors,[]);assert.equal(hash(fs.readFileSync(binary)),binaryHash);assert.equal(git(['rev-parse','HEAD']).toString().trim(),driverRevision);assert.equal(git(['status','--porcelain']).toString().trim(),'');
    e.checks.push('Named-save cancellation, actual Load and Continue preserve the exact native campaign, policy and recorded officeholder');
    e.passed=true;e.finished_utc=new Date().toISOString();write('result.json',e);console.log(JSON.stringify({passed:true,result:path.join(out,'result.json')}));
  }catch(error){e.failed_stage=stage;e.failure=String(error.stack||error);write('result.json',e);if(page)try{await page.screenshot({path:path.join(out,'failure.png')});fs.writeFileSync(path.join(out,'failure.html'),await page.content());}catch{}throw error;}
  finally{if(browser)await browser.close();server.kill();log.end();}
}
main().catch(error=>{console.error(error);process.exitCode=1;});
