//! One-shot tool: turn Natural Earth GeoJSON into the baked SVG paths the UI
//! ships with, plus the sim's district roster. Run it only when the map data
//! needs regenerating:
//!
//!   cargo run --release -p spheres-web --bin mapgen --features mapgen -- \
//!       spheres-web/data/ne_10m_admin_0.geojson spheres-web/data/ne_10m_admin_1.geojson
//!
//! Writes spheres-web/ui/world.js (admin-0 countries), spheres-web/ui/districts.js
//! (admin-1 political districts, same projection/canvas), and
//! spheres-sim/data/districts.json (per-nation district ownership at the 1990
//! start, plus land adjacency between districts — BIBLE section 5 as amended
//! 2026-08-30 makes the district map tactical geography as well as political).
//! The outputs are committed, so the game itself never needs this tool or the
//! source data. Identity, ownership and adjacency are all transcribed from
//! Natural Earth geometry; nothing is invented. Adjacency goes into
//! districts.json ONLY — the UI derives fronts from sim data, not geometry.
//!
//! Since the terrain pass, districts.json also carries three merged keys per
//! district — `t` (terrain class), `f` (feature name, omitted when null) and
//! `riv` (river-crossed neighbour subset, omitted when empty) — read from
//! the committed tools/terrain/ outputs `spheres-web/data/
//! district_terrain.json` and `crossing_edges.json`. Run those generators
//! first (see tools/terrain/README.md); mapgen stays the sole writer of
//! districts.json and panics on any transcription gap.

use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::fmt::Write as _;

const W: f64 = 2400.0;
// Robinson: the compromise projection used on most printed world maps. Neither
// equal-area nor conformal, but it is the one that "looks like a map" — the
// high latitudes are not smeared the way equirectangular smears them.
const LAT_TOP: f64 = 83.0;
const LAT_BOT: f64 = -58.0;

/// Douglas–Peucker tolerances, in projected px. Tune against the byte budgets
/// asserted at the end of main().
const EPS_WORLD: f64 = 0.35;
const EPS_DISTRICT: f64 = 0.8;

/// Drop specks: at this scale a sub-2px island is noise, and there are
/// hundreds. The floor applies to SECONDARY rings only — a feature's largest
/// ring always survives, so Bahrain, Malta, Singapore and the Maldives get a
/// shape at 10m instead of being a name without one. That is intended: the
/// old flat floor silently erased whole micro-states.
const MIN_AREA_PX_WORLD: f64 = 4.0;
/// District speck floor applies to SECONDARY rings only: a district's largest
/// ring always survives, because a district must exist for ownership even if
/// it draws four points.
const MIN_AREA_PX_DISTRICT: f64 = 1.5;

/// Robinson's published table, at 5-degree steps from the equator to the pole.
/// X scales the length of each parallel, Y its distance from the equator.
const RX: [f64; 19] = [
    1.0000, 0.9986, 0.9954, 0.9900, 0.9822, 0.9730, 0.9600, 0.9427, 0.9216,
    0.8962, 0.8679, 0.8350, 0.7986, 0.7597, 0.7186, 0.6732, 0.6213, 0.5722, 0.5322,
];
const RY: [f64; 19] = [
    0.0000, 0.0620, 0.1240, 0.1860, 0.2480, 0.3100, 0.3720, 0.4340, 0.4958,
    0.5571, 0.6176, 0.6769, 0.7346, 0.7903, 0.8435, 0.8936, 0.9394, 0.9761, 1.0000,
];

fn interp(table: &[f64; 19], lat_abs: f64) -> f64 {
    let t = (lat_abs / 5.0).min(18.0);
    let i = t.floor() as usize;
    if i >= 18 {
        return table[18];
    }
    table[i] + (t - i as f64) * (table[i + 1] - table[i])
}

/// Radius chosen so a full 360 degrees of equator spans exactly W.
fn radius() -> f64 {
    W / (2.0 * 0.8487 * std::f64::consts::PI)
}

/// Signed vertical offset from the equator, positive north.
fn robinson_y(lat: f64) -> f64 {
    1.3523 * radius() * interp(&RY, lat.abs()) * if lat < 0.0 { -1.0 } else { 1.0 }
}

fn height() -> f64 {
    robinson_y(LAT_TOP) - robinson_y(LAT_BOT)
}

fn project(lon: f64, lat: f64) -> (f64, f64) {
    let lat = lat.clamp(LAT_BOT, LAT_TOP);
    let x = W / 2.0 + 0.8487 * radius() * interp(&RX, lat.abs()) * lon.to_radians();
    let y = robinson_y(LAT_TOP) - robinson_y(lat);
    (x, y)
}

// ---------------------------------------------------------------------------
// Geometry helpers
// ---------------------------------------------------------------------------

fn dist2(a: (f64, f64), b: (f64, f64)) -> f64 {
    let (dx, dy) = (a.0 - b.0, a.1 - b.1);
    dx * dx + dy * dy
}

/// Perpendicular distance from p to the line through a and b.
fn perp_dist(p: (f64, f64), a: (f64, f64), b: (f64, f64)) -> f64 {
    let (dx, dy) = (b.0 - a.0, b.1 - a.1);
    let l2 = dx * dx + dy * dy;
    if l2 == 0.0 {
        return dist2(p, a).sqrt();
    }
    ((p.0 - a.0) * dy - (p.1 - a.1) * dx).abs() / l2.sqrt()
}

/// Textbook recursive Douglas–Peucker on an open polyline; keeps endpoints.
/// The farthest-point tie-break is index-based so output is byte-stable.
fn dp(pts: &[(f64, f64)], eps: f64) -> Vec<(f64, f64)> {
    if pts.len() <= 2 {
        return pts.to_vec();
    }
    let (a, b) = (pts[0], pts[pts.len() - 1]);
    let (mut imax, mut dmax) = (0usize, -1.0f64);
    for (i, &p) in pts.iter().enumerate().take(pts.len() - 1).skip(1) {
        let d = perp_dist(p, a, b);
        if d > dmax {
            dmax = d;
            imax = i;
        }
    }
    if dmax <= eps {
        return vec![a, b];
    }
    let mut out = dp(&pts[..=imax], eps);
    out.pop();
    out.extend(dp(&pts[imax..], eps));
    out
}

/// Ramer–Douglas–Peucker on a closed ring: anchor at index 0 and at the point
/// farthest from it, simplify each half, rejoin. Tolerance in projected px.
fn simplify_ring(pts: &[(f64, f64)], eps: f64) -> Vec<(f64, f64)> {
    if pts.len() <= 4 {
        return pts.to_vec();
    }
    let far = (1..pts.len())
        .max_by(|&a, &b| {
            let da = dist2(pts[0], pts[a]);
            let db = dist2(pts[0], pts[b]);
            da.partial_cmp(&db).unwrap().then(a.cmp(&b)) // deterministic tie-break
        })
        .unwrap();
    let mut out = dp(&pts[..=far], eps);
    out.pop();
    out.extend(dp(&pts[far..], eps));
    out
}

/// Shoelace centroid and signed area of a projected ring.
fn shoelace(pts: &[(f64, f64)]) -> (f64, f64, f64) {
    let mut area = 0.0;
    let (mut cx, mut cy) = (0.0, 0.0);
    for i in 0..pts.len() {
        let (xa, ya) = pts[i];
        let (xb, yb) = pts[(i + 1) % pts.len()];
        let cross = xa * yb - xb * ya;
        area += cross;
        cx += (xa + xb) * cross;
        cy += (ya + yb) * cross;
    }
    area *= 0.5;
    if area.abs() < 1e-9 {
        // Degenerate sliver: fall back to the bbox centre so the centroid is
        // at least on the shape.
        let (x0, y0, x1, y1) = bbox(pts);
        return ((x0 + x1) / 2.0, (y0 + y1) / 2.0, 0.0);
    }
    (cx / (6.0 * area), cy / (6.0 * area), area)
}

fn bbox(pts: &[(f64, f64)]) -> (f64, f64, f64, f64) {
    let (mut x0, mut y0, mut x1, mut y1) = (f64::MAX, f64::MAX, f64::MIN, f64::MIN);
    for &(x, y) in pts {
        x0 = x0.min(x);
        y0 = y0.min(y);
        x1 = x1.max(x);
        y1 = y1.max(y);
    }
    (x0, y0, x1, y1)
}

fn bbox_area(pts: &[(f64, f64)]) -> f64 {
    let (x0, y0, x1, y1) = bbox(pts);
    (x1 - x0) * (y1 - y0)
}

/// Round to a tenth of a pixel and collapse consecutive points that round to
/// the same place — identical semantics to the pre-district emitter.
fn round_dedup(pts: &[(f64, f64)]) -> Vec<(f64, f64)> {
    let mut out: Vec<(f64, f64)> = Vec::with_capacity(pts.len());
    for &(x, y) in pts {
        let r = ((x * 10.0).round() / 10.0, (y * 10.0).round() / 10.0);
        if let Some(&l) = out.last() {
            if (r.0 - l.0).abs() < 0.05 && (r.1 - l.1).abs() < 0.05 {
                continue;
            }
        }
        out.push(r);
    }
    out
}

fn emit_ring(pts: &[(f64, f64)], out: &mut String) {
    for (i, &(x, y)) in pts.iter().enumerate() {
        let _ = write!(out, "{}{} {}", if i == 0 { "M" } else { "L" }, x, y);
    }
    out.push('Z');
}

/// Spherical-excess area of one lon/lat ring in km² (R = 6371.0), from the
/// UNSIMPLIFIED source geometry.
fn ring_area_sqkm(ring: &[(f64, f64)]) -> f64 {
    const R: f64 = 6371.0;
    let mut s = 0.0;
    for i in 0..ring.len() {
        let (l1, p1) = ring[i];
        let (l2, p2) = ring[(i + 1) % ring.len()];
        s += (l2.to_radians() - l1.to_radians()) * (p1.to_radians().sin() + p2.to_radians().sin());
    }
    (s * R * R / 2.0).abs()
}

/// Feature area: exterior minus holes, summed over polygons.
fn polys_area_sqkm(polys: &[Vec<Vec<(f64, f64)>>]) -> f64 {
    let mut total = 0.0;
    for poly in polys {
        if poly.is_empty() {
            continue;
        }
        let ext = ring_area_sqkm(&poly[0]);
        let holes: f64 = poly[1..].iter().map(|r| ring_area_sqkm(r)).sum();
        total += (ext - holes).max(0.0);
    }
    total
}

// ---------------------------------------------------------------------------
// GeoJSON plumbing
// ---------------------------------------------------------------------------

/// polygons -> rings -> lon/lat points.
fn geometry_polys(geom: &serde_json::Value) -> Vec<Vec<Vec<(f64, f64)>>> {
    let polys: Vec<&serde_json::Value> = match geom["type"].as_str() {
        Some("Polygon") => vec![&geom["coordinates"]],
        Some("MultiPolygon") => geom["coordinates"]
            .as_array()
            .map(|a| a.iter().collect())
            .unwrap_or_default(),
        _ => vec![],
    };
    let mut out = vec![];
    for poly in polys {
        let Some(rings) = poly.as_array() else { continue };
        let mut p = vec![];
        for ring in rings {
            let Some(r) = ring.as_array() else { continue };
            let pts: Vec<(f64, f64)> = r
                .iter()
                .filter_map(|c| {
                    let a = c.as_array()?;
                    Some((a.first()?.as_f64()?, a.get(1)?.as_f64()?))
                })
                .collect();
            if pts.len() >= 3 {
                p.push(pts);
            }
        }
        if !p.is_empty() {
            out.push(p);
        }
    }
    out
}

fn to_geo(polys: &[Vec<Vec<(f64, f64)>>]) -> geo::MultiPolygon<f64> {
    geo::MultiPolygon::new(
        polys
            .iter()
            .filter(|p| !p.is_empty())
            .map(|p| {
                let ext = geo::LineString::from(p[0].clone());
                let ints: Vec<geo::LineString<f64>> =
                    p[1..].iter().map(|r| geo::LineString::from(r.clone())).collect();
                geo::Polygon::new(ext, ints)
            })
            .collect(),
    )
}

fn from_geo(mp: &geo::MultiPolygon<f64>) -> Vec<Vec<(f64, f64)>> {
    let mut out = vec![];
    for poly in mp {
        out.push(poly.exterior().0.iter().map(|c| (c.x, c.y)).collect());
        for i in poly.interiors() {
            out.push(i.0.iter().map(|c| (c.x, c.y)).collect());
        }
    }
    out
}

/// Union the members' polygons in lon/lat before projection. Real-world
/// Natural Earth geometry can make boolean ops panic or come back empty; the
/// caller falls back to concatenating rings when this returns None.
fn union_members(members: &[&Vec<Vec<Vec<(f64, f64)>>>]) -> Option<Vec<Vec<(f64, f64)>>> {
    let res = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let mut acc = to_geo(members[0]);
        for m in &members[1..] {
            acc = geo::BooleanOps::union(&acc, &to_geo(m));
        }
        acc
    }))
    .ok()?;
    let rings = from_geo(&res);
    if rings.is_empty() {
        None
    } else {
        Some(rings)
    }
}

// ---------------------------------------------------------------------------
// TERRITORY parse: the UI's nation -> ISO3 map is the single source of which
// real-world codes each sim nation holds. Parsed from index.html with the
// same anchors the sim-side regression test uses — never a copy.
// ---------------------------------------------------------------------------

fn territory_map(html: &str) -> BTreeMap<String, Vec<String>> {
    let body = html
        .split_once("const TERRITORY = {")
        .expect("TERRITORY map present")
        .1
        .split_once("};")
        .expect("brace-terminated")
        .0;
    // Strip // comments first, then scan the whole body for `Ident : [ ... ]`
    // — entries span lines (USSR wraps).
    let clean: String = body
        .lines()
        .map(|l| l.split("//").next().unwrap())
        .collect::<Vec<_>>()
        .join("\n");
    let chars: Vec<char> = clean.chars().collect();
    let mut out = BTreeMap::new();
    let mut i = 0;
    while i < chars.len() {
        if chars[i].is_ascii_alphanumeric() || chars[i] == '_' {
            let start = i;
            while i < chars.len() && (chars[i].is_ascii_alphanumeric() || chars[i] == '_') {
                i += 1;
            }
            let ident: String = chars[start..i].iter().collect();
            let mut j = i;
            while j < chars.len() && chars[j].is_whitespace() {
                j += 1;
            }
            if j < chars.len() && chars[j] == ':' {
                let mut k = j + 1;
                while k < chars.len() && chars[k].is_whitespace() {
                    k += 1;
                }
                if k < chars.len() && chars[k] == '[' {
                    let mut end = k + 1;
                    while end < chars.len() && chars[end] != ']' {
                        end += 1;
                    }
                    let list: String = chars[k + 1..end].iter().collect();
                    let codes: Vec<String> =
                        list.split('"').skip(1).step_by(2).map(str::to_string).collect();
                    out.insert(ident, codes);
                    i = end + 1;
                    continue;
                }
            }
        } else {
            i += 1;
        }
    }
    out
}

// ---------------------------------------------------------------------------
// District identity
// ---------------------------------------------------------------------------

/// Lowercase; every non-alphanumeric run becomes one `-`; trimmed. ASCII
/// transliteration deliberately not attempted — stability over beauty.
fn slug(s: &str) -> String {
    let mut out = String::new();
    for c in s.to_lowercase().chars() {
        if c.is_ascii_alphanumeric() {
            out.push(c);
        } else if !out.ends_with('-') && !out.is_empty() {
            out.push('-');
        }
    }
    out.trim_end_matches('-').to_string()
}

/// `^[A-Z]{2}-[A-Z0-9]{1,3}$` — rejects the `SY-X01~` / `-99-` placeholders.
fn clean_iso_3166_2(s: &str) -> bool {
    let b = s.as_bytes();
    if b.len() < 4 || b.len() > 6 {
        return false;
    }
    b[0].is_ascii_uppercase()
        && b[1].is_ascii_uppercase()
        && b[2] == b'-'
        && b[3..].iter().all(|c| c.is_ascii_uppercase() || c.is_ascii_digit())
}

/// Countries whose admin-1 mesh is too fine for a strategy map; Natural Earth
/// carries its own `region` grouping for these and we aggregate by it —
/// transcribed, never invented. Countries without a region field stay as-is.
const AGGREGATE: [&str; 16] = [
    "AZE", "ESP", "FRA", "GBR", "HUN", "IRL", "ITA", "LKA", "LVA", "MKD", "MLT", "PHL", "SVN",
    "THA", "UGA", "VNM",
];

struct District {
    id: String,
    name: String,
    path: String,
    cx: f64,
    cy: f64,
    area_sqkm: f64,
}

/// Project all rings, simplify, and build the path + centroid. The largest
/// ring always survives; secondaries obey the speck floor.
fn district_geom(rings_ll: &[Vec<(f64, f64)>]) -> Option<(String, f64, f64)> {
    let proj: Vec<Vec<(f64, f64)>> = rings_ll
        .iter()
        .filter(|r| r.len() >= 3)
        .map(|r| r.iter().map(|&(lo, la)| project(lo, la)).collect())
        .collect();
    if proj.is_empty() {
        return None;
    }
    let largest = proj
        .iter()
        .enumerate()
        .max_by(|(i, a), (j, b)| {
            let (aa, ab) = (shoelace(a).2.abs(), shoelace(b).2.abs());
            aa.partial_cmp(&ab).unwrap().then(i.cmp(j))
        })
        .unwrap()
        .0;
    let mut d = String::new();
    for (i, pts) in proj.iter().enumerate() {
        if i != largest && bbox_area(pts) < MIN_AREA_PX_DISTRICT {
            continue;
        }
        let mut rd = round_dedup(&simplify_ring(pts, EPS_DISTRICT));
        if rd.len() < 3 {
            if i != largest {
                continue;
            }
            // The largest ring must exist for ownership: fall back to the
            // pre-collapse rounded points.
            rd = round_dedup(pts);
        }
        emit_ring(&rd, &mut d);
    }
    if d.is_empty() {
        emit_ring(&round_dedup(&proj[largest]), &mut d);
    }
    let (cx, cy, _) = shoelace(&proj[largest]);
    Some((d, cx, cy))
}

// ---------------------------------------------------------------------------
// World pass (admin-0)
// ---------------------------------------------------------------------------

/// `keep` overrides the speck floor and the degenerate-ring drop: the caller
/// passes it for a feature's largest ring, which must always draw.
fn ring_to_path(pts: &[(f64, f64)], keep: bool, out: &mut String) -> Option<(f64, f64, f64)> {
    if pts.len() < 3 {
        return None;
    }
    if !keep && bbox_area(pts) < MIN_AREA_PX_WORLD {
        return None;
    }
    // Shoelace, for area-weighted centroids and to rank a country's landmasses.
    let (cx, cy, area) = shoelace(pts);
    if !keep && area.abs() < 1e-9 {
        return None;
    }
    let mut rd = round_dedup(&simplify_ring(pts, EPS_WORLD));
    if rd.len() < 3 {
        if !keep {
            return None;
        }
        rd = round_dedup(pts);
    }
    emit_ring(&rd, out);
    Some((cx, cy, area.abs()))
}

/// Meridians and parallels, sampled finely enough to curve smoothly.
fn graticule() -> Vec<String> {
    let mut out = vec![];
    let mut lon = -180.0;
    while lon <= 180.0 {
        let mut d = String::new();
        let mut lat = LAT_BOT;
        while lat <= LAT_TOP {
            let (x, y) = project(lon, lat);
            let _ = write!(d, "{}{:.1} {:.1}", if d.is_empty() { "M" } else { "L" }, x, y);
            lat += 2.0;
        }
        out.push(d);
        lon += 30.0;
    }
    let mut lat = -40.0;
    while lat <= 80.0 {
        let mut d = String::new();
        let mut lon = -180.0;
        while lon <= 180.0 {
            let (x, y) = project(lon, lat);
            let _ = write!(d, "{}{:.1} {:.1}", if d.is_empty() { "M" } else { "L" }, x, y);
            lon += 4.0;
        }
        out.push(d);
        lat += 20.0;
    }
    out
}

fn world_pass(gj: &serde_json::Value) -> String {
    let mut paths: BTreeMap<String, String> = BTreeMap::new();
    let mut centroids: BTreeMap<String, (f64, f64, f64)> = BTreeMap::new();

    for f in gj["features"].as_array().expect("features") {
        let props = &f["properties"];
        // 10m admin-0 carries uppercase property names, admin-1 lowercase;
        // read both defensively.
        let code = props["ADM0_A3"]
            .as_str()
            .or_else(|| props["adm0_a3"].as_str())
            .or_else(|| props["ISO_A3"].as_str())
            .or_else(|| props["iso_a3"].as_str())
            .unwrap_or("")
            .to_string();
        // Antarctica is a wall of ice across the bottom of the frame and no
        // actor in this game; the projection is clipped short of it anyway.
        if code.is_empty() || code == "-99" || code == "ATA" {
            continue;
        }
        let mut d = String::new();
        let mut best: Option<(f64, f64, f64)> = None;
        let rings: Vec<Vec<(f64, f64)>> = geometry_polys(&f["geometry"])
            .into_iter()
            .flatten()
            .map(|ring| ring.iter().map(|&(lo, la)| project(lo, la)).collect())
            .collect();
        // The feature's largest ring always draws, floor or no floor — a
        // micro-state is a shape, not a rounding error.
        let largest = rings
            .iter()
            .enumerate()
            .max_by(|(i, a), (j, b)| {
                let (aa, ab) = (shoelace(a).2.abs(), shoelace(b).2.abs());
                aa.partial_cmp(&ab).unwrap().then(i.cmp(j))
            })
            .map(|(i, _)| i);
        for (i, pts) in rings.iter().enumerate() {
            if let Some((cx, cy, a)) = ring_to_path(pts, Some(i) == largest, &mut d) {
                // Label the biggest landmass, not the average of
                // scattered islands — otherwise the USA is labelled
                // somewhere in the Pacific.
                if best.is_none_or(|(_, _, ba)| a > ba) {
                    best = Some((cx, cy, a));
                }
            }
        }
        if d.is_empty() {
            continue;
        }
        paths.entry(code.clone()).and_modify(|e| e.push_str(&d)).or_insert(d);
        if let Some(b) = best {
            centroids
                .entry(code)
                .and_modify(|e| {
                    if b.2 > e.2 {
                        *e = b
                    }
                })
                .or_insert(b);
        }
    }

    let mut out = String::new();
    out.push_str("// Generated by `cargo run -p spheres-web --bin mapgen --features mapgen`.\n");
    out.push_str("// Source: Natural Earth 10m admin-0 countries (public domain).\n");
    out.push_str("// Robinson, clipped to 83N..58S. Do not hand-edit.\n");
    let _ = write!(out, "window.WORLD={{w:{},h:{:.1},countries:{{", W, height());
    for (i, (code, d)) in paths.iter().enumerate() {
        let _ = write!(out, "{}\"{}\":\"{}\"", if i > 0 { "," } else { "" }, code, d);
    }
    out.push_str("},centroids:{");
    for (i, (code, (cx, cy, _))) in centroids.iter().enumerate() {
        let _ = write!(out, "{}\"{}\":[{:.1},{:.1}]", if i > 0 { "," } else { "" }, code, cx, cy);
    }
    out.push_str("},graticule:[");
    for (i, g) in graticule().iter().enumerate() {
        let _ = write!(out, "{}\"{}\"", if i > 0 { "," } else { "" }, g);
    }
    // The edge of the projection itself — the curved envelope the map sits in.
    let mut frame = String::new();
    let mut lat = LAT_BOT;
    while lat <= LAT_TOP {
        let (x, y) = project(-180.0, lat);
        let _ = write!(frame, "{}{:.1} {:.1}", if frame.is_empty() { "M" } else { "L" }, x, y);
        lat += 2.0;
    }
    let mut lat = LAT_TOP;
    while lat >= LAT_BOT {
        let (x, y) = project(180.0, lat);
        let _ = write!(frame, "L{:.1} {:.1}", x, y);
        lat -= 2.0;
    }
    frame.push('Z');
    let _ = writeln!(out, "],frame:\"{}\"}};", frame);
    println!("{} countries in world.js ({:.0} KB)", paths.len(), out.len() as f64 / 1024.0);
    out
}

// ---------------------------------------------------------------------------
// District pass (admin-1)
// ---------------------------------------------------------------------------

struct Adm1Feature {
    adm1_code: String,
    iso_3166_2: Option<String>,
    name: String,
    region: Option<String>,
    polys: Vec<Vec<Vec<(f64, f64)>>>,
    area_sqkm: f64,
}

impl Adm1Feature {
    fn rings(&self) -> Vec<Vec<(f64, f64)>> {
        self.polys.iter().flat_map(|p| p.iter().cloned()).collect()
    }
    /// Projected shoelace centroid of the largest projected ring — the same
    /// rule countries use for labels.
    fn centroid_px(&self) -> (f64, f64) {
        let mut best: Option<((f64, f64), f64)> = None;
        for ring in self.rings() {
            let proj: Vec<(f64, f64)> = ring.iter().map(|&(lo, la)| project(lo, la)).collect();
            let (cx, cy, a) = shoelace(&proj);
            if best.is_none() || a.abs() > best.unwrap().1 {
                best = Some(((cx, cy), a.abs()));
            }
        }
        best.map(|(c, _)| c).unwrap_or((0.0, 0.0))
    }
}

/// Returns the drawable districts per country AND the raw (unprojected,
/// unsimplified) lon/lat rings per district id, for the adjacency pass. Raw,
/// because simplification is per-ring and would break the shared-vertex
/// sequences Natural Earth carries along common borders, and because
/// projection clamps latitude to 83N..58S which would collapse every
/// high-latitude edge onto the frame line (false neighbours across the top of
/// Canada and Russia).
fn district_pass(
    gj: &serde_json::Value,
    territory: &BTreeMap<String, Vec<String>>,
) -> (BTreeMap<String, Vec<District>>, BTreeMap<String, Vec<Vec<(f64, f64)>>>) {
    let roster_codes: BTreeSet<&str> =
        territory.values().flatten().map(String::as_str).collect();

    // Bucket features by country, sorted by adm1_code throughout.
    let mut by_adm0: BTreeMap<String, Vec<Adm1Feature>> = BTreeMap::new();
    for f in gj["features"].as_array().expect("features") {
        let props = &f["properties"];
        let adm0 = props["adm0_a3"]
            .as_str()
            .or_else(|| props["ADM0_A3"].as_str())
            .unwrap_or("")
            .to_string();
        if adm0.is_empty() || adm0 == "-99" || adm0 == "ATA" {
            continue;
        }
        // Scenery countries get no districts — the country layer already
        // draws them.
        if !roster_codes.contains(adm0.as_str()) {
            continue;
        }
        let polys = geometry_polys(&f["geometry"]);
        if polys.is_empty() {
            continue;
        }
        let adm1_code = props["adm1_code"].as_str().unwrap_or("").to_string();
        let iso = props["iso_3166_2"].as_str().map(str::to_string);
        let name = props["name"]
            .as_str()
            .filter(|s| !s.is_empty())
            .or_else(|| props["name_en"].as_str().filter(|s| !s.is_empty()))
            .unwrap_or(&adm1_code)
            .to_string();
        let region = props["region"]
            .as_str()
            .filter(|s| !s.is_empty())
            .map(str::to_string);
        let area_sqkm = polys_area_sqkm(&polys);
        by_adm0.entry(adm0).or_default().push(Adm1Feature {
            adm1_code,
            iso_3166_2: iso,
            name,
            region,
            polys,
            area_sqkm,
        });
    }
    for feats in by_adm0.values_mut() {
        feats.sort_by(|a, b| a.adm1_code.cmp(&b.adm1_code));
    }

    let mut by_country: BTreeMap<String, Vec<District>> = BTreeMap::new();
    // Raw lon/lat rings per district id, captured for the adjacency pass. Ids
    // are globally unique (the seen_ids panics below guarantee it), so one
    // entry per district even when districts.json repeats a record under a
    // federation and its successor.
    let mut raw_rings: BTreeMap<String, Vec<Vec<(f64, f64)>>> = BTreeMap::new();
    // Global id uniqueness guard: id -> adm1_code of first claimant.
    let mut seen_ids: BTreeMap<String, String> = BTreeMap::new();

    for (adm0, feats) in &by_adm0 {
        let mut districts: Vec<District> = vec![];
        if AGGREGATE.contains(&adm0.as_str()) {
            // Group by Natural Earth's own `region` — transcribed grouping.
            let mut groups: BTreeMap<String, Vec<usize>> = BTreeMap::new();
            for (i, f) in feats.iter().enumerate() {
                if let Some(r) = &f.region {
                    groups.entry(r.clone()).or_default().push(i);
                }
            }
            assert!(
                !groups.is_empty(),
                "{adm0} is in the AGGREGATE set but has no region fields"
            );
            // Region mean centroids (projected), over region-carrying members.
            let means: BTreeMap<String, (f64, f64)> = groups
                .iter()
                .map(|(r, idxs)| {
                    let (mut sx, mut sy) = (0.0, 0.0);
                    for &i in idxs {
                        let (x, y) = feats[i].centroid_px();
                        sx += x;
                        sy += y;
                    }
                    (r.clone(), (sx / idxs.len() as f64, sy / idxs.len() as f64))
                })
                .collect();
            // Null-region features go to the nearest region by squared px
            // distance; ties break by region name ascending (BTree order +
            // strict <).
            for (i, f) in feats.iter().enumerate() {
                if f.region.is_some() {
                    continue;
                }
                let c = f.centroid_px();
                let mut best: Option<(&String, f64)> = None;
                for (r, m) in &means {
                    let d = dist2(c, *m);
                    if best.is_none() || d < best.unwrap().1 {
                        best = Some((r, d));
                    }
                }
                groups.get_mut(best.unwrap().0).unwrap().push(i);
            }
            // Distinct region strings can slug identically (Natural Earth
            // ships VNM's "Đông Bắc" twice, once mojibake-encoded) — suffix
            // -2, -3, … in region-name (BTree) order, same rule as features.
            let mut used: BTreeMap<String, usize> = BTreeMap::new();
            for (region, mut idxs) in groups {
                idxs.sort(); // adm1_code order (feats already sorted)
                let member_polys: Vec<&Vec<Vec<Vec<(f64, f64)>>>> =
                    idxs.iter().map(|&i| &feats[i].polys).collect();
                let rings = union_members(&member_polys).unwrap_or_else(|| {
                    eprintln!(
                        "warning: union failed for {adm0} region {region}; concatenating member rings"
                    );
                    idxs.iter().flat_map(|&i| feats[i].rings()).collect()
                });
                let Some((path, cx, cy)) = district_geom(&rings) else {
                    continue;
                };
                let area: f64 = idxs.iter().map(|&i| feats[i].area_sqkm).sum();
                let base = format!("{}_{}", adm0, slug(&region));
                let n = used.entry(base.clone()).or_insert(0);
                *n += 1;
                let id = if *n == 1 { base } else { format!("{}-{}", base, n) };
                if let Some(prev) = seen_ids.insert(id.clone(), feats[idxs[0]].adm1_code.clone())
                {
                    panic!("duplicate district id {id}: {prev} vs {}", feats[idxs[0]].adm1_code);
                }
                raw_rings.insert(id.clone(), rings);
                districts.push(District { id, name: region, path, cx, cy, area_sqkm: area });
            }
        } else {
            // Per-feature districts. iso_3166_2 is the id only when clean AND
            // unique within its country; everything else slugs.
            let mut iso_count: BTreeMap<&str, usize> = BTreeMap::new();
            for f in feats {
                if let Some(iso) = f.iso_3166_2.as_deref() {
                    if clean_iso_3166_2(iso) {
                        *iso_count.entry(iso).or_insert(0) += 1;
                    }
                }
            }
            let mut used: BTreeMap<String, usize> = BTreeMap::new();
            for f in feats {
                let rings = f.rings();
                let Some((path, cx, cy)) = district_geom(&rings) else {
                    continue;
                };
                let base = match f.iso_3166_2.as_deref() {
                    Some(iso) if clean_iso_3166_2(iso) && iso_count[iso] == 1 => iso.to_string(),
                    _ => {
                        let s = slug(&f.name);
                        if s.is_empty() {
                            format!("{}_{}", adm0, slug(&f.adm1_code))
                        } else {
                            format!("{}_{}", adm0, s)
                        }
                    }
                };
                // Same-country collisions suffix -2, -3, … in adm1_code order
                // (feats are iterated in that order).
                let n = used.entry(base.clone()).or_insert(0);
                *n += 1;
                let id = if *n == 1 { base } else { format!("{}-{}", base, n) };
                if let Some(prev) = seen_ids.insert(id.clone(), f.adm1_code.clone()) {
                    panic!("duplicate district id {id}: {prev} vs {}", f.adm1_code);
                }
                raw_rings.insert(id.clone(), rings);
                districts.push(District {
                    id,
                    name: f.name.clone(),
                    path,
                    cx,
                    cy,
                    area_sqkm: f.area_sqkm,
                });
            }
        }
        districts.sort_by(|a, b| a.id.cmp(&b.id));
        if !districts.is_empty() {
            by_country.insert(adm0.clone(), districts);
        }
    }
    (by_country, raw_rings)
}

// ---------------------------------------------------------------------------
// Adjacency pass
// ---------------------------------------------------------------------------

/// Quantize a lon/lat vertex to integer 1e-4-degree steps (~11 m at the
/// equator): comfortably above f64/boolean-union noise, comfortably below the
/// Natural Earth 10m feature scale. Integers so the key is exactly orderable —
/// no float comparison anywhere in the pass. The antimeridian is normalized so
/// edges split at -180 and +180 (Chukotka, Fiji, the Aleutians) still match.
fn quant(p: (f64, f64)) -> (i64, i64) {
    let mut lon = (p.0 * 1e4).round() as i64;
    let lat = (p.1 * 1e4).round() as i64;
    if lon == -1_800_000 {
        lon = 1_800_000;
    }
    (lon, lat)
}

/// Quantized shared-EDGE detection over the raw admin-1 rings: two districts
/// are neighbours when they share at least one positive-length boundary
/// segment (two consecutive quantized vertices — so at least two shared
/// quantized boundary points, and on a shared polyline, not a corner). Edge
/// sharing rather than point sharing is deliberate: a single-point touch
/// (Four Corners: US-AZ/US-CO) is not a land border a front can cross.
///
/// Sea links: none. A theatre's access rules already model reach; islands
/// simply have no land neighbours and emit an empty list.
///
/// Deterministic: ids iterate in BTreeMap order, edge records sort on integer
/// tuples with the district index as final tie-break, and neighbour lists are
/// emitted sorted ascending.
fn adjacency_pass(
    raw_rings: &BTreeMap<String, Vec<Vec<(f64, f64)>>>,
) -> BTreeMap<String, Vec<String>> {
    let ids: Vec<&String> = raw_rings.keys().collect();
    // (edge a, edge b, district index), a <= b. A flat sorted Vec instead of
    // a map-of-sets: millions of edges, one allocation.
    let mut recs: Vec<((i64, i64), (i64, i64), u32)> = Vec::new();
    for (idx, rings) in raw_rings.values().enumerate() {
        for ring in rings {
            // Exteriors AND holes: an enclave borders its host along the
            // host's interior ring. Wraparound closes an unclosed ring; the
            // degenerate-edge skip kills the GeoJSON closing-point duplicate.
            for i in 0..ring.len() {
                let a = quant(ring[i]);
                let b = quant(ring[(i + 1) % ring.len()]);
                if a == b {
                    continue;
                }
                let (lo, hi) = if a <= b { (a, b) } else { (b, a) };
                recs.push((lo, hi, idx as u32));
            }
        }
    }
    recs.sort_unstable();
    let mut adj: BTreeMap<&str, BTreeSet<&str>> = BTreeMap::new();
    let mut i = 0;
    while i < recs.len() {
        let mut j = i + 1;
        while j < recs.len() && recs[j].0 == recs[i].0 && recs[j].1 == recs[i].1 {
            j += 1;
        }
        // Every distinct pair of districts on this edge becomes a neighbour
        // pair. The a != b filter matters for the AGGREGATE union-failure
        // fallback, where a merged district's internal borders survive as
        // doubled edges under one id.
        for x in i..j {
            for y in (x + 1)..j {
                let (da, db) = (recs[x].2, recs[y].2);
                if da != db {
                    let (na, nb) = (ids[da as usize].as_str(), ids[db as usize].as_str());
                    adj.entry(na).or_default().insert(nb);
                    adj.entry(nb).or_default().insert(na);
                }
            }
        }
        i = j;
    }
    // Every district gets a list, empty for islands — consumers must not
    // assume connectivity, or any neighbours at all.
    raw_rings
        .keys()
        .map(|id| {
            let ns = adj
                .get(id.as_str())
                .map(|s| s.iter().map(|n| n.to_string()).collect::<Vec<_>>())
                .unwrap_or_default();
            (id.clone(), ns)
        })
        .collect()
}

fn emit_districts_js(by_country: &BTreeMap<String, Vec<District>>) -> String {
    let mut out = String::new();
    out.push_str("// Generated by `cargo run -p spheres-web --bin mapgen --features mapgen`.\n");
    out.push_str("// Source: Natural Earth 10m admin-1 states and provinces (public domain).\n");
    out.push_str("// Robinson, clipped to 83N..58S, same canvas as world.js. Do not hand-edit.\n");
    let _ = write!(out, "window.DISTRICTS={{meta:{{w:{},h:{:.1}}},byCountry:{{", W, height());
    for (i, (code, ds)) in by_country.iter().enumerate() {
        let _ = write!(out, "{}\"{}\":[", if i > 0 { "," } else { "" }, code);
        for (j, d) in ds.iter().enumerate() {
            let _ = write!(
                out,
                "{}{{id:\"{}\",name:{},path:\"{}\",cx:{:.1},cy:{:.1}}}",
                if j > 0 { "," } else { "" },
                d.id,
                serde_json::to_string(&d.name).unwrap(),
                d.path,
                d.cx,
                d.cy
            );
        }
        out.push(']');
    }
    out.push_str("}};");
    out.push('\n');
    out
}

/// The terrain merge inputs — python outputs from tools/terrain/, committed
/// beside the Natural Earth data. mapgen stays the SOLE writer of
/// districts.json: python produces these intermediates, mapgen merges them
/// into every district record. A district missing a class, an unknown id in
/// either input, or a crossing pair that is not an adjacency edge is a
/// transcription failure, and transcription failures are build failures.
#[allow(clippy::type_complexity)]
fn terrain_merge_tables(
    adjacency: &BTreeMap<String, Vec<String>>,
) -> (
    BTreeMap<String, (String, Option<String>)>,
    BTreeMap<String, Vec<String>>,
) {
    let raw = std::fs::read_to_string("spheres-web/data/district_terrain.json")
        .expect("read district_terrain.json (run tools/terrain/classify_districts.py)");
    let v: serde_json::Value =
        serde_json::from_str(&raw).expect("parse district_terrain.json");
    let mut terrain: BTreeMap<String, (String, Option<String>)> = BTreeMap::new();
    for (id, e) in v.as_object().expect("district_terrain.json is an object") {
        assert!(
            adjacency.contains_key(id),
            "district_terrain.json names '{}', which mapgen does not emit",
            id
        );
        let t = e["t"].as_str().expect("terrain class string").to_string();
        let f = e["f"].as_str().map(str::to_string);
        terrain.insert(id.clone(), (t, f));
    }
    for id in adjacency.keys() {
        assert!(
            terrain.contains_key(id),
            "district '{}' is missing from district_terrain.json",
            id
        );
    }
    let raw = std::fs::read_to_string("spheres-web/data/crossing_edges.json")
        .expect("read crossing_edges.json (run tools/terrain/crossing_edges.py)");
    let v: serde_json::Value =
        serde_json::from_str(&raw).expect("parse crossing_edges.json");
    let edges = v["edges"].as_array().expect("edges array");
    assert_eq!(
        edges.len() as u64,
        v["count"].as_u64().expect("count"),
        "crossing_edges.json count disagrees with its edge list"
    );
    let mut river: BTreeMap<String, Vec<String>> = BTreeMap::new();
    for e in edges {
        let a = e[0].as_str().expect("edge district id").to_string();
        let b = e[1].as_str().expect("edge district id").to_string();
        assert!(
            adjacency.get(&a).is_some_and(|ns| ns.contains(&b)),
            "crossing pair ({}, {}) is not an adjacency edge",
            a,
            b
        );
        river.entry(a.clone()).or_default().push(b.clone());
        river.entry(b).or_default().push(a);
    }
    for ns in river.values_mut() {
        ns.sort();
        ns.dedup();
    }
    (terrain, river)
}

fn emit_districts_json(
    by_country: &BTreeMap<String, Vec<District>>,
    territory: &BTreeMap<String, Vec<String>>,
    adjacency: &BTreeMap<String, Vec<String>>,
    terrain: &BTreeMap<String, (String, Option<String>)>,
    river: &BTreeMap<String, Vec<String>>,
) -> String {
    let mut nations = serde_json::Map::new();
    for (nation, codes) in territory {
        let mut arr = vec![];
        for code in codes {
            if let Some(ds) = by_country.get(code) {
                for d in ds {
                    // Lookup by id, so a district repeated under a federation
                    // and its successor carries an identical adj array in
                    // every occurrence.
                    let adj = adjacency.get(&d.id).map(Vec::as_slice).unwrap_or(&[]);
                    let (t, f) = terrain
                        .get(&d.id)
                        .unwrap_or_else(|| panic!("no terrain for '{}'", d.id));
                    let mut rec = serde_json::json!({
                        "id": d.id,
                        "name": d.name,
                        "area_sqkm": (d.area_sqkm * 10.0).round() / 10.0,
                        "adj": adj,
                        "t": t,
                    });
                    // Both keys OMITTED rather than null/empty — the sim's
                    // serde defaults cover absence, and absence is smaller.
                    if let Some(f) = f {
                        rec["f"] = serde_json::json!(f);
                    }
                    if let Some(riv) = river.get(&d.id) {
                        rec["riv"] = serde_json::json!(riv);
                    }
                    arr.push(rec);
                }
            }
        }
        nations.insert(nation.clone(), serde_json::Value::Array(arr));
    }
    // The `meta` block is provenance only; the sim parses and ignores it.
    let root = serde_json::json!({
        "meta": {
            "generator": "spheres-web/src/bin/mapgen.rs (--features mapgen)",
            "sources": [
                "Natural Earth 10m admin-1 (identity, area, adjacency)",
                "tools/terrain/classify_districts.py (t, f)",
                "tools/terrain/crossing_edges.py (riv)"
            ]
        },
        "nations": nations
    });
    let mut s = serde_json::to_string_pretty(&root).expect("serialize districts.json");
    s.push('\n');
    s
}

fn main() {
    let (Some(admin0), Some(admin1)) = (std::env::args().nth(1), std::env::args().nth(2)) else {
        eprintln!("usage: mapgen <ne_10m_admin_0.geojson> <ne_10m_admin_1.geojson>");
        std::process::exit(1);
    };

    // TERRITORY comes from the UI itself — the one place the nation -> ISO
    // mapping lives.
    let html = std::fs::read_to_string("spheres-web/ui/index.html").expect("read ui/index.html");
    let territory = territory_map(&html);
    assert!(territory.len() >= 150, "TERRITORY parse too small: {} keys", territory.len());
    assert_eq!(territory["USA"], vec!["USA"]);
    assert_eq!(territory["USSR"].len(), 15, "USSR should hold 15 republics");
    assert_eq!(territory["Yugoslavia"].len(), 6, "Yugoslavia should hold 6 republics");

    // --- world.js ---
    let raw = std::fs::read_to_string(&admin0).expect("read admin-0 geojson");
    let gj: serde_json::Value = serde_json::from_str(&raw).expect("parse admin-0 geojson");
    let world = world_pass(&gj);
    drop(gj);
    assert!(world.len() < 3_200_000, "world.js over budget: raise EPS_WORLD");
    std::fs::write("spheres-web/ui/world.js", &world).expect("write world.js");
    println!("wrote spheres-web/ui/world.js ({:.0} KB)", world.len() as f64 / 1024.0);

    // --- districts.js + districts.json ---
    let raw = std::fs::read_to_string(&admin1).expect("read admin-1 geojson");
    let gj: serde_json::Value = serde_json::from_str(&raw).expect("parse admin-1 geojson");
    let (by_country, raw_rings) = district_pass(&gj, &territory);
    drop(gj);

    let djs = emit_districts_js(&by_country);
    assert!(djs.len() <= 6_500_000, "districts.js over budget: raise EPS_DISTRICT");
    std::fs::write("spheres-web/ui/districts.js", &djs).expect("write districts.js");
    let n_districts: usize = by_country.values().map(Vec::len).sum();
    println!(
        "{} districts in {} countries -> spheres-web/ui/districts.js ({:.0} KB)",
        n_districts,
        by_country.len(),
        djs.len() as f64 / 1024.0
    );

    let adjacency = adjacency_pass(&raw_rings);
    drop(raw_rings);
    let n_edges: usize = adjacency.values().map(Vec::len).sum::<usize>() / 2;
    let n_isolated = adjacency.values().filter(|v| v.is_empty()).count();
    println!("adjacency: {} land edges, {} isolated districts", n_edges, n_isolated);

    let (terrain, river) = terrain_merge_tables(&adjacency);
    let n_crossed: usize = river.values().map(Vec::len).sum::<usize>() / 2;
    println!("terrain merge: {} classed districts, {} river-crossed edges", terrain.len(), n_crossed);

    let djson = emit_districts_json(&by_country, &territory, &adjacency, &terrain, &river);
    std::fs::write("spheres-sim/data/districts.json", &djson).expect("write districts.json");
    println!(
        "{} nations -> spheres-sim/data/districts.json ({:.0} KB)",
        territory.len(),
        djson.len() as f64 / 1024.0
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simplify_ring_collapses_collinear_points() {
        let ring = vec![
            (0.0, 0.0),
            (5.0, 0.0),
            (10.0, 0.0),
            (10.0, 10.0),
            (0.0, 10.0),
            (0.0, 0.0),
        ];
        let out = simplify_ring(&ring, 0.5);
        assert!(!out.contains(&(5.0, 0.0)), "collinear midpoint should collapse: {out:?}");
        assert!(out.contains(&(10.0, 10.0)) && out.contains(&(0.0, 10.0)));
    }

    #[test]
    fn simplify_ring_keeps_a_square_verbatim() {
        let ring = vec![(0.0, 0.0), (10.0, 0.0), (10.0, 10.0), (0.0, 10.0), (0.0, 0.0)];
        assert_eq!(simplify_ring(&ring, 0.5), ring);
    }

    #[test]
    fn simplify_ring_is_deterministic_on_a_tie() {
        // Three points equidistant from the anchor: the farthest-point pick
        // must tie-break by index, so two runs agree byte-for-byte.
        let ring = vec![(0.0, 0.0), (8.0, 6.0), (10.0, 0.0), (8.0, -6.0), (0.0, 0.0)];
        let a = simplify_ring(&ring, 0.5);
        let b = simplify_ring(&ring, 0.5);
        assert_eq!(a, b);
        assert_eq!(a, ring, "well-separated corners all survive");
    }

    #[test]
    fn slug_flattens_names() {
        assert_eq!(slug("Balkh"), "balkh");
        assert_eq!(slug("Île-de-France"), "le-de-france");
        assert_eq!(slug("Some  Name!"), "some-name");
        assert_eq!(slug("--x--"), "x");
    }

    #[test]
    fn clean_iso_rejects_placeholders() {
        assert!(clean_iso_3166_2("AF-BAL"));
        assert!(clean_iso_3166_2("UA-30"));
        assert!(!clean_iso_3166_2("SY-X01~"));
        assert!(!clean_iso_3166_2("-99-"));
        assert!(!clean_iso_3166_2("af-bal"));
    }

    fn ring(pts: &[(f64, f64)]) -> Vec<Vec<(f64, f64)>> {
        let mut r: Vec<(f64, f64)> = pts.to_vec();
        r.push(pts[0]); // GeoJSON-style closing duplicate
        vec![r]
    }

    #[test]
    fn adjacency_shared_edge_yes_corner_touch_no() {
        let mut raw = BTreeMap::new();
        // A and B share the segment x=1, y in 0..1. C touches A only at the
        // single point (1,1) — Four Corners style, not a land border.
        raw.insert("A".to_string(), ring(&[(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)]));
        raw.insert("B".to_string(), ring(&[(1.0, 0.0), (2.0, 0.0), (2.0, 1.0), (1.0, 1.0)]));
        raw.insert("C".to_string(), ring(&[(1.0, 1.0), (2.0, 1.0), (2.0, 2.0), (1.0, 2.0)]));
        let adj = adjacency_pass(&raw);
        assert_eq!(adj["A"], vec!["B"], "A borders B along a shared segment");
        assert!(!adj["A"].contains(&"C".to_string()), "corner touch is not adjacency");
        assert_eq!(adj["B"], vec!["A", "C"], "B shares an edge with both");
    }

    #[test]
    fn adjacency_matches_a_border_walked_in_opposite_directions() {
        // Vertex-sequence sharing is the contract: D and E share the exact
        // quantized segment even though E's ring runs it in the opposite
        // direction (undirected key).
        let mut raw = BTreeMap::new();
        raw.insert("D".to_string(), ring(&[(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)]));
        raw.insert("E".to_string(), ring(&[(1.0, 1.0), (1.0, 0.0), (2.0, 0.0), (2.0, 1.0)]));
        let adj = adjacency_pass(&raw);
        assert_eq!(adj["D"], vec!["E"]);
        assert_eq!(adj["E"], vec!["D"]);
    }

    #[test]
    fn adjacency_matches_across_the_antimeridian_and_isolates_islands() {
        let mut raw = BTreeMap::new();
        // W ends at lon +180, X starts at lon -180: Natural Earth splits
        // geometry at the seam; quantization must rejoin it.
        raw.insert("W".to_string(), ring(&[(179.0, 0.0), (180.0, 0.0), (180.0, 1.0), (179.0, 1.0)]));
        raw.insert(
            "X".to_string(),
            ring(&[(-180.0, 0.0), (-179.0, 0.0), (-179.0, 1.0), (-180.0, 1.0)]),
        );
        // An island far away has no neighbours but still gets a (empty) list.
        raw.insert("ISL".to_string(), ring(&[(10.0, 10.0), (11.0, 10.0), (11.0, 11.0), (10.0, 11.0)]));
        let adj = adjacency_pass(&raw);
        assert_eq!(adj["W"], vec!["X"]);
        assert_eq!(adj["X"], vec!["W"]);
        assert!(adj["ISL"].is_empty(), "islands carry an empty list, not a missing one");
    }

    #[test]
    fn adjacency_hole_ring_binds_enclave_to_host() {
        let mut raw = BTreeMap::new();
        // HOST is a big square with a hole; ENC is the enclave filling the
        // hole exactly. The border exists only between HOST's interior ring
        // and ENC's exterior ring.
        let outer: Vec<(f64, f64)> =
            vec![(0.0, 0.0), (3.0, 0.0), (3.0, 3.0), (0.0, 3.0), (0.0, 0.0)];
        let hole: Vec<(f64, f64)> =
            vec![(1.0, 1.0), (2.0, 1.0), (2.0, 2.0), (1.0, 2.0), (1.0, 1.0)];
        raw.insert("HOST".to_string(), vec![outer, hole.clone()]);
        raw.insert("ENC".to_string(), vec![hole]);
        let adj = adjacency_pass(&raw);
        assert_eq!(adj["HOST"], vec!["ENC"]);
        assert_eq!(adj["ENC"], vec!["HOST"]);
    }

    #[test]
    fn adjacency_is_deterministic_across_runs() {
        let mut raw = BTreeMap::new();
        raw.insert("A".to_string(), ring(&[(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)]));
        raw.insert("B".to_string(), ring(&[(1.0, 0.0), (2.0, 0.0), (2.0, 1.0), (1.0, 1.0)]));
        raw.insert("C".to_string(), ring(&[(0.0, 1.0), (1.0, 1.0), (1.0, 2.0), (0.0, 2.0)]));
        assert_eq!(adjacency_pass(&raw), adjacency_pass(&raw));
    }

    #[test]
    fn the_terrain_merge_is_verbatim() {
        // The committed outputs, checked against each other: every district
        // districts.json emits carries a class from district_terrain.json,
        // every `riv` list is symmetric and a subset of `adj`, and the riv
        // edges are exactly crossing_edges.json's list — count included.
        // Paths are relative to the spheres-web package dir (test cwd).
        let dj: serde_json::Value = serde_json::from_str(
            &std::fs::read_to_string("../spheres-sim/data/districts.json").unwrap(),
        )
        .unwrap();
        let terr: serde_json::Value = serde_json::from_str(
            &std::fs::read_to_string("data/district_terrain.json").unwrap(),
        )
        .unwrap();
        let ce: serde_json::Value = serde_json::from_str(
            &std::fs::read_to_string("data/crossing_edges.json").unwrap(),
        )
        .unwrap();
        let mut adj: BTreeMap<&str, Vec<&str>> = BTreeMap::new();
        let mut riv: BTreeMap<&str, Vec<&str>> = BTreeMap::new();
        for list in dj["nations"].as_object().unwrap().values() {
            for d in list.as_array().unwrap() {
                let id = d["id"].as_str().unwrap();
                let t = d["t"].as_str().expect("every emitted district is classed");
                let te = &terr[id];
                assert_eq!(t, te["t"].as_str().unwrap(), "{} class drifted", id);
                assert_eq!(
                    d.get("f").and_then(|f| f.as_str()),
                    te["f"].as_str(),
                    "{} feature drifted",
                    id
                );
                let names = |key: &str| -> Vec<&str> {
                    d.get(key)
                        .and_then(|v| v.as_array())
                        .map(|a| a.iter().map(|x| x.as_str().unwrap()).collect())
                        .unwrap_or_default()
                };
                adj.insert(id, names("adj"));
                riv.insert(id, names("riv"));
            }
        }
        assert_eq!(adj.len(), terr.as_object().unwrap().len(), "census/terrain size");
        let mut riv_edges: BTreeSet<(&str, &str)> = BTreeSet::new();
        for (id, ns) in &riv {
            for n in ns {
                assert!(adj[id].contains(n), "{} riv {} outside adj", id, n);
                assert!(riv[n].contains(id), "{} riv {} one-way", id, n);
                let (a, b) = if id < n { (*id, *n) } else { (*n, *id) };
                riv_edges.insert((a, b));
            }
        }
        let want: BTreeSet<(&str, &str)> = ce["edges"]
            .as_array()
            .unwrap()
            .iter()
            .map(|e| (e[0].as_str().unwrap(), e[1].as_str().unwrap()))
            .collect();
        assert_eq!(riv_edges, want, "riv edges != crossing_edges.json");
        assert_eq!(riv_edges.len() as u64, ce["count"].as_u64().unwrap());
    }

    #[test]
    fn territory_map_parses_multiline_entries() {
        let html = r#"junk before
const TERRITORY = {
  USA: ["USA"], Russia: ["RUS"],
  // Bahrain has no polygon: `grep -o '"BHR"'` finds nothing, fake: ["XXX"]
  Bahrain: [],
  USSR: ["RUS", "UKR",
         "BLR"],
};
junk after"#;
        let t = territory_map(html);
        assert_eq!(t.len(), 4);
        assert_eq!(t["USA"], vec!["USA"]);
        assert_eq!(t["USSR"], vec!["RUS", "UKR", "BLR"]);
        assert!(t["Bahrain"].is_empty());
        assert!(!t.contains_key("fake"));
    }
}
