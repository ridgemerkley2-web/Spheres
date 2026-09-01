// The globe's camera, input and screen-space overlay. No runtime network
// requests and no third-party code.
//
// WHAT THIS FILE IS NOT: it does not shade the globe. The ground — elevation,
// bathymetry, land cover, hillshade, water — is drawn by the page's own WebGL2
// layer (see "GL physical underlay" in index.html), which owns the baked
// textures and the 680-line shader that reads them. This file hands that layer
// a camera and gets out of the way. The split is deliberate: the shading is the
// expensive, invariant-heavy half and it did not change when the map became a
// sphere; only the question "which point of the world is under this pixel"
// did, and that question is answered here.
//
// ONE COORDINATE SPACE RUNS THROUGH EVERYTHING: the Robinson canvas that
// mapgen.rs bakes, WORLD.w x WORLD.h. Country outlines, district outlines,
// rivers, the graticule, the elevation/coast/cover textures and the political
// texture painted per tick are all in it. The globe is therefore not a second
// projection to keep in step with the first — it is a lens onto the same
// canvas, and every layer stays registered by construction.
(function () {
  "use strict";

  const PI = Math.PI;
  const DEG = PI / 180;

  // mapgen.rs's own Robinson tables (mapgen.rs:19-81). RX is the parallel's
  // length as a fraction of the equator's; RY is its distance from the equator
  // as a fraction of the pole's. Both are sampled every 5 degrees.
  const RX_T = [1, .9986, .9954, .99, .9822, .973, .96, .9427, .9216, .8962,
    .8679, .835, .7986, .7597, .7186, .6732, .6213, .5722, .5322];
  const RY_T = [0, .062, .124, .186, .248, .31, .372, .434, .4958, .5571,
    .6176, .6769, .7346, .7903, .8435, .8936, .9394, .9761, 1];
  // The bake's clip. Robinson is defined to the poles; the canvas is not, and a
  // globe that pretended otherwise would stretch the last drawn parallel over
  // the cap. LAT_TOP/LAT_BOTTOM are where the texture genuinely ends.
  const LAT_TOP = 83;
  const LAT_BOTTOM = -58;

  const clamp = (v, lo, hi) => Math.max(lo, Math.min(hi, v));

  // RADIUS is DERIVED from the live bake's width rather than written down. The
  // 1000-wide bake this file was first written against is gone; hard-coding its
  // constants would have moved every coastline by a factor of 2.4 while the
  // page kept insisting the map was fine.
  function mapWidth() { return (window.WORLD && window.WORLD.w) || 2400; }
  function radius() { return mapWidth() / (2 * .8487 * PI); }

  function interpolate(table, latitude) {
    const t = Math.min(18, Math.abs(latitude) / 5);
    const i = Math.floor(t);
    return i >= 18 ? table[18] : table[i] + (t - i) * (table[i + 1] - table[i]);
  }
  function robinsonY(latitude) {
    return 1.3523 * radius() * interpolate(RY_T, latitude) * (latitude < 0 ? -1 : 1);
  }
  /// lon/lat -> Robinson canvas. The exact forward of mapgen.rs's projection:
  /// asserted against it by the shader, which carries the same two tables and
  /// inverts them.
  function project(longitude, latitude) {
    return projectIn(longitude, latitude, LAT_BOTTOM, LAT_TOP);
  }
  function projectIn(longitude, latitude, lo, hi) {
    const lat = clamp(latitude, lo, hi);
    return [
      mapWidth() / 2 + .8487 * radius() * interpolate(RX_T, lat) * longitude * DEG,
      robinsonY(LAT_TOP) - robinsonY(lat),
    ];
  }
  /// Robinson canvas -> lon/lat. Bisection on y because robinson_y is a table,
  /// not a closed form; 24 halvings put the latitude inside 1e-5 degrees, which
  /// is four orders finer than the polygons this is used to hit-test.
  function unproject(x, y) { return unprojectIn(x, y, LAT_BOTTOM, LAT_TOP); }
  function unprojectIn(x, y, lo, hi) {
    let low = lo, high = hi;
    for (let i = 0; i < 24; i += 1) {
      const middle = (low + high) / 2;
      if (projectIn(0, middle, lo, hi)[1] > y) low = middle;
      else high = middle;
    }
    const latitude = (low + high) / 2;
    const scale = .8487 * radius() * interpolate(RX_T, latitude);
    return [clamp((x - mapWidth() / 2) / scale / DEG, -180, 180), latitude];
  }

  // ---- the camera's own latitude, which is NOT the canvas's ----------------
  // project()/unproject() clamp to the bake's 83N/58S because that is where the
  // TEXTURE stops, and every hit test and every label wants that clamp. The
  // CAMERA does not: the shader deliberately paints ice past both clips, so the
  // player can look at the Antarctic, and a camera stored through the clamped
  // pair loses the excess and snaps ~12 degrees north on the next render.
  //
  // CAM_LAT is the pitch limit setView already enforces, so the free pair is
  // lossless over exactly the range the camera can reach and no further.
  const CAM_LAT = 86.4;
  function projectFree(lon, lat) { return projectIn(lon, lat, -CAM_LAT, CAM_LAT); }
  function unprojectFree(x, y) { return unprojectIn(x, y, -CAM_LAT, CAM_LAT); }

  // ---- the sphere ---------------------------------------------------------
  // Model space: +x at 90E on the equator, +y at the north pole, +z at 0E/0N.
  // View space is the model turned by yaw then pitch, with the camera parked on
  // +z looking down -z. Every screen-space question is asked in VIEW space and
  // only the answer is turned back, which is why there is one inverse rotation
  // here and no forward one.
  function pointOnSphere(longitude, latitude, r) {
    const lon = longitude * DEG, lat = latitude * DEG;
    const c = Math.cos(lat);
    return [r * c * Math.sin(lon), r * Math.sin(lat), r * c * Math.cos(lon)];
  }
  function rotate(p, yaw, pitch) {
    const cy = Math.cos(yaw), sy = Math.sin(yaw);
    const cx = Math.cos(pitch), sx = Math.sin(pitch);
    const x1 = cy * p[0] + sy * p[2];
    const z1 = -sy * p[0] + cy * p[2];
    return [x1, cx * p[1] - sx * z1, sx * p[1] + cx * z1];
  }
  function inverseRotate(p, yaw, pitch) {
    const cx = Math.cos(pitch), sx = Math.sin(pitch);
    const cy = Math.cos(yaw), sy = Math.sin(yaw);
    const y1 = cx * p[1] + sx * p[2];
    const z1 = -sx * p[1] + cx * p[2];
    return [cy * p[0] - sy * z1, y1, sy * p[0] + cy * z1];
  }

  const FOV_Y = 42 * DEG;            // vertical field of view
  const HALF_TAN = Math.tan(FOV_Y / 2);
  const ZOOM_MIN = 1, ZOOM_MAX = 48;

  /// Camera distance from the sphere's CENTRE, in sphere radii. The near limit
  /// is 1 + a hair: the camera may approach the surface but never enter it.
  function distanceFor(zoom) { return 1 + 2.45 / clamp(zoom, ZOOM_MIN, ZOOM_MAX); }

  class Globe3D {
    constructor(options) {
      this.options = options || {};
      this.canvas = options.canvas;
      this.overlay = options.overlay;
      this.overlayContext = this.overlay.getContext("2d");
      this.yaw = options.yaw || 0;
      this.pitch = options.pitch || 0;
      this.zoom = clamp(options.zoom || 1, ZOOM_MIN, ZOOM_MAX);
      this.pointers = new Map();
      this.drag = null;
      this.pinch = null;
      this.destroyed = false;
      this.frame = 0;
      this.lastSize = null;
      this.citiesShown = 0;
      this.bind();
      // A pane that changes shape must redraw: the projection is a function of
      // the aspect ratio, so a resize moves every pixel of the globe even
      // though the camera did not turn.
      this.resizeObserver = new ResizeObserver(() => this.schedule());
      this.resizeObserver.observe(this.canvas);
    }

    // ---- camera ----------------------------------------------------------

    distance() { return distanceFor(this.zoom); }

    /// Device pixels per Robinson canvas unit at the point the camera is over.
    /// This is what the ground shader's level-of-detail and its antialiasing
    /// widths are measured in, and it is the one number that has to cross from
    /// this file into the shader honestly: too large and the ground samples a
    /// sharper mip than the screen can show (sparkle), too small and it blurs.
    ///
    /// It is read at the SUB-CAMERA POINT, not averaged over the disc. The
    /// centre is where the player is looking and where the detail has to be
    /// right; the limb is foreshortened into a few pixels no matter what.
    pixelsPerWorld(size) {
      const pxPerRadian = (size.height / 2) / HALF_TAN / Math.max(this.distance() - 1, 1e-4);
      const lat = this.pitch / DEG;
      const worldPerRadian = .8487 * radius() * interpolate(RX_T, lat);
      return pxPerRadian / Math.max(worldPerRadian, 1e-6);
    }

    /// The inverse rotation, column-major, for the shader. Built by turning the
    /// three basis vectors rather than by writing the matrix out, so it cannot
    /// disagree with inverseRotate() — which is what picking uses.
    inverseBasis() {
      const x = inverseRotate([1, 0, 0], this.yaw, this.pitch);
      const y = inverseRotate([0, 1, 0], this.yaw, this.pitch);
      const z = inverseRotate([0, 0, 1], this.yaw, this.pitch);
      return new Float32Array([x[0], x[1], x[2], y[0], y[1], y[2], z[0], z[1], z[2]]);
    }

    setView(yaw, pitch, zoom, silent) {
      // Yaw wraps rather than accumulating: a player who spins the globe for a
      // minute should not hand the shader a number with no precision left in
      // its low bits.
      this.yaw = ((yaw + PI) % (2 * PI) + 2 * PI) % (2 * PI) - PI;
      // Pitch stops just short of the poles. At exactly +/-90 the yaw axis and
      // the view axis coincide and the drag loses its handle on the globe.
      this.pitch = clamp(pitch, -PI * .48, PI * .48);
      this.zoom = clamp(zoom, ZOOM_MIN, ZOOM_MAX);
      if (!silent && this.options.onViewChange) {
        this.options.onViewChange(this.yaw, this.pitch, this.zoom);
      }
      this.schedule();
    }

    nudge(horizontal, vertical) {
      // The step shrinks as the globe fills the screen, so a keypress moves the
      // view by roughly the same fraction of what is on screen at every zoom.
      const step = 1 / Math.sqrt(this.zoom);
      this.setView(this.yaw + horizontal * PI / 12 * step,
                   this.pitch + vertical * PI / 18 * step, this.zoom);
    }

    zoomBy(factor) { this.setView(this.yaw, this.pitch, this.zoom * factor); }

    /// Turn the globe so a lon/lat sits under the camera.
    lookAt(longitude, latitude, zoom) {
      this.setView(-longitude * DEG, latitude * DEG, zoom === undefined ? this.zoom : zoom);
    }

    /// …and the same thing said in canvas coordinates, which is how the rest of
    /// the page names places (anchors, centroids, front seams).
    lookAtWorld(x, y, zoom) {
      const geo = unproject(x, y);
      this.lookAt(geo[0], geo[1], zoom);
    }

    // ---- frame -----------------------------------------------------------

    /// Coalesce to one draw per animation frame. The globe answers a drag with
    /// a full-screen shader pass; a pointermove burst that outruns the display
    /// would otherwise pay for frames nobody sees.
    schedule() {
      if (this.destroyed || this.frame) return;
      this.frame = requestAnimationFrame(() => { this.frame = 0; this.render(); });
    }

    /// THE ONE PLACE THE BACKING STORE IS SIZED, and that is the point of it.
    /// A second opinion about how big the drawing surface is would be a second
    /// opinion about where every pixel of the world lands: the fragment shader
    /// derives its rays from gl_FragCoord over uRes, so a canvas sized by one
    /// rule and described to the shader by another puts the globe somewhere the
    /// pointer is not.
    ///
    /// A HIDDEN PANE IS REPORTED, NOT RESIZED. #pane-map is display:none while
    /// another tab is up, so clientWidth is 0 -- and rounding that up to 1 would
    /// throw away a full-size backing store on every tab switch and reallocate
    /// it on the way back. `hidden` says so and the caller declines the frame.
    resize() {
      // Read through a function, not captured at boot: this is the performance
      // guard's one lever, and a value copied at construction makes pulling it
      // a no-op -- the guard would halve a number nothing reads and then
      // escalate to standing the whole layer down.
      const cap = typeof this.options.dprCap === "function" ? this.options.dprCap() : 2;
      const ratio = Math.min(cap || 2, window.devicePixelRatio || 1);
      const cw = this.canvas.clientWidth, chh = this.canvas.clientHeight;
      if (!cw || !chh) {
        const last = this.lastSize;
        return { width: last ? last.width : 1, height: last ? last.height : 1,
                 ratio, aspect: last ? last.aspect : 1, hidden: true };
      }
      const width = Math.max(1, Math.round(cw * ratio));
      const height = Math.max(1, Math.round(chh * ratio));
      if (this.canvas.width !== width || this.canvas.height !== height) {
        this.canvas.width = width;
        this.canvas.height = height;
      }
      if (this.overlay.width !== width || this.overlay.height !== height) {
        this.overlay.width = width;
        this.overlay.height = height;
      }
      // `cw`/`chh` are recorded, not just used. They are the box the backing
      // store was derived from, and rayToSphere has to normalise the pointer
      // against THAT box rather than against getBoundingClientRect() -- see the
      // note there.
      return { width, height, ratio, aspect: width / height, cw, ch: chh, hidden: false };
    }

    /// The view the shader is handed. Everything in it is derived from yaw,
    /// pitch, zoom and the canvas size — there is no second camera to drift.
    view(size) {
      return {
        width: size.width, height: size.height, ratio: size.ratio, aspect: size.aspect,
        yaw: this.yaw, pitch: this.pitch, zoom: this.zoom,
        distance: this.distance(),
        halfTan: HALF_TAN,
        invBasis: this.inverseBasis(),
        pxPerWorld: this.pixelsPerWorld(size),
        lk: Math.log2(Math.max(this.zoom, 1e-6)),
      };
    }

    render() {
      if (this.destroyed) return;
      // A synchronous draw satisfies any frame already queued. Without this,
      // applyCam's setView schedules a rAF and then renders immediately, and the
      // queued callback draws the same camera a second time -- a full shader
      // pass and a full overlay rebuild, twice, on every tick and every tween
      // frame.
      if (this.frame) { cancelAnimationFrame(this.frame); this.frame = 0; }
      const size = this.resize();
      if (size.hidden) return;          // laid out at zero: nothing to draw on
      this.lastSize = size;
      const view = this.view(size);
      if (this.options.onDraw) this.options.onDraw(view);
      const context = this.overlayContext;
      context.clearRect(0, 0, size.width, size.height);
      context.lineJoin = "round";
      this.drawCities(view);
      if (this.options.onOverlay) this.options.onOverlay(context, view, this);
    }

    // ---- projection ------------------------------------------------------

    /// lon/lat -> backing-store pixels, or null when the point is behind the
    /// globe or off the frame. `lift` places the mark just above the surface so
    /// a label on a mountain is not z-fought by the mountain.
    projectGeo(longitude, latitude, view, lift) {
      const p = rotate(pointOnSphere(longitude, latitude, lift || 1.004), this.yaw, this.pitch);
      const distance = view.distance;
      // The horizon, exactly: a point on a sphere of radius r is visible from
      // distance d only while its z exceeds r^2/d. Using z > 0 instead would
      // let the far limb bleed a ring of labels around the edge.
      const r = lift || 1.004;
      if (p[2] <= r * r / distance) return null;
      const divisor = distance - p[2];
      if (divisor <= 1e-6) return null;
      const ndcX = p[0] / HALF_TAN / view.aspect / divisor;
      const ndcY = p[1] / HALF_TAN / divisor;
      if (Math.abs(ndcX) > 1.15 || Math.abs(ndcY) > 1.15) return null;
      return [(ndcX * .5 + .5) * view.width, (.5 - ndcY * .5) * view.height];
    }

    /// The same, named in Robinson canvas coordinates.
    projectWorld(x, y, view, lift) {
      const geo = unproject(x, y);
      return this.projectGeo(geo[0], geo[1], view, lift);
    }

    /// How square-on the surface is at a place: 1 directly under the camera,
    /// 0 at the limb, negative behind. A label at 0.15 is legible; the same
    /// label at 0.02 is sitting on ground compressed into two pixels and is
    /// lying about where it points. This is what the overlay fades on.
    facingGeo(longitude, latitude) {
      return rotate(pointOnSphere(longitude, latitude, 1), this.yaw, this.pitch)[2];
    }
    facingWorld(x, y) {
      const geo = unproject(x, y);
      return this.facingGeo(geo[0], geo[1]);
    }

    /// How many backing-store pixels one canvas unit spans AT a given place —
    /// used to size a mark that has to hold a constant screen size while the
    /// surface under it turns away. Measured, not modelled: two projections and
    /// a distance.
    scaleAt(x, y, view) {
      const a = this.projectWorld(x, y, view);
      if (!a) return 0;
      const b = this.projectWorld(x + 4, y, view);
      if (!b) return 0;
      return Math.hypot(b[0] - a[0], b[1] - a[1]) / 4;
    }

    /// Screen pixel -> model-space point on the sphere, or null off the disc.
    ///
    /// THE BOX IS MEASURED THE SAME WAY THE SHADER MEASURED IT, and that is the
    /// entire content of these four lines. The shader's rays come from
    /// gl_FragCoord over uRes, and uRes is the BACKING STORE, which resize()
    /// derived from `clientWidth`/`clientHeight` -- integers. getBoundingClient-
    /// Rect() returns the true fractional box, and the two differ by up to half
    /// a pixel: measured at 282.359 against 282, which put every pick a
    /// constant 0.18 px west of where the globe was actually drawn.
    ///
    /// This is the same defect as reading an svg's element box where its
    /// viewBox content rect was meant, in a new place: two opinions about one
    /// box. There is one opinion here, and resize() is where it is formed.
    rayToSphere(clientX, clientY) {
      const rect = this.canvas.getBoundingClientRect();
      if (!rect.width || !rect.height) return null;
      const last = this.lastSize;
      const cw = last && last.cw ? last.cw : rect.width;
      const ch = last && last.ch ? last.ch : rect.height;
      const ndcX = (clientX - rect.left) / cw * 2 - 1;
      const ndcY = 1 - (clientY - rect.top) / ch * 2;
      const aspect = last ? last.aspect : cw / ch;
      // Solved in VIEW space, where the camera is on +z and the ray needs no
      // rotation; only the hit point is turned back into model space.
      const dir = [ndcX * aspect * HALF_TAN, ndcY * HALF_TAN, -1];
      const len = Math.hypot(dir[0], dir[1], dir[2]);
      dir[0] /= len; dir[1] /= len; dir[2] /= len;
      const distance = this.distance();
      const b = dir[2] * distance;                 // dot(origin, dir), origin = (0,0,d)
      const c = distance * distance - 1;
      const disc = b * b - c;
      if (disc < 0) return null;
      const t = -b - Math.sqrt(disc);
      if (t < 0) return null;
      const hit = [dir[0] * t, dir[1] * t, distance + dir[2] * t];
      return inverseRotate(hit, this.yaw, this.pitch);
    }

    /// Screen pixel -> lon/lat, or null off the disc.
    geoAt(clientX, clientY) {
      const p = this.rayToSphere(clientX, clientY);
      if (!p) return null;
      return [Math.atan2(p[0], p[2]) / DEG, Math.asin(clamp(p[1], -1, 1)) / DEG];
    }

    /// Screen pixel -> Robinson canvas point, or null off the disc or off the
    /// bake's latitude clip. This is the bridge every hit test crosses: past
    /// here the page is back in the one coordinate space it already knows.
    worldAt(clientX, clientY) {
      const geo = this.geoAt(clientX, clientY);
      if (!geo) return null;
      if (geo[1] > LAT_TOP || geo[1] < LAT_BOTTOM) return null;
      return project(geo[0], geo[1]);
    }

    // ---- cities ----------------------------------------------------------

    /// Which cities have earned their ink at this zoom. Natural Earth's scale
    /// rank is the source's own judgement of when a place matters, so the
    /// bands are cuts on it rather than on population — population alone would
    /// hide Canberra behind six Chinese prefectures nobody is looking for.
    cityVisible(city) {
      if (this.zoom < 1.6) return city.rank <= 1 || (city.capital && city.pop > 5000000);
      if (this.zoom < 2.6) return city.rank <= 3 || (city.capital && city.pop > 1500000);
      if (this.zoom < 4.5) return city.rank <= 5 || city.capital;
      if (this.zoom < 9) return city.rank <= 7 || city.capital;
      if (this.zoom < 18) return city.rank <= 9 || city.capital;
      return true;
    }

    drawCities(view) {
      const source = this.options.cities || window.CITIES || [];
      if (!source.length || this.options.showCities === false) {
        this.citiesShown = 0;
        if (this.options.onCitiesChange) this.options.onCitiesChange(0);
        return;
      }
      const context = this.overlayContext;
      const ratio = view.ratio;
      const cities = source.filter((c) => this.cityVisible(c));
      // Strongest claim first, so the collision test below drops the weaker
      // label rather than whichever happened to be earlier in the file.
      cities.sort((a, b) => a.rank - b.rank || b.pop - a.pop);
      const placed = [];
      let shown = 0;
      for (const city of cities) {
        const point = this.projectGeo(city.lon, city.lat, view, 1.002);
        if (!point) continue;
        const fontSize = clamp(10.5 + Math.log2(Math.max(this.zoom, 1)) * 1.1, 10.5, 16) * ratio;
        const width = city.name.length * fontSize * .56;
        // Rectangle overlap against everything already drawn. O(n^2) on a list
        // the zoom bands keep short — 1249 cities only all pass the filter at
        // the closest zoom, where almost none of them project.
        if (placed.some((p) => Math.abs(p[0] - point[0]) < (p[2] + width) * .5 + 8 * ratio
                            && Math.abs(p[1] - point[1]) < fontSize + 6 * ratio)) continue;
        placed.push([point[0], point[1], width]);
        context.beginPath();
        context.arc(point[0], point[1], (city.capital ? 3.2 : 2.2) * ratio, 0, PI * 2);
        context.fillStyle = city.capital ? "#ffd08a" : "#7de1f2";
        context.fill();
        context.strokeStyle = "#06101bcc";
        context.lineWidth = 1.5 * ratio;
        context.stroke();
        context.font = `${city.capital ? 600 : 400} ${fontSize}px Inter, system-ui, sans-serif`;
        context.textBaseline = "middle";
        context.lineWidth = 3 * ratio;
        context.strokeStyle = "#050b13ee";
        context.strokeText(city.name, point[0] + 6 * ratio, point[1]);
        context.fillStyle = city.capital ? "#ffe0ad" : "#d8e7f3";
        context.fillText(city.name, point[0] + 6 * ratio, point[1]);
        shown += 1;
      }
      this.citiesShown = shown;
      if (this.options.onCitiesChange) this.options.onCitiesChange(shown);
    }

    // ---- input -----------------------------------------------------------

    bind() {
      this.onPointerDown = (event) => {
        if (event.button !== undefined && event.button !== 0 && this.pointers.size === 0) return;
        if (this.options.onUserInput) this.options.onUserInput();
        document.documentElement.classList.add("globe-dragging");
        try { this.canvas.setPointerCapture(event.pointerId); } catch (_) {}
        this.pointers.set(event.pointerId, { x: event.clientX, y: event.clientY });
        if (this.pointers.size === 1) {
          this.drag = { x: event.clientX, y: event.clientY, yaw: this.yaw, pitch: this.pitch, moved: false };
        } else if (this.pointers.size === 2) {
          const p = [...this.pointers.values()];
          this.pinch = { distance: Math.hypot(p[1].x - p[0].x, p[1].y - p[0].y), zoom: this.zoom };
        }
        event.preventDefault();
      };
      this.onPointerMove = (event) => {
        if (!this.pointers.has(event.pointerId)) {
          if (this.options.onHover) this.options.onHover(event);
          return;
        }
        this.pointers.set(event.pointerId, { x: event.clientX, y: event.clientY });
        if (this.pointers.size >= 2 && this.pinch) {
          const p = [...this.pointers.values()];
          const d = Math.hypot(p[1].x - p[0].x, p[1].y - p[0].y);
          this.setView(this.yaw, this.pitch, this.pinch.zoom * d / Math.max(1, this.pinch.distance));
        } else if (this.drag) {
          const dx = event.clientX - this.drag.x;
          const dy = event.clientY - this.drag.y;
          if (Math.hypot(dx, dy) > 4) this.drag.moved = true;
          // The turn per pixel falls with zoom so the ground keeps roughly the
          // same speed under the pointer as the globe grows. It is not the flat
          // map's exact 1:1 — a sphere cannot give that away from the
          // sub-camera point — but it holds there, which is where a drag starts.
          const rate = 1 / (this.distance() - 1) * .0092;
          this.setView(this.drag.yaw + dx * rate, this.drag.pitch + dy * rate, this.zoom);
        }
        event.preventDefault();
      };
      // `cancelled` is passed by the pointercancel binding below. A gesture the
      // BROWSER tore up -- a scroll takeover, a system gesture, a lost device --
      // is not a click, and treating it as one opens a dossier the player never
      // asked for.
      this.onPointerUp = (event, cancelled) => {
        const wasClick = !cancelled && this.drag && !this.drag.moved && this.pointers.size === 1;
        this.pointers.delete(event.pointerId);
        if (wasClick && this.options.onPick) this.options.onPick(event);
        if (this.pointers.size === 1) {
          const p = [...this.pointers.values()][0];
          this.drag = { x: p.x, y: p.y, yaw: this.yaw, pitch: this.pitch, moved: true };
        } else if (!this.pointers.size) {
          this.drag = null;
          document.documentElement.classList.remove("globe-dragging");
        }
        this.pinch = null;
      };
      this.onWheel = (event) => {
        if (this.options.onUserInput) this.options.onUserInput();
        this.setView(this.yaw, this.pitch, this.zoom * Math.exp(-event.deltaY * .0015));
        event.preventDefault();
      };
      this.onDoubleClick = (event) => {
        // Dive toward what was double-clicked rather than straight in, so the
        // gesture reads as "go there" and not "magnify the middle".
        const geo = this.geoAt(event.clientX, event.clientY);
        if (geo) this.lookAt(geo[0], geo[1], this.zoom * 1.9);
        else this.setView(this.yaw, this.pitch, this.zoom * 1.9);
        event.preventDefault();
      };
      this.onLeave = () => { if (this.options.onHover) this.options.onHover(null); };
      this.canvas.addEventListener("pointerdown", this.onPointerDown);
      this.canvas.addEventListener("pointermove", this.onPointerMove);
      this.canvas.addEventListener("pointerup", this.onPointerUp);
      this.onPointerCancel = (event) => this.onPointerUp(event, true);
      this.canvas.addEventListener("pointercancel", this.onPointerCancel);
      this.canvas.addEventListener("pointerleave", this.onLeave);
      this.canvas.addEventListener("wheel", this.onWheel, { passive: false });
      this.canvas.addEventListener("dblclick", this.onDoubleClick);
    }

    destroy() {
      this.destroyed = true;
      if (this.frame) cancelAnimationFrame(this.frame);
      this.frame = 0;
      document.documentElement.classList.remove("globe-dragging");
      try { this.resizeObserver.disconnect(); } catch (_) {}
      this.canvas.removeEventListener("pointerdown", this.onPointerDown);
      this.canvas.removeEventListener("pointermove", this.onPointerMove);
      this.canvas.removeEventListener("pointerup", this.onPointerUp);
      this.canvas.removeEventListener("pointercancel", this.onPointerCancel);
      this.canvas.removeEventListener("pointerleave", this.onLeave);
      this.canvas.removeEventListener("wheel", this.onWheel);
      this.canvas.removeEventListener("dblclick", this.onDoubleClick);
    }
  }

  Globe3D.project = project;
  Globe3D.unproject = unproject;
  Globe3D.projectFree = projectFree;
  Globe3D.unprojectFree = unprojectFree;
  Globe3D.CAM_LAT = CAM_LAT;
  Globe3D.LAT_TOP = LAT_TOP;
  Globe3D.LAT_BOTTOM = LAT_BOTTOM;
  Globe3D.ZOOM_MIN = ZOOM_MIN;
  Globe3D.ZOOM_MAX = ZOOM_MAX;
  Globe3D.distanceFor = distanceFor;
  window.Globe3D = Globe3D;
})();
