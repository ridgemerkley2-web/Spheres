// Run: node --test tools/ui/check_chronicle.cjs. No browser or dependencies.
const assert = require('node:assert/strict');
const fs = require('node:fs');
const path = require('node:path');
const vm = require('node:vm');
const { test } = require('node:test');
const text = fs.readFileSync(path.resolve(__dirname, '../../spheres-web/ui/chronicle-data.js'), 'utf8');
const uiText = fs.readFileSync(path.resolve(__dirname, '../../spheres-web/ui/chronicle-ui.js'), 'utf8');
const context = vm.createContext({});
vm.runInContext(text, context);
const { chronicleOrdinal: ordinal, chronicleRange: range, chroniclePoints: points, chronicleDelta: delta } = context;
const plain = value => JSON.parse(JSON.stringify(value));
function history(labels, nations) { return { t: labels.map((_, i) => i / 31), labels, nations }; }
const empty = { start: null, end: null, indices: [] };

test('six metric descriptions and their delta rules are deeply immutable', () => {
  const specs = vm.runInContext('CHRONICLE_METRICS', context);
  assert.deepEqual(Object.keys(specs), ['gdp', 'growth', 'inflation', 'debt', 'stability', 'mil']);
  assert(Object.isFrozen(specs));
  for (const [key, value] of Object.entries(specs)) {
    assert(Object.isFrozen(value)); assert(value.label); assert(value.unit);
    assert.equal(value.deltaKind, ['gdp', 'mil'].includes(key) ? 'percent' : key === 'stability' ? 'points' : 'pp');
  }
});

test('date labels use real UTC days, accept legacy months, and reject rollover dates', () => {
  assert.equal(ordinal('1 Jan 1990'), Date.UTC(1990, 0, 1) / 86400000);
  assert.equal(ordinal('Jan 1990'), ordinal('1 Jan 1990'));
  assert.equal(ordinal(' 29 FEBRUARY 2000 '), Date.UTC(2000, 1, 29) / 86400000);
  assert.equal(ordinal('1 Mar 2000') - ordinal('28 Feb 2000'), 2);
  assert.equal(ordinal('1 Mar 1900') - ordinal('28 Feb 1900'), 1);
  assert.equal(ordinal('1 Jan 99'), ordinal('2 Jan 99') - 1, 'years below 100 must not silently become 1999');
  for (const invalid of [null, undefined, 0, '', 'February', '31 Apr 1990', '29 Feb 1900', '0 Jan 1990', '1 Foo 1990', '1 Jan 0000', '1990-01-01', '1 Jan 1990 extra']) {
    assert.equal(ordinal(invalid), null, String(invalid));
  }
});

test('daily windows include end-minus-days-plus-one with leap and month-end boundaries', () => {
  const h = history(['29 Jan 2000', '30 Jan 2000', '31 Jan 2000', '28 Feb 2000', '29 Feb 2000'], { A: {t0: 0, gdp: [1, 2, 3, 4, 5]} });
  assert.deepEqual(plain(range(h, 'A', 30)), {start: 2, end: 4, indices: [2, 3, 4]});
  assert.deepEqual(plain(range(h, 'A', 1)), {start: 4, end: 4, indices: [4]});
  assert.deepEqual(plain(range(h, 'A', 0)), {start: 0, end: 4, indices: [0, 1, 2, 3, 4]});
});

test('30/365/1825-day views are calendar windows rather than counts of irregular observations', () => {
  const labels = ['31 Dec 1989', '1 Jan 1990', '31 Dec 1990', '1 Jan 1994', '31 Jan 1994', '1 Feb 1994', '1 Jan 1995'];
  const h = history(labels, { A: {t0: 0, gdp: labels.map((_,i) => i + 1)} });
  assert.deepEqual(plain(range(h, 'A', 30)).indices, [6]);
  assert.deepEqual(plain(range(h, 'A', 365)).indices, [4, 5, 6]);
  assert.deepEqual(plain(range(h, 'A', 1825)).indices, [2, 3, 4, 5, 6]);
  const monthly = history(['Jan 1990', 'Feb 1990', 'Mar 1990'], { A: {t0: 0, gdp: [1, 2, 3]} });
  assert.deepEqual(plain(range(monthly, 'A', 30)), {start: 1, end: 2, indices: [1, 2]});
});

test('the first recorded day is usable and an extinct nation ends at its final observation', () => {
  const labels = ['1 Jan 1990', '2 Jan 1990', '3 Jan 1990', '4 Jan 1990', '5 Jan 1990'];
  const h = history(labels, {
    First: {t0: 0, gdp: [10]},
    Dead: {t0: 0, gdp: [100, 101], mil: [20, 21, 22, 23, 24]},
    Born: {t0: 3, gdp: [50, 51]},
  });
  assert.deepEqual(plain(range(h, 'First', 365)), {start: 0, end: 0, indices: [0]});
  assert.deepEqual(plain(range(h, 'Dead', 365)), {start: 0, end: 1, indices: [0, 1]});
  assert.deepEqual(plain(points(h, 'Dead', 'mil', 0, 4)), [{i: 0, value: 20}, {i: 1, value: 21}]);
  assert.deepEqual(plain(range(h, 'Born', 365)), {start: 3, end: 4, indices: [3, 4]});
  assert.deepEqual(plain(points(h, 'Born', 'gdp', 0, 8)), [{i: 3, value: 50}, {i: 4, value: 51}]);
});

test('series spans intersect the world history and fall back only when GDP is missing', () => {
  const h = history(['1 Jan 1990', '2 Jan 1990', '3 Jan 1990'], {
    Long: {t0: 1, gdp: [1,2,3,4]},
    Missing: {t0: 1, mil: [0,2], stability: [30]},
    Future: {t0: 7, gdp: [1]}, Negative: {t0: -1, gdp: [1]}, Invalid: {t0:'0', gdp:[1]}, Empty: {t0:0},
  });
  assert.deepEqual(plain(range(h, 'Long', 0)), {start: 1, end: 2, indices: [1, 2]});
  assert.deepEqual(plain(range(h, 'Missing', 0)), {start: 1, end: 2, indices: [1, 2]});
  for (const id of ['Future','Negative','Invalid','Empty','Absent']) assert.deepEqual(plain(range(h,id,0)),empty);
  for (const days of [-1,NaN,Infinity,'30',undefined,1.5]) assert.deepEqual(plain(range(h,'Long',days)),empty);
  assert.deepEqual(plain(range(null,'Long',30)),empty);
  assert.deepEqual(plain(range({t:[],nations:{}},'Long',0)),empty);
});

test('missing or invalid date labels never turn into made-up daily observations', () => {
  const h = history(['1 Jan 1990', 'nonsense', '3 Jan 1990'], {A:{t0:0,gdp:[1,2,3]}});
  assert.deepEqual(plain(range(h,'A',30)),{start:0,end:2,indices:[0,2]});
  h.labels[2] = null;
  assert.deepEqual(plain(range(h,'A',30)),empty);
  assert.deepEqual(plain(range(h,'A',0)),{start:0,end:2,indices:[0,1,2]});
});

test('points preserve null, nonfinite, undefined and sparse holes without coercing strings', () => {
  const values = [0, null, undefined, NaN, Infinity, -Infinity, '3', false, -4];
  values.length = 10;
  const h = history(Array.from({length:12}, (_,i) => `${i+1} Jan 1990`), {A:{t0:2,gdp:Array(10).fill(100),growth:values}});
  const result = plain(points(h,'A','growth',0,11));
  assert.deepEqual(result.map(p=>p.i),[2,3,4,5,6,7,8,9,10,11]);
  assert.deepEqual(result.map(p=>p.value),[0,null,null,null,null,null,null,null,-4,null]);
  assert.deepEqual(plain(points(h,'A','debt',2,3)),[{i:2,value:null},{i:3,value:null}]);
  assert.deepEqual(plain(points(h,'A','unknown',2,3)),[]);
  assert.deepEqual(plain(points(h,'A','growth',4,3)),[]);
  assert.deepEqual(plain(points(h,'A','growth',null,3)),[]);
  assert.deepEqual(plain(points(h,'missing','growth',0,1)),[]);
  assert.equal(h.nations.A.growth[0],0);
  assert.equal(h.nations.A.growth[6],'3');
});

test('delta distinguishes percent changes, percentage points and raw stability points', () => {
  assert.deepEqual(plain(delta('gdp',100,125)),{value:25,kind:'percent'});
  assert.deepEqual(plain(delta('mil',40,30)),{value:-25,kind:'percent'});
  assert.deepEqual(plain(delta('gdp',-100,-50)),{value:50,kind:'percent'});
  assert.deepEqual(plain(delta('mil',10,0)),{value:-100,kind:'percent'});
  assert.deepEqual(plain(delta('growth',-.02,.03)),{value:5,kind:'pp'});
  assert.deepEqual(plain(delta('inflation',.25,.5)),{value:25,kind:'pp'});
  assert.deepEqual(plain(delta('debt',.25,0)),{value:-25,kind:'pp'});
  assert.deepEqual(plain(delta('stability',0,80)),{value:80,kind:'points'});
  for (const key of ['gdp','mil']) assert.equal(delta(key,0,10),null);
  for (const invalid of [null,undefined,NaN,Infinity,-Infinity,'0',false]) {
    assert.equal(delta('growth',invalid,1),null);
    assert.equal(delta('growth',1,invalid),null);
  }
  assert.equal(delta('toString',1,2),null);
  assert.equal(delta('gdp',Number.MIN_VALUE,Number.MAX_VALUE),null);
});

function uiFixture(labels = ['1 Jan 1990', '2 Jan 1990', '3 Jan 1990']) {
  const moneyCalls = [];
  const c = vm.createContext({
    fmt: { money: value => { moneyCalls.push(value); return `$${value}bn`; } },
    labelAt: i => labels[i] || '',
    escText: value => String(value).replace(/&/g, '&amp;').replace(/</g, '&lt;').replace(/"/g, '&quot;'),
  });
  vm.runInContext(text + '\n' + uiText, c);
  c.moneyCalls = moneyCalls;
  return c;
}

test('infographic values distinguish real zero from missing and keep displayed rate units', () => {
  const c = uiFixture();
  assert.equal(c.chronicleValue('gdp', 0), '$0bn');
  assert.deepEqual(c.moneyCalls, [0], 'GDP must use the established money formatter');
  assert.equal(c.chronicleValue('growth', -.0123), '-1.23%');
  assert.equal(c.chronicleValue('inflation', 0), '0%');
  assert.equal(c.chronicleValue('debt', 1.5), '150%');
  assert.equal(c.chronicleValue('stability', 42.125), '42.13');
  assert.equal(c.chronicleValue('mil', 1000), '1,000');
  for (const missing of [null, undefined, NaN, Infinity, -Infinity, '0', false]) {
    assert.equal(c.chronicleValue('gdp', missing), '—');
    assert.equal(c.chronicleValue('growth', missing), '—');
  }
  assert.deepEqual(c.moneyCalls, [0], 'unknown GDP cannot become a real zero in the money formatter');
});

test('trend axes distinguish narrow real GDP spans with a common compact currency unit', () => {
  const c=uiFixture();
  for(const [lo,hi,suffix] of [[6000.12,6008.34,'tn'],[6.0012,6.0083,'bn'],[.0060012,.0060083,'m'],[.0000060012,.0000060083,'k'],[.0000000060012,.0000000060083,'']]) {
    const low=c.chronicleAxisValue('gdp',lo,lo,hi),high=c.chronicleAxisValue('gdp',hi,lo,hi);
    assert.notEqual(low,high,`collapsed GDP bounds for ${lo}..${hi}`);
    assert(low.startsWith('$') && high.startsWith('$'));
    assert(low.endsWith(suffix) && high.endsWith(suffix));
    assert(low.length<=11 && high.length<=11,`${low} / ${high} should fit a compact axis`);
  }
  assert.equal(c.chronicleAxisValue('gdp',999,999,1001),'$0.999tn');
  assert.equal(c.chronicleAxisValue('gdp',1001,999,1001),'$1.001tn');
  assert.deepEqual(c.moneyCalls,[],'axis labels must not reuse the rounded headline money formatter');
  const chart=c.chronicleChart('gdp',[{i:0,value:6000.12},{i:1,value:6008.34}],{range:{start:0,end:1},cursor:1,times:[0,42]},330);
  const labels=[...chart.matchAll(/<text class="chr-axis"[^>]*text-anchor="end">([^<]*)<\/text>/g)].map(m=>m[1]);
  assert.notEqual(labels[0],labels[1],'the two actual rendered currency bounds must read differently');
});

test('adaptive rate and score axes retain nonzero signs, small spans and scientific extremes', () => {
  const c=uiFixture();
  for(const [key,lo,hi] of [['growth',.0200001,.0200004],['inflation',-.00000002,-.00000001],['debt',.500001,.500002],['stability',50.00001,50.00002],['mil',0,.0000001],['growth',1e-18,2e-18],['gdp',1e-28,2e-28],['mil',1e30,2e30]]) {
    const low=c.chronicleAxisValue(key,lo,lo,hi),high=c.chronicleAxisValue(key,hi,lo,hi);
    assert.notEqual(low,high,`${key} must distinguish ${lo}..${hi}`);
    const number=label=>Number(label.replace(/^\$/,'').replace(/(?:tn|bn|m|k|%)$/,''));
    if(lo!==0) assert.notEqual(number(low),0,`${low} falsely became zero`);
    if(hi!==0) assert.notEqual(number(high),0,`${high} falsely became zero`);
    if(lo<0) assert(low.includes('-'));
    assert(!/NaN|Infinity/.test(low+high));
  }
  assert.equal(c.chronicleAxisValue('growth',0,0,.01),'0%');
  assert.equal(c.chronicleAxisValue('gdp',0,0,0),'$0');
  for(const bad of [null,undefined,NaN,Infinity,'0']) assert.equal(c.chronicleAxisValue('gdp',bad,0,1),'—');
});

test('infographic changes distinguish direction, no change, unavailable and percentage points', () => {
  const c = uiFixture();
  assert.deepEqual(plain(c.chronicleChange('gdp', 100, 125)), {arrow:'↑', text:'25%'});
  assert.deepEqual(plain(c.chronicleChange('mil', 40, 30)), {arrow:'↓', text:'25%'});
  assert.deepEqual(plain(c.chronicleChange('growth', -.02, .03)), {arrow:'↑', text:'5 pp'});
  assert.deepEqual(plain(c.chronicleChange('stability', 90, 80)), {arrow:'↓', text:'10 pts'});
  assert.deepEqual(plain(c.chronicleChange('debt', 0, 0)), {arrow:'→', text:'No change'});
  assert.deepEqual(plain(c.chronicleChange('inflation', 0, .000001)), {arrow:'↑', text:'<0.01 pp'});
  for (const [key,first,last] of [['gdp',0,1],['mil',0,1],['growth',null,0],['growth',0,undefined]]) {
    assert.deepEqual(plain(c.chronicleChange(key,first,last)), {arrow:'·', text:'Change unavailable'});
  }
});

test('trace thinning preserves endpoints, local extrema, order and data gaps', () => {
  const c = uiFixture();
  const series = Array.from({length:1000}, (_,i) => ({i,value:i%13}));
  series[301].value = 5000; series[302].value = -6000;
  series[555].value = null; series[556].value = NaN;
  const snapshot = series.map(p=>({...p}));
  const thin = c.chronicleThin(series, 20);
  assert.equal(thin[0].i, 0); assert.equal(thin.at(-1).i, 999);
  for (const i of [301,302,555,556]) assert(thin.some(p=>p.i===i), `lost spike or hole at ${i}`);
  assert(thin.length < series.length);
  assert.equal(new Set(thin.map(p=>p.i)).size, thin.length);
  assert(thin.every((p,i)=>i===0 || p.i>thin[i-1].i));
  assert.deepEqual(series,snapshot,'rendering may not modify historical observations');
});

test('sparkline geometry uses actual dates across irregular month ends and leap years', () => {
  const labels = ['1 Jan 2000', '1 Feb 2000', '1 Mar 2000'];
  const c = uiFixture(labels);
  const model = {range:{start:0,end:2},cursor:1,times:labels.map(ordinal)};
  const chart = c.chronicleChart('growth',[{i:0,value:-.01},{i:1,value:.02},{i:2,value:.03}],model,400);
  const path = /class="chr-line" d="([^"]*)"/.exec(chart)[1];
  const x = [...path.matchAll(/[ML]([\d.-]+),/g)].map(m=>+m[1]);
  assert.deepEqual(x,[66,232.37,388], 'February must occupy its actual day fraction, not an equal index step');
  assert(chart.includes('1 Jan 2000 to 1 Mar 2000'));
  assert(chart.includes('Vertical scale is independent for each metric'));
  assert(chart.includes('role="img"'));
  assert(!/NaN|Infinity/.test(chart));
});

test('sparklines break at null observations and show one truthful dot on day one', () => {
  const c = uiFixture(['1 Jan 1990','2 Jan 1990','3 Jan 1990','4 Jan 1990','5 Jan 1990']);
  const model = {range:{start:0,end:4},cursor:2,times:[0,1,2,3,4]};
  const chart = c.chronicleChart('stability',[{i:0,value:50},{i:1,value:55},{i:2,value:null},{i:3,value:60},{i:4,value:62}],model,330);
  const path = /class="chr-line" d="([^"]*)"/.exec(chart)[1];
  assert.equal((path.match(/M/g)||[]).length,2);
  assert.equal((path.match(/L/g)||[]).length,2);
  assert(!chart.includes('class="chr-cursor"'),'a missing observation cannot acquire a fabricated cursor value');
  for (const value of [0,10,-10]) {
    const single = c.chronicleChart('stability',[{i:0,value}],{range:{start:0,end:0},cursor:0,times:[ordinal('1 Jan 1990')]},230);
    assert(single.includes('class="chr-dot"'));
    assert(single.includes('1 recorded values'));
    assert(!/NaN|Infinity/.test(single));
    assert(!/ d="[^"]*L/.test(single),'a single snapshot must not become an invented flat time series');
  }
  const none=c.chronicleChart('gdp',[{i:0,value:null}],{range:{start:0,end:0},cursor:0,times:[0]},330);
  assert.equal(none,'<div class="chr-no-series">No recorded values in this period.</div>');
});

test('late and extinct national charts label only their recorded lifespan', () => {
  const labels=['1 Jan 1990','2 Jan 1990','3 Jan 1990','4 Jan 1990','5 Jan 1990'];
  const h=history(labels,{Dead:{t0:1,gdp:[100,105]}});
  const c=uiFixture(labels), r=range(h,'Dead',0);
  const chart=c.chronicleChart('gdp',points(h,'Dead','gdp',r.start,r.end),{range:r,cursor:r.end,times:labels.map(ordinal)},330);
  assert(chart.includes('2 Jan 1990 to 3 Jan 1990'));
  assert(!chart.includes('1 Jan 1990'));
  assert(!chart.includes('5 Jan 1990'));
});

test('classic-script order and global names remain safe and all Chronicle assets are served', () => {
  const page=fs.readFileSync(path.resolve(__dirname,'../../spheres-web/ui/index.html'),'utf8');
  const server=fs.readFileSync(path.resolve(__dirname,'../../spheres-web/src/main.rs'),'utf8');
  const css=fs.readFileSync(path.resolve(__dirname,'../../spheres-web/ui/chronicle.css'),'utf8');
  const dataAt=page.indexOf('<script src="/chronicle-data.js"></script>');
  const uiAt=page.indexOf('<script src="/chronicle-ui.js"></script>');
  const inlineAt=page.indexOf('<script>',uiAt);
  assert(dataAt>=0 && dataAt<uiAt && uiAt<inlineAt);
  const inlines=[...page.matchAll(/<script(?:\s[^>]*)?>([\s\S]*?)<\/script>/g)].map(m=>m[1]).filter(s=>s.trim());
  new vm.Script([text,uiText,...inlines].join('\n'));
  const declarations=[...`${text}\n${uiText}`.matchAll(/^(?:const|let|var|function)\s+([A-Za-z_$][\w$]*)/gm)].map(m=>m[1]);
  assert.equal(new Set(declarations).size,declarations.length);
  for(const name of declarations) {
    assert(!inlines.some(s=>new RegExp(`^(?:const|let|var|function)\\s+${name}\\b`,'m').test(s)),`${name} would override a shared UI global`);
  }
  assert(page.includes('<link rel="stylesheet" href="/chronicle.css">'));
  for(const asset of ['chronicle-data.js','chronicle-ui.js','chronicle.css']) {
    assert(server.includes(`include_str!("../ui/${asset}")`));
    assert(server.includes(`"/${asset}"`));
  }
  assert(css.includes('.chr-chart .chr-axis'));
  assert(css.includes('@media (max-width: 650px)'));
  assert(css.includes('.chr-metric-grid { grid-template-columns: 1fr;'));
});

// Minimal DOM for the renderer's state wiring; the chart output is tested
// directly above. No DOM parser, browser, simulation commands or network.
function renderFixture(h) {
  const elements = new Map();
  const doc = {activeElement:null};
  const element = selector => {
    if (!elements.has(selector)) elements.set(selector, {
      innerHTML:'',value:'',id:selector.slice(1),clientWidth:800,
      contains:()=>false,querySelectorAll:()=>[],
      focus(){doc.activeElement=this;},
    });
    return elements.get(selector);
  };
  doc.querySelector = element;
  doc.getElementById = id => element('#'+id);
  const c = vm.createContext({
    HIST:h,S:{player:'A'},document:doc,
    escText:value=>String(value),fmt:{money:value=>`$${value}bn`},
  });
  vm.runInContext(text+'\n'+uiText+'\nfunction labelAt(i){return HIST.labels[i] || "";} paintChronicle = function(){};',c);
  c.element = element;
  return c;
}

test('renderer pins the selected date through history trimming and follows latest only when requested', () => {
  const h=history(['1 Jan 1990','2 Jan 1990','3 Jan 1990'],{A:{name:'A',t0:0,gdp:[10,20,30]}});
  const c=renderFixture(h);
  const state=()=>vm.runInContext('CHRONICLE',c);
  c.renderChronicle();
  assert.equal(state().model.cursor,2);
  const slider=c.element('#chronicleDate');slider.value='1';slider.oninput();
  assert.equal(state().cursor,'2 Jan 1990','pin the date, not its current array offset');
  c.HIST=history(['2 Jan 1990','3 Jan 1990','4 Jan 1990'],{A:{name:'A',t0:0,gdp:[20,30,40]}});
  c.renderChronicle();
  assert.equal(state().model.cursor,0);
  assert.equal(c.HIST.labels[state().model.cursor],'2 Jan 1990');
  c.HIST=history(['3 Jan 1990','4 Jan 1990','5 Jan 1990'],{A:{name:'A',t0:0,gdp:[30,40,50]}});
  c.renderChronicle();
  assert.equal(state().model.cursor,0,'a retired date clamps to the oldest retained observation');
  slider.value='2';slider.oninput();
  assert.equal(state().cursor,null);
  c.HIST=history(['3 Jan 1990','4 Jan 1990','5 Jan 1990','6 Jan 1990'],{A:{name:'A',t0:0,gdp:[30,40,50,60]}});
  const before=JSON.stringify(c.HIST);
  c.renderChronicle();
  assert.equal(state().model.cursor,3,'latest mode follows newly recorded observations');
  assert.equal(JSON.stringify(c.HIST),before,'read-only history rendering must not change observations');
});

test('renderer uses one coherent fallback time axis and explains retained history and smoothed rates', () => {
  const h=history(['1 Jan 1990','bad date','3 Jan 1990'],{A:{name:'A',t0:0,gdp:[10,20,30]}});
  h.t=[240,NaN,242];
  const c=renderFixture(h);
  c.renderChronicle();
  assert.deepEqual(plain(vm.runInContext('CHRONICLE.model.times',c)),[0,1,2]);
  c.HIST.t=[240,241,242];c.renderChronicle();
  assert.deepEqual(plain(vm.runInContext('CHRONICLE.model.times',c)),[240,241,242]);
  const markup=c.element('#pane-charts').innerHTML;
  assert(markup.includes('All recorded history'));
  assert(markup.includes('History retains up to 3,000 snapshots'));
  assert(markup.includes('restarts after loading a save'));
  assert(markup.includes('Smoothed annualized growth rate'));
  assert(!markup.includes('Full campaign'));
});

test('first-day and empty retained histories have truthful interaction states', () => {
  const c=renderFixture(history(['1 Jan 1990'],{A:{name:'A',t0:0,gdp:[0]}}));
  c.renderChronicle();
  const markup=c.element('#pane-charts').innerHTML;
  assert.match(markup, /id="chronicleDate"[^>]*min="0" max="0"[^>]* disabled/);
  assert(markup.includes('One snapshot so far'));
  c.HIST=history([],{});c.renderChronicle();
  assert.equal(vm.runInContext('CHRONICLE.model',c),null);
  assert(c.element('#pane-charts').innerHTML.includes('Your story starts here'));
});
