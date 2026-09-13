// S10.e: actual browser controls against an explicitly authored native fixture.
// The driver never rewrites a world, injects a response or touches an existing
// review server. Native export provides before, accepted and one-day oracles.
const assert=require('node:assert/strict'),fs=require('node:fs'),path=require('node:path');
const cp=require('node:child_process'),net=require('node:net'),crypto=require('node:crypto');
const {chromium}=require('playwright'),integrated=require('./ci-integrated.cjs');
const audit=require('./supplier-archive-audit.cjs'),reviewUI=require('./government-review-assertions.cjs');
const root=path.resolve(__dirname,'../..'),quoted=value=>JSON.stringify(String(value));
const hash=value=>crypto.createHash('sha256').update(value).digest('hex');
const route=(response,p)=>new URL(response.url()).pathname===p&&response.request().method()==='POST';
const deadlineOrder=(a,b)=>a.expires<b.expires?-1:a.expires>b.expires?1:a.id-b.id;
const stableQuote=quote=>{const {session_id,review_token,...stable}=quote;return stable;};
const SLOT='s10e-authored-commitments',SAVED='s10e-reviewed-commitments';
const CAPTURES=new Set(['s10e-before-review','s10e-after-cancel','s10e-after-reply','s10e-after-load-cancel','s10e-after-load','s10e-after-continue','s10e-after-day']);
async function freePort(){const s=net.createServer();await new Promise(resolve=>s.listen(0,'127.0.0.1',resolve));const port=s.address().port;await new Promise(resolve=>s.close(resolve));return port;}
async function get(page,url,p){const r=await page.request.get(url+p);try{assert(r.ok(),p);return await r.json();}finally{await r.dispose();}}
async function idle(page){await page.waitForFunction(()=>!SESSION.busy&&!COMMAND_CHANNEL.busy&&!COMMAND_CHANNEL.pending&&!advancing&&!pendingAdvance&&!AGENCY.busy);}
function regular(file){assert(fs.lstatSync(file).isFile()&&!fs.lstatSync(file).isSymbolicLink());assert.equal(fs.realpathSync(file),file);return file;}
function nativeFile(directory,name){assert(['before.json','expected-after.json','expected-after-day.json'].includes(name));return regular(path.join(directory,name));}
async function capture(page,url,run,slot){
  assert(CAPTURES.has(slot),'Only a driver-owned slot can be moved');
  const saves=path.join(fs.realpathSync(run),'saves');assert.equal(fs.realpathSync(saves),saves);
  const source=path.join(saves,slot+'.json');assert(!fs.existsSync(source));
  const r=await page.request.post(url+'/api/save',{data:{slot}});try{assert(r.ok());await r.body();}finally{await r.dispose();}
  regular(source);const destination=path.join(run,'captures');if(!fs.existsSync(destination))fs.mkdirSync(destination);
  assert.equal(fs.realpathSync(destination),destination);const target=path.join(destination,slot+'.json');assert(!fs.existsSync(target));
  fs.renameSync(source,target);return audit.inspect(target,'France');
}
async function within(page,selector,label){
  const r=await page.locator(selector).evaluate(e=>({width:e.clientWidth,scroll:e.scrollWidth,left:e.scrollLeft}));
  assert(r.width>0&&r.scroll<=r.width+1&&Math.abs(r.left)<1,label+' has horizontal overflow: '+JSON.stringify(r));
  assert(!/\bNaN\b|\bundefined\b/.test(await page.locator(selector).innerText()),label+' has missing values');return r;
}
async function readable(locator,label){
  await locator.evaluate(e=>e.scrollIntoView({block:'center',inline:'nearest',behavior:'instant'}));
  const result=await locator.evaluate(e=>{
    const rect=e.getBoundingClientRect(),range=document.createRange();range.selectNodeContents(e);
    const ranges=Array.from(range.getClientRects()).filter(r=>r.width&&r.height);
    const errors=[],inside=r=>r.left>=-1&&r.right<=document.documentElement.clientWidth+1&&r.top>=-1&&r.bottom<=innerHeight+1;
    if(!inside(rect))errors.push('outside viewport');
    for(const r of ranges){if(!inside(r)||r.left<rect.left-1||r.right>rect.right+1||r.top<rect.top-1||r.bottom>rect.bottom+1)errors.push('clipped text');
      for(const fraction of [.2,.5,.8]){const hit=document.elementFromPoint(r.left+r.width*fraction,r.top+r.height/2);if(!hit||!(hit===e||e.contains(hit)))errors.push('obscured text');}}
    return {text:e.textContent,rect:{left:rect.left,right:rect.right,top:rect.top,bottom:rect.bottom},errors};
  });assert.deepEqual(result.errors,[],label+': '+JSON.stringify(result));return result;
}
async function commitmentDesk(page,agency,width){
  await page.setViewportSize({width,height:width===1440?1000:844});
  const board=agency.commitments;assert(board,'Native commitments are required');
  const section=page.locator('#agencyCommitments');assert(await section.isVisible());
  for(const [key,attr,id] of [['defense_pacts','pact','partner'],['trade_agreements','trade','partner'],['conflicts','conflict','conflict_id']]){
    assert.deepEqual(await section.locator('[data-agency-'+attr+']').evaluateAll((rows,k)=>rows.map(e=>e.getAttribute('data-agency-'+k)),attr),board[key].map(r=>String(r[id])),'Exact native '+key+' roster');
  }
  const evidence={width,defense:[],trade:[],conflicts:[]};
  for(const row of board.defense_pacts){
    const card=section.locator('[data-agency-pact='+quoted(row.partner)+']'),body=await card.innerText();
    for(const value of [row.partner_name,row.since,row.status_label,row.upkeep.next_step_label])assert(body.includes(value),'Native pact field missing: '+value);
    for(const [hook,value] of [['since',row.since],['status',row.status_label],['upkeep',row.upkeep.next_step_label]])assert.equal(await card.locator('[data-agency-'+hook+']').textContent(),value,'Exact native pact '+hook);
    assert.deepEqual(await card.locator('[data-agency-commitment-reply]').evaluateAll(rows=>rows.map(e=>Number(e.dataset.agencyCommitmentReply))),row.pending_offer_ids);
    evidence.defense.push({partner:row.partner,text:body,readings:await cardReadings(card,'Defense pact '+row.partner+' at '+width)});
  }
  for(const row of board.trade_agreements){
    const card=section.locator('[data-agency-trade='+quoted(row.partner)+']'),body=await card.innerText();
    for(const value of [row.partner_name,row.status_label])assert(body.includes(value),'Native trade field missing: '+value);
    assert.equal(await card.locator('[data-agency-status]').textContent(),row.status_label);
    for(const [hook,key] of [['depth','depth'],['dependency','dependency'],['partner-dependency','partner_dependency']]){assert.equal(typeof row[key],'number');assert(Number.isFinite(row[key]));assert.equal(await card.locator('[data-agency-'+hook+']').textContent(),(row[key]*100).toFixed(1)+'%','Exact native trade '+key+' to displayed precision');}
    evidence.trade.push({partner:row.partner,text:body,readings:await cardReadings(card,'Trade '+row.partner+' at '+width)});
  }
  for(const row of board.conflicts){
    const card=section.locator('[data-agency-conflict='+quoted(row.conflict_id)+']'),body=await card.innerText();
    for(const value of [row.theatre,row.started,row.side_label,row.rung_name,row.shooting_label,...row.allies.map(p=>p.name),...row.opponents.map(p=>p.name)])assert(body.includes(value),'Native conflict field missing: '+value);
    for(const [hook,value] of [['theatre',row.theatre],['since',row.started],['side',row.side_label],['rung',row.rung==null?'No commitment':row.rung+' · '+row.rung_name],['status',row.shooting_label]])assert.equal(await card.locator('[data-agency-'+hook+']').textContent(),value,'Exact native conflict '+hook);
    assert.equal(await card.locator('[data-agency-open-conflict='+quoted(row.conflict_id)+']').count(),1);
    evidence.conflicts.push({conflict_id:row.conflict_id,text:body,readings:await cardReadings(card,'Conflict '+row.conflict_id+' at '+width)});
  }
  assert.equal(await section.locator('[data-agency-total-upkeep]').textContent(),board.upkeep.next_step_label,'Exact native total upkeep estimate');
  await readable(page.locator('#agencyClose'),'Close decisions at '+width);
  await within(page,'#agencyCommitments','Commitments at '+width);await within(page,'#agencyPanel','Decisions at '+width);return evidence;
}
async function cardReadings(card,label){
  const readings=[];
  for(const leaf of await card.locator('h3,h4,p,dt,dd,button').all()){
    if(!await leaf.isVisible())continue;
    readings.push(await readable(leaf,label));
  }
  return readings;
}
async function callContext(page,quote,width){
  await page.setViewportSize({width,height:width===1440?1000:844});
  const native=quote.call_context,context=page.locator('#agencyCallContext');assert(native&&native.conflict,'Fixture needs matched native call context');
  const body=await context.innerText();
  for(const text of [native.requester_name,native.requested_rung_name,native.status_label,native.conflict.theatre,native.conflict.started,...native.conflict.defenders.map(p=>p.name),...native.conflict.opponents.map(p=>p.name),native.note])assert(body.includes(text),'Native call context missing: '+text);
  for(const [hook,value] of [['rung',native.requested_rung+' · '+native.requested_rung_name],['theatre',native.conflict.theatre],['since',native.conflict.started]])assert.equal(await context.locator('[data-agency-'+hook+']').textContent(),value,'Exact native call context '+hook);
  const readings=await cardReadings(context,'Call context at '+width);
  await within(page,'#agencyCallContext','Call context at '+width);return {width,text:body,readings};
}
async function requestLink(page,id,telemetry){
  const previews=telemetry.previews.length,commands=telemetry.commands.length;
  await page.locator('#agencyCommitments [data-agency-commitment-reply='+quoted(id)+']').click();
  const card=page.locator('#agencyInbox [data-agency-offer='+quoted(id)+']');
  assert.equal(await card.evaluate(e=>document.activeElement===e),true,'Neutral link focuses its exact inbox request');
  assert.equal(telemetry.previews.length,previews,'Neutral View request must not choose an answer');
  assert.equal(telemetry.commands.length,commands,'Neutral View request must not send a decision');
  assert.equal(await page.locator('#agencyReview').count(),0);
  return {id,focused:true,previews_sent:0,commands_sent:0};
}
async function review(page,command,manifest){
  const offer=manifest.before_agency.offers.find(o=>o.id===command.offer);assert(offer);
  const response=page.waitForResponse(r=>route(r,'/api/decisions/preview'));
  await page.locator('#agencyInbox [data-agency-'+(command.accept?'accept':'decline')+'='+quoted(command.offer)+']').click();
  const r=await response;assert(r.ok());const quote=await r.json();await page.waitForFunction(()=>AGENCY.review&&!AGENCY.review.loading);
  assert.equal(quote.valid,true);assert.deepEqual(quote.command,command);assert.equal(typeof quote.review_token,'string');assert(quote.review_token);
  const native=manifest.quotes.find(q=>q.command.offer===command.offer&&q.command.accept===command.accept);assert(native);
  assert.deepEqual(stableQuote(quote),stableQuote(native),'Served quote must match independent native fixture execution');
  assert.equal(await page.locator('#agencyReviewTitle').textContent(),quote.title);assert(quote.title.includes(offer.from_name)&&quote.title.includes(offer.title));
  assert(quote.description.includes('Reply before '+offer.expires));
  assert.equal(await page.locator('#agencyReview [data-agency-confirm]').isEnabled(),true);
  await reviewUI.exactChanges(page,'agency',quote);return quote;
}
async function campaigns(page){
  if(await page.locator('#agencyPanel').isVisible())await page.locator('#agencyClose').click();
  if(await page.locator('#app').isVisible()){
    if(await page.locator('.arc-time-menu').getAttribute('open')===null)await page.locator('.arc-time-menu > summary').click();
    await page.locator('#campaignsBtn').click();
  }
  if(!await page.locator('#savedCampaigns').isVisible())await page.locator('#openSavesBtn').click();
  await page.locator('#savedCampaigns').waitFor();
}
function stateFacts(state,manifest,prefix){
  assert.equal(state.player,manifest.player);assert.equal(state.date,prefix==='before'?manifest.date:manifest[prefix+'_date']);
  assert.deepEqual(state.agency,manifest[prefix+'_agency'],'Exact pending offers, deadlines, policy and reply ledger');
  assert.deepEqual(state.log,manifest[prefix+'_log'],'Exact native dispatches');assert.equal(state.dispatch_count,manifest[prefix+'_dispatch_count']);
}
async function main(){
  assert(process.env.SPHERES_BINARY&&process.env.SPHERES_S10E_COMMITMENT_FIXTURE,'Set built SPHERES_BINARY and exported SPHERES_S10E_COMMITMENT_FIXTURE manifest path');
  const expected=process.env.SPHERES_EXPECTED_REVISION||'';assert.match(expected,/^[a-f0-9]{40}$/);
  const git=args=>cp.execFileSync('git',['-c','core.longpaths=true',...args],{cwd:root,windowsHide:true,maxBuffer:32*1024*1024});
  const revision=git(['rev-parse','HEAD']).toString().trim();assert.equal(git(['status','--porcelain']).toString().trim(),'','Require a clean committed checkout');
  assert.equal(git(['diff','--name-only',expected,revision,'--','spheres-sim','spheres-cli','spheres-web','Cargo.toml','Cargo.lock']).toString().trim(),'','Driver must preserve pinned runtime source');
  const binary=regular(path.resolve(process.env.SPHERES_BINARY)),binaryHash=audit.fileHash(binary);
  const manifestPath=regular(path.resolve(process.env.SPHERES_S10E_COMMITMENT_FIXTURE)),fixtureDir=path.dirname(manifestPath);
  const manifest=JSON.parse(fs.readFileSync(manifestPath,'utf8'));assert.equal(manifest.fixture,'s10e-authored-native-diplomatic-commitments');assert.equal(manifest.version,1);
  assert.equal(manifest.compiled_revision,git(['rev-parse','--short=12',expected]).toString().trim(),'Native fixture must come from the pinned clean runtime');
  assert.equal(manifest.days_advanced,1);assert.equal(manifest.reply_days_advanced,0);assert.equal(manifest.player,'France');
  const beforeFile=nativeFile(fixtureDir,manifest.before_file),expectedFile=nativeFile(fixtureDir,manifest.expected_after_file),dayFile=nativeFile(fixtureDir,manifest.expected_day_file);
  const fixtureHashes={manifest:audit.fileHash(manifestPath),before:audit.fileHash(beforeFile),expected_after:audit.fileHash(expectedFile),expected_day:audit.fileHash(dayFile)};
  assert.equal(manifest.before_agency.offers.length,2);
  assert.equal(manifest.before_agency.commitments.defense_pacts.length,2);
  assert.equal(manifest.before_agency.commitments.trade_agreements.length,1);
  assert.equal(manifest.before_agency.commitments.conflicts.length,0);
  assert.equal(manifest.after_agency.commitments.conflicts.length,1);
  assert.equal(manifest.after_agency.commitments.conflicts[0].conflict_id,manifest.conflict_id);
  assert.equal(manifest.after_agency.commitments.conflicts[0].side,'defending');
  assert.equal(manifest.after_agency.commitments.conflicts[0].rung,2);
  const output=path.resolve(process.env.SPHERES_COMMITMENT_OUTPUT||path.join(root,'artifacts/browser-diplomatic-commitments-ci'));
  fs.mkdirSync(output,{recursive:true});const out=fs.mkdtempSync(path.join(output,'authored-france-')),run=path.join(out,'server');fs.mkdirSync(run);fs.mkdirSync(path.join(run,'saves'));
  fs.copyFileSync(beforeFile,path.join(run,'saves',SLOT+'.json'),fs.constants.COPYFILE_EXCL);
  const port=await freePort(),url='http://127.0.0.1:'+port;
  const server=cp.spawn(binary,['--port',String(port),'--no-open'],{cwd:run,windowsHide:true,stdio:['ignore','pipe','pipe']});
  const log=fs.createWriteStream(path.join(out,'server.log'));server.stdout.pipe(log);server.stderr.pipe(log);
  let browser,page,launchError,stage='launch';server.on('error',error=>{launchError=error;});
  const e={passed:false,run,url,started_utc:new Date().toISOString(),scope:manifest.scope,fixture:{manifest_path:manifestPath,source:manifest,sha256:fixtureHashes},test_source:{revision,driver_sha256:audit.fileHash(__filename),review_assertions_sha256:audit.fileHash(require.resolve('./government-review-assertions.cjs')),runtime_source_equal:true},screenshots:[],commands:[],previews:[],advances:[],errors:[],checks:[]};
  const write=()=>fs.writeFileSync(path.join(out,'result.json'),JSON.stringify(e,null,2)+'\n');
  const telemetry=(event,details)=>fs.appendFileSync(path.join(out,'progress.jsonl'),JSON.stringify({utc:new Date().toISOString(),stage,event,...details})+'\n');
  const shot=async name=>{await page.screenshot({path:path.join(out,name+'.png')});e.screenshots.push(name+'.png');};audit.configure(out,telemetry);
  try{
    const until=Date.now()+20000;for(;;){if(launchError)throw launchError;assert.equal(server.exitCode,null);try{const r=await fetch(url+'/api/build',{headers:{Connection:'close'},signal:AbortSignal.timeout(2000)});await r.arrayBuffer();if(r.ok)break;}catch(error){if(Date.now()>=until)throw error;}assert(Date.now()<until,'Startup exceeded 20 seconds');await new Promise(resolve=>setTimeout(resolve,100));}
    browser=await chromium.launch({headless:true,...(process.env.SPHERES_BROWSER_CHANNEL?{channel:process.env.SPHERES_BROWSER_CHANNEL}:{})});
    e.browser_version=browser.version();const context=await browser.newContext({viewport:{width:1440,height:1000},reducedMotion:'reduce'});page=await context.newPage();page.setDefaultTimeout(30000);
    page.on('pageerror',error=>e.errors.push(error.message));page.on('request',r=>{if(r.method()==='POST'){const p=new URL(r.url()).pathname;if(p==='/api/command')e.commands.push(r.postDataJSON());if(p==='/api/decisions/preview')e.previews.push(r.postDataJSON());if(p==='/api/advance')e.advances.push(r.postDataJSON());}});
    e.build=await integrated.verifyBuild({page,url,root,run,binary});
    for(const name of ['agency-ui.js','agency.css']){
      const checkout=fs.readFileSync(path.join(root,'spheres-web/ui',name)),committed=git(['show',expected+':spheres-web/ui/'+name]);assert(Buffer.from(checkout.toString('utf8').replace(/\r\n/g,'\n')).equals(committed));
      const r=await page.request.get(url+'/'+name);try{assert(r.ok());const served=await r.body();assert(served.equals(checkout));e.build.assets[name]={served_sha256:hash(served),checkout_sha256:hash(checkout),committed_sha256:hash(committed)};}finally{await r.dispose();}
    }
    const authored=audit.inspect(beforeFile,'France'),nativeExpected=audit.inspect(expectedFile,'France'),nativeDay=audit.inspect(dayFile,'France');
    stage='ordinary Load of disclosed fixture';await page.goto(url,{waitUntil:'domcontentloaded'});await page.locator('#campaignHome').waitFor();await page.waitForFunction(()=>!!SESSION.live?.session_id);
    const hasLivePlayer=await page.evaluate(()=>!!SESSION.live?.player);await campaigns(page);await page.locator('#saveSlots').selectOption(SLOT);
    const loading=page.waitForResponse(r=>route(r,'/api/load'));await page.locator('#loadBtn').click();if(hasLivePlayer)await page.locator('#campaignConfirmAccept').click();assert((await loading).ok());
    await page.locator('#app').waitFor();await idle(page);assert.equal(await page.evaluate(()=>clock.running),false);
    const initial=await get(page,url,'/api/state');stateFacts(initial,manifest,'before');e.capabilities=await integrated.capabilities({page,url,player:'France'});
    const before=await capture(page,url,run,'s10e-before-review');audit.compare(before,authored,'Ordinary Load preserves the complete authored native world');
    stage='standing commitments at desktop and both phone widths';await page.locator('#agencyBtn').click();await page.locator('#agencyPanel').waitFor();e.initial_layout=[];
    for(const width of [1440,390,320]){e.initial_layout.push(await commitmentDesk(page,initial.agency,width));await page.locator('#agencyCommitments h3').first().scrollIntoViewIfNeeded();await shot('standing-commitments-'+width);}
    e.checks.push('Native saved pacts, dates, upkeep and trade roster display without hidden text or controls at 1440, 390 and 320 px');
    stage='neutral request link and both replies reviewed then cancelled';e.cancelled_reviews=[];e.request_links=[];e.context_layout=[];
    for(const native of manifest.quotes){
      e.request_links.push(await requestLink(page,native.command.offer,e));
      await page.setViewportSize({width:1440,height:1000});const quote=await review(page,native.command,manifest);e.cancelled_reviews.push(quote);
      if(native.command.accept){
        assert(quote.changes.some(c=>c.label.startsWith('Participation in ')&&c.before==='Outside this conflict'&&c.after==='Defending coalition'));
        assert(quote.changes.some(c=>c.label.startsWith('Commitment in ')&&c.before==='No commitment'&&c.after.startsWith('2 · ')));
        for(const width of [1440,390,320]){e.context_layout.push(await callContext(page,quote,width));await page.locator('#agencyCallContext h4').first().scrollIntoViewIfNeeded();await shot('call-context-'+width);}
        e.effect_layout=await reviewUI.mobileChanges(page,'agency',quote,async width=>{await within(page,'#agencyReview','Native call effects '+width);await shot('call-effects-'+width);});
      }
      await page.locator('#agencyReview [data-agency-review-cancel]').click();await page.locator('#agencyReview').waitFor({state:'hidden'});assert.equal(e.commands.length,0);assert.deepEqual(e.advances,[]);
    }
    stateFacts(await get(page,url,'/api/state'),manifest,'before');audit.compare(await capture(page,url,run,'s10e-after-cancel'),before,'Neutral commitment links, both defense-call reviews and cancellations preserve the complete native world');
    e.checks.push('Neutral commitment link focuses the exact existing request without choosing an answer; both previews equal independent native outcomes; cancelling preserves the complete world');
    stage='one confirmed guaranteed call';await page.setViewportSize({width:390,height:844});e.request_links.push(await requestLink(page,manifest.command.offer,e));const quote=await review(page,manifest.command,manifest);e.confirmed_review=quote;
    const applying=page.waitForResponse(r=>route(r,'/api/command'));await page.locator('#agencyReview [data-agency-confirm]').click();const applied=await applying;assert(applied.ok());assert.deepEqual((await applied.json()).errors,[]);await idle(page);
    const after=await get(page,url,'/api/state');stateFacts(after,manifest,'after');assert.equal(e.commands.length,1);assert.equal(e.commands[0].review_token,quote.review_token);assert.equal(e.commands[0].review_kind,'decisions');assert.deepEqual(e.commands[0].commands,[manifest.command]);
    assert.equal(await page.locator('#agencyInbox [data-agency-offer='+quoted(manifest.command.offer)+']').count(),0);
    const record=after.agency.history.find(h=>h.offer.id===manifest.command.offer);assert(record&&record.outcome==='accepted');
    assert((await page.locator('#agencyBody').innerText()).includes('Request #'+record.offer.id+': accepted'));
    await page.waitForFunction(()=>document.activeElement===document.getElementById('agencyStatus'));
    assert((await page.locator('#agencyStatus').innerText()).trim());
    e.settled_notice=await page.locator('#agencyStatus').evaluate(e=>{const r=e.getBoundingClientRect();return {focused:document.activeElement===e,visible:r.top>=0&&r.bottom<=innerHeight&&r.left>=0&&r.right<=innerWidth,text:e.textContent};});
    assert(e.settled_notice.focused&&e.settled_notice.visible,'Result must receive focus and remain visible without driver scrolling');await shot('reply-recorded-390');
    const afterReply=await capture(page,url,run,'s10e-after-reply');audit.compare(afterReply,nativeExpected,'One UI confirmation equals independently executed native acceptance including coalition, posture, reputation, relations, treaties and reply ledger');
    e.checks.push('Exactly one protected UI order removes its pending offer and records the native accepted result; complete native world equals native exporter execution');
    e.joined_layout=[];for(const width of [1440,390,320]){e.joined_layout.push(await commitmentDesk(page,after.agency,width));await page.locator('#agencyCommitments [data-agency-conflict]').scrollIntoViewIfNeeded();await shot('joined-commitments-'+width);}
    stage='open the exact joined conflict without changing orders';await page.setViewportSize({width:1440,height:1000});
    await page.locator('#agencyCommitments [data-agency-open-conflict='+quoted(manifest.conflict_id)+']').click();await page.locator('#sheet').waitFor();
    assert.equal(await page.locator('#agencyPanel').isVisible(),false);assert.equal(await page.evaluate(()=>selectedWar),manifest.conflict_id);
    assert.equal(await page.locator('#sheet .head h2').textContent(),after.agency.commitments.conflicts[0].theatre);
    assert.equal(e.commands.length,1);assert.deepEqual(e.advances,[]);await shot('open-exact-conflict-1440');await page.locator('#sheet .head .close').click();
    e.checks.push('Open conflict reaches the existing sheet for the exact saved conflict without sending an order');
    stage='named Save, cancelled Load, Load and Continue';await page.setViewportSize({width:1440,height:1000});await campaigns(page);await page.locator('#saveName').fill(SAVED);
    const saving=page.waitForResponse(r=>route(r,'/api/save'));await page.locator('#saveNamedBtn').click();assert((await saving).ok());await page.locator('#saveSlots option[value='+quoted(SAVED)+']').waitFor({state:'attached'});await page.locator('#saveSlots').selectOption(SAVED);
    await page.locator('#loadBtn').click();await page.locator('#campaignConfirmCancel').click();stateFacts(await get(page,url,'/api/state'),manifest,'after');audit.compare(await capture(page,url,run,'s10e-after-load-cancel'),afterReply,'Cancelled Load preserves the complete native accepted world');
    const reloading=page.waitForResponse(r=>route(r,'/api/load'));await page.locator('#loadBtn').click();await page.locator('#campaignConfirmAccept').click();assert((await reloading).ok());await page.locator('#app').waitFor();await idle(page);
    const loaded=await get(page,url,'/api/state');assert.notEqual(loaded.session_id,after.session_id);stateFacts(loaded,manifest,'after');audit.compare(await capture(page,url,run,'s10e-after-load'),afterReply,'Named Save/Load retains the complete native world and diplomatic ledger');
    await page.reload({waitUntil:'domcontentloaded'});await page.locator('#continueBtn').click();await page.locator('#app').waitFor();await idle(page);stateFacts(await get(page,url,'/api/state'),manifest,'after');audit.compare(await capture(page,url,run,'s10e-after-continue'),afterReply,'Continue retains the complete native world and diplomatic ledger');
    await page.locator('#agencyBtn').click();await page.locator('#agencyPanel').waitFor();e.continued_layout=await commitmentDesk(page,manifest.after_agency,320);await shot('continued-commitments-320');
    assert.equal(e.commands.length,1);assert.deepEqual(e.advances,[]);
    stage='one ordinary day from continued joined state';await page.locator('#agencyClose').click();await page.setViewportSize({width:1440,height:1000});
    const advancing=page.waitForResponse(r=>route(r,'/api/advance'));await page.locator('#stepBtn').click();const advanced=await advancing;assert(advanced.ok());assert.equal(advanced.request().postDataJSON().days,1);const advancedState=await advanced.json();assert(!advancedState.advance_pending);await idle(page);
    const day=await get(page,url,'/api/state');stateFacts(day,manifest,'day');assert.equal(e.advances.length,1);assert.equal(e.advances[0].days,1);assert.deepEqual(e.advances[0].commands,[]);assert.equal(await page.evaluate(()=>clock.running),false);
    audit.compare(await capture(page,url,run,'s10e-after-day'),nativeDay,'One ordinary +1 DAY after Save/Load/Continue equals independent native one-day oracle including joined posture, pending deadline, upkeep and all other world fields');
    await page.locator('#agencyBtn').click();await page.locator('#agencyPanel').waitFor();e.day_layout=await commitmentDesk(page,day.agency,390);await shot('commitments-after-one-day-390');
    assert.equal(e.commands.length,1);assert.equal(e.advances.length,1);assert.deepEqual(e.errors,[]);assert.equal(audit.fileHash(binary),binaryHash);
    assert.deepEqual({manifest:audit.fileHash(manifestPath),before:audit.fileHash(beforeFile),expected_after:audit.fileHash(expectedFile),expected_day:audit.fileHash(dayFile)},fixtureHashes);
    assert.equal(git(['rev-parse','HEAD']).toString().trim(),revision);assert.equal(git(['status','--porcelain']).toString().trim(),'');
    e.checks.push('Ordinary named save, cancelled load, Load and Continue retain complete native state; a single subsequent +1 DAY exactly matches the independent native continuation oracle');
    e.saved_slot=SAVED;e.days_advanced=1;e.reply_days_advanced=0;e.final_capture='captures/s10e-after-day.json';e.passed=true;e.finished_utc=new Date().toISOString();write();console.log(JSON.stringify({passed:true,result:path.join(out,'result.json')}));
  }catch(error){e.failed_stage=stage;e.failure=String(error.stack||error);write();if(page)try{await page.screenshot({path:path.join(out,'failure.png')});fs.writeFileSync(path.join(out,'failure.html'),await page.content());}catch{}throw error;}
  finally{if(browser)await browser.close();server.kill();log.end();}
}
main().catch(error=>{console.error(error);process.exitCode=1;});
