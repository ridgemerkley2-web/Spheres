// Run only against a disposable local game. This creates a France campaign;
// it never calls /api/save or touches an existing save file.
// NODE_PATH must include an installation of Playwright.
// node tools/logistics/check_browser.cjs http://127.0.0.1:7777 --allow-new-game [screenshot-dir]
const assert = require('node:assert/strict');
const path = require('node:path');
const fs = require('node:fs');
const { chromium } = require('playwright');

(async () => {
  const url = new URL(process.argv[2] || 'http://127.0.0.1:7777');
  assert(['127.0.0.1', 'localhost'].includes(url.hostname), 'Use an isolated localhost server');
  assert.equal(process.argv[3], '--allow-new-game', 'Explicit permission to create a disposable campaign is required');
  const screenshotDir = process.argv[4];
  if (screenshotDir) fs.mkdirSync(screenshotDir, { recursive: true });
  const browser = await chromium.launch({ headless: true,
    ...(process.env.PLAYWRIGHT_CHANNEL ? { channel: process.env.PLAYWRIGHT_CHANNEL } : {}) });
  try {
    const page = await browser.newPage({ viewport: { width: 1440, height: 1000 } });
    const errors = [];
    page.on('pageerror', error => errors.push(error.message));
    await page.goto(url.href);
    await page.locator('#nationPick [aria-label^="France;"]').click();
    await page.locator('#startBtn').click();
    await page.locator('#logisticsDockBtn').waitFor({ state: 'visible' });
    const before = await (await page.request.get(new URL('/api/state', url).href)).json();
    assert.equal(before.simulation_cadence, 'daily');
    const firstDay = page.waitForResponse(response => response.url().endsWith('/api/advance'));
    await page.locator('[data-adv="1"]').click();
    const after = await (await firstDay).json();
    await page.waitForFunction(() => !document.querySelector('[data-adv="1"]').disabled);
    assert.equal(after.day, 2);
    const nation = state => state.nations.find(n => n.id === state.player);
    assert.notEqual(nation(before).gdp, nation(after).gdp, 'GDP must change before month-end');
    assert.notEqual(nation(before).population, nation(after).population, 'Population must accrue daily');
    const history = await (await page.request.get(new URL('/api/history', url).href)).json();
    assert.equal(history.labels.at(-1), '2 Jan 1990');
    assert(after.resources.rows.some(r => r.unit.endsWith('/day') || r.unit.endsWith('/d')));
    await page.locator('#stockBtn').click();
    await page.locator('#stockScreen').waitFor({state:'visible'});
    assert(!/NaN|undefined/.test(await page.locator('#stockScreen').innerText()));
    if (screenshotDir) await page.screenshot({path:path.join(screenshotDir,'daily-resources.png')});
    await page.locator('#stockScreen button.x').click();
    await page.locator('#techBtn').click();
    await page.locator('#techMenu').waitFor({state:'visible'});
    assert((await page.locator('#techMenu').innerText()).includes('points a day'));
    await page.keyboard.press('Escape');
    await page.locator('#productionDockBtn').click();
    await page.locator('#productionManufactureTab').click();
    await page.locator('.manufacturing-hero').waitFor();
    assert(/per day/i.test(await page.locator('.manufacturing-hero').innerText()));
    if (screenshotDir) await page.screenshot({path:path.join(screenshotDir,'daily-manufacturing.png')});
    await page.locator('#productionClose').click();
    for (let month = 0; month < 2; month++) {
      const advanced = page.waitForResponse(response => response.url().endsWith('/api/advance'));
      await page.locator('[data-adv="30"]').click();
      assert((await advanced).ok());
      await page.waitForFunction(() => !document.querySelector('[data-adv="30"]').disabled);
    }
    await page.locator('#logisticsDockBtn').click();
    await page.locator('[data-logi-policy="land_only"]').waitFor();
    await page.locator('[data-logi-policy="land_only"]').click();
    await page.waitForFunction(() => document.querySelector('[data-logi-policy="land_only"]')?.getAttribute('aria-pressed') === 'true');
    const manifest = await (await page.request.get(new URL('/api/logistics', url).href)).json();
    assert.equal(manifest.physical, true);
    assert.equal(manifest.cadence, 'daily');
    assert(manifest.settled_day && manifest.settled_day.day > 0);
    assert.equal(manifest.policy.selected, 'land_only');
    assert(manifest.cargo.length > 0, 'The real market should create cargo for this campaign');
    assert(manifest.cargo.every(c => c.route.nodes.length > 1 && c.route.distance_km > 0));
    assert(manifest.cargo.every(c => ['in_transit', 'held'].includes(c.state)));
    assert(manifest.cargo.every(c => c.due_day && c.dispatched_day));
    assert(manifest.lanes.every(l => !l.unit.endsWith('/mo') && l.capacity_period === 'day'));
    assert(manifest.arrivals.every(c => c.state === 'arrived' && c.arrived_month));
    assert((await page.locator('#logisticsBody').innerText()).toLowerCase().includes('freight network'));
    const bounds = await page.locator('#logisticsPanel').boundingBox();
    if (screenshotDir) await page.screenshot({ path: path.join(screenshotDir, 'logistics-desktop.png') });
    const frame = await page.evaluate(() => ['body', '#app', 'main', '#center'].map(selector => {
      const e = document.querySelector(selector), r = e.getBoundingClientRect();
      return { selector, x: r.x, width: r.width, scrollLeft: e.scrollLeft, scrollWidth: e.scrollWidth, overflow: getComputedStyle(e).overflow };
    }));
    assert(bounds && bounds.x >= 0 && bounds.y >= 0 && bounds.x + bounds.width <= 1441 && bounds.y + bounds.height <= 1001, JSON.stringify({ bounds, frame }));
    const cargo = page.locator('[data-logi-lane^="cargo:"]').first();
    await cargo.click();
    await page.waitForFunction(() => !document.querySelector('main').scrollLeft);
    // Let the camera reach the route before capturing its selected label.
    await page.waitForTimeout(400);
    if (screenshotDir) await page.screenshot({ path: path.join(screenshotDir, 'logistics-desktop.png') });
    const more = page.locator('[data-logi-cargo-more]');
    if (await more.count()) {
      await more.click();
      assert((await page.locator('[data-logi-lane^="cargo:"]').count()) >= 3);
      await more.click();
    }
    await page.locator('[data-logi-policy="fastest"]').click();
    await page.waitForFunction(() => document.querySelector('[data-logi-policy="fastest"]')?.getAttribute('aria-pressed') === 'true');
    await page.setViewportSize({ width: 820, height: 900 });
    await page.locator('#logisticsPanel').waitFor();
    const narrow = await page.locator('#logisticsPanel').boundingBox();
    assert(narrow && narrow.x >= 0 && narrow.x + narrow.width <= 821 && narrow.y + narrow.height <= 901);
    if (screenshotDir) await page.screenshot({ path: path.join(screenshotDir, 'logistics-narrow.png') });
    await page.locator('[data-logi-build]').click();
    await page.locator('#productionPanel.open').waitFor();
    assert.deepEqual(errors, [], 'The logistics screen must not throw browser errors');
    console.log(JSON.stringify({ ok: true, cargo: manifest.cargo.length, arrivals: manifest.arrivals.length,
      lanes: manifest.lanes.length, daily: after.simulation_cadence, firstDay:after.date,
      screenshots: screenshotDir || null, browserErrors: errors }));
  } finally {
    await browser.close();
  }
})().catch(error => { console.error(error); process.exitCode = 1; });
