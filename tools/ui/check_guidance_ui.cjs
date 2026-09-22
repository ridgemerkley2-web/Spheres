// Controller and rendered HTML checks only; no browser, network or simulation process.
const test = require('node:test');
const assert = require('node:assert/strict');
const fs = require('node:fs');
const Guidance = require('../../spheres-web/ui/guidance-ui.js');
const Tutorial = require('../../spheres-web/ui/tutorial-model.js');
const Advisor = require('../../spheres-web/ui/advisor-model.js');
const ids = Tutorial.lessons.map(lesson => lesson.id);
const copy = value => JSON.parse(JSON.stringify(value));
const routes = new Set(['home', 'budget', 'cash_flow', 'construction', 'industry', 'government', 'research', 'equipment', 'air', 'companies', 'world', 'campaign']);
// Load the real controller with an injected advisor so route handling is tested against the contract alone.
const guidanceSource = fs.readFileSync(require.resolve('../../spheres-web/ui/guidance-ui.js'), 'utf8');
function withAdvisor(advisor) {
  const module = {exports: {}};
  new Function('module', 'require', guidanceSource)(module, name => name === './tutorial-model.js' ? Tutorial : advisor);
  return module.exports;
}
const world = (overrides = {}) => ({session_id: 'campaign-a', player: 'USA', player_name: 'United States',
  t: 0, date: '1990-01-01', year: 1990, month: 1, day: 1, simulation_cadence: 'daily',
  programs: {enabled: true, due: true},
  nations: [{id: overrides.player || 'USA', name: overrides.player || 'United States', alive: true,
    annual_budget: {due: true, fiscal_year: 1990}}], ...overrides});
const production = () => ({mode: 'province_projects', nation: 'USA',
  construction_budget: {enrolled: true, daily_budget_bn: 0.01, available_bn: 0.01},
  queue: [], mine_queue: [], completed: [], suggestions: {as_of_day: 0, items: []}});
function deferred() {
  let resolve, reject;
  const promise = new Promise((yes, no) => { resolve = yes; reject = no; });
  return {promise, resolve, reject};
}
function fixture(options = {}) {
  const G = options.guidance || Guidance;
  let current = Object.hasOwn(options, 'state') ? options.state : world();
  let available = options.available !== false, busy = false;
  const stored = {[G.STORAGE_KEY]: options.stored ?? null, [G.RECEIPTS_KEY]: options.receipts ?? null};
  const reads = [], writes = [], navigation = [], pending = [];
  const storage = {
    getItem(key) { reads.push(key); if (options.readError) throw new Error('storage denied'); return stored[key] ?? null; },
    setItem(key, value) { writes.push([key, value]); if (options.writeError) throw new Error('quota exceeded'); stored[key] = value; }
  };
  const adapter = {
    storage: Object.hasOwn(options, 'storage') ? options.storage : storage,
    getState: () => current, canNavigate: () => available, busy: () => busy,
    supports: kind => routes.has(kind), navigate: action => navigation.push(copy(action)),
    readSnapshot(requested) { const task = deferred(); pending.push({requested, ...task}); return task.promise; }
  };
  const session = G.createSession(adapter);
  return {session, adapter, reads, writes, navigation, pending, stored,
    get current() { return current; }, set current(value) { current = value; },
    set busy(value) { busy = value; }, set available(value) { available = value; },
    packet(index = pending.length - 1) { return {state: copy(pending[index].requested), production: production()}; },
    async ready() {
      const promise = session.refresh();
      const packet = this.packet(); pending.at(-1).resolve(packet); await promise;
      assert.equal(session.state().status, 'ready'); return packet;
    }
  };
}
function rendered(overrides = {}) {
  return {view: 'advisors', filter: 'all', query: '', status: 'idle', notice: '', storageNotice: '',
    progress: Tutorial.normalize(null), cards: [], hiddenCount: 0, snapshot: null, canNavigate: true, ...overrides};
}

test('missing saved progress begins fresh and persists only the tutorial preference key', () => {
  const f = fixture();
  assert.deepEqual(f.reads, [Guidance.STORAGE_KEY, Guidance.RECEIPTS_KEY]);
  assert.equal(Guidance.STORAGE_KEY, 'spheres.guidance.tutorial.v1');
  assert.equal(Guidance.RECEIPTS_KEY, 'spheres.guidance.receipts.v1');
  assert.deepEqual(f.session.state().progress, Tutorial.normalize(null));
  f.session.learn({type: 'complete', id: ids[0]});
  assert.equal(f.writes.length, 1);
  assert.equal(f.writes[0][0], Guidance.STORAGE_KEY);
  assert.deepEqual(JSON.parse(f.writes[0][1]), f.session.state().progress);
  assert.deepEqual(f.navigation, []);
  assert.equal(f.pending.length, 0);
});

test('corrupted and future-schema storage recover without restoring invented completions', () => {
  for (const stored of ['not json', 'null', '[]', '{"version":2,"done":["map-time"],"started":true}',
    '{"version":1,"done":["constructor","__proto__"],"skipped":{},"started":"true"}']) {
    const f = fixture({stored});
    assert.deepEqual(f.session.state().progress, Tutorial.normalize(null), stored);
    f.session.learn({type: 'skip', id: ids[0]});
    assert.deepEqual(f.session.state().progress.skipped, [ids[0]]);
    assert.deepEqual(f.session.state().progress.done, []);
  }
  assert.equal({}.started, undefined);
});

test('missing or denied storage uses memory and explicitly discloses visit-only progress', () => {
  for (const options of [{storage: undefined}, {storage: null}, {readError: true}]) {
    const f = fixture(options);
    assert.match(f.session.state().storageNotice, /visit|memory|unavailable/i);
    f.session.learn({type: 'complete', id: ids[0]});
    assert.deepEqual(f.session.state().progress.done, [ids[0]]);
    assert.match(Guidance.render(f.session.state()), /visit|memory|unavailable/i);
  }
});

test('failed preference writes retain the latest learning state in memory', () => {
  const f = fixture({writeError: true});
  f.session.learn({type: 'complete', id: ids[0]});
  f.session.learn({type: 'skip', id: ids[1]});
  assert.deepEqual(f.session.state().progress.done, [ids[0]]);
  assert.deepEqual(f.session.state().progress.skipped, [ids[1]]);
  assert.match(f.session.state().storageNotice, /visit|memory|unavailable/i);
  assert.equal(f.writes.length, 2);
  assert.equal(f.pending.length, 0);
});

test('fresh advice evaluates exactly the validated snapshot using the actual advisor model', async () => {
  const f = fixture(); const packet = await f.ready();
  assert.equal(f.pending[0].requested, f.current);
  assert.equal(f.session.state().snapshot, packet.state);
  assert.deepEqual(f.session.state().cards, Advisor.evaluate(packet.state, packet.production, null, {saves: [], loads: []}));
  assert.deepEqual(f.navigation, []);
  assert.equal(f.writes.length, 0);
});

test('all four snapshot identity fields must match exactly before advice becomes ready', async () => {
  for (const [key, bad] of [['session_id', 'campaign-b'], ['player', 'Japan'], ['t', 1],
    ['t', '0'], ['date', '1990-01-02'], ['date', null]]) {
    const f = fixture(); const work = f.session.refresh(); const packet = f.packet();
    packet.state[key] = bad;
    f.pending[0].resolve(packet); await work;
    assert.equal(f.session.state().status, 'error', key);
    assert.equal(f.session.state().snapshot, null);
    assert.deepEqual(f.session.state().cards, []);
    assert.equal(f.session.follow({kind: 'budget'}, true), false);
    assert.deepEqual(f.navigation, []);
  }
  const f = fixture(); const work = f.session.refresh();
  f.pending[0].resolve({production: production()}); await work;
  assert.equal(f.session.state().status, 'error');
});

test('replacing the state object while loading rejects even an otherwise matching old response', async () => {
  const f = fixture(); const work = f.session.refresh(); const packet = f.packet();
  f.current = copy(f.current);
  f.pending[0].resolve(packet); await work;
  assert.equal(f.session.state().status, 'error');
  assert.match(f.session.state().notice, /changed|moved/i);
  assert.deepEqual(f.session.state().cards, []);
});

test('in-place session, player, turn or date mutation cannot keep ready advice actionable', async () => {
  for (const [key, value] of [['session_id', 'campaign-b'], ['player', 'Japan'], ['t', 1], ['date', '1990-01-02']]) {
    const f = fixture(); await f.ready();
    f.current[key] = value;
    assert.equal(f.session.follow({kind: 'budget'}, true), false, key);
    assert.deepEqual(f.navigation, []);
    f.session.changed();
    assert.equal(f.session.state().status, 'stale', key);
    assert.equal(f.session.state().snapshot, null);
  }
});

test('a response matching a mutated request object cannot bypass the original request identity', async () => {
  const f = fixture(); const work = f.session.refresh();
  f.current.t = 1; f.current.date = '1990-01-02';
  f.pending[0].resolve(f.packet()); await work;
  assert.equal(f.session.state().status, 'error');
  assert.equal(f.session.state().snapshot, null);
});

test('changed invalidates loading requests and late success or failure cannot replace stale state', async () => {
  for (const fail of [false, true]) {
    const f = fixture(); const work = f.session.refresh(); const packet = f.packet();
    f.current = world({t: 1, date: '1990-01-02'}); f.session.changed();
    assert.equal(f.session.state().status, 'stale');
    const before = copy(f.session.state());
    if (fail) f.pending[0].reject(new Error('obsolete failure')); else f.pending[0].resolve(packet);
    await work;
    assert.deepEqual(f.session.state(), before);
  }
});

test('cancel discards late refresh responses without repainting closed guidance', async () => {
  for (const fail of [false, true]) {
    const f = fixture(); const work = f.session.refresh(); const packet = f.packet();
    let emissions = 0; const off = f.session.subscribe(() => emissions++);
    f.session.cancel(); const before = copy(f.session.state()); const count = emissions;
    if (fail) f.pending[0].reject(new Error('cancelled failure')); else f.pending[0].resolve(packet);
    await work;
    assert.deepEqual(f.session.state(), before);
    assert.equal(emissions, count);
    assert.equal(f.session.state().status, 'idle'); off();
  }
});

test('out-of-order refresh completion cannot replace the latest success or latest error', async () => {
  for (const latestFails of [false, true]) {
    const f = fixture(); const oldWork = f.session.refresh(); const oldPacket = f.packet();
    const newWork = f.session.refresh(); const newPacket = f.packet();
    if (latestFails) f.pending[1].reject(new Error('current failure')); else f.pending[1].resolve(newPacket);
    await newWork; const before = copy(f.session.state());
    f.pending[0].resolve(oldPacket); await oldWork;
    assert.deepEqual(f.session.state(), before);
    assert.equal(f.session.state().status, latestFails ? 'error' : 'ready');
  }
});

test('a failed refresh is retryable and successful retry clears the old error', async () => {
  const f = fixture(); const failed = f.session.refresh();
  f.pending[0].reject(new Error('temporary outage')); await failed;
  assert.equal(f.session.state().status, 'error');
  assert.match(f.session.state().notice, /temporary outage/);
  await f.ready();
  assert.equal(f.session.state().notice, '');
  assert.ok(f.session.state().snapshot);
});

test('navigation requires a campaign and readiness for advice, but tutorial reading is independent', async () => {
  for (const options of [{state: null}, {state: {}}, {available: false}]) {
    const f = fixture(options); await f.session.refresh();
    assert.equal(f.pending.length, 0);
    assert.equal(f.session.follow({kind: 'home'}), false);
    assert.deepEqual(f.navigation, []);
    f.session.learn({type: 'complete', id: ids[0]});
    assert.deepEqual(f.session.state().progress.done, [ids[0]]);
  }
  const f = fixture();
  assert.equal(f.session.follow({kind: 'budget'}, true), false, 'idle advice');
  const work = f.session.refresh();
  assert.equal(f.session.follow({kind: 'budget'}, true), false, 'loading advice');
  f.pending[0].reject(new Error('offline')); await work;
  assert.equal(f.session.follow({kind: 'budget'}, true), false, 'failed advice');
  assert.deepEqual(f.navigation, []);
});

test('busy state blocks tutorial and advice navigation without calling the adapter', async () => {
  const f = fixture(); await f.ready(); f.busy = true;
  for (const advice of [false, true]) assert.equal(f.session.follow({kind: 'budget'}, advice), false);
  assert.deepEqual(f.navigation, []);
  assert.match(f.session.state().notice, /pending|confirmation/i);
  f.busy = false;
  assert.equal(f.session.follow({kind: 'budget'}, true), true);
  assert.deepEqual(f.navigation, [{kind: 'budget'}]);
});

test('tutorial navigation is explicit, read-only routing and never marks a lesson complete', () => {
  const f = fixture(); const before = copy(f.session.state().progress);
  assert.equal(f.session.follow(Tutorial.lessons[0].action), true);
  assert.deepEqual(f.navigation, [{kind: 'home'}]);
  assert.deepEqual(f.session.state().progress, before);
  assert.equal(f.pending.length, 0); assert.equal(f.writes.length, 0);
  for (const action of [null, {}, {kind: '__proto__'}, {kind: 'constructor'}, {kind: '<script>bad</script>'},
    {kind: 'advance'}, {kind: 'buy_equipment'}, {kind: 1}]) {
    assert.equal(f.session.follow(action), false);
  }
  assert.equal(f.navigation.length, 1);
});

test('reading choices survive refresh, campaign switches, filters and view changes', async () => {
  const f = fixture();
  f.session.learn({type: 'complete', id: ids[0]});
  f.session.learn({type: 'skip', id: ids[1]});
  f.session.learn({type: 'select', id: ids[0]});
  const progress = copy(f.session.state().progress); const writes = f.writes.length;
  await f.ready(); f.session.setView('advisors'); f.session.setFilter('economy');
  f.session.setView('glossary'); f.session.setQuery('funding');
  f.current = world({session_id: 'campaign-b', player: 'Japan'}); f.session.changed(); await f.ready();
  assert.deepEqual(f.session.state().progress, progress);
  assert.equal(f.writes.length, writes);
  const restored = fixture({stored: JSON.stringify(progress)});
  assert.deepEqual(restored.session.state().progress, progress);
});

test('hide and restore affect advice visibility only, and a new campaign clears hidden advice', async () => {
  const f = fixture(); await f.ready();
  const cards = f.session.state().cards;
  assert.ok(cards.length, 'The actual model supplies actionable or unavailable cards for the opening reading');
  const progress = copy(f.session.state().progress);
  f.session.dismiss(cards[0].id);
  assert.equal(f.session.state().hiddenCount, 1);
  assert.equal(f.session.state().cards.length, cards.length - 1);
  f.session.dismiss(cards[0].id); f.session.dismiss('__proto__');
  assert.equal(f.session.state().hiddenCount, 1);
  assert.match(Guidance.render({...f.session.state(), view: 'advisors'}), /Show 1 hidden/);
  f.session.restore(); assert.deepEqual(f.session.state().cards, cards);
  f.session.dismiss(cards[0].id);
  f.current = world({session_id: 'campaign-b'}); f.session.changed(); await f.ready();
  assert.equal(f.session.state().hiddenCount, 0);
  assert.deepEqual(f.session.state().progress, progress);
  assert.equal(f.writes.length, 0);
});

test('render distinguishes unavailable, loading, stale and empty ready briefings', () => {
  const failed = Guidance.render(rendered({status: 'error', notice: 'Network unavailable'}));
  assert.match(failed, /The briefing is unavailable/);
  assert.doesNotMatch(failed, /No recommendations in this view/);
  assert.match(Guidance.render(rendered({status: 'loading'})), /aria-busy="true"/);
  assert.match(Guidance.render(rendered({status: 'stale'})), /A fresh briefing is needed/);
  const empty = Guidance.render(rendered({status: 'ready'}));
  assert.match(empty, /No recommendations in this view/);
  assert.match(empty, /Missing data is never treated as a clean bill of health/);
  const unavailable = Guidance.render(rendered({canNavigate: false}));
  assert.match(unavailable, /Continue or start a campaign/);
  assert.match(unavailable, /data-guidance-refresh disabled/);
});

test('render escapes untrusted advice, attributes, snapshot names and notices without routing injection', () => {
  const attack = '"><script>attack()</script><img src=x onerror="attack()">&\'';
  const html = Guidance.render(rendered({status: 'ready', notice: attack, storageNotice: attack,
    snapshot: {player: attack, date: attack}, cards: [{id: attack, area: attack, priority: attack, title: attack,
      reason: attack, evidence: [attack], caution: attack, actionLabel: attack, action: {kind: 'advance', command: attack}}]}),
    () => '/safe.webp" onerror="attack()');
  assert.doesNotMatch(html, /<script|<img src=x|src="\/safe.webp" onerror=|data-guidance-follow=""/);
  assert.match(html, /&lt;script&gt;attack\(\)&lt;\/script&gt;/);
  assert.match(html, /&quot;/); assert.match(html, /&amp;/); assert.match(html, /&#39;/);
  assert.doesNotMatch(html, /data-guidance-follow="advance"|data-guidance-command|onclick=/);
  const search = Guidance.render(rendered({view: 'glossary', query: attack}));
  assert.doesNotMatch(search, /<script|<img src=x/);
  assert.match(search, /No matching concepts/);
});

test('filters, invalid view input and glossary queries remain presentation choices', async () => {
  const f = fixture(); await f.ready();
  f.session.setView('__proto__'); assert.equal(f.session.state().view, 'tutorial');
  f.session.setFilter('constructor'); assert.equal(f.session.state().filter, 'all');
  f.session.setView('glossary'); f.session.setQuery('x'.repeat(500));
  assert.equal(f.session.state().query.length, 200);
  f.session.setQuery('party leader');
  assert.match(Guidance.render(f.session.state()), /Party leader and national leader/);
  assert.deepEqual(f.navigation, []); assert.equal(f.writes.length, 0);
});

test('rendered tutorial completion excludes skips and keeps skipped chapters replayable', () => {
  const f = fixture();
  for (let i = 0; i < ids.length; i++) f.session.learn({type: i === 2 ? 'skip' : 'complete', id: ids[i]});
  let html = Guidance.render(f.session.state());
  assert.match(html, /9 read · 1 skipped · 10 lessons/);
  assert.match(html, /<progress max="10" value="9"/);
  assert.match(html, /reached the end of the tour/);
  assert.doesNotMatch(html, /finished the introduction/);
  assert.match(html, /data-guidance-lesson="construction-effects"/);
  f.session.learn({type: 'select', id: ids[2]});
  html = Guidance.render(f.session.state());
  assert.match(html, /data-guidance-complete/);
  f.session.learn({type: 'complete', id: ids[2]});
  html = Guidance.render(f.session.state());
  assert.match(html, /10 read · 0 skipped · 10 lessons/);
  assert.match(html, /never counts as a campaign result/);
  assert.match(html, /finished the introduction/);
});

test('subscriptions unsubscribe cleanly without affecting model progress', () => {
  const f = fixture(); const events = [];
  const off = f.session.subscribe(value => events.push(value.view));
  f.session.setView('glossary'); off(); f.session.setView('tutorial');
  assert.deepEqual(events, ['glossary']);
  assert.deepEqual(f.session.state().progress, Tutorial.normalize(null));
});

// Minimal dialog/focus surface reproducing detached focus and disabled controls.
// It is deliberately not a browser runner: rendering and all event handlers are real.
function dialogDocument() {
  const doc = {activeElement: null};
  function element(tag, attributes = {}) {
    const node = {tagName: tag.toUpperCase(), attrs: {...attributes}, children: [], parentNode: null,
      isConnected: true, hidden: false, handlers: {}, value: attributes.value || '',
      get disabled() { return Object.hasOwn(this.attrs, 'disabled'); },
      set disabled(value) { if (value) this.attrs.disabled = ''; else delete this.attrs.disabled; },
      get type() { return this.attrs.type || ''; }, set type(value) { this.attrs.type = value; },
      get attributes() { return Object.entries(this.attrs).map(([name, value]) => ({name, value})); },
      get dataset() { return Object.fromEntries(Object.entries(this.attrs).filter(([key]) => key.startsWith('data-'))
        .map(([key, value]) => [key.slice(5).replace(/-([a-z])/g, (_, c) => c.toUpperCase()), value])); },
      getAttribute(key) { return Object.hasOwn(this.attrs, key) ? this.attrs[key] : null; },
      hasAttribute(key) { return Object.hasOwn(this.attrs, key); },
      setAttribute(key, value) { this.attrs[key] = String(value); },
      removeAttribute(key) { delete this.attrs[key]; },
      contains(other) { return this === other || this.children.some(child => child.contains(other)); },
      focus() { if (this.isConnected && !this.disabled) doc.activeElement = this; },
      closest(selector) { return selector === 'button' && this.tagName === 'BUTTON' ? this : null; },
      append(child) { this.children.push(child); child.parentNode = this; },
      addEventListener(kind, fn) { (this.handlers[kind] ||= []).push(fn); },
      dispatch(kind, event = {}) { for (const fn of this.handlers[kind] || []) fn(event); },
      matches(selector) {
        if (selector.startsWith('.')) return String(this.attrs.class || '').split(' ').includes(selector.slice(1));
        const attr = /^\[([^=\]]+)(?:="([^"]*)")?\]$/.exec(selector);
        if (attr) return this.hasAttribute(attr[1]) && (attr[2] === undefined || this.getAttribute(attr[1]) === attr[2]);
        return selector.toUpperCase() === this.tagName;
      },
      querySelectorAll(selector) {
        const choices = selector.split(',').map(value => value.trim());
        return this.children.flatMap(child => [...(choices.some(part => child.matches(part)) ? [child] : []),
          ...child.querySelectorAll(selector)]);
      },
      querySelector(selector) { return this.querySelectorAll(selector)[0] || null; },
      showModal() { this.open = true; this.focus(); },
      close() { this.open = false; this.dispatch('close'); },
      setSelectionRange(start, end) { this.selectionStart = start; this.selectionEnd = end; }
    };
    Object.defineProperty(node, 'innerHTML', {
      get() { return this.html || ''; },
      set(html) {
        if (this !== doc.body && this.children.some(child => child.contains(doc.activeElement))) doc.activeElement = doc.body;
        for (const old of this.children) old.isConnected = false;
        this.children = []; this.html = html;
        if (this.tagName === 'DIALOG') {
          this.append(element('button', {'data-guidance-close': '', type: 'button'}));
          this.append(element('div', {class: 'guidance-content'})); return;
        }
        for (const match of html.matchAll(/<(button|input|select|summary|h3)\b([^>]*)>/g)) {
          const attrs = {};
          for (const attr of match[2].matchAll(/([^\s=]+)(?:="([^"]*)")?/g)) attrs[attr[1]] = attr[2] || '';
          this.append(element(match[1], attrs));
        }
      }
    });
    return node;
  }
  doc.body = element('body'); doc.activeElement = doc.body;
  doc.createElement = tag => element(tag);
  doc.querySelector = selector => doc.body.querySelector(selector);
  return doc;
}

test('mount opens a paused tutorial and routes only after an explicit lesson click', () => {
  const f = fixture(); const document = dialogDocument(); let pauses = 0;
  const ui = Guidance.mount({...f.adapter, document, pause() { pauses++; }});
  ui.open('tutorial');
  const box = document.body.children.find(node => node.tagName === 'DIALOG');
  assert.equal(pauses, 1); assert.equal(box.open, true);
  assert.deepEqual(f.navigation, []);
  assert.equal(f.pending.length, 1, 'Opening the tutorial on a live campaign reads its current results');
  assert.equal(f.writes.length, 0);
  const button = box.querySelector('[data-guidance-open-lesson]');
  box.dispatch('click', {target: button});
  assert.deepEqual(f.navigation, [{kind: 'home'}]);
  assert.equal(box.open, false);
  assert.deepEqual(ui.session.state().progress.done, []);
  assert.equal(ui.session.state().progress.started, true);
});

test('hosts can suppress the floating launcher without disabling guidance or tutorial return progress', () => {
  for (const launcher of [undefined, false]) {
    const f = fixture(); const document = dialogDocument(); let pauses = 0;
    const ui = Guidance.mount({...f.adapter, document, launcher, pause() { pauses++; }});
    const button = document.body.children.find(node => node.id === 'guidanceLauncher');
    assert(button);
    ui.changed(); assert.equal(button.hidden, launcher === false);
    ui.open('tutorial');
    const box = document.body.children.find(node => node.tagName === 'DIALOG');
    assert.equal(box.open, true); assert.equal(pauses, 1);
    box.dispatch('click', {target: box.querySelector('[data-guidance-open-lesson]')});
    assert.equal(box.open, false); assert.deepEqual(f.navigation, [{kind: 'home'}]);
    assert.equal(ui.session.state().progress.started, true);
    assert.match(button.textContent, /Tutorial/);
    assert.equal(button.hidden, launcher === false, 'following a lesson must retain the host visibility choice');
    f.available = false; ui.changed(); assert.equal(button.hidden, true);
    f.available = true; ui.changed(); assert.equal(button.hidden, launcher === false);
    ui.open('tutorial'); assert.equal(box.open, true, 'the host entry point still reopens the saved lesson');
    assert.equal(ui.session.state().progress.started, true);
    assert.deepEqual(f.navigation, [{kind: 'home'}], 'reopening guidance issues no extra order');
  }
});

test('refresh and hiding the focused last card retain enabled focus inside the guidance dialog', async () => {
  const f = fixture(); const document = dialogDocument();
  const ui = Guidance.mount({...f.adapter, document, pause() {}});
  ui.open('advisors');
  f.pending[0].resolve(f.packet()); await f.pending[0].promise;
  const box = document.body.children.find(node => node.tagName === 'DIALOG');
  const refresh = box.querySelector('[data-guidance-refresh]'); refresh.focus();
  box.dispatch('click', {target: refresh});
  assert.equal(ui.session.state().status, 'loading');
  assert.ok(box.contains(document.activeElement), 'Disabled refresh cannot leave keyboard focus on the page behind the dialog');
  assert.equal(document.activeElement.disabled, false);
  f.pending[1].resolve(f.packet()); await f.pending[1].promise;
  const hide = box.querySelectorAll('[data-guidance-hide]').at(-1);
  assert.ok(hide); hide.focus(); box.dispatch('click', {target: hide});
  assert.ok(box.contains(document.activeElement), 'Removing the last focused card keeps keyboard focus inside guidance');
  assert.equal(document.activeElement.disabled, false);
  let stopped = 0; box.dispatch('keydown', {key: 'n', stopPropagation() { stopped++; }});
  assert.equal(stopped, 1, 'Guidance handles bubbling keyboard events before page shortcuts');
  assert.deepEqual(f.navigation, []);
});

// First-hour route. The advisor is injected, so these checks hold the controller to the route contract only.
const milestone = (id, label, status, date = null, detail = null) => ({id, label, status, day: date ? 0 : null, date, detail});
function sampleRoute(overrides = {}) {
  return {status: 'ready', reason: null, as_of: {session_id: 'campaign-a', player: 'USA', date: '1990-01-01', day: 0},
    steps: [
      {id: 'finances', title: 'Enact a yearly budget', lessons: ['budget-treasury'], status: 'achieved', summary: 'The yearly budget is enacted.',
        milestones: [milestone('budget', 'Yearly budget enacted', 'done', '1990-01-01', 'program_budget')], obstacle: null},
      {id: 'construction', title: 'Fund a useful construction project', lessons: ['construction-effects', 'industry-operations'], status: 'not_yet',
        summary: 'No project has paid work yet.', milestones: [milestone('paid', 'Paid work', 'pending')],
        obstacle: {title: 'Choose a project to fund', reason: 'No player project has paid work.', action: {kind: 'construction'}, actionLabel: 'Review construction'}},
      {id: 'research_design', title: 'Research toward a design', lessons: ['research-components'], status: 'progress', summary: 'Research is active.',
        milestones: [milestone('active', 'Component research active', 'done', '1990-01-01'), milestone('result', 'Dated design or research result', 'pending')],
        obstacle: {title: 'Save a design', reason: 'No dated design yet.', action: {kind: 'equipment', tab: 'designer'}, actionLabel: 'Open the designer'}},
      {id: 'procurement', title: 'Buy equipment from a company', lessons: ['equipment-procurement'], status: 'not_yet', summary: 'No certified product yet.',
        milestones: [milestone('delivered', 'Delivery received', 'pending')],
        obstacle: {title: 'No certified product', reason: 'Development takes at least 180 days.', action: {kind: 'companies'}, actionLabel: 'Review companies'}},
      {id: 'air_force', title: 'Prepare an air force', lessons: ['air-force'], status: 'unknown', summary: 'Aviation records are unavailable.',
        milestones: [milestone('funded', 'Airbase project funded', 'unknown')], obstacle: null},
      {id: 'save_resume', title: 'Save and resume', lessons: ['save-review'], status: 'not_yet', summary: 'No native save receipt yet.',
        milestones: [milestone('saved', 'Saved', 'pending')],
        obstacle: {title: 'Save this campaign', reason: 'A named save lets you resume.', action: {kind: 'campaign'}, actionLabel: 'Open campaigns'}}
    ], ...overrides};
}
function fakeAdvisor(route = () => sampleRoute()) {
  const calls = {recognize: [], evaluate: []};
  return {calls, evaluate(...args) { calls.evaluate.push(args); return []; },
    recognize(response, receipts) { calls.recognize.push({response, receipts}); return route(response, receipts); }};
}
function routeFixture(options = {}) {
  const advisor = options.advisor || fakeAdvisor();
  const G = withAdvisor(advisor), f = fixture({...options, guidance: G});
  f.G = G; f.advisor = advisor; return f;
}
// Six step fragments of the rendered route panel, keyed by step id.
function routeCards(html) {
  const cards = {};
  for (const part of html.split('data-guidance-route-card="').slice(1)) cards[part.slice(0, part.indexOf('"'))] = part;
  return cards;
}
const outcomesFor = (overrides = {}) => ({nation: 'USA', date: '1 Jan 1990', as_of_day: 0, money: {journal_available: true, decisions: []}, ...overrides});
const saveInput = (state, overrides = {}) => ({path: '/api/save', body: {slot: 'first-hour', session_id: state.session_id},
  data: {ok: true, slot: 'first-hour', path: 'saves/first-hour.json', date: state.date, history_points: 1, dispatches: 3}, state, ...overrides});
const loadInput = (state, overrides = {}) => ({path: '/api/load', body: {slot: 'first-hour', backup: false},
  data: {session_id: 'campaign-z', player: 'USA', date: '1990-01-01', storage_notice: 'Campaign and its history restored.', dispatch_count: 3}, state, ...overrides});
async function readyWith(f, extra = {}) {
  const work = f.session.refresh(); const packet = {...f.packet(), ...extra};
  f.pending.at(-1).resolve(packet); await work;
  assert.equal(f.session.state().status, 'ready'); return packet;
}

test('the route is recognized from the whole validated response with stored receipts', async () => {
  const f = routeFixture(); const packet = await readyWith(f, {outcomes: outcomesFor()});
  assert.equal(f.advisor.calls.recognize.length, 1);
  const {response, receipts} = f.advisor.calls.recognize[0];
  assert.equal(response.state, packet.state); assert.equal(response.production, packet.production);
  assert.equal(response.outcomes, packet.outcomes);
  assert.deepEqual(receipts, {saves: [], loads: []});
  assert.equal(f.advisor.calls.evaluate[0][0], packet.state);
  assert.equal(f.advisor.calls.evaluate[0][2], packet.outcomes);
  assert.deepEqual(f.advisor.calls.evaluate[0][3], {saves: [], loads: []});
  assert.deepEqual(f.session.state().route, sampleRoute());
  assert.equal(f.session.state().routeStale, false);
  assert.equal(f.session.state().routeChecked, '1990-01-01');
  assert.equal(f.session.state().routeAvailable, true);
  assert.deepEqual(f.navigation, []); assert.equal(f.writes.length, 0);
});

test('outcomes for another nation or day are unknown without failing the advice reading', async () => {
  const bad = [outcomesFor({nation: 'FRA'}), outcomesFor({as_of_day: 1}), outcomesFor({as_of_day: '0'}), outcomesFor({as_of_day: undefined}),
    outcomesFor({nation: undefined}), 'outcomes', [outcomesFor()]];
  for (const outcomes of bad) {
    const f = routeFixture(); await readyWith(f, {outcomes});
    assert.equal(f.advisor.calls.recognize[0].response.outcomes, null, JSON.stringify(outcomes));
    assert.equal(f.advisor.calls.evaluate[0][2], null);
  }
  for (const state of [world({day: 32}), world({year: 1989}), world({month: undefined})]) {
    const f = routeFixture({state}); await readyWith(f, {outcomes: outcomesFor()});
    assert.equal(f.advisor.calls.recognize[0].response.outcomes, null, 'A served date that has no day index cannot match outcomes');
  }
  const later = routeFixture({state: world({t: 0, date: '1990-01-02', day: 2})});
  await readyWith(later, {outcomes: outcomesFor({as_of_day: 1})});
  assert.equal(later.advisor.calls.recognize[0].response.outcomes.as_of_day, 1, 'as_of_day is the served state date');
  for (const outcomes of [null, undefined]) {
    const f = routeFixture(); await readyWith(f, {outcomes});
    assert.equal(f.advisor.calls.recognize[0].response.outcomes, null);
  }
});

test('reading progress never changes the route and a skipped lesson keeps an achieved result', async () => {
  const f = routeFixture(); await readyWith(f, {outcomes: outcomesFor()});
  const route = f.session.state().route;
  let cards = routeCards(f.G.render({...f.session.state(), view: 'tutorial'}));
  assert.deepEqual(Object.keys(cards), ['finances', 'construction', 'research_design', 'procurement', 'air_force', 'save_resume']);
  assert.match(cards.finances, /Reading:<\/span> Not read/);
  assert.match(cards.finances, /Campaign:<\/span> Achieved in this campaign · as of 1990-01-01/);
  f.session.learn({type: 'skip', id: 'budget-treasury'});
  f.session.learn({type: 'complete', id: 'construction-effects'});
  assert.equal(f.session.state().route, route); assert.equal(f.advisor.calls.recognize.length, 1);
  cards = routeCards(f.G.render({...f.session.state(), view: 'tutorial'}));
  assert.match(cards.finances, /Reading:<\/span> Skipped/);
  assert.match(cards.finances, /Campaign:<\/span> Achieved in this campaign/);
  assert.match(cards.construction, /Reading:<\/span> Not read · 1 of 2 read/);
  assert.match(cards.construction, /Campaign:<\/span> Not yet/);
  f.session.learn({type: 'complete', id: 'save-review'});
  cards = routeCards(f.G.render({...f.session.state(), view: 'tutorial'}));
  assert.match(cards.save_resume, /Reading:<\/span> Read/);
  assert.match(cards.save_resume, /Campaign:<\/span> Not yet/, 'Reading a lesson never completes its campaign step');
  assert.match(cards.research_design, /Campaign:<\/span> In progress/);
  assert.match(cards.air_force, /Campaign:<\/span> Unknown/);
  assert.match(cards.research_design, /Done<\/span><span>Component research active · <time datetime="1990-01-01">1990-01-01<\/time>/);
  assert.match(cards.research_design, /Not yet<\/span><span>Dated design or research result<\/span>/);
});

test('changed keeps a labelled last-checked route only for the same session and player', async () => {
  const f = routeFixture(); await readyWith(f, {outcomes: outcomesFor()});
  const route = copy(f.session.state().route);
  f.current.t = 1; f.current.date = '1990-01-02'; f.current.day = 2; f.session.changed();
  let model = f.session.state();
  assert.equal(model.status, 'stale'); assert.deepEqual(model.route, route);
  assert.equal(model.routeStale, true); assert.equal(model.routeChecked, '1990-01-01');
  let html = f.G.render({...model, view: 'tutorial'});
  assert.match(html, /Last checked 1990-01-01 — refresh/);
  assert.match(routeCards(html).finances, /Last checked 1990-01-01: Achieved in this campaign/);
  assert.match(routeCards(html).construction, /data-guidance-route-step="construction"[^>]* disabled>/);
  assert.equal(f.session.follow(route.steps[1].obstacle.action, true), false);
  assert.deepEqual(f.navigation, []);
  f.current = {...f.current}; f.session.changed();
  assert.deepEqual(f.session.state().route, route, 'A replaced state object of the same campaign keeps the labelled route');
  const loading = f.session.refresh();
  assert.equal(f.session.state().routeStale, true, 'A route stays labelled while its replacement loads');
  f.pending.at(-1).resolve({...f.packet(), outcomes: outcomesFor({as_of_day: 1})}); await loading;
  assert.equal(f.session.state().routeStale, false); assert.equal(f.session.state().routeChecked, '1990-01-02');
  for (const next of [world({session_id: 'campaign-b'}), world({player: 'Japan'}), null]) {
    const g = routeFixture(); await readyWith(g, {outcomes: outcomesFor()});
    g.current = next; g.session.changed();
    model = g.session.state();
    assert.equal(model.route, null, JSON.stringify(next)); assert.equal(model.routeChecked, null);
    html = g.G.render({...model, view: 'tutorial'});
    assert.doesNotMatch(html, /Last checked|data-guidance-route-card/);
  }
});

test('stale and out-of-order route readings are dropped before recognition', async () => {
  const f = routeFixture(); const work = f.session.refresh(); const packet = {...f.packet(), outcomes: outcomesFor()};
  f.current = world({t: 1, date: '1990-01-02', day: 2}); f.session.changed();
  f.pending[0].resolve(packet); await work;
  assert.equal(f.advisor.calls.recognize.length, 0); assert.equal(f.session.state().route, null);
  const g = routeFixture(); const oldWork = g.session.refresh(); const oldPacket = {...g.packet(), outcomes: outcomesFor()};
  const newWork = g.session.refresh(); g.pending[1].resolve({...g.packet(), outcomes: outcomesFor()}); await newWork;
  g.pending[0].resolve(oldPacket); await oldWork;
  assert.equal(g.advisor.calls.recognize.length, 1);
  const h = routeFixture(); const failing = h.session.refresh(); h.pending[0].reject(new Error('offline')); await failing;
  assert.equal(h.session.state().route, null); assert.equal(h.advisor.calls.recognize.length, 0);
  const i = routeFixture(); await readyWith(i, {outcomes: outcomesFor()});
  const retry = i.session.refresh(); i.pending.at(-1).reject(new Error('offline')); await retry;
  assert.equal(i.session.state().status, 'error'); assert.equal(i.session.state().routeStale, true, 'A failed refresh keeps only a labelled earlier route');
});

test('a malformed, throwing or absent recognizer never becomes a campaign result', async () => {
  for (const route of [() => null, () => ({status: 'ready'}), () => { throw new Error('bad'); }]) {
    const f = routeFixture({advisor: fakeAdvisor(route)}); await readyWith(f, {outcomes: outcomesFor()});
    assert.equal(f.session.state().route.status, 'unknown');
    assert.deepEqual(f.session.state().route.steps, []);
    assert.match(f.G.render({...f.session.state(), view: 'tutorial'}), /Campaign results are unknown/);
  }
  const G = withAdvisor({evaluate: () => []}); const f = fixture({guidance: G}); await readyWith(f, {outcomes: outcomesFor()});
  assert.equal(f.session.state().route, null); assert.equal(f.session.state().routeAvailable, false);
  assert.match(G.render({...f.session.state(), view: 'tutorial'}), /Campaign results are unavailable in this build/);
});

test('save and load receipts are validated exactly and stored under their own key', () => {
  const f = routeFixture(); const state = f.current;
  assert.equal(f.session.receipt(saveInput(state)), true);
  assert.deepEqual(f.writes.map(([key]) => key), [Guidance.RECEIPTS_KEY]);
  assert.deepEqual(JSON.parse(f.writes[0][1]), {version: 1, loads: [],
    saves: [{session_id: 'campaign-a', player: 'USA', slot: 'first-hour', date: '1990-01-01', dispatches: 3}]});
  const rejectedSaves = [
    saveInput(state, {data: {...saveInput(state).data, ok: false}}), saveInput(state, {data: {...saveInput(state).data, ok: 'true'}}),
    saveInput(state, {data: {...saveInput(state).data, slot: 'other'}}), saveInput(state, {data: {...saveInput(state).data, slot: undefined}}),
    saveInput(state, {data: {...saveInput(state).data, date: '1990-01-02'}}), saveInput(state, {data: {...saveInput(state).data, date: undefined}}),
    saveInput(state, {body: {slot: 'first-hour', session_id: 'campaign-b'}}), saveInput(state, {body: {slot: 'first-hour'}}),
    saveInput(state, {state: null}), saveInput(state, {state: {...state, player: null}}), saveInput(state, {body: undefined}),
    saveInput(state, {data: null}), saveInput(state, {path: '/api/command'}), saveInput(state, {path: '/api/new'}), null, 'save'];
  for (const input of rejectedSaves) assert.equal(f.session.receipt(input), false, JSON.stringify(input));
  assert.equal(f.writes.length, 1, 'Rejected receipts are never stored');
  assert.equal(f.session.receipt(loadInput(state)), true);
  assert.deepEqual(JSON.parse(f.writes.at(-1)[1]).loads,
    [{session_id: 'campaign-z', player: 'USA', slot: 'first-hour', backup: false, date: '1990-01-01', dispatch_count: 3}]);
  const load = loadInput(state).data;
  const rejectedLoads = [
    loadInput(state, {data: {...load, session_id: 'campaign-a'}}), loadInput(state, {data: {...load, session_id: ''}}),
    loadInput(state, {data: {...load, session_id: 7}}),
    loadInput(state, {data: {...load, storage_notice: 'Simulation save restored. Earlier history was not recorded in this file; new campaign saves preserve it.'}}),
    loadInput(state, {data: {...load, storage_notice: undefined}}), loadInput(state, {data: {...load, player: undefined}}),
    loadInput(state, {data: {...load, date: null}}), loadInput(state, {body: undefined})];
  for (const input of rejectedLoads) assert.equal(f.session.receipt(input), false, JSON.stringify(input));
  assert.equal(f.session.receipt(loadInput(null, {body: {slot: 'backup-slot', backup: true}, data: {...load, session_id: 'campaign-y', dispatch_count: undefined}})), true);
  assert.equal(f.session.receipt(loadInput(state, {body: {}, data: {...load, session_id: 'campaign-x'}})), true);
  assert.deepEqual(JSON.parse(f.writes.at(-1)[1]).loads.slice(1), [
    {session_id: 'campaign-y', player: 'USA', slot: 'backup-slot', backup: true, date: '1990-01-01', dispatch_count: null},
    {session_id: 'campaign-x', player: 'USA', slot: null, backup: false, date: '1990-01-01', dispatch_count: 3}]);
  assert.deepEqual(f.navigation, []); assert.equal(f.pending.length, 0);
});

test('receipts for a foreign session are ignored and recognition receives only validated copies', async () => {
  const f = routeFixture(); const state = f.current;
  assert.equal(f.session.receipt(saveInput(state, {body: {slot: 'first-hour', session_id: 'campaign-b'}})), false);
  assert.equal(f.session.receipt(saveInput(state)), true);
  await readyWith(f, {outcomes: outcomesFor()});
  const received = f.advisor.calls.recognize[0].receipts;
  assert.deepEqual(received, {saves: [{session_id: 'campaign-a', player: 'USA', slot: 'first-hour', date: '1990-01-01', dispatches: 3}], loads: []});
  received.saves[0].session_id = 'tampered'; received.saves.push({session_id: 'campaign-b'});
  await readyWith(f, {outcomes: outcomesFor()});
  assert.equal(f.advisor.calls.recognize[1].receipts.saves.length, 1);
  assert.equal(f.advisor.calls.recognize[1].receipts.saves[0].session_id, 'campaign-a');
});

test('a receipt re-reads the route and cards only for the current verified reading', async () => {
  const f = routeFixture(); await readyWith(f, {outcomes: outcomesFor()});
  assert.equal(f.session.receipt(saveInput(f.current)), true);
  assert.equal(f.advisor.calls.recognize.length, 2); assert.equal(f.advisor.calls.evaluate.length, 2);
  assert.equal(f.advisor.calls.recognize[1].response.outcomes.nation, 'USA');
  assert.equal(f.advisor.calls.evaluate[1][3].saves.length, 1);
  f.current.t = 1; f.current.date = '1990-01-02'; f.session.changed();
  assert.equal(f.session.receipt(saveInput(f.current)), true);
  assert.equal(f.advisor.calls.recognize.length, 2, 'A stale reading is never re-evaluated with new receipts');
  assert.equal(f.session.state().routeStale, true);
});

test('stored receipts are parsed within bounds and malformed rows are dropped', async () => {
  const save = i => ({session_id: 'campaign-a', player: 'USA', slot: `slot-${i}`, date: '1990-01-01', dispatches: i});
  const hostile = '{"__proto__":{"session_id":"campaign-a"},"player":"USA","slot":"x","date":"1990-01-01","dispatches":1}';
  const raw = `{"version":1,"saves":[${Array.from({length: 26}, (_, i) => JSON.stringify(save(i))).join(',')},"x",null,{"session_id":1},${hostile},` +
    `{"session_id":"campaign-a","player":"USA","slot":"s","date":"1990-01-01","dispatches":-1}],` +
    `"loads":[{"session_id":"campaign-z","player":"USA","slot":null,"backup":"yes","date":"1990-01-01","dispatch_count":1},` +
    `{"session_id":"campaign-z","player":"USA","slot":"first-hour","backup":false,"date":"1990-01-01","dispatch_count":1,"extra":"dropped"}]}`;
  const f = routeFixture({receipts: raw}); await readyWith(f, {outcomes: outcomesFor()});
  const received = f.advisor.calls.recognize[0].receipts;
  assert.deepEqual(received.saves, Array.from({length: 15}, (_, i) => save(i + 11)), 'Only the newest twenty stored entries are considered');
  assert.deepEqual(received.loads, [{session_id: 'campaign-z', player: 'USA', slot: 'first-hour', backup: false, date: '1990-01-01', dispatch_count: 1}]);
  assert.equal({}.session_id, undefined);
  for (const bad of ['not json', 'null', '[]', '7', '{"version":2,"saves":[],"loads":[]}', '{"version":1,"saves":[]}', '{"version":1,"saves":[],"loads":"oops"}',
    '{"version":1,"saves":[],"loads":[]', `{"version":1,"saves":[${JSON.stringify(save(1))}],"loads":[],"pad":"${'x'.repeat(100001)}"}`]) {
    const g = routeFixture({receipts: bad}); await readyWith(g, {outcomes: outcomesFor()});
    assert.deepEqual(g.advisor.calls.recognize[0].receipts, {saves: null, loads: null}, `An unreadable store vouches for nothing: ${bad.slice(0, 40)}`);
    assert.match(g.session.state().storageNotice, /could not be read/, bad.slice(0, 40));
  }
  const missing = routeFixture(); await readyWith(missing, {outcomes: outcomesFor()});
  assert.deepEqual(missing.advisor.calls.recognize[0].receipts, {saves: [], loads: []}, 'A missing record means none recorded in this browser');
  assert.equal(missing.session.state().storageNotice, '');
  const h = routeFixture();
  for (let i = 0; i < 25; i++) assert.equal(h.session.receipt(saveInput(h.current, {body: {slot: `slot-${i}`, session_id: 'campaign-a'}, data: {...saveInput(h.current).data, slot: `slot-${i}`}})), true);
  assert.equal(h.session.receipt(saveInput(h.current, {body: {slot: 'slot-10', session_id: 'campaign-a'}, data: {...saveInput(h.current).data, slot: 'slot-10'}})), true);
  const stored = JSON.parse(h.stored[Guidance.RECEIPTS_KEY]).saves;
  assert.equal(stored.length, 20);
  assert.deepEqual(stored.map(row => row.slot), [...Array.from({length: 20}, (_, i) => `slot-${i + 5}`).filter(slot => slot !== 'slot-10'), 'slot-10'],
    'The oldest receipts are dropped and a repeated identical receipt moves to the end instead of duplicating');
});

test('receipt storage failures keep receipts in memory and disclose visit-only storage', async () => {
  const f = routeFixture({writeError: true});
  assert.equal(f.session.state().storageNotice, '');
  assert.equal(f.session.receipt(saveInput(f.current)), true);
  assert.match(f.session.state().storageNotice, /receipts are kept for this visit/i);
  await readyWith(f, {outcomes: outcomesFor()});
  assert.equal(f.advisor.calls.recognize[0].receipts.saves.length, 1);
  assert.match(f.G.render({...f.session.state(), view: 'tutorial'}), /kept for this visit/);
  const g = routeFixture({readError: true});
  assert.match(g.session.state().storageNotice, /Progress and save receipts are kept for this visit/);
  assert.equal(g.session.receipt(saveInput(g.current)), true);
  await readyWith(g, {outcomes: outcomesFor()});
  assert.equal(g.advisor.calls.recognize[0].receipts.saves.length, 1);
});

// One browser storage shared by several tabs.
function sharedStorage() {
  const data = {};
  return {data, getItem: key => Object.hasOwn(data, key) ? data[key] : null, setItem(key, value) { data[key] = String(value); }};
}
const dated = (overrides = {}) => world({date: '1 Jan 1990', ...overrides});
const datedOutcomes = () => outcomesFor({date: '1 Jan 1990'});
const loadInto = (state, session) => loadInput(state, {data: {...loadInput(state).data, session_id: session, date: '1 Jan 1990'}});

test('receipts from another tab are merged before every write and every reading', async () => {
  const shared = sharedStorage();
  const a = routeFixture({storage: shared, state: dated()}), c = routeFixture({storage: shared, state: dated()});
  const b = routeFixture({storage: shared, state: dated()});
  assert.equal(b.session.receipt(loadInto(b.current, 'L2')), true, 'Tab B loads a save, which begins session L2');
  a.current = dated({session_id: 'L2'});
  assert.equal(a.session.receipt(saveInput(a.current)), true, 'Tab A, mounted earlier, continues into L2 and saves');
  const stored = JSON.parse(shared.data[Guidance.RECEIPTS_KEY]);
  assert.deepEqual(stored.loads.map(row => row.session_id), ['L2'], 'A write keeps the load receipt another tab stored');
  assert.deepEqual(stored.saves.map(row => row.session_id), ['L2']);
  c.current = dated({session_id: 'L2'}); await readyWith(c, {outcomes: datedOutcomes()});
  assert.deepEqual(c.advisor.calls.recognize[0].receipts.loads.map(row => row.session_id), ['L2'], 'A refresh reads receipts another tab stored after mount');
  assert.deepEqual(c.advisor.calls.recognize[0].receipts.saves.map(row => row.session_id), ['L2']);
  const reloaded = fixture({storage: shared, state: dated({session_id: 'L2'})}); await readyWith(reloaded, {outcomes: datedOutcomes()});
  const step = reloaded.session.state().route.steps.find(row => row.id === 'save_resume');
  assert.equal(step.status, 'achieved', 'After a reload the resumed session still counts');
  assert.deepEqual(step.milestones.map(row => row.status), ['done', 'done']);
});

test('a storage event for the receipt key re-reads only the current verified reading', async () => {
  const shared = sharedStorage(), handlers = [];
  const win = {addEventListener(type, fn) { if (type === 'storage') handlers.push(fn); }};
  const f = routeFixture({storage: shared, state: dated({session_id: 'L3'})});
  const ui = f.G.mount({...f.adapter, document: dialogDocument(), window: win, pause() {}});
  assert.equal(handlers.length, 1, 'Mount listens for storage changes from other tabs');
  ui.open('tutorial'); f.pending[0].resolve({...f.packet(), outcomes: datedOutcomes()}); await f.pending[0].promise;
  assert.equal(ui.session.state().status, 'ready');
  const before = f.advisor.calls.recognize.length;
  assert.deepEqual(f.advisor.calls.recognize.at(-1).receipts.loads, []);
  const other = routeFixture({storage: shared, state: dated()});
  assert.equal(other.session.receipt(loadInto(other.current, 'L3')), true);
  handlers[0]({key: 'unrelated'});
  assert.equal(f.advisor.calls.recognize.length, before, 'Other keys are ignored');
  handlers[0]({key: Guidance.RECEIPTS_KEY});
  assert.equal(f.advisor.calls.recognize.length, before + 1);
  assert.deepEqual(f.advisor.calls.recognize.at(-1).receipts.loads.map(row => row.session_id), ['L3']);
  assert.equal(f.advisor.calls.evaluate.at(-1)[3].loads.length, 1, 'Cards are re-evaluated with the same receipts');
  f.current.t = 1; ui.changed(); handlers[0]({key: null});
  assert.equal(f.advisor.calls.recognize.length, before + 1, 'A stale reading is never re-evaluated');
  assert.deepEqual(f.navigation, []);
});

test('an unreadable receipt store vouches for nothing, is never overwritten, and still counts this visit', async () => {
  const future = '{"version":2,"saves":[],"loads":[{"session_id":"L9"}]}';
  const f = routeFixture({receipts: future});
  assert.match(f.session.state().storageNotice, /could not be read/);
  assert.equal(f.session.receipt(saveInput(f.current)), true);
  assert.equal(f.session.receipt(loadInput(f.current)), true, 'A load receipt for another session is kept in memory');
  assert.deepEqual(f.writes, [], 'A record this version cannot read is never overwritten');
  assert.equal(f.stored[Guidance.RECEIPTS_KEY], future);
  await readyWith(f, {outcomes: outcomesFor()});
  assert.deepEqual(f.advisor.calls.recognize[0].receipts,
    {saves: [{session_id: 'campaign-a', player: 'USA', slot: 'first-hour', date: '1990-01-01', dispatches: 3}], loads: null},
    'Only a kind this visit recorded for this session is known; the other stays unknown');
  f.current = world({session_id: 'campaign-z'}); f.session.changed(); await readyWith(f, {outcomes: outcomesFor()});
  assert.deepEqual(f.advisor.calls.recognize.at(-1).receipts.loads.map(row => row.session_id), ['campaign-z'], 'A load in this visit can still count');
  assert.equal(f.advisor.calls.recognize.at(-1).receipts.saves, null);
  assert.match(f.G.render({...f.session.state(), view: 'tutorial'}), /could not be read/);
  const denied = routeFixture({readError: true}); await readyWith(denied, {outcomes: outcomesFor()});
  assert.deepEqual(denied.advisor.calls.recognize[0].receipts, {saves: null, loads: null}, 'A store that cannot be read at all vouches for nothing');
  for (const [receipts, status] of [['{"version":1,"saves":[],"loads":"oops"', 'unknown'], [null, 'not_yet']]) {
    const real = fixture({receipts, state: dated()}); await readyWith(real, {outcomes: datedOutcomes()});
    assert.equal(real.session.state().route.steps.find(row => row.id === 'save_resume').status, status, String(receipts));
  }
});

test('route panel escapes every served field and never renders an unknown status as progress', () => {
  const attack = '"><script>attack()</script><img src=x onerror="attack()">&\'';
  const route = {status: 'ready', reason: attack, as_of: null, steps: [{id: attack, title: attack, lessons: [attack, '__proto__'], status: 'done', summary: attack,
    milestones: [{id: attack, label: attack, status: 'achieved', day: 1, date: attack, detail: attack}],
    obstacle: {title: attack, reason: attack, action: {kind: 'construction', command: attack}, actionLabel: attack}}]};
  const html = Guidance.render(rendered({view: 'tutorial', status: 'ready', route, routeChecked: attack, routeAvailable: true}));
  assert.doesNotMatch(html, /<script|<img src=x|onerror="attack|data-guidance-command|onclick=/);
  assert.match(html, /data-guidance-route-step="&quot;&gt;&lt;script&gt;/);
  assert.match(html, /Campaign:<\/span> Unknown/); assert.match(html, /Reading:<\/span> No lesson/);
  assert.match(html, /Unknown<\/span><span>&quot;&gt;&lt;script&gt;/);
  const unknown = Guidance.render(rendered({view: 'tutorial', status: 'ready', routeAvailable: true, routeChecked: '1990-01-01',
    route: {...sampleRoute(), status: 'unknown', reason: 'The campaign identity did not match.'}}));
  assert.match(unknown, /Campaign results are unknown: The campaign identity did not match\./);
  assert.doesNotMatch(unknown, /Achieved in this campaign|In progress|Campaign:<\/span> Not yet/);
  assert.doesNotMatch(unknown, /class="guidance-milestone--done"/);
});

test('route panel states separate no campaign, loading, stale and ready readings', () => {
  const base = {view: 'tutorial', routeAvailable: true};
  const none = Guidance.render(rendered({...base, canNavigate: false}));
  assert.match(none, /Continue or start a campaign to check its results/);
  assert.match(none, /<button type="button" data-guidance-refresh disabled>/);
  assert.match(none, /Your first steps in Spheres/);
  const loading = Guidance.render(rendered({...base, status: 'loading'}));
  assert.match(loading, /class="guidance-route" aria-labelledby="guidanceRouteTitle" aria-busy="true"/);
  assert.match(loading, /Reading your campaign…/);
  const ready = Guidance.render(rendered({...base, status: 'ready', route: sampleRoute(), routeChecked: '1 Jan 1990'}));
  assert.match(ready, /<h3 id="guidanceRouteTitle">Your first hour<\/h3>/);
  assert.match(ready, /Checked 1 Jan 1990\. Only dated records from this campaign count\./);
  assert.match(ready, /<button type="button" data-guidance-route-step="construction" aria-label="Review construction: Fund a useful construction project" >Review construction/);
  assert.equal((ready.match(/data-guidance-route-step=/g) || []).length, 4, 'Only steps with an obstacle action get a review button');
  assert.equal((ready.match(/data-guidance-lesson="budget-treasury"/g) || []).length, 1, 'Browser CI selectors stay unique');
  const route = ready.slice(ready.indexOf('class="guidance-route"'), ready.indexOf('class="guidance-footer"'));
  assert.equal((route.match(/data-guidance-route-card=/g) || []).length, 6, 'The slice holds the whole route panel');
  assert.doesNotMatch(route, /<(a|div|span|li)[^>]*(tabindex|onclick)/, 'Every route control is a native button');
  assert.equal((route.match(/<button /g) || []).length, (route.match(/<button type="button"/g) || []).length);
  const unavailable = Guidance.render(rendered({...base, status: 'ready', route: sampleRoute(), routeChecked: '1 Jan 1990', canNavigate: false}));
  assert.match(unavailable, /data-guidance-route-step="construction"[^>]* disabled>/);
});

test('the lesson comes first and one plain line summarises campaign results beside lesson progress', () => {
  const base = {view: 'tutorial', routeAvailable: true, status: 'ready', routeChecked: '1 Jan 1990'};
  const ready = Guidance.render(rendered({...base, route: sampleRoute()}));
  const at = marker => { const i = ready.indexOf(marker); assert.ok(i >= 0, marker); return i; };
  assert.ok(at('class="guidance-progress"') < at('class="guidance-results-line"'));
  assert.ok(at('class="guidance-results-line"') < at('class="guidance-learning"'));
  for (const control of ['data-guidance-heading', 'data-guidance-complete', 'data-guidance-skip']) {
    assert.ok(at(control) < at('class="guidance-route"'), `${control} is not pushed below the route panel`);
  }
  assert.ok(at('class="guidance-route"') < at('class="guidance-footer"'));
  assert.match(ready, /<p class="guidance-results-line">Campaign results: 1 of 6 achieved · see Your first hour below<\/p>/);
  const stale = Guidance.render(rendered({...base, route: sampleRoute(), routeStale: true}));
  assert.match(stale, /<p class="guidance-results-line">Campaign results \(last checked 1 Jan 1990\): 1 of 6 achieved · see Your first hour below<\/p>/);
  const unknown = Guidance.render(rendered({...base, route: {...sampleRoute(), status: 'unknown', reason: 'No reading.'}}));
  assert.match(unknown, /<p class="guidance-results-line">Campaign results: unknown · see Your first hour below<\/p>/);
  assert.doesNotMatch(unknown, /of 6 achieved/, 'An unknown route never reports a count');
  assert.doesNotMatch(Guidance.render(rendered({...base, route: null})), /guidance-results-line/);
});

test('opening the tutorial reads a live campaign, and route buttons follow only ready current advice', async () => {
  const f = routeFixture(); const document = dialogDocument(); let pauses = 0;
  const ui = f.G.mount({...f.adapter, document, pause() { pauses++; }});
  assert.equal(typeof ui.receipt, 'function');
  ui.open('tutorial');
  const box = document.body.children.find(node => node.tagName === 'DIALOG');
  assert.equal(pauses, 1); assert.equal(f.pending.length, 1); assert.deepEqual(f.navigation, []);
  assert.equal(box.querySelector('[data-guidance-route-step]'), null);
  f.pending[0].resolve({...f.packet(), outcomes: outcomesFor()}); await f.pending[0].promise;
  const step = box.querySelector('[data-guidance-route-step="construction"]');
  assert.ok(step); assert.equal(step.disabled, false);
  step.focus(); ui.session.report('Keep focus');
  assert.equal(document.activeElement.getAttribute('data-guidance-focus'), 'data-guidance-route-step:construction', 'Focus survives a repaint');
  box.dispatch('click', {target: document.activeElement});
  assert.deepEqual(f.navigation, [{kind: 'construction'}]); assert.equal(box.open, false);
  assert.equal(f.writes.length, 0);
  ui.open('tutorial'); assert.equal(f.pending.length, 2);
  f.pending[1].resolve({...f.packet(), outcomes: outcomesFor()}); await f.pending[1].promise;
  f.current.date = '1990-01-02'; ui.changed();
  const stale = box.querySelector('[data-guidance-route-step="procurement"]');
  assert.ok(stale); assert.equal(stale.disabled, true);
  box.dispatch('click', {target: stale});
  assert.deepEqual(f.navigation, [{kind: 'construction'}], 'A stale route cannot open a review');
  const idle = routeFixture({available: false}); const other = dialogDocument();
  idle.G.mount({...idle.adapter, document: other, pause() {}}).open('tutorial');
  assert.equal(idle.pending.length, 0, 'No reading without a live campaign');
  const glossary = routeFixture(); glossary.G.mount({...glossary.adapter, document: dialogDocument(), pause() {}}).open('glossary');
  assert.equal(glossary.pending.length, 0);
});

test('the illustrative review page shows its annual-budget card and a readable first-hour route', async () => {
  const vm = require('node:vm');
  const source = fs.readFileSync(require.resolve('./guidance-review.js'), 'utf8');
  for (const scenario of ['opening', 'construction', 'empty', 'change']) {
    let adapter; const elements = {};
    vm.runInContext(source, vm.createContext({structuredClone, GuidanceUI: {mount(value) { adapter = value; return {open() {}, changed() {}}; }},
      document: {getElementById: id => elements[id] ||= {}}}));
    if (scenario === 'change') elements.change.onclick(); else if (scenario !== 'opening') elements.scenario.onchange({target: {value: scenario}});
    const data = await adapter.readSnapshot(), cards = Advisor.evaluate(data.state, data.production, data.outcomes, {saves: [], loads: []}).map(card => card.id);
    const route = Advisor.recognize(data, {saves: [], loads: []});
    assert.equal(cards.includes('annual-budget'), scenario === 'opening' || scenario === 'change', `${scenario}: ${cards}`);
    assert.equal(route.status, scenario === 'empty' ? 'unknown' : 'ready', `${scenario}: ${route.reason}`);
    if (scenario !== 'empty') assert.deepEqual(route.steps.map(step => step.status), Array(6).fill('not_yet'), scenario);
  }
});

test('milestone dates never break across lines inside the route panel',()=>{
  const css=require('node:fs').readFileSync(require('node:path').join(__dirname,'../../spheres-web/ui/guidance-ui.css'),'utf8');
  assert.match(css,/\.guidance-milestones time\{white-space:nowrap\}/,'the route panel allows breaking anywhere, so its dates must opt out');
});
