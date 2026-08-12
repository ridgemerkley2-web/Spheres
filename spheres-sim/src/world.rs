use serde::{Deserialize, Serialize};

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

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum NationId {
    USA, USSR, Russia, Ukraine, China, Japan, Germany, UK, France, Italy,
    India, Pakistan, Iraq, Kuwait, SaudiArabia, Iran, SouthKorea, Poland,
    Brazil, Indonesia, Egypt, Israel, Turkey, Nigeria, Vietnam,
    Yugoslavia, Serbia, Croatia, Slovenia, Bosnia,
    Argentina, Mexico, Chile, Colombia, Venezuela, Peru, Cuba, Bolivia,
    Ecuador, Uruguay,
}
pub const ALL_START_NATIONS: [NationId; 34] = [
    NationId::USA, NationId::USSR, NationId::China, NationId::Japan,
    NationId::Germany, NationId::UK, NationId::France, NationId::Italy,
    NationId::India, NationId::Pakistan, NationId::Iraq, NationId::Kuwait,
    NationId::SaudiArabia, NationId::Iran, NationId::SouthKorea, NationId::Poland,
    NationId::Brazil, NationId::Indonesia, NationId::Egypt, NationId::Israel,
    NationId::Turkey, NationId::Nigeria, NationId::Vietnam,
    NationId::Yugoslavia,
    NationId::Argentina, NationId::Mexico, NationId::Chile, NationId::Colombia,
    NationId::Venezuela, NationId::Peru, NationId::Cuba, NationId::Bolivia,
    NationId::Ecuador, NationId::Uruguay,
];
/// States that only exist if a federation comes apart.
pub const SUCCESSOR_NATIONS: [NationId; 6] = [
    NationId::Russia, NationId::Ukraine, NationId::Serbia, NationId::Croatia,
    NationId::Slovenia, NationId::Bosnia,
];

impl NationId {
    pub fn name(&self) -> &'static str {
        match self {
            NationId::USA => "United States",
            NationId::USSR => "Soviet Union",
            NationId::Russia => "Russia",
            NationId::Ukraine => "Ukraine",
            NationId::China => "China",
            NationId::Japan => "Japan",
            NationId::Germany => "Germany",
            NationId::UK => "United Kingdom",
            NationId::France => "France",
            NationId::Italy => "Italy",
            NationId::India => "India",
            NationId::Pakistan => "Pakistan",
            NationId::Iraq => "Iraq",
            NationId::Kuwait => "Kuwait",
            NationId::SaudiArabia => "Saudi Arabia",
            NationId::Iran => "Iran",
            NationId::SouthKorea => "South Korea",
            NationId::Poland => "Poland",
            NationId::Brazil => "Brazil",
            NationId::Indonesia => "Indonesia",
            NationId::Egypt => "Egypt",
            NationId::Israel => "Israel",
            NationId::Turkey => "Turkey",
            NationId::Nigeria => "Nigeria",
            NationId::Vietnam => "Vietnam",
            NationId::Yugoslavia => "Yugoslavia",
            NationId::Serbia => "Serbia",
            NationId::Croatia => "Croatia",
            NationId::Slovenia => "Slovenia",
            NationId::Bosnia => "Bosnia",
            NationId::Argentina => "Argentina",
            NationId::Mexico => "Mexico",
            NationId::Chile => "Chile",
            NationId::Colombia => "Colombia",
            NationId::Venezuela => "Venezuela",
            NationId::Peru => "Peru",
            NationId::Cuba => "Cuba",
            NationId::Bolivia => "Bolivia",
            NationId::Ecuador => "Ecuador",
            NationId::Uruguay => "Uruguay",
        }
    }
    pub fn parse(s: &str) -> Option<NationId> {
        let t = s.trim().to_lowercase();
        Some(match t.as_str() {
            "usa" | "us" | "united states" | "america" => NationId::USA,
            "ussr" | "soviet union" | "soviets" => NationId::USSR,
            "russia" => NationId::Russia,
            "ukraine" | "ukr" => NationId::Ukraine,
            "china" | "prc" => NationId::China,
            "japan" => NationId::Japan,
            "germany" => NationId::Germany,
            "uk" | "britain" | "united kingdom" => NationId::UK,
            "france" => NationId::France,
            "italy" => NationId::Italy,
            "india" => NationId::India,
            "pakistan" => NationId::Pakistan,
            "iraq" => NationId::Iraq,
            "kuwait" => NationId::Kuwait,
            "saudi arabia" | "saudi" | "ksa" => NationId::SaudiArabia,
            "iran" => NationId::Iran,
            "south korea" | "korea" | "rok" => NationId::SouthKorea,
            "poland" => NationId::Poland,
            "brazil" | "bra" => NationId::Brazil,
            "indonesia" | "idn" => NationId::Indonesia,
            "egypt" | "egy" | "uar" => NationId::Egypt,
            "israel" | "isr" => NationId::Israel,
            "turkey" | "turkiye" | "tur" => NationId::Turkey,
            "nigeria" | "nga" => NationId::Nigeria,
            "vietnam" | "viet nam" | "vnm" => NationId::Vietnam,
            "yugoslavia" | "sfry" | "yugo" => NationId::Yugoslavia,
            "serbia" | "serbia and montenegro" | "fry" => NationId::Serbia,
            "croatia" => NationId::Croatia,
            "slovenia" => NationId::Slovenia,
            "bosnia" | "bosnia and herzegovina" | "bih" => NationId::Bosnia,
            "argentina" | "arg" => NationId::Argentina,
            "mexico" | "mex" => NationId::Mexico,
            "chile" | "chl" => NationId::Chile,
            "colombia" | "col" => NationId::Colombia,
            "venezuela" | "ven" => NationId::Venezuela,
            "peru" | "per" => NationId::Peru,
            "cuba" | "cub" => NationId::Cuba,
            "bolivia" | "bol" => NationId::Bolivia,
            "ecuador" | "ecu" => NationId::Ecuador,
            "uruguay" | "ury" | "uru" => NationId::Uruguay,
            _ => return None,
        })
    }
}

/// Every nation id, in declaration order. The index into this array is the
/// index into the relations matrix, so the two must never drift apart.
pub const ALL_NATION_IDS: [NationId; NATION_COUNT] = [
    NationId::USA, NationId::USSR, NationId::Russia, NationId::Ukraine,
    NationId::China, NationId::Japan, NationId::Germany, NationId::UK,
    NationId::France, NationId::Italy, NationId::India, NationId::Pakistan,
    NationId::Iraq, NationId::Kuwait, NationId::SaudiArabia, NationId::Iran,
    NationId::SouthKorea, NationId::Poland, NationId::Brazil, NationId::Indonesia,
    NationId::Egypt, NationId::Israel, NationId::Turkey, NationId::Nigeria,
    NationId::Vietnam, NationId::Yugoslavia, NationId::Serbia, NationId::Croatia,
    NationId::Slovenia, NationId::Bosnia,
    NationId::Argentina, NationId::Mexico, NationId::Chile, NationId::Colombia,
    NationId::Venezuela, NationId::Peru, NationId::Cuba, NationId::Bolivia,
    NationId::Ecuador, NationId::Uruguay,
];
pub const NATION_COUNT: usize = 40;

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
            let (x, y) = (a as usize, b as usize);
            if x >= y { (x, y) } else { (y, x) }
        };
        hi * (hi + 1) / 2 + lo
    }
    fn ensure(&mut self) {
        if self.tri.len() != NATION_COUNT * (NATION_COUNT + 1) / 2 {
            self.tri.resize(NATION_COUNT * (NATION_COUNT + 1) / 2, 0.0);
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
    pub fn pairs_mut(&mut self) -> impl Iterator<Item = (NationId, NationId, &mut f64)> {
        self.ensure();
        self.tri.iter_mut().enumerate().filter_map(|(i, v)| {
            // invert the triangular index
            let mut hi = 0usize;
            while (hi + 1) * (hi + 2) / 2 <= i {
                hi += 1;
            }
            let lo = i - hi * (hi + 1) / 2;
            match (ALL_NATION_IDS.get(hi), ALL_NATION_IDS.get(lo)) {
                (Some(a), Some(b)) => Some((*a, *b, v)),
                _ => None,
            }
        })
    }
}

impl Serialize for Relations {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut out: Vec<(NationId, NationId, f64)> = vec![];
        for (i, v) in self.tri.iter().enumerate() {
            if *v == 0.0 {
                continue;
            }
            let mut hi = 0usize;
            while (hi + 1) * (hi + 2) / 2 <= i {
                hi += 1;
            }
            let lo = i - hi * (hi + 1) / 2;
            if let (Some(a), Some(b)) = (ALL_NATION_IDS.get(hi), ALL_NATION_IDS.get(lo)) {
                out.push((*b, *a, *v));
            }
        }
        out.serialize(s)
    }
}

impl<'de> Deserialize<'de> for Relations {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let triples = Vec::<(NationId, NationId, f64)>::deserialize(d)?;
        let mut r = Relations::default();
        r.ensure();
        for (a, b, v) in triples {
            r.set(a, b, v);
        }
        Ok(r)
    }
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq)]
pub enum EconomySystem {
    Market,
    Command,
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
    /// Public debt as share of GDP
    pub debt_gdp: f64,
    /// Oil production, million barrels/day (0 for non-producers)
    pub oil_mbd: f64,
    /// Asset bubble intensity 0..1 (Japan starts hot)
    pub bubble: f64,
    /// Last 12m real growth, annualized — for briefings
    pub growth_last: f64,

    // --- Politics ---
    /// 0..100 — regime stability/legitimacy
    pub stability: f64,
    /// Nationalities/separatism strain 0..1 (USSR's clock)
    pub separatism: f64,

    // --- Military ---
    /// Abstract strength index
    pub mil_strength: f64,
    /// 0..1, accumulates in war, decays in peace
    pub war_exhaustion: f64,
    pub nuclear: bool,

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

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GameRules {
    pub seed: u64,
    /// Multiplier on AI war appetite
    pub ai_aggression: f64,
    /// Crisis intensity multiplier (bubbles, collapses)
    pub crisis_intensity: f64,
}
impl Default for GameRules {
    fn default() -> Self {
        GameRules { seed: 1990, ai_aggression: 1.0, crisis_intensity: 1.0 }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WorldState {
    pub rules: GameRules,
    pub rng: Rng,
    pub year: i32,
    pub month: u32, // 1..=12
    pub nations: Vec<Nation>,
    /// relations[(a,b)] symmetric, -100..100 — stored as sorted-pair list
    pub relations: Relations,
    /// sanctions: (imposer, target)
    pub sanctions: Vec<(NationId, NationId)>,
    pub wars: Vec<War>,
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
}

impl WorldState {
    pub fn nation(&self, id: NationId) -> &Nation {
        self.nations.iter().find(|n| n.id == id).expect("nation")
    }
    pub fn nation_mut(&mut self, id: NationId) -> &mut Nation {
        self.nations.iter_mut().find(|n| n.id == id).expect("nation")
    }
    /// For nations that may not exist in this world at all — successor states
    /// before their federation falls, or an older save with a smaller roster.
    pub fn nation_opt(&self, id: NationId) -> Option<&Nation> {
        self.nations.iter().find(|n| n.id == id)
    }
    pub fn has_flag(&self, f: &str) -> bool {
        self.flags.iter().any(|x| x == f)
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
    pub fn trade_dependency(&self, id: NationId, partner: NationId) -> f64 {
        let depth = self.trade_depth(id, partner);
        if depth <= 0.0 {
            return 0.0;
        }
        let (mine, theirs) = match (self.nation_opt(id), self.nation_opt(partner)) {
            (Some(m), Some(t)) => (m.gdp, t.gdp),
            _ => return 0.0,
        };
        depth * (theirs / (mine + theirs).max(1.0))
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
    /// Share of a producer's exports shut out of the market by embargo, 0..1.
    /// Weighted by the sanctioners' share of world GDP — an embargo bites in
    /// proportion to the demand that closes its doors. Smuggling keeps a floor.
    pub fn oil_blockade(&self, target: NationId) -> f64 {
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
        (blocking / world_gdp * 1.15).min(0.85)
    }
    /// Share of a producer's barrels that still reach the market. Embargo shuts
    /// buyers out; war shuts the terminals themselves. Both the world price and
    /// the producer's own revenue read from this one number.
    pub fn oil_export_share(&self, id: NationId) -> f64 {
        let war_share = if self.at_war(id) { 0.25 } else { 1.0 };
        (1.0 - self.oil_blockade(id)) * war_share
    }
    pub fn at_war(&self, id: NationId) -> bool {
        self.wars.iter().any(|w| w.involves(id))
    }
    pub fn headline(&mut self, s: String) {
        self.headlines.push(s);
    }
    pub fn date_str(&self) -> String {
        const M: [&str; 12] = ["Jan","Feb","Mar","Apr","May","Jun","Jul","Aug","Sep","Oct","Nov","Dec"];
        format!("{} {}", M[(self.month - 1) as usize], self.year)
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
