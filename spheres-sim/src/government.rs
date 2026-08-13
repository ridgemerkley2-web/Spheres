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
    // Spain — Congress of Deputies, 29 October 1989: PSOE 39.6%, PP 25.8%,
    // IU 9.1%, CDS 7.9%, CiU 5.0%, PNV 1.2%, HB 1.1%. Gonzalez's third term,
    // and the last of his majorities. The next election was due by October 1993
    // and came on 6 June.
    // https://en.wikipedia.org/wiki/1989_Spanish_general_election
    Polity {
        nation: NationId::Spain,
        // The legal bar is 3% within each constituency, but the fifty-two
        // provincial districts are small enough that what actually decides who
        // sits is regional concentration, not a national share. The PNV took
        // 1.2% of the Spanish vote and five seats; a 5% national threshold
        // would delete the Basque and Catalan nationalists from the Cortes and
        // with them the entire arithmetic of Spanish minority government, which
        // is the same mistake the Italy block above records having avoided.
        system: Electoral::ProportionalLowBar,
        term_months: 48,
        next: (1993, 6),
        parties: &[
            p("es_psoe", "Spanish Socialist Workers' Party", "Partido Socialista Obrero Espanol", Family::SocialDemocratic, 0.396),
            p("es_pp", "People's Party", "Partido Popular", Family::Conservative, 0.258),
            p("es_iu", "United Left", "Izquierda Unida", Family::Communist, 0.091),
            p("es_cds", "Democratic and Social Centre", "Centro Democratico y Social", Family::Liberal, 0.079),
            p("es_ciu", "Convergence and Union", "Convergencia i Unio", Family::Regionalist, 0.050),
            p("es_pnv", "Basque Nationalist Party", "Partido Nacionalista Vasco", Family::Regionalist, 0.012),
            // The genuine article, and the reason the pariah flag is not a
            // French and Italian curiosity: Herri Batasuna was ETA's political
            // wing, and its deputies did not merely go uncourted — they refused
            // to take the seats they had won. Nobody in Madrid would govern
            // with them and they would not have sat if asked.
            pariah("es_hb", "Herri Batasuna", "Herri Batasuna", Family::Regionalist, 0.011),
        ],
        ruling: "the Congress of Deputies",
        pillars: &[],
    },

    // ======================================================================
    // Sub-Saharan Africa (branch feat/r-ssafrica)
    //
    // A note on method, because this region strains the table's stated rule
    // more than Europe does. The rule is "vote shares are from the last
    // national election before January 1990". In nine of these eleven
    // countries there was no such election in any meaningful sense: the last
    // poll was a single-list referendum on the one legal party, or a whites-
    // only franchise, or nothing at all. So the second half of the rule does
    // most of the work here — "where a party's founding postdates that
    // election its share is its result at the first one it contested" — and
    // the Nigeria block above is the precedent, since it carries the 12 June
    // 1993 result for parties that existed by decree in 1989.
    //
    // Where that has been done it is stated on the block, with the real
    // pre-1990 poll named beside it. The alternative was to give the sim an
    // Africa in which the ANC, UNITA and the SDF do not exist, which would
    // be a worse lie than a dated share.
    // ======================================================================

    // South Africa — and this is the hardest transcription in the block, so
    // both answers are on the record. The last election before the game opens
    // was for the House of Assembly on 6 September 1989: National Party 48.2%,
    // Conservative Party 31.2%, Democratic Party 20.0%. That was a whites-only
    // roll of about 3.2m voters in a country of some 40m, and its shares
    // describe who was allowed to vote rather than who held the country. The
    // shares entered instead are from 27 April 1994, the first election on a
    // universal franchise and the first that measured South Africa: ANC
    // 62.65%, NP 20.39%, IFP 10.54%, Freedom Front 2.17%, DP 1.73%, PAC 1.25%,
    // ACDP 0.45%. Every one of those organisations existed in January 1990 —
    // the ANC and PAC were unbanned on 2 February, four weeks into the game —
    // so this is the "first election it contested" rule rather than an
    // invention. `next` is (0, 0) and the pillars are non-empty because the
    // authoritarianism figure of 0.62 in southafrica.json sits above the 0.60
    // electoral ceiling, which is the correct reading of a state whose
    // government could not be removed by the governed.
    // https://en.wikipedia.org/wiki/1994_South_African_general_election
    Polity {
        nation: NationId::SouthAfrica,
        // Proportional with a very low effective bar: the 1994 election used
        // national and provincial party lists with 400 seats and no formal
        // threshold, which is how the ACDP took two seats on 0.45%. The same
        // choice as Israel's, and for the same reason — a high bar here would
        // delete exactly the small parties whose presence is the point.
        system: Electoral::ProportionalLowBar,
        term_months: 60,
        next: (0, 0),
        parties: &[
            p("za_anc", "African National Congress", "", Family::BigTent, 0.6265),
            p("za_np", "National Party", "Nasionale Party", Family::Conservative, 0.2039),
            p("za_ifp", "Inkatha Freedom Party", "iNkatha yeNkululeko yeSizwe", Family::Regionalist, 0.1054),
            // The Volksfront's electoral successor, and the reason
            // southafrica.json carries a separatism figure at all: the Freedom
            // Front's entire programme was an Afrikaner volkstaat.
            p("za_ff", "Freedom Front", "Vryheidsfront", Family::Nationalist, 0.0217),
            p("za_dp", "Democratic Party", "", Family::Liberal, 0.0173),
            p("za_pac", "Pan Africanist Congress", "", Family::Nationalist, 0.0125),
            p("za_acdp", "African Christian Democratic Party", "", Family::Religious, 0.0045),
        ],
        ruling: "the tricameral Parliament",
        pillars: &[
            pl(Pillar::Army, "the South African Defence Force"),
            pl(Pillar::Security, "the Security Branch of the South African Police"),
            pl(Pillar::Business, "the Chamber of Mines"),
        ],
    },

    // Ethiopia — the People's Democratic Republic, proclaimed on 12 September
    // 1987 when the Derg dissolved itself into a civilian constitution and the
    // National Shengo was elected on a single Workers' Party of Ethiopia list.
    // No competing organisation was legal, so one party at 1.00, which is the
    // same shape as the Vietnam block above. Mengistu announced a mixed economy
    // on 5 March 1990 and the WPE renamed itself in an attempt to broaden; both
    // were far too late, and Addis Ababa fell on 28 May 1991.
    Polity {
        nation: NationId::Ethiopia,
        system: Electoral::FirstPastThePost,
        term_months: 60,
        next: (0, 0),
        parties: &[
            p("et_wpe", "Workers' Party of Ethiopia", "Ye'Ityopya Serategnoch Party", Family::Communist, 1.00),
        ],
        ruling: "the Workers' Party of Ethiopia",
        pillars: &[
            // Named formations rather than "the army", per the rule this table
            // sets itself. The Second Revolutionary Army was the Eritrean
            // command and it was destroyed at Afabet in March 1988 and again at
            // Massawa in February 1990 — a pillar that had already given way
            // three weeks after the game opens.
            pl(Pillar::Army, "the Second Revolutionary Army"),
            pl(Pillar::Party, "the WPE Politburo"),
            pl(Pillar::Security, "the Ministry of Public and National Security"),
        ],
    },

    // Kenya — a one-party state in law. Section 2A of the constitution, added
    // by amendment in June 1982, made KANU the sole legal party; the general
    // election of 21 March 1988 was contested inside it by queue-voting in
    // public, which is how the mlolongo system got its name and its reputation.
    // Section 2A was repealed in December 1991 and Kenya voted multi-party in
    // December 1992.
    Polity {
        nation: NationId::Kenya,
        system: Electoral::FirstPastThePost,
        term_months: 60,
        next: (0, 0),
        parties: &[
            p("ke_kanu", "Kenya African National Union", "", Family::BigTent, 1.00),
        ],
        ruling: "the Kenya African National Union",
        pillars: &[
            pl(Pillar::Party, "the KANU Governing Council"),
            pl(Pillar::Security, "the Special Branch"),
            // The army is a pillar here in the strict sense the doc comment
            // means: it is what could remove the government, and in Kenya it
            // is what nearly did. The air force rose on 1 August 1982 and Moi
            // answered by disbanding it outright and rebuilding it under army
            // command.
            pl(Pillar::Army, "the Kenya Army"),
        ],
    },

    // Ghana — the Provisional National Defence Council, eight years in and
    // with no legislature of any kind. Parties had been banned since Rawlings
    // took power on 31 December 1981, so there was no pre-1990 election to
    // transcribe; the shares are the presidential poll of 3 November 1992, the
    // first vote after the ban was lifted in May 1992 — Rawlings 58.4%,
    // Adu Boahen 30.3%, Limann 6.7%, Darko 2.8%, Erskine 1.8%.
    // https://en.wikipedia.org/wiki/1992_Ghanaian_presidential_election
    Polity {
        nation: NationId::Ghana,
        system: Electoral::FirstPastThePost,
        term_months: 48,
        next: (0, 0),
        parties: &[
            p("gh_ndc", "National Democratic Congress", "", Family::SocialDemocratic, 0.584),
            p("gh_npp", "New Patriotic Party", "", Family::Liberal, 0.303),
            p("gh_pnc", "People's National Convention", "", Family::SocialDemocratic, 0.067),
            p("gh_nip", "National Independence Party", "", Family::Liberal, 0.028),
            p("gh_php", "People's Heritage Party", "", Family::SocialDemocratic, 0.018),
        ],
        ruling: "the Provisional National Defence Council",
        pillars: &[
            pl(Pillar::Army, "the Ghana Armed Forces"),
            pl(Pillar::Security, "the Bureau of National Investigations"),
            // Street-level surveillance and rationing committees, and the
            // organisation that made the PNDC something other than a junta.
            pl(Pillar::Party, "the Committees for the Defence of the Revolution"),
        ],
    },

    // Zaire — the Popular Movement of the Revolution, sole legal party since
    // 1967 and written into the constitution as the party every Zairean
    // belonged to by birth. The last election, in September 1987, was a single
    // MPR list. On 24 April 1990 Mobutu announced the Third Republic and a
    // three-party system; the UDPS, founded illegally by thirteen dissident
    // parliamentarians in 1982, became legal and never got the election it was
    // promised — the Sovereign National Conference of 1991-92 ended in
    // deadlock and the first real vote in Congo was in 2006. So the table
    // carries the MPR alone, and the point of the block is that the party
    // slot is empty of everything else.
    Polity {
        nation: NationId::Zaire,
        system: Electoral::FirstPastThePost,
        term_months: 60,
        next: (0, 0),
        parties: &[
            p("zr_mpr", "Popular Movement of the Revolution", "Mouvement Populaire de la Revolution", Family::BigTent, 1.00),
        ],
        ruling: "the Popular Movement of the Revolution",
        pillars: &[
            // The distinction this table insists on, and Zaire is the textbook
            // case of it: the Forces Armees Zairoises could not defend the
            // country and were not meant to. The Division Speciale
            // Presidentielle was Israeli-trained, paid, and the only formation
            // that mattered — the FAZ mutinied over pay in September 1991 and
            // looted Kinshasa, and the DSP put it down.
            pl(Pillar::Army, "the Division Speciale Presidentielle"),
            pl(Pillar::Security, "the Service National d'Intelligence et de Protection"),
            // The copper monopoly was the fiscal state. Its Kamoto gallery
            // collapsed in September 1990 and the government's revenue went
            // with it.
            pl(Pillar::Business, "Gecamines"),
        ],
    },

    // Angola — the MPLA-Workers' Party, Marxist-Leninist and sole legal party
    // since independence in November 1975. No election had ever been held, so
    // the shares are the legislative poll of 29-30 September 1992, the first
    // one ever and the one whose result Savimbi rejected: MPLA 53.74%, UNITA
    // 34.10%, FNLA 2.40%, PLD 2.39%, PRS 2.27%. The war resumed within weeks
    // and killed more people in the two years after that election than in the
    // sixteen before it.
    // https://en.wikipedia.org/wiki/1992_Angolan_general_election
    Polity {
        nation: NationId::Angola,
        system: Electoral::ProportionalLowBar,
        term_months: 48,
        next: (0, 0),
        parties: &[
            p("ao_mpla", "MPLA", "Movimento Popular de Libertacao de Angola", Family::SocialDemocratic, 0.5374),
            // NOT marked pariah, deliberately. A pariah in this table is a
            // party inside a parliament that nobody will govern with — the
            // Italian, French and Spanish cordons. UNITA was an armed rival
            // that took a third of the vote and then went back to the bush.
            // That is a civil war, which the model has other machinery for,
            // and the roster's rule that no fourth cordon sanitaire be
            // invented is the right rule here.
            p("ao_unita", "UNITA", "Uniao Nacional para a Independencia Total de Angola", Family::Nationalist, 0.3410),
            p("ao_fnla", "FNLA", "Frente Nacional de Libertacao de Angola", Family::Nationalist, 0.0240),
            p("ao_pld", "Liberal Democratic Party", "Partido Liberal Democratico", Family::Liberal, 0.0239),
            p("ao_prs", "Social Renewal Party", "Partido de Renovacao Social", Family::SocialDemocratic, 0.0227),
        ],
        ruling: "the MPLA Political Bureau",
        pillars: &[
            pl(Pillar::Army, "the Forcas Armadas Populares de Libertacao de Angola"),
            pl(Pillar::Party, "the MPLA Political Bureau"),
            pl(Pillar::Security, "the Ministerio da Seguranca do Estado"),
        ],
    },

    // Zimbabwe — the only nation in this block below the electoral ceiling
    // besides Senegal, and the call is argued in full in zimbabwe.json. The
    // shares are a real, contested, pre-1990 election: the House of Assembly
    // common roll of 28-30 March 1990, ZANU-PF 80.6%, ZUM 16.5%,
    // ZANU-Ndonga 1.9%, UANC 0.6%. Mugabe put a one-party state to the
    // ZANU-PF politburo that September and lost the argument; the next
    // parliamentary election was duly held on 8-9 April 1995, which is what
    // `next` carries.
    // https://en.wikipedia.org/wiki/1990_Zimbabwean_general_election
    Polity {
        nation: NationId::Zimbabwe,
        system: Electoral::FirstPastThePost,
        term_months: 60,
        next: (1995, 4),
        parties: &[
            // BigTent rather than Communist, though ZANU-PF still called
            // itself Marxist-Leninist in 1990: what it actually was is the
            // thing this family describes — a liberation front that had
            // absorbed its rival wholesale under the Unity Accord of 22
            // December 1987 and contained everything from war veterans to
            // the commercial farming lobby's accommodationists.
            p("zw_zanupf", "Zimbabwe African National Union - Patriotic Front", "", Family::BigTent, 0.806),
            p("zw_zum", "Zimbabwe Unity Movement", "", Family::Liberal, 0.165),
            p("zw_zanun", "ZANU-Ndonga", "", Family::Nationalist, 0.019),
            p("zw_uanc", "United African National Council", "", Family::Conservative, 0.006),
        ],
        ruling: "the House of Assembly",
        pillars: &[
            // A pillar on an electoral government, which the Turkey block
            // above establishes is legal and sometimes necessary. The CIO
            // reported to the prime minister, not to parliament, and it ran
            // Gukurahundi in Matabeleland between 1983 and 1987. An
            // accountable legislature and an unaccountable intelligence
            // service in the same state is the whole of Zimbabwe's 0.58.
            pl(Pillar::Security, "the Central Intelligence Organisation"),
        ],
    },

    // Tanzania — Chama Cha Mapinduzi, sole legal party since the merger of
    // TANU and the Afro-Shirazi Party in February 1977, and the election of
    // 28 October 1990 (which returned Ali Hassan Mwinyi for a second term)
    // was a single-party one. Nyerere gave up the party chairmanship that
    // August and told the CCM to consider opposition parties; the Nyalali
    // Commission reported in 1991 and the constitution was amended in May
    // 1992, which is the liberalisation this block is waiting for.
    Polity {
        nation: NationId::Tanzania,
        system: Electoral::FirstPastThePost,
        term_months: 60,
        next: (0, 0),
        parties: &[
            p("tz_ccm", "Chama Cha Mapinduzi", "Chama Cha Mapinduzi", Family::BigTent, 1.00),
        ],
        ruling: "Chama Cha Mapinduzi",
        pillars: &[
            // The party genuinely outranked the state here, which is not true
            // of most of the single-party regimes in this block: the CCM's
            // National Executive Committee chose the sole presidential
            // candidate and the electorate confirmed him.
            pl(Pillar::Party, "the CCM National Executive Committee"),
            pl(Pillar::Army, "the Tanzania People's Defence Force"),
            pl(Pillar::Security, "the National Security Service"),
        ],
    },

    // Uganda — the Movement system, which is not quite a one-party state and
    // is certainly not a multi-party one. Parties were never banned; they
    // were forbidden to campaign, field candidates or hold rallies, and
    // elections to the National Resistance Council on 11-28 February 1989
    // were fought on "individual merit" with no party labels at all. There
    // is therefore no pre-1990 vote share to transcribe. The shares entered
    // are the presidential election of 9 May 1996, the first national vote
    // Uganda held: Museveni 74.2%, Ssemogerere 23.7%, Mayanja 2.2%.
    Polity {
        nation: NationId::Uganda,
        system: Electoral::FirstPastThePost,
        term_months: 60,
        next: (0, 0),
        parties: &[
            p("ug_nrm", "National Resistance Movement", "", Family::BigTent, 0.742),
            // Ssemogerere led the Democratic Party and stood in 1996 for a
            // DP-UPC alliance, which is why his share sits against the DP.
            p("ug_dp", "Democratic Party", "", Family::ChristianDemocratic, 0.237),
            p("ug_ku", "Kibirige Mayanja's campaign", "", Family::Liberal, 0.022),
        ],
        ruling: "the National Resistance Movement",
        pillars: &[
            pl(Pillar::Army, "the National Resistance Army"),
            pl(Pillar::Party, "the National Resistance Council"),
            pl(Pillar::Security, "the Internal Security Organisation"),
        ],
    },

    // Senegal — a genuine multi-party democracy and the only one in this
    // block with an ordinary pre-1990 election to transcribe. National
    // Assembly, 28 February 1988: Parti Socialiste 71.3%, Parti Democratique
    // Senegalais 24.7%, and the small left lists behind them. The result was
    // disputed violently, Dakar went under a state of emergency and Wade was
    // convicted and given a suspended sentence — and then joined a government
    // of national unity in April 1991, which is the Senegalese pattern. The
    // Assembly's five-year term ran from February 1988, so the next was due
    // in early 1993; it was held on 9 May.
    // https://en.wikipedia.org/wiki/1988_Senegalese_general_election
    Polity {
        nation: NationId::Senegal,
        // Mixed in reality — 70 seats by departmental majority list and 50 by
        // national proportional list. Proportional is the closer of the two
        // available choices and it is the national list that decides anything:
        // the PS swept nearly every department, and without the proportional
        // half the PDS's quarter of the vote would have produced almost no
        // seats at all.
        system: Electoral::Proportional,
        term_months: 60,
        next: (1993, 2),
        parties: &[
            p("sn_ps", "Socialist Party", "Parti Socialiste du Senegal", Family::SocialDemocratic, 0.713),
            p("sn_pds", "Senegalese Democratic Party", "Parti Democratique Senegalais", Family::Liberal, 0.247),
            p("sn_ldmpt", "Democratic League - Labour Party Movement", "Ligue Democratique - Mouvement pour le Parti du Travail", Family::Communist, 0.014),
            p("sn_pit", "Party of Independence and Labour", "Parti de l'Independance et du Travail", Family::Communist, 0.013),
        ],
        ruling: "the National Assembly",
        pillars: &[],
    },

    // Cameroon — the Cameroon People's Democratic Movement, sole legal party
    // (as the Cameroon National Union until 1985) from 1966 until the law of
    // 19 December 1990. The last poll before the game opens was the single-
    // list legislative election of 24 April 1988. The shares entered are the
    // first multi-party legislative election, 1 March 1992: CPDM 45.4%, UNDP
    // 18.9%, UPC 12.7%, MDR 6.2%. The Social Democratic Front — launched at
    // Bamenda on 26 May 1990 with six people shot dead at the rally, and the
    // organisation that broke the single-party state — boycotted that
    // election, which is why the party with the best claim to have earned a
    // place in this table does not appear in it. Recorded here rather than
    // padded in.
    Polity {
        nation: NationId::Cameroon,
        system: Electoral::Proportional,
        term_months: 60,
        next: (0, 0),
        parties: &[
            p("cm_cpdm", "Cameroon People's Democratic Movement", "Rassemblement Democratique du Peuple Camerounais", Family::BigTent, 0.454),
            p("cm_undp", "National Union for Democracy and Progress", "Union Nationale pour la Democratie et le Progres", Family::Liberal, 0.189),
            p("cm_upc", "Cameroon People's Union", "Union des Populations du Cameroun", Family::SocialDemocratic, 0.127),
            p("cm_mdr", "Movement for the Defence of the Republic", "Mouvement pour la Defense de la Republique", Family::Conservative, 0.062),
        ],
        ruling: "the Cameroon People's Democratic Movement",
        pillars: &[
            // The formation that tried to remove Biya on 6 April 1984 and
            // failed after two days of fighting in Yaounde, and was rebuilt
            // afterwards as the thing that keeps him.
            pl(Pillar::Army, "the Garde Presidentielle"),
            pl(Pillar::Party, "the CPDM Central Committee"),
            pl(Pillar::Security, "the Delegation Generale a la Surete Nationale"),
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

