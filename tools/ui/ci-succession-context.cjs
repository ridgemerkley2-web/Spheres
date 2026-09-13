// S10.f: two disclosed native succession fixtures through ordinary campaign UI.
// This driver issues no appointment, substitutes no response and advances no day.
const assert=require('node:assert/strict'),fs=require('node:fs'),path=require('node:path');
const cp=require('node:child_process'),net=require('node:net');
const {chromium}=require('playwright'),integrated=require('./ci-integrated.cjs');
const audit=require('./supplier-archive-audit.cjs');
const root=path.resolve(__dirname,'../..'),quoted=value=>JSON.stringify(String(value));
const names=['uk-1990-same-day-death','uk-2027-term-limit'];
const route=(response,p)=>new URL(response.url()).pathname===p&&response.request().method()==='POST';
const party=board=>{const row=board.parties.find(p=>p.party_id==='uk_con');assert(row);return row;};
async function freePort(){const s=net.createServer();await new Promise(resolve=>s.listen(0,'127.0.0.1',resolve));const port=s.address().port;await new Promise(resolve=>s.close(resolve));return port;}
async function get(page,url,p){const r=await page.request.get(url+p);try{assert(r.ok(),p);return await r.json();}finally{await r.dispose();}}
async function idle(page){await page.waitForFunction(()=>!SESSION.busy&&!COMMAND_CHANNEL.busy&&!COMMAND_CHANNEL.pending&&!advancing&&!pendingAdvance&&!AGENCY.busy&&!gov.busy);}
function regular(file){assert(fs.lstatSync(file).isFile()&&!fs.lstatSync(file).isSymbolicLink());assert.equal(fs.realpathSync(file),file);return file;}
async function capture(page,url,run,name,stage){
  assert(names.includes(name));assert(['loaded','browsed','load-cancelled','reloaded','continued'].includes(stage));
  const slot='s10f-audit-'+name+'-'+stage,saves=path.join(fs.realpathSync(run),'saves');
  assert.equal(fs.realpathSync(saves),saves);const source=path.join(saves,slot+'.json');assert(!fs.existsSync(source));
  const r=await page.request.post(url+'/api/save',{data:{slot}});try{assert(r.ok());await r.body();}finally{await r.dispose();}
  regular(source);const directory=path.join(run,'captures');if(!fs.existsSync(directory))fs.mkdirSync(directory);
  assert.equal(fs.realpathSync(directory),directory);const target=path.join(directory,slot+'.json');assert(!fs.existsSync(target));
  fs.renameSync(source,target);return audit.inspect(target,'UK');
}
async function readable(locator,label){
  await locator.evaluate(e=>e.scrollIntoView({block:'center',inline:'nearest',behavior:'instant'}));
  const result=await locator.evaluate(e=>{
    const rect=e.getBoundingClientRect(),range=document.createRange();range.selectNodeContents(e);
    const ranges=Array.from(range.getClientRects()).filter(r=>r.width&&r.height),errors=[];
    const inside=r=>r.left>=-1&&r.right<=document.documentElement.clientWidth+1&&r.top>=-1&&r.bottom<=innerHeight+1;
    if(!inside(rect))errors.push('outside viewport');
    for(const r of ranges){if(!inside(r)||r.left<rect.left-1||r.right>rect.right+1||r.top<rect.top-1||r.bottom>rect.bottom+1)errors.push('clipped text');
      for(const fraction of [.2,.5,.8]){const hit=document.elementFromPoint(r.left+r.width*fraction,r.top+r.height/2);if(!hit||!(hit===e||e.contains(hit)))errors.push('obscured text');}}
    return {text:e.textContent,rect:{left:rect.left,right:rect.right,top:rect.top,bottom:rect.bottom},errors};
  });assert.deepEqual(result.errors,[],label+': '+JSON.stringify(result));return result;
}
async function within(locator,label){
  const value=await locator.evaluate(e=>({width:e.clientWidth,scroll:e.scrollWidth,left:e.scrollLeft}));
  assert(value.width>0&&value.scroll<=value.width+1&&Math.abs(value.left)<1,label+' overflows: '+JSON.stringify(value));
  assert(!/\bNaN\b|\bundefined\b/.test(await locator.innerText()),label+' has missing values');return value;
}
async function readLeaves(locator,label){const out=[];for(const leaf of await locator.locator('h4,p,dt,dd,summary').all())if(await leaf.isVisible())out.push(await readable(leaf,label));return out;}
async function government(page,fixture){
  if(!await page.locator('#govScreen').isVisible())await page.locator('#govBtn').click();
  await page.waitForFunction(()=>gov.open&&gov.data&&gov.dataState===S&&!gov.data.error);
  assert.deepEqual(await page.evaluate(()=>gov.data),fixture.government,'Government UI must consume exact native fixture reading');
  await page.locator('#gov-tab-leadership').click();
  await page.locator('[data-gov-leadership-mode="campaign"]').click();
  await page.locator('[data-gov-leadership-search]').fill(fixture.party_id);
  assert.equal(await page.locator('[data-gov-leadership-party]').count(),1,'Search selects the exact party');
}
async function details(locator,opened){const current=await locator.getAttribute('open');if((current!==null)!==opened)await locator.locator(':scope > summary').click();}
async function statusCard(card,native,width){
  const status=native.campaign_succession;assert(status,'Native campaign entry needs succession metadata');
  const box=card.locator('[data-gov-succession]');assert.equal(await box.count(),1);
  for(const [hook,value] of [['party',status.party.label],['executive',status.national_office.label],['death',status.death_date],['exclusion',status.office_excluded_date]]){
    const cell=box.locator('[data-gov-succession-'+hook+']');if(value==null)assert.equal(await cell.count(),0);else assert.equal(await cell.textContent(),value,'Exact native succession '+hook);
  }
  const text=await box.innerText();for(const value of [status.date,status.party.note,status.national_office.note,status.note].filter(Boolean))assert(text.includes(value),'Missing native explanation: '+value);
  const leaves=await readLeaves(box,'Succession status '+native.person.id+' at '+width);await within(box,'Succession card at '+width);await within(card,'Person card at '+width);
  return {person:native.person.id,status,text,leaves};
}
async function campaignLayout(page,fixture,width){
  await page.setViewportSize({width,height:width===1440?1000:844});
  const native=party(fixture.campaign),card=page.locator('[data-gov-leadership-party='+quoted(fixture.party_id)+']');
  const holders=card.locator('.gov-ui-party-leaders > [data-gov-person]');
  assert.deepEqual(await holders.evaluateAll(rows=>rows.map(e=>e.dataset.govPerson)),native.campaign.map(e=>e.person.id),'Exact saved campaign party holders');
  const evidence={width,holders:[],future:[]};
  for(const entry of native.campaign){
    const person=card.locator('.gov-ui-party-leaders > [data-gov-person='+quoted(entry.person.id)+']');
    assert.equal(await person.locator('h4').textContent(),entry.person.name);
    if(entry.campaign_succession)evidence.holders.push(await statusCard(person,entry,width));
  }
  if(fixture.name==='uk-2027-term-limit'){
    const entry=native.campaign.find(e=>e.person.id===fixture.focus_person);assert(entry);
    assert.equal(entry.campaign_succession.party.status,'recorded_holder');assert.equal(entry.campaign_succession.national_office.status,'excluded');
    assert.equal(entry.executive_eligibility.authorized,true);assert.equal(entry.person.fiction.origin,'fictional_successor');
    const pane=card.locator('[data-gov-detail='+quoted('leader-future:'+fixture.party_id)+']');await details(pane,true);
    const future=native.future_candidates.find(e=>e.person.id===fixture.focus_person);assert(future,'Excluded fictional holder stays in native party cast');
    const person=pane.locator('[data-gov-person='+quoted(fixture.focus_person)+']');assert.equal(await person.locator('h4').textContent(),future.person.name);
    evidence.future.push(await statusCard(person,future,width));await details(pane,false);
  }
  await within(card,'Party card at '+width);await within(page.locator('#govScreen'),'Government at '+width);return evidence;
}
async function reference(page,fixture){
  const request=page.waitForResponse(r=>new URL(r.url()).pathname==='/api/party-leadership'&&r.request().method()==='GET');
  await page.locator('[data-gov-leadership-mode="reference"]').click();const response=await request;assert(response.ok());
  assert.deepEqual(await response.json(),fixture.reference,'Real reference response equals native source-only reading');
  await page.waitForFunction(()=>gov.leadershipReference?.data&&!gov.leadershipReference.loading);
  assert.equal(await page.locator('[data-gov-leadership-date]').inputValue(),fixture.reference_date);
  assert.deepEqual(await page.evaluate(()=>gov.leadershipReference.data),fixture.reference);
  assert.equal(await page.locator('[data-gov-succession]').count(),0,'Historical browsing cannot display campaign restrictions');
}
async function referenceLayout(page,fixture,width){
  await page.setViewportSize({width,height:width===1440?1000:844});
  const row=party(fixture.reference),card=page.locator('[data-gov-leadership-party='+quoted(fixture.party_id)+']');
  assert.deepEqual(await card.locator('.gov-ui-party-leaders > [data-gov-person]').evaluateAll(rows=>rows.map(e=>e.dataset.govPerson)),row.historical.map(e=>e.person.id),'Exact source-only officeholder roster');
  const out={width,people:[]};
  for(const entry of row.historical){
    const person=card.locator('.gov-ui-party-leaders > [data-gov-person='+quoted(entry.person.id)+']');
    assert.equal(await person.locator('h4').textContent(),entry.person.name);assert.equal(await person.locator('[data-gov-succession]').count(),0);
    const proof=person.locator('.gov-ui-person-evidence');await details(proof,true);
    const links=await proof.locator('a').evaluateAll(rows=>rows.map(e=>e.getAttribute('href')));
    for(const source of [...entry.person.sources,...entry.term.sources])assert(links.includes(source),'Sourced historical identity must retain its actual citations');
    out.people.push({id:entry.person.id,name:entry.person.name,links,leaves:await readLeaves(person,'Historical identity at '+width)});await details(proof,false);
  }
  if(fixture.name==='uk-1990-same-day-death'){
    assert(row.historical.some(e=>e.person.id===fixture.focus_person&&e.person.name==='Margaret Thatcher'));
    assert(row.eligible.some(e=>e.person.id===fixture.focus_person),'Campaign death must not suppress same-date historical candidate');
    const candidates=card.locator('[data-gov-detail='+quoted('leader-candidates:'+fixture.party_id)+']');await details(candidates,true);
    assert.equal(await candidates.locator('[data-gov-person='+quoted(fixture.focus_person)+'] h4').textContent(),'Margaret Thatcher');await details(candidates,false);
  }
  await within(card,'Historical party at '+width);await within(page.locator('#govScreen'),'Historical government at '+width);return out;
}
async function campaigns(page){
  if(await page.locator('#govScreen').isVisible())await page.locator('#govScreen .tbar .x').click();
  if(await page.locator('#app').isVisible()){
    if(await page.locator('.arc-time-menu').getAttribute('open')===null)await page.locator('.arc-time-menu > summary').click();
    await page.locator('#campaignsBtn').click();
  }
  if(!await page.locator('#savedCampaigns').isVisible())await page.locator('#openSavesBtn').click();await page.locator('#savedCampaigns').waitFor();
}
function stateFacts(state,fixture){assert.equal(state.player,fixture.player);assert.equal(state.date,fixture.date);assert.deepEqual(state.log,fixture.log);assert.equal(state.dispatch_count,fixture.dispatch_count);}
async function main(){
  assert(process.env.SPHERES_BINARY&&process.env.SPHERES_S10F_SUCCESSION_FIXTURE,'Set built SPHERES_BINARY and SPHERES_S10F_SUCCESSION_FIXTURE manifest path');
  const expected=process.env.SPHERES_EXPECTED_REVISION||'';assert.match(expected,/^[a-f0-9]{40}$/);
  const git=args=>cp.execFileSync('git',['-c','core.longpaths=true',...args],{cwd:root,windowsHide:true,maxBuffer:32*1024*1024});
  const revision=git(['rev-parse','HEAD']).toString().trim();assert.equal(git(['status','--porcelain']).toString().trim(),'','Require a clean committed checkout');
  assert.equal(git(['diff','--name-only',expected,revision,'--','spheres-sim','spheres-cli','spheres-web','Cargo.toml','Cargo.lock']).toString().trim(),'','Driver must preserve pinned runtime source');
  const binary=regular(path.resolve(process.env.SPHERES_BINARY)),binaryHash=audit.fileHash(binary);
  const manifestPath=regular(path.resolve(process.env.SPHERES_S10F_SUCCESSION_FIXTURE)),directory=path.dirname(manifestPath);
  const manifest=JSON.parse(fs.readFileSync(manifestPath,'utf8'));assert.equal(manifest.fixture,'s10f-authored-native-succession-context');assert.equal(manifest.version,1);
  assert.equal(manifest.compiled_revision,git(['rev-parse','--short=12',expected]).toString().trim());assert.equal(manifest.days_advanced,0);
  assert.deepEqual(manifest.cases.map(c=>c.name),names);
  const files=new Map(manifest.cases.map(c=>{assert.equal(c.file,c.name+'.json');assert.equal(c.player,'UK');assert.equal(c.days_advanced,0);return [c.name,regular(path.join(directory,c.file))];}));
  const fixtureHashes={manifest:audit.fileHash(manifestPath),...Object.fromEntries([...files].map(([name,file])=>[name,audit.fileHash(file)]))};
  const output=path.resolve(process.env.SPHERES_SUCCESSION_OUTPUT||path.join(root,'artifacts/browser-succession-context-ci'));
  fs.mkdirSync(output,{recursive:true});const out=fs.mkdtempSync(path.join(output,'authored-uk-')),run=path.join(out,'server');fs.mkdirSync(run);fs.mkdirSync(path.join(run,'saves'));
  for(const [name,file] of files)fs.copyFileSync(file,path.join(run,'saves','s10f-authored-'+name+'.json'),fs.constants.COPYFILE_EXCL);
  const port=await freePort(),url='http://127.0.0.1:'+port;
  const server=cp.spawn(binary,['--port',String(port),'--no-open'],{cwd:run,windowsHide:true,stdio:['ignore','pipe','pipe']});
  const log=fs.createWriteStream(path.join(out,'server.log'));server.stdout.pipe(log);server.stderr.pipe(log);
  let browser,page,launchError,stage='launch';server.on('error',error=>{launchError=error;});
  const e={passed:false,run,url,started_utc:new Date().toISOString(),scope:manifest.scope,fixture:{manifest_path:manifestPath,source:manifest,sha256:fixtureHashes},test_source:{revision,driver_sha256:audit.fileHash(__filename),runtime_source_equal:true},screenshots:[],commands:[],previews:[],advances:[],errors:[],cases:[],checks:[]};
  const write=()=>fs.writeFileSync(path.join(out,'result.json'),JSON.stringify(e,null,2)+'\n');
  const telemetry=(event,details)=>fs.appendFileSync(path.join(out,'progress.jsonl'),JSON.stringify({utc:new Date().toISOString(),stage,event,...details})+'\n');
  const shot=async name=>{await page.screenshot({path:path.join(out,name+'.png')});e.screenshots.push(name+'.png');};audit.configure(out,telemetry);
  try{
    const until=Date.now()+20000;for(;;){if(launchError)throw launchError;assert.equal(server.exitCode,null);try{const r=await fetch(url+'/api/build',{headers:{Connection:'close'},signal:AbortSignal.timeout(2000)});await r.arrayBuffer();if(r.ok)break;}catch(error){if(Date.now()>=until)throw error;}assert(Date.now()<until,'Startup exceeded 20 seconds');await new Promise(resolve=>setTimeout(resolve,100));}
    browser=await chromium.launch({headless:true,...(process.env.SPHERES_BROWSER_CHANNEL?{channel:process.env.SPHERES_BROWSER_CHANNEL}:{})});e.browser_version=browser.version();
    const context=await browser.newContext({viewport:{width:1440,height:1000},reducedMotion:'reduce'});page=await context.newPage();page.setDefaultTimeout(30000);
    page.on('pageerror',error=>e.errors.push(error.message));page.on('request',r=>{if(r.method()==='POST'){const p=new URL(r.url()).pathname;if(p==='/api/command')e.commands.push(r.postDataJSON());if(['/api/government/preview','/api/decisions/preview'].includes(p))e.previews.push(r.postDataJSON());if(p==='/api/advance')e.advances.push(r.postDataJSON());}});
    e.build=await integrated.verifyBuild({page,url,root,run,binary});await page.goto(url,{waitUntil:'domcontentloaded'});await page.locator('#campaignHome').waitFor();await page.waitForFunction(()=>!!SESSION.live?.session_id);
    for(const fixture of manifest.cases){
      const result={name:fixture.name,scope:fixture.scope,campaign_layout:[],reference_layout:[]};e.cases.push(result);
      const authored=audit.inspect(files.get(fixture.name),'UK');
      stage=fixture.name+': ordinary Load';await page.setViewportSize({width:1440,height:1000});const hasPlayer=await page.evaluate(()=>!!SESSION.live?.player);await campaigns(page);
      await page.locator('#saveSlots').selectOption('s10f-authored-'+fixture.name);const loading=page.waitForResponse(r=>route(r,'/api/load'));await page.locator('#loadBtn').click();if(hasPlayer)await page.locator('#campaignConfirmAccept').click();assert((await loading).ok());
      await page.locator('#app').waitFor();await idle(page);assert.equal(await page.evaluate(()=>clock.running),false);stateFacts(await get(page,url,'/api/state'),fixture);
      result.capabilities=await integrated.capabilities({page,url,player:'UK'});const loaded=await capture(page,url,run,fixture.name,'loaded');audit.compare(loaded,authored,'Ordinary Load preserves complete native '+fixture.name+' world');
      stage=fixture.name+': campaign and historical leadership browsing';await government(page,fixture);
      for(const width of [1440,390,320]){result.campaign_layout.push(await campaignLayout(page,fixture,width));const target=page.locator('[data-gov-succession]').first();if(await target.count())await target.scrollIntoViewIfNeeded();else await page.locator('[data-gov-leadership-party] h3').scrollIntoViewIfNeeded();await shot(fixture.name+'-campaign-'+width);}
      await reference(page,fixture);
      for(const width of [1440,390,320]){result.reference_layout.push(await referenceLayout(page,fixture,width));const person=page.locator('.gov-ui-party-leaders > [data-gov-person] h4').first();if(await person.count())await person.scrollIntoViewIfNeeded();await shot(fixture.name+'-reference-'+width);}
      await page.locator('[data-gov-leadership-mode="campaign"]').click();assert.equal(await page.locator('[data-gov-leadership-date]').count(),0);
      assert.deepEqual(await get(page,url,'/api/party-leadership?nation=UK'),fixture.campaign);stateFacts(await get(page,url,'/api/state'),fixture);
      audit.compare(await capture(page,url,run,fixture.name,'browsed'),loaded,'Campaign status, historical source browsing and return preserve complete native '+fixture.name+' world');
      assert.deepEqual(e.commands,[]);assert.deepEqual(e.previews,[]);assert.deepEqual(e.advances,[]);
      stage=fixture.name+': named Save, cancelled Load, Load and Continue';await page.setViewportSize({width:1440,height:1000});await campaigns(page);const saved='s10f-reviewed-'+fixture.name;await page.locator('#saveName').fill(saved);
      const saving=page.waitForResponse(r=>route(r,'/api/save'));await page.locator('#saveNamedBtn').click();assert((await saving).ok());await page.locator('#saveSlots option[value='+quoted(saved)+']').waitFor({state:'attached'});await page.locator('#saveSlots').selectOption(saved);
      await page.locator('#loadBtn').click();await page.locator('#campaignConfirmCancel').click();stateFacts(await get(page,url,'/api/state'),fixture);audit.compare(await capture(page,url,run,fixture.name,'load-cancelled'),loaded,'Cancelled Load preserves complete native '+fixture.name+' world');
      const previous=(await get(page,url,'/api/state')).session_id,reloading=page.waitForResponse(r=>route(r,'/api/load'));await page.locator('#loadBtn').click();await page.locator('#campaignConfirmAccept').click();assert((await reloading).ok());await page.locator('#app').waitFor();await idle(page);
      const reloaded=await get(page,url,'/api/state');assert.notEqual(reloaded.session_id,previous);stateFacts(reloaded,fixture);audit.compare(await capture(page,url,run,fixture.name,'reloaded'),loaded,'Named Save/Load preserves complete native '+fixture.name+' world');
      await page.reload({waitUntil:'domcontentloaded'});await page.locator('#continueBtn').click();await page.locator('#app').waitFor();await idle(page);stateFacts(await get(page,url,'/api/state'),fixture);
      audit.compare(await capture(page,url,run,fixture.name,'continued'),loaded,'Continue preserves complete native '+fixture.name+' world');await government(page,fixture);result.continued_layout=await campaignLayout(page,fixture,320);await shot(fixture.name+'-continued-320');
      result.saved_slot=saved;result.final_capture='captures/s10f-audit-'+fixture.name+'-continued.json';result.days_advanced=0;result.passed=true;
    }
    assert.deepEqual(e.commands,[]);assert.deepEqual(e.previews,[]);assert.deepEqual(e.advances,[]);assert.deepEqual(e.errors,[]);assert.equal(audit.fileHash(binary),binaryHash);
    assert.deepEqual({manifest:audit.fileHash(manifestPath),...Object.fromEntries([...files].map(([name,file])=>[name,audit.fileHash(file)]))},fixtureHashes);
    assert.equal(git(['rev-parse','HEAD']).toString().trim(),revision);assert.equal(git(['status','--porcelain']).toString().trim(),'');
    e.checks.push('Same-date historical Thatcher identity, candidate and source citations survive an authored campaign Death','Native fictional party holder and distinct national-office exclusion display at 1440, 390 and 320 px','Both disclosed fixtures retain the complete native world through read-only browsing, cancelled Load, named Save/Load and Continue','No command, decision preview or simulation advance was issued by this browser journey');
    e.days_advanced=0;e.passed=true;e.finished_utc=new Date().toISOString();write();console.log(JSON.stringify({passed:true,result:path.join(out,'result.json')}));
  }catch(error){e.failed_stage=stage;e.failure=String(error.stack||error);write();if(page)try{await page.screenshot({path:path.join(out,'failure.png')});fs.writeFileSync(path.join(out,'failure.html'),await page.content());}catch{}throw error;}
  finally{if(browser)await browser.close();server.kill();log.end();}
}
main().catch(error=>{console.error(error);process.exitCode=1;});
