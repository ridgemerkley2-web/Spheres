use serde::{Deserialize, Serialize};

pub use crate::nations::{
    adjacent, all_nations, claim_share, majors, nation_count, patrons, registry, start_nations,
    successor_nations, Claim, NationDef, NationId, NationRow, ROSTER,
};
use crate::tech::TechState;

/// SplitMix64 — the single seeded RNG. Determinism is sacred.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Rng {
    pub state: u64,
}
impl Rng {
    pub fn new(seed: u64) -> Self {
        Rng { state: seed }
    }
    pub fn next_u64(&mut self) -> u64 {
        self.state = self.state.wrapping_add(0x9E3779B97F4A7C15);
        let mut z = self.state;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58476D1CE4E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D049BB133111EB);
        z ^ (z >> 31)
    }
    /// Uniform in [0,1)
    pub fn f64(&mut self) -> f64 {
        (self.next_u64() >> 11) as f64 / (1u64 << 53) as f64
    }
    /// Uniform in [lo, hi)
    pub fn range(&mut self, lo: f64, hi: f64) -> f64 {
        lo + self.f64() * (hi - lo)
    }
    pub fn chance(&mut self, p: f64) -> bool {
        self.f64() < p
    }
}

/// Symmetric relations, held as a dense lower triangle so a lookup is an index
/// rather than a search.
///
/// It was a `Vec<(NationId, NationId, f64)>` scanned linearly, and the tech
/// module calls `relation()` inside a per-nation loop — O(n^2 * |relations|),
/// which is O(n^4). Invisible at thirty nations and roughly 650 million
/// comparisons a month at the hundred and ninety the design calls for, which
/// would have made the century-run CI invariant impossible before anyone wrote
/// a single new nation.
///
/// Serialized as (a, b, value) triples rather than as the raw array, so a save
/// stays self-describing and survives the roster changing shape — the same
/// reason the technology tree writes ids instead of indices.
#[derive(Clone, Debug, Default)]
pub struct Relations {
    tri: Vec<f64>,
}

impl Relations {
    fn slot(a: NationId, b: NationId) -> usize {
        let (hi, lo) = {
            let (x, y) = (a.index(), b.index());
            if x >= y { (x, y) } else { (y, x) }
        };
        hi * (hi + 1) / 2 + lo
    }
    fn ensure(&mut self) {
        let want = nation_count() * (nation_count() + 1) / 2;
        if self.tri.len() != want {
            self.tri.resize(want, 0.0);
        }
    }
    pub fn get(&self, a: NationId, b: NationId) -> f64 {
        self.tri.get(Self::slot(a, b)).copied().unwrap_or(0.0)
    }
    pub fn set(&mut self, a: NationId, b: NationId, v: f64) {
        self.ensure();
        let s = Self::slot(a, b);
        self.tri[s] = v;
    }
    /// Every stored pair, in a fixed order. Deterministic by construction —
    /// there is no map here whose iteration order could vary.
    ///
    /// The row and column are carried forward rather than recovered from the
    /// slot index, and that is the whole of the century run's super-linearity.
    /// Recovering them counted up from zero for every one of the n(n+1)/2 slots,
    /// which made a single sweep of the matrix O(n^3): at the 160-wide roster
    /// that is 1.36 million loop iterations per sweep, once a month, 1.6 billion
    /// over a century, and it was the largest cost in the tick — larger than the
    /// technology tree, which is what everyone suspected. It also scaled with
    /// the *width* of the matrix rather than with how many nations were alive,
    /// which is why it hid: the 30-nation world it was written in swept 435
    /// slots and this cost nothing at all.
    ///
    /// Same pairs, same order, same values. `pairs_mut_walks_the_triangle_in_order`
    /// holds it against the arithmetic the inverted form did.
    pub fn pairs_mut(&mut self) -> impl Iterator<Item = (NationId, NationId, &mut f64)> {
        self.ensure();
        let ids = all_nations();
        let (mut hi, mut lo) = (0usize, 0usize);
        self.tri.iter_mut().filter_map(move |v| {
            let (row, col) = (hi, lo);
            if lo == hi {
                hi += 1;
                lo = 0;
            } else {
                lo += 1;
            }
            match (ids.get(row), ids.get(col)) {
                (Some(a), Some(b)) => Some((*a, *b, v)),
                _ => None,
            }
        })
    }
}

impl Serialize for Relations {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        // Carried forward for the same reason as in `pairs_mut`, and it matters
        // more here than it looks: `state_hash` serializes the whole world, the
        // determinism tests hash after every month, and this walk was O(n^3) in
        // every one of them.
        let mut out: Vec<(NationId, NationId, f64)> = vec![];
        let (mut hi, mut lo) = (0usize, 0usize);
        for v in self.tri.iter() {
            let (row, col) = (hi, lo);
            if lo == hi {
                hi += 1;
                lo = 0;
            } else {
                lo += 1;
            }
            if *v == 0.0 {
                continue;
            }
            if let (Some(a), Some(b)) = (all_nations().get(row), all_nations().get(col)) {
                out.push((*b, *a, *v));
            }
        }
        out.serialize(s)
    }
}

impl<'de> Deserialize<'de> for Relations {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        // Read the codes as plain strings and drop any this build cannot
        // resolve, rather than failing the whole load or — far worse — letting
        // a shifted roster reinterpret a pair as its neighbour. Same rule the
        // technology tree follows for an id from a later build, and the same
        // bug it was written to prevent.
        let triples = Vec::<(String, String, f64)>::deserialize(d)?;
        let mut r = Relations::default();
        r.ensure();
        for (a, b, v) in triples {
            if let (Some(a), Some(b)) = (NationId::from_code(&a), NationId::from_code(&b)) {
                r.set(a, b, v);
            }
        }
        Ok(r)
    }
}

/// A nation that has not fired anything yet has full magazines, and so does one
/// loaded from a save written before magazines existed.
fn full_magazines() -> f64 {
    1.0
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq)]
pub enum EconomySystem {
    Market,
    Command,
}

pub const BUDGET_MINISTRIES: usize = 10;
pub const BUDGET_HEALTH: usize = 0;
pub const BUDGET_EDUCATION: usize = 1;
pub const BUDGET_FAMILIES: usize = 2;
pub const BUDGET_PENSIONS: usize = 3;
pub const BUDGET_INFRASTRUCTURE: usize = 4;
pub const BUDGET_INDUSTRY: usize = 5;
pub const BUDGET_SCIENCE: usize = 6;
pub const BUDGET_DEFENSE: usize = 7;
pub const BUDGET_SECURITY: usize = 8;
pub const BUDGET_DIPLOMACY: usize = 9;
pub const BUDGET_CAPS: [f64; BUDGET_MINISTRIES] = [
    0.15, // health
    0.12, // education
    0.15, // families
    0.20, // pensions
    0.15, // infrastructure
    0.12, // industry and energy
    0.08, // science
    0.35, // defense
    0.12, // security
    0.08, // diplomacy
];

/// A cabinet's enacted fiscal-year plan. Every allocation is a share of GDP,
/// which keeps the same controls readable for a small state and a superpower.
/// `reference` is the settlement inherited when the detailed books were first
/// opened, so merely adopting this surface does not rewrite the 1990 world.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct AnnualBudget {
    pub fiscal_year: i32,
    pub allocations: [f64; BUDGET_MINISTRIES],
    pub reference: [f64; BUDGET_MINISTRIES],
}

impl AnnualBudget {
    pub fn inherited(fiscal_year: i32, social: f64, investment: f64, defense: f64) -> Self {
        // Both decompositions preserve their old aggregate exactly.
        let allocations = [
            social * 0.25,
            social * 0.18,
            social * 0.20,
            social * 0.28,
            investment * 0.55,
            investment * 0.30,
            investment * 0.15,
            defense,
            social * 0.07,
            social * 0.02,
        ];
        Self { fiscal_year, allocations, reference: allocations }
    }

    pub fn total(&self) -> f64 {
        self.allocations.iter().sum()
    }

    pub fn social_total(&self) -> f64 {
        [
            BUDGET_HEALTH,
            BUDGET_EDUCATION,
            BUDGET_FAMILIES,
            BUDGET_PENSIONS,
            BUDGET_SECURITY,
            BUDGET_DIPLOMACY,
        ]
        .iter()
        .map(|i| self.allocations[*i])
        .sum()
    }

    pub fn investment_total(&self) -> f64 {
        self.allocations[BUDGET_INFRASTRUCTURE]
            + self.allocations[BUDGET_INDUSTRY]
            + self.allocations[BUDGET_SCIENCE]
    }

    pub fn defense(&self) -> f64 {
        self.allocations[BUDGET_DEFENSE]
    }

    pub fn gap(&self, ministry: usize) -> f64 {
        self.allocations[ministry] - self.reference[ministry]
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Nation {
    pub id: NationId,
    pub alive: bool,
    pub system: EconomySystem,
    /// 0..1 — how authoritarian; gates elections and colors stability
    pub authoritarianism: f64,

    // --- Economy ---
    /// Real GDP, billions of 1990 USD
    pub gdp: f64,
    /// Population, millions
    pub population: f64,
    /// Total factor productivity growth trend, annual (e.g. 0.012)
    pub tfp_trend: f64,
    /// This nation's demography, as the standing difference between its own
    /// transcribed 1990 population growth and what the income-driven
    /// demographic transition would have said at its 1990 income. The assembled
    /// rate is `economy::transition(gdp_pc) + pop_growth_offset`, so on 1
    /// January 1990 it is exactly the transcribed figure and it then moves with
    /// the transition — the same `base + 1990 offset` construction the
    /// technology branch uses for `tfp_base + tfp_1990_offset`, and for the same
    /// reason: a 1990 fact must survive the model's own dynamics rather than be
    /// overwritten by them.
    ///
    /// Defaulted to 0.0 so an older save loads: zero is precisely "behave as the
    /// pure income function did", which is what a save written before this
    /// field existed was doing.
    #[serde(default)]
    pub pop_growth_offset: f64,
    /// Annual inflation rate (0.04 = 4%)
    pub inflation: f64,
    /// Central bank policy rate, annual
    pub interest_rate: f64,
    /// Tax take as share of GDP
    pub tax_rate: f64,
    /// Military spending as share of GDP
    pub mil_spend_gdp: f64,
    /// State/public investment as share of GDP
    pub state_invest_gdp: f64,
    /// Private investment share of GDP (endogenous-ish)
    pub priv_invest_gdp: f64,
    /// Player-directed welfare and public services. `None` preserves the
    /// political system's automatic 17--22% baseline for old saves and for AI
    /// governments that have not chosen a different settlement.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub social_spend_gdp: Option<f64>,
    /// The player's detailed fiscal plan. AI and old saves keep `None` and use
    /// the calibrated legacy envelopes until somebody opens the books.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub annual_budget: Option<AnnualBudget>,
    /// Public debt as share of GDP
    pub debt_gdp: f64,
    /// Oil production, million barrels/day (0 for non-producers)
    pub oil_mbd: f64,
    /// Asset bubble intensity 0..1 (Japan starts hot)
    pub bubble: f64,
    /// Last 12m real growth, annualized — for briefings
    pub growth_last: f64,
    /// How much of the trade level gain this nation has already been paid, as a
    /// fraction of output. Defaulted so an older save loads: a save written
    /// before this existed carries a GDP that already reflects what it was
    /// paid, and seeding at zero would pay it a second time — so `None` means
    /// "seed from the current portfolio and pay nothing for it".
    #[serde(default)]
    pub trade_level_paid: Option<f64>,
    /// How much of the capital-deepening LEVEL a nation has already been paid,
    /// in logs. `None` is load-bearing for exactly the same reason as above, and
    /// for one more: the 1990 transcription already reflects the 1990
    /// investment share, so a transcribed starting figure must never be
    /// repriced. See the `CAPITAL_ELASTICITY` block in economy.rs.
    #[serde(default)]
    pub capital_level_paid: Option<f64>,
    // A THIRD MEMBER OF THIS FAMILY IS OWED AND IS NOT HERE: `oil_level_paid`,
    // for the oil terms-of-trade windfall, which `economy::tick` still pays as a
    // permanent growth RATE reaching +500%/yr. The field was written, the fix
    // was measured, and it is blocked on an owner ruling rather than on a
    // design question — the block above the producer arm in economy.rs has the
    // numbers. Adding it is a save-format change and should land with the fix,
    // not before it.
    /// The public investment share this nation entered 1990 with, and the
    /// ceiling on what fiscal consolidation may hand back.
    ///
    /// `politics.rs`'s debt rule cuts `state_invest_gdp` by 0.5% a month while
    /// debt runs above 85% of output and NEVER restored it — a one-way ratchet.
    /// That was cheap while the investment share only bought a small growth
    /// rate; once it buys a permanent LEVEL of output (see `capital_level_paid`)
    /// an irreversible cut is an irreversible loss, and a government that
    /// consolidates its way back to fiscal space would never recover the output
    /// it gave up. Measured on China, which crosses 85% around 1995 and is back
    /// under 30% by 2010: the share fell 0.240 -> 0.167 and stayed there, worth
    /// -0.294 pt/yr of output level for the remaining twenty-five years.
    ///
    /// `None` — a mid-run successor state, or a save written before this field
    /// — means no recovery, which is exactly the behaviour those worlds already
    /// had. A successor state has no transcribed 1990 share and inventing one
    /// would be a refusal (iron rule 4).
    #[serde(default)]
    pub state_invest_1990: Option<f64>,

    // --- Politics ---
    /// 0..100 — regime stability/legitimacy
    pub stability: f64,
    /// Nationalities/separatism strain 0..1 (USSR's clock)
    pub separatism: f64,

    // --- Military ---
    /// **Force structure** — trained people and platforms, the first of BIBLE
    /// §6's three stocks. Decades to build, impossible to surge. It is *not* a
    /// measure of what this nation can bring to bear anywhere: three multipliers
    /// stand between this number and combat power, and each of them is a place a
    /// decision lives. See `war::committed_force`.
    pub mil_strength: f64,
    /// **Munitions** — 0..1, months of high-intensity fire left in the
    /// magazines. Drains in weeks at rung 6 and rebuilds over years, and that
    /// mismatch is what makes short wars end for logistical rather than
    /// political reasons.
    #[serde(default = "full_magazines")]
    pub munitions: f64,
    /// 0..1, accumulates in war, decays in peace
    pub war_exhaustion: f64,
    pub nuclear: bool,
    /// What this nation has bought, has on order, and is still flying.
    /// Defaulted so a save written before procurement existed still loads.
    #[serde(default)]
    pub arsenal: crate::arsenal::Arsenal,

    // --- Political capital ---
    /// 0..100. The second of the two currencies SPEC spines the game on, beside
    /// economic output: what a government can still spend on doing things its
    /// people will not thank it for. Earned by delivering — growth, stable
    /// prices, order — and burned by asking for sacrifice.
    ///
    /// An authoritarian government is not exempt. It holds capital differently,
    /// drawing on coercion rather than consent, which makes its stock steadier
    /// and its ceiling lower: it can act without permission and cannot recover
    /// quickly from spending everything.
    ///
    /// Defaulted so older saves load; `ensure_political_capital` seats it from
    /// the nation's own condition rather than from a transcribed figure, since
    /// there is no such figure to transcribe.
    #[serde(default)]
    pub political_capital: f64,

    // --- Technology ---
    /// What this nation knows, what it is paying to learn, and what that
    /// knowledge has done to it. Defaulted so older saves still load.
    #[serde(default)]
    pub tech: TechState,
}

impl Nation {
    /// The inherited welfare state before a player deliberately reprioritises
    /// the budget. More open systems carry a larger automatic social compact.
    pub fn baseline_social_spend(&self) -> f64 {
        0.17 + (1.0 - self.authoritarianism) * 0.05
    }

    pub fn social_spend(&self) -> f64 {
        self.social_spend_gdp.unwrap_or_else(|| self.baseline_social_spend())
    }

    pub fn budget_for(&self, fiscal_year: i32) -> AnnualBudget {
        self.annual_budget.clone().unwrap_or_else(|| {
            AnnualBudget::inherited(
                fiscal_year,
                self.social_spend(),
                self.state_invest_gdp,
                self.mil_spend_gdp,
            )
        })
    }

    pub fn budget_gap(&self, ministry: usize) -> f64 {
        self.annual_budget.as_ref().map(|b| b.gap(ministry)).unwrap_or(0.0)
    }
}

/// What a patron sends a client. Money keeps a government standing; guns let it
/// fight the neighbour the patron would rather not fight itself.
#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum AidKind {
    Economic,
    Arms,
}
impl AidKind {
    pub fn label(&self) -> &'static str {
        match self {
            AidKind::Economic => "economic aid",
            AidKind::Arms => "arms",
        }
    }
    pub fn parse(s: &str) -> Option<AidKind> {
        match s.trim().to_lowercase().as_str() {
            "economic" | "aid" | "money" => Some(AidKind::Economic),
            "arms" | "weapons" | "military" => Some(AidKind::Arms),
            _ => None,
        }
    }
}

/// A standing commitment of a share of the patron's output, renewed every month
/// until someone cancels it.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct AidFlow {
    pub patron: NationId,
    pub client: NationId,
    pub kind: AidKind,
    /// Annual share of the patron's GDP
    pub share_gdp: f64,
    pub since_year: i32,
}

/// A mutual defence guarantee. Stored with `a <= b` so the pair is canonical.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Pact {
    pub a: NationId,
    pub b: NationId,
    pub since_year: i32,
    pub since_month: u32,
}

/// A trade agreement, and how far integration has actually gone since signature.
/// Depth is the whole point: a fresh treaty is a gesture, a mature one is a
/// dependency that the larger partner can pull on.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct TradePact {
    pub a: NationId,
    pub b: NationId,
    /// 0..1
    pub depth: f64,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum CovertOp {
    FundOpposition,
    StirSeparatists,
    SabotageIndustry,
}
impl CovertOp {
    pub fn label(&self) -> &'static str {
        match self {
            CovertOp::FundOpposition => "funding the opposition",
            CovertOp::StirSeparatists => "arming separatists",
            CovertOp::SabotageIndustry => "industrial sabotage",
        }
    }
    pub fn parse(s: &str) -> Option<CovertOp> {
        match s.trim().to_lowercase().as_str() {
            "opposition" | "fund" | "coup" => Some(CovertOp::FundOpposition),
            "separatists" | "separatism" | "stir" => Some(CovertOp::StirSeparatists),
            "sabotage" | "industry" => Some(CovertOp::SabotageIndustry),
            _ => None,
        }
    }
}

/// What a state has done for and to other states short of war, kept together so
/// that `WorldState` grows by one field rather than five.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Statecraft {
    pub pacts: Vec<Pact>,
    pub aid: Vec<AidFlow>,
    pub trade: Vec<TradePact>,
    /// (sponsor, target, heat 0..1) — how well-trodden a covert channel has
    /// become. Nothing gets a service caught like using the same one twice.
    pub covert_heat: Vec<(NationId, NationId, f64)>,
    /// Sparse, like `relations`: a state that has never broken its word is
    /// simply absent and reads as the baseline.
    pub reputation: Vec<(NationId, f64)>,
}

/// What a state's word is worth before it has spent any of it.
pub const BASE_REPUTATION: f64 = 70.0;

/// The legacy war object. Nothing writes one any more — it survives only so
/// that a save written before the commitment ladder still loads, and `load()`
/// drains it into `conflicts` through `migrate_legacy_wars`.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct War {
    pub attacker: NationId,
    pub defender: NationId,
    pub attacker_allies: Vec<NationId>,
    pub defender_allies: Vec<NationId>,
    pub start_year: i32,
    pub start_month: u32,
    /// Net progress: positive favors attacker side, [-100, 100]
    pub progress: f64,
}

/// What a belligerent is trying to achieve, which decides what its committed
/// force is *for*. Not decoration: each of these is a different pair of
/// multipliers on killing and on taking ground.
#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum Objective {
    /// Stop them having it. No seizing at all, and theirs is halved.
    Deny,
    /// Break the thing rather than hold the ground.
    Degrade,
    /// Take it.
    Seize,
    /// Keep what you have and stop paying for what you do not.
    Hold,
    /// Hold the ground down rather than the enemy, and stop them rebuilding.
    Stabilise,
    /// Leave, one rung a month, and stop the bleeding.
    Withdraw,
}
impl Objective {
    pub fn label(&self) -> &'static str {
        match self {
            Objective::Deny => "deny",
            Objective::Degrade => "degrade",
            Objective::Seize => "seize",
            Objective::Hold => "hold",
            Objective::Stabilise => "stabilise",
            Objective::Withdraw => "withdraw",
        }
    }
    pub fn parse(s: &str) -> Option<Objective> {
        Some(match s.trim().to_lowercase().as_str() {
            "deny" => Objective::Deny,
            "degrade" => Objective::Degrade,
            "seize" | "take" => Objective::Seize,
            "hold" => Objective::Hold,
            "stabilise" | "stabilize" => Objective::Stabilise,
            "withdraw" | "leave" | "out" => Objective::Withdraw,
            _ => return None,
        })
    }
}

/// How much you are willing to break to win faster. The click where winning
/// sooner and keeping your host's airbase are the same decision.
#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum Roe {
    Restrained,
    Standard,
    Unrestricted,
}
impl Roe {
    pub fn label(&self) -> &'static str {
        match self {
            Roe::Restrained => "restrained",
            Roe::Standard => "standard",
            Roe::Unrestricted => "unrestricted",
        }
    }
    pub fn parse(s: &str) -> Option<Roe> {
        Some(match s.trim().to_lowercase().as_str() {
            "restrained" | "restrict" => Roe::Restrained,
            "standard" | "normal" => Roe::Standard,
            "unrestricted" | "unrestrained" | "total" => Roe::Unrestricted,
            _ => return None,
        })
    }
}

/// Recomputed every month from the rungs on the board rather than stored as
/// truth. A conflict slides between these; it is not born one.
#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum ConflictClass {
    Conventional,
    Irregular,
    Frozen,
}

/// The nine rungs of BIBLE §6. Rungs 2 through 5 are the statecraft instruments
/// that already exist; the ladder binds them into a sequence rather than
/// duplicating them.
pub const RUNG_NAMES: [&str; 10] = [
    "-",
    "rhetoric",
    "sanctions",
    "arms to a proxy",
    "advisers and intelligence",
    "deniable forces",
    "standoff strike",
    "blockade or limited incursion",
    "full conventional campaign",
    "occupation",
];
pub fn rung_name(r: u8) -> &'static str {
    RUNG_NAMES[(r as usize).min(9)]
}

/// One participant's own war. Each side picks its own rung monthly, and
/// mismatched rungs are the interesting state.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Belligerent {
    pub nation: NationId,
    /// 1..=9
    pub rung: u8,
    /// A publicly announced limit. Binds the automatic and AI responses, and the
    /// opponent reads it.
    pub ceiling: u8,
    pub objective: Objective,
    pub roe: Roe,
    /// 0..1 — the political stock that actually loses modern wars.
    pub resolve: f64,
    /// When resolve crosses this, the objective switches to Withdraw by itself.
    pub red_line: f64,
    pub months_at_rung: u32,
    /// 0..1 — what losing here would mean at home. Home ground is 1.0.
    pub stake: f64,
}

impl Belligerent {
    /// A fresh belligerent at a chosen rung, with everything else at its
    /// default: full resolve, standard rules, no announced ceiling, no red line.
    pub fn new(nation: NationId, rung: u8, objective: Objective) -> Belligerent {
        Belligerent {
            nation,
            rung: rung.clamp(1, 9),
            ceiling: 9,
            objective,
            roe: Roe::Standard,
            resolve: 1.0,
            red_line: 0.0,
            months_at_rung: 0,
            stake: 0.20,
        }
    }
}

/// BIBLE §6 object 1. Persistent, between coalitions, carrying each
/// belligerent's own commitment rung. It does not begin with a declaration or
/// end with a capitulation: it begins when someone climbs and ends when a track
/// collapses — and frozen conflicts do not end at all, they go quiet.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Conflict {
    pub id: u32,
    pub theatre: crate::theatre::TheatreId,
    pub side_a: Vec<NationId>,
    pub side_b: Vec<NationId>,
    /// One entry per participant, in join order. Deterministic by construction.
    pub posture: Vec<Belligerent>,
    /// -1..=+1. +1 means side A holds the ground. Separate from resolve on
    /// purpose: a single progress bar cannot express "holding the ground and
    /// losing the war", and two tracks can.
    pub control: f64,
    pub months: u32,
    /// Consecutive months with nobody above rung 5. Eighteen of them and the
    /// conflict freezes; it does not end, it goes quiet.
    #[serde(default)]
    pub quiet_months: u32,
    pub frozen_since: Option<(i32, u32)>,
    pub start_year: i32,
    pub start_month: u32,
    /// Who opened it. Kept because conquest, reparations and the burned-fingers
    /// flag all need to know who started it years later.
    pub origin_attacker: NationId,
    /// Whether somebody has already crossed a border in force here. The
    /// coalition forms once, at the moment the quarrel becomes an invasion, and
    /// never again however many times the aggressor climbs back up afterwards.
    #[serde(default)]
    pub invasion_declared: bool,
    /// Ground held along the front, keyed by district id. +1 = side A holds
    /// it, -1 = side B. Delta-encoded against ownership: an absent district
    /// sits at its owner's side (+1 if owned by the side-A principal, -1 if
    /// side-B), and `front.rs` prunes the map back to deviations — plus the
    /// base-valued districts adjacent to a deviation, kept so the map's front
    /// line never vanishes along a fully-captured edge — after every month.
    /// Empty for pre-front saves; `front::reseed_fronts` reconstructs it on
    /// load. Skipped when empty so a frontless conflict serializes exactly as
    /// it did before the operational map existed.
    #[serde(default, skip_serializing_if = "std::collections::BTreeMap::is_empty")]
    pub front: std::collections::BTreeMap<String, f32>,
    /// Encircled held groups as of the last resolved month, ids sorted within
    /// each group, groups sorted lexicographically. Derived monthly, but
    /// serialized so a mid-month save hashes identically to its reload.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub pockets: Vec<Vec<String>>,
    /// What a resource war is for: the district and the line (resources.rs;
    /// cut two writes it, nothing before). Skipped when absent so a conflict
    /// serializes exactly as it did before the aim existed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub aim: Option<crate::resources::Aim>,
}

impl Conflict {
    pub fn involves(&self, id: NationId) -> bool {
        self.side_a.contains(&id) || self.side_b.contains(&id)
    }
    /// true = side A.
    pub fn side_of(&self, id: NationId) -> Option<bool> {
        if self.side_a.contains(&id) {
            Some(true)
        } else if self.side_b.contains(&id) {
            Some(false)
        } else {
            None
        }
    }
    pub fn attacker(&self) -> NationId {
        *self.side_a.first().unwrap_or(&self.origin_attacker)
    }
    pub fn defender(&self) -> NationId {
        *self.side_b.first().unwrap_or(&self.origin_attacker)
    }
    pub fn participants(&self) -> Vec<NationId> {
        let mut v = self.side_a.clone();
        v.extend(self.side_b.iter().copied());
        v
    }
    pub fn posture_of(&self, id: NationId) -> Option<&Belligerent> {
        self.posture.iter().find(|b| b.nation == id)
    }
    pub fn posture_mut(&mut self, id: NationId) -> Option<&mut Belligerent> {
        self.posture.iter_mut().find(|b| b.nation == id)
    }
    /// The highest rung anyone on a side is standing on.
    pub fn top_rung(&self, side_a: bool) -> u8 {
        let side = if side_a { &self.side_a } else { &self.side_b };
        self.posture
            .iter()
            .filter(|b| side.contains(&b.nation))
            .map(|b| b.rung)
            .max()
            .unwrap_or(1)
    }
    /// Anyone shooting. Rung 6 — standoff strike — is where a conflict becomes
    /// a war for everything downstream that reads `at_war`.
    pub fn shooting(&self) -> bool {
        self.posture.iter().any(|b| b.rung >= SHOOTING_RUNG)
    }
    pub fn class(&self) -> ConflictClass {
        if self.frozen_since.is_some() {
            return ConflictClass::Frozen;
        }
        let (a, b) = (self.top_rung(true), self.top_rung(false));
        // Both standing and fighting is conventional. One side standing in the
        // open while the other does not is the irregular case, and it is the
        // state the whole model exists to express.
        if a >= 6 && b >= 6 {
            ConflictClass::Conventional
        } else if a.max(b) >= 6 {
            ConflictClass::Irregular
        } else {
            ConflictClass::Frozen
        }
    }
}

/// The rung at which a conflict becomes a shooting war, and therefore the rung
/// at which `at_war` starts returning true — which is what the economy's war
/// drag, the oil terminals and the sanctions-relief clock all read.
pub const SHOOTING_RUNG: u8 = 6;

/// The rung at which crossing a border stops being an incident and becomes an
/// invasion: a full conventional campaign, the thing a coalition forms against
/// and the thing a guarantor is called for. Below it a quarrel is a quarrel,
/// however unpleasant, and the world does not mobilise about it.
pub const INVASION_RUNG: u8 = 8;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GameRules {
    pub seed: u64,
    /// Multiplier on AI war appetite
    pub ai_aggression: f64,
    /// Crisis intensity multiplier (bubbles, collapses)
    pub crisis_intensity: f64,
    /// The resource gates (resources.rs): whether a procurement line that
    /// asks for a commodity it cannot get is delayed. The census control
    /// arm. True by default and skipped when true, so a save written before
    /// the switch existed reads as gated and a default world serializes
    /// exactly as it did before the switch was declared.
    #[serde(default = "rules_true", skip_serializing_if = "is_true")]
    pub resource_gates: bool,
    /// The resource market (resources.rs, cut two): the sanction ration on
    /// the open market, the AI buy pass, the refusal memory and the
    /// last-resort war. Fork F1(b): OFF by default, so every test and the
    /// headless CLI run the world the goldens pin, bit for bit, and the
    /// browser's new-game path turns it on. Skipped when false, so a default
    /// world serializes exactly as it did before the switch was declared.
    #[serde(default, skip_serializing_if = "is_false")]
    pub resource_market: bool,
    /// The abstract shipment ledger and hard route closures (resources.rs).
    /// Operational only while `resource_market` is also on. OFF by default and
    /// skipped when false, preserving old saves and the calibrated baseline.
    #[serde(default, skip_serializing_if = "is_false")]
    pub logistics_routes: bool,
}
fn rules_true() -> bool {
    true
}
fn is_true(v: &bool) -> bool {
    *v
}
fn is_false(v: &bool) -> bool {
    !*v
}
impl Default for GameRules {
    fn default() -> Self {
        GameRules {
            seed: 1990,
            ai_aggression: 1.0,
            crisis_intensity: 1.0,
            resource_gates: true,
            resource_market: false,
            logistics_routes: false,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WorldState {
    pub rules: GameRules,
    pub rng: Rng,
    pub year: i32,
    pub month: u32, // 1..=12
    /// Day of month. Older monthly saves omitted this field and therefore load
    /// on the first day of their saved month. The first is also omitted when
    /// saving so a world that still advances monthly keeps its historical hash.
    #[serde(default = "first_day", skip_serializing_if = "is_first_day")]
    pub day: u32, // 1..=days_in_month(year, month)
    pub nations: Vec<Nation>,
    /// relations[(a,b)] symmetric, -100..100 — stored as sorted-pair list
    pub relations: Relations,
    /// sanctions: (imposer, target)
    pub sanctions: Vec<(NationId, NationId)>,
    /// Every live conflict, in creation order. Iterated in vector order, which
    /// is why resolution is deterministic without sorting anything.
    #[serde(default)]
    pub conflicts: Vec<Conflict>,
    /// Dead legacy field. Nothing writes it; `load()` drains any that a pre-
    /// ladder save carries into `conflicts` and leaves this empty forever after.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub wars: Vec<War>,
    /// The operating areas, and who is home to them. Mutable, because
    /// federations come apart and their successors have to inherit the ground.
    #[serde(default = "crate::theatre::default_theatres")]
    pub theatres: Vec<crate::theatre::Theatre>,
    /// Standing consents to basing and overflight.
    #[serde(default)]
    pub access: Vec<crate::theatre::Access>,
    /// Pacts, patronage, trade and the intelligence services. Defaulted so that
    /// a save written before statecraft existed still loads.
    #[serde(default)]
    pub statecraft: Statecraft,
    /// Parties, elections, coalitions, and the pillars an unelected regime has
    /// to keep paying. Defaulted for the same reason `statecraft` is: a save
    /// written before governments existed still loads, and `government::ensure`
    /// seats each one from the transcribed 1990 party table on the next tick.
    #[serde(default)]
    pub governments: crate::government::Governments,
    /// Brent-ish oil price, USD/barrel
    pub oil_price: f64,
    /// Event log for the current month (drained by UI)
    pub headlines: Vec<String>,
    /// One-shot flags
    pub flags: Vec<String>,
    pub player: Option<NationId>,
    /// Has the player ever set their own policy rate?
    ///
    /// An absent governor is not a governor choosing 8%. The central bank in
    /// `politics.rs` skips the player's nation so that the AI can never
    /// overwrite a rate the player chose — but before this latch it skipped a
    /// player who had chosen nothing just as thoroughly, and the seat simply
    /// held whatever 1990 handed it while the world moved on. Taking the United
    /// States and advancing the clock left 8% standing into a deflation.
    ///
    /// So the bank runs on the player's behalf until the player takes the
    /// wheel, and never again after. Latching on the command rather than on
    /// "the rate differs from its 1990 value" is what keeps the two cases
    /// apart: a player who deliberately re-sets 8% has still governed, and is
    /// owed the same silence as one who sets 3%.
    ///
    /// Defaulted for the reason `statecraft` and `governments` are: a save
    /// written before the latch existed still loads. Such a save reads as "has
    /// not governed", which restores the AI bank to a seat that was drifting
    /// without one — the right reading for the far more common case of a save
    /// made by a player who never touched the rate.
    #[serde(default)]
    pub player_set_rate: bool,

    /// Who holds each admin-1 district. Political geography only — BIBLE
    /// section 5's sanction and its whole extent: districts own identity and
    /// change hands at a settlement; they are not fought over individually
    /// and never will be. Keys are the stable district ids from
    /// `data/districts.json`; a district appears here only while some nation
    /// holds it (Namibia's and East Timor's are unowned until a birth site
    /// exists).
    ///
    /// BTreeMap so serialization order is key order — one more field the
    /// golden hash covers without a sort. Defaulted so a save written before
    /// districts existed still loads; `load()` reseeds an empty map from the
    /// 1990 table plus alive successors, which is right for every save except
    /// one taken after a pre-upgrade annexation or concession, which reads as
    /// "borders at default" — the honest reconstruction available.
    #[serde(default)]
    pub districts: std::collections::BTreeMap<String, NationId>,

    /// Residents of each admin-1 district, in millions. Population remains
    /// attached to the province when ownership changes.
    #[serde(default)]
    pub district_population: std::collections::BTreeMap<String, f64>,
    /// Cumulative demographic growth by current owner. Province values use a
    /// rebased basis so monthly updates scale with the roster, not the map.
    #[serde(default)]
    pub(crate) district_population_scale: Vec<f64>,

    /// The resource system's persisted state: the stockpile, and (cut two)
    /// contracts, offers, refusals, grievances. Skipped while empty, which an
    /// untouched world always is: the pile is written only when a gated line
    /// asks for what it cannot get, and folds away when it is full again.
    #[serde(default, skip_serializing_if = "crate::resources::Resources::is_empty")]
    pub resources: crate::resources::Resources,

    /// Where each roster id sits in `nations`, or `u16::MAX` for a state that
    /// has not been born. Derived and never serialized: a save that carried it
    /// could disagree with the vector it indexes, and it must not touch the
    /// timeline hash.
    ///
    /// `nations` is only ever appended to — never reordered, never drained —
    /// so its length is a sufficient stamp: if it has not changed since the
    /// index was built, the index is current. Every lookup checks that and
    /// falls back to the linear scan when it does not hold, which means a stale
    /// index can cost time and can never cost correctness. The index resolves
    /// the FIRST occurrence of an id, exactly as `find` did.
    #[serde(skip)]
    pub(crate) by_id: Vec<u16>,
    #[serde(skip)]
    pub(crate) by_id_len: usize,
    /// The derived HAVE ledger — who produces what this month, read through
    /// the current ownership map. Never serialized, never hashed; rebuilt by
    /// `resources::tick` when `districts_epoch` or the living roster moves.
    #[serde(skip)]
    pub(crate) resource_have: crate::resources::Have,
    /// Bumped by every function in `districts.rs` that moves ground. Derived,
    /// never serialized: `load()` leaves it at 0 with the ledger unbuilt and
    /// the first tick rebuilds.
    #[serde(skip)]
    pub(crate) districts_epoch: u64,
    /// Exact population multipliers written by economy and consumed by tech
    /// later in the same monthly tick.
    #[serde(skip)]
    pub(crate) district_population_growth: Vec<(NationId, f64)>,
}

fn first_day() -> u32 {
    1
}

fn is_first_day(day: &u32) -> bool {
    *day == 1
}

/// Gregorian month length for the playable calendar.
pub fn days_in_month(year: i32, month: u32) -> u32 {
    match month {
        4 | 6 | 9 | 11 => 30,
        2 if year % 400 == 0 || (year % 4 == 0 && year % 100 != 0) => 29,
        2 => 28,
        _ => 31,
    }
}

/// Nowhere in `nations`.
const ABSENT: u16 = u16::MAX;

/// The key `prefix`_`a`_`b`, assembled in a stack buffer.
///
/// The obvious form of this — four `strip_prefix` calls down the flag list —
/// is the same answer and measurably slower, which is worth recording because
/// it is not obvious: comparing two `String`s checks the lengths first and
/// rejects almost every flag in a byte, while walking the pieces has to
/// memcmp the shared prefix of every `pressed_*` before it can rule one out.
/// Building the key once and comparing whole restores the length check and
/// keeps the allocation gone. Measured on the century run at 137 nations: the
/// piecewise form put 1.3 seconds back that the rest of this branch had taken
/// out, and it took a like-for-like profile to see it.
fn pair_flag_key<'a>(buf: &'a mut [u8; 64], prefix: &str, a: &str, b: &str) -> Option<&'a [u8]> {
    let n = prefix.len() + 1 + a.len() + 1 + b.len();
    if n > buf.len() {
        return None;
    }
    let mut i = 0;
    for piece in [prefix.as_bytes(), b"_", a.as_bytes(), b"_", b.as_bytes()] {
        buf[i..i + piece.len()].copy_from_slice(piece);
        i += piece.len();
    }
    Some(&buf[..n])
}

impl WorldState {
    /// Rebuild the id -> position index. One pass, idempotent, and safe to call
    /// at any time: nothing downstream can tell whether it has been called.
    pub fn reindex(&mut self) {
        self.by_id.clear();
        self.by_id.resize(nation_count(), ABSENT);
        for (i, n) in self.nations.iter().enumerate() {
            if let Some(slot) = self.by_id.get_mut(n.id.index()) {
                if *slot == ABSENT {
                    *slot = i as u16;
                }
            }
        }
        self.by_id_len = self.nations.len();
    }
    #[inline]
    fn indexed(&self) -> bool {
        self.by_id_len == self.nations.len() && self.by_id.len() == nation_count()
    }
    /// Where `id` lives, when the index can be trusted.
    #[inline]
    fn position(&self, id: NationId) -> Option<Option<usize>> {
        if !self.indexed() {
            return None;
        }
        match self.by_id.get(id.index()) {
            Some(&ABSENT) | None => Some(None),
            Some(&p) => Some(Some(p as usize)),
        }
    }
    pub fn nation(&self, id: NationId) -> &Nation {
        if let Some(p) = self.position(id) {
            return &self.nations[p.expect("nation")];
        }
        self.nations.iter().find(|n| n.id == id).expect("nation")
    }
    pub fn nation_mut(&mut self, id: NationId) -> &mut Nation {
        if let Some(p) = self.position(id) {
            return &mut self.nations[p.expect("nation")];
        }
        self.nations.iter_mut().find(|n| n.id == id).expect("nation")
    }
    /// For nations that may not exist in this world at all — successor states
    /// before their federation falls, or an older save with a smaller roster.
    pub fn nation_opt(&self, id: NationId) -> Option<&Nation> {
        if let Some(p) = self.position(id) {
            return p.map(|i| &self.nations[i]);
        }
        self.nations.iter().find(|n| n.id == id)
    }
    pub fn has_flag(&self, f: &str) -> bool {
        self.flags.iter().any(|x| x == f)
    }
    /// `has_flag` for the flags that name an ordered pair — `pressed_A_B`,
    /// `burned_A_B` — which is where nearly all of the asking happens:
    /// `dyads::war_appetite` asks twice for every candidate dyad every month,
    /// two and a half million times over a century at the current roster, and
    /// each question built a `String` to throw away.
    ///
    /// Exactly `has_flag(&format!("{}_{:?}_{:?}", prefix, a, b))` — `{:?}` on a
    /// `NationId` writes its code, so the key is these four pieces joined, and
    /// matching them piecewise costs no allocation at all.
    /// `pair_flag_is_the_formatted_flag` holds the two together.
    pub fn has_pair_flag(&self, prefix: &str, a: NationId, b: NationId) -> bool {
        let mut buf = [0u8; 64];
        match pair_flag_key(&mut buf, prefix, a.code(), b.code()) {
            Some(key) => self.flags.iter().any(|f| f.as_bytes() == key),
            // Longer than any flag this game writes; fall back rather than
            // truncate, because a truncated key would answer the wrong question.
            None => self.has_flag(&format!("{}_{:?}_{:?}", prefix, a, b)),
        }
    }
    pub fn set_flag(&mut self, f: &str) {
        if !self.has_flag(f) {
            self.flags.push(f.to_string());
        }
    }
    pub fn relation(&self, a: NationId, b: NationId) -> f64 {
        self.relations.get(a, b)
    }
    pub fn set_relation(&mut self, a: NationId, b: NationId, v: f64) {
        self.relations.set(a, b, v.clamp(-100.0, 100.0));
    }
    pub fn shift_relation(&mut self, a: NationId, b: NationId, d: f64) {
        let cur = self.relation(a, b);
        self.set_relation(a, b, cur + d);
    }
    pub fn allied(&self, a: NationId, b: NationId) -> bool {
        let (x, y) = if a <= b { (a, b) } else { (b, a) };
        self.statecraft.pacts.iter().any(|p| p.a == x && p.b == y)
    }
    /// Everyone who has guaranteed `id`, in NationId order — the order matters
    /// because each of them will be asked, with a die roll, to honour it.
    pub fn pact_partners(&self, id: NationId) -> Vec<NationId> {
        let mut v: Vec<NationId> = self
            .statecraft
            .pacts
            .iter()
            .filter_map(|p| {
                if p.a == id {
                    Some(p.b)
                } else if p.b == id {
                    Some(p.a)
                } else {
                    None
                }
            })
            .collect();
        v.sort();
        v
    }
    pub fn aid_flow(&self, patron: NationId, client: NationId, kind: AidKind) -> Option<&AidFlow> {
        self.statecraft
            .aid
            .iter()
            .find(|f| f.patron == patron && f.client == client && f.kind == kind)
    }
    /// Share of its own output a patron has already promised away.
    pub fn aid_share_committed(&self, patron: NationId) -> f64 {
        self.statecraft
            .aid
            .iter()
            .filter(|f| f.patron == patron)
            .map(|f| f.share_gdp)
            .sum()
    }
    /// Everything one patron sends one client, across cash and weapons alike.
    pub fn aid_share_to(&self, patron: NationId, client: NationId) -> f64 {
        self.statecraft
            .aid
            .iter()
            .filter(|f| f.patron == patron && f.client == client)
            .map(|f| f.share_gdp)
            .sum()
    }
    /// Everyone paying `client`, deduplicated and ordered.
    pub fn patrons_of(&self, client: NationId) -> Vec<NationId> {
        let mut v: Vec<NationId> = self
            .statecraft
            .aid
            .iter()
            .filter(|f| f.client == client)
            .map(|f| f.patron)
            .collect();
        v.sort();
        v.dedup();
        v
    }
    pub fn trade_depth(&self, a: NationId, b: NationId) -> f64 {
        let (x, y) = if a <= b { (a, b) } else { (b, a) };
        self.statecraft
            .trade
            .iter()
            .find(|t| t.a == x && t.b == y)
            .map(|t| t.depth)
            .unwrap_or(0.0)
    }
    /// How much of `id`'s trade runs through `partner`, 0..1. This is leverage:
    /// the side with the smaller number can afford to walk away.
    ///
    /// Two forms, and the larger wins: the pact form below, and a supply
    /// contract's (`resources::contract_dependency` — the depth-weighted
    /// share of a line's sourcing that arrives under contract from
    /// `partner`). commitment.rs and theatre.rs read this one name, so a
    /// contract is leverage for free; with no contract the two forms are
    /// identical and `abrogate_trade`'s arithmetic reads exactly what it did.
    pub fn trade_dependency(&self, id: NationId, partner: NationId) -> f64 {
        let pact = {
            let depth = self.trade_depth(id, partner);
            if depth <= 0.0 {
                0.0
            } else {
                match (self.nation_opt(id), self.nation_opt(partner)) {
                    (Some(m), Some(t)) => depth * (t.gdp / (m.gdp + t.gdp).max(1.0)),
                    _ => 0.0,
                }
            }
        };
        pact.max(crate::resources::contract_dependency(self, id, partner))
    }
    pub fn reputation(&self, id: NationId) -> f64 {
        self.statecraft
            .reputation
            .iter()
            .find(|(x, _)| *x == id)
            .map(|(_, v)| *v)
            .unwrap_or(BASE_REPUTATION)
    }
    pub fn shift_reputation(&mut self, id: NationId, d: f64) {
        let v = (self.reputation(id) + d).clamp(0.0, 100.0);
        if let Some(e) = self.statecraft.reputation.iter_mut().find(|(x, _)| *x == id) {
            e.1 = v;
        } else {
            self.statecraft.reputation.push((id, v));
        }
    }
    pub fn covert_heat(&self, sponsor: NationId, target: NationId) -> f64 {
        self.statecraft
            .covert_heat
            .iter()
            .find(|(s, t, _)| *s == sponsor && *t == target)
            .map(|(_, _, v)| *v)
            .unwrap_or(0.0)
    }
    pub fn add_covert_heat(&mut self, sponsor: NationId, target: NationId, d: f64) {
        if let Some(e) = self
            .statecraft
            .covert_heat
            .iter_mut()
            .find(|(s, t, _)| *s == sponsor && *t == target)
        {
            e.2 = (e.2 + d).clamp(0.0, 1.0);
        } else {
            self.statecraft.covert_heat.push((sponsor, target, d.clamp(0.0, 1.0)));
        }
    }
    pub fn sanctioned_by_count(&self, target: NationId) -> usize {
        self.sanctions.iter().filter(|(_, t)| *t == target).count()
    }
    pub fn is_sanctioning(&self, imposer: NationId, target: NationId) -> bool {
        self.sanctions.iter().any(|(i, t)| *i == imposer && *t == target)
    }
    /// The sanctioning coalition's share of world output, 0..1 — how much of the
    /// world economy has shut its doors to this nation.
    ///
    /// This is the honest measure of a sanctions regime and `sanctioned_by_count`
    /// is not. A count treats every signature as worth the same, so Luxembourg
    /// joining an embargo costs the target as much as the United States joining
    /// it, and the total price of being sanctioned rises without limit as the
    /// roster grows. A share cannot do either: it is bounded by one, and adding
    /// a hundred more nations to the world leaves a G5 regime weighing what a G5
    /// regime weighs. `oil_blockade` has always read this quantity; the growth
    /// drag in `economy.rs` now reads it too, and the three remaining count-based
    /// sanction channels are listed in the comment there.
    pub fn sanction_weight(&self, target: NationId) -> f64 {
        let world_gdp: f64 = self.nations.iter().filter(|n| n.alive).map(|n| n.gdp).sum();
        if world_gdp <= 0.0 {
            return 0.0;
        }
        let blocking: f64 = self
            .sanctions
            .iter()
            .filter(|(_, t)| *t == target)
            .filter_map(|(i, _)| self.nations.iter().find(|n| n.id == *i && n.alive))
            .map(|n| n.gdp)
            .sum();
        blocking / world_gdp
    }
    /// Share of a producer's exports shut out of the market by embargo, 0..1.
    /// Weighted by the sanctioners' share of world GDP — an embargo bites in
    /// proportion to the demand that closes its doors. Smuggling keeps a floor.
    pub fn oil_blockade(&self, target: NationId) -> f64 {
        (self.sanction_weight(target) * 1.15).min(0.85)
    }
    /// Share of a producer's barrels that still reach the market. Embargo shuts
    /// buyers out; war shuts the terminals themselves. Both the world price and
    /// the producer's own revenue read from this one number.
    pub fn oil_export_share(&self, id: NationId) -> f64 {
        let war_share = if self.at_war(id) { 0.25 } else { 1.0 };
        (1.0 - self.oil_blockade(id)) * war_share
    }
    /// Shooting, not merely quarrelling. Read by the economy's war drag, the oil
    /// terminals, trade collapse, the sanctions-relief clock and the guarantor
    /// rolls, all of which mean "a war is on" — so a rung-2 sanctions conflict
    /// must not shut down a country's oil terminals, and does not.
    pub fn at_war(&self, id: NationId) -> bool {
        self.conflicts.iter().any(|c| c.involves(id) && c.shooting())
    }
    /// Anywhere on the ladder, including the rungs where nobody has fired.
    pub fn in_conflict(&self, id: NationId) -> bool {
        self.conflicts.iter().any(|c| c.involves(id))
    }
    pub fn conflict(&self, id: u32) -> Option<&Conflict> {
        self.conflicts.iter().find(|c| c.id == id)
    }
    pub fn conflict_mut(&mut self, id: u32) -> Option<&mut Conflict> {
        self.conflicts.iter_mut().find(|c| c.id == id)
    }
    /// Where two states are already fighting each other, if they are.
    pub fn conflict_between(&self, a: NationId, b: NationId) -> Option<&Conflict> {
        self.conflicts.iter().find(|c| match (c.side_of(a), c.side_of(b)) {
            (Some(x), Some(y)) => x != y,
            _ => false,
        })
    }
    pub fn next_conflict_id(&self) -> u32 {
        self.conflicts.iter().map(|c| c.id).max().map_or(1, |m| m + 1)
    }
    pub fn headline(&mut self, s: String) {
        self.headlines.push(s);
    }
    pub fn date_str(&self) -> String {
        const M: [&str; 12] = ["Jan","Feb","Mar","Apr","May","Jun","Jul","Aug","Sep","Oct","Nov","Dec"];
        format!("{} {} {}", self.day, M[(self.month - 1) as usize], self.year)
    }
}

impl War {
    pub fn involves(&self, id: NationId) -> bool {
        self.attacker == id
            || self.defender == id
            || self.attacker_allies.contains(&id)
            || self.defender_allies.contains(&id)
    }
    pub fn side_of(&self, id: NationId) -> Option<bool> {
        // true = attacker side
        if self.attacker == id || self.attacker_allies.contains(&id) {
            Some(true)
        } else if self.defender == id || self.defender_allies.contains(&id) {
            Some(false)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod perf_refactor_guards {
    //! Three lookups in this file were rewritten for speed alone, and the whole
    //! case for that rewrite is that the answers did not move. Each of these
    //! holds the fast form against the slow one it replaced, so a future edit
    //! to either cannot quietly separate them. The two pinned timeline hashes
    //! prove the same thing end to end; these say *where* if one ever breaks.

    use super::*;
    use crate::init::world_1990;

    /// The triangular walk, against the index inversion it replaced.
    #[test]
    fn pairs_mut_walks_the_triangle_in_order() {
        let mut r = Relations::default();
        r.ensure();
        // What the old code computed, for every slot, from scratch.
        let expected: Vec<(usize, usize)> = (0..r.tri.len())
            .map(|i| {
                let mut hi = 0usize;
                while (hi + 1) * (hi + 2) / 2 <= i {
                    hi += 1;
                }
                (hi, i - hi * (hi + 1) / 2)
            })
            .collect();
        let got: Vec<(usize, usize)> =
            r.pairs_mut().map(|(a, b, _)| (a.index(), b.index())).collect();
        assert_eq!(got.len(), expected.len(), "the sweep changed length");
        for (i, (g, e)) in got.iter().zip(expected.iter()).enumerate() {
            assert_eq!(g, e, "slot {} inverted to {:?}, was {:?}", i, g, e);
        }
    }

    /// Every value reached, and reached exactly once, by the pair it is filed
    /// under. This is the property `politics`'s grievance decay depends on: it
    /// mutates through the iterator and must not touch a slot twice.
    #[test]
    fn every_relation_is_reached_by_its_own_pair() {
        let mut w = world_1990(GameRules::default());
        let sample: Vec<(NationId, NationId, f64)> = vec![
            (NationId::USA, NationId::USSR, -37.0),
            (NationId::Iraq, NationId::Kuwait, -61.5),
            (NationId::India, NationId::Pakistan, -55.0),
        ];
        for (a, b, v) in &sample {
            w.set_relation(*a, *b, *v);
        }
        let mut seen = 0usize;
        for (a, b, v) in w.relations.pairs_mut() {
            if let Some((_, _, want)) =
                sample.iter().find(|(x, y, _)| (*x == a && *y == b) || (*x == b && *y == a))
            {
                assert_eq!(*v, *want, "{:?}/{:?} came back as {}", a, b, v);
                seen += 1;
            }
        }
        assert_eq!(seen, sample.len(), "a pair was skipped or visited twice");
    }

    /// The position index, against the linear scan it replaced — including for
    /// the successor states that are on the roster and not yet in the world.
    #[test]
    fn the_position_index_answers_what_the_scan_answered() {
        let mut w = world_1990(GameRules::default());
        for _ in 0..240 {
            crate::tick_month(&mut w, &[]);
        }
        assert!(w.indexed(), "a month ended with the index stale");
        for id in all_nations().iter().copied() {
            let scanned = w.nations.iter().position(|n| n.id == id);
            let indexed = w.nation_opt(id).map(|n| n.id);
            assert_eq!(
                indexed,
                scanned.map(|i| w.nations[i].id),
                "{:?}: index and scan disagree",
                id
            );
        }
        // ...and with the index deliberately stale, the fallback still answers.
        let saved = std::mem::take(&mut w.by_id);
        for id in all_nations().iter().copied() {
            assert!(!w.indexed());
            let scanned = w.nations.iter().find(|n| n.id == id).map(|n| n.id);
            assert_eq!(w.nation_opt(id).map(|n| n.id), scanned, "{:?}: stale index lied", id);
        }
        w.by_id = saved;
    }

    /// The allocation-free pair-flag query, against `has_flag(&format!(..))`.
    #[test]
    fn pair_flag_is_the_formatted_flag() {
        let mut w = world_1990(GameRules::default());
        let pairs = [
            (NationId::Iraq, NationId::Kuwait),
            (NationId::Kuwait, NationId::Iraq),
            (NationId::USA, NationId::Iraq),
            (NationId::India, NationId::Pakistan),
        ];
        for (a, b) in pairs {
            w.set_flag(&format!("pressed_{:?}_{:?}", a, b));
        }
        w.set_flag("burned_IRQ_KWT_and_then_some");
        w.set_flag("pressed");
        for prefix in ["pressed", "burned"] {
            for a in all_nations().iter().copied().take(40) {
                for b in all_nations().iter().copied().take(40) {
                    let formatted = w.has_flag(&format!("{}_{:?}_{:?}", prefix, a, b));
                    assert_eq!(
                        w.has_pair_flag(prefix, a, b),
                        formatted,
                        "{}_{:?}_{:?}",
                        prefix,
                        a,
                        b
                    );
                }
            }
        }
    }
}
