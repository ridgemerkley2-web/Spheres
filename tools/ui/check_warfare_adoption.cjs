// Pure controls with synthetic state. Native fixtures separately qualify campaign property.
const {test}=require('node:test'),assert=require('node:assert/strict'),fs=require('node:fs'),path=require('node:path'),vm=require('node:vm');
const root=path.resolve(__dirname,'../..'),page=fs.readFileSync(path.join(root,'spheres-web/ui/index.html'),'utf8');
const ui=require(path.join(root,'spheres-web/ui/campaign-operations-ui.js'));
const host=page.slice(page.indexOf('const WARFARE_UPGRADE='),page.indexOf('\nfunction bindForceAllocations(',page.indexOf('const WARFARE_UPGRADE=')));
const defer=()=>{let resolve;return {promise:new Promise(r=>resolve=r),resolve:v=>resolve(v)};};
const flush=()=>new Promise(r=>setImmediate(r));
function harness(){
  const confirmation=defer(),request=defer(),calls=[],view={enabled:false,available:true,effect:'Synthetic rule review'};
  const c=vm.createContext({S:{session_id:'one',player:'France',warfare_adoption:view},view,COMMAND_CHANNEL:{},SESSION:{},advancing:false,pendingAdvance:false,
    campaignConfirm:(...args)=>{calls.push(['review',...args]);return confirmation.promise;},api:(...args)=>{calls.push(['api',...args]);return request.promise;},adopt:async value=>{calls.push(['adopt',value]);c.S=value;},banner:value=>calls.push(['error',value]),
  });vm.runInContext(host,c);Object.assign(c,{confirmation,request,calls});return c;
}
test('operations adoption is reviewed before one normal command is sent',async()=>{
  const c=harness(),pending=c.reviewWarfareAdoption(c.view);assert.equal(c.calls.length,1);assert.equal(await c.reviewWarfareAdoption(c.view),false);
  assert.deepEqual(JSON.parse(JSON.stringify(c.calls[0])),['review',c.view.effect,{title:'Enable campaign operations',confirmLabel:'Enable operations'}],'The real dialog contract must display the native effects before approval');
  c.confirmation.resolve(true);await flush();assert.equal(c.calls[1][1],'/api/command');
  assert.deepEqual(JSON.parse(JSON.stringify(c.calls[1][2])),{commands:[{kind:'enable_operational_warfare'}]});
  c.request.resolve({session_id:'one',player:'France',errors:[]});assert.equal(await pending,true);assert.equal(c.calls.filter(row=>row[0]==='adopt').length,1);
});
test('cancelling or changing the campaign while reviewing cannot adopt warfare',async()=>{
  for(const reason of ['cancel','campaign','state','pending']){
    const c=harness(),pending=c.reviewWarfareAdoption(c.view);
    if(reason==='campaign')c.S={session_id:'two',player:'Japan'};
    if(reason==='state')c.S={...c.S};if(reason==='pending')c.COMMAND_CHANNEL.pending={};
    c.confirmation.resolve(reason!=='cancel');assert.equal(await pending,false);assert.equal(c.calls.filter(row=>row[0]==='api').length,0);
  }
});
test('unavailable or already enabled adoption is inert and escapes its explanation',async()=>{
  const c=harness();c.view.available=false;c.view.reason='<img src=x>';assert.equal(await c.reviewWarfareAdoption(c.view),false);
  const html=ui.adoptionHtml(c.view);assert(html.includes('&lt;img'));assert(html.includes('disabled'));assert(!html.includes('<img'));
  c.view.enabled=true;assert.equal(ui.adoptionHtml(c.view),'');assert.equal(ui.adoptionHtml(null),'');
});
test('uncertain receipt and stale response are never adopted or retried automatically',async()=>{
  for(const mode of ['pending','replaced']){
    const c=harness(),pending=c.reviewWarfareAdoption(c.view);c.confirmation.resolve(true);await flush();
    if(mode==='replaced')c.S={session_id:'two',player:'Japan'};
    c.request.resolve({session_id:'one',player:'France',command_pending:mode==='pending'});assert.equal(await pending,false);
    assert.equal(c.calls.filter(row=>row[0]==='api').length,1);assert.equal(c.calls.filter(row=>row[0]==='adopt').length,0);
  }
});
test('campaign order bindings refuse forms from a replaced world before calling transport',async()=>{
  const start=page.indexOf('function bindForceAllocations('),end=page.indexOf('\n}',start)+2,source=page.slice(start,end);
  let send;const calls=[],c=vm.createContext({S:{session_id:'one'},COMMAND_CHANNEL:{},advancing:false,pendingAdvance:false,
    window:{CampaignOperationsUI:{bind:(_,callback)=>{send=callback;}}},clockPause:()=>{},api:async()=>{calls.push('api');return {};},adopt:async()=>{},reviewWarfareAdoption:()=>{},
  });vm.runInContext(source,c);c.bindForceAllocations({isConnected:true,querySelectorAll:()=>[]});c.S={session_id:'two'};
  await assert.rejects(send({kind:'operation',conflict:7}),/campaign or current order changed/);assert.deepEqual(calls,[]);
});
test('staged inline JavaScript remains syntactically valid',()=>{for(const script of page.matchAll(/<script\b([^>]*)>([\s\S]*?)<\/script>/g))if(!/\btype\s*=\s*["'](?:application\/json|importmap)/.test(script[1]))new vm.Script(script[2]);});
