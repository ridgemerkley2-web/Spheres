#[test]
fn s08_missing_or_reidentified_supplier_retains_paid_import_before_any_buyer_mutation() {
    for reidentified in [false, true] {
        let (mut w, c, p) = stocked("ground_apc");
        book(&mut w, c, p, false, 1);
        end_day(&mut w);
        let due = w.companies.imports.contracts[0].due_day.unwrap();
        while clock::absolute_day(&w) < due {
            advance(&mut w);
        }
        let revision = format!("import-{c}-{p}");
        assert_eq!(imported_units(&w, &revision), 0);
        let contract = w.companies.imports.contracts[0].clone();
        assert!(contract.escrow_bn > 0.0);
        let buyer_before = serde_json::to_value(w.nation(BUYER)).unwrap();
        // Deliberately break the live source identity after real payment. Such
        // a save is still invalid; the dated owner must independently fail
        // closed before touching the buyer, whether the firm vanished or its
        // ID now belongs to another government's company.
        if reidentified {
            w.companies
                .firms
                .iter_mut()
                .find(|f| f.id == c)
                .unwrap()
                .nation = N::Germany;
        } else {
            w.companies.firms.retain(|f| f.id != c);
        }
        companies::tick_day(&mut w);
        assert_eq!(serde_json::to_value(w.nation(BUYER)).unwrap(), buyer_before);
        let retained = &w.companies.imports.contracts[0];
        assert_eq!(retained.status, "blocked");
        assert!(retained.reason.contains("original supplier identity"));
        assert_eq!(retained.escrow_bn, contract.escrow_bn);
        assert_eq!(retained.refunded_bn, 0.0);
        assert_eq!(retained.refunded_day, None);
        assert_eq!(retained.delivered_day, None);
        assert_eq!(retained.cancelled_day, None);
        assert_eq!(retained.due_day, Some(due + 1));
        assert_eq!(companies::inbound_units(&w, BUYER, &revision), 1);
        let after = spheres_sim::save(&w);
        companies::tick_day(&mut w);
        assert_eq!(spheres_sim::save(&w), after, "one delay per dated tick");
        assert!(
            spheres_sim::load(&after).is_err(),
            "the save validator must still reject a missing source"
        );
    }
}

#[test]
fn s08_market_affordability_matches_review_at_sub_ulp_funding_boundaries() {
    let (mut w, c, p) = stocked("ground_apc");
    w.player = Some(HOME);
    apply(
        &mut w,
        companies::CompanyOrder::Inventory {
            company: c,
            product: p,
            stock_target: 12,
        },
    );
    until_stock(&mut w, c, p, 12);
    w.player = Some(BUYER);
    programs::set_construction_budget(&mut w, BUYER, 0.0).unwrap();
    let quote = companies::import_purchase_quote(&w, BUYER, HOME, c, p, false, 1);
    let price = quote.unit_price_bn;
    assert!(price > 0.0);
    let mut witnessed_division_rounding = false;
    for quantity in 1..=12 {
        let cost = price * quantity as f64;
        let boundary = f64::from_bits(cost.to_bits() - 1);
        let plan = w.nation_mut(BUYER).program_budget.as_mut().unwrap();
        plan.available_bn[D][3] = boundary;
        plan.prepaid_bn[D][3] = 0.0;
        let before = spheres_sim::save(&w);
        let offer = companies::import_offers(&w, BUYER)
            .into_iter()
            .find(|o| o.company == c && o.product == p)
            .unwrap();
        assert_eq!(offer.purchase_available_bn, boundary);
        assert!(price * offer.affordable_quantity as f64 <= boundary);
        assert!(!companies::import_purchase_quote(&w, BUYER, HOME, c, p, false, quantity).valid);
        if offer.affordable_quantity > 0 {
            let review = companies::import_purchase_quote(
                &w,
                BUYER,
                HOME,
                c,
                p,
                false,
                offer.affordable_quantity,
            );
            assert!(review.valid, "{}: {:?}", quantity, review.reason);
        }
        witnessed_division_rounding |= (boundary / price).floor() >= quantity as f64;
        assert_eq!(spheres_sim::save(&w), before, "offers and reviews are pure");
    }
    assert!(
        witnessed_division_rounding,
        "fixture must exercise division rounding upward past a payable whole lot"
    );
}
