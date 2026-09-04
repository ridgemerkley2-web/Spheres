/* Campaign infographic. Read-only presentation of /api/history; no commands,
   fabricated observations, forecasts, or new simulation state. */
const CHRONICLE = { view: "sheet", nation: null, days: 0, cursor: null, observer: null, model: null };
const CHRONICLE_COLORS = { gdp: "mint", growth: "mint", inflation: "rose", debt: "gold", stability: "lilac", mil: "blue" };
const CHRONICLE_COPY = {
  gdp: ["The economic engine", "Annual national output"],
  growth: ["The growth pulse", "Smoothed annualized growth rate"],
  inflation: ["The cost of living", "Annual inflation rate"],
  debt: ["The national balance", "Debt as a share of GDP"],
  stability: ["The home front", "Stability · 0–100 score"],
  mil: ["Your weight in the world", "Military strength · model index"],
};

function setChronicleView(view) {
  if (!["sheet", "compare"].includes(view)) return;
  CHRONICLE.view = view;
  CHRONICLE.observer?.disconnect();
  renderCharts();
  document.querySelector(view === "sheet" ? "#chronicleNation" : "#chronicleBack")?.focus({ preventScroll: true });
}

function chronicleValue(key, value) {
  if (!Number.isFinite(value)) return "—";
  if (key === "gdp") return fmt.money(value);
  if (["growth", "inflation", "debt"].includes(key)) return (value * 100).toLocaleString("en-US", { maximumFractionDigits: 2 }) + "%";
  return value.toLocaleString("en-US", { maximumFractionDigits: 2 });
}

// Headline money stays compact; a chart's bounds need enough shared precision
// to explain a real change. Choose one currency unit for the entire scale.
function chronicleAxisValue(key, value, lo, hi) {
  if (![value, lo, hi].every(Number.isFinite)) return "—";
  const rate = ["growth", "inflation", "debt"].includes(key);
  const magnitude = Math.max(Math.abs(lo), Math.abs(hi));
  let scale = rate ? 100 : 1, prefix = "", suffix = rate ? "%" : "";
  if (key === "gdp") {
    const unit = magnitude >= 1000 ? [1e-3, "tn"] : magnitude >= 1 ? [1, "bn"]
      : magnitude >= 1e-3 ? [1e3, "m"] : magnitude >= 1e-6 ? [1e6, "k"] : [1e9, ""];
    [scale, suffix] = unit; prefix = "$";
  }
  const low = lo * scale, high = hi * scale, current = value * scale;
  if (![low, high, current].every(Number.isFinite)) return "—";
  const span = Math.abs(high - low);
  const initial = span > 0 ? Math.max(0, Math.ceil(-Math.log10(span)) + 1) : 2;
  const honest = format => (lo === hi || format(low) !== format(high))
    && [low, high, current].every(n => n === 0 || Number(format(n)) !== 0);
  for (let digits = Math.min(12, initial); digits <= 12; digits++) {
    const format = n => n.toLocaleString("en-US", { useGrouping: false, maximumFractionDigits: digits });
    if (honest(format)) {
      const label = format(current);
      if (label.length <= 14) return prefix + label + suffix;
      break;
    }
  }
  // Very small/large values remain legible without becoming zero. Precision
  // rises only until the two finite scale bounds actually read differently.
  for (let digits = 2; digits <= 16; digits++) {
    const format = n => n.toExponential(digits).replace(/(\.\d*?[1-9])0+e/, "$1e").replace(/\.0+e/, "e");
    if (honest(format)) return prefix + format(current) + suffix;
  }
  return prefix + String(current) + suffix;
}

function chronicleChange(key, first, last) {
  const delta = chronicleDelta(key, first, last);
  if (!delta) return { arrow: "·", text: "Change unavailable" };
  const magnitude = Math.abs(delta.value);
  const unit = delta.kind === "percent" ? "%" : delta.kind === "pp" ? " pp" : " pts";
  if (magnitude < 1e-10) return { arrow: "→", text: "No change" };
  const number = magnitude < .01 ? "<0.01" : magnitude.toLocaleString("en-US", { maximumFractionDigits: 2 });
  return { arrow: delta.value > 0 ? "↑" : "↓", text: number + unit };
}

// A pixel bucket retains its first/last and extrema. Missing observations break
// the line; neither downsampling nor the shared cursor fills a data gap.
function chronicleThin(points, columns = 160) {
  if (points.length <= columns * 4) return points;
  const size = Math.ceil(points.length / columns), result = [];
  for (let i = 0; i < points.length; i += size) {
    const bucket = points.slice(i, i + size);
    if (bucket.some(p => !Number.isFinite(p.value))) { result.push(...bucket); continue; }
    let min = bucket[0], max = bucket[0];
    for (const p of bucket) { if (p.value < min.value) min = p; if (p.value > max.value) max = p; }
    const chosen = [...new Map([bucket[0], min, max, bucket[bucket.length - 1]].map(p => [p.i, p])).values()];
    result.push(...chosen.sort((a, b) => a.i - b.i));
  }
  return result;
}

function chronicleChart(key, points, model, width) {
  const W = Math.max(230, Math.floor(width)), H = 176, right = W - 12, top = 15, bottom = H - 36;
  const valid = points.filter(p => Number.isFinite(p.value));
  if (!valid.length) return `<div class="chr-no-series">No recorded values in this period.</div>`;
  let lo = Infinity, hi = -Infinity;
  for (const p of valid) { lo = Math.min(lo, p.value); hi = Math.max(hi, p.value); }
  const span = hi - lo, padding = span > 0 ? span * .12 : Math.max(Math.abs(hi) * .025, .01);
  lo -= padding; hi += padding;
  if (valid.every(p => p.value >= 0)) lo = Math.max(0, lo);
  const axisHi = chronicleAxisValue(key, hi, lo, hi), axisLo = chronicleAxisValue(key, lo, lo, hi);
  // Reserve real room for precise labels instead of clipping narrow GDP ranges.
  const left = Math.min(W - 100, Math.max(66, Math.ceil(Math.max(axisHi.length, axisLo.length) * 6.6 + 8)));
  const startTime = model.times[model.range.start], endTime = model.times[model.range.end];
  const X = i => endTime === startTime ? (left + right) / 2 : left + (model.times[i] - startTime) / (endTime - startTime) * (right - left);
  const Y = v => bottom - (v - lo) / (hi - lo) * (bottom - top);
  let path = "", pen = false;
  for (const p of chronicleThin(points, Math.max(40, Math.floor((right - left) / 2)))) {
    if (!Number.isFinite(p.value)) { pen = false; continue; }
    path += `${pen ? "L" : "M"}${X(p.i).toFixed(2)},${Y(p.value).toFixed(2)} `; pen = true;
  }
  const current = points.find(p => p.i === model.cursor);
  const marker = current && Number.isFinite(current.value)
    ? `<line class="chr-cursor" x1="${X(current.i)}" x2="${X(current.i)}" y1="${top}" y2="${bottom}"/><circle class="chr-dot" cx="${X(current.i)}" cy="${Y(current.value)}" r="5"/>` : "";
  const first = points[0], last = points[points.length - 1];
  const firstDateX = (labelAt(first.i).length + labelAt(last.i).length) * 6 > right - left ? 0 : left;
  const desc = `${CHRONICLE_METRICS[key].label}. ${labelAt(first.i)} to ${labelAt(last.i)}. ${valid.length} recorded values. Vertical scale is independent for each metric.`;
  return `<svg class="chr-chart" viewBox="0 0 ${W} ${H}" width="${W}" height="${H}" role="img" aria-label="${escText(desc)}">
    <line class="chr-grid" x1="${left}" x2="${right}" y1="${top}" y2="${top}"/><line class="chr-grid" x1="${left}" x2="${right}" y1="${bottom}" y2="${bottom}"/>
    <text class="chr-axis" x="${left - 8}" y="${top + 5}" text-anchor="end">${escText(axisHi)}</text>
    <text class="chr-axis" x="${left - 8}" y="${bottom + 4}" text-anchor="end">${escText(axisLo)}</text>
    <path class="chr-line" d="${path.trim()}"/>
    ${valid.length === 1 ? `<circle class="chr-dot" cx="${X(valid[0].i)}" cy="${Y(valid[0].value)}" r="4"/>` : ""}${marker}
    <text class="chr-axis" x="${firstDateX}" y="${H - 10}">${escText(labelAt(first.i))}</text>
    ${last.i !== first.i ? `<text class="chr-axis" x="${right}" y="${H - 10}" text-anchor="end">${escText(labelAt(last.i))}</text>` : ""}
  </svg>`;
}

function paintChronicle() {
  const model = CHRONICLE.model, root = document.querySelector("#chronicleSheet");
  if (!root || !model || model.range.start == null) return;
  const date = labelAt(model.cursor);
  root.querySelector("#chronicleDateText").textContent = date;
  const slider = root.querySelector("#chronicleDate");
  slider.value = String(model.cursor); slider.setAttribute("aria-valuetext", date);
  root.querySelector("#chronicleAtLatest").textContent = model.cursor === model.range.end ? "Latest recorded snapshot" : "Reading an earlier snapshot";
  for (const key of Object.keys(CHRONICLE_METRICS)) {
    const card = root.querySelector(`[data-chronicle-metric="${key}"]`);
    const points = model.points[key], first = points[0]?.value;
    const now = points.find(p => p.i === model.cursor)?.value;
    const change = chronicleChange(key, first, now);
    card.querySelector(".chr-number").textContent = chronicleValue(key, now);
    card.querySelector(".chr-change-arrow").textContent = change.arrow;
    card.querySelector(".chr-change-value").textContent = model.cursor === model.range.start && Number.isFinite(now) ? "Starting point" : change.text;
    card.querySelector(".chr-baseline").textContent = Number.isFinite(now) ? `from ${chronicleValue(key, first)} · ${labelAt(model.range.start)}` : "No observation recorded for this date";
    const plot = card.querySelector(".chr-plot");
    plot.innerHTML = chronicleChart(key, points, model, plot.clientWidth || 330);
  }
}

function renderChronicle() {
  CHRONICLE.observer?.disconnect();
  const box = document.querySelector("#pane-charts");
  const activeId = box.contains(document.activeElement) ? document.activeElement.id : null;
  const nations = Object.entries(HIST?.nations || {}).sort((a,b) => a[1].name.localeCompare(b[1].name));
  if (!nations.length) {
    box.innerHTML = `<div class="arc-empty"><strong>Your story starts here.</strong>Advance time to build a history. Recorded snapshots will appear here as your campaign unfolds.</div>`;
    CHRONICLE.model = null; return;
  }
  if (!HIST.nations[CHRONICLE.nation]) CHRONICLE.nation = HIST.nations[S.player] ? S.player : nations[0][0];
  const nation = HIST.nations[CHRONICLE.nation], range = chronicleRange(HIST, CHRONICLE.nation, CHRONICLE.days);
  // Pin the date, not the retained-array offset: old observations are trimmed.
  const pinned = CHRONICLE.cursor == null ? -1 : HIST.labels.indexOf(CHRONICLE.cursor);
  const cursor = range.start == null ? null : CHRONICLE.cursor == null ? range.end : Math.max(range.start, Math.min(range.end, pinned < 0 ? range.start : pinned));
  const points = Object.fromEntries(Object.keys(CHRONICLE_METRICS).map(key => [key, range.start == null ? [] : chroniclePoints(HIST, CHRONICLE.nation, key, range.start, range.end)]));
  const ordinals = HIST.labels.map(chronicleOrdinal);
  // Old/corrupt labels cannot put one point on a different kind of time axis.
  const times = ordinals.every(Number.isFinite) ? ordinals : HIST.t.every(Number.isFinite) ? HIST.t : HIST.t.map((_,i) => i);
  CHRONICLE.model = { range, cursor, points, times };
  const archived = range.end != null && range.end < HIST.t.length - 1;
  const first = range.start == null ? "—" : labelAt(range.start), last = range.end == null ? "—" : labelAt(range.end);
  const periodLabel = CHRONICLE.days ? `Last ${CHRONICLE.days.toLocaleString("en-US")} days of the record` : "All recorded history";
  box.innerHTML = `<section id="chronicleSheet" class="chronicle" aria-label="Campaign infographic">
    <header class="chr-heading"><div><div class="arc-kicker">The campaign chronicle · Over time</div><h1>A nation in motion.</h1><p>Six perspectives on ${escText(nation.name)}. One timeline.</p></div><div class="chr-orbit" aria-hidden="true"><span>↗</span><i></i></div></header>
    <div class="chr-toolbar"><label for="chronicleNation">Follow a nation<select id="chronicleNation">${nations.map(([id,n]) => `<option value="${escText(id)}"${id === CHRONICLE.nation ? " selected" : ""}>${escText(n.name)}</option>`).join("")}</select></label>
      <div class="chr-period"><span>Time window</span><div role="group" aria-label="History time window">${[[30,"30 days"],[365,"1 year"],[1825,"5 years"],[0,"All time"]].map(([days,label]) => `<button id="chroniclePeriod${days}" type="button" data-chronicle-days="${days}" aria-pressed="${days === CHRONICLE.days}">${label}</button>`).join("")}</div></div>
      <button id="chronicleCompare" type="button">Compare nations ↗</button></div>
    ${range.start == null ? `<div class="arc-empty"><strong>No recorded history.</strong>This nation's observations are not available yet.</div>` : `
    <div class="chr-timeline"><div class="chr-time-top"><label for="chronicleDate">Read the timeline<small id="chronicleAtLatest">Latest recorded snapshot</small></label><output id="chronicleDateText" for="chronicleDate">${escText(labelAt(cursor))}</output></div>
      <input id="chronicleDate" type="range" min="${range.start}" max="${range.end}" step="1" value="${cursor}"${range.start === range.end ? " disabled" : ""} aria-valuetext="${escText(labelAt(cursor))}">
      <div class="chr-time-ends"><span>${escText(first)}</span><span>${escText(last)}</span></div>
      <p>${archived ? "Archived nation · the record ends here. " : ""}${range.start === range.end ? "One snapshot so far. Advance time to build a history." : "Move the date slider to read all six metrics together."}</p></div>
    <div class="chr-metric-grid">${Object.entries(CHRONICLE_METRICS).map(([key,spec]) => `<article class="chr-metric chr-${CHRONICLE_COLORS[key]}" data-chronicle-metric="${key}" aria-labelledby="chrTitle-${key}"><div class="chr-card-top"><span>${CHRONICLE_COPY[key][0]}</span><span class="chr-card-mark" aria-hidden="true">${key === "gdp" ? "◈" : key === "growth" ? "↗" : key === "inflation" ? "≋" : key === "debt" ? "▤" : key === "stability" ? "◎" : "⚑"}</span></div><h2 id="chrTitle-${key}">${escText(spec.label)}</h2><div class="chr-number">—</div><p class="chr-unit">${CHRONICLE_COPY[key][1]}</p><div class="chr-change"><span class="chr-change-arrow"></span><strong class="chr-change-value"></strong></div><p class="chr-baseline"></p><div class="chr-plot"></div></article>`).join("")}</div>`}
    <footer class="chr-foot"><span>${escText(periodLabel)} · ${escText(first)} — ${escText(last)}</span><details><summary>How to read this sheet</summary><p>Each chart has its own labeled scale. Changes compare the selected date with the first recorded date in the window. Percentage-point changes (pp) are not percentage growth. Missing values stay empty; a nation's line ends when its record does. Growth is the recorded smoothed annual rate, not GDP change over this window. History retains up to 3,000 snapshots and restarts after loading a save. Figures are recorded observations, not forecasts.</p></details></footer>
  </section>`;
  document.querySelector("#chronicleNation").onchange = e => { CHRONICLE.nation = e.target.value; CHRONICLE.cursor = null; renderChronicle(); };
  box.querySelectorAll("[data-chronicle-days]").forEach(b => { b.onclick = () => { CHRONICLE.days = +b.dataset.chronicleDays; CHRONICLE.cursor = null; renderChronicle(); }; });
  document.querySelector("#chronicleCompare").onclick = () => setChronicleView("compare");
  const slider = document.querySelector("#chronicleDate");
  if (slider) slider.oninput = () => { CHRONICLE.cursor = +slider.value === range.end ? null : HIST.labels[+slider.value]; CHRONICLE.model.cursor = +slider.value; paintChronicle(); };
  paintChronicle();
  if (typeof ResizeObserver !== "undefined") {
    let lastWidth = box.clientWidth;
    CHRONICLE.observer = new ResizeObserver(() => { if (box.clientWidth > 0 && box.clientWidth !== lastWidth) { lastWidth = box.clientWidth; paintChronicle(); } });
    CHRONICLE.observer.observe(box);
  }
  if (activeId) document.getElementById(activeId)?.focus({ preventScroll: true });
}
