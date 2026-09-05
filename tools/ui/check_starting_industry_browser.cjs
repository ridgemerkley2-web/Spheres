// Replaces a disposable local campaign. Never run against a user's live game.
// NODE_PATH=<Playwright modules> PLAYWRIGHT_CHANNEL=msedge node this-file URL --disposable [screenshots]
const assert = require('node:assert/strict');
const { chromium } = require('playwright');
const fs = require('node:fs');
const path = require('node:path');
(async () => {
  const base = new URL(process.argv[2]);
  assert(['localhost', '127.0.0.1'].includes(base.hostname));
  assert.equal(process.argv[3], '--disposable');
  const shots = process.argv[4];
  if (shots) fs.mkdirSync(shots, {recursive:true});
  const browser = await chromium.launch({headless:true,
    ...(process.env.PLAYWRIGHT_CHANNEL ? {channel:process.env.PLAYWRIGHT_CHANNEL} : {})});
  try {
    const page = await browser.newPage({viewport:{width:1440,height:1000}, reducedMotion:'reduce'});
    page.setDefaultTimeout(30000);
    const errors = [];
    page.on('pageerror', e => errors.push(e.message));
    page.on('dialog', d => d.accept());
    await page.goto(base.href);
    await page.waitForFunction(() => !!SESSION.live);
    if (await page.locator('#newCampaignPicker').isHidden()) await page.locator('#newCampaignBtn').click();
    await page.locator('#nationPick [aria-label^="France;"]').click();
    await page.locator('#startBtn').click();
    await page.locator('#app').waitFor({state:'visible'});
    await page.locator('#competitionDockBtn').click();
    await page.locator('[data-comp-action="enable"]').click();
    await page.waitForFunction(() => COMP.data?.enabled && !COMP.busy && !COMP.loading);
    assert.equal(await page.locator('.comp-inherited .pe-inherited-group').count(), 5);
    assert(await page.locator('.comp-inherited').innerText().then(t => t.includes('stockpile')));
    for (const [width,height] of [[1440,1000],[820,800],[390,844]]) {
      await page.setViewportSize({width,height});
      await page.locator('.comp-inherited').scrollIntoViewIfNeeded();
      for (const card of await page.locator('.comp-inherited .pe-inherited-group').all()) {
        assert(!await card.evaluate(e => e.scrollWidth > e.clientWidth + 1), `inherited card overflow ${width}`);
      }
      assert(!await page.locator('#competitionRoom').evaluate(e => e.scrollWidth > e.clientWidth + 1));
      if (shots) await page.screenshot({path:path.join(shots, `inherited-industry-${width}.png`)});
    }
    // Authoritative all-country dossier API, including an unmapped and a small country.
    for (const nation of ['USA','Japan','Tonga','Bahrain']) {
      const response = await page.request.get(new URL(`/api/economic-ledger/${nation}`,base).href);
      assert(response.ok());
      const data = await response.json();
      assert.equal(data.starting_industry.groups.length, 5);
      assert(data.starting_industry.factory_equivalents > 0);
      const sum = data.provinces.reduce((s,p) => s+p.total_gdp_bn, data.unallocated_gdp_bn);
      assert(Math.abs(sum-data.total_gdp_bn) < 1e-7, `${nation} GDP reconciliation`);
      if (nation === 'Bahrain') assert(data.starting_industry.unallocated_factory_equivalents > 0);
      if (nation === 'Tonga') assert(data.starting_industry.factory_equivalents < 1);
    }
    await page.locator('[data-comp-action="budget"]').first().click();
    await page.locator('#cab-tab-budget').click();
    await page.locator('[data-cab-ministry="pensions"]').click();
    const inspector = page.locator('#cabinetInspector');
    assert.equal(await inspector.getAttribute('aria-label'), 'Welfare funding');
    assert(await inspector.getByRole('heading', {name:'Welfare',exact:true}).isVisible());
    assert.equal(await inspector.locator('.arm-line[title*="not job creation"]').count(), 1);
    for (const [width,height] of [[1440,1000],[390,844]]) {
      await page.setViewportSize({width,height});
      await inspector.scrollIntoViewIfNeeded();
      assert(!await inspector.evaluate(e => e.scrollWidth > e.clientWidth+1), `Welfare overflow ${width}`);
      if (shots) await page.screenshot({path:path.join(shots, `welfare-${width}.png`)});
    }
    assert.deepEqual(errors, []);
    console.log('Inherited industry + Welfare: desktop/tablet/mobile, five groups, country coverage, GDP reconciliation and no page errors passed.');
  } finally { await browser.close(); }
})().catch(error => { console.error(error); process.exitCode=1; });
