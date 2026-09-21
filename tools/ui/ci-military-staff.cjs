// S17: native authored starting forces, actual autonomous staff and reviewed browser controls.
// Exact
// saved-world oracles. No command/advance injection or response substitution.
const assert=require('node:assert/strict'),fs=require('node:fs'),path=require('node:path'),cp=require('node:child_process'),net=require('node:net'),crypto=require('node:crypto');
const {chromium}=require('playwright'),integrated=require('./ci-integrated.cjs'),audit=require('./supplier-archive-audit.cjs');
const root=path.resolve(__dirname,'../..'),q=x=>JSON.stringify(String(x)),copy=x=>JSON.parse(JSON.stringify(x));
const hash=b=>crypto.createHash('sha256').update(b).digest('hex'),route=(r,p)=>new URL(r.url()).pathname===p&&(typeof r.request==='function'?r.request():r).method()==='POST';
const SLOT='s17-staff-input',SAVED='s17-military-staff';
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
async function capture(page,url,run,id){assert(/^[a-z][a-z0-9_-]{0,44}$/.test(id));const saves=fs.realpathSync(path.join(run,'saves')),slot='s17-audit-'+id,file=withinPath(saves,path.join(saves,slot+'.json'));assert(!fs.existsSync(file));const r=await page.request.post(url+'/api/save',{data:{slot}});try{assert(r.ok());await r.body();}finally{await r.dispose();}regular(file);const dir=withinPath(run,path.join(run,'captures'));if(!fs.existsSync(dir))fs.mkdirSync(dir);const target=withinPath(dir,path.join(dir,slot+'.json'));assert(!fs.existsSync(target));fs.renameSync(file,target);return inspect(target);}
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
async function main(){
  const binary=regular(path.resolve(process.env.SPHERES_BINARY)),fixtureDir=fs.realpathSync(process.env.SPHERES_S17_FIXTURE_DIR),output=path.resolve(process.env.SPHERES_S17_BROWSER_OUTPUT),pin=process.env.SPHERES_EXPECTED_REVISION;
  assert.match(pin||'',/^[a-f0-9]{40}$/);const git=args=>cp.execFileSync('git',['-c','core.longpaths=true',...args],{cwd:root,windowsHide:true}).toString().trim();assert.equal(git(['rev-parse','HEAD']),pin);assert.equal(git(['status','--porcelain']),'');
  const manifest=JSON.parse(fs.readFileSync(path.join(fixtureDir,'manifest.json'),'utf8'));assert.equal(manifest.revision,pin.slice(0,12));
  assert(path.relative(root,output).startsWith('..'));fs.mkdirSync(output,{recursive:true});const out=fs.mkdtempSync(path.join(output,'staff-')),run=path.join(out,'server');fs.mkdirSync(path.join(run,'saves'),{recursive:true});fs.copyFileSync(path.join(fixtureDir,'before.json'),path.join(run,'saves',SLOT+'.json'));
  const port=await freePort(),url='http://127.0.0.1:'+port,binaryHash=audit.fileHash(binary),server=cp.spawn(binary,['--port',String(port),'--no-open'],{cwd:run,windowsHide:true,stdio:['ignore','pipe','pipe']}),log=fs.createWriteStream(path.join(out,'server.log'));server.stdout.pipe(log);server.stderr.pipe(log);
  const e={passed:false,revision:pin,binary_sha256:binaryHash,run,url,fixture:manifest.scope,commands:[],advances:[],world_comparisons:[],envelope_comparisons:[],screenshots:[],errors:[],started_utc:new Date().toISOString()};
  let browser,page;
  const write=()=>fs.writeFileSync(path.join(out,'result.json'),JSON.stringify(e,null,2)+'\n');
  audit.configure(out,()=>{});
  async function check(id,expected){const actual=await capture(page,url,run,id);compare(actual,inspect(path.join(fixtureDir,expected)),id,e);return actual;}
  async function shot(id){await page.locator('#banner').waitFor({state:'hidden'});await page.screenshot({path:path.join(out,id+'.png')});e.screenshots.push(id+'.png');}
  try{
    const until=Date.now()+30000;for(;;){assert.equal(server.exitCode,null);try{const r=await fetch(url+'/api/build',{signal:AbortSignal.timeout(2000)});await r.arrayBuffer();if(r.ok)break;}catch{}assert(Date.now()<until);await new Promise(r=>setTimeout(r,100));}
    browser=await chromium.launch({headless:true,channel:process.env.SPHERES_BROWSER_CHANNEL||'chrome'});page=await browser.newPage({viewport:{width:1440,height:1000},reducedMotion:'reduce'});page.setDefaultTimeout(60000);
    page.on('pageerror',error=>e.errors.push(error.message));page.on('request',r=>{if(route(r,'/api/command'))e.commands.push(r.postDataJSON());if(route(r,'/api/advance'))e.advances.push(r.postDataJSON());});
    e.build=await integrated.verifyBuild({page,url,root,run,binary});await page.goto(url,{waitUntil:'domcontentloaded'});await load(page,SLOT);await check('loaded','before.json');
    for(const [enabled,label,days] of [[true,'active',4],[false,'paused',2]]){
      const command={kind:'military_ai',enabled};await review(page,url,command);const reviewed=await quote(page);assert(reviewed.requirements.some(x=>x.includes('Already contracted')));
      await page.locator('.eq-review').scrollIntoViewIfNeeded();await shot(label+'-review');
      await check(label+'-pure-review',enabled?'before.json':'active.json');
      await confirm(page,command);await check(label+'-confirmed',label+'-confirmed.json');
      for(let i=0;i<days;i++)await advance(page);
      await check(label,label+'.json');await equipment(page);const data=await board(page,url);assert.equal(data.flight.staff.status,enabled?'Active':'Paused');
      const detail=page.locator('.eq-staff-details');if(await detail.getAttribute('open')===null)await detail.locator('summary').click();assert((await detail.innerText()).includes('Italy'));assert((await detail.innerText()).includes('Procurement'));
      await detail.evaluate(el=>el.scrollIntoView({block:'start'}));await within(page.locator('#equipmentRoot'),'staff desktop');await shot(label+'-staff-desktop');await page.setViewportSize({width:390,height:844});await detail.evaluate(el=>el.scrollIntoView({block:'start'}));const heading=await detail.locator('summary').boundingBox();assert(heading&&heading.y>=-1&&heading.y+heading.height<=844,'Narrow staff heading must be in the captured viewport');await within(page.locator('#equipmentRoot'),'staff narrow');await shot(label+'-staff-narrow');await page.setViewportSize({width:1440,height:1000});
    }
    await campaigns(page);await page.locator('#saveName').fill(SAVED);const response=page.waitForResponse(r=>route(r,'/api/save'));await page.locator('#saveNamedBtn').click();assert((await response).ok());await load(page,SAVED);await check('reloaded','paused.json');
    await page.reload({waitUntil:'domcontentloaded'});await page.locator('#continueBtn').click();await page.locator('#app').waitFor({state:'visible'});await idle(page);await check('continued','paused.json');
    assert.equal(e.commands.length,2);assert.equal(e.advances.length,6);assert.deepEqual(e.errors,[]);assert.equal(audit.fileHash(binary),binaryHash);assert.equal(git(['rev-parse','HEAD']),pin);assert.equal(git(['status','--porcelain']),'');e.passed=true;e.finished_utc=new Date().toISOString();write();console.log(JSON.stringify({passed:true,result:path.join(out,'result.json')}));
  }catch(error){e.failure=String(error.stack||error);write();if(page)await page.screenshot({path:path.join(out,'failure.png')}).catch(()=>{});throw error;}
  finally{if(browser)await browser.close();server.kill();log.end();}
}
main().catch(error=>{console.error(error);process.exitCode=1;});
