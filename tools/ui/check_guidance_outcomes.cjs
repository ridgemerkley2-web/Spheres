'use strict';
const test=require('node:test'),assert=require('node:assert/strict');
const fs=require('node:fs'),path=require('node:path'),vm=require('node:vm');
const advisor=require('../../spheres-web/ui/advisor-model.js');
const ORDER=['finances','construction','research_design','procurement','air_force','save_resume'];
const KINDS=['budget','cash_flow','construction','project','industry','research','equipment','companies','air','campaign'];
const MON=['Jan','Feb','Mar','Apr','May','Jun','Jul','Aug','Sep','Oct','Nov','Dec'];
const DAY=(y,m,d)=>(Date.UTC(y,m-1,d)-Date.UTC(1990,0,1))/86400000;
const iso=day=>new Date(Date.UTC(1990,0,1)+day*86400000).toISOString().slice(0,10);
const NONE=Object.freeze({saves:Object.freeze([]),loads:Object.freeze([])});
// A fresh browser campaign in the served shapes: main.rs state/production and guidance_outcomes.rs outcomes.
function reading(y=1990,m=6,d=1){const day=DAY(y,m,d),date=`${d} ${MON[m-1]} ${y}`;return {
  state:{player:'FRA',player_name:'France',session_id:'4242-1-1',date,year:y,month:m,day:d,t:(y-1990)*12+m-1,simulation_cadence:'daily',
    nations:[{id:'FRA',name:'France',alive:true,annual_budget:{due:false,fiscal_year:y}},{id:'USA',name:'United States',alive:true}],programs:{enabled:true,due:false},wars:[]},
  production:{mode:'province_projects',nation:'FRA',date,as_of_day:day,construction_budget:{enrolled:true,daily_budget_bn:.01,available_bn:1}},
  outcomes:{nation:'FRA',date,as_of_day:day,
    money:{journal_available:true,decisions:[],program:{enabled:true,fiscal_year:y,settled_day:day-1}},
    construction:{projects:[],completions:[],operating:[],mines:[]},
    // equipment::EquipmentState starts with an empty learned set; only research_step inserts into it.
    research:{active:null,learned:0,last_completed_day:null,drafts:[],revisions:[],projects:[]},
    procurement:{companies:0,certified_products:0,developing_products:0,deliveries:[],imports:[]},
    aviation:{bases:[],squadrons:[],missions:[]}}};}
// Every step achieved from dated native records, read on 1 Jul 1992 in the session a load created.
function achieved(){const r=reading(1992,7,1),o=r.outcomes,today=o.as_of_day;r.state.session_id='s-2';
  o.money.decisions=[{id:1,day:0,kind:'program_budget'},{id:2,day:DAY(1991,1,2),kind:'program_budget'},{id:3,day:DAY(1992,1,2),kind:'annual_budget'},{id:4,day:DAY(1992,3,1),kind:'tax_rate'}];
  o.construction={projects:[{id:7,kind:'starter_industry',district:'FR-IDF',spent_bn:.4,contract_cost_bn:1,last_day:today-1,last_spent_bn:.01,progress_days:40,total_days:90}],
    completions:[{date:'28 Dec 1990',day:DAY(1990,12,28),text:'France completes a 5% Starter Industry module in FR-IDF.',district:'FR-IDF',kind:'starter_industry'}],
    operating:[{district:'FR-IDF',kind:'starter_industry',day:today-1,output_daily:.002}],mines:[]};
  // A completed development contract as the daily tick leaves it: last_spent_bn zeroed, last_day stamped yesterday.
  o.research={active:null,learned:13,last_completed_day:DAY(1990,4,1),drafts:[{name:'Mirage study',updated_day:DAY(1990,1,5),valid:true}],
    revisions:[{id:'FRA-design-1',platform:'air_fighter',created_day:DAY(1990,2,1),certified_day:DAY(1990,10,1),imported:false}],
    projects:[{id:3,revision:'FRA-design-1',started_day:DAY(1990,2,1),completed_day:DAY(1990,10,1),last_day:today-1,last_spent_bn:0,spent_bn:.5}]};
  o.procurement={companies:1,certified_products:1,developing_products:0,imports:[],
    deliveries:[{id:4,company:'Dassault',revision:'FRA-design-1',quantity:12,purchased_day:DAY(1991,3,1),settled_day:DAY(1991,3,1),due_day:DAY(1991,3,8),delivered_day:DAY(1991,3,8),status:'delivered'}]};
  o.aviation={bases:[{id:'FR-IDF',name:'Paris',project:null,history:[{track:'capacity',target_level:1,started_day:DAY(1990,1,2),completed_day:DAY(1990,1,14)}]}],
    squadrons:[{id:1,assigned:12,ready:true,blocker:null}],missions:[{id:1,status:'flown',report_day:DAY(1992,6,1),aircraft:4}]};
  return r;}
const loaded=()=>({saves:[{session_id:'s-1',player:'FRA',slot:'route',date:'30 Jun 1992',dispatches:400}],loads:[{session_id:'s-2',player:'FRA',slot:'route',backup:false,date:'30 Jun 1992',dispatch_count:400}]});
const run=(r,receipts=NONE)=>advisor.recognize(r,receipts);
const step=(route,id)=>route.steps.find(s=>s.id===id);
const mile=(route,id,m)=>step(route,id).milestones.find(x=>x.id===m);
function deepFrozen(v,seen=new Set()){if(v===null||typeof v!=='object'||seen.has(v))return true;seen.add(v);return Object.isFrozen(v)&&Object.values(v).every(x=>deepFrozen(x,seen));}
function objects(v,out=new Set()){if(v!==null&&typeof v==='object'&&!out.has(v)){out.add(v);Object.values(v).forEach(x=>objects(x,out));}return out;}
function freezeAll(v){if(v&&typeof v==='object'){Object.values(v).forEach(freezeAll);Object.freeze(v);}return v;}
function contract(route){
  assert.deepEqual(Object.keys(route),['status','reason','as_of','steps']);assert.deepEqual(route.steps.map(s=>s.id),ORDER);assert(deepFrozen(route));
  if(route.status==='ready'){assert.equal(route.reason,null);assert.deepEqual(Object.keys(route.as_of),['session_id','player','date','day']);}else{assert.equal(route.status,'unknown');assert.equal(route.as_of,null);assert.equal(typeof route.reason,'string');}
  for(const s of route.steps){
    assert.deepEqual(Object.keys(s),['id','title','lessons','status','summary','milestones','obstacle']);assert(['achieved','progress','not_yet','unknown'].includes(s.status));assert.equal(typeof s.summary,'string');assert(s.lessons.length>0);
    for(const m of s.milestones){
      assert.deepEqual(Object.keys(m),['id','label','status','day','date','detail']);assert(['done','pending','unknown'].includes(m.status));
      if(m.day===null)assert.equal(m.date,null);else{assert(Number.isSafeInteger(m.day)&&m.day>=0&&m.day<=route.as_of.day);assert.equal(m.date,iso(m.day));assert.equal(m.status,'done');}
      assert(m.detail===null||typeof m.detail==='string'&&m.detail.length<=120);
    }
    if(s.obstacle){assert.deepEqual(Object.keys(s.obstacle),['title','reason','action','actionLabel']);assert(KINDS.includes(s.obstacle.action.kind),s.obstacle.action.kind);assert(['not_yet','progress'].includes(s.status));}
    if(s.status==='achieved'||s.status==='unknown')assert.equal(s.obstacle,null);
  }
  return route;
}
function allUnknown(route,reason){contract(route);assert.equal(route.status,'unknown');if(reason)assert.match(route.reason,reason);
  for(const s of route.steps){assert.equal(s.status,'unknown');assert.equal(s.obstacle,null);assert(s.milestones.length>0&&s.milestones.every(m=>m.status==='unknown'&&m.day===null&&m.date===null));}}

test('a fresh campaign reads not yet on every step, with an obstacle and a review route',()=>{
  const route=contract(run(reading()));
  assert.equal(route.status,'ready');assert.deepEqual(route.as_of,{session_id:'4242-1-1',player:'FRA',date:'1 Jun 1990',day:151});
  assert.deepEqual(route.steps.map(s=>s.status),['not_yet','not_yet','not_yet','not_yet','not_yet','not_yet']);
  assert.deepEqual(route.steps.map(s=>s.lessons),[['budget-treasury'],['construction-effects','industry-operations'],['research-components'],['equipment-procurement'],['air-force'],['save-review']]);
  assert.deepEqual(route.steps.map(s=>s.obstacle.action),[{kind:'budget'},{kind:'construction'},{kind:'equipment',tab:'designer'},{kind:'companies'},{kind:'air',page:'bases'},{kind:'campaign'}]);
  assert.deepEqual(route.steps.map(s=>s.obstacle.title),['Enact this year\'s budget','Start a useful construction project','Save your first design','Establish a manufacturer','Fund an airbase foundation','Save your campaign']);
  for(const s of route.steps){assert(s.milestones.every(m=>m.status==='pending'));assert(s.obstacle.reason.length<=240,s.id);}
  assert.match(step(route,'finances').obstacle.reason,/1990/);assert.match(step(route,'air_force').obstacle.reason,/12 funded days/);
});

test('real native records are recognised as achieved with their dates',()=>{
  const route=contract(run(achieved(),loaded()));
  assert.equal(route.status,'ready');assert.deepEqual(route.as_of,{session_id:'s-2',player:'FRA',date:'1 Jul 1992',day:DAY(1992,7,1)});
  assert.deepEqual(route.steps.map(s=>s.status),Array(6).fill('achieved'));
  assert(route.steps.every(s=>s.obstacle===null));
  assert.deepEqual(mile(route,'finances','budget_enacted'),{id:'budget_enacted',label:'Yearly budget enacted',status:'done',day:DAY(1992,1,2),date:'1992-01-02',detail:'Annual budget enacted.'});
  assert.equal(mile(route,'construction','work_paid').date,'1992-06-30');assert.match(mile(route,'construction','work_paid').detail,/\$0\.01bn.*starter industry in FR-IDF/);
  assert.equal(mile(route,'construction','project_completed').date,'1990-12-28');assert.equal(mile(route,'construction','site_producing').status,'done');
  assert.equal(step(route,'research_design').summary,'Design saved on 1990-01-05.');
  assert.deepEqual(['design_saved','design_commissioned','development_paid','research_completed'].map(m=>mile(route,'research_design',m).date),['1990-01-05','1990-02-01','1990-10-01','1990-04-01']);
  assert.equal(mile(route,'research_design','development_paid').detail,'Paid $0.5bn in total toward FRA-design-1.','the paid total survives the tick zeroing last_spent_bn');
  assert.match(mile(route,'research_design','design_commissioned').detail,/Certified on 1990-10-01/);
  assert.deepEqual(['manufacturer','certified_product','purchase_placed','purchase_paid','equipment_delivered'].map(m=>mile(route,'procurement',m).status),Array(5).fill('done'));
  assert.equal(mile(route,'procurement','equipment_delivered').date,'1991-03-08');assert.equal(mile(route,'procurement','manufacturer').day,null);
  assert.equal(step(route,'air_force').summary,'Airbase improvement completed on 1990-01-14.');assert.equal(mile(route,'air_force','mission_flown').date,'1992-06-01');
  assert.equal(mile(route,'air_force','squadron_ready').day,null,'squadrons carry no native date');
  assert.equal(mile(route,'save_resume','campaign_resumed').date,'1992-06-30');assert.equal(mile(route,'save_resume','campaign_saved').status,'done');
  const cards=advisor.evaluate(achieved().state,achieved().production,achieved().outcomes,loaded());assert(!cards.some(c=>c.id==='route-next'),'no route card once every step is achieved');
});

test('reading progress, commands and previews never achieve a result',()=>{
  const r=reading();const progress={version:1,done:{'budget-treasury':true,'save-review':true},skipped:{'construction-effects':true}};
  allUnknown(run({state:r.state},loaded()),/no campaign results/);
  allUnknown(run({state:r.state,production:r.production},loaded()),/no campaign results/);
  allUnknown(run({state:r.state,production:r.production,outcomes:null},loaded()),/no campaign results/);
  allUnknown(run({state:r.state,outcomes:progress},loaded()),/different nation/);
  const none=run({state:r.state,outcomes:{nation:'FRA',as_of_day:151,date:'1 Jun 1990',errors:[],command_replayed:false,valid:true}},loaded());contract(none);
  assert(none.steps.slice(0,5).every(s=>s.status==='unknown'),'a command response or preview token is not a native record');
  assert(advisor.evaluate(r.state,r.production).every(c=>c.id!=='route-next'));
  // Skipped or completed lessons neither add nor remove a native result.
  const full=achieved(),base=run(full,loaded());assert.deepEqual(run({...full,progress,tutorial:progress},loaded()),base);
  const fresh=reading();assert.deepEqual(run({...fresh,progress},NONE),run(fresh,NONE));
});

test('finances needs a dated yearly budget decision from this calendar year',()=>{
  const at=(decisions,r=reading())=>{r.outcomes.money.decisions=decisions;return contract(run(r));},today=151;
  assert.equal(mile(at([{id:1,day:0,kind:'program_budget'}]),'finances','budget_enacted').date,'1990-01-01');
  assert.equal(step(at([{id:1,day:today,kind:'annual_budget'}]),'finances').status,'achieved','an immediate command decision carries day == as_of_day');
  for(const kind of ['tax_rate','interest_rate','fiscal_consolidation','PROGRAM_BUDGET','CONSTRUCTION_BUDGET'])assert.equal(step(at([{id:1,day:10,kind}]),'finances').status,'not_yet',kind);
  // programs::set_construction_budget dispatches SetAnnualBudget{fiscal_year: w.year}, so funding construction seats this year's plan.
  let funded=at([{id:1,day:10,kind:'construction_budget'}]);assert.equal(step(funded,'finances').status,'achieved','construction funding enacts this year\'s budget natively');
  assert.deepEqual(mile(funded,'finances','budget_enacted'),{id:'budget_enacted',label:'Yearly budget enacted',status:'done',day:10,date:iso(10),detail:'Inherited plan enacted with construction funding.'});
  assert.equal(mile(at([{id:1,day:5,kind:'program_budget'},{id:2,day:10,kind:'construction_budget'}]),'finances','budget_enacted').detail,'Ministry budget enacted.','the earliest enactment this year names the plan');
  assert.equal(step(at([{id:1,day:today+1,kind:'construction_budget'}]),'finances').status,'not_yet');
  assert.equal(step(at([{id:1,day:DAY(1990,12,31),kind:'construction_budget'}],reading(1991,6,1)),'finances').status,'not_yet','last year\'s construction funding enacted last year\'s plan');
  const fr=reading();fr.outcomes.money.decisions=[{id:1,day:10,kind:'construction_budget'}];const frCards=advisor.evaluate(fr.state,fr.production,fr.outcomes,NONE);
  assert(!frCards.some(c=>c.title==='Enact this year\'s budget'),'no route card pushes a paid re-enactment after construction funding');assert.deepEqual(frCards.find(c=>c.id==='route-next').action,{kind:'construction'});
  let route=at([{id:1,day:today+1,kind:'program_budget'}]);assert.equal(step(route,'finances').status,'not_yet','a day after as_of_day is rejected');assert.equal(mile(route,'finances','budget_enacted').day,null);
  for(const day of [-1,1.5,'0',undefined,NaN,Infinity,-Infinity,2**60]){route=at([{id:1,day,kind:'program_budget'}]);assert.equal(step(route,'finances').status,'unknown',String(day));assert.equal(mile(route,'finances','budget_enacted').day,null);}
  assert.equal(step(at([{id:1,day:-1,kind:'program_budget'},{id:2,day:5,kind:'program_budget'}]),'finances').status,'achieved');
  for(const row of [null,7,'program_budget',{id:1,day:5},{id:1,day:5,kind:3}])assert.equal(step(at([row]),'finances').status,'unknown',JSON.stringify(row));
  const later=()=>reading(1991,6,1);
  assert.equal(step(at([{id:1,day:DAY(1990,12,31),kind:'program_budget'}],later()),'finances').status,'not_yet','last year\'s budget does not count');
  assert.equal(mile(at([{id:1,day:DAY(1991,1,1),kind:'program_budget'}],later()),'finances','budget_enacted').date,'1991-01-01');
  const trimmed=Array.from({length:12},(_,i)=>({id:i+5,day:DAY(1991,2,1)+i,kind:'tax_rate'}));
  route=at(trimmed,later());assert.equal(step(route,'finances').status,'unknown','a trimmed journal reaching into this year cannot prove absence');assert.match(mile(route,'finances','budget_enacted').detail,/trimmed/);
  assert.equal(step(at([{id:5,day:DAY(1990,11,1),kind:'tax_rate'},...trimmed.slice(1)],later()),'finances').status,'not_yet','the trimmed rows are older than this year');
  for(const money of [{journal_available:false,decisions:null,program:null},{journal_available:false,decisions:[{id:1,day:0,kind:'program_budget'}]},{journal_available:true,decisions:null},{journal_available:'true',decisions:[]},{decisions:[]}]){
    const r=reading();r.outcomes.money=money;const s=step(contract(run(r)),'finances');assert.equal(s.status,'unknown',JSON.stringify(money));assert.equal(s.obstacle,null);}
});

test('construction needs dated paid work; completion and output are later milestones',()=>{
  const r=reading(),today=151,c=r.outcomes.construction,project={id:7,kind:'starter_industry',district:'FR-IDF',spent_bn:0,contract_cost_bn:1,last_day:null,last_spent_bn:null,progress_days:0,total_days:90};
  c.projects=[{...project}];let route=contract(run(r)),s=step(route,'construction');
  assert.equal(s.status,'not_yet');assert.deepEqual(s.obstacle.action,{kind:'project',id:7});assert.match(s.obstacle.title,/awaits paid work/);
  r.production.construction_budget.enrolled=false;assert.equal(step(run(r),'construction').obstacle.title,'Open construction funding');
  r.production.construction_budget={enrolled:true,daily_budget_bn:0};assert.equal(step(run(r),'construction').obstacle.title,'Open construction funding');
  r.production.nation='USA';assert.match(step(run(r),'construction').obstacle.title,/awaits paid work/,'another nation\'s construction reading is ignored');
  c.projects=[{...project,last_day:today-1,last_spent_bn:.01}];s=step(contract(run(r)),'construction');assert.equal(s.status,'achieved');assert.equal(s.summary,`Construction work paid on ${iso(today-1)}.`);
  c.projects=[{...project,last_day:today,last_spent_bn:.01}];assert.equal(step(run(r),'construction').status,'achieved');
  c.projects=[{...project,last_day:today+1,last_spent_bn:.01}];assert.equal(step(run(r),'construction').status,'not_yet','a payment after as_of_day is rejected');
  c.projects=[{...project,last_day:today-1,last_spent_bn:0}];assert.equal(step(run(r),'construction').status,'not_yet');
  for(const bad of [{last_day:-1,last_spent_bn:.01},{last_day:3.5,last_spent_bn:.01},{last_day:'3',last_spent_bn:.01},{last_day:null,last_spent_bn:.01},{last_day:3,last_spent_bn:'0.01'},{last_day:3,last_spent_bn:-1},{last_day:3,last_spent_bn:NaN}]){
    c.projects=[{...project,...bad}];assert.equal(step(contract(run(r)),'construction').status,'unknown',JSON.stringify(bad));}
  c.projects=[7];assert.equal(step(run(r),'construction').status,'unknown');
  const plant={date:'28 Dec 1990',day:DAY(1990,12,28),text:'France completes Materials Processing level 2 in FR-IDF.',district:'FR-IDF',kind:'processing_plant'};
  c.projects=[];c.completions=[{...plant}];
  let late=reading(1991,2,1);late.outcomes.construction=c;route=contract(run(late));s=step(route,'construction');
  assert.equal(s.status,'achieved','a dated completion keeps the step achieved after its paid project leaves the queue');assert.equal(mile(route,'construction','project_completed').date,'1990-12-28');assert.equal(s.obstacle,null);
  const out=late.outcomes.as_of_day-1,site=k=>step(run(late),'construction').milestones.find(m=>m.id==='site_producing')[k];
  c.operating=[{district:'FR-IDF',kind:'processing_plant',day:out,output_daily:.5}];assert.equal(site('date'),iso(out));assert.equal(site('detail'),'Output recorded at processing plant in FR-IDF.');
  c.operating=[{district:'FR-PAC',kind:'processing_plant',day:out,output_daily:.5}];assert.equal(site('status'),'pending','output at a site without a player completion is not construction evidence');
  c.operating=[{district:'FR-ID',kind:'processing_plant',day:out,output_daily:.5}];assert.equal(site('status'),'pending','district names must match exactly');
  c.operating=[{district:'FR-IDF',kind:'processing_plant',day:out,output_daily:0}];assert.equal(site('status'),'pending');
  c.operating=[{district:'FR-IDF',kind:'processing_plant',day:out+2,output_daily:.5}];assert.equal(site('status'),'pending','a receipt after as_of_day is rejected');
  // The site is district plus kind: another nation's plant in a district where the player completed something else is not the player's output.
  c.completions=[{...plant,text:'France completes Power grid level 2 in FR-IDF.',kind:'power_grid'}];c.operating=[{district:'FR-IDF',kind:'processing_plant',day:out,output_daily:.5}];
  route=contract(run(late));assert.equal(mile(route,'construction','project_completed').status,'done');assert.equal(mile(route,'construction','site_producing').status,'pending','a Power grid completion does not credit a processing plant in the same district');
  c.completions=[{...plant,district:null,kind:null}];assert.equal(site('status'),'pending','a headline naming the district without a mapped site proves no output');assert.equal(step(run(late),'construction').status,'achieved');
  c.completions=[{...plant,district:'FR-IDF',kind:null}];assert.equal(site('status'),'pending');
  for(const bad of [{district:undefined},{kind:undefined},{district:7},{kind:''},{kind:['processing_plant']}]){c.completions=[{...plant,...bad}];route=contract(run(late));
    assert.equal(mile(route,'construction','site_producing').status,'unknown',JSON.stringify(bad));assert.equal(mile(route,'construction','project_completed').status,'unknown',JSON.stringify(bad));}
  c.completions=[{...plant,kind:7},{...plant}];route=contract(run(late));assert.equal(mile(route,'construction','site_producing').status,'done','a matching dated site still proves output beside a malformed row');
  c.completions=[{...plant}];for(const bad of [{kind:null},{kind:''},{kind:3},{district:null}]){c.operating=[{district:'FR-IDF',kind:'processing_plant',day:out,output_daily:.5,...bad}];assert.equal(site('status'),'unknown',JSON.stringify(bad));}
  c.completions=null;c.operating=[{district:'FR-IDF',kind:'processing_plant',day:out,output_daily:.5}];assert.equal(site('status'),'unknown','unreadable completions cannot prove absence of a player site');c.completions=[{...plant}];
  c.completions=[{...plant,date:'bad',day:null}];
  route=run(late);assert.equal(step(route,'construction').status,'not_yet','an undated headline proves nothing');assert.equal(mile(route,'construction','site_producing').status,'pending');
  c.completions=[{...plant,date:'1 Jan 2000',day:DAY(2000,1,1)}];route=run(late);assert.equal(step(route,'construction').status,'not_yet');assert.equal(mile(route,'construction','site_producing').status,'pending','a future completion names no site yet');
  c.completions=[{day:-3,text:'x',district:null,kind:null}];assert.equal(step(run(late),'construction').status,'unknown');
});

test('paid mine work funded from the construction budget counts as construction work',()=>{
  const r=reading(),today=151,c=r.outcomes.construction,mine={district:'FR-NOR',commodity:'coal',spent_bn:0,last_day:null,progress_days:0,total_days:120};
  c.mines=[{...mine}];let route=contract(run(r)),s=step(route,'construction');
  assert.equal(s.status,'not_yet');assert.equal(s.obstacle.title,'Your mine awaits paid work','a queued mine is not told to start a project');assert.deepEqual(s.obstacle.action,{kind:'construction'});
  r.production.construction_budget.enrolled=false;s=step(run(r),'construction');assert.equal(s.obstacle.title,'Open construction funding');assert.match(s.obstacle.reason,/Your mine is queued/);r.production.construction_budget.enrolled=true;
  c.mines=[{...mine,spent_bn:.03,last_day:today-1,progress_days:4}];route=contract(run(r));s=step(route,'construction');
  assert.equal(s.status,'achieved');assert.equal(mile(route,'construction','work_paid').day,null,'mine funding stamps last_day on unpaid days too, so it bounds rather than dates the payment');
  assert.equal(mile(route,'construction','work_paid').detail,`Paid mine work: $0.03bn for coal in FR-NOR by ${iso(today-1)}.`);
  c.mines=[{...mine,spent_bn:.03,last_day:today}];assert.equal(step(run(r),'construction').status,'achieved');
  c.mines=[{...mine,spent_bn:.03,last_day:today+1}];assert.equal(step(run(r),'construction').status,'not_yet','a funding day after as_of_day is rejected');
  c.mines=[{...mine,spent_bn:0,last_day:today-1}];assert.equal(step(run(r),'construction').status,'not_yet','a settle attempt that paid nothing is not paid work');
  c.mines=[{...mine,spent_bn:null,last_day:null}];s=step(contract(run(r)),'construction');assert.equal(s.status,'not_yet','a legacy prepaid row with no funding entry is never estimated as paid');assert.equal(s.obstacle.title,'Start a useful construction project');
  for(const bad of [{spent_bn:.03,last_day:null},{spent_bn:null,last_day:3},{spent_bn:'0.03',last_day:3},{spent_bn:-1,last_day:3},{spent_bn:NaN,last_day:3},{spent_bn:.03,last_day:-1},{spent_bn:.03,last_day:2.5},{spent_bn:.03,last_day:'3'},{district:null,spent_bn:.03,last_day:3},{commodity:'',spent_bn:.03,last_day:3},{spent_bn:undefined}]){
    c.mines=[{...mine,...bad}];assert.equal(step(contract(run(r)),'construction').status,'unknown',JSON.stringify(bad));}
  c.mines=[7];assert.equal(step(run(r),'construction').status,'unknown');
  for(const v of [undefined,null,{},'x']){const x=reading();if(v===undefined)delete x.outcomes.construction.mines;else x.outcomes.construction.mines=v;assert.equal(step(contract(run(x)),'construction').status,'unknown',`mines=${JSON.stringify(v)}: missing mine rows cannot prove no paid work`);}
  const both=reading();both.outcomes.construction.mines=[{...mine,spent_bn:.03,last_day:today-1}];both.outcomes.construction.projects=[{id:7,kind:'starter_industry',district:'FR-IDF',last_day:today-2,last_spent_bn:.01}];
  assert.equal(mile(run(both),'construction','work_paid').date,iso(today-2),'a dated project payment is preferred to a bounded mine date');
  const done=reading(1991,2,1);done.outcomes.construction.completions=[{date:'28 Dec 1990',day:DAY(1990,12,28),text:'France opens the Coal mine in FR-NOR.',district:'FR-NOR',kind:'mine'}];
  route=contract(run(done));assert.equal(step(route,'construction').status,'achieved','a completed mine headline counts as a completed project');
  const card=reading();card.outcomes.money.decisions=[{id:1,day:0,kind:'program_budget'}];card.outcomes.construction.mines=[{...mine}];
  const next=advisor.evaluate(card.state,card.production,card.outcomes,NONE).find(c=>c.id==='route-next');assert.equal(next.title,'Your mine awaits paid work');assert.deepEqual(next.action,{kind:'construction'});
});

test('research and design results need a dated saved design, revision, payment or completion',()=>{
  const at=edit=>{const r=reading();edit(r.outcomes.research,r.outcomes.as_of_day);return contract(run(r));};
  let route=at(()=>{}),s=step(route,'research_design');assert.equal(s.status,'not_yet');assert.equal(s.obstacle.title,'Save your first design');
  route=at(x=>{x.active={component:'radar_pulse_doppler',progress:4,cost:24};});s=step(route,'research_design');
  assert.equal(s.status,'progress');assert.equal(mile(route,'research_design','research_active').day,null);assert.match(mile(route,'research_design','research_active').detail,/radar pulse doppler: 4 of 24 points/);
  assert.equal(s.obstacle.title,'Save a design while research runs');assert.deepEqual(s.obstacle.action,{kind:'equipment',tab:'designer'});
  for(const active of [{component:'radar',progress:4,cost:0},{component:'radar',progress:-1,cost:24},{component:'',progress:1,cost:24},{component:'radar',progress:1,cost:null},'radar'])
    assert.equal(step(at(x=>{x.active=active;}),'research_design').status,'unknown',JSON.stringify(active));
  assert.equal(mile(at((x,d)=>{x.drafts=[{name:'Test',updated_day:d,valid:true}];}),'research_design','design_saved').detail,'Saved "Test".');
  assert.equal(step(at((x,d)=>{x.drafts=[{name:'Test',updated_day:d+1,valid:true}];}),'research_design').status,'not_yet','a draft dated after as_of_day is rejected');
  assert.equal(step(at(x=>{x.drafts=[{name:'Test',updated_day:-2,valid:true}];}),'research_design').status,'unknown');
  assert.equal(step(at(x=>{x.revisions=[{id:'import-4-2',created_day:3,certified_day:3,imported:true}];}),'research_design').status,'not_yet','a bought import is not research');
  route=at(x=>{x.revisions=[{id:'FRA-design-1',created_day:5,certified_day:null,imported:false}];});assert.equal(mile(route,'research_design','design_commissioned').detail,'In company development.');assert.equal(step(route,'research_design').status,'achieved');
  assert.equal(step(at(x=>{x.revisions=[{id:'FRA-design-1',created_day:5,certified_day:null}];}),'research_design').status,'unknown','a revision without an import flag is not assumed to be own work');
  assert.equal(step(at((x,d)=>{x.revisions=[{id:'FRA-design-1',created_day:d+3,certified_day:null,imported:false}];}),'research_design').status,'not_yet');
  assert.equal(mile(at(x=>{x.projects=[{id:3,revision:'FRA-design-1',last_day:9,last_spent_bn:.002,spent_bn:.01}];}),'research_design','development_paid').date,iso(9));
  assert.equal(step(at(x=>{x.projects=[{id:3,revision:'FRA-design-1',last_day:null,last_spent_bn:0,spent_bn:0}];}),'research_design').status,'not_yet');
  assert.equal(mile(at(x=>{x.learned=1;x.last_completed_day=40;}),'research_design','research_completed').date,iso(40));
  for(const day of [-1,2.5,'40',undefined])assert.equal(step(at(x=>{x.learned=1;x.last_completed_day=day;}),'research_design').status,'unknown',String(day));
  assert.equal(step(at((x,d)=>{x.last_completed_day=d+1;}),'research_design').status,'not_yet');
});

test('only a draft the native designer check accepts counts as a saved design',()=>{
  const at=edit=>{const r=reading();edit(r.outcomes.research,r.outcomes.as_of_day);return contract(run(r));},draft={name:'Test',updated_day:40};
  let route=at(x=>{x.drafts=[{...draft,valid:true}];});assert.equal(step(route,'research_design').status,'achieved');assert.equal(mile(route,'research_design','design_saved').date,iso(40));
  route=at(x=>{x.drafts=[{...draft,valid:false}];});let s=step(route,'research_design');
  assert.equal(s.status,'not_yet','an invalid draft is not a design');assert.deepEqual(mile(route,'research_design','design_saved'),{id:'design_saved',label:'Design saved',status:'pending',day:null,date:null,detail:'Saved drafts are not valid designs yet.'});
  assert.equal(s.obstacle.title,'Make a saved design valid');assert.match(s.obstacle.reason,/not valid designs yet/);assert.deepEqual(s.obstacle.action,{kind:'equipment',tab:'designer'});
  route=at(x=>{x.drafts=[{...draft,valid:false}];x.active={component:'radar',progress:4,cost:24};});s=step(route,'research_design');assert.equal(s.status,'progress');assert.equal(s.obstacle.title,'Make a saved design valid');assert.match(s.obstacle.reason,/a valid saved design/);
  route=at(x=>{x.drafts=[{...draft,valid:false},{...draft,name:'Good',updated_day:50,valid:true}];});assert.equal(mile(route,'research_design','design_saved').detail,'Saved "Good".','a valid draft counts beside an invalid one');
  for(const valid of [null,undefined,'true',1,0,{}]){route=at(x=>{x.drafts=[{...draft,valid}];});const m=mile(route,'research_design','design_saved');
    assert.equal(m.status,'unknown',`valid=${JSON.stringify(valid)}`);assert.equal(step(route,'research_design').status,'unknown',`valid=${JSON.stringify(valid)}`);}
  route=at(x=>{const row={...draft};delete row.valid;x.drafts=[row];});assert.equal(mile(route,'research_design','design_saved').status,'unknown','a draft without a validity reading is unknown');
  route=at((x,day)=>{x.drafts=[{...draft,updated_day:day+1,valid:false}];});assert.equal(mile(route,'research_design','design_saved').detail,null,'a future invalid draft is not reported as saved');
  const card=reading();card.outcomes.money.decisions=[{id:1,day:0,kind:'program_budget'}];card.outcomes.construction.projects=[{id:7,kind:'starter_industry',district:'FR-IDF',last_day:150,last_spent_bn:.01}];card.outcomes.research.drafts=[{...draft,valid:false}];
  const next=advisor.evaluate(card.state,card.production,card.outcomes,NONE).find(c=>c.id==='route-next');assert.equal(next.title,'Make a saved design valid');assert.deepEqual(next.evidence,['First-hour step: Research toward a design.','Not yet achieved in this campaign.']);
  card.outcomes.research.drafts=[{...draft,valid:true}];assert.equal(advisor.evaluate(card.state,card.production,card.outcomes,NONE).find(c=>c.id==='route-next').action.kind,'companies','a valid draft moves the route card on');
});

test('completed component research survives the next research order clearing its date',()=>{
  const at=edit=>{const r=reading();edit(r.outcomes.research,r.outcomes.as_of_day);return contract(run(r));};
  // Day 150: research_step sets last_research_completed_day and inserts into learned.
  let route=at(x=>{x.learned=1;x.last_completed_day=150;});assert.equal(step(route,'research_design').status,'achieved');assert.equal(mile(route,'research_design','research_completed').date,iso(150));
  // Day 151: equipment::start_research clears last_research_completed_day; learned keeps the result.
  route=at(x=>{x.learned=1;x.last_completed_day=null;x.active={component:'tank_fire_control',progress:0,cost:24};});let s=step(route,'research_design');
  assert.equal(s.status,'achieved','starting the next component does not undo a finished one');assert.equal(s.obstacle,null);
  assert.deepEqual(mile(route,'research_design','research_completed'),{id:'research_completed',label:'Component research completed',status:'done',day:null,date:null,detail:'Components researched: 1.'});
  route=at(x=>{x.learned=0;x.last_completed_day=null;x.active={component:'tank_fire_control',progress:3,cost:24};});s=step(route,'research_design');
  assert.equal(s.status,'progress');assert.equal(mile(route,'research_design','research_completed').status,'pending');assert.equal(s.obstacle.title,'Save a design while research runs');
  assert.equal(step(at(x=>{x.learned=0;}),'research_design').status,'not_yet');
  assert.equal(mile(at((x,d)=>{x.learned=2;x.last_completed_day=d+1;}),'research_design','research_completed').day,null,'a future date is rejected but the learned set still proves completion');
  assert.equal(mile(at(x=>{x.learned=0;x.last_completed_day=40;}),'research_design','research_completed').status,'unknown','a completion date with nothing learned is contradictory');
  for(const learned of [undefined,null,-1,1.5,'1',NaN,true])for(const last of [null,40]){const m=mile(at(x=>{x.learned=learned;x.last_completed_day=last;}),'research_design','research_completed');assert.equal(m.status,'unknown',`learned=${String(learned)} last=${last}`);}
  const card=reading();card.outcomes.money.decisions=[{id:1,day:0,kind:'program_budget'}];card.outcomes.construction.projects=[{id:7,kind:'starter_industry',district:'FR-IDF',last_day:150,last_spent_bn:.01}];
  Object.assign(card.outcomes.research,{learned:1,last_completed_day:null,active:{component:'tank_fire_control',progress:0,cost:24}});
  assert.deepEqual(advisor.evaluate(card.state,card.production,card.outcomes,NONE).find(c=>c.id==='route-next').action,{kind:'companies'},'the route card does not send the player back to research');
});

test('development payment survives the daily tick zeroing last_spent_bn',()=>{
  const at=edit=>{const r=reading();edit(r.outcomes.research,r.outcomes.as_of_day);return contract(run(r));};
  const job={id:3,revision:'FRA-design-1',started_day:5,completed_day:null,last_day:9,last_spent_bn:.002,spent_bn:.01};
  let route=at((x,d)=>{x.projects=[{...job,completed_day:d-3,last_day:d-1,last_spent_bn:0,spent_bn:.1}];});
  assert.equal(step(route,'research_design').status,'achieved','a completed contract stays paid');assert.equal(mile(route,'research_design','development_paid').date,iso(148));assert.equal(mile(route,'research_design','development_paid').detail,'Paid $0.1bn in total toward FRA-design-1.');
  route=at((x,d)=>{x.projects=[{...job,last_day:d-1,last_spent_bn:0,spent_bn:.01}];});
  assert.equal(mile(route,'research_design','development_paid').status,'done','a paused day keeps the paid total');assert.equal(mile(route,'research_design','development_paid').day,null,'an unfinished contract has no completion date to show');
  assert.equal(mile(at((x,d)=>{x.projects=[{...job,last_day:d-1,last_spent_bn:.002,spent_bn:.01}];}),'research_design','development_paid').date,iso(150),'a dated daily payment keeps its date');
  assert.equal(step(at((x,d)=>{x.projects=[{...job,last_day:d-1,last_spent_bn:0,spent_bn:0}];}),'research_design').status,'not_yet','nothing paid is not paid');
  assert.equal(step(at((x,d)=>{x.projects=[{...job,last_day:d+1,last_spent_bn:0,spent_bn:.01}];}),'research_design').status,'not_yet','a tick after as_of_day is rejected');
  assert.equal(step(at((x,d)=>{x.projects=[{...job,completed_day:d+2,last_day:d-1,last_spent_bn:0,spent_bn:.01}];}),'research_design').status,'not_yet','a completion after as_of_day is rejected');
  for(const bad of [{spent_bn:'0.01'},{spent_bn:-1},{spent_bn:NaN},{spent_bn:undefined},{completed_day:-1},{completed_day:'9'},{last_day:null},{last_day:2.5}])
    assert.equal(step(at(x=>{x.projects=[{...job,last_spent_bn:0,...bad}];}),'research_design').status,'unknown',JSON.stringify(bad));
});

test('procurement names the current obstacle and counts only a dated delivery',()=>{
  const r=reading(),p=r.outcomes.procurement,today=151,order={id:4,company:'Dassault',revision:'FRA-design-1',quantity:12,purchased_day:today-1,settled_day:null,due_day:null,delivered_day:null,status:'awaiting_settlement'};
  const at=()=>contract(run(r)),obs=()=>step(at(),'procurement').obstacle;
  assert.equal(obs().title,'Establish a manufacturer');
  p.companies=1;assert.equal(step(at(),'procurement').status,'progress');assert.equal(obs().title,'Finish a design in development');assert.match(obs().reason,/180 days.*240/);assert.match(obs().reason,/Start one early/);
  p.developing_products=1;assert.match(obs().reason,/already in development/);
  p.certified_products=1;assert.equal(obs().title,'Buy from available stock');
  p.deliveries=[{...order}];let route=at();assert.equal(mile(route,'procurement','purchase_placed').date,iso(today-1));assert.equal(step(route,'procurement').obstacle.title,'Awaiting fiscal settlement');assert.match(step(route,'procurement').obstacle.reason,/No action is needed/);
  p.deliveries=[{...order,settled_day:today-1,due_day:today+6,status:'in_transit'}];route=at();assert.equal(mile(route,'procurement','purchase_paid').status,'done');
  assert.equal(step(route,'procurement').obstacle.title,'Delivery on its way');assert.match(step(route,'procurement').obstacle.reason,new RegExp(iso(today+6)));assert.deepEqual(step(route,'procurement').obstacle.action,{kind:'companies'});
  p.deliveries=[{...order,purchased_day:today-8,settled_day:today-8,due_day:today-1,delivered_day:today-1,status:'delivered'}];route=at();
  assert.equal(step(route,'procurement').status,'achieved');assert.equal(mile(route,'procurement','equipment_delivered').detail,'Delivered: 12 × FRA-design-1.');
  p.deliveries=[{...order,purchased_day:today-8,settled_day:today-8,due_day:today+1,delivered_day:today+1,status:'delivered'}];route=at();
  assert.equal(step(route,'procurement').status,'progress','a delivery dated after as_of_day is rejected');assert.equal(step(route,'procurement').obstacle.title,'Awaiting delivery');assert.doesNotMatch(step(route,'procurement').obstacle.reason,/No action is needed|due date/,'only an in_transit lot is promised an arrival');
  for(const bad of [{delivered_day:today-1,settled_day:null},{settled_day:today-3,purchased_day:today-2},{delivered_day:today-5,settled_day:today-4,purchased_day:today-6},{quantity:0},{quantity:'12'},{purchased_day:-1},{purchased_day:1.5},{due_day:-1}]){
    p.deliveries=[{...order,...bad}];assert.equal(step(at(),'procurement').status,'unknown',JSON.stringify(bad));}
  p.deliveries=[];p.companies=0;p.certified_products=0;
  const imp={id:2,seller:'USA',revision:'import-9-1',ammunition:false,quantity:4,purchased_day:today-20,settled_day:today-20,due_day:today-6,delivered_day:today-6,cancelled_day:null,status:'delivered'};
  p.imports=[{...imp}];route=at();assert.equal(step(route,'procurement').status,'achieved','foreign stock skips the domestic company route');assert.match(mile(route,'procurement','equipment_delivered').detail,/\(import\)/);
  p.imports=[{...imp,ammunition:true}];assert.equal(step(at(),'procurement').status,'not_yet','ammunition is not an equipment purchase');
  p.imports=[{...imp,cancelled_day:today-10}];assert.equal(step(at(),'procurement').status,'not_yet','a cancelled contract does not count');
  for(const bad of [{ammunition:undefined},{ammunition:'false'},{cancelled_day:'x'},{cancelled_day:-4}]){p.imports=[{...imp,...bad}];assert.equal(step(at(),'procurement').status,'unknown',JSON.stringify(bad));}
  p.imports=[];for(const v of [undefined,null,'1',-1,1.5]){p.companies=v;assert.equal(step(at(),'procurement').status,'unknown',String(v));}
  p.companies=0;p.deliveries=null;assert.equal(step(at(),'procurement').status,'unknown');
});

test('a blocked delivery or import is never advertised as on its way',()=>{
  const r=reading(),p=r.outcomes.procurement,today=151,promise=/No action is needed|arrives on its due date/;p.companies=1;p.certified_products=1;
  const lot={id:4,company:'Dassault',revision:'FRA-design-1',quantity:12,purchased_day:today-20,settled_day:today-20,due_day:today+3,delivered_day:null,status:'blocked'};
  const imp={id:2,seller:'USA',revision:'import-9-1',ammunition:false,quantity:4,purchased_day:today-30,settled_day:today-30,due_day:today+1,delivered_day:null,cancelled_day:null,status:'blocked'};
  const shapes=[['domestic delivery whose due day slides forward each blocked day',{deliveries:[lot],imports:[]}],
    ['domestic delivery refused by the Arsenal after its due day',{deliveries:[{...lot,due_day:today-4}],imports:[]}],
    ['import whose route closed',{deliveries:[],imports:[imp]}]];
  for(const [name,rows] of shapes){Object.assign(p,structuredClone(rows));const route=contract(run(r)),s=step(route,'procurement');
    assert.equal(s.status,'progress',name);assert.equal(mile(route,'procurement','purchase_paid').status,'done','the payment is still a true result');assert.equal(mile(route,'procurement','equipment_delivered').status,'pending',name);
    assert.notEqual(s.obstacle.title,'Delivery on its way',name);assert.equal(s.obstacle.title,'Delivery blocked',name);assert.doesNotMatch(s.obstacle.reason,promise,name);assert.doesNotMatch(s.obstacle.reason,/\d{4}-\d{2}-\d{2}/,name);
    assert.deepEqual(s.obstacle.action,{kind:'companies'});
    const card=reading();card.outcomes.money.decisions=[{id:1,day:0,kind:'program_budget'}];card.outcomes.construction.projects=[{id:7,kind:'starter_industry',district:'FR-IDF',last_day:150,last_spent_bn:.01}];
    card.outcomes.research.drafts=[{name:'x',updated_day:3,valid:true}];Object.assign(card.outcomes.procurement,{companies:1,certified_products:1},structuredClone(rows));
    const next=advisor.evaluate(card.state,card.production,card.outcomes,NONE).find(c=>c.id==='route-next');assert.equal(next.title,'Delivery blocked',name);assert.doesNotMatch(next.reason,promise,name);}
  // A blocked lot outranks one still moving, so the stop is not hidden behind a transit promise.
  p.deliveries=[{...lot,id:5,status:'in_transit',due_day:today+2},{...lot}];p.imports=[];assert.equal(step(run(r),'procurement').obstacle.title,'Delivery blocked');
  p.deliveries=[{...lot,status:'in_transit',due_day:today+2}];let s=step(run(r),'procurement');assert.equal(s.obstacle.title,'Delivery on its way');assert.match(s.obstacle.reason,new RegExp(`arrives on its due date, ${iso(today+2)}\\. No action is needed\\.`));
  p.deliveries=[{...lot,status:'in_transit',due_day:today-1}];s=step(run(r),'procurement');assert.equal(s.obstacle.title,'Delivery on its way');assert.doesNotMatch(s.obstacle.reason,/due date|\d{4}-\d{2}-\d{2}/,'a past due day is not presented as an arrival date');
  for(const status of ['awaiting_refund_settlement','delivered','Blocked','x'.repeat(41),undefined]){const row={...lot};if(status===undefined)delete row.status;else row.status=status;p.deliveries=[row];s=step(contract(run(r)),'procurement');
    assert.equal(s.obstacle.title,'Awaiting delivery',String(status));assert.doesNotMatch(s.obstacle.reason,promise,String(status));}
  for(const status of [null,7,{},true]){p.deliveries=[{...lot,status}];assert.equal(step(contract(run(r)),'procurement').status,'unknown',JSON.stringify(status));}
  p.deliveries=[{...lot,settled_day:null,due_day:null,status:'awaiting_settlement'}];assert.match(step(run(r),'procurement').obstacle.reason,/No action is needed/);
  p.deliveries=[{...lot,settled_day:null,due_day:null,status:'unexpected'}];s=step(run(r),'procurement');assert.equal(s.obstacle.title,'Order not yet paid');assert.doesNotMatch(s.obstacle.reason,promise);
});

test('air force preparation follows the airbase route before squadrons and missions',()=>{
  const r=reading(),a=r.outcomes.aviation,today=151,project={track:'capacity',target_level:1,started_day:today-3,last_paid_day:null,paid_bn:0,total_cost_bn:.024,completed_day:null,cancelled_day:null};
  const base=(p,h=[])=>[{id:'FR-IDF',name:'Paris',project:p,history:h}],at=()=>contract(run(r));
  let route=at(),s=step(route,'air_force');assert.equal(s.obstacle.title,'Fund an airbase foundation');assert.deepEqual(s.obstacle.action,{kind:'air',page:'bases'});
  assert.deepEqual(['squadron_formed','squadron_ready','mission_flown'].map(m=>mile(route,'air_force',m).detail),['Needs delivered aircraft.','Needs delivered aircraft and a base.','Needs ready aircraft and an eligible conflict.']);
  a.bases=base({...project});s=step(at(),'air_force');assert.equal(s.status,'not_yet');assert.equal(s.obstacle.title,'Fund your airbase work');assert.deepEqual(s.obstacle.action,{kind:'air',page:'bases'});
  a.bases=base({...project,last_paid_day:today-1,paid_bn:.004});route=at();s=step(route,'air_force');
  assert.equal(s.status,'progress');assert.equal(mile(route,'air_force','base_funded').date,iso(today-1));assert.equal(s.obstacle.title,'Keep airbase work funded');assert.match(s.obstacle.reason,/\$0\.004bn of \$0\.024bn/);assert.deepEqual(s.obstacle.action,{kind:'air',page:'bases'},'airbase advice opens the Bases page');
  a.bases=base({...project,last_paid_day:today+1,paid_bn:.004});assert.equal(mile(at(),'air_force','base_funded').status,'pending','a payment after as_of_day is rejected');
  a.bases=base({...project,last_paid_day:today-1,paid_bn:.004,cancelled_day:today-1});s=step(at(),'air_force');assert.equal(s.status,'not_yet');assert.equal(s.obstacle.title,'Fund an airbase foundation');
  a.bases=base(null,[{track:'capacity',target_level:1,started_day:today-14,completed_day:today-2}]);route=at();
  assert.equal(step(route,'air_force').status,'achieved');assert.equal(mile(route,'air_force','base_completed').date,iso(today-2));assert.equal(mile(route,'air_force','base_funded').status,'done');
  a.bases=base(null,[{track:'capacity',target_level:1,started_day:today-14,completed_day:today+2}]);assert.equal(step(at(),'air_force').status,'not_yet');
  for(const bad of [{paid_bn:'0.004'},{paid_bn:-1},{last_paid_day:-1},{last_paid_day:.5},{completed_day:'x'},{cancelled_day:1.5}]){a.bases=base({...project,...bad});assert.equal(step(at(),'air_force').status,'unknown',JSON.stringify(bad));}
  a.bases=[{id:'FR-IDF',project:null}];assert.equal(step(at(),'air_force').status,'unknown','a base without its history list is unreadable');
  a.bases=[];a.squadrons=[{id:1,assigned:12,ready:false,blocker:'No home base.'}];route=at();
  assert.equal(step(route,'air_force').status,'progress');assert.equal(mile(route,'air_force','squadron_formed').status,'done');assert.equal(mile(route,'air_force','squadron_ready').detail,'No home base.');
  a.squadrons=[{id:1,assigned:12,ready:true,blocker:null}];route=at();assert.equal(step(route,'air_force').status,'achieved');assert.equal(step(route,'air_force').summary,'Squadron ready.');
  for(const sq of [{id:1,assigned:0,ready:true,blocker:null},{id:1,assigned:12,ready:true,blocker:'In transit.'}])
    {a.squadrons=[sq];assert.notEqual(step(at(),'air_force').status,'achieved',JSON.stringify(sq));}
  for(const sq of [{id:1,assigned:'12',ready:true,blocker:null},{id:1,assigned:12,ready:'true',blocker:null},{id:1,assigned:12,ready:true},{id:1,assigned:12,ready:true,blocker:4}])
    {a.squadrons=[sq];assert.equal(step(at(),'air_force').status,'unknown',JSON.stringify(sq));}
  a.squadrons=[];a.missions=[{id:1,status:'flown',report_day:today-1,aircraft:4}];route=at();assert.equal(mile(route,'air_force','mission_flown').date,iso(today-1));assert.equal(step(route,'air_force').status,'progress');
  for(const m of [{id:1,status:'flown',report_day:today+1,aircraft:4},{id:1,status:'cancelled',report_day:null,aircraft:null},{id:1,status:'blocked',report_day:today-1,aircraft:0}])
    {a.missions=[m];assert.equal(mile(at(),'air_force','mission_flown').status,'pending',JSON.stringify(m));}
  a.missions=[{id:1,status:'flown',report_day:null,aircraft:4}];assert.equal(mile(at(),'air_force','mission_flown').status,'unknown');
});

test('identity mismatch or an unreadable reading makes every step unknown',()=>{
  const edits=[r=>{delete r.state.session_id;},r=>{r.state.session_id='';},r=>{r.state.session_id=42;},r=>{r.state.session_id='x'.repeat(500);},
    r=>{r.state.player=null;},r=>{r.state.player='USA';},r=>{r.state.nations[0].alive=false;},r=>{r.state.nations[0].alive='true';},r=>{r.state.nations.push({id:'FRA',alive:true});},r=>{r.state.nations=null;},
    r=>{r.state.date='2 Jun 1990';},r=>{r.state.day=2;},r=>{r.state.month=13;},r=>{r.state.year='1990';},r=>{r.state.date='01 Jun 1990';},r=>{r.state.date=null;},
    r=>{r.outcomes.nation='USA';},r=>{r.outcomes.nation='France';},r=>{r.outcomes.as_of_day+=1;},r=>{r.outcomes.as_of_day-=1;},r=>{r.outcomes.as_of_day=String(r.outcomes.as_of_day);},
    r=>{r.outcomes.date='2 Jun 1990';},r=>{delete r.outcomes.date;},r=>{r.outcomes=[];},r=>{r.outcomes='FRA';},r=>{r.state=[];}];
  for(const edit of edits){const r=achieved();edit(r);const route=run(r,loaded());allUnknown(route);assert(!advisor.evaluate(r.state,r.production,r.outcomes,loaded()).some(c=>c.id==='route-next'),edit.toString());}
  allUnknown(run(achieved().outcomes),/No campaign reading/);allUnknown(run(null),/No campaign reading/);allUnknown(run(undefined));
  const r=achieved();r.production={...r.production,nation:'USA',as_of_day:0};assert.equal(run(r,loaded()).status,'ready','production only refines advice and never gates results');
});

test('a disabled or missing section is unknown, never not yet',()=>{
  for(const [key,id] of [['money','finances'],['construction','construction'],['research','research_design'],['procurement','procurement'],['aviation','air_force']]){
    for(const value of [null,undefined,[],'x',0,true]){
      const r=reading();if(value===undefined)delete r.outcomes[key];else r.outcomes[key]=value;const route=contract(run(r));
      assert.equal(route.status,'ready');const s=step(route,id);assert.equal(s.status,'unknown',`${key}=${JSON.stringify(value)}`);assert.equal(s.obstacle,null);assert.match(s.summary,/not enabled/);
      assert(s.milestones.every(m=>m.status==='unknown'));for(const other of ORDER.filter(o=>o!==id))assert.equal(step(route,other).status,'not_yet');}
    const r=reading();r.outcomes[key]={};assert.equal(step(contract(run(r)),id).status,'unknown',`${key}={}`);
  }
});

test('a loaded save re-derives results from native records and needs a matching load receipt',()=>{
  const before=achieved();before.state.session_id='s-1';setDate(before,1992,6,30);
  const receipts={saves:[{session_id:'s-1',player:'FRA',slot:'route',date:'30 Jun 1992',dispatches:400}],loads:[]};
  let route=contract(run(before,receipts));assert.deepEqual(route.steps.map(s=>s.status),['achieved','achieved','achieved','achieved','achieved','progress']);
  assert.equal(step(route,'save_resume').obstacle.title,'Resume from your save');assert.match(step(route,'save_resume').obstacle.reason,/replaces anything played since/);
  const after=structuredClone(before);after.state.session_id='s-2';
  let resumed=contract(run(after,receipts));assert.deepEqual(resumed.steps.slice(0,5),route.steps.slice(0,5),'native results survive the load under the new session');
  assert.equal(step(resumed,'save_resume').status,'not_yet','the old session\'s save receipt is not inherited');
  receipts.loads.push({session_id:'s-2',player:'FRA',slot:'route',backup:false,date:'30 Jun 1992',dispatch_count:400});
  resumed=contract(run(after,receipts));assert.equal(step(resumed,'save_resume').status,'achieved');assert.equal(resumed.as_of.session_id,'s-2');
  assert.equal(mile(resumed,'save_resume','campaign_saved').detail,'Saved "route".','the save that the load restored');assert.equal(mile(resumed,'save_resume','campaign_resumed').detail,'Loaded "route".');
  assert.equal(step(contract(run(before,receipts)),'save_resume').status,'progress','a load receipt for another session never completes this one');
  const cont=structuredClone(after);setDate(cont,1992,7,4);assert.equal(step(run(cont,receipts),'save_resume').status,'achieved','reload and Continue keeps the session, so the receipt still applies');
  const next=structuredClone(after);next.state.session_id='s-3';assert.equal(step(run(next,receipts),'save_resume').status,'not_yet','a new game or campaign has a different session');
  const branch=structuredClone(after);branch.outcomes.procurement.deliveries=[];branch.outcomes.aviation.bases=[];branch.outcomes.aviation.squadrons=[];
  resumed=contract(run(branch,receipts));assert.equal(step(resumed,'procurement').status,'progress');assert.equal(step(resumed,'air_force').status,'progress','records absent from the loaded world are not carried forward');
  const backup={...receipts.loads[0],backup:true,date:'1 Jun 1992'};assert.match(mile(run(after,{saves:receipts.saves,loads:[backup]}),'save_resume','campaign_resumed').detail,/from its backup/);
  assert.equal(mile(run(after,{saves:receipts.saves,loads:[backup]}),'save_resume','campaign_saved').status,'pending','the backup restored a different date than the recorded save');
});

test('receipts from another session, player or later date do not count',()=>{
  const r=reading(),today=r.state.date,s=r.state.session_id;
  const save={session_id:s,player:'FRA',slot:'main',date:today,dispatches:9},load={session_id:s,player:'FRA',slot:'main',backup:false,date:today,dispatch_count:9};
  const at=(saves,loads)=>{const route=contract(run(r,{saves,loads}));return [mile(route,'save_resume','campaign_saved').status,mile(route,'save_resume','campaign_resumed').status];};
  assert.deepEqual(at([save],[]),['done','pending']);assert.deepEqual(at([save],[load]),['done','done']);
  assert.deepEqual(at([{...save,session_id:'other'}],[]),['pending','pending']);
  assert.deepEqual(at([{...save,player:'USA'}],[]),['pending','pending']);
  assert.deepEqual(at([{...save,date:'2 Jun 1990'}],[]),['pending','pending']);
  assert.deepEqual(at([],[{...load,session_id:'other'}]),['pending','pending']);
  assert.deepEqual(at([],[{...load,player:'USA'}]),['pending','pending']);
  assert.deepEqual(at([],[{...load,date:'2 Jun 1990'}]),['pending','pending']);
  assert.deepEqual(at([{...save,session_id:'other',slot:'main'}],[{...load,slot:'other-slot'}]),['pending','done'],'a foreign save counts only when it is the save this load restored');
  for(const bad of [{backup:'false'},{backup:undefined},{dispatch_count:-1},{dispatch_count:'9'},{date:'1990-06-01'},{date:'31 Jun 1990'},{slot:''},{slot:'x'.repeat(300)},{session_id:7},{player:null}])
    assert.deepEqual(at([],[{...load,...bad}]),['pending','pending'],JSON.stringify(bad));
  for(const bad of [{dispatches:undefined},{dispatches:1.5},{date:'Jun 1 1990'},{session_id:''}])assert.deepEqual(at([{...save,...bad}],[]),['pending','pending'],JSON.stringify(bad));
  for(const receipts of [null,undefined,'x',[],7]){const st=step(contract(advisor.recognize(r,receipts)),'save_resume');assert.equal(st.status,'unknown');assert.match(st.summary,/not available/);}
  assert.equal(step(run(r,{saves:[save]}),'save_resume').status,'unknown','missing load receipts cannot prove a resume did not happen');
  assert.deepEqual(at('x',[load]),['unknown','done']);
});

test('receipts are bounded to the newest twenty and read only own data properties',()=>{
  const r=reading(),s=r.state.session_id,date=r.state.date,load={session_id:s,player:'FRA',slot:'main',backup:false,date,dispatch_count:9},junk=i=>({session_id:`old-${i}`,player:'FRA',slot:'x',backup:false,date,dispatch_count:1});
  const resumed=loads=>step(run(r,{saves:[],loads}),'save_resume').status;
  assert.equal(resumed([load,...Array.from({length:20},(_,i)=>junk(i))]),'not_yet','only the newest twenty receipts are read');
  assert.equal(resumed([...Array.from({length:24},(_,i)=>junk(i)),load]),'achieved');
  let calls=0;const getter={...load};Object.defineProperty(getter,'session_id',{enumerable:true,get(){calls++;return s;}});
  assert.equal(resumed([getter]),'not_yet');assert.equal(calls,0,'an accessor is never invoked');
  const inherited=Object.create(load);assert.equal(resumed([inherited]),'not_yet','inherited fields are ignored');
  const receipts={saves:[]};Object.defineProperty(receipts,'loads',{enumerable:true,get(){calls++;return [load];}});
  assert.equal(step(run(r,receipts),'save_resume').status,'unknown');assert.equal(calls,0);
  assert.equal(step(run(r,Object.create({saves:[],loads:[load]})),'save_resume').status,'unknown');
  const sparse=[];sparse.length=2**32-1;sparse[2**32-2]=load;assert.equal(resumed(sparse),'achieved','a huge sparse list is read from its newest end only');
  const holes=[];holes.length=1e9;assert.equal(resumed(holes),'not_yet');
  const hostile=new Proxy([load],{getOwnPropertyDescriptor(){throw new Error('boom');}});
  const route=contract(run(r,{saves:[],loads:hostile}));assert.equal(route.status,'ready');assert.equal(step(route,'save_resume').status,'unknown');assert.equal(step(route,'finances').status,'not_yet','a hostile receipt list affects only its own step');
});

test('hostile, huge, sparse and accessor inputs are bounded and never throw',()=>{
  let calls=0;const trap=new Proxy({},{get(){calls++;throw new Error('get');},getOwnPropertyDescriptor(){calls++;throw new Error('descriptor');},ownKeys(){throw new Error('keys');},getPrototypeOf(){throw new Error('proto');},has(){throw new Error('has');}});
  allUnknown(run(trap,trap),/could not be checked/);allUnknown(run({state:trap,outcomes:trap}));
  const {proxy,revoke}=Proxy.revocable({},{});revoke();allUnknown(run(proxy,proxy));allUnknown(run({state:reading().state,outcomes:proxy}));
  const r=reading();assert.doesNotThrow(()=>advisor.evaluate(r.state,r.production,trap,trap));assert(!advisor.evaluate(r.state,r.production,trap,trap).some(c=>c.id==='route-next'));
  calls=0;const g=reading();Object.defineProperty(g,'outcomes',{enumerable:true,get(){calls++;return achieved().outcomes;}});allUnknown(run(g),/no campaign results/);assert.equal(calls,0);
  const m=reading();Object.defineProperty(m.outcomes,'money',{enumerable:true,get(){calls++;return {journal_available:true,decisions:[{id:1,day:0,kind:'program_budget'}]};}});
  assert.equal(step(run(m),'finances').status,'unknown');assert.equal(calls,0,'section accessors are never invoked');
  const inherited=reading();inherited.outcomes=Object.assign(Object.create({money:achieved().outcomes.money}),{...inherited.outcomes});delete inherited.outcomes.money;
  assert.equal(step(run(inherited),'finances').status,'unknown','prototype fields are not campaign records');
  const sparse=reading();sparse.outcomes.construction.projects=[];sparse.outcomes.construction.projects.length=2**32-1;
  let t=Date.now();assert.equal(step(run(sparse),'construction').status,'unknown');assert(Date.now()-t<2000,'a huge sparse list is bounded');
  const many=reading();many.outcomes.procurement.deliveries=Array.from({length:5000},(_,i)=>({id:i,revision:'r',quantity:1,purchased_day:0,settled_day:null,due_day:null,delivered_day:null,status:'awaiting_settlement'}));
  t=Date.now();let route=contract(run(many));assert.equal(step(route,'procurement').status,'unknown','rows beyond the bound cannot prove a delivery did not happen');assert.equal(mile(route,'procurement','purchase_placed').status,'done');assert(Date.now()-t<2000);
  const arrayLike=reading();arrayLike.outcomes.aviation.bases={length:1,0:{id:'FR-IDF',project:null,history:[]}};assert.equal(step(run(arrayLike),'air_force').status,'unknown');
  const holes=reading();holes.outcomes.research.drafts=[,{name:'x',updated_day:3,valid:true}];assert.equal(step(run(holes),'research_design').status,'achieved');
  holes.outcomes.research.drafts=[,];assert.equal(step(run(holes),'research_design').status,'unknown','a hole is missing data, not an empty list');
  const text=reading();text.outcomes.construction.completions=[{date:'1 Jan 1990',day:0,text:'France completes '+'x'.repeat(1e6)+' in FR-IDF.',district:null,kind:null}];
  route=contract(run(text));assert(mile(route,'construction','project_completed').detail.length<=120);
  const cyclic=reading();cyclic.outcomes.money.self=cyclic.outcomes;cyclic.state.self=cyclic;assert.equal(contract(run(cyclic)).status,'ready');
  const weird=reading();weird.outcomes.construction.projects=[{id:Symbol('x'),last_day:()=>1,last_spent_bn:1n}];assert.doesNotThrow(()=>run(weird));assert.equal(step(run(weird),'construction').status,'unknown');
  const big=reading();big.outcomes.as_of_day=1e300;allUnknown(run(big));
});

test('the route is deterministic, deeply frozen and retains no caller object',()=>{
  const r=achieved(),receipts=loaded(),snapshot=JSON.stringify([r,receipts]);freezeAll(r);freezeAll(receipts);
  const a=run(r,receipts),b=run(r,receipts);assert.equal(a.status,'ready','frozen inputs are only read');assert.deepEqual(a,b);assert.notEqual(a,b);assert.equal(JSON.stringify([r,receipts]),snapshot);
  assert(deepFrozen(a));assert.throws(()=>{a.steps[0].status='achieved';},TypeError);assert.throws(()=>{a.steps.push({});},TypeError);assert.throws(()=>{a.steps[3].milestones[0].day=1;},TypeError);
  const inputs=objects([r,receipts]);for(const o of objects(a))assert(!inputs.has(o),'no input object appears in the route');
  const live=achieved(),mine=loaded(),before=JSON.stringify(run(live,mine)),route=run(live,mine);
  live.outcomes.money.decisions[2].day=0;live.outcomes.procurement.deliveries.length=0;live.state.session_id='changed';mine.loads.length=0;
  assert.equal(JSON.stringify(route),before,'later changes to inputs do not reach an earlier route');
  const unknown=run(null);assert(deepFrozen(unknown));assert.throws(()=>{unknown.steps[0].obstacle={};},TypeError);
  const src=fs.readFileSync(path.join(__dirname,'../../spheres-web/ui/advisor-model.js'),'utf8');
  assert.doesNotMatch(src,/Date\.now|new Date\(\)|localStorage|sessionStorage|indexedDB|fetch\(|XMLHttpRequest|setTimeout|setInterval|Math\.random|\/api\/command/);
});

test('evaluate adds at most one route card, for the first unachieved step with an obstacle',()=>{
  const cards=(r,receipts)=>advisor.evaluate(r.state,r.production,r.outcomes,receipts),route=list=>list.filter(c=>c.id==='route-next');
  const r=reading();let got=route(cards(r,NONE));assert.equal(got.length,1);
  assert.deepEqual(Object.keys(got[0]),['id','area','priority','title','reason','evidence','caution','action','actionLabel']);
  assert.deepEqual([got[0].area,got[0].priority,got[0].title,got[0].actionLabel],['economy','opportunity','Enact this year\'s budget','Review yearly budget']);
  assert.deepEqual(got[0].action,{kind:'budget'});assert.deepEqual(got[0].evidence,['First-hour step: Enact a yearly budget.','Not yet achieved in this campaign.']);assert.match(got[0].caution,/sends no order/);
  got[0].action.kind='tampered';assert.deepEqual(route(cards(r,NONE))[0].action,{kind:'budget'},'each card carries a fresh action');
  r.state.nations[0].annual_budget.due=true;const due=cards(r,NONE);assert(due.some(c=>c.id==='annual-budget'));assert.equal(route(due).length,0,'an open budget card already covers the first step');
  r.state.nations[0].annual_budget.due=false;
  const full=achieved(),o=full.outcomes,steps=[
    [x=>{x.outcomes.construction=reading().outcomes.construction;},'economy',{kind:'construction'}],
    [x=>{x.outcomes.research=reading().outcomes.research;},'research',{kind:'equipment',tab:'designer'}],
    [x=>{x.outcomes.procurement={...o.procurement,deliveries:[]};},'military',{kind:'companies'}],
    [x=>{x.outcomes.aviation={bases:[],squadrons:[],missions:[]};},'military',{kind:'air',page:'bases'}],
    [x=>{x.outcomes.procurement=null;x.outcomes.aviation={bases:[],squadrons:[],missions:[]};},'military',{kind:'air',page:'bases'}]];
  for(const [edit,area,action] of steps){const x=achieved();edit(x);got=route(cards(x,loaded()));assert.equal(got.length,1,JSON.stringify(action));assert.equal(got[0].area,area);assert.deepEqual(got[0].action,action);}
  const unsaved=achieved();unsaved.state.session_id='s-9';got=route(cards(unsaved,NONE));assert.equal(got.length,1);assert.equal(got[0].area,'economy');assert.deepEqual(got[0].action,{kind:'campaign'});
  assert.equal(route(cards(unsaved)).length,0,'without receipts the save step is unknown and gets no card');
  assert.equal(route(cards(achieved(),loaded())).length,0,'achieved steps never get a route card');
  const early=reading();early.outcomes.money.decisions=[{id:1,day:0,kind:'program_budget'}];early.outcomes.construction.projects=[{id:9,kind:'power_grid',district:'FR-IDF',last_day:null,last_spent_bn:null}];
  got=route(cards(early,NONE));assert.deepEqual(got[0].action,{kind:'project',id:9});assert.equal(got[0].title,'Your project awaits paid work');
  const mixed=reading();mixed.outcomes.money.decisions=[{id:1,day:0,kind:'program_budget'}];mixed.state.nations[0].treasury=-1;mixed.state.fiscal_recovery={recovery_required:true,status:'crisis'};const list=cards(mixed,NONE);
  assert.equal(route(list).length,1);assert.deepEqual(route(list)[0].action,{kind:'construction'});assert.equal(list[list.length-1].id,'route-next','route advice ranks after attention cards');assert(list.filter(c=>c.priority==='attention').length>=2);
  for(const bad of [null,undefined,{...r.outcomes,nation:'USA'},{...r.outcomes,as_of_day:0}]){const x=reading();x.outcomes=bad;assert.equal(route(cards(x,NONE)).length,0);}
  assert.equal(route(advisor.evaluate(r.state,r.production)).length,0);
});

test('the browser UMD build exposes recognize without DOM, clock or storage',()=>{
  const context={};vm.runInNewContext(fs.readFileSync(path.join(__dirname,'../../spheres-web/ui/advisor-model.js'),'utf8'),context);
  const route=context.AdvisorModel.recognize(achieved(),loaded());assert.equal(route.status,'ready');assert.equal(route.steps.map(s=>s.status).join(),Array(6).fill('achieved').join());
  assert.equal(context.AdvisorModel.recognize(null).status,'unknown');
});

function setDate(r,y,m,d){const day=DAY(y,m,d),date=`${d} ${MON[m-1]} ${y}`;Object.assign(r.state,{year:y,month:m,day:d,date,t:(y-1990)*12+m-1});r.outcomes.date=date;r.outcomes.as_of_day=day;r.production.date=date;r.production.as_of_day=day;return r;}
