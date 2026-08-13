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
    // The southern border is declared from this end too, because the roster's
    // symmetry check reads the raw rows and not the closure. Canada is not in
    // the roster, so the northern one is correctly absent.
    row("USA", "United States", &["usa", "us", "america"], "NorthAmerica",
        &["Mexico"], &[], true, true, true),

    // The union borders China, Poland, Turkey and Iran; the successor Russia
    // borders only the first two of those, which is a real change the model
    // gets for free from the data rather than from a special case.
    row("USSR", "Soviet Union", &["ussr", "soviets"], "Eurasia",
        &["China", "Poland", "Turkey", "Iran", "Norway", "Finland", "Czechoslovakia", "Hungary", "Romania"], &[], true, true, false),

    // Russia's claim on Ukraine is the 1989 Soviet census: 22.1% of the
    // Ukrainian SSR's people were ethnic Russians, concentrated in Crimea and
    // the Donbas, and Crimea plus the Black Sea Fleet were in open dispute from
    // 1992. https://en.wikipedia.org/wiki/1989_Soviet_census
    // Nothing here schedules 2014; it states the inherited arithmetic and lets
    // the model do what it does with it.
    row("Russia", "Russia", &["rus", "russian federation"], "Eurasia",
        &["China", "Poland", "Ukraine", "Norway", "Finland", "Belarus", "Kazakhstan", "Georgia", "Azerbaijan", "Lithuania", "Latvia", "Estonia"], &[claim("Ukraine", 0.22)], false, true, false),

    row("Ukraine", "Ukraine", &["ukr"], "Eurasia",
        &["Russia", "Poland", "Czechoslovakia", "Hungary", "Romania", "Belarus", "Moldova"], &[], false, false, false),

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
        &["Poland", "France", "Netherlands", "Belgium", "Denmark", "Switzerland", "Austria", "Czechoslovakia"], &[], true, true, true),

    row("UK", "United Kingdom", &["uk", "britain", "gb"], "WesternEurope",
        &["France", "Ireland"], &[], true, true, true),

    row("France", "France", &["fra"], "WesternEurope",
        &["Germany", "Italy", "UK", "Spain", "Belgium", "Switzerland"], &[], true, true, true),

    row("Italy", "Italy", &["ita"], "WesternEurope",
        &["France", "Yugoslavia", "Slovenia", "Switzerland", "Austria"], &[], true, false, false),

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
        &["Kuwait", "SaudiArabia", "Iran", "Turkey", "Syria", "Jordan"],
        &[claim("Kuwait", 0.95), claim("Iran", 0.05)], true, false, false),

    row("Kuwait", "Kuwait", &["kwt"], "MiddleEast",
        &["Iraq", "SaudiArabia"], &[], true, false, false),

    row("SaudiArabia", "Saudi Arabia", &["saudi arabia", "saudi", "ksa"], "MiddleEast",
        &["Iraq", "Kuwait", "Jordan", "UAE", "Oman", "Yemen", "Qatar", "Bahrain"],
        &[], true, false, false),

    // Tehran's claim is the Shatt al-Arab thalweg conceded at Algiers in 1975
    // and torn up in 1980, and behind it the shrine cities of Najaf and Karbala.
    row("Iran", "Iran", &["irn", "persia"], "MiddleEast",
        &["Iraq", "Turkey", "Pakistan", "USSR", "Armenia", "Azerbaijan"], &[claim("Iraq", 0.03)], true, false, false),

    row("SouthKorea", "South Korea", &["south korea", "korea", "rok"], "EastAsia",
        &[], &[], true, false, false),

    row("Poland", "Poland", &["pol"], "EasternEurope",
        &["Germany", "USSR", "Russia", "Ukraine", "Czechoslovakia", "Belarus", "Lithuania"], &[], true, false, false),

    // Brazil touches every South American state except Chile and Ecuador. Six of
    // those ten neighbours are now in the roster and are declared here; Paraguay,
    // Guyana, Suriname and French Guiana are not simulated. Brazil holds no claim
    // on any of them and never has - the Baron of Rio Branco settled all of it by
    // arbitration and purchase between 1895 and 1909, which is why the largest
    // state in the region is also the one with nothing outstanding.
    row("Brazil", "Brazil", &["bra"], "LatinAmerica",
        &["Argentina", "Uruguay", "Bolivia", "Peru", "Colombia", "Venezuela"],
        &[], true, false, false),

    row("Indonesia", "Indonesia", &["idn"], "SoutheastAsia", &[], &[], true, false, false),

    // Egypt's claim is the Hala'ib triangle, and it is the cleanest example in
    // this table of a border that two states draw from two different treaties.
    // The Anglo-Egyptian condominium agreement of 19 January 1899 put the line
    // on the 22nd parallel, which makes Hala'ib Egyptian; the administrative
    // boundary of 4 November 1902 handed the tribes north of it to Khartoum,
    // which is why Sudan administered the triangle in 1990 and Egypt took it by
    // force in 1995. Cairo asserts 1899, Khartoum asserts 1902, and neither
    // claims Bir Tawil — the mirror-image scrap each line awards to the other —
    // which is the reason that patch of desert is the only unclaimed land on
    // earth. 20,580 km2 and on the order of 20,000 people against a Sudan of
    // 26.8m: 0.0008, a grievance stated as the number that keeps it a grievance.
    // https://en.wikipedia.org/wiki/Hala%27ib_Triangle
    row("Egypt", "Egypt", &["egy", "uar"], "NorthAfrica",
        &["Israel", "Libya", "Sudan"], &[claim("Sudan", 0.0008)], true, false, false),

    row("Israel", "Israel", &["isr"], "MiddleEast",
        &["Egypt", "Syria", "Jordan", "Lebanon"], &[], true, false, false),

    // Ankara's claim is the Mosul vilayet, awarded to Iraq by the League of
    // Nations in 1926 over Turkey's objection and pressed again by Ozal during
    // the Gulf crisis: roughly 2m of Iraq's 17m people in the north.
    row("Turkey", "Turkey", &["turkiye", "tur"], "MiddleEast",
        &["Iraq", "Iran", "USSR", "Greece", "Bulgaria", "Georgia", "Armenia", "Azerbaijan", "Syria"], &[claim("Iraq", 0.12)], true, false, false),

    row("Nigeria", "Nigeria", &["nga"], "WestAfrica", &[], &[], true, false, false),

    // The Paracels and Spratlys from the other end, plus the border strips.
    row("Vietnam", "Vietnam", &["viet nam", "vnm"], "SoutheastAsia",
        &["China"], &[claim("China", 0.002)], true, false, false),

    row("Yugoslavia", "Yugoslavia", &["sfry", "yugo"], "Balkans",
        &["Italy", "Austria", "Greece", "Hungary", "Romania", "Bulgaria", "Albania"], &[], true, false, false),

    // The 1991 census is the whole of the Yugoslav tragedy in three numbers.
    // Serbs were 31.2% of Bosnia and 12.2% of Croatia — concentrated in the
    // Krajina — and about 2% of Slovenia, which is why Belgrade's claim on
    // Slovenia is not in this table at all. Croats were 17.4% of Bosnia and
    // declared Herceg-Bosna in November 1991.
    // https://en.wikipedia.org/wiki/Demographic_history_of_Bosnia_and_Herzegovina
    row("Serbia", "Serbia", &["serbia and montenegro", "fry", "srb"], "Balkans",
        &["Croatia", "Bosnia", "Hungary", "Romania", "Bulgaria", "Albania"],
        &[claim("Bosnia", 0.31), claim("Croatia", 0.12)], false, false, false),

    row("Croatia", "Croatia", &["hrv"], "Balkans",
        &["Serbia", "Bosnia", "Slovenia", "Hungary"], &[claim("Bosnia", 0.17)], false, false, false),

    row("Slovenia", "Slovenia", &["svn"], "Balkans",
        &["Croatia", "Italy", "Austria", "Hungary"], &[], false, false, false),

    row("Bosnia", "Bosnia", &["bosnia and herzegovina", "bih"], "Balkans",
        &["Serbia", "Croatia"], &[], false, false, false),

    // Spain's only land border with a nation in this roster is the Pyrenean
    // frontier with France, and — since the North African rows below were added
    // — the fenced perimeters of Ceuta and Melilla, which are land borders with
    // Morocco whatever Rabat calls them. Portugal and Andorra are not
    // simulated, so those borders are correctly absent rather than wrong. The
    // claim is Gibraltar, ceded at Utrecht in
    // 1713, never accepted, raised at the UN Committee of 24 every year and
    // under the negotiating process the Brussels Declaration of 27 November
    // 1984 opened. It is the textbook rounding-error claim this table's
    // doc comment describes: Gibraltar's population at the 1991 census was
    // 28,074 against a United Kingdom of 57.4m, or 0.0005 of the target.
    // A grievance that will never be worth a war, stated as the number that
    // makes it never worth a war.
    // https://en.wikipedia.org/wiki/Brussels_Agreement_(1984)
    row("Spain", "Spain", &["esp", "espana"], "WesternEurope",
        &["France", "Portugal", "Morocco"], &[claim("UK", 0.0005)], true, false, false),

    // ---- Western Europe, the rest of it. -----------------------------------
    // A general note that applies to all eleven rows below, because the shape
    // of the data is the finding: this is the least irredentist neighbourhood
    // on the board. Eleven countries, thirty-odd land borders, and exactly TWO
    // claims between them — and both are rounding errors by design. Western
    // Europe in 1990 is what a region looks like after it has finished
    // arguing about lines on maps, and the war model should read it that way
    // rather than be handed conflicts that were not there.

    // Two land borders, Germany and Belgium, both of them among the oldest
    // uncontested frontiers in Europe. No claims: the Dutch annexation demands
    // of 1945-49 were abandoned and the Elten and Selfkant territories were
    // handed back to West Germany in 1963 under the treaty of 1960.
    row("Netherlands", "Netherlands", &["nld", "holland", "the netherlands"], "WesternEurope",
        &["Germany", "Belgium"], &[], true, false, false),

    // Three land borders in the roster (Luxembourg is not simulated). No
    // claims: the Eupen-Malmedy cantons taken from Germany in 1920 are settled
    // and German-speaking Belgium became one of the three constitutionally
    // recognised communities in the reforms of 1970-89 — the answer to that
    // question was federalism, not a border.
    row("Belgium", "Belgium", &["bel", "belgique", "belgie"], "WesternEurope",
        &["France", "Germany", "Netherlands"], &[], true, false, false),

    // Sweden borders Norway and Finland by land. It does NOT border Denmark:
    // the Oresund is four kilometres at its narrowest and the bridge did not
    // open until July 2000, so the strait is not one an army marches over and
    // the border is correctly absent. No claims — Sweden has not fought a war
    // since 1814 and has no outstanding territorial question with anyone.
    row("Sweden", "Sweden", &["swe", "sverige"], "WesternEurope",
        &["Norway", "Finland"], &[], true, false, false),

    // Four land borders in the roster; Liechtenstein is not simulated. No
    // claims, which for a state that has been neutral since 1815 is the whole
    // point of it.
    row("Switzerland", "Switzerland", &["che", "swiss", "suisse", "schweiz"], "WesternEurope",
        &["France", "Germany", "Italy", "Austria"], &[], true, false, false),

    // Five borders in the roster; Czechoslovakia, Hungary and Liechtenstein
    // are not simulated. Austria borders Yugoslavia through Slovenia, so both
    // the federation and the successor are listed — the same construction the
    // Italy row uses, and it is what lets the model keep the border when
    // Yugoslavia comes apart.
    //
    // NO CLAIM, and this is the deliberate call in this batch. South Tyrol is
    // the obvious candidate: 200,000 German speakers annexed by Italy in 1919,
    // a bombing campaign by the Befreiungsausschuss Sudtirol into the 1960s,
    // and a dispute Austria took to the United Nations General Assembly in
    // 1960 and 1961. But Austria's legal position after the Gruber-De Gasperi
    // Agreement of 5 September 1946 was that of a PROTECTING POWER for an
    // autonomy statute, not a claimant to the territory, and Vienna delivered
    // its formal declaration that the dispute was settled to Rome and the UN
    // on 11 June 1992. Entering a territorial claim here would turn a
    // minority-protection guarantee into a war aim, which is exactly the
    // failure mode this table's doc comment warns about.
    // https://en.wikipedia.org/wiki/Gruber%E2%80%93De_Gasperi_Agreement
    row("Austria", "Austria", &["aut", "osterreich"], "WesternEurope",
        &["Germany", "Italy", "Switzerland", "Yugoslavia", "Slovenia"], &[], true, false, false),

    // One land border, and one of the two claims in this whole region. Olivenca
    // — Olivenza — is a town of some twelve thousand people that Spain took in
    // the War of the Oranges in 1801 and that Article 105 of the Final Act of
    // the Congress of Vienna, 9 June 1815, required be returned. Spain signed
    // and never returned it. Portugal has never recognised Spanish sovereignty
    // de jure, keeps the territory off its own official maps as Spanish, and
    // as recently as the 1980s the Comite Olivenca kept the file open; equally,
    // no Portuguese government has ever pressed it, and the two states joined
    // the European Community together on 1 January 1986. Twelve thousand people
    // against a Spain of 38.9m is 0.0003 of the target: the same species of
    // never-worth-a-war grievance as Spain's own claim on Gibraltar directly
    // above, and stated as the number that keeps it that way.
    // https://en.wikipedia.org/wiki/Olivenza
    row("Portugal", "Portugal", &["prt", "portucale"], "WesternEurope",
        &["Spain"], &[claim("Spain", 0.0003)], true, false, false),

    // Greece's land borders inside this roster are Yugoslavia and Turkey;
    // Albania and Bulgaria are not simulated. Region: WesternEurope rather than
    // Balkans, and it is a judgement rather than a geography lesson. Greece
    // joined NATO in 1952 and the European Community on 1 January 1981, and its
    // diplomatic community in 1990 was Brussels and Washington, which is what
    // the region field feeds (it auto-populates contacts). The two borders that
    // actually mattered militarily — the Yugoslav one and the Turkish one — are
    // stated explicitly above and do not depend on the region at all.
    //
    // NO CLAIM on Turkey, which is the call worth defending. The Greek-Turkish
    // quarrel is real and was very nearly a war in March 1987, but every part
    // of it — the continental shelf, the ten-mile airspace claim, the status of
    // the Aegean islands' militarisation — is a MARITIME and airspace dispute,
    // not a demand for a share of Turkish territory. Cyprus, the other half of
    // it, is a third state and is not in this roster. A claim entered here
    // would tell the war model that Athens wanted land in Anatolia, which no
    // Greek government has said since 1922.
    row("Greece", "Greece", &["grc", "hellas", "ellada"], "WesternEurope",
        &["Yugoslavia", "Turkey"], &[], true, false, false),

    // One land border. The Schleswig question, which produced three wars
    // between 1848 and 1920, was closed by the plebiscites of February and
    // March 1920 and by the Bonn-Copenhagen Declarations of 29 March 1955, in
    // which each state guaranteed the other's minority rights and neither
    // asked for the line to move. No claim.
    row("Denmark", "Denmark", &["dnk", "danmark"], "WesternEurope",
        &["Germany"], &[], true, false, false),

    // Norway holds NATO's only land border with the Soviet Union outside
    // Turkey: 196 km at Kirkenes, settled in 1826, uncontested, and two hours'
    // drive from the Northern Fleet's bases on Kola. Both the union and the
    // successor are listed, because Russia inherits this border where it does
    // not inherit the Turkish or Iranian ones. Svalbard is Norwegian under the
    // Spitsbergen Treaty of 1920 with a Soviet mining settlement on it at
    // Barentsburg; that is a treaty regime, not a claim, and is not entered.
    row("Norway", "Norway", &["nor", "norge"], "WesternEurope",
        &["Sweden", "Finland", "USSR", "Russia"], &[], true, false, false),

    // 1,340 km with the Soviet Union — the longest such border any Western
    // European state has, and the reason the Finnish army in this roster is
    // sized the way it is. No claim, and the absence is the point: Karelia,
    // ceded at Moscow in March 1940 and again at Paris in 1947, is about
    // 11% of pre-war Finnish territory and 400,000 people were evacuated from
    // it, but no Finnish government has ever tabled a demand for its return.
    // The Treaty of Friendship, Cooperation and Mutual Assistance of 1948 was
    // still in force on 1 January 1990. Finlandisation is a foreign policy, and
    // a foreign policy of not asking is correctly a claim table with nothing
    // in it. https://en.wikipedia.org/wiki/Moscow_Peace_Treaty
    row("Finland", "Finland", &["fin", "suomi"], "WesternEurope",
        &["Sweden", "Norway", "USSR", "Russia"], &[], true, false, false),

    // The other of the region's two claims, and much the larger. Articles 2
    // and 3 of Bunreacht na hEireann, adopted by referendum on 1 July 1937,
    // declared that "the national territory consists of the whole island of
    // Ireland, its islands and the territorial seas" — a standing
    // constitutional claim on six counties of the United Kingdom, live and
    // unamended on 1 January 1990, and held by the Supreme Court in
    // McGimpsey v Ireland (1 March 1990, decided inside the game's first
    // quarter) to be a "claim of legal right". It was removed only by the
    // referendum of 22 May 1998 that ratified the Good Friday Agreement.
    // Northern Ireland's population at the 1991 census was 1,577,836 against
    // a United Kingdom of 57.4m: 0.0275 of the target. Fifty-five times
    // Spain's Gibraltar claim and still small — which is the honest shape of
    // it, because for all the deaths this dispute caused between 1969 and
    // 1998, no Irish government at any point contemplated taking the territory
    // by force. https://en.wikipedia.org/wiki/Articles_2_and_3_of_the_Constitution_of_Ireland
    row("Ireland", "Ireland", &["irl", "eire", "republic of ireland"], "WesternEurope",
        &["UK"], &[claim("UK", 0.0275)], true, false, false),

    // ---- Eastern Europe, January 1990 ----------------------------------
    // Five states, and the region field splits them where the map does: the
    // three north of the Danube watershed sit in EasternEurope with Poland,
    // and Bulgaria and Albania sit in Balkans with Yugoslavia, because that
    // is where their contacts actually are. Region membership auto-populates
    // dyads, so this choice decides who is close enough to have an opinion
    // about whom, and putting Albania anywhere but Balkans would leave the
    // one state with a live irredentist claim on Yugoslavia outside its
    // neighbourhood.

    // Czechoslovakia is ONE state here, which is the transcription for 1 January
    // 1990 and stays true for the whole three years the federation had left.
    // Its borders are the four this roster simulates: Germany (the Bohemian
    // frontier with the FRG, ~356km), Poland, Hungary, and the USSR through
    // Transcarpathia — which is why Ukraine, not Russia, inherits that border.
    // Austria and the GDR are not simulated, so those two frontiers are
    // correctly absent rather than wrong.
    // No claims. Czechoslovakia held none in 1990: the Sudeten question ran the
    // other way, and the Benes decrees are a property dispute with Bonn and
    // Vienna rather than a claim on anyone's territory.
    row("Czechoslovakia", "Czechoslovakia", &["csfr", "cssr", "czechoslovak", "cs"], "EasternEurope",
        &["Germany", "Poland", "Hungary", "USSR", "Ukraine"], &[], true, false, false),

    // Hungary borders every state in this region except Albania and Bulgaria,
    // and it is the only nation in the roster that borders all four Yugoslav
    // entities the model carries: the federation, then Serbia through
    // Vojvodina, Croatia along the Drava, and Slovenia's 102km in the Prekmurje.
    // NO CLAIMS, and that is a transcription rather than an omission. Three
    // million Hungarians lived outside Hungary in 1990 — 1.6m in Romania,
    // 0.57m in Slovakia, 0.34m in Vojvodina, 0.16m in Ukraine — and Antall's
    // remark about being "in spirit the prime minister of fifteen million"
    // was read across the region as revisionism. But no Hungarian government
    // after 1947 ever asserted a territorial claim, and Budapest signed basic
    // treaties with Ukraine (1991), Slovakia (1995) and Romania (1996)
    // explicitly renouncing one. The grievance is real and it belongs in the
    // relations matrix, where it is; it is not a border demand and this table
    // will not turn it into one.
    row("Hungary", "Hungary", &["hun", "magyarorszag"], "EasternEurope",
        &["Czechoslovakia", "USSR", "Ukraine", "Romania", "Yugoslavia", "Serbia", "Croatia", "Slovenia"],
        &[], true, false, false),

    // Romania's claim is Bessarabia and northern Bukovina, taken by the Soviet
    // ultimatum of 26 June 1940 under the secret protocol of the
    // Molotov-Ribbentrop pact. The Romanian Parliament's declaration of
    // 24 June 1991 pronounced the pact null and void ab initio and demanded
    // that its consequences be removed — the nearest thing to a formal claim
    // any government in this region made in this period. Stated at what it
    // covers and no more: the Moldavian SSR's 4.34m plus roughly 0.9m in the
    // Chernivtsi and Budjak districts of the Ukrainian SSR, about 5.2m of a
    // Soviet Union of 288.6m, or 0.018. When the union goes, the residue is
    // the Ukrainian portion alone — northern Bukovina, southern Bessarabia and
    // Snake Island, ~0.9m of Ukraine's 51.7m — which stayed genuinely disputed
    // until the 1997 treaty and the ICJ delimitation of 2009.
    // What is NOT here: Moldova is not in this roster, so union with Moldova
    // cannot be modelled and is not smuggled in as a claim on someone else.
    // https://en.wikipedia.org/wiki/Molotov%E2%80%93Ribbentrop_Pact
    row("Romania", "Romania", &["rou", "rom", "romania"], "EasternEurope",
        &["Hungary", "USSR", "Ukraine", "Yugoslavia", "Serbia", "Bulgaria"],
        &[claim("USSR", 0.018), claim("Ukraine", 0.017)], true, false, false),

    // Bulgaria borders Romania along the Danube, Yugoslavia (and after 1992 its
    // Serbian rump) in the west, and Turkey at the Kapitan Andreevo crossing
    // over which 340,000 Bulgarian Turks were driven in the summer of 1989.
    // Greece and Macedonia are not simulated.
    // No claims, which for the state that entered both world wars to get the
    // Macedonian and Aegean territories back is worth stating rather than
    // leaving to inference: Bulgaria renounced them at the Paris peace treaty
    // of 1947 and never reasserted them. Sofia's quarrel with Skopje after
    // 1991 was over whether Macedonians are a nation, not over where the
    // border runs — Bulgaria was the first state to recognise Macedonia's
    // independence, in January 1992. A dispute about identity is not a claim
    // on territory and this table does not have a field for it.
    row("Bulgaria", "Bulgaria", &["bgr", "bul", "balgariya"], "Balkans",
        &["Romania", "Yugoslavia", "Serbia", "Turkey"], &[], true, false, false),

    // Albania's only simulated land borders are with Yugoslavia and, after the
    // federation goes, with the Serbian rump through Kosovo and Montenegro.
    // Greece and Macedonia are not simulated.
    // The claim is Kosovo, and it is the one entry in this region that requires
    // a judgement rather than a transcription, so here is exactly what is and
    // is not being asserted. Kosovo's autonomy was stripped by Belgrade in
    // March 1989, its assembly dissolved in July 1990, and Yugoslav police
    // killed protesters there in January and February 1990 — weeks into this
    // start state. Ramiz Alia's stated position in January 1990 was
    // self-determination for Kosovars WITHIN Yugoslavia, not annexation; Tirana
    // asserted no border change. But in October 1991 Albania became the only
    // state on earth to recognise the self-declared Republic of Kosova, and
    // Albanian irredentism is a continuous fact of the period rather than an
    // invention of this table. Entered at what Kosovo was: about 1.9m of a
    // Yugoslavia of 23.5m, 0.08 of the target. That share against a strength
    // index of 3.0 is a grievance the appetite model will price as unaffordable
    // forever, which is the correct answer — Albania never fired a shot for
    // Kosovo, and somebody else's air force settled it in 1999.
    // https://en.wikipedia.org/wiki/Republic_of_Kosova
    row("Albania", "Albania", &["alb", "shqiperia"], "Balkans",
        &["Yugoslavia", "Serbia"], &[claim("Yugoslavia", 0.08)], true, false, false),

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

    // ---- Latin America ----------------------------------------------------
    //
    // Ten states. The borders below are the ones that exist between nations *in
    // this roster*: Paraguay, Guyana, Suriname, Panama, Guatemala and Belize are
    // not simulated, so those frontiers are correctly absent rather than wrong.
    //
    // Brazil's land border with France by way of French Guiana is deliberately
    // not declared. It is a real 730 km frontier, but it is a border with an
    // overseas department 7,000 km from Paris, and a Franco-Brazilian
    // war-appetite dyad would be an artefact of how the map is drawn rather than
    // a fact about either state. The same reasoning that leaves Spain's border
    // with Morocco out leaves this one out.
    //
    // Three claims exist in this region and no more. What is striking about
    // South America in 1990 is how few there are: the continent had settled
    // almost all of its frontiers by arbitration between 1900 and 1942, and the
    // three that were left over are the three that produced shooting.

    // Buenos Aires holds one claim and it is the Falklands - taken by Britain on
    // 3 January 1833, invaded on 2 April 1982, pressed at the UN Committee of 24
    // every year since 1965. Relations were restored on 19 February 1990 under
    // the Madrid formula, in which both sides agreed to set sovereignty aside
    // rather than settle it, so the game opens with the claim live and the
    // relation merely cold.
    //
    // The share is the arithmetic this table's doc comment demands, and it is
    // brutal. The 1991 Falklands census counted 2,121 people against a United
    // Kingdom of 57.4m: 0.00004 of the target, an order of magnitude smaller
    // than Spain's Gibraltar claim. The appetite model will therefore never find
    // an invasion worth its cost, and that is the correct reading rather than a
    // failure. The 1982 war was not a calculation about what the islands were
    // worth. It was a junta three days after the largest general strike of the
    // dictatorship looking for something else for the country to think about -
    // domestic collapse, which this sim models, and not territorial appetite,
    // which it also models and which was not the mechanism.
    // https://en.wikipedia.org/wiki/1991_Falkland_Islands_census
    row("Argentina", "Argentina", &["arg"], "LatinAmerica",
        &["Chile", "Bolivia", "Brazil", "Uruguay"],
        &[claim("UK", 0.00004)], true, false, false),

    // The 3,145 km with the United States is Mexico's only border inside this
    // roster. No claim, and the absence is the transcription: the territorial
    // question was closed by the Treaty of Guadalupe Hidalgo in 1848 and the
    // Gadsden Purchase in 1853, the last live fragment of it - the Chamizal -
    // was settled in 1963, and no Mexican government since has raised any of it.
    row("Mexico", "Mexico", &["mex"], "LatinAmerica",
        &["USA"], &[], true, false, false),

    row("Chile", "Chile", &["chl"], "LatinAmerica",
        &["Argentina", "Bolivia", "Peru"], &[], true, false, false),

    // Bogota's disputes with Caracas and with Managua are maritime - the Gulf of
    // Venezuela and the Los Monjes cays, San Andres and Providencia - and this
    // table has no way to state them, because `share` is a fraction of a target
    // state and a delimitation over water is not a fraction of anybody. They are
    // in relations_1990.json instead, where a cold dyad with no claim behind it
    // is exactly the right shape for a quarrel that brings frigates out and
    // never brings armies.
    row("Colombia", "Colombia", &["col"], "LatinAmerica",
        &["Venezuela", "Ecuador", "Peru", "Brazil"], &[], true, false, false),

    // Venezuela's real claim is the Essequibo, two thirds of Guyana, asserted
    // since the Geneva Agreement of 1966 reopened the 1899 arbitration. Guyana
    // is not in this roster, so the claim is absent - and it is worth saying
    // plainly that this is the one place in the region where an unsimulated
    // neighbour hides a genuinely large territorial appetite. An agent adding
    // Guyana should add `claim("Guyana", 0.62)` here at the same time.
    row("Venezuela", "Venezuela", &["ven"], "LatinAmerica",
        &["Colombia", "Brazil"], &[], true, false, false),

    row("Peru", "Peru", &["per"], "LatinAmerica",
        &["Ecuador", "Colombia", "Brazil", "Bolivia", "Chile"], &[], true, false, false),

    // An island, and therefore no land borders at all. The one thing Havana
    // wants back - the Guantanamo Bay naval station, held under a 1903 lease
    // Cuba has declared void since 1959 and whose rent cheques it has refused to
    // cash - cannot be written as a claim in this table, because Guantanamo is
    // Cuban territory under foreign occupation rather than a fraction of the
    // United States. The grievance is in the relations file, at -75.
    row("Cuba", "Cuba", &["cub"], "LatinAmerica", &[], &[], true, false, false),

    // The Litoral: 120,000 km2 of Atacama coast and the port of Antofagasta,
    // lost to Chile in the War of the Pacific and ceded by the treaty of 1904.
    // Bolivia has demanded sovereign access to the sea every year since, has had
    // no ambassador in Santiago since 1978, still maintains a navy on Lake
    // Titicaca, and still parades it on the Dia del Mar. It took the case to the
    // ICJ in 2013 and lost in 2018.
    //
    // The share is 0.03 and the choice of denominator is the argument. By area
    // the Litoral is 0.16 of Chile; by people it is far less, because the
    // Antofagasta Region held 410,724 at the 1992 census against a Chile of
    // 13.35m. This table's doc comment settles it - China's claim on India is
    // 0.02 for Aksai Chin precisely because it is "a large area and almost no
    // people" - so the population denominator is the one in use and 0.031 is the
    // figure. A grievance that is permanent, total in Bolivian politics, and
    // still not worth a war against an army three times the size of yours.
    // https://en.wikipedia.org/wiki/Bolivian_littoral_dispute
    row("Bolivia", "Bolivia", &["bol"], "LatinAmerica",
        &["Peru", "Chile", "Argentina", "Brazil"],
        &[claim("Chile", 0.03)], true, false, false),

    // The one claim in this region that did produce a war inside the sim's
    // window. Ecuador signed the Rio Protocol under Peruvian occupation in
    // January 1942, losing roughly 200,000 km2 of Amazon headwater, and
    // President Velasco Ibarra declared the protocol null in 1960 - a position
    // every Ecuadorian government held until 1998. They shot at each other at
    // Paquisha in January 1981 and fought the Cenepa war in January-February
    // 1995, five years after this file opens.
    //
    // 0.05 rather than the 0.16 the disputed area is of Peru, for the same
    // population reason as Bolivia's row above: the Cordillera del Condor is
    // some of the emptiest land in South America. It is set an order of
    // magnitude above the Falklands and the Litoral because unlike those two it
    // is a live, undemarcated, patrolled frontier that both armies were standing
    // on. The Cenepa war should be reachable from this number and the border,
    // and it should be reachable without being certain.
    // https://en.wikipedia.org/wiki/Cenepa_War
    row("Ecuador", "Ecuador", &["ecu"], "LatinAmerica",
        &["Colombia", "Peru"], &[claim("Peru", 0.05)], true, false, false),

    row("Uruguay", "Uruguay", &["ury", "uru"], "LatinAmerica",
        &["Brazil", "Argentina"], &[], true, false, false),

    // ===== Middle East =====

    // Syria borders Iraq, Turkey, Israel, Jordan and Lebanon in this roster.
    // Two claims, both of them lines drawn by departing empires and never
    // accepted.
    //
    // The Golan Heights: taken in June 1967, and on 14 December 1981 the Knesset
    // extended Israeli "law, jurisdiction and administration" over them, which
    // Security Council Resolution 497 declared null and void three days later,
    // on 17 December. About 1,200 km2 against the roughly 22,000 km2 Israel
    // administers
    // inside the Green Line plus the Golan, so 0.05 of the target.
    // https://en.wikipedia.org/wiki/Golan_Heights_Law
    //
    // Hatay: the Sanjak of Alexandretta, which France detached from its Syrian
    // mandate and handed to Turkey in June 1939 to keep Ankara out of the coming
    // war. Syria has never recognised the transfer and still prints the province
    // inside its own borders. Hatay held about 1.1m of Turkey's 56m people in
    // 1990, so 0.02 - scored on population exactly as the Turkey-on-Iraq Mosul
    // claim four rows above is, so the two are the same kind of object.
    // https://en.wikipedia.org/wiki/Hatay_State
    //
    // NOT entered, and the omission is the argued half of this row: Syria's
    // relationship to Lebanon. Damascus refused to exchange ambassadors with
    // Beirut until 2008 precisely because Greater Syria doctrine never conceded
    // that Lebanon was a separate country, and after the Taif Accord of 22
    // October 1989 and the assault on Aoun of 13 October 1990 Syria was the
    // effective government of the place. But `claim.share` in this table is the
    // fraction of a target a war would annex, and Syria never sought to annex
    // Lebanon - it sought to control it, which is occupation and veto, not
    // irredentism. The border and the relations value carry that; a fabricated
    // annexation share would not.
    row("Syria", "Syria", &["syr"], "MiddleEast",
        &["Iraq", "Turkey", "Israel", "Jordan", "Lebanon"],
        &[claim("Israel", 0.05), claim("Turkey", 0.02)], true, false, false),

    // Jordan borders Iraq, Saudi Arabia, Israel, Syria - and the claims list is
    // empty for a reason worth stating, because it is the rarest thing in this
    // table: a claim that was formally given up. On 31 July 1988 King Hussein
    // announced the severance of Jordan's legal and administrative ties to the
    // West Bank, dissolving the Chamber's Palestinian seats and ending the
    // annexation Jordan had held since 1950. The 1989 election was the first
    // fought on the East Bank alone. Jordan wants nothing from anybody here.
    // https://en.wikipedia.org/wiki/Jordanian_disengagement_from_the_West_Bank
    row("Jordan", "Jordan", &["jor"], "MiddleEast",
        &["Iraq", "SaudiArabia", "Israel", "Syria"], &[], true, false, false),

    // Lebanon borders Syria and Israel, and in January 1990 both of them had
    // armies inside it: roughly 40,000 Syrian troops under the Arab Deterrent
    // Force mandate, and the Israeli "security zone" north of the 1978 line.
    // No claims, and again the absence is the fact. The Shebaa Farms claim -
    // the one thing Lebanon does assert against Israel - was not made until
    // 2000, when Hezbollah needed a reason for the resistance to continue after
    // the withdrawal; in 1990 those farms were understood to be Syrian.
    row("Lebanon", "Lebanon", &["lbn"], "MiddleEast",
        &["Syria", "Israel"], &[], true, false, false),

    // The Emirates border Saudi Arabia and Oman. They do NOT border Qatar: the
    // Khawr al Udayd corridor of Saudi territory runs between them, which is
    // why Qatar's only land neighbour below is Riyadh.
    //
    // The claim is Abu Musa and the Greater and Lesser Tunbs, which Iranian
    // troops occupied on 30 November 1971 - the day before the federation was
    // proclaimed and two days before Britain's treaties lapsed. The UAE has
    // raised it at every Arab League and UN session since. It is a
    // rounding-error claim by any measure: about 30 km2 against Iran's
    // 1,648,000 (0.00002) and roughly 800 inhabitants against 56m (0.00001).
    // Entered at 0.00002, which is the number that says a grievance is
    // permanent and is never going to be worth a war.
    // https://en.wikipedia.org/wiki/Greater_and_Lesser_Tunbs
    row("UAE", "United Arab Emirates", &["uae", "emirates", "are"], "MiddleEast",
        &["SaudiArabia", "Oman"], &[claim("Iran", 0.00002)], true, false, false),

    // Qatar's one land border is with Saudi Arabia. Its claim is the Hawar
    // Islands, and this is not a paper dispute: on 26 April 1986 Qatari special
    // forces helicoptered onto Fasht al-Dibal, an artificial island Bahrain was
    // building, and took twenty-nine workers prisoner. Saudi mediation stopped
    // it. Qatar filed the whole question at the International Court of Justice
    // on 8 July 1991, eighteen months into this game, and lost Hawar there on
    // 16 March 2001. Hawar is about 52 km2 against Bahrain's 765 km2 including
    // it, so 0.07 of the target - a real slice of a very small country, which
    // is why two allies came close to shooting over it.
    // https://en.wikipedia.org/wiki/Hawar_Islands
    row("Qatar", "Qatar", &["qat"], "MiddleEast",
        &["SaudiArabia"], &[claim("Bahrain", 0.07)], true, false, false),

    // Oman borders Saudi Arabia across the Rub' al Khali, the UAE, and Yemen.
    // No claims, and 1990 is the year that became true: the Saudi-Omani border
    // was finally settled by treaty on 21 March 1990, eleven weeks into this
    // game, and the Omani-Yemeni border on 1 October 1992. Muscat is the state
    // in this region that finished drawing its lines by agreement.
    // https://en.wikipedia.org/wiki/Oman%E2%80%93Saudi_Arabia_border
    row("Oman", "Oman", &["omn", "muscat"], "MiddleEast",
        &["SaudiArabia", "UAE", "Yemen"], &[], true, false, false),

    // Yemen. See yemen.json for the unification problem in full; the short of it
    // is that on 1 January 1990 there were two Yemens and on 22 May 1990 there
    // was one, and this roster carries the unified Republic from the start
    // because SPHERES creates successor states when a federation comes apart and
    // has no machinery for the reverse, and BIBLE section 5 refuses the scripted
    // event that would be the alternative.
    //
    // The claim is Asir, Jizan and Najran, which the Imam ceded to Ibn Saud in
    // the Treaty of Taif of 20 May 1934 after losing the war - on twenty-year
    // renewable terms that Yemen spent the next sixty-six years declining to
    // treat as permanent. The frontier east of Najran was never demarcated at
    // all until the Jeddah Treaty of 12 June 2000, and Saudi and Yemeni forces
    // shot at each other over it in 1994, 1995 and 1998. Those three provinces
    // held roughly 2.2m of Saudi Arabia's 16m people, so 0.13 - scored on
    // population, the same method as Turkey-on-Iraq and Syria-on-Turkey above.
    // https://en.wikipedia.org/wiki/Treaty_of_Taif
    row("Yemen", "Yemen", &["yem", "north yemen", "south yemen"], "MiddleEast",
        &["SaudiArabia", "Oman"], &[claim("SaudiArabia", 0.13)], true, false, false),

    // Bahrain's neighbour list has exactly one entry and it is a bridge. The
    // King Fahd Causeway opened on 25 November 1986: twenty-five kilometres of
    // road and embankment from Al Khobar to Bahrain, built by Saudi money and
    // wide enough to drive an armoured column across, which is precisely what
    // Saudi Arabia did on 14 March 2011. This table's doc comment admits "the
    // two straits narrow enough to march across"; a causeway is not a strait,
    // it is better than one, and leaving it out would make Bahrain an island
    // nobody can reach in a model whose whole war logic reads adjacency.
    //
    // The claim is Zubarah, on the Qatari mainland, from which the Al Khalifa
    // ruled before they took Bahrain in 1783 and which Bahrain pressed against
    // Doha until the ICJ awarded it to Qatar on 16 March 2001. Roughly 1% of
    // Qatar. The reciprocal Qatari claim on Hawar is four rows up: the two
    // smallest states in this roster hold claims on each other and nearly went
    // to war over them in 1986, which is the sort of thing a derived appetite
    // table gets right and an enumerated one would never have bothered to list.
    // https://en.wikipedia.org/wiki/Zubarah
    row("Bahrain", "Bahrain", &["bhr"], "MiddleEast",
        &["SaudiArabia"], &[claim("Qatar", 0.01)], true, false, false),

    // ---- North Africa ----------------------------------------------------
    // Five states along one coast, and the thing to notice is how few claims
    // there are between them. The Maghreb's quarrels in 1990 were over a
    // territory that is not in this roster at all — the Western Sahara, which
    // Morocco holds and the Polisario Front contests from Algerian soil — so
    // the Algiers-Rabat dyad is a proxy war carried on a border neither side
    // was seriously trying to move. The Arab Maghreb Union treaty of 17
    // February 1989 is the other half of that: these five had just signed one.
    // Modelling the region as a set of irredentist appetites would invent a
    // decade of wars that did not happen.

    // Algeria borders every other Maghreb state in this roster and claims none
    // of them. Its 1963 war with Morocco was fought on Moroccan initiative and
    // over Algerian ground, its 1990 quarrel with Rabat was over the Western
    // Sahara, and Tindouf — the Polisario's base since 1976 — was the thing
    // Algeria was defending, not seeking. The other long frontiers, with Mali,
    // Niger, Mauritania and the Western Sahara, are borders with states this
    // roster does not carry, so they are absent rather than wrong.
    row("Algeria", "Algeria", &["dza", "alg", "algerie"], "NorthAfrica",
        &["Morocco", "Tunisia", "Libya"], &[], true, false, false),

    // Morocco is the one North African state in 1990 with live claims on two
    // neighbours it can walk to, and both are rounding errors by design.
    //
    // On Spain: Ceuta and Melilla, held since 1668 and 1497, claimed by every
    // Moroccan government since independence in 1956 and raised at the UN
    // Committee of 24 in the same breath Spain raises Gibraltar. Their 1991
    // census populations were 67,615 and 56,600 — 124,215 against a Spain of
    // 38.87m, or 0.0032.
    //
    // On Algeria: the Tindouf frontier zone, the object of the Sand War of
    // October 1963 and of the Amgala clashes of January 1976. Rabat signed the
    // border convention of 15 June 1972 and then did not ratify it until 22 May
    // 1992, which is precisely why the claim is live in January 1990 and dead
    // two years later. Tindouf wilaya held 16,339 people at the 1987 Algerian
    // census against 23.04m, or 0.0007, entered at 0.001. Stated conservatively:
    // the maximal Moroccan claim reached Bechar as well, which would put it near
    // 0.009, but Tindouf is what Rabat was actually still asserting by 1990.
    //
    // The Western Sahara, which is the claim that mattered, is not in this
    // roster, so it is correctly absent. Mauritania likewise.
    // https://en.wikipedia.org/wiki/Sand_War
    row("Morocco", "Morocco", &["mar", "maroc", "al-maghrib"], "NorthAfrica",
        &["Algeria", "Spain"],
        &[claim("Spain", 0.0032), claim("Algeria", 0.001)], true, false, false),

    // Tunisia claims nothing from anybody, which is not an oversight: its one
    // territorial dispute of the era, the continental shelf in the Gulf of
    // Gabes, went to the International Court of Justice and was decided on 24
    // February 1982 and again on 10 December 1985, and Tunis accepted both.
    // A state that litigates its borders is a state with no claims to enter.
    row("Tunisia", "Tunisia", &["tun", "tunisie"], "NorthAfrica",
        &["Algeria", "Libya"], &[], true, false, false),

    // Libya's irredentism was real and enormous and points entirely off this
    // map: the Aouzou Strip, annexed from Chad in 1973, fought over until the
    // rout of March 1987 and surrendered to the ICJ in 1994. Chad is not in
    // this roster, so Libya opens with no claims — which reads as pacific and
    // is not. What the borders do carry is reach: Libya touches Egypt, with
    // whom it fought a four-day war in July 1977, and Sudan, whose government
    // it had spent a decade trying to choose.
    // https://en.wikipedia.org/wiki/Aouzou_Strip
    row("Libya", "Libya", &["lby", "libyan arab jamahiriya", "jamahiriya"], "NorthAfrica",
        &["Algeria", "Tunisia", "Egypt", "Sudan"], &[], true, false, false),

    // Sudan claims nothing. Under the 1902 administrative line it prefers, the
    // Hala'ib triangle is already Sudanese and Bir Tawil is already Egyptian,
    // so there is nothing left for Khartoum to ask for — the asymmetry is in
    // Egypt's row above, not this one. Sudan's war in 1990 is entirely
    // internal, which is what the separatism figure in sudan.json carries.
    // The long southern and western frontiers — Ethiopia, Chad, Uganda, Kenya,
    // Zaire, the Central African Republic — are with states this roster does
    // not have.
    row("Sudan", "Sudan", &["sdn", "as-sudan"], "NorthAfrica",
        &["Egypt", "Libya"], &[], true, false, false),
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
    Netherlands => "Netherlands",
    Belgium => "Belgium",
    Sweden => "Sweden",
    Switzerland => "Switzerland",
    Austria => "Austria",
    Portugal => "Portugal",
    Greece => "Greece",
    Denmark => "Denmark",
    Norway => "Norway",
    Finland => "Finland",
    Ireland => "Ireland",
    Czechoslovakia => "Czechoslovakia",
    Hungary => "Hungary",
    Romania => "Romania",
    Bulgaria => "Bulgaria",
    Albania => "Albania",
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
    Argentina => "Argentina",
    Mexico => "Mexico",
    Chile => "Chile",
    Colombia => "Colombia",
    Venezuela => "Venezuela",
    Peru => "Peru",
    Cuba => "Cuba",
    Bolivia => "Bolivia",
    Ecuador => "Ecuador",
    Uruguay => "Uruguay",
    Syria => "Syria",
    Jordan => "Jordan",
    Lebanon => "Lebanon",
    UAE => "UAE",
    Qatar => "Qatar",
    Oman => "Oman",
    Yemen => "Yemen",
    Bahrain => "Bahrain",
    Algeria => "Algeria",
    Morocco => "Morocco",
    Tunisia => "Tunisia",
    Libya => "Libya",
    Sudan => "Sudan",
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
