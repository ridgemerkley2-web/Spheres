'use strict';
const assert=require('node:assert/strict');

// Browser-only actions with native GET observations. No synthetic campaign,
// command injection, save editing or outcome substitution.
module.exports=async function laterProcurement(h){
  const {page,tap,type,press,command,reviewQuote,openGuide,readRoute,follow,
    stepDay,nativeGuidance,observe,shot,progress,evidence,check}=h;
  const q=JSON.stringify;
  async function companies(){
    await openGuide('later-procurement-navigation');
    const route=await readRoute('later-procurement-navigation');
    assert(route.step('procurement').obstacle,'An unfinished procurement route must retain a navigation action');
    const nav=await follow('procurement');
    await page.waitForFunction(()=>equipmentCurrent()&&EQUIP.tab==='companies');nav.done();
    const administration=page.locator('[data-equipment-detail="company-administration"]');
    if(await administration.count() && await administration.getAttribute('open')===null)
      await tap('[data-equipment-detail="company-administration"] > summary');
  }
  async function action(kind){
    const options=await page.locator('[data-equipment-action]').evaluateAll(nodes=>nodes.filter(e=>!e.disabled).map(e=>({
      path:e.dataset.equipmentAction,scope:e.dataset.equipmentScope,
      command:equipmentActionAt(e.dataset.equipmentAction,e.dataset.equipmentScope)?.command
    })));
    const row=options.find(r=>r.command?.kind===kind);
    assert(row,'No visible enabled '+kind+' offer: '+JSON.stringify(options));
    await tap(`[data-equipment-action=${q(row.path)}][data-equipment-scope=${q(row.scope)}]`);
    return reviewQuote();
  }
  async function confirm(kind){
    const quote=await reviewQuote();
    assert(quote.valid,kind+' blocked: '+JSON.stringify(quote));
    const index=quote.actions.findIndex(a=>a.command?.kind===kind&&a.enabled!==false);
    assert(index>=0,'No reviewed confirmation for '+kind);
    const payload=await command(()=>tap(`[data-equipment-intent="${index}"]`),kind);
    evidence.later_commands.push(kind);
    return payload.commands.find(c=>c.kind===kind);
  }
  evidence.later_commands=[];
  await companies();
  if(await page.locator('[data-equipment-action="companies.overview.actions.0"]').isDisabled()){
    const board=await observe('/api/equipment');
    const actions=board.companies.overview.actions;
    const build=actions.findIndex(a=>a.navigate?.action==='construction'&&a.navigate.kind==='arms_plant');
    assert(build>=0,'A missing plant must provide construction navigation');
    await tap(`[data-equipment-action="companies.overview.actions.${build}"]`);
    await page.locator('[data-prod-province]').first().waitFor({state:'visible'});
    const district=await page.locator('[data-prod-province]').first().getAttribute('data-prod-province');
    await tap(`[data-prod-province=${q(district)}]`);
    await page.waitForFunction(()=>constructionPreviewCurrent());
    const preview=await page.evaluate(()=>JSON.parse(JSON.stringify(PROD.preview)));
    assert(preview.can_start,'Arms plant must be affordable through normal construction: '+JSON.stringify(preview));
    await command(()=>tap('[data-construction-confirm]'),'start_project');
    evidence.later_commands.push('start_project');
    let available=false;
    for(let day=0;day<900;day++){
      await stepDay();
      const state=await observe('/api/state');
      const reading=await observe('/api/equipment?session_id='+encodeURIComponent(state.session_id));
      const establish=reading.companies.overview.actions.find(a=>a.command?.kind==='company_establish');
      if(day%30===0)progress('later-manufacturer-plant',{days:day+1,date:state.date,offer:establish});
      if(establish&&establish.enabled!==false){available=true;evidence.manufacturer_plant={district,date:state.date,days:day+1};break;}
    }
    assert(available,'No free completed Arms Plant after 900 normal days');
    await companies();
  }
  const establishQuote=await action('company_establish');
  assert(establishQuote.valid,'Establishment requires an available real plant and funding: '+JSON.stringify(establishQuote));
  const established=await confirm('company_establish');
  await companies();
  await tap('[role="tab"][data-equipment-tab="library"]');
  await page.waitForFunction(()=>equipmentCurrent()&&EQUIP.tab==='library');
  await action('company_develop');
  const developed=await confirm('company_develop');
  let purchase=null,product=null,stockReading=null;
  for(let day=0;day<540;day++){
    await stepDay();
    const state=await observe('/api/state');
    const board=await observe('/api/equipment?session_id='+encodeURIComponent(state.session_id));
    const products=board.companies?.products||[];
    product=products.find(p=>(p.actions||[]).some(a=>a.command?.kind==='company_purchase'&&a.command.company===developed.company));
    if(day%30===0)progress('later-procurement',{days:day+1,date:state.date,products});
    if(product){stockReading={day:day+1,date:state.date,product};break;}
  }
  assert(stockReading,'No company stock after 540 ordinary days; inspect development and finance blockers in progress evidence');
  await companies();
  await action('company_purchase');
  // Buy one real unit, explicitly chosen through the quoted quantity control.
  await type('[data-equipment-order-input="quantity"]','1');
  await press('Tab');
  const purchasedDay=(await nativeGuidance()).native.outcomes.as_of_day;
  purchase=await confirm('company_purchase');
  assert.equal(purchase.quantity,1);
  assert.equal(purchase.company,product.company);
  assert.equal(purchase.product,product.product);
  let delivered=null;
  for(let day=0;day<120;day++){
    const reading=await nativeGuidance();
    const receipt=reading.native.outcomes.procurement.deliveries.find(d=>d.company===product.supplier_name&&d.revision===product.source_revision&&d.purchased_day===purchasedDay&&d.quantity===purchase.quantity&&d.settled_day!==null&&d.delivered_day!==null);
    if(receipt){delivered={date:reading.state.date,receipt};break;}
    if(day%10===0)progress('later-delivery',{days:day,date:reading.state.date,procurement:reading.native.outcomes.procurement});
    await stepDay();
  }
  assert(delivered,'Purchased equipment did not deliver within 120 days');
  await openGuide('later-procurement-delivered');
  const route=await readRoute('later-procurement-delivered');
  const procurement=route.step('procurement');
  assert.equal(procurement.status,'achieved');
  for(const id of ['manufacturer','certified_product','purchase_placed','purchase_paid','equipment_delivered'])
    assert.equal(procurement.milestones.find(m=>m.id===id)?.status,'done',id+' must derive from real campaign receipts');
  // Require actual paid and delivered receipts, not just a company or stock.
  assert(delivered.receipt.settled_day>=delivered.receipt.purchased_day);
  assert(delivered.receipt.delivered_day>=delivered.receipt.settled_day);
  await shot('route-later-procurement-desktop',page.locator('#guidanceDialog [data-guidance-route-card="procurement"]'));
  evidence.later_procurement={established,developed,stockReading,purchase,delivered,route:procurement};
  check('Ordinary company establishment, funded development, stock purchase and actual payment/delivery reached guidance; save/load stages must retain the milestones');
};
