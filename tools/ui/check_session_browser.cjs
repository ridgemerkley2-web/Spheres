// Real-browser regressions. ONLY use a disposable server whose working directory
// is disposable too: this test intentionally writes and loads its save.json.
// NODE_PATH=<Playwright packages> PLAYWRIGHT_CHANNEL=msedge node this-file URL --disposable
const assert = require('node:assert/strict');
const { chromium } = require('playwright');
const fs = require('node:fs');
const path = require('node:path');

(async () => {
  const base = new URL(process.argv[2]);
  assert(['127.0.0.1', 'localhost'].includes(base.hostname));
  assert.equal(process.argv[3], '--disposable', 'This test replaces campaigns and save.json');
  const screenshots = process.argv[4];
  if (screenshots) fs.mkdirSync(screenshots, {recursive:true});
  const browser = await chromium.launch({headless:true,
    ...(process.env.PLAYWRIGHT_CHANNEL ? {channel:process.env.PLAYWRIGHT_CHANNEL} : {})});
  try {
    const page = await browser.newPage({viewport:{width:1440,height:1000}, reducedMotion:'reduce'});
    page.setDefaultTimeout(20000);
    const errors = [];
    page.on('pageerror', error => errors.push(error.message));
    page.on('dialog', dialog => dialog.accept());
    const state = async () => (await page.request.get(new URL('/api/state', base).href)).json();
    const settled = () => page.waitForFunction(() => !advancing);
    const more = async () => {
      if (await page.locator('.arc-time-menu').getAttribute('open') === null) {
        await page.locator('.arc-time-menu > summary').click();
      }
    };
    await page.goto(base.href);
    await page.waitForFunction(() => !!SESSION.live);
    // A repeated test may find the previous disposable campaign.
    if (await page.locator('#newCampaignPicker').isHidden()) await page.locator('#newCampaignBtn').click();
    for (const [width,height] of [[1440,1000],[414,1000],[820,700]]) {
      await page.setViewportSize({width,height});
      // Trial clicks perform real hit testing without loading or starting a game.
      await page.locator('#loadBtn').click({trial:true});
      await page.locator('#newCampaignBtn').click({trial:true});
      await page.locator('#nationPick [aria-label^="France;"]').click();
      await page.locator('#startBtn').click({trial:true});
      assert(await page.evaluate(() => document.documentElement.scrollWidth <= innerWidth + 1),
        `Country picker overflows at ${width}`);
      if (screenshots) await page.screenshot({path:path.join(screenshots,`setup-${width}.png`)});
    }
    await page.setViewportSize({width:1440,height:1000});
    await page.locator('#nationPick [aria-label^="France;"]').click();
    const started = page.waitForResponse(r => r.url().endsWith('/api/new'));
    await page.locator('#startBtn').click();
    const initial = await (await started).json();
    await page.locator('#app').waitFor({state:'visible'});
    const current = await state();
    assert.equal(initial.simulation_cadence, 'daily');
    assert.equal(initial.resources.rows[0].unit, current.resources.rows[0].unit);
    assert.equal(initial.session_id, current.session_id);
    await page.waitForFunction(() => ui.cam.cx !== undefined && ui.cam.cy !== undefined);
    const camera = await page.evaluate(() => ({cx:ui.cam.cx, cy:ui.cam.cy}));
    assert(camera.cx !== 0 || camera.cy !== 0, 'Initial map must not use the obsolete (0,0) sentinel');
    if (screenshots) await page.screenshot({path:path.join(screenshots,'fixed-map.png')});

    // Parse failure must reject the WHOLE batch without applying the valid tax.
    const bad = await page.request.post(new URL('/api/advance', base).href, {data:{days:1,commands:[
      {kind:'tax',value:0.3}, {kind:'tax',value:'invalid'}]}});
    assert.equal(bad.status(), 400);
    assert.equal((await bad.json()).not_advanced, true);
    assert.deepEqual(await state(), current);

    // An edit made during a real HTTP advance must stay queued for the next day.
    let release, observed;
    const gate = new Promise(resolve => {release=resolve;});
    const seen = new Promise(resolve => {observed=resolve;});
    await page.route('**/api/advance', async route => {observed(); await gate; await route.continue();});
    await page.locator('[data-step="1"]').click();
    await seen;
    await page.locator('[data-drawer="cabinetDrawer"]').click();
    await page.locator('#cab-tab-budget').click();
    await page.getByRole('button',{name:'Raise Health',exact:true}).click();
    const draft = await page.evaluate(() => JSON.stringify(queued));
    assert.notEqual(draft, '[]');
    release(); await settled();
    assert.equal(await page.evaluate(() => JSON.stringify(queued)), draft);
    await page.keyboard.press('Escape');
    await page.unroute('**/api/advance');

    // Server commits, but the response is lost. Reload and retry that receipt:
    // the date and enacted budget must not move a second time.
    let committed;
    await page.route('**/api/advance', async route => {
      const response = await route.fetch();
      committed = await response.json();
      await route.fulfill({status:500,contentType:'application/json',body:JSON.stringify({error:'Lost response fixture'})});
    });
    await page.locator('[data-step="1"]').click();
    await page.locator('#pendingTurn').waitFor({state:'visible'});
    await settled();
    assert.match(await page.locator('#banner').innerText(), /Lost response fixture/);
    assert(committed && committed.date);
    assert.equal(await page.evaluate(() => JSON.stringify(queued)), draft);
    await page.unroute('**/api/advance');
    await page.reload();
    await page.locator('#continueBtn').waitFor({state:'visible'});
    assert.equal((await state()).date, committed.date);
    await page.locator('#continueBtn').click();
    await page.locator('#app').waitFor({state:'visible'});
    await page.locator('#retryAdvanceBtn').click();
    await settled();
    await page.locator('#pendingTurn').waitFor({state:'hidden'});
    assert.equal((await state()).date, committed.date, 'Receipt retry advanced twice');
    assert.equal(await page.evaluate(() => queued.length), 0);

    // Save errors are visible and do not change the active campaign.
    await page.route('**/api/save', route => route.fulfill({status:500,
      contentType:'application/json',body:JSON.stringify({error:'Save failure fixture'})}));
    await more();
    await page.locator('#saveBtn').click();
    await page.waitForFunction(() => document.querySelector('#banner').textContent.includes('Save failure fixture'));
    await page.unroute('**/api/save');
    const saved = page.waitForResponse(r => r.url().endsWith('/api/save'));
    await page.locator('#saveBtn').click(); assert((await saved).ok());
    const savedWorld = await state();
    await page.locator('[data-step="1"]').click(); await settled();
    assert.notEqual((await state()).date, savedWorld.date);
    await more();
    await page.locator('#campaignsBtn').click();
    await page.locator('#loadBtn').click();
    await page.locator('#app').waitFor({state:'visible'});
    const loaded = await state();
    assert.equal(loaded.date, savedWorld.date);
    assert.notEqual(loaded.session_id, savedWorld.session_id);
    assert.equal(loaded.player, savedWorld.player);
    assert.equal(await page.locator('.arc-time-menu').getAttribute('open'), null,
      'Background More menu must close when campaigns are replaced');
    // History failure after a committed turn is a display problem, not a reason
    // to replay its command batch or keep a pending receipt.
    await page.route('**/api/history', route => route.fulfill({status:500,body:'history unavailable'}));
    await page.locator('[data-step="1"]').click(); await settled();
    assert.notEqual((await state()).date, loaded.date);
    assert.equal(await page.evaluate(() => pendingAdvance), null);
    await page.unroute('**/api/history');

    // Arcade rooms remain readable and usable at desktop and phone widths.
    for (const width of [1440,414]) {
      await page.setViewportSize({width,height:1000});
      for (const [name,button,panel] of [
        ['command','#dominationDockBtn','#dominationScreen'],
        ['cabinet','[data-drawer="cabinetDrawer"]','#cabinetDrawer'],
        ['resources','#stockBtn','#stockScreen'],
        ['research','#techBtn','#techMenu'],
        ['production','#productionDockBtn','#productionPanel'],
        ['freight','#logisticsDockBtn','#logisticsPanel'],
      ]) {
        await page.locator(button).click(); await page.locator(panel).waitFor({state:'visible'});
        const rect = await page.locator(panel).boundingBox();
        assert(rect && rect.x >= -1 && rect.x + rect.width <= width + 1, `${name} overflow at ${width}`);
        assert(!/\bNaN\b|\bundefined\b/.test(await page.locator(panel).innerText()), `${name} invalid label`);
        if (screenshots && ['cabinet','command'].includes(name)) await page.screenshot({path:path.join(screenshots,`${name}-${width}.png`)});
        await page.keyboard.press('Escape');
      }
    }
    assert.deepEqual(errors, [], 'No unhandled browser errors');
    console.log(JSON.stringify({ok:true,firstDate:initial.date,loadedDate:loaded.date,
      requestRetryDate:committed.date,viewports:[1440,414],browserErrors:errors}));
  } finally { await browser.close(); }
})().catch(error => {console.error(error);process.exitCode=1;});
