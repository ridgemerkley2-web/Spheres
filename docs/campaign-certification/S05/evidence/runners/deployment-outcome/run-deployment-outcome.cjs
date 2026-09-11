#!/usr/bin/env node
'use strict';
// Isolated public-HTTP outcome comparison. Requires Node >=22 JSON reviver source.
const fs = require('node:fs');
const path = require('node:path');
const crypto = require('node:crypto');
const net = require('node:net');
const { spawn } = require('node:child_process');
const { once } = require('node:events');

const PROTOCOL = 's05-deployment-outcome-v1';
const SOURCE_SHA = '5fba07caf82335609aab312f14a37f775dcac84e4821bddc36ab63a5bf795463';
const args = {};
for (let i = 2; i < process.argv.length; i += 2) {
  if (!process.argv[i].startsWith('--') || process.argv[i + 1] === undefined) throw Error('Arguments are --name value pairs.');
  args[process.argv[i].slice(2)] = process.argv[i + 1];
}
for (const name of ['binary', 'binary-sha256', 'revision', 'archive', 'output']) {
  if (!args[name]) throw Error(`Missing --${name}.`);
}
const binary = fs.realpathSync(args.binary);
const archive = fs.realpathSync(args.archive);
const output = path.resolve(args.output);
const baselineRoot = args.baseline ? fs.realpathSync(args.baseline) : null;
const sha = data => crypto.createHash('sha256').update(data).digest('hex');
const fileSha = file => sha(fs.readFileSync(file));

// JSON.parse's ordinary Number would round the saved u64 RNG. Retain every
// numeric wire token instead; canonicalization sorts only object keys and
// removes whitespace. It never applies tolerance or normalizes numeric values.
class WireNumber { constructor(source) { this.source = source; } }
function lossless(text) {
  return JSON.parse(text, (key, value, context) => {
    if (typeof value !== 'number') return value;
    if (!context || typeof context.source !== 'string') throw Error('This Node runtime lacks lossless JSON reviver source.');
    return new WireNumber(context.source);
  });
}
function canonical(value) {
  if (value instanceof WireNumber) return value.source;
  if (Array.isArray(value)) return '[' + value.map(canonical).join(',') + ']';
  if (value && typeof value === 'object') return '{' + Object.keys(value).sort().map(key => JSON.stringify(key) + ':' + canonical(value[key])).join(',') + '}';
  return JSON.stringify(value);
}
function unpack(value) {
  let world = value.world;
  while (world && world.format && world.world) world = world.world;
  if (!world || !world.rules || !world.rng) throw Error('Expected a complete archived simulation world.');
  return world;
}
function count(value) { return value ? (Array.isArray(value) ? value.length : Object.keys(value).length) : 0; }
function date(value) { return { year: Number(value.year.source), month: Number(value.month.source), day: value.day ? Number(value.day.source) : 1 }; }
function dayIndex(value) { const d = date(value); return Date.UTC(d.year, d.month - 1, d.day) / 86400000; }
function checkpointData(text) {
  const saved = lossless(text);
  if (saved.format !== 'spheres-campaign') throw Error('Expected /api/save to write a campaign archive.');
  // Exactly one persisted nondeterministic field is excluded. Session IDs are
  // transport-only and do not appear in this archive; no recursive omissions.
  if (!Object.hasOwn(saved, 'saved_unix')) throw Error('Campaign archive lacks its documented wall-clock field.');
  delete saved.saved_unix;
  const world = unpack(saved);
  return { saved, world, canonical: canonical(saved) };
}
function firstDifference(a, b, location = '$') {
  if (canonical(a) === canonical(b)) return null;
  if (a instanceof WireNumber || b instanceof WireNumber || !a || !b || typeof a !== 'object' || typeof b !== 'object') {
    return { path: location, baseline: canonical(a), candidate: canonical(b) };
  }
  if (Array.isArray(a) !== Array.isArray(b)) return { path: location, reason: 'Different container type' };
  const keys = new Set([...Object.keys(a), ...Object.keys(b)]);
  for (const key of keys) {
    if (!Object.hasOwn(a, key) || !Object.hasOwn(b, key)) return { path: `${location}.${key}`, reason: 'Missing property', baseline_has: Object.hasOwn(a, key), candidate_has: Object.hasOwn(b, key) };
    const result = firstDifference(a[key], b[key], `${location}.${key}`);
    if (result) return result;
  }
  return { path: location, reason: 'Different value' };
}

if (canonical(lossless('{"u":18446744073709551615,"f":-0.22152066}')) !== '{"f":-0.22152066,"u":18446744073709551615}') throw Error('Lossless canonicalization self-check failed.');
const binaryHash = fileSha(binary);
if (binaryHash !== args['binary-sha256'].toLowerCase()) throw Error('Executable SHA256 does not match the supplied pin.');
if (fileSha(archive) !== SOURCE_SHA) throw Error('Source archive is not the unchanged genuine pinned-master fixture.');
if (fs.existsSync(output)) throw Error('Output must be a new disposable directory.');
if (baselineRoot === output) throw Error('Candidate output cannot be the baseline directory.');
const baseline = baselineRoot ? JSON.parse(fs.readFileSync(path.join(baselineRoot, 'result.json'), 'utf8')) : null;
if (baseline && (!baseline.passed || baseline.protocol !== PROTOCOL || baseline.source_archive_sha256 !== SOURCE_SHA)) throw Error('Baseline provenance/success/protocol does not match.');
fs.mkdirSync(path.join(output, 'saves'), { recursive: true });
const copiedArchive = path.join(output, 'saves', 'source-master.json');
fs.copyFileSync(archive, copiedArchive, fs.constants.COPYFILE_EXCL);
if (fileSha(copiedArchive) !== SOURCE_SHA) throw Error('Disposable source copy does not match original.');

const record = {
  protocol: PROTOCOL, passed: false, comparison_passed: null,
  binary, binary_sha256: binaryHash, revision: args.revision,
  runner_sha256: fileSha(__filename), node_version: process.version,
  source_archive: archive, source_archive_sha256: SOURCE_SHA,
  baseline: baselineRoot, output, started_utc: new Date().toISOString(), finished_utc: null,
  server_pid: null, server_port: null, stopped_own_server: false,
  launch_arguments: null, sanitized_environment_keys: [],
  checkpoint_days: [0, 1, 15, 31], checkpoints: [], requests: [], comparison: [],
  rules_commands: [], session_changed_on_load: null, session_changed_on_day15_reload: null,
  reload_archive_unchanged: null, source_unchanged_after: null, binary_unchanged_after: null,
  scope: 'Outcome equality only, no timing/performance claim. Genuine pinned-master technical USA fixture retains its original date and complete books. Normal HTTP load/31 one-day advances/save and day-15 reload in a private fresh directory; no new rule or gameplay commands. Every persisted archive field is compared except top-level saved_unix; numeric wire tokens and all history/log entries remain exact. No existing game process or user save is touched.',
  failure: null,
};
let child;
const writeRecord = () => fs.writeFileSync(path.join(output, 'result.json'), JSON.stringify(record, null, 2) + '\n');
function persistRequest(row) {
  record.requests.push(row);
  fs.appendFileSync(path.join(output, 'requests.jsonl'), JSON.stringify(row) + '\n');
}
async function freePort() {
  const server = net.createServer();
  server.listen(0, '127.0.0.1');
  await once(server, 'listening');
  const port = server.address().port;
  await new Promise((resolve, reject) => server.close(error => error ? reject(error) : resolve()));
  return port;
}
async function run() {
  const port = await freePort();
  record.server_port = port;
  const environment = { ...process.env };
  for (const key of Object.keys(environment)) {
    if (/^SPHERES_/i.test(key) || key.toUpperCase() === 'PORT') {
      record.sanitized_environment_keys.push(key); delete environment[key];
    }
  }
  const launch = ['--no-open', '--port', String(port)];
  record.launch_arguments = launch;
  child = spawn(binary, launch, { cwd: output, env: environment, windowsHide: true, stdio: ['ignore', 'pipe', 'pipe'] });
  record.server_pid = child.pid;
  writeRecord();
  let announced = '';
  child.stdout.on('data', chunk => { announced += chunk.toString('utf8'); fs.appendFileSync(path.join(output, 'server.stdout.log'), chunk); });
  child.stderr.on('data', chunk => fs.appendFileSync(path.join(output, 'server.stderr.log'), chunk));
  let childError = null;
  child.on('error', error => { childError = error; });
  const deadline = Date.now() + 60000;
  while (!announced.includes(`SPHERES is running at http://127.0.0.1:${port}`)) {
    if (childError) throw childError;
    if (child.exitCode !== null) throw Error(`Own server exited ${child.exitCode} before readiness.`);
    if (Date.now() > deadline) throw Error('Own server did not announce readiness within 60 seconds.');
    await new Promise(resolve => setTimeout(resolve, 100));
  }
  async function request(route, payload) {
    const response = await fetch(`http://127.0.0.1:${port}${route}`, {
      method: payload === undefined ? 'GET' : 'POST',
      headers: payload === undefined ? {} : { 'Content-Type': 'application/json' },
      body: payload === undefined ? undefined : JSON.stringify(payload),
      signal: AbortSignal.timeout(90000),
    });
    const text = await response.text();
    let value;
    try { value = JSON.parse(text); } catch { throw Error(`HTTP ${route} did not return JSON (${response.status}).`); }
    persistRequest({ route, payload: payload ?? null, status: response.status,
      session_id: value.session_id ?? null, date: value.date ?? null,
      error: value.error ?? null, reason: value.reason ?? null, ok: value.ok ?? null });
    if (!response.ok || value.error || (Array.isArray(value.errors) && value.errors.length)) throw Error(`HTTP ${route} refused: ${text.slice(0, 1000)}`);
    return value;
  }
  const boot = await request('/api/state');
  let state = await request('/api/load', { slot: 'source-master' });
  let session = state.session_id;
  if (typeof session !== 'string' || !session) throw Error('Load response has no campaign session.');
  record.session_changed_on_load = boot.session_id !== session;
  if (!record.session_changed_on_load) throw Error('Explicit load did not replace the boot session.');
  let startDay;
  async function capture(label, elapsedDays) {
    const slot = `outcome-${label}`;
    await request('/api/save', { slot, session_id: session });
    const filename = path.join(output, 'saves', `${slot}.json`);
    const text = fs.readFileSync(filename, 'utf8');
    const data = checkpointData(text);
    const currentDay = dayIndex(data.world);
    if (startDay === undefined) startDay = currentDay;
    if (currentDay !== startDay + elapsedDays) throw Error(`Checkpoint ${label} did not advance exactly ${elapsedDays} calendar days.`);
    const canonicalFile = path.join(output, `${label}.canonical.json`);
    fs.writeFileSync(canonicalFile, data.canonical);
    const row = { label, elapsed_days: elapsedDays, date: date(data.world), saved_date: data.saved.saved_date,
      archive: path.relative(output, filename), archive_sha256: sha(text), canonical_archive: path.basename(canonicalFile),
      canonical_sha256: sha(data.canonical), canonical_world_sha256: sha(canonical(data.saved.world)),
      history_sha256: sha(canonical(data.saved.history)), log_sha256: sha(canonical(data.saved.log)),
      rng_state: data.world.rng.state.source, history_count: count(data.saved.history), log_count: count(data.saved.log),
      actual_rules: JSON.parse(canonical(data.world.rules)),
      workload: { conflicts: count(data.world.conflicts), sectors: count(data.world.campaign?.sectors),
        transfers: count(data.world.campaign?.transfers), service_buffers: count(data.world.campaign_supply?.buffers), service_cargo: count(data.world.campaign_supply?.cargo) } };
    record.checkpoints.push(row);
    writeRecord();
    process.stdout.write(JSON.stringify({ checkpoint: label, date: row.date, canonical_sha256: row.canonical_sha256, rng_state: row.rng_state, workload: row.workload }) + '\n');
    return data;
  }
  const initial = await capture('day00', 0);
  const original = lossless(fs.readFileSync(copiedArchive, 'utf8'));
  if (dayIndex(initial.world) !== dayIndex(unpack(original))) throw Error('Load changed the original archived date.');
  if (Number(initial.world.rules.operational_warfare?.source) !== 1 || count(initial.world.conflicts) < 1 || count(initial.world.campaign?.sectors) < 1 || count(initial.world.campaign_supply?.cargo) < 1) throw Error('Expected genuine existing operational warfare workload after load.');
  for (let day = 1; day <= 31; day++) {
    state = await request('/api/advance', { days: 1, commands: [], session_id: session, client_id: 's05-outcome-replay', request_seq: day });
    if (state.session_id !== session) throw Error('A normal daily turn unexpectedly replaced the campaign session.');
    if (day === 1) await capture('day01', day);
    if (day === 15) {
      const before = await capture('day15-before-reload', day);
      const oldSession = session;
      state = await request('/api/load', { slot: 'outcome-day15-before-reload' });
      session = state.session_id;
      record.session_changed_on_day15_reload = typeof session === 'string' && session !== oldSession;
      if (!record.session_changed_on_day15_reload) throw Error('Day-15 reload did not establish a new campaign session.');
      const after = await capture('day15-after-reload', day);
      record.reload_archive_unchanged = before.canonical === after.canonical;
      if (!record.reload_archive_unchanged) throw Error('Day-15 reload changed persisted state: ' + JSON.stringify(firstDifference(before.saved, after.saved)));
    }
    if (day === 31) await capture('day31', day);
  }
  if (baselineRoot) {
    for (const row of record.checkpoints) {
      const earlier = baseline.checkpoints.find(item => item.label === row.label);
      if (!earlier) throw Error(`Baseline lacks ${row.label}.`);
      const beforeText = fs.readFileSync(path.join(baselineRoot, earlier.canonical_archive), 'utf8');
      if (sha(beforeText) !== earlier.canonical_sha256) throw Error(`Baseline ${row.label} canonical evidence changed.`);
      const candidateText = fs.readFileSync(path.join(output, row.canonical_archive), 'utf8');
      const equal = beforeText === candidateText;
      record.comparison.push({ label: row.label, equal, baseline_sha256: earlier.canonical_sha256, candidate_sha256: row.canonical_sha256,
        first_difference: equal ? null : firstDifference(lossless(beforeText), lossless(candidateText)) });
    }
    record.comparison_passed = record.comparison.every(row => row.equal);
    if (!record.comparison_passed) throw Error('A/B persisted outcomes differ; see exact checkpoint comparisons.');
  }
  record.source_unchanged_after = fileSha(archive) === SOURCE_SHA && fileSha(copiedArchive) === SOURCE_SHA;
  record.binary_unchanged_after = fileSha(binary) === binaryHash;
  if (!record.source_unchanged_after || !record.binary_unchanged_after) throw Error('An input changed during the isolated replay.');
  record.passed = true;
}
run().catch(error => { record.failure = error.stack || String(error); process.exitCode = 1; }).finally(async () => {
  if (child && child.pid) {
    if (child.exitCode === null && child.signalCode === null) {
      const stopped = once(child, 'exit');
      child.kill();
      await stopped;
    }
    record.stopped_own_server = child.exitCode !== null || child.signalCode !== null;
  }
  record.finished_utc = new Date().toISOString();
  writeRecord();
  process.stdout.write(JSON.stringify({ passed: record.passed, comparison_passed: record.comparison_passed, output, failure: record.failure }) + '\n');
});
