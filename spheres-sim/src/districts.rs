//! Districts as data. Identity is transcribed from Natural Earth 10m admin-1
//! (BIBLE section 5); ownership is the only thing the sim adds. No population,
//! GDP or resources per district in this pass — that would be invention. Two
//! per-district quantities are carried, both computed by mapgen from the
//! transcribed geometry, neither invented: `area_sqkm` (ranks districts when a
//! peace settlement moves some of them) and `adj`, the land-adjacency list.
//!
//! Three more transcribed quantities ride the same file since the terrain
//! pass: `t`, the district's terrain class (one of six, classified from
//! Natural Earth's physiographic polygons by tools/terrain/); `f`, the named
//! physical feature it sits on when NE names one; and `riv`, the subset of
//! `adj` reached across a major river. All three are static tables like the
//! rest of the census — never in `WorldState`, never serialized, never
//! hashed — and their sole sim consumer is the front projection's
//! distribution pass in `front.rs` (BIBLE §5: terrain shapes where and how
//! fast the front's budget lands, never how much budget there is).
//!
//! Districts are political AND tactical geography — BIBLE section 5 as
//! amended 2026-08-30: front lines are drawn across the admin-1 district map.
//! `adj` is quantized shared-edge adjacency (land borders only; islands carry
//! an empty list — a theatre's access rules already model reach over water).
//! It exists for the war tick's front projection to consume. There is still
//! no district-level command and no RNG draw anywhere in this module, and
//! ownership still moves at exactly three existing outcome points —
//! annexation (`annex_all`), territorial concession (`cede_share`, and its
//! front-aware form `cede_share_preferring`, which moves the ground the
//! winner actually holds first), and federation dissolution (`dissolve_to`)
//! — riding the same war.rs / politics.rs sites that already move the gdp
//! and population slices.
//!
//! ## The data file
//!
//! `data/districts.json` is emitted by the same mapgen pass that writes
//! `spheres-web/ui/districts.js`, so district ids are byte-identical in both.
//! Keys of `nations` are sim nation codes (`format!("{:?}", NationId)`); a
//! nation's list is every admin-1 feature whose adm0_a3 sits in the browser
//! TERRITORY map's ISO3 list for that nation. Federations therefore carry the
//! union of their republics' districts and each successor repeats its own
//! subset verbatim. Six start nations have TERRITORY-empty polygon lists and
//! ship an empty district list: Bahrain, CapeVerde, Comoros, Maldives,
//! Mauritius, Seychelles.
//!
//! Multi-ISO3 entries (USSR, Yugoslavia, Ethiopia) are grouped by constituent
//! country in TERRITORY order, each group sorted by id — so a list is not
//! globally id-sorted and the validator checks uniqueness rather than
//! adjacency order. Nothing downstream depends on list order: every list is
//! only ever poured into a `BTreeMap`, whose key order is the deterministic
//! order everything iterates in.
//!
//! ## Successors without a parent
//!
//! Namibia and EastTimor have entries (their TERRITORY codes exist for the
//! map) but no federation to inherit from and no birth site in the sim today,
//! so their districts are simply unowned until a birth mechanism exists — a
//! district appears in `WorldState::districts` only while some nation holds
//! it.

use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};
use std::sync::OnceLock;

use crate::data::{render_errors, LoadError};
use crate::world::{NationId, WorldState};

pub const EMBEDDED_DISTRICTS: &str = include_str!("../data/districts.json");
const POPULATION_FALLBACK: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../spheres-web/data/district_population.json"
));
const FILE: &str = "data/districts.json";

#[derive(Deserialize)]
struct PopulationFallback {
    counts: BTreeMap<String, f64>,
}

/// One admin-1 district: a stable id, the Natural Earth name, the area mapgen
/// computed from the geometry (the shipped NE property is zero everywhere, so
/// this is derived from transcription, not invented), and the land neighbours
/// mapgen detected by quantized shared-edge over the same geometry.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DistrictRecord {
    pub id: String,
    pub name: String,
    pub area_sqkm: f64,
    /// Sorted district ids sharing a land border. Empty for islands, and
    /// defaulted so a file written before the operational map parses clean.
    #[serde(default)]
    pub adj: Vec<String>,
    /// Terrain class, transcribed from Natural Earth geography regions
    /// (tools/terrain/classify_districts.py). One of the six class strings;
    /// validated below. Absent in pre-terrain files and synthetic test JSON —
    /// and coverage is all-or-nothing: a file where only SOME districts carry
    /// a class is a broken merge, not an old file.
    #[serde(default, rename = "t", skip_serializing_if = "Option::is_none")]
    pub terrain: Option<String>,
    /// Named physical feature the district sits on, when NE names one
    /// ("Zagros Mountains", "Syrian Desert"). Display flavour, not mechanics.
    #[serde(default, rename = "f", skip_serializing_if = "Option::is_none")]
    pub feature: Option<String>,
    /// Sorted subset of `adj`: neighbours reached across a major river
    /// (tools/terrain/crossing_edges.py, eps 0.05 deg). Symmetric like `adj`.
    #[serde(default, rename = "riv", skip_serializing_if = "Vec::is_empty")]
    pub river_adj: Vec<String>,
}

/// The six terrain classes the map ships — the complete vocabulary of the
/// terrain pass (tools/terrain/classify_districts.py). `Lowland` is the
/// reference class, and what `terrain_of` answers for an id nobody shipped
/// or a pre-terrain data file.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum TerrainClass {
    Lowland,
    Highland,
    Mountain,
    Desert,
    Wetland,
    Tundra,
}

impl TerrainClass {
    /// The transcription strings, exactly as classify_districts.py emits them.
    pub fn from_str(s: &str) -> Option<TerrainClass> {
        Some(match s {
            "lowland" => TerrainClass::Lowland,
            "highland" => TerrainClass::Highland,
            "mountain" => TerrainClass::Mountain,
            "desert" => TerrainClass::Desert,
            "wetland" => TerrainClass::Wetland,
            "tundra" => TerrainClass::Tundra,
            _ => return None,
        })
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DistrictsFile {
    /// Provenance block, when the generator writes one. Not read by the sim.
    #[serde(default)]
    pub meta: Option<serde_json::Value>,
    /// Keyed by NationId code. BTreeMap so iteration is code order.
    pub nations: BTreeMap<String, Vec<DistrictRecord>>,
}

/// Which successor inherits from which federation. Order within each
/// federation's block matches the push order in its dissolution function, and
/// the first heir is the continuation state (Russia; Serbia) — the one that
/// takes any residue the federation won by conquest.
pub const SUCCESSOR_PARENTS: &[(NationId, NationId)] = &[
    (NationId::Russia, NationId::USSR),
    (NationId::Ukraine, NationId::USSR),
    (NationId::Belarus, NationId::USSR),
    (NationId::Kazakhstan, NationId::USSR),
    (NationId::Uzbekistan, NationId::USSR),
    (NationId::Georgia, NationId::USSR),
    (NationId::Armenia, NationId::USSR),
    (NationId::Azerbaijan, NationId::USSR),
    (NationId::Lithuania, NationId::USSR),
    (NationId::Latvia, NationId::USSR),
    (NationId::Estonia, NationId::USSR),
    (NationId::Moldova, NationId::USSR),
    (NationId::Kyrgyzstan, NationId::USSR),
    (NationId::Tajikistan, NationId::USSR),
    (NationId::Turkmenistan, NationId::USSR),
    (NationId::Serbia, NationId::Yugoslavia),
    (NationId::Croatia, NationId::Yugoslavia),
    (NationId::Slovenia, NationId::Yugoslavia),
    (NationId::Bosnia, NationId::Yugoslavia),
    (NationId::Macedonia, NationId::Yugoslavia),
    (NationId::Montenegro, NationId::Yugoslavia),
];

/// Pass one: parse. Pass two: validate. Both collect every error, in the
/// idiom of `data::parse_nations` — a broken file should report all of its
/// problems at once.
pub fn parse_and_validate(json: &str) -> Result<DistrictsFile, Vec<LoadError>> {
    let f: DistrictsFile = serde_json::from_str(json).map_err(|e| {
        vec![LoadError { file: FILE.into(), nation: None, message: e.to_string() }]
    })?;
    let mut errors = vec![];
    // id -> (name, area, adj, terrain, feature, river_adj). A district
    // repeated across entries (USSR + Ukraine) must be the same district —
    // transcription, not two opinions — and adjacency, terrain and river
    // crossings are all part of that identity.
    #[allow(clippy::type_complexity)]
    let mut seen: BTreeMap<
        &str,
        (&str, f64, &[String], Option<&str>, Option<&str>, &[String]),
    > = BTreeMap::new();
    for (code, list) in &f.nations {
        if NationId::from_code(code).is_none() {
            errors.push(LoadError {
                file: FILE.into(),
                nation: Some(code.clone()),
                message: format!("'{}' is not a nation this build has", code),
            });
        }
        let mut in_list: BTreeSet<&str> = BTreeSet::new();
        for d in list {
            if d.id.is_empty() || d.name.is_empty() {
                errors.push(LoadError {
                    file: FILE.into(),
                    nation: Some(code.clone()),
                    message: format!("district '{}' is missing identity fields", d.id),
                });
            }
            if !(d.area_sqkm > 0.0) {
                errors.push(LoadError {
                    file: FILE.into(),
                    nation: Some(code.clone()),
                    message: format!("district '{}' has non-positive area", d.id),
                });
            }
            if let Some(t) = &d.terrain {
                if TerrainClass::from_str(t).is_none() {
                    errors.push(LoadError {
                        file: FILE.into(),
                        nation: Some(code.clone()),
                        message: format!(
                            "district '{}' has unknown terrain class '{}'",
                            d.id, t
                        ),
                    });
                }
            }
            if !in_list.insert(&d.id) {
                errors.push(LoadError {
                    file: FILE.into(),
                    nation: Some(code.clone()),
                    message: format!("district '{}' appears twice in one list", d.id),
                });
            }
            match seen.get(d.id.as_str()) {
                Some(&(n, ar, adj, t, fe, riv))
                    if n != d.name
                        || ar != d.area_sqkm
                        || adj != d.adj
                        || t != d.terrain.as_deref()
                        || fe != d.feature.as_deref()
                        || riv != d.river_adj =>
                {
                    errors.push(LoadError {
                        file: FILE.into(),
                        nation: Some(code.clone()),
                        message: format!(
                            "district '{}' disagrees with an earlier occurrence",
                            d.id
                        ),
                    });
                }
                None => {
                    seen.insert(
                        &d.id,
                        (
                            &d.name,
                            d.area_sqkm,
                            &d.adj,
                            d.terrain.as_deref(),
                            d.feature.as_deref(),
                            &d.river_adj,
                        ),
                    );
                }
                _ => {}
            }
        }
    }
    // Terrain coverage is all-or-nothing: either every district in the census
    // carries a class (a merged file) or none does (a pre-terrain file, which
    // stays valid). A partial file is a broken merge, not an old one.
    let classed = seen.values().filter(|v| v.3.is_some()).count();
    if classed != 0 && classed != seen.len() {
        errors.push(LoadError {
            file: FILE.into(),
            nation: None,
            message: format!(
                "terrain coverage is partial: {} of {} districts classed",
                classed,
                seen.len()
            ),
        });
    }
    // Adjacency is transcribed geometry, so it obeys geometry: sorted-unique
    // (determinism), no self-loops, every neighbour in the census, and
    // symmetric — a border has two sides or it is not a border.
    for (&id, &(_, _, adj, _, _, riv)) in &seen {
        if adj.windows(2).any(|w| w[0] >= w[1]) {
            errors.push(LoadError {
                file: FILE.into(),
                nation: None,
                message: format!("district '{}' adjacency is not sorted and unique", id),
            });
        }
        for n in adj {
            if n == id {
                errors.push(LoadError {
                    file: FILE.into(),
                    nation: None,
                    message: format!("district '{}' is adjacent to itself", id),
                });
                continue;
            }
            match seen.get(n.as_str()) {
                None => errors.push(LoadError {
                    file: FILE.into(),
                    nation: None,
                    message: format!(
                        "district '{}' is adjacent to '{}', which no nation ships",
                        id, n
                    ),
                }),
                Some(&(_, _, back, _, _, _)) if !back.iter().any(|b| b == id) => {
                    errors.push(LoadError {
                        file: FILE.into(),
                        nation: None,
                        message: format!(
                            "adjacency is one-way: '{}' lists '{}' but not back",
                            id, n
                        ),
                    });
                }
                _ => {}
            }
        }
        // River crossings are a property OF a land border, so `riv` obeys the
        // same geometry rules as `adj` and must be a subset of it: a river
        // crossing without a border crossed nothing.
        if riv.windows(2).any(|w| w[0] >= w[1]) {
            errors.push(LoadError {
                file: FILE.into(),
                nation: None,
                message: format!(
                    "district '{}' river adjacency is not sorted and unique",
                    id
                ),
            });
        }
        for n in riv {
            if n == id {
                errors.push(LoadError {
                    file: FILE.into(),
                    nation: None,
                    message: format!("district '{}' river-crosses itself", id),
                });
                continue;
            }
            if !adj.iter().any(|a| a == n) {
                errors.push(LoadError {
                    file: FILE.into(),
                    nation: None,
                    message: format!(
                        "district '{}' river-crosses '{}', which is not a land neighbour",
                        id, n
                    ),
                });
                continue;
            }
            if let Some(&(_, _, _, _, _, back)) = seen.get(n.as_str()) {
                if !back.iter().any(|b| b == id) {
                    errors.push(LoadError {
                        file: FILE.into(),
                        nation: None,
                        message: format!(
                            "river crossing is one-way: '{}' lists '{}' but not back",
                            id, n
                        ),
                    });
                }
            }
        }
    }
    // Every start nation needs an entry, even an empty one — a missing key is
    // a truncated file, not a nation without land.
    for id in crate::nations::start_nations().iter().copied() {
        if !f.nations.contains_key(id.code()) {
            errors.push(LoadError {
                file: FILE.into(),
                nation: Some(id.code().into()),
                message: "start nation has no district entry at all".into(),
            });
        }
    }
    // Start-nation lists must partition: no district owned twice at 1990.
    // (USSR vs Ukraine is fine — Ukraine is a successor, not a starter.)
    let mut start_seen: BTreeMap<&str, &str> = BTreeMap::new();
    for id in crate::nations::start_nations().iter().copied() {
        if let Some(list) = f.nations.get(id.code()) {
            for d in list {
                if let Some(prev) = start_seen.insert(&d.id, id.code()) {
                    errors.push(LoadError {
                        file: FILE.into(),
                        nation: Some(id.code().into()),
                        message: format!(
                            "district '{}' is 1990-owned by both {} and {}",
                            d.id,
                            prev,
                            id.code()
                        ),
                    });
                }
            }
        }
    }
    // Every successor's list is a subset of its federation's list, or a
    // dissolution would mint ground the federation never held.
    for &(heir, parent) in SUCCESSOR_PARENTS {
        let (Some(h), Some(p)) = (f.nations.get(heir.code()), f.nations.get(parent.code()))
        else {
            continue;
        };
        let parent_ids: BTreeSet<&str> = p.iter().map(|d| d.id.as_str()).collect();
        for d in h {
            if !parent_ids.contains(d.id.as_str()) {
                errors.push(LoadError {
                    file: FILE.into(),
                    nation: Some(heir.code().into()),
                    message: format!("'{}' is not in {}'s list", d.id, parent.code()),
                });
            }
        }
    }
    if errors.is_empty() {
        Ok(f)
    } else {
        Err(errors)
    }
}

/// The static census. Parsed once, never in `WorldState`, never in the hash —
/// the same reasoning as `data::sources_for`: immutable start-of-game content
/// must not enter a save or move the timeline fingerprint.
struct Tables {
    /// district id -> (name, area). Full census across all entries.
    info: BTreeMap<String, (String, f64)>,
    /// district id -> sorted land neighbours. Empty for islands.
    adj: BTreeMap<String, Vec<String>>,
    /// nation -> its id list (starters, federations, successors alike).
    lists: BTreeMap<NationId, Vec<String>>,
    /// district id -> its 1990 start owner (start nations only).
    start_owner: BTreeMap<String, NationId>,
    /// district id -> (terrain class, feature name). Absent per id only when
    /// the whole file predates the terrain pass (all-or-nothing validated).
    terrain: BTreeMap<String, (TerrainClass, Option<String>)>,
    /// district id -> the sorted subset of `adj` reached across a major
    /// river. Only districts with at least one crossing have an entry.
    river_adj: BTreeMap<String, Vec<String>>,
}

fn tables() -> &'static Tables {
    static T: OnceLock<Tables> = OnceLock::new();
    T.get_or_init(|| {
        let f = parse_and_validate(EMBEDDED_DISTRICTS)
            .unwrap_or_else(|e| panic!("{}", render_errors(&e)));
        let mut info = BTreeMap::new();
        let mut adj: BTreeMap<String, Vec<String>> = BTreeMap::new();
        let mut lists: BTreeMap<NationId, Vec<String>> = BTreeMap::new();
        let mut terrain: BTreeMap<String, (TerrainClass, Option<String>)> = BTreeMap::new();
        let mut river_adj: BTreeMap<String, Vec<String>> = BTreeMap::new();
        for (code, list) in &f.nations {
            let id = NationId::from_code(code).expect("validated above");
            for d in list {
                info.entry(d.id.clone())
                    .or_insert_with(|| (d.name.clone(), d.area_sqkm));
                adj.entry(d.id.clone()).or_insert_with(|| d.adj.clone());
                if let Some(t) = &d.terrain {
                    let tc = TerrainClass::from_str(t).expect("validated above");
                    terrain
                        .entry(d.id.clone())
                        .or_insert_with(|| (tc, d.feature.clone()));
                }
                if !d.river_adj.is_empty() {
                    river_adj
                        .entry(d.id.clone())
                        .or_insert_with(|| d.river_adj.clone());
                }
            }
            lists.insert(id, list.iter().map(|d| d.id.clone()).collect());
        }
        let mut start_owner = BTreeMap::new();
        for id in crate::nations::start_nations().iter().copied() {
            if let Some(list) = lists.get(&id) {
                for d in list {
                    start_owner.insert(d.clone(), id);
                }
            }
        }
        Tables { info, adj, lists, start_owner, terrain, river_adj }
    })
}

/// Sourced 1990 residents from the resource artifact, with the underlying GHS
/// surface filling only the map districts the resource artifact does not seat.
struct PopulationTables {
    counts: BTreeMap<String, f64>,
    opening: BTreeMap<String, f64>,
}

fn population_tables() -> &'static PopulationTables {
    static P: OnceLock<PopulationTables> = OnceLock::new();
    P.get_or_init(|| {
        let census = tables();
        let population = &crate::resources::tables().file.pop_1990;
        let fallback: PopulationFallback = serde_json::from_str(POPULATION_FALLBACK)
            .unwrap_or_else(|e| panic!("district_population.json: {e}"));
        assert_eq!(fallback.counts.len(), census.info.len());
        for id in fallback.counts.keys() {
            assert!(census.info.contains_key(id), "population has unknown district {id}");
        }
        let mut counts = fallback.counts;
        for (district, people) in population {
            counts.insert(district.clone(), *people as f64);
        }
        let opening = counts
            .iter()
            .map(|(district, people)| (district.clone(), *people / 1e6))
            .collect();
        PopulationTables { counts, opening }
    })
}

pub fn population_1990() -> BTreeMap<String, f64> {
    population_tables().opening.clone()
}

pub fn population_1990_of(district: &str) -> Option<f64> {
    population_tables().opening.get(district).copied()
}

/// Reconstruct the best honest province split for a save written before the
/// province population layer existed.
pub fn reseed_population(w: &mut WorldState) {
    w.district_population_scale = vec![1.0; crate::nations::nation_count()];
    let source = population_tables();
    w.district_population = source
        .counts
        .iter()
        .map(|(d, p)| (d.clone(), p / 1e6))
        .collect();
    let owners: Vec<(NationId, f64)> = w
        .nations
        .iter()
        .filter(|n| n.alive)
        .map(|n| (n.id, n.population))
        .collect();
    for (owner, total_population) in owners {
        let held: Vec<String> = w
            .districts
            .iter()
            .filter(|&(_, &o)| o == owner)
            .map(|(d, _)| d.clone())
            .collect();
        if held.is_empty() {
            continue;
        }
        let weight: f64 = held
            .iter()
            .map(|d| source.counts.get(d).copied().unwrap_or(0.0))
            .sum();
        if weight > 0.0 {
            for d in held {
                let share = source.counts.get(&d).copied().unwrap_or(0.0) / weight;
                w.district_population.insert(d, total_population * share);
            }
        } else {
            let each = total_population / held.len() as f64;
            for d in held {
                w.district_population.insert(d, each);
            }
        }
    }
}

pub fn grow_populations_compounded(
    w: &mut WorldState,
    first: &[(NationId, f64)],
    second: &[(NationId, f64)],
) {
    let count = crate::nations::nation_count();
    if w.district_population_scale.len() != count {
        w.district_population_scale = vec![1.0; count];
    }
    let mut first_by_owner = vec![1.0_f64; count];
    let mut second_by_owner = vec![1.0_f64; count];
    for &(owner, multiplier) in first {
        first_by_owner[owner.index()] = multiplier;
    }
    for &(owner, multiplier) in second {
        second_by_owner[owner.index()] = multiplier;
    }
    for owner in 0..count {
        w.district_population_scale[owner] *= first_by_owner[owner];
        w.district_population_scale[owner] *= second_by_owner[owner];
    }
}

pub fn population_of(w: &WorldState, district: &str) -> Option<f64> {
    let basis = w.district_population.get(district).copied()?;
    let scale = w
        .districts
        .get(district)
        .and_then(|owner| w.district_population_scale.get(owner.index()))
        .copied()
        .unwrap_or(1.0);
    Some(basis * scale)
}

fn rebase_population_for_owner(w: &mut WorldState, district: &str, to: NationId) {
    let Some(actual) = population_of(w, district) else { return };
    let scale = w
        .district_population_scale
        .get(to.index())
        .copied()
        .unwrap_or(1.0);
    let basis = if scale > 0.0 { actual / scale } else { actual };
    w.district_population.insert(district.to_string(), basis);
}

/// A district's land neighbours, sorted by id. Empty for islands, and for an
/// id nobody shipped — never assume a district has neighbours, or that a
/// nation's district graph is connected (Kaliningrad, Alaska, Cabinda).
pub fn adj_of(id: &str) -> &'static [String] {
    tables().adj.get(id).map_or(&[], |v| v)
}

/// A district's area in square kilometres, or 0.0 for an id nobody shipped.
pub fn area_of(id: &str) -> f64 {
    tables().info.get(id).map_or(0.0, |i| i.1)
}

/// A district's terrain class. `Lowland` — the reference going — for an id
/// nobody shipped and for a pre-terrain data file, so the projection treats
/// unclassified ground exactly as the pre-terrain code did.
pub fn terrain_of(id: &str) -> TerrainClass {
    tables().terrain.get(id).map_or(TerrainClass::Lowland, |t| t.0)
}

/// The named physical feature the district sits on, when Natural Earth
/// names one ("Zagros Mountains"). Display flavour, not mechanics.
pub fn feature_of(id: &str) -> Option<&'static str> {
    tables().terrain.get(id).and_then(|t| t.1.as_deref())
}

/// The subset of `adj_of(id)` reached across a major river. Sorted; empty
/// for an id nobody shipped and for a pre-terrain file.
pub fn river_adj_of(id: &str) -> &'static [String] {
    tables().river_adj.get(id).map_or(&[], |v| v)
}

/// Whether the a—b land border crosses a major river. Symmetric, because the
/// validator holds `riv` symmetric the way it holds `adj` symmetric.
pub fn crosses_river(a: &str, b: &str) -> bool {
    river_adj_of(a).binary_search_by(|n| n.as_str().cmp(b)).is_ok()
}

/// A district's display name, if the census has it.
pub fn name_of(id: &str) -> Option<&'static str> {
    tables().info.get(id).map(|i| i.0.as_str())
}

/// Who held this district in January 1990 (start nations only; a successor's
/// ground answers with its federation).
pub fn start_owner_1990(id: &str) -> Option<NationId> {
    tables().start_owner.get(id).copied()
}

/// The districts filed under one nation's code — for a federation, the union
/// of its republics'; for a successor, its own subset.
pub fn list_of(n: NationId) -> &'static [String] {
    tables().lists.get(&n).map_or(&[], |v| v)
}

/// The 1990 ownership map: every district a start nation holds, keys sorted.
pub fn ownership_1990() -> BTreeMap<String, NationId> {
    tables().start_owner.clone()
}

/// Reconstruct ownership for a save written before districts existed.
/// Start defaults first, then every ALIVE successor overrides its own list —
/// so a post-dissolution save reads correctly. Transfers that happened before
/// the upgrade (annexations, concessions) are unknowable and stay at default;
/// that is the same forgiveness the `statecraft`/`governments` serde defaults
/// extend to old saves.
pub fn reseed(w: &mut WorldState) {
    w.districts = ownership_1990();
    for &(heir, _parent) in SUCCESSOR_PARENTS {
        if w.nation_opt(heir).map_or(false, |n| n.alive) {
            for d in list_of(heir) {
                w.districts.insert(d.clone(), heir);
            }
        }
    }
}

/// Annexation: the loser's every district, in BTreeMap (= sorted) order.
/// Returns how many moved.
pub fn annex_all(w: &mut WorldState, winner: NationId, loser: NationId) -> usize {
    w.districts_epoch = w.districts_epoch.wrapping_add(1);
    let taken: Vec<String> = w
        .districts
        .iter()
        .filter(|&(_, &o)| o == loser)
        .map(|(d, _)| d.clone())
        .collect();
    let n = taken.len();
    for d in taken {
        rebase_population_for_owner(w, &d, winner);
        w.districts.insert(d, winner);
    }
    n
}

/// Concession: the loser's most valuable districts. Value is area for now
/// (per-district economy comes later); rank area desc, tie-break id asc via
/// `f64::total_cmp` so the order is total and identical on every machine.
/// Count = ceil(share * held), and a nation that survives the peace keeps at
/// least one district — a loser holding one district cedes nothing.
///
/// Why a count share rather than cumulative area up to `share`: ranking
/// descending means the single largest district alone almost always exceeds
/// 12% of a nation's area, so a cumulative rule would move exactly one
/// district forever. `ceil(share * count)` keeps the transfer proportional to
/// the concession's existing magnitude while staying rank-deterministic.
pub fn cede_share(
    w: &mut WorldState,
    winner: NationId,
    loser: NationId,
    share: f64,
) -> Vec<String> {
    w.districts_epoch = w.districts_epoch.wrapping_add(1);
    if share <= 0.0 {
        return vec![];
    }
    let mut held: Vec<String> = w
        .districts
        .iter()
        .filter(|&(_, &o)| o == loser)
        .map(|(d, _)| d.clone())
        .collect();
    if held.len() < 2 {
        return vec![];
    }
    held.sort_by(|a, b| area_of(b).total_cmp(&area_of(a)).then_with(|| a.cmp(b)));
    let k = ((share * held.len() as f64).ceil() as usize).clamp(1, held.len() - 1);
    held.truncate(k);
    for d in &held {
        rebase_population_for_owner(w, d, winner);
        w.districts.insert(d.clone(), winner);
    }
    held
}

/// `cede_share`, preferring the districts in `preferred` — the ground the
/// winner's front actually holds when the peace is signed. Same count
/// formula, same `(area desc, id asc)` comparator, ranked in two tiers:
/// preferred first, remainder second. With an empty `preferred` this is
/// `cede_share` exactly, list for list — the test suite holds the two
/// together — and `cede_share` itself is retained verbatim above.
pub fn cede_share_preferring(
    w: &mut WorldState,
    winner: NationId,
    loser: NationId,
    share: f64,
    preferred: &BTreeSet<String>,
) -> Vec<String> {
    w.districts_epoch = w.districts_epoch.wrapping_add(1);
    if share <= 0.0 {
        return vec![];
    }
    let mut held: Vec<String> = w
        .districts
        .iter()
        .filter(|&(_, &o)| o == loser)
        .map(|(d, _)| d.clone())
        .collect();
    if held.len() < 2 {
        return vec![];
    }
    held.sort_by(|a, b| {
        preferred
            .contains(b)
            .cmp(&preferred.contains(a))
            .then_with(|| area_of(b).total_cmp(&area_of(a)))
            .then_with(|| a.cmp(b))
    });
    let k = ((share * held.len() as f64).ceil() as usize).clamp(1, held.len() - 1);
    held.truncate(k);
    for d in &held {
        rebase_population_for_owner(w, d, winner);
        w.districts.insert(d.clone(), winner);
    }
    held
}

/// Dissolution: each heir claims its own list where the parent still holds it
/// (a district the parent already conceded does not teleport back), then any
/// residue the parent won by conquest goes to the first heir — the
/// continuation state (Russia; Serbia), which is also the historical answer.
pub fn dissolve_to(w: &mut WorldState, parent: NationId, heirs: &[NationId]) {
    w.districts_epoch = w.districts_epoch.wrapping_add(1);
    for &h in heirs {
        for d in list_of(h) {
            if w.districts.get(d) == Some(&parent) {
                rebase_population_for_owner(w, d, h);
                w.districts.insert(d.clone(), h);
            }
        }
    }
    if let Some(&first) = heirs.first() {
        let residue: Vec<String> = w
            .districts
            .iter()
            .filter(|&(_, &o)| o == parent)
            .map(|(d, _)| d.clone())
            .collect();
        for d in residue {
            rebase_population_for_owner(w, &d, first);
            w.districts.insert(d, first);
        }
    }
}

/// Consent (resources.rs, spec section 4.9): one district from `from` to
/// `to`, with what it carries. The owner must be `from` and `from` keeps at
/// least one district. The people move by the district's share of the
/// ceder's current population (`resources::pop_share_of`), output by
/// three-quarters of that — the settlement's own ratio, war.rs — and oil by
/// the share of the ceder's ledger the table locates in the district (an
/// unlocated producer's ground locates nothing, so it moves none). The
/// ceder's stability falls a point per percent of its people and floors at
/// five; the receiver's separatism rises by the share. No relation write —
/// the contract's +5 covers it. `annex_all`, `cede_share*` and
/// `dissolve_to` are untouched: this is the one consent path and they are
/// the settlement's.
pub fn transfer_district(
    w: &mut WorldState,
    from: NationId,
    to: NationId,
    id: &str,
) -> Result<(), String> {
    if w.districts.get(id) != Some(&from) {
        return Err(format!("{} does not hold {}.", from.name(), id));
    }
    if w.districts.values().filter(|&&o| o == from).count() < 2 {
        return Err("It is all we have.".into());
    }
    w.districts_epoch = w.districts_epoch.wrapping_add(1);
    let pop = crate::resources::pop_share_of(w, from, id);
    let oil = crate::resources::located_oil_fraction(w, from, id);
    let (moved_pop, moved_gdp, moved_oil) = {
        let f = w.nation(from);
        (f.population * pop, f.gdp * 0.75 * pop, f.oil_mbd * oil)
    };
    {
        let f = w.nation_mut(from);
        f.population -= moved_pop;
        f.gdp -= moved_gdp;
        f.oil_mbd -= moved_oil;
        f.stability = (f.stability - 100.0 * pop).max(5.0);
    }
    {
        let t = w.nation_mut(to);
        t.population += moved_pop;
        t.gdp += moved_gdp;
        t.oil_mbd += moved_oil;
        t.separatism = (t.separatism + pop).min(1.0);
    }
    rebase_population_for_owner(w, id, to);
    w.districts.insert(id.to_string(), to);
    Ok(())
}

/// Districts whose owner differs from the 1990 default — the /api/state
/// delta. Sorted by id because the map is a BTreeMap. Usually empty early
/// game, which is the point of delta encoding.
pub fn deltas(w: &WorldState) -> Vec<(String, NationId)> {
    w.districts
        .iter()
        .filter(|&(d, &o)| start_owner_1990(d) != Some(o))
        .map(|(d, &o)| (d.clone(), o))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn embedded_districts_parse_clean() {
        let f = parse_and_validate(EMBEDDED_DISTRICTS).unwrap_or_else(|e| {
            panic!("{}", render_errors(&e));
        });
        // Spot-checks against the shipped census, pinned so a regeneration
        // that changes counts or identity goes red here rather than in a
        // war test three layers up.
        assert_eq!(f.nations["Kuwait"].len(), 6);
        assert_eq!(f.nations["Iraq"].len(), 18);
        assert_eq!(f.nations["USSR"].len(), 269);
        assert_eq!(f.nations["Ukraine"].len(), 25);
        assert_eq!(f.nations["Yugoslavia"].len(), 106);
        let kyiv = f.nations["Ukraine"]
            .iter()
            .find(|d| d.id == "UA-30")
            .expect("Kyiv City is transcribed");
        assert_eq!(kyiv.name, "Kiev City");
        // The federations' lists are exactly the unions of their heirs'.
        for (parent, heirs_expected) in
            [(NationId::USSR, 15usize), (NationId::Yugoslavia, 6usize)]
        {
            let heirs: Vec<NationId> = SUCCESSOR_PARENTS
                .iter()
                .filter(|(_, p)| *p == parent)
                .map(|(h, _)| *h)
                .collect();
            assert_eq!(heirs.len(), heirs_expected);
            let total: usize = heirs.iter().map(|h| list_of(*h).len()).sum();
            assert_eq!(
                total,
                list_of(parent).len(),
                "{:?}'s heirs do not partition it",
                parent
            );
        }
    }

    #[test]
    fn every_start_nation_the_map_can_draw_has_a_list() {
        // The six with TERRITORY-empty polygon lists ship empty on purpose;
        // everyone else on the 1990 board has ground.
        let empty_ok = [
            NationId::Bahrain,
            NationId::CapeVerde,
            NationId::Comoros,
            NationId::Maldives,
            NationId::Mauritius,
            NationId::Seychelles,
        ];
        for id in crate::nations::start_nations().iter().copied() {
            if empty_ok.contains(&id) {
                assert!(list_of(id).is_empty(), "{:?} unexpectedly grew districts", id);
            } else {
                assert!(!list_of(id).is_empty(), "{:?} has no districts", id);
            }
        }
    }

    #[test]
    fn embedded_adjacency_is_transcribed_land_borders() {
        let f = parse_and_validate(EMBEDDED_DISTRICTS).unwrap_or_else(|e| {
            panic!("{}", render_errors(&e));
        });
        // Kuwait's Al Jahra shares the border the Gulf War crosses: Basra.
        let ja = f.nations["Kuwait"]
            .iter()
            .find(|d| d.id == "KW-JA")
            .expect("Al Jahra is transcribed");
        assert!(ja.adj.iter().any(|n| n == "IQ-BA"), "KW-JA touches Basra: {:?}", ja.adj);
        // Hawaii is an island: a present-but-empty list, not a missing one.
        let hi = f.nations["USA"]
            .iter()
            .find(|d| d.id == "US-HI")
            .expect("Hawaii is transcribed");
        assert!(hi.adj.is_empty(), "no sea links in this pass: {:?}", hi.adj);
        // The accessor serves the census, and answers unknowns with empty.
        assert!(adj_of("US-TX").iter().any(|n| n == "US-OK"));
        assert!(adj_of("no-such-district").is_empty());
    }

    #[test]
    fn adjacency_is_a_valid_graph() {
        // The full census, asserted directly on the shipped data rather than
        // through the validator, so a mapgen regression that unmoors the
        // front goes red here by name: closure, symmetry, no self-loops,
        // sorted-unique lists, the KW-JA <-> Basra edge the Gulf settlement
        // rides on, and an island with an empty list rather than a missing
        // one.
        let f = parse_and_validate(EMBEDDED_DISTRICTS).unwrap_or_else(|e| {
            panic!("{}", render_errors(&e));
        });
        let mut census: BTreeMap<&str, &[String]> = BTreeMap::new();
        for list in f.nations.values() {
            for d in list {
                census.insert(&d.id, &d.adj);
            }
        }
        for (id, adj) in &census {
            assert!(
                adj.windows(2).all(|w| w[0] < w[1]),
                "{} adjacency is not sorted-unique: {:?}",
                id,
                adj
            );
            for n in adj.iter() {
                assert_ne!(n.as_str(), *id, "{} is adjacent to itself", id);
                let back = census
                    .get(n.as_str())
                    .unwrap_or_else(|| panic!("{} names {}, which no nation ships", id, n));
                assert!(back.iter().any(|b| b == *id), "{} -> {} is one-way", id, n);
            }
        }
        assert!(
            census["KW-JA"].iter().any(|n| n == "IQ-BA"),
            "the Gulf War border is gone: {:?}",
            census["KW-JA"]
        );
        assert!(census["US-HI"].is_empty(), "Hawaii grew a land border");
    }

    #[test]
    fn bad_adjacency_is_refused() {
        // One-way adjacency: a border has two sides.
        let json = r#"{ "nations": {
            "Iraq": [ {"id": "XX-1", "name": "One", "area_sqkm": 10.0, "adj": ["XX-2"]},
                      {"id": "XX-2", "name": "Two", "area_sqkm": 10.0} ]
        } }"#;
        let err = parse_and_validate(json).expect_err("one-way adjacency must be refused");
        assert!(err.iter().any(|e| e.message.contains("one-way")), "{err:?}");
        // Self-adjacency.
        let json = r#"{ "nations": {
            "Iraq": [ {"id": "XX-1", "name": "One", "area_sqkm": 10.0, "adj": ["XX-1"]} ]
        } }"#;
        let err = parse_and_validate(json).expect_err("self-adjacency must be refused");
        assert!(err.iter().any(|e| e.message.contains("adjacent to itself")), "{err:?}");
        // A neighbour nobody ships.
        let json = r#"{ "nations": {
            "Iraq": [ {"id": "XX-1", "name": "One", "area_sqkm": 10.0, "adj": ["ZZ-9"]} ]
        } }"#;
        let err = parse_and_validate(json).expect_err("ghost neighbour must be refused");
        assert!(err.iter().any(|e| e.message.contains("no nation ships")), "{err:?}");
        // Unsorted lists would leak nondeterminism into every consumer.
        let json = r#"{ "nations": {
            "Iraq": [ {"id": "XX-1", "name": "One", "area_sqkm": 10.0, "adj": ["XX-3", "XX-2"]},
                      {"id": "XX-2", "name": "Two", "area_sqkm": 10.0, "adj": ["XX-1"]},
                      {"id": "XX-3", "name": "Three", "area_sqkm": 10.0, "adj": ["XX-1"]} ]
        } }"#;
        let err = parse_and_validate(json).expect_err("unsorted adjacency must be refused");
        assert!(err.iter().any(|e| e.message.contains("sorted")), "{err:?}");
        // And a record with no adj at all still deserializes — a file written
        // before the operational map defaults to empty lists.
        let json = r#"{ "nations": {
            "Iraq": [ {"id": "XX-1", "name": "One", "area_sqkm": 10.0} ]
        } }"#;
        let f: DistrictsFile =
            serde_json::from_str(json).expect("absent adj defaults to empty");
        assert!(f.nations["Iraq"][0].adj.is_empty());
    }

    #[test]
    fn embedded_terrain_is_transcribed() {
        // The terrain pass, pinned the way the adjacency pass is pinned
        // above: every census district carries a class, the histogram equals
        // the POST-66N-override counts check.py pins (the pre-override
        // staging read lowland 1720 / highland 151 / tundra 15; RU-YAN and
        // RU-KYA moved), and half the world sits on a named feature.
        let f = parse_and_validate(EMBEDDED_DISTRICTS).unwrap_or_else(|e| {
            panic!("{}", render_errors(&e));
        });
        let mut hist: BTreeMap<TerrainClass, usize> = BTreeMap::new();
        let mut ids: BTreeSet<&str> = BTreeSet::new();
        let mut named = 0usize;
        for list in f.nations.values() {
            for d in list {
                if !ids.insert(&d.id) {
                    continue;
                }
                let t = d.terrain.as_deref().unwrap_or_else(|| {
                    panic!("district '{}' shipped without a terrain class", d.id)
                });
                *hist.entry(TerrainClass::from_str(t).unwrap()).or_insert(0) += 1;
                if d.feature.is_some() {
                    named += 1;
                }
            }
        }
        assert_eq!(ids.len(), 2610, "census size moved");
        let pinned = [
            (TerrainClass::Lowland, 1719usize),
            (TerrainClass::Mountain, 583),
            (TerrainClass::Highland, 150),
            (TerrainClass::Desert, 124),
            (TerrainClass::Wetland, 17),
            (TerrainClass::Tundra, 17),
        ];
        for (t, n) in pinned {
            assert_eq!(hist.get(&t).copied().unwrap_or(0), n, "{:?} count moved", t);
        }
        assert!(named >= 1314, "only {} districts carry a feature name", named);
    }

    #[test]
    fn famous_terrain_is_transcribed() {
        // The data half of `the_cold_and_the_open_read_from_the_map` (the
        // behavioural half lives in lib.rs beside the front tests): the
        // accessors answer with the geography everyone knows. RU-YAN is the
        // 66N-override's marquee fix — the Western Siberian Plain polygon
        // used to class Yamal-Nenets lowland.
        assert_eq!(terrain_of("RU-YAN"), TerrainClass::Tundra);
        assert_eq!(terrain_of("IQ-AN"), TerrainClass::Desert);
        assert_eq!(feature_of("IQ-AN"), Some("Syrian Desert"));
        assert_eq!(terrain_of("IR-17"), TerrainClass::Mountain);
        assert_eq!(feature_of("IR-17"), Some("Zagros Mountains"));
        assert_eq!(terrain_of("CH-VS"), TerrainClass::Mountain);
        // The Rio Grande and the Shatt al-Arab, both ways round.
        assert!(crosses_river("MX-TAM", "US-TX"));
        assert!(crosses_river("US-TX", "MX-TAM"));
        assert!(crosses_river("IQ-BA", "IR-10"));
        assert!(crosses_river("IR-10", "IQ-BA"));
        // A land border with no major river on it is not crossed, and the
        // subset rule holds on the accessor too.
        assert!(!crosses_river("IR-17", "IQ-SU"));
        assert!(!crosses_river("US-TX", "US-OK") || crosses_river("US-OK", "US-TX"));
        for n in river_adj_of("MX-TAM") {
            assert!(adj_of("MX-TAM").contains(n), "riv is not a subset of adj");
        }
        // Unknowns answer with the reference class and no crossings.
        assert_eq!(terrain_of("no-such-district"), TerrainClass::Lowland);
        assert_eq!(feature_of("no-such-district"), None);
        assert!(river_adj_of("no-such-district").is_empty());
    }

    #[test]
    fn bad_terrain_is_refused() {
        // An unknown class string.
        let json = r#"{ "nations": {
            "Iraq": [ {"id": "XX-1", "name": "One", "area_sqkm": 10.0, "t": "swamp"} ]
        } }"#;
        let err = parse_and_validate(json).expect_err("unknown class must be refused");
        assert!(err.iter().any(|e| e.message.contains("unknown terrain class")), "{err:?}");
        // riv not a subset of adj: a crossing without a border crossed nothing.
        let json = r#"{ "nations": {
            "Iraq": [ {"id": "XX-1", "name": "One", "area_sqkm": 10.0, "adj": ["XX-2"], "riv": ["XX-3"]},
                      {"id": "XX-2", "name": "Two", "area_sqkm": 10.0, "adj": ["XX-1", "XX-3"]},
                      {"id": "XX-3", "name": "Three", "area_sqkm": 10.0, "adj": ["XX-2"]} ]
        } }"#;
        let err = parse_and_validate(json).expect_err("riv outside adj must be refused");
        assert!(err.iter().any(|e| e.message.contains("not a land neighbour")), "{err:?}");
        // Unsorted riv leaks nondeterminism into `crosses_river`'s search.
        let json = r#"{ "nations": {
            "Iraq": [ {"id": "XX-1", "name": "One", "area_sqkm": 10.0, "adj": ["XX-2", "XX-3"], "riv": ["XX-3", "XX-2"]},
                      {"id": "XX-2", "name": "Two", "area_sqkm": 10.0, "adj": ["XX-1"], "riv": ["XX-1"]},
                      {"id": "XX-3", "name": "Three", "area_sqkm": 10.0, "adj": ["XX-1"], "riv": ["XX-1"]} ]
        } }"#;
        let err = parse_and_validate(json).expect_err("unsorted riv must be refused");
        assert!(
            err.iter().any(|e| e.message.contains("river adjacency is not sorted")),
            "{err:?}"
        );
        // One-way riv: a river has two banks.
        let json = r#"{ "nations": {
            "Iraq": [ {"id": "XX-1", "name": "One", "area_sqkm": 10.0, "adj": ["XX-2"], "riv": ["XX-2"]},
                      {"id": "XX-2", "name": "Two", "area_sqkm": 10.0, "adj": ["XX-1"]} ]
        } }"#;
        let err = parse_and_validate(json).expect_err("one-way riv must be refused");
        assert!(
            err.iter().any(|e| e.message.contains("river crossing is one-way")),
            "{err:?}"
        );
        // Partial terrain coverage: a broken merge, not an old file.
        let json = r#"{ "nations": {
            "Iraq": [ {"id": "XX-1", "name": "One", "area_sqkm": 10.0, "t": "desert"},
                      {"id": "XX-2", "name": "Two", "area_sqkm": 10.0} ]
        } }"#;
        let err = parse_and_validate(json).expect_err("partial coverage must be refused");
        assert!(err.iter().any(|e| e.message.contains("coverage is partial")), "{err:?}");
        // A repeated district must agree on the NEW fields too — same rule
        // as `a_disagreeing_duplicate_is_refused`, stated for terrain.
        let json = r#"{ "nations": {
            "Iraq": [ {"id": "XX-1", "name": "One", "area_sqkm": 10.0, "t": "desert"} ],
            "Iran": [ {"id": "XX-1", "name": "One", "area_sqkm": 10.0, "t": "lowland"} ]
        } }"#;
        let err = parse_and_validate(json).expect_err("two terrain opinions must be refused");
        assert!(err.iter().any(|e| e.message.contains("disagrees")), "{err:?}");
        // And the acceptance direction: a well-formed record WITH t/f/riv
        // deserializes — red before the loader learned the three fields
        // (deny_unknown_fields refused them wholesale) — and none of the
        // NEW validators objects to it (the snippet still trips the census
        // completeness checks, like every fixture in this file, so the
        // assertion is on the terrain checks specifically).
        let json = r#"{ "nations": {
            "Iraq": [ {"id": "XX-1", "name": "One", "area_sqkm": 10.0, "adj": ["XX-2"],
                       "t": "mountain", "f": "Test Range", "riv": ["XX-2"]},
                      {"id": "XX-2", "name": "Two", "area_sqkm": 10.0, "adj": ["XX-1"],
                       "t": "lowland", "riv": ["XX-1"]} ]
        } }"#;
        let f: DistrictsFile =
            serde_json::from_str(json).expect("well-formed terrain fields must parse");
        let one = &f.nations["Iraq"][0];
        assert_eq!(one.terrain.as_deref(), Some("mountain"));
        assert_eq!(one.feature.as_deref(), Some("Test Range"));
        assert_eq!(one.river_adj, vec!["XX-2".to_string()]);
        assert_eq!(f.nations["Iraq"][1].feature, None);
        let err = parse_and_validate(json).expect_err("fixtures never list every nation");
        assert!(
            !err.iter().any(|e| e.message.contains("terrain")
                || e.message.contains("river")
                || e.message.contains("coverage")),
            "the terrain validators objected to a well-formed record: {err:?}"
        );
    }

    #[test]
    fn a_disagreeing_duplicate_is_refused() {
        // The same district stated twice must be the same district.
        let json = r#"{ "nations": {
            "Iraq": [ {"id": "XX-1", "name": "One", "area_sqkm": 10.0} ],
            "Iran": [ {"id": "XX-1", "name": "Other", "area_sqkm": 10.0} ]
        } }"#;
        let err = parse_and_validate(json).expect_err("two opinions must be refused");
        assert!(
            err.iter().any(|e| e.message.contains("disagrees")),
            "{err:?}"
        );
        // And even agreeing, two START nations cannot both own it at 1990.
        let json = r#"{ "nations": {
            "Iraq": [ {"id": "XX-1", "name": "One", "area_sqkm": 10.0} ],
            "Iran": [ {"id": "XX-1", "name": "One", "area_sqkm": 10.0} ]
        } }"#;
        let err = parse_and_validate(json).expect_err("double 1990 ownership must be refused");
        assert!(
            err.iter().any(|e| e.message.contains("1990-owned by both")),
            "{err:?}"
        );
    }
}
