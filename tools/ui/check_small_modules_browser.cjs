// Mutates only an explicitly disposable local campaign. Uses real UI/server rules.
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
    const page=await browser.newPage({viewport:{width:1440,height:1000},reducedMotion:'reduce'});
    page.setDefaultTimeout(30000);
    const errors=[];page.on('pageerror',e=>errors.push(e.message));page.on('dialog',d=>d.accept());
    await page.goto(base.href);await page.waitForFunction(()=>!!SESSION.live);
    if(await page.locator('#newCampaignPicker').isHidden())await page.locator('#newCampaignBtn').click();
    await page.locator('#nationPick [aria-label^="Tonga;"]').click();
    await page.locator('#startBtn').click();await page.locator('#app').waitFor({state:'visible'});
    await page.locator('#competitionDockBtn').click();await page.locator('[data-comp-action="enable"]').click();
    await page.waitForFunction(()=>COMP.data?.enabled&&!COMP.busy&&!COMP.loading);
    await page.keyboard.press('Escape');
    await page.evaluate(async()=>{await adopt(await api('/api/command',{commands:[programBudgetCommand(me())]}),false);});
    await page.locator('#competitionDockBtn').click();
    await page.waitForFunction(()=>COMP.data?.module_board&&!COMP.loading);
    await page.locator('[data-comp-action="module-toggle"]').click();
    assert.equal(await page.locator('.comp-module-choice').count(),3);
    const quote=await page.evaluate(()=>COMP.data.module_board.selection.quotes[1]);
    assert(quote.capacity_micros>0&&quote.capacity_micros<10000,'Tonga receives a genuinely small package');
    assert(quote.cost_bn<.005&&quote.output_daily>0&&quote.can_start);
    for(const [width,height] of [[1440,1000],[820,900],[390,844]]){
      await page.setViewportSize({width,height});
      await page.locator('#moduleHeading').scrollIntoViewIfNeeded();
      assert(!await page.locator('#competitionRoom').evaluate(e=>e.scrollWidth>e.clientWidth+1));
      assert(!await page.locator('.comp-workshop').evaluate(e=>e.scrollWidth>e.clientWidth+1));
      if(screenshots)await page.screenshot({path:path.join(screenshots,`starter-workshop-${width}.png`)});
      await page.locator('.comp-module-choice').nth(1).scrollIntoViewIfNeeded();
      assert(await page.locator('[data-comp-action="module-build"][data-module-quote="1"]').isVisible());
      if(screenshots)await page.screenshot({path:path.join(screenshots,`starter-workshop-choice-${width}.png`)});
    }
    await page.setViewportSize({width:1440,height:1000});
    const second=await page.locator('#competitionModuleProvince option').nth(1).getAttribute('value');
    await page.locator('#competitionModuleProvince').selectOption(second);
    await page.waitForFunction(d=>COMP.moduleQuotes?.district===d&&!COMP.moduleLoading,second);
    const chosen=await page.evaluate(()=>COMP.moduleQuotes.quotes[1]);
    let committed;
    await page.route('**/api/command',async route=>{
      const response=await route.fetch();committed=await response.json();
      await route.fulfill({status:500,contentType:'application/json',body:JSON.stringify({error:'Lost workshop response fixture'})});
    });
    await page.locator('[data-comp-action="module-build"][data-module-quote="1"]').click();
    await page.waitForFunction(()=>!!COMP.pending&&!COMP.busy);
    assert(committed);assert.deepEqual(committed.errors,[]);
    await page.unroute('**/api/command');
    const replay=page.waitForResponse(r=>r.url().endsWith('/api/command'));
    await page.getByRole('button',{name:'Check order receipt'}).click();
    assert.equal((await(await replay).json()).command_replayed,true);
    await page.waitForFunction(()=>!COMP.pending&&!COMP.busy&&!COMP.loading);
    const projects=await page.evaluate(()=>COMP.data.module_board.projects);
    assert.equal(projects.length,1,'Lost response never orders another module');
    assert.equal(projects[0].capacity_micros,chosen.capacity_micros);
    assert.equal(projects[0].province.id,second);
    assert(Math.abs(projects[0].finance.cost_bn-chosen.cost_bn)<1e-12);
    assert.equal(projects[0].progress,0,'Placing an order does not commission production');
    const planned=await page.evaluate(()=>COMP.data.capacity_plan.goods.find(g=>g.good==='intermediates'));
    assert.equal(planned.installed_daily,0,'queued work must not become installed capacity');
    assert.equal(planned.committed_daily,chosen.output_daily,'planning uses the actual frozen order size');
    for(const width of [1440,390]) {
      await page.setViewportSize({width,height:1000});
      const card=page.locator('.comp-capacity-intermediates');
      await card.scrollIntoViewIfNeeded();
      assert(!await card.evaluate(e=>e.scrollWidth>e.clientWidth+1),`queued-capacity card overflows at ${width}`);
      assert.equal(await card.locator('.comp-metrics').first().locator('dd').nth(1).innerText(),
        await page.evaluate(value=>competitionNumber(value),chosen.output_daily));
      if(screenshots)await card.screenshot({path:path.join(screenshots,`capacity-queued-workshop-${width}.png`)});
    }
    await page.keyboard.press('Escape');
    assert.equal(await page.evaluate(()=>document.activeElement.id),'competitionDockBtn');
    assert.deepEqual(errors,[]);
    console.log(`PASS: real Tonga ${chosen.capacity_micros}µ workshop quote, province selection, desktop/tablet/mobile, exact-size order, lost-response retry once, no instant output, zero page errors.`);
  }finally{await browser.close();}
})().catch(e=>{console.error(e);process.exitCode=1;});
