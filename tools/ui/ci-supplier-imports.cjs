// S08: actual native supplier stock, reviewed imports and delivered service.
// Input is an unchanged supplied save, never synthetic campaign state. User
// actions use visible controls; API saves only capture evidence/purity snapshots.
const assert=require('node:assert/strict'),fs=require('node:fs'),path=require('node:path');
const cp=require('node:child_process'),crypto=require('node:crypto'),net=require('node:net');
const {chromium}=require('playwright'),integrated=require('./ci-integrated.cjs');
const root=path.resolve(__dirname,'../..'),hash=b=>crypto.createHash('sha256').update(b).digest('hex');
const copy=v=>JSON.parse(JSON.stringify(v));
const withoutSession=v=>{const r=copy(v);delete r.session_id;return r;};
const routeIs=(r,p)=>new URL(r.url()).pathname===p&&r.request().method()==='POST';
const q=value=>'"'+String(value).replace(/\\/g,'\\\\').replace(/"/g,'\\"')+'"';
async function port(){const s=net.createServer();await new Promise(r=>s.listen(0,'127.0.0.1',r));const p=s.address().port;await new Promise(r=>s.close(r));return p;}
async function read(page,url,p){const r=await page.request.get(url+p);assert(r.ok(),p+': '+r.status());return r.json();}
async function state(page,url){return read(page,url,'/api/state');}
async function board(page,url){const s=await state(page,url);return read(page,url,'/api/equipment?session_id='+encodeURIComponent(s.session_id));}
function worldOf(value){while(value?.world&&typeof value.world==='object')value=value.world;assert(Array.isArray(value?.nations),'Evidence save must contain the native world');return value;}
function nation(w,id){const n=w.nations.find(n=>n.id===id);assert(n,'Missing nation '+id);return n;}
function contract(w,id){const d=w.companies?.imports?.contracts?.find(d=>d.id===id);assert(d,'Missing import contract '+id);return d;}
function held(w,buyer,revision){return (nation(w,buyer).arsenal?.held||[]).filter(h=>h.design_id===revision).reduce((s,h)=>s+h.units,0);}
function sourceProduct(w,offer){const c=w.companies.firms.find(c=>c.id===offer.company&&c.nation===offer.seller_nation);assert(c);const p=c.products.find(p=>p.id===offer.product);assert(p);return p;}
async function idle(page){await page.waitForFunction(()=>!advancing&&!pendingAdvance&&!SESSION.busy&&!COMMAND_CHANNEL.busy&&!COMMAND_CHANNEL.pending&&!EQUIP.busy);}
async function closePanels(page){
  if(await page.locator('#equipmentRoom').isVisible())await page.locator('[data-equipment-close]').click();
  if(await page.locator('#cabinetDrawer').isVisible())await page.locator('#cabinetDrawer [data-close-drawers]').click();
}
async function market(page){
  if(!await page.locator('#equipmentRoom').isVisible()){
    if(!await page.locator('#cabinetDrawer').isVisible())await page.locator('[data-drawer="cabinetDrawer"]').click();
    await page.locator('#cab-tab-companies').click();await page.waitForFunction(()=>companiesCurrent());
    await page.locator('[data-company-equipment]').click();
  }else if(!await page.locator('#equipmentMarketTitle').isVisible())await page.locator('[data-equipment-tab="companies"]').first().click();
  await page.waitForFunction(()=>equipmentCurrent()&&EQUIP.tab==='companies'&&!!EQUIP.data.companies?.market);
}
async function campaigns(page){
  await closePanels(page);if(await page.locator('#app').isVisible()){
    if(await page.locator('.arc-time-menu').getAttribute('open')===null)await page.locator('.arc-time-menu > summary').click();
    await page.locator('#campaignsBtn').click();
  }
  if(!await page.locator('#savedCampaigns').isVisible())await page.locator('#openSavesBtn').click();
  await page.locator('#savedCampaigns').waitFor({state:'visible'});
}
async function loadSlot(page,slot){
  await campaigns(page);await page.locator('#saveSlots option[value='+q(slot)+']').waitFor({state:'attached'});
  await page.locator('#saveSlots').selectOption(slot);await page.locator('#loadBtn').click();
  const response=page.waitForResponse(r=>routeIs(r,'/api/load'));await page.locator('#campaignConfirmAccept').click();assert((await response).ok());
  await page.locator('#app').waitFor({state:'visible'});await idle(page);
}
async function archive(page,url,run,slot){
  const response=await page.request.post(url+'/api/save',{data:{slot}});assert(response.ok(),'Could not capture '+slot);
  return worldOf(JSON.parse(fs.readFileSync(path.join(run,'saves',slot+'.json'),'utf8')));
}
async function saveLoad(page,url,run,slot){
  const s=await state(page,url),b=await board(page,url),before=await archive(page,url,run,slot+'-before');
  await campaigns(page);await page.locator('#saveName').fill(slot);
  const save=page.waitForResponse(r=>routeIs(r,'/api/save'));await page.locator('#saveNamedBtn').click();assert((await save).ok());
  await page.locator('#saveSlots option[value='+q(slot)+']').waitFor({state:'attached'});await page.locator('#saveSlots').selectOption(slot);
  await page.locator('#loadBtn').click();assert((await page.locator('#campaignConfirmMessage').innerText()).includes(slot));
  await page.locator('#campaignConfirmCancel').click();assert.deepEqual(await state(page,url),s,'Cancelling load changed state');
  await loadSlot(page,slot);const after=await state(page,url);assert.notEqual(after.session_id,s.session_id);assert.equal(after.date,s.date);
  assert.deepEqual(withoutSession(await board(page,url)),withoutSession(b),'Save/load changed the equipment/market reading');
  assert.deepEqual(await archive(page,url,run,slot+'-after'),before,'Save/load changed native property or receipts');
  return {slot,date:s.date,new_session:true,exact_world:true,exact_market:true,cancel_pure:true};
}
async function day(page,url){
  await closePanels(page);const response=page.waitForResponse(r=>routeIs(r,'/api/advance'));await page.locator('#stepBtn').click();
  const r=await response;assert(r.ok());assert.equal(r.request().postDataJSON().days,1);const v=await r.json();assert(!v.advance_pending);await idle(page);return state(page,url);
}
async function currentQuote(page){await page.waitForFunction(()=>equipmentReviewQuoteCurrent());return page.evaluate(()=>JSON.parse(JSON.stringify(EQUIP.review.quote)));}
async function reviewOffer(page,offer){
  await market(page);await page.locator('[data-equipment-market-origin="foreign"]').click();
  await page.locator('[data-equipment-market-availability="affordable"]').click();
  const data=await page.evaluate(()=>JSON.parse(JSON.stringify(EQUIP.data))),index=data.companies.market.offers.findIndex(r=>r.id===offer.id);assert(index>=0);
  const row=data.companies.market.offers[index],action=row.actions.findIndex(a=>a.command?.kind==='company_import_purchase');assert(action>=0);
  await page.locator('[data-equipment-action='+q('companies.market.offers.'+index+'.actions.'+action)+']').click();return currentQuote(page);
}
async function confirm(page,kind){
  const quote=await currentQuote(page);assert(quote.valid,JSON.stringify(quote));const index=quote.actions.findIndex(a=>a.command?.kind===kind&&a.enabled!==false);assert(index>=0);
  const response=page.waitForResponse(r=>routeIs(r,'/api/command'));await page.locator('[data-equipment-intent="'+index+'"]').click();
  const r=await response;assert(r.ok());const result=await r.json();assert.deepEqual(result.errors||[],[]);assert(!result.command_pending);
  assert.deepEqual(r.request().postDataJSON().commands,[quote.actions[index].command],'Confirmation changed the native reviewed command');await idle(page);await market(page);return quote.actions[index].command;
}
async function noOverflow(page,selector){const b=await page.locator(selector).evaluate(e=>({width:e.clientWidth,scroll:e.scrollWidth}));assert(b.width>0&&b.scroll<=b.width+1,selector+' overflows '+JSON.stringify(b));}
async function extraAssets(page,url,revision){
  const result={};for(const name of ['equipment-model.js']){
    const checkout=fs.readFileSync(path.join(root,'spheres-web/ui',name));const r=cp.spawnSync('git',['show',revision+':spheres-web/ui/'+name],{cwd:root,windowsHide:true,maxBuffer:8*1024*1024});assert.equal(r.status,0);
    assert(Buffer.from(checkout.toString('utf8').replace(/\r\n/g,'\n')).equals(r.stdout),'Uncommitted '+name);
    const response=await page.request.get(url+'/'+name);assert(response.ok());const actual=await response.body();assert(actual.equals(checkout),'Wrong embedded '+name);
    result[name]={served_sha256:hash(actual),committed_sha256:hash(r.stdout),newline_normalization:'CRLF to LF for checkout-to-commit only'};
  }return result;
}
async function main(){
  assert(process.env.SPHERES_BINARY,'Set SPHERES_BINARY');assert.match(process.env.SPHERES_EXPECTED_REVISION||'',/^[a-f0-9]{40}$/);
  const binary=path.resolve(process.env.SPHERES_BINARY),output=path.resolve(process.env.SPHERES_SUPPLIER_OUTPUT||'artifacts/browser-supplier-imports-ci');fs.mkdirSync(output,{recursive:true});
  const external=process.env.SPHERES_SUPPLIER_URL,slot=process.env.SPHERES_SUPPLIER_SLOT||'s08-earned-stock';
  assert(/^[A-Za-z0-9_-]{1,60}$/.test(slot),'Use a simple named save slot');
  const evidenceDir=fs.mkdtempSync(path.join(output,'supplier-'));
  let run,input,url,server,log,browser,page,stage='launch',launchError;
  if(external){
    assert(process.env.SPHERES_SUPPLIER_SERVER_ROOT,'An external server must identify its disposable save root');
    run=path.resolve(process.env.SPHERES_SUPPLIER_SERVER_ROOT);url=external.replace(/\/$/,'');input=path.join(run,'saves',slot+'.json');
    assert(fs.existsSync(input),'The explicit external server needs the named input save');
  }else{
    assert(process.env.SPHERES_SUPPLIER_SAVE,'Set SPHERES_SUPPLIER_SAVE to the genuine earned-stock archive');
    input=path.resolve(process.env.SPHERES_SUPPLIER_SAVE);run=path.join(evidenceDir,'server');fs.mkdirSync(path.join(run,'saves'),{recursive:true});
    fs.copyFileSync(input,path.join(run,'saves',slot+'.json'));url='http://127.0.0.1:'+await port();
  }
  const preparation=process.env.SPHERES_SUPPLIER_PREPARATION_SAVE?path.resolve(process.env.SPHERES_SUPPLIER_PREPARATION_SAVE):null;
  const preparationSlot='s08-preparation-input',preparationHash=preparation?hash(fs.readFileSync(preparation)):null;
  if(preparation){
    const target=path.join(run,'saves',preparationSlot+'.json');
    assert(!fs.existsSync(target),'Refuse to overwrite the preparation slot in this disposable server');
    fs.copyFileSync(preparation,target);
  }
  const inputHash=hash(fs.readFileSync(input)),e={passed:false,url,run,evidence_directory:evidenceDir,fixture:{path:input,sha256:inputHash,slot,
    note:'Supplied archive copied unchanged. This runner proves its UI journey; fixture creation provenance is recorded separately.'},commands:[],screenshots:[],errors:[],advances:[]};
  if(process.env.SPHERES_SUPPLIER_PROVENANCE){const p=path.resolve(process.env.SPHERES_SUPPLIER_PROVENANCE);e.fixture.provenance={path:p,sha256:hash(fs.readFileSync(p)),record:JSON.parse(fs.readFileSync(p,'utf8'))};}
  const write=(name,v)=>fs.writeFileSync(path.join(evidenceDir,name),JSON.stringify(v,null,2)+'\n');
  const shot=async(name,selector)=>{if(selector)await page.locator(selector).scrollIntoViewIfNeeded();await page.screenshot({path:path.join(evidenceDir,name+'.png')});e.screenshots.push(name+'.png');};
  try{
    if(!external){
      server=cp.spawn(binary,['--port',new URL(url).port,'--no-open'],{cwd:run,windowsHide:true,stdio:['ignore','pipe','pipe']});
      server.on('error',error=>launchError=error);log=fs.createWriteStream(path.join(evidenceDir,'server.log'));server.stdout.pipe(log);server.stderr.pipe(log);
    }
    const deadline=Date.now()+30000;for(;;){
      if(launchError)throw launchError;if(server&&server.exitCode!==null)throw Error('Server exited '+server.exitCode);
      try{const r=await fetch(url+'/api/build',{headers:{Connection:'close'},signal:AbortSignal.timeout(Math.max(1,Math.min(2000,deadline-Date.now())))});await r.arrayBuffer();if(r.ok)break;}catch(_){}
      if(Date.now()>=deadline)throw Error('Server start timeout');await new Promise(r=>setTimeout(r,100));
    }
    browser=await chromium.launch({headless:true,channel:process.env.SPHERES_BROWSER_CHANNEL||'chrome'});e.browser_version=browser.version();
    page=await browser.newPage({viewport:{width:1440,height:1000},reducedMotion:'reduce'});page.setDefaultTimeout(30000);
    page.on('pageerror',error=>e.errors.push(error.message));page.on('request',r=>{if(r.method()==='POST'&&new URL(r.url()).pathname==='/api/command')e.commands.push(r.postDataJSON());});
    e.build=await integrated.verifyBuild({page,url,root,run,binary});e.assets=await extraAssets(page,url,e.build.revision);
    stage='load genuine input';await page.goto(url,{waitUntil:'domcontentloaded'});await page.locator('#campaignHome').waitFor({state:'visible'});await page.waitForFunction(()=>!!SESSION.live?.session_id);
    if(preparation){
      stage='genuine preparation snapshot';await loadSlot(page,preparationSlot);await market(page);
      const prepState=await state(page,url),prepBoard=await board(page,url),prepWorld=await archive(page,url,run,'s08-preparation-before');
      assert(prepBoard.companies.market.programs.length>0,'Preparation save must show actual supplier programmes');write('preparation-market.json',prepBoard);
      for(const size of [{width:1440,height:1000,name:'desktop'},{width:390,height:844,name:'mobile'}]){
        await page.setViewportSize({width:size.width,height:size.height});await noOverflow(page,'#equipmentRoot');
        await shot('preparation-market-'+size.name,'#equipmentMarketTitle');await shot('preparation-programmes-'+size.name,'#equipmentMarketProgramsTitle');
      }
      assert.deepEqual(await archive(page,url,run,'s08-preparation-after'),prepWorld,'Preparation browsing changed the paid programme');
      e.preparation={path:preparation,sha256:preparationHash,slot:preparationSlot,date:prepState.date,player:prepState.player,
        programs:prepBoard.companies.market.programs,pure:true,note:'Intentionally inspected an earlier genuine scenario snapshot, then loaded the separately supplied earned-stock snapshot. No reverse-time campaign operation or synthesized progress.'};
      await page.setViewportSize({width:1440,height:1000});
    }
    stage='load earned stock snapshot';await loadSlot(page,slot);e.opening=await state(page,url);const buyer=e.opening.player;assert(buyer);assert.equal(e.opening.simulation_cadence,'daily');
    stage='Companies to supplier market';await market(page);let data=await board(page,url);assert.deepEqual(await page.evaluate(()=>JSON.parse(JSON.stringify(EQUIP.data))),copy(data));write('initial-market.json',data);
    assert(Array.isArray(data.companies.market.programs));
    const sizes=[{width:1440,height:1000,name:'desktop'},{width:390,height:844,name:'mobile'}];
    for(const size of sizes){await page.setViewportSize({width:size.width,height:size.height});await noOverflow(page,'#equipmentRoot');await shot('market-'+size.name,'#equipmentMarketTitle');if(data.companies.market.programs.length)await shot('programs-'+size.name,'#equipmentMarketProgramsTitle');}
    await page.setViewportSize({width:1440,height:1000});
    const choose=d=>d.companies.market.offers.find(o=>o.origin==='foreign'&&o.family!=='ammunition'&&o.spec&&o.availability.ready_stock>0&&o.availability.affordable_quantity>0&&o.access.allowed&&(!process.env.SPHERES_SUPPLIER_OFFER||o.id===process.env.SPHERES_SUPPLIER_OFFER));
    let offer=choose(data);assert(offer,'The supplied genuine fixture must expose an affordable accessible foreign equipment lot');e.offer=copy(offer);
    stage='exact model and preview purity';const beforeReview=await archive(page,url,run,'s08-preview-before');
    await page.locator('[data-equipment-market-origin="foreign"]').click();await page.locator('[data-equipment-market-availability="affordable"]').click();
    const index=data.companies.market.offers.findIndex(o=>o.id===offer.id);await page.locator('[data-equipment-market-model="'+index+'"]').click();
    assert.deepEqual(await page.evaluate(()=>JSON.parse(JSON.stringify(equipmentCompanyProduct().spec))),copy(offer.spec));
    assert.equal(await page.locator('[data-equipment-model]').count(),1);await page.locator('[data-model-status]').filter({hasText:'3D model ready'}).waitFor();await shot('exact-supplier-model','#equipmentCompanyModelTitle');
    let quote=await reviewOffer(page,offer);assert(quote.valid);const initialQuantity=await page.locator('[data-equipment-order-input="quantity"]').inputValue();
    if(Math.min(offer.availability.ready_stock,offer.availability.affordable_quantity)>=2){
      const changed=Number(initialQuantity)===1?2:1;await page.locator('[data-equipment-order-input="quantity"]').fill(String(changed));
      await page.waitForFunction(n=>equipmentReviewQuoteCurrent()&&EQUIP.review.command.quantity===n,changed);
      assert.equal(await page.evaluate(()=>EQUIP.review.command.quantity),changed);e.quantity_edit={from:Number(initialQuantity),reviewed:changed};
      await page.locator('[data-equipment-order-input="quantity"]').fill(initialQuantity);quote=await currentQuote(page);
    }else e.quantity_edit={skipped:'Only one unit is within native stock/funding limits; no fabricated second unit or authority'};
    for(const size of sizes){await page.setViewportSize({width:size.width,height:size.height});await noOverflow(page,'.eq-review');await shot('purchase-review-'+size.name,'.eq-review');}
    await page.setViewportSize({width:1440,height:1000});assert.deepEqual(await archive(page,url,run,'s08-preview-after'),beforeReview,'Model/quantity review changed property or research');
    await page.locator('[data-equipment-dismiss]').click();assert.equal(e.commands.length,0,'Review-only controls placed an order');
    stage='discard delayed review after real date change';
    let release,intercepted,handled;const seen=new Promise(r=>intercepted=r),heldResponse=new Promise(r=>release=r),completedResponse=new Promise(r=>handled=r);
    await page.route('**/api/equipment-preview',async route=>{
      if(route.request().postDataJSON()?.command?.kind!=='company_import_purchase')return route.continue();
      const response=await route.fetch();intercepted();await heldResponse;await route.fulfill({response});handled();
    });
    const current=await page.evaluate(()=>JSON.parse(JSON.stringify(EQUIP.data))),oi=current.companies.market.offers.findIndex(o=>o.id===offer.id),ai=current.companies.market.offers[oi].actions.findIndex(a=>a.command?.kind==='company_import_purchase');
    await page.locator('[data-equipment-action='+q('companies.market.offers.'+oi+'.actions.'+ai)+']').click();await seen;
    const oldDate=(await state(page,url)).date;await closePanels(page);e.advances.push(await day(page,url));release();await completedResponse;await page.unroute('**/api/equipment-preview');
    await market(page);assert.notEqual((await state(page,url)).date,oldDate);assert.equal(await page.locator('.eq-review').count(),0,'Old-date review survived navigation and real advancement');assert.equal(e.commands.length,0);
    e.stale_review={actual_delayed_response:true,advanced_visible_day:true,no_command:true,discarded:true};
    data=await board(page,url);offer=choose(data);assert(offer,'Affordable actual stock disappeared after the stale-review day');
    e.purchase_offer={date:(await state(page,url)).date,offer:copy(offer),same_as_initial:offer.id===e.offer.id,
      note:'Fresh native offer selected after the real date change; this is the actual purchase source.'};
    stage='reviewed purchase';const before=await archive(page,url,run,'s08-purchase-before');quote=await reviewOffer(page,offer);assert(quote.valid);e.purchase_command=await confirm(page,'company_import_purchase');
    const after=await archive(page,url,run,'s08-purchase-after'),oldIds=new Set((before.companies.imports?.contracts||[]).map(d=>d.id));
    const added=after.companies.imports.contracts.filter(d=>!oldIds.has(d.id));assert.equal(added.length,1);const purchased=added[0];e.contract_id=purchased.id;
    assert.equal(purchased.buyer,buyer);assert.equal(purchased.quantity,e.purchase_command.quantity);assert.equal(purchased.company,offer.company);assert.equal(purchased.product,offer.product);
    assert.equal(purchased.seller,offer.seller_nation);assert.deepEqual(purchased.source_revision.spec,copy(offer.spec));
    assert(purchased.buyer_revision,'An equipment import must retain its prospective model ID');
    assert(!nation(before,buyer).equipment?.revisions?.[purchased.buyer_revision],'The genuine input already installed this imported model');
    assert.equal(held(before,buyer,purchased.buyer_revision),0,'The genuine input already supplied this import to the fleet');
    assert.equal(sourceProduct(before,offer).stock-sourceProduct(after,offer).stock,purchased.quantity,'The chosen stock was not reserved exactly once');
    assert.deepEqual(nation(after,buyer).equipment?.learned||[],nation(before,buyer).equipment?.learned||[],'Purchasing granted component research');
    assert.deepEqual(nation(after,buyer).tech,nation(before,buyer).tech,'Purchasing granted technology');
    assert(!nation(after,buyer).equipment?.revisions?.[purchased.buyer_revision],'Purchasing installed an unarrived model');
    assert.equal(held(after,buyer,purchased.buyer_revision),held(before,buyer,purchased.buyer_revision),'Unarrived import entered service');
    e.purchase={contract:purchased,stock_reserved_once:true,no_component_grant:true,no_early_service:true};
    stage='cancel preview and midpoint load';data=await board(page,url);const di=data.companies.market.deliveries.findIndex(d=>d.contract===purchased.id);assert(di>=0);
    const ca=data.companies.market.deliveries[di].actions.findIndex(a=>a.command?.kind==='company_import_cancel');assert(ca>=0);
    await page.locator('[data-equipment-action='+q('companies.market.deliveries.'+di+'.actions.'+ca)+']').click();await currentQuote(page);await shot('cancellation-review','.eq-review');await page.locator('[data-equipment-dismiss]').click();
    assert.deepEqual(await archive(page,url,run,'s08-cancel-review'),after,'Cancellation review changed paid property');
    // Settle the real accepted payment through a visible day before saving the
    // intermediate checkpoint. No early-arrival inventory is supplied by setup.
    e.advances.push(await day(page,url));const transit=await archive(page,url,run,'s08-paid-transit'),inTransit=contract(transit,purchased.id);
    assert(inTransit.settled_day!=null&&inTransit.delivered_day==null,'Intermediate checkpoint must contain a settled import still in transit');
    assert(!nation(transit,buyer).equipment?.revisions?.[purchased.buyer_revision]);assert.equal(held(transit,buyer,purchased.buyer_revision),0);
    e.midpoint=await saveLoad(page,url,run,'s08-import-midpoint');e.midpoint.phase='Paid transit before arrival';e.midpoint.model_not_installed=true;
    stage='actual delivery';const maximum=Number(process.env.SPHERES_SUPPLIER_MAX_DAYS||120);assert(Number.isSafeInteger(maximum)&&maximum>=1&&maximum<=365);
    let finalWorld;for(let i=0;i<maximum;i++){
      e.advances.push(await day(page,url));data=await board(page,url);const delivery=data.companies.market.deliveries.find(d=>d.contract===purchased.id);assert(delivery);
      if(i===0||i%7===0){await market(page);await shot('delivery-day-'+(i+1),'[data-equipment-record='+q(delivery.id)+']');}
      if(delivery.status==='Delivered'){finalWorld=await archive(page,url,run,'s08-delivered');break;}
      assert(i<maximum-1,'Import did not arrive within the declared visible-day bound: '+JSON.stringify(delivery));
    }
    const delivered=contract(finalWorld,purchased.id);assert(delivered.delivered_day!=null);assert(delivered.settled_day!=null);assert.equal(delivered.escrow_bn,0);assert.equal(delivered.refunded_bn,0);
    assert.deepEqual(delivered.source_revision,purchased.source_revision,'Delivery changed the frozen source model');
    const buyerRevision=nation(finalWorld,buyer).equipment.revisions[delivered.buyer_revision];assert(buyerRevision);assert.deepEqual(buyerRevision.spec,purchased.source_revision.spec);
    assert.equal(held(finalWorld,buyer,delivered.buyer_revision)-held(before,buyer,delivered.buyer_revision),purchased.quantity,'Delivered quantity does not match the purchased lot');
    e.delivered={contract:delivered,exact_revision:true,exact_quantity:true,escrow_released:true};
    stage='in service and maintenance';await market(page);await page.locator('[data-equipment-tab="service"]').first().click();await page.waitForFunction(()=>equipmentCurrent()&&EQUIP.tab==='service');
    const lot='[data-equipment-record='+q(delivered.buyer_revision)+']';await page.locator(lot).waitFor({state:'visible'});
    assert.match(await page.locator(lot).innerText(),/Maintenance requirement/);assert.match(await page.locator(lot).innerText(),/Last settled maintenance coverage/);
    for(const size of sizes){await page.setViewportSize({width:size.width,height:size.height});await noOverflow(page,'#equipmentRoot');await noOverflow(page,lot);await shot('delivered-service-'+size.name,lot);}
    await page.setViewportSize({width:1440,height:1000});
    stage='review native maintenance recommendation';data=await board(page,url);
    const maintenanceIndex=data.maintenance.actions.findIndex(a=>a.command?.kind==='equipment_maintenance'&&a.enabled!==false);
    assert(maintenanceIndex>=0,'The native service board must expose the existing maintenance action');
    const recommendation=copy(data.maintenance.actions[maintenanceIndex].command),maintenanceBefore=finalWorld,maintenanceDate=(await state(page,url)).date;
    assert(Number.isFinite(recommendation.daily_budget_mn)&&recommendation.daily_budget_mn>0,'The native maintenance recommendation must fund a positive ceiling');
    await page.locator('[data-equipment-action='+q('maintenance.actions.'+maintenanceIndex)+']').click();
    const maintenanceQuote=await currentQuote(page);assert(maintenanceQuote.valid,JSON.stringify(maintenanceQuote));
    assert.equal(Number(await page.locator('[data-equipment-order-input="daily_budget_mn"]').inputValue()),recommendation.daily_budget_mn,'Use the native recommended limit without synthetic funding edits');
    assert.deepEqual(await archive(page,url,run,'s08-maintenance-review'),maintenanceBefore,'Maintenance review changed money or property');
    await shot('maintenance-review','.eq-review');write('maintenance-review.json',maintenanceQuote);
    stage='approve actual maintenance plan';e.maintenance_command=await confirm(page,'equipment_maintenance');
    assert.deepEqual(e.maintenance_command,recommendation,'Maintenance confirmation changed the native recommendation');
    const maintenanceApproved=await archive(page,url,run,'s08-maintenance-approved'),approvedPlan=nation(maintenanceApproved,buyer).equipment.maintenance_plan;
    assert(approvedPlan);assert.equal(approvedPlan.daily_limit_bn,recommendation.daily_budget_mn/1000);
    const expectedApproval=copy(maintenanceBefore);nation(expectedApproval,buyer).equipment.maintenance_plan=copy(approvedPlan);
    assert.deepEqual(maintenanceApproved,expectedApproval,'Approving maintenance must only set its plan: no instant money, authority, research or property grant');
    assert.equal((await state(page,url)).date,maintenanceDate,'Maintenance approval advanced time');
    const priorReceiptDay=nation(maintenanceBefore,buyer).equipment.maintenance_plan?.receipt?.day??null;
    e.maintenance={recommendation,command:e.maintenance_command,approved_plan:copy(approvedPlan),preview_pure:true,no_instant_grant:true,advances:[]};
    stage='settle positive maintenance for imported equipment';
    for(let i=0;i<7;i++){
      const advanced=await day(page,url);e.advances.push(advanced);e.maintenance.advances.push(advanced);
      const observed=await archive(page,url,run,'s08-maintenance-day-'+(i+1)),n=nation(observed,buyer),plan=n.equipment.maintenance_plan,r=plan?.receipt;
      if(r&&r.day>=approvedPlan.from_day&&r.day>=(delivered.delivered_day??r.day)&&(priorReceiptDay==null||r.day>priorReceiptDay)&&r.custom_required_bn>0&&r.custom_paid_bn>0){
        const required=held(observed,buyer,delivered.buyer_revision)*buyerRevision.profile.maintenance_bn_day,paid=r.custom_paid_bn+r.legacy_paid_bn,tolerance=1e-12;
        assert(required>0,'The imported model must remain physically present in the billed fleet');
        assert(r.custom_required_bn+tolerance>=required,'The invoice omits the imported model requirement');
        assert(r.authority_bn>0&&paid<=r.authority_bn+tolerance&&paid<=r.daily_limit_bn+tolerance,'Maintenance exceeds its real authority or reviewed ceiling');
        assert.equal(n.program_budget.day,r.day);assert.equal(n.program_budget.settled_day,r.day,'The invoice must have reached ordinary fiscal settlement');
        const posted=n.program_budget.spent_today_bn[7][2]+n.program_budget.prepaid_used_today_bn[7][2];
        assert(posted+tolerance>=paid,'The maintenance receipt is absent from actual Defense expenditure');
        assert.deepEqual(contract(observed,purchased.id),delivered,'Maintenance changed the completed purchase ownership');
        e.maintenance.receipt=copy(r);e.maintenance.imported_model_requirement_bn=required;
        e.maintenance.imported_model_support_fraction=r.custom_paid_bn/r.custom_required_bn;e.maintenance.settled_defense_payment_bn=posted;
        e.maintenance.note='The dated custom-fleet invoice includes the imported model and uses existing Defense maintenance authority. Its common support fraction is derived from that aggregate receipt; no separate model payment or full coverage is invented.';
        finalWorld=observed;break;
      }
      e.maintenance.latest_observation={date:advanced.date,plan:copy(plan||null),defense_budget:copy(n.program_budget)};
      assert(i<6,'The native recommended maintenance plan did not produce a positive settled custom-fleet payment within seven real days: '+JSON.stringify(e.maintenance.latest_observation));
    }
    assert(e.maintenance.receipt,'No paid maintenance receipt');
    await market(page);await page.locator('[data-equipment-tab="service"]').first().click();await page.waitForFunction(()=>equipmentCurrent()&&EQUIP.tab==='service');
    write('paid-maintenance.json',{service:await board(page,url),evidence:e.maintenance});
    for(const size of sizes){await page.setViewportSize({width:size.width,height:size.height});await noOverflow(page,'#equipmentRoot');await shot('paid-maintenance-'+size.name,'section[aria-label="Maintenance and readiness"]');await shot('maintained-import-'+size.name,lot);}
    await page.setViewportSize({width:1440,height:1000});e.complete=await saveLoad(page,url,run,'s08-import-complete');
    await page.reload();await page.locator('#continueBtn').click();await page.locator('#app').waitFor({state:'visible'});await idle(page);
    assert.deepEqual(await archive(page,url,run,'s08-continue'),finalWorld,'Continue changed the completed native campaign');
    assert.equal(e.commands.length,2,'The journey must issue one purchase and one separate maintenance approval');
    assert.deepEqual(e.commands.flatMap(request=>request.commands).map(command=>command.kind),['company_import_purchase','equipment_maintenance'],'The imported lot must be purchased exactly once; maintenance is a separate reviewed command');assert.deepEqual(e.errors,[]);
    assert.equal(hash(fs.readFileSync(input)),inputHash,'The original input archive changed');if(preparation)assert.equal(hash(fs.readFileSync(preparation)),preparationHash,'The original preparation archive changed');e.passed=true;e.finished_utc=new Date().toISOString();write('result.json',e);
    console.log('S08 supplier journey passed: '+path.join(evidenceDir,'result.json'));
  }catch(error){e.failed_stage=stage;e.failure=error.stack||String(error);write('result.json',e);if(page)try{await page.screenshot({path:path.join(evidenceDir,'failure.png')});fs.writeFileSync(path.join(evidenceDir,'failure.html'),await page.content());}catch(_){}throw error;
  }finally{if(browser)await browser.close();if(server)server.kill();if(log)log.end();}
}
main().catch(error=>{console.error(error);process.exitCode=1;});
