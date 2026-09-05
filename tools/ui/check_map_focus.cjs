// Run: node --test tools/ui/check_map_focus.cjs
// Executes the shipped helpers; fake DOM nodes model replacement and focus only.
const assert = require('node:assert/strict');
const fs = require('node:fs');
const path = require('node:path');
const vm = require('node:vm');
const {test} = require('node:test');
const page = fs.readFileSync(path.resolve(__dirname, '../../spheres-web/ui/index.html'), 'utf8');
const helperNames = ['rememberMapControlFocus', 'restoreMapControlFocus', 'closeNavigationDisclosure', 'resourceCommodityChips'];
function source(name) {
  const found = new RegExp('^function ' + name + '\\(', 'm').exec(page);
  assert(found, 'Missing shipped helper ' + name);
  const first = page.slice(found.index, page.indexOf('\n', found.index));
  if (/\}\s*$/.test(first)) return first;
  const end = page.indexOf('\n}', found.index);
  assert(end > found.index, 'Missing closing brace for ' + name);
  return page.slice(found.index, end + 2);
}

function fixture() {
  const document = {activeElement: null};
  function simple(node, selector) {
    const attrs = [...selector.matchAll(/\[([^\]=]+)(?:=["']?([^\]"']+)["']?)?\]/g)];
    for (const [, name, value] of attrs) if (!node.hasAttribute(name) || (value !== undefined && node.getAttribute(name) !== value)) return false;
    selector = selector.replace(/\[[^\]]*\]/g, '');
    const id = selector.match(/#([\w-]+)/)?.[1]; if (id && node.id !== id) return false;
    for (const [, name] of selector.matchAll(/\.([\w-]+)/g)) if (!node.className.split(/\s+/).includes(name)) return false;
    const tag = selector.match(/^[\w-]+/)?.[0]; return !tag || tag.toUpperCase() === node.tagName;
  }
  function matches(node, selector) {
    return selector.split(',').some(part => {
      const parts = part.trim().split(/\s+(?![^\[]*\])/); if (!simple(node, parts.pop())) return false;
      let parent = node.parentElement;
      while (parts.length) {
        const direct = parts.at(-1) === '>'; if (direct) parts.pop(); const next = parts.pop();
        if (direct) {if (!parent || !simple(parent, next)) return false; parent = parent.parentElement; continue;}
        while (parent && !simple(parent, next)) parent = parent.parentElement;
        if (!parent) return false; parent = parent.parentElement;
      }
      return true;
    });
  }
  function node(tag, options = {}, parent) {
    const n = {tagName: tag.toUpperCase(), id: '', className: '', dataset: {}, attributes: {}, children: [],
      parentElement: null, disabled: false, open: false, hidden: false, ...options,
      get isConnected() {return this === document.body || !!this.parentElement?.isConnected;},
      getAttribute(key) {if (key === 'id') return this.id || null; if (key === 'class') return this.className || null;
        if (key === 'open') return this.open ? '' : null; if (key === 'disabled') return this.disabled ? '' : null;
        if (key.startsWith('data-')) return this.dataset[key.slice(5).replace(/-([a-z])/g, (_, ch) => ch.toUpperCase())] ?? null;
        return this.attributes[key] ?? null;},
      hasAttribute(key) {return this.getAttribute(key) !== null;},
      contains(other) {return this === other || this.children.some(child => child.contains(other));},
      closest(selector) {for (let item = this; item; item = item.parentElement) if (matches(item, selector)) return item; return null;},
      querySelectorAll(selector) {return this.children.flatMap(child => [...(matches(child, selector) ? [child] : []), ...child.querySelectorAll(selector)]);},
      querySelector(selector) {return this.querySelectorAll(selector)[0] || null;},
      getClientRects() {if (!this.isConnected || this.hidden) return []; for (let child = this; child.parentElement; child = child.parentElement) {
        const parent = child.parentElement; if (parent.hidden || (parent.tagName === 'DETAILS' && !parent.open && child.tagName !== 'SUMMARY')) return [];
      } return [{}];},
      focus(options) {if (this.isConnected && !this.disabled) {document.activeElement = this; this.focusOptions = options;}},
      remove() {if (!this.parentElement) return; this.parentElement.children = this.parentElement.children.filter(child => child !== this);
        this.parentElement = null; if (this.contains(document.activeElement)) document.activeElement = document.body;},
    };
    if (parent) {parent.children.push(n); n.parentElement = parent;} return n;
  }
  document.body = node('body'); document.activeElement = document.body;
  document.querySelectorAll = selector => document.body.querySelectorAll(selector);
  document.querySelector = selector => document.querySelectorAll(selector)[0] || null;
  document.getElementById = id => document.querySelector('#' + id);
  const app = node('main', {id: 'app'}, document.body);
  const timeMenu = node('details', {className: 'arc-time-menu'}, app);
  const timeSummary = node('summary', {}, timeMenu); const save = node('button', {}, timeMenu);
  const layers = node('details', {className: 'arc-map-tools', open: true}, app);
  const layersSummary = node('summary', {}, layers);
  const mapModes = node('div', {id: 'mapModes'}, layers);
  const mode = node('button', {dataset: {mode: 'terrain'}}, mapModes);
  const overlay = node('button', {dataset: {resovl: '1'}}, mapModes);
  const resChips = node('div', {id: 'resChips'}, app);
  const commodity = node('button', {dataset: {com: 'coal'}}, resChips);
  const detailMenu = node('details', {className: 'map-detail-menu'}, app);
  const detailSummary = node('summary', {}, detailMenu); const detail = node('button', {}, detailMenu);
  const outside = node('button', {id: 'roomAction'}, app);
  const c = vm.createContext({document,
    RES_ALL: '__all', RESOURCES: {commodities: {oil: {label: 'Oil & gas'}, coal: {label: 'Coal'}}},
    RCOLOR: {oil: '#aaa', coal: '#bbb'}, rglyph: () => '<svg aria-hidden="true"></svg>', resHue: () => '#abc',
    CSS: {escape: value => String(value).replace(/["\\]/g, char => '\\' + char)},
  });
  for (const name of helperNames) vm.runInContext(source(name), c);
  const event = (target, key = 'Escape') => ({key, target, prevented: false, stopped: false,
    preventDefault() {this.prevented = true;}, stopPropagation() {this.stopped = true;}});
  return {c, document, node, app, mapModes, mode, overlay, resChips, commodity, layers, layersSummary,
    timeMenu, timeSummary, save, detailMenu, detailSummary, detail, outside, event};
}

test('header modes, resource overlay and commodity selection return to their replacement button', () => {
  for (const [field, containerId, key, value] of [['mode', 'mapModes', 'mode', 'terrain'], ['overlay', 'mapModes', 'resovl', '1'], ['commodity', 'resChips', 'com', 'coal']]) {
    const f = fixture(); const old = f[field]; old.focus();
    const saved = f.c.rememberMapControlFocus(); assert.deepEqual({containerId: saved.containerId, key: saved.key, value: saved.value}, {containerId, key, value});
    assert.equal(saved.element, old);
    old.remove(); const replacement = f.node('button', {dataset: {[key]: value}}, f[containerId]);
    assert.equal(f.document.activeElement, f.document.body, 'DOM replacement removes active focus');
    assert.equal(f.c.restoreMapControlFocus(saved), true);
    assert.equal(f.document.activeElement, replacement); assert.equal(replacement.focusOptions.preventScroll, true);
  }
});

test('ordinary map refresh leaves focus in an unrelated room control', () => {
  const f = fixture(); f.outside.focus();
  const saved = f.c.rememberMapControlFocus(); assert.equal(saved, null);
  assert.equal(f.c.restoreMapControlFocus(saved), false); assert.equal(f.document.activeElement, f.outside);
});

test('missing or disabled commodities never receive focus', () => {
  const f = fixture(); f.commodity.focus(); const saved = f.c.rememberMapControlFocus(); f.commodity.remove();
  f.outside.focus(); assert.equal(f.c.restoreMapControlFocus(saved), false); assert.equal(f.document.activeElement, f.outside);
  const disabled = f.node('button', {dataset: {com: 'coal'}, disabled: true}, f.resChips);
  f.document.activeElement = f.document.body;
  assert.equal(f.c.restoreMapControlFocus(saved), false); assert.notEqual(f.document.activeElement, disabled);
  f.resChips.remove(); assert.equal(f.c.restoreMapControlFocus(saved), false);
});

test('a removed or disabled header choice falls back to More layers summary', () => {
  const f = fixture(); f.overlay.focus(); const saved = f.c.rememberMapControlFocus(); f.overlay.remove();
  assert.equal(f.c.restoreMapControlFocus(saved), true); assert.equal(f.document.activeElement, f.layersSummary);
  assert.equal(f.layersSummary.focusOptions.preventScroll, true);
  f.node('button', {dataset: {resovl: '1'}, disabled: true}, f.mapModes); f.document.activeElement = f.document.body;
  assert.equal(f.c.restoreMapControlFocus(saved), true); assert.equal(f.document.activeElement, f.layersSummary);
});

test('a closed disclosure cannot receive focus inside its concealed buttons', () => {
  const f = fixture(); f.mode.focus(); const saved = f.c.rememberMapControlFocus(); f.layers.open = false; f.document.activeElement = f.document.body;
  f.c.restoreMapControlFocus(saved);
  assert.notEqual(f.document.activeElement, f.mode);
  assert([f.document.body, f.layersSummary].includes(f.document.activeElement));
});

test('a room that acquires focus during replacement keeps it despite a saved map target', () => {
  const f = fixture(); f.mode.focus(); const saved = f.c.rememberMapControlFocus(); f.mode.remove();
  f.node('button', {dataset: {mode: 'terrain'}}, f.mapModes); f.outside.focus();
  assert.equal(f.c.restoreMapControlFocus(saved), false); assert.equal(f.document.activeElement, f.outside);
});

test('Escape closes the disclosure containing its target before another open menu', () => {
  const f = fixture(); f.timeMenu.open = true; f.detailMenu.open = true; f.detail.focus();
  const event = f.event(f.detail); assert.equal(f.c.closeNavigationDisclosure(event), true);
  assert.equal(f.detailMenu.open, false); assert.equal(f.layers.open, true); assert.equal(f.timeMenu.open, true);
  assert.equal(event.prevented, true); assert.equal(event.stopped, true);
  assert.equal(f.document.activeElement, f.detailSummary); assert.equal(f.detailSummary.focusOptions.preventScroll, true);
});

test('Escape uses active menu focus, then falls back to the first open global menu', () => {
  const f = fixture(); f.timeMenu.open = true; f.detailMenu.open = true; f.overlay.focus();
  assert.equal(f.c.closeNavigationDisclosure(f.event(null)), true);
  assert.equal(f.layers.open, false); assert.equal(f.document.activeElement, f.layersSummary);
  f.outside.focus(); assert.equal(f.c.closeNavigationDisclosure(f.event(f.outside)), true);
  assert.equal(f.timeMenu.open, false); assert.equal(f.document.activeElement, f.timeSummary);
  assert.equal(f.detailMenu.open, true);
});

test('unrelated keys and closed navigation leave focus and event propagation alone', () => {
  const f = fixture(); f.outside.focus(); const enter = f.event(f.overlay, 'Enter');
  assert.equal(f.c.closeNavigationDisclosure(enter), false); assert.equal(f.layers.open, true);
  assert.equal(enter.prevented, false); assert.equal(enter.stopped, false); assert.equal(f.document.activeElement, f.outside);
  f.layers.open = false; const escape = f.event(f.outside);
  assert.equal(f.c.closeNavigationDisclosure(escape), false); assert.equal(escape.prevented, false); assert.equal(escape.stopped, false);
  assert.equal(f.document.activeElement, f.outside);
});

test('resource filters are native buttons with an explicit single pressed choice', () => {
  const f = fixture(); const html = f.c.resourceCommodityChips(['oil', 'coal'], 'coal');
  const buttons = [...html.matchAll(/<button\b([^>]*)>([\s\S]*?)<\/button>/g)];
  assert.equal(buttons.length, 3, 'All and two commodities are rendered');
  for (const [, attributes] of buttons) assert.match(attributes, /\btype="button"/);
  assert.match(buttons[0][1], /\bdata-com="__all"/); assert.match(buttons[0][1], /\baria-pressed="false"/);
  assert.match(buttons[1][1], /\bdata-com="oil"/); assert.match(buttons[1][1], /\baria-pressed="false"/);
  assert.match(buttons[2][1], /\bdata-com="coal"/); assert.match(buttons[2][1], /\baria-pressed="true"/);
  assert.doesNotMatch(html, /<span\b[^>]*\bdata-com=/);
  const all = f.c.resourceCommodityChips(['oil', 'coal'], '__all');
  assert.equal((all.match(/aria-pressed="true"/g) || []).length, 1);
  assert.match(all, /<button\b(?=[^>]*data-com="__all")(?=[^>]*aria-pressed="true")[^>]*>/);
});

test('resource labels and commodity identifiers cannot escape their HTML text or attributes', () => {
  const f = fixture(); const key = 'ore"><img src=x onerror=alert(1)>';
  f.c.RESOURCES.commodities[key] = {label: 'Ore <script>bad()</script> & "metal"'};
  const html = f.c.resourceCommodityChips([key], key);
  assert.doesNotMatch(html, /<img|<script|onerror="|data-com="ore">/);
  assert.match(html, /&lt;/); assert.match(html, /&gt;/); assert.match(html, /&amp;/); assert.match(html, /&quot;/);
  assert.equal((html.match(/<button\b/g) || []).length, 2);
});

test('renderMap preserves auxiliary focus around DOM replacement and resPanel uses native chips', () => {
  const render = source('renderMap');
  const capture = render.indexOf('rememberMapControlFocus('), replace = render.indexOf('$("#pane-map").innerHTML = s'), restore = render.indexOf('restoreMapControlFocus(');
  assert(capture >= 0 && capture < replace, 'header/filter focus is captured before the pane is rebuilt');
  assert(restore > replace, 'focus returns only after replacements exist');
  assert.match(source('resPanel'), /resourceCommodityChips\(/);
  assert.doesNotMatch(source('resPanel'), /<span\b[^>]*\bdata-com=/);
});

test('the global Escape path handles navigation before closing a province and respects active rooms', () => {
  const start = page.indexOf('document.addEventListener("keydown", (e) => {');
  const end = page.indexOf('$("#keysBtn").onclick', start); const keys = page.slice(start, end);
  const nav = keys.indexOf('closeNavigationDisclosure(e)'), province = keys.indexOf('if (selectedDistrict)');
  assert(nav >= 0 && nav < province, 'Escape closes the open menu before the underlying province');
  assert.match(keys, /if\s*\(\s*!room\s*&&\s*closeNavigationDisclosure\(e\)\s*\)\s*return/);
});

test('the shipped map focus, disclosure and commodity helpers exist and parse', () => {
  for (const name of helperNames) assert.doesNotThrow(() => new vm.Script(source(name)));
});
