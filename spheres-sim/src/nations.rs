//! Nation identity as a runtime value, and the roster it is interned from.
//!
//! `NationId` used to be a closed Rust enum with a hand-written `name()` and
//! `parse()` arm per country and fixed-size roster arrays beside it. At the
//! ~190 nations BIBLE.md commits to that is some 380 match arms nobody can
//! keep in agreement, and — far worse — it forces every *dyadic* fact about
//! the world to be written as a `match (a, b)`, which is ~36,000 ordered pairs.
//! A dyad table cannot be a match statement at that size.
//!
//! So identity is now an interned index: `NationId` is a `u16` handle into the
//! roster below, `Copy`, cheap to compare, and usable directly as the index
//! into the relations matrix. Adding a country is adding a row, not editing
//! code. The roster is a static table today because that is what is
//! transcribed; when it becomes a JSON file it replaces `ROSTER` and nothing
//! above this module changes, because nothing above this module knows how many
//! nations there are.
//!
//! Three things this module has to get right:
//!
//! * **Order is the relations matrix.** A nation's index is its row in the
//!   dense triangle in `world.rs`, so the registry order is load-bearing and
//!   the order here is deliberately the order the old enum declared.
//! * **Saves carry codes, never indices.** `NationId` serializes as its stable
//!   code string — exactly the discipline `tech::known_serde` follows, and for
//!   exactly the reason: the technology tree was already bitten once by a save
//!   that stored registry indices and silently reinterpreted them on the next
//!   build.
//! * **Dyadic facts are data.** `neighbours` and `claims` are what the derived
//!   war-appetite model in `dyads.rs` reads instead of a per-pair match arm.

use std::sync::OnceLock;

use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// A territorial or irredentist claim one state holds on another.
///
/// `share` is the fraction of the *target* a claim would take if pressed to its
/// full extent — the whole country for Iraq on Kuwait, a third for Belgrade on
/// Bosnia, a rounding error for a reef. That single number is what separates a
/// border dispute from a war of conquest, and it is the reason the appetite
/// model does not need to know either country's name.
#[derive(Clone, Copy, Debug)]
pub struct Claim {
    pub on: &'static str,
    pub share: f64,
}
const fn claim(on: &'static str, share: f64) -> Claim {
    Claim { on, share }
}

/// One transcribed row of the roster.
#[derive(Clone, Copy, Debug)]
pub struct NationRow {
    /// Stable identifier. This is what a save file writes and what a data file
    /// would key on; it must never be renamed once written.
    pub code: &'static str,
    pub name: &'static str,
    /// Everything a human might type at the CLI for this nation.
    pub aliases: &'static [&'static str],
    pub region: &'static str,
    /// Land borders (and the two straits narrow enough to march across) with
    /// other nations *in this roster*. Neighbours outside the roster are simply
    /// absent, which is correct: a border with an unsimulated state is not a
    /// dyad this model can have an opinion about.
    pub neighbours: &'static [&'static str],
    pub claims: &'static [Claim],
    /// On the board in January 1990. The rest are successor states that exist
    /// only if a federation comes apart.
    pub start_1990: bool,
    /// Keeps clients: has a bloc to hold together and something to lose if it
    /// goes over to the other side.
    pub patron: bool,
    /// Sanctions an aggressor and may intervene for its victim.
    pub major: bool,
}

const fn row(
    code: &'static str,
    name: &'static str,
    aliases: &'static [&'static str],
    region: &'static str,
    neighbours: &'static [&'static str],
    claims: &'static [Claim],
    start_1990: bool,
    patron: bool,
    major: bool,
) -> NationRow {
    NationRow { code, name, aliases, region, neighbours, claims, start_1990, patron, major }
}

// Region codes. Coarse on purpose: a region is "close enough that force can be
// projected without anybody's permission", not a UN statistical division.
pub const REGIONS: &[&str] = &[
    "NorthAmerica", "LatinAmerica", "WesternEurope", "EasternEurope", "Balkans",
    "Eurasia", "MiddleEast", "NorthAfrica", "WestAfrica", "SouthAsia",
    "EastAsia", "SoutheastAsia",
];

/// The roster. **Order is the relations-matrix order and must not be shuffled.**
///
/// Borders are geographic fact. Claims are historical fact and each carries its
/// source; the number is the share of the target the claim covers, so that a
/// claim on a coral reef and a claim on a whole emirate are not the same object.
#[rustfmt::skip]
pub const ROSTER: &[NationRow] = &[
    row("USA", "United States", &["usa", "us", "america"], "NorthAmerica",
        &[], &[], true, true, true),

    // The union borders China, Poland, Turkey and Iran; the successor Russia
    // borders only the first two of those, which is a real change the model
    // gets for free from the data rather than from a special case.
    row("USSR", "Soviet Union", &["ussr", "soviets"], "Eurasia",
        &["China", "Poland", "Turkey", "Iran"], &[], true, true, false),

    // Russia's claim on Ukraine is the 1989 Soviet census: 22.1% of the
    // Ukrainian SSR's people were ethnic Russians, concentrated in Crimea and
    // the Donbas, and Crimea plus the Black Sea Fleet were in open dispute from
    // 1992. https://en.wikipedia.org/wiki/1989_Soviet_census
    // Nothing here schedules 2014; it states the inherited arithmetic and lets
    // the model do what it does with it.
    row("Russia", "Russia", &["rus", "russian federation"], "Eurasia",
        &["China", "Poland", "Ukraine", "Belarus", "Kazakhstan", "Georgia", "Azerbaijan", "Lithuania", "Latvia", "Estonia"], &[claim("Ukraine", 0.22)], false, true, false),

    row("Ukraine", "Ukraine", &["ukr"], "Eurasia",
        &["Russia", "Poland", "Belarus", "Moldova"], &[], false, false, false),

    // Beijing's border with Moscow was fought over at Damansky/Zhenbao in 1969
    // and not finally settled until the agreements of 1991 and 2004. The claim
    // on India is Aksai Chin, held since 1962, plus Arunachal Pradesh — a large
    // area and almost no people, hence a share near zero. On Vietnam it is the
    // Paracels (taken 1974), the Spratlys (Johnson Reef, March 1988) and the
    // land border strips disputed until 1999.
    row("China", "China", &["prc", "cn"], "EastAsia",
        &["USSR", "Russia", "India", "Pakistan", "Vietnam", "Kazakhstan"],
        &[claim("India", 0.02), claim("Vietnam", 0.01), claim("Russia", 0.004)],
        true, true, false),

    row("Japan", "Japan", &["jpn"], "EastAsia", &[], &[], true, true, true),

    row("Germany", "Germany", &["ger", "frg", "brd"], "WesternEurope",
        &["Poland", "France"], &[], true, true, true),

    row("UK", "United Kingdom", &["uk", "britain", "gb"], "WesternEurope",
        &["France"], &[], true, true, true),

    row("France", "France", &["fra"], "WesternEurope",
        &["Germany", "Italy", "UK", "Spain"], &[], true, true, true),

    row("Italy", "Italy", &["ita"], "WesternEurope",
        &["France", "Yugoslavia", "Slovenia"], &[], true, false, false),

    // Kashmir, from both ends. India claims the whole of the former princely
    // state, which means Azad Kashmir and Gilgit-Baltistan — about 5.5m of
    // Pakistan's 108m in 1990. Pakistan claims Indian-administered Jammu and
    // Kashmir, about 8m of India's 870m.
    row("India", "India", &["ind", "bharat"], "SouthAsia",
        &["Pakistan", "China"],
        &[claim("Pakistan", 0.06), claim("China", 0.005)], true, false, false),

    row("Pakistan", "Pakistan", &["pak"], "SouthAsia",
        &["India", "China", "Iran"], &[claim("India", 0.01)], true, false, false),

    // Baghdad never accepted the Anglo-Ottoman line of 1913, claimed Kuwait as
    // part of the Basra vilayet on Kuwaiti independence in 1961, and annexed it
    // as the nineteenth province in August 1990: a claim on the entire state.
    // On Iran it is Khuzestan — "Arabistan" — and the Shatt al-Arab, the stated
    // casus belli of September 1980.
    row("Iraq", "Iraq", &["irq"], "MiddleEast",
        &["Kuwait", "SaudiArabia", "Iran", "Turkey"],
        &[claim("Kuwait", 0.95), claim("Iran", 0.05)], true, false, false),

    row("Kuwait", "Kuwait", &["kwt"], "MiddleEast",
        &["Iraq", "SaudiArabia"], &[], true, false, false),

    row("SaudiArabia", "Saudi Arabia", &["saudi arabia", "saudi", "ksa"], "MiddleEast",
        &["Iraq", "Kuwait"], &[], true, false, false),

    // Tehran's claim is the Shatt al-Arab thalweg conceded at Algiers in 1975
    // and torn up in 1980, and behind it the shrine cities of Najaf and Karbala.
    row("Iran", "Iran", &["irn", "persia"], "MiddleEast",
        &["Iraq", "Turkey", "Pakistan", "USSR", "Armenia", "Azerbaijan"], &[claim("Iraq", 0.03)], true, false, false),

    row("SouthKorea", "South Korea", &["south korea", "korea", "rok"], "EastAsia",
        &[], &[], true, false, false),

    row("Poland", "Poland", &["pol"], "EasternEurope",
        &["Germany", "USSR", "Russia", "Ukraine", "Belarus", "Lithuania"], &[], true, false, false),

    row("Brazil", "Brazil", &["bra"], "LatinAmerica", &[], &[], true, false, false),

    row("Indonesia", "Indonesia", &["idn"], "SoutheastAsia", &[], &[], true, false, false),

    row("Egypt", "Egypt", &["egy", "uar"], "NorthAfrica",
        &["Israel"], &[], true, false, false),

    row("Israel", "Israel", &["isr"], "MiddleEast",
        &["Egypt"], &[], true, false, false),

    // Ankara's claim is the Mosul vilayet, awarded to Iraq by the League of
    // Nations in 1926 over Turkey's objection and pressed again by Ozal during
    // the Gulf crisis: roughly 2m of Iraq's 17m people in the north.
    row("Turkey", "Turkey", &["turkiye", "tur"], "MiddleEast",
        &["Iraq", "Iran", "USSR", "Georgia", "Armenia", "Azerbaijan"], &[claim("Iraq", 0.12)], true, false, false),

    row("Nigeria", "Nigeria", &["nga"], "WestAfrica", &[], &[], true, false, false),

    // The Paracels and Spratlys from the other end, plus the border strips.
    row("Vietnam", "Vietnam", &["viet nam", "vnm"], "SoutheastAsia",
        &["China"], &[claim("China", 0.002)], true, false, false),

    row("Yugoslavia", "Yugoslavia", &["sfry", "yugo"], "Balkans",
        &["Italy"], &[], true, false, false),

    // The 1991 census is the whole of the Yugoslav tragedy in three numbers.
    // Serbs were 31.2% of Bosnia and 12.2% of Croatia — concentrated in the
    // Krajina — and about 2% of Slovenia, which is why Belgrade's claim on
    // Slovenia is not in this table at all. Croats were 17.4% of Bosnia and
    // declared Herceg-Bosna in November 1991.
    // https://en.wikipedia.org/wiki/Demographic_history_of_Bosnia_and_Herzegovina
    row("Serbia", "Serbia", &["serbia and montenegro", "fry", "srb"], "Balkans",
        &["Croatia", "Bosnia"],
        &[claim("Bosnia", 0.31), claim("Croatia", 0.12)], false, false, false),

    row("Croatia", "Croatia", &["hrv"], "Balkans",
        &["Serbia", "Bosnia", "Slovenia"], &[claim("Bosnia", 0.17)], false, false, false),

    row("Slovenia", "Slovenia", &["svn"], "Balkans",
        &["Croatia", "Italy"], &[], false, false, false),

    row("Bosnia", "Bosnia", &["bosnia and herzegovina", "bih"], "Balkans",
        &["Serbia", "Croatia"], &[], false, false, false),

    // Spain's only land border with a nation in this roster is the Pyrenean
    // frontier with France; Portugal, Andorra and Morocco (the Ceuta and
    // Melilla enclaves) are not simulated, so those borders are correctly
    // absent rather than wrong. The claim is Gibraltar, ceded at Utrecht in
    // 1713, never accepted, raised at the UN Committee of 24 every year and
    // under the negotiating process the Brussels Declaration of 27 November
    // 1984 opened. It is the textbook rounding-error claim this table's
    // doc comment describes: Gibraltar's population at the 1991 census was
    // 28,074 against a United Kingdom of 57.4m, or 0.0005 of the target.
    // A grievance that will never be worth a war, stated as the number that
    // makes it never worth a war.
    // https://en.wikipedia.org/wiki/Brussels_Agreement_(1984)
    row("Spain", "Spain", &["esp", "espana"], "WesternEurope",
        &["France"], &[claim("UK", 0.0005)], true, false, false),

    // -----------------------------------------------------------------------
    // The other ten Soviet successors. None of these is on the board in
    // January 1990: every one of them is a union republic whose sovereignty is
    // a consequence of the dissolution `politics::dissolve_ussr` runs, in the
    // same way Serbia and Croatia are consequences of Yugoslavia's. Their
    // borders are the union's internal administrative lines, which is exactly
    // what the borders of 1991 turned out to be — the Alma-Ata Protocol of 21
    // December 1991 recognised the republican boundaries as the international
    // ones, and every war fought here since has been about the places where
    // that principle collided with where the people actually lived.
    //
    // Borders below are with roster members only. Turkmenistan, Tajikistan,
    // Kyrgyzstan, Romania, Afghanistan, Finland, Norway and Mongolia are not
    // simulated, so those frontiers are correctly absent rather than wrong.
    // -----------------------------------------------------------------------

    // Belarus - 77.9% Belarusian in the 1989 census with no territorial
    // minority anywhere and no border anybody disputes: the one republic that
    // came out of the union with neither a claim on a neighbour nor a claim
    // against it. That absence is the datum. It is also the most militarised
    // ground in Europe per head, three combined-arms armies of the Belorussian
    // Military District and 81 SS-25s at Lida and Mozyr, all of it handed back
    // under the Lisbon Protocol of 23 May 1992.
    // https://en.wikipedia.org/wiki/1989_Soviet_census
    row("Belarus", "Belarus", &["blr", "byelorussia", "belorussia"], "Eurasia",
        &["Russia", "Ukraine", "Poland", "Lithuania", "Latvia"], &[], false, false, false),

    // Kazakhstan - and the claim this row deliberately does NOT carry. The 1989
    // census gives 39.7% Kazakhs against 37.8% Russians, the Russians
    // concentrated in the northern oblasts along the frontier, which is the
    // same arithmetic that put Russia's 0.22 on Ukraine above. Boris Yeltsin's
    // press secretary Pavel Voshchanov raised exactly that on 26 August 1991,
    // warning that the RSFSR reserved the right to revise borders with any
    // republic that left the union. Nazarbayev protested, a delegation was sent
    // to Alma-Ata, and the statement was withdrawn inside the week and never
    // restated by any Russian government. A claim asserted for four days and
    // repudiated is not a claim; entering 0.378 here would manufacture the
    // largest war in the model out of a press conference.
    // https://en.wikipedia.org/wiki/Alma-Ata_Protocol
    row("Kazakhstan", "Kazakhstan", &["kaz", "kazakstan"], "Eurasia",
        &["Russia", "China", "Uzbekistan"], &[], false, false, false),

    // Uzbekistan - the most populous republic in the region, 20.5m in 1990, and
    // the one whose borders were drawn to be awkward: the Fergana valley was
    // partitioned between three republics in the delimitation of 1924-36 and
    // left enclaves inside each. All of those disputes are with Kyrgyzstan and
    // Tajikistan, neither of which is simulated, so this row has no claims.
    row("Uzbekistan", "Uzbekistan", &["uzb"], "Eurasia",
        &["Kazakhstan"], &[], false, false, false),

    // Georgia - Abkhazia and South Ossetia were autonomous units inside the
    // Georgian SSR and both fought secession wars between 1991 and 1993. That
    // is separatism, not a claim: they are Georgian territory in this model's
    // terms, and the strain belongs in `separatism`, which `dissolve_ussr` sets
    // to 0.75, the highest of any successor. Tbilisi claims nothing across a
    // border and nobody claims it.
    row("Georgia", "Georgia", &["geo", "sakartvelo"], "Eurasia",
        &["Russia", "Turkey", "Armenia", "Azerbaijan"], &[], false, false, false),

    // Armenia - the one live irredentist claim in the former union, and the
    // only one that became a war between two of its successors. The Nagorno-
    // Karabakh Autonomous Oblast was inside the Azerbaijan SSR and 76.9%
    // Armenian at the 1989 census; its soviet voted on 20 February 1988 to
    // transfer to Armenia, and the Armenian Supreme Soviet voted on 1 December
    // 1989 to unify with it. The share is the oblast against its host: 189,000
    // people of the Azerbaijan SSR's 7,021,000, or 0.027.
    //
    // Not entered: any claim on Turkey. The Armenian declaration of
    // independence of 23 August 1990 names the genocide and Armenia has never
    // ratified the Treaty of Kars, but Ter-Petrosyan's government explicitly
    // declined to make a territorial claim and sought normalisation instead.
    // The claim belongs to the diaspora and to the Dashnaks, not to the state
    // this row describes.
    // https://en.wikipedia.org/wiki/Nagorno-Karabakh_Autonomous_Oblast
    row("Armenia", "Armenia", &["arm", "hayastan"], "Eurasia",
        &["Georgia", "Azerbaijan", "Turkey", "Iran"],
        &[claim("Azerbaijan", 0.027)], false, false, false),

    // Azerbaijan - and the reason the Karabakh claim is one-directional. The
    // ground both armies fought over is Azerbaijani territory under the border
    // Alma-Ata recognised, so Baku is defending rather than claiming, and a
    // reciprocal claim entered here would double-count the same war. The
    // Azerbaijanis expelled from Armenia in 1988-89 - 84,860 of Armenia's
    // 3,304,776 people at the 1989 census - are a real grievance, but the
    // "Western Azerbaijan" doctrine that turns them into a territorial claim is
    // a thing of the 2020s and was not the policy of any government in this
    // period. Nakhchivan's short frontier with Turkey is the border in this row
    // that surprises people; it is 17 kilometres and it is real.
    row("Azerbaijan", "Azerbaijan", &["aze", "azerbaydzhan"], "Eurasia",
        &["Georgia", "Armenia", "Russia", "Turkey", "Iran"], &[], false, false, false),

    // Lithuania - the border with Russia is Kaliningrad, and Vilnius never
    // claimed it: the Lithuanian position from 1990 was that the oblast was a
    // problem for Moscow and Bonn to solve and that raising it would have cost
    // the recognition it was fighting for. The absence of a claim on a Russian
    // exclave wedged between Lithuania and Poland is a decision, not an
    // oversight. 79.6% Lithuanian in 1989, the most homogeneous Baltic, and the
    // only one that gave citizenship to every resident.
    row("Lithuania", "Lithuania", &["ltu", "lietuva"], "EasternEurope",
        &["Latvia", "Belarus", "Poland", "Russia"], &[], false, false, false),

    // Latvia - the Abrene claim. The peace treaty of 11 August 1920 put the
    // Abrene district on the Latvian side; the Soviet Union transferred it to
    // the RSFSR in 1944 and it is Pytalovo today. Latvia held the transfer void
    // along with the annexation itself, and did not drop the claim until the
    // border treaty of 27 March 2007. About 12,000 people against a Russian
    // Federation of 148m: 0.0001, the same order as Spain on Gibraltar, and
    // stated as the number that keeps it a grievance rather than a war.
    // https://en.wikipedia.org/wiki/Latvian-Soviet_Peace_Treaty
    row("Latvia", "Latvia", &["lva", "latvija"], "EasternEurope",
        &["Lithuania", "Estonia", "Belarus", "Russia"],
        &[claim("Russia", 0.0001)], false, false, false),

    // Estonia - the same claim in the same shape and from the same year. The
    // Treaty of Tartu of 2 February 1920 put Petserimaa and the land east of
    // the Narva river inside Estonia; both went to the RSFSR in 1945. The
    // Riigikogu resolved in 1994 that Tartu remained in force, and Estonia
    // conceded the line only with the treaty of 2005. Roughly 27,000 people of
    // a Russia of 148m: 0.0002.
    // https://en.wikipedia.org/wiki/Treaty_of_Tartu_(Russian-Estonian)
    row("Estonia", "Estonia", &["est", "eesti"], "EasternEurope",
        &["Latvia", "Russia"], &[claim("Russia", 0.0002)], false, false, false),

    // Moldova - Transnistria declared on 2 September 1990 and Gagauzia on 19
    // August 1990, and the war of 1992 was decided by the Soviet 14th Army,
    // which was already on the left bank and did not leave. Both are internal,
    // so both are separatism rather than a claim, and `dissolve_ussr` carries
    // them at 0.45. The union with Romania that the Popular Front wanted is not
    // a claim either, and Romania is not simulated in any case.
    row("Moldova", "Moldova", &["mda", "moldavia"], "EasternEurope",
        &["Ukraine"], &[], false, false, false),
];

// ---------------------------------------------------------------------------
// The handle
// ---------------------------------------------------------------------------

/// A nation, as a handle. The inner index is the roster row and the relations
/// matrix row; it is deliberately private so nothing outside this module can
/// invent one out of an integer read off a disk.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct NationId(u16);

impl NationId {
    /// Index into the roster and into the relations matrix.
    pub fn index(self) -> usize {
        self.0 as usize
    }
    pub fn from_index(i: usize) -> Option<NationId> {
        if i < ROSTER.len() {
            Some(NationId(i as u16))
        } else {
            None
        }
    }
    pub fn def(self) -> &'static NationDef {
        &registry()[self.0 as usize]
    }
    /// The stable identifier. This is what goes to disk.
    pub fn code(self) -> &'static str {
        ROSTER[self.0 as usize].code
    }
    pub fn name(self) -> &'static str {
        ROSTER[self.0 as usize].name
    }
    pub fn region(self) -> &'static str {
        ROSTER[self.0 as usize].region
    }
    /// Resolve a code exactly, as a save file does. Case-sensitive on purpose:
    /// this is an identifier, not user input.
    pub fn from_code(code: &str) -> Option<NationId> {
        ROSTER
            .iter()
            .position(|r| r.code == code)
            .map(|i| NationId(i as u16))
    }
    /// What a human typed. Codes, display names and aliases, case-insensitive.
    pub fn parse(s: &str) -> Option<NationId> {
        let t = s.trim().to_lowercase();
        if t.is_empty() {
            return None;
        }
        ROSTER
            .iter()
            .position(|r| {
                r.code.to_lowercase() == t
                    || r.name.to_lowercase() == t
                    || r.aliases.iter().any(|a| *a == t)
            })
            .map(|i| NationId(i as u16))
    }
}

/// Debug prints the code, so that anything built out of `{:?}` — the
/// `burned_*` flags in `war.rs`, the ids the browser UI keys on — stays a
/// stable string rather than a number that moves when the roster grows.
impl std::fmt::Debug for NationId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.code())
    }
}
impl std::fmt::Display for NationId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name())
    }
}

impl Serialize for NationId {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.code())
    }
}
impl<'de> Deserialize<'de> for NationId {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let s = String::deserialize(d)?;
        NationId::from_code(&s).ok_or_else(|| {
            serde::de::Error::custom(format!("save names a nation this build does not have: {}", s))
        })
    }
}

// ---------------------------------------------------------------------------
// The thin compatibility layer
// ---------------------------------------------------------------------------

/// A const per well-known nation, so call sites stay readable and the ~500
/// existing `NationId::Iraq` spellings keep working. These are resolved at
/// compile time from the roster, so a code that is renamed or removed is a
/// build error rather than a silently wrong index.
///
/// This is a convenience, not the mechanism. Nothing in the sim *needs* a
/// nation to have a const, and nothing may pattern-match on one.
macro_rules! well_known {
    ($($konst:ident => $code:literal),* $(,)?) => {
        /// The same handles as free constants, so a data table can say `USA`
        /// rather than `NationId::USA` thirty times a row.
        pub mod ids {
            use super::{of, NationId};
            $(
                #[allow(non_upper_case_globals)]
                pub const $konst: NationId = of($code);
            )*
        }
        #[allow(non_upper_case_globals)]
        impl NationId {
            $( pub const $konst: NationId = ids::$konst; )*
        }
    };
}

well_known! {
    USA => "USA",
    USSR => "USSR",
    Russia => "Russia",
    Ukraine => "Ukraine",
    China => "China",
    Japan => "Japan",
    Germany => "Germany",
    UK => "UK",
    France => "France",
    Italy => "Italy",
    India => "India",
    Pakistan => "Pakistan",
    Iraq => "Iraq",
    Kuwait => "Kuwait",
    SaudiArabia => "SaudiArabia",
    Iran => "Iran",
    SouthKorea => "SouthKorea",
    Poland => "Poland",
    Brazil => "Brazil",
    Indonesia => "Indonesia",
    Egypt => "Egypt",
    Israel => "Israel",
    Turkey => "Turkey",
    Nigeria => "Nigeria",
    Vietnam => "Vietnam",
    Yugoslavia => "Yugoslavia",
    Serbia => "Serbia",
    Croatia => "Croatia",
    Slovenia => "Slovenia",
    Bosnia => "Bosnia",
    Spain => "Spain",
    Belarus => "Belarus",
    Kazakhstan => "Kazakhstan",
    Uzbekistan => "Uzbekistan",
    Georgia => "Georgia",
    Armenia => "Armenia",
    Azerbaijan => "Azerbaijan",
    Lithuania => "Lithuania",
    Latvia => "Latvia",
    Estonia => "Estonia",
    Moldova => "Moldova",
}

const fn bytes_eq(a: &str, b: &str) -> bool {
    let (a, b) = (a.as_bytes(), b.as_bytes());
    if a.len() != b.len() {
        return false;
    }
    let mut i = 0;
    while i < a.len() {
        if a[i] != b[i] {
            return false;
        }
        i += 1;
    }
    true
}

/// Compile-time roster lookup. Panics at build time on an unknown code.
pub const fn of(code: &str) -> NationId {
    let mut i = 0;
    while i < ROSTER.len() {
        if bytes_eq(ROSTER[i].code, code) {
            return NationId(i as u16);
        }
        i += 1;
    }
    panic!("no nation with that code in the roster")
}

// ---------------------------------------------------------------------------
// The resolved registry
// ---------------------------------------------------------------------------

/// A roster row with its string references resolved to handles, plus the
/// derived sets the dyad model walks. Built once; never mutated.
#[derive(Clone, Debug)]
pub struct NationDef {
    pub id: NationId,
    pub code: &'static str,
    pub name: &'static str,
    pub region: &'static str,
    /// Symmetric closure of the declared borders, sorted. Symmetric because a
    /// border is, and closed here so a one-sided row in the data cannot make
    /// two nations disagree about whether they touch.
    pub neighbours: Vec<NationId>,
    /// Sorted by target.
    pub claims: Vec<(NationId, f64)>,
    /// Everyone this nation could plausibly use force against: neighbours, the
    /// states it claims something from, and the rest of its region. The dyad
    /// model walks this instead of the whole roster, which is what keeps the
    /// appetite pass O(n·k) rather than O(n²) at 190 nations.
    pub contacts: Vec<NationId>,
    pub start_1990: bool,
    pub patron: bool,
    pub major: bool,
}

static REGISTRY: OnceLock<Vec<NationDef>> = OnceLock::new();

pub fn registry() -> &'static [NationDef] {
    REGISTRY.get_or_init(build_registry)
}

fn build_registry() -> Vec<NationDef> {
    let resolve = |code: &str| -> NationId {
        NationId::from_code(code)
            .unwrap_or_else(|| panic!("roster names a nation that is not in it: {}", code))
    };

    // Borders first, unioned in both directions.
    let mut adj: Vec<Vec<NationId>> = vec![vec![]; ROSTER.len()];
    for (i, r) in ROSTER.iter().enumerate() {
        for nb in r.neighbours {
            let b = resolve(nb);
            assert!(b.index() != i, "{} borders itself", r.code);
            if !adj[i].contains(&b) {
                adj[i].push(b);
            }
            if !adj[b.index()].contains(&NationId(i as u16)) {
                adj[b.index()].push(NationId(i as u16));
            }
        }
    }

    ROSTER
        .iter()
        .enumerate()
        .map(|(i, r)| {
            let id = NationId(i as u16);
            assert!(REGIONS.contains(&r.region), "{} is in no known region", r.code);
            let mut neighbours = adj[i].clone();
            neighbours.sort();

            let mut claims: Vec<(NationId, f64)> =
                r.claims.iter().map(|c| (resolve(c.on), c.share)).collect();
            claims.sort_by_key(|(t, _)| *t);

            let mut contacts: Vec<NationId> = neighbours.clone();
            for (t, _) in &claims {
                if !contacts.contains(t) {
                    contacts.push(*t);
                }
            }
            for (j, o) in ROSTER.iter().enumerate() {
                if j != i && o.region == r.region {
                    let o = NationId(j as u16);
                    if !contacts.contains(&o) {
                        contacts.push(o);
                    }
                }
            }
            contacts.sort();

            NationDef {
                id,
                code: r.code,
                name: r.name,
                region: r.region,
                neighbours,
                claims,
                contacts,
                start_1990: r.start_1990,
                patron: r.patron,
                major: r.major,
            }
        })
        .collect()
}

/// How many nations this build knows about, ever — the width of the relations
/// matrix, not the number currently alive.
pub fn nation_count() -> usize {
    ROSTER.len()
}

/// Every id, in registry order. The index into this is the index into the
/// relations matrix, so the two can never drift apart: they are the same thing.
pub fn all_nations() -> &'static [NationId] {
    static ALL: OnceLock<Vec<NationId>> = OnceLock::new();
    ALL.get_or_init(|| (0..ROSTER.len()).map(|i| NationId(i as u16)).collect())
}

/// On the board in January 1990.
pub fn start_nations() -> &'static [NationId] {
    static START: OnceLock<Vec<NationId>> = OnceLock::new();
    START.get_or_init(|| all_nations().iter().copied().filter(|n| n.def().start_1990).collect())
}

/// States that only exist if a federation comes apart.
pub fn successor_nations() -> &'static [NationId] {
    static SUCC: OnceLock<Vec<NationId>> = OnceLock::new();
    SUCC.get_or_init(|| all_nations().iter().copied().filter(|n| !n.def().start_1990).collect())
}

/// The powers that keep clients. Not simply the strongest — the ones with a
/// bloc to hold together and something to lose if it goes over to the other
/// side. Japan belongs here despite its two-decimal army: it passed the United
/// States in 1989 to become the largest aid donor in the world and stayed there
/// until 2000. https://ies.princeton.edu/pdf/E196.pdf
pub fn patrons() -> &'static [NationId] {
    static P: OnceLock<Vec<NationId>> = OnceLock::new();
    P.get_or_init(|| ordered_by(PATRON_ORDER, |d| d.patron))
}

/// Precedence, not membership. Which nations are patrons and majors is data on
/// the roster row; the ORDER they are walked in is behaviour, because the first
/// power to reach a client claims it and the first to answer a call joins the
/// war. Deriving that order from registry position made it an accident of where
/// a nation sits in a file, which silently moved 2025 China by 38%. It is
/// therefore stated, and stated in the order the hand-written arrays used to
/// hold before the roster became data.
const PATRON_ORDER: &[&str] = &["USA", "USSR", "Russia", "China", "UK", "France", "Germany", "Japan"];
const MAJOR_ORDER: &[&str] = &["USA", "UK", "France", "Germany", "Japan"];

/// Everything matching `flag`, in the precedence order given, with anything the
/// order forgot appended in registry order so a new patron cannot vanish.
fn ordered_by(order: &[&str], flag: fn(&NationDef) -> bool) -> Vec<NationId> {
    let mut out: Vec<NationId> = vec![];
    for code in order {
        if let Some(id) = NationId::from_code(code) {
            if flag(id.def()) && !out.contains(&id) {
                out.push(id);
            }
        }
    }
    for id in all_nations() {
        if flag(id.def()) && !out.contains(id) {
            out.push(*id);
        }
    }
    out
}

/// The powers that sanction an aggressor and may intervene for its victim.
/// The AI's expectations in `politics::ai_wars` read this same list — if they
/// diverge, aggressors invade into coalitions they never saw coming and never
/// learn better.
pub fn majors() -> &'static [NationId] {
    static M: OnceLock<Vec<NationId>> = OnceLock::new();
    M.get_or_init(|| ordered_by(MAJOR_ORDER, |d| d.major))
}

/// Do these two share a border?
pub fn adjacent(a: NationId, b: NationId) -> bool {
    a.def().neighbours.binary_search(&b).is_ok()
}

/// What `a` claims of `b`, as a share of `b`. Zero when there is no claim.
pub fn claim_share(a: NationId, b: NationId) -> f64 {
    a.def()
        .claims
        .binary_search_by_key(&b, |(t, _)| *t)
        .map(|i| a.def().claims[i].1)
        .unwrap_or(0.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_roster_is_internally_consistent() {
        let reg = registry();
        assert_eq!(reg.len(), ROSTER.len());
        for (i, d) in reg.iter().enumerate() {
            assert_eq!(d.id.index(), i);
            assert_eq!(NationId::from_code(d.code), Some(d.id), "{} does not resolve", d.code);
            // A code is an identifier: unique, and never a display name.
            assert_eq!(
                reg.iter().filter(|o| o.code == d.code).count(),
                1,
                "duplicate code {}",
                d.code
            );
        }
        // Borders are symmetric, in the raw data and not only after the union.
        for r in ROSTER {
            for nb in r.neighbours {
                let other = ROSTER.iter().find(|o| o.code == *nb).expect("neighbour exists");
                assert!(
                    other.neighbours.contains(&r.code),
                    "{} lists {} as a neighbour but not the other way round",
                    r.code,
                    nb
                );
            }
        }
        // Every claim names a real target and is a share, not a magnitude.
        for r in ROSTER {
            for c in r.claims {
                assert!(NationId::from_code(c.on).is_some(), "{} claims a ghost", r.code);
                assert!((0.0..=1.0).contains(&c.share), "{}'s claim is not a share", r.code);
                assert_ne!(c.on, r.code, "{} claims itself", r.code);
            }
        }
    }

    #[test]
    fn every_name_the_old_parser_took_still_parses_to_the_same_nation() {
        // The enum's hand-written parse arms, moved to data. If one of these
        // ever resolves somewhere else, a saved game or a player's muscle
        // memory has been quietly broken.
        let cases: &[(&str, NationId)] = &[
            ("usa", NationId::USA), ("us", NationId::USA),
            ("united states", NationId::USA), ("america", NationId::USA),
            ("ussr", NationId::USSR), ("soviet union", NationId::USSR),
            ("soviets", NationId::USSR), ("russia", NationId::Russia),
            ("ukraine", NationId::Ukraine), ("ukr", NationId::Ukraine),
            ("china", NationId::China), ("prc", NationId::China),
            ("japan", NationId::Japan), ("germany", NationId::Germany),
            ("uk", NationId::UK), ("britain", NationId::UK),
            ("united kingdom", NationId::UK), ("france", NationId::France),
            ("italy", NationId::Italy), ("india", NationId::India),
            ("pakistan", NationId::Pakistan), ("iraq", NationId::Iraq),
            ("kuwait", NationId::Kuwait), ("saudi arabia", NationId::SaudiArabia),
            ("saudi", NationId::SaudiArabia), ("ksa", NationId::SaudiArabia),
            ("iran", NationId::Iran), ("south korea", NationId::SouthKorea),
            ("korea", NationId::SouthKorea), ("rok", NationId::SouthKorea),
            ("poland", NationId::Poland), ("brazil", NationId::Brazil),
            ("bra", NationId::Brazil), ("indonesia", NationId::Indonesia),
            ("idn", NationId::Indonesia), ("egypt", NationId::Egypt),
            ("egy", NationId::Egypt), ("uar", NationId::Egypt),
            ("israel", NationId::Israel), ("isr", NationId::Israel),
            ("turkey", NationId::Turkey), ("turkiye", NationId::Turkey),
            ("tur", NationId::Turkey), ("nigeria", NationId::Nigeria),
            ("nga", NationId::Nigeria), ("vietnam", NationId::Vietnam),
            ("viet nam", NationId::Vietnam), ("vnm", NationId::Vietnam),
            ("yugoslavia", NationId::Yugoslavia), ("sfry", NationId::Yugoslavia),
            ("yugo", NationId::Yugoslavia), ("serbia", NationId::Serbia),
            ("serbia and montenegro", NationId::Serbia), ("fry", NationId::Serbia),
            ("croatia", NationId::Croatia), ("slovenia", NationId::Slovenia),
            ("bosnia", NationId::Bosnia),
            ("bosnia and herzegovina", NationId::Bosnia), ("bih", NationId::Bosnia),
        ];
        for (text, id) in cases {
            assert_eq!(NationId::parse(text), Some(*id), "{} no longer parses", text);
        }
        assert_eq!(NationId::parse("atlantis"), None);
        assert_eq!(NationId::parse(""), None);
    }

    #[test]
    fn identity_is_a_code_on_disk_and_an_index_in_memory() {
        // The lesson the technology tree paid for: an index is a runtime
        // detail and must never reach a file.
        let json = serde_json::to_string(&NationId::Iraq).unwrap();
        assert_eq!(json, "\"Iraq\"");
        let back: NationId = serde_json::from_str(&json).unwrap();
        assert_eq!(back, NationId::Iraq);
        assert!(serde_json::from_str::<NationId>("\"Atlantis\"").is_err());
        // ...and an index is never a code.
        assert!(serde_json::from_str::<NationId>("12").is_err());
    }

    #[test]
    fn the_roster_knows_who_touches_whom() {
        assert!(adjacent(NationId::Iraq, NationId::Kuwait));
        assert!(adjacent(NationId::Kuwait, NationId::Iraq));
        // The whole of the Slovene escape in one assertion: Belgrade cannot
        // walk to Ljubljana, and has nothing there it wants.
        assert!(!adjacent(NationId::Serbia, NationId::Slovenia));
        assert_eq!(claim_share(NationId::Serbia, NationId::Slovenia), 0.0);
        assert!(claim_share(NationId::Serbia, NationId::Bosnia) > claim_share(NationId::Serbia, NationId::Croatia));
        assert_eq!(claim_share(NationId::Iraq, NationId::Kuwait), 0.95);
    }
}
