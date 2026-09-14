// S10.g: a disclosed native sanctions fixture through actual player controls.
// No response interception, world injection, appointment or simulation advance.
const assert=require('node:assert/strict'),fs=require('node:fs'),path=require('node:path');
const cp=require('node:child_process'),net=require('node:net'),crypto=require('node:crypto');
const {chromium}=require('playwright'),integrated=require('./ci-integrated.cjs');
const audit=require('./supplier-archive-audit.cjs'),reviewUI=require('./government-review-assertions.cjs');
const root=path.resolve(__dirname,'../..'),quoted=x=>JSON.stringify(String(x));
const hash=x=>crypto.createHash('sha256').update(x).digest('hex');
const route=(r,p)=>new URL(r.url()).pathname===p&&r.request().method()==='POST';
const stableQuote=q=>{const {session_id,review_token,...stable}=q;return stable;};
const SLOT='s10g-authored-sanctions',SAVED='s10g-reviewed-sanctions';
const stages=['loaded','cancelled','lift','sanction','improve','foreign','load-cancelled','reloaded','continued'];
async function freePort(){const s=net.createServer();await new Promise(r=>s.listen(0,'127.0.0.1',r));const p=s.address().port;await new Promise(r=>s.close(r));return p;}
async function get(page,url,p){const r=await page.request.get(url+p);try{assert(r.ok(),p);return await r.json();}finally{await r.dispose();}}
async function idle(page){await page.waitForFunction(()=>!SESSION.busy&&!COMMAND_CHANNEL.busy&&!COMMAND_CHANNEL.pending&&!advancing&&!pendingAdvance&&!AGENCY.busy&&!gov.busy);}
function regular(file){assert(fs.lstatSync(file).isFile()&&!fs.lstatSync(file).isSymbolicLink());assert.equal(fs.realpathSync(file),file);return file;}
function envelope(file){
  // The shared audit compares every world field. Compare every remaining
  // campaign-envelope field too, except the separately retained save timestamp.
  // An ordinary new Save necessarily has a new saved_unix; no history is omitted.
  const code=`import hashlib,json,sys\np=sys.argv[1]\nwith open(p,encoding='utf-8-sig') as f: d=json.load(f)\nassert set(d)=={'format','version','world','history','log','history_epoch','saved_date','player','saved_unix'}\nassert d['format']=='spheres-campaign' and d['version']==1\nassert isinstance(d['saved_unix'],int) and d['saved_unix']>=0\nsaved=d.pop('saved_unix'); d.pop('world')\nb=json.dumps(d,sort_keys=True,separators=(',',':'),ensure_ascii=False,allow_nan=False).encode('utf-8')\nprint(json.dumps({'sha256':hashlib.sha256(b).hexdigest(),'bytes':len(b),'saved_unix':saved,'keys':sorted(d),'history_points':len(d['history']),'dispatches':len(d['log']),'saved_date':d['saved_date'],'player':d['player']}))`;
  const r=cp.spawnSync(process.env.SPHERES_AUDIT_PYTHON||'python',['-c',code,file],{windowsHide:true,encoding:'utf8',maxBuffer:65536,timeout:120000});
  if(r.error)throw r.error;assert.equal(r.status,0,'Campaign envelope inspection: '+r.stderr);return JSON.parse(r.stdout);
}
function inspect(file){const result=audit.inspect(file,'France');assert.deepEqual(result.canonical.ignored_paths,[]);result.envelope=envelope(file);return result;}
function compare(left,right,message,e){audit.compare(left,right,message);assert.equal(left.envelope.sha256,right.envelope.sha256,message+' (complete saved history and envelope)');assert.equal(left.envelope.bytes,right.envelope.bytes);e.envelope_comparisons.push({message,left:left.input.path,right:right.input.path,sha256:left.envelope.sha256,bytes:left.envelope.bytes,left_saved_unix:left.envelope.saved_unix,right_saved_unix:right.envelope.saved_unix,timestamp_rule:'saved_unix retained and validated separately; every world and historical field compared'});}
async function capture(page,url,run,stage){
  assert(stages.includes(stage));const slot='s10g-audit-'+stage,saves=path.join(fs.realpathSync(run),'saves');assert.equal(fs.realpathSync(saves),saves);
  const source=path.join(saves,slot+'.json');assert(!fs.existsSync(source));const r=await page.request.post(url+'/api/save',{data:{slot}});try{assert(r.ok());await r.body();}finally{await r.dispose();}
  regular(source);const dir=path.join(run,'captures');if(!fs.existsSync(dir))fs.mkdirSync(dir);assert.equal(fs.realpathSync(dir),dir);
  const target=path.join(dir,slot+'.json');assert(!fs.existsSync(target));fs.renameSync(source,target);return inspect(target);
}
async function readable(locator,label){
  await locator.evaluate(e=>e.scrollIntoView({block:'center',inline:'nearest',behavior:'instant'}));
  const value=await locator.evaluate(e=>{
    const rect=e.getBoundingClientRect(),ranges=[],errors=[],clips=new Map(),box=r=>({left:r.left,top:r.top,right:r.right,bottom:r.bottom});
    const inside=r=>r.left>=-1&&r.right<=document.documentElement.clientWidth+1&&r.top>=-1&&r.bottom<=innerHeight+1;
    if(!inside(rect))errors.push('outside viewport');const walker=document.createTreeWalker(e,NodeFilter.SHOW_TEXT);
    for(let text=walker.nextNode();text;text=walker.nextNode()){
      if(!text.textContent.trim()||text.parentElement.closest('.gov-ui-sr-only,.sr-only'))continue;
      const range=document.createRange();range.selectNodeContents(text);
      for(const r of Array.from(range.getClientRects()).filter(r=>r.width&&r.height)){
        ranges.push(box(r));if(!inside(r))errors.push('text outside viewport');
        for(let node=text.parentElement;node;node=node.parentElement){
          if(!clips.has(node)){const s=getComputedStyle(node),b=node.getBoundingClientRect(),paint=/(^|\s)(paint|strict|content)(\s|$)/.test(s.contain);clips.set(node,{element:node.id||node.className||node.tagName,x:paint||/^(auto|scroll|hidden|clip)$/.test(s.overflowX),y:paint||/^(auto|scroll|hidden|clip)$/.test(s.overflowY),left:b.left+node.clientLeft,right:b.left+node.clientLeft+node.clientWidth,top:b.top+node.clientTop,bottom:b.top+node.clientTop+node.clientHeight});}
          const c=clips.get(node);if(c.x&&(r.left<c.left-1||r.right>c.right+1)||c.y&&(r.top<c.top-1||r.bottom>c.bottom+1))errors.push('text clipped by '+c.element);
        }
        for(const f of [.2,.5,.8]){const hit=document.elementFromPoint(r.left+r.width*f,r.top+r.height/2);if(!hit||!(hit===e||e.contains(hit)))errors.push('obscured text');}
      }
    }
    if(!ranges.length)errors.push('no rendered text');return {text:e.textContent,rect:box(rect),text_rects:ranges,errors};
  });assert.deepEqual(value.errors,[],label+': '+JSON.stringify(value));return value;
}
async function within(locator,label){const r=await locator.evaluate(e=>({width:e.clientWidth,scroll:e.scrollWidth,left:e.scrollLeft}));assert(r.width>0&&r.scroll<=r.width+1&&Math.abs(r.left)<1,label+' overflow '+JSON.stringify(r));assert(!/\bNaN\b|\bundefined\b/.test(await locator.innerText()),label+' missing values');return r;}
async function leaves(locator,label){const out=[];for(const leaf of await locator.locator('h3,h4,p,dt,dd,button,label > span').all())if(await leaf.isVisible())out.push(await readable(leaf,label));return out;}
async function desk(page,agency,width){
  await page.setViewportSize({width,height:width===1440?1000:844});const native=agency.sanctions,section=page.locator('#agencySanctions');assert(native);assert(await section.isVisible());
  const out={width,directions:{}};
  for(const direction of ['outgoing','incoming']){
    const group=section.locator('[data-agency-sanction-direction='+quoted(direction)+']'),rows=native[direction],key=direction==='outgoing'?'target':'imposer';
    assert.deepEqual(await group.locator('[data-agency-sanction-partner]').evaluateAll(a=>a.map(e=>e.dataset.agencySanctionPartner)),rows.map(r=>r[key]),'Exact native '+direction+' sanction roster');
    out.directions[direction]=[];
    for(const row of rows){const card=group.locator('[data-agency-sanction-partner='+quoted(row[key])+']');assert((await card.innerText()).includes(row[key+'_name']));
      assert.equal(await card.locator('[data-agency-sanction-imposer]').textContent(),row.imposer_name);assert.equal(await card.locator('[data-agency-sanction-target]:not(button)').textContent(),row.target_name);
      const lift=card.locator('[data-agency-sanction-action="lift"]');assert.equal(await lift.count(),row.can_review_lift?1:0,'Only your outgoing sanction can be lifted');
      if(row.can_review_lift)assert.equal(await lift.getAttribute('data-agency-sanction-target'),row.target);
      out.directions[direction].push({partner:row[key],readings:await leaves(card,direction+' '+row[key]+' '+width)});
    }
  }
  const optionRows=await section.locator('#agencyDiplomacyTarget option').evaluateAll(rows=>rows.filter(e=>e.value).map(e=>({id:e.value,name:e.textContent})));
  assert.deepEqual(optionRows,native.targets.map(r=>({id:r.id,name:r.name})),'Target selector must match exact native active-country roster');
  assert.equal(await section.locator('[data-agency-sanction-drag]').textContent(),native.growth_drag.label);assert((await section.innerText()).includes(native.growth_drag.note));
  await readable(page.locator('#agencyClose'),'Close decisions at '+width);await within(section,'Sanctions '+width);await within(page.locator('#agencyPanel'),'Decisions '+width);return out;
}
async function context(page,quote,width){
  await page.setViewportSize({width,height:width===1440?1000:844});const native=quote.bilateral_context,section=page.locator('#agencyBilateralContext');assert(native);assert(await section.isVisible());
  const body=await section.innerText();assert(body.includes(native.actor_name));assert(body.includes(native.target_name));assert(body.includes(native.note));const out={width,states:{}};
  for(const side of ['before','after']){const row=native[side],card=section.locator('[data-agency-bilateral-state='+quoted(side)+']');assert(row);
    const text=await card.innerText();for(const value of [row.relation_label,row.restriction_label,row.target_growth_drag.label,row.trade_treaty.status_label,row.freight.outbound.label,row.freight.inbound.label,...row.other_sanctioners.map(r=>r.name)])assert(text.includes(value),'Native bilateral '+side+' missing '+value);
    const flag=value=>value===true?'Active':value===false?'None':'Not available';
    for(const [hook,value] of [['relations',row.relation_label],['outgoing',flag(row.your_sanctions)],['incoming',flag(row.their_sanctions)],['restrictions',row.restriction_label],['treaty',row.trade_treaty.status_label],['outbound',row.freight.outbound.label],['inbound',row.freight.inbound.label],['drag',row.target_growth_drag.label]])assert.equal(await card.locator('[data-agency-bilateral-'+hook+']').textContent(),value,'Exact native bilateral '+side+' '+hook);
    assert(text.includes(row.freight.note));assert(text.includes(row.target_growth_drag.note));
    out.states[side]={text,readings:await leaves(card,side+' bilateral '+width)};
  }
  await within(section,'Bilateral context '+width);return out;
}
async function review(page,step){
  await page.locator('#agencyDiplomacyTarget').selectOption('Japan');
  const response=page.waitForResponse(r=>route(r,'/api/decisions/preview'));
  const control=page.locator('#agencySanctions [data-agency-sanction-action='+quoted(step.kind)+'][data-agency-sanction-target="Japan"]').last();
  assert(await control.isEnabled());await control.click();const r=await response;assert(r.ok());const quote=await r.json();await page.waitForFunction(()=>AGENCY.review&&!AGENCY.review.loading);
  assert.equal(quote.valid,true);assert.deepEqual(quote.command,step.command);assert.equal(typeof quote.review_token,'string');assert(quote.review_token);
  assert.deepEqual(stableQuote(quote),stableQuote(step.quote),'Browser preview equals independent native fixture quote');
  assert.equal(await page.locator('#agencyReviewTitle').textContent(),quote.title);assert.equal(await page.locator('#agencyReview [data-agency-confirm]').isEnabled(),true);await reviewUI.exactChanges(page,'agency',quote);return quote;
}
function facts(state,expected){assert.equal(state.player,'France');assert.equal(state.date,expected.date);assert.deepEqual(state.agency,expected.agency,'Exact native agency, direction, policy and treaty readings');assert.deepEqual(state.log,expected.log,'Exact dated native dispatches');assert.equal(state.dispatch_count,expected.dispatch_count);const n=state.nations.find(n=>n.id==='France');assert(n);}
async function campaigns(page){
  if(await page.locator('#agencyPanel').isVisible())await page.locator('#agencyClose').click();if(await page.locator('#govScreen').isVisible())await page.locator('#govScreen .tbar .x').click();
  if(await page.locator('#app').isVisible()){if(await page.locator('.arc-time-menu').getAttribute('open')===null)await page.locator('.arc-time-menu > summary').click();await page.locator('#campaignsBtn').click();}
  if(!await page.locator('#savedCampaigns').isVisible())await page.locator('#openSavesBtn').click();await page.locator('#savedCampaigns').waitFor();
}
async function main(){
  assert(process.env.SPHERES_BINARY&&process.env.SPHERES_S10G_SANCTIONS_FIXTURE);const expected=process.env.SPHERES_EXPECTED_REVISION||'';assert.match(expected,/^[a-f0-9]{40}$/);
  const git=args=>cp.execFileSync('git',['-c','core.longpaths=true',...args],{cwd:root,windowsHide:true,maxBuffer:32*1024*1024});
  const revision=git(['rev-parse','HEAD']).toString().trim();assert.equal(git(['status','--porcelain']).toString().trim(),'','Require clean committed source');assert.equal(git(['diff','--name-only',expected,revision,'--','spheres-sim','spheres-cli','spheres-web','Cargo.toml','Cargo.lock']).toString().trim(),'','Pinned runtime source must remain unchanged');
  const binary=regular(path.resolve(process.env.SPHERES_BINARY)),binaryHash=audit.fileHash(binary),manifestPath=regular(path.resolve(process.env.SPHERES_S10G_SANCTIONS_FIXTURE));
  const manifest=JSON.parse(fs.readFileSync(manifestPath,'utf8')),fixtureDir=path.dirname(manifestPath);assert.equal(manifest.fixture,'s10g-authored-native-sanctions-desk');assert.equal(manifest.version,1);assert.equal(manifest.compiled_revision,git(['rev-parse','--short=12',expected]).toString().trim());assert.equal(manifest.player,'France');assert.equal(manifest.days_advanced,0);assert.deepEqual(manifest.steps.map(s=>s.kind),['lift','sanction','improve']);
  const filenames=['before.json','expected-after-lift.json','expected-after-sanction.json','expected-after-improve.json'];assert.equal(manifest.before_file,filenames[0]);assert.deepEqual(manifest.steps.map(s=>s.expected_file),filenames.slice(1));
  const files=Object.fromEntries(filenames.map(n=>[n,regular(path.join(fixtureDir,n))])),fixtureHashes=Object.fromEntries(Object.entries(files).map(([n,f])=>[n,audit.fileHash(f)]));fixtureHashes.manifest=audit.fileHash(manifestPath);
  const output=path.resolve(process.env.SPHERES_SANCTIONS_OUTPUT||path.join(root,'artifacts/browser-sanctions-desk-ci'));fs.mkdirSync(output,{recursive:true});const out=fs.mkdtempSync(path.join(output,'authored-france-')),run=path.join(out,'server');fs.mkdirSync(run);fs.mkdirSync(path.join(run,'saves'));fs.copyFileSync(files['before.json'],path.join(run,'saves',SLOT+'.json'),fs.constants.COPYFILE_EXCL);
  const port=await freePort(),url='http://127.0.0.1:'+port,server=cp.spawn(binary,['--port',String(port),'--no-open'],{cwd:run,windowsHide:true,stdio:['ignore','pipe','pipe']});const log=fs.createWriteStream(path.join(out,'server.log'));server.stdout.pipe(log);server.stderr.pipe(log);
  let browser,page,launchError,stage='launch';server.on('error',error=>{launchError=error;});
  const e={passed:false,run,url,started_utc:new Date().toISOString(),scope:manifest.scope,fixture:{manifest_path:manifestPath,source:manifest,sha256:fixtureHashes},test_source:{revision,driver_sha256:audit.fileHash(__filename),review_assertions_sha256:audit.fileHash(require.resolve('./government-review-assertions.cjs')),runtime_source_equal:true},screenshots:[],commands:[],previews:[],government_previews:[],advances:[],errors:[],checks:[],envelope_comparisons:[]};
  const write=()=>fs.writeFileSync(path.join(out,'result.json'),JSON.stringify(e,null,2)+'\n');const telemetry=(event,details)=>fs.appendFileSync(path.join(out,'progress.jsonl'),JSON.stringify({utc:new Date().toISOString(),stage,event,...details})+'\n');const shot=async name=>{await page.screenshot({path:path.join(out,name+'.png')});e.screenshots.push(name+'.png');};audit.configure(out,telemetry);
  try{
    const until=Date.now()+20000;for(;;){if(launchError)throw launchError;assert.equal(server.exitCode,null);try{const r=await fetch(url+'/api/build',{headers:{Connection:'close'},signal:AbortSignal.timeout(2000)});await r.arrayBuffer();if(r.ok)break;}catch(error){if(Date.now()>=until)throw error;}assert(Date.now()<until,'Startup exceeded 20 seconds');await new Promise(r=>setTimeout(r,100));}
    browser=await chromium.launch({headless:true,...(process.env.SPHERES_BROWSER_CHANNEL?{channel:process.env.SPHERES_BROWSER_CHANNEL}:{})});e.browser_version=browser.version();const browserContext=await browser.newContext({viewport:{width:1440,height:1000},reducedMotion:'reduce'});page=await browserContext.newPage();page.setDefaultTimeout(30000);
    page.on('pageerror',error=>e.errors.push(error.message));page.on('request',r=>{if(r.method()!=='POST')return;const p=new URL(r.url()).pathname;if(p==='/api/command')e.commands.push(r.postDataJSON());if(p==='/api/decisions/preview')e.previews.push(r.postDataJSON());if(p==='/api/government/preview')e.government_previews.push(r.postDataJSON());if(p==='/api/advance')e.advances.push(r.postDataJSON());});
    e.build=await integrated.verifyBuild({page,url,root,run,binary});
    for(const name of ['agency-ui.js','agency.css','government-ui.js','government-ui.css']){const checkout=fs.readFileSync(path.join(root,'spheres-web/ui',name)),committed=git(['show',expected+':spheres-web/ui/'+name]);assert(Buffer.from(checkout.toString('utf8').replace(/\r\n/g,'\n')).equals(committed));const r=await page.request.get(url+'/'+name);try{assert(r.ok());const served=await r.body();assert(served.equals(checkout));e.build.assets[name]={served_sha256:hash(served),checkout_sha256:hash(checkout),committed_sha256:hash(committed)};}finally{await r.dispose();}}
    const oracles=Object.fromEntries(Object.entries(files).map(([n,f])=>[n,inspect(f)]));
    stage='ordinary Load of disclosed fixture';await page.goto(url,{waitUntil:'domcontentloaded'});await page.locator('#campaignHome').waitFor();await page.waitForFunction(()=>!!SESSION.live?.session_id);const hasPlayer=await page.evaluate(()=>!!SESSION.live?.player);await campaigns(page);await page.locator('#saveSlots').selectOption(SLOT);
    const loading=page.waitForResponse(r=>route(r,'/api/load'));await page.locator('#loadBtn').click();if(hasPlayer)await page.locator('#campaignConfirmAccept').click();assert((await loading).ok());await page.locator('#app').waitFor();await idle(page);assert.equal(await page.evaluate(()=>clock.running),false);const initial=await get(page,url,'/api/state');facts(initial,manifest.before);e.capabilities=await integrated.capabilities({page,url,player:'France'});
    const loaded=await capture(page,url,run,'loaded');compare(loaded,oracles['before.json'],'Ordinary Load preserves complete native world and saved history',e);
    stage='directional sanction desk at desktop and phone widths';await page.locator('#agencyBtn').click();await page.locator('#agencyPanel').waitFor();e.initial_layout=[];
    for(const width of [1440,390,320]){e.initial_layout.push(await desk(page,initial.agency,width));await page.locator('#agencySanctions h3').first().scrollIntoViewIfNeeded();await shot('sanctions-desk-'+width);}
    stage='lift review and cancellation';await page.setViewportSize({width:1440,height:1000});const cancelled=await review(page,manifest.steps[0]);e.cancelled_review=cancelled;e.cancelled_context=[];
    for(const width of [1440,390,320]){e.cancelled_context.push(await context(page,cancelled,width));await page.locator('#agencyBilateralContext h3,#agencyBilateralContext h4').first().scrollIntoViewIfNeeded();await shot('lift-context-'+width);}
    e.cancelled_effects=await reviewUI.mobileChanges(page,'agency',cancelled,async width=>shot('lift-effects-'+width));await page.locator('#agencyReview [data-agency-review-cancel]').click();await page.locator('#agencyReview').waitFor({state:'hidden'});facts(await get(page,url,'/api/state'),manifest.before);compare(await capture(page,url,run,'cancelled'),loaded,'Directional browsing, exact native review and cancellation preserve complete campaign',e);assert.equal(e.commands.length,0);
    e.actions=[];let finalArchive;
    for(const [index,step] of manifest.steps.entries()){
      stage='reviewed '+step.kind;await page.setViewportSize({width:390,height:844});const quote=await review(page,step),action={kind:step.kind,quote,context:await context(page,quote,390)};
      action.effects=await reviewUI.mobileChanges(page,'agency',quote,async width=>shot(step.kind+'-review-effects-'+width));const applying=page.waitForResponse(r=>route(r,'/api/command'));await page.locator('#agencyReview [data-agency-confirm]').click();const response=await applying;assert(response.ok());assert.deepEqual((await response.json()).errors,[]);await idle(page);
      const state=await get(page,url,'/api/state');facts(state,step.after);assert.equal(e.commands.length,index+1);const sent=e.commands[index];assert.deepEqual(sent.commands,[step.command]);assert.equal(sent.review_kind,'decisions');assert.equal(sent.review_token,quote.review_token);
      await page.waitForFunction(()=>document.activeElement===document.getElementById('agencyStatus'));action.notice=await page.locator('#agencyStatus').evaluate(el=>{const r=el.getBoundingClientRect();return {text:el.textContent,focused:document.activeElement===el,visible:r.top>=0&&r.bottom<=innerHeight&&r.left>=0&&r.right<=innerWidth};});assert(action.notice.focused&&action.notice.visible&&action.notice.text.trim());await shot(step.kind+'-recorded-390');
      finalArchive=await capture(page,url,run,step.kind);compare(finalArchive,oracles[step.expected_file],'Reviewed '+step.kind+' equals independent native world, dispatches and complete history',e);
      action.layout=await desk(page,state.agency,320);await page.locator('#agencySanctions h3').first().scrollIntoViewIfNeeded();await shot(step.kind+'-desk-320');e.actions.push(action);
    }
    const finalFacts=manifest.steps.at(-1).after;
    stage='foreign government inspection is read-only';await page.locator('#agencyClose').click();await page.setViewportSize({width:1440,height:1000});const previews=e.previews.length;await page.locator('#govBtn').click();await page.waitForFunction(()=>gov.open&&gov.data&&gov.dataState===S&&!gov.data.error);await page.locator('#govPick').selectOption('Japan');await page.waitForFunction(()=>gov.nation==='Japan'&&gov.data?.nation==='Japan'&&gov.dataState===S&&!gov.data.error);await page.locator('#gov-tab-decisions').click();await page.locator('[data-gov-filter="all"]').click();
    assert.equal(await page.evaluate(()=>gov.data.mine),false);assert.match(await page.locator('#gov-panel-decisions').innerText(),/foreign government.*inspection/i);const controls=page.locator('#gov-panel-decisions [data-gov-review]');assert(await controls.count());assert.equal(await controls.evaluateAll(a=>a.every(e=>e.disabled)),true);e.foreign_inspection={nation:'Japan',disabled_controls:await controls.count(),previews_sent:e.previews.length-previews,commands_sent:e.commands.length-3};assert.equal(e.foreign_inspection.previews_sent,0);assert.equal(e.foreign_inspection.commands_sent,0);assert.deepEqual(e.government_previews,[]);await shot('foreign-government-read-only-1440');facts(await get(page,url,'/api/state'),finalFacts);compare(await capture(page,url,run,'foreign'),finalArchive,'Foreign government inspection preserves complete native campaign',e);
    stage='named Save, cancelled Load, Load and Continue';await campaigns(page);await page.locator('#saveName').fill(SAVED);const saving=page.waitForResponse(r=>route(r,'/api/save'));await page.locator('#saveNamedBtn').click();assert((await saving).ok());await page.locator('#saveSlots option[value='+quoted(SAVED)+']').waitFor({state:'attached'});await page.locator('#saveSlots').selectOption(SAVED);const beforeLoad=await get(page,url,'/api/state');
    await page.locator('#loadBtn').click();await page.locator('#campaignConfirmCancel').click();assert.deepEqual(await get(page,url,'/api/state'),beforeLoad);compare(await capture(page,url,run,'load-cancelled'),finalArchive,'Cancelled Load preserves complete world and saved history',e);
    const reloading=page.waitForResponse(r=>route(r,'/api/load'));await page.locator('#loadBtn').click();await page.locator('#campaignConfirmAccept').click();assert((await reloading).ok());await page.locator('#app').waitFor();await idle(page);const reloaded=await get(page,url,'/api/state');assert.notEqual(reloaded.session_id,beforeLoad.session_id);facts(reloaded,finalFacts);compare(await capture(page,url,run,'reloaded'),finalArchive,'Named Save/Load preserves complete world and saved history',e);
    await page.reload({waitUntil:'domcontentloaded'});await page.locator('#continueBtn').click();await page.locator('#app').waitFor();await idle(page);facts(await get(page,url,'/api/state'),finalFacts);compare(await capture(page,url,run,'continued'),finalArchive,'Continue preserves complete world and saved history',e);await page.locator('#agencyBtn').click();await page.locator('#agencyPanel').waitFor();e.continued_layout=await desk(page,finalFacts.agency,320);await page.locator('#agencySanctions h3').first().scrollIntoViewIfNeeded();await shot('continued-sanctions-320');
    assert.equal(e.commands.length,3);assert.deepEqual(e.commands.map(c=>c.commands),manifest.steps.map(s=>[s.command]));assert.deepEqual(e.previews.map(p=>p.command),[manifest.steps[0].command,...manifest.steps.map(s=>s.command)]);assert.deepEqual(e.advances,[]);assert.deepEqual(e.government_previews,[]);assert.deepEqual(e.errors,[]);assert.equal(await page.evaluate(()=>clock.running),false);assert.equal(audit.fileHash(binary),binaryHash);assert.equal(audit.fileHash(manifestPath),fixtureHashes.manifest);for(const [n,f] of Object.entries(files))assert.equal(audit.fileHash(f),fixtureHashes[n]);assert.equal(git(['rev-parse','HEAD']).toString().trim(),revision);assert.equal(git(['status','--porcelain']).toString().trim(),'');
    e.checks.push('Exact native outgoing and incoming sanctions with no incoming lift action at 1440, 390 and 320 px','Exact bilateral review retains reverse sanction barrier, third-party pressure and saved treaty; cancellation is pure','Lift, reimpose and improve apply once each with independent native world and historical-envelope equality','Foreign government inspection sends no order or preview','Ordinary named Save, cancelled Load, actual Load and Continue preserve all native and historical fields; zero simulation days');e.saved_slot=SAVED;e.days_advanced=0;e.final_capture='captures/s10g-audit-continued.json';e.passed=true;e.finished_utc=new Date().toISOString();write();console.log(JSON.stringify({passed:true,result:path.join(out,'result.json')}));
  }catch(error){e.failed_stage=stage;e.failure=String(error.stack||error);write();if(page)try{await page.screenshot({path:path.join(out,'failure.png')});fs.writeFileSync(path.join(out,'failure.html'),await page.content());}catch{}throw error;}
  finally{if(browser)await browser.close();server.kill();log.end();}
}
main().catch(error=>{console.error(error);process.exitCode=1;});
