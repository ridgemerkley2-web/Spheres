// Run: node --test tools/ui/check_military_operations.cjs
const assert = require('node:assert/strict');
const { test } = require('node:test');
const fs = require('node:fs');
const path = require('node:path');
const vm = require('node:vm');
const context = vm.createContext({});
vm.runInContext(fs.readFileSync(path.join(__dirname, '../../spheres-web/ui/operations-ui.js'), 'utf8'), context);
const ui = context.MilitaryOperationsUI;
const data = { enabled: true, structure: 10, deployed: 3, reserve: 7, overseas_deployed: 1,
  overseas_limit: 2, capabilities: {land: 1, strike: .8, lift: .9}, deployments: [
    { conflict: 4, overseas: true, allocation_bp: null, deployed: 1, effective_force: .9, burn_monthly: .02 },
    { conflict: 5, overseas: false, allocation_bp: 1234, deployed: 2, effective_force: 2, burn_monthly: .04 },
  ] };
test('live national totals, per-theatre ceilings and peaceful reserves remain visible', () => {
  assert.equal(ui.html({enabled: false}), '');
  const html = ui.html(data, [{id:4, theatre_name:'<img src=x onerror="alert(1)">'}]);
  assert(html.includes('7.00'));
  assert(html.includes('value="1234" selected'));
  assert(html.includes('12.34%'));
  assert(html.includes('&lt;img'));
  assert(!html.includes('<img'));
  const focused = ui.html(data, [], 4);
  assert(focused.includes('data-force-id="4"'));
  assert(!focused.includes('data-force-id="5"'));
  assert(ui.html({...data, deployments:[]}).includes('Your force remains in reserve'));
});
function formFixture(value, send) {
  const button = {disabled:false}, status = {textContent:''}, select = {value};
  const form = {dataset:{forceId:'4'}, querySelector:s => s === 'button' ? button : s === 'select' ? select : status};
  ui.bind({querySelectorAll:()=>[form]}, send);
  return {form, button, status, submit:()=>form.onsubmit({preventDefault(){}})};
}
test('zero and automatic allocations send distinct exact commands', async () => {
  for (const [value, share] of [['0', 0], ['', null], ['1234',1234]]) {
    const calls = [], f = formFixture(value, async cmd => calls.push(JSON.parse(JSON.stringify(cmd))));
    await f.submit();
    assert.deepEqual(calls, [{kind:'force_allocation', conflict:4, share_bp:share}]);
    assert.equal(f.button.disabled, false);
    assert.equal(f.status.textContent, 'Allocation applied.');
  }
});
test('invalid input is rejected before dispatch', async () => {
  for (const value of ['-1', '10001', '1.5', 'no']) {
    let sent = false; const f = formFixture(value, async()=>{sent=true;});
    await f.submit(); assert.equal(sent,false); assert.match(f.status.textContent,/valid force ceiling/);
  }
});
test('one in-flight order, with recoverable failure feedback', async () => {
  let reject, calls=0;
  const f = formFixture('500', () => { calls++; return new Promise((_,r)=>{reject=r;}); });
  const first = f.submit();
  assert.equal(f.button.disabled,true);
  await f.submit(); assert.equal(calls,1);
  reject(new Error('The conflict ended.')); await first;
  assert.equal(f.button.disabled,false);
  assert.equal(f.status.textContent,'The conflict ended.');
});

test('physical stores override fleet potential without hiding movement or reconnaissance', () => {
  const capabilities={land:1,strike:.8,lift:.9,ground_roles:{fire_support:.25,protected_mobility:.14,reconnaissance:.17,air_defense:.35}};
  const deployment={...data.deployments[0],capabilities,ammunition:{fire_support:0,air_defense:0,fire_fraction:0,maneuver_fraction:1,physical_dry:true}};
  const html=ui.html({...data,deployments:[deployment]},[],4);
  assert.match(html,/Find targets<\/dt><dd>17\.0%/);
  assert.match(html,/Protected movement<\/dt><dd>14\.0%/);
  assert.match(html,/Fire support<\/dt><dd>0\.0%/);
  assert.match(html,/Air defense<\/dt><dd>0\.0%/);
  assert.match(html,/no usable firing supply/);
  assert.match(html,/already included/);
  assert(!html.includes('25.0%'));assert(!html.includes('35.0%'));
  const supplied=ui.html({...data,deployments:[{...deployment,ammunition:{...deployment.ammunition,fire_support:.08,air_defense:.12,fire_fraction:.3,physical_dry:false}}]});
  assert.match(supplied,/Fire support<\/dt><dd>8\.0%/);
  assert.match(supplied,/Air defense<\/dt><dd>12\.0%/);
  assert.match(supplied,/Firing support: 30\.0%/);
  assert(!supplied.includes('no usable firing supply'));
  const legacy=ui.html({...data,deployments:[{...deployment,ammunition:null}]});
  assert.match(legacy,/Fire support<\/dt><dd>25\.0%/);
  assert.match(legacy,/shared magazine system/);
});

test('standoff strike commitments do not claim ground specialist effects',()=>{
  const row={...data.deployments[0],rung:6,capabilities:{ground_roles:{fire_support:.2,protected_mobility:.25,reconnaissance:.15,air_defense:.35}}};
  const html=ui.html({...data,deployments:[row],ground_fleet:{title:'National holdings',models:[],actions:[]}});
  assert(!html.includes('military-ground-roles'));assert(!html.includes('Find targets'));assert(!html.includes('Protected movement'));
  assert(html.includes('National holdings'),'Ground holdings remain inspectable while the commitment uses strikes');
});

test('current holdings and dated national loss receipts remain distinct and escape model names', () => {
  const fleet={title:'Ground fleet',detail:'National holdings',maintenance:{coverage:.6,recorded_day:12},
    models:[{revision:'ifv-1',name:'<img src=x>',role:'Protected infantry',delivered:4,available:2,reserved:2,ammunition_family:'30 mm',supported_fraction:.48}],
    actions:[{label:'Review maintenance',tab:'service'},{label:'Browse stock',tab:'companies'},{label:'Unsafe',tab:'javascript:x'}],
    last_report:{day:10,day_label:'11 January 1990',conflicts:[4,5],revisions:[{revision_id:'ifv-1',name:'<img src=x>',role:'Protected infantry',opening_delivered:10,opening_available:8,opening_reserved:2,lost:1,remaining_delivered:9,remaining_available:7,remaining_reserved:2}]} };
  const html=ui.html({...data,ground_fleet:fleet});
  assert.match(html,/Committed<\/dt>/);assert.match(html,/forces must still travel/i);
  assert.match(html,/Owned<\/dt><dd>4/);assert.match(html,/Condition and upkeep support: 48\.0%/);
  assert.match(html,/capability factor, not a count/);
  assert.match(html,/Last settled fleet support: 60\.0%/);
  assert.match(html,/11 January 1990/);assert.match(html,/across conflicts #4, #5/);
  assert.match(html,/Vehicles lost<\/dt><dd>1/);assert.match(html,/Available after combat<\/dt><dd>7/);
  assert.match(html,/Protected in refit<\/dt><dd>2/);assert.match(html,/Owned at settlement: 10 → 9/);
  assert.match(html,/Current holdings are shown above/);
  assert.match(html,/&lt;img/);assert(!html.includes('<img'));assert(!html.includes('javascript:x'));
  assert.match(html,/data-ground-equipment-tab="companies"/);
  const fresh=ui.html({...data,ground_fleet:{...fleet,last_report:null,maintenance:{coverage:1,recorded_day:null}}});
  assert.match(fresh,/Maintenance has not settled yet/);assert.match(fresh,/No ground-equipment loss settlement/);
  assert(!fresh.includes('Last settled fleet support: 100.0%'));
});

test('allocation blocks a second form and uncertain responses never report success', async () => {
  let resolve,calls=0;
  const first=formFixture('500',()=>{calls++;return new Promise(r=>{resolve=r;});});
  const second=formFixture('1000',async()=>{calls++;});
  const pending=first.submit();await second.submit();assert.equal(calls,1);
  assert.match(second.status.textContent,/Wait for the current order/);
  resolve({command_pending:true});await pending;
  assert.match(first.status.textContent,/awaits confirmation/);assert.equal(first.button.disabled,false);
  const rejected=formFixture('500',async()=>({errors:['Conflict closed.']}));await rejected.submit();
  assert.equal(rejected.status.textContent,'Conflict closed.');
});

test('equipment shortcuts pass only supported destinations and surface host refusal', async () => {
  const status={textContent:''},buttons=['service','ammunition','companies','unknown'].map(tab=>({dataset:{groundEquipmentTab:tab},disabled:false}));
  const calls=[],container={querySelectorAll:s=>s==='[data-ground-equipment-tab]'?buttons:[],querySelector:()=>status};
  ui.bind(container,async()=>{},async tab=>{calls.push(tab);if(tab==='companies')throw Error('The campaign changed.');});
  for(const button of buttons)await button.onclick();
  assert.deepEqual(calls,['service','ammunition','companies']);assert.equal(status.textContent,'The campaign changed.');
  assert(buttons.every(b=>b.disabled===false));
});
