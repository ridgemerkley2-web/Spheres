//! Reporting invariants, not calibration: opening cash, appropriations and the
//! one supplier plant/warehouse are disclosed synthetic fixture endowments.
use journal::{CashCause as C, SupplierInclusion as S};
use spheres_sim::world::{
    GameRules, NationId as N, WorldState, BUDGET_DEFENSE as D, BUDGET_INDUSTRY as I,
};
use spheres_sim::{
    apply_command, clock, commerce, companies, economy, fiscal_journal as journal,
    fiscal_recovery as fiscal, init::world_1990, load, logistics, production, programs, resources,
    save, tick_day, Command,
};
const HOME: N = N::France;
fn near(a: f64, b: f64) {
    assert!((a - b).abs() < 1e-10, "{a:.15} != {b:.15}");
}
fn cash(w: &WorldState, n: N) -> (f64, f64) {
    let n = w.nation(n);
    (n.treasury_bn.unwrap(), n.debt_bn.unwrap())
}
fn base() -> WorldState {
    let mut w = world_1990(GameRules {
        daily_simulation: true,
        ai_aggression: 0.0,
        crisis_intensity: 0.0,
        ..Default::default()
    });
    w.player = Some(HOME);
    let n = w.nation_mut(HOME);
    n.treasury_bn = Some(20.0);
    n.debt_bn = Some(80.0);
    n.debt_gdp = 80.0 / n.gdp;
    n.political_capital = 1000.0;
    fiscal::enable(&mut w);
    w
}
fn strip_journals(w: &mut WorldState) {
    for f in w.fiscal_recovery.nations.values_mut() {
        f.money_journal = None;
    }
}

#[test]
fn s06_old_saves_views_and_enrollment_do_not_backfill_money_history() {
    let mut w = base();
    let before = save(&w);
    assert!(journal::view(&w, HOME).is_none());
    fiscal::prepare(&mut w);
    fiscal::validate(&w).unwrap();
    assert_eq!(save(&w), before);
    assert_eq!(save(&load(&before).unwrap()), before);
    let mut old = world_1990(GameRules::default());
    let old_save = save(&old);
    assert!(journal::view(&old, HOME).is_none());
    fiscal::prepare(&mut old);
    fiscal::tick(&mut old);
    assert_eq!(save(&old), old_save);
}

#[test]
fn s06_reporting_preserves_each_existing_cash_and_debt_operation_bit_for_bit() {
    let mut observed = base();
    let mut plain = observed.clone();
    plain.rules.fiscal_recovery = false;
    let sequence = [
        (C::ResourceMarket, 5.0),
        (C::GoodsImportEscrow, 25.0),
        (C::GoodsRefund, -4.0),
        (C::SupplierInputs, -90.0),
        (C::PactUpkeep, 0.25),
        (C::ForeignAid, 0.75),
        (C::RefitRefund, -10.0),
        (C::AssetSale, -200.0),
    ];
    for (cause, amount) in sequence {
        let share = amount / observed.nation(HOME).gdp;
        let expected = economy::charge(&mut plain, HOME, amount, share);
        let actual = economy::charge_for(&mut observed, HOME, amount, share, cause);
        assert_eq!(actual.to_bits(), expected.to_bits());
        assert_eq!(cash(&observed, HOME), cash(&plain, HOME));
        assert_eq!(observed.rng, plain.rng);
    }
    let report = journal::view(&observed, HOME).unwrap();
    assert_eq!(report.days.len(), 1);
    assert_eq!(report.days[0].flows.len(), sequence.len());
    near(report.days[0].unrecorded_cash_delta_bn, 0.0);
    near(report.days[0].unrecorded_debt_delta_bn, 0.0);
    fiscal::validate(&observed).unwrap();
    plain.rules.fiscal_recovery = true;
    strip_journals(&mut observed);
    assert_eq!(
        save(&observed),
        save(&plain),
        "only observational journal fields may differ"
    );
}

#[test]
fn s06_real_ordinary_settlement_reports_signed_interest_and_no_second_charge() {
    let mut w = base();
    w.nation_mut(HOME).interest_rate = 0.0;
    w.nation_mut(HOME).inflation = 0.05;
    let before = cash(&w, HOME);
    economy::tick(&mut w);
    let after = cash(&w, HOME);
    let report = journal::view(&w, HOME).unwrap();
    let day = report.days.last().unwrap();
    let detail = day.fiscal.as_ref().unwrap();
    assert!(detail.interest_bn < 0.0);
    let flow = &day.flows[&C::BudgetSettlement];
    near(flow.treasury_delta_bn, after.0 - before.0);
    near(flow.debt_delta_bn, after.1 - before.1);
    near(
        (after.0 - after.1) - (before.0 - before.1),
        detail.revenue_bn - detail.spending_bn - detail.interest_bn,
    );
    let paid = save(&w);
    let mut resumed = load(&paid).unwrap();
    fiscal::tick(&mut w);
    fiscal::tick(&mut resumed);
    assert_eq!(cash(&w, HOME), after);
    assert_eq!(save(&w), save(&resumed));
    let saved = save(&w);
    journal::view(&w, HOME);
    fiscal::validate(&w).unwrap();
    assert_eq!(save(&w), saved);
}

fn funded_company(prepaid: bool) -> WorldState {
    let mut w = world_1990(GameRules {
        daily_simulation: true,
        military_operations: true,
        production_system: true,
        manufacturing_system: true,
        resource_market: true,
        ai_aggression: 0.0,
        crisis_intensity: 0.0,
        ..Default::default()
    });
    w.player = Some(HOME);
    w.conflicts.clear();
    w.nation_mut(HOME).political_capital = 1000.0;
    resources::tick(&mut w);
    programs::set_construction_budget(&mut w, HOME, 0.2).unwrap();
    programs::begin_day(&mut w);
    let district = w
        .districts
        .iter()
        .find(|(_, n)| **n == HOME)
        .unwrap()
        .0
        .clone();
    if let Some(site) = w
        .production
        .provinces
        .iter_mut()
        .find(|p| p.district == district)
    {
        site.arms_plants = 1;
    } else {
        w.production
            .provinces
            .push(production::ProvinceCapabilities {
                district: district.clone(),
                arms_plants: 1,
                infrastructure: 0,
                civilian_industry: 0,
                power_grid: 0,
                research_centers: 0,
            });
        w.production
            .provinces
            .sort_by(|a, b| a.district.cmp(&b.district));
    }
    let n = w.nation_mut(HOME);
    n.treasury_bn = Some(20.0);
    n.debt_bn = Some(80.0);
    n.debt_gdp = 80.0 / n.gdp;
    n.arsenal.held.clear();
    n.arsenal.orders.clear();
    n.arsenal.banked = 0.0;
    let p = n.program_budget.as_mut().unwrap();
    p.available_bn = programs::ZERO;
    p.prepaid_bn = programs::ZERO;
    p.spent_today_bn = programs::ZERO;
    p.prepaid_used_today_bn = programs::ZERO;
    p.noncapital_spent_today_bn = programs::ZERO;
    p.available_bn[I][0] = 0.2;
    if prepaid {
        p.prepaid_bn[D][3] = 1.0;
    } else {
        p.available_bn[D][3] = 1.0;
    }
    p.revenue_today_bn = 0.3;
    p.interest_today_bn = 0.1;
    p.fiscal_staged = true;
    fiscal::enable(&mut w);
    programs::spend_construction(&mut w, HOME, 0.2).unwrap();
    let q = companies::establishment_quote(&w, HOME, "S06 Receipt Works", &district, 1.0);
    assert!(q.valid, "{:?}", q.reason);
    apply_command(
        &mut w,
        &Command::Company {
            nation: HOME,
            order: companies::CompanyOrder::Establish {
                name: "S06 Receipt Works".into(),
                district,
                capitalization_bn: 1.0,
                quote: q.token,
            },
        },
    )
    .unwrap();
    w
}
#[test]
fn s06_company_receipts_are_budget_inclusions_and_prepaid_money_is_not_paid_twice() {
    for prepaid in [false, true] {
        let mut w = funded_company(prepaid);
        let before = cash(&w, HOME);
        assert!(
            journal::view(&w, HOME).is_none(),
            "approved supplier authority is not paid public money"
        );
        assert_eq!(w.companies.firms[0].cash_bn, 0.0);
        let mut resumed = load(&save(&w)).unwrap();
        for world in [&mut w, &mut resumed] {
            programs::finish_day(world);
            companies::settle_receivables(world);
            fiscal::tick(world);
        }
        assert_eq!(save(&w), save(&resumed));
        let v = journal::view(&w, HOME).unwrap();
        let row = v.days.last().unwrap();
        let detail = row.fiscal.as_ref().unwrap();
        near(detail.construction_bn, 0.2);
        near(detail.spending_bn, if prepaid { 0.2 } else { 1.2 });
        near(detail.prepaid_used_bn, if prepaid { 1.0 } else { 0.0 });
        near(detail.supplier_inclusions_bn[&S::Capitalization], 1.0);
        assert_eq!(row.flows.len(), 1);
        assert_eq!(row.flows[&C::BudgetSettlement].postings, 1);
        let after = cash(&w, HOME);
        near(
            after.0 - after.1,
            before.0 - before.1 - detail.spending_bn - 0.1 + 0.3,
        );
        near(w.companies.firms[0].cash_bn, 1.0);
        let settled = save(&w);
        programs::finish_day(&mut w);
        companies::settle_receivables(&mut w);
        fiscal::tick(&mut w);
        assert_eq!(
            save(&w),
            settled,
            "repeated settlement cannot duplicate a supplier inclusion"
        );
        fiscal::validate(&w).unwrap();
    }
}

#[test]
fn s06_goods_escrow_refund_and_dispatch_receipts_have_their_actual_cash_dates() {
    let mut w = world_1990(GameRules {
        daily_simulation: true,
        economic_competition: true,
        production_system: true,
        resource_market: true,
        resource_gates: true,
        logistics_routes: true,
        physical_logistics: true,
        ..Default::default()
    });
    let buyer = N::Belgium;
    let seller = N::Netherlands;
    for id in [buyer, seller] {
        let n = w.nation_mut(id);
        n.treasury_bn = Some(1.0);
        n.debt_bn = Some(0.5);
        n.debt_gdp = 0.5 / n.gdp;
    }
    w.production
        .industry
        .goods
        .entry(seller)
        .or_default()
        .intermediates = 100.0;
    commerce::set_sale(
        &mut w,
        seller,
        commerce::Good::Intermediates,
        10.0,
        1.0,
        true,
    )
    .unwrap();
    fiscal::enable(&mut w);
    let price = commerce::reference_price_bn(commerce::Good::Intermediates);
    let proposal = commerce::propose(
        &mut w,
        buyer,
        seller,
        commerce::Good::Intermediates,
        10.0,
        price,
        30,
    )
    .unwrap();
    assert_eq!(proposal.kind, "contract");
    let total = price * 10.0;
    near(
        journal::view(&w, buyer).unwrap().days[0].flows[&C::GoodsImportEscrow].outflow_bn,
        total,
    );
    assert!(
        journal::view(&w, seller).is_none(),
        "reserved stock is not seller revenue"
    );
    commerce::cancel(&mut w, buyer, proposal.id).unwrap();
    let row = journal::view(&w, buyer).unwrap().days.remove(0);
    near(row.flows[&C::GoodsRefund].inflow_bn, total);
    near(row.flows[&C::GoodsRefund].treasury_delta_bn, 0.0);
    near(row.flows[&C::GoodsRefund].debt_delta_bn, -total);
    let saved = save(&w);
    assert!(commerce::cancel(&mut w, buyer, proposal.id).is_err());
    assert_eq!(save(&w), saved);
    let second = commerce::propose(
        &mut w,
        buyer,
        seller,
        commerce::Good::Intermediates,
        10.0,
        price,
        30,
    )
    .unwrap();
    logistics::begin_month(&mut w);
    commerce::tick_day(&mut w);
    let ledger = w.commerce.as_ref().unwrap();
    let received = ledger.accounts[&seller].exports_received_bn;
    assert!(received > 0.0);
    assert!(ledger.cargo.iter().any(|c| c.contract == second.id));
    near(
        journal::view(&w, seller).unwrap().days[0].flows[&C::GoodsExportReceipt].inflow_bn,
        received,
    );
    assert_eq!(save(&load(&save(&w)).unwrap()), save(&w));
}

#[test]
fn s06_unattributed_stock_changes_are_visible_and_never_repaired_by_reading() {
    let mut w = base();
    economy::charge_for(&mut w, HOME, 1.0, 1.0 / 1190.0, C::PactUpkeep);
    *w.nation_mut(HOME).treasury_bn.as_mut().unwrap() += 1.5; // Explicit unrelated balance mutation.
    let saved = save(&w);
    near(
        journal::view(&w, HOME).unwrap().unrecorded_cash_delta_bn,
        1.5,
    );
    fiscal::validate(&w).unwrap();
    assert_eq!(save(&w), saved);
    assert_eq!(save(&load(&saved).unwrap()), saved);
    economy::charge_for(&mut w, HOME, 0.5, 0.5 / 1190.0, C::ForeignAid);
    let v = journal::view(&w, HOME).unwrap();
    near(v.days[0].unrecorded_cash_delta_bn, 1.5);
    near(v.unrecorded_cash_delta_bn, 0.0);
    near(
        v.days[0].closing_treasury_bn - v.days[0].opening_treasury_bn,
        0.0,
    );
    fiscal::validate(&w).unwrap();
}

#[test]
fn s06_policy_and_real_later_receipts_survive_reload_without_driving_the_simulation() {
    let mut w = base();
    // Disclosed distressed starting finances; the baseline receipt and all
    // later exposures are produced by the real daily simulation.
    let n = w.nation_mut(HOME);
    n.treasury_bn = Some(0.0);
    n.debt_bn = Some(n.gdp * 2.0);
    n.debt_gdp = 2.0;
    n.tax_rate = 0.08;
    n.interest_rate = 0.18;
    for _ in 0..31 {
        tick_day(&mut w, &[]);
    }
    let baseline = w.fiscal_recovery.nations[&HOME]
        .observations
        .last()
        .unwrap()
        .clone();
    assert!(fiscal::assessment(&w, HOME).unwrap().recovery_required);
    let pc = w.nation(HOME).political_capital;
    apply_command(
        &mut w,
        &Command::EnactStratagem {
            nation: HOME,
            id: "austerity".into(),
        },
    )
    .unwrap();
    let decision = journal::view(&w, HOME)
        .unwrap()
        .decisions
        .last()
        .unwrap()
        .clone();
    assert_eq!(decision.kind, "fiscal_consolidation");
    assert_eq!(decision.baseline_observation, Some(baseline));
    assert_eq!(
        journal::view(&w, HOME).unwrap().decisions.len(),
        1,
        "internal policy dispatch must not create duplicate decisions"
    );
    assert_eq!(
        w.nation(HOME).political_capital,
        pc - 34.0,
        "one package pays its one political price"
    );
    assert!(decision.after.tax_rate > decision.before.tax_rate);
    assert!(decision.latest_outcome.is_none());
    assert_eq!(decision.before.treasury_bn, decision.after.treasury_bn);
    assert_eq!(decision.before.debt_bn, decision.after.debt_bn);
    let mut resumed = load(&save(&w)).unwrap();
    let mut no_history = w.clone();
    strip_journals(&mut no_history);
    for _ in 0..30 {
        tick_day(&mut w, &[]);
        tick_day(&mut resumed, &[]);
        tick_day(&mut no_history, &[]);
        assert_eq!(save(&w), save(&resumed));
        let mut stripped = w.clone();
        strip_journals(&mut stripped);
        strip_journals(&mut no_history);
        assert_eq!(
            save(&stripped),
            save(&no_history),
            "report retention cannot drive world or RNG outcomes"
        );
    }
    let view = journal::view(&w, HOME).unwrap();
    assert!(view.days.len() <= journal::RETAINED_DAYS);
    let d = view.decisions.iter().find(|d| d.id == decision.id).unwrap();
    assert!(d.latest_outcome.as_ref().unwrap().day > d.day);
    assert!(
        d.latest_observation.as_ref().unwrap().month
            > d.baseline_observation.as_ref().unwrap().month
    );
    assert_eq!(
        d.latest_observation.as_ref().unwrap(),
        w.fiscal_recovery.nations[&HOME]
            .observations
            .last()
            .unwrap()
    );
    let outcome = d.latest_outcome.as_ref().unwrap();
    assert_eq!((outcome.treasury_bn, outcome.debt_bn), cash(&w, HOME));
    assert!(d.latest_observation.as_ref().unwrap().year_fraction > 0.0);
    assert_eq!(d.before, decision.before);
    fiscal::validate(&w).unwrap();
}

#[test]
fn s06_refused_policy_is_atomic_and_bounded_history_rejects_forged_money() {
    let mut w = base();
    let saved = save(&w);
    let b = w.nation(HOME).budget_for(w.year);
    let stale_year = w.year - 1;
    assert!(apply_command(
        &mut w,
        &Command::SetAnnualBudget {
            nation: HOME,
            fiscal_year: stale_year,
            allocations: b.allocations
        }
    )
    .is_err());
    assert_eq!(save(&w), saved);
    for i in 0..14 {
        apply_command(
            &mut w,
            &Command::SetTaxRate {
                nation: HOME,
                rate: 0.25 + i as f64 * 0.001,
            },
        )
        .unwrap();
    }
    let view = journal::view(&w, HOME).unwrap();
    assert_eq!(view.decisions.len(), 12);
    assert_eq!(view.decisions[0].id, 3);
    let mut corrupted = w.clone();
    corrupted
        .fiscal_recovery
        .nations
        .get_mut(&HOME)
        .unwrap()
        .money_journal
        .as_mut()
        .unwrap()
        .days[0]
        .closing_treasury_bn += 1.0;
    assert!(fiscal::validate(&corrupted).is_err());
    assert!(load(&save(&corrupted)).is_err());
    let mut future = w.clone();
    let tomorrow = clock::absolute_day(&w) + 1;
    future
        .fiscal_recovery
        .nations
        .get_mut(&HOME)
        .unwrap()
        .money_journal
        .as_mut()
        .unwrap()
        .decisions[0]
        .day = tomorrow;
    assert!(fiscal::validate(&future).is_err());
    fiscal::validate(&w).unwrap();
}

#[test]
fn s06_tiny_real_payments_keep_stock_precision_without_accepting_forged_money() {
    let mut w = base();
    w.nation_mut(HOME).treasury_bn = Some(10_000.0);
    economy::charge_for(&mut w, HOME, 1e-8, 1e-8 / 1190.0, C::FreightService);
    let row = journal::view(&w, HOME).unwrap().days.remove(0);
    let fee = &row.flows[&C::FreightService];
    let residual = (fee.treasury_delta_bn + fee.outflow_bn).abs();
    assert!(
        residual
            > 128.0 * f64::EPSILON * (fee.treasury_delta_bn.abs() + fee.outflow_bn + 1.0) * 2.0,
        "fixture reproduces cancellation outside the former delta-only bound"
    );
    assert_eq!(save(&load(&save(&w)).unwrap()), save(&w));
    let mut corrupted = w.clone();
    corrupted
        .fiscal_recovery
        .nations
        .get_mut(&HOME)
        .unwrap()
        .money_journal
        .as_mut()
        .unwrap()
        .days[0]
        .flows
        .get_mut(&C::FreightService)
        .unwrap()
        .outflow_bn += 0.001;
    assert!(fiscal::validate(&corrupted).is_err());
}

#[test]
fn s06_unknown_owned_journal_fields_are_refused_at_each_nested_level() {
    let mut w = base();
    apply_command(
        &mut w,
        &Command::SetTaxRate {
            nation: HOME,
            rate: 0.31,
        },
    )
    .unwrap();
    tick_day(&mut w, &[]);
    let saved: serde_json::Value = serde_json::from_str(&save(&w)).unwrap();
    for suffix in [
        "",
        "/days/0",
        "/days/0/flows/budget_settlement",
        "/days/0/fiscal",
        "/decisions/0",
        "/decisions/0/before",
        "/decisions/0/after",
        "/decisions/0/latest_outcome",
    ] {
        let mut bad = saved.clone();
        let path = format!("/world/fiscal_recovery/nations/France/money_journal{suffix}");
        bad.pointer_mut(&path)
            .unwrap()
            .as_object_mut()
            .unwrap()
            .insert("unsupported_money_owner".into(), serde_json::json!(1));
        assert!(load(&bad.to_string()).is_err(), "must refuse {path}");
    }
    assert_eq!(save(&load(&save(&w)).unwrap()), save(&w));
}
