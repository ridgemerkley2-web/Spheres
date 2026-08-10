//! One-shot tool: turn a Natural Earth countries GeoJSON into the baked SVG
//! paths the UI ships with. Run it only when the map data needs regenerating:
//!
//!   cargo run --release -p spheres-web --bin mapgen -- world.geojson
//!
//! Writes spheres-web/ui/world.js. The output is committed, so the game itself
//! never needs this tool or the source data.

use std::collections::BTreeMap;
use std::fmt::Write as _;

const W: f64 = 1000.0;
// Robinson: the compromise projection used on most printed world maps. Neither
// equal-area nor conformal, but it is the one that "looks like a map" — the
// high latitudes are not smeared the way equirectangular smears them.
const LAT_TOP: f64 = 83.0;
const LAT_BOT: f64 = -58.0;

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

/// Drop specks: at this scale a 2px island is noise, and there are hundreds.
const MIN_AREA_PX: f64 = 3.0;

fn ring_to_path(ring: &[serde_json::Value], out: &mut String) -> Option<(f64, f64, f64)> {
    let pts: Vec<(f64, f64)> = ring
        .iter()
        .filter_map(|p| {
            let a = p.as_array()?;
            Some(project(a.first()?.as_f64()?, a.get(1)?.as_f64()?))
        })
        .collect();
    if pts.len() < 3 {
        return None;
    }
    let (mut x0, mut y0, mut x1, mut y1) = (f64::MAX, f64::MAX, f64::MIN, f64::MIN);
    for (x, y) in &pts {
        x0 = x0.min(*x); y0 = y0.min(*y); x1 = x1.max(*x); y1 = y1.max(*y);
    }
    if (x1 - x0) * (y1 - y0) < MIN_AREA_PX {
        return None;
    }
    // Shoelace, for area-weighted centroids and to rank a country's landmasses.
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
        return None;
    }
    cx /= 6.0 * area;
    cy /= 6.0 * area;

    let mut last = (f64::MAX, f64::MAX);
    for (i, (x, y)) in pts.iter().enumerate() {
        let (rx, ry) = ((x * 10.0).round() / 10.0, (y * 10.0).round() / 10.0);
        // Collapse points that round to the same tenth of a pixel.
        if i > 0 && (rx - last.0).abs() < 0.05 && (ry - last.1).abs() < 0.05 {
            continue;
        }
        let _ = write!(out, "{}{} {}", if i == 0 { "M" } else { "L" }, rx, ry);
        last = (rx, ry);
    }
    out.push('Z');
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

fn main() {
    let path = std::env::args().nth(1).unwrap_or_else(|| {
        eprintln!("usage: mapgen <world.geojson>");
        std::process::exit(1);
    });
    let raw = std::fs::read_to_string(&path).expect("read geojson");
    let gj: serde_json::Value = serde_json::from_str(&raw).expect("parse geojson");

    let mut paths: BTreeMap<String, String> = BTreeMap::new();
    let mut centroids: BTreeMap<String, (f64, f64, f64)> = BTreeMap::new();

    for f in gj["features"].as_array().expect("features") {
        let props = &f["properties"];
        let code = props["ADM0_A3"]
            .as_str()
            .or_else(|| props["ISO_A3"].as_str())
            .unwrap_or("")
            .to_string();
        // Antarctica is a wall of ice across the bottom of the frame and no
        // actor in this game; the projection is clipped short of it anyway.
        if code.is_empty() || code == "-99" || code == "ATA" {
            continue;
        }
        let geom = &f["geometry"];
        let polys: Vec<&serde_json::Value> = match geom["type"].as_str() {
            Some("Polygon") => vec![&geom["coordinates"]],
            Some("MultiPolygon") => geom["coordinates"].as_array().map(|a| a.iter().collect()).unwrap_or_default(),
            _ => continue,
        };
        let mut d = String::new();
        let mut best: Option<(f64, f64, f64)> = None;
        for poly in polys {
            if let Some(rings) = poly.as_array() {
                for ring in rings {
                    if let Some(r) = ring.as_array() {
                        if let Some((cx, cy, a)) = ring_to_path(r, &mut d) {
                            // Label the biggest landmass, not the average of
                            // scattered islands — otherwise the USA is labelled
                            // somewhere in the Pacific.
                            if best.map_or(true, |(_, _, ba)| a > ba) {
                                best = Some((cx, cy, a));
                            }
                        }
                    }
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
                .and_modify(|e| if b.2 > e.2 { *e = b })
                .or_insert(b);
        }
    }

    let mut out = String::new();
    out.push_str("// Generated by `cargo run -p spheres-web --bin mapgen`.\n");
    out.push_str("// Source: Natural Earth 110m admin-0 countries (public domain).\n");
    out.push_str("// Equirectangular, clipped to 84N..56S. Do not hand-edit.\n");
    let _ = write!(out, "window.WORLD={{w:{},h:{:.1},countries:{{", W, height());
    for (i, (code, d)) in paths.iter().enumerate() {
        let _ = write!(out, "{}\"{}\":\"{}\"", if i > 0 { "," } else { "" }, code, d);
    }
    out.push_str("},centroids:{");
    for (i, (code, (cx, cy, _))) in centroids.iter().enumerate() {
        let _ = write!(
            out, "{}\"{}\":[{:.1},{:.1}]", if i > 0 { "," } else { "" }, code, cx, cy
        );
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
    let _ = write!(out, "],frame:\"{}\"}};\n", frame);

    let dest = "spheres-web/ui/world.js";
    std::fs::write(dest, &out).expect("write world.js");
    println!("{} countries -> {} ({:.0} KB)", paths.len(), dest, out.len() as f64 / 1024.0);
}
