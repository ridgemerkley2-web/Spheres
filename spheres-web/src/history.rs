//! Presentation history. Recent days, older month/year endpoints and nation
//! lifespan boundaries survive for the whole campaign; none enters sim state.
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use spheres_sim::world::{days_in_month, NationId, WorldState};
use std::collections::{BTreeMap, BTreeSet};

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq)]
pub(crate) struct Row {
    pub gdp: f64,
    pub growth: f64,
    pub inflation: f64,
    pub debt: f64,
    pub stability: f64,
    pub mil: f64,
}
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub(crate) struct Snapshot {
    pub t: f64,
    pub year: i32,
    pub month: u32,
    pub day: Option<u32>,
    pub oil: f64,
    pub rows: Vec<(NationId, Row)>,
    #[serde(default)]
    pub milestone: bool,
}
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub(crate) struct Event {
    pub t: u32,
    pub date: String,
    pub text: String,
    pub cat: String,
    pub tags: Vec<NationId>,
}
impl Snapshot {
    pub fn from_world(w: &WorldState) -> Self {
        Self {
            t: crate::month_index(w.year, w.month) as f64
                + if w.rules.daily_simulation {
                    (w.day.max(1) - 1) as f64 / days_in_month(w.year, w.month) as f64
                } else {
                    0.0
                },
            year: w.year,
            month: w.month,
            day: w.rules.daily_simulation.then_some(w.day),
            oil: w.oil_price,
            rows: w
                .nations
                .iter()
                .filter(|n| n.alive)
                .map(|n| {
                    (
                        n.id,
                        Row {
                            gdp: n.gdp,
                            growth: n.growth_last,
                            inflation: n.inflation,
                            debt: n.debt_gdp,
                            stability: n.stability,
                            mil: n.mil_strength,
                        },
                    )
                })
                .collect(),
            milestone: false,
        }
    }
    pub fn date_label(&self) -> String {
        match self.day {
            Some(day) => format!("{} {}", day, crate::month_name(self.month, self.year)),
            None => crate::month_name(self.month, self.year),
        }
    }
}

/// Keep exact observations, never averages masquerading as a point-in-time GDP.
/// Three years of daily detail; twenty years of monthly endpoints; annual
/// endpoints thereafter. The first and both sides of every birth/death remain.
pub(crate) fn record(history: &mut Vec<Snapshot>, w: &WorldState) -> bool {
    let mut next = Snapshot::from_world(w);
    if let Some(previous) = history.last_mut() {
        if !previous
            .rows
            .iter()
            .map(|r| r.0)
            .eq(next.rows.iter().map(|r| r.0))
        {
            previous.milestone = true;
            next.milestone = true;
        }
    } else {
        next.milestone = true;
    }
    history.push(next);
    // Compaction happens at a calendar boundary, so daily clients normally
    // append deltas; at most one additional month of daily detail is retained.
    if w.day == 1 {
        compact(history)
    } else {
        false
    }
}
pub(crate) fn compact(history: &mut Vec<Snapshot>) -> bool {
    let Some(latest) = history.last().map(|s| s.t) else {
        return false;
    };
    let old_len = history.len();
    let mut endpoints = BTreeMap::new();
    let mut keep = BTreeSet::new();
    for (i, s) in history.iter().enumerate() {
        if s.milestone || i == 0 || latest - s.t <= 36.0 {
            keep.insert(i);
            continue;
        }
        let key = if latest - s.t <= 240.0 {
            (s.year, s.month)
        } else {
            (s.year, 0)
        };
        endpoints.insert(key, i);
    }
    keep.extend(endpoints.into_values());
    let mut i = 0;
    history.retain(|_| {
        let retain = keep.contains(&i);
        i += 1;
        retain
    });
    history.len() != old_len
}

#[derive(Default)]
struct Series {
    t0: usize,
    rows: Vec<Row>,
}
/// One traversal of snapshot rows, with indexed country lookup. No per-country
/// linear search through every nation's row in every snapshot.
pub(crate) fn json(history: &[Snapshot], selected: Option<&BTreeSet<NationId>>) -> Value {
    let mut series: BTreeMap<NationId, Series> = BTreeMap::new();
    let mut order = Vec::new();
    for (i, s) in history.iter().enumerate() {
        for &(id, row) in &s.rows {
            if selected.is_some_and(|ids| !ids.contains(&id)) {
                continue;
            }
            let entry = series.entry(id).or_insert_with(|| {
                order.push(id);
                Series {
                    t0: i,
                    rows: vec![],
                }
            });
            // Retain established legacy resurrection behavior: bridge only an
            // internal gap; a dead nation's trailing record stops at death.
            if let Some(last) = entry.rows.last().copied() {
                entry.rows.resize(i - entry.t0, last);
            }
            entry.rows.push(row);
        }
    }
    let nations=series.into_iter().map(|(id,s)| (format!("{id:?}"),json!({
        "name":id.name(),"t0":s.t0,
        "gdp":s.rows.iter().map(|r|crate::round_sig(r.gdp,6)).collect::<Vec<_>>(),
        "growth":s.rows.iter().map(|r|crate::round(r.growth,5)).collect::<Vec<_>>(),
        "inflation":s.rows.iter().map(|r|crate::round(r.inflation,5)).collect::<Vec<_>>(),
        "debt":s.rows.iter().map(|r|crate::round(r.debt,4)).collect::<Vec<_>>(),
        "stability":s.rows.iter().map(|r|crate::round(r.stability,2)).collect::<Vec<_>>(),
        "mil":s.rows.iter().map(|r|crate::round_sig(r.mil,6)).collect::<Vec<_>>(),
    }))).collect::<serde_json::Map<_,_>>();
    json!({"t":history.iter().map(|s|s.t).collect::<Vec<_>>(),
        "labels":history.iter().map(Snapshot::date_label).collect::<Vec<_>>(),
        "oil":history.iter().map(|s|crate::round(s.oil,2)).collect::<Vec<_>>(),
        "metrics":["gdp","growth","inflation","debt","stability","mil"],
        "order":order.iter().map(|id|format!("{id:?}")).collect::<Vec<_>>(),"nations":nations})
}

pub(crate) fn request(g: &crate::Game, url: &str) -> Value {
    let query = url.split_once('?').map(|(_, q)| q).unwrap_or("");
    let param = |key| {
        query
            .split('&')
            .find_map(|s| s.split_once('=').filter(|(k, _)| *k == key).map(|(_, v)| v))
    };
    let selected = param("nations")
        .map(|s| {
            s.split(',')
                .filter_map(NationId::parse)
                .collect::<BTreeSet<_>>()
        })
        .or_else(|| crate::nation_param(url).map(|n| BTreeSet::from([n])));
    let same_epoch = param("epoch").and_then(|s| s.parse::<u64>().ok()) == Some(g.history_epoch);
    let after = param("after")
        .and_then(|s| s.parse::<f64>().ok())
        .filter(|v| v.is_finite());
    let start = if same_epoch {
        after
            .map(|t| g.history.partition_point(|s| s.t <= t))
            .unwrap_or(0)
    } else {
        0
    };
    let mut out = json(&g.history[start..], selected.as_ref());
    out["reset"] = (!same_epoch || after.is_none()).into();
    out["epoch"] = g.history_epoch.into();
    out["cursor"] = g.history.last().map(|s| s.t).into();
    out["session_id"] = g.session_id.clone().into();
    let mut available = BTreeMap::new();
    for s in &g.history {
        for (id, _) in &s.rows {
            available.insert(format!("{id:?}"), id.name());
        }
    }
    out["available"] = json!(available);
    out["retention"] = json!({"daily_years":3,"monthly_years":20,"older":"annual endpoints","milestones":"nation births and endings"});
    out
}

/// Immutable archive offsets let the browser read older dispatches on demand.
/// New events append, so a response lost or repeated cannot skip an old page.
pub(crate) fn events(g: &crate::Game, url: &str) -> Value {
    let query = url.split_once('?').map(|(_, q)| q).unwrap_or("");
    let param = |key| {
        query.split('&').find_map(|s| {
            s.split_once('=')
                .filter(|(k, _)| *k == key)
                .and_then(|(_, v)| v.parse::<usize>().ok())
        })
    };
    let end = param("before").unwrap_or(g.log.len()).min(g.log.len());
    let limit = param("limit").unwrap_or(500).clamp(1, 1000);
    let start = end.saturating_sub(limit);
    json!({"session_id":g.session_id,"total":g.log.len(),"before":start,
        "events":g.log[start..end].iter().rev().collect::<Vec<_>>()})
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn multidecade_archive_preserves_dates_lifespans_and_daily_detail() {
        let mut w = crate::world_1990(crate::GameRules {
            daily_simulation: true,
            ..Default::default()
        });
        let mut archive = vec![];
        for year in 1990..=2040 {
            for month in 1..=12 {
                for day in 1..=days_in_month(year, month) {
                    w.year = year;
                    w.month = month;
                    w.day = day;
                    // A two-day polity must not disappear between annual endpoints.
                    w.nation_mut(NationId::France).alive =
                        year != 1995 || (month == 5 && (day == 8 || day == 9));
                    record(&mut archive, &w);
                }
            }
        }
        assert_eq!(archive.first().unwrap().date_label(), "1 Jan 1990");
        assert_eq!(archive.last().unwrap().date_label(), "31 Dec 2040");
        assert!(
            archive.len() < 1600,
            "{} points instead of ~18,000 daily rows",
            archive.len()
        );
        assert!(archive
            .iter()
            .any(|s| s.year == 1995 && s.month == 5 && s.day == Some(8)));
        assert!(archive
            .iter()
            .any(|s| s.year == 1995 && s.month == 5 && s.day == Some(9)));
        assert!(archive
            .iter()
            .any(|s| s.year == 1995 && s.month == 5 && s.day == Some(10)));
        assert_eq!(archive.iter().filter(|s| s.year == 2040).count(), 366);
        assert!(archive.windows(2).all(|p| p[0].t < p[1].t));
    }
    #[test]
    fn selected_series_and_delta_equal_full_history() {
        let mut g = crate::Game::new(1990, Some(NationId::USA));
        crate::play_rules(&mut g);
        g.history.clear();
        g.snapshot();
        let first = request(&g, "/api/history?nations=USA");
        g.world.day = 2;
        g.snapshot();
        let delta = request(
            &g,
            &format!("/api/history?nations=USA&epoch={}&after=0", g.history_epoch),
        );
        assert_eq!(delta["reset"], false);
        assert_eq!(delta["t"].as_array().unwrap().len(), 1);
        let full = request(&g, "/api/history");
        assert_eq!(
            delta["nations"]["USA"]["gdp"][0],
            full["nations"]["USA"]["gdp"][1]
        );
        assert_eq!(first["nations"].as_object().unwrap().len(), 1);
        g.history_epoch += 1;
        assert_eq!(request(&g, "/api/history?epoch=0&after=0")["reset"], true);
    }
    #[test]
    fn event_pages_preserve_the_entire_archive_across_new_dispatches() {
        let mut g = crate::Game::new(1990, Some(NationId::USA));
        for i in 0..4500 {
            g.record(format!("Dispatch {i}"));
        }
        assert_eq!(g.log.len(), 4500);
        let page = events(&g, "/api/events?before=500&limit=500");
        assert_eq!(page["before"], 0);
        assert_eq!(page["events"][0]["text"], "Dispatch 499");
        assert_eq!(page["events"][499]["text"], "Dispatch 0");
        g.record("Another dispatch".into());
        assert_eq!(
            events(&g, "/api/events?before=500&limit=500")["events"],
            page["events"]
        );
    }
}
