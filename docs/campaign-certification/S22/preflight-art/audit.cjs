'use strict';
const fs=require('node:fs'),path=require('node:path'),cp=require('node:child_process'),crypto=require('node:crypto');
const root=path.resolve(__dirname,'../../integration');
const sha=b=>crypto.createHash('sha256').update(b).digest('hex');
const git=args=>cp.execFileSync('git',args,{cwd:root,encoding:'utf8',windowsHide:true}).trim();
const files=[
 'docs/art/3D_MODEL_MASTER_ROADMAP.md','docs/art/P0_BUDGETS.md','docs/art/P0_MEASUREMENTS.json',
 'docs/art/P0_MANIFEST.md','docs/art/P0_MANIFEST.json','docs/art/TOWN_SCENE_RENDERING.md',
 'tools/ui/bench_art.cjs','tools/ui/build_art_manifest.cjs','tools/ui/build_equipment_models.cjs',
 'tools/ui/mesh-accounting.cjs','tools/ui/art-budget-units.cjs','tools/ui/check_art_budget_contract.cjs',
 'tools/ui/check_mesh_accounting.cjs','tools/ui/check_art_budget_units.cjs','tools/ui/check_art_worst_spec.cjs',
 'tools/ui/check_town_scene_budget.cjs','tools/ui/check_equipment_export.cjs','tools/ui/check_equipment_mesh.cjs',
 'tools/ui/check_s16_fighter_mesh.cjs','tools/ui/check_art_gallery.cjs',
 'spheres-web/ui/equipment-mesh.js','spheres-web/ui/equipment-export.js','spheres-web/ui/site-mesh.js',
 'spheres-web/ui/town-mesh.js','spheres-web/ui/arsenal3d.js',
 ...fs.readdirSync(path.join(root,'spheres-web/ui/equipment-models')).filter(f=>f.endsWith('.glb')||f==='README.md').map(f=>'spheres-web/ui/equipment-models/'+f)
];
const snapshot=()=>files.map(file=>{const bytes=fs.readFileSync(path.join(root,file));return{file,bytes:bytes.length,sha256:sha(bytes)};});
const result={schema:'spheres-art-preflight/v1',started_utc:new Date().toISOString(),revision:git(['rev-parse','HEAD']),
 node:process.version,node_path:process.execPath,scope:'Offline generated geometry, payload accounting, byte reproduction and contract limits. No browser, FPS, driver VRAM or campaign timing measurement.',
 status_before:git(['status','--porcelain']),source_before:snapshot(),commands:[],passed:false};
fs.writeFileSync(path.join(__dirname,'audit-start.json'),JSON.stringify(result,null,2)+'\n');
const jobs=[
 ['budget-and-records',['tools/ui/bench_art.cjs','--check']],
 ['manifest',['tools/ui/build_art_manifest.cjs','--check']],
 ['canonical-exports',['tools/ui/build_equipment_models.cjs','--check']],
 ['focused-contract-accounting-quality',['--test','--test-concurrency=1',
  'tools/ui/check_art_budget_contract.cjs','tools/ui/check_mesh_accounting.cjs','tools/ui/check_art_budget_units.cjs',
  'tools/ui/check_art_worst_spec.cjs','tools/ui/check_town_scene_budget.cjs','tools/ui/check_equipment_export.cjs',
  'tools/ui/check_equipment_mesh.cjs','tools/ui/check_s16_fighter_mesh.cjs','tools/ui/check_art_gallery.cjs']]
];
for(const [name,args] of jobs){
 const log=name+'.log',fd=fs.openSync(path.join(__dirname,log),'w'),started=new Date().toISOString();
 console.log('START '+name+' '+started);
 const run=cp.spawnSync(process.execPath,args,{cwd:root,windowsHide:true,stdio:['ignore',fd,fd],timeout:600000});
 fs.closeSync(fd);const raw=fs.readFileSync(path.join(__dirname,log));
 const entry={name,executable:process.execPath,args,started_utc:started,finished_utc:new Date().toISOString(),exit_code:run.status,signal:run.signal,error:run.error?.message||null,log,log_bytes:raw.length,log_sha256:sha(raw)};
 result.commands.push(entry);fs.writeFileSync(path.join(__dirname,'commands.json'),JSON.stringify(result.commands,null,2)+'\n');
 console.log('DONE '+name+' exit='+run.status);
}
result.source_after=snapshot();result.source_unchanged=JSON.stringify(result.source_before)===JSON.stringify(result.source_after);
result.finished_utc=new Date().toISOString();result.head_after=git(['rev-parse','HEAD']);result.status_after=git(['status','--porcelain']);
result.passed=result.source_unchanged&&result.commands.every(r=>r.exit_code===0&&!r.error);
fs.writeFileSync(path.join(__dirname,'result.json'),JSON.stringify(result,null,2)+'\n');
console.log(result.passed?'PASS: offline art contract, accounting and reproduction':'FAIL: inspect result.json and command logs');
if(!result.passed)process.exitCode=1;
