'use strict';
// Verify the lossless packet without restoring its multi-gigabyte raw files.
// Optional restoration writes only beneath a new destination, never original paths.
const fs = require('node:fs');
const path = require('node:path');
const crypto = require('node:crypto');
const zlib = require('node:zlib');
const cp = require('node:child_process');
const assert = require('node:assert/strict');
const root = __dirname;
const manifestBytes = fs.readFileSync(path.join(root, 'manifest.json'));
const manifest = JSON.parse(manifestBytes);
const repo = cp.execFileSync('git', ['rev-parse', '--show-toplevel'], { cwd: root, encoding: 'utf8' }).trim();
const sha = b => crypto.createHash('sha256').update(b).digest('hex');
const args = process.argv.slice(2);
assert(args.length === 0 || (args.length === 2 && args[0] === '--restore'), 'Use: node archive.cjs [--restore NEW_DIRECTORY]');
const destination = args.length ? path.resolve(args[1]) : null;
if (destination) assert(!fs.existsSync(destination), 'Restore destination must not exist');
const payloads = new Map(manifest.payloads.map(p => [p.sha256, p]));
assert.equal(payloads.size, manifest.payloads.length, 'Duplicate payload IDs');
const originals = new Set();
for (const f of manifest.files) {
  const key = f.original_path.replaceAll('\\', '/').toLowerCase();
  assert(!originals.has(key), 'Duplicate original path');
  originals.add(key);
}
function enclosed(base, relative) {
  assert(!path.isAbsolute(relative) && !relative.split(/[\\/]/).includes('..'), 'Unsafe archive path');
  const p = path.resolve(base, relative);
  assert(p.startsWith(path.resolve(base) + path.sep), 'Archive path escaped root');
  return p;
}
function payloadBytes(id) {
  const p = payloads.get(id);
  assert(p, 'Missing payload ' + id);
  return Buffer.concat(p.chunks.map(c => fs.readFileSync(enclosed(root, c.path))));
}
function repositoryBytes(relative) {
  const p = enclosed(repo, relative);
  return fs.existsSync(p) ? fs.readFileSync(p) : cp.execFileSync('git', ['show', 'HEAD:' + relative], { cwd: repo, maxBuffer: 256 * 1024 * 1024 });
}
function decoded(f) {
  if (f.encoding === 'repository-gzip') {
    const gzip = repositoryBytes(f.repository_path);
    assert.equal(gzip.length, f.compressed_bytes);
    assert.equal(sha(gzip), f.compressed_sha256);
    return zlib.gunzipSync(gzip);
  }
  const b = payloadBytes(f.payload);
  if (f.encoding === 'gzip') return zlib.gunzipSync(b);
  assert.equal(f.encoding, 'identity');
  return b;
}
let chunks = 0;
for (const p of payloads.values()) {
  const h = crypto.createHash('sha256');
  let bytes = 0;
  for (const c of p.chunks) {
    const b = fs.readFileSync(enclosed(root, c.path));
    assert.equal(b.length, c.bytes, c.path);
    assert.equal(sha(b), c.sha256, c.path);
    assert(b.length <= manifest.chunk_limit_bytes, 'Oversized Git chunk');
    h.update(b); bytes += b.length; chunks++;
  }
  assert.equal(bytes, p.bytes, p.sha256);
  assert.equal(h.digest('hex'), p.sha256);
}
const checked = new Set();
const external = [];
for (const f of manifest.files) {
  if (f.encoding === 'external-binary') { external.push({ path: f.original_path, bytes: f.bytes, sha256: f.sha256 }); continue; }
  const key = [f.encoding, f.payload || f.repository_path, f.bytes, f.sha256].join('|');
  if (!checked.has(key)) {
    const raw = decoded(f);
    assert.equal(raw.length, f.bytes, f.original_path);
    assert.equal(sha(raw), f.sha256, f.original_path);
    checked.add(key);
  }
}
const pairFiles = manifest.files.filter(f => /[\\/]evidence[\\/]s22-qualification-pair-02[\\/]/.test(f.original_path));
assert.equal(pairFiles.length, manifest.original_pair_files);
assert.equal(pairFiles.reduce((sum, f) => sum + f.bytes, 0), manifest.original_pair_bytes);
const mirrored = [
  ['records/independent-review.md', /[\\/]evidence[\\/]s22-pair02-independent-review.md$/],
  ['records/independent-summary.json', /[\\/]evidence[\\/]s22-pair02-independent-summary.json$/],
  ['records/independent-verdict.json', /[\\/]evidence[\\/]s22-pair02-independent-verdict.json$/],
  ['records/verdict.json', /[\\/]s22-qualification-pair-02-runner[\\/]verdict.json$/],
  ['records/run-summary.json', /[\\/]s22-qualification-pair-02-runner[\\/]run-summary.json$/],
  ['records/frozen-manifest.json', /[\\/]evidence[\\/]s22-qualification-pair-02-manifest.json$/],
];
for (const [name, match] of mirrored) {
  const f = manifest.files.find(f => match.test(f.original_path)); assert(f, name);
  const b = fs.readFileSync(enclosed(root, name)); assert.equal(b.length, f.bytes); assert.equal(sha(b), f.sha256);
}
// Verification finishes before restoration starts. Files are create-only.
if (destination) {
  fs.mkdirSync(destination);
  const mappings = [];
  for (const f of manifest.files) {
    if (f.encoding === 'external-binary') continue;
    const portable = f.original_path.replaceAll('\\', '/').replace(/^([A-Za-z]):\//, '$1/').replace(/^\/+/, '');
    const target = enclosed(destination, portable);
    fs.mkdirSync(path.dirname(target), { recursive: true });
    const raw = decoded(f);
    assert.equal(raw.length, f.bytes); assert.equal(sha(raw), f.sha256);
    fs.writeFileSync(target, raw, { flag: 'wx' });
    mappings.push({ original: f.original_path, restored: target, bytes: f.bytes, sha256: f.sha256 });
  }
  fs.writeFileSync(path.join(destination, 'restore-map.json'), JSON.stringify({ recorded_qualification_passed: manifest.qualification_passed, qualification_rerun: false, mappings, external_binaries: external }, null, 2) + '\n', { flag: 'wx' });
}
console.log(JSON.stringify({
  archive_verified: true, recorded_qualification_passed: manifest.qualification_passed, qualification_rerun: false,
  manifest_sha256: sha(manifestBytes), files: manifest.files.length,
  original_pair_files: pairFiles.length, original_pair_bytes: manifest.original_pair_bytes,
  payloads: payloads.size, chunks,
  stored_payload_bytes: manifest.payloads.reduce((n, p) => n + p.bytes, 0),
  independently_decoded_encodings: checked.size,
  external_binary_bodies_verified: false, external_binaries: external,
  restored: destination,
}, null, 2));
