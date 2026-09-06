/* Regional terrain is real displaced geometry, sampled from the bundled NOAA
   ETOPO elevation tiles. It is a visual layer: it never reads or changes the
   simulation. The ordinary globe remains visible while local tiles load. */
(function () {
  "use strict";
  const DEG = Math.PI / 180, EARTH_METRES = 6371000;
  const TILE_DEGREES = 10, TILE_CELLS = 600, TILE_SIZE = 602;
  const SAMPLE_DEGREES = TILE_DEGREES / TILE_CELLS;
  const HEIGHT_LOW = -1500, HEIGHT_RANGE = 10500;
  // [existing RIVERS.lakes index, HydroLAKES water metres, path FNV-1a]. These
  // are the six ETOPO bed lakes, not new lake outlines. Reproducible source,
  // exact HydroLAKES records and CC-BY attribution are in lake-surfaces.json.
  const BED_LAKES = Object.freeze([[1, 449, 4049581666], [4, 172, 2943202480],
    [9, 175, 1824147311], [14, 175, 4191224988], [20, 73, 2319471044], [23, 179, 575582281]].map(Object.freeze));
  const IDENTITY = new Float32Array([1, 0, 0, 0, 1, 0, 0, 0, 1]);
  const clamp = (x, lo, hi) => Math.max(lo, Math.min(hi, x));
  const wrap = x => ((x + 180) % 360 + 360) % 360 - 180;
  const vertexSource = `#version 300 es
precision highp float;
layout(location=0) in vec3 aSurface;
layout(location=1) in float aElevation;
layout(location=2) in vec2 aSlope;
uniform mat3 uInvRot;
uniform vec3 uCamera;
uniform mat3 uRayBasis;
uniform vec2 uHalf;
out vec3 vSurface;
out float vElevation;
out vec2 vSlope;
void main() {
  vSurface = aSurface;
  vElevation = aElevation;
  vSlope = aSlope;
  vec3 local = transpose(uInvRot) * aSurface;
  vec3 eye = transpose(uRayBasis) * (local - uCamera);
  const float nearPlane = 0.001;
  const float farPlane = 10.0;
  float A = (farPlane + nearPlane) / (nearPlane - farPlane);
  float B = 2.0 * farPlane * nearPlane / (nearPlane - farPlane);
  gl_Position = vec4(eye.xy / uHalf, A * eye.z + B, -eye.z);
}`;

  function multiply(matrix, p) {
    return [matrix[0] * p[0] + matrix[3] * p[1] + matrix[6] * p[2],
      matrix[1] * p[0] + matrix[4] * p[1] + matrix[7] * p[2],
      matrix[2] * p[0] + matrix[5] * p[1] + matrix[8] * p[2]];
  }
  function tileAt(longitude, latitude) {
    const lon = wrap(longitude), lat = clamp(latitude, -90, 90);
    const x = clamp(Math.floor((lon + 180) / TILE_DEGREES), 0, 35);
    const y = clamp(Math.floor((90 - lat) / TILE_DEGREES), 0, 17);
    return {x, y, key: x + ":" + y,
      // Raster cells are centred half a sample inside each tile. The gutter
      // is another cell centre outside it, so the boundary lies at pixel .5.
      px: .5 + ((lon + 180) / TILE_DEGREES - x) * TILE_CELLS,
      py: .5 + ((90 - lat) / TILE_DEGREES - y) * TILE_CELLS};
  }
  function tilePath(x, y) {
    return "/terrain-tiles/x" + String(x).padStart(2, "0") + "_y" + String(y).padStart(2, "0") + ".png";
  }
  function decodePixels(pixels, width = TILE_SIZE, height = TILE_SIZE) {
    if (width !== TILE_SIZE || height !== TILE_SIZE || pixels.length !== width * height * 4)
      throw new Error("Regional elevation tile dimensions disagree with the bake");
    const heights = new Float32Array(width * height);
    for (let i = 0, p = 0; i < heights.length; i++, p += 4)
      heights[i] = HEIGHT_LOW + (pixels[p] * 256 + pixels[p + 1]) * HEIGHT_RANGE / 65535;
    return heights;
  }
  function sampleTile(heights, px, py) {
    const x = clamp(px, 0, TILE_SIZE - 1), y = clamp(py, 0, TILE_SIZE - 1);
    const ix = Math.min(TILE_SIZE - 2, Math.floor(x)), iy = Math.min(TILE_SIZE - 2, Math.floor(y));
    const fx = x - ix, fy = y - iy, offset = iy * TILE_SIZE + ix;
    const a = heights[offset] + (heights[offset + 1] - heights[offset]) * fx;
    const b = heights[offset + TILE_SIZE] + (heights[offset + TILE_SIZE + 1] - heights[offset + TILE_SIZE]) * fx;
    return a + (b - a) * fy;
  }
  function makeLakeSampler(rivers, globe) {
    if (!rivers?.lakes || !globe?.project || !globe?.unproject) return null;
    const lakes = [];
    for (const [index, level, expectedHash] of BED_LAKES) {
      const path = rivers.lakes[index];
      let hash = 2166136261;
      for (let i = 0; i < (path?.length || 0); i++) hash = Math.imul(hash ^ path.charCodeAt(i), 16777619) >>> 0;
      if (!path || hash !== expectedHash) {
        console.warn("[glmap] lake surface omitted because its shoreline changed:", index);
        continue;
      }
      const rings = path.split("M").filter(Boolean).map(chunk => {
        const values = chunk.match(/-?\d+(?:\.\d+)?/g).map(Number), points = [];
        for (let i = 0; i < values.length; i += 2) points.push([values[i], values[i + 1]]);
        return points;
      });
      const points = rings.flat(), geo = points.map(p => globe.unproject(p[0], p[1]));
      lakes.push({level, rings, west: Math.min(...geo.map(p => p[0])), east: Math.max(...geo.map(p => p[0])),
        south: Math.min(...geo.map(p => p[1])), north: Math.max(...geo.map(p => p[1]))});
    }
    function near(lon, lat, margin = SAMPLE_DEGREES) {
      return lakes.some(lake => lon >= lake.west - margin && lon <= lake.east + margin &&
        lat >= lake.south - margin && lat <= lake.north + margin);
    }
    function at(longitude, latitude) {
      const lon = wrap(longitude), lat = latitude;
      if (!near(lon, lat, 0)) return null;
      const [x, y] = globe.project(lon, lat);
      for (const lake of lakes) {
        if (lon < lake.west || lon > lake.east || lat < lake.south || lat > lake.north) continue;
        let inside = false;
        for (const ring of lake.rings) for (let i = 0, j = ring.length - 1; i < ring.length; j = i++) {
          const a = ring[i], b = ring[j];
          if ((a[1] > y) !== (b[1] > y) && x < (b[0] - a[0]) * (y - a[1]) / (b[1] - a[1]) + a[0]) inside = !inside;
        }
        if (inside) return lake.level;
      }
      return null;
    }
    return {at, near, count: lakes.length};
  }
  let manifestPromise;
  function readManifest() {
    if (!manifestPromise) manifestPromise = fetch("/terrain-tiles/manifest.json", {cache: "force-cache"}).then(async response => {
      if (!response.ok) throw new Error("Regional elevation manifest HTTP " + response.status);
      const manifest = await response.json(), grid = manifest.grid;
      if (!grid || grid.columns !== 36 || grid.rows !== 18 || grid.tile_degrees !== TILE_DEGREES ||
          grid.interior_pixels !== TILE_CELLS || grid.gutter_pixels !== 1 || grid.png_pixels !== TILE_SIZE ||
          grid.samples_per_degree !== 60 || !manifest.tiles)
        throw new Error("Regional elevation manifest disagrees with the renderer");
      return manifest;
    });
    return manifestPromise;
  }
  async function fetchTile(x, y, signal) {
    const path = tilePath(x, y), name = path.slice(path.lastIndexOf("/") + 1), key = name.slice(0, -4);
    const manifest = await readManifest(), entry = manifest.tiles[key];
    if (!entry || entry.x !== x || entry.y !== y) throw new Error("Regional elevation tile is missing from its manifest");
    if (entry.file === null && Number.isInteger(entry.constant_code) && entry.constant_code >= 0 && entry.constant_code <= 65535)
      return new Float32Array(TILE_SIZE * TILE_SIZE).fill(HEIGHT_LOW + entry.constant_code * HEIGHT_RANGE / 65535);
    if (entry.file !== name) throw new Error("Regional elevation manifest contains an unexpected tile path");
    const response = await fetch(path, {signal, cache: "force-cache"});
    if (!response.ok) throw new Error("Regional elevation HTTP " + response.status);
    const bitmap = await createImageBitmap(await response.blob(), {colorSpaceConversion: "none", premultiplyAlpha: "none"});
    try {
      if (bitmap.width !== TILE_SIZE || bitmap.height !== TILE_SIZE)
        throw new Error("Regional elevation tile dimensions disagree with the bake");
      const canvas = typeof OffscreenCanvas === "function" ? new OffscreenCanvas(TILE_SIZE, TILE_SIZE) : document.createElement("canvas");
      canvas.width = TILE_SIZE; canvas.height = TILE_SIZE;
      const context = canvas.getContext("2d", {willReadFrequently: true, colorSpace: "srgb"});
      if (!context) throw new Error("Regional elevation decoder unavailable");
      context.drawImage(bitmap, 0, 0);
      return decodePixels(context.getImageData(0, 0, TILE_SIZE, TILE_SIZE).data);
    } finally { bitmap.close(); }
  }

  // CPU copies are retained only for the current region and a bounded nearby
  // cache. PNG/GPU dimensions never grow with zoom. Six requests can be active;
  // a new view replaces the queue rather than loading every place dragged past.
  class TileCache {
    constructor(options = {}) {
      this.limit = clamp(options.cacheLimit || 64, 4, 64);
      this.concurrency = clamp(options.concurrency || 6, 1, 6);
      this.loadTile = options.loadTile || fetchTile;
      this.waterSampler = options.waterSampler || null;
      this.onChange = options.onChange || (() => {});
      this.tiles = new Map(); this.pending = new Map(); this.failed = new Set();
      this.wanted = new Map(); this.queue = []; this.disposed = false;
    }
    require(tiles) {
      if (this.disposed) return false;
      this.wanted = new Map(tiles.map(tile => [tile.key, tile]));
      this.queue = tiles.filter(tile => !this.tiles.has(tile.key) && !this.pending.has(tile.key) && !this.failed.has(tile.key));
      this.pump();
      return tiles.length <= this.limit && tiles.every(tile => this.tiles.has(tile.key));
    }
    pump() {
      while (!this.disposed && this.pending.size < this.concurrency && this.queue.length) {
        const tile = this.queue.shift(), controller = new AbortController();
        this.pending.set(tile.key, controller);
        Promise.resolve().then(() => this.loadTile(tile.x, tile.y, controller.signal)).then(heights => {
          if (this.disposed) return;
          if (!(heights instanceof Float32Array) || heights.length !== TILE_SIZE * TILE_SIZE)
            throw new Error("Regional elevation decoder returned an invalid tile");
          this.tiles.set(tile.key, heights);
          this.trim();
        }).catch(error => {
          if (this.disposed || error?.name === "AbortError") return;
          this.failed.add(tile.key);
          if (!this.warned) {
            this.warned = true;
            console.warn("[glmap] regional elevation unavailable; using globe relief:", error.message);
          }
        }).finally(() => {
          this.pending.delete(tile.key);
          if (!this.disposed) { this.pump(); this.onChange(); }
        });
      }
    }
    trim() {
      for (const key of this.tiles.keys()) {
        if (this.tiles.size <= this.limit) break;
        if (!this.wanted.has(key)) this.tiles.delete(key);
      }
      // Coverage larger than the budget is rejected by the surface before
      // loading, but preserve this guarantee for direct cache callers too.
      while (this.tiles.size > this.limit) this.tiles.delete(this.tiles.keys().next().value);
    }
    sample(longitude, latitude) {
      const tile = tileAt(longitude, latitude), heights = this.tiles.get(tile.key);
      return heights ? (this.waterSampler?.at(longitude, latitude) ?? sampleTile(heights, tile.px, tile.py)) : null;
    }
    sampleSurface(longitude, latitude) {
      const tile = tileAt(longitude, latitude), heights = this.tiles.get(tile.key);
      if (!heights) return null;
      const x = tile.px, y = tile.py;
      if (x >= 1 && x <= TILE_SIZE - 2 && y >= 1 && y <= TILE_SIZE - 2) {
        const values = [sampleTile(heights, x, y), sampleTile(heights, x + 1, y), sampleTile(heights, x - 1, y),
          sampleTile(heights, x, y + 1), sampleTile(heights, x, y - 1)];
        if (this.waterSampler?.near(longitude, latitude)) {
          const coordinates = [[longitude, latitude], [longitude + SAMPLE_DEGREES, latitude], [longitude - SAMPLE_DEGREES, latitude],
            [longitude, latitude - SAMPLE_DEGREES], [longitude, latitude + SAMPLE_DEGREES]];
          for (let i = 0; i < 5; i++) values[i] = this.waterSampler.at(...coordinates[i]) ?? values[i];
        }
        return values;
      }
      // The last half cell at a boundary needs the neighbouring tile for a
      // complete central difference. Everywhere else one lookup serves all
      // five samples, keeping mesh rebuilds out of the drag's hot path.
      return [this.waterSampler?.at(longitude, latitude) ?? sampleTile(heights, x, y), this.sample(longitude + SAMPLE_DEGREES, latitude),
        this.sample(longitude - SAMPLE_DEGREES, latitude), this.sample(longitude, latitude - SAMPLE_DEGREES),
        this.sample(longitude, latitude + SAMPLE_DEGREES)];
    }
    dispose() {
      this.disposed = true;
      for (const controller of this.pending.values()) controller.abort();
      this.pending.clear(); this.queue = []; this.wanted.clear(); this.tiles.clear(); this.failed.clear();
    }
  }

  function viewCamera(view) { return view.camera || [0, 0, view.distance]; }
  function viewHalf(view) { return view.half || [view.halfTan * view.aspect, view.halfTan]; }
  function footprint(view) {
    const camera = viewCamera(view), basis = view.rayBasis || IDENTITY, inverse = view.invBasis || IDENTITY;
    const half = viewHalf(view), anchor = wrap(-(view.yaw || 0) / DEG), points = [];
    function cast(x, y) {
      let ray = multiply(basis, [x * half[0], y * half[1], -1]);
      const length = Math.hypot(...ray); ray = ray.map(v => v / length);
      const b = camera[0] * ray[0] + camera[1] * ray[1] + camera[2] * ray[2];
      const c = camera[0] ** 2 + camera[1] ** 2 + camera[2] ** 2 - 1, disc = b * b - c;
      if (disc < 0) return null;
      const t = -b - Math.sqrt(disc); if (t <= 0) return null;
      const point = multiply(inverse, camera.map((v, i) => v + ray[i] * t));
      const lon = Math.atan2(point[0], point[2]) / DEG;
      return [anchor + wrap(lon - anchor), Math.asin(clamp(point[1], -1, 1)) / DEG];
    }
    // A tilted frame can cross the horizon. Merely ignoring missed corners
    // cuts the mesh off before the skyline, so refine every grid edge where a
    // ground hit turns into sky and include that silhouette in the footprint.
    const grid = [];
    for (let row = 0; row <= 4; row++) for (let col = 0; col <= 4; col++) {
      const p = {x: col / 2 - 1, y: row / 2 - 1}; p.hit = cast(p.x, p.y);
      grid.push(p); if (p.hit) points.push(p.hit);
    }
    for (let row = 0; row <= 4; row++) for (let col = 0; col <= 4; col++) {
      const a = grid[row * 5 + col];
      for (const b of [col < 4 ? grid[row * 5 + col + 1] : null, row < 4 ? grid[(row + 1) * 5 + col] : null]) {
        if (!b || Boolean(a.hit) === Boolean(b.hit)) continue;
        let hit = a.hit ? a : b, miss = a.hit ? b : a;
        for (let i = 0; i < 18; i++) {
          const mid = {x: (hit.x + miss.x) / 2, y: (hit.y + miss.y) / 2}; mid.hit = cast(mid.x, mid.y);
          if (mid.hit) hit = mid; else miss = mid;
        }
        points.push(hit.hit);
      }
    }
    if (!points.length) return null;
    return {west: Math.min(...points.map(p => p[0])), east: Math.max(...points.map(p => p[0])),
      south: Math.min(...points.map(p => p[1])), north: Math.max(...points.map(p => p[1])), anchor};
  }
  function paddedBounds(box, exaggeration = 3) {
    const latPad = Math.max((box.north - box.south) * .16, 9000 * exaggeration / EARTH_METRES / DEG * 1.8);
    const lonPad = Math.max((box.east - box.west) * .16, latPad / Math.max(.12, Math.cos((box.north + box.south) * .5 * DEG)));
    return {west: box.west - lonPad, east: box.east + lonPad,
      south: clamp(box.south - latPad, -89.98, 89.98), north: clamp(box.north + latPad, -89.98, 89.98), anchor: box.anchor};
  }
  function tilesForBounds(box) {
    const margin = SAMPLE_DEGREES * 1.05, tiles = new Map();
    const firstX = Math.floor((box.west - margin + 180) / TILE_DEGREES);
    const lastX = Math.floor((box.east + margin + 180) / TILE_DEGREES);
    const firstY = clamp(Math.floor((90 - box.north - margin) / TILE_DEGREES), 0, 17);
    const lastY = clamp(Math.floor((90 - box.south + margin) / TILE_DEGREES), 0, 17);
    for (let y = firstY; y <= lastY; y++) for (let tx = firstX; tx <= lastX; tx++) {
      const x = ((tx % 36) + 36) % 36, key = x + ":" + y;
      tiles.set(key, {x, y, key});
    }
    const centerLon = (box.west + box.east) / 2, centerLat = (box.north + box.south) / 2;
    const distance = tile => wrap(-175 + tile.x * 10 - centerLon) ** 2 + (85 - tile.y * 10 - centerLat) ** 2;
    return [...tiles.values()].sort((a, b) => distance(a) - distance(b) || a.y - b.y || a.x - b.x);
  }
  function contains(outer, inner) {
    const shift = Math.round((outer.anchor - inner.anchor) / 360) * 360;
    return outer.west <= inner.west + shift && outer.east >= inner.east + shift &&
      outer.south <= inner.south && outer.north >= inner.north;
  }
  function buildMesh(bounds, sample, options = {}) {
    const maxSegments = clamp(options.maxSegments || 256, 16, 384), exaggeration = options.exaggeration ?? 3;
    const columns = clamp(Math.ceil((bounds.east - bounds.west) / SAMPLE_DEGREES), 16, maxSegments);
    const rows = clamp(Math.ceil((bounds.north - bounds.south) / SAMPLE_DEGREES), 16, maxSegments);
    const vertices = new Float32Array((columns + 1) * (rows + 1) * 6);
    const indices = new Uint32Array(columns * rows * 6), stepMetres = SAMPLE_DEGREES * DEG * EARTH_METRES;
    let v = 0, index = 0, minHeight = Infinity, maxHeight = -Infinity;
    for (let row = 0; row <= rows; row++) {
      const latitude = bounds.north + (bounds.south - bounds.north) * row / rows;
      const lat = latitude * DEG, cosLat = Math.cos(lat), sinLat = Math.sin(lat);
      for (let col = 0; col <= columns; col++) {
        const longitude = bounds.west + (bounds.east - bounds.west) * col / columns, lon = longitude * DEG;
        const values = options.sampleSurface ? options.sampleSurface(longitude, latitude) :
          [sample(longitude, latitude), sample(longitude + SAMPLE_DEGREES, latitude), sample(longitude - SAMPLE_DEGREES, latitude),
            sample(longitude, latitude - SAMPLE_DEGREES), sample(longitude, latitude + SAMPLE_DEGREES)];
        if (!values || !values.every(Number.isFinite)) return null;
        const [height, e, w, s, n] = values;
        const r = 1 + Math.max(0, height) * exaggeration / EARTH_METRES;
        vertices[v++] = r * cosLat * Math.sin(lon);
        vertices[v++] = r * sinLat;
        vertices[v++] = r * cosLat * Math.cos(lon);
        vertices[v++] = height;
        vertices[v++] = (Math.max(0, e) - Math.max(0, w)) / (2 * stepMetres * Math.max(.001, cosLat));
        vertices[v++] = (Math.max(0, s) - Math.max(0, n)) / (2 * stepMetres);
        minHeight = Math.min(minHeight, height); maxHeight = Math.max(maxHeight, height);
        if (row < rows && col < columns) {
          const a = row * (columns + 1) + col, b = a + 1, c = a + columns + 1, d = c + 1;
          indices[index++] = a; indices[index++] = c; indices[index++] = b;
          indices[index++] = b; indices[index++] = c; indices[index++] = d;
        }
      }
    }
    return {vertices, indices, columns, rows, bounds, minHeight, maxHeight, exaggeration};
  }

  function create(gl, options = {}) {
    let disposed = false, failed = false, ready = false, dirty = true, mesh = null, lastView = null;
    let vao = null, vertexBuffer = null, indexBuffer = null, previousScale = 0;
    const uniforms = new WeakMap(), minZoom = options.minZoom || 12;
    const cache = new TileCache({...options, waterSampler: options.waterSampler || makeLakeSampler(window.RIVERS, window.Globe3D),
      onChange() { dirty = true; if (!disposed) options.onChange?.(); }});
    function upload(next) {
      const oldVAO = gl.getParameter(gl.VERTEX_ARRAY_BINDING), oldArray = gl.getParameter(gl.ARRAY_BUFFER_BINDING);
      try {
        vao ||= gl.createVertexArray(); vertexBuffer ||= gl.createBuffer(); indexBuffer ||= gl.createBuffer();
        if (!vao || !vertexBuffer || !indexBuffer) throw new Error("Regional terrain buffer allocation unavailable");
        gl.bindVertexArray(vao);
        gl.bindBuffer(gl.ARRAY_BUFFER, vertexBuffer); gl.bufferData(gl.ARRAY_BUFFER, next.vertices, gl.DYNAMIC_DRAW);
        gl.enableVertexAttribArray(0); gl.vertexAttribPointer(0, 3, gl.FLOAT, false, 24, 0);
        gl.enableVertexAttribArray(1); gl.vertexAttribPointer(1, 1, gl.FLOAT, false, 24, 12);
        gl.enableVertexAttribArray(2); gl.vertexAttribPointer(2, 2, gl.FLOAT, false, 24, 16);
        gl.bindBuffer(gl.ELEMENT_ARRAY_BUFFER, indexBuffer); gl.bufferData(gl.ELEMENT_ARRAY_BUFFER, next.indices, gl.DYNAMIC_DRAW);
        mesh = next;
      } finally { gl.bindVertexArray(oldVAO); gl.bindBuffer(gl.ARRAY_BUFFER, oldArray); }
    }
    function update(view) {
      lastView = view; ready = false;
      if (disposed || failed || view.zoom < minZoom || view.terrainEnabled === false) return false;
      const box = footprint(view); if (!box) return false;
      const exaggeration = view.terrainExaggeration ?? options.exaggeration ?? 3;
      const scale = Math.max(box.east - box.west, box.north - box.south);
      const changedScale = previousScale && (scale < previousScale * .85 || scale > previousScale * 1.15);
      const reusable = mesh && mesh.exaggeration === exaggeration && contains(mesh.bounds, box) && !changedScale;
      if (reusable && !dirty) { ready = true; return true; }
      const bounds = reusable ? mesh.bounds : paddedBounds(box, exaggeration), tiles = tilesForBounds(bounds);
      if (tiles.length > cache.limit || bounds.east - bounds.west >= 180) return false;
      if (!cache.require(tiles)) return false;
      if (!reusable || dirty) {
        const next = buildMesh(bounds, (lon, lat) => cache.sample(lon, lat),
          {...options, exaggeration, sampleSurface: (lon, lat) => cache.sampleSurface(lon, lat)});
        if (!next) return false;
        try { upload(next); } catch (error) {
          failed = true;
          console.warn("[glmap] regional terrain unavailable; using globe relief:", error.message);
          return false;
        }
        previousScale = scale; dirty = false;
      }
      ready = true; return true;
    }
    function draw(program, view = lastView) {
      if (!ready || disposed || !mesh || !program || !view) return false;
      const oldVAO = gl.getParameter(gl.VERTEX_ARRAY_BINDING), oldProgram = gl.getParameter(gl.CURRENT_PROGRAM);
      let locations = uniforms.get(program);
      if (!locations) {
        locations = Object.fromEntries(["uInvRot", "uCamera", "uRayBasis", "uHalf"].map(key => [key, gl.getUniformLocation(program, key)]));
        uniforms.set(program, locations);
      }
      try {
        gl.useProgram(program); gl.bindVertexArray(vao);
        gl.uniformMatrix3fv(locations.uInvRot, false, view.invBasis || IDENTITY);
        gl.uniform3fv(locations.uCamera, viewCamera(view));
        gl.uniformMatrix3fv(locations.uRayBasis, false, view.rayBasis || IDENTITY);
        gl.uniform2fv(locations.uHalf, viewHalf(view));
        gl.drawElements(gl.TRIANGLES, mesh.indices.length, gl.UNSIGNED_INT, 0);
        return true;
      } finally { gl.bindVertexArray(oldVAO); gl.useProgram(oldProgram); }
    }
    return {
      update, draw,
      sampleHeight(longitude, latitude) { return cache.sample(longitude, latitude) ?? options.baseHeight?.(longitude, latitude) ?? 0; },
      get ready() { return ready; },
      get loading() { return cache.pending.size > 0 || cache.queue.length > 0; },
      get stats() { return {tiles: cache.tiles.size, requests: cache.pending.size, ready,
        vertices: mesh ? mesh.vertices.length / 6 : 0, triangles: mesh ? mesh.indices.length / 3 : 0,
        minHeight: mesh?.minHeight ?? null, maxHeight: mesh?.maxHeight ?? null}; },
      dispose() {
        disposed = true; ready = false; cache.dispose();
        if (vao) gl.deleteVertexArray(vao); if (vertexBuffer) gl.deleteBuffer(vertexBuffer); if (indexBuffer) gl.deleteBuffer(indexBuffer);
        vao = vertexBuffer = indexBuffer = null; mesh = null;
      },
    };
  }
  window.TerrainSurface = Object.freeze({create, vertexSource, TileCache, tileAt, tilePath, decodePixels,
    sampleTile, footprint, paddedBounds, tilesForBounds, buildMesh, makeLakeSampler, bedLakes: BED_LAKES,
    earthMetres: EARTH_METRES, tileSize: TILE_SIZE, tileCells: TILE_CELLS, sampleDegrees: SAMPLE_DEGREES});
})();
