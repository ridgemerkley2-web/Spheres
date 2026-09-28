/* CLAUDE-C03-REVIEW-01: checks for the cartoon review workbench (tools/ui/cartoon-review/).
 *
 * The pure-function and export-contract tests always run, so this file stays a
 * serverless member of `node tools/ui/run-unit.cjs`. The real-browser journey runs
 * only when SPHERES_BROWSER_CHANNEL is set (for example `chrome`) and Playwright
 * resolves (NODE_PATH to a pinned 1.58.2 install). It serves this repository from an
 * in-process static server on 127.0.0.1 with an OS-assigned port, drives the real
 * reviewer and writes screenshots plus result.json only when
 * CARTOON_REVIEW_EVIDENCE_DIR names a directory.
 *
 *   node --test tools/ui/check_cartoon_review.cjs
 *   SPHERES_BROWSER_CHANNEL=chrome NODE_PATH=<playwright node_modules> node --test tools/ui/check_cartoon_review.cjs
 */
'use strict';
const {test, before, after} = require('node:test');
const assert = require('node:assert/strict');
const fs = require('node:fs');
const path = require('node:path');
const http = require('node:http');
const crypto = require('node:crypto');
const {execFileSync} = require('node:child_process');
const review = require('./cartoon-review/cartoon-review.js');

const root = path.resolve(__dirname, '../..');
const sha = (bytes) => crypto.createHash('sha256').update(bytes).digest('hex');
const readExport = () => JSON.parse(fs.readFileSync(path.join(root, review.EXPORT_PATH), 'utf8'));

// ------------------------------------------------------------ pure functions

const sample = [
  {id: 'historical:tupou#0', collection: 'historical', name: "Taufa'ahau Tupou IV", countries: ['Tonga'],
    identity: {key: 'person:tupou', person_id: 'tupou'}, interval: {from: '1990-01-01', to: '1991-01-01', valid: true}, labels: ['historical', 'sample-reviewed']},
  {id: 'fictional:emi#0', collection: 'fictional', name: 'Nakano Emi', countries: ['Japan'],
    identity: {key: 'fictional:emi'}, interval: {from: '2026-09-08', to: '2036-01-01', valid: true}, labels: ['fictional']},
  {id: 'selector:Tonga', collection: 'selector', name: 'Sālote Tupou III', countries: ['Tonga'], identity: {key: 'figure:Q1'}, labels: ['selector']},
  {id: 'missing:carol', collection: 'missing', name: 'Carol Missing', countries: ['Tonga'],
    identity: {key: 'person:carol'}, required_windows: [{from: '1990-01-01', to: '1990-01-02'}], labels: ['missing-art']},
  {id: 'file:x.png', collection: 'unregistered', name: 'x.png', countries: [], identity: {key: null, status: 'unbound'}, labels: ['unregistered', 'unknown-identity', 'country-unknown']},
  {id: 'historical:bad#0', collection: 'historical', name: 'Bad Interval', countries: ['Japan'],
    identity: {key: 'person:bad'}, interval: {from: '1995-01-01', to: '1990-01-01', valid: false}, labels: ['historical', 'interval-issue']},
];
const ids = (list) => list.map((i) => i.id);
const nations = {Tonga: 'Tonga', Japan: 'Japan'};

test('collection, country, label and search filters keep missing, fictional and unknown items distinct', () => {
  assert.deepEqual(ids(review.filterItems(sample, {collection: 'cartoons'}, nations)), ['historical:tupou#0', 'fictional:emi#0', 'selector:Tonga', 'historical:bad#0']);
  assert.deepEqual(ids(review.filterItems(sample, {collection: 'missing'}, nations)), ['missing:carol']);
  assert.deepEqual(ids(review.filterItems(sample, {country: 'Tonga'}, nations)), ['historical:tupou#0', 'selector:Tonga', 'missing:carol']);
  assert.deepEqual(ids(review.filterItems(sample, {country: 'unknown'}, nations)), ['file:x.png']);
  assert.deepEqual(ids(review.filterItems(sample, {label: 'unknown-identity'}, nations)), ['file:x.png']);
  assert.deepEqual(ids(review.filterItems(sample, {q: 'salote'}, nations)), ['selector:Tonga'], 'search ignores diacritics');
  assert.deepEqual(ids(review.filterItems(sample, {q: 'tupou tonga'}, nations)), ['historical:tupou#0', 'selector:Tonga'], 'all terms must match');
  assert.deepEqual(ids(review.filterItems(sample, {q: 'not a real person'}, nations)), ['fictional:emi#0'], 'labels are searchable by their visible text');
  assert.deepEqual(review.filterItems(sample, {}, nations).length, sample.length);
});

test('era and date filters use half-open intervals; invalid and undated items are not dated', () => {
  assert.deepEqual(ids(review.filterItems(sample, {era: '1990s'}, nations)), ['historical:tupou#0', 'missing:carol']);
  assert.deepEqual(ids(review.filterItems(sample, {era: 'fictional'}, nations)), ['fictional:emi#0']);
  assert.deepEqual(ids(review.filterItems(sample, {era: 'undated'}, nations)), ['selector:Tonga', 'file:x.png', 'historical:bad#0']);
  assert.deepEqual(ids(review.filterItems(sample, {date: '1990-01-01'}, nations)), ['historical:tupou#0', 'missing:carol']);
  assert.deepEqual(ids(review.filterItems(sample, {date: '1990-01-02'}, nations)), ['historical:tupou#0'], 'the window end is excluded');
  assert.deepEqual(ids(review.filterItems(sample, {date: '1991-01-01'}, nations)), []);
  assert.throws(() => review.filterItems(sample, {era: '1980s'}, nations), /Unknown era/);
  assert.throws(() => review.filterItems(sample, {date: '01/02/1990'}, nations), /YYYY-MM-DD/);
  assert.equal(review.formatInterval({from: '1990-01-01', to: '1991-01-01'}), '1990-01-01 → before 1991-01-01');
  assert.equal(review.formatInterval({from: '1990-01-01', to: '9999-12-31'}), '1990-01-01 → open end');
  assert.equal(review.formatInterval(null), 'No dated appearance');
});

test('keyboard grid movement clamps inside the visible sheet', () => {
  assert.equal(review.moveIndex(0, 'ArrowRight', 4, 10), 1);
  assert.equal(review.moveIndex(1, 'ArrowDown', 4, 10), 5);
  assert.equal(review.moveIndex(5, 'ArrowUp', 4, 10), 1);
  assert.equal(review.moveIndex(0, 'ArrowLeft', 4, 10), 0);
  assert.equal(review.moveIndex(8, 'ArrowDown', 4, 10), 9, 'the last row clamps to the last card');
  assert.equal(review.moveIndex(3, 'End', 4, 10), 9);
  assert.equal(review.moveIndex(9, 'Home', 4, 10), 0);
  assert.equal(review.moveIndex(0, 'PageDown', 4, 10), 9);
  assert.equal(review.moveIndex(4, 'q', 4, 10), 4, 'other keys leave focus alone');
  assert.equal(review.moveIndex(0, 'ArrowRight', 4, 0), -1);
  assert.equal(review.columnsFromTops([10, 10, 10, 200, 200]), 3);
  assert.equal(review.columnsFromTops([]), 1);
});

test('paths and links stay inside the repository and never become script or credential URLs', () => {
  assert.equal(review.safePath('spheres-web/ui/person-portraits/a-v1.png'), 'spheres-web/ui/person-portraits/a-v1.png');
  for (const bad of ['../secret.png', 'spheres-web/ui/../../x.png', '/spheres-web/ui/a.png', 'spheres-web\\ui\\a.png',
    'C:/x.png', 'spheres-web/ui/%2e%2e/a.png', 'src/main.rs', 'spheres-web//ui/a.png', 'spheres-web/ui/a.png?x=1', 42, null]) {
    assert.equal(review.safePath(bad), null, String(bad));
  }
  assert.equal(review.publicURL('https://example.org/a'), 'https://example.org/a');
  for (const bad of ['http://example.org', 'javascript:alert(1)', 'https://u:p@example.org', 'file:///c:/x', 'not a url']) assert.equal(review.publicURL(bad), null, bad);
  const doc = {people: {'a/b': {portraits: [{x: 1}]}, 'c~d': 2}};
  assert.deepEqual(review.resolvePointer(doc, '/people/a~1b/portraits/0'), {x: 1});
  assert.equal(review.resolvePointer(doc, '/people/c~0d'), 2);
  assert.equal(review.resolvePointer(doc, '/people/zzz'), undefined);
  assert.equal(review.resolvePointer(doc, 'people'), undefined);
  assert.equal(review.initials('Sālote Tupou III'), 'SI');
  assert.equal(review.reasonText(['sourced_party_term', 'Tonga', null]), 'sourced party term · Tonga');
  assert.equal(review.sizesFor({collection: 'selector'})[0].label, 'Selector pick card');
  assert.deepEqual(review.sizesFor({collection: 'historical'}).map((s) => [s.width, s.height]), [[106, 152], [90, 132], [156, 218]]);
});

test('input verification compares exact bytes and fails closed', async () => {
  const bytes = Buffer.from('{"version":1}\n');
  const input = {role: 'x', path: 'spheres-web/data/x.json', bytes: bytes.length, sha256: sha(bytes)};
  const fetcher = async (url) => {
    assert.equal(url, 'http://127.0.0.1:1/spheres-web/data/x.json');
    return {ok: true, arrayBuffer: async () => bytes.buffer.slice(bytes.byteOffset, bytes.byteOffset + bytes.length)};
  };
  const [ok] = await review.verifyInputs([input], fetcher, crypto.webcrypto, 'http://127.0.0.1:1/');
  assert.equal(ok.matches, true);
  const [changed] = await review.verifyInputs([{...input, sha256: '0'.repeat(64)}], fetcher, crypto.webcrypto, 'http://127.0.0.1:1/');
  assert.equal(changed.matches, false);
  await assert.rejects(review.verifyInputs([{...input, path: '../x.json'}], fetcher, crypto.webcrypto, 'http://127.0.0.1:1/'), /unsafe input path/);
  await assert.rejects(review.verifyInputs([input], fetcher, {}, 'http://127.0.0.1:1/'), /secure context/);
  await assert.rejects(review.verifyInputs([input], async () => ({ok: false, status: 404}), crypto.webcrypto, 'http://127.0.0.1:1/'), /unavailable \(404\)/);
  await assert.rejects(review.verifyInputs([], fetcher, crypto.webcrypto, 'http://127.0.0.1:1/'), /no pinned inputs/);
  const next = review.latest();
  const first = next();
  const second = next();
  assert.equal(first(), false, 'a superseded full-size load cannot replace the latest one');
  assert.equal(second(), true);
});

test('explicit LF text scope survives CRLF, preserves other bytes and rejects content changes', async () => {
  const lf = Buffer.from('\ufeff{\n"name":"Sālote", "note":"escaped\\r\\n"\n}\n');
  const crlf = Buffer.from(lf.toString('utf8').replace(/\n/g, '\r\n'));
  const input = {role: 'x', path: 'spheres-web/data/x.json', hash_scope: 'utf8-lf', bytes: lf.length, sha256: sha(lf)};
  const fetcher = bytes => async () => ({ok: true, arrayBuffer: async () => bytes.buffer.slice(bytes.byteOffset, bytes.byteOffset + bytes.length)});
  for (const bytes of [lf, crlf]) {
    const [result] = await review.verifyInputs([input], fetcher(bytes), crypto.webcrypto, 'http://127.0.0.1/');
    assert.equal(result.matches, true);
    assert.equal(result.raw_bytes, bytes.length);
    assert.equal(result.raw_sha256, sha(bytes));
  }
  const [rawMismatch] = await review.verifyInputs([{...input, hash_scope: 'raw'}], fetcher(crlf), crypto.webcrypto, 'http://127.0.0.1/');
  assert.equal(rawMismatch.matches, false);
  for (const changed of [Buffer.from(lf.toString('utf8').replace('Sālote', 'Other')), Buffer.concat([lf, Buffer.from('\r')])]) {
    assert.equal((await review.verifyInputs([input], fetcher(changed), crypto.webcrypto, 'http://127.0.0.1/'))[0].matches, false);
  }
  await assert.rejects(review.verifyInputs([{...input, hash_scope: 'unknown'}], fetcher(lf), crypto.webcrypto, 'http://127.0.0.1/'), /Unsupported input hash scope/);
});

test('the committed export keeps automated findings apart from approval and labels every collection', () => {
  const data = readExport();
  assert.equal(data.format, 'spheres-c03-cartoon-review/v1');
  assert.match(data.statement, /not visual approval/);
  assert.equal(data.summary.approved_by_this_export, 0);
  assert.deepEqual(Object.keys(data.collections).sort(), ['fictional', 'historical', 'missing', 'selector', 'unregistered']);
  const files = new Map(data.files.map((f) => [f.path, f]));
  for (const item of data.items) {
    assert.ok(item.labels.length, item.id);
    assert.equal(item.labels[0], item.collection === 'missing' ? 'missing-art' : item.collection, item.id);
    for (const key of ['asset', 'card']) if (item[key]) assert.ok(review.safePath(item[key]) && files.has(item[key]), `${item.id} ${key}`);
    if (item.collection === 'missing') assert.equal(item.asset, undefined, item.id);
    if (item.collection === 'unregistered') assert.ok(item.labels.includes('unknown-identity'), item.id);
    if (!(item.countries || []).length) assert.ok(item.labels.includes('country-unknown'), item.id);
  }
  for (const input of data.inputs) assert.ok(review.safePath(input.path) && /^[0-9a-f]{64}$/.test(input.sha256), input.path);
  const entries = data.visual_review_sample.entries;
  assert.ok(entries.length >= 6 && entries.length <= 8);
  for (const entry of entries) {
    assert.ok(['fixes_proposed', 'reference_check_required'].includes(entry.decision), entry.item);
    assert.ok(entry.fixes.length > 0, entry.item);
    assert.ok(data.items.some((i) => i.id === entry.item && i.labels.includes('sample-reviewed')), entry.item);
  }
});

// ------------------------------------------------------------ real browser journey

const channel = process.env.SPHERES_BROWSER_CHANNEL;
const evidenceDir = process.env.CARTOON_REVIEW_EVIDENCE_DIR ? path.resolve(root, process.env.CARTOON_REVIEW_EVIDENCE_DIR) : null;
const skipBrowser = channel ? false : 'set SPHERES_BROWSER_CHANNEL=chrome (and NODE_PATH to a Playwright 1.58.2 install) to drive the real reviewer';
const SERVED = ['tools/ui/cartoon-review/', 'docs/campaign-certification/C03/preparation/cartoon-review/', 'docs/campaign-certification/S10/c/',
  'spheres-web/ui/', 'spheres-web/data/', 'spheres-sim/data/', 'tools/avatars/'];
const TYPES = {'.html': 'text/html; charset=utf-8', '.js': 'text/javascript; charset=utf-8', '.css': 'text/css; charset=utf-8',
  '.json': 'application/json; charset=utf-8', '.png': 'image/png', '.webp': 'image/webp', '.jpg': 'image/jpeg', '.jpeg': 'image/jpeg',
  '.md': 'text/plain; charset=utf-8', '.txt': 'text/plain; charset=utf-8'};
const env = {server: null, origin: null, browser: null, context: null, requests: [], failures: [], errors: [],
  proof: {format: 'spheres-c03-cartoon-review-browser/v1', task: 'CLAUDE-C03-REVIEW-01', checks: {}, screenshots: []}};

function serve() {
  const server = http.createServer((req, res) => {
    let name;
    try {
      name = decodeURIComponent(new URL(req.url, 'http://127.0.0.1').pathname).replace(/^\/+/, '');
    } catch (error) {
      res.writeHead(400);
      return res.end();
    }
    if (name === '' || name.endsWith('/')) name += 'index.html';
    const file = path.resolve(root, name);
    if (req.method !== 'GET' || !SERVED.some((p) => name.startsWith(p)) || !file.startsWith(root + path.sep) || !fs.existsSync(file) || !fs.statSync(file).isFile()) {
      res.writeHead(404);
      return res.end();
    }
    res.writeHead(200, {'Content-Type': TYPES[path.extname(file).toLowerCase()] || 'application/octet-stream', 'Cache-Control': 'no-store'});
    fs.createReadStream(file).pipe(res);
  });
  return new Promise((resolve) => server.listen(0, '127.0.0.1', () => resolve(server)));
}

async function ready(page) {
  await page.waitForFunction(() => ['ready', 'stale', 'error'].includes(document.body.dataset.state), null, {timeout: 60000});
  assert.equal(await page.evaluate(() => document.body.dataset.state), 'ready', await page.locator('#status').textContent());
}

async function visibleIds(page) {
  return page.$$eval('#sheet li:not([hidden]) .card', (cards) => cards.map((c) => c.dataset.id));
}

async function setFilter(page, id, value) {
  if (id === 'f-search' || id === 'f-date') await page.locator('#' + id).fill(value);
  else await page.locator('#' + id).selectOption(value);
}

async function toFilters(page) {
  await page.locator('#filters').evaluate((el) => el.scrollIntoView({block: 'start'}));
}

async function settle(page) {
  // Cards load lazily and decode asynchronously: wait until every image inside the
  // viewport has loaded and decoded, then let two frames paint before capturing.
  const inView = `(img) => { const r = img.getBoundingClientRect(); return r.width > 0 && r.bottom > 0 && r.top < innerHeight && r.right > 0 && r.left < innerWidth; }`;
  await page.waitForFunction((test) => [...document.querySelectorAll('img')].filter(eval(test)).every((img) => img.complete && img.naturalWidth > 0), inView, {timeout: 60000});
  await page.evaluate(async (test) => {
    await Promise.all([...document.querySelectorAll('img')].filter(eval(test)).map((img) => img.decode().catch(() => null)));
    await new Promise((resolve) => requestAnimationFrame(() => requestAnimationFrame(resolve)));
  }, inView);
}

async function shot(page, name) {
  if (!evidenceDir) return;
  await settle(page);
  const file = path.join(evidenceDir, name);
  await page.screenshot({path: file, type: 'jpeg', quality: 70});
  const bytes = fs.readFileSync(file);
  env.proof.screenshots.push({file: path.relative(root, file).split(path.sep).join('/'), viewport: page.viewportSize(), bytes: bytes.length, sha256: sha(bytes)});
}

async function noHorizontalOverflow(page) {
  const reading = await page.evaluate(() => {
    const viewport = document.documentElement.clientWidth;
    const visible = (n) => n.getClientRects().length && getComputedStyle(n).visibility !== 'hidden';
    const outside = [...document.querySelectorAll('header *, main *, .filters *, .refs *, #detail.open *')]
      .filter(visible).filter((n) => { const r = n.getBoundingClientRect(); return r.width && (r.left < -1 || r.right > viewport + 1); })
      .map((n) => n.tagName + (n.id ? '#' + n.id : '') + '.' + n.className);
    return {viewport, scrollWidth: document.documentElement.scrollWidth, outside: outside.slice(0, 5)};
  });
  assert.ok(reading.scrollWidth <= reading.viewport + 1, JSON.stringify(reading));
  assert.deepEqual(reading.outside, []);
  return reading;
}

before(async () => {
  if (skipBrowser) return;
  const {chromium} = require('playwright');
  env.server = await serve();
  const port = env.server.address().port;
  assert.notEqual(port, 7777, 'never use the shared 7777 port');
  env.origin = `http://127.0.0.1:${port}`;
  env.browser = await chromium.launch({channel, headless: true});
  env.context = await env.browser.newContext({viewport: {width: 1280, height: 900}, deviceScaleFactor: 1});
  env.context.on('request', (request) => env.requests.push({method: request.method(), url: request.url()}));
  env.context.on('response', (response) => { if (response.status() >= 400) env.failures.push(`${response.status()} ${response.url()}`); });
  if (evidenceDir) fs.mkdirSync(evidenceDir, {recursive: true});
  env.proof.started_utc = new Date().toISOString();
  env.proof.browser = {channel, version: env.browser.version(), headless: true};
  env.proof.origin = env.origin;
});

after(async () => {
  if (skipBrowser) return;
  if (env.browser) await env.browser.close();
  if (env.server) await new Promise((resolve) => env.server.close(resolve));
  if (!evidenceDir) return;
  const git = (...args) => { try { return execFileSync('git', args, {cwd: root, encoding: 'utf8'}).trim(); } catch (error) { return null; } };
  const exportBytes = fs.readFileSync(path.join(root, review.EXPORT_PATH));
  const sources = {};
  for (const p of ['tools/ui/cartoon-review/index.html', 'tools/ui/cartoon-review/cartoon-review.css', 'tools/ui/cartoon-review/cartoon-review.js',
    'tools/ui/check_cartoon_review.cjs', 'tools/avatars/cartoon_review.py']) sources[p] = sha(fs.readFileSync(path.join(root, p), 'utf8').replace(/\r\n/g, '\n'));
  const origins = [...new Set(env.requests.map((r) => new URL(r.url).origin))];
  Object.assign(env.proof, {
    finished_utc: new Date().toISOString(), revision: git('rev-parse', 'HEAD'),
    revision_note: 'HEAD when the journey ran; reviewer source hashes below are LF-normalized working-tree bytes.',
    export: {path: review.EXPORT_PATH, bytes: exportBytes.length, sha256: sha(exportBytes)},
    reviewer_sources_sha256_lf: sources,
    requests: {count: env.requests.length, origins, non_get: env.requests.filter((r) => r.method !== 'GET').length, http_errors: env.failures},
    page_errors: env.errors,
    scope: 'Static repository files only: no game server, API, saved campaign or campaign time. Screenshots are JPEG quality 70.',
  });
  fs.writeFileSync(path.join(evidenceDir, 'result.json'), JSON.stringify(env.proof, null, 2) + '\n');
});

test('real reviewer in a headless browser', {skip: skipBrowser}, async (t) => {
  const data = readExport();
  const page = await env.context.newPage();
  page.on('pageerror', (error) => env.errors.push(error.message));
  page.on('console', (message) => { if (message.type() === 'error') env.errors.push(message.text()); });
  const expected = (filters) => ids(review.filterItems(data.items, filters, data.nations));

  await t.test('verifies every pinned input by SHA-256 before showing the sheet', async () => {
    await page.goto(env.origin + '/tools/ui/cartoon-review/');
    await ready(page);
    const verification = await page.evaluate(() => window.CartoonReviewState.verification.map(({path, hash_scope, bytes, sha256, raw_bytes, raw_sha256, matches}) => ({path, hash_scope, bytes, sha256, raw_bytes, raw_sha256, matches})));
    assert.deepEqual(verification.map((v) => [v.path, v.bytes, v.sha256]), data.inputs.map((i) => [i.path, i.bytes, i.sha256]));
    assert.ok(verification.every((v) => v.matches));
    assert.match(await page.locator('#status').textContent(), new RegExp(`Export verified: ${data.inputs.length} pinned inputs match`));
    assert.equal(await page.locator('#sheet .card').count(), data.items.length);
    assert.deepEqual(await visibleIds(page), ids(data.items));
    assert.match(await page.locator('#summary').textContent(), /0Approved by this export/);
    env.proof.inputs_verified_in_browser = verification;
    env.proof.checks.load = {items: data.items.length, inputs_verified: verification.length};
  });

  await t.test('filters by country, collection, person, era, date and label', async () => {
    await setFilter(page, 'f-country', 'Tonga');
    const tonga = await visibleIds(page);
    assert.deepEqual(tonga, expected({country: 'Tonga'}));
    assert.ok(tonga.includes('historical:taufaahau_tupou_iv#0') && tonga.includes('selector:Tonga'));
    assert.ok((await page.$$eval('#sheet li:not([hidden]) .card', (cards) => cards.every((c) => c.dataset.countries.split(' ').includes('Tonga')))));
    await setFilter(page, 'f-date', '1990-06-01');
    assert.deepEqual(await visibleIds(page), expected({country: 'Tonga', date: '1990-06-01'}));
    assert.deepEqual(await visibleIds(page), ['historical:taufaahau_tupou_iv#0'], 'only the 1990 appearance is shown on 1 June 1990');
    await setFilter(page, 'f-date', '');
    await page.locator('#sheet .card[data-id="historical:taufaahau_tupou_iv#0"]').click();
    await page.locator('#detail .verify[data-verified="true"]').waitFor();
    await toFilters(page);
    await shot(page, 'desktop-tonga-filter-side-by-side.jpg');

    await page.locator('#filters button[type="reset"]').click();
    await page.waitForFunction((n) => document.querySelectorAll('#sheet li:not([hidden])').length === n, data.items.length);
    await setFilter(page, 'f-collection', 'fictional');
    const fictional = await visibleIds(page);
    assert.deepEqual(fictional, expected({collection: 'fictional'}));
    assert.equal(fictional.length, data.summary.items.fictional);
    for (const id of fictional) assert.match(await page.locator(`#sheet .card[data-id="${id}"] .chips`).textContent(), /Fictional · not a real person/);

    await setFilter(page, 'f-collection', 'missing');
    await setFilter(page, 'f-country', 'France');
    const missing = await visibleIds(page);
    assert.deepEqual(missing, expected({collection: 'missing', country: 'France'}));
    assert.ok(missing.length > 0);
    assert.equal(await page.locator('#sheet li:not([hidden]) .card img').count(), 0, 'missing art never shows a substitute image');
    assert.equal(await page.locator('#sheet li:not([hidden]) .card .placeholder-text', {hasText: 'Artwork missing'}).count(), missing.length);
    await page.locator(`#sheet .card[data-id="${missing[0]}"]`).click();
    assert.match(await page.locator('#detail .full-holder').textContent(), /Artwork missing/);
    assert.equal(await page.locator('#detail img').count(), 0);
    assert.equal(await page.locator('#detail .verify').textContent(), 'Nothing to verify.');
    await toFilters(page);
    await shot(page, 'desktop-missing-art-france.jpg');

    await setFilter(page, 'f-collection', 'all');
    await setFilter(page, 'f-country', 'all');
    await setFilter(page, 'f-search', 'salote');
    assert.deepEqual(await visibleIds(page), expected({q: 'salote'}));
    assert.ok((await visibleIds(page)).includes('selector:Tonga'), 'search ignores diacritics');
    await setFilter(page, 'f-search', '');
    await setFilter(page, 'f-era', '2000s');
    const era = await visibleIds(page);
    assert.deepEqual(era, expected({era: '2000s'}));
    assert.ok(era.includes('historical:tony_blair#0') && era.includes('historical:gordon_brown#0'));
    await setFilter(page, 'f-era', 'all');
    await setFilter(page, 'f-label', 'unknown-identity');
    const unknown = await visibleIds(page);
    assert.deepEqual(unknown, expected({label: 'unknown-identity'}));
    assert.ok(unknown.length && unknown.every((id) => id.startsWith('file:')));
    for (const id of unknown) assert.match(await page.locator(`#sheet .card[data-id="${id}"] .chips`).textContent(), /Unregistered file.*Unknown identity/);
    await setFilter(page, 'f-label', 'all');
    env.proof.checks.filters = {tonga: tonga.length, tonga_on_1990_06_01: 1, fictional: fictional.length, france_missing_art: missing.length,
      search_salote: expected({q: 'salote'}).length, era_2000s: era.length, unknown_identity: unknown.length};
  });

  await t.test('keyboard: roving focus, arrows, Home/End, Enter and [ ]', async () => {
    await setFilter(page, 'f-collection', 'cartoons');
    const visible = await visibleIds(page);
    assert.deepEqual(visible, expected({collection: 'cartoons'}));
    assert.equal(await page.locator('#sheet .card[tabindex="0"]').count(), 1, 'exactly one card is in the tab order');
    await page.locator('#sheet .card[tabindex="0"]').focus();
    const active = () => page.evaluate(() => document.activeElement && document.activeElement.dataset.id);
    const start = await active();
    const index = visible.indexOf(start);
    const columns = await page.evaluate(() => {
      const tops = [...document.querySelectorAll('#sheet li:not([hidden])')].map((li) => li.offsetTop);
      return tops.filter((t) => Math.abs(t - tops[0]) < 2).length;
    });
    assert.ok(columns >= 2);
    await page.keyboard.press('Home');
    assert.equal(await active(), visible[0]);
    await page.keyboard.press('ArrowRight');
    assert.equal(await active(), visible[1]);
    await page.keyboard.press('ArrowDown');
    assert.equal(await active(), visible[1 + columns]);
    await page.keyboard.press('ArrowLeft');
    assert.equal(await active(), visible[columns]);
    await page.keyboard.press('ArrowUp');
    assert.equal(await active(), visible[0]);
    await page.keyboard.press('End');
    assert.equal(await active(), visible[visible.length - 1]);
    await page.keyboard.press('Home');
    const outline = await page.evaluate(() => getComputedStyle(document.activeElement).outlineWidth);
    assert.equal(outline, '3px', 'focus is visible');
    await page.keyboard.press('Enter');
    const name = (id) => data.items.find((i) => i.id === id).name;
    assert.equal(await page.locator('#detail-name').textContent(), name(visible[0]));
    assert.equal(await page.locator(`#sheet .card[data-id="${visible[0]}"]`).getAttribute('aria-pressed'), 'true');
    await page.keyboard.press(']');
    assert.equal(await page.locator('#detail-name').textContent(), name(visible[1]));
    await page.keyboard.press(']');
    assert.equal(await page.locator('#detail-name').textContent(), name(visible[2]));
    await page.keyboard.press('[');
    assert.equal(await page.locator('#detail-name').textContent(), name(visible[1]));
    assert.match(page.url(), new RegExp('item=' + encodeURIComponent(visible[1]).replace(/[()*]/g, '\\$&')));
    await page.locator('#sheet .card[tabindex="0"]').focus();
    await page.keyboard.press('Tab');
    assert.equal(await page.evaluate(() => Boolean(document.activeElement.closest('#sheet'))), false, 'Tab leaves the sheet in one step');
    env.proof.checks.keyboard = {columns, start_index: index, home: true, arrows: true, end: true, enter_opens: true, bracket_steps: true, roving_tabindex: true, focus_outline: outline};
  });

  await t.test('side-by-side card sizes and byte-verified full size, with visual notes kept apart from findings', async () => {
    await page.goto(env.origin + '/tools/ui/cartoon-review/?country=Tonga&item=' + encodeURIComponent('historical:taufaahau_tupou_iv#0'));
    await ready(page);
    await page.locator('#detail .verify[data-verified="true"]').waitFor();
    const boxes = await page.$$eval('#detail .small-views .thumb', (els) => els.map((e) => [Math.round(e.getBoundingClientRect().width), Math.round(e.getBoundingClientRect().height)]));
    assert.deepEqual(boxes, [[106, 152], [90, 132], [156, 218]]);
    await page.waitForFunction(() => [...document.querySelectorAll('#detail .small-views img')].every((img) => img.complete && img.naturalWidth > 0));
    const full = await page.$eval('#detail .full-holder img', (img) => [img.naturalWidth, img.naturalHeight]);
    assert.deepEqual(full, [1024, 1536]);
    assert.match(await page.locator('#detail .verify').textContent(), /SHA-256 verified against the export/);
    assert.match(await page.locator('#detail .automated').textContent(), /not visual approval/);
    assert.match(await page.locator('#detail .visual').textContent(), /fixes proposed · not approval/);
    await page.locator('#compare-anchor').check();
    await page.waitForFunction(() => [...document.querySelectorAll('#detail .anchor-strip img')].filter((img) => img.complete && img.naturalWidth > 0).length === 3);
    await toFilters(page);
    await shot(page, 'desktop-tupou-side-by-side-with-style-references.jpg');
    await page.locator('#compare-anchor').uncheck();

    await page.goto(env.origin + '/tools/ui/cartoon-review/?item=' + encodeURIComponent('selector:Tonga'));
    await ready(page);
    await page.locator('#detail .verify[data-verified="true"]').waitFor();
    const selector = data.items.find((i) => i.id === 'selector:Tonga');
    assert.ok((await page.$eval('#detail .small-views img', (img) => img.getAttribute('src'))).endsWith(selector.card.split('/').pop()), 'the selector card uses its display derivative');
    assert.match(await page.locator('#detail .small-views').textContent(), /Selector pick card 143×174/);
    const reference = page.locator('#detail details.reference');
    assert.equal(await reference.locator('img').count(), 0, 'the reference photograph is not loaded until asked for');
    assert.match(await reference.locator('summary').textContent(), /not game art; never shown as an avatar/);
    env.proof.checks.side_by_side = {card_boxes: boxes, full_size: full, verified: true, style_reference_strip: 3, selector_card: selector.card, reference_photo_deferred: true};
  });

  await t.test('a changed file is flagged and a changed input fails closed', async () => {
    const other = fs.readFileSync(path.join(root, 'spheres-web/ui/person-portraits/margaret-thatcher-cartoon-1990-v3.png'));
    const tampered = await env.context.newPage();
    tampered.on('pageerror', (error) => env.errors.push(error.message));
    await tampered.route('**/taufaahau-tupou-iv-cartoon-1990-v1.png', (route) => route.fulfill({status: 200, contentType: 'image/png', body: other}));
    await tampered.goto(env.origin + '/tools/ui/cartoon-review/?item=' + encodeURIComponent('historical:taufaahau_tupou_iv#0'));
    await ready(tampered);
    await tampered.locator('#detail .verify[data-verified="false"]').waitFor();
    assert.match(await tampered.locator('#detail .verify').textContent(), /Bytes differ from the export/);
    await tampered.close();

    const stale = await env.context.newPage();
    await stale.route('**/spheres-web/data/person_portraits.json', async (route) => {
      const response = await route.fetch();
      await route.fulfill({response, body: (await response.text()) + ' '});
    });
    await stale.goto(env.origin + '/tools/ui/cartoon-review/');
    await stale.waitForFunction(() => document.body.dataset.state === 'stale');
    assert.equal(await stale.locator('#status').getAttribute('role'), 'alert');
    assert.match(await stale.locator('#status').textContent(), /export is stale: spheres-web\/data\/person_portraits\.json/);
    assert.equal(await stale.locator('#sheet .card').count(), 0);
    await stale.close();
    env.proof.checks.tamper = {changed_image_flagged: true, changed_input_fails_closed: true};
  });

  await t.test('narrow layouts at 390 and 320 px: no horizontal overflow, dialog detail, Escape returns focus', async () => {
    const readings = [];
    for (const [width, height] of [[390, 844], [320, 700]]) {
      await page.setViewportSize({width, height});
      await page.goto(env.origin + '/tools/ui/cartoon-review/?collection=cartoons&country=Japan');
      await ready(page);
      await page.locator('#count').evaluate((el) => el.scrollIntoView({block: 'start'}));
      readings.push({width, page: await noHorizontalOverflow(page)});
      assert.equal(await page.locator('#detail').isVisible(), false, 'the detail waits for a card on narrow screens');
      await shot(page, `narrow-${width}-sheet.jpg`);
      const card = page.locator('#sheet .card[tabindex="0"]');
      const id = await card.getAttribute('data-id');
      await card.focus();
      await page.keyboard.press('Enter');
      const detail = page.locator('#detail');
      await detail.locator('.verify[data-verified="true"]').waitFor();
      assert.equal(await detail.getAttribute('role'), 'dialog');
      assert.equal(await detail.getAttribute('aria-modal'), 'true');
      assert.equal(await page.evaluate(() => document.activeElement.id), 'detail-close');
      const box = await detail.boundingBox();
      assert.ok(box.x >= 0 && box.width <= width + 0.5, JSON.stringify(box));
      const inner = await detail.evaluate((el) => ({scroll: el.scrollWidth, client: el.clientWidth}));
      assert.ok(inner.scroll <= inner.client + 1, JSON.stringify(inner));
      readings.push({width, dialog: inner});
      if (width === 390) await shot(page, 'narrow-390-detail-dialog.jpg');
      await page.keyboard.press('Escape');
      assert.equal(await detail.isVisible(), false);
      assert.equal(await page.evaluate(() => document.activeElement.dataset.id), id, 'focus returns to the card');
    }
    await page.setViewportSize({width: 1280, height: 900});
    env.proof.checks.narrow = readings;
  });

  await t.test('only this repository was contacted and nothing failed', async () => {
    await page.close();
    const foreign = env.requests.filter((r) => !r.url.startsWith(env.origin + '/') && !r.url.startsWith('blob:') && !r.url.startsWith('data:'));
    assert.deepEqual(foreign, []);
    assert.deepEqual(env.requests.filter((r) => r.method !== 'GET'), []);
    assert.deepEqual(env.failures, []);
    assert.deepEqual(env.errors, []);
    env.proof.passed = true;
  });
});
