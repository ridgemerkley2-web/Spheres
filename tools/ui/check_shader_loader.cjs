// Run: node --test tools/ui/check_shader_loader.cjs
// Executes the real loader with a controlled GPU and task queue; no real-time sleeps.
const assert = require('node:assert/strict');
const fs = require('node:fs');
const path = require('node:path');
const vm = require('node:vm');
const {test} = require('node:test');
const source = fs.readFileSync(path.join(__dirname, '../../spheres-web/ui/shader-loader.js'), 'utf8');
function fixture(options = {}) {
  const calls = [], pending = [], resources = [], deleted = new Set(), attachments = new Map();
  let clock = 0, lost = Boolean(options.initialLoss), shaderCount = 0, queryCount = 0;
  const gl = {VERTEX_SHADER: 'vertex', FRAGMENT_SHADER: 'fragment', COMPILE_STATUS: 'compile', LINK_STATUS: 'link'};
  const ext = {COMPLETION_STATUS_KHR: 'complete'};
  const allocate = kind => {const value = {kind, id: resources.length}; resources.push(value); return value;};
  Object.assign(gl, {
    isContextLost() {return lost;},
    getExtension(name) {calls.push({op: 'extension', name}); return options.extension === false ? null : ext;},
    createShader(type) {shaderCount++; calls.push({op: 'createShader', type}); return options.failShader === shaderCount ? null : allocate(type);},
    shaderSource(shader, text) {calls.push({op: 'source', shader, text});},
    compileShader(shader) {calls.push({op: 'compile', shader}); if (options.compileThrow) throw new Error('driver compile exception');},
    createProgram() {calls.push({op: 'createProgram'}); return options.failProgram ? null : allocate('program');},
    attachShader(program, shader) {calls.push({op: 'attach', program, shader}); attachments.set(shader, program);},
    detachShader(program, shader) {assert.equal(attachments.get(shader), program); attachments.delete(shader); calls.push({op: 'detach', shader});},
    linkProgram(program) {calls.push({op: 'link', program}); if (options.linkThrow) throw new Error('driver link exception');},
    getProgramParameter(program, key) {
      calls.push({op: 'programParameter', program, key});
      if (key === ext.COMPLETION_STATUS_KHR) {
        queryCount++;
        if (options.lossOnCompletion) {lost = true; return true;}
        return options.neverComplete ? false : queryCount > (options.pendingPolls ?? 2);
      }
      assert.equal(key, gl.LINK_STATUS);
      assert(options.extension === false || queryCount > (options.pendingPolls ?? 2), 'LINK_STATUS must not be queried while compilation is pending');
      return options.linkStatus !== false;
    },
    getShaderParameter(shader, key) {calls.push({op: 'shaderParameter', shader, key}); assert.equal(key, gl.COMPILE_STATUS); return !(options.failedStages || []).includes(shader.kind);},
    getShaderInfoLog(shader) {calls.push({op: 'shaderLog', shader}); return options.emptyLogs ? '' : shader.kind + ' full compile log';},
    getProgramInfoLog(program) {calls.push({op: 'programLog', program}); return options.emptyLogs ? '' : 'full link log';},
    deleteShader(shader) {assert(!deleted.has(shader)); deleted.add(shader); calls.push({op: 'deleteShader', shader});},
    deleteProgram(program) {assert(!deleted.has(program)); deleted.add(program); calls.push({op: 'deleteProgram', program});},
  });
  const sandbox = vm.createContext({window: {}, performance: {now: () => clock},
    setTimeout(callback, delay) {calls.push({op: 'timer', delay}); pending.push(callback);},
  });
  vm.runInContext(source, sandbox, {filename: 'shader-loader.js'});
  return {api: sandbox.window.ShaderLoader, gl, ext, calls, pending, resources, deleted, attachments,
    lose() {lost = true;},
    async step(elapsed = 20) {assert(pending.length, 'a polling task is pending'); clock += elapsed; pending.shift()(); await Promise.resolve();},
    cleanExcept(program) {assert.equal(attachments.size, 0); assert.deepEqual(resources.filter(value => !deleted.has(value)), program ? [program] : []);},
  };
}

test('parallel compilation yields to menu tasks and polls only nonblocking completion until ready', async () => {
  const f = fixture(); let settled = false;
  const result = f.api.program(f.gl, 'actual vertex source', 'actual fragment source', 'map').then(value => {settled = true; return value;});
  assert.equal(settled, false); assert.equal(f.pending.length, 1);
  assert.deepEqual(f.calls.filter(call => call.op === 'source').map(call => call.text), ['actual vertex source', 'actual fragment source']);
  assert.equal(f.calls.filter(call => call.op === 'compile').length, 2);
  assert(!f.calls.some(call => call.op === 'shaderParameter'));
  assert(!f.calls.some(call => call.op === 'programParameter' && call.key === f.gl.LINK_STATUS));
  let menuHandled = false; queueMicrotask(() => {menuHandled = true;}); await Promise.resolve();
  assert.equal(menuHandled, true); assert.equal(settled, false);
  await f.step(); assert.equal(f.pending.length, 1); assert.equal(settled, false);
  await f.step(); const program = await result;
  assert.equal(f.pending.length, 0); assert.equal(settled, true);
  assert(f.calls.filter(call => call.op === 'timer').every(call => call.delay === 20));
  assert.equal(f.calls.filter(call => call.op === 'programParameter' && call.key === f.gl.LINK_STATUS).length, 1);
  assert(!f.calls.some(call => call.op === 'shaderParameter')); f.cleanExcept(program);
});

test('an already cached parallel program completes without scheduling unnecessary polls', async () => {
  const f = fixture({pendingPolls: 0}); const program = await f.api.program(f.gl, 'vs', 'fs', 'map');
  assert.equal(f.pending.length, 0); assert(!f.calls.some(call => call.op === 'timer')); f.cleanExcept(program);
});

test('without the extension the equivalent synchronous link path remains available', async () => {
  const f = fixture({extension: false}); const program = await f.api.program(f.gl, 'vs', 'fs', 'map');
  assert.equal(f.pending.length, 0);
  assert.deepEqual(f.calls.filter(call => call.op === 'programParameter').map(call => call.key), [f.gl.LINK_STATUS]);
  assert(!f.calls.some(call => call.op === 'shaderParameter')); f.cleanExcept(program);
});

test('failed linking reports both shader compile logs and the complete program log after completion', async () => {
  const f = fixture({pendingPolls: 1, linkStatus: false, failedStages: ['vertex', 'fragment']});
  const result = f.api.program(f.gl, 'vs', 'fs', 'map');
  const rejection = assert.rejects(result, error => {
    assert.match(error.message, /map\.vs compile: vertex full compile log/);
    assert.match(error.message, /map\.fs compile: fragment full compile log/);
    assert.match(error.message, /map link: full link log/); return true;
  });
  assert(!f.calls.some(call => call.op === 'shaderParameter')); await f.step(); await rejection; f.cleanExcept();
});

test('a link-only failure retains its diagnostic and releases every resource', async () => {
  const f = fixture({extension: false, linkStatus: false});
  await assert.rejects(f.api.program(f.gl, 'vs', 'fs', 'map'), /^Error: map link: full link log$/);
  assert.equal(f.calls.filter(call => call.op === 'shaderParameter').length, 2); f.cleanExcept();
});

test('missing driver logs use an explicit diagnostic instead of an empty error', async () => {
  const f = fixture({extension: false, linkStatus: false, failedStages: ['fragment'], emptyLogs: true});
  await assert.rejects(f.api.program(f.gl, 'vs', 'fs', 'map'), /map\.fs compile: \(no info log\)\nmap link: \(no info log\)/); f.cleanExcept();
});

test('a context already lost fails before allocation', async () => {
  const f = fixture({initialLoss: true}); await assert.rejects(f.api.program(f.gl, 'vs', 'fs', 'map'), /context lost/);
  assert.equal(f.calls.length, 0); f.cleanExcept();
});

test('context loss during a pending poll cleans up without any blocking status query', async () => {
  const f = fixture(); const result = f.api.program(f.gl, 'vs', 'fs', 'map');
  const rejection = assert.rejects(result, /context lost/); f.lose(); await f.step(); await rejection;
  assert.equal(f.pending.length, 0); assert(!f.calls.some(call => call.op === 'programParameter' && call.key === f.gl.LINK_STATUS)); f.cleanExcept();
});

test('completion true caused by context loss is never accepted as success', async () => {
  const f = fixture({lossOnCompletion: true}); await assert.rejects(f.api.program(f.gl, 'vs', 'fs', 'map'), /context lost/);
  assert.equal(f.pending.length, 0); assert(!f.calls.some(call => call.op === 'programParameter' && call.key === f.gl.LINK_STATUS)); f.cleanExcept();
});

test('a stalled driver is bounded by elapsed time with only one scheduled poll at once', async () => {
  const f = fixture({neverComplete: true}); const result = f.api.program(f.gl, 'vs', 'fs', 'map');
  const rejection = assert.rejects(result, /compilation timed out/);
  await f.step(60000); assert.equal(f.pending.length, 1);
  await f.step(60000); await rejection; assert.equal(f.pending.length, 0); f.cleanExcept();
});

test('partial allocation failures clean up all handles that were created', async () => {
  for (const options of [{failShader: 1}, {failShader: 2}, {failProgram: true}]) {
    const f = fixture(options); await assert.rejects(f.api.program(f.gl, 'vs', 'fs', 'map'), /allocation failed/); f.cleanExcept();
  }
});

test('unexpected compile or link exceptions also release temporary GPU resources', async () => {
  for (const options of [{compileThrow: true}, {linkThrow: true}]) {
    const f = fixture(options); await assert.rejects(f.api.program(f.gl, 'vs', 'fs', 'map'), /driver .* exception/); f.cleanExcept();
  }
});
