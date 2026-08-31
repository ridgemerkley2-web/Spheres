//! Nations as data.
//!
//! SPEC section 2 puts "content as data — nations and events in JSON files,
//! two-pass validation, `deny_unknown_fields`, moddable by design" in the
//! architecture constitution, and BIBLE section 2 says the wedge over
//! Millennium Dawn is that authored content is finite. A roster of ~190 states
//! cannot live as positional calls to an 18-argument constructor: `init.rs`
//! held every nation as `n(USA, Market, 0.10, 5980.0, 250.0, ...)`, where the
//! twelfth argument is a debt ratio only because it is the twelfth, and where
//! the 1990 provenance was a Rust comment that no tool could read.
//!
//! Here each nation is one JSON file with named fields, and the sourcing note
//! that justifies its numbers travels with it in `sources`, so iron rule 4
//! ("transcribed, not invented") is enforceable rather than a convention.
//!
//! ## Why the default set is embedded
//!
//! The same constitution says `spheres-sim` is an "engine-agnostic headless
//! core with no I/O". Those two lines look opposed and are not. The canonical
//! 1990 set is embedded with `include_str!`, so the library does no file
//! access, the binary is self-contained, and every test is hermetic — a
//! determinism test that could fail because a data file moved would be worse
//! than no determinism test. Modding is served by the *shape of the API*
//! rather than by the sim opening files: [`load_world`] takes sources as
//! strings, so a shell that is allowed to do I/O (the CLI, the web server, a
//! future mod loader) can read a directory and pass the contents in without
//! the sim ever growing an `std::fs`.
//!
//! There is a determinism reason to prefer an explicit manifest over a
//! directory scan, too. Roster order is serialization order is the golden
//! hash; a `read_dir` would hand back whatever order the filesystem felt like,
//! which differs between Windows and Linux. [`EMBEDDED_NATIONS`] is an ordered
//! list, and a disk loader must impose an order of its own.

use serde::{Deserialize, Serialize};

use crate::world::*;

pub mod embedded;

pub use embedded::{EMBEDDED_NATIONS, EMBEDDED_RELATIONS};

/// One content file, named so an error can point at it.
#[derive(Clone, Copy, Debug)]
pub struct Source<'a> {
    /// Path or logical name, used only in error messages.
    pub file: &'a str,
    pub json: &'a str,
}

/// A problem with the content, addressed to whoever wrote it.
///
/// Never a bare serde message: a modder needs to know which file and which
/// nation, which is exactly what the positional constructor could not tell
/// anyone.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LoadError {
    pub file: String,
    pub nation: Option<String>,
    pub message: String,
}

impl LoadError {
    fn file_level(file: &str, message: impl Into<String>) -> Self {
        LoadError { file: file.to_string(), nation: None, message: message.into() }
    }
    fn nation_level(file: &str, nation: &str, message: impl Into<String>) -> Self {
        LoadError {
            file: file.to_string(),
            nation: Some(nation.to_string()),
            message: message.into(),
        }
    }
}

impl std::fmt::Display for LoadError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.nation {
            Some(n) => write!(f, "{} [{}]: {}", self.file, n, self.message),
            None => write!(f, "{}: {}", self.file, self.message),
        }
    }
}

/// Render a whole batch of errors as one message. Two-pass validation exists to
/// report every problem in the content at once rather than the first one.
pub fn render_errors(errors: &[LoadError]) -> String {
    let mut s = format!("{} problem(s) in the nation data:\n", errors.len());
    for e in errors {
        s.push_str("  - ");
        s.push_str(&e.to_string());
        s.push('\n');
    }
    s
}

// ---------------------------------------------------------------------------
// The schema
// ---------------------------------------------------------------------------

/// Everything `init.rs` used to pass positionally, with a name on each figure.
///
/// `deny_unknown_fields` everywhere: a misspelled key is a silent zero
/// otherwise, and a silent zero in `debt_gdp` is a nation that starts the game
/// solvent by typo.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct NationRecord {
    pub id: NationId,
    /// Display name. Cross-checked in pass two against `NationId::name()`, which
    /// is still the runtime authority while ids are a compile-time enum. When
    /// ids become runtime values this field becomes the authority instead.
    pub name: String,
    pub system: EconomySystem,
    /// 0..1 — how authoritarian; gates elections and colours stability.
    pub authoritarianism: f64,
    pub economy: EconomyRecord,
    pub politics: PoliticsRecord,
    pub military: MilitaryRecord,
    /// Where the 1990 figures above come from, and what they mean. Iron rule 4
    /// requires this provenance; moving the nations out of Rust must not lose
    /// it, so it is a field rather than a comment.
    #[serde(default)]
    pub sources: Vec<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct EconomyRecord {
    /// Real GDP, billions of 1990 USD.
    pub gdp_bn: f64,
    /// Population, millions.
    pub population_m: f64,
    /// Total factor productivity growth trend, annual.
    pub tfp_trend: f64,
    /// Annual inflation rate (0.04 = 4%).
    pub inflation: f64,
    /// Central bank policy rate, annual.
    pub interest_rate: f64,
    /// Tax take as share of GDP.
    pub tax_rate: f64,
    /// Military spending as share of GDP.
    pub mil_spend_gdp: f64,
    /// State/public investment as share of GDP.
    pub state_invest_gdp: f64,
    /// Private investment as share of GDP.
    pub priv_invest_gdp: f64,
    /// Public debt as share of GDP.
    pub debt_gdp: f64,
    /// Oil production, million barrels/day (0 for non-producers).
    pub oil_mbd: f64,
    /// Asset bubble intensity 0..1. Only Japan starts hot, and it used to be a
    /// stray mutation in `world_1990` after the table was built — which is
    /// exactly the kind of fact that belongs with the nation it describes.
    #[serde(default)]
    pub bubble: f64,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct PoliticsRecord {
    /// 0..100 — regime stability/legitimacy.
    pub stability: f64,
    /// Nationalities/separatism strain 0..1.
    pub separatism: f64,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct MilitaryRecord {
    /// Abstract strength index.
    pub strength: f64,
    pub nuclear: bool,
}

/// The seed relations table.
///
/// Kept in blocks so the prose that explains a group of dyads — why Jakarta
/// opens negative toward Beijing, why Ankara's line to Baghdad is a pipeline
/// and not a friendship — survives the move out of Rust comments. Phase 3.2
/// replaces the literal table with dyads derived from adjacency and claims;
/// until then this is transcribed content like any other.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Default)]
#[serde(deny_unknown_fields)]
pub struct RelationsFile {
    #[serde(default)]
    pub blocks: Vec<RelationBlock>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct RelationBlock {
    #[serde(default)]
    pub note: Vec<String>,
    pub pairs: Vec<RelationRecord>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct RelationRecord {
    pub a: NationId,
    pub b: NationId,
    /// -100..100, symmetric.
    pub value: f64,
}

// ---------------------------------------------------------------------------
// Pass one — parse, and validate each record against itself
// ---------------------------------------------------------------------------

/// Parse every nation file. Errors are collected, not short-circuited: a modder
/// with three broken files should see three messages.
pub fn parse_nations(sources: &[Source]) -> Result<Vec<NationRecord>, Vec<LoadError>> {
    let mut out: Vec<NationRecord> = Vec::with_capacity(sources.len());
    let mut errors: Vec<LoadError> = vec![];
    for src in sources {
        match serde_json::from_str::<NationRecord>(src.json) {
            Ok(rec) => {
                errors.extend(check_record(src.file, &rec));
                out.push(rec);
            }
            Err(e) => errors.push(LoadError::file_level(src.file, e.to_string())),
        }
    }
    if errors.is_empty() {
        Ok(out)
    } else {
        Err(errors)
    }
}

pub fn parse_relations(src: &Source) -> Result<RelationsFile, Vec<LoadError>> {
    serde_json::from_str::<RelationsFile>(src.json)
        .map_err(|e| vec![LoadError::file_level(src.file, e.to_string())])
}

/// Bounds that are facts about the model rather than opinions about history:
/// a share is a share, stability is a percentage, and a negative population is
/// a typo in every possible world.
fn check_record(file: &str, r: &NationRecord) -> Vec<LoadError> {
    let mut e = vec![];
    let who = format!("{:?}", r.id);
    let mut unit = |v: f64, field: &str| {
        if !(0.0..=1.0).contains(&v) || !v.is_finite() {
            e.push(LoadError::nation_level(
                file,
                &who,
                format!("{field} is {v}, expected a share in 0..=1"),
            ));
        }
    };
    unit(r.authoritarianism, "authoritarianism");
    unit(r.economy.tax_rate, "economy.tax_rate");
    unit(r.economy.mil_spend_gdp, "economy.mil_spend_gdp");
    unit(r.economy.state_invest_gdp, "economy.state_invest_gdp");
    unit(r.economy.priv_invest_gdp, "economy.priv_invest_gdp");
    unit(r.economy.bubble, "economy.bubble");
    unit(r.politics.separatism, "politics.separatism");

    if !(r.economy.gdp_bn.is_finite() && r.economy.gdp_bn > 0.0) {
        e.push(LoadError::nation_level(
            file,
            &who,
            format!("economy.gdp_bn is {}, expected a positive number", r.economy.gdp_bn),
        ));
    }
    if !(r.economy.population_m.is_finite() && r.economy.population_m > 0.0) {
        e.push(LoadError::nation_level(
            file,
            &who,
            format!("economy.population_m is {}, expected a positive number", r.economy.population_m),
        ));
    }
    if !(r.economy.debt_gdp.is_finite() && r.economy.debt_gdp >= 0.0) {
        e.push(LoadError::nation_level(
            file,
            &who,
            format!("economy.debt_gdp is {}, expected a non-negative ratio", r.economy.debt_gdp),
        ));
    }
    if !(r.economy.oil_mbd.is_finite() && r.economy.oil_mbd >= 0.0) {
        e.push(LoadError::nation_level(
            file,
            &who,
            format!("economy.oil_mbd is {}, expected a non-negative number", r.economy.oil_mbd),
        ));
    }
    // Inflation and the policy rate had no bounds at all, which is how three
    // nations came to carry figures that contradict the sources block sitting
    // beside them. The bounds below are not taste. Each one is a place where
    // the model stops being able to represent the number:
    //
    //  * `economy.rs` clamps inflation into -0.05..=3.0 on the first tick, so a
    //    start value outside that band is a claim the sim discards in month one.
    //    Brazil's real 1990 CPI print of 2948% is 29.48 here, ten times the
    //    ceiling.
    //  * the demand channel is linear and unbounded in `neutral - real_rate` at
    //    a coefficient of 0.55 to annual growth. A real rate of tens therefore
    //    drives |growth| into the tens.
    //
    //    This bound used to be justified by what happened next: `1.0 +
    //    growth/12.0` went through zero, GDP changed sign, and entering
    //    Brazil's true 2948%/9394% pair sent it to +inf and took twelve tests
    //    with it. That route is closed — `economy.rs` floors the annual rate at
    //    -0.95, so the monthly factor cannot reach zero and no input inverts
    //    output any more. The bound stands on fidelity instead, which is the
    //    weaker claim but the one that survives: re-measured with the floor in
    //    place, the true pair does not break the world (no nation loses its
    //    sign or its finiteness over ten years) and still cannot be carried —
    //    Brazil falls to 9.3% of its 1990 output within three years and sits
    //    pinned at the -5% deflation clamp for four of them, against a real
    //    1990 contraction of 4.3%. Unrepresentable rather than fatal.
    //
    // The gap bound of 0.5 is 27.5 points of annual growth from the rate
    // channel alone, which is already past anything a calibration test here
    // would accept; the widest figure the 1990 roster actually ships is
    // Yugoslavia at 0.325.
    if !r.economy.inflation.is_finite() || !(-0.05..=3.0).contains(&r.economy.inflation) {
        e.push(LoadError::nation_level(
            file,
            &who,
            format!(
                "economy.inflation is {}, outside -0.05..=3.0 — the fraction-of-1 band \
                 the model can hold (economy.rs clamps to it on the first tick, so a \
                 figure above 3.0 is a start state the sim throws away in month one). \
                 If the real 1990 print was larger than 300%, say so in sources and \
                 enter what the model can carry",
                r.economy.inflation
            ),
        ));
    }
    if !(r.economy.interest_rate.is_finite() && r.economy.interest_rate >= 0.0) {
        e.push(LoadError::nation_level(
            file,
            &who,
            format!(
                "economy.interest_rate is {}, expected a non-negative annual rate",
                r.economy.interest_rate
            ),
        ));
    }
    let real_rate_gap = 0.025 - (r.economy.interest_rate - r.economy.inflation);
    if !real_rate_gap.is_finite() || real_rate_gap.abs() > 0.5 {
        e.push(LoadError::nation_level(
            file,
            &who,
            format!(
                "economy.interest_rate {} against economy.inflation {} puts the real \
                 rate {:.3} from neutral. The demand channel in economy.rs is linear \
                 and unbounded in that distance at 0.55, so this opens the game at \
                 {:+.0}% annual growth from monetary policy alone. The growth floor \
                 stops that inverting GDP, but it does not make the figure playable: \
                 Brazil's true pair collapses it to 9% of its 1990 output inside three \
                 years. Say the real print in sources and enter what the model can carry",
                r.economy.interest_rate,
                r.economy.inflation,
                real_rate_gap,
                real_rate_gap * 0.55 * 100.0
            ),
        ));
    }

    if !(0.0..=100.0).contains(&r.politics.stability) {
        e.push(LoadError::nation_level(
            file,
            &who,
            format!("politics.stability is {}, expected 0..=100", r.politics.stability),
        ));
    }
    if !(r.military.strength.is_finite() && r.military.strength >= 0.0) {
        e.push(LoadError::nation_level(
            file,
            &who,
            format!("military.strength is {}, expected a non-negative index", r.military.strength),
        ));
    }
    if r.sources.is_empty() {
        e.push(LoadError::nation_level(
            file,
            &who,
            "sources is empty — iron rule 4 says starting data is transcribed, \
             so say where these 1990 figures came from"
                .to_string(),
        ));
    }
    e
}

// ---------------------------------------------------------------------------
// Pass two — cross-references between records
// ---------------------------------------------------------------------------

/// The second pass. Nothing here can be checked while looking at one file:
/// duplicate ids, a name that disagrees with the roster, a relation naming a
/// nation nobody shipped, a start nation with no file at all.
pub fn validate(nations: &[NationRecord], relations: &RelationsFile, relations_file: &str) -> Result<(), Vec<LoadError>> {
    let mut errors = vec![];

    // Duplicate ids. Two files claiming to be Iraq is the failure mode a
    // directory of mod files invites, and the old Vec-of-literals could not
    // express it at all.
    for (i, r) in nations.iter().enumerate() {
        if let Some(j) = nations.iter().take(i).position(|o| o.id == r.id) {
            errors.push(LoadError::nation_level(
                &file_of(nations, i),
                &format!("{:?}", r.id),
                format!(
                    "duplicate id — already defined by entry {} ({})",
                    j + 1,
                    file_of(nations, j)
                ),
            ));
        }
    }

    // The name must agree with the roster the rest of the engine uses.
    for (i, r) in nations.iter().enumerate() {
        if r.name != r.id.name() {
            errors.push(LoadError::nation_level(
                &file_of(nations, i),
                &format!("{:?}", r.id),
                format!(
                    "name is {:?} but the roster calls this nation {:?}",
                    r.name,
                    r.id.name()
                ),
            ));
        }
    }

    // Every start nation the engine knows about needs a file, and every file
    // must name a start nation. This check is a consequence of ids still being
    // a compile-time enum; when 1.1's first half lands and ids become runtime
    // values, the data becomes the sole authority and this collapses into a
    // uniqueness check.
    for want in start_nations().iter().copied() {
        if !nations.iter().any(|r| r.id == want) {
            errors.push(LoadError::file_level(
                "<roster>",
                format!(
                    "{} ({:?}) is a start nation but no data file defines it",
                    want.name(),
                    want
                ),
            ));
        }
    }
    for (i, r) in nations.iter().enumerate() {
        if !start_nations().contains(&r.id) {
            errors.push(LoadError::nation_level(
                &file_of(nations, i),
                &format!("{:?}", r.id),
                "not a start nation — successor states are created when a \
                 federation comes apart, not loaded at 1990"
                    .to_string(),
            ));
        }
    }

    // Relations may only name nations that exist in this set, and a dyad may
    // only be stated once — the matrix is symmetric, so stating both (A,B) and
    // (B,A) is one of them silently winning.
    let mut seen: Vec<(NationId, NationId)> = vec![];
    for block in &relations.blocks {
        for p in &block.pairs {
            for side in [p.a, p.b] {
                if !nations.iter().any(|r| r.id == side) {
                    errors.push(LoadError::file_level(
                        relations_file,
                        format!(
                            "relation {:?}-{:?} names {:?}, which no nation file defines",
                            p.a, p.b, side
                        ),
                    ));
                }
            }
            if p.a == p.b {
                errors.push(LoadError::file_level(
                    relations_file,
                    format!("relation {:?}-{:?} is a nation with itself", p.a, p.b),
                ));
            }
            if !(-100.0..=100.0).contains(&p.value) {
                errors.push(LoadError::file_level(
                    relations_file,
                    format!(
                        "relation {:?}-{:?} is {}, outside -100..=100",
                        p.a, p.b, p.value
                    ),
                ));
            }
            let key = if p.a <= p.b { (p.a, p.b) } else { (p.b, p.a) };
            if seen.contains(&key) {
                errors.push(LoadError::file_level(
                    relations_file,
                    format!(
                        "relation {:?}-{:?} is stated twice; relations are symmetric",
                        p.a, p.b
                    ),
                ));
            }
            seen.push(key);
        }
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

/// The nation files are parsed in the order given, so index i is source i. Kept
/// as a helper rather than threading the source list through pass two, which
/// would make every signature carry a lifetime for the sake of an error string.
fn file_of(nations: &[NationRecord], i: usize) -> String {
    nations
        .get(i)
        .map(|r| format!("{}.json", id_slug(r.id)))
        .unwrap_or_else(|| "<unknown>".to_string())
}

/// The filename a nation's data lives under: the id, lowercased.
pub fn id_slug(id: NationId) -> String {
    format!("{:?}", id).to_lowercase()
}

// ---------------------------------------------------------------------------
// Building the world
// ---------------------------------------------------------------------------

impl NationRecord {
    /// The record as the engine's `Nation`.
    ///
    /// Two fields are absent from the data on purpose. `political_capital` is
    /// derived — nobody recorded a government's standing in January 1990, so it
    /// is read off the conditions that *were* recorded. `tech` is seeded from
    /// the TFP trend by the technology module. Transcribing either would be
    /// inventing.
    pub fn to_nation(&self) -> Nation {
        Nation {
            id: self.id,
            alive: true,
            system: self.system,
            authoritarianism: self.authoritarianism,
            gdp: self.economy.gdp_bn,
            population: self.economy.population_m,
            tfp_trend: self.economy.tfp_trend,
            inflation: self.economy.inflation,
            interest_rate: self.economy.interest_rate,
            tax_rate: self.economy.tax_rate,
            mil_spend_gdp: self.economy.mil_spend_gdp,
            state_invest_gdp: self.economy.state_invest_gdp,
            priv_invest_gdp: self.economy.priv_invest_gdp,
            debt_gdp: self.economy.debt_gdp,
            oil_mbd: self.economy.oil_mbd,
            bubble: self.economy.bubble,
            growth_last: 0.0,
            stability: self.politics.stability,
            separatism: self.politics.separatism,
            mil_strength: self.military.strength,
            // Full magazines in January 1990: a peacetime stock nobody
            // recorded per-nation, so it is derived rather than transcribed,
            // exactly like political_capital above.
            munitions: 1.0,
            war_exhaustion: 0.0,
            nuclear: self.military.nuclear,
            arsenal: crate::arsenal::inheritance(self),
            political_capital: crate::politics::seated_political_capital(
                self.politics.stability,
                self.economy.inflation,
                self.authoritarianism,
            ),
            tech: crate::tech::TechState::new(self.economy.tfp_trend),
        }
    }
}

/// Load a world from content. Both passes run before anything is built, so a
/// broken data set never produces a half-populated `WorldState`.
///
/// The order of `nation_sources` is the roster order, which is serialization
/// order, which is the golden hash. A caller reading from disk must impose a
/// deterministic order of its own — see the module note.
pub fn load_world(
    nation_sources: &[Source],
    relations_source: &Source,
    rules: GameRules,
) -> Result<WorldState, Vec<LoadError>> {
    let nations = parse_nations(nation_sources)?;
    let relations = parse_relations(relations_source)?;
    validate(&nations, &relations, relations_source.file)?;

    let mut w = WorldState {
        rng: Rng::new(rules.seed),
        rules,
        year: 1990,
        month: 1,
        nations: nations.iter().map(|r| r.to_nation()).collect(),
        relations: Relations::default(),
        sanctions: vec![],
        wars: vec![],
        statecraft: Statecraft::default(),
        governments: Default::default(),
        conflicts: vec![],
        theatres: crate::theatre::default_theatres(),
        access: vec![],
        oil_price: 20.0,
        headlines: vec![],
        flags: vec![],
        player: None,
        player_set_rate: false,
        districts: crate::districts::ownership_1990(),
        by_id: vec![],
        by_id_len: 0,
    };
    for block in &relations.blocks {
        for p in &block.pairs {
            w.set_relation(p.a, p.b, p.value);
        }
    }
    w.reindex();
    Ok(w)
}

/// The provenance of one nation's 1990 figures, for showing to a player.
///
/// The `sources` blocks are the reason this directory exists — iron rule 4 says
/// starting data is transcribed, and a transcription nobody can read is a claim
/// rather than a citation. They were being parsed, validated for non-emptiness,
/// and then dropped on the floor: nothing downstream of the loader could see
/// them, so the only way to check what a number meant was to open the JSON.
/// `spheres-web` serves this at `/api/sources`, which is what makes Brazil's
/// stand-in inflation figure an admission the player encounters rather than a
/// comment in a file they will never open.
///
/// Parsed on demand from the embedded set rather than carried in `WorldState`:
/// this is immutable start-of-game documentation, it must not enter a save, and
/// it must not touch the timeline hash.
pub fn sources_for(id: NationId) -> Vec<String> {
    EMBEDDED_NATIONS
        .iter()
        .filter_map(|s| serde_json::from_str::<NationRecord>(s.json).ok())
        .find(|r| r.id == id)
        .map(|r| r.sources)
        .unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn usa() -> String {
        EMBEDDED_NATIONS
            .iter()
            .find(|s| s.file.ends_with("usa.json"))
            .expect("usa in the embedded set")
            .json
            .to_string()
    }

    #[test]
    fn every_embedded_nation_parses_and_validates() {
        let nations = parse_nations(EMBEDDED_NATIONS).expect("nation files parse");
        let rel = parse_relations(&EMBEDDED_RELATIONS).expect("relations parse");
        validate(&nations, &rel, EMBEDDED_RELATIONS.file).expect("cross-references resolve");
        assert_eq!(nations.len(), start_nations().len());
    }

    #[test]
    fn an_unknown_field_is_refused() {
        // The whole point of deny_unknown_fields: `debt_gpd` must not be a
        // nation that quietly starts with no debt.
        let broken = usa().replace("\"debt_gdp\"", "\"debt_gpd\"");
        let err = parse_nations(&[Source { file: "usa.json", json: &broken }])
            .expect_err("a misspelled field must be refused");
        assert_eq!(err.len(), 1);
        assert_eq!(err[0].file, "usa.json");
        assert!(err[0].message.contains("debt_gpd"), "{}", err[0].message);
    }

    fn brazil() -> String {
        EMBEDDED_NATIONS
            .iter()
            .find(|s| s.file.ends_with("brazil.json"))
            .expect("brazil in the embedded set")
            .json
            .to_string()
    }

    #[test]
    fn the_true_brazilian_hyperinflation_is_refused_at_the_door() {
        // Brazil's sources block quotes a 2948% CPI print and a 9394% deposit
        // rate, and carries 2.95 and 2.9. Somebody will eventually notice the
        // gap and "fix" it. What used to happen then was that the world built
        // fine, Brazil's GDP went to +inf a few hundred ticks later, and twelve
        // tests failed a long way from the cause. It is refused at load now,
        // with the ceiling named.
        //
        // The +inf is no longer what would happen — the growth floor in
        // economy.rs closed that route — but the refusal is still right, and
        // for the reason the message now gives: measured with the floor in
        // place, the true pair leaves the world intact and Brazil at 9.3% of
        // its 1990 output. A figure the sim survives and cannot represent is
        // still a figure to refuse at the door.
        let corrected = brazil()
            .replace("\"inflation\": 2.95", "\"inflation\": 29.48")
            .replace("\"interest_rate\": 2.9", "\"interest_rate\": 93.94");
        let err = parse_nations(&[Source { file: "brazil.json", json: &corrected }])
            .expect_err("figures the model cannot hold must be refused");
        assert!(
            err.iter().any(|e| e.message.contains("economy.inflation")
                && e.message.contains("-0.05..=3.0")),
            "the inflation ceiling was not named: {err:?}"
        );
        assert!(
            err.iter().any(|e| e.message.contains("real")
                && e.message.contains("demand channel")),
            "the real-rate channel was not named: {err:?}"
        );
    }

    #[test]
    fn every_nation_can_show_its_working() {
        // The surface, not the storage. A player asking where Brazil's opening
        // inflation came from has to be able to get an answer, and the answer
        // has to include the admission that the figure is a stand-in rather
        // than the print — otherwise the honesty added to the file is honesty
        // nobody in the game can reach.
        for id in start_nations().iter().copied() {
            assert!(
                !sources_for(id).is_empty(),
                "{:?} has no provenance to show",
                id
            );
        }
        let brazil = sources_for(NationId::Brazil).join(" ");
        assert!(brazil.contains("2948%"), "the true print is not reachable");
        assert!(
            brazil.contains("NOT THE TRANSCRIBED PRINTS"),
            "the stand-in is not admitted where a player can see it"
        );
    }

    #[test]
    fn a_plausible_real_rate_is_not_refused() {
        // The guard above must not be so eager that it rejects the roster it
        // ships with. Turkey is the honest high-inflation entry — 60.3% against
        // a 50% policy rate, both transcribed at their true magnitude — and it
        // has to keep loading.
        let nations = parse_nations(EMBEDDED_NATIONS).expect("the shipped roster still parses");
        let turkey = nations.iter().find(|n| n.id == NationId::Turkey).unwrap();
        assert!((turkey.economy.inflation - 0.603).abs() < 1e-9);
        let gap = 0.025 - (turkey.economy.interest_rate - turkey.economy.inflation);
        assert!(gap.abs() <= 0.5, "Turkey's own gap is {gap}");
    }

    #[test]
    fn a_missing_field_names_the_file() {
        let broken = usa().replace("\"stability\"", "\"stabilty\"");
        let err = parse_nations(&[Source { file: "usa.json", json: &broken }])
            .expect_err("a missing required field must be refused");
        assert_eq!(err[0].file, "usa.json");
    }

    #[test]
    fn a_duplicate_id_is_caught_in_pass_two() {
        let one = usa();
        let recs = parse_nations(&[
            Source { file: "usa.json", json: &one },
            Source { file: "usa_copy.json", json: &one },
        ])
        .expect("both parse on their own");
        // Pass two sees what pass one structurally cannot.
        let rel = RelationsFile::default();
        let err = validate(&recs, &rel, "relations_1990.json")
            .expect_err("a duplicate id must be caught");
        assert!(
            err.iter().any(|e| e.message.contains("duplicate id")
                && e.nation.as_deref() == Some("USA")),
            "{err:?}"
        );
    }

    #[test]
    fn a_relation_naming_an_unknown_nation_is_caught() {
        let nations = parse_nations(EMBEDDED_NATIONS).unwrap();
        let rel = RelationsFile {
            blocks: vec![RelationBlock {
                note: vec![],
                pairs: vec![RelationRecord { a: NationId::USA, b: NationId::Bosnia, value: 10.0 }],
            }],
        };
        let err = validate(&nations, &rel, "relations_1990.json")
            .expect_err("Bosnia does not exist in 1990");
        assert!(
            err.iter().any(|e| e.message.contains("Bosnia") && e.file == "relations_1990.json"),
            "{err:?}"
        );
    }

    #[test]
    fn a_dyad_stated_twice_is_caught() {
        let nations = parse_nations(EMBEDDED_NATIONS).unwrap();
        let rel = RelationsFile {
            blocks: vec![RelationBlock {
                note: vec![],
                pairs: vec![
                    RelationRecord { a: NationId::USA, b: NationId::USSR, value: -45.0 },
                    RelationRecord { a: NationId::USSR, b: NationId::USA, value: 20.0 },
                ],
            }],
        };
        let err = validate(&nations, &rel, "relations_1990.json").expect_err("stated twice");
        assert!(err.iter().any(|e| e.message.contains("stated twice")), "{err:?}");
    }

    #[test]
    fn a_missing_start_nation_names_itself() {
        let all = parse_nations(EMBEDDED_NATIONS).unwrap();
        let without_iraq: Vec<NationRecord> =
            all.into_iter().filter(|r| r.id != NationId::Iraq).collect();
        let rel = RelationsFile::default();
        let err = validate(&without_iraq, &rel, "relations_1990.json")
            .expect_err("a hole in the roster must be caught");
        assert!(err.iter().any(|e| e.message.contains("Iraq")), "{err:?}");
    }

    #[test]
    fn provenance_is_required() {
        // Iron rule 4 is enforced, not merely documented: a nation with no
        // sourcing note is a nation whose numbers were invented.
        let broken = usa().replace("\"sources\"", "\"sources_\"");
        let recs = parse_nations(&[Source { file: "usa.json", json: &broken }]);
        // `sources_` is unknown, so this trips deny_unknown_fields first. Drop
        // the block entirely to reach the emptiness check.
        assert!(recs.is_err());

        let empty = NationRecord {
            sources: vec![],
            ..parse_nations(&[Source { file: "usa.json", json: &usa() }]).unwrap()[0].clone()
        };
        let e = check_record("usa.json", &empty);
        assert!(e.iter().any(|x| x.message.contains("transcribed")), "{e:?}");
    }

    #[test]
    fn every_nation_carries_its_sourcing() {
        for r in parse_nations(EMBEDDED_NATIONS).unwrap() {
            assert!(
                !r.sources.is_empty(),
                "{:?} shipped without a sourcing note",
                r.id
            );
        }
    }
}
