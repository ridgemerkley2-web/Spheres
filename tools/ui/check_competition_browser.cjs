// Destructive campaign QA: use ONLY a separate local server with a disposable cwd.
// NODE_PATH=<bundled libraries> PLAYWRIGHT_CHANNEL=msedge node this-file URL --disposable [screenshots]
const assert=require('node:assert/strict');
const {chromium}=require('playwright');
const fs=require('node:fs');
const path=require('node:path');
(async()=>{
  const base=new URL(process.argv[2]);
  assert(['localhost','127.0.0.1'].includes(base.hostname));
  assert.equal(process.argv[3],'--disposable','This test replaces the server campaign');
  const screenshots=process.argv[4];if(screenshots)fs.mkdirSync(screenshots,{recursive:true});
  const browser=await chromium.launch({headless:true,...(process.env.PLAYWRIGHT_CHANNEL?{channel:process.env.PLAYWRIGHT_CHANNEL}:{})});
  try {
    const page=await browser.newPage({viewport:{width:1440,height:1000},reducedMotion:'reduce'});
    page.setDefaultTimeout(30000);
    const errors=[];page.on('pageerror',e=>errors.push(e.message));page.on('dialog',d=>d.accept());
    await page.goto(base.href);await page.waitForFunction(()=>!!SESSION.live);
    if(await page.locator('#newCampaignPicker').isHidden())await page.locator('#newCampaignBtn').click();
    await page.locator('#nationPick [aria-label^="France;"]').click();
    await page.locator('#startBtn').click();await page.locator('#app').waitFor({state:'visible'});
    await page.locator('#competitionDockBtn').click();
    await page.locator('[data-comp-action="enable"]').click();
    await page.waitForFunction(()=>COMP.data?.enabled && !COMP.busy && !COMP.loading);
    assert.equal(await page.locator('#app').evaluate(e=>e.inert),true);
    assert(await page.getByRole('heading',{name:'Make something the world needs.'}).isVisible());
    const date=await page.evaluate(()=>S.date);
    await page.keyboard.press('4');assert.equal(await page.evaluate(()=>S.date),date,'modal shortcut advanced a year');
    for(const [width,height] of [[1440,1000],[820,700],[390,844]]) {
      await page.setViewportSize({width,height});
      for(const tab of ['industry','trade','world','sphere']) {
        await page.locator(`[data-comp-tab="${tab}"]`).click();
        await page.waitForFunction(()=>{
          const tabs=[...document.querySelectorAll('[data-comp-tab]')];
          return tabs.every(t=>(getComputedStyle(t).backgroundColor==='rgb(198, 182, 223)')===(t.getAttribute('aria-selected')==='true'));
        });
        await page.locator('#competitionClose').click({trial:true});
        const overflow=await page.locator('#competitionRoom').evaluate(e=>e.scrollWidth>e.clientWidth+1);
        assert(!overflow,`${tab} overflows room at ${width}`);
        assert.equal(await page.locator('#competitionBody').getAttribute('aria-labelledby'),`comp-tab-${tab}`);
        if(tab==='industry') {
          const cards=page.locator('.comp-capacity-card');
          assert.equal(await cards.count(),2,'Materials and Machinery planning cards');
          const supplyCards=page.locator('.comp-supply-card');
          assert.equal(await supplyCards.count(),2,'Materials and Machinery supply forecast cards');
          for(const card of await supplyCards.all()) {
            assert(!await card.evaluate(e=>e.scrollWidth>e.clientWidth+1),`supply card overflows at ${width}`);
            assert.equal(await card.locator('.comp-supply-score>div').count(),3,'Need, Covered and Gap stay distinct');
            assert(await card.locator('summary').evaluate(e=>e.getBoundingClientRect().height>=44));
          }
          await page.locator('#capacityHeading').scrollIntoViewIfNeeded();
          assert(await page.getByText('not a 1990 factory census',{exact:false}).isVisible());
          for(const card of await cards.all()) {
            assert(!await card.evaluate(e=>e.scrollWidth>e.clientWidth+1),`capacity card overflows at ${width}`);
            assert(await card.locator('h4').evaluate(e=>parseFloat(getComputedStyle(e).fontSize)>=24));
            assert(await card.locator('.comp-capacity-reason').evaluate(e=>parseFloat(getComputedStyle(e).fontSize)>=16));
            assert(await card.locator('summary').evaluate(e=>e.getBoundingClientRect().height>=44));
          }
          if(screenshots)await page.screenshot({path:path.join(screenshots,`capacity-plan-${width}.png`)});
          await cards.first().locator('summary').click();
          assert(await cards.first().getByText('Incoming is not usable until delivery.',{exact:false}).isVisible());
          assert(!await page.locator('#competitionBody').evaluate(e=>e.scrollWidth>e.clientWidth+1),`planning detail overflows at ${width}`);
          if(screenshots)await cards.first().screenshot({path:path.join(screenshots,`capacity-materials-detail-${width}.png`)});
          await cards.first().locator('summary').click();
        }
        if(tab==='world') {
          const first=page.locator('.comp-table tbody tr').first();
          assert(await first.isVisible());
          if(width===390) {
            assert.equal(await first.evaluate(e=>getComputedStyle(e).display),'grid','phone world rows become cards');
            for(const selector of ['.comp-table-wrap','.comp-table tbody tr','.comp-decision-cell']) {
              const el=page.locator(selector).first();
              assert(!await el.evaluate(e=>e.scrollWidth>e.clientWidth+1),`${selector} overflows phone world card`);
            }
          }
        }
        if(screenshots && (width===1440||width===390))await page.screenshot({path:path.join(screenshots,`exchange-${tab}-${width}.png`)});
      }
    }
    await page.setViewportSize({width:1440,height:1000});
    await page.locator('[data-comp-tab="world"]').click();
    assert.equal(await page.locator('.comp-table tbody tr').count(),137);
    await page.locator('#competitionTier').selectOption('Micro');
    assert(await page.locator('.comp-table tbody tr').count()>0);
    await page.locator('#competitionFilter').fill('zzz-no-country');
    assert.equal(await page.locator('.comp-table tbody tr').count(),0);
    await page.locator('[data-comp-tab="trade"]').click();
    await page.getByRole('button',{name:'Find suppliers',exact:true}).click();
    await page.waitForFunction(()=>!COMP.searching);
    assert(await page.getByText('No affordable, reachable supplier', {exact:false}).isVisible());
    // The real server commits an immediate command; browser sees a lost response.
    // Reload must preserve and resend the SAME receipt without a second execution.
    await page.keyboard.press('Escape');
    assert.equal(await page.locator('#app').evaluate(e=>e.inert),false);
    assert.equal(await page.evaluate(()=>document.activeElement.id),'competitionDockBtn');
    await page.evaluate(async()=>{
      const command=programBudgetCommand(me());
      await adopt(await api('/api/command',{commands:[command]}),false);
    });
    await page.locator('#competitionDockBtn').click();
    await page.waitForFunction(()=>COMP.data && !COMP.loading);
    await page.locator('[data-comp-tab="trade"]').click();
    let committed=null,releaseOrder;
    const heldOrder=new Promise(resolve=>releaseOrder=resolve);
    await page.route('**/api/command',async route=>{
      const response=await route.fetch();committed=await response.json();
      await heldOrder;
      await route.fulfill({status:500,contentType:'application/json',body:JSON.stringify({error:'Lost economic response fixture'})});
    });
    const form=page.locator('.comp-sale-form').first();
    await form.locator('[name="enabled"]').check();
    await form.getByRole('button',{name:'Apply export policy'}).click();
    await page.waitForFunction(()=>COMP.busy && !!COMP.pending);
    await page.keyboard.press('Escape');
    const beforePendingAdvance=await page.evaluate(()=>S.date);
    assert.equal(await page.evaluate(()=>advance(1)),false,'time must wait for the in-flight economic receipt');
    assert.equal(await page.evaluate(()=>S.date),beforePendingAdvance);
    releaseOrder();
    await page.waitForFunction(()=>!!COMP.pending&&!COMP.busy);
    assert(committed);const identity=await page.evaluate(()=>JSON.stringify(COMP.pending));
    // Private/full browser storage cannot erase the in-memory receipt on reopen.
    await page.evaluate(()=>Object.defineProperty(window,'sessionStorage',{configurable:true,value:{
      getItem(){throw new Error('Storage denied fixture');},setItem(){throw new Error('Storage denied fixture');},removeItem(){throw new Error('Storage denied fixture');}
    }}));
    await page.keyboard.press('Escape');
    await page.locator('#competitionDockBtn').click();
    assert.equal(await page.evaluate(()=>JSON.stringify(COMP.pending)),identity);
    await page.unroute('**/api/command');
    await page.reload();await page.locator('#continueBtn').click();await page.locator('#app').waitFor({state:'visible'});
    await page.locator('#competitionDockBtn').click();
    assert.equal(await page.evaluate(()=>JSON.stringify(COMP.pending)),identity);
    const replay=page.waitForResponse(r=>r.url().endsWith('/api/command'));
    await page.getByRole('button',{name:'Check order receipt'}).click();
    assert.equal((await(await replay).json()).command_replayed,true);
    await page.waitForFunction(()=>!COMP.pending&&!COMP.busy);
    assert.equal(await page.evaluate(()=>S.date),committed.date);
    assert.deepEqual(errors,[]);
    console.log('PASS: Exchange four views and readable capacity cards desktop/tablet/mobile, no planning overflow,137-country filters, no blind time, pure supplier search, session receipt reload/retry, focus return, zero page errors.');
  } finally {await browser.close();}
})().catch(e=>{console.error(e);process.exitCode=1;});
