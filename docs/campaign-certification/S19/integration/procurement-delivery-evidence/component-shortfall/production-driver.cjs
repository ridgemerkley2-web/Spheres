'use strict';
const assert=require('node:assert/strict');
// All campaign actions use visible controls; reads observe native outcomes.
module.exports=async function supplierProduction(h){
  const {page,tap,state,read,shot,companies,evidence}=h;
  const p=evidence.production={passed:false,checkpoints:[],annual_budgets:[],commands:[]};
  const idle=()=>page.waitForFunction(()=>!advancing&&!pendingAdvance&&!SESSION.busy&&!CAB.busy&&!PROD.busy&&!COMMAND_CHANNEL.busy&&!COMMAND_CHANNEL.pending&&!EQUIP.busy);
  async function close(){
    for(const [panel,button] of [['#guidanceDialog','#guidanceDialog [data-guidance-close]'],['#equipmentRoom','[data-equipment-close]'],['#productionPanel','#productionClose'],['#cabinetDrawer','#cabinetDrawer [data-close-drawers]']])
      if(await page.locator(panel).isVisible())await tap(button);
  }
  async function guide(){
    await close();evidence.actions.push({key:'F1'});await page.keyboard.press('F1');
    await page.waitForFunction(()=>GUIDANCE.session.state().status==='ready'&&!GUIDANCE.session.state().routeStale);
  }
  async function command(selector,kind){
    const wait=page.waitForResponse(r=>new URL(r.url()).pathname==='/api/command'&&r.request().method()==='POST');
    await tap(selector);const response=await wait;assert(response.ok());const body=await response.json();
    assert.deepEqual(body.errors||[],[]);assert(!body.command_pending);await idle();
    const sent=response.request().postDataJSON().commands.find(c=>c.kind===kind);assert(sent,'Expected '+kind);p.commands.push(sent);return sent;
  }
  async function advance(selector){
    const wait=page.waitForResponse(r=>new URL(r.url()).pathname==='/api/advance'&&r.request().method()==='POST');
    await tap(selector);const response=await wait;assert(response.ok());assert.equal(response.request().postDataJSON().days,1);
    const body=await response.json();assert.deepEqual(body.errors||[],[]);assert(!body.advance_pending);await idle();return state();
  }
  let fundedYear=(await state()).year;
  async function day(){
    await close();let now=await advance('#stepBtn');
    if(now.year!==fundedYear){
      await guide();await tap('[data-guidance-route-step="finances"]');await page.locator('#cabinet-budget').waitFor();
      now=await advance('#cabinetEnact');fundedYear=now.year;p.annual_budgets.push({year:now.year,date:now.date});
    }
    return now;
  }
  async function save(slot){
    await close();if(await page.locator('.arc-time-menu').getAttribute('open')===null)await tap('.arc-time-menu > summary');
    await tap('#campaignsBtn');await tap('#openSavesBtn');
    evidence.actions.push({fill:'#saveName',value:slot});await page.locator('#saveName').fill(slot);
    const wait=page.waitForResponse(r=>new URL(r.url()).pathname==='/api/save'&&r.request().method()==='POST');
    await tap('#saveNamedBtn');assert((await wait).ok());await page.locator(`#saveSlots option[value="${slot}"]`).waitFor({state:'attached'});
    await tap('#savedCampaigns [data-menu-back]');await tap('#continueBtn');await idle();
    p.checkpoints.push({slot,date:(await state()).date});console.log('CHECKPOINT',JSON.stringify(p.checkpoints.at(-1)));
  }
  assert(evidence.construction_preview.can_start,'Reviewed component plant must be legal and affordable');
  p.plant_order=await command('[data-construction-confirm]','start_project');
  await save('s19-component-plant-ordered');
  let stock=null,output=null,completedAt=null;
  for(let i=0;i<900;i++){
    const now=await day();
    if(i%15!==0 && i<660)continue;
    const industry=await read('industry');
    const site=industry.sites.find(s=>s.kind==='advanced_industry'&&s.district===evidence.construction_preview.district);
    if(site && !completedAt){completedAt=i;p.completed={date:now.date,site};await save('s19-component-plant-completed');}
    if(site?.output_daily>0 && !output){output={date:now.date,site};p.output=output;await save('s19-components-produced');}
    const board=await read('equipment');const product=board.companies.products.find(x=>x.company===1&&x.product===3);
    if(i%30===0||site)console.log('PRODUCTION',JSON.stringify({days:i+1,date:now.date,site:site?{status:site.status,output:site.output_daily,reason:site.reason}:null,queue:industry.queue,goods:industry.goods,product:product?{status:product.status,detail:product.detail,stock:product.availability.ready_stock}:null}));
    if(i>0&&i%120===0)await save('s19-component-progress-'+i);
    if(product?.availability.ready_stock>0){stock={date:now.date,product};break;}
    if(completedAt!==null && !output && i-completedAt>=60){p.operation_blocker={date:now.date,site,industry};await save('s19-component-operation-blocked');break;}
  }
  assert(output,'The component plant has not produced: inspect its recorded operating blocker');
  assert(stock,'No company stock within the ordinary 900-day bound');p.stock=stock;
  await save('s19-tank-stock-ready');await companies();
  const board=await read('equipment'),index=board.companies.products.findIndex(x=>x.company===1&&x.product===3);
  const buy=board.companies.products[index].actions.findIndex(a=>a.command?.kind==='company_purchase');assert(buy>=0);
  await tap(`[data-equipment-action="companies.products.${index}.actions.${buy}"]`);
  await page.waitForFunction(()=>equipmentReviewQuoteCurrent());
  evidence.actions.push({fill:'[data-equipment-order-input="quantity"]',value:'1'});await page.locator('[data-equipment-order-input="quantity"]').fill('1');await page.keyboard.press('Tab');
  await page.waitForFunction(()=>equipmentReviewQuoteCurrent());const quote=await page.evaluate(()=>JSON.parse(JSON.stringify(EQUIP.review.quote)));
  assert(quote.valid,JSON.stringify(quote));p.purchase_quote=quote;
  const confirm=quote.actions.findIndex(a=>a.enabled!==false&&a.command?.kind==='company_purchase');assert(confirm>=0);
  p.purchase=await command(`[data-equipment-intent="${confirm}"]`,'company_purchase');assert.equal(p.purchase.quantity,1);
  for(let i=0;i<90;i++){
    const reading=await read('guidance');
    const delivered=reading.outcomes.procurement.deliveries.find(d=>d.revision===stock.product.source_revision&&d.quantity===1&&d.settled_day!==null&&d.delivered_day!==null);
    if(delivered){p.delivery=delivered;break;}await day();
  }
  assert(p.delivery,'No real paid delivery after 90 days');assert(p.delivery.delivered_day>=p.delivery.settled_day);
  await guide();p.guidance_before=await page.evaluate(()=>JSON.parse(JSON.stringify(GUIDANCE.session.state().routeChecked)));
  await shot('tank-delivery-guidance');await save('s19-tank-delivered');
  await close();if(await page.locator('.arc-time-menu').getAttribute('open')===null)await tap('.arc-time-menu > summary');
  await tap('#campaignsBtn');await tap('#openSavesBtn');await page.locator('#saveSlots').selectOption('s19-tank-delivered');
  await tap('#loadBtn');await tap('#campaignConfirmAccept');await idle();
  const loaded=await read('guidance');assert(loaded.outcomes.procurement.deliveries.some(d=>d.revision===p.delivery.revision&&d.purchased_day===p.delivery.purchased_day&&d.delivered_day===p.delivery.delivered_day&&d.settled_day===p.delivery.settled_day));
  p.loaded_outcomes=loaded.outcomes;await guide();await shot('tank-delivery-reloaded');
  p.passed=true;console.log('PASS: ordinary component production, tank stock, purchase, payment, delivery and load');
};
