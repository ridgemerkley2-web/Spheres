mod air_support_tests {
    use super::*;
    use crate::{arsenal, companies, programs, resources, world::BUDGET_DEFENSE as D};
    const ID: NationId = NationId::USA;
    const MODEL: &str = "air-support-fixture";
    const FAMILY: &str = "air_bomb_unguided";

    fn near(a: f64, b: f64) {
        assert!((a - b).abs() < 1e-12, "{a} != {b}");
    }

    // Explicit synthetic certified hardware and a completed plant, followed by
    // ordinary company capitalization, paid fabrication and supplier purchases.
    fn fixture(with_supplier: bool) -> WorldState {
        let (mut w, district) = super::tests::fixture();
        let day = clock::absolute_day(&w);
        let spec = default_spec("air_light_attack");
        let profile = design_preview(&w, ID, &spec).profile.unwrap();
        w.nation_mut(ID).arsenal.held.clear();
        w.nation_mut(ID).arsenal.orders.clear();
        state_mut(w.nation_mut(ID)).revisions.insert(
            MODEL.into(),
            DesignRevision {
                id: MODEL.into(),
                name: "Synthetic support aircraft".into(),
                specification_key: specification_key(&spec),
                spec,
                profile,
                created_day: day,
                certified_day: Some(day),
            },
        );
        arsenal::deliver_design(w.nation_mut(ID), MODEL, 4, 0.0).unwrap();
        w.nation_mut(ID).treasury_bn = Some(1000.0);
        set_maintenance_plan(&mut w, ID, 1.0).unwrap();
        if with_supplier {
            for c in resources::ALL {
                if c.tracked() {
                    resources::set_stockpile_for_test(&mut w, ID, c, 1_000_000.0);
                }
            }
            programs::begin_day(&mut w);
            let q =
                companies::establishment_quote(&w, ID, "Synthetic store supplier", &district, 0.01);
            assert!(q.valid, "{:?}", q.reason);
            companies::apply(
                &mut w,
                ID,
                &companies::CompanyOrder::Establish {
                    name: "Synthetic store supplier".into(),
                    district,
                    capitalization_bn: 0.01,
                    quote: q.token,
                },
            )
            .unwrap();
            let company = w.companies.firms.last().unwrap().id;
            let q = companies::ammo_supply_quote(&w, ID, company, FAMILY, 200);
            assert!(q.valid, "{:?}", q.reason);
            companies::apply(
                &mut w,
                ID,
                &companies::CompanyOrder::AmmoSupply {
                    company,
                    family: FAMILY.into(),
                    stock_target: 200,
                    quote: q.token,
                },
            )
            .unwrap();
        }
        close(&mut w);
        for _ in 0..4 {
            open_next(&mut w);
            companies::tick_day(&mut w);
            settle_support(&mut w);
            close(&mut w);
        }
        if with_supplier {
            assert!(w.companies.firms[0].ammunition_products[0].stock > 0);
            companies::validate_state(&w).unwrap();
        }
        validate_state(w.nation(ID)).unwrap();
        w
    }

    fn close(w: &mut WorldState) {
        programs::stage_fiscal(w.nation_mut(ID), 0.0, 0.0);
        programs::finish_day(w);
        companies::settle_receivables(w);
    }

    fn open_next(w: &mut WorldState) {
        clock::advance_date(w);
        programs::begin_day(w);
    }

    fn review_next(w: &mut WorldState) {
        open_next(w);
        settle_support(w);
        tick_air_support(w);
    }

    fn policy(w: &WorldState) -> &AirSupportPolicy {
        w.nation(ID)
            .equipment
            .as_ref()
            .unwrap()
            .air_support
            .as_ref()
            .unwrap()
    }

    fn receipt(w: &WorldState) -> &AirSupportReceipt {
        policy(w).receipt.as_ref().unwrap()
    }

    #[test]
    fn air_support_is_prospective_and_configuration_never_buys_hardware_or_stores() {
        let mut w = fixture(true);
        let before = crate::save(&w);
        let _ = air_support_status(&w, ID);
        assert_eq!(crate::save(&w), before);
        let inventory = serde_json::to_string(&w.nation(ID).arsenal).unwrap();
        let companies = w.companies.clone();
        let fiscal = w.nation(ID).program_budget.clone();
        set_air_support(&mut w, ID, 1.0, 30, true).unwrap();
        tick_air_support(&mut w);
        assert!(policy(&w).receipt.is_none());
        assert_eq!(
            serde_json::to_string(&w.nation(ID).arsenal).unwrap(),
            inventory
        );
        assert_eq!(w.companies, companies);
        assert_eq!(w.nation(ID).program_budget, fiscal);
        assert_eq!(
            air_support_maintenance_limit(w.nation(ID), clock::absolute_day(&w)),
            None
        );
        review_next(&mut w);
        assert!(!receipt(&w).purchases.is_empty());
        assert_eq!(
            serde_json::to_string(&w.nation(ID).arsenal).unwrap(),
            inventory
        );
        assert!(w
            .nation(ID)
            .equipment
            .as_ref()
            .unwrap()
            .ammunition
            .as_ref()
            .is_none_or(|a| a.stocks.get(FAMILY).copied().unwrap_or(0.0) == 0.0));
    }

    #[test]
    fn air_support_caps_combined_upkeep_and_stores_and_repeat_hooks_are_inert() {
        let mut w = fixture(true);
        let required = fleet_maintenance_requirement(w.nation(ID));
        let unit = companies::domestic_offers(&w, ID)
            .iter()
            .find(|o| o.ammunition)
            .unwrap()
            .unit_price_bn;
        let cap = required + unit * 3.5;
        set_air_support(&mut w, ID, cap * 1000.0, 30, true).unwrap();
        review_next(&mut w);
        let r = receipt(&w);
        assert_eq!(r.purchases.iter().map(|p| p.quantity).sum::<u32>(), 3);
        assert!(r.maintenance_paid_bn + r.stores_paid_bn <= cap + 1e-12);
        assert!((r.maintenance_paid_bn - required).abs() < 1e-12);
        assert_eq!(r.families[0].bought, 3);
        let saved = crate::save(&w);
        tick_air_support(&mut w);
        settle_support(&mut w);
        tick_air_support(&mut w);
        assert_eq!(crate::save(&w), saved);
        close(&mut w);
        validate_state(w.nation(ID)).unwrap();
        companies::validate_state(&w).unwrap();
    }

    #[test]
    fn air_support_zero_and_half_caps_cannot_grant_maintenance_or_stores() {
        for fraction in [0.0, 0.5] {
            let mut w = fixture(true);
            let required = fleet_maintenance_requirement(w.nation(ID));
            set_air_support(&mut w, ID, required * fraction * 1000.0, 30, true).unwrap();
            review_next(&mut w);
            let r = receipt(&w);
            assert!((r.maintenance_paid_bn - required * fraction).abs() < 1e-12);
            assert_eq!(r.stores_paid_bn, 0.0);
            assert!(r.purchases.is_empty());
            assert!(
                (w.nation(ID)
                    .equipment
                    .as_ref()
                    .unwrap()
                    .maintenance_fraction
                    - fraction)
                    .abs()
                    < 1e-9
            );
            validate_state(w.nation(ID)).unwrap();
        }
    }

    #[test]
    fn air_support_paid_inbound_prevents_duplicate_target_purchases_after_reload() {
        let mut w = fixture(true);
        set_air_support(&mut w, ID, 100.0, 30, true).unwrap();
        review_next(&mut w);
        assert!(!receipt(&w).purchases.is_empty());
        assert_eq!(receipt(&w).families[0].bought, receipt(&w).families[0].gap);
        close(&mut w);
        let deliveries = w.companies.ammunition_deliveries.clone();
        let saved = crate::save(&w);
        let mut loaded = crate::load(&saved).unwrap();
        assert_eq!(crate::save(&loaded), saved);
        for _ in 0..3 {
            review_next(&mut w);
            review_next(&mut loaded);
            assert!(receipt(&w).purchases.is_empty());
            assert!(receipt(&w).families[0].inbound > 0);
            close(&mut w);
            close(&mut loaded);
            assert_eq!(crate::save(&w), crate::save(&loaded));
            assert_eq!(w.companies.ammunition_deliveries, deliveries);
        }
    }

    #[test]
    fn air_support_edits_and_toggles_preserve_today_and_the_booked_receipt() {
        let mut w = fixture(true);
        let required = fleet_maintenance_requirement(w.nation(ID));
        set_air_support(&mut w, ID, required * 500.0, 30, true).unwrap();
        review_next(&mut w);
        let day = clock::absolute_day(&w);
        let booked = receipt(&w).clone();
        let fiscal = w.nation(ID).program_budget.clone();
        set_air_support(&mut w, ID, 100.0, 90, false).unwrap();
        let status = air_support_status(&w, ID);
        assert!(status.reason.contains("next simulation day"));
        assert!(status.active.as_ref().unwrap().automatic);
        assert!(!status.pending.as_ref().unwrap().automatic);
        near(
            status.active.as_ref().unwrap().daily_budget_bn,
            required * 0.5,
        );
        assert_eq!(status.remaining_bn, 0.0);
        set_air_support(&mut w, ID, 100.0, 90, true).unwrap();
        assert_eq!(
            air_support_maintenance_limit(w.nation(ID), day),
            Some(required * 0.5)
        );
        tick_air_support(&mut w);
        assert_eq!(*receipt(&w), booked);
        assert_eq!(w.nation(ID).program_budget, fiscal);
        close(&mut w);
        review_next(&mut w);
        assert!(!receipt(&w).purchases.is_empty());
        assert_eq!(receipt(&w).day, day + 1);
    }

    #[test]
    fn air_support_has_no_supplier_creation_or_import_enrollment_fallback() {
        let mut w = fixture(false);
        let before_companies = w.companies.clone();
        let before_research = w.nation(ID).equipment.as_ref().unwrap().learned.clone();
        set_air_support(&mut w, ID, 100.0, 365, true).unwrap();
        review_next(&mut w);
        assert!(receipt(&w).purchases.is_empty());
        assert!(receipt(&w).families[0]
            .reason
            .contains("No allowed supplier"));
        assert_eq!(w.companies, before_companies);
        assert_eq!(
            w.nation(ID).equipment.as_ref().unwrap().learned,
            before_research
        );
        assert!(w
            .nation(ID)
            .equipment
            .as_ref()
            .unwrap()
            .ammunition
            .as_ref()
            .is_none_or(|a| a.orders.is_empty()));
        assert!(!companies::imports_enabled(&w, ID));
    }

    #[test]
    fn air_support_exhausted_authority_blocks_purchases_without_minting_authority() {
        let mut w = fixture(true);
        set_air_support(&mut w, ID, 100.0, 30, true).unwrap();
        open_next(&mut w);
        let p = w.nation_mut(ID).program_budget.as_mut().unwrap();
        p.available_bn[D][2] = 0.0;
        p.prepaid_bn[D][2] = 0.0;
        let before = w.nation(ID).program_budget.clone();
        settle_support(&mut w);
        tick_air_support(&mut w);
        assert_eq!(receipt(&w).maintenance_paid_bn, 0.0);
        assert_eq!(receipt(&w).stores_paid_bn, 0.0);
        assert!(receipt(&w).purchases.is_empty());
        assert_eq!(w.nation(ID).program_budget, before);
    }

    #[test]
    fn air_support_rejects_invalid_limits_and_forged_purchase_receipts() {
        let mut w = fixture(true);
        let before = crate::save(&w);
        for amount in [f64::NAN, f64::INFINITY, -1.0, 1_000_001.0] {
            assert!(set_air_support(&mut w, ID, amount, 30, true).is_err());
            assert_eq!(crate::save(&w), before);
        }
        for days in [0, 366] {
            assert!(set_air_support(&mut w, ID, 1.0, days, true).is_err());
            assert_eq!(crate::save(&w), before);
        }
        set_air_support(&mut w, ID, 100.0, 30, true).unwrap();
        review_next(&mut w);
        validate_air_support_world(&w).unwrap();
        let mut wrong = w.clone();
        state_mut(wrong.nation_mut(ID))
            .air_support
            .as_mut()
            .unwrap()
            .receipt
            .as_mut()
            .unwrap()
            .purchases[0]
            .delivery += 1000;
        assert!(validate_air_support_world(&wrong).is_err());
        state_mut(w.nation_mut(ID))
            .air_support
            .as_mut()
            .unwrap()
            .receipt
            .as_mut()
            .unwrap()
            .purchases[0]
            .quantity += 1;
        assert!(validate_air_support(w.nation(ID)).is_err());
    }

    #[test]
    fn air_support_counts_existing_public_work_and_never_substitutes_incompatible_stores() {
        let mut w = fixture(false);
        let district = w
            .districts
            .iter()
            .find(|(_, n)| **n == ID)
            .unwrap()
            .0
            .clone();
        let target = air_support_families(&w, ID, 30)[0].target_stores;
        let order = start_ammo_order(&mut w, ID, FAMILY, &district, target, 1.0).unwrap();
        pause_ammo_order(&mut w, ID, order, true).unwrap();
        let row = &air_support_families(&w, ID, 30)[0];
        assert_eq!(row.gap, 0);
        assert_eq!(row.public_committed, target as u64);
        cancel_ammo_order(&mut w, ID, order).unwrap();
        seed_test_ammunition(&mut w, ID, "air_bomb_guided", 100_000);
        let row = &air_support_families(&w, ID, 30)[0];
        assert_eq!(row.family, FAMILY);
        assert_eq!(row.stock, 0.0);
        assert_eq!(row.gap, target);
        set_air_support(&mut w, ID, 100.0, 30, true).unwrap();
        review_next(&mut w);
        assert!(receipt(&w).purchases.is_empty());
        assert_eq!(
            w.nation(ID)
                .equipment
                .as_ref()
                .unwrap()
                .ammunition
                .as_ref()
                .unwrap()
                .stocks["air_bomb_guided"],
            100_000.0
        );
    }

    fn add_service_squadron(w: &mut WorldState, service_days_left: u8) {
        let district = w
            .districts
            .iter()
            .find(|(d, n)| **n == ID && crate::airbases::district_location(d).is_some())
            .unwrap()
            .0
            .clone();
        w.airbases = Some(crate::airbases::AirbaseState {
            bases: vec![crate::airbases::Airbase {
                id: district.clone(),
                district: district.clone(),
                name: "Authored support fixture".into(),
                sponsor: ID,
                capacity_level: 1,
                support_level: 0,
                protection_level: 0,
                project: None,
                history: vec![],
            }],
            ..Default::default()
        });
        w.nation_mut(ID).aviation = Some(crate::aviation::AviationState {
            next_id: 2,
            last_service_day: None,
            squadrons: vec![crate::aviation::Squadron {
                id: 1,
                name: "Service fixture".into(),
                revision: MODEL.into(),
                assigned: 4,
                base: Some(district),
                transit: None,
                service_days_left,
            }],
        });
    }

    fn service_left(w: &WorldState) -> u8 {
        w.nation(ID).aviation.as_ref().unwrap().squadrons[0].service_days_left
    }

    #[test]
    fn air_support_repairs_need_full_funding_one_day_at_a_time_and_resume_exactly() {
        let mut w = fixture(false);
        add_service_squadron(&mut w, 2);
        set_air_support(&mut w, ID, 0.0, 30, true).unwrap();
        review_next(&mut w);
        assert_eq!(service_left(&w), 2);
        assert_eq!(
            crate::aviation::ready_units(w.nation(ID), MODEL, clock::absolute_day(&w)),
            0
        );
        set_air_support(&mut w, ID, 100.0, 30, true).unwrap();
        tick_air_support(&mut w);
        assert_eq!(service_left(&w), 2);
        close(&mut w);
        review_next(&mut w);
        assert_eq!(service_left(&w), 1);
        tick_air_support(&mut w);
        assert_eq!(service_left(&w), 1);
        close(&mut w);
        let saved = crate::save(&w);
        let mut loaded = crate::load(&saved).unwrap();
        review_next(&mut w);
        review_next(&mut loaded);
        assert_eq!(service_left(&w), 0);
        assert_eq!(crate::save(&w), crate::save(&loaded));
        assert_eq!(
            crate::aviation::ready_units(w.nation(ID), MODEL, clock::absolute_day(&w)),
            4
        );
    }

    #[test]
    fn air_support_cannot_repair_aircraft_without_an_accessible_base() {
        let mut w = fixture(false);
        add_service_squadron(&mut w, 2);
        w.nation_mut(ID).aviation.as_mut().unwrap().squadrons[0].base = None;
        set_air_support(&mut w, ID, 100.0, 30, true).unwrap();
        review_next(&mut w);
        assert!(receipt(&w).maintenance_paid_bn > 0.0);
        assert_eq!(service_left(&w), 2);
    }

    #[test]
    fn air_support_one_cap_prospectively_funds_upkeep_and_protects_it_before_store_orders() {
        let mut w = fixture(true);
        set_maintenance_plan(&mut w, ID, 0.0).unwrap();
        let required = fleet_maintenance_requirement(w.nation(ID));
        set_air_support(&mut w, ID, 100.0, 30, true).unwrap();
        open_next(&mut w);
        let protected = companies::ammo_purchase_funding(&w, ID).0;
        near(protected, required);
        settle_support(&mut w);
        tick_air_support(&mut w);
        near(receipt(&w).maintenance_paid_bn, required);
        assert!(!receipt(&w).purchases.is_empty());
        assert_eq!(
            w.nation(ID)
                .equipment
                .as_ref()
                .unwrap()
                .maintenance_plan
                .as_ref()
                .unwrap()
                .daily_limit_bn,
            0.0
        );
        set_air_support(&mut w, ID, 100.0, 30, false).unwrap();
        close(&mut w);
        open_next(&mut w);
        assert_eq!(
            air_support_maintenance_limit(w.nation(ID), clock::absolute_day(&w)),
            None
        );
        settle_support(&mut w);
        tick_air_support(&mut w);
        assert_eq!(
            w.nation(ID)
                .equipment
                .as_ref()
                .unwrap()
                .maintenance_fraction,
            0.0
        );
    }

    #[test]
    fn air_support_does_not_buy_stores_after_an_underfunded_dated_upkeep_invoice() {
        let mut w = fixture(true);
        let required = fleet_maintenance_requirement(w.nation(ID));
        set_air_support(&mut w, ID, 100.0, 30, true).unwrap();
        open_next(&mut w);
        let p = w.nation_mut(ID).program_budget.as_mut().unwrap();
        p.available_bn[D][2] = required * 0.5;
        p.prepaid_bn[D][2] = 0.0;
        settle_support(&mut w);
        // Explicit fixture authority restoration after the dated invoice. It
        // cannot silently repair or reopen that invoice on the same date.
        w.nation_mut(ID)
            .program_budget
            .as_mut()
            .unwrap()
            .available_bn[D][2] = 1.0;
        tick_air_support(&mut w);
        near(receipt(&w).maintenance_paid_bn, required * 0.5);
        assert!(receipt(&w).purchases.is_empty());
        assert!(receipt(&w).families[0].reason.contains("underfunded"));
    }
}
