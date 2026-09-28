'use strict';
const assert=require('node:assert/strict'),crypto=require('node:crypto'),fs=require('node:fs'),path=require('node:path'),zlib=require('node:zlib');
const hash=bytes=>crypto.createHash('sha256').update(bytes).digest('hex');
const MINISTRIES=['health','education','housing','pensions','infrastructure','industry','science','defense','security','diplomacy'];
const SOURCES=['tools/ui/worldwide_startup_browser.cjs','tools/ui/worldwide-startup-lib.cjs','tools/ui/verify_worldwide_startup.cjs','tools/ui/ci-integrated.cjs','tools/ui/successor-fixtures/lib.cjs'];
function canonicalRoster(bytes){return require('./successor-fixtures/lib.cjs').rosterRows(bytes.toString('utf8')).filter(r=>r.start_1990).map(r=>({id:r.code,name:r.name})).sort((a,b)=>a.id.localeCompare(b.id));}
const CHECKS=['ordinary_selection','opening_map','opening_government','opening_budget','opening_guidance','seven_visible_days','named_save','ordinary_reload','reloaded_map','reloaded_government','reloaded_budget','reloaded_guidance','exact_archive_roundtrip','no_page_errors','ordinary_mutations_only'];
function day(state){assert(Number.isInteger(state.year)&&Number.isInteger(state.month)&&Number.isInteger(state.day));return Date.UTC(state.year,state.month-1,state.day)/86400000;}
function archive(bytes){
  const text=bytes.toString('utf8'),value=JSON.parse(text);
  assert(Buffer.from(text,'utf8').equals(bytes),'Native archive must be valid UTF-8');
  assert.equal(value.format,'spheres-campaign');assert.equal(value.version,1);
  for(const key of ['world','history','log','journey','history_epoch','saved_date','player','saved_unix'])assert(Object.hasOwn(value,key),'Missing native archive '+key);
  assert(Number.isSafeInteger(value.saved_unix)&&value.saved_unix>0);
  const pattern=/(,"saved_unix":)\d+(?=\}\s*$)/;assert(pattern.test(text),'Expected terminal native saved_unix');
  assert.equal(value.world?.format,'spheres-integrated-save');const world=value.world.world;assert(world&&typeof world.player==='string');
  return {bytes:bytes.length,sha256:hash(bytes),campaign_sha256:hash(text.replace(pattern,'$1<TIMESTAMP>')),player:value.player,date:value.saved_date,world_player:world.player,world_date:{year:world.year,month:world.month,day:world.day}};
}
function roster(rows){
  assert(Array.isArray(rows));const values=rows.filter(row=>row.alive!==false).map(row=>({id:row.id,name:row.name})).sort((a,b)=>a.id.localeCompare(b.id));
  assert.equal(values.length,137,'Expected all 137 ordinary starters');assert.equal(new Set(values.map(row=>row.id)).size,137);
  assert(values.every(row=>typeof row.id==='string'&&row.id&&typeof row.name==='string'&&row.name));return values;
}
function validate(result){
  const errors=[],need=(ok,why)=>{if(!ok)errors.push(why);};
  need(result?.format==='spheres-worldwide-browser/v1','Wrong result format');
  const rows=result.roster||[],ids=rows.map(r=>r.id),wanted=result.requested_ids||[],cases=result.cases||[];
  need(ids.length===137&&new Set(ids).size===137,'Roster must contain 137 unique starters');
  need(JSON.stringify(result.canonical_roster?.rows)===JSON.stringify(rows)&&/^[a-f0-9]{40}$/.test(result.canonical_roster?.git_blob||''),'Roster must match the pinned native start_1990 inventory');
  need(Number.isSafeInteger(result.seed)&&result.seed>=0,'Missing ordinary campaign seed');
  need(wanted.length>0&&new Set(wanted).size===wanted.length&&wanted.every(id=>ids.includes(id)),'Requested identities are missing, duplicated or unknown');
  if(result.coverage==='all_starters')need(wanted.length===137&&result.pilot===false&&result.attached_server===false,'A full run cannot be a subset, pilot or attached-server run');
  else need(result.coverage==='pilot'&&result.pilot===true,'Unknown or mislabeled run scope');
  need(cases.length===wanted.length&&new Set(cases.map(c=>c.nation)).size===wanted.length&&cases.every(c=>wanted.includes(c.nation)),'Case inventory must exactly match the request');
  need(/^[a-f0-9]{40}$/.test(result.runtime?.expected_revision||'')&&result.runtime?.build?.full_revision===result.runtime?.expected_revision,'Runtime revision is missing or inconsistent');
  need(/^[a-f0-9]{64}$/.test(result.runtime?.binary_sha256||''),'Missing binary identity');
  need(result.cleanup?.browser_closed===true&&(result.attached_server?result.cleanup?.server_left_running===true:result.cleanup?.server_stopped===true),'Cleanup is incomplete');
  need(result.immutable_inputs_unchanged===true&&result.runtime?.binary_sha256_after===result.runtime?.binary_sha256,'Frozen runtime/harness inputs changed during the run');
  for(const c of cases){
    const row=rows.find(r=>r.id===c.nation);need(row&&c.name===row.name,c.nation+': display name does not match the roster');
    need(c.passed===true,c.nation+': failed/unfinished case');
    need(Array.isArray(c.checks)&&CHECKS.every(k=>c.checks.includes(k)),c.nation+': missing required checks');
    need(c.before?.year===1990&&c.before?.month===1&&c.before?.day===1&&c.before?.player===c.nation,c.nation+': wrong ordinary opening');
    const advances=c.advances||[];
    need(advances.length===7,c.nation+': missing seven visible advances');
    if(advances.length===7){let expected=Date.UTC(1990,0,1)/86400000;for(const a of advances){need(a.before===expected&&a.after===expected+1&&a.days===1&&a.player===c.nation&&!a.errors?.length,c.nation+': incorrect daily advance');expected++;}}
    need(c.after?.year===1990&&c.after?.month===1&&c.after?.day===8&&c.after?.player===c.nation,c.nation+': wrong resumed date/player');
    need(c.before_save?.campaign_sha256===c.after_save?.campaign_sha256&&/^[a-f0-9]{64}$/.test(c.before_save?.campaign_sha256||''),c.nation+': archive roundtrip mismatch');
    need(c.old_session&&c.new_session&&c.old_session!==c.new_session,c.nation+': load did not renew session');
    need(Array.isArray(c.page_errors)&&c.page_errors.length===0,c.nation+': browser errors');
    for(const phase of ['opening','reloaded']){const view=c.views?.[phase],m=view?.map;need(m?.draws_after>m?.draws_before&&m?.width>0&&m?.height>0&&m?.owned_districts>0&&m?.player===c.nation&&m?.visible===true&&m?.in_viewport===true&&Object.values(m.camera||{}).length===5&&Object.values(m.camera).every(Number.isFinite),c.nation+': no visible rendered owned map at '+phase);need(view?.government?.nation===c.nation&&view?.government?.hero===c.name&&view?.government?.officeholder&&view?.budget?.nation===c.nation&&JSON.stringify([...(view?.budget?.ministries||[])].sort())===JSON.stringify([...MINISTRIES].sort())&&view?.budget?.summary&&view?.budget?.kicker?.includes(c.name)&&view?.guidance?.player===c.nation&&view?.guidance?.cards,c.nation+': missing room observations at '+phase);for(const room of ['government','budget','guidance']){const box=view?.[room]?.layout;need(Number.isFinite(box?.width)&&box.width>0&&Number.isFinite(box.scroll)&&box.scroll>=0&&box.scroll<=box.width+1,c.nation+': invalid '+phase+' '+room+' layout');}}
    const mutations=(c.requests||[]).filter(r=>r.path!=='/api/program-preview');
    need(JSON.stringify(mutations.map(r=>r.path))===JSON.stringify(['/api/new',...Array(7).fill('/api/advance'),'/api/save','/api/load','/api/save']),c.nation+': unexpected or missing mutation');
    need(mutations.filter(r=>r.path==='/api/advance').every(r=>Array.isArray(r.payload?.commands)&&r.payload.commands.length===0&&r.payload.days===1),c.nation+': unexpected daily commands');
    need(mutations[0]?.payload?.seed===result.seed&&mutations[0]?.payload?.nation===c.name,c.nation+': ordinary seed or selected country mismatch');
    need((c.screenshots||[]).length===8,c.nation+': incomplete visible evidence');
    for(const phase of ['opening','reloaded'])for(const room of ['map','government','budget','guidance']){const shots=(c.screenshots||[]).filter(s=>s.stage===phase+'-'+room);need(shots.length===1&&shots[0].viewport?.width===(phase==='opening'?1440:390)&&shots[0].viewport?.height===(phase==='opening'?1000:844),c.nation+': missing '+phase+' '+room+' viewport');}
  }
  return errors;
}
function verifyEvidence(result,root){
  assert.equal(result.passed,true,'Cannot verify a failed or unfinished attempt as passed');assert(!result.error,'Retained attempt contains an error');
  assert.deepEqual(validate(result),[],'Recorded run is incomplete or failed');root=path.resolve(root);let files=0,bytes=0;
  function file(pin,gzip){assert(pin&&typeof pin.path==='string');const target=path.resolve(root,pin.path),rel=path.relative(root,target);assert(rel&&!rel.startsWith('..')&&!path.isAbsolute(rel),'Evidence path escaped its root');assert(fs.lstatSync(target).isFile()&&!fs.lstatSync(target).isSymbolicLink());assert.equal(fs.realpathSync(target),target);const data=fs.readFileSync(target);assert.equal(data.length,gzip?pin.gzip_bytes:pin.bytes);assert.equal(hash(data),gzip?pin.gzip_sha256:pin.sha256);files++;bytes+=data.length;return data;}
  assert.deepEqual((result.harness?.sources||[]).map(p=>p.source_path).sort(),[...SOURCES].sort(),'Retain every harness dependency');for(const pin of result.harness.sources)file(pin,false);
  const nations=file(result.canonical_roster,false);assert.equal(crypto.createHash('sha1').update(Buffer.from('blob '+nations.length+'\0')).update(nations).digest('hex'),result.canonical_roster.git_blob);assert.deepEqual(canonicalRoster(nations),result.roster,'Served roster differs from the retained native source');
  for(const c of result.cases){for(const pin of [c.before_save,c.after_save]){const raw=zlib.gunzipSync(file(pin,true)),observed=archive(raw);for(const key of ['sha256','bytes','campaign_sha256','player','date','world_player','world_date'])assert.deepEqual(observed[key],pin[key],c.nation+' archive '+key);assert.equal(observed.player,c.name);assert.equal(observed.world_player,c.nation);assert.deepEqual(observed.world_date,{year:1990,month:1,day:8});assert.equal(observed.date,'8 Jan 1990');}for(const shot of c.screenshots)file(shot,false);}
  return {passed:true,files_verified:files,retained_bytes:bytes,cases_verified:result.cases.length,all_137_passed:result.coverage==='all_starters',qualification:false};
}
module.exports={hash,day,archive,roster,validate,verifyEvidence,CHECKS,MINISTRIES,SOURCES,canonicalRoster};
