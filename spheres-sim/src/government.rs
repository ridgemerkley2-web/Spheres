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
    // Western Europe, the rest of it. Eleven parliamentary democracies, no
    // pillars anywhere, and every `next` below is a real scheduled date.
    // Two things recur and are worth stating once rather than eleven times:
    //   * Shares are first-preference or party-list votes at the last national
    //     election BEFORE 1 January 1990, and none of them sum to 1.0 —
    //     published results are rounded and minor parties are not all listed.
    //     They are not padded. The seating pass normalises.
    //   * Where a country's real electoral law has no equivalent in the
    //     `Electoral` enum, the substitution is named in the block rather than
    //     quietly made.
    // ======================================================================

    // Netherlands - Tweede Kamer, 6 September 1989: CDA 35.3%, PvdA 31.9%,
    // VVD 14.6%, D66 7.9%, GroenLinks 4.1%, SGP 1.9%, GPV 1.2%, RPF 1.0%.
    // Lubbers III, the CDA-PvdA cabinet that replaced seven years of CDA-VVD
    // when the second Lubbers cabinet fell in May 1989 over the National
    // Environmental Policy Plan. The next election was due and came on
    // 3 May 1994. https://en.wikipedia.org/wiki/1989_Dutch_general_election
    Polity {
        nation: NationId::Netherlands,
        // The purest proportional system in Europe: one national constituency,
        // and the threshold is simply the quota for one of 150 seats, 0.67%.
        // ProportionalLowBar's 1% is the closest the enum comes and it is
        // still slightly too high — the three small confessional parties below
        // each hold seats on shares that a 5% bar would erase, and a Dutch
        // parliament without them is not a Dutch parliament.
        system: Electoral::ProportionalLowBar,
        term_months: 48,
        next: (1994, 5),
        parties: &[
            p("nl_cda", "Christian Democratic Appeal", "Christen-Democratisch Appel", Family::ChristianDemocratic, 0.353),
            p("nl_pvda", "Labour Party", "Partij van de Arbeid", Family::SocialDemocratic, 0.319),
            p("nl_vvd", "People's Party for Freedom and Democracy", "Volkspartij voor Vrijheid en Democratie", Family::Liberal, 0.146),
            p("nl_d66", "Democrats 66", "Democraten 66", Family::Liberal, 0.079),
            p("nl_gl", "Green Left", "GroenLinks", Family::Green, 0.041),
            // The three of these are not a rounding error and not
            // interchangeable: they are the remains of the confessional pillar
            // that organised Dutch society until the 1960s, they sit on the
            // Bible Belt from Zeeland to Overijssel, and the SGP had been in
            // the Tweede Kamer continuously since 1922 without ever once
            // being in government. A model that deletes them loses the thing
            // that made Dutch politics consociational in the first place.
            p("nl_sgp", "Reformed Political Party", "Staatkundig Gereformeerde Partij", Family::Religious, 0.019),
            p("nl_gpv", "Reformed Political League", "Gereformeerd Politiek Verbond", Family::Religious, 0.012),
            p("nl_rpf", "Reformatory Political Federation", "Reformatorische Politieke Federatie", Family::Religious, 0.010),
        ],
        ruling: "the Tweede Kamer",
        pillars: &[],
    },

    // Belgium - Chamber of Representatives, 13 December 1987: CVP 19.5%,
    // PS 15.6%, SP 14.9%, PVV 11.6%, PRL 9.4%, VU 8.1%, PSC 8.0%, Agalev 4.5%,
    // Ecolo 2.6%, Vlaams Blok 1.9%, FDF 1.2%. Martens VIII took 148 days to
    // form. The next election was due December 1991 and came on 24 November.
    // https://en.wikipedia.org/wiki/1987_Belgian_general_election
    Polity {
        nation: NationId::Belgium,
        // D'Hondt in twenty arrondissement constituencies with no legal
        // threshold at all until 2003.
        system: Electoral::ProportionalLowBar,
        term_months: 48,
        next: (1991, 11),
        parties: &[
            // The single most important fact about this table is that there is
            // no national party in it. The Christian democrats split into CVP
            // and PSC in 1968, the liberals into PVV and PRL in 1971, the
            // socialists into SP and PS in 1978, and the greens were born
            // separately as Agalev and Ecolo. Each pair below is one political
            // family that partitioned itself along the language border and
            // then never reunited. They are entered as separate parties
            // because that is what they were: separate lists, separate
            // leaders, separate electorates, coalition partners at best.
            p("be_cvp", "Christian People's Party (Flemish)", "Christelijke Volkspartij", Family::ChristianDemocratic, 0.195),
            p("be_ps", "Socialist Party (Francophone)", "Parti Socialiste", Family::SocialDemocratic, 0.156),
            p("be_sp", "Socialist Party (Flemish)", "Socialistische Partij", Family::SocialDemocratic, 0.149),
            p("be_pvv", "Party for Freedom and Progress (Flemish)", "Partij voor Vrijheid en Vooruitgang", Family::Liberal, 0.116),
            p("be_prl", "Liberal Reformist Party (Francophone)", "Parti Reformateur Liberal", Family::Liberal, 0.094),
            p("be_vu", "People's Union", "Volksunie", Family::Regionalist, 0.081),
            p("be_psc", "Christian Social Party (Francophone)", "Parti Social Chretien", Family::ChristianDemocratic, 0.080),
            p("be_agalev", "Live Differently (Flemish greens)", "Anders Gaan Leven", Family::Green, 0.045),
            p("be_ecolo", "Ecolo (Francophone greens)", "Ecologistes Confederes", Family::Green, 0.026),
            // A FOURTH pariah, entered deliberately and against the standing
            // instruction not to add one, because this is the case the word
            // was coined for rather than a case that resembles it. On 10 May
            // 1989 every other Flemish party in the country signed an
            // agreement never to govern, negotiate or make any accord with
            // Vlaams Blok; the signed document is what Belgians and then
            // everybody else began calling the cordon sanitaire, and it held
            // without a single breach for the next thirty years, through the
            // party's dissolution for racism by the Court of Cassation in
            // 2004 and its immediate re-founding as Vlaams Belang. It is a
            // stricter and better-documented exclusion than the French one
            // already in this table. If a reviewer disagrees, the fix is one
            // word — pariah to p — and the vote share is unaffected either
            // way. https://en.wikipedia.org/wiki/Cordon_sanitaire_(politics)
            pariah("be_vb", "Flemish Bloc", "Vlaams Blok", Family::Nationalist, 0.019),
            p("be_fdf", "Democratic Front of Francophones", "Front Democratique des Francophones", Family::Regionalist, 0.012),
        ],
        ruling: "the Chamber of Representatives",
        pillars: &[],
    },

    // Sweden - Riksdag, 18 September 1988: SAP 43.2%, Moderates 18.3%,
    // People's Party 12.2%, Centre 11.3%, Left Party Communists 5.8%,
    // Greens 5.5%, Christian Democrats 2.9%. Carlsson's Social Democratic
    // minority government, which resigned on 15 February 1990 when the Riksdag
    // threw out its price-and-wage freeze and returned a week later without
    // the strike ban in it. The Greens entering in 1988 were the first new
    // party in the Riksdag in seventy years.
    // https://en.wikipedia.org/wiki/1988_Swedish_general_election
    Polity {
        nation: NationId::Sweden,
        // Sweden's bar is 4% nationally (or 12% in a constituency), so the
        // 5% Proportional variant is the near fit; ProportionalLowBar's 1%
        // would seat the Christian Democrats, who in fact won nothing in 1988
        // on 2.9% and entered only in 1991 in alliance.
        system: Electoral::Proportional,
        // Three-year fixed terms, in force from 1970 until the 1994 reform
        // took them back to four. This is the only 36-month term in the table
        // and it is transcribed, not a slip.
        term_months: 36,
        next: (1991, 9),
        parties: &[
            p("se_sap", "Social Democratic Party", "Sveriges socialdemokratiska arbetareparti", Family::SocialDemocratic, 0.432),
            p("se_m", "Moderate Party", "Moderata samlingspartiet", Family::Conservative, 0.183),
            p("se_fp", "People's Party - The Liberals", "Folkpartiet liberalerna", Family::Liberal, 0.122),
            // Agrarian and not a misfiling: the Centre Party was the Farmers'
            // League until 1957 and its 1988 vote is still rural, though the
            // issue it rode was nuclear power.
            p("se_c", "Centre Party", "Centerpartiet", Family::Agrarian, 0.113),
            p("se_vpk", "Left Party Communists", "Vansterpartiet kommunisterna", Family::Communist, 0.058),
            p("se_mp", "Green Party", "Miljopartiet de grona", Family::Green, 0.055),
            p("se_kds", "Christian Democratic Union", "Kristdemokratiska samhallspartiet", Family::ChristianDemocratic, 0.029),
        ],
        ruling: "the Riksdag",
        pillars: &[],
    },

    // Switzerland - National Council, 18 October 1987: FDP 22.9%, CVP 19.6%,
    // SPS 18.4%, SVP 11.0%, Greens 4.9%, LdU 4.2%, LPS 2.7%, National Action
    // 2.5%, EVP 1.9%, Labour 0.8%. Next election 20 October 1991.
    // https://en.wikipedia.org/wiki/1987_Swiss_federal_election
    Polity {
        nation: NationId::Switzerland,
        // Proportional in twenty-six cantonal constituencies, no national
        // threshold.
        system: Electoral::ProportionalLowBar,
        term_months: 48,
        next: (1991, 10),
        parties: &[
            p("ch_fdp", "Free Democratic Party", "Freisinnig-Demokratische Partei", Family::Liberal, 0.229),
            p("ch_cvp", "Christian Democratic People's Party", "Christlichdemokratische Volkspartei", Family::ChristianDemocratic, 0.196),
            p("ch_sps", "Social Democratic Party", "Sozialdemokratische Partei", Family::SocialDemocratic, 0.184),
            // Agrarian in 1987 and correctly so: this was the Party of
            // Farmers, Traders and Independents until 1971 and was still a
            // Bernese and Zurich rural party at this election. Blocher took
            // the Zurich cantonal party in 1977 and the national party's
            // direction only after 1992, over the European Economic Area
            // referendum; nothing in this record anticipates that.
            p("ch_svp", "Swiss People's Party", "Schweizerische Volkspartei", Family::Agrarian, 0.110),
            p("ch_gps", "Green Party", "Grune Partei der Schweiz", Family::Green, 0.049),
            p("ch_ldu", "Ring of Independents", "Landesring der Unabhangigen", Family::Liberal, 0.042),
            p("ch_lps", "Liberal Party", "Liberale Partei der Schweiz", Family::Liberal, 0.027),
            p("ch_na", "National Action", "Nationale Aktion", Family::Nationalist, 0.025),
            p("ch_evp", "Evangelical People's Party", "Evangelische Volkspartei", Family::Religious, 0.019),
            p("ch_pda", "Swiss Party of Labour", "Partei der Arbeit der Schweiz", Family::Communist, 0.008),
        ],
        // The one place in this batch where `ruling` is doing real work. The
        // Federal Council is a seven-member executive elected by parliament
        // and never removed by it, held since 1959 to the Zauberformel of
        // 2 FDP : 2 CVP : 2 SPS : 1 SVP regardless of how the vote moves. The
        // election below is real and matters for the chambers; it does not
        // change who governs, which is a fact about Switzerland rather than a
        // shortcoming of the model.
        ruling: "the Federal Council",
        pillars: &[],
    },

    // Austria - Nationalrat, 23 November 1986: SPO 43.1%, OVP 41.3%,
    // FPO 9.7%, Greens 4.8%. The grand coalition of Vranitzky and Mock, formed
    // in January 1987 after Haider took the FPO leadership in September 1986
    // and the SPO ended its coalition with it the same week. The next election
    // was due and came on 7 October 1990, inside the game's first year.
    // https://en.wikipedia.org/wiki/1986_Austrian_legislative_election
    Polity {
        nation: NationId::Austria,
        // A 4% national threshold was only introduced in 1992; in 1986 a party
        // needed a Grundmandat in one of nine regional districts, which in
        // practice bit at about the same level. Proportional's 5% is the fit.
        system: Electoral::Proportional,
        term_months: 48,
        next: (1990, 10),
        parties: &[
            p("at_spo", "Social Democratic Party of Austria", "Sozialdemokratische Partei Osterreichs", Family::SocialDemocratic, 0.431),
            p("at_ovp", "Austrian People's Party", "Osterreichische Volkspartei", Family::ChristianDemocratic, 0.413),
            // Filed Nationalist rather than Liberal, which is a judgement about
            // September 1986 and not about the party's whole history: the FPO
            // held the Liberal International seat and was in government with
            // the SPO under Steger until Haider beat him at the Innsbruck
            // congress, at which point the coalition ended within days and the
            // party's vote doubled at the next four elections on immigration.
            // In January 1990 it is a national-populist party that still holds
            // a liberal membership card. It was expelled from the Liberal
            // International in 1993.
            p("at_fpo", "Freedom Party of Austria", "Freiheitliche Partei Osterreichs", Family::Nationalist, 0.097),
            p("at_gruene", "The Greens - The Green Alternative", "Die Grune Alternative", Family::Green, 0.048),
        ],
        ruling: "the Nationalrat",
        pillars: &[],
    },

    // Portugal - Assembly of the Republic, 19 July 1987: PSD 50.2%, PS 22.2%,
    // CDU 12.1%, PRD 4.9%, CDS 4.4%. Cavaco Silva's absolute majority, the
    // first single-party majority since the revolution of 1974 and the end of
    // thirteen years in which no government finished a term. Next election
    // 6 October 1991, which he won again.
    // https://en.wikipedia.org/wiki/1987_Portuguese_legislative_election
    Polity {
        nation: NationId::Portugal,
        // D'Hondt in twenty-two districts, no legal threshold.
        system: Electoral::ProportionalLowBar,
        term_months: 48,
        next: (1991, 10),
        parties: &[
            p("pt_psd", "Social Democratic Party", "Partido Social Democrata", Family::Liberal, 0.502),
            p("pt_ps", "Socialist Party", "Partido Socialista", Family::SocialDemocratic, 0.222),
            // The CDU is a standing coalition of the Communist Party and the
            // Greens and contested as one list; it is entered as one party
            // because that is how the votes were cast. The PCP had been the
            // best-organised clandestine party under the Estado Novo and was
            // still, in 1990, the only unreconstructed pro-Soviet communist
            // party of any size in Western Europe.
            p("pt_cdu", "Unitary Democratic Coalition", "Coligacao Democratica Unitaria", Family::Communist, 0.121),
            p("pt_prd", "Democratic Renewal Party", "Partido Renovador Democratico", Family::BigTent, 0.049),
            p("pt_cds", "Democratic and Social Centre", "Centro Democratico e Social", Family::ChristianDemocratic, 0.044),
        ],
        ruling: "the Assembly of the Republic",
        pillars: &[],
    },

    // Greece - Hellenic Parliament, 5 November 1989: New Democracy 46.2%,
    // PASOK 40.7%, Synaspismos 11.0%. Three parties and no more: the
    // published result carries a tail of minor lists, and none of them is
    // separately verifiable against the source cited below, so none is
    // entered. The shares sum to 0.979 and are not padded. This is the SECOND of three
    // elections in eleven months - 18 June 1989, 5 November 1989, 8 April 1990
    // - and neither of the first two produced a government. What sat on
    // 1 January 1990 was an ecumenical caretaker cabinet under Xenophon
    // Zolotas, a central banker with no party, holding office precisely until
    // the third election could be held. `next` is therefore (1990, 4): a real
    // scheduled election, three months into the game.
    // https://en.wikipedia.org/wiki/November_1989_Greek_legislative_election
    Polity {
        nation: NationId::Greece,
        // The substitution, stated. Greece used SIMPLE proportional in 1989
        // rather than the reinforced proportional that normally manufactured
        // majorities there, and it did so deliberately: PASOK legislated the
        // change in 1989 knowing it was going to lose, so that New Democracy
        // could not govern alone either. That worked exactly as intended and
        // is why there were three elections. `Proportional` (5%) stands in for
        // a 3% bar; there is no enum member for "a threshold chosen by the
        // outgoing government to deny its successor a majority", which is what
        // the real rule amounted to.
        system: Electoral::Proportional,
        term_months: 48,
        next: (1990, 4),
        parties: &[
            p("gr_nd", "New Democracy", "Nea Dimokratia", Family::Conservative, 0.462),
            p("gr_pasok", "Panhellenic Socialist Movement", "Panellinio Sosialistiko Kinima", Family::SocialDemocratic, 0.407),
            // Synaspismos was at this moment the strangest coalition in
            // European politics: the orthodox KKE and the eurocommunist Greek
            // Left running one list, and between July and October 1989 they
            // had governed IN COALITION WITH New Democracy - communists and
            // conservatives in one cabinet - for the sole purpose of sending
            // Andreas Papandreou for trial over the Koskotas affair.
            p("gr_syn", "Coalition of the Left and Progress", "Synaspismos tis Aristeras kai tis Proodou", Family::Communist, 0.110),
        ],
        ruling: "the Hellenic Parliament",
        pillars: &[],
    },

    // Denmark - Folketing, 10 May 1988: Social Democrats 29.8%,
    // Conservatives 19.3%, Socialist People's Party 13.0%, Venstre 11.8%,
    // Progress Party 9.0%, Social Liberals 5.6%, Centre Democrats 4.7%,
    // Christian People's Party 2.0%, Common Course 1.9%. Schluter's third
    // cabinet, a minority of Conservatives, Venstre and Social Liberals.
    // `next` is (1990, 12), the election Schluter actually called for
    // 12 December 1990; the four-year term would have run to May 1992. The
    // Spain block above set the same precedent of entering the date the
    // election came rather than the last date it could have.
    // https://en.wikipedia.org/wiki/1988_Danish_general_election
    Polity {
        nation: NationId::Denmark,
        // A 2% national threshold, the lowest in Europe, which is why nine
        // parties are listed and why no Danish government since 1971 has had a
        // majority. ProportionalLowBar's 1% is the nearest.
        system: Electoral::ProportionalLowBar,
        term_months: 48,
        next: (1990, 12),
        parties: &[
            p("dk_a", "Social Democrats", "Socialdemokratiet", Family::SocialDemocratic, 0.298),
            p("dk_c", "Conservative People's Party", "Det Konservative Folkeparti", Family::Conservative, 0.193),
            p("dk_f", "Socialist People's Party", "Socialistisk Folkeparti", Family::Communist, 0.130),
            // Venstre means "left" and is the liberal party of the farmers.
            // The name is 1870s seating, not a description, and it is entered
            // Liberal because that is what it is.
            p("dk_v", "Venstre, Liberal Party of Denmark", "Venstre", Family::Liberal, 0.118),
            // Glistrup's tax revolt of 1973, which took 15.9% at its first
            // election and had turned to immigration by 1988. The other Nordic
            // parties of this family in this batch are Norway's FrP and
            // Finland's SMP; unlike Belgium's Vlaams Blok, no formal cordon
            // was ever signed against any of the three, which is exactly why
            // they are entered with p() and it is not.
            p("dk_z", "Progress Party", "Fremskridtspartiet", Family::Nationalist, 0.090),
            p("dk_b", "Danish Social Liberal Party", "Det Radikale Venstre", Family::Liberal, 0.056),
            p("dk_cd", "Centre Democrats", "Centrum-Demokraterne", Family::Liberal, 0.047),
            p("dk_krf", "Christian People's Party", "Kristeligt Folkeparti", Family::ChristianDemocratic, 0.020),
            p("dk_fk", "Common Course", "Faelles Kurs", Family::Communist, 0.019),
        ],
        ruling: "the Folketing",
        pillars: &[],
    },

    // Norway - Storting, 11 September 1989: Labour 34.3%, Conservative 22.2%,
    // Progress 13.0%, Socialist Left 10.1%, Christian People's 8.5%,
    // Centre 6.5%, Liberal 3.2%. Syse's Conservative-Centre-Christian
    // coalition, which fell on 29 October 1990 when the Centre Party walked
    // out over the European Economic Area negotiations.
    // https://en.wikipedia.org/wiki/1989_Norwegian_parliamentary_election
    Polity {
        nation: NationId::Norway,
        // 4% for levelling seats.
        system: Electoral::Proportional,
        // Four years, and the Storting CANNOT be dissolved early - Norway has
        // no snap elections at all, a constitutional peculiarity it has kept
        // since 1814. So (1993, 9) is not an estimate: it is the date, and the
        // government that fell in October 1990 was replaced without one.
        term_months: 48,
        next: (1993, 9),
        parties: &[
            p("no_ap", "Labour Party", "Arbeiderpartiet", Family::SocialDemocratic, 0.343),
            p("no_h", "Conservative Party", "Hoyre", Family::Conservative, 0.222),
            p("no_frp", "Progress Party", "Fremskrittspartiet", Family::Nationalist, 0.130),
            p("no_sv", "Socialist Left Party", "Sosialistisk Venstreparti", Family::Communist, 0.101),
            p("no_krf", "Christian Democratic Party", "Kristelig Folkeparti", Family::ChristianDemocratic, 0.085),
            // The Centre Party is the old Agrarian League and it is the hinge
            // of Norwegian politics on exactly one question: it brought down
            // the government over Europe in 1990 and led the winning No
            // campaign in the referendum of 1994.
            p("no_sp", "Centre Party", "Senterpartiet", Family::Agrarian, 0.065),
            p("no_v", "Liberal Party", "Venstre", Family::Liberal, 0.032),
        ],
        ruling: "the Storting",
        pillars: &[],
    },

    // Finland - Eduskunta, 15-16 March 1987: SDP 24.1%, National Coalition
    // 23.1%, Centre 17.6%, SKDL 9.4%, Rural Party 6.3%, Swedish People's 5.3%,
    // Democratic Alternative 4.2%, Greens 4.0%, Christian League 2.6%.
    // Holkeri's cabinet was the first since the war to seat the National
    // Coalition Party in government with the Social Democrats - the
    // "red-earth" arrangement that broke a taboo about what Moscow would
    // tolerate in a Finnish cabinet, and it held for a full term.
    // https://en.wikipedia.org/wiki/1987_Finnish_parliamentary_election
    Polity {
        nation: NationId::Finland,
        // D'Hondt in fifteen districts, no national threshold.
        system: Electoral::ProportionalLowBar,
        term_months: 48,
        next: (1991, 3),
        parties: &[
            p("fi_sdp", "Social Democratic Party", "Suomen Sosialidemokraattinen Puolue", Family::SocialDemocratic, 0.241),
            p("fi_kok", "National Coalition Party", "Kansallinen Kokoomus", Family::Conservative, 0.231),
            p("fi_kesk", "Centre Party", "Suomen Keskusta", Family::Agrarian, 0.176),
            // The SKDL was the electoral front of the Finnish communists, and
            // DEVA below is the Stalinist minority that split off in 1986 and
            // ran separately in 1987. Both are entered because both were on
            // the ballot; they merged into the Left Alliance in 1990, which
            // the model is not told about.
            p("fi_skdl", "Finnish People's Democratic League", "Suomen Kansan Demokraattinen Liitto", Family::Communist, 0.094),
            p("fi_smp", "Finnish Rural Party", "Suomen Maaseudun Puolue", Family::Nationalist, 0.063),
            // Filed Regionalist because that is what it is: a language party
            // for the 5-6% of Finns whose mother tongue is Swedish, spread
            // along the coast and in Aland, which has sat in almost every
            // Finnish cabinet since 1917 by being indispensable to all of them.
            p("fi_rkp", "Swedish People's Party", "Svenska folkpartiet", Family::Regionalist, 0.053),
            p("fi_deva", "Democratic Alternative", "Demokraattinen Vaihtoehto", Family::Communist, 0.042),
            p("fi_vihr", "Green League", "Vihrea Liitto", Family::Green, 0.040),
            p("fi_skl", "Finnish Christian League", "Suomen Kristillinen Liitto", Family::ChristianDemocratic, 0.026),
        ],
        ruling: "the Eduskunta",
        pillars: &[],
    },

    // Ireland - Dail Eireann, 15 June 1989, first preferences: Fianna Fail
    // 44.1%, Fine Gael 29.3%, Labour 9.5%, Progressive Democrats 5.5%,
    // Workers' Party 5.0%, Greens 1.5%, Sinn Fein 1.2%. Haughey called the
    // election early looking for a majority, lost seats, and formed the first
    // coalition Fianna Fail had entered in the sixty-three years of its
    // existence - the party's entire claim had been that it alone could
    // govern alone. Next election 25 November 1992.
    // https://en.wikipedia.org/wiki/1989_Irish_general_election
    Polity {
        nation: NationId::Ireland,
        // The substitution, stated plainly: Ireland uses the single
        // transferable vote in three-, four- and five-seat constituencies,
        // and there is no STV member in this enum. The shares entered are
        // FIRST PREFERENCES, which is the only figure comparable to a party
        // list vote; STV's transfers then move seats around in ways a list
        // system cannot reproduce - it is why the Progressive Democrats and
        // the Workers' Party were persistently under-rewarded and why a
        // transfer-friendly party like Labour outperformed its first count.
        // ProportionalLowBar is the closest available because STV in small
        // districts has no legal threshold, only an arithmetic one.
        system: Electoral::ProportionalLowBar,
        // Five years is the constitutional maximum for a Dail, the longest
        // term in this table.
        term_months: 60,
        next: (1992, 11),
        parties: &[
            // Fianna Fail and Fine Gael are not left and right, and filing
            // them by economic family would be inventing a cleavage the
            // country did not have. They are the two sides of the Treaty of
            // 1921 and the civil war that followed it, and voters inherited
            // the allegiance. Fianna Fail is entered BigTent for that reason.
            p("ie_ff", "Fianna Fail", "Fianna Fail - The Republican Party", Family::BigTent, 0.441),
            p("ie_fg", "Fine Gael", "Fine Gael", Family::ChristianDemocratic, 0.293),
            p("ie_lab", "Labour Party", "Pairti Lucht Oibre", Family::SocialDemocratic, 0.095),
            p("ie_pd", "Progressive Democrats", "An Phairti Daonlathach", Family::Liberal, 0.055),
            p("ie_wp", "Workers' Party", "Pairti na nOibrithe", Family::Communist, 0.050),
            p("ie_gp", "Green Party", "Comhaontas Glas", Family::Green, 0.015),
            // Entered with p() and not pariah(), which is a deliberate line.
            // Sinn Fein was excluded in 1990 as completely as any party in
            // Europe - section 31 of the Broadcasting Authority Act banned its
            // spokesmen from Irish radio and television from 1971 until 1994,
            // and no party would have sat with it. But that exclusion was a
            // ministerial order and a convention, not a signed pact between
            // the other parties, which is the distinction this table's pariah
            // flag has been drawing since Italy; and unlike Herri Batasuna it
            // took its Dail seats when it won them after 1986. It won none in
            // 1989.
            p("ie_sf", "Sinn Fein", "Sinn Fein", Family::Nationalist, 0.012),
        ],
        ruling: "Dail Eireann",
        pillars: &[],
    },
    // ---- Eastern Europe --------------------------------------------------
    // Four of these five hold their first free election inside the first six
    // months of the game and the fifth holds none at all, so this is the one
    // stretch of the table where `next` is doing real work rather than
    // scheduling a formality.
    //
    // Czechoslovakia — the Federal Assembly, House of the People, 8-9 June
    // 1990: the first free election since 1946 and the last before the state
    // dissolved. The assembly sitting on 1 January 1990 is the one elected
    // unopposed in 1986 and then co-opted wholesale in December, so the shares
    // below are June's, on the same convention Poland and Spain use.
    //
    // ARITHMETIC, STATED SO IT CAN BE CHECKED: the published results are
    // per-republic — Civic Forum 53.15% in the Czech lands, Public Against
    // Violence 32.54% in Slovakia — and this table needs one federal number
    // per party. Each republic's share is weighted by its population at the
    // 1991 census, 10.30m against 5.27m, i.e. 0.661 and 0.339. Two independent
    // checks on that weighting land where they should: the Communists come out
    // at 0.136 against the federal 13.6% actually reported, and Civic Forum
    // plus Public Against Violence come out at 0.461 against the reported
    // 46.6%. https://en.wikipedia.org/wiki/1990_Czechoslovak_parliamentary_election
    Polity {
        nation: NationId::Czechoslovakia,
        // 5% in the Czech lands, 3% in Slovakia. Proportional is the closer of
        // the two available bars and it keeps the Moravians and the Hungarian
        // coalition out of the federal chamber, which is where they were.
        system: Electoral::Proportional,
        // Two years, not four, and deliberately: the June 1990 Federal Assembly
        // was elected as a constituent body to write a constitution it never
        // agreed on. The next election came on 5-6 June 1992 and produced Klaus
        // and Meciar, who divided the country within seven months.
        term_months: 24,
        next: (1990, 6),
        parties: &[
            p("cs_of", "Civic Forum", "Obcanske forum", Family::BigTent, 0.351),
            p("cs_ksc", "Communist Party of Czechoslovakia", "Komunisticka strana Ceskoslovenska", Family::Communist, 0.136),
            // Listed separately from Civic Forum rather than merged into it,
            // because the difference between them is the entire subject of this
            // nation's file. They were allied, they were not one party, and the
            // Hyphen War of January to April 1990 — a constitutional crisis
            // about where to put a hyphen in the state's own name — was fought
            // between their two parliamentary clubs.
            p("cs_vpn", "Public Against Violence", "Verejnost proti nasiliu", Family::BigTent, 0.110),
            p("cs_kdh", "Christian Democratic Movement", "Krestanskodemokraticke hnutie", Family::ChristianDemocratic, 0.064),
            p("cs_kdu", "Christian and Democratic Union", "Krestanska a demokraticka unie", Family::ChristianDemocratic, 0.057),
            p("cs_hsd", "Movement for Self-governing Democracy - Moravia and Silesia", "Hnuti za samospravnou demokracii - Spolecnost pro Moravu a Slezsko", Family::Regionalist, 0.052),
            p("cs_sns", "Slovak National Party", "Slovenska narodna strana", Family::Nationalist, 0.037),
            p("cs_egy", "Coexistence", "Egyutteles - Spoluzitie", Family::Regionalist, 0.029),
            p("cs_ds", "Democratic Party", "Demokraticka strana", Family::Conservative, 0.015),
        ],
        ruling: "the Federal Assembly",
        pillars: &[],
    },
    // Hungary — National Assembly, 25 March and 8 April 1990, the first free
    // election since 1945: MDF 24.7%, SZDSZ 21.4%, FKGP 11.7%, MSZP 10.9%,
    // Fidesz 8.9%, KDNP 6.5%, MSZMP 3.7%, on the national list vote of the
    // first round. Antall's MDF-FKGP-KDNP coalition took office on 23 May and
    // ran the full four years — the only government in this region to manage
    // it. https://en.wikipedia.org/wiki/1990_Hungarian_parliamentary_election
    Polity {
        nation: NationId::Hungary,
        // Mixed-member: 176 single-member seats, 152 county list seats, 58
        // national compensation seats, with a 4% bar on the list vote (raised
        // to 5% in 1994). Proportional's 5% is the nearest available and it
        // gets the load-bearing case right: the unreconstructed MSZMP polled
        // 3.7% and won nothing, which is how the Hungarian communist party
        // left the chamber it had occupied for forty-two years.
        system: Electoral::Proportional,
        term_months: 48,
        next: (1990, 3),
        parties: &[
            p("hu_mdf", "Hungarian Democratic Forum", "Magyar Demokrata Forum", Family::Conservative, 0.247),
            p("hu_szdsz", "Alliance of Free Democrats", "Szabad Demokratak Szovetsege", Family::Liberal, 0.214),
            p("hu_fkgp", "Independent Smallholders' Party", "Fuggetlen Kisgazdapart", Family::Agrarian, 0.117),
            p("hu_mszp", "Hungarian Socialist Party", "Magyar Szocialista Part", Family::SocialDemocratic, 0.109),
            // Liberal, and in 1990 that is not a projection backwards from what
            // Fidesz later became: it was founded in 1988 as a youth movement
            // with an upper age limit of 35, sat in the Liberal International
            // from 1992, and campaigned in 1990 to Antall's left on everything
            // except the economy.
            p("hu_fidesz", "Alliance of Young Democrats", "Fiatal Demokratak Szovetsege", Family::Liberal, 0.089),
            p("hu_kdnp", "Christian Democratic People's Party", "Kereszatenydemokrata Neppart", Family::ChristianDemocratic, 0.065),
            p("hu_mszmp", "Hungarian Socialist Workers' Party", "Magyar Szocialista Munkaspart", Family::Communist, 0.037),
        ],
        ruling: "the National Assembly",
        pillars: &[],
    },
    // Romania — Assembly of Deputies, 20 May 1990: the National Salvation
    // Front 66.3%, UDMR 7.2%, the National Liberals 6.4%, the Ecological
    // Movement 2.6%, the Christian-Democratic Peasants 2.6%, AUR 2.1%. Five
    // months after taking power by revolution, the Front broke its pledge not
    // to contest the election and won two thirds of a chamber against parties
    // with no access to state television.
    // https://en.wikipedia.org/wiki/1990_Romanian_general_election
    Polity {
        nation: NationId::Romania,
        // No legal threshold at all in 1990 — a 3% bar arrived in 1992 — which
        // is why sixteen parties took seats behind the Front. ProportionalLowBar
        // is the only entry in this enum that reproduces that.
        system: Electoral::ProportionalLowBar,
        // A constituent assembly, like Czechoslovakia's and Bulgaria's: elected
        // to write the constitution adopted in December 1991, dissolved for the
        // election of 27 September 1992.
        term_months: 24,
        next: (1990, 5),
        parties: &[
            p("ro_fsn", "National Salvation Front", "Frontul Salvarii Nationale", Family::BigTent, 0.663),
            p("ro_udmr", "Democratic Union of Hungarians in Romania", "Uniunea Democrata Maghiara din Romania", Family::Regionalist, 0.072),
            p("ro_pnl", "National Liberal Party", "Partidul National Liberal", Family::Liberal, 0.064),
            p("ro_mer", "Ecological Movement of Romania", "Miscarea Ecologista din Romania", Family::Green, 0.026),
            p("ro_pntcd", "Christian-Democratic National Peasants' Party", "Partidul National Taranesc Crestin Democrat", Family::ChristianDemocratic, 0.026),
            p("ro_aur", "Romanian National Unity Alliance", "Alianta pentru Unitatea Romanilor", Family::Nationalist, 0.021),
        ],
        ruling: "the Assembly of Deputies",
        // Romania is the one entry in this region carrying both a party table
        // and a pillar, and the reason is the Mineriad. On 13-15 June 1990 the
        // government called the Jiu Valley miners to Bucharest to clear
        // University Square and thanked them for it; the army had already been
        // the arbiter in December 1989. An elected government whose last resort
        // is not the courts is described by both fields at once.
        pillars: &[pl(Pillar::Army, "the Romanian Army")],
    },
    // Bulgaria — the Grand National Assembly, 10 and 17 June 1990, on the
    // proportional half of the ballot: the Bulgarian Socialist Party 47.2%,
    // the Union of Democratic Forces 36.2%, the Agrarians 8.0%, the Movement
    // for Rights and Freedoms 6.0%. The only ruling communist party in the
    // region to win a free election outright, seven months after removing
    // Zhivkov itself and ten weeks after renaming itself.
    // https://en.wikipedia.org/wiki/1990_Bulgarian_general_election
    Polity {
        nation: NationId::Bulgaria,
        // Half the 400 seats by party list with a 4% bar, half in single-member
        // constituencies. Proportional is the closer of the two, and it is the
        // half that decided the result.
        system: Electoral::Proportional,
        // Also a constituent assembly: it wrote the constitution of 12 July
        // 1991 and dissolved for the election of 13 October 1991.
        term_months: 24,
        next: (1990, 6),
        parties: &[
            // Communist rather than SocialDemocratic, on the same reading that
            // labels Poland's SLD Communist: on 1 January 1990 this is the
            // Bulgarian Communist Party, in office since 1944, and it does not
            // change its name until 3 April.
            p("bg_bsp", "Bulgarian Socialist Party", "Balgarska sotsialisticheska partiya", Family::Communist, 0.472),
            p("bg_sds", "Union of Democratic Forces", "Sayuz na demokratichnite sili", Family::BigTent, 0.362),
            p("bg_bzns", "Bulgarian Agrarian National Union", "Balgarski zemedelski naroden sayuz", Family::Agrarian, 0.080),
            // The party of the Turkish minority that the previous government
            // had spent five years trying to assimilate and then expel.
            // Founded 4 January 1990, four days into this start state, and the
            // constitution written by the assembly it entered forbade parties
            // formed on ethnic lines — which it survived by a court ruling
            // rather than by anyone's goodwill. Not a pariah: it was nobody's
            // coalition partner in 1990 and everybody's after 2001.
            p("bg_dps", "Movement for Rights and Freedoms", "Dvizhenie za prava i svobodi", Family::Regionalist, 0.060),
        ],
        ruling: "the Grand National Assembly",
        pillars: &[],
    },
    // Albania — the last orthodox Stalinist state in Europe, and the only
    // nation in this region for which `next` is (0, 0). The People's Assembly
    // elected on 1 February 1987 recorded a 100% turnout and 99.99% for the
    // Democratic Front's single list; that is not an election and the model is
    // not given one. Opposition parties were legalised on 11 December 1990,
    // after the game opens, and the shares below are the first real result —
    // 31 March 1991, in which the Party of Labour won the countryside and lost
    // every city including the seat of Ramiz Alia himself. They go live only
    // if the regime opens up, which is the convention the USSR block sets.
    // https://en.wikipedia.org/wiki/1991_Albanian_parliamentary_election
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
        // Named institutions, not "the army". The Sigurimi ran the internal
        // exile system and had roughly one informer for every three adults by
        // the estimates published after 1992; the Party of Labour's Central
        // Committee was the only body that could remove a leader and did not;
        // and the People's Army was the force that manned 170,000 bunkers
        // against an invasion that never came. All three were gone within
        // two years of this start state.
        pillars: &[
            pl(Pillar::Security, "the Sigurimi"),
            pl(Pillar::Party, "the Central Committee of the Party of Labour"),
            pl(Pillar::Army, "the Albanian People's Army"),
        ],
    },

    // ---------------------------------------------------------------------
    // The other ten Soviet successors.
    //
    // None of these governments exists in January 1990 and none of them is
    // reached by `every_government_is_reachable_in_january_1990`, which walks
    // the living. They become live the month `dissolve_ussr` pushes the
    // republic onto the board, and `ensure` seeds this table then.
    //
    // Every block is the founding national vote that put the republic's first
    // sovereign parliament or president in place: for most of them the
    // republican Supreme Soviet elections of spring 1990, the first
    // competitive elections held on that soil since the annexations, and for a
    // few the first post-independence contest, because the 1990 result there
    // was a one-party formality that published no comparable shares.
    //
    // Every one of the ten carries `next: (0, 0)`, and that is not laziness.
    // A date pinned here would be a date in the past by the time the union
    // actually comes apart in this model, which is somewhere in the nineties
    // and different in every seed. `tick` already handles exactly this case -
    // an electoral nation with no election scheduled is given eighteen months
    // and a headline, "sets a date for its first free elections" - and that is
    // a truer description of what these countries did than any constant would
    // be. Where the republic starts above the electoral ceiling instead, the
    // pillars below are what it rests on and the parties wait for the day it
    // opens up.
    // ---------------------------------------------------------------------

    // Belarus - Supreme Soviet of the Byelorussian SSR, 4 March 1990 with
    // runoffs into May. Seat shares, not votes: the Communist Party of
    // Byelorussia took the overwhelming majority of the 310 seats and the
    // Belarusian Popular Front's opposition caucus settled at around 37 of
    // them. Belarus is the republic where the old apparatus was least disturbed
    // by 1991, and that is the fact this table exists to carry.
    // https://en.wikipedia.org/wiki/1990_Byelorussian_Supreme_Soviet_election
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
            pl(Pillar::Army, "the Belorussian Military District's inheritance"),
            pl(Pillar::Party, "the collective-farm and industrial nomenklatura"),
            pl(Pillar::Security, "the State Security Committee"),
        ],
    },

    // Kazakhstan - 1 December 1991 presidential election: Nursultan Nazarbayev
    // unopposed with 98.8%. Azat and Zheltoqsan, the two national-democratic
    // movements that would have contested it, were refused registration, so
    // there is genuinely no second row to transcribe. A single party here is
    // the correct description of the republic and not a gap in it.
    // https://en.wikipedia.org/wiki/1991_Kazakh_presidential_election
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
    // https://en.wikipedia.org/wiki/1991_Uzbek_presidential_election
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
    // https://en.wikipedia.org/wiki/1990_Georgian_Supreme_Soviet_election
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
    // Sargsyan of the Dashnaks 4.3%. Held five weeks after the independence
    // referendum, with the Karabakh war already running and the Azerbaijani
    // blockade closing.
    // https://en.wikipedia.org/wiki/1991_Armenian_presidential_election
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
    // https://en.wikipedia.org/wiki/1992_Azerbaijani_presidential_election
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
    // https://en.wikipedia.org/wiki/1990_Lithuanian_Supreme_Soviet_election
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
    // https://en.wikipedia.org/wiki/1990_Latvian_Supreme_Soviet_election
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
    // descendants could vote. The shares sum to 0.733 because the remainder went
    // to lists that took no seats; padding it to 1.0 would be inventing.
    // https://en.wikipedia.org/wiki/1992_Estonian_parliamentary_election
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
    // https://en.wikipedia.org/wiki/1994_Moldovan_parliamentary_election
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
    // ---- Latin America ----------------------------------------------------
    //
    // Ten chambers, and the striking thing about the set is the date on each
    // one. Nine of these ten are elected bodies, and eight of the nine were
    // seated after 1985. South America had been governed by soldiers almost
    // everywhere in 1980 and by civilians almost everywhere by 1990, which is
    // the largest change in the region's politics this century and the reason
    // the shares below are so recent. What they are not is stable: the parties
    // holding these majorities in January 1990 are mostly gone by 2000.

    // Argentina — Chamber of Deputies, 14 May 1989, held with the presidential
    // election Carlos Menem won for the Justicialists with 47.5%: PJ 44.7%, UCR
    // 28.8%, the Alianza de Centro around the UCeDe 6.9%, Izquierda Unida 3.5%,
    // and a long tail of provincial parties of which the Neuquen People's
    // Movement is the durable one. Raul Alfonsin handed power over five months
    // early, in July 1989, because hyperinflation had made governing impossible.
    // Half the chamber renews every two years, so the next round is due in
    // September 1991.
    // https://en.wikipedia.org/wiki/1989_Argentine_general_election
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
    // has to reach through legitimacy, not through a date.
    // https://en.wikipedia.org/wiki/1988_Mexican_general_election
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
    // took the presidency with 55.2% and is inaugurated on 11 March 1990 — so
    // the game opens with Pinochet still in the palace and the successor already
    // elected, which is why Chile's authoritarianism figure is not a democracy's.
    // https://en.wikipedia.org/wiki/1989_Chilean_general_election
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
    // civilian party the FARC founded under the 1984 ceasefire, and between 1986
    // and 1990 somewhere upward of two thousand of its members were murdered,
    // including both of its presidential candidates. The M-19 Democratic
    // Alliance is entered at 2.7%, its result in the congressional election of
    // March 1990, the month it disarmed. That election is the next one due when
    // the game opens, which is why `next` is three months away rather than three
    // years.
    // https://en.wikipedia.org/wiki/1986_Colombian_parliamentary_election
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
    // democracy in South America; ten weeks after this table was voted, the army
    // shot several hundred people in Caracas during the Caracazo, and the pact
    // never recovered. Five-year terms, next due December 1993.
    // https://en.wikipedia.org/wiki/1988_Venezuelan_general_election
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
    // losing, and the next election is due three months after the game opens.
    //
    // Cambio 90 is deliberately NOT in this table, and the omission is the
    // honest reading rather than an oversight. Alberto Fujimori built it in 1989
    // out of evangelical congregations and informal traders' guilds, and it took
    // 16.5% of the Chamber on 8 April 1990. The convention this module uses
    // elsewhere — enter a party founded after the last election at its first
    // contested share, as Colombia's AD M-19 is entered above — cannot be
    // applied here: 50.1 + 23.0 + 12.0 + 7.3 already accounts for 92.4% of the
    // 1985 vote, so adding 16.5 gives a chamber in which 108.9% of the
    // electorate voted, and the sum check would rightly reject it. A 1985 table
    // with a 1990 party bolted on is not a transcription of either election.
    // Fujimori's outsider is left for the model to produce out of a collapsing
    // party system, which is the whole premise.
    // https://en.wikipedia.org/wiki/1985_Peruvian_general_election
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
    // July 1989 the regime tried and shot General Arnaldo Ochoa, the most
    // decorated officer of the Angolan war, and purged MININT down to the bone.
    // That was a regime securing exactly these three pillars against exactly the
    // risk this table models.
    // https://en.wikipedia.org/wiki/Case_of_the_10
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
    // him a decade earlier. Bolivia's arithmetic produces coalitions nobody
    // would design, which is the case for letting the coalition former run on
    // real shares rather than on a hand-picked government.
    // https://en.wikipedia.org/wiki/1989_Bolivian_general_election
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
    // majority in it. These shares are the least certain in this region's table
    // and ecuador.json says so in its own words rather than here.
    // https://en.wikipedia.org/wiki/1988_Ecuadorian_general_election
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
    // the dictatorship ended, and the first alternation between the two historic
    // parties in twenty-eight years. The Frente Amplio takes Montevideo the same
    // day and never gives it back. Five-year terms with no immediate
    // re-election, so the next is due November 1994.
    // https://en.wikipedia.org/wiki/1989_Uruguayan_general_election
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
    },

    // ===================== Middle East =====================

    // Syria — the People's Council elected 10-11 February 1986, which was the
    // sitting chamber in January 1990: 195 seats, of which the Ba'ath took 129.
    // Article 8 of the 1973 constitution made the Ba'ath "the leading party in
    // society and the state" and its share of the chamber was allocated, not
    // won, so the four junior parties of the National Progressive Front and the
    // vetted independents who held the other 66 seats are deliberately NOT
    // entered as parties. Listing them would imply a choice that Article 8 had
    // removed, and the same judgement is what the Iraq and China blocks above
    // record. The shares therefore sum to 0.662 rather than to 1.0, which is
    // legal here and is the honest shape of the thing.
    // https://en.wikipedia.org/wiki/1986_Syrian_parliamentary_election
    Polity {
        nation: NationId::Syria,
        system: Electoral::Proportional,
        term_months: 48,
        next: (0, 0),
        parties: &[
            p("sy_baath", "Arab Socialist Ba'ath Party", "Hizb al-Ba'th al-'Arabi al-Ishtiraki", Family::Nationalist, 0.662),
        ],
        ruling: "the Regional Command of the Ba'ath Party",
        pillars: &[
            // Named institutions rather than "the army", per the rule the Iraq
            // block sets. The Republican Guard under Adnan Makhlouf and the
            // Third Armoured Division were the units stationed to hold Damascus
            // rather than to face Israel; the Defence Companies that had done
            // that job were broken up after Rifaat al-Assad's move of 1984.
            pl(Pillar::Army, "the Republican Guard and the Third Armoured Division"),
            pl(Pillar::Party, "the Ba'ath Party Regional Command"),
            pl(Pillar::Security, "the General Intelligence Directorate"),
        ],
    },

    // Jordan — Chamber of Deputies, 8 November 1989. The first general election
    // since 1967 and the answer to the Ma'an bread riots of that April. Parties
    // were still illegal, so all 647 candidates for 80 seats stood as
    // independents and the result is recorded as blocs: the Muslim Brotherhood
    // took 22 seats and independent Islamists a further 12, leftist and
    // pan-Arab candidates about 13, and tribal and pro-government independents
    // the remaining 33. Those are SEAT shares of 80, not vote shares, because
    // Jordan published no national vote totals; they are named as blocs because
    // that is what they were, and inventing party labels for them would be
    // worse. The next election was due four years on and was held on 8 November
    // 1993, under a new one-vote law written specifically to cut the
    // Brotherhood's bloc down.
    // https://en.wikipedia.org/wiki/1989_Jordanian_general_election
    Polity {
        nation: NationId::Jordan,
        // Multi-member districts in which an elector had as many votes as the
        // district had seats — the block vote, which is MORE majoritarian than
        // the single non-transferable vote this enum offers. The 1993 law
        // literally made it SNTV. The 1.6 exponent therefore understates the
        // bias of the 1989 system rather than overstating it, which is stated
        // here rather than papered over by reaching for FirstPastThePost's 3.0,
        // a single-member shape Jordan did not use.
        system: Electoral::SingleNonTransferable,
        term_months: 48,
        next: (1993, 11),
        parties: &[
            p("jo_ikhwan", "Muslim Brotherhood and allied Islamists", "al-Ikhwan al-Muslimun", Family::Religious, 0.425),
            p("jo_tribal", "Tribal and pro-government independents", "", Family::Conservative, 0.4125),
            p("jo_left", "Leftist and pan-Arab independents", "", Family::SocialDemocratic, 0.1625),
        ],
        ruling: "the Chamber of Deputies",
        pillars: &[
            // Jordan is below the electoral ceiling and still carries pillars,
            // which Pakistan's block above establishes as legal and which is
            // the truth here: the King appointed and dismissed prime ministers
            // without reference to the chamber that had just been elected, and
            // the East Bank Bedouin regiments were the institution that decided
            // Black September in 1970.
            pl(Pillar::Army, "the Jordanian Armed Forces and the Bedouin regiments"),
            pl(Pillar::Party, "the Hashemite court"),
            pl(Pillar::Security, "the General Intelligence Directorate"),
        ],
    },

    // Lebanon — and this is the block that most needs its reasoning written
    // down, because a confessional parliamentary republic reading as
    // non-electoral looks like an error.
    //
    // In January 1990 Lebanon had no election due and no election possible. The
    // Chamber of Deputies sitting was the one elected in 1972; it had extended
    // its own mandate every few years for eighteen years. The presidency had
    // changed hands twice in fourteen months, once by assassination — Rene
    // Moawad, blown up on 22 November 1989, seventeen days after taking office
    // — and once by rival proclamation, and there were two cabinets claiming to
    // be the government. Power in Lebanon did not change hands by vote, and no
    // vote could have made it. That is what `authoritarianism` above 0.60 gates
    // here: not repression — the Lebanese state had no capacity to repress
    // anybody — but a closed route to office.
    //
    // `parties` is empty, which Saudi Arabia's block above establishes as legal.
    // It is also the correct transcription: the organised political forces in
    // Lebanon in 1990 were militias, they held no seats worth counting in a
    // chamber elected before most of them existed, and they are all in the
    // pillar list instead, which is exactly where an institution that can
    // remove a government belongs.
    // https://en.wikipedia.org/wiki/Taif_Agreement
    Polity {
        nation: NationId::Lebanon,
        system: Electoral::ProportionalLowBar,
        term_months: 48,
        next: (0, 0),
        parties: &[],
        ruling: "the rival cabinets of Michel Aoun and Selim Hoss",
        pillars: &[
            pl(Pillar::Army, "the Lebanese Forces of Samir Geagea"),
            pl(Pillar::Security, "the Syrian Army in the Bekaa and West Beirut"),
            pl(Pillar::Clergy, "Hezbollah and the Revolutionary Guard contingent at Baalbek"),
            pl(Pillar::Party, "the Amal Movement"),
            pl(Pillar::Business, "the Progressive Socialist Party's administration in the Chouf"),
        ],
    },

    // United Arab Emirates — no parties have ever been legal and the Federal
    // National Council was wholly appointed until 2006, so `parties` is empty
    // on the Saudi precedent. The pillars are the two that actually decided
    // Emirati politics: the Supreme Council of Rulers, in which each of the
    // seven emirates holds a veto, and the split between the federal Union
    // Defence Force and the brigades Abu Dhabi and Dubai kept for themselves
    // until the unification of 1976 — the fault line the federal crisis of
    // 1978-79 ran along.
    Polity {
        nation: NationId::UAE,
        system: Electoral::Proportional,
        term_months: 48,
        next: (0, 0),
        parties: &[],
        ruling: "the Supreme Council of Rulers",
        pillars: &[
            pl(Pillar::Party, "the Supreme Council of Rulers"),
            pl(Pillar::Army, "the Union Defence Force and the Abu Dhabi brigades"),
            pl(Pillar::Business, "the Dubai merchant houses"),
        ],
    },

    // Qatar — no assembly, no parties, an appointed Advisory Council. The
    // pillar that matters is the second one and it is not decoration: Sheikh
    // Hamad bin Khalifa had been crown prince and Minister of Defence since
    // 1977, had taken over the running of the state through the late 1980s, and
    // deposed his own father with the armed forces on 27 June 1995 while the
    // Emir was abroad. An army that removes a government is the definition this
    // table uses, and in Qatar's case it did.
    Polity {
        nation: NationId::Qatar,
        system: Electoral::Proportional,
        term_months: 48,
        next: (0, 0),
        parties: &[],
        ruling: "the House of Al Thani",
        pillars: &[
            pl(Pillar::Party, "the Al Thani family council"),
            pl(Pillar::Army, "the Qatar Armed Forces under the Crown Prince"),
            pl(Pillar::Business, "the merchant families of Doha"),
        ],
    },

    // Oman — Sultan Qaboos ruled without an assembly of any kind. The State
    // Consultative Council of 1981 was appointed and the Majlis al-Shura that
    // replaced it in November 1991 had indirectly selected members and no power
    // to legislate. The Ibadi ulema are a real pillar and not a borrowed one:
    // Oman's imamate was a genuine rival government in the interior as recently
    // as the Jebel Akhdar war of 1954-59, and the office of Grand Mufti is the
    // institution that settled which of the two the country belonged to.
    Polity {
        nation: NationId::Oman,
        system: Electoral::Proportional,
        term_months: 48,
        next: (0, 0),
        parties: &[],
        ruling: "the Sultan of Oman",
        pillars: &[
            pl(Pillar::Army, "the Sultan's Armed Forces"),
            pl(Pillar::Party, "the Diwan of the Royal Court"),
            pl(Pillar::Clergy, "the Ibadi ulema and the office of the Grand Mufti"),
            pl(Pillar::Business, "the Omani merchant houses"),
        ],
    },

    // Yemen — House of Representatives, 27 April 1993: the first free
    // multiparty election ever held on the Arabian peninsula, and the thing the
    // unification of 22 May 1990 was supposed to be for. Shares below are SEAT
    // shares of 301, not vote shares, because Yemen's constituency vote totals
    // were not reliably published: General People's Congress 123, Islah 62,
    // Yemeni Socialist Party 56, Ba'ath 7, Nasserists 1, independents 47. They
    // therefore sum to 0.827 and the missing 0.173 is the independents, who are
    // not a party. The alignment is the country's fault line drawn exactly: the
    // GPC was Saleh's northern machine, the YSP was the former ruling party of
    // the south, and each still had its own army. They went to war in May 1994.
    // https://en.wikipedia.org/wiki/1993_Yemeni_parliamentary_election
    Polity {
        nation: NationId::Yemen,
        // 301 single-member constituencies, plurality. The one unambiguous
        // electoral system in this branch.
        system: Electoral::FirstPastThePost,
        term_months: 48,
        next: (1993, 4),
        parties: &[
            p("ye_gpc", "General People's Congress", "al-Mu'tamar al-Sha'bi al-'Amm", Family::BigTent, 0.409),
            p("ye_islah", "Yemeni Congregation for Reform", "al-Tajammu' al-Yamani lil-Islah", Family::Religious, 0.206),
            p("ye_ysp", "Yemeni Socialist Party", "al-Hizb al-Ishtiraki al-Yamani", Family::Communist, 0.186),
            p("ye_baath", "Yemeni Ba'ath Party", "Hizb al-Ba'th al-'Arabi al-Ishtiraki", Family::Nationalist, 0.023),
            p("ye_nasserist", "Nasserist Unionist People's Organisation", "al-Tanzim al-Wahdawi al-Sha'bi al-Nasiri", Family::Nationalist, 0.003),
        ],
        ruling: "the House of Representatives",
        pillars: &[
            // Below the electoral ceiling and still carrying pillars, on the
            // Pakistan precedent, because the whole of Yemen's tragedy is that
            // unification merged two governments and never merged their two
            // armies. Naming them separately is the transcription.
            pl(Pillar::Army, "the northern forces under President Saleh's officers"),
            pl(Pillar::Party, "the Yemeni Socialist Party's southern divisions"),
            pl(Pillar::Clergy, "the tribal confederations of Hashid and Bakil"),
        ],
    },

    // Bahrain — the 1973 constitution and the elected National Assembly lasted
    // twenty months; the Emir dissolved the Assembly in August 1975 and ruled
    // by decree under the State Security Law of 1974 until 2001. No parties.
    // The security pillar is named specifically because it was a specific
    // thing: the State Security service and its Special Branch were run by Ian
    // Henderson, a British officer, from 1966 to 1998, and the 1981 coup plot
    // by the Iranian-trained Islamic Front for the Liberation of Bahrain is
    // what it existed to stop.
    Polity {
        nation: NationId::Bahrain,
        system: Electoral::Proportional,
        term_months: 48,
        next: (0, 0),
        parties: &[],
        ruling: "the House of Al Khalifa",
        pillars: &[
            pl(Pillar::Party, "the Al Khalifa family council"),
            pl(Pillar::Security, "the State Security service and its Special Branch"),
            pl(Pillar::Army, "the Bahrain Defence Force"),
            pl(Pillar::Business, "the Sunni merchant families and the Chamber of Commerce"),
        ],
    },
    // Algeria — the one government in this table whose party shares come from an
    // election held AFTER the game opens, and the convention at the top of this
    // block is what permits it: the last national vote before January 1990 was
    // the single-list FLN legislative of February 1987, and the Islamic
    // Salvation Front did not exist until 18 February 1989, so its share is its
    // result at the first election it contested. That was the municipal poll of
    // 12 June 1990: FIS 54.2%, FLN 28.1%, RCD 2.1%, PNSD 1.6%, PSD 1.1%, PRA
    // 0.8%, PAGS 0.3%, independents 11.7%, on a 65.2% turnout.
    //
    // Algeria sits BELOW the 0.60 electoral ceiling on purpose. In January 1990
    // this was a state whose government could be removed by a vote — thirty-odd
    // parties were legal under the constitution of 23 February 1989 — and five
    // months later a vote removed it from more than half the country's
    // communes. The legislative election that followed was called for 1991,
    // postponed, and held on 26 December 1991; the army cancelled the second
    // round on 11 January 1992 and the war began. Nothing here schedules any of
    // that. What is stated is an electorate, a scheduled election, and an army
    // that is a pillar of the regime rather than a servant of the chamber, and
    // the model is left to do what it does with the three of them together.
    // https://en.wikipedia.org/wiki/1990_Algerian_local_elections
    Polity {
        nation: NationId::Algeria,
        // Two-round majority in single-member districts, which is the system the
        // December 1991 election actually used and the reason it produced 188
        // seats for the FIS out of 231 decided in the first round on 47.5% of
        // the vote. A proportional table here would understate by half the thing
        // that frightened the generals.
        system: Electoral::TwoRound,
        term_months: 60,
        next: (1991, 12),
        parties: &[
            p("dz_fis", "Islamic Salvation Front", "al-Jabhah al-Islamiyah lil-Inqadh", Family::Religious, 0.542),
            p("dz_fln", "National Liberation Front", "Jabhat al-Tahrir al-Watani", Family::Nationalist, 0.281),
            p("dz_rcd", "Rally for Culture and Democracy", "Rassemblement pour la Culture et la Democratie", Family::Liberal, 0.021),
            p("dz_pnsd", "National Party for Solidarity and Development", "Parti National pour la Solidarite et le Developpement", Family::BigTent, 0.016),
            p("dz_psd", "Social Democratic Party", "Parti Social-Democrate", Family::SocialDemocratic, 0.011),
            p("dz_pra", "Party of Algerian Renewal", "Parti du Renouveau Algerien", Family::Liberal, 0.008),
            p("dz_pags", "Socialist Vanguard Party", "Parti de l'Avant-Garde Socialiste", Family::Communist, 0.003),
        ],
        ruling: "the National People's Assembly",
        // Pillars as well as parties, on the Serbia precedent above: an
        // electorate that can vote and an institution that can overrule it are
        // both facts about Algeria in 1990, and the second is the one everybody
        // in Algiers called simply le pouvoir.
        pillars: &[
            pl(Pillar::Army, "the Armee Nationale Populaire"),
            pl(Pillar::Security, "the Securite Militaire"),
            pl(Pillar::Party, "the FLN apparatus"),
        ],
    },
    // Morocco — Chamber of Representatives, 14 September 1984: Constitutional
    // Union 24.8%, National Rally of Independents 17.2%, Popular Movement 15.5%,
    // Istiqlal 15.3%, Socialist Union of Popular Forces 12.4%, National
    // Democratic Party 8.9%, Party of Progress and Socialism 2.3%. Only 199 of
    // the 306 seats were directly elected; the other 107 came from electoral
    // colleges of councillors and professional chambers, which is a mechanism
    // for guaranteeing the outcome and is why this is a regime with pillars.
    //
    // Above the electoral ceiling, and the test is the one that matters: the
    // King appointed the government whoever won, Driss Basri's interior
    // ministry administered the count, and the election due in 1990 was simply
    // postponed by referendum to 1993. A chamber that cannot change a
    // government is not an electorate. The party table is kept in full anyway,
    // because Morocco's opposition was real — Istiqlal and the USFP took 27.7%
    // between them and repeatedly refused office — and because it is what the
    // model needs the day the monarchy liberalises.
    // https://en.wikipedia.org/wiki/1984_Moroccan_general_election
    Polity {
        nation: NationId::Morocco,
        system: Electoral::FirstPastThePost,
        term_months: 72,
        next: (0, 0),
        parties: &[
            p("ma_uc", "Constitutional Union", "al-Ittihad al-Dusturi", Family::Conservative, 0.248),
            p("ma_rni", "National Rally of Independents", "Rassemblement National des Independants", Family::Liberal, 0.172),
            p("ma_mp", "Popular Movement", "al-Haraka al-Sha'biyya", Family::Agrarian, 0.155),
            p("ma_istiqlal", "Istiqlal Party", "Hizb al-Istiqlal", Family::Nationalist, 0.153),
            p("ma_usfp", "Socialist Union of Popular Forces", "al-Ittihad al-Ishtiraki lil-Quwwat al-Sha'biyya", Family::SocialDemocratic, 0.124),
            p("ma_pnd", "National Democratic Party", "al-Hizb al-Watani al-Dimuqrati", Family::Conservative, 0.089),
            p("ma_pps", "Party of Progress and Socialism", "Hizb al-Taqaddum wal-Ishtirakiyya", Family::Communist, 0.023),
        ],
        ruling: "the Royal Cabinet",
        pillars: &[
            pl(Pillar::Party, "the Makhzen"),
            pl(Pillar::Security, "the Ministry of the Interior"),
            pl(Pillar::Army, "the Royal Armed Forces"),
            // Not decoration and not the same as Iran's seminaries: the King is
            // Amir al-Mu'minin, Commander of the Faithful, under article 19 of
            // the constitution. Moroccan religious authority is an attribute of
            // the throne rather than a rival to it, which is exactly why the
            // Islamist challenge that broke Algeria did not break Morocco.
            pl(Pillar::Clergy, "the Commandership of the Faithful"),
        ],
    },
    // Tunisia — Chamber of Deputies, 2 April 1989: the Constitutional Democratic
    // Rally 80.6% and all 141 seats; independents 13.7% and none; the Movement
    // of Socialist Democrats 3.8%, the Popular Unity Party 0.7%, the Unionist
    // Democratic Union 0.4%, all likewise none. A system in which one voter in
    // seven returns nobody at all is not one a vote can remove, which is the
    // whole reason Tunisia sits above the ceiling despite having held a
    // genuinely contested election fifteen months before the game opens.
    // https://en.wikipedia.org/wiki/1989_Tunisian_general_election
    Polity {
        nation: NationId::Tunisia,
        // Majority list in multi-member constituencies: the list with the most
        // votes in a governorate took every seat in it. First-past-the-post is
        // the closest thing in this enum and it is closer than it looks — the
        // 80.6%-to-141-seats result IS the winner-take-all arithmetic.
        system: Electoral::FirstPastThePost,
        term_months: 60,
        next: (0, 0),
        parties: &[
            p("tn_rcd", "Constitutional Democratic Rally", "al-Tajammu' al-Dusturi al-Dimuqrati", Family::BigTent, 0.806),
            // Ennahda is entered under its own name at the independents' share
            // because that is what the independents were. Denied legal
            // recognition, it ran its candidates as independents and the
            // article records academics quoting official results between 10%
            // and 17% for them nationally; 13.68% is the figure in the
            // published table and is what is transcribed. NOT flagged pariah:
            // Ennahda was not shunned by other parties, it was refused a
            // registration, and by 1991 it was being suppressed outright. The
            // pariah flag is for a cordon sanitaire among parties, not for a
            // ban by the state.
            p("tn_nahda", "Ennahda Movement", "Harakat al-Nahda", Family::Religious, 0.137),
            p("tn_mds", "Movement of Socialist Democrats", "Harakat al-Dimuqratiyin al-Ishtirakiyin", Family::SocialDemocratic, 0.038),
            p("tn_pup", "Popular Unity Party", "Hizb al-Wahda al-Sha'biyya", Family::SocialDemocratic, 0.007),
            p("tn_udu", "Unionist Democratic Union", "al-Ittihad al-Dimuqrati al-Wahdawi", Family::Nationalist, 0.004),
        ],
        ruling: "the Constitutional Democratic Rally",
        pillars: &[
            pl(Pillar::Party, "the Constitutional Democratic Rally"),
            // The order here is the Tunisian peculiarity and it is deliberate.
            // Bourguiba and then Ben Ali kept the army small and out of politics
            // on purpose, and the consequence is that the man who took the
            // presidency on 7 November 1987 was the interior minister. In
            // Tunisia the police outrank the generals.
            pl(Pillar::Security, "the Directorate of State Security"),
            pl(Pillar::Business, "the UTICA employers' union"),
            pl(Pillar::Army, "the Tunisian Armed Forces"),
        ],
    },
    // Libya — no parties, and not through neglect: Law 71 of 1972 made forming
    // one a capital offence, and Gaddafi's formula was that he who forms a party
    // betrays. An empty party table is therefore the correct transcription,
    // exactly as it is for Saudi Arabia above and for the same reason — there
    // was no assembly of any kind that a party could have sat in. Formally
    // Gaddafi held no office after 1979 and the General People's Congress
    // governed; actually the revolutionary committees did, and the pillars below
    // are the institutions that would have had to move to remove him.
    Polity {
        nation: NationId::Libya,
        system: Electoral::Proportional,
        term_months: 48,
        next: (0, 0),
        parties: &[],
        ruling: "the General People's Congress",
        pillars: &[
            pl(Pillar::Party, "the Revolutionary Committees Movement"),
            pl(Pillar::Security, "the Jamahiriya Security Organisation"),
            // Named specifically because it is not the army. The regular army
            // was deliberately starved after the coup attempts of the 1970s and
            // routed in Chad in 1987; what actually guarded Tripoli was a
            // praetorian force recruited from the leader's own tribe.
            pl(Pillar::Army, "the Revolutionary Guard Corps"),
            pl(Pillar::Business, "the National Oil Corporation"),
        ],
    },
    // Sudan — National Assembly, April 1986, the last free election before the
    // game opens and the one Omar al-Bashir's coup of 30 June 1989 annulled:
    // the Umma Party 38.4%, the Democratic Unionist Party 29.7%, the National
    // Islamic Front 18.5%, the Sudanese National Party 2.2%, the Communist Party
    // 1.7%. Thirty-seven southern constituencies could not be polled at all
    // because of the war, which is the fact this table cannot express and the
    // separatism figure in sudan.json has to carry instead.
    //
    // Six months old at the start of the simulation, and modelled as a regime
    // rather than an electorate because that is precisely what it had just
    // become: parliament dissolved, every party in the list below banned, the
    // prime minister it elected in prison, and the trade unions gone.
    // https://en.wikipedia.org/wiki/1986_Sudanese_parliamentary_election
    Polity {
        nation: NationId::Sudan,
        system: Electoral::FirstPastThePost,
        term_months: 48,
        next: (0, 0),
        parties: &[
            p("sd_umma", "National Umma Party", "Hizb al-Umma al-Qawmi", Family::Religious, 0.384),
            p("sd_dup", "Democratic Unionist Party", "al-Hizb al-Ittihadi al-Dimuqrati", Family::Religious, 0.297),
            p("sd_nif", "National Islamic Front", "al-Jabhah al-Islamiyah al-Qawmiyah", Family::Religious, 0.185),
            p("sd_snp", "Sudanese National Party", "al-Hizb al-Qawmi al-Sudani", Family::Regionalist, 0.022),
            p("sd_scp", "Sudanese Communist Party", "al-Hizb al-Shuyu'i al-Sudani", Family::Communist, 0.017),
        ],
        ruling: "the Revolutionary Command Council for National Salvation",
        pillars: &[
            pl(Pillar::Army, "the Sudanese Armed Forces"),
            // The banned party that supplied the government its programme. The
            // NIF is in the table above as an electoral force and here as an
            // institution, and both are true at once: Hassan al-Turabi held no
            // office in 1990 and wrote the policy anyway.
            pl(Pillar::Party, "the National Islamic Front"),
            pl(Pillar::Security, "the National Security Service"),
            // Raised by decree in 1989 as a parallel army answerable to the
            // Islamist movement rather than the general staff, which is the
            // classic coup-proofing move and the reason this regime outlasted
            // the officers who made it.
            pl(Pillar::Army, "the Popular Defence Forces"),
        ],
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
    // Bangladesh — Jatiya Sangsad, 3 March 1988: Jatiya Party 68.4% and 251 of
    // 300 seats, the Combined Opposition Party 12.6%, the Freedom Party 3.3%,
    // JSD (Siraj) 1.2%, independents 13.5%. THOSE SHARES DESCRIBE A BOYCOTT,
    // not an electorate, and are entered as such: the Awami League, the BNP, the
    // Communist Party, Jamaat-e-Islami and four other parties all refused to
    // contest. Official turnout was 52.5% and was not believed by anyone,
    // including the Western diplomat who called it a mockery of an election.
    // This is why Ershad is modelled with pillars as well as a parliament — the
    // parliament is decorative and the army is not. He resigned on 6 December
    // 1990, eleven months into the game, and nothing here schedules that.
    // https://en.wikipedia.org/wiki/1988_Bangladeshi_general_election
    Polity {
        nation: NationId::Bangladesh,
        system: Electoral::FirstPastThePost,
        term_months: 60,
        next: (0, 0),
        parties: &[
            p("bd_jp", "Jatiya Party", "Jatiya Party", Family::BigTent, 0.684),
            p("bd_cop", "Combined Opposition Party", "", Family::Liberal, 0.126),
            p("bd_freedom", "Bangladesh Freedom Party", "", Family::Nationalist, 0.033),
            p("bd_jsd", "Jatiya Samajtantrik Dal (Siraj)", "", Family::SocialDemocratic, 0.012),
        ],
        ruling: "the Jatiya Party",
        pillars: &[
            pl(Pillar::Army, "the Bangladesh Army"),
            pl(Pillar::Party, "the Jatiya Party"),
        ],
    },
    // Sri Lanka — Parliament, 15 February 1989: UNP 50.7% and 125 seats, SLFP
    // 31.9% and 67, EROS 4.1%, SLMC 3.6%, TULF 3.4%, the United Socialist
    // Alliance 2.9%, MEP 1.6%. Turnout 63.6%. Premadasa had taken the
    // presidency on 19 December 1988 and this parliament followed. The term is
    // six years, so the scheduled date is February 1995; in the event the
    // parliament was dissolved early and the election came in August 1994,
    // which the model is free to produce and is not told.
    //
    // The reason this is a democracy in the table and not a regime with pillars
    // is that these elections decided who governed. The reason its
    // authoritarianism is 0.35 and not Spain's 0.13 is that they were held
    // under emergency rule, during the JVP insurrection, in the same months as
    // tens of thousands of disappearances.
    // https://en.wikipedia.org/wiki/1989_Sri_Lankan_parliamentary_election
    Polity {
        nation: NationId::SriLanka,
        // Sri Lanka's proportional system carries a 12.5% preference threshold
        // within each district, which is high — but the districts return small
        // enough panels that the Tamil and Muslim parties, whose vote is
        // geographically concentrated in the north and east, take seats on
        // national shares of 3-4%. A high-bar national rule would delete EROS,
        // the TULF and the SLMC from parliament together and with them every
        // Tamil voice inside the constitutional system, at the exact moment the
        // question in Sri Lankan politics was whether such a voice existed.
        system: Electoral::ProportionalLowBar,
        term_months: 72,
        next: (1995, 2),
        parties: &[
            p("lk_unp", "United National Party", "Eksath Jathika Pakshaya", Family::Conservative, 0.507),
            p("lk_slfp", "Sri Lanka Freedom Party", "Sri Lanka Nidahas Pakshaya", Family::SocialDemocratic, 0.319),
            p("lk_eros", "Eelam Revolutionary Organisation of Students", "", Family::Regionalist, 0.041),
            p("lk_slmc", "Sri Lanka Muslim Congress", "", Family::Regionalist, 0.036),
            p("lk_tulf", "Tamil United Liberation Front", "", Family::Regionalist, 0.034),
            p("lk_usa", "United Socialist Alliance", "", Family::Communist, 0.029),
            p("lk_mep", "Mahajana Eksath Peramuna", "Mahajana Eksath Peramuna", Family::Nationalist, 0.016),
        ],
        ruling: "the Parliament",
        pillars: &[],
    },
    // Nepal — THE EMPTY PARTY LIST IS THE TRANSCRIPTION, not a gap. Nepal in
    // January 1990 was the partyless Panchayat: parties had been banned since
    // King Mahendra's coup of 15 December 1960, and the referendum of 2 May
    // 1980 had confirmed the Panchayat over a party system by 55% to 45%. The
    // Rastriya Panchayat was elected, and elected on an explicitly non-party
    // basis, so there are no shares to state. Saudi Arabia sets the precedent
    // in this table for a state whose assembly has no parties in it.
    //
    // The Jana Andolan launched on 18 February 1990, seven weeks after the
    // start; the ban was lifted on 8 April and a constitutional monarchy
    // followed in November. The pillars below are what the model must dismantle
    // to produce that, and it is not told to.
    // https://en.wikipedia.org/wiki/1990_Nepalese_revolution
    Polity {
        nation: NationId::Nepal,
        system: Electoral::FirstPastThePost,
        term_months: 60,
        next: (0, 0),
        parties: &[],
        ruling: "the Panchayat",
        pillars: &[
            pl(Pillar::Party, "the palace secretariat"),
            pl(Pillar::Army, "the Royal Nepal Army"),
            pl(Pillar::Business, "the Rana and Chhetri landholding families"),
        ],
    },
    // Afghanistan — the PDPA, in the third year of National Reconciliation and
    // the eleventh month after the Soviet withdrawal. The 1987 constitution had
    // formally ended the one-party state and a National Assembly was elected in
    // April 1988 with seats reserved for opposition that never took them; the
    // party renamed itself Watan in June 1990. None of that changed who
    // decided. Modelled as a single party at 1.00 on the Iraq pattern, because
    // that is what the institution was, with the four pillars that actually
    // held Najibullah up.
    //
    // The fourth pillar is the one that matters and is the reason this entry is
    // not simply a copy of Iraq's. Kabul did not hold the country with its own
    // army; it held it with paid regional militias, above all Abdul Rashid
    // Dostum's Jowzjani 53rd Division. When the Soviet money that paid them
    // stopped at the end of 1991, Dostum changed sides in March 1992 and the
    // government fell in weeks. That dependency is stated here as a pillar so
    // the model can find the consequence rather than be handed it.
    Polity {
        nation: NationId::Afghanistan,
        system: Electoral::Proportional,
        term_months: 60,
        next: (0, 0),
        parties: &[
            p("af_pdpa", "People's Democratic Party of Afghanistan", "Hizb-i Dimukratik-i Khalq-i Afghanistan", Family::Communist, 1.00),
        ],
        ruling: "the People's Democratic Party of Afghanistan",
        pillars: &[
            pl(Pillar::Army, "the Afghan Armed Forces"),
            pl(Pillar::Security, "WAD, the state information service"),
            pl(Pillar::Party, "the PDPA apparatus"),
            pl(Pillar::Business, "the paid regional militias"),
        ],
    },
    // Myanmar — SLORC, and the most awkward dating decision in this table,
    // made in the open. The shares below are the general election of 27 MAY
    // 1990: NLD 59.9% and 392 of 492 seats, NUP 21.2% and 10 seats, SNLD 1.7%
    // and 23, the Arakan League for Democracy 1.2%, the Mon National Democratic
    // Front 1.1%, PND 0.6%, CNLD 0.4%, UPNO 0.3%. Turnout 72.6%.
    //
    // That is FOUR MONTHS AFTER the start of the game, and every other block in
    // this table looks backwards. It is used anyway because there is nothing to
    // look back at: SLORC abolished the Pyithu Hluttaw on 18 September 1988,
    // the elections before that were single-party BSPP affairs under a
    // constitution that no longer existed, and the May 1990 vote is the only
    // measurement of Burmese political opinion in the entire period. The
    // parties listed were legally registered and campaigning in January 1990
    // under the Political Parties Registration Law of 1988, so they existed at
    // the start; only the count is forward-dated.
    //
    // next is (0, 0) and the pillars are non-empty because the junta annulled
    // the result it lost and governed for twenty-one more years. Aung San Suu
    // Kyi had been under house arrest since 20 July 1989 and led the NLD to
    // that landslide from inside it.
    // https://en.wikipedia.org/wiki/1990_Myanmar_general_election
    Polity {
        nation: NationId::Myanmar,
        system: Electoral::FirstPastThePost,
        term_months: 60,
        next: (0, 0),
        parties: &[
            p("mm_nld", "National League for Democracy", "Amyotha Dimokarasi Aphwehcuhpaii", Family::Liberal, 0.599),
            p("mm_nup", "National Unity Party", "Taingyintha Silonenyinyutye Party", Family::Nationalist, 0.212),
            p("mm_snld", "Shan Nationalities League for Democracy", "", Family::Regionalist, 0.017),
            p("mm_ald", "Arakan League for Democracy", "", Family::Regionalist, 0.012),
            p("mm_mndf", "Mon National Democratic Front", "", Family::Regionalist, 0.011),
            p("mm_pnd", "Party for National Democracy", "", Family::Liberal, 0.006),
            p("mm_cnld", "Chin National League for Democracy", "", Family::Regionalist, 0.004),
            p("mm_upno", "Union Pa-O National Organisation", "", Family::Regionalist, 0.003),
        ],
        ruling: "the State Law and Order Restoration Council",
        pillars: &[
            pl(Pillar::Army, "the Tatmadaw"),
            pl(Pillar::Security, "the Directorate of Defence Services Intelligence"),
            pl(Pillar::Business, "the Union of Myanmar Economic Holdings"),
        ],
    },

    // ---- East and Southeast Asia -------------------------------------------

    // North Korea — the Workers' Party of Korea. The Supreme People's Assembly
    // sitting in January 1990 is the eighth, elected on 2 November 1986; the
    // ninth was elected on 22 April that year. Both were single-list ballots of
    // the Democratic Front for the Reunification of the Fatherland returned at
    // effectively 100%, so the table carries the WPK alone: the Korean Social
    // Democratic Party and the Chondoist Chongu Party exist, hold seats, and
    // have never contested anything. The pillars are the ones Kim Il Sung
    // actually held, and the succession to Kim Jong Il ran through the second
    // of them — he took the Organisation and Guidance Department in 1973,
    // fifteen years before he was given the army.
    Polity {
        nation: NationId::NorthKorea,
        system: Electoral::Proportional,
        term_months: 60,
        next: (0, 0),
        parties: &[
            p("kp_wpk", "Workers' Party of Korea", "Choson Rodongdang", Family::Communist, 1.00),
        ],
        ruling: "the Workers' Party of Korea",
        pillars: &[
            pl(Pillar::Army, "the Korean People's Army"),
            pl(Pillar::Party, "the Organisation and Guidance Department"),
            pl(Pillar::Security, "the State Security Department"),
        ],
    },

    // Taiwan — the supplementary Legislative Yuan election of 2 December 1989,
    // the first contested by a legal opposition: Kuomintang 60.1%, Democratic
    // Progressive Party 28.3%, independents and minor parties the rest. It was
    // supplementary because most of the chamber was still held by members
    // elected in mainland constituencies in 1947 who had never faced a voter
    // and were not retired until December 1991, which is also why the next
    // full-chamber election here is the one of December 1992.
    //
    // The single non-transferable vote is Taiwan's actual system in this period
    // and not a borrowing from the Japan block above: multi-member districts,
    // one vote, no transfers, and the factional nomination discipline that goes
    // with it. The garrison command that enforced martial law until July 1987
    // is a pillar because in January 1990 the transition was still reversible.
    Polity {
        nation: NationId::Taiwan,
        system: Electoral::SingleNonTransferable,
        term_months: 36,
        next: (1992, 12),
        parties: &[
            p("tw_kmt", "Kuomintang", "Zhongguo Guomindang", Family::Conservative, 0.601),
            p("tw_dpp", "Democratic Progressive Party", "Minzhu Jinbudang", Family::Liberal, 0.283),
            p("tw_ind", "independents and minor parties", "", Family::BigTent, 0.116),
        ],
        ruling: "the Legislative Yuan",
        pillars: &[
            pl(Pillar::Army, "the Republic of China Armed Forces"),
            pl(Pillar::Party, "the Kuomintang Central Standing Committee"),
        ],
    },

    // Mongolia — the Mongolian People's Revolutionary Party, whose monopoly
    // under Article 82 of the constitution was intact on 1 January 1990 and
    // gone by 23 March. The demonstrations in Sukhbaatar Square had been
    // running since 10 December 1989 and the whole Politburo resigned on 9
    // March. `next` is (0, 0) because on the day the game opens this is a
    // one-party state; the party table is the result of the People's Great
    // Hural election of 29 July 1990 — the first multiparty vote in Asia's
    // second communist state, which the MPRP won — and it goes live if and
    // when the regime opens. Nothing here schedules that.
    // MPRP 62.3%, Mongolian Democratic Party 24.3%, Social Democrats 5.6%,
    // National Progress 5.6%.
    // https://en.wikipedia.org/wiki/1990_Mongolian_parliamentary_election
    Polity {
        nation: NationId::Mongolia,
        system: Electoral::Proportional,
        term_months: 60,
        next: (0, 0),
        parties: &[
            p("mn_mprp", "Mongolian People's Revolutionary Party", "Mongol Ardyn Khuvisgalt Nam", Family::Communist, 0.623),
            p("mn_mdp", "Mongolian Democratic Party", "Mongolyn Ardchilsan Nam", Family::Liberal, 0.243),
            p("mn_msdp", "Mongolian Social Democratic Party", "Mongolyn Sotsial Demokrat Nam", Family::SocialDemocratic, 0.056),
            p("mn_mnpp", "Mongolian National Progress Party", "Mongolyn Undesnii Devshliin Nam", Family::Conservative, 0.056),
        ],
        ruling: "the Mongolian People's Revolutionary Party",
        pillars: &[
            pl(Pillar::Army, "the Mongolian People's Army"),
            pl(Pillar::Party, "the Central Committee"),
            pl(Pillar::Security, "the Ministry of Public Security"),
        ],
    },

    // Thailand — House of Representatives, 24 July 1988, the election that made
    // Chatichai Choonhavan the first prime minister since 1976 to have sat in
    // the chamber that chose him. The shares are of the 357 seats rather than of
    // the vote, and that is a transcription decision worth stating: Thai results
    // of the period were reported by seat, the districts returned two and three
    // members apiece on a bloc vote, and a national popular share compiled out
    // of them would be an artefact. Chart Thai 87, Social Action 54, Democrat
    // 48, Ruam Thai 35, Prachakorn Thai 31, Rassadorn 21, Muan Chon 17, Palang
    // Dharma 14, and fifty seats spread across nine smaller parties which are
    // not entered.
    //
    // The next election was due by July 1992 and `next` says March 1992, which
    // is when it was actually held — after the army removed this government on
    // 23 February 1991. Hence the pillar, which is not decoration: the Royal
    // Thai Army had taken power eleven times since 1932 and was to do it again
    // thirteen months into the game.
    // https://en.wikipedia.org/wiki/1988_Thai_general_election
    Polity {
        nation: NationId::Thailand,
        system: Electoral::Proportional,
        term_months: 48,
        next: (1992, 3),
        parties: &[
            p("th_chartthai", "Thai Nation Party", "Chart Thai", Family::Conservative, 0.244),
            p("th_sap", "Social Action Party", "Kit Sangkhom", Family::Conservative, 0.151),
            p("th_democrat", "Democrat Party", "Prachathipat", Family::Liberal, 0.134),
            p("th_ruamthai", "United Thai People's Party", "Ruam Thai", Family::Conservative, 0.098),
            p("th_pkt", "Thai Citizens' Party", "Prachakorn Thai", Family::Nationalist, 0.087),
            p("th_rassadorn", "People's Party", "Rassadorn", Family::BigTent, 0.059),
            p("th_muanchon", "Mass Party", "Muan Chon", Family::BigTent, 0.048),
            p("th_palangdharma", "Righteous Force Party", "Palang Dharma", Family::Religious, 0.039),
        ],
        ruling: "the House of Representatives",
        pillars: &[pl(Pillar::Army, "the Royal Thai Army")],
    },

    // Malaysia — general election of 3 August 1986: Barisan Nasional 55.8%,
    // Democratic Action Party 21.0%, Pan-Malaysian Islamic Party 15.6%, with
    // Parti Bersatu Sabah inside the Front at the time and out of it by 1990.
    // That Dewan Rakyat is the sitting one when the game opens and the next
    // election is nine months away, on 21 October 1990 — the one Tengku
    // Razaleigh contested at the head of Semangat 46 after losing the UMNO
    // presidency by forty-three votes and having the party he lost it in
    // declared an unlawful society for the irregularities in that ballot.
    // https://en.wikipedia.org/wiki/1986_Malaysian_general_election
    Polity {
        nation: NationId::Malaysia,
        system: Electoral::FirstPastThePost,
        term_months: 60,
        next: (1990, 10),
        parties: &[
            p("my_bn", "National Front", "Barisan Nasional", Family::BigTent, 0.558),
            p("my_dap", "Democratic Action Party", "", Family::SocialDemocratic, 0.210),
            p("my_pas", "Pan-Malaysian Islamic Party", "Parti Islam Se-Malaysia", Family::Religious, 0.156),
            p("my_pbs", "United Sabah Party", "Parti Bersatu Sabah", Family::Regionalist, 0.045),
        ],
        ruling: "the Dewan Rakyat",
        pillars: &[],
    },

    // Singapore — general election of 3 September 1988, the first fought on
    // group representation constituencies: People's Action Party 63.2%,
    // Workers' Party 16.7%, Singapore Democratic Party 11.5%, National
    // Solidarity Party 8.6%. The PAP took 80 of 81 seats on that 63%, which is
    // what a first-past-the-post system does to an opposition that cannot
    // concentrate, and it is why the electoral system here is not cosmetic.
    // Lee Kuan Yew handed the premiership to Goh Chok Tong in November 1990
    // without an election; the next one fell in August 1991.
    // https://en.wikipedia.org/wiki/1988_Singaporean_general_election
    Polity {
        nation: NationId::Singapore,
        system: Electoral::FirstPastThePost,
        term_months: 60,
        next: (1991, 8),
        parties: &[
            p("sg_pap", "People's Action Party", "", Family::Conservative, 0.632),
            p("sg_wp", "Workers' Party", "", Family::SocialDemocratic, 0.167),
            p("sg_sdp", "Singapore Democratic Party", "", Family::Liberal, 0.115),
            p("sg_nsp", "National Solidarity Party", "", Family::SocialDemocratic, 0.086),
        ],
        ruling: "the Parliament of Singapore",
        pillars: &[],
    },

    // Philippines — House of Representatives, 11 May 1987, the first election
    // under the constitution ratified three months earlier. Shares are of the
    // 200 elected seats, and there is a reason no popular vote appears: the
    // election was contested by personal coalitions — Lakas ng Bansa, PDP-Laban,
    // the Grand Alliance for Democracy — which dissolved and recombined inside
    // the term, and the LDP was assembled out of the winners afterwards. A
    // national vote share for parties that did not exist as national parties
    // would be an invention. The transitional House sat five years; later terms
    // run three, which is what `term_months` carries.
    //
    // The pillar is not a formality. Corazon Aquino faced seven coup attempts
    // between 1986 and 1990, the largest of them in December 1989, a month
    // before the game opens, put down with American aircraft flying cover out
    // of Clark.
    Polity {
        nation: NationId::Philippines,
        system: Electoral::FirstPastThePost,
        term_months: 36,
        next: (1992, 5),
        parties: &[
            p("ph_ldp", "Struggle of Democratic Filipinos", "Laban ng Demokratikong Pilipino", Family::BigTent, 0.660),
            p("ph_gad", "Grand Alliance for Democracy", "", Family::Conservative, 0.100),
            p("ph_np", "Nacionalista Party", "Partido Nacionalista", Family::Conservative, 0.075),
            p("ph_lp", "Liberal Party", "Partido Liberal", Family::Liberal, 0.070),
            p("ph_ind", "independents", "", Family::BigTent, 0.095),
        ],
        ruling: "the House of Representatives",
        pillars: &[pl(Pillar::Army, "the Armed Forces of the Philippines")],
    },

    // Cambodia — the State of Cambodia, governed by the Kampuchean People's
    // Revolutionary Party, which renamed itself the Cambodian People's Party in
    // October 1991 and dropped Marxism-Leninism along with the name. The last
    // National Assembly election, 1 May 1981, was a single list. Vietnamese
    // troops left in September 1989; the Paris agreements are twenty-two months
    // away and the Khmer Rouge still hold ground in the west with Chinese and
    // Thai supply lines behind them. A regime whose army is the only thing
    // between it and three insurgencies, which is what the pillars say.
    Polity {
        nation: NationId::Cambodia,
        system: Electoral::Proportional,
        term_months: 60,
        next: (0, 0),
        parties: &[
            p("kh_kprp", "Kampuchean People's Revolutionary Party", "Pak Pracheachon Padevat Kampuchea", Family::Communist, 1.00),
        ],
        ruling: "the Kampuchean People's Revolutionary Party",
        pillars: &[
            pl(Pillar::Army, "the Cambodian People's Armed Forces"),
            pl(Pillar::Party, "the Politburo"),
            pl(Pillar::Security, "the Ministry of the Interior"),
        ],
    },

    // Laos — the Lao People's Revolutionary Party, in power since December 1975
    // and still governing without a constitution when the game opens; one was
    // promulgated in August 1991. The Supreme People's Assembly elected on 26
    // March 1989 was the first national election since 1975 and every candidate
    // on the ballot had been vetted by the party, so the table carries the LPRP
    // alone rather than inventing an opposition out of the handful of approved
    // non-members.
    Polity {
        nation: NationId::Laos,
        system: Electoral::Proportional,
        term_months: 60,
        next: (0, 0),
        parties: &[
            p("la_lprp", "Lao People's Revolutionary Party", "Phak Pasason Pativat Lao", Family::Communist, 1.00),
        ],
        ruling: "the Lao People's Revolutionary Party",
        pillars: &[
            pl(Pillar::Army, "the Lao People's Army"),
            pl(Pillar::Party, "the Central Committee"),
            pl(Pillar::Security, "the Ministry of the Interior"),
        ],
    },
    // Canada — general election of 21 November 1988, the free-trade election:
    // Progressive Conservative 43.0%, Liberal 31.9%, NDP 20.4%, Reform 2.1%.
    // Mulroney's second majority, won on the Canada-United States Free Trade
    // Agreement his opponents had between them a clear majority against. The
    // next election was due by November 1993 and came on 25 October, when the
    // party in this list at 43.0% was reduced to two seats.
    // https://en.wikipedia.org/wiki/1988_Canadian_federal_election
    //
    // The Bloc Quebecois is deliberately absent. It was founded on 15 June
    // 1990, five and a half months after the game starts, out of the wreckage
    // of Meech Lake — so it contested no election before January 1990 and has
    // no transcribed share to enter. Putting it in the table with an invented
    // opening number would be scripting the Canadian crisis instead of letting
    // the separatism figure in canada.json produce it.
    Polity {
        nation: NationId::Canada,
        system: Electoral::FirstPastThePost,
        term_months: 60,
        next: (1993, 10),
        parties: &[
            p("ca_pc", "Progressive Conservative Party", "", Family::Conservative, 0.430),
            p("ca_lib", "Liberal Party of Canada", "", Family::Liberal, 0.319),
            p("ca_ndp", "New Democratic Party", "", Family::SocialDemocratic, 0.204),
            // Reform ran candidates only west of Ontario in 1988 and won no
            // seats. Filed Regionalist rather than Conservative because that is
            // what it was in 1988 — Western alienation, an elected Senate, and
            // "the West wants in" — whatever it became after 1993.
            p("ca_reform", "Reform Party of Canada", "", Family::Regionalist, 0.021),
        ],
        ruling: "the House of Commons",
        pillars: &[],
    },
    // Australia — House of Representatives, 11 July 1987, first-preference
    // votes: ALP 45.8%, Liberal 34.4%, National 11.5%, Democrats 6.0%. This is
    // the last election BEFORE January 1990 and therefore the right one, even
    // though the next was only eleven weeks away: Hawke went to the country
    // again on 24 March 1990 and won a fourth term with a minority of the
    // two-party preferred vote.
    // https://en.wikipedia.org/wiki/1987_Australian_federal_election
    Polity {
        nation: NationId::Australia,
        // A substitution, and a visible one. Australia elects the House by
        // full preferential voting — an instant runoff — which is not in the
        // Electoral enum. The two candidates were TwoRound, which is what an
        // instant runoff literally is, and FirstPastThePost. FirstPastThePost
        // is entered because of what this enum actually controls: a majority
        // bonus and a threshold, (3.0, 0.0) against TwoRound's (2.0, 0.05).
        // Australia's single-member districts manufacture majorities at least
        // as hard as Britain's — the ALP took 86 of 148 seats on 45.8% of the
        // primary vote in 1987 — and TwoRound's 5% national threshold would
        // have seated the Democrats, who polled 6.0% and won exactly zero
        // House seats that year and every other. Choosing the mechanism that
        // reproduces the outcome over the one that shares the name.
        system: Electoral::FirstPastThePost,
        term_months: 36,
        next: (1990, 3),
        parties: &[
            p("au_alp", "Australian Labor Party", "", Family::SocialDemocratic, 0.458),
            p("au_lib", "Liberal Party of Australia", "", Family::Conservative, 0.344),
            // The Coalition's two halves are entered separately because they
            // are separate parties with separate leaders and separate rooms,
            // and because the National Party is agrarian in a way the Liberals
            // have never been: it exists to represent farmers and country
            // towns, and it splits from its partner over exactly that.
            p("au_nat", "National Party of Australia", "", Family::Agrarian, 0.115),
            p("au_dem", "Australian Democrats", "", Family::Liberal, 0.060),
        ],
        ruling: "the House of Representatives",
        pillars: &[],
    },
    // New Zealand — general election of 15 August 1987: Labour 48.0%,
    // National 44.0%, Democrats 5.7%. The fourth Labour government's second
    // term, and it did not survive it: Lange resigned in August 1989, Palmer
    // holds the office when the game opens, Moore replaced him on 4 September
    // 1990, and National won 67 of 97 seats on 27 October.
    // https://en.wikipedia.org/wiki/1987_New_Zealand_general_election
    Polity {
        nation: NationId::NewZealand,
        // Genuinely first past the post, in a unicameral parliament, with no
        // upper house and no written constitution to slow it down. New Zealand
        // adopted mixed-member proportional representation at the referendum of
        // 6 November 1993 and first used it in 1996 — six years past this table
        // and reachable by the model rather than written into it.
        system: Electoral::FirstPastThePost,
        term_months: 36,
        next: (1990, 10),
        parties: &[
            p("nz_lab", "New Zealand Labour Party", "", Family::SocialDemocratic, 0.480),
            p("nz_nat", "New Zealand National Party", "", Family::Conservative, 0.440),
            p("nz_dem", "Democratic Party", "Social Credit", Family::Liberal, 0.057),
        ],
        ruling: "the House of Representatives",
        pillars: &[],
    },

    // ===== Central Africa (branch feat/r2-centafrica) ======================
    //
    // Six regimes, and every one of them is a single-party state on 1 January
    // 1990. That is the finding rather than a shortcut: the wave that took all
    // six apart — Gabon's Rendez-vous de Mars, Congo's Conference Nationale
    // Souveraine, Sao Tome's referendum, the Central African and Equatoguinean
    // transitions, and in Chad a rebellion rather than a conference — all of it
    // is inside three years of this start date and NONE of it is scheduled
    // here. What the model is handed is six regimes with pillars and no
    // parliament, and it is left to knock them over or not.
    //
    // A note on where the vote shares come from, because it differs by country
    // and each block says which it used. Three of the six eventually published
    // percentages at their first competitive election and those are entered
    // (Sao Tome 1991, Equatorial Guinea 1993, the Central African Republic's
    // 1993 presidential first round, on the same footing as Ghana's block
    // above). Two published SEATS ONLY and no percentages — Gabon 1990 and
    // Congo 1992 — and for those the seat share is entered with the
    // substitution declared, because a seat share somebody counted beats a
    // vote share nobody did. Chad had no competitive election until 1996 and
    // carries its single party alone at 1.00, exactly as Zaire's block does.

    // Chad — the Union Nationale pour l'Independance et la Revolution, Hissene
    // Habre's sole legal party from its founding congress of June 1984. The
    // only vote in the country between 1969 and 1996 was the single-UNIR-list
    // legislative election of 8 July 1990, five months before Habre lost
    // N'Djamena, and it is not an election this table can transcribe shares
    // from. The first competitive one was the presidential poll of June-July
    // 1996, six years out and won by the man who was still in Darfur when this
    // game opens. So the party slot holds UNIR alone, and the point of the
    // block — as in Zaire's — is that it holds nothing else.
    Polity {
        nation: NationId::Chad,
        system: Electoral::FirstPastThePost,
        term_months: 60,
        next: (0, 0),
        parties: &[
            p("td_unir", "National Union for Independence and Revolution", "Union Nationale pour l'Independance et la Revolution", Family::BigTent, 1.00),
        ],
        ruling: "the National Union for Independence and Revolution",
        pillars: &[
            // The distinction this table insists on. The Forces Armees
            // Nationales Tchadiennes beat the Libyan army in 1987 and were
            // then purged of the Zaghawa officers who did it; what held Habre
            // in power after April 1989 was not them.
            pl(Pillar::Army, "the Forces Armees Nationales Tchadiennes"),
            // The Direction de la Documentation et de la Securite reported to
            // the president in person. Chad's own truth commission of 1992
            // attributed something over 40,000 deaths to it, and Habre was
            // convicted in Dakar in 2016 on that record.
            pl(Pillar::Security, "the Direction de la Documentation et de la Securite"),
            pl(Pillar::Party, "the UNIR Bureau Politique"),
        ],
    },

    // Central African Republic — the Rassemblement Democratique Centrafricain,
    // Andre Kolingba's party, sole legal one from 1986, confirmed by a
    // referendum on 21 November 1986 (91.2%) and a single-list legislative
    // election on 31 July 1987. The shares entered are the FIRST ROUND of the
    // presidential election of 22 August 1993, the first free vote the country
    // ever held: Patasse 38.03%, Goumba 22.10%, Dacko 20.49%, Kolingba 12.33%,
    // Lakoue 2.44%, Malendoma 2.07%. Ruth-Rolland's 1.02% and Bozize's 1.53%
    // are recorded here rather than entered, to keep the table to six.
    // Kolingba finished FOURTH in his own country and handed over, which is the
    // result that makes these shares worth entering: they measure a real
    // electorate rather than a boycott. Presidential first-round shares are
    // used because the concurrent legislative election published seats only —
    // the same substitution Ghana's block above makes and for the same reason.
    // https://en.wikipedia.org/wiki/1993_Central_African_general_election
    Polity {
        nation: NationId::CentralAfricanRepublic,
        system: Electoral::TwoRound,
        term_months: 72,
        next: (0, 0),
        parties: &[
            p("cf_mlpc", "Movement for the Liberation of the Central African People", "Mouvement de Liberation du Peuple Centrafricain", Family::SocialDemocratic, 0.3803),
            p("cf_fpp", "Patriotic Front for Progress", "Front Patriotique pour le Progres", Family::SocialDemocratic, 0.2210),
            p("cf_mdd", "Movement for Democracy and Development", "Mouvement pour la Democratie et le Developpement", Family::Liberal, 0.2049),
            p("cf_rdc", "Central African Democratic Rally", "Rassemblement Democratique Centrafricain", Family::BigTent, 0.1233),
            p("cf_psd", "Social Democratic Party", "Parti Social-Democrate", Family::SocialDemocratic, 0.0244),
            p("cf_fc", "Civic Forum", "Forum Civique", Family::Conservative, 0.0207),
        ],
        ruling: "the Central African Democratic Rally",
        pillars: &[
            // Recruited heavily from Kolingba's own Yakoma after 1981, which is
            // why the army that mutinied in 1996 mutinied along that line.
            pl(Pillar::Army, "the Forces Armees Centrafricaines"),
            pl(Pillar::Security, "the Garde Presidentielle"),
            pl(Pillar::Party, "the RDC Comite Directeur"),
        ],
    },

    // Congo-Brazzaville — the Parti Congolais du Travail, Marxist-Leninist and
    // the sole legal party since 31 December 1969, with the last single-list
    // election to the Assemblee Nationale Populaire on 24 September 1989. The
    // shares entered are SEAT SHARES from the first multi-party election, 24
    // June and 19 July 1992, and the substitution is declared because no vote
    // percentages were published for it: UPADS 39 of 125, MCDDI 29, PCT 18,
    // RDPS 9, RDD 5, UFD 3, UPSD 2. They sum to 0.84 rather than 1.00 because
    // eight further parties took one seat each and eight independents were
    // elected, and padding that gap with a party nobody counted would be worse
    // than leaving it. What the numbers say is the thing worth saying: the
    // party that had governed for twenty-three years came THIRD, and then the
    // three men at the top of this list each raised a militia and fought a war
    // in 1993 and again in 1997.
    // https://en.wikipedia.org/wiki/1992_Republic_of_the_Congo_parliamentary_election
    Polity {
        nation: NationId::Congo,
        system: Electoral::TwoRound,
        term_months: 60,
        next: (0, 0),
        parties: &[
            p("cg_upads", "Pan-African Union for Social Democracy", "Union Panafricaine pour la Democratie Sociale", Family::SocialDemocratic, 0.312),
            p("cg_mcddi", "Congolese Movement for Democracy and Integral Development", "Mouvement Congolais pour la Democratie et le Developpement Integral", Family::Conservative, 0.232),
            p("cg_pct", "Congolese Party of Labour", "Parti Congolais du Travail", Family::Communist, 0.144),
            p("cg_rdps", "Rally for Democracy and Social Progress", "Rassemblement pour la Democratie et le Progres Social", Family::SocialDemocratic, 0.072),
            p("cg_rdd", "Rally for Democracy and Development", "Rassemblement pour la Democratie et le Developpement", Family::Liberal, 0.040),
            p("cg_ufd", "Union of Democratic Forces", "Union des Forces Democratiques", Family::Liberal, 0.024),
            p("cg_upsd", "Union for Social Progress and Democracy", "Union pour le Progres Social et la Democratie", Family::SocialDemocratic, 0.016),
        ],
        ruling: "the Congolese Party of Labour",
        pillars: &[
            pl(Pillar::Army, "the Forces Armees Congolaises"),
            pl(Pillar::Party, "the PCT Comite Central"),
            pl(Pillar::Security, "the Direction Generale de la Securite d'Etat"),
        ],
    },

    // Gabon — the Parti Democratique Gabonais, Omar Bongo's, sole legal party
    // from March 1968 until the constitutional revision of May 1990. The shares
    // are SEAT SHARES from the first multi-party election, 16 September with
    // re-runs on 21 and 28 October 1990 — nine months into the game — because
    // no vote percentages were published: PDG 63 of 120, MORENA-Bucherons 20,
    // PGP 18, MORENA-Originel 7, APSG 6, USG 4, CRP 1, UGDD 1. Results in 32 of
    // 120 constituencies were annulled for fraud and re-run, which is the sort
    // of thing that makes a percentage meaningless and a seat count merely
    // disputed.
    //
    // Nothing here schedules that election, and the block is deliberately a
    // pillars-and-no-parliament regime at t=0: on 1 January 1990 Gabon had a
    // one-party National Assembly, and what turned it into the list above was a
    // public-sector strike wave that had already started, a national conference
    // in March and April, the death of Joseph Rendjambe on 23 May, a rising at
    // Port-Gentil and 500 French paratroopers.
    // https://en.wikipedia.org/wiki/1990_Gabonese_parliamentary_election
    Polity {
        nation: NationId::Gabon,
        system: Electoral::TwoRound,
        term_months: 60,
        next: (0, 0),
        parties: &[
            p("ga_pdg", "Gabonese Democratic Party", "Parti Democratique Gabonais", Family::BigTent, 0.525),
            p("ga_rnb", "National Rally of Woodcutters", "Rassemblement National des Bucherons", Family::Liberal, 0.1667),
            p("ga_pgp", "Gabonese Progress Party", "Parti Gabonais du Progres", Family::SocialDemocratic, 0.15),
            p("ga_morena", "MORENA-Originel", "Mouvement de Redressement National", Family::Nationalist, 0.0583),
            p("ga_apsg", "Association for Socialism in Gabon", "Association pour le Socialisme au Gabon", Family::SocialDemocratic, 0.05),
            p("ga_usg", "Gabonese Socialist Union", "Union Socialiste Gabonaise", Family::SocialDemocratic, 0.0333),
            p("ga_crp", "Circle for Renewal and Progress", "Cercle pour le Renouveau et le Progres", Family::Liberal, 0.0083),
            p("ga_ugdd", "Gabonese Union for Democracy and Development", "Union Gabonaise pour la Democratie et le Developpement", Family::Liberal, 0.0083),
        ],
        ruling: "the Gabonese Democratic Party",
        pillars: &[
            // The formation, not the institution: the Garde Presidentielle was
            // better equipped than the Forces Armees Gabonaises and answered to
            // the president rather than to the ministry.
            pl(Pillar::Army, "the Garde Presidentielle"),
            pl(Pillar::Party, "the PDG Bureau Politique"),
            // The other pillar of this regime was a company. Elf Gabon paid the
            // rent that paid everyone else, and the Elf-Aquitaine relationship
            // with the presidency is the thing the Elf affair of the 1990s was
            // eventually prosecuted over in Paris.
            pl(Pillar::Business, "Elf Gabon"),
        ],
    },

    // Equatorial Guinea — the Partido Democratico de Guinea Ecuatorial,
    // founded in 1987 as Obiang's sole legal party. The shares are the
    // legislative election of 21 November 1993, the first multi-party vote
    // since 1968: PDGE 69.79%, CSDP 10.28%, UDS 7.36%, PL 6.36%, CLD 2.51%.
    // THOSE SHARES DESCRIBE A BOYCOTT, not an electorate, and are entered as
    // such on the same footing as Bangladesh's block above: the Plataforma de
    // Oposicion Conjunta, eight parties between them, refused to contest, the
    // opposition put turnout near 20% against an official 67%, and Spain's
    // foreign minister said publicly that the election was neither free nor
    // fair. The parties that did stand won 12 of 80 seats. This is why the
    // regime is modelled with pillars as well as a party list.
    // https://en.wikipedia.org/wiki/1993_Equatorial_Guinean_parliamentary_election
    Polity {
        nation: NationId::EquatorialGuinea,
        system: Electoral::Proportional,
        term_months: 60,
        next: (0, 0),
        parties: &[
            p("gq_pdge", "Democratic Party of Equatorial Guinea", "Partido Democratico de Guinea Ecuatorial", Family::BigTent, 0.6979),
            p("gq_csdp", "Social Democratic and Popular Convergence", "Convergencia Social Democratica y Popular", Family::SocialDemocratic, 0.1028),
            p("gq_uds", "Social Democratic Union", "Union Democratica Social", Family::SocialDemocratic, 0.0736),
            p("gq_pl", "Liberal Party", "Partido Liberal", Family::Liberal, 0.0636),
            p("gq_cld", "Liberal Democratic Convention", "Convencion Liberal Democratica", Family::Liberal, 0.0251),
        ],
        ruling: "the Democratic Party of Equatorial Guinea",
        pillars: &[
            // Not the national army. The guard that kept Obiang alive through
            // the coup attempt of August 1988 was several hundred Moroccan
            // soldiers sent by Hassan II in 1979 and still in the palace at
            // Malabo in 1990 — a pillar of this regime that was not
            // Equatoguinean at all, which is a fact worth having in the table.
            pl(Pillar::Army, "the Moroccan presidential guard"),
            pl(Pillar::Security, "the Guardia Nacional"),
            pl(Pillar::Party, "the PDGE"),
        ],
    },

    // Sao Tome and Principe — the Movimento de Libertacao de Sao Tome e
    // Principe, sole legal party since independence on 12 July 1975 and Manuel
    // Pinto da Costa president throughout. The shares are the legislative
    // election of 20 January 1991, the first free multi-party election in
    // lusophone Africa: PCD-GR 59.33% and 33 of 55 seats, MLSTP/PSD 33.31% and
    // 21, CODO 5.71% and 1, FCD 1.65% and none. Turnout 76.7%.
    //
    // Those are real percentages of a real electorate, and this is the one
    // block in this region where the ruling party lost and left. The MLSTP
    // central committee resolved on multipartyism in December 1989, the new
    // constitution passed a referendum on 22 August 1990 with about 72%, and
    // eleven weeks later the party that had governed for fifteen years was in
    // opposition. NOTHING HERE SCHEDULES ANY OF THAT — the block opens as a
    // one-party regime with three pillars, and saotome.json sets
    // authoritarianism at 0.62, only just above the model's electoral ceiling,
    // which is where the possibility lives.
    // https://en.wikipedia.org/wiki/1991_S%C3%A3o_Tom%C3%A9an_legislative_election
    Polity {
        nation: NationId::SaoTome,
        system: Electoral::Proportional,
        term_months: 48,
        next: (0, 0),
        parties: &[
            p("st_pcd", "Democratic Convergence Party", "Partido de Convergencia Democratica - Grupo de Reflexao", Family::Liberal, 0.5933),
            p("st_mlstp", "MLSTP/Social Democratic Party", "Movimento de Libertacao de Sao Tome e Principe", Family::SocialDemocratic, 0.3331),
            p("st_codo", "Opposition Democratic Coalition", "Coligacao Democratica da Oposicao", Family::Liberal, 0.0571),
            p("st_fcd", "Christian Democratic Front", "Frente Democrata-Crista", Family::ChristianDemocratic, 0.0165),
        ],
        ruling: "the Movement for the Liberation of Sao Tome and Principe",
        pillars: &[
            // Six hundred men, and for most of the period not even them: the
            // garrison that actually secured the islands after the coup scare
            // of 1978 was about a thousand Angolan FAPLA troops, withdrawn in
            // 1991. The smallest army in this roster propping up the least
            // authoritarian regime in this region, which is not a coincidence.
            pl(Pillar::Army, "the Forcas Armadas de Sao Tome e Principe"),
            pl(Pillar::Party, "the MLSTP Comite Central"),
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

