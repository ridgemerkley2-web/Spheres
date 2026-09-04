// Execute the real comparison-chart helpers, without a browser or live game.
// Run: node --test tools/ui/check_history_charts.cjs
const assert = require('node:assert/strict');
const fs = require('node:fs');
const path = require('node:path');
const vm = require('node:vm');
const { test } = require('node:test');

const page = fs.readFileSync(path.resolve(__dirname, '../../spheres-web/ui/index.html'), 'utf8');
function source(name) {
  const found = new RegExp(`^function ${name}\\(`, 'm').exec(page);
  assert(found, `Missing actual helper ${name}`);
  const end = page.indexOf('\n}', found.index);
  assert(end > found.index);
  return page.slice(found.index, end + 2);
}
function fixture(t, labels, log = []) {
  const context = vm.createContext({ HIST: { t, labels, nations: {} }, S: { log } });
  vm.runInContext(`const CHARTS = {}; let chartSeq = 0;
    const clamp = (v, lo, hi) => Math.max(lo, Math.min(hi, v));
    ${['niceStep', 'labelAt', 'chart', 'wireCharts', 'marksFor'].map(source).join('\n')}`, context);
  return context;
}
const run = (c, code) => vm.runInContext(code, c, { timeout: 1000 });
const close = (actual, expected, message) => assert(Math.abs(actual - expected) < 1e-6, `${message}: ${actual} != ${expected}`);
const line = (name, vals, t0 = 0) => ({ name, vals, t0, color: '#aabbcc' });
function draw(c, series, extra = {}) {
  c.config = { series, ...extra };
  const html = run(c, 'chart(config)');
  const id = /data-chart="([^"]+)"/.exec(html)?.[1];
  c.chartId = id;
  return { html, state: id ? run(c, 'CHARTS[chartId]') : null };
}
function paths(html) {
  return [...html.matchAll(/<path d="([^"]*)"/g)].map(match =>
    [...match[1].matchAll(/[ML]([\d.-]+) ([\d.-]+)/g)].map(p => [Number(p[1]), Number(p[2])])
  );
}
function event(date, text = date, extra = {}) {
  return { date, text, t: 0, cat: 'war', tags: ['USA'], ...extra };
}
function hoverFixture(c, state) {
  const node = () => ({ attrs: {}, setAttribute(k, v) { this.attrs[k] = String(v); } });
  const cur = node(), dots = state.ser.map(node), events = {};
  const readout = { textContent: 'Original hint', innerHTML: '' };
  const svg = {
    dataset: { chart: c.chartId },
    getBoundingClientRect: () => ({ left: 0, width: 960 }),
    querySelector: () => cur, querySelectorAll: () => dots,
    addEventListener(k, fn) { events[k] = fn; },
  };
  c.document = { getElementById: () => readout };
  c.root = { querySelectorAll: () => [svg] };
  run(c, 'wireCharts(root)');
  return { cur, dots, readout, move: fraction => events.mousemove({ clientX: state.PADL + fraction * state.PW }), leave: () => events.mouseleave() };
}

test('daily chart labels retain the year and position observations by real dates', () => {
  const c = fixture([0, 1 / 31, 2 / 31], ['1 Jan 1990', '2 Jan 1990', '3 Jan 1990']);
  const { html, state } = draw(c, [line('USA', [100, 101, 102])]);
  assert.match(html, />1 Jan 1990<\/text>/);
  assert.match(html, />2 Jan 1990<\/text>/);
  assert.doesNotMatch(html, />Jan<\/text>/);
  close(state.times[1] - state.times[0], 1, 'one daily tick');
  close(state.times[2] - state.times[0], 2, 'two daily ticks');
  close(paths(html)[0][1][0], state.PADL + state.PW / 2, 'middle daily sample');
});

test('mixed monthly and daily observations occupy their elapsed calendar time', () => {
  const c = fixture([0, 1, 1 + 1 / 28, 2], ['Jan 1990', '1 Feb 1990', '2 Feb 1990', '1 Mar 1990']);
  const { html, state } = draw(c, [line('USA', [100, 110, 111, 120])]);
  close(state.times[1] - state.times[0], 31, 'January distance');
  close(state.times[2] - state.times[1], 1, 'one February day');
  close(state.times[3] - state.times[0], 59, 'January plus February');
  const points = paths(html)[0];
  assert(Math.abs(points[1][0] - (state.PADL + state.PW * 31 / 59)) <= .05);
  assert(Math.abs(points[2][0] - (state.PADL + state.PW * 32 / 59)) <= .05);
  assert(points[2][0] - points[1][0] < 14, 'adjacent daily points cannot take a third of the chart');
});

test('leap-day and month-boundary intervals remain one actual day each', () => {
  const c = fixture([25 + 27 / 29, 25 + 28 / 29, 26], ['28 Feb 1992', '29 Feb 1992', '1 Mar 1992']);
  const { state } = draw(c, [line('USA', [1, 2, 3])]);
  close(state.times[1] - state.times[0], 1, 'leap day');
  close(state.times[2] - state.times[1], 1, 'March boundary');
  const ordinary = fixture([13 + 27 / 28, 14], ['28 Feb 1991', '1 Mar 1991']);
  const other = draw(ordinary, [line('USA', [1, 2])]).state;
  close(other.times[1] - other.times[0], 1, 'ordinary February boundary');
});

test('a multi-year daily chart uses years, not the month word from daily labels', () => {
  const c = fixture([0, 12, 24, 36], ['1 Jan 1990', '1 Jan 1991', '1 Jan 1992', '1 Jan 1993']);
  const { html } = draw(c, [line('USA', [1, 2, 3, 4])]);
  for (const year of ['1990', '1991', '1992', '1993']) assert.match(html, new RegExp(`>${year}<\\/text>`));
  assert.doesNotMatch(html, />Jan<\/text>/);
});

test('truncated history starts at its retained date and excludes out-of-window events', () => {
  const dates = ['10 Jan 2000', '11 Jan 2000', '12 Jan 2000'];
  const c = fixture([120 + 9 / 31, 120 + 10 / 31, 120 + 11 / 31], dates, [
    event('9 Jan 2000'), event(dates[0]), event(dates[1]), event(dates[2]), event('13 Jan 2000'),
    event('11 Jan 2000', 'not selected', { tags: ['FRA'] }),
    event('11 Jan 2000', 'not a marked category', { cat: 'economy' }),
  ]);
  const marks = run(c, 'marksFor(["USA"])');
  assert.equal(marks.length, 3);
  assert.deepEqual(Array.from(marks, m => m.i), [0, 1, 2]);
  assert.deepEqual(Array.from(marks, m => m.title), dates.map(d => `${d} — ${d}`));
  const { html, state } = draw(c, [line('USA', [10, 11, 12])], { marks });
  const points = paths(html)[0];
  close(points[0][0], state.PADL, 'first retained point at left');
  close(points[2][0], state.PADL + state.PW, 'last retained point at right');
  assert.doesNotMatch(html, /9 Jan 2000|13 Jan 2000|not selected/);
});

test('same-month events use their exact date between sparse observations', () => {
  const c = fixture([0, 1], ['Jan 1990', 'Feb 1990'], [event('16 Jan 1990'), event('25 Jan 1990')]);
  const marks = run(c, 'marksFor(["USA"])');
  close(marks[0].t, 15 / 31, 'first dated event');
  close(marks[1].t, 24 / 31, 'second dated event');
  const { html, state } = draw(c, [line('USA', [1, 2])], { marks });
  const xs = [...html.matchAll(/<rect x="([\d.-]+)"[^>]*\n\s*fill="#[^"]+" opacity="\.85"/g)].map(m => Number(m[1]) + 1);
  assert.equal(xs.length, 2);
  assert(Math.abs(xs[0] - (state.PADL + state.PW * 15 / 31)) <= .05);
  assert(Math.abs(xs[1] - (state.PADL + state.PW * 24 / 31)) <= .05);
  assert(xs[1] > xs[0], 'events sharing the old month index must no longer stack at the left');
});

test('legacy event months work, invalid dates fall back, and absent logs are harmless', () => {
  const c = fixture([1, 1 + 1 / 28, 2], ['1 Feb 1990', '2 Feb 1990', '1 Mar 1990'], [
    event('Feb 1990', 'month-only'), event('31 Feb 1990', 'bad-date fallback', { t: 1 }),
    event('', 'numeric month', { t: 2 }), event('', 'missing time', { t: null }),
    event('1 Jan 1990', 'before retained window', { t: 1 }),
  ]);
  const marks = run(c, 'marksFor()');
  assert.equal(marks.length, 3);
  assert.deepEqual(Array.from(marks, m => m.t), [1, 1, 2]);
  assert.deepEqual(Array.from(marks, m => m.i), [0, 0, 2]);
  c.S = {}; assert.equal(run(c, 'marksFor().length'), 0);
  c.HIST = null; assert.equal(run(c, 'marksFor().length'), 0);
});

test('hover uses nearest calendar observation and never extends dead or unborn series', () => {
  const c = fixture([0, 1, 1 + 1 / 28, 2], ['Jan 1990', '1 Feb 1990', '2 Feb 1990', '1 Mar 1990']);
  const { state } = draw(c, [line('Alive', [100, 101, 102, 103]), line('Ended', [50, 51]), line('Successor', [10, 11], 2)]);
  const h = hoverFixture(c, state);
  h.move(.2); // Jan 13: nearer Jan 1 than Feb 1, despite being 20% of the axis.
  assert.match(h.readout.innerHTML, /<b>Jan 1990<\/b>/);
  assert.match(h.readout.innerHTML, /Alive 100\.0/);
  assert.equal(h.dots[1].attrs.opacity, '1');
  assert.equal(h.dots[2].attrs.opacity, '0');
  close(Number(h.cur.attrs.x1), state.PADL, 'hover snaps to actual January sample');
  h.move(32 / 59);
  assert.match(h.readout.innerHTML, /<b>2 Feb 1990<\/b>/);
  assert.doesNotMatch(h.readout.innerHTML, /Ended/);
  assert.equal(h.dots[1].attrs.opacity, '0');
  assert.equal(h.dots[2].attrs.opacity, '1');
  close(Number(h.cur.attrs.x1), state.PADL + state.PW * 32 / 59, 'hover and path share x position');
  h.move(1);
  assert.match(h.readout.innerHTML, /Successor 11\.0/);
  assert.doesNotMatch(h.readout.innerHTML, /Ended/);
  h.leave();
  assert.equal(h.readout.textContent, 'Original hint');
  assert.equal(h.cur.attrs.opacity, '0');
  assert(h.dots.every(d => d.attrs.opacity === '0'));
});

test('existing chart hover retains its own date labels if the history payload refreshes', () => {
  const c = fixture([0, 1], ['Jan 1990', 'Feb 1990']);
  const { state } = draw(c, [line('USA', [1, 2])]);
  const h = hoverFixture(c, state);
  c.HIST = { t: [12, 13], labels: ['Jan 1991', 'Feb 1991'] };
  h.move(1);
  assert.match(h.readout.innerHTML, /Feb 1990/);
  assert.doesNotMatch(h.readout.innerHTML, /1991/);
});

test('ended nation labels remain at their own chronological endpoint without extending the path', () => {
  const c = fixture([0, 1, 1 + 1 / 28, 2], ['Jan 1990', '1 Feb 1990', '2 Feb 1990', '1 Mar 1990']);
  const { html, state } = draw(c, [line('Alive', [100, 101, 102, 103]), line('Ended', [50, 51])]);
  assert.equal(paths(html)[1].length, 2);
  const x = Number(/<text x="([\d.-]+)"[^>]*>Ended<\/text>/.exec(html)[1]);
  assert(Math.abs(x - (state.PADL + state.PW * 31 / 59 + 6)) <= .05);
  assert(x < state.W - state.PADR, 'a dead nation cannot be labelled in the live right margin');
});

test('empty histories remain empty and unusable time axes fall back without NaN paths', () => {
  const empty = fixture([], []);
  assert.match(draw(empty, [line('USA', [])]).html, /Advance time/);
  assert.equal(run(empty, 'marksFor().length'), 0);
  const c = fixture([0, null, 2], ['Jan 1990', 'Unknown', 'Mar 1990']);
  const { html, state } = draw(c, [line('USA', [1, 2, 3])]);
  assert.deepEqual(Array.from(state.times), [0, 1, 2]);
  assert.doesNotMatch(html, /NaN|Infinity/);
  close(paths(html)[0][1][0], state.PADL + state.PW / 2, 'safe fallback middle sample');
});
