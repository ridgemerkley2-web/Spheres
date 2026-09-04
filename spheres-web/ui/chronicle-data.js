/* Read-only history presentation helpers. No simulation coefficients or dates
   are invented here: labels and nation lifespans come from /api/history. */
const CHRONICLE_METRICS = Object.freeze({
  gdp: Object.freeze({ label: "Economic output", unit: "$bn / year", deltaKind: "percent" }),
  growth: Object.freeze({ label: "Growth", unit: "% / year", deltaKind: "pp" }),
  inflation: Object.freeze({ label: "Inflation", unit: "% / year", deltaKind: "pp" }),
  debt: Object.freeze({ label: "Debt / GDP", unit: "% of GDP", deltaKind: "pp" }),
  stability: Object.freeze({ label: "Stability", unit: "points / 100", deltaKind: "points" }),
  mil: Object.freeze({ label: "Military strength", unit: "strength", deltaKind: "percent" }),
});

// An ordinal is a UTC calendar day, not a snapshot index or a rounded month.
// Legacy month-only snapshots describe the first day of the named month.
function chronicleOrdinal(label) {
  if (typeof label !== "string") return null;
  const match = /^(?:(\d{1,2})\s+)?([A-Za-z]+)\s+(\d{1,4})$/.exec(label.trim());
  if (!match) return null;
  const months = ["january", "february", "march", "april", "may", "june", "july", "august", "september", "october", "november", "december"];
  const name = match[2].toLowerCase();
  const month = months.findIndex(value => value === name || value.slice(0, 3) === name);
  const day = match[1] === undefined ? 1 : Number(match[1]);
  const year = Number(match[3]);
  if (month < 0 || day < 1 || day > 31 || year < 1) return null;
  const date = new Date(0);
  date.setUTCFullYear(year, month, day);
  date.setUTCHours(0, 0, 0, 0);
  if (date.getUTCFullYear() !== year || date.getUTCMonth() !== month || date.getUTCDate() !== day) return null;
  return date.getTime() / 86400000;
}

// GDP is the authoritative recorded lifespan when present. A missing GDP
// column may fall back to the other recorded series; a short metric column
// does not stretch a dead nation or truncate the other metrics.
function chronicleSpan(history, nationId) {
  const nation = history && history.nations && history.nations[nationId];
  if (!nation || !Array.isArray(history.t) || !history.t.length ||
      !Number.isInteger(nation.t0) || nation.t0 < 0) return null;
  const length = Array.isArray(nation.gdp) && nation.gdp.length
    ? nation.gdp.length
    : Math.max(0, ...Object.keys(CHRONICLE_METRICS).map(key => Array.isArray(nation[key]) ? nation[key].length : 0));
  const start = nation.t0;
  const end = Math.min(history.t.length - 1, start + length - 1);
  return end >= start ? { start, end } : null;
}

function chronicleRange(history, nationId, days) {
  const empty = () => ({ start: null, end: null, indices: [] });
  const span = chronicleSpan(history, nationId);
  if (!span || !Number.isInteger(days) || days < 0) return empty();
  let cutoff = null;
  if (days > 0) {
    const endDay = chronicleOrdinal(history.labels && history.labels[span.end]);
    if (endDay === null) return empty();
    cutoff = endDay - days + 1;
  }
  const indices = [];
  for (let i = span.start; i <= span.end; i++) {
    if (cutoff === null) indices.push(i);
    else {
      const day = chronicleOrdinal(history.labels && history.labels[i]);
      if (day !== null && day >= cutoff) indices.push(i);
    }
  }
  return indices.length ? { start: indices[0], end: indices[indices.length - 1], indices } : empty();
}

function chroniclePoints(history, nationId, key, start, end) {
  const span = chronicleSpan(history, nationId);
  if (!span || !Object.hasOwn(CHRONICLE_METRICS, key) ||
      !Number.isInteger(start) || !Number.isInteger(end) || start > end) return [];
  const nation = history.nations[nationId];
  const values = Array.isArray(nation[key]) ? nation[key] : [];
  const points = [];
  for (let i = Math.max(start, span.start); i <= Math.min(end, span.end); i++) {
    const value = values[i - nation.t0];
    points.push({ i, value: Number.isFinite(value) ? value : null });
  }
  return points;
}

function chronicleDelta(key, first, last) {
  if (!Object.hasOwn(CHRONICLE_METRICS, key) || !Number.isFinite(first) || !Number.isFinite(last)) return null;
  const kind = CHRONICLE_METRICS[key].deltaKind;
  if (kind === "percent" && first === 0) return null;
  const value = kind === "percent" ? 100 * (last - first) / Math.abs(first)
    : kind === "pp" ? 100 * (last - first) : last - first;
  return Number.isFinite(value) ? { value, kind } : null;
}
