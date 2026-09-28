// Included inside companies::tests, sharing its declared synthetic plant and
// appropriation fixture. These are command-equivalence checks, not earned play.
mod stock_target_tests {
    use super::*;

    fn fixture() -> WorldState {
        let mut w = supplier_fixture();
        let company = w.companies.firms[0].id;
        let spec = equipment::default_spec("ground_recon");
        let q = development_quote(&w, HOME, company, "Target test vehicle", &spec, 0.01, 2);
        assert!(q.valid, "{:?}", q.reason);
        apply(
            &mut w,
            HOME,
            &CompanyOrder::Develop {
                company,
                name: "Target test vehicle".into(),
                spec,
                daily_budget_bn: 0.01,
                stock_target: 2,
                quote: q.token,
            },
        )
        .unwrap();
        // A separate disclosed certified source permits ordinary ammo licensing
        // without pretending that this unit test earned hundreds of work days.
        let revision = w.companies.firms[0].products[0].revision_id.clone();
        let mut source = w.nation(HOME).equipment.as_ref().unwrap().revisions[&revision].clone();
        source.id = "synthetic-certified-ammo-source".into();
        source.certified_day = Some(clock::absolute_day(&w));
        w.nation_mut(HOME)
            .equipment
            .as_mut()
            .unwrap()
            .revisions
            .insert(source.id.clone(), source);
        equipment::set_maintenance_plan(&mut w, HOME, 0.01).unwrap();
        let q = ammo_supply_quote(&w, HOME, company, "mg_127", 100);
        assert!(q.valid, "{:?}", q.reason);
        apply(
            &mut w,
            HOME,
            &CompanyOrder::AmmoSupply {
                company,
                family: "mg_127".into(),
                stock_target: 100,
                quote: q.token,
            },
        )
        .unwrap();
        w.nation_mut(HOME).political_capital = 0.0;
        w
    }

    fn order(w: &WorldState, ammo: bool, target: u32) -> CompanyOrder {
        let c = &w.companies.firms[0];
        if ammo {
            CompanyOrder::AmmoInventory {
                company: c.id,
                product: c.ammunition_products[0].id,
                stock_target: target,
            }
        } else {
            CompanyOrder::Inventory {
                company: c.id,
                product: c.products[0].id,
                stock_target: target,
            }
        }
    }

    fn compare(w: &WorldState, order: CompanyOrder, succeeds: bool) -> WorldState {
        let before = crate::save(w);
        let command = crate::Command::Company {
            nation: HOME,
            order: order.clone(),
        };
        // This is the exact original retained-trial boundary. These commands
        // have zero standing cost and no policy journal; keep that explicit.
        assert!(crate::command_price(w, &command)
            .filter(|(_, p, _)| *p > 0.0)
            .is_none());
        assert!(crate::fiscal_journal::before_policy(w, &command).is_none());
        let staged = trial(w, HOME, &order);
        let expected = staged.as_ref().map(|_| ()).map_err(Clone::clone);
        assert_eq!(expected.is_ok(), succeeds, "{order:?}: {expected:?}");
        assert_eq!(crate::refusal_of(w, &command), expected.clone().err());
        assert_eq!(crate::save(w), before, "read preview changed input");
        let mut actual = w.clone();
        assert_eq!(crate::apply_command(&mut actual, &command), expected);
        let original = staged.unwrap_or_else(|_| w.clone());
        assert_eq!(
            crate::save(&actual),
            crate::save(&original),
            "full-world command parity"
        );
        let mut direct = w.clone();
        assert_eq!(
            try_apply_stock_target(&mut direct, HOME, &order),
            Some(expected)
        );
        assert_eq!(
            crate::save(&direct),
            crate::save(&original),
            "complete direct-path parity"
        );
        if !succeeds {
            assert_eq!(
                crate::save(&actual),
                before,
                "refusal must not partially mutate"
            );
        }
        actual
    }

    #[test]
    fn stock_targets_preserve_full_world_and_native_refusal_precedence() {
        let base = fixture();
        for ammo in [false, true] {
            let maximum = if ammo { MAX_AMMO_STOCK } else { MAX_STOCK };
            for target in [maximum + 1, u32::MAX] {
                compare(&base, order(&base, ammo, target), false);
            }
            for change in [
                (|w: &mut WorldState| w.rules.daily_simulation = false) as fn(&mut WorldState),
                |w| w.rules.military_operations = false,
                |w| w.rules.resource_market = false,
                |w| w.rules.manufacturing_system = false,
                |w| w.nation_mut(HOME).alive = false,
                |w| w.nations.retain(|n| n.id != HOME),
                |w| w.player = Some(NationId::USA),
                |w| w.nation_mut(HOME).program_budget = None,
                |w| {
                    let p = w.nation_mut(HOME).program_budget.as_mut().unwrap();
                    p.departments[BUDGET_DEFENSE][0] = 0;
                    p.departments[BUDGET_DEFENSE][1] = 0;
                },
            ] {
                let mut w = base.clone();
                change(&mut w);
                // Both valid and bad targets: actor/account errors still win.
                compare(&w, order(&base, ammo, 1), false);
                compare(&w, order(&base, ammo, maximum + 1), false);
            }
            for change in [
                (|w: &mut WorldState| w.companies.firms.clear()) as fn(&mut WorldState),
                |w| w.companies.firms[0].nation = NationId::USA,
                |w| {
                    w.companies.firms[0].products.clear();
                    w.companies.firms[0].ammunition_products.clear();
                },
            ] {
                let mut w = base.clone();
                change(&mut w);
                compare(&w, order(&base, ammo, 1), false);
                compare(&w, order(&base, ammo, maximum + 1), false);
            }
        }
        let mut cancelled = base.clone();
        cancelled.companies.firms[0].products[0].cancelled_day = Some(clock::absolute_day(&base));
        compare(&cancelled, order(&base, false, 1), false);
        compare(&cancelled, order(&base, false, MAX_STOCK + 1), false);
        // Ammo's native setter checks the licensed product identity, not the
        // status of the separate vehicle programme. Preserve that distinction.
        compare(&cancelled, order(&base, true, 1), true);
    }

    #[test]
    fn stock_targets_preserve_player_ai_replay_and_pending_finance() {
        let base = fixture();
        assert!(!base.companies.firms[0].receivables.is_empty());
        for ai in [false, true] {
            for ammo in [false, true] {
                let mut w = base.clone();
                if ai {
                    w.player = Some(NationId::USA);
                    w.rules.economic_competition = true;
                }
                let maximum = if ammo { MAX_AMMO_STOCK } else { MAX_STOCK };
                for target in [0, 1, maximum] {
                    let c = order(&w, ammo, target);
                    let updated = compare(&w, c.clone(), true);
                    let replayed = compare(&updated, c, true);
                    assert_eq!(crate::save(&updated), crate::save(&replayed));
                    assert_eq!(updated.nation(HOME).political_capital, 0.0);
                    let mut only_target = w.clone();
                    if ammo {
                        only_target.companies.firms[0].ammunition_products[0].stock_target = target;
                    } else {
                        only_target.companies.firms[0].products[0].stock_target = target;
                    }
                    assert_eq!(crate::save(&updated), crate::save(&only_target),
                        "no money, receipt, production, transactions or other world fields may change");
                }
            }
        }
        // A closed financial date and pending receipts must stay untouched;
        // inventory requests do not open or settle a fiscal day.
        let mut closed = base.clone();
        let day = clock::absolute_day(&closed);
        closed
            .nation_mut(HOME)
            .program_budget
            .as_mut()
            .unwrap()
            .settled_day = Some(day);
        clock::advance_date(&mut closed);
        for ammo in [false, true] {
            let c = order(&closed, ammo, 1);
            let mut actual = compare(&closed, c.clone(), true);
            let mut original = trial(&closed, HOME, &c).unwrap();
            crate::tick_day(&mut actual, &[]);
            crate::tick_day(&mut original, &[]);
            assert_eq!(
                crate::save(&actual),
                crate::save(&original),
                "future settlement parity"
            );
        }
        let mut certified = base.clone();
        certified.companies.firms[0].products[0].certified_day = Some(day);
        certified.companies.firms[0].products[0].status = "retired".into();
        compare(&certified, order(&base, false, 0), true);
    }

    #[test]
    fn stock_target_shortcut_excludes_other_corporate_orders_without_writes() {
        let mut w = fixture();
        let company = w.companies.firms[0].id;
        let product = w.companies.firms[0].products[0].id;
        for order in [
            CompanyOrder::Funding {
                company,
                product,
                daily_budget_bn: 1.0,
            },
            CompanyOrder::CancelDevelopment { company, product },
            CompanyOrder::Capitalize {
                company,
                amount_bn: 1.0,
                quote: "stale".into(),
            },
            CompanyOrder::Purchase {
                company,
                product,
                quantity: 1,
                quote: "stale".into(),
            },
            CompanyOrder::AmmoPurchase {
                company,
                product,
                quantity: 1,
                quote: "stale".into(),
            },
        ] {
            let before = crate::save(&w);
            assert!(try_apply_stock_target(&mut w, HOME, &order).is_none());
            assert_eq!(crate::save(&w), before);
        }
    }
}
