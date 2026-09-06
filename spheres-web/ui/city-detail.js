/* Settlement symbols and selection from the existing Natural Earth city points.
   Buildings below are fixed-size cartographic icons, never street geometry or
   historic built-up footprints. Population remains the source's undated value.

   const cities = CityDetail.build(CITIES);
   const frame = CityDetail.draw(ctx, view, globe, cities, {selectedId, focusedId});
   CityDetail.activate(frame, backingX, backingY, city => selectCity(city));
   CityDetail.search(cities, query) supplies records for ordinary keyboard-accessible
   Find buttons. The module installs no event listeners or DOM of its own.
   settings.project(city, view, globe) can place points on a displaced terrain
   camera; otherwise the existing globe.projectGeo method is used. */
(function () {
  "use strict";
  const cache = new WeakMap();
  const clamp = (x, low, high) => Math.max(low, Math.min(high, x));
  const compare = (a, b) => a < b ? -1 : a > b ? 1 : 0;
  const fold = text => String(text || "").normalize("NFKD").replace(/[\u0300-\u036f]/g, "").toLowerCase().trim();
  const id = city => encodeURIComponent(String(city?.name || "").trim()) + "@" + city?.lon + "," + city?.lat;
  const order = (a, b) => a.rank - b.rank || (b.pop || 0) - (a.pop || 0) || compare(a.id, b.id);
  const skylineSize = zoom => clamp(24 + Math.log2(Math.max(32, zoom) / 32) * 3, 24, 32);

  function build(source) {
    if (!Array.isArray(source)) return Object.freeze([]);
    if (cache.has(source)) return cache.get(source);
    const rows = [];
    for (const city of source) {
      if (typeof city?.name !== "string" || !city.name.trim() || !Number.isFinite(city.lon) || !Number.isFinite(city.lat) ||
          Math.abs(city.lon) > 180 || Math.abs(city.lat) > 90) continue;
      rows.push(Object.freeze({id: id(city), name: city.name.trim(), lon: city.lon, lat: city.lat,
        pop: Number.isFinite(city.pop) && city.pop >= 0 ? city.pop : null,
        rank: Number.isFinite(city.rank) ? city.rank : Infinity, capital: city.capital === true}));
    }
    rows.sort(order);
    const seen = new Set();
    const cities = Object.freeze(rows.filter(city => !seen.has(city.id) && seen.add(city.id)));
    cache.set(source, cities); cache.set(cities, cities);
    return cities;
  }

  function visible(city, zoom) {
    if (zoom < 1.6) return city.rank <= 1 || (city.capital && city.pop > 5000000);
    if (zoom < 2.6) return city.rank <= 3 || (city.capital && city.pop > 1500000);
    if (zoom < 4.5) return city.rank <= 5 || city.capital;
    if (zoom < 9) return city.rank <= 7 || city.capital;
    if (zoom < 18) return city.rank <= 9 || city.capital;
    return true;
  }

  function population(city) {
    const count = city.pop;
    if (!(count > 0)) return "Source population unavailable";
    const compact = count >= 1000000 ? (count / 1000000).toFixed(1).replace(/\.0$/, "") + "m" :
      count >= 10000 ? Math.round(count / 1000) + "k" : Math.round(count).toLocaleString("en-US");
    return "Source pop. " + compact;
  }

  function search(cities, query, limit = 8) {
    const words = fold(query).split(/\s+/).filter(Boolean);
    if (!words.length) return [];
    const needle = words.join(" ");
    const count = Number.isFinite(limit) ? clamp(Math.floor(limit), 0, 30) : 8;
    return build(cities).map(city => {
      const name = fold(city.name);
      return {city, name, score: name === needle ? 0 : name.startsWith(needle) ? 1 : 2};
    }).filter(row => words.every(word => row.name.includes(word)))
      .sort((a, b) => a.score - b.score || order(a.city, b.city)).slice(0, count).map(row => row.city);
  }

  function star(ctx, x, y, radius) {
    ctx.beginPath();
    for (let i = 0; i < 10; i++) {
      const angle = -Math.PI / 2 + i * Math.PI / 5, r = i % 2 ? radius * .44 : radius;
      const px = x + Math.cos(angle) * r, py = y + Math.sin(angle) * r;
      if (i === 0) ctx.moveTo(px, py); else ctx.lineTo(px, py);
    }
    ctx.closePath(); ctx.fill(); ctx.stroke();
  }

  function skyline(ctx, city, x, y, size, ratio, highlighted) {
    const polygon = (points, color) => {
      ctx.beginPath(); points.forEach(([px, py], i) => i ? ctx.lineTo(px, py) : ctx.moveTo(px, py));
      ctx.closePath(); ctx.fillStyle = color; ctx.fill(); ctx.stroke();
    };
    ctx.lineWidth = .8 * ratio; ctx.strokeStyle = "#142934";
    // A fixed-size isometric map symbol. The five blocks are icon artwork,
    // not measured buildings, streets, city area or a claim about 1990 density.
    polygon([[x, y-size*.16], [x+size*.5, y+size*.02], [x, y+size*.2], [x-size*.5, y+size*.02]], "#173440df");
    if (highlighted) {
      ctx.strokeStyle = "#f3d79a"; ctx.lineWidth = 1.6 * ratio; ctx.stroke();
      ctx.strokeStyle = "#142934"; ctx.lineWidth = .8 * ratio;
    }
    const block = (dx, dy, width, height) => {
      const cx = x + dx * size, cy = y + dy * size, w = width * size / 2, d = w * .52, h = height * size;
      const top = [cx, cy-d-h], right = [cx+w, cy-h], front = [cx, cy+d-h], left = [cx-w, cy-h];
      polygon([left, front, [cx,cy+d], [cx-w,cy]], "#95aaa8");
      polygon([front, right, [cx+w,cy], [cx,cy+d]], "#557a85");
      polygon([top, right, front, left], "#e1dfcb");
    };
    block(-.18, -.08, .25, .28); block(.18, -.06, .24, .36);
    block(0, .02, .29, .61);
    block(-.28, .08, .24, .29); block(.27, .09, .23, .22);
    if (city.capital) {
      const sx = x + size*.35, sy = y-size*.60, r = size*.14;
      ctx.beginPath(); ctx.arc(sx, sy, r + ratio, 0, Math.PI*2); ctx.fillStyle = "#26303bdc"; ctx.fill();
      ctx.fillStyle = "#ffe1a1"; ctx.strokeStyle = "#674a26"; ctx.lineWidth = .7 * ratio;
      star(ctx, sx, sy, r);
    }
  }

  function symbol(ctx, city, x, y, radius, ratio, zoom, highlighted) {
    if (zoom >= 32) { skyline(ctx, city, x, y, skylineSize(zoom) * ratio, ratio, highlighted); return; }
    ctx.lineWidth = 1.4 * ratio;
    ctx.strokeStyle = "#10212be8";
    ctx.fillStyle = city.capital ? "#f4cf91" : "#a8cbd0";
    if (highlighted) {
      ctx.beginPath(); ctx.arc(x, y, radius + 4 * ratio, 0, Math.PI * 2);
      ctx.strokeStyle = city.capital ? "#ffe0a5" : "#b5edf3"; ctx.lineWidth = 1.7 * ratio; ctx.stroke();
      ctx.strokeStyle = "#10212be8"; ctx.lineWidth = 1.4 * ratio;
    }
    ctx.beginPath();
    if (city.capital) {
      star(ctx, x, y, radius);
    } else {
      ctx.arc(x, y, radius, 0, Math.PI * 2);
      if (zoom >= 14) {
        ctx.fillStyle = "#142b35"; ctx.fill(); ctx.strokeStyle = "#9bc5cd"; ctx.stroke();
        ctx.fillStyle = "#d2e4dd";
        // A settlement glyph inside a screen-space badge, not a footprint.
        ctx.fillRect(x - radius * .57, y - radius * .15, radius * .3, radius * .62);
        ctx.fillRect(x - radius * .17, y - radius * .59, radius * .34, radius * 1.06);
        ctx.fillRect(x + radius * .27, y - radius * .32, radius * .3, radius * .79);
      } else { ctx.fill(); ctx.stroke(); }
    }
  }

  function draw(ctx, view, globe, source, settings = {}) {
    const empty = () => ({shown: 0, hits: []});
    if (settings.enabled === false || !ctx || !globe) return empty();
    const zoom = Number(view?.zoom), ratio = Number(view?.ratio) || 1, width = Number(view?.width), height = Number(view?.height);
    if (![zoom, ratio, width, height].every(Number.isFinite) || zoom <= 0 || ratio <= 0 || width <= 0 || height <= 0) return empty();
    const project = settings.project || ((city, incoming, camera) => camera.projectGeo(city.lon, city.lat, incoming, 1.002));
    if (typeof project !== "function" || (!settings.project && typeof globe.projectGeo !== "function")) return empty();
    const selected = settings.selectedId, focused = settings.focusedId;
    const priority = city => city.id === selected ? 2 : city.id === focused ? 1 : 0;
    const cities = [...build(source)].filter(city => priority(city) || visible(city, zoom))
      .sort((a, b) => priority(b) - priority(a) || order(a, b));
    const placed = globe.labelBoxes || (globe.labelBoxes = []), hits = [];
    const padding = 5 * ratio;
    const labelLimit = Math.min(Number.isFinite(settings.maxLabels) ? clamp(Math.floor(settings.maxLabels), 0, 80) : 48,
      Math.max(1, Math.floor(width * height / (ratio * ratio * 18000))));
    const projected = [];
    for (const city of cities) {
      if (typeof globe.facingGeo === "function" && globe.facingGeo(city.lon, city.lat) <= 0) continue;
      const point = project(city, view, globe);
      if (!point || !Number.isFinite(point[0]) || !Number.isFinite(point[1])) continue;
      const [x, y] = point;
      const radius = (zoom >= 32 ? skylineSize(zoom) * .55 : city.capital ? clamp(4 + Math.log2(Math.max(zoom, 1)) * .6, 4, 8) :
        zoom >= 14 ? clamp(6 + Math.log2(zoom / 14) * .6, 6, 8) : clamp(2.3 + Math.log2(Math.max(zoom, 1)) * .45, 2.3, 4)) * ratio;
      const symbolBox = zoom >= 32 ? [x, y - skylineSize(zoom) * .28 * ratio, radius*2, skylineSize(zoom)*1.02*ratio] : [x,y,radius*2,radius*2];
      if (symbolBox[0]-symbolBox[2]/2 < 0 || symbolBox[0]+symbolBox[2]/2 > width ||
          symbolBox[1]-symbolBox[3]/2 < 0 || symbolBox[1]+symbolBox[3]/2 > height) continue;
      projected.push({city, x, y, radius, symbolBox});
    }
    let labels = 0;
    ctx.save();
    try {
      ctx.textAlign = "center"; ctx.textBaseline = "middle"; ctx.lineJoin = "round"; ctx.globalAlpha = 1;
      for (const {city, x, y, radius, symbolBox} of projected) {
        symbol(ctx, city, x, y, radius, ratio, zoom, priority(city) > 0);
        const hitSize = (Number.isFinite(settings.hitSize) ? clamp(settings.hitSize, 16, 44) : 28) * ratio;
        const hit = {city, point: [x, y], radius, symbolBox,
          markerBox: [symbolBox[0], symbolBox[1], Math.max(hitSize, symbolBox[2]), Math.max(hitSize, symbolBox[3])], labelBox: null,
          caption: priority(city) ? population(city) : null};
        hits.push(hit);
        if (settings.labels === false || labels >= labelLimit) continue;
        const nameSize = clamp(11.5 + Math.log2(Math.max(zoom, 1)) * .9, 11.5, 17) * ratio;
        const nameFont = `${priority(city) || city.capital ? 600 : 500} ${nameSize}px Inter, system-ui, sans-serif`;
        const captionSize = clamp(nameSize * .74, 10.5 * ratio, 12 * ratio);
        const measure = (text, font, size) => {
          ctx.font = font; const metrics = ctx.measureText(text);
          return {width: 2 * Math.max(metrics.width / 2, metrics.actualBoundingBoxLeft || 0, metrics.actualBoundingBoxRight || 0) + 6 * ratio,
            height: 2 * Math.max(size * .6, metrics.actualBoundingBoxAscent || 0, metrics.actualBoundingBoxDescent || 0) + 4 * ratio};
        };
        const name = measure(city.name, nameFont, nameSize);
        const captionFont = `400 ${captionSize}px Inter, system-ui, sans-serif`;
        const caption = hit.caption ? measure(hit.caption, captionFont, captionSize) : null;
        const boxWidth = Math.max(name.width, caption?.width || 0), boxHeight = name.height + (caption ? caption.height + 1 * ratio : 0);
        const gap = 6 * ratio, [sx, sy, sw, sh] = symbolBox;
        const candidates = [[sx + sw/2 + gap + boxWidth/2, sy], [sx - sw/2 - gap - boxWidth/2, sy],
          [sx, sy - sh/2 - gap - boxHeight/2], [sx, sy + sh/2 + gap + boxHeight/2]];
        for (const [cx, cy] of candidates) {
          if (cx - boxWidth / 2 < padding || cx + boxWidth / 2 > width - padding || cy - boxHeight / 2 < padding || cy + boxHeight / 2 > height - padding) continue;
          if (placed.some(box => Math.abs(box[0] - cx) < (box[2] + boxWidth) / 2 + padding &&
              Math.abs(box[1] - cy) < ((box[3] || nameSize) + boxHeight) / 2 + padding)) continue;
          // Reserve every visible settlement before laying out names so a
          // lower-ranked marker cannot later punch through a higher-ranked name.
          if (projected.some(other => other.city !== city && Math.abs(other.symbolBox[0] - cx) < (boxWidth + other.symbolBox[2]) / 2 + padding &&
              Math.abs(other.symbolBox[1] - cy) < (boxHeight + other.symbolBox[3]) / 2 + padding)) continue;
          const box = [cx, cy, boxWidth, boxHeight];
          const nameY = cy - (caption ? (caption.height + ratio) / 2 : 0);
          if (caption) {
            ctx.fillStyle = "#11242dec"; ctx.fillRect(cx - boxWidth / 2, cy - boxHeight / 2, boxWidth, boxHeight);
          }
          ctx.font = nameFont; ctx.lineWidth = 3 * ratio; ctx.strokeStyle = "#0b1922ee";
          ctx.fillStyle = city.capital ? "#ffe1aa" : "#e1eced";
          ctx.strokeText(city.name, cx, nameY); ctx.fillText(city.name, cx, nameY);
          if (caption) {
            ctx.font = captionFont; ctx.fillStyle = "#b3c9cc";
            const captionY = cy + (name.height + ratio) / 2;
            ctx.strokeText(hit.caption, cx, captionY); ctx.fillText(hit.caption, cx, captionY);
          }
          placed.push(box); hit.labelBox = box; labels++; break;
        }
      }
    } finally { ctx.restore(); }
    return {shown: hits.length, hits};
  }

  function pick(frame, x, y) {
    if (!Number.isFinite(x) || !Number.isFinite(y) || !Array.isArray(frame?.hits)) return null;
    const contains = box => box && Math.abs(x - box[0]) <= box[2] / 2 && Math.abs(y - box[1]) <= box[3] / 2;
    let chosen = null, best = Infinity;
    for (const hit of frame.hits) {
      const distance = Math.hypot(x - hit.point[0], y - hit.point[1]);
      const onSymbol = hit.symbolBox ? contains(hit.symbolBox) : distance <= hit.radius + 2;
      const score = onSymbol ? distance : contains(hit.labelBox) ? 1000 + distance : contains(hit.markerBox) ? 2000 + distance : Infinity;
      if (score < best) { best = score; chosen = hit.city; }
    }
    return chosen;
  }

  function activate(frame, x, y, onSelect) {
    const city = pick(frame, x, y);
    if (city && typeof onSelect === "function") onSelect(city);
    return city;
  }
  window.CityDetail = Object.freeze({id, build, draw, pick, activate, search});
})();
