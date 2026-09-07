// Exercise the shipped cash-flow display and async lifecycle, without copying its model.
const {test}=require('node:test');
const assert=require('node:assert/strict');
const fs=require('node:fs'),path=require('node:path'),vm=require('node:vm');
const base=path.resolve(__dirname,'../..');
const source=fs.readFileSync(path.join(base,'spheres-web/ui/cash-flow-ui.js'),'utf8');
const css=fs.readFileSync(path.join(base,'spheres-web/ui/cash-flow-ui.css'),'utf8');
const economy=fs.readFileSync(path.join(base,'spheres-web/ui/province-economy-ui.js'),'utf8');
const money=economy.slice(economy.indexOf('function economyMoney('),economy.indexOf('\n}',economy.indexOf('function economyMoney('))+2);
const plain=value=>JSON.parse(JSON.stringify(value));
const decode=value=>value.replace(/&quot;/g,'"').replace(/&#39;/g,"'").replace(/&lt;/g,'<').replace(/&gt;/g,'>').replace(/&amp;/g,'&');
const tick=()=>new Promise(resolve=>setImmediate(resolve));
function fixture(){
  const calls=[],requests=[],doc={activeElement:null},scroller={scrollTop:0};let html='';
  function control(extra={}){return {disabled:false,dataset:{},listeners:[],addEventListener(type,callback){this.listeners.push(type);this['on'+type]=callback;},focus(){if(!this.disabled)doc.activeElement=this;},...extra};}
  const mount={dataset:{cashFlowSession:'one'},attributes:{},actions:[],controls:{},
    setAttribute(key,value){this.attributes[key]=value;},get innerHTML(){return html;},
    set innerHTML(value){html=value;doc.activeElement=null;this.controls={};
      this.actions=[...value.matchAll(/<button\b([^>]*?)data-cash-flow-action="([^"]+)"([^>]*)>/g)].map(m=>control({dataset:{cashFlowAction:decode(m[2])},disabled:/\bdisabled\b/.test(m[3])}));
      for(const m of value.matchAll(/<(h1|summary|button)\b([^>]*?)data-cash-flow-focus="([^"]+)"([^>]*)>/g)){
        const key=m[3],button=control({dataset:{cashFlowFocus:key},disabled:/\bdisabled\b/.test(m[4])});
        this.controls[`[data-cash-flow-focus="${key}"]`]=button;
        if(key==='refresh'||key==='retry')this.controls[`[data-cash-flow-${key}]`]=button;
        if(key==='title')this.controls['#cashFlowTitle']=button;
      }
      const detail=/<details\b[^>]*id="cashFlowBudgetDetails"([^>]*)>/.exec(value);
      if(detail)this.controls['#cashFlowBudgetDetails']={open:/\bopen\b/.test(detail[1])};
    },querySelector(selector){return this.controls[selector]||null;},querySelectorAll(selector){return selector==='[data-cash-flow-action]'?this.actions:[];},
  };
  doc.querySelector=selector=>selector==='#cashFlowRoot'?mount:selector==='#left'?scroller:mount.querySelector(selector);
  const c=vm.createContext({console,document:doc,S:{session_id:'one',player:'USA'},CAB:{tab:'overview',busy:false,draft:{tax:.31}},queued:[],
    COMMAND_CHANNEL:{busy:false,pending:null},SESSION:{busy:false},PROD:{busy:false},COMP:{busy:false},advancing:false,pendingAdvance:null,
    cabinetIsOpen:()=>true,cashFlowDraftNotice:()=>'',cashFlowNavigate:action=>calls.push(plain(action)),
    api:async(...args)=>{requests.push(args);return reading();},
  });
  vm.runInContext(money+'\n'+source,c);c.flow=vm.runInContext('CFLOW',c);Object.assign(c.flow,{open:true,session:'one',nation:'USA'});
  Object.assign(c,{mount,scroller,calls,requests});return c;
}
function reading(extra={}){return {session_id:'one',nation:'USA',name:'United States',date:'11 Feb 1990',as_of_day:40,daily:true,on_the_books:true,
  balances:{treasury_bn:5,debt_bn:20,net_position_bn:-15,debt_gdp:.3},
  settled:{day:39,label:'10 Feb 1990',fiscal_year:1990,revenue_bn:.7,ministry_spend_bn:.5,interest_bn:.1,total_outflow_bn:.6,balance_bn:.1,
    services_bn:.3,plant_operating_bn:.08,construction_bn:.1,other_capital_bn:.02,prepaid_used_bn:.04,note:'Raw adapter implementation note'},
  annual:{fiscal_year:1990,renewed:true,basis_gdp_bn:1000,revenue_bn:90,tax_revenue_bn:80,resource_revenue_bn:10,authorized_spend_bn:100,interest_bn:3,
    total_at_full_use_bn:103,balance_at_full_use_bn:-13,posted_spending_run_rate_bn:77,posted_spending_day:39,note:'Raw adapter implementation note'},
  ministries:[{key:'industry',name:'Industry & Energy',annual_bn:12,daily_authorized_bn:.033,available_bn:.02,last_spent_bn:.011,spent_ytd_bn:.7}],
  construction:{daily_budget_bn:.05,planned_daily_bn:.02,available_bn:.04,authority_bn:.15,spent_today_bn:.1,queued_count:2,
    affordability:{status:'funded',title:'Next construction work has funding',detail:'The planned next work fits the available authority and daily limit.'}},
  alerts:[],actions:[{action:'budget',label:'Review annual budget'},{action:'construction',label:'Manage construction funding'},{action:'industry',label:'Inspect operating industry'},
    {action:'policy',label:'Review taxes and interest'},{action:'trade',label:'Review goods cash commitments'}],note:'No complete all-source daily cash journal is stored.',...extra};}
function loaded(c,data=reading()){Object.assign(c.flow,{data,state:c.S,stale:false,loading:false,error:''});return data;}
function priorities(){return [
  {id:'interest',level:'attention',title:'Interest explains the deficit',detail:'Review debt and rates alongside ministry allocations.',metrics:[{label:'Balance before interest',amount_bn:.005,period:'Settled 10 Feb 1990'},{label:'Interest paid',amount_bn:.009,period:'Settled 10 Feb 1990'}],action:{action:'policy',control:'rate'}},
  {id:'largest',level:'info',title:'Largest recorded ministry payment',detail:'This is the funding source of recorded work, not a saving estimate.',metrics:[{label:'Industry & Energy',amount_bn:.011,period:'Settled 10 Feb 1990'}],action:{action:'budget',ministry:'industry'}},
  {id:'annual',level:'attention',title:'Review the annual full-use gap',detail:'This assumes every ministry allocation is used.',metrics:[{label:'Full-use budget balance',amount_bn:-13,period:'Annual estimate'}],action:{action:'policy',control:'tax'}},
];}

test('financial priorities preserve server order, evidence amounts and periods with at most three cards',()=>{
  const c=fixture(),data=reading({priorities:[...priorities(),{id:'fourth',title:'Must remain hidden',metrics:[],action:{action:'construction'}}]});
  data.settled.primary_balance_bn=999;data.settled.interest_bn=999;loaded(c,data);const before=plain(data),html=c.cashFlowPrioritiesHtml(data);
  assert.match(html,/Financial priorities/);assert.equal((html.match(/<article /g)||[]).length,3);
  assert(html.indexOf('Interest explains the deficit')<html.indexOf('Largest recorded ministry payment'));assert(html.indexOf('Largest recorded ministry payment')<html.indexOf('Review the annual full-use gap'));
  for(const text of ['Balance before interest</dt><dd>$5m','Interest paid</dt><dd>$9m','Industry &amp; Energy</dt><dd>$11m','Full-use budget balance</dt><dd>-$13bn','Settled 10 Feb 1990','Annual estimate'])assert(html.includes(text),text);
  assert.doesNotMatch(html,/999|Must remain hidden|\$100m/,'Priority evidence is not recomputed from other readings');assert.deepEqual(plain(data),before);
  const content=c.cashFlowContentHtml();assert(content.indexOf('Revenue, spending and balance')<content.indexOf('Financial priorities'));assert(content.indexOf('Financial priorities')<content.indexOf('id="cashFlowConstructionTitle"'));
});

test('missing priorities produce no invented diagnosis and unknown evidence remains unknown',()=>{
  const c=fixture(),data=loaded(c);data.settled.balance_bn=-50;assert.equal(c.cashFlowPrioritiesHtml(data),'');data.priorities=[];assert.equal(c.cashFlowPrioritiesHtml(data),'');
  data.priorities=priorities();data.priorities[0].metrics=[{label:'Unknown receipt',amount_bn:null,period:'Settled 10 Feb 1990'},{label:'Small recorded cost',amount_bn:.00000075,period:'Settled 10 Feb 1990'}];
  const html=c.cashFlowPrioritiesHtml(data);assert.match(html,/Unknown receipt<\/dt><dd>—/);assert.match(html,/Small recorded cost<\/dt><dd>\$750/);assert.doesNotMatch(html,/\$0|50bn/);
});

test('priority buttons navigate to the exact server control and reject stale or replaced readings',()=>{
  const c=fixture();loaded(c,reading({priorities:priorities()}));c.cashFlowRender();
  const rate=c.mount.actions.find(button=>JSON.parse(button.dataset.cashFlowAction).control==='rate');assert(rate);assert.equal(rate.onclick(),true);assert.deepEqual(c.calls,[{action:'policy',control:'rate'}]);
  assert.match(c.mount.innerHTML,/Review debt and rates/);assert.match(c.mount.innerHTML,/Review tax policy/);assert.doesNotMatch(c.mount.innerHTML,/Cut rates|Lower taxes/);
  c.flow.stale=true;assert.equal(rate.onclick(),false);c.flow.stale=false;c.flow.data=reading({priorities:priorities()});assert.equal(rate.onclick(),false);
  c.cashFlowRender();c.pendingAdvance={};const replacement=c.mount.actions.find(button=>JSON.parse(button.dataset.cashFlowAction).control==='tax');assert.equal(replacement.onclick(),false);assert.equal(c.calls.length,1);assert.equal(c.requests.length,0);
});

test('annual ministry rows expose exact ministry review actions without adding a table column',()=>{
  const c=fixture(),data=reading();data.ministries.push({...data.ministries[0],key:'science',name:'Science'});loaded(c,data);
  const html=c.cashFlowAnnualHtml(data);assert.match(html,/Review Industry &amp; Energy/);assert.match(html,/Review Science/);assert.equal((html.match(/<th scope="col">/g)||[]).length,6);
  c.cashFlowRender();const science=c.mount.actions.find(button=>JSON.parse(button.dataset.cashFlowAction).ministry==='science');assert.equal(science.onclick(),true);assert.deepEqual(c.calls,[{action:'budget',ministry:'science'}]);
  c.flow.stale=true;assert.match(c.cashFlowAnnualHtml(data),/ministry&quot;:&quot;science&quot;}" disabled/);
});

test('priority text and periods are escaped and only exactly duplicated alerts are removed',()=>{
  const c=fixture(),data=reading({priorities:priorities()});data.priorities[0].title='<img>';data.priorities[0].detail='<script>';data.priorities[0].metrics[0].period='<svg>';
  data.alerts=[{title:'<img>',detail:'<script>'},{title:'<img>',detail:'A separate activation warning'}];loaded(c,data);const html=c.cashFlowContentHtml();
  assert.doesNotMatch(html,/<img|<script|<svg/);assert.match(html,/&lt;svg&gt;/);assert.equal(html.split('&lt;script&gt;').length-1,1);assert.match(html,/A separate activation warning/);
});

test('only alerts represented by a displayed priority ID are suppressed',()=>{
  const c=fixture(),data=reading({priorities:[...priorities(),{id:'renew_budget',title:'Renew budget',metrics:[]}]});
  data.alerts=[{id:'annual',title:'Annual alert',detail:'The annual gap is already explained.'},
    {id:'renew_budget',title:'Annual renewal still needed',detail:'This priority was outside the three-card limit.'},
    {id:'open_books',title:'Treasury books are not open',detail:'Review the first budget.'}];
  loaded(c,data);const html=c.cashFlowContentHtml();
  assert.doesNotMatch(html,/Annual alert/);assert.match(html,/Annual renewal still needed/);assert.match(html,/Treasury books are not open/);
});

test('dated government spending is separate from current balances and the enacted annual plan',()=>{
  const c=fixture(),data=loaded(c),before=plain(data),html=c.cashFlowContentHtml();
  for(const text of ['Your cash flow','Reading for 11 Feb 1990','Latest daily government budget','10 Feb 1990','Budget outflow','Budget balance','Industry and research operations','Treasury cash','Public debt','Cash minus debt','Enacted annual plan','Fiscal year 1990','Available authorization'])assert(html.includes(text),text);
  assert.match(html,/Treasury cash<\/dt><dd>\$5bn/);assert.match(html,/Public debt<\/dt><dd>\$20bn/);
  assert.match(html,/Budget outflow<\/dt><dd>\$600m/);assert.match(html,/Budget balance<\/dt><dd>\+\$100m/);
  assert.match(html,/Total annual cost at full use<\/dt><dd>\$103bn/);assert.match(html,/Annual balance at full use<\/dt><dd>-\$13bn/);
  assert.match(html,/Trade, transfers and other transactions can also change cash and debt/);assert.match(html,/surplus repays debt before building treasury cash/);
  assert.doesNotMatch(html,/journal|Raw adapter|cash runway|Actual daily cash flow|Total cash outflow/);
  assert.deepEqual(plain(data),before);assert.equal(c.requests.length,0);
});

test('server totals and net position are displayed unchanged, never reconstructed from component sums',()=>{
  const c=fixture(),data=reading();data.settled.total_outflow_bn=.123;data.settled.balance_bn=.456;data.balances.net_position_bn=-7;
  data.annual.total_at_full_use_bn=107;data.annual.balance_at_full_use_bn=-8;loaded(c,data);const html=c.cashFlowContentHtml();
  assert.match(html,/Budget outflow<\/dt><dd>\$123m/);assert.match(html,/Budget balance<\/dt><dd>\+\$456m/);
  assert.match(html,/Cash minus debt<\/dt><dd>-\$7bn/);assert.match(html,/Total annual cost at full use<\/dt><dd>\$107bn/);
  assert.match(html,/Annual balance at full use<\/dt><dd>-\$8bn/);
});

test('construction and prepaid usage are disclosed once without adding a second cash charge',()=>{
  const c=fixture(),data=loaded(c),html=c.cashFlowSettlementHtml(data);
  assert.match(html,/Construction work<\/dt><dd>\$100m/);assert.match(html,/Ministry spending total<\/dt><dd>\$500m/);
  assert.match(html,/Budget outflow<\/dt><dd>\$600m/);assert.match(html,/Previously paid funds used<\/dt><dd>\$40m/);
  assert.match(html,/Paid earlier; this usage is not another cash outflow/);assert.match(html,/components are included in budget outflow/);
  assert.doesNotMatch(html,/Budget outflow<\/dt><dd>\$(?:640|700|740)m/);
});

test('income split belongs only to current annual estimates, with the oil fiscal source named precisely',()=>{
  const c=fixture(),data=loaded(c),annual=c.cashFlowAnnualHtml(data),daily=c.cashFlowSettlementHtml(data);
  assert.match(annual,/Annual tax income estimate<\/dt><dd>\$80bn/);assert.match(annual,/Oil-related fiscal revenue<\/dt><dd>\$10bn/);
  assert.match(annual,/daily receipt above reports total revenue without an income split/);assert.match(annual,/annualizes one settled day/);
  assert.doesNotMatch(daily,/tax income|Oil-related|\$80bn/);
  data.annual.tax_revenue_bn=null;data.annual.resource_revenue_bn=null;
  assert.doesNotMatch(c.cashFlowAnnualHtml(data),/Annual tax income estimate<\/dt>|Oil-related fiscal revenue<\/dt>/);
});

test('missing receipts and unknown balances or actual expenses never become fabricated zeroes',()=>{
  const c=fixture(),data=reading({settled:null,balances:{treasury_bn:null,debt_bn:null,net_position_bn:null},on_the_books:false});
  Object.assign(data.annual,{renewed:false,interest_bn:null,total_at_full_use_bn:null,balance_at_full_use_bn:null,posted_spending_run_rate_bn:null});
  Object.assign(data.ministries[0],{last_spent_bn:null,spent_ytd_bn:null});loaded(c,data);const html=c.cashFlowContentHtml();
  assert.match(html,/No settled daily receipt yet/);assert.match(html,/Treasury cash<\/dt><dd>—/);assert.match(html,/Annual interest estimate<\/dt><dd>—/);
  assert.match(html,/<td>—<\/td><td>—<\/td>/);assert.doesNotMatch(c.cashFlowSettlementHtml(data),/\$0/);
  assert.match(html,/Annual plan & ministry spending/);assert.doesNotMatch(html,/enacted plan|Enacted annual plan/);
  assert.equal(c.cashFlowMoney(null),'—');assert.equal(c.cashFlowMoney(NaN),'—');assert.equal(c.cashFlowMoney(0),'$0');assert.equal(c.cashFlowMoney(.00000075),'$750');
});

test('construction affordability comes from the server with authority distinct from cash',()=>{
  const c=fixture(),data=loaded(c),before=plain(data.construction),html=c.cashFlowConstructionHtml(data);
  for(const text of ['Next construction work has funding','The planned next work fits the available authority and daily limit.','Daily spending limit','Planned construction','Available for work','This is budget authorization, not treasury cash','Unused daily construction funding is not charged'])assert(html.includes(text),text);
  assert.match(html,/Daily spending limit<\/dt><dd>\$50m/);assert.match(html,/Planned construction<\/dt><dd>\$20m/);assert.match(html,/Available for work<\/dt><dd>\$40m/);
  assert.deepEqual(plain(data.construction),before);
  data.construction.affordability={status:'unfunded',title:'No construction funds are available',detail:'Review annual authority.'};
  data.alerts=[{title:'No construction funds are available',detail:'Review annual authority.',action:'construction'}];
  assert.equal(c.cashFlowContentHtml().split('No construction funds are available').length-1,1,'Funding alert is not duplicated beside the same explanation');
});

test('navigation is concise, strictly read-only, and retains exact ministry targets',()=>{
  const c=fixture();loaded(c);const html=c.cashFlowContentHtml();
  assert.doesNotMatch(html,/Manage construction funding|Inspect operating industry/);assert.match(html,/Review goods cash commitments/);
  assert.equal(c.cashFlowNavigateCurrent({action:'budget',ministry:'industry',department:2,command:'spend',amount:999}),true);
  assert.deepEqual(c.calls,[{action:'budget',ministry:'industry',department:2}]);
  for(const action of [{action:'start_project'},{action:'trade',enabled:false},{action:'budget',available:false}])assert.equal(c.cashFlowNavigateCurrent(action),false);
  assert.equal(c.cashFlowButton('start_project'),'');assert.equal(c.requests.length,0);
});

test('draft notice updates without writes, requests, discarded edits or a false navigation lock',()=>{
  const c=fixture();loaded(c);c.cashFlowRender();const before=plain(c.CAB.draft);
  c.queued.push({type:'annual_budget',payload:{tax:.31}});c.cashFlowDraftNotice=()=> 'Budget edits are queued. This reading still uses the enacted plan.';
  c.cashFlowDraftChanged();assert.match(c.mount.innerHTML,/Budget edits are queued/);assert.equal(c.cashFlowCurrent(),true);
  assert.equal(c.cashFlowNavigateCurrent({action:'budget'}),true);assert.deepEqual(plain(c.CAB.draft),before);assert.equal(c.queued.length,1);assert.equal(c.requests.length,0);
});

test('stale readings and pending orders cannot navigate, while captured actions reject replaced payloads',()=>{
  for(const change of [c=>c.flow.stale=true,c=>c.flow.loading=true,c=>c.flow.error='failed',c=>c.advancing=true,c=>c.pendingAdvance={},c=>c.COMMAND_CHANNEL.busy=true,c=>c.COMMAND_CHANNEL.pending={},c=>c.SESSION.busy=true,c=>c.CAB.busy=true,c=>c.PROD.busy=true,c=>c.COMP.pending={},c=>c.CAB.tab='industry',c=>c.flow.open=false,c=>c.S={session_id:'one',player:'USA'}]){
    const c=fixture();loaded(c);change(c);assert.equal(c.cashFlowNavigateCurrent('budget'),false);assert.equal(c.calls.length,0);assert.match(c.cashFlowButton('budget'),/disabled/);
  }
  const c=fixture();loaded(c);c.cashFlowRender();const button=c.mount.actions[0];c.flow.data=reading();
  assert.equal(button.onclick(),false);assert.equal(c.calls.length,0);c.cashFlowRender();assert.equal(c.mount.actions[0].onclick(),true);
});

test('GET is session-bound and accepted readings belong to the exact world object',async()=>{
  const c=fixture();assert.equal(await c.cashFlowFetch(),true);assert.deepEqual(c.requests,[['/api/cash-flow?session_id=one']]);
  assert.equal(c.flow.state,c.S);assert.equal(c.cashFlowCurrent(),true);
  for(const bad of [reading({session_id:'other'}),reading({nation:'Canada'}),reading({ministries:null}),reading({annual:null})]){
    c.api=async()=>bad;assert.equal(await c.cashFlowFetch(true),false);assert.match(c.flow.error,/did not match this campaign/);assert.equal(c.cashFlowCurrent(),false);
  }
  const escaped=fixture();escaped.S.session_id='a &b';escaped.api=async(...args)=>{escaped.requests.push(args);return reading({session_id:'a &b'});};
  assert.equal(await escaped.cashFlowFetch(),true);assert.equal(escaped.requests[0][0],'/api/cash-flow?session_id=a%20%26b');
});

test('late responses cannot survive state adoption, a closed Cabinet, another tab or a newer request',async()=>{
  for(const change of [c=>c.S={session_id:'one',player:'USA'},c=>c.cashFlowClose(),c=>c.CAB.tab='industry',c=>c.cabinetIsOpen=()=>false]){
    const c=fixture();let resolve;c.api=()=>new Promise(r=>resolve=r);const pending=c.cashFlowFetch();change(c);resolve(reading());
    assert.equal(await pending,false);assert.equal(c.flow.data,null);
  }
  const c=fixture(),resolvers=[];c.api=()=>new Promise(r=>resolvers.push(r));
  const first=c.cashFlowFetch();c.S={session_id:'one',player:'USA'};c.cashFlowOnStateChanged();assert.equal(resolvers.length,2);
  resolvers[1](reading({name:'Latest'}));await tick();resolvers[0](reading({name:'Old'}));await first;
  assert.equal(c.flow.data.name,'Latest');assert.equal(c.flow.state,c.S);
});

test('new campaign immediately removes old nation balances and view state before its read completes',async()=>{
  const c=fixture();loaded(c);c.cashFlowRender();c.scroller.scrollTop=700;c.mount.querySelector('#cashFlowBudgetDetails').open=true;c.cashFlowRememberView();
  c.S={session_id:'two',player:'Canada'};let resolve;c.api=()=>new Promise(r=>resolve=r);c.cashFlowOnStateChanged();
  assert.equal(c.flow.data,null);assert.equal(c.flow.session,'two');assert.equal(c.flow.nation,'Canada');assert.equal(c.flow.detailOpen,false);assert.equal(c.flow.scroll,0);
  assert.doesNotMatch(c.mount.innerHTML,/United States|\$20bn/);resolve(reading({session_id:'two',nation:'Canada',name:'Canada'}));await tick();
  assert.equal(c.flow.data.name,'Canada');assert.equal(c.cashFlowCurrent(),true);
});

test('failed refresh retains a dated disabled reading and exposes a working explicit retry',async()=>{
  const c=fixture();loaded(c);c.api=async()=>{throw new Error('Connection lost');};assert.equal(await c.cashFlowFetch(true),false);
  assert.match(c.mount.innerHTML,/Cash flow could not be refreshed/);assert.match(c.mount.innerHTML,/Connection lost/);assert.match(c.mount.innerHTML,/previous dated reading/);assert.equal(c.cashFlowCurrent(),false);
  c.cashFlowBind();assert.equal(c.flow.loading,false,'Failure must not produce an automatic retry loop');
  c.api=async()=>reading();c.mount.querySelector('[data-cash-flow-retry]').onclick();await tick();assert.equal(c.flow.error,'');assert.equal(c.cashFlowCurrent(),true);
});

test('expanded annual plan, scroll and keyboard focus survive repaint but hidden tabs cannot overwrite them',()=>{
  const c=fixture();loaded(c);c.cashFlowRender();c.scroller.scrollTop=430;
  const details=c.mount.querySelector('#cashFlowBudgetDetails');details.open=true;details.ontoggle();c.mount.querySelector('[data-cash-flow-focus="budget"]').focus();c.cashFlowRender();
  assert.equal(c.mount.querySelector('#cashFlowBudgetDetails').open,true);assert.equal(c.scroller.scrollTop,430);assert.equal(c.document.activeElement,c.mount.querySelector('[data-cash-flow-focus="budget"]'));
  c.cashFlowClose();c.CAB.tab='budget';c.scroller.scrollTop=17;c.cashFlowPanelHtml();assert.equal(c.flow.scroll,430);
  c.cashFlowOnStateChanged();assert.equal(c.requests.length,0);c.cashFlowBind();assert.equal(c.requests.length,0);
});

test('refresh restores keyboard focus after success and failure without duplicate event binding',async()=>{
  for(const failed of [false,true]){
    const c=fixture();loaded(c);c.cashFlowRender();c.cashFlowBind(false);c.cashFlowBind(false);
    const button=c.mount.querySelector('[data-cash-flow-refresh]');assert.equal(button.listeners.length,0);
    let resolve,reject,reads=0;c.api=()=>{reads++;return new Promise((a,b)=>{resolve=a;reject=b;});};
    button.focus();button.onclick();assert.equal(reads,1);assert.equal(c.flow.loading,true);assert.equal(c.mount.querySelector('[data-cash-flow-refresh]').disabled,true);
    assert.equal(c.flow.focus,'refresh');if(failed)reject(new Error('Offline'));else resolve(reading());await tick();
    assert.equal(c.document.activeElement,c.mount.querySelector('[data-cash-flow-refresh]'));assert.equal(c.flow.focus,null);
  }
});

test('explicit cash-flow reveal resets deep overview scroll and keeps title focused after async refresh',async()=>{
  const c=fixture();loaded(c);c.cashFlowRender();c.scroller.scrollTop=1500;c.cashFlowRememberView();
  let resolve;c.api=()=>new Promise(r=>resolve=r);const pending=c.cashFlowFetch(true);c.cashFlowReveal();
  assert.equal(c.flow.scroll,0);assert.equal(c.scroller.scrollTop,0);assert.equal(c.document.activeElement,c.mount.querySelector('#cashFlowTitle'));
  resolve(reading());await pending;assert.equal(c.scroller.scrollTop,0);assert.equal(c.flow.scroll,0);assert.equal(c.document.activeElement,c.mount.querySelector('#cashFlowTitle'));
});

test('ordinary tab activation makes one request and never polls while closed',async()=>{
  const c=fixture();c.flow.open=false;c.cashFlowTabChanged();c.cashFlowBind();await tick();assert.equal(c.requests.length,1);
  c.CAB.tab='industry';c.cashFlowTabChanged();assert.equal(c.flow.open,false);c.cashFlowOnStateChanged();assert.equal(c.requests.length,1);
  c.CAB.tab='overview';c.cabinetIsOpen=()=>false;c.cashFlowTabChanged();assert.equal(c.requests.length,1);
  c.cabinetIsOpen=()=>true;c.cashFlowTabChanged();await tick();assert.equal(c.requests.length,2);assert.equal(c.cashFlowCurrent(),true);
});

test('external text and attributes are escaped, and standalone assets parse with scoped responsive styles',()=>{
  const c=fixture();loaded(c,reading({name:'<script>',date:'<img>',alerts:[{title:'<svg>',detail:'<iframe>'}]}));
  const html=c.cashFlowContentHtml();assert.doesNotMatch(html,/<script|<img|<svg|<iframe/);assert.match(html,/&lt;script&gt;/);
  assert.match(c.cashFlowButton({action:'budget',label:'<img>',ministry:'" onclick="bad()'}),/&quot;/);
  new vm.Script(source);assert.match(css,/\.cash-flow/);assert.match(css,/@media\(max-width:530px\)/);assert.match(css,/focus-visible/);
  assert.doesNotMatch(source,/\/api\/command|start_project|spend_construction/);
});
