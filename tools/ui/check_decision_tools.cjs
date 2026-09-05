const test=require('node:test'),assert=require('node:assert/strict');
const d=require('../../spheres-web/ui/decision-tools.js');
test('shared labels reserve the entire text rectangle across layers',()=>{assert(d.overlap([10,10,40,20],[35,15,10,10]));assert(!d.overlap([10,10,40,20],[100,100,10,10]));});
test('finder finds owned provinces without requiring a globe hit',()=>{const rows=d.search([{id:'USA',name:'United States',alive:true}],{'US-CA':{name:'California'}},()=> 'USA','California','USA');assert.equal(rows[0].id,'US-CA');assert.equal(rows[0].kind,'province');});
test('advisor observes funding, missing inputs and completed output in order',()=>{assert.equal(d.guide({enabled:false},{}).step,0);assert.equal(d.guide({enabled:true},{queue:[{status:'blocked',reason:'No steel'}]}).step,2);assert.equal(d.guide({enabled:true},{completed:[{}]}).step,4);});
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
