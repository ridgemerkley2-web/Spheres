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
