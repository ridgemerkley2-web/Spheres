// Controller and rendered HTML checks only; no browser, network or simulation process.
const test = require('node:test');
const assert = require('node:assert/strict');
const Guidance = require('../../spheres-web/ui/guidance-ui.js');
const Tutorial = require('../../spheres-web/ui/tutorial-model.js');
const Advisor = require('../../spheres-web/ui/advisor-model.js');
const ids = Tutorial.lessons.map(lesson => lesson.id);
const copy = value => JSON.parse(JSON.stringify(value));
const routes = new Set(['home', 'budget', 'construction', 'industry', 'government', 'research', 'equipment', 'world', 'campaign']);
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
  let current = Object.hasOwn(options, 'state') ? options.state : world();
  let available = options.available !== false, busy = false;
  let stored = options.stored ?? null;
  const reads = [], writes = [], navigation = [], pending = [];
  const storage = {
    getItem(key) { reads.push(key); if (options.readError) throw new Error('storage denied'); return stored; },
    setItem(key, value) { writes.push([key, value]); if (options.writeError) throw new Error('quota exceeded'); stored = value; }
  };
  const adapter = {
    storage: Object.hasOwn(options, 'storage') ? options.storage : storage,
    getState: () => current, canNavigate: () => available, busy: () => busy,
    supports: kind => routes.has(kind), navigate: action => navigation.push(copy(action)),
    readSnapshot(requested) { const task = deferred(); pending.push({requested, ...task}); return task.promise; }
  };
  const session = Guidance.createSession(adapter);
  return {session, adapter, reads, writes, navigation, pending,
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
  assert.deepEqual(f.reads, [Guidance.STORAGE_KEY]);
  assert.equal(Guidance.STORAGE_KEY, 'spheres.guidance.tutorial.v1');
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
  assert.deepEqual(f.session.state().cards, Advisor.evaluate(packet.state, packet.production));
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
  assert.match(html, /8 completed · 1 skipped · 9 lessons/);
  assert.match(html, /<progress max="9" value="8"/);
  assert.match(html, /reached the end of the tour/);
  assert.doesNotMatch(html, /finished the introduction/);
  assert.match(html, /data-guidance-lesson="construction-effects"/);
  f.session.learn({type: 'select', id: ids[2]});
  html = Guidance.render(f.session.state());
  assert.match(html, /data-guidance-complete/);
  f.session.learn({type: 'complete', id: ids[2]});
  html = Guidance.render(f.session.state());
  assert.match(html, /9 completed · 0 skipped · 9 lessons/);
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
  assert.deepEqual(f.navigation, []); assert.equal(f.pending.length, 0);
  const button = box.querySelector('[data-guidance-open-lesson]');
  box.dispatch('click', {target: button});
  assert.deepEqual(f.navigation, [{kind: 'home'}]);
  assert.equal(box.open, false);
  assert.deepEqual(ui.session.state().progress.done, []);
  assert.equal(ui.session.state().progress.started, true);
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
