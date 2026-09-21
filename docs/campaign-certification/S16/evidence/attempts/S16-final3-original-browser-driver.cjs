// S16: native authored defense scenarios, ordinary reviewed browser controls and exact
// saved-world oracles. No command/advance injection or response substitution.
const assert=require('node:assert/strict'),fs=require('node:fs'),path=require('node:path'),cp=require('node:child_process'),net=require('node:net'),crypto=require('node:crypto');
const {chromium}=require('playwright'),integrated=require('./ci-integrated.cjs'),audit=require('./supplier-archive-audit.cjs');
const root=path.resolve(__dirname,'../..'),q=x=>JSON.stringify(String(x)),copy=x=>JSON.parse(JSON.stringify(x));
const hash=b=>crypto.createHash('sha256').update(b).digest('hex'),route=(r,p)=>new URL(r.url()).pathname===p&&r.request().method()==='POST';
const SLOT='s16-authored-defense',SAVED='s16-air-defense';
function regular(file){assert(fs.lstatSync(file).isFile()&&!fs.lstatSync(file).isSymbolicLink());assert.equal(fs.realpathSync(file),file);return file;}
function withinPath(parent,child){const relative=path.relative(parent,child);assert(relative&&!relative.startsWith('..'+path.sep)&&relative!=='..'&&!path.isAbsolute(relative),'Path escaped its declared directory');return child;}
async function freePort(){const s=net.createServer();await new Promise(r=>s.listen(0,'127.0.0.1',r));const port=s.address().port;await new Promise(r=>s.close(r));return port;}
async function get(page,url,p){const r=await page.request.get(url+p);try{assert(r.ok(),p);return await r.json();}finally{await r.dispose();}}
async function state(page,url){return get(page,url,'/api/state');}
async function board(page,url){const s=await state(page,url);return get(page,url,'/api/equipment?session_id='+encodeURIComponent(s.session_id));}
async function idle(page){await page.waitForFunction(()=>!SESSION.busy&&!COMMAND_CHANNEL.busy&&!COMMAND_CHANNEL.pending&&!advancing&&!pendingAdvance&&!EQUIP.busy);}
async function closePanels(page){
  if(await page.locator('#equipmentRoom').isVisible())await page.locator('[data-equipment-close]').click();
  if(await page.locator('#sheet').isVisible())await page.locator('#sheet > .head .close').click();
  if(await page.locator('#intelDrawer').getAttribute('aria-hidden')==='false')await page.locator('#intelDrawer [data-close-drawers]').click();
  if(await page.locator('#cabinetDrawer').isVisible())await page.locator('#cabinetDrawer [data-close-drawers]').click();
}
async function equipment(page,tab='flight'){
  if(!await page.locator('#equipmentRoom').isVisible()){
    if(await page.locator('#sheet').isVisible())await page.locator('#sheet > .head .close').click();
    if(await page.locator('#intelDrawer').getAttribute('aria-hidden')!=='false')await page.locator('[data-drawer="intelDrawer"]').click();
    await page.locator('#warsCard [data-ground-equipment-tab="service"]').click();
    await page.waitForFunction(()=>equipmentCurrent());
  }
  if(await page.locator('[role="tab"][data-equipment-tab='+q(tab)+']').getAttribute('aria-selected')!=='true')await page.locator('[role="tab"][data-equipment-tab='+q(tab)+']').click();
  await page.waitForFunction(tab=>equipmentCurrent()&&EQUIP.tab===tab,tab);
}
async function campaigns(page){await closePanels(page);if(await page.locator('#app').isVisible()){if(await page.locator('.arc-time-menu').getAttribute('open')===null)await page.locator('.arc-time-menu > summary').click();await page.locator('#campaignsBtn').click();}if(!await page.locator('#savedCampaigns').isVisible())await page.locator('#openSavesBtn').click();await page.locator('#savedCampaigns').waitFor({state:'visible'});}
async function load(page,slot){await campaigns(page);await page.locator('#saveSlots option[value='+q(slot)+']').waitFor({state:'attached'});await page.locator('#saveSlots').selectOption(slot);const replacing=await page.evaluate(()=>!!SESSION.live?.player),response=page.waitForResponse(r=>route(r,'/api/load'));await page.locator('#loadBtn').click();if(replacing){assert((await page.locator('#campaignConfirmMessage').innerText()).includes(slot));await page.locator('#campaignConfirmAccept').click();}assert((await response).ok());await page.locator('#app').waitFor({state:'visible'});await idle(page);}
function nativeFacts(file){
  const code=`import datetime,hashlib,json,sys
with open(sys.argv[1],encoding='utf-8-sig') as f:d=json.load(f)
w=d;wrappers=[]
while isinstance(w,dict) and isinstance(w.get('world'),dict):
 wrappers.append({k:v for k,v in w.items() if k!='world'});w=w['world']
assert isinstance(w,dict) and isinstance(w.get('nations'),list)
n=next(n for n in w['nations'] if n['id']=='France');eq=n.get('equipment') or {};co=w.get('companies') or {}
calendar={'year':w['year'],'month':w['month'],'day':w.get('day',1)}
out={'calendar':calendar,'as_of_day':(datetime.date(calendar['year'],calendar['month'],max(1,calendar['day']))-datetime.date(1990,1,1)).days,'holdings':n['arsenal']['held'],'aviation':n.get('aviation'),'airbases':w.get('airbases'),'missions':w.get('air_missions'),'ammunition':eq.get('ammunition'),'support':eq.get('air_support'),'construction':n.get('program_budget'),'firms':[f for f in co.get('firms',[]) if f['nation']=='France'],'deliveries':[x for x in co.get('deliveries',[]) if x.get('buyer')=='France'],'ammunition_deliveries':[x for x in co.get('ammunition_deliveries',[]) if x.get('buyer')=='France'],'envelope':None}
if 'world' in d:
 assert set(d)=={'format','version','world','history','log','history_epoch','saved_date','player','saved_unix'}
 assert d['format']=='spheres-campaign' and d['version']==1
 assert isinstance(d['saved_unix'],int) and d['saved_unix']>=0
 wall=wrappers[0].pop('saved_unix');b=json.dumps(wrappers,sort_keys=True,separators=(',',':'),ensure_ascii=False,allow_nan=False).encode('utf-8')
 out['envelope']={'sha256':hashlib.sha256(b).hexdigest(),'bytes':len(b),'saved_unix':wall,'history_points':len(d['history']),'dispatches':len(d['log'])}
text=json.dumps(out,ensure_ascii=True,allow_nan=False);assert len(text)<8*1024*1024;print(text)`;
  const r=cp.spawnSync(process.env.SPHERES_AUDIT_PYTHON||'python',['-c',code,file],{windowsHide:true,encoding:'utf8',timeout:120000,maxBuffer:8*1024*1024+65536});if(r.error)throw r.error;assert.equal(r.status,0,'Native flight projection failed: '+r.stderr);return JSON.parse(r.stdout);
}
function inspect(file){const result=audit.inspect(regular(file),'France');assert.deepEqual(result.canonical.ignored_paths,[]);result.flight=nativeFacts(file);return result;}
function compare(left,right,message,e,envelope=true){audit.compare(left,right,message);e.world_comparisons.push({message,left:left.input,right:right.input,sha256:left.canonical.sha256,ignored_paths:[]});if(envelope){assert(left.flight.envelope&&right.flight.envelope);assert.equal(left.flight.envelope.sha256,right.flight.envelope.sha256,message+' (saved history)');assert.equal(left.flight.envelope.bytes,right.flight.envelope.bytes);e.envelope_comparisons.push({message,left:left.flight.envelope,right:right.flight.envelope,rule:'Only separately validated saved_unix may differ; native world and historical fields remain exact.'});}}
async function capture(page,url,run,id){assert(/^[a-z][a-z0-9_-]{0,44}$/.test(id));const saves=fs.realpathSync(path.join(run,'saves')),slot='s16-audit-'+id,file=withinPath(saves,path.join(saves,slot+'.json'));assert(!fs.existsSync(file));const r=await page.request.post(url+'/api/save',{data:{slot}});try{assert(r.ok());await r.body();}finally{await r.dispose();}regular(file);const dir=withinPath(run,path.join(run,'captures'));if(!fs.existsSync(dir))fs.mkdirSync(dir);const target=withinPath(dir,path.join(dir,slot+'.json'));assert(!fs.existsSync(target));fs.renameSync(file,target);return inspect(target);}
async function within(locator,label){const size=await locator.evaluate(e=>({width:e.clientWidth,scroll:e.scrollWidth}));assert(size.width>0&&size.scroll<=size.width+1,label+' overflows '+JSON.stringify(size));assert(!/\bNaN\b|\bundefined\b/.test(await locator.innerText()),label+' has invalid labels');}
function actions(data){const found=[];function walk(v,p){if(!v||typeof v!=='object')return;if(Array.isArray(v)){v.forEach((x,i)=>walk(x,p?`${p}.${i}`:String(i)));return;}if(Array.isArray(v.actions))v.actions.forEach((action,i)=>found.push({action,path:p?`${p}.actions.${i}`:`actions.${i}`}));for(const [key,x]of Object.entries(v))if(key!=='actions')walk(x,p?`${p}.${key}`:key);}walk(data,'');for(const [index,action] of (data.flight?.base_actions||[]).entries())found.push({action,path:`flight.base_actions.${index}`});return found;}
function at(value,keys){for(const key of keys){if(value==null)return undefined;value=value[key];}return value;}
function withoutInputs(value,inputs){const out=copy(value);for(const f of inputs){const keys=f.path||[f.key],parent=at(out,keys.slice(0,-1));if(parent&&typeof parent==='object')delete parent[keys.at(-1)];}return out;}
function sameCommand(actual,expected){const a=copy(actual);delete a.quote;assert.deepEqual(a,expected,'The confirmed native command must match the intended ordinary control values');}
async function quote(page){await page.waitForFunction(()=>equipmentReviewQuoteCurrent());return page.evaluate(()=>JSON.parse(JSON.stringify(EQUIP.review.quote)));}
async function review(page,url,command,valid=true){
  const tab=command.kind==='company_purchase'?'companies':command.kind.startsWith('company_ammo')||command.kind.startsWith('equipment_ammo')?'ammunition':'flight';
  await equipment(page,tab);const data=await board(page,url);assert.deepEqual(await page.evaluate(()=>JSON.parse(JSON.stringify(EQUIP.data))),copy(data));
  const candidates=actions(data).filter(({action})=>{if(action.command?.kind!==command.kind||action.enabled===false)return false;try{assert.deepEqual(withoutInputs(action.command,action.inputs||[]),withoutInputs(command,action.inputs||[]));return true;}catch{return false;}});
  let found;for(const candidate of candidates){const control=page.locator('[data-equipment-action='+q(candidate.path)+']');if(await control.count()&&await control.first().isVisible()){found=candidate;break;}}
  assert(found,'No ordinary visible action for '+JSON.stringify(command));await page.locator('[data-equipment-action='+q(found.path)+']').first().click();await quote(page);
  for(const field of found.action.inputs||[]){const requested=at(command,field.path||[field.key]);if(requested===undefined)continue;const value=String(requested),control=page.locator('[data-equipment-order-input='+q(field.key)+']');if(await control.inputValue()===value)continue;if(field.type==='select')await control.selectOption(value);else await control.fill(value);await quote(page);}
  const result=await quote(page);assert.equal(result.valid,valid,JSON.stringify(result.blockers));const selected=result.actions.find(a=>a.command?.kind===command.kind&&a.enabled!==false);if(valid){assert(selected);sameCommand(selected.command,command);}else{assert(!selected);assert(result.blockers?.length);}return result;
}
async function confirm(page,command){const value=await quote(page),index=value.actions.findIndex(a=>a.command?.kind===command.kind&&a.enabled!==false);assert(index>=0);sameCommand(value.actions[index].command,command);const response=page.waitForResponse(r=>route(r,'/api/command'));await page.locator('[data-equipment-intent='+q(index)+']').click();const r=await response;assert(r.ok());const result=await r.json();assert(!result.command_pending);assert.deepEqual(result.errors||[],[]);assert.deepEqual(r.request().postDataJSON().commands,[value.actions[index].command]);await idle(page);}
async function advance(page){await closePanels(page);const response=page.waitForResponse(r=>route(r,'/api/advance'));await page.locator('#stepBtn').click();const r=await response;assert(r.ok());assert.equal(r.request().postDataJSON().days,1);const result=await r.json();assert(!result.advance_pending);assert.deepEqual(result.errors||[],[]);await idle(page);}
function verifyOutcomes(e,manifest){
  const r=manifest.required_outcomes,by=id=>e.stages.find(s=>s.id===id).flight,first=by('loaded'),funded=by('defense_flown'),depleted=by('depleted_flown'),quiet=by('quiet_flown');
  const order=(f,key)=>f.missions.orders.find(o=>o.id===key),holding=(f,key)=>f.holdings.filter(h=>h.design_id===key).reduce((n,h)=>n+h.units,0);
  assert.equal(first.aviation??null,null);assert.equal(holding(first,r.aircraft_revision),0);
  const delivery=funded.deliveries.find(d=>d.company===r.company&&d.product===r.aircraft_product&&d.quantity===4&&d.purchased_day>=first.as_of_day);
  assert(delivery&&delivery.total_price_bn>0&&delivery.settled_day!=null&&delivery.delivered_day>delivery.purchased_day);
  assert(funded.ammunition_deliveries.some(d=>d.family==='air_missile_short_range'&&d.purchased_day>=first.as_of_day&&d.total_price_bn>0),'Routine support must buy compatible finite supplier missiles');
  assert(by('delivery_transit').aviation.squadrons.some(s=>s.transit));assert.equal(order(funded,r.cancelled_mission).status,'cancelled');
  const quietReport=order(quiet,r.quiet_mission).report,defense=order(funded,r.defense_mission).report,hostile=order(funded,r.hostile_mission).report,baseline=order(depleted,r.hostile_mission).report;
  assert(quietReport.stores_used>0&&!quietReport.contacted&&quietReport.applied_power===0&&quietReport.defense.opposing_missions===0);
  assert(defense.contacted&&defense.stores_used>0&&defense.defense.opposing_missions===1&&defense.defense.prevented_power>0);assert.equal(defense.defense.ground_defense_expected_loss,0);
  assert(hostile.defense.air_combat_expected_loss>0);assert(hostile.applied_power<baseline.applied_power);assert.equal(baseline.defense??null,null);
  assert.equal(by('depleted_before').ammunition.stocks.air_missile_short_range,0);assert(e.refusal_checks.some(x=>x.id==='depleted_defense'&&x.valid===false));
  assert.deepEqual(quietReport,manifest.comparison.quiet);assert.deepEqual(defense,manifest.comparison.defended);assert.deepEqual(hostile,manifest.comparison.hostile_with_defense);assert.deepEqual(baseline,manifest.comparison.unopposed_hostile_baseline);
  assert.equal(holding(funded,r.aircraft_revision),4-quietReport.aircraft_lost-defense.aircraft_lost);assert.equal(e.advances.length,manifest.days_advanced);
  return {paid_fighter_delivery:delivery,quiet_patrol:quietReport,funded_defense:defense,hostile_with_defense:hostile,unopposed_hostile_baseline:baseline,prevented_hostile_power:baseline.applied_power-hostile.applied_power,cancelled_mission:r.cancelled_mission,refusal_checks:e.refusal_checks,days_advanced:manifest.days_advanced,final_restored_checkpoint:'defense_flown',final_save_slot:SAVED};
}
async function main(){
  for(const key of ['SPHERES_BINARY','SPHERES_S16_FIXTURE_DIR','SPHERES_S16_BROWSER_OUTPUT'])assert(process.env[key],key+' is required');assert.match(process.env.SPHERES_EXPECTED_REVISION||'',/^[a-f0-9]{40}$/);
  const binary=regular(path.resolve(process.env.SPHERES_BINARY)),binaryHash=audit.fileHash(binary),fixtureDir=fs.realpathSync(process.env.SPHERES_S16_FIXTURE_DIR),fixtureFile=regular(path.join(fixtureDir,'before.json')),manifestFile=regular(path.join(fixtureDir,'manifest.json')),manifest=JSON.parse(fs.readFileSync(manifestFile,'utf8'));
  assert.equal(manifest.player,'France');assert(manifest.commands&&manifest.required_outcomes&&manifest.expected_stages);
  const git=args=>cp.execFileSync('git',['-c','core.longpaths=true',...args],{cwd:root,windowsHide:true,maxBuffer:32*1024*1024});
  const revision=git(['rev-parse','HEAD']).toString().trim();assert.equal(git(['status','--porcelain']).toString().trim(),'');assert.equal(manifest.compiled_revision,git(['rev-parse','--short=12',process.env.SPHERES_EXPECTED_REVISION]).toString().trim());assert.equal(git(['diff','--name-only',process.env.SPHERES_EXPECTED_REVISION,revision,'--','spheres-sim','spheres-web','Cargo.toml','Cargo.lock']).toString().trim(),'');
  const output=path.resolve(process.env.SPHERES_S16_BROWSER_OUTPUT);assert(path.relative(root,output).startsWith('..'),'Browser evidence must be outside the repository');fs.mkdirSync(output,{recursive:true});const out=fs.mkdtempSync(path.join(output,'air-defense-')),run=path.join(out,'server');fs.mkdirSync(path.join(run,'saves'),{recursive:true});fs.copyFileSync(fixtureFile,path.join(run,'saves',SLOT+'.json'));
  const importedFiles=manifest.steps.filter(s=>s.load).map(s=>({slot:'s16-'+s.load,file:regular(withinPath(fixtureDir,path.resolve(fixtureDir,s.file)))}));importedFiles.push({slot:'s16-funded-result',file:regular(withinPath(fixtureDir,path.resolve(fixtureDir,manifest.expected_stages.defense_flown)))});for(const item of importedFiles)fs.copyFileSync(item.file,path.join(run,'saves',item.slot+'.json'));
  const port=await freePort(),url='http://127.0.0.1:'+port,server=cp.spawn(binary,['--port',String(port),'--no-open'],{cwd:run,windowsHide:true,stdio:['ignore','pipe','pipe']}),log=fs.createWriteStream(path.join(out,'server.log'));server.stdout.pipe(log);server.stderr.pipe(log);
  let browser,page,stage='launch',launchError;server.on('error',error=>{launchError=error;});
  const e={passed:false,url,run,started_utc:new Date().toISOString(),fixture:{path:fixtureFile,sha256:audit.fileHash(fixtureFile),manifest_path:manifestFile,manifest_sha256:audit.fileHash(manifestFile),scope:manifest.scope},source:{revision,driver_sha256:hash(fs.readFileSync(__filename))},commands:[],advances:[],refusal_checks:[],imported_fixtures:importedFiles.map(x=>({...x,sha256:audit.fileHash(x.file)})),screenshots:[],errors:[],stages:[],world_comparisons:[],envelope_comparisons:[]};
  const write=(name,value)=>fs.writeFileSync(path.join(out,name),JSON.stringify(value,null,2)+'\n'),mark=value=>{stage=value;fs.appendFileSync(path.join(out,'progress.jsonl'),JSON.stringify({utc:new Date().toISOString(),stage})+'\n');};audit.configure(out,(event,details={})=>fs.appendFileSync(path.join(out,'progress.jsonl'),JSON.stringify({utc:new Date().toISOString(),stage,event,...details})+'\n'));
  const shot=async(name,selector)=>{if(selector)await page.locator(selector).first().scrollIntoViewIfNeeded();await page.screenshot({path:path.join(out,name+'.png')});e.screenshots.push(name+'.png');};
  const mapShot=async name=>{
    await page.waitForFunction(()=>camAnim===0&&GLOBE?.frame===0&&flightMapCurrent());
    const back=page.locator('#flightMapCard button').filter({hasText:'Back to Air Force'});await back.scrollIntoViewIfNeeded();
    await page.evaluate(()=>new Promise(resolve=>requestAnimationFrame(()=>requestAnimationFrame(resolve))));await page.waitForFunction(()=>camAnim===0&&GLOBE?.frame===0&&flightMapCurrent());
    const reading=await page.evaluate(()=>{const card=document.querySelector('#flightMapCard'),button=[...card.querySelectorAll('button')].find(b=>b.textContent==='Back to Air Force'),box=button.getBoundingClientRect(),clip=card.getBoundingClientRect();return {width:innerWidth,camera:{...ui.cam},target:FLIGHT_MAP.row.target||null,rangeKm:FLIGHT_MAP.row.rangeKm,backVisible:box.top>=Math.max(0,clip.top)&&box.bottom<=Math.min(innerHeight,clip.bottom)&&box.left>=Math.max(0,clip.left)&&box.right<=Math.min(innerWidth,clip.right)};});assert(reading.backVisible,'Map return control must be visible in the captured card');assert(reading.camera.k>1.2,'The native flight camera focus must finish before capture');(e.map_readings??=[]).push({screenshot:name+'.png',...reading});await shot(name);
  };
  const checkpoint=async id=>{const native=await state(page,url),saved=await capture(page,url,run,id),expectedFile=manifest.expected_stages[id];if(expectedFile)compare(saved,inspect(regular(withinPath(fixtureDir,path.resolve(fixtureDir,expectedFile)))),'Exact native checkpoint '+id,e,id==='intercept_before'||id==='depleted_before');write(id+'-state.json',native);e.stages.push({id,date:native.date,archive:saved.input,flight:saved.flight});return saved;};
  try{
    const until=Date.now()+20000;for(;;){if(launchError)throw launchError;assert.equal(server.exitCode,null);try{const r=await fetch(url+'/api/build',{headers:{Connection:'close'},signal:AbortSignal.timeout(2000)});await r.arrayBuffer();if(r.ok)break;}catch(error){if(Date.now()>=until)throw error;}assert(Date.now()<until);await new Promise(r=>setTimeout(r,100));}
    browser=await chromium.launch({headless:true,...(process.env.SPHERES_BROWSER_CHANNEL?{channel:process.env.SPHERES_BROWSER_CHANNEL}:{})});e.browser_version=browser.version();page=await browser.newPage({viewport:{width:1440,height:1000},reducedMotion:'reduce'});page.setDefaultTimeout(30000);
    page.on('pageerror',error=>e.errors.push(error.message));page.on('request',r=>{const p=new URL(r.url()).pathname;if(r.method()==='POST'&&p==='/api/command')e.commands.push({stage,payload:r.postDataJSON()});if(r.method()==='POST'&&p==='/api/advance')e.advances.push({stage,payload:r.postDataJSON()});});
    e.build=await integrated.verifyBuild({page,url,root,run,binary});
    for(const name of ['equipment-ui.js','equipment-ui.css','equipment-mesh.js']){const expected=fs.readFileSync(path.join(root,'spheres-web/ui',name)),r=await page.request.get(url+'/'+name);try{assert(r.ok());assert((await r.body()).equals(expected));}finally{await r.dispose();}}
    await page.goto(url,{waitUntil:'domcontentloaded'});await page.locator('#campaignHome').waitFor();mark('loaded');await load(page,SLOT);let current=await checkpoint('loaded');compare(current,inspect(fixtureFile),'Ordinary Load preserves authored native world and history',e);assert.equal(await page.evaluate(()=>clock.running),false);
    mark('inspect supplier fighter');await equipment(page,'companies');
    const catalogue=await board(page,url),fighterIndex=catalogue.companies.market.offers.findIndex(row=>row.spec?.platform==='air_fighter');assert(fighterIndex>=0,'The actual supplier fighter must be inspectable');
    await page.locator('[data-equipment-market-model='+q(fighterIndex)+']').click();await page.waitForFunction(()=>EQUIPMENT_VIEWER.controller&&JSON.parse(EQUIPMENT_VIEWER.key).platform==='air_fighter');
    await page.locator('[data-model-view="side"]').click();await page.waitForFunction(()=>document.querySelector('[data-model-canvas] canvas')?.width>0);await page.evaluate(()=>new Promise(r=>requestAnimationFrame(()=>requestAnimationFrame(r))));
    e.fighter_inspection=await page.evaluate(()=>({spec:JSON.parse(EQUIPMENT_VIEWER.key),status:document.querySelector('[data-model-status]')?.textContent,canvas:!!document.querySelector('[data-model-canvas] canvas')}));assert(e.fighter_inspection.canvas);assert(!/could not|unavailable|Preparing/.test(e.fighter_inspection.status));await shot('supplier-fighter-inspection-1440','.eq-company-model');
    compare(await capture(page,url,run,'fighter-inspection'),current,'Ordinary supplier fighter inspection preserves native world and history',e);
    for(const step of manifest.steps){
      if(step.command){mark(step.command);const command=manifest.commands[step.command];assert(command);await review(page,url,command);
        if(['quiet_patrol','defend_against_strike'].includes(step.command)){
          await within(page.locator('#equipmentRoot'),'Defense review');const conditions=page.locator('.eq-review [data-equipment-detail="order-requirements"]');if(await conditions.getAttribute('open')===null)await conditions.locator('summary').click();assert((await page.locator('.eq-review').innerText()).includes('Defends this province against Support army and Strike target missions'));
          await shot(step.command+'-review-1440','.eq-review');await page.setViewportSize({width:390,height:844});await within(page.locator('#equipmentRoot'),'Narrow defense review');await shot(step.command+'-review-390','.eq-review');await page.setViewportSize({width:1440,height:1000});
        }
        if(step.command==='defend_against_strike'){
          const reviewed=await quote(page),mapIndex=reviewed.actions.findIndex(a=>a.navigate?.action==='flight_map'&&a.navigate.target);assert(mapIndex>=0);const map=reviewed.actions[mapIndex].navigate,commandsBefore=e.commands.length;assert.equal(map.missionKind,'defend_skies');
          await page.locator('[data-equipment-intent='+q(mapIndex)+']').click();await page.locator('#flightMapCard').waitFor({state:'visible'});assert((await page.locator('#flightMapCard').innerText()).includes('Defense area: '+map.target.name));assert((await page.locator('#flightMapCard').innerText()).includes('does not attack ground forces'));
          assert.deepEqual(await page.evaluate(()=>JSON.parse(JSON.stringify(FLIGHT_MAP.row.target))),map.target);assert.equal(await page.evaluate(()=>FLIGHT_MAP.row.rangeKm),900);
          await within(page.locator('#flightMapCard'),'Defense area map');await mapShot('defense-area-map-1440');await page.setViewportSize({width:390,height:844});await within(page.locator('#flightMapCard'),'Narrow defense area map');await mapShot('defense-area-map-390');await page.setViewportSize({width:1440,height:1000});
          await page.locator('#flightMapCard button').filter({hasText:'Back to Air Force'}).click();await page.waitForFunction(()=>equipmentCurrent()&&EQUIP.tab==='flight');assert.equal(e.commands.length,commandsBefore);
          compare(await capture(page,url,run,'defense-map-review'),current,'Ordinary defense map review preserves complete native world and history',e);await review(page,url,command);
        }
        await confirm(page,command);
      }else if(step.days){mark('advance-'+step.days);for(let i=0;i<step.days;i++)await advance(page);}
      else if(step.load){mark('import-'+step.load);await load(page,'s16-'+step.load);e.imported_fixtures.find(x=>x.slot==='s16-'+step.load).loaded=true;}
      else if(step.refusal){mark(step.refusal);const commandsBefore=e.commands.length,result=await review(page,url,step.command,false);assert(result.blockers.some(b=>(typeof b==='string'?b:b.detail||b.reason||'').includes(step.reason)));assert.equal(e.commands.length,commandsBefore);e.refusal_checks.push({id:step.refusal,valid:result.valid,blockers:result.blockers,intended_command:step.command,native_reason:step.reason});await shot('depleted-defense-review-1440','.eq-review');await page.setViewportSize({width:390,height:844});await within(page.locator('#equipmentRoot'),'Narrow depleted defense review');await shot('depleted-defense-review-390','.eq-review');await page.setViewportSize({width:1440,height:1000});}
      else if(step.checkpoint){mark(step.checkpoint);current=await checkpoint(step.checkpoint);await equipment(page);await within(page.locator('#equipmentRoot'),'Air command board');
        if(['delivery_transit','quiet_flown','defense_queued','defense_flown'].includes(step.checkpoint)){
          const subject=step.checkpoint==='delivery_transit'?'section[aria-label="Your squadrons"] .eq-card':step.checkpoint==='defense_queued'?'section[aria-label="Mission orders"] .eq-card':'section[aria-label="Dated mission results"] .eq-card';
          await shot(step.checkpoint+'-1440',subject);await page.setViewportSize({width:390,height:844});await within(page.locator('#equipmentRoot'),'Narrow air command');await shot(step.checkpoint+'-390',subject);await page.setViewportSize({width:1440,height:1000});
          if(step.checkpoint==='defense_flown'){const report=await page.locator('section[aria-label="Dated mission results"]').innerText();assert(report.includes('Expected own losses · fighter combat')&&report.includes('Expected own losses · ground air defense')&&report.includes('Aircraft lost'));}
        }
      }else assert.fail('Unknown native journey step');
    }
    e.required_outcomes=verifyOutcomes(e,manifest);write('required-outcomes.json',e.required_outcomes);
    mark('restore funded interception');await load(page,'s16-funded-result');current=await checkpoint('funded_restored');compare(current,inspect(importedFiles.find(x=>x.slot==='s16-funded-result').file),'Ordinary Load restores successful funded defense for review',e);
    mark('named Save Load Continue');await campaigns(page);await page.locator('#saveName').fill(SAVED);const saveResponse=page.waitForResponse(r=>route(r,'/api/save'));await page.locator('#saveNamedBtn').click();assert((await saveResponse).ok());await load(page,SAVED);const loaded=await checkpoint('reloaded');compare(loaded,current,'Save/Load preserves exact aircraft, bases, support, missions and history',e);
    await page.reload({waitUntil:'domcontentloaded'});await page.locator('#continueBtn').click();await page.locator('#app').waitFor({state:'visible'});await idle(page);const continued=await checkpoint('continued');compare(continued,current,'Continue preserves exact completed mission and support records',e);
    await equipment(page);await shot('continued-funded-defense','section[aria-label="Dated mission results"] .eq-card');assert.deepEqual(e.errors,[]);assert.equal(audit.fileHash(binary),binaryHash);assert.equal(audit.fileHash(fixtureFile),e.fixture.sha256);assert.equal(audit.fileHash(manifestFile),e.fixture.manifest_sha256);for(const f of e.imported_fixtures)assert.equal(audit.fileHash(f.file),f.sha256);assert.equal(git(['rev-parse','HEAD']).toString().trim(),revision);assert.equal(git(['status','--porcelain']).toString().trim(),'');
    assert.equal(e.commands.length,manifest.steps.filter(s=>s.command).length);assert.equal(e.advances.length,manifest.days_advanced);e.passed=true;e.finished_utc=new Date().toISOString();write('result.json',e);console.log(JSON.stringify({passed:true,result:path.join(out,'result.json')}));
  }catch(error){e.failed_stage=stage;e.failure=String(error.stack||error);write('result.json',e);if(page)try{await page.screenshot({path:path.join(out,'failure.png')});fs.writeFileSync(path.join(out,'failure.html'),await page.content());}catch{}throw error;}
  finally{if(browser)await browser.close();server.kill();log.end();}
}
main().catch(error=>{console.error(error);process.exitCode=1;});
