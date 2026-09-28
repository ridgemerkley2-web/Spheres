use super::*;

fn compare(w: &WorldState, id: NationId) {
    let before = spheres_sim::save(w);
    let actual = serde_json::to_vec(&ministries_json(w, id)).unwrap();
    let expected = serde_json::to_vec(&original_ministries_json(w, id)).unwrap();
    assert_eq!(
        actual,
        expected,
        "{}: exact full ministry payload",
        id.code()
    );
    assert_eq!(
        spheres_sim::save(w),
        before,
        "{}: read changed world",
        id.code()
    );
}

#[test]
fn reused_ministry_samples_preserve_exact_bytes_across_countries_and_budgets() {
    let ids = [
        NationId::USA,
        NationId::France,
        NationId::Belgium,
        NationId::Brazil,
        NationId::Malta,
        NationId::Tonga,
        NationId::Iraq,
        NationId::Zaire,
    ];
    let mut w = world_1990(GameRules {
        daily_simulation: true,
        ..GameRules::default()
    });
    for id in ids {
        w.player = Some(id);
        compare(&w, id);
        programs::set_construction_budget(&mut w, id, 0.001).unwrap();
        compare(&w, id);
        // Disclosed synthetic view fixtures: exercise non-grid-aligned reference
        // settlements, saturation, empty departmental Industry, different fleet
        // shapes, and nonuniform department splits without advancing a campaign.
        let n = w.nation_mut(id);
        for (m, reference) in n
            .annual_budget
            .as_mut()
            .unwrap()
            .reference
            .iter_mut()
            .enumerate()
        {
            *reference = 0.013375 + m as f64 * 0.002133;
        }
        n.annual_budget.as_mut().unwrap().allocations = BUDGET_CAPS;
        for (m, departments) in n
            .program_budget
            .as_mut()
            .unwrap()
            .departments
            .iter_mut()
            .enumerate()
        {
            departments.fill(0);
            let slot = m % departments.len();
            departments[slot] = 10_000;
        }
        n.political_capital = 99.999999;
        n.stability = 0.001;
        n.gdp = if id == NationId::Malta {
            0.000001
        } else {
            n.gdp * 3.17
        };
        n.arsenal.held.truncate(1);
        compare(&w, id);
    }
    // The population feature changes the names and formulas of education and
    // welfare arms; preserve that alternate complete payload too.
    spheres_sim::population::enable(&mut w).unwrap();
    for id in ids {
        compare(&w, id);
    }
}

#[test]
#[ignore = "Read-only actual-checkpoint byte parity; set SPHERES_S22_INPUT and coordinate with timing runs"]
fn s22_ministry_curves_match_original_on_actual_checkpoint() {
    let path =
        std::path::PathBuf::from(std::env::var_os("SPHERES_S22_INPUT").expect("SPHERES_S22_INPUT"));
    let source = std::fs::read(&path).unwrap();
    let g = storage::decode(std::str::from_utf8(&source).unwrap()).unwrap();
    let player = g.world.player.expect("Actual checkpoint player");
    for id in [
        player,
        NationId::USA,
        NationId::UK,
        NationId::Belgium,
        NationId::Malta,
        NationId::Tonga,
    ] {
        if g.world.nation_opt(id).is_some() {
            compare(&g.world, id);
        }
    }
    assert_eq!(
        std::fs::read(path).unwrap(),
        source,
        "source checkpoint changed"
    );
    println!(
        "Exact complete ministry JSON parity and world/source immutability passed at {}",
        g.world.date_str()
    );
}

// The exact prior read model is the byte-level oracle; all formulas continue
// to come from the simulation in both versions.
fn original_ministries_json(w: &WorldState, me: NationId) -> serde_json::Value {
    use spheres_sim::ministries;
    let n = w.nation(me);
    let reference = ministries::reference_of(w, me);
    let ids = [
        "health",
        "education",
        "housing",
        "pensions",
        "infrastructure",
        "industry",
        "science",
        "defense",
        "security",
        "diplomacy",
    ];
    let names = [
        "Health",
        "Education",
        "Housing",
        "Welfare",
        "Infrastructure",
        "Industry & energy",
        "Science",
        "Defense",
        "Security",
        "Diplomacy",
    ];
    let at = |ministry: usize, share: f64| ministries::arms_at(w, n, ministry, share);
    // THE GRID IS ANCHORED ON THE REFERENCE, not on zero, and that is not a
    // detail. Every arm is a function of the GAP, and the dial moves in 0.005
    // steps FROM whatever the inherited settlement was -- 0.05375 of GDP for
    // Belgian health, which is not a multiple of anything. Sampled from zero,
    // the nearest sample to a freshly enacted budget was up to half a step
    // away, and the card read "+0.004pp of population" for a government that
    // had changed nothing. Anchored here, sample `MINISTRY_CURVE_ZERO` IS the
    // enacted settlement, every press lands exactly on a sample, and an unmoved
    // dial reads exactly zero.
    let sample = |m: usize, i: usize| {
        reference[m] + (i as f64 - MINISTRY_CURVE_ZERO as f64) * MINISTRY_CURVE_STEP
    };
    let list: Vec<serde_json::Value> = (0..spheres_sim::world::BUDGET_MINISTRIES)
        .map(|m| {
            let here = at(m, n.budget_for(w.year).allocations[m]);
            let arms: Vec<serde_json::Value> = here
                .iter()
                .enumerate()
                .map(|(a, arm)| {
                    let curve: Vec<f64> = (0..=MINISTRY_CURVE_STEPS)
                        .map(|i| round(at(m, sample(m, i))[a].value, 6))
                        .collect();
                    let per_point: Vec<f64> = (0..=MINISTRY_CURVE_STEPS)
                        .map(|i| {
                            let share = sample(m, i);
                            round(at(m, share + 0.01)[a].value - at(m, share)[a].value, 6)
                        })
                        .collect();
                    serde_json::json!({
                        "id": arm.id,
                        "name": arm.name,
                        "note": arm.note,
                        "kind": arm.kind.id(),
                        "curve": curve,
                        "per_point": per_point,
                    })
                })
                .collect();
            serde_json::json!({
                "index": m,
                "id": ids[m],
                "name": names[m],
                "cap": spheres_sim::world::BUDGET_CAPS[m],
                "reference": round(reference[m], 6),
                "arms": arms,
            })
        })
        .collect();
    serde_json::json!({
        "curve_step": MINISTRY_CURVE_STEP,
        "curve_steps": MINISTRY_CURVE_STEPS,
        "curve_zero": MINISTRY_CURVE_ZERO,
        "ministries": list,
    })
}
