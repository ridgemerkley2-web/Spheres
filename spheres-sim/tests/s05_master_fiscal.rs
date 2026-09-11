//! Small reproduction of the pinned master's retained ordinary receipt. The
//! fiscal row is exact USSR evidence from the recorded 1 Feb 1990 archive; the
//! surrounding world/empty contractor roster are explicitly synthetic. The
//! ignored web archive qualification separately uses the unchanged real file.
use serde_json::{json, Value};
use spheres_sim::{clock, fiscal_recovery as fiscal, init::world_1990, programs, Command};
use spheres_sim::world::{GameRules, NationId as N, WorldState};

fn original_master_case() -> Value {
    let mut w = world_1990(GameRules {
        daily_simulation: true,
        ai_aggression: 0.0,
        crisis_intensity: 0.0,
        ..Default::default()
    });
    w.player = Some(N::USSR);
    fiscal::enable(&mut w);
    let allocations = w.nation(N::USSR).budget_for(1990).allocations;
    spheres_sim::apply_command(&mut w, &Command::SetProgramBudget {
        nation: N::USSR, fiscal_year: 1990, allocations,
        departments: programs::default_departments(),
    }).unwrap();
    w.month = 2;
    w.day = 1;
    let n = w.nation_mut(N::USSR);
    n.gdp = 1604.5470383110583;
    n.debt_bn = Some(709.7198394359675);
    n.treasury_bn = Some(0.0);
    n.debt_gdp = n.debt_bn.unwrap() / n.gdp;
    let p = n.program_budget.as_mut().unwrap();
    p.day = Some(30);
    p.settled_day = Some(30);
    let mut source = serde_json::to_value(&w).unwrap();
    source["fiscal_recovery"]["nations"]["USSR"] = json!({
        "started_day":0,"last_day":30,"last_ai_review_day":null,"last_restructuring_day":null,
        "months_observed":0,"observed_month_fraction":0.9677419354838706,
        "stress_months":0,"improving_months":1,"confidence_pressure":0.0,
        "pending":{"day":0,"interest_bn":-0.019294422181844465,
            "revenue_bn":1.8438908966223846,"spending_bn":2.2260008549533814},
        "accumulating":null,"opening_debt_bn":720.0,"opening_gdp":1600.0,"opening_treasury_bn":0.0,
        "previous_adjustment":0.0,"previous_primary":0.08139148066025093,"previous_trend":-0.09525832594511778,
        "observations":[{"month":0,"opening_debt_bn":720.0,"opening_gdp":1600.0,"opening_treasury_bn":0.0,
            "closing_debt_bn":709.7198394359675,"closing_gdp":1604.5470383110583,"closing_treasury_bn":0.0,
            "gdp_years":129.22946816213224,"interest_bn":-0.28003229133688196,
            "revenue_bn":55.49883148933687,"spending_bn":44.980653730684175,
            "tax_gdp_years":51.691787264852884,"year_fraction":0.08064516129032255}]
    });
    // The complete original contractor dialect is the import authority. A
    // native economy/party/company wrapper cannot opt into this migration.
    source.as_object_mut().unwrap().remove("sector_contractors");
    source.as_object_mut().unwrap().remove("supplier_operations");
    source["companies"] = json!({"enabled":false,"roster":[],"assignments":[],"growth":{},
        "news":[],"next_id":0,"last_day":null,"last_month":null});
    source
}

fn nominal(w: &WorldState) -> Value {
    json!({"rng":w.rng,"date":[w.year,w.month,w.day],"nations":w.nations.iter()
        .map(|n|json!([n.id,n.gdp,n.debt_bn,n.treasury_bn,n.population])).collect::<Vec<_>>()})
}

#[test]
fn original_master_receipt_survives_load_without_backfilling_or_repaying() {
    let source = original_master_case();
    let text = source.to_string();
    let mut w = spheres_sim::load(&text).unwrap();
    let loaded = serde_json::to_value(&w).unwrap();
    assert_eq!(loaded["rng"],source["rng"]);
    assert_eq!(w.date_str(),"1 Feb 1990");
    let f = &w.fiscal_recovery.nations[&N::USSR];
    let retained = f.legacy_ordinary_receipt.as_ref().unwrap().clone();
    assert_eq!(retained.observed_through_day,30);
    assert_eq!(Some(&retained.receipt),f.pending.as_ref());
    let mut fiscal_after = loaded["fiscal_recovery"]["nations"]["USSR"].clone();
    fiscal_after.as_object_mut().unwrap().remove("legacy_ordinary_receipt");
    assert_eq!(fiscal_after,source["fiscal_recovery"]["nations"]["USSR"],
        "Import may add provenance but cannot rewrite any original fiscal property");
    let original_n = source["nations"].as_array().unwrap().iter().find(|n|n["id"]=="USSR").unwrap();
    let loaded_n = loaded["nations"].as_array().unwrap().iter().find(|n|n["id"]=="USSR").unwrap();
    assert_eq!(loaded_n,original_n,"Import cannot pay, enroll or reprice the government");
    let canonical = spheres_sim::save(&w);
    assert_eq!(spheres_sim::save(&spheres_sim::load(&canonical).unwrap()),canonical);
    let mut resumed = spheres_sim::load(&canonical).unwrap();
    let money = nominal(&w);
    let observations = w.fiscal_recovery.nations[&N::USSR].observations.clone();
    // No Feb 1 bill exists yet. Observing cannot backfill the day-0 receipt,
    // fabricate another month exposure, alter money, or advance the campaign.
    for state in [&mut w,&mut resumed] {
        fiscal::tick(state);
        assert_eq!(nominal(state),money);
        let f = &state.fiscal_recovery.nations[&N::USSR];
        assert!(f.pending.is_none());
        assert_eq!(f.legacy_ordinary_receipt.as_ref(),Some(&retained));
        assert_eq!(f.observations,observations);
        assert!(f.accumulating.is_none());
        assert_eq!(f.last_day,Some(30));
        assert_eq!(f.observed_month_fraction,0.9677419354838706);
        fiscal::validate(state).unwrap();
    }
    let before_day = clock::absolute_day(&w);
    spheres_sim::tick_day(&mut w,&[]);
    spheres_sim::tick_day(&mut resumed,&[]);
    assert_eq!(clock::absolute_day(&w),before_day+1);
    assert_eq!(spheres_sim::save(&w),spheres_sim::save(&resumed));
    assert_eq!(w.fiscal_recovery.nations[&N::USSR].legacy_ordinary_receipt,Some(retained));
    assert_eq!(spheres_sim::save(&spheres_sim::load(&spheres_sim::save(&w)).unwrap()),spheres_sim::save(&w));
    assert_eq!(source.to_string(),text,"Original source is immutable");
}

#[test]
fn retained_master_receipt_does_not_relax_current_or_corrupt_receipts() {
    let source = original_master_case();
    let good = spheres_sim::load(&source.to_string()).unwrap();
    for case in 0..4 {
        let mut bad = good.clone();
        let f = bad.fiscal_recovery.nations.get_mut(&N::USSR).unwrap();
        match case {
            0 => f.legacy_ordinary_receipt = None,
            1 => f.pending.as_mut().unwrap().spending_bn += 0.01,
            2 => f.legacy_ordinary_receipt.as_mut().unwrap().observed_through_day = 31,
            _ => f.legacy_ordinary_receipt.as_mut().unwrap().receipt.interest_bn = f64::INFINITY,
        }
        assert!(fiscal::validate(&bad).is_err(),"Accepted corrupt retained case {case}");
        if case<3 {assert!(spheres_sim::load(&spheres_sim::save(&bad)).is_err());}
    }
    for case in 0..4 {
        let mut bad = source.clone();
        match case {
            0 => bad["fiscal_recovery"]["nations"]["USSR"]["pending"]["spending_bn"] = json!(-1.0),
            1 => bad["fiscal_recovery"]["nations"]["USSR"]["pending"]["day"] = json!(32),
            2 => {
                let n = bad["nations"].as_array_mut().unwrap().iter_mut().find(|n|n["id"]=="USSR").unwrap();
                n["program_budget"] = Value::Null;
            }
            _ => {
                // A new wrapper with an original stale receipt receives no
                // master exception, even when other fiscal fields are valid.
                bad.as_object_mut().unwrap().remove("companies");
                bad = json!({"format":"spheres-economy-save","version":1,
                    "equipment_version":0,"party_leadership_version":0,"world":bad});
            }
        }
        assert!(spheres_sim::load(&bad.to_string()).is_err(),"Accepted invalid source case {case}");
    }
}
