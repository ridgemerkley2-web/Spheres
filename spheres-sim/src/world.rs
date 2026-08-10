use serde::{Deserialize, Serialize};

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
}
pub const ALL_START_NATIONS: [NationId; 24] = [
    NationId::USA, NationId::USSR, NationId::China, NationId::Japan,
    NationId::Germany, NationId::UK, NationId::France, NationId::Italy,
    NationId::India, NationId::Pakistan, NationId::Iraq, NationId::Kuwait,
    NationId::SaudiArabia, NationId::Iran, NationId::SouthKorea, NationId::Poland,
    NationId::Brazil, NationId::Indonesia, NationId::Egypt, NationId::Israel,
    NationId::Turkey, NationId::Nigeria, NationId::Vietnam,
    NationId::Yugoslavia,
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
            _ => return None,
        })
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
}

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
    pub relations: Vec<(NationId, NationId, f64)>,
    /// sanctions: (imposer, target)
    pub sanctions: Vec<(NationId, NationId)>,
    pub wars: Vec<War>,
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
        let (x, y) = if a <= b { (a, b) } else { (b, a) };
        self.relations
            .iter()
            .find(|(p, q, _)| *p == x && *q == y)
            .map(|(_, _, v)| *v)
            .unwrap_or(0.0)
    }
    pub fn set_relation(&mut self, a: NationId, b: NationId, v: f64) {
        let (x, y) = if a <= b { (a, b) } else { (b, a) };
        let v = v.clamp(-100.0, 100.0);
        if let Some(r) = self
            .relations
            .iter_mut()
            .find(|(p, q, _)| *p == x && *q == y)
        {
            r.2 = v;
        } else {
            self.relations.push((x, y, v));
        }
    }
    pub fn shift_relation(&mut self, a: NationId, b: NationId, d: f64) {
        let cur = self.relation(a, b);
        self.set_relation(a, b, cur + d);
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
