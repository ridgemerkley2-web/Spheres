'use strict';
// Load explicitly reviewed harness bytes at the original module filename.
// This preserves __dirname and normal relative/node_modules resolution without
// changing repository files, require.cache, globals, browser APIs or responses.
const assert = require('node:assert/strict');
const fs = require('node:fs');
const path = require('node:path');
const crypto = require('node:crypto');
const Module = require('node:module');
const args = process.argv.slice(2);
assert.equal(args.length, 5, 'Use ORIGINAL PATCHED ORIGINAL_SHA PATCHED_SHA EXECUTION_PROOF');
const [originalName, patchedName, expectedOriginal, expectedPatched, proofName] = args;
assert.match(expectedOriginal, /^[a-f0-9]{64}$/);
assert.match(expectedPatched, /^[a-f0-9]{64}$/);
const original = path.resolve(originalName), patched = path.resolve(patchedName);
assert.notEqual(original, patched, 'The overlay must not overwrite the tracked harness');
const hash = bytes => crypto.createHash('sha256').update(bytes).digest('hex');
const originalBytes = fs.readFileSync(original), patchedBytes = fs.readFileSync(patched);
assert.equal(hash(originalBytes), expectedOriginal, 'Tracked harness bytes differ from the reviewed source');
assert.equal(hash(patchedBytes), expectedPatched, 'Corrected harness bytes differ from the reviewed source');
const compiled = new Module(original, module);
compiled.filename = original;
compiled.paths = Module._nodeModulePaths(path.dirname(original));
const proof = {
  captured_utc: new Date().toISOString(),
  scope: 'Exact bytes loaded for Module._compile; browser acceptance is established only by its actual result and exit status.',
  original_filename: original,
  original_sha256: expectedOriginal,
  evaluated_source: patched,
  evaluated_sha256: hash(patchedBytes),
  module_filename: compiled.filename,
  module_directory: path.dirname(original),
  node_executable: process.execPath,
  node_versions: process.versions,
  argv: process.argv,
};
fs.writeFileSync(path.resolve(proofName), JSON.stringify(proof, null, 2) + '\n', {flag: 'wx'});
compiled._compile(patchedBytes.toString('utf8'), original);
compiled.loaded = true;
