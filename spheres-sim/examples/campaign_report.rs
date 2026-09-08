//! Descriptive operational-war panel using the full daily simulation and normal
//! commands. This is an observer, not a historical calibration or balance bar.
//!
//! cargo run --locked --release -p spheres-sim --example campaign_report -- \
//!   --days 90 --seeds 1990,7,42 --output artifacts/campaign-report.json
//!
//! Optional: --scenarios unequal_neighbors,regional_peers. At least one opening
//! day runs before declaration, so new deployments exercise normal travel.
//! Timings are output metadata only and never decide simulated events.
use serde::Serialize;
use serde_json::{json, Value};
use spheres_sim::{
    apply_command, campaign,
    campaign::{AirMission, Approach, NavalMission, OperationOrder},
    campaign_peace::{Offer, PeaceOrder, PeaceRecord, Terms, WarAim},
    campaign_supply, clock,
    init::world_1990,
    province_economy, resources, starting_industry, tick_day,
    world::{Conflict, GameRules, NationId as N, Objective, WorldState},
    Command,
};
use std::{
    collections::{BTreeMap, BTreeSet},
    path::PathBuf,
    time::Instant,
};

// Numerical roundoff allowances, not gameplay/balance tolerances. An actual
// excess is reported and fails the run; it is never reconciled by this observer.
const FORCE_EPS: f64 = 1e-7;
const DIRECTION_EPS: f64 = 1e-8;
const SCENARIOS: [(&str, N, &str); 2] = [
    (
        "unequal_neighbors",
        N::Kuwait,
        "Iraq attacks Kuwait; actual 1990 forces and normal coalition reactions.",
    ),
    (
        "regional_peers",
        N::Iran,
        "Iraq attacks Iran; actual 1990 forces, geography and regional opposition.",
    ),
];

#[derive(Default, Serialize)]
struct ParticipantTotals {
    observed_days: u32,
    fielded_days: u32,
    fielded_force_days: f64,
    in_transit_force_days: f64,
    peak_fielded: f64,
    peak_in_transit: f64,
    underfilled_supply_days: u32,
    supply_coverage_sum_on_fielded_days: f64,
    readiness_min_when_fielded: Option<f64>,
    readiness_sum_on_fielded_days: f64,
    reported_losses: f64,
    report_days: u32,
}

#[derive(Serialize)]
struct ParticipantDay {
    nation: N,
    alive: bool,
    national_force: f64,
    campaign_accounted_force: f64,
    fielded: f64,
    in_transit: f64,
    readiness: Option<f64>,
    supply_coverage: Option<f64>,
    supply_status: String,
    losses_reported_today: Option<f64>,
}

#[derive(Serialize)]
struct Day {
    elapsed_day: u32,
    settled_date: String,
    conflict_active_at_start: bool,
    conflict_active_at_end: bool,
    control: Option<f64>,
    participants: Vec<ParticipantDay>,
}

#[derive(Default)]
struct Observer {
    tracked: Option<TrackedConflict>,
    known_participants: BTreeSet<N>,
    participants: BTreeMap<N, ParticipantTotals>,
    seen_reports: BTreeSet<(i32, N)>,
    observed_offer_ids: BTreeSet<u64>,
    recorded_offer_ids: BTreeSet<u64>,
    peace_history: Vec<PeaceRecord>,
    rows: Vec<Day>,
    active_days: u32,
    first_closed_date: Option<String>,
    previous_control: Option<f64>,
    control_min: Option<f64>,
    control_max: Option<f64>,
    control_direction: i8,
    control_reversals: u32,
    advancing_days: u32,
    retreating_days: u32,
    max_national_mass_excess: f64,
}

#[derive(Serialize)]
struct TrackedConflict {
    id: u32,
    theatre: spheres_sim::theatre::TheatreId,
    origin_attacker: N,
    initial_defender: N,
    start_year: i32,
    start_month: u32,
    observed_start_day: i32,
}

impl TrackedConflict {
    fn matches(&self, conflict: &Conflict) -> bool {
        self.id == conflict.id
            && self.theatre == conflict.theatre
            && self.origin_attacker == conflict.origin_attacker
            && self.start_year == conflict.start_year
            && self.start_month == conflict.start_month
    }

    fn matches_offer(&self, offer: &Offer) -> bool {
        offer.conflict == self.id
            && offer.issued_day >= self.observed_start_day
            && offer.participants.contains(&self.origin_attacker)
            && offer.participants.contains(&self.initial_defender)
    }
}

fn date(day: i32) -> String {
    let (y, m, d) = clock::date_from_day(day);
    format!("{y:04}-{m:02}-{d:02}")
}

fn world(seed: u64) -> Result<WorldState, String> {
    let mut w = world_1990(GameRules {
        seed,
        daily_simulation: true,
        resource_market: true,
        logistics_routes: true,
        physical_logistics: true,
        military_operations: true,
        operational_warfare: 1,
        production_system: true,
        industry_rebuild: true,
        manufacturing_system: true,
        ideology_blocs: true,
        ideology_takeover: false,
        ..Default::default()
    });
    w.player = Some(N::Iraq);
    starting_industry::enable_new_world(&mut w)?;
    starting_industry::enrich_new_world(&mut w)?;
    spheres_sim::government::ensure_all(&mut w);
    campaign::enroll(&mut w);
    province_economy::enable(&mut w);
    spheres_sim::companies::enable(&mut w);
    spheres_sim::population::enable(&mut w);
    resources::warm(&mut w);
    Ok(w)
}

fn execute(w: &mut WorldState, command: Command, events: &mut Vec<Value>) {
    let before = w.nation(N::Iraq).political_capital;
    let result = apply_command(w, &command);
    events.push(json!({
        "date": date(clock::absolute_day(w)), "command": command,
        "accepted": result.is_ok(), "refusal": result.err(),
        "political_capital_charged": before - w.nation(N::Iraq).political_capital,
    }));
}

fn schedule(
    w: &mut WorldState,
    enemy: N,
    day: u32,
    cid: &mut Option<u32>,
    events: &mut Vec<Value>,
) {
    if day == 1 {
        execute(
            w,
            Command::DeclareWar {
                attacker: N::Iraq,
                defender: enemy,
            },
            events,
        );
        *cid = w.conflict_between(N::Iraq, enemy).map(|c| c.id);
    }
    if ![1, 14, 28, 42, 60, 74, 75].contains(&day) {
        return;
    }
    let Some(conflict) = cid.filter(|id| w.conflict(*id).is_some()) else {
        events.push(json!({"date":date(clock::absolute_day(w)),"schedule_day":day,
            "skipped":"The observed conflict is not active; no replacement conflict or order was invented."}));
        return;
    };
    if day == 1 {
        execute(
            w,
            Command::WarDiplomacy {
                nation: N::Iraq,
                order: PeaceOrder::SetAim {
                    conflict,
                    aim: WarAim::Concession,
                },
            },
            events,
        );
    }
    if day == 60 || day == 74 {
        execute(
            w,
            Command::WarDiplomacy {
                nation: N::Iraq,
                order: PeaceOrder::Propose {
                    conflict,
                    terms: if day == 60 {
                        Terms::Reparations { share_bp: 50 }
                    } else {
                        Terms::Ceasefire
                    },
                },
            },
            events,
        );
        return;
    }
    if day == 75 {
        execute(
            w,
            Command::SetObjective {
                conflict,
                nation: N::Iraq,
                objective: Objective::Withdraw,
            },
            events,
        );
    }
    let (approach, reserve_bp, air) = match day {
        14 => (Approach::Breakthrough, 500, AirMission::GroundSupport),
        28 => (Approach::Hold, 3500, AirMission::Interception),
        42 => (Approach::Advance, 1500, AirMission::GroundSupport),
        75 => (Approach::FightingWithdrawal, 2500, AirMission::None),
        _ => (Approach::Advance, 1500, AirMission::Reconnaissance),
    };
    execute(
        w,
        Command::SetOperation {
            order: OperationOrder {
                conflict,
                nation: N::Iraq,
                target: None,
                approach,
                reserve_bp,
                air,
                naval: NavalMission::None,
            },
        },
        events,
    );
}

impl Observer {
    fn active_conflict<'a>(&self, w: &'a WorldState) -> Option<&'a Conflict> {
        if self.first_closed_date.is_some() {
            return None;
        }
        let tracked = self.tracked.as_ref()?;
        w.conflict(tracked.id).filter(|c| tracked.matches(c))
    }

    fn capture_peace(&mut self, w: &WorldState, discover: bool) {
        let Some(tracked) = &self.tracked else { return };
        if discover {
            self.observed_offer_ids.extend(
                w.campaign_peace
                    .offers
                    .iter()
                    .filter(|offer| tracked.matches_offer(offer))
                    .map(|offer| offer.id),
            );
        }
        // The world's history is a rolling window. Keep each original offer's
        // outcome once, including delayed invalidation of an already seen offer.
        // After closure, a reused conflict ID cannot admit new offer IDs.
        for record in &w.campaign_peace.history {
            if (self.observed_offer_ids.contains(&record.offer.id)
                || discover && tracked.matches_offer(&record.offer))
                && self.recorded_offer_ids.insert(record.offer.id)
            {
                self.observed_offer_ids.insert(record.offer.id);
                self.peace_history.push(record.clone());
            }
        }
    }

    fn pending_proposals(&self, w: &WorldState) -> Vec<Offer> {
        let Some(conflict) = self.active_conflict(w) else {
            return vec![];
        };
        w.campaign_peace
            .offers
            .iter()
            .filter(|offer| {
                offer.conflict == conflict.id && self.observed_offer_ids.contains(&offer.id)
            })
            .cloned()
            .collect()
    }

    fn before(&mut self, w: &WorldState, cid: Option<u32>) -> bool {
        if self.tracked.is_none() {
            self.tracked = cid.and_then(|id| w.conflict(id)).map(|c| TrackedConflict {
                id: c.id,
                theatre: c.theatre,
                origin_attacker: c.origin_attacker,
                initial_defender: c.defender(),
                start_year: c.start_year,
                start_month: c.start_month,
                observed_start_day: clock::absolute_day(w),
            });
        }
        let active = self.active_conflict(w);
        self.capture_peace(w, active.is_some());
        if self.tracked.is_some() && active.is_none() && self.first_closed_date.is_none() {
            self.first_closed_date = Some(date(clock::absolute_day(w)));
        }
        if let Some(c) = active {
            self.active_days += 1;
            self.known_participants.extend(c.participants());
            // Include the initial pre-resolution control in the range/change
            // series; do not infer a missing terminal control after settlement.
            if self.previous_control.is_none() {
                self.previous_control = Some(c.control);
                self.control_min = Some(c.control);
                self.control_max = Some(c.control);
            }
        }
        active.is_some()
    }

    fn after(
        &mut self,
        w: &WorldState,
        cid: Option<u32>,
        elapsed_day: u32,
        settled_day: i32,
        was_active: bool,
    ) {
        let conflict = self.active_conflict(w);
        self.capture_peace(w, was_active || conflict.is_some());
        if let Some(c) = conflict {
            self.known_participants.extend(c.participants());
            self.control_min = Some(self.control_min.unwrap_or(c.control).min(c.control));
            self.control_max = Some(self.control_max.unwrap_or(c.control).max(c.control));
            if let Some(previous) = self.previous_control {
                let delta = c.control - previous;
                let direction = if delta > DIRECTION_EPS {
                    1
                } else if delta < -DIRECTION_EPS {
                    -1
                } else {
                    0
                };
                if direction != 0 {
                    if self.control_direction != 0 && direction != self.control_direction {
                        self.control_reversals += 1;
                    }
                    self.control_direction = direction;
                    if direction > 0 {
                        self.advancing_days += 1;
                    } else {
                        self.retreating_days += 1;
                    }
                }
            }
            self.previous_control = Some(c.control);
        } else if was_active && self.first_closed_date.is_none() {
            self.first_closed_date = Some(date(settled_day));
        }
        let mut rows = Vec::new();
        for &id in self
            .known_participants
            .iter()
            .filter(|_| was_active || conflict.is_some())
        {
            let n = w.nation(id);
            let sectors: Vec<_> = w
                .campaign
                .sectors
                .values()
                .filter(|s| conflict.is_some() && Some(s.conflict) == cid && s.nation == id)
                .collect();
            let fielded: f64 = sectors.iter().map(|s| s.strength).sum();
            let in_transit: f64 = w
                .campaign
                .transfers
                .iter()
                .filter(|t| {
                    conflict.is_some() && t.conflict == cid && cid.is_some() && t.nation == id
                })
                .map(|t| t.strength)
                .sum();
            let readiness = (fielded > FORCE_EPS)
                .then(|| sectors.iter().map(|s| s.strength * s.cohesion).sum::<f64>() / fielded);
            let supply = conflict.map(|conflict| campaign_supply::view(w, conflict.id, id));
            let coverage = readiness.and_then(|_| supply.as_ref().map(|s| s.coverage));
            // Reports survive the closing day and are pruned by the next
            // prepare. Date + nation dedup prevents counting a retained report.
            let report = cid
                .and_then(|conflict| w.campaign.reports.get(&campaign::order_key(conflict, id)))
                .filter(|report| {
                    report.day == settled_day
                        && was_active
                        && (conflict.is_some() || cid.and_then(|id| w.conflict(id)).is_none())
                });
            let losses = report
                .filter(|_| self.seen_reports.insert((settled_day, id)))
                .map(|r| r.losses);
            let totals = self.participants.entry(id).or_default();
            totals.observed_days += 1;
            totals.fielded_force_days += fielded;
            totals.in_transit_force_days += in_transit;
            totals.peak_fielded = totals.peak_fielded.max(fielded);
            totals.peak_in_transit = totals.peak_in_transit.max(in_transit);
            if let Some(readiness) = readiness {
                totals.fielded_days += 1;
                totals.readiness_sum_on_fielded_days += readiness;
                totals.readiness_min_when_fielded = Some(
                    totals
                        .readiness_min_when_fielded
                        .unwrap_or(readiness)
                        .min(readiness),
                );
            }
            if let Some(coverage) = coverage {
                totals.supply_coverage_sum_on_fielded_days += coverage;
                if coverage < 1.0 - 1e-9 {
                    totals.underfilled_supply_days += 1;
                }
            }
            if let Some(losses) = losses {
                totals.reported_losses += losses;
                totals.report_days += 1;
            }
            rows.push(ParticipantDay {
                nation: id,
                alive: n.alive,
                national_force: n.mil_strength,
                campaign_accounted_force: campaign::accounted_force(w, id),
                fielded,
                in_transit,
                readiness,
                supply_coverage: coverage,
                supply_status: supply.map_or("Not active".into(), |s| s.status),
                losses_reported_today: losses,
            });
        }
        self.rows.push(Day {
            elapsed_day,
            settled_date: date(settled_day),
            conflict_active_at_start: was_active,
            conflict_active_at_end: conflict.is_some(),
            control: conflict.map(|c| c.control),
            participants: rows,
        });
    }
}

fn invariants(w: &WorldState, observer: &mut Observer) -> Vec<String> {
    let mut errors = Vec::new();
    for n in &w.nations {
        if ![
            n.gdp,
            n.population,
            n.mil_strength,
            n.munitions,
            n.inflation,
            n.debt_gdp,
            n.stability,
            n.political_capital,
        ]
        .iter()
        .all(|v| v.is_finite())
        {
            errors.push(format!("{:?}: non-finite national state", n.id));
        }
        if n.mil_strength < -FORCE_EPS {
            errors.push(format!(
                "{:?}: negative national force {}",
                n.id, n.mil_strength
            ));
        }
        if n.alive {
            let mass = campaign::accounted_force(w, n.id);
            if !mass.is_finite() || mass < -FORCE_EPS {
                errors.push(format!("{:?}: invalid campaign force {mass}", n.id));
            }
            let excess = mass - n.mil_strength;
            observer.max_national_mass_excess = observer.max_national_mass_excess.max(excess);
            if excess > FORCE_EPS {
                errors.push(format!(
                    "{:?}: campaign force {mass} exceeds national force {} by {excess}",
                    n.id, n.mil_strength
                ));
            }
        }
    }
    for (key, s) in &w.campaign.sectors {
        if !s.strength.is_finite()
            || s.strength < -FORCE_EPS
            || !s.cohesion.is_finite()
            || !(0.0..=1.0).contains(&s.cohesion)
            || !s.preparation.is_finite()
            || !(0.0..=1.0).contains(&s.preparation)
        {
            errors.push(format!("Invalid sector {key}"));
        }
    }
    for (id, r) in &w.campaign.reserves {
        if !r.strength.is_finite()
            || r.strength < -FORCE_EPS
            || !r.cohesion.is_finite()
            || !(0.0..=1.0).contains(&r.cohesion)
        {
            errors.push(format!("Invalid reserve {id:?}"));
        }
    }
    for t in &w.campaign.transfers {
        if !t.strength.is_finite()
            || t.strength < -FORCE_EPS
            || !t.cohesion.is_finite()
            || !(0.0..=1.0).contains(&t.cohesion)
        {
            errors.push(format!("Invalid transfer for {:?}", t.nation));
        }
    }
    for c in &w.conflicts {
        if !c.control.is_finite()
            || !(-1.0..=1.0).contains(&c.control)
            || c.front.values().any(|v| !v.is_finite())
        {
            errors.push(format!("Invalid conflict control {}", c.id));
        }
    }
    for (key, r) in &w.campaign.reports {
        if [r.losses, r.advanced, r.retreated]
            .iter()
            .any(|v| !v.is_finite() || *v < -FORCE_EPS)
            || !r.readiness.is_finite()
            || !r.supply.is_finite()
        {
            errors.push(format!("Invalid field receipt {key}"));
        }
    }
    for amount in w
        .campaign_supply
        .sources
        .values()
        .map(|s| s.service)
        .chain(w.campaign_supply.buffers.values().map(|b| b.service))
        .chain(w.campaign_supply.cargo.iter().map(|c| c.service))
    {
        if !amount.is_finite() || amount < -FORCE_EPS {
            errors.push(format!("Invalid sustainment service {amount}"));
        }
    }
    errors
}

fn run(name: &str, enemy: N, description: &str, seed: u64, days: u32) -> Result<Value, String> {
    let started = Instant::now();
    let mut w = world(seed)?;
    let setup_seconds = started.elapsed().as_secs_f64();
    let opening = [N::Iraq,enemy].map(|id|json!({"nation":id,"national_force":w.nation(id).mil_strength,
        "gdp_bn":w.nation(id).gdp,"population_m":w.nation(id).population,"political_capital":w.nation(id).political_capital}));
    let mut cid = None;
    let mut observer = Observer::default();
    let mut commands = Vec::new();
    let mut headlines = Vec::new();
    let mut failures = Vec::new();
    let (mut simulation_seconds, mut observer_seconds, mut command_seconds) = (0.0, 0.0, 0.0);
    eprintln!("START {name} seed={seed} days={days}");
    for elapsed in 0..days {
        let start = Instant::now();
        schedule(&mut w, enemy, elapsed, &mut cid, &mut commands);
        command_seconds += start.elapsed().as_secs_f64();
        let settled_day = clock::absolute_day(&w);
        let start = Instant::now();
        let active = observer.before(&w, cid);
        observer_seconds += start.elapsed().as_secs_f64();
        let start = Instant::now();
        let news = tick_day(&mut w, &[]);
        simulation_seconds += start.elapsed().as_secs_f64();
        let start = Instant::now();
        observer.after(&w, cid, elapsed, settled_day, active);
        failures = invariants(&w, &mut observer);
        for text in news {
            if text.contains("peace")
                || text.contains("capitulat")
                || text.contains("withdraw")
                || text.contains("annex")
            {
                headlines.push(json!({"date":date(settled_day),"text":text}));
            }
        }
        observer_seconds += start.elapsed().as_secs_f64();
        if !failures.is_empty() {
            break;
        }
        if (elapsed + 1) % 30 == 0 {
            eprintln!(
                "DAY {name} seed={seed} elapsed={} active={}",
                elapsed + 1,
                observer.active_conflict(&w).is_some()
            );
        }
    }
    let participant_summaries: Vec<_> = observer.participants.iter().map(|(nation,totals)|json!({
        "nation":nation,"totals":totals,
        "mean_readiness_when_fielded":(totals.fielded_days>0).then(||totals.readiness_sum_on_fielded_days/totals.fielded_days as f64),
        "mean_supply_coverage_when_fielded":(totals.fielded_days>0).then(||totals.supply_coverage_sum_on_fielded_days/totals.fielded_days as f64),
    })).collect();
    let pending = observer.pending_proposals(&w);
    let hash = format!("{:016x}", spheres_sim::state_hash(&w));
    let result = json!({"scenario":name,"description":description,"seed":seed,"rules":w.rules,"opening":opening,
        "conflict":cid,"conflict_identity":observer.tracked,"requested_days":days,"observed_days":observer.rows.len(),"active_days":observer.active_days,
        "first_closed_date":observer.first_closed_date,"active_at_end":observer.active_conflict(&w).is_some(),
        "last_observed_control":observer.previous_control,"control_min":observer.control_min,"control_max":observer.control_max,
        "control_direction_changes":observer.control_reversals,"advancing_days":observer.advancing_days,"retreating_days":observer.retreating_days,
        "participants":participant_summaries,"peace_history":observer.peace_history,"pending_proposals":pending,"relevant_world_headlines":headlines,
        "command_attempts":commands,"daily":observer.rows,"final_world_hash":hash,
        "invariants":{"passed":failures.is_empty(),"violations":failures,"maximum_national_mass_excess":observer.max_national_mass_excess,"force_roundoff_tolerance":FORCE_EPS},
        "performance":{"setup_seconds":setup_seconds,"command_seconds":command_seconds,"simulation_seconds":simulation_seconds,
            "observer_seconds":observer_seconds,"total_wall_seconds":started.elapsed().as_secs_f64()}});
    eprintln!("END {name} seed={seed} observed={} active={} simulation={simulation_seconds:.3}s invariants={}",observer.rows.len(),observer.active_days,result["invariants"]["passed"]);
    Ok(result)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (mut days, mut seeds, mut output) = (
        90u32,
        vec![1990u64, 7, 42],
        PathBuf::from("campaign-report.json"),
    );
    let mut selected: Option<Vec<String>> = None;
    let mut args = std::env::args().skip(1);
    while let Some(arg) = args.next() {
        if arg == "--help" {
            println!("campaign_report --days 90 --seeds 1990,7,42 --output campaign-report.json [--scenarios unequal_neighbors,regional_peers]\nFull daily world; sourced initial forces; normal paid orders; descriptive JSON and universal invariants only. Timings do not affect the world.");
            return Ok(());
        }
        let value = args.next().ok_or("Every option needs a value")?;
        match arg.as_str() {
            "--days" => days = value.parse()?,
            "--seeds" => {
                seeds = value
                    .split(',')
                    .map(str::parse)
                    .collect::<Result<Vec<_>, _>>()?
            }
            "--output" => output = PathBuf::from(value),
            "--scenarios" => selected = Some(value.split(',').map(String::from).collect()),
            _ => return Err(format!("Unknown option {arg}").into()),
        }
    }
    if days == 0
        || days > 3650
        || seeds.is_empty()
        || seeds.iter().collect::<BTreeSet<_>>().len() != seeds.len()
    {
        return Err("Use 1–3650 days and distinct seeds; at least one seed is required.".into());
    }
    if let Some(names) = &selected {
        if names.is_empty()
            || names
                .iter()
                .any(|name| !SCENARIOS.iter().any(|scenario| scenario.0 == name))
        {
            return Err("Unknown scenario; use unequal_neighbors and/or regional_peers.".into());
        }
    }
    let mut runs = Vec::new();
    for (name, enemy, description) in SCENARIOS {
        if selected
            .as_ref()
            .is_some_and(|names| !names.iter().any(|item| item == name))
        {
            continue;
        }
        for &seed in &seeds {
            runs.push(run(name, enemy, description, seed, days)?);
        }
    }
    let passed = runs.iter().all(|run| run["invariants"]["passed"] == true);
    let report = json!({"instrument":"campaign_report_v2","days":days,"seeds":seeds,
        "scope":"Descriptive full-world daily runs, not a calibrated historical frequency, difficulty score or proof of fun. No statistical success thresholds are added.",
        "initialization":"Browser-equivalent fresh daily play, inherited industry, industry rebuild, company operators, population and household labor, political lens, province economy and physical logistics. Political takeover stays off. Sourced forces are unchanged. AI uses normal incentives and prices. The first date settles peacefully before the day-1 declaration.",
        "schedule":"Elapsed days: 1 declare war, concession aim, advance/15% reserve/recon; 14 breakthrough/5%/ground support; 28 hold/35%/interception; 42 advance/15%/ground support; 60 offer 0.5% GDP reparations; 74 offer ceasefire; 75 strategic withdrawal and fighting withdrawal/25%. Commands use actual validation and costs. Closed conflicts skip later orders; refusals remain recorded.",
        "metric_definitions":{
            "active_days":"Days on which the original conflict exists after scheduled commands and before tick_day. Identity records its ID, origin, theatre and start; the first observed closure permanently ends tracking. Reused IDs and later wars do not reopen it. Full-world ticking/invariants continue for the requested horizon.",
            "control":"Actual conflict control at each post-tick boundary, side A = Iraq. Missing after closure; terminal control is not invented.",
            "control_direction_changes":"Sign changes between consecutive nonzero observed daily control deltas; absolute deltas <=1e-8 count as roundoff, not reversals. This is not a count of encirclements or battles won.",
            "force_days":"Sum of post-tick fielded or in-transit abstract force points while the original conflict remains active, by its observed belligerents. The closing day can retain a final loss receipt; later daily participant rows are empty. This is neither troop count nor national net force loss.",
            "readiness":"Force-weighted sector cohesion. Means/minima include only dates with fielded force >1e-7; no-force dates are unknown rather than zero-readiness samples.",
            "underfilled_supply_days":"Post-tick dates with fielded force and supplied coverage <1-1e-9. Covers preparing/interrupted/short service; it is not a claim that all such days represent encirclement.",
            "reported_losses":"Sum of saved BattleReport.losses only when its day matches the settled date, once per participant/date. Includes closing-day receipts retained until next prepare. No inference from net force, equipment value or GDP.",
            "peace_history":"Original-conflict offer outcomes accumulated once per offer during observation, preserving records evicted from the world's rolling history. Already seen offers may finish after closure; a reused conflict ID cannot admit another war's offers. Pending proposals are included only while the original conflict is active. Empty history is not inferred victory; separate headlines report world events.",
            "invariants":"Finite national military/economic state, nonnegative physical force/service, bounded cohesion/control, finite nonnegative reported loss, and living nations' campaign pools <= national force+1e-7. Successors may have unassigned force until next preparation, so equality is not required.",
            "performance":"Instant durations measure setup, command application, tick_day and observer work separately. Total wall time also includes hashing/serialization preparation; timing is machine-dependent and never fed into simulation."
        },"invariants_passed":passed,"runs":runs});
    if let Some(parent) = output.parent().filter(|p| !p.as_os_str().is_empty()) {
        std::fs::create_dir_all(parent)?;
    }
    std::fs::write(&output, serde_json::to_string_pretty(&report)?)?;
    eprintln!("WROTE {}", output.display());
    if !passed {
        return Err("Operational invariant violation; inspect the written diagnostic JSON.".into());
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn a_closed_conflict_id_cannot_reopen_the_observed_war() {
        let mut w = world_1990(Default::default());
        spheres_sim::war::declare_war(&mut w, N::Iraq, N::Kuwait).unwrap();
        let cid = w.conflict_between(N::Iraq, N::Kuwait).map(|c| c.id);
        let mut observer = Observer::default();
        let active = observer.before(&w, cid);
        observer.after(&w, cid, 0, 0, active);
        let participants = observer.known_participants.clone();
        w.conflicts.clear();
        let active = observer.before(&w, cid);
        observer.after(&w, cid, 1, 1, active);
        assert!(
            observer.first_closed_date.is_some(),
            "closure before a daily tick was missed"
        );
        spheres_sim::war::declare_war(&mut w, N::China, N::India).unwrap();
        assert_eq!(w.conflict_between(N::China, N::India).map(|c| c.id), cid);
        let active = observer.before(&w, cid);
        observer.after(&w, cid, 2, 2, active);
        assert_eq!(
            observer.active_days, 1,
            "reused ID added another war's active days"
        );
        assert_eq!(observer.known_participants, participants);
        assert!(!observer.rows.last().unwrap().conflict_active_at_end);
        assert!(observer.rows.last().unwrap().control.is_none());
        assert!(observer.rows.last().unwrap().participants.is_empty());
    }

    #[test]
    fn replacement_at_the_resolution_boundary_is_not_the_original_conflict() {
        let mut w = world_1990(Default::default());
        spheres_sim::war::declare_war(&mut w, N::Iraq, N::Kuwait).unwrap();
        let cid = w.conflict_between(N::Iraq, N::Kuwait).map(|c| c.id);
        let mut observer = Observer::default();
        let active = observer.before(&w, cid);
        let participants = observer.known_participants.clone();
        w.conflicts.clear();
        spheres_sim::war::declare_war(&mut w, N::China, N::India).unwrap();
        assert_eq!(w.conflict_between(N::China, N::India).map(|c| c.id), cid);
        observer.after(&w, cid, 0, 0, active);
        assert_eq!(observer.first_closed_date, Some(date(0)));
        assert_eq!(observer.known_participants, participants);
        assert!(!observer.rows.last().unwrap().conflict_active_at_end);
        assert!(observer.rows.last().unwrap().control.is_none());
        assert!(observer.active_conflict(&w).is_none());
        assert!(!observer.before(&w, cid));
        assert_eq!(observer.active_days, 1);
    }

    #[test]
    fn original_peace_history_survives_eviction_and_excludes_reused_ids() {
        let mut w = world_1990(Default::default());
        spheres_sim::war::declare_war(&mut w, N::Iraq, N::Kuwait).unwrap();
        let cid = w.conflict_between(N::Iraq, N::Kuwait).unwrap().id;
        let offer = |id, participants: Vec<N>| Offer {
            id,
            conflict: cid,
            from: participants[0],
            loser: participants[1],
            terms: Terms::Ceasefire,
            approved: vec![participants[0]],
            side_a: vec![participants[0]],
            participants,
            issued_day: 0,
            expires_day: 21,
        };
        let first = offer(7, vec![N::Iraq, N::Kuwait]);
        let second = offer(8, vec![N::Iraq, N::Kuwait]);
        w.campaign_peace.offers = vec![first.clone(), second.clone()];
        let mut observer = Observer::default();
        let active = observer.before(&w, Some(cid));
        assert_eq!(observer.pending_proposals(&w).len(), 2);
        w.campaign_peace.history.push(PeaceRecord {
            offer: first,
            day: 0,
            outcome: "accepted".into(),
        });
        w.campaign_peace.offers.clear();
        w.conflicts.clear();
        observer.after(&w, Some(cid), 0, 0, active);
        assert_eq!(observer.peace_history.len(), 1);
        // Model eviction by later world events and another war's ID reuse.
        w.campaign_peace.history.clear();
        spheres_sim::war::declare_war(&mut w, N::China, N::India).unwrap();
        let unrelated = offer(9, vec![N::China, N::India]);
        w.campaign_peace.offers.push(unrelated.clone());
        w.campaign_peace.history = vec![
            PeaceRecord {
                offer: second,
                day: 1,
                outcome: "superseded".into(),
            },
            PeaceRecord {
                offer: unrelated,
                day: 1,
                outcome: "rejected".into(),
            },
        ];
        let saved = spheres_sim::save(&w);
        let active = observer.before(&w, Some(cid));
        observer.after(&w, Some(cid), 1, 1, active);
        assert_eq!(
            observer
                .peace_history
                .iter()
                .map(|r| r.offer.id)
                .collect::<Vec<_>>(),
            vec![7, 8]
        );
        assert_eq!(observer.peace_history[0].outcome, "accepted");
        assert!(observer.pending_proposals(&w).is_empty());
        assert_eq!(
            spheres_sim::save(&w),
            saved,
            "history observation changed the world"
        );
    }

    #[test]
    fn observer_does_not_change_the_world_or_its_rng() {
        let mut w = world(1990).unwrap();
        tick_day(&mut w, &[]);
        let mut commands = Vec::new();
        let mut cid = None;
        schedule(&mut w, N::Iran, 1, &mut cid, &mut commands);
        let settled = clock::absolute_day(&w);
        tick_day(&mut w, &[]);
        let before = spheres_sim::save(&w);
        let rng = w.rng.state;
        let mut observer = Observer::default();
        let active = observer.before(&w, cid);
        observer.after(&w, cid, 1, settled, active);
        assert!(invariants(&w, &mut observer).is_empty());
        assert_eq!(spheres_sim::save(&w), before);
        assert_eq!(w.rng.state, rng);
    }
}
