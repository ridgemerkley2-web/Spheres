/* Spheres cartoon review workbench (CLAUDE-C03-REVIEW-01).
 *
 * Read-only presentation of the export written by tools/avatars/cartoon_review.py.
 * Before anything is shown, every input the export pins is fetched from this
 * repository and its SHA-256 compared; a changed input fails closed. Images load
 * from the repository only; nothing is fetched from another origin. Automated
 * findings are shown separately from visual notes, and nothing here approves art.
 */
(function (root, factory) {
  'use strict';
  const api = factory();
  if (typeof module === 'object' && module.exports) module.exports = api;
  else {
    root.CartoonReview = api;
    if (root.document) root.document.addEventListener('DOMContentLoaded', () => api.boot(root.document, root));
  }
})(typeof self !== 'undefined' ? self : this, function () {
  'use strict';

  const EXPORT_PATH = 'docs/campaign-certification/C03/preparation/cartoon-review/cartoon-review.json';
  const EXPORT_FORMAT = 'spheres-c03-cartoon-review/v1';
  const ALLOWED_PREFIXES = ['spheres-web/ui/', 'spheres-web/data/', 'spheres-sim/data/', 'docs/campaign-certification/', 'tools/avatars/'];
  const OPEN_END = '9999-12-31';
  const ERAS = [
    { id: 'all', label: 'All eras' },
    { id: '1990s', label: '1990–1999', from: '1990-01-01', to: '2000-01-01' },
    { id: '2000s', label: '2000–2009', from: '2000-01-01', to: '2010-01-01' },
    { id: '2010s', label: '2010–2019', from: '2010-01-01', to: '2020-01-01' },
    { id: '2020s', label: '2020 – 7 Sep 2026 (historical)', from: '2020-01-01', to: '2026-09-08' },
    { id: 'fictional', label: '8 Sep 2026 – 2035 (fictional period)', from: '2026-09-08', to: '2036-01-01' },
    { id: 'undated', label: 'No dated appearance' },
  ];
  const COLLECTION_OPTIONS = [
    ['all', 'Everything'], ['cartoons', 'Existing cartoons'], ['historical', 'Historical people'],
    ['fictional', 'Fictional (not real people)'], ['selector', 'Country-selector figures'],
    ['missing', 'Artwork missing'], ['unregistered', 'Unregistered files'],
  ];
  const LABELS = {
    historical: 'Historical person', fictional: 'Fictional · not a real person', selector: 'Country selector',
    unregistered: 'Unregistered file', 'missing-art': 'Artwork missing', 'style-reference': 'Style reference',
    'file-missing': 'File missing', 'unknown-identity': 'Unknown identity', 'country-unknown': 'Country unknown',
    duplicate: 'Duplicate image', 'interval-issue': 'Interval issue', 'coverage-gap': 'Coverage gap',
    'rights-gap': 'Rights gap', 'sample-reviewed': 'Visual notes · not approval', 'integrity-error': 'Integrity error',
  };
  // Sizes taken from the game's CSS: government-ui.css person cards and index.html selector cards.
  const CARD_SIZES = [
    { id: 'card', label: 'Government card', width: 106, height: 152 },
    { id: 'narrow', label: 'Narrow card', width: 90, height: 132 },
    { id: 'featured', label: 'Featured card', width: 156, height: 218 },
  ];
  const SELECTOR_SIZES = [
    { id: 'pick', label: 'Selector pick card', width: 143, height: 174 },
    { id: 'compact', label: 'Compact showcase', width: 116, height: 145 },
    { id: 'showcase', label: 'Selector showcase', width: 174, height: 218 },
  ];

  // ------------------------------------------------------------ pure helpers

  function fold(text) {
    return String(text || '').normalize('NFD').replace(/\p{M}/gu, '').toLocaleLowerCase();
  }

  function labelText(label) {
    return LABELS[label] || label;
  }

  function itemIntervals(item) {
    if (item.interval && item.interval.valid) return [{ from: item.interval.from, to: item.interval.to || OPEN_END }];
    if (item.collection === 'missing') return (item.required_windows || []).filter((w) => w.from && w.to).map((w) => ({ from: w.from, to: w.to }));
    return [];
  }

  function overlaps(a, b) {
    return a.from < b.to && b.from < a.to;
  }

  function contains(interval, day) {
    return interval.from <= day && day < interval.to;
  }

  function searchText(item, nations) {
    const countries = item.countries || [];
    return fold([item.name, item.id, item.identity && item.identity.person_id, item.identity && item.identity.wikidata,
      ...countries, ...countries.map((c) => (nations || {})[c] || ''), ...(item.labels || []).map(labelText)].filter(Boolean).join(' '));
  }

  function matchesFilters(item, filters, nations) {
    const f = filters || {};
    if (f.collection && f.collection !== 'all') {
      if (f.collection === 'cartoons') {
        if (!['historical', 'fictional', 'selector'].includes(item.collection)) return false;
      } else if (item.collection !== f.collection) return false;
    }
    const countries = item.countries || [];
    if (f.country && f.country !== 'all') {
      if (f.country === 'unknown' ? countries.length : !countries.includes(f.country)) return false;
    }
    if (f.label && f.label !== 'all' && !(item.labels || []).includes(f.label)) return false;
    const intervals = itemIntervals(item);
    if (f.era && f.era !== 'all') {
      if (f.era === 'undated') {
        if (intervals.length) return false;
      } else {
        const era = ERAS.find((e) => e.id === f.era);
        if (!era) throw new Error('Unknown era filter: ' + f.era);
        if (!intervals.some((i) => overlaps(i, era))) return false;
      }
    }
    if (f.date) {
      if (!/^\d{4}-\d{2}-\d{2}$/.test(f.date)) throw new Error('Dates use YYYY-MM-DD');
      if (!intervals.some((i) => contains(i, f.date))) return false;
    }
    const terms = fold(f.q).split(/\s+/).filter(Boolean);
    if (terms.length) {
      const hay = searchText(item, nations);
      if (!terms.every((t) => hay.includes(t))) return false;
    }
    return true;
  }

  function filterItems(items, filters, nations) {
    return items.filter((item) => matchesFilters(item, filters, nations));
  }

  function moveIndex(index, key, columns, count) {
    if (!count) return -1;
    const cols = Math.max(1, Math.floor(columns) || 1);
    const steps = { ArrowRight: 1, ArrowLeft: -1, ArrowDown: cols, ArrowUp: -cols, PageDown: cols * 3, PageUp: -cols * 3 };
    let next;
    if (key === 'Home') next = 0;
    else if (key === 'End') next = count - 1;
    else if (key in steps) next = index + steps[key];
    else return index;
    return Math.min(count - 1, Math.max(0, next));
  }

  function columnsFromTops(tops) {
    if (!tops.length) return 1;
    let n = 0;
    for (const top of tops) {
      if (Math.abs(top - tops[0]) < 2) n += 1;
      else break;
    }
    return Math.max(1, n);
  }

  function safePath(value) {
    if (typeof value !== 'string' || !value || /[\\:%?#]/.test(value) || value.startsWith('/')) return null;
    if (value.split('/').some((part) => part === '' || part === '.' || part === '..')) return null;
    return ALLOWED_PREFIXES.some((prefix) => value.startsWith(prefix)) ? value : null;
  }

  function publicURL(value) {
    try {
      const url = new URL(value);
      return url.protocol === 'https:' && !url.username && !url.password ? url.href : null;
    } catch (error) {
      return null;
    }
  }

  function resolvePointer(document, pointer) {
    if (typeof pointer !== 'string' || !pointer.startsWith('/')) return undefined;
    let node = document;
    for (const raw of pointer.slice(1).split('/')) {
      const key = raw.replace(/~1/g, '/').replace(/~0/g, '~');
      if (node === null || typeof node !== 'object' || !Object.prototype.hasOwnProperty.call(node, key)) return undefined;
      node = node[key];
    }
    return node;
  }

  function sizesFor(item) {
    return item && item.collection === 'selector' ? SELECTOR_SIZES : CARD_SIZES;
  }

  function reasonText(reason) {
    return (Array.isArray(reason) ? reason : []).filter(Boolean).map((part) => String(part).replace(/_/g, ' ')).join(' · ');
  }

  function formatInterval(interval) {
    if (!interval) return 'No dated appearance';
    const to = interval.to && interval.to !== OPEN_END ? `before ${interval.to}` : 'open end';
    return `${interval.from} → ${to}`;
  }

  function initials(name) {
    const words = String(name || '').split(/[\s-]+/).map((w) => w.replace(/[^\p{L}\p{N}]/gu, '')).filter(Boolean);
    if (!words.length) return '?';
    return (words[0][0] + (words.length > 1 ? words[words.length - 1][0] : '')).toUpperCase();
  }

  async function sha256Hex(buffer, cryptoImpl) {
    const digest = await cryptoImpl.subtle.digest('SHA-256', buffer);
    return Array.from(new Uint8Array(digest), (b) => b.toString(16).padStart(2, '0')).join('');
  }

  async function verifyInputs(inputs, fetcher, cryptoImpl, base) {
    if (!cryptoImpl || !cryptoImpl.subtle) throw new Error('Input verification needs a secure context: open this page from http://127.0.0.1 or http://localhost.');
    if (!Array.isArray(inputs) || !inputs.length) throw new Error('The export lists no pinned inputs.');
    const results = [];
    for (const input of inputs) {
      const path = safePath(input.path);
      if (!path) throw new Error('The export names an unsafe input path: ' + input.path);
      const response = await fetcher(base + path, { cache: 'no-store' });
      if (!response.ok) throw new Error(`Input unavailable (${response.status}): ${path}`);
      const buffer = await response.arrayBuffer();
      const sha256 = await sha256Hex(buffer, cryptoImpl);
      results.push({ role: input.role, path, bytes: buffer.byteLength, sha256, matches: sha256 === input.sha256 && buffer.byteLength === input.bytes, buffer });
    }
    return results;
  }

  function latest() {
    let token = 0;
    return () => {
      const mine = ++token;
      return () => mine === token;
    };
  }

  function nationOptions(items, nations) {
    const used = new Set();
    let unknown = 0;
    for (const item of items) {
      if (!(item.countries || []).length) unknown += 1;
      for (const c of item.countries || []) used.add(c);
    }
    const options = [...used].map((id) => [id, (nations || {})[id] || id]).sort((a, b) => a[1].localeCompare(b[1]));
    return unknown ? [...options, ['unknown', 'Country unknown']] : options;
  }

  // ------------------------------------------------------------ page

  function boot(document, window) {
    const $ = (id) => document.getElementById(id);
    const node = (tag, text, className) => {
      const el = document.createElement(tag);
      if (text !== undefined && text !== null) el.textContent = text;
      if (className) el.className = className;
      return el;
    };
    const base = new URL('../../../', window.location.href).href;
    const state = { data: null, items: [], byId: new Map(), cards: new Map(), visible: [], selected: null,
      manifests: new Map(), findings: new Map(), sample: new Map(), files: new Map(), verification: [] };
    const nextFull = latest();
    let fullURL = null;
    const narrow = window.matchMedia('(max-width: 899px)');

    function setStatus(message, alert) {
      const status = $('status');
      status.textContent = message;
      status.setAttribute('role', alert ? 'alert' : 'status');
      status.classList.toggle('alert', Boolean(alert));
    }

    function url(path) {
      const safe = safePath(path);
      return safe ? base + safe.split('/').map(encodeURIComponent).join('/') : null;
    }

    function chip(label) {
      const el = node('span', labelText(label), 'chip chip-' + label);
      el.dataset.label = label;
      return el;
    }

    function readFilters() {
      return { q: $('f-search').value, country: $('f-country').value, era: $('f-era').value,
        date: $('f-date').value, collection: $('f-collection').value, label: $('f-label').value };
    }

    function writeURL() {
      const params = new URLSearchParams();
      const f = readFilters();
      for (const [key, value] of Object.entries(f)) if (value && value !== 'all') params.set(key, value);
      if (state.selected) params.set('item', state.selected);
      const query = params.toString();
      window.history.replaceState(null, '', query ? '?' + query : window.location.pathname);
    }

    function fillSelect(select, options, value) {
      select.replaceChildren(...options.map(([v, label]) => {
        const option = node('option', label);
        option.value = v;
        return option;
      }));
      if (options.some(([v]) => v === value)) select.value = value;
    }

    function thumb(item, size, eager) {
      const box = node('span', undefined, 'thumb thumb-' + item.collection);
      box.style.width = size.width + 'px';
      box.style.height = size.height + 'px';
      const src = item.collection === 'missing' ? null : url(item.card || item.asset);
      const facts = state.files.get(item.card || item.asset);
      if (src && facts && facts.exists) {
        const img = node('img');
        img.alt = altText(item);
        img.decoding = 'async';
        img.loading = eager ? 'eager' : 'lazy';
        img.width = size.width;
        img.height = size.height;
        img.src = src;
        box.append(img);
      } else {
        box.classList.add('placeholder');
        box.append(node('span', initials(item.name), 'initials'));
        box.append(node('span', item.collection === 'missing' ? 'Artwork missing' : 'File missing', 'placeholder-text'));
      }
      return box;
    }

    function altText(item) {
      if (item.collection === 'fictional') return `Fictional cartoon of ${item.name} (not a real person)`;
      if (item.collection === 'selector') return `Country-selector cartoon of ${item.name}`;
      if (item.collection === 'unregistered') return `Unregistered image file ${item.name}`;
      return `Cartoon of ${item.name}`;
    }

    function eraText(item) {
      if (item.interval && item.interval.valid) return formatInterval(item.interval);
      if (item.interval) return 'Invalid interval';
      if (item.collection === 'missing') return `${(item.required_windows || []).length} art window(s) needed`;
      if (item.collection === 'selector') return item.years ? `Figure ${item.years}` : 'No dated appearance';
      return 'No dated appearance';
    }

    function buildCards() {
      const sheet = $('sheet');
      const fragment = document.createDocumentFragment();
      for (const item of state.items) {
        const li = node('li');
        const button = node('button', undefined, 'card card-' + item.collection);
        button.type = 'button';
        button.tabIndex = -1;
        button.dataset.id = item.id;
        button.dataset.countries = (item.countries || []).join(' ');
        button.dataset.collection = item.collection;
        button.setAttribute('aria-pressed', 'false');
        button.append(thumb(item, CARD_SIZES[0], false));
        const text = node('span', undefined, 'card-text');
        text.append(node('span', item.name, 'card-name'));
        const countries = (item.countries || []).map((c) => state.data.nations[c] || c).join(', ') || 'Country unknown';
        text.append(node('span', countries, 'card-meta'));
        text.append(node('span', eraText(item), 'card-meta'));
        const chips = node('span', undefined, 'chips');
        for (const label of item.labels || []) chips.append(chip(label));
        text.append(chips);
        button.append(text);
        li.append(button);
        fragment.append(li);
        state.cards.set(item.id, li);
      }
      sheet.replaceChildren(fragment);
    }

    function applyFilters(keepFocus) {
      let filters;
      try {
        filters = readFilters();
        state.visible = filterItems(state.items, filters, state.data.nations);
      } catch (error) {
        setStatus(error.message, true);
        return;
      }
      const shown = new Set(state.visible.map((i) => i.id));
      for (const [id, li] of state.cards) li.hidden = !shown.has(id);
      const counts = {};
      for (const item of state.visible) counts[item.collection] = (counts[item.collection] || 0) + 1;
      const parts = COLLECTION_OPTIONS.slice(2).map(([id, label]) => counts[id] ? `${counts[id]} ${label.toLocaleLowerCase()}` : null).filter(Boolean);
      $('count').textContent = `${state.visible.length} of ${state.items.length} items shown` + (parts.length ? ` · ${parts.join(' · ')}` : '');
      $('empty').hidden = state.visible.length > 0;
      const current = document.activeElement && document.activeElement.closest && document.activeElement.closest('.card');
      const target = state.visible.find((i) => i.id === state.selected) || state.visible[0];
      rove(target ? target.id : null, keepFocus && current && !current.parentElement.hidden);
      writeURL();
    }

    function rove(id, focus) {
      for (const li of state.cards.values()) li.firstChild.tabIndex = -1;
      const li = id && state.cards.get(id);
      if (li) {
        li.firstChild.tabIndex = 0;
        if (focus) li.firstChild.focus();
      }
    }

    function columns() {
      const tops = state.visible.map((i) => state.cards.get(i.id).offsetTop);
      return columnsFromTops(tops);
    }

    function onSheetKey(event) {
      const card = event.target.closest('.card');
      if (!card) return;
      const index = state.visible.findIndex((i) => i.id === card.dataset.id);
      if (event.key === 'Enter' || event.key === ' ') {
        event.preventDefault();
        select(card.dataset.id, true);
        return;
      }
      const next = moveIndex(index, event.key, columns(), state.visible.length);
      if (next !== index || ['ArrowRight', 'ArrowLeft', 'ArrowUp', 'ArrowDown', 'Home', 'End', 'PageUp', 'PageDown'].includes(event.key)) {
        if (next === -1) return;
        event.preventDefault();
        rove(state.visible[next].id, true);
      }
    }

    function step(delta) {
      if (!state.visible.length) return;
      const index = state.visible.findIndex((i) => i.id === state.selected);
      const next = Math.min(state.visible.length - 1, Math.max(0, (index === -1 ? 0 : index + delta)));
      select(state.visible[next].id, true);
    }

    function openOverlay() {
      const detail = $('detail');
      detail.classList.add('open');
      detail.setAttribute('role', 'dialog');
      detail.setAttribute('aria-modal', 'true');
      document.body.classList.add('overlay-open');
      $('detail-close').hidden = false;
      $('detail-close').focus();
    }

    function closeOverlay() {
      const detail = $('detail');
      if (!detail.classList.contains('open')) return;
      detail.classList.remove('open');
      detail.setAttribute('role', 'region');
      detail.removeAttribute('aria-modal');
      document.body.classList.remove('overlay-open');
      $('detail-close').hidden = true;
      const li = state.selected && state.cards.get(state.selected);
      if (li && !li.hidden) li.firstChild.focus();
    }

    function select(id, fromUser) {
      const item = state.byId.get(id);
      if (!item) return;
      state.selected = id;
      for (const li of state.cards.values()) li.firstChild.setAttribute('aria-pressed', 'false');
      const li = state.cards.get(id);
      if (li) li.firstChild.setAttribute('aria-pressed', 'true');
      rove(id, fromUser && !narrow.matches && document.activeElement && document.activeElement.closest && document.activeElement.closest('.card'));
      renderDetail(item);
      writeURL();
      $('announce').textContent = `Showing ${item.name}`;
      if (fromUser && narrow.matches) openOverlay();
    }

    function section(title, className) {
      const el = node('section', undefined, 'panel ' + (className || ''));
      el.append(node('h3', title));
      return el;
    }

    function facts(list) {
      const dl = node('dl', undefined, 'facts');
      for (const [term, value] of list) {
        if (value === undefined || value === null || value === '') continue;
        dl.append(node('dt', term));
        const dd = node('dd');
        if (value instanceof window.Node) dd.append(value);
        else dd.textContent = String(value);
        dl.append(dd);
      }
      return dl;
    }

    function fileLink(path, label) {
      const href = url(path);
      if (!href) return node('span', path || '—');
      const a = node('a', label || path);
      a.href = href;
      a.target = '_blank';
      a.rel = 'noopener';
      return a;
    }

    function externalLink(value) {
      const href = publicURL(value);
      if (!href) return value ? node('span', value) : null;
      const a = node('a', value);
      a.href = href;
      a.target = '_blank';
      a.rel = 'noopener noreferrer';
      return a;
    }

    function views(item) {
      const wrap = node('div', undefined, 'views');
      const small = node('div', undefined, 'small-views');
      for (const size of sizesFor(item)) {
        const figure = node('figure', undefined, 'view view-' + size.id);
        figure.append(thumb(item, size, true));
        figure.append(node('figcaption', `${size.label} ${size.width}×${size.height}`));
        small.append(figure);
      }
      wrap.append(small);
      const compare = $('compare-anchor').checked;
      if (compare) {
        const strip = node('div', undefined, 'anchor-strip');
        strip.append(node('p', 'Style references at government-card size', 'note'));
        const row = node('div', undefined, 'small-views');
        for (const ref of state.data.style_references || []) {
          const refItem = state.byId.get(ref.item);
          if (!refItem) continue;
          const figure = node('figure', undefined, 'view');
          figure.append(thumb(refItem, CARD_SIZES[0], true));
          figure.append(node('figcaption', ref.role));
          row.append(figure);
        }
        strip.append(row);
        wrap.append(strip);
      }
      const full = node('figure', undefined, 'view view-full');
      const holder = node('div', undefined, 'full-holder');
      const badge = node('p', undefined, 'verify');
      full.append(holder);
      const facts0 = item.asset && state.files.get(item.asset);
      const caption = node('figcaption', facts0 && facts0.width ? `Full image ${facts0.width}×${facts0.height}, scaled to fit · ` : 'Full image');
      if (facts0 && facts0.exists) caption.append(fileLink(item.asset, 'open at 1:1 in a new tab'));
      full.append(caption);
      full.append(badge);
      wrap.append(full);
      if (item.collection === 'missing' || !facts0 || !facts0.exists) {
        holder.classList.add('placeholder');
        holder.append(node('span', item.collection === 'missing' ? 'Artwork missing: no cartoon exists for this person yet.' : 'File missing on disk.', 'placeholder-text'));
        badge.textContent = item.collection === 'missing' ? 'Nothing to verify.' : 'Recorded file is absent.';
        return wrap;
      }
      badge.textContent = 'Verifying file bytes…';
      loadFull(item, facts0, holder, badge);
      return wrap;
    }

    async function loadFull(item, facts0, holder, badge) {
      const current = nextFull();
      try {
        const response = await window.fetch(url(item.asset), { cache: 'no-store' });
        if (!response.ok) throw new Error(`HTTP ${response.status}`);
        const buffer = await response.arrayBuffer();
        const digest = await sha256Hex(buffer, window.crypto);
        if (!current()) return;
        if (fullURL) window.URL.revokeObjectURL(fullURL);
        fullURL = window.URL.createObjectURL(new window.Blob([buffer]));
        const img = node('img');
        img.alt = altText(item) + ', full size';
        img.src = fullURL;
        holder.replaceChildren(img);
        const ok = digest === facts0.sha256;
        badge.textContent = ok ? `SHA-256 verified against the export (${digest.slice(0, 12)}…)` : `Bytes differ from the export (${digest.slice(0, 12)}…); regenerate the export`;
        badge.classList.toggle('bad', !ok);
        badge.dataset.verified = ok ? 'true' : 'false';
      } catch (error) {
        if (!current()) return;
        badge.textContent = 'Could not load the full-size file: ' + error.message;
        badge.classList.add('bad');
        badge.dataset.verified = 'false';
      }
    }

    function renderDetail(item) {
      const body = $('detail-body');
      const head = node('header', undefined, 'detail-head');
      const title = node('h2', item.name);
      title.id = 'detail-name';
      head.append(title);
      const collection = state.data.collections[item.collection] || {};
      const where = (item.countries || []).map((c) => state.data.nations[c] || c).join(', ') || 'Country unknown';
      head.append(node('p', `${collection.label || item.collection} · ${where}`, 'detail-sub'));
      const chips = node('p', undefined, 'chips');
      for (const label of item.labels || []) chips.append(chip(label));
      head.append(chips);
      const nav = node('div', undefined, 'detail-nav');
      const prev = node('button', '← Previous', 'nav');
      prev.type = 'button';
      prev.setAttribute('aria-keyshortcuts', '[');
      prev.addEventListener('click', () => step(-1));
      const next = node('button', 'Next →', 'nav');
      next.type = 'button';
      next.setAttribute('aria-keyshortcuts', ']');
      next.addEventListener('click', () => step(1));
      nav.append(prev, next);
      head.append(nav);

      const parts = [head, views(item)];
      parts.push(appearance(item), automated(item), visual(item), recorded(item), provenance(item));
      const reference = referencePhoto(item);
      if (reference) parts.push(reference);
      body.replaceChildren(...parts.filter(Boolean));
      $('detail').hidden = false;
      $('detail').scrollTop = 0;
    }

    function appearance(item) {
      const panel = section('Appearance and identity');
      const identity = item.identity || {};
      const rows = [['Identity', identity.key || 'Not bound (identity unknown)'], ['Identity status', identity.status]];
      if (item.collection === 'selector') {
        rows.push(['Figure years', item.years], ['Dated appearance', 'None: selector art is not tied to an appearance interval']);
      } else if (item.interval) {
        rows.push(['Appearance interval', item.interval.valid ? formatInterval(item.interval) : `Invalid (${item.interval.from} → ${item.interval.to})`]);
      }
      if (item.life) rows.push(['Born', item.life.born], ['Died', item.life.died]);
      if (item.required_windows) rows.push(['Sourced art windows', item.required_windows.map(formatInterval).join('; ') || 'None recorded']);
      if (item.coverage_gaps) rows.push(['Uncovered windows', item.coverage_gaps.map(formatInterval).join('; ')]);
      if (item.reasons) rows.push(['Why art is needed', item.reasons.map(reasonText).join('; ')]);
      if (item.pending_art_jobs !== undefined) rows.push(['Pending art jobs', item.pending_art_jobs]);
      if (item.fiction) rows.push(['Fictional catalogue', Object.entries(item.fiction).map(([k, v]) => `${k}: ${v}`).join(' · ')]);
      panel.append(facts(rows));
      return panel;
    }

    function automated(item) {
      const panel = section('Automated findings', 'automated');
      panel.append(node('p', 'Integrity and consistency checks from the export. They are not visual approval, and a duplicate is not proof of a wrong identity.', 'note'));
      const list = state.findings.get(item.id) || [];
      if (!list.length) {
        panel.append(node('p', 'No automated findings for this item. This is not an approval.', 'quiet'));
        return panel;
      }
      const ul = node('ul', undefined, 'findings');
      for (const f of list) {
        const li = node('li', undefined, 'finding sev-' + f.severity);
        li.append(node('span', f.severity, 'sev'), node('code', f.code), node('span', ' ' + f.message));
        ul.append(li);
      }
      panel.append(ul);
      return panel;
    }

    function visual(item) {
      const entry = state.sample.get(item.id);
      const panel = section('Visual notes from this packet', 'visual');
      if (!entry) {
        panel.append(node('p', 'Not in the visual review sample. No visual judgement is recorded here.', 'quiet'));
        return panel;
      }
      const decision = node('p', undefined, 'decision');
      decision.append(node('strong', entry.decision.replace(/_/g, ' ')), node('span', ' · not approval · the artwork is unchanged', 'quiet'));
      if (!entry.current_sha256_matches) decision.append(node('span', ' · the file changed after this review', 'bad'));
      panel.append(decision);
      panel.append(facts([['Compared with', (entry.compared_with || []).join('; ')], ['At card size', entry.card_size], ['At full size', entry.full_size],
        ['Related automated findings', entry.automated_findings]]));
      panel.append(node('h4', 'Specific fixes proposed'));
      const ol = node('ol', undefined, 'fixes');
      for (const fix of entry.fixes || []) ol.append(node('li', fix));
      panel.append(ol);
      return panel;
    }

    function recorded(item) {
      const panel = section('Review recorded in the manifest');
      const review = item.review;
      if (!review) {
        panel.append(node('p', 'No review is recorded for this item.', 'quiet'));
        return panel;
      }
      panel.append(node('p', 'Recorded by an earlier production pass; reproduced here, not re-decided.', 'note'));
      panel.append(facts(Object.entries(review).map(([k, v]) => [k, typeof v === 'boolean' ? (v ? 'true' : 'false') : v])));
      return panel;
    }

    function provenance(item) {
      const panel = section('Provenance and rights');
      const asset = item.asset && state.files.get(item.asset);
      const card = item.card && state.files.get(item.card);
      const rights = item.rights || {};
      const prompt = item.prompt || {};
      panel.append(facts([
        ['Master file', item.asset ? fileLink(item.asset) : null],
        ['Master facts', asset ? (asset.exists ? `${asset.format || '?'} ${asset.width || '?'}×${asset.height || '?'} · ${asset.bytes} bytes` : 'absent') : null],
        ['Master SHA-256', asset && asset.sha256],
        ['Card image', item.card ? fileLink(item.card) : null],
        ['Card facts', card ? `${card.format} ${card.width}×${card.height} · ${card.bytes} bytes` : null],
        ['Generated', item.generated_at], ['Style', item.style],
        ['Artwork licence', rights.artwork_license], ['Derivative licence', rights.derivative_license],
        ['Identity reference', rights.identity_reference], ['Reference licence', rights.reference_license],
        ['Reference source', rights.reference_url ? externalLink(rights.reference_url) : null],
        ['Prompt record', prompt.path ? fileLink(prompt.path) : null],
        ['Prompt record SHA-256 (LF)', prompt.sha256_lf], ['Prompt record status', prompt.exists === false ? 'missing' : prompt.kind],
        ['Reference note', prompt.reference_note], ['Reference files named', (prompt.reference_files || []).join(', ')],
        ['Identity reference in prompt', prompt.identity_reference],
      ]));
      const manifestPath = (state.data.collections[item.collection] || {}).manifest;
      const record = manifestPath && state.manifests.has(manifestPath) ? resolvePointer(state.manifests.get(manifestPath), item.record) : undefined;
      if (record !== undefined) {
        const details = node('details', undefined, 'record');
        details.append(node('summary', `Manifest record (${manifestPath}${item.record})`));
        details.append(node('pre', JSON.stringify(record, null, 2)));
        panel.append(details);
      }
      return panel;
    }

    function referencePhoto(item) {
      const path = item.rights && item.rights.reference_file;
      if (!path) return null;
      const details = node('details', undefined, 'panel reference');
      details.append(node('summary', 'Identity reference photograph (not game art; never shown as an avatar)'));
      details.addEventListener('toggle', () => {
        if (!details.open || details.querySelector('img')) return;
        const img = node('img');
        img.alt = `Identity reference photograph for ${item.name} (not game art)`;
        img.loading = 'lazy';
        img.src = url(path);
        details.append(img, node('p', path, 'note'));
      });
      return details;
    }

    function renderRefs() {
      const strip = $('refs-list');
      strip.replaceChildren();
      for (const ref of state.data.style_references || []) {
        const item = state.byId.get(ref.item);
        const figure = node('figure', undefined, 'ref');
        if (item) figure.append(thumb(item, CARD_SIZES[0], false));
        const caption = node('figcaption');
        caption.append(node('strong', ref.role));
        if (ref.approval && ref.approval.scope) caption.append(node('span', ' · ' + ref.approval.scope));
        if (item) {
          const open = node('button', 'Open', 'link');
          open.type = 'button';
          open.addEventListener('click', () => {
            $('f-collection').value = 'all';
            select(item.id, true);
          });
          caption.append(node('br'), open);
        }
        figure.append(caption);
        strip.append(figure);
      }
    }

    function renderSummary() {
      const s = state.data.summary;
      const box = $('summary');
      box.replaceChildren();
      const add = (label, value, filter) => {
        const button = node('button', undefined, 'stat');
        button.type = 'button';
        button.append(node('b', String(value)), node('span', label));
        if (filter) button.addEventListener('click', () => {
          for (const [key, v] of Object.entries(filter)) $('f-' + key).value = v;
          applyFilters(false);
        });
        else button.disabled = true;
        box.append(button);
      };
      add('Existing cartoons', s.items.historical + s.items.fictional + s.items.selector, { collection: 'cartoons', label: 'all' });
      add('Artwork missing', s.items.missing, { collection: 'missing', label: 'all' });
      add('Integrity errors', s.findings_by_severity.error, { collection: 'all', label: 'integrity-error' });
      add('Different-identity duplicates', s.duplicate_groups_with_different_identities, { collection: 'all', label: 'duplicate' });
      add('Coverage gaps', s.findings_by_code.coverage_gap || 0, { collection: 'all', label: 'coverage-gap' });
      add('Rights gaps', (state.items.filter((i) => (i.labels || []).includes('rights-gap'))).length, { collection: 'all', label: 'rights-gap' });
      add('Visual notes (not approval)', s.visual_review_entries, { collection: 'all', label: 'sample-reviewed' });
      add('Approved by this export', s.approved_by_this_export, null);
    }

    function onGlobalKey(event) {
      const typing = event.target.closest && event.target.closest('input, select, textarea');
      if (event.key === 'Escape' && $('detail').classList.contains('open')) {
        event.preventDefault();
        closeOverlay();
        return;
      }
      if (typing || event.altKey || event.ctrlKey || event.metaKey) return;
      if (event.key === ']') { event.preventDefault(); step(1); }
      else if (event.key === '[') { event.preventDefault(); step(-1); }
      if (event.key === 'Tab' && $('detail').classList.contains('open')) {
        const focusable = [...$('detail').querySelectorAll('button:not([hidden]), a[href], summary, input')].filter((el) => el.offsetParent !== null);
        if (!focusable.length) return;
        const first = focusable[0];
        const last = focusable[focusable.length - 1];
        if (event.shiftKey && document.activeElement === first) { event.preventDefault(); last.focus(); }
        else if (!event.shiftKey && document.activeElement === last) { event.preventDefault(); first.focus(); }
      }
    }

    async function start() {
      setStatus('Loading the export…');
      if (window.location.protocol === 'file:') throw new Error('Serve the repository over http://127.0.0.1 (for example `python -m http.server 8790 --bind 127.0.0.1` from the repository root); browsers block local file reads.');
      const response = await window.fetch(base + EXPORT_PATH, { cache: 'no-store' });
      if (!response.ok) throw new Error(`Export unavailable (${response.status}). Generate it with python -X utf8 tools/avatars/cartoon_review.py`);
      const data = await response.json();
      if (data.format !== EXPORT_FORMAT) throw new Error('Unsupported export format: ' + data.format);
      setStatus('Verifying the export against its pinned inputs…');
      const results = await verifyInputs(data.inputs, window.fetch.bind(window), window.crypto, base);
      const changed = results.filter((r) => !r.matches);
      if (changed.length) {
        const error = new Error('The export is stale: ' + changed.map((r) => r.path).join(', ') + ' changed after it was generated. Regenerate with python -X utf8 tools/avatars/cartoon_review.py and reload.');
        error.stale = true;
        throw error;
      }
      const decoder = new window.TextDecoder('utf-8');
      for (const r of results) {
        try { state.manifests.set(r.path, JSON.parse(decoder.decode(r.buffer))); } catch (error) { /* non-JSON inputs are verified only */ }
        delete r.buffer;
      }
      state.verification = results;
      state.data = data;
      state.items = data.items;
      for (const item of data.items) state.byId.set(item.id, item);
      for (const f of data.files) state.files.set(f.path, f);
      for (const f of data.findings) {
        if (!f.item) continue;
        if (!state.findings.has(f.item)) state.findings.set(f.item, []);
        state.findings.get(f.item).push(f);
      }
      for (const entry of (data.visual_review_sample || {}).entries || []) state.sample.set(entry.item, entry);
      const params = new URLSearchParams(window.location.search);
      fillSelect($('f-country'), [['all', 'All countries'], ...nationOptions(data.items, data.nations)], params.get('country'));
      fillSelect($('f-era'), ERAS.map((e) => [e.id, e.label]), params.get('era'));
      fillSelect($('f-collection'), COLLECTION_OPTIONS, params.get('collection') || 'all');
      const used = new Set(data.items.flatMap((i) => i.labels || []));
      fillSelect($('f-label'), [['all', 'Any label or finding'], ...Object.keys(LABELS).filter((l) => used.has(l)).map((l) => [l, LABELS[l]])], params.get('label'));
      $('f-search').value = params.get('q') || '';
      if (/^\d{4}-\d{2}-\d{2}$/.test(params.get('date') || '')) $('f-date').value = params.get('date');
      buildCards();
      renderRefs();
      renderSummary();
      const s = data.summary;
      setStatus(`Export verified: ${results.length} pinned inputs match their SHA-256. ${data.items.length} items · ${s.findings_by_severity.error} integrity errors · ${s.findings_by_severity.warning} warnings · ${s.findings_by_severity.notice} notices. Automated findings are not visual approval.`);
      document.body.dataset.state = 'ready';
      applyFilters(false);
      const wanted = params.get('item');
      const initial = state.byId.has(wanted) ? wanted : (state.visible[0] || {}).id;
      if (initial) select(initial, false);
      if (wanted && state.byId.has(wanted) && narrow.matches) openOverlay();
    }

    for (const id of ['f-search', 'f-date']) $(id).addEventListener('input', () => applyFilters(false));
    for (const id of ['f-country', 'f-era', 'f-collection', 'f-label']) $(id).addEventListener('change', () => applyFilters(false));
    $('filters').addEventListener('submit', (event) => event.preventDefault());
    $('filters').addEventListener('reset', () => window.setTimeout(() => { $('f-collection').value = 'all'; applyFilters(false); }, 0));
    $('compare-anchor').addEventListener('change', () => { const item = state.byId.get(state.selected); if (item) renderDetail(item); });
    $('sheet').addEventListener('keydown', onSheetKey);
    $('sheet').addEventListener('click', (event) => {
      const card = event.target.closest('.card');
      if (card) select(card.dataset.id, true);
    });
    $('detail-close').addEventListener('click', closeOverlay);
    document.addEventListener('keydown', onGlobalKey);
    narrow.addEventListener('change', () => { if (!narrow.matches) closeOverlay(); });
    window.CartoonReviewState = state;
    start().catch((error) => {
      document.body.dataset.state = error.stale ? 'stale' : 'error';
      $('sheet').replaceChildren();
      $('detail').hidden = true;
      setStatus(error.message, true);
    });
  }

  return { EXPORT_PATH, ERAS, LABELS, CARD_SIZES, SELECTOR_SIZES, COLLECTION_OPTIONS, fold, labelText, itemIntervals, overlaps, contains,
    matchesFilters, filterItems, moveIndex, columnsFromTops, safePath, publicURL, resolvePointer, formatInterval, initials, sizesFor, reasonText,
    sha256Hex, verifyInputs, latest, nationOptions, boot };
});
