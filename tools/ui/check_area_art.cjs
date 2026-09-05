// Presentation regressions exercise the actual helper and room renderers.
const {test} = require('node:test');
const assert = require('node:assert/strict');
const fs = require('node:fs');
const path = require('node:path');
const vm = require('node:vm');
const art = require('../../spheres-web/ui/area-art.js');
const read = name => fs.readFileSync(path.join(__dirname, '../../spheres-web/ui', name), 'utf8');
const page = read('index.html');
function functionSource(name) {
  const start = page.indexOf(`function ${name}(`);
  assert(start >= 0);
  return page.slice(start, page.indexOf('\n}', start) + 2);
}

test('ten distinct local paintings have immutable keys and accessible decorative markup', () => {
  assert.deepEqual(art.keys, ['cabinet','treasury','production','research','diplomacy','military','logistics','resources','history','campaign']);
  assert(Object.isFrozen(art.keys));
  assert.equal(new Set(art.keys.map(art.url)).size, 10);
  for (const key of art.keys) {
    const html = art.html(key);
    assert(html.includes(`src="/art/areas/${key}-v1.webp"`));
    assert.match(html, /aria-hidden="true"/);
    assert.match(html, /alt=""/);
    assert.match(html, /width="1536" height="1024"/);
    assert.match(html, /decoding="async"/);
    assert.doesNotMatch(html, /<button|<a |tabindex|https?:\/\//);
  }
  assert.match(art.html('campaign'), /loading="eager"/);
  assert.match(art.html('research'), /loading="lazy"/);
});

test('untrusted paths and layout values cannot enter image markup', () => {
  for (const value of ['', null, undefined, '../cabinet', '/cabinet', 'cabinet-v1.webp', 'constructor', '__proto__', 'cabinet" onerror="alert(1)', 'https://example.test/image']) {
    assert.equal(art.url(value), null);
    assert.equal(art.html(value), '');
  }
  const markup = art.html('cabinet', 'hero" onload="alert(1)');
  assert.match(markup, /class="area-art area-art--hero"/);
  assert.doesNotMatch(markup, /onload|alert/);
});

test('declarative campaign mounting uses the same allowlist and preserves its host', () => {
  const slots = [
    {dataset:{areaArt:'campaign',areaArtSize:'campaign'},innerHTML:''},
    {dataset:{areaArt:'../bad'},innerHTML:''},
  ];
  art.mount({querySelectorAll(selector) { assert.equal(selector, '[data-area-art]'); return slots; }});
  assert.equal(slots[0].innerHTML, art.html('campaign','campaign'));
  assert.equal(slots[1].innerHTML, '');
  assert(page.indexOf('/area-art.js') < page.indexOf('/agency-ui.js'));
  assert.match(page, /class="session-room-art" data-area-art="campaign"/);
});

test('production, arms manufacturing and logistics render the matching paintings', () => {
  const c = vm.createContext({AreaArt:art,escText:value=>String(value)});
  vm.runInContext(functionSource('operationsSceneHtml') + '\n' + functionSource('operationsHeroHtml'), c);
  for (const [kind,key] of [['production','production'],['manufacturing','military'],['logistics','logistics']]) {
    const html = c.operationsHeroHtml(kind,'Room','Title','Description');
    assert(html.includes(art.html(key)));
    assert.match(html, /<h3>Title<\/h3><p>Description<\/p>/);
    assert.doesNotMatch(html, /<svg/);
  }
});

test('Exchange keeps industry, resources and diplomacy visually distinct', () => {
  const c = vm.createContext({AreaArt:art});
  vm.runInContext(read('competition-ui.js'), c);
  for (const [tab,key] of [['industry','production'],['trade','resources'],['world','production'],['sphere','diplomacy']]) {
    vm.runInContext(`COMP.tab=${JSON.stringify(tab)}`, c);
    const html = c.competitionHero('Context','A <title>','A description');
    assert(html.includes(art.html(key)));
    assert.match(html, /A &lt;title&gt;/);
  }
});

test('military overview has one painting and a focused allocation retains its compact controls', () => {
  const c = vm.createContext({AreaArt:art});
  vm.runInContext(read('operations-ui.js'), c);
  const data = {enabled:true,structure:10,deployed:0,reserve:10,overseas_deployed:0,overseas_limit:2,deployments:[]};
  const overview = c.MilitaryOperationsUI.html(data, []);
  assert(overview.includes(art.html('military','banner')));
  assert.match(overview, /No deployments/);
  assert.doesNotMatch(c.MilitaryOperationsUI.html(data, [], 42), /data-area=/);
  assert.equal(c.MilitaryOperationsUI.html({enabled:false}), '');
});
