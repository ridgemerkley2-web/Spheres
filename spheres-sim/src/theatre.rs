//! Theatres — the operating area a conflict happens in, and who has to consent
//! to your being there.
//!
//! BIBLE §6 object 3. A theatre carries terrain, urbanisation, the states that
//! are *home* to it, and an **access requirement**: the third states whose
//! consent to basing, transit and overflight a non-resident power needs before
//! it can commit above roughly rung 5. Access is a diplomatic quantity settled
//! by a parliament, not a supply formula — Ankara's refusal in March 2003
//! reshaped an invasion, and that is the mechanic this file exists to carry.
//!
//! Who is *home* to a theatre is derived from the roster's `region` column, not
//! typed here. That matters more than it sounds: a state that is home to no
//! theatre is expeditionary in its own capital, and would defend its own border
//! with the fraction of itself it could have sent to Angola. A hand-written
//! membership list was true at thirty-one nations, false at 108, and would have
//! had to be remembered again at every federation that comes apart. Now
//! Yugoslavia's four successors are home to the Balkans because the roster says
//! they are Balkan, and nothing has to be done when the flag comes down.
//!
//! The hosts are still transcribed the way `init.rs`'s relations table is: real
//! airfields, real consents.

use serde::{Deserialize, Serialize};

use crate::world::{NationId, WorldState};

/// No power commits above this rung into a theatre it is not home to without a
/// consenting host in range. The single line that makes statecraft a direct
/// military input.
pub const MAX_RUNG_WITHOUT_ACCESS: u8 = 5;

/// The theatres, one per neighbourhood on the roster.
///
/// This list used to be eleven hand-picked areas holding thirty-one hand-listed
/// nations. At 108 that stopped being possible to keep true — and a nation that
/// is home to no theatre is expeditionary in its own capital, fighting for its
/// own border with its deployable fraction, which is the single most
/// consequential number in §6. So membership is no longer typed: `region` is
/// already on every roster row, and a theatre is a region. Add a nation to the
/// roster and it is seated automatically; `every_nation_has_a_home` is the
/// guard, and after this change it is a guard on the mapping rather than on
/// somebody's diligence.
///
/// One region splits. The Middle East on the roster is two neighbourhoods that
/// the model has to be able to tell apart — the Gulf littoral and the eastern
/// Mediterranean — because they are 1,500km and a different set of hosts apart,
/// and putting Kuwait and Lebanon in one operating area would make Desert Storm
/// and the Lebanese civil war the same theatre. That split, and only that split,
/// is transcribed by name in `LEVANT`.
#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum TheatreId {
    Gulf,
    Levant,
    Balkans,
    SouthAsia,
    EastAsia,
    SoutheastAsia,
    CentralEurope,
    WesternEurope,
    Eurasia,
    NorthAmerica,
    LatinAmerica,
    NorthAfrica,
    WestAfrica,
    EastAfrica,
    CentralAfrica,
    SouthernAfrica,
    Oceania,
    /// Nobody's home and nobody's to grant: where a blockade lives.
    Maritime,
}

/// Every theatre, in a fixed order. Iteration order is behaviour — hosts are
/// walked in table order so a consent roll is reproducible — so this is a
/// stated list and never a map's key order.
pub const ALL_THEATRES: [TheatreId; 18] = [
    TheatreId::Gulf,
    TheatreId::Levant,
    TheatreId::Balkans,
    TheatreId::SouthAsia,
    TheatreId::EastAsia,
    TheatreId::SoutheastAsia,
    TheatreId::CentralEurope,
    TheatreId::WesternEurope,
    TheatreId::Eurasia,
    TheatreId::NorthAmerica,
    TheatreId::LatinAmerica,
    TheatreId::NorthAfrica,
    TheatreId::WestAfrica,
    TheatreId::EastAfrica,
    TheatreId::CentralAfrica,
    TheatreId::SouthernAfrica,
    TheatreId::Oceania,
    TheatreId::Maritime,
];

impl TheatreId {
    pub fn name(&self) -> &'static str {
        match self {
            TheatreId::Gulf => "the Gulf",
            TheatreId::Levant => "the Levant",
            TheatreId::Balkans => "the Balkans",
            TheatreId::SouthAsia => "South Asia",
            TheatreId::EastAsia => "East Asia",
            TheatreId::SoutheastAsia => "Southeast Asia",
            TheatreId::CentralEurope => "Central Europe",
            TheatreId::WesternEurope => "Western Europe",
            TheatreId::Eurasia => "Eurasia",
            TheatreId::NorthAmerica => "North America",
            TheatreId::LatinAmerica => "Latin America",
            TheatreId::NorthAfrica => "North Africa",
            TheatreId::WestAfrica => "West Africa",
            TheatreId::EastAfrica => "East Africa",
            TheatreId::CentralAfrica => "Central Africa",
            TheatreId::SouthernAfrica => "Southern Africa",
            TheatreId::Oceania => "Oceania",
            TheatreId::Maritime => "the sea lanes",
        }
    }
    /// Accepts what a player would type: "gulf", "the Gulf", "south asia",
    /// "SouthAsia". Matched against the display names rather than a second
    /// hand-kept table, so a theatre cannot be added and left unparseable.
    pub fn parse(s: &str) -> Option<TheatreId> {
        let want = squash(s);
        // "sea", "sea lanes" and "maritime" all mean the one theatre nobody is
        // home to, and none of them is spelled the way it is displayed.
        if matches!(want.as_str(), "sea" | "sealanes" | "maritime") {
            return Some(TheatreId::Maritime);
        }
        ALL_THEATRES.iter().copied().find(|t| squash(t.name()) == want)
    }

    /// Which region on the roster this theatre is the operating area for.
    /// `Maritime` is the one theatre that is nobody's neighbourhood.
    fn region(&self) -> Option<&'static str> {
        Some(match self {
            // The Middle East is two theatres; `LEVANT` decides which, and the
            // Gulf takes the remainder.
            TheatreId::Gulf | TheatreId::Levant => "MiddleEast",
            TheatreId::Balkans => "Balkans",
            TheatreId::SouthAsia => "SouthAsia",
            TheatreId::EastAsia => "EastAsia",
            TheatreId::SoutheastAsia => "SoutheastAsia",
            TheatreId::CentralEurope => "EasternEurope",
            TheatreId::WesternEurope => "WesternEurope",
            TheatreId::Eurasia => "Eurasia",
            TheatreId::NorthAmerica => "NorthAmerica",
            TheatreId::LatinAmerica => "LatinAmerica",
            TheatreId::NorthAfrica => "NorthAfrica",
            TheatreId::WestAfrica => "WestAfrica",
            TheatreId::EastAfrica => "EastAfrica",
            TheatreId::CentralAfrica => "CentralAfrica",
            TheatreId::SouthernAfrica => "SouthernAfrica",
            TheatreId::Oceania => "Oceania",
            TheatreId::Maritime => return None,
        })
    }
}

fn squash(s: &str) -> String {
    let s = s.trim().to_lowercase().replace([' ', '-', '_'], "");
    s.strip_prefix("the").unwrap_or(&s).to_string()
}

/// The eastern-Mediterranean half of the Middle East region. Everything else
/// filed under `MiddleEast` on the roster is home to the Gulf.
///
/// Turkey is here and not in Europe because that is where its army fights: the
/// Syrian and Iraqi borders, not the Bulgarian one. Egypt is *not* here — it is
/// home to North Africa, where it lives — but it is an access host for both this
/// theatre and the Gulf, which is exactly the distinction §6 draws between where
/// you live and whose airfields you need.
const LEVANT: &[&str] = &["Israel", "Syria", "Jordan", "Lebanon", "Turkey"];

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Theatre {
    pub id: TheatreId,
    /// Whose ground this is. A belligerent fighting at home commits its whole
    /// force structure; everybody else is limited by its deployable fraction.
    pub home: Vec<NationId>,
    /// Whose consent buys an outsider a base within range.
    pub access_hosts: Vec<NationId>,
    /// 0..1 — mountain, forest, jungle, marsh. Cover from being found.
    pub rough: f64,
    /// 0..1 — how much of the fighting happens among buildings and people.
    pub urbanisation: f64,
    /// Millions, for the weight an occupation has to hold down.
    pub population: f64,
}

/// A standing consent, granted once by a parliament and revocable. Not a
/// per-month roll: a base agreement is a treaty, and treaties persist until
/// somebody votes them away.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Access {
    pub theatre: TheatreId,
    pub host: NationId,
    pub seeker: NationId,
    pub since_year: i32,
    pub since_month: u32,
}

/// Ground, hosts and terrain for one theatre.
///
/// `home` is derived — it is the region's membership on the roster, so a nation
/// added to the roster is seated without anyone remembering to seat it, and a
/// federation that comes apart hands its ground to successors who were already
/// filed under their own regions. Only the three things a region cannot state
/// are transcribed here: whose airfields an outsider needs, and what the ground
/// is like.
///
/// The access-host lists are the real ones. Every name on them either hosted a
/// foreign force in the period or is the state whose consent an outsider had to
/// have to operate there at all.
pub fn default_theatres() -> Vec<Theatre> {
    ALL_THEATRES
        .iter()
        .copied()
        .map(|id| Theatre {
            id,
            home: home_roster(id),
            access_hosts: access_hosts(id),
            rough: terrain(id).0,
            urbanisation: terrain(id).1,
            population: terrain(id).2,
        })
        .collect()
}

/// Everyone on the roster whose region this theatre covers, in roster order.
fn home_roster(id: TheatreId) -> Vec<NationId> {
    let region = match id.region() {
        Some(r) => r,
        None => return vec![],
    };
    crate::world::all_nations()
        .iter()
        .copied()
        .filter(|n| n.def().region == region)
        .filter(|n| match id {
            // The one split. Everything in the Middle East that is not on the
            // Levant list is on the Gulf.
            TheatreId::Levant => LEVANT.contains(&n.def().code),
            TheatreId::Gulf => !LEVANT.contains(&n.def().code),
            _ => true,
        })
        .collect()
}

/// Whose consent buys an outsider a base within range. Transcribed, and the
/// only country names left in this file besides `LEVANT`.
fn access_hosts(id: TheatreId) -> Vec<NationId> {
    let codes: &[&str] = match id {
        // Dhahran, Incirlik, Cairo West, Seeb, Sheikh Isa, Doha, Al Dhafra —
        // the 1990-91 coalition's actual laydown.
        TheatreId::Gulf => &["SaudiArabia", "Turkey", "Egypt", "Oman", "Bahrain", "Qatar", "UAE"],
        TheatreId::Levant => &["Turkey", "Egypt", "Israel", "Jordan", "SaudiArabia"],
        // Aviano, Ramstein, Incirlik, Souda Bay, Taszar.
        TheatreId::Balkans => &["Italy", "Germany", "Turkey", "Greece", "Hungary"],
        TheatreId::SouthAsia => &["India", "Pakistan"],
        // Yokosuka, Kadena, Osan, and Subic and Clark until the Senate voted
        // them out in September 1991.
        TheatreId::EastAsia => &["Japan", "SouthKorea", "Philippines"],
        TheatreId::SoutheastAsia => &["Thailand", "Singapore", "Philippines", "Australia"],
        TheatreId::CentralEurope => &["Germany", "Poland", "Hungary", "Romania"],
        TheatreId::WesternEurope => {
            &["UK", "Germany", "Italy", "France", "Spain", "Portugal", "Greece"]
        }
        // The ways in are the rim, not the middle: Turkey, the Caucasus and the
        // Central Asian republics that later hosted Karshi-Khanabad and the rest.
        TheatreId::Eurasia => &["Turkey", "Georgia", "Azerbaijan", "Uzbekistan", "Kazakhstan"],
        TheatreId::NorthAmerica => &["USA", "Canada"],
        TheatreId::LatinAmerica => &["Brazil", "Colombia", "Ecuador", "Peru", "Cuba"],
        TheatreId::NorthAfrica => &["Egypt", "Morocco", "Tunisia", "Algeria"],
        TheatreId::WestAfrica => &["Nigeria", "Senegal", "Ghana"],
        TheatreId::EastAfrica => &["Kenya", "Ethiopia", "Uganda", "Tanzania"],
        TheatreId::CentralAfrica => &["Zaire", "Cameroon"],
        TheatreId::SouthernAfrica => &["SouthAfrica", "Angola", "Zimbabwe"],
        TheatreId::Oceania => &["Australia", "NewZealand"],
        // A blockade asks nobody.
        TheatreId::Maritime => &[],
    };
    codes.iter().filter_map(|c| NationId::from_code(c)).collect()
}

/// `(rough, urbanisation, population in millions)`. Cover from being found, how
/// much of the fighting happens among buildings, and the weight an occupation
/// has to hold down. The population figures are 1990 regional totals and are
/// carried for display; nothing in the resolution reads them.
fn terrain(id: TheatreId) -> (f64, f64, f64) {
    match id {
        TheatreId::Gulf => (0.15, 0.35, 109.5),
        TheatreId::Levant => (0.35, 0.60, 80.1),
        TheatreId::Balkans => (0.65, 0.40, 35.5),
        TheatreId::SouthAsia => (0.55, 0.30, 1132.0),
        TheatreId::EastAsia => (0.45, 0.45, 1343.0),
        TheatreId::SoutheastAsia => (0.70, 0.35, 435.0),
        TheatreId::CentralEurope => (0.25, 0.55, 99.6),
        TheatreId::WesternEurope => (0.25, 0.75, 374.8),
        // The one theatre where the ground itself is the defence, and the
        // reason nobody has ever finished a campaign in it.
        TheatreId::Eurasia => (0.45, 0.60, 289.0),
        TheatreId::NorthAmerica => (0.30, 0.70, 277.7),
        TheatreId::LatinAmerica => (0.55, 0.70, 383.0),
        TheatreId::NorthAfrica => (0.20, 0.45, 145.0),
        TheatreId::WestAfrica => (0.40, 0.30, 119.7),
        TheatreId::EastAfrica => (0.60, 0.20, 114.4),
        TheatreId::CentralAfrica => (0.75, 0.30, 48.6),
        TheatreId::SouthernAfrica => (0.45, 0.45, 56.1),
        TheatreId::Oceania => (0.35, 0.85, 20.5),
        TheatreId::Maritime => (0.0, 0.0, 0.0),
    }
}

pub fn theatre(w: &WorldState, id: TheatreId) -> &Theatre {
    w.theatres
        .iter()
        .find(|t| t.id == id)
        .expect("every theatre id is seeded")
}

/// Where a nation lives. Every living nation is home to exactly one theatre —
/// see `every_nation_has_a_home`, which is the guard against a dissolution
/// forgetting to re-seat its successors.
pub fn home_theatre(w: &WorldState, id: NationId) -> Option<TheatreId> {
    w.theatres
        .iter()
        .find(|t| t.home.contains(&id))
        .map(|t| t.id)
}

pub fn is_home(w: &WorldState, id: NationId, th: TheatreId) -> bool {
    theatre(w, th).home.contains(&id)
}

/// True if `seeker` can sustain force in `th` and NOBODY'S ANSWER CAN CHANGE
/// THAT: it lives there, or it is itself one of the theatre's hosts.
///
/// [`has_access`]'s two short-circuits, extracted so the one other place that
/// has to know them can be told instead of guessing — the browser's basing
/// panel, which was offering to sell a nation home to a theatre the basing it
/// already had and could not lose. Behaviour is unchanged: `has_access` calls
/// this, so there is still one definition of the rule.
pub fn needs_no_host(w: &WorldState, seeker: NationId, th: TheatreId) -> bool {
    // A state that is itself a host of this theatre is a host because its own
    // airfields are within range of it. Turkey does not have to ask Ankara for
    // Incirlik — that is the whole reason it is on the list.
    is_home(w, seeker, th) || theatre(w, th).access_hosts.contains(&seeker)
}

/// True if `seeker` can sustain force in `th`: either it needs nobody's
/// permission, or some host of that theatre has said yes and not since changed
/// its mind.
pub fn has_access(w: &WorldState, seeker: NationId, th: TheatreId) -> bool {
    if needs_no_host(w, seeker, th) {
        return true;
    }
    theatre(w, th)
        .access_hosts
        .iter()
        .any(|h| w.access.iter().any(|a| a.theatre == th && a.host == *h && a.seeker == seeker))
}

/// Which host, if any, is carrying an outsider's presence — for the readout that
/// tells a player in one line why they can be where they are.
pub fn granting_host(w: &WorldState, seeker: NationId, th: TheatreId) -> Option<NationId> {
    let t = theatre(w, th);
    t.access_hosts
        .iter()
        .copied()
        .find(|h| w.access.iter().any(|a| a.theatre == th && a.host == *h && a.seeker == seeker))
}

/// Has this host already answered this seeker, either way?
pub fn already_granted(w: &WorldState, host: NationId, seeker: NationId, th: TheatreId) -> bool {
    w.access
        .iter()
        .any(|a| a.theatre == th && a.host == host && a.seeker == seeker)
}

/// The odds a parliament says yes, spelled out so the player can read the
/// drivers off the access panel rather than guess at them.
///
/// Nothing here is a supply formula. A base agreement is granted by a government
/// that likes you, dislikes the state you mean to use it against, cannot afford
/// to annoy your exporters, or has already promised to fight beside you — and by
/// nobody who is currently sanctioning you. Ankara in March 2003 is a roll on
/// this line, and it came up short.
pub fn consent_probability(
    w: &WorldState,
    host: NationId,
    seeker: NationId,
    target: Option<NationId>,
    unrestricted: bool,
) -> f64 {
    if host == seeker {
        return 1.0;
    }
    if w.nation_opt(host).is_none_or(|n| !n.alive) {
        return 0.0;
    }
    // You do not get to fly out of a country that has closed its market to you.
    if w.is_sanctioning(host, seeker) || w.is_sanctioning(seeker, host) {
        return 0.0;
    }
    let mut p = ((w.relation(host, seeker) + 20.0) / 120.0).clamp(0.0, 0.90);
    if let Some(t) = target {
        // The enemy of the state you are about to hit is the friend of your
        // airbase request.
        p *= 1.0 + 0.6 * (-w.relation(host, t) / 100.0).clamp(0.0, 1.0);
        // ...and nobody hosts a strike on themselves or on their own guarantor.
        if t == host || w.allied(host, t) {
            return 0.0;
        }
    }
    p *= 1.0 + 0.8 * w.trade_dependency(host, seeker);
    if w.allied(host, seeker) {
        p *= 1.5;
    }
    // Civilian casualties are what turns a parliament, and they turn it against
    // the government that let the aircraft take off from its soil.
    if unrestricted {
        p *= 0.6;
    }
    p.clamp(0.0, 0.95)
}

/// Record a standing consent. Idempotent: a parliament that has already voted
/// yes does not vote yes twice.
pub fn grant(w: &mut WorldState, host: NationId, seeker: NationId, th: TheatreId) {
    if already_granted(w, host, seeker, th) {
        return;
    }
    let (y, m) = (w.year, w.month);
    w.access.push(Access { theatre: th, host, seeker, since_year: y, since_month: m });
}

pub fn revoke(w: &mut WorldState, host: NationId, seeker: NationId, th: TheatreId) -> bool {
    let before = w.access.len();
    w.access
        .retain(|a| !(a.theatre == th && a.host == host && a.seeker == seeker));
    w.access.len() != before
}

/// The hosts of a theatre a seeker has not yet asked, in table order — so the
/// order a request walks them in is fixed and the roll is reproducible.
pub fn unasked_hosts(w: &WorldState, seeker: NationId, th: TheatreId) -> Vec<NationId> {
    theatre(w, th)
        .access_hosts
        .iter()
        .copied()
        .filter(|h| *h != seeker)
        .filter(|h| !already_granted(w, *h, seeker, th))
        .filter(|h| w.nation_opt(*h).is_some_and(|n| n.alive))
        .collect()
}
