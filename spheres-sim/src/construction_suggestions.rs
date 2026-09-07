//! Pure, bounded advice for the current player. Suggestions reserve no money,
//! issue no orders and do not opt the campaign into the economic AI pilot.
use crate::{
    clock,
    commerce::Good,
    districts, industrial_modules as modules, industry, industry_planning,
    production::{self, ProjectKind as K},
    programs,
    world::{NationId, WorldState},
};
use serde::Serialize;

const EPS: f64 = 1e-9;

#[derive(Clone, Debug, Serialize, PartialEq)]
pub struct Suggestion {
    pub id: String,
    pub project_kind: K,
    pub district: String,
    pub capacity_micros: Option<u32>,
    pub name: String,
    pub priority: String,
    pub reason: String,
    pub evidence: Vec<String>,
    pub caution: Option<String>,
}
#[derive(Clone, Debug, Serialize, PartialEq)]
pub struct Suggestions {
    pub as_of_day: i32,
    pub items: Vec<Suggestion>,
    pub note: String,
}
struct Ranked {
    rank: u8,
    gap: f64,
    item: Suggestion,
}

fn eligible(w: &WorldState, nation: NationId, district: &str, kind: K, size: Option<u32>) -> bool {
    if kind == K::StarterIndustry {
        size.is_some_and(|v| modules::start_error(w, nation, district, v).is_none())
    } else {
        production::start_project_error(w, nation, district, kind).is_none()
    }
}
fn item(
    w: &WorldState,
    nation: NationId,
    district: &str,
    kind: K,
    size: Option<u32>,
    priority: &str,
    reason: String,
    evidence: Vec<String>,
    caution: &str,
) -> Suggestion {
    let funding = if !programs::enrolled(w, nation)
        || programs::construction_daily_budget_bn(w, nation) <= 0.0
    {
        " Apply a positive construction budget before this order can advance."
    } else {
        ""
    };
    Suggestion {
        id: format!("{}:{}:{}", kind.key(), district, size.unwrap_or(0)),
        project_kind: kind,
        district: district.into(),
        capacity_micros: size,
        name: production::catalog(kind).name.into(),
        priority: priority.into(),
        reason,
        evidence,
        caution: Some(format!("{caution}{funding}")),
    }
}
/// Prefer an already invested site, then local grid and freight infrastructure.
/// This is a transparent placement heuristic, never a claim of highest returns.
fn ordered_sites<'a>(
    w: &WorldState,
    plan: &'a industry_planning::CapacityPlan,
) -> Vec<&'a industry_planning::ProvinceCapacity> {
    let mut sites: Vec<_> = plan.provinces.iter().filter(|p| !p.contested).collect();
    sites.sort_by(|a, b| {
        let paid = |p: &industry_planning::ProvinceCapacity| {
            p.processing_daily + p.machinery_daily + p.estate
        };
        paid(b)
            .total_cmp(&paid(a))
            .then(b.grid_daily.total_cmp(&a.grid_daily))
            .then(
                production::level(w, &b.district, K::Infrastructure).cmp(&production::level(
                    w,
                    &a.district,
                    K::Infrastructure,
                )),
            )
            .then(a.district.cmp(&b.district))
    });
    sites
}
fn placement(site: &industry_planning::ProvinceCapacity, w: &WorldState) -> String {
    format!("Site has {:.6} estate equivalents, {:.6} installed civilian packs/day, {:.6} local grid units/day and infrastructure level {}. Equally suitable sites use stable province order; this is not a forecast of economic returns.",
        site.estate,site.processing_daily+site.machinery_daily,site.grid_daily,production::level(w,&site.district,K::Infrastructure))
}
fn queued_summary(w: &WorldState, nation: NationId) -> Option<String> {
    let project = production::projects_for(w, nation)
        .filter(|p| w.districts.get(&p.district) == Some(&nation))
        .filter(|p| {
            matches!(
                p.kind,
                K::StarterIndustry
                    | K::CivilianIndustry
                    | K::PowerGrid
                    | K::Generation
                    | K::ProcessingPlant
                    | K::MachineryWorks
                    | K::Warehouse
                    | K::Automation
                    | K::Efficiency
            )
        })
        .min_by_key(|p| (if p.kind == K::StarterIndustry { 0 } else { 1 }, p.id))?;
    Some(format!("{} in {} is already queued. Its committed capacity is counted here; finish or review funded work before buying a duplicate industrial step.",production::catalog(project.kind).name,districts::name_of(&project.district).unwrap_or(&project.district)))
}
/// Only recent settled operation evidence can diagnose a raw/cash problem.
/// An empty stockpile alone may merely mean today's inputs were just consumed.
fn operating_blocker(w: &WorldState, nation: NationId) -> Option<String> {
    let today = clock::absolute_day(w);
    if !w
        .production
        .industry
        .last_day
        .is_some_and(|day| day >= today - 1 && day <= today)
    {
        return None;
    }
    w.production.industry.operations.iter().filter(|o|w.districts.get(&o.district)==Some(&nation))
        .filter_map(|o| {
            let reason=o.reason.as_deref()?;
            let lower=reason.to_ascii_lowercase();
            (lower.contains("complete operating bundle")||lower.contains("raw inputs")||lower.contains("supply limits")
                ||lower.contains("no department operating authority")||lower.contains("active department budget"))
                .then(||format!("{} in {} reports: {} Restore its operating supplies or funding; additional factory capacity does not resolve that constraint.",production::catalog(o.kind).name,districts::name_of(&o.district).unwrap_or(&o.district),reason))
        }).next()
}

pub fn suggestions(w: &WorldState, nation: NationId) -> Suggestions {
    let mut out=Suggestions {as_of_day:clock::absolute_day(w),items:vec![],
        note:"Suggestions use installed and queued capacity, not promised GDP, tax income, jobs or investment returns. Review a project before ordering; nothing is started automatically.".into()};
    if w.player != Some(nation) || !w.nation_opt(nation).is_some_and(|n| n.alive) {
        out.note =
            "Construction advice is available for the current, active player government.".into();
        return out;
    }
    if !clock::is_daily(w) || !w.rules.production_system {
        out.note = "Construction advice requires daily construction to be enabled.".into();
        return out;
    }
    let plan = industry_planning::plan(w, nation);
    let sites = ordered_sites(w, &plan);
    if sites.is_empty() {
        out.note =
            "No controlled, uncontested province is eligible for a construction suggestion.".into();
        return out;
    }
    let load: f64 = sites.iter().map(|p| p.power_required_daily).sum();
    let installed = industry::power_capacity(w, nation);
    // Include blocked/paused owned projects: their paid capacity is still a
    // commitment, not an invitation to buy a duplicate solution elsewhere.
    let generation_committed = plan.generation_committed_daily;
    let generation_gap = (load - installed - generation_committed).max(0.0);
    let conflict_generation = (plan.generation_daily - installed).max(0.0);
    let operations = operating_blocker(w, nation);
    let mut candidates = vec![];
    let capacity_caution="These are full-capacity requirements, not measured lost output. Operating inputs, funds and storage still determine actual production.";

    if generation_gap > EPS && generation_gap > conflict_generation + EPS {
        if let Some(site) = sites
            .iter()
            .find(|p| eligible(w, nation, &p.district, K::Generation, None))
        {
            candidates.push(Ranked {rank:0,gap:generation_gap,item:item(w,nation,&site.district,K::Generation,None,"Bottleneck",
                format!("Installed and planned plants need {:.6} power units/day at full capacity; installed and already-queued generation provide {:.6}.",load,installed+generation_committed),
                vec![format!("Uncontested installed generation: {:.6}; queued generation: {:.6}; remaining full-capacity gap: {:.6} units/day.",installed,generation_committed,generation_gap),placement(site,w)],capacity_caution)});
        }
    }
    for site in &sites {
        let supplied = site.grid_daily + site.grid_committed_daily;
        let gap = (site.power_required_daily - supplied).max(0.0);
        if gap > EPS && eligible(w, nation, &site.district, K::PowerGrid, None) {
            candidates.push(Ranked {rank:1,gap,item:item(w,nation,&site.district,K::PowerGrid,None,"Bottleneck",
                format!("This province's installed and planned lines need {:.6} power units/day at full capacity, above its {:.6} installed and queued local grid ceiling.",site.power_required_daily,supplied),
                vec![format!("Installed local grid: {:.6}; queued local grid: {:.6}; remaining full-capacity gap: {:.6} units/day.",site.grid_daily,site.grid_committed_daily,gap),
                    "Grid capacity belongs to this province; spare grid capacity elsewhere cannot cover this local limit.".into()],capacity_caution)});
        }
    }
    let storage = plan.storage + plan.storage_committed;
    let tight = plan
        .goods
        .iter()
        .filter(|g| {
            g.demand_daily > EPS
                && g.stock + g.incoming + g.contracted_remaining + EPS >= storage * 0.9
                && g.demand_daily * industry_planning::STOCK_COVER_DAYS > storage + EPS
        })
        .max_by(|a, b| a.demand_daily.total_cmp(&b.demand_daily));
    if let Some(good) = tight {
        if let Some(site) = sites
            .iter()
            .find(|p| eligible(w, nation, &p.district, K::Warehouse, None))
        {
            let label = if good.good == Good::Intermediates {
                "intermediate packs"
            } else {
                "capital-goods packs"
            };
            candidates.push(Ranked {rank:2,gap:good.demand_daily*industry_planning::STOCK_COVER_DAYS-storage,
                item:item(w,nation,&site.district,K::Warehouse,None,"Bottleneck",
                format!("Storage for {label} is tight against current modeled use; a full pile alone would not justify an expansion."),
                vec![format!("Demand: {:.6} packs/day; 90-day planning buffer: {:.6}; installed plus queued storage: {:.6} packs for this good.",good.demand_daily,good.demand_daily*industry_planning::STOCK_COVER_DAYS,storage),
                    format!("Stock plus paid incoming and finite domestic orders: {:.6} packs.",good.stock+good.incoming+good.contracted_remaining),placement(site,w)],
                "Storage is shared nationally and applies separately to each good. More space creates neither buyers nor inventory; the 90-day buffer is a planning policy, not a profit forecast.")});
        }
    }

    // One optional upstream remedy for demonstrated domestic intermediate use.
    // Do not expand a raw/cash-starved processor or create customers from GDP.
    if operations.is_none() {
        if let Some(g) = plan
            .goods
            .iter()
            .find(|g| g.good == Good::Intermediates && g.domestic_daily > EPS)
        {
            let gap = (g.domestic_daily * industry_planning::CAPACITY_HEADROOM
                - g.installed_daily
                - g.committed_daily
                - g.contracted_daily)
                .max(0.0);
            let covered = g.stock + g.incoming + g.contracted_remaining;
            if gap > EPS && covered + EPS < g.domestic_daily * industry_planning::STOCK_COVER_DAYS {
                if let Some(site) = sites.iter().find(|p| {
                    // Match CapacityPlan's committed site-wide upgrades: they
                    // also affect the proposed line after commissioning.
                    let future_level = |kind| {
                        production::level(w, &p.district, kind) as f64
                            + production::projects_for(w, nation)
                                .filter(|project| {
                                    project.district == p.district && project.kind == kind
                                })
                                .count() as f64
                    };
                    let output = 1.0 + future_level(K::Automation) * 0.2;
                    let added = output * (1.0 - future_level(K::Efficiency) * 0.1).max(0.5);
                    eligible(w, nation, &p.district, K::ProcessingPlant, None)
                        && output <= gap + EPS
                        && p.grid_daily + p.grid_committed_daily + EPS
                            >= p.power_required_daily + added
                        && installed + generation_committed + EPS >= load + added
                }) {
                    candidates.push(Ranked {rank:3,gap,item:item(w,nation,&site.district,K::ProcessingPlant,None,"Bottleneck",
                        format!("Domestic machinery or prototype plans use {:.6} intermediate packs/day; installed, queued and contracted processing leave {:.6} packs/day of headroom unmet.",g.domestic_daily,gap),
                        vec![format!("Installed processing: {:.6}; queued: {:.6}; contracted: {:.6} packs/day. The planning allowance is 25% above domestic use.",g.installed_daily,g.committed_daily,g.contracted_daily),
                            format!("Stock and paid supply cover {:.2} days of that use, below the 90-day planning buffer.",covered/g.domestic_daily),placement(site,w)],
                        "This adds rated supply, not guaranteed sales. Processing still needs iron, bauxite, coal, power and operating funds; review the exact project preview before ordering.")});
                }
            }
        }
    }
    candidates.sort_by(|a, b| {
        a.rank
            .cmp(&b.rank)
            .then(b.gap.total_cmp(&a.gap))
            .then(a.item.district.cmp(&b.item.district))
    });
    out.items = candidates.into_iter().take(3).map(|c| c.item).collect();

    let productive = plan.provinces.iter().any(|p| {
        p.processing_daily
            + p.machinery_daily
            + p.processing_committed_daily
            + p.machinery_committed_daily
            > EPS
    });
    let queued = queued_summary(w, nation);
    if out.items.is_empty() && !productive && queued.is_none() {
        let daily = programs::construction_daily_budget_bn(w, nation);
        if daily > 0.0 {
            let size = modules::recommended_capacity_micros(w, nation);
            if let Some(site) = sites
                .iter()
                .find(|p| eligible(w, nation, &p.district, K::StarterIndustry, Some(size)))
            {
                out.items.push(item(w,nation,&site.district,K::StarterIndustry,Some(size),"Development",
                    "Consider a first self-contained starter workshop: its paid package includes fractional processing, estate, generation and local grid capacity. This is a development option, not an observed shortage.".into(),
                    vec![format!("No commissioned or queued intermediate/capital-goods line is recorded. Suggested package: {:.6} of one standard workshop, sized against the current {:.6} $bn/day construction budget.",size as f64/modules::STANDARD_MICROS as f64,daily),placement(site,w)],
                    "The national economy already includes inherited industry. This adds a new modeled workshop; check future intermediate use and real operating inputs rather than assuming GDP, jobs or tax returns."));
            }
        } else {
            out.note="Set a positive construction budget to size a first-workshop development option. No arbitrary minimum-size order is presented as affordable while construction is paused.".into();
        }
    }
    if out.items.is_empty() {
        if let Some(why) = queued {
            out.note = why;
        } else if let Some(why) = operations {
            out.note = why;
        } else if generation_gap > EPS && generation_gap <= conflict_generation + EPS {
            out.note="Existing generation is unavailable in contested provinces. Restore control or review those assets before buying duplicate capacity.".into();
        } else if productive {
            out.note="No eligible construction bottleneck is demonstrated after counting installed capacity, queued work and paid supply. A full stockpile without demand is not a reason to build more; review operating supplies and funding first.".into();
        }
    } else if let Some(why) = operations {
        out.note = why;
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{init::world_1990, save, world::GameRules};
    const USA: NationId = NationId::USA;
    fn prepared() -> (WorldState, Vec<String>) {
        let mut w = world_1990(GameRules {
            daily_simulation: true,
            production_system: true,
            resource_market: true,
            ..Default::default()
        });
        w.player = Some(USA);
        programs::set_construction_budget(&mut w, USA, 0.001).unwrap();
        let d = w
            .districts
            .iter()
            .filter(|(_, n)| **n == USA)
            .map(|(d, _)| d.clone())
            .collect();
        (w, d)
    }
    fn built(w: &mut WorldState, d: &str, k: K) {
        production::complete_capability(w, d, k);
    }
    #[test]
    fn advice_is_pure_deterministic_and_does_not_require_the_ai_pilot() {
        let (w, _) = prepared();
        assert!(!w.rules.economic_competition);
        let before = save(&w);
        let a = suggestions(&w, USA);
        let b = suggestions(&w, USA);
        assert_eq!(a, b);
        assert_eq!(save(&w), before);
        assert_eq!(a.items.len(), 1);
        assert_eq!(a.items[0].project_kind, K::StarterIndustry);
        assert_eq!(a.items[0].priority, "Development");
        assert!(a.items[0].reason.contains("not an observed shortage"));
        assert!(eligible(
            &w,
            USA,
            &a.items[0].district,
            K::StarterIndustry,
            a.items[0].capacity_micros
        ));
    }
    #[test]
    fn queued_workshop_suppresses_redundant_startup_and_its_own_utilities() {
        let (mut w, d) = prepared();
        modules::start(&mut w, USA, &d[0], 100_000).unwrap();
        let before = save(&w);
        let advice = suggestions(&w, USA);
        assert!(advice.items.is_empty());
        assert!(advice.note.contains("Starter Industry"));
        assert!(advice.note.contains(districts::name_of(&d[0]).unwrap()));
        assert_eq!(save(&w), before);
    }
    #[test]
    fn real_full_capacity_power_and_grid_gaps_have_numeric_evidence() {
        let (mut w, d) = prepared();
        built(&mut w, &d[1], K::CivilianIndustry);
        built(&mut w, &d[1], K::ProcessingPlant);
        let advice = suggestions(&w, USA);
        assert!(advice.items.len() <= 3);
        for kind in [K::Generation, K::PowerGrid] {
            let p = advice
                .items
                .iter()
                .find(|p| p.project_kind == kind)
                .unwrap();
            assert_eq!(p.district, d[1]);
            assert_eq!(p.priority, "Bottleneck");
            assert!(p.reason.contains("full capacity"));
            assert!(p.evidence.iter().any(|e| e.contains("1.000000")));
            assert!(p
                .caution
                .as_deref()
                .unwrap()
                .contains("not measured lost output"));
        }
        production::start_project(&mut w, USA, &d[1], K::Generation).unwrap();
        production::start_project(&mut w, USA, &d[1], K::PowerGrid).unwrap();
        let next = suggestions(&w, USA);
        assert!(!next
            .items
            .iter()
            .any(|p| matches!(p.project_kind, K::Generation | K::PowerGrid)));
    }
    #[test]
    fn no_warehouse_is_suggested_for_unsold_stock_without_use() {
        let (mut w, d) = prepared();
        for k in [
            K::CivilianIndustry,
            K::ProcessingPlant,
            K::PowerGrid,
            K::Generation,
        ] {
            built(&mut w, &d[0], k);
        }
        w.production.industry.goods.insert(
            USA,
            industry::Goods {
                intermediates: 250.0,
                capital_goods: 250.0,
            },
        );
        let advice = suggestions(&w, USA);
        assert!(advice.items.is_empty());
        assert!(advice.note.contains("without demand"));
    }
    #[test]
    fn processing_order_must_fit_the_evidenced_gap_and_count_queued_supply() {
        let (mut w, d) = prepared();
        for k in [
            K::CivilianIndustry,
            K::MachineryWorks,
            K::PowerGrid,
            K::Generation,
        ] {
            built(&mut w, &d[0], k);
        }
        // One machine line uses 0.5/day: even the 25% planning allowance
        // leaves only 0.625/day, too little to justify a full 1/day processor.
        let has_processing = |w: &WorldState| {
            suggestions(w, USA)
                .items
                .iter()
                .any(|p| p.project_kind == K::ProcessingPlant)
        };
        let plan = industry_planning::plan(&w, USA);
        let intermediates = plan
            .goods
            .iter()
            .find(|g| g.good == Good::Intermediates)
            .unwrap();
        assert!((intermediates.domestic_daily - 0.5).abs() < EPS);
        assert!(!has_processing(&w));

        // Two machine lines establish enough domestic use for one full plant.
        built(&mut w, &d[0], K::MachineryWorks);
        assert!(has_processing(&w));
        production::start_project(&mut w, USA, &d[0], K::ProcessingPlant).unwrap();
        assert!(!has_processing(&w));

        // Queued automation applies to the proposed line too: current output
        // of 1/day fits this 1.05/day gap, but its committed 1.2/day does not.
        let (mut w, d) = prepared();
        for k in [
            K::CivilianIndustry,
            K::Generation,
            K::PowerGrid,
            K::PowerGrid,
            K::ProcessingPlant,
            K::MachineryWorks,
            K::MachineryWorks,
            K::MachineryWorks,
        ] {
            built(&mut w, &d[0], k);
        }
        production::start_project(&mut w, USA, &d[0], K::Automation).unwrap();
        let plan = industry_planning::plan(&w, USA);
        let intermediates = plan
            .goods
            .iter()
            .find(|g| g.good == Good::Intermediates)
            .unwrap();
        assert!((intermediates.expansion_daily - 1.05).abs() < EPS);
        assert!(!has_processing(&w));
    }
    #[test]
    fn paused_budget_and_ineligible_actor_produce_no_arbitrary_order() {
        let (mut w, _) = prepared();
        programs::set_construction_budget(&mut w, USA, 0.0).unwrap();
        let advice = suggestions(&w, USA);
        assert!(advice.items.is_empty());
        assert!(advice.note.contains("positive construction budget"));
        assert!(suggestions(&w, NationId::Canada).items.is_empty());
        w.rules.production_system = false;
        assert!(suggestions(&w, USA).items.is_empty());
    }
    #[test]
    fn first_workshop_prefers_existing_compatible_investment_and_explains_ties() {
        let (mut w, d) = prepared();
        built(&mut w, &d[2], K::PowerGrid);
        let advice = suggestions(&w, USA);
        let p = &advice.items[0];
        assert_eq!(p.project_kind, K::StarterIndustry);
        assert_eq!(p.district, d[2]);
        assert!(p
            .evidence
            .iter()
            .any(|s| s.contains("stable province order")));
    }
}
