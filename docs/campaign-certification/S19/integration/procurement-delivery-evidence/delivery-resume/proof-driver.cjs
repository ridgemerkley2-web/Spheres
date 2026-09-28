'use strict';
const assert=require('node:assert/strict');
// This proof consumes a real delivered campaign. It cannot create purchases or
// settlement receipts; the caller records the source save hash and native build.
module.exports=async function supplierDeliveryProof({page,tap,read,state,shot,evidence}){
  const proof=evidence.delivery_proof={passed:false,readings:[]};
  async function close(){
    for(const [panel,button] of [['#guidanceDialog','#guidanceDialog [data-guidance-close]'],['#equipmentRoom','[data-equipment-close]'],['#productionPanel','#productionClose'],['#cabinetDrawer','#cabinetDrawer [data-close-drawers]']])
      if(await page.locator(panel).isVisible())await tap(button);
  }
  async function verify(label){
    await page.locator('#app').waitFor({state:'visible'});
    await page.waitForFunction(()=>!SESSION.busy&&!advancing&&!pendingAdvance&&!COMMAND_CHANNEL.busy&&!COMMAND_CHANNEL.pending);
    await close();await page.keyboard.press('F1');evidence.actions.push({key:'F1',label});
    await page.waitForFunction(()=>GUIDANCE.session.state().status==='ready'&&!GUIDANCE.session.state().routeStale);
    const native=await read('guidance'),now=await state();
    const model=await page.evaluate(()=>JSON.parse(JSON.stringify(GUIDANCE.session.state().route)));
    assert.equal(model.status,'ready');assert.equal(model.as_of.date,now.date);
    const recomputed=await page.evaluate(data=>{
      const stored=JSON.parse(localStorage.getItem('spheres.guidance.receipts.v1')||'null');
      return JSON.parse(JSON.stringify(AdvisorModel.recognize(data,stored||{saves:[],loads:[]})));
    },native);
    assert.deepEqual(recomputed,model);
    const procurement=model.steps.find(s=>s.id==='procurement');assert.equal(procurement.status,'achieved');
    for(const id of ['manufacturer','certified_product','purchase_placed','purchase_paid','equipment_delivered']){
      const m=procurement.milestones.find(m=>m.id===id);assert.equal(m?.status,'done',id);
      // Manufacturer and certification are current native tallies, without
      // historical timestamps. Only actual purchase/settlement/delivery receipts
      // can establish these dates; never invent dates for the tally milestones.
      if(['purchase_placed','purchase_paid','equipment_delivered'].includes(id))assert(m.date,id+' must be dated');
    }
    const rendered=page.locator('[data-guidance-route-card="procurement"]');
    assert(await rendered.evaluate(el=>el.classList.contains('guidance-route-step--achieved')));
    const dates=await rendered.locator('time').evaluateAll(nodes=>nodes.map(n=>n.getAttribute('datetime')));
    for(const m of procurement.milestones.filter(m=>m.status==='done'&&m.date))assert(dates.includes(m.date));
    const delivery=native.outcomes.procurement.deliveries.find(d=>d.revision==='France-design-1'&&d.quantity===1&&d.settled_day!==null&&d.delivered_day!==null);
    assert(delivery,'Native paid and delivered tank receipt');
    assert(delivery.settled_day>=delivery.purchased_day);assert(delivery.delivered_day>=delivery.settled_day);
    const construction=model.steps.find(s=>s.id==='construction');
    for(const id of ['work_paid','project_completed','site_producing'])assert.equal(construction.milestones.find(m=>m.id===id)?.status,'done',id);
    await rendered.locator('h4').click();await shot(label);
    const reading={label,date:now.date,session_id:now.session_id,delivery,procurement,construction};proof.readings.push(reading);return reading;
  }
  const before=await verify('delivery-before-save');
  await close();if(await page.locator('.arc-time-menu').getAttribute('open')===null)await tap('.arc-time-menu > summary');
  await tap('#campaignsBtn');await tap('#openSavesBtn');
  const slot='s19-delivery-proof';await page.locator('#saveName').fill(slot);evidence.actions.push({fill:'#saveName',value:slot});
  const saving=page.waitForResponse(r=>new URL(r.url()).pathname==='/api/save'&&r.request().method()==='POST');
  await tap('#saveNamedBtn');assert((await saving).ok());
  await page.locator(`#saveSlots option[value="${slot}"]`).waitFor({state:'attached'});await page.locator('#saveSlots').selectOption(slot);
  evidence.actions.push({select:'#saveSlots',value:slot});await tap('#loadBtn');await tap('#campaignConfirmAccept');
  await page.waitForFunction(()=>!SESSION.busy&&S?.player);
  const loaded=await verify('delivery-after-load');assert.deepEqual(loaded.delivery,before.delivery);assert.deepEqual(loaded.procurement.milestones,before.procurement.milestones);
  await close();await page.reload();await page.locator('#continueBtn').waitFor({state:'visible'});await tap('#continueBtn');
  const continued=await verify('delivery-after-continue');assert.deepEqual(continued.delivery,before.delivery);assert.deepEqual(continued.procurement.milestones,before.procurement.milestones);
  await page.setViewportSize({width:390,height:844});await page.locator('[data-guidance-route-card="procurement"] h4').click();await shot('delivery-narrow');
  assert(await page.locator('#guidanceDialog').evaluate(el=>el.scrollWidth<=el.clientWidth+1));
  proof.passed=true;console.log('PASS: native procurement and construction milestones survive named save/load and Continue');
};
