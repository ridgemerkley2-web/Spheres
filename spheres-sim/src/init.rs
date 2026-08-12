use crate::world::*;


/// Transcribed-not-invented: approximate 1990 historical starting conditions.
pub fn world_1990(rules: GameRules) -> WorldState {
    use crate::nations::ids::*;
    use EconomySystem::*;

    // (id, system, auth, gdp $bn, pop m, tfp, infl, rate, tax, mil%, state_inv%, priv_inv%, debt%, oil mbd, stability, separatism, mil, nuclear)
    let rows: Vec<Nation> = vec![
        n(USA,        Market,  0.10, 5980.0, 250.0, 0.013, 0.054, 0.080, 0.27, 0.055, 0.035, 0.14, 0.62, 7.4, 78.0, 0.02, 100.0, true),
        n(USSR,       Command, 0.85, 1600.0, 289.0, -0.008, 0.06, 0.05, 0.40, 0.120, 0.220, 0.02, 0.45, 11.4, 42.0, 0.55, 92.0, true),
        n(China,      Command, 0.90,  390.0, 1135.0, 0.030, 0.031, 0.08, 0.22, 0.025, 0.240, 0.06, 0.08, 2.8, 62.0, 0.10, 32.0, true),
        n(Japan,      Market,  0.12, 3140.0, 124.0, 0.018, 0.031, 0.060, 0.30, 0.010, 0.050, 0.24, 0.64, 0.0, 82.0, 0.00, 22.0, false),
        n(Germany,    Market,  0.10, 1710.0,  79.0, 0.016, 0.027, 0.070, 0.38, 0.028, 0.038, 0.16, 0.41, 0.0, 80.0, 0.03, 38.0, false),
        n(UK,         Market,  0.10, 1090.0,  57.0, 0.014, 0.095, 0.140, 0.35, 0.040, 0.030, 0.14, 0.33, 1.9, 74.0, 0.06, 42.0, true),
        n(France,     Market,  0.12, 1270.0,  57.0, 0.014, 0.034, 0.095, 0.42, 0.036, 0.038, 0.15, 0.35, 0.0, 75.0, 0.03, 44.0, true),
        n(Italy,      Market,  0.14, 1180.0,  57.0, 0.012, 0.065, 0.120, 0.38, 0.021, 0.032, 0.15, 0.94, 0.0, 68.0, 0.05, 26.0, false),
        n(India,      Market,  0.30,  320.0, 870.0, 0.020, 0.090, 0.100, 0.16, 0.032, 0.070, 0.10, 0.55, 0.7, 60.0, 0.15, 30.0, false),
        n(Pakistan,   Market,  0.55,   40.0, 108.0, 0.016, 0.090, 0.100, 0.14, 0.062, 0.050, 0.08, 0.50, 0.0, 52.0, 0.12, 16.0, false),
        n(Iraq,       Command, 0.95,   60.0,  17.0, 0.005, 0.180, 0.060, 0.30, 0.200, 0.100, 0.04, 1.10, 2.8, 48.0, 0.20, 26.0, false),
        n(Kuwait,     Market,  0.60,   18.0,   2.1, 0.010, 0.035, 0.070, 0.05, 0.050, 0.060, 0.10, 0.10, 1.7, 70.0, 0.00, 2.0, false),
        n(SaudiArabia,Market,  0.85,  117.0,  16.0, 0.008, 0.021, 0.070, 0.08, 0.140, 0.080, 0.09, 0.35, 6.4, 66.0, 0.04, 14.0, false),
        n(Iran,       Command, 0.85,  120.0,  56.0, 0.006, 0.090, 0.060, 0.18, 0.030, 0.090, 0.06, 0.30, 3.1, 55.0, 0.10, 20.0, false),
        n(SouthKorea, Market,  0.35,  280.0,  43.0, 0.028, 0.086, 0.100, 0.18, 0.037, 0.060, 0.30, 0.13, 0.0, 66.0, 0.00, 18.0, false),
        n(Poland,     Market,  0.35,   66.0,  38.0, 0.010, 0.550, 0.300, 0.30, 0.027, 0.050, 0.10, 0.80, 0.0, 55.0, 0.05, 12.0, false),
        // Brazil: World Bank has 1990 GDP at $385bn and 149.1m people, with CPI
        // inflation of 2948% — the year of the Collor Plan. Nothing here scripts
        // that plan: the game opens in January, ten weeks before Collor took
        // office and froze prices and confiscated the savings accounts. What the
        // start state carries is the position he inherited — prices doubling
        // every six weeks, and an overnight rate chasing them (the World Bank's
        // 1990 deposit rate for Brazil is 9394%). The disinflation is left to the
        // sim's central bank, which is the same treatment Yugoslavia gets below.
        // Oil: Petrobras pumped 0.63 mbd from Campos against 1.4 mbd of demand,
        // so Brazil was buying, not selling.
        n(Brazil,     Market,  0.20,  385.0, 149.1, -0.005, 2.950, 2.900, 0.29, 0.024, 0.050, 0.15, 0.40, 0.63, 40.0, 0.02, 16.0, false),
        // Indonesia: $106bn and 183.5m (1990 census: 179m, WB series 183.5m).
        // Suharto's New Order is 24 years old and unchallenged; the strain is at
        // the edges — Aceh, where the military operations zone was declared in
        // 1990, plus East Timor and Papua. Bank Indonesia was still running the
        // 1990 "Sumarlin shock" squeeze, hence a policy rate far above inflation.
        n(Indonesia,  Market,  0.80,  106.0, 183.5, 0.020, 0.078, 0.150, 0.19, 0.014, 0.090, 0.24, 0.46, 1.50, 62.0, 0.35, 8.0, false),
        // Egypt: $43bn on the World Bank's series and 58.4m people. Mubarak's
        // Egypt is the second-largest recipient of US aid after Israel, and its
        // debt is crushing — external debt alone is 79% of GNI on the eve of the
        // Paris Club write-off it earned by joining the coalition. A modest
        // producer: 0.87 mbd from the Gulf of Suez and the Western Desert.
        n(Egypt,      Market,  0.75,   43.0,  58.4, 0.008, 0.168, 0.140, 0.20, 0.035, 0.130, 0.16, 1.10, 0.87, 58.0, 0.05, 14.0, false),
        // Israel: $62bn and 4.66m people, and 12.4% of GDP on defence — the
        // highest share in the world outside the Gulf. Central government debt is
        // 132% of GDP, the hangover from 1985's stabilisation. The military index
        // is set well above what $62bn would buy anyone else: air superiority
        // over every neighbour, US kit, and reserves that put half a million
        // people in uniform in days. Nuclear is true. Israel has never confirmed
        // it and never will — the policy is called amimut, ambiguity — but the
        // Dimona arsenal dates to the late 1960s and every capital in the region
        // planned around it in 1990. Separatism is the First Intifada, running
        // since December 1987.
        n(Israel,     Market,  0.15,   62.0,   4.66, 0.018, 0.173, 0.155, 0.38, 0.124, 0.060, 0.16, 1.32, 0.0, 62.0, 0.40, 34.0, true),
        // Turkey: $151bn and 56.0m, inflation at 60% and interbank rates chasing
        // it. Ozal's Turkey is NATO's southern flank and about to become the
        // hinge of the coalition's northern front. Separatism is the PKK war in
        // the southeast, escalating every year since 1984.
        n(Turkey,     Market,  0.35,  151.0,  56.0, 0.014, 0.603, 0.500, 0.15, 0.035, 0.080, 0.17, 0.30, 0.07, 55.0, 0.30, 24.0, false),
        // Nigeria: $54bn and 97.1m, pumping 1.8 mbd — the swing African producer,
        // and the reason the Gulf crisis was good for Lagos. Babangida's military
        // government survived the Orkar coup attempt in April 1990; separatism is
        // what Biafra left behind and the Delta is beginning to add to. Tax take
        // is the non-oil figure; the sim adds petroleum revenue separately.
        n(Nigeria,    Market,  0.85,   54.0,  97.1, 0.000, 0.074, 0.175, 0.08, 0.008, 0.060, 0.09, 0.65, 1.80, 35.0, 0.30, 6.0, false),
        // Vietnam: the World Bank's 1990 GDP is $6.5bn for 65.5m people, which is
        // an artefact of an official exchange rate nobody transacted at — but it
        // is the transcribed figure, and it is the one the sim's catch-up term
        // reads. Inflation ran 67% in 1990 as Doi Moi let prices go. Still a
        // command economy: the reforms are four years old and the plan is intact.
        // The army is the outlier — 1.2m men demobilising out of Cambodia, which
        // is why the military index sits near South Korea's on a hundredth of the
        // GDP. Soviet aid ends this year and there is nothing to replace it.
        n(Vietnam,    Command, 0.92,    6.5,  65.5, 0.022, 0.670, 0.550, 0.14, 0.079, 0.070, 0.06, 1.20, 0.06, 55.0, 0.02, 18.0, false),
        // Yugoslavia: GDP ~$88bn and 23.5m people. The game opens three weeks
        // into Markovic's stabilisation programme (18 Dec 1989) — hence the
        // punishing policy rate against still-enormous inflation — and the same
        // month the League of Communists broke up at its 14th Congress, which
        // is why separatism starts higher than the USSR's 0.55.
        n(Yugoslavia, Command, 0.65,   88.0,  23.5, -0.010, 0.600, 0.300, 0.36, 0.048, 0.120, 0.08, 0.28, 0.05, 44.0, 0.72, 20.0, false),
    ];

    let mut w = WorldState {
        rng: Rng::new(rules.seed),
        rules,
        year: 1990,
        month: 1,
        nations: rows,
        relations: Relations::default(),
        sanctions: vec![],
        wars: vec![],
        statecraft: Statecraft::default(),
        oil_price: 20.0,
        headlines: vec![],
        flags: vec![],
        player: None,
    };

    // Japan's bubble is at its peak on day one.
    w.nation_mut(Japan).bubble = 0.95;

    // Cold War-shaped relations
    let pairs: &[(NationId, NationId, f64)] = &[
        (USA, USSR, -45.0), (USA, China, -10.0), (USA, Japan, 60.0),
        (USA, Germany, 65.0), (USA, UK, 80.0), (USA, France, 60.0),
        (USA, Italy, 60.0), (USA, SouthKorea, 65.0), (USA, SaudiArabia, 50.0),
        (USA, Kuwait, 45.0), (USA, Iraq, -20.0), (USA, Iran, -60.0),
        (USA, India, 5.0), (USA, Pakistan, 30.0), (USA, Poland, 25.0),
        (USSR, China, -15.0), (USSR, India, 40.0), (USSR, Iraq, 30.0),
        (USSR, Poland, 10.0), (USSR, Germany, -10.0),
        (Iraq, Kuwait, -35.0), (Iraq, SaudiArabia, -25.0), (Iraq, Iran, -50.0),
        (India, Pakistan, -55.0), (China, India, -25.0),
        (UK, France, 55.0), (UK, Germany, 55.0), (France, Germany, 60.0),
        (Japan, SouthKorea, 15.0), (China, Japan, -5.0),
        (Kuwait, SaudiArabia, 45.0), (UK, Kuwait, 50.0),
        // Non-aligned: on cordial terms with both blocs and neither's client.
        // Bonn's warmth here is what makes its later push for recognition bite.
        (Yugoslavia, USA, 25.0), (Yugoslavia, Germany, 30.0), (Yugoslavia, Italy, 25.0),
        (Yugoslavia, UK, 20.0), (Yugoslavia, France, 20.0), (Yugoslavia, USSR, 5.0),
        (Yugoslavia, Poland, 10.0), (Yugoslavia, India, 30.0),

        // Brazil: friendly with everyone and committed to nobody. The 1980s debt
        // crisis is what Washington means to Brasilia, so the warmth is real but
        // thin, and the Latin American nuclear rivalry with Argentina ended at
        // Guadalajara in 1991.
        (Brazil, USA, 45.0), (Brazil, Japan, 30.0), (Brazil, Germany, 25.0),
        (Brazil, UK, 20.0), (Brazil, France, 20.0), (Brazil, Italy, 25.0),
        (Brazil, USSR, 5.0), (Brazil, China, 10.0), (Brazil, India, 15.0),

        // Indonesia: an anti-communist client of the West since 1965, and Japan's
        // largest aid recipient. Relations with Beijing were suspended outright
        // in 1967 over the coup and are only restored in August 1990 — which is
        // why they start negative and have to be repaired.
        (Indonesia, USA, 40.0), (Indonesia, Japan, 55.0), (Indonesia, China, -20.0),
        (Indonesia, USSR, 0.0), (Indonesia, India, 20.0), (Indonesia, SaudiArabia, 25.0),
        (Indonesia, SouthKorea, 25.0), (Indonesia, Vietnam, 5.0),

        // Egypt post-Camp David: the peace with Israel is cold but eleven years
        // old, and it bought Cairo a permanent American subsidy. The suspension
        // from the Arab League ended in 1989 and Egypt sits with Iraq in the Arab
        // Cooperation Council as the game opens — Cairo's turn against Baghdad in
        // August is a choice the sim has to make, not a fact it starts with.
        (Egypt, USA, 60.0), (Egypt, Israel, 20.0), (Egypt, Iraq, 20.0),
        (Egypt, SaudiArabia, 55.0), (Egypt, Kuwait, 45.0), (Egypt, USSR, 15.0),
        (Egypt, UK, 35.0), (Egypt, France, 35.0), (Egypt, Iran, -20.0),
        (Egypt, Turkey, 25.0), (Egypt, China, 15.0), (Egypt, India, 25.0),

        // Israel: the American relationship is the deepest in the game. Moscow
        // broke relations in 1967 and does not restore them until October 1991,
        // even as it lets its Jews leave. Ankara is the one Muslim capital with
        // full ties.
        (Israel, USA, 80.0), (Israel, Germany, 45.0), (Israel, UK, 30.0),
        (Israel, France, 20.0), (Israel, Italy, 20.0), (Israel, Turkey, 35.0),
        (Israel, USSR, -25.0), (Israel, Iraq, -70.0), (Israel, Iran, -55.0),
        (Israel, SaudiArabia, -50.0), (Israel, Kuwait, -45.0), (Israel, India, 5.0),

        // Turkey in NATO: forty years in the alliance, two million of its people
        // working in Germany, and Incirlik. Baghdad is a customer, not a friend —
        // the Kirkuk-Yumurtalik pipeline is Iraq's main outlet and Ozal shuts it
        // in August 1990.
        (Turkey, USA, 60.0), (Turkey, Germany, 50.0), (Turkey, UK, 50.0),
        (Turkey, France, 40.0), (Turkey, Italy, 45.0), (Turkey, USSR, -20.0),
        (Turkey, Iraq, 10.0), (Turkey, Iran, 5.0), (Turkey, SaudiArabia, 25.0),
        (Turkey, Pakistan, 30.0), (Turkey, Poland, 10.0),

        // Nigeria: Commonwealth ties to London, oil ties to Washington, and the
        // non-aligned habits of a country that has led African diplomacy since
        // independence.
        (Nigeria, UK, 45.0), (Nigeria, USA, 35.0), (Nigeria, France, 10.0),
        (Nigeria, SaudiArabia, 20.0), (Nigeria, India, 20.0), (Nigeria, Brazil, 15.0),

        // Vietnam post-Cambodia: the last troops came home in September 1989 but
        // nothing has been forgiven. Washington's embargo is fifteen years old and
        // has four more to run; Beijing fought a border war in 1979, kept
        // shelling into 1988, and sank three Vietnamese ships off Johnson Reef
        // that March. Moscow is the patron whose money stops arriving this year.
        (Vietnam, USSR, 60.0), (Vietnam, China, -45.0), (Vietnam, USA, -50.0),
        (Vietnam, India, 35.0), (Vietnam, Japan, 0.0), (Vietnam, France, 10.0),
        (Vietnam, SouthKorea, -10.0), (Vietnam, Poland, 25.0),
    ];
    for (a, b, v) in pairs {
        w.set_relation(*a, *b, *v);
    }
    w
}

#[allow(clippy::too_many_arguments)]
fn n(
    id: NationId, system: EconomySystem, auth: f64, gdp: f64, pop: f64, tfp: f64,
    infl: f64, rate: f64, tax: f64, mil: f64, sinv: f64, pinv: f64, debt: f64,
    oil: f64, stab: f64, sep: f64, strength: f64, nuclear: bool,
) -> Nation {
    Nation {
        id,
        alive: true,
        system,
        authoritarianism: auth,
        gdp,
        population: pop,
        tfp_trend: tfp,
        inflation: infl,
        interest_rate: rate,
        tax_rate: tax,
        mil_spend_gdp: mil,
        state_invest_gdp: sinv,
        priv_invest_gdp: pinv,
        debt_gdp: debt,
        oil_mbd: oil,
        bubble: 0.0,
        growth_last: 0.0,
        stability: stab,
        separatism: sep,
        mil_strength: strength,
        war_exhaustion: 0.0,
        nuclear,
        // Derived, not transcribed: a government's standing in January 1990 is
        // not a number anyone recorded, so it is read off the conditions that
        // are. Order held and prices under control is a government with room to
        // act; the ones that opened at 55% inflation had none.
        political_capital: crate::politics::seated_political_capital(stab, infl, auth),
        tech: crate::tech::TechState::new(tfp),
    }
}
