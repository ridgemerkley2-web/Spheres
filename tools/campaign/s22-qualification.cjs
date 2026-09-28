#!/usr/bin/env node
'use strict';

// S22's frozen limits are evaluated per cell, never across pooled/retried runs.
// This tool neither launches a workload nor edits a measurement artifact.
const fs = require('node:fs');
const path = require('node:path');
const crypto = require('node:crypto');
const cp = require('node:child_process');
const assert = require('node:assert/strict');

const SCHEMA = 'spheres-s22-qualification/v1';
const CASES = [
  { id: 'early-france-1999', date: '1999-05-13', end: '1999-06-13' },
  { id: 'mid-france-2015', date: '2015-01-01', end: '2015-02-01' },
  { id: 'late-france-2035', date: '2035-11-30', end: '2035-12-31' },
];
const ROUNDS = ['initial_full_qualification', 'second_full_confirmation'];
const ROLES = ['native', 'map', 'renderer'];
const RUNTIME = ['Cargo.toml', 'Cargo.lock', 'spheres-sim', 'spheres-web'];
const HARNESS = [
  'tools/campaign/run-s22-profile.ps1', 'tools/campaign/s22-qualification.cjs',
  'tools/ui/s22-performance-browser.cjs', 'tools/ui/s22-renderer-browser.cjs',
  'tools/ui/webgl-measurement.js', 'tools/ui/webgl-texture-measurement.js',
  'tools/ui/s22-browser-contract.cjs', 'tools/ui/s22-layout-browser.cjs',
  'tools/ui/s22-browser-observation.js',
];
const ASSETS = ['index.html', 'map-controls.js', 'globe3d.js', 'arsenal3d.js',
  'equipment-model.js', 'city-mesh.js', 'city-layer.js', 'arsenal-models.js',
  'equipment-mesh.js', 'equipment-ui.js', 'terrain-surface.js', 'cities.js',
  'rivers.js', 'water-detail.js', 'height-detail.js', 'coast.png', 'lake.png',
  'relief.png', 'tank-surface.js', 'military-surface.js'];
const VIEWS = [
  { name: 'world', zoom: 1.25, amplitude: .3 },
  { name: 'national', zoom: 8, amplitude: .07 },
  { name: 'regional', zoom: 40, amplitude: .015 },
  { name: 'paris-city', zoom: 1500, amplitude: .00036 },
];
const CONTROLS = ['west', 'east', 'zoom-in', 'zoom-out'];
const LIMITS = Object.freeze({ sim_p95_ms: 300, turn_p95_ms: 400, turn_max_ms: 750,
  memory_bytes: 1073741824, map_fps: 30, control_p95_ms: 200, viewer_fps: 60,
  inspection_min_triangles: 100000, inspection_max_triangles: 250000,
  city_triangles: 800000, city_models: 8, city_payload_bytes: 86400000,
  arsenal_triangles: 1200000 });
const WORKLOAD = Object.freeze({ samples: 31, viewport: { width: 1920, height: 1080 }, dpr: 1,
  details: ['standard', 'low'], views: VIEWS, centre: { lon: 2.3522, lat: 48.8566 },
  map_canvas: { width:1920,height:792,css_width:1920,css_height:792 },
  presets: { standard:{relief:true,borders:true,provinces:true,cities:true,labels:true,features:true,grid:false},
    low:{relief:false,borders:true,provinces:true,cities:false,labels:true,features:false,grid:false} },
  settle_ms: 3000, active_ms: 12000, controls: CONTROLS, controls_start_zoom: 8,
  control_selector: '[data-map-action="{action}"]', native_renew_budget: true,
  native_certified: true, gpu_completion: 'real WebGL finish after actual draw',
  layouts: [{ width: 390, height: 844 }, { width: 3440, height: 1440 }],
  browser_launch: { headless: true, channel: 'msedge', extra_flags: [] } });

function need(value, message) { assert.ok(value, message); }
function same(a, b, message) { assert.deepStrictEqual(a, b, message); }
function num(n, label, min = 0) { need(typeof n === 'number' && Number.isFinite(n) && n >= min, label + ' must be finite and >= ' + min); return n; }
function integer(n, label, min = 0) { num(n, label, min); need(Number.isSafeInteger(n), label + ' must be an integer'); return n; }
function text(s, label) { need(typeof s === 'string' && s.length > 0, label + ' required'); return s; }
function sha(s, label) { need(typeof s === 'string' && /^[a-f0-9]{64}$/.test(s), label + ' SHA256 required'); return s; }
function fullRevision(s) { need(typeof s === 'string' && /^[a-f0-9]{40}$/.test(s), 'Full candidate revision required'); return s; }
function json(file) { return JSON.parse(fs.readFileSync(file, 'utf8').replace(/^\uFEFF/, '')); }
function bytesHash(bytes) { return crypto.createHash('sha256').update(bytes).digest('hex'); }
function fileHash(file) {
  const fd = fs.openSync(file, 'r'), hash = crypto.createHash('sha256'), buffer = Buffer.allocUnsafe(4 * 1024 * 1024);
  try { for (;;) { const n = fs.readSync(fd, buffer, 0, buffer.length, null); if (!n) break; hash.update(buffer.subarray(0, n)); } }
  finally { fs.closeSync(fd); }
  return hash.digest('hex');
}
function canonical(p) { return path.resolve(p).replace(/\\/g, '/').replace(/\/$/, '').toLowerCase(); }
function samePath(a, b, label) { same(canonical(a), canonical(b), label); }
function within(parent, child) { const rel = path.relative(parent, child); return rel !== '' && !rel.startsWith('..' + path.sep) && rel !== '..' && !path.isAbsolute(rel); }
function artifact(parent, relative) {
  text(relative, 'Artifact filename'); need(!path.isAbsolute(relative), 'Artifact filename must be relative');
  const file = path.resolve(parent, relative); need(within(parent, file), 'Artifact path escapes output root'); return file;
}
function fileRecord(file) { file = fs.realpathSync(file); const stat = fs.statSync(file); need(stat.isFile() && stat.size > 0, 'Nonempty file required: ' + file); return { path: file, bytes: stat.size, sha256: fileHash(file) }; }
function git(repo, args) { return cp.execFileSync('git', ['-C', repo, ...args], { encoding: 'utf8', windowsHide: true }).trim(); }
function runtimeTree(repo, revision) { return Object.fromEntries(RUNTIME.map(p => [p, git(repo, ['rev-parse', revision + ':' + p])])); }
function cleanSource(repo, revision, harness) {
  same(runtimeTree(repo, 'HEAD'), runtimeTree(repo, revision), 'Runtime tree changed; documentation-only commits are allowed');
  same(git(repo, ['status', '--porcelain', '--untracked-files=all', '--', ...RUNTIME, ...harness]), '', 'Runtime or harness has uncommitted changes');
}
function utc(value, label) { const n = Date.parse(text(value, label)); need(Number.isFinite(n), label + ' is not a date'); return n; }
function calendar(value) { need(Array.isArray(value) && value.length === 3, 'Expected actual [year,month,day]'); return value.map((n, i) => String(integer(n, 'Calendar', 1)).padStart(i ? 2 : 4, '0')).join('-'); }
function nextDate(date, days) { const value = new Date(date + 'T00:00:00Z'); value.setUTCDate(value.getUTCDate() + days); return value.toISOString().slice(0, 10); }
function inputDate(file) {
  const value = json(file); same(value.format, 'spheres-campaign', 'Campaign envelope');
  same(value.world?.format, 'spheres-integrated-save', 'Integrated save envelope');
  const w = value.world.world; need(w && typeof w === 'object', 'Actual saved world required');
  return calendar([w.year, w.month, w.day ?? 1]);
}
function summary(values) {
  need(Array.isArray(values) && values.length > 0, 'Nonempty raw measurements required');
  values.forEach(n => num(n, 'Raw measurement')); const sorted = [...values].sort((a, b) => a - b);
  return { samples: sorted.length, median_ms: sorted[Math.floor(sorted.length / 2)], p95_ms: sorted[Math.ceil(sorted.length * .95) - 1], max_ms: sorted.at(-1) };
}
function close(a, b, label) { num(a, label); num(b, label); need(Math.abs(a - b) <= Math.max(1, Math.abs(b)) * 1e-12, label + ' contradicts raw arithmetic'); }
function summaryMatches(actual, expected, label) { for (const key of Object.keys(expected)) same(actual?.[key], expected[key], label + '.' + key); }
function stamp(row, manifest) { const start = utc(row.started_utc, 'started_utc'), end = utc(row.finished_utc, 'finished_utc'); need(start >= utc(manifest.frozen_utc, 'freeze time') && end >= start, 'Measurements must start after the immutable freeze'); return { start, end }; }

function validateProtocol(p) {
  same(p.schema, 'spheres-s22-measurement-protocol/v1'); same(p.status, 'frozen_before_qualification');
  same(p.cases.map(c => ({ id: c.id, date: c.anchor_date, end: c.sample_end_date })), CASES);
  same(p.cases.map(c => c.samples), [31, 31, 31]); same(p.acceptance.rounds_required, ROUNDS);
  same(p.native.samples_per_case, 31); same(p.native.percentile.p95_rank_for_31, 30);
  same(p.native.limits, { simulation_plus_history_p95_ms: 300, whole_server_turn_p95_ms: 400,
    whole_server_turn_max_ms: 750, observed_os_peak_working_set_bytes: 1073741824, max_sampled_private_bytes: 1073741824 });
  same(p.browser.map_fps_minimum, 30); same(p.browser.ui_controls.p95_limit_ms, 200);
  same(p.browser.ui_controls.samples_per_case_profile, 31); same(p.browser.ui_controls.ordered_controls, CONTROLS);
  same(p.browser.ui_controls.starting_zoom, 8); same(p.browser.inspection.existing_fps_target, 60);
  same(p.browser.inspection.minimum_triangles, 100000); same(p.browser.settle_seconds_each_view, 3);
  same(p.browser.measured_active_navigation_seconds_each_view, 12);
}
function validateHardware(h) {
  same(h.reference_id, 'SPHERES-REF-WIN-01'); need(/Ryzen 7 9800X3D/i.test(h.cpu), 'Reference CPU required');
  need(/RTX 5070/i.test(h.gpu), 'Reference GPU required'); integer(h.physical_ram_bytes, 'Physical RAM', 1);
  text(h.os, 'OS/build'); text(h.gpu_driver, 'GPU driver'); text(h.power_background_observations, 'Power/background observations');
  text(h.browser?.version, 'Browser version'); same(h.browser.channel, 'msedge');
  same(h.browser.headless, true); same(h.browser.extra_flags, []);
}
function validateLineage(lineage, source, cases, evidence, frozenUtc) {
  same(lineage.schema,'spheres-s22-reviewed-lineage/v1');same(lineage.status,'reviewed');
  text(lineage.reviewed_by,'Actual lineage reviewer');need(utc(lineage.reviewed_utc,'Lineage review time')<=utc(frozenUtc,'Freeze time'),'Lineage must be reviewed before qualification');
  text(lineage.review_scope,'Lineage review scope and limitation');same(lineage.original_source_sha256,source.sha256);
  same(lineage.adopted_early_sha256,cases[0].input.sha256);
  same(lineage.inputs,cases.map(c=>({id:c.id,actual_date:c.date,sha256:c.input.sha256})),'Reviewed lineage input identities');
  need(Array.isArray(lineage.links)&&lineage.links.length>=3,'Reviewed source/adoption/preparation links required');
  const proofs=new Map(evidence.map(e=>[canonical(e.path),e])),nodes=new Map([[source.sha256,CASES[0].date]]);
  let adoption=0;
  for(const link of lineage.links){
    sha(link.from_sha256,'Lineage parent');sha(link.to_sha256,'Lineage child');
    same(nodes.get(link.from_sha256),link.from_date,'Lineage links must follow reachable retained checkpoints');
    need(/^\d{4}-\d{2}-\d{2}$/.test(link.to_date)&&link.to_date>=link.from_date,'Lineage calendar cannot move backwards');
    need(!nodes.has(link.to_sha256),'Lineage child identity repeated');
    need(['completed','checkpoint_retained'].includes(link.outcome),'Completed preparation or retained interrupted checkpoint must be explicit');
    need(Array.isArray(link.evidence)&&link.evidence.length>0,'Every lineage link needs immutable reviewed evidence');
    for(const item of link.evidence){const record=proofs.get(canonical(item.path));need(record,'Lineage references an unfrozen evidence file');same(item.sha256,record.sha256,'Lineage evidence hash');}
    if(link.kind==='ordinary_competition_adoption'){
      adoption++;same(link.from_sha256,source.sha256);same(link.to_sha256,cases[0].input.sha256);same(link.from_date,CASES[0].date);same(link.to_date,CASES[0].date);
      same(link.command,'EnableEconomicCompetition');same(link.price_pc,0);
    }else {need(['native_preparation','native_preparation_resume'].includes(link.kind),'Unknown lineage link kind');need(link.to_date>link.from_date,'Preparation must actually age the campaign');}
    nodes.set(link.to_sha256,link.to_date);
  }
  same(adoption,1,'Exactly one ordinary competition adoption required');
  for(const c of cases)same(nodes.get(c.input.sha256),c.date,'Every dated qualification input must be reached from the earned source');
  return {reviewed_by:lineage.reviewed_by,reviewed_utc:lineage.reviewed_utc,links:lineage.links.length,
    scope:'Hashes and recorded checkpoint/date linkage independently checked against an explicit reviewed provenance record. This does not independently replay the intervening campaign years.'};
}
function validateCells(cells) {
  need(Array.isArray(cells) && cells.length === 18, 'Declare exactly 18 cells before launch');
  const ids = new Set(), tuples = new Set(), paths = [];
  for (const cell of cells) {
    need(/^[a-zA-Z0-9_-]+$/.test(cell.id), 'Stable cell ID required'); need(!ids.has(cell.id), 'Duplicate cell ID'); ids.add(cell.id);
    need(ROUNDS.includes(cell.round) && CASES.some(c => c.id === cell.case) && ROLES.includes(cell.role), 'Unknown cell tuple');
    const tuple = [cell.round, cell.case, cell.role].join('/'); need(!tuples.has(tuple), 'Duplicate cell tuple'); tuples.add(tuple);
    need(path.isAbsolute(cell.output_path), 'Cell output path must be absolute');
    for (const previous of paths) need(canonical(previous) !== canonical(cell.output_path) && !within(previous, cell.output_path) && !within(cell.output_path, previous), 'Cell output roots overlap');
    paths.push(cell.output_path);
  }
}
function registryRows(file) { return fs.existsSync(file) ? fs.readFileSync(file, 'utf8').trim().split(/\r?\n/).filter(Boolean).map(JSON.parse) : []; }
function exclusiveJson(file, value) { const fd = fs.openSync(file, 'wx'); try { fs.writeFileSync(fd, JSON.stringify(value, null, 2) + '\n'); fs.fsyncSync(fd); } finally { fs.closeSync(fd); } }

function freeze(configPath, manifestPath) {
  need(!fs.existsSync(manifestPath), 'Freeze destination exists; never overwrite a declared pair');
  const configRecord = fileRecord(configPath), config = json(configPath);
  same(config.schema, 'spheres-s22-qualification-config/v1'); need(/^[a-zA-Z0-9_-]+$/.test(config.attempt_id), 'Stable attempt_id required');
  const repo = fs.realpathSync(config.repository), revision = fullRevision(config.candidate_revision);
  const harness = [...new Set([...HARNESS, ...(config.additional_harness_files || [])])];
  harness.forEach(p => { need(!path.isAbsolute(p) && within(repo, path.resolve(repo, p)), 'Harness path must stay in repository'); });
  cleanSource(repo, revision, harness); same(git(repo, ['rev-parse', revision]), revision);
  const protocol = fileRecord(config.protocol_path); validateProtocol(json(protocol.path));
  const hardware = fileRecord(config.hardware_path); validateHardware(json(hardware.path));
  const source = fileRecord(config.source_path); same(source.sha256, json(protocol.path).source_campaign.uncompressed_sha256, 'Original earned source must be preserved');
  need(Array.isArray(config.provenance_paths) && config.provenance_paths.length > 0, 'Adoption/preparation provenance required');
  const provenance = config.provenance_paths.map(fileRecord);
  same(config.cases?.map(c => c.id), CASES.map(c => c.id));
  const cases = config.cases.map((c, i) => { const input = fileRecord(c.input_path); same(inputDate(input.path), CASES[i].date, 'Input has actual frozen date'); return { ...CASES[i], input }; });
  need(new Set(cases.map(c => c.input.sha256)).size === 3, 'Three distinct dated actual inputs required');
  const lineage=fileRecord(config.lineage_path);validateLineage(json(lineage.path),source,cases,provenance,new Date().toISOString());
  validateCells(config.cells); config.cells.forEach(c => need(!fs.existsSync(c.output_path), 'Output already exists; preflights cannot become qualification: ' + c.output_path));
  const binaries = { native_test: fileRecord(config.binaries.native_test), server: fileRecord(config.binaries.server) };
  const files = Object.fromEntries([...harness, ...ASSETS.map(n => 'spheres-web/ui/' + n)].map(p => [p, fileRecord(path.resolve(repo, p))]));
  same(files['tools/campaign/s22-qualification.cjs'].sha256, fileHash(__filename), 'Executing verifier must match the frozen verifier artifact');
  const registryPath = path.resolve(text(config.registry_path, 'Shared registry path'));
  fs.mkdirSync(path.dirname(registryPath), { recursive: true });
  const lock = registryPath + '.lock', lockFd = fs.openSync(lock, 'wx');
  try {
    const prior = registryRows(registryPath); need(!prior.some(r => r.attempt_id === config.attempt_id), 'Attempt ID already reserved');
    for (const row of prior) for (const old of row.output_paths) for (const cell of config.cells)
      need(canonical(old) !== canonical(cell.output_path) && !within(old, cell.output_path) && !within(cell.output_path, old), 'Output path already reserved by another pair');
    config.cells.forEach(c => need(!fs.existsSync(c.output_path), 'Output appeared during freeze'));
    const manifest = { schema: SCHEMA, attempt_id: config.attempt_id, frozen_utc: new Date().toISOString(),
      registry_path: registryPath, repository: repo, candidate_revision: revision, runtime_tree: runtimeTree(repo, revision),
      config: configRecord, protocol, hardware, source, provenance, lineage, binaries, files, cases, cells: config.cells,
      limits: LIMITS, workload: WORKLOAD, rounds: ROUNDS,
      policy: 'Both complete rounds independently pass. No replacement, pooled percentile, promoted preflight, or selected retry. Any changed identity starts a new full pair.' };
    exclusiveJson(manifestPath, manifest);
    const record = { schema: 'spheres-s22-attempt-reservation/v1', attempt_id: manifest.attempt_id, frozen_utc: manifest.frozen_utc,
      manifest_path: path.resolve(manifestPath), manifest_sha256: fileHash(manifestPath), output_paths: manifest.cells.map(c => c.output_path) };
    const fd = fs.openSync(registryPath, 'a'); try { fs.writeFileSync(fd, JSON.stringify(record) + '\n'); fs.fsyncSync(fd); } finally { fs.closeSync(fd); }
    return manifest;
  } finally { fs.closeSync(lockFd); fs.unlinkSync(lock); }
}

function identity(manifest, manifestPath) {
  same(manifest.schema, SCHEMA); same(manifest.limits, LIMITS); same(manifest.workload, WORKLOAD); same(manifest.rounds, ROUNDS);
  validateCells(manifest.cells); same(manifest.cases.map(c => ({ id: c.id, date: c.date, end: c.end })), CASES);
  const reservations = registryRows(manifest.registry_path).filter(r => r.attempt_id === manifest.attempt_id);
  same(reservations.length, 1, 'Exactly one original reservation required'); same(reservations[0].manifest_sha256, fileHash(manifestPath), 'Frozen manifest changed');
  samePath(reservations[0].manifest_path, manifestPath, 'Manifest moved from reserved identity');
  same(reservations[0].output_paths, manifest.cells.map(c => c.output_path));
  cleanSource(manifest.repository, manifest.candidate_revision, Object.keys(manifest.files).filter(p => p.startsWith('tools/')));
  same(runtimeTree(manifest.repository, 'HEAD'), manifest.runtime_tree, 'Runtime tree differs from frozen pair');
  const records = [manifest.config, manifest.protocol, manifest.hardware, manifest.source, manifest.lineage, ...manifest.provenance,
    ...Object.values(manifest.binaries), ...Object.values(manifest.files), ...manifest.cases.map(c => c.input)];
  for (const record of records) same(fileRecord(record.path), record, 'Frozen file changed: ' + record.path);
  validateProtocol(json(manifest.protocol.path)); validateHardware(json(manifest.hardware.path));
  const lineage=validateLineage(json(manifest.lineage.path),manifest.source,manifest.cases,manifest.provenance,manifest.frozen_utc);
  return { files_rehashed: records.length, runtime_tree_unchanged: true, lineage };
}
function certified(facts, reviewed = false) {
  same(facts.player, 'France'); need(/^[0-9a-f]{16}$/.test(facts.world_fnv64), 'Full-world fingerprint required');
  for (const key of ['daily_simulation','ideology_blocs','historical_party_leadership','resource_market','logistics_routes',
    'physical_logistics','military_operations','production_system','manufacturing_system','industry_rebuild','fiscal_recovery','economic_competition']) same(facts.rules?.[key], true, 'Certified rule ' + key);
  same(facts.rules.operational_warfare, 1);
  for (const key of ['campaign_initialized','party_leadership','population','sector_contractors']) same(facts.capabilities?.[key], true, 'Certified capability ' + key);
  same(facts.capabilities.supplier_operations_version, 1);
  for (const key of ['economic_competition','economic_active','supplier_catalogue_enabled','supplier_catalogue_active','military_enabled','military_active']) same(facts.autonomous_policies?.[key], true, 'Active AI ' + key);
  if (reviewed) for (const key of ['economic_reviews','supplier_reviewed_plans','military_reviews']) integer(facts.autonomous_policies[key], 'Executed ' + key, 1);
}
function nativeCell(manifest, cell, fixture) {
  const root = cell.output_path, runner = json(path.join(root, 'runner-result.json')), profile = json(path.join(root, 'profile.json'));
  same(runner.format, 'spheres-s22-runner/v1'); same(runner.mode, 'measure'); same(runner.passed, true); same(runner.failures, []);
  same(runner.numerical_acceptance, true); same(runner.exit_code, 0); same(runner.source_unchanged, true);
  same(runner.candidate_revision, manifest.candidate_revision); same(runner.test_binary_sha256, manifest.binaries.native_test.sha256);
  same(runner.wrapper_sha256, manifest.files['tools/campaign/run-s22-profile.ps1'].sha256);
  samePath(runner.test_binary, manifest.binaries.native_test.path); samePath(runner.original_input, fixture.input.path); samePath(runner.evidence_root, root);
  samePath(runner.copied_input, path.join(root, 'input.json')); samePath(runner.profile_json, path.join(root, 'profile.json'));
  for (const key of ['original_input_sha256_before','original_input_sha256_after','copied_input_sha256_before','copied_input_sha256_after']) same(runner[key], fixture.input.sha256, key);
  same(fileHash(runner.copied_input), fixture.input.sha256, 'Measured copied input changed');
  same(runner.arguments, ['performance::s22_measure_checkpoint','--ignored','--exact','--nocapture','--test-threads=1']);
  same(runner.child_environment, { SPHERES_S22_INPUT: runner.copied_input, SPHERES_S22_OUT: runner.profile_json,
    SPHERES_S22_RENEW_BUDGET: '1', SPHERES_S22_REQUIRE_CERTIFIED: '1' });
  same(profile.format, 'spheres-s22-profile/v1'); same(profile.revision, manifest.candidate_revision.slice(0,12));
  same(profile.mode, 'measure_input'); same(profile.passed, true); same(profile.failure, null); same(profile.source_unchanged, true);
  same(profile.certified_profile_required, true); same(profile.renew_existing_budget, true); samePath(profile.input, runner.copied_input);
  same(profile.input_bytes, fixture.input.bytes); need(/^[a-f0-9]{16}$/.test(profile.input_fnv64), 'Input fingerprint required');
  same(calendar(profile.starting.calendar), fixture.date); certified(profile.starting);
  same(profile.samples.length, 31); same(profile.control_commands.length, 31);
  for (const [i, sample] of profile.samples.entries()) {
    same(sample.index, i); same(profile.control_commands[i].index, i); need(Array.isArray(profile.control_commands[i].commands), 'Native commands retained');
    same(calendar(sample.facts.calendar), nextDate(fixture.date, i+1), 'Consecutive native daily sample'); certified(sample.facts, i >= 29);
    for (const field of ['simulation_history_ms','state_read_model_ms','state_serialization_ms','history_delta_serialization_ms','whole_turn_ms']) num(sample[field], field);
    need(sample.whole_turn_ms >= sample.simulation_history_ms, 'Whole turn cannot exclude simulation');
    const stages=sample.simulation_history_ms+sample.state_read_model_ms+sample.state_serialization_ms+sample.history_delta_serialization_ms;
    need(sample.whole_turn_ms+Math.max(1,stages)*1e-12>=stages,'Whole turn cannot exclude sequential timed stages');
    integer(sample.state_bytes, 'State read bytes', 1); integer(sample.history_delta_bytes, 'History delta bytes', 1);
  }
  same(profile.final, profile.samples.at(-1).facts); same(calendar(profile.final.calendar), fixture.end);
  const sim = summary(profile.samples.map(s => s.simulation_history_ms)), turn = summary(profile.samples.map(s => s.whole_turn_ms));
  for (const field of ['simulation_history_ms','state_read_model_ms','state_serialization_ms','history_delta_serialization_ms','whole_turn_ms'])
    summaryMatches(profile.summaries?.[field], summary(profile.samples.map(s=>s[field])), 'Native profile summary ' + field);
  same(runner.frozen_limits, { simulation_history_p95_ms:300,whole_turn_p95_ms:400,whole_turn_max_ms:750,sampled_private_bytes:1073741824,observed_os_peak_working_set_bytes:1073741824 });
  summaryMatches(runner.latency.simulation_history, sim, 'Runner simulation'); summaryMatches(runner.latency.whole_turn, turn, 'Runner turn');
  need(sim.p95_ms <= LIMITS.sim_p95_ms, 'Simulation/history p95 exceeds 300 ms');
  need(turn.p95_ms <= LIMITS.turn_p95_ms, 'Whole-turn p95 exceeds 400 ms'); need(turn.max_ms <= LIMITS.turn_max_ms, 'Whole-turn max exceeds 750 ms');
  same(profile.batch.ordinary_days, 31); same(profile.batch.same_final_facts, true); same(profile.batch.final, profile.final, 'Independent batch final world/facts differ');
  same(runner.latency.batch, profile.batch); num(profile.batch.elapsed_ms, 'Actual batch interval', Number.MIN_VALUE);
  close(profile.batch.days_per_second, 31000/profile.batch.elapsed_ms, 'Batch throughput');
  same(profile.batch.control_commands, profile.control_commands.map(c => c.commands).filter(c => c.length), 'Independent batch command sequence');
  const lines = fs.readFileSync(path.join(root, 'memory-samples.csv'), 'utf8').replace(/^\uFEFF/, '').trim().split(/\r?\n/);
  same(lines.shift(), 'elapsed_ms,private_bytes,working_set_bytes,os_peak_working_set_bytes');
  const memory = lines.map(line => { const fields=line.split(',');same(fields.length,4);need(fields.every(f=>/^\d+(?:\.\d+)?$/.test(f)),'Memory CSV requires nonempty numeric observations');
    const values=fields.map(Number);num(values[0],'Memory elapsed');values.slice(1).forEach(n=>integer(n,'Observed memory bytes',1));return values; });
  need(memory.length > 0, 'Missing memory observations'); same(runner.memory.samples, memory.length);
  for (let i = 1; i < memory.length; i++) need(memory[i][0] >= memory[i-1][0], 'Memory samples out of order');
  const privateMax = Math.max(...memory.map(r => r[1])), workingMax = Math.max(...memory.map(r => r[2])), osMax = Math.max(...memory.map(r => r[3]));
  same(runner.memory.max_sampled_private_bytes, privateMax); same(runner.memory.max_sampled_working_set_bytes, workingMax);
  // The runner reads OS peak before rejecting an otherwise unavailable sample.
  // Never discard a higher observed peak just because that row lacked private bytes.
  num(runner.memory.os_peak_working_set_bytes, 'Observed OS peak', osMax);
  if (runner.memory.failed_samples === 0) same(runner.memory.os_peak_working_set_bytes, osMax);
  need(privateMax <= LIMITS.memory_bytes && runner.memory.os_peak_working_set_bytes <= LIMITS.memory_bytes, 'Native observed memory exceeds 1 GiB');
  num(runner.memory.nominal_interval_ms, 'Memory interval', Number.MIN_VALUE); need(runner.memory.nominal_interval_ms <= 100, 'Memory sampling coarser than frozen nominal 100 ms');
  integer(runner.memory.failed_samples, 'Failed sample disclosure'); num(runner.memory.max_observed_sample_gap_ms, 'Observed gap disclosure');
  const csvGap=memory.slice(1).reduce((largest,row,i)=>Math.max(largest,row[0]-memory[i][0]),0);
  // CSV elapsed is F3 while the retained runner gap has full clock precision.
  need(Math.abs(csvGap-runner.memory.max_observed_sample_gap_ms)<=.001001,'Reported memory gap contradicts raw F3 sample timestamps');
  num(runner.wall_seconds,'Runner wall seconds',Number.MIN_VALUE);need(memory.at(-1)[0]<=runner.wall_seconds*1000+.000501,'Memory timestamp exceeds runner interval');
  return { ...stamp(runner, manifest), simulation: sim, whole_turn: turn, memory: runner.memory,
    batch_elapsed_ms: profile.batch.elapsed_ms, batch_days_per_second: profile.batch.days_per_second,
    final_world_fnv64: profile.final.world_fnv64,
    evidence_files:['runner-result.json','profile.json','memory-samples.csv'].map(name=>fileRecord(path.join(root,name))) };
}

function humanDate(iso) { const [year, month, day] = iso.split('-').map(Number); return day + ' ' + ['Jan','Feb','Mar','Apr','May','Jun','Jul','Aug','Sep','Oct','Nov','Dec'][month-1] + ' ' + year; }
function gpu(row) {
  same(row?.reference_hardware_match, true); need(typeof row.version === 'string' && row.version.startsWith('WebGL'), 'Actual WebGL version required');
  need(/ANGLE/i.test(row.unmasked || '') && /NVIDIA.*RTX\s*5070\b/i.test(row.unmasked || '') && !/SwiftShader|llvmpipe|software|WARP|Microsoft Basic/i.test(row.unmasked), 'Actual frozen RTX 5070 GPU required');
}
function texture(row) {
  for (const key of ['probe_errors','unmeasured_allocation_events','unmeasured_texture_allocations']) same(row?.[key], 0, 'Incomplete texture accounting');
  num(row.declared_texture_texel_payload_bytes, 'Declared texel payload'); num(row.peak_declared_texture_texel_payload_bytes, 'Peak declared texel payload');
}
function buffers(row){
  for(const key of ['resident_buffer_payload_bytes','peak_buffer_payload_bytes','live_buffers','buffer_data_calls','buffer_data_payload_bytes','draw_calls','submitted_triangles','offscreen_submitted_triangles','context_losses'])integer(row?.[key],'Queried buffer '+key);
  need(row.peak_buffer_payload_bytes>=row.resident_buffer_payload_bytes,'Buffer high water below current payload');
}
function processMemory(row){
  utc(row?.captured_utc,'Browser process memory capture');text(row.scope,'Browser memory method');
  need(Array.isArray(row.browser_owned_processes)&&row.browser_owned_processes.length>0,'Browser process identities missing');
  const ids=new Set(row.browser_owned_processes.map(p=>integer(p.id,'Browser process ID',1)));
  const counters=Array.isArray(row.windows_counters)?row.windows_counters:[row.windows_counters];need(counters.length>0,'Browser process observations missing');
  for(const p of counters){need(ids.has(p?.Id),'Memory observation is not a disposable browser process');for(const key of ['PrivateMemorySize64','WorkingSet64','PeakWorkingSet64'])integer(p[key],'Browser '+key);need(p.PeakWorkingSet64>=p.WorkingSet64,'OS high water below working set');}
}
function mapMemory(row){
  need(Array.isArray(row?.metrics?.metrics),'CDP metrics missing');const metrics=new Map(row.metrics.metrics.map(m=>[m.name,m.value]));
  for(const name of ['JSHeapUsedSize','JSHeapTotalSize'])num(metrics.get(name),'Observed '+name);
  for(const name of ['documents','nodes','jsEventListeners'])integer(row.dom?.[name],'Observed DOM '+name);
  need(row.contexts?.length>0,'Actual map allocation observations missing');for(const c of row.contexts){buffers(c.buffer_payload);texture(c.declared_texture_payload);same(c.texture_diagnostics,[]);}
}
function cameraChange(key, before, after) {
  for (const c of [before, after]) for (const k of ['yaw','pitch','zoom']) need(typeof c?.[k] === 'number' && Number.isFinite(c[k]), 'Finite actual camera values required');
  const yaw = Math.atan2(Math.sin(after.yaw-before.yaw), Math.cos(after.yaw-before.yaw));
  if (key === 'west' || key === 'east') { need(key === 'west' ? yaw < 0 : yaw > 0, 'Control must change yaw in requested direction'); same(after.pitch, before.pitch); same(after.zoom, before.zoom); }
  else { need(key === 'zoom-in' ? after.zoom > before.zoom : after.zoom < before.zoom, 'Control must change zoom in requested direction'); same(after.yaw, before.yaw); same(after.pitch, before.pitch); }
}
function inputs(rows) {
  same(rows?.length, 31, 'Exactly 31 ordered controls required');
  for (const [i, row] of rows.entries()) {
    same(row.key, CONTROLS[i%4], 'Control sequence changed'); same(row.trusted, true);
    num(row.event_timestamp, 'Trusted event timestamp'); num(row.start, 'Listener entry'); num(row.complete, 'Completion');
    need(row.start >= row.event_timestamp && row.complete >= row.start, 'Control timestamps out of order');
    close(row.elapsed_ms, row.complete-row.event_timestamp, 'Raw control latency'); cameraChange(row.key, row.before_globe, row.after_globe);
    if (i) { need(row.event_timestamp >= rows[i-1].complete, 'Control observations overlap'); same(row.before_globe, rows[i-1].after_globe, 'Camera changed outside ordered workload'); }
    else same(row.before_globe.zoom, 8, 'Controls must start at frozen national zoom');
  }
  const result = summary(rows.map(r => r.elapsed_ms)); need(result.p95_ms <= 200, 'Raw control p95 exceeds 200 ms'); return result;
}
function mapPhase(phase, duration) {
  num(phase?.elapsed_ms, 'Map phase elapsed', duration); need(Array.isArray(phase.frames) && phase.frames.length > 0, 'Actual completed map frames required');
  let previous = 0;
  for (const frame of phase.frames) {
    num(frame.start_ms, 'Draw start'); num(frame.complete_ms, 'Draw completion', frame.start_ms);
    need(frame.start_ms >= previous && frame.complete_ms <= phase.elapsed_ms, 'Map frame lies outside full measured window');
    close(frame.draw_ms, frame.complete_ms-frame.start_ms, 'Draw duration'); close(frame.interval_ms, frame.complete_ms-previous, 'Completion interval');
    integer(frame.draw_calls, 'Actual map draw calls', 1); integer(frame.city_draw_calls, 'Actual city draws'); num(frame.triangles, 'Submitted triangles', 1);
    previous = frame.complete_ms;
  }
  same(previous, phase.elapsed_ms, 'Map measurement must retain its final completed frame');
  return phase.frames.length*1000/phase.elapsed_ms;
}
function validateMap(result) {
  same(result.cells?.length, 2); need(!result.renderer, 'Renderer output cannot fill a map cell'); const verdict = [];
  for (const [i, cell] of result.cells.entries()) {
    same(cell.detail, ['standard','low'][i]); same(cell.viewport, WORKLOAD.viewport); same(cell.dpr, 1); same(cell.views?.length, 4);
    for (const [j, view] of cell.views.entries()) {
      same(view.view, VIEWS[j]); same(view.map_mode, 'terrain'); same(view.map_details,WORKLOAD.presets[cell.detail]);same(view.active_canvas,WORKLOAD.map_canvas,'Actual map drawable must match frozen DPR1 layout');
      same(view.ready, true); same(view.workload_valid, true); same(view.passed, true); same(view.buffers.context_losses, 0); texture(view.declared_texture_payload);
      mapPhase(view.settle, 3000); const fps = mapPhase(view.measured, 12000); same(view.fps, fps, 'Stored map FPS contradicts full raw window'); need(fps >= 30, 'Actual completed map FPS below 30');
      if (i === 0 && j >= 2) same(view.terrain.ready, true);
      if (i === 0 && j === 3) need(view.measured.frames.some(f => f.city_draw_calls > 0), 'City geometry must actually draw');
      if (i === 1) need(view.measured.frames.every(f => f.city_draw_calls === 0), 'Low-detail workload must disable city draws');
      verdict.push({ detail: cell.detail, view: view.view.name, fps, frames: view.measured.frames.length, elapsed_ms: view.measured.elapsed_ms });
    }
    same(cell.input_passed, true); same(cell.input_workload_valid, true); const control = inputs(cell.inputs);
    same(cell.input_summary.count, 31); same(cell.input_summary.p95, control.p95_ms); same(cell.input_summary.max, control.max_ms);
  }
  for (const memory of [result.memory_before, result.memory_after, ...result.cells.map(c => c.memory)]) {
    mapMemory(memory);
  }
  return { views: verdict, controls: result.cells.map(c => ({ detail: c.detail, ...summary(c.inputs.map(i => i.elapsed_ms)) })) };
}
function visiblePanel(panel, viewport) {
  text(panel?.text?.trim(), 'Visible panel text'); same(panel.viewport, viewport); num(panel.rect.width, 'Panel width', Number.MIN_VALUE); num(panel.rect.height, 'Panel height', Number.MIN_VALUE);
  need(panel.rect.left >= 0 && panel.rect.right <= viewport.width+1 && panel.rect.top >= 0 && panel.rect.bottom <= viewport.height+1, 'Panel framing failed');
  need(panel.scroll_width <= panel.client_width+1 && panel.document_width <= viewport.width+1, 'Panel horizontal overflow');
}
function validateLayout(layout, output) {
  same(layout?.passed, true); same(layout.profiles?.length, 2);
  for (const [i, row] of layout.profiles.entries()) {
    const viewport = WORKLOAD.layouts[i]; same(row.viewport, viewport); same(row.dpr, 1); same(row.passed, true); same(row.map.viewport, viewport);
    need(row.map.document_width <= viewport.width+1, 'Map horizontal overflow'); need(row.map.controls?.length > 0, 'Map controls absent');
    for (const c of row.map.controls) need(c.left >= 0 && c.right <= viewport.width+1 && c.top >= 0 && c.bottom <= viewport.height+1 && c.width > 0 && c.height > 0, 'Control framing failed');
    same(row.keyboard.action, 'west'); cameraChange('west', row.keyboard.before, row.keyboard.after);
    same(row.touch.action, 'east'); cameraChange('east', row.touch.before, row.touch.after);
    same(row.focus_return?.length, 3); row.focus_return.forEach(r => same(r.passed, true));
    same(row.focus_return[2].visible, true); same(row.focus_return[2].inert, false); need(row.focus_return[2].tag !== 'BODY', 'Focus fell to body');
    for (const panel of [row.province,row.equipment]) {
      visiblePanel(panel.before, viewport); visiblePanel(panel.after, viewport);
      if (panel.before.scroll_height > panel.before.client_height+1) { same(panel.scrolled, true); need(panel.before.scroll_top !== panel.after.scroll_top, 'Scrollable panel did not scroll'); }
    }
    const reading = row.province.reading; for (const field of ['name','date','owner']) text(reading[field], 'Province ' + field);
    need(row.province.after.text.includes(reading.owner) && row.province.after.text.includes(reading.date), 'Current province reading absent');
    same(row.checks.length, 6); same(row.screenshots.length, 2); row.screenshots.forEach(f => need(fs.statSync(artifact(output, f)).size > 0, 'Layout screenshot missing'));
  }
}
function validateCold(result) {
  const cold = result.cold_loading; same(cold?.fresh_context, true); num(cold.menu_navigation_to_usable_ms, 'Cold menu boundary'); num(cold.campaign_save_load_to_usable_map_ms, 'Save-load boundary');
  same(cold.menu_navigation_to_usable_ms, result.cold_menu_ms); same(cold.campaign_save_load_to_usable_map_ms, result.campaign_save_load_to_usable_map_ms);
  need(cold.after_map?.contexts?.length > 0 && cold.final_observation?.contexts?.length > 0, 'First actual draw observations missing');
  same(cold.after_map.phases[0].name, 'uncached-page-entry');
  const loadPhase = cold.after_map.phases.find(p => p.name === 'campaign-save-load'); need(loadPhase, 'Campaign load boundary missing');
  const draws = cold.after_map.contexts.flatMap(c => c.first_draw_by_phase || []).filter(d => d.phase === 'campaign-save-load' && d.canvas_id === 'glmap');
  need(draws.length > 0, 'No observed actual map draw after save load');
  for (const d of draws) { num(d.count, 'Actual draw vertices', 1); num(d.completed_ms, 'Completed first map draw', loadPhase.at_ms); same(d.connected, true); }
  need(Array.isArray(cold.resource_entries) && cold.resource_entries.length > 0, 'Cold resource observations missing');
  need(Array.isArray(result.network?.events) && result.network.events.some(e => e.kind === 'request') && result.network.events.some(e => e.kind === 'response'), 'Raw network observations missing');
}
function rendererPhase(phase, duration) {
  same(phase?.requested_ms, duration); num(phase.elapsed_ms, 'Orbit full interval', duration); same(phase.draw_frames, phase.raw_frames?.length);
  need(phase.draw_frames > 0, 'Actual completed viewer draws required'); same(phase.animation_callbacks, phase.raw_animation_ticks?.length);
  let previous = 0;
  for (const frame of phase.raw_frames) {
    num(frame.started_ms, 'Viewer draw start', previous); num(frame.completed_ms, 'Viewer completed draw', frame.started_ms);
    need(frame.completed_ms <= phase.elapsed_ms, 'Viewer frame outside full measured interval');
    close(frame.callback_and_gpu_ms, frame.completed_ms-frame.started_ms, 'Viewer draw duration');
    num(frame.gpu_finish_wait_ms, 'Actual GPU finish wait'); need(frame.gpu_finish_wait_ms <= frame.callback_and_gpu_ms, 'GPU wait larger than draw');
    integer(frame.draw_calls, 'Viewer actual draw calls', 1); num(frame.submitted_triangles, 'Viewer submitted geometry', 1); previous = frame.completed_ms;
  }
  let lastTick = 0, lastDraws = 0;
  for (const tick of phase.raw_animation_ticks) { num(tick.elapsed_ms, 'Animation clock', lastTick); integer(tick.completed_draw_frames, 'Completed draw counter', lastDraws); need(tick.completed_draw_frames <= phase.draw_frames, 'Animation counter exceeds draws'); lastTick = tick.elapsed_ms; lastDraws = tick.completed_draw_frames; }
  same(lastTick, phase.elapsed_ms); same(lastDraws, phase.draw_frames);
  const fps = phase.draw_frames*1000/phase.elapsed_ms; same(phase.completed_draw_fps, fps, 'Viewer FPS must use full window'); return fps;
}
function validateOrbit(orbit){
  same(orbit.platform,'air_fighter');same(orbit.viewport,{width:1920,height:1080,dpr:1});same(orbit.target_fps,60);same(orbit.passed,true);
  rendererPhase(orbit.warmup,3000);const fps=rendererPhase(orbit.sample,12000);need(fps>=60,'Actual full-window inspection FPS below 60');return fps;
}
function rendererSnapshot(row) {
  same(row.city.cap_triangles, 800000); same(row.city.cap_count, 8); need(row.city.entries.length <= 8, 'City cache model cap exceeded');
  const triangles = row.city.entries.reduce((n, e) => n + integer(e.triangles, 'City triangles'), 0);
  need(triangles <= 800000, 'City cache triangle cap exceeded'); same(row.city.resident, triangles*108); num(row.city.peak, 'City peak payload'); need(row.city.peak <= 86400000, 'Transient city payload cap exceeded');
  same(row.arsenal.cap, 1200000); num(row.arsenal.triangles, 'Arsenal retained triangles'); need(row.arsenal.triangles <= 1200000, 'Arsenal cap exceeded');
  need(Array.isArray(row.contexts) && row.contexts.length > 0, 'Actual renderer contexts required');
  for (const context of row.contexts) { buffers(context.metrics);texture(context.declared_texture_payload); same(context.texture_diagnostics, []); same(context.diagnostics, []); }
  need(Object.hasOwn(row,'js_heap'),'JS heap availability must be explicit');if(row.js_heap!==null)for(const key of ['used','total','limit'])num(row.js_heap?.[key],'Observed JS heap '+key);
}
function validateRenderer(result, root, manifest) {
  same(result.cells, [], 'Map cells cannot substitute for renderer qualification'); const proof = result.renderer;
  same(proof?.passed, true); same(proof.memory_complete, true); same(proof.errors, []); need(!proof.failure, 'Renderer failure retained');
  same(proof.build.revision, manifest.candidate_revision.slice(0,12)); gpu(proof.reference_gpu); same(proof.after_state, proof.before_state, 'Renderer navigation changed native state');
  need(Array.isArray(proof.requests) && proof.requests.every(r => r.path === '/api/equipment-preview'), 'Renderer issued an unexpected mutation');
  same(proof.checks?.length, 3); same(proof.inspection_loads?.length, 6);
  const records = new Map();
  for (const entry of proof.observations) {
    need(!records.has(entry.file), 'Duplicate renderer observation'); const file = artifact(root, entry.file), stat = fs.statSync(file);
    same(stat.size, entry.bytes); same(fileHash(file), sha(entry.sha256, 'Observation')); records.set(entry.file, file);
  }
  const read = name => { const file = records.get(name + '.json'); need(file, 'Missing renderer observation: ' + name); return json(file); };
  for (const [file, target] of records) {
    if (file.includes('state-before') || file.includes('state-after')) continue;
    const observation = json(target); if (observation.city && observation.arsenal) rendererSnapshot(observation);
  }
  const tour = read('s22-city-tour-complete'); need(tour.city.deletions > 0 && tour.city.uploaded > 86400000, 'Actual city eviction workload missing');
  same(read('s22-city-revisit').city.uploads, tour.city.uploads, 'City revisit failed cache reuse');
  same(read('s22-cities-disabled').map.details.cities, false); same(read('s22-low-detail').map.details.relief, false);
  for (const name of ['tokyo','new-york','mexico-city','mumbai','sao-paulo','delhi']) read('s22-city-' + name);
  for (const kind of ['globe','arsenal','inspection']) {
    const recovery = read('s22-' + kind + '-context-recovery'); same(recovery.lost.buffers.live_buffers, 0); same(recovery.lost.textures.live_textures, 0);
    same(recovery.lost.textures.allocated_texture_levels, 0); same(recovery.lost.textures.declared_texture_texel_payload_bytes, 0); rendererSnapshot(recovery.after);
    const restored = recovery.after.contexts.find(c => c.id === recovery.before.id); need(restored?.metrics.live_buffers > 0, 'Lost context not restored');
  }
  const meshes = [];
  for (let i = 0; i < 6; i++) {
    same(proof.inspection_loads[i].visit, i); num(proof.inspection_loads[i].elapsed_ms, 'Inspection entry duration');
    const city = read('s22-city-room-closed-' + i); same(city.arsenal.mounted, 0); same(city.arsenal.pending, 0);
    const mesh = read('s22-aircraft-mesh-' + i); same(mesh.platform, 'air_fighter'); same(mesh.spec.platform, 'air_fighter');
    integer(mesh.triangles, 'Inspection mesh triangles', 100000); need(mesh.triangles <= 250000, 'Existing inspection ceiling exceeded'); meshes.push(mesh.triangles);
    const visit = read('s22-aircraft-visit-' + i), contexts = visit.contexts.filter(c => c.type === 'webgl' && c.connected);
    same(contexts.length, 1); need(contexts[0].metrics.draw_calls > 0, 'Inspection geometry never drew');
    for (const context of read('s22-aircraft-closed-' + i).contexts.filter(c => c.type === 'webgl')) {
      same(context.metrics.live_buffers, 0); same(context.declared_texture_payload.live_textures, 0); same(context.declared_texture_payload.allocated_texture_levels, 0); same(context.declared_texture_payload.declared_texture_texel_payload_bytes, 0);
    }
  }
  for (const [name, pending, mounted] of [['pending',1,0],['visible',0,1],['removed',0,0]]) { const row = read('s22-lazy-card-' + name); same(row.arsenal.pending, pending); same(row.arsenal.mounted, mounted); }
  read('s22-renderer-final'); same(read('s22-renderer-state-before'), proof.before_state); same(read('s22-renderer-state-after'), proof.after_state);
  const orbit = proof.viewer_orbit; same(orbit, read('s22-fighter-orbit-performance')); const fps=validateOrbit(orbit);
  const cold = proof.cold_inspection; same(cold?.first_entry, true); same(cold.before_city_lifecycle_work, true);
  same(cold.entry_phase.name, 'inspection-first-entry'); same(cold.fighter_phase.name, 'inspection-first-fighter');
  const draw = cold.first_fighter_draw; same(draw.phase, cold.fighter_phase.name); same(draw.design_platform, 'air_fighter');same(draw.rendered_platform,'air_fighter');text(draw.model_key,'Mounted inspection model identity'); same(draw.inspection_canvas, true); same(draw.connected, true);
  need(draw.count > 0 && draw.completed_ms >= cold.fighter_phase.at_ms && cold.fighter_phase.at_ms >= cold.entry_phase.at_ms, 'Invalid actual first fighter draw boundary');
  same(cold.navigation_to_first_fighter_draw_ms, draw.completed_ms); close(cold.entry_to_first_fighter_draw_ms, draw.completed_ms-cold.entry_phase.at_ms, 'First inspection boundary');
  const sources = new Map(); for (const s of proof.sources) { need(!sources.has(s.path), 'Duplicate renderer source'); need(manifest.files[s.path], 'Unfrozen renderer source'); same(s.sha256, manifest.files[s.path].sha256); same(s.bytes, manifest.files[s.path].bytes); sources.set(s.path,s); }
  for (const asset of ASSETS) need(sources.has('spheres-web/ui/' + asset), 'Missing served renderer asset ' + asset);
  return { viewer_fps: fps, frames: orbit.sample.draw_frames, elapsed_ms: orbit.sample.elapsed_ms, inspection_triangles: meshes, observations: records.size };
}
function browserChild(parent) {
  const children = fs.readdirSync(parent, { withFileTypes: true });
  same(children.length, 1, 'Browser parent must contain exactly one declared attempt, including failures');
  need(children[0].isDirectory() && /^browser-[a-zA-Z0-9]+$/.test(children[0].name), 'Expected a single browser-* child'); return path.join(parent, children[0].name);
}
function normalizedSaveHash(file) { const zlib = require('node:zlib'), value = JSON.parse(zlib.gunzipSync(fs.readFileSync(file))); delete value.saved_unix; return bytesHash(JSON.stringify(value)); }
function validateTrace(result,root){
  need(!result.trace_failure,'Trace failure contradicts passed result');same(result.trace?.file,'chrome-trace.json.gz');num(result.trace.raw_bytes,'Raw trace bytes',1);integer(result.trace.compressed_bytes,'Compressed trace bytes',1);
  const trace=artifact(root,result.trace.file);same(fs.statSync(trace).size,result.trace.compressed_bytes);same(fileHash(trace),sha(result.trace.sha256,'Trace'));
  same(require('node:zlib').gunzipSync(fs.readFileSync(trace)).length,result.trace.raw_bytes,'Trace must decode to its recorded raw byte count');
}
function browserCell(manifest, cell, fixture) {
  const root = browserChild(cell.output_path), result = json(path.join(root, 'result.json')), hardware = json(manifest.hardware.path);
  same(result.passed, true); same(result.qualification, true); same(result.errors, []); need(!result.failure && !result.trace_failure, 'Failed browser attempt cannot pass');
  same(result.revision, manifest.candidate_revision); same(result.build.revision, manifest.candidate_revision.slice(0,12));
  same(result.binary_sha256, manifest.binaries.server.sha256); same(result.checkpoint_sha256, fixture.input.sha256); samePath(result.out, root); samePath(result.run, path.join(root,'server'));
  same(fileHash(path.join(root,'server/saves/s22-input.json')), fixture.input.sha256, 'Loaded campaign copy changed');
  same(result.campaign.date, humanDate(fixture.date)); same(result.campaign.player, 'France'); text(result.campaign.session_id, 'Native campaign session');
  same(result.browser, hardware.browser.version); same(result.browser_launch, WORKLOAD.browser_launch); gpu(result.reference_gpu); same(result.memory_complete, true);
  processMemory(result.browser_process_memory_before);processMemory(result.browser_process_memory_after);
  for (const [field, name, copy] of [['driver_sha256','s22-performance-browser.cjs','driver.cjs'],['probe_sha256','webgl-measurement.js','webgl-measurement.js'],['texture_probe_sha256','webgl-texture-measurement.js','webgl-texture-measurement.js']]) {
    same(result[field], manifest.files['tools/ui/' + name].sha256); same(fileHash(path.join(root,copy)), result[field]);
  }
  same(result.harness_dependencies?.length, 3);
  for (const name of ['s22-browser-contract.cjs','s22-browser-observation.js','s22-layout-browser.cjs']) {
    const rows = result.harness_dependencies.filter(r => r.name === name); same(rows.length,1); same(rows[0].sha256, manifest.files['tools/ui/' + name].sha256); same(fileHash(path.join(root,name)), rows[0].sha256);
  }
  same(result.assets?.length, 5);
  for (const name of ['index.html','map-controls.js','globe3d.js','arsenal3d.js','equipment-model.js']) { const rows=result.assets.filter(a=>a.name===name); same(rows.length,1); same(rows[0].sha256, manifest.files['spheres-web/ui/' + name].sha256); }
  need(Array.isArray(result.requests) && result.requests.every(r => ['/api/save','/api/load','/api/program-preview','/api/equipment-preview'].includes(r.path)), 'Unexpected order/time mutation during browser observation');
  sha(result.before_save, 'Before save'); same(result.before_save, result.after_save); same(normalizedSaveHash(path.join(root,'s22-before.json.gz')), result.before_save);
  same(normalizedSaveHash(path.join(root,'s22-after.json.gz')), result.after_save, 'Complete saved world/history changed');
  validateTrace(result,root);
  need(result.screenshots?.length > 0, 'Screenshots missing'); result.screenshots.forEach(f=>need(fs.statSync(artifact(root,f)).size>0,'Empty screenshot'));
  validateCold(result); validateLayout(result.layout_functional,root);
  const details=cell.role==='map'?validateMap(result):validateRenderer(result,root,manifest);
  return {...stamp(result,manifest),output:root,result_file:fileRecord(path.join(root,'result.json')),trace_sha256:result.trace.sha256,...details};
}
function verify(manifestPath) {
  const manifest=json(manifestPath), report={schema:'spheres-s22-verdict/v1',attempt_id:manifest.attempt_id,
    checked_utc:new Date().toISOString(),passed:false,status:'incomplete',identity:null,cells:[],rounds:[],failures:[]};
  try { report.identity=identity(manifest,manifestPath); }
  catch(error) { report.status='invalid_identity'; report.failures.push(error.message); return report; }
  for (const cell of manifest.cells) {
    const row={id:cell.id,round:cell.round,case:cell.case,role:cell.role,output_path:cell.output_path,passed:false,status:'incomplete'}; report.cells.push(row);
    if(!fs.existsSync(cell.output_path)) { row.reason='Declared output not created'; continue; }
    try {
      const fixture=manifest.cases.find(c=>c.id===cell.case);
      row.observed=cell.role==='native'?nativeCell(manifest,cell,fixture):browserCell(manifest,cell,fixture);
      row.passed=true;row.status='passed';
    } catch(error) { row.status=error.code==='ENOENT'?'incomplete':'failed';row.reason=error.message; }
  }
  // Full measured windows must not overlap: frozen reference uses one lane at a time.
  const finished=report.cells.filter(c=>c.passed).sort((a,b)=>a.observed.start-b.observed.start);
  for(let i=1;i<finished.length;i++) if(finished[i].observed.start<finished[i-1].observed.end) report.failures.push('Concurrent cells: '+finished[i-1].id+' and '+finished[i].id);
  for(const round of ROUNDS) { const rows=report.cells.filter(c=>c.round===round); report.rounds.push({round,passed:rows.length===9&&rows.every(c=>c.passed),complete:rows.length===9&&rows.every(c=>c.status!=='incomplete'),cells:rows.map(c=>c.id)}); }
  if(report.rounds.every(r=>r.passed)) {
    const first=finished.filter(c=>c.round===ROUNDS[0]),second=finished.filter(c=>c.round===ROUNDS[1]);
    if(Math.min(...second.map(c=>c.observed.start))<Math.max(...first.map(c=>c.observed.end))) report.failures.push('Confirmation began before initial full round completed');
    for(const fixture of CASES) { const runs=finished.filter(c=>c.role==='native'&&c.case===fixture.id); if(runs[0].observed.final_world_fnv64!==runs[1].observed.final_world_fnv64) report.failures.push('Native final world differs between unchanged rounds: '+fixture.id); }
  }
  report.passed=report.rounds.every(r=>r.passed)&&report.failures.length===0;
  report.status=report.passed?'passed':report.failures.length||report.cells.some(c=>c.status==='failed')?'failed':'incomplete';
  return report;
}

module.exports = { CASES, ROUNDS, ROLES, LIMITS, WORKLOAD, HARNESS, ASSETS, VIEWS, CONTROLS,
  freeze, verify, identity, validateCells, validateProtocol, validateHardware, nativeCell, browserCell,
  validateMap, validateRenderer, validateLayout, validateCold, validateOrbit, validateTrace, validateLineage, inputs, mapPhase, rendererPhase, processMemory, mapMemory,
  summary, fileHash, fileRecord, inputDate, browserChild, certified };

if (require.main === module) {
  try {
    const [command,...args]=process.argv.slice(2), flags={};
    need(args.length%2===0,'Use named --config/--manifest/--out arguments');
    for(let i=0;i<args.length;i+=2) { need(['--config','--manifest','--out'].includes(args[i])&&!flags[args[i]],'Unknown or duplicate option');flags[args[i]]=args[i+1]; }
    text(flags['--manifest'],'--manifest');
    const result=command==='freeze'?freeze(text(flags['--config'],'--config'),flags['--manifest']):command==='verify'?verify(flags['--manifest']):assert.fail('Expected freeze or verify');
    if(flags['--out'])exclusiveJson(flags['--out'],result);
    process.stdout.write(JSON.stringify(result,null,2)+'\n');
    if(command==='verify'&&!result.passed)process.exitCode=1;
  } catch(error) { process.stderr.write(error.stack+'\n');process.exitCode=1; }
}
