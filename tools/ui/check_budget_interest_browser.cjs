// Disposable native/browser regression for the debt-rate explanation.
// NODE_PATH=<Playwright modules> SPHERES_BROWSER_CHANNEL=msedge node this-file BINARY FULL_REVISION NEW_OUTPUT
const assert = require('node:assert/strict');
const fs = require('node:fs');
const path = require('node:path');
const net = require('node:net');
const cp = require('node:child_process');
const crypto = require('node:crypto');
const {chromium} = require('playwright');
const {verifyBuild} = require('./ci-integrated.cjs');
const root = path.resolve(__dirname, '../..');
const hash = file => crypto.createHash('sha256').update(fs.readFileSync(file)).digest('hex');
async function freePort() {
  const server = net.createServer();
  await new Promise(resolve => server.listen(0, '127.0.0.1', resolve));
  const port = server.address().port;
  await new Promise(resolve => server.close(resolve));
  return port;
}
async function main() {
  const [binaryArg, revision, outputArg] = process.argv.slice(2);
  assert(binaryArg && outputArg && /^[a-f0-9]{40}$/.test(revision), 'Supply binary, full revision and new output directory');
  const binary = path.resolve(binaryArg), out = path.resolve(outputArg);
  assert(!fs.existsSync(out), 'Keep prior evidence; output directory must be new');
  const run = path.join(out, 'disposable-campaign');
  fs.mkdirSync(run, {recursive:true});
  const port = await freePort(), url = 'http://127.0.0.1:' + port;
  const proof = {scope:'Ordinary Brazil/France first-budget display regression; not S24 qualification',
    revision, binary_sha256:hash(binary), started_utc:new Date().toISOString(), cases:[], errors:[], passed:false};
  const server = cp.spawn(binary, ['--port', String(port), '--no-open'],
    {cwd:run, windowsHide:true, stdio:['ignore','pipe','pipe']});
  const log = fs.createWriteStream(path.join(out, 'server.log'));
  server.stdout.pipe(log); server.stderr.pipe(log);
  let browser, page;
  try {
    const deadline = Date.now() + 30000;
    for (;;) {
      try {
        const response = await fetch(url+'/api/build', {signal:AbortSignal.timeout(2000)});
        await response.arrayBuffer();
        if (response.ok) break;
      } catch (error) {
        if (Date.now() >= deadline) throw error;
      }
      assert(Date.now() < deadline, 'Disposable server startup timed out');
      await new Promise(resolve => setTimeout(resolve, 100));
    }
    browser = await chromium.launch({headless:true, channel:process.env.SPHERES_BROWSER_CHANNEL || 'msedge'});
    page = await browser.newPage({viewport:{width:1440,height:1000}, reducedMotion:'reduce'});
    page.setDefaultTimeout(30000);
    page.on('pageerror', error => proof.errors.push(error.message));
    process.env.SPHERES_EXPECTED_REVISION = revision;
    proof.build = await verifyBuild({page,url,root,run,binary});
    for (const nation of ['Brazil','France']) {
      await page.goto(url, {waitUntil:'domcontentloaded'});
      await page.waitForFunction(() => !!SESSION.live?.session_id);
      await page.locator('#newCampaignBtn').click();
      await page.locator('#nationPick [aria-label^="'+nation+';"]').click();
      const replacing = await page.evaluate(() => !!SESSION.live?.player);
      await page.locator('#startBtn').click();
      if (replacing) await page.locator('#campaignConfirmAccept').click();
      await page.locator('#app').waitFor({state:'visible'});
      await page.locator('[data-drawer="cabinetDrawer"]').click();
      await page.locator('#cab-tab-budget').click();
      const advanced = page.waitForResponse(r => new URL(r.url()).pathname === '/api/advance' && r.request().method() === 'POST');
      await page.locator('#cabinetEnact').click();
      assert((await advanced).ok());
      await page.waitForFunction(() => !advancing && !CAB.busy && S.policy?.money?.on_the_books);
      const money = await page.evaluate(() => S.policy.money);
      assert(Math.abs(money.real_rate + money.real_rate_floor_adjustment + money.spread - money.effective_rate) < 2e-6);
      assert.equal(money.real_rate_after_floor, Math.max(money.real_rate, -0.02));
      if (nation === 'Brazil') assert(money.real_rate_floor_adjustment > 0, 'Brazil must exercise the real floor');
      if (nation === 'France') assert.equal(money.real_rate_floor_adjustment, 0);
      const views = [];
      for (const width of [1440,390]) {
        await page.setViewportSize({width,height:width===390 ? 844 : 1000});
        await page.locator('#cab-tab-budget').click();
        const row = page.locator('#cabinet-budget .interest-row');
        await row.scrollIntoViewIfNeeded();
        const text = await row.innerText();
        assert.equal(/debt-rate floor/i.test(text), money.real_rate_floor_adjustment > 0);
        assert.equal(/no sovereign spread/i.test(text), money.spread <= 5e-7);
        assert(!/NaN|undefined/.test(text));
        assert(!await row.evaluate(e => e.scrollWidth > e.clientWidth + 1), 'Interest row horizontal overflow');
        const shot = nation.toLowerCase()+'-'+width+'.png';
        await row.screenshot({path:path.join(out,shot)});
        views.push({width,text,screenshot:shot,sha256:hash(path.join(out,shot))});
      }
      proof.cases.push({nation,money,views});
    }
    assert.deepEqual(proof.errors, []);
    assert.equal(hash(binary), proof.binary_sha256);
    proof.passed = true;
  } catch (error) {
    proof.failure = String(error.stack || error);
    if (page) await page.screenshot({path:path.join(out,'failure.png')}).catch(()=>{});
    throw error;
  } finally {
    proof.finished_utc = new Date().toISOString();
    fs.writeFileSync(path.join(out,'result.json'), JSON.stringify(proof,null,2)+'\n');
    if (browser) await browser.close();
    server.kill();
    log.end();
  }
  console.log(JSON.stringify({passed:true,result:path.join(out,'result.json')}));
}
main().catch(error => {console.error(error); process.exitCode=1;});
