// Disclosed synthetic catalogue cases plus a separately invoked actual-save
// oracle. Neither one changes any campaign/performance qualification input.
mod empty_import_tests {
    use super::*;

    fn original_lists<T>(run: impl FnOnce() -> T) -> T {
        struct Reset(bool);
        impl Drop for Reset {
            fn drop(&mut self) { TEST_ORIGINAL_PROCUREMENT_IMPORT_LISTS.with(|flag|flag.set(self.0)); }
        }
        let _reset=Reset(TEST_ORIGINAL_PROCUREMENT_IMPORT_LISTS.with(|flag|flag.replace(true)));
        run()
    }

    fn filtered(mut offers: Vec<co::ImportOffer>) -> Vec<co::ImportOffer> {
        offers.retain(|o| !o.ammunition && o.ready_stock>0);
        offers
    }

    fn offer_bits(offers: &[co::ImportOffer]) -> Vec<[u64;3]> {
        offers.iter().map(|o|[o.unit_price_bn.to_bits(),o.purchase_available_bn.to_bits(),
            o.protected_maintenance_bn.to_bits()]).collect()
    }

    #[test]
    fn empty_import_precheck_keeps_filtered_rows_order_and_public_catalogue_identical() {
        let (base,_)=stocked();
        for n in [HOME,SMALL] { for case in 0..7 {
            let mut w=base.clone();
            match case {
                1=>{w.companies.firms[0].products[0].stock=0;w.companies.firms[0].products[0].stock_cost_bn=0.0;},
                2=>w.companies.firms[0].products[0].cancelled_day=Some(clock::absolute_day(&base)),
                3=>{let id=w.companies.firms[0].products[0].revision_id.clone();w.nation_mut(HOME).equipment.as_mut().unwrap().revisions.remove(&id);},
                4=>w.sanctions.push((HOME,SMALL)),
                5=>w.companies.firms[0].products[0].stock_cost_bn=f64::NAN,
                6=>w.companies.firms.clear(),
                _=>{},
            }
            let before=crate::save(&w);
            let public=co::import_offers(&w,n);
            let public_bytes=serde_json::to_vec(&public).unwrap();
            let skips=TEST_EMPTY_PROCUREMENT_IMPORT_SKIPS.with(|count|count.get());
            let actual=filtered(procurement_import_offers(&w,n));
            let expected=filtered(public);
            assert_eq!(serde_json::to_vec(&actual).unwrap(),serde_json::to_vec(&expected).unwrap(),"{n:?}/{case}");
            assert_eq!(offer_bits(&actual),offer_bits(&expected),"exact quote float bits");
            let should_skip=n==HOME || matches!(case,1|6);
            assert_eq!(TEST_EMPTY_PROCUREMENT_IMPORT_SKIPS.with(|count|count.get())-skips,u64::from(should_skip));
            assert_eq!(serde_json::to_vec(&co::import_offers(&w,n)).unwrap(),public_bytes,"public catalogue unchanged");
            assert_eq!(crate::save(&w),before,"read-only prerequisite");
        }}
    }

    pub(super) fn with_ammunition_stock() -> WorldState {
        // Reuse the existing ordinary paid/certified supplier fixture. The
        // ammunition programme and stock below are earned by native work days.
        let (mut w,cid)=stocked();
        support(&mut w,HOME).unwrap();
        work_day(&mut w);work_day(&mut w);programs::begin_day(&mut w);
        let mut allowance=1.0;
        assert!(develop(&mut w,HOME,&mut allowance).unwrap().contains("finite compatible"));
        for _ in 0..60 {
            let p=&co::company(&w,HOME,cid).unwrap().ammunition_products[0];
            if p.stock>=p.stock_target {break;}
            work_day(&mut w);
        }
        programs::begin_day(&mut w);
        assert!(co::company(&w,HOME,cid).unwrap().ammunition_products[0].stock>0);
        support(&mut w,SMALL).unwrap();
        w.nation_mut(SMALL).program_budget.as_mut().unwrap().available_bn[DEF][3]=1.0;
        w
    }

    #[test]
    fn empty_import_precheck_preserves_full_procurement_decisions_spending_and_observation() {
        let base=with_ammunition_stock();
        for (case,nation,cap) in [(0,SMALL,1.0),(1,SMALL,1.0),(2,SMALL,1.0),
            (3,HOME,1.0),(4,SMALL,1.0),(0,SMALL,0.0)] {
            let mut actual=base.clone();
            match case {
                1=>{actual.companies.firms[0].products[0].stock=0;actual.companies.firms[0].products[0].stock_cost_bn=0.0;},
                2=>actual.companies.firms[0].products.clear(), // ammunition-only visible catalogue
                4=>actual.companies.firms[0].products[0].cancelled_day=Some(clock::absolute_day(&base)),
                _=>{},
            }
            if matches!(case,1|2) {
                assert!(co::import_offers(&actual,nation).iter().any(|o|o.ammunition&&o.ready_stock>0),
                    "public ammunition offers must remain available");
            }
            let mut original=actual.clone();
            let (mut actual_cap,mut old_cap)=(cap,cap);
            let before=TEST_EMPTY_PROCUREMENT_IMPORT_SKIPS.with(|count|count.get());
            let mut rows=Vec::new();
            let outcome=procure_observed(&mut actual,nation,&mut actual_cap,
                &mut Some(&mut |stage,country,_|rows.push((stage.to_owned(),country))));
            let expected=original_lists(||procure(&mut original,nation,&mut old_cap));
            assert_eq!(outcome,expected,"decision case {case}/{nation:?}/{cap}");
            assert_eq!(actual_cap.to_bits(),old_cap.to_bits());
            assert!(crate::save(&actual)==crate::save(&original),"complete world case {case}/{nation:?}/{cap}");
            assert_eq!(actual.headlines,original.headlines);
            assert_eq!(rows.iter().filter(|(stage,country)|stage=="review.procure.offers.import"&&*country==Some(nation)).count(),1,
                "the existing measured stage is retained even for a skipped empty catalogue");
            let should_skip=matches!(case,1|2|3);
            assert_eq!(TEST_EMPTY_PROCUREMENT_IMPORT_SKIPS.with(|count|count.get())-before,u64::from(should_skip));
            if case==0&&cap>0.0 {assert!(outcome.unwrap().starts_with("Bought 1"));}
        }
    }

    #[test]
    #[ignore="explicit actual-checkpoint original import-list parity; no timing assertions"]
    fn empty_import_precheck_matches_actual_checkpoint_for_31_complete_days() {
        let path=std::env::var("SPHERES_S22_CHECKPOINT").expect("actual checkpoint path required");
        let source=std::fs::read_to_string(&path).unwrap();
        let mut value:serde_json::Value=serde_json::from_str(&source).unwrap();
        let world=if value.get("format").and_then(serde_json::Value::as_str)==Some("spheres-campaign") {
            let world=value.get_mut("world").expect("campaign simulation payload").take();
            drop(value);world
        } else {value};
        let mut actual=crate::load_value(world).unwrap();
        assert!(actual.military_ai.enabled&&economic_ai::enabled(&actual));
        let mut original=actual.clone();
        let before=TEST_EMPTY_PROCUREMENT_IMPORT_SKIPS.with(|count|count.get());
        // Native complete days, no new player orders or budget renewal. This
        // is behavior equivalence, not the web qualification timing workload.
        for day in 0..31 {
            let a=crate::tick_day(&mut actual,&[]);
            let b=original_lists(||crate::tick_day(&mut original,&[]));
            assert_eq!(a,b,"returned headlines day {day}");
            assert_eq!(actual.headlines,original.headlines,"retained headlines day {day}");
            assert!(crate::save(&actual)==crate::save(&original),"complete native world day {day}");
        }
        let skipped=TEST_EMPTY_PROCUREMENT_IMPORT_SKIPS.with(|count|count.get())-before;
        assert!(skipped>0,"actual checkpoint must exercise the empty-equipment precondition");
        assert_eq!(std::fs::read_to_string(path).unwrap(),source,"source checkpoint remains immutable");
        eprintln!("31 native days matched the original complete import lists; {skipped} empty equipment catalogues skipped.");
    }
}
