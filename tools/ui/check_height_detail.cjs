// Run: node --test tools/ui/check_height_detail.cjs
// Execute the real optional-height loader with a stateful GL recorder. Evaluate
// its actual scalar shader expressions; browser QA verifies GPU compilation.
const assert = require('node:assert/strict');
const fs = require('node:fs');
const path = require('node:path');
const vm = require('node:vm');
const {test} = require('node:test');
const ui = path.resolve(__dirname, '../../spheres-web/ui');
const moduleSource = fs.readFileSync(path.join(ui, 'height-detail.js'), 'utf8');
const page = fs.readFileSync(path.join(ui, 'index.html'), 'utf8');
const shader = /const GLSL_MAP = `([\s\S]*?)`;/.exec(page)[1];
function fixture(options = {}) {
  const warnings = [], calls = [], allocated = new Set(), deleted = new Set(), params = new Map(), enabled = new Set();
  const sandbox = vm.createContext({window: {}, console: {warn(...args) {warnings.push(args.join(' '));}}});
  vm.runInContext(moduleSource, sandbox, {filename: 'height-detail.js'});
  const api = sandbox.window.HeightDetail;
  const constants = ['MAX_TEXTURE_SIZE', 'MAX_VIEWPORT_DIMS', 'ACTIVE_TEXTURE', 'CURRENT_PROGRAM', 'DRAW_FRAMEBUFFER_BINDING', 'READ_FRAMEBUFFER_BINDING',
    'VIEWPORT', 'COLOR_WRITEMASK', 'BLEND', 'DEPTH_TEST', 'SCISSOR_TEST', 'CULL_FACE', 'RASTERIZER_DISCARD', 'TEXTURE_BINDING_2D',
    'UNPACK_COLORSPACE_CONVERSION_WEBGL', 'UNPACK_PREMULTIPLY_ALPHA_WEBGL', 'UNPACK_FLIP_Y_WEBGL', 'UNPACK_ALIGNMENT',
    'TEXTURE0', 'TEXTURE2', 'TEXTURE_2D', 'RGB8', 'RGB', 'NEAREST', 'R16F', 'TEXTURE_WRAP_S', 'TEXTURE_WRAP_T',
    'CLAMP_TO_EDGE', 'TEXTURE_MIN_FILTER', 'TEXTURE_MAG_FILTER', 'LINEAR', 'LINEAR_MIPMAP_LINEAR',
    'FRAMEBUFFER', 'DRAW_FRAMEBUFFER', 'READ_FRAMEBUFFER', 'COLOR_ATTACHMENT0', 'FRAMEBUFFER_COMPLETE', 'TRIANGLES', 'NO_ERROR'];
  const gl = Object.fromEntries(constants.map(name => [name, name]));
  const oldProgram = {original: 'program'}, oldDraw = {original: 'drawFramebuffer'}, oldRead = {original: 'readFramebuffer'};
  const textures = new Map([[gl.TEXTURE0, {original: 'texture0'}], [gl.TEXTURE2, {original: 'texture2'}]]);
  params.set(gl.MAX_TEXTURE_SIZE, options.maxTexture || 8192);
  params.set(gl.MAX_VIEWPORT_DIMS, options.maxViewport || [8192, 8192]);
  params.set(gl.ACTIVE_TEXTURE, gl.TEXTURE2); params.set(gl.CURRENT_PROGRAM, oldProgram);
  params.set(gl.DRAW_FRAMEBUFFER_BINDING, oldDraw); params.set(gl.READ_FRAMEBUFFER_BINDING, oldRead);
  params.set(gl.VIEWPORT, [12, 14, 1000, 800]); params.set(gl.COLOR_WRITEMASK, [false, true, false, true]);
  params.set(gl.UNPACK_COLORSPACE_CONVERSION_WEBGL, 'BROWSER_DEFAULT'); params.set(gl.UNPACK_PREMULTIPLY_ALPHA_WEBGL, true);
  params.set(gl.UNPACK_FLIP_Y_WEBGL, true); params.set(gl.UNPACK_ALIGNMENT, 4);
  for (const key of [gl.BLEND, gl.SCISSOR_TEST, gl.RASTERIZER_DISCARD]) enabled.add(key);
  const initialParams = new Map([...params].map(([key, value]) => [key, Array.isArray(value) ? [...value] : value]));
  const initialTextures = new Map(textures), initialEnabled = new Set(enabled);
  let nextId = 1, error = gl.NO_ERROR, bitmapClosed = 0;
  const resource = kind => {const value = {kind, id: nextId++}; allocated.add(value); calls.push({op: 'create', value}); return value;};
  Object.assign(gl, {
    getParameter(key) {return key === gl.TEXTURE_BINDING_2D ? textures.get(params.get(gl.ACTIVE_TEXTURE)) : params.get(key);},
    activeTexture(unit) {params.set(gl.ACTIVE_TEXTURE, unit);},
    bindTexture(target, texture) {assert.equal(target, gl.TEXTURE_2D); textures.set(params.get(gl.ACTIVE_TEXTURE), texture);},
    bindFramebuffer(target, framebuffer) {
      if (target !== gl.READ_FRAMEBUFFER) params.set(gl.DRAW_FRAMEBUFFER_BINDING, framebuffer);
      if (target !== gl.DRAW_FRAMEBUFFER) params.set(gl.READ_FRAMEBUFFER_BINDING, framebuffer);
    },
    viewport(...values) {params.set(gl.VIEWPORT, values);},
    colorMask(...values) {params.set(gl.COLOR_WRITEMASK, values);},
    isEnabled(key) {return enabled.has(key);}, enable(key) {enabled.add(key);}, disable(key) {enabled.delete(key);},
    pixelStorei(key, value) {params.set(key, value);},
    useProgram(program) {params.set(gl.CURRENT_PROGRAM, program);},
    createTexture() {return resource('texture');}, createFramebuffer() {return options.allocationFailure ? null : resource('framebuffer');},
    deleteTexture(value) {assert(!deleted.has(value), 'texture deleted only once'); deleted.add(value);},
    deleteFramebuffer(value) {if (value === null) return; assert(!deleted.has(value), 'framebuffer deleted only once'); deleted.add(value);},
    deleteProgram(value) {assert(!deleted.has(value), 'program deleted only once'); deleted.add(value);},
    texStorage2D(target, levels, format, width, height) {calls.push({op: 'storage', target, levels, format, width, height, texture: gl.getParameter(gl.TEXTURE_BINDING_2D)});},
    texParameteri(target, key, value) {calls.push({op: 'parameter', target, key, value, texture: gl.getParameter(gl.TEXTURE_BINDING_2D)});},
    framebufferTexture2D(target, attachment, kind, texture, level) {calls.push({op: 'attach', target, attachment, kind, texture, level});},
    checkFramebufferStatus() {return options.framebufferFailure ? 'INCOMPLETE_ATTACHMENT' : gl.FRAMEBUFFER_COMPLETE;},
    getUniformLocation(program, name) {return {program, name};},
    uniform1i(location, value) {calls.push({op: 'uniform', location, value});},
    drawArrays(mode, first, count) {
      calls.push({op: 'draw', mode, first, count, viewport: gl.getParameter(gl.VIEWPORT), source: textures.get(gl.TEXTURE0),
        colorMask: gl.getParameter(gl.COLOR_WRITEMASK), capabilities: [...enabled]});
      if (options.drawFailure) error = 'INVALID_OPERATION';
    },
    generateMipmap(target) {calls.push({op: 'mipmap', target, texture: gl.getParameter(gl.TEXTURE_BINDING_2D)}); if (options.mipmapFailure) error = 'INVALID_OPERATION';},
    getError() {const value = error; error = gl.NO_ERROR; return value;},
  });
  const helpers = {
    async bitmap(url) {
      calls.push({op: 'bitmap', url}); if (options.fetchFailure) throw new Error('HTTP 404');
      return {width: options.width || api.width, height: options.height || api.height, close() {bitmapClosed += 1;}};
    },
    program(incoming, source, label) {
      assert.equal(incoming, gl); calls.push({op: 'program', source, label});
      if (options.programFailure) throw new Error('shader compilation failed');
      return resource('program');
    },
    upload(incoming, bitmap, internal, format, filter) {
      assert.equal(incoming, gl); const texture = gl.createTexture();
      gl.bindTexture(gl.TEXTURE_2D, texture);
      gl.pixelStorei(gl.UNPACK_COLORSPACE_CONVERSION_WEBGL, 'NONE'); gl.pixelStorei(gl.UNPACK_PREMULTIPLY_ALPHA_WEBGL, false);
      gl.pixelStorei(gl.UNPACK_FLIP_Y_WEBGL, false); gl.pixelStorei(gl.UNPACK_ALIGNMENT, 1);
      calls.push({op: 'upload', bitmap, internal, format, filter, texture}); return texture;
    },
  };
  return {api, gl, helpers, calls, warnings, allocated, deleted, bitmapClosed: () => bitmapClosed,
    assertRestored() {
      for (const [key, value] of initialParams) assert.deepEqual(params.get(key), value, key + ' restored');
      assert.deepEqual(textures, initialTextures); assert.deepEqual(enabled, initialEnabled);
    },
  };
}
const math = {clamp: (x, low, high) => Math.max(low, Math.min(high, x)),
  mix: (a, b, t) => a + (b - a) * t, log2: Math.log2, max: Math.max, min: Math.min, exp2: n => 2 ** n,
  smoothstep: (low, high, x) => {const t = Math.max(0, Math.min(1, (x - low) / (high - low))); return t * t * (3 - 2 * t);}};
function expression(name) {
  const value = new RegExp(`float\\s+${name}\\s*=\\s*([^;]+);`).exec(shader)?.[1];
  assert(value, name + ' exists in actual shader'); return value;
}
function sample(options = {}) {
  const scope = {...math, uHeightEnabled: 1, uTerrainView: 1, uLk: Math.log2(8), uTexPerWorld: 1, uPxPerWorld: 4,
    uMaxLod: 7, uHeightMaxLod: 8, tRelief: 1, ...options};
  const context = vm.createContext(scope);
  for (const key of ['texPerPx', 'macroBlur', 'L', 'heightMix', 'heightL', 'st', 'heightSt']) scope[key] = vm.runInContext(expression(key), context);
  return scope;
}

test('packed elevation decodes exact byte order, endpoints and high-byte rollover', () => {
  const {api} = fixture();
  assert.equal(api.low, -1500); assert.equal(api.high, 9000);
  assert.match(api.decodeShader, /texelFetch\(uEncoded, ivec2\(gl_FragCoord\.xy\), 0\)\.rg/);
  assert.match(api.decodeShader, /floor\([^;]*\* 255\.0 \+ 0\.5\)/);
  const rhs = /heightMetres\s*=\s*([^;]+);/.exec(api.decodeShader)[1];
  const decode = bytes => vm.runInNewContext(rhs, {bytes, vec2: (...x) => x, dot: (a, b) => a.reduce((sum, x, i) => sum + x * b[i], 0)});
  assert.equal(decode([0, 0]), -1500); assert.equal(decode([255, 255]), 9000);
  const step = 10500 / 65535;
  assert(Math.abs((decode([1, 0]) - decode([0, 255])) - step) < 1e-10);
  for (const encoded of [1, 255, 256, 32768, 60000, 65534]) {
    assert.equal(decode([encoded >> 8, encoded & 255]), -1500 + encoded * step);
  }
});

test('4800x2036 detail uploads unfiltered packed RGB then stores one R16F channel with mips', async () => {
  const f = fixture(), result = await f.api.create(f.gl, f.helpers);
  assert(result); assert.equal(result.scale, 2); assert.equal(result.maxLod, 8);
  const upload = f.calls.find(call => call.op === 'upload'), storage = f.calls.find(call => call.op === 'storage');
  assert.equal(upload.internal, f.gl.RGB8); assert.equal(upload.format, f.gl.RGB); assert.equal(upload.filter, f.gl.NEAREST);
  assert.deepEqual([storage.width, storage.height, storage.levels, storage.format], [4800, 2036, 13, f.gl.R16F]);
  assert.equal(storage.texture, result.texture);
  let texels = 0;
  for (let level = 0; level < storage.levels; level++) texels += Math.max(1, storage.width >> level) * Math.max(1, storage.height >> level);
  assert(texels * 2 < 27 * 1024 * 1024, 'single half-float channel plus full mip tail fits under 27 MiB');
  assert(f.calls.some(call => call.op === 'parameter' && call.texture === result.texture && call.key === f.gl.TEXTURE_MIN_FILTER && call.value === f.gl.LINEAR_MIPMAP_LINEAR));
  assert.equal(f.bitmapClosed(), 1); assert.equal(f.warnings.length, 0); f.assertRestored();
  assert.deepEqual([...f.allocated].filter(value => !f.deleted.has(value)), [result.texture], 'only the returned texture survives');
});

test('decode covers its own viewport with blending/scissor discarded and restores caller GPU state', async () => {
  const f = fixture(); await f.api.create(f.gl, f.helpers);
  const draw = f.calls.find(call => call.op === 'draw');
  assert.deepEqual(draw.viewport, [0, 0, 4800, 2036]); assert.deepEqual(draw.colorMask, [true, true, true, true]);
  assert.deepEqual(draw.capabilities, []); assert.equal(draw.mode, f.gl.TRIANGLES); assert.equal(draw.count, 3);
  assert.equal(draw.source, f.calls.find(call => call.op === 'upload').texture); f.assertRestored();
});

test('unsupported texture or viewport dimensions fall back before fetching or allocating', async () => {
  for (const options of [{maxTexture: 4096}, {maxViewport: [4096, 8192]}, {maxViewport: [8192, 1024]}]) {
    const f = fixture(options); assert.equal(await f.api.create(f.gl, f.helpers), null);
    assert.equal(f.calls.length, 0); assert.equal(f.allocated.size, 0); f.assertRestored();
  }
});

test('dimension mismatch closes the bitmap without touching existing GPU textures', async () => {
  for (const options of [{width: 2400}, {height: 1018}]) {
    const f = fixture(options); assert.equal(await f.api.create(f.gl, f.helpers), null);
    assert.equal(f.allocated.size, 0); assert.equal(f.bitmapClosed(), 1); assert.match(f.warnings[0], /dimensions disagree/); f.assertRestored();
  }
});

test('fetch or shader failure uses base relief with no leaked resources', async () => {
  for (const options of [{fetchFailure: true}, {programFailure: true}]) {
    const f = fixture(options); assert.equal(await f.api.create(f.gl, f.helpers), null);
    assert([...f.allocated].every(value => f.deleted.has(value))); assert.equal(f.warnings.length, 1); f.assertRestored();
  }
});

test('incomplete float framebuffer falls back and deletes every partial allocation', async () => {
  const f = fixture({framebufferFailure: true}); assert.equal(await f.api.create(f.gl, f.helpers), null);
  assert([...f.allocated].every(value => f.deleted.has(value))); assert(!f.calls.some(call => call.op === 'draw'));
  assert.match(f.warnings[0], /framebuffer unavailable/); assert.equal(f.bitmapClosed(), 1); f.assertRestored();
});

test('failed framebuffer allocation cannot accidentally draw height bytes into the main canvas', async () => {
  const f = fixture({allocationFailure: true}); assert.equal(await f.api.create(f.gl, f.helpers), null);
  assert(!f.calls.some(call => call.op === 'draw')); assert([...f.allocated].every(value => f.deleted.has(value)));
  assert.match(f.warnings[0], /allocation unavailable/); f.assertRestored();
});

test('decode errors are checked before mipmaps instead of accepting a blank elevation texture', async () => {
  const f = fixture({drawFailure: true}); assert.equal(await f.api.create(f.gl, f.helpers), null);
  assert(!f.calls.some(call => call.op === 'mipmap')); assert([...f.allocated].every(value => f.deleted.has(value)));
  assert.match(f.warnings[0], /decode unavailable/); f.assertRestored();
});

test('half-float mipmap failure discards optional detail and restores all caller bindings', async () => {
  const f = fixture({mipmapFailure: true}); assert.equal(await f.api.create(f.gl, f.helpers), null);
  assert.equal(f.calls.filter(call => call.op === 'mipmap').length, 1);
  assert([...f.allocated].every(value => f.deleted.has(value))); assert.match(f.warnings[0], /mipmaps unavailable/); f.assertRestored();
});

test('actual shader keeps detail disabled outside Terrain or when optional GPU loading failed', () => {
  for (const uLk of [0, .65, 1, 1.65, 3, Math.log2(48)]) {
    assert.equal(sample({uTerrainView: 0, uLk}).heightMix, 0);
    assert.equal(sample({uHeightEnabled: 0, uLk}).heightMix, 0);
  }
  assert.equal(sample({uLk: .65}).heightMix, 0); assert.equal(sample({uLk: 1.65}).heightMix, 1);
  assert(Math.abs(sample({uLk: 1.15}).heightMix - .5) < 1e-12);
  assert.match(page, /uniform1f\(u\.terrainView, ui\.mapMode === "terrain" \? 1 : 0\)/);
});

test('height samples get their own half-sized minimum step without changing coast or water sampling', () => {
  const zoomed = sample({uPxPerWorld: 20}); assert.equal(zoomed.st, 1); assert.equal(zoomed.heightSt, .5);
  for (const uPxPerWorld of [.05, .25, 1, 4, 48]) {
    const on = sample({uPxPerWorld}), off = sample({uPxPerWorld, uHeightEnabled: 0});
    assert.equal(on.st, off.st); assert.equal(on.L, off.L);
    assert(on.heightL >= 0 && on.heightL <= 8); assert(on.heightSt > 0);
  }
  assert(!/\b(?:C|E|Wt|S|N)\.[rgba]\s*=/.test(shader), 'base physics channels are never overwritten by optional height');
  assert.match(shader, /float dep = max\(C\.g, 0\.0\)/); assert.match(shader, /float sdf = C\.b/);
  assert.match(shader, /float V\s*= clamp\(C\.a/);
  assert.match(shader, /heightMix > 0\.0 && isLand > 0\.0/);
  assert.match(shader, /lakeAt\(dE, L\).*lakeAt\(dW, L\).*lakeAt\(dS, L\).*lakeAt\(dN, L\)/);
  assert.match(shader, /hTap\(textureLod\(uHeightDetail, dE, heightL\)\.r, lakeHeights\.x, heightCenter, aaL\)/);
});

test('detail texture has its own sampler unit and fallback never replaces base physics', () => {
  assert.match(page, /uniform1i\(u\.phys, 0\)/);
  assert.match(page, /uniform1i\(u\.heightDetail, 6\)/);
  assert.match(page, /activeTexture\(gl\.TEXTURE0\); gl\.bindTexture\(gl\.TEXTURE_2D, GLR\.phys\)/);
  assert.match(page, /activeTexture\(gl\.TEXTURE6\); gl\.bindTexture\(gl\.TEXTURE_2D, GLR\.heightDetail\?\.texture \|\| GLR\.phys\)/);
  const enabled = /uniform1f\(u\.heightEnabled, ([^;]+)\);/.exec(page)?.[1];
  assert(enabled, 'actual optional detail uniform is wired');
  for (const available of [false, true]) {
    for (const setting of [undefined, false, true]) {
      const result = vm.runInNewContext(enabled, {GLR: {heightDetail: available ? {texture: {}} : null}, ui: {mapDetails: {relief: setting}}});
      assert.equal(result, available && setting !== false ? 1 : 0, 'capability and user toggle both control detail');
    }
  }
});
