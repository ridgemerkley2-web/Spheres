// Included in military_ai::tests to reuse its explicitly endowed fixture and
// ordinarily developed/paid finished stock. These are command parity tests,
// not campaign or performance qualification evidence.
mod idle_import_tests {
    use super::*;

    fn with_original_import_trial<T>(run: impl FnOnce() -> T) -> T {
        struct Reset(bool);
        impl Drop for Reset {
            fn drop(&mut self) { crate::TEST_ORIGINAL_IMPORT_TRIAL.with(|flag| flag.set(self.0)); }
        }
        let _reset = Reset(crate::TEST_ORIGINAL_IMPORT_TRIAL.with(|flag| flag.replace(true)));
        run()
    }

    #[test]
    #[ignore = "explicit actual-checkpoint original-import parity; no timing assertions"]
    fn idle_equipment_import_matches_actual_checkpoint_for_31_complete_days() {
        let path = std::env::var("SPHERES_S22_CHECKPOINT").expect("actual checkpoint path required");
        let source = std::fs::read_to_string(&path).unwrap();
        let mut actual = crate::load(&source).unwrap();
        // Complete simulation ticks from the immutable checkpoint, with no
        // added player orders or annual-budget renewal. This is an equivalence
        // diagnostic, not the separate web qualification workload.
        assert!(actual.military_ai.enabled && economic_ai::enabled(&actual),
            "checkpoint must exercise military procurement");
        let initial_imports = actual.companies.imports.contracts.len();
        let initial_hits = crate::TEST_IDLE_IMPORT_SUCCESSES.with(|count| count.get());
        let mut original = actual.clone();
        for day in 0..31 {
            let received = crate::tick_day(&mut actual, &[]);
            let expected = with_original_import_trial(|| crate::tick_day(&mut original, &[]));
            assert_eq!(received, expected, "returned headlines day {day}");
            assert_eq!(actual.headlines, original.headlines, "retained headlines day {day}");
            assert_eq!(crate::save(&actual), crate::save(&original), "complete actual world day {day}");
            financial_bits(&actual, &original);
        }
        assert!(actual.companies.imports.contracts.len() > initial_imports,
            "checkpoint must exercise actual native import purchases");
        assert!(crate::TEST_IDLE_IMPORT_SUCCESSES.with(|count| count.get()) > initial_hits,
            "checkpoint must complete real purchases through the guarded import path");
        assert_eq!(std::fs::read_to_string(path).unwrap(), source, "input checkpoint is immutable");
    }

    fn fixture() -> WorldState {
        let (mut w, _) = stocked();
        support(&mut w, SMALL).unwrap();
        // The same disclosed current-year appropriation as the existing small
        // country import regression. No fast-path code creates authority.
        let p = w.nation_mut(SMALL).program_budget.as_mut().unwrap();
        p.available_bn[DEF][3] = 1.0;
        p.prepaid_bn[DEF][3] = 0.0;
        programs::begin_day(&mut w);
        assert!(co::import_purchase_quote(&w, SMALL, HOME, w.companies.firms[0].id,
            w.companies.firms[0].products[0].id, false, 1).valid);
        w
    }

    fn order(w: &WorldState, buyer: NationId, quantity: u32) -> co::CompanyOrder {
        let c = &w.companies.firms[0];
        let q = co::import_purchase_quote(w, buyer, HOME, c.id, c.products[0].id, false, quantity);
        co::CompanyOrder::ImportPurchase { seller: HOME, company: c.id,
            product: c.products[0].id, ammunition: false, quantity, quote: q.token }
    }

    fn financial_bits(a: &WorldState, b: &WorldState) {
        for (a, b) in a.nations.iter().zip(&b.nations) {
            assert_eq!(a.political_capital.to_bits(), b.political_capital.to_bits());
            assert_eq!(a.treasury_bn.map(f64::to_bits), b.treasury_bn.map(f64::to_bits));
            assert_eq!(a.debt_bn.map(f64::to_bits), b.debt_bn.map(f64::to_bits));
            if let (Some(a), Some(b)) = (&a.program_budget, &b.program_budget) {
                let bits = |p: &programs::ProgramBudget| [p.available_bn, p.prepaid_bn,
                    p.prepaid_used_today_bn, p.spent_today_bn, p.spent_ytd_bn,
                    p.noncapital_spent_today_bn].into_iter().flatten().flatten()
                    .map(f64::to_bits).collect::<Vec<_>>();
                assert_eq!(bits(a), bits(b));
            }
        }
        for (a, b) in a.companies.firms.iter().zip(&b.companies.firms) {
            assert_eq!(a.cash_bn.to_bits(), b.cash_bn.to_bits());
            for (a, b) in a.products.iter().zip(&b.products) {
                assert_eq!(a.stock_cost_bn.to_bits(), b.stock_cost_bn.to_bits());
                assert_eq!((a.stock, a.sold_units), (b.stock, b.sold_units));
            }
        }
    }

    fn compare(w: &WorldState, buyer: NationId, order: co::CompanyOrder,
        fast: bool, succeeds: bool, label: &str) -> WorldState {
        let before = crate::save(w);
        let command = Command::Company { nation: buyer, order: order.clone() };
        // Current imports carry no standing charge or policy snapshot. The
        // dispatcher must retain the native trial if either contract changes.
        assert!(crate::command_price(w, &command).filter(|(_, price, _)| *price > 0.0).is_none());
        assert!(crate::fiscal_journal::before_policy(w, &command).is_none());
        let mut direct = w.clone();
        let shortcut = co::try_apply_idle_equipment_import(&mut direct, buyer, &order);
        assert_eq!(shortcut.is_some(), fast, "{label}: idle eligibility");
        if !fast {
            assert_eq!(crate::save(&direct), before, "{label}: fallback probe is pure");
            financial_bits(&direct, w);
        }
        let mut actual = w.clone();
        let mut original = w.clone();
        let outcome = crate::apply_command_impl(&mut actual, &command, true);
        let expected = crate::apply_command_impl(&mut original, &command, false);
        assert_eq!(outcome, expected, "{label}: old full-world trial error/precedence");
        assert_eq!(outcome.is_ok(), succeeds, "{label}: {outcome:?}");
        assert_eq!(crate::save(&actual), crate::save(&original), "{label}: full-world parity");
        assert_eq!(actual.headlines, original.headlines);
        financial_bits(&actual, &original);
        if let Some(result) = shortcut {
            assert_eq!(result, outcome, "{label}: direct path result");
            assert_eq!(crate::save(&direct), crate::save(&actual), "{label}: dispatcher effects");
            financial_bits(&direct, &actual);
        }
        if !succeeds {
            assert_eq!(crate::save(&actual), before, "{label}: atomic refusal");
            financial_bits(&actual, w);
        }
        assert_eq!(crate::save(w), before, "{label}: source unchanged");
        actual
    }

    #[test]
    fn idle_equipment_import_matches_old_trial_stock_payment_replay_and_standing() {
        let base = fixture();
        for player in [Some(SMALL), Some(NationId::USA)] {
            for standing in [0.0, 100.0] {
                let mut w = base.clone();
                w.player = player;
                w.nation_mut(SMALL).political_capital = standing;
                let reviewed = order(&w, SMALL, 1);
                let actual = compare(&w, SMALL, reviewed.clone(), true, true, "paid player/AI import");
                assert_eq!(actual.companies.firms[0].products[0].stock,
                    w.companies.firms[0].products[0].stock - 1);
                assert_eq!(actual.companies.imports.contracts.len(), w.companies.imports.contracts.len() + 1);
                assert_eq!(actual.companies.imports.contracts.last().unwrap().settled_day, None);
                compare(&actual, SMALL, reviewed, true, false, "old quote replay");
                let command = Command::Company { nation: SMALL, order: order(&w, SMALL, 1) };
                let mut original = w.clone();
                crate::apply_command_impl(&mut original, &command, false).unwrap();
                let mut future = actual;
                crate::tick_day(&mut future, &[]);
                crate::tick_day(&mut original, &[]);
                assert_eq!(crate::save(&future), crate::save(&original), "later native fiscal/import settlement");
                financial_bits(&future, &original);
            }
        }
    }

    #[test]
    fn idle_equipment_import_preserves_all_precharge_refusals_and_malformed_money() {
        let base = fixture();
        for quantity in [0, 5, co::MAX_STOCK + 1] {
            compare(&base, SMALL, order(&base, SMALL, quantity), true, false, "invalid quantity or stock");
        }
        let mut stale = order(&base, SMALL, 1);
        if let co::CompanyOrder::ImportPurchase { quote, .. } = &mut stale { quote.push_str("-stale"); }
        compare(&base, SMALL, stale, true, false, "stale token");
        for case in 0..8 {
            let mut w = base.clone();
            let revision = w.companies.firms[0].products[0].revision_id.clone();
            match case {
                0 => w.nation_mut(SMALL).program_budget.as_mut().unwrap().available_bn[DEF][3] = 0.0,
                1 => w.nation_mut(SMALL).program_budget.as_mut().unwrap().settled_day = Some(clock::absolute_day(&base)),
                2 => w.companies.firms[0].products[0].cancelled_day = Some(clock::absolute_day(&base)),
                3 => { w.nation_mut(HOME).equipment.as_mut().unwrap().revisions.remove(&revision); }
                4 => { let district = w.companies.firms[0].district.clone(); w.districts.insert(district, NationId::UK); }
                5 => w.companies.next_id = u32::MAX - 8,
                6 => w.nation_mut(SMALL).alive = false,
                _ => w.rules.economic_competition = false,
            }
            compare(&w, SMALL, order(&w, SMALL, 1), true, false, &format!("refusal case {case}"));
        }
        for basis in [f64::NAN, f64::INFINITY, -1.0] {
            let mut w = base.clone();
            w.companies.firms[0].products[0].stock_cost_bn = basis;
            compare(&w, SMALL, order(&w, SMALL, 1), true, false, "invalid spend retains atomicity and exact float bits");
        }
        let mut missing_budget = base.clone();
        missing_budget.nation_mut(SMALL).program_budget = None;
        compare(&missing_budget, SMALL, order(&missing_budget, SMALL, 1), true, false, "missing department account");
        let mut missing = base.clone();
        missing.nations.retain(|n| n.id != SMALL); missing.reindex();
        compare(&missing, SMALL, order(&base, SMALL, 1), true, false, "missing actor precedes quote checks");
    }

    #[test]
    fn equipment_import_retains_trial_for_global_day_opening_and_due_receivables() {
        let base = fixture();
        let mut unopened = base.clone();
        unopened.nation_mut(HOME).program_budget.as_mut().unwrap().day = None;
        compare(&unopened, SMALL, order(&unopened, SMALL, 1), false, true, "unrelated alive budget opens");
        for kind in ["capitalization", "unrecognized-synthetic-kind"] {
            let mut due = base.clone();
            let day = clock::absolute_day(&due);
            due.companies.firms[0].receivables.push(co::Receivable {
                id: 90_000, day, kind: kind.into(), amount_bn: 0.001,
                product: None, delivery: None, refit: None,
            });
            due.nation_mut(HOME).program_budget.as_mut().unwrap().settled_day = Some(day);
            compare(&due, SMALL, order(&due, SMALL, 1), false, true, "global matching receipt, including unknown kind");
            due.nation_mut(SMALL).program_budget.as_mut().unwrap().settled_day = Some(day);
            compare(&due, SMALL, order(&due, SMALL, 1), false, false, "receipt effects roll back on closed buyer funding");
        }
        let mut ammunition = order(&base, SMALL, 1);
        if let co::CompanyOrder::ImportPurchase { ammunition, .. } = &mut ammunition { *ammunition = true; }
        compare(&base, SMALL, ammunition, false, false, "ammunition remains outside optimization");
        let mut monthly = base.clone(); monthly.rules.daily_simulation = false;
        compare(&monthly, SMALL, order(&monthly, SMALL, 1), false, false, "non-daily actor uses ordinary refusal");
    }

    #[test]
    fn equipment_import_retains_other_buyers_escrow_and_cancellation_refund_hooks() {
        let mut base = fixture();
        let other = NationId::UK;
        programs::set_construction_budget(&mut base, other, 0.0).unwrap();
        let q = co::import_enrollment_quote(&base, other); assert!(q.valid, "{:?}", q.reason);
        act(&mut base, other, co::CompanyOrder::EnableImports { quote:q.token }).unwrap();
        programs::begin_day(&mut base);
        base.nation_mut(other).program_budget.as_mut().unwrap().available_bn[DEF][3] = 1.0;
        let command = Command::Company { nation:other, order:order(&base, other, 1) };
        crate::apply_command_impl(&mut base, &command, false).unwrap();
        let day = clock::absolute_day(&base);
        let mut due = base.clone();
        due.nation_mut(other).program_budget.as_mut().unwrap().settled_day = Some(day);
        let settled = compare(&due, SMALL, order(&due, SMALL, 1), false, true, "another buyer's import escrow");
        assert_eq!(settled.companies.imports.contracts[0].settled_day, Some(day));
        let id = base.companies.imports.contracts[0].id;
        let q = co::import_cancel_quote(&base, other, id); assert!(q.valid, "{:?}", q.reason);
        act(&mut base, other, co::CompanyOrder::CancelImport { contract:id, quote:q.token }).unwrap();
        assert!(base.companies.imports.contracts[0].cancelled_day.is_some());
        assert!(base.companies.imports.contracts[0].refunded_day.is_none());
        base.nation_mut(other).program_budget.as_mut().unwrap().settled_day = Some(day);
        let refunded = compare(&base, SMALL, order(&base, SMALL, 1), false, true, "another buyer's cancellation settlement and refund");
        assert_eq!(refunded.companies.imports.contracts[0].refunded_day, Some(day));
    }
}
