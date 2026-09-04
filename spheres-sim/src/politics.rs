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
/// WHERE A GOVERNMENT'S STANDING CEILING SITS BEFORE PENSIONS TOUCHES IT, and
/// before the 0..100 clamp.
///
/// Factored out of `political_capital` so the pensions CARD can be told what
/// the arm actually delivers rather than what its slope would deliver on an
/// unbounded scale. The clamp is the whole reason: `pensions_standing` is
/// `gap * 1000.0`, the largest reachable pensions gap is about 0.145, and the
/// ceiling saturates long before that -- so a card quoting the raw slope
/// promises ten points per point of GDP at dial positions that deliver none.
/// `ministries::arms_at` differences `clamp(this + arm)` against `clamp(this)`,
/// which is exactly what the sim charges.
pub fn standing_target(w: &WorldState, id: NationId) -> f64 {
    let composition = crate::government::standing_modifier(w, id);
    let n = w.nation(id);
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
    target
}

fn political_capital(w: &mut WorldState) {
    let loss_rate = crate::clock::blend(w, 0.055);
    let gain_rate = crate::clock::blend(w, 0.028);
    let ids: Vec<NationId> = w.nations.iter().filter(|n| n.alive).map(|n| n.id).collect();
    for id in ids {
        let mut target = standing_target(w, id);
        let n = w.nation_mut(id);
        // PENSIONS' first named arm: a standing cut BLEEDS THE CEILING for as
        // long as it stands.
        //
        // The budget desk already charges a one-off 1.35x on any cut, which is
        // the vote. This is the other half, and it is the half a pension cut
        // actually consists of: a government that has taken money off
        // pensioners does not pay for it once, it pays for it every month the
        // cut is in force, because the constituency it took the money from is
        // the one that turns out. It is symmetric — a standing increase raises
        // the ceiling the same way, which is why every government that could
        // afford one bought one — and it is a CEILING and not a stock, so it
        // moves the government's standing at the slow rate below rather than
        // handing it a lump sum.
        //
        // INVENTED, and labelled as the design requires: the 1000.0 slope,
        // which reads as TEN POINTS OF STANDING CEILING PER POINT OF GDP. The
        // design sizes it at "+0.5% of GDP is about +5 points", and 0.005 *
        // 1000.0 = 5.0 is exactly that. The existing clamp bounds it: pensions
        // caps at 0.20 of GDP against a reference near 0.056, so the largest
        // reachable gap is about 0.144 and the term saturates the 100-point
        // ceiling long before that, while the largest possible cut, -0.056, is
        // -56 points and stays inside the floor.
        target += crate::ministries::pensions_standing(n.budget_gap(BUDGET_PENSIONS));
        let target = target.clamp(0.0, 100.0);
        // Standing is slow to build and quicker to lose.
        let rate = if target < n.political_capital { loss_rate } else { gain_rate };
        n.political_capital += (target - n.political_capital) * rate;
        n.political_capital = n.political_capital.clamp(0.0, 100.0);
    }
}

/// The politics phase: standing, banks, budgets, and the events that end
/// regimes.
///
/// Runs last of the eight entries in `crate::SYSTEMS` — economy, tech,
/// statecraft, stratagems, ai_stratagems, war, government, politics — so that
/// what a government is judged on this month is what the world has just done to
/// it. Prices political capital, runs the AI central banks and fiscal
/// consolidation, then handles proliferation, regime collapse and the two
/// modelled dissolutions.
///
/// Note that `government::tick` runs immediately before this, and did until
/// recently run as its first statement. It is now the entry before this one in
/// `crate::SYSTEMS`, which is the same call in the same place — moved out only
/// so a profiler can price the two separately. The ordering guarantee it was
/// written for still holds: who holds office is settled before what their
/// standing is worth, because an election held this month, or a coup, has to be
/// reflected in the capital the government wakes up holding.
pub fn tick(w: &mut WorldState) {
    let dt = crate::clock::month_fraction(w);
    let bank_rate = crate::clock::blend(w, 0.15);
    let fiscal_cut = crate::clock::decay(w, 0.995);
    let fiscal_restore = crate::clock::decay(w, 1.005);
    let grievance_rate = crate::clock::blend(w, 0.008);
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

    // ---- Central banks (AI-controlled nations, and an unmanned player seat) ----
    for id in &ids {
        // The player's bank is theirs the moment they use it, and not before.
        // Skipping the seat outright meant an idle player kept 1990's rate
        // forever: 8% held into a deflation is a 13% real rate, a permanent
        // -5.8pt demand gap, and a United States that shrinks every month for
        // thirty-five years without one line of it being a decision anyone made.
        // See `WorldState::player_set_rate`.
        if Some(*id) == w.player && w.player_set_rate {
            continue;
        }
        let n = w.nation_mut(*id);
        // Taylor-lite: respond to inflation above the anchor.
        //
        // ONE ANCHOR, READ FROM economy.rs. This was 0.025 while the price
        // equation there anchored at 0.020 — two constants naming one object,
        // the inflation an economy settles on when demand is at potential. The
        // half-point disagreement closed a loop with a fixed point at
        // g* = +0.00108, and every nation on the board collected it as permanent
        // free growth. See `economy::INFLATION_ANCHOR` for the algebra and for
        // why it is this half that moved.
        let target = crate::economy::INFLATION_ANCHOR;
        let desired = (0.025 + n.inflation + (n.inflation - target) * 0.6).clamp(0.0, 0.45);
        n.interest_rate += (desired - n.interest_rate) * bank_rate;
    }

    // ---- Fiscal AI: consolidate when debt runs hot ----
    for id in &ids {
        if Some(*id) == w.player {
            continue;
        }
        let n = w.nation_mut(*id);
        if n.debt_gdp > 0.85 {
            n.tax_rate = (n.tax_rate + 0.002 * dt).min(0.55);
            n.mil_spend_gdp = (n.mil_spend_gdp * fiscal_cut).max(0.01);
            n.state_invest_gdp = (n.state_invest_gdp * fiscal_cut).max(0.02);
        } else if n.debt_gdp < 0.3 {
            if n.tax_rate > 0.30 {
                n.tax_rate -= 0.001 * dt;
            }
            // CONSOLIDATION IS REVERSIBLE, and it was not. The branch above
            // cuts public investment 0.5% a month while debt runs hot and
            // nothing ever gave it back, so a nation that consolidated its way
            // out of a debt crisis carried the cut for the rest of the run. That
            // was a small standing drag while the investment share only bought a
            // growth rate; now that it buys a permanent output LEVEL it is a
            // permanent unearned loss, and the same symmetry argument the
            // capital level payment rests on applies here — what a cut costs, a
            // restoration returns.
            //
            // Mirrors the cut: the same 0.5% a month, against the same debt
            // thresholds already in this rule, and capped at the share the
            // nation actually entered 1990 with so that recovery can never
            // manufacture investment it never had. No new threshold and no new
            // coefficient. `None` — a successor state with no transcribed 1990
            // share — recovers nothing.
            if let Some(base) = n.state_invest_1990 {
                n.state_invest_gdp = (n.state_invest_gdp * fiscal_restore).min(base);
            }
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
        } else if !is_ussr && !is_yugo && stab < 12.0 && monthly_chance(w, 0.10 * w.rules.crisis_intensity) {
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
        w.conflicts.iter().map(|c| (c.attacker(), c.defender())).collect();
    for (a, b, v) in w.relations.pairs_mut() {
        if *v >= 0.0 {
            continue;
        }
        if belligerents.iter().any(|(x, y)| (*x == a && *y == b) || (*x == b && *y == a)) {
            continue; // an active war keeps the wound open
        }
        *v -= *v * grievance_rate; // ~9%/yr toward indifference
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

/// A NOTE ON WHAT THE SUCCESSORS INHERIT, AND WHAT THEY ARE STILL PAID TWICE
/// FOR. `TechState::inherit` now carries the parent's 1990 offset forward, so a
/// republic that takes the union's transcribed 1990 technology does not also
/// collect its own cited trend on top of it. That closes the endowment half.
///
/// It does not close the other half, and the other half is a defect that exists
/// on today's board with nothing granted to anybody. A successor's authored
/// trend — Russia's 0.008, Ukraine's 0.002, `r.tfp` for the rest — goes straight
/// into `tfp_base`, and `apply_bonuses` then adds `(s - reference)` on top of
/// it for the ENTIRE inherited set, including everything the union researched
/// between 1990 and the dissolution. A successor of a parent that out-researched
/// the world therefore opens above its cited figure, and one of a parent that
/// fell behind opens below it.
///
/// Whether that is wrong is a real question rather than an obvious bug: the
/// model pays `(s - reference)` as a differential to every nation, so a
/// successor keeping its parent's earned position is arguably right, and
/// re-anchoring it at birth would throw that position away. It was MEASURED
/// either way and it is left alone here for a procedural reason: rebasing the
/// successors against the live world reference moves `golden_hash_of_a_known_run`
/// on a board where no nation file carries a single grant, which makes it a
/// change to the shipped timeline rather than machinery for one, and it needs
/// its own decision and its own re-pin.
/// WHAT A SUCCESSOR HAS ALREADY BEEN PAID FOR ITS CAPITAL STOCK, and the single
/// largest reason the post-communist bloc grew through the nineties.
///
/// `capital_level_paid: None` is a claim, and it is written out beside
/// `CAPITAL_ELASTICITY` in economy.rs: *the transcribed 1990 figure already
/// reflects the investment share beside it, so this must never reprice a
/// transcribed starting figure.* That claim is true of every nation in
/// `data/nations/` and false of every nation created here. A successor's GDP is
/// not transcribed. It is a SHARE of its parent's GDP, and its parent's GDP is
/// priced at its PARENT's investment share.
///
/// So `None` at these two sites said "the union's plant, valued at the union's
/// 22%-of-output investment programme, is worth exactly the same thing to a
/// republic that will direct 4%" — and it said it in the one direction that
/// matters. Every Soviet successor is born directing far less of its output into
/// investment than the union did: Russia 0.14 of GDP against the union's 0.24,
/// Ukraine 0.12, the other thirteen 0.13. `None` forgave the whole difference,
/// fifteen times over, on the month the flag came down.
///
/// Carrying the parent's marker instead charges it, through machinery that
/// already exists and with no new coefficient: `0.49 * ln(0.14/0.20) = -0.175`
/// against the union's `+0.089`, so Russia owes 0.264 in logs — about 23% of its
/// output level — paid in at the 0.02 a month the capital block already uses,
/// which is a 35-month half-life and most of a decade to complete. Ukraine, on
/// the same arithmetic and a thinner investment share, owes 29%.
///
/// THAT IS THE TRANSITION COLLAPSE, AND IT IS NOT A NEW MECHANISM. It is the
/// same statement economy.rs already makes — a change in the investment share
/// buys a permanently different LEVEL of output — applied at the one site that
/// was exempting itself from it. The plant the union built did not become worth
/// what a market economy's investment share says the morning after; discovering
/// that it was not is what the transition *was*, and the depth and the shape
/// both fall out of each republic's own transcribed shares rather than out of
/// anything named after a country (BIBLE §7).
///
/// `None` in, `None` out: a parent that has genuinely never been repriced hands
/// on the same claim, which is the honest carry rather than a fabricated zero.
fn inherited_capital_level(parent: &Nation) -> Option<f64> {
    parent.capital_level_paid
}

fn dissolve_ussr(w: &mut WorldState) {
    w.set_flag("ussr_dissolved");
    // `pop_off` is the union's own transcribed 1990 demography, and every
    // successor inherits it. A republic that did not exist in 1990 has no
    // transcribed rate of its own and inventing one is a refusal (iron rule 4);
    // what it demonstrably does have is the demography of the state it was part
    // of, which is the honest thing to carry across. Leaving it at zero would
    // silently re-impose the income-driven function this fix removed.
    let (gdp, pop, oil, strength, pop_off, inherited_capital, inherited_tech) = {
        let u = w.nation(NationId::USSR);
        (
            u.gdp,
            u.population,
            u.oil_mbd,
            u.mil_strength,
            u.pop_growth_offset,
            inherited_capital_level(u),
            u.tech.clone(),
        )
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
        pop_growth_offset: pop_off,
        inflation: 0.90, // transition price liberalization
        interest_rate: 0.20,
        tax_rate: 0.28,
        mil_spend_gdp: 0.045,
        state_invest_gdp: 0.04,
        priv_invest_gdp: 0.10,
        social_spend_gdp: None,
        annual_budget: None,
        program_budget: None,
        province_investment_reference: None,
        debt_gdp: 0.35,
        oil_mbd: oil * 0.85,
        bubble: 0.0,
        growth_last: -0.05,
        trade_level_paid: None,
        capital_level_paid: inherited_capital,
        state_invest_1990: None,
        // THE BOOKS CLOSE WITH THE FLAG, and this is a rule rather than a
        // default. `debt_gdp` at this site is an authored successor figure --
        // 0.35 for Russia against the union's 0.45, 0.15 for Ukraine because
        // of the 1994 zero option -- and NOT a share of the parent's dollar
        // debt, so carrying the parent's stock across would contradict the
        // ratio typed beside it. `None` says the successor's finances are the
        // ratio and nothing else until its own government opens its books,
        // which is exactly the state every other nation on the board is in.
        treasury_bn: None,
        debt_bn: None,
        // A successor state has no roads it built. `None` is the EXPLICIT
        // rule the design demands rather than an inherited accident: the
        // stock is a thing a government bought with a budget, and this
        // government has never enacted one.
        infra_extraction: None,
        stability: 38.0,
        separatism: 0.20,
        mil_strength: strength * 0.65,
        munitions: 1.0,
        war_exhaustion: 0.0,
        nuclear: true,
        arsenal: Default::default(),
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
        pop_growth_offset: pop_off,
        inflation: 1.10,
        interest_rate: 0.18,
        tax_rate: 0.30,
        mil_spend_gdp: 0.035,
        state_invest_gdp: 0.04,
        priv_invest_gdp: 0.08,
        social_spend_gdp: None,
        annual_budget: None,
        program_budget: None,
        province_investment_reference: None,
        debt_gdp: 0.15,
        oil_mbd: oil * 0.01,
        bubble: 0.0,
        growth_last: -0.06,
        trade_level_paid: None,
        capital_level_paid: inherited_capital,
        state_invest_1990: None,
        // THE BOOKS CLOSE WITH THE FLAG, and this is a rule rather than a
        // default. `debt_gdp` at this site is an authored successor figure --
        // 0.35 for Russia against the union's 0.45, 0.15 for Ukraine because
        // of the 1994 zero option -- and NOT a share of the parent's dollar
        // debt, so carrying the parent's stock across would contradict the
        // ratio typed beside it. `None` says the successor's finances are the
        // ratio and nothing else until its own government opens its books,
        // which is exactly the state every other nation on the board is in.
        treasury_bn: None,
        debt_bn: None,
        // A successor state has no roads it built. `None` is the EXPLICIT
        // rule the design demands rather than an inherited accident: the
        // stock is a thing a government bought with a budget, and this
        // government has never enacted one.
        infra_extraction: None,
        stability: 34.0,
        separatism: 0.35, // Crimea and the Donbas, from the day the flag went up
        mil_strength: strength * 0.15,
        munitions: 1.0,
        war_exhaustion: 0.0,
        // Ukraine woke up with the third-largest nuclear arsenal on earth — some
        // 1,900 strategic warheads left on its territory — and gave every one of
        // them back. The Budapest Memorandum of December 1994 traded them for
        // security assurances from Russia, the United States and Britain. It is
        // the only nuclear disarmament of its size ever carried out, and it is
        // why Ukraine enters the sim without the deterrent Russia keeps.
        nuclear: false,
        arsenal: Default::default(),
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
    // of the quarter of the union that was previously abstracted away. Nothing
    // is abstracted away now: feat/r2-gulf2 added Kyrgyzstan, Tajikistan and
    // Turkmenistan at the end of this list, which were the last 2.4% of net
    // material product and 4.6% of the people still missing, and the union now
    // comes apart into all fifteen of its republics.
    //
    // THERE IS NO HONEST DOLLAR GDP FOR ANY OF THESE REPUBLICS IN 1990, and
    // this block does not pretend otherwise. The official rouble rate was
    // fiction, the World Bank's early series for the region were built
    // backwards from 1992 dollar figures taken during a collapse, and the CIA
    // and Goskomstat disagree by a factor of two on the union itself. A
    // republican share of a union aggregate is the defensible thing to
    // transcribe, and that is what every `gdp` below is — a share, applied to
    // whatever the model's Soviet Union is worth on the month it dies.
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
        // Belarus — 4.2% of net material product and 10.2m of the union's
        // people. The most militarised ground in Europe per head: the
        // Belorussian Military District held three combined-arms armies and
        // something over 2,000 tanks, and 81 SS-25 mobile ICBMs stood at Lida
        // and Mozyr. It gave every warhead back under the Lisbon Protocol of
        // 23 May 1992, so it enters with no deterrent. Separatism is near zero:
        // the 1989 census counts 77.9% Belarusians with no territorial
        // minority anywhere, and the Popular Front never took the country.
        Republic { id: NationId::Belarus, gdp: 0.042, pop: 0.036, army: 0.045, oil: 0.003,
                   sep: 0.05, auth: 0.40, stab: 45.0, tfp: 0.004, infl: 1.20, rate: 0.22,
                   debt: 0.08, milspend: 0.030 },
        // Kazakhstan — 4.3% of output, 16.7m people, and 25.8 million tonnes of
        // crude in 1990, about 0.53 mbd, which is most of the union's oil
        // outside Russia. It also inherited 1,410 strategic warheads, 104 SS-18
        // silos and the Semipalatinsk test site, and returned all of it by
        // 1995. Separatism is high and it is demographic: the 1989 census gives
        // 39.7% Kazakhs against 37.8% Russians, and the Russians are the
        // northern oblasts along the border. That is the same arithmetic the
        // roster deliberately declined to write as a Russian claim; it belongs
        // here, as strain inside Kazakhstan, which is where it stayed.
        Republic { id: NationId::Kazakhstan, gdp: 0.043, pop: 0.058, army: 0.025, oil: 0.046,
                   sep: 0.30, auth: 0.62, stab: 42.0, tfp: 0.003, infl: 1.30, rate: 0.22,
                   debt: 0.08, milspend: 0.028 },
        // Uzbekistan — the most populous of the ten at 20.5m, but only 3.3% of
        // output: a cotton monoculture the union bought at administered prices,
        // and the Aral Sea spent to grow it. Karimov's apparatus survived 1991
        // without a break, which is why it starts the most stable of the poor
        // ones. It was also the most authoritarian state in the region until
        // Turkmenistan was added below, and it is not any more: Niyazov ran
        // unopposed on 98.3% where Karimov faced a named opponent and took
        // 86.0%, and Turkmenistan's `auth` is set above this one accordingly.
        Republic { id: NationId::Uzbekistan, gdp: 0.033, pop: 0.071, army: 0.018, oil: 0.005,
                   sep: 0.15, auth: 0.78, stab: 46.0, tfp: 0.002, infl: 1.00, rate: 0.20,
                   debt: 0.10, milspend: 0.025 },
        // Georgia — 1.6% of output and 5.5m people, and the worst opening
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
        // Armenia — 0.9% of output and 3.3m people, still rebuilding from the
        // Spitak earthquake of 7 December 1988 when the union ended. Internally
        // it is the most homogeneous republic in the union at 93.3% Armenian in
        // the 1989 census, so separatism is low; the strain is external, and it
        // is Karabakh. Azerbaijan and Turkey both closed their borders, leaving
        // one road through Georgia and one through Iran. The military share is
        // small in absolute terms and enormous relative to the economy under
        // it, which is what 6% of GDP on defence means here.
        Republic { id: NationId::Armenia, gdp: 0.009, pop: 0.0114, army: 0.008, oil: 0.000,
                   sep: 0.12, auth: 0.45, stab: 27.0, tfp: -0.002, infl: 1.50, rate: 0.28,
                   debt: 0.12, milspend: 0.060 },
        // Azerbaijan — 1.7% of output, 7.2m people, and 12.5 million tonnes of
        // crude in 1990, about 0.25 mbd from fields that had been the world's
        // largest in 1900 and were badly depleted by 1990. Separatism at 0.55 is
        // Nagorno-Karabakh, 76.9% Armenian in the 1989 census, whose soviet had
        // voted to leave for Armenia in February 1988. Two governments fell over
        // it in three years.
        Republic { id: NationId::Azerbaijan, gdp: 0.017, pop: 0.025, army: 0.010, oil: 0.022,
                   sep: 0.55, auth: 0.55, stab: 24.0, tfp: 0.000, infl: 1.40, rate: 0.26,
                   debt: 0.05, milspend: 0.060 },
        // Lithuania — 1.4% of output and 3.7m people, and the republic that
        // went first: the Supreme Council declared the restoration of
        // independence on 11 March 1990, took an economic blockade for it in
        // April, and buried fourteen people at the Vilnius television tower in
        // January 1991. Separatism is the lowest of the three Baltics because
        // the country is the most homogeneous of them, 79.6% Lithuanian, and
        // it was the only one to grant citizenship to all residents.
        Republic { id: NationId::Lithuania, gdp: 0.014, pop: 0.0128, army: 0.002, oil: 0.000,
                   sep: 0.10, auth: 0.20, stab: 50.0, tfp: 0.012, infl: 0.80, rate: 0.30,
                   debt: 0.05, milspend: 0.020 },
        // Latvia — 1.1% of output and 2.7m people, and the sharpest
        // demographic problem in the union outside Kazakhstan: 52.0% Latvian
        // against 34.0% Russian in the 1989 census, with Riga itself under half
        // Latvian. The citizenship law of 1994 left roughly a quarter of the
        // residents stateless. Almost no army: the Soviet garrison left in
        // August 1994 and took its equipment with it.
        Republic { id: NationId::Latvia, gdp: 0.011, pop: 0.0093, army: 0.002, oil: 0.000,
                   sep: 0.18, auth: 0.20, stab: 50.0, tfp: 0.012, infl: 0.80, rate: 0.30,
                   debt: 0.04, milspend: 0.020 },
        // Estonia — the smallest of the ten at 1.6m people and 0.7% of output,
        // and the fastest reformer of any successor: its own currency in June
        // 1992 on a currency board against the D-Mark, a flat income tax in
        // 1994, and unilateral free trade. Separatism is the Russophone
        // north-east, 30.3% of the country in 1989, which held autonomy
        // referendums in Narva and Sillamae in July 1993.
        Republic { id: NationId::Estonia, gdp: 0.007, pop: 0.0055, army: 0.001, oil: 0.000,
                   sep: 0.20, auth: 0.18, stab: 53.0, tfp: 0.015, infl: 0.75, rate: 0.30,
                   debt: 0.03, milspend: 0.020 },
        // Moldova — 1.2% of output and 4.4m people, and a border dispute with
        // itself. Transnistria declared on 2 September 1990 and Gagauzia on 19
        // August 1990; the war of 1992 was decided by the Soviet 14th Army,
        // which was already stationed on the left bank and did not leave.
        // Separatism at 0.45 is a state that lost a tenth of its territory and
        // most of its power generation in its second year.
        Republic { id: NationId::Moldova, gdp: 0.012, pop: 0.015, army: 0.003, oil: 0.000,
                   sep: 0.45, auth: 0.40, stab: 28.0, tfp: 0.000, infl: 1.40, rate: 0.26,
                   debt: 0.07, milspend: 0.035 },
        // Kyrgyzstan — 0.8% of net material product and 4.37m people, the last
        // republic in the union to have anything worth taking and the first to
        // try being a democracy about it. Akayev was a physicist, not a First
        // Secretary, and the republic left the rouble zone first, in May 1993,
        // taking the som and the inflation cure with it; that early exit is
        // what the inflation multiplier here is below Tajikistan's for. There
        // is essentially no oil and essentially no army — the Turkestan
        // Military District's Kyrgyz remnant was a training establishment and
        // two motor-rifle regiments. Separatism at 0.25 is Osh: 12.9% Uzbeks by
        // the 1989 census, concentrated in two southern oblasts, and several
        // hundred dead there in June 1990.
        Republic { id: NationId::Kyrgyzstan, gdp: 0.008, pop: 0.0151, army: 0.003, oil: 0.000,
                   sep: 0.25, auth: 0.35, stab: 40.0, tfp: -0.002, infl: 1.30, rate: 0.26,
                   debt: 0.10, milspend: 0.025 },
        // Tajikistan — 0.8% of output and 5.25m people, and on every measure
        // that survives the rouble the poorest republic in the union: the
        // lowest income per head, the highest birth rate, and a cotton and
        // aluminium economy the union bought at administered prices the way it
        // bought Uzbekistan's. Separatism at 0.55 is not the ethnic split but
        // the regional one, which is the fact this row exists to carry:
        // Leninabad supplied every First Secretary for fifty years, Kulob
        // supplied the militias, and Gharm and Badakhshan supplied the
        // opposition, with Gorno-Badakhshan an autonomous oblast over 45% of
        // the territory. It starts the least stable successor of the fifteen —
        // below Georgia — on the Dushanbe riots of February 1990, the state of
        // emergency that followed and a presidential election 43% of the
        // republic did not accept. `milspend` is low and that is deliberate:
        // Tajikistan had no national army at all until 1993, because the 201st
        // Motor Rifle Division on its soil stayed Russian and did the work.
        // Nothing here schedules the civil war; this is the ground it started
        // from. https://en.wikipedia.org/wiki/1990_Dushanbe_riots
        Republic { id: NationId::Tajikistan, gdp: 0.008, pop: 0.0182, army: 0.002, oil: 0.000,
                   sep: 0.55, auth: 0.55, stab: 18.0, tfp: -0.006, infl: 1.70, rate: 0.30,
                   debt: 0.12, milspend: 0.025 },
        // Turkmenistan — 0.8% of output and 3.62m people, and the only one of
        // the three with an export: 87.8 billion cubic metres of gas in 1990,
        // second in the union after Russia, plus 5.7 million tonnes of crude.
        // THE `oil` FIGURE BELOW UNDERSTATES THIS REPUBLIC AND THE MODEL HAS NO
        // PLACE TO SAY SO. 0.010 is the crude share and it is right; the gas is
        // roughly ten times the energy and there is no `gas_bcm` field, so
        // Turkmenistan enters poorer in tradeable resources than it was. The
        // debt is low for the same reason the gas is missing — Ashgabat was a
        // net creditor through the nineties, owed for gas by customers who did
        // not pay, which is a worse position than the number looks and the one
        // that broke the economy in 1997. Separatism is the lowest of the three
        // at 72.0% Turkmen with no territorial minority; `auth` at 0.85 is the
        // highest in the model and is Niyazov's 98.3% against an empty ballot.
        Republic { id: NationId::Turkmenistan, gdp: 0.008, pop: 0.0125, army: 0.008, oil: 0.010,
                   sep: 0.08, auth: 0.85, stab: 48.0, tfp: -0.001, infl: 1.50, rate: 0.24,
                   debt: 0.05, milspend: 0.035 },
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
            pop_growth_offset: pop_off,
            inflation: r.infl,
            interest_rate: r.rate,
            tax_rate: 0.30,
            mil_spend_gdp: r.milspend,
            state_invest_gdp: 0.04,
            priv_invest_gdp: 0.09,
            social_spend_gdp: None,
            annual_budget: None,
            program_budget: None,
            province_investment_reference: None,
            // The "zero option" of 1994 gave Moscow every rouble of Soviet
            // foreign debt and, with it, every Soviet foreign asset. The other
            // republics therefore start almost unencumbered, and what debt they
            // carry here is domestic and inherited from the enterprises.
            debt_gdp: r.debt,
            oil_mbd: oil * r.oil,
            bubble: 0.0,
            growth_last: -0.08,
            trade_level_paid: None,
            capital_level_paid: inherited_capital,
            state_invest_1990: None,
            // The books close with the flag. See the note at `dissolve_ussr`.
            treasury_bn: None,
            debt_bn: None,
            // A successor state has no roads it built. `None` is the EXPLICIT
            // rule the design demands rather than an inherited accident: the
            // stock is a thing a government bought with a budget, and this
            // government has never enacted one.
            infra_extraction: None,
            stability: r.stab,
            separatism: r.sep,
            mil_strength: strength * r.army,
            // Full, and for once that is not a default: what these republics
            // woke up holding was the Soviet depot system, and the stocks on
            // their ground were the stocks of a force built to fight NATO. The
            // army they inherited was a fraction of the union's; the ammunition
            // was not.
            munitions: 1.0,
            war_exhaustion: 0.0,
            // Belarus and Kazakhstan both woke up holding strategic warheads
            // and both gave them back, Belarus by 1996 and Kazakhstan by 1995,
            // under the Lisbon Protocol of May 1992 and the Budapest assurances
            // of December 1994. Nobody in this group enters with a deterrent.
            nuclear: false,
            arsenal: Default::default(),
            political_capital: seated_political_capital(r.stab, r.infl, r.auth),
            tech: crate::tech::TechState::inherit(&inherited_tech, r.tfp),
        });
    }

    // Reconcile every successor's productivity base against the technology it
    // just inherited, exactly as the loader does at 1990. `inherit` clones the
    // parent's whole known set and takes the successor's own transcribed trend
    // straight into `tfp_base`, so without this the union's entire technology
    // stock is paid for a second time — fifteen times over, from the month the
    // flag comes down. It is the one double-count a t=0 acceptance test cannot
    // see, which is why the test that guards it runs through the dissolution.
    //
    // Same two functions the loader uses. One implementation of the identity.

    // A successor inherits a thawed version of the union's standing abroad —
    // but only if it agreed to be a successor. The Alma-Ata Protocol of 21
    // December 1991 was signed by eleven republics; Georgia and the three
    // Baltic states did not sign it, and their refusal was the whole of their
    // foreign policy. Lithuania, Latvia and Estonia held that the annexation of
    // 1940 was void, that they were the inter-war republics restored rather
    // than new states, and on that ground they refused the Commonwealth,
    // refused a share of Soviet debt and refused a share of Soviet assets.
    // Georgia stayed out until Russian pressure put it in, in December 1993.
    // Those four therefore start neutral to the world and have to build their
    // own relations from nothing, which is what they did.
    let rels: Vec<(NationId, f64)> = start_nations()
        .iter()
        .filter(|x| **x != NationId::USSR)
        .map(|x| (*x, w.relation(NationId::USSR, *x) * 0.5 + 10.0))
        .collect();
    let signed_alma_ata = |id: NationId| {
        !matches!(
            id,
            NationId::Georgia | NationId::Lithuania | NationId::Latvia | NationId::Estonia
        )
    };
    let successors: Vec<NationId> = [NationId::Russia, NationId::Ukraine]
        .into_iter()
        .chain(republics.iter().map(|r| r.id))
        .filter(|id| signed_alma_ata(*id))
        .collect();
    for successor in successors.iter().copied() {
        for (other, v) in &rels {
            w.set_relation(successor, *other, *v);
        }
    }
    // How the fifteen regard each other on the morning after. The pattern is not
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
        // The Karabakh war is the one open conflict inside the group, and the
        // roster states it as a claim rather than as a scripted war.
        (NationId::Armenia, NationId::Azerbaijan, -70.0),
        (NationId::Georgia, NationId::Azerbaijan, 15.0),
        (NationId::Georgia, NationId::Armenia, 10.0),
        (NationId::Kazakhstan, NationId::Uzbekistan, 15.0),
        // The three added with feat/r2-gulf2. Bishkek and Dushanbe both needed
        // Moscow and both said so — Kyrgyzstan signed the Collective Security
        // Treaty and Tajikistan's government was kept in the building by the
        // 201st Motor Rifle Division — so both sit near Belarus rather than
        // near Tashkent. Ashgabat is the outlier and the interesting one:
        // Niyazov refused the Collective Security Treaty in 1992, refused full
        // CIS membership, and had permanent neutrality recognised by the
        // General Assembly on 12 December 1995. Turkmenistan is warm to nobody
        // and cold to nobody, which is a policy and not an absence of one.
        (NationId::Russia, NationId::Kyrgyzstan, 40.0),
        (NationId::Russia, NationId::Tajikistan, 45.0),
        (NationId::Russia, NationId::Turkmenistan, 15.0),
        // Tashkent is the regional power and behaves like one. Karimov backed
        // Kulob against the Tajik opposition with Uzbek aircraft from 1992,
        // which bought a friendly government and a permanent quarrel with the
        // half of Tajikistan that lost; the Fergana enclaves and the gas cutoffs
        // did the rest. Kyrgyzstan is the milder version of the same relation.
        (NationId::Uzbekistan, NationId::Kyrgyzstan, 10.0),
        (NationId::Uzbekistan, NationId::Tajikistan, -5.0),
        (NationId::Uzbekistan, NationId::Turkmenistan, 0.0),
        (NationId::Kazakhstan, NationId::Kyrgyzstan, 30.0),
        (NationId::Kazakhstan, NationId::Turkmenistan, 15.0),
        (NationId::Kyrgyzstan, NationId::Tajikistan, 10.0),
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
    w.shift_relation(NationId::Turkey, NationId::Turkmenistan, 20.0);
    w.shift_relation(NationId::Turkey, NationId::Kyrgyzstan, 20.0);
    w.shift_relation(NationId::Turkey, NationId::Armenia, -50.0);
    // Tajikistan is Persian-speaking and gets none of that: Ankara's summits
    // were for the Turkic states and Dushanbe was not invited to them. Tehran
    // took the opposite half of the map for the same reason — a common language
    // with Tajikistan, a 992km border and the Korpeje-Kurt Kui gas line with
    // Turkmenistan, and no interest at all in a strong Azerbaijan next to its
    // own twenty million Azeris. Iran hosted the Tajik opposition and then
    // brokered the peace that ended the war in 1997, which is the one case in
    // this block of an outside power making a settlement rather than a client.
    w.shift_relation(NationId::Iran, NationId::Armenia, 25.0);
    w.shift_relation(NationId::Iran, NationId::Azerbaijan, -15.0);
    w.shift_relation(NationId::Iran, NationId::Turkmenistan, 30.0);
    w.shift_relation(NationId::Iran, NationId::Tajikistan, 30.0);
    // Beijing and the Central Asians, which is the dyad this block most has to
    // get right: Kazakhstan is a 1,765km land border with a great power, and
    // the model's derived appetite reads a border. What actually happened is
    // the opposite of appetite. China recognised Kazakhstan on 27 December
    // 1991 and opened an embassy on 3 January 1992; the two governments signed
    // the border agreement of 26 April 1994 that split the 34,000 sq km the
    // Sino-Soviet talks had left in dispute, settled the remainder in 1997 and
    // 1998, and founded the Shanghai Five together on 26 April 1996 — the only
    // multilateral body China has ever built. Demarcation was complete by 2002.
    // Entering these two as strangers at 5 and letting a border do the rest
    // produces a Chinese invasion of Kazakhstan, which is a war this model
    // would be inventing out of a frontier both states spent the decade
    // agreeing on. https://en.wikipedia.org/wiki/Shanghai_Five
    w.set_relation(NationId::China, NationId::Kazakhstan, 40.0);
    w.set_relation(NationId::China, NationId::Uzbekistan, 25.0);
    // Kyrgyzstan and Tajikistan are the same argument and the same evidence.
    // The Shanghai Five of 26 April 1996 had exactly five members — China,
    // Russia, Kazakhstan, Kyrgyzstan and Tajikistan — and it was founded on a
    // border agreement, not against anybody. China settled with Kyrgyzstan in
    // 1996 and 1999 and with Tajikistan in 2002, taking under 1% of the 28,000
    // sq km it had claimed in the Pamirs. Two new states with almost no army
    // sharing 858km and 414km of frontier with a great power is precisely the
    // shape `dyads.rs` reads as an opportunity, and the thing that stopped it
    // was this. https://en.wikipedia.org/wiki/Shanghai_Five
    w.set_relation(NationId::China, NationId::Kyrgyzstan, 35.0);
    w.set_relation(NationId::China, NationId::Tajikistan, 30.0);
    // Warsaw was the first capital to recognise Ukraine, on 2 December 1991,
    // and recognised Belarus and Moldova within the month; the Polish-Moldovan
    // treaty of friendship was signed in 1994. Poland and Moldova share no
    // border, only the region this model puts them both in, and leaving them
    // at zero lets that regional contact produce a war between two states that
    // have never had a quarrel.
    // Ukraine is left where the inheritance put it: Warsaw recognised it first,
    // on 2 December 1991, but that dyad is already modelled and this block does
    // not reach back into a nation that was here before it.
    w.set_relation(NationId::Poland, NationId::Moldova, 30.0);
    w.set_relation(NationId::Poland, NationId::Belarus, 25.0);
    // The three Baltic states do not start neutral to the West, because the
    // West never accepted that they had left it. The Welles Declaration of 23
    // July 1940 refused recognition of the annexation and the United States
    // held that line unbroken for fifty-one years, keeping the pre-war legations
    // open in Washington the whole time; the Baltic gold reserves sat untouched
    // in London and Stockholm. Iceland recognised Lithuania on 11 February 1991,
    // months before anyone else dared, the Community and the Nordics followed in
    // August, and all three were seated at the United Nations on 17 September
    // 1991. That is a starting position, not something to be earned.
    for baltic in [NationId::Lithuania, NationId::Latvia, NationId::Estonia] {
        w.set_relation(baltic, NationId::USA, 45.0);
        w.set_relation(baltic, NationId::Germany, 40.0);
        w.set_relation(baltic, NationId::UK, 35.0);
        w.set_relation(baltic, NationId::France, 30.0);
        // Warsaw is warm but not uncomplicated: the Polish minority around
        // Vilnius kept Poland and Lithuania at arm's length until the treaty
        // of April 1994 settled it.
        w.set_relation(baltic, NationId::Poland, 25.0);
        w.set_relation(baltic, NationId::Japan, 15.0);
    }
    // Georgia was recognised on the same terms as the rest and had no such
    // history to trade on; Washington's interest arrives later, with the
    // pipeline.
    w.shift_relation(NationId::Georgia, NationId::USA, 15.0);

    // The Collective Security Treaty, signed at Tashkent on 15 May 1992 by
    // Russia, Armenia, Kazakhstan, Kyrgyzstan, Tajikistan and Uzbekistan, with
    // Belarus acceding on 31 December 1993. It is written here as four
    // guarantees radiating from Moscow rather than as the clique the treaty
    // text describes, because that is what it was: nobody in Alma-Ata ever
    // believed Yerevan would come, and Armenia's own guarantee — the 102nd
    // Military Base at Gyumri, garrisoned continuously from 1992 — was Russian
    // troops on Armenian soil and nothing else.
    //
    // This is transcription with teeth, and it is entered because leaving it
    // out was measurably wrong rather than merely incomplete. A newly sovereign
    // Kazakhstan is a 1,765km border, sixteen million people, half a million
    // barrels a day and almost no army, sitting next to the largest land power
    // on earth; with no guarantee behind it the derived appetite model in
    // dyads.rs invades it in roughly two runs in five, which is a war that did
    // not happen and never came close to happening. The reason it did not
    // happen is exactly this treaty and the arsenal behind it, so the treaty is
    // the thing to write down. Deterrence is `dyads.rs`'s own term for it: a
    // pact partner's strength counts toward what an aggressor expects to face,
    // and unlike a hoped-for coalition it is barely discounted.
    // https://en.wikipedia.org/wiki/Collective_Security_Treaty_Organization
    //
    // Kyrgyzstan and Tajikistan were named in the Tashkent text itself and are
    // now on the board, so they are here. Turkmenistan was not a signatory and
    // is deliberately absent: Niyazov refused the treaty in 1992 and had
    // permanent neutrality recognised by UN General Assembly resolution 50/80
    // on 12 December 1995, the only state ever given it. That refusal is the
    // most consequential fact about Turkmenistan's foreign policy and the one
    // this list would erase by rounding six signatures up to seven.
    for client in [
        NationId::Armenia,
        NationId::Kazakhstan,
        NationId::Uzbekistan,
        NationId::Belarus,
        NationId::Kyrgyzstan,
        NationId::Tajikistan,
    ] {
        let (a, b) = if NationId::Russia <= client {
            (NationId::Russia, client)
        } else {
            (client, NationId::Russia)
        };
        w.statecraft.pacts.push(Pact { a, b, since_year: w.year, since_month: w.month });
    }
    w.headline(
        "The Collective Security Treaty is signed at Tashkent; Moscow guarantees six of its neighbours."
            .into(),
    );
    // Kyiv and Moscow start as quarrelling relatives rather than enemies: the
    // Black Sea Fleet and Crimea are already in dispute, but the divorce of
    // December 1991 was signed, not fought.
    w.set_relation(NationId::Russia, NationId::Ukraine, 15.0);

    // The successors inherit the ground. A state that is home to no theatre
    // would be expeditionary in its own capital, and would fight for Moscow with
    // the fraction of itself it could have sent to Angola. Nothing has to be
    // moved here any more: a theatre's home list is its region's membership on
    // the roster, and each of the twelve carries its own region — so the seats
    // were always there and the dissolution simply fills them. What still has
    // to go is the paperwork, because a consent given to a state that no longer
    // exists is not a consent.
    w.access.retain(|a| a.host != NationId::USSR && a.seeker != NationId::USSR);

    // Each republic takes its own ground; anything the union held beyond its
    // own list — conquests — stays with the continuation state, Russia, which
    // is first in this list on purpose.
    crate::districts::dissolve_to(
        w,
        NationId::USSR,
        &[
            NationId::Russia,
            NationId::Ukraine,
            NationId::Belarus,
            NationId::Kazakhstan,
            NationId::Uzbekistan,
            NationId::Georgia,
            NationId::Armenia,
            NationId::Azerbaijan,
            NationId::Lithuania,
            NationId::Latvia,
            NationId::Estonia,
            NationId::Moldova,
            NationId::Kyrgyzstan,
            NationId::Tajikistan,
            NationId::Turkmenistan,
        ],
    );

    w.headline("THE SOVIET UNION HAS DISSOLVED. Fifteen republics take up their own seats.".into());
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
    // See `dissolve_ussr` for why the successors inherit the federation's
    // demographic offset rather than starting at zero.
    let (gdp, pop, oil, strength, infl, debt, pop_off, inherited_capital, inherited_tech) = {
        let y = w.nation(NationId::Yugoslavia);
        (y.gdp, y.population, y.oil_mbd, y.mil_strength, y.inflation, y.debt_gdp,
         y.pop_growth_offset, inherited_capital_level(y), y.tech.clone())
    };
    {
        let y = w.nation_mut(NationId::Yugoslavia);
        y.alive = false;
        y.gdp = 0.0;
    }

    // (id, GDP share, pop share, JNA share, separatism, authoritarianism, stability, tfp)
    //
    // SIX republics now, not four. Macedonia and Montenegro used to leave
    // unsimulated with a note saying so — Macedonia because it seceded without
    // a shot and Montenegro because it was folded into Belgrade's row, which
    // is what the "serbia and montenegro" alias on the Serbia roster row and
    // the SRB+MNE territory pairing in the browser UI both recorded. Both are
    // in the roster now, so both take their own share here.
    //
    // The shares sum to 1.000 in every column and that is the constraint this
    // block is built to. Pulling two republics out could not be done by
    // appending two rows: the four incumbents summed to 0.94 of output and
    // 0.895 of population, leaving 0.06 and 0.105 for the two that were being
    // dropped, and the real figures for those two are larger than the gap.
    // Macedonia's 1990 GDP on World Bank NY.GDP.MKTP.CD (series MKD) is
    // $4.700bn against the $88bn yugoslavia.json carries for the federation,
    // which is 0.053, and Montenegro was about 0.019 of Yugoslav social
    // product. On population the 1991 census gives Macedonia 2,034,000 and
    // Montenegro 615,000 of 23.53m, or 0.086 and 0.026. Serbia's two figures
    // are therefore reduced — 0.36 to 0.348 and 0.42 to 0.411 — and Belgrade
    // carries the rounding for all six so that a dissolution does not
    // manufacture a percent of Europe's output and population out of nothing.
    // Serbia's population share at 0.411 is about 1.2% under the census
    // (9.78m of 23.53m is 0.4156); that difference is the upward rounding of
    // Croatia, Slovenia and Bosnia, left where the incumbents put it rather
    // than redistributed across rows this branch does not own.
    // A type alias would name the tuple without making the eight columns any
    // clearer; the comment beside each row already does that.
    #[allow(clippy::type_complexity)]
    let parts: [(NationId, f64, f64, f64, f64, f64, f64, f64); 6] = [
        // Belgrade keeps the army, and Kosovo and Vojvodina keep Belgrade busy.
        (NationId::Serbia,   0.348, 0.411, 0.70, 0.45, 0.75, 40.0, 0.002),
        // A twelve percent Serb minority concentrated in the Krajina.
        (NationId::Croatia,  0.25, 0.20, 0.12, 0.35, 0.45, 45.0, 0.012),
        // ~88% Slovene, no minority worth a war, and the richest republic.
        (NationId::Slovenia, 0.20, 0.085, 0.08, 0.05, 0.25, 62.0, 0.020),
        // 44% Bosniak, 31% Serb, 17% Croat — a republic that is all minorities.
        (NationId::Bosnia,   0.13, 0.19, 0.05, 0.85, 0.40, 30.0, 0.006),
        // The poorest republic, and the only one that got out clean. 21%
        // Albanian at the 1991 census, concentrated in the north-west, which
        // is a real strain and an order of magnitude short of Bosnia's — hence
        // 0.30. The JNA share is 0.02 and that is the fact rather than an
        // estimate of weakness: the army withdrew from Macedonia on 26 March
        // 1992 and took every piece of heavy equipment with it, having already
        // confiscated the Territorial Defence stocks in 1990, so Skopje
        // started with essentially nothing and had to buy an army.
        // Authoritarianism 0.30 — Gligorov's was the freest of the six.
        (NationId::Macedonia, 0.053, 0.086, 0.02, 0.30, 0.30, 55.0, 0.004),
        // The republic that did not leave. Bulatovic's League of Communists
        // won 58.3% in December 1990 as Milosevic's ally and Montenegro stayed
        // inside the Federal Republic until the referendum of 21 May 2006, so
        // authoritarianism 0.60 tracks Belgrade's 0.75 rather than Skopje's
        // 0.30 and the two open at +60 below. Separatism 0.25 is not a
        // minority — Montenegro was 61.9% Montenegrin, 14.6% Muslim, 9.3%
        // Serb — it is the Montenegrin/Serb identity question that eventually
        // produced a 55.5% independence vote. The 0.03 JNA share is the navy
        // in the Boka Kotorska and nothing else; the Podgorica corps answered
        // to Belgrade.
        //
        // WHAT 0.03 DOES TO THIS REPUBLIC, measured over 25 years across seeds
        // 1..20 and written down rather than fixed by inflating the number.
        // Montenegro is invaded 9 times in those 20 runs — Albania 4, Croatia
        // 2, Serbia 1, Bosnia 1, plus one repeat — because 0.03 of a strength
        // index of 20 is 0.6, which makes it the weakest state in the Balkans
        // sitting inside four borders. None of those wars happened. Three
        // things say leave it alone anyway. The share is the fact: Montenegro
        // seized nothing in 1991 because it did not leave in 1991, and the
        // reason it did not leave is precisely that it could not have defended
        // itself alone — the model is showing the counterfactual the roster
        // cannot express, not getting the arithmetic wrong. The rate is not an
        // outlier: Serbia invades Bosnia 29 times in the same 20 runs, so a
        // small Balkan successor being eaten repeatedly is this model's
        // existing behaviour and not something these two rows introduced. And
        // inflating the JNA share to make the wars stop would be encoding the
        // outcome, which is the one thing this table is not for.
        (NationId::Montenegro, 0.019, 0.026, 0.03, 0.25, 0.60, 45.0, 0.002),
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
            pop_growth_offset: pop_off,
            inflation: infl,
            interest_rate: 0.25,
            tax_rate: 0.33,
            mil_spend_gdp: 0.05,
            state_invest_gdp: 0.05,
            priv_invest_gdp: 0.10,
            social_spend_gdp: None,
            annual_budget: None,
            program_budget: None,
            province_investment_reference: None,
            debt_gdp: debt,
            oil_mbd: oil * g,
            bubble: 0.0,
            growth_last: -0.06,
            trade_level_paid: None,
            capital_level_paid: inherited_capital,
            state_invest_1990: None,
            // The books close with the flag. See the note at `dissolve_ussr`.
            treasury_bn: None,
            debt_bn: None,
            // A successor state has no roads it built. `None` is the EXPLICIT
            // rule the design demands rather than an inherited accident: the
            // stock is a thing a government bought with a budget, and this
            // government has never enacted one.
            infra_extraction: None,
            stability: stab,
            separatism: sep,
            mil_strength: strength * m,
            munitions: 1.0,
            war_exhaustion: 0.0,
            nuclear: false,
            arsenal: Default::default(),
            political_capital: seated_political_capital(stab, infl, auth),
            // Each republic keeps the federation's technical base and starts its
            // own research from nothing.
            tech: crate::tech::TechState::inherit(&inherited_tech, tfp),
        });
    }
    // Same reconciliation as the Soviet path: the federation's technical base is
    // inherited in full, and the successors' transcribed trends already price it
    // in, so it must come back out of `tfp_base` or every republic is paid twice
    // for Yugoslav industry.

    // Successors inherit the federation's standing abroad, thinned out.
    let inherited: Vec<(NationId, f64)> = start_nations()
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
        // Montenegro did not leave with the others and the number says so.
        // +60 is the highest opening figure in this set, and it is the fifteen
        // years the two republics spent as one federal state after everybody
        // else had gone — a fact the roster cannot express as a border and
        // therefore carries here.
        (NationId::Serbia, NationId::Montenegro, 60.0),
        // Belgrade let Skopje go. The JNA's withdrawal in March 1992 was
        // negotiated and unopposed, and Serbia and Macedonia are the one pair
        // in the Yugoslav wreckage that never fired at each other. Mildly
        // positive rather than neutral, because the border itself stayed
        // undemarcated until the treaty of February 2001.
        (NationId::Serbia, NationId::Macedonia, 10.0),
        // Zagreb and Ljubljana had no quarrel with Skopje and every reason to
        // want the federation's exit door to look survivable.
        (NationId::Croatia, NationId::Macedonia, 15.0),
        (NationId::Slovenia, NationId::Macedonia, 15.0),
        // Prevlaka: 93 hectares of Croatian territory the Yugoslav navy held
        // from 1992, and the shelling of Dubrovnik in the autumn of 1991 was
        // launched across this border by Montenegrin reservists. Negative, and
        // well short of Serbia's -45 because Montenegro's part in that war was
        // Belgrade's decision rather than Podgorica's.
        (NationId::Croatia, NationId::Montenegro, -25.0),
        (NationId::Bosnia, NationId::Montenegro, -10.0),
        (NationId::Macedonia, NationId::Montenegro, 5.0),
    ];
    for (a, b, v) in between {
        w.set_relation(*a, *b, *v);
    }

    // Bosnia's war is fought in the Balkans' own mountains and towns, and its
    // successors are home to them — which needs nothing done here, because a
    // theatre's home list is its region's membership on the roster and all four
    // republics are filed under Balkans. This used to be a hand-written swap,
    // and a hand-written swap is exactly the thing that gets forgotten when the
    // next federation comes apart: every republic would then be treated as
    // expeditionary in its own country, fighting for Sarajevo with the fraction
    // of itself it could have sent abroad.
    w.access
        .retain(|a| a.host != NationId::Yugoslavia && a.seeker != NationId::Yugoslavia);

    // Each republic takes its own ground; any conquest of the federation's
    // stays with Belgrade, the continuation state, first in this list.
    crate::districts::dissolve_to(
        w,
        NationId::Yugoslavia,
        &[
            NationId::Serbia,
            NationId::Croatia,
            NationId::Slovenia,
            NationId::Bosnia,
            NationId::Macedonia,
            NationId::Montenegro,
        ],
    );

    w.headline(
        "YUGOSLAVIA HAS DISSOLVED. Slovenia, Croatia, Bosnia, Macedonia, Montenegro and Serbia stand alone."
            .into(),
    );
    w.headline("The JNA's divisions, and its arsenal, remain in Belgrade's hands.".into());
}

// The powers that keep clients are a flag on the roster row now rather than an
// array here, so adding a nation cannot leave the list stale — see
// `nations::patrons`, which holds the membership and the reasoning for it.

/// What a great power does in the eleven months of the year it is not invading
/// anyone. Each patron gets a handful of independent low-probability chances per
/// month to open a chequebook, sign a guarantee, open a market, or pay somebody
/// to make a rival's client ungovernable. Everything goes through the same
/// `Command` the player uses, so the AI cannot do anything a player could not.
/// Preserve a monthly event hazard while making the decision available daily.
/// The one RNG remains WorldState's, and command effects are never prorated.
fn monthly_chance(w: &mut WorldState, monthly_probability: f64) -> bool {
    let probability = crate::clock::chance(w, monthly_probability);
    w.rng.chance(probability)
}

fn ai_statecraft(w: &mut WorldState) {
    // Ruling 4's "must first try to buy": every AI state short of a line asks
    // every willing seller before this month's appetite is priced. Behind the
    // market switch; consumes no randomness; returns at once while nothing is
    // short (resources.rs, spec section 6.2). ONCE A MONTH in daily play too:
    // the pass's memory, its PATIENCE and its inbox cap are all written in
    // months, and physical freight counts only the day's fills, so a pass run
    // every day re-asked while its own signing was still at sea — seven
    // France-USA copper contracts on seven January days (2026-09-03, seed 7,
    // player Iraq) against one signing in three legacy months.
    if crate::clock::month_end(w) {
        crate::resources::ai_purchases(w);
    }

    let active: Vec<NationId> = patrons()
        .iter()
        .copied()
        .filter(|p| w.nation_opt(*p).is_some_and(|n| n.alive) && Some(*p) != w.player)
        .collect();

    for p in active {
        // A patron with its own house on fire stops buying friends.
        if w.nation(p).stability < 25.0 {
            continue;
        }

        // ---- Patronage. The scoring is where the competition lives. ----
        let headroom = crate::statecraft::MAX_AID_SHARE - w.aid_share_committed(p);
        if headroom > 0.0008 && monthly_chance(w, 0.10) {
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
        if monthly_chance(w, 0.05) {
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
        if monthly_chance(w, 0.022) {
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
        if !monthly_chance(w, 0.015) {
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
        .filter(|id| Some(*id) != w.player && !patrons().contains(id))
        .collect();
    for s in seekers {
        let strength = w.nation(s).mil_strength;
        let threatened = w.nations.iter().any(|n| {
            n.alive && n.id != s && n.mil_strength > strength * 1.3 && w.relation(s, n.id) < -30.0
        });
        if !threatened || !monthly_chance(w, 0.07) {
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
        .filter(|c| !patrons().contains(c) && !majors().contains(c))
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

/// Every state that might start a war this month, and how likely it is to.
///
/// There is no list of aggressors and no table of pairs. Anyone alive may want
/// something from anyone they can reach; `dyads::war_appetite` decides how much,
/// out of borders, claims, relations and the two governments' own condition.
///
/// Order is registry order for the attacker and registry order within its
/// contact set for the target, so the sequence of die rolls — and therefore the
/// whole timeline — is fixed by construction.
fn ai_wars(w: &mut WorldState) {
    let candidates: Vec<(NationId, NationId, f64)> = {
        let mut v = vec![];
        for a in all_nations().iter().copied() {
            let an = match w.nation_opt(a) {
                Some(n) => n,
                None => continue, // has not been born yet, if it ever is
            };
            if !an.alive || Some(a) == w.player || w.at_war(a) || an.war_exhaustion > 0.3 {
                continue;
            }
            // Ruling 4's predicate asks whether `a` has a stall, and that
            // does not depend on the target: one mask for the whole sweep
            // over `a`'s contacts, not one per dyad (resources.rs,
            // `action_stalled_mask`).
            let stalled = if w.rules.resource_market {
                Some(crate::resources::action_stalled_mask(w, a))
            } else {
                None
            };
            for t in crate::dyads::contacts(a).iter().copied() {
                match w.nation_opt(t) {
                    Some(n) if n.alive => {}
                    _ => continue,
                }
                if w.at_war(t) {
                    continue;
                }
                let p = crate::dyads::war_appetite_with(w, a, t, stalled.as_ref());
                if p > 0.0 {
                    v.push((a, t, p));
                }
            }
        }
        v
    };
    for (a, t, p) in candidates {
        if w.at_war(a) {
            continue;
        }
        if !monthly_chance(w, p) {
            continue;
        }
        // The appetite does not go away because there is already an argument.
        // When it comes up again and the quarrel is open, it is a push up the
        // ladder — which is where the political reasons for a war (debt, a
        // neighbour coming apart, a border nobody accepts) finally reach the
        // commitment ladder instead of only deciding whether there is an
        // argument at all. Priced like any other rung, and refusable.
        if let Some(c) = w.conflict_between(a, t) {
            let (id, rung) = (c.id, c.posture_of(a).map_or(1, |b| b.rung));
            if rung < INVASION_RUNG {
                let _ = crate::apply_command(
                    w,
                    &crate::Command::SetCommitment { conflict: id, nation: a, rung: rung + 1 },
                );
            }
            continue;
        }
        // What this roll used to do was launch an invasion. It now opens a
        // QUARREL, at rung 1, and nothing else — the appetite is the same, the
        // reasoning above it is the same, but the state has bought a public
        // grievance rather than a war, and every rung between here and an army
        // crossing the border is a separate decision it has to pay for
        // (`commitment::ai_ladder`). This is the single change that turns a
        // nine-rung ladder from a three-state machine into a climb: no conflict
        // in the world is born above rung 1 any more.
        let th = war::theatre_between(w, a, t);
        if crate::apply_command(w, &crate::Command::OpenConflict { opener: a, target: t, theatre: th })
            .is_ok()
        {
            // Saying out loud what the quarrel is for. Cheap, and it is what
            // separates a state that wants the ground from one that only wants
            // the neighbour weakened.
            if let Some(c) = w.conflict_between(a, t) {
                let id = c.id;
                let _ = crate::apply_command(
                    w,
                    &crate::Command::SetObjective { conflict: id, nation: a, objective: Objective::Seize },
                );
                // Ruling 4: when the quarrel is a last resort for a line every
                // seller refused, say which district it is for. Free,
                // refusable, validated by the command; behind the market
                // switch, like the appetite term that opened it.
                if w.rules.resource_market {
                    if let Some(aim) = crate::dyads::last_resort(w, a, t) {
                        let _ = crate::apply_command(
                            w,
                            &crate::Command::SetAim {
                                conflict: id,
                                nation: a,
                                district: aim.district,
                                commodity: aim.commodity,
                            },
                        );
                    }
                }
            }
        }
    }

    // AI peace offers: badly losing attackers sue for peace (abstract: white peace at high exhaustion handled in war tick)
}
