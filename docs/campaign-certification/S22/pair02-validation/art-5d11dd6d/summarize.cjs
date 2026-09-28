'use strict';
const fs=require('node:fs'),path=require('node:path'),assert=require('node:assert/strict');
const root=path.resolve(__dirname,'../../integration');
const m=JSON.parse(fs.readFileSync(path.join(root,'docs/art/P0_MEASUREMENTS.json'),'utf8'));
const {BUDGETS,verifyBudgets}=require(path.join(root,'tools/ui/bench_art.cjs'));verifyBudgets();
const expected={vehicle_lod0:[20000,150000,false],specialist_lod0:[8000,48000,false],aircraft_lod0:[100000,250000,true],
 vehicle_lod1:[4000,12000,false],vehicle_lod2:[300,1500,false],building_near:[2000,12000,false],building_far:[100,800,false],scene:[null,150000,false]};
const bounds=Object.fromEntries(Object.entries(BUDGETS).map(([k,b])=>[k,[b.min,b.max,b.requiredMin===true]]));
assert.deepEqual(bounds,expected,'Current recorded contract bounds must remain exact');
const expectedFiles=['balanced','mobile','heavy','light','destroyer'].map(id=>'spheres-tank-'+id+'.glb')
 .concat(['ifv','apc','recon','artillery','air-defense'].map(id=>'spheres-ground-'+id+'.glb'))
 .concat(['fighter','light-attack','tactical-strike'].map(id=>'spheres-air-'+id+'.glb')).sort();
const actualFiles=fs.readdirSync(path.join(root,'spheres-web/ui/equipment-models')).filter(f=>f.endsWith('.glb')).sort();
assert.deepEqual(actualFiles,expectedFiles,'Canonical file set must have no extra or missing GLBs');
const facts={contract_version:m.budget_contract_version,contract_bounds:bounds,canonical_file_set:actualFiles,
 graded:m.graded.length,pass:m.graded.filter(r=>r.state==='PASS').length,advisory_density_notes:m.graded.filter(r=>r.state==='UNDER'&&!r.budget.requiredMin).length,
 actual_current_ceiling_failures:m.over,actual_required_floor_failures:m.qualityUnder,actual_export_failures:m.source.glb.filter(r=>r.state==='OVER'),
 superseded_proposal_overages:m.legacyOver,
 raw_town_blocks:m.blocks.rows.map(r=>({district:r.district,full_close_triangles:r.close.hi.tris,actual_draw_plan_triangles:r.rendered.hi.tris,scene_ceiling:r.rendered.budget.max,headroom:r.rendered.budget.max-r.rendered.hi.tris})),
 aircraft_inspection:m.details.rows.filter(r=>r.asset.startsWith('aviation.')&&r.config==='baseline LOD0').map(r=>({asset:r.asset,triangles:r.tris,base_attribute_bytes:r.bytes,cpu_backing_bytes:r.accounting.cpu_backing_buffer_bytes})),
 hypothetical_inventory:m.inventory,
 boundaries:[
  'The 22 September contract-v2 reconciliation predates this audit; its source patch is retained. No current or legacy limits changed in this audit.',
  '33 original-proposal overages remain explicitly retained. Two complete raw town blocks exceed150000; current gallery camera submissions fit the unchanged ceiling.',
  'The gallery adaptive TownMesh planner is not the campaign globe CityMesh path. This offline pass does not qualify the campaign city renderer.',
  'CPU mesh backing and base attributes exclude derived renderer surface attributes, floor/shadow/texture resources, driver overhead and application heap. These are explicit scope limits, not live-residency figures.',
  'The hypothetical inventory excludes aircraft; aircraft detail payloads are separately recorded. No current frame draws the inventory total.',
  'Baseline aircraft and all twelve baseline detail levels are measured. Greedy ground-component sweeps are not exhaustive proofs of every possible component combination.',
  'No frame rates, cold-load/control/tick performance or driver VRAM are measured by these checks.'
 ]};
fs.writeFileSync(path.join(__dirname,'findings.json'),JSON.stringify(facts,null,2)+'\n');
console.log('PASS: exact unchanged contract bounds and13-file canonical GLB set; findings.json records current/legacy verdicts and scope');
