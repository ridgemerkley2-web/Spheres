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
    /// What this nation already knew on 1 January 1990.
    ///
    /// Appended after `sources` on purpose: the eight fields above keep the
    /// serialized order they have always had, so a diff of any existing nation
    /// file shows an addition and nothing else.
    #[serde(default)]
    pub tech_1990: Tech1990Record,
}

/// A nation's 1990 technology, transcribed like every other 1990 figure.
///
/// Absent means "not yet authored", which is the state the whole roster is in
/// while the machinery lands ahead of the data, and it is why this defaults
/// rather than being required — a schema that refused to load 137 unauthored
/// files would make the machinery unlandable. An EMPTY `granted` with a `note`
/// is the different and stronger claim: authored, and the answer is nothing.
/// That distinction is the point of the nested shape. A bare array of ids could
/// not tell "nobody has looked at Bhutan yet" from "Bhutan was looked at, the
/// two global series were checked, and neither reaches it".
///
/// Iron rule 4's refusal lives on the individual grant rather than on the block,
/// because the block cannot bind a citation to a technology and the failure mode
/// being guarded is a thin cell that merely looks sourced.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Default)]
#[serde(deny_unknown_fields)]
pub struct Tech1990Record {
    /// Free prose about the nation's 1990 technological position: what was
    /// checked, and — where the answer is nothing — why nothing.
    ///
    /// A note alongside a non-empty `granted` is deliberately allowed rather
    /// than refused. The data-keyed tail will routinely hold one or two
    /// technologies AND need to say which further candidates were examined and
    /// rejected, and refusing that combination would push that reasoning out of
    /// the file into a commit message nobody reads.
    #[serde(default)]
    pub note: Vec<String>,
    #[serde(default)]
    pub granted: Vec<TechGrant>,
}

/// One technology this nation held, and the citation that justifies saying so.
///
/// Grant-level provenance rather than a line in the file's `sources` array,
/// because "an unsourced entry is a refusal, not a default" is only structurally
/// enforceable when the source travels with the cell. A `sources` block can be
/// long, impressive and attached to nothing in particular; this cannot.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct TechGrant {
    /// A `TechDef::id` from `crate::tech::registry()`. A stable id, never an
    /// index — the registry is a concatenation of eight independently authored
    /// files, so every index moves whenever an earlier domain gains an entry.
    pub id: String,
    /// Where the claim that this nation held this technology in January 1990
    /// comes from. Checked for non-emptiness at load; that check is iron rule 4
    /// as a refusal rather than a convention.
    pub source: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct EconomyRecord {
    /// Real GDP, billions of 1990 USD.
    pub gdp_bn: f64,
    /// Population, millions.
    pub population_m: f64,
    /// The annual rate this nation's population was growing at on 1 January
    /// 1990 — a starting condition on all fours with `tfp_trend` beside it: a
    /// 1990 rate, transcribed with a source, then evolved by the model.
    ///
    /// IT REPLACES A FUNCTION OF INCOME, AND THAT IS THE WHOLE POINT. Population
    /// growth used to be read off GDP per head alone, which made it *decreasing*
    /// in income precisely where the mature panel lives — the United States'
    /// labour term ran +0.254 pt/yr in the 1990s and **-0.217 by the 2020s**, a
    /// population shrinking 0.36%/yr against a real +0.7%. Among rich countries
    /// population growth is not a function of income; it is set by migration,
    /// and several of the very richest have the fastest-growing populations.
    /// The sign was backwards, and it was backwards fastest for the richest.
    ///
    /// THE WINDOW ENDS ON 31 DECEMBER 1989, and it is five years wide rather
    /// than one. Both halves matter. A single-year 1990 rate would carry events
    /// this simulation is meant to PRODUCE rather than be handed: the World
    /// Bank's 1990 figures put Kuwait at -27.5%/yr and Jordan at +7.8%/yr, which
    /// is the Iraqi invasion and the expulsion it caused, and transcribing them
    /// would write the Gulf War into the starting data. No window ending on the
    /// eve of the start date can contain any of it. And a five-year compound
    /// rate is a *trend* rather than one year's noise, which is what this field
    /// is for.
    ///
    /// IT IS NOT A 35-YEAR OUTTURN, deliberately. Transcribing the realised
    /// 1990-2025 average would fit the answer and is scripting, not calibration
    /// (iron rule 3). The cost of refusing is real and is stated rather than
    /// hidden: the 1990 trend puts the United States correctly at the top of the
    /// panel and Italy correctly at the bottom, but it misplaces the United
    /// Kingdom, whose population grew slowly into 1990 and quickly after it.
    ///
    /// Unsourced is a refusal, not a default (iron rule 4): this field carries
    /// no `#[serde(default)]`, so a nation file without it fails to load rather
    /// than silently entering at zero.
    pub pop_growth_1990: f64,
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
    /// WHAT THE STATE HELD IN THE BANK on 1 January 1990, in billions of
    /// current US dollars: total reserves including gold.
    ///
    /// Appended last so that every existing key in every existing nation file
    /// keeps the position it has always had, and a diff of one of those files
    /// shows an addition and nothing else.
    ///
    /// THE OBSERVATION IS THE 1989 ONE, and deliberately. A reserve is a STOCK,
    /// and the stock on the morning of 1 January 1990 is the end-1989 figure;
    /// the 1990 observation is where the reserve ended up after a year this
    /// simulation is meant to PRODUCE. That is the same reasoning
    /// `pop_growth_1990` above sets out for ending its window on 31 December
    /// 1989, and for the same reason: no window ending on the eve of the start
    /// date can contain events the game has not played yet.
    ///
    /// `Option`, and absent is a REFUSAL rather than a zero (iron rule 4).
    /// Two kinds of absence, and they are different claims:
    ///   - SEVENTEEN nations have no observation in the series at all, so no
    ///     figure could be sourced. Albania, Angola, Brunei, Bulgaria,
    ///     Cambodia, Cuba, Czechoslovakia, Iran, Mongolia, North Korea, Sao
    ///     Tome, Senegal, Taiwan, the USSR, Vietnam, Yemen and Yugoslavia.
    ///     Taiwan and the USSR are the two where that is a real loss and both
    ///     are recorded in BUGS rather than guessed at.
    ///   - FORTY-ONE were sourced and left out as IMMATERIAL: under the line
    ///     stated below, the stock is spent inside a quarter of a plausible
    ///     deficit and cannot change the shape of a fiscal path.
    /// MATERIALITY, stated mechanically so it is not a per-country judgement:
    /// the figure is carried when it is at least 5% of that nation's own 1990
    /// output -- about six weeks of total state spending for a state spending
    /// 35% of output -- or at least $10bn in absolute terms. 79 of the 137
    /// nations clear it.
    ///
    /// `None` reaches the treasury as "no figure", which seats an empty till.
    /// Skipped on serialization so that a nation without one round-trips to
    /// the file it was read from.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reserves_bn: Option<f64>,
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
    if let Some(res) = r.economy.reserves_bn {
        if !(res.is_finite() && res >= 0.0) {
            e.push(LoadError::nation_level(
                file,
                &who,
                format!("economy.reserves_bn is {res}, expected a non-negative stock in $bn"),
            ));
        }
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

    // The 1990 grant. This belongs in pass one and not pass two: pass two is for
    // what cannot be checked while looking at one file, and every check below is
    // answerable from this record plus `crate::tech::registry()`, which is a
    // compile-time static. Keeping it here also means a modder with three broken
    // files sees all three, which is the collecting contract this function has.
    //
    // NOTHING HERE MAY GO THROUGH `tech::known_serde`. That deserializer is
    // `filter_map(index_of)` and silently drops an id this build does not know —
    // correct for a save written by an older build, which must still load, and
    // catastrophic for authored data, where a typo would produce a nation
    // quietly starting with one technology fewer and no error anywhere. Grants
    // resolve explicitly, and an id that does not resolve is refused.
    let reg = crate::tech::registry();
    for (i, g) in r.tech_1990.granted.iter().enumerate() {
        match crate::tech::index_of(&g.id) {
            None => e.push(LoadError::nation_level(
                file,
                &who,
                format!(
                    "tech_1990.granted[{i}] names {:?}, which is not a technology in \
                     the tree — check it against the ids in spheres-sim/src/tech/",
                    g.id
                ),
            )),
            Some(idx) => {
                let def = &reg[idx as usize];
                // A technology nobody could research yet is a technology nobody
                // could hold yet. `tech::tick` refuses to complete a project
                // before its `earliest_year`, so granting one at a 1990 start is
                // a board state the tick loop forbids anyone to reach — and it
                // is the only cheap check that catches a typo which happens to
                // resolve to a real but wrong id.
                if def.earliest_year > 1990 {
                    e.push(LoadError::nation_level(
                        file,
                        &who,
                        format!(
                            "tech_1990.granted[{i}] is {:?}, whose earliest year is {} — \
                             it cannot be held on 1 January 1990, and the research \
                             engine would refuse to complete it until then",
                            g.id, def.earliest_year
                        ),
                    ));
                }
            }
        }
        if g.source.trim().is_empty() {
            e.push(LoadError::nation_level(
                file,
                &who,
                format!(
                    "tech_1990.granted[{i}] ({:?}) has no source — iron rule 4 says \
                     starting data is transcribed, and an unsourced entry is a \
                     refusal, not a default. Say where the claim that this nation \
                     held this technology in January 1990 comes from, or drop it",
                    g.id
                ),
            ));
        }
        // A repeat is not cosmetic. `TechState::grant_1990` dedups, so the
        // second entry vanishes without a trace and takes its citation with it —
        // and two citations for one cell is a transcription error somebody needs
        // to see rather than a harmless duplicate.
        if let Some(j) = r.tech_1990.granted.iter().take(i).position(|o| o.id == g.id) {
            e.push(LoadError::nation_level(
                file,
                &who,
                format!(
                    "tech_1990.granted[{i}] repeats {:?}, already granted at [{j}] — \
                     one technology, one citation",
                    g.id
                ),
            ));
        }
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
    /// Two fields are absent from the data here for different reasons, and the
    /// comment that used to stand in this place got the second one wrong.
    ///
    /// `political_capital` is derived. Nobody recorded a government's standing
    /// in January 1990, so it is read off the conditions that *were* recorded,
    /// and transcribing it would be inventing.
    ///
    /// `tech` is neither derived nor absent. The old comment claimed it was
    /// "seeded from the TFP trend by the technology module", and no such
    /// mechanism ever existed: `TechState::new` returns an empty known set, and
    /// the `tfp_base` it is handed is a productivity baseline read only by
    /// `apply_bonuses` — there is no code path from `tfp_trend` to `known`. What
    /// the comment described as a mechanism was a default of zero, which is a
    /// claim about the world and a false one. Technology is now transcribed per
    /// nation in `tech_1990`, each grant carrying its own citation, and applied
    /// in `load_world` below rather than here — see the pass-three note for why
    /// neither half of the grant is a fact about one record (BIBLE §8 and iron
    /// rule 4, both amended 2026-08-30).
    pub fn to_nation(&self) -> Nation {
        Nation {
            id: self.id,
            alive: true,
            system: self.system,
            authoritarianism: self.authoritarianism,
            gdp: self.economy.gdp_bn,
            population: self.economy.population_m,
            tfp_trend: self.economy.tfp_trend,
            // The nation's own 1990 demography, carried as the standing
            // difference from what the income-driven transition says at its own
            // 1990 income — so on 1 January 1990 `population_growth` returns the
            // transcribed rate exactly, and moves with the transition after.
            pop_growth_offset: self.economy.pop_growth_1990
                - crate::economy::transition(
                    self.economy.gdp_bn * 1000.0 / self.economy.population_m,
                ),
            inflation: self.economy.inflation,
            interest_rate: self.economy.interest_rate,
            tax_rate: self.economy.tax_rate,
            mil_spend_gdp: self.economy.mil_spend_gdp,
            state_invest_gdp: self.economy.state_invest_gdp,
            priv_invest_gdp: self.economy.priv_invest_gdp,
            social_spend_gdp: None,
            annual_budget: None,
            debt_gdp: self.economy.debt_gdp,
            // THE BOOKS START CLOSED for every nation on the board, including
            // the one the player will pick. The transcribed reserve is not
            // seated here: it is read at the moment a government opens its
            // books (`Command::SetAnnualBudget`), by `reserves_1990_bn` below.
            // Seating it here would put a `Some` into the 1990 save and move
            // `the_1990_start_is_pinned` for a figure no arm reads yet, which
            // is exactly the movement the pin exists to refuse.
            treasury_bn: None,
            debt_bn: None,
            // No 1990 nation has bought this; it exists only after a plan
            // is enacted. `None` keeps the 1990 save byte-identical.
            infra_extraction: None,
            oil_mbd: self.economy.oil_mbd,
            bubble: self.economy.bubble,
            growth_last: 0.0,
            // The transcribed 1990 figures already reflect the 1990 trade
            // portfolio and the 1990 investment share. `None` says so.
            trade_level_paid: None,
            capital_level_paid: None,
            // The ceiling on what consolidation may hand back. See
            // `Nation::state_invest_1990`.
            state_invest_1990: Some(self.economy.state_invest_gdp),
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
        day: 1,
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
        district_population: crate::districts::population_1990(),
        district_population_scale: vec![1.0; crate::nations::nation_count()],
        resources: Default::default(),
        production: Default::default(),
        manufacturing: Default::default(),
        domination: Default::default(),
        by_id: vec![],
        by_id_len: 0,
        resource_have: Default::default(),
        districts_epoch: 0,
        district_population_growth: vec![],
    };
    for block in &relations.blocks {
        for p in &block.pairs {
            w.set_relation(p.a, p.b, p.value);
        }
    }

    // ---- The 1990 technology endowment, in three passes ----
    //
    // Three passes and not one, and the reason is the trap in this arithmetic: a
    // single loop that grants a nation its technology and then rebases it is
    // WRONG. `reference` is the GDP-weighted mean of what the whole world holds,
    // every nation must be scored against the same number, and that number does
    // not exist until the last grant has been applied.
    //
    // It also cannot live in `to_nation`, because neither half of it is a fact
    // about one record. The effects need the built `Nation` — `saturated_tech_tfp`
    // reads investment shares off it — and the reference needs the whole roster.
    //
    // Index i of `w.nations` is index i of `nations`; that identity is what
    // `file_of` already relies on.
    let known: Vec<Vec<u16>> = nations
        .iter()
        .map(|r| {
            r.tech_1990
                .granted
                .iter()
                // Every id resolved in pass one or we never got here, so this
                // cannot drop a grant on the floor the way `known_serde` would.
                .filter_map(|g| crate::tech::index_of(&g.id))
                .collect()
        })
        .collect();
    // Pass one — grant. Every nation, before any reference is computed.
    for (n, k) in w.nations.iter_mut().zip(known.iter()) {
        n.tech.grant_1990(k);
    }
    // Pass two — one reference and one frontier for the whole roster. Both are
    // properties of the finished board and neither exists until the last grant
    // is in; the frontier is here for exactly the reason the reference is, and
    // a loop that granted and rebased nation by nation would get both wrong.
    let reference = crate::tech::world_reference(&w.nations);
    let frontier_1990 = crate::tech::world_frontier(&w.nations);
    // Pass three — take the endowment back out of each nation's productivity
    // base and its distance to the frontier, so the trend the tick reassembles
    // is the transcribed one and neither the technology nor the standing 1990
    // gap is paid for twice.
    for (n, r) in w.nations.iter_mut().zip(nations.iter()) {
        crate::tech::rebase_to_transcribed(n, r.economy.tfp_trend, reference, frontier_1990);
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

/// What this nation held in reserve on 1 January 1990, in billions of dollars,
/// or `None` if no figure could be sourced for it.
///
/// The same posture as `sources_for` and `tech_1990_for` and for the same
/// reasons: parsed on demand from the embedded set rather than carried on
/// `Nation`, because it is an immutable start-of-game fact, it must not enter a
/// save, and IT MUST NOT TOUCH THE TIMELINE HASH. A stock read at the moment a
/// government opens its books is exactly that kind of fact.
pub fn reserves_1990_bn(id: NationId) -> Option<f64> {
    EMBEDDED_NATIONS
        .iter()
        .filter_map(|s| serde_json::from_str::<NationRecord>(s.json).ok())
        .find(|r| r.id == id)
        .and_then(|r| r.economy.reserves_bn)
}

/// What this nation was granted on 1 January 1990, and why, for showing to a
/// player.
///
/// The same argument as `sources_for` and the same posture. Grant citations that
/// no surface can reach would make iron rule 4's falsifiability claim a claim
/// about a file nobody opens, which is the exact complaint the note above makes
/// about `sources` in the years it was parsed, validated and then dropped on the
/// floor. Parsed on demand: this is immutable start-of-game documentation, it
/// must not enter a save, and it must not touch the timeline hash.
pub fn tech_1990_for(id: NationId) -> Tech1990Record {
    EMBEDDED_NATIONS
        .iter()
        .filter_map(|s| serde_json::from_str::<NationRecord>(s.json).ok())
        .find(|r| r.id == id)
        .map(|r| r.tech_1990)
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

    // ---- The 1990 technology endowment ----
    //
    // `usa()` NOW CARRIES AN AUTHORED `tech_1990` BLOCK. It did not when these
    // fixtures were written, and each of them spliced one in; against the
    // authored roster that produces two `tech_1990` keys and serde refuses the
    // file for a duplicate field, which is a refusal for the wrong reason and
    // would let every assertion below pass without ever exercising what it
    // names. So the authored block is removed first and the fixture's own put in
    // its place — the fixture changed, no assertion did.
    //
    // Removed through `serde_json::Value` rather than by string surgery on
    // purpose: a textual strip would be pinned to the formatting the merge
    // happened to write, and would start silently splicing a second block again
    // the first time a nation file is reformatted.
    fn usa_without_tech() -> String {
        let mut v: serde_json::Value = serde_json::from_str(&usa()).expect("usa.json parses");
        let removed = v
            .as_object_mut()
            .expect("a nation file is an object")
            .remove("tech_1990");
        assert!(
            removed.is_some(),
            "usa.json no longer carries a tech_1990 block — these fixtures assume \
             one is present and must be checked, not silently repaired"
        );
        serde_json::to_string_pretty(&v).expect("re-serializes")
    }

    // Anchored on the `"sources"` key, which every nation file has and which is
    // the last of the eight original top-level fields.
    fn usa_with_tech(block: &str) -> String {
        usa_without_tech().replace("\"sources\"", &format!("{block},\n  \"sources\""))
    }

    fn granted(json: &str) -> String {
        usa_with_tech(&format!("\"tech_1990\": {{ \"granted\": [{json}] }}"))
    }

    #[test]
    fn a_granted_technology_the_tree_does_not_have_is_refused() {
        // The failure this exists for is not a mod author inventing a
        // technology; it is a typo. `tech::known_serde` would drop this id
        // silently — correct for a save written by a build that no longer has
        // it, and fatal for authored data, where the nation would simply start
        // one technology short with nothing anywhere to say so.
        let broken = granted(
            "{ \"id\": \"comp_microprocesor\", \"source\": \"a real citation, a misspelled id\" }",
        );
        let err = parse_nations(&[Source { file: "usa.json", json: &broken }])
            .expect_err("an id the tree does not have must be refused");
        assert_eq!(err[0].file, "usa.json");
        assert_eq!(err[0].nation.as_deref(), Some("USA"));
        assert!(err[0].message.contains("comp_microprocesor"), "{}", err[0].message);
    }

    #[test]
    fn a_granted_technology_with_no_source_is_refused() {
        // Iron rule 4 as a refusal rather than a convention: an unsourced entry
        // is not a default. Whitespace is not a source either — that is the
        // form the refusal actually arrives in, because a blank string is
        // conspicuous and two spaces are not.
        let broken = granted("{ \"id\": \"comp_microprocessor\", \"source\": \"   \" }");
        let err = parse_nations(&[Source { file: "usa.json", json: &broken }])
            .expect_err("an unsourced grant must be refused");
        assert!(
            err.iter().any(|e| e.message.contains("no source")
                && e.message.contains("transcribed")),
            "the rule was not named: {err:?}"
        );
    }

    #[test]
    fn the_same_technology_granted_twice_is_refused() {
        // `grant_1990` dedups, so the second entry disappears and takes its
        // citation with it. Two citations for one cell is a transcription error
        // and has to be seen rather than silently reconciled.
        let broken = granted(
            "{ \"id\": \"comp_microprocessor\", \"source\": \"one\" }, \
             { \"id\": \"comp_microprocessor\", \"source\": \"two\" }",
        );
        let err = parse_nations(&[Source { file: "usa.json", json: &broken }])
            .expect_err("a repeated grant must be refused");
        assert!(
            err.iter().any(|e| e.message.contains("repeats")),
            "{err:?}"
        );
    }

    #[test]
    fn a_technology_that_did_not_exist_yet_cannot_be_held_in_1990() {
        // The catch for a typo that resolves to a real but wrong id, and the
        // rule the research engine already enforces going forward: `tech::tick`
        // refuses to complete a project before its `earliest_year`, so a 1997
        // technology held at a 1990 start is a board state no nation could ever
        // have reached by playing.
        let late = crate::tech::registry()
            .iter()
            .find(|d| d.earliest_year > 1990)
            .expect("the tree has technologies later than 1990");
        let broken = granted(&format!(
            "{{ \"id\": \"{}\", \"source\": \"a citation for something that had not happened\" }}",
            late.id
        ));
        let err = parse_nations(&[Source { file: "usa.json", json: &broken }])
            .expect_err("a post-1990 technology must be refused at a 1990 start");
        assert!(
            err.iter().any(|e| e.message.contains(&late.earliest_year.to_string())
                && e.message.contains("1 January 1990")),
            "the year was not named: {err:?}"
        );
    }

    #[test]
    fn a_misspelled_tech_1990_key_is_refused() {
        // `deny_unknown_fields` is what stops the whole block being optional in
        // the dangerous direction. `tech_1990` defaults when absent, so without
        // this a misspelled key would be a nation that quietly starts knowing
        // nothing while its file plainly says otherwise.
        let broken = usa_with_tech("\"tech_l990\": { \"granted\": [] }");
        let err = parse_nations(&[Source { file: "usa.json", json: &broken }])
            .expect_err("a misspelled block key must be refused");
        assert!(err[0].message.contains("tech_l990"), "{}", err[0].message);
    }

    #[test]
    fn an_unknown_field_inside_the_block_is_refused() {
        let broken = usa_with_tech(
            "\"tech_1990\": { \"granted\": [{ \"id\": \"core_pcr\", \"citation\": \"x\" }] }",
        );
        let err = parse_nations(&[Source { file: "usa.json", json: &broken }])
            .expect_err("`citation` is not `source`");
        assert!(err[0].message.contains("citation"), "{}", err[0].message);
    }

    #[test]
    fn a_well_formed_grant_reaches_the_nation_it_was_written_for() {
        // The positive control for every refusal above. Without it the schema
        // could be rejecting everything, including the correct thing, and every
        // other test in this group would still pass.
        let json = granted(
            "{ \"id\": \"core_pcr\", \"source\": \"a citation\" }, \
             { \"id\": \"comm_digital_switching\", \"source\": \"another citation\" }",
        );
        let recs = parse_nations(&[Source { file: "usa.json", json: &json }])
            .expect("a well-formed grant must load");
        assert_eq!(recs[0].tech_1990.granted.len(), 2);

        // A one-nation roster cannot go through `load_world` — pass two would
        // report 136 missing start nations — so the grant is applied here the
        // way the loader's first pass applies it.
        let mut n = recs[0].to_nation();
        let ids: Vec<u16> = recs[0]
            .tech_1990
            .granted
            .iter()
            .map(|g| crate::tech::index_of(&g.id).expect("validated above"))
            .collect();
        n.tech.grant_1990(&ids);
        assert_eq!(n.tech.count(), 2, "the grant did not land");
        assert!(n.tech.knows("core_pcr"));
        assert!(n.tech.knows("comm_digital_switching"));
        assert!(
            crate::tech::saturated_tech_tfp(&n) > 0.0,
            "a held technology that is worth nothing is a grant that did nothing"
        );
    }

    #[test]
    fn the_authored_endowment_leaves_every_transcribed_trend_exactly_where_it_was() {
        // THE RE-POINTED INERT-MACHINERY PROOF.
        //
        // This is `with_nothing_authored_every_nation_starts_exactly_where_it_did`
        // after the 1990 board landed, re-pointed exactly as that test's own
        // comment instructed ("When Tier A lands this test starts failing for the
        // nations that carry data, and that is the signal it exists to give ...
        // Re-point it then; do not delete it"). Nothing it asserted has been
        // dropped except the two claims the data itself falsified, and both are
        // named here rather than quietly removed:
        //
        //   - `tech.count() == 0` for every nation. Now `count()` must equal the
        //     number of grants that nation's file actually carries, which is the
        //     stronger statement: it catches a grant silently dropped on the
        //     floor as well as one invented, and it is what `known_serde` would
        //     have done to a typo if pass one had not refused it first.
        //   - `tfp_1990_offset` is POSITIVE ZERO for every nation. It cannot be
        //     any more, and not only for the nations that hold something: the
        //     rebase pays `(s - reference)` to everybody, so a nation holding
        //     nothing now carries `-reference`. That is the mechanism that moves
        //     both golden hashes, and moving them is what the endowment is for.
        //
        // What survives untouched, and is the load-bearing claim: the transcribed
        // trend is still bit-for-bit the figure in the JSON, and `tfp_base` plus
        // the offset still reconstructs it. Technology is not paid for twice.
        let recs = parse_nations(EMBEDDED_NATIONS).expect("the roster parses");
        let w = load_world(EMBEDDED_NATIONS, &EMBEDDED_RELATIONS, GameRules::default())
            .expect("the roster loads");
        assert_eq!(w.nations.len(), recs.len());

        let mut authored = 0usize;
        let mut grants = 0usize;
        let mut worst = 0.0f64;
        for (n, r) in w.nations.iter().zip(recs.iter()) {
            let held = r.tech_1990.granted.len();
            grants += held;
            if held > 0 {
                authored += 1;
            }
            assert_eq!(
                n.tech.count(),
                held,
                "{:?}: the file grants {} technologies and the nation holds {}",
                n.id,
                held,
                n.tech.count()
            );
            // Unchanged from the original, and the reason the whole rebasing
            // pass exists: whatever the endowment is worth, the trend the tick
            // loop reassembles is still the one that was transcribed.
            assert_eq!(
                n.tfp_trend.to_bits(),
                r.economy.tfp_trend.to_bits(),
                "{:?}: the transcribed trend did not survive construction",
                n.id
            );
            // The construction residual, against the same 1e-12 bar
            // `granting_the_1990_stock_does_not_move_the_transcribed_trend`
            // already uses. Not a widened tolerance: the original compared bits
            // because the only arithmetic was `x - 0.0`, and there is real
            // arithmetic now.
            let residual = (n.tech.tfp_base + n.tech.tfp_1990_offset - r.economy.tfp_trend).abs();
            worst = worst.max(residual);
            assert!(
                residual <= 1e-12,
                "{:?}: base {} plus offset {} does not reconstruct the transcribed {}",
                n.id,
                n.tech.tfp_base,
                n.tech.tfp_1990_offset,
                r.economy.tfp_trend
            );
            assert!(n.tech.tfp_1990_offset.is_finite(), "{:?}", n.id);
            // Both unchanged. The oil identity is the no-double-count rule: a
            // producer granted 3-D seismic and horizontal drilling must not walk
            // `oil_mbd` upward on the first tick for barrels its transcribed
            // figure already contains.
            assert_eq!(
                n.tech.oil_yield_applied,
                n.tech.bonus.oil_yield_eff(),
                "{:?}: the granted oil yield was not marked as already applied",
                n.id
            );
            assert_eq!(n.tech.absorption_rate, 0.0, "{:?}", n.id);
            if held == 0 {
                assert_eq!(n.tech.count(), 0, "{:?}", n.id);
                assert_eq!(n.tech.oil_yield_applied, 0.0, "{:?}", n.id);
            }
        }
        // A guard against this test going quietly vacuous the way its ancestor
        // did: if the board is ever emptied, say so here rather than passing.
        assert!(
            authored >= 100 && grants >= 300,
            "the board has shrunk to {authored} nations and {grants} grants — \
             re-point this test deliberately, do not let it pass on an empty board"
        );
        assert!(
            crate::tech::world_reference(&w.nations) > 0.0,
            "a world where 137 nations hold {grants} technologies has a zero reference"
        );
        println!("worst construction residual across {} nations: {worst:e}", w.nations.len());
    }

    #[test]
    fn a_nation_can_show_what_it_was_granted_and_why() {
        // The same argument as `every_nation_can_show_its_working`: a citation no
        // surface can reach is a claim rather than a citation.
        for id in start_nations().iter().copied() {
            let t = tech_1990_for(id);
            for g in &t.granted {
                assert!(
                    !g.source.trim().is_empty(),
                    "{:?} shows a grant of {} with nothing behind it",
                    id,
                    g.id
                );
            }
            // Every nation is now AUTHORED, and the schema's whole point is that
            // an empty `granted` with a `note` is the different and stronger
            // claim — "looked at, and the answer is nothing" — rather than "not
            // yet looked at". That distinction is only worth anything if the
            // note is actually there, so it is asserted rather than assumed.
            assert!(
                !t.note.is_empty(),
                "{:?} was left unauthored: no note and {} grants",
                id,
                t.granted.len()
            );
        }
        // The accessor reaches the authored block. This replaces an assertion
        // that `tech_1990_for(USA)` was the default, which was a placeholder
        // pinned to the state of the roster before any of it was written.
        let usa = tech_1990_for(NationId::USA);
        assert!(
            usa.granted.len() > 20,
            "the United States shows {} technologies for 1990",
            usa.granted.len()
        );
        assert!(usa.granted.iter().any(|g| g.id == "core_pcr"));
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
