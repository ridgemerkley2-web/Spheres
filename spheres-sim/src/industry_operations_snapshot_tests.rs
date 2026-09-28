//! Pure snapshot parity against the original per-province fresh ledger reads.
//! Synthetic fixture edits below never modify a qualification checkpoint.
use super::*;
use std::cell::Cell;

thread_local! { pub(super) static ELIMINATED: Cell<usize> = const { Cell::new(0) }; }

fn fixture() -> WorldState {
    let mut w = crate::init::world_1990(crate::world::GameRules {
        daily_simulation: true, production_system: true, resource_market: true,
        industry_rebuild: true, ..Default::default()
    });
    starting_industry::enable_new_world(&mut w).unwrap();
    crate::province_economy::enable(&mut w);
    crate::population::enable(&mut w).unwrap();
    w.player = Some(NationId::France);
    w.nation_mut(NationId::France).political_capital = 1000.0;
    let year = w.year;
    programs::install(&mut w, NationId::France, year, programs::default_departments());
    let district = w.districts.iter().find(|(_, n)| **n == NationId::France).unwrap().0.clone();
    for kind in [K::OfficeDistrict, K::AdvancedIndustry, K::Generation, K::PowerGrid] {
        production::complete_capability(&mut w, &district, kind);
    }
    w.production.industry.modules.insert(district, 375_000);
    w
}

fn assert_same(w: &WorldState, nation: NationId) -> Vec<u8> {
    let actual = serde_json::to_vec(&snapshot(w, nation)).unwrap();
    let original = serde_json::to_vec(&snapshot_impl(w, nation, false)).unwrap();
    assert_eq!(actual, original, "complete ordered native snapshot, including exact emitted number text: {nation:?}");
    actual
}

#[test]
fn connected_snapshot_reuses_exact_ledger_across_ownership_fractional_and_missing_states() {
    let base = fixture();
    let district = base.districts.iter().find(|(_, n)| **n == NationId::France).unwrap().0.clone();
    let before_count = ELIMINATED.with(Cell::get);
    for case in 0..9 {
        let mut w = base.clone();
        match case {
            1 => {
                w.districts.insert(district.clone(), NationId::UK);
                w.nation_mut(NationId::France).gdp *= 0.873;
                w.nation_mut(NationId::UK).gdp *= 1.137;
            }
            2 => {
                let assets = w.starting_industry.as_mut().unwrap().provinces.get_mut(&district).unwrap();
                for quantity in &mut assets.factory_equivalents { *quantity *= 0.375; }
            }
            3 => w.nation_mut(NationId::France).gdp = 0.0,
            4 => w.province_economy = None,
            5 => w.starting_industry = None,
            6 => w.nation_mut(NationId::France).alive = false,
            7 => { w.rules.daily_simulation = false; w.rules.industry_rebuild = false; }
            8 => { w.population_system.enabled = false; }
            _ => {}
        }
        let before = crate::save(&w);
        for nation in [NationId::France, NationId::UK, NationId::Tonga] { assert_same(&w, nation); }
        assert_eq!(crate::save(&w), before, "pure native views cannot edit any ledger, receipts or RNG: case {case}");
    }
    assert!(ELIMINATED.with(Cell::get) > before_count, "exercise real per-province redundant national queries");
}

#[test]
fn connected_snapshot_rebuilds_after_a_same_day_command() {
    let mut w = fixture();
    let nation = NationId::France;
    let before = assert_same(&w, nation);
    let date = (w.year, w.month, w.day);
    let old_rate = w.nation(nation).tax_rate;
    let rate = if old_rate < 0.59 { old_rate + 0.01 } else { old_rate - 0.01 };
    crate::apply_command(&mut w, &crate::Command::SetTaxRate { nation, rate }).unwrap();
    assert_eq!((w.year, w.month, w.day), date);
    let enacted = crate::save(&w);
    let after = assert_same(&w, nation);
    assert_ne!(after, before, "fresh inherited facility tax readings must reflect the enacted policy");
    assert_eq!(crate::save(&w), enacted, "reading after the command stays pure");
}

#[test]
#[ignore = "explicit immutable actual 2015/2035 snapshot parity; no timing or qualification claim"]
fn connected_snapshot_matches_actual_checkpoint_and_eliminates_redundant_national_reads() {
    let path = std::env::var("SPHERES_S22_CHECKPOINT").expect("actual 2015/2035 checkpoint path required");
    let source = std::fs::read_to_string(&path).unwrap();
    let mut value: serde_json::Value = serde_json::from_str(&source).unwrap();
    let world = if value.get("format").and_then(serde_json::Value::as_str) == Some("spheres-campaign") {
        let world = value.get_mut("world").expect("campaign simulation payload").take();
        drop(value); world
    } else { value };
    let w = crate::load_value(world).unwrap();
    assert!(w.year == 2015 || w.year == 2035);
    assert!(w.starting_industry.is_some() && w.province_economy.is_some());
    let nation = w.player.expect("actual player required");
    let before = crate::save(&w);
    let count = ELIMINATED.with(Cell::get);
    assert_same(&w, nation);
    let eliminated = ELIMINATED.with(Cell::get) - count;
    assert!(eliminated > 0, "actual snapshot must exercise inherited provincial rows");
    assert_eq!(crate::save(&w), before, "entire loaded world is unchanged");
    assert_eq!(std::fs::read_to_string(path).unwrap(), source, "immutable campaign bytes");
    eprintln!("Exact actual {} snapshot parity; {eliminated} repeated national ledger queries eliminated in one response.", w.year);
}
