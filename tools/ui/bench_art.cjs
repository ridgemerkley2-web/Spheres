#!/usr/bin/env node
// The P0 art budget harness — measured, not promised.
//
// Roadmap section 4 prints a budget table and says outright that those are
// "initial budgets to validate on the user's machine, not measured performance
// promises". The P0 exit gate then asks for "measured performance recorded".
// This tool measures the ground-vehicle, site and town set, counts the
// triangles the generator emits, records CPU mesh and base attribute payload
// bytes, weighs the source that produces them, and states PASS,
// UNDER or OVER against the roadmap's own numbers.
//
//   node tools/ui/bench_art.cjs            -> docs/art/P0_BUDGETS.md, timings on stdout
//   node tools/ui/bench_art.cjs --check    -> exit 1 if the committed file is stale
//                                             OR a ceiling/required floor fails
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
const {measureMesh, sumPayloads} = require('./mesh-accounting.cjs');
const {measureBuildingUnits} = require('./art-budget-units.cjs');

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
const Arsenal3D = require(ui('arsenal3d.js'));

// ------------------------------------------------------------------ budgets
// Each entry quotes the roadmap cell it was derived from, verbatim. verifyBudgets
// re-reads section 4 and refuses to run if that cell has changed: a budget table
// drifting away from the tool grading against it is precisely the failure this
// file exists to prevent.
const BUDGETS = {
  vehicle_lod0: { row: "Tank inspection LOD0", cell: "20–150k; actual GLB <12 MB", min: 20000, max: 150000 },
  specialist_lod0: { row: "Armoured specialist inspection LOD0", cell: "8–48k; actual GLB <5 MB", min: 8000, max: 48000 },
  aircraft_lod0: { row: "Aircraft inspection LOD0", cell: "100–250k; actual GLB <28 MB", min: 100000, max: 250000, requiredMin: true },
  vehicle_lod1: { row: "LOD1 catalogue preview", cell: "4–12k", min: 4000, max: 12000 },
  vehicle_lod2: { row: "LOD2 map vehicle", cell: "300–1,500", min: 300, max: 1500 },
  building_near: { row: "Building close view / map", cell: "2–12k / 100–800", min: 2000, max: 12000 },
  building_far: { row: "Building close view / map", cell: "2–12k / 100–800", min: 100, max: 800 },
  scene: { row: "Scene assembly", cell: "Target ≤150k visible triangles initially", min: null, max: 150000 },
};

// Preserve the superseded proposal as a visible diagnostic, without pretending
// it can also satisfy the later 100k+ aircraft requirement or grade a campus as
// one building. Current budgets above come from the existing quality/export
// contracts; the reconciliation is documented in roadmap section 4.
const LEGACY_GROUND = {min:20000,max:45000};
const LEGACY_AIRCRAFT = {min:25000,max:60000};

function collectBuildings(worst, mesh, config, where) {
  const units = measureBuildingUnits(mesh, where);
  for (const b of units.buildings) {
    if (!worst.has(b.id) || b.triangles > worst.get(b.id).tris) {
      worst.set(b.id, {id:b.id, label:b.label, config, tris:b.triangles,
        ownedTriangles:b.ownedTriangles, sharedTriangles:b.sharedTriangles,
        budget:BUDGETS.building_near, v:verdict(b.triangles,BUDGETS.building_near)});
    }
  }
  return units;
}

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
// Material-class bytes belong to the CPU mesh; the viewer derives a separate
// float surface attribute. Base uploads are not the viewer's total residency.
function payload(mesh, where) {
  const accounting = measureMesh(mesh, where);
  return {bytes: accounting.base_attribute_upload_bytes, accounting};
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

function budgetFailed(row) {
  return row.state === 'OVER' || row.state === 'UNDER' && row.budget.requiredMin === true;
}

// ------------------------------------------------------------------ vehicles
const PLATFORMS = ["tank_standard", "tank_heavy", "tank_light", "tank_destroyer",
  "ground_ifv", "ground_apc", "ground_recon", "ground_artillery", "ground_air_defense"];

// The legal component list is the simulation's, read the way
// check_equipment_mesh.cjs reads it, so a component added to the game is weighed
// here without anyone remembering to add it to a list in this file.
function catalogue() {
  const rows = [];
  for (const [file,minimum] of [["equipment.rs",11],["equipment_specs.rs",30],["equipment_ground.rs",30]]) {
    const src = fs.readFileSync(path.join(ROOT, "spheres-sim", "src", file), "utf8");
    const found = [...src.matchAll(/component!\("([^"]+)","[^"]+","([^"]+)"/g)].map((m) => [m[1], m[2]]);
    if (found.length < minimum) { console.error(`${file}: only ${found.length} components parsed; the macro shape changed`); process.exit(1); }
    rows.push(...found);
  }
  return rows;
}

// A heavy generator-accepted specification, not a native compatibility proof.
// Defaults omit optional tank slots, so discover and seed every accepted slot
// before greedy coordinate ascent. Otherwise tracks, turret, suspension and
// the other omitted slots never enter the search. Catalogue order makes this
// deterministic; up to four greedy passes per seed remain a lower bound, not
// an exhaustive maximum. Heavy armor has coupled turret/protection choices,
// so its existing fully loaded regression fixture is a second starting point.
// The untouched default is measured separately.
function maxedSpec(platform, components) {
  const base = EquipmentMesh.build({ platform });
  let spec = { platform, components: { ...base.specification.components } };
  let builds = 1, passes = 0;
  for (const [id, slot] of components) {
    if (Object.hasOwn(spec.components, slot)) continue;
    const mesh = EquipmentMesh.build({platform, components:{...spec.components, [slot]:id}});
    builds++;
    if (mesh.specification.components[slot] === id) {
      spec = {platform, components:{...mesh.specification.components}};
    }
  }
  const seeds = [spec];
  if (platform === 'tank_heavy') {
    // Same authored fixture as check_equipment_mesh's fully loaded vehicle.
    // Measure its real mesh; do not substitute its old recorded triangle count.
    const loaded = EquipmentMesh.build({platform, components:{
      mobility:'engine_turbine_1500', transmission:'transmission_auto', tracks:'tracks_wide',
      suspension:'suspension_hydro', turret:'turret_heavy', armament:'gun_125', ammunition:'ammo_penetrator',
      protection:'protection_heavy', active_protection:'aps_hard', sensors:'optics_thermal',
      fire_control:'fcs_digital', communications:'comms_data'
    }});
    builds++;
    seeds.push({platform, components:{...loaded.specification.components}});
  }
  let winner = null;
  for (const seed of seeds) {
    spec = seed;
    let best = EquipmentMesh.build(spec).triangleCount;
    builds++;
    for (let pass = 0; pass < 4; pass++) {
      let gained = false;
      passes = Math.max(passes, pass + 1);
      for (const [id, slot] of components) {
        if (!(slot in spec.components) || spec.components[slot] === id) continue;
        const candidate = { platform, components: { ...spec.components, [slot]: id } };
        const mesh = EquipmentMesh.build(candidate); builds++;
        if (mesh.specification.components[slot] !== id) continue; // rejected for this chassis
        if (mesh.triangleCount > best) {
          best = mesh.triangleCount;
          spec = {platform, components:{...mesh.specification.components}};
          gained = true;
        }
      }
      if (!gained) break;
    }
    if (!winner || best > winner.triangles) winner = {spec, triangles:best};
  }
  spec = winner.spec;
  return { spec, mesh: EquipmentMesh.build(spec), builds, passes, base };
}

function measureVehicles() {
  const components = catalogue();
  const rows = [];
  let builds = 0, passes = 0;
  for (const platform of PLATFORMS) {
    const m = maxedSpec(platform, components);
    const inspectionBudget = platform.startsWith('tank_') ? BUDGETS.vehicle_lod0 : BUDGETS.specialist_lod0;
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
        tris: mesh.triangleCount, ...payload(mesh, `${platform} ${config}`),
        parts: mesh.parts.length, budget: inspectionBudget,
        v: verdict(mesh.triangleCount, inspectionBudget),
      });
    }
  }
  // The ground renderer takes numeric LODs. This small probe records that
  // contract without pretending the LOD0 sweep grades every coarse variant.
  const lodProbe = [0, 1, 2].map(lod => EquipmentMesh.build({ platform: "tank_heavy", lod }).triangleCount);
  return { rows, builds: builds + 3, passes, components: components.length, lodProbe };
}

// Grade actual cheaper meshes too; baseline probes do not prove every possible
// component combination, and are labelled accordingly in the report.
function measureDetailLevels() {
  const rows = [];
  for (const platform of [...PLATFORMS, 'air_fighter', 'air_light_attack', 'air_tactical_strike']) {
    for (const lod of [0, 1, 2]) {
      if (lod === 0 && !platform.startsWith('air_')) continue;
      const mesh = EquipmentMesh.build({platform, lod});
      const budget = lod === 0 ? BUDGETS.aircraft_lod0 : BUDGETS[`vehicle_lod${lod}`];
      rows.push({asset: `${platform.startsWith('air_') ? 'aviation' : 'ground'}.${platform}.baseline.v1`,
        platform, config: `baseline LOD${lod}`, lod, tris: mesh.triangleCount,
        ...payload(mesh, `${platform} LOD${lod}`), budget, v: verdict(mesh.triangleCount, budget)});
    }
  }
  return {rows, builds: rows.length};
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
    const constituents = new Map();
    for (const lod of [0, 1]) {
      let lo = null, hi = null;
      for (const stage of kindStages) {
        for (const level of levels) {
          for (const status of statuses) {
            const mesh = SiteMesh.build(kind, stage.key, { level, status, lod: lod ? "far" : 0 });
            builds++;
            const at = { tris: mesh.triangleCount, config: `${stage.key}/L${level}/${status}` };
            if (!lod) collectBuildings(constituents, mesh, at.config, `site ${kind} ${at.config}`);
            if (!lo || at.tris < lo.tris) lo = at;
            if (!hi || at.tris > hi.tris) hi = { ...at, ...payload(mesh, `${kind} ${at.config}`), parts: mesh.parts.length };
          }
        }
      }
      const budget = lod ? BUDGETS.building_far : BUDGETS.scene;
      row[lod ? "far" : "near"] = { lo, hi, budget, v: verdict(hi.tris, budget), vlo: verdict(lo.tris, budget) };
    }
    row.constituents = [...constituents.values()];
    rows.push(row);
  }
  return { rows, builds, sweep: `${SiteMesh.kinds().length} kinds x ${stages} stages x ${levels.length} levels x ${statuses.length} statuses x 2 LODs` };
}

// ---------------------------------------------------------------- town blocks
// Eight ids per district. A block is seeded by its id, so one id is one sample
// of a generator that hands back a different street on the next tile.
const BLOCK_IDS = [1990, 1991, 1992, 1993, 1994, 1995, 1996, 1997, '1990'];
// Device-pixel viewports; the string id is the gallery's actual provider id.
// Use the renderer's own camera/frustum/LOD planner, including selected-lot
// framing. Geometry inventory remains reported separately below.
function measureTownScene(scene) {
  let hi = null, views = 0;
  for (const [width,height] of [[390,280],[780,560],[1280,720],[2048,1440]]) {
    for (const yaw of [0,34,90,180,270]) for (const pitch of [8,20,35,75]) for (const zoom of [0.55,1,2.5,4]) {
      for (const selected of [null,...scene.lots.map(lot=>lot.id)]) {
        const plan = Arsenal3D.scenePlan(scene,{width,height,yaw,pitch,zoom,selected});
        if (plan.budget !== BUDGETS.scene.max) throw new Error('town renderer scene ceiling drifted');
        if (!Number.isSafeInteger(plan.triangles) || plan.triangles <= 0) throw new Error('empty/invalid town draw plan');
        views++;
        if (!hi || plan.triangles > hi.tris) hi = {tris:plan.triangles,
          config:`${width}x${height}, yaw ${yaw}, pitch ${pitch}, zoom ${zoom}, ${selected || 'overview'}`,
          drawCalls:plan.drawCalls,culled:plan.culled.length,demotions:plan.demotions};
      }
    }
  }
  return {hi,views,budget:BUDGETS.scene,v:verdict(hi.tris,BUDGETS.scene)};
}
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
        if (!hi || at.tris > hi.tris) hi = { ...at, ...payload(mesh, `town ${district} ${lod}`), parts: mesh.parts.length };
      }
      row[lod] = { lo, hi, budget: BUDGETS.scene, v: verdict(hi.tris, BUDGETS.scene) };
    }
    let rendered = null;
    for (const id of BLOCK_IDS) {
      const measured = measureTownScene(TownMesh.scene({id,district}));
      if (!rendered) rendered = {...measured,hi:{...measured.hi,config:`id ${JSON.stringify(id)}, ${measured.hi.config}`}};
      else {
        rendered.views += measured.views;
        if (measured.hi.tris > rendered.hi.tris) {
          rendered.hi = {...measured.hi,config:`id ${JSON.stringify(id)}, ${measured.hi.config}`};
          rendered.v = measured.v;
        }
      }
    }
    row.rendered = rendered;
    rows.push(row);
  }
  return { rows, builds, sweep: `${TownMesh.districts().length} districts x ${BLOCK_IDS.length} ids x 2 stored LODs`,
    plannedViews:rows.reduce((n,r)=>n+r.rendered.views,0) };
}

// The kit the blocks are assembled from, at the largest footprint and the
// tallest storey count each kind admits. These are constituents of a block, not
// separately resident assets, so they are graded but never added to the
// inventory total — that would count the same triangles twice.
function measureTownBuildings() {
  const rows = []; let builds = 0;
  for (const kind of TownMesh.kinds()) {
    const info = TownMesh.kindInfo(kind);
    const width = Math.max(...info.widths), storeys = Math.max(...info.storeys);
    const row = { kind, label: info.label, width, storeys, depth: info.depth };
    const constituents = new Map();
    let assembly = false;
    for (const lod of ["close", "map"]) {
      let hi = null;
      for (const id of [0, 1, 2, 3]) {
        const mesh = TownMesh.building(kind, { id, width, storeys, lod: lod === "map" ? "map" : undefined });
        builds++;
        if (lod === 'close' && mesh.budgetUnits) {
          assembly = true;
          collectBuildings(constituents, mesh, `maximum size, seed ${id}`, `town ${kind}/${id}`);
        }
        if (!hi || mesh.triangleCount > hi.tris) hi = { tris: mesh.triangleCount, ...payload(mesh, `${kind} ${lod}`), parts: mesh.parts.length };
      }
      const budget = lod === "map" ? BUDGETS.building_far : assembly ? BUDGETS.scene : BUDGETS.building_near;
      row[lod] = { hi, budget, v: verdict(hi.tris, budget) };
    }
    row.assembly = assembly;
    row.constituents = [...constituents.values()];
    rows.push(row);
  }
  return { rows, builds, sweep: `${TownMesh.kinds().length} kinds at maximum width and storeys x 4 seeds x 2 LODs` };
}

// -------------------------------------------------------------- source weight
function measureSource() {
  const files = GENERATORS.map((f) => {
    // Git's LF text, so Windows checkout conversion cannot stale the record.
    // This is canonical source weight, not a browser HTTP transfer measurement.
    const buf = Buffer.from(fs.readFileSync(ui(f), 'utf8').replace(/\r\n/g, '\n'));
    return { file: `spheres-web/ui/${f}`, bytes: buf.length, gzip: zlib.gzipSync(buf, { level: 9 }).length };
  });
  const dir = path.join(ROOT, "spheres-web", "ui", "equipment-models");
  const glb = fs.readdirSync(dir).filter((f) => f.endsWith(".glb")).sort()
    .map((f) => gradeExport({file:f, bytes:fs.statSync(path.join(dir,f)).size}));
  if (!glb.length) throw new Error('equipment export inventory is empty; rebuild canonical models');
  return {
    files, glb,
    bytes: files.reduce((a, f) => a + f.bytes, 0),
    gzip: files.reduce((a, f) => a + f.gzip, 0),
    glbBytes: glb.reduce((a, f) => a + f.bytes, 0),
  };
}

function gradeExport(row) {
  const maxBytesExclusive = row.file.startsWith('spheres-tank-') ? 12000000
    : row.file.startsWith('spheres-ground-') ? 5000000
    : row.file.startsWith('spheres-air-') ? 28000000 : null;
  if (maxBytesExclusive === null) throw new Error(`unclassified equipment export: ${row.file}`);
  if (!Number.isSafeInteger(row.bytes) || row.bytes <= 0) throw new Error(`invalid export size: ${row.file}`);
  return {...row,maxBytesExclusive,state:row.bytes<maxBytesExclusive?'PASS':'OVER'};
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
    + `${fmt(r.map.hi.tris)} | ${fmt(r.rendered.hi.tris)} | ${r.rendered.v.text} | ${fmt(r.close.hi.bytes)} |`).join("\n");

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
    : "No kit measurement falls below its applied density range. Single buildings use the building ceiling; "
      + "terraces and campuses use the scene ceiling plus a separate check for every physical building. "
      + "The legacy park, utility and stadium kit classifications remain unchanged.";

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

  const constituentRows = [...m.sites.rows.flatMap(r=>r.constituents.map(b=>({asset:`site.${r.kind}`, ...b}))),
    ...m.buildings.rows.flatMap(r=>r.constituents.map(b=>({asset:`town.kit.${r.kind}`, ...b})))];
  const constituentTable = constituentRows.map(r=>`| \`${r.asset}/${r.id}\` | ${r.config} | ${fmt(r.ownedTriangles)} | ${fmt(r.sharedTriangles)} | ${fmt(r.tris)} | ${r.v.text} |`).join('\n');
  const legacyTable = m.legacyOver.map(r=>`| \`${r.asset}\` | ${r.config} | ${fmt(r.tris)} | ${fmt(r.budget.max)} |`).join('\n');
  const floorFailures = m.qualityUnder.length
    ? m.qualityUnder.map(r=>`- ${r.asset}: ${fmt(r.tris)} is below required ${fmt(r.budget.min)}.`).join('\n')
    : 'No required inspection quality floor is missed.';

  return `# P0 art budgets — measured

Generated by \`node tools/ui/bench_art.cjs\` — do not edit by hand. Every count in
this file is read off a mesh this tool built; nothing is quoted from a design
document, and the budgets it is graded against are re-read from roadmap section 4
on every run, so a budget edited there fails this tool rather than silently
changing a verdict.

    node tools/ui/bench_art.cjs           regenerate this file, print timings to stdout
    node tools/ui/bench_art.cjs --check   exit 1 if stale, over budget or below a required quality floor
    node tools/ui/bench_art.cjs --check-records   check reproducibility only; does not pass the budget gate

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
- **Live GPU residency.** Base-upload figures sum only position, normal and
  colour arrays. They exclude derived surface attributes, floor geometry,
  shadow targets, textures, driver alignment and overhead. CPU material-class
  tags are measured separately in P0_MEASUREMENTS.json. No offline inventory
  total describes what a browser is holding or drawing at a given moment.
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

This measures generated triangles, CPU mesh arrays, base attribute upload
payloads and source bytes. The browser probe in
\`tools/ui/art-memory-browser.cjs\` separately records actual buffer storage
requests and draw submissions; neither tool measures driver VRAM or visible pixels.

## Budgets applied

Read from roadmap section 4. The cell text is compared on every run; if section
4 changes, this tool stops instead of grading against a number that has moved.

| class | roadmap row | roadmap cell | applied as | applied to |
| --- | --- | --- | --- | --- |
| tank LOD0 | ${BUDGETS.vehicle_lod0.row} | ${BUDGETS.vehicle_lod0.cell} | ${budgetText(BUDGETS.vehicle_lod0)} tris | four tank platforms, baseline and heaviest |
| specialist LOD0 | ${BUDGETS.specialist_lod0.row} | ${BUDGETS.specialist_lod0.cell} | ${budgetText(BUDGETS.specialist_lod0)} tris | five armoured specialist platforms, baseline and heaviest |
| aircraft LOD0 | ${BUDGETS.aircraft_lod0.row} | ${BUDGETS.aircraft_lod0.cell} | ${budgetText(BUDGETS.aircraft_lod0)} tris | all three CP1 aircraft baselines |
| vehicle LOD1 | ${BUDGETS.vehicle_lod1.row} | ${BUDGETS.vehicle_lod1.cell} | ${budgetText(BUDGETS.vehicle_lod1)} tris | all twelve ground/air baselines |
| vehicle LOD2 | ${BUDGETS.vehicle_lod2.row} | ${BUDGETS.vehicle_lod2.cell} | ${budgetText(BUDGETS.vehicle_lod2)} tris | all twelve ground/air baselines |
| building near | ${BUDGETS.building_near.row} | ${BUDGETS.building_near.cell} | ${budgetText(BUDGETS.building_near)} tris | each physical building, including shared envelope cost |
| building far | ${BUDGETS.building_far.row} | ${BUDGETS.building_far.cell} | ${budgetText(BUDGETS.building_far)} tris | construction sites LOD1, town kit map |
| scene assembly | ${BUDGETS.scene.row} | ${BUDGETS.scene.cell} | ${budgetText(BUDGETS.scene)} tris | one town block, complete compound, terrace or campus |

\`OVER\` means above the ceiling and fails \`--check\`. \`under\` means below the
floor of a range. Aircraft inspection's 100,000 minimum is a required quality
floor and FAILS the gate; other lower bounds remain density notes, with their
platform-specific quality checks enforced by the equipment suite. A budget
with only a ceiling can only be \`PASS\` or \`OVER\`.

## Verdicts

${m.graded.length} graded configurations: ${gradeCount("PASS")} PASS, ${gradeCount("UNDER")} under the detail floor, ${gradeCount("OVER")} over the ceiling.

The geometry sweep covers nine ground platforms, all three CP1 aircraft,
construction sites and town assets. Contract revision 2 reconciles the original
proposal with the subsequently implemented inspection quality/export contracts
and distinguishes a physical building from a multi-building scene. The original
comparison remains visible below. No frame-rate guarantee follows from this gate.

${overSection}

${floorFailures}

## Original proposal comparison (superseded units/inspection targets)

${m.legacyOver.length} configurations still exceed the original proposal on
the CURRENT meshes. These are diagnostic comparisons, not current-contract
passes or claims that those triangles disappeared. Tank/specialist inspection
now follows the already-enforced quality and serialized export limits; aircraft
preserve the user's later 100k+ requirement. Whole compounds, terraces and
campuses also pay the scene budget, with every actual building separately graded.

| asset | configuration | triangles | original ceiling |
| --- | --- | ---: | ---: |
${legacyTable}

## Physical building accounting

Authored ranges cover each triangle exactly once. Terrain and props remain in
the total scene cost. A shared roof/wall is charged in FULL to every owning
building, rather than divided across them, and once to the assembly. Unknown
owners, duplicate ranges, gaps and missing building geometry fail measurement.
Each row records its worst observed configuration; a sum of these independent
maxima is not a simultaneously rendered scene.

| asset / physical building | worst configuration | own tris | shared tris (full) | charged tris | 12k ceiling verdict |
| --- | --- | ---: | ---: | ---: | --- |
${constituentTable}

## Ground vehicles, LOD0

\`baseline\` is \`EquipmentMesh.build({platform})\` with no components named.
\`heaviest\` is the costliest sampled specification found by greedy per-slot ascent
over all ${m.vehicles.components} components in \`equipment.rs\`,
\`equipment_specs.rs\` and \`equipment_ground.rs\`. All generator-accepted slots are
seeded, including the seven optional tank slots omitted by the default mesh.
Heavy tanks also start a search from the existing fully loaded regression fixture.
This is a measured lower bound on the true maximum, not an exhaustive proof.
The generator accepts the samples; the native compatibility matrix may refuse
some combinations as orderable designs. Baseline measurements remain separate.

| asset | configuration | LOD0 tris | budget | verdict | base upload bytes | parts |
| --- | --- | --- | --- | --- | --- | --- |
${vehicleRows}

Components that make each platform heaviest:

${m.vehicles.rows.filter((r) => r.config === "heaviest").map((r) => `- \`${r.platform}\`: ${r.note}`).join("\n")}

## Aircraft inspection and ground/air cheaper detail levels

Every row uses the current baseline specification. This proves these builds,
not the full space of component combinations. All three aircraft now have
authored LOD1 and LOD2 meshes. A baseline \`tank_heavy\` probe measures
${m.vehicles.lodProbe.map(fmt).join(" / ")} triangles across LOD0/1/2.

| platform | detail | triangles | budget | verdict | base upload bytes | CPU backing bytes |
| --- | --- | --- | --- | --- | --- | --- |
${m.details.rows.map(r => `| ${r.platform} | LOD${r.lod} | ${fmt(r.tris)} | ${budgetText(r.budget)} | ${r.v.text} | ${fmt(r.bytes)} | ${fmt(r.accounting.cpu_backing_buffer_bytes)} |`).join("\n")}

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

Swept over ${m.blocks.sweep} = ${fmt(m.blocks.builds)} raw builds and
${fmt(m.blocks.plannedViews)} camera/selection plans from the renderer itself. A block is one
${TownMesh.defaultTile[0]} x ${TownMesh.defaultTile[1]} m tile of ${TownMesh.era} temperate town,
graded against the unchanged 150k **visible** scene ceiling using the actual
draw plan. Full close geometry is preserved; its overage remains an explicit
storage diagnostic, not a claim that those triangles disappeared. The renderer
selects close/mid/map per projected lot size, culls outside the camera frustum,
and budgets the resulting draw list. A selected lot keeps close geometry.

| asset | raw close min | raw close max | raw close comparison | raw map max | max drawn | visible verdict | raw close bytes |
| --- | --- | --- | --- | --- | --- | --- | --- |
${blockRows}

| asset | most expensive sampled view | draw calls | culled lots | budget demotions |
| --- | --- | ---: | ---: | ---: |
${m.blocks.rows.map(r=>`| ${r.district} | ${r.rendered.hi.config} | ${r.rendered.hi.drawCalls} | ${r.rendered.hi.culled} | ${r.rendered.hi.demotions} |`).join('\n')}

This camera sweep uses four device-pixel viewports, five yaw angles, four pitch angles, four zoom
levels, overview and every selectable lot, across eight numeric seeds plus the
gallery's string seed. It invokes the same planner consumed by WebGL drawScene;
browser submission, context-loss and navigation tests verify that connection.
It does not measure frame rate, GPU memory, or the separate globe CityMesh path.

The map LOD has no roadmap row of its own — section 4 budgets a scene assembly
and a building, not a separate coarse scene — so both block detail levels are
graded against the existing scene ceiling. The heaviest map block is ${fmt(m.blockMapMax)} triangles, which is
${(m.blockMapMax / BUDGETS.building_far.max).toFixed(1)}x the building-far ceiling
of ${fmt(BUDGETS.building_far.max)}; that says the row is the wrong one for a whole
tile of town, not that the mesh is wrong. A coarse-scene budget is a gap in
section 4; the common scene ceiling remains enforced until a stricter coarse
scene budget is established.

## Town building kit

The pieces a block is assembled from, each at the largest width and tallest
storey count its kind admits (${m.buildings.sweep} = ${fmt(m.buildings.builds)} builds).
These are constituents of the blocks above, not separately resident assets, so
they are graded but never added into the inventory totals — that would count the
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

## Hypothetical ground/site/town inventory, not live residency

This sums one measured mesh per asset. No frame draws this set and no runtime
cache is being observed. Aircraft appear in the detail table above but are not
added here. CPU backing bytes and per-attribute layouts are in P0_MEASUREMENTS.json.

| set | assets | stored triangles | base upload payload bytes |
| --- | --- | --- | --- |
| Ground vehicles, heaviest specification | ${m.inventory.vehicles.n} | ${fmt(m.inventory.vehicles.tris)} | ${fmt(m.inventory.vehicles.bytes)} |
| Construction sites, worst case near | ${m.inventory.sites.n} | ${fmt(m.inventory.sites.tris)} | ${fmt(m.inventory.sites.bytes)} |
| Town blocks, worst case close | ${m.inventory.blocks.n} | ${fmt(m.inventory.blocks.tris)} | ${fmt(m.inventory.blocks.bytes)} |
| **Measured set, close detail** | **${m.inventory.total.n}** | **${fmt(m.inventory.total.tris)}** | **${fmt(m.inventory.total.bytes)}** (${mib(m.inventory.total.bytes)}) |
| Sites/towns coarse; ground vehicles retained at LOD0 for comparison | ${m.inventory.map.n} | ${fmt(m.inventory.map.tris)} | ${fmt(m.inventory.map.bytes)} (${mib(m.inventory.map.bytes)}) |

The comparison row deliberately retains the measured vehicles at LOD0:
${fmt(m.inventory.map.vehicleShare)} of its ${fmt(m.inventory.map.tris)} triangles are
the nine ground vehicles, with ${fmt(m.inventory.map.tris - m.inventory.map.vehicleShare)}
for the coarse sites and blocks. It is not the live map's rendering cost and
does not imply the available ground LOD1/LOD2 geometry is unused.

Payloads are summed from each actual attribute's \`byteLength\`, not inferred
from a universal bytes-per-triangle constant. Known CPU-only material tags
are validated and counted separately; unknown layouts fail with an explanation.
Shared CPU backing buffers are counted once per mesh. Independently uploaded
attributes count once per upload even if their CPU views alias one allocation.
Tank/armoured inspection additionally prepares four floats per vertex; the
browser probe records this actual upload plus the floor. Repeated shadow,
glass and highlight passes submit geometry again without storing another copy.

## Canonical source cost — not measured network traffic

The generators ship as source and build their meshes in the browser without
fetching GLB assets or requiring a build step. These figures measure the three
geometry generators normalized to Git's LF text. Checkout line endings, HTTP
compression/headers, renderer, stylesheet and other page costs are not included.

| file | bytes | |
| --- | --- | --- |
${sourceRows}
| **total** | **${fmt(m.source.bytes)}** | **${kib(m.source.bytes)}** |

${fmt(m.source.bytes)} bytes of source produce ${fmt(m.inventory.total.tris)} triangles of
geometry — ${fmt(Math.round(m.inventory.total.bytes / m.source.bytes))}x its own weight in vertex data. That ratio is not fixed
at authoring time either: it grows with every extra seed, stage, level and
district asked of the same source.

${m.source.glb.length} exported \`.glb\` files sit in \`spheres-web/ui/equipment-models/\`
totalling ${fmt(m.source.glbBytes)} bytes (${mib(m.source.glbBytes)}). They are the
portable deliverable roadmap section 4 asks for, not a runtime download — the game
never fetches them — and they are the comparison that settles the argument:
${m.source.glb.length} equipment exports (${m.source.glb.filter(f => f.file.startsWith("spheres-air-")).length} aircraft and ${m.source.glb.filter(f => !f.file.startsWith("spheres-air-")).length} ground-vehicle configurations) as binary assets weigh
${(m.source.glbBytes / m.source.bytes).toFixed(1)}x the entire generator source
that builds every vehicle, every site at every stage and every town block.

Actual committed export sizes are enforced with the existing strict byte limits.
The separate export-reproduction check also verifies the canonical file set and
contents against the generator; size alone does not prove a valid asset.

| export | bytes | must be below | verdict |
| --- | ---: | ---: | --- |
${m.source.glb.map(r=>`| \`${r.file}\` | ${fmt(r.bytes)} | ${fmt(r.maxBytesExclusive)} | ${r.state} |`).join('\n')}

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
  const details = measureDetailLevels();
  const sites = measureSites();
  const blocks = measureTownBlocks();
  const buildings = measureTownBuildings();
  const source = measureSource();

  // Everything graded in one list, so the summary and the failure list cannot
  // disagree with the tables above them.
  const graded = [];
  for (const r of vehicles.rows) graded.push({ asset: r.asset, config: r.config, tris: r.tris, budget: r.budget, ...r.v });
  for (const r of details.rows) graded.push({ asset: r.asset, config: r.config, tris: r.tris, budget: r.budget, ...r.v });
  for (const r of sites.rows) {
    graded.push({ asset: `site.${r.kind}.v1`, config: `near ${r.near.hi.config}`, tris: r.near.hi.tris, budget: r.near.budget, ...r.near.v });
    graded.push({ asset: `site.${r.kind}.v1`, config: `far ${r.far.hi.config}`, tris: r.far.hi.tris, budget: r.far.budget, ...r.far.v });
    for (const b of r.constituents) graded.push({asset:`site.${r.kind}.v1/${b.id}`, config:b.config, tris:b.tris, budget:b.budget, ...b.v});
  }
  for (const r of blocks.rows) for (const lod of ['rendered','map']) graded.push({ asset: `town.temperate.${r.district}.v1`, config: `${lod} ${r[lod].hi.config}`, tris: r[lod].hi.tris, budget: r[lod].budget, ...r[lod].v });
  for (const r of buildings.rows) {
    graded.push({ asset: `town.kit.${r.kind}`, config: "close, maximum size", tris: r.close.hi.tris, budget: r.close.budget, ...r.close.v });
    graded.push({ asset: `town.kit.${r.kind}`, config: "map, maximum size", tris: r.map.hi.tris, budget: r.map.budget, ...r.map.v });
    for (const b of r.constituents) graded.push({asset:`town.kit.${r.kind}/${b.id}`, config:b.config, tris:b.tris, budget:b.budget, ...b.v});
  }
  const over = graded.filter((g) => g.state === "OVER");
  const qualityUnder = graded.filter(g=>g.state === 'UNDER' && g.budget.requiredMin);
  const failures = graded.filter(budgetFailed);
  const exportFailures = source.glb.filter(r=>r.state==='OVER');
  const legacyGraded = graded.filter(g=>!g.asset.includes('/') && !(g.asset.startsWith('town.temperate.') && g.config.startsWith('map '))).map(g=>{
    let budget=g.budget;
    if (g.asset.startsWith('ground.') && ['baseline','heaviest'].includes(g.config)) budget=LEGACY_GROUND;
    if (g.asset.startsWith('aviation.') && g.config==='baseline LOD0') budget=LEGACY_AIRCRAFT;
    if (g.asset.startsWith('site.') && g.config.startsWith('near ')) budget=BUDGETS.building_near;
    if (g.asset.startsWith('town.kit.') && g.config.startsWith('close,')) budget=BUDGETS.building_near;
    return {...g,budget,...verdict(g.tris,budget)};
  });
  const legacyOver=[...legacyGraded.filter(g=>g.state==='OVER'),
    ...blocks.rows.filter(r=>r.close.v.state==='OVER').map(r=>({asset:`town.temperate.${r.district}.v1`,
      config:`raw close ${r.close.hi.config}`,tris:r.close.hi.tris,budget:BUDGETS.scene,...r.close.v}))];

  const heaviest = vehicles.rows.filter((r) => r.config === "heaviest");
  const near = [...heaviest, ...sites.rows.map(r => r.near.hi), ...blocks.rows.map(r => r.close.hi)];
  const coarse = [...heaviest, ...sites.rows.map(r => r.far.hi), ...blocks.rows.map(r => r.map.hi)];
  const vehicleSet = sumPayloads(heaviest);
  const siteSet = sumPayloads(sites.rows.map(r => r.near.hi));
  const blockSet = sumPayloads(blocks.rows.map(r => r.close.hi));
  const inventory = {
    vehicles: vehicleSet, sites: siteSet, blocks: blockSet,
    total: sumPayloads(near),
    map: {...sumPayloads(coarse), vehicleShare: vehicleSet.tris},
  };

  const blockMapMax = Math.max(...blocks.rows.map((r) => r.map.hi.tris));
  const measurement = {schema_version: 1, scope: 'Generated payloads and renderer camera draw plans; no live residency, frame cost or driver VRAM measurement',
    budget_contract_version:2, vehicles, details, sites, blocks, buildings, source,
    graded, over, qualityUnder, legacyOver, inventory, blockMapMax};
  const md = render(measurement);
  const outMd = path.join(ROOT, "docs", "art", "P0_BUDGETS.md");
  const outJson = path.join(ROOT, 'docs', 'art', 'P0_MEASUREMENTS.json');
  const json = JSON.stringify(measurement, null, 2) + '\n';

  if (argv.includes("--check") || argv.includes('--check-records')) {
    const stale = !fs.existsSync(outMd) || fs.readFileSync(outMd, "utf8").replace(/\r\n/g, "\n") !== md
      || !fs.existsSync(outJson) || fs.readFileSync(outJson, 'utf8') !== json;
    if (stale) console.error(`stale measurement records; re-run without --check:\n  ${outMd}\n  ${outJson}`);
    for (const o of over) {
      console.error(`over budget: ${o.asset} (${o.config}) is ${fmt(o.tris)} triangles against a ceiling of `
        + `${fmt(o.budget.max)} — over by ${fmt(o.over)} (${((o.over / o.budget.max) * 100).toFixed(1)}%)`);
    }
    for (const q of qualityUnder) console.error(`required quality floor missed: ${q.asset} (${q.config}): ${q.tris} < ${q.budget.min}`);
    for (const r of exportFailures) console.error(`export budget missed: ${r.file}: ${r.bytes} must be below ${r.maxBytesExclusive} bytes`);
    if (stale || argv.includes('--check') && (failures.length || exportFailures.length)) process.exit(1);
    console.log(`records current: ${graded.length} graded configurations; ${over.length} over budget; ${qualityUnder.length} required quality floors missed; ${exportFailures.length} export overruns. ${failures.length || exportFailures.length ? 'Budget gate remains FAIL.' : 'Budget gate PASS.'}`);
    return;
  }

  fs.mkdirSync(path.dirname(outMd), { recursive: true });
  fs.writeFileSync(outMd, md, "utf8");
  fs.writeFileSync(outJson, json, 'utf8');

  const builds = vehicles.builds + details.builds + sites.builds + blocks.builds + buildings.builds;
  console.log(`wrote docs/art/P0_BUDGETS.md — ${graded.length} graded configurations from ${fmt(builds)} builds`);
  console.log(`  triangles (hypothetical inventory, close detail): ${fmt(inventory.total.tris)}`);
  console.log(`  base attribute payload bytes:                  ${fmt(inventory.total.bytes)} (${mib(inventory.total.bytes)})`);
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
  if (qualityUnder.length) {
    console.error(`\n${qualityUnder.length} required quality floor failure(s) — \`--check\` will exit 1:`);
    for (const o of qualityUnder) console.error(`  ${o.asset} (${o.config}): ${fmt(o.tris)} below required ${fmt(o.budget.min)}`);
  }
  for (const r of exportFailures) console.error(`export budget missed: ${r.file}: ${r.bytes} must be below ${r.maxBytesExclusive} bytes`);
}

if (require.main === module) main();
module.exports = {BUDGETS, verdict, budgetFailed, verifyBudgets, gradeExport, maxedSpec, measureTownScene};
