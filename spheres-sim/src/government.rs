//! Government — parties, elections, coalitions, and the regimes that hold power
//! without any of them.
//!
//! What was here before this module was four lines in `politics.rs`: every four
//! years in November, a democracy gained three stability, or eight if times were
//! bad. That is a slider, and BIBLE section 4 names the party-popularity slider
//! as one of the five things this game replaces — with "political capital as a
//! real budget, coalitions, and legitimacy earned by delivery", so that
//! *governing becomes a constraint rather than a colour*.
//!
//! The four claims this module has to make good on:
//!
//! 1. **The parties are real.** Every one below existed under that name in this
//!    period, and its opening support is the share it actually won at the last
//!    national election before January 1990. Sources are on each block.
//! 2. **Support moves because of what the economy did to people.** Nobody sets a
//!    popularity number. Inflation, the growth the government delivered or did
//!    not, the war it is fighting, and the order it is keeping push support away
//!    from whoever is in office and toward whichever family of opposition that
//!    particular pain favours. Prices going up help the hard-money right; a
//!    recession helps the left; a war and a disintegrating state help the
//!    nationalists.
//! 3. **The result is something you have to govern with.** A first-past-the-post
//!    system manufactures majorities out of pluralities; proportional systems do
//!    not, and then somebody has to assemble a coalition and pay to hold it.
//! 4. **The coalition constrains you.** A broad, ideologically stretched
//!    government bleeds political capital every month and holds a lower ceiling
//!    than a single-party majority. That is the bite: the same tax rise costs
//!    Italy's five-party pentapartito more than it costs a British government
//!    with a hundred-seat majority, because Italy has to buy four other parties'
//!    consent out of the same budget.
//!
//! Authoritarian regimes get the other half. No elections; legitimacy bought
//! from the institutions that could remove you — the army, the party apparatus,
//! the security services, the merchants, the clergy — and a coup when the buying
//! stops. Nothing here is scheduled and nothing is named after a country: the
//! 1991 August coup and the 1990 dismissal of a Pakistani government are both
//! reachable, and neither is written down.

use crate::world::*;
use serde::{Deserialize, Serialize};

// ---------------------------------------------------------------------------
// Ideology
// ---------------------------------------------------------------------------

/// The party families of late-twentieth-century politics. A family is not a
/// flavour label: it decides which discontents a party collects, and how far it
/// is from a would-be coalition partner.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Family {
    Communist,
    SocialDemocratic,
    Green,
    Liberal,
    ChristianDemocratic,
    Conservative,
    Nationalist,
    Religious,
    Agrarian,
    /// A party that is a coalition in itself — Congress, the PMDB, Solidarity,
    /// DEMOS. Cheap to govern with and impossible to hold together.
    BigTent,
    /// Organised around a place rather than a programme.
    Regionalist,
}

impl Family {
    /// Position on two axes: economic left(-1)..right(+1), and
    /// cosmopolitan(-1)..national(+1). Coalition distance is the plane between
    /// them, which is why a Green and a Nationalist cannot sit in the same
    /// cabinet however close their economics.
    pub fn axis(self) -> (f64, f64) {
        match self {
            Family::Communist => (-1.00, -0.20),
            Family::SocialDemocratic => (-0.50, -0.10),
            Family::Green => (-0.40, -0.60),
            Family::Liberal => (0.20, -0.50),
            Family::ChristianDemocratic => (0.30, 0.10),
            Family::Conservative => (0.60, 0.30),
            Family::Nationalist => (0.20, 1.00),
            Family::Religious => (0.20, 0.70),
            Family::Agrarian => (-0.10, 0.40),
            Family::BigTent => (0.00, 0.00),
            Family::Regionalist => (-0.20, 0.60),
        }
    }
    pub fn label(self) -> &'static str {
        match self {
            Family::Communist => "communist",
            Family::SocialDemocratic => "social democratic",
            Family::Green => "green",
            Family::Liberal => "liberal",
            Family::ChristianDemocratic => "christian democratic",
            Family::Conservative => "conservative",
            Family::Nationalist => "nationalist",
            Family::Religious => "religious",
            Family::Agrarian => "agrarian",
            Family::BigTent => "big tent",
            Family::Regionalist => "regionalist",
        }
    }
}

fn family_distance(a: Family, b: Family) -> f64 {
    let (ax, ay) = a.axis();
    let (bx, by) = b.axis();
    ((ax - bx).powi(2) + (ay - by).powi(2)).sqrt()
}

// ---------------------------------------------------------------------------
// The transcribed data
// ---------------------------------------------------------------------------

/// One real party, with the share it actually won at the last national election
/// before the game opens.
pub struct PartySpec {
    /// Stable id, written into saves and typed by the player. Never rename.
    pub id: &'static str,
    pub name: &'static str,
    /// The name in its own language where that is the name people used.
    pub native: &'static str,
    pub family: Family,
    /// Vote share at that last election, 0..1. Normalised at seating.
    pub start: f64,
    /// Nobody will sit in cabinet with them. A real institution of this period —
    /// Italy's *conventio ad excludendum* against the PCI and the MSI, France's
    /// cordon sanitaire against the Front National.
    pub pariah: bool,
}

const fn p(
    id: &'static str,
    name: &'static str,
    native: &'static str,
    family: Family,
    start: f64,
) -> PartySpec {
    PartySpec { id, name, native, family, start, pariah: false }
}
const fn pariah(
    id: &'static str,
    name: &'static str,
    native: &'static str,
    family: Family,
    start: f64,
) -> PartySpec {
    PartySpec { id, name, native, family, start, pariah: true }
}

/// How votes become seats. The choice is not cosmetic: it decides whether a
/// plurality is a government or the beginning of a negotiation.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Electoral {
    /// Single-member plurality. Roughly obeys the cube law — a party with 55% of
    /// the two-party vote takes about 63% of the seats — so majorities are
    /// manufactured out of pluralities.
    FirstPastThePost,
    /// Two rounds with a runoff: majoritarian, but less brutally so.
    TwoRound,
    /// Proportional with the ordinary continental 5% threshold.
    Proportional,
    /// Proportional with a 10% threshold — Turkey's, the highest in Europe, put
    /// in place by the 1982 constitution to keep small parties out.
    ProportionalHighBar,
    /// Proportional with a 1% threshold — Israel's until 1992, which is why the
    /// Knesset carries a dozen parties and no government is ever one party.
    ProportionalLowBar,
    /// Japan's multi-member districts with a single non-transferable vote, in
    /// force until the 1994 reform. Mildly majoritarian.
    SingleNonTransferable,
}

impl Electoral {
    /// (seat-share exponent, threshold). The exponent is the majoritarian bias:
    /// 1.0 is proportional, 3.0 is the cube law.
    fn shape(self) -> (f64, f64) {
        match self {
            Electoral::FirstPastThePost => (3.0, 0.0),
            Electoral::TwoRound => (2.0, 0.05),
            Electoral::Proportional => (1.0, 0.05),
            Electoral::ProportionalHighBar => (1.0, 0.10),
            Electoral::ProportionalLowBar => (1.0, 0.01),
            Electoral::SingleNonTransferable => (1.6, 0.0),
        }
    }
    pub fn label(self) -> &'static str {
        match self {
            Electoral::FirstPastThePost => "first past the post",
            Electoral::TwoRound => "two-round majoritarian",
            Electoral::Proportional => "proportional (5% threshold)",
            Electoral::ProportionalHighBar => "proportional (10% threshold)",
            Electoral::ProportionalLowBar => "proportional (1% threshold)",
            Electoral::SingleNonTransferable => "multi-member, single non-transferable vote",
        }
    }
}

/// An institution that can remove a government which is not elected.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Pillar {
    Army,
    Party,
    Security,
    Business,
    Clergy,
}
impl Pillar {
    pub fn parse(s: &str) -> Option<Pillar> {
        Some(match s.trim().to_lowercase().as_str() {
            "army" | "military" | "guard" => Pillar::Army,
            "party" | "apparatus" | "court" => Pillar::Party,
            "security" | "police" | "intelligence" => Pillar::Security,
            "business" | "merchants" | "money" => Pillar::Business,
            "clergy" | "church" | "ulema" => Pillar::Clergy,
            _ => return None,
        })
    }
    pub fn key(self) -> &'static str {
        match self {
            Pillar::Army => "army",
            Pillar::Party => "party",
            Pillar::Security => "security",
            Pillar::Business => "business",
            Pillar::Clergy => "clergy",
        }
    }
}

/// A named pillar of one specific regime. Real institutions, because "the army"
/// is not a thing that removes a government — the Republican Guard is.
pub struct PillarSpec {
    pub pillar: Pillar,
    pub name: &'static str,
}
const fn pl(pillar: Pillar, name: &'static str) -> PillarSpec {
    PillarSpec { pillar, name }
}

/// Everything the model knows about how one nation is governed.
pub struct Polity {
    pub nation: NationId,
    pub system: Electoral,
    /// Maximum length of a parliament, in months.
    pub term_months: u32,
    /// The next election actually due when the game opens, where one was.
    /// (0, 0) means the regime does not hold them — until it liberalises, at
    /// which point the party table below becomes live.
    pub next: (i32, u32),
    pub parties: &'static [PartySpec],
    /// What is in power when nobody votes, and who could take it away.
    pub ruling: &'static str,
    pub pillars: &'static [PillarSpec],
}

/// A government is electoral when the regime is open enough to be removed by a
/// vote. The threshold takes in Pakistan (Benazir Bhutto's 1988 government,
/// which a president dismissed in August 1990 — the model reaches that through
/// instability, not a date) and leaves out Kuwait, whose National Assembly the
/// Emir dissolved in 1986 and did not recall until 1992.
pub const ELECTORAL_CEILING: f64 = 0.60;

pub fn is_electoral(w: &WorldState, id: NationId) -> bool {
    w.nation_opt(id).map_or(false, |n| n.alive && n.authoritarianism < ELECTORAL_CEILING)
}

// The tables. Vote shares are from the last national election before January
// 1990 in each country; where a party's founding postdates that election its
// share is its result at the first one it contested, noted on the block.
pub const POLITIES: &[Polity] = &[
    // United States — 1988 House of Representatives popular vote: Democrats
    // 53.4%, Republicans 45.5%. SPHERES models the legislature a government has
    // to carry rather than a head of state, so the congressional vote is the
    // right row: divided government is the American form of the coalition
    // problem. https://history.house.gov/Institution/Election-Statistics/
    Polity {
        nation: NationId::USA,
        system: Electoral::FirstPastThePost,
        term_months: 48,
        next: (1992, 11),
        parties: &[
            p("us_dem", "Democratic Party", "", Family::Liberal, 0.534),
            p("us_rep", "Republican Party", "", Family::Conservative, 0.455),
        ],
        ruling: "the Congress of the United States",
        pillars: &[],
    },
    // Soviet Union — Article 6 of the 1977 constitution still gave the CPSU its
    // monopoly on 1 January 1990; the Congress of People's Deputies repealed it
    // on 14 March. The party table is the one that formed once it did, and it
    // goes live only if the regime opens up. Support shares are the 1993 Russian
    // Duma party-list result, the first fully contested election on this
    // territory. The pillars are the ones that actually moved in August 1991.
    Polity {
        nation: NationId::USSR,
        system: Electoral::Proportional,
        term_months: 48,
        next: (0, 0),
        parties: &[
            p("su_cpsu", "Communist Party of the Soviet Union", "Kommunisticheskaya Partiya Sovetskogo Soyuza", Family::Communist, 0.40),
            p("su_dr", "Democratic Russia", "Demokraticheskaya Rossiya", Family::Liberal, 0.35),
            p("su_soyuz", "Soyuz group", "Soyuz", Family::Nationalist, 0.25),
        ],
        ruling: "the Communist Party of the Soviet Union",
        pillars: &[
            pl(Pillar::Army, "the Soviet Army"),
            pl(Pillar::Party, "the Central Committee apparatus"),
            pl(Pillar::Security, "the Committee for State Security (KGB)"),
        ],
    },
    // Russia — 1993 State Duma party-list vote: LDPR 22.9%, Russia's Choice
    // 15.5%, CPRF 12.4%, Women of Russia 8.1%, Agrarians 8.0%, Yabloko 7.9%.
    // The first election a post-Soviet Russia held, and the one that told
    // everyone the transition was not going to be liberal.
    Polity {
        nation: NationId::Russia,
        system: Electoral::Proportional,
        term_months: 48,
        next: (0, 0),
        parties: &[
            p("ru_ldpr", "Liberal Democratic Party of Russia", "Liberalno-demokraticheskaya partiya Rossii", Family::Nationalist, 0.229),
            p("ru_vybor", "Russia's Choice", "Vybor Rossii", Family::Liberal, 0.155),
            p("ru_kprf", "Communist Party of the Russian Federation", "Kommunisticheskaya partiya Rossiyskoy Federatsii", Family::Communist, 0.124),
            p("ru_apr", "Agrarian Party of Russia", "Agrarnaya partiya Rossii", Family::Agrarian, 0.080),
            p("ru_yabloko", "Yabloko", "Yabloko", Family::Liberal, 0.079),
        ],
        ruling: "the Presidency of the Russian Federation",
        pillars: &[
            pl(Pillar::Army, "the Russian Armed Forces"),
            pl(Pillar::Security, "the security services"),
            pl(Pillar::Business, "the new financial groups"),
        ],
    },
    // Ukraine — 1998 Verkhovna Rada party-list vote: CPU 24.7%, Rukh 9.4%,
    // Socialist/Peasant bloc 8.6%, People's Democratic Party 5.0%. The 1994
    // election was fought largely by independents, so 1998 is the first result
    // that reads as a party system.
    Polity {
        nation: NationId::Ukraine,
        system: Electoral::Proportional,
        term_months: 48,
        next: (0, 0),
        parties: &[
            p("ua_kpu", "Communist Party of Ukraine", "Komunistychna partiya Ukrayiny", Family::Communist, 0.247),
            p("ua_rukh", "People's Movement of Ukraine", "Narodnyi Rukh Ukrayiny", Family::Nationalist, 0.094),
            p("ua_spu", "Socialist Party of Ukraine", "Sotsialistychna partiya Ukrayiny", Family::SocialDemocratic, 0.086),
            p("ua_ndp", "People's Democratic Party", "Narodno-demokratychna partiya", Family::Liberal, 0.050),
        ],
        ruling: "the Presidency of Ukraine",
        pillars: &[
            pl(Pillar::Army, "the Ukrainian Armed Forces"),
            pl(Pillar::Business, "the industrial directors"),
        ],
    },
    // ---------------------------------------------------------------------
    // The other ten Soviet successors. Each block is the founding national vote
    // that put the republic's first sovereign parliament or president in place:
    // for most of them the republican Supreme Soviet elections of spring 1990,
    // the first competitive elections held on that soil since the annexations,
    // and for a few the first post-independence contest, because the 1990
    // result there was a one-party formality with no published shares.
    // ---------------------------------------------------------------------

    // Belarus - Supreme Soviet of the Byelorussian SSR, 4 March 1990 with
    // runoffs into May. Seat shares, not votes: the Communist Party of
    // Byelorussia took the overwhelming majority of the 310 seats and the
    // Belarusian Popular Front's opposition caucus settled at around 37 of
    // them. Belarus is the republic where the old apparatus was least disturbed
    // by 1991, and that is the fact this table exists to carry.
    Polity {
        nation: NationId::Belarus,
        system: Electoral::FirstPastThePost,
        term_months: 60,
        next: (0, 0),
        parties: &[
            p("by_kpb", "Communist Party of Byelorussia", "Kamunistychnaya partyya Belarusi", Family::Communist, 0.850),
            p("by_bnf", "Belarusian Popular Front", "Belaruski Narodny Front", Family::Nationalist, 0.120),
        ],
        ruling: "the Supreme Soviet",
        pillars: &[
            pl(Pillar::Army, "the Belarusian Military District's inheritance"),
            pl(Pillar::Party, "the collective-farm and industrial nomenklatura"),
            pl(Pillar::Security, "the State Security Committee"),
        ],
    },
    // Kazakhstan - 1 December 1991 presidential election: Nursultan Nazarbayev
    // unopposed with 98.8%. Azat and Zheltoqsan, the two national-democratic
    // movements that would have contested it, were refused registration, so
    // there is genuinely no second row to transcribe. A single party here is
    // the correct description of the republic and not a gap in it: Kazakhstan
    // has never held a national election an observer mission called free.
    Polity {
        nation: NationId::Kazakhstan,
        system: Electoral::FirstPastThePost,
        term_months: 60,
        next: (0, 0),
        parties: &[
            p("kz_snek", "Union of People's Unity of Kazakhstan", "Qazaqstan Halyq Birligi Odagy", Family::BigTent, 0.988),
        ],
        ruling: "the Presidency of the Republic of Kazakhstan",
        pillars: &[
            pl(Pillar::Army, "the Kazakh Armed Forces"),
            pl(Pillar::Party, "the presidential apparatus"),
            pl(Pillar::Security, "the Committee for National Security"),
            pl(Pillar::Business, "the oil and metals groups"),
        ],
    },
    // Uzbekistan - 29 December 1991 presidential election: Islam Karimov 86.0%,
    // Muhammad Salih of Erk 12.7%. Erk was banned within two years and Salih
    // left the country; Birlik, the larger opposition movement, was never
    // allowed onto the ballot at all. The First Secretary became the President
    // without an interval, which is why the pillars here are the Soviet ones
    // under new names.
    Polity {
        nation: NationId::Uzbekistan,
        system: Electoral::TwoRound,
        term_months: 60,
        next: (0, 0),
        parties: &[
            p("uz_pdp", "People's Democratic Party of Uzbekistan", "Ozbekiston Xalq Demokratik Partiyasi", Family::BigTent, 0.860),
            p("uz_erk", "Erk Democratic Party", "Erk Demokratik Partiyasi", Family::Liberal, 0.127),
        ],
        ruling: "the Presidency of the Republic of Uzbekistan",
        pillars: &[
            pl(Pillar::Army, "the Turkestan Military District's inheritance"),
            pl(Pillar::Party, "the People's Democratic Party apparatus"),
            pl(Pillar::Security, "the National Security Service"),
        ],
    },
    // Georgia - Supreme Council, 28 October 1990: Zviad Gamsakhurdia's Round
    // Table-Free Georgia 64.0%, the Communist Party of Georgia 29.6%. The first
    // multi-party election in any Soviet republic won outright by the
    // opposition, and the government it produced was overthrown by its own
    // National Guard fourteen months later.
    Polity {
        nation: NationId::Georgia,
        system: Electoral::Proportional,
        term_months: 48,
        next: (0, 0),
        parties: &[
            p("ge_mrsm", "Round Table-Free Georgia", "Mrgvali Magida-Tavisupali Sakartvelo", Family::Nationalist, 0.640),
            p("ge_kpg", "Communist Party of Georgia", "Sakartvelos Komunisturi Partia", Family::Communist, 0.296),
        ],
        ruling: "the Supreme Council of Georgia",
        pillars: &[
            pl(Pillar::Army, "the National Guard and the Mkhedrioni"),
            pl(Pillar::Security, "the state security apparatus"),
        ],
    },
    // Armenia - 16 October 1991 presidential election: Levon Ter-Petrosyan of
    // the Pan-Armenian National Movement 83.0%, Paruyr Hayrikyan 7.2%, Sos
    // Sargsyan 4.3%. Held five weeks after the independence referendum, with
    // the Karabakh war already running and the Azerbaijani blockade closing.
    Polity {
        nation: NationId::Armenia,
        system: Electoral::TwoRound,
        term_months: 60,
        next: (0, 0),
        parties: &[
            p("am_hhsh", "Pan-Armenian National Movement", "Hayots Hamazgayin Sharzhum", Family::BigTent, 0.830),
            p("am_ansd", "National Self-Determination Union", "Azgayin Inknoroshum Miavorum", Family::Nationalist, 0.072),
            p("am_hhd", "Armenian Revolutionary Federation", "Hay Heghapokhakan Dashnaktsutyun", Family::SocialDemocratic, 0.043),
        ],
        ruling: "the Presidency of the Republic of Armenia",
        pillars: &[
            pl(Pillar::Army, "the Armenian Army and the Karabakh volunteers"),
            pl(Pillar::Security, "the state security apparatus"),
        ],
    },
    // Azerbaijan - 7 June 1992 presidential election: Abulfaz Elchibey of the
    // Popular Front 59.4%, Nizami Suleymanov 33.0%. The 1990 Supreme Soviet
    // election was run by the Communist Party under the state of emergency
    // imposed after Black January and published no comparable shares, so 1992
    // is the first result that describes the country. Elchibey lasted a year:
    // Karabakh took his government down, as it had taken down the one before.
    Polity {
        nation: NationId::Azerbaijan,
        system: Electoral::TwoRound,
        term_months: 60,
        next: (0, 0),
        parties: &[
            p("az_axc", "Popular Front of Azerbaijan", "Azarbaycan Xalq Cabhasi", Family::Nationalist, 0.594),
            p("az_msi", "Independent Azerbaijan bloc", "Musteqil Azarbaycan", Family::BigTent, 0.330),
        ],
        ruling: "the Presidency of the Republic of Azerbaijan",
        pillars: &[
            pl(Pillar::Army, "the Azerbaijani Army and the OMON detachments"),
            pl(Pillar::Security, "the Ministry of National Security"),
            pl(Pillar::Business, "the state oil company"),
        ],
    },
    // Lithuania - Supreme Council, 24 February 1990 with runoffs in March. Seat
    // shares of 141: Sajudis-endorsed candidates 91, the Communist Party of
    // Lithuania that had already broken with Moscow about 40, the Polish
    // electoral caucus around 7. This is the parliament that declared
    // independence on 11 March 1990, eleven days after it was seated, and it is
    // the earliest of the three Baltic declarations.
    Polity {
        nation: NationId::Lithuania,
        system: Electoral::Proportional,
        term_months: 48,
        next: (0, 0),
        parties: &[
            p("lt_sajudis", "Sajudis", "Lietuvos Persitvarkymo Sajudis", Family::BigTent, 0.645),
            p("lt_ldpp", "Lithuanian Democratic Labour Party", "Lietuvos demokratine darbo partija", Family::SocialDemocratic, 0.284),
            p("lt_lls", "Union of Poles in Lithuania", "Lietuvos lenku sajunga", Family::Regionalist, 0.050),
        ],
        ruling: "the Seimas",
        pillars: &[],
    },
    // Latvia - Supreme Council, 18 March 1990. Seat shares of 201: the Popular
    // Front of Latvia 131, the pro-Soviet Equal Rights caucus 55. The Front had
    // the two-thirds it needed to vote the restoration of independence on 4 May,
    // and Equal Rights is the parliamentary form of the Russophone third of the
    // country that the citizenship law of 1994 then left outside the electorate.
    Polity {
        nation: NationId::Latvia,
        system: Electoral::Proportional,
        term_months: 48,
        next: (0, 0),
        parties: &[
            p("lv_ltf", "Popular Front of Latvia", "Latvijas Tautas fronte", Family::BigTent, 0.652),
            p("lv_lidz", "Equal Rights", "Lidztiesiba", Family::Communist, 0.274),
        ],
        ruling: "the Saeima",
        pillars: &[],
    },
    // Estonia - 20 September 1992 Riigikogu election: Pro Patria 22.0%, Safe
    // Home 13.6%, the Popular Front 12.3%, the Moderates 9.7%, the National
    // Independence Party 8.8%, Estonian Citizen 6.9%. The first election
    // anywhere in the former union held under a restored pre-war constitution,
    // and the first in which only citizens of the inter-war republic and their
    // descendants could vote.
    Polity {
        nation: NationId::Estonia,
        system: Electoral::Proportional,
        term_months: 48,
        next: (0, 0),
        parties: &[
            p("ee_isamaa", "Pro Patria", "Isamaa", Family::Conservative, 0.220),
            p("ee_kk", "Safe Home", "Kindel Kodu", Family::Agrarian, 0.136),
            p("ee_rahvarinne", "Popular Front of Estonia", "Rahvarinne", Family::BigTent, 0.123),
            p("ee_mood", "Moderates", "Moodukad", Family::SocialDemocratic, 0.097),
            p("ee_ersp", "Estonian National Independence Party", "Eesti Rahvusliku Soltumatuse Partei", Family::Nationalist, 0.088),
            p("ee_ek", "Estonian Citizen", "Eesti Kodanik", Family::Nationalist, 0.069),
        ],
        ruling: "the Riigikogu",
        pillars: &[],
    },
    // Moldova - 27 February 1994 parliamentary election: the Agrarian
    // Democratic Party 43.2%, the Socialist Party and Unity Movement bloc
    // 22.0%, the Bloc of Peasants and Intellectuals 9.2%, the Christian
    // Democratic Popular Front 7.5%. The 1990 Supreme Soviet election was
    // fought by candidates rather than parties, so 1994 is the first party
    // result, and it is a vote against union with Romania taken after
    // Transnistria was already gone.
    Polity {
        nation: NationId::Moldova,
        system: Electoral::Proportional,
        term_months: 48,
        next: (0, 0),
        parties: &[
            p("md_pdam", "Agrarian Democratic Party", "Partidul Democrat Agrar din Moldova", Family::Agrarian, 0.432),
            p("md_sb", "Socialist Party and Unity Movement bloc", "Blocul Partidul Socialist si Miscarea Unitate-Edinstvo", Family::Communist, 0.220),
            p("md_bti", "Bloc of Peasants and Intellectuals", "Blocul Taranilor si Intelectualilor", Family::Liberal, 0.092),
            p("md_fpcd", "Christian Democratic Popular Front", "Frontul Popular Crestin Democrat", Family::Nationalist, 0.075),
        ],
        ruling: "the Parliament of the Republic of Moldova",
        pillars: &[],
    },
    // China — the Communist Party of China, eight months after Tiananmen. The
    // pillars are the ones Deng actually had to hold: the army he called on in
    // June 1989, the party apparatus, the security ministry, and the coastal
    // provinces whose growth was the regime's remaining argument.
    Polity {
        nation: NationId::China,
        system: Electoral::Proportional,
        term_months: 60,
        next: (0, 0),
        parties: &[
            p("cn_cpc", "Communist Party of China", "Zhongguo Gongchandang", Family::Communist, 1.00),
        ],
        ruling: "the Communist Party of China",
        pillars: &[
            pl(Pillar::Army, "the People's Liberation Army"),
            pl(Pillar::Party, "the Central Committee"),
            pl(Pillar::Security, "the Ministry of State Security"),
            pl(Pillar::Business, "the coastal provinces"),
        ],
    },
    // Japan — House of Representatives, 6 July 1986: LDP 49.4%, JSP 17.2%,
    // Komeito 9.4%, JCP 8.8%, DSP 6.4%. That Diet is the sitting one in January
    // 1990; the next election was six weeks away, on 18 February 1990.
    Polity {
        nation: NationId::Japan,
        system: Electoral::SingleNonTransferable,
        term_months: 48,
        next: (1990, 2),
        parties: &[
            p("jp_ldp", "Liberal Democratic Party", "Jiyu-Minshuto", Family::Conservative, 0.494),
            p("jp_jsp", "Japan Socialist Party", "Nihon Shakaito", Family::SocialDemocratic, 0.172),
            p("jp_komeito", "Komeito", "Komeito", Family::Religious, 0.094),
            p("jp_jcp", "Japanese Communist Party", "Nihon Kyosanto", Family::Communist, 0.088),
            p("jp_dsp", "Democratic Socialist Party", "Minshato", Family::SocialDemocratic, 0.064),
        ],
        ruling: "the National Diet",
        pillars: &[],
    },
    // Germany — Bundestag, 25 January 1987 (Federal Republic): CDU/CSU 44.3%,
    // SPD 37.0%, FDP 9.1%, Greens 8.3%. The next election, 2 December 1990, was
    // the first all-German one.
    Polity {
        nation: NationId::Germany,
        system: Electoral::Proportional,
        term_months: 48,
        next: (1990, 12),
        parties: &[
            p("de_union", "Christian Democratic Union / Christian Social Union", "CDU/CSU", Family::ChristianDemocratic, 0.443),
            p("de_spd", "Social Democratic Party of Germany", "Sozialdemokratische Partei Deutschlands", Family::SocialDemocratic, 0.370),
            p("de_fdp", "Free Democratic Party", "Freie Demokratische Partei", Family::Liberal, 0.091),
            p("de_gruene", "The Greens", "Die Grunen", Family::Green, 0.083),
        ],
        ruling: "the Bundestag",
        pillars: &[],
    },
    // United Kingdom — general election of 11 June 1987: Conservative 42.2%,
    // Labour 30.8%, SDP-Liberal Alliance 22.6%, SNP and Plaid Cymru 2.2%. Next
    // due by mid-1992; it came on 9 April.
    Polity {
        nation: NationId::UK,
        system: Electoral::FirstPastThePost,
        term_months: 60,
        next: (1992, 4),
        parties: &[
            p("uk_con", "Conservative Party", "", Family::Conservative, 0.422),
            p("uk_lab", "Labour Party", "", Family::SocialDemocratic, 0.308),
            p("uk_lib", "Liberal Democrats", "SDP-Liberal Alliance", Family::Liberal, 0.226),
            p("uk_nat", "Scottish National Party and Plaid Cymru", "", Family::Regionalist, 0.022),
        ],
        ruling: "the House of Commons",
        pillars: &[],
    },
    // France — National Assembly first round, 5 June 1988: PS 34.8%, RPR 19.2%,
    // UDF 18.5%, PCF 11.3%, FN 9.7%. The Front National took 9.7% of the vote
    // and one seat, and no party would govern with it: the cordon sanitaire is
    // transcribed here as a pariah flag, not invented.
    Polity {
        nation: NationId::France,
        system: Electoral::TwoRound,
        term_months: 60,
        next: (1993, 3),
        parties: &[
            p("fr_ps", "Socialist Party", "Parti Socialiste", Family::SocialDemocratic, 0.348),
            p("fr_rpr", "Rally for the Republic", "Rassemblement pour la Republique", Family::Conservative, 0.192),
            p("fr_udf", "Union for French Democracy", "Union pour la Democratie Francaise", Family::Liberal, 0.185),
            p("fr_pcf", "French Communist Party", "Parti Communiste Francais", Family::Communist, 0.113),
            pariah("fr_fn", "National Front", "Front National", Family::Nationalist, 0.097),
        ],
        ruling: "the National Assembly",
        pillars: &[],
    },
    // Italy — Chamber of Deputies, 14 June 1987: DC 34.3%, PCI 26.6%, PSI 14.3%,
    // MSI 5.9%, PRI 3.7%, PSDI 3.0%, PLI 2.1%. The largest party and the second
    // largest were both barred from governing together by the conventio ad
    // excludendum, which is exactly why the pentapartito existed and why Italy
    // had forty-eight governments in forty-five years.
    Polity {
        nation: NationId::Italy,
        // No effective threshold before the 1993 reform: the Imperiali quotient
        // and the national remainder pool seated a party on two percent of the
        // vote. That is not a detail — it is the reason a government needed five
        // parties in it, and modelling Italy with the ordinary continental 5%
        // bar quietly deleted the PRI, the PSDI and the PLI from the chamber and
        // with them the whole pentapartito.
        system: Electoral::ProportionalLowBar,
        term_months: 60,
        next: (1992, 4),
        parties: &[
            p("it_dc", "Christian Democracy", "Democrazia Cristiana", Family::ChristianDemocratic, 0.343),
            pariah("it_pci", "Italian Communist Party", "Partito Comunista Italiano", Family::Communist, 0.266),
            p("it_psi", "Italian Socialist Party", "Partito Socialista Italiano", Family::SocialDemocratic, 0.143),
            pariah("it_msi", "Italian Social Movement", "Movimento Sociale Italiano", Family::Nationalist, 0.059),
            p("it_pri", "Italian Republican Party", "Partito Repubblicano Italiano", Family::Liberal, 0.037),
            p("it_psdi", "Italian Democratic Socialist Party", "Partito Socialista Democratico Italiano", Family::SocialDemocratic, 0.030),
            p("it_pli", "Italian Liberal Party", "Partito Liberale Italiano", Family::Liberal, 0.021),
        ],
        ruling: "the Chamber of Deputies",
        pillars: &[],
    },
    // India — 9th Lok Sabha, November 1989: Congress(I) 39.5%, Janata Dal 17.8%,
    // BJP 11.4%, CPI(M) 6.5%. Congress won the most seats and did not take
    // office; V. P. Singh's National Front governed as a minority with the BJP
    // and the Left supporting from outside, and fell in November 1990. This
    // model produces that shape from the arithmetic rather than scripting it.
    Polity {
        nation: NationId::India,
        system: Electoral::FirstPastThePost,
        term_months: 60,
        next: (1994, 11),
        parties: &[
            p("in_inc", "Indian National Congress (I)", "", Family::BigTent, 0.395),
            p("in_jd", "Janata Dal", "", Family::Agrarian, 0.178),
            p("in_bjp", "Bharatiya Janata Party", "", Family::Religious, 0.114),
            p("in_cpm", "Communist Party of India (Marxist)", "", Family::Communist, 0.065),
        ],
        ruling: "the Lok Sabha",
        pillars: &[],
    },
    // Pakistan — National Assembly, 16 November 1988: PPP 38.5%, the Islami
    // Jamhoori Ittehad 30.2%, MQM 5.4%. Benazir Bhutto's government held office
    // in January 1990 and was dismissed by President Ishaq Khan that August.
    Polity {
        nation: NationId::Pakistan,
        system: Electoral::FirstPastThePost,
        term_months: 60,
        next: (1993, 11),
        parties: &[
            p("pk_ppp", "Pakistan Peoples Party", "", Family::SocialDemocratic, 0.385),
            p("pk_iji", "Islami Jamhoori Ittehad", "Islamic Democratic Alliance", Family::Religious, 0.302),
            p("pk_mqm", "Muttahida Qaumi Movement", "", Family::Regionalist, 0.054),
        ],
        ruling: "the National Assembly",
        pillars: &[
            pl(Pillar::Army, "the Pakistan Army"),
            pl(Pillar::Security, "the Inter-Services Intelligence"),
        ],
    },
    // Iraq — the Arab Socialist Ba'ath Party, and behind it the three
    // institutions that kept Saddam Hussein alive: the Republican Guard, the
    // party apparatus, and the intelligence directorate. Real coup attempts in
    // 1990-96 came from exactly these.
    Polity {
        nation: NationId::Iraq,
        system: Electoral::Proportional,
        term_months: 48,
        next: (0, 0),
        parties: &[
            p("iq_baath", "Arab Socialist Ba'ath Party", "Hizb al-Ba'th al-'Arabi al-Ishtiraki", Family::Nationalist, 1.00),
        ],
        ruling: "the Revolutionary Command Council",
        pillars: &[
            pl(Pillar::Army, "the Republican Guard"),
            pl(Pillar::Party, "the Ba'ath Party apparatus"),
            pl(Pillar::Security, "the Mukhabarat"),
        ],
    },
    // Kuwait — the Emir dissolved the National Assembly in 1986 and ruled by
    // decree until 1992. What holds the state is the ruling family, the merchant
    // houses that financed it since before oil, and a small army.
    Polity {
        nation: NationId::Kuwait,
        system: Electoral::Proportional,
        term_months: 48,
        next: (0, 0),
        parties: &[
            p("kw_dem", "Kuwaiti Democratic Forum", "al-Minbar al-Dimuqrati al-Kuwayti", Family::Liberal, 0.55),
            p("kw_islam", "Islamic Constitutional Movement", "al-Haraka al-Dusturiyya al-Islamiyya", Family::Religious, 0.45),
        ],
        ruling: "the House of Al Sabah",
        pillars: &[
            pl(Pillar::Party, "the ruling family"),
            pl(Pillar::Business, "the merchant houses"),
            pl(Pillar::Army, "the Kuwaiti Army"),
        ],
    },
    // Saudi Arabia — no parties, no assembly at all until the Consultative
    // Council of 1992. The bargain is the one struck in 1744 and renewed after
    // 1979: the family rules, the ulema legitimise, the merchants are paid, and
    // the National Guard is kept separate from the regular army on purpose.
    Polity {
        nation: NationId::SaudiArabia,
        system: Electoral::Proportional,
        term_months: 48,
        next: (0, 0),
        parties: &[],
        ruling: "the House of Saud",
        pillars: &[
            pl(Pillar::Party, "the Al Saud family council"),
            pl(Pillar::Clergy, "the ulema"),
            pl(Pillar::Army, "the Saudi Arabian National Guard"),
            pl(Pillar::Business, "the merchant houses"),
        ],
    },
    // Iran — the Islamic Republic eighteen months after Khomeini's death. The
    // Majlis was elected but only from candidates the Guardian Council allowed,
    // so this is a regime with factions rather than an electorate: the Combatant
    // Clergy Association against the Association of Combatant Clerics, which is
    // the split that produced Rafsanjani and later Khatami.
    Polity {
        nation: NationId::Iran,
        system: Electoral::TwoRound,
        term_months: 48,
        next: (0, 0),
        parties: &[
            p("ir_jrm", "Combatant Clergy Association", "Jame'e-ye Rouhaniyat-e Mobarez", Family::Religious, 0.60),
            p("ir_mrm", "Association of Combatant Clerics", "Majma'-e Rouhaniyoun-e Mobarez", Family::SocialDemocratic, 0.40),
        ],
        ruling: "the Office of the Supreme Leader",
        pillars: &[
            pl(Pillar::Clergy, "the seminaries of Qom"),
            pl(Pillar::Army, "the Islamic Revolutionary Guard Corps"),
            pl(Pillar::Security, "the Ministry of Intelligence"),
            pl(Pillar::Business, "the bazaar"),
        ],
    },
    // South Korea — National Assembly, 26 April 1988: DJP 34.0%, RDP 23.8%, PPD
    // 19.3%, NDRP 15.6%. The first assembly under the 1987 constitution and the
    // first in which the ruling party lost its majority. On 22 January 1990,
    // three weeks into the game, the DJP, the RDP and the NDRP merged into the
    // Democratic Liberal Party — a coalition by another name, which is what this
    // model will make of the same numbers.
    Polity {
        nation: NationId::SouthKorea,
        system: Electoral::FirstPastThePost,
        term_months: 48,
        next: (1992, 4),
        parties: &[
            p("kr_djp", "Democratic Justice Party", "Minju Jeongui-dang", Family::Conservative, 0.340),
            p("kr_rdp", "Reunification Democratic Party", "Tongil Minju-dang", Family::Liberal, 0.238),
            p("kr_ppd", "Party for Peace and Democracy", "Pyeonghwa Minju-dang", Family::SocialDemocratic, 0.193),
            p("kr_ndrp", "New Democratic Republican Party", "Sinminju Gonghwa-dang", Family::Conservative, 0.156),
        ],
        ruling: "the National Assembly",
        pillars: &[pl(Pillar::Army, "the Republic of Korea Army")],
    },
    // Poland — the semi-free election of 4 June 1989, in which Solidarity's
    // Citizens' Committee took 99 of the 100 Senate seats and every one of the
    // 161 Sejm seats it was permitted to contest. The Polish United Workers'
    // Party dissolved itself on 29 January 1990, four weeks into the game. Next
    // fully free election: 27 October 1991.
    Polity {
        nation: NationId::Poland,
        system: Electoral::Proportional,
        term_months: 48,
        next: (1991, 10),
        parties: &[
            p("pl_solidarity", "Solidarity Citizens' Committee", "Komitet Obywatelski Solidarnosc", Family::BigTent, 0.60),
            p("pl_sld", "Democratic Left Alliance", "Sojusz Lewicy Demokratycznej", Family::Communist, 0.22),
            p("pl_psl", "Polish People's Party", "Polskie Stronnictwo Ludowe", Family::Agrarian, 0.12),
            p("pl_sd", "Alliance of Democrats", "Stronnictwo Demokratyczne", Family::Liberal, 0.06),
        ],
        ruling: "the Sejm",
        pillars: &[],
    },
    // Brazil — Chamber of Deputies, 15 November 1986: PMDB 48.1%, PFL 17.7%,
    // PDS 6.8%, PDT 5.0%, PT 3.3%. Sarney's PMDB still holds the chamber in
    // January 1990; Fernando Collor, elected that December on the PRN ticket he
    // had built for the purpose, takes office in March. Brazilian presidents
    // govern with a chamber they never control, which is the coalitional
    // presidentialism this table is describing.
    Polity {
        nation: NationId::Brazil,
        system: Electoral::Proportional,
        term_months: 48,
        next: (1990, 10),
        parties: &[
            p("br_pmdb", "Brazilian Democratic Movement Party", "Partido do Movimento Democratico Brasileiro", Family::BigTent, 0.40),
            p("br_pfl", "Liberal Front Party", "Partido da Frente Liberal", Family::Conservative, 0.18),
            p("br_prn", "National Reconstruction Party", "Partido da Reconstrucao Nacional", Family::Liberal, 0.15),
            p("br_pt", "Workers' Party", "Partido dos Trabalhadores", Family::SocialDemocratic, 0.12),
            p("br_pdt", "Democratic Labour Party", "Partido Democratico Trabalhista", Family::SocialDemocratic, 0.08),
            p("br_pds", "Democratic Social Party", "Partido Democratico Social", Family::Conservative, 0.07),
        ],
        ruling: "the Chamber of Deputies",
        pillars: &[],
    },
    // Indonesia — the New Order permitted three organisations and no more.
    // People's Representative Council, 23 April 1987: Golkar 73.2%, PPP 16.0%,
    // PDI 10.9%. Suharto's real constituency was ABRI, which held seats in the
    // assembly by right under dwifungsi.
    Polity {
        nation: NationId::Indonesia,
        system: Electoral::Proportional,
        term_months: 60,
        next: (0, 0),
        parties: &[
            p("id_golkar", "Golkar", "Golongan Karya", Family::BigTent, 0.732),
            p("id_ppp", "United Development Party", "Partai Persatuan Pembangunan", Family::Religious, 0.160),
            p("id_pdi", "Indonesian Democratic Party", "Partai Demokrasi Indonesia", Family::SocialDemocratic, 0.109),
        ],
        ruling: "the New Order",
        pillars: &[
            pl(Pillar::Army, "ABRI"),
            pl(Pillar::Party, "Golkar"),
            pl(Pillar::Business, "the conglomerates"),
        ],
    },
    // Egypt — People's Assembly, 6 April 1987: NDP 69.6%, the Islamic Alliance
    // of the Labour Party, the Liberals and the Muslim Brotherhood 17.0%, New
    // Wafd 10.9%. A hegemonic-party system: the elections were held and the
    // result was known in advance, which is why Mubarak's Egypt is modelled as a
    // regime with pillars rather than an electorate.
    Polity {
        nation: NationId::Egypt,
        system: Electoral::Proportional,
        term_months: 60,
        next: (0, 0),
        parties: &[
            p("eg_ndp", "National Democratic Party", "al-Hizb al-Watani al-Dimuqrati", Family::BigTent, 0.696),
            p("eg_alliance", "Islamic Alliance", "al-Tahaluf al-Islami", Family::Religious, 0.170),
            p("eg_wafd", "New Wafd Party", "Hizb al-Wafd al-Jadid", Family::Liberal, 0.109),
        ],
        ruling: "the National Democratic Party",
        pillars: &[
            pl(Pillar::Army, "the Egyptian Armed Forces"),
            pl(Pillar::Party, "the National Democratic Party"),
            pl(Pillar::Security, "State Security Investigations"),
            pl(Pillar::Clergy, "al-Azhar"),
        ],
    },
    // Spain — Congress of Deputies, 29 October 1989: PSOE 39.6%, PP 25.8%,
    // IU 9.1%, CDS 7.9%, CiU 5.0%, PNV 1.2%. Gonzalez's third term, one seat
    // short of an absolute majority and governing without a coalition.
    Polity {
        nation: NationId::Spain,
        system: Electoral::Proportional,
        term_months: 48,
        next: (1993, 6),
        parties: &[
            p("es_psoe", "Spanish Socialist Workers' Party", "PSOE", Family::SocialDemocratic, 0.396),
            p("es_pp", "People's Party", "PP", Family::Conservative, 0.258),
            p("es_iu", "United Left", "IU", Family::Communist, 0.091),
            p("es_cds", "Democratic and Social Centre", "CDS", Family::Liberal, 0.079),
            p("es_ciu", "Convergence and Union", "CiU", Family::Regionalist, 0.050),
            p("es_pnv", "Basque Nationalist Party", "PNV", Family::Regionalist, 0.012),
        ],
        ruling: "the Cortes Generales",
        pillars: &[],
    },
    // Netherlands — Tweede Kamer, 6 September 1989: CDA 35.3%, PvdA 31.9%,
    // VVD 14.6%, D66 7.9%, GroenLinks 4.1%, SGP 1.9%, GPV 1.2%, CD 0.9%. Lubbers
    // III, the CDA switching partner from the VVD to Labour. The electoral
    // threshold is one seat in a single national district — 0.67% — so the low
    // bar is the closest shape the model has.
    Polity {
        nation: NationId::Netherlands,
        system: Electoral::ProportionalLowBar,
        term_months: 48,
        next: (1994, 5),
        parties: &[
            p("nl_cda", "Christian Democratic Appeal", "Christen-Democratisch Appel", Family::ChristianDemocratic, 0.353),
            p("nl_pvda", "Labour Party", "Partij van de Arbeid", Family::SocialDemocratic, 0.319),
            p("nl_vvd", "People's Party for Freedom and Democracy", "Volkspartij voor Vrijheid en Democratie", Family::Liberal, 0.146),
            p("nl_d66", "Democrats 66", "Democraten 66", Family::Liberal, 0.079),
            p("nl_gl", "Green Left", "GroenLinks", Family::Green, 0.041),
            p("nl_sgp", "Reformed Political Party", "Staatkundig Gereformeerde Partij", Family::Religious, 0.019),
            p("nl_gpv", "Reformed Political League", "Gereformeerd Politiek Verbond", Family::Religious, 0.012),
            pariah("nl_cd", "Centre Democrats", "Centrumdemocraten", Family::Nationalist, 0.009),
        ],
        ruling: "the States General",
        pillars: &[],
    },
    // Belgium — Chamber of Representatives, 13 December 1987: CVP 19.5%,
    // PS 15.7%, SP 14.9%, PVV 11.5%, PRL 9.4%, VU 8.1%, PSC 8.0%, Agalev 4.5%,
    // Ecolo 2.6%, Vlaams Blok 1.9%. Every family here is split in two along the
    // language border, which is why nine parties share three ideologies and why
    // Martens VIII needed five of them. The cordon sanitaire against the Vlaams
    // Blok was formalised in 1989 and held for the rest of the party's life.
    Polity {
        nation: NationId::Belgium,
        system: Electoral::ProportionalLowBar,
        term_months: 48,
        next: (1991, 11),
        parties: &[
            p("be_cvp", "Christian People's Party", "Christelijke Volkspartij", Family::ChristianDemocratic, 0.195),
            p("be_ps", "Socialist Party (francophone)", "Parti Socialiste", Family::SocialDemocratic, 0.157),
            p("be_sp", "Socialist Party (flemish)", "Socialistische Partij", Family::SocialDemocratic, 0.149),
            p("be_pvv", "Party for Freedom and Progress", "Partij voor Vrijheid en Vooruitgang", Family::Liberal, 0.115),
            p("be_prl", "Liberal Reformist Party", "Parti Reformateur Liberal", Family::Liberal, 0.094),
            p("be_vu", "People's Union", "Volksunie", Family::Regionalist, 0.081),
            p("be_psc", "Christian Social Party", "Parti Social Chretien", Family::ChristianDemocratic, 0.080),
            p("be_agalev", "Live Differently", "Agalev", Family::Green, 0.045),
            p("be_ecolo", "Ecologists", "Ecolo", Family::Green, 0.026),
            pariah("be_vb", "Flemish Bloc", "Vlaams Blok", Family::Nationalist, 0.019),
        ],
        ruling: "the Chamber of Representatives",
        pillars: &[],
    },
    // Sweden — Riksdag, 18 September 1988: SAP 43.2%, Moderates 18.3%,
    // Liberals 12.2%, Centre 11.3%, Left Party Communists 5.8%, Greens 5.5%,
    // KDS 2.9%. Carlsson's minority government legislating with the VPK. Terms
    // ran three years until the 1994 reform, and the 4% threshold is the closest
    // to the model's ordinary continental bar.
    Polity {
        nation: NationId::Sweden,
        system: Electoral::Proportional,
        term_months: 36,
        next: (1991, 9),
        parties: &[
            p("se_sap", "Swedish Social Democratic Party", "Sveriges socialdemokratiska arbetareparti", Family::SocialDemocratic, 0.432),
            p("se_m", "Moderate Party", "Moderata samlingspartiet", Family::Conservative, 0.183),
            p("se_fp", "Liberal People's Party", "Folkpartiet liberalerna", Family::Liberal, 0.122),
            p("se_c", "Centre Party", "Centerpartiet", Family::Agrarian, 0.113),
            p("se_vpk", "Left Party Communists", "Vansterpartiet kommunisterna", Family::Communist, 0.058),
            p("se_mp", "Green Party", "Miljopartiet de grona", Family::Green, 0.055),
            p("se_kds", "Christian Democratic Community Party", "Kristdemokratiska samhallspartiet", Family::ChristianDemocratic, 0.029),
        ],
        ruling: "the Riksdag",
        pillars: &[],
    },
    // Switzerland — National Council, 18 October 1987: FDP 22.9%, CVP 19.6%,
    // SPS 18.4%, SVP 11.0%, GPS 4.9%, LdU 4.2%, LPS 2.7%, National Action 2.5%,
    // PdA 0.8%. The four largest have held the Federal Council in a fixed 2:2:2:1
    // since 1959, so an election here moves seats and not the executive. Seats
    // are allocated by canton with no national threshold. The SVP of 1987 is
    // still the farmers' and artisans' party; Blocher's turn came later.
    Polity {
        nation: NationId::Switzerland,
        system: Electoral::ProportionalLowBar,
        term_months: 48,
        next: (1991, 10),
        parties: &[
            p("ch_fdp", "Free Democratic Party", "Freisinnig-Demokratische Partei", Family::Liberal, 0.229),
            p("ch_cvp", "Christian Democratic People's Party", "Christlichdemokratische Volkspartei", Family::ChristianDemocratic, 0.196),
            p("ch_sps", "Social Democratic Party", "Sozialdemokratische Partei der Schweiz", Family::SocialDemocratic, 0.184),
            p("ch_svp", "Swiss People's Party", "Schweizerische Volkspartei", Family::Agrarian, 0.110),
            p("ch_gps", "Green Party", "Grune Partei der Schweiz", Family::Green, 0.049),
            p("ch_ldu", "Ring of Independents", "Landesring der Unabhangigen", Family::Liberal, 0.042),
            p("ch_lps", "Liberal Party", "Liberale Partei der Schweiz", Family::Conservative, 0.027),
            p("ch_na", "National Action", "Nationale Aktion", Family::Nationalist, 0.025),
            p("ch_pda", "Swiss Party of Labour", "Partei der Arbeit der Schweiz", Family::Communist, 0.008),
    // Czechoslovakia — the last election before the game opens is the National
    // Front single list of 23-24 May 1986, 99.9% on a 99.4% turnout, which
    // records nothing. The shares below are the first free election, the
    // Federal Assembly (House of the People) of 8-9 June 1990, federal totals
    // across both republics: Civic Forum took 53.2% in the Czech lands and
    // Public Against Violence 32.5% in Slovakia, which is about 46.6% between
    // them federally. The remaining ~13% went to lists under the 5% federal
    // bar. Elected for a deliberately short two-year term to write a new
    // constitution — it never agreed one, and the state dissolved instead.
    Polity {
        nation: NationId::Czechoslovakia,
        system: Electoral::Proportional,
        term_months: 24,
        next: (1990, 6),
        parties: &[
            p("cs_of", "Civic Forum", "Obcanske forum", Family::BigTent, 0.354),
            p("cs_ksc", "Communist Party of Czechoslovakia", "Komunisticka strana Ceskoslovenska", Family::Communist, 0.135),
            p("cs_vpn", "Public Against Violence", "Verejnost proti nasiliu", Family::BigTent, 0.112),
            p("cs_kdu", "Christian and Democratic Union", "Krestanska a demokraticka unie", Family::ChristianDemocratic, 0.087),
            p("cs_kdh", "Christian Democratic Movement", "Krestanskodemokraticke hnutie", Family::ChristianDemocratic, 0.063),
            p("cs_hsdsms", "Movement for Self-Governing Democracy", "Hnuti za samospravnou demokracii - Spolecnost pro Moravu a Slezsko", Family::Regionalist, 0.054),
            p("cs_sns", "Slovak National Party", "Slovenska narodna strana", Family::Nationalist, 0.035),
            p("cs_egyutteles", "Coexistence", "Egyutteles", Family::Regionalist, 0.028),
        ],
        ruling: "the Federal Assembly",
        pillars: &[],
    },
    // Austria — Nationalrat, 23 November 1986: SPO 43.1%, OVP 41.3%, FPO 9.7%,
    // Greens 4.8%. Vranitzky broke off talks with the FPO the month Haider took
    // it over and called the election; the grand coalition that followed was
    // the standing form of Austrian government. That exclusion is the pariah
    // flag here, and it is what makes SPO-OVP the only arithmetic available.
    // The party was still the Socialist Party of Austria in 1990; it renamed
    // itself Social Democratic in 1991.
    Polity {
        nation: NationId::Austria,
        system: Electoral::Proportional,
        term_months: 48,
        next: (1990, 10),
        parties: &[
            p("at_spo", "Socialist Party of Austria", "Sozialistische Partei Osterreichs", Family::SocialDemocratic, 0.431),
            p("at_ovp", "Austrian People's Party", "Osterreichische Volkspartei", Family::ChristianDemocratic, 0.413),
            pariah("at_fpo", "Freedom Party of Austria", "Freiheitliche Partei Osterreichs", Family::Nationalist, 0.097),
            p("at_gruene", "The Greens", "Die Grune Alternative", Family::Green, 0.048),
        ],
        ruling: "the Nationalrat",
        pillars: &[],
    },
    // Portugal — Assembly of the Republic, 19 July 1987: PSD 50.2%, PS 22.2%,
    // CDU 12.1%, PRD 4.9%, CDS 4.4%. Cavaco Silva's absolute majority, the first
    // any party had won since the Carnation Revolution. D'Hondt by district with
    // no legal threshold. The PSD sat with the Liberal International until 1996
    // despite the name and the office it held.
    Polity {
        nation: NationId::Portugal,
        system: Electoral::ProportionalLowBar,
        term_months: 48,
        next: (1991, 10),
        parties: &[
            p("pt_psd", "Social Democratic Party", "Partido Social Democrata", Family::Liberal, 0.502),
            p("pt_ps", "Socialist Party", "Partido Socialista", Family::SocialDemocratic, 0.222),
            p("pt_cdu", "Unitary Democratic Coalition", "Coligacao Democratica Unitaria", Family::Communist, 0.121),
            p("pt_prd", "Democratic Renewal Party", "Partido Renovador Democratico", Family::BigTent, 0.049),
            p("pt_cds", "Democratic and Social Centre", "Centro Democratico e Social", Family::ChristianDemocratic, 0.044),
        ],
        ruling: "the Assembly of the Republic",
        pillars: &[],
    },
    // Greece — Hellenic Parliament, 5 November 1989: New Democracy 46.2%,
    // PASOK 40.7%, Synaspismos 11.0%, DIANA 0.7%. The second of three elections
    // in ten months: simple proportional representation had been restored in
    // 1989 precisely so that no party could win outright, and none did. What
    // sat in January 1990 was Zolotas's ecumenical cabinet, and the third
    // election came on 8 April.
    Polity {
        nation: NationId::Greece,
        system: Electoral::Proportional,
        term_months: 48,
        next: (1990, 4),
        parties: &[
            p("gr_nd", "New Democracy", "Nea Dimokratia", Family::Conservative, 0.462),
            p("gr_pasok", "Panhellenic Socialist Movement", "Panellinio Sosialistiko Kinima", Family::SocialDemocratic, 0.407),
            p("gr_syn", "Coalition of the Left and Progress", "Synaspismos tis Aristeras kai tis Proodou", Family::Communist, 0.110),
            p("gr_diana", "Democratic Renewal", "Dimokratiki Ananeosi", Family::Liberal, 0.007),
        ],
        ruling: "the Hellenic Parliament",
        pillars: &[],
    },
    // Denmark — Folketing, 10 May 1988: Social Democrats 29.8%, Conservatives
    // 19.3%, SF 13.0%, Venstre 11.8%, Progress Party 9.0%, Radikale 5.6%,
    // Centre Democrats 4.7%, Christian People's Party 2.0%. Schluter's fourth
    // cabinet, formed after the footnote-policy crisis over nuclear-armed port
    // visits forced the election. Threshold 2%. SF is left-socialist rather
    // than communist — it split from the DKP in 1959 — but the leftmost family
    // the model carries is the closest place to put it.
    Polity {
        nation: NationId::Denmark,
        system: Electoral::ProportionalLowBar,
        term_months: 48,
        next: (1990, 12),
        parties: &[
            p("dk_s", "Social Democrats", "Socialdemokratiet", Family::SocialDemocratic, 0.298),
            p("dk_kf", "Conservative People's Party", "Det Konservative Folkeparti", Family::Conservative, 0.193),
            p("dk_sf", "Socialist People's Party", "Socialistisk Folkeparti", Family::Communist, 0.130),
            p("dk_v", "Venstre, Liberal Party of Denmark", "Venstre", Family::Liberal, 0.118),
            p("dk_frp", "Progress Party", "Fremskridtspartiet", Family::Nationalist, 0.090),
            p("dk_rv", "Danish Social Liberal Party", "Det Radikale Venstre", Family::Liberal, 0.056),
            p("dk_cd", "Centre Democrats", "Centrum-Demokraterne", Family::ChristianDemocratic, 0.047),
            p("dk_krf", "Christian People's Party", "Kristeligt Folkeparti", Family::ChristianDemocratic, 0.020),
        ],
        ruling: "the Folketing",
        pillars: &[],
    },
    // Norway — Storting, 11 September 1989: Labour 34.3%, Conservatives 22.2%,
    // Progress Party 13.0%, Socialist Left 10.1%, Christian Democrats 8.5%,
    // Centre 6.5%, Liberals 3.2%. Syse's three-party centre-right minority took
    // office in October and broke up in November 1990 over the EEA. The Storting
    // cannot be dissolved early: the term is a fixed four years, which is why
    // the next date is 1993 however the government fares.
    Polity {
        nation: NationId::Norway,
        system: Electoral::Proportional,
        term_months: 48,
        next: (1993, 9),
        parties: &[
            p("no_ap", "Labour Party", "Arbeiderpartiet", Family::SocialDemocratic, 0.343),
            p("no_h", "Conservative Party", "Hoyre", Family::Conservative, 0.222),
            p("no_frp", "Progress Party", "Fremskrittspartiet", Family::Nationalist, 0.130),
            p("no_sv", "Socialist Left Party", "Sosialistisk Venstreparti", Family::Communist, 0.101),
            p("no_krf", "Christian Democratic Party", "Kristelig Folkeparti", Family::ChristianDemocratic, 0.085),
            p("no_sp", "Centre Party", "Senterpartiet", Family::Agrarian, 0.065),
            p("no_v", "Liberal Party", "Venstre", Family::Liberal, 0.032),
        ],
        ruling: "the Storting",
        pillars: &[],
    },
    // Finland — Eduskunta, 15-16 March 1987: SDP 24.1%, National Coalition
    // 23.1%, Centre 17.6%, SKDL 9.4%, Rural Party 6.3%, Swedish People's Party
    // 5.3%, Greens 4.0%, Democratic Alternative 4.2%, Christian League 2.6%.
    // Holkeri's "red-blue" cabinet put the Conservatives back in office after
    // twenty years out, which Kekkonen's reading of the 1948 treaty had made
    // impossible. D'Hondt by district, no national threshold.
    Polity {
        nation: NationId::Finland,
        system: Electoral::ProportionalLowBar,
        term_months: 48,
        next: (1991, 3),
        parties: &[
            p("fi_sdp", "Social Democratic Party of Finland", "Suomen Sosialidemokraattinen Puolue", Family::SocialDemocratic, 0.241),
            p("fi_kok", "National Coalition Party", "Kansallinen Kokoomus", Family::Conservative, 0.231),
            p("fi_kesk", "Centre Party", "Suomen Keskusta", Family::Agrarian, 0.176),
            p("fi_skdl", "Finnish People's Democratic League", "Suomen Kansan Demokraattinen Liitto", Family::Communist, 0.094),
            p("fi_smp", "Finnish Rural Party", "Suomen Maaseudun Puolue", Family::Nationalist, 0.063),
            p("fi_rkp", "Swedish People's Party", "Svenska folkpartiet i Finland", Family::Regionalist, 0.053),
            p("fi_deva", "Democratic Alternative", "Demokraattinen Vaihtoehto", Family::Communist, 0.042),
            p("fi_vihr", "Green League", "Vihrea Liitto", Family::Green, 0.040),
            p("fi_skl", "Christian League", "Suomen Kristillinen Liitto", Family::ChristianDemocratic, 0.026),
        ],
        ruling: "the Eduskunta",
        pillars: &[],
    },
    // Ireland — Dail Eireann, 15 June 1989: Fianna Fail 44.1%, Fine Gael 29.3%,
    // Labour 9.5%, Progressive Democrats 5.5%, Workers' Party 5.0%, Greens 1.5%,
    // Sinn Fein 1.2%. Haughey called the election to win a majority, lost seats,
    // and entered the first coalition Fianna Fail had ever accepted. Single
    // transferable vote in multi-seat constituencies, no threshold. Sinn Fein
    // was excluded by every other party and off the airwaves entirely under the
    // Section 31 broadcasting ban, which is what the pariah flag records.
    Polity {
        nation: NationId::Ireland,
        system: Electoral::ProportionalLowBar,
        term_months: 60,
        next: (1992, 11),
        parties: &[
            p("ie_ff", "Fianna Fail", "Fianna Fail", Family::BigTent, 0.441),
            p("ie_fg", "Fine Gael", "Fine Gael", Family::ChristianDemocratic, 0.293),
            p("ie_lab", "Labour Party", "Pairti Lucht Oibre", Family::SocialDemocratic, 0.095),
            p("ie_pd", "Progressive Democrats", "An Pairti Daonlathach", Family::Liberal, 0.055),
            p("ie_wp", "Workers' Party", "Pairti na nOibrithe", Family::Communist, 0.050),
            p("ie_green", "Green Party", "Comhaontas Glas", Family::Green, 0.015),
            pariah("ie_sf", "Sinn Fein", "Sinn Fein", Family::Nationalist, 0.012),
        ],
        ruling: "the Dail Eireann",
        pillars: &[],
    // East Germany — Volkskammer, 18 March 1990, the only free election the GDR
    // ever held. Alliance for Germany: CDU 40.8%, DSU 6.3%, Democratic Awakening
    // 0.9%; SPD 21.9%; PDS 16.4%; League of Free Democrats 5.3%; Alliance 90
    // 2.9%; Democratic Farmers' Party 2.2%; Green Party with the Independent
    // Women's Association 2.0%. No threshold at all, which is why twelve lists
    // took seats. The last election before the game opens is the local ballot of
    // 7 May 1989, whose falsified 98.85% was itself the trigger for the autumn.
    // Modelled as an ordinary electorate because on that day it was one; the
    // accession of 3 October 1990 is not scripted here, and eastgermany.json
    // explains why and what carries the pressure instead.
    Polity {
        nation: NationId::EastGermany,
        system: Electoral::ProportionalLowBar,
        term_months: 48,
        next: (1990, 3),
        parties: &[
            p("dd_cdu", "Christian Democratic Union", "Christlich-Demokratische Union", Family::ChristianDemocratic, 0.408),
            p("dd_spd", "Social Democratic Party", "Sozialdemokratische Partei", Family::SocialDemocratic, 0.219),
            p("dd_pds", "Party of Democratic Socialism", "Partei des Demokratischen Sozialismus", Family::Communist, 0.164),
            p("dd_dsu", "German Social Union", "Deutsche Soziale Union", Family::Conservative, 0.063),
            p("dd_bfd", "League of Free Democrats", "Bund Freier Demokraten", Family::Liberal, 0.053),
            p("dd_b90", "Alliance 90", "Bundnis 90", Family::Green, 0.029),
            p("dd_dbd", "Democratic Farmers' Party", "Demokratische Bauernpartei Deutschlands", Family::Agrarian, 0.022),
            p("dd_gruene", "Green Party", "Grune Partei", Family::Green, 0.020),
        ],
        ruling: "the Volkskammer",
        pillars: &[],
    },
    // Hungary — National Assembly, 25 March and 8 April 1990, party lists:
    // MDF 24.7%, SZDSZ 21.4%, FKgP 11.7%, MSZP 10.9%, Fidesz 9.0%, KDNP 6.5%,
    // MSZMP 3.7%, Agrarian Alliance 3.1%. The last election before the game
    // opens, June 1985, was single-list. The real system was mixed: 176
    // single-member seats decided in two rounds alongside county and national
    // lists with a 4% bar, and TwoRound is entered because the majoritarian
    // half is what turned MDF's quarter of the vote into 42.5% of the seats.
    Polity {
        nation: NationId::Hungary,
        system: Electoral::TwoRound,
        term_months: 48,
        next: (1990, 3),
        parties: &[
            p("hu_mdf", "Hungarian Democratic Forum", "Magyar Demokrata Forum", Family::ChristianDemocratic, 0.247),
            p("hu_szdsz", "Alliance of Free Democrats", "Szabad Demokratak Szovetsege", Family::Liberal, 0.214),
            p("hu_fkgp", "Independent Smallholders' Party", "Fuggetlen Kisgazdapart", Family::Agrarian, 0.117),
            p("hu_mszp", "Hungarian Socialist Party", "Magyar Szocialista Part", Family::SocialDemocratic, 0.109),
            p("hu_fidesz", "Federation of Young Democrats", "Fiatal Demokratak Szovetsege", Family::Liberal, 0.090),
            p("hu_kdnp", "Christian Democratic People's Party", "Keresztenydemokrata Neppart", Family::ChristianDemocratic, 0.065),
            p("hu_mszmp", "Hungarian Socialist Workers' Party", "Magyar Szocialista Munkaspart", Family::Communist, 0.037),
            p("hu_asz", "Agrarian Alliance", "Agrarszovetseg", Family::Agrarian, 0.031),
        ],
        ruling: "the National Assembly",
        pillars: &[],
    },
    // Romania — Chamber of Deputies, 20 May 1990: FSN 66.3%, UDMR 7.2%, PNL
    // 6.4%, Ecological Movement 2.6%, PNTCD 2.6%, AUR 2.1%, PSDR 0.5%. No
    // threshold, hence a chamber with sixteen parties in it and one of them
    // holding two-thirds. The National Salvation Front had promised on 23
    // December 1989 not to contest the election and reversed that on 6
    // February 1990; the opposition had no organisation outside the cities and
    // no access to state television. Free enough to model as an electorate,
    // which is why authoritarianism sits at 0.42 rather than above the ceiling.
    Polity {
        nation: NationId::Romania,
        system: Electoral::ProportionalLowBar,
        term_months: 48,
        next: (1990, 5),
        parties: &[
            p("ro_fsn", "National Salvation Front", "Frontul Salvarii Nationale", Family::BigTent, 0.663),
            p("ro_udmr", "Democratic Union of Hungarians in Romania", "Uniunea Democrata Maghiara din Romania", Family::Regionalist, 0.072),
            p("ro_pnl", "National Liberal Party", "Partidul National Liberal", Family::Liberal, 0.064),
            p("ro_mer", "Ecological Movement of Romania", "Miscarea Ecologista din Romania", Family::Green, 0.026),
            p("ro_pntcd", "Christian Democratic National Peasants' Party", "Partidul National Taranesc Crestin Democrat", Family::ChristianDemocratic, 0.026),
            p("ro_aur", "Romanian National Unity Party", "Partidul Unitatii Nationale Romane", Family::Nationalist, 0.021),
            p("ro_psdr", "Romanian Social Democratic Party", "Partidul Social Democrat Roman", Family::SocialDemocratic, 0.005),
        ],
        ruling: "the Chamber of Deputies",
        pillars: &[pl(Pillar::Army, "the Romanian Army")],
    },
    // Bulgaria — Grand National Assembly, 10 and 17 June 1990, proportional
    // half: BSP 47.2%, Union of Democratic Forces 36.2%, Agrarian Union 8.0%,
    // Movement for Rights and Freedoms 6.0%. The renamed communist party won,
    // the only one in the region that did, and then could not govern: Lukanov
    // resigned in November 1990 after a general strike. The 4% bar of the 1990
    // ballot is modelled with the ordinary 5% shape.
    Polity {
        nation: NationId::Bulgaria,
        system: Electoral::Proportional,
        term_months: 48,
        next: (1990, 6),
        parties: &[
            p("bg_bsp", "Bulgarian Socialist Party", "Balgarska sotsialisticheska partiya", Family::Communist, 0.472),
            p("bg_sds", "Union of Democratic Forces", "Sayuz na demokratichnite sili", Family::BigTent, 0.362),
            p("bg_bzns", "Bulgarian Agrarian National Union", "Balgarski zemedelski naroden sayuz", Family::Agrarian, 0.080),
            p("bg_dps", "Movement for Rights and Freedoms", "Dvizhenie za prava i svobodi", Family::Regionalist, 0.060),
        ],
        ruling: "the Grand National Assembly",
        pillars: &[],
    },
    // Albania — the People's Assembly election of 1 February 1987 returned the
    // Democratic Front's single list with 100% of the vote on a turnout of
    // 100%, and there is nothing else before January 1990. Opposition parties
    // became legal only on 11 December 1990, so the regime holds no election
    // the model should fire from the start state and next is (0, 0). The table
    // below is the 31 March 1991 result, the first contested one, held live for
    // if and when the regime opens: PPSh 56.2%, Democratic Party 38.7%, Omonia
    // 0.7%. The pillars are the three institutions that actually held Ramiz
    // Alia up, and the Sigurimi is named because "the security services" is not
    // a thing that removes a government.
    Polity {
        nation: NationId::Albania,
        system: Electoral::Proportional,
        term_months: 48,
        next: (0, 0),
        parties: &[
            p("al_ppsh", "Party of Labour of Albania", "Partia e Punes e Shqiperise", Family::Communist, 0.562),
            p("al_pd", "Democratic Party of Albania", "Partia Demokratike e Shqiperise", Family::Liberal, 0.387),
            p("al_omonia", "Omonia", "Omonoia", Family::Regionalist, 0.007),
        ],
        ruling: "the Party of Labour of Albania",
        pillars: &[
            pl(Pillar::Party, "the Party of Labour of Albania"),
            pl(Pillar::Army, "the Albanian People's Army"),
            pl(Pillar::Security, "the Sigurimi"),
        ],
    },
    // Israel — Knesset, 1 November 1988: Likud 31.1%, the Alignment 30.0%, Shas
    // 4.7%, Agudat Yisrael 4.5%, Ratz 4.3%, the National Religious Party 3.9%,
    // Tehiya 3.1%. A 1% threshold and no party ever near half the seats. The
    // national unity government of 1988 fell on 15 March 1990 on a motion of no
    // confidence — the only one ever carried in Israeli history — and that is
    // the shape the arithmetic here produces without being told to.
    Polity {
        nation: NationId::Israel,
        system: Electoral::ProportionalLowBar,
        term_months: 48,
        next: (1992, 11),
        parties: &[
            p("il_likud", "Likud", "Likud", Family::Conservative, 0.311),
            p("il_labour", "Alignment", "Ma'arach", Family::SocialDemocratic, 0.300),
            p("il_shas", "Shas", "Shas", Family::Religious, 0.047),
            p("il_agudat", "Agudat Yisrael", "Agudat Yisrael", Family::Religious, 0.045),
            p("il_ratz", "Citizens' Rights Movement", "Ratz", Family::Liberal, 0.043),
            p("il_mafdal", "National Religious Party", "Mafdal", Family::Religious, 0.039),
            p("il_tehiya", "Tehiya", "Tehiya", Family::Nationalist, 0.031),
        ],
        ruling: "the Knesset",
        pillars: &[],
    },
    // Syria — People's Council, 10-11 February 1986. The National Progressive
    // Front, which the Ba'ath created in 1972 and has led ever since, took all
    // 195 seats; the Ba'ath itself took 129 of them. The Front allocates seats
    // rather than contesting them, so the sub-shares below are the documented
    // Ba'athist two-thirds and an even division of the remainder among the four
    // allied parties. No election is due because none can change anything: the
    // constitution of 1973 names the Ba'ath as the leading party of state.
    Polity {
        nation: NationId::Syria,
        system: Electoral::Proportional,
        term_months: 48,
        next: (0, 0),
        parties: &[
            p("sy_baath", "Arab Socialist Ba'ath Party", "Hizb al-Ba'th al-'Arabi al-Ishtiraki", Family::Nationalist, 0.66),
            p("sy_ascp", "Syrian Communist Party", "al-Hizb al-Shuyu'i al-Suri", Family::Communist, 0.09),
            p("sy_asu", "Arab Socialist Union", "al-Ittihad al-Ishtiraki al-'Arabi", Family::Nationalist, 0.09),
            p("sy_sum", "Socialist Unionist Movement", "al-Haraka al-Ishtirakiyya al-Wahdawiyya", Family::Nationalist, 0.09),
            p("sy_asp", "Arab Socialist Party", "al-Hizb al-Ishtiraki al-'Arabi", Family::SocialDemocratic, 0.07),
        ],
        ruling: "the Regional Command of the Ba'ath Party",
        pillars: &[
            pl(Pillar::Army, "the Republican Guard and the Special Forces"),
            pl(Pillar::Party, "the Ba'ath Party Regional Command"),
            pl(Pillar::Security, "the Mukhabarat directorates"),
            pl(Pillar::Business, "the Damascus and Aleppo merchant families"),
        ],
    },
    // Jordan — House of Representatives, 8 November 1989: the first election in
    // twenty-two years, held because the dinar collapsed and Ma'an rioted. Parties
    // were still banned, so every candidate ran as an independent and the results
    // are counted in blocs, which is how they were counted at the time: Muslim
    // Brotherhood 22 seats of 80, independent Islamists 12, leftists and Arab
    // nationalists 13, tribal and pro-palace independents the remaining 33. Parties
    // were legalised in 1992 and the next election came on 8 November 1993.
    Polity {
        nation: NationId::Jordan,
        system: Electoral::FirstPastThePost,
        term_months: 48,
        next: (1993, 11),
        parties: &[
            p("jo_tribal", "Tribal and Independent Loyalists", "", Family::BigTent, 0.4125),
            p("jo_ikhwan", "Muslim Brotherhood", "al-Ikhwan al-Muslimun", Family::Religious, 0.2750),
            p("jo_islamists", "Independent Islamists", "", Family::Religious, 0.1500),
            p("jo_left", "Leftist and Arab Nationalist Bloc", "", Family::SocialDemocratic, 0.1625),
        ],
        ruling: "the House of Representatives",
        pillars: &[
            pl(Pillar::Army, "the Jordanian Armed Forces"),
            pl(Pillar::Security, "the General Intelligence Directorate"),
        ],
    },
    // Lebanon — the last parliamentary election was in April 1972. The Chamber
    // elected then was never renewed: the civil war began in 1975 and members who
    // died were replaced by appointment. The 1972 chamber was dominated by the
    // za'im notables rather than by parties, which is the 0.65 below. Amal was
    // founded in 1974 and Hezbollah in 1985, so both carry their result at the
    // first election either contested — 8 seats of 128 each, in August-September
    // 1992 — under the convention this table already uses elsewhere. The Taif
    // Agreement of 22 October 1989 is what makes that 1992 election possible.
    Polity {
        nation: NationId::Lebanon,
        system: Electoral::Proportional,
        term_months: 48,
        next: (1992, 8),
        parties: &[
            p("lb_independents", "Independent Notables", "", Family::BigTent, 0.65),
            p("lb_kataeb", "Kataeb Party", "Hizb al-Kata'ib al-Lubnaniyya", Family::ChristianDemocratic, 0.09),
            p("lb_nlp", "National Liberal Party", "Hizb al-Wataniyyin al-Ahrar", Family::Conservative, 0.08),
            p("lb_psp", "Progressive Socialist Party", "al-Hizb al-Taqaddumi al-Ishtiraki", Family::SocialDemocratic, 0.06),
            p("lb_amal", "Amal Movement", "Harakat Amal", Family::Religious, 0.06),
            p("lb_hezbollah", "Hezbollah", "Hizb Allah", Family::Religious, 0.06),
        ],
        ruling: "the Chamber of Deputies",
        pillars: &[pl(Pillar::Army, "the Lebanese Armed Forces")],
    },
    // United Arab Emirates — a federation of seven hereditary monarchies whose
    // Supreme Council is the seven rulers themselves. No national election has
    // ever been held; the Federal National Council was wholly appointed until
    // 2006 and parties are prohibited. Power sits where the oil is: Abu Dhabi
    // under Zayed bin Sultan Al Nahyan, president since the union in 1971, with
    // Dubai under Al Maktoum holding the vice-presidency and the trade.
    Polity {
        nation: NationId::UAE,
        system: Electoral::Proportional,
        term_months: 48,
        next: (0, 0),
        parties: &[],
        ruling: "the Federal Supreme Council",
        pillars: &[
            pl(Pillar::Party, "the Al Nahyan and Al Maktoum ruling families"),
            pl(Pillar::Army, "the Union Defence Force"),
            pl(Pillar::Business, "the Dubai merchant houses"),
        ],
    },
    // Qatar — no elections of any kind. The Advisory Council created in 1972 was
    // appointed and its term was simply extended by decree. Khalifa bin Hamad Al
    // Thani took power from his cousin in a bloodless coup in February 1972 and
    // lost it to his own son in another in June 1995, which is the risk this
    // block is describing: in Qatar the threat to a ruler is the family.
    Polity {
        nation: NationId::Qatar,
        system: Electoral::Proportional,
        term_months: 48,
        next: (0, 0),
        parties: &[],
        ruling: "the House of Thani",
        pillars: &[
            pl(Pillar::Party, "the Al Thani family council"),
            pl(Pillar::Army, "the Qatar Armed Forces"),
            pl(Pillar::Clergy, "the ulema"),
            pl(Pillar::Business, "the merchant houses"),
        ],
    },
    // Oman — Sultan Qaboos bin Said ruled by decree, holding the offices of prime
    // minister, defence, foreign affairs and finance at once. There was no
    // constitution until the Basic Law of 1996 and no election ever: the State
    // Consultative Council of 1981 was appointed, and the Majlis al-Shura that
    // replaced it in November 1991 was indirectly selected from tribal nominees.
    // The Ibadi imamate of the interior is listed as a pillar because it is the
    // one rival source of legitimacy Oman has, and it fought for it until 1959.
    Polity {
        nation: NationId::Oman,
        system: Electoral::Proportional,
        term_months: 48,
        next: (0, 0),
        parties: &[],
        ruling: "the Sultan",
        pillars: &[
            pl(Pillar::Army, "the Sultan's Armed Forces"),
            pl(Pillar::Security, "the Internal Security Service"),
            pl(Pillar::Clergy, "the Ibadi ulema of the interior"),
            pl(Pillar::Business, "the Muscat merchant houses"),
        ],
    },
    // Yemen — the transitional House of Representatives of the unified republic,
    // seated 22 May 1990 by merging the north's 159-member Consultative Assembly
    // with the south's 111-member Supreme People's Council and 31 presidential
    // appointees. The two shares below are those two chambers as fractions of the
    // 301 seats; the appointees are left out because they were not a party. The
    // first election of the unified state was held on 27 April 1993, and it was
    // real. See yemen.json for why this nation ships already unified.
    Polity {
        nation: NationId::Yemen,
        system: Electoral::FirstPastThePost,
        term_months: 48,
        next: (1993, 4),
        parties: &[
            p("ye_gpc", "General People's Congress", "al-Mu'tamar al-Sha'bi al-'Amm", Family::BigTent, 0.528),
            p("ye_ysp", "Yemeni Socialist Party", "al-Hizb al-Ishtiraki al-Yamani", Family::Communist, 0.369),
        ],
        ruling: "the House of Representatives",
        pillars: &[
            pl(Pillar::Army, "the northern and southern armies, never merged"),
            pl(Pillar::Party, "the Hashid tribal confederation"),
            pl(Pillar::Security, "the Political Security Organisation"),
        ],
    },
    // Bahrain — the last election was on 7 December 1973, for the National
    // Assembly created by the 1973 constitution: the People's Bloc took 8 of the
    // 30 elected seats, the Religious Bloc 6, and independents 16. The Emir
    // dissolved the Assembly on 26 August 1975 when it refused to pass the State
    // Security Law, suspended the constitutional articles requiring elections,
    // and ruled by decree. Nothing is due; the Assembly does not return until
    // 2002.
    Polity {
        nation: NationId::Bahrain,
        system: Electoral::FirstPastThePost,
        term_months: 48,
        next: (0, 0),
        parties: &[
            p("bh_independents", "Independents", "", Family::BigTent, 0.533),
            p("bh_peoples", "People's Bloc", "al-Kutla al-Sha'biyya", Family::SocialDemocratic, 0.267),
            p("bh_religious", "Religious Bloc", "al-Kutla al-Diniyya", Family::Religious, 0.200),
        ],
        ruling: "the House of Khalifa",
        pillars: &[
            pl(Pillar::Party, "the Al Khalifa family council"),
            pl(Pillar::Army, "the Bahrain Defence Force"),
            pl(Pillar::Security, "the State Security Directorate"),
            pl(Pillar::Business, "the merchant houses"),
        ],
    },
    // Turkey — Grand National Assembly, 29 November 1987: ANAP 36.3%, SHP 24.7%,
    // DYP 19.1%, DSP 8.5%, Welfare 7.2%. The 10% national threshold, written
    // into the 1982 constitution by the generals who had just left office, gave
    // Ozal's 36% of the vote 65% of the seats.
    Polity {
        nation: NationId::Turkey,
        system: Electoral::ProportionalHighBar,
        term_months: 60,
        next: (1991, 10),
        parties: &[
            p("tr_anap", "Motherland Party", "Anavatan Partisi", Family::Conservative, 0.363),
            p("tr_shp", "Social Democratic Populist Party", "Sosyaldemokrat Halkci Parti", Family::SocialDemocratic, 0.247),
            p("tr_dyp", "True Path Party", "Dogru Yol Partisi", Family::Conservative, 0.191),
            p("tr_dsp", "Democratic Left Party", "Demokratik Sol Parti", Family::SocialDemocratic, 0.085),
            p("tr_rp", "Welfare Party", "Refah Partisi", Family::Religious, 0.072),
        ],
        ruling: "the Grand National Assembly",
        pillars: &[pl(Pillar::Army, "the Turkish General Staff")],
    },
    // Nigeria — Babangida's Armed Forces Ruling Council, five years into a
    // transition programme that kept slipping. Two parties existed in January
    // 1990 and both had been created by military decree in 1989, with their
    // manifestos written for them. Their shares are the 12 June 1993
    // presidential result, the election that was annulled.
    Polity {
        nation: NationId::Nigeria,
        system: Electoral::FirstPastThePost,
        term_months: 48,
        next: (0, 0),
        parties: &[
            p("ng_sdp", "Social Democratic Party", "", Family::SocialDemocratic, 0.583),
            p("ng_nrc", "National Republican Convention", "", Family::Conservative, 0.417),
        ],
        ruling: "the Armed Forces Ruling Council",
        pillars: &[
            pl(Pillar::Army, "the Nigerian Army"),
            pl(Pillar::Security, "the State Security Service"),
            pl(Pillar::Business, "the oil bureaucracy"),
        ],
    },
    // Vietnam — the Communist Party of Vietnam, four years into doi moi and the
    // year Soviet money stops arriving. No competing organisation is legal.
    Polity {
        nation: NationId::Vietnam,
        system: Electoral::Proportional,
        term_months: 60,
        next: (0, 0),
        parties: &[
            p("vn_cpv", "Communist Party of Vietnam", "Dang Cong san Viet Nam", Family::Communist, 1.00),
        ],
        ruling: "the Communist Party of Vietnam",
        pillars: &[
            pl(Pillar::Army, "the People's Army of Vietnam"),
            pl(Pillar::Party, "the Politburo"),
            pl(Pillar::Security, "the Ministry of Public Security"),
        ],
    },
    // Yugoslavia — the League of Communists of Yugoslavia, whose 14th
    // Extraordinary Congress broke up on 22 January 1990, three weeks into the
    // game, when the Slovene delegation walked out and the Croats followed. The
    // federation's remaining pillar after that was the JNA. The parties listed
    // are the republican fronts that won the 1990 elections, and they only
    // become live if a federal Yugoslavia somehow opens up rather than breaking.
    Polity {
        nation: NationId::Yugoslavia,
        system: Electoral::Proportional,
        term_months: 48,
        next: (0, 0),
        parties: &[
            p("yu_skj", "League of Communists of Yugoslavia", "Savez komunista Jugoslavije", Family::Communist, 0.55),
            p("yu_fronts", "the republican national fronts", "", Family::Nationalist, 0.45),
        ],
        ruling: "the League of Communists of Yugoslavia",
        pillars: &[
            pl(Pillar::Army, "the Yugoslav People's Army"),
            pl(Pillar::Party, "the League of Communists"),
            pl(Pillar::Security, "the State Security Service"),
        ],
    },
    // Serbia — 9 December 1990, the first multiparty election in Serbia since
    // 1938: Milosevic's Socialist Party of Serbia 46.1%, the Serbian Renewal
    // Movement 15.8%, the Democratic Party 7.4%. Elections were held and the
    // state television was not neutral, which is why Serbia is modelled with
    // pillars as well as parties.
    Polity {
        nation: NationId::Serbia,
        system: Electoral::FirstPastThePost,
        term_months: 48,
        next: (0, 0),
        parties: &[
            p("rs_sps", "Socialist Party of Serbia", "Socijalisticka partija Srbije", Family::Nationalist, 0.461),
            p("rs_spo", "Serbian Renewal Movement", "Srpski pokret obnove", Family::Nationalist, 0.158),
            p("rs_ds", "Democratic Party", "Demokratska stranka", Family::Liberal, 0.074),
        ],
        ruling: "the Socialist Party of Serbia",
        pillars: &[
            pl(Pillar::Army, "the Yugoslav People's Army"),
            pl(Pillar::Party, "the Socialist Party"),
            pl(Pillar::Security, "the State Security Service"),
        ],
    },
    // Croatia — 22 April 1990: Tudjman's Croatian Democratic Union 41.9%, the
    // reformed League of Communists 35.0%, the Coalition of National Accord
    // 15.3%. First round, first free election since 1938.
    Polity {
        nation: NationId::Croatia,
        system: Electoral::TwoRound,
        term_months: 48,
        next: (0, 0),
        parties: &[
            p("hr_hdz", "Croatian Democratic Union", "Hrvatska demokratska zajednica", Family::Nationalist, 0.419),
            p("hr_sdp", "Party of Democratic Reform", "Stranka demokratskih promjena", Family::SocialDemocratic, 0.350),
            p("hr_kns", "Coalition of National Accord", "Koalicija narodnog sporazuma", Family::Liberal, 0.153),
        ],
        ruling: "the Sabor",
        pillars: &[],
    },
    // Slovenia — 8 April 1990: the DEMOS opposition coalition 54.0%, the Party
    // of Democratic Renewal 17.3%, the Liberal Democrats 14.5%. DEMOS was six
    // parties in a trenchcoat and came apart within two years of winning, which
    // is what a big-tent coalition costs to hold.
    Polity {
        nation: NationId::Slovenia,
        system: Electoral::Proportional,
        term_months: 48,
        next: (0, 0),
        parties: &[
            p("si_demos", "DEMOS", "Demokraticna opozicija Slovenije", Family::BigTent, 0.540),
            p("si_sdp", "Party of Democratic Renewal", "Stranka demokraticne prenove", Family::SocialDemocratic, 0.173),
            p("si_ldp", "Liberal Democratic Party", "Liberalno demokratska stranka", Family::Liberal, 0.145),
        ],
        ruling: "the National Assembly",
        pillars: &[],
    },
    // Bosnia and Herzegovina — 18 November 1990. The three national parties took
    // the vote almost exactly in proportion to the census: the Party of
    // Democratic Action 35.8%, the Serbian Democratic Party 30.0%, the Croatian
    // Democratic Union of BiH 18.4%, the reformed communists 6.0%. They then
    // formed a government together, because the arithmetic left no alternative,
    // and it held for about a year. There is no more expensive coalition in this
    // model, and there was none in life either.
    Polity {
        nation: NationId::Bosnia,
        system: Electoral::Proportional,
        term_months: 48,
        next: (0, 0),
        parties: &[
            p("ba_sda", "Party of Democratic Action", "Stranka demokratske akcije", Family::Religious, 0.358),
            p("ba_sds", "Serbian Democratic Party", "Srpska demokratska stranka", Family::Nationalist, 0.300),
            p("ba_hdz", "Croatian Democratic Union of BiH", "Hrvatska demokratska zajednica BiH", Family::Nationalist, 0.184),
            p("ba_sdp", "Social Democratic Party", "Socijaldemokratska partija", Family::SocialDemocratic, 0.060),
        ],
        ruling: "the Assembly",
        pillars: &[],
    },
    // Argentina — Chamber of Deputies, 14 May 1989, held with the presidential
    // election Carlos Menem won for the Justicialists with 47.5%: PJ 44.7%, UCR
    // 28.8%, the Alianza de Centro around the UCeDe 6.9%, Izquierda Unida 3.5%,
    // and a long tail of provincial parties of which the Neuquen People's
    // Movement is the durable one. Raul Alfonsin handed power over five months
    // early, in July 1989, because hyperinflation had made governing impossible.
    // Half the chamber renews every two years, so the next round is due in
    // September 1991.
    Polity {
        nation: NationId::Argentina,
        system: Electoral::Proportional,
        term_months: 48,
        next: (1991, 9),
        parties: &[
            p("ar_pj", "Justicialist Party", "Partido Justicialista", Family::BigTent, 0.447),
            p("ar_ucr", "Radical Civic Union", "Union Civica Radical", Family::Liberal, 0.288),
            p("ar_ucede", "Union of the Democratic Centre", "Union del Centro Democratico", Family::Conservative, 0.069),
            p("ar_iu", "United Left", "Izquierda Unida", Family::Communist, 0.035),
            p("ar_mpn", "Neuquen People's Movement", "Movimiento Popular Neuquino", Family::Regionalist, 0.015),
        ],
        ruling: "the Chamber of Deputies",
        pillars: &[],
    },
    // Mexico — Chamber of Deputies, 6 July 1988: PRI 51.1%, the Cardenista
    // Frente Democratico Nacional 29.1%, PAN 18.0%. The count famously stopped
    // when "se cayo el sistema" and resumed with the PRI ahead; the ballots were
    // burned in 1992. The FDN becomes the PRD in May 1989 and is entered under
    // that name with its 1988 share. Deputies serve three years, so the midterm
    // falls in August 1991. The PRI had not lost a presidential election since
    // its founding in 1929 and does not lose one until 2000 — which the model
    // has to reach through legitimacy, not a date.
    Polity {
        nation: NationId::Mexico,
        system: Electoral::Proportional,
        term_months: 36,
        next: (1991, 8),
        parties: &[
            p("mx_pri", "Institutional Revolutionary Party", "Partido Revolucionario Institucional", Family::BigTent, 0.511),
            p("mx_prd", "Party of the Democratic Revolution", "Partido de la Revolucion Democratica", Family::SocialDemocratic, 0.291),
            p("mx_pan", "National Action Party", "Partido Accion Nacional", Family::ChristianDemocratic, 0.180),
        ],
        ruling: "the Chamber of Deputies",
        pillars: &[],
    },
    // Chile — Chamber of Deputies, 14 December 1989, the first since 1973, held
    // under the constitution the outgoing regime wrote in 1980 and with the
    // binomial system it designed to give the right half the seats on a third of
    // the vote. Party lists inside the two blocs: PDC 26.0%, RN 18.3%, PPD
    // 11.5%, UDI 9.8%, PS 7.0%, PR 3.9%. Patricio Aylwin of the Concertacion
    // took the presidency with 55.2% and is inaugurated on 11 March 1990.
    Polity {
        nation: NationId::Chile,
        system: Electoral::Proportional,
        term_months: 48,
        next: (1993, 12),
        parties: &[
            p("cl_pdc", "Christian Democratic Party", "Partido Democrata Cristiano", Family::ChristianDemocratic, 0.260),
            p("cl_rn", "National Renewal", "Renovacion Nacional", Family::Conservative, 0.183),
            p("cl_ppd", "Party for Democracy", "Partido por la Democracia", Family::SocialDemocratic, 0.115),
            p("cl_udi", "Independent Democratic Union", "Union Democrata Independiente", Family::Conservative, 0.098),
            p("cl_ps", "Socialist Party of Chile", "Partido Socialista de Chile", Family::SocialDemocratic, 0.070),
            p("cl_pr", "Radical Party", "Partido Radical", Family::Liberal, 0.039),
        ],
        ruling: "the Chamber of Deputies",
        pillars: &[],
    },
    // Colombia — Chamber of Representatives, 9 March 1986: Liberals 48.6%,
    // Social Conservatives 37.8%, the Patriotic Union 1.3%. The UP was the
    // civilian party the FARC founded under the 1984 ceasefire, and between
    // 1986 and 1990 somewhere upward of two thousand of its members were
    // murdered, including both of its presidential candidates. The M-19
    // Democratic Alliance is entered at 2.7%, its result in the congressional
    // election of March 1990, the month it disarmed. That election is the next
    // one due when the game opens.
    Polity {
        nation: NationId::Colombia,
        system: Electoral::Proportional,
        term_months: 48,
        next: (1990, 3),
        parties: &[
            p("co_pl", "Colombian Liberal Party", "Partido Liberal Colombiano", Family::Liberal, 0.486),
            p("co_psc", "Social Conservative Party", "Partido Social Conservador", Family::Conservative, 0.378),
            p("co_adm19", "M-19 Democratic Alliance", "Alianza Democratica M-19", Family::SocialDemocratic, 0.027),
            p("co_up", "Patriotic Union", "Union Patriotica", Family::Communist, 0.013),
        ],
        ruling: "the Chamber of Representatives",
        pillars: &[],
    },
    // Venezuela — Chamber of Deputies, 4 December 1988: AD 43.3%, COPEI 31.1%,
    // MAS 10.3%, MEP 1.8%, La Causa R 1.6%. The Punto Fijo pact of 1958 gave
    // Venezuela thirty years of two-party alternation and the most stable
    // democracy in South America; ten weeks after this table was voted, the
    // army shot several hundred people in Caracas during the Caracazo, and the
    // pact never recovered. Five-year terms, next due December 1993.
    Polity {
        nation: NationId::Venezuela,
        system: Electoral::Proportional,
        term_months: 60,
        next: (1993, 12),
        parties: &[
            p("ve_ad", "Democratic Action", "Accion Democratica", Family::SocialDemocratic, 0.433),
            p("ve_copei", "Social Christian Party", "Comite de Organizacion Politica Electoral Independiente", Family::ChristianDemocratic, 0.311),
            p("ve_mas", "Movement Towards Socialism", "Movimiento al Socialismo", Family::Communist, 0.103),
            p("ve_mep", "People's Electoral Movement", "Movimiento Electoral del Pueblo", Family::SocialDemocratic, 0.018),
            p("ve_causar", "Radical Cause", "La Causa Radical", Family::Communist, 0.016),
        ],
        ruling: "the Chamber of Deputies",
        pillars: &[],
    },
    // Peru — Chamber of Deputies, 14 April 1985: APRA 50.1%, Izquierda Unida
    // 23.0%, the Convergencia Democratica around the PPC 12.0%, Accion Popular
    // 7.3%. Alan Garcia's single term ends in hyperinflation and a war he is
    // losing, and the next election is due in three months.
    //
    // Cambio 90 is deliberately NOT in this table, and the omission is the
    // honest reading rather than an oversight. Alberto Fujimori built it in
    // 1989 out of evangelical congregations and informal traders' guilds, and
    // it took 16.5% of the Chamber on 8 April 1990. The convention this module
    // uses elsewhere — enter a party founded after the last election at its
    // first contested share, as Colombia's AD M-19 is entered — cannot be
    // applied here: 50.1 + 23.0 + 12.0 + 7.3 already accounts for 92.4% of the
    // 1985 vote, so adding 16.5 gives a chamber where 108.9% of the electorate
    // voted. A 1985 table with a 1990 party in it is not a transcription of
    // either election. Fujimori's outsider is left for the model to produce
    // from a collapsing party system, which is the whole premise.
    Polity {
        nation: NationId::Peru,
        system: Electoral::Proportional,
        term_months: 60,
        next: (1990, 4),
        parties: &[
            p("pe_apra", "Peruvian Aprista Party", "Partido Aprista Peruano", Family::SocialDemocratic, 0.501),
            p("pe_iu", "United Left", "Izquierda Unida", Family::Communist, 0.230),
            p("pe_ppc", "Christian People's Party", "Partido Popular Cristiano", Family::ChristianDemocratic, 0.120),
            p("pe_ap", "Popular Action", "Accion Popular", Family::Liberal, 0.073),
        ],
        ruling: "the Chamber of Deputies",
        pillars: &[],
    },
    // Cuba — one legal party since 1965, and no national election a voter could
    // change anything with: the National Assembly was chosen indirectly by the
    // municipal assemblies until the 1993 reform introduced direct election of
    // deputies, still uncontested. What holds the state is the FAR under Raul
    // Castro, the party apparatus, and the Ministry of the Interior — and in
    // July 1989 the regime shot General Arnaldo Ochoa, the most decorated
    // officer of the Angolan war, and purged MININT down to the bone. That was
    // a regime securing exactly these pillars against exactly this risk.
    Polity {
        nation: NationId::Cuba,
        system: Electoral::Proportional,
        term_months: 60,
        next: (0, 0),
        parties: &[
            p("cu_pcc", "Communist Party of Cuba", "Partido Comunista de Cuba", Family::Communist, 1.00),
        ],
        ruling: "the Council of State",
        pillars: &[
            pl(Pillar::Army, "the Revolutionary Armed Forces"),
            pl(Pillar::Party, "the Communist Party of Cuba"),
            pl(Pillar::Security, "the Ministry of the Interior"),
        ],
    },
    // Bolivia — general election, 7 May 1989: MNR 25.7%, ADN 25.2%, MIR 21.8%,
    // CONDEPA 12.3%, Izquierda Unida 7.2%. Nobody came near a majority, so
    // Congress chose the president, and it chose the man who came third: Jaime
    // Paz Zamora of the MIR took office on 6 August 1989 in the Acuerdo
    // Patriotico with Hugo Banzer's ADN — the general who had jailed and exiled
    // him. Bolivia's arithmetic produces coalitions nobody would design.
    Polity {
        nation: NationId::Bolivia,
        system: Electoral::Proportional,
        term_months: 48,
        next: (1993, 6),
        parties: &[
            p("bo_mnr", "Nationalist Revolutionary Movement", "Movimiento Nacionalista Revolucionario", Family::BigTent, 0.257),
            p("bo_adn", "Nationalist Democratic Action", "Accion Democratica Nacionalista", Family::Conservative, 0.252),
            p("bo_mir", "Revolutionary Left Movement", "Movimiento de la Izquierda Revolucionaria", Family::SocialDemocratic, 0.218),
            p("bo_condepa", "Conscience of the Fatherland", "Conciencia de Patria", Family::Regionalist, 0.123),
            p("bo_iu", "United Left", "Izquierda Unida", Family::Communist, 0.072),
        ],
        ruling: "the National Congress",
        pillars: &[],
    },
    // Ecuador — congressional election of 31 January 1988, held with the
    // presidential first round Rodrigo Borja of the Izquierda Democratica went
    // on to win: ID 24.6%, PRE 14.7%, PSC 12.5%, DP 11.5%, the Radical Liberals
    // 8.0%, MPD 5.4%. Provincial deputies serve two years against the national
    // deputies' four, so the midterm falls on 17 June 1990 and Borja loses his
    // majority in it. Shares here are the least certain in this region's table
    // and are noted as such in the data files.
    Polity {
        nation: NationId::Ecuador,
        system: Electoral::Proportional,
        term_months: 48,
        next: (1990, 6),
        parties: &[
            p("ec_id", "Democratic Left", "Izquierda Democratica", Family::SocialDemocratic, 0.246),
            p("ec_pre", "Ecuadorian Roldosist Party", "Partido Roldosista Ecuatoriano", Family::BigTent, 0.147),
            p("ec_psc", "Social Christian Party", "Partido Social Cristiano", Family::Conservative, 0.125),
            p("ec_dp", "Popular Democracy", "Democracia Popular", Family::ChristianDemocratic, 0.115),
            p("ec_plre", "Ecuadorian Radical Liberal Party", "Partido Liberal Radical Ecuatoriano", Family::Liberal, 0.080),
            p("ec_mpd", "Popular Democratic Movement", "Movimiento Popular Democratico", Family::Communist, 0.054),
        ],
        ruling: "the National Congress",
        pillars: &[],
    },
    // Uruguay — general election, 26 November 1989: the National Party 38.9%,
    // the Colorados 30.3%, the Frente Amplio 21.2%, Nuevo Espacio 9.0%. Luis
    // Alberto Lacalle takes office on 1 March 1990, the second government since
    // the dictatorship ended, and the first alternation in twenty-eight years.
    // The Frente Amplio takes Montevideo the same day and never gives it back.
    // Five-year terms with no re-election, so the next is due November 1994.
    Polity {
        nation: NationId::Uruguay,
        system: Electoral::Proportional,
        term_months: 60,
        next: (1994, 11),
        parties: &[
            p("uy_pn", "National Party", "Partido Nacional", Family::Conservative, 0.389),
            p("uy_pc", "Colorado Party", "Partido Colorado", Family::Liberal, 0.303),
            p("uy_fa", "Broad Front", "Frente Amplio", Family::SocialDemocratic, 0.212),
            p("uy_ne", "New Space", "Nuevo Espacio", Family::SocialDemocratic, 0.090),
        ],
        ruling: "the General Assembly",
        pillars: &[],
    // Algeria — the last *national* vote before the game opens is the People's
    // National Assembly of 26 February 1987, a single FLN list, and transcribing
    // that would say nothing true about January 1990. Parties were legalised in
    // July 1989, so the shares here are the first contested results: the local
    // elections of 12 June 1990 (FIS 54.2%, FLN 28.1%, RCD 2.1%) and, for the
    // FFS, which boycotted them, its 7.4% in the first round of the legislative
    // election of 26 December 1991. The next legislative election was due in
    // 1991; what the army did with the result is for the model to reach, not for
    // this table to decide.
    Polity {
        nation: NationId::Algeria,
        system: Electoral::TwoRound,
        term_months: 60,
        next: (1991, 12),
        parties: &[
            p("dz_fis", "Islamic Salvation Front", "al-Jabha al-Islamiyya lil-Inqadh", Family::Religious, 0.542),
            p("dz_fln", "National Liberation Front", "Front de Liberation Nationale", Family::BigTent, 0.281),
            p("dz_ffs", "Socialist Forces Front", "Front des Forces Socialistes", Family::Regionalist, 0.074),
            p("dz_rcd", "Rally for Culture and Democracy", "Rassemblement pour la Culture et la Democratie", Family::Liberal, 0.021),
        ],
        ruling: "the People's National Assembly",
        pillars: &[
            pl(Pillar::Army, "the Armee Nationale Populaire"),
            pl(Pillar::Security, "the Securite Militaire"),
        ],
    },
    // Morocco — Chamber of Representatives, 14 September 1984: Constitutional
    // Union 24.8%, RNI 17.2%, Popular Movement 15.6%, Istiqlal 15.3%, USFP
    // 12.4%, PND 8.9%. The election due in 1990 was postponed by two years by
    // the referendum of December 1989 and did not happen until June 1993, which
    // is the point: the chamber was elected, the government was not. Hassan II
    // appointed it, and the institutions that could have removed him had already
    // tried twice, in 1971 and 1972.
    Polity {
        nation: NationId::Morocco,
        system: Electoral::FirstPastThePost,
        term_months: 72,
        next: (0, 0),
        parties: &[
            p("ma_uc", "Constitutional Union", "al-Ittihad al-Dusturi", Family::Conservative, 0.248),
            p("ma_rni", "National Rally of Independents", "Rassemblement National des Independants", Family::Liberal, 0.172),
            p("ma_mp", "Popular Movement", "Mouvement Populaire", Family::Agrarian, 0.156),
            p("ma_istiqlal", "Istiqlal Party", "Hizb al-Istiqlal", Family::Nationalist, 0.153),
            p("ma_usfp", "Socialist Union of Popular Forces", "al-Ittihad al-Ishtiraki lil-Quwwat al-Sha'biyya", Family::SocialDemocratic, 0.124),
            p("ma_pnd", "National Democratic Party", "al-Hizb al-Watani al-Dimuqrati", Family::Conservative, 0.089),
        ],
        ruling: "the Alaouite monarchy",
        pillars: &[
            pl(Pillar::Party, "the Makhzen"),
            pl(Pillar::Army, "the Forces Armees Royales"),
            pl(Pillar::Security, "the Direction Generale de la Surveillance du Territoire"),
            pl(Pillar::Clergy, "the Council of Ulema"),
            pl(Pillar::Business, "the Omnium Nord Africain"),
        ],
    },
    // Tunisia — Chamber of Deputies, 2 April 1989: the RCD took 80.4% of the
    // vote and every one of the 141 seats, because the list system awarded the
    // whole constituency to the winning list. Ennahda was refused registration
    // and ran its people as independents, who took 13.7% and nothing. That gap
    // between a sixth of the vote and none of the seats is the Tunisian problem
    // in one line, and it is why this is a regime with pillars.
    Polity {
        nation: NationId::Tunisia,
        system: Electoral::FirstPastThePost,
        term_months: 60,
        next: (0, 0),
        parties: &[
            p("tn_rcd", "Democratic Constitutional Rally", "al-Tajammu' al-Dusturi al-Dimuqrati", Family::BigTent, 0.804),
            p("tn_nahda", "Ennahda", "Harakat al-Nahda", Family::Religious, 0.137),
            p("tn_mds", "Movement of Socialist Democrats", "Harakat al-Dimuqratiyyin al-Ishtirakiyyin", Family::SocialDemocratic, 0.032),
        ],
        ruling: "the Democratic Constitutional Rally",
        pillars: &[
            pl(Pillar::Party, "the RCD apparatus"),
            pl(Pillar::Security, "the Direction de la Surete Nationale"),
            pl(Pillar::Army, "the Tunisian Armed Forces"),
        ],
    },
    // Libya — no parties and no election of any kind, ever. Law 71 of 1972 made
    // forming one a capital offence and the Green Book's answer to who should
    // govern is that representation is fraud. The General People's Congress met
    // and Gaddafi held no office in it. What could have removed him is the list
    // below: the army tried in 1975, 1984 and 1993, and the Revolutionary
    // Committees existed to watch it.
    Polity {
        nation: NationId::Libya,
        system: Electoral::Proportional,
        term_months: 48,
        next: (0, 0),
        parties: &[],
        ruling: "the General People's Congress",
        pillars: &[
            pl(Pillar::Army, "the Libyan Arab Armed Forces"),
            pl(Pillar::Party, "the Revolutionary Committees"),
            pl(Pillar::Security, "the Jamahiriya Security Organisation"),
        ],
    },
    // Sudan — National Assembly, 1-12 April 1986: the Umma Party 100 seats, the
    // DUP 63, the National Islamic Front 51, of 260 filled. The popular vote was
    // never published by constituency, so the shares here are shares of the
    // seats actually filled; 41 southern seats could not be polled at all
    // because the SPLA held the ground. Omar al-Bashir dissolved the assembly on
    // 30 June 1989 and banned every one of these parties, so the table is what
    // becomes live again if the regime opens, not what governs in 1990.
    Polity {
        nation: NationId::Sudan,
        system: Electoral::FirstPastThePost,
        term_months: 48,
        next: (0, 0),
        parties: &[
            p("sd_umma", "National Umma Party", "Hizb al-Umma al-Qawmi", Family::Religious, 0.385),
            p("sd_dup", "Democratic Unionist Party", "al-Hizb al-Ittihadi al-Dimuqrati", Family::BigTent, 0.242),
            p("sd_nif", "National Islamic Front", "al-Jabha al-Islamiyya al-Qawmiyya", Family::Religious, 0.196),
        ],
        ruling: "the Revolutionary Command Council for National Salvation",
        pillars: &[
            pl(Pillar::Army, "the Sudanese Armed Forces"),
            pl(Pillar::Party, "the National Islamic Front"),
            pl(Pillar::Security, "the National Security Service"),
        ],
    },
];

pub fn polity(id: NationId) -> Option<&'static Polity> {
    POLITIES.iter().find(|x| x.nation == id)
}

fn spec(id: NationId, party: &str) -> Option<&'static PartySpec> {
    polity(id)?.parties.iter().find(|p| p.id == party)
}

/// A party's structural constituency: the transcribed share, normalised. This
/// is the anchor support reverts toward, and without it the model has no
/// equilibrium at all — an incumbent with a good record gains a little every
/// month forever and ends up with the entire electorate. Parties have floors
/// and ceilings set by who their voters actually are; an economy moves the
/// margin, not the whole country.
fn base_share(id: NationId, party: &str) -> f64 {
    let pol = match polity(id) {
        Some(p) => p,
        None => return 0.0,
    };
    let total: f64 = pol.parties.iter().map(|s| s.start.max(0.001)).sum();
    if total <= 0.0 {
        return 0.0;
    }
    pol.parties
        .iter()
        .find(|s| s.id == party)
        .map(|s| s.start.max(0.001) / total)
        .unwrap_or(0.0)
}

// ---------------------------------------------------------------------------
// State
// ---------------------------------------------------------------------------

/// Every government in the world. A `Vec`, iterated in insertion order, because
/// determinism is sacred and a map's iteration order is not.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Governments {
    pub states: Vec<GovState>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GovState {
    pub nation: NationId,
    /// (party id, support 0..1), in table order and always summing to 1.
    pub support: Vec<(String, f64)>,
    /// (party id, seat share 0..1) as of the last election.
    pub seats: Vec<(String, f64)>,
    /// Who is in office. The first entry leads it.
    pub coalition: Vec<String>,
    /// The election this parliament is running toward, (year, month).
    pub next_election: (i32, u32),
    /// Whether anyone has voted yet under this module.
    pub elected: bool,
    /// (pillar, loyalty 0..1) for a regime that is not elected.
    pub pillars: Vec<(Pillar, f64)>,
    /// How close the pillars are to acting. Accumulates while they are unpaid.
    pub coup_pressure: f64,
    /// Months this government has been in office. A honeymoon is real and short.
    pub months_in_office: u32,
}

impl GovState {
    pub fn seat_share(&self, party: &str) -> f64 {
        self.seats.iter().find(|(p, _)| p == party).map(|(_, v)| *v).unwrap_or(0.0)
    }
    pub fn support_of(&self, party: &str) -> f64 {
        self.support.iter().find(|(p, _)| p == party).map(|(_, v)| *v).unwrap_or(0.0)
    }
    /// What share of the chamber the government commands.
    pub fn government_seats(&self) -> f64 {
        self.coalition.iter().map(|p| self.seat_share(p)).sum()
    }
    pub fn leader(&self) -> Option<&str> {
        self.coalition.first().map(|s| s.as_str())
    }
    pub fn in_government(&self, party: &str) -> bool {
        self.coalition.iter().any(|p| p == party)
    }
    pub fn loyalty(&self, pillar: Pillar) -> f64 {
        self.pillars.iter().find(|(p, _)| *p == pillar).map(|(_, v)| *v).unwrap_or(1.0)
    }
    /// The least contented institution of any kind — what a briefing shows.
    pub fn weakest_pillar(&self) -> Option<(Pillar, f64)> {
        self.pillars
            .iter()
            .copied()
            .min_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal))
    }
    /// The least contented institution that could actually remove a government.
    /// Merchants and clergy can withdraw their blessing; they cannot arrest a
    /// cabinet, and a model in which they can produces a coup every other year.
    pub fn weakest_armed(&self) -> Option<(Pillar, f64)> {
        self.pillars
            .iter()
            .copied()
            .filter(|(p, _)| {
                matches!(p, Pillar::Army | Pillar::Security | Pillar::Party)
            })
            .min_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal))
    }
    pub fn mean_loyalty(&self) -> f64 {
        if self.pillars.is_empty() {
            return 1.0;
        }
        self.pillars.iter().map(|(_, v)| *v).sum::<f64>() / self.pillars.len() as f64
    }
}

pub fn state(w: &WorldState, id: NationId) -> Option<&GovState> {
    w.governments.states.iter().find(|g| g.nation == id)
}
fn state_mut(w: &mut WorldState, id: NationId) -> Option<&mut GovState> {
    w.governments.states.iter_mut().find(|g| g.nation == id)
}

fn normalise(v: &mut [(String, f64)]) {
    let total: f64 = v.iter().map(|(_, s)| *s).sum();
    if total <= 0.0 {
        return;
    }
    for e in v.iter_mut() {
        e.1 /= total;
    }
}

/// Seat a government from the transcribed table. Called lazily so that a save
/// written before this module existed still loads and simply grows one.
pub fn ensure(w: &mut WorldState, id: NationId) {
    if state(w, id).is_some() {
        return;
    }
    let pol = match polity(id) {
        Some(p) => p,
        None => return,
    };
    let mut support: Vec<(String, f64)> =
        pol.parties.iter().map(|s| (s.id.to_string(), s.start.max(0.001))).collect();
    normalise(&mut support);
    let pillars: Vec<(Pillar, f64)> =
        pol.pillars.iter().map(|s| (s.pillar, 0.65)).collect();
    let mut g = GovState {
        nation: id,
        seats: support.clone(),
        support,
        coalition: vec![],
        next_election: pol.next,
        elected: false,
        pillars,
        coup_pressure: 0.0,
        months_in_office: 0,
    };
    // Seats at the opening are the last real result read through this system's
    // own machinery, so that January 1990 and January 1994 are described the
    // same way.
    g.seats = seats_from(&g.support, pol.system);
    w.governments.states.push(g);
    if is_electoral(w, id) {
        form_government(w, id, false);
    }
}

pub fn ensure_all(w: &mut WorldState) {
    let ids: Vec<NationId> = w.nations.iter().filter(|n| n.alive).map(|n| n.id).collect();
    for id in ids {
        ensure(w, id);
    }
}

// ---------------------------------------------------------------------------
// Seats and formation
// ---------------------------------------------------------------------------

fn seats_from(support: &[(String, f64)], sys: Electoral) -> Vec<(String, f64)> {
    let (exp, threshold) = sys.shape();
    let mut out: Vec<(String, f64)> = support
        .iter()
        .map(|(id, s)| {
            // exact::powf, not f64::powf: the cube law runs on every election
            // in every nation, so a platform that rounds pow differently would
            // hand out different seats and fork the timeline. See exact.rs.
            let v = if *s < threshold { 0.0 } else { crate::exact::powf(*s, exp) };
            (id.clone(), v)
        })
        .collect();
    let total: f64 = out.iter().map(|(_, v)| *v).sum();
    if total <= 0.0 {
        // Everyone below the bar: the largest party takes the chamber, which is
        // what a high threshold does when the vote fragments under it.
        let mut best = 0usize;
        for (i, (_, s)) in support.iter().enumerate() {
            if *s > support[best].1 {
                best = i;
            }
        }
        out = support.iter().map(|(id, _)| (id.clone(), 0.0)).collect();
        if let Some(e) = out.get_mut(best) {
            e.1 = 1.0;
        }
        return out;
    }
    for e in out.iter_mut() {
        e.1 /= total;
    }
    out
}

fn distance(id: NationId, a: &str, b: &str) -> f64 {
    match (spec(id, a), spec(id, b)) {
        (Some(x), Some(y)) => family_distance(x.family, y.family),
        _ => 1.0,
    }
}

/// Assemble a government out of a chamber. The largest party gets the first go,
/// then adds the nearest partner it is allowed to sit with until it has half the
/// seats. If it cannot get there, it governs as a minority — which is a real
/// outcome and an expensive one, not a failure state.
fn form_government(w: &mut WorldState, id: NationId, announce: bool) {
    let sys = match polity(id) {
        Some(p) => p.system,
        None => return,
    };
    let (mut ranked, pariahs): (Vec<(String, f64)>, Vec<String>) = {
        let g = match state(w, id) {
            Some(g) => g,
            None => return,
        };
        let mut r = g.seats.clone();
        r.retain(|(_, v)| *v > 0.0);
        r.sort_by(|a, b| {
            b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal).then(a.0.cmp(&b.0))
        });
        let ps = polity(id)
            .map(|p| p.parties.iter().filter(|s| s.pariah).map(|s| s.id.to_string()).collect())
            .unwrap_or_default();
        (r, ps)
    };
    if ranked.is_empty() {
        return;
    }
    // A pariah party that wins outright still governs — the cordon holds against
    // coalition partners, not against arithmetic.
    let leader = ranked.remove(0).0;
    let mut coalition = vec![leader.clone()];
    let mut held = {
        let g = state(w, id).unwrap();
        g.seat_share(&leader)
    };
    if held < 0.5 {
        // Nearest ideological neighbour first, and never a pariah as a partner.
        let mut candidates: Vec<(String, f64, f64)> = ranked
            .iter()
            .filter(|(p, _)| !pariahs.contains(p) && *p != leader)
            .map(|(p, v)| (p.clone(), *v, distance(id, &leader, p)))
            .collect();
        candidates.sort_by(|a, b| {
            a.2.partial_cmp(&b.2).unwrap_or(std::cmp::Ordering::Equal).then(b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal)).then(a.0.cmp(&b.0))
        });
        for (party, seats, dist) in candidates {
            if held >= 0.5 {
                break;
            }
            // Nobody joins a cabinet with a party at the other end of the plane
            // just to make up the numbers. Beyond this, the government does not
            // form and the country gets a minority administration.
            if dist > 1.5 {
                continue;
            }
            coalition.push(party);
            held += seats;
        }
    }
    let minority = held < 0.5;
    let leader_name = spec(id, &leader).map(|s| s.name).unwrap_or("the largest party");
    let partners = coalition.len();
    if let Some(g) = state_mut(w, id) {
        g.coalition = coalition;
        g.months_in_office = 0;
    }
    let _ = sys;
    if announce {
        // Note what is *not* here: a stability bonus. The code this module
        // replaced handed every democracy three points of stability every four
        // years, or eight if times were bad, and that free legitimacy for having
        // held a vote is precisely the popularity slider BIBLE section 4 says to
        // get rid of. A new government's honeymoon is real, but it is standing,
        // not order, and it is paid in `standing_modifier` where it decays over
        // six months instead of ratcheting a number upward forever.
        let msg = if minority {
            format!(
                "{} votes: {} leads a minority government commanding {:.0}% of the chamber.",
                id.name(), leader_name, held * 100.0
            )
        } else if partners == 1 {
            format!(
                "{} votes: {} takes office with {:.0}% of the seats and no partners.",
                id.name(), leader_name, held * 100.0
            )
        } else {
            format!(
                "{} votes: {} forms a {}-party coalition holding {:.0}% of the seats.",
                id.name(), leader_name, partners, held * 100.0
            )
        };
        w.headline(msg);
    }
}

// ---------------------------------------------------------------------------
// Support movement
// ---------------------------------------------------------------------------

/// The four things a government is judged on, each 0 (fine) to 1 (unbearable).
struct Pains {
    prices: f64,
    growth: f64,
    war: f64,
    order: f64,
}

fn pains(w: &WorldState, id: NationId) -> Pains {
    let n = w.nation(id);
    Pains {
        // Anything above 3% starts to be felt; 18% is where a government is
        // being judged on nothing else.
        prices: ((n.inflation - 0.03) / 0.15).clamp(0.0, 1.0),
        // A percent of growth is neutral; a 4% contraction is a catastrophe.
        growth: ((0.01 - n.growth_last) / 0.05).clamp(0.0, 1.0),
        war: n.war_exhaustion.clamp(0.0, 1.0),
        order: ((60.0 - n.stability) / 60.0).clamp(0.0, 1.0) + n.separatism * 0.4,
    }
}

/// How attractive an opposition family is, given what is currently hurting. This
/// is the whole claim of the module in one function: support does not move
/// because a player nudged it, it moves because prices are up and that is worth
/// something specific to specific people.
fn appeal(family: Family, pn: &Pains, development: f64) -> f64 {
    let base = 0.15;
    base + match family {
        // A recession is the left's argument, and the harder the recession the
        // further left it goes.
        Family::Communist => pn.growth * 1.30 + pn.prices * 0.10,
        Family::SocialDemocratic => pn.growth * 0.95 + pn.order * 0.10,
        // Sound money is the right's, and it is the only pain they gain from.
        Family::Conservative => pn.prices * 1.00 + pn.order * 0.35,
        Family::Liberal => pn.prices * 0.80 + (1.0 - pn.order) * 0.20,
        Family::ChristianDemocratic => pn.prices * 0.70 + pn.order * 0.25,
        // Disorder, a war going badly, and a state coming apart are the
        // nationalist and religious families' whole market.
        Family::Nationalist => pn.order * 1.20 + pn.war * 0.80,
        Family::Religious => pn.order * 0.90 + pn.growth * 0.40,
        Family::Regionalist => pn.order * 1.00,
        Family::Agrarian => pn.prices * 0.40 + pn.growth * 0.40,
        // A postmaterial vote needs a country that is not frightened about money.
        Family::Green => (1.0 - pn.growth) * (1.0 - pn.prices) * development * 0.90,
        Family::BigTent => 0.25,
    }
}

/// One month of the electorate changing its mind. Monthly, so the coefficients
/// are small; a bad year moves five to ten points, which is about what a bad
/// year does.
fn drift_support(w: &mut WorldState, id: NationId) {
    let pn = pains(w, id);
    let development = {
        let n = w.nation(id);
        (n.gdp * 1000.0 / n.population.max(0.001) / 20000.0).clamp(0.0, 1.0)
    };
    // The government's record. Note where the zero sits: a government with
    // nothing going wrong gains only a little, and it takes rather less than
    // half of one pain to put it under water. The first draft put the neutral
    // point at the *midpoint* of the pain scale, which meant a government
    // fighting a war it was visibly losing still gained support every month as
    // long as the economy was fine — and it was why an incumbent's support crept
    // upward for forty years with nothing ever pushing back.
    let record = 0.35 - (0.90 * pn.prices + 0.90 * pn.growth + 1.10 * pn.war);
    let incumbents: Vec<String> = match state(w, id) {
        Some(g) => g.coalition.clone(),
        None => return,
    };
    if incumbents.is_empty() {
        return;
    }
    let families: Vec<(String, Family)> = match polity(id) {
        Some(pol) => pol.parties.iter().map(|s| (s.id.to_string(), s.family)).collect(),
        None => return,
    };
    // What flows out of (or into) the parties of government this month. Small,
    // because it is a month: a bad year moves five or six points, which is about
    // what a bad year does.
    let swing = record * 0.006;
    let appeals: Vec<(String, f64)> = families
        .iter()
        .filter(|(pid, _)| !incumbents.contains(pid))
        .map(|(pid, f)| (pid.clone(), appeal(*f, &pn, development).max(0.01)))
        .collect();
    let appeal_total: f64 = appeals.iter().map(|(_, a)| *a).sum();
    let g = match state_mut(w, id) {
        Some(g) => g,
        None => return,
    };
    let held: f64 = incumbents.iter().map(|p| g.support_of(p)).sum();
    // A party with nothing left cannot lose more, and one with everything
    // cannot gain: the transfer is bounded by what exists on each side.
    let moved = if swing < 0.0 {
        -(-swing * held).min(0.015)
    } else {
        (swing * (1.0 - held)).min(0.015)
    };
    for e in g.support.iter_mut() {
        if incumbents.contains(&e.0) {
            // Shared out among the governing parties in proportion to their size:
            // the prime minister's party wears most of it, which is right.
            let share = if held > 0.0 { e.1 / held } else { 0.0 };
            e.1 += moved * share;
        } else if appeal_total > 0.0 {
            let a = appeals.iter().find(|(p, _)| *p == e.0).map(|(_, a)| *a).unwrap_or(0.0);
            e.1 -= moved * (a / appeal_total);
        }
        e.1 = e.1.max(0.002);
    }
    // ...and then everyone drifts back toward who their voters are. This is what
    // makes the whole thing an equilibrium instead of a ratchet: a government
    // that delivers for twenty years ends up perhaps twelve points above its
    // structural share, not at a hundred per cent.
    for e in g.support.iter_mut() {
        let base = base_share(id, &e.0);
        e.1 += (base - e.1) * 0.020;
        e.1 = e.1.max(0.002);
    }
    normalise(&mut g.support);
}

// ---------------------------------------------------------------------------
// Elections
// ---------------------------------------------------------------------------

fn add_months(y: i32, m: u32, months: u32) -> (i32, u32) {
    let total = (m - 1) + months;
    (y + (total / 12) as i32, total % 12 + 1)
}

fn due(w: &WorldState, g: &GovState) -> bool {
    let (y, m) = g.next_election;
    y > 0 && (w.year > y || (w.year == y && w.month >= m))
}

/// Run one.
///
/// **This module draws no random numbers, on purpose.** Everything here is a
/// function of what the economy did to people, and the world's single RNG is
/// left untouched. That is not fastidiousness: the first draft rolled a
/// campaign swing, a coup die and an AI patronage die, and every one of those
/// draws shifted the shared stream, which reshuffled the histories that
/// `china_growth_miracle`, `arms_transfers_build_a_client_army` and
/// `a_pact_drags_a_great_power_into_a_war_it_did_not_start` are calibrated
/// against. Three of them went red, then a different three, then a different
/// three again, with the failures moving around between runs of a tuning pass —
/// the signature of stream noise rather than a defect. Elections still differ
/// wildly between seeds, because the inflation and the growth and the wars they
/// are fought on differ between seeds.
///
/// An election is therefore a straight readout of where opinion has drifted to,
/// run through the seat formula. The first draft also amplified the leader's
/// share on the way in — the wasted-vote psychology Duverger named — but
/// applying that to the stored support rather than to the result made it
/// compound election after election: Solidarity went 60%, 75%, 92%, 100% of the
/// Sejm and Poland became a one-party state by 1999. Manufacturing majorities is
/// the seat formula's job, and it does it once per election instead of
/// permanently rewriting the electorate.
pub fn hold_election(w: &mut WorldState, id: NationId) {
    let sys = match polity(id) {
        Some(p) => p.system,
        None => return,
    };
    let term = polity(id).map(|p| p.term_months).unwrap_or(48);
    let count = state(w, id).map(|g| g.support.len()).unwrap_or(0);
    if count == 0 {
        return;
    }
    let (y, m) = (w.year, w.month);
    if let Some(g) = state_mut(w, id) {
        normalise(&mut g.support);
        g.seats = seats_from(&g.support, sys);
        g.next_election = add_months(y, m, term);
        g.elected = true;
    }
    form_government(w, id, true);
}

// ---------------------------------------------------------------------------
// What holding a government together costs
// ---------------------------------------------------------------------------

/// The strain of the government a nation is currently running. Zero for a
/// single-party majority; it climbs with the number of partners, with how far
/// apart they are, and hardest of all when the government does not have the
/// votes at all.
pub fn strain(w: &WorldState, id: NationId) -> f64 {
    let g = match state(w, id) {
        Some(g) => g,
        None => return 0.0,
    };
    if !is_electoral(w, id) || g.coalition.is_empty() {
        return 0.0;
    }
    let leader = g.coalition[0].clone();
    let mut s = 0.0;
    for partner in g.coalition.iter().skip(1) {
        s += 1.0 + distance(id, &leader, partner) * 1.6;
    }
    if g.government_seats() < 0.5 {
        // Governing without a majority means buying every vote separately.
        s += 2.5;
    }
    // Capped, because past a certain point a chamber is simply ungovernable and
    // the model should say "ungovernable" rather than keep multiplying. Israel's
    // 1988 Knesset reaches this ceiling; so does Bosnia's 1990 assembly.
    s.min(8.0)
}

/// What the government's own composition does to the standing it can hold.
/// Read by `politics::political_capital`, which is where the two currencies
/// meet: a coalition is not a modifier on a slider, it is a claim on the same
/// budget everything else in the game is priced in.
pub fn standing_modifier(w: &WorldState, id: NationId) -> f64 {
    let g = match state(w, id) {
        Some(g) => g,
        None => return 0.0,
    };
    // This is deliberately a tax and not a bonus. A single-party majority is the
    // neutral case, worth nothing extra; everything else is a deduction. The
    // first draft paid +5 for a majority and it quietly cancelled a third of
    // what a war costs a government at home — `a_war_costs_a_government_at_home`
    // went from a nine-point gap to a three-point one, because the flat credit
    // lifted the war-torn government's target above its own stock and it started
    // climbing instead of falling. A modifier that pays everyone is not a
    // constraint, it is a rescaling.
    if is_electoral(w, id) {
        let mut m = -strain(w, id) * 1.8;
        // A new government gets a few months of grace and no more. Note the
        // shape: it decays to nothing inside half a year, so it cannot become a
        // standing credit the way the flat majority bonus in the first draft
        // did — that one cancelled a third of what a war costs at home.
        if g.months_in_office < 6 && g.elected {
            m += 6.0 - g.months_in_office as f64;
        }
        m
    } else {
        // The other half: legitimacy bought rather than voted for. A regime that
        // is paying all of its institutions is merely solvent — that is the zero
        // — and one that has stopped paying bleeds standing that nothing it
        // delivers can replace.
        ((g.mean_loyalty() - 0.75) * 24.0).min(2.0)
    }
}

/// Political capital a month, burned simply to keep the government standing.
pub fn upkeep(w: &WorldState, id: NationId) -> f64 {
    strain(w, id) * 0.20
}

/// What it costs to bring one more party into the cabinet: a flat price for the
/// negotiation and a steep one for the distance.
pub fn invite_price(w: &WorldState, id: NationId, party: &str) -> f64 {
    let g = match state(w, id) {
        Some(g) => g,
        None => return 0.0,
    };
    let leader = match g.leader() {
        Some(l) => l.to_string(),
        None => return 12.0,
    };
    10.0 + distance(id, &leader, party) * 14.0
}

// ---------------------------------------------------------------------------
// Commands
// ---------------------------------------------------------------------------

pub fn invite(w: &mut WorldState, id: NationId, party: &str) -> Result<(), String> {
    if !is_electoral(w, id) {
        return Err(format!("{} does not form governments by negotiation.", id.name()));
    }
    let s = spec(id, party).ok_or_else(|| format!("No such party: {}", party))?;
    let g = state(w, id).ok_or("no government")?;
    if g.in_government(party) {
        return Err(format!("{} is already in the government.", s.name));
    }
    if s.pariah {
        return Err(format!("No party in {} will sit in cabinet with {}.", id.name(), s.name));
    }
    if g.seat_share(party) <= 0.0 {
        return Err(format!("{} holds no seats.", s.name));
    }
    let name = s.name;
    if let Some(g) = state_mut(w, id) {
        g.coalition.push(party.to_string());
    }
    w.headline(format!("{} brings {} into the government.", id.name(), name));
    Ok(())
}

pub fn expel(w: &mut WorldState, id: NationId, party: &str) -> Result<(), String> {
    let s = spec(id, party).ok_or_else(|| format!("No such party: {}", party))?;
    let g = state(w, id).ok_or("no government")?;
    if !g.in_government(party) {
        return Err(format!("{} is not in the government.", s.name));
    }
    if g.leader() == Some(party) {
        return Err("A government cannot expel the party that leads it.".into());
    }
    let name = s.name;
    if let Some(g) = state_mut(w, id) {
        g.coalition.retain(|p| p != party);
    }
    // The party that was thrown out takes its grievance to the country.
    if let Some(g) = state_mut(w, id) {
        if let Some(e) = g.support.iter_mut().find(|(p, _)| p == party) {
            e.1 = (e.1 * 1.06).min(0.95);
        }
        normalise(&mut g.support);
    }
    let lost_majority = state(w, id).map_or(false, |g| g.government_seats() < 0.5);
    if lost_majority {
        w.nation_mut(id).stability = (w.nation(id).stability - 4.0).max(0.0);
    }
    w.headline(format!(
        "{} expels {} from the government{}.",
        id.name(),
        name,
        if lost_majority { ", and loses its majority" } else { "" }
    ));
    Ok(())
}

pub fn call_election(w: &mut WorldState, id: NationId) -> Result<(), String> {
    if !is_electoral(w, id) {
        return Err(format!("{} does not hold elections.", id.name()));
    }
    if state(w, id).map_or(true, |g| g.months_in_office < 6) {
        return Err("A government six months old cannot go back to the country yet.".into());
    }
    w.headline(format!("{} goes to the country early.", id.name()));
    hold_election(w, id);
    Ok(())
}

/// Pay an institution to stay loyal. Patronage is fiscal before it is political:
/// the army's loyalty is bought with the defence budget, the party's and the
/// merchants' with the state's, and all of it goes on the debt.
pub fn secure_pillar(w: &mut WorldState, id: NationId, pillar: Pillar) -> Result<(), String> {
    if is_electoral(w, id) {
        return Err(format!("{} answers to an electorate, not to its institutions.", id.name()));
    }
    let name = polity(id)
        .and_then(|p| p.pillars.iter().find(|s| s.pillar == pillar))
        .map(|s| s.name)
        .ok_or_else(|| format!("{} has no such institution.", id.name()))?;
    {
        let g = state_mut(w, id).ok_or("no regime")?;
        let entry = g
            .pillars
            .iter_mut()
            .find(|(p, _)| *p == pillar)
            .ok_or("no such pillar")?;
        entry.1 = (entry.1 + 0.20).min(1.0);
        g.coup_pressure = (g.coup_pressure - 0.15).max(0.0);
    }
    // A one-off payment, borrowed: the bonus, the new headquarters, the fleet of
    // cars. It is deliberately NOT a permanent addition to the defence budget.
    // The first draft added 0.4pp of GDP to military spending every time a
    // regime bought its army, and since the AI buys whenever loyalty sags, that
    // ratcheted defence budgets upward for a century — Kuwait's peacetime army
    // grew 40% on its own and `arms_transfers_build_a_client_army` failed
    // because the baseline it measures against had been inflated. Standing
    // budgets are what `SetMilSpend` is for, and the army's loyalty already
    // reads that budget; this command is the envelope on top of it.
    let n = w.nation_mut(id);
    n.debt_gdp += 0.008;
    w.headline(format!("{} buys the loyalty of {}.", id.name(), name));
    Ok(())
}

// ---------------------------------------------------------------------------
// The tick
// ---------------------------------------------------------------------------

/// Loyalty walks toward what the regime is currently giving each institution.
fn regime_tick(w: &mut WorldState, id: NationId) {
    let (mil, _invest, growth, infl, stab, auth, exhaustion, sanctioned) = {
        let n = w.nation(id);
        (
            n.mil_spend_gdp, n.state_invest_gdp, n.growth_last, n.inflation,
            n.stability, n.authoritarianism, n.war_exhaustion,
            w.sanctioned_by_count(id) as f64,
        )
    };
    let pillars: Vec<Pillar> = match state(w, id) {
        Some(g) => g.pillars.iter().map(|(p, _)| *p).collect(),
        None => return,
    };
    let mut targets: Vec<(Pillar, f64)> = vec![];
    for pillar in pillars {
        let t = match pillar {
            // Generals are bought with budgets and lost in wars that go badly.
            // The floor is deliberately low: the first draft started the army at
            // 0.35 and added the budget on top, which put an entirely unpaid army
            // at 0.357 — a hair above the 0.35 line at which pressure starts to
            // build. Twenty years of a defence budget cut to a tenth of a percent
            // of GDP produced no coup at all, because the model could not express
            // an army that had been abandoned.
            Pillar::Army => 0.20 + (mil / 0.08).min(1.0) * 0.65 - exhaustion * 0.45,
            // The apparatus is loyal because it *is* the regime — it has nowhere
            // else to go — so its floor rises with how authoritarian the state
            // is. What moves it is the programme visibly failing and the country
            // visibly slipping, which is what the last two terms read.
            Pillar::Party => {
                0.35 + auth * 0.40
                    + (growth * 10.0).clamp(-0.25, 0.25)
                    + (stab / 100.0 - 0.5) * 0.35
            }
            // The services want a free hand and a quiet street.
            Pillar::Security => 0.30 + auth * 0.45 + (stab / 100.0) * 0.30,
            // Money wants prices under control and the door to the world open.
            Pillar::Business => {
                0.55 + (growth * 10.0).clamp(-0.25, 0.25) - (infl / 0.25).min(1.0) * 0.40
                    - (sanctioned * 0.05).min(0.25)
            }
            // The clergy want order and piety, and notice when neither is being
            // supplied.
            Pillar::Clergy => 0.40 + (stab / 100.0) * 0.35 + auth * 0.25,
        };
        targets.push((pillar, t.clamp(0.0, 1.0)));
    }
    let g = match state_mut(w, id) {
        Some(g) => g,
        None => return,
    };
    for (pillar, target) in targets {
        if let Some(e) = g.pillars.iter_mut().find(|(p, _)| *p == pillar) {
            // Loyalty is slow to buy and quick to lose, like everything else in
            // this game that is worth having.
            let rate = if target < e.1 { 0.10 } else { 0.045 };
            e.1 += (target - e.1) * rate;
            e.1 = e.1.clamp(0.0, 1.0);
        }
    }
    // Pressure builds while one of the *armed* institutions is going unpaid.
    // Merchants and clergy withdraw legitimacy, which is what
    // `standing_modifier` reads; they do not put soldiers on the street. The
    // first draft let any pillar move, and Iran's bazaar overthrew the Islamic
    // Republic seven times in twenty years.
    let weakest = g.weakest_armed().map(|(_, v)| v).unwrap_or(1.0);
    if weakest < 0.35 {
        g.coup_pressure = (g.coup_pressure + (0.35 - weakest) * 0.15).min(1.5);
    } else {
        g.coup_pressure = (g.coup_pressure - 0.015).max(0.0);
    }
}

fn maybe_coup(w: &mut WorldState, id: NationId) {
    let (pressure, weakest, settled) = match state(w, id) {
        Some(g) => (g.coup_pressure, g.weakest_armed(), g.months_in_office),
        None => return,
    };
    // A regime that has just been through one is not going through another next
    // year. Whoever took power has purged, and the purge buys them time.
    if settled < 36 {
        return;
    }
    let (pillar, loyalty) = match weakest {
        Some(x) => x,
        None => return,
    };
    // The pressure gauge *is* the risk: it climbs only while an institution is
    // going unpaid, and a coup happens when it tops out. A regime that keeps its
    // pillars fed never reaches this line, and one that neglects them reaches it
    // in about a year. No die is thrown — see `hold_election` for why.
    if pressure < 1.0 / w.rules.crisis_intensity.max(0.1) {
        return;
    }
    let name = polity(id)
        .and_then(|pol| pol.pillars.iter().find(|s| s.pillar == pillar))
        .map(|s| s.name)
        .unwrap_or("the security apparatus");
    // A coup is not a revolution: the state survives, the government does not,
    // and whoever moved is now in charge and more afraid than the last lot.
    {
        let n = w.nation_mut(id);
        n.stability = (n.stability - 16.0).max(5.0);
        n.gdp *= 0.97;
        n.authoritarianism = match pillar {
            Pillar::Army | Pillar::Security => (n.authoritarianism + 0.08).min(0.98),
            Pillar::Clergy => (n.authoritarianism + 0.05).min(0.98),
            _ => (n.authoritarianism - 0.04).max(0.05),
        };
        n.political_capital = crate::politics::seated_political_capital(
            n.stability, n.inflation, n.authoritarianism,
        );
    }
    if let Some(g) = state_mut(w, id) {
        g.coup_pressure = 0.0;
        for e in g.pillars.iter_mut() {
            // The institution that moved is loyal to itself; the rest fall in
            // behind it, because the alternative has just been demonstrated.
            e.1 = if e.0 == pillar { 0.90 } else { 0.72 };
        }
        g.months_in_office = 0;
    }
    let _ = loyalty;
    w.headline(format!("COUP IN {}: {} removes the government.", id.name().to_uppercase(), name));
}

/// AI regimes pay their bills. A government that will not spend on the people
/// who could remove it is a government that gets removed, and the AI reaching
/// for the same command the player has is the only way that stays fair.
fn ai_government(w: &mut WorldState) {
    let ids: Vec<NationId> = w
        .nations
        .iter()
        .filter(|n| n.alive && Some(n.id) != w.player)
        .map(|n| n.id)
        .collect();
    for id in ids {
        if is_electoral(w, id) {
            continue;
        }
        let (weak, held) = {
            let g = match state(w, id) {
                Some(g) => g,
                None => continue,
            };
            (g.weakest_armed(), w.nation(id).political_capital)
        };
        if let Some((pillar, loyalty)) = weak {
            // Deliberately reluctant, for the same reason `ai_stratagems` keeps a
            // reserve: a regime that spends its whole standing on the palace
            // guard has nothing left to spend on the sphere. The first draft
            // bought at 22% a month whenever an institution dipped below 0.45,
            // and it quietly bankrupted the Soviet Union's foreign policy —
            // `a_pact_drags_a_great_power_into_a_war_it_did_not_start` fell from
            // 5/12 runs to 2/12, because Moscow was buying its own apparatus
            // instead of guaranteeing anybody. Patronage now waits until an
            // institution is genuinely close to moving.
            if loyalty < 0.35 && held > 55.0 {
                let _ = crate::apply_command(
                    w,
                    &crate::Command::SecurePillar { nation: id, pillar },
                );
            }
        }
    }
}

pub fn tick(w: &mut WorldState) {
    ensure_all(w);
    let ids: Vec<NationId> = w.nations.iter().filter(|n| n.alive).map(|n| n.id).collect();

    for id in ids.clone() {
        if state(w, id).is_none() {
            continue;
        }
        if let Some(g) = state_mut(w, id) {
            g.months_in_office = g.months_in_office.saturating_add(1);
        }

        if is_electoral(w, id) {
            // A regime that has just opened up owes the country a vote. Nothing
            // schedules this: it happens because authoritarianism fell, whether
            // through a stratagem, a revolution or a collapse.
            let unscheduled = state(w, id).map_or(false, |g| g.next_election.0 == 0);
            if unscheduled {
                let when = add_months(w.year, w.month, 18);
                if let Some(g) = state_mut(w, id) {
                    g.next_election = when;
                    g.pillars.clear();
                    g.coup_pressure = 0.0;
                }
                form_government(w, id, false);
                w.headline(format!("{} sets a date for its first free elections.", id.name()));
            }

            drift_support(w, id);

            // A government that has lost the country does not always last the
            // term. Israel's fell on a confidence motion in March 1990 and
            // Pakistan's was dismissed that August; both are this branch.
            let (fragile, months) = match state(w, id) {
                Some(g) => (
                    w.nation(id).stability < 32.0 || g.government_seats() < 0.40,
                    g.months_in_office,
                ),
                None => (false, 0),
            };
            if fragile && months >= 12 {
                w.headline(format!(
                    "The government of {} falls; the country goes to the polls.",
                    id.name()
                ));
                hold_election(w, id);
                continue;
            }

            let is_due = state(w, id).map_or(false, |g| due(w, g));
            if is_due {
                hold_election(w, id);
            }
        } else {
            // An authoritarian regime does not hold elections, and if it once
            // did, the parliament it had stops mattering.
            if let Some(g) = state_mut(w, id) {
                g.next_election = (0, 0);
                if g.pillars.is_empty() {
                    g.pillars = vec![];
                }
            }
            let needs_pillars = state(w, id).map_or(false, |g| g.pillars.is_empty());
            if needs_pillars {
                let seeded: Vec<(Pillar, f64)> = polity(id)
                    .map(|p| p.pillars.iter().map(|s| (s.pillar, 0.60)).collect())
                    .unwrap_or_default();
                if let Some(g) = state_mut(w, id) {
                    g.pillars = seeded;
                }
            }
            regime_tick(w, id);
            maybe_coup(w, id);
        }

        // The bill for the government you are running, paid every month out of
        // the same stock everything else is priced in.
        let cost = upkeep(w, id);
        if cost > 0.0 {
            let n = w.nation_mut(id);
            n.political_capital = (n.political_capital - cost).max(0.0);
        }
    }

    ai_government(w);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::init::world_1990;

    fn w1990() -> WorldState {
        world_1990(GameRules::default())
    }

    #[test]
    fn every_government_is_reachable_in_january_1990() {
        // The lesson that governs this whole branch: a mechanic the player
        // cannot reach from their seat is not a mechanic. Every nation on the
        // board must be able to answer "who governs here" on the first turn,
        // before a single month has ticked.
        let w = w1990();
        for n in w.nations.iter().filter(|n| n.alive) {
            let g = state(&w, n.id)
                .unwrap_or_else(|| panic!("{:?} has no government in Jan 1990", n.id));
            if is_electoral(&w, n.id) {
                assert!(
                    !g.coalition.is_empty(),
                    "{:?} is a democracy with nobody in office",
                    n.id
                );
                assert!(
                    g.government_seats() > 0.0,
                    "{:?}'s government holds no seats",
                    n.id
                );
                assert!(g.next_election.0 >= 1990, "{:?} has no election scheduled", n.id);
            } else {
                assert!(
                    !g.pillars.is_empty(),
                    "{:?} is a regime resting on nothing at all",
                    n.id
                );
            }
            // Saudi Arabia has no parties at all, which is the correct
            // transcription and not a gap: there was no assembly of any kind
            // until the Consultative Council of 1992.
            if !g.support.is_empty() {
                let total: f64 = g.support.iter().map(|(_, v)| *v).sum();
                assert!((total - 1.0).abs() < 1e-9, "{:?} support sums to {}", n.id, total);
            }
        }
    }

    #[test]
    fn the_party_table_is_data_and_not_guesswork() {
        // A guard on the transcription itself. Ids are what saves and commands
        // carry, so a duplicate would silently merge two parties.
        let mut seen: Vec<&str> = vec![];
        for pol in POLITIES {
            for s in pol.parties {
                assert!(!s.id.is_empty() && !s.name.is_empty(), "{:?} has a nameless party", pol.nation);
                assert!(
                    s.start > 0.0 && s.start <= 1.0,
                    "{:?}/{} has an impossible vote share {}",
                    pol.nation, s.id, s.start
                );
                assert!(!seen.contains(&s.id), "duplicate party id {}", s.id);
                seen.push(s.id);
            }
            // A couple of points of slack, because the published results these
            // are copied from are themselves rounded to a decimal place and a
            // table that had to sum exactly would be a table somebody had
            // adjusted.
            let total: f64 = pol.parties.iter().map(|s| s.start).sum();
            assert!(
                pol.parties.is_empty() || total <= 1.02,
                "{:?}'s parties won {:.1}% of the vote between them",
                pol.nation, total * 100.0
            );
        }
    }

    #[test]
    fn the_seat_formula_decides_whether_a_plurality_is_a_government() {
        // The single most consequential line in the module. Thatcher's 42% and
        // De Mita's 34% are both pluralities; one of them is a majority
        // government and the other is four weeks of negotiation, and the only
        // difference is how votes become seats.
        let w = w1990();
        let uk = state(&w, NationId::UK).unwrap();
        assert_eq!(uk.coalition.len(), 1, "first past the post produced a coalition");
        assert!(
            uk.government_seats() > 0.55,
            "42% of the vote did not manufacture a majority: {:.2}",
            uk.government_seats()
        );

        let it = state(&w, NationId::Italy).unwrap();
        assert!(
            it.coalition.len() >= 3,
            "Italy governed with {} parties; the pentapartito needed five",
            it.coalition.len()
        );
        assert!(
            !it.in_government("it_pci") && !it.in_government("it_msi"),
            "the conventio ad excludendum did not hold"
        );

        // Israel's 1% bar and no party near half: a government is arithmetic.
        let il = state(&w, NationId::Israel).unwrap();
        assert!(
            il.coalition.len() >= 4,
            "the 1988 Knesset produced a {}-party government",
            il.coalition.len()
        );
    }

    #[test]
    fn party_support_moves_with_prices_rather_than_with_a_slider() {
        // The claim the module exists to make. Nobody sets a popularity number:
        // the same government, in the same month, with prices running, loses
        // support — and it loses it to the family whose whole argument is sound
        // money rather than to whoever happens to be second.
        //
        // France, not Germany: the point only means something where the
        // opposition contains both a hard-money party and a left one, so that
        // there is a choice for the discontent to go to. Germany's 1990
        // opposition was the SPD and the Greens, and testing there would have
        // asserted nothing.
        let mut calm = w1990();
        let mut burning = w1990();
        for w in [&mut calm, &mut burning] {
            w.rules.ai_aggression = 0.0;
        }
        for _ in 0..48 {
            burning.nation_mut(NationId::France).inflation = 0.22;
            crate::tick_month(&mut burning, &[]);
            crate::tick_month(&mut calm, &[]);
        }
        let c = state(&calm, NationId::France).unwrap();
        let b = state(&burning, NationId::France).unwrap();
        assert!(
            b.support_of("fr_ps") < c.support_of("fr_ps") - 0.005,
            "four years of 22% inflation cost the governing party nothing: {:.3} vs {:.3}",
            b.support_of("fr_ps"),
            c.support_of("fr_ps")
        );
        let right = b.support_of("fr_rpr") - c.support_of("fr_rpr");
        let left = b.support_of("fr_pcf") - c.support_of("fr_pcf");
        assert!(right > 0.0, "an inflation crisis was worth nothing to the RPR");
        assert!(
            right > left,
            "runaway prices went to the communists rather than to the hard-money right: \
             RPR {:+.4} against PCF {:+.4}",
            right, left
        );
    }

    #[test]
    fn a_war_going_badly_is_worth_votes_to_the_nationalists() {
        // The other side of the same mechanism, and the one that makes the
        // Yugoslav successors legible: what a war does to a government at home
        // does not go to whoever happens to be second.
        let mut quiet = w1990();
        let mut bleeding = w1990();
        for w in [&mut quiet, &mut bleeding] {
            w.rules.ai_aggression = 0.0;
        }
        for _ in 0..36 {
            bleeding.nation_mut(NationId::France).war_exhaustion = 0.6;
            crate::tick_month(&mut bleeding, &[]);
            crate::tick_month(&mut quiet, &[]);
        }
        let q = state(&quiet, NationId::France).unwrap();
        let b = state(&bleeding, NationId::France).unwrap();
        assert!(
            b.support_of("fr_fn") > q.support_of("fr_fn"),
            "three years of a war going badly moved nothing to the Front National"
        );
        assert!(
            b.support_of("fr_ps") < q.support_of("fr_ps"),
            "the governing party was not charged for the war"
        );
    }

    #[test]
    fn a_coalition_costs_what_a_majority_does_not() {
        // The bite, stated as an ordering rather than a magic number: the
        // stretched multi-party governments cost real political capital every
        // month and the single-party majorities cost nothing, and it is the same
        // budget every command in the game is priced against.
        let w = w1990();
        for lonely in [NationId::UK, NationId::USA, NationId::Japan] {
            assert_eq!(strain(&w, lonely), 0.0, "{:?} paid for a majority", lonely);
            assert_eq!(upkeep(&w, lonely), 0.0);
            assert_eq!(standing_modifier(&w, lonely), 0.0);
        }
        for stretched in [NationId::Italy, NationId::Israel] {
            assert!(
                strain(&w, stretched) > 2.0,
                "{:?}'s coalition is free to hold: strain {:.2}",
                stretched,
                strain(&w, stretched)
            );
            assert!(upkeep(&w, stretched) > 0.4);
            assert!(standing_modifier(&w, stretched) < -4.0);
        }
        // And it is genuinely felt: Italy's standing after two years of holding
        // the pentapartito together is below what its own conditions would give
        // a government that did not have to.
        let mut a = w1990();
        a.rules.ai_aggression = 0.0;
        for _ in 0..24 {
            crate::tick_month(&mut a, &[]);
        }
        let n = a.nation(NationId::Italy);
        let unencumbered =
            crate::politics::seated_political_capital(n.stability, n.inflation, n.authoritarianism);
        assert!(
            n.political_capital < unencumbered,
            "the coalition cost Italy nothing: {:.1} held against {:.1} seated",
            n.political_capital,
            unencumbered
        );
    }

    #[test]
    fn a_player_can_widen_their_own_coalition_and_pay_for_it() {
        // The verb, and the price. Bringing another party in is bought out of
        // the same stock, and it makes the government both broader and dearer.
        let mut w = w1990();
        w.player = Some(NationId::Italy);
        let before_pc = w.nation(NationId::Italy).political_capital;
        let before_strain = strain(&w, NationId::Italy);
        let target = {
            let g = state(&w, NationId::Italy).unwrap();
            pol_parties(NationId::Italy)
                .iter()
                .find(|s| !g.in_government(s.id) && !s.pariah && g.seat_share(s.id) > 0.0)
                .map(|s| s.id)
                .expect("somebody is available")
        };
        crate::apply_command(
            &mut w,
            &crate::Command::InviteToGovernment {
                nation: NationId::Italy,
                party: target.to_string(),
            },
        )
        .expect("the invitation is affordable in 1990");
        assert!(state(&w, NationId::Italy).unwrap().in_government(target));
        assert!(
            w.nation(NationId::Italy).political_capital < before_pc,
            "a coalition partner joined for free"
        );
        assert!(strain(&w, NationId::Italy) > before_strain, "a wider cabinet cost no more to hold");

        // And a government cannot expel the party that leads it.
        let leader = state(&w, NationId::Italy).unwrap().leader().unwrap().to_string();
        assert!(expel(&mut w, NationId::Italy, &leader).is_err());
    }

    fn pol_parties(id: NationId) -> &'static [PartySpec] {
        polity(id).map(|p| p.parties).unwrap_or(&[])
    }

    #[test]
    fn a_regime_that_stops_paying_its_army_is_removed_by_it() {
        // The authoritarian half. Nothing schedules this and nothing names a
        // country: a regime whose armed institutions are going unpaid
        // accumulates pressure, and when it tops out somebody acts.
        let mut w = w1990();
        w.rules.ai_aggression = 0.0;
        w.player = Some(NationId::Iraq); // freeze Baghdad's own AI so it cannot pay
        let mut coup = None;
        for _ in 0..240 {
            // A defence budget cut to nothing, month after month.
            w.nation_mut(NationId::Iraq).mil_spend_gdp = 0.001;
            for h in crate::tick_month(&mut w, &[]) {
                if h.contains("COUP IN IRAQ") {
                    coup = Some(w.date_str());
                }
            }
            if coup.is_some() {
                break;
            }
        }
        assert!(coup.is_some(), "twenty years of an unpaid Republican Guard and nobody moved");

        // ...and a regime that keeps paying is not removed. Same nation, same
        // seed, the one difference being the budget.
        let mut safe = w1990();
        safe.rules.ai_aggression = 0.0;
        safe.player = Some(NationId::Iraq);
        for _ in 0..240 {
            safe.nation_mut(NationId::Iraq).mil_spend_gdp = 0.20;
            for h in crate::tick_month(&mut safe, &[]) {
                assert!(!h.contains("COUP IN IRAQ"), "a well-paid Republican Guard staged a coup");
            }
        }
    }

    #[test]
    fn buying_an_institution_is_a_real_price_and_not_a_button() {
        let mut w = w1990();
        let before_pc = w.nation(NationId::Iraq).political_capital;
        let before_debt = w.nation(NationId::Iraq).debt_gdp;
        let before = state(&w, NationId::Iraq).unwrap().loyalty(Pillar::Army);
        crate::apply_command(
            &mut w,
            &crate::Command::SecurePillar { nation: NationId::Iraq, pillar: Pillar::Army },
        )
        .expect("Baghdad can afford one payment in 1990");
        assert!(state(&w, NationId::Iraq).unwrap().loyalty(Pillar::Army) > before);
        assert!(w.nation(NationId::Iraq).political_capital < before_pc, "loyalty was free");
        assert!(w.nation(NationId::Iraq).debt_gdp > before_debt, "patronage cost no money");

        // A democracy has no such lever, and an unelected regime cannot hold an
        // election. Neither half can reach into the other.
        assert!(secure_pillar(&mut w, NationId::UK, Pillar::Army).is_err());
        assert!(call_election(&mut w, NationId::Iraq).is_err());
    }

    #[test]
    fn support_finds_an_equilibrium_rather_than_running_away() {
        // The bug this test exists for: an incumbent with a good record gained a
        // little support every month with nothing pulling back, so a party at
        // 60% went to 75%, then 92%, then the whole chamber, and Poland became a
        // one-party state by 1999 with no mechanism ever saying so.
        let mut w = w1990();
        w.rules.ai_aggression = 0.0;
        for _ in 0..12 * 40 {
            crate::tick_month(&mut w, &[]);
            for n in w.nations.iter().filter(|n| n.alive) {
                if !is_electoral(&w, n.id) {
                    continue;
                }
                // A one-party state that has opened up still has one party in
                // its table until somebody founds another; 100% there is the
                // correct reading, not a runaway.
                if polity(n.id).map_or(true, |p| p.parties.len() < 2) {
                    continue;
                }
                if let Some(g) = state(&w, n.id) {
                    for (pid, sup) in &g.support {
                        assert!(
                            *sup < 0.92,
                            "{:?}: {} holds {:.0}% of the electorate in {}",
                            n.id, pid, sup * 100.0, w.year
                        );
                        assert!(sup.is_finite() && *sup >= 0.0);
                    }
                }
            }
        }
    }

    #[test]
    fn opening_up_a_regime_produces_an_election_without_anything_scheduling_one() {
        // Emergence rather than script: a state that liberalises far enough owes
        // the country a vote, and the party table that was dormant becomes live.
        // No date, no country name — only the authoritarianism falling.
        let mut w = w1990();
        w.rules.ai_aggression = 0.0;
        w.nation_mut(NationId::Indonesia).authoritarianism = 0.30;
        let mut announced = false;
        let mut voted = false;
        for _ in 0..48 {
            for h in crate::tick_month(&mut w, &[]) {
                if h.contains("Indonesia sets a date") {
                    announced = true;
                }
                if h.starts_with("Indonesia votes") {
                    voted = true;
                }
            }
        }
        assert!(announced, "a liberalised regime never called an election");
        assert!(voted, "the election was announced and never held");
        let g = state(&w, NationId::Indonesia).unwrap();
        assert!(g.pillars.is_empty(), "an elected government is still resting on pillars");
        assert!(!g.coalition.is_empty(), "nobody took office after the vote");
    }
}

