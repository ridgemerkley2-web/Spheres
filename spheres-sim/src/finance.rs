use crate::world::*;

/// Currencies, capital flows and the way a promise about an exchange rate turns
/// into a depression.
///
/// The chain this module models runs in one direction. A government fixes its
/// currency to buy credibility it has not earned; the fixed rate makes borrowing
/// abroad look free, so the borrowing is done in a currency the borrower cannot
/// print; the money that funds it can leave in a month. Nothing about that is
/// dangerous while the promise holds. When it stops holding, the debt doubles
/// overnight in the money that has to service it, and an economy that was growing
/// at eight percent discovers it is insolvent.
///
/// The last step is contagion, and it is the reason the whole thing is here.
/// Money does not reassess one country when a peg breaks; it reassesses everyone
/// who looked like that country — same regime, same leverage, same neighbourhood
/// — and it does so whether or not those countries had anything wrong with them.
/// Nothing below knows what year it is.

/// The anchor economy's productivity trend, against which everyone else's
/// currency appreciates or does not.
const NEUTRAL_TFP: f64 = 0.013;

/// One country as the market sees it this month, before anyone acts on it.
struct Assessment {
    id: NationId,
    gdp: f64,
    infl: f64,
    rate: f64,
    growth: f64,
    system: EconomySystem,
    auth: f64,
    stab: f64,
    dev: f64,
    /// How far the rate sits above where the fundamentals put it.
    misalign: f64,
    /// Current account, annual share of GDP; positive is a surplus.
    ca: f64,
    cover: f64,
    credibility: f64,
    /// Appetite for this country's paper, before it is compared with anyone
    /// else's.
    pull: f64,
    /// The balance sheet with the month's repricing already in it.
    f: Finance,
}

pub fn tick(w: &mut WorldState) {
    admit_newcomers(w);

    let (anchor_rate, anchor_infl) = anchor(w);
    let oil_price = w.oil_price;
    let vol = w.rules.financial_volatility;
    let ids: Vec<NationId> = w.nations.iter().filter(|n| n.alive).map(|n| n.id).collect();

    // First pass: reprice every currency and work out how attractive each
    // country looks, without touching anything. Nothing here draws from the RNG.
    let mut assessed: Vec<Assessment> = Vec::with_capacity(ids.len());
    for id in ids.iter().copied() {
        assessed.push(assess(w, id, anchor_rate, anchor_infl, oil_price));
    }

    // Money that leaves one country arrives in another. Appetite is therefore
    // relative: what matters is not whether a country looks attractive but
    // whether it looks more attractive than the alternatives, which is why a
    // global panic still leaves the safest borrower with money pouring in.
    let world_gdp: f64 = assessed.iter().map(|a| a.gdp).sum();
    let mean_pull: f64 = if world_gdp > 0.0 {
        assessed.iter().map(|a| a.pull * a.gdp).sum::<f64>() / world_gdp
    } else {
        0.0
    };

    // Breaks are collected and their fallout applied after everyone has been
    // assessed, so that within a month the market judges all countries on the
    // same information rather than on roster order.
    let mut breaks: Vec<(NationId, f64)> = vec![];

    for a in assessed {
        let Assessment {
            id, gdp, infl, rate, growth, system, auth, stab, dev,
            misalign, ca, cover, credibility, pull, mut f,
        } = a;

        // Two draws per nation per month, always in the same order, whatever the
        // nation turns out to do with them.
        let flow_noise = w.rng.range(-1.0, 1.0);
        let roll = w.rng.f64();

        let flow = f.capital_openness * ((pull - mean_pull * 0.8) * vol + flow_noise * 0.018 * vol);

        // The stock moves in slowly and, short of a break, leaves only a third
        // at a time. A break is what makes it leave all at once.
        let d = {
            let raw = flow / 12.0;
            if raw < 0.0 { raw.max(-f.hot_money_gdp * 0.30) } else { raw }
        };
        f.hot_money_gdp = (f.hot_money_gdp + d).clamp(0.0, 0.60);
        // Hot money arrives as foreign-currency bank credit and is repaid far
        // more slowly than it arrived. The ratchet is the whole danger.
        f.fx_debt_gdp = (f.fx_debt_gdp + d.max(0.0) * 0.70 + d.min(0.0) * 0.25).clamp(0.0, 3.0);

        // ---- Who absorbs the net position ----
        let fx_before = f.fx_index;
        let net = ca + flow;
        let net_bn = net * gdp / 12.0;
        match f.stance {
            FxStance::Pegged => {
                // The rate does not move, so the reserves take all of it.
                f.fx_index = f.peg_rate;
                f.reserves += net_bn;
            }
            FxStance::Managed => {
                // A crawl bleeds off part of the pressure the reserves would
                // otherwise have to carry, which is why managed floats rarely
                // end as violently as hard pegs.
                f.reserves += net_bn * 0.55;
                f.fx_index += (f.fair_index - f.fx_index) * 0.035;
                f.peg_rate = f.fx_index;
            }
            FxStance::Floating => {
                // A floating central bank leans against an inflow — the reserves
                // it buys on the way up are what it will wish it had later — but
                // it has nothing to defend on the way down, so the rate takes
                // most of the strain instead.
                let ca_bn = ca * gdp / 12.0;
                f.reserves += d * gdp * 0.45
                    + if ca_bn > 0.0 { ca_bn * 0.45 } else { ca_bn * 0.10 };
                f.fx_index += (f.fair_index - f.fx_index) * 0.14;
                f.fx_index *= 1.0 + net.clamp(-0.20, 0.20) * 0.25 - f.risk * 0.015 * vol;
            }
        }
        f.fx_index = f.fx_index.max(1.0e-12);
        f.reserves = f.reserves.max(0.0);

        // How far the rate moved this month, taken before any zeroes are struck
        // off the currency below.
        let depreciation = (1.0 - f.fx_index / fx_before).clamp(0.0, 0.5);

        // A currency that has lost four zeroes gets them struck off, as every
        // hyperinflation eventually does. Every ratio here is unaffected; the
        // numbers just stop being absurd.
        let mut redenominated = false;
        if f.fx_index < 1.0e-4 {
            f.fx_index *= 1.0e4;
            f.fair_index *= 1.0e4;
            f.peg_rate *= 1.0e4;
            redenominated = true;
        }

        // ---- What the promise costs ----
        // A peg imports the anchor's price level — that is what it is bought for
        // — but only as far as anyone believes it. A floating currency imports
        // its own depreciation instead.
        let mut new_infl = infl;
        if f.stance == FxStance::Floating {
            new_infl += depreciation * 0.35;
        } else {
            new_infl += (anchor_infl - infl) * 0.05 * credibility;
            // Under a peg the central bank has to buy every dollar that arrives,
            // and it pays for them with money it prints. Sterilisation is never
            // complete, so a boom financed from abroad shows up as inflation —
            // which is precisely what makes the fixed rate overvalued.
            new_infl += d.max(0.0) * 0.60;
        }

        // Defending a rate means making it expensive to be short the currency.
        // It works, and it puts the domestic economy through a wringer, because
        // the same rate is charged to everyone at home.
        let mut new_rate = rate;
        if f.stance != FxStance::Floating && net < 0.0 {
            let defence =
                (anchor_rate + infl.max(0.0) + (-net).min(0.20) * 1.6 + f.risk * 0.15).min(0.60);
            if defence > rate {
                new_rate = rate + (defence - rate) * 0.35;
            }
        }

        // ---- Debt you did not print, and a currency that sells ----
        let ext_service = f.fx_debt_gdp * (anchor_rate + f.risk * 0.12);
        let service_drag = ((ext_service - 0.015).max(0.0) * 0.55).min(0.09);
        let export_bonus = (-misalign).max(0.0).min(0.5) * 0.016;

        // Foreign debt is repaid slowly and shrinks against a growing economy —
        // but only against half of the growth, because a boom is exactly when
        // more of it gets taken on.
        f.fx_debt_gdp = (f.fx_debt_gdp * (1.0 - 0.001) / (1.0 + growth / 24.0)).clamp(0.0, 3.0);
        // Debt past any prospect of repayment gets written down, slowly and
        // grudgingly, because the alternative is never being paid at all. Poland
        // had half of its foreign debt forgiven in 1991 on exactly that logic.
        // Without this the arithmetic has no exit: each devaluation enlarges the
        // debt, the debt shrinks the economy, and the ratio compounds forever.
        f.fx_debt_gdp -= (f.fx_debt_gdp - 0.9).max(0.0) * 0.012;

        // ---- Risk reprices ----
        // Panic decays; fundamentals do not. Between crises this pulls back
        // toward what the balance sheet deserves.
        //
        // Reserves are only a fundamental if there is a promise standing on them
        // — nobody asks a floating currency what it is backed by — and even then
        // the market does not do the arithmetic until it has a reason to. A peg
        // that has held is taken as evidence that it will hold, so cover has to
        // get genuinely thin before anyone charges for it. That indulgence is
        // what lets the exposure build to the size that makes the reckoning
        // worth having.
        let reserve_risk = if f.stance == FxStance::Floating {
            0.0
        } else {
            (0.55 - cover).max(0.0) * 0.55
        };
        // Nobody borrows for free: even the best credit carries a floor.
        let fundamental = (0.05
            + (f.fx_debt_gdp - 0.25).max(0.0) * 0.55
            + reserve_risk
            + misalign.max(0.0) * 0.18
            + (55.0 - stab).max(0.0) * 0.006)
            .clamp(0.0, 1.0);
        f.risk = (f.risk * 0.93 + fundamental * 0.07).clamp(0.0, 1.0);

        // ---- Capital accounts open as countries get rich and answerable ----
        // Which is to say: a country qualifies for the money that can ruin it by
        // succeeding. A command economy simply does not let it in.
        let target_open = if system == EconomySystem::Command {
            0.06
        } else {
            (0.30 + 0.55 * dev + 0.25 * (1.0 - auth)).min(1.0)
        };
        f.capital_openness += (target_open - f.capital_openness) * 0.008;

        if f.months_since_break >= 0 {
            f.months_since_break += 1;
        }

        // ---- Does the promise hold ----
        let mut broke = false;
        let mut adopted: Option<&'static str> = None;
        if f.stance != FxStance::Floating {
            if f.reserves <= gdp * 0.002 && net < 0.0 {
                broke = true; // nothing left to defend with
            } else {
                // What a speculator is looking at: whether the reserves can
                // cover what could be demanded of them, how far the rate is from
                // where it belongs, what the rest of the market already thinks,
                // and how much money is sitting there ready to go first.
                let vuln = (0.75 - cover).max(0.0) * 1.2
                    + misalign.max(0.0) * 0.55
                    + f.risk * 0.90
                    + (f.hot_money_gdp * 1.6).min(0.45);
                let p = ((vuln - 0.55).max(0.0).powi(2) * 0.22 * vol).min(0.45);
                broke = roll < p;
            }
        } else if (f.months_since_break < 0 || f.months_since_break > 36) && cover > 0.9 {
            // Two reasons a government reaches for someone else's currency, and
            // both of them are good reasons at the time.
            let to_kill_inflation = new_infl > 0.12;
            // The developing-country trade: a fixed rate is a promise to foreign
            // lenders, and the promise is worth several points off the cost of
            // capital. Every fast-growing economy that could take that trade
            // took it, and the money it brought in is what there was to lose.
            let to_borrow_credibility = dev < 0.55 && growth > 0.035 && new_infl < 0.10;
            if to_kill_inflation && roll < 0.035 * vol {
                adopted = Some("to break its inflation");
            } else if to_borrow_credibility && roll < 0.020 * vol {
                adopted = Some("to bring the world's money in");
            }
            if adopted.is_some() {
                f.stance = FxStance::Managed;
                f.peg_rate = f.fx_index;
            }
        }

        *w.fin_mut(id) = f;
        {
            let n = w.nation_mut(id);
            n.inflation = new_infl.clamp(-0.05, 3.0);
            n.interest_rate = new_rate.clamp(0.0, 0.60);
            n.gdp *= 1.0 + (export_bonus - service_drag) / 12.0;
            n.priv_invest_gdp = (n.priv_invest_gdp + d * 0.30).clamp(0.02, 0.45);
            // Cheap foreign credit inflates whatever it is lent against. It only
            // adds to a bubble that is still inflating; it cannot undo a bust.
            if d > 0.0 && n.bubble >= 0.0 {
                n.bubble = (n.bubble + d * 2.5).min(1.0);
            }
        }

        if redenominated {
            w.headline(format!(
                "{} strikes four zeroes from the {}.",
                id.name(),
                id.currency()
            ));
        }
        if let Some(reason) = adopted {
            w.headline(format!(
                "{} anchors the {} to the dollar {}.",
                id.name(),
                id.currency(),
                reason
            ));
        }
        if broke {
            let severity = devalue(w, id);
            breaks.push((id, severity));
        }
    }

    for (id, severity) in breaks {
        contagion(w, id, severity);
    }
}

/// Reprice one currency and one balance sheet. Pure: it reads the world and
/// returns what the market makes of this country, changing nothing.
fn assess(
    w: &WorldState,
    id: NationId,
    anchor_rate: f64,
    anchor_infl: f64,
    oil_price: f64,
) -> Assessment {
    let n = w.nation(id);
    let (gdp, infl, rate, tfp, growth, oil_mbd, system, auth, stab) = (
        n.gdp, n.inflation, n.interest_rate, n.tfp_trend, n.growth_last,
        n.oil_mbd, n.system, n.authoritarianism, n.stability,
    );
    let dev = (gdp * 1000.0 / n.population / 24000.0).min(1.0);
    let mut f = w.fin(id).clone();

    // ---- What the currency is actually worth ----
    // Relative inflation is most of it: a currency losing purchasing power twice
    // as fast as the anchor's has to buy less of it. Productivity pushes the
    // other way, and so does an oil field.
    let infl_gap = infl - anchor_infl;
    let prod_gap = tfp - NEUTRAL_TFP;
    // Terms of trade, measured against a $20 barrel so that an ordinary oil price
    // leaves an exporter's currency where it was. A boom lifts it; a glut takes
    // it away again.
    let oil_share = (oil_mbd * oil_price * 0.365 / gdp).min(0.5);
    let oil_tot = if oil_mbd > 0.5 {
        (oil_price - 20.0) / 20.0 * oil_share * 0.5
    } else {
        -(oil_price - 20.0) / 20.0 * 0.004
    };
    let drift = (1.0 - (infl_gap - prod_gap * 0.4 - oil_tot * 0.5) / 12.0).clamp(0.60, 1.02);
    f.fair_index = (f.fair_index * drift).max(1.0e-12);

    // Positive means the currency is dearer than the fundamentals justify. Under
    // a peg this number grows on its own, doing nothing, until it is large enough
    // to be worth attacking.
    let misalign = (f.fx_index / f.fair_index - 1.0).clamp(-0.9, 4.0);

    // ---- The current account ----
    // An overvalued currency prices its exports out of the market, and an
    // investment boom is mostly imported machinery — which is why the fastest
    // growing economies run the largest deficits and are the most dependent on
    // the money continuing to arrive. Capped, because a country cannot run an
    // arbitrarily large deficit: it runs out of financing first, and running out
    // of financing is the crisis.
    let ca = (0.012 - misalign.max(-0.4) * 0.055 - (growth - 0.025).max(0.0) * 0.55
        + oil_tot * 0.30)
        .clamp(-0.12, 0.10);

    // ---- What the money thinks ----
    // It chases the carry and the growth story and runs from risk. What makes a
    // peg so attractive to it is that a credible peg promises the carry will not
    // be taken back by the exchange rate — so the more reserves a central bank
    // has, the cheaper it is to borrow against it, and the more there is to run
    // when the reserves turn out to be finite.
    let cover = f.reserve_cover(gdp);
    let credibility = (cover / 1.5).min(1.0) * (1.0 - f.risk);
    let expected_dep = match f.stance {
        FxStance::Floating => infl_gap.max(0.0),
        _ => (1.0 - credibility) * misalign.max(0.0) * 0.6,
    };
    let carry = (rate - anchor_rate - expected_dep).clamp(-0.4, 0.4);
    // Appetite falls as the position gets crowded: there is only so much of one
    // country a portfolio will hold, however good the story.
    let crowding = (f.hot_money_gdp - 0.10).max(0.0) * 0.35;
    // A mild recession in a rich country is not a reason to leave, so the growth
    // term is bounded below; a boom is very much a reason to arrive.
    let pull = carry * 0.85 + (growth - 0.025).clamp(-0.04, 0.08) * 0.9
        - (f.risk - 0.10) * 0.45
        - crowding;

    Assessment {
        id, gdp, infl, rate, growth, system, auth, stab, dev,
        misalign, ca, cover, credibility, pull, f,
    }
}

/// Break the rate and let the news travel. A government that jumps before it is
/// pushed still tells the market the same thing about everyone who looks like it.
pub fn break_peg(w: &mut WorldState, id: NationId) {
    let severity = devalue(w, id);
    contagion(w, id, severity);
}

/// Break the rate. This is the discrete, violent event the rest of the module
/// exists to lead up to and to propagate.
///
/// The devaluation itself is the small part. The large part is that every
/// foreign-currency liability on every balance sheet in the country has just
/// grown by the size of the devaluation, measured in the money that has to
/// service it, while the assets behind those liabilities have not.
pub fn devalue(w: &mut WorldState, id: NationId) -> f64 {
    let vol = w.rules.financial_volatility;
    let extra = w.rng.range(0.0, 0.14);
    let gdp = w.nation(id).gdp;
    let mut f = w.fin(id).clone();

    let misalign = (f.fx_index / f.fair_index - 1.0).max(0.0);
    // How far it falls is how far it was held up.
    let severity = ((0.16 + misalign * 0.42 + f.risk * 0.22 + extra) * vol).clamp(0.12, 0.70);

    f.fx_index *= 1.0 - severity;
    f.peg_rate = f.fx_index;
    f.stance = FxStance::Floating;
    f.months_since_break = 0;
    f.risk = (f.risk + 0.30).min(1.0);

    // The money that was here on the strength of the promise leaves within the
    // month, and takes what is left of the reserves with it.
    let flight = f.hot_money_gdp * 0.70;
    f.hot_money_gdp -= flight;
    f.reserves = (f.reserves - flight * gdp * 0.45).max(0.0);

    // The transmission channel.
    let before = f.fx_debt_gdp;
    f.fx_debt_gdp = (before / (1.0 - severity)).min(3.0);
    let blowup = f.fx_debt_gdp - before;

    // Firms and banks whose debts just doubled stop lending, hiring and
    // investing, all at once and all in the same quarter.
    let hit = (blowup * 0.40 + severity * 0.045 + flight * 0.35).min(0.24);
    f.fx_debt_gdp = (f.fx_debt_gdp / (1.0 - hit)).min(3.0);

    *w.fin_mut(id) = f;
    {
        let n = w.nation_mut(id);
        n.gdp *= 1.0 - hit;
        // Import prices are the first thing to move, and they move immediately.
        n.inflation = (n.inflation + severity * 0.45).clamp(-0.05, 3.0);
        n.priv_invest_gdp = (n.priv_invest_gdp * (1.0 - (hit * 2.0).min(0.5))).max(0.02);
        // A hole in the banking system is a drag that outlasts the crisis by a
        // decade — the same machinery Japan's bubble leaves behind.
        n.bubble = (n.bubble - (blowup * 1.6 + severity * 0.5).min(1.6)).max(-1.6);
        // Recapitalising the banks, and a smaller economy to carry the old debt.
        n.debt_gdp = (n.debt_gdp / (1.0 - hit) + blowup * 0.35).min(2.5);
        n.stability = (n.stability - (7.0 + blowup * 28.0 + flight * 40.0).min(26.0)).max(0.0);
    }

    w.headline(format!(
        "CURRENCY CRISIS: the {} falls {:.0}% — {} abandons the peg.",
        id.currency(),
        severity * 100.0,
        id.name()
    ));
    if blowup > 0.08 {
        w.headline(format!(
            "{}'s foreign-currency debts grew by {:.0}% of national income overnight; the banks are insolvent.",
            id.name(),
            blowup * 100.0
        ));
    }
    severity
}

/// A devaluation is information about more than one country. The market does not
/// re-examine the country that broke — it already knows about that one — it
/// re-prices everyone who looked like it, and the resemblance that matters is
/// the shape of the balance sheet, not the trade relationship.
fn contagion(w: &mut WorldState, source: NationId, severity: f64) {
    let vol = w.rules.financial_volatility;
    let src = w.fin(source).clone();
    let src_growth = w.nation(source).growth_last;
    let ids: Vec<NationId> = w
        .nations
        .iter()
        .filter(|n| n.alive && n.id != source)
        .map(|n| n.id)
        .collect();

    let mut worst: Option<(NationId, f64)> = None;
    for id in ids {
        let (gdp, growth) = {
            let n = w.nation(id);
            (n.gdp, n.growth_last)
        };
        let sim = resemblance(&src, source, src_growth, w.fin(id), id, growth);
        let bump = sim * severity * 1.35 * vol;
        if bump < 0.01 {
            continue;
        }
        let f = w.fin_mut(id);
        f.risk = (f.risk + bump).min(1.0);
        // The reassessment is not only a price. Some of the money actually goes,
        // and it goes before anyone has established whether it needed to.
        let run = (f.hot_money_gdp * bump * 1.2).min(f.hot_money_gdp);
        f.hot_money_gdp -= run;
        f.reserves = (f.reserves - run * gdp * 0.5).max(0.0);
        if worst.map_or(true, |(_, b)| bump > b) {
            worst = Some((id, bump));
        }
    }

    if let Some((id, bump)) = worst {
        if bump > 0.12 {
            w.headline(format!(
                "Money flees {} on the strength of {}'s devaluation alone.",
                id.name(),
                source.name()
            ));
        }
    }
}

/// How much a devaluation in `a` is treated as news about `b`, 0..1.
fn resemblance(
    a: &Finance,
    a_id: NationId,
    a_growth: f64,
    b: &Finance,
    b_id: NationId,
    b_growth: f64,
) -> f64 {
    // A devaluation anywhere is a reminder that a peg is an opinion.
    let mut s = 0.06;
    if a.stance != FxStance::Floating && b.stance != FxStance::Floating {
        s += 0.28; // the same trick, and therefore the same trap
    }
    if a_id.region() == b_id.region() {
        s += 0.24;
    }
    // The comparison that actually gets made in a dealing room: how much of the
    // debt is in money the borrower cannot print.
    s += (a.fx_debt_gdp.min(b.fx_debt_gdp) * 1.1).min(0.24);
    s += (a.hot_money_gdp.min(b.hot_money_gdp) * 2.2).min(0.20);
    // Two economies that were sold to investors as the same story.
    if a_growth > 0.045 && b_growth > 0.045 {
        s += 0.12;
    }
    // A closed capital account is a firewall, because there is nobody to run.
    // Not a perfect one — trade finance and bank lines get pulled anyway — but
    // it is the difference between a bad year and a lost decade.
    s * (0.20 + 0.80 * b.capital_openness)
}

/// The reserve currency sets the world's risk-free rate and the price level every
/// other peg is measured against. If it is ever gone, the largest market economy
/// left inherits the job.
fn anchor(w: &WorldState) -> (f64, f64) {
    if let Some(n) = w.nation_opt(NationId::USA).filter(|n| n.alive) {
        return (n.interest_rate, n.inflation);
    }
    let mut best: Option<&Nation> = None;
    for n in w
        .nations
        .iter()
        .filter(|n| n.alive && n.system == EconomySystem::Market)
    {
        if best.map_or(true, |b| n.gdp > b.gdp) {
            best = Some(n);
        }
    }
    best.map(|n| (n.interest_rate, n.inflation)).unwrap_or((0.05, 0.03))
}

/// States born out of a dissolution need a balance sheet before they can have a
/// crisis. They get one here rather than in the dissolution code so that any
/// future successor is covered without that code knowing finance exists.
fn admit_newcomers(w: &mut WorldState) {
    let missing: Vec<NationId> = w
        .nations
        .iter()
        .filter(|n| n.alive)
        .map(|n| n.id)
        .filter(|id| w.fin_opt(*id).is_none())
        .collect();
    for id in missing {
        let f = birth_profile(w, id);
        w.finance.push(f);
    }
}

/// A successor inherits its predecessor's creditors and almost none of its
/// reserves. Its currency starts at par because it is a new currency — the
/// rouble, the tolar, the kuna were all issued at one to one and then found out
/// what they were worth.
fn birth_profile(w: &WorldState, id: NationId) -> Finance {
    let gdp = w.nation(id).gdp;
    let (parent, debt_mult, open, risk) = match id {
        // Russia assumed the whole Soviet external debt under the 1993 "zero
        // option" while holding a bit over half the output, so the ratio it
        // inherits is roughly double the union's.
        NationId::Russia => (Some(NationId::USSR), 1.9, 0.50, 0.65),
        // The Yugoslav successors spent years arguing over the federal debt and
        // each ended up carrying roughly its own share of it.
        NationId::Serbia | NationId::Croatia | NationId::Slovenia | NationId::Bosnia => {
            (Some(NationId::Yugoslavia), 1.0, 0.35, 0.55)
        }
        _ => (None, 1.0, 0.50, 0.30),
    };
    let fx_debt = parent
        .and_then(|p| w.fin_opt(p))
        .map(|p| p.fx_debt_gdp * debt_mult)
        .unwrap_or(0.25);
    Finance {
        id,
        stance: FxStance::Floating,
        fx_index: 1.0,
        peg_rate: 1.0,
        fair_index: 1.0,
        reserves: gdp * 0.02,
        fx_debt_gdp: fx_debt.min(3.0),
        hot_money_gdp: 0.0,
        capital_openness: open,
        risk,
        months_since_break: -1,
    }
}
