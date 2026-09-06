/* River hierarchy and names from the bundled Natural Earth paths.
   The baked data has no scale-rank field: projected course length is a display
   heuristic, not a claim about discharge or physical river size. No paths are
   simplified or changed here. Labels use only vertices of those source paths.

   const riverLabels = WaterDetail.buildLabels(RIVERS.rivers);
   WaterDetail.paint(ctx, RIVERS.rivers, detail,
     {terrainView, zoom: ui.cam.k, path: p2d});
   WaterDetail.drawLabels(ctx, view, globe, riverLabels, {enabled: namesOn});
   Draw labels after political/terrain names and before cities, sharing the
   globe.labelBoxes collision list. Context and coordinates use backing pixels. */
(function () {
  "use strict";
  const cache = new WeakMap();
  const clamp = (n, lo, hi) => Math.max(lo, Math.min(hi, n));
  const compare = (a, b) => a < b ? -1 : a > b ? 1 : 0;
  const fade = (n, lo, hi) => {
    const t = clamp((n - lo) / (hi - lo), 0, 1);
    return t * t * (3 - 2 * t);
  };

  function readParts(path) {
    // The committed exporter emits absolute M/L coordinates only. Refuse to
    // invent label positions if a future exporter introduces curves or arcs.
    if (typeof path !== "string" || !path.trim()) return [];
    const tokens = path.match(/[A-Za-z]|[-+]?(?:\d*\.\d+|\d+\.?\d*)(?:[eE][-+]?\d+)?/g) || [];
    const parts = [];
    let part = null, move = false, index = 0;
    while (index < tokens.length) {
      if (/^[A-Za-z]$/.test(tokens[index])) {
        const command = tokens[index++];
        if (command !== "M" && command !== "L") return [];
        move = command === "M";
      }
      const x = Number(tokens[index++]), y = Number(tokens[index++]);
      if (!Number.isFinite(x) || !Number.isFinite(y) || (!part && !move)) return [];
      if (move) { part = { length: 0, points: [] }; parts.push(part); move = false; }
      const previous = part.points[part.points.length - 1];
      if (previous) part.length += Math.hypot(x - previous.x, y - previous.y);
      part.points.push({x, y, distance: part.length});
    }
    return parts.filter(part => part.length > 0 && part.points.length > 1);
  }

  function prepare(rivers) {
    if (!Array.isArray(rivers)) return {entries: [], labels: Object.freeze([])};
    if (cache.has(rivers)) return cache.get(rivers);
    const groups = new Map();
    const entries = [];
    for (const river of rivers) {
      if (typeof river?.d !== "string" || !river.d) continue;
      const parts = readParts(river.d);
      const name = typeof river.n === "string" ? river.n.trim() : "";
      const length = parts.reduce((sum, part) => sum + part.length, 0);
      // Unnamed courses retain their own visual importance; unrelated unnamed
      // paths must not be combined into a fictitious long river.
      const key = name || river;
      if (!groups.has(key)) groups.set(key, {name, length: 0, parts: []});
      const group = groups.get(key);
      group.length += length;
      group.parts.push(...parts);
      entries.push({path: river.d, group});
    }
    const labels = [];
    for (const group of groups.values()) {
      group.tier = group.length >= 140 ? 0 : group.length >= 55 ? 1 : 2;
      if (!group.name || !group.parts.length) continue;
      // Seed from the midpoint of the longest actual part. Alternate positions
      // let labels remain visible as the camera follows a long river. Every
      // retained point is an exported vertex, including disconnected reaches.
      const parts = [...group.parts].sort((a, b) => b.length - a.length ||
        a.points[0].x - b.points[0].x || a.points[0].y - b.points[0].y);
      const pool = [];
      for (const part of parts.slice(0, 8)) {
        for (const fraction of [.5, .25, .75]) {
          const distance = part.length * fraction;
          let nearest = part.points[0];
          for (const point of part.points) {
            if (Math.abs(point.distance - distance) < Math.abs(nearest.distance - distance)) nearest = point;
          }
          if (!pool.some(point => point.x === nearest.x && point.y === nearest.y)) pool.push(nearest);
        }
      }
      const anchors = [pool[0]];
      while (anchors.length < Math.min(8, pool.length)) {
        let next = null, separation = -1;
        for (const point of pool) {
          if (anchors.includes(point)) continue;
          const gap = Math.min(...anchors.map(other => (point.x - other.x) ** 2 + (point.y - other.y) ** 2));
          if (gap > separation) { separation = gap; next = point; }
        }
        if (!next) break;
        anchors.push(next);
      }
      labels.push(Object.freeze({name: group.name, length: group.length,
        minZoom: [2, 3, 4.5][group.tier],
        anchors: Object.freeze(anchors.map(point => Object.freeze({x: point.x, y: point.y})))}));
    }
    labels.sort((a, b) => b.length - a.length || compare(a.name, b.name));
    const prepared = {entries, labels: Object.freeze(labels)};
    cache.set(rivers, prepared); // Weak keys release data if its source is replaced.
    return prepared;
  }

  function buildLabels(rivers) { return prepare(rivers).labels; }

  function paint(ctx, rivers, style = {}, settings = {}) {
    if (!ctx || typeof settings.path !== "function") return 0;
    const opacity = Number(style.riverOpacity), baseWidth = Number(style.riverWidth);
    if (!Number.isFinite(opacity) || !(opacity > 0) || !Number.isFinite(baseWidth) || !(baseWidth > 0)) return 0;
    const zoom = Number.isFinite(settings.zoom) ? Math.max(1, settings.zoom) : 1;
    let drawn = 0;
    ctx.save();
    try {
      // Political/front/resource views retain their existing line semantics.
      if (!settings.terrainView) {
        ctx.globalAlpha = clamp(opacity, 0, 1);
        ctx.strokeStyle = style.color || "#4c8ee0";
        ctx.lineWidth = baseWidth;
        for (const entry of prepare(rivers).entries) { ctx.stroke(settings.path(entry.path)); drawn += 1; }
        return drawn;
      }
      ctx.lineCap = "round";
      ctx.lineJoin = "round";
      for (const entry of prepare(rivers).entries) {
        const tier = entry.group.tier;
        const visibility = tier === 0 ? .65 + .35 * fade(zoom, 1, 3) : fade(zoom, tier === 1 ? 1.3 : 2.4, tier === 1 ? 2.5 : 4);
        if (!(visibility > 0)) continue;
        const width = clamp(baseWidth * [1.12, .85, .62][tier], .14, .88);
        const resolved = settings.path(entry.path);
        const close = fade(zoom, 2.4, 5);
        if (close > 0) {
          ctx.globalAlpha = clamp(opacity, 0, 1) * visibility * close * .2;
          ctx.strokeStyle = "#173d4b";
          ctx.lineWidth = Math.min(1.15, width + .25);
          ctx.stroke(resolved);
        }
        ctx.globalAlpha = clamp(opacity, 0, 1) * visibility;
        ctx.strokeStyle = style.color || "#86bac3";
        ctx.lineWidth = width;
        ctx.stroke(resolved);
        drawn += 1;
      }
    } finally { ctx.restore(); }
    return drawn;
  }

  function drawLabels(ctx, view, globe, labels, settings = {}) {
    if (settings.enabled === false || !ctx || !Array.isArray(labels) || !labels.length ||
        !globe || typeof globe.projectWorld !== "function") return 0;
    const zoom = Number(view?.zoom), ratio = Number(view?.ratio) || 1;
    const width = Number(view?.width), height = Number(view?.height);
    if (![zoom, ratio, width, height].every(Number.isFinite) || zoom <= 2 || ratio <= 0 || width <= 0 || height <= 0) return 0;
    const requested = Number.isFinite(settings.maxLabels) ? Math.floor(settings.maxLabels) : 10;
    const limit = Math.min(clamp(requested, 0, 20), Math.max(1, Math.floor(width * height / (ratio * ratio * 65000))));
    if (!limit) return 0;
    const placed = globe.labelBoxes || (globe.labelBoxes = []);
    const padding = 7 * ratio, seen = new Set();
    let drawn = 0;
    ctx.save();
    try {
      const size = clamp(10.5 + Math.log2(zoom) * .65, 11, 14) * ratio;
      ctx.font = `italic 500 ${size}px Georgia, "Times New Roman", serif`;
      ctx.textAlign = "center";
      ctx.textBaseline = "middle";
      ctx.strokeStyle = "#132c39e6";
      ctx.fillStyle = "#a8d3db";
      ctx.lineWidth = 3 * ratio;
      for (const label of labels) {
        if (drawn >= limit) break;
        if (!(zoom > label.minZoom) || seen.has(label.name)) continue;
        seen.add(label.name);
        const candidates = [];
        for (const anchor of label.anchors) {
          const face = typeof globe.facingWorld === "function" ? globe.facingWorld(anchor.x, anchor.y) : 1;
          if (!(face > .3)) continue;
          const point = globe.projectWorld(anchor.x, anchor.y, view, 1.003);
          if (!point || !Number.isFinite(point[0]) || !Number.isFinite(point[1])) continue;
          candidates.push({x: point[0], y: point[1] - 9 * ratio, face,
            score: (point[0] - width / 2) ** 2 + (point[1] - height / 2) ** 2});
        }
        candidates.sort((a, b) => a.score - b.score || a.x - b.x || a.y - b.y);
        if (!candidates.length) continue;
        const metrics = ctx.measureText(label.name);
        const boxWidth = 2 * Math.max(metrics.width / 2, metrics.actualBoundingBoxLeft || 0, metrics.actualBoundingBoxRight || 0) + 4 * ratio;
        const boxHeight = 2 * Math.max(size * .55, metrics.actualBoundingBoxAscent || 0, metrics.actualBoundingBoxDescent || 0) + 4 * ratio;
        for (const candidate of candidates) {
          const {x, y, face} = candidate;
          if (x - boxWidth / 2 < padding || x + boxWidth / 2 > width - padding || y - boxHeight / 2 < padding || y + boxHeight / 2 > height - padding) continue;
          if (placed.some(other => Math.abs(other[0] - x) < (other[2] + boxWidth) / 2 + padding &&
              Math.abs(other[1] - y) < ((other[3] || size) + boxHeight) / 2 + padding)) continue;
          ctx.globalAlpha = .9 * fade(zoom, label.minZoom, label.minZoom + .6) * fade(face, .3, .5);
          ctx.strokeText(label.name, x, y);
          ctx.fillText(label.name, x, y);
          placed.push([x, y, boxWidth, boxHeight]);
          drawn += 1;
          break;
        }
      }
    } finally { ctx.restore(); }
    return drawn;
  }
  window.WaterDetail = Object.freeze({paint, buildLabels, drawLabels});
})();
