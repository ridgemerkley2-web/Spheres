const test=require('node:test'),assert=require('node:assert/strict');
const d=require('../../spheres-web/ui/decision-tools.js');
test('shared labels reserve the entire text rectangle across layers',()=>{assert(d.overlap([10,10,40,20],[35,15,10,10]));assert(!d.overlap([10,10,40,20],[100,100,10,10]));});
test('finder finds owned provinces without requiring a globe hit',()=>{const rows=d.search([{id:'USA',name:'United States',alive:true}],{'US-CA':{name:'California'}},()=> 'USA','California','USA');assert.equal(rows[0].id,'US-CA');assert.equal(rows[0].kind,'province');});
test('advisor distinguishes annual renewal, unopened funding and an intentional daily pause',()=>{
  const renewal=d.guide({enabled:true,due:true},{construction_budget:{enrolled:true,daily_budget_bn:0}});
  assert.equal(renewal.action,'budget');assert.match(renewal.title,/yearly budget/);
  const unopened=d.guide({enabled:false},{construction_budget:{enrolled:false,daily_budget_bn:.01}});
  assert.equal(unopened.action,'works');assert.match(unopened.title,/Open construction funding/);
  const paused=d.guide({enabled:true},{construction_budget:{enrolled:true,daily_budget_bn:0}});
  assert.equal(paused.step,2);assert.match(paused.title,/paused/);assert.match(paused.text,/Choose a positive limit when/);
});
test('advisor fixes blocked work before the server suggestion and uses that suggestion before healthy work or old assets',()=>{
  const suggestion={id:'grid',name:'Local grid',district:'US-CA',project_kind:'power_grid',reason:'Grid limits the existing factory.'};
  const work={construction_budget:{enrolled:true,daily_budget_bn:.01,available_bn:.01},
    suggestions:{items:[suggestion]},queue:[{id:1,status:'slowed',reason:'Daily funding covers half the work.'}],completed:[{}]};
  const before=JSON.stringify(work),blocked=d.guide({enabled:true},work);
  assert.equal(blocked.project,work.queue[0]);assert.equal(blocked.text,work.queue[0].reason);assert.equal(blocked.action,'works');
  work.queue[0].status='building';const recommended=d.guide({enabled:true},work);
  assert.equal(recommended.suggestion,suggestion);assert.equal(recommended.text,suggestion.reason);assert.equal(recommended.action,'suggestion');
  assert.equal(JSON.stringify({...work,queue:[{...work.queue[0],status:'slowed'}]}),before,'Advice must not edit the queue or recommendation');
});
test('advisor preserves server suggestion order and exact workshop size without scoring candidates',()=>{
  const first={id:'small',name:'Small workshop',project_kind:'starter_industry',district:'US-CA',capacity_micros:5001};
  const second={id:'other',name:'More expensive grid',project_kind:'power_grid',district:'US-NV'};
  const work={suggestions:{items:[{...second,eligible:false},first,second]}};
  assert.equal(d.guide({enabled:true},work).suggestion,first);
  assert.equal(d.guide({enabled:true},work).suggestion.capacity_micros,5001);
});
test('advisor falls through to healthy work, operation and exploration only after funding and suggestions',()=>{
  const p={id:1,status:'building'};
  assert.equal(d.guide({enabled:true},{queue:[p]}).project,p);
  assert.equal(d.guide({enabled:true},{completed:[{}]}).action,'economy');
  assert.equal(d.guide({enabled:true},{suggestions:{items:[],note:'No demonstrated construction bottleneck.'}}).text,'No demonstrated construction bottleneck.');
  const emptyCash=d.guide({enabled:true},{construction_budget:{daily_budget_bn:.01,available_bn:0},queue:[p]});
  assert.match(emptyCash.title,/available construction funding/);
});
test('advisor progress matches the construction queue without hiding small paid work',()=>{
  for(const [input,text] of [[0,'0%'],[-0,'0%'],[.00125,'0.1%'],[.00001,'<0.1%'],[.4567,'45.7%'],[1,'100%'],[null,'—'],[NaN,'—']])assert.equal(d.progressText(input),text);
});
test('research list keeps full names, filters available prerequisites, and sorts focus first',()=>{const nodes=[{id:'a',domain:'Energy',name:'A long discovery',state:'open',year:1990},{id:'b',domain:'Energy',name:'Other',state:'locked',year:1980},{id:'c',domain:'Energy',name:'Focused',state:'open',focus:true,year:2000}];assert.deepEqual(d.research(nodes,'all','','available').map(n=>n.id),['c','a']);assert.equal(d.research(nodes,'all','long','all')[0].id,'a');});

test('Advisor growth drags keep the subtraction used by the simulation',()=>{
  const rows=d.causes({war:.03,sanctions:.02,oil:-.01,demand_output_now:.005});
  assert.deepEqual(rows.map(r=>[r.key,r.value]),[['war',-.03],['sanctions',-.02],['oil',-.01],['demand_output_now',.005]]);
  assert.deepEqual(d.causes({war:NaN,debt_drag:Infinity}),[]);
});
test('stability explains served signed points and scope without calculating a second economic formula',()=>{
  const policy={stability:{monthly_points_before_bounds:-.1234,step_points_before_bounds:-.0042,month_fraction:1/31,
    terms:[{id:'inflation',label:'Inflation above 5%',monthly_points:-.2,action:'decisions'},
      {id:'housing',label:'Housing budget versus baseline',monthly_points:.08,action:'budget'},
      {id:'mean_reversion',label:'Gradual return toward 60 stability',monthly_points:0,action:''}]}};
  const before=JSON.stringify(policy),html=d.stabilityHtml(policy);
  assert.match(html,/-0\.1234 stability points per month/);assert.match(html,/-0\.0042 points before bounds/);
  assert.match(html,/Inflation above 5%: -0\.2000 points\/month/);assert.match(html,/Housing budget versus baseline: \+0\.0800 points\/month/);
  assert.match(html,/political events, war outcomes and other direct changes are separate/);
  assert.match(html,/data-stability-action="decisions"/);assert.match(html,/data-stability-action="budget"/);
  assert.equal(JSON.stringify(policy),before);
});
test('stability display handles missing data and escapes labels without trusting action names',()=>{
  assert.equal(d.stabilityHtml({}), '');
  const html=d.stabilityHtml({stability:{monthly_points_before_bounds:0,month_fraction:1,terms:[{label:'<script>bad</script>',monthly_points:1,action:'<script>'}]}});
  assert.match(html,/&lt;script&gt;bad&lt;\/script&gt;/);assert.doesNotMatch(html,/<script>|data-stability-action|daily step/);
});
