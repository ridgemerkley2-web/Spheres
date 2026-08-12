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
//! Membership is transcribed the way `init.rs`'s relations table is: real
//! neighbourhoods, real hosts. It is *mutable*, because federations come apart —
//! when Yugoslavia dissolves, its four successors become home to the Balkans in
//! its place, and a state that is home to no theatre would be expeditionary in
//! its own capital.

use serde::{Deserialize, Serialize};

use crate::world::{NationId, WorldState};

/// No power commits above this rung into a theatre it is not home to without a
/// consenting host in range. The single line that makes statecraft a direct
/// military input.
pub const MAX_RUNG_WITHOUT_ACCESS: u8 = 5;

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum TheatreId {
    Gulf,
    Levant,
    Balkans,
    SouthAsia,
    EastAsia,
    CentralEurope,
    WesternEurope,
    NorthAmerica,
    WestAfrica,
    SouthAmerica,
    /// Nobody's home and nobody's to grant: where a blockade lives.
    Maritime,
}

impl TheatreId {
    pub fn name(&self) -> &'static str {
        match self {
            TheatreId::Gulf => "the Gulf",
            TheatreId::Levant => "the Levant",
            TheatreId::Balkans => "the Balkans",
            TheatreId::SouthAsia => "South Asia",
            TheatreId::EastAsia => "East Asia",
            TheatreId::CentralEurope => "Central Europe",
            TheatreId::WesternEurope => "Western Europe",
            TheatreId::NorthAmerica => "North America",
            TheatreId::WestAfrica => "West Africa",
            TheatreId::SouthAmerica => "South America",
            TheatreId::Maritime => "the sea lanes",
        }
    }
    pub fn parse(s: &str) -> Option<TheatreId> {
        Some(match s.trim().to_lowercase().as_str() {
            "gulf" | "the gulf" => TheatreId::Gulf,
            "levant" | "the levant" => TheatreId::Levant,
            "balkans" | "the balkans" => TheatreId::Balkans,
            "southasia" | "south asia" => TheatreId::SouthAsia,
            "eastasia" | "east asia" => TheatreId::EastAsia,
            "centraleurope" | "central europe" => TheatreId::CentralEurope,
            "westerneurope" | "western europe" => TheatreId::WesternEurope,
            "northamerica" | "north america" => TheatreId::NorthAmerica,
            "westafrica" | "west africa" => TheatreId::WestAfrica,
            "southamerica" | "south america" => TheatreId::SouthAmerica,
            "maritime" | "sea" | "sea lanes" => TheatreId::Maritime,
            _ => return None,
        })
    }
}

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

/// Transcribed neighbourhoods, in the same spirit as the relations table in
/// `init.rs`: who lives where, and whose airfields an outsider needs.
pub fn default_theatres() -> Vec<Theatre> {
    use NationId::*;
    vec![
        Theatre {
            id: TheatreId::Gulf,
            home: vec![Iraq, Kuwait, SaudiArabia, Iran],
            access_hosts: vec![SaudiArabia, Turkey, Egypt],
            rough: 0.15,
            urbanisation: 0.35,
            population: 91.0,
        },
        Theatre {
            id: TheatreId::Levant,
            home: vec![Israel, Egypt, Turkey],
            access_hosts: vec![Turkey, Egypt, SaudiArabia],
            rough: 0.30,
            urbanisation: 0.55,
            population: 119.0,
        },
        Theatre {
            id: TheatreId::Balkans,
            home: vec![Yugoslavia],
            access_hosts: vec![Italy, Germany, Turkey],
            rough: 0.65,
            urbanisation: 0.40,
            population: 23.5,
        },
        Theatre {
            id: TheatreId::SouthAsia,
            home: vec![India, Pakistan],
            access_hosts: vec![India, Pakistan],
            rough: 0.55,
            urbanisation: 0.30,
            population: 978.0,
        },
        Theatre {
            id: TheatreId::EastAsia,
            home: vec![China, Japan, SouthKorea, Vietnam, Indonesia],
            access_hosts: vec![Japan, SouthKorea],
            rough: 0.45,
            urbanisation: 0.45,
            population: 1551.0,
        },
        Theatre {
            id: TheatreId::CentralEurope,
            home: vec![Germany, Poland, USSR],
            access_hosts: vec![Germany, Poland],
            rough: 0.20,
            urbanisation: 0.60,
            population: 406.0,
        },
        Theatre {
            id: TheatreId::WesternEurope,
            home: vec![UK, France, Italy],
            access_hosts: vec![UK, France, Italy],
            rough: 0.25,
            urbanisation: 0.75,
            population: 171.0,
        },
        Theatre {
            id: TheatreId::NorthAmerica,
            home: vec![USA],
            access_hosts: vec![USA],
            rough: 0.30,
            urbanisation: 0.70,
            population: 250.0,
        },
        Theatre {
            id: TheatreId::WestAfrica,
            home: vec![Nigeria],
            access_hosts: vec![Nigeria],
            rough: 0.40,
            urbanisation: 0.30,
            population: 97.1,
        },
        Theatre {
            id: TheatreId::SouthAmerica,
            home: vec![Brazil],
            access_hosts: vec![Brazil],
            rough: 0.55,
            urbanisation: 0.70,
            population: 149.1,
        },
        Theatre {
            id: TheatreId::Maritime,
            home: vec![],
            access_hosts: vec![],
            rough: 0.0,
            urbanisation: 0.0,
            population: 0.0,
        },
    ]
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

/// True if `seeker` can sustain force in `th`: either it lives there, or some
/// host of that theatre has said yes and not since changed its mind.
pub fn has_access(w: &WorldState, seeker: NationId, th: TheatreId) -> bool {
    if is_home(w, seeker, th) {
        return true;
    }
    let t = theatre(w, th);
    // A state that is itself a host of this theatre is a host because its own
    // airfields are within range of it. Turkey does not have to ask Ankara for
    // Incirlik — that is the whole reason it is on the list.
    if t.access_hosts.contains(&seeker) {
        return true;
    }
    t.access_hosts
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
    if w.nation_opt(host).map_or(true, |n| !n.alive) {
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
        .filter(|h| w.nation_opt(*h).map_or(false, |n| n.alive))
        .collect()
}

/// Swap a departed federation out of its theatre and seat its successors in its
/// place. A state that is home to nothing would be expeditionary in its own
/// capital, and the Bosnian war would resolve as if nobody could reach it.
pub fn replace_home(w: &mut WorldState, th: TheatreId, gone: NationId, successors: &[NationId]) {
    if let Some(t) = w.theatres.iter_mut().find(|t| t.id == th) {
        t.home.retain(|x| *x != gone);
        for s in successors {
            if !t.home.contains(s) {
                t.home.push(*s);
            }
        }
    }
}
