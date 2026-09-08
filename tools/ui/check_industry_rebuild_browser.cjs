// Exercises a disposable local campaign. Never point this at a user's live game.
// NODE_PATH=<Playwright modules> PLAYWRIGHT_CHANNEL=msedge node this-file URL --disposable [screenshots]
const assert=require('node:assert/strict');
const fs=require('node:fs'),path=require('node:path');
const {chromium}=require('playwright');
(async()=>{
  const base=new URL(process.argv[2]);
  assert(['localhost','127.0.0.1'].includes(base.hostname));
  assert.equal(process.argv[3],'--disposable');
  const shots=process.argv[4];if(shots)fs.mkdirSync(shots,{recursive:true});
  const browser=await chromium.launch({headless:true,...(process.env.PLAYWRIGHT_CHANNEL?{channel:process.env.PLAYWRIGHT_CHANNEL}:{})});
  try{
    const page=await browser.newPage({viewport:{width:1440,height:1000},reducedMotion:'reduce'});
    page.setDefaultTimeout(30000);const errors=[];
    page.on('pageerror',e=>errors.push(e.message));
    await page.goto(base.href,{waitUntil:'domcontentloaded',timeout:60000});await page.waitForFunction(()=>!!SESSION.live);
    if(await page.locator('#newCampaignPicker').isHidden())await page.locator('#newCampaignBtn').click();
    await page.locator('#nationPick [aria-label^="France;"]').click();
    const replacing=await page.evaluate(()=>!!SESSION.live?.player);
    await page.locator('#startBtn').click();if(replacing)await page.locator('#campaignConfirmAccept').click();
    await page.locator('#app').waitFor({state:'visible'});
    await page.locator('#productionDockBtn').click();
    await page.waitForFunction(()=>!!PROD.data?.industry_rebuild?.capacity&&!PROD.loading);
    assert(await page.locator('#constructionCapacityTitle').isVisible());
    await page.locator('#constructionDailyBudget').fill('5');
    await page.locator('#constructionBudgetForm').getByRole('button',{name:'Apply budget',exact:true}).click();
    await page.waitForFunction(()=>!PROD.busy&&!PROD.loading&&PROD.data?.construction_budget?.explicit===true);
    if(shots)await page.locator('#banner').waitFor({state:'hidden'});
    await page.locator('[data-prod-new]').click();
    for(const title of ['Build faster','Earn income','Equip forces','Support economy','Facility upgrades'])assert(await page.locator('.construction-catalog-group h3').filter({hasText:title}).isVisible(),title);
    assert(await page.locator('#constructionUpgradeProvince').isVisible());
    assert.equal(await page.locator('[data-prod-kind="automation"]').count(),0);
    for(const [width,height]of [[1440,1000],[820,800],[390,844]]){
      await page.setViewportSize({width,height});
      await page.getByRole('heading',{name:'What will you build?',exact:true}).scrollIntoViewIfNeeded();
      for(const card of await page.locator('.construction-capacity,.project-choice').all()){const overflow=await card.evaluate(el=>el.getClientRects().length&&el.scrollWidth>el.clientWidth+1?{className:el.className,text:el.innerText.slice(0,100),width:el.clientWidth,scroll:el.scrollWidth}:null);if(overflow){if(shots)await page.screenshot({path:path.join(shots,`industry-overflow-${width}.png`)});assert.fail(`catalog overflow at ${width}: ${JSON.stringify(overflow)}`);}}
      assert(!await page.locator('#productionBody').evaluate(el=>el.scrollWidth>el.clientWidth+1),`panel overflow at ${width}`);
      if(shots)await page.screenshot({path:path.join(shots,`industry-catalog-${width}.png`)});
    }
    await page.setViewportSize({width:1440,height:1000});
    await page.locator('[data-prod-kind="office_district"]').click();
    await page.locator('[data-prod-province]').first().click();
    await page.waitForFunction(()=>constructionPreviewCurrent());
    assert.match(await page.locator('#constructionPreviewTitle').innerText(),/Office/i);
    assert(await page.locator('#constructionNationalImpact').isVisible());
    assert.equal(await page.locator('[data-construction-confirm]').count(),1);
    for(const [width,height]of [[1440,1000],[390,844]]){
      await page.setViewportSize({width,height});
      await page.locator('#constructionPreviewTitle').scrollIntoViewIfNeeded();
      assert(!await page.locator('#productionBody').evaluate(el=>el.scrollWidth>el.clientWidth+1),`preview overflow at ${width}`);
      if(shots)await page.screenshot({path:path.join(shots,`industry-preview-${width}.png`)});
    }
    await page.setViewportSize({width:1440,height:1000});
    await page.locator('[data-prod-back]').click();
    await page.locator('[data-prod-back]').click();
    await page.locator('[data-prod-kind="industry_preset"]').click();
    await page.locator('[data-prod-province]').first().click();
    await page.waitForFunction(()=>constructionPreviewCurrent()&&PROD.preview?.project_kind==='industry_preset');
    assert.equal(await page.locator('.construction-preset-components li').count(),4);
    const before=await page.evaluate(()=>PROD.data.queue.length);
    await page.locator('[data-construction-confirm]').click();
    await page.waitForFunction(n=>PROD.view==='queue'&&!PROD.busy&&!PROD.loading&&PROD.data.queue.length===n+4,before);
    const allocation=page.locator('[data-construction-allocation]').first();
    const project=await allocation.getAttribute('data-construction-allocation');
    await allocation.locator('input').fill('0');
    await allocation.getByRole('button',{name:'Assign capacity',exact:true}).click();
    await page.waitForFunction(id=>!PROD.busy&&!PROD.loading&&productionProject(id)?.allocation?.requested===0,project);
    assert.match(await page.locator(`[data-construction-allocation="${project}"]`).innerText(),/Manual.*0 assigned/s);
    await page.locator(`[data-construction-allocation-auto="${project}"]`).click();
    await page.waitForFunction(id=>!PROD.busy&&!PROD.loading&&productionProject(id)?.allocation?.mode==='auto',project);
    if(shots)await page.locator('#banner').waitFor({state:'hidden'});
    const capacity=await page.evaluate(()=>PROD.data.industry_rebuild.capacity);
    assert(capacity.assigned_capacity<=capacity.total_capacity+1e-8);
    for(const [width,height]of [[1440,1000],[390,844]]){
      await page.setViewportSize({width,height});
      await page.locator('[data-construction-allocation]').first().scrollIntoViewIfNeeded();
      for(const card of await page.locator('.construction-allocation,.work-card').all())assert(!await card.evaluate(el=>el.scrollWidth>el.clientWidth+1),`allocation overflow at ${width}`);
      if(shots)await page.screenshot({path:path.join(shots,`industry-queue-${width}.png`)});
    }
    await page.locator('[data-construction-cabinet="industry"]').click();
    await page.waitForFunction(()=>IDESK.data?.industry_rebuild?.operations&&!IDESK.loading);
    assert(await page.locator('#industryUnifiedTitle').isVisible());
    assert.match(await page.locator('.industry-unified').innerText(),/Installed capacity/);
    assert.match(await page.locator('.industry-unified').innerText(),/Inherited national industry/);
    for(const [width,height]of [[1440,1000],[820,800],[390,844]]){
      await page.setViewportSize({width,height});
      await page.locator('.industry-unified .industry-site').first().scrollIntoViewIfNeeded();
      for(const card of await page.locator('.industry-unified .industry-site').all())assert(!await card.evaluate(el=>el.scrollWidth>el.clientWidth+1),`operations overflow at ${width}`);
      if(shots)await page.screenshot({path:path.join(shots,`industry-operations-${width}.png`)});
    }
    await page.locator('#cab-tab-construction').click();
    await page.locator('[data-construction-cabinet="companies"]').click();
    await page.waitForFunction(()=>CDESK.data&&!CDESK.loading&&!CDESK.stale&&CDESK.state===S);
    assert(await page.locator('#company-directory-title').isVisible());
    assert(await page.locator('[data-company-record]').count()>0,'new campaign has a company roster');
    for(const [width,height]of [[1440,1000],[390,844]]){
      await page.setViewportSize({width,height});
      await page.locator('[data-company-record]').first().scrollIntoViewIfNeeded();
      for(const card of await page.locator('.company-card').all())assert(!await card.evaluate(el=>el.scrollWidth>el.clientWidth+1),`company overflow at ${width}`);
      if(shots)await page.screenshot({path:path.join(shots,`industry-companies-${width}.png`)});
    }
    await page.locator('#cab-tab-companies').focus();
    await page.keyboard.press('ArrowLeft');
    await page.waitForFunction(()=>CAB.tab==='industry'&&IDESK.data?.industry_rebuild?.operations&&!IDESK.loading&&!IDESK.stale&&IDESK.state===S);
    assert(await page.locator('#cab-tab-industry').evaluate(el=>el===document.activeElement),'keyboard returns to Industry');
    assert.deepEqual(errors,[]);console.log('Industry rebuild browser flow passed: project roles, preset, shared allocation, operations, Companies coexistence and 390px layout.');
  }finally{await browser.close();}
})().catch(error=>{console.error(error);process.exit(1);});
