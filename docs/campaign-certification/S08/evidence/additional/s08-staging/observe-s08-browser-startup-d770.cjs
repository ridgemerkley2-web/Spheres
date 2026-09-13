'use strict';

// Observational preload for the unchanged installed-Chrome CI harness.
// It never installs a route, reads a response body, changes a timeout, or sends
// a game command. Navigation failures are rethrown unchanged after bounded reads.
const fs = require('node:fs');
const path = require('node:path');
const crypto = require('node:crypto');
const { createRequire } = require('node:module');
const base = path.resolve(__dirname, '..');
const ci = path.join(base, 'integration', 'tools', 'ui', 'ci-browser.cjs');
const scoped = createRequire(ci);
const { chromium } = scoped('playwright');
const selected = process.env.SPHERES_BROWSER_OBSERVE_OUT;
if (selected && !path.isAbsolute(selected)) {
  throw new Error('SPHERES_BROWSER_OBSERVE_OUT must be an absolute, nonexistent directory.');
}
const out = selected ? path.resolve(selected) : fs.mkdtempSync(path.join(__dirname, 'd770-navigation-observation-'));
if (selected) fs.mkdirSync(out);
const eventsPath = path.join(out, 'events.jsonl');
const started = Date.now();
let eventCount = 0;
let pageSequence = 0;
let writeErrorReported = false;
let eventLimitReported = false;
const MAX_EVENTS = 12000;
const short = (value, max = 4096) => String(value == null ? '' : value).slice(0, max);
const errorInfo = error => ({ name: short(error?.name, 128), message: short(error?.message || error), stack: short(error?.stack, 12000) });
function emit(type, data = {}, important = false) {
  // A logging error must not turn a successful application operation into a
  // different result. The console warning makes incomplete observation explicit.
  try {
    if (!important && eventCount >= MAX_EVENTS) {
      if (!eventLimitReported) {
        eventLimitReported = true;
        emit('observation.event_limit', { maximum: MAX_EVENTS }, true);
      }
      return;
    }
    eventCount += 1;
    fs.appendFileSync(eventsPath, JSON.stringify({ at: new Date().toISOString(), elapsed_ms: Date.now() - started, type, ...data }) + '\n');
  } catch (error) {
    if (!writeErrorReported) {
      writeErrorReported = true;
      process.stderr.write(`[S08 observer] Observation write failed: ${short(error)}\n`);
    }
  }
}
function listen(surface, event, callback) {
  surface.on(event, (...args) => {
    try { callback(...args); }
    catch (error) { emit('observation.listener_error', { event, error: errorInfo(error) }, true); }
  });
}
function hash(file) {
  return { path: file, sha256: crypto.createHash('sha256').update(fs.readFileSync(file)).digest('hex') };
}
const metadata = {
  schema: 's08-browser-startup-observation-v1',
  expected_candidate: 'd770592aeead51f1313d507edd26b02d75a69bba',
  expected_revision_environment: process.env.SPHERES_EXPECTED_REVISION || process.env.EXPECTED_REVISION || null,
  created_at: new Date().toISOString(), pid: process.pid,
  argv: process.argv, exec_argv: process.execArgv,
  node: process.version, platform: process.platform, arch: process.arch,
  source_files: [__filename, ci, path.join(base, 'integration', 'tools', 'ui', 'ci-integrated.cjs'), path.join(base, 's05-installed-browser.cjs')].map(hash),
  semantics: 'Observe the real browser with the original launch/navigation arguments, timeouts, routes, and assertions. A navigation failure adds at most two bounded diagnostic reads before rethrowing the same error. This is diagnostic evidence; the outer runner remains the revision/binary/acceptance authority.',
};
fs.writeFileSync(path.join(out, 'metadata.json'), JSON.stringify(metadata, null, 2) + '\n');
process.stdout.write(`[S08 observer] ${out}\n`);
emit('observation.start', { out }, true);

async function bounded(label, milliseconds, operation) {
  let timer;
  try {
    return await Promise.race([
      Promise.resolve().then(operation).then(value => ({ status: 'ok', value }), error => ({ status: 'error', error: errorInfo(error) })),
      new Promise(resolve => { timer = setTimeout(() => resolve({ status: 'timeout', timeout_ms: milliseconds, label }), milliseconds); }),
    ]);
  } finally { clearTimeout(timer); }
}

function observePage(page) {
  const pageId = ++pageSequence;
  const requests = new WeakMap();
  const pending = new Map();
  let requestSequence = 0;
  let failureSequence = 0;
  let droppedPending = 0;
  const frameUrl = request => {
    try { return request.frame().url(); }
    catch { return null; }
  };
  const requestInfo = request => ({
    request_id: requests.get(request) || null,
    url: short(request.url(), 8192), method: request.method(),
    resource_type: request.resourceType(), navigation: request.isNavigationRequest(),
    frame_url: frameUrl(request),
  });
  listen(page, 'request', request => {
    const id = `${pageId}:${++requestSequence}`;
    requests.set(request, id);
    const info = { ...requestInfo(request), elapsed_ms_started: Date.now() - started };
    if (pending.size < 2048) pending.set(id, info); else droppedPending += 1;
    emit('request', { page_id: pageId, ...info });
  });
  listen(page, 'response', response => {
    const headers = response.headers();
    emit('response', { page_id: pageId, ...requestInfo(response.request()), status: response.status(),
      from_service_worker: response.fromServiceWorker(), content_type: headers['content-type'] || null,
      content_length: headers['content-length'] || null });
  });
  listen(page, 'requestfinished', request => {
    pending.delete(requests.get(request));
    emit('requestfinished', { page_id: pageId, ...requestInfo(request), timing: request.timing() });
  });
  listen(page, 'requestfailed', request => {
    pending.delete(requests.get(request));
    emit('requestfailed', { page_id: pageId, ...requestInfo(request), failure: request.failure() }, true);
  });
  listen(page, 'console', message => emit('console', { page_id: pageId, level: message.type(), text: short(message.text()), location: message.location() }));
  listen(page, 'pageerror', error => emit('pageerror', { page_id: pageId, error: errorInfo(error) }, true));
  listen(page, 'crash', () => emit('page.crash', { page_id: pageId }, true));
  listen(page, 'close', () => emit('page.close', { page_id: pageId, pending: [...pending.values()], untracked_requests: droppedPending }, true));
  listen(page, 'domcontentloaded', () => emit('domcontentloaded', { page_id: pageId, url: page.url() }, true));
  listen(page, 'load', () => emit('load', { page_id: pageId, url: page.url() }, true));
  listen(page, 'framenavigated', frame => emit('framenavigated', { page_id: pageId, main_frame: frame === page.mainFrame(), url: frame.url() }));

  async function captureFailure(method, error) {
    const prefix = `page-${pageId}-${method}-failure-${++failureSequence}`;
    const snapshot = {
      schema: 's08-browser-navigation-failure-v1', page_id: pageId, method,
      at: new Date().toISOString(), error: errorInfo(error), url: page.url(),
      closed: page.isClosed(), pending: [...pending.values()], untracked_requests: droppedPending,
      process_memory: process.memoryUsage(),
    };
    emit('navigation.failure', snapshot, true);
    snapshot.document = await bounded('read document state', 2000, () => page.evaluate(() => {
      const safe = read => { try { return read(); } catch (error) { return { unavailable: String(error) }; } };
      return {
        url: location.href, ready_state: document.readyState, visibility_state: document.visibilityState,
        title: document.title,
        navigation: performance.getEntriesByType('navigation').map(entry => entry.toJSON()),
        scripts: Array.from(document.scripts).map(script => ({ src: script.src || '[inline]', async: script.async, defer: script.defer, type: script.type || 'classic' })),
        resource_tail: performance.getEntriesByType('resource').slice(-100).map(entry => ({ name: entry.name, initiator_type: entry.initiatorType, start_time: entry.startTime, duration: entry.duration, response_end: entry.responseEnd, transfer_size: entry.transferSize })),
        incomplete_images: Array.from(document.images).filter(image => !image.complete).slice(0, 100).map(image => ({ src: image.currentSrc || image.src, loading: image.loading })),
        session: safe(() => typeof SESSION === 'undefined' ? { defined: false } : { defined: true, has_live: !!SESSION.live, has_live_session_id: !!SESSION.live?.session_id, has_player: !!SESSION.live?.player }),
        menu: safe(() => ({ view: document.querySelector('#setup')?.dataset.menuView || null, has_home: !!document.querySelector('#campaignHome') })),
      };
    }));
    const screenshotPath = path.join(out, `${prefix}.png`);
    snapshot.screenshot = await bounded('capture viewport', 2250, async () => {
      await page.screenshot({ path: screenshotPath, timeout: 2000 });
      return { path: screenshotPath };
    });
    fs.writeFileSync(path.join(out, `${prefix}.json`), JSON.stringify(snapshot, null, 2) + '\n');
    emit('navigation.failure_diagnostics', { page_id: pageId, method, artifact: `${prefix}.json`, document_status: snapshot.document.status, screenshot_status: snapshot.screenshot.status }, true);
  }

  for (const method of ['goto', 'reload']) {
    const original = page[method].bind(page);
    page[method] = async (...args) => {
      emit('navigation.start', { page_id: pageId, method, arguments: args }, true);
      try {
        const result = await original(...args);
        emit('navigation.success', { page_id: pageId, method, url: page.url() }, true);
        return result;
      } catch (error) {
        try { await captureFailure(method, error); }
        catch (diagnosticError) { emit('observation.capture_error', { page_id: pageId, method, error: errorInfo(diagnosticError) }, true); }
        throw error;
      }
    };
  }
  emit('page.observed', { page_id: pageId }, true);
  return page;
}

const originalLaunch = chromium.launch.bind(chromium);
chromium.launch = async (...args) => {
  emit('browser.launch', { arguments: args }, true);
  const browser = await originalLaunch(...args);
  emit('browser.launched', { version: browser.version() }, true);
  listen(browser, 'disconnected', () => emit('browser.disconnected', {}, true));
  const originalNewPage = browser.newPage.bind(browser);
  browser.newPage = async (...pageArgs) => observePage(await originalNewPage(...pageArgs));
  return browser;
};
process.on('exit', code => emit('process.exit', { code, event_count: eventCount, event_limit_reached: eventLimitReported, observation_write_failed: writeErrorReported }, true));
