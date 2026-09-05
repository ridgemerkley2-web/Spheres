// Run from any directory: node tools/ui/check_cabinet.cjs
// Read-only regression checks against the REAL page's helpers. No browser,
// server, save file, external package or copied economic model is involved.
const assert = require('node:assert/strict');
const fs = require('node:fs');
const path = require('node:path');
const vm = require('node:vm');
const { test } = require('node:test');

const root = path.resolve(__dirname, '../..');
const page = fs.readFileSync(path.join(root, 'spheres-web/ui/index.html'), 'utf8');

// These named page helpers are top-level declarations with a column-zero
// closing brace. Extract their actual source instead of maintaining a second
// implementation in a fixture. A changed declaration fails loudly.
function functionSource(name) {
  const match = new RegExp(`^(?:async\\s+)?function\\s+${name}\\(`, 'm').exec(page);
  assert(match, `The page must declare ${name}`);
  const firstLine = page.slice(match.index, page.indexOf('\n', match.index));
  if (/\}\s*$/.test(firstLine)) return firstLine;
  const end = page.indexOf('\n}', match.index);
  assert(end >= 0, `${name} must have its top-level closing brace`);
  return page.slice(match.index, end + 2);
}

function ministrySource() {
  const start = page.indexOf('const MINISTRIES = [');
  const end = page.indexOf('\n];', start);
  assert(start >= 0 && end > start, 'The ministry presentation list must exist');
  return page.slice(start, end + 3);
}

function fixture(names) {
  const elements = new Map();
  const document = { activeElement: null };
  function element(id) {
    if (elements.has(id)) return elements.get(id);
    const classes = new Set();
    const result = {
      id, textContent: '', innerHTML: '', inert: false, isConnected: true,
      tabIndex: 0, attributes: {}, items: [], dataset: {}, hidden: false, scrollTop: 0,
      classList: {
        contains: name => classes.has(name), add: name => classes.add(name),
        remove: name => classes.delete(name),
      },
      setAttribute(name, value) { this.attributes[name] = value; },
      getAttribute(name) { return this.attributes[name]; },
      querySelector: () => null,
      querySelectorAll() { return this.items; },
      getClientRects: () => [1],
      focus() { document.activeElement = result; },
    };
    elements.set(id, result);
    return result;
  }
  document.querySelectorAll = selector => {
    if (selector === '.game-drawer.open') return [...elements.values()].filter(e => e.classList.contains('open') && e.id.endsWith('Drawer'));
    if (selector === '[data-drawer]') return [element('economyDock')];
    if (selector === '[data-cab-tab]') return ['overview', 'budget', 'policy'].map(tab => {
      const button = element(`cab-tab-${tab}`); button.dataset.cabTab = tab; return button;
    });
    if (selector === '.cab-page') return ['overview', 'budget', 'policy'].map(tab => element(`cabinet-${tab}`));
    return [];
  };
  const context = vm.createContext({
    assert,
    document,
    element,
    $: selector => element(selector.replace(/^#/, '')),
    escText: text => String(text).replace(/&/g, '&amp;').replace(/</g, '&lt;'),
    clamp: (value, low, high) => Math.min(high, Math.max(low, value)),
    fmt: {
      bn: value => `$${value}bn`,
      money: value => `$${Number(value.toFixed(6))}bn`,
      pct: (value, places = 1) => `${(value * 100).toFixed(places)}%`,
      pp: value => `${value >= 0 ? '+' : ''}${value * 100}pp`,
    },
    statRow: (label, value) => `<row>${label}: ${value}</row>`,
    noteQueued: () => { context.notes += 1; },
    renderLeft: () => { context.renders += 1; },
    notes: 0,
    renders: 0,
  });
  const helpers = [...new Set(['cabinetIsOpen', ...names])];
  vm.runInContext(`${ministrySource()}
    const CAB = {tab:'overview', ministry:'health', lastFocus:null, busy:false, error:''};
    const ARCADE_ROOMS = {sheetFocus:null, worldFocus:null, helpFocus:null};
    let queued = [];
    let advancing = false;
    let pendingAdvance = null;
    let S = { year: 1991, ministries: {
      curve_step: 0.005, curve_zero: 2,
      ministries: MINISTRIES.map((m, i) => ({id:m.id, index:i, cap:0.15, reference:0.02}))
    }};
    const m = { gdp:1000, political_capital:100, rate:0.08, tax:0.25, annual_budget:{fiscal_year:1990, due:true} };
    MINISTRIES.forEach(spec => { m.annual_budget[spec.id] = 0.02; });
    function me() { return m; }
    ${helpers.map(functionSource).join('\n')}
  `, context, { filename: 'cabinet-helpers-from-index.html' });
  return context;
}

function evaluate(context, code) {
  return vm.runInContext(code, context, { timeout: 1000 });
}

function plain(context, code) {
  return JSON.parse(evaluate(context, `JSON.stringify(${code})`));
}

test('annual drafts adopt the current fiscal year without changing the saved draft', () => {
  const c = fixture(['annualBudgetOf']);
  const inherited = plain(c, 'annualBudgetOf(m)');
  assert.equal(inherited.kind, 'annual_budget');
  assert.equal(inherited.fiscal_year, 1991);
  assert.equal(Object.keys(inherited).length, 12, 'all ten ministries, kind and fiscal year');
  evaluate(c, 'queued = [{...annualBudgetOf(m), fiscal_year:1990, health:0.04}]');
  assert.equal(evaluate(c, 'annualBudgetOf(m).health'), 0.04);
  assert.equal(evaluate(c, 'annualBudgetOf(m).fiscal_year'), 1991);
  assert.equal(evaluate(c, 'queued[0].fiscal_year'), 1990, 'reading a draft is not a mutation');
});

test('Welfare keeps the existing Pensions budget key and changes only its own allocation', () => {
  const c = fixture(['annualBudgetOf', 'annualSocial', 'annualInvest', 'ministryServed',
    'ministryCap', 'queueBudgetDial', 'proposed']);
  const ministries = plain(c, 'MINISTRIES');
  assert.equal(ministries.length, 10);
  assert.equal(ministries[3].id, 'pensions', 'saved drafts and posted commands keep the stable key');
  assert.equal(ministries[3].name, 'Welfare');
  assert.match(ministries[3].story, /income support/);
  const before = plain(c, 'annualBudgetOf(m)');
  evaluate(c, 'queueBudgetDial(m, MINISTRIES[3].id, 0.005)');
  const after = plain(c, 'annualBudgetOf(m)');
  assert(Math.abs(after.pensions - before.pensions - 0.005) < 1e-12);
  for (const key of Object.keys(before).filter(key => key !== 'pensions')) {
    assert.deepEqual(after[key], before[key], `${key} must not change with Welfare`);
  }
  assert.equal(Object.hasOwn(after, 'welfare'), false, 'do not introduce an unparsed command key');
  assert.equal(evaluate(c, 'queued.filter(order => order.kind === "annual_budget").length'), 1);
  assert.equal(evaluate(c, 'm.annual_budget.pensions'), before.pensions, 'drafting is not spending');
});

test('ministry presses preserve tax, rate and unrelated orders and one atomic annual draft', () => {
  const c = fixture(['annualBudgetOf', 'annualSocial', 'annualInvest', 'ministryServed',
    'ministryCap', 'queueBudgetDial', 'proposed']);
  evaluate(c, `queued = [
    {kind:'rate', value:0.05}, {kind:'tax', value:0.32},
    {kind:'sanction', target:'Canada'}, {kind:'military', value:0.10},
    {kind:'invest', value:0.08}, {...annualBudgetOf(m), health:0.03}
  ]; queueBudgetDial(m, 'health', 0.005);`);
  const orders = plain(c, 'queued');
  assert.equal(orders.filter(x => x.kind === 'annual_budget').length, 1);
  assert.deepEqual(orders.filter(x => x.kind !== 'annual_budget'), [
    {kind:'rate', value:0.05}, {kind:'tax', value:0.32}, {kind:'sanction', target:'Canada'},
  ]);
  const p = plain(c, 'proposed(m)');
  assert.equal(p.rate, 0.05);
  assert.equal(p.tax, 0.32);
  assert(Math.abs(p.annual.health - 0.035) < 1e-12);
  assert.equal(p.annual.fiscal_year, 1991);
  assert.equal(p.military, 0.02);
  assert(Math.abs(p.invest - 0.06) < 1e-12);
  assert.equal(c.notes, 1);
  assert.equal(c.renders, 1);
  evaluate(c, 'queueBudgetDial(m, "health", 1)');
  assert.equal(evaluate(c, 'annualBudgetOf(m).health'), 0.15, 'cap comes from the server');
  evaluate(c, 'queueBudgetDial(m, "health", -1)');
  assert.equal(evaluate(c, 'annualBudgetOf(m).health'), 0, 'a cut cannot make negative spending');
});

test('revert discards every queued order and redraws the live budget', () => {
  const c = fixture(['annualBudgetOf', 'revertOrders']);
  evaluate(c, 'queued = [{kind:"tax",value:0.3}, {...annualBudgetOf(m),health:0.04}]; revertOrders();');
  assert.equal(evaluate(c, 'queued.length'), 0);
  assert.equal(evaluate(c, 'annualBudgetOf(m).health'), 0.02);
  assert.equal(c.notes, 1);
  assert.equal(c.renders, 1);
});

test('closing the cabinet preserves drafts, removes modality and restores focus', () => {
  const c = fixture(['annualBudgetOf', 'closeGameDrawers']);
  evaluate(c, `queued = [{kind:'rate',value:0.05}, {...annualBudgetOf(m),health:0.04}];
    element('cabinetDrawer').classList.add('open');
    element('app').inert = true; CAB.lastFocus = element('economyDock');
    closeGameDrawers();`);
  assert.equal(evaluate(c, 'queued.length'), 2);
  assert.equal(evaluate(c, 'queued[1].health'), 0.04);
  assert.equal(evaluate(c, 'cabinetIsOpen()'), false);
  assert.equal(c.element('cabinetDrawer').attributes['aria-hidden'], 'true');
  assert.equal(c.element('app').inert, false);
  assert.equal(c.document.activeElement.id, 'economyDock');
});

test('cabinet keyboard navigation wraps tabs and traps focus without consuming native range keys', () => {
  const c = fixture(['cabinetKeys']);
  evaluate(c, `function selectCabinetTab(tab) { CAB.tab = tab; }
    let prevented = 0;
    function key(key, tablist, shiftKey = false) {
      cabinetKeys({key, shiftKey, preventDefault(){ prevented++; },
        target:{closest(){ return tablist ? {} : null; }}});
    }`);
  evaluate(c, 'key("ArrowLeft", true)');
  assert.equal(evaluate(c, 'CAB.tab'), 'policy');
  assert.equal(c.document.activeElement.id, 'cab-tab-policy');
  evaluate(c, 'key("ArrowRight", true)');
  assert.equal(evaluate(c, 'CAB.tab'), 'overview');
  evaluate(c, 'key("End", true)');
  assert.equal(evaluate(c, 'CAB.tab'), 'policy');
  evaluate(c, 'key("Home", true)');
  assert.equal(evaluate(c, 'CAB.tab'), 'overview');

  const first = c.element('firstControl');
  const last = c.element('lastControl');
  const excluded = c.element('notInTabOrder');
  excluded.tabIndex = -1;
  c.element('cabinetDrawer').items = [excluded, first, last];
  first.focus();
  evaluate(c, 'key("Tab", false, true)');
  assert.equal(c.document.activeElement, last);
  evaluate(c, 'key("Tab", false)');
  assert.equal(c.document.activeElement, first);
  const prevented = evaluate(c, 'prevented');
  for (const key of ['ArrowLeft', 'ArrowRight', ' ', 'Enter']) {
    evaluate(c, `key(${JSON.stringify(key)}, false)`);
  }
  assert.equal(evaluate(c, 'prevented'), prevented, 'native controls retain their own keys');
});

test('cabinet is an accessible modal with three tabs and a gameplay-shortcut guard', () => {
  const modal = page.match(/<section\b[^>]*\bid="cabinetDrawer"[^>]*>/);
  assert(modal, 'cabinet has a full-screen section');
  for (const attribute of ['role="dialog"', 'aria-modal="true"', 'aria-labelledby="cabinetTitle"']) {
    assert(modal[0].includes(attribute), `modal must carry ${attribute}`);
  }
  for (const tab of ['overview', 'budget', 'policy']) {
    const button = page.match(new RegExp(`<button\\b[^>]*id="cab-tab-${tab}"[^>]*>`));
    assert(button, `${tab} has a tab control`);
    assert(button[0].includes('role="tab"'));
    assert(button[0].includes(`aria-controls="cabinet-${tab}"`));
    const panel = page.match(new RegExp(`<section\\b[^>]*id="cabinet-${tab}"[^>]*>`));
    assert(panel && panel[0].includes('role="tabpanel"'), `${tab} must have its own tabpanel`);
    assert(panel[0].includes(`aria-labelledby="cab-tab-${tab}"`));
  }
  assert.match(page, /if\s*\(cabinetIsOpen\(\)\)\s*\{\s*cabinetKeys\(e\);\s*return;/,
    'gameplay keyboard shortcuts must stop while the cabinet is open');
  assert.match(page, /id="cabinetDraft"/);
  assert.match(page, /id="cabinetLive"[^>]*aria-live="polite"/);
  assert.match(page, /#cabinetDrawer\s*\{[^}]*position:\s*fixed;[^}]*inset:\s*0;/);
});

test('tab selection changes visibility and roving focus, never the pending budget', () => {
  const c = fixture(['annualBudgetOf', 'selectCabinetTab']);
  evaluate(c, 'queued = [{...annualBudgetOf(m),health:0.04}]; selectCabinetTab("budget");');
  assert.equal(c.element('cabinet-budget').hidden, false);
  assert.equal(c.element('cabinet-overview').hidden, true);
  assert.equal(c.element('cabinet-policy').hidden, true);
  assert.equal(c.element('cab-tab-budget').tabIndex, 0);
  assert.equal(c.element('cab-tab-budget').attributes['aria-selected'], 'true');
  assert.equal(c.element('cab-tab-overview').tabIndex, -1);
  evaluate(c, 'selectCabinetTab("not-a-tab")');
  assert.equal(evaluate(c, 'CAB.tab'), 'budget');
  assert.equal(evaluate(c, 'queued[0].health'), 0.04);
});

test('draft summary reacts to queued tax and includes interest exactly once', () => {
  const c = fixture(['annualBudgetOf', 'annualSocial', 'annualInvest', 'proposed', 'dragsOf',
    'ledgerOf', 'cabinetBudgetSummary']);
  evaluate(c, 'S.policy = {budget_oil_revenue:0.01,money:{interest_gdp:0.02}}; queued = [{kind:"tax",value:0.30}];');
  const summary = evaluate(c, 'cabinetBudgetSummary(m)');
  assert(summary.includes('$310bn'), 'draft tax plus served oil revenue');
  assert(summary.includes('$220bn'), 'ten ministries plus served interest, once');
  assert.match(summary, /Surplus/);
  assert(summary.includes('$90bn'), 'revenue less ministries less interest');
  assert.match(summary, /\/ year/);
  assert.match(functionSource('wireSliders'), /paintCabinetDraft\(m\)/,
    'moving tax/rate must refresh the shared draft summary, not only the old ledger');
});

test('enacting an untouched first budget queues the inherited plan and advances exactly one day', async () => {
  const c = fixture(['annualBudgetOf', 'cabinetEnact']);
  evaluate(c, `let advances = []; let draftsPainted = 0;
    function paintCabinetDraft(){ draftsPainted++; }
    async function advance(days){ advances.push({days,commands:JSON.parse(JSON.stringify(queued))}); queued=[]; return true; }
    queued = [{kind:'tax',value:0.3}];`);
  await evaluate(c, 'cabinetEnact()');
  const calls = plain(c, 'advances');
  assert.equal(calls.length, 1);
  assert.equal(calls[0].days, 1);
  assert.equal(calls[0].commands[0].kind, 'tax');
  const budget = calls[0].commands.find(order => order.kind === 'annual_budget');
  assert(budget, 'Enact must actually open the books, not just advance an empty queue');
  assert.equal(budget.fiscal_year, 1991);
  assert.equal(budget.health, 0.02);
  assert.equal(evaluate(c, 'CAB.busy'), false);
  assert.match(c.element('cabinetLive').textContent, /one day/);
});

test('a failed enact preserves the mixed draft and releases the busy guard for retry', async () => {
  const c = fixture(['annualBudgetOf', 'cabinetEnact']);
  evaluate(c, `let attempts = 0;
    function paintCabinetDraft(){}
    async function advance(){ attempts++; throw new Error('Connection lost'); }
    queued = [{kind:'rate',value:0.05},{...annualBudgetOf(m),health:0.04}];`);
  const before = plain(c, 'queued');
  await evaluate(c, 'cabinetEnact()');
  assert.deepEqual(plain(c, 'queued'), before);
  assert.equal(evaluate(c, 'attempts'), 1);
  assert.equal(evaluate(c, 'CAB.busy'), false);
  assert.equal(evaluate(c, 'CAB.error'), 'Connection lost');
  assert.equal(c.element('cabinetLive').textContent, 'Connection lost');
  evaluate(c, 'CAB.busy = true');
  await evaluate(c, 'cabinetEnact()');
  assert.equal(evaluate(c, 'attempts'), 1, 'double-click must not issue another advance');
});

function cabinetActionFixture() {
  return fixture(['annualBudgetOf', 'annualPoliticalCost', 'annualSocial', 'annualInvest',
    'proposed', 'dragsOf', 'ledgerOf', 'cabinetBudgetSummary', 'paintCabinetDraft', 'cabinetEnact']);
}

test('all draft controls are inert only while the asynchronous enact is pending', async () => {
  const c = cabinetActionFixture();
  evaluate(c, `let finishAdvance; let calls = 0;
    function advance(){ calls++; return new Promise(resolve => { finishAdvance = resolve; }); }
    S.date = '1 Jan 1991'; paintCabinetDraft(m);`);
  assert.equal(c.element('left').inert, false);
  const pending = evaluate(c, 'cabinetEnact()');
  assert.equal(evaluate(c, 'CAB.busy'), true);
  assert.equal(c.element('left').inert, true, 'pending network request must block edits that would otherwise be discarded');
  assert.equal(c.element('left').attributes['aria-busy'], 'true');
  await evaluate(c, 'cabinetEnact()');
  assert.equal(evaluate(c, 'calls'), 1, 'enact remains guarded while its first request is pending');
  evaluate(c, 'finishAdvance(true)');
  await pending;
  assert.equal(evaluate(c, 'CAB.busy'), false);
  assert.equal(c.element('left').inert, false, 'the workbench must become editable again');
  assert.equal(c.element('left').attributes['aria-busy'], 'false');
});

test('dated command refusals are surfaced in the footer and live region after advance', async () => {
  const c = cabinetActionFixture();
  evaluate(c, `S.date = '1 Jan 1991';
    async function advance() {
      S.date = '2 Jan 1991';
      S.errors = ['This field is not the advance endpoint contract'];
      S.log = [
        {date:'31 Dec 1990',text:'[rejected] Old failure'},
        {date:'1 Jan 1991',text:'An ordinary event'},
        {date:'2 Jan 1991',text:'[rejected] Another date'},
        {date:'1 Jan 1991',text:'[rejected] SetAnnualBudget { nation: France }: Not enough political capital'}
      ];
      queued = [];
      return true;
    }`);
  await evaluate(c, 'cabinetEnact()');
  assert.equal(evaluate(c, 'CAB.error'), 'Not enough political capital');
  assert.equal(c.element('cabinetLive').textContent, 'Not enough political capital');
  assert.match(c.element('cabinetDraft').innerHTML, /Not enough political capital/);
  assert.doesNotMatch(c.element('cabinetDraft').innerHTML, /Old failure|Another date|This field is not/);
  assert.equal(c.element('left').inert, false, 'refusal must not leave the editor locked');
});

test('the illustrated cabinet asset is local and routed as SVG', () => {
  const server = fs.readFileSync(path.join(root, 'spheres-web/src/main.rs'), 'utf8');
  const asset = fs.readFileSync(path.join(root, 'spheres-web/ui/cabinet-city.svg'), 'utf8');
  assert.match(page, /src="\/assets\/cabinet-city\.svg"/);
  assert.match(server, /\(Method::Get, "\/assets\/cabinet-city\.svg"\)/);
  assert.match(server, /CABINET_CITY_SVG[^;]*include_str!\("\.\.\/ui\/cabinet-city\.svg"\)/);
  assert.match(asset, /<svg\b/);
  assert.doesNotMatch(asset, /<script\b|<image\b|href=["']https?:/i,
    'the illustration must not depend on scripts or remote imagery');
});

test('effect previews index the served reference-anchored curve and respect its limits', () => {
  const c = fixture(['ministryAt']);
  evaluate(c, 'const served = {reference:0.05375}; const curve = [10,20,30,40,50];');
  assert.equal(evaluate(c, 'ministryAt(served, curve, 0.05375)'), 30);
  assert.equal(evaluate(c, 'ministryAt(served, curve, 0.05875)'), 40);
  assert.equal(evaluate(c, 'ministryAt(served, curve, 0.04875)'), 20);
  assert.equal(evaluate(c, 'ministryAt(served, curve, 100)'), 50);
  assert.equal(evaluate(c, 'ministryAt(served, curve, -100)'), 10);
  assert.equal(evaluate(c, 'ministryAt(served, [], 0.05375)'), null);
});

test('budget cost distinguishes first enactment, cuts and reopening an enacted year', () => {
  const c = fixture(['annualBudgetOf', 'annualPoliticalCost']);
  assert.equal(evaluate(c, 'annualPoliticalCost(m, annualBudgetOf(m))'), 0);
  const raise = evaluate(c, 'annualPoliticalCost(m, {...annualBudgetOf(m),health:0.025})');
  const cut = evaluate(c, 'annualPoliticalCost(m, {...annualBudgetOf(m),health:0.015})');
  assert(Math.abs(raise - 0.55) < 1e-12);
  assert(Math.abs(cut - 0.7425) < 1e-12);
  evaluate(c, 'm.annual_budget.due = false');
  assert(Math.abs(evaluate(c, 'annualPoliticalCost(m, {...annualBudgetOf(m),health:0.025})') - (raise + 4)) < 1e-12);
  assert.equal(evaluate(c, 'annualPoliticalCost(m, annualBudgetOf(m))'), 0, 'no reversal surcharge for no change');
});

test('the treasury card distinguishes absent books from zero money and preserves annual units', () => {
  const c = fixture(['moneyCard']);
  evaluate(c, 'S.policy = {money:{on_the_books:false,treasury_bn:null,debt_bn:null}}');
  const closed = evaluate(c, 'moneyCard(m)');
  assert.match(closed, /books closed/i);
  assert.doesNotMatch(closed, /\$0bn|\$nullbn|NaN|undefined/);
  evaluate(c, `S.policy.money = {on_the_books:true, treasury_bn:100,
    revenue_bn:372, spend_bn:248, interest_bn:12, balance_bn:112,
    interest_gdp:0.012, balance_gdp:0.112, debt_bn:150,
    debt_gdp:0.15, net_position_bn:-50, effective_rate:0.08, real_rate:0.02, spread:0.06};`);
  const open = evaluate(c, 'moneyCard(m)');
  for (const annual of ['$372bn/yr', '$248bn/yr', '$12bn/yr', '$112bn/yr']) {
    assert(open.includes(annual), `served annual amount must retain its unit: ${annual}`);
  }
  assert.match(open, /Treasury: \$100bn/);
  assert.match(open, /Debt: \$150bn/);
  assert.match(open, /debtor \$50bn/);
  assert.doesNotMatch(open, /NaN|undefined/);
});
