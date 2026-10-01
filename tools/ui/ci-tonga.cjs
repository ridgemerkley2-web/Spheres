// CI-only production browser journey. Never targets an existing server or save.
// Later dates are native-authored fixtures, not a simulated campaign duration.
const assert=require('node:assert/strict'),fs=require('node:fs'),path=require('node:path'),net=require('node:net'),cp=require('node:child_process'),crypto=require('node:crypto');
const {chromium}=require('playwright'),integrated=require('./ci-integrated.cjs'),contract=require('./ci-tonga-contract.cjs');
const root=path.resolve(__dirname,'../..'),out=path.join(root,'artifacts/tonga-browser-ci');
const hash=bytes=>crypto.createHash('sha256').update(bytes).digest('hex');
function git(args){return cp.execFileSync('git',args,{cwd:root,windowsHide:true}).toString().trim();}
async function port(){const s=net.createServer();await new Promise(r=>s.listen(0,'127.0.0.1',r));const p=s.address().port;await new Promise(r=>s.close(r));return p;}
async function main(){
 assert.equal(process.env.CI,'true','This native runner is reserved for disposable CI execution');
 const revision=process.env.SPHERES_EXPECTED_REVISION||git(['rev-parse','HEAD']);assert.equal(git(['rev-parse','HEAD']),revision);
 assert.equal(git(['status','--porcelain','--untracked-files=no']),'','Build and fixtures require the clean exact checkout');
 fs.mkdirSync(out,{recursive:true});const run=fs.mkdtempSync(path.join(out,'campaign-')),fixtures=path.join(run,'authored-fixtures');
 const command=['run','--locked','--release','-p','spheres-sim','--example','tonga_browser_fixtures','--',fixtures,revision];
 const logFd=fs.openSync(path.join(run,'fixture-export.log'),'wx');
 let exported;try{exported=cp.spawnSync('cargo',command,{cwd:root,windowsHide:true,stdio:['ignore',logFd,logFd],timeout:600000});}finally{fs.closeSync(logFd);}
 assert.equal(exported.status,0,'Native fixture export failed; see fixture-export.log');
 const manifest=JSON.parse(fs.readFileSync(path.join(fixtures,'manifest.json'),'utf8'));contract.fixtureManifest(manifest,revision);
 const provenance={revision,command:['cargo',...command],days_advanced:0,scope:manifest.scope,inputs:{},fixtures:{}};
 for(const file of ['spheres-sim/examples/tonga_browser_fixtures.rs','tools/ui/ci-tonga.cjs','tools/ui/ci-tonga-contract.cjs','.github/workflows/verify.yml']){const bytes=fs.readFileSync(path.join(root,file));provenance.inputs[file]={sha256:hash(bytes),bytes:bytes.length};}
 fs.mkdirSync(path.join(run,'saves'));
 for(const c of manifest.cases){const bytes=fs.readFileSync(path.join(fixtures,c.file));provenance.fixtures[c.name]={sha256:hash(bytes),bytes:bytes.length,date:c.date};fs.copyFileSync(path.join(fixtures,c.file),path.join(run,'saves',c.file),fs.constants.COPYFILE_EXCL);}
 fs.writeFileSync(path.join(out,'fixture-provenance.json'),JSON.stringify(provenance,null,2));
 const p=await port(),url=`http://127.0.0.1:${p}`,binary=path.resolve(process.env.SPHERES_BINARY||path.join(root,'target/release/spheres-web'+(process.platform==='win32'?'.exe':'')));
 const server=cp.spawn(binary,['--port',String(p),'--no-open'],{cwd:run,windowsHide:true,stdio:['ignore','pipe','pipe']});
 const log=fs.createWriteStream(path.join(run,'server.log'));server.stdout.pipe(log);server.stderr.pipe(log);
 let browser,page;const evidence={passed:false,stage:'server-start',scope:manifest.scope,revision,checks:[],commands:[],refusals:[],portraits:[]},errors=[],requests=[];
 try{
  const deadline=Date.now()+30000;for(;;){try{const r=await fetch(url+'/api/build',{signal:AbortSignal.timeout(1000)});await r.arrayBuffer();if(r.ok)break;}catch(_){}assert(server.exitCode===null&&Date.now()<deadline,'Disposable Tonga server did not start');await new Promise(r=>setTimeout(r,100));}
  browser=await chromium.launch({headless:true});page=await browser.newPage({viewport:{width:1440,height:1000},reducedMotion:'reduce'});page.setDefaultTimeout(30000);
  page.on('pageerror',e=>errors.push(e.message));page.on('request',r=>{if(r.method()==='POST')requests.push({path:new URL(r.url()).pathname,payload:r.postDataJSON()});});
  evidence.build=await integrated.verifyBuild({page,url,root,run,binary});
  const get=async route=>{const r=await page.request.get(url+route);assert(r.ok(),route);return r.json();};
  const state=()=>get('/api/state'),government=async()=>{const data=await get('/api/government?nation=Tonga');evidence.latest_government=data;return data;};
  let capture=0;
  async function archive(){const sequence=++capture,slot='tonga-proof-'+(sequence%2),r=await page.request.post(url+'/api/save',{data:{slot}});assert(r.ok());const text=contract.archiveText(fs.readFileSync(path.join(run,'saves',slot+'.json'),'utf8'));(evidence.archive_hashes??=[]).push({sequence,slot,sha256:hash(text)});return text;}
  async function openGovernment(){await page.waitForFunction(()=>S?.player==='Tonga'&&!SESSION.busy&&SESSION.live?.session_id===S.session_id);await page.locator('#govBtn').click();await page.locator('#govScreen').waitFor({state:'visible'});await page.locator('#gov-tab-institutions').click();await page.locator('[data-gov-section="institutions"]').waitFor();}
  async function load(slot){
   evidence.stage='load:'+slot;
   await page.goto(url,{waitUntil:'domcontentloaded'});await page.locator('#campaignHome').waitFor({state:'visible'});
   const live=await state();await page.waitForFunction(session=>SESSION.live?.session_id===session&&SESSION.live.player==='Tonga'&&!SESSION.busy,live.session_id);
   await page.locator('#openSavesBtn').click();
   await page.locator('#saveSlots option[value="'+slot+'"]').waitFor({state:'attached'});await page.locator('#saveSlots').selectOption(slot);await page.locator('#loadBtn').click();
   await page.locator('#campaignConfirmDialog').waitFor({state:'visible'});const response=page.waitForResponse(r=>new URL(r.url()).pathname==='/api/load'&&r.request().method()==='POST');
   await page.locator('#campaignConfirmAccept').click();const r=await response;assert(r.ok());assert.equal(r.request().postDataJSON().slot,slot);
   await page.locator('#app').waitFor({state:'visible'});assert.equal((await state()).player,'Tonga');await openGovernment();
  }
  async function visibleCards(data,{opening=false}={}){
   const b=contract.board(data,data.party_leadership.institutional_leadership.date);
   evidence.stage='visible-overview-identities:'+b.date;
   // Institutions deliberately uses a compact hero whose leader line is hidden.
   // Verify the visible hero on Overview, then the actual visible Crown card.
   await page.locator('#gov-tab-overview').click();const line=page.locator('#govScreen .gov-ui-hero .gov-ui-leader-line');await line.waitFor({state:'visible'});
   assert.equal((await line.locator('strong').innerText()).trim(),data.leader.name);assert((await line.innerText()).includes('King'));assert(!(await line.innerText()).includes(b.prime_minister?.person.name||'__no_pm__'));
   if(b.prime_minister){const premier=page.locator('#gov-panel-overview [data-gov-institution-person="'+b.prime_minister.person.id+'"]').first();await premier.scrollIntoViewIfNeeded();assert.equal((await premier.locator('h3').innerText()).trim(),b.prime_minister.person.name);}
   await page.locator('#gov-tab-institutions').click();const crown=page.locator('#gov-panel-institutions [data-gov-institution-person="'+contract.KING+'"]').first();await crown.scrollIntoViewIfNeeded();assert(await crown.isVisible());assert.equal((await crown.locator('h3').innerText()).trim(),data.leader.name);
   evidence.stage='visible-institution-portraits:'+b.date;
   assert.equal(await page.locator('#gov-panel-institutions [data-gov-institution-person="'+contract.IDS[3]+'"] [data-gov-review]').count(),0);
   for(const person of [...b.future_preview.map(c=>c.person),...(opening?[data.party_leadership.executive_person,b.prime_minister.person]:[])]){
    const card=page.locator('#gov-panel-institutions [data-gov-institution-person="'+person.id+'"]').first();await card.scrollIntoViewIfNeeded();assert((await card.innerText()).includes(person.name));
    assert.match(person.portrait?.url||'',/^\/art\/people\/[a-z0-9-]+\.png$/);const image=card.locator('img').first();await image.waitFor();
    await image.evaluate(img=>img.decode());assert(await image.evaluate(img=>img.complete&&img.naturalWidth>0));assert.equal(await image.getAttribute('src'),person.portrait.url);
    const response=await page.request.get(url+person.portrait.url);assert(response.ok());const actual=await response.body(),expected=fs.readFileSync(path.join(root,'spheres-web/ui/person-portraits',path.basename(person.portrait.url)));assert(actual.equals(expected));
    evidence.portraits.push({person_id:person.id,date:b.date,url:person.portrait.url,sha256:hash(actual)});
   }
   assert.equal(b.historical_bindings.length,50);assert(b.historical_bindings.every(row=>row.portrait_date_is_tenure===false));
   if(opening){
    const historical=page.locator('#gov-panel-institutions > section.gov-ui-panel').filter({has:page.getByRole('heading',{name:'Sourced office references',exact:true})}).locator('article.gov-ui-institution-person');
    assert.equal(await historical.count(),b.historical_bindings.length);const missing=[];
    for(const [index,binding] of b.historical_bindings.entries()){
     const card=historical.nth(index);assert.equal(await card.getAttribute('data-gov-institution-person'),binding.person_id);
     const roleLabel=(await card.locator('.gov-ui-kicker').innerText()).trim();assert(roleLabel);assert.doesNotMatch(roleLabel,/^TO(?:[\s_]|$)/i,'Historical office must have a readable label: '+binding.role_id);
     if(binding.person_id==='to_fatai_helu'){assert(!binding.person.portrait?.url);assert.equal(await card.locator('img').count(),0);missing.push(binding.person_id);continue;}
     assert(binding.person.portrait?.url,'Historical portrait missing for '+binding.person_id);await card.scrollIntoViewIfNeeded();const image=card.locator('img').first();await image.evaluate(img=>img.decode());
     assert.equal(await image.getAttribute('src'),binding.person.portrait.url);assert(await image.evaluate(img=>img.complete&&img.naturalWidth>0));
    }
    assert.deepEqual([...new Set(missing)],['to_fatai_helu']);assert(b.historical_bindings.some(b=>b.person_id===contract.KING&&b.person.portrait?.url));
    evidence.checks.push('historical-King-IV-image','historical-art-only-Fatai-unavailable');
   }
  }
  async function action(type,extra={},cancel=false){
   evidence.stage=(cancel?'cancel:':'action:')+type+':'+JSON.stringify(extra);
   await page.locator('#gov-tab-institutions').click();const data=await government(),index=contract.actionIndex(data,type,extra),a=data.actions[index];assert.equal(a.refusal,null);
   const before=await archive(),crown=JSON.stringify(data.leader),previewResponse=page.waitForResponse(r=>new URL(r.url()).pathname==='/api/government/preview');
   await page.locator('#gov-panel-institutions [data-gov-review="'+index+'"]').click();const response=await previewResponse;assert(response.ok());const preview=await response.json();
   assert.equal(preview.valid,true);assert(preview.review_token);assert.deepEqual(preview.command,a.command);assert.equal(preview.price_pc,a.price);
   await page.locator('#govReview [data-gov-confirm]:not([disabled])').waitFor();assert.deepEqual(await archive(),before,'Review must preserve complete native world and history');
   if(cancel){await page.locator('#govReview [data-gov-review-close]').first().click();assert.deepEqual(await archive(),before);evidence.checks.push('cancelled-'+type+'-preserves-save');return;}
   const commandResponse=page.waitForResponse(r=>new URL(r.url()).pathname==='/api/command'&&r.request().method()==='POST');await page.locator('#govReview [data-gov-confirm]').click();const committed=await commandResponse;assert(committed.ok());const result=await committed.json();assert.deepEqual(result.errors||[],[]);
   const payload=committed.request().postDataJSON();assert.deepEqual(payload.commands,[a.command]);assert.equal(payload.review_kind,'government');assert.equal(payload.review_token,preview.review_token);
   await page.waitForFunction(()=>typeof gov!=='undefined'&&!gov.busy&&!gov.loading&&!!gov.data&&gov.dataState===S);
   const after=await government();assert.equal(JSON.stringify(after.leader),crown,'Civilian actions preserve the actual Crown');assert.equal(after.political_capital,data.political_capital-a.price);
   evidence.commands.push({command:a.command,price:a.price,pc_before:data.political_capital,pc_after:after.political_capital});await page.locator('#gov-tab-institutions').click();return after;
  }
  async function refused(type,extra={},reason){
   evidence.stage='refusal:'+type+':'+JSON.stringify(extra);
   const data=await government(),index=contract.actionIndex(data,type,extra),a=data.actions[index];assert.match(a.refusal,reason);const before=await archive();
   const button=page.locator('#gov-panel-institutions [data-gov-review="'+index+'"]').first();assert(await button.isDisabled());
   const r=await page.request.post(url+'/api/government/preview',{data:{nation:'Tonga',command:a.command,session_id:(await state()).session_id}});assert(r.ok());const v=await r.json();assert.equal(v.valid,false);assert(!v.review_token);assert.deepEqual(await archive(),before);
   evidence.refusals.push({date:data.party_leadership.institutional_leadership.date,command:a.command,reason:a.refusal});
  }
  // Actual fresh production start precedes every authored-date fixture.
  evidence.stage='fresh-Tonga-start';
  await page.goto(url,{waitUntil:'domcontentloaded'});await page.locator('#newCampaignBtn').click();await page.locator('#nationPick [aria-label^="Tonga;"]').click();await page.locator('#startBtn').click();await page.locator('#app').waitFor({state:'visible'});await openGovernment();
  const initial=await government();evidence.opening_government=initial;contract.unappointed(contract.board(initial,'1990-01-01'));await visibleCards(initial,{opening:true});
  const beforeReading=await archive();await page.setViewportSize({width:414,height:896});
  const mobile=await page.locator('#govScreen .gov-ui').evaluate(e=>({width:e.clientWidth,scroll:e.scrollWidth}));assert(mobile.width>0&&mobile.scroll<=mobile.width+1,'Tonga institutions must fit the mobile panel');
  assert(!/\bNaN\b|\bundefined\b/.test(await page.locator('#govScreen .gov-ui').innerText()));await page.screenshot({path:path.join(out,'opening-mobile.png')});await page.setViewportSize({width:1440,height:1000});assert.deepEqual(await archive(),beforeReading);
  evidence.checks.push('fresh-Tonga-opening-Crown-PM-portraits','four-fictional-preview-portraits','fifty-reference-bindings','mobile-read-only');
  await load('tonga-reference-cutoff');contract.unappointed(contract.board(await government(),'2026-09-07'));await action('reform');await refused('hold_election',{},/No reviewed election candidate pool/);await refused('recommend_prime_minister',{person_id:contract.IDS[1]},/only from 8 September 2026/);
  await load('tonga-first-fiction');contract.unappointed(contract.board(await government(),'2026-09-08'));await action('reform',{},true);await action('reform');await refused('recommend_prime_minister',{person_id:contract.IDS[1]},/first hold a people's seat|new election/);await action('hold_election');await refused('appoint_minister',{person_id:contract.IDS[2]},/confirmed Prime Minister/);await action('recommend_prime_minister',{person_id:contract.IDS[1]});await action('appoint_minister',{person_id:contract.IDS[2]});
  contract.appointed(contract.board(await government(),'2026-09-08'));await visibleCards(await government());await page.screenshot({path:path.join(out,'appointed-desktop.png')});
  const archived=await archive(),slot='tonga-completed';assert((await page.request.post(url+'/api/save',{data:{slot}})).ok());const oldSession=(await state()).session_id;
  await load(slot);assert.notEqual((await state()).session_id,oldSession);assert.deepEqual(await archive(),archived,'Ordinary Save/Load must preserve complete native world, history and institutions');
  await page.reload();await page.locator('#continueBtn').click();await page.locator('#app').waitFor({state:'visible'});await openGovernment();assert.deepEqual(await archive(),archived,'Reload/Continue is read-only');contract.appointed(contract.board(await government(),'2026-09-08'));evidence.checks.push('save-load-full-archive','reload-continue-full-archive');
  await action('vacate',{office:'prime_minister',person_id:contract.IDS[1]});const vacant=contract.board(await government(),'2026-09-08');assert.equal(vacant.prime_minister,null);assert.equal(vacant.recommendation_due,true);assert.equal(vacant.nonelected_ministers[0].person.id,contract.IDS[2]);await action('recommend_prime_minister',{person_id:contract.IDS[1]});
  await load('tonga-last-fiction');contract.unappointed(contract.board(await government(),'2035-12-31'));await action('reform');await action('hold_election');await action('recommend_prime_minister',{person_id:contract.IDS[1]});await action('appoint_minister',{person_id:contract.IDS[2]});contract.appointed(contract.board(await government(),'2035-12-31'));
  await load('tonga-after-endpoint');contract.appointed(contract.board(await government(),'2036-01-01'));await refused('hold_election',{},/No reviewed election candidate pool/);await refused('appoint_minister',{person_id:contract.IDS[2]},/only from 8 September 2026/);await action('vacate',{office:'prime_minister',person_id:contract.IDS[1]});await refused('recommend_prime_minister',{person_id:contract.IDS[1]},/only from 8 September 2026/);
  evidence.checks.push('historical-cutoff-refusals','explicit-reform-election-PM-minister','actual-vacancy-caretaker-recommendation','2035-inclusive','2036-new-appointments-refused-incumbents-retained');
  assert.equal(requests.filter(r=>r.path==='/api/advance').length,0,'No day or year is simulated');assert.deepEqual(errors,[]);evidence.stage='complete';evidence.passed=true;
 }catch(e){evidence.error=e.stack||String(e);if(page){await page.screenshot({path:path.join(out,'failure.png'),timeout:5000}).catch(()=>{});fs.writeFileSync(path.join(out,'failure.html'),await page.content().catch(()=>''));}throw e;
 }finally{evidence.page_errors=errors;evidence.requests=requests;fs.writeFileSync(path.join(out,'result.json'),JSON.stringify(evidence,null,2));if(browser)await browser.close();server.kill();log.end();}
}
if(require.main===module)main().catch(e=>{console.error(e);process.exitCode=1;});
