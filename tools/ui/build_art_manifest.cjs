#!/usr/bin/env node
// The P0 art manifest, generated rather than written.
//
// A hand-kept manifest is a second source of truth that starts drifting the day
// it is committed. This one asks the generators themselves for every number it
// records — triangle counts, bounds, part names, LOD pairs — so a manifest that
// disagrees with the art is a bug in this script and not a stale document.
//
//   node tools/ui/build_art_manifest.cjs            -> docs/art/P0_MANIFEST.json + .md
//   node tools/ui/build_art_manifest.cjs --check    -> exit 1 if the committed files are stale
//
// Fields follow roadmap section 4 ("Naming and metadata"). Where a field cannot
// be filled honestly it says so rather than guessing: `basis` is "fictional" for
// original game art, and there is no `glb` for the modules that have no exporter.
"use strict";

const fs = require("fs");
const path = require("path");

const ROOT = path.resolve(__dirname, "..", "..");
const ui = (f) => path.join(ROOT, "spheres-web", "ui", f);
const EquipmentMesh = require(ui("equipment-mesh.js"));
const SiteMesh = require(ui("site-mesh.js"));
const TownMesh = require(ui("town-mesh.js"));

const CONVENTION = { units: "metres", axes: "+X right, +Y up, +Z forward", handedness: "right" };

function bounds(m) {
  const b = m.bounds;
  return {
    min: b.min.map((v) => Math.round(v * 1000) / 1000),
    max: b.max.map((v) => Math.round(v * 1000) / 1000),
    size: [0, 1, 2].map((i) => Math.round((b.max[i] - b.min[i]) * 1000) / 1000),
  };
}

// ---- ground and tactical aviation platforms, at their default specification ----
const GLB = {
  tank_standard: "spheres-tank-balanced.glb", tank_heavy: "spheres-tank-heavy.glb",
  tank_light: "spheres-tank-light.glb", tank_destroyer: "spheres-tank-destroyer.glb",
  ground_ifv: "spheres-ground-ifv.glb", ground_apc: "spheres-ground-apc.glb",
  ground_recon: "spheres-ground-recon.glb", ground_artillery: "spheres-ground-artillery.glb",
  ground_air_defense: "spheres-ground-air-defense.glb",
  air_light_attack: "spheres-air-light-attack.glb", air_tactical_strike: "spheres-air-tactical-strike.glb",
};
const vehicles = Object.keys(GLB).map((platform) => {
  const m = EquipmentMesh.build({ platform });
  const aviation = platform.startsWith("air_");
  const lod = { LOD0: m.triangleCount };
  // Aircraft currently expose inspection geometry only; do not report copies
  // at ignored LOD settings as lower-detail aircraft assets.
  if (!aviation) for (const level of [1, 2]) lod[`LOD${level}`] = EquipmentMesh.build({ platform, lod: level }).triangleCount;
  const file = path.join(ROOT, "spheres-web", "ui", "equipment-models", GLB[platform]);
  return {
    asset_id: `${aviation ? "aviation" : "ground"}.${platform}.baseline.v1`,
    display_name: m.description.split(".")[0],
    kind: "modular platform",
    game_id: platform,
    source: "spheres-web/ui/equipment-mesh.js",
    generator: "EquipmentMesh.build({platform})",
    runtime: "spheres-web/ui/equipment-model.js (WebGL2 viewer, selectable parts)",
    glb: `spheres-web/ui/equipment-models/${GLB[platform]}`,
    glb_bytes: fs.existsSync(file) ? fs.statSync(file).size : null,
    regenerate: "node tools/ui/build_equipment_models.cjs",
    lod,
    parts: m.parts.length,
    slots: Object.keys(m.specification.components).sort(),
    materials: 1,
    pivot: aviation ? "airframe centreline, parked landing-gear contact at Y=0" : "hull centreline, ground contact at Y=0",
    bounds: bounds(m),
    basis: "fictional — original game art, not a named real vehicle",
    era: ["1990-baseline"],
    validation: "tools/ui/check_equipment_mesh.cjs, check_equipment_model.cjs, check_equipment_export.cjs",
    milestone: "P0",
    ...CONVENTION,
  };
});

// ---- construction sites: thirteen kinds, five stages, two LODs ----
const sites = SiteMesh.kinds().map((kind) => {
  const meta = SiteMesh.meta(kind);
  const stages = SiteMesh.stages(kind);
  const near = stages.map((s) => SiteMesh.build(kind, s.key, { level: 1 }));
  const far = stages.map((s) => SiteMesh.build(kind, s.key, { level: 1, lod: "far" }));
  return {
    asset_id: `site.${kind}.v1`,
    display_name: meta.name || kind,
    kind: "facility assembly (staged)",
    game_id: kind,
    game_id_source: "spheres-sim/src/production.rs PROJECT_KINDS",
    source: "spheres-web/ui/site-mesh.js",
    generator: "SiteMesh.build(kind, stageOrProgress, {level, status, lod})",
    runtime: "spheres-web/ui/arsenal3d.js via the 'site' provider; card strip in productionCardHtml",
    glb: null,
    regenerate: "none — built in the browser from source",
    stages: stages.map((s) => ({ key: s.key, name: s.name, from: s.from, to: s.to })),
    lod: {
      LOD0: { min: Math.min(...near.map((m) => m.triangleCount)), max: Math.max(...near.map((m) => m.triangleCount)) },
      LOD1: { min: Math.min(...far.map((m) => m.triangleCount)), max: Math.max(...far.map((m) => m.triangleCount)) },
    },
    parts: near[near.length - 1].parts.length,
    materials: "vertex colour only",
    pivot: "footprint centre at grade, Y=0",
    bounds: bounds(near[near.length - 1]),
    basis: "fictional — representative industrial scenery, no sourced building",
    placeholder: !!meta.placeholder,
    era: ["1990-baseline"],
    validation: "tools/ui/check_site_mesh.cjs",
    milestone: meta.placeholder ? "P0 (placeholder massing, art pass pending in P2)" : "P0",
    ...CONVENTION,
  };
});

// ---- the town block ----
const townBlocks = TownMesh.districts().map((district) => {
  const near = TownMesh.block({ id: 1990, district });
  const far = TownMesh.block({ id: 1990, district, lod: "map" });
  return {
    asset_id: `town.temperate.${district}.v1`,
    display_name: `Temperate town block — ${district}`,
    kind: "scene assembly",
    game_id: "visual_only",
    source: "spheres-web/ui/town-mesh.js",
    generator: "TownMesh.block({id, district, lod})",
    runtime: "spheres-web/ui/arsenal3d.js via the 'town' provider",
    glb: null,
    lod: { LOD0: near.triangleCount, LOD1: far.triangleCount },
    parts: near.parts.length,
    lots: near.lots ? near.lots.length : null,
    materials: "vertex colour only",
    pivot: "tile centre at grade, Y=0",
    bounds: bounds(near),
    basis: "representative — plausible 1990 temperate massing, not a reconstruction of any real town",
    era: ["1990-baseline"],
    validation: "tools/ui/check_town_mesh.cjs",
    milestone: "P0",
    ...CONVENTION,
  };
});

const manifest = {
  milestone: "P0",
  convention: CONVENTION,
  pipeline: {
    authoritative_source: "the deterministic JavaScript generators in spheres-web/ui/",
    portable_export: "glTF 2.0 .glb via spheres-web/ui/equipment-export.js (equipment platforms only)",
    reverse_leg: "spheres-web/ui/equipment-import.js — GLB back into the runtime mesh shape",
    runtime_loader: "runtime geometry is generated locally; no GLB asset fetch or build step is required",
  },
  counts: { vehicles: vehicles.length, sites: sites.length, town_blocks: townBlocks.length },
  vehicles, sites, town_blocks: townBlocks,
};

const outJson = path.join(ROOT, "docs", "art", "P0_MANIFEST.json");
const json = `${JSON.stringify(manifest, null, 2)}\n`;

const rows = [
  ...vehicles.map((a) => [a.asset_id, a.game_id, String(a.lod.LOD0), a.lod.LOD1 ?? "—", a.lod.LOD2 ?? "—", `${a.bounds.size.join(" x ")} m`]),
  ...sites.map((a) => [a.asset_id, a.game_id, `${a.lod.LOD0.min}-${a.lod.LOD0.max}`,
    `${a.lod.LOD1.min}-${a.lod.LOD1.max}`, "—", `${a.bounds.size.join(" x ")} m`]),
  ...townBlocks.map((a) => [a.asset_id, a.game_id, String(a.lod.LOD0), String(a.lod.LOD1), "—", `${a.bounds.size.join(" x ")} m`]),
];
const placeholders = sites.filter((s) => s.placeholder);
const md = `# P0 art manifest

Generated by \`node tools/ui/build_art_manifest.cjs\` — do not edit by hand. Every
number here is read from the generator, so this file and the art cannot disagree.

Convention: ${CONVENTION.units}, ${CONVENTION.axes}, ${CONVENTION.handedness}-handed.
Authoritative source: ${manifest.pipeline.authoritative_source}.
Runtime loader: ${manifest.pipeline.runtime_loader}.

| asset | game id | LOD0 tris | LOD1 tris | LOD2 tris | size |
| --- | --- | --- | --- | --- | --- |
${rows.map((r) => `| \`${r[0]}\` | \`${r[1]}\` | ${r[2]} | ${r[3]} | ${r[4]} | ${r[5]} |`).join("\n")}

${placeholders.length ? `${placeholders.length} of the ${sites.length} construction kinds remain placeholder massing: ${placeholders.map(s => `\`${s.game_id}\``).join(", ")}.` : `All ${sites.length} construction kinds have authored compositions; none is marked as placeholder massing.`}
Placeholder status comes from each generator's metadata and is also retained in the JSON.
Ground equipment has three authored detail levels. Tactical aircraft currently
have inspection geometry only. A missing level is shown as a dash, not as a
duplicate lower-detail asset. These counts do not certify runtime frame rates;
measured budget limits and remaining overruns are recorded in P0_BUDGETS.md.
`;
const outMd = path.join(ROOT, "docs", "art", "P0_MANIFEST.md");

if (process.argv.includes("--check")) {
  const stale = [];
  if (!fs.existsSync(outJson) || fs.readFileSync(outJson, "utf8") !== json) stale.push(outJson);
  if (!fs.existsSync(outMd) || fs.readFileSync(outMd, "utf8").replace(/\r\n/g, "\n") !== md) stale.push(outMd);
  if (stale.length) {
    console.error(`stale, re-run without --check:\n  ${stale.join("\n  ")}`);
    process.exit(1);
  }
  console.log(`manifest current: ${rows.length} assets`);
} else {
  fs.mkdirSync(path.dirname(outJson), { recursive: true });
  fs.writeFileSync(outJson, json, "utf8");
  fs.writeFileSync(outMd, md, "utf8");
  console.log(`wrote ${rows.length} assets -> docs/art/P0_MANIFEST.json + .md`);
}
