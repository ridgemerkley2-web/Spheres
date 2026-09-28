'use strict';
// S24 successor identity inventory, generated from current data and simulation
// source. It records the expectation (23) and flags every count or mapping
// discrepancy instead of altering the expectation.
//
//   node tools/ui/successor-fixtures/inventory.cjs                 summary
//   node tools/ui/successor-fixtures/inventory.cjs --write <file>  regenerate
//   node tools/ui/successor-fixtures/inventory.cjs --check <file>  exit 1 on drift
const fs = require('node:fs');
const path = require('node:path');
const cp = require('node:child_process');
const lib = require('./lib.cjs');

const ROOT = path.resolve(__dirname, '../../..');
const INPUTS = {
  pathway: 'docs/planning/campaign-pathway.json',
  countries: 'docs/campaign-certification/C01/countries.json',
  census: 'docs/campaign-certification/C01/census.json',
  roster: 'spheres-sim/src/nations.rs',
  parents: 'spheres-sim/src/districts.rs',
  politics: 'spheres-sim/src/politics.rs',
  journey: 'spheres-web/src/campaign_journey.rs',
  districts: 'spheres-sim/data/districts.json',
  web: 'spheres-web/src/main.rs'
};
const REFUSAL = 'is not on the board in January 1990';

// Reviewed observations that are not mechanically derivable from one table.
// They are stable text, so a regenerated inventory reproduces them exactly.
const OBSERVATIONS = [
  { id: 'O1', kind: 'documentation', detected_by: 'manual_review', identities: ['Namibia', 'EastTimor'],
    summary: 'spheres-web/src/main.rs sources_json says the twenty-three successors are "transcribed and sourced where the sim seats them"; the simulation seats only the 21 Soviet and Yugoslav republics. Namibia and East Timor are never seated (nations.rs records East Timor\'s "mechanism gap"; Namibia\'s row is a successor by date only).',
    evidence: ['spheres-web/src/main.rs fn sources_json doc comment', 'spheres-sim/src/nations.rs EastTimor row comment "A GAP THE INTEGRATOR SHOULD SEE"', 'spheres-sim/src/nations.rs Namibia row comment "NAMIBIA IS A SUCCESSOR, NOT A STARTER"'] },
  { id: 'O2', kind: 'conflation_hazard', detected_by: 'manual_review', identities: [],
    summary: 'census.json counts.existing_lifecycle_records is also 23, but it counts party lifecycle disclosures in roles-and-lifecycle.json (registry bounds and future editorial disclosures), not successor identities. Do not use it as the successor count.',
    evidence: ['docs/campaign-certification/C01/census.json counts.existing_lifecycle_records', 'docs/campaign-certification/C01/roles-and-lifecycle.json lifecycle_disclosures'] },
  { id: 'O3', kind: 'historical_parent', detected_by: 'manual_review', identities: ['Namibia', 'EastTimor'],
    summary: 'Namibia (independent 21 March 1990 from South African administration) and East Timor (independent 20 May 2002 after Indonesian occupation) are successors by start date, not federation breakups. Activating them needs a design ruling and a sourced opening state, not only a fixture hook; inventing a trigger would script history.',
    evidence: ['spheres-sim/src/nations.rs Namibia and EastTimor row comments', 'spheres-sim/src/data/embedded.rs notes on the missing Namibia and East Timor data files'] }
];

function read(rel) { return fs.readFileSync(path.join(ROOT, rel), 'utf8'); }
function sorted(values) { return [...values].sort(); }
function minus(a, b) { const s = new Set(b); return sorted(a.filter(x => !s.has(x))); }

function buildInventory() {
  const pathway = JSON.parse(read(INPUTS.pathway)), countries = JSON.parse(read(INPUTS.countries)), census = JSON.parse(read(INPUTS.census));
  const roster = lib.rosterRows(read(INPUTS.roster)), parents = lib.successorParents(read(INPUTS.parents));
  const families = lib.dissolutionFamilies(read(INPUTS.politics)), journey = lib.journeyFamilies(read(INPUTS.journey));
  const districts = JSON.parse(read(INPUTS.districts)).nations, web = read(INPUTS.web);
  const s24 = pathway.sessions.find(s => s.id === 'S24');
  const accept = s24 && (s24.accept || []).find(text => /successor identities/.test(text));
  const pathwayCount = accept && Number((/All (\d+) successor identities/.exec(accept) || [])[1]);

  const rosterSucc = roster.filter(r => !r.start_1990), rosterById = new Map(roster.map(r => [r.code, r]));
  const c01Succ = countries.filter(r => r.start_1990 === false), c01ById = new Map(countries.map(r => [r.id, r]));
  const starters = new Set(roster.filter(r => r.start_1990).map(r => r.code));
  const parentOf = new Map(parents.map(p => [p.heir, p.parent]));
  const listedUnder = id => {
    const own = new Set((districts[id] || []).map(d => d.id));
    return sorted(Object.keys(districts).filter(k => k !== id && starters.has(k) && districts[k].some(d => own.has(d.id))));
  };
  const ids = sorted(new Set([...rosterSucc.map(r => r.code), ...c01Succ.map(r => r.id)]));

  const rows = ids.map(id => {
    const r = rosterById.get(id), c = c01ById.get(id), parent = parentOf.get(id) || null;
    const family = parent && families[parent], heirIndex = family ? family.heirs.indexOf(id) : -1;
    const continuation = !!(parent && journey[parent] && journey[parent].family.includes(id));
    const native = !!(parent && heirIndex >= 0 && continuation);
    return {
      id, name: r ? r.name : null, c01_name: c ? c.name : null, region: r ? r.region : null,
      certified_case: !!(c && c.certified_case), certified_identity: (census.certified_identity_ids || []).includes(id),
      roster_start_1990: r ? r.start_1990 : null, c01_start_1990: c ? c.start_1990 : null,
      parent, dissolution: heirIndex >= 0 ? family.function : null, heir_position: heirIndex >= 0 ? heirIndex + 1 : null,
      continuation_state: heirIndex === 0, player_continuation: continuation,
      map_districts: (districts[id] || []).length, districts_listed_under_1990_starters: listedUnder(id),
      native_activation_path: native,
      recipes: ['G0', ...(native ? ['H1'] : []), ...(native && parent === 'USSR' ? ['N1'] : []), ...(native ? ['B1'] : [])]
    };
  });

  const discrepancies = [];
  // Successor-relevant counts are semantic; whole-roster totals are context,
  // so adding an ordinary 1990 starter elsewhere does not read as drift here.
  const counts = {
    pathway_s24_expectation: pathwayCount || null, census_successor_nations: census.counts.successor_nations,
    c01_start_1990_false: c01Succ.length, roster_start_1990_false: rosterSucc.length,
    successor_parent_pairs: parents.length, native_activation_paths: rows.filter(r => r.native_activation_path).length,
    player_continuation_targets: rows.filter(r => r.player_continuation).length, no_native_hook: rows.filter(r => !r.native_activation_path).length
  };
  const context = {
    census_nation_identities: census.counts.nation_identities, census_starting_nations: census.counts.starting_nations,
    c01_rows: countries.length, c01_start_1990_true: countries.filter(r => r.start_1990 === true).length,
    roster_rows: roster.length, roster_start_1990_true: roster.length - rosterSucc.length, pathway_s24_statement: accept || null
  };
  const expectation = counts.pathway_s24_expectation || counts.census_successor_nations;
  if (!counts.pathway_s24_expectation)
    discrepancies.push({ id: 'D-pathway', kind: 'count', detected_by: 'automatic', identities: [], summary: 'campaign-pathway.json S24 no longer states "All N successor identities"; the census count is used as the expectation until reviewed.', expectation_unchanged: true });
  const successorCounts = { pathway_s24_expectation: counts.pathway_s24_expectation, census_successor_nations: counts.census_successor_nations,
    c01_start_1990_false: counts.c01_start_1990_false, roster_start_1990_false: counts.roster_start_1990_false, inventory_rows: rows.length };
  if (new Set(Object.values(successorCounts).filter(v => v !== null)).size !== 1)
    discrepancies.push({ id: 'D-count', kind: 'count', detected_by: 'automatic', identities: [], summary: 'Successor counts disagree across sources.', values: successorCounts, expectation_unchanged: true });
  if (context.census_nation_identities !== context.roster_rows || context.census_starting_nations !== context.roster_start_1990_true || context.c01_rows !== context.roster_rows)
    discrepancies.push({ id: 'D-roster', kind: 'count', detected_by: 'automatic', identities: [], summary: 'Roster, C01 and census nation totals disagree.', values: { census_nation_identities: context.census_nation_identities, census_starting_nations: context.census_starting_nations, c01_rows: context.c01_rows, roster_rows: context.roster_rows, roster_start_1990_true: context.roster_start_1990_true }, expectation_unchanged: true });
  const onlyRoster = minus(rosterSucc.map(r => r.code), c01Succ.map(r => r.id)), onlyC01 = minus(c01Succ.map(r => r.id), rosterSucc.map(r => r.code));
  if (onlyRoster.length || onlyC01.length)
    discrepancies.push({ id: 'D-identity', kind: 'identity_mapping', detected_by: 'automatic', identities: sorted([...onlyRoster, ...onlyC01]), summary: 'Roster successors and C01 successor rows are different sets.', only_roster: onlyRoster, only_c01: onlyC01, expectation_unchanged: true });
  const noHook = rows.filter(r => !r.native_activation_path).map(r => r.id);
  if (noHook.length)
    discrepancies.push({ id: 'D-activation', kind: 'no_native_activation', detected_by: 'automatic', identities: noHook,
      summary: expectation + ' successor identities are expected, but only ' + counts.native_activation_paths + ' have a native activation path (a politics.rs dissolution that seats them, a districts::SUCCESSOR_PARENTS entry and a campaign_journey continuation family). The rest cannot be activated by any existing entry point; their activation cases are recorded unrun with an integration proposal. The expectation is not reduced.',
      missing: Object.fromEntries(noHook.map(id => { const r = rows.find(x => x.id === id); return [id, { successor_parent: r.parent, dissolution: r.dissolution, player_continuation: r.player_continuation }]; })),
      expectation_unchanged: true, proposal: lib.PROPOSAL });
  const unownedAtStart = rows.filter(r => r.map_districts > 0 && !r.districts_listed_under_1990_starters.length).map(r => r.id);
  if (unownedAtStart.length)
    discrepancies.push({ id: 'D-map', kind: 'map_ownership', detected_by: 'automatic', identities: unownedAtStart,
      summary: 'These successors have district lists in spheres-sim/data/districts.json, but no 1990 starter lists those districts, so the territory has no start owner on the 1 January 1990 board and no parent to dissolve from.', expectation_unchanged: true });
  const noMap = rows.filter(r => !r.map_districts).map(r => r.id);
  if (noMap.length) discrepancies.push({ id: 'D-nomap', kind: 'map_ownership', detected_by: 'automatic', identities: noMap, summary: 'Successors without any district list.', expectation_unchanged: true });
  for (const parent of sorted(new Set([...Object.keys(families), ...Object.keys(journey), ...parents.map(p => p.parent)]))) {
    const table = parents.filter(p => p.parent === parent).map(p => p.heir), heirs = (families[parent] || {}).heirs || [], chosen = (journey[parent] || {}).family || [];
    const diffs = { successor_parents_not_dissolved: minus(table, heirs), dissolved_not_in_successor_parents: minus(heirs, table), dissolved_not_continuable: minus(heirs, chosen), continuable_not_dissolved: minus(chosen, heirs) };
    const bad = sorted(new Set(Object.values(diffs).flat()));
    if (bad.length) discrepancies.push({ id: 'D-family-' + parent, kind: 'family_mismatch', detected_by: 'automatic', identities: bad, summary: parent + ' family tables disagree.', ...diffs, expectation_unchanged: true });
    const startersInFamily = heirs.filter(h => starters.has(h));
    if (startersInFamily.length) discrepancies.push({ id: 'D-starter-' + parent, kind: 'identity_mapping', detected_by: 'automatic', identities: startersInFamily, summary: parent + ' dissolution seats a nation that is already a 1990 starter.', expectation_unchanged: true });
  }
  const guard = web.includes(REFUSAL);
  if (!guard) discrepancies.push({ id: 'D-guard', kind: 'selection_guard', detected_by: 'automatic', identities: [], summary: 'spheres-web new_game no longer carries the successor start refusal text "' + REFUSAL + '".', expectation_unchanged: true });

  return {
    format: lib.FORMAT_INVENTORY,
    scope: 'S24 preparation only. Identity inventory for the successor-country fixture harness; it certifies nothing.',
    expectation: { successor_identities: expectation, sources: ['docs/planning/campaign-pathway.json sessions[S24].accept ("All N successor identities ...")', 'docs/campaign-certification/C01/census.json counts.successor_nations'] },
    expected_ids: sorted(c01Succ.map(r => r.id)),
    counts,
    context,
    families: Object.fromEntries(Object.entries(families).map(([parent, f]) => [parent, { parent_name: rosterById.has(parent) ? rosterById.get(parent).name : null, dissolution: f.function, flag: f.flag, trigger: f.trigger, heirs: f.heirs, continuation_family: (journey[parent] || {}).family || [] }])),
    selection_guard: { source: 'spheres-web/src/main.rs fn new_game', refusal_text_present: guard, refusal_fragment: REFUSAL },
    rows,
    discrepancies,
    observations: OBSERVATIONS
  };
}

function inputProvenance() {
  const lf = rel => Buffer.from(read(rel).replace(/\r\n/g, '\n'), 'utf8');
  return Object.values(INPUTS).map(rel => { const bytes = lf(rel); return { path: rel, sha256_lf: lib.sha256(bytes), bytes_lf: bytes.length }; });
}
function headRevision() {
  const r = cp.spawnSync('git', ['rev-parse', 'HEAD'], { cwd: ROOT, encoding: 'utf8', windowsHide: true });
  return r.status === 0 ? r.stdout.trim() : null;
}
// Everything except provenance and whole-roster context is semantic.
const semantic = lib.inventorySemantic;

function main(argv) {
  const [flag, file] = argv;
  const inv = buildInventory();
  if (flag === '--write') {
    if (!file) throw new Error('--write needs a file');
    const errors = lib.validateInventory(inv);
    if (errors.length) throw new Error('Refusing to write an invalid inventory:\n' + errors.join('\n'));
    const out = { ...inv, generated: { by: 'tools/ui/successor-fixtures/inventory.cjs', from_revision: headRevision(), inputs: inputProvenance(), note: 'Input hashes use LF-normalised text; they are provenance only and are excluded from --check.' } };
    fs.writeFileSync(path.resolve(file), JSON.stringify(out, null, 2) + '\n');
    console.log('Wrote ' + file + ': ' + inv.rows.length + ' rows, ' + inv.discrepancies.length + ' discrepancies');
    return 0;
  }
  if (flag === '--check') {
    if (!file) throw new Error('--check needs a file');
    const committed = JSON.parse(fs.readFileSync(path.resolve(file), 'utf8'));
    const errors = lib.validateInventory(committed);
    const same = JSON.stringify(semantic(committed)) === JSON.stringify(semantic(inv));
    if (!same) errors.push('Inventory differs from current sources; regenerate with --write and review the discrepancy list');
    const stale = (committed.generated && committed.generated.inputs || []).filter(i => { const now = inputProvenance().find(x => x.path === i.path); return !now || now.sha256_lf !== i.sha256_lf; }).map(i => i.path);
    if (stale.length) console.log('Note (not a failure): inputs changed since generation without semantic drift: ' + stale.join(', '));
    if (errors.length) { console.error(errors.join('\n')); return 1; }
    console.log('PASS: ' + committed.rows.length + ' successor identities, ' + committed.discrepancies.length + ' discrepancies, inventory current');
    return 0;
  }
  console.log(JSON.stringify({ counts: inv.counts, discrepancies: inv.discrepancies.map(d => d.id + ' ' + d.identities.join(',')) }, null, 2));
  return 0;
}

module.exports = { buildInventory, semantic, inputProvenance, INPUTS, ROOT };
if (require.main === module) {
  try { process.exitCode = main(process.argv.slice(2)); } catch (error) { console.error(error.stack || String(error)); process.exitCode = 1; }
}
