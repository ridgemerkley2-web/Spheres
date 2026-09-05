//! Proportional, purchased starter capacity, never a sourced 1990 asset.
//! Each millionth buys the same fraction of estate, generation, grid and
//! processing. Completed capacity belongs to its province; national GDP never
//! resizes it. Normalized project progress retains precision for tiny modules.
use crate::{
    clock, industry,
    production::{self, Priority, Project, ProjectKind as K, ProjectStatus},
    programs,
    world::{NationId, WorldState, BUDGET_INDUSTRY},
};
use serde::Serialize;

pub const STANDARD_MICROS: u32 = 1_000_000;
pub const MAX_MICROS: u32 = 5_000_000;
pub const MIN_CALENDAR_DAYS: u32 = 90;
pub const COMPONENTS: [K; 4] = [
    K::CivilianIndustry,
    K::Generation,
    K::PowerGrid,
    K::ProcessingPlant,
];

pub fn capacity(w: &WorldState, district: &str) -> f64 {
    w.production
        .industry
        .modules
        .get(district)
        .copied()
        .unwrap_or(0) as f64
        / STANDARD_MICROS as f64
}
pub fn effective_capacity(w: &WorldState, district: &str, kind: K) -> f64 {
    if kind == K::StarterIndustry {
        return capacity(w, district);
    }
    production::level(w, district, kind) as f64
        + if COMPONENTS.contains(&kind) {
            capacity(w, district)
        } else {
            0.0
        }
}
/// Capacity already installed OR committed by every relevant queue entry.
/// Reserving at order time prevents parallel normal/module orders crossing cap.
pub fn reserved_capacity(w: &WorldState, district: &str, kind: K) -> u32 {
    let installed = production::level(w, district, kind) as u64 * STANDARD_MICROS as u64
        + w.production
            .industry
            .modules
            .get(district)
            .copied()
            .unwrap_or(0) as u64;
    let pending: u64 = w
        .production
        .projects
        .iter()
        .filter(|p| p.district == district)
        .map(|p| {
            if p.kind == K::StarterIndustry {
                p.capacity_micros.unwrap_or(0) as u64
            } else if p.kind == kind {
                STANDARD_MICROS as u64
            } else {
                0
            }
        })
        .sum();
    installed.saturating_add(pending).min(u32::MAX as u64) as u32
}
pub fn recommended_capacity_micros(w: &WorldState, nation: NationId) -> u32 {
    let Some(n) = w.nation_opt(nation) else {
        return 1;
    };
    let share = n
        .program_budget
        .as_ref()
        .map_or(2000, |p| p.departments[BUDGET_INDUSTRY][0]) as f64
        / 10_000.0;
    let annual = n.gdp * n.budget_for(w.year).allocations[BUDGET_INDUSTRY] * share;
    if !annual.is_finite() || annual <= 0.0 {
        return 1;
    }
    // Appropriations release twelve equal monthly shares, not 365 equal
    // daily shares. Size against 365 days at the slowest (31-day month)
    // release rate. Otherwise early January funding can leave usable work
    // capacity idle and push an ostensibly one-year package to day 366.
    let calendar_cash = annual * (365.0 / (12.0 * 31.0));
    let work_scale = 365.0 * production::construction_capacity(w, nation).min(1.5) / 1800.0;
    let mut micros = ((calendar_cash / industry::work_cost_bn(K::StarterIndustry)).min(work_scale)
        * STANDARD_MICROS as f64)
        .floor()
        .clamp(1.0, STANDARD_MICROS as f64) as u32;
    // Inverse division can round a mathematically exact boundary upward by
    // one ULP. Validate against the forward bill the player actually pays.
    // One micro remains the explicit minimum even below a one-year budget.
    while micros > 1 {
        let fraction = micros as f64 / STANDARD_MICROS as f64;
        if industry::work_cost_bn(K::StarterIndustry) * fraction <= calendar_cash
            && 1800.0 * fraction <= 365.0 * production::construction_capacity(w, nation).min(1.5)
        {
            break;
        }
        micros -= 1;
    }
    micros
}
#[derive(Clone, Debug, Serialize)]
pub struct ModuleQuote {
    pub capacity_micros: u32,
    pub scale: f64,
    pub cost_bn: f64,
    pub recipe: [f64; 12],
    /// Physical work; Project.total_days remains 1800 normalized units.
    pub nominal_work_days: f64,
    pub minimum_calendar_days: u32,
    pub political_cost: f64,
    pub department: usize,
    pub output_daily: f64,
    pub power_capacity_daily: f64,
    pub grid_capacity_daily: f64,
    pub construction_capacity_daily: f64,
    pub annual_authority_bn: f64,
    pub lower_bound_days: Option<u32>,
    pub recommended_micros: u32,
    pub reason: Option<String>,
}
pub fn quote(
    w: &WorldState,
    nation: NationId,
    district: &str,
    capacity_micros: u32,
) -> ModuleQuote {
    let scale = capacity_micros as f64 / STANDARD_MICROS as f64;
    let annual_authority_bn = w.nation_opt(nation).map_or(0.0, |n| {
        n.gdp
            * n.budget_for(w.year).allocations[BUDGET_INDUSTRY]
            * n.program_budget
                .as_ref()
                .map_or(2000, |p| p.departments[BUDGET_INDUSTRY][0]) as f64
            / 10_000.0
    });
    let cost = industry::work_cost_bn(K::StarterIndustry) * scale;
    let available = if w.nation_opt(nation).is_some() {
        industry::project_authority(w, nation, K::StarterIndustry)
    } else {
        0.0
    };
    let lower_bound_days =
        (annual_authority_bn > 0.0 && annual_authority_bn.is_finite()).then(|| {
            ((cost - available).max(0.0) / annual_authority_bn * 365.0)
                .max(1800.0 * scale / production::construction_capacity(w, nation).min(1.5))
                .max(MIN_CALENDAR_DAYS as f64)
                .ceil() as u32
        });
    ModuleQuote {
        capacity_micros,
        scale,
        cost_bn: industry::work_cost_bn(K::StarterIndustry) * scale,
        recipe: production::catalog(K::StarterIndustry)
            .recipe
            .map(|r| r * scale),
        nominal_work_days: 1800.0 * scale,
        minimum_calendar_days: MIN_CALENDAR_DAYS,
        political_cost: production::catalog(K::StarterIndustry).political_cost,
        department: 0,
        output_daily: scale,
        power_capacity_daily: 10.0 * scale,
        grid_capacity_daily: 5.0 * scale,
        construction_capacity_daily: 0.15 * scale,
        annual_authority_bn,
        lower_bound_days,
        recommended_micros: recommended_capacity_micros(w, nation),
        reason: start_error(w, nation, district, capacity_micros),
    }
}
pub fn start_error(
    w: &WorldState,
    nation: NationId,
    district: &str,
    micros: u32,
) -> Option<String> {
    if !(1..=STANDARD_MICROS).contains(&micros) {
        return Some("Capacity must be 1–1,000,000 millionths of one standard package.".into());
    }
    if !clock::is_daily(w) || !w.rules.economic_competition {
        return Some("Starter Industry requires daily Economic Competition.".into());
    }
    if !programs::enrolled(w, nation) {
        return Some("Enact a department budget before ordering a Starter Industry module.".into());
    }
    if let Some(reason) =
        production::start_project_common_error(w, nation, district, K::StarterIndustry)
    {
        return Some(reason);
    }
    if COMPONENTS.iter().any(|k| {
        reserved_capacity(w, district, *k)
            .checked_add(micros)
            .is_none_or(|v| v > MAX_MICROS)
    }) {
        return Some("This package would exceed five standard capacities in this province, including existing facilities and queued work.".into());
    }
    if w.production.next_id.max(1).checked_add(1).is_none() {
        return Some("Construction project identifiers are exhausted.".into());
    }
    None
}
pub fn start(
    w: &mut WorldState,
    nation: NationId,
    district: &str,
    capacity_micros: u32,
) -> Result<(), String> {
    if let Some(reason) = start_error(w, nation, district, capacity_micros) {
        return Err(reason);
    }
    let id = w.production.next_id.max(1);
    let day = clock::absolute_day(w);
    w.production.next_id = id.checked_add(1).expect("validated identifier");
    w.production.projects.push(Project {
        id,
        nation,
        district: district.into(),
        kind: K::StarterIndustry,
        priority: Priority::Normal,
        status: ProjectStatus::Building,
        reason: None,
        progress_days: 0.0,
        total_days: 1800,
        resources_used: [0.0; 12],
        capacity_micros: Some(capacity_micros),
        started_day: Some(day),
    });
    w.production
        .industry
        .projects
        .insert(id, industry::ProjectFunding::default());
    w.headline(format!(
        "{} orders a {:.4}% Starter Industry module in {}.",
        nation.name(),
        capacity_micros as f64 / 10_000.0,
        district
    ));
    Ok(())
}
pub(crate) fn complete(w: &mut WorldState, p: &Project) {
    let micros = p.capacity_micros.expect("validated starter capacity");
    let slot = w
        .production
        .industry
        .modules
        .entry(p.district.clone())
        .or_default();
    *slot = slot
        .checked_add(micros)
        .filter(|v| *v <= MAX_MICROS)
        .expect("capacity reserved when ordered");
}
pub fn scale(p: &Project) -> f64 {
    if p.kind == K::StarterIndustry {
        p.capacity_micros.unwrap_or(0) as f64 / STANDARD_MICROS as f64
    } else {
        1.0
    }
}
/// Normalize paid physical work for the existing progress fraction. A package
/// cannot commission faster than 90 actual dates, even with banked authority.
pub fn normalized_advance(w: &WorldState, p: &Project, physical_capacity: f64) -> f64 {
    if p.kind != K::StarterIndustry {
        return physical_capacity;
    }
    let s = scale(p);
    let Some(start) = p.started_day else {
        return 0.0;
    };
    if !(s > 0.0 && s <= 1.0) || clock::absolute_day(w) < start {
        return 0.0;
    }
    let elapsed = clock::absolute_day(w)
        .saturating_sub(start)
        .saturating_add(1)
        .max(0) as f64;
    let daily = p.total_days as f64 / MIN_CALENDAR_DAYS as f64;
    (physical_capacity / s)
        .min(daily)
        .min((daily * elapsed - p.progress_days).max(0.0))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        init::world_1990,
        load,
        resources::{self, Commodity, ALL},
        save,
        world::GameRules,
    };
    const USA: NationId = NationId::USA;
    fn prepared() -> (WorldState, String) {
        let mut w = world_1990(GameRules {
            daily_simulation: true,
            economic_competition: true,
            production_system: true,
            resource_market: true,
            ..Default::default()
        });
        w.player = Some(USA);
        let year = w.year;
        programs::install(&mut w, USA, year, programs::default_departments());
        programs::begin_day(&mut w);
        for c in ALL {
            if c != Commodity::Oil {
                resources::set_stockpile_for_test(&mut w, USA, c, 1_000_000.0);
            }
        }
        let d = w
            .districts
            .iter()
            .find(|(_, n)| **n == USA)
            .unwrap()
            .0
            .clone();
        (w, d)
    }
    fn next_day(w: &mut WorldState) {
        programs::stage_fiscal(w.nation_mut(USA), 0.0, 0.0);
        programs::finish_day(w);
        clock::advance_date(w);
        programs::begin_day(w);
    }
    fn near(a: f64, b: f64) {
        assert!((a - b).abs() < 1e-10, "{a} != {b}");
    }
    #[test]
    fn proportional_quote_is_pure_and_complete_recipe_sum() {
        let (w, d) = prepared();
        let before = save(&w);
        let full = quote(&w, USA, &d, 1_000_000);
        let tiny = quote(&w, USA, &d, 1);
        near(full.cost_bn, 0.58);
        near(tiny.cost_bn, 0.00000058);
        assert_eq!(full.nominal_work_days, 1800.0);
        near(tiny.nominal_work_days, 0.0018);
        assert_eq!(tiny.minimum_calendar_days, 90);
        for i in 0..12 {
            near(
                full.recipe[i],
                COMPONENTS
                    .iter()
                    .map(|k| production::catalog(*k).recipe[i])
                    .sum(),
            );
            near(tiny.recipe[i], full.recipe[i] * 1e-6);
        }
        assert_eq!(save(&w), before);
    }
    #[test]
    fn range_ownership_ambiguous_and_overflow_orders_refuse_atomically() {
        let (mut w, d) = prepared();
        for micros in [0, 1_000_001, u32::MAX] {
            let before = save(&w);
            assert!(start(&mut w, USA, &d, micros).is_err());
            assert_eq!(before, save(&w));
        }
        let foreign = w
            .districts
            .iter()
            .find(|(_, n)| **n != USA)
            .unwrap()
            .0
            .clone();
        assert!(start(&mut w, USA, &foreign, 1).is_err());
        assert!(production::start_project(&mut w, USA, &d, K::StarterIndustry).is_err());
        w.production.next_id = u32::MAX;
        let before = save(&w);
        assert!(start(&mut w, USA, &d, 1).is_err());
        assert_eq!(before, save(&w));
    }
    #[test]
    fn tiny_module_needs_ninety_dates_and_consumes_exact_scaled_bill_and_inputs() {
        for micros in [1, 10_000] {
            let (mut w, d) = prepared();
            let q = quote(&w, USA, &d, micros);
            for c in ALL {
                if c != Commodity::Oil {
                    resources::set_stockpile_for_test(&mut w, USA, c, q.recipe[c.idx()]);
                }
            }
            start(&mut w, USA, &d, micros).unwrap();
            let mut paid = 0.0;
            for day in 0..90 {
                let before = w
                    .nation(USA)
                    .program_budget
                    .as_ref()
                    .unwrap()
                    .spent_today_bn[BUDGET_INDUSTRY][0];
                production::tick_day(&mut w);
                paid += w
                    .nation(USA)
                    .program_budget
                    .as_ref()
                    .unwrap()
                    .spent_today_bn[BUDGET_INDUSTRY][0]
                    - before;
                let once = save(&w);
                production::tick_day(&mut w);
                assert_eq!(save(&w), once, "same-day replay");
                if day < 89 {
                    assert_eq!(capacity(&w, &d), 0.0);
                    assert_eq!(w.production.projects.len(), 1);
                    next_day(&mut w);
                }
            }
            assert!(w.production.projects.is_empty());
            near(paid, q.cost_bn);
            near(capacity(&w, &d), q.scale);
            for c in ALL {
                if c != Commodity::Oil {
                    assert!(
                        resources::stockpile(&w, USA, c).abs() < 1e-9,
                        "unused {:?}",
                        c
                    );
                }
            }
            assert_eq!(production::level(&w, &d, K::CivilianIndustry), 0);
            assert_eq!(production::level(&w, &d, K::ProcessingPlant), 0);
            near(
                production::construction_capacity(&w, USA),
                1.25 + 0.15 * q.scale,
            );
            near(industry::power_capacity(&w, USA), 10.0 * q.scale);
            assert_eq!(
                industry::snapshot(&w, USA).goods,
                industry::Goods::default(),
                "completion grants no stock"
            );
        }
    }
    #[test]
    fn missing_input_blocks_money_and_work_then_paid_output_is_fractional() {
        let (mut w, d) = prepared();
        start(&mut w, USA, &d, 10_000).unwrap();
        resources::set_stockpile_for_test(&mut w, USA, Commodity::Copper, 0.0);
        let cash = w
            .nation(USA)
            .program_budget
            .as_ref()
            .unwrap()
            .spent_today_bn;
        production::tick_day(&mut w);
        assert_eq!(w.production.projects[0].progress_days, 0.0);
        assert_eq!(
            w.nation(USA)
                .program_budget
                .as_ref()
                .unwrap()
                .spent_today_bn,
            cash
        );
        resources::set_stockpile_for_test(&mut w, USA, Commodity::Copper, 100.0);
        for _ in 0..90 {
            production::tick_day(&mut w);
            next_day(&mut w);
        }
        assert!(w.production.projects.is_empty());
        industry::tick_day(&mut w);
        let s = industry::snapshot(&w, USA);
        let row = s
            .sites
            .iter()
            .find(|s| s.kind == K::StarterIndustry)
            .unwrap();
        near(row.output_daily, 0.01);
        near(row.power_used_daily, 0.01);
        assert_eq!(row.level, 0);
        assert_eq!(row.capacity_micros, Some(10_000));
        near(s.goods.intermediates, 0.01);
        near(s.goods.capital_goods, 0.0);
        let once = save(&w);
        industry::tick_day(&mut w);
        assert_eq!(save(&w), once);
    }
    #[test]
    fn expansions_pay_added_capacity_and_reserve_caps_against_legacy_orders() {
        let (mut w, d) = prepared();
        w.production.industry.modules.insert(d.clone(), 4_500_000);
        assert!(start(&mut w, USA, &d, 500_001).is_err());
        assert!(production::start_project(&mut w, USA, &d, K::CivilianIndustry).is_err());
        start(&mut w, USA, &d, 500_000).unwrap();
        let p = &w.production.projects[0];
        near(industry::project_cost_bn(p), 0.29);
        let id = p.id;
        production::cancel_project(&mut w, USA, id).unwrap();
        near(capacity(&w, &d), 4.5);
        production::complete_capability(&mut w, &d, K::CivilianIndustry);
        assert!(
            start(&mut w, USA, &d, 1).is_err(),
            "existing normal capacity counts too"
        );
    }
    #[test]
    fn full_facility_requires_a_whole_paid_estate_but_retrofits_recognize_partial_processing() {
        let (mut w, d) = prepared();
        w.production.industry.modules.insert(d.clone(), 10_000);
        assert!(production::start_project_error(&w, USA, &d, K::MachineryWorks).is_some());
        let upgrade = industry::project_refusal(&w, USA, &d, K::Automation);
        assert!(!upgrade
            .as_deref()
            .is_some_and(|r| r.contains("Build a Machinery")));
        let old_rate = industry::plant_rate(&w, &d, K::StarterIndustry);
        industry::complete_site(&mut w, &d, K::Automation);
        near(
            industry::plant_rate(&w, &d, K::StarterIndustry),
            old_rate * 1.2,
        );
        w.production
            .industry
            .modules
            .insert(d.clone(), STANDARD_MICROS);
        assert!(production::start_project_error(&w, USA, &d, K::MachineryWorks).is_none());
        assert_eq!(production::level(&w, &d, K::CivilianIndustry), 0);
    }
    #[test]
    fn mixed_queue_shares_actual_work_and_department_authority() {
        let (mut w, d) = prepared();
        let other = w
            .districts
            .iter()
            .find(|(x, n)| **n == USA && **x != d)
            .unwrap()
            .0
            .clone();
        start(&mut w, USA, &d, 100_000).unwrap();
        production::start_project(&mut w, USA, &other, K::CivilianIndustry).unwrap();
        w.nation_mut(USA)
            .program_budget
            .as_mut()
            .unwrap()
            .available_bn[BUDGET_INDUSTRY][0] = 0.00001;
        let plans = industry::project_plans(&w);
        let physical: f64 = w
            .production
            .projects
            .iter()
            .filter_map(|p| {
                plans
                    .get(&p.id)
                    .filter(|plan| plan.reason.is_none())
                    .map(|plan| plan.advance_days * scale(p))
            })
            .sum();
        assert!(physical <= production::construction_capacity(&w, USA));
        let money: f64 = plans
            .values()
            .filter(|p| p.reason.is_none())
            .map(|p| p.cash_bn)
            .sum();
        assert!(money <= 0.00001);
        let before = w
            .nation(USA)
            .program_budget
            .as_ref()
            .unwrap()
            .spent_today_bn[BUDGET_INDUSTRY][0];
        production::tick_day(&mut w);
        let spent = w
            .nation(USA)
            .program_budget
            .as_ref()
            .unwrap()
            .spent_today_bn[BUDGET_INDUSTRY][0]
            - before;
        near(spent, money);
    }
    #[test]
    fn unfinished_module_replays_identically_after_reload_and_keeps_frozen_price() {
        let (mut uninterrupted, d) = prepared();
        start(&mut uninterrupted, USA, &d, 10_003).unwrap();
        for _ in 0..31 {
            production::tick_day(&mut uninterrupted);
            next_day(&mut uninterrupted);
        }
        let mut resumed = load(&save(&uninterrupted)).unwrap();
        let frozen = industry::project_cost_bn(&resumed.production.projects[0]);
        for _ in 31..90 {
            production::tick_day(&mut uninterrupted);
            next_day(&mut uninterrupted);
            production::tick_day(&mut resumed);
            next_day(&mut resumed);
        }
        assert_eq!(save(&uninterrupted), save(&resumed));
        near(frozen, 0.58 * 0.010003);
        near(capacity(&resumed, &d), 0.010003);
    }
    #[test]
    fn scale_survives_save_capture_and_gdp_changes_without_integer_unlock() {
        let (mut w, d) = prepared();
        w.production.industry.modules.insert(d.clone(), 17_001);
        let old = capacity(&w, &d);
        let encoded = save(&w);
        let mut loaded = load(&encoded).unwrap();
        assert_eq!(encoded, save(&loaded));
        loaded.nation_mut(USA).gdp *= 10.0;
        assert_eq!(capacity(&loaded, &d), old);
        loaded.districts.insert(d.clone(), NationId::Japan);
        near(industry::power_capacity(&loaded, USA), 0.0);
        near(
            industry::power_capacity(&loaded, NationId::Japan),
            old * 10.0,
        );
        assert_eq!(capacity(&loaded, &d), old);
        assert_eq!(production::level(&loaded, &d, K::PowerGrid), 0);
    }
    #[test]
    fn legacy_save_stays_absent_and_recommendation_fits_money_and_work() {
        let w = world_1990(GameRules::default());
        let old = save(&w);
        assert!(!old.contains("capacity_micros"));
        assert!(!old.contains("\"modules\""));
        assert_eq!(save(&load(&old).unwrap()), old);
        let (w, d) = prepared();
        let size = recommended_capacity_micros(&w, USA);
        let q = quote(&w, USA, &d, size);
        assert!(q.cost_bn <= q.annual_authority_bn);
        assert!(q.nominal_work_days <= 365.0 * production::construction_capacity(&w, USA).min(1.5));
    }
    #[test]
    fn recommendation_fits_exact_slowest_calendar_release_cash_bill() {
        let (mut w, d) = prepared();
        for gdp in [0.1, 0.112, 1.6964999999999997, 2.5, 37.0] {
            w.nation_mut(USA).gdp = gdp;
            let n = w.nation(USA);
            let annual = n.gdp
                * n.budget_for(w.year).allocations[BUDGET_INDUSTRY]
                * (n.program_budget.as_ref().unwrap().departments[BUDGET_INDUSTRY][0] as f64
                    / 10_000.0);
            let micros = recommended_capacity_micros(&w, USA);
            let q = quote(&w, USA, &d, micros);
            assert!(q.cost_bn.is_finite() && q.cost_bn > 0.0);
            assert!(
                q.cost_bn <= annual * (365.0 / (12.0 * 31.0)),
                "{gdp}: {micros} micros over calendar cash bound"
            );
        }
    }
}
