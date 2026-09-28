'use strict';
// S24 successor-fixture harness: shared, dependency-free helpers.
// Pure functions only. Nothing here starts a server, opens a browser, touches
// a campaign or writes outside the paths a caller passes explicitly.
const crypto = require('node:crypto');
const fs = require('node:fs');
const path = require('node:path');
const zlib = require('node:zlib');

const FORMAT_INVENTORY = 'spheres-s24-successor-inventory/v1';
const FORMAT_RESULT = 'spheres-s24-successor-fixture-results/v1';
const STATUSES = Object.freeze(['passed', 'failed', 'blocked', 'unrun']);
const LAYERS = Object.freeze(['selection_guard', 'activation_save_load', 'browser_ui']);
const UNRUN_REASONS = Object.freeze(['no_native_hook', 'not_selected']);
const DISCLAIMER = 'FIXTURE: an authored or staged activation. It does not prove organic succession, historical timing or the S25 long-campaign route.';
const PROPOSAL = 'docs/campaign-certification/S24/preparation/successor-fixtures/integration-proposal.md';

// Every activation this harness can perform is one of these two kinds. The
// selection guard creates no successor state at all and is labelled as such.
const KINDS = Object.freeze({
  selection_guard_no_activation: Object.freeze({
    fixture: false, activation: false,
    note: 'Pre-activation guard only: the successor is refused as a 1 January 1990 start and is absent from the starter roster. No successor state is created.'
  }),
  native_s21_authored_dissolution: Object.freeze({
    fixture: true, activation: true,
    source: 'cargo test --locked --release -p spheres-web campaign_journey::tests::s21_export_review_fixtures -- --ignored --exact (succession.json)',
    staging: 'Existing S21 exporter: Game::new_fresh(1990, USSR) with fresh_play_rules and a Prosperity aim; the union\'s stability 0 and separatism 1 are written in memory; one real daily advance lets politics::tick run dissolve_ussr. The player then continues through the served continue_campaign command.'
  }),
  harness_staged_parent_collapse: Object.freeze({
    fixture: true, activation: true,
    source: 'tools/ui/successor-fixtures/run.cjs (recipe H1)',
    staging: 'A disposable campaign created and saved by the exact binary (/api/new seed 1990 with the parent as player). Only the parent nation\'s stability and separatism raw values are replaced by 0.0 and 1.0 in that save (byte-exact splice, every other byte unchanged). The ordinary Load path and one real daily advance let politics::tick run the parent\'s dissolution. The player then continues through the served continue_campaign command.'
  })
});

function label(kind) {
  const spec = KINDS[kind];
  if (!spec) throw new Error('Unknown fixture kind: ' + kind);
  const out = { kind, fixture: spec.fixture, organic: false, activation: spec.activation };
  if (spec.fixture) { out.disclaimer = DISCLAIMER; out.source = spec.source; out.staging = spec.staging; } else out.note = spec.note;
  return out;
}

function sha256(value) { return crypto.createHash('sha256').update(value).digest('hex'); }
// Key-sorted JSON, so equal readings hash equally whatever their key order.
function canonical(value) {
  if (Array.isArray(value)) return '[' + value.map(canonical).join(',') + ']';
  if (value && typeof value === 'object') return '{' + Object.keys(value).sort().map(k => JSON.stringify(k) + ':' + canonical(value[k])).join(',') + '}';
  return JSON.stringify(value);
}
function digest(value) { return sha256(Buffer.from(canonical(value), 'utf8')); }
function fileSha256(file) {
  const digest = crypto.createHash('sha256'), buffer = Buffer.allocUnsafe(1 << 20), fd = fs.openSync(file, 'r');
  try { for (;;) { const n = fs.readSync(fd, buffer, 0, buffer.length, null); if (!n) break; digest.update(buffer.subarray(0, n)); } }
  finally { fs.closeSync(fd); }
  return digest.digest('hex');
}

// ---------------------------------------------------------------------------
// Rust source reading. These read the simulation's own tables rather than a
// hand-copied list, so a changed roster or dissolution family shows up as
// inventory drift instead of a silently stale expectation.
// ---------------------------------------------------------------------------
const IDENT = /[A-Za-z0-9_]/;
function stripRustComments(src) {
  let out = '', i = 0;
  const n = src.length;
  while (i < n) {
    const c = src[i], d = src[i + 1];
    if (c === '/' && d === '/') { while (i < n && src[i] !== '\n') i++; continue; }
    if (c === '/' && d === '*') {
      let depth = 1; i += 2;
      while (i < n && depth) {
        if (src[i] === '/' && src[i + 1] === '*') { depth++; i += 2; } else if (src[i] === '*' && src[i + 1] === '/') { depth--; i += 2; } else i++;
      }
      out += ' '; continue;
    }
    const prev = src[i - 1] || '', prev2 = src[i - 2] || '';
    if (c === 'r' && (d === '"' || d === '#') && (!IDENT.test(prev) || (prev === 'b' && !IDENT.test(prev2)))) {
      let j = i + 1, hashes = 0;
      while (src[j] === '#') { hashes++; j++; }
      if (src[j] === '"') {
        const close = '"' + '#'.repeat(hashes), end = src.indexOf(close, j + 1);
        if (end < 0) throw new Error('Unterminated raw string literal');
        out += src.slice(i, end + close.length); i = end + close.length; continue;
      }
    }
    if (c === '"') {
      let j = i + 1;
      while (j < n && src[j] !== '"') { if (src[j] === '\\') j++; j++; }
      if (j >= n) throw new Error('Unterminated string literal');
      out += src.slice(i, j + 1); i = j + 1; continue;
    }
    if (c === "'") {
      if (d === '\\') { const end = src.indexOf("'", i + 3); if (end > 0 && end - i <= 12) { out += src.slice(i, end + 1); i = end + 1; continue; } }
      else if (src[i + 2] === "'") { out += src.slice(i, i + 3); i += 3; continue; }
    }
    out += c; i++;
  }
  return out;
}

// Index just past the bracket that closes the one at `open`, in comment-free source.
function scanBalanced(src, open) {
  const pairs = { '(': ')', '[': ']', '{': '}' }, stack = [];
  if (!pairs[src[open]]) throw new Error('Not an opening bracket at ' + open);
  for (let i = open; i < src.length; i++) {
    const c = src[i];
    if (c === '"') { i++; while (i < src.length && src[i] !== '"') { if (src[i] === '\\') i++; i++; } continue; }
    if (c === "'") {
      if (src[i + 1] === '\\') { const end = src.indexOf("'", i + 3); if (end > 0 && end - i <= 12) { i = end; continue; } }
      else if (src[i + 2] === "'") { i += 2; continue; }
      continue;
    }
    if (pairs[c]) stack.push(pairs[c]);
    else if (c === ')' || c === ']' || c === '}') {
      if (stack.pop() !== c) throw new Error('Unbalanced ' + c + ' at ' + i);
      if (!stack.length) return i + 1;
    }
  }
  throw new Error('No closing bracket for ' + open);
}

function splitTopLevel(text) {
  const parts = [];
  let depth = 0, start = 0;
  for (let i = 0; i < text.length; i++) {
    const c = text[i];
    if (c === '"') { i++; while (i < text.length && text[i] !== '"') { if (text[i] === '\\') i++; i++; } continue; }
    if ('([{'.includes(c)) depth++;
    else if (')]}'.includes(c)) depth--;
    else if (c === ',' && depth === 0) { parts.push(text.slice(start, i).trim()); start = i + 1; }
  }
  const last = text.slice(start).trim();
  if (last) parts.push(last);
  return parts;
}

function rustString(token) {
  const m = /^"((?:[^"\\]|\\.)*)"$/s.exec(token.trim());
  if (!m) return null;
  return m[1].replace(/\\(["\\])/g, '$1');
}
function rustStringList(token) {
  const t = token.trim();
  const m = /^&\s*\[([\s\S]*)\]$/.exec(t);
  if (!m) throw new Error('Expected a slice literal: ' + t.slice(0, 60));
  return splitTopLevel(m[1]).map(item => { const s = rustString(item); if (s === null) throw new Error('Expected a string: ' + item); return s; });
}
function nationIdList(token) {
  const m = /^&\s*\[([\s\S]*)\]$/.exec(token.trim());
  if (!m) throw new Error('Expected a NationId slice: ' + token.slice(0, 60));
  return splitTopLevel(m[1]).map(item => { const id = /^NationId::(\w+)$/.exec(item.trim()); if (!id) throw new Error('Expected NationId::X, found ' + item); return id[1]; });
}

// Every `row("Code", "Name", &[aliases], "Region", &[neighbours], &[claims], start_1990, patron, major)`.
function rosterRows(nationsRs) {
  const src = stripRustComments(nationsRs), rows = [], re = /\brow\(/g;
  let m;
  while ((m = re.exec(src))) {
    const open = m.index + 3, close = scanBalanced(src, open), args = splitTopLevel(src.slice(open + 1, close - 1));
    const code = args.length === 9 ? rustString(args[0]) : null;
    if (code === null) continue; // the const fn definition itself
    const bool = t => { if (t !== 'true' && t !== 'false') throw new Error(code + ': expected a boolean, found ' + t); return t === 'true'; };
    rows.push({ code, name: rustString(args[1]), aliases: rustStringList(args[2]), region: rustString(args[3]),
      neighbours: rustStringList(args[4]), start_1990: bool(args[6]), patron: bool(args[7]), major: bool(args[8]) });
    re.lastIndex = close;
  }
  return rows;
}

function successorParents(districtsRs) {
  const src = stripRustComments(districtsRs), at = src.search(/\bconst\s+SUCCESSOR_PARENTS\b/);
  if (at < 0) throw new Error('SUCCESSOR_PARENTS not found');
  const open = src.indexOf('[', src.indexOf('=', at)), body = src.slice(open + 1, scanBalanced(src, open) - 1);
  return splitTopLevel(body).map(pair => {
    const p = /^\(\s*NationId::(\w+)\s*,\s*NationId::(\w+)\s*\)$/.exec(pair);
    if (!p) throw new Error('Unreadable SUCCESSOR_PARENTS entry: ' + pair);
    return { heir: p[1], parent: p[2] };
  });
}

function functionBody(src, name) {
  const at = src.search(new RegExp('\\bfn\\s+' + name + '\\s*\\('));
  if (at < 0) return null;
  const open = src.indexOf('{', scanBalanced(src, src.indexOf('(', at)));
  return src.slice(open, scanBalanced(src, open));
}

// The dissolution functions in politics.rs: each one's dissolve_to heir list,
// plus the trigger the regime-collapse block evaluates before calling it.
function dissolutionFamilies(politicsRs) {
  const src = stripRustComments(politicsRs), families = {};
  const re = /\bfn\s+(dissolve_\w+)\s*\(/g;
  let m;
  while ((m = re.exec(src))) {
    const fn = m[1], body = functionBody(src, fn);
    const call = body && body.search(/\bdissolve_to\s*\(/);
    if (!body || call < 0) continue;
    const open = body.indexOf('(', call), args = splitTopLevel(body.slice(open + 1, scanBalanced(body, open) - 1));
    const parent = /^NationId::(\w+)$/.exec(args[1] || '');
    const flag = /set_flag\(\s*"(\w+)"\s*\)/.exec(body);
    if (!parent) throw new Error(fn + ': unreadable dissolve_to parent');
    families[parent[1]] = { function: 'politics::' + fn, flag: flag ? flag[1] : null, heirs: nationIdList(args[2]), trigger: null };
  }
  // `if is_ussr && (stab < 25.0 || sep > 0.9) && !w.has_flag("ussr_dissolved") { dissolve_ussr(w); }`
  const trig = /\(\s*stab\s*<\s*([\d.]+)\s*\|\|\s*sep\s*>\s*([\d.]+)\s*\)\s*&&\s*!\s*w\.has_flag\(\s*"(\w+)"\s*\)\s*\{\s*(dissolve_\w+)\s*\(/g;
  while ((m = trig.exec(src))) {
    const family = Object.values(families).find(f => f.function === 'politics::' + m[4]);
    if (family) family.trigger = { stability_below: Number(m[1]), separatism_above: Number(m[2]), unless_flag: m[3], site: 'politics::tick regime-collapse block' };
  }
  return families;
}

// campaign_journey.rs `successors`: the authored continuation family a dead
// player may choose from after the parent dissolves.
function journeyFamilies(journeyRs) {
  const src = stripRustComments(journeyRs), body = functionBody(src, 'successors');
  if (!body) throw new Error('campaign_journey::successors not found');
  const out = {}, re = /Some\(\s*NationId::(\w+)\s*\)\s*if\s+w\.has_flag\(\s*"(\w+)"\s*\)\s*=>\s*&\s*\[/g;
  let m;
  while ((m = re.exec(body))) {
    const open = body.indexOf('[', m.index + m[0].length - 1);
    out[m[1]] = { flag: m[2], family: nationIdList('&' + body.slice(open, scanBalanced(body, open))) };
  }
  return out;
}

// ---------------------------------------------------------------------------
// Byte-exact save staging. Saves carry u64 values (the SplitMix64 state once
// the world has ticked) that a JavaScript number cannot hold, so a save is
// never re-serialised here: raw value tokens are located and spliced.
// ---------------------------------------------------------------------------
function skipWs(t, i) { while (i < t.length) { const c = t.charCodeAt(i); if (c === 32 || c === 9 || c === 10 || c === 13) i++; else break; } return i; }
function scanString(t, i) {
  if (t[i] !== '"') throw new Error('Expected a JSON string at ' + i);
  for (i++; i < t.length; i++) { const c = t.charCodeAt(i); if (c === 92) { i++; continue; } if (c === 34) return i + 1; }
  throw new Error('Unterminated JSON string');
}
function scanValue(t, i) {
  i = skipWs(t, i);
  const c = t[i];
  if (c === '"') return scanString(t, i);
  if (c === '{' || c === '[') {
    let depth = 0;
    for (; i < t.length; i++) {
      const ch = t[i];
      if (ch === '"') { i = scanString(t, i) - 1; continue; }
      if (ch === '{' || ch === '[') depth++;
      else if (ch === '}' || ch === ']') { depth--; if (!depth) return i + 1; }
    }
    throw new Error('Unterminated JSON container');
  }
  let j = i;
  while (j < t.length && !',}] \t\r\n'.includes(t[j])) j++;
  if (j === i) throw new Error('Expected a JSON value at ' + i);
  return j;
}
function objectEntries(t, i) {
  i = skipWs(t, i);
  if (t[i] !== '{') throw new Error('Expected a JSON object at ' + i);
  const out = [];
  i = skipWs(t, i + 1);
  if (t[i] === '}') return out;
  for (;;) {
    const keyEnd = scanString(t, i), key = JSON.parse(t.slice(i, keyEnd));
    i = skipWs(t, keyEnd);
    if (t[i] !== ':') throw new Error('Expected : at ' + i);
    const start = skipWs(t, i + 1), end = scanValue(t, start);
    if (out.some(e => e.key === key)) throw new Error('Duplicate JSON key: ' + key);
    out.push({ key, start, end });
    i = skipWs(t, end);
    if (t[i] === ',') { i = skipWs(t, i + 1); continue; }
    if (t[i] === '}') return out;
    throw new Error('Expected , or } at ' + i);
  }
}
function arrayItems(t, i) {
  i = skipWs(t, i);
  if (t[i] !== '[') throw new Error('Expected a JSON array at ' + i);
  const out = [];
  i = skipWs(t, i + 1);
  if (t[i] === ']') return out;
  for (;;) {
    const start = i, end = scanValue(t, start);
    out.push({ start, end });
    i = skipWs(t, end);
    if (t[i] === ',') { i = skipWs(t, i + 1); continue; }
    if (t[i] === ']') return out;
    throw new Error('Expected , or ] at ' + i);
  }
}
const entry = (entries, key) => entries.find(e => e.key === key);

// Locate `world` (a campaign envelope, an integrated wrapper or a raw world)
// down to the object that owns `nations`, then the requested nation's fields.
function locateNation(text, nationId) {
  let pointer = '', entries = objectEntries(text, 0);
  for (let guard = 0; guard < 4 && !entry(entries, 'nations'); guard++) {
    const world = entry(entries, 'world');
    if (!world) throw new Error('No world object with nations in this save');
    pointer += '/world'; entries = objectEntries(text, world.start);
  }
  const nations = entry(entries, 'nations');
  if (!nations) throw new Error('No nations array in this save');
  const items = arrayItems(text, nations.start);
  const matches = [];
  items.forEach((item, index) => {
    const fields = objectEntries(text, item.start), id = entry(fields, 'id');
    if (id && JSON.parse(text.slice(id.start, id.end)) === nationId) matches.push({ index, fields });
  });
  if (matches.length !== 1) throw new Error('Expected exactly one ' + nationId + ' nation record, found ' + matches.length);
  return { pointer: pointer + '/nations/' + matches[0].index, fields: matches[0].fields };
}

// The simulation world's top-level entries as raw text, for lossless diffs.
function worldEntries(text) {
  let entries = objectEntries(text, 0);
  for (let guard = 0; guard < 4 && !entry(entries, 'nations'); guard++) {
    const world = entry(entries, 'world');
    if (!world) throw new Error('No world object with nations in this save');
    entries = objectEntries(text, world.start);
  }
  return entries.map(e => ({ key: e.key, raw: text.slice(e.start, e.end) }));
}

// Returns the staged text and an exact patch record (UTF-8 byte offsets).
function stageNationFields(text, nationId, values) {
  if (text.charCodeAt(0) === 0xfeff) throw new Error('Refusing a save with a byte-order mark; the server never writes one');
  const found = locateNation(text, nationId), alive = entry(found.fields, 'alive');
  if (!alive || text.slice(alive.start, alive.end) !== 'true') throw new Error(nationId + ' is not alive in this save');
  const edits = Object.entries(values).map(([field, raw]) => {
    if (!/^-?\d+(\.\d+)?$/.test(raw)) throw new Error('Staged values must be plain JSON numbers: ' + raw);
    const e = entry(found.fields, field);
    if (!e) throw new Error(nationId + ' has no ' + field + ' field');
    const from = text.slice(e.start, e.end);
    if (!/^-?\d+(\.\d+)?([eE][-+]?\d+)?$/.test(from)) throw new Error(nationId + '.' + field + ' is not a plain number: ' + from);
    return { field, start: e.start, end: e.end, from_raw: from, to_raw: raw };
  }).sort((a, b) => a.start - b.start);
  let out = '', cursor = 0;
  for (const e of edits) { out += text.slice(cursor, e.start) + e.to_raw; cursor = e.end; }
  out += text.slice(cursor);
  const patch = edits.map(e => ({ op: 'replace', path: found.pointer + '/' + e.field, from_raw: e.from_raw, to_raw: e.to_raw,
    byte_offset: Buffer.byteLength(text.slice(0, e.start), 'utf8') }));
  return { text: out, patch };
}

// Independent proof that two byte buffers differ ONLY at the patch spans.
function verifyOnlyPatched(before, after, patch) {
  let a = 0, b = 0;
  for (const p of [...patch].sort((x, y) => x.byte_offset - y.byte_offset)) {
    const length = p.byte_offset - a;
    if (length < 0 || !before.subarray(a, p.byte_offset).equals(after.subarray(b, b + length))) return false;
    const from = Buffer.from(p.from_raw, 'utf8'), to = Buffer.from(p.to_raw, 'utf8');
    if (!before.subarray(p.byte_offset, p.byte_offset + from.length).equals(from)) return false;
    if (!after.subarray(b + length, b + length + to.length).equals(to)) return false;
    a = p.byte_offset + from.length; b = b + length + to.length;
  }
  return before.subarray(a).equals(after.subarray(b));
}

// A campaign archive without its wall-clock `saved_unix` tail: equal digests
// mean equal world, history, log, journey and metadata bytes.
function archiveProjection(text) {
  const m = /,"saved_unix":\d+\}\s*$/.exec(text);
  if (!m || !text.startsWith('{"format":"spheres-campaign"')) throw new Error('Not a spheres-campaign archive with a trailing saved_unix');
  return text.slice(0, m.index) + '}';
}

// ---------------------------------------------------------------------------
// Validation of an inventory and of a harness result file.
// ---------------------------------------------------------------------------
function validateInventory(inv) {
  const errors = [];
  if (!inv || inv.format !== FORMAT_INVENTORY) return ['Inventory format must be ' + FORMAT_INVENTORY];
  const rows = Array.isArray(inv.rows) ? inv.rows : [];
  const expected = inv.expectation && inv.expectation.successor_identities;
  if (!Number.isInteger(expected) || expected <= 0) errors.push('Inventory has no integer successor expectation');
  if (rows.length !== expected) errors.push('Inventory has ' + rows.length + ' rows; the expectation is ' + expected);
  const seen = new Map();
  for (const row of rows) {
    if (!row || typeof row.id !== 'string' || !row.id) { errors.push('Inventory row without an id'); continue; }
    if (seen.has(row.id)) errors.push('Duplicate successor identity: ' + row.id);
    seen.set(row.id, row);
    if (row.roster_start_1990 !== false || row.c01_start_1990 !== false) errors.push(row.id + ' is not a successor in both the roster and the C01 census');
    if (row.native_activation_path) {
      if (!row.parent || !row.dissolution || !row.player_continuation) errors.push(row.id + ' claims a native path without parent, dissolution and continuation');
    } else if (row.player_continuation || row.dissolution) errors.push(row.id + ' has a partial activation mapping');
    const recipes = Array.isArray(row.recipes) ? row.recipes : [];
    if (!recipes.includes('G0')) errors.push(row.id + ' lacks the selection-guard recipe');
    if (row.native_activation_path !== (recipes.includes('H1') && recipes.includes('B1'))) errors.push(row.id + ' recipes disagree with its activation path');
  }
  for (const id of (inv.expected_ids || [])) if (!seen.has(id)) errors.push('Missing successor identity: ' + id);
  for (const id of seen.keys()) if (inv.expected_ids && !inv.expected_ids.includes(id)) errors.push('Unexpected identity in inventory: ' + id);
  const noHook = rows.filter(r => !r.native_activation_path).map(r => r.id);
  if (noHook.length) {
    const flagged = (inv.discrepancies || []).filter(d => d.kind === 'no_native_activation').flatMap(d => d.identities || []);
    for (const id of noHook) if (!flagged.includes(id)) errors.push(id + ' has no native activation path but no discrepancy flags it');
  }
  return errors;
}

// The inventory content that results depend on: everything except provenance.
function inventorySemantic(inv) { const copy = { ...inv }; delete copy.generated; delete copy.context; return copy; }
function inventoryDigest(inv) { return digest(inventorySemantic(inv)); }

function summarize(cases) {
  const summary = {};
  for (const layer of LAYERS) summary[layer] = Object.fromEntries(STATUSES.map(s => [s, 0]));
  for (const c of cases) if (summary[c.layer] && STATUSES.includes(c.status)) summary[c.layer][c.status]++;
  return summary;
}

const REQUIRED_ASSETS = Object.freeze(['index.html', 'campaign-transport.js', 'campaign-ui.js', 'fiscal-recovery-ui.js', 'cash-flow-ui.js', 'companies-ui.js', 'companies.css', 'government-ui.js', 'government-ui.css', 'campaign-operations-ui.js', 'campaign-operations-ui.css', 'map-controls.js', 'map-controls.css', 'guidance-ui.js', 'guidance-ui.css', 'globe3d.js', 'city-layer.js', 'city-mesh.js', 'water-detail.js', 'equipment-mesh.js', 'equipment-ui.js', 'equipment-ui.css']);
function requiredChecks(layer) {
  if (layer === 'selection_guard') return ['start_refused', 'refusal_left_campaign_untouched', 'absent_from_starter_roster', 'dossier_identity'];
  if (layer === 'activation_save_load') return ['fixture:dissolved_parent', 'fixture:succession_offer', 'fixture:turns_paused_until_continuation', 'continuation_command', 'lost_response_replay_is_idempotent', 'ordinary_reload', 'readings_equal_after_reload', 'archive_roundtrip_exact', 'seven_days_as_successor', ...['after_activation', 'after_reload'].flatMap(stage => ['selection_identity', 'map_ownership', 'government', 'budget', 'guidance', 'capabilities_retained'].map(part => stage + ':' + part))];
  if (layer === 'browser_ui') return ['ui_fixture_loaded', 'ui_succession_offer', 'ui_continuation', 'ui_named_save', 'ui_reload', 'ui_archive_roundtrip_exact', 'ui_single_continuation_and_no_advance', 'ui_no_page_errors', ...['after_activation', 'after_reload'].flatMap(stage => ['ui_selection_identity', 'ui_government', 'ui_budget', 'ui_guidance'].map(part => stage + ':' + part))];
  return [];
}
function safeRelative(file) {
  return typeof file === 'string' && file.length > 0 && !path.posix.isAbsolute(file) && !path.win32.isAbsolute(file)
    && !file.includes('\\') && !file.includes(':') && file.split('/').every(p => p && p !== '.' && p !== '..');
}
// The callback must read the exact retained gzip bytes from the result directory.
// Historical hash-only receipts are inspectable only by explicit opt-in; they
// never satisfy current fixture-retention acceptance.
function retainedArchive(record, readBytes) {
  if (!record || !safeRelative(record.file) || record.compression !== 'gzip' || !HEX64.test(record.sha256 || '') || !HEX64.test(record.gzip_sha256 || '')
    || !Number.isSafeInteger(record.bytes) || record.bytes <= 0 || !Number.isSafeInteger(record.gzip_bytes) || record.gzip_bytes <= 0) throw new Error('invalid retained archive descriptor');
  if (typeof readBytes !== 'function') throw new Error('retained archive byte verification is required');
  const packed = readBytes(record.file);
  if (!Buffer.isBuffer(packed) || packed.length !== record.gzip_bytes || sha256(packed) !== record.gzip_sha256) throw new Error('retained gzip bytes/hash differ');
  const raw = zlib.gunzipSync(packed, { maxOutputLength: record.bytes });
  if (raw.length !== record.bytes || sha256(raw) !== record.sha256) throw new Error('retained archive bytes/hash differ');
  if (!Buffer.from(raw.toString('utf8'), 'utf8').equals(raw)) throw new Error('retained archive is not UTF-8');
  return raw;
}
function verifyServedText(committed, served) {
  if (!Buffer.isBuffer(committed) || !Buffer.isBuffer(served)) throw new Error('Asset verification requires bytes');
  const text = served.toString('utf8');
  if (!Buffer.from(text, 'utf8').equals(served)) throw new Error('Served asset is not exact UTF-8');
  const normalized = Buffer.from(text.replace(/\r\n/g, '\n'), 'utf8');
  if (!normalized.equals(committed)) throw new Error('Served asset differs from exact committed source after CRLF-to-LF normalization');
  return { served_sha256: sha256(served), served_bytes: served.length, committed_sha256: sha256(committed), committed_bytes: committed.length,
    canonical_served_sha256: sha256(normalized), crlf_pairs: (text.match(/\r\n/g) || []).length,
    comparison: 'Exact UTF-8 bytes, with CRLF-to-LF only on served text. No claim that this review checkout equals the original build checkout raw bytes.' };
}
function resultExitCode(result, errors) {
  return errors.length || (result.cases || []).some(c => c.status === 'failed' || c.status === 'blocked')
    || (result.fixture_inputs || []).some(f => f.status === 'failed') ? 1 : 0;
}

const HEX40 = /^[0-9a-f]{40}$/, HEX64 = /^[0-9a-f]{64}$/;
function validateResults(result, inventory, { requireFinal = false, exists = () => true, hashOf = null, readBytes = null, allowHistoricalHashOnly = false } = {}) {
  const errors = [...validateInventory(inventory)];
  if (!result || result.format !== FORMAT_RESULT) return ['Result format must be ' + FORMAT_RESULT];
  if (result.disclaimer !== DISCLAIMER) errors.push('Result-level fixture disclaimer is missing or altered');
  if (requireFinal && result.development_run !== false) errors.push('Committed evidence must come from a non-development run');
  if (!Array.isArray(result.cases) || !Array.isArray(result.fixture_inputs)) return [...errors, 'Results need cases and fixture_inputs arrays'];
  if (result.cases.some(c => !c || typeof c !== 'object')) return [...errors, 'Malformed case'];
  const b = result.build || {};
  const executed = (result.cases || []).filter(c => c.status !== 'unrun');
  if (executed.length) {
    if (!HEX40.test(b.expected_revision || '')) errors.push('Executed results need the exact 40-character build revision');
    if (!HEX64.test(b.binary_sha256 || '')) errors.push('Executed results need the binary SHA-256');
    if (b.served_revision !== String(b.expected_revision || '').slice(0, 12)) errors.push('Served revision does not match the expected build revision');
    if (b.save_directory_isolated !== true) errors.push('Executed results must prove the isolated save directory');
    if (b.runtime_source_equal_to_expected !== true) errors.push('Runtime source identity must be verified');
    if (requireFinal && result.harness?.tree_clean_for_harness_and_runtime !== true) errors.push('Final results require a clean harness/runtime tree');
  }
  if (!result.inventory || result.inventory.digest !== inventoryDigest(inventory)) errors.push('Results were produced against a different inventory; regenerate the inventory and re-run the harness');
  if (result.cases.some(c => c.layer === 'browser_ui' && c.status === 'passed')) {
    const verified = b.served_assets_verified;
    if (!verified || verified.error || verified.revision !== b.expected_revision || verified.binary_sha256 !== b.binary_sha256
      || REQUIRED_ASSETS.some(name => !HEX64.test(verified.assets?.[name] || ''))) errors.push('Passed browser cases require complete served asset identity verification');
  }
  const rows = new Map((inventory.rows || []).map(r => [r.id, r]));
  const inputs = new Map();
  for (const f of result.fixture_inputs || []) {
    const where = 'Fixture input ' + (f && f.id);
    if (!f || typeof f.id !== 'string' || inputs.has(f.id)) { errors.push(where + ': missing or duplicate id'); continue; }
    inputs.set(f.id, f);
    if (!KINDS[f.kind] || !KINDS[f.kind].fixture) errors.push(where + ': unknown activation kind');
    if (!HEX64.test(f.archive_sha256 || '') || !HEX64.test(f.projection_sha256 || '')) errors.push(where + ': archive and projection SHA-256 are required');
    if (f.kind === 'native_s21_authored_dissolution') {
      if (f.parent !== 'USSR' || f.exporter?.revision !== b.expected_revision || !HEX64.test(f.exporter?.test_binary_sha256 || '')
        || !f.exporter?.runs?.some(run => run.succession_sha256 === f.archive_sha256 && run.projection_sha256 === f.projection_sha256 && run.bytes === f.bytes)) errors.push(where + ': native exporter provenance does not match the consumed input/build');
    }
    if (f.kind === 'harness_staged_parent_collapse') {
      const fields = (f.patch || []).map(p => String(p.path).split('/').pop()).sort().join(',');
      const values = Object.fromEntries((f.patch || []).map(p => [String(p.path).split('/').pop(), p.to_raw]));
      if (fields !== 'separatism,stability' || values.stability !== '0.0' || values.separatism !== '1.0') errors.push(where + ': staging must replace exactly stability=0.0 and separatism=1.0');
      if (f.status !== 'passed' || !Array.isArray(f.checks) || !f.checks.length || f.checks.some(c => c.status !== 'passed')) errors.push(where + ': staged fixture checks did not pass');
      if (f.verified_only_patched !== true) errors.push(where + ': staged bytes were not proven identical outside the patch');
      if (!HEX64.test(f.base_archive_sha256 || '')) errors.push(where + ': the unstaged base archive hash is required');
    }
    if (!allowHistoricalHashOnly || f.retained) {
      try {
        const raw = retainedArchive(f.retained?.archive, readBytes);
        if (sha256(raw) !== f.archive_sha256 || raw.length !== f.bytes || sha256(Buffer.from(archiveProjection(raw.toString('utf8')), 'utf8')) !== f.projection_sha256) throw new Error('consumed archive does not match retained input');
        if (f.kind === 'harness_staged_parent_collapse') {
          const base = retainedArchive(f.retained?.base, readBytes), staged = retainedArchive(f.retained?.staged, readBytes);
          if (sha256(base) !== f.base_archive_sha256 || sha256(staged) !== f.staged_archive_sha256 || !verifyOnlyPatched(base, staged, f.patch)) throw new Error('retained staging proof differs');
          const recreated = stageNationFields(base.toString('utf8'), f.parent, { stability: '0.0', separatism: '1.0' });
          if (!Buffer.from(recreated.text, 'utf8').equals(staged) || canonical(recreated.patch) !== canonical(f.patch)) throw new Error('staging did not target the declared parent');
        }
      } catch (error) { errors.push(where + ': retained input verification failed: ' + error.message); }
    }
  }
  const seen = new Set();
  for (const c of result.cases || []) {
    const key = c.identity + '/' + c.layer, where = 'Case ' + key;
    if (!rows.has(c.identity)) { errors.push(where + ': identity is not in the inventory'); continue; }
    if (!LAYERS.includes(c.layer)) { errors.push(where + ': unknown layer'); continue; }
    if (seen.has(key)) errors.push(where + ': duplicate result');
    seen.add(key);
    if (!STATUSES.includes(c.status)) { errors.push(where + ': status must be one of ' + STATUSES.join('/')); continue; }
    const row = rows.get(c.identity), checks = Array.isArray(c.checks) ? c.checks : [];
    if (checks.some(k => !k || k.status !== 'passed' && k.status !== 'failed')) errors.push(where + ': every executed check must be passed or failed');
    if (c.status === 'unrun') {
      if (!UNRUN_REASONS.includes(c.reason)) errors.push(where + ': unrun needs a reason of ' + UNRUN_REASONS.join('/'));
      if (checks.length) errors.push(where + ': an unrun case cannot carry executed checks');
      if (c.reason === 'no_native_hook' && (row.native_activation_path || c.layer === 'selection_guard')) errors.push(where + ': no_native_hook contradicts the available native path');
      if (c.reason === 'no_native_hook' && c.proposal !== PROPOSAL) errors.push(where + ': a missing native hook must cite the integration proposal');
    } else {
      if (!checks.length && c.status !== 'blocked') errors.push(where + ': an executed case needs checks');
      const names = checks.map(k => k && k.name);
      if (new Set(names).size !== names.length) errors.push(where + ': duplicate check name');
      if (c.status === 'passed') for (const name of requiredChecks(c.layer)) if (!names.includes(name)) errors.push(where + ': missing required check ' + name);
      if (c.status === 'passed' && checks.some(k => k.status !== 'passed')) errors.push(where + ': passed with a failed check');
      if (c.status === 'failed' && !checks.some(k => k.status === 'failed')) errors.push(where + ': failed without a failed check');
      if (c.status === 'blocked' && (!c.blocked_stage || !c.error)) errors.push(where + ': blocked needs the blocking stage and error');
    }
    // Labels. Anything claiming organic history, or an activation without the
    // exact disclaimer, is mislabelled regardless of its status.
    const l = c.label;
    if (c.status !== 'unrun') {
      if (!l || typeof l !== 'object') errors.push(where + ': executed case without a label');
      else if (l.organic !== false) errors.push(where + ': mislabelled — a fixture can never claim organic succession');
      else if (c.layer === 'selection_guard') {
        if (l.kind !== 'selection_guard_no_activation' || l.activation !== false || l.fixture !== false) errors.push(where + ': selection guard must be labelled as a no-activation guard');
      } else {
        if (!['native_s21_authored_dissolution', 'harness_staged_parent_collapse'].includes(l.kind) || l.fixture !== true || l.activation !== true) errors.push(where + ': activation must be labelled as an authored or staged fixture');
        if (l.disclaimer !== DISCLAIMER) errors.push(where + ': activation fixture without the exact disclaimer');
        if (l.kind === 'native_s21_authored_dissolution' && row.parent !== 'USSR') errors.push(where + ': the S21 exporter only authors the USSR dissolution');
        const f = inputs.get(c.fixture_input);
        if (!f) errors.push(where + ': executed activation without a retained fixture input');
        else if (f.kind !== l.kind || f.parent !== row.parent) errors.push(where + ': label kind or parent differs from its fixture input');
      }
    }
    if (c.layer !== 'selection_guard' && !row.native_activation_path) {
      if (c.status !== 'unrun' || c.reason !== 'no_native_hook') errors.push(where + ': no native activation path exists, so this must be unrun/no_native_hook');
    }
    if (c.status === 'passed' && c.layer === 'browser_ui' && !(c.screenshots || []).length) errors.push(where + ': passed browser case needs retained screenshots');
    for (const shot of c.screenshots || []) {
      if (!shot || !safeRelative(shot.file) || !HEX64.test(shot.sha256 || '')) { errors.push(where + ': screenshot without file and SHA-256'); continue; }
      if (!exists(shot.file)) errors.push(where + ': missing screenshot ' + shot.file);
      else if (hashOf && hashOf(shot.file) !== shot.sha256) errors.push(where + ': screenshot hash differs for ' + shot.file);
    }
  }
  for (const id of rows.keys()) for (const layer of LAYERS) if (!seen.has(id + '/' + layer)) errors.push('Missing result: ' + id + '/' + layer);
  const recomputed = summarize(result.cases || []);
  if (JSON.stringify(recomputed) !== JSON.stringify(result.summary)) errors.push('Summary counts do not match the cases: ' + JSON.stringify(recomputed));
  return errors;
}

module.exports = {
  verifyServedText, REQUIRED_ASSETS, requiredChecks, safeRelative, retainedArchive, resultExitCode, FORMAT_INVENTORY, FORMAT_RESULT, STATUSES, LAYERS, UNRUN_REASONS, DISCLAIMER, PROPOSAL, KINDS,
  label, sha256, canonical, digest, fileSha256, stripRustComments, scanBalanced, splitTopLevel, rosterRows, successorParents,
  dissolutionFamilies, journeyFamilies, stageNationFields, verifyOnlyPatched, archiveProjection, locateNation,
  objectEntries, worldEntries, validateInventory, validateResults, summarize, inventorySemantic, inventoryDigest
};
