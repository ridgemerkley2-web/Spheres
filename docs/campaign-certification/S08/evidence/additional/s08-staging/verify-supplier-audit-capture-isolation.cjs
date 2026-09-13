// Exact archive helper in a VM, using tiny real outside-only files and a native
// save boundary double. No browser, server or real campaign archive is loaded.
'use strict';
const assert=require('node:assert/strict'),fs=require('node:fs'),path=require('node:path'),vm=require('node:vm'),crypto=require('node:crypto');
const staging=__dirname,source=path.join(staging,'ci-supplier-imports.audit-capture-isolation.cjs');
const bindings=JSON.parse(fs.readFileSync(path.join(staging,'supplier-audit-capture-isolation-bindings.json'),'utf8'));
const text=fs.readFileSync(source,'utf8'),prior=fs.readFileSync(bindings.prior,'utf8'),hash=bytes=>crypto.createHash('sha256').update(bytes).digest('hex');
assert.equal(hash(text),bindings.candidate_sha256);
const expression=/async function archive\(page,url,run,slot\)\{[\s\S]*?\n\}/;
const helper=text.match(expression)[0],old=prior.match(expression)[0];
assert.equal(text.replace(helper,old),prior,'Only archive() may differ from c74');
const retained=fs.mkdtempSync(path.join(staging,'supplier-audit-placement-vm-'));
const output=process.argv[2]||path.join(staging,'supplier-audit-capture-isolation-vm-evidence.json');
const evidence={passed:false,started_utc:new Date().toISOString(),source,source_sha256:hash(text),helper_sha256:hash(helper),prior_sha256:hash(prior),verifier_sha256:hash(fs.readFileSync(__filename)),complete_patch_sha256:bindings.complete_patch.sha256,incremental_patch_sha256:bindings.incremental_patch.sha256,retained_tiny_fixture_directory:retained,tests:[]};
const bytes=Buffer.from('{\n "native": "unchanged Ω", "signed": -0, "amount": 2.543259532221702e-7, "world": {"order":[3,1,2]}\n}\n','utf8');
const auditNames=['s08-preparation-before','s08-preparation-after','s08-preview-before','s08-preview-after','s08-purchase-before','s08-purchase-after','s08-cancel-review','s08-paid-transit','s08-import-midpoint-before','s08-import-midpoint-after','s08-delivered','s08-maintenance-review','s08-maintenance-approved','s08-import-complete-before','s08-import-complete-after','s08-continue',...Array.from({length:7},(_,i)=>'s08-maintenance-day-'+(i+1))];
let fixtureSequence=0;
function fixture(options={}){
 const run=path.join(retained,String(++fixtureSequence)),saves=path.join(run,'saves');fs.mkdirSync(saves,{recursive:true});
 const realSlots=['s08-earned-stock','s08-preparation-input','s08-import-midpoint','s08-import-complete','auto-2','player-campaign'];
 for(const name of realSlots)fs.writeFileSync(path.join(saves,name+'.json'),'untouched real slot '+name,{flag:'wx'});
 const preserved=realSlots.map(name=>({name,sha256:hash(fs.readFileSync(path.join(saves,name+'.json')))}));
 const calls={post:0,body:0,dispose:0,evaluate:0,inspect:0,rename:0},moves=[];let capture,lastSlot;
 const fsView={...fs,
  realpathSync(file){if(options.escapeSaves&&file===saves)return path.join(retained,'other-run','saves');if(options.escapeTarget&&file===path.join(run,'audit-captures'))return path.join(retained,'other-run','audit-captures');return fs.realpathSync(file);},
  mkdtempSync(prefix){const directory=fs.mkdtempSync(prefix);if(options.targetCollision)fs.writeFileSync(path.join(directory,lastSlot+'.json'),'preserved collision',{flag:'wx'});return directory;},
  renameSync(from,to){
   const check=(parent,file)=>{const rel=path.relative(parent,file);assert(rel&&rel!=='..'&&!rel.startsWith('..'+path.sep)&&!path.isAbsolute(rel));};
   check(saves,from);check(path.join(run,'audit-captures'),to);assert(!fs.existsSync(to));calls.rename++;
   if(options.renameError)throw options.renameError;
   const before=hash(fs.readFileSync(from));fs.renameSync(from,to);const after=hash(fs.readFileSync(to));assert.equal(after,before);
   capture=to;moves.push({from,to,before_sha256:before,after_sha256:after});
  }
 };
 const page={request:{async post(url,args){assert.equal(url,'http://fixture.invalid/api/save');assert.deepEqual(Object.keys(args),['data']);assert.deepEqual(Object.keys(args.data),['slot']);lastSlot=args.data.slot;calls.post++;if(!options.saveError)fs.writeFileSync(path.join(saves,lastSlot+'.json'),bytes,{flag:'wx'});return {ok(){return !options.saveError;},async body(){calls.body++;return Buffer.from('{"ok":true}');},async dispose(){calls.dispose++;if(options.disposeError)throw options.disposeError;}};}},async evaluate(){calls.evaluate++;if(options.buyerError)throw options.buyerError;return 'Tonga';}};
 const audit={inspect(file,buyer){calls.inspect++;assert.equal(buyer,'Tonga');assert.equal(file,capture);assert(fs.readFileSync(file).equals(bytes),'Native archive bytes must never be reserialized');assert(!fs.existsSync(path.join(saves,lastSlot+'.json')));if(options.auditError)throw options.auditError;return {input:{path:file,sha256:hash(fs.readFileSync(file))},canonical:{existing_worker_contract:true}};}};
 const fn=vm.runInNewContext(helper+'\narchive',{assert,fs:fsView,path,audit,S:{player:'Tonga'}});
 const clean=()=>{for(const row of preserved)assert.equal(hash(fs.readFileSync(path.join(saves,row.name+'.json'))),row.sha256,'Actual named/input slot changed');};
 return {run,saves,calls,moves,page,fn,clean,capture:()=>capture};
}
async function test(name,run){const row={name,passed:false};evidence.tests.push(row);try{await run(row);row.passed=true;}catch(error){row.error=String(error.stack||error);throw error;}}
async function main(){
 await test('all actual callsites map to the closed audit-only inventory',async row=>{
  const literals=[...prior.matchAll(/archive\(page,url,run,'([^']+)'\)/g)].map(match=>match[1]);
  assert(literals.every(name=>auditNames.includes(name)));assert(prior.includes("archive(page,url,run,slot+'-before')")&&prior.includes("archive(page,url,run,slot+'-after')"));
  assert.deepEqual([...prior.matchAll(/saveLoad\(page,url,run,'([^']+)'\)/g)].map(match=>match[1]),['s08-import-midpoint','s08-import-complete']);
  assert(prior.includes('for(let i=0;i<7;i++){')&&prior.includes("archive(page,url,run,'s08-maintenance-day-'+(i+1))"));
  const expanded=[...new Set([...literals,'s08-import-midpoint-before','s08-import-midpoint-after','s08-import-complete-before','s08-import-complete-after',...Array.from({length:7},(_,i)=>'s08-maintenance-day-'+(i+1))])];
  assert.deepEqual(expanded.sort(),[...auditNames].sort());row.allowed_slots=[...auditNames];
 });
 await test('every audit name preserves exact native bytes outside visible slots',async row=>{
  const f=fixture();for(const name of auditNames){const result=await f.fn(f.page,'http://fixture.invalid',f.run,name);assert.equal(result.input.sha256,hash(bytes));}
  assert.equal(f.calls.rename,auditNames.length);assert.equal(f.calls.dispose,auditNames.length);assert.equal(fs.readdirSync(f.saves).length,6);f.clean();row.moves=f.moves;
 });
 await test('real slot names and unlisted maintenance days fail before saving or moving',async()=>{
  for(const name of ['s08-earned-stock','s08-preparation-input','s08-import-midpoint','s08-import-complete','auto-2','player-campaign','../escape','s08-maintenance-day-8']){
   const f=fixture();await assert.rejects(f.fn(f.page,'http://fixture.invalid',f.run,name),/non-audit/);assert.equal(f.calls.post,0);assert.equal(f.calls.rename,0);f.clean();
  }
 });
 await test('existing audit-named input cannot be overwritten by the capture POST',async()=>{
  const f=fixture(),source=path.join(f.saves,'s08-preview-before.json');fs.writeFileSync(source,'preserved',{flag:'wx'});
  await assert.rejects(f.fn(f.page,'http://fixture.invalid',f.run,'s08-preview-before'),/existing audit-named/);assert.equal(f.calls.post,0);assert.equal(fs.readFileSync(source,'utf8'),'preserved');f.clean();
 });
 await test('exclusive target collision preserves both evidence and original capture',async()=>{
  const f=fixture({targetCollision:true});await assert.rejects(f.fn(f.page,'http://fixture.invalid',f.run,'s08-preview-before'),/overwrite preserved/);
  assert.equal(f.calls.rename,0);assert(fs.readFileSync(path.join(f.saves,'s08-preview-before.json')).equals(bytes));
  const folder=path.join(f.run,'audit-captures'),first=fs.readdirSync(folder)[0];assert.equal(fs.readFileSync(path.join(folder,first,'s08-preview-before.json'),'utf8'),'preserved collision');f.clean();
 });
 for(const guard of ['escapeSaves','escapeTarget'])await test(guard+' is rejected before relocation',async()=>{
  const f=fixture({[guard]:true});await assert.rejects(f.fn(f.page,'http://fixture.invalid',f.run,'s08-preview-before'),/disposable run/);assert.equal(f.calls.rename,0);f.clean();
 });
 for(const stage of ['auditError','buyerError','disposeError'])await test(stage+' still retains exact capture in evidence directory',async()=>{
  const error=Error('sentinel '+stage),f=fixture({[stage]:error});await assert.rejects(f.fn(f.page,'http://fixture.invalid',f.run,'s08-preview-before'),cause=>cause===error);
  assert.equal(f.calls.rename,1);assert(fs.readFileSync(f.capture()).equals(bytes));assert.equal(f.calls.dispose,1);f.clean();
 });
 await test('rename failure preserves the original native capture',async()=>{
  const error=Error('sentinel rename failure'),f=fixture({renameError:error});await assert.rejects(f.fn(f.page,'http://fixture.invalid',f.run,'s08-preview-before'),cause=>cause===error);
  assert(fs.readFileSync(path.join(f.saves,'s08-preview-before.json')).equals(bytes));assert.equal(f.calls.inspect,0);assert.equal(f.calls.dispose,1);f.clean();
 });
 await test('failed native save response does not move or inspect a capture',async()=>{
  const f=fixture({saveError:true});await assert.rejects(f.fn(f.page,'http://fixture.invalid',f.run,'s08-preview-before'),/Could not capture/);assert.equal(f.calls.rename,0);assert.equal(f.calls.inspect,0);assert.equal(f.calls.dispose,1);f.clean();
 });
 evidence.passed=true;
}
main().catch(error=>{evidence.failure=String(error.stack||error);process.exitCode=1;}).finally(()=>{
 evidence.finished_utc=new Date().toISOString();fs.writeFileSync(output,JSON.stringify(evidence,null,2)+'\n');fs.writeFileSync(output.replace(/\.json$/,'')+'.log',evidence.tests.map(row=>(row.passed?'PASS ':'FAIL ')+row.name+(row.error?' '+row.error:'')).join('\n')+'\n');console.log(JSON.stringify({passed:evidence.passed,tests:evidence.tests.length,evidence:output}));
});
