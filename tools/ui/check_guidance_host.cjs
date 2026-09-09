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
const closeNames = ['closeGlobalMenus', 'closeGameDrawers', 'closeTech', 'closeStock', 'closeGovernment',
  'closeSheet', 'closeTechMenu', 'closeProduction', 'closeLogistics', 'closeDomination', 'setKeysCard'];
const reviewNames = ['homeNation', 'openConstructionCabinet', 'openConstruction', 'constructionPreviewProject',
  'openIndustry', 'openGovernment', 'openTech', 'openEquipmentDrawer', 'toggleGameDrawer', 'openAgency',
  'openStock', 'showCampaigns'];
const forbiddenNames = ['api', 'advance', 'advanceDay', 'clockPlay', 'clockPause', 'clockToggle', 'clockStep',
  'clockSpeed', 'clockSetSpeed', 'saveGame', 'saveCampaign', 'saveNamedCampaign', 'cabinetEnact',
  'equipmentCommand', 'constructionStart', 'sendCommand'];
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
  run(c, `${routeDeclaration}\n${functionSource('guidanceBusy')}\n${functionSource('guidanceNavigate')}`);
  return {c, calls, flags, navigate(action) { c.action = action; return run(c, 'guidanceNavigate(action)'); }};
}

test('every actual guidance route opens its existing review and preserves routing details without orders', () => {
  const cases = [
    [{kind: 'home'}, [['homeNation']]],
    [{kind: 'budget', command: {kind: 'advance'}}, [['openConstructionCabinet', 'budget']]],
    [{kind: 'construction'}, [['openConstruction']]],
    [{kind: 'project', id: 987}, [['openConstruction', {project: 987}]]],
    [{kind: 'suggestion', project_kind: 'starter_industry', district: 'JP-13', capacity_micros: 5001},
      [['openConstruction'], ['constructionPreviewProject', 'starter_industry', 'JP-13', 5001]]],
    [{kind: 'industry'}, [['openIndustry']]],
    [{kind: 'government', nation: 'USA'}, [['openGovernment', 'Japan']]],
    [{kind: 'research', domain: 'Energy'}, [['openTech', 'Energy']]],
    [{kind: 'equipment'}, [['openEquipmentDrawer']]],
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
