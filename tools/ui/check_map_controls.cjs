// Run: node --test tools/ui/check_map_controls.cjs from the repository.
// Executes the actual map-controls module; the fixture only supplies DOM/camera boundaries.
const assert = require('node:assert/strict');
const fs = require('node:fs');
const path = require('node:path');
const vm = require('node:vm');
const {test} = require('node:test');
const modulePath = path.join(__dirname, '../../spheres-web/ui/map-controls.js');
const source = fs.readFileSync(modulePath, 'utf8');

function makeDocument() {
  const document = {activeElement: null, readyState: 'complete', listeners: new Map()};
  const decode = text => String(text).replace(/&quot;/g, '"').replace(/&#39;|&apos;/g, "'")
    .replace(/&lt;/g, '<').replace(/&gt;/g, '>').replace(/&amp;/g, '&');
  function simpleMatch(node, selector) {
    selector = selector.trim();
    if (!selector || !node.tagName) return false;
    let reject = false;
    selector = selector.replace(/:not\(([^)]+)\)/g, (_, inner) => {if (simpleMatch(node, inner)) reject = true; return '';});
    if (reject) return false;
    if (/:disabled/.test(selector) && !node.disabled) return false;
    if (/:enabled/.test(selector) && node.disabled) return false;
    selector = selector.replace(/:(?:disabled|enabled)/g, '');
    const attrs = [...selector.matchAll(/\[([^\]=\s]+)(?:\s*=\s*["']?([^\]"']*)["']?)?\]/g)];
    for (const [, key, value] of attrs) {
      if (!node.hasAttribute(key) || (value !== undefined && node.getAttribute(key) !== value.trim())) return false;
    }
    selector = selector.replace(/\[[^\]]*\]/g, '');
    const id = selector.match(/#([\w-]+)/)?.[1];
    if (id && node.id !== id) return false;
    if ([...selector.matchAll(/\.([\w-]+)/g)].some(([, name]) => !node.classList.contains(name))) return false;
    const tag = selector.match(/^[\w-]+/)?.[0];
    return !tag || tag.toUpperCase() === node.tagName;
  }
  function matches(node, selector) {
    return selector.split(',').some(part => {
      const parts = part.trim().split(/\s+(?![^\[]*\])/);
      if (!simpleMatch(node, parts.pop())) return false;
      let parent = node.parentElement;
      while (parts.length) {
        const direct = parts.at(-1) === '>';
        if (direct) parts.pop();
        const next = parts.pop();
        if (direct) {if (!parent || !simpleMatch(parent, next)) return false; parent = parent.parentElement; continue;}
        while (parent && !simpleMatch(parent, next)) parent = parent.parentElement;
        if (!parent) return false;
        parent = parent.parentElement;
      }
      return true;
    });
  }
  class Element {
    constructor(tagName) {
      this.tagName = tagName.toUpperCase(); this.attributes = {}; this.dataset = {};
      this.children = []; this.parentElement = null; this.disabled = false; this.hidden = false;
      this.listeners = new Map(); this.style = {}; this._text = ''; this.open = false;
      this.classList = {
        contains: name => (this.attributes.class || '').split(/\s+/).includes(name),
        toggle: (name, force) => {const classes = new Set((this.attributes.class || '').split(/\s+/).filter(Boolean));
          const on = force === undefined ? !classes.has(name) : !!force;
          if (on) classes.add(name); else classes.delete(name); this.attributes.class = [...classes].join(' '); return on;},
        add: (...names) => names.forEach(name => this.classList.toggle(name, true)),
        remove: (...names) => names.forEach(name => this.classList.toggle(name, false)),
      };
    }
    get id() {return this.attributes.id || '';}
    set id(value) {this.setAttribute('id', value);}
    get className() {return this.attributes.class || '';}
    set className(value) {this.setAttribute('class', value);}
    get textContent() {return this._text + this.children.map(child => child.textContent).join('');}
    set textContent(value) {this.children = []; this._text = String(value);}
    get isConnected() {return this === document.body || !!this.parentElement?.isConnected;}
    get title() {return this.getAttribute('title') || '';}
    set title(value) {this.setAttribute('title', value);}
    setAttribute(key, value) {
      this.attributes[key] = String(value);
      if (key.startsWith('data-')) this.dataset[key.slice(5).replace(/-([a-z])/g, (_, ch) => ch.toUpperCase())] = String(value);
      if (key === 'disabled') this.disabled = true;
      if (key === 'hidden') this.hidden = true;
      if (key === 'open') this.open = true;
    }
    getAttribute(key) {return this.attributes[key] ?? null;}
    hasAttribute(key) {return Object.hasOwn(this.attributes, key);}
    removeAttribute(key) {delete this.attributes[key]; if (key === 'disabled') this.disabled = false; if (key === 'hidden') this.hidden = false; if (key === 'open') this.open = false;}
    appendChild(child) {child.parentElement?.removeChild(child); this.children.push(child); child.parentElement = this; return child;}
    removeChild(child) {this.children = this.children.filter(item => item !== child); child.parentElement = null; if (child.contains(document.activeElement)) document.activeElement = document.body; return child;}
    contains(node) {return node === this || this.children.some(child => child.contains(node));}
    matches(selector) {return matches(this, selector);}
    closest(selector) {for (let node = this; node; node = node.parentElement) if (node.matches(selector)) return node; return null;}
    querySelectorAll(selector) {const found = []; for (const child of this.children) {if (child.matches(selector)) found.push(child); found.push(...child.querySelectorAll(selector));} return found;}
    querySelector(selector) {return this.querySelectorAll(selector)[0] || null;}
    getClientRects() {return this.isConnected && !this.hidden ? [{}] : [];}
    focus(options) {if (!this.disabled && this.isConnected) {document.activeElement = this; this.focusOptions = options;}}
    addEventListener(type, fn) {if (!this.listeners.has(type)) this.listeners.set(type, new Set()); this.listeners.get(type).add(fn);}
    removeEventListener(type, fn) {this.listeners.get(type)?.delete(fn);}
    dispatchEvent(event) {
      event.target ||= this; event.currentTarget = this;
      event.preventDefault ||= function() {this.defaultPrevented = true;};
      event.stopPropagation ||= function() {this.stopped = true;};
      this['on' + event.type]?.(event);
      for (const fn of this.listeners.get(event.type) || []) fn(event);
      if (!event.stopped && event.bubbles !== false) {
        if (this.parentElement) this.parentElement.dispatchEvent(event);
        else document.dispatchEvent(event);
      }
      return !event.defaultPrevented;
    }
    click() {if (!this.disabled) this.dispatchEvent({type: 'click', target: this, bubbles: true});}
    set innerHTML(html) {
      for (const child of [...this.children]) this.removeChild(child);
      this._text = ''; this._html = String(html); const stack = [this];
      for (const token of String(html).match(/<!--[\s\S]*?-->|<[^>]+>|[^<]+/g) || []) {
        if (token.startsWith('<!--')) continue;
        if (token.startsWith('</')) {if (stack.length > 1) stack.pop(); continue;}
        if (!token.startsWith('<')) {stack.at(-1)._text += decode(token); continue;}
        const tag = token.match(/^<([\w-]+)/)?.[1]; if (!tag) continue;
        const node = new Element(tag);
        const attributes = token.slice(tag.length + 1, -1);
        for (const [, key, quoted, single, plain] of attributes.matchAll(/([\w:-]+)(?:\s*=\s*(?:"([^"]*)"|'([^']*)'|([^\s>]+)))?/g)) node.setAttribute(key, decode(quoted ?? single ?? plain ?? ''));
        stack.at(-1).appendChild(node);
        if (!/\/$/.test(attributes.trim()) && !['INPUT', 'BR', 'HR', 'IMG', 'META', 'LINK'].includes(node.tagName)) stack.push(node);
      }
    }
    get innerHTML() {return this._html || '';}
  }
  document.createElement = tag => new Element(tag);
  document.body = new Element('body'); document.activeElement = document.body;
  document.querySelectorAll = selector => document.body.querySelectorAll(selector);
  document.querySelector = selector => document.querySelectorAll(selector)[0] || null;
  document.getElementById = id => document.querySelector('#' + id);
  document.addEventListener = (type, fn) => {if (!document.listeners.has(type)) document.listeners.set(type, new Set()); document.listeners.get(type).add(fn);};
  document.removeEventListener = (type, fn) => document.listeners.get(type)?.delete(fn);
  document.dispatchEvent = event => {for (const fn of document.listeners.get(event.type) || []) fn(event);};
  return document;
}

function fixture() {
  const document = makeDocument(), calls = [];
  const simulation = {day: 12, orders: [{kind: 'budget', amount: 5}], running: true};
  const originalOrders = JSON.stringify(simulation.orders);
  const forbidden = name => (...args) => {throw new Error('View control invoked ' + name + ': ' + JSON.stringify(args));};
  const c = vm.createContext({document, console, calls,
    ui: {tab: 'map', mapMode: 'political', mapDetails: {borders: true, provinces: true, cities: true, labels: true}, cam: {k: 2}},
    MAP_MODES: {political: {label: 'Political'}, terrain: {label: 'Terrain'}, resources: {label: 'Resources'}, fronts: {label: 'Fronts'}},
    POL: {dirty: false}, SEL: {dirty: false}, S: {player: 'USA', day: 12}, queued: simulation.orders,
    clock: {running: true}, advancing: true, pendingAdvance: {day: 12},
    COMMAND_CHANNEL: {busy: true, pending: {}}, SESSION: {busy: true},
    Globe3D: {ZOOM_MIN: 1, ZOOM_MAX: 32},
    GLOBE: {options: {showCities: true}, render: () => calls.push(['globeRender'])},
    showTab: tab => {c.ui.tab = tab; calls.push(['showTab', tab]);},
    renderMap: () => {calls.push(['renderMap']);},
    mapZoom: factor => {c.ui.cam.k = Math.min(32, Math.max(1, c.ui.cam.k * factor)); calls.push(['mapZoom', factor]);},
    resetCam: () => {c.ui.cam.k = 1; calls.push(['resetCam']);},
    homeNation: () => calls.push(['homeNation']),
    globeNudge: (h, v) => calls.push(['globeNudge', h, v]),
    api: forbidden('api'), fetch: forbidden('fetch'), advance: forbidden('advance'), stepDay: forbidden('stepDay'),
    setSpeed: forbidden('setSpeed'), clockPlay: forbidden('clockPlay'), clockPause: forbidden('clockPause'),
    doSave: forbidden('doSave'), queueOrder: forbidden('queueOrder'),
    requestAnimationFrame: fn => {fn(); return 1;},
    $: selector => document.querySelector(selector),
  });
  c.window = c;
  vm.runInContext(source, c, {filename: modulePath, timeout: 1000});
  const app = document.createElement('div'); app.id = 'app'; document.body.appendChild(app);
  app.innerHTML = '<details class="arc-map-tools"><summary>Map layers</summary></details><button id="outsideMapControls">Outside map controls</button>';
  const root = document.createElement('div'); root.id = 'pane-map'; app.appendChild(root);
  const mount = () => {root.innerHTML = c.MapControls.html(); c.MapControls.bind(); c.MapControls.sync();};
  c.renderMap = () => {calls.push(['renderMap']); mount();};
  mount();
  return {c, document, root, calls, mount, simulation,
    one: selector => {const found = document.querySelector(selector); assert(found, 'Missing actual rendered control ' + selector); return found;},
    assertViewOnly() {assert.equal(c.S.day, 12); assert.equal(c.clock.running, true); assert.equal(JSON.stringify(c.queued), originalOrders);},
  };
}

test('switching from History to Terrain changes only the view even while a day is in flight', () => {
  const f = fixture(); f.c.ui.tab = 'charts'; const camera = JSON.stringify(f.c.ui.cam);
  f.c.MapControls.setMode('terrain');
  assert.equal(f.c.ui.mapMode, 'terrain'); assert.equal(f.c.ui.tab, 'map');
  assert.equal(f.c.POL.dirty, true); assert.equal(f.c.SEL.dirty, true);
  assert(f.calls.some(call => call[0] === 'renderMap'), 'the chosen shading must be rendered');
  assert.equal(JSON.stringify(f.c.ui.cam), camera, 'changing shading preserves camera position');
  assert.deepEqual(Object.keys(f.c.ui.mapDetails).filter(key => !f.c.ui.mapDetails[key]), []);
  f.assertViewOnly();
});

test('unknown shading cannot replace the active mode or repaint the map', () => {
  const f = fixture(); f.calls.length = 0;
  f.c.MapControls.setMode('not-a-map-mode');
  assert.equal(f.c.ui.mapMode, 'political'); assert.equal(f.calls.length, 0); f.assertViewOnly();
});

test('each detail toggle changes exactly its own flag and repaints without simulation commands', () => {
  for (const key of ['borders', 'provinces', 'cities', 'labels']) {
    const f = fixture(); f.calls.length = 0;
    f.c.MapControls.toggleDetail(key);
    assert.equal(f.c.ui.mapDetails[key], false, key + ' switches off');
    for (const other of ['borders', 'provinces', 'cities', 'labels'].filter(name => name !== key)) assert.equal(f.c.ui.mapDetails[other], true, other + ' remains on');
    assert.equal(f.c.POL.dirty, true, key + ' invalidates political paint');
    assert.equal(f.c.SEL.dirty, true, key + ' invalidates selection paint');
    assert(f.calls.some(call => call[0] === 'globeRender' || call[0] === 'renderMap'), key + ' repaints immediately');
    if (key === 'cities') assert.equal(f.c.GLOBE.options.showCities, false, 'the globe renderer receives the city flag');
    f.c.MapControls.toggleDetail(key);
    assert.equal(f.c.ui.mapDetails[key], true, key + ' can be restored');
    if (key === 'cities') assert.equal(f.c.GLOBE.options.showCities, true);
    f.assertViewOnly();
  }
});

test('unknown detail keys cannot introduce state or repaint the map', () => {
  const f = fixture(); const original = JSON.stringify(f.c.ui.mapDetails); f.calls.length = 0;
  f.c.MapControls.toggleDetail('not-a-map-detail');
  assert.equal(JSON.stringify(f.c.ui.mapDetails), original); assert.equal(f.calls.length, 0); f.assertViewOnly();
});

test('sync exposes current shading and every toggle state through native controls', () => {
  const f = fixture();
  for (const mode of ['terrain', 'political', 'fronts']) {
    const control = f.one('[data-map-mode="' + mode + '"]');
    assert.equal(control.tagName, 'BUTTON'); assert.equal(control.getAttribute('type'), 'button');
    assert.equal(control.getAttribute('aria-pressed'), String(mode === 'political'));
  }
  f.c.ui.mapMode = 'resources'; f.c.ui.cam.k = 6;
  f.c.ui.mapDetails.labels = false; f.c.MapControls.sync();
  assert.equal(f.one('#mapViewStatus').textContent, 'Resources · 6.0×');
  assert.equal(f.one('.arc-map-tools > summary').textContent, 'More layers · Resources');
  for (const control of f.root.querySelectorAll('[data-map-mode]')) assert.equal(control.getAttribute('aria-pressed'), 'false');
  assert.equal(f.one('[data-map-detail="labels"]').getAttribute('aria-pressed'), 'false');
  assert.equal(f.one('[data-map-detail="labels"] .map-detail-state').textContent, 'Off');
  assert.equal(f.one('[data-map-detail="borders"]').getAttribute('aria-pressed'), 'true');
  assert.equal(f.one('[data-map-detail="borders"] .map-detail-state').textContent, 'On');
  f.assertViewOnly();
});

test('zoom buttons reflect both camera limits and immediately refresh after a click', () => {
  const f = fixture(); const minus = f.one('[data-map-action="zoom-out"]'), plus = f.one('[data-map-action="zoom-in"]');
  f.c.ui.cam.k = 1; f.c.MapControls.sync();
  assert.equal(minus.disabled, true); assert.equal(plus.disabled, false); f.calls.length = 0;
  minus.click(); assert.equal(f.calls.length, 0, 'minimum zoom does not dispatch a camera action');
  plus.click(); assert.equal(f.c.ui.cam.k, 1.35); assert.equal(minus.disabled, false);
  assert.equal(f.one('#mapViewStatus').textContent, 'Political · 1.4×');
  f.c.ui.cam.k = 32; f.c.MapControls.sync();
  assert.equal(plus.disabled, true); assert.equal(minus.disabled, false); f.calls.length = 0;
  plus.click(); assert.equal(f.calls.length, 0, 'maximum zoom does not dispatch a camera action');
  minus.click(); assert(f.c.ui.cam.k < 32); assert.equal(plus.disabled, false);
  assert.equal(f.calls.filter(call => call[0] === 'mapZoom').length, 1); f.assertViewOnly();
});

test('pending days and busy campaign transport leave every view control available', () => {
  const f = fixture();
  assert.equal(f.c.clock.running, true); assert.equal(f.c.advancing, true); assert(f.c.pendingAdvance);
  assert.equal(f.c.COMMAND_CHANNEL.busy, true); assert(f.c.COMMAND_CHANNEL.pending); assert.equal(f.c.SESSION.busy, true);
  f.c.ui.cam.k = 4; f.c.MapControls.bind(); f.c.MapControls.sync();
  for (const button of f.root.querySelectorAll('button')) assert.equal(button.disabled, false, button.dataset.mapFocus + ' stays available during transport activity');
  f.c.ui.cam.k = 1; f.c.MapControls.sync();
  for (const button of f.root.querySelectorAll('button')) assert.equal(button.disabled, button.dataset.mapAction === 'zoom-out', button.dataset.mapFocus + ' is disabled only at its camera boundary');
  f.assertViewOnly();
});

test('World, Home and turn buttons use only their existing camera handlers', () => {
  const f = fixture(); f.calls.length = 0;
  for (const action of ['world', 'home', 'west', 'east']) f.one('[data-map-action="' + action + '"]').click();
  assert.deepEqual(f.calls, [['resetCam'], ['homeNation'], ['globeNudge', -1, 0], ['globeNudge', 1, 0]]);
  assert.equal(f.one('[data-map-action="zoom-out"]').disabled, true, 'World resets the zoom boundary readout');
  f.assertViewOnly();
});

test('repeated installation binds one action and restores focus to the rebuilt mode button', () => {
  const f = fixture(); const old = f.one('[data-map-mode="terrain"]');
  old.focus(); f.c.ui.tab = 'charts';
  for (let i = 0; i < 4; i++) {f.c.MapControls.bind(); f.c.MapControls.install();}
  f.calls.length = 0; old.click();
  assert.equal(f.calls.filter(call => call[0] === 'renderMap').length, 1, 'a single click performs one render');
  assert.equal(f.calls.filter(call => call[0] === 'showTab').length, 1, 'a single click performs one pane switch');
  assert.equal(f.c.ui.tab, 'map'); assert.equal(f.c.ui.mapMode, 'terrain');
  const replacement = f.one('[data-map-mode="terrain"]');
  assert.notEqual(replacement, old); assert.equal(old.isConnected, false);
  assert.equal(f.document.activeElement, replacement); assert.equal(replacement.getAttribute('aria-pressed'), 'true');
  assert.equal(replacement.focusOptions.preventScroll, true); f.assertViewOnly();
});

test('detail selection survives redraw and Escape returns focus without reaching game shortcuts', () => {
  const f = fixture(); const originalMenu = f.one('.map-detail-menu');
  originalMenu.open = true; originalMenu.dispatchEvent({type: 'toggle', bubbles: false});
  const originalButton = f.one('[data-map-detail="borders"]'); originalButton.focus(); originalButton.click();
  const menu = f.one('.map-detail-menu'), button = f.one('[data-map-detail="borders"]');
  assert.notEqual(menu, originalMenu); assert.equal(menu.open, true, 'the open disclosure survives renderMap');
  assert.equal(f.document.activeElement, button); assert.equal(button.getAttribute('aria-pressed'), 'false');
  assert.equal(button.querySelector('.map-detail-state').textContent, 'Off');
  let globalKeys = 0; f.document.addEventListener('keydown', () => {globalKeys++;});
  const event = {type: 'keydown', key: 'Escape', bubbles: true}; button.dispatchEvent(event);
  assert.equal(menu.open, false); assert.equal(event.defaultPrevented, true); assert.equal(globalKeys, 0);
  assert.equal(f.document.activeElement, menu.querySelector('summary')); f.assertViewOnly();
});

test('ordinary redraw restores map focus but never steals focus from another screen control', () => {
  const f = fixture(); const old = f.one('[data-map-action="home"]'); old.focus();
  f.mount(); assert.equal(old.isConnected, false); assert.equal(f.document.activeElement, f.one('[data-map-action="home"]'));
  const outside = f.one('#outsideMapControls'); outside.focus();
  f.mount(); assert.equal(f.document.activeElement, outside);
  f.c.MapControls.sync(); assert.equal(f.document.activeElement, outside); f.assertViewOnly();
});

test('missing legacy detail preferences default on while explicit false remains off', () => {
  const f = fixture(); f.c.ui.mapDetails = {borders: false}; f.mount();
  assert.equal(f.c.ui.mapDetails.borders, false);
  for (const key of ['provinces', 'cities', 'labels']) assert.equal(f.c.ui.mapDetails[key], true);
  assert.equal(f.one('[data-map-detail="borders"]').getAttribute('aria-pressed'), 'false');
  assert.equal(f.one('[data-map-detail="cities"]').getAttribute('aria-pressed'), 'true'); f.assertViewOnly();
});

test('unmounted controls allow harmless sync and install calls', () => {
  const f = fixture(); const controls = f.one('#mapControls'); controls.parentElement.removeChild(controls); f.calls.length = 0;
  assert.doesNotThrow(() => f.c.MapControls.sync()); assert.equal(f.c.MapControls.install(), false);
  assert.equal(f.calls.length, 0); f.assertViewOnly();
});
