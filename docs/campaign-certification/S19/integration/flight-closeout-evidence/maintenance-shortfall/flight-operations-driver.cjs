'use strict';
const assert=require('node:assert/strict');

// Continues a real delivered-aircraft campaign. Every mutation is a visible
// review/confirmation or the caller's ordinary +1 DAY/save controls. GETs and
// page evaluation only inspect native records and the accepted rendered model.
module.exports=async function flightOperations(ctx){
  const {page,tap,state,read,shot,evidence,equipment,field,confirm,save,day,close,observe}=ctx;
  const proof=evidence.flight_operations={passed:false,stages:[],commands:[],mission_reviews:[],readings:[]};
  const clone=x=>JSON.parse(JSON.stringify(x));
  const metric=(row,label)=>row?.metrics?.find(m=>m.label===label)?.value;
  const idle=()=>page.waitForFunction(()=>!SESSION.busy&&!advancing&&!pendingAdvance&&!COMMAND_CHANNEL.busy&&!COMMAND_CHANNEL.pending&&!EQUIP.busy&&!CAB.busy);
  const quote=async()=>{await page.waitForFunction(()=>equipmentReviewQuoteCurrent());return page.evaluate(()=>JSON.parse(JSON.stringify(EQUIP.review.quote)));};
  async function dismiss(){if(await page.locator('[data-equipment-dismiss]').isVisible())await tap('[data-equipment-dismiss]');}
  async function closeAll(){
    await close();
    if(await page.locator('#sheet').isVisible())await tap('#sheet > .head .close');
    if(await page.locator('#intelDrawer').getAttribute('aria-hidden')==='false')await tap('#intelDrawer [data-close-drawers]');
  }
  async function airPage(name='command'){
    if(await page.locator('#sheet').isVisible())await tap('#sheet > .head .close');
    await equipment('flight');await dismiss();
    const selector=`[data-equipment-flight-page="${name}"]`;
    if(await page.locator(selector).getAttribute('aria-pressed')!=='true')await tap(selector);
  }
  async function openAction(predicate,description){
    await dismiss();
    const options=await page.locator('[data-equipment-action]').evaluateAll(nodes=>nodes.filter(e=>!e.disabled&&e.getClientRects().length).map(e=>({path:e.dataset.equipmentAction,scope:e.dataset.equipmentScope,action:equipmentActionAt(e.dataset.equipmentAction,e.dataset.equipmentScope)})));
    const found=options.find(r=>predicate(r.action?.command,r.action));
    assert(found,'No visible '+description+': '+JSON.stringify(options));
    await tap(`[data-equipment-action=${JSON.stringify(found.path)}][data-equipment-scope=${JSON.stringify(found.scope)}]`);
    return {action:found.action,quote:await quote()};
  }
  async function issue(kind){const q=await quote();assert(q.valid,JSON.stringify(q));const c=await confirm(kind);proof.commands.push(c);return c;}
  async function visibleCommand(selector,kind){
    const waiting=page.waitForResponse(r=>new URL(r.url()).pathname==='/api/command'&&r.request().method()==='POST');
    await tap(selector);const response=await waiting;assert(response.ok());const body=await response.json();assert.deepEqual(body.errors||[],[]);assert(!body.command_pending);await idle();
    const command=response.request().postDataJSON().commands.find(c=>c.kind===kind);assert(command,'Expected '+kind);proof.commands.push(command);return command;
  }
  async function snapshot(label){const row=await observe(label);proof.stages.push({label,date:row.state.date});return row;}
  const starting=await snapshot('flight-operations-start');
  const aircraft=starting.equipment.flight.aircraft.find(r=>r.name==='S19 Light Attack'&&Number(metric(r,'Delivered aircraft'))>=1);
  assert(aircraft,'The operations route requires an actually delivered S19 Light Attack aircraft');
  proof.revision=aircraft.id;
  const delivery=starting.guidance.outcomes.procurement.deliveries.find(d=>d.revision===aircraft.id&&d.settled_day!==null&&d.delivered_day!==null);
  assert(delivery,'Aircraft purchase must be paid and delivered');proof.delivery=delivery;
  let squadron=starting.equipment.flight.squadrons.find(s=>metric(s,'Exact revision')===aircraft.id&&Number(metric(s,'Assigned aircraft'))>0);
  if(!squadron){
    await airPage('aircraft');
    await openAction(c=>c?.kind==='air_squadron'&&c.order?.Create?.revision===aircraft.id,'Form squadron for delivered revision');
    await field('name','S19 First Flight');await field('quantity',1);await issue('air_squadron');
    squadron=(await read('equipment')).flight.squadrons.find(s=>metric(s,'Exact revision')===aircraft.id&&Number(metric(s,'Assigned aircraft'))>0);
  }
  assert(squadron);proof.squadron=squadron.id;
  const base=starting.equipment.flight.bases.find(b=>b.id==='FRA_le-de-france'&&Number(metric(b,'Aircraft spaces'))>=1&&b.status==='Available');
  assert(base,'The recorded Paris foundation must still be completed and available');
  await airPage();
  if(metric(squadron,'Current base')!==base.name){
    await openAction(c=>c?.kind==='air_base'&&c.order?.action==='rebase'&&c.order?.squadron===squadron.id,'Move to completed airbase');
    await field('base',base.id);await issue('air_base');
  }
  await airPage();
  const support=await openAction(c=>c?.kind==='equipment_air_support','Routine support');
  // Retain the simulation's computed whole-fleet requirement and add a modest
  // store allowance. The cap grants no ministry authority or finished stores.
  const nativeCap=Number(support.action.command.daily_budget_mn);
  assert(Number.isFinite(nativeCap)&&nativeCap>0);
  await field('daily_budget_mn',Math.max(1,Math.ceil(nativeCap+1)));await field('target_days',30);await field('automatic','true');
  proof.support_review=await quote();await issue('equipment_air_support');
  for(let i=0;i<35;i++){
    await closeAll();await day();const g=await read('guidance');const sq=g.outcomes.aviation.squadrons.find(s=>s.id===squadron.id);
    if(sq?.ready&&sq.assigned>0&&sq.blocker===null){proof.ready_native=clone(sq);break;}
  }
  assert(proof.ready_native,'Squadron never completed its actual base transfer/service');
  await save('s19-squadron-ready');await snapshot('flight-squadron-ready');

  // The family comes from the certified model's native compatibility record,
  // not from a hard-coded assumption about the designer's default stores.
  let board=await read('equipment');
  const familyRow=board.ammunition.families.find(r=>r.id.startsWith('air_bomb_')&&String(r.detail).includes(aircraft.name));
  assert(familyRow,'Native ammunition board must name this aircraft and its exact store family');
  const family=familyRow.id;proof.store_family=family;
  const stores=b=>Number(String(metric(b.ammunition.families.find(r=>r.id===family),'Mission stores on hand')??'0').replace(/,/g,''));
  if(!(stores(board)>0)){
    await equipment('ammunition');
    const supplyActions=board.ammunition.supplier_market?.offers||[];
    const supplier=supplyActions.find(o=>o.ammo_family===family&&o.actions?.some(a=>a.command?.kind==='company_ammo_supply'));
    if(supplier){
      const opened=await openAction(c=>c?.kind==='company_ammo_supply'&&c.family===family,'Authorize compatible company ammunition');
      await field('stock_target',12);proof.ammunition_supply_review=await quote();
      assert(proof.ammunition_supply_review.valid,JSON.stringify(proof.ammunition_supply_review));
      const company=opened.action.command.company;await issue('company_ammo_supply');proof.ammunition_company=company;
    }else{
      const existing=supplyActions.some(o=>o.ammo_family===family&&o.product!=null);
      if(!existing){
        await openAction(c=>c?.kind==='equipment_ammo_order'&&c.family===family,'Fund a finite compatible ammunition batch');
        await field('quantity',12);await field('daily_budget_mn',1);proof.ammunition_batch_review=await quote();await issue('equipment_ammo_order');
      }
    }
    await save('s19-flight-stores-ordered');
    for(let i=0;i<160;i++){
      await closeAll();await day();board=await read('equipment');
      if(stores(board)>0)break;
      if(i%10===0)console.log('FLIGHT_STORES',JSON.stringify({days:i+1,date:(await state()).date,ammunition:board.ammunition,company_products:board.companies.products}));
      const offer=(board.ammunition.supplier_market?.offers||[]).find(o=>o.ammo_family===family&&o.actions?.some(a=>a.command?.kind==='company_ammo_purchase'&&a.enabled!==false));
      // Normally the explicitly enabled routine policy purchases the finite
      // finished stock. A visible manual purchase handles available stock when
      // its automatic cap/target has not selected it yet.
      if(offer&&!proof.ammunition_purchase){
        const order=offer.actions.find(a=>a.command?.kind==='company_ammo_purchase'&&a.enabled!==false).command;
        await equipment('ammunition');await openAction(c=>c?.kind==='company_ammo_purchase'&&c.company===order.company&&c.product===order.product,'Purchase compatible finished mission stores');
        await field('quantity',1);const q=await quote();
        if(q.valid)proof.ammunition_purchase=await issue('company_ammo_purchase');else{proof.ammunition_purchase_refusal=q;await dismiss();}
      }
    }
    assert(stores(board)>0,'No actual compatible mission stores delivered after 160 ordinary days; inspect retained native ammunition/supplier reasons');
  }
  proof.stores_before_mission=stores(board);await save('s19-flight-stores-ready');await snapshot('flight-stores-ready');

  // Prefer an already eligible campaign. This saved peaceful France has none;
  // create its nearby conflict through the ordinary national dossier and its
  // explicit declaration confirmation, never through a synthesized POST.
  let campaign=await state();
  let conflict=campaign.wars.find(w=>w.posture?.some(p=>p.id===campaign.player&&p.rung>=6));
  if(!conflict){
    const target=campaign.nations.find(n=>n.id==='Belgium');assert(target,'Nearby Belgium must exist before declaring this bounded review conflict');
    await closeAll();await tap('.decision-nav button:has-text("Find")');await page.locator('#worldFindInput').fill(target.name);
    evidence.actions.push({fill:'#worldFindInput',value:target.name});await tap(`[data-find-kind="nation"][data-find-id=${JSON.stringify(target.id)}]`);
    await tap('#sheet button:has-text("Declare war")');await page.locator('#campaignConfirmAccept').waitFor({state:'visible'});
    proof.war_confirmation=await page.locator('#campaignConfirmMessage').innerText();assert(proof.war_confirmation.includes(target.name));
    await visibleCommand('#campaignConfirmAccept','war');campaign=await state();
    conflict=campaign.wars.find(w=>w.posture?.some(p=>p.id===campaign.player)&&w.posture.some(p=>p.id===target.id));assert(conflict);
  }
  proof.conflict=clone(conflict);
  await closeAll();await tap('[data-drawer="intelDrawer"]');
  const form=`#warsCard form[data-force-id="${conflict.id}"]`;
  await page.locator(form+' select[name="share"]').selectOption('2500');evidence.actions.push({select:form+' select[name="share"]',value:'2500'});
  await visibleCommand(form+' button[type="submit"]','force_allocation');
  await closeAll();await airPage();
  const missionAction=await openAction(c=>c?.kind==='air_mission'&&c.order?.action==='queue'&&c.order?.squadron===squadron.id&&c.order?.kind==='strike_target','Strike target');
  await field('conflict',conflict.id);
  const targetOptions=missionAction.action.inputs.find(f=>f.key==='target')?.options||[];
  let valid=null;
  for(const option of targetOptions.filter(o=>o.enabled!==false)){
    const q=await field('target',option.value);proof.mission_reviews.push({target:option.value,label:option.label,quote:q});
    if(q.valid){valid=q;break;}
  }
  assert(valid,'No legitimate in-range supported Strike target: '+JSON.stringify(proof.mission_reviews));
  await shot('flight-mission-review');proof.mission_order=await issue('air_mission');
  await save('s19-flight-mission-queued');
  const existingIds=new Set((await read('guidance')).outcomes.aviation.missions.filter(m=>m.status==='flown').map(m=>m.id));
  for(let i=0;i<8;i++){
    await closeAll();await day();const g=await read('guidance');
    const result=g.outcomes.aviation.missions.find(m=>m.status==='flown'&&m.aircraft>0&&!existingIds.has(m.id));
    if(result){proof.flown_native=result;break;}
    const failed=g.outcomes.aviation.missions.find(m=>m.status==='blocked'&&!existingIds.has(m.id));
    assert(!failed,'Mission was held before launch: '+JSON.stringify(await read('equipment')));
  }
  assert(proof.flown_native,'No actual dated flown mission');
  board=await read('equipment');proof.flown_report=board.flight.missions.results.find(r=>r.id===proof.flown_native.id);
  assert(proof.flown_report&&proof.flown_report.status==='Flown');
  assert(Number(String(metric(proof.flown_report,'Compatible stores used')).replace(/,/g,''))>0,'Flight must consume actual compatible stores');
  assert(Number(metric(proof.flown_report,'Aircraft launched'))>0);assert(proof.flown_report.receipt_label);
  await airPage('reports');await shot('flight-real-mission-result');await snapshot('flight-mission-flown');
  // Ready is a current physical state, so let the actual funded post-flight
  // service finish before checking retention. No completed-history claim is
  // inferred from the immediately unready aircraft after landing.
  for(let i=0;i<12;i++){
    const g=await read('guidance'),sq=g.outcomes.aviation.squadrons.find(s=>s.id===squadron.id);
    if(sq?.ready&&sq.assigned>0&&sq.blocker===null){proof.recovered_native=sq;break;}
    await closeAll();await day();
  }
  assert(proof.recovered_native,'Post-flight paid service did not restore squadron readiness');

  async function verify(label){
    await closeAll();await page.keyboard.press('F1');evidence.actions.push({key:'F1',label});
    await page.waitForFunction(()=>GUIDANCE.session.state().status==='ready'&&!GUIDANCE.session.state().routeStale);
    const native=await read('guidance'),now=await state(),route=await page.evaluate(()=>JSON.parse(JSON.stringify(GUIDANCE.session.state().route)));
    assert.equal(route.status,'ready');assert.equal(route.as_of.date,now.date);
    const recomputed=await page.evaluate(data=>{const saved=JSON.parse(localStorage.getItem('spheres.guidance.receipts.v1')||'null');return JSON.parse(JSON.stringify(AdvisorModel.recognize(data,saved||{saves:[],loads:[]})));},native);
    assert.deepEqual(route,recomputed);
    const air=route.steps.find(s=>s.id==='air_force');assert.equal(air.status,'achieved');
    for(const id of ['base_funded','base_completed','squadron_formed','squadron_ready','mission_flown'])assert.equal(air.milestones.find(m=>m.id===id)?.status,'done',id);
    const flight=air.milestones.find(m=>m.id==='mission_flown');assert(flight.date);
    const card=page.locator('[data-guidance-route-card="air_force"]');assert(await card.evaluate(e=>e.classList.contains('guidance-route-step--achieved')));
    assert((await card.locator('time').evaluateAll(nodes=>nodes.map(n=>n.getAttribute('datetime')))).includes(flight.date));
    const receipt=native.outcomes.aviation.missions.find(m=>m.id===proof.flown_native.id);assert.deepEqual(receipt,proof.flown_native);
    await card.locator('h4').click();await shot(label);
    const reading={label,date:now.date,session_id:now.session_id,aviation:native.outcomes.aviation,air,procurement:route.steps.find(s=>s.id==='procurement'),construction:route.steps.find(s=>s.id==='construction')};proof.readings.push(reading);return reading;
  }
  const before=await verify('flight-guidance-before-save');
  await closeAll();const slot='s19-flight-qualified';await save(slot);
  async function loadSaved(){
    await closeAll();if(await page.locator('.arc-time-menu').getAttribute('open')===null)await tap('.arc-time-menu > summary');await tap('#campaignsBtn');await tap('#openSavesBtn');
    await page.locator(`#saveSlots option[value="${slot}"]`).waitFor({state:'attached'});await page.locator('#saveSlots').selectOption(slot);evidence.actions.push({select:'#saveSlots',value:slot});
    const response=page.waitForResponse(r=>new URL(r.url()).pathname==='/api/load'&&r.request().method()==='POST');await tap('#loadBtn');await tap('#campaignConfirmAccept');assert((await response).ok());await idle();await page.locator('#app').waitFor({state:'visible'});
  }
  await loadSaved();const loaded=await verify('flight-guidance-after-load');assert.deepEqual(loaded.aviation,before.aviation);assert.deepEqual(loaded.air.milestones,before.air.milestones);
  assert.deepEqual(loaded.procurement.milestones,before.procurement.milestones);assert.deepEqual(loaded.construction.milestones,before.construction.milestones);
  await closeAll();await page.reload({waitUntil:'domcontentloaded'});await tap('#continueBtn');await idle();const continued=await verify('flight-guidance-after-continue');assert.deepEqual(continued.aviation,before.aviation);assert.deepEqual(continued.air.milestones,before.air.milestones);
  assert.deepEqual(continued.procurement.milestones,before.procurement.milestones);assert.deepEqual(continued.construction.milestones,before.construction.milestones);
  await page.setViewportSize({width:390,height:844});await page.locator('[data-guidance-route-card="air_force"] h4').click();await shot('flight-guidance-narrow');
  assert(await page.locator('#guidanceDialog').evaluate(e=>e.scrollWidth<=e.clientWidth+1));await page.setViewportSize({width:1440,height:1000});
  await closeAll();if(await page.locator('.arc-time-menu').getAttribute('open')===null)await tap('.arc-time-menu > summary');await tap('#campaignsBtn');await tap('#newCampaignBtn');await page.locator('#newCampaignPicker').waitFor({state:'visible'});
  await tap('#nationPick [aria-label^="France;"]');await tap('#startBtn');const newResponse=page.waitForResponse(r=>new URL(r.url()).pathname==='/api/new'&&r.request().method()==='POST');await tap('#campaignConfirmAccept');assert((await newResponse).ok());await idle();
  await page.keyboard.press('F1');await page.waitForFunction(()=>GUIDANCE.session.state().status==='ready'&&!GUIDANCE.session.state().routeStale);
  const fresh=await read('guidance'),freshModel=await page.evaluate(()=>JSON.parse(JSON.stringify(GUIDANCE.session.state().route)));
  assert.notEqual((await state()).session_id,before.session_id);assert.deepEqual(fresh.outcomes.aviation.missions,[]);assert.deepEqual(fresh.outcomes.aviation.squadrons,[]);
  for(const step of freshModel.steps){assert.notEqual(step.status,'achieved',step.id+' cannot inherit prior campaign outcomes');assert(step.milestones.every(m=>m.status!=='done'),step.id);}
  proof.new_campaign={state:await state(),aviation:fresh.outcomes.aviation,route:freshModel};await shot('flight-new-campaign-isolation');
  await loadSaved();const restored=await verify('flight-qualified-restored');assert.deepEqual(restored.aviation,before.aviation);
  proof.passed=true;console.log('PASS: real delivered aircraft, funded support and stores, ready squadron, flown mission, retained guidance and new-campaign isolation');
};
