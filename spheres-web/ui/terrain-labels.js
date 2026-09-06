/* Physical names from the bundled terrain transcription; no simulation state.

   Build once after the district index exists:
     const features = TerrainLabels.build(TERRAIN.byId, DINDEX, {worldWidth: WORLD.w});
   Draw after nation/province names and before Globe3D draws cities:
     TerrainLabels.draw(ctx, view, globe, features, {enabled: terrainView && labelsOn});

   Anchors are existing district centers, not surveyed feature coordinates.
   A retained medoid and up to five spread member centers keep long/disconnected
   groups on their sourced districts instead of averaging them into empty space.
   Draw reserves backing-pixel [centerX, centerY, width, height] boxes in the
   same globe.labelBoxes array used by the other label passes. */
(function () {
  "use strict";
  const compare = (a, b) => a < b ? -1 : a > b ? 1 : 0;
  const clamp = (value, low, high) => Math.max(low, Math.min(high, value));
  const fade = (value, low, high) => {
    const t = clamp((value - low) / (high - low), 0, 1);
    return t * t * (3 - 2 * t);
  };
  function build(terrainById, districtById, options = {}) {
    if (!terrainById || !districtById) return Object.freeze([]);
    const worldWidth = Number.isFinite(options.worldWidth) && options.worldWidth > 0 ? options.worldWidth : 2400;
    const distance = (a, b) => {
      const raw = Math.abs(a.x - b.x) % worldWidth;
      const dx = Math.min(raw, worldWidth - raw);
      return dx * dx + (a.y - b.y) * (a.y - b.y);
    };
    const groups = new Map();
    for (const id of Object.keys(terrainById).sort(compare)) {
      const terrain = terrainById[id], district = districtById[id];
      if (typeof terrain?.f !== "string" || !terrain.f.trim() || !district ||
          !Number.isFinite(district.cx) || !Number.isFinite(district.cy)) continue;
      const name = terrain.f.trim();
      if (!groups.has(name)) groups.set(name, { members: [], types: new Map() });
      const group = groups.get(name);
      group.members.push(Object.freeze({ id, x: district.cx, y: district.cy }));
      const type = typeof terrain.t === "string" ? terrain.t : "";
      group.types.set(type, (group.types.get(type) || 0) + 1);
    }
    const features = [];
    for (const [name, group] of groups) {
      const members = group.members;
      let anchor = members[0], best = Infinity;
      // Stable ids break equal-distance ties without locale-dependent sorting.
      for (const candidate of members) {
        const score = members.reduce((total, other) => total + distance(candidate, other), 0);
        if (score < best) { best = score; anchor = candidate; }
      }
      const anchors = [anchor];
      while (anchors.length < Math.min(6, members.length)) {
        let next = null, separation = 0;
        for (const candidate of members) {
          const gap = Math.min(...anchors.map(other => distance(candidate, other)));
          if (gap > separation) { separation = gap; next = candidate; }
        }
        if (!next) break; // duplicate coordinates do not add another placement
        anchors.push(next);
      }
      const terrain = [...group.types].sort((a, b) => b[1] - a[1] || compare(a[0], b[0]))[0][0];
      features.push(Object.freeze({ name, terrain, count: members.length,
        anchor, anchors: Object.freeze(anchors),
        minZoom: members.length >= 12 ? 1.6 : members.length >= 4 ? 2.4 : 3.2 }));
    }
    features.sort((a, b) => b.count - a.count || compare(a.name, b.name));
    return Object.freeze(features);
  }

  function draw(ctx, view, globe, features, settings = {}) {
    if (settings.enabled === false || !Array.isArray(features) || !features.length ||
        !ctx || !globe || typeof globe.projectWorld !== "function") return 0;
    const zoom = Number(view?.zoom), ratio = Number(view?.ratio) || 1;
    const width = Number(view?.width), height = Number(view?.height);
    if (![zoom, ratio, width, height].every(Number.isFinite) ||
        !(zoom > 1.6) || !(ratio > 0) || !(width > 0) || !(height > 0)) return 0;
    const requested = Number.isFinite(settings.maxLabels) ? Math.floor(settings.maxLabels) : 12;
    const limit = Math.min(clamp(requested, 0, 24), Math.max(1, Math.floor(width * height / (ratio * ratio * 48000))));
    if (!limit) return 0;
    const placed = globe.labelBoxes || (globe.labelBoxes = []);
    const seen = new Set();
    const padding = 8 * ratio;
    let drawn = 0;
    ctx.save();
    try {
      ctx.textAlign = "center";
      ctx.textBaseline = "middle";
      const size = clamp(11 + Math.log2(zoom) * .8, 11, 15) * ratio;
      ctx.font = `italic 500 ${size}px Georgia, "Times New Roman", serif`;
      ctx.lineWidth = 3 * ratio;
      ctx.strokeStyle = "#14221ee6";
      for (const feature of features) {
        if (drawn >= limit) break;
        if (seen.has(feature.name) || !(zoom > feature.minZoom)) continue;
        seen.add(feature.name);
        const candidates = [];
        for (const anchor of feature.anchors) {
          const face = typeof globe.facingWorld === "function" ? globe.facingWorld(anchor.x, anchor.y) : 1;
          if (!(face > .25)) continue;
          const point = globe.projectWorld(anchor.x, anchor.y, view, 1.003);
          if (!point || !Number.isFinite(point[0]) || !Number.isFinite(point[1])) continue;
          candidates.push({ anchor, point, face,
            primary: anchor.id === feature.anchor.id,
            score: (point[0] - width / 2) ** 2 + (point[1] - height / 2) ** 2 });
        }
        // Keep regional names near their central member while it is visible.
        // Choosing the closest fringe district first can move e.g. Tibet's
        // label south of its main plateau when the camera centers on Nepal.
        candidates.sort((a, b) => Number(b.primary) - Number(a.primary) || a.score - b.score || compare(a.anchor.id, b.anchor.id));
        if (!candidates.length) continue;
        const metrics = ctx.measureText(feature.name);
        // Include italic overhang and the halo, rather than only advance width.
        const boxWidth = 2 * Math.max(metrics.width / 2,
          metrics.actualBoundingBoxLeft || 0, metrics.actualBoundingBoxRight || 0) + 4 * ratio;
        const boxHeight = 2 * Math.max(size * .55,
          metrics.actualBoundingBoxAscent || 0, metrics.actualBoundingBoxDescent || 0) + 4 * ratio;
        for (const candidate of candidates) {
          const [x, y] = candidate.point;
          const box = [x, y, boxWidth, boxHeight];
          if (x - boxWidth / 2 < padding || x + boxWidth / 2 > width - padding ||
              y - boxHeight / 2 < padding || y + boxHeight / 2 > height - padding) continue;
          if (placed.some(other => Math.abs(other[0] - x) < (other[2] + boxWidth) / 2 + padding &&
              Math.abs(other[1] - y) < ((other[3] || size) + boxHeight) / 2 + 5 * ratio)) continue;
          ctx.globalAlpha = .78 * fade(zoom, feature.minZoom, feature.minZoom + .6) * fade(candidate.face, .25, .45);
          ctx.fillStyle = feature.terrain === "desert" ? "#dfc69a" : feature.terrain === "wetland" ? "#a9c9bb" : "#d1ceb1";
          ctx.strokeText(feature.name, x, y);
          ctx.fillText(feature.name, x, y);
          placed.push(box);
          drawn += 1;
          break;
        }
      }
    } finally {
      ctx.restore();
    }
    return drawn;
  }
  window.TerrainLabels = Object.freeze({ build, draw });
})();
