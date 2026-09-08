//! Optional military supply policy through dated commands and full daily ticks.
//! Fixture-only certified hardware, paid tooling, large copper recipe and seller
//! stock make a finite shortage explicit; none is a historical starting grant.
use spheres_sim::world::{GameRules, NationId as N, WorldState, BUDGET_DEFENSE as D};
use spheres_sim::{
    clock, equipment, logistics, production, programs, resources, Command, EquipmentOrder,
};

const BUYER: N = N::France;
const SELLER: N = N::Germany;
const COPPER: resources::Commodity = resources::Commodity::Copper;

fn command(order: EquipmentOrder) -> Command {
    Command::Equipment {
        nation: BUYER,
        order,
    }
}

fn policy(cap: f64, floor: f64, interval: u32, automatic: bool) -> Command {
    command(EquipmentOrder::SupplyPolicy {
        horizon_days: 365,
        spending_cap_bn: cap,
        cash_floor_bn: floor,
        review_interval_days: interval,
        automatic,
    })
}

fn step(w: &mut WorldState, commands: &[Command]) {
    let day = clock::absolute_day(w);
    let events = spheres_sim::tick_day(w, commands);
    assert!(
        !events.iter().any(|e| e.starts_with("[rejected]")),
        "{events:?}"
    );
    assert_eq!(clock::absolute_day(w), day + 1);
    equipment::validate_state(w.nation(BUYER)).unwrap();
}

fn fixture() -> WorldState {
    let mut w = spheres_sim::init::world_1990(GameRules {
        daily_simulation: true,
        military_operations: true,
        production_system: true,
        manufacturing_system: true,
        resource_market: true,
        resource_gates: true,
        logistics_routes: true,
        physical_logistics: true,
        ..Default::default()
    });
    w.player = Some(BUYER);
    programs::set_construction_budget(&mut w, BUYER, 0.0).unwrap();
    let spec = equipment::default_spec("ground_ifv");
    let mut profile = equipment::design_preview(&w, BUYER, &spec).profile.unwrap();
    profile.recipe = [0.0; 12];
    profile.recipe[COPPER.idx()] = 1_000_000.0;
    let day = clock::absolute_day(&w);
    let s = w
        .nation_mut(BUYER)
        .equipment
        .get_or_insert_with(Default::default);
    s.finance_from_day = day;
    s.revisions.insert(
        "supply-fixture".into(),
        equipment::DesignRevision {
            id: "supply-fixture".into(),
            name: "Synthetic supply-test IFV".into(),
            specification_key: equipment::specification_key(&spec),
            spec,
            profile,
            created_day: day,
            certified_day: Some(day),
        },
    );
    let district = w
        .districts
        .iter()
        .find(|(_, n)| **n == BUYER)
        .unwrap()
        .0
        .clone();
    if let Some(p) = w
        .production
        .provinces
        .iter_mut()
        .find(|p| p.district == district)
    {
        p.arms_plants = 3;
    } else {
        w.production
            .provinces
            .push(production::ProvinceCapabilities {
                district: district.clone(),
                infrastructure: 0,
                civilian_industry: 0,
                power_grid: 0,
                research_centers: 0,
                arms_plants: 3,
            });
        w.production
            .provinces
            .sort_by(|a, b| a.district.cmp(&b.district));
    }
    equipment::start_production(&mut w, BUYER, "supply-fixture", &district, 10, 0.5).unwrap();
    let project = &mut w.nation_mut(BUYER).equipment.as_mut().unwrap().projects[0];
    project.work_days = project.tooling_days as f64;
    project.spent_bn = project.tooling_cost_bn;
    // Keep the settlement a net invoice so testing a cash floor distinguishes
    // genuinely post-fiscal purchases from spending against unpaid bills.
    w.nation_mut(BUYER).tax_rate = 0.0;
    step(
        &mut w,
        &[command(EquipmentOrder::Maintenance {
            daily_budget_bn: 0.0,
        })],
    );
    let market = w.resources.market.as_mut().unwrap();
    for (nation, quantity) in [(BUYER, 0.0), (SELLER, 10_000_000.0)] {
        if let Some(stock) = market
            .stocks
            .iter_mut()
            .find(|s| s.nation == nation && s.commodity == COPPER)
        {
            stock.quantity = quantity;
        } else {
            market.stocks.push(resources::Stock {
                nation,
                commodity: COPPER,
                quantity,
                reserve_target: 0.0,
            });
        }
    }
    market.stocks.sort_by_key(|s| (s.nation, s.commodity));
    market.prices[COPPER.idx()] = 1000.0;
    for (id, cash) in [(BUYER, 100.0), (SELLER, 0.0)] {
        let n = w.nation_mut(id);
        n.treasury_bn = Some(cash);
        n.debt_bn = Some(0.0);
        n.debt_gdp = 0.0;
    }
    let q = equipment::replenishment_quote(&w, BUYER, 365, 0.001);
    assert!(q.valid, "{:?}", q.reason);
    assert!(q.requested[COPPER.idx()] > 0.0);
    w
}

fn automation(w: &WorldState) -> &equipment::EquipmentSupplyAutomation {
    w.nation(BUYER)
        .equipment
        .as_ref()
        .unwrap()
        .supply_automation
        .as_ref()
        .unwrap()
}

fn reviews(w: &WorldState) -> &[equipment::EquipmentSupplyReview] {
    w.nation(BUYER)
        .equipment
        .as_ref()
        .unwrap()
        .supply_automation
        .as_ref()
        .map_or(&[], |a| a.reviews.as_slice())
}

fn near(a: f64, b: f64, tolerance: f64) {
    assert!((a - b).abs() <= tolerance, "{a:.16e} != {b:.16e}");
}

#[test]
fn absent_and_disabled_policies_keep_the_daily_economy_identical() {
    let mut disabled = fixture();
    let mut absent = disabled.clone();
    assert!(!spheres_sim::save(&absent).contains("supply_automation"));
    step(&mut disabled, &[policy(0.001, 0.0, 1, false)]);
    step(&mut absent, &[]);
    for _ in 0..3 {
        step(&mut disabled, &[]);
        step(&mut absent, &[]);
    }
    assert!(reviews(&disabled).is_empty());
    let saved = spheres_sim::save(&disabled);
    assert_eq!(
        saved,
        spheres_sim::save(&spheres_sim::load(&saved).unwrap())
    );
    // A disabled preference is the only new state. No tax, factory, market,
    // shipment, fiscal or RNG change is attributable to merely saving it.
    disabled
        .nation_mut(BUYER)
        .equipment
        .as_mut()
        .unwrap()
        .supply_automation = None;
    disabled.headlines = absent.headlines.clone();
    assert_eq!(spheres_sim::save(&disabled), spheres_sim::save(&absent));
}

#[test]
fn prospective_purchase_uses_only_cash_above_the_floor_after_fiscal_settlement() {
    let mut w = fixture();
    let mut baseline = w.clone();
    step(&mut baseline, &[]);
    let before_due_cash = resources::raw_purchase_cash(&baseline, BUYER);
    step(&mut baseline, &[]);
    let settled_cash = resources::raw_purchase_cash(&baseline, BUYER);
    assert!(
        settled_cash < before_due_cash,
        "Fixture must have an unpaid net fiscal bill before the tail hook."
    );
    // Permit just $1.60 of post-bill cash under an otherwise generous cap.
    let floor = settled_cash - 1.6e-9;
    let authorized = clock::absolute_day(&w);
    step(&mut w, &[policy(1.0, floor, 1, true)]);
    assert!(
        reviews(&w).is_empty(),
        "Authorization cannot purchase on its own date."
    );
    step(&mut w, &[]);
    let review = &reviews(&w)[0];
    assert_eq!(review.day, authorized + 1);
    let purchase = review
        .purchase
        .as_ref()
        .expect("The remaining cash can pay one whole dollar.");
    near(purchase.spent_bn, 1e-9, 1e-15);
    near(
        settled_cash - resources::raw_purchase_cash(&w, BUYER),
        purchase.spent_bn,
        1e-12,
    );
    assert!(resources::raw_purchase_cash(&w, BUYER) >= floor - 1e-12);
    assert_eq!(w.nation(BUYER).debt_bn, baseline.nation(BUYER).debt_bn);
    assert_eq!(
        w.nation(BUYER).program_budget,
        baseline.nation(BUYER).program_budget,
        "Raw purchases must not duplicate D2/D3 fabrication expenses."
    );
    for d in [2, 3] {
        assert_eq!(
            w.nation(BUYER)
                .program_budget
                .as_ref()
                .unwrap()
                .spent_today_bn[D][d],
            baseline
                .nation(BUYER)
                .program_budget
                .as_ref()
                .unwrap()
                .spent_today_bn[D][d]
        );
    }
}

#[test]
fn weekly_reviews_and_save_resume_preserve_one_cap_per_due_date() {
    let mut continuous = fixture();
    let authorized = clock::absolute_day(&continuous);
    step(&mut continuous, &[policy(0.001, 0.0, 7, true)]);
    assert!(reviews(&continuous).is_empty());
    step(&mut continuous, &[]);
    assert_eq!(reviews(&continuous).len(), 1);
    assert_eq!(reviews(&continuous)[0].day, authorized + 1);
    assert!(reviews(&continuous)[0].purchase.is_some());
    let checkpoint = spheres_sim::save(&continuous);
    let mut resumed = spheres_sim::load(&checkpoint).unwrap();
    assert_eq!(checkpoint, spheres_sim::save(&resumed));
    for _ in 0..6 {
        step(&mut continuous, &[]);
        step(&mut resumed, &[]);
        assert_eq!(
            reviews(&continuous).len(),
            1,
            "Unused daily limits are not additional review authority."
        );
        assert_eq!(
            spheres_sim::state_hash(&continuous),
            spheres_sim::state_hash(&resumed)
        );
    }
    step(&mut continuous, &[]);
    step(&mut resumed, &[]);
    assert_eq!(reviews(&continuous).len(), 2);
    assert_eq!(reviews(&continuous)[1].day, authorized + 8);
    for r in reviews(&continuous) {
        if let Some(p) = &r.purchase {
            assert!(p.spent_bn <= 0.001 + 1e-12);
        }
    }
    assert_eq!(spheres_sim::save(&continuous), spheres_sim::save(&resumed));
    let after = spheres_sim::save(&continuous);
    equipment::tick_supply_automation(&mut continuous);
    assert_eq!(
        spheres_sim::save(&continuous),
        after,
        "A repeated tail hook cannot repeat the last paid review."
    );
}

#[test]
fn paid_late_cargo_reduces_need_and_policy_edit_or_clear_keeps_its_history() {
    let mut w = fixture();
    step(&mut w, &[policy(0.001, 0.0, 1, true)]);
    step(&mut w, &[]);
    let bought = reviews(&w)[0].purchase.as_ref().unwrap().purchased[COPPER.idx()];
    assert!(bought > 0.0);
    let dispatch_day = reviews(&w)[0].day;
    let delayed_until = clock::absolute_day(&w) + 400;
    let cargo_ids: Vec<_> = w
        .logistics
        .cargo
        .iter_mut()
        .filter(|c| {
            c.buyer == BUYER && c.commodity == COPPER && c.dispatched_day == Some(dispatch_day)
        })
        .map(|c| {
            c.due_day = Some(delayed_until);
            c.id
        })
        .collect();
    assert!(!cargo_ids.is_empty());
    let paid = w
        .logistics
        .cargo
        .iter()
        .filter(|c| cargo_ids.contains(&c.id))
        .map(|c| c.quantity)
        .sum::<f64>();
    let with_paid = equipment::replenishment_quote(&w, BUYER, 365, 0.001);
    let mut without_paid = w.clone();
    without_paid
        .logistics
        .cargo
        .retain(|c| !cargo_ids.contains(&c.id));
    let without_paid = equipment::replenishment_quote(&without_paid, BUYER, 365, 0.001);
    near(
        with_paid.requested[COPPER.idx()],
        (without_paid.requested[COPPER.idx()] - paid).max(0.0),
        1e-6,
    );
    let history = serde_json::to_string(reviews(&w)).unwrap();
    step(&mut w, &[policy(0.0, 10.0, 30, false)]);
    assert_eq!(serde_json::to_string(reviews(&w)).unwrap(), history);
    step(&mut w, &[command(EquipmentOrder::SupplyPolicyClear)]);
    assert!(automation(&w).policy.is_none());
    assert_eq!(serde_json::to_string(reviews(&w)).unwrap(), history);
    for id in cargo_ids {
        let c = w
            .logistics
            .cargo
            .iter()
            .find(|c| c.id == id)
            .expect("Policy changes cannot erase paid freight.");
        assert_eq!(c.due_day, Some(delayed_until));
        assert!(c.quantity > 0.0);
    }
    assert!(logistics::pending(&w, BUYER, COPPER) >= bought);
    let saved = spheres_sim::save(&w);
    assert_eq!(
        saved,
        spheres_sim::save(&spheres_sim::load(&saved).unwrap())
    );
}

#[test]
fn an_unbuilt_fleet_target_is_not_a_material_purchase_contract() {
    let mut w = fixture();
    let project = w.nation(BUYER).equipment.as_ref().unwrap().projects[0].id;
    step(
        &mut w,
        &[
            command(EquipmentOrder::Cancel { project }),
            command(EquipmentOrder::Target {
                revision: "supply-fixture".into(),
                quantity: Some(100_000),
            }),
            policy(1.0, 0.0, 1, true),
        ],
    );
    for _ in 0..3 {
        step(&mut w, &[]);
    }
    assert_eq!(reviews(&w).len(), 3);
    for r in reviews(&w) {
        assert!(r.purchase.is_none());
        assert!(r.requested.iter().all(|v| *v == 0.0));
    }
    let s = w.nation(BUYER).equipment.as_ref().unwrap();
    assert_eq!(s.projects.len(), 1);
    assert_eq!(s.projects[0].status, equipment::ProjectStatus::Cancelled);
    assert_eq!(s.fleet_targets["supply-fixture"], 100_000);
}

#[test]
fn cash_blocked_reviews_wait_the_interval_and_overdue_reviews_do_not_catch_up() {
    let mut w = fixture();
    step(&mut w, &[policy(0.001, 1_000.0, 7, true)]);
    step(&mut w, &[]);
    assert_eq!(reviews(&w).len(), 1);
    let first = reviews(&w)[0].day;
    assert!(reviews(&w)[0].purchase.is_none());
    assert_eq!(reviews(&w)[0].effective_cap_bn, 0.0);
    // Explicit fixture windfall: recovery of cash cannot turn a previously
    // blocked weekly review into a new daily spending instruction.
    w.nation_mut(BUYER).treasury_bn = Some(10_000.0);
    for _ in 0..6 {
        step(&mut w, &[]);
        assert_eq!(reviews(&w).len(), 1);
    }
    step(&mut w, &[]);
    assert_eq!(reviews(&w).len(), 2);
    assert_eq!(reviews(&w)[1].day, first + 7);
    assert_eq!(reviews(&w)[1].effective_cap_bn, 0.001);

    // Exercise a persisted overdue policy against a later game date. Missed
    // reviews authorize neither a burst of purchases nor banked spending caps.
    let overdue = first + 52;
    let (year, month, day) = clock::date_from_day(overdue);
    w.year = year;
    w.month = month;
    w.day = day;
    step(&mut w, &[]);
    assert_eq!(reviews(&w).len(), 3);
    let last = reviews(&w).last().unwrap();
    assert_eq!(last.day, overdue);
    assert!(last.effective_cap_bn <= 0.001);
    if let Some(p) = &last.purchase {
        assert!(p.spent_bn <= 0.001 + 1e-12);
    }
    let saved = spheres_sim::save(&w);
    assert_eq!(
        saved,
        spheres_sim::save(&spheres_sim::load(&saved).unwrap())
    );
}
