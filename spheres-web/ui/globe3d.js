// Native WebGL globe renderer. No runtime network requests or third-party code.
(function () {
  "use strict";

  const PI = Math.PI;
  const RX = [1, .9986, .9954, .99, .9822, .973, .96, .9427, .9216, .8962, .8679, .835, .7986, .7597, .7186, .6732, .6213, .5722, .5322];
  const RY = [0, .062, .124, .186, .248, .31, .372, .434, .4958, .5571, .6176, .6769, .7346, .7903, .8435, .8936, .9394, .9761, 1];
  const LAT_TOP = 83;
  const LAT_BOTTOM = -58;
  const MAP_WIDTH = 1000;
  const RADIUS = MAP_WIDTH / (2 * .8487 * PI);
  const TEXTURE_WIDTH = 1024;
  const TEXTURE_HEIGHT = 512;

  function clamp(value, low, high) { return Math.max(low, Math.min(high, value)); }
  function interpolate(table, latitude) {
    const t = Math.min(18, Math.abs(latitude) / 5);
    const i = Math.floor(t);
    return i >= 18 ? table[18] : table[i] + (t - i) * (table[i + 1] - table[i]);
  }
  function robinsonY(latitude) {
    return 1.3523 * RADIUS * interpolate(RY, latitude) * (latitude < 0 ? -1 : 1);
  }
  function projectRobinson(longitude, latitude) {
    const lat = clamp(latitude, LAT_BOTTOM, LAT_TOP);
    return [
      MAP_WIDTH / 2 + .8487 * RADIUS * interpolate(RX, lat) * longitude * PI / 180,
      robinsonY(LAT_TOP) - robinsonY(lat),
    ];
  }

  function unprojectRobinson(x, y) {
    let low = LAT_BOTTOM, high = LAT_TOP;
    for (let i = 0; i < 24; i += 1) {
      const middle = (low + high) / 2;
      if (projectRobinson(0, middle)[1] > y) low = middle;
      else high = middle;
    }
    const latitude = (low + high) / 2;
    const scale = .8487 * RADIUS * interpolate(RX, latitude);
    const longitude = clamp((x - MAP_WIDTH / 2) / scale * 180 / PI, -180, 180);
    return [longitude, latitude];
  }

  function compile(gl, type, source) {
    const shader = gl.createShader(type);
    gl.shaderSource(shader, source);
    gl.compileShader(shader);
    if (!gl.getShaderParameter(shader, gl.COMPILE_STATUS)) {
      throw new Error(gl.getShaderInfoLog(shader) || "WebGL shader compilation failed");
    }
    return shader;
  }

  function program(gl, vertexSource, fragmentSource) {
    const result = gl.createProgram();
    gl.attachShader(result, compile(gl, gl.VERTEX_SHADER, vertexSource));
    gl.attachShader(result, compile(gl, gl.FRAGMENT_SHADER, fragmentSource));
    gl.linkProgram(result);
    if (!gl.getProgramParameter(result, gl.LINK_STATUS)) {
      throw new Error(gl.getProgramInfoLog(result) || "WebGL program link failed");
    }
    return result;
  }

  function perspective(fieldOfView, aspect, near, far) {
    const f = 1 / Math.tan(fieldOfView / 2);
    const nf = 1 / (near - far);
    return new Float32Array([
      f / aspect, 0, 0, 0,
      0, f, 0, 0,
      0, 0, (far + near) * nf, -1,
      0, 0, 2 * far * near * nf, 0,
    ]);
  }

  function modelMatrix(yaw, pitch) {
    const cy = Math.cos(yaw), sy = Math.sin(yaw);
    const cx = Math.cos(pitch), sx = Math.sin(pitch);
    // Column-major Rx(pitch) * Ry(yaw).
    return new Float32Array([
      cy, sx * sy, -cx * sy, 0,
      0, cx, sx, 0,
      sy, -sx * cy, cx * cy, 0,
      0, 0, 0, 1,
    ]);
  }

  function rotatePoint(point, yaw, pitch) {
    const cy = Math.cos(yaw), sy = Math.sin(yaw);
    const cx = Math.cos(pitch), sx = Math.sin(pitch);
    const x1 = cy * point[0] + sy * point[2];
    const z1 = -sy * point[0] + cy * point[2];
    return [x1, cx * point[1] - sx * z1, sx * point[1] + cx * z1];
  }

  function inverseRotatePoint(point, yaw, pitch) {
    const cx = Math.cos(pitch), sx = Math.sin(pitch);
    const cy = Math.cos(yaw), sy = Math.sin(yaw);
    const y1 = cx * point[1] + sx * point[2];
    const z1 = -sx * point[1] + cx * point[2];
    return [cy * point[0] - sy * z1, y1, sy * point[0] + cy * z1];
  }

  function pointOnSphere(longitude, latitude, radius = 1) {
    const lon = longitude * PI / 180;
    const lat = latitude * PI / 180;
    const c = Math.cos(lat);
    return [radius * c * Math.sin(lon), radius * Math.sin(lat), radius * c * Math.cos(lon)];
  }

  function normalize(point) {
    const length = Math.hypot(point[0], point[1], point[2]) || 1;
    return [point[0] / length, point[1] / length, point[2] / length];
  }

  function sphereMesh(latitudes = 64, longitudes = 96) {
    const vertices = [];
    const indices = [];
    for (let y = 0; y <= latitudes; y += 1) {
      const v = y / latitudes;
      const latitude = PI / 2 - v * PI;
      for (let x = 0; x <= longitudes; x += 1) {
        const u = x / longitudes;
        const longitude = u * PI * 2 - PI;
        const c = Math.cos(latitude);
        vertices.push(c * Math.sin(longitude), Math.sin(latitude), c * Math.cos(longitude), u, 1 - v);
      }
    }
    for (let y = 0; y < latitudes; y += 1) {
      for (let x = 0; x < longitudes; x += 1) {
        const a = y * (longitudes + 1) + x;
        const b = a + longitudes + 1;
        indices.push(a, b, a + 1, b, b + 1, a + 1);
      }
    }
    return { vertices: new Float32Array(vertices), indices: new Uint16Array(indices) };
  }

  let textureLookup = null;
  function robinsonLookup(world) {
    if (textureLookup && textureLookup.height === Math.ceil(world.h)) return textureLookup.indices;
    const sourceHeight = Math.ceil(world.h);
    const indices = new Int32Array(TEXTURE_WIDTH * TEXTURE_HEIGHT);
    indices.fill(-1);
    for (let y = 0; y < TEXTURE_HEIGHT; y += 1) {
      const latitude = 90 - y / (TEXTURE_HEIGHT - 1) * 180;
      if (latitude < LAT_BOTTOM || latitude > LAT_TOP) continue;
      for (let x = 0; x < TEXTURE_WIDTH; x += 1) {
        const longitude = x / (TEXTURE_WIDTH - 1) * 360 - 180;
        const source = projectRobinson(longitude, latitude);
        const sx = clamp(Math.round(source[0]), 0, world.w - 1);
        const sy = clamp(Math.round(source[1]), 0, sourceHeight - 1);
        indices[y * TEXTURE_WIDTH + x] = sy * world.w + sx;
      }
    }
    textureLookup = { height: sourceHeight, indices };
    return indices;
  }

  function buildTextureCanvas(options) {
    const world = options.world;
    const source = document.createElement("canvas");
    source.width = world.w;
    source.height = Math.ceil(world.h);
    const context = source.getContext("2d", { willReadFrequently: true });
    context.fillStyle = "#08192a";
    context.fillRect(0, 0, source.width, source.height);
    context.lineJoin = "round";
    context.lineCap = "round";

    context.strokeStyle = "#274761";
    context.lineWidth = .55;
    context.globalAlpha = .7;
    for (const path of world.graticule) context.stroke(new Path2D(path));
    context.globalAlpha = 1;

    const claimed = {};
    for (const nation of options.nations) {
      for (const code of options.territory[nation.id] || []) claimed[code] = nation;
    }
    for (const code of Object.keys(world.countries)) {
      const nation = claimed[code];
      context.fillStyle = nation ? options.colorFor(nation) : "#263b4c";
      context.strokeStyle = nation ? "#6d8296" : "#415b70";
      context.lineWidth = nation && nation.id === options.playerId ? 1.35 : .55;
      const path = new Path2D(world.countries[code]);
      context.fill(path, "evenodd");
      context.stroke(path);
    }

    const sourcePixels = context.getImageData(0, 0, source.width, source.height).data;
    const texture = document.createElement("canvas");
    texture.width = TEXTURE_WIDTH;
    texture.height = TEXTURE_HEIGHT;
    const target = texture.getContext("2d");
    const image = target.createImageData(TEXTURE_WIDTH, TEXTURE_HEIGHT);
    const lookup = robinsonLookup(world);
    for (let i = 0; i < lookup.length; i += 1) {
      const to = i * 4;
      const sourceIndex = lookup[i];
      if (sourceIndex < 0) {
        const latitude = 90 - Math.floor(i / TEXTURE_WIDTH) / (TEXTURE_HEIGHT - 1) * 180;
        const polar = latitude < -72 ? 1 : 0;
        image.data[to] = polar ? 98 : 5;
        image.data[to + 1] = polar ? 118 : 18;
        image.data[to + 2] = polar ? 132 : 32;
        image.data[to + 3] = 255;
      } else {
        const from = sourceIndex * 4;
        image.data[to] = sourcePixels[from];
        image.data[to + 1] = sourcePixels[from + 1];
        image.data[to + 2] = sourcePixels[from + 2];
        image.data[to + 3] = 255;
      }
    }
    target.putImageData(image, 0, 0);
    return texture;
  }

  class Globe3D {
    constructor(options) {
      this.options = options;
      this.canvas = options.canvas;
      this.overlay = options.overlay;
      this.overlayContext = this.overlay.getContext("2d");
      this.yaw = options.yaw || 0;
      this.pitch = options.pitch || 0;
      this.zoom = clamp(options.zoom || 1, 1, 4.25);
      this.pointers = new Map();
      this.drag = null;
      this.pinch = null;
      this.destroyed = false;
      this.gl = this.canvas.getContext("webgl", { antialias: true, alpha: true, preserveDrawingBuffer: true });
      if (!this.gl) throw new Error("WebGL is unavailable in this browser");
      this.preparePicking();
      this.prepareWebGL();
      this.bind();
      this.resizeObserver = new ResizeObserver(() => this.render());
      this.resizeObserver.observe(this.canvas);
      this.render();
    }

    preparePicking() {
      this.pickCanvas = document.createElement("canvas");
      this.pickContext = this.pickCanvas.getContext("2d");
      this.pickPaths = [];
      for (const nation of this.options.nations) {
        const paths = (this.options.territory[nation.id] || [])
          .filter((code) => this.options.world.countries[code])
          .map((code) => new Path2D(this.options.world.countries[code]));
        if (paths.length) this.pickPaths.push({ id: nation.id, paths });
      }
    }

    prepareWebGL() {
      const gl = this.gl;
      const vertex = `
        attribute vec3 a_position;
        attribute vec2 a_uv;
        uniform mat4 u_model;
        uniform mat4 u_projection;
        uniform float u_distance;
        varying vec2 v_uv;
        varying vec3 v_normal;
        varying vec3 v_position;
        void main() {
          vec4 world = u_model * vec4(a_position, 1.0);
          vec4 view = world;
          view.z -= u_distance;
          gl_Position = u_projection * view;
          v_uv = a_uv;
          v_normal = normalize((u_model * vec4(a_position, 0.0)).xyz);
          v_position = world.xyz;
        }`;
      const fragment = `
        precision mediump float;
        uniform sampler2D u_texture;
        varying vec2 v_uv;
        varying vec3 v_normal;
        varying vec3 v_position;
        void main() {
          vec3 base = texture2D(u_texture, v_uv).rgb;
          vec3 lightDirection = normalize(vec3(-0.45, 0.65, 1.0));
          float diffuse = max(dot(v_normal, lightDirection), 0.0);
          float rim = pow(1.0 - max(v_normal.z, 0.0), 2.5);
          vec3 lit = base * (0.38 + diffuse * 0.72) + vec3(0.12, 0.48, 0.62) * rim * 0.24;
          gl_FragColor = vec4(lit, 1.0);
        }`;
      this.program = program(gl, vertex, fragment);
      const mesh = sphereMesh();
      this.indexCount = mesh.indices.length;
      this.vertexBuffer = gl.createBuffer();
      gl.bindBuffer(gl.ARRAY_BUFFER, this.vertexBuffer);
      gl.bufferData(gl.ARRAY_BUFFER, mesh.vertices, gl.STATIC_DRAW);
      this.indexBuffer = gl.createBuffer();
      gl.bindBuffer(gl.ELEMENT_ARRAY_BUFFER, this.indexBuffer);
      gl.bufferData(gl.ELEMENT_ARRAY_BUFFER, mesh.indices, gl.STATIC_DRAW);

      this.texture = gl.createTexture();
      gl.bindTexture(gl.TEXTURE_2D, this.texture);
      gl.pixelStorei(gl.UNPACK_FLIP_Y_WEBGL, true);
      gl.texImage2D(gl.TEXTURE_2D, 0, gl.RGBA, gl.RGBA, gl.UNSIGNED_BYTE, buildTextureCanvas(this.options));
      gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_MIN_FILTER, gl.LINEAR_MIPMAP_LINEAR);
      gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_MAG_FILTER, gl.LINEAR);
      gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_WRAP_S, gl.REPEAT);
      gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_WRAP_T, gl.CLAMP_TO_EDGE);
      gl.generateMipmap(gl.TEXTURE_2D);

      gl.useProgram(this.program);
      this.locations = {
        position: gl.getAttribLocation(this.program, "a_position"),
        uv: gl.getAttribLocation(this.program, "a_uv"),
        model: gl.getUniformLocation(this.program, "u_model"),
        projection: gl.getUniformLocation(this.program, "u_projection"),
        distance: gl.getUniformLocation(this.program, "u_distance"),
        texture: gl.getUniformLocation(this.program, "u_texture"),
      };
    }

    bind() {
      this.onPointerDown = (event) => {
        document.documentElement.classList.add("globe-dragging");
        this.canvas.setPointerCapture(event.pointerId);
        this.pointers.set(event.pointerId, { x: event.clientX, y: event.clientY });
        if (this.pointers.size === 1) {
          this.drag = { x: event.clientX, y: event.clientY, yaw: this.yaw, pitch: this.pitch, moved: false };
        } else if (this.pointers.size === 2) {
          const points = [...this.pointers.values()];
          this.pinch = { distance: Math.hypot(points[1].x - points[0].x, points[1].y - points[0].y), zoom: this.zoom };
        }
        event.preventDefault();
      };
      this.onPointerMove = (event) => {
        if (!this.pointers.has(event.pointerId)) return;
        this.pointers.set(event.pointerId, { x: event.clientX, y: event.clientY });
        if (this.pointers.size >= 2 && this.pinch) {
          const points = [...this.pointers.values()];
          const distance = Math.hypot(points[1].x - points[0].x, points[1].y - points[0].y);
          this.setView(this.yaw, this.pitch, this.pinch.zoom * distance / Math.max(1, this.pinch.distance));
        } else if (this.drag) {
          const dx = event.clientX - this.drag.x;
          const dy = event.clientY - this.drag.y;
          if (Math.hypot(dx, dy) > 4) this.drag.moved = true;
          this.setView(this.drag.yaw + dx * .006 / Math.sqrt(this.zoom), this.drag.pitch + dy * .005 / Math.sqrt(this.zoom), this.zoom);
        }
        event.preventDefault();
      };
      this.onPointerUp = (event) => {
        const wasClick = this.drag && !this.drag.moved && this.pointers.size === 1;
        this.pointers.delete(event.pointerId);
        if (wasClick) this.pick(event.clientX, event.clientY);
        if (this.pointers.size === 1) {
          const point = [...this.pointers.values()][0];
          this.drag = { x: point.x, y: point.y, yaw: this.yaw, pitch: this.pitch, moved: true };
        } else if (!this.pointers.size) {
          this.drag = null;
          document.documentElement.classList.remove("globe-dragging");
        }
        this.pinch = null;
      };
      this.onWheel = (event) => {
        this.setView(this.yaw, this.pitch, this.zoom * Math.exp(-event.deltaY * .0015));
        event.preventDefault();
      };
      this.onDoubleClick = (event) => {
        this.setView(this.yaw, this.pitch, this.zoom * 1.45);
        event.preventDefault();
      };
      this.canvas.addEventListener("pointerdown", this.onPointerDown);
      this.canvas.addEventListener("pointermove", this.onPointerMove);
      this.canvas.addEventListener("pointerup", this.onPointerUp);
      this.canvas.addEventListener("pointercancel", this.onPointerUp);
      this.canvas.addEventListener("wheel", this.onWheel, { passive: false });
      this.canvas.addEventListener("dblclick", this.onDoubleClick);
    }

    setView(yaw, pitch, zoom) {
      this.yaw = yaw;
      this.pitch = clamp(pitch, -PI * .48, PI * .48);
      this.zoom = clamp(zoom, 1, 4.25);
      if (this.options.onViewChange) this.options.onViewChange(this.yaw, this.pitch, this.zoom);
      this.render();
    }

    nudge(horizontal, vertical) {
      this.setView(this.yaw + horizontal * PI / 12, this.pitch + vertical * PI / 18, this.zoom);
    }

    distance() { return 3.45 - (this.zoom - 1) / 3.25 * 2.18; }

    resize() {
      const ratio = Math.min(2, window.devicePixelRatio || 1);
      const width = Math.max(1, Math.round(this.canvas.clientWidth * ratio));
      const height = Math.max(1, Math.round(this.canvas.clientHeight * ratio));
      if (this.canvas.width !== width || this.canvas.height !== height) {
        this.canvas.width = width;
        this.canvas.height = height;
        this.overlay.width = width;
        this.overlay.height = height;
      }
      return { width, height, ratio };
    }

    render() {
      if (this.destroyed) return;
      const size = this.resize();
      const gl = this.gl;
      gl.viewport(0, 0, size.width, size.height);
      gl.clearColor(.015, .027, .047, 0);
      gl.clear(gl.COLOR_BUFFER_BIT | gl.DEPTH_BUFFER_BIT);
      gl.enable(gl.DEPTH_TEST);
      gl.enable(gl.CULL_FACE);
      gl.cullFace(gl.BACK);
      gl.useProgram(this.program);
      gl.bindBuffer(gl.ARRAY_BUFFER, this.vertexBuffer);
      gl.enableVertexAttribArray(this.locations.position);
      gl.vertexAttribPointer(this.locations.position, 3, gl.FLOAT, false, 20, 0);
      gl.enableVertexAttribArray(this.locations.uv);
      gl.vertexAttribPointer(this.locations.uv, 2, gl.FLOAT, false, 20, 12);
      gl.bindBuffer(gl.ELEMENT_ARRAY_BUFFER, this.indexBuffer);
      gl.activeTexture(gl.TEXTURE0);
      gl.bindTexture(gl.TEXTURE_2D, this.texture);
      gl.uniform1i(this.locations.texture, 0);
      gl.uniformMatrix4fv(this.locations.model, false, modelMatrix(this.yaw, this.pitch));
      gl.uniformMatrix4fv(this.locations.projection, false, perspective(42 * PI / 180, size.width / size.height, .05, 20));
      gl.uniform1f(this.locations.distance, this.distance());
      gl.drawElements(gl.TRIANGLES, this.indexCount, gl.UNSIGNED_SHORT, 0);
      this.lastSize = size;
      this.drawStrategicOverlay(size);
      this.drawCities(size);
    }

    projectGeo(place, size, radius = 1.006) {
      const point = rotatePoint(pointOnSphere(place.lon, place.lat, radius), this.yaw, this.pitch);
      const distance = this.distance();
      if (point[2] <= 1 / distance) return null;
      const f = 1 / Math.tan(21 * PI / 180);
      const divisor = distance - point[2];
      const ndcX = point[0] * f / (size.width / size.height) / divisor;
      const ndcY = point[1] * f / divisor;
      if (Math.abs(ndcX) > 1.08 || Math.abs(ndcY) > 1.08) return null;
      return [(ndcX * .5 + .5) * size.width, (.5 - ndcY * .5) * size.height];
    }

    drawStrategicOverlay(size) {
      const context = this.overlayContext;
      context.clearRect(0, 0, size.width, size.height);
      for (const war of this.options.wars || []) {
        const a = this.projectGeo(war.a, size, 1.012);
        const b = this.projectGeo(war.b, size, 1.012);
        for (const point of [a, b].filter(Boolean)) {
          const glow = context.createRadialGradient(point[0], point[1], 0, point[0], point[1], 28 * size.ratio);
          glow.addColorStop(0, "#ff626b99");
          glow.addColorStop(1, "#ff626b00");
          context.fillStyle = glow;
          context.beginPath();
          context.arc(point[0], point[1], 28 * size.ratio, 0, PI * 2);
          context.fill();
        }
        if (a && b) {
          context.strokeStyle = "#ff7079dd";
          context.lineWidth = 1.5 * size.ratio;
          context.setLineDash([5 * size.ratio, 4 * size.ratio]);
          context.beginPath();
          context.moveTo(a[0], a[1]);
          context.lineTo(b[0], b[1]);
          context.stroke();
          context.setLineDash([]);
        }
      }
      for (const nation of this.options.nationPoints || []) {
        if (!nation.pointOnly && !nation.player) continue;
        const point = this.projectGeo(nation, size, 1.014);
        if (!point) continue;
        context.beginPath();
        context.arc(point[0], point[1], (nation.player ? 5 : 3.5) * size.ratio, 0, PI * 2);
        context.fillStyle = nation.player ? "#ffd08a" : "#67d7ec";
        context.fill();
        context.strokeStyle = "#06101b";
        context.lineWidth = 1.5 * size.ratio;
        context.stroke();
      }
    }

    cityVisible(city) {
      if (this.zoom < 1.2) return city.rank <= 1 || (city.capital && city.pop > 5000000);
      if (this.zoom < 1.75) return city.rank <= 3 || (city.capital && city.pop > 1500000);
      if (this.zoom < 2.45) return city.rank <= 5 || city.capital;
      if (this.zoom < 3.25) return city.rank <= 7 || city.capital;
      return true;
    }

    drawCities(size) {
      const context = this.overlayContext;
      const cities = (this.options.cities || []).filter((city) => this.cityVisible(city));
      cities.sort((a, b) => a.rank - b.rank || b.pop - a.pop);
      const placed = [];
      let shown = 0;
      for (const city of cities) {
        const point = this.projectGeo(city, size);
        if (!point) continue;
        const fontSize = clamp(10 * size.ratio + (this.zoom - 1) * 1.1, 10, 17 * size.ratio);
        const labelWidth = city.name.length * fontSize * .56;
        if (placed.some((item) => Math.abs(item[0] - point[0]) < (item[2] + labelWidth) * .5 + 8 && Math.abs(item[1] - point[1]) < fontSize + 7)) continue;
        placed.push([point[0], point[1], labelWidth]);
        context.beginPath();
        context.arc(point[0], point[1], city.capital ? 3.2 * size.ratio : 2.2 * size.ratio, 0, PI * 2);
        context.fillStyle = city.capital ? "#ffd08a" : "#7de1f2";
        context.fill();
        context.strokeStyle = "#06101bcc";
        context.lineWidth = 1.5 * size.ratio;
        context.stroke();
        context.font = `${city.capital ? 600 : 400} ${fontSize}px Inter, system-ui, sans-serif`;
        context.textBaseline = "middle";
        context.lineWidth = 3 * size.ratio;
        context.strokeStyle = "#050b13ee";
        context.strokeText(city.name, point[0] + 6 * size.ratio, point[1]);
        context.fillStyle = city.capital ? "#ffe0ad" : "#d8e7f3";
        context.fillText(city.name, point[0] + 6 * size.ratio, point[1]);
        shown += 1;
      }
      if (this.options.onCitiesChange) this.options.onCitiesChange(shown);
    }

    rayToSphere(clientX, clientY) {
      const rect = this.canvas.getBoundingClientRect();
      const ndcX = (clientX - rect.left) / rect.width * 2 - 1;
      const ndcY = 1 - (clientY - rect.top) / rect.height * 2;
      const aspect = rect.width / rect.height;
      const tan = Math.tan(21 * PI / 180);
      const directionCamera = normalize([ndcX * aspect * tan, ndcY * tan, -1]);
      const origin = inverseRotatePoint([0, 0, this.distance()], this.yaw, this.pitch);
      const direction = inverseRotatePoint(directionCamera, this.yaw, this.pitch);
      const b = 2 * (origin[0] * direction[0] + origin[1] * direction[1] + origin[2] * direction[2]);
      const c = origin[0] ** 2 + origin[1] ** 2 + origin[2] ** 2 - 1;
      const discriminant = b * b - 4 * c;
      if (discriminant < 0) return null;
      const distance = (-b - Math.sqrt(discriminant)) / 2;
      if (distance < 0) return null;
      return [origin[0] + direction[0] * distance, origin[1] + direction[1] * distance, origin[2] + direction[2] * distance];
    }

    pick(clientX, clientY) {
      if (this.lastSize) {
        const rect = this.canvas.getBoundingClientRect();
        const x = (clientX - rect.left) * this.lastSize.ratio;
        const y = (clientY - rect.top) * this.lastSize.ratio;
        for (const nation of this.options.nationPoints || []) {
          if (!nation.pointOnly) continue;
          const screen = this.projectGeo(nation, this.lastSize, 1.014);
          if (screen && Math.hypot(screen[0] - x, screen[1] - y) < 13 * this.lastSize.ratio) {
            if (this.options.onPick) this.options.onPick(nation.id);
            return;
          }
        }
      }
      const point = this.rayToSphere(clientX, clientY);
      if (!point) return;
      const longitude = Math.atan2(point[0], point[2]) * 180 / PI;
      const latitude = Math.asin(clamp(point[1], -1, 1)) * 180 / PI;
      const mapPoint = projectRobinson(longitude, latitude);
      for (const nation of this.pickPaths) {
        if (nation.paths.some((path) => this.pickContext.isPointInPath(path, mapPoint[0], mapPoint[1], "evenodd"))) {
          if (this.options.onPick) this.options.onPick(nation.id);
          return;
        }
      }
    }

    destroy() {
      this.destroyed = true;
      document.documentElement.classList.remove("globe-dragging");
      this.resizeObserver.disconnect();
      this.canvas.removeEventListener("pointerdown", this.onPointerDown);
      this.canvas.removeEventListener("pointermove", this.onPointerMove);
      this.canvas.removeEventListener("pointerup", this.onPointerUp);
      this.canvas.removeEventListener("pointercancel", this.onPointerUp);
      this.canvas.removeEventListener("wheel", this.onWheel);
      this.canvas.removeEventListener("dblclick", this.onDoubleClick);
    }
  }

  Globe3D.mapPointToGeo = (point) => unprojectRobinson(point[0], point[1]);
  window.Globe3D = Globe3D;
})();
