// Browser receipt/backup intent checks. Native S05 cases check actual money/work.
const {test}=require('node:test'),assert=require('node:assert/strict');
const fs=require('node:fs'),path=require('node:path'),vm=require('node:vm');
const sourceRoot=fs.existsSync(path.join(__dirname,'../../spheres-web/ui/index.html'))?path.join(__dirname,'../..'):path.join(__dirname,'../../../s04-staging');
const page=fs.readFileSync(path.join(sourceRoot,'spheres-web/ui/index.html'),'utf8');
const transportPath=path.join(sourceRoot,'spheres-web/ui/campaign-transport.js');
const transport=require(fs.existsSync(transportPath)?transportPath:path.join(__dirname,'../../../integration/spheres-web/ui/campaign-transport.js'));
function fn(name){const hit=new RegExp(`^(?:async\\s+)?function ${name}\\(`,'m').exec(page);assert(hit);return page.slice(hit.index,page.indexOf('\n}',hit.index)+2);}
function pageFixture(){
  let confirm;const sent=[],banners=[],adopted=[];
  const nodes=new Map();const $=key=>{if(!nodes.has(key))nodes.set(key,{value:'Integrated-France',disabled:false});return nodes.get(key);};
  const context=vm.createContext({$,banner:message=>banners.push(message),clockPause(){},renderSessionActions(){},syncCommandControls(){},guardEconomicOrder:()=>false,
    campaignConfirm:()=>new Promise(resolve=>{confirm=resolve;}),enterCampaign:async(state,replacing)=>adopted.push({state,replacing}),
    fetch:async(url,options)=>{sent.push({url,payload:JSON.parse(options.body)});return{ok:true,text:async()=>JSON.stringify({session_id:'loaded',player:'France',ok:true})};}});
  vm.runInContext(`let S={session_id:'current',player:'France'};const SESSION={live:S,busy:false,slot:'Integrated-France'};
    let advancing=false,pendingAdvance=null;const COMMAND_CHANNEL={busy:false,pending:null};
    ${fn('api')}\n${fn('loadCampaign')}\n${fn('doSave')}`,context);
  return{context,sent,banners,adopted,confirm:value=>confirm(value)};
}
test('a company receipt appearing during load confirmation is retained and blocks replacement at the real API boundary',async()=>{
  const f=pageFixture(),p=vm.runInContext('loadCampaign(true)',f.context);
  vm.runInContext(`COMMAND_CHANNEL.pending={session_id:'current',client_id:'supplier-order',request_seq:7,commands:[{kind:'company_establish',quote:'reviewed-paid-formation'}]}`,f.context);
  f.confirm(true);await p;
  assert.equal(f.sent.length,0);assert.equal(f.adopted.length,0);
  assert.equal(vm.runInContext('COMMAND_CHANNEL.pending.commands[0].quote',f.context),'reviewed-paid-formation');
  assert.ok(f.banners.some(message=>/pending order/.test(message)));
  assert.equal(vm.runInContext('SESSION.busy',f.context),false);
});
test('explicit backup load preserves slot and backup intent and adopts the replacement once',async()=>{
  const f=pageFixture(),p=vm.runInContext('loadCampaign(true)',f.context);f.confirm(true);await p;
  assert.deepEqual(f.sent,[{url:'/api/load',payload:{slot:'Integrated-France',backup:true}}]);
  assert.equal(f.adopted.length,1);assert.equal(f.adopted[0].replacing,true);
  assert.equal(vm.runInContext('SESSION.slot',f.context),'Integrated-France');
});
test('cancelled backup review leaves the live campaign and write lane untouched',async()=>{
  const f=pageFixture(),p=vm.runInContext('loadCampaign(true)',f.context);f.confirm(false);await p;
  assert.equal(f.sent.length,0);assert.equal(f.adopted.length,0);assert.equal(vm.runInContext('SESSION.busy',f.context),false);
});
test('an unconfirmed paid company order blocks saving through the real common API boundary',async()=>{
  const f=pageFixture();vm.runInContext(`COMMAND_CHANNEL.pending={session_id:'current',commands:[{kind:'company_establish'}]}`,f.context);
  assert.equal(await vm.runInContext('doSave()',f.context),false);assert.equal(f.sent.length,0);
  assert.ok(f.banners.some(message=>/Save failed:.*pending order/.test(message)));
});
test('persisted typed contractor review retains the exact quote and cannot replay into a loaded campaign',async()=>{
  const storage=new Map(),sent=[];let current='campaign-before-load';
  const options={session:()=>current,identity:()=>({client_id:'company-browser',request_seq:19}),storage:{getItem:key=>storage.get(key),setItem:(key,value)=>storage.set(key,value),removeItem:key=>storage.delete(key)},request:async payload=>{sent.push(JSON.parse(JSON.stringify(payload)));throw Error('lost response');}};
  const first=transport.create(options),command={kind:'assign_sector_contractor',company:3,target:{kind:'construction',project:71},quote:'current-reviewed-token'};
  await assert.rejects(first.send({commands:[command]}),/lost response/);command.target.project=99;
  const restored=transport.create(options);assert.equal(restored.pending.commands[0].target.project,71);assert.equal(restored.pending.commands[0].quote,'current-reviewed-token');
  current='loaded-campaign';await assert.rejects(restored.retry(),/another campaign/);assert.equal(sent.length,1);assert.ok(restored.pending);
});
