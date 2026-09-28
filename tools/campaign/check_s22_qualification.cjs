'use strict';
// Synthetic verifier fixtures exercise rejection semantics only. They are not
// game saves, performance evidence, or accepted qualification measurements.
const test=require('node:test'),assert=require('node:assert/strict'),fs=require('node:fs'),path=require('node:path'),os=require('node:os'),cp=require('node:child_process');
const q=require('./s22-qualification.cjs');
const temporary=()=>fs.mkdtempSync(path.join(os.tmpdir(),'s22-verifier-unit-'));
const write=(file,value)=>{fs.mkdirSync(path.dirname(file),{recursive:true});fs.writeFileSync(file,typeof value==='string'?value:JSON.stringify(value));};
const read=file=>JSON.parse(fs.readFileSync(file,'utf8'));
const mutate=(file,fn)=>{const value=read(file);fn(value);write(file,value);};
function cells(root){let i=0;return q.ROUNDS.flatMap(round=>q.CASES.flatMap(c=>q.ROLES.map(role=>({id:'cell-'+(++i),round,case:c.id,role,output_path:path.join(root,'output-'+i)}))));}
function calendar(date){return date.split('-').map(Number);}
function dated(date,days){const d=new Date(date+'T00:00:00Z');d.setUTCDate(d.getUTCDate()+days);return d.toISOString().slice(0,10);}
function facts(date){return {calendar:calendar(date),player:'France',world_fnv64:'1234567812345678',
  rules:Object.fromEntries(['daily_simulation','ideology_blocs','historical_party_leadership','resource_market','logistics_routes','physical_logistics','military_operations','production_system','manufacturing_system','industry_rebuild','fiscal_recovery','economic_competition'].map(k=>[k,true]).concat([['operational_warfare',1]])),
  capabilities:{campaign_initialized:true,party_leadership:true,population:true,sector_contractors:true,supplier_operations_version:1},
  autonomous_policies:{economic_competition:true,economic_active:true,supplier_catalogue_enabled:true,supplier_catalogue_active:true,military_enabled:true,military_active:true,economic_reviews:1,supplier_reviewed_plans:1,military_reviews:1}};}
function nativeFixture(){
  const root=temporary(),output=path.join(root,'native'),input=path.join(root,'checkpoint.json'),binary=path.join(root,'native.exe'),wrapper=path.join(root,'runner.ps1');
  write(input,{unit_fixture:true});write(binary,'fake unit-test binary');write(wrapper,'fake unit-test wrapper');fs.mkdirSync(output);
  fs.copyFileSync(input,path.join(output,'input.json'));
  const manifest={candidate_revision:'1'.repeat(40),frozen_utc:'2026-01-01T00:00:00Z',binaries:{native_test:q.fileRecord(binary)},files:{'tools/campaign/run-s22-profile.ps1':q.fileRecord(wrapper)}};
  const fixture={...q.CASES[0],input:q.fileRecord(input)},cell={output_path:output};
  const samples=Array.from({length:31},(_,i)=>({index:i,date:dated(fixture.date,i+1),facts:facts(dated(fixture.date,i+1)),simulation_history_ms:100+i,state_read_model_ms:20,state_serialization_ms:2,history_delta_serialization_ms:1,whole_turn_ms:150+i,state_bytes:20,history_delta_bytes:5}));
  const control_commands=samples.map((_,index)=>({index,commands:[],event_pause:null}));
  const summaries=Object.fromEntries(['simulation_history_ms','state_read_model_ms','state_serialization_ms','history_delta_serialization_ms','whole_turn_ms'].map(k=>[k,q.summary(samples.map(s=>s[k]))]));
  const profile={format:'spheres-s22-profile/v1',revision:manifest.candidate_revision.slice(0,12),mode:'measure_input',passed:true,failure:null,source_unchanged:true,certified_profile_required:true,renew_existing_budget:true,input:path.join(output,'input.json'),input_bytes:fixture.input.bytes,input_fnv64:'1234567812345678',starting:facts(fixture.date),final:samples.at(-1).facts,samples,control_commands,summaries,batch:{ordinary_days:31,same_final_facts:true,final:samples.at(-1).facts,elapsed_ms:3100,days_per_second:10,control_commands:[]}};
  const runner={format:'spheres-s22-runner/v1',mode:'measure',passed:true,failures:[],numerical_acceptance:true,exit_code:0,source_unchanged:true,candidate_revision:manifest.candidate_revision,test_binary:binary,test_binary_sha256:manifest.binaries.native_test.sha256,wrapper_sha256:manifest.files['tools/campaign/run-s22-profile.ps1'].sha256,original_input:input,evidence_root:output,copied_input:path.join(output,'input.json'),profile_json:path.join(output,'profile.json'),original_input_sha256_before:fixture.input.sha256,original_input_sha256_after:fixture.input.sha256,copied_input_sha256_before:fixture.input.sha256,copied_input_sha256_after:fixture.input.sha256,arguments:['performance::s22_measure_checkpoint','--ignored','--exact','--nocapture','--test-threads=1'],child_environment:{SPHERES_S22_INPUT:path.join(output,'input.json'),SPHERES_S22_OUT:path.join(output,'profile.json'),SPHERES_S22_RENEW_BUDGET:'1',SPHERES_S22_REQUIRE_CERTIFIED:'1'},latency:{simulation_history:summaries.simulation_history_ms,whole_turn:summaries.whole_turn_ms,batch:profile.batch},memory:{samples:2,nominal_interval_ms:100,failed_samples:0,max_sampled_private_bytes:200,max_sampled_working_set_bytes:250,os_peak_working_set_bytes:300,max_observed_sample_gap_ms:100},frozen_limits:{simulation_history_p95_ms:300,whole_turn_p95_ms:400,whole_turn_max_ms:750,sampled_private_bytes:1073741824,observed_os_peak_working_set_bytes:1073741824},started_utc:'2026-01-01T00:00:01Z',finished_utc:'2026-01-01T00:00:20Z'};
  runner.wall_seconds=20;write(path.join(output,'runner-result.json'),runner);write(path.join(output,'profile.json'),profile);
  write(path.join(output,'memory-samples.csv'),'elapsed_ms,private_bytes,working_set_bytes,os_peak_working_set_bytes\n0,100,150,200\n100,200,250,300\n');
  return {root,output,manifest,fixture,cell,run:()=>q.nativeCell(manifest,cell,fixture)};
}
function controlRows(){let camera={yaw:0,pitch:.5,zoom:8};return Array.from({length:31},(_,i)=>{const key=q.CONTROLS[i%4],before={...camera};if(key==='west')camera.yaw-=.1;else if(key==='east')camera.yaw+=.1;else if(key==='zoom-in')camera.zoom*=2;else camera.zoom/=2;return {key,trusted:true,event_timestamp:i*1000,start:i*1000+1,complete:i*1000+101,elapsed_ms:101,before_globe:before,after_globe:{...camera}};});}
function mapPhase(duration,fps){const count=duration*fps/1000;return {elapsed_ms:duration,frames:Array.from({length:count},(_,i)=>({start_ms:i*duration/count,complete_ms:(i+1)*duration/count,interval_ms:duration/count,draw_ms:duration/count,draw_calls:1,city_draw_calls:0,triangles:1}))};}
function mapFixture(){
  const texture={probe_errors:0,unmeasured_allocation_events:0,unmeasured_texture_allocations:0,declared_texture_texel_payload_bytes:1,peak_declared_texture_texel_payload_bytes:1};
  const buffers={resident_buffer_payload_bytes:1,peak_buffer_payload_bytes:1,live_buffers:1,buffer_data_calls:1,buffer_data_payload_bytes:1,draw_calls:1,submitted_triangles:1,offscreen_submitted_triangles:0,context_losses:0};
  const memory={metrics:{metrics:[{name:'JSHeapUsedSize',value:1},{name:'JSHeapTotalSize',value:2}]},dom:{documents:1,nodes:1,jsEventListeners:1},contexts:[{buffer_payload:buffers,declared_texture_payload:texture,texture_diagnostics:[]}]};
  return {memory_before:memory,memory_after:memory,cells:['standard','low'].map(detail=>({detail,viewport:q.WORKLOAD.viewport,dpr:1,memory,inputs:controlRows(),input_passed:true,input_workload_valid:true,input_summary:{count:31,p95:101,max:101},
    views:q.VIEWS.map((view,i)=>{const measured=mapPhase(12000,30);if(detail==='standard'&&i===3)measured.frames[0].city_draw_calls=1;
      return {view,map_mode:'terrain',map_details:q.WORKLOAD.presets[detail],active_canvas:q.WORKLOAD.map_canvas,ready:true,workload_valid:true,passed:true,buffers,declared_texture_payload:texture,settle:mapPhase(3000,30),measured,fps:30,terrain:{ready:true}};})}))};
}

test('frozen matrix requires every unique round/case/role and disjoint output root',()=>{
  const rows=cells(temporary());q.validateCells(rows);
  assert.throws(()=>q.validateCells(rows.slice(1)),/18 cells/);
  const wrong=structuredClone(rows);wrong[0].case='mid-france-2006';assert.throws(()=>q.validateCells(wrong),/Unknown/);
  const duplicate=structuredClone(rows);duplicate[1]={...duplicate[0],id:'distinct'};assert.throws(()=>q.validateCells(duplicate),/Duplicate cell tuple/);
  const overlap=structuredClone(rows);overlap[1].output_path=path.join(overlap[0].output_path,'child');assert.throws(()=>q.validateCells(overlap),/overlap/);
});
test('31-day native acceptance recomputes raw percentiles, batch equivalence and memory',()=>{
  const f=nativeFixture();const result=f.run();assert.equal(result.whole_turn.p95_ms,179);assert.equal(result.memory.max_sampled_private_bytes,200);
  mutate(path.join(f.output,'profile.json'),p=>{p.samples[0].whole_turn_ms=750.00000001;p.summaries.whole_turn_ms=q.summary(p.samples.map(s=>s.whole_turn_ms));});
  mutate(path.join(f.output,'runner-result.json'),r=>{r.latency.whole_turn=read(path.join(f.output,'profile.json')).summaries.whole_turn_ms;});
  assert.throws(f.run,/maximum|exceeds 750/);
});
test('native refuses insufficient samples, date drift, missing capability and unequal batch',()=>{
  for(const [change,pattern] of [
    [p=>p.samples.pop(),/31/],
    [p=>p.starting.calendar=[2006,1,1],/2006-01-01|1999-05-13/],
    [p=>p.samples[15].facts.calendar=[2006,1,1],/Consecutive/],
    [p=>p.samples[30].facts.autonomous_policies.military_reviews=0,/Executed/],
    [p=>p.batch.final={...p.final,world_fnv64:'f'.repeat(16)},/batch final/],
    [p=>p.certified_profile_required=false,/false/],
  ]) {const f=nativeFixture();mutate(path.join(f.output,'profile.json'),change);assert.throws(f.run,pattern);}
});
test('native refuses false binary identity, altered copied input, understated memory and contradictory pass',()=>{
  for(const change of [r=>{r.test_binary_sha256='0'.repeat(64);},r=>{r.memory.max_sampled_private_bytes=199;},r=>{r.failures=['timing failed'];},r=>{r.numerical_acceptance=false;}]){
    const f=nativeFixture();mutate(path.join(f.output,'runner-result.json'),change);assert.throws(f.run);
  }
  const f=nativeFixture();write(path.join(f.output,'input.json'),'changed');assert.throws(f.run,/copied input changed/);
});
test('failed final memory observation cannot hide a higher observed OS peak',()=>{
  const f=nativeFixture();mutate(path.join(f.output,'runner-result.json'),r=>{r.memory.failed_samples=1;r.memory.os_peak_working_set_bytes=301;});assert.equal(f.run().memory.os_peak_working_set_bytes,301);
  mutate(path.join(f.output,'runner-result.json'),r=>{r.memory.os_peak_working_set_bytes=1073741825;});assert.throws(f.run,/1 GiB/);
});
test('native rejects damaged empty CSV, understated gaps and missing sequential stages',()=>{
  const blank=nativeFixture();write(path.join(blank.output,'memory-samples.csv'),'elapsed_ms,private_bytes,working_set_bytes,os_peak_working_set_bytes\n0,,150,200\n100,200,250,300\n');assert.throws(blank.run,/nonempty numeric/);
  const gap=nativeFixture();mutate(path.join(gap.output,'runner-result.json'),r=>r.memory.max_observed_sample_gap_ms=1);assert.throws(gap.run,/gap contradicts/);
  const interval=nativeFixture();mutate(path.join(interval.output,'runner-result.json'),r=>r.wall_seconds=.01);assert.throws(interval.run,/exceeds runner interval/);
  const stage=nativeFixture();mutate(path.join(stage.output,'profile.json'),p=>p.samples[0].state_read_model_ms=100);assert.throws(stage.run,/sequential timed stages/);
});
test('camera controls require all 31 exact trusted actions, actual direction and unrounded p95',()=>{
  assert.equal(q.inputs(controlRows()).p95_ms,101);
  const swapped=controlRows();[swapped[0],swapped[1]]=[swapped[1],swapped[0]];assert.throws(()=>q.inputs(swapped),/sequence/);
  const untrusted=controlRows();untrusted[3].trusted=false;assert.throws(()=>q.inputs(untrusted));
  const unchanged=controlRows();unchanged[2].after_globe=unchanged[2].before_globe;assert.throws(()=>q.inputs(unchanged),/zoom/);
  assert.throws(()=>q.inputs(controlRows().slice(1)),/31/);
  const slow=controlRows();for(const i of [29,30]){slow[i].elapsed_ms=200.000001;slow[i].complete=slow[i].event_timestamp+slow[i].elapsed_ms;}assert.throws(()=>q.inputs(slow),/200 ms/);
});
test('zoom round-trip precision requires unchanged raw map centre and rejects real drift',()=>{
  const rows=controlRows(),r=rows[2],centre={cx:1213.7113132067784,cy:217.6780759386204};
  r.before_camera={...centre};r.after_camera={...centre};
  for(let i=2;i<rows.length;i++){
    rows[i].after_globe.pitch+=2e-8;
    if(i>2)rows[i].before_globe.pitch+=2e-8;
  }
  assert.doesNotThrow(()=>q.inputs(rows));
  const moved=structuredClone(rows);moved[2].after_camera.cx+=.001;assert.throws(()=>q.inputs(moved));
  const absent=structuredClone(rows);delete absent[2].before_camera;assert.throws(()=>q.inputs(absent),/centre/);
  const drift=structuredClone(rows);drift[2].after_globe.pitch+=1e-6;assert.throws(()=>q.inputs(drift),/precision/);
});
test('draw validator counts actual completed frames using the full measured interval',()=>{
  assert.equal(q.mapPhase(mapPhase(12000,30),12000),30);
  const invalid=mapPhase(12000,30);invalid.frames[0].draw_calls=0;assert.throws(()=>q.mapPhase(invalid,12000),/draw calls/);
  const omitted=mapPhase(12000,30);omitted.frames.pop();assert.throws(()=>q.mapPhase(omitted,12000),/final completed/);
});
test('complete map matrix independently enforces all eight views, full preset, drawable and memory',()=>{
  const good=mapFixture();assert.equal(q.validateMap(good).views.length,8);
  const preset=structuredClone(good);preset.cells[0].views[0].map_details.features=false;assert.throws(()=>q.validateMap(preset));
  const tiny=structuredClone(good);tiny.cells[0].views[0].active_canvas.height=10;assert.throws(()=>q.validateMap(tiny),/drawable/);
  const missing=structuredClone(good);missing.cells[1].views.pop();assert.throws(()=>q.validateMap(missing));
  const hidden=structuredClone(good);hidden.cells[0].views[3].measured.frames[0].city_draw_calls=0;assert.throws(()=>q.validateMap(hidden),/actually draw/);
  const noMemory=structuredClone(good);delete noMemory.memory_before.metrics;assert.throws(()=>q.validateMap(noMemory),/CDP metrics/);
});
function orbitPhase(duration,count){return {requested_ms:duration,elapsed_ms:duration,draw_frames:count,animation_callbacks:count,
  completed_draw_fps:count*1000/duration,raw_frames:Array.from({length:count},(_,i)=>({started_ms:i*duration/count,completed_ms:(i+1)*duration/count,callback_and_gpu_ms:duration/count,gpu_finish_wait_ms:0,draw_calls:1,submitted_triangles:100001})),
  raw_animation_ticks:Array.from({length:count},(_,i)=>({elapsed_ms:(i+1)*duration/count,completed_draw_frames:i+1}))};}
test('actual viewer orbit accepts 60, rejects 59.x and rejects out-of-order clocks/counters',()=>{
  const orbit={platform:'air_fighter',viewport:{width:1920,height:1080,dpr:1},target_fps:60,passed:true,warmup:orbitPhase(3000,180),sample:orbitPhase(12000,720)};
  assert.equal(q.validateOrbit(orbit),60);orbit.sample=orbitPhase(12000,719);assert.throws(()=>q.validateOrbit(orbit),/below 60/);
  const backwards=orbitPhase(12000,720);backwards.raw_animation_ticks[2].elapsed_ms=0;assert.throws(()=>q.rendererPhase(backwards,12000),/Animation clock/);
  const counter=orbitPhase(12000,720);counter.raw_animation_ticks[2].completed_draw_frames=0;assert.throws(()=>q.rendererPhase(counter,12000),/Completed draw counter/);
});
test('passed flag cannot hide a trace error, empty trace or hash mismatch',()=>{
  const root=temporary(),file=path.join(root,'chrome-trace.json.gz');fs.writeFileSync(file,require('node:zlib').gzipSync(Buffer.from('{"traceEvents":[]}')));
  const row={passed:true,trace:{file:'chrome-trace.json.gz',raw_bytes:18,compressed_bytes:fs.statSync(file).size,sha256:q.fileHash(file)}};q.validateTrace(row,root);
  row.trace_failure='capture failed';assert.throws(()=>q.validateTrace(row,root),/Trace failure/);delete row.trace_failure;
  row.trace.raw_bytes=0;assert.throws(()=>q.validateTrace(row,root),/Raw trace bytes/);row.trace.raw_bytes=18;
  row.trace.sha256='a'.repeat(64);assert.throws(()=>q.validateTrace(row,root));
});
test('browser memory observations are required separately without applying native memory ceiling',()=>{
  const row={captured_utc:'2026-01-01Z',scope:'Observed OS counters, not VRAM',browser_owned_processes:[{id:42}],windows_counters:[{Id:42,PrivateMemorySize64:2*1073741824,WorkingSet64:100,PeakWorkingSet64:110}]};q.processMemory(row);
  assert.throws(()=>q.processMemory({}),/capture/);delete row.windows_counters[0].PrivateMemorySize64;assert.throws(()=>q.processMemory(row),/PrivateMemorySize64/);
});
test('browser roots never select one result from several attempts',()=>{
  const root=temporary();fs.mkdirSync(path.join(root,'browser-first'));assert.equal(q.browserChild(root),path.join(root,'browser-first'));
  fs.mkdirSync(path.join(root,'browser-failed'));assert.throws(()=>q.browserChild(root),/exactly one/);
});
test('actual save date is read from the nested world, including default day one',()=>{
  const file=path.join(temporary(),'save.json');write(file,{format:'spheres-campaign',world:{format:'spheres-integrated-save',world:{year:2035,month:11,day:30}}});assert.equal(q.inputDate(file),'2035-11-30');
  mutate(file,p=>{delete p.world.world.day;p.world.world.month=12;});assert.equal(q.inputDate(file),'2035-12-01');
});

// Small committed repository lets the freeze test exercise real Git tree checks
// without touching any game checkout or large qualification inputs.
function freezeFixture(){
  const root=temporary(),repo=path.join(root,'repo');fs.mkdirSync(repo);
  const git=args=>cp.execFileSync('git',['-C',repo,...args],{stdio:'pipe',windowsHide:true}).toString().trim();
  git(['init','-q']);git(['config','user.name','Verifier unit fixture']);git(['config','user.email','unit@example.invalid']);
  for(const file of [...q.HARNESS,...q.ASSETS.map(n=>'spheres-web/ui/'+n),'Cargo.toml','Cargo.lock','spheres-sim/src/lib.rs'])write(path.join(repo,file),'unit fixture '+file);
  fs.copyFileSync(path.join(__dirname,'s22-qualification.cjs'),path.join(repo,'tools/campaign/s22-qualification.cjs'));
  git(['add','.']);git(['commit','-qm','Synthetic verifier unit fixture']);const revision=git(['rev-parse','HEAD']);
  const protocol=read(path.join(__dirname,'../../docs/campaign-certification/S22/measurement-protocol.json'));
  const source=path.join(root,'source.json');write(source,'original synthetic source');protocol.source_campaign.uncompressed_sha256=q.fileHash(source);write(path.join(root,'protocol.json'),protocol);
  write(path.join(root,'hardware.json'),{reference_id:'SPHERES-REF-WIN-01',cpu:'AMD Ryzen 7 9800X3D',gpu:'NVIDIA GeForce RTX 5070',physical_ram_bytes:123,os:'synthetic fixture',gpu_driver:'synthetic fixture',power_background_observations:'synthetic fixture',browser:{version:'unit',channel:'msedge',headless:true,extra_flags:[]}});
  write(path.join(root,'provenance.json'),{unit_fixture:true});write(path.join(root,'server.exe'),'unit binary');write(path.join(root,'native.exe'),'unit native');
  const cases=q.CASES.map(c=>{const file=path.join(root,c.id+'.json'),[year,month,day]=calendar(c.date);write(file,{format:'spheres-campaign',world:{format:'spheres-integrated-save',world:{year,month,day}}});return {id:c.id,input_path:file};});
  const config={schema:'spheres-s22-qualification-config/v1',attempt_id:'unit-only-01',repository:repo,candidate_revision:revision,protocol_path:path.join(root,'protocol.json'),hardware_path:path.join(root,'hardware.json'),source_path:source,provenance_paths:[path.join(root,'provenance.json')],binaries:{native_test:path.join(root,'native.exe'),server:path.join(root,'server.exe')},cases,cells:cells(root),registry_path:path.join(root,'registry.jsonl')};
  const evidence=config.provenance_paths.map(p=>({path:p,sha256:q.fileHash(p)})),inputs=cases.map((c,i)=>({id:c.id,actual_date:q.CASES[i].date,sha256:q.fileHash(c.input_path)}));
  const lineage={schema:'spheres-s22-reviewed-lineage/v1',status:'reviewed',reviewed_by:'Synthetic verifier test fixture (not a real review)',reviewed_utc:'2026-01-01T00:00:00Z',review_scope:'Synthetic test only, no campaign provenance claim',original_source_sha256:q.fileHash(source),adopted_early_sha256:inputs[0].sha256,inputs,
    links:inputs.map((input,i)=>({kind:i?'native_preparation_resume':'ordinary_competition_adoption',from_sha256:i?inputs[i-1].sha256:q.fileHash(source),to_sha256:input.sha256,from_date:i?inputs[i-1].actual_date:input.actual_date,to_date:input.actual_date,outcome:'completed',evidence,...(i?{}:{command:'EnableEconomicCompetition',price_pc:0})}))};
  config.lineage_path=path.join(root,'lineage.json');write(config.lineage_path,lineage);
  const configPath=path.join(root,'config.json'),manifestPath=path.join(root,'manifest.json');write(configPath,config);
  return {root,repo,config,configPath,manifestPath,git};
}
test('freeze reserves all outputs before launch, refuses overwrite and reports missing cells truthfully',()=>{
  const f=freezeFixture();q.freeze(f.configPath,f.manifestPath);assert.throws(()=>q.freeze(f.configPath,f.manifestPath),/never overwrite/);
  const result=q.verify(f.manifestPath);assert.equal(result.passed,false);assert.equal(result.status,'incomplete');assert.equal(result.cells.length,18);
  f.config.attempt_id='unit-only-02';write(f.configPath,f.config);assert.throws(()=>q.freeze(f.configPath,path.join(f.root,'second.json')),/already reserved/);
});
test('freeze rejects a pre-existing output and wrong late campaign date before reserving',()=>{
  const f=freezeFixture();fs.mkdirSync(f.config.cells[0].output_path);assert.throws(()=>q.freeze(f.configPath,f.manifestPath),/already exists/);
  const wrong=freezeFixture();mutate(wrong.config.cases[2].input_path,p=>{p.world.world.day=31;});assert.throws(()=>q.freeze(wrong.configPath,wrong.manifestPath),/actual frozen date/);
});
test('post-freeze changed config, input, binary, harness or runtime invalidates the whole pair',()=>{
  for(const kind of ['config','input','binary','harness','runtime']){
    const f=freezeFixture();q.freeze(f.configPath,f.manifestPath);
    if(kind==='config')fs.appendFileSync(f.configPath,'\n');
    if(kind==='input')fs.appendFileSync(f.config.cases[0].input_path,'\n');
    if(kind==='binary')fs.appendFileSync(f.config.binaries.server,'x');
    if(kind==='harness')fs.appendFileSync(path.join(f.repo,q.HARNESS[0]),'x');
    if(kind==='runtime')fs.appendFileSync(path.join(f.repo,'spheres-sim/src/lib.rs'),'x');
    const verdict=q.verify(f.manifestPath);assert.equal(verdict.status,'invalid_identity',kind);assert.equal(verdict.passed,false);
  }
});
test('documentation-only committed HEAD changes preserve the pinned runtime tree',()=>{
  const f=freezeFixture();q.freeze(f.configPath,f.manifestPath);write(path.join(f.repo,'docs/note.md'),'unit note');f.git(['add','docs/note.md']);f.git(['commit','-qm','Documentation only']);assert.equal(q.verify(f.manifestPath).status,'incomplete');
});
test('freeze requires explicit reviewed, reachable, hash-matching source/adoption lineage',()=>{
  for(const change of [l=>{l.status='pending';},l=>{l.inputs[2].sha256='f'.repeat(64);},l=>{l.links[0].price_pc=1;},l=>{l.links[2].from_sha256='a'.repeat(64);},l=>{l.links[1].evidence[0].sha256='b'.repeat(64);}]){
    const f=freezeFixture();mutate(f.config.lineage_path,change);assert.throws(()=>q.freeze(f.configPath,f.manifestPath));
  }
});
