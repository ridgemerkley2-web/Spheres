'use strict';
const assert=require('node:assert/strict'),fs=require('node:fs'),path=require('node:path');
// Visible controls mutate the campaign. Native GETs only observe outcomes.
module.exports=async function flightRoute({page,tap,state,read,shot,evidence}){
  const f=evidence.flight={passed:false,observations:[],commands:[],checkpoints:[]};
  const idle=()=>page.waitForFunction(()=>!SESSION.busy&&!advancing&&!pendingAdvance&&!COMMAND_CHANNEL.busy&&!COMMAND_CHANNEL.pending&&!EQUIP.busy&&!CAB.busy);
  async function close(){for(const [panel,button] of [['#guidanceDialog','#guidanceDialog [data-guidance-close]'],['#equipmentRoom','[data-equipment-close]'],['#productionPanel','#productionClose'],['#cabinetDrawer','#cabinetDrawer [data-close-drawers]']])if(await page.locator(panel).isVisible())await tap(button);}
  async function equipment(tab){
    if(!await page.locator('#equipmentRoom').isVisible()){
      await close();if(await page.locator('#intelDrawer').getAttribute('aria-hidden')!=='false')await tap('[data-drawer="intelDrawer"]');
      await tap('#warsCard [data-ground-equipment-tab="service"]');await page.waitForFunction(()=>equipmentCurrent());
    }
    await tap(`[role="tab"][data-equipment-tab="${tab}"]`);await page.waitForFunction(tab=>equipmentCurrent()&&EQUIP.tab===tab,tab);
  }
  async function observe(label){const row={label,state:await state(),equipment:await read('equipment'),guidance:await read('guidance')};f.observations.push(row);fs.writeFileSync(path.join(evidence.out,label+'.json'),JSON.stringify(row,null,2));return row;}
  async function command(selector,kind){const waiting=page.waitForResponse(r=>new URL(r.url()).pathname==='/api/command'&&r.request().method()==='POST');await tap(selector);const response=await waiting;assert(response.ok());const body=await response.json();assert.deepEqual(body.errors||[],[]);assert(!body.command_pending);await idle();const sent=response.request().postDataJSON().commands.find(c=>c.kind===kind);assert(sent);f.commands.push(sent);return sent;}
  const quote=async()=>{await page.waitForFunction(()=>equipmentReviewQuoteCurrent());return page.evaluate(()=>JSON.parse(JSON.stringify(EQUIP.review.quote)));};
  async function action(kind,match={}){
    const options=await page.locator('[data-equipment-action]').evaluateAll(nodes=>nodes.filter(e=>!e.disabled&&e.getClientRects().length).map(e=>({path:e.dataset.equipmentAction,scope:e.dataset.equipmentScope,command:equipmentActionAt(e.dataset.equipmentAction,e.dataset.equipmentScope)?.command})));
    const row=options.find(r=>r.command?.kind===kind&&Object.entries(match).every(([k,v])=>r.command[k]===v));assert(row,'No visible action '+kind+': '+JSON.stringify(options));
    await tap(`[data-equipment-action=${JSON.stringify(row.path)}][data-equipment-scope=${JSON.stringify(row.scope)}]`);return quote();
  }
  async function field(key,value){const e=page.locator(`[data-equipment-order-input="${key}"]`);if(await e.evaluate(el=>el.tagName==='SELECT'))await e.selectOption(String(value));else{await e.fill(String(value));await page.keyboard.press('Tab');}return quote();}
  async function confirm(kind){const q=await quote();assert(q.valid,JSON.stringify(q));const i=q.actions.findIndex(a=>a.enabled!==false&&a.command?.kind===kind);assert(i>=0);return command(`[data-equipment-intent="${i}"]`,kind);}
  async function save(slot){await close();if(await page.locator('.arc-time-menu').getAttribute('open')===null)await tap('.arc-time-menu > summary');await tap('#campaignsBtn');await tap('#openSavesBtn');await page.locator('#saveName').fill(slot);evidence.actions.push({fill:'#saveName',value:slot});await tap('#saveNamedBtn');await page.locator(`#saveSlots option[value="${slot}"]`).waitFor({state:'attached'});await tap('#savedCampaigns [data-menu-back]');await tap('#continueBtn');await idle();const row={slot,date:(await state()).date};f.checkpoints.push(row);console.log('CHECKPOINT',JSON.stringify(row));}
  async function step(selector){const waiting=page.waitForResponse(r=>new URL(r.url()).pathname==='/api/advance'&&r.request().method()==='POST');await tap(selector);const response=await waiting;assert(response.ok());assert.equal(response.request().postDataJSON().days,1);const body=await response.json();assert.deepEqual(body.errors||[],[]);assert(!body.advance_pending);await idle();}
  let year=(await state()).year;
  async function day(){await close();await step('#stepBtn');let now=await state();if(now.year!==year){await page.keyboard.press('F1');await page.waitForFunction(()=>GUIDANCE.session.state().status==='ready'&&!GUIDANCE.session.state().routeStale);await tap('[data-guidance-route-step="finances"]');await step('#cabinetEnact');year=now.year;}return state();}
  await idle();await observe('flight-start');
  if(process.env.SPHERES_FLIGHT_RESUME==='operations'){
    const helper=path.join(__dirname,'guidance-flight-operations.cjs'),bytes=fs.readFileSync(helper);
    evidence.flight_operations_driver_sha256=require('node:crypto').createHash('sha256').update(bytes).digest('hex');fs.writeFileSync(path.join(evidence.out,'flight-operations-driver.cjs'),bytes);
    await require(helper)({page,tap,state,read,shot,evidence,equipment,action,field,confirm,save,day,close,observe});
    assert(evidence.flight_operations.passed);f.passed=true;return;
  }
  if(!process.env.SPHERES_FLIGHT_RESUME){
  await equipment('designer');await tap('[data-equipment-family="aviation"]');await page.waitForFunction(()=>equipmentPreviewCurrent());
  await page.locator('#equipmentPlatform').selectOption('air_light_attack');await page.locator('#equipmentName').fill('S19 Light Attack');await page.keyboard.press('Tab');await page.waitForFunction(()=>equipmentPreviewCurrent());
  f.design_preview=await page.evaluate(()=>JSON.parse(JSON.stringify(EQUIP.preview)));assert(f.design_preview.valid,JSON.stringify(f.design_preview));
  await tap('[data-equipment-action][data-equipment-scope="preview"]:has-text("Save design draft")');await page.locator('#equipmentActionReviewTitle').waitFor();await command('[data-equipment-confirm]','equipment_save');
  await page.waitForFunction(()=>equipmentPreviewCurrent());const developQuote=await action('company_develop');f.development_quote=developQuote;
  const stockCapital=developQuote.costs.find(c=>c.label==='Company capital needed for first stock target')?.amount_bn;assert(stockCapital>0);
  f.development=await confirm('company_develop');
  await equipment('companies');const admin=page.locator('[data-equipment-detail="company-administration"]');if(await admin.getAttribute('open')===null)await tap('[data-equipment-detail="company-administration"] > summary');
  await action('company_capitalize');await field('amount_mn',Math.ceil(stockCapital*1100));await confirm('company_capitalize');
  await save('s19-aircraft-development');
  }
  let product;
  for(let i=0;i<700;i++){
    const now=await day();if(i%15!==0)continue;const board=await read('equipment');product=board.companies.products.find(p=>p.name==='S19 Light Attack');
    console.log('AIRCRAFT',JSON.stringify({days:i+1,date:now.date,product}));
    if(i>0&&i%120===0)await save('s19-aircraft-progress-'+i);
    if(product?.availability?.ready_stock>=1)break;
  }
  assert(product?.availability?.ready_stock>=1,'No actual aircraft stock in 700 ordinary days');f.stock=product;
  await save('s19-aircraft-stock');await equipment('companies');await action('company_purchase',{product:product.product});await field('quantity',1);f.purchase=await confirm('company_purchase');
  for(let i=0;i<40;i++){const native=await read('guidance');f.delivery=native.outcomes.procurement.deliveries.find(d=>d.revision===product.source_revision&&d.quantity===1&&d.delivered_day!==null);if(f.delivery)break;await day();}
  assert(f.delivery);await save('s19-aircraft-delivered');await observe('flight-aircraft-delivered');await shot('flight-aircraft-delivered');
  f.passed=true;
};
