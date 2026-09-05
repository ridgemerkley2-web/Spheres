//! Actual candidate selection, not just the structural-order helper. The
//! fixture's plants and delivery history are explicit test state, never grants
//! made by the inherited-capacity model to a real campaign.
use spheres_sim::{
    apply_command, clock,
    commerce::{Good, GoodsDelivery},
    economic_ai, industry_planning,
    init::world_1990,
    production::{self, ProjectKind as K},
    programs, province_economy, save, starting_industry,
    world::{GameRules, NationId, WorldState},
    Command,
};

fn prepared() -> (WorldState, String) {
    let mut w = world_1990(GameRules {
        daily_simulation: true,
        economic_competition: true,
        production_system: true,
        resource_market: true,
        resource_gates: true,
        physical_logistics: true,
        logistics_routes: true,
        ..GameRules::default()
    });
    starting_industry::enable_new_world(&mut w).unwrap();
    province_economy::enable(&mut w);
    w.conflicts.clear();
    let nation = NationId::USA;
    w.nation_mut(nation).political_capital = 1000.0;
    let allocations = w.nation(nation).budget_for(w.year).allocations;
    apply_command(
        &mut w,
        &Command::SetProgramBudget {
            nation,
            fiscal_year: 1990,
            allocations,
            departments: programs::default_departments(),
        },
    )
    .unwrap();
    let district = w
        .districts
        .iter()
        .find(|(_, owner)| **owner == nation)
        .unwrap()
        .0
        .clone();
    w.production
        .provinces
        .push(production::ProvinceCapabilities {
            district: district.clone(),
            civilian_industry: 1,
            power_grid: 2,
            infrastructure: 0,
            research_centers: 0,
            arms_plants: 0,
        });
    // Site order is generation, machinery, processing, storage, freight,
    // automation, efficiency. Both real producers exist, avoiding bootstrap.
    w.production
        .industry
        .sites
        .insert(district.clone(), [1, 1, 1, 0, 0, 0, 0]);
    let today = clock::absolute_day(&w);
    for (contract, good) in [(1, Good::Intermediates), (2, Good::CapitalGoods)] {
        w.commerce
            .get_or_insert_with(Default::default)
            .goods_deliveries
            .push(GoodsDelivery {
                contract,
                day: today,
                buyer: NationId::Canada,
                seller: nation,
                good,
                quantity: 5000.0,
            });
    }
    // Growth changes current modeled utilization, never the installed pack
    // plants or inventories. Both sectors begin with structural pressure.
    w.nation_mut(nation).gdp *= 1.20;
    (w, district)
}

#[test]
fn real_ai_candidate_uses_inherited_priority_only_between_evidenced_pack_expansions() {
    let (baseline, district) = prepared();
    let nation = NationId::USA;
    let baseline_plan = industry_planning::plan(&baseline, nation);
    assert!(baseline_plan.goods.iter().all(|g| {
        g.export_daily > 0.0 && g.expansion_daily > 1.0 && g.stock == 0.0 && g.incoming == 0.0
    }));
    for kind in [K::ProcessingPlant, K::MachineryWorks] {
        assert_eq!(production::level(&baseline, &district, kind), 1);
        assert_eq!(
            production::start_project_error(&baseline, nation, &district, kind),
            None
        );
    }
    assert!(baseline_plan.generation_daily > baseline_plan.power_required_daily + 1.0);
    let site = baseline_plan
        .provinces
        .iter()
        .find(|p| p.district == district)
        .unwrap();
    assert!(site.grid_daily > site.power_required_daily + 1.0);

    for (covered_sector, preferred, pressure_key) in [
        (1, K::MachineryWorks, "machinery_electronics"),
        (3, K::ProcessingPlant, "materials"),
    ] {
        let mut w = baseline.clone();
        // TEST-ONLY alternate historical-capacity estimate. No government
        // command can resize these frozen records or receive physical goods.
        let owned: Vec<_> = w
            .districts
            .iter()
            .filter(|(_, n)| **n == nation)
            .map(|(d, _)| d.clone())
            .collect();
        for d in owned {
            w.starting_industry
                .as_mut()
                .unwrap()
                .provinces
                .get_mut(&d)
                .unwrap()
                .factory_equivalents[covered_sector] *= 2.0;
        }
        let plan = industry_planning::plan(&w, nation);
        assert_eq!(
            plan.goods, baseline_plan.goods,
            "estimates cannot alter pack demand/supply"
        );
        assert!(
            plan.inherited_sectors
                .iter()
                .find(|s| s.key == pressure_key)
                .unwrap()
                .pressure
                > 0.0
        );
        assert_eq!(plan.inherited_sectors[covered_sector].pressure, 0.0);
        assert_eq!(w.nation(nation).gdp, baseline.nation(nation).gdp);
        assert_eq!(w.production, baseline.production);
        assert_eq!(w.commerce, baseline.commerce);

        let before = save(&w);
        let choice = economic_ai::candidate(&w, nation).unwrap();
        assert_eq!(
            choice.0, district,
            "use the already-powered eligible province"
        );
        assert_eq!(
            choice.1, preferred,
            "actual AI must respect sector priority after real-demand gates"
        );
        assert_eq!(
            save(&w),
            before,
            "a candidate read cannot order or pay for a plant"
        );
    }
}
