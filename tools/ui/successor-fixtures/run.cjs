'use strict';
// S24 successor-country fixture harness (preparation only).
//
// Starts its OWN server from an already-built binary, with the server's working
// directory (its only save root) inside a new --out directory, on a free port
// that is never 7777. It never reads, writes or lists any other save location.
// Every activation is a labelled fixture: it does not prove organic succession,
// historical timing or the S25 long-campaign route. See README.md beside this file.
const assert = require('node:assert/strict');
const fs = require('node:fs');
const path = require('node:path');
const cp = require('node:child_process');
const net = require('node:net');
const zlib = require('node:zlib');
const lib = require('./lib.cjs');
const { buildInventory, ROOT } = require('./inventory.cjs');

const CLIENT = 's24-successor-fixtures';
const DEFAULT_BROWSER = 'Russia:N1,Kazakhstan:N1,Serbia:H1,Croatia:H1';
const HARNESS_FILES = ['tools/ui/successor-fixtures/run.cjs', 'tools/ui/successor-fixtures/lib.cjs', 'tools/ui/successor-fixtures/inventory.cjs'];
const RUNTIME_PATHS = ['spheres-sim', 'spheres-cli', 'spheres-web', 'Cargo.toml', 'Cargo.lock'];
const MINISTRIES = ['health', 'education', 'housing', 'pensions', 'infrastructure', 'industry', 'science', 'defense', 'security', 'diplomacy'];
// Opening macro readings retained for review (not pass/fail criteria).
const MACRO = ['gdp', 'population', 'stability', 'separatism', 'inflation', 'rate', 'interest_rate', 'debt', 'debt_gdp', 'tax', 'political_capital', 'mil_strength', 'unemployment'];

function usage() {
  return 'Usage: node tools/ui/successor-fixtures/run.cjs --binary <spheres-web.exe> --expected-revision <40-hex> --out <new absolute dir>\n'
    + '  [--native-s21 <dir with succession.json>] [--native-provenance <json>] [--api all|none|A,B] [--browser ' + DEFAULT_BROWSER + '|none]\n'
    + '  [--cross-check] [--keep-saves] [--allow-dirty]';
}
function parseArgs(argv) {
  const o = { api: 'all', browser: DEFAULT_BROWSER, crossCheck: false, keepSaves: false, allowDirty: false };
  for (let i = 0; i < argv.length; i++) {
    const a = argv[i], next = () => { if (i + 1 >= argv.length) throw new Error(a + ' needs a value\n' + usage()); return argv[++i]; };
    if (a === '--binary') o.binary = next();
    else if (a === '--expected-revision') o.expected = next();
    else if (a === '--out') o.out = next();
    else if (a === '--native-s21') o.nativeDir = next();
    else if (a === '--native-provenance') o.nativeProvenance = next();
    else if (a === '--api') o.api = next();
    else if (a === '--browser') o.browser = next();
    else if (a === '--cross-check') o.crossCheck = true;
    else if (a === '--keep-saves') o.keepSaves = true;
    else if (a === '--allow-dirty') o.allowDirty = true;
    else throw new Error('Unknown argument ' + a + '\n' + usage());
  }
  if (!o.binary || !o.expected || !o.out) throw new Error(usage());
  if (!/^[0-9a-f]{40}$/.test(o.expected)) throw new Error('--expected-revision must be a full 40-character commit');
  for (const [what, values] of [['api', o.api === 'all' || o.api === 'none' ? [] : o.api.split(',')], ['browser', o.browser === 'none' ? [] : o.browser.split(',').map(value => { const parts = value.split(':'); if (parts.length > 2 || !['H1', 'N1'].includes(parts[1] || 'H1')) throw new Error('Unknown browser recipe: ' + value); return parts[0]; })]]) {
    if (values.some(value => !value) || new Set(values).size !== values.length) throw new Error('Empty or duplicate ' + what + ' identity');
  }
  if (!path.isAbsolute(o.out)) throw new Error('--out must be absolute');
  return o;
}

const git = args => { const r = cp.spawnSync('git', ['-c', 'core.longpaths=true', ...args], { cwd: ROOT, encoding: 'utf8', windowsHide: true, maxBuffer: 1 << 26 }); if (r.status !== 0) throw new Error('git ' + args.join(' ') + ': ' + r.stderr); return r.stdout.trim(); };
async function freePort() {
  for (;;) {
    const s = net.createServer();
    await new Promise(resolve => s.listen(0, '127.0.0.1', resolve));
    const port = s.address().port;
    await new Promise(resolve => s.close(resolve));
    if (port !== 7777) return port; // 7777 belongs to another session's server
  }
}
const { canonical, digest } = lib;
function without(value, keys) { const copy = { ...value }; for (const k of keys) delete copy[k]; return copy; }

// ---------------------------------------------------------------------------
// One case = an ordered list of checks. A product check that fails stops the
// case as FAILED; an environment error stops it as BLOCKED. Nothing is skipped
// silently and nothing is retried to manufacture a pass.
// ---------------------------------------------------------------------------
class CheckFailed extends Error {}
function newCase(identity, layer, extra) { return { identity, layer, status: 'unrun', checks: [], ...extra }; }
function check(c, name, ok, observed, expected) {
  const row = { name, status: ok ? 'passed' : 'failed', observed };
  if (expected !== undefined) row.expected = expected;
  c.checks.push(row);
  if (!ok) throw new CheckFailed(name);
}
async function runCase(c, stage, fn) {
  try { await fn(s => { stage = s; }); c.status = 'passed'; }
  catch (error) {
    if (error instanceof CheckFailed) { c.status = 'failed'; c.stopped_at = stage; }
    else { c.status = 'blocked'; c.blocked_stage = stage; c.error = String(error && error.stack || error).slice(0, 4000); }
  }
  return c;
}

// ---------------------------------------------------------------------------
// Server and HTTP.
// ---------------------------------------------------------------------------
async function startServer(ctx) {
  ctx.port = await freePort();
  ctx.url = 'http://127.0.0.1:' + ctx.port;
  const log = fs.openSync(path.join(ctx.out, 'server.log'), 'a');
  ctx.server = cp.spawn(ctx.binary, ['--port', String(ctx.port), '--no-open'], { cwd: ctx.run, windowsHide: true, stdio: ['ignore', log, log] });
  let launchError = null;
  ctx.server.on('error', e => { launchError = e; });
  const until = Date.now() + 30000;
  for (;;) {
    if (launchError) throw launchError;
    if (ctx.server.exitCode !== null) throw new Error('Server exited during startup with ' + ctx.server.exitCode);
    try { const r = await fetch(ctx.url + '/api/build', { signal: AbortSignal.timeout(2000) }); if (r.ok) { ctx.build = await r.json(); break; } } catch (e) { if (Date.now() > until) throw e; }
    if (Date.now() > until) throw new Error('Server startup exceeded 30 seconds');
    await new Promise(resolve => setTimeout(resolve, 100));
  }
}
async function stopServer(ctx) {
  const proc = ctx.server;
  if (!proc || proc.exitCode !== null || proc.signalCode) return true;
  await new Promise(resolve => { const timer = setTimeout(resolve, 5000); proc.once('exit', () => { clearTimeout(timer); resolve(); }); proc.kill(); });
  return proc.exitCode !== null || proc.signalCode !== null;
}
async function api(ctx, route, body) {
  const started = Date.now();
  const r = await fetch(ctx.url + route, body === undefined ? { signal: AbortSignal.timeout(180000) }
    : { method: 'POST', headers: { 'Content-Type': 'application/json' }, body: JSON.stringify(body), signal: AbortSignal.timeout(180000) });
  const text = await r.text();
  let json = null;
  try { json = JSON.parse(text); } catch { json = null; }
  ctx.telemetry('http', { route: route.split('?')[0], method: body === undefined ? 'GET' : 'POST', status: r.status, ms: Date.now() - started, bytes: text.length });
  return { status: r.status, json, text };
}
async function ok(ctx, route, body) { const r = await api(ctx, route, body); if (r.status !== 200 || !r.json) throw new Error(route + ' returned ' + r.status + ': ' + r.text.slice(0, 400)); return r.json; }

// A frozen binary may have been built from a Windows checkout with mixed LF/
// CRLF. Compare embedded content to the exact Git revision, retaining both raw
// hashes and the narrow normalization proof instead of assuming this separate
// review worktree reproduces every original build-input newline byte.
async function verifyServedAssets(ctx, page) {
  const details = {};
  for (const name of lib.REQUIRED_ASSETS) {
    const original = cp.spawnSync('git', ['show', ctx.expected + ':spheres-web/ui/' + name], { cwd: ROOT, windowsHide: true, maxBuffer: 1 << 26 });
    assert.equal(original.status, 0, 'Cannot read exact committed asset ' + name);
    const response = await page.request.get(ctx.url + (name === 'index.html' ? '/' : '/' + name));
    assert(response.ok(), 'Missing served asset ' + name);
    details[name] = lib.verifyServedText(original.stdout, await response.body());
  }
  return { revision: ctx.expected, binary_sha256: lib.fileSha256(ctx.binary), assets: Object.fromEntries(Object.entries(details).map(([name, proof]) => [name, proof.served_sha256])), source_details: details };
}

// Saves are written by the server into <out>/server/saves only; slots are
// harness-chosen literals, so nothing can resolve outside the run directory.
function slotFile(ctx, slot) { assert.match(slot, /^[A-Za-z0-9_-]{1,64}$/); return path.join(ctx.run, 'saves', slot + '.json'); }
async function saveSlot(ctx, slot, sessionId) {
  const file = slotFile(ctx, slot);
  assert(!fs.existsSync(file), 'Refusing to overwrite ' + file);
  const r = await ok(ctx, '/api/save', { slot, session_id: sessionId });
  assert.equal(r.ok, true);
  assert.equal(fs.realpathSync(file), file, 'Save escaped the isolated run directory');
  const text = fs.readFileSync(file, 'utf8');
  return { slot, file, bytes: Buffer.byteLength(text, 'utf8'), archive_sha256: lib.sha256(Buffer.from(text, 'utf8')), projection_sha256: lib.sha256(Buffer.from(lib.archiveProjection(text), 'utf8')) };
}
function discard(ctx, file) {
  if (ctx.keepSaves) return;
  const saves = fs.realpathSync(path.join(ctx.run, 'saves'));
  for (const f of [file, file.replace(/\.json$/, '.json.bak')]) if (fs.existsSync(f)) {
    assert(/^[A-Za-z0-9_-]+\.json(?:\.bak)?$/.test(path.basename(f)), 'Unexpected disposable archive name');
    assert.equal(path.dirname(fs.realpathSync(f)), saves, 'Refusing cleanup outside the isolated saves directory');
    fs.rmSync(f); ctx.discarded.push(path.relative(ctx.out, f));
  }
}
function retainArchive(ctx, file, name) {
  assert.match(name, /^[A-Za-z0-9_-]+$/);
  const raw = fs.readFileSync(file), packed = zlib.gzipSync(raw), rel = 'fixtures/' + name + '.json.gz';
  fs.mkdirSync(path.join(ctx.out, 'fixtures'), { recursive: true });
  fs.writeFileSync(path.join(ctx.out, rel), packed, { flag: 'wx' });
  const record = { file: rel, compression: 'gzip', bytes: raw.length, sha256: lib.sha256(raw), gzip_bytes: packed.length, gzip_sha256: lib.sha256(packed) };
  assert(lib.retainedArchive(record, file => fs.readFileSync(path.join(ctx.out, file))).equals(raw), 'Retained fixture roundtrip differs');
  return record;
}
const DISTRICTS = { nations: null };
function districtList(id) { return (DISTRICTS.nations[id] || []).map(d => d.id); }

// Compact, session-free projections of the served readings a player uses to
// confirm who they govern: identity, map, government, budget and guidance.
function nationRow(state, id) { return (state.nations || []).find(n => n.id === id) || null; }
function project(state, gov, cash, guidance, companies, identity, name) {
  const me = nationRow(state, identity), list = districtList(identity);
  const owned = list.filter(d => state.districts && state.districts[d] === identity).length;
  const ministries = (state.ministries && state.ministries.ministries) || [];
  const outcomes = guidance && guidance.outcomes || {};
  const macro = me ? Object.fromEntries(MACRO.filter(k => k in me).map(k => [k, me[k]])) : null;
  return {
    macro,
    selection: { player: state.player, player_name: state.player_name, date: state.date, alive: !!(me && me.alive), name: me && me.name,
      journey_status: state.campaign_journey && state.campaign_journey.status, journey_nation: state.campaign_journey && state.campaign_journey.nation,
      transitions: state.campaign_journey && state.campaign_journey.transitions, expected_name: name },
    map: { listed: list.length, owned_by_identity: owned },
    government: gov && { nation: gov.nation, nation_name: gov.nation_name, mine: gov.mine, electoral: gov.electoral, system: gov.system,
      ruling_institution: gov.ruling_institution, leader: gov.leader || null, ruling_bloc: gov.ruling_bloc, government_of_the_day_name: gov.government_of_the_day_name,
      political_capital: gov.political_capital, parties: (gov.groups || []).reduce((n, g) => n + (g.parties || []).length, 0), pillars: (gov.pillars || []).length, actions: (gov.actions || []).length },
    budget: { ministries: ministries.map(m => m.id), programs_enabled: !!(state.programs && state.programs.enabled), programs_fiscal_year: state.programs && state.programs.fiscal_year,
      cash_flow_nation: cash && cash.nation, cash_flow_name: cash && cash.name, on_the_books: cash && cash.on_the_books, cash_flow_ministries: cash && Array.isArray(cash.ministries) ? cash.ministries.length : null,
      // The budget card's debt-service line (policy_json money block), retained verbatim for review.
      money: state.policy && state.policy.money ? Object.fromEntries(['effective_rate', 'policy_rate', 'real_rate', 'spread', 'debt_gdp', 'interest_bn', 'interest_gdp'].map(k => [k, state.policy.money[k]])) : null },
    guidance: { player: guidance && guidance.state && guidance.state.player, session_matches: !!(guidance && guidance.state && guidance.state.session_id === state.session_id),
      outcomes_nation: outcomes.nation || null, outcomes_date: outcomes.date || null, production_nation: guidance && guidance.production && guidance.production.nation || null },
    capabilities: { cadence: state.simulation_cadence, connected_economy: !!(state.connected_economy && state.connected_economy.enabled), population: state.population_enabled,
      fiscal_recovery: state.fiscal_recovery_enabled, operational_warfare: !!(state.warfare_adoption && state.warfare_adoption.enabled),
      companies: !!(companies && companies.enabled), companies_nation: companies && companies.nation, supplier_operations: !!(companies && companies.operations && companies.operations.enabled) }
  };
}
async function readings(ctx, identity, name) {
  const state = await ok(ctx, '/api/state'), sid = encodeURIComponent(state.session_id);
  const gov = await ok(ctx, '/api/government?nation=' + encodeURIComponent(identity));
  const cash = await ok(ctx, '/api/cash-flow?session_id=' + sid);
  const guidance = await ok(ctx, '/api/guidance?session_id=' + sid);
  const companies = await ok(ctx, '/api/companies?session_id=' + sid);
  const sources = await ok(ctx, '/api/sources?nation=' + encodeURIComponent(identity));
  const p = project(state, gov, cash, guidance, companies, identity, name);
  p.sources = { id: sources.id, name: sources.name, start_1990: sources.start_1990 };
  const session = ['session_id', 'storage_notice', 'interrupt'];
  p.digests = { government: digest(gov), cash_flow: digest(without(cash, ['session_id'])), guidance_outcomes: digest(guidance.outcomes), guidance_production: digest(guidance.production),
    state_without_session: digest(without(state, session)) };
  return { state, p };
}
function assertInspection(c, stage, p, identity, name, parent) {
  const s = p.selection;
  check(c, stage + ':selection_identity', s.player === identity && s.player_name === name && s.alive && s.name === name && s.journey_nation === identity && s.journey_status !== 'succession'
    && Array.isArray(s.transitions) && s.transitions.length === 1 && s.transitions[0].from === parent && s.transitions[0].to === identity
    && p.sources.id === identity && p.sources.start_1990 === false, s, { player: identity, player_name: name, transition: parent + ' -> ' + identity, start_1990: false });
  check(c, stage + ':map_ownership', p.map.listed > 0 && p.map.owned_by_identity === p.map.listed, p.map, 'every listed district owned by ' + identity);
  const g = p.government;
  check(c, stage + ':government', !!g && g.nation === identity && g.nation_name === name && g.mine === true && (!!g.leader || !!g.ruling_institution) && Number.isFinite(g.political_capital),
    g, { nation: identity, mine: true, officeholder_or_institution: 'present' });
  const b = p.budget;
  check(c, stage + ':budget', JSON.stringify(b.ministries) === JSON.stringify(MINISTRIES) && b.cash_flow_nation === identity && b.cash_flow_name === name, b, { ministries: MINISTRIES, cash_flow_nation: identity });
  const gd = p.guidance;
  check(c, stage + ':guidance', gd.player === identity && gd.session_matches && (gd.outcomes_nation === null || gd.outcomes_nation === identity), gd, { player: identity, outcomes_nation: identity });
  const k = p.capabilities;
  check(c, stage + ':capabilities_retained', k.cadence === 'daily' && k.connected_economy && k.population && k.fiscal_recovery && k.operational_warfare && k.companies && k.companies_nation === identity && k.supplier_operations,
    k, 'daily cadence, connected economy, population, fiscal recovery, operational warfare, companies and supplier operations retained for ' + identity);
}
function comparable(p) { const { digests, ...rest } = p; return { ...rest, digests: without(digests, ['state_without_session']) }; }

// ---------------------------------------------------------------------------
// Fixture inputs.
// ---------------------------------------------------------------------------
async function nativeFixture(ctx, dir, provenance) {
  const source = path.resolve(dir, 'succession.json');
  const text = fs.readFileSync(source, 'utf8');
  const f = { id: 'n1-ussr-succession', recipe: 'N1', kind: 'native_s21_authored_dissolution', parent: 'USSR', label: lib.label('native_s21_authored_dissolution'),
    source_file: source, bytes: Buffer.byteLength(text, 'utf8'), archive_sha256: lib.sha256(Buffer.from(text, 'utf8')),
    projection_sha256: lib.sha256(Buffer.from(lib.archiveProjection(text), 'utf8')), exporter: provenance || null,
    verified_by: 'Each consuming case re-checks the loaded archive (fixture:dissolved_parent, fixture:succession_offer, fixture:turns_paused_until_continuation).' };
  const slot = 'n1-ussr-succession';
  fs.copyFileSync(source, slotFile(ctx, slot), fs.constants.COPYFILE_EXCL);
  f.slot = slot;
  assert.equal(provenance?.revision, ctx.expected, 'Native exporter revision must match the exact runtime');
  assert(/^[0-9a-f]{64}$/.test(provenance?.test_binary_sha256 || ''), 'Native exporter binary hash is required');
  assert(provenance?.runs?.some(run => run.succession_sha256 === f.archive_sha256 && run.projection_sha256 === f.projection_sha256 && run.bytes === f.bytes), 'Native exporter provenance must pin the actual consumed input');
  f.retained = { archive: retainArchive(ctx, source, f.id) };
  return f;
}
async function stagedFixture(ctx, parent, { aim = false } = {}) {
  const id = 'h1-' + parent.toLowerCase() + (aim ? '-aim' : '');
  const f = { id, recipe: 'H1', kind: 'harness_staged_parent_collapse', parent, seed: 1990, label: lib.label('harness_staged_parent_collapse'), served_aim_command: aim ? { kind: 'choose_campaign_aim', aim: 'prosperity' } : null, checks: [] };
  const fresh = await ok(ctx, '/api/new', { seed: 1990, nation: parent });
  f.checks.push({ name: 'fresh_parent_campaign', status: fresh.player === parent && fresh.date === '1 Jan 1990' ? 'passed' : 'failed', observed: { player: fresh.player, date: fresh.date } });
  let session = fresh.session_id;
  if (aim) {
    const r = await ok(ctx, '/api/command', { session_id: session, client_id: CLIENT + '-aim', request_seq: 1, commands: [f.served_aim_command] });
    f.checks.push({ name: 'served_aim_command', status: (r.errors || []).length === 0 && r.campaign_aims && r.campaign_aims.active ? 'passed' : 'failed', observed: { errors: r.errors, active: r.campaign_aims && r.campaign_aims.active && r.campaign_aims.active.aim } });
  }
  const base = await saveSlot(ctx, id + '-base', session);
  const baseText = fs.readFileSync(base.file, 'utf8');
  const staged = lib.stageNationFields(baseText, parent, { stability: '0.0', separatism: '1.0' });
  const stagedFile = slotFile(ctx, id + '-staged');
  fs.writeFileSync(stagedFile, staged.text, { flag: 'wx' });
  const before = fs.readFileSync(base.file), after = fs.readFileSync(stagedFile);
  f.patch = staged.patch;
  f.verified_only_patched = lib.verifyOnlyPatched(before, after, staged.patch);
  f.base_archive_sha256 = base.archive_sha256; f.base_projection_sha256 = base.projection_sha256;
  f.staged_archive_sha256 = lib.sha256(after);
  f.checks.push({ name: 'byte_exact_staging', status: f.verified_only_patched ? 'passed' : 'failed', observed: staged.patch });
  const loaded = await ok(ctx, '/api/load', { slot: id + '-staged' });
  const row = nationRow(loaded, parent);
  f.checks.push({ name: 'ordinary_load_of_staged_save', status: loaded.player === parent && row && row.alive && row.stability === 0 && loaded.date === '1 Jan 1990' ? 'passed' : 'failed',
    observed: { player: loaded.player, date: loaded.date, alive: row && row.alive, stability: row && row.stability } });
  const advanced = await ok(ctx, '/api/advance', { days: 1, commands: [], session_id: loaded.session_id, client_id: CLIENT, request_seq: 1 });
  const flag = ctx.inventory.families[parent].flag, dead = (advanced.dead || []).map(d => d.id);
  const offered = ((advanced.campaign_journey || {}).actions || []).filter(a => a.command && a.command.action === 'successor').map(a => a.command.target);
  f.dissolution = { date: advanced.date, flag_set: (advanced.flags || []).includes(flag), parent_dead: dead.includes(parent), journey_status: advanced.campaign_journey && advanced.campaign_journey.status, offered_continuations: offered, interrupt: advanced.interrupt || null };
  const family = ctx.inventory.families[parent].continuation_family;
  f.checks.push({ name: 'real_daily_dissolution', status: f.dissolution.flag_set && f.dissolution.parent_dead && f.dissolution.journey_status === 'succession' && JSON.stringify([...offered].sort()) === JSON.stringify([...family].sort()) ? 'passed' : 'failed', observed: f.dissolution });
  const dissolved = await saveSlot(ctx, id + '-dissolved', advanced.session_id);
  Object.assign(f, { slot: id + '-dissolved', bytes: dissolved.bytes, archive_sha256: dissolved.archive_sha256, projection_sha256: dissolved.projection_sha256 });
  f.retained = { base: retainArchive(ctx, base.file, id + '-base'), staged: retainArchive(ctx, stagedFile, id + '-staged'), archive: retainArchive(ctx, dissolved.file, id + '-dissolved') };
  discard(ctx, base.file); discard(ctx, stagedFile);
  f.status = f.checks.every(k => k.status === 'passed') ? 'passed' : 'failed';
  return f;
}
// Everything a loaded fixture must show before the player chooses a successor.
async function fixtureReady(ctx, c, fixture, identity, name, setStage) {
  setStage('load fixture');
  const loaded = await ok(ctx, '/api/load', { slot: fixture.slot });
  const flag = ctx.inventory.families[fixture.parent].flag, dead = (loaded.dead || []).map(d => d.id);
  check(c, 'fixture:dissolved_parent', loaded.player === fixture.parent && dead.includes(fixture.parent) && (loaded.flags || []).includes(flag),
    { player: loaded.player, date: loaded.date, parent_dead: dead.includes(fixture.parent), flag }, fixture.parent + ' dead with ' + flag);
  const action = ((loaded.campaign_journey || {}).actions || []).find(a => a.command && a.command.action === 'successor' && a.command.target === identity);
  check(c, 'fixture:succession_offer', loaded.campaign_journey && loaded.campaign_journey.status === 'succession' && !!action && action.label === 'Continue as ' + name,
    { status: loaded.campaign_journey && loaded.campaign_journey.status, label: action && action.label }, 'Continue as ' + name);
  setStage('turns paused before continuation');
  const paused = await api(ctx, '/api/advance', { days: 1, commands: [], session_id: loaded.session_id });
  const after = await ok(ctx, '/api/state');
  check(c, 'fixture:turns_paused_until_continuation', paused.status === 400 && /no longer exists/.test(String(paused.json && paused.json.error)) && after.date === loaded.date,
    { status: paused.status, error: paused.json && paused.json.error, date: after.date }, 'advance refused while the player government is dissolved');
  return { loaded, action };
}

// ---------------------------------------------------------------------------
// Layer G0: every successor is refused as a 1990 start and absent from the roster.
// ---------------------------------------------------------------------------
async function selectionGuards(ctx, cases) {
  const roster = await ok(ctx, '/api/roster'), rosterIds = new Set(roster.nations.map(n => n.id));
  const before = await ok(ctx, '/api/state');
  for (const row of ctx.inventory.rows) {
    const c = newCase(row.id, 'selection_guard', { recipe: 'G0', label: lib.label('selection_guard_no_activation') });
    cases.push(c);
    await runCase(c, 'selection guard', async () => {
      const r = await api(ctx, '/api/new', { seed: 1990, nation: row.id });
      const text = String(r.json && r.json.error);
      check(c, 'start_refused', r.status === 400 && text === row.name + ' is not on the board in January 1990 — it exists only if the state it succeeds comes apart. Choose another nation.', { status: r.status, error: text });
      const now = await ok(ctx, '/api/state');
      check(c, 'refusal_left_campaign_untouched', now.session_id === before.session_id && now.player === before.player && now.date === before.date, { session_unchanged: now.session_id === before.session_id, player: now.player, date: now.date });
      check(c, 'absent_from_starter_roster', !rosterIds.has(row.id) && roster.nations.length === ctx.inventory.context.roster_start_1990_true, { in_roster: rosterIds.has(row.id), roster_size: roster.nations.length });
      const s = await ok(ctx, '/api/sources?nation=' + encodeURIComponent(row.id));
      check(c, 'dossier_identity', s.id === row.id && s.name === row.name && s.start_1990 === false, { id: s.id, name: s.name, start_1990: s.start_1990 });
    });
  }
}

// ---------------------------------------------------------------------------
// Layer A: activation, save/reload and served inspection through the API.
// ---------------------------------------------------------------------------
async function activationCase(ctx, row, fixture) {
  const c = newCase(row.id, 'activation_save_load', { recipe: fixture.recipe, fixture_input: fixture.id, label: lib.label(fixture.kind) });
  const name = row.name, parent = row.parent;
  return runCase(c, 'start', async setStage => {
    const { loaded, action } = await fixtureReady(ctx, c, fixture, row.id, name, setStage);
    setStage('served continuation');
    const payload = { session_id: loaded.session_id, client_id: CLIENT, request_seq: 1, commands: [action.command] };
    const continued = await ok(ctx, '/api/command', payload);
    check(c, 'continuation_command', (continued.errors || []).length === 0 && continued.command_replayed === false && continued.player === row.id, { errors: continued.errors, player: continued.player, command: action.command });
    const replay = await ok(ctx, '/api/command', payload);
    check(c, 'lost_response_replay_is_idempotent', replay.command_replayed === true && replay.player === row.id && replay.campaign_journey.transitions.length === 1, { command_replayed: replay.command_replayed, transitions: replay.campaign_journey.transitions.length });
    setStage('inspect after activation');
    const first = await readings(ctx, row.id, name);
    assertInspection(c, 'after_activation', first.p, row.id, name, parent);
    c.readings = { after_activation: first.p };
    setStage('save and reload');
    const saved = await saveSlot(ctx, 'a-' + row.id.toLowerCase() + '-activated', first.state.session_id);
    const reloaded = await ok(ctx, '/api/load', { slot: saved.slot });
    check(c, 'ordinary_reload', reloaded.session_id !== first.state.session_id && reloaded.player === row.id && reloaded.date === first.state.date, { new_session: reloaded.session_id !== first.state.session_id, player: reloaded.player, date: reloaded.date });
    const second = await readings(ctx, row.id, name);
    assertInspection(c, 'after_reload', second.p, row.id, name, parent);
    const equal = canonical(comparable(first.p)) === canonical(comparable(second.p));
    c.readings.after_reload = { equal_to_after_activation: equal, digest: digest(comparable(second.p)) };
    check(c, 'readings_equal_after_reload', equal, { equal, state_digest_equal: first.p.digests.state_without_session === second.p.digests.state_without_session });
    const again = await saveSlot(ctx, 'a-' + row.id.toLowerCase() + '-reloaded', reloaded.session_id);
    check(c, 'archive_roundtrip_exact', saved.projection_sha256 === again.projection_sha256, { activated: saved.projection_sha256, reloaded: again.projection_sha256, bytes: saved.bytes });
    c.archives = { activated: without(saved, ['file']), reloaded: without(again, ['file']) };
    setStage('seven-day continuation');
    let state = reloaded, seq = 0;
    const start = new Date(Date.UTC(state.year, state.month - 1, state.day)), target = new Date(start.getTime() + 7 * 86400000), interrupts = [];
    const dayOf = s => new Date(Date.UTC(s.year, s.month - 1, s.day));
    while (dayOf(state) < target) {
      const days = Math.round((target - dayOf(state)) / 86400000), body = { days, commands: [], session_id: state.session_id, player_context: row.id, client_id: CLIENT + '-days', request_seq: ++seq };
      const next = await ok(ctx, '/api/advance', body);
      const dup = await ok(ctx, '/api/advance', body);
      check(c, 'advance_' + seq + '_duplicate_turn_replay', digest(without(dup, ['storage_notice'])) === digest(without(next, ['storage_notice'])), { date: next.date });
      if (next.interrupt) interrupts.push({ date: next.date, reason: next.interrupt });
      check(c, 'advance_' + seq + '_progress', next.player === row.id && dayOf(next) > dayOf(state) && dayOf(next) <= target && (dayOf(next).getTime() === target.getTime() || !!next.interrupt), { date: next.date, player: next.player, interrupt: next.interrupt || null });
      state = next;
      if (seq > 7) break;
    }
    check(c, 'seven_days_as_successor', dayOf(state).getTime() === target.getTime() && state.player === row.id, { date: state.date, interruptions: interrupts });
    discard(ctx, saved.file); discard(ctx, again.file);
  });
}

// ---------------------------------------------------------------------------
// Layer B: the same fixture through ordinary browser controls.
// ---------------------------------------------------------------------------
const IDLE = () => !SESSION.busy && !COMMAND_CHANNEL.busy && !COMMAND_CHANNEL.pending && !advancing && !pendingAdvance && !AGENCY.busy && !gov.busy;
async function idle(page) { await page.waitForFunction(IDLE); }
const isPost = (r, p) => new URL(r.url()).pathname === p && r.request().method() === 'POST';
async function campaigns(page) {
  if (await page.locator('#campaignJourney').isVisible()) await page.locator('#journeyClose').click();
  if (await page.locator('#govScreen').isVisible()) await page.locator('#govScreen .tbar .x').click();
  if (await page.locator('#app').isVisible()) {
    if (await page.locator('.arc-time-menu').getAttribute('open') === null) await page.locator('.arc-time-menu > summary').click();
    await page.locator('#campaignsBtn').click();
  }
  if (!await page.locator('#savedCampaigns').isVisible()) await page.locator('#openSavesBtn').click();
  await page.locator('#savedCampaigns').waitFor({ state: 'visible' });
}
async function uiLoad(page, slot) {
  await campaigns(page);
  await page.locator('#saveSlots').selectOption(slot);
  const replacing = await page.evaluate(() => !!SESSION.live?.player);
  const response = page.waitForResponse(r => isPost(r, '/api/load'));
  await page.locator('#loadBtn').click();
  if (replacing) await page.locator('#campaignConfirmAccept').click();
  const r = await response;
  if (!r.ok()) throw new Error('UI load failed: ' + await r.text());
  await page.locator('#app').waitFor({ state: 'visible' });
  await idle(page);
}
async function noOverflow(locator) {
  const box = await locator.evaluate(e => ({ scroll: e.scrollWidth, width: e.clientWidth }));
  const text = await locator.innerText();
  return { fits: box.width > 0 && box.scroll <= box.width + 1, invalid_value_text: /\bNaN\b|\bundefined\b/.test(text), box };
}
// Modest evidence: only the part of the room visible in the viewport, as JPEG.
async function shot(ctx, c, locator, name) {
  const page = locator.page(), rel = 'screenshots/' + c.identity.toLowerCase() + '-' + name + '.jpg', file = path.join(ctx.out, rel);
  fs.mkdirSync(path.dirname(file), { recursive: true });
  await locator.evaluate(e => e.scrollIntoView({ block: 'start', inline: 'nearest', behavior: 'instant' }));
  // Fresh contexts have no image cache: wait (bounded) for the room's art to
  // load and decode so the capture shows what a player sees.
  const images = await locator.evaluate(e => {
    const shown = [...e.querySelectorAll('img')].filter(img => {
      const r = img.getBoundingClientRect(), s = getComputedStyle(img);
      return r.width > 0 && r.height > 0 && s.display !== 'none' && s.visibility !== 'hidden' && r.bottom > 0 && r.right > 0 && r.top < innerHeight && r.left < innerWidth;
    });
    return Promise.race([
      Promise.all(shown.map(img => (img.complete ? Promise.resolve() : new Promise(done => { img.addEventListener('load', done, { once: true }); img.addEventListener('error', done, { once: true }); }))
        .then(() => (img.decode ? img.decode().catch(() => {}) : null)))).then(() => 'complete'),
      new Promise(done => setTimeout(() => done('timeout'), 10000))
    ]).then(state => ({ state, in_viewport: shown.length, loaded: shown.filter(img => img.complete && img.naturalWidth > 0).length }));
  });
  const box = await locator.boundingBox(), vp = page.viewportSize();
  const x = Math.max(0, box.x), y = Math.max(0, box.y);
  const clip = { x, y, width: Math.min(box.x + box.width, vp.width) - x, height: Math.min(box.y + box.height, vp.height) - y };
  await page.screenshot({ path: file, type: 'jpeg', quality: 72, clip, animations: 'disabled' });
  c.screenshots.push({ file: rel, sha256: lib.fileSha256(file), bytes: fs.statSync(file).size, viewport: vp, clip, images, stage: name });
}
// A room that cannot be opened or read is a product failure for this identity,
// recorded as a failed check with the error, not an environment block.
async function room(c, name, fn) {
  try { await fn(); }
  catch (error) { if (error instanceof CheckFailed) throw error; check(c, name, false, { error: String(error && error.message || error).slice(0, 1500) }); }
}
async function uiInspect(ctx, c, page, row, stage, shots) {
  const name = row.name, parentName = ctx.inventory.families[row.parent].parent_name;
  await room(c, stage + ':ui_selection_identity', async () => {
    const hdr = (await page.locator('#hdrYou').innerText()).trim();
    const state = await page.evaluate(() => ({ player: S.player, player_name: S.player_name }));
    check(c, stage + ':ui_selection_identity', state.player === row.id && state.player_name === name && hdr.includes(name) && !hdr.includes(parentName),
      { header: hdr, page_state: state }, { header_includes: name, header_excludes: parentName });
  });
  // Government: the room consumes the exact native reading for this nation.
  await room(c, stage + ':ui_government', async () => {
    await page.locator('#govBtn').click();
    await page.waitForFunction(() => gov.open && gov.data && gov.dataState === S && !gov.data.error);
    const govUi = await page.evaluate(() => gov.data);
    const govApi = await (await page.request.get(ctx.url + '/api/government?nation=' + encodeURIComponent(row.id))).json();
    const hero = (await page.locator('#govScreen .gov-ui-hero h1').innerText()).trim();
    const overview = await page.locator('#gov-panel-overview').innerText();
    const officeholder = govUi.leader && (govUi.leader.name || govUi.leader.described) || govUi.ruling_institution;
    const fit = await noOverflow(page.locator('#govScreen .gov-ui'));
    const same = canonical(govUi) === canonical(govApi);
    // Country-specific structure: an electoral system shows its parliament, a regime its institutions.
    const politicsTab = (await page.locator('#gov-tab-politics').innerText()).trim(), expectedTab = govUi.electoral ? 'Parliament & parties' : 'Regime & institutions';
    check(c, stage + ':ui_government', govUi.nation === row.id && govUi.mine === true && hero === name && same && !!officeholder && overview.includes(officeholder) && politicsTab === expectedTab && fit.fits && !fit.invalid_value_text,
      { hero, nation: govUi.nation, mine: govUi.mine, electoral: govUi.electoral, system: govUi.system, politics_tab: politicsTab, officeholder, equals_native_reading: same, layout: fit },
      { hero: name, mine: true, officeholder_visible: true, politics_tab: expectedTab });
    if (shots.includes('government')) await shot(ctx, c, page.locator('#govScreen .gov-ui'), stage + '-government');
    await page.locator('#govScreen .tbar .x').click();
  });
  // Budget: the Economy room's yearly budget renders the ten ministries.
  await room(c, stage + ':ui_budget', async () => {
    await page.locator('[data-drawer="cabinetDrawer"]').click();
    await page.locator('#cabinetDrawer').waitFor({ state: 'visible' });
    await page.locator('#cab-tab-budget').click();
    await page.locator('#cabinet-budget').waitFor({ state: 'visible' });
    const tiles = await page.locator('#cabinet-budget [data-cab-ministry]').evaluateAll(els => els.map(e => e.dataset.cabMinistry));
    const summary = (await page.locator('#cabinetBudgetSummary').innerText()).trim();
    const kicker = (await page.locator('#cabinetNationDetails .cab-kicker').first().evaluate(e => e.textContent)).trim();
    const fit = await noOverflow(page.locator('#cabinet-budget'));
    check(c, stage + ':ui_budget', JSON.stringify([...tiles].sort()) === JSON.stringify([...MINISTRIES].sort()) && tiles.length === 10 && summary.length > 0 && kicker.includes(name) && fit.fits && !fit.invalid_value_text,
      { tiles, summary: summary.slice(0, 300), nation_kicker: kicker, layout: fit }, { tiles: MINISTRIES, kicker_includes: name });
    if (shots.includes('budget')) await shot(ctx, c, page.locator('#cabinet-budget'), stage + '-budget');
    await page.locator('#cabinetDrawer [data-close-drawers]').click();
  });
  // Guidance: the standard Advisors entry loads the current native reading.
  await room(c, stage + ':ui_guidance', async () => {
    const reading = page.waitForResponse(r => new URL(r.url()).pathname === '/api/guidance' && r.request().method() === 'GET');
    await page.locator('.decision-nav').getByRole('button', { name: 'Advisors', exact: true }).click();
    await page.locator('#guidanceDialog').getByRole('heading', { name: 'What needs your attention?', exact: true }).waitFor();
    const response = await reading, guidance = await response.json();
    await page.locator('#guidanceDialog .guidance-cards[aria-busy="false"]').waitFor();
    const cards = (await page.locator('#guidanceDialog .guidance-cards').innerText()).trim();
    const fit = await noOverflow(page.locator('#guidanceDialog'));
    check(c, stage + ':ui_guidance', response.ok() && guidance.state.player === row.id && cards.length > 0 && fit.fits && !fit.invalid_value_text,
      { reading_player: guidance.state.player, outcomes_nation: guidance.outcomes && guidance.outcomes.nation, cards: cards.slice(0, 400), layout: fit }, { reading_player: row.id });
    if (shots.includes('guidance')) await shot(ctx, c, page.locator('#guidanceDialog'), stage + '-guidance');
    await page.getByRole('button', { name: 'Close tutorial and advisors', exact: true }).click();
  });
}
async function browserCase(ctx, browser, row, fixture) {
  const c = newCase(row.id, 'browser_ui', { recipe: fixture.recipe + '+B1', fixture_input: fixture.id, label: lib.label(fixture.kind), screenshots: [] });
  let context, page;
  const errors = [], commands = [], advances = [];
  await runCase(c, 'browser start', async setStage => {
    const slot = 'b-' + row.id.toLowerCase() + '-fixture';
    fs.copyFileSync(path.join(ctx.run, 'saves', fixture.slot + '.json'), slotFile(ctx, slot), fs.constants.COPYFILE_EXCL);
    context = await browser.newContext({ viewport: { width: 1280, height: 800 }, reducedMotion: 'reduce' });
    page = await context.newPage();
    page.setDefaultTimeout(45000);
    page.on('pageerror', e => errors.push(e.message));
    page.on('request', r => { if (r.method() === 'POST') { const p = new URL(r.url()).pathname; if (p === '/api/command') commands.push(r.postDataJSON()); if (p === '/api/advance') advances.push(r.postDataJSON()); } });
    setStage('ordinary load of the fixture');
    await page.goto(ctx.url, { waitUntil: 'domcontentloaded' });
    await page.locator('#campaignHome').waitFor();
    await page.waitForFunction(() => !!SESSION.live?.session_id);
    await uiLoad(page, slot);
    const loaded = await page.evaluate(() => S.player);
    check(c, 'ui_fixture_loaded', loaded === row.parent, { player: loaded }, row.parent);
    setStage('visible continuation control');
    await page.locator('#campaignJourneyBtn').click();
    await page.waitForFunction(() => document.querySelector('#journeyStatus').textContent === '' && document.querySelector('#journeyBody').childElementCount > 0);
    const button = page.getByRole('button', { name: 'Continue as ' + row.name, exact: true });
    check(c, 'ui_succession_offer', await button.isVisible(), { label: 'Continue as ' + row.name });
    const response = page.waitForResponse(r => isPost(r, '/api/command'));
    await button.click();
    await page.locator('#campaignConfirmAccept').click();
    const r = await response, data = await r.json();
    await idle(page);
    await page.waitForFunction(() => document.querySelector('#journeyStatus').textContent === '');
    const journeyText = await page.locator('#journeyBody').innerText();
    check(c, 'ui_continuation', r.ok() && (data.errors || []).length === 0 && data.player === row.id && journeyText.includes(row.name), { status: r.status(), errors: data.errors, player: data.player });
    await page.locator('#journeyClose').click();
    setStage('inspect after activation');
    await uiInspect(ctx, c, page, row, 'after_activation', ['government', 'budget']);
    setStage('ordinary save and reload');
    const savedSlot = 'b-' + row.id.toLowerCase() + '-saved';
    await campaigns(page);
    await page.locator('#saveName').fill(savedSlot);
    const saving = page.waitForResponse(x => isPost(x, '/api/save'));
    await page.locator('#saveNamedBtn').click();
    const saved = await saving;
    check(c, 'ui_named_save', saved.ok(), { status: saved.status() });
    await page.locator('#saveSlots option[value="' + savedSlot + '"]').waitFor({ state: 'attached' });
    const beforeSession = await page.evaluate(() => S.session_id);
    await uiLoad(page, savedSlot);
    const afterSession = await page.evaluate(() => ({ session: S.session_id, player: S.player }));
    check(c, 'ui_reload', afterSession.session !== beforeSession && afterSession.player === row.id, { new_session: afterSession.session !== beforeSession, player: afterSession.player });
    await page.setViewportSize({ width: 390, height: 844 });
    setStage('inspect after reload (390 px)');
    await uiInspect(ctx, c, page, row, 'after_reload', ['guidance']);
    await page.setViewportSize({ width: 1280, height: 800 });
    setStage('archive comparison');
    const text = fs.readFileSync(slotFile(ctx, savedSlot), 'utf8');
    const state = await (await page.request.get(ctx.url + '/api/state')).json();
    const captured = await saveSlot(ctx, 'b-' + row.id.toLowerCase() + '-reloaded', state.session_id);
    const savedProjection = lib.sha256(Buffer.from(lib.archiveProjection(text), 'utf8'));
    check(c, 'ui_archive_roundtrip_exact', savedProjection === captured.projection_sha256, { saved: savedProjection, reloaded: captured.projection_sha256 });
    check(c, 'ui_single_continuation_and_no_advance', commands.length === 1 && advances.length === 0 && commands[0].commands[0].action === 'successor' && commands[0].commands[0].target === row.id, { commands: commands.length, advances: advances.length });
    check(c, 'ui_no_page_errors', errors.length === 0, { errors });
    discard(ctx, slotFile(ctx, slot)); discard(ctx, slotFile(ctx, savedSlot)); discard(ctx, captured.file);
  });
  if (c.status !== 'passed' && page) {
    try { const rel = 'failure-' + row.id.toLowerCase() + '.png'; await page.screenshot({ path: path.join(ctx.out, rel) }); c.failure_screenshot = rel; } catch { /* the case result already records the failure */ }
  }
  if (context) await context.close().catch(() => {});
  return c;
}

// ---------------------------------------------------------------------------
// Cross-check: the harness-staged recipe against the existing native exporter,
// on the one family where both exist. Informational; never a case status.
// ---------------------------------------------------------------------------
function worldDiff(aText, bText) {
  const a = lib.worldEntries(aText), b = lib.worldEntries(bText), keys = [...new Set([...a.map(e => e.key), ...b.map(e => e.key)])].sort();
  const differing = keys.filter(k => { const x = a.find(e => e.key === k), y = b.find(e => e.key === k); return !x || !y || x.raw !== y.raw; });
  const envelope = t => Object.fromEntries(lib.objectEntries(t, 0).filter(e => ['history', 'log', 'journey', 'saved_date', 'player'].includes(e.key)).map(e => [e.key, lib.sha256(Buffer.from(t.slice(e.start, e.end), 'utf8'))]));
  const ea = envelope(aText), eb = envelope(bText);
  return { world_keys: keys.length, differing_keys: differing, envelope_fields_differing: Object.keys(ea).filter(k => ea[k] !== eb[k]).sort() };
}

// Findings surfaced while inspecting the served readings. They are reported
// for review; they never change a case status and assert no money semantics.
function observe(cases) {
  const knee = Number((/const\s+SPREAD_KNEE\s*:\s*f64\s*=\s*([\d.]+)/.exec(fs.readFileSync(path.join(ROOT, 'spheres-sim/src/economy.rs'), 'utf8')) || [])[1]);
  const rows = cases.filter(c => c.layer === 'activation_save_load' && c.readings && c.readings.after_activation && c.readings.after_activation.budget.money)
    .map(c => ({ identity: c.identity, ...c.readings.after_activation.budget.money, inflation: c.readings.after_activation.macro && c.readings.after_activation.macro.inflation }))
    .filter(m => Number.isFinite(knee) && m.spread > 5e-7 && m.debt_gdp < knee);
  const out = [];
  if (rows.length) out.push({ id: 'F1', kind: 'presentation', informational: true, identities: rows.map(r => r.identity), spread_knee: knee,
    summary: 'The budget card\'s debt-service line labels the served money.spread as a "sovereign spread at N% of GDP", but for these newborn successors debt is below the ' + (knee * 100) + '% knee, where the simulation charges no debt spread. The served money.real_rate is policy rate minus inflation without the simulation\'s REAL_RATE_FLOOR (-2%), so money.spread = effective_rate - real_rate absorbs the floor. The effective rate itself matches economy::effective_interest_rate; only the breakdown and label disagree.',
    evidence: ['spheres-web/src/main.rs fn policy_json money.real_rate and money.spread', 'spheres-sim/src/economy.rs effective_interest_rate, REAL_RATE_FLOOR, SPREAD_KNEE', 'spheres-web/ui/index.html budget card "Rate paid" line'],
    values: rows });
  return out;
}

async function main() {
  const o = parseArgs(process.argv.slice(2));
  const out = path.resolve(o.out);
  const repo = path.resolve(ROOT);
  if (fs.existsSync(out)) throw new Error('--out must not exist: ' + out);
  if ((out + path.sep).toLowerCase().startsWith((repo + path.sep).toLowerCase())) throw new Error('--out must be outside the repository');
  const binary = fs.realpathSync(path.resolve(o.binary));
  const head = git(['rev-parse', 'HEAD']), dirty = git(['status', '--porcelain', '--untracked-files=all', '--', 'tools/ui/successor-fixtures', ...RUNTIME_PATHS]);
  const runtimeDiff = git(['diff', '--name-only', o.expected, head, '--', ...RUNTIME_PATHS]);
  if (runtimeDiff) throw new Error('Runtime source differs from the expected build revision:\n' + runtimeDiff);
  if (dirty && !o.allowDirty) throw new Error('Commit the harness and runtime first (or pass --allow-dirty for a development run):\n' + dirty);
  fs.mkdirSync(out, { recursive: true });
  const run = path.join(out, 'server');
  fs.mkdirSync(path.join(run, 'saves'), { recursive: true });
  const inventory = buildInventory();
  DISTRICTS.nations = JSON.parse(fs.readFileSync(path.join(ROOT, 'spheres-sim/data/districts.json'), 'utf8')).nations;
  const telemetryFile = path.join(out, 'progress.jsonl');
  const ctx = { out, run, binary, inventory, expected: o.expected, keepSaves: o.keepSaves, discarded: [], telemetry: (event, details) => fs.appendFileSync(telemetryFile, JSON.stringify({ utc: new Date().toISOString(), event, ...details }) + '\n') };
  const result = {
    format: lib.FORMAT_RESULT,
    scope: 'S24 preparation only: labelled successor-country fixtures (selection guard, activation, save/reload and served/UI inspection). Not S24 qualification, not organic succession and not the S25 campaign route; Codex retains exact-build startup/recovery qualification.',
    disclaimer: lib.DISCLAIMER,
    development_run: !!dirty,
    started_utc: new Date().toISOString(),
    harness: { revision: head, tree_clean_for_harness_and_runtime: !dirty, files_sha256_lf: Object.fromEntries(HARNESS_FILES.map(f => [f, lib.sha256(Buffer.from(fs.readFileSync(path.join(ROOT, f), 'utf8').replace(/\r\n/g, '\n'), 'utf8'))])),
      node: process.version, argv: process.argv.slice(2), options: { api: o.api, browser: o.browser, cross_check: o.crossCheck, keep_saves: o.keepSaves } },
    build: null,
    inventory: { format: inventory.format, rows: inventory.rows.length, expectation: inventory.expectation.successor_identities, native_activation_paths: inventory.counts.native_activation_paths, digest: lib.inventoryDigest(inventory) },
    retention_policy: 'Exact consumed fixture inputs and staging bases retained as verified gzip; transient comparison outputs are hash receipts.',
    fixture_inputs: [], cases: [], cross_checks: [], observations: [], summary: null, cleanup: {}
  };
  const write = () => fs.writeFileSync(path.join(out, 'result.json'), JSON.stringify(result, null, 2) + '\n');
  let browser;
  try {
    await startServer(ctx);
    const savesDir = path.resolve(ctx.build.save_directory);
    result.build = { expected_revision: o.expected, served_revision: ctx.build.revision, branch: ctx.build.branch, version: ctx.build.version, built_at_unix_seconds: ctx.build.built_at_unix_seconds,
      binary_path: binary, binary_sha256: lib.fileSha256(binary), binary_bytes: fs.statSync(binary).size, runtime_source_equal_to_expected: true,
      save_directory: ctx.build.save_directory, save_directory_isolated: savesDir.toLowerCase() === run.toLowerCase(), port: ctx.port };
    assert.equal(ctx.build.revision, o.expected.slice(0, 12), 'The binary was not built from --expected-revision');
    assert(result.build.save_directory_isolated, 'The server is not using the isolated save directory');
    write();

    await selectionGuards(ctx, result.cases);
    write();

    const selected = o.api === 'all' ? inventory.rows.filter(r => r.native_activation_path).map(r => r.id) : o.api === 'none' ? [] : o.api.split(',');
    const browserPlan = o.browser === 'none' ? [] : o.browser.split(',').map(s => { const [id, recipe = 'H1'] = s.split(':'); return { id, recipe }; });
    for (const id of [...selected, ...browserPlan.map(b => b.id)]) if (!inventory.rows.some(r => r.id === id)) throw new Error('Unknown successor identity ' + id);
    const fixtures = {};
    const need = parent => { if (!fixtures['H1-' + parent]) fixtures['H1-' + parent] = stagedFixture(ctx, parent); return fixtures['H1-' + parent]; };
    let native = null;
    if (o.nativeDir) {
      const provenance = o.nativeProvenance ? JSON.parse(fs.readFileSync(o.nativeProvenance, 'utf8')) : null;
      native = await nativeFixture(ctx, o.nativeDir, provenance);
      result.fixture_inputs.push(native);
    }
    const fixtureFor = async (row, recipe) => {
      if (recipe === 'N1') { if (!native) throw new Error('Recipe N1 needs --native-s21'); if (row.parent !== 'USSR') throw new Error('N1 only authors the USSR dissolution'); return native; }
      const f = await need(row.parent);
      if (!result.fixture_inputs.includes(f)) result.fixture_inputs.push(f);
      if (f.status !== 'passed') throw new Error('Staged fixture failed its native checks: ' + f.id);
      return f;
    };
    for (const row of inventory.rows) {
      if (!row.native_activation_path) { result.cases.push(newCase(row.id, 'activation_save_load', { reason: 'no_native_hook', detail: 'No politics.rs dissolution seats ' + row.id + ', it has no districts::SUCCESSOR_PARENTS entry and no campaign_journey continuation family; no existing entry point can activate it.', proposal: lib.PROPOSAL })); continue; }
      if (!selected.includes(row.id)) { result.cases.push(newCase(row.id, 'activation_save_load', { reason: 'not_selected' })); continue; }
      const recipe = row.parent === 'USSR' && native ? 'N1' : 'H1';
      let fixture;
      try { fixture = await fixtureFor(row, recipe); }
      catch (error) { const c = newCase(row.id, 'activation_save_load', { recipe, label: lib.label(recipe === 'N1' ? 'native_s21_authored_dissolution' : 'harness_staged_parent_collapse'), status: 'blocked', blocked_stage: 'fixture input', error: String(error.stack || error).slice(0, 4000) }); result.cases.push(c); continue; }
      result.cases.push(await activationCase(ctx, row, fixture));
      write();
      console.log(row.id + ' activation_save_load: ' + result.cases.at(-1).status);
    }

    if (o.crossCheck && native) {
      const staged = await stagedFixture(ctx, 'USSR', { aim: true });
      result.fixture_inputs.push(staged);
      const a = fs.readFileSync(slotFile(ctx, staged.slot), 'utf8'), b = fs.readFileSync(slotFile(ctx, native.slot), 'utf8');
      const diff = worldDiff(a, b);
      result.cross_checks.push({ name: 'H1 staged recipe (with the served Prosperity aim command) versus the native S21 exporter fixture, USSR', informational: true,
        staged_fixture: staged.id, native_fixture: native.id, world_bytes_equal: diff.differing_keys.length === 0, world_keys_compared: diff.world_keys, differing_world_keys: diff.differing_keys,
        envelope_fields_differing: diff.envelope_fields_differing, staged_dissolution: staged.dissolution,
        note: 'Compares the simulation world value of both post-dissolution archives key by key as raw bytes. Envelope history/log may differ by construction: the served aim command records its headline in the archive log, while the exporter calls campaign_aims::choose directly.' });
      write();
    }

    for (const row of inventory.rows) {
      const plan = browserPlan.find(b => b.id === row.id);
      if (!row.native_activation_path) { result.cases.push(newCase(row.id, 'browser_ui', { reason: 'no_native_hook', detail: 'No activation entry point exists for ' + row.id + '; there is no fixture to load into the UI.', proposal: lib.PROPOSAL })); continue; }
      if (!plan) { result.cases.push(newCase(row.id, 'browser_ui', { reason: 'not_selected' })); continue; }
      let fixture, c;
      try {
        fixture = await fixtureFor(row, plan.recipe);
        if (!browser) {
          const { chromium } = require('playwright');
          browser = await chromium.launch({ headless: true, ...(process.env.SPHERES_BROWSER_CHANNEL ? { channel: process.env.SPHERES_BROWSER_CHANNEL } : {}) });
          result.harness.browser = { version: browser.version(), channel: process.env.SPHERES_BROWSER_CHANNEL || 'bundled chromium' };
          try {
            const page = await (await browser.newContext()).newPage();
            result.build.served_assets_verified = await verifyServedAssets(ctx, page);
            await page.context().close();
          } catch (error) { result.build.served_assets_verified = { error: String(error.stack || error).slice(0, 2000) }; throw error; }
        }
        assert(result.build.served_assets_verified && !result.build.served_assets_verified.error, 'Served asset identity verification must pass before browser cases');
        c = await browserCase(ctx, browser, row, fixture);
      } catch (error) {
        c = newCase(row.id, 'browser_ui', { recipe: plan.recipe + '+B1', label: lib.label(plan.recipe === 'N1' ? 'native_s21_authored_dissolution' : 'harness_staged_parent_collapse'), status: 'blocked', blocked_stage: 'browser setup', error: String(error.stack || error).slice(0, 4000), screenshots: [] });
        if (fixture) c.fixture_input = fixture.id;
      }
      result.cases.push(c);
      write();
      console.log(row.id + ' browser_ui: ' + c.status);
    }
  } finally {
    if (browser) await browser.close().catch(() => {});
    result.cleanup.server_stopped = await stopServer(ctx);
    for (const f of result.fixture_inputs) if (f.slot) discard(ctx, slotFile(ctx, f.slot));
    result.cleanup.discarded_archives = ctx.discarded.length;
    result.cleanup.remaining_saves = fs.existsSync(path.join(run, 'saves')) ? fs.readdirSync(path.join(run, 'saves')) : [];
    result.cleanup.note = o.keepSaves ? 'Archives kept by --keep-saves; delete ' + run + ' when finished.' : 'Every disposable archive was hashed and then deleted; logs, results, screenshots and verified original fixture gzip archives remain.';
    const order = new Map(inventory.rows.map((r, i) => [r.id, i]));
    result.cases.sort((x, y) => lib.LAYERS.indexOf(x.layer) - lib.LAYERS.indexOf(y.layer) || order.get(x.identity) - order.get(y.identity));
    try { result.observations.push(...observe(result.cases)); } catch (error) { result.observations.push({ id: 'observe-error', error: String(error) }); }
    result.summary = lib.summarize(result.cases);
    result.finished_utc = new Date().toISOString();
    write();
  }
  const errors = lib.validateResults(result, inventory, { exists: f => fs.existsSync(path.join(out, f)), hashOf: f => lib.fileSha256(path.join(out, f)), readBytes: f => fs.readFileSync(path.join(out, f)) });
  console.log(JSON.stringify({ result: path.join(out, 'result.json'), summary: result.summary, validation_errors: errors }, null, 2));
  process.exitCode = lib.resultExitCode(result, errors);
}

if (require.main === module) main().catch(error => { console.error(error.stack || String(error)); process.exitCode = 1; });
module.exports = { parseArgs, canonical, project, discard, retainArchive };
