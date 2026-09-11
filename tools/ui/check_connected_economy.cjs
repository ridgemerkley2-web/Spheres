// Behavioral controls use clearly synthetic numbers. Real campaign snapshots
// are produced separately by the native s02_connected_reading... test.
const {test}=require('node:test');
const assert=require('node:assert/strict');
const fs=require('node:fs'),path=require('node:path'),vm=require('node:vm');
const root=path.resolve(__dirname,'../..');
const ui=require(path.join(root,'spheres-web/ui/fiscal-recovery-ui.js'));
const page=fs.readFileSync(path.join(root,'spheres-web/ui/index.html'),'utf8');
const host=page.slice(page.indexOf('const CONNECTED_ECONOMY_UI='),page.indexOf('\nfunction cashFlowNavigate(',page.indexOf('const CONNECTED_ECONOMY_UI=')));
const defer=()=>{let resolve,reject;const promise=new Promise((a,b)=>{resolve=a;reject=b;});return {promise,resolve,reject};};
const flush=()=>new Promise(resolve=>setImmediate(resolve));
function harness(){
  const calls=[],confirmation=defer(),request=defer();
  const c=vm.createContext({S:{session_id:'synthetic-one',player:'France'},pending:false,calls,confirmation,request,FiscalRecoveryUI:ui,
    escText:String,cashFlowOrdersPending:()=>c.pending,
    campaignConfirm:(...args)=>{calls.push(['confirm',...args]);return confirmation.promise;},
    api:(...args)=>{calls.push(['api',...args]);return request.promise;},
    adopt:async value=>{calls.push(['adopt',value]);c.S=value;},banner:value=>calls.push(['banner',value]),renderLeft:()=>{},me:()=>null,
  });
  vm.runInContext(host,c);return c;
}
const command={kind:'enable_connected_economy'};
function issue(c){return c.connectedEconomyAct({},c.S,command,'Adopt connected economy','Synthetic reviewed effect',()=>true);}

test('relocated recovery and population controls both bind once and share current-state guards',()=>{
  const c=harness(),bindings=[],cleaned=[];
  c.FiscalRecoveryUI={bind:(panel,options)=>{bindings.push({panel,options});return ()=>cleaned.push(panel.id);}};
  const panels=[{id:'recovery'},{id:'population'}],container={querySelectorAll:selector=>{
    assert.equal(selector,'[data-connected-economy]');return panels;
  }};
  let current=true;const state=c.S;
  c.connectedEconomyBind(container,{upgrade:{available:true}},state,()=>current);
  assert.equal(bindings.length,2);assert(bindings.every(b=>b.options.isCurrent()));
  current=false;assert(bindings.every(b=>!b.options.isCurrent()));current=true;
  c.pending=true;assert(bindings.every(b=>b.options.isBusy()));c.pending=false;
  c.connectedEconomyBind(container,{},state,()=>current);
  assert.deepEqual(cleaned,['recovery','population']);assert.equal(bindings.length,4);
  c.S={...state};assert(bindings.every(b=>!b.options.isCurrent()),'same session with a replacement reading still invalidates old handlers');
});

test('economy adoption awaits review and sends one order through the normal command lane',async()=>{
  const c=harness(),pending=issue(c);
  assert.equal(c.calls.filter(x=>x[0]==='api').length,0);
  assert.equal(await issue(c),false,'a second click cannot queue another confirmation');
  c.confirmation.resolve(true);await flush();
  const requests=c.calls.filter(x=>x[0]==='api');assert.equal(requests.length,1);
  assert.equal(requests[0][1],'/api/command');assert.equal(requests[0].length,3,'no raw transport bypass');
  assert.deepEqual(JSON.parse(JSON.stringify(requests[0][2])),{commands:[command]});
  c.request.resolve({session_id:'synthetic-one',player:'France',errors:[]});
  assert.equal(await pending,true);assert.equal(c.calls.filter(x=>x[0]==='adopt').length,1);
});

test('cancelled review and a preexisting pending command cannot enact economy rules',async()=>{
  const c=harness(),pending=issue(c);c.confirmation.resolve(false);
  assert.equal(await pending,false);assert.equal(c.calls.filter(x=>x[0]==='api').length,0);
  const busy=harness();busy.pending=true;assert.equal(await issue(busy),false);assert.equal(busy.calls.length,0);
});

test('a changed campaign or intervening order invalidates an open confirmation',async()=>{
  for(const change of [c=>{c.S={session_id:'synthetic-two',player:'Japan'};},c=>{c.pending=true;}]){
    const c=harness(),pending=issue(c);change(c);c.confirmation.resolve(true);
    assert.equal(await pending,false);assert.equal(c.calls.filter(x=>x[0]==='api').length,0);
  }
});

test('an old response cannot replace a newly loaded campaign',async()=>{
  const c=harness(),pending=issue(c);c.confirmation.resolve(true);await flush();
  c.S={session_id:'synthetic-two',player:'Japan'};
  c.request.resolve({session_id:'synthetic-one',player:'France',errors:[]});
  assert.equal(await pending,false);assert.equal(c.calls.filter(x=>x[0]==='adopt').length,0);
  assert.equal(c.S.session_id,'synthetic-two');
});

test('a refused command is shown without adopting a success state or automatic retry',async()=>{
  const c=harness(),pending=issue(c);c.confirmation.resolve(true);await flush();
  c.request.resolve({session_id:'synthetic-one',player:'France',errors:['Policy changed while review was open.']});
  assert.equal(await pending,false);assert.equal(c.calls.filter(x=>x[0]==='adopt').length,0);
  assert.equal(c.calls.filter(x=>x[0]==='api').length,1);
  assert(c.calls.some(x=>x[0]==='banner'&&x[1].includes('Policy changed')));
});

function control(dataset){return {dataset,disabled:false,hasAttribute(key){return Object.hasOwn(this.dataset,key.replace(/^data-/,'').replace(/-([a-z])/g,(_,s)=>s.toUpperCase()));},closest(){return this;}};}
test('policy and upgrade bindings respect server availability, stale snapshots and cleanup',()=>{
  const available=control({economyPolicy:'trade_schools',economyAvailable:'true'}),refused=control({economyPolicy:'universities',economyAvailable:'false'}),upgrade=control({economyUpgrade:''});
  const buttons=[available,refused,upgrade],calls=[];let current=true,busy=false,handler=null;
  const el={querySelectorAll:()=>buttons,contains:b=>buttons.includes(b),addEventListener:(_,h)=>handler=h,removeEventListener:(_,h)=>{if(handler===h)handler=null;}};
  const unbind=ui.bind(el,{isCurrent:()=>current,isBusy:()=>busy,canUpgrade:false,onPolicy:value=>calls.push(value),onEnable:()=>calls.push('upgrade')});
  assert.equal(available.disabled,false);assert.equal(refused.disabled,true);assert.equal(upgrade.disabled,true);
  handler({target:available});handler({target:refused});handler({target:upgrade});assert.deepEqual(calls,['trade_schools']);
  current=false;handler({target:available});current=true;busy=true;handler({target:available});assert.equal(calls.length,1);
  unbind();assert.equal(handler,null);
});

test('connected presentation escapes names, preserves dated output and avoids fabricated output from readiness',()=>{
  const data={enabled:true,date:'4 Jan 1990',industry:{jobs_required:100,jobs_filled:0,advanced_components_stock:0,note:'Synthetic test reading',facilities:[
    {name:'<img src=x onerror=alert(1)>',district:'Synthetic & district',inherited:false,reason:'Ready next day',output_daily:0,output_unit:'parts',recorded_day:2,recorded_date:'3 Jan 1990',utilization:1,jobs_filled:0},
  ]}};
  const before=JSON.stringify(data),html=ui.renderConnected(data);
  assert(!html.includes('<img'));assert(html.includes('&lt;img'));assert(html.includes('Synthetic &amp; district'));
  assert(html.includes('Recorded 3 Jan 1990'));assert(!html.includes('simulation day'));
  assert(html.includes('0.000 parts'));assert(html.includes('100.0%'));assert.equal(JSON.stringify(data),before);
  assert.equal(ui.renderConnected(null),'');
});

test('every shipped inline script parses after connected economy integration',()=>{
  for(const script of page.matchAll(/<script\b([^>]*)>([\s\S]*?)<\/script>/g)){
    if(/\bsrc=|type="application\//.test(script[1]))continue;
    new vm.Script(script[2]);
  }
});

test('fiscal stability keeps the model native 0-to-100 point scale',()=>{
  const html=ui.render({available:true,enabled:true,assessment:{stability_change_per_month:-0.05,interest_gdp:-0.016,interest_revenue:0},actions:[]});
  assert(html.includes('-0.050 points / month'));
  assert(!html.includes('-5.00 percentage points'));
  assert(html.includes('-1.6%'));assert(html.includes('Positive net interest only'));
});

test('a partly adopted campaign still offers the complete upgrade in the compact budget view',()=>{
  const c=harness();
  const html=c.connectedEconomyPanelHtml({enabled:false,upgrade:{available:true,effect:'Adopt remaining accounts'},recovery:{available:true,enabled:true,assessment:{}}},true);
  assert(html.includes('data-economy-upgrade'));assert(html.includes('Adopt remaining accounts'));
});

test('a shipyard-only site can be selected for a server-eligible naval programme',()=>{
  const start=page.indexOf('function manufacturingProvinceHtml(');
  const source=page.slice(start,page.indexOf('\n}',start)+2);
  const kit={id:'nav_patrol',name:'Synthetic patrol',naval:true,eligible_provinces:['synthetic-dock']};
  const site={id:'synthetic-dock',name:'Synthetic dock',arms_plants:0,free_slots:0,naval_slots:1,free_naval_slots:1,actions:{start:['nav_patrol']}};
  const c=vm.createContext({MANU:{pickKit:'nav_patrol'},manufacturingKit:()=>kit,manufacturingProvinces:()=>[site],
    logisticsEscAttr:String,escText:String,manufacturingClassMark:()=>'',manufacturingBn:()=>'',fmtQ:String});
  vm.runInContext(source,c);
  const html=c.manufacturingProvinceHtml();assert(html.includes('data-manu-province="synthetic-dock"'));
  assert(html.includes('1 free · shipyard 1'));assert(!html.includes('0 free · arms plant 0'));
  kit.eligible_provinces=[];site.actions.start=[];assert(!c.manufacturingProvinceHtml().includes('data-manu-province='));
});

test('shipyard capacity stays separate and makes the ordinary catalogue entry action available',()=>{
  const start=page.indexOf('function manufacturingLinesHtml('),source=page.slice(start,page.indexOf('\n}',start)+2);
  const c=vm.createContext({MANU:{data:{summary:{capacity:0,used_slots:0,free_slots:0,naval_slots:1,used_naval_slots:0,free_naval_slots:1},actions:{start:true}}},
    manufacturingLines:()=>[],manufacturingLineHtml:()=>'',manufacturingBn:()=>'',operationsHeroHtml:()=>'',manufacturingLedgerHtml:()=>''});
  vm.runInContext(source,c);const html=c.manufacturingLinesHtml();
  assert(html.includes('Shipyard berths<b>0/1'));assert(html.includes('Plant slots<b>0/0'));
  assert(html.includes('data-manu-new>＋ Start equipment line'));assert(!html.includes('No arms-plant capacity'));
});
