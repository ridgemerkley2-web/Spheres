'use strict';
// S24 successor-fixture harness checks (preparation only). Pure file reads: no
// server, browser, network or campaign. Validates the committed 23-row
// inventory against current sources, and the committed raw results for
// missing/duplicate identities and mislabelled or inconsistent outcomes.
const { test } = require('node:test');
const assert = require('node:assert/strict');
const fs = require('node:fs');
const path = require('node:path');
const cp = require('node:child_process');
const zlib = require('node:zlib');
const lib = require(process.env.S24_HARNESS_MODULE_DIR ? path.join(process.env.S24_HARNESS_MODULE_DIR, 'lib.cjs') : './successor-fixtures/lib.cjs');
const harness = require(process.env.S24_HARNESS_MODULE_DIR ? path.join(process.env.S24_HARNESS_MODULE_DIR, 'run.cjs') : './successor-fixtures/run.cjs');
const { buildInventory } = require('./successor-fixtures/inventory.cjs');

const ROOT = path.resolve(__dirname, '../..');
const DIR = path.join(ROOT, 'docs/campaign-certification/S24/preparation/successor-fixtures');
const EVIDENCE = path.join(DIR, 'evidence');
const readJson = file => JSON.parse(fs.readFileSync(file, 'utf8'));
const inventory = () => readJson(path.join(DIR, 'inventory.json'));
const results = () => readJson(path.join(EVIDENCE, 'result.json'));
const clone = value => JSON.parse(JSON.stringify(value));
const options = { requireFinal: true, allowHistoricalHashOnly: true, exists: f => fs.existsSync(path.join(EVIDENCE, f)), hashOf: f => lib.fileSha256(path.join(EVIDENCE, f)) };
const identityView = inv => ({ expectation: inv.expectation.successor_identities,
  rows: inv.rows.map(r => [r.id, r.parent, r.dissolution, r.player_continuation, r.native_activation_path]) });

test('committed inventory identities and activation paths match the current sources', () => {
  // Identity-level only, so unrelated roster or wording edits do not fail
  // this suite; `inventory.cjs --check` compares the full inventory.
  assert.deepEqual(identityView(inventory()), identityView(buildInventory()),
    'Regenerate with node tools/ui/successor-fixtures/inventory.cjs --write and review its discrepancies');
});

test('the inventory holds each of the 23 expected successor identities exactly once', () => {
  const inv = inventory();
  assert.equal(inv.expectation.successor_identities, 23);
  assert.equal(inv.rows.length, 23);
  assert.equal(new Set(inv.rows.map(r => r.id)).size, 23);
  const countries = readJson(path.join(ROOT, 'docs/campaign-certification/C01/countries.json'));
  const census = readJson(path.join(ROOT, 'docs/campaign-certification/C01/census.json'));
  assert.deepEqual(inv.rows.map(r => r.id).sort(), countries.filter(r => r.start_1990 === false).map(r => r.id).sort());
  assert.equal(census.counts.successor_nations, inv.rows.length);
  assert.deepEqual(lib.validateInventory(inv), []);
});

test('mapping discrepancies are flagged without reducing the expectation', () => {
  const inv = inventory();
  const gap = inv.discrepancies.find(d => d.kind === 'no_native_activation');
  assert(gap, 'The missing activation path must be an explicit discrepancy');
  assert.deepEqual(gap.identities, ['EastTimor', 'Namibia']);
  assert.equal(gap.expectation_unchanged, true);
  assert.equal(gap.proposal, lib.PROPOSAL);
  assert(fs.existsSync(path.join(ROOT, lib.PROPOSAL)), 'The integration proposal must exist');
  for (const id of gap.identities) {
    const row = inv.rows.find(r => r.id === id);
    assert.equal(row.native_activation_path, false);
    assert.deepEqual(row.recipes, ['G0']);
  }
  assert.equal(inv.counts.native_activation_paths + inv.counts.no_native_hook, inv.expectation.successor_identities);
  const russia = inv.rows.find(r => r.id === 'Russia');
  assert.equal(russia.parent, 'USSR'); assert.equal(russia.continuation_state, true); assert.equal(russia.certified_case, true);
});

test('the inventory validator rejects missing, duplicate, misclassified and unflagged identities', () => {
  const base = inventory();
  const expectError = (mutate, pattern) => { const inv = clone(base); mutate(inv); assert.match(lib.validateInventory(inv).join('\n'), pattern); };
  expectError(inv => { inv.rows = inv.rows.filter(r => r.id !== 'Ukraine'); }, /Missing successor identity: Ukraine/);
  expectError(inv => { inv.rows.push(clone(inv.rows.find(r => r.id === 'Croatia'))); }, /Duplicate successor identity: Croatia/);
  expectError(inv => { inv.rows.find(r => r.id === 'Estonia').roster_start_1990 = true; }, /Estonia is not a successor/);
  expectError(inv => { inv.discrepancies = inv.discrepancies.filter(d => d.kind !== 'no_native_activation'); }, /Namibia has no native activation path but no discrepancy flags it/);
  expectError(inv => { inv.rows.find(r => r.id === 'Namibia').recipes.push('H1', 'B1'); }, /Namibia recipes disagree/);
  expectError(inv => { inv.expectation.successor_identities = 21; }, /the expectation is 21/);
});

test('committed results are complete, labelled and tied to this exact inventory and build', () => {
  const r = results();
  assert.deepEqual(lib.validateResults(r, inventory(), options), []);
  assert.equal(r.development_run, false);
  assert.match(r.build.expected_revision, /^[0-9a-f]{40}$/);
  assert.equal(r.build.served_revision, r.build.expected_revision.slice(0, 12));
  assert.equal(r.build.runtime_source_equal_to_expected, true);
  assert.equal(r.cleanup.server_stopped, true);
  assert.deepEqual(r.cleanup.remaining_saves, [], 'Disposable campaigns must be deleted after hashing');
});

test('historical submitted results pin the exact historical harness blobs', () => {
  // Preserve the authored run; later fixes get separate evidence. These old
  // receipts contain no restorable input and do not meet current acceptance.
  const recorded = results().harness.files_sha256_lf;
  assert.equal(results().harness.tree_clean_for_harness_and_runtime, true);
  for (const file of ['run.cjs', 'lib.cjs', 'inventory.cjs']) {
    const rel = 'tools/ui/successor-fixtures/' + file;
    const git = cp.spawnSync('git', ['show', results().harness.revision + ':' + rel], { cwd: ROOT });
    assert.equal(git.status, 0, 'Historical source must remain available in Git');
    const lf = git.stdout;
    assert.equal(recorded[rel], lib.sha256(lf), rel + ' historical source pin differs');
  }
});

test('at least three representative activations ran, including USSR -> Russia, with inspection after save/reload', () => {
  const r = results(), by = (layer, id) => r.cases.find(c => c.layer === layer && c.identity === id);
  const passedBrowser = r.cases.filter(c => c.layer === 'browser_ui' && c.status === 'passed');
  assert(passedBrowser.length >= 3, 'At least three executed browser cases');
  assert(passedBrowser.some(c => c.identity === 'Russia'));
  const russia = by('activation_save_load', 'Russia');
  assert.equal(russia.status, 'passed');
  const names = new Set(russia.checks.map(k => k.name));
  for (const stage of ['after_activation', 'after_reload'])
    for (const part of ['selection_identity', 'government', 'budget', 'guidance']) assert(names.has(stage + ':' + part), stage + ':' + part);
  for (const c of passedBrowser) {
    const ui = new Set(c.checks.map(k => k.name));
    for (const stage of ['after_activation', 'after_reload'])
      for (const part of ['ui_selection_identity', 'ui_government', 'ui_budget', 'ui_guidance']) assert(ui.has(stage + ':' + part), c.identity + ' ' + stage + ':' + part);
    assert(c.screenshots.length >= 1, c.identity + ' has screenshots');
  }
  assert(r.cases.filter(c => c.layer === 'activation_save_load' && c.status === 'passed').length >= 3);
});

test('passed, failed, blocked and unrun are reported independently for every layer', () => {
  const r = results(), inv = inventory();
  for (const layer of lib.LAYERS) {
    assert.deepEqual(Object.keys(r.summary[layer]).sort(), [...lib.STATUSES].sort());
    assert.equal(Object.values(r.summary[layer]).reduce((a, b) => a + b, 0), inv.rows.length);
  }
  const noHook = r.cases.filter(c => c.reason === 'no_native_hook').map(c => c.identity + '/' + c.layer).sort();
  assert.deepEqual(noHook, ['EastTimor/activation_save_load', 'EastTimor/browser_ui', 'Namibia/activation_save_load', 'Namibia/browser_ui']);
});

test('the result validator rejects mislabelled or inconsistent results', () => {
  const base = results(), inv = inventory();
  const find = (r, id, layer) => r.cases.find(c => c.identity === id && c.layer === layer);
  const expectError = (mutate, pattern) => { const r = clone(base); mutate(r); assert.match(lib.validateResults(r, inv, options).join('\n'), pattern); };
  expectError(r => { find(r, 'Russia', 'activation_save_load').label.organic = true; }, /can never claim organic succession/);
  expectError(r => { delete find(r, 'Russia', 'browser_ui').label.disclaimer; }, /without the exact disclaimer/);
  expectError(r => { find(r, 'Croatia', 'selection_guard').label = clone(find(r, 'Croatia', 'activation_save_load').label); }, /selection guard must be labelled/);
  expectError(r => { find(r, 'Serbia', 'activation_save_load').label.kind = 'native_s21_authored_dissolution'; }, /only authors the USSR dissolution/);
  expectError(r => { Object.assign(find(r, 'Namibia', 'activation_save_load'), { status: 'passed', reason: undefined, checks: [{ name: 'forged', status: 'passed' }], label: lib.label('harness_staged_parent_collapse') }); }, /must be unrun\/no_native_hook/);
  expectError(r => { find(r, 'Russia', 'activation_save_load').checks[0].status = 'failed'; }, /passed with a failed check/);
  expectError(r => { find(r, 'Latvia', 'browser_ui').status = 'skipped'; }, /status must be one of/);
  expectError(r => { find(r, 'EastTimor', 'browser_ui').checks = [{ name: 'x', status: 'passed' }]; }, /unrun case cannot carry executed checks/);
  expectError(r => { r.cases = r.cases.filter(c => !(c.identity === 'Ukraine' && c.layer === 'activation_save_load')); }, /Missing result: Ukraine\/activation_save_load/);
  expectError(r => { r.cases.push(clone(find(r, 'Kazakhstan', 'browser_ui'))); }, /Kazakhstan\/browser_ui: duplicate result/);
  expectError(r => { r.summary.browser_ui.passed += 1; }, /Summary counts do not match/);
  expectError(r => { r.disclaimer = 'organic succession proven'; }, /disclaimer is missing or altered/);
  expectError(r => { find(r, 'Croatia', 'activation_save_load').fixture_input = 'n1-missing'; }, /without a retained fixture input/);
  expectError(r => { const f = r.fixture_inputs.find(x => x.kind === 'harness_staged_parent_collapse'); f.patch[0].to_raw = '0.5'; }, /exactly stability=0.0 and separatism=1.0/);
  expectError(r => { r.development_run = true; }, /non-development run/);
  expectError(r => { r.inventory.digest = '0'.repeat(64); }, /different inventory/);
  expectError(r => { find(r, 'Russia', 'browser_ui').screenshots[0].sha256 = 'f'.repeat(64); }, /screenshot hash differs/);
});

test('staging splices only the two parent values and preserves unrepresentable integers', () => {
  const big = '18446744073709551557'; // above 2^53: JSON.parse would round it
  const text = '{"format":"spheres-campaign","version":1,"world":{"format":"spheres-integrated-save","world":{"rng":{"state":' + big
    + '},"nations":[{"id":"Poland","alive":true,"stability":61.0,"separatism":0.1,"motto":"Łódź \u2014 😀"},{"id":"USSR","alive":true,"separatism":0.55,"stability":42.0,"name":"\\"quoted\\""}]}},"saved_unix":1}';
  const staged = lib.stageNationFields(text, 'USSR', { stability: '0.0', separatism: '1.0' });
  assert(staged.text.includes('"state":' + big));
  assert(staged.text.includes('{"id":"USSR","alive":true,"separatism":1.0,"stability":0.0,"name":"\\"quoted\\""}'));
  const at = text.indexOf('"separatism":0.55') + '"separatism":'.length;
  assert.equal(staged.patch[0].byte_offset, Buffer.byteLength(text.slice(0, at), 'utf8'), 'Patch offsets are UTF-8 byte offsets');
  assert.notEqual(staged.patch[0].byte_offset, at, 'The multibyte motto makes byte offsets differ from string indices');
  assert(staged.text.includes('"stability":61.0,"separatism":0.1'), 'Other nations are untouched');
  assert.deepEqual(staged.patch.map(p => [p.path, p.from_raw, p.to_raw]), [['/world/world/nations/1/separatism', '0.55', '1.0'], ['/world/world/nations/1/stability', '42.0', '0.0']]);
  const before = Buffer.from(text, 'utf8'), after = Buffer.from(staged.text, 'utf8');
  assert.equal(lib.verifyOnlyPatched(before, after, staged.patch), true);
  const tampered = Buffer.from(staged.text.replace('61.0', '62.0'), 'utf8');
  assert.equal(lib.verifyOnlyPatched(before, tampered, staged.patch), false, 'Any other changed byte is detected');
  assert.equal(lib.archiveProjection(text).endsWith('}}}'), true);
});

test('staging refuses missing, duplicated, dead or malformed parent records', () => {
  const wrap = nations => '{"world":{"world":{"nations":[' + nations + ']}}}';
  assert.throws(() => lib.stageNationFields(wrap('{"id":"France","alive":true,"stability":1.0,"separatism":0.0}'), 'USSR', { stability: '0.0' }), /exactly one USSR/);
  assert.throws(() => lib.stageNationFields(wrap('{"id":"USSR","alive":true,"stability":1.0},{"id":"USSR","alive":true,"stability":2.0}'), 'USSR', { stability: '0.0' }), /exactly one USSR/);
  assert.throws(() => lib.stageNationFields(wrap('{"id":"USSR","alive":false,"stability":1.0}'), 'USSR', { stability: '0.0' }), /not alive/);
  assert.throws(() => lib.stageNationFields(wrap('{"id":"USSR","alive":true,"stability":"high"}'), 'USSR', { stability: '0.0' }), /not a plain number/);
  assert.throws(() => lib.stageNationFields(wrap('{"id":"USSR","alive":true,"stability":1.0}'), 'USSR', { stability: 'NaN' }), /plain JSON numbers/);
  assert.throws(() => lib.stageNationFields('﻿' + wrap('{"id":"USSR","alive":true,"stability":1.0}'), 'USSR', { stability: '0.0' }), /byte-order mark/);
});

test('Rust readers ignore comments, strings and lifetimes', () => {
  const src = [
    '// row("Commented", "Out", &[], "Nowhere", &[], &[], false, false, false),',
    'const fn row(code: &\'static str, name: &\'static str) -> NationRow { todo!() }',
    'row("Real", "Real // not a comment", &["r"], "Somewhere", &[ /* inline */ "Other"], &[claim("Other", 0.1)], false, false, true),',
    '/* row("Blocked", "Out", &[], "X", &[], &[], true, false, false), */',
    'row("Start", "Start \\"quoted\\"", &[], "Somewhere",',
    '    // a note between arguments',
    '    &["Real"], &[], true, false, false),'
  ].join('\r\n');
  const rows = lib.rosterRows(src);
  assert.deepEqual(rows.map(r => [r.code, r.name, r.start_1990, r.major]), [['Real', 'Real // not a comment', false, true], ['Start', 'Start "quoted"', true, false]]);
  assert.deepEqual(rows[0].neighbours, ['Other']);
  const journey = 'fn successors(g: &Game) -> Vec<NationId> {\n let family: &[NationId] = match w.player {\n'
    + ' Some(NationId::USSR) if w.has_flag("ussr_dissolved") => &[NationId::Russia, // first\n NationId::Ukraine],\n _ => &[],\n };\n}';
  assert.deepEqual(lib.journeyFamilies(journey), { USSR: { flag: 'ussr_dissolved', family: ['Russia', 'Ukraine'] } });
});

test('historical exporter receipts report matching projections, without retained original bytes', () => {
  const provenance = readJson(path.join(EVIDENCE, 'native-s21-export.json'));
  assert.equal(provenance.runs.length, 2);
  assert.equal(provenance.runs[0].projection_sha256, provenance.runs[1].projection_sha256);
  assert.equal(provenance.reproducible_projection, true);
  const n1 = results().fixture_inputs.find(f => f.kind === 'native_s21_authored_dissolution');
  assert(n1, 'The USSR cases consume the existing S21 exporter fixture');
  assert.equal(n1.projection_sha256, provenance.runs[0].projection_sha256);
  assert.equal(n1.exporter.revision, results().build.expected_revision);
});

// Independent review regressions. The historical receipts above are immutable;
// these negative controls close validator/retention gaps without rewriting them.
test('current acceptance rejects historical hash-only inputs', () => {
  const errors = lib.validateResults(results(), inventory(), { ...options, allowHistoricalHashOnly: false });
  assert.match(errors.join('\n'), /retained input verification failed/);
});
test('passing requires every named check exactly once and retained browser screenshots', () => {
  const inv = inventory();
  for (const layer of lib.LAYERS) {
    const r = results(), c = r.cases.find(c => c.layer === layer && c.status === 'passed');
    c.checks = [{ name: 'arbitrary', status: 'passed' }];
    assert.match(lib.validateResults(r, inv, options).join('\n'), /missing required check/);
  }
  const r = results(), c = r.cases.find(c => c.layer === 'browser_ui' && c.status === 'passed');
  c.checks.push(clone(c.checks[0])); c.screenshots = [];
  assert.match(lib.validateResults(r, inv, options).join('\n'), /duplicate check name/);
  assert.match(lib.validateResults(r, inv, options).join('\n'), /needs retained screenshots/);
});
test('runtime, clean-tree and served asset proof cannot be waived by a passed browser case', () => {
  for (const mutate of [r => { r.build.runtime_source_equal_to_expected = false; }, r => { r.harness.tree_clean_for_harness_and_runtime = false; }, r => { r.build.served_assets_verified = { error: 'wrong asset' }; }, r => { delete r.build.served_assets_verified.assets['index.html']; }, r => { r.build.served_assets_verified.binary_sha256 = '0'.repeat(64); }]) {
    const r = results(); mutate(r);
    assert(lib.validateResults(r, inventory(), options).length > 0);
  }
});
test('native exporter provenance must pin the exact consumed archive and runtime', () => {
  for (const mutate of [f => { f.exporter.revision = '0'.repeat(40); }, f => { f.exporter.runs[0].succession_sha256 = '0'.repeat(64); }, f => { f.exporter.runs[0].bytes++; }, f => { f.exporter.test_binary_sha256 = null; }]) {
    const r = results(), f = r.fixture_inputs.find(f => f.recipe === 'N1'); mutate(f);
    assert.match(lib.validateResults(r, inventory(), options).join('\n'), /exporter provenance does not match/);
  }
});
test('failed staging cannot support a passed activation', () => {
  const r = results(), f = r.fixture_inputs.find(f => f.recipe === 'H1');
  f.status = 'failed'; f.checks[0].status = 'failed';
  assert.match(lib.validateResults(r, inventory(), options).join('\n'), /staged fixture checks did not pass/);
});
test('blocked environment errors remain blocked even before the first check', () => {
  const r = results(), c = r.cases.find(c => c.layer === 'browser_ui' && c.status === 'passed');
  Object.assign(c, { status: 'blocked', checks: [], blocked_stage: 'browser launch', error: 'executable missing', screenshots: [] });
  r.summary = lib.summarize(r.cases);
  assert.deepEqual(lib.validateResults(r, inventory(), options), []);
});
test('failed or blocked cases yield nonzero command exit even when the record is valid', () => {
  assert.equal(lib.resultExitCode({ cases: [{ status: 'passed' }, { status: 'unrun' }] }, []), 0);
  for (const status of ['failed', 'blocked']) assert.equal(lib.resultExitCode({ cases: [{ status }] }, []), 1);
  assert.equal(lib.resultExitCode({ cases: [] }, ['invalid']), 1);
});
test('unrun no-hook claims cannot hide supported activation or selection cases', () => {
  const r = results(), c = r.cases.find(c => c.identity === 'Russia' && c.layer === 'activation_save_load');
  Object.assign(c, { status: 'unrun', checks: [], reason: 'no_native_hook', proposal: lib.PROPOSAL });
  r.summary = lib.summarize(r.cases);
  assert.match(lib.validateResults(r, inventory(), options).join('\n'), /contradicts the available native path/);
});
test('retained archive verification checks gzip and decoded bytes and confines references', () => {
  const raw = Buffer.from('{"format":"spheres-campaign","world":{},"saved_unix":1}'), packed = zlib.gzipSync(raw);
  const rec = { file: 'fixtures/input.json.gz', compression: 'gzip', bytes: raw.length, sha256: lib.sha256(raw), gzip_bytes: packed.length, gzip_sha256: lib.sha256(packed) };
  assert(lib.retainedArchive(rec, () => packed).equals(raw));
  for (const file of ['../input.gz', '/input.gz', 'C:/input.gz', 'fixtures/../input.gz', 'fixtures\\input.gz']) assert.throws(() => lib.retainedArchive({ ...rec, file }, () => packed), /invalid/);
  assert.throws(() => lib.retainedArchive({ ...rec, sha256: '0'.repeat(64) }, () => packed), /archive bytes\/hash differ/);
  assert.throws(() => lib.retainedArchive(rec, () => Buffer.from('wrong')), /gzip bytes\/hash differ/);
  assert.throws(() => lib.retainedArchive(rec), /byte verification is required/);
});
test('screenshot references cannot leave the evidence directory', () => {
  const r = results(), c = r.cases.find(c => c.screenshots?.length);
  c.screenshots[0].file = '../foreign.jpg';
  assert.match(lib.validateResults(r, inventory(), { ...options, exists: () => true, hashOf: () => c.screenshots[0].sha256 }).join('\n'), /screenshot without file/);
});
test('staging rejects duplicate JSON keys instead of patching a shadowed value', () => {
  const text = '{"nations":[{"id":"USSR","alive":true,"stability":50,"stability":60,"separatism":0.5}]}';
  assert.throws(() => lib.stageNationFields(text, 'USSR', { stability: '0.0', separatism: '1.0' }), /Duplicate JSON key/);
});
test('command options reject unknown recipes and duplicate requested identities', () => {
  const base = ['--binary', 'unused.exe', '--expected-revision', '0'.repeat(40), '--out', path.resolve('unused-output')];
  for (const extra of [['--browser', 'Russia:invented'], ['--browser', 'Russia:N1:ignored'], ['--browser', 'Russia:N1,Russia:H1'], ['--api', 'Russia,Russia']]) assert.throws(() => harness.parseArgs([...base, ...extra]), /recipe|duplicate/);
});
