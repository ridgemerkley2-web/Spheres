const test = require('node:test');
const assert = require('node:assert/strict');
const fs = require('node:fs');
const vm = require('node:vm');
const model = require('../../spheres-web/ui/tutorial-model.js');
const ids = model.lessons.map(lesson => lesson.id);
const initial = () => ({version: 1, done: [], skipped: [], current: ids[0], started: false});

test('nine frozen lessons describe real reading workflows and navigation only', () => {
  assert.equal(ids.length, 9);
  assert.equal(new Set(ids).size, ids.length);
  assert.deepEqual(model.lessons.map(lesson => lesson.action.kind),
    ['home', 'budget', 'construction', 'industry', 'government', 'research', 'equipment', 'world', 'campaign']);
  for (const lesson of model.lessons) {
    for (const key of ['id', 'title', 'area', 'summary', 'lookFor', 'actionLabel']) {
      assert.equal(typeof lesson[key], 'string');
      assert.ok(lesson[key].trim());
    }
    assert.ok(lesson.steps.length >= 2 && lesson.steps.every(step => typeof step === 'string' && step.trim()));
    assert.deepEqual(Object.keys(lesson.action), ['kind']);
    assert.ok(Object.isFrozen(lesson) && Object.isFrozen(lesson.steps) && Object.isFrozen(lesson.action));
  }
  assert.ok(Object.isFrozen(model.lessons));
  assert.match(model.lessons[1].steps.join(' '), /Enact & advance 1 day/);
  assert.match(model.lessons[6].steps.join(' '), /own working capital/);
  assert.match(model.lessons[8].lookFor, /separate from campaign saves/);
});

test('UMD browser export works without a DOM, localStorage or campaign services', () => {
  const context = vm.createContext({});
  vm.runInContext(fs.readFileSync(require.resolve('../../spheres-web/ui/tutorial-model.js'), 'utf8'), context);
  assert.deepEqual(Object.keys(context.TutorialModel), ['lessons', 'normalize', 'advance']);
  assert.equal(context.TutorialModel.lessons.length, 9);
  assert.equal(vm.runInContext('TutorialModel.advance(null,{type:"start"}).started', context), true);
});

test('corrupted values and future or missing schema versions recover to a fresh preference', () => {
  for (const raw of [undefined, null, false, 7, 'broken JSON', [], {done: ids},
    {version: '1', done: ids}, {version: 2, done: ids, started: true}]) {
    assert.deepEqual(model.normalize(raw), initial());
  }
  assert.deepEqual(model.normalize({version: 1, done: 'all', skipped: {}, current: 'missing', started: 'true'}), initial());
});

test('normalization deduplicates known IDs in lesson order and completion wins overlap', () => {
  const progress = model.normalize({version: 1, done: [ids[2], ids[0], ids[2], '__proto__', null],
    skipped: [ids[0], ids[1], 'constructor'], current: 'unknown', started: true, campaign: {money: 10}});
  assert.deepEqual(progress, {version: 1, done: [ids[0], ids[2]], skipped: [ids[1]], current: ids[3], started: true});
  assert.equal(Object.hasOwn(progress, 'campaign'), false);
  assert.equal(model.normalize({...progress, current: ids[0]}).current, ids[0], 'Replay selection survives storage');
});

test('prototype pollution, inherited fields and getters cannot supply progress or events', () => {
  const inherited = Object.create({version: 1, done: ids, started: true});
  assert.deepEqual(model.normalize(inherited), initial());
  const hostile = JSON.parse('{"version":1,"done":["__proto__","constructor"],"skipped":["prototype"],"__proto__":{"started":true}}');
  assert.deepEqual(model.normalize(hostile), initial());
  assert.equal({}.started, undefined);
  let reads = 0;
  const getter = () => { reads++; throw new Error('Getter must not run'); };
  const data = {version: 1, started: true};
  Object.defineProperty(data, 'done', {get: getter});
  const sparse = [];
  Object.defineProperty(sparse, '0', {get: getter});
  Object.defineProperty(sparse, '1', {value: ids[1]});
  data.skipped = sparse;
  assert.deepEqual(model.normalize(data).skipped, [ids[1]]);
  const event = {};
  Object.defineProperty(event, 'type', {get: getter});
  assert.deepEqual(model.advance(null, event), initial());
  assert.deepEqual(model.advance(null, Object.create({type: 'complete', id: ids[0]})), initial());
  assert.equal(reads, 0);
  assert.deepEqual(model.normalize(Object.assign(Object.create(null), initial())), initial());
  const broken = new Proxy({}, {getPrototypeOf() { throw new Error('bad input'); }});
  assert.deepEqual(model.normalize(broken), initial());
  assert.deepEqual(model.advance(null, broken), initial());
});

test('large sparse arrays recover within bounded work and do not accept inherited entries', () => {
  const sparse = Array(2 ** 32 - 1);
  sparse[0] = ids[0];
  Object.setPrototypeOf(sparse, {1: ids[1]});
  assert.deepEqual(model.normalize({version: 1, done: sparse}).done, [ids[0]]);
});

test('start and selecting a lesson never complete it; unknown events and IDs are no-ops', () => {
  const started = model.advance(null, {type: 'start'});
  assert.deepEqual(started, {...initial(), started: true});
  const selected = model.advance(started, {type: 'select', id: ids[4]});
  assert.equal(selected.current, ids[4]);
  assert.deepEqual(selected.done, []);
  assert.deepEqual(selected.skipped, []);
  for (const event of [null, 4, 'complete', {type: 'complete'}, {type: 'complete', id: 'unknown'},
    {type: 'complete', id: '__proto__'}, {type: 'game_won', id: ids[0]}]) {
    assert.deepEqual(model.advance(selected, event), selected);
  }
});

test('completion and skipping are separate, navigate to unresolved lessons and wrap', () => {
  let progress = model.advance(null, {type: 'complete', id: ids[0]});
  assert.deepEqual(progress.done, [ids[0]]);
  assert.equal(progress.current, ids[1]);
  progress = model.advance(progress, {type: 'skip', id: ids[1]});
  assert.deepEqual(progress.skipped, [ids[1]]);
  assert.deepEqual(progress.done, [ids[0]]);
  assert.equal(progress.current, ids[2]);
  progress = model.advance(progress, {type: 'complete', id: ids[8]});
  assert.equal(progress.current, ids[2], 'An out-of-order last lesson wraps to the first unresolved chapter');
  progress = model.advance(progress, {type: 'complete', id: ids[1]});
  assert.deepEqual(progress.skipped, []);
  assert.deepEqual(progress.done, [ids[0], ids[1], ids[8]]);
  progress = model.advance(progress, {type: 'skip', id: ids[0]});
  assert.deepEqual(progress.skipped, [], 'Skipping a replay must not erase a prior completion');
  assert.ok(progress.done.includes(ids[0]));
});

test('end of the reading list preserves skipped status and supports replay and restart', () => {
  let progress = model.normalize(null);
  for (let i = 0; i < ids.length; i++) progress = model.advance(progress, {type: i === 3 ? 'skip' : 'complete', id: ids[i]});
  assert.equal(progress.current, null);
  assert.equal(progress.done.length, 8);
  assert.deepEqual(progress.skipped, [ids[3]]);
  assert.deepEqual(model.normalize(progress), progress);
  assert.equal(model.advance(progress, {type: 'start'}).current, null);
  progress = model.advance(progress, {type: 'select', id: ids[3]});
  assert.equal(progress.current, ids[3]);
  assert.equal(progress.done.length, 8);
  progress = model.advance(progress, {type: 'complete', id: ids[3]});
  assert.equal(progress.current, null);
  assert.deepEqual(progress.done, ids);
  assert.deepEqual(progress.skipped, []);
  assert.deepEqual(model.advance(progress, {type: 'restart'}), {...initial(), started: true});
});

test('normalization and every event are immutable and do not retain caller arrays', () => {
  const input = Object.freeze({version: 1, done: Object.freeze([ids[0]]), skipped: Object.freeze([ids[1]]), current: ids[2], started: true});
  const before = JSON.stringify(input);
  for (const event of [{type: 'start'}, {type: 'select', id: ids[0]}, {type: 'complete', id: ids[2]},
    {type: 'skip', id: ids[2]}, {type: 'restart'}, {type: 'unknown'}]) {
    const frozenEvent = Object.freeze(event);
    const output = model.advance(input, frozenEvent);
    assert.notEqual(output, input);
    assert.notEqual(output.done, input.done);
    assert.notEqual(output.skipped, input.skipped);
    output.done.push('outside');
    output.skipped.push('outside');
    assert.equal(JSON.stringify(input), before);
  }
  const normalized = model.normalize(input);
  normalized.done.length = 0;
  assert.equal(JSON.stringify(input), before);
});
