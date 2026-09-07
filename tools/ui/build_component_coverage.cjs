#!/usr/bin/env node
// Which component choices actually change the model, and which quietly do not.
//
// The equipment designer lets a player pick a component and shows them a
// vehicle. Nothing in this repository said which of those picks produce
// DISTINCT geometry, which merely recolour or nudge a sibling, and which change
// nothing at all — so a player who selected a component, saw no change, and
// wondered whether that was a bug or the design had no way to find out, and
// neither did the next session. This measures it.
//
//   node tools/ui/build_component_coverage.cjs           -> docs/art/COMPONENT_COVERAGE.md
//   node tools/ui/build_component_coverage.cjs --check   -> exit 1 if the committed file is stale
//
// Everything in the document is read from the two Rust catalogues and from the
// mesh generator itself, so a report that disagrees with the art is a bug in
// this script and not a stale document. There are no timestamps and no machine
// names in the output: two runs of the same tree write the same bytes.
"use strict";

const fs = require("fs");
const path = require("path");
const crypto = require("crypto");

const ROOT = path.resolve(__dirname, "..", "..");
const SPECS = path.join(ROOT, "spheres-sim", "src", "equipment_specs.rs");
const GROUND = path.join(ROOT, "spheres-sim", "src", "equipment_ground.rs");
const OUT = path.join(ROOT, "docs", "art", "COMPONENT_COVERAGE.md");
const { build } = require(path.join(ROOT, "spheres-web", "ui", "equipment-mesh.js"));

const specsSrc = fs.readFileSync(SPECS, "utf8");
const groundSrc = fs.readFileSync(GROUND, "utf8");

// The baseline this pass started from, recorded so the improvement is not taken
// on trust. It is a measurement, not a target: re-derive it by checking out the
// tree before the P1 ammunition pass and running this tool. Nothing checks it,
// which is exactly why it says where it came from.
const BASELINE = {
  note: "measured by this tool on the tree before the P1 ammunition and silhouette pass",
  pairs: { distinct: 127, weak: 31, absent: 11 },
  ids: { distinct: 27, weak: 10, absent: 6 },
  holes: "every absent pair was in the `ammunition` slot: the tank hulls drew no ammunition geometry at all, "
    + "and the specialist hulls drew one locker whose only response to the choice was its label",
};

// ---------------------------------------------------------------------------
// The catalogues, scraped rather than retyped
// ---------------------------------------------------------------------------
function components(src, origin) {
  return [...src.matchAll(/component!\("([^"]+)","([^"]+)","([^"]+)"/g)]
    .map(([, id, name, slot]) => ({ id, name, slot, origin }));
}
const CATALOGUE = [...components(specsSrc, "equipment_specs.rs"), ...components(groundSrc, "equipment_ground.rs")];
const BY_ID = new Map(CATALOGUE.map(c => [c.id, c]));
if (CATALOGUE.length < 60) throw new Error(`the catalogue scrape found only ${CATALOGUE.length} components`);

// ---------------------------------------------------------------------------
// The default specification of every platform, from the Rust that builds it
// ---------------------------------------------------------------------------
// tank_spec() lays down twelve slots and then overwrites a handful per platform;
// default_spec() adds the ground families on top, drops `tracks` from the two
// wheeled hulls and inserts their mission slot. Both are parsed here rather than
// copied, so a change to either goes through this report.
const pairs = text => [...text.matchAll(/\("([a-z_]+)","([a-z0-9_]+)"\)/g)].map(m => [m[1], m[2]]);
function armTable(src, marker) {
  const start = src.indexOf(marker);
  if (start < 0) throw new Error(`cannot find ${marker}`);
  const body = src.slice(start, src.indexOf("};", start));
  const arms = new Map();
  for (const m of body.matchAll(/"([a-z_]+)" => &\[([^\]]*)\]/g)) arms.set(m[1], pairs(m[2]));
  const fallback = /_ => &\[([^\]]*)\]/.exec(body);
  arms.set("_", fallback ? pairs(fallback[1]) : []);
  return arms;
}
const TANK_BASE = pairs(specsSrc.slice(specsSrc.indexOf("pub fn tank_spec"), specsSrc.indexOf("let changes", specsSrc.indexOf("pub fn tank_spec"))));
const TANK_ARMS = armTable(specsSrc.slice(specsSrc.indexOf("pub fn tank_spec")), "let changes:&[(&str,&str)] = match platform {");
const GROUND_SEED = pairs(/for \(slot,id\) in \[(.*?)\] \{ spec\.components\.insert/.exec(groundSrc)[1]);
const GROUND_ARMS = armTable(groundSrc.slice(groundSrc.indexOf("pub fn default_spec")), "let changes: &[(&str,&str)] = match platform {");
const TANK_PLATFORMS = ["tank_standard", "tank_heavy", "tank_light", "tank_destroyer"];
const GROUND_PLATFORMS = ["ground_ifv", "ground_apc", "ground_recon", "ground_artillery", "ground_air_defense"];
const PLATFORMS = [...TANK_PLATFORMS, ...GROUND_PLATFORMS];
const isGround = platform => GROUND_PLATFORMS.includes(platform);

function defaultSpec(platform) {
  const spec = new Map(TANK_BASE.map(([k, v]) => [k, v]));
  for (const [k, v] of TANK_ARMS.get(platform) || TANK_ARMS.get("_")) spec.set(k, v);
  if (!isGround(platform)) return spec;
  for (const [k, v] of GROUND_SEED) spec.set(k, v);
  if (platform === "ground_apc" || platform === "ground_recon") spec.delete("tracks");
  for (const [k, v] of GROUND_ARMS.get(platform) || GROUND_ARMS.get("_")) spec.set(k, v);
  return spec;
}
const DEFAULTS = new Map(PLATFORMS.map(p => [p, defaultSpec(p)]));
for (const [platform, spec] of DEFAULTS) {
  const want = isGround(platform) ? 13 : 12;
  if (spec.size !== want) throw new Error(`${platform} resolved ${spec.size} slots, expected ${want}`);
}

// ---------------------------------------------------------------------------
// Which platform legally fields which component, from component_compatible()
// ---------------------------------------------------------------------------
// This is the per-choice filter. The Rust comment above it is explicit that
// paired weapon/mount/payload validation is a SEPARATE gate
// (`configuration_refusals`), which this report deliberately does not apply:
// changing one slot at a time is the only way to attribute a change in the mesh
// to the component that caused it.
const compatibleSrc = groundSrc.slice(groundSrc.indexOf("pub fn component_compatible"));
const groundMatch = compatibleSrc.slice(compatibleSrc.indexOf("match c.slot {"), compatibleSrc.indexOf("_ => false,"));
const idsIn = text => [...text.matchAll(/"([a-z0-9_]+)"/g)].map(m => m[1]).filter(id => BY_ID.has(id));

const LEGAL = new Map(PLATFORMS.map(p => [p, new Map()]));   // platform -> slot -> Set(id)
const allow = (platform, slot, id) => {
  const bySlot = LEGAL.get(platform);
  if (!bySlot.has(slot)) bySlot.set(slot, new Set());
  bySlot.get(slot).add(id);
};
{
  // Each `"<slot>" => ...` arm either lists ids for every ground platform or
  // opens its own `match platform { ... }`. Arms are split on the slot keys
  // themselves rather than on commas, because a `matches!` list is full of
  // commas and the last arm shares its line with the catch-all. One arm also
  // carries a single `(platform == "x" && c.id == "y")` rider, which is pulled
  // out first so the general collector cannot read it as a blanket allowance.
  const slotKeys = [...groundMatch.matchAll(/\n {8}"([a-z_]+)" => /g)];
  if (slotKeys.length < 17) throw new Error(`component_compatible parsed ${slotKeys.length} slot arms`);
  for (let k = 0; k < slotKeys.length; k++) {
    const slot = slotKeys[k][1];
    const from = slotKeys[k].index + slotKeys[k][0].length;
    const to = k + 1 < slotKeys.length ? slotKeys[k + 1].index : groundMatch.length;
    let body = groundMatch.slice(from, to);
    for (const rider of body.matchAll(/\(platform == "([a-z_]+)" && c\.id == "([a-z0-9_]+)"\)/g)) allow(rider[1], slot, rider[2]);
    body = body.replace(/\(platform == "[a-z_]+" && c\.id == "[a-z0-9_]+"\)/g, "");
    const nested = body.indexOf("match platform {");
    if (nested < 0) { for (const id of idsIn(body)) for (const p of GROUND_PLATFORMS) allow(p, slot, id); continue; }
    // Inside a nested match the arms are keyed by platform, and the catch-all
    // stands for every ground platform the arm did not name.
    const inner = body.slice(nested + "match platform {".length);
    const platformKeys = [...inner.matchAll(/"(ground_[a-z_]+)" =>/g)];
    const covered = new Set(platformKeys.map(m => m[1]));
    for (let j = 0; j < platformKeys.length; j++) {
      const from2 = platformKeys[j].index + platformKeys[j][0].length;
      const next = j + 1 < platformKeys.length ? platformKeys[j + 1].index : inner.length;
      const stop = inner.indexOf("_ =>", from2);
      const to2 = Math.min(next, stop < 0 ? inner.length : stop);
      for (const id of idsIn(inner.slice(from2, to2))) allow(platformKeys[j][1], slot, id);
    }
    const rest = inner.lastIndexOf("_ =>");
    if (rest >= 0) for (const id of idsIn(inner.slice(rest))) for (const p of GROUND_PLATFORMS) if (!covered.has(p)) allow(p, slot, id);
  }
}
{
  // The tank branch is an exclusion list, not an inclusion one: every design
  // component is legal unless the platform refuses it. Only tank_light's list is
  // data; the two casemate rules are asserted verbatim so a change to them
  // cannot slip past as silence.
  const lightList = /"tank_light" => !matches!\(c\.id,([^)]*)\)/.exec(compatibleSrc);
  if (!lightList) throw new Error("cannot read tank_light's exclusion list");
  const lightBans = new Set(idsIn(lightList[1]));
  for (const line of ['"tank_destroyer" => c.slot != "turret" || c.id == "turret_casemate"', '_ => c.id != "turret_casemate"'])
    if (!compatibleSrc.includes(line)) throw new Error(`the tank branch no longer reads ${line}`);
  const designSlots = new Set(TANK_BASE.map(([slot]) => slot));
  const groundIds = new Set(components(groundSrc, "").map(c => c.id));
  for (const c of CATALOGUE) {
    if (!designSlots.has(c.slot) || groundIds.has(c.id)) continue;
    for (const platform of TANK_PLATFORMS) {
      if (platform === "tank_light" && lightBans.has(c.id)) continue;
      if (platform === "tank_destroyer" ? (c.slot === "turret" && c.id !== "turret_casemate") : c.id === "turret_casemate") continue;
      allow(platform, c.slot, c.id);
    }
  }
}
// Guard the derivation: every platform's own default must be legal on it, and
// no slot the platform carries may end up with nothing to choose from.
for (const [platform, spec] of DEFAULTS) {
  for (const [slot, id] of spec) {
    if (!BY_ID.has(id)) continue;                       // the legacy catalogue is out of scope
    const set = LEGAL.get(platform).get(slot);
    if (!set || !set.has(id)) throw new Error(`${platform} default ${slot}=${id} is not legal under the derived table`);
  }
}

// ---------------------------------------------------------------------------
// Measuring one swap
// ---------------------------------------------------------------------------
const hash = view => crypto.createHash("sha256").update(Buffer.from(view.buffer, view.byteOffset, view.byteLength)).digest("hex");
const specOf = (platform, slot, id) => {
  const out = {};
  for (const [k, v] of DEFAULTS.get(platform)) out[k] = v;
  if (slot) out[slot] = id;
  return { platform, components: out };
};

// SILHOUETTE. The verdict "reads the same" has to be measured, not asserted, so
// this rasterises the mesh into three orthographic occupancy grids — side, front
// and plan — in a FIXED world frame per platform, and compares the two grids
// cell by cell. Each triangle contributes its three corners and its centroid,
// which at 128 cells across an 8 m vehicle is one sample per 63 mm of a mesh
// whose smallest primitive is a 20 mm bolt head: dense enough that a solid volume
// fills, coarse enough that a 5 mm nudge does not register as a new shape.
//
// The score is the AREA of the outline that got redrawn, in square metres,
// summed over the three views — not a fraction of the vehicle. A fraction makes
// the same fitting score differently on a big hull and a small one, which it did:
// the automatic gearbox housing scored 0.28% on the carrier and 0.44% on the
// fighting vehicle for identical geometry, and would have been filed under two
// different verdicts. Square metres of changed outline is the same number on both.
const GRID = 128;
const FRAMES = new Map();
function frameOf(platform) {
  if (!FRAMES.has(platform)) {
    const b = build(specOf(platform)).bounds;
    const low = [b.min[0] - 0.6, -0.2, b.min[2] - 0.6], high = [b.max[0] + 0.6, b.max[1] + 0.9, b.max[2] + 0.6];
    const span = [0, 1, 2].map(i => Math.max(1e-6, high[i] - low[i]));
    const cell = span.map(s => s / GRID);
    // side view spans Z x Y, front view X x Y, plan view X x Z
    FRAMES.set(platform, { low, span, area: [cell[2] * cell[1], cell[0] * cell[1], cell[0] * cell[2]] });
  }
  return FRAMES.get(platform);
}
function silhouette(mesh, frame) {
  const cells = new Uint8Array(GRID * GRID * 3);
  const { low, span } = frame;
  const mark = (x, y, z) => {
    const ix = Math.min(GRID - 1, Math.max(0, Math.floor((x - low[0]) / span[0] * GRID)));
    const iy = Math.min(GRID - 1, Math.max(0, Math.floor((y - low[1]) / span[1] * GRID)));
    const iz = Math.min(GRID - 1, Math.max(0, Math.floor((z - low[2]) / span[2] * GRID)));
    cells[iz * GRID + iy] = 1;                                      // side view, along X
    cells[GRID * GRID + ix * GRID + iy] = 1;                        // front view, along Z
    cells[2 * GRID * GRID + ix * GRID + iz] = 1;                    // plan view, along Y
  };
  const p = mesh.positions;
  for (let i = 0; i < p.length; i += 9) {
    mark(p[i], p[i + 1], p[i + 2]); mark(p[i + 3], p[i + 4], p[i + 5]); mark(p[i + 6], p[i + 7], p[i + 8]);
    mark((p[i] + p[i + 3] + p[i + 6]) / 3, (p[i + 1] + p[i + 4] + p[i + 7]) / 3, (p[i + 2] + p[i + 5] + p[i + 8]) / 3);
  }
  return cells;
}
function silhouetteArea(a, b, frame) {
  let area = 0;
  for (let view = 0; view < 3; view++) {
    let differ = 0;
    const from = view * GRID * GRID, to = from + GRID * GRID;
    for (let i = from; i < to; i++) if (a[i] !== b[i]) differ++;
    area += differ * frame.area[view];
  }
  return area;
}

// A change is DISTINCT when it redraws at least this much outline, WEAK below it.
// There is NO natural gap in the measured spread — the values run continuously
// from nothing up to several square metres — so this is a chosen bar and not a
// discovered one, and the cases nearest it are named in the document so a reader
// can disagree with the placement rather than take it on faith. It sits at
// 0.06 m^2: about a 250 mm square of new outline, which is roughly the smallest
// fitting on these hulls that a player can see on a catalogue card. Below it lie
// the recolours, the pad thicknesses and the panels that grow in place.
const SILHOUETTE_BAR = 0.06;
// A choice is also DISTINCT when it adds or removes a named part, or moves the
// vehicle's own extent, regardless of how little outline that costs.
const BOUNDS_BAR = 0.05;

// Parts are keyed by SLOT AND ORDINAL, never by name. A name here carries the
// chosen component's label, so keying by name reports a pure relabelling as a
// part appearing and another disappearing — which is exactly how six ammunition
// ids read as "distinct" while their geometry was byte-for-byte identical.
function partIndex(mesh) {
  const out = new Map(), seen = new Map();
  for (const part of mesh.parts) {
    const ordinal = (seen.get(part.slot) || 0);
    seen.set(part.slot, ordinal + 1);
    out.set(`${part.slot}#${ordinal}`, { name: part.name, count: part.count, digest: hash(mesh.positions.subarray(part.first * 3, (part.first + part.count) * 3)) });
  }
  return out;
}
function measure(platform, slot, id) {
  const frame = frameOf(platform);
  const base = build(specOf(platform)), next = build(specOf(platform, slot, id));
  const baseLod1 = build({ ...specOf(platform), lod: 1 }), nextLod1 = build({ ...specOf(platform, slot, id), lod: 1 });
  const movedGeometry = hash(base.positions) !== hash(next.positions);
  const movedColour = hash(base.colors) !== hash(next.colors);
  const before = partIndex(base), after = partIndex(next);
  const added = [...after.keys()].filter(key => !before.has(key));
  const removed = [...before.keys()].filter(key => !after.has(key));
  const changed = [...after.keys()].filter(key => before.has(key) && before.get(key).digest !== after.get(key).digest);
  const resized = changed.filter(key => before.get(key).count !== after.get(key).count);
  const relabelled = [...after.keys()].filter(key => before.has(key) && before.get(key).name !== after.get(key).name);
  let reach = 0;
  for (let axis = 0; axis < 3; axis++) reach = Math.max(reach, Math.abs(next.bounds.max[axis] - base.bounds.max[axis]), Math.abs(next.bounds.min[axis] - base.bounds.min[axis]));
  const area = movedGeometry ? silhouetteArea(silhouette(base, frame), silhouette(next, frame), frame) : 0;
  const name = key => (after.get(key) || before.get(key)).name;
  const carrier = [...added, ...changed, ...removed, ...relabelled].map(name)[0] || "—";
  let verdict = "absent";
  if (movedGeometry || movedColour) verdict = "weak";
  if (added.length || removed.length || reach > BOUNDS_BAR || area >= SILHOUETTE_BAR) verdict = "distinct";
  return {
    platform, slot, id, verdict, carrier,
    triangles: next.triangleCount - base.triangleCount,
    area, reach, added: added.length, removed: removed.length, changed: changed.length, resized: resized.length,
    relabelled: relabelled.length, colourOnly: !movedGeometry && movedColour,
    lod1: hash(baseLod1.positions) !== hash(nextLod1.positions) || hash(baseLod1.colors) !== hash(nextLod1.colors),
  };
}

// ---------------------------------------------------------------------------
// The sweep
// ---------------------------------------------------------------------------
const rows = [], baselines = [];
for (const platform of PLATFORMS) {
  const fitted = DEFAULTS.get(platform);
  for (const [slot, ids] of [...LEGAL.get(platform)].sort((a, b) => a[0].localeCompare(b[0]))) {
    if (!fitted.has(slot)) continue;
    for (const id of [...ids].sort()) {
      if (fitted.get(slot) === id) { baselines.push({ platform, slot, id }); continue; }
      rows.push(measure(platform, slot, id));
    }
  }
}
rows.sort((a, b) => a.slot.localeCompare(b.slot) || a.id.localeCompare(b.id) || PLATFORMS.indexOf(a.platform) - PLATFORMS.indexOf(b.platform));

const RANK = { distinct: 0, weak: 1, absent: 2 };
const worstById = new Map();
for (const row of rows) {
  const held = worstById.get(row.id);
  if (!held || RANK[row.verdict] > RANK[held.verdict]) worstById.set(row.id, row);
}
// A component that is only ever the default of every hull that fields it is never
// measured against anything, and saying "distinct" about it would be a guess.
for (const { id } of baselines) if (!worstById.has(id)) worstById.set(id, { id, verdict: "baseline only" });

const tally = source => {
  const out = { distinct: 0, weak: 0, absent: 0 };
  for (const row of source) if (row.verdict in out) out[row.verdict]++;
  return out;
};
const pairCounts = tally(rows), idCounts = tally(worstById.values());
const baselineOnly = [...worstById.values()].filter(r => r.verdict === "baseline only");

// ---------------------------------------------------------------------------
// The document
// ---------------------------------------------------------------------------
const area = value => `${value.toFixed(3)}`;
const signed = value => (value > 0 ? `+${value}` : String(value));
const note = row => {
  const bits = [];
  if (row.added) bits.push(`+${row.added} part${row.added > 1 ? "s" : ""}`);
  if (row.removed) bits.push(`-${row.removed} part${row.removed > 1 ? "s" : ""}`);
  if (row.resized) bits.push(`${row.resized} resized`);
  else if (row.changed) bits.push(`${row.changed} reshaped`);
  if (row.reach > BOUNDS_BAR) bits.push(`extent ${row.reach.toFixed(2)} m`);
  if (row.colourOnly) bits.push("colour only");
  if (!row.changed && !row.added && !row.removed && row.relabelled) bits.push("relabelled only");
  return bits.join(", ") || "no change";
};
const table = source => [
  "| slot | component | platform | verdict | Δ tris | outline m² | LOD1 | carried by | change |",
  "| --- | --- | --- | --- | --- | --- | --- | --- | --- |",
  ...source.map(r => `| \`${r.slot}\` | \`${r.id}\` | \`${r.platform}\` | ${r.verdict} | ${signed(r.triangles)} | ${area(r.area)} | ${r.lod1 ? "yes" : "no"} | ${r.carrier} | ${note(r)} |`),
].join("\n");
// The cases nearest the bar, so the placement can be argued with.
const nearBar = rows.filter(r => r.area > 0 && !r.added && !r.removed && r.reach <= BOUNDS_BAR)
  .sort((a, b) => Math.abs(a.area - SILHOUETTE_BAR) - Math.abs(b.area - SILHOUETTE_BAR)).slice(0, 6)
  .sort((a, b) => a.area - b.area);

const holes = rows.filter(r => r.verdict !== "distinct");
const md = `# Component visual coverage

Generated by \`node tools/ui/build_component_coverage.cjs\` — do not edit by hand.
Re-run it after any change to \`spheres-web/ui/equipment-mesh.js\` or to either Rust
catalogue; \`--check\` fails when this file and the geometry disagree, and
\`tools/ui/check_equipment_mesh.cjs\` runs that check.

## What is measured

Every component id in \`spheres-sim/src/equipment_specs.rs\` and
\`spheres-sim/src/equipment_ground.rs\`, on every platform that legally fields it
under \`component_compatible\`, selected one slot at a time against that platform's
own \`default_spec\`. Paired weapon/mount/payload validity is a separate gate in the
simulation and is deliberately not applied here: changing more than one slot would
make it impossible to say which component caused the change in the mesh.

For each pair the tool builds both meshes and compares the position and colour
buffers, the per-part vertex ranges keyed by slot and ordinal, the model bounds,
and a ${GRID}x${GRID} orthographic occupancy grid in side, front and plan view, held in
a fixed world frame so the two grids are comparable. The outline column is the
area of that occupancy that got redrawn, in square metres summed over the three
views. Parts are matched by slot and ordinal rather than by name because a part
name carries the chosen component's own label: keyed by name, a pure relabelling
reads as one part appearing and another disappearing, which is how the ammunition
slot looked covered while its geometry never moved.

| verdict | meaning |
| --- | --- |
| distinct | the choice adds or removes a named part, moves the vehicle's extent by more than ${BOUNDS_BAR * 100} cm, or redraws at least ${SILHOUETTE_BAR} m² of outline |
| weak | the buffers differ but the shape reads the same — a recolour, a thickness, a pitch |
| absent | positions and colours are byte-identical: selecting it changes nothing |

${SILHOUETTE_BAR} m² is a chosen bar, not a discovered one: the measured spread runs
continuously from nothing upward with no natural gap to sit in. It is about a
250 mm square of new outline. The ${nearBar.length} pairs nearest it, so the placement can be
argued with rather than taken on trust${nearBar.every(r => r.verdict === "distinct") ? " — nothing currently falls below it" : ""}:

${table(nearBar)}

## Coverage

| scope | distinct | weak | absent | total |
| --- | --- | --- | --- | --- |
| platform/component pairs | ${pairCounts.distinct} | ${pairCounts.weak} | ${pairCounts.absent} | ${rows.length} |
| components, at their weakest platform | ${idCounts.distinct} | ${idCounts.weak} | ${idCounts.absent} | ${idCounts.distinct + idCounts.weak + idCounts.absent} |

${baselineOnly.length} further component${baselineOnly.length === 1 ? " is" : "s are"} the default of every hull that fields
${baselineOnly.length === 1 ? "it" : "them"} and so ${baselineOnly.length === 1 ? "is" : "are"} never measured against a sibling: ${baselineOnly.map(r => `\`${r.id}\``).join(", ")}.
Calling those distinct would be a guess, so they are counted separately — but they
are not unexercised: each one is the geometry every measured pair on its own hull
was compared against, so a row above that reads distinct says as much about the
default it displaced as about the choice that displaced it.

Recorded baseline, ${BASELINE.note}: ${BASELINE.pairs.distinct} distinct, ${BASELINE.pairs.weak} weak and
${BASELINE.pairs.absent} absent pairs; ${BASELINE.ids.distinct} distinct, ${BASELINE.ids.weak} weak and ${BASELINE.ids.absent} absent components.
There, ${BASELINE.holes}.

## Everything that is not distinct

${holes.length ? table(holes) : "Nothing. Every component the simulation offers changes the silhouette of every platform that fields it."}

## Every measured pair

${table(rows)}
`;

if (process.argv.includes("--check")) {
  const current = fs.existsSync(OUT) ? fs.readFileSync(OUT, "utf8").replace(/\r\n/g, "\n") : null;
  if (current !== md) {
    console.error(`stale, re-run without --check:\n  ${OUT}`);
    process.exit(1);
  }
  console.log(`coverage current: ${rows.length} pairs, ${pairCounts.distinct} distinct / ${pairCounts.weak} weak / ${pairCounts.absent} absent`);
} else {
  fs.mkdirSync(path.dirname(OUT), { recursive: true });
  fs.writeFileSync(OUT, md, "utf8");
  console.log(`wrote ${rows.length} pairs -> docs/art/COMPONENT_COVERAGE.md (${pairCounts.distinct} distinct / ${pairCounts.weak} weak / ${pairCounts.absent} absent)`);
}
