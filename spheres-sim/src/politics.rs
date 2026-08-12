use crate::war;
use crate::world::*;

/// Monthly politics & AI tick: central banks, elections, collapses, AI wars, peace.
/// Where a government's standing sits when it has neither earned nor spent
/// anything — a function of the order it keeps and the prices it holds, with
/// coercion substituting for consent the more authoritarian it is.
pub fn seated_political_capital(stability: f64, inflation: f64, authoritarianism: f64) -> f64 {
    let order = (stability / 100.0).clamp(0.0, 1.0);
    let prices = (1.0 - (inflation / 0.30).clamp(0.0, 1.0)).clamp(0.0, 1.0);
    let consent = 0.60 * order + 0.40 * prices;
    // A police state holds a floor it did not earn and a ceiling it cannot pass.
    let floor = 18.0 * authoritarianism;
    (floor + (78.0 - 30.0 * authoritarianism) * consent).clamp(0.0, 100.0)
}

/// The stock walks toward what the government's record currently justifies. It
/// is a stock and not a flow: a government that has spent everything cannot act
/// again until it has delivered something, which is the whole point of having
/// the currency at all.
fn political_capital(w: &mut WorldState) {
    let ids: Vec<NationId> = w.nations.iter().filter(|n| n.alive).map(|n| n.id).collect();
    for id in ids {
        let composition = crate::government::standing_modifier(w, id);
        let n = w.nation_mut(id);
        let mut target = seated_political_capital(n.stability, n.inflation, n.authoritarianism);
        // Delivering growth is the ordinary way a government earns the right to
        // ask for anything, and a recession is the ordinary way it loses it.
        target += (n.growth_last * 100.0).clamp(-6.0, 6.0) * 2.2;
        // A war costs a government at home long before it costs it at the front.
        target -= n.war_exhaustion * 45.0;
        // ...and so does the government's own shape. A four-party coalition
        // stretched across the ideological plane holds a lower ceiling than a
        // single-party majority, and a regime that has bought its army and its
        // security service holds a higher one than its record earns. This is
        // where the two halves of `government.rs` reach the budget.
        target += composition;
        let target = target.clamp(0.0, 100.0);
        // Standing is slow to build and quicker to lose.
        let rate = if target < n.political_capital { 0.055 } else { 0.028 };
        n.political_capital += (target - n.political_capital) * rate;
        n.political_capital = n.political_capital.clamp(0.0, 100.0);
    }
}

pub fn tick(w: &mut WorldState) {
    // Who holds office is settled before what their standing is worth: an
    // election held this month, or a coup, has to be reflected in the capital
    // the government wakes up holding.
    crate::government::tick(w);
    political_capital(w);
    let ids: Vec<NationId> = w.nations.iter().filter(|n| n.alive).map(|n| n.id).collect();

    // ---- Proliferation: India & Pakistan test in 1998 (if alive and unconquered) ----
    if w.year == 1998 && w.month == 5 && !w.has_flag("sasia_nuclear") {
        w.set_flag("sasia_nuclear");
        for id in [NationId::India, NationId::Pakistan] {
            if w.nation(id).alive {
                w.nation_mut(id).nuclear = true;
                w.headline(format!("{} conducts nuclear tests. The world condemns; deterrence descends on the subcontinent.", id.name()));
            }
        }
    }

    // ---- Central banks (AI-controlled nations only) ----
    for id in &ids {
        if Some(*id) == w.player {
            continue;
        }
        let n = w.nation_mut(*id);
        // Taylor-lite: respond to inflation above 2-3% band
        let target = 0.025;
        let desired = (0.025 + n.inflation + (n.inflation - target) * 0.6).clamp(0.0, 0.45);
        n.interest_rate += (desired - n.interest_rate) * 0.15;
    }

    // ---- Fiscal AI: consolidate when debt runs hot ----
    for id in &ids {
        if Some(*id) == w.player {
            continue;
        }
        let n = w.nation_mut(*id);
        if n.debt_gdp > 0.85 {
            n.tax_rate = (n.tax_rate + 0.002).min(0.55);
            n.mil_spend_gdp = (n.mil_spend_gdp * 0.995).max(0.01);
            n.state_invest_gdp = (n.state_invest_gdp * 0.995).max(0.02);
        } else if n.debt_gdp < 0.3 && n.tax_rate > 0.30 {
            n.tax_rate -= 0.001;
        }
    }

    // Elections used to live here: every four years in November, a democracy
    // gained three stability, or eight if times were bad. That is the party
    // popularity slider BIBLE section 4 says this game replaces, and it is now
    // `government.rs` — real parties, support that moves with what the economy
    // did to people, a result that has to be governed with, and a coalition
    // that costs political capital to hold.

    // ---- Regime collapse & USSR dissolution ----
    for id in ids.clone() {
        let (stab, sep, is_ussr, alive) = {
            let n = w.nation(id);
            (n.stability, n.separatism, id == NationId::USSR, n.alive)
        };
        if !alive {
            continue;
        }
        let is_yugo = id == NationId::Yugoslavia;
        if is_ussr && (stab < 25.0 || sep > 0.9) && !w.has_flag("ussr_dissolved") {
            dissolve_ussr(w);
        } else if is_yugo && (stab < 25.0 || sep > 0.9) && !w.has_flag("yugoslavia_dissolved") {
            dissolve_yugoslavia(w);
        } else if !is_ussr && !is_yugo && stab < 12.0 && w.rng.chance(0.10 * w.rules.crisis_intensity) {
            // Generic regime collapse: chaos, then a new regime
            let auth_shift = w.rng.range(-0.3, 0.2);
            let n = w.nation_mut(id);
            n.stability = 45.0;
            n.gdp *= 0.93;
            n.authoritarianism = (n.authoritarianism + auth_shift).clamp(0.05, 0.95);
            w.headline(format!("Revolution in {} — the old regime falls.", id.name()));
        }
    }

    // ---- AI statecraft, then AI wars: a guarantee signed this month is a
    // guarantee the aggressor has to price in this month. ----
    ai_statecraft(w);
    ai_wars(w);

    // ---- Grievances fade; alliances are institutional and do not ----
    let belligerents: Vec<(NationId, NationId)> =
        w.wars.iter().map(|war| (war.attacker, war.defender)).collect();
    for (a, b, v) in w.relations.pairs_mut() {
        if *v >= 0.0 {
            continue;
        }
        if belligerents.iter().any(|(x, y)| (*x == a && *y == b) || (*x == b && *y == a)) {
            continue; // an active war keeps the wound open
        }
        *v -= *v * 0.008; // ~9%/yr toward indifference
    }

    // ---- Sanctions relief: an embargo outlasts the war that caused it, and
    // lifts only once the grievance behind it has cooled. ----
    let decisions: Vec<((NationId, NationId), bool)> = w
        .sanctions
        .iter()
        .map(|&(imposer, target)| {
            let target_alive = w.nations.iter().any(|n| n.id == target && n.alive);
            // Grievance decay sets the clock: a minor partner's embargo fades in
            // ~5 years, a principal antagonist's holds for a decade.
            let keep = target_alive && (w.at_war(target) || w.relation(imposer, target) < -15.0);
            ((imposer, target), keep)
        })
        .collect();
    let mut kept: Vec<(NationId, NationId)> = vec![];
    let mut lifted: Vec<(NationId, NationId)> = vec![];
    for (pair, keep) in decisions {
        if keep {
            kept.push(pair);
        } else {
            lifted.push(pair);
        }
    }
    w.sanctions = kept;
    // Headline only when a target comes fully in from the cold.
    let freed: Vec<NationId> = lifted
        .iter()
        .map(|(_, t)| *t)
        .filter(|t| w.nations.iter().any(|n| n.id == *t && n.alive))
        .filter(|t| w.sanctioned_by_count(*t) == 0)
        .collect();
    for t in freed {
        if !w.headlines.iter().any(|h| h.contains(&format!("Sanctions on {}", t.name()))) {
            w.headline(format!("Sanctions on {} are lifted.", t.name()));
        }
    }
}

fn dissolve_ussr(w: &mut WorldState) {
    w.set_flag("ussr_dissolved");
    let (gdp, pop, oil, strength, inherited_tech) = {
        let u = w.nation(NationId::USSR);
        (u.gdp, u.population, u.oil_mbd, u.mil_strength, u.tech.clone())
    };
    {
        let u = w.nation_mut(NationId::USSR);
        u.alive = false;
        u.gdp = 0.0;
    }
    // Russia inherits roughly 60% of the economy, half the people, most oil, the arsenal.
    // The rest of the union is abstracted away except Ukraine, which is too big
    // to fold into the scenery: 52m of the USSR's 289m people, and the second
    // economy of the union.
    let russia = Nation {
        id: NationId::Russia,
        alive: true,
        system: EconomySystem::Market, // shock-therapy transition
        authoritarianism: 0.45,
        gdp: gdp * 0.55,
        population: pop * 0.51,
        tfp_trend: 0.008,
        inflation: 0.90, // transition price liberalization
        interest_rate: 0.20,
        tax_rate: 0.28,
        mil_spend_gdp: 0.045,
        state_invest_gdp: 0.04,
        priv_invest_gdp: 0.10,
        debt_gdp: 0.35,
        oil_mbd: oil * 0.85,
        bubble: 0.0,
        growth_last: -0.05,
        stability: 38.0,
        separatism: 0.20,
        mil_strength: strength * 0.65,
        war_exhaustion: 0.0,
        nuclear: true,
        // A successor government starts on what its own condition earns it and
        // no record of its own to trade on. In Moscow in 1991 that is very
        // little: the shops are empty and the prices are about to be freed.
        political_capital: seated_political_capital(38.0, 0.90, 0.45),
        // The institutes and the engineers do not vanish with the flag over the
        // Kremlin; the research programmes they were working to do.
        tech: crate::tech::TechState::inherit(&inherited_tech, 0.008),
    };
    w.nations.push(russia);

    // Ukraine: about a fifth of Soviet output — 17% of net material product but a
    // heavier share than that of the heavy industry, since the Donbas coalfield,
    // the Krivoy Rog ore, the Zaporizhzhia steel and the Yuzhmash missile plant
    // were all inside it. That inheritance is a liability in a transition, not an
    // asset: the plants it got were the ones with no market, and it starts with
    // worse inflation and a shallower reform than Russia. Almost none of the oil
    // was Ukrainian. Debt is low because of the 1994 "zero option" — Moscow took
    // all Soviet foreign debt and, with it, all Soviet foreign assets.
    let ukraine = Nation {
        id: NationId::Ukraine,
        alive: true,
        system: EconomySystem::Market,
        authoritarianism: 0.40,
        gdp: gdp * 0.19,
        population: pop * 0.18,
        tfp_trend: 0.002,
        inflation: 1.10,
        interest_rate: 0.18,
        tax_rate: 0.30,
        mil_spend_gdp: 0.035,
        state_invest_gdp: 0.04,
        priv_invest_gdp: 0.08,
        debt_gdp: 0.15,
        oil_mbd: oil * 0.01,
        bubble: 0.0,
        growth_last: -0.06,
        stability: 34.0,
        separatism: 0.35, // Crimea and the Donbas, from the day the flag went up
        mil_strength: strength * 0.15,
        war_exhaustion: 0.0,
        // Ukraine woke up with the third-largest nuclear arsenal on earth — some
        // 1,900 strategic warheads left on its territory — and gave every one of
        // them back. The Budapest Memorandum of December 1994 traded them for
        // security assurances from Russia, the United States and Britain. It is
        // the only nuclear disarmament of its size ever carried out, and it is
        // why Ukraine enters the sim without the deterrent Russia keeps.
        nuclear: false,
        political_capital: seated_political_capital(34.0, 1.10, 0.40),
        // Yuzhmash and Antonov were Soviet design bureaux before they were
        // Ukrainian ones: the knowledge is inherited in full, and it is the
        // economy underneath it, not the engineers, that fails to keep up.
        tech: crate::tech::TechState::inherit(&inherited_tech, 0.002),
    };
    w.nations.push(ukraine);

    // The other ten republics that are large enough, or awkward enough, to
    // matter on their own. Shares are of the union's 1990 totals: output by
    // republican share of net material product (Goskomstat SSSR, Narodnoye
    // khozyaystvo SSSR v 1990 g.), people by the January 1990 estimates rolled
    // forward from the 1989 census, oil by republican crude output in 1990, and
    // the army by where the Soviet Armed Forces actually were when the flag came
    // down. Russia and Ukraine keep the shares they already had; these come out
    // of the quarter of the union that was previously abstracted away, and
    // roughly five per cent of it still is - Turkmenistan, Tajikistan and
    // Kyrgyzstan are not modelled here.
    //
    // There is no honest dollar GDP for any of these republics in 1990. The
    // official rouble rate was fiction, the World Bank's early series for the
    // region were built backwards from 1992 dollar figures during a collapse,
    // and the CIA and Goskomstat disagree by a factor of two on the union
    // itself. Republican shares of a union aggregate are the defensible thing
    // to transcribe, and that is what these are.
    struct Republic {
        id: NationId,
        gdp: f64,
        pop: f64,
        army: f64,
        oil: f64,
        sep: f64,
        auth: f64,
        stab: f64,
        tfp: f64,
        infl: f64,
        rate: f64,
        debt: f64,
        milspend: f64,
    }
    let republics = [
        // Belarus - 4.2% of net material product and 10.2m of the union's
        // people. The most militarised ground in Europe per head: the
        // Belorussian Military District held three combined-arms armies and
        // something over 2,000 tanks, and 81 SS-25 mobile ICBMs stood at Lida
        // and Mozyr. It gave every warhead back under the Lisbon Protocol of
        // May 1992, so it enters with no deterrent. Separatism is near zero:
        // the 1989 census counts 77.9% Belarusians with no territorial
        // minority anywhere, and the Popular Front never took the country.
        Republic { id: NationId::Belarus, gdp: 0.042, pop: 0.036, army: 0.045, oil: 0.003,
                   sep: 0.05, auth: 0.40, stab: 45.0, tfp: 0.004, infl: 1.20, rate: 0.22,
                   debt: 0.08, milspend: 0.030 },
        // Kazakhstan - 4.3% of output, 16.7m people, and 25.8 million tonnes of
        // crude in 1990, about 0.53 mbd, which is most of the union's oil
        // outside Russia. It also inherited 1,410 strategic warheads, 104 SS-18
        // silos and the Semipalatinsk test site, and returned all of it by
        // 1995. Separatism is high and it is demographic: the 1989 census gives
        // 39.7% Kazakhs against 37.8% Russians, and the Russians are the
        // northern oblasts along the border.
        Republic { id: NationId::Kazakhstan, gdp: 0.043, pop: 0.058, army: 0.025, oil: 0.046,
                   sep: 0.30, auth: 0.62, stab: 42.0, tfp: 0.003, infl: 1.30, rate: 0.22,
                   debt: 0.08, milspend: 0.028 },
        // Uzbekistan - the most populous of the ten at 20.5m, but only 3.3% of
        // output: a cotton monoculture the union bought at administered prices,
        // and the Aral Sea spent to grow it. Karimov's apparatus survived 1991
        // without a break, which is why it starts the most authoritarian state
        // in the region and the most stable of the poor ones.
        Republic { id: NationId::Uzbekistan, gdp: 0.033, pop: 0.071, army: 0.018, oil: 0.005,
                   sep: 0.15, auth: 0.78, stab: 46.0, tfp: 0.002, infl: 1.00, rate: 0.20,
                   debt: 0.10, milspend: 0.025 },
        // Georgia - 1.6% of output and 5.5m people, and the worst opening
        // position of any Soviet successor. Separatism at 0.75 is not an
        // opinion: Abkhazia and South Ossetia were autonomous units with their
        // own institutions and both fought secession wars between 1991 and
        // 1993, while Tbilisi's own government was shelled out of office by its
        // National Guard in December 1991. Georgian output fell by roughly
        // three quarters between 1990 and 1994, the steepest peacetime collapse
        // recorded anywhere in the period, which is what the negative TFP trend
        // and the stability of 20 are carrying.
        Republic { id: NationId::Georgia, gdp: 0.016, pop: 0.019, army: 0.008, oil: 0.000,
                   sep: 0.75, auth: 0.45, stab: 20.0, tfp: -0.005, infl: 1.60, rate: 0.28,
                   debt: 0.12, milspend: 0.045 },
        // Armenia - 0.9% of output and 3.3m people, still rebuilding from the
        // Spitak earthquake of December 1988 when the union ended. Internally
        // it is the most homogeneous republic in the union at 93.3% Armenian in
        // the 1989 census, so separatism is low; the strain is external, and it
        // is Karabakh. Azerbaijan and Turkey both closed their borders, leaving
        // one road through Georgia and one through Iran.
        Republic { id: NationId::Armenia, gdp: 0.009, pop: 0.0114, army: 0.008, oil: 0.000,
                   sep: 0.12, auth: 0.45, stab: 27.0, tfp: -0.002, infl: 1.50, rate: 0.28,
                   debt: 0.12, milspend: 0.060 },
        // Azerbaijan - 1.7% of output, 7.2m people, and 12.5 million tonnes of
        // crude in 1990, about 0.25 mbd from fields that had been the world's
        // largest in 1900 and were badly depleted by 1990. Separatism at 0.55 is
        // Nagorno-Karabakh, 76.9% Armenian in the 1989 census, whose soviet had
        // voted to leave for Armenia in February 1988. Two governments fell over
        // it in three years.
        Republic { id: NationId::Azerbaijan, gdp: 0.017, pop: 0.025, army: 0.010, oil: 0.022,
                   sep: 0.55, auth: 0.55, stab: 24.0, tfp: 0.000, infl: 1.40, rate: 0.26,
                   debt: 0.05, milspend: 0.060 },
        // Lithuania - 1.4% of output and 3.7m people, and the republic that
        // went first: the Supreme Council declared the restoration of
        // independence on 11 March 1990, took an economic blockade for it in
        // April, and buried fourteen people at the Vilnius television tower in
        // January 1991. Separatism is the lowest of the three Baltics because
        // the country is the most homogeneous of them, 79.6% Lithuanian, and
        // it was the only one to grant citizenship to all residents.
        Republic { id: NationId::Lithuania, gdp: 0.014, pop: 0.0128, army: 0.002, oil: 0.000,
                   sep: 0.10, auth: 0.20, stab: 50.0, tfp: 0.012, infl: 0.80, rate: 0.30,
                   debt: 0.05, milspend: 0.020 },
        // Latvia - 1.1% of output and 2.7m people, and the sharpest
        // demographic problem in the union outside Kazakhstan: 52.0% Latvian
        // against 34.0% Russian in the 1989 census, with Riga itself under half
        // Latvian. The citizenship law of 1994 left roughly a quarter of the
        // residents stateless. Almost no army: the Soviet garrison left in
        // August 1994 and took its equipment with it.
        Republic { id: NationId::Latvia, gdp: 0.011, pop: 0.0093, army: 0.002, oil: 0.000,
                   sep: 0.18, auth: 0.20, stab: 50.0, tfp: 0.012, infl: 0.80, rate: 0.30,
                   debt: 0.04, milspend: 0.020 },
        // Estonia - the smallest of the ten at 1.6m people and 0.7% of output,
        // and the fastest reformer of any successor: its own currency in June
        // 1992 on a currency board against the D-Mark, a flat income tax in
        // 1994, and unilateral free trade. Separatism is the Russophone
        // north-east, 30.3% of the country in 1989, which held autonomy
        // referendums in Narva and Sillamae in July 1993.
        Republic { id: NationId::Estonia, gdp: 0.007, pop: 0.0055, army: 0.001, oil: 0.000,
                   sep: 0.20, auth: 0.18, stab: 53.0, tfp: 0.015, infl: 0.75, rate: 0.30,
                   debt: 0.03, milspend: 0.020 },
        // Moldova - 1.2% of output and 4.4m people, and a border dispute with
        // itself. Transnistria declared on 2 September 1990 and Gagauzia in
        // August 1990; the war of 1992 was decided by the Soviet 14th Army,
        // which was already stationed on the left bank and did not leave.
        // Separatism at 0.45 is a state that lost a tenth of its territory and
        // most of its power generation in its second year.
        Republic { id: NationId::Moldova, gdp: 0.012, pop: 0.015, army: 0.003, oil: 0.000,
                   sep: 0.45, auth: 0.40, stab: 28.0, tfp: 0.000, infl: 1.40, rate: 0.26,
                   debt: 0.07, milspend: 0.035 },
    ];
    for r in &republics {
        w.nations.push(Nation {
            id: r.id,
            alive: true,
            system: EconomySystem::Market,
            authoritarianism: r.auth,
            gdp: gdp * r.gdp,
            population: pop * r.pop,
            tfp_trend: r.tfp,
            inflation: r.infl,
            interest_rate: r.rate,
            tax_rate: 0.30,
            mil_spend_gdp: r.milspend,
            state_invest_gdp: 0.04,
            priv_invest_gdp: 0.09,
            // The "zero option" of 1994 gave Moscow every rouble of Soviet
            // foreign debt and, with it, every Soviet foreign asset. The other
            // republics therefore start almost unencumbered, and what debt they
            // carry here is domestic and inherited from the enterprises.
            debt_gdp: r.debt,
            oil_mbd: oil * r.oil,
            bubble: 0.0,
            growth_last: -0.08,
            stability: r.stab,
            separatism: r.sep,
            mil_strength: strength * r.army,
            war_exhaustion: 0.0,
            // Belarus and Kazakhstan both woke up holding strategic warheads
            // and both gave them back, Belarus by 1996 and Kazakhstan by 1995,
            // under the Lisbon Protocol of May 1992 and the Budapest assurances
            // of December 1994. Nobody in this group enters with a deterrent.
            nuclear: false,
            political_capital: seated_political_capital(r.stab, r.infl, r.auth),
            tech: crate::tech::TechState::inherit(&inherited_tech, r.tfp),
        });
    }

    // Every successor inherits a thawed version of USSR relations
    let rels: Vec<(NationId, f64)> = ALL_START_NATIONS
        .iter()
        .filter(|x| **x != NationId::USSR)
        .map(|x| (*x, w.relation(NationId::USSR, *x) * 0.5 + 10.0))
        .collect();
    let successors: Vec<NationId> = [NationId::Russia, NationId::Ukraine]
        .into_iter()
        .chain(republics.iter().map(|r| r.id))
        .collect();
    for successor in successors.iter().copied() {
        for (other, v) in &rels {
            w.set_relation(successor, *other, *v);
        }
    }
    // How the twelve regard each other on the morning after. The pattern is not
    // distance from Moscow but whether the republic needs Moscow: Minsk and
    // Yerevan do and say so, Almaty hedges, Tbilisi and Chisinau have Russian
    // soldiers on ground they claim, and the Baltics spent the next three years
    // negotiating a withdrawal.
    let among: &[(NationId, NationId, f64)] = &[
        (NationId::Russia, NationId::Belarus, 45.0),
        (NationId::Russia, NationId::Kazakhstan, 35.0),
        (NationId::Russia, NationId::Armenia, 40.0),
        (NationId::Russia, NationId::Uzbekistan, 15.0),
        (NationId::Russia, NationId::Azerbaijan, 0.0),
        (NationId::Russia, NationId::Georgia, -10.0),
        (NationId::Russia, NationId::Moldova, -15.0),
        (NationId::Russia, NationId::Lithuania, -20.0),
        (NationId::Russia, NationId::Latvia, -25.0),
        (NationId::Russia, NationId::Estonia, -25.0),
        (NationId::Ukraine, NationId::Belarus, 25.0),
        (NationId::Ukraine, NationId::Moldova, 20.0),
        (NationId::Ukraine, NationId::Georgia, 15.0),
        // The Karabakh war is the one open conflict inside the group.
        (NationId::Armenia, NationId::Azerbaijan, -70.0),
        (NationId::Georgia, NationId::Azerbaijan, 15.0),
        (NationId::Georgia, NationId::Armenia, 10.0),
        (NationId::Kazakhstan, NationId::Uzbekistan, 15.0),
        (NationId::Lithuania, NationId::Latvia, 45.0),
        (NationId::Lithuania, NationId::Estonia, 40.0),
        (NationId::Latvia, NationId::Estonia, 45.0),
    ];
    for (a, b, v) in among {
        w.set_relation(*a, *b, *v);
    }
    // Turkey recognised the Turkic republics within days and closed its border
    // with Armenia in 1993 over Karabakh; Iran, against every expectation about
    // Islamic solidarity, backed Christian Armenia, because a strong Azerbaijan
    // is an irredentist claim on the twenty million Azeris inside Iran.
    w.shift_relation(NationId::Turkey, NationId::Azerbaijan, 45.0);
    w.shift_relation(NationId::Turkey, NationId::Uzbekistan, 20.0);
    w.shift_relation(NationId::Turkey, NationId::Kazakhstan, 20.0);
    w.shift_relation(NationId::Turkey, NationId::Armenia, -50.0);
    w.shift_relation(NationId::Iran, NationId::Armenia, 25.0);
    w.shift_relation(NationId::Iran, NationId::Azerbaijan, -15.0);
    // Warsaw and Vilnius over the Polish minority around Vilnius, and the
    // treaty of 1994 that settled it.
    w.shift_relation(NationId::Poland, NationId::Lithuania, 20.0);
    // Kyiv and Moscow start as quarrelling relatives rather than enemies: the
    // Black Sea Fleet and Crimea are already in dispute, but the divorce of
    // December 1991 was signed, not fought.
    w.set_relation(NationId::Russia, NationId::Ukraine, 15.0);

    w.headline("THE SOVIET UNION HAS DISSOLVED. Twelve republics take up their own seats.".into());
    w.headline("Russia inherits the arsenal; Ukraine's warheads go back east under the Budapest assurances.".into());
}

/// Yugoslavia comes apart into republics of unequal wealth and — the part that
/// decides everything — unequal ethnic homogeneity. Each successor inherits a
/// separatism value drawn from its real 1991 census: Slovenia is nearly all
/// Slovene and leaves almost intact, while Bosnia inherits a state with no
/// majority at all. Nothing here schedules a war; the strain is simply handed to
/// the successors, and the existing war machinery does what it does with it.
fn dissolve_yugoslavia(w: &mut WorldState) {
    w.set_flag("yugoslavia_dissolved");
    let (gdp, pop, oil, strength, infl, debt, inherited_tech) = {
        let y = w.nation(NationId::Yugoslavia);
        (y.gdp, y.population, y.oil_mbd, y.mil_strength, y.inflation, y.debt_gdp, y.tech.clone())
    };
    {
        let y = w.nation_mut(NationId::Yugoslavia);
        y.alive = false;
        y.gdp = 0.0;
    }

    // (id, GDP share, pop share, JNA share, separatism, authoritarianism, stability, tfp)
    // Macedonia's ~5% of output and ~9% of the people leave with it, unsimulated:
    // it seceded without a shot and never fought anyone.
    let parts: [(NationId, f64, f64, f64, f64, f64, f64, f64); 4] = [
        // Belgrade keeps the army, and Kosovo and Vojvodina keep Belgrade busy.
        (NationId::Serbia,   0.36, 0.42, 0.70, 0.45, 0.75, 40.0, 0.002),
        // A twelve percent Serb minority concentrated in the Krajina.
        (NationId::Croatia,  0.25, 0.20, 0.12, 0.35, 0.45, 45.0, 0.012),
        // ~88% Slovene, no minority worth a war, and the richest republic.
        (NationId::Slovenia, 0.20, 0.085, 0.08, 0.05, 0.25, 62.0, 0.020),
        // 44% Bosniak, 31% Serb, 17% Croat — a republic that is all minorities.
        (NationId::Bosnia,   0.13, 0.19, 0.05, 0.85, 0.40, 30.0, 0.006),
    ];
    for (id, g, p, m, sep, auth, stab, tfp) in parts {
        w.nations.push(Nation {
            id,
            alive: true,
            system: EconomySystem::Market, // the plan dies with the federation
            authoritarianism: auth,
            gdp: gdp * g,
            population: pop * p,
            tfp_trend: tfp,
            inflation: infl,
            interest_rate: 0.25,
            tax_rate: 0.33,
            mil_spend_gdp: 0.05,
            state_invest_gdp: 0.05,
            priv_invest_gdp: 0.10,
            debt_gdp: debt,
            oil_mbd: oil * g,
            bubble: 0.0,
            growth_last: -0.06,
            stability: stab,
            separatism: sep,
            mil_strength: strength * m,
            war_exhaustion: 0.0,
            nuclear: false,
            political_capital: seated_political_capital(stab, infl, auth),
            // Each republic keeps the federation's technical base and starts its
            // own research from nothing.
            tech: crate::tech::TechState::inherit(&inherited_tech, tfp),
        });
    }

    // Successors inherit the federation's standing abroad, thinned out.
    let inherited: Vec<(NationId, f64)> = ALL_START_NATIONS
        .iter()
        .filter(|x| **x != NationId::Yugoslavia)
        .map(|x| (*x, w.relation(NationId::Yugoslavia, *x) * 0.6))
        .collect();
    for (id, _, _, _, _, _, _, _) in parts {
        for (other, v) in &inherited {
            w.set_relation(id, *other, *v);
        }
    }
    // Bonn recognised Slovenia and Croatia early and over everyone's objections.
    w.shift_relation(NationId::Germany, NationId::Slovenia, 25.0);
    w.shift_relation(NationId::Germany, NationId::Croatia, 25.0);

    // How the republics regard each other on day one. Belgrade's claim on the
    // Serb minorities abroad is the fault line; Ljubljana has no such quarrel.
    let between: &[(NationId, NationId, f64)] = &[
        (NationId::Serbia, NationId::Croatia, -45.0),
        (NationId::Serbia, NationId::Bosnia, -35.0),
        (NationId::Serbia, NationId::Slovenia, -20.0),
        (NationId::Croatia, NationId::Bosnia, -10.0),
        (NationId::Croatia, NationId::Slovenia, 20.0),
        (NationId::Bosnia, NationId::Slovenia, 5.0),
    ];
    for (a, b, v) in between {
        w.set_relation(*a, *b, *v);
    }

    w.headline("YUGOSLAVIA HAS DISSOLVED. Slovenia, Croatia, Bosnia and Serbia stand alone.".into());
    w.headline("The JNA's divisions, and its arsenal, remain in Belgrade's hands.".into());
}

/// The powers that keep clients. Not simply the strongest — the ones with a bloc
/// to hold together and something to lose if it goes over to the other side.
/// Japan belongs here despite its two-decimal army: it passed the United States
/// in 1989 to become the largest aid donor in the world and stayed there until
/// 2000. https://ies.princeton.edu/pdf/E196.pdf
pub const PATRONS: [NationId; 8] = [
    NationId::USA, NationId::USSR, NationId::Russia, NationId::China,
    NationId::UK, NationId::France, NationId::Germany, NationId::Japan,
];

/// What a great power does in the eleven months of the year it is not invading
/// anyone. Each patron gets a handful of independent low-probability chances per
/// month to open a chequebook, sign a guarantee, open a market, or pay somebody
/// to make a rival's client ungovernable. Everything goes through the same
/// `Command` the player uses, so the AI cannot do anything a player could not.
fn ai_statecraft(w: &mut WorldState) {
    let patrons: Vec<NationId> = PATRONS
        .iter()
        .copied()
        .filter(|p| w.nation_opt(*p).map_or(false, |n| n.alive) && Some(*p) != w.player)
        .collect();

    for p in patrons {
        // A patron with its own house on fire stops buying friends.
        if w.nation(p).stability < 25.0 {
            continue;
        }

        // ---- Patronage. The scoring is where the competition lives. ----
        let headroom = crate::statecraft::MAX_AID_SHARE - w.aid_share_committed(p);
        if headroom > 0.0008 && w.rng.chance(0.10) {
            if let Some(c) = best_client(w, p) {
                // Guns for a client with an enemy, money for one with a problem.
                let threatened = w.at_war(c)
                    || w.nations.iter().any(|n| {
                        n.alive && n.id != c && n.id != p && w.relation(c, n.id) < -30.0
                    });
                let kind = if threatened { AidKind::Arms } else { AidKind::Economic };
                let current = w.aid_flow(p, c, kind).map(|f| f.share_gdp).unwrap_or(0.0);
                let room = headroom
                    .min(crate::statecraft::MAX_CLIENT_SHARE - w.aid_share_to(p, c))
                    .max(0.0);
                let share = current + 0.002_f64.min(room);
                let _ = crate::apply_command(
                    w,
                    &crate::Command::PledgeAid { patron: p, client: c, kind, share_gdp: share },
                );
            }
        }

        // ---- Cut a client loose once it has drifted out of the sphere. ----
        let lapsed: Vec<(NationId, AidKind)> = w
            .statecraft
            .aid
            .iter()
            .filter(|f| f.patron == p && w.relation(p, f.client) < -10.0)
            .map(|f| (f.client, f.kind))
            .collect();
        for (c, kind) in lapsed {
            let _ = crate::apply_command(w, &crate::Command::EndAid { patron: p, client: c, kind });
        }

        // ---- A guarantee, but only for a client you are already paying for and
        // cannot afford to lose. Handing them out cheaply is how a great power
        // ends up in a war over a country it could not find on a map. ----
        if w.rng.chance(0.05) {
            let candidate = w
                .statecraft
                .aid
                .iter()
                .filter(|f| f.patron == p)
                .map(|f| f.client)
                .filter(|c| !w.allied(p, *c) && w.relation(p, *c) >= 60.0)
                .max_by(|a, b| {
                    w.relation(p, *a)
                        .partial_cmp(&w.relation(p, *b))
                        .unwrap_or(std::cmp::Ordering::Equal)
                });
            if let Some(c) = candidate {
                let _ = crate::apply_command(w, &crate::Command::ProposeAlliance { from: p, to: c });
            }
        }

        // ---- Subversion, aimed at whoever is both hostile and brittle. ----
        if w.rng.chance(0.022) {
            if let Some(t) = best_covert_target(w, p) {
                let (stab, sep) = {
                    let n = w.nation(t);
                    (n.stability, n.separatism)
                };
                let op = if sep > 0.25 {
                    CovertOp::StirSeparatists
                } else if stab < 50.0 {
                    CovertOp::FundOpposition
                } else {
                    CovertOp::SabotageIndustry
                };
                let _ = crate::apply_command(
                    w,
                    &crate::Command::CovertAction { sponsor: p, target: t, op },
                );
            }
        }
    }

    // ---- Trade is not a great-power monopoly: anybody on decent terms with a
    // bigger market will try to get into it. ----
    let traders: Vec<NationId> = w
        .nations
        .iter()
        .filter(|n| n.alive)
        .map(|n| n.id)
        .filter(|id| Some(*id) != w.player)
        .collect();
    for a in traders {
        if w.statecraft.trade.iter().filter(|t| t.a == a || t.b == a).count() >= 4 {
            continue;
        }
        if !w.rng.chance(0.015) {
            continue;
        }
        let partner = w
            .nations
            .iter()
            .filter(|n| n.alive && n.id != a)
            .map(|n| (n.id, n.gdp))
            .filter(|(b, _)| {
                w.relation(a, *b) >= 40.0
                    && w.trade_depth(a, *b) <= 0.0
                    && !w.is_sanctioning(a, *b)
                    && !w.is_sanctioning(*b, a)
                    && !crate::statecraft::belligerents(w, a, *b)
            })
            .max_by(|x, y| x.1.partial_cmp(&y.1).unwrap_or(std::cmp::Ordering::Equal))
            .map(|(b, _)| b);
        if let Some(b) = partner {
            let _ = crate::apply_command(w, &crate::Command::ProposeTrade { from: a, to: b });
        }
    }

    // ---- The other side of the market for guarantees. A small state with a
    // stronger enemy goes looking for a protector, and looks first at whoever is
    // already paying its bills — which is how patronage turns into commitment. ----
    let seekers: Vec<NationId> = w
        .nations
        .iter()
        .filter(|n| n.alive)
        .map(|n| n.id)
        .filter(|id| Some(*id) != w.player && !PATRONS.contains(id))
        .collect();
    for s in seekers {
        let strength = w.nation(s).mil_strength;
        let threatened = w.nations.iter().any(|n| {
            n.alive && n.id != s && n.mil_strength > strength * 1.3 && w.relation(s, n.id) < -30.0
        });
        if !threatened || !w.rng.chance(0.07) {
            continue;
        }
        let protector = w
            .nations
            .iter()
            .filter(|n| n.alive && n.id != s && !w.allied(s, n.id))
            .filter(|n| {
                let rel = w.relation(s, n.id);
                // Nobody underwrites a country they have no stake in. Either the
                // friendship is deep already, or the cheques have made it so.
                rel >= 60.0 || (rel >= 45.0 && w.aid_share_to(n.id, s) > 0.0)
            })
            .max_by(|x, y| {
                x.mil_strength
                    .partial_cmp(&y.mil_strength)
                    .unwrap_or(std::cmp::Ordering::Equal)
            })
            .map(|n| n.id);
        if let Some(p) = protector {
            let _ = crate::apply_command(w, &crate::Command::ProposeAlliance { from: s, to: p });
        }
    }
}

/// What a client is worth to a patron. The decisive term is the last one: a
/// government your rival is already buying is worth *more* to you, not less.
/// That single inversion is what turns a list of aid budgets into a Cold War.
fn client_score(w: &WorldState, patron: NationId, client: NationId) -> f64 {
    let rel = w.relation(patron, client);
    if rel < -25.0 {
        return 0.0;
    }
    if w.aid_share_to(patron, client) >= crate::statecraft::MAX_CLIENT_SHARE - 1e-9 {
        return 0.0; // this one is already taking all it can be given
    }
    let c = w.nation(client);
    // A country rich enough to fund its own government is not for sale, whatever
    // else it can be talked into.
    if c.gdp * 1000.0 / c.population > 15000.0 {
        return 0.0;
    }
    // Oil, an economy worth having on your side of the ledger, and an army that
    // can hold a line you would rather not send your own soldiers to.
    let value = c.oil_mbd * 1.4 + (c.gdp / 150.0).min(5.0) + c.mil_strength / 12.0;
    // Squared, because patrons do not shop on price alone: who a government
    // already leans toward dominates what it is objectively worth.
    let affinity = ((rel + 40.0) / 140.0).clamp(0.05, 1.0).powi(2);
    let backers = w.patrons_of(client);
    let contested = backers
        .iter()
        .any(|q| *q != patron && w.relation(patron, *q) < -20.0);
    let already_mine = backers.contains(&patron);
    value
        * affinity
        * if contested { 2.2 } else { 1.0 }
        * if already_mine { 0.60 } else { 1.0 }
}

fn best_client(w: &WorldState, patron: NationId) -> Option<NationId> {
    w.nations
        .iter()
        .filter(|n| n.alive && n.id != patron)
        .map(|n| n.id)
        .filter(|c| !PATRONS.contains(c) && !war::MAJORS.contains(c))
        .filter(|c| !crate::statecraft::belligerents(w, patron, *c))
        .map(|c| (c, client_score(w, patron, c)))
        .filter(|(_, s)| *s > 0.0)
        .max_by(|x, y| x.1.partial_cmp(&y.1).unwrap_or(std::cmp::Ordering::Equal))
        .map(|(c, _)| c)
}

/// Subversion goes where hostility meets brittleness. A rival's client is the
/// classic target: cheaper to break than the rival, and it hurts the rival anyway.
fn best_covert_target(w: &WorldState, sponsor: NationId) -> Option<NationId> {
    w.nations
        .iter()
        .filter(|n| n.alive && n.id != sponsor)
        .map(|n| n.id)
        .filter(|t| w.relation(sponsor, *t) <= -30.0 && !w.allied(sponsor, *t))
        .map(|t| {
            let n = w.nation(t);
            let brittle = (70.0 - n.stability).max(0.0) / 70.0 + n.separatism * 0.5;
            // A channel already half-blown is a reason to wait, not to press.
            let caution = 1.0 - w.covert_heat(sponsor, t) * 0.8;
            let proxy = if w.patrons_of(t).iter().any(|q| w.relation(sponsor, *q) < -20.0) {
                1.6
            } else {
                1.0
            };
            (t, brittle * caution * proxy)
        })
        .filter(|(_, s)| *s > 0.05)
        .max_by(|x, y| x.1.partial_cmp(&y.1).unwrap_or(std::cmp::Ordering::Equal))
        .map(|(t, _)| t)
}

fn ai_wars(w: &mut WorldState) {
    // Iraq's Kuwait calculus: debt-strained oil state eyeing a rich, weak neighbor.
    let candidates: Vec<(NationId, NationId, f64)> = {
        let mut v = vec![];
        let aggressors = [
            NationId::Iraq, NationId::Iran, NationId::Pakistan, NationId::India,
            NationId::Serbia, NationId::Croatia,
            NationId::China, NationId::Israel, NationId::Turkey,
        ];
        // Successor states are not in ALL_START_NATIONS, so consider everyone alive.
        let living: Vec<NationId> = w.nations.iter().filter(|n| n.alive).map(|n| n.id).collect();
        for a in aggressors {
            let an = match w.nation_opt(a) {
                Some(n) => n,
                None => continue, // has not been born yet, if it ever is
            };
            if !an.alive || Some(a) == w.player || w.at_war(a) || an.war_exhaustion > 0.3 {
                continue;
            }
            for t in living.iter().copied() {
                if t == a {
                    continue;
                }
                let tn = match w.nation_opt(t) {
                    Some(n) => n,
                    None => continue,
                };
                if !tn.alive || w.at_war(t) {
                    continue;
                }
                // Only historically-plausible dyads have nonzero base appetite (regional gates)
                let base: f64 = match (a, t) {
                    (NationId::Iraq, NationId::Kuwait) => 0.030,
                    (NationId::Iraq, NationId::SaudiArabia) => 0.004,
                    (NationId::Iraq, NationId::Iran) => 0.002,
                    (NationId::Iran, NationId::Iraq) => 0.002,
                    (NationId::Pakistan, NationId::India) => 0.001,
                    (NationId::India, NationId::Pakistan) => 0.001,
                    // The Sino-Vietnamese quarrel is the live one in Asia in
                    // 1990: a border war in 1979, artillery duels along it until
                    // 1988, and three Vietnamese ships sunk off Johnson Reef in
                    // the Spratlys that March. Relations are not normalised until
                    // November 1991, and Hanoi's Soviet guarantor is dying. Only
                    // this direction: Beijing has the bomb and Hanoi does not.
                    // Kept low because the quarrel was cooling, not heating: the
                    // multipliers below (relations at -45, a strength ratio near
                    // two) already triple whatever base it is given.
                    (NationId::China, NationId::Vietnam) => 0.0003,
                    // Osirak is the precedent and it is only nine years old.
                    // Israel struck Iraq's reactor from the air in 1981 and was
                    // arguing internally about doing it again through 1990, up to
                    // and past the point where Scuds were landing on Tel Aviv.
                    // What it never became was an invasion, which is what this
                    // table declares — hence a base an order below Iraq's own
                    // appetite for Kuwait, further damped because Israel's 132%
                    // debt would otherwise read to the AI as fiscal desperation.
                    (NationId::Israel, NationId::Iraq) => 0.0003,
                    // Turkey put divisions across the Iraqi border repeatedly in
                    // the nineties — 1992, 1995, 1997 — chasing the PKK into a
                    // north Iraq Baghdad no longer controlled, and Ozal pressed
                    // his allies for a share of the outcome, Mosul included. The
                    // appetite is real but conditional: it needs Baghdad to be
                    // beaten first, which the coalition's sanctions and the
                    // relations shift after an invasion supply.
                    (NationId::Turkey, NationId::Iraq) => 0.002,
                    // Wars of succession: neighbours of a state that just came
                    // apart, with kin on the wrong side of the new borders.
                    (NationId::Serbia, NationId::Croatia) => 0.022,
                    (NationId::Serbia, NationId::Bosnia) => 0.030,
                    (NationId::Serbia, NationId::Slovenia) => 0.010,
                    (NationId::Croatia, NationId::Bosnia) => 0.014,
                    _ => 0.0,
                };
                if base <= 0.0 {
                    continue;
                }
                let rel = w.relation(a, t);
                // Expected defense includes likely interveners — but a first-time
                // gambler discounts them (Saddam's 1990 misjudgment). After one
                // repelled invasion the lesson is learned permanently.
                let learned = w.has_flag(&format!("burned_{:?}_{:?}", a, t));
                let mut expected_def = tn.mil_strength;
                let coalition_discount = if learned { 1.0 } else { 0.10 };
                // Read the same list the coalition actually forms from, or the
                // lesson of a repelled invasion is never available to be learned.
                for m in war::MAJORS {
                    if war::would_intervene(w, m, t, a) {
                        expected_def += w.nation(m).mil_strength * coalition_discount;
                    }
                }
                // A signed guarantee is the one commitment an aggressor cannot
                // pretend not to have seen. It is still discounted — pacts are
                // broken — but far less than a hoped-for coalition.
                for g in w.pact_partners(t) {
                    if g == a || war::MAJORS.contains(&g) && war::would_intervene(w, g, t, a) {
                        continue;
                    }
                    if w.nation_opt(g).map_or(true, |n| !n.alive) {
                        continue;
                    }
                    let honoured = if learned { 0.75 } else { 0.20 };
                    expected_def += w.nation(g).mil_strength * honoured;
                }
                let strength_ratio = an.mil_strength / expected_def.max(1.0);
                if strength_ratio < 0.8 {
                    continue; // deterred
                }
                // Fiscal desperation raises appetite (Iraq 1990); bad relations too
                let desperation = (an.debt_gdp - 0.6).max(0.0) * 1.5 + (0.4 - an.growth_last * 10.0).max(0.0) * 0.2;
                // A neighbour that cannot hold itself together is an invitation:
                // its own minorities are a lever, and its army is busy at home.
                let mut p = base
                    * w.rules.ai_aggression
                    * (1.0 + desperation)
                    * (1.0 + tn.separatism * 1.5)
                    * strength_ratio.min(4.0)
                    * if rel < -20.0 { 1.5 } else { 0.5 };
                // Deterrence: never attack a nuclear power without your own arsenal
                if tn.nuclear && !an.nuclear {
                    p = 0.0;
                }
                v.push((a, t, p.min(0.25)));
            }
        }
        v
    };
    for (a, t, p) in candidates {
        if w.at_war(a) {
            continue;
        }
        let roll = w.rng.chance(p);
        if roll {
            let _ = war::declare_war(w, a, t);
        }
    }

    // AI peace offers: badly losing attackers sue for peace (abstract: white peace at high exhaustion handled in war tick)
}
