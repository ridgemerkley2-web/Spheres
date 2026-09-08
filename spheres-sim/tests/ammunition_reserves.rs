//! Reserve policies exercised through dated game commands and the complete daily tick.
//! The certified APC, completed factory and raw stocks below are explicit synthetic
//! test inputs. No opening ammunition or historical equipment grant is assumed.
use spheres_sim::world::{GameRules, NationId as N, WorldState, BUDGET_DEFENSE as D};
use spheres_sim::{clock, equipment, production, programs, resources, Command, EquipmentOrder};

const FAMILY: &str = "mg_127";

fn command(order: EquipmentOrder) -> Command {
    Command::Equipment {
        nation: N::USA,
        order,
    }
}

fn reserve(district: &str, target_rounds: u32, daily_budget_bn: f64, automatic: bool) -> Command {
    command(EquipmentOrder::AmmoReserve {
        family: FAMILY.into(),
        target_rounds,
        district: district.into(),
        daily_budget_bn,
        automatic,
    })
}

fn step(w: &mut WorldState, commands: &[Command]) {
    let day = clock::absolute_day(w);
    let events = spheres_sim::tick_day(w, commands);
    assert!(
        !events.iter().any(|line| line.starts_with("[rejected]")),
        "{events:?}"
    );
    assert_eq!(clock::absolute_day(w), day + 1);
    equipment::validate_state(w.nation(N::USA)).unwrap();
}

fn fixture() -> (WorldState, String) {
    let mut w = spheres_sim::init::world_1990(GameRules {
        daily_simulation: true,
        military_operations: true,
        production_system: true,
        manufacturing_system: true,
        resource_market: true,
        resource_gates: true,
        ..Default::default()
    });
    w.player = Some(N::USA);
    programs::set_construction_budget(&mut w, N::USA, 0.0).unwrap();
    let spec = equipment::default_spec("ground_apc");
    assert_eq!(equipment::ammunition_family(&spec), Some(FAMILY));
    let preview = equipment::design_preview(&w, N::USA, &spec);
    assert!(preview.valid, "{:?}", preview.blockers);
    let day = clock::absolute_day(&w);
    w.nation_mut(N::USA).equipment = Some(equipment::EquipmentState::default());
    w.nation_mut(N::USA)
        .equipment
        .as_mut()
        .unwrap()
        .revisions
        .insert(
            "reserve-fixture".into(),
            equipment::DesignRevision {
                id: "reserve-fixture".into(),
                name: "Synthetic reserve-test APC".into(),
                specification_key: equipment::specification_key(&spec),
                spec,
                profile: preview.profile.unwrap(),
                created_day: day,
                certified_day: Some(day),
            },
        );
    let district = w
        .districts
        .iter()
        .find(|(_, owner)| **owner == N::USA)
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
    // Open the actual market and make maintenance prospective through the same
    // command/date boundary used by a campaign before adding fixture raw stocks.
    step(
        &mut w,
        &[command(EquipmentOrder::Maintenance {
            daily_budget_bn: 0.0,
        })],
    );
    let market = w.resources.market.as_mut().unwrap();
    for commodity in [
        resources::Commodity::Iron,
        resources::Commodity::Copper,
        resources::Commodity::Coal,
    ] {
        if let Some(stock) = market
            .stocks
            .iter_mut()
            .find(|s| s.nation == N::USA && s.commodity == commodity)
        {
            stock.quantity = 10_000.0;
        } else {
            market.stocks.push(resources::Stock {
                nation: N::USA,
                commodity,
                quantity: 10_000.0,
                reserve_target: 0.0,
            });
        }
    }
    market.stocks.sort_by_key(|s| (s.nation, s.commodity));
    (w, district)
}

fn ammo(w: &WorldState) -> Option<&equipment::AmmunitionState> {
    w.nation(N::USA)
        .equipment
        .as_ref()
        .unwrap()
        .ammunition
        .as_ref()
}

fn orders(w: &WorldState) -> &[equipment::AmmoOrder] {
    ammo(w).map_or(&[], |a| a.orders.as_slice())
}

fn near(a: f64, b: f64) {
    assert!((a - b).abs() < 1e-11, "{a:.16e} != {b:.16e}");
}

fn paid(w: &WorldState) -> f64 {
    w.nation(N::USA)
        .program_budget
        .as_ref()
        .unwrap()
        .spent_ytd_bn[D][2]
}

fn conserved(w: &WorldState) {
    let Some(a) = ammo(w) else {
        return;
    };
    for family in equipment::ammo_catalog() {
        let delivered = a
            .orders
            .iter()
            .filter(|o| o.family == family.id)
            .map(|o| o.completed_rounds as f64)
            .sum::<f64>();
        near(
            a.stocks.get(family.id).copied().unwrap_or(0.0)
                + a.consumed.get(family.id).copied().unwrap_or(0.0),
            delivered,
        );
    }
    for o in &a.orders {
        assert!(o.completed_rounds <= o.quantity);
        assert!(o.spent_bn <= o.cost_bn + 1e-12);
        let def = equipment::ammo_def(&o.family).unwrap();
        near(o.spent_bn, o.work_rounds * def.fabrication_bn);
        for (used, recipe) in o.resources_used.iter().zip(o.recipe_per_round.iter()) {
            // The physical resource ledger quantizes cumulative recipes at
            // 1e-9 commodity units, including tiny machine-gun batches.
            assert!((*used - o.work_rounds * recipe).abs() <= 1.001e-9);
        }
    }
}

fn frozen_contract(o: &equipment::AmmoOrder) -> serde_json::Value {
    serde_json::json!({
        "id": o.id, "family": o.family, "district": o.district,
        "quantity": o.quantity, "cost": o.cost_bn, "limit": o.daily_limit_bn,
        "recipe": o.recipe_per_round, "rate": o.rounds_per_day, "started": o.started_day,
    })
}

#[test]
fn manual_targets_spend_nothing_and_automatic_authority_starts_on_a_later_date() {
    let (mut w, district) = fixture();
    let mut control = w.clone();
    step(&mut w, &[reserve(&district, 400, 0.001, false)]);
    step(&mut control, &[]);
    for _ in 0..2 {
        step(&mut w, &[]);
        step(&mut control, &[]);
    }
    assert!(orders(&w).is_empty());
    assert_eq!(
        w.nation(N::USA).program_budget,
        control.nation(N::USA).program_budget
    );
    assert_eq!(
        w.nation(N::USA).treasury_bn,
        control.nation(N::USA).treasury_bn
    );
    assert_eq!(w.resources.market, control.resources.market);
    let baseline = paid(&w);

    let authorization_day = clock::absolute_day(&w);
    step(&mut w, &[reserve(&district, 400, 0.001, true)]);
    assert!(
        orders(&w).is_empty(),
        "Enabling the policy must not manufacture a same-day order."
    );
    near(paid(&w), baseline);
    step(&mut w, &[]);
    let order = &orders(&w)[0];
    assert_eq!(
        (order.started_day, order.quantity, order.completed_rounds),
        (authorization_day + 1, 400, 0)
    );
    assert_eq!(order.spent_bn, 0.0);
    near(paid(&w), baseline);
    step(&mut w, &[]);
    assert_eq!(orders(&w).len(), 1);
    assert_eq!(orders(&w)[0].status, equipment::ProjectStatus::Complete);
    near(ammo(&w).unwrap().stocks[FAMILY], 400.0);
    near(paid(&w) - baseline, orders(&w)[0].cost_bn);
    conserved(&w);
}

#[test]
fn paused_manual_commitments_prevent_auto_stacking_and_cancellation_frees_only_the_remainder() {
    let (mut w, district) = fixture();
    step(
        &mut w,
        &[
            command(EquipmentOrder::AmmoOrder {
                family: FAMILY.into(),
                district: district.clone(),
                quantity: 100,
                daily_budget_bn: 0.0,
            }),
            reserve(&district, 1_000, 0.001, true),
        ],
    );
    let manual_id = orders(&w)[0].id;
    step(
        &mut w,
        &[command(EquipmentOrder::AmmoPause {
            project: manual_id,
            paused: true,
        })],
    );
    for _ in 0..2 {
        step(&mut w, &[]);
    }
    assert_eq!(
        orders(&w).len(),
        1,
        "A second free plant must not create another batch of the same family."
    );
    let status = equipment::ammo_reserve_status(&w, N::USA, FAMILY);
    assert_eq!(
        (status.committed, status.paused_committed, status.gap),
        (100, 100, 900)
    );
    assert!(!status.can_schedule);

    step(
        &mut w,
        &[command(EquipmentOrder::AmmoCancel { project: manual_id })],
    );
    assert_eq!(orders(&w).len(), 2);
    assert_eq!(orders(&w)[0].status, equipment::ProjectStatus::Cancelled);
    assert_eq!(
        (orders(&w)[1].quantity, orders(&w)[1].completed_rounds),
        (1_000, 0)
    );
    assert_eq!(
        equipment::ammo_reserve_status(&w, N::USA, FAMILY).committed,
        1_000
    );
    step(&mut w, &[]);
    assert_eq!(orders(&w).len(), 2);
    near(ammo(&w).unwrap().stocks[FAMILY], 1_000.0);
    conserved(&w);
}

#[test]
fn policy_edits_disable_and_clear_preserve_existing_manual_and_automatic_batch_contracts() {
    let (mut w, district) = fixture();
    let slow = equipment::ammo_def(FAMILY).unwrap().fabrication_bn * 2.0;
    step(&mut w, &[reserve(&district, 100, slow, true)]);
    step(&mut w, &[]);
    let automatic_contract = frozen_contract(&orders(&w)[0]);
    step(
        &mut w,
        &[command(EquipmentOrder::AmmoOrder {
            family: FAMILY.into(),
            district: district.clone(),
            quantity: 70,
            daily_budget_bn: 0.0,
        })],
    );
    let manual_contract = frozen_contract(&orders(&w)[1]);
    let alternate = w
        .districts
        .iter()
        .find(|(d, owner)| **owner == N::USA && *d != &district)
        .unwrap()
        .0
        .clone();
    step(&mut w, &[reserve(&alternate, 500, 0.002, true)]);
    step(&mut w, &[reserve(&alternate, 20, 0.0, false)]);
    step(
        &mut w,
        &[command(EquipmentOrder::AmmoReserveClear {
            family: FAMILY.into(),
        })],
    );
    for _ in 0..2 {
        step(&mut w, &[]);
    }
    assert!(w
        .nation(N::USA)
        .equipment
        .as_ref()
        .unwrap()
        .ammunition_reserves
        .is_empty());
    assert_eq!(orders(&w).len(), 2);
    assert_eq!(frozen_contract(&orders(&w)[0]), automatic_contract);
    assert_eq!(frozen_contract(&orders(&w)[1]), manual_contract);
    assert!(
        orders(&w)[0].completed_rounds > 0,
        "Clearing a policy must not cancel its paid production."
    );
    assert_eq!(orders(&w)[1].completed_rounds, 0);
    assert_ne!(orders(&w)[0].status, equipment::ProjectStatus::Cancelled);
    conserved(&w);
    spheres_sim::load(&spheres_sim::save(&w)).unwrap();
}

#[test]
fn save_resume_preserves_automatic_review_dates_batches_and_funding_exactly() {
    let (mut continuous, district) = fixture();
    let slow = equipment::ammo_def(FAMILY).unwrap().fabrication_bn * 2.0;
    let baseline = paid(&continuous);
    step(&mut continuous, &[reserve(&district, 30, slow, true)]);
    step(&mut continuous, &[]);
    step(&mut continuous, &[]);
    assert_eq!(orders(&continuous).len(), 1);
    assert!(orders(&continuous)[0].work_rounds > 0.0);
    let checkpoint = spheres_sim::save(&continuous);
    let mut resumed = spheres_sim::load(&checkpoint).unwrap();
    assert_eq!(checkpoint, spheres_sim::save(&resumed));
    let id = orders(&continuous)[0].id;
    for day in 0..7 {
        let commands = match day {
            0 => vec![command(EquipmentOrder::AmmoPause {
                project: id,
                paused: true,
            })],
            2 => vec![reserve(&district, 50, 0.001, true)],
            3 => vec![command(EquipmentOrder::AmmoPause {
                project: id,
                paused: false,
            })],
            4 => vec![command(EquipmentOrder::AmmoCancel { project: id })],
            _ => vec![],
        };
        step(&mut continuous, &commands);
        step(&mut resumed, &commands);
        assert_eq!(
            spheres_sim::state_hash(&continuous),
            spheres_sim::state_hash(&resumed),
            "Diverged on continuation date {day}"
        );
        conserved(&continuous);
    }
    assert_eq!(orders(&continuous).len(), 2);
    assert_eq!(
        orders(&continuous)[0].status,
        equipment::ProjectStatus::Cancelled
    );
    let completed_before_cancel = orders(&continuous)[0].completed_rounds;
    assert!(completed_before_cancel > 0);
    assert_eq!(
        orders(&continuous)[1].quantity,
        50 - completed_before_cancel
    );
    near(ammo(&continuous).unwrap().stocks[FAMILY], 50.0);
    near(
        paid(&continuous) - baseline,
        orders(&continuous).iter().map(|o| o.spent_bn).sum(),
    );
    assert_eq!(spheres_sim::save(&continuous), spheres_sim::save(&resumed));
}

#[test]
fn repeated_support_settlement_cannot_create_a_second_batch_or_spend_on_its_start_date() {
    let (mut w, district) = fixture();
    step(&mut w, &[reserve(&district, 20, 0.001, true)]);
    programs::begin_day(&mut w);
    equipment::settle_support(&mut w);
    assert_eq!(orders(&w).len(), 1);
    assert_eq!(orders(&w)[0].completed_rounds, 0);
    let after_first = spheres_sim::save(&w);
    equipment::settle_support(&mut w);
    assert_eq!(spheres_sim::save(&w), after_first);
    let paid_before = paid(&w);
    step(&mut w, &[]);
    assert_eq!(orders(&w).len(), 1);
    assert_eq!(orders(&w)[0].completed_rounds, 0);
    near(paid(&w), paid_before);
    conserved(&w);
}
