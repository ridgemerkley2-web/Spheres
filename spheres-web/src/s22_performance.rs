// S22 instruments the real Game/storage path. Preparation and measurement are
// deliberately separate ignored tests; no date, stock or treasury is authored.
// Environment: SPHERES_S22_INPUT is an immutable native JSON save,
// SPHERES_S22_OUT is a new absolute report file. PREPARE optionally accepts
// SPHERES_S22_UNTIL=YYYY-MM-DD (default 2035-11-30) for resumable bounded runs.
// SPHERES_S22_RENEW_BUDGET=1 authorizes only renewing the player's existing
// allocation/shares through normal priced commands. No successful renewal is
// inferred from a submitted command. The wrapper records cryptographic hashes;
// the native report's FNV fingerprints are explicitly not SHA-256.

fn s22_date(g: &Game) -> (i32, u32, u32) {
    (g.world.year, g.world.month, g.world.day)
}

fn s22_fingerprint(bytes: &[u8]) -> String {
    let mut value = 0xcbf29ce484222325u64;
    for byte in bytes { value = (value ^ u64::from(*byte)).wrapping_mul(0x100000001b3); }
    format!("{value:016x}")
}

fn s22_facts(g: &Game) -> Value {
    let w = &g.world;
    let player = w.player.expect("S22 needs a player");
    let n = w.nation(player);
    json!({"date":w.date_str(),"calendar":s22_date(g),"absolute_day":spheres_sim::clock::absolute_day(w),
        "player":player,"alive":w.nation(player).alive,"seed":w.rules.seed,
        "rules":w.rules,"capabilities":profile_capabilities(w),"owned_work":profile_owned_work(w),
        "history_points":g.history.len(),"dispatches":g.log.len(),"history_epoch":g.history_epoch,
        "player_projects":spheres_sim::production::projects_for(w,player).count(),
        "feature_workload":{"airbases":w.airbases.as_ref().map_or(0,|a|a.bases.len()),
            "air_mission_orders":w.air_missions.as_ref().map_or(0,|a|a.orders.len()),
            "player_squadrons":n.aviation.as_ref().map_or(0,|a|a.squadrons.len()),
            "player_assigned_aircraft":n.aviation.as_ref().map_or(0,|a|a.squadrons.iter().map(|s|s.assigned as u64).sum::<u64>()),
            "player_arsenal_holdings":n.arsenal.held.len(),
            "player_equipment_revisions":n.equipment.as_ref().map_or(0,|e|e.revisions.len()),
            "government_records":w.governments.states.len(),
            "government_performance_records":w.governments.states.iter().filter(|s|s.political_record.is_some()).count(),
            "party_leadership_assignments":w.party_leadership.as_ref().map_or(0,|p|p.assignments.len()),
            "executive_bindings":w.party_leadership.as_ref().map_or(0,|p|p.executives.len()),
            "operational_reserves":w.campaign.reserves.len(),"operational_orders":w.campaign.orders.len(),
            "operational_reports":w.campaign.reports.len()},
        "shooting_wars":w.conflicts.iter().filter(|c|c.shooting()).count(),
        "world_fnv64":format!("{:016x}",spheres_sim::state_hash(w)),
        "journey":g.journey,"pause_reason":campaign_journey::pause_reason(g)})
}

fn s22_paths() -> (std::path::PathBuf, std::path::PathBuf) {
    let input = std::path::PathBuf::from(std::env::var_os("SPHERES_S22_INPUT").expect("Set SPHERES_S22_INPUT"));
    let out = std::path::PathBuf::from(std::env::var_os("SPHERES_S22_OUT").expect("Set SPHERES_S22_OUT"));
    assert!(input.is_absolute() && out.is_absolute() && input.is_file() && !out.exists() && input != out);
    std::fs::create_dir_all(out.parent().unwrap()).unwrap();
    (input, out)
}

fn s22_write_report(path: &std::path::Path, value: &Value) {
    // Only the new disposable report is replaced. Native checkpoints use the
    // game's atomic storage writer and are never overwritten by this harness.
    std::fs::write(path, serde_json::to_vec_pretty(value).unwrap()).unwrap();
}

fn s22_checkpoint(root: &std::path::Path, slot: &str, g: &Game) -> Value {
    let path = root.join("saves").join(format!("{slot}.json"));
    assert!(!path.exists(), "Refusing to replace {}", path.display());
    let receipt = storage::write(root, slot, g).expect("Checkpoint write");
    let raw = std::fs::read(&path).unwrap();
    json!({"receipt":receipt,"bytes":raw.len(),"file_fnv64":s22_fingerprint(&raw),"facts":s22_facts(g)})
}

fn s22_renewal(g: &Game, renew: bool) -> Result<Vec<Command>, String> {
    if let Some(reason) = campaign_journey::pause_reason(g) { return Err(reason); }
    let me = g.world.player.ok_or("No player government")?;
    if !g.world.nation(me).alive { return Err("Player government no longer exists".into()); }
    if !g.world.rules.daily_simulation { return Err("S22 requires the saved daily simulation rule".into()); }
    let nation = g.world.nation(me);
    if !renew { return Ok(vec![]); }
    let due = nation.program_budget.as_ref().map_or_else(
        || nation.annual_budget.as_ref().is_none_or(|p|p.fiscal_year != g.world.year),
        |p|p.fiscal_year != g.world.year);
    if !due { return Ok(vec![]); }
    let allocations = nation.budget_for(g.world.year).allocations;
    let command = if let Some(plan) = &nation.program_budget {
        Command::SetProgramBudget { nation:me, fiscal_year:g.world.year, allocations, departments:plan.departments }
    } else {
        Command::SetAnnualBudget { nation:me, fiscal_year:g.world.year, allocations }
    };
    // A preflight on an independent clone cannot grant anything to the measured
    // world. The actual command still runs inside the normal daily settlement.
    let mut trial = g.world.clone();
    apply_command(&mut trial, &command)?;
    Ok(vec![command])
}

fn s22_validate_day(g: &Game, before: i32, log_start: usize, ordered: bool) -> Result<(), String> {
    if spheres_sim::clock::absolute_day(&g.world) != before + 1 {
        return Err("Daily advance did not settle exactly one real day".into());
    }
    if ordered {
        let me = g.world.player.unwrap();
        let nation = g.world.nation(me);
        // A year-end order concerns the day being settled, not the next year.
        let settled_year = spheres_sim::clock::date_from_day(before).0;
        let enacted = nation.program_budget.as_ref().map_or_else(
            || nation.annual_budget.as_ref().map(|p|p.fiscal_year), |p|Some(p.fiscal_year));
        if enacted != Some(settled_year) || g.log[log_start..].iter().any(|e|e.text.starts_with("[rejected]")) {
            return Err("Normal daily settlement rejected the requested budget renewal".into());
        }
    }
    Ok(())
}

fn s22_target(value: &str) -> (i32, u32, u32) {
    let parts: Vec<_> = value.split('-').collect();
    assert_eq!(parts.len(),3,"Expected YYYY-MM-DD");
    let date = (parts[0].parse::<i32>().unwrap(),parts[1].parse::<u32>().unwrap(),parts[2].parse::<u32>().unwrap());
    assert!((1990..=2035).contains(&date.0) && (1..=12).contains(&date.1));
    assert!((1..=spheres_sim::world::days_in_month(date.0,date.1)).contains(&date.2));
    assert!(date <= (2035,11,30), "Reserve December 2035 for measurement");
    date
}

#[test]
#[ignore = "Long ordinary France preparation; explicit immutable input and new output required, no performance claim"]
fn s22_prepare_checkpoints() {
    use std::io::Write;
    let (input,out) = s22_paths();
    let original = std::fs::read(&input).unwrap();
    let mut g = storage::decode(std::str::from_utf8(&original).unwrap()).unwrap();
    assert_eq!(g.world.player,Some(NationId::France),"This preparation retains the earned France campaign");
    let target = s22_target(&std::env::var("SPHERES_S22_UNTIL").unwrap_or_else(|_|"2035-11-30".into()));
    assert!(s22_date(&g) <= target, "Never redate an input backwards");
    let root = out.parent().unwrap().join("campaigns");
    assert!(!root.exists(), "Use a new output directory when resuming a real checkpoint");
    let renew = std::env::var("SPHERES_S22_RENEW_BUDGET").as_deref()==Ok("1");
    let mut report = json!({"format":"spheres-s22-preparation/v1","revision":env!("SPHERES_REVISION"),
        "mode":"prepare_only","passed":false,"input":input,"input_bytes":original.len(),
        "input_fnv64":s22_fingerprint(&original),"starting":s22_facts(&g),"target":target,
        "renew_existing_budget":renew,"checkpoints":[],"control_commands":[],"ordinary_days":0,
        "source_unchanged":true,"method":"Actual daily Game advancement from an unchanged archived France save; no redating, reseeding, grants or authored outcomes. Existing allocations/shares optionally renewed through ordinary priced commands. Every January checkpoint and requested final date saved with the native envelope. Preparation has no performance claim."});
    let journal_path = out.parent().unwrap().join("preparation-journal.jsonl");
    let mut journal = std::fs::OpenOptions::new().write(true).create_new(true).open(journal_path).unwrap();
    report["checkpoints"].as_array_mut().unwrap().push(s22_checkpoint(&root,"s22-resume-input",&g));
    s22_write_report(&out,&report);
    let mut failure = None;
    while s22_date(&g) < target {
        let commands = match s22_renewal(&g,renew) { Ok(c)=>c,Err(e)=>{failure=Some(e);break;} };
        let before = spheres_sim::clock::absolute_day(&g.world);
        let log_start = g.log.len();
        let command_row = json!({"date":g.world.date_str(),"commands":commands,
            "prices_pc":commands.iter().map(|c|spheres_sim::price_of(&g.world,c)).collect::<Vec<_>>()});
        let ordered = !commands.is_empty();
        let outcome = g.advance_days(1,commands);
        if ordered { report["control_commands"].as_array_mut().unwrap().push(command_row); }
        if let Err(e) = s22_validate_day(&g,before,log_start,ordered) {failure=Some(e);break;}
        let days = report["ordinary_days"].as_u64().unwrap()+1;report["ordinary_days"]=json!(days);
        if g.world.day==1 || outcome.1.is_some() {
            writeln!(journal,"{}",json!({"facts":s22_facts(&g),"event_pause":outcome.1,
                "response":"Ordinary event acknowledged by subsequent one-day request; terminal campaign pauses stop preparation"})).unwrap();
            journal.flush().unwrap();
        }
        if g.world.month==1 && g.world.day==1 || s22_date(&g)==target {
            let slot=format!("s22-{:04}-{:02}-{:02}",g.world.year,g.world.month,g.world.day);
            report["checkpoints"].as_array_mut().unwrap().push(s22_checkpoint(&root,&slot,&g));
            report["current"]=s22_facts(&g);s22_write_report(&out,&report);
            eprintln!("S22 PREPARED {} after {days} ordinary days",g.world.date_str());
        }
    }
    if let Some(reason)=failure {
        report["failure"]=json!(reason);
        report["blocked_checkpoint"]=s22_checkpoint(&root,"s22-blocked",&g);
    } else { report["passed"]=json!(s22_date(&g)==target); }
    report["final"]=s22_facts(&g);
    report["source_unchanged"]=json!(std::fs::read(&input).unwrap()==original);
    s22_write_report(&out,&report);
    assert_eq!(report["source_unchanged"],true);
    assert_eq!(report["passed"],true,"Preparation stopped: {}",report["failure"]);
}

fn s22_read_rooms(g: &Game) -> Value {
    let player=g.world.player.unwrap();let before=spheres_sim::state_hash(&g.world);let mut rows=vec![];
    macro_rules! room {($label:literal,$value:expr)=>{{let start=Instant::now();let value=$value;
        let bytes=serde_json::to_vec(&value).unwrap().len();rows.push(json!({"room":$label,"elapsed_ms":ms(start),"bytes":bytes}));}}}
    room!("equipment",equipment_view::view(&g.world,player,&g.session_id));
    room!("industry",industry_json(&g.world,player));
    room!("construction",production_json(&g.world,player));
    room!("ministry_budget",programs_json(&g.world,player,None));
    room!("campaign",campaign_journey::view(g,"/api/campaign"));
    assert_eq!(spheres_sim::state_hash(&g.world),before,"Read-only room inspection changed world");
    json!(rows)
}

#[test]
#[ignore = "31-day S22 native timing and independent throughput; run alone on an immutable actual checkpoint"]
fn s22_measure_checkpoint() {
    let (input,out)=s22_paths();let original=std::fs::read(&input).unwrap();
    let mut g=storage::decode(std::str::from_utf8(&original).unwrap()).unwrap();
    let renew=std::env::var("SPHERES_S22_RENEW_BUDGET").as_deref()==Ok("1");
    assert!(s22_date(&g)<=(2035,11,30),"31 daily samples must fit through 31 December 2035");
    let initial=s22_facts(&g);let mut samples=vec![];let mut commands_log=vec![];let mut failure=None;
    for index in 0..31 {
        let commands=match s22_renewal(&g,renew){Ok(c)=>c,Err(e)=>{failure=Some(e);break;}};
        let orders=serde_json::to_value(&commands).unwrap();let ordered=!commands.is_empty();
        let before=spheres_sim::clock::absolute_day(&g.world);let log_start=g.log.len();
        let epoch=g.history_epoch;let after=g.history.last().expect("Native checkpoint history").t;
        let player=g.world.player.unwrap();let all=Instant::now();let start=Instant::now();
        let outcome=g.advance_days(1,commands);let simulation=ms(start);
        let start=Instant::now();let state=state_json(&g,None);let read_model=ms(start);
        let start=Instant::now();let state_bytes=serde_json::to_vec(&state).unwrap().len();let serialization=ms(start);
        let start=Instant::now();let delta=history::request(&g,&format!("/api/history?nations={}&epoch={epoch}&after={after}",player.code()));
        let delta_bytes=serde_json::to_vec(&delta).unwrap().len();let delta_ms=ms(start);let whole=ms(all);
        if let Err(e)=s22_validate_day(&g,before,log_start,ordered){failure=Some(e);break;}
        commands_log.push(json!({"index":index,"commands":orders,"event_pause":outcome.1}));
        samples.push(json!({"index":index,"date":g.world.date_str(),"simulation_history_ms":simulation,
            "state_read_model_ms":read_model,"state_serialization_ms":serialization,
            "history_delta_serialization_ms":delta_ms,"whole_turn_ms":whole,"state_bytes":state_bytes,
            "history_delta_bytes":delta_bytes,"rooms":s22_read_rooms(&g),"facts":s22_facts(&g)}));
    }
    let final_facts=s22_facts(&g);
    // A fresh independent load measures a true batch wall interval. Loading is
    // excluded; renewals, daily history and per-day validity bookkeeping are
    // included. This is native day stepping, not HTTP/browser/server throughput.
    drop(g);let mut batch=storage::decode(std::str::from_utf8(&original).unwrap()).unwrap();
    let batch_start=Instant::now();let mut settled=0;let mut batch_orders=vec![];
    if failure.is_none() { for _ in 0..31 {
        let commands=match s22_renewal(&batch,renew){Ok(c)=>c,Err(e)=>{failure=Some(e);break;}};
        let ordered=!commands.is_empty();if ordered{batch_orders.push(serde_json::to_value(&commands).unwrap());}
        let day=spheres_sim::clock::absolute_day(&batch.world);let log_start=batch.log.len();batch.advance_days(1,commands);
        if let Err(e)=s22_validate_day(&batch,day,log_start,ordered){failure=Some(e);break;}settled+=1;
    }}
    let batch_ms=ms(batch_start);let batch_final=s22_facts(&batch);
    let extract=|name:&str|samples.iter().map(|s|s[name].as_f64().unwrap()).collect::<Vec<_>>();
    let mut summaries=serde_json::Map::new();
    if !samples.is_empty(){for name in ["simulation_history_ms","state_read_model_ms","state_serialization_ms","history_delta_serialization_ms","whole_turn_ms"]{
        summaries.insert(name.into(),summary(&extract(name)));}}
    let equivalent=final_facts==batch_final;
    let passed=failure.is_none()&&samples.len()==31&&settled==31&&equivalent;
    let report=json!({"format":"spheres-s22-profile/v1","revision":env!("SPHERES_REVISION"),"mode":"measure_input",
        "passed":passed,"failure":failure,"input":input,"input_bytes":original.len(),"input_fnv64":s22_fingerprint(&original),
        "renew_existing_budget":renew,"starting":initial,"final":final_facts,"samples":samples,"summaries":summaries,
        "control_commands":commands_log,"batch":{"ordinary_days":settled,"elapsed_ms":batch_ms,
            "days_per_second":if batch_ms>0.0{settled as f64*1000.0/batch_ms}else{0.0},"control_commands":batch_orders,
            "final":batch_final,"same_final_facts":equivalent},"source_unchanged":std::fs::read(&input).unwrap()==original,
        "method":"31 sequential real daily advances. Raw stage clocks exclude loading, preparation, room reads, save I/O, network and browser. Whole turn includes simulation/history, state read/serialization and selected-history delta; its percentile is not a sum of percentiles. Rooms independently include read plus serialization after each timed turn. Batch reloads the identical input and measures 31 actual daily advances as one wall interval, including command selection/preflight and validity bookkeeping but no room/state serialization. Structural pass does not imply latency/memory acceptance; use frozen S01 limits separately."});
    s22_write_report(&out,&report);assert_eq!(report["source_unchanged"],true);assert!(passed,"Incomplete or unequal native measurements: {}",report["failure"]);
}

#[test]
fn s22_instrumentation_renews_without_grants_and_retains_native_archive() {
    let mut g=Game::new_fresh(1990,Some(NationId::France));fresh_play_rules(&mut g).unwrap();g.history.clear();g.snapshot();
    let original=save(&g.world);let orders=s22_renewal(&g,true).unwrap();assert_eq!(save(&g.world),original,"Preflight is pure");
    let before=spheres_sim::clock::absolute_day(&g.world);let start=g.log.len();let ordered=!orders.is_empty();
    let mut expected=storage::decode(&storage::encode(&g).unwrap()).unwrap();
    expected.advance_days(1,orders.clone());g.advance_days(1,orders);
    s22_validate_day(&g,before,start,ordered).unwrap();assert_eq!(save(&g.world),save(&expected.world));
    assert_eq!(g.history.len(),expected.history.len());assert_eq!(g.log.len(),expected.log.len());
    assert!(s22_renewal(&g,true).unwrap().is_empty());
    let encoded=storage::encode(&g).unwrap();let loaded=storage::decode(&encoded).unwrap();
    assert_eq!(s22_facts(&loaded),s22_facts(&g));
    g.world.nation_mut(NationId::France).alive=false;
    assert!(s22_renewal(&g,true).is_err(),"A dead player requires real continuation; no bypass");
}

#[test]
fn s22_calendar_targets_leave_a_complete_last_month_for_measurement() {
    let mid=s22_target("2015-01-01");let late=s22_target("2035-11-30");
    assert_eq!(mid,(2015,1,1));
    let final_day=spheres_sim::clock::date_from_day(spheres_sim::clock::date_day(late.0,late.1,late.2)+31);
    assert_eq!(final_day,(2035,12,31));
    assert!(std::panic::catch_unwind(||s22_target("2015-02-30")).is_err());
    assert!(std::panic::catch_unwind(||s22_target("2035-12-01")).is_err());
}
