//! Fresh browser startup adopts the integrated capabilities without a model
//! step. The separate loaded-play path retains explicit legacy opt-in.
use super::*;
use spheres_sim::{clock, company_network, operational_warfare, supplier_operations};
use std::collections::BTreeSet;

fn economy_ready(player: Option<NationId>) -> Game {
    let mut g = Game::new(1990, player);
    clock::enable_daily_play(&mut g.world);
    spheres_sim::starting_industry::enable_new_world(&mut g.world).unwrap();
    spheres_sim::starting_industry::enrich_new_world(&mut g.world).unwrap();
    play_rules(&mut g);
    spheres_sim::party_leadership::enable_campaign(&mut g.world).unwrap();
    spheres_sim::connected_economy::enable(&mut g.world).unwrap();
    g
}
fn json_difference(a: &serde_json::Value, b: &serde_json::Value, path: &str) -> Option<String> {
    if a == b { return None; }
    match (a, b) {
        (serde_json::Value::Object(a), serde_json::Value::Object(b)) => {
            let keys: BTreeSet<_> = a.keys().chain(b.keys()).collect();
            for key in keys {
                let path = format!("{path}.{key}");
                match (a.get(key), b.get(key)) {
                    (Some(a), Some(b)) => if let Some(diff) = json_difference(a, b, &path) { return Some(diff); },
                    _ => return Some(path),
                }
            }
        }
        (serde_json::Value::Array(a), serde_json::Value::Array(b)) => {
            if a.len() != b.len() { return Some(format!("{path}.length")); }
            for (index, (a, b)) in a.iter().zip(b).enumerate() {
                if let Some(diff) = json_difference(a, b, &format!("{path}[{index}]")) { return Some(diff); }
            }
        }
        _ => return Some(path.to_owned()),
    }
    None
}
fn same_json(a: serde_json::Value, b: serde_json::Value, context: &str) {
    assert!(a == b, "{context}: first changed path {:?}", json_difference(&a, &b, "$"));
}
fn startup_unrelated(w: &WorldState) -> serde_json::Value {
    let mut value = serde_json::to_value(w).unwrap();
    let object = value.as_object_mut().unwrap();
    // These are the only newly adopted owned books. Each is checked below for
    // grants or receipts; all remaining fields must match the S02 starting world.
    for key in ["sector_contractors", "supplier_operations", "supplier_catalogue", "military_ai", "campaign"] { object.remove(key); }
    object.get_mut("rules").unwrap().as_object_mut().unwrap().remove("operational_warfare");
    let mut resources = w.resources.clone();
    resources.market = None;
    // Empty resources are omitted by WorldState serde; normalize only that
    // representation so both sides still compare every other resource field.
    object.insert("resources".into(), serde_json::to_value(resources).unwrap());
    value
}
fn quoted_stocks(w: &WorldState) -> Vec<(NationId, Commodity, f64)> {
    w.nations.iter().filter(|n| n.alive).flat_map(|n| resources::ALL.into_iter()
        .map(move |commodity| (n.id, commodity, resources::stock_quantity(w, n.id, commodity)))).collect()
}
fn assert_existing_cover(w: &WorldState, before: &[(NationId, Commodity, f64)]) {
    for &(nation, commodity, quantity) in before {
        // The existing market writer canonically rounds table-unit stocks to
        // 1e-9. Compare exact ledger values, not an enlarged numeric tolerance.
        let canonical = (quantity.max(0.0) * 1e9).round() / 1e9;
        assert_eq!(resources::stock_quantity(w, nation, commodity), canonical,
            "opening ledger changed {:?} {:?} beyond its canonical representation", nation, commodity);
    }
}
fn assert_no_startup_settlement(w: &WorldState) {
    assert_eq!((w.year, w.month, w.day), (1990, 1, 1));
    assert_eq!(w.resources.last_tick_day, None);
    let market = w.resources.market.as_ref().expect("opening market must be materialized");
    assert_eq!(market.last_produced, i32::MIN);
    assert_eq!(market.last_cleared, i32::MIN);
    assert_eq!(market.last_produced_day, None);
    assert_eq!(market.last_cleared_day, None);
    assert_eq!(market.period_days, None);
    assert_eq!(market.cleared_volume, [0.0; 12]);
    assert_eq!(market.unmet_orders, [0.0; 12]);
    assert_eq!(market.prices, market.previous_prices);
    assert!(market.cash.is_empty() && market.fills.is_empty() && market.contract_fills.is_empty()
        && market.shipment_audits.is_empty() && market.contract_spend_bn.is_empty());
    assert!(w.companies.is_empty(), "startup cannot establish suppliers or grant corporate stock");
    assert_eq!(w.supplier_catalogue, spheres_sim::supplier_catalogue::Catalogue {
        enabled: true, plans: Default::default(),
    }, "fresh supplier adoption is only permission metadata; no managed company or scheduled order exists yet");
    assert!(w.military_ai.enabled && w.military_ai.plans.is_empty(), "staff enrollment cannot issue any order during startup");
    assert_eq!(w.companies.last_tick_day, None);
    assert!(w.supplier_operations.contracts.is_empty() && w.supplier_operations.receipts.is_empty());
    assert!(w.supplier_operations.grandfathered_units.is_empty()
        && w.supplier_operations.grandfathered_programs.is_empty()
        && w.supplier_operations.grandfathered_refits.is_empty());
    assert_eq!(w.supplier_operations.last_day, None);
    assert!(w.sector_contractors.assignments.is_empty());
    assert_eq!(w.sector_contractors.last_day, None);
    assert_eq!(w.sector_contractors.last_month, None);
    assert!(w.sector_contractors.roster.iter().all(|c|
        c.total_work == 0.0 && c.total_bonus == 0.0 && c.total_fees_bn == 0.0));
    assert!(!w.campaign.initialized && w.campaign.last_prepared.is_none());
    assert!(w.campaign.migration_conflicts.as_ref().is_some_and(BTreeSet::is_empty));
    assert!(w.campaign.orders.is_empty() && w.campaign.sectors.is_empty() && w.campaign.reserves.is_empty()
        && w.campaign.transfers.is_empty() && w.campaign.observations.is_empty() && w.campaign.reports.is_empty());
    assert!(w.campaign_supply.is_empty() && w.campaign_peace.is_empty());
}

#[test]
fn opening_market_preserves_legacy_cover_and_posts_nothing() {
    let mut g = economy_ready(Some(NationId::France));
    assert!(g.world.resources.market.is_none());
    assert!(g.world.resources.cover.is_empty());
    let mut partial = [6.0; 12];
    partial[Commodity::Iron.idx()] = 0.0;
    g.world.resources.cover.push(resources::Cover { nation: NationId::France, months: partial });
    let quoted = quoted_stocks(&g.world);
    let before = serde_json::to_value(&g.world).unwrap();
    resources::materialize_opening_market(&mut g.world).unwrap();
    assert_existing_cover(&g.world, &quoted);
    assert_eq!(resources::stock_quantity(&g.world, NationId::France, Commodity::Iron), 0.0);
    let mut after = serde_json::to_value(&g.world).unwrap();
    after["resources"].as_object_mut().unwrap().remove("market");
    same_json(before, after, "opening a physical ledger must not post any other state");
    let market = g.world.resources.market.as_ref().unwrap();
    assert_eq!((market.last_produced, market.last_cleared), (i32::MIN, i32::MIN));
    assert!(market.cash.is_empty() && market.fills.is_empty() && market.contract_fills.is_empty()
        && market.contract_spend_bn.is_empty() && market.shipment_audits.is_empty());
    assert!(market.last_produced_day.is_none() && market.last_cleared_day.is_none() && market.period_days.is_none());
    // An already-open and depleted physical stock is property, not a setup gap.
    g.world.resources.market.as_mut().unwrap().stocks.iter_mut().find(|s| s.quantity > 0.0).unwrap().quantity = 0.0;
    let before = serde_json::to_value(&g.world).unwrap();
    resources::materialize_opening_market(&mut g.world).unwrap();
    same_json(before, serde_json::to_value(&g.world).unwrap(), "reopening may not refill or reprice existing property");
}

#[test]
fn opening_market_refuses_a_disabled_market_without_changes() {
    let mut g = Game::new(1990, Some(NationId::France));
    let before = serde_json::to_value(&g.world).unwrap();
    assert!(resources::materialize_opening_market(&mut g.world).is_err());
    same_json(before, serde_json::to_value(&g.world).unwrap(), "disabled-market refusal must be atomic");
}

#[test]
fn fresh_start_adopts_all_capabilities_without_free_work_or_supplier_stock() {
    let base = economy_ready(Some(NationId::France));
    let quoted = quoted_stocks(&base.world);
    let mut fresh = Game::new(1990, Some(NationId::France));
    fresh_play_rules(&mut fresh).unwrap();
    assert!(fresh.world.rules.historical_party_leadership && fresh.world.rules.industry_rebuild
        && fresh.world.rules.fiscal_recovery && spheres_sim::population::active(&fresh.world));
    assert!(fresh.world.sector_contractors.enabled && !fresh.world.sector_contractors.roster.is_empty());
    assert!(supplier_operations::enabled(&fresh.world) && spheres_sim::campaign::enabled(&fresh.world));
    assert_eq!(fresh.world.supplier_operations.adopted_day, Some(0));
    assert_existing_cover(&fresh.world, &quoted);
    same_json(startup_unrelated(&base.world), startup_unrelated(&fresh.world),
        "S03/S04/S08 startup may only adopt their declared books and materialize existing cover");
    assert_no_startup_settlement(&fresh.world);
    assert!(fresh.advance_receipts.is_empty() && fresh.command_receipts.is_empty());
}

#[test]
fn fresh_startup_repeat_and_save_resume_preserve_exact_owned_state() {
    let mut fresh = Game::new(1990, Some(NationId::France));
    fresh_play_rules(&mut fresh).unwrap();
    let before = save(&fresh.world);
    fresh_play_rules(&mut fresh).unwrap();
    assert!(before == save(&fresh.world), "repeated fresh setup must be byte-identical");
    let loaded = loaded_play_game(load(&before).unwrap());
    assert!(before == save(&loaded.world), "save/resume must not reenroll or settle the unified startup");
    assert_no_startup_settlement(&loaded.world);
}

#[test]
fn setup_without_a_selected_nation_and_tiny_new_campaign_use_unified_rules() {
    let mut boot = Game::new(1990, None);
    fresh_play_rules(&mut boot).unwrap();
    assert!(company_network::has_state(&boot.world) && operational_warfare::has_state(&boot.world));
    assert_no_startup_settlement(&boot.world);
    let (_, started) = new_game(&mut boot, 1990, Some(NationId::Tonga));
    assert!(started);
    assert_eq!(boot.world.player, Some(NationId::Tonga));
    assert!(boot.world.sector_contractors.enabled && supplier_operations::enabled(&boot.world)
        && spheres_sim::campaign::enabled(&boot.world));
    assert_no_startup_settlement(&boot.world);
}

#[test]
fn legacy_load_keeps_company_and_operational_upgrades_opt_in() {
    let legacy = Game::new(1990, Some(NationId::France));
    let mut loaded = loaded_play_game(load(&save(&legacy.world)).unwrap());
    assert!(!loaded.world.rules.industry_rebuild && !loaded.world.rules.fiscal_recovery);
    assert!(!company_network::has_state(&loaded.world) && !operational_warfare::has_state(&loaded.world));
    assert!(loaded.world.resources.market.is_none());
    assert!(loaded.world.companies.is_empty() && loaded.world.sector_contractors.is_empty()
        && loaded.world.supplier_operations.is_empty());
    assert!(loaded.world.supplier_catalogue.is_empty());
    assert!(loaded.world.military_ai.is_empty());
    assert!(loaded.world.campaign.is_empty() && loaded.world.campaign_supply.is_empty()
        && loaded.world.campaign_peace.is_empty());
    let before = save(&loaded.world);
    play_rules(&mut loaded);
    assert!(before == save(&loaded.world), "ordinary browser adoption cannot enable new capabilities");
    assert_eq!((loaded.world.year, loaded.world.month, loaded.world.day), (1990, 1, 1));
}
