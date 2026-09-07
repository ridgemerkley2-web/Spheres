#!/usr/bin/env node
// The P0 art budget harness — measured, not promised.
//
// Roadmap section 4 prints a budget table and says outright that those are
// "initial budgets to validate on the user's machine, not measured performance
// promises". The P0 exit gate then asks for "measured performance recorded".
// This tool measures the ground-vehicle, site and town set, counts the
// triangles the generator actually emits, computes the bytes those meshes would
// occupy on the GPU, weighs the source that produces them, and states PASS,
// UNDER or OVER against the roadmap's own numbers.
//
//   node tools/ui/bench_art.cjs            -> docs/art/P0_BUDGETS.md, timings on stdout
//   node tools/ui/bench_art.cjs --check    -> exit 1 if the committed file is stale
//                                             OR if any asset is over its budget
//   node tools/ui/bench_art.cjs --cold X   -> internal: one cold build in a fresh process
//
// Two rules shape the output.
//
// DETERMINISM (roadmap 7.6). The committed document carries no timings, no
// dates and no machine identity, so two runs anywhere produce identical bytes
// and --check tests the art rather than the clock. Timing is real work and is
// printed to stdout by the same run that writes the file.
//
// A BENCHMARK THAT CANNOT GO RED IS DECORATION. --check fails on an over-budget
// asset, naming it and the overage, exactly as it fails on a stale file. The
// budgets themselves are read back out of the roadmap and compared with the
// ones declared here, so an edit to section 4 fails loudly instead of silently
// grading against numbers that no longer exist. Writing the document still
// exits 0 while something is over — the record has to be regenerable while the
// art is being fixed — but it says so on stderr, and the gate is --check.
"use strict";

const fs = require("fs");
const path = require("path");
const zlib = require("zlib");
const { spawnSync } = require("child_process");

const ROOT = path.resolve(__dirname, "..", "..");
const ui = (f) => path.join(ROOT, "spheres-web", "ui", f);
const GENERATORS = ["equipment-mesh.js", "site-mesh.js", "town-mesh.js"];

const REQUIRE_MS = {};
function timedRequire(file) {
  const t = process.hrtime.bigint();
  const mod = require(ui(file));
  REQUIRE_MS[file] = Number(process.hrtime.bigint() - t) / 1e6;
  return mod;
}
const EquipmentMesh = timedRequire("equipment-mesh.js");
const SiteMesh = timedRequire("site-mesh.js");
const TownMesh = timedRequire("town-mesh.js");

// ------------------------------------------------------------------ budgets
// Each entry quotes the roadmap cell it was derived from, verbatim. verifyBudgets
// re-reads section 4 and refuses to run if that cell has changed: a budget table
// drifting away from the tool grading against it is precisely the failure this
// file exists to prevent.
const BUDGETS = {
  vehicle_lod0: { row: "Ground vehicle close inspection LOD0", cell: "20–45k triangles assembled", min: 20000, max: 45000 },
  vehicle_lod1: { row: "LOD1 catalogue preview", cell: "4–12k", min: 4000, max: 12000 },
  vehicle_lod2: { row: "LOD2 map vehicle", cell: "300–1,500", min: 300, max: 1500 },
  building_near: { row: "Building close view / map", cell: "2–12k / 100–800", min: 2000, max: 12000 },
  building_far: { row: "Building close view / map", cell: "2–12k / 100–800", min: 100, max: 800 },
  scene: { row: "Scene assembly", cell: "Target ≤150k visible triangles initially", min: null, max: 150000 },
};

function verifyBudgets() {
  const roadmap = fs.readFileSync(path.join(ROOT, "docs", "art", "3D_MODEL_MASTER_ROADMAP.md"), "utf8");
  const cells = new Map();
  for (const line of roadmap.split(/\r?\n/)) {
    if (!line.startsWith("|")) continue;
    const parts = line.split("|").map((s) => s.trim());
    if (parts.length >= 4) cells.set(parts[1], parts[2]);
  }
  const drift = [];
  for (const [key, b] of Object.entries(BUDGETS)) {
    const found = cells.get(b.row);
    if (found !== b.cell) {
      drift.push(`  ${key}: roadmap row "${b.row}" now reads ${found === undefined ? "(row missing)" : `"${found}"`}`
        + `, this tool was derived from "${b.cell}"`);
    }
  }
  if (drift.length) {
    console.error("roadmap section 4 has moved under this tool; re-derive BUDGETS in tools/ui/bench_art.cjs:\n" + drift.join("\n"));
    process.exit(1);
  }
}

// ------------------------------------------------------------- mesh arithmetic
// Non-indexed, three Float32Array attributes, nine floats per triangle each.
// That is the arithmetic behind every byte figure in the document, so it is
// checked against the real arrays rather than assumed: a generator that starts
// emitting UVs or an index buffer stops this tool instead of quietly making
// every number in the file wrong.
const BYTES_PER_TRI = 3 /* vertices */ * 3 /* components */ * 4 /* bytes */ * 3 /* attributes */;
function uploadBytes(mesh, where) {
  const views = Object.keys(mesh).filter((k) => ArrayBuffer.isView(mesh[k]));
  const expect = ["positions", "normals", "colors"];
  const shaped = views.length === 3 && expect.every((k) => views.includes(k))
    && expect.every((k) => mesh[k] instanceof Float32Array && mesh[k].length === mesh.triangleCount * 9);
  if (!shaped) {
    console.error(`${where}: vertex layout changed (attributes: ${views.join(", ") || "none"}). `
      + "The triangles*3*3*4*3 upload arithmetic in tools/ui/bench_art.cjs no longer holds; fix the tool before trusting a budget.");
    process.exit(1);
  }
  const actual = expect.reduce((a, k) => a + mesh[k].byteLength, 0);
  if (actual !== mesh.triangleCount * BYTES_PER_TRI) {
    console.error(`${where}: ${actual} attribute bytes against ${mesh.triangleCount * BYTES_PER_TRI} predicted`);
    process.exit(1);
  }
  return actual;
}

const fmt = (n) => String(n).replace(/\B(?=(\d{3})+(?!\d))/g, ",");
const mib = (b) => `${(b / 1048576).toFixed(2)} MiB`;
const kib = (b) => `${(b / 1024).toFixed(1)} KiB`;
const budgetText = (b) => (b.min == null ? `≤ ${fmt(b.max)}` : `${fmt(b.min)}–${fmt(b.max)}`);
const dim = (n) => String(Math.round(n * 10) / 10); // the kit's widths carry float noise
const plural = (n, word) => `${n} ${word}${n === 1 ? "" : "s"}`;

function verdict(tris, budget) {
  if (budget.max != null && tris > budget.max) {
    const over = tris - budget.max;
    return { state: "OVER", over, text: `**OVER by ${fmt(over)} (${((over / budget.max) * 100).toFixed(1)}%)**` };
  }
  if (budget.min != null && tris < budget.min) return { state: "UNDER", over: 0, text: `under by ${fmt(budget.min - tris)}` };
  return { state: "PASS", over: 0, text: "PASS" };
}

// ------------------------------------------------------------------ vehicles
const PLATFORMS = ["tank_standard", "tank_heavy", "tank_light", "tank_destroyer",
  "ground_ifv", "ground_apc", "ground_recon", "ground_artillery", "ground_air_defense"];

// The legal component list is the simulation's, read the way
// check_equipment_mesh.cjs reads it, so a component added to the game is weighed
// here without anyone remembering to add it to a list in this file.
function catalogue() {
  const rows = [];
  for (const file of ["equipment_specs.rs", "equipment_ground.rs"]) {
    const src = fs.readFileSync(path.join(ROOT, "spheres-sim", "src", file), "utf8");
    const found = [...src.matchAll(/component!\("([^"]+)","[^"]+","([^"]+)"/g)].map((m) => [m[1], m[2]]);
    if (found.length < 30) { console.error(`${file}: only ${found.length} components parsed; the macro shape changed`); process.exit(1); }
    rows.push(...found);
  }
  return rows;
}

// The heaviest specification a player can actually order. Greedy coordinate
// ascent, one slot at a time, repeated until a whole pass buys nothing. The
// components are near enough independent that it converges immediately, and it
// is deterministic because the catalogue order is. It is a lower bound on the
// true maximum, not a proof of it, and the document says so.
function maxedSpec(platform, components) {
  const base = EquipmentMesh.build({ platform });
  let spec = { platform, components: { ...base.specification.components } };
  let best = base.triangleCount, builds = 1, passes = 0;
  for (let pass = 0; pass < 4; pass++) {
    let gained = false;
    passes = pass + 1;
    for (const [id, slot] of components) {
      if (!(slot in spec.components) || spec.components[slot] === id) continue;
      const candidate = { platform, components: { ...spec.components, [slot]: id } };
      const mesh = EquipmentMesh.build(candidate); builds++;
      if (mesh.specification.components[slot] !== id) continue; // the generator refused it for this chassis
      if (mesh.triangleCount > best) { best = mesh.triangleCount; spec = candidate; gained = true; }
    }
    if (!gained) break;
  }
  return { spec, mesh: EquipmentMesh.build(spec), builds, passes, base };
}

function measureVehicles() {
  const components = catalogue();
  const rows = [];
  let builds = 0, passes = 0;
  for (const platform of PLATFORMS) {
    const m = maxedSpec(platform, components);
    builds += m.builds; passes = Math.max(passes, m.passes);
    const changed = Object.entries(m.spec.components)
      .filter(([slot, id]) => m.base.specification.components[slot] !== id)
      .map(([slot, id]) => `${slot}=${id}`).sort();
    for (const [config, mesh, note] of [
      ["baseline", m.base, "default specification"],
      ["heaviest", m.mesh, changed.length ? changed.join(", ") : "no component adds geometry; baseline is the heaviest"],
    ]) {
      rows.push({
        asset: `ground.${platform}.baseline.v1`, platform, config, note,
        tris: mesh.triangleCount, bytes: uploadBytes(mesh, `${platform} ${config}`),
        parts: mesh.parts.length, budget: BUDGETS.vehicle_lod0,
        v: verdict(mesh.triangleCount, BUDGETS.vehicle_lod0),
      });
    }
  }
  // The ground renderer takes numeric LODs. This small probe records that
  // contract without pretending the LOD0 sweep grades every coarse variant.
  const lodProbe = [0, 1, 2].map(lod => EquipmentMesh.build({ platform: "tank_heavy", lod }).triangleCount);
  return { rows, builds: builds + 3, passes, components: components.length, lodProbe };
}

// --------------------------------------------------------------------- sites
// Every kind x stage x level x status x LOD. Status is in the sweep because it
// changes the mesh — a stopped site grows its own marker — so leaving it out
// would report a worst case that is not the worst case.
function measureSites() {
  const statuses = SiteMesh.statuses;
  const levels = Array.from({ length: SiteMesh.maxLevel }, (_, i) => i + 1);
  const rows = []; let builds = 0, stages = 0;
  for (const kind of SiteMesh.kinds()) {
    const meta = SiteMesh.meta(kind);
    const kindStages = SiteMesh.stages(kind);
    stages = Math.max(stages, kindStages.length);
    const row = { kind, name: meta.name || kind, placeholder: !!meta.placeholder };
    for (const lod of [0, 1]) {
      let lo = null, hi = null;
      for (const stage of kindStages) {
        for (const level of levels) {
          for (const status of statuses) {
            const mesh = SiteMesh.build(kind, stage.key, { level, status, lod: lod ? "far" : 0 });
            builds++;
            const at = { tris: mesh.triangleCount, config: `${stage.key}/L${level}/${status}` };
            if (!lo || at.tris < lo.tris) lo = at;
            if (!hi || at.tris > hi.tris) hi = { ...at, bytes: uploadBytes(mesh, `${kind} ${at.config}`), parts: mesh.parts.length };
          }
        }
      }
      const budget = lod ? BUDGETS.building_far : BUDGETS.building_near;
      row[lod ? "far" : "near"] = { lo, hi, budget, v: verdict(hi.tris, budget), vlo: verdict(lo.tris, budget) };
    }
    rows.push(row);
  }
  return { rows, builds, sweep: `${SiteMesh.kinds().length} kinds x ${stages} stages x ${levels.length} levels x ${statuses.length} statuses x 2 LODs` };
}

// ---------------------------------------------------------------- town blocks
// Eight ids per district. A block is seeded by its id, so one id is one sample
// of a generator that hands back a different street on the next tile.
const BLOCK_IDS = [1990, 1991, 1992, 1993, 1994, 1995, 1996, 1997];
function measureTownBlocks() {
  const rows = []; let builds = 0;
  for (const district of TownMesh.districts()) {
    const row = { district };
    for (const lod of ["close", "map"]) {
      let lo = null, hi = null;
      for (const id of BLOCK_IDS) {
        const mesh = TownMesh.block({ id, district, lod: lod === "map" ? "map" : undefined });
        builds++;
        const at = { tris: mesh.triangleCount, config: `id ${id}`, lots: mesh.lots ? mesh.lots.length : null };
        if (!lo || at.tris < lo.tris) lo = at;
        if (!hi || at.tris > hi.tris) hi = { ...at, bytes: uploadBytes(mesh, `town ${district} ${lod}`), parts: mesh.parts.length };
      }
      row[lod] = { lo, hi, budget: BUDGETS.scene, v: verdict(hi.tris, BUDGETS.scene) };
    }
    rows.push(row);
  }
  return { rows, builds, sweep: `${TownMesh.districts().length} districts x ${BLOCK_IDS.length} ids x 2 LODs` };
}

// The kit the blocks are assembled from, at the largest footprint and the
// tallest storey count each kind admits. These are constituents of a block, not
// separately resident assets, so they are graded but never added to the
// resident total — that would count the same triangles twice.
function measureTownBuildings() {
  const rows = []; let builds = 0;
  for (const kind of TownMesh.kinds()) {
    const info = TownMesh.kindInfo(kind);
    const width = Math.max(...info.widths), storeys = Math.max(...info.storeys);
    const row = { kind, label: info.label, width, storeys, depth: info.depth };
    for (const lod of ["close", "map"]) {
      let hi = null;
      for (const id of [0, 1, 2, 3]) {
        const mesh = TownMesh.building(kind, { id, width, storeys, lod: lod === "map" ? "map" : undefined });
        builds++;
        if (!hi || mesh.triangleCount > hi.tris) hi = { tris: mesh.triangleCount, bytes: uploadBytes(mesh, `${kind} ${lod}`), parts: mesh.parts.length };
      }
      const budget = lod === "map" ? BUDGETS.building_far : BUDGETS.building_near;
      row[lod] = { hi, budget, v: verdict(hi.tris, budget) };
    }
    rows.push(row);
  }
  return { rows, builds, sweep: `${TownMesh.kinds().length} kinds at maximum width and storeys x 4 seeds x 2 LODs` };
}

// -------------------------------------------------------------- source weight
function measureSource() {
  const files = GENERATORS.map((f) => {
    const buf = fs.readFileSync(ui(f));
    return { file: `spheres-web/ui/${f}`, bytes: buf.length, gzip: zlib.gzipSync(buf, { level: 9 }).length };
  });
  const dir = path.join(ROOT, "spheres-web", "ui", "equipment-models");
  const glb = fs.existsSync(dir)
    ? fs.readdirSync(dir).filter((f) => f.endsWith(".glb")).sort()
      .map((f) => ({ file: f, bytes: fs.statSync(path.join(dir, f)).size }))
    : [];
  return {
    files, glb,
    bytes: files.reduce((a, f) => a + f.bytes, 0),
    gzip: files.reduce((a, f) => a + f.gzip, 0),
    glbBytes: glb.reduce((a, f) => a + f.bytes, 0),
  };
}

// ------------------------------------------------------------------- timings
// Never written to the document (roadmap 7.6). Printed to stdout, where a
// number that differs between two machines is information rather than a diff.
const COLD = {
  vehicle: { module: "equipment-mesh.js", label: "EquipmentMesh.build tank_heavy", run: () => EquipmentMesh.build({ platform: "tank_heavy" }) },
  site_near: { module: "site-mesh.js", label: "SiteMesh.build arms_plant complete L5", run: () => SiteMesh.build("arms_plant", "complete", { level: 5 }) },
  site_far: { module: "site-mesh.js", label: "SiteMesh.build arms_plant complete L5 far", run: () => SiteMesh.build("arms_plant", "complete", { level: 5, lod: "far" }) },
  block_close: { module: "town-mesh.js", label: "TownMesh.block residential", run: () => TownMesh.block({ id: 1990, district: "residential" }) },
  block_map: { module: "town-mesh.js", label: "TownMesh.block residential map", run: () => TownMesh.block({ id: 1990, district: "residential", lod: "map" }) },
  building: { module: "town-mesh.js", label: "TownMesh.building house", run: () => TownMesh.building("house", { id: 0 }) },
};

function warmProfile(fn) {
  fn(-1); // the first call is the cold path and is measured in its own process
  const samples = [];
  const started = Date.now();
  while (samples.length < 5000 && (samples.length < 20 || Date.now() - started < 600)) {
    const t = process.hrtime.bigint();
    fn(samples.length);
    samples.push(Number(process.hrtime.bigint() - t) / 1e6);
  }
  const mean = samples.reduce((a, b) => a + b, 0) / samples.length;
  const sd = Math.sqrt(samples.reduce((a, b) => a + (b - mean) ** 2, 0) / samples.length);
  const sorted = [...samples].sort((a, b) => a - b);
  return { n: samples.length, mean, median: sorted[sorted.length >> 1], min: sorted[0], max: sorted[sorted.length - 1], rse: sd / Math.sqrt(samples.length) / mean };
}

function coldProfile(name) {
  const r = spawnSync(process.execPath, [__filename, "--cold", name], { encoding: "utf8" });
  if (r.status !== 0) return null;
  try { return JSON.parse(r.stdout.trim().split("\n").pop()); } catch { return null; }
}

// --------------------------------------------------------------------- report
function render(m) {
  const over = m.over;
  const gradeCount = (state) => m.graded.filter((g) => g.state === state).length;

  const vehicleRows = m.vehicles.rows.map((r) =>
    `| \`${r.asset}\` | ${r.config} | ${fmt(r.tris)} | ${budgetText(r.budget)} | ${r.v.text} | ${fmt(r.bytes)} | ${r.parts} |`).join("\n");

  const siteRows = m.sites.rows.map((r) =>
    `| \`site.${r.kind}.v1\` | ${fmt(r.near.lo.tris)} | ${fmt(r.near.hi.tris)} | ${r.near.v.text} | `
    + `${fmt(r.far.lo.tris)} | ${fmt(r.far.hi.tris)} | ${r.far.v.text} | ${fmt(r.near.hi.bytes)} |`).join("\n");

  const siteWorst = m.sites.rows.map((r) =>
    `| \`site.${r.kind}.v1\` | ${r.near.hi.config} | ${r.near.hi.parts} | ${r.far.hi.config} | ${r.far.hi.parts} |`).join("\n");

  const blockRows = m.blocks.rows.map((r) =>
    `| \`town.temperate.${r.district}.v1\` | ${fmt(r.close.lo.tris)} | ${fmt(r.close.hi.tris)} | ${r.close.v.text} | `
    + `${fmt(r.map.lo.tris)} | ${fmt(r.map.hi.tris)} | ${fmt(r.close.hi.bytes)} | ${r.close.hi.lots == null ? "—" : r.close.hi.lots} |`).join("\n");

  const kitRows = m.buildings.rows.map((r) =>
    `| \`${r.kind}\` | ${dim(r.width)} x ${dim(r.depth)} m, ${plural(r.storeys, "storey")} | ${fmt(r.close.hi.tris)} | ${r.close.v.text} | `
    + `${fmt(r.map.hi.tris)} | ${r.map.v.text} |`).join("\n");

  // Said from the data rather than from expectation: which kit pieces, if any,
  // fall under the building floor, and how much headroom the tightest pass has.
  const kitUnder = m.buildings.rows.flatMap((r) => [
    ...(r.close.v.state === "UNDER" ? [`\`${r.kind}\` close at ${fmt(r.close.hi.tris)}`] : []),
    ...(r.map.v.state === "UNDER" ? [`\`${r.kind}\` map at ${fmt(r.map.hi.tris)}`] : []),
  ]);
  const kitNote = kitUnder.length
    ? `${kitUnder.length} kit measurement${kitUnder.length === 1 ? "" : "s"} land under the building floor — ${kitUnder.join(", ")} — `
      + "because those pieces are props wearing a building's budget. Section 4 has a tree/prop row and no rule for which "
      + "kit pieces belong to it, so under-floor here is a classification gap in the roadmap, not a cost problem."
    : "Every kit piece stays inside the building row at the largest size its kind admits, at both LODs — including the "
      + "props (`park`, `utility`, `stadium`), which section 4 gives no rule for classifying and which would want the "
      + "tree/prop row rather than this one.";

  const tightest = [...m.graded].filter((g) => g.state === "PASS" && g.budget.max)
    .map((g) => ({ ...g, use: g.tris / g.budget.max }))
    .sort((a, b) => b.use - a.use || a.asset.localeCompare(b.asset)).slice(0, 5)
    .map((g) => `| \`${g.asset}\` | ${g.config} | ${fmt(g.tris)} | ${fmt(g.budget.max)} | ${(g.use * 100).toFixed(1)}% |`).join("\n");

  const sourceRows = m.source.files.map((f) =>
    `| \`${f.file}\` | ${fmt(f.bytes)} | ${kib(f.bytes)} |`).join("\n");

  const overSection = over.length
    ? `${over.length} measured configuration${over.length === 1 ? " is" : "s are"} over budget. \`--check\` exits 1 while any row here has content.\n\n`
      + "| asset | configuration | measured | budget ceiling | over by |\n| --- | --- | --- | --- | --- |\n"
      + over.map((o) => `| \`${o.asset}\` | ${o.config} | ${fmt(o.tris)} | ${fmt(o.budget.max)} | ${fmt(o.over)} (${((o.over / o.budget.max) * 100).toFixed(1)}%) |`).join("\n")
      + "\n\nWidening the roadmap budget to make this table empty is the one repair\n"
      + "this harness exists to forbid. Either the mesh loses the triangles, or\n"
      + "section 4's ceiling is deliberately re-argued and re-derived — a design\n"
      + "decision, recorded as such, not a quiet edit to a number in a table."
    : "Nothing measured is over its budget.";

  return `# P0 art budgets — measured

Generated by \`node tools/ui/bench_art.cjs\` — do not edit by hand. Every count in
this file is read off a mesh this tool built; nothing is quoted from a design
document, and the budgets it is graded against are re-read from roadmap section 4
on every run, so a budget edited there fails this tool rather than silently
changing a verdict.

    node tools/ui/bench_art.cjs           regenerate this file, print timings to stdout
    node tools/ui/bench_art.cjs --check   exit 1 if this file is stale or anything is over budget

## What this file deliberately does not contain

Roadmap 7.6 asks for canonical outputs that reproduce, so there are no timings,
no dates and no machine names below — counts, bytes and verdicts only, which are
properties of the art and are identical on any machine that runs the same
generators. Build times are measured by the same run and printed to stdout.

## What a Node harness cannot measure

Stated plainly, because these numbers are easy to mistake for performance:

- **Frame time and frame rate.** Nothing here draws a pixel. There is no GL
  context, no rasteriser and no shader in this process, so nothing in this file
  is evidence for the roadmap's 60fps viewer or 30fps map targets.
- **GPU memory as the driver actually allocates it.** The byte figures are the
  size of the attribute arrays as JavaScript produces them. A driver aligns and
  pads buffers, may keep a shadow copy in system memory, and charges per-buffer
  overhead on top. Read the totals as a floor on GPU memory, never as an
  allocation.
- **Upload stalls.** How long \`bufferData\` blocks, whether it lands mid-frame,
  and what the driver does on re-specification are runtime properties of the
  real WebGL path.
- **Everything resolution-dependent**: overdraw, fill rate, depth complexity,
  MSAA cost. A cheap mesh can still cost a frame if it covers the screen twice.
- **Vertex processing efficiency.** These meshes are non-indexed, so the
  post-transform vertex cache never hits and every triangle transforms three
  vertices. That is a real cost this harness prices only as triangles.
- **Draw calls, state changes and batching** as \`arsenal3d.js\` issues them, and
  everything about LOD switching in motion — hysteresis, popping, cache churn.
- **The browser conditions roadmap 7.8 asks for**: 390px width, WebGL
  unavailable, context loss and restore, keyboard selection. Those need a
  browser and are the job of the check tools that drive one.

What it *does* measure honestly: how many triangles the generators emit, how
many bytes those triangles occupy in a vertex buffer, and how many bytes of
source the game downloads to be able to make them.

## Budgets applied

Read from roadmap section 4. The cell text is compared on every run; if section
4 changes, this tool stops instead of grading against a number that has moved.

| class | roadmap row | roadmap cell | applied as | applied to |
| --- | --- | --- | --- | --- |
| vehicle LOD0 | ${BUDGETS.vehicle_lod0.row} | ${BUDGETS.vehicle_lod0.cell} | ${budgetText(BUDGETS.vehicle_lod0)} tris | every ground platform, baseline and heaviest |
| building near | ${BUDGETS.building_near.row} | ${BUDGETS.building_near.cell} | ${budgetText(BUDGETS.building_near)} tris | construction sites LOD0, town kit close |
| building far | ${BUDGETS.building_far.row} | ${BUDGETS.building_far.cell} | ${budgetText(BUDGETS.building_far)} tris | construction sites LOD1, town kit map |
| scene assembly | ${BUDGETS.scene.row} | ${BUDGETS.scene.cell} | ${budgetText(BUDGETS.scene)} tris | one town block |

\`OVER\` means above the ceiling and fails \`--check\`. \`under\` means below the
floor of a range: that is a detail-density note, not a performance risk, and it
does not fail. A budget with only a ceiling can only be \`PASS\` or \`OVER\`.

## Verdicts

${m.graded.length} graded configurations: ${gradeCount("PASS")} PASS, ${gradeCount("UNDER")} under the detail floor, ${gradeCount("OVER")} over the ceiling.

The geometry sweep covers nine ground platforms, construction sites and town
assets. The export inventory below also includes tactical aircraft; aircraft
geometry is listed in [P0_MANIFEST.md](P0_MANIFEST.md) but is not graded by this ground-only
vehicle budget sweep.

${overSection}

## Ground vehicles, LOD0

\`baseline\` is \`EquipmentMesh.build({platform})\` with no components named.
\`heaviest\` is the costliest specification found by greedy per-slot ascent over
all ${m.vehicles.components} components the simulation defines in
\`equipment_specs.rs\` and \`equipment_ground.rs\`, keeping only the ones the
generator accepts for that chassis. It is a ceiling on the ART: greed is a lower
bound on the true maximum rather than a proof of it, and the simulation's own
compatibility matrix may refuse some of these combinations as designs. What it
answers is the question the budget asks — how heavy can this platform get.

| asset | configuration | LOD0 tris | budget | verdict | upload bytes | parts |
| --- | --- | --- | --- | --- | --- | --- |
${vehicleRows}

Components that make each platform heaviest:

${m.vehicles.rows.filter((r) => r.config === "heaviest").map((r) => `- \`${r.platform}\`: ${r.note}`).join("\n")}

Ground equipment has numeric LOD0, LOD1 and LOD2. A baseline \`tank_heavy\`
probe measures ${m.vehicles.lodProbe.map(fmt).join(" / ")} triangles respectively.
The graded vehicle sweep above remains LOD0-only; this probe is not a complete
coarse-configuration budget audit. [P0_MANIFEST.md](P0_MANIFEST.md) records all
ground baselines at each detail level, and \`check_equipment_mesh.cjs\` checks
the ${BUDGETS.vehicle_lod1.cell} catalogue and ${BUDGETS.vehicle_lod2.cell} map bands
across individual and combined component choices. Aircraft currently have
inspection geometry only and are outside this vehicle sweep.

## Construction sites

Swept over ${m.sites.sweep} = ${fmt(m.sites.builds)} builds. \`min\` and \`max\` are
across that whole sweep, so \`max\` is the worst case the sim can ask for.

| asset | near min | near max | near verdict | far min | far max | far verdict | worst-case near bytes |
| --- | --- | --- | --- | --- | --- | --- | --- |
${siteRows}

Which configuration is the worst case, and how many selectable parts it carries:

| asset | worst near | parts | worst far | parts |
| --- | --- | --- | --- | --- |
${siteWorst}

## Town blocks

Swept over ${m.blocks.sweep} = ${fmt(m.blocks.builds)} builds. A block is one
${TownMesh.defaultTile[0]} x ${TownMesh.defaultTile[1]} m tile of ${TownMesh.era} temperate town,
graded against the scene-assembly ceiling because a block is a scene, not a
building.

| asset | close min | close max | close verdict | map min | map max | worst close bytes | lots |
| --- | --- | --- | --- | --- | --- | --- | --- |
${blockRows}

The map LOD has no roadmap row of its own — section 4 budgets a scene assembly
and a building, not a coarse scene — so the map column is recorded without a
verdict. The heaviest map block is ${fmt(m.blockMapMax)} triangles, which is
${(m.blockMapMax / BUDGETS.building_far.max).toFixed(1)}x the building-far ceiling
of ${fmt(BUDGETS.building_far.max)}; that says the row is the wrong one for a whole
tile of town, not that the mesh is wrong. A coarse-scene budget is a gap in
section 4, and until it exists the map block is measured and left ungraded rather
than graded against a number written for one building.

## Town building kit

The pieces a block is assembled from, each at the largest width and tallest
storey count its kind admits (${m.buildings.sweep} = ${fmt(m.buildings.builds)} builds).
These are constituents of the blocks above, not separately resident assets, so
they are graded but never added into the resident totals — that would count the
same triangles twice.

| kind | worst case | close tris | close verdict | map tris | map verdict |
| --- | --- | --- | --- | --- | --- |
${kitRows}

${kitNote}

## Tightest passes

The five graded configurations closest to their ceiling. Nothing here is a
failure; this is where the next art pass will push something over.

| asset | configuration | triangles | ceiling | of ceiling |
| --- | --- | --- | --- | --- |
${tightest}

## Resident cost, if the measured ground, site and town set were resident at once

No frame draws this. It is the measured set held at once, which is the
number that decides whether a bounded cache can keep everything rather than
rebuild it.

| set | assets | triangles | upload bytes |
| --- | --- | --- | --- |
| Ground vehicles, heaviest specification | ${m.resident.vehicles.n} | ${fmt(m.resident.vehicles.tris)} | ${fmt(m.resident.vehicles.bytes)} |
| Construction sites, worst case near | ${m.resident.sites.n} | ${fmt(m.resident.sites.tris)} | ${fmt(m.resident.sites.bytes)} |
| Town blocks, worst case close | ${m.resident.blocks.n} | ${fmt(m.resident.blocks.tris)} | ${fmt(m.resident.blocks.bytes)} |
| **Measured set, close detail** | **${m.resident.total.n}** | **${fmt(m.resident.total.tris)}** | **${fmt(m.resident.total.bytes)}** (${mib(m.resident.total.bytes)}) |
| Sites/towns coarse; ground vehicles retained at LOD0 for comparison | ${m.resident.map.n} | ${fmt(m.resident.map.tris)} | ${fmt(m.resident.map.bytes)} (${mib(m.resident.map.bytes)}) |

The comparison row deliberately retains the measured vehicles at LOD0:
${fmt(m.resident.map.vehicleShare)} of its ${fmt(m.resident.map.tris)} triangles are
the nine ground vehicles, with ${fmt(m.resident.map.tris - m.resident.map.vehicleShare)}
for the coarse sites and blocks. It is not the live map's rendering cost and
does not imply the available ground LOD1/LOD2 geometry is unused.

Upload arithmetic, for every byte figure above: the meshes are non-indexed with
three \`Float32Array\` attributes, so bytes = triangles x 3 vertices x 3
components x 4 bytes x 3 attributes = triangles x ${BYTES_PER_TRI}. The tool
checks that against the real \`byteLength\` of every mesh it measures and stops if
a generator ever stops matching it.

## Source cost — what the player actually downloads

The generators ship as source and build their meshes in the browser without
fetching GLB assets or requiring a build step. These figures measure the three
geometry generators; renderer, stylesheet and other page costs are not included.

| file | bytes | |
| --- | --- | --- |
${sourceRows}
| **total** | **${fmt(m.source.bytes)}** | **${kib(m.source.bytes)}** |

${fmt(m.source.bytes)} bytes of source produce ${fmt(m.resident.total.tris)} triangles of
geometry — ${fmt(Math.round(m.resident.total.bytes / m.source.bytes))}x its own weight in vertex data. That ratio is not fixed
at authoring time either: it grows with every extra seed, stage, level and
district asked of the same source.

${m.source.glb.length} exported \`.glb\` files sit in \`spheres-web/ui/equipment-models/\`
totalling ${fmt(m.source.glbBytes)} bytes (${mib(m.source.glbBytes)}). They are the
portable deliverable roadmap section 4 asks for, not a runtime download — the game
never fetches them — and they are the comparison that settles the argument:
${m.source.glb.length} equipment exports (${m.source.glb.filter(f => f.file.startsWith("spheres-air-")).length} aircraft and ${m.source.glb.filter(f => !f.file.startsWith("spheres-air-")).length} ground-vehicle configurations) as binary assets weigh
${(m.source.glbBytes / m.source.bytes).toFixed(1)}x the entire generator source
that builds every vehicle, every site at every stage and every town block.

## Method

- Every number is measured by building the mesh and reading \`triangleCount\`,
  \`parts\` and the attribute arrays. Nothing is copied from another document.
- Sweeps: vehicles ${fmt(m.vehicles.builds)} builds, sites ${fmt(m.sites.builds)},
  town blocks ${fmt(m.blocks.builds)}, town kit ${fmt(m.buildings.builds)}.
- The vehicle worst case is greedy coordinate ascent over the simulation's own
  component catalogue, repeated until a pass buys nothing (${m.vehicles.passes} pass${m.vehicles.passes === 1 ? "" : "es"}).
- Build times are measured cold in a fresh process and warm in a loop, and are
  printed to stdout rather than written here.
`;
}

// ----------------------------------------------------------------------- main
function main() {
  const argv = process.argv.slice(2);

  const coldIndex = argv.indexOf("--cold");
  if (coldIndex >= 0) {
    const name = argv[coldIndex + 1];
    const c = COLD[name];
    if (!c) { console.error(`unknown cold case ${name}`); process.exit(2); }
    const t = process.hrtime.bigint();
    const mesh = c.run();
    const build = Number(process.hrtime.bigint() - t) / 1e6;
    console.log(JSON.stringify({ name, label: c.label, require: REQUIRE_MS[c.module], build, tris: mesh.triangleCount }));
    return;
  }

  verifyBudgets();

  const vehicles = measureVehicles();
  const sites = measureSites();
  const blocks = measureTownBlocks();
  const buildings = measureTownBuildings();
  const source = measureSource();

  // Everything graded in one list, so the summary and the failure list cannot
  // disagree with the tables above them.
  const graded = [];
  for (const r of vehicles.rows) graded.push({ asset: r.asset, config: r.config, tris: r.tris, budget: r.budget, ...r.v });
  for (const r of sites.rows) {
    graded.push({ asset: `site.${r.kind}.v1`, config: `near ${r.near.hi.config}`, tris: r.near.hi.tris, budget: r.near.budget, ...r.near.v });
    graded.push({ asset: `site.${r.kind}.v1`, config: `far ${r.far.hi.config}`, tris: r.far.hi.tris, budget: r.far.budget, ...r.far.v });
  }
  for (const r of blocks.rows) graded.push({ asset: `town.temperate.${r.district}.v1`, config: `close ${r.close.hi.config}`, tris: r.close.hi.tris, budget: r.close.budget, ...r.close.v });
  for (const r of buildings.rows) {
    graded.push({ asset: `town.kit.${r.kind}`, config: "close, maximum size", tris: r.close.hi.tris, budget: r.close.budget, ...r.close.v });
    graded.push({ asset: `town.kit.${r.kind}`, config: "map, maximum size", tris: r.map.hi.tris, budget: r.map.budget, ...r.map.v });
  }
  const over = graded.filter((g) => g.state === "OVER");

  const heaviest = vehicles.rows.filter((r) => r.config === "heaviest");
  const sum = (rows, pick) => rows.reduce((a, r) => a + pick(r), 0);
  const set = (n, tris) => ({ n, tris, bytes: tris * BYTES_PER_TRI });
  const vehicleSet = set(heaviest.length, sum(heaviest, (r) => r.tris));
  const siteSet = set(sites.rows.length, sum(sites.rows, (r) => r.near.hi.tris));
  const blockSet = set(blocks.rows.length, sum(blocks.rows, (r) => r.close.hi.tris));
  const mapTris = vehicleSet.tris + sum(sites.rows, (r) => r.far.hi.tris) + sum(blocks.rows, (r) => r.map.hi.tris);
  const resident = {
    vehicles: vehicleSet, sites: siteSet, blocks: blockSet,
    total: set(vehicleSet.n + siteSet.n + blockSet.n, vehicleSet.tris + siteSet.tris + blockSet.tris),
    map: { ...set(vehicleSet.n + siteSet.n + blockSet.n, mapTris), vehicleShare: vehicleSet.tris },
  };

  const blockMapMax = Math.max(...blocks.rows.map((r) => r.map.hi.tris));
  const md = render({ vehicles, sites, blocks, buildings, source, graded, over, resident, blockMapMax });
  const outMd = path.join(ROOT, "docs", "art", "P0_BUDGETS.md");

  if (argv.includes("--check")) {
    const stale = !fs.existsSync(outMd) || fs.readFileSync(outMd, "utf8").replace(/\r\n/g, "\n") !== md;
    if (stale) console.error(`stale, re-run without --check:\n  ${outMd}`);
    for (const o of over) {
      console.error(`over budget: ${o.asset} (${o.config}) is ${fmt(o.tris)} triangles against a ceiling of `
        + `${fmt(o.budget.max)} — over by ${fmt(o.over)} (${((o.over / o.budget.max) * 100).toFixed(1)}%)`);
    }
    if (stale || over.length) process.exit(1);
    console.log(`budgets current: ${graded.length} graded configurations, all inside roadmap section 4`);
    return;
  }

  fs.mkdirSync(path.dirname(outMd), { recursive: true });
  fs.writeFileSync(outMd, md, "utf8");

  const builds = vehicles.builds + sites.builds + blocks.builds + buildings.builds;
  console.log(`wrote docs/art/P0_BUDGETS.md — ${graded.length} graded configurations from ${fmt(builds)} builds`);
  console.log(`  triangles (everything resident, close detail): ${fmt(resident.total.tris)}`);
  console.log(`  upload bytes:                                  ${fmt(resident.total.bytes)} (${mib(resident.total.bytes)})`);
  console.log(`  generator source:                              ${fmt(source.bytes)} (${kib(source.bytes)}), ${fmt(source.gzip)} gzipped (${kib(source.gzip)})`);

  console.log("\nbuild time — cold is the first build in a fresh process, warm is the mean of a loop in a hot one.");
  console.log("(neither is written to the document: roadmap 7.6 wants that file byte-identical between runs)");
  console.log("case                                       require     cold  warm mean   median      min     n    rse");
  for (const [name, c] of Object.entries(COLD)) {
    const cold = coldProfile(name);
    const w = warmProfile(() => c.run());
    const pad = (s, n) => String(s).padEnd(n);
    const num = (v, n, d = 2) => String(v.toFixed(d)).padStart(n);
    console.log(`${pad(c.label, 42)} ${cold ? num(cold.require, 6) : "     ?"}  ${cold ? num(cold.build, 7) : "      ?"}  `
      + `${num(w.mean, 9)}  ${num(w.median, 7)}  ${num(w.min, 7)}  ${String(w.n).padStart(4)}  ${(w.rse * 100).toFixed(1)}%`);
  }
  console.log("all times in milliseconds. The warm loop runs at least 20 builds and then keeps going for 600 ms, capped at 5,000;");
  console.log("rse is the relative standard error of that mean, so a row is worth reading to about that fraction of itself.");
  console.log("town-mesh.js memoises baked variants module-wide, so a warm block is mostly stamping cached geometry; cold is the true first-paint cost.");

  if (over.length) {
    console.error(`\n${over.length} configuration${over.length === 1 ? "" : "s"} OVER budget — \`--check\` will exit 1:`);
    for (const o of over) console.error(`  ${o.asset} (${o.config}): ${fmt(o.tris)} against ${fmt(o.budget.max)}, over by ${fmt(o.over)}`);
  }
}

main();
