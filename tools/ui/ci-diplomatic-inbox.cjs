// S10.d: actual browser controls against an explicitly authored native fixture.
// The driver never rewrites a world, injects a response, advances a day or
// touches an existing review server. Native export provides both comparison worlds.
const assert=require('node:assert/strict'),fs=require('node:fs'),path=require('node:path');
const cp=require('node:child_process'),net=require('node:net'),crypto=require('node:crypto');
const {chromium}=require('playwright'),integrated=require('./ci-integrated.cjs');
const audit=require('./supplier-archive-audit.cjs'),reviewUI=require('./government-review-assertions.cjs');
const root=path.resolve(__dirname,'../..'),quoted=value=>JSON.stringify(String(value));
const hash=value=>crypto.createHash('sha256').update(value).digest('hex');
const route=(response,p)=>new URL(response.url()).pathname===p&&response.request().method()==='POST';
const deadlineOrder=(a,b)=>a.expires<b.expires?-1:a.expires>b.expires?1:a.id-b.id;
const stableQuote=quote=>{const {session_id,review_token,...stable}=quote;return stable;};
const SLOT='s10d-authored-inbox',SAVED='s10d-reviewed-inbox';
const CAPTURES=new Set(['s10d-before-review','s10d-after-cancel','s10d-after-reply','s10d-after-load-cancel','s10d-after-load','s10d-after-continue']);
async function freePort(){const s=net.createServer();await new Promise(resolve=>s.listen(0,'127.0.0.1',resolve));const port=s.address().port;await new Promise(resolve=>s.close(resolve));return port;}
async function get(page,url,p){const r=await page.request.get(url+p);try{assert(r.ok(),p);return await r.json();}finally{await r.dispose();}}
async function idle(page){await page.waitForFunction(()=>!SESSION.busy&&!COMMAND_CHANNEL.busy&&!COMMAND_CHANNEL.pending&&!advancing&&!pendingAdvance&&!AGENCY.busy);}
function regular(file){assert(fs.lstatSync(file).isFile()&&!fs.lstatSync(file).isSymbolicLink());assert.equal(fs.realpathSync(file),file);return file;}
function nativeFile(directory,name){assert(['before.json','expected-after.json'].includes(name));return regular(path.join(directory,name));}
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
async function inbox(page,agency,width){
  await page.setViewportSize({width,height:width===1440?1000:844});
  const sorted=agency.offers.slice().sort(deadlineOrder);
  assert.deepEqual(await page.locator('#agencyInbox [data-agency-offer]').evaluateAll(rows=>rows.map(e=>Number(e.dataset.agencyOffer))),sorted.map(o=>o.id));
  assert.equal(await page.locator('#agencyBody > section').first().getAttribute('id'),'agencyInbox','Pending replies must precede campaign aims');
  assert((await page.locator('#agencyInbox .agency-inbox-summary').innerText()).includes(sorted[0].expires));
  const readings=[];
  for(const offer of sorted){
    const card=page.locator('#agencyInbox [data-agency-offer='+quoted(offer.id)+']');
    assert.equal(await card.locator('h4').textContent(),offer.from_name+' · '+offer.title);
    const deadline=await card.locator('.agency-deadline').textContent();assert(deadline.startsWith('Reply before '+offer.expires+' · '));
    assert(deadline.includes(offer.days_remaining+' days remaining'));
    assert.equal(await card.locator('[data-agency-accept]').textContent(),'Review acceptance');
    assert.equal(await card.locator('[data-agency-decline]').textContent(),'Review decline');
    readings.push({id:offer.id,title:await readable(card.locator('h4'),'Offer title at '+width),deadline:await readable(card.locator('.agency-deadline'),'Offer deadline at '+width),accept:await readable(card.locator('[data-agency-accept]'),'Acceptance button at '+width),decline:await readable(card.locator('[data-agency-decline]'),'Decline button at '+width)});
    await within(page,'#agencyInbox [data-agency-offer='+quoted(offer.id)+']','Offer at '+width);
  }
  await within(page,'#agencyPanel','Reply desk at '+width);return {width,ordered_ids:sorted.map(o=>o.id),readings};
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
  assert.equal(state.player,manifest.player);assert.equal(state.date,manifest.date);
  assert.deepEqual(state.agency,manifest[prefix+'_agency'],'Exact pending offers, deadlines, policy and reply ledger');
  assert.deepEqual(state.log,manifest[prefix+'_log'],'Exact native dispatches');assert.equal(state.dispatch_count,manifest[prefix+'_dispatch_count']);
}
async function main(){
  assert(process.env.SPHERES_BINARY&&process.env.SPHERES_S10D_INBOX_FIXTURE,'Set built SPHERES_BINARY and exported SPHERES_S10D_INBOX_FIXTURE manifest path');
  const expected=process.env.SPHERES_EXPECTED_REVISION||'';assert.match(expected,/^[a-f0-9]{40}$/);
  const git=args=>cp.execFileSync('git',['-c','core.longpaths=true',...args],{cwd:root,windowsHide:true,maxBuffer:32*1024*1024});
  const revision=git(['rev-parse','HEAD']).toString().trim();assert.equal(git(['status','--porcelain']).toString().trim(),'','Require a clean committed checkout');
  assert.equal(git(['diff','--name-only',expected,revision,'--','spheres-sim','spheres-cli','spheres-web','Cargo.toml','Cargo.lock']).toString().trim(),'','Driver must preserve pinned runtime source');
  const binary=regular(path.resolve(process.env.SPHERES_BINARY)),binaryHash=audit.fileHash(binary);
  const manifestPath=regular(path.resolve(process.env.SPHERES_S10D_INBOX_FIXTURE)),fixtureDir=path.dirname(manifestPath);
  const manifest=JSON.parse(fs.readFileSync(manifestPath,'utf8'));assert.equal(manifest.fixture,'s10d-authored-native-diplomatic-inbox');assert.equal(manifest.version,1);
  assert.equal(manifest.compiled_revision,git(['rev-parse','--short=12',expected]).toString().trim(),'Native fixture must come from the pinned clean runtime');
  assert.equal(manifest.days_advanced,0);assert.equal(manifest.player,'France');
  const beforeFile=nativeFile(fixtureDir,manifest.before_file),expectedFile=nativeFile(fixtureDir,manifest.expected_after_file);
  const fixtureHashes={manifest:audit.fileHash(manifestPath),before:audit.fileHash(beforeFile),expected_after:audit.fileHash(expectedFile)};
  assert.equal(manifest.before_agency.offers.length,3);const sourceOrder=manifest.before_agency.offers.map(o=>o.id);
  const expectedOrder=manifest.before_agency.offers.slice().sort(deadlineOrder).map(o=>o.id);
  assert.notDeepEqual(sourceOrder,expectedOrder,'Fixture must challenge chronological insertion order');
  const output=path.resolve(process.env.SPHERES_INBOX_OUTPUT||path.join(root,'artifacts/browser-diplomatic-inbox-ci'));
  fs.mkdirSync(output,{recursive:true});const out=fs.mkdtempSync(path.join(output,'authored-france-')),run=path.join(out,'server');fs.mkdirSync(run);fs.mkdirSync(path.join(run,'saves'));
  fs.copyFileSync(beforeFile,path.join(run,'saves',SLOT+'.json'),fs.constants.COPYFILE_EXCL);
  const port=await freePort(),url='http://127.0.0.1:'+port;
  const server=cp.spawn(binary,['--port',String(port),'--no-open'],{cwd:run,windowsHide:true,stdio:['ignore','pipe','pipe']});
  const log=fs.createWriteStream(path.join(out,'server.log'));server.stdout.pipe(log);server.stderr.pipe(log);
  let browser,page,launchError,stage='launch';server.on('error',error=>{launchError=error;});
  const e={passed:false,run,url,started_utc:new Date().toISOString(),scope:manifest.scope,fixture:{manifest_path:manifestPath,source:manifest,sha256:fixtureHashes},test_source:{revision,driver_sha256:audit.fileHash(__filename),review_assertions_sha256:audit.fileHash(require.resolve('./government-review-assertions.cjs')),runtime_source_equal:true},screenshots:[],commands:[],advances:[],errors:[],checks:[]};
  const write=()=>fs.writeFileSync(path.join(out,'result.json'),JSON.stringify(e,null,2)+'\n');
  const telemetry=(event,details)=>fs.appendFileSync(path.join(out,'progress.jsonl'),JSON.stringify({utc:new Date().toISOString(),stage,event,...details})+'\n');
  const shot=async name=>{await page.screenshot({path:path.join(out,name+'.png')});e.screenshots.push(name+'.png');};audit.configure(out,telemetry);
  try{
    const until=Date.now()+20000;for(;;){if(launchError)throw launchError;assert.equal(server.exitCode,null);try{const r=await fetch(url+'/api/build',{headers:{Connection:'close'},signal:AbortSignal.timeout(2000)});await r.arrayBuffer();if(r.ok)break;}catch(error){if(Date.now()>=until)throw error;}assert(Date.now()<until,'Startup exceeded 20 seconds');await new Promise(resolve=>setTimeout(resolve,100));}
    browser=await chromium.launch({headless:true,...(process.env.SPHERES_BROWSER_CHANNEL?{channel:process.env.SPHERES_BROWSER_CHANNEL}:{})});
    e.browser_version=browser.version();const context=await browser.newContext({viewport:{width:1440,height:1000},reducedMotion:'reduce'});page=await context.newPage();page.setDefaultTimeout(30000);
    page.on('pageerror',error=>e.errors.push(error.message));page.on('request',r=>{if(r.method()==='POST'){const p=new URL(r.url()).pathname;if(p==='/api/command')e.commands.push(r.postDataJSON());if(p==='/api/advance')e.advances.push(r.postDataJSON());}});
    e.build=await integrated.verifyBuild({page,url,root,run,binary});
    for(const name of ['agency-ui.js','agency.css']){
      const checkout=fs.readFileSync(path.join(root,'spheres-web/ui',name)),committed=git(['show',expected+':spheres-web/ui/'+name]);assert(Buffer.from(checkout.toString('utf8').replace(/\r\n/g,'\n')).equals(committed));
      const r=await page.request.get(url+'/'+name);try{assert(r.ok());const served=await r.body();assert(served.equals(checkout));e.build.assets[name]={served_sha256:hash(served),checkout_sha256:hash(checkout),committed_sha256:hash(committed)};}finally{await r.dispose();}
    }
    const authored=audit.inspect(beforeFile,'France'),nativeExpected=audit.inspect(expectedFile,'France');
    stage='ordinary Load of disclosed fixture';await page.goto(url,{waitUntil:'domcontentloaded'});await page.locator('#campaignHome').waitFor();await page.waitForFunction(()=>!!SESSION.live?.session_id);
    const hasLivePlayer=await page.evaluate(()=>!!SESSION.live?.player);await campaigns(page);await page.locator('#saveSlots').selectOption(SLOT);
    const loading=page.waitForResponse(r=>route(r,'/api/load'));await page.locator('#loadBtn').click();if(hasLivePlayer)await page.locator('#campaignConfirmAccept').click();assert((await loading).ok());
    await page.locator('#app').waitFor();await idle(page);assert.equal(await page.evaluate(()=>clock.running),false);
    const initial=await get(page,url,'/api/state');stateFacts(initial,manifest,'before');e.capabilities=await integrated.capabilities({page,url,player:'France'});
    const before=await capture(page,url,run,'s10d-before-review');audit.compare(before,authored,'Ordinary Load preserves the complete authored native world');
    stage='deadline desk at desktop and both phone widths';await page.locator('#agencyBtn').click();await page.locator('#agencyPanel').waitFor();e.inbox_layout=[];
    for(const width of [1440,390,320]){e.inbox_layout.push(await inbox(page,initial.agency,width));await page.locator('#agencyInbox .agency-inbox-summary').scrollIntoViewIfNeeded();await shot('reply-desk-'+width);}
    e.checks.push('Unsorted native offers render nearest deadline first with native dates and explicit review controls at 1440, 390 and 320 px');
    stage='all six native choices reviewed then cancelled';e.cancelled_reviews=[];
    const longest=manifest.quotes.slice().sort((a,b)=>Math.max(...b.changes.map(c=>Math.max(c.before.length,c.after.length)))-Math.max(...a.changes.map(c=>Math.max(c.before.length,c.after.length))))[0];
    assert(Math.max(...longest.changes.map(c=>Math.max(c.before.length,c.after.length)))>40,'Fixture needs a real prose effect, not numeric-only cards');
    for(const native of manifest.quotes){
      await page.setViewportSize({width:1440,height:1000});const quote=await review(page,native.command,manifest);e.cancelled_reviews.push(quote);
      if(native.command.offer===longest.command.offer&&native.command.accept===longest.command.accept){
        await within(page,'#agencyReview','Longest native review desktop');await shot('long-native-effects-1440');
        e.long_effect_layout=await reviewUI.mobileChanges(page,'agency',quote,async width=>{await within(page,'#agencyReview','Longest native review '+width);await shot('long-native-effects-'+width);});
      }
      await page.locator('#agencyReview [data-agency-review-cancel]').click();await page.locator('#agencyReview').waitFor({state:'hidden'});assert.equal(e.commands.length,0);
    }
    stateFacts(await get(page,url,'/api/state'),manifest,'before');audit.compare(await capture(page,url,run,'s10d-after-cancel'),before,'All six treaty and defense-call reviews and cancellations leave native world unchanged');
    e.checks.push('All six actual choice previews equal the native exporter, including unchanged world after review and cancellation; long prose reads at 320 px');
    stage='one confirmed treaty reply';await page.setViewportSize({width:390,height:844});const quote=await review(page,manifest.command,manifest);e.confirmed_review=quote;
    const applying=page.waitForResponse(r=>route(r,'/api/command'));await page.locator('#agencyReview [data-agency-confirm]').click();const applied=await applying;assert(applied.ok());assert.deepEqual((await applied.json()).errors,[]);await idle(page);
    const after=await get(page,url,'/api/state');stateFacts(after,manifest,'after');assert.equal(e.commands.length,1);assert.equal(e.commands[0].review_token,quote.review_token);assert.equal(e.commands[0].review_kind,'decisions');assert.deepEqual(e.commands[0].commands,[manifest.command]);
    assert.equal(await page.locator('#agencyInbox [data-agency-offer='+quoted(manifest.command.offer)+']').count(),0);
    const record=after.agency.history.find(h=>h.offer.id===manifest.command.offer);assert(record&&record.outcome==='accepted');
    assert((await page.locator('#agencyBody').innerText()).includes('Request #'+record.offer.id+': accepted'));
    assert((await page.locator('#agencyStatus').innerText()).trim());await page.locator('#agencyStatus').scrollIntoViewIfNeeded();await shot('reply-recorded-390');
    const afterReply=await capture(page,url,run,'s10d-after-reply');audit.compare(afterReply,nativeExpected,'One UI confirmation equals independently executed native acceptance including exact relation cap, trade depth and reply ledger');
    e.checks.push('Exactly one protected UI order removes its pending offer and records the native accepted result; complete native world equals native exporter execution');
    stage='named Save, cancelled Load, Load and Continue';await page.setViewportSize({width:1440,height:1000});await campaigns(page);await page.locator('#saveName').fill(SAVED);
    const saving=page.waitForResponse(r=>route(r,'/api/save'));await page.locator('#saveNamedBtn').click();assert((await saving).ok());await page.locator('#saveSlots option[value='+quoted(SAVED)+']').waitFor({state:'attached'});await page.locator('#saveSlots').selectOption(SAVED);
    await page.locator('#loadBtn').click();await page.locator('#campaignConfirmCancel').click();stateFacts(await get(page,url,'/api/state'),manifest,'after');audit.compare(await capture(page,url,run,'s10d-after-load-cancel'),afterReply,'Cancelled Load preserves the complete native accepted world');
    const reloading=page.waitForResponse(r=>route(r,'/api/load'));await page.locator('#loadBtn').click();await page.locator('#campaignConfirmAccept').click();assert((await reloading).ok());await page.locator('#app').waitFor();await idle(page);
    const loaded=await get(page,url,'/api/state');assert.notEqual(loaded.session_id,after.session_id);stateFacts(loaded,manifest,'after');audit.compare(await capture(page,url,run,'s10d-after-load'),afterReply,'Named Save/Load retains the complete native world and diplomatic ledger');
    await page.reload({waitUntil:'domcontentloaded'});await page.locator('#continueBtn').click();await page.locator('#app').waitFor();await idle(page);stateFacts(await get(page,url,'/api/state'),manifest,'after');audit.compare(await capture(page,url,run,'s10d-after-continue'),afterReply,'Continue retains the complete native world and diplomatic ledger');
    await page.locator('#agencyBtn').click();await page.locator('#agencyPanel').waitFor();e.continued_layout=await inbox(page,manifest.after_agency,320);await shot('continued-reply-desk-320');
    assert.equal(e.commands.length,1);assert.deepEqual(e.advances,[]);assert.deepEqual(e.errors,[]);assert.equal(audit.fileHash(binary),binaryHash);
    assert.deepEqual({manifest:audit.fileHash(manifestPath),before:audit.fileHash(beforeFile),expected_after:audit.fileHash(expectedFile)},fixtureHashes);
    assert.equal(git(['rev-parse','HEAD']).toString().trim(),revision);assert.equal(git(['status','--porcelain']).toString().trim(),'');
    e.checks.push('Ordinary named save, cancelled load, load and Continue retain pending deadlines, dated reply ledger and native dispatches; zero advance requests');
    e.saved_slot=SAVED;e.passed=true;e.finished_utc=new Date().toISOString();write();console.log(JSON.stringify({passed:true,result:path.join(out,'result.json')}));
  }catch(error){e.failed_stage=stage;e.failure=String(error.stack||error);write();if(page)try{await page.screenshot({path:path.join(out,'failure.png')});fs.writeFileSync(path.join(out,'failure.html'),await page.content());}catch{}throw error;}
  finally{if(browser)await browser.close();server.kill();log.end();}
}
main().catch(error=>{console.error(error);process.exitCode=1;});
