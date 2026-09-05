//! Historical game-capacity estimates and physical packs are related for
//! investment priority, never substituted for each other's inventory.
use spheres_sim::{
    clock, industry_planning as planning, init::world_1990, production,
    province_economy, save, starting_industry,
    world::{GameRules, NationId, WorldState},
};

fn world() -> WorldState {
    let mut w = world_1990(GameRules::default());
    clock::enable_daily_play(&mut w);
    starting_industry::enable_new_world(&mut w).unwrap();
    province_economy::enable(&mut w);
    w
}

#[test]
fn inherited_sectors_cover_the_opening_economy_but_are_not_usable_packs() {
    let w = world();
    let before = save(&w);
    for nation in w.nations.iter().filter(|n| n.alive) {
        let plan = planning::plan(&w, nation.id);
        assert_eq!(plan.inherited_sectors.len(), 5, "{}", nation.id.code());
        assert!(plan.inherited_sectors.iter().all(|r| r.expansion_annual_bn == 0.0));
        assert!(plan.inherited_sectors.iter().all(|r| r.pressure.is_finite()));
        assert!(plan.inherited_sectors.iter().map(|r| r.inherited_factory_equivalents).sum::<f64>() > 0.0);
        assert!(plan.goods.iter().all(|g| g.installed_daily == 0.0 && g.stock == 0.0 && g.incoming == 0.0));
        assert_eq!(plan.generation_daily, 0.0, "no free usable power from aggregate estimates");
    }
    assert_eq!(save(&w), before);
}

#[test]
fn actual_and_queued_factories_count_once_beside_inherited_capacity() {
    use production::ProjectKind as K;
    let mut w = world();
    let me = NationId::USA;
    let district = w.districts.iter().find(|(_, n)| **n == me).unwrap().0.clone();
    let baseline = planning::plan(&w, me);
    // Explicit fixture assets, not a data grant: generation, machinery,
    // processing; plus a fractional paid starter processor.
    w.production.industry.sites.insert(district.clone(), [1, 2, 3, 0, 0, 0, 0]);
    w.production.industry.modules.insert(district.clone(), 250_000);
    let installed = planning::plan(&w, me);
    let materials = |p: &planning::CapacityPlan| p.inherited_sectors.iter().find(|s| s.key == "materials").unwrap().clone();
    let first = materials(&installed);
    assert_eq!(first.inherited_factory_equivalents, materials(&baseline).inherited_factory_equivalents);
    assert!(first.funded_capacity_annual_bn > 0.0);
    assert_eq!(first.committed_capacity_annual_bn, 0.0);
    assert_eq!(installed.goods[0].installed_daily, 3.25);
    w.production.projects.push(production::Project {
        id: 7, nation: me, district: district.clone(), kind: K::ProcessingPlant,
        total_days: 180, progress_days: 0.0, priority: production::Priority::Normal,
        status: production::ProjectStatus::Building, reason: None,
        resources_used: [0.0; 12], capacity_micros: None, started_day: Some(clock::absolute_day(&w)),
    });
    let queued = planning::plan(&w, me);
    let with_queue = materials(&queued);
    assert_eq!(with_queue.funded_capacity_annual_bn, first.funded_capacity_annual_bn);
    assert!((with_queue.committed_capacity_annual_bn - first.funded_capacity_annual_bn / 3.25).abs() < 1e-12);
    assert_eq!(queued.goods[0].committed_daily, 1.0);
    assert_eq!(with_queue.output_annual_bn, first.output_annual_bn, "queued potential is not achieved GDP");
}

#[test]
fn inherited_pressure_ranks_candidates_without_inventing_physical_demand() {
    use production::ProjectKind as K;
    let mut w = world();
    let me = NationId::USA;
    // A larger inherited economy increases utilization, not the number of
    // historical equivalents. Extra materials capacity then changes priority.
    w.nation_mut(me).gdp *= 1.20;
    let before = planning::plan(&w, me);
    let mut plan = before.clone();
    let materials = plan.inherited_sectors.iter_mut().find(|s| s.key == "materials").unwrap();
    materials.pressure = 0.0;
    assert_eq!(planning::expansion_order(&plan), [K::MachineryWorks, K::ProcessingPlant]);
    assert!(plan.goods.iter().all(|g| g.expansion_daily == 0.0 && g.demand_daily == 0.0));
    for row in &mut plan.inherited_sectors { row.pressure = 0.0; }
    assert_eq!(planning::expansion_order(&plan), [K::ProcessingPlant, K::MachineryWorks]);
    assert!(before.inherited_sectors.iter().filter(|r| r.inherited_capacity_annual_bn > 0.0)
        .all(|r| r.expansion_annual_bn > 0.0));
}

#[test]
fn manufacturing_pressure_excludes_installation_and_uses_reconciled_transfer_receipts() {
    use spheres_sim::gdp_projects;
    use production::ProjectKind as K;
    let mut w = world();
    let me = NationId::USA;
    let district = w.districts.iter().find(|(_, n)| **n == me).unwrap().0.clone();
    let project = production::Project {
        id: 9, nation: me, district: district.clone(), kind: K::ProcessingPlant,
        total_days: 180, progress_days: 0.0, priority: production::Priority::Normal,
        status: production::ProjectStatus::Building, reason: None,
        resources_used: [0.0; 12], capacity_micros: None, started_day: Some(clock::absolute_day(&w)),
    };
    // Explicit receipt fixtures: $1bn annual-equivalent installation, not
    // $1bn of factory production. Both share the same project-kind identifier.
    gdp_projects::record_construction(&mut w, &project, 1.0, 1.0 / 365.0, true);
    let construction = w.province_economy.as_ref().unwrap().flows.receipts.values().next().unwrap().clone();
    w.province_economy.as_mut().unwrap().posted_contributions.push(construction);
    let inherited = starting_industry::snapshot(&w, me).unwrap();
    let plan = planning::plan(&w, me);
    assert_eq!(plan.inherited_sectors[1].output_annual_bn, inherited.groups[1].current_output_annual_bn,
        "building a processor is construction GDP, not its manufactured output");

    gdp_projects::record_factory(&mut w, me, &district, K::ProcessingPlant, 1.0, 1.0, [0.0; 12], 0.0, 0.0);
    let factory = w.province_economy.as_ref().unwrap().flows.receipts.values()
        .find(|r| r.sector == "manufacturing").unwrap().clone();
    w.province_economy.as_mut().unwrap().posted_contributions.push(factory.clone());
    let inherited = starting_industry::snapshot(&w, me).unwrap();
    let plan = planning::plan(&w, me);
    assert!((plan.inherited_sectors[1].output_annual_bn
        - inherited.groups[1].current_output_annual_bn - factory.annual_gdp_bn).abs() < 1e-12,
        "actual manufacturing contributes exactly once");

    w.districts.insert(district, NationId::Tonga);
    let ledger = province_economy::snapshot(&w, NationId::Tonga).unwrap();
    let reconciled = ledger.projects.iter().filter(|r| r.counted && r.sector == "manufacturing")
        .map(|r| r.annual_gdp_bn).sum::<f64>();
    assert!(reconciled < factory.annual_gdp_bn, "fixture crosses the receiving GDP bound");
    let inherited = starting_industry::snapshot(&w, NationId::Tonga).unwrap();
    let plan = planning::plan(&w, NationId::Tonga);
    assert!((plan.inherited_sectors[1].output_annual_bn
        - inherited.groups[1].current_output_annual_bn - reconciled).abs() < 1e-12,
        "capacity advice must use the same reconciled output as the provincial GDP view");
}
