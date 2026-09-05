// Uses actual UI/server commands; replaces only an explicitly disposable local campaign.
// NODE_PATH=<libraries> PLAYWRIGHT_CHANNEL=msedge node this-file URL --disposable [screenshots]
const assert=require('node:assert/strict');
const {chromium}=require('playwright');
const fs=require('node:fs');
const path=require('node:path');
(async()=>{
  const base=new URL(process.argv[2]);
  assert(['localhost','127.0.0.1'].includes(base.hostname));
  assert.equal(process.argv[3],'--disposable','Requires an isolated disposable server');
  const screenshots=process.argv[4];if(screenshots)fs.mkdirSync(screenshots,{recursive:true});
  const browser=await chromium.launch({headless:true,...(process.env.PLAYWRIGHT_CHANNEL?{channel:process.env.PLAYWRIGHT_CHANNEL}:{})});
  try{
    const page=await browser.newPage({viewport:{width:1430,height:1000},reducedMotion:'reduce'});
    page.setDefaultTimeout(30000);
    const captureCard=async(selector,file,width,height)=>{
      if(!screenshots)return;
      // A nested scroll room clips element screenshots outside its viewport.
      // Temporarily make that viewport tall enough for the whole card at the
      // same tested width, then restore the actual device-height interaction.
      const card=page.locator(selector),box=await card.boundingBox();
      await page.setViewportSize({width,height:Math.max(height,Math.ceil(box.height+340))});
      await card.evaluate(e=>e.scrollIntoView({block:'start'}));
      await card.screenshot({path:path.join(screenshots,file)});
      await page.setViewportSize({width,height});
    };
    const errors=[];page.on('pageerror',e=>errors.push(e.message));page.on('dialog',d=>d.accept());
    await page.goto(base.href);await page.waitForFunction(()=>!!SESSION.live);
    if(await page.locator('#newCampaignPicker').isHidden())await page.locator('#newCampaignBtn').click();
    await page.locator('#nationPick [aria-label^="France;"]').click();
    await page.locator('#startBtn').click();await page.locator('#app').waitFor({state:'visible'});
    await page.locator('#competitionDockBtn').click();await page.locator('[data-comp-action="enable"]').click();
    await page.waitForFunction(()=>COMP.data?.enabled&&!COMP.busy&&!COMP.loading);
    await page.keyboard.press('Escape');
    // Existing public command builder enrolls an unchanged ministry plan; no test-only stock/cash grant.
    await page.evaluate(async()=>{await adopt(await api('/api/command',{commands:[programBudgetCommand(me())]}),false);});
    await page.locator('#competitionDockBtn').click();
    await page.waitForFunction(()=>COMP.data?.materials&&!COMP.loading);
    assert.equal(await page.locator('.comp-material-stat').count(),3);
    assert.equal(await page.locator('.comp-material-ledger').getAttribute('open'),null);
    const initial=await page.evaluate(()=>COMP.data.materials);
    assert(initial.capacity_daily>0);assert(initial.output_daily===0,'opening output is exactly zero (including IEEE negative zero)');
    for(const [width,height] of [[1430,1000],[820,900],[390,844]]){
      await page.setViewportSize({width,height});
      await page.locator('.comp-materials').evaluate(e=>e.scrollIntoView({block:'start'}));
      for(const selector of ['#competitionRoom','.comp-materials','.comp-material-stat']){
        for(const el of await page.locator(selector).all())assert(!await el.evaluate(e=>e.scrollWidth>e.clientWidth+1),`${selector} overflow at ${width}`);
      }
      await captureCard('.comp-materials',`materials-dashboard-${width}.png`,width,height);
    }
    await page.locator('[data-comp-action="materials-toggle"]').click();
    const form=page.locator('#competitionMaterialsForm');
    assert(await form.isVisible());
    const before=await page.evaluate(()=>({quantity:competitionMaterialsSelection().quote.quantity,days:competitionMaterialsSelection().quote.delivery_days}));
    await form.locator('[name="quantity"]').fill(String(before.quantity/2));
    assert.equal(await page.locator('[data-comp-action="materials-order"]').count(),0,'editing removes obsolete quote');
    const quoteResponse=page.waitForResponse(r=>r.url().endsWith('/api/materials-quote'));
    await form.getByRole('button',{name:'Check this order',exact:true}).click();
    const quote=await(await quoteResponse).json();
    await page.waitForFunction(()=>!COMP.materialLoading&&!!COMP.materialQuote);
    assert.equal(quote.quantity,before.quantity/2);assert(quote.can_start,quote.refusal||'order should be eligible');
    assert.equal(quote.delivery_days,before.days);
    for(const [width,height] of [[1430,1000],[820,900],[390,844]]){
      await page.setViewportSize({width,height});
      await page.locator('.comp-material-quote').scrollIntoViewIfNeeded();
      assert(!await page.locator('.comp-material-quote').evaluate(e=>e.scrollWidth>e.clientWidth+1));
      assert(await page.locator('[data-comp-action="materials-order"]').isVisible());
      await captureCard('.comp-material-quote',`materials-order-${width}.png`,width,height);
    }
    // Real phone-height evidence: a fixed header must not cover editable
    // controls or the purchase action; scrollWidth alone cannot prove that.
    await page.setViewportSize({width:390,height:844});
    const assertInsideRoom=async selector=>{
      const box=await page.locator(selector).boundingBox(),body=await page.locator('#competitionBody').boundingBox();
      assert(box&&body&&box.x>=-1&&box.x+box.width<=391,`${selector} must fit the phone horizontally`);
      assert(box.y>=body.y-1&&box.y+box.height<=Math.min(body.y+body.height,844)+1,`${selector} must not be clipped by the fixed header or viewport`);
    };
    await form.evaluate(e=>e.scrollIntoView({block:'start'}));
    for(const selector of ['#competitionMaterialsProvince','#competitionMaterialsForm [name="quantity"]',
      '#competitionMaterialsForm [name="delivery_days"]','#competitionMaterialsCheck'])await assertInsideRoom(selector);
    if(screenshots)await page.screenshot({path:path.join(screenshots,'mobile-order-start.png')});
    await page.locator('[data-comp-action="materials-order"]').evaluate(e=>e.scrollIntoView({block:'center'}));
    await assertInsideRoom('[data-comp-action="materials-order"]');
    if(screenshots)await page.screenshot({path:path.join(screenshots,'mobile-order-action.png')});
    // Deliberately lose an already-committed HTTP reply, then use the UI receipt retry.
    let committed;
    await page.route('**/api/command',async route=>{
      const response=await route.fetch();committed=await response.json();
      await route.fulfill({status:500,contentType:'application/json',body:JSON.stringify({error:'Lost Materials response fixture'})});
    });
    await page.locator('[data-comp-action="materials-order"]').click();
    await page.waitForFunction(()=>!!COMP.pending&&!COMP.busy);
    assert(committed);assert.deepEqual(committed.errors,[]);
    await page.unroute('**/api/command');
    const replay=page.waitForResponse(r=>r.url().endsWith('/api/command'));
    await page.getByRole('button',{name:'Check order receipt'}).click();
    assert.equal((await(await replay).json()).command_replayed,true);
    await page.waitForFunction(()=>!COMP.pending&&!COMP.busy&&!COMP.loading);
    const ordered=await page.evaluate(()=>COMP.data.materials);
    assert.equal(ordered.orders.length,1);assert.equal(ordered.orders[0].quantity,quote.quantity);
    assert.equal(ordered.orders[0].district,quote.district);
    assert.equal(ordered.orders[0].delivered,0);assert.equal(ordered.stock,initial.stock,'order placement grants no goods');
    await page.locator('[data-comp-action="materials-cancel"]').click();
    await page.waitForFunction(()=>!COMP.pending&&!COMP.busy&&!COMP.loading&&COMP.data.materials.orders[0].status==='cancelled');
    const cancelled=await page.evaluate(()=>COMP.data.materials.orders[0]);assert.equal(cancelled.delivered,0);
    await page.locator('[data-comp-action="materials-import"]').click();
    assert(await page.locator('#competitionQuoteForm').isVisible());
    assert.equal(await page.locator('#competitionQuoteForm [name="good"]').inputValue(),'intermediates');
    await page.locator('[data-comp-tab="industry"]').click();
    await page.locator('[data-comp-action="materials-sell"]').click();
    assert(await page.locator('.comp-sale-form[data-good="intermediates"]').isVisible());
    await page.locator('[data-comp-tab="industry"]').click();
    await page.locator('[data-comp-action="materials-expand"]').click();
    assert.equal(await page.evaluate(()=>PROD.pickKind),'processing_plant');
    assert.equal(await page.evaluate(()=>PROD.view),'provinces');
    await page.locator('#productionClose').click();await page.locator('#competitionDockBtn').click();
    await page.waitForFunction(()=>!COMP.loading);
    await page.locator('[data-comp-action="materials-upgrade"]').click();
    assert.equal(await page.evaluate(()=>PROD.pickKind),'automation');
    await page.locator('#productionClose').click();await page.locator('#competitionDockBtn').click();
    await page.waitForFunction(()=>!COMP.loading);
    await page.keyboard.press('Escape');
    assert.equal(await page.evaluate(()=>document.activeElement.id),'competitionDockBtn');
    assert.deepEqual(errors,[]);
    console.log('PASS: Materials desktop/tablet/mobile, phone control/action bounds, three authoritative cards, editable paid quote, stale-draft barrier, exact order, lost-response replay once, no free goods, cancellation, Expand/Upgrade/Import/Sell routes and zero page errors.');
  }finally{await browser.close();}
})().catch(error=>{console.error(error);process.exitCode=1;});
