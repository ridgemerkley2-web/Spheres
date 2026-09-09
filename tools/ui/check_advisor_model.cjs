const test=require('node:test'),assert=require('node:assert/strict');
const fs=require('node:fs'),path=require('node:path'),vm=require('node:vm');
const advisor=require('../../spheres-web/ui/advisor-model.js');
function state(){return {player:'Japan',year:1990,month:1,day:1,simulation_cadence:'daily',nations:[{id:'Japan',name:'Japan',alive:true,treasury:null,annual_budget:{due:false,fiscal_year:1990}},{id:'USA',name:'United States',alive:true},{id:'Canada',name:'Canada',alive:true}],programs:{enabled:true,due:false},wars:[]};}
function works(){return {mode:'province_projects',nation:'Japan',suggestions:{as_of_day:0,items:[]},construction_budget:{enrolled:true,daily_budget_bn:.01,available_bn:.02},queue:[],mine_queue:[],completed:[],catalog:[{kind:'power_grid',actions:{start:true},eligible_provinces:['JP-13'],start_reason:null}],provinces:[{id:'JP-13',name:'Tokyo',actions:{start:['power_grid']},start_refusals:{}}]};}
function suggestion(){return {id:'power_grid:JP-13:0',project_kind:'power_grid',district:'JP-13',district_name:'Tokyo',capacity_micros:null,name:'Power grid',reason:'The province has an operating grid bottleneck.',evidence:['Served grid reading.'],cost_bn:.05,minimum_days:90,eta_days:120};}
function domain(overrides={}){return {domain:'Computing',name:'Computing',project:null,wait:'none',banked:0,cost:0,rate:1,options:[],...overrides};}
function research(s,domains){s.research={nation:'Japan',priority:null,monthly:1,domains};return s;}
function get(s,w,id){return advisor.evaluate(s,w).find(c=>c.id===id);}
function freeze(value){if(value&&typeof value==='object'){Object.values(value).forEach(freeze);Object.freeze(value);}return value;}

test('empty, malformed and observer snapshots cannot masquerade as a healthy player',()=>{
  for(const s of [undefined,null,[],{},state().nations,{...state(),player:null},{...state(),nations:null},{...state(),nations:[{id:'Japan',alive:false}]},{...state(),nations:[{id:'Japan',alive:true},{id:'Japan',alive:true}]}])assert.deepEqual(advisor.evaluate(s),[]);
  assert.deepEqual(advisor.evaluate(state()),[]);
});
test('real annual renewal takes priority and null treasury does not become zero',()=>{
  const s=state();s.programs.due=true;const w=works();w.construction_budget.enrolled=false;
  const cards=advisor.evaluate(s,w);assert.equal(cards[0].id,'annual-budget');assert(cards.some(c=>c.id==='construction-enrollment'));assert(!cards.some(c=>c.id==='treasury-balance'));
  s.programs.due='true';assert(!get(s,w,'annual-budget'));
  s.nations[0].annual_budget.due=true;assert(get(s,w,'annual-budget'));
});
test('treasury accepts only actual finite numbers and invents no low-cash threshold',()=>{
  for(const value of [null,undefined,'0','-3',NaN,Infinity,-Infinity,false,{},.00001,100]){const s=state();s.nations[0].treasury=value;assert(!get(s,null,'treasury-balance'),String(value));}
  for(const value of [0,-3]){const s=state();s.nations[0].treasury=value;assert.match(get(s,null,'treasury-balance').evidence[0],/Treasury: \$/);}
});
test('construction requires a matching dated production snapshot and valid calendar',()=>{
  const s=state(),w=works();w.construction_budget.daily_budget_bn=0;
  assert(get(s,w,'construction-paused'));
  for(const bad of [{...w,nation:'USA'},{...w,mode:'legacy'},{...w,suggestions:{...w.suggestions,as_of_day:1}},{...w,suggestions:{}},{}])assert.deepEqual(advisor.evaluate(s,bad),[]);
  for(const bad of [{...s,year:'1990'},{...s,year:1e50},{...s,month:13},{...s,month:2,day:30}])assert.deepEqual(advisor.evaluate(bad,w),[]);
  const leap={...s,year:1992,month:2,day:29};w.suggestions.as_of_day=(Date.UTC(1992,1,29)-Date.UTC(1990,0,1))/86400000;assert(get(leap,w,'construction-paused'));
});
test('funding and blocked projects precede recommendations without inventing blocked causes',()=>{
  const s=state(),w=works();w.suggestions.items=[suggestion()];w.queue=[{id:7,name:'Grid',status:'blocked',reason:'The province is contested.'}];
  let cards=advisor.evaluate(s,w);assert.equal(cards[0].id,'construction-project');assert.equal(cards[0].reason,'The province is contested.');assert.deepEqual(cards[0].action,{kind:'project',id:7});assert(!cards.some(c=>c.action.kind==='suggestion'));
  w.construction_budget.daily_budget_bn=0;cards=advisor.evaluate(s,w);assert.equal(cards[0].id,'construction-paused');
  w.construction_budget.daily_budget_bn=.1;w.construction_budget.available_bn=0;assert.equal(advisor.evaluate(s,w)[0].id,'construction-cash');
  w.queue=[{id:'oil:JP-13',status:'blocked'}];assert.deepEqual(get(s,w,'construction-project').action,{kind:'construction'});
});
test('only exact eligible suggestion evidence and province permissions permit a review',()=>{
  const s=state(),w=works();w.suggestions.items=[suggestion()];assert.deepEqual(get(s,w,'construction-suggestion').action,{kind:'suggestion',project_kind:'power_grid',district:'JP-13',capacity_micros:null});
  const edits=[i=>i.eligible=false,i=>i.can_start='true',i=>i.cost_bn=null,i=>i.cost_bn=Infinity,i=>i.minimum_days='90',i=>i.minimum_days=0,i=>i.capacity_micros=50,i=>i.evidence=[],i=>i.project_kind='invented',i=>i.district='US-CA'];
  for(const edit of edits){const bad=works(),item=suggestion();edit(item);bad.suggestions.items=[item];assert(!get(s,bad,'construction-suggestion'));}
  for(const edit of [v=>v.catalog[0].actions.start=false,v=>v.catalog[0].eligible_provinces=[],v=>v.catalog[0].start_reason='Denied',v=>v.provinces[0].actions.start=[],v=>v.provinces[0].start_refusals.power_grid='Denied',v=>v.provinces.push({...v.provinces[0]})]){const bad=works();bad.suggestions.items=[suggestion()];edit(bad);assert(!get(s,bad,'construction-suggestion'));}
});
test('sized workshop uses the server exact-size preview and preserves server order',()=>{
  const s=state(),w=works(),small={...suggestion(),id:'starter_industry:JP-13:5001',project_kind:'starter_industry',capacity_micros:5001};
  w.suggestions.items=[{...suggestion(),eligible:false},small,suggestion()];w.provinces[0].actions.start=[];
  assert.equal(get(s,w,'construction-suggestion').action.capacity_micros,5001);
  for(const size of [0,-1,1000001,1.5,'5001',NaN,null]){w.suggestions.items=[{...small,capacity_micros:size}];assert(!get(s,w,'construction-suggestion'));}
  w.suggestions.items=[small];s.simulation_cadence='monthly';assert(!get(s,w,'construction-suggestion'));
});
test('bad funding numbers cannot support a new building recommendation',()=>{
  for(const value of [null,undefined,'0.02',NaN,Infinity,-1]){const w=works();w.suggestions.items=[suggestion()];w.construction_budget.available_bn=value;assert(!get(state(),w,'construction-suggestion'));}
});
test('optional industry allocation zero is distinct from cash and blocks expansion advice',()=>{
  const s=state(),w=works();w.suggestions.items=[suggestion()];w.queue=[{id:8,status:'building',progress:.2,allocation:{assigned:0,requested:3}}];w.industry_rebuild={enabled:true,capacity:{idle_capacity:0}};
  const card=get(s,w,'construction-allocation');assert(card);assert.match(card.caution,/separate from the cash budget/);assert(!get(s,w,'construction-cash'));assert(!get(s,w,'construction-suggestion'));
  w.industry_rebuild.enabled=false;assert(!get(s,w,'construction-allocation'));assert(get(s,w,'construction-suggestion'));
  w.industry_rebuild.enabled=true;w.queue[0].allocation.assigned='0';assert(!get(s,w,'construction-allocation'));
});
test('building status is not a guarantee and completed facilities retain the industry destination',()=>{
  const w=works();w.queue=[{id:0,status:'building',progress:.05,name:'Grid'}];const c=get(state(),w,'construction-progress');assert.equal(c.evidence[1],'Recorded progress: 5%.');assert.match(c.caution,/can change/);
  w.queue=[{id:0,status:'building',progress:null}];assert(!get(state(),w,'construction-progress'));
  w.queue=[];w.completed=[{province:{id:'JP-13'}}];assert.equal(get(state(),w,'completed-industry').action.kind,'industry');
});
test('unavailable construction details can use only explicit state attention',()=>{
  const s=state();s.production_summary={attention:2,attention_ids:[9]};const c=advisor.evaluate(s)[0];assert.equal(c.id,'construction-summary');assert.match(c.caution,/unavailable/);assert.deepEqual(c.action,{kind:'project',id:9});
  s.production_summary.attention='2';assert.deepEqual(advisor.evaluate(s),[]);
});
test('government warnings use open server routes, not stability thresholds or aggregate disabled routes',()=>{
  const s=state();s.nations[0].stability=1;s.nations[0].takeover={half_armed:true,coup:{open:false,armed:true},round_table:{open:false,half_armed:true,reason:'calibration pending'}};
  assert.deepEqual(advisor.evaluate(s),[]);
  s.nations[0].takeover.coup={open:true,armed:true,gauges:[{name:'Served condition',met:true,value:1,trigger:2},{name:'Unmet',met:false}]};const c=advisor.evaluate(s)[0];assert.equal(c.id,'government-coup');assert.deepEqual(c.evidence,['Reported condition met: Served condition.']);
  s.nations[0].takeover.coup.armed='true';assert.deepEqual(advisor.evaluate(s),[]);
});
test('research options in the future are not presented as current projects',()=>{
  const s=research(state(),[domain({options:[{id:'future',name:'Future computing',year:2000}]})]);assert(!get(s,null,'research-project'));
  s.research.domains[0].options.push({id:'now',name:'Current computing',year:1990});const c=get(s,null,'research-project');assert.match(c.reason,/Current computing/);assert.doesNotMatch(c.reason,/Future computing/);
  s.research.nation='United States';assert.deepEqual(advisor.evaluate(s),[]);
});
test('a paid calendar wait is never a funding stall',()=>{
  const s=research(state(),[domain({project:{id:'f',name:'Future project',year:2000},wait:'year',banked:10,cost:10,rate:0})]);
  const c=get(s,null,'research-calendar');assert.equal(c.priority,'routine');assert.match(c.reason,/2000/);assert.match(c.caution,/does not bring/);assert(!get(s,null,'research-stalled'));
  s.research.domains[0].banked=null;assert(!get(s,null,'research-calendar'));
});
test('research funding stall requires unpaid effort and actual zero rate',()=>{
  const s=research(state(),[domain({project:{id:'p',name:'Project',year:1990},wait:'stalled',banked:1,cost:10,rate:0})]);assert(get(s,null,'research-stalled'));
  for(const rate of [null,'0',NaN,Infinity,-1,1]){s.research.domains[0].rate=rate;assert(!get(s,null,'research-stalled'));}
  s.research.domains[0].rate=0;s.research.domains[0].banked=10;assert(!get(s,null,'research-stalled'));
});
test('monthly acquisition allowance and priority are separate research reviews',()=>{
  const s=research(state(),[domain({project:{id:'p',name:'Project',year:1990},wait:'funded',banked:10,cost:10,acquisition_wait:true,acquisitions_remaining:0})]);
  assert.equal(get(s,null,'research-quota').priority,'routine');assert(get(s,null,'research-focus'));assert(!get(s,null,'research-stalled'));
  s.research.monthly=null;assert(!get(s,null,'research-focus'));
});
test('disjoint enemy wars and foreign operations do not become player conflicts',()=>{
  const s=state();s.wars=[{id:1,posture:[{id:'USA'},{id:'Canada'}],class:'Conventional'}];s.operations={enabled:true,deployments:[{conflict:1,nation:'USA',requested:10,deployed:0},{conflict:99,nation:'Japan',requested:10,deployed:0}]};assert.deepEqual(advisor.evaluate(s),[]);
  s.wars.push({id:2,theatre_name:'Korea',class:'Stand-off',posture:[{id:'Japan'},{id:'USA'}]});assert(get(s,null,'military-conflicts'));assert(!get(s,null,'military-deployment'));
});
test('deployment advice requires an active player conflict and valid requested/deployed values',()=>{
  const s=state();s.wars=[{id:1,posture:[{id:'Japan'},{id:'USA'}]}];s.operations={enabled:true,deployments:[{conflict:1,nation:'Japan',requested:10,deployed:4}]};assert(get(s,null,'military-deployment'));
  for(const value of [null,'4',NaN,-1,11]){s.operations.deployments[0].deployed=value;assert(!get(s,null,'military-deployment'));assert(get(s,null,'military-conflicts'));}
});
test('incoming agency requests are distinct from possible action menus and are deadline checked',()=>{
  const s=state(),offer={id:1,from:'USA',from_name:'United States',title:'Trade treaty requested',expires:'1990-01-03',days_remaining:2,consequence:'A trade treaty would be signed.',accept_blocked:null};
  s.stratagems={offers:[offer]};s.domination={offers:[offer]};assert.deepEqual(advisor.evaluate(s),[]);
  s.agency={offers:[offer]};assert.equal(get(s,null,'diplomacy-request').priority,'opportunity');assert.equal(get(s,null,'diplomacy-request').action.kind,'decisions');
  s.agency.offers=[{...offer,expires:'1990-01-01',days_remaining:0}];assert.equal(get(s,null,'diplomacy-request').priority,'attention');
  for(const bad of [{...offer,from:'Japan'},{...offer,from:'Unknown'},{...offer,id:'1'},{...offer,expires:'1989-12-31'},{...offer,expires:'1990-02-30'},{...offer,days_remaining:null},{...offer,days_remaining:-1},{...offer,days_remaining:0}]){s.agency.offers=[bad];assert(!get(s,null,'diplomacy-request'));}
});
test('incoming resource offer requires counterpart, deadline and an actual political cost',()=>{
  const s=state(),offer={id:3,from:'United States',from_id:'USA',legs:'Oil for capital goods',expires_in_days:3,accept_pc:2};s.resources={offers:[offer]};assert.equal(get(s,null,'diplomacy-resource-offer').action.kind,'resources');
  for(const bad of [{...offer,accept_pc:null},{...offer,expires_in_days:-1},{...offer,from_id:'Japan'},{...offer,from_id:'Unknown'}]){s.resources.offers=[bad];assert(!get(s,null,'diplomacy-resource-offer'));}
});
test('advice is deterministic, immutable, bounded and attention is always first',()=>{
  const s=research(state(),[domain({options:[{id:'a',name:'Research',year:1990}]})]),w=works();s.programs.due=true;s.nations[0].treasury=-3;
  s.nations[0].takeover=Object.fromEntries(['coup','uprising','round_table','collapse'].map(k=>[k,{open:true,armed:true,gauges:[]} ]));w.queue=[{id:4,status:'blocked'}];w.construction_budget.daily_budget_bn=0;s.manufacturing_summary={attention:2};s.wars=[{id:1,posture:[{id:'Japan'}]}];
  s.agency={offers:[{id:1,from:'USA',title:'Request',expires:'1990-01-01',days_remaining:0}]};s.resources={offers:[{id:2,from_id:'USA',legs:'Trade',expires_in_days:2,accept_pc:0}]};
  const before=JSON.stringify([s,w]);freeze(s);freeze(w);const a=advisor.evaluate(s,w),b=advisor.evaluate(s,w);assert.deepEqual(a,b);assert.equal(JSON.stringify([s,w]),before);assert(a.length<=12);assert.equal(new Set(a.map(c=>c.id)).size,a.length);
  const priorities=a.map(c=>({attention:0,opportunity:1,routine:2})[c.priority]);assert.deepEqual(priorities,[...priorities].sort());
  for(const c of a){assert.deepEqual(Object.keys(c),['id','area','priority','title','reason','evidence','caution','action','actionLabel']);assert(c.evidence.length<=5);assert(['economy','government','research','military','diplomacy'].includes(c.area));assert(['budget','construction','project','suggestion','industry','government','research','equipment','world','resources','decisions'].includes(c.action.kind));}
  a[0].action.kind='tampered';assert.equal(advisor.evaluate(s,w)[0].action.kind,'budget');
});
test('browser UMD loads without DOM, timers, network, commands or global state',()=>{
  const context={};vm.runInNewContext(fs.readFileSync(path.join(__dirname,'../../spheres-web/ui/advisor-model.js'),'utf8'),context);assert.equal(typeof context.AdvisorModel.evaluate,'function');assert.equal(context.AdvisorModel.evaluate(state()).length,0);
});

const fixture=path.resolve(__dirname,'../../../leadership-2035-evidence/source-state-before.json');
test('archived real API reading identifies renewal without disabled takeover alarms',{skip:!fs.existsSync(fixture)},()=>{
  const s=JSON.parse(fs.readFileSync(fixture,'utf8')),cards=advisor.evaluate(s);assert(cards.some(c=>c.id==='annual-budget'));assert(!cards.some(c=>c.id.startsWith('government-')));assert(!cards.some(c=>c.id==='treasury-balance'));assert(cards.some(c=>c.id==='research-project'));
});
