// Exact catalogue/decision comparisons. Synthetic edge cases never modify the
// immutable campaign inputs used by the separately invoked 31-day oracle.
mod ground_offer_tests {
    use super::*;

    pub(super) fn original<T>(run: impl FnOnce() -> T) -> T {
        struct Reset(bool);
        impl Drop for Reset {
            fn drop(&mut self) { TEST_ORIGINAL_GROUND_OFFERS.with(|flag| flag.set(self.0)); }
        }
        let _reset = Reset(TEST_ORIGINAL_GROUND_OFFERS.with(|flag| flag.replace(true)));
        run()
    }

    fn consumed(mut offers: Vec<co::ImportOffer>, family: &str) -> Vec<co::ImportOffer> {
        offers.retain(|o| o.ammunition && o.ready_stock > 0 && o.family.as_deref() == Some(family));
        offers
    }

    fn bits(offers: &[co::ImportOffer]) -> Vec<[u64; 3]> {
        offers.iter().map(|o| [o.unit_price_bn.to_bits(), o.purchase_available_bn.to_bits(),
            o.protected_maintenance_bn.to_bits()]).collect()
    }

    #[test]
    fn ground_offer_filter_keeps_order_fields_float_bits_and_refusal_rows() {
        let base = empty_import_tests::with_ammunition_stock();
        let family = base.companies.firms[0].ammunition_products[0].family.clone();
        for buyer in [HOME, SMALL] { for case in 0..9 {
            let mut w = base.clone();
            match case {
                1 => { w.companies.firms[0].ammunition_products[0].stock = 0; },
                2 => { w.sanctions.push((HOME, SMALL)); },
                3 => { w.nation_mut(buyer).program_budget.as_mut().unwrap().available_bn[DEF][2] = 0.0; },
                4 => { w.companies.firms[0].ammunition_products[0].stock_cost_bn = f64::NAN; },
                5 => {
                    let mut duplicate = w.companies.firms[0].ammunition_products[0].clone();
                    duplicate.stock = 0;
                    w.companies.firms[0].ammunition_products.insert(0, duplicate);
                },
                6 => {
                    let duplicate = w.companies.firms[0].clone();
                    w.companies.firms.push(duplicate);
                },
                7 => { w.companies.firms[0].ammunition_products[0].source_revision = "missing".into(); },
                8 => { w.companies.firms.clear(); },
                _ => {},
            }
            let before = crate::save(&w);
            let public_domestic = co::domestic_offers(&w, buyer);
            let public_import = co::import_offers(&w, buyer);
            let public_bytes = serde_json::to_vec(&(&public_domestic, &public_import)).unwrap();
            let mut full = public_domestic;
            full.extend(public_import);
            let expected = consumed(full, &family);
            let required = BTreeMap::from([(family.clone(), 1.0)]);
            let actual = ground_store_offers(&w, buyer, &required);
            assert_eq!(serde_json::to_vec(&actual).unwrap(), serde_json::to_vec(&expected).unwrap(), "{buyer:?}/{case}");
            assert_eq!(bits(&actual), bits(&expected), "exact floating point bits");
            assert!(ground_store_offers(&w, buyer, &BTreeMap::new()).is_empty());
            assert_eq!(serde_json::to_vec(&(co::domestic_offers(&w, buyer), co::import_offers(&w, buyer))).unwrap(), public_bytes,
                "private filtering does not alter public catalogues");
            assert_eq!(crate::save(&w), before, "pure read {buyer:?}/{case}");
            if case == 0 { assert!(!actual.is_empty(), "real stocked matching row required"); }
        }}
    }

    #[test]
    fn ground_offer_filter_keeps_multiple_required_families_and_omits_only_unconsumed_rows() {
        let mut w = empty_import_tests::with_ammunition_stock();
        let first = w.companies.firms[0].ammunition_products[0].clone();
        // Explicit synthetic catalogue entries isolate family filtering. Their
        // original certification/source and any refusal reasons remain visible.
        let other: Vec<_> = eq::ammo_catalog().iter().filter(|d| d.id != first.family).take(2).collect();
        for (i, def) in other.iter().enumerate() {
            let mut row = first.clone();
            row.id += 100 + i as u32;
            row.family = def.id.into();
            w.companies.firms[0].ammunition_products.push(row);
        }
        let required = BTreeMap::from([(first.family.clone(), 1.0), (other[0].id.to_string(), 2.0)]);
        for buyer in [HOME, SMALL] {
            let before = crate::save(&w);
            let mut expected = co::domestic_offers(&w, buyer);
            expected.extend(co::import_offers(&w, buyer));
            expected.retain(|o| o.ammunition && o.ready_stock > 0 && o.family.as_ref().is_some_and(|f| required.contains_key(f)));
            let actual = ground_store_offers(&w, buyer, &required);
            assert_eq!(actual.len(), 2);
            assert_eq!(serde_json::to_vec(&actual).unwrap(), serde_json::to_vec(&expected).unwrap());
            assert_eq!(bits(&actual), bits(&expected));
            assert_eq!(crate::save(&w), before);
        }
    }

    #[test]
    #[ignore = "explicit immutable actual-checkpoint original ground-offer comparison; no timing claims"]
    fn ground_offer_filter_matches_actual_checkpoint_for_31_complete_days() {
        let path = std::env::var("SPHERES_S22_CHECKPOINT").expect("actual checkpoint required");
        let source = std::fs::read_to_string(&path).unwrap();
        let mut value: serde_json::Value = serde_json::from_str(&source).unwrap();
        let world = if value["format"] == "spheres-campaign" { value["world"].take() } else { value.take() };
        drop(value);
        let mut actual = crate::load_value(world).unwrap();
        assert!(actual.year == 2015 || actual.year == 2035);
        assert!(actual.military_ai.enabled && economic_ai::enabled(&actual));
        let mut expected = actual.clone();
        let before = co::skipped_ammunition_offer_quotes();
        let reviews = TEST_FILTERED_GROUND_REVIEWS.with(|count| count.get());
        for day in 0..31 {
            let a = crate::tick_day(&mut actual, &[]);
            let b = original(|| crate::tick_day(&mut expected, &[]));
            assert_eq!(a, b, "returned headlines day {day}");
            assert_eq!(actual.headlines, expected.headlines, "retained headlines day {day}");
            assert!(crate::save(&actual) == crate::save(&expected), "complete native world day {day}");
        }
        let skipped = co::skipped_ammunition_offer_quotes() - before;
        let reviews = TEST_FILTERED_GROUND_REVIEWS.with(|count| count.get()) - reviews;
        assert!(skipped > 0 && reviews > 0, "actual stock review must omit unconsumed quotes");
        assert_eq!(std::fs::read_to_string(path).unwrap(), source, "immutable source");
        eprintln!("31 native days match original ground offers; {reviews} reviews omitted {skipped} unconsumed quotes.");
    }
}
