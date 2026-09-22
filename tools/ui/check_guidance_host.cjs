// Execute the actual page's guidance route and keyboard helpers without mounting or a browser.
const test = require('node:test');
const assert = require('node:assert/strict');
const fs = require('node:fs');
const path = require('node:path');
const vm = require('node:vm');
const uiRoot = path.resolve(__dirname, '../../spheres-web/ui');
const page = fs.readFileSync(path.join(uiRoot, 'index.html'), 'utf8').replace(/\r\n/g, '\n');
const run = (context, code) => vm.runInContext(code, context, {timeout: 1000});
const plain = value => value === undefined ? undefined : JSON.parse(JSON.stringify(value));
function functionSource(name) {
  const start = new RegExp(`^function ${name}\\(`, 'm').exec(page);
  assert.ok(start, `Missing actual helper ${name}`);
  const end = page.indexOf('\n}', start.index);
  assert.ok(end > start.index, `Missing closing brace for ${name}`);
  return page.slice(start.index, end + 2);
}
const routeDeclaration = /^const GUIDANCE_ROUTES = new Set\([^\n]+\);/m.exec(page)?.[0];
assert.ok(routeDeclaration, 'Extract the real route allowlist, not a copy');
const equipmentSource = fs.readFileSync(path.join(uiRoot, 'equipment-ui.js'), 'utf8').replace(/\r\n/g, '\n');
const equipmentTabs = /^const EQUIPMENT_TABS=\[[^\n]+\];/m.exec(equipmentSource)?.[0];
assert.ok(equipmentTabs, 'Extract the real equipment tab list, not a copy');
const closeNames = ['closeGlobalMenus', 'closeGameDrawers', 'closeTech', 'closeStock', 'closeGovernment',
  'closeSheet', 'closeTechMenu', 'closeProduction', 'closeLogistics', 'closeDomination', 'setKeysCard'];
const reviewNames = ['homeNation', 'openConstructionCabinet', 'openCashFlow', 'openConstruction', 'constructionPreviewProject',
  'openIndustry', 'openGovernment', 'openTech', 'openEquipmentDrawer', 'openEquipment', 'equipmentFlightSelectPage', 'toggleGameDrawer', 'openAgency',
  'openStock', 'showCampaigns'];
const forbiddenNames = ['api', 'advance', 'advanceDay', 'clockPlay', 'clockPause', 'clockToggle', 'clockStep',
  'clockSpeed', 'clockSetSpeed', 'saveGame', 'saveCampaign', 'saveNamedCampaign', 'cabinetEnact',
  'equipmentCommand', 'constructionStart', 'sendCommand'];

test('campaign mount uses its standard guidance navigation and suppresses the redundant floating launcher', () => {
  const start = page.indexOf('const GUIDANCE = GuidanceUI.mount({'); assert(start >= 0);
  const end = page.indexOf('\n});', start); assert(end > start);
  let adapter;
  const c = vm.createContext({GuidanceUI: {mount(value) {adapter = value;}},
    guidanceBusy() {}, guidanceNavigate() {}, localStorage: {}, S: null,
    gameIsUp: () => true, clockPause() {throw Error('mount must not change time');}});
  run(c, page.slice(start, end + 4));
  assert.equal(adapter.launcher, false);
  assert.equal(adapter.canNavigate(), true, 'normal campaign navigation remains available');
  const nav = /<nav class="decision-nav" aria-label="Map assistance">([\s\S]*?)<\/nav>/.exec(page)?.[1];
  assert(nav, 'the campaign must supply its own visible guidance entry points');
  assert.match(nav, /onclick="openGuidance\('advisors'\)"[^>]*>Advisors<\/button>/);
  assert.match(nav, /onclick="openGuidance\('tutorial'\)"[^>]*>Tutorial<\/button>/);
});
function hostFixture() {
  const calls = [], flags = {externalPending: false, domination: true, keys: true};
  const c = vm.createContext({flags, S: {player: 'Japan', session_id: 'safe-session', date: '1990-01-01'},
    advancing: false, pendingAdvance: null, SESSION: {busy: false}, CAB: {busy: false},
    PROD: {busy: false, open: true}, LOGI: {open: true}, COMMAND_CHANNEL: {busy: false, pending: null},
    EQUIP: {busy: false}, gov: {busy: false}, AGENCY: {busy: false},
    equipmentExternalPending: () => flags.externalPending,
    dominationIsOpen: () => flags.domination, keysCardIsOpen: () => flags.keys});
  c.gameIsUp = () => !!c.S?.player;
  for (const name of [...closeNames, ...reviewNames]) c[name] = (...args) => calls.push([name, ...args.map(plain)]);
  for (const name of forbiddenNames) c[name] = () => { throw new Error(`Guidance invoked forbidden game action: ${name}`); };
  run(c, `${routeDeclaration}\n${equipmentTabs}\n${functionSource('guidanceBusy')}\n${functionSource('guidanceNavigate')}`);
  return {c, calls, flags, navigate(action) { c.action = action; return run(c, 'guidanceNavigate(action)'); }};
}

test('every actual guidance route opens its existing review and preserves routing details without orders', () => {
  const cases = [
    [{kind: 'home'}, [['homeNation']]],
    [{kind: 'budget', command: {kind: 'advance'}}, [['openConstructionCabinet', 'budget']]],
    [{kind: 'cash_flow', command: {kind: 'advance'}}, [['openCashFlow']]],
    [{kind: 'construction'}, [['openConstruction']]],
    [{kind: 'project', id: 987}, [['openConstruction', {project: 987}]]],
    [{kind: 'suggestion', project_kind: 'starter_industry', district: 'JP-13', capacity_micros: 5001},
      [['openConstruction'], ['constructionPreviewProject', 'starter_industry', 'JP-13', 5001]]],
    [{kind: 'industry'}, [['openIndustry']]],
    [{kind: 'government', nation: 'USA'}, [['openGovernment', 'Japan']]],
    [{kind: 'research', domain: 'Energy'}, [['openTech', 'Energy']]],
    [{kind: 'equipment'}, [['openEquipmentDrawer']]],
    [{kind: 'air', tab: 'service'}, [['openEquipment', {tab: 'flight'}]]],
    [{kind: 'companies', tab: 'designer'}, [['openEquipment', {tab: 'companies'}]]],
    [{kind: 'world'}, [['toggleGameDrawer', 'intelDrawer']]],
    [{kind: 'decisions'}, [['openAgency']]],
    [{kind: 'resources'}, [['openStock']]],
    [{kind: 'campaign'}, [['showCampaigns']]]
  ];
  const f = hostFixture();
  assert.deepEqual(Array.from(run(f.c, 'GUIDANCE_ROUTES')).sort(), cases.map(([action]) => action.kind).sort());
  for (const [action, expected] of cases) {
    const g = hostFixture(); const stateBefore = JSON.stringify(g.c.S);
    assert.equal(g.navigate(action), true, action.kind);
    assert.deepEqual(g.calls.slice(0, closeNames.length), closeNames.map(name => name === 'setKeysCard' ? [name, false] : [name]));
    assert.deepEqual(g.calls.slice(closeNames.length), expected, action.kind);
    assert.equal(JSON.stringify(g.c.S), stateBefore, 'Navigation does not mutate the campaign');
  }
  const standard = hostFixture();
  assert.equal(standard.navigate({kind: 'suggestion', project_kind: 'power_grid', district: 'JP-13'}), true);
  assert.deepEqual(standard.calls.at(-1), ['constructionPreviewProject', 'power_grid', 'JP-13', undefined]);
});

test('equipment tabs are validated against the real tab list and industry opens a named site only when complete', () => {
  const tabs = run(hostFixture().c, 'EQUIPMENT_TABS.map(row => row[0])');
  assert.ok(tabs.includes('flight') && tabs.includes('companies'), 'Air command and Companies are real equipment tabs');
  const cases = [
    ...Array.from(tabs, tab => [{kind: 'equipment', tab}, [['openEquipment', {tab}]]]),
    [{kind: 'equipment', tab: 'bogus'}, [['openEquipmentDrawer']]],
    [{kind: 'equipment', tab: '__proto__'}, [['openEquipmentDrawer']]],
    [{kind: 'equipment', tab: 7}, [['openEquipmentDrawer']]],
    [{kind: 'equipment', tab: null}, [['openEquipmentDrawer']]],
    [{kind: 'industry', district: 'JP-13', project_kind: 'starter_industry'}, [['openIndustry', {district: 'JP-13', kind: 'starter_industry'}]]],
    [{kind: 'industry', district: 'JP-13'}, [['openIndustry']]],
    [{kind: 'industry', project_kind: 'starter_industry'}, [['openIndustry']]],
    [{kind: 'industry', district: 13, project_kind: 'starter_industry'}, [['openIndustry']]],
    [{kind: 'air', page: 'bases'}, [['openEquipment', {tab: 'flight'}], ['equipmentFlightSelectPage', 'bases']]],
    [{kind: 'air', page: 'reports'}, [['openEquipment', {tab: 'flight'}]]],
    [{kind: 'air', page: ['bases']}, [['openEquipment', {tab: 'flight'}]]]
  ];
  for (const [action, expected] of cases) {
    const g = hostFixture(); const stateBefore = JSON.stringify(g.c.S);
    assert.equal(g.navigate(action), true, JSON.stringify(action));
    assert.deepEqual(g.calls.slice(closeNames.length), expected, JSON.stringify(action));
    assert.equal(JSON.stringify(g.c.S), stateBefore);
  }
  const older = hostFixture(); older.c.equipmentFlightSelectPage = undefined;
  assert.equal(older.navigate({kind: 'air', page: 'bases'}), true, 'A build without flight pages still opens Air command');
  assert.deepEqual(older.calls.slice(closeNames.length), [['openEquipment', {tab: 'flight'}]]);
});

test('an airbase route lands on the real Air command Bases page after the campaign reset', () => {
  const equipmentFunction = name => {
    const start = new RegExp(`^(?:async )?function ${name}\\(`, 'm').exec(equipmentSource);
    assert.ok(start, `Missing actual equipment helper ${name}`);
    return equipmentSource.slice(start.index, equipmentSource.indexOf('\n}', start.index) + 2);
  };
  const flightPages = /^const EQUIPMENT_FLIGHT_PAGES=\[[^\n]+\];/m.exec(equipmentSource)?.[0];
  assert.ok(flightPages && flightPages.includes('["bases","Bases"]'), 'Bases is a real Air command page');
  assert.match(equipmentFunction('equipmentFlightSelectPage'), /if\(!EQUIPMENT_FLIGHT_PAGES\.some\(\(\[key\]\)=>key===page\)\|\|equipmentPending\(\)\)return false;/,
    'The page helper validates the page and refuses while work is pending');
  assert.match(equipmentFunction('equipmentResetCampaign'), /EQUIP\.flightPage="command"/, 'A new campaign resets the flight page, so the page is chosen afterwards');
  for (const [action, page, pending] of [[{kind: 'air', page: 'bases'}, 'bases', false], [{kind: 'air'}, 'command', false],
    [{kind: 'air', page: 'bases'}, 'command', true]]) {
    const g = hostFixture(), renders = [];
    g.c.EQUIP = {busy: false, open: false, session: 'earlier-session', nation: 'Japan', tab: 'designer', flightPage: 'reports',
      details: new Set(), seq: 0, previewSeq: 0, draftSeq: 0, loading: false};
    Object.assign(g.c, {equipmentPending: () => pending, equipmentDisposeModel() {},
      equipmentRender() { renders.push(g.c.EQUIP.flightPage); }, equipmentFetch: async () => true});
    run(g.c, `${flightPages}\n${equipmentFunction('equipmentResetCampaign')}\n${equipmentFunction('openEquipment')}\n${equipmentFunction('equipmentFlightSelectPage')}`);
    assert.equal(g.navigate(action), true, JSON.stringify(action));
    assert.equal(g.c.EQUIP.tab, 'flight'); assert.equal(g.c.EQUIP.open, true);
    assert.equal(g.c.EQUIP.flightPage, page, JSON.stringify({action, pending}));
    assert.equal(renders.at(-1), page, 'The page shown after navigation is the selected page');
    assert.deepEqual(g.calls.slice(closeNames.length), [['openEquipmentDrawer']]);
  }
});

test('every new guidance destination is an actual global opener loaded by the page', () => {
  assert.match(functionSource('openCashFlow'), /openConstructionCabinet\("overview"\)/);
  const industry = fs.readFileSync(path.join(uiRoot, 'industry-ui.js'), 'utf8');
  assert.match(equipmentSource, /^async function openEquipment\(options=\{\}\)\{/m);
  assert.match(equipmentSource, /^window\.openEquipment=openEquipment;/m);
  assert.match(equipmentSource, /if\(EQUIPMENT_TABS\.some\(row=>row\[0\]===options\.tab\)\)EQUIP\.tab=options\.tab;/);
  assert.match(industry, /^function openIndustry\(options=\{\}\) \{/m);
  assert.match(industry, /^window\.openIndustry=openIndustry;/m);
  assert.match(industry, /typeof options\.district==="string"&&typeof options\.kind==="string"/);
  for (const name of ['equipment-ui.js', 'industry-ui.js']) assert.equal(page.split(`<script src="/${name}"></script>`).length - 1, 1, name);
});

// The actual api() helper, run against a fake fetch, proves where save/load receipts are captured.
const apiStart = page.indexOf('async function api(path, body, rawCommand = false) {');
assert.ok(apiStart >= 0, 'Find the actual api() helper');
const apiSource = page.slice(apiStart, page.indexOf('\n}', apiStart) + 2);
function apiFixture(reply, options = {}) {
  const receipts = [], requests = [];
  const c = vm.createContext({S: {session_id: 'campaign-a', player: 'Japan', date: '1 Jan 1990'},
    window: Object.hasOwn(options, 'window') ? options.window : {GUIDANCE_RECEIPT: value => { receipts.push(value); if (options.throws) throw new Error('guidance failed'); }},
    async fetch(url, init) {
      requests.push([url, init.method, init.body === undefined ? undefined : JSON.parse(init.body)]);
      if (options.replace) c.S = options.replace;
      const answer = reply(url);
      return {ok: answer.ok !== false, status: answer.status || 200, text: async () => JSON.stringify(answer.data)};
    }});
  run(c, apiSource);
  return {c, receipts, requests, call(url, body) { c.url = url; c.body = body; return run(c, 'api(url, body)'); }};
}

test('api captures only successful native save and load responses for guidance receipts', async () => {
  const saved = {ok: true, slot: 'first-hour', path: 'saves/first-hour.json', date: '1 Jan 1990', history_points: 1, dispatches: 2};
  const save = apiFixture(() => ({data: saved}));
  const state = save.c.S; const result = await save.call('/api/save', {slot: 'first-hour'});
  assert.deepEqual(plain(result), saved);
  assert.equal(save.receipts.length, 1);
  assert.equal(save.receipts[0].data, result); assert.equal(save.receipts[0].state, state);
  assert.deepEqual(plain(save.receipts[0]), {path: '/api/save', body: {slot: 'first-hour', session_id: 'campaign-a'}, data: saved,
    state: {session_id: 'campaign-a', player: 'Japan', date: '1 Jan 1990'}});
  const loaded = {session_id: 'campaign-z', player: 'Japan', date: '1 Jan 1990', storage_notice: 'Campaign and its history restored.', dispatch_count: 2};
  const load = apiFixture(() => ({data: loaded}));
  const before = load.c.S; await load.call('/api/load', {slot: 'first-hour', backup: false});
  assert.equal(load.receipts.length, 1);
  assert.equal(load.receipts[0].state, before, 'A load receipt is taken before the loaded state is adopted');
  assert.deepEqual(plain(load.receipts[0].body), {slot: 'first-hour', backup: false});
  for (const [url, body] of [['/api/guidance?session_id=campaign-a', undefined], ['/api/state', undefined], ['/api/new', {seed: 1}],
    ['/api/command', {kind: 'noop'}], ['/api/advance', {days: 1}], ['/api/save', undefined]]) {
    const other = apiFixture(() => ({data: {ok: true}}));
    await other.call(url, body);
    assert.equal(other.receipts.length, 0, url);
  }
  const refused = apiFixture(() => ({ok: false, status: 500, data: {ok: false, error: 'Could not save campaign'}}));
  await assert.rejects(refused.call('/api/save', {slot: 'first-hour'}), /Could not save campaign/);
  assert.equal(refused.receipts.length, 0);
  const moved = apiFixture(() => ({data: saved}), {replace: {session_id: 'campaign-b', player: 'Japan', date: '1 Jan 1990'}});
  await assert.rejects(moved.call('/api/save', {slot: 'first-hour'}), /campaign changed/);
  assert.equal(moved.receipts.length, 0, 'A save answered for a campaign that changed in flight is not captured');
  const failing = apiFixture(() => ({data: saved}), {throws: true});
  assert.deepEqual(plain(await failing.call('/api/save', {slot: 'first-hour'})), saved, 'A guidance failure never changes the save result');
  for (const window of [{}, {GUIDANCE_RECEIPT: null}]) {
    const absent = apiFixture(() => ({data: saved}), {window});
    assert.deepEqual(plain(await absent.call('/api/save', {slot: 'first-hour'})), saved);
  }
});

test('the page exposes the mounted guidance receipt handler after mounting', () => {
  const line = /^window\.GUIDANCE_RECEIPT = [^\n]+;$/m.exec(page);
  assert.ok(line && line.index > page.indexOf('const GUIDANCE = GuidanceUI.mount({'));
  const calls = [];
  const c = vm.createContext({window: {}, GUIDANCE: {receipt(value) { calls.push(value); return true; }}});
  run(c, line[0]);
  const receipt = {path: '/api/save'};
  assert.equal(c.window.GUIDANCE_RECEIPT(receipt), true);
  assert.deepEqual(calls, [receipt]);
});

test('unknown routes, pending work and absent campaigns return before closing or navigating anything', () => {
  for (const action of [null, {}, {kind: 'advance'}, {kind: 'save'}, {kind: '__proto__'}, {kind: 'constructor'},
    {kind: '<script>run()</script>'}, {kind: 1}]) {
    const f = hostFixture(); assert.equal(f.navigate(action), false); assert.deepEqual(f.calls, []);
  }
  for (const flag of ['advancing = true', 'pendingAdvance = {}', 'SESSION.busy = true', 'CAB.busy = true',
    'PROD.busy = true', 'COMMAND_CHANNEL.busy = true', 'COMMAND_CHANNEL.pending = {}',
    'flags.externalPending = true', 'EQUIP.busy = true', 'gov.busy = true', 'AGENCY.busy = true']) {
    const f = hostFixture(); assert.equal(run(f.c, '!!guidanceBusy()'), false);
    run(f.c, flag); assert.equal(run(f.c, '!!guidanceBusy()'), true, flag);
    assert.equal(f.navigate({kind: 'budget'}), false, flag); assert.deepEqual(f.calls, []);
  }
  for (const state of [null, {}, {player: null}]) {
    const f = hostFixture(); f.c.S = state;
    assert.equal(f.navigate({kind: 'government'}), false); assert.deepEqual(f.calls, []);
  }
});

test('actual document keyboard listener keeps N, speed keys and Space inert behind open guidance', () => {
  const marker = 'document.addEventListener("keydown", (e) => {\n  const room = arcadeTopRoom();';
  const start = page.indexOf(marker); assert.ok(start >= 0, 'Find the actual global room/clock keyboard handler');
  const end = page.indexOf('\n});', start); assert.ok(end > start);
  const source = page.slice(start, end + 4);
  const handlers = [], calls = [];
  const dialog = {open: true};
  const close = {focus() { calls.push('focus-close'); }};
  const c = vm.createContext({
    document: {addEventListener(type, fn) { assert.equal(type, 'keydown'); handlers.push(fn); },
      getElementById(id) { assert.equal(id, 'guidanceDialog'); return dialog; },
      querySelector(selector) { assert.equal(selector, '#guidanceDialog [data-guidance-close]'); return close; }},
    arcadeTopRoom: () => null,
    clock: {running: true, speed: 1},
    typing: () => false,
    tech: {open: false}, stock: {open: false}, gov: {open: false}
  });
  for (const name of forbiddenNames) c[name] = () => { throw new Error(`Hidden game shortcut reached ${name}`); };
  run(c, source); assert.equal(handlers.length, 1);
  for (const key of ['n', 'N', '1', '5', ' ']) {
    let prevented = 0; calls.length = 0;
    handlers[0]({key, target: {closest: () => null}, preventDefault() { prevented++; }});
    assert.equal(prevented, 1, key); assert.deepEqual(calls, ['focus-close']);
    assert.deepEqual(plain(c.clock), {running: true, speed: 1});
  }
  for (const key of [' ', 'Enter', 'ArrowDown']) {
    let prevented = 0; calls.length = 0;
    handlers[0]({key, target: {closest: selector => selector === '#guidanceDialog' ? dialog : null},
      preventDefault() { prevented++; }});
    assert.equal(prevented, 0, 'Native guidance controls keep their own keyboard interaction');
    assert.deepEqual(calls, []);
  }
});

test('guidance scripts, styles and artwork are referenced and served, and every inline script parses', () => {
  const server = fs.readFileSync(path.resolve(uiRoot, '../src/main.rs'), 'utf8');
  const scripts = ['tutorial-model.js', 'advisor-model.js', 'guidance-ui.js'];
  const positions = scripts.map(name => {
    const reference = `<script src="/${name}"></script>`;
    assert.equal(page.split(reference).length - 1, 1, `${name} loaded exactly once`);
    assert.ok(fs.existsSync(path.join(uiRoot, name)));
    assert.ok(server.includes(`include_str!("../ui/${name}")`));
    assert.ok(server.includes(`(Method::Get, "/${name}")`));
    new vm.Script(fs.readFileSync(path.join(uiRoot, name), 'utf8'), {filename: name});
    return page.indexOf(reference);
  });
  assert.ok(positions[0] < positions[2] && positions[1] < positions[2], 'Models load before controller');
  assert.ok(positions[2] < page.indexOf('GuidanceUI.mount('), 'Controller loads before host wiring');
  assert.ok(page.includes('<link rel="stylesheet" href="/guidance-ui.css">'));
  assert.ok(fs.statSync(path.join(uiRoot, 'guidance-ui.css')).size > 0);
  assert.ok(server.includes('include_str!("../ui/guidance-ui.css")'));
  assert.ok(server.includes('(Method::Get, "/guidance-ui.css")'));
  let parsed = 0;
  for (const [index, match] of Array.from(page.matchAll(/<script\b([^>]*)>([\s\S]*?)<\/script>/gi)).entries()) {
    if (!match[2].trim()) continue;
    const type = /\btype=["']([^"']+)["']/.exec(match[1])?.[1];
    if (type === 'application/json' || type === 'importmap') JSON.parse(match[2]);
    else new vm.Script(match[2], {filename: `index-inline-${index}.js`});
    parsed++;
  }
  assert.ok(parsed > 0, 'At least one actual inline script was checked');
  const Guidance = require('../../spheres-web/ui/guidance-ui.js');
  const Tutorial = require('../../spheres-web/ui/tutorial-model.js');
  const rendered = [];
  for (const lesson of Tutorial.lessons) rendered.push(Guidance.render({view: 'tutorial',
    progress: Tutorial.advance(null, {type: 'select', id: lesson.id}), canNavigate: true}));
  rendered.push(Guidance.render({view: 'advisors', filter: 'all', status: 'ready', progress: Tutorial.normalize(null),
    canNavigate: true, cards: ['economy', 'government', 'research', 'military', 'diplomacy'].map(area =>
      ({id: area, area, title: area, reason: 'Review', priority: 'routine', actionLabel: 'Review'}))}));
  const assets = new Set(Array.from(rendered.join('').matchAll(/src="(\/art\/areas\/[^"?]+)"/g), match => match[1]));
  assert.ok(assets.size > 0);
  for (const asset of assets) {
    const filename = path.basename(asset);
    assert.ok(fs.existsSync(path.join(uiRoot, 'area-art', filename)), asset);
    assert.ok(server.includes(`"${filename}" => include_bytes!("../ui/area-art/${filename}")`), `${filename} served`);
  }
});
