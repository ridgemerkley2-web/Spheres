//! S07 ownership invariants. Synthetic opening service support, inventories and
//! appropriations isolate construction and operation; they are not calibration.
use contractors::{CompanySector as Sector, CompanyTarget as Target};
use spheres_sim::world::{GameRules, NationId as N, WorldState, BUDGET_INDUSTRY as I};
use spheres_sim::{
    apply_command, clock, construction_preview, fiscal_journal, fiscal_recovery, gdp_projects,
    industry, industry_operations as ops, load, population,
    production::{self, ProjectKind as K},
    programs, province_economy,
    resources::{self, Commodity as C},
    save, sector_contractors as contractors, Command,
};

fn near(a: f64, b: f64) {
    assert!((a - b).abs() < 1e-9, "{a:.15} != {b:.15}");
}
fn base(n: N) -> (WorldState, String) {
    let mut w = spheres_sim::init::world_1990(GameRules {
        daily_simulation: true,
        production_system: true,
        resource_market: true,
        industry_rebuild: true,
        ai_aggression: 0.0,
        crisis_intensity: 0.0,
        ..Default::default()
    });
    w.player = Some(n);
    w.conflicts.clear();
    let nation = w.nation_mut(n);
    nation.political_capital = 1000.0;
    nation.treasury_bn = Some(20.0);
    nation.debt_bn = Some(0.0);
    nation.debt_gdp = 0.0;
    programs::set_construction_budget(&mut w, n, 0.01).unwrap();
    resources::tick(&mut w);
    programs::begin_day(&mut w);
    let d = w
        .districts
        .iter()
        .find(|(_, owner)| **owner == n)
        .unwrap()
        .0
        .clone();
    funds(&mut w, n);
    stock(&mut w, n, 0.0);
    (w, d)
}
fn funds(w: &mut WorldState, n: N) {
    let p = w.nation_mut(n).program_budget.as_mut().unwrap();
    for department in 0..3 {
        p.available_bn[I][department] = 1.0;
    }
}
fn stock(w: &mut WorldState, n: N, amount: f64) {
    let market = w.resources.market.as_mut().unwrap();
    market.stocks.retain(|s| s.nation != n);
    for commodity in resources::ALL {
        market.stocks.push(resources::Stock {
            nation: n,
            commodity,
            quantity: amount,
            reserve_target: 0.0,
        });
    }
    market.stocks.sort_by_key(|s| (s.nation, s.commodity));
}
fn raw(w: &WorldState, n: N) -> [f64; 12] {
    std::array::from_fn(|i| resources::stockpile(w, n, C::from_idx(i).unwrap()))
}
fn support(w: &mut WorldState, d: &str) {
    w.production
        .provinces
        .push(production::ProvinceCapabilities {
            district: d.into(),
            infrastructure: 0,
            civilian_industry: 1,
            power_grid: 2,
            research_centers: 0,
            arms_plants: 0,
        });
    w.production
        .provinces
        .sort_by(|a, b| a.district.cmp(&b.district));
    w.production
        .industry
        .sites
        .insert(d.into(), [0, 1, 0, 0, 0, 0, 0]);
}
fn construction_day(w: &mut WorldState, n: N) {
    province_economy::begin_day(w);
    programs::begin_day(w);
    let before = w.nation(n).treasury_bn;
    production::tick_day(w);
    assert_eq!(
        w.nation(n).treasury_bn,
        before,
        "construction spends authority, fiscal close owns cash"
    );
    stage_zero_receipt(w, n);
    programs::finish_day(w);
    province_economy::finish_day(w);
    fiscal_recovery::tick(w);
    clock::advance_date(w);
}
fn operate(w: &mut WorldState, n: N) {
    province_economy::begin_day(w);
    programs::begin_day(w);
    funds(w, n);
    ops::begin_day(w);
    industry::tick_day(w);
    ops::tick_day(w);
}
fn stage_zero_receipt(w: &mut WorldState, n: N) {
    // Isolate actual paid project/service work from unrelated recurring taxes.
    let p = w.nation_mut(n).program_budget.as_mut().unwrap();
    p.revenue_today_bn = 0.0;
    p.interest_today_bn = 0.0;
    p.fiscal_staged = true;
}

#[test]
fn s07_financial_queue_pause_cancel_complete_then_staff_and_operate_france_and_tonga() {
    for n in [N::France, N::Tonga] {
        let (mut w, d) = base(n);
        if n == N::France {
            support(&mut w, &d);
        }
        population::enable(&mut w).unwrap();
        fiscal_recovery::enable(&mut w);
        province_economy::enable(&mut w);
        let kind = if n == N::France {
            K::OfficeDistrict
        } else {
            K::StarterIndustry
        };
        let micros = (n == N::Tonga).then_some(1_000);
        let preview = construction_preview::preview(&w, n, &d, kind, micros);
        assert!(preview.can_start, "{:?}", preview.reason);
        let command = if let Some(capacity_micros) = micros {
            Command::StartIndustryModule {
                nation: n,
                district: d.clone(),
                capacity_micros,
            }
        } else {
            Command::StartProject {
                nation: n,
                district: d.clone(),
                kind,
            }
        };
        let opening = w.nation(n).treasury_bn;
        apply_command(&mut w, &command).unwrap();
        assert_eq!(w.nation(n).treasury_bn, opening);
        let id = w.production.projects.last().unwrap().id;
        apply_command(
            &mut w,
            &Command::SetProjectPriority {
                nation: n,
                project: id,
                priority: production::Priority::High,
            },
        )
        .unwrap();
        construction_day(&mut w, n);
        let progress = w.production.projects[0].progress_days;
        assert!(progress > 0.0);
        let paid = w.production.industry.projects[&id].spent_bn;
        apply_command(
            &mut w,
            &Command::SetConstructionBudget {
                nation: n,
                daily_budget_bn: 0.0,
            },
        )
        .unwrap();
        construction_day(&mut w, n);
        assert_eq!(w.production.projects[0].progress_days, progress);
        assert_eq!(w.production.industry.projects[&id].spent_bn, paid);
        let before_cancel = w.nation(n).treasury_bn;
        apply_command(
            &mut w,
            &Command::CancelProject {
                nation: n,
                project: id,
            },
        )
        .unwrap();
        assert_eq!(
            w.nation(n).treasury_bn,
            before_cancel,
            "paid construction is sunk, no invented refund"
        );
        assert!(w.production.projects.is_empty());
        apply_command(
            &mut w,
            &Command::SetConstructionBudget {
                nation: n,
                daily_budget_bn: 0.01,
            },
        )
        .unwrap();
        apply_command(&mut w, &command).unwrap();
        let replacement = w.production.projects.last().unwrap().id;
        let opening_paid = w
            .nation(n)
            .program_budget
            .as_ref()
            .unwrap()
            .construction_spent_ytd_bn;
        let residents = w.nation(n).population;
        let expected_cost = industry::contract_cost_bn(&w, w.production.projects.last().unwrap());
        let mut resumed = load(&save(&w)).unwrap();
        let mut days = 0;
        while !w.production.projects.is_empty() && days < 800 {
            construction_day(&mut w, n);
            construction_day(&mut resumed, n);
            days += 1;
        }
        assert!(
            w.production.projects.is_empty(),
            "{n:?} queue should commission within quoted finite work"
        );
        assert_eq!(save(&w), save(&resumed));
        assert_eq!(
            w.nation(n).population,
            residents,
            "construction does not fabricate workers"
        );
        assert_eq!(raw(&w, n), [0.0; 12]);
        near(
            w.nation(n)
                .program_budget
                .as_ref()
                .unwrap()
                .construction_spent_ytd_bn
                - opening_paid,
            expected_cost,
        );
        assert!(!w.production.industry.projects.contains_key(&replacement));
        assert!(
            ops::current_site_staffing(&w, &d, kind).is_none(),
            "completion awaits actual population matching"
        );
        population::tick(&mut w);
        let staffing = ops::current_site_staffing(&w, &d, kind).unwrap();
        assert!(staffing.jobs_assigned > 0.0);
        near(
            staffing.jobs_required,
            ops::jobs_per_level(kind) * micros.map_or(1.0, |v| v as f64 / 1e6),
        );
        stock(&mut w, n, 10.0);
        w.production.industry.goods.insert(
            n,
            industry::Goods {
                intermediates: 10.0,
                capital_goods: 0.0,
            },
        );
        let people = serde_json::to_value(&w.population_system).unwrap();
        let cash = w.nation(n).treasury_bn;
        operate(&mut w, n);
        assert_eq!(w.nation(n).treasury_bn, cash);
        assert_eq!(serde_json::to_value(&w.population_system).unwrap(), people);
        let output = if kind == K::StarterIndustry {
            w.production
                .industry
                .operations
                .iter()
                .find(|s| s.district == d && s.kind == kind)
                .unwrap()
                .output_daily
        } else {
            w.production
                .operations
                .receipts
                .iter()
                .find(|s| s.district == d && s.kind == kind.key())
                .unwrap()
                .output_daily
        };
        assert!(
            output > 0.0,
            "{n:?} commissioned site must produce after staffing and operating supply"
        );
        let flow = &w.province_economy.as_ref().unwrap().flows.receipts
            [&format!("site:{d}:{}", kind.key())];
        assert!(gdp_projects::incremental_gdp_bn(flow) > 0.0);
        assert_eq!(flow.inherited_annual_gdp_bn, 0.0);
        let once = save(&w);
        industry::tick_day(&mut w);
        ops::tick_day(&mut w);
        assert_eq!(save(&w), once);
        stage_zero_receipt(&mut w, n);
        programs::finish_day(&mut w);
        let journal = fiscal_journal::view(&w, n).unwrap();
        let row = journal.days.last().unwrap();
        near(
            row.fiscal.as_ref().unwrap().spending_bn,
            w.nation(n)
                .program_budget
                .as_ref()
                .unwrap()
                .spent_today_bn
                .iter()
                .flatten()
                .sum(),
        );
        assert_eq!(save(&load(&save(&w)).unwrap()), save(&w));
    }
}

fn assign(w: &mut WorldState, n: N, d: &str, sector: Sector) {
    let company = contractors::companies_for(w, n)
        .filter(|c| c.sector == sector)
        .max_by(|a, b| a.input_saving.total_cmp(&b.input_saving))
        .unwrap()
        .id;
    let target = Target::Facility {
        district: d.into(),
        sector,
    };
    let quote = contractors::assignment_quote(w, n, company, &target);
    assert!(quote.valid, "{:?}", quote.reason);
    apply_command(
        w,
        &Command::AssignSectorContractor {
            nation: n,
            company,
            target,
            quote: quote.token,
        },
    )
    .unwrap();
}
fn result(w: &WorldState, n: N, d: &str, kind: K) -> (f64, f64) {
    if kind == K::AdvancedIndustry {
        let row = w
            .production
            .operations
            .receipts
            .iter()
            .find(|r| r.district == d && r.kind == kind.key())
            .unwrap();
        (row.output_daily, row.cash_spent_daily_bn)
    } else {
        let row = w
            .production
            .industry
            .operations
            .iter()
            .find(|r| r.district == d && r.kind == kind)
            .unwrap();
        assert!(row.operation.as_ref().is_some_and(|c| c.nation == n));
        (row.output_daily, row.cash_spent_daily_bn)
    }
}
#[test]
fn s07_contractor_preview_matches_actual_expanded_plant_recipe_and_cash() {
    for kind in [K::ProcessingPlant, K::AdvancedIndustry] {
        let n = N::France;
        let (mut w, d) = base(n);
        support(&mut w, &d);
        if kind == K::ProcessingPlant {
            w.production.industry.sites.get_mut(&d).unwrap()[2] = 1;
        } else {
            w.production.rebuild_sites.insert(d.clone(), [0, 0, 1]);
        }
        contractors::enable(&mut w);
        assign(&mut w, n, &d, Sector::Manufacturing);
        assign(&mut w, n, &d, Sector::Energy);
        let saved = save(&w);
        let preview = construction_preview::preview(&w, n, &d, kind, None);
        assert_eq!(save(&w), saved);
        let mut after = w.clone();
        if kind == K::ProcessingPlant {
            after.production.industry.sites.get_mut(&d).unwrap()[2] += 1;
        } else {
            after.production.rebuild_sites.get_mut(&d).unwrap()[2] += 1;
        }
        for world in [&mut w, &mut after] {
            stock(world, n, 100.0);
            world.production.industry.goods.insert(
                n,
                industry::Goods {
                    intermediates: 100.0,
                    capital_goods: 0.0,
                },
            );
            operate(world, n);
        }
        let (before_output, before_cash) = result(&w, n, &d, kind);
        let (next_output, next_cash) = result(&after, n, &d, kind);
        assert!(next_output > before_output);
        let need = |label: &str| {
            preview
                .operating_requirements
                .iter()
                .find(|r| r.label == label)
                .unwrap()
                .value
                .unwrap()
        };
        if kind == K::ProcessingPlant {
            near(need("Change in operating cash"), next_cash - before_cash);
            for c in [C::Iron, C::Bauxite, C::Coal] {
                near(
                    need(&format!("Change in {} demand", c.name())),
                    resources::stockpile(&w, n, c) - resources::stockpile(&after, n, c),
                );
            }
        } else {
            near(
                need("Operating service cash") + need("Generation service cash"),
                next_cash - before_cash,
            );
            near(
                need("Intermediate packs"),
                w.production.industry.goods[&n].intermediates
                    - after.production.industry.goods[&n].intermediates,
            );
            for c in [C::Copper, C::RareEarths, C::Coal] {
                near(
                    need(c.name()),
                    resources::stockpile(&w, n, c) - resources::stockpile(&after, n, c),
                );
            }
            let rated = preview
                .province_effects
                .iter()
                .find(|e| e.label == "Rated advanced-component output")
                .unwrap();
            near(
                rated.after.unwrap() - rated.before.unwrap(),
                next_output - before_output,
            );
        }
    }
}

#[test]
fn s07_assigned_workers_remain_employed_when_inputs_block_and_receipts_keep_original_capacity() {
    let n = N::France;
    let (mut w, d) = base(n);
    support(&mut w, &d);
    w.production.industry.sites.get_mut(&d).unwrap()[2] = 1;
    w.production.rebuild_sites.insert(d.clone(), [0, 0, 1]);
    population::enable(&mut w).unwrap();
    let processor = ops::current_site_staffing(&w, &d, K::ProcessingPlant).unwrap();
    let advanced = ops::current_site_staffing(&w, &d, K::AdvancedIndustry).unwrap();
    let p = &w.population_system.provinces[&d];
    for grade in 0..3 {
        let all = p.jobs[2][grade] + p.project_jobs[2][grade];
        let inherited = if all > 0.0 {
            p.filled[2][grade] * (p.jobs[2][grade] / all) * 1e6
        } else {
            0.0
        };
        near(
            inherited
                + processor.assigned_by_qualification[grade]
                + advanced.assigned_by_qualification[grade],
            p.filled[2][grade] * 1e6,
        );
    }
    let employed = p.employed_m();
    operate(&mut w, n);
    let r = w
        .production
        .industry
        .operations
        .iter()
        .find(|r| r.kind == K::ProcessingPlant)
        .unwrap();
    let context = r.operation.as_ref().unwrap().clone();
    assert_eq!(context.jobs_used, 0.0);
    assert!(context.jobs_assigned.unwrap() > 0.0);
    assert_eq!(w.population_system.provinces[&d].employed_m(), employed);
    w.production.industry.sites.get_mut(&d).unwrap()[2] = 2;
    assert!(ops::current_site_staffing(&w, &d, K::ProcessingPlant).is_none());
    assert_eq!(
        w.production
            .industry
            .operations
            .iter()
            .find(|r| r.kind == K::ProcessingPlant)
            .unwrap()
            .operation
            .as_ref(),
        Some(&context)
    );
    let saved = save(&w);
    let restored = load(&saved).unwrap();
    assert_eq!(save(&restored), saved);
    let mut invalid = w.clone();
    invalid
        .production
        .industry
        .operations
        .iter_mut()
        .find(|r| r.kind == K::ProcessingPlant)
        .unwrap()
        .operation
        .as_mut()
        .unwrap()
        .jobs_used = 10_000.0;
    assert!(ops::validate(&invalid).is_err());
    let mut wrong_required = w.clone();
    wrong_required.production.industry.operations[0]
        .operation
        .as_mut()
        .unwrap()
        .jobs_required += 1.0;
    assert!(
        load(&save(&wrong_required)).is_err(),
        "dated employment cannot be detached from purchased capacity"
    );
    let mut wrong_capacity = w.clone();
    wrong_capacity.production.industry.operations[0]
        .operation
        .as_mut()
        .unwrap()
        .installed_capacity = 0.01;
    assert!(
        load(&save(&wrong_capacity)).is_err(),
        "dated context must match the receipt's original plant"
    );
    let mut duplicate = w.clone();
    duplicate
        .production
        .industry
        .operations
        .push(duplicate.production.industry.operations[0].clone());
    assert!(
        load(&save(&duplicate)).is_err(),
        "one site cannot provide two dated operating receipts"
    );
}

#[test]
fn s07_research_building_quotes_base_terms_without_inventing_one_future_target_contract() {
    let n = N::France;
    let (mut w, d) = base(n);
    support(&mut w, &d);
    w.production
        .provinces
        .iter_mut()
        .find(|p| p.district == d)
        .unwrap()
        .research_centers = 1;
    contractors::enable(&mut w);
    let company = contractors::companies_for(&w, n)
        .filter(|c| c.sector == Sector::Research)
        .max_by(|a, b| a.work_bonus.total_cmp(&b.work_bonus))
        .unwrap()
        .id;
    let target = Target::Research {
        domain: "Materials".into(),
    };
    let quote = contractors::assignment_quote(&w, n, company, &target);
    assert!(quote.valid, "{:?}", quote.reason);
    assert!(
        quote.work_rate > 1.0 && quote.fee_rate > 0.0,
        "real assigned research terms differ from an unassigned laboratory"
    );
    apply_command(
        &mut w,
        &Command::AssignSectorContractor {
            nation: n,
            company,
            target,
            quote: quote.token,
        },
    )
    .unwrap();
    let before = save(&w);
    let preview = construction_preview::preview(&w, n, &d, K::ResearchCenter, None);
    let capacity = preview
        .province_effects
        .iter()
        .find(|r| r.label == "Base prototype-service capacity")
        .unwrap();
    near(
        capacity.after.unwrap() - capacity.before.unwrap(),
        industry::PROTOTYPE_WORK_PER_LEVEL_DAY,
    );
    let cash = preview
        .operating_requirements
        .iter()
        .find(|r| r.label == "Base Science operating cash per added level")
        .unwrap();
    assert_eq!(cash.value, Some(industry::PROTOTYPE_CASH_PER_LEVEL_DAY_BN));
    let terms = preview
        .operating_requirements
        .iter()
        .find(|r| r.label == "Research-target contractor terms")
        .unwrap();
    assert_eq!(terms.value, None);
    assert_eq!(terms.unit, "target-specific");
    assert!(capacity.detail.contains("contractor") && cash.detail.contains("before"));
    assert_eq!(
        save(&w),
        before,
        "reviewing a future lab never chooses or bills a research target"
    );
}

#[test]
fn s07_generation_preview_uses_the_same_pooled_fuel_and_cash_terms_as_actual_dispatch() {
    let n = N::France;
    let (mut w, d) = base(n);
    support(&mut w, &d);
    w.production.industry.sites.get_mut(&d).unwrap()[2] = 1;
    contractors::enable(&mut w);
    assign(&mut w, n, &d, Sector::Energy);
    let saved = save(&w);
    let preview = construction_preview::preview(&w, n, &d, K::Generation, None);
    assert_eq!(save(&w), saved);
    let mut after = w.clone();
    after.production.industry.sites.get_mut(&d).unwrap()[1] += 1;
    let delta = industry::power_capacity(&after, n) - industry::power_capacity(&w, n);
    stock(&mut after, n, 100.0);
    operate(&mut after, n);
    let receipt = after
        .production
        .industry
        .operations
        .iter()
        .find(|r| r.kind == K::ProcessingPlant)
        .unwrap();
    assert!(receipt.output_daily > 0.0 && receipt.power_used_daily > 0.0);
    let coal_used = 100.0 - resources::stockpile(&after, n, C::Coal);
    // The processor's direct coal ingredient is separate from generator fuel.
    let generator_fuel = coal_used - receipt.output_daily * 0.25;
    let energy_cash = after
        .nation(n)
        .program_budget
        .as_ref()
        .unwrap()
        .spent_today_bn[I][1];
    let coal = preview
        .operating_requirements
        .iter()
        .find(|r| r.label == "Coal if all added generation is used")
        .unwrap()
        .value
        .unwrap();
    let bill = preview
        .operating_requirements
        .iter()
        .find(|r| r.label == "Generating service bill at full added output")
        .unwrap()
        .value
        .unwrap();
    near(coal / delta, generator_fuel / receipt.power_used_daily);
    near(bill / delta, energy_cash / receipt.power_used_daily);
    assert!(
        coal < delta * 0.02 && bill > delta * 0.000002,
        "assigned generator savings and fees must be visible"
    );
}
