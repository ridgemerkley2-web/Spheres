//! Completed laboratories buy specific, nontransferable prototype work.
//! All coefficients in this mechanic are game rules, not historical estimates.
use spheres_sim::{
    apply_command, clock, industry,
    init::world_1990,
    load, production, programs, save, tech,
    world::{GameRules, NationId, WorldState, BUDGET_SCIENCE},
    Command,
};

const USA: NationId = NationId::USA;

fn prepared(nation: NationId) -> (WorldState, String, u16) {
    let mut w = world_1990(GameRules {
        daily_simulation: true,
        production_system: true,
        resource_market: true,
        economic_competition: true,
        ai_aggression: 0.0,
        ..GameRules::default()
    });
    w.player = Some(nation);
    let year = w.year;
    let allocations = w.nation(nation).budget_for(year).allocations;
    apply_command(
        &mut w,
        &Command::SetProgramBudget {
            nation,
            fiscal_year: year,
            allocations,
            departments: programs::default_departments(),
        },
    )
    .unwrap();
    programs::begin_day(&mut w);
    let district = w
        .districts
        .iter()
        .find(|(_, n)| **n == nation)
        .unwrap()
        .0
        .clone();
    let t = tech::index_of("matl_industrial_robotics").unwrap();
    let n = w.nation_mut(nation);
    n.tech.known.retain(|known| *known != t);
    n.tech.focus.fill(None);
    n.tech.focus[tech::Domain::Materials.index()] = Some(t);
    n.tech.progress.fill(0.0);
    n.tech.allocation = Some([0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0]);
    w.production.industry.goods.insert(
        nation,
        industry::Goods {
            intermediates: 100.0,
            capital_goods: 100.0,
        },
    );
    (w, district, t)
}

fn center(w: &mut WorldState, district: &str, level: u8) {
    w.production
        .provinces
        .push(production::ProvinceCapabilities {
            district: district.into(),
            infrastructure: 0,
            civilian_industry: 0,
            power_grid: 0,
            research_centers: level,
            arms_plants: 0,
        });
    w.production
        .provinces
        .sort_by(|a, b| a.district.cmp(&b.district));
}

fn next_day(w: &mut WorldState) {
    programs::finish_day(w);
    clock::advance_date(w);
    programs::begin_day(w);
}

fn near(a: f64, b: f64) {
    assert!((a - b).abs() < 1e-10, "{a} != {b}");
}

#[test]
fn completed_center_has_a_paid_price_side_effect_not_a_research_multiplier() {
    let (mut with, district, t) = prepared(USA);
    let mut without = with.clone();
    center(&mut with, &district, 1);
    tech::tick(&mut with);
    tech::tick(&mut without);
    assert!(
        tech::cost_of(&with, USA, t) < tech::cost_of(&without, USA, t),
        "a completed supplied laboratory must do useful prototype work"
    );
    assert_eq!(
        with.nation(USA).tech.research_total,
        without.nation(USA).tech.research_total,
        "Science must not multiply Education's research output"
    );
    assert_eq!(
        with.nation(USA).gdp,
        without.nation(USA).gdp,
        "a laboratory is not a direct GDP bonus"
    );
    assert!(
        programs::available_bn(&with, USA, BUDGET_SCIENCE, 0)
            < programs::available_bn(&without, USA, BUDGET_SCIENCE, 0)
    );
    assert!(
        with.production.industry.goods[&USA].capital_goods
            < without.production.industry.goods[&USA].capital_goods
    );
}

#[test]
fn unavailable_research_inputs_pause_without_partial_charges() {
    for missing in ["budget", "authority", "intermediates", "capital_goods"] {
        let (mut w, d, t) = prepared(USA);
        center(&mut w, &d, 1);
        match missing {
            "budget" => w.nation_mut(USA).program_budget = None,
            "authority" => {
                let p = w.nation_mut(USA).program_budget.as_mut().unwrap();
                p.available_bn[BUDGET_SCIENCE][0] = 0.0;
                p.prepaid_bn[BUDGET_SCIENCE][0] = 0.0;
            }
            "intermediates" => {
                w.production
                    .industry
                    .goods
                    .get_mut(&USA)
                    .unwrap()
                    .intermediates = 0.0
            }
            _ => {
                w.production
                    .industry
                    .goods
                    .get_mut(&USA)
                    .unwrap()
                    .capital_goods = 0.0
            }
        }
        let goods = w.production.industry.goods.clone();
        let plan = w.nation(USA).program_budget.clone();
        let cost = tech::cost_of(&w, USA, t);
        industry::research_day(&mut w);
        assert_eq!(w.production.industry.goods, goods, "missing {missing}");
        assert_eq!(w.nation(USA).program_budget, plan, "missing {missing}");
        assert_eq!(tech::cost_of(&w, USA, t), cost);
        let op = &industry::research_status(&w, USA)[0];
        assert_eq!(
            op.status,
            if missing == "budget" { "blocked" } else { "paused" }
        );
        assert_eq!(op.prototype_credit, 0.0);
    }
}

#[test]
fn unfinished_centers_and_absent_or_ineligible_focus_never_buy_work() {
    let (mut w, d, _) = prepared(USA);
    w.production.projects.push(production::Project {
        id: 1,
        nation: USA,
        district: d.clone(),
        kind: production::ProjectKind::ResearchCenter,
        priority: production::Priority::Normal,
        status: production::ProjectStatus::Building,
        reason: None,
        progress_days: 599.0,
        total_days: 600,
        resources_used: [0.0; 12],
        capacity_micros: None,
        started_day: None,
    });
    let before = save(&w);
    industry::research_day(&mut w);
    assert_eq!(
        save(&w),
        before,
        "599 of 600 days is not completed capacity"
    );
    for mode in [
        "no_focus",
        "future_year",
        "prerequisites",
        "zero_allocation",
        "fully_banked",
    ] {
        let (mut w, d, t) = prepared(USA);
        center(&mut w, &d, 1);
        let di = tech::Domain::Materials.index();
        match mode {
            "no_focus" => w.nation_mut(USA).tech.focus.fill(None),
            "future_year" => w.year = 1989,
            "prerequisites" => {
                let candidate = tech::registry()
                    .iter()
                    .enumerate()
                    .find(|(i, def)| {
                        def.domain == tech::Domain::Materials
                            && def.earliest_year <= 1990
                            && !tech::prereqs_of(*i as u16).is_empty()
                    })
                    .unwrap()
                    .0 as u16;
                w.nation_mut(USA).tech.known.clear();
                w.nation_mut(USA).tech.focus[di] = Some(candidate);
            }
            "zero_allocation" => {
                w.nation_mut(USA).tech.allocation = Some([1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0])
            }
            _ => w.nation_mut(USA).tech.progress[di] = tech::cost_of(&w, USA, t),
        }
        let goods = w.production.industry.goods.clone();
        let plan = w.nation(USA).program_budget.clone();
        assert_eq!(industry::research_work_demand(&w, USA), 0.0, "{mode}");
        industry::research_day(&mut w);
        assert_eq!(w.production.industry.goods, goods, "{mode}");
        assert_eq!(w.nation(USA).program_budget, plan, "{mode}");
        assert_eq!(
            industry::research_status(&w, USA)[0].prototype_credit,
            0.0,
            "{mode}"
        );
    }
}

#[test]
fn target_switch_keeps_credit_on_its_original_technology() {
    let (mut w, d, first) = prepared(USA);
    center(&mut w, &d, 1);
    industry::research_day(&mut w);
    let first_credit = w.production.industry.research[&USA].credits[&first];
    let second = tech::index_of("matl_lean_production").unwrap();
    w.nation_mut(USA).tech.known.retain(|t| *t != second);
    let second_cost = tech::cost_of(&w, USA, second);
    apply_command(
        &mut w,
        &Command::SetResearchFocus {
            nation: USA,
            domain: tech::Domain::Materials,
            tech: Some("matl_lean_production".into()),
        },
    )
    .unwrap();
    assert_eq!(tech::cost_of(&w, USA, second), second_cost);
    assert_eq!(
        w.production.industry.research[&USA].credits.get(&second),
        None
    );
    next_day(&mut w);
    industry::research_day(&mut w);
    assert!(w.production.industry.research[&USA].credits[&second] > 0.0);
    assert_eq!(
        w.production.industry.research[&USA].credits[&first],
        first_credit
    );
}

#[test]
fn captured_building_uses_new_owners_budget_but_never_steals_credits() {
    let (mut w, d, t) = prepared(USA);
    center(&mut w, &d, 1);
    industry::research_day(&mut w);
    let credit = w.production.industry.research[&USA].credits[&t];
    let (other, _, _) = prepared(NationId::India);
    w.nation_mut(NationId::India).program_budget =
        other.nation(NationId::India).program_budget.clone();
    w.nation_mut(NationId::India).tech = other.nation(NationId::India).tech.clone();
    w.production.industry.goods.insert(
        NationId::India,
        industry::Goods {
            intermediates: 10.0,
            capital_goods: 10.0,
        },
    );
    w.districts.insert(d.clone(), NationId::India);
    industry::research_day(&mut w);
    assert!(
        !w.production
            .industry
            .research
            .contains_key(&NationId::India),
        "one building cannot operate twice on capture day"
    );
    next_day(&mut w);
    let old_goods = w.production.industry.goods[&USA].clone();
    industry::research_day(&mut w);
    assert_eq!(w.production.industry.research[&USA].credits[&t], credit);
    assert_eq!(w.production.industry.goods[&USA], old_goods);
    let new_op = &industry::research_status(&w, NationId::India)[0];
    assert_eq!(new_op.district, d);
    assert!(new_op.prototype_credit > 0.0);
    near(
        w.production.industry.research[&NationId::India].credits[&t],
        new_op.prototype_credit,
    );
    assert!(industry::research_status(&w, USA).is_empty());
}

#[test]
fn contested_laboratory_cannot_spend_even_with_legal_ownership() {
    let (mut w, d, _) = prepared(USA);
    center(&mut w, &d, 1);
    spheres_sim::war::declare_war(&mut w, NationId::Iraq, NationId::Kuwait).unwrap();
    w.conflicts[0].front.insert(d, -1.0);
    let goods = w.production.industry.goods.clone();
    let plan = w.nation(USA).program_budget.clone();
    industry::research_day(&mut w);
    assert_eq!(w.production.industry.goods, goods);
    assert_eq!(w.nation(USA).program_budget, plan);
    assert!(industry::research_status(&w, USA)[0]
        .reason
        .contains("uncontested"));
}

#[test]
fn many_centers_share_effort_and_stop_at_the_useful_remaining_bill() {
    let (mut w, _, t) = prepared(USA);
    let districts: Vec<_> = w
        .districts
        .iter()
        .filter(|(_, n)| **n == USA)
        .take(20)
        .map(|(d, _)| d.clone())
        .collect();
    for d in districts {
        center(&mut w, &d, 5);
    }
    let cost = tech::cost_of(&w, USA, t);
    let output = tech::research_output(&w, w.nation(USA), tech::dev_of(w.nation(USA)))
        * clock::month_fraction(&w);
    let almost = cost * industry::PROTOTYPE_COST_SHARE - 0.000001;
    w.production
        .industry
        .research
        .entry(USA)
        .or_default()
        .credits
        .insert(t, almost);
    industry::research_day(&mut w);
    let ops = industry::research_status(&w, USA);
    let earned: f64 = ops.iter().map(|o| o.prototype_credit).sum();
    near(earned, 0.000001);
    assert!(earned <= output * industry::PROTOTYPE_EFFORT_SHARE);
    near(tech::cost_of(&w, USA, t), cost * 0.8);
    next_day(&mut w);
    let inventory = w.production.industry.goods.clone();
    industry::research_day(&mut w);
    assert_eq!(
        w.production.industry.goods, inventory,
        "fully funded prototype cap must not keep spending"
    );
    // A changed world price does not refund excess work or make acquisition free.
    near(
        industry::research_cost(&w, USA, t, cost * 0.01),
        cost * 0.008,
    );
}

#[test]
fn quote_and_real_research_charge_use_the_same_credit() {
    let nation = NationId::EquatorialGuinea;
    let (mut w, d, t) = prepared(nation);
    center(&mut w, &d, 1);
    industry::research_day(&mut w);
    let quote = tech::cost_of(&w, nation, t);
    let di = tech::Domain::Materials.index();
    let (_, _, card_price) = tech::project_of(&w, nation, tech::Domain::Materials).unwrap();
    assert_eq!(quote, card_price);
    w.nation_mut(nation).tech.progress[di] = quote;
    let output = tech::research_output(&w, w.nation(nation), tech::dev_of(w.nation(nation)))
        * clock::month_fraction(&w);
    let goods = w.production.industry.goods.clone();
    tech::tick(&mut w);
    assert!(w.nation(nation).tech.knows_index(t));
    near(w.nation(nation).tech.progress[di], output);
    assert_eq!(
        w.production.industry.goods, goods,
        "quote/charge must not settle labs twice"
    );
}

#[test]
fn save_resume_and_repeated_reads_or_ticks_do_not_duplicate_work() {
    let (mut a, d, _) = prepared(USA);
    center(&mut a, &d, 2);
    industry::research_day(&mut a);
    let before = save(&a);
    let mut b = load(&before).unwrap();
    assert_eq!(save(&b), before);
    for _ in 0..3 {
        let _ = industry::research_status(&a, USA);
        let _ = industry::research_goods_demand(&a, USA);
        let _ = industry::snapshot(&a, USA);
        industry::research_day(&mut a);
    }
    assert_eq!(save(&a), before);
    for _ in 0..8 {
        next_day(&mut a);
        next_day(&mut b);
        industry::research_day(&mut a);
        industry::research_day(&mut b);
        tech::tick(&mut a);
        tech::tick(&mut b);
        assert_eq!(save(&a), save(&b));
    }
}

#[test]
fn default_and_monthly_paths_are_byte_inert_even_with_completed_labs() {
    for (enabled, daily) in [(false, true), (false, false), (true, false)] {
        let (mut w, d, _) = prepared(USA);
        center(&mut w, &d, 1);
        w.rules.economic_competition = enabled;
        w.rules.daily_simulation = daily;
        let before = save(&w);
        industry::research_day(&mut w);
        assert_eq!(save(&w), before);
        assert!(industry::research_status(&w, USA).is_empty());
        assert_eq!(
            industry::research_goods_demand(&w, USA),
            industry::Goods::default()
        );
        assert!(!save(&w).contains("\"research_day\""));
    }
}

#[test]
fn finite_partial_work_for_small_and_large_nations_and_no_shortage_hiding() {
    for nation in [
        USA,
        NationId::China,
        NationId::India,
        NationId::EquatorialGuinea,
    ] {
        let (mut w, d, _) = prepared(nation);
        center(&mut w, &d, 1);
        let expected = industry::research_goods_demand(&w, nation);
        assert!(
            expected.capital_goods.is_finite() && expected.capital_goods > 0.0,
            "{nation:?}"
        );
        w.production
            .industry
            .goods
            .get_mut(&nation)
            .unwrap()
            .capital_goods = 0.0;
        assert_eq!(
            industry::research_goods_demand(&w, nation),
            expected,
            "a missing supply must not erase demand"
        );
        w.production
            .industry
            .goods
            .get_mut(&nation)
            .unwrap()
            .capital_goods = expected.capital_goods / 2.0;
        industry::research_day(&mut w);
        let op = &industry::research_status(&w, nation)[0];
        assert!(op.prototype_credit.is_finite() && op.prototype_credit > 0.0);
        assert!(op.cash_spent_daily_bn.is_finite() && op.cash_spent_daily_bn > 0.0);
        assert_eq!(op.status, "limited");
        near(op.goods_used.capital_goods, expected.capital_goods / 2.0);
        assert!(w.production.industry.goods[&nation].capital_goods >= 0.0);
    }
}

#[test]
fn gdp_discloses_actual_service_but_never_values_credit_as_output() {
    let (mut w, d, _) = prepared(USA);
    center(&mut w, &d, 1);
    spheres_sim::province_economy::enable(&mut w);
    spheres_sim::province_economy::begin_day(&mut w);
    industry::research_day(&mut w);
    let op = &industry::research_status(&w, USA)[0];
    let rows = spheres_sim::gdp_projects::contributions(&w);
    let row = rows
        .iter()
        .find(|r| r.district == d && r.kind == "research_center")
        .unwrap();
    assert!(!row.counted);
    assert_eq!(row.classification, "enabling_asset");
    assert_eq!(row.annual_gdp_bn, 0.0);
    assert_eq!(row.daily_value_added_bn, 0.0);
    assert_eq!(row.gross_output_daily_bn, 0.0);
    assert_eq!(row.payments_daily_bn, op.cash_spent_daily_bn);
    assert_eq!(row.output_quantity_daily, op.prototype_credit);
    assert!(row.reason.as_ref().unwrap().contains("Prototype/testing"));
}

#[test]
fn acquired_robotics_opens_the_existing_industrial_upgrade_gate() {
    let nation = NationId::EquatorialGuinea;
    let (mut w, d, t) = prepared(nation);
    center(&mut w, &d, 1);
    w.production.provinces[0].civilian_industry = 1;
    w.production
        .industry
        .sites
        .insert(d.clone(), [1, 0, 0, 0, 0, 0, 0]);
    let kind = production::ProjectKind::Automation;
    assert!(industry::project_refusal(&w, nation, &d, kind)
        .unwrap()
        .contains("Industrial Robot Cells"));
    industry::research_day(&mut w);
    w.nation_mut(nation).tech.progress[tech::Domain::Materials.index()] =
        tech::cost_of(&w, nation, t);
    tech::tick(&mut w);
    assert!(w.nation(nation).tech.knows_index(t));
    assert_eq!(
        industry::project_refusal(&w, nation, &d, kind),
        None,
        "the real acquisition, not owning a lab, unlocks automation"
    );
    let snapshot = industry::snapshot(&w, nation);
    let site = snapshot
        .sites
        .iter()
        .find(|s| s.kind == production::ProjectKind::ResearchCenter)
        .unwrap();
    assert_eq!(site.status, "running");
    assert_eq!(
        site.output_daily, 0.0,
        "prototype credits must not masquerade as factory packs"
    );
}

#[test]
fn defeated_owner_has_no_operating_center_or_inheritable_science_credit() {
    let (mut w, d, t) = prepared(USA);
    center(&mut w, &d, 1);
    industry::research_day(&mut w);
    assert!(w.production.industry.research[&USA].credits[&t] > 0.0);
    w.nation_mut(USA).alive = false;
    next_day(&mut w);
    let goods = w.production.industry.goods.clone();
    industry::research_day(&mut w);
    assert_eq!(w.production.industry.goods, goods);
    assert!(w.production.industry.research[&USA].credits.is_empty());
    assert_eq!(industry::research_status(&w, USA)[0].status, "blocked");
}

#[test]
fn prototype_credits_save_stable_technology_ids_and_never_reassign_retired_ids() {
    let (mut w, d, t) = prepared(USA);
    center(&mut w, &d, 1);
    industry::research_day(&mut w);
    let programme = &w.production.industry.research[&USA];
    let mut encoded = serde_json::to_value(programme).unwrap();
    let credits = encoded["credits"].as_object_mut().unwrap();
    assert!(credits.contains_key("matl_industrial_robotics"));
    assert!(
        !credits.contains_key(&t.to_string()),
        "runtime registry positions must never be persisted as credit ownership"
    );
    credits.insert(
        "removed_technology_never_reassign_to_neighbor".into(),
        serde_json::json!(9000.0),
    );
    let decoded: industry::ResearchProgram = serde_json::from_value(encoded).unwrap();
    assert_eq!(decoded.credits, programme.credits);
    let loaded = load(&save(&w)).unwrap();
    assert_eq!(tech::cost_of(&loaded, USA, t), tech::cost_of(&w, USA, t));
    assert_eq!(save(&loaded), save(&w));
}
