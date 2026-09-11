'use strict';
// Read-only input audit. Writes only the three named fresh evidence outputs.
const fs = require('node:fs');
const path = require('node:path');
const crypto = require('node:crypto');
const { spawnSync } = require('node:child_process');
const root = path.resolve(__dirname, '..');
const integration = path.join(root, 'integration');
const evidence = path.join(root, 'evidence');
const output = path.join(evidence, 'S05-preservation-runtime-final.json');
const logFile = path.join(evidence, 'S05-preservation-runtime-final.log');
const exitFile = path.join(evidence, 'S05-preservation-runtime-final.exit');
const expectedCandidate = 'db9d17c8d726aa102aa143ceb3599009c558ffee';
for (const file of [output, logFile, exitFile]) if (fs.existsSync(file)) throw Error('Fresh evidence required: ' + file);
const read = file => JSON.parse(fs.readFileSync(file, 'utf8'));
async function sha(file) {
  const hash = crypto.createHash('sha256');
  for await (const chunk of fs.createReadStream(file)) hash.update(chunk);
  return hash.digest('hex');
}
function log(message) { fs.appendFileSync(logFile, message + '\n'); process.stdout.write(message + '\n'); }
function git(cwd, args) {
  const command = ['--no-optional-locks', '-C', cwd, ...args];
  const result = spawnSync('git', command, { encoding: 'utf8', windowsHide: true });
  log(JSON.stringify({ command: ['git', ...command], status: result.status, stdout: result.stdout.trim(), stderr: result.stderr.trim() }));
  if (result.status !== 0) throw Error('Read-only git query failed.');
  return result.stdout.trim();
}
const lines = text => text ? text.split(/\r?\n/) : [];
const previousPath = path.join(__dirname, 'preservation-provenance-2618979.json');
const previous = read(previousPath);
const s01 = path.join(integration, 'docs', 'campaign-certification', 'S01');
const activeRoot = path.join(root, 'fixtures', 's05-original-active');
const activeManifestPath = path.join(activeRoot, 'active-fixture-manifest.json');
const activeProvenancePath = path.join(activeRoot, 'evidence', 'run-provenance.json');
const masterGenerationPath = path.join(root, 'fixtures', 's05-pinned-master', 'generation.json');
const record = {
  format: 'spheres-s05-preservation-provenance', version: 2,
  captured_utc: new Date().toISOString(), candidate: null, candidate_clean: false,
  expected_candidate: expectedCandidate, previous_record: previousPath,
  supporting_script: __filename, supporting_script_sha256: null,
  supporting_log: logFile, supporting_log_sha256: null, exit_file: exitFile,
  original_active_provenance: activeProvenancePath,
  protected_campaign_files: [], fixture_files: [], source_binaries: [], source_worktrees: [],
  counts: {}, actions: { user_campaign_file_writes: false, fixture_file_writes: false,
    integration_source_edits: false, server_browser_or_tests_launched: false, linux_commands_run: false,
    only_new_outputs: [output, logFile, exitFile] },
  limitations: [
    'Original active HEAD is 038fe0b; the S01 runtime/data pin is 5f7f355. The intervening recorded documentation and verification changes are listed, not relabeled as the original commit.',
    'The three active exporter binaries lack S01 historical SHA256 pins. Their current hashes are compared with the original export-run before/after hashes and unchanged original source/fingerprint evidence. Only the master producer has the recorded S01 binary SHA256 pin.',
    'This is input/source preservation evidence, not a test-suite result, gameplay balance result, or performance measurement.'
  ], passed: false, failure: null,
};
async function checkFile(file, expected, bytes, label) {
  const actual = await sha(file);
  const size = fs.statSync(file).size;
  const match = actual === expected && (bytes === undefined || size === bytes);
  log(JSON.stringify({ check: label, path: file, bytes: size, sha256: actual, expected_sha256: expected, matches: match }));
  if (!match) throw Error('Preservation mismatch: ' + file);
  return { path: file, bytes: size, sha256: actual };
}
async function run() {
  record.supporting_script_sha256 = await sha(__filename);
  record.previous_record_sha256 = await sha(previousPath);
  record.candidate = git(integration, ['rev-parse', 'HEAD']);
  record.candidate_clean = git(integration, ['status', '--porcelain']).length === 0;
  if (record.candidate !== expectedCandidate || !record.candidate_clean) throw Error('Integration is not the requested clean candidate.');
  record.s01_manifest_sha256 = (await checkFile(path.join(s01, 'manifest.json'), previous.s01_manifest_sha256, undefined, 'S01 manifest')).sha256;
  const protectedPath = path.join(s01, 'evidence', 'protected-campaigns-before.json');
  const protectedFiles = read(protectedPath);
  record.protected_campaign_manifest = protectedPath;
  record.protected_campaign_manifest_sha256 = await sha(protectedPath);
  if (protectedFiles.length !== 8) throw Error('Expected the original eight protected campaigns.');
  for (const item of protectedFiles) {
    const old = previous.protected_campaign_files.find(row => row.path === item.path);
    if (!old || old.sha256 !== item.sha256 || old.bytes !== item.bytes) throw Error('S01 campaign inventory differs from prior verified provenance.');
    record.protected_campaign_files.push({ ...await checkFile(item.path, item.sha256, item.bytes, 'Protected campaign'), matches_s01: true });
  }
  record.active_fixture_manifest_sha256 = (await checkFile(activeManifestPath, previous.active_fixture_manifest_sha256, undefined, 'Original active fixture manifest')).sha256;
  const activeManifest = read(activeManifestPath);
  const provenance = read(activeProvenancePath);
  record.original_active_provenance_sha256 = await sha(activeProvenancePath);
  if (activeManifest.fixtures.length !== 28 || activeManifest.source_revision !== '5f7f355502f17bd6bd8f0383a2d14f0024fa7884') throw Error('Unexpected active fixture count/source pin.');
  for (const item of activeManifest.fixtures) {
    const file = path.resolve(activeRoot, item.file);
    if (!file.startsWith(activeRoot + path.sep)) throw Error('Fixture path escaped its original root.');
    record.fixture_files.push({ ...await checkFile(file, item.sha256, item.bytes, 'Original active fixture'), matches_recorded_manifest: true });
  }
  record.master_generation_sha256 = (await checkFile(masterGenerationPath, previous.master_generation_sha256, undefined, 'Original master generation record')).sha256;
  const master = read(masterGenerationPath);
  record.fixture_files.push({ ...await checkFile(master.archive, master.archive_sha256, undefined, 'Original genuine master archive'), matches_recorded_manifest: true });
  for (const item of provenance.source_binaries) {
    const manifestItem = activeManifest.source_binaries.find(row => row.path === item.path);
    if (!manifestItem || manifestItem.sha256 !== item.sha256_before || item.sha256_before !== item.sha256_after) throw Error('Active binary generation provenance does not agree.');
    const binary = await checkFile(item.path, item.sha256_after, item.bytes, 'Original active producer binary');
    const sourceFile = path.join(previous.source_worktrees.find(row => row.name === 'original_active').path, item.source_path);
    const source = await checkFile(sourceFile, item.source_blob_sha256, undefined, 'Original exporter source');
    const fingerprint = await checkFile(item.fingerprint_path, item.fingerprint_sha256, undefined, 'Original exporter Cargo fingerprint');
    record.source_binaries.push({ ...binary, matches_export_generation_hash: true, historical_s01_hash_pin: false,
      source_path: source.path, source_sha256: source.sha256, fingerprint_path: fingerprint.path, fingerprint_sha256: fingerprint.sha256 });
  }
  record.source_binaries.push({ ...await checkFile(master.test_binary, master.test_binary_sha256, undefined, 'Original master producer binary'), matches_export_generation_hash: true, historical_s01_hash_pin: true });
  for (const old of previous.source_worktrees) {
    const head = git(old.path, ['rev-parse', 'HEAD']);
    const clean = git(old.path, ['status', '--porcelain']).length === 0;
    const changed = lines(git(old.path, ['diff', '--name-only', old.expected_source_revision, 'HEAD']));
    const sourceChanges = changed.filter(file => !old.changed_paths_since_source_pin.includes(file));
    if (!clean || head !== old.head || JSON.stringify(changed) !== JSON.stringify(old.changed_paths_since_source_pin) || sourceChanges.length) throw Error('Original source worktree changed since its qualified preservation record: ' + old.name);
    record.source_worktrees.push({ name: old.name, path: old.path, expected_source_revision: old.expected_source_revision,
      head, clean, head_exactly_matches_source_pin: head === old.expected_source_revision,
      changed_paths_since_source_pin: changed, runtime_and_data_paths_changed_since_pin: [],
      qualified_result: old.qualified_result });
  }
  record.counts = { protected_campaign_files: record.protected_campaign_files.length,
    original_active_fixtures: activeManifest.fixtures.length, original_master_archives: 1,
    source_binaries: record.source_binaries.length, source_worktrees: record.source_worktrees.length };
  record.candidate_after = git(integration, ['rev-parse', 'HEAD']);
  record.candidate_clean_after = git(integration, ['status', '--porcelain']).length === 0;
  if (record.candidate_after !== record.candidate || !record.candidate_clean_after) throw Error('Candidate changed during preservation audit.');
  record.passed = true;
  record.status = 'passed with explicit original-active HEAD and historical exporter-hash qualifications';
}
run().catch(error => { record.failure = error.stack || String(error); process.exitCode = 1; }).finally(async () => {
  record.finished_utc = new Date().toISOString();
  log(JSON.stringify({ passed: record.passed, candidate: record.candidate, counts: record.counts, failure: record.failure }));
  record.supporting_log_sha256 = await sha(logFile);
  fs.writeFileSync(output, JSON.stringify(record, null, 2) + '\n', { flag: 'wx' });
  fs.writeFileSync(exitFile, (record.passed ? '0' : '1') + '\n', { flag: 'wx' });
});
