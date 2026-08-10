use crate::world::*;

/// Transcribed-not-invented: approximate 1990 historical starting conditions.
pub fn world_1990(rules: GameRules) -> WorldState {
    use EconomySystem::*;
    use NationId::*;

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
        relations: vec![],
        sanctions: vec![],
        wars: vec![],
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
    }
}
