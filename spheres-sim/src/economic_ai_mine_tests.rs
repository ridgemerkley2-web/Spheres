//! Test-only original eager mine read and exact review/day oracles. Explicit
//! synthetic stocks/sites below never alter any qualification campaign.
use super::*;
use std::cell::Cell;

thread_local! {
    pub(super) static ORIGINAL: Cell<bool> = const { Cell::new(false) };
    pub(super) static REUSED: Cell<u64> = const { Cell::new(0) };
}

fn original_reads<T>(work: impl FnOnce() -> T) -> T {
    struct Reset(bool);
    impl Drop for Reset {
        fn drop(&mut self) { ORIGINAL.with(|flag| flag.set(self.0)); }
    }
    let _reset = Reset(ORIGINAL.with(|flag| flag.replace(true)));
    work()
}

// Exact pre-optimization selection body. In original mode the full review
// always takes the second fresh raw forecast when it records its decision.
pub(super) fn original_mine_for_shortage(
    w: &WorldState,
    nation: NationId,
    raw_context: &RawSupplyContext,
) -> Option<(String, Commodity)> {
    if w.resources.mine_projects.iter().any(|p| p.started_by == nation) {
        return None;
    }
    let today = clock::absolute_day(w);
    if w.resources.market.as_ref()
        .is_none_or(|market| market.last_cleared_day != Some(today))
    {
        return None;
    }
    let demand = resources::automatic_tick_draw(w, nation);
    let forecast = raw_supply_forecast_with_context(w, nation, raw_context);
    for c in resources::ALL {
        let stock = resources::stockpile(w, nation, c);
        let run_gap = forecast.lines[c.idx()].shortage[0];
        if demand[c.idx()] <= stock || run_gap <= 1e-9 {
            continue;
        }
        if resources::has_new_inbound_contract(w, nation, c)
            || w.resources.offers.iter().any(|offer| {
                offer.from == nation
                    && Some(offer.to) == w.player
                    && resources::offer_refusal(w, offer.to, offer.id).is_none()
                    && offer.take.iter().any(|leg| {
                        matches!(leg, resources::Leg::Commodity { c: asked, .. } if *asked == c)
                    })
            })
        {
            continue;
        }
        let foreign_producers: Vec<_> = resources::producers(w, c)
            .into_iter().filter(|producer| *producer != nation).collect();
        let peaceful_option = foreign_producers.iter().any(|producer| {
            resources::peaceful_supplier_available(w, nation, *producer, c)
        });
        let trade_closed = !peaceful_option || resources::refused_all(w, nation, c).is_some();
        if !trade_closed { continue; }
        for (district, owner) in &w.districts {
            if *owner == nation && resources::mine_refusal(w, nation, district, c).is_none() {
                return Some((district.clone(), c));
            }
        }
    }
    None
}

const ME: NationId = NationId::USA;

fn set_raw(w: &mut WorldState, commodity: Commodity, quantity: f64) {
    let stocks = &mut w.resources.market.as_mut().unwrap().stocks;
    match stocks.binary_search_by_key(&(ME, commodity), |s| (s.nation, s.commodity)) {
        Ok(i) => stocks[i].quantity = quantity,
        Err(i) => stocks.insert(i, resources::Stock {
            nation: ME, commodity, quantity, reserve_target: 0.0,
        }),
    }
}

// Matches the existing economic_ai_supply integration fixture's real bauxite
// deficit: an installed processor, mapped domestic deposit, no domestic flow.
fn prepared(active: bool, closed: bool) -> (WorldState, String) {
    let mut w = crate::init::world_1990(GameRules {
        daily_simulation: true, economic_competition: true,
        production_system: true, resource_gates: true, resource_market: true,
        manufacturing_system: true, physical_logistics: true,
        logistics_routes: true, ai_aggression: 0.0, ..Default::default()
    });
    crate::starting_industry::enable_new_world(&mut w).unwrap();
    crate::province_economy::enable(&mut w);
    w.conflicts.clear(); w.sanctions.clear();
    w.player = Some(NationId::Tonga);
    w.nation_mut(ME).political_capital = 1000.0;
    w.nation_mut(ME).debt_gdp = 0.0;
    let allocations = w.nation(ME).budget_for(w.year).allocations;
    let mut departments = programs::default_departments();
    if active { departments[BUDGET_INDUSTRY] = [6000, 1000, 1000, 1000, 1000]; }
    crate::apply_command(&mut w, &Command::SetProgramBudget {
        nation: ME, fiscal_year: 1990, allocations, departments,
    }).unwrap();
    w.nation_mut(ME).treasury_bn = Some(100.0);
    w.nation_mut(ME).debt_bn = Some(0.0);
    let district = w.districts.iter().filter(|(_, owner)| **owner == ME)
        .map(|(d, _)| d.clone())
        .find(|d| crate::materials::capacity_daily(&w, d) >= 0.5).unwrap();
    w.production.provinces.retain(|p| p.district != district);
    w.production.provinces.push(production::ProvinceCapabilities {
        district: district.clone(), civilian_industry: 1, power_grid: 3,
        infrastructure: 0, research_centers: 0, arms_plants: 0,
    });
    w.production.provinces.sort_by(|a,b| a.district.cmp(&b.district));
    resources::tick(&mut w);
    w.production.industry.sites.insert(district.clone(), [1, 2, 1, 0, 0, 0, 0]);
    w.production.industry.goods.entry(ME).or_default().intermediates = 45.0;
    for c in resources::ALL { set_raw(&mut w, c, 1000.0); }
    set_raw(&mut w, Commodity::Bauxite, 0.0);
    if closed {
        for producer in resources::producers(&w, Commodity::Bauxite) {
            if producer != ME { w.sanctions.push((producer, ME)); }
        }
    }
    programs::begin_day(&mut w);
    if active {
        crate::apply_command(&mut w, &Command::StartProject {
            nation: ME, district: district.clone(), kind: K::CivilianIndustry,
        }).unwrap();
    }
    crate::arsenal::tick(&mut w);
    // The open fixture needs an unresolved opening shortage, not stock bought
    // during the normal market clearing used to establish today's opportunity.
    set_raw(&mut w, Commodity::Bauxite, 0.0);
    assert_eq!(w.resources.market.as_ref().unwrap().last_cleared_day,
        Some(clock::absolute_day(&w)));
    assert!(resources::automatic_tick_draw(&w, ME)[Commodity::Bauxite.idx()] > 0.0);
    (w, district)
}

fn candidate(review: &MineReview) -> Option<(String, Commodity)> {
    match review {
        MineReview::Candidate(d, c) => Some((d.clone(), *c)),
        MineReview::NoCommand(_) => None,
    }
}

#[test]
fn mine_review_preserves_eager_selection_and_exact_public_forecast() {
    let (base, _) = prepared(false, true);
    let mut candidates = 0;
    let mut retained = 0;
    let mut deferred = 0;
    for case in 0..9 {
        let mut w = base.clone();
        match case {
            1 => { w.sanctions.clear(); }
            2 => { for c in resources::ALL { set_raw(&mut w,c,1e9); } }
            3 => { w.resources.market.as_mut().unwrap().last_cleared_day = None; }
            4 => {
                let context = RawSupplyContext::new(&w);
                let (district, commodity) = original_mine_for_shortage(&w,ME,&context).unwrap();
                crate::apply_command(&mut w,&Command::DevelopResource {nation:ME,district,commodity}).unwrap();
            }
            5 => set_raw(&mut w, Commodity::Bauxite, -0.0),
            6 => set_raw(&mut w, Commodity::Bauxite, f64::NAN),
            7 => set_raw(&mut w, Commodity::Bauxite, f64::INFINITY),
            8 => { crate::logistics::set_policy(&mut w,ME,crate::logistics::RoutePolicy::LandOnly).unwrap(); }
            _ => {}
        }
        let before = crate::save(&w);
        let context = RawSupplyContext::new(&w);
        let public = raw_supply_forecast_with_context(&w,ME,&context);
        let expected = original_mine_for_shortage(&w,ME,&context);
        let actual = mine_for_shortage(&w,ME,&context);
        assert_eq!(candidate(&actual),expected,"exact district/commodity selection case {case}");
        match actual {
            MineReview::Candidate(_,_) => candidates += 1,
            MineReview::NoCommand(Some(forecast)) => {
                retained += 1;
                assert_eq!(serde_json::to_vec(&forecast).unwrap(),serde_json::to_vec(&public).unwrap(),
                    "all forecast quantities, ordered source rows and reasons case {case}");
            }
            MineReview::NoCommand(None) => {
                deferred += 1;
                if case == 2 { assert!(resources::ALL.into_iter().all(|c|
                    resources::automatic_tick_draw(&w,ME)[c.idx()] <= resources::stockpile(&w,ME,c))); }
            }
        }
        assert_eq!(crate::save(&w),before,"all reads preserve the world and RNG case {case}");
        assert_eq!(serde_json::to_vec(&raw_supply_forecast_with_context(&w,ME,&context)).unwrap(),
            serde_json::to_vec(&public).unwrap(),"public forecast is unchanged");
    }
    assert!(candidates > 0 && retained > 0 && deferred > 0,"exercise all ownership boundaries");
}

#[test]
fn mine_review_reuses_only_no_command_reads_and_matches_full_original_review() {
    let mut total_reused = 0;
    let mut accepted = 0;
    let mut refused = 0;
    for active in [false,true] { for closed in [false,true] { for low_pc in [false,true] {
        let (mut actual,_) = prepared(active,closed);
        if low_pc { actual.nation_mut(ME).political_capital = 0.0; }
        let mut original = actual.clone();
        let context = RawSupplyContext::new(&actual);
        let before = REUSED.with(Cell::get);
        let mut stages = Vec::new();
        let mut old_stages = Vec::new();
        review(&mut actual,ME,&context,
            &mut Some(&mut |stage,_,_| stages.push(stage.to_owned())));
        original_reads(|| review(&mut original,ME,&context,
            &mut Some(&mut |stage,_,_| old_stages.push(stage.to_owned()))));
        assert_eq!(stages,old_stages,"same observer stages and command attempts {active}/{closed}/{low_pc}");
        assert_eq!(crate::save(&actual),crate::save(&original),
            "complete world, forecast float serialization, reason, fiscal books, RNG and history {active}/{closed}/{low_pc}");
        assert_eq!(actual.headlines,original.headlines);
        let reused = REUSED.with(Cell::get)-before;
        total_reused += reused;
        if stages.iter().any(|s|s=="review.mine") && stages.iter().any(|s|s=="review.execute") {
            assert_eq!(reused,0,"a proposed mine cannot retain a pre-command forecast");
            if actual.resources.mine_projects.iter().any(|p|p.started_by==ME) { accepted += 1; }
            else { refused += 1; }
        }
        let saved = crate::save(&actual);
        evaluate(&mut actual,ME);
        assert_eq!(crate::save(&actual),saved,"same-day scheduled review is still inert");
    }}}
    assert!(total_reused>0,"exercise actual no-command forecast handoff");
    assert!(accepted>0 && refused>0,"exercise both successful and refused candidate commands");
}

#[test]
fn mine_review_never_reuses_a_forecast_across_budget_renewal() {
    let (mut actual,_) = prepared(false,false);
    actual.nation_mut(ME).program_budget.as_mut().unwrap().fiscal_year -= 1;
    let mut original = actual.clone();
    let context = RawSupplyContext::new(&actual);
    let before = REUSED.with(Cell::get);
    let mut stages = Vec::new();
    review(&mut actual,ME,&context,&mut Some(&mut |stage,_,_|stages.push(stage.to_owned())));
    original_reads(||review(&mut original,ME,&context,&mut None));
    assert_eq!(crate::save(&actual),crate::save(&original));
    assert!(stages.iter().any(|s|s=="review.execute"));
    assert!(!stages.iter().any(|s|s=="review.mine"));
    assert_eq!(REUSED.with(Cell::get),before);
}

#[test]
#[ignore="explicit immutable actual-checkpoint 31-day original mine-read parity; no timing assertions"]
fn mine_review_matches_actual_checkpoint_for_31_complete_days() {
    let path = std::env::var("SPHERES_S22_CHECKPOINT").expect("actual checkpoint path required");
    let source = std::fs::read_to_string(&path).unwrap();
    let mut value: serde_json::Value = serde_json::from_str(&source).unwrap();
    let world = if value.get("format").and_then(serde_json::Value::as_str)==Some("spheres-campaign") {
        let world=value.get_mut("world").expect("campaign simulation payload").take();
        drop(value); world
    } else { value };
    let mut actual=crate::load_value(world).unwrap();
    assert!(enabled(&actual));
    let mut original=actual.clone();
    let before=REUSED.with(Cell::get);
    let opening_month=actual.month;
    let mut crossed_month=false;
    // Native full days and no new player orders or budget renewal. This is an
    // exact behavior oracle, not the web qualification timing workload.
    for day in 0..31 {
        let a=crate::tick_day(&mut actual,&[]);
        let b=original_reads(||crate::tick_day(&mut original,&[]));
        assert_eq!(a,b,"returned headlines day {day}");
        assert_eq!(actual.headlines,original.headlines,"retained headlines day {day}");
        assert!(crate::save(&actual)==crate::save(&original),"complete native world day {day}");
        crossed_month |= actual.month != opening_month;
    }
    let reused=REUSED.with(Cell::get)-before;
    assert!(crossed_month,"include the monthly review boundary");
    assert!(reused>0,"actual checkpoint must exercise real no-command forecast reuse");
    assert_eq!(std::fs::read_to_string(path).unwrap(),source,"immutable source campaign");
    eprintln!("31 complete native days match original eager mine reads; {reused} exact no-command forecasts reused.");
}
