// Air command uses the same server review, confirmation and command channel as
// procurement. Native squadrons, bases and support policies own every number.
use spheres_sim::airmissions as am;
use spheres_sim::{airbases as ab, aviation as av};

fn flight_input(key: &str, label: &str, value: Value, path: Vec<&str>, kind: &str) -> Value {
    json!({"key":key,"label":label,"value":value,"path":path,"type":kind})
}
fn flight_number(key: &str, label: &str, value: u32, path: Vec<&str>, max: u32) -> Value {
    let mut input = flight_input(key, label, json!(value), path, "number");
    input["min"] = json!(0);
    input["max"] = json!(max);
    input["step"] = json!(1);
    input
}
fn flight_select(
    key: &str,
    label: &str,
    value: Value,
    path: Vec<&str>,
    options: Vec<Value>,
) -> Value {
    let mut input = flight_input(key, label, value, path, "select");
    input["options"] = json!(options);
    input
}
fn flight_squadron_order(order: av::SquadronCommand) -> Value {
    json!({"kind":"air_squadron","order":order})
}
fn flight_base_order(order: ab::AirbaseCommand) -> Value {
    json!({"kind":"air_base","order":order})
}
fn flight_mission_order(order: am::MissionCommand) -> Value {
    json!({"kind":"air_mission","order":order})
}
fn flight_mission_actions(w: &WorldState, me: NationId, squadron: u32) -> Vec<Value> {
    let fighter = w
        .nation(me)
        .aviation
        .as_ref()
        .and_then(|a| a.squadrons.iter().find(|s| s.id == squadron))
        .and_then(|s| w.nation(me).equipment.as_ref()?.revisions.get(&s.revision))
        .is_some_and(|r| eq::is_fighter_platform(&r.spec.platform));
    let conflicts: Vec<_> = w
        .conflicts
        .iter()
        .filter(|c| c.posture_of(me).is_some_and(|p| p.rung >= 6))
        .collect();
    let conflict_options:Vec<_>=conflicts.iter().map(|c|json!({"value":c.id,"label":format!("{} · conflict {}",c.theatre.name(),c.id),"enabled":true})).collect();
    let mut targets = std::collections::BTreeMap::<String, Value>::new();
    for c in &conflicts {
        let contested = spheres_sim::front::contested_set(w, c);
        let side = c.side_of(me).unwrap_or(false);
        for (district, (base_a, _)) in &contested.k {
            let control = c
                .front
                .get(district)
                .map_or(if *base_a { 1.0 } else { -1.0 }, |v| *v as f64);
            let friendly_held = (side && control >= spheres_sim::front::HELD_BAND)
                || (!side && control <= -spheres_sim::front::HELD_BAND);
            let enemy_held = (side && control <= -spheres_sim::front::HELD_BAND)
                || (!side && control >= spheres_sim::front::HELD_BAND);
            if if fighter { enemy_held } else { friendly_held } {
                continue;
            }
            if ab::district_location(district).is_none() {
                continue;
            }
            targets.insert(district.clone(),json!({"value":district,"label":spheres_sim::districts::name_of(district).unwrap_or(district),"enabled":true}));
        }
    }
    let targets: Vec<_> = targets.into_values().collect();
    let (Some(first_conflict), Some(first_target)) = (conflict_options.first(), targets.first())
    else {
        return vec![];
    };
    let kinds = if fighter {
        vec![am::MissionKind::DefendSkies]
    } else {
        vec![am::MissionKind::SupportArmy, am::MissionKind::StrikeTarget]
    };
    kinds
        .into_iter()
        .map(|kind| {
            intent(
                kind.name(),
                flight_mission_order(am::MissionCommand::Queue {
                    squadron,
                    conflict: first_conflict["value"].as_u64().unwrap() as u32,
                    kind,
                    target: first_target["value"].as_str().unwrap().into(),
                }),
                vec![
                    flight_select(
                        "conflict",
                        "Campaign conflict",
                        first_conflict["value"].clone(),
                        vec!["order", "conflict"],
                        conflict_options.clone(),
                    ),
                    flight_select(
                        "target",
                        if fighter {
                            "Defense area · friendly-held or contested province"
                        } else {
                            "Enemy-held or contested province"
                        },
                        first_target["value"].clone(),
                        vec!["order", "target"],
                        targets.clone(),
                    ),
                ],
            )
        })
        .collect()
}
fn flight_missions_board(w: &WorldState, me: NationId) -> Value {
    let mut orders = vec![];
    let mut results = vec![];
    for o in w
        .air_missions
        .as_ref()
        .into_iter()
        .flat_map(|s| s.orders.iter().rev())
        .filter(|o| o.nation == me)
    {
        let squadron = w
            .nation(me)
            .aviation
            .as_ref()
            .and_then(|s| s.squadrons.iter().find(|q| q.id == o.squadron))
            .map(|q| q.name.clone())
            .unwrap_or_else(|| format!("Squadron {}", o.squadron));
        let mut metrics = vec![
            metric("Squadron", squadron),
            metric(
                if o.kind == am::MissionKind::DefendSkies {
                    "Defense area"
                } else {
                    "Target"
                },
                spheres_sim::districts::name_of(&o.target).unwrap_or(&o.target),
            ),
            metric("Conflict", o.conflict),
            metric(
                "Launch date",
                super::settled_day_json(o.launch_day)["label"].clone(),
            ),
        ];
        let mut actions = vec![];
        if o.status == am::MissionStatus::Queued {
            actions.push(intent(
                "Cancel queued mission",
                flight_mission_order(am::MissionCommand::Cancel { mission: o.id }),
                vec![],
            ));
        }
        if let Some(r) = &o.report {
            metrics.extend([
                metric("Aircraft launched", r.aircraft),
                metric("Settled sorties", r.sorties),
                metric("Compatible stores used", ammo_quantity(r.stores_used)),
                metric("Aircraft lost", r.aircraft_lost),
                metric(
                    if o.kind == am::MissionKind::DefendSkies {
                        "Patrol contact"
                    } else {
                        "Target contact"
                    },
                    if r.contacted {
                        "Recorded"
                    } else {
                        "No contact"
                    },
                ),
            ]);
            if o.kind == am::MissionKind::DefendSkies {
                metrics.push(metric("Ground attack", "None · defensive patrol"));
            } else {
                metrics.push(metric("Applied campaign power", r.applied_power));
            }
            if let Some(d) = &r.defense {
                metrics.extend([
                    metric("Opposing missions encountered", d.opposing_missions),
                    metric(
                        if o.kind == am::MissionKind::DefendSkies {
                            "Hostile strike power prevented"
                        } else {
                            "Own strike power prevented"
                        },
                        d.prevented_power,
                    ),
                    metric(
                        "Expected own losses · fighter combat",
                        d.air_combat_expected_loss,
                    ),
                    metric(
                        "Expected own losses · ground air defense",
                        d.ground_defense_expected_loss,
                    ),
                ]);
            }
        }
        let row = json!({"id":o.id,"name":format!("{} · {}",o.kind.name(),spheres_sim::districts::name_of(&o.target).unwrap_or(&o.target)),"status":match o.status{am::MissionStatus::Queued=>"Queued",am::MissionStatus::Flown=>"Flown",am::MissionStatus::Blocked=>"Held before launch",am::MissionStatus::Cancelled=>"Cancelled"},"detail":o.report.as_ref().map(|r|r.summary.clone()).unwrap_or_else(||"Squadron reserved until launch or cancellation. Eligibility and shared stores are checked again on the launch day.".into()),"metrics":metrics,"requirements":if o.report.as_ref().is_some_and(|r|r.defense.is_some()){vec!["Fighter combat and ground air defense are separate estimates of this squadron's own losses. Aircraft lost is the single settled whole-aircraft total; the estimates are not enemy kills."]}else{vec![]},"receipt_label":o.report.as_ref().map(|r|super::settled_day_json(r.day)["label"].clone()),"actions":actions});
        if o.status == am::MissionStatus::Queued {
            orders.push(row);
        } else if results.len() < 30 {
            results.push(row);
        }
    }
    let eligible = w
        .conflicts
        .iter()
        .filter(|c| c.posture_of(me).is_some_and(|p| p.rung >= 6))
        .count();
    json!({"overview":{"title":"Campaign missions","status":format!("{} queued orders",orders.len()),"detail":"Attack squadrons can Support army or Strike target. Fighter squadrons can Defend skies over a friendly-held or contested province. Reviews check actual range, compatible stores and shared campaign allocation. Orders launch on the next simulation day; the latest 30 dated results appear below.","metrics":[metric("Conflicts with air authority",eligible),metric("Queued missions",orders.len())],"warnings":if eligible==0{vec!["Authorize at least Air raid in an active conflict before ordering a patrol or strike. Support army also needs a ground operation and army contact."]}else{vec![]}},"orders":orders,"results":results})
}
fn flight_budget(path: Vec<&str>, amount: f64) -> Value {
    let mut input = flight_input(
        "daily_budget_mn",
        "Daily construction cap",
        json!(amount),
        path,
        "number",
    );
    input["min"] = json!(0.001);
    input["max"] = json!(1000);
    input["step"] = json!(0.1);
    input["unit"] = json!("$m / day");
    input
}
fn flight_base_name(w: &WorldState, id: &str) -> String {
    ab::base(w, id)
        .map(|b| b.name.clone())
        .unwrap_or_else(|| spheres_sim::districts::name_of(id).unwrap_or(id).into())
}
fn flight_revision_name(n: &Nation, id: &str) -> String {
    n.equipment
        .as_ref()
        .and_then(|s| s.revisions.get(id))
        .map(|r| r.name.clone())
        .unwrap_or_else(|| id.into())
}
fn flight_map_action(
    w: &WorldState,
    me: NationId,
    base: &ab::Airbase,
    squadron: Option<&av::Squadron>,
) -> Value {
    let location = ab::district_location(&base.district);
    let range = squadron
        .and_then(|q| w.nation(me).equipment.as_ref()?.revisions.get(&q.revision))
        .map(|r| ab::range_km(&r.spec));
    let blocker = squadron.map_or_else(
        || ab::base_blocker(w, me, &base.id),
        |q| ab::squadron_blocker(w, me, q),
    );
    nav(
        if squadron.is_some() {
            "Show base and range on map"
        } else {
            "Show airbase on map"
        },
        json!({"action":"flight_map","tab":"flight","base":base.id,"district":base.district,"name":base.name,"lon":location.map(|p|p.lon),"lat":location.map(|p|p.lat),"rangeKm":range,"capacity":base.capacity(),"occupied":ab::occupied_capacity(w,&base.id),"blocker":blocker,"transit":squadron.and_then(|q|q.transit.as_ref())}),
    )
}
fn flight_support_board(w: &WorldState, me: NationId) -> Value {
    let status = eq::air_support_status(w, me);
    let saved = w
        .nation(me)
        .equipment
        .as_ref()
        .and_then(|s| s.air_support.as_ref());
    let default_cap = saved.map_or(status.maintenance_required_bn * 1000.0 + 0.05, |p| {
        p.daily_budget_bn * 1000.0
    });
    let mut cap = budget_input(default_cap / 1000.0);
    cap["label"] = json!("Fleet upkeep + aircraft stores cap");
    let action = intent(
        "Review routine air support",
        json!({"kind":"equipment_air_support","daily_budget_mn":default_cap,"target_days":status.target_days,"automatic":saved.is_none_or(|p|p.automatic)}),
        vec![
            cap,
            json!({"key":"target_days","label":"Compatible store reserve","type":"number","value":status.target_days,"min":1,"max":365,"step":1,"unit":"planning days"}),
            json!({"key":"automatic","label":"Buy compatible finished stores within the remaining cap","type":"select","value":saved.is_none_or(|p|p.automatic),"options":[{"value":true,"label":"Automatic purchases on"},{"value":false,"label":"Automatic purchases off"}]}),
        ],
    );
    let roles:Vec<_>=status.families.iter().map(|f|json!({"label":eq::ammo_def(&f.family).map_or(f.family.as_str(),|a|a.name),
        "value":format!("{} held · {} incoming · {} still needed",ammo_quantity(f.stock),f.inbound,f.gap),
        "detail":format!("Target {} stores for {} owned aircraft; {} stores committed to public work. {}",f.target_stores,f.aircraft,f.public_committed,f.reason)})).collect();
    let receipt = saved.and_then(|p| p.receipt.as_ref());
    let mut metrics = vec![
        metric(
            if status.pending.is_some() {
                "Reviewed cap for next day"
            } else {
                "Daily combined cap"
            },
            service_money(status.daily_budget_bn),
        ),
        metric(
            "Active combined cap today",
            service_money(
                status
                    .active
                    .as_ref()
                    .filter(|p| p.automatic)
                    .map_or(0.0, |p| p.daily_budget_bn),
            ),
        ),
        metric(
            "Whole-fleet upkeep required",
            service_money(status.maintenance_required_bn),
        ),
        metric(
            "Fleet upkeep paid today",
            service_money(status.maintenance_paid_bn),
        ),
        metric(
            "Aircraft stores paid today",
            service_money(status.stores_paid_bn),
        ),
        metric(
            "Store reserve target",
            format!("{} planning days", status.target_days),
        ),
    ];
    if let Some(r) = receipt {
        metrics.push(metric(
            "Latest settled support",
            super::settled_day_json(r.day)["label"].clone(),
        ));
        metrics.push(metric(
            "Latest combined payment",
            service_money(r.maintenance_paid_bn + r.stores_paid_bn),
        ));
    }
    json!({"title":"Routine support","status":if saved.is_none(){"Set a support cap"}else if status.pending.is_some(){"Change scheduled for next day"}else if status.automatic{"Automatic stores enabled"}else{"Automatic stores off"},"detail":status.detail,
        "metrics":metrics,"roles_title":"Compatible stores","roles":roles,"warnings":[status.reason],"actions":[action,nav("Review Defense maintenance allocation",json!({"action":"budget","ministry":"defense","department":2}))]})
}

fn flight_staff_board(w: &WorldState) -> Value {
    let enabled=w.military_ai.enabled;
    json!({"title":"Other countries’ military staff", "status":if !enabled {"Paused"} else if !spheres_sim::economic_ai::enabled(w) {"Waiting for economic competition"} else {"Active"},
        "detail":spheres_sim::military_ai::NOTE,
        "metrics":[metric("Countries with recorded decisions",w.military_ai.plans.len()),metric("Acquisition and basing reviews","Every 30 days"),metric("Combat decisions","Daily · up to one strike and one defense order per country")],
        "actions":[intent(if enabled {"Review pause of military staff"} else {"Review enabling military staff"},json!({"kind":"military_ai","enabled":!enabled}),vec![])]})
}
fn flight_staff_countries(w:&WorldState)->Vec<Value> {
    w.military_ai.plans.iter().map(|(id,p)|json!({"title":id.name(),"name":id.name(),
        "status":if w.player==Some(*id) {"Player controlled"} else if !w.nation(*id).alive {"Inactive government"} else if w.military_ai.enabled && spheres_sim::economic_ai::enabled(w) {"Staff decisions"} else {"Planning paused"},
        "detail":p.operations,"metrics":[metric("Strategic review",p.last_review_day.map(|d|super::settled_day_json(d)["label"].clone())),metric("Completed reviews",p.reviews),metric("Procurement",&p.procurement),metric("Development",&p.development),metric("Support",&p.support),metric("Basing",&p.basing),metric("Last review purchase/capital commitments",company_money(p.committed_bn)),metric("Last review spending ceiling",company_money(p.purchase_limit_bn))],"actions":[]})).collect()
}
fn flight_board(w: &WorldState, me: NationId) -> Value {
    let n = w.nation(me);
    let today = spheres_sim::clock::absolute_day(w);
    let models: Vec<_> = n
        .equipment
        .as_ref()
        .into_iter()
        .flat_map(|s| s.revisions.values())
        .filter(|r| r.certified_day.is_some() && r.profile.aviation.is_some())
        .collect();
    let revision_options:Vec<_>=models.iter().map(|r|json!({"value":r.id,"label":format!("{} · {} unassigned",r.name,av::unassigned_units(n,&r.id)),"enabled":true})).collect();
    let aircraft:Vec<_>=models.iter().map(|r|{
        let held:f64=n.arsenal.held.iter().filter(|h|h.design_id.as_deref()==Some(r.id.as_str())).map(|h|h.units).sum();
        let reserved:u64=n.arsenal.held.iter().filter(|h|h.design_id.as_deref()==Some(r.id.as_str())).map(|h|h.refit_reserved as u64).sum();
        let incoming:f64=n.arsenal.orders.iter().filter(|o|o.design_id.as_deref()==Some(r.id.as_str())).map(|o|o.units).sum();
        let free=av::unassigned_units(n,&r.id);let initial=free.min(6).max(1);
        let mut name=flight_input("name","Squadron name",json!(format!("{} squadron",r.name).chars().take(80).collect::<String>()),vec!["order","Create","name"],"text");name["maxlength"]=json!(80);
        let mut quantity=flight_number("quantity","Aircraft to assign",initial,vec!["order","Create","quantity"],free.max(1));quantity["min"]=json!(1);
        let mut establish=intent("Form squadron",flight_squadron_order(av::SquadronCommand::Create{name:name["value"].as_str().unwrap().into(),revision:r.id.clone(),quantity:initial}),vec![name,quantity]);
        if free==0 {establish["enabled"]=json!(false);establish["reason"]=json!("Receive unassigned aircraft of this revision first. Other squadrons and refit reservations are unavailable.");}
        json!({"id":r.id,"name":r.name,"status":format!("{free} unassigned"),"detail":format!("Exact revision {}. Assigned aircraft remain in the national Arsenal.",r.id),
            "metrics":[metric("Delivered aircraft",held),metric("Assigned to squadrons",av::assigned_units(n,&r.id)),metric("Reserved for refit",reserved),metric("Purchased / produced deliveries pending",incoming),metric("Mission radius",format!("{:.0} km",ab::range_km(&r.spec)))],
            "actions":[establish,production_action(w,me,r)]})
    }).collect();
    let base_options:Vec<_>=w.airbases.as_ref().into_iter().flat_map(|s|&s.bases).map(|b|json!({"value":b.id,"label":format!("{} · {} free spaces",b.name,ab::free_capacity(w,&b.id)),"enabled":true,"detail":ab::base_blocker(w,me,&b.id).unwrap_or_else(||"Access available; squadron size and ferry reach are checked before confirmation.".into())})).collect();
    let squadrons:Vec<_>=n.aviation.as_ref().into_iter().flat_map(|s|&s.squadrons).map(|q|{
        let current_base=q.base.as_deref().map(|b|flight_base_name(w,b)).unwrap_or_else(||"Awaiting a base".into());
        let blocker=ab::squadron_blocker(w,me,q);
        let mut metrics=vec![metric("Assigned aircraft",q.assigned),metric("Exact revision",&q.revision),metric("Current base",current_base),metric("Service remaining",format!("{} funded days",q.service_days_left))];
        if let Some(t)=&q.transit {metrics.push(metric("Moving to",flight_base_name(w,&t.to)));metrics.push(metric("Expected arrival",super::settled_day_json(t.arrival_day)["label"].clone()));}
        let maximum=av::unassigned_units(n,&q.revision).saturating_add(q.assigned);
        let resize=intent("Change squadron size",flight_squadron_order(av::SquadronCommand::Resize{squadron:q.id,quantity:q.assigned}),vec![flight_number("quantity","Assigned aircraft",q.assigned,vec!["order","Resize","quantity"],maximum)]);
        let swap=intent("Change aircraft revision",flight_squadron_order(av::SquadronCommand::ChangeRevision{squadron:q.id,revision:q.revision.clone()}),vec![flight_select("revision","Use already delivered aircraft",json!(q.revision),vec!["order","ChangeRevision","revision"],revision_options.clone())]);
        let mut actions=flight_mission_actions(w,me,q.id);actions.extend([resize,swap]);
        if let Some(first)=base_options.first(){actions.push(intent("Move to base",flight_base_order(ab::AirbaseCommand::Rebase{squadron:q.id,base:first["value"].as_str().unwrap().into()}),vec![flight_select("base","Destination airbase",first["value"].clone(),vec!["order","base"],base_options.clone())]));}
        let mut disband=intent("Disband squadron",flight_squadron_order(av::SquadronCommand::Disband{squadron:q.id}),vec![]);disband["detail"]=json!("Return this squadron's aircraft to unassigned national holdings. This does not retire, sell or refund aircraft.");actions.push(disband);
        if let Some(base)=q.base.as_deref().or_else(||q.transit.as_ref().map(|t|t.to.as_str())){if let Some(b)=ab::base(w,base){actions.push(flight_map_action(w,me,b,Some(q)));}}
        json!({"id":q.id,"name":q.name,"status":if q.assigned==0{"Empty establishment"}else if blocker.is_some(){"Preparation needed"}else{"Ready for mission review"},"detail":format!("{} · aircraft are claimed once from national holdings.",flight_revision_name(n,&q.revision)),"metrics":metrics,"blockers":blocker.into_iter().collect::<Vec<_>>(),"actions":actions})
    }).collect();
    let bases:Vec<_>=w.airbases.as_ref().into_iter().flat_map(|s|&s.bases).map(|b|{
        let blocker=ab::base_blocker(w,me,&b.id);let owner=w.districts.get(&b.district).copied();
        let mut actions=vec![flight_map_action(w,me,b,None)];
        if b.project.as_ref().is_some_and(|p|p.sponsor==me)||ab::site_access_refusal(w,me,&b.district).is_none() {
            if let Some(p)=&b.project {
                if p.sponsor==me {
                    actions.push(intent(if p.paused{"Resume improvement"}else{"Pause improvement"},flight_base_order(ab::AirbaseCommand::PauseUpgrade{base:b.id.clone(),paused:!p.paused}),vec![]));
                    actions.push(intent("Cancel unfinished improvement",flight_base_order(ab::AirbaseCommand::CancelUpgrade{base:b.id.clone()}),vec![]));
                }
            }else{for track in [ab::UpgradeTrack::Capacity,ab::UpgradeTrack::Support,ab::UpgradeTrack::Protection]{if b.level(track)<track.max_level(){actions.push(intent(&format!("Improve {}",track.name()),flight_base_order(ab::AirbaseCommand::Upgrade{base:b.id.clone(),track,daily_budget_mn:2.0}),vec![flight_budget(vec!["order","daily_budget_mn"],2.0)]));}}}
        }
        let mut metrics=vec![metric("Host",owner.map_or("Unknown",NationId::name)),metric("Aircraft spaces",b.capacity()),metric("Stationed or inbound",ab::occupied_capacity(w,&b.id)),metric("Support level",b.support_level),metric("Protection level",b.protection_level)];
        let mut costs=vec![];
        if let Some(p)=&b.project {metrics.push(metric("Work",format!("{} → level {} · {:.1} / {} funded days",p.track.name(),p.target_level,p.progress_days,p.total_days)));costs.push(cost("Already paid",p.paid_bn,"completed work"));costs.push(cost("Contract price",p.total_cost_bn,"financial construction; no material purchase"));}
        let location=ab::district_location(&b.district);
        json!({"id":b.id,"name":b.name,"status":if b.project.as_ref().is_some_and(|p|p.paused){"Work paused"}else if b.project.is_some(){"Improvement in progress"}else if blocker.is_some(){"Access needed"}else{"Available"},
            "detail":format!("{}{}",spheres_sim::districts::name_of(&b.district).unwrap_or(&b.district),location.map(|p|format!(" · {:.2}°, {:.2}°",p.lat,p.lon)).unwrap_or_default()),
            "progress":b.project.as_ref().map(|p|p.progress_days/p.total_days as f64),"metrics":metrics,"costs":costs,"blockers":blocker.into_iter().chain(b.project.as_ref().and_then(|p|p.reason.clone())).collect::<Vec<_>>(),"actions":actions})
    }).collect();
    let district_options:Vec<_>=w.districts.iter().filter(|(d,_)|ab::site_access_refusal(w,me,d).is_none()&&ab::district_location(d).is_some()&&ab::base(w,d).is_none_or(|b|b.capacity_level==0&&b.project.is_none()))
        .map(|(d,owner)|json!({"value":d,"label":format!("{} · {}",spheres_sim::districts::name_of(d).unwrap_or(d),owner.name()),"enabled":true})).collect();
    let mut base_actions = vec![];
    if let Some(first) = district_options.first() {
        let name = format!("{} airbase", first["label"].as_str().unwrap_or("New"))
            .chars()
            .take(80)
            .collect::<String>();
        let mut name_input = flight_input(
            "name",
            "Airbase name",
            json!(name),
            vec!["order", "name"],
            "text",
        );
        name_input["maxlength"] = json!(80);
        base_actions.push(intent(
            "Establish airbase",
            flight_base_order(ab::AirbaseCommand::Establish {
                district: first["value"].as_str().unwrap().into(),
                name,
                daily_budget_mn: 2.0,
            }),
            vec![
                flight_select(
                    "district",
                    "Province",
                    first["value"].clone(),
                    vec!["order", "district"],
                    district_options.clone(),
                ),
                name_input,
                flight_budget(vec!["order", "daily_budget_mn"], 2.0),
            ],
        ));
    }
    let legacy:Vec<_>=av::legacy_summary(n).iter().map(|h|json!({"label":h.name,"value":format!("{} formation equivalents",h.formation_equivalents),"detail":format!("Exact legacy quantity · age {} months · excluded from individual aircraft assignment",h.age_months)})).collect();
    let total_assigned: u64 = n
        .aviation
        .as_ref()
        .into_iter()
        .flat_map(|s| &s.squadrons)
        .map(|q| q.assigned as u64)
        .sum();
    let ready: u64 = n
        .aviation
        .as_ref()
        .into_iter()
        .flat_map(|s| &s.squadrons)
        .filter(|q| ab::squadron_blocker(w, me, q).is_none())
        .map(|q| q.assigned as u64)
        .sum();
    json!({"overview":{"title":"Air command","status":format!("{ready} aircraft ready for mission review"),"detail":"Buy and receive aircraft, form a squadron, move it to a completed base, then fund upkeep and compatible stores. Readiness also depends on mission range and target access.",
        "metrics":[metric("Squadrons",squadrons.len()),metric("Assigned aircraft",total_assigned),metric("Unassigned aircraft",models.iter().map(|r|av::unassigned_units(n,&r.id) as u64).sum::<u64>()),metric("Dated reading",super::settled_day_json(today)["label"].clone())],"actions":[nav("Buy aircraft",json!({"action":"equipment","tab":"companies"})),nav("Prepare mission stores",json!({"action":"equipment","tab":"ammunition"}))]},
        "aircraft":aircraft,"squadrons":squadrons,"bases":bases,"base_actions":base_actions,"support":flight_support_board(w,me),"legacy":{"title":"Inherited air formations","detail":av::LEGACY_AIRCRAFT_NOTE,"roles_title":"Exact retained holdings","roles":legacy},"missions":flight_missions_board(w,me),"staff":flight_staff_board(w),"staff_countries":flight_staff_countries(w)})
}

fn flight_preview(
    w: &WorldState,
    me: NationId,
    session: &str,
    command: &Value,
    parsed: &Command,
) -> Value {
    let n = w.nation(me);
    let mut metrics = vec![];
    let mut costs = vec![];
    let mut timing = vec![];
    let mut requirements = vec![];
    let mut navigation = vec![];
    let (label, detail) = match parsed {
        Command::MilitaryAi {enabled,..} => {
            metrics.push(metric("Autonomous military staff",if *enabled {"Enabled"} else {"Paused"}));
            timing.push(json!({"label":"Effective","value":"Next simulation day; strategic reviews retain their 30-day cadence"}));
            requirements.push(spheres_sim::military_ai::NOTE.into());
            requirements.push("Pausing stops new staff decisions. Already contracted development, deliveries, standing support and queued missions retain their ordinary obligations. Independent supplier and civilian AI retain their existing policies. Economic competition must also be enabled for staff to act.".into());
            ("Confirm military staff setting","This changes other countries’ military planning. Your government remains under your control. Enabling grants no money, research, equipment or airfields.")
        },
        Command::AirSquadron { order, .. } => {
            match order {
                av::SquadronCommand::Create {
                    name,
                    revision,
                    quantity,
                } => {
                    metrics.extend([
                        metric("Squadron", name),
                        metric("Aircraft revision", flight_revision_name(n, revision)),
                        metric("Exact revision ID", revision),
                        metric("Aircraft to assign", quantity),
                        metric("Currently unassigned", av::unassigned_units(n, revision)),
                    ]);
                }
                av::SquadronCommand::Resize { squadron, quantity } => {
                    if let Some(q) = n
                        .aviation
                        .as_ref()
                        .and_then(|s| s.squadrons.iter().find(|q| q.id == *squadron))
                    {
                        metrics.extend([
                            metric("Squadron", &q.name),
                            metric("Current aircraft", q.assigned),
                            metric("New assigned aircraft", quantity),
                            metric("Currently unassigned", av::unassigned_units(n, &q.revision)),
                        ]);
                    }
                }
                av::SquadronCommand::ChangeRevision { squadron, revision } => {
                    if let Some(q) = n
                        .aviation
                        .as_ref()
                        .and_then(|s| s.squadrons.iter().find(|q| q.id == *squadron))
                    {
                        metrics.extend([
                            metric("Squadron", &q.name),
                            metric(
                                "Aircraft returned to unassigned stock",
                                format!(
                                    "{} × {}",
                                    q.assigned,
                                    flight_revision_name(n, &q.revision)
                                ),
                            ),
                            metric(
                                "Replacement aircraft assigned",
                                format!("{} × {}", q.assigned, flight_revision_name(n, revision)),
                            ),
                            metric(
                                "Replacement currently unassigned",
                                av::unassigned_units(n, revision),
                            ),
                        ]);
                    }
                }
                av::SquadronCommand::Disband { squadron } => {
                    if let Some(q) = n
                        .aviation
                        .as_ref()
                        .and_then(|s| s.squadrons.iter().find(|q| q.id == *squadron))
                    {
                        metrics.extend([
                            metric("Squadron removed", &q.name),
                            metric("Aircraft returned to unassigned holdings", q.assigned),
                        ]);
                    }
                }
            }
            requirements.push("Only delivered whole aircraft of the exact certified revision can be assigned. Other squadron claims, refit reservations, aircraft in transit and unfinished service cannot be reused.".to_string());
            ("Confirm squadron order","This changes squadron assignments immediately. Existing aircraft, age, research, money and stores stay in their current accounts. Choosing a revision exchanges already owned aircraft; it performs no refit.")
        }
        Command::AirBase { order, .. } => {
            requirements.push(ab::FOREIGN_SPONSORSHIP_NOTE.into());
            let quote = match order {
                ab::AirbaseCommand::Establish {
                    district,
                    name,
                    daily_budget_mn,
                } => {
                    metrics.extend([
                        metric("Airbase", name),
                        metric(
                            "Province",
                            spheres_sim::districts::name_of(district).unwrap_or(district),
                        ),
                    ]);
                    Some(ab::improvement_quote(
                        None,
                        ab::UpgradeTrack::Capacity,
                        *daily_budget_mn,
                    ))
                }
                ab::AirbaseCommand::Upgrade {
                    base,
                    track,
                    daily_budget_mn,
                } => {
                    metrics.extend([
                        metric("Airbase", flight_base_name(w, base)),
                        metric("Improvement", track.name()),
                        metric(
                            "Current capacity",
                            ab::base(w, base).map_or(0, |b| b.capacity()),
                        ),
                    ]);
                    Some(ab::improvement_quote(
                        ab::base(w, base),
                        *track,
                        *daily_budget_mn,
                    ))
                }
                ab::AirbaseCommand::Rebase { squadron, base } => {
                    if let Some(q) = n
                        .aviation
                        .as_ref()
                        .and_then(|s| s.squadrons.iter().find(|q| q.id == *squadron))
                    {
                        metrics.extend([
                            metric("Squadron", &q.name),
                            metric("Aircraft moving", q.assigned),
                            metric("Destination", flight_base_name(w, base)),
                            metric("Destination free spaces", ab::free_capacity(w, base)),
                        ]);
                        if let Ok(days) = ab::travel_days(w, me, q, base) {
                            timing.push(json!({"label":"Travel time","value":format!("{days} simulation days")}));
                        }
                    }
                    requirements.push("Aircraft reserve destination capacity and cannot fly missions during transit. Arrival waits for access and sufficient completed capacity; use Move to base to redirect an interrupted transfer.".into());
                    None
                }
                ab::AirbaseCommand::PauseUpgrade { base, paused } => {
                    metrics.extend([
                        metric("Airbase", flight_base_name(w, base)),
                        metric(
                            "Construction after confirmation",
                            if *paused { "Paused" } else { "Resumed" },
                        ),
                    ]);
                    None
                }
                ab::AirbaseCommand::CancelUpgrade { base } => {
                    metrics.push(metric("Airbase", flight_base_name(w, base)));
                    requirements.push("Paid work remains a sunk cost. Cancellation grants no unfinished capacity, support or protection, and issues no refund.".into());
                    None
                }
            };
            if let Some(q) = quote {
                metrics.push(metric("Completed effect", q.effect));
                costs.extend([
                    cost(
                        "Fixed construction price",
                        q.total_cost_bn,
                        "paid only as funded work completes",
                    ),
                    cost(
                        "Daily project cap",
                        q.daily_budget_mn / 1000.0,
                        "within existing national construction funding",
                    ),
                ]);
                timing.push(
                    json!({"label":"Minimum funded work","value":format!("{} days",q.total_days)}),
                );
                timing.push(json!({"label":"At this daily cap","value":if q.earliest_funded_days==u32::MAX{"No funded completion".into()}else{format!("At least {} funded days",q.earliest_funded_days)}}));
                requirements.push("The existing financial construction pool pays for work. Completed capacity changes only after the full contract is paid; access and other construction commitments can delay completion.".into());
            }
            ("Confirm airbase order","Review the mapped location, capacity and funded timing. All prices, capacities and travel times are modeled game assumptions.")
        }
        Command::AirMission { order, .. } => {
            match order {
                am::MissionCommand::Queue {
                    squadron,
                    conflict,
                    kind,
                    target,
                } => {
                    let q = am::quote(w, me, *squadron, *conflict, *kind, target);
                    let defensive = *kind == am::MissionKind::DefendSkies;
                    let sq = n
                        .aviation
                        .as_ref()
                        .and_then(|s| s.squadrons.iter().find(|s| s.id == *squadron));
                    let radius = sq
                        .and_then(|s| n.equipment.as_ref()?.revisions.get(&s.revision))
                        .map(|r| ab::range_km(&r.spec));
                    if let Some(squadron) = sq {
                        if let Some(base) = squadron.base.as_deref().and_then(|b| ab::base(w, b)) {
                            let mut map = flight_map_action(w, me, base, Some(squadron));
                            let location = ab::district_location(target);
                            map["label"] = json!(if defensive {
                                "Review defense area and patrol range on map"
                            } else {
                                "Review target and range on map"
                            });
                            map["navigate"]["missionKind"] = json!(kind);
                            map["navigate"]["target"] = json!({"district":target,"name":spheres_sim::districts::name_of(target).unwrap_or(target),"lon":location.map(|p|p.lon),"lat":location.map(|p|p.lat)});
                            map["navigate"]["blocker"] = json!(q.reason);
                            navigation.push(map);
                        }
                    }
                    metrics.extend([
                        metric("Mission", kind.name()),
                        metric(
                            "Squadron",
                            sq.map(|s| s.name.clone())
                                .unwrap_or_else(|| format!("Squadron {squadron}")),
                        ),
                        metric(
                            if defensive {
                                "Defense area"
                            } else {
                                "Target province"
                            },
                            spheres_sim::districts::name_of(target).unwrap_or(target),
                        ),
                        metric("Aircraft assigned", q.aircraft),
                        metric(
                            if defensive {
                                "Installed patrol radius"
                            } else {
                                "Installed mission radius"
                            },
                            radius
                                .map(|r| format!("{r:.0} km"))
                                .unwrap_or_else(|| "Unavailable".into()),
                        ),
                        metric(
                            if defensive {
                                "Defense area distance"
                            } else {
                                "Target distance"
                            },
                            if q.distance_km > 0.0 || q.valid {
                                format!("{:.0} km", q.distance_km)
                            } else {
                                "Awaiting valid mission geography".into()
                            },
                        ),
                    ]);
                    if !q.family.is_empty() {
                        metrics.extend([
                            metric(
                                "Compatible stores",
                                eq::ammo_def(&q.family).map_or(q.family.as_str(), |a| a.name),
                            ),
                            metric(
                                "Stores required before shared allocation",
                                ammo_quantity(q.stores_required),
                            ),
                            metric(
                                "National stores currently delivered",
                                ammo_quantity(q.stores_available),
                            ),
                            metric("Sorties before competing missions", q.sorties),
                            metric(
                                "Share of national deployment",
                                format!("{:.1}%", q.force_share * 100.0),
                            ),
                        ]);
                    }
                    timing.push(json!({"label":"Planned launch","value":super::settled_day_json(q.launch_day)["label"].clone()}));
                    if defensive {
                        metrics.push(metric(
                            "Mission role",
                            "Fighter interception · no ground attack",
                        ));
                        requirements.push("Defends this province against Support army and Strike target missions. One next-day patrol can use stores and require funded service even if no hostile flight arrives. Legacy abstract air raids are outside this patrol. Fighter interception and ground air defense are reported separately; neither figure claims enemy aircraft kills.".into());
                    }
                    requirements.push(q.detail);
                }
                am::MissionCommand::Cancel { mission } => {
                    if let Some(o) = w
                        .air_missions
                        .as_ref()
                        .and_then(|s| s.orders.iter().find(|o| o.id == *mission && o.nation == me))
                    {
                        metrics.extend([
                            metric("Mission cancelled", o.kind.name()),
                            metric(
                                if o.kind == am::MissionKind::DefendSkies {
                                    "Defense area"
                                } else {
                                    "Target province"
                                },
                                spheres_sim::districts::name_of(&o.target).unwrap_or(&o.target),
                            ),
                            metric("Stores consumed by cancellation", 0),
                            metric("Aircraft lost by cancellation", 0),
                        ]);
                    }
                    requirements.push("Only a queued mission can be cancelled. Cancellation releases its squadron and records the dated outcome without ammunition use or aircraft loss.".into());
                }
            }
            ("Confirm campaign mission","This is a campaign order. Launch-day access, maintenance, competing allocations and actual compatible stores can reduce or prevent the mission. Its dated result records target contact, stores used and aircraft losses once.")
        }
        Command::Equipment {
            order:
                EquipmentOrder::AirSupport {
                    daily_budget_mn,
                    target_days,
                    automatic,
                },
            ..
        } => {
            metrics.extend([
                metric(
                    "Combined daily cap",
                    service_money(*daily_budget_mn / 1000.0),
                ),
                metric(
                    "Compatible store target",
                    format!("{target_days} planning days"),
                ),
                metric(
                    "Automatic finished-store purchases",
                    if *automatic { "On" } else { "Off" },
                ),
                metric(
                    "Whole-fleet upkeep requirement",
                    service_money(
                        eq::fleet_maintenance_requirement(n)
                            + eq::legacy_maintenance_requirement(n),
                    ),
                ),
            ]);
            timing.push(json!({"label":"Policy starts","value":"Next simulation day"}));
            requirements.push(eq::AIR_SUPPORT_DETAIL.into());
            ("Confirm routine support policy","Within your combined cap, existing whole-fleet upkeep is paid first. Remaining permission may buy compatible finished aircraft stores from eligible suppliers. Confirmation spends no money immediately.")
        }
        _ => (
            "Confirm air order",
            "Review this air operation before confirmation.",
        ),
    };
    let action = checked(w, me, label, command.clone());
    let blockers: Vec<_> = action["reason"]
        .as_str()
        .map(str::to_string)
        .into_iter()
        .collect();
    let mut actions = vec![action];
    actions.extend(navigation);
    json!({"session_id":session,"nation":me,"valid":blockers.is_empty(),"blockers":blockers,"metrics":metrics,"costs":costs,"timing":timing,"requirements":requirements,"detail":detail,"actions":actions})
}

#[cfg(test)]
mod flight_view_tests {
    use super::*;
    const ME: NationId = NationId::France;
    #[test]
    fn s17_staff_review_is_pure_player_bound_and_grants_no_assets() {
        let mut g=fixture();
        spheres_sim::operational_warfare::enable(&mut g.world).unwrap();
        let command=json!({"kind":"military_ai","enabled":true,"nation":"USA"});
        let before=spheres_sim::save(&g.world);
        let q=preview(&g.world,ME,&g.session_id,&json!({"command":command})).unwrap();
        assert_eq!(q["valid"],true,"{q}");
        assert!(q["requirements"][1].as_str().unwrap().contains("Already contracted"));
        let _=flight_staff_board(&g.world);let _=flight_staff_countries(&g.world);
        assert_eq!(spheres_sim::save(&g.world),before);
        let parsed=super::super::parse_command(&g.world,&command,ME).unwrap();
        assert_eq!(parsed,Command::MilitaryAi {nation:ME,enabled:true});
        spheres_sim::apply_command(&mut g.world,&parsed).unwrap();
        assert!(g.world.military_ai.enabled && g.world.military_ai.plans.is_empty());
        g.world.military_ai=Default::default();
        assert_eq!(spheres_sim::save(&g.world),before,"Confirmation only changes staff permission");
        assert!(super::super::parse_command(&g.world,&json!({"kind":"military_ai","enabled":"true"}),ME).is_none());
    }
    pub(super) fn fixture() -> super::super::Game {
        let mut g = ammunition_view_tests::fixture();
        let spec = eq::default_spec("air_light_attack");
        let p = eq::design_preview(&g.world, ME, &spec);
        assert!(p.valid, "{:?}", p.blockers);
        let day = spheres_sim::clock::absolute_day(&g.world);
        let n = g.world.nation_mut(ME);
        n.equipment.as_mut().unwrap().revisions.insert(
            "flight-demo".into(),
            eq::DesignRevision {
                id: "flight-demo".into(),
                name: "Lark <attack>".into(),
                specification_key: eq::specification_key(&spec),
                spec,
                profile: p.profile.unwrap(),
                created_day: day,
                certified_day: Some(day),
            },
        );
        spheres_sim::arsenal::deliver_design(n, "flight-demo", 6, 12.5).unwrap();
        g
    }
    #[test]
    fn s12_flight_board_and_typed_squadron_reviews_are_pure_and_show_exact_claims() {
        let mut g = fixture();
        let before = spheres_sim::save(&g.world);
        let board = flight_board(&g.world, ME);
        let action = &board["aircraft"][0]["actions"][0];
        assert_eq!(action["label"], "Form squadron");
        assert_eq!(
            action["inputs"][1]["path"],
            json!(["order", "Create", "quantity"])
        );
        let command = &action["command"];
        let reviewed = preview(&g.world, ME, &g.session_id, &json!({"command":command})).unwrap();
        assert_eq!(reviewed["valid"], true, "{reviewed}");
        assert_eq!(reviewed["actions"][0]["command"], *command);
        assert!(reviewed["detail"].as_str().unwrap().contains("no refit"));
        assert_eq!(spheres_sim::save(&g.world), before);
        let parsed = super::super::parse_command(&g.world, command, ME).unwrap();
        spheres_sim::apply_command(&mut g.world, &parsed).unwrap();
        let board = flight_board(&g.world, ME);
        assert_eq!(board["squadrons"][0]["metrics"][0]["value"], 6);
        assert_eq!(board["aircraft"][0]["actions"][0]["enabled"], false);
        assert!(board["squadrons"][0]["blockers"][0]
            .as_str()
            .unwrap()
            .contains("Move to base"));
        let bad = flight_squadron_order(av::SquadronCommand::Resize {
            squadron: 1,
            quantity: 7,
        });
        let saved = spheres_sim::save(&g.world);
        let q = preview(&g.world, ME, &g.session_id, &json!({"command":bad})).unwrap();
        assert_eq!(q["valid"], false);
        assert!(!q["blockers"].as_array().unwrap().is_empty());
        assert_eq!(spheres_sim::save(&g.world), saved);
    }
    #[test]
    fn s13_airbase_review_discloses_funded_completion_and_map_reads_native_range() {
        let mut g = fixture();
        let board = flight_board(&g.world, ME);
        let command = board["base_actions"][0]["command"].clone();
        let before = spheres_sim::save(&g.world);
        let q = preview(&g.world, ME, &g.session_id, &json!({"command":command})).unwrap();
        assert_eq!(q["valid"], true, "{q}");
        assert_eq!(q["costs"][0]["amount_bn"], 0.024);
        assert!(q["requirements"]
            .as_array()
            .unwrap()
            .iter()
            .any(|v| v.as_str().is_some_and(|s| s.contains("only after"))));
        assert_eq!(spheres_sim::save(&g.world), before);
        let district = command["order"]["district"].as_str().unwrap().to_string();
        let b = ab::Airbase {
            id: district.clone(),
            district: district.clone(),
            name: "Mapped base".into(),
            sponsor: ME,
            capacity_level: 1,
            support_level: 0,
            protection_level: 0,
            project: None,
            history: vec![],
        };
        g.world.airbases = Some(ab::AirbaseState {
            bases: vec![b],
            ..Default::default()
        });
        av::apply(
            &mut g.world,
            ME,
            &av::SquadronCommand::Create {
                name: "Map squadron".into(),
                revision: "flight-demo".into(),
                quantity: 3,
            },
        )
        .unwrap();
        g.world.nation_mut(ME).aviation.as_mut().unwrap().squadrons[0].base =
            Some(district.clone());
        let before = spheres_sim::save(&g.world);
        let board = flight_board(&g.world, ME);
        let action = board["squadrons"][0]["actions"]
            .as_array()
            .unwrap()
            .iter()
            .find(|a| a["navigate"]["action"] == "flight_map")
            .unwrap();
        assert_eq!(action["navigate"]["rangeKm"], 700.0);
        assert_eq!(action["navigate"]["capacity"], 12);
        assert_eq!(action["navigate"]["occupied"], 3);
        assert_eq!(action["navigate"]["tab"], "flight");
        assert_eq!(action["navigate"]["district"], district);
        assert!(action["navigate"]["lon"].is_number());
        assert!(action.get("command").is_none());
        assert_eq!(spheres_sim::save(&g.world), before);
    }
    #[test]
    fn s14_support_review_preserves_one_cap_and_records_permission_without_spending() {
        let mut g = fixture();
        let command = json!({"kind":"equipment_air_support","daily_budget_mn":0.2,"target_days":14,"automatic":true});
        let before = spheres_sim::save(&g.world);
        let q = preview(&g.world, ME, &g.session_id, &json!({"command":command})).unwrap();
        assert_eq!(q["valid"], true, "{q}");
        assert!(q["requirements"][0]
            .as_str()
            .unwrap()
            .contains("whole-fleet maintenance"));
        assert!(q["detail"]
            .as_str()
            .unwrap()
            .contains("spends no money immediately"));
        assert_eq!(spheres_sim::save(&g.world), before);
        let funds = g.world.nation(ME).treasury_bn;
        let stock = serde_json::to_value(&g.world.nation(ME).arsenal).unwrap();
        let parsed = super::super::parse_command(&g.world, &command, ME).unwrap();
        spheres_sim::apply_command(&mut g.world, &parsed).unwrap();
        let policy = g
            .world
            .nation(ME)
            .equipment
            .as_ref()
            .unwrap()
            .air_support
            .as_ref()
            .unwrap();
        assert_eq!(policy.daily_budget_bn, 0.0002);
        assert_eq!(policy.target_days, 14);
        assert!(policy.automatic);
        assert_eq!(g.world.nation(ME).treasury_bn, funds);
        assert_eq!(
            serde_json::to_value(&g.world.nation(ME).arsenal).unwrap(),
            stock
        );
        assert_eq!(
            policy.from_day,
            spheres_sim::clock::absolute_day(&g.world) + 1
        );
    }

    #[test]
    fn s15_invalid_mission_reviews_are_pure_and_reports_retain_native_settlements() {
        let mut g = fixture();
        av::apply(
            &mut g.world,
            ME,
            &av::SquadronCommand::Create {
                name: "Mission squadron".into(),
                revision: "flight-demo".into(),
                quantity: 3,
            },
        )
        .unwrap();
        let district = g
            .world
            .districts
            .iter()
            .find(|(_, owner)| **owner == ME)
            .unwrap()
            .0
            .clone();
        let command = flight_mission_order(am::MissionCommand::Queue {
            squadron: 1,
            conflict: 999,
            kind: am::MissionKind::StrikeTarget,
            target: district.clone(),
        });
        let before = spheres_sim::save(&g.world);
        let q = preview(&g.world, ME, &g.session_id, &json!({"command":command})).unwrap();
        assert_eq!(q["valid"], false);
        assert_eq!(q["actions"][0]["enabled"], false);
        assert!(q["blockers"][0].as_str().is_some_and(|s| !s.is_empty()));
        assert_eq!(spheres_sim::save(&g.world), before);
        let day = spheres_sim::clock::absolute_day(&g.world);
        // Authored completed report isolates presentation from resolver tests.
        g.world.air_missions = Some(am::AirMissionsState {
            next_id: 2,
            orders: vec![am::MissionOrder {
                id: 1,
                nation: ME,
                squadron: 1,
                conflict: 999,
                kind: am::MissionKind::StrikeTarget,
                target: district,
                issued_day: day - 1,
                launch_day: day,
                status: am::MissionStatus::Flown,
                report: Some(am::MissionReport {
                    day,
                    summary: "Exact authored settlement".into(),
                    aircraft: 3,
                    sorties: 0.375,
                    family: "air_bomb_unguided".into(),
                    stores_used: 0.75,
                    aircraft_lost: 1,
                    applied_power: 0.125,
                    contacted: true,
                    defense: None,
                }),
            }],
            ..Default::default()
        });
        let before = spheres_sim::save(&g.world);
        let board = flight_missions_board(&g.world, ME);
        assert!(board["orders"].as_array().unwrap().is_empty());
        let row = &board["results"][0];
        assert_eq!(row["detail"], "Exact authored settlement");
        assert!(row["metrics"]
            .as_array()
            .unwrap()
            .iter()
            .any(|m| m["label"] == "Aircraft lost" && m["value"] == 1));
        assert!(row["metrics"]
            .as_array()
            .unwrap()
            .iter()
            .any(|m| m["label"] == "Settled sorties" && m["value"] == 0.375));
        assert_eq!(spheres_sim::save(&g.world), before);
    }
}

#[cfg(test)]
mod flight_defense_view_tests {
    use super::*;
    const ME: NationId = NationId::France;

    fn fighter_fixture() -> super::super::Game {
        let mut g = flight_view_tests::fixture();
        g.world
            .nation_mut(ME)
            .equipment
            .as_mut()
            .unwrap()
            .learned
            .extend(
                [
                    "air_propulsion_integration",
                    "air_mission_systems",
                    "air_fighter_integration",
                ]
                .map(str::to_string),
            );
        let spec = eq::default_spec("air_fighter");
        let preview = eq::design_preview(&g.world, ME, &spec);
        assert!(preview.valid, "{:?}", preview.blockers);
        let day = spheres_sim::clock::absolute_day(&g.world);
        let n = g.world.nation_mut(ME);
        n.equipment.as_mut().unwrap().revisions.insert(
            "fighter-demo".into(),
            eq::DesignRevision {
                id: "fighter-demo".into(),
                name: "Defensive <fighter>".into(),
                specification_key: eq::specification_key(&spec),
                spec,
                profile: preview.profile.unwrap(),
                created_day: day,
                certified_day: Some(day),
            },
        );
        spheres_sim::arsenal::deliver_design(n, "fighter-demo", 4, 0.0).unwrap();
        g
    }
    fn conflict(g: &mut super::super::Game) -> (u32, String, String) {
        g.world.conflicts.clear();
        let id = spheres_sim::commitment::open_conflict(
            &mut g.world,
            ME,
            NationId::Italy,
            spheres_sim::theatre::TheatreId::WesternEurope,
        )
        .unwrap();
        for p in &mut g.world.conflict_mut(id).unwrap().posture {
            p.rung = 6;
        }
        let c = g.world.conflict(id).unwrap();
        let side = c.side_of(ME).unwrap();
        let set = spheres_sim::front::contested_set(&g.world, c);
        let district = |friendly: bool| {
            set.k
                .iter()
                .find(|(d, (a, _))| (*a == side) == friendly && ab::district_location(d).is_some())
                .unwrap()
                .0
                .clone()
        };
        (id, district(true), district(false))
    }
    fn form(g: &mut super::super::Game, revision: &str) -> u32 {
        av::apply(
            &mut g.world,
            ME,
            &av::SquadronCommand::Create {
                name: revision.into(),
                revision: revision.into(),
                quantity: 2,
            },
        )
        .unwrap();
        g.world
            .nation(ME)
            .aviation
            .as_ref()
            .unwrap()
            .squadrons
            .last()
            .unwrap()
            .id
    }
    #[test]
    fn s16_fighters_offer_only_defense_and_distinguish_friendly_contested_and_enemy_areas() {
        let mut g = fighter_fixture();
        let (id, friendly, enemy) = conflict(&mut g);
        let fighter = form(&mut g, "fighter-demo");
        let attack = form(&mut g, "flight-demo");
        let before = spheres_sim::save(&g.world);
        let defense = flight_mission_actions(&g.world, ME, fighter);
        assert_eq!(defense.len(), 1);
        assert_eq!(defense[0]["command"]["order"]["kind"], "defend_skies");
        assert!(defense[0]["inputs"][1]["label"]
            .as_str()
            .unwrap()
            .contains("friendly-held or contested"));
        let choices = defense[0]["inputs"][1]["options"].as_array().unwrap();
        assert!(choices.iter().any(|o| o["value"] == friendly));
        assert!(!choices.iter().any(|o| o["value"] == enemy));
        let attacks = flight_mission_actions(&g.world, ME, attack);
        assert_eq!(attacks.len(), 2);
        assert!(attacks
            .iter()
            .all(|a| a["command"]["order"]["kind"] != "defend_skies"));
        assert!(attacks[0]["inputs"][1]["options"]
            .as_array()
            .unwrap()
            .iter()
            .any(|o| o["value"] == enemy));
        assert_eq!(spheres_sim::save(&g.world), before);
        g.world
            .conflict_mut(id)
            .unwrap()
            .front
            .insert(enemy.clone(), 0.0);
        assert!(
            flight_mission_actions(&g.world, ME, fighter)[0]["inputs"][1]["options"]
                .as_array()
                .unwrap()
                .iter()
                .any(|o| o["value"] == enemy)
        );
    }
    #[test]
    fn s16_defense_review_is_pure_and_maps_a_patrol_area_with_no_ground_attack_claim() {
        let mut g = fighter_fixture();
        let (id, friendly, _) = conflict(&mut g);
        let fighter = form(&mut g, "fighter-demo");
        g.world.airbases = Some(ab::AirbaseState {
            bases: vec![ab::Airbase {
                id: friendly.clone(),
                district: friendly.clone(),
                name: "Defense base".into(),
                sponsor: ME,
                capacity_level: 1,
                support_level: 0,
                protection_level: 0,
                project: None,
                history: vec![],
            }],
            ..Default::default()
        });
        g.world.nation_mut(ME).aviation.as_mut().unwrap().squadrons[0].base =
            Some(friendly.clone());
        let command = flight_mission_order(am::MissionCommand::Queue {
            squadron: fighter,
            conflict: id,
            kind: am::MissionKind::DefendSkies,
            target: friendly.clone(),
        });
        let before = spheres_sim::save(&g.world);
        let q = preview(&g.world, ME, &g.session_id, &json!({"command":command})).unwrap();
        assert_eq!(q["actions"][0]["command"], command);
        assert!(q["metrics"]
            .as_array()
            .unwrap()
            .iter()
            .any(|m| m["label"] == "Defense area"
                && m["value"] == spheres_sim::districts::name_of(&friendly).unwrap()));
        assert!(q["requirements"].as_array().unwrap().iter().any(|r| r
            .as_str()
            .unwrap()
            .contains("even if no hostile flight arrives")));
        let map = q["actions"]
            .as_array()
            .unwrap()
            .iter()
            .find(|a| a["navigate"]["action"] == "flight_map")
            .unwrap();
        assert_eq!(map["navigate"]["missionKind"], "defend_skies");
        assert_eq!(map["navigate"]["target"]["district"], friendly);
        assert_eq!(map["navigate"]["rangeKm"], 900.0);
        assert_eq!(spheres_sim::save(&g.world), before);
    }
    #[test]
    fn s16_reports_separate_own_expected_interception_and_ground_defense_losses_from_actual_total()
    {
        let mut g = fighter_fixture();
        let (id, friendly, _) = conflict(&mut g);
        let day = spheres_sim::clock::absolute_day(&g.world);
        let defense = am::AirDefenseEffect {
            opposing_missions: 2,
            prevented_power: 1.25,
            air_combat_expected_loss: 0.125,
            ground_defense_expected_loss: 0.25,
        };
        let orders = [am::MissionKind::DefendSkies, am::MissionKind::StrikeTarget]
            .into_iter()
            .enumerate()
            .map(|(i, kind)| am::MissionOrder {
                id: i as u32 + 1,
                nation: ME,
                squadron: i as u32 + 1,
                conflict: id,
                kind,
                target: friendly.clone(),
                issued_day: day - 1,
                launch_day: day,
                status: am::MissionStatus::Flown,
                report: Some(am::MissionReport {
                    day,
                    summary: "Native loss settlement".into(),
                    aircraft: 4,
                    sorties: 1.0,
                    family: if kind == am::MissionKind::DefendSkies {
                        "air_missile_short_range"
                    } else {
                        "air_bomb_unguided"
                    }
                    .into(),
                    stores_used: 2.0,
                    aircraft_lost: 1,
                    applied_power: if kind == am::MissionKind::DefendSkies {
                        0.0
                    } else {
                        3.0
                    },
                    contacted: true,
                    defense: Some(defense.clone()),
                }),
            })
            .collect();
        g.world.air_missions = Some(am::AirMissionsState {
            next_id: 3,
            orders,
            ..Default::default()
        });
        let before = spheres_sim::save(&g.world);
        let board = flight_missions_board(&g.world, ME);
        for row in board["results"].as_array().unwrap() {
            let metrics = row["metrics"].as_array().unwrap();
            assert!(metrics.iter().any(
                |m| m["label"] == "Expected own losses · fighter combat" && m["value"] == 0.125
            ));
            assert!(metrics
                .iter()
                .any(|m| m["label"] == "Expected own losses · ground air defense"
                    && m["value"] == 0.25));
            assert_eq!(
                metrics
                    .iter()
                    .filter(|m| m["label"] == "Aircraft lost" && m["value"] == 1)
                    .count(),
                1
            );
            assert!(row["requirements"][0]
                .as_str()
                .unwrap()
                .contains("not enemy kills"));
        }
        assert!(board["results"][0]["metrics"]
            .as_array()
            .unwrap()
            .iter()
            .any(|m| m["label"] == "Own strike power prevented"));
        assert!(board["results"][1]["metrics"]
            .as_array()
            .unwrap()
            .iter()
            .any(|m| m["label"] == "Hostile strike power prevented"));
        assert_eq!(spheres_sim::save(&g.world), before);
    }
    #[test]
    fn s16_fighter_design_library_guidance_and_comparison_use_interception() {
        let g = fighter_fixture();
        let r = &g.world.nation(ME).equipment.as_ref().unwrap().revisions["fighter-demo"];
        let before = spheres_sim::save(&g.world);
        let metrics = profile_metrics(&r.profile);
        assert!(metrics
            .iter()
            .any(|m| m["label"] == "Supported interception effectiveness"));
        assert!(!metrics
            .iter()
            .any(|m| m["label"] == "Supported strike effectiveness"));
        assert_eq!(
            guidance_primary(&r.spec.platform, &r.profile),
            r.profile.aviation.as_ref().unwrap().intercept_factor
        );
        assert!(profile_rows(&r.profile)
            .iter()
            .any(|m| m.0 == "air_interception"));
        let board = view(&g.world, ME, &g.session_id);
        assert!(board["platforms"]
            .as_array()
            .unwrap()
            .iter()
            .any(|p| p["id"] == "air_fighter"));
        assert!(board["research"]
            .as_array()
            .unwrap()
            .iter()
            .any(|p| p["id"] == "air_fighter_integration"));
        assert!(aviation_board(&g.world, ME)["roles"]
            .as_array()
            .unwrap()
            .iter()
            .any(|r| r["detail"]
                .as_str()
                .unwrap()
                .contains("supported interception effectiveness")));
        assert_eq!(spheres_sim::save(&g.world), before);
    }
}
