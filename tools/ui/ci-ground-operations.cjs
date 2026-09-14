// S11: disclosed native fixture, ordinary player controls, complete saved-world comparisons.
// No response substitution, campaign injection or direct command/advance requests.
const assert=require('node:assert/strict'),fs=require('node:fs'),path=require('node:path');
const cp=require('node:child_process'),net=require('node:net'),crypto=require('node:crypto');
const {chromium}=require('playwright'),integrated=require('./ci-integrated.cjs'),audit=require('./supplier-archive-audit.cjs');
const root=path.resolve(__dirname,'../..'),q=x=>JSON.stringify(String(x)),copy=x=>JSON.parse(JSON.stringify(x));
const hash=b=>crypto.createHash('sha256').update(b).digest('hex');
const route=(r,p)=>new URL(r.url()).pathname===p&&r.request().method()==='POST';
const SLOT='s11-authored-ground',SAVED='s11-ground-operations';
let requiredOutcomes;
const percent=x=>Number.isFinite(x)?(x*100).toFixed(1)+'%':'Not assessed';
const count=x=>Number.isFinite(x)?x.toLocaleString('en-US',{maximumFractionDigits:3}):'—';
const near=(a,b,label)=>assert(Math.abs(a-b)<1e-9,label+': '+a+' != '+b);
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
async function operations(page){
  if(await page.locator('#equipmentRoom').isVisible())await page.locator('[data-equipment-close]').click();
  if(await page.locator('#sheet').isVisible())await page.locator('#sheet > .head .close').click();
  if(await page.locator('#intelDrawer').getAttribute('aria-hidden')!=='false')await page.locator('[data-drawer="intelDrawer"]').click();
  await page.locator('#warsCard .military-operations').waitFor({state:'visible'});await idle(page);
}
async function equipment(page,tab){
  await operations(page);await page.locator('#warsCard [data-ground-equipment-tab='+q(tab)+']').click();
  await page.waitForFunction(tab=>equipmentCurrent()&&EQUIP.tab===tab,tab);
  assert(await page.locator('#equipmentRoom').isVisible());
}
async function campaigns(page){
  await closePanels(page);if(await page.locator('#app').isVisible()){
    if(await page.locator('.arc-time-menu').getAttribute('open')===null)await page.locator('.arc-time-menu > summary').click();
    await page.locator('#campaignsBtn').click();
  }
  if(!await page.locator('#savedCampaigns').isVisible())await page.locator('#openSavesBtn').click();
  await page.locator('#savedCampaigns').waitFor({state:'visible'});
}
async function load(page,slot){
  await campaigns(page);await page.locator('#saveSlots option[value='+q(slot)+']').waitFor({state:'attached'});await page.locator('#saveSlots').selectOption(slot);
  const replacing=await page.evaluate(()=>!!SESSION.live?.player),response=page.waitForResponse(r=>route(r,'/api/load'));await page.locator('#loadBtn').click();
  if(replacing){assert((await page.locator('#campaignConfirmMessage').innerText()).includes(slot));await page.locator('#campaignConfirmAccept').click();}
  assert((await response).ok());await page.locator('#app').waitFor({state:'visible'});await idle(page);
}
function nativeFacts(file){
  // A bounded read-only projection supplements the complete canonical world.
  // It also checks every non-world envelope field for save/load comparisons.
  const code=`import datetime,hashlib,json,sys
with open(sys.argv[1],encoding='utf-8-sig') as f: d=json.load(f)
required=json.loads(sys.argv[2])
w=d; wrappers=[]
while isinstance(w,dict) and isinstance(w.get('world'),dict):
 wrappers.append({k:v for k,v in w.items() if k!='world'});w=w['world']
assert isinstance(w,dict) and isinstance(w.get('nations'),list), 'Native world must contain nations'
# Native clock uses this Gregorian epoch; WorldState omits day on the first.
calendar={'year':w['year'],'month':w['month'],'day':w.get('day',1)};assert all(isinstance(v,int) for v in calendar.values())
as_of_day=(datetime.date(calendar['year'],calendar['month'],max(1,calendar['day']))-datetime.date(1990,1,1)).days
n=next(n for n in w['nations'] if n['id']=='France'); e=n.get('equipment') or {}
out={'calendar':calendar,'as_of_day':as_of_day,'holdings':n['arsenal']['held'],'receipt':e.get('ground_operations_receipt'),'ammunition':e.get('ammunition'),'maintenance_plan':e.get('maintenance_plan'),'maintenance_fraction':e.get('maintenance_fraction'),'envelope':None}
companies=w.get('companies') or {}; firms=[c for c in companies.get('firms',[]) if c['id']==required['company']];assert len(firms)==1;firm=firms[0]
def exact(rows,key):
 matches=[r for r in rows if r['id']==key];assert len(matches)<=1;return matches[0] if matches else None
out['ledgers']={'company':{'id':firm['id'],'nation':firm['nation']},'refit':exact(firm.get('refits',[]),required['refit']),'vehicle_delivery':exact(companies.get('deliveries',[]),required['vehicle_delivery']),'ammunition_delivery':exact(companies.get('ammunition_deliveries',[]),required['ammunition_delivery'])}
if 'world' in d:
 assert set(d)=={'format','version','world','history','log','history_epoch','saved_date','player','saved_unix'}
 assert d['format']=='spheres-campaign' and d['version']==1
 assert isinstance(d['saved_unix'],int) and d['saved_unix']>=0
 wall=wrappers[0].pop('saved_unix')
 b=json.dumps(wrappers,sort_keys=True,separators=(',',':'),ensure_ascii=False,allow_nan=False).encode('utf-8')
 out['envelope']={'sha256':hashlib.sha256(b).hexdigest(),'bytes':len(b),'saved_unix':wall,'history_points':len(d['history']),'dispatches':len(d['log']),'wrapper_count':len(wrappers)}
text=json.dumps(out,ensure_ascii=True,allow_nan=False);assert len(text)<4*1024*1024;print(text)`;
  const r=cp.spawnSync(process.env.SPHERES_AUDIT_PYTHON||'python',['-c',code,file,JSON.stringify(requiredOutcomes)],{windowsHide:true,encoding:'utf8',timeout:120000,maxBuffer:4*1024*1024+65536});
  if(r.error)throw r.error;assert.equal(r.status,0,'Native ground projection failed: '+r.stderr);return JSON.parse(r.stdout);
}
function inspect(file){const result=audit.inspect(regular(file),'France');assert.deepEqual(result.canonical.ignored_paths,[]);result.ground=nativeFacts(file);return result;}
function compare(left,right,message,e,withEnvelope=true){
  audit.compare(left,right,message);e.world_comparisons.push({message,left:left.input,right:right.input,canonical_sha256:left.canonical.sha256,ignored_paths:[]});
  if(withEnvelope){assert(left.ground.envelope&&right.ground.envelope);assert.equal(left.ground.envelope.sha256,right.ground.envelope.sha256,message+' (complete saved history)');assert.equal(left.ground.envelope.bytes,right.ground.envelope.bytes);e.envelope_comparisons.push({message,left:left.ground.envelope,right:right.ground.envelope,timestamp_rule:'Only saved_unix differs; retained and validated separately. No native-world or historical field omitted.'});}
}
async function capture(page,url,run,id){
  assert(/^[a-z][a-z0-9_-]{0,44}$/.test(id));const slot='s11-audit-'+id,saves=path.join(fs.realpathSync(run),'saves');assert.equal(fs.realpathSync(saves),saves);
  const file=withinPath(saves,path.join(saves,slot+'.json'));assert(!fs.existsSync(file));
  const r=await page.request.post(url+'/api/save',{data:{slot}});try{assert(r.ok());await r.body();}finally{await r.dispose();}
  regular(file);const folder=withinPath(run,path.join(run,'captures'));if(!fs.existsSync(folder))fs.mkdirSync(folder);assert.equal(fs.realpathSync(folder),folder);
  const target=withinPath(folder,path.join(folder,slot+'.json'));assert(!fs.existsSync(target));fs.renameSync(file,target);return inspect(target);
}
async function within(locator,label){const b=await locator.evaluate(e=>({width:e.clientWidth,scroll:e.scrollWidth}));assert(b.width>0&&b.scroll<=b.width+1,label+' overflows '+JSON.stringify(b));assert(!/\bNaN\b|\bundefined\b/.test(await locator.innerText()),label+' has invalid labels');return b;}
async function details(locator){if(await locator.getAttribute('open')===null)await locator.locator(':scope > summary').click();}
async function metric(locator,label,expected){const value=await locator.locator('dl > div').filter({has:locator.page().locator('dt').filter({hasText:new RegExp('^'+label.replace(/[.*+?^${}()|[\]\\]/g,'\\$&')+'$')})}).locator('dd').textContent();assert.equal(value.trim(),expected,label);}
function verifyGround(native,archive){
  const panel=native.operations.ground_fleet,facts=archive.ground;assert(panel&&Array.isArray(panel.models));
  assert.deepEqual(facts.calendar,{year:native.year,month:native.month,day:native.day},'Published calendar must match the complete native save');assert(Number.isInteger(facts.as_of_day));
  for(const model of panel.models){const h=facts.holdings.filter(h=>h.design_id===model.revision);assert.equal(h.length,1);near(model.delivered,h[0].units,'Native owned count');assert.equal(model.reserved,h[0].refit_reserved||0);near(model.available+model.reserved,model.delivered,'Available plus reservations conserves physical stock');assert(model.supported_fraction>=0&&model.supported_fraction<=1);}
  if(panel.last_report){
    assert(facts.receipt);assert.equal(panel.last_report.day,facts.receipt.day);assert.deepEqual(panel.last_report.conflicts,facts.receipt.conflicts);
    assert.deepEqual(panel.last_report.revisions.map(({name,role,platform,...r})=>r),facts.receipt.revisions,'Report must retain every exact native loss receipt field');
  }else assert.equal(facts.receipt??null,null);
  return {models:copy(panel.models),holdings:copy(facts.holdings),last_report:copy(panel.last_report),maintenance:copy(panel.maintenance),ammunition:copy(facts.ammunition),ledgers:copy(facts.ledgers)};
}
function verifyOutcomes(e,manifest){
  const required=manifest.required_outcomes,opening=e.stages[0],final=e.stages.at(-1),before=opening.ground,after=final.ground;
  assert.equal(opening.id,'loaded');assert(Number.isInteger(opening.as_of_day)&&Number.isInteger(final.as_of_day));assert(final.as_of_day>opening.as_of_day,'Actual simulation days must elapse');
  assert.equal(final.as_of_day-opening.as_of_day,manifest.days_advanced,'Browser and independent native journey elapsed different numbers of days');
  assert.equal(before.ledgers.company.id,required.company);assert.equal(before.ledgers.company.nation,'France');assert.deepEqual(after.ledgers.company,before.ledgers.company);
  for(const key of ['refit','vehicle_delivery','ammunition_delivery'])assert.equal(before.ledgers[key],null,'Required '+key+' must be created by this journey, not pre-authored');
  function confirmed(kind){const found=e.commands.filter(r=>r.payload.commands[0]?.kind===kind);assert.equal(found.length,1,'Exactly one ordinary '+kind+' command is required');return {command:found[0].payload.commands[0],stage:e.stages.find(s=>s.id===found[0].stage)};}
  function dated(day,label,afterDay=opening.as_of_day){assert(Number.isInteger(day)&&day>=afterDay&&day<=final.as_of_day,label+' must fall in the actual advanced interval');}
  const refit=after.ledgers.refit,booked=confirmed('company_refit');assert(refit&&booked.stage);assert.equal(booked.command.company,required.company);
  assert.equal(refit.id,required.refit);assert.equal(refit.product,booked.command.product);assert.equal(refit.source_revision,booked.command.source);assert.equal(refit.target_revision,required.target_revision);
  assert.equal(refit.quantity,booked.command.quantity);assert.equal(refit.completed_units,booked.command.quantity);assert.equal(refit.cancelled_units,0);assert.equal(refit.cancelled_day,null);
  assert.equal(refit.booked_day,booked.stage.as_of_day);dated(refit.settled_day,'Refit settlement');dated(refit.closed_day,'Refit completion',refit.booked_day+1);near(refit.escrow_bn,0,'Completed refit has no unearned advance');
  const held=(ground,revision)=>ground.holdings.filter(h=>h.design_id===revision).reduce((a,h)=>a+h.units,0),reserved=(ground,revision)=>ground.holdings.filter(h=>h.design_id===revision).reduce((a,h)=>a+(h.refit_reserved||0),0);
  assert.equal(reserved(before,refit.source_revision),0,'The fixture begins without an older reservation for this model');
  assert.equal(reserved(booked.stage.ground,refit.source_revision),booked.command.quantity,'Booking withdraws the requested real vehicles');
  assert.equal(reserved(after,refit.source_revision),0,'Completed conversion releases the source reservation');
  const deliveries={};
  for(const [key,kind] of [['vehicle_delivery','company_purchase'],['ammunition_delivery','company_ammo_purchase']]){
    const d=after.ledgers[key],purchase=confirmed(kind);assert(d&&purchase.stage);assert.equal(d.id,required[key]);assert.equal(d.company,required.company);assert.equal(d.buyer,'France');assert.equal(d.company,purchase.command.company);assert.equal(d.product,purchase.command.product);assert.equal(d.quantity,purchase.command.quantity);assert(d.quantity>0);
    assert.equal(d.purchased_day,purchase.stage.as_of_day);assert.equal(purchase.stage.ground.ledgers[key].delivered_day,null,'Buying stock does not deliver it immediately');dated(d.settled_day,key+' settlement');dated(d.delivered_day,key+' arrival',d.purchased_day+1);
    assert(d.total_price_bn>0,'Supplier delivery must have an actual paid price');assert.equal(d.status,'delivered');deliveries[key]=copy(d);
  }
  assert.equal(deliveries.vehicle_delivery.revision_id,required.target_revision);
  const returned=booked.command.quantity+deliveries.vehicle_delivery.quantity;assert(held(after,required.target_revision)>=held(before,required.target_revision)+returned,'Purchase and refit must both return real target vehicles to the Arsenal');
  const retired=confirmed('equipment_retire');assert.equal(retired.command.revision,required.retired_revision);assert.equal(retired.command.quantity,required.retired_quantity);
  const at=e.stages.findIndex(s=>s.id===retired.stage.id);assert(at>0);near(held(e.stages[at-1].ground,required.retired_revision)-held(retired.stage.ground,required.retired_revision),required.retired_quantity,'Confirmed retirement removes the exact available quantity');
  assert.deepEqual(retired.stage.ground.last_report,e.stages[at-1].ground.last_report,'Retirement must preserve the historical combat receipt');
  const conflicts=[...required.conflicts].sort((a,b)=>a-b);assert.equal(conflicts.length,2);assert.equal(new Set(conflicts).size,2);
  const receipts=e.stages.flatMap(s=>s.ground.last_report?[{stage:s.id,receipt:s.ground.last_report}]:[]).filter(({receipt})=>receipt.day>=opening.as_of_day&&receipt.revisions.some(r=>Number.isSafeInteger(r.lost)&&r.lost>0)&&JSON.stringify([...receipt.conflicts].sort((a,b)=>a-b))===JSON.stringify(conflicts));
  assert(receipts.length,'Actual positive whole vehicle losses must be settled nationally across both intended fronts');
  for(const {receipt} of receipts){dated(receipt.day,'Ground combat receipt');for(const r of receipt.revisions){assert(Number.isSafeInteger(r.lost)&&r.lost>=0);assert.equal(r.opening_delivered-r.lost,r.remaining_delivered);assert.equal(r.opening_reserved,r.remaining_reserved,'Combat protects refit reservations');assert.equal(r.opening_available-r.lost,r.remaining_available);}}
  const consumed=(ground)=>Object.values(ground.ammunition?.consumed||{}).reduce((a,v)=>a+v,0),consumedDelta=consumed(after)-consumed(before);assert(consumedDelta>0,'Compatible physical ammunition must be consumed during this actual journey');
  assert(after.ammunition?.last_consumption);dated(after.ammunition.last_consumption.day,'Physical ammunition receipt');
  return {new_refit:copy(refit),new_deliveries:deliveries,returned_target_units:returned,retirement:{revision:required.retired_revision,quantity:required.retired_quantity,stage:retired.stage.id},shared_combat_receipts:receipts,ammunition_consumed_delta:consumedDelta,days_advanced:final.as_of_day-opening.as_of_day};
}
async function groundReading(page,native,selector){
  const host=page.locator(selector),panel=native.operations.ground_fleet;assert(await host.isVisible());
  assert.deepEqual(await host.locator('[data-ground-revision]').evaluateAll(rows=>rows.map(e=>e.dataset.groundRevision)),panel.models.map(m=>m.revision));
  if(panel.models.length)await details(host.locator('.military-fleet-models'));
  for(const model of panel.models){const card=host.locator('[data-ground-revision='+q(model.revision)+']');assert((await card.innerText()).includes(model.name));await metric(card,'Owned',count(model.delivered));await metric(card,'Available',count(model.available));await metric(card,'In refit',count(model.reserved));assert((await card.innerText()).includes('Condition and upkeep support: '+percent(model.supported_fraction)));}
  if(panel.last_report){
    const report=host.locator('.military-ground-receipt');await details(report);assert((await report.innerText()).includes(panel.last_report.day_label));
    const rows=report.locator('.military-loss-model');assert.equal(await rows.count(),panel.last_report.revisions.length);
    for(let i=0;i<panel.last_report.revisions.length;i++){const r=panel.last_report.revisions[i],card=rows.nth(i);assert((await card.innerText()).includes(r.name||r.revision_id));await metric(card,'Vehicles lost',count(r.lost));await metric(card,'Available after combat',count(r.remaining_available));await metric(card,'Protected in refit',count(r.remaining_reserved));}
    assert((await report.innerText()).includes('not the losses of this theatre alone'));
  }
  for(const row of native.operations.deployments){
    const form=host.locator('[data-force-id='+q(row.conflict)+']');if(!await form.count())continue;
    const roles=row.capabilities?.ground_roles;if(!roles)continue;const detail=form.locator('.military-ground-roles');if(row.rung===6){assert.equal(await detail.count(),0,'Standoff strike commitments must not claim ground specialist effects');continue;}await details(detail);
    for(const [label,value] of [['Find targets',roles.reconnaissance],['Protected movement',roles.protected_mobility],['Fire support',row.ammunition?row.ammunition.fire_support:roles.fire_support],['Air defense',row.ammunition?row.ammunition.air_defense:roles.air_defense]])await metric(detail,label,percent(value));
  }
  await within(host,'Ground operations');
}
function actions(data){const found=[];function walk(v,p){if(!v||typeof v!=='object')return;if(Array.isArray(v)){v.forEach((x,i)=>walk(x,p?`${p}.${i}`:String(i)));return;}if(Array.isArray(v.actions))v.actions.forEach((action,i)=>found.push({action,path:p?`${p}.actions.${i}`:`actions.${i}`}));for(const [key,x] of Object.entries(v))if(key!=='actions')walk(x,p?`${p}.${key}`:key);}walk(data,'');return found;}
function sameCommand(actual,expected){for(const [key,value] of Object.entries(expected))assert.deepEqual(actual[key],value,'Command field '+key);assert.deepEqual(Object.keys(actual).filter(k=>k!=='quote').sort(),Object.keys(expected).sort(),'No extra command fields');}
async function quote(page){await page.waitForFunction(()=>equipmentReviewQuoteCurrent());return page.evaluate(()=>JSON.parse(JSON.stringify(EQUIP.review.quote)));}
async function review(page,url,command){
  const tab=command.kind.startsWith('equipment_ammo')||command.kind.startsWith('company_ammo')?'ammunition':command.kind==='company_purchase'||command.kind==='company_import_purchase'?'companies':'service';
  await equipment(page,tab);const data=await board(page,url);assert.deepEqual(await page.evaluate(()=>JSON.parse(JSON.stringify(EQUIP.data))),data);
  const candidates=actions(data).filter(({action})=>{if(action.command?.kind!==command.kind||action.enabled===false)return false;const editable=new Set((action.inputs||[]).map(r=>r.key));return Object.entries(command).every(([k,v])=>editable.has(k)||JSON.stringify(action.command[k])===JSON.stringify(v));});
  let found;
  for(const candidate of candidates){const control=page.locator('[data-equipment-action='+q(candidate.path)+']');if(await control.count()&&await control.first().isVisible()){found=candidate;break;}}
  if(!found&&tab==='companies'&&await page.locator('[data-equipment-detail="company-administration"]').count()){
    await details(page.locator('[data-equipment-detail="company-administration"]'));
    for(const candidate of candidates){const control=page.locator('[data-equipment-action='+q(candidate.path)+']');if(await control.count()&&await control.first().isVisible()){found=candidate;break;}}
  }
  assert(found,'No visible ordinary native action for '+JSON.stringify(command));await page.locator('[data-equipment-action='+q(found.path)+']').first().click();await quote(page);
  for(const field of found.action.inputs||[]){if(!(field.key in command))continue;const value=String(command[field.key]),control=page.locator('[data-equipment-order-input='+q(field.key)+']');if(await control.inputValue()===value)continue;if(field.type==='select')await control.selectOption(value);else await control.fill(value);await quote(page);}
  const result=await quote(page);assert.equal(result.valid,true,JSON.stringify(result.blockers));const selected=result.actions.find(a=>a.command?.kind===command.kind&&a.enabled!==false);assert(selected);sameCommand(selected.command,command);return result;
}
async function confirm(page,command){
  const value=await quote(page),index=value.actions.findIndex(a=>a.command?.kind===command.kind&&a.enabled!==false);assert(index>=0);sameCommand(value.actions[index].command,command);
  const response=page.waitForResponse(r=>route(r,'/api/command'));await page.locator('[data-equipment-intent='+q(index)+']').click();const r=await response;assert(r.ok());const result=await r.json();assert(!result.command_pending);assert.deepEqual(result.errors||[],[]);assert.deepEqual(r.request().postDataJSON().commands,[value.actions[index].command]);await idle(page);return copy(value.actions[index].command);
}
async function allocate(page,command){
  await operations(page);const form=page.locator('#warsCard [data-force-id='+q(command.conflict)+']');await form.locator('select').selectOption(command.share_bp==null?'':String(command.share_bp));
  const response=page.waitForResponse(r=>route(r,'/api/command'));await form.locator('button[type="submit"]').click();const r=await response;assert(r.ok());assert.deepEqual(r.request().postDataJSON().commands,[command]);const v=await r.json();assert(!v.command_pending);assert.deepEqual(v.errors||[],[]);await idle(page);
}
async function advance(page){
  await closePanels(page);const response=page.waitForResponse(r=>route(r,'/api/advance'));await page.locator('#stepBtn').click();const r=await response;assert(r.ok());assert.equal(r.request().postDataJSON().days,1);const result=await r.json();assert(!result.advance_pending);assert.deepEqual(result.errors||[],[]);await idle(page);return result;
}
async function main(){
  assert(process.env.SPHERES_BINARY);assert(process.env.SPHERES_S11_FIXTURE_DIR);assert.match(process.env.SPHERES_EXPECTED_REVISION||'',/^[a-f0-9]{40}$/);
  const binary=regular(path.resolve(process.env.SPHERES_BINARY)),binaryHash=audit.fileHash(binary),fixtureDir=fs.realpathSync(process.env.SPHERES_S11_FIXTURE_DIR),fixtureFile=regular(path.join(fixtureDir,'before.json')),fixtureManifest=regular(path.join(fixtureDir,'manifest.json')),manifest=JSON.parse(fs.readFileSync(fixtureManifest,'utf8'));
  assert.equal(manifest.player,'France');assert(manifest.commands);assert(manifest.required_outcomes);requiredOutcomes=manifest.required_outcomes;
  for(const key of ['company','refit','vehicle_delivery','ammunition_delivery','retired_quantity'])assert(Number.isSafeInteger(requiredOutcomes[key])&&requiredOutcomes[key]>0,'Missing native outcome '+key);
  for(const key of ['target_revision','retired_revision'])assert(typeof requiredOutcomes[key]==='string'&&requiredOutcomes[key]);assert(Array.isArray(requiredOutcomes.conflicts));
  const git=args=>cp.execFileSync('git',['-c','core.longpaths=true',...args],{cwd:root,windowsHide:true,maxBuffer:32*1024*1024});
  assert.equal(manifest.compiled_revision,git(['rev-parse','--short=12',process.env.SPHERES_EXPECTED_REVISION]).toString().trim(),'Native fixture and browser binary must come from the same clean committed source');
  const revision=git(['rev-parse','HEAD']).toString().trim();assert.equal(git(['status','--porcelain']).toString().trim(),'','Require a clean committed browser candidate');
  assert.equal(git(['diff','--name-only',process.env.SPHERES_EXPECTED_REVISION,revision,'--','spheres-sim','spheres-web','Cargo.toml','Cargo.lock']).toString().trim(),'','Candidate runtime differs from compiled source');
  const output=path.resolve(process.env.SPHERES_S11_BROWSER_OUTPUT||path.join(root,'artifacts/browser-ground-operations-ci'));fs.mkdirSync(output,{recursive:true});const out=fs.mkdtempSync(path.join(output,'france-')),run=path.join(out,'server');fs.mkdirSync(path.join(run,'saves'),{recursive:true});
  fs.copyFileSync(fixtureFile,path.join(run,'saves',SLOT+'.json'));const port=await freePort(),url='http://127.0.0.1:'+port;
  const server=cp.spawn(binary,['--port',String(port),'--no-open'],{cwd:run,windowsHide:true,stdio:['ignore','pipe','pipe']}),log=fs.createWriteStream(path.join(out,'server.log'));server.stdout.pipe(log);server.stderr.pipe(log);
  let browser,page,stage='launch',launchError;server.on('error',error=>{launchError=error;});
  const e={passed:false,url,run,started_utc:new Date().toISOString(),fixture:{path:fixtureFile,sha256:audit.fileHash(fixtureFile),manifest_path:fixtureManifest,manifest_sha256:audit.fileHash(fixtureManifest),note:'Disclosed native authored France scenario. Starting property and conflicts are fixture conditions; no claim that this is an unassisted campaign.'},test_source:{revision,driver_sha256:hash(fs.readFileSync(__filename))},commands:[],advances:[],screenshots:[],errors:[],stages:[],world_comparisons:[],envelope_comparisons:[],checks:[]};
  const write=(name,value)=>fs.writeFileSync(path.join(out,name),JSON.stringify(value,null,2)+'\n');
  const mark=value=>{stage=value;fs.appendFileSync(path.join(out,'progress.jsonl'),JSON.stringify({utc:new Date().toISOString(),stage})+'\n');};
  const telemetry=(event,details={})=>fs.appendFileSync(path.join(out,'progress.jsonl'),JSON.stringify({utc:new Date().toISOString(),stage,event,...details})+'\n');audit.configure(out,telemetry);
  const shot=async(name,selector)=>{if(selector)await page.locator(selector).scrollIntoViewIfNeeded();await page.screenshot({path:path.join(out,name+'.png')});e.screenshots.push(name+'.png');};
  const inspectStage=async(id,previous)=>{const native=await state(page,url),saved=await capture(page,url,run,id),reading=verifyGround(native,saved),file=manifest.expected_stages?.[id];
    if(file){const expected=inspect(regular(withinPath(fixtureDir,path.resolve(fixtureDir,file))));compare(saved,expected,'Exact native fixture stage '+id,e,false);}
    write(id+'-state.json',native);write(id+'-ground.json',reading);e.stages.push({id,date:native.date,as_of_day:saved.ground.as_of_day,archive:saved.input,ground:reading,expected_stage:file||null});return {native,saved};};
  try{
    const until=Date.now()+20000;for(;;){if(launchError)throw launchError;assert.equal(server.exitCode,null);try{const r=await fetch(url+'/api/build',{headers:{Connection:'close'},signal:AbortSignal.timeout(2000)});await r.arrayBuffer();if(r.ok)break;}catch(error){if(Date.now()>=until)throw error;}assert(Date.now()<until);await new Promise(r=>setTimeout(r,100));}
    browser=await chromium.launch({headless:true,...(process.env.SPHERES_BROWSER_CHANNEL?{channel:process.env.SPHERES_BROWSER_CHANNEL}:{})});e.browser_version=browser.version();page=await browser.newPage({viewport:{width:1440,height:1000},reducedMotion:'reduce'});page.setDefaultTimeout(30000);
    page.on('pageerror',error=>e.errors.push(error.message));page.on('request',request=>{const p=new URL(request.url()).pathname;if(request.method()==='POST'&&p==='/api/command')e.commands.push({stage,payload:request.postDataJSON()});if(request.method()==='POST'&&p==='/api/advance')e.advances.push({stage,payload:request.postDataJSON()});});
    e.build=await integrated.verifyBuild({page,url,root,run,binary});
    for(const name of ['operations-ui.js','operations-ui.css']){const checkout=fs.readFileSync(path.join(root,'spheres-web/ui',name)),committed=git(['show',process.env.SPHERES_EXPECTED_REVISION+':spheres-web/ui/'+name]);assert(Buffer.from(checkout.toString('utf8').replace(/\r\n/g,'\n')).equals(committed));const r=await page.request.get(url+'/'+name);try{assert(r.ok());const served=await r.body();assert(served.equals(checkout));e.build.assets[name]={served_sha256:hash(served),committed_sha256:hash(committed),served_bytes:served.length};}finally{await r.dispose();}}
    mark('load authored fixture');await page.goto(url,{waitUntil:'domcontentloaded'});await page.locator('#campaignHome').waitFor();await load(page,SLOT);let current=await inspectStage('loaded');assert.equal(current.native.player,'France');assert.equal(await page.evaluate(()=>clock.running),false);e.capabilities=await integrated.capabilities({page,url,player:'France'});
    const fixture=inspect(fixtureFile);compare(current.saved,fixture,'Ordinary Load preserves the complete authored native world',e,false);
    await operations(page);await groundReading(page,current.native,'#warsCard .military-operations');await shot('ground-opening-desktop','#warsCard .military-ground-fleet');
    for(const tab of ['service','ammunition','companies']){await equipment(page,tab);await within(page.locator('#equipmentRoot'),'Equipment '+tab);await shot('ground-shortcut-'+tab,'#equipmentRoot');}
    compare(await capture(page,url,run,'navigation'),current.saved,'Browsing all equipment shortcuts changes no campaign property or history',e);
    const steps=manifest.steps||[
      ...['allocation_zero','allocation_auto','maintenance','ammunition_activate','refit','retire'].filter(k=>manifest.commands[k]).map(id=>({id,command:id})),
      ...Array.from({length:manifest.advance_days||0},(_,i)=>({id:'day_'+(i+1),days:1}))];assert(steps.length>0);
    assert(steps.every(step=>typeof manifest.expected_stages?.[step.id]==='string'),'Every command and advance needs its independently generated native expected stage');
    for(const step of steps){
      assert(/^[a-z][a-z0-9_]{0,35}$/.test(step.id));mark(step.id);
      if(step.days!=null){assert.equal(step.days,1,'Every fixture step must advance exactly one day through the visible control');await advance(page);}
      else{
        const command=manifest.commands[step.command];assert(command);
        if(command.kind==='force_allocation')await allocate(page,command);
        else{
          const countBefore=e.commands.length,first=await review(page,url,command);write(step.id+'-cancelled-review.json',first);await within(page.locator('#equipmentRoot'),step.id+' review');await shot(step.id+'-review','.eq-review');
          compare(await capture(page,url,run,step.id+'-review'),current.saved,step.id+' review is complete-world pure',e);
          await page.locator('[data-equipment-dismiss]').click();await page.waitForFunction(()=>equipmentCurrent()&&!EQUIP.review);
          compare(await capture(page,url,run,step.id+'-cancelled'),current.saved,step.id+' cancellation is complete-world pure',e);assert.equal(e.commands.length,countBefore);
          await review(page,url,command);const confirmed=await confirm(page,command);write(step.id+'-confirmed-command.json',confirmed);
        }
      }
      current=await inspectStage(step.id,current);await operations(page);await groundReading(page,current.native,'#warsCard .military-operations');
    }
    const expectedCommands=steps.filter(s=>s.command).map(s=>manifest.commands[s.command]);assert.equal(e.commands.length,expectedCommands.length,'No extra player command was submitted');
    e.commands.forEach((r,i)=>{assert.equal(r.payload.commands.length,1);sameCommand(r.payload.commands[0],expectedCommands[i]);});
    assert.equal(e.advances.length,steps.filter(s=>s.days===1).length,'Every one-day step is deliberate and recorded once');
    assert(e.commands.some(r=>r.payload.commands.some(c=>c.kind==='company_refit')),'The journey must book a real manufacturer refit');
    assert(e.commands.some(r=>r.payload.commands.some(c=>c.kind==='equipment_retire')),'The journey must retire real available vehicles');
    e.required_outcomes=verifyOutcomes(e,manifest);write('required-outcomes.json',e.required_outcomes);
    mark('desktop and narrow ground reports');
    for(const width of [1440,390,320]){await page.setViewportSize({width,height:width===1440?1000:844});await operations(page);await groundReading(page,current.native,'#warsCard .military-operations');await within(page.locator('#intelDrawer'),'World drawer '+width);await shot('ground-final-'+width,'#warsCard .military-ground-fleet');
      if(current.native.operations.ground_fleet.last_report)await shot('ground-losses-'+width,'#warsCard .military-ground-receipt');
      const own=current.native.wars.find(w=>w.operation?.enabled);assert(own);await page.locator('#warsCard .warcard[onclick='+q('openConflict('+own.id+')')+']').click();await page.locator('#sheet .campaign-command').waitFor({state:'visible'});assert((await page.locator('#sheet .campaign-command').innerText()).includes('Arrived in theatre'));await groundReading(page,current.native,'#sheet .military-operations');await within(page.locator('#sheet'),'Conflict sheet '+width);await shot('ground-theatre-'+width,'#sheet .campaign-command');await page.locator('#sheet > .head .close').click();}
    mark('named Save Load Continue');await page.setViewportSize({width:1440,height:1000});await campaigns(page);await page.locator('#saveName').fill(SAVED);const save=page.waitForResponse(r=>route(r,'/api/save'));await page.locator('#saveNamedBtn').click();assert((await save).ok());
    await page.locator('#saveSlots option[value='+q(SAVED)+']').waitFor({state:'attached'});await page.locator('#saveSlots').selectOption(SAVED);await page.locator('#loadBtn').click();await page.locator('#campaignConfirmCancel').click();
    compare(await capture(page,url,run,'load-cancelled'),current.saved,'Cancelled Load preserves all native property and history',e);await load(page,SAVED);const loaded=await inspectStage('reloaded');compare(loaded.saved,current.saved,'Named Save/Load preserves all native property, losses and history',e);
    await page.reload({waitUntil:'domcontentloaded'});await page.locator('#continueBtn').click();await page.locator('#app').waitFor({state:'visible'});await idle(page);const continued=await inspectStage('continued');compare(continued.saved,current.saved,'Continue preserves all native property, losses and history',e);
    assert.notEqual(loaded.native.session_id,current.native.session_id);assert.equal(continued.native.date,current.native.date);assert.deepEqual(continued.native.operations,current.native.operations);
    await operations(page);await groundReading(page,continued.native,'#warsCard .military-operations');await shot('ground-continued','#warsCard .military-ground-fleet');
    assert.deepEqual(e.errors,[]);assert.equal(audit.fileHash(binary),binaryHash);assert.equal(audit.fileHash(fixtureFile),e.fixture.sha256);assert.equal(audit.fileHash(fixtureManifest),e.fixture.manifest_sha256);assert.equal(git(['rev-parse','HEAD']).toString().trim(),revision);assert.equal(git(['status','--porcelain']).toString().trim(),'');
    e.saved_slot=SAVED;e.final_state={player:continued.native.player,date:continued.native.date,as_of_day:continued.saved.ground.as_of_day};e.elapsed_days=continued.saved.ground.as_of_day-e.stages[0].as_of_day;e.passed=true;e.finished_utc=new Date().toISOString();write('result.json',e);console.log(JSON.stringify({passed:true,result:path.join(out,'result.json')}));
  }catch(error){e.failed_stage=stage;e.failure=String(error.stack||error);write('result.json',e);if(page)try{await page.screenshot({path:path.join(out,'failure.png')});fs.writeFileSync(path.join(out,'failure.html'),await page.content());}catch{}throw error;}
  finally{if(browser)await browser.close();server.kill();log.end();}
}
main().catch(error=>{console.error(error);process.exitCode=1;});
