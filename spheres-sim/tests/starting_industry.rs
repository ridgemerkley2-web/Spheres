//! Universal accounting and lifecycle invariants. These tests verify the GAME
//! conversion and honest coverage, not the historical accuracy of a census.
use spheres_sim::{
    clock, districts,
    init::world_1990,
    load, province_economy, save, starting_industry as inherited, tick_day,
    world::{start_nations, GameRules, NationId, WorldState},
};

fn fresh() -> WorldState {
    world_1990(GameRules {
        daily_simulation: true,
        ..GameRules::default()
    })
}
fn seeded() -> WorldState {
    let mut w = fresh();
    assert_eq!(inherited::enable_new_world(&mut w).unwrap(), true);
    province_economy::enable(&mut w);
    w
}
fn close(a: f64, b: f64) {
    assert!(
        (a - b).abs() <= 1e-10 * a.abs().max(b.abs()).max(1.0),
        "{a} != {b}"
    );
}
fn first_district(w: &WorldState, nation: NationId) -> String {
    w.districts
        .iter()
        .find(|(_, owner)| **owner == nation)
        .unwrap()
        .0
        .clone()
}

#[test]
fn dataset_covers_exactly_the_137_canonical_starters_with_valid_distinct_profiles() {
    let data = inherited::data();
    inherited::validate_data(data).unwrap();
    assert_eq!(data.countries.len(), 137);
    assert_eq!(data.countries.len(), start_nations().len());
    let min = data
        .countries
        .values()
        .map(|p| p.manufacturing_share)
        .fold(f64::INFINITY, f64::min);
    let max = data
        .countries
        .values()
        .map(|p| p.manufacturing_share)
        .fold(0.0, f64::max);
    assert!(
        max - min > 0.10,
        "A universal manufacturing share is not a country baseline"
    );
    for row in data.countries.values() {
        assert!(!row.source.is_empty() && !row.notes.is_empty());
        assert!(!row.share_quality.is_empty() && !row.mix_quality.is_empty());
        close(row.sector_weights.values().iter().sum(), 1.0);
    }
}

#[test]
fn dataset_rejects_missing_roster_invalid_shares_overlapping_mix_and_absent_provenance() {
    let baseline = inherited::data().clone();
    let mut missing = baseline.clone();
    missing.countries.remove(&NationId::USA);
    assert!(inherited::validate_data(&missing).is_err());
    let mut invalid = baseline.clone();
    invalid
        .countries
        .get_mut(&NationId::USA)
        .unwrap()
        .manufacturing_share = f64::NAN;
    assert!(inherited::validate_data(&invalid).is_err());
    let mut overlap = baseline.clone();
    overlap
        .countries
        .get_mut(&NationId::USA)
        .unwrap()
        .sector_weights
        .other = 1.0;
    assert!(inherited::validate_data(&overlap).is_err());
    let mut unsourced = baseline;
    unsourced
        .countries
        .get_mut(&NationId::USA)
        .unwrap()
        .source
        .clear();
    assert!(inherited::validate_data(&unsourced).is_err());
}

#[test]
fn structured_source_evidence_and_note_arrays_survive_the_saved_profile() {
    let input = serde_json::json!({
        "manufacturing_share":0.2, "share_quality":"explicit_test_fixture",
        "sector_weights":{"food_textiles":0.2,"materials":0.2,"chemicals":0.2,"machinery_electronics":0.2,"other":0.2},
        "mix_quality":"explicit_test_fixture",
        "source":{"indicator":"test-series","year":1990,"value":20.0,"caveat":"not a plant census"},
        "notes":["First caveat", "Second caveat"]
    });
    let profile: inherited::CountryProfile = serde_json::from_value(input.clone()).unwrap();
    assert_eq!(serde_json::from_str::<serde_json::Value>(&profile.source).unwrap(), input["source"]);
    assert_eq!(profile.notes, "First caveat\nSecond caveat");
    let saved = serde_json::to_string(&profile).unwrap();
    assert_eq!(serde_json::from_str::<inherited::CountryProfile>(&saved).unwrap(), profile);
}

#[test]
fn seeding_adds_only_frozen_estimate_records_not_gdp_cash_goods_or_work() {
    let mut w = fresh();
    let before = serde_json::to_value(&w).unwrap();
    inherited::enable_new_world(&mut w).unwrap();
    let mut after = serde_json::to_value(&w).unwrap();
    assert!(after
        .as_object_mut()
        .unwrap()
        .remove("starting_industry")
        .is_some());
    assert_eq!(
        after, before,
        "Registration must not write any other world field"
    );
    assert!(w.production.is_empty() && w.manufacturing.is_empty());
    assert_eq!(w.starting_industry.as_ref().unwrap().provinces.len(), 2584);
    assert_eq!(w.starting_industry.as_ref().unwrap().unallocated.len(), 6);
    assert_eq!(inherited::enable_new_world(&mut w).unwrap(), false);
}

#[test]
fn all_country_estimates_reconcile_to_existing_manufacturing_gdp_without_double_counting() {
    let w = seeded();
    let before = save(&w);
    for &id in start_nations() {
        let row = inherited::snapshot(&w, id).unwrap();
        let economy = province_economy::snapshot(&w, id).unwrap();
        let profile = &w.starting_industry.as_ref().unwrap().profiles[&id];
        assert_eq!(row.groups.len(), 5);
        close(
            row.opening_output_annual_bn,
            w.nation(id).gdp * profile.manufacturing_share,
        );
        close(row.current_output_annual_bn, row.opening_output_annual_bn);
        close(row.current_output_annual_bn, economy.sectors[2].gdp_bn);
        close(
            row.factory_equivalents
                * inherited::ANNUAL_CAPACITY_PER_EQUIVALENT_BN
                * inherited::STARTING_UTILIZATION,
            row.opening_output_annual_bn,
        );
        close(
            row.groups.iter().map(|g| g.factory_equivalents).sum(),
            row.factory_equivalents,
        );
        close(
            economy.sectors.iter().map(|s| s.gdp_bn).sum(),
            w.nation(id).gdp,
        );
        close(
            economy
                .provinces
                .iter()
                .map(|p| p.total_gdp_bn)
                .sum::<f64>()
                + economy.unallocated_gdp_bn,
            w.nation(id).gdp,
        );
        assert_eq!(economy.project_gdp_bn, 0.0);
        if row.factory_equivalents > 0.0 {
            close(row.utilization, 0.8);
        }
        for g in &row.groups {
            assert!(g.factory_equivalents.is_finite() && g.factory_equivalents >= 0.0);
            assert!(g.current_output_annual_bn.is_finite() && g.current_output_annual_bn >= 0.0);
        }
    }
    assert_eq!(before, save(&w), "Every read endpoint must be pure");
}

#[test]
fn microstates_retain_fractional_capacity_and_the_six_map_gaps_stay_unallocated() {
    let w = seeded();
    let tonga = inherited::snapshot(&w, NationId::Tonga).unwrap();
    assert!(tonga.factory_equivalents > 0.0 && tonga.factory_equivalents < 1.0);
    assert_eq!(tonga.unallocated_factory_equivalents, 0.0);
    assert!(tonga.province_count > 0);
    for id in [
        NationId::Bahrain,
        NationId::Mauritius,
        NationId::Seychelles,
        NationId::Comoros,
        NationId::CapeVerde,
        NationId::Maldives,
    ] {
        let row = inherited::snapshot(&w, id).unwrap();
        assert_eq!(row.province_count, 0);
        assert!(row.factory_equivalents > 0.0);
        close(row.factory_equivalents, row.unallocated_factory_equivalents);
        assert!(!w.districts.values().any(|owner| *owner == id));
    }
}

#[test]
fn economic_and_population_growth_never_respawn_or_resize_inherited_factories() {
    let mut w = seeded();
    let before = w.starting_industry.clone();
    let old = inherited::snapshot(&w, NationId::USA).unwrap();
    w.nation_mut(NationId::USA).gdp *= 2.0;
    w.nation_mut(NationId::USA).population *= 2.0;
    let next = inherited::snapshot(&w, NationId::USA).unwrap();
    assert_eq!(next.factory_equivalents, old.factory_equivalents);
    close(
        next.current_output_annual_bn,
        2.0 * old.current_output_annual_bn,
    );
    close(next.utilization, 1.6); // Do not hide demand above the frozen model capacity.
    assert_eq!(inherited::enable_new_world(&mut w).unwrap(), false);
    assert_eq!(w.starting_industry, before);
}

#[test]
fn inherited_records_follow_transferred_land_and_do_not_duplicate_in_the_old_owner() {
    let mut w = seeded();
    let d = first_district(&w, NationId::USA);
    let asset = inherited::province(&w, &d).unwrap();
    let old_usa = inherited::snapshot(&w, NationId::USA)
        .unwrap()
        .factory_equivalents;
    let old_canada = inherited::snapshot(&w, NationId::Canada)
        .unwrap()
        .factory_equivalents;
    let records = w.starting_industry.clone();
    districts::transfer_district(&mut w, NationId::USA, NationId::Canada, &d).unwrap();
    let transferred = inherited::province(&w, &d).unwrap();
    assert_eq!(transferred.nation, NationId::Canada);
    assert_eq!(transferred.origin, NationId::USA);
    assert_eq!(transferred.factory_equivalents, asset.factory_equivalents);
    close(
        inherited::snapshot(&w, NationId::USA)
            .unwrap()
            .factory_equivalents,
        old_usa - asset.factory_equivalents,
    );
    let canadian = inherited::snapshot(&w, NationId::Canada).unwrap();
    close(
        canadian.factory_equivalents,
        old_canada + asset.factory_equivalents,
    );
    assert!(canadian.sources.iter().any(|s| s.origin == NationId::USA));
    assert_eq!(
        w.starting_industry, records,
        "Ownership changes attribution, not the frozen records"
    );
    let economy = province_economy::snapshot(&w, NationId::Canada).unwrap();
    close(canadian.current_output_annual_bn, economy.sectors[2].gdp_bn);
}

#[test]
fn an_unmapped_country_keeps_its_unlocated_industry_when_it_acquires_a_province() {
    let mut w = seeded();
    let d = first_district(&w, NationId::Kuwait);
    let old = inherited::snapshot(&w, NationId::Bahrain).unwrap();
    districts::transfer_district(&mut w, NationId::Kuwait, NationId::Bahrain, &d).unwrap();
    let now = inherited::snapshot(&w, NationId::Bahrain).unwrap();
    let economy = province_economy::snapshot(&w, NationId::Bahrain).unwrap();
    assert_eq!(
        now.unallocated_factory_equivalents,
        old.unallocated_factory_equivalents
    );
    assert_eq!(now.province_count, 1);
    assert!(
        economy.unallocated_gdp_bn > 0.0,
        "Unmapped old output must not be silently moved into acquired land"
    );
    close(
        economy.unallocated_gdp_bn
            + economy
                .provinces
                .iter()
                .map(|p| p.total_gdp_bn)
                .sum::<f64>(),
        economy.total_gdp_bn,
    );
    close(now.current_output_annual_bn, economy.sectors[2].gdp_bn);
}

#[test]
fn old_campaigns_and_default_headless_worlds_do_not_receive_a_backfilled_endowment() {
    let legacy = world_1990(GameRules::default());
    let text = save(&legacy);
    assert!(!text.contains("starting_industry"));
    let mut loaded = load(&text).unwrap();
    assert!(inherited::enable_new_world(&mut loaded).is_err());
    assert_eq!(save(&loaded), text);
    clock::enable_daily_play(&mut loaded);
    province_economy::enable(&mut loaded);
    assert!(inherited::enable_new_world(&mut loaded).is_err());
    assert!(loaded.starting_industry.is_none());
    let mut later = fresh();
    later.year = 1995;
    let before = save(&later);
    assert!(inherited::enable_new_world(&mut later).is_err());
    assert_eq!(before, save(&later));
    let mut active = fresh();
    active.production.next_id = 1;
    assert!(inherited::enable_new_world(&mut active).is_err());
    assert!(active.starting_industry.is_none());
}

#[test]
fn registration_leaves_the_macro_timeline_unchanged_and_saves_resume_exactly() {
    let mut baseline = fresh();
    province_economy::enable(&mut baseline);
    let mut estimates = seeded();
    let mut resumed = load(&save(&estimates)).unwrap();
    assert_eq!(save(&estimates), save(&resumed));
    for _ in 0..3 {
        tick_day(&mut baseline, &[]);
        tick_day(&mut estimates, &[]);
        tick_day(&mut resumed, &[]);
        assert_eq!(save(&estimates), save(&resumed));
        for &id in start_nations() {
            assert_eq!(
                baseline.nation(id).gdp,
                estimates.nation(id).gdp,
                "{} must not receive another output/growth channel",
                id.code()
            );
            assert_eq!(
                baseline.nation(id).treasury_bn,
                estimates.nation(id).treasury_bn
            );
            assert_eq!(baseline.nation(id).debt_bn, estimates.nation(id).debt_bn);
        }
        assert_eq!(baseline.rng, estimates.rng);
    }
}
