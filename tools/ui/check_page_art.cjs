const test = require('node:test');
const assert = require('node:assert/strict');
const fs = require('node:fs');
const path = require('node:path');
const art = require('../../spheres-web/ui/page-art.js');
const ui = path.resolve(__dirname, '../../spheres-web/ui');
const css = fs.readFileSync(path.join(ui, 'page-art.css'), 'utf8');

test('every painted scene is a packaged local WebP rather than a remote dependency', () => {
  const urls = [...new Set([...css.matchAll(/url\(['"]([^'"]+)['"]\)/g)].map(match => match[1]))];
  assert.equal(urls.length, 39);
  assert.deepEqual(new Set(urls), new Set(Object.values(art.scenes).map(scene => scene.url)));
  for (const scene of Object.values(art.scenes)) {
    const file = path.join(ui, scene.file);
    assert.ok(fs.existsSync(file), `Missing painted scene: ${scene.file}`);
    const bytes = fs.readFileSync(file);
    assert.equal(bytes.toString('ascii', 0, 4), 'RIFF', scene.file);
    assert.equal(bytes.toString('ascii', 8, 12), 'WEBP', scene.file);
    assert.equal(bytes.readUInt32LE(4) + 8, bytes.length, `Truncated WebP: ${scene.file}`);
    assert.ok(bytes.length < 2_000_000, `Scene exceeds 2 MB asset budget: ${scene.file}`);
  }
});

test('scene catalogue rejects arbitrary paths and prototype properties', () => {
  for (const key of ['__proto__', 'constructor', '../campaign', 'https://example.com/x', '', null]) assert.equal(art.scene(key), null);
  assert.equal(art.scene('tank-designer').url, '/art/pages/tank-designer-v1.webp');
  assert.ok(Object.isFrozen(art.scenes));
  assert.ok(Object.isFrozen(art.scene('tank-designer')));
});

test('art is the final presentation stylesheet so existing room backdrops cannot mask it', () => {
  const index = fs.readFileSync(path.join(ui, 'index.html'), 'utf8');
  const styles = [...index.matchAll(/<link\b[^>]*rel="stylesheet"[^>]*href="([^"]+)"[^>]*>/g)].map(match => match[1]);
  assert.equal(styles.at(-1), '/page-art.css');
  const rust = fs.readFileSync(path.join(ui, '../src/main.rs'), 'utf8');
  const embeds = fs.readFileSync(path.join(ui, '../src/page_art_assets.rs'), 'utf8');
  assert.ok(embeds.includes('include_str!("../ui/page-art.css")'), 'Page art CSS must be embedded in the offline build');
  assert.ok(rust.includes('"/page-art.css"'), 'Page art CSS must have a public asset route');
});

test('background choice follows existing accessible page state without new UI lifecycle work', () => {
  const index = fs.readFileSync(path.join(ui, 'index.html'), 'utf8');
  const equipment = fs.readFileSync(path.join(ui, 'equipment-ui.js'), 'utf8');
  assert.match(equipment, /data-equipment-tab-view/);
  for (const view of ['designer', 'library', 'development', 'production', 'service']) assert.ok(css.includes(`[data-equipment-tab-view=${view}]`));
  for (const field of ['Computing', 'Communications', 'Energy', 'Materials', 'Aerospace', 'Biotech', 'Transport', 'Agriculture']) {
    assert.ok(css.includes(`[data-view=${field}]`));
    assert.ok(index.includes(`data-dom="${field}"`));
  }
  const runtime = fs.readFileSync(path.join(ui, 'page-art.js'), 'utf8');
  assert.doesNotMatch(runtime, /\b(?:setInterval|setTimeout|requestAnimationFrame|MutationObserver|fetch|addEventListener)\s*\(/);
  assert.doesNotMatch(css, /#(?:pane-map|mapCanvas|mapViewport|globeCanvas)\b/);
});

test('art includes narrow-screen and high-contrast treatments without new moving backgrounds', () => {
  assert.match(css, /@media\s*\(max-width:760px\)/);
  assert.match(css, /@media\s*\(forced-colors:active\)/);
  assert.match(css, /@media\s*\(prefers-reduced-motion:reduce\)/);
  assert.doesNotMatch(css, /background-attachment\s*:\s*fixed|@keyframes|filter\s*:\s*blur\(/);
  assert.doesNotMatch(css, /pointer-events\s*:\s*(?:auto|all)/);
});
