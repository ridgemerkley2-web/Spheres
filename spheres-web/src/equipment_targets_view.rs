// Quantity planning reads the same physical stock and programme ledger as the
// simulation. Suggestions remain ordinary, individually reviewed orders.
fn target_action(w: &WorldState, me: NationId, revision: &str) -> Value {
    let plan = eq::fleet_target_plans_world(w,me)
        .into_iter()
        .find(|p| p.revision == revision);
    let quantity = plan.as_ref().map_or(0, |p| {
        p.desired
            .unwrap_or(p.projected.min(eq::MAX_FLEET_TARGET as u64) as u32)
    });
    let mut action = intent(
        if plan.as_ref().is_some_and(|p| p.desired.is_some()) {
            "Review fleet target"
        } else {
            "Set fleet target"
        },
        json!({"kind":"equipment_target","revision":revision,"quantity":quantity}),
        vec![
            json!({"key":"quantity","label":"Desired vehicles of this revision","type":"number","value":quantity,"min":0,"max":eq::MAX_FLEET_TARGET,"step":1}),
        ],
    );
    action["detail"] = json!("Save a quantity goal for this exact model. Targets place no orders and spend no money; zero means a goal of no vehicles, without retiring anything.");
    action
}

fn target_metrics(p: &eq::FleetTargetPlan) -> Vec<Value> {
    vec![
        metric(
            "Desired vehicles",
            p.desired
                .map(|v| v.to_string())
                .unwrap_or("Not tracked".into()),
        ),
        metric("Delivered now", p.delivered),
        metric("Available for operations", p.available),
        metric("Completed and in transit", p.incoming),
        metric("New manufacture remaining", p.production_remaining),
        metric("Refits into this revision", p.refit_incoming),
        metric("Refits out of this revision", p.refit_outgoing),
        metric("If all active plans finish", p.projected),
        metric("Still to order", p.shortfall),
        metric("Above target after plans", p.excess),
    ]
}

fn targets_board(w: &WorldState, me: NationId) -> Vec<Value> {
    let n = w.nation(me);
    let Some(s) = &n.equipment else {
        return vec![];
    };
    let site_options = sites(w, me);
    eq::fleet_target_plans_world(w,me).into_iter().filter(|p| p.desired.is_some()).map(|p| {
        let revision = &s.revisions[&p.revision];
        let mut actions = vec![target_action(w, me, &p.revision)];
        let mut reasons = vec![];
        if p.conditional_incoming > 0 {
            reasons.push(format!("{} future vehicles still need funded work and raw inputs. The final count is conditional, not a delivery promise.", p.conditional_incoming));
        }
        if p.stalled_incoming > 0 || p.stalled_outgoing > 0 {
            reasons.push(format!("Paused, blocked or unfunded work affects {} incoming and {} outgoing refit vehicles. Resume or resolve those programmes for the projection to hold.", p.stalled_incoming, p.stalled_outgoing));
            actions.push(nav("Review existing programmes", json!({"action":"equipment","tab":"production"})));
        }
        if p.shortfall > 0 {
            let quantity = p.shortfall.min(eq::MAX_BATCH as u64) as u32;
            let site = site_options.iter().find(|o| o["value"].as_str().is_some_and(|district| eq::production_quote(w, me, &p.revision, district, quantity, 0.0002).valid))
                .or_else(|| site_options.first()).map(|o| o["value"].clone()).unwrap_or(json!(""));
            let mut production = intent("Review production for shortfall", json!({"kind":"equipment_produce","revision":p.revision,"district":site,"quantity":quantity,"daily_budget_mn":0.2}), vec![
                json!({"key":"district","label":"Arms plant province","type":"select","value":site,"options":site_options}),
                json!({"key":"quantity","label":"Vehicles to manufacture","type":"number","value":quantity,"min":1,"max":eq::MAX_BATCH,"step":1}), budget_input(0.0002)]);
            production["detail"] = json!(format!("Your target is {} vehicles short after all active plans. This proposes {} new vehicles; review the site, spending limit and materials before placing the order. Existing plans are not changed.", p.shortfall, quantity));
            if company_supplies_revision(w,me,&p.revision) {
                actions.push(nav("Review manufacturer stock",json!({"action":"equipment","tab":"companies"})));
                reasons.push("This revision is supplied by a company. Buy its finished stock; a fleet target does not authorize the company to spend government funds.".into());
            } else {actions.push(production);}
            if site_options.is_empty() {
                reasons.push("New manufacture or refit needs a completed arms plant with an available slot.".into());
                actions.push(nav("Build an arms plant", json!({"action":"construction","kind":"arms_plant"})));
            }
            for candidate in eq::fleet_target_refits_world(w,me, &p.revision).into_iter().take(3) {
                let source = &s.revisions[&candidate.source_revision];
                if company_supplies_revision(w,me,&p.revision) {
                    if let Some(mut action)=company_refit_action(w,me,&source.id,Some(&p.revision),candidate.quantity) {
                        action["label"]=json!(format!("Review manufacturer refit from {}",source.name));
                        actions.push(action);
                    }
                    continue;
                }
                let site = site_options.iter().find(|o| o["value"].as_str().is_some_and(|district| eq::refit_quote(w, me, &source.id, &p.revision, district, candidate.quantity, 0.0001).valid))
                    .or_else(|| site_options.first()).map(|o| o["value"].clone()).unwrap_or(json!(""));
                let mut action = intent(&format!("Review refit from {}", source.name), json!({"kind":"equipment_refit","source":source.id,"target":p.revision,"district":site,"quantity":candidate.quantity,"daily_budget_mn":0.1}), vec![
                    json!({"key":"district","label":"Arms plant province","type":"select","value":site,"options":site_options}),
                    json!({"key":"quantity","label":"Source vehicles to convert","type":"number","value":candidate.quantity,"min":1,"max":candidate.quantity,"step":1}), budget_input(0.0001)]);
                action["detail"] = json!(format!("Alternative to new manufacture: convert up to {} delivered, unreserved {} vehicles into {}. This reduces the source fleet by the same count and preserves its saved target. These are alternatives, not a recommendation to order both.", candidate.quantity, source.name, revision.name));
                actions.push(action);
            }
        } else if p.excess > 0 {
            reasons.push(format!("{} vehicles would be above your goal if all plans finish. Review existing orders or delivered vehicles; the target never cancels work or retires equipment automatically.", p.excess));
            actions.push(nav("Review production and refits", json!({"action":"equipment","tab":"production"})));
        } else {
            reasons.push("Existing stock and active plans cover this target. No additional order is suggested.".into());
        }
        actions.push(intent("Stop tracking this target", json!({"kind":"equipment_target","revision":p.revision,"quantity":null}), vec![]));
        json!({"id":format!("target:{}",p.revision),"name":revision.name,
            "status":if p.shortfall>0 {"More vehicles needed"} else if p.excess>0 {"Above planned quantity"} else if p.conditional_incoming>0 || p.refit_outgoing>0 || p.incoming>0 {"Covered if plans finish"} else {"Target met"},
            "detail":format!("{} Delivered and in-transit vehicles are counted once. Refit conversions move vehicles between revisions; cancelled work is excluded. Quantity goals do not predict combat losses or create spending authority.", reasons.join(" ")),
            "metrics":target_metrics(&p),"costs":[],"actions":actions})
    }).collect()
}

fn target_preview(
    w: &WorldState,
    me: NationId,
    session: &str,
    command: &Value,
    parsed: &Command,
    revision: &str,
    quantity: Option<u32>,
) -> Value {
    let action = checked(
        w,
        me,
        if quantity.is_some() {
            "Save fleet target"
        } else {
            "Stop tracking target"
        },
        command.clone(),
    );
    let blockers: Vec<_> = action["reason"]
        .as_str()
        .map(|reason| vec![reason.to_string()])
        .unwrap_or_default();
    let plan = eq::fleet_target_plans_world(w,me)
        .into_iter()
        .find(|p| p.revision == revision);
    let mut metrics = plan.as_ref().map(target_metrics).unwrap_or_default();
    metrics.push(metric(
        "Proposed target",
        quantity
            .map(|v| v.to_string())
            .unwrap_or("Stop tracking".into()),
    ));
    if blockers.is_empty() {
        let mut proposed = w.clone();
        if spheres_sim::apply_command(&mut proposed, parsed).is_ok() {
            if let Some(next) = eq::fleet_target_plans_world(&proposed,me)
                .into_iter()
                .find(|p| p.revision == revision)
            {
                metrics.push(metric("Still to order after this change", next.shortfall));
                metrics.push(metric("Above target after this change", next.excess));
            }
        }
    }
    json!({"session_id":session,"nation":me,"valid":blockers.is_empty(),"blockers":blockers,
        "metrics":metrics,"costs":[],"timing":[],"requirements":["This saves a planning preference only. It orders, converts and retires no vehicles, spends no funds, and leaves all existing programmes unchanged. Production and refit suggestions require a separate review and confirmation."],"actions":[action],
        "detail":"A target applies to this exact certified revision. Zero records a goal of no vehicles; stopping tracking removes only the preference. Future counts depend on current plans finishing and exclude future combat losses."})
}

#[cfg(test)]
mod target_view_tests {
    use super::*;
    fn fixture() -> super::super::Game {
        let mut g = super::super::Game::new(1990, Some(NationId::USA));
        super::super::play_rules(&mut g);
        spheres_sim::programs::set_construction_budget(&mut g.world, NationId::USA, 0.0).unwrap();
        let day = spheres_sim::clock::absolute_day(&g.world);
        for (key, upgraded) in [("target-base", false), ("target-next", true)] {
            let mut spec = eq::baseline_spec();
            if upgraded {
                spec.components
                    .insert("mobility".into(), "drive_mobile".into());
            }
            let profile = eq::design_preview(&g.world, NationId::USA, &spec)
                .profile
                .unwrap();
            let s = g
                .world
                .nation_mut(NationId::USA)
                .equipment
                .get_or_insert_with(Default::default);
            s.revisions.insert(
                key.into(),
                eq::DesignRevision {
                    id: key.into(),
                    name: key.into(),
                    specification_key: eq::specification_key(&spec),
                    spec,
                    profile,
                    created_day: day,
                    certified_day: Some(day),
                },
            );
            s.finance_from_day = day;
        }
        spheres_sim::arsenal::deliver_design(
            g.world.nation_mut(NationId::USA),
            "target-base",
            8,
            12.0,
        )
        .unwrap();
        g
    }
    #[test]
    fn target_library_action_and_preview_are_pure_and_only_confirmation_saves() {
        let mut g = fixture();
        let saved = spheres_sim::save(&g.world);
        let data = view(&g.world, NationId::USA, &g.session_id);
        assert!(data["designs"]
            .as_array()
            .unwrap()
            .iter()
            .all(|r| r["actions"]
                .as_array()
                .unwrap()
                .iter()
                .any(|a| a["command"]["kind"] == "equipment_target")));
        let command = json!({"kind":"equipment_target","revision":"target-next","quantity":12});
        let quote = preview(
            &g.world,
            NationId::USA,
            &g.session_id,
            &json!({"command":command}),
        )
        .unwrap();
        assert_eq!(quote["valid"], true);
        assert_eq!(quote["actions"][0]["label"], "Save fleet target");
        assert_eq!(spheres_sim::save(&g.world), saved);
        let parsed =
            super::super::parse_command(&g.world, &quote["actions"][0]["command"], NationId::USA)
                .unwrap();
        spheres_sim::apply_command(&mut g.world, &parsed).unwrap();
        assert_eq!(
            g.world
                .nation(NationId::USA)
                .equipment
                .as_ref()
                .unwrap()
                .fleet_targets["target-next"],
            12
        );
        for bad in [json!(-1), json!(1.5), json!("2"), json!(4294967296u64)] {
            assert!(super::super::parse_command(
                &g.world,
                &json!({"kind":"equipment_target","revision":"target-next","quantity":bad}),
                NationId::USA
            )
            .is_none());
        }
        assert!(
            super::super::parse_command(
                &g.world,
                &json!({"kind":"equipment_target","revision":"target-next"}),
                NationId::USA
            )
            .is_none(),
            "missing quantity must not silently clear a target"
        );
    }
    #[test]
    fn target_suggestions_are_reviewed_and_honor_source_goal() {
        let mut g = fixture();
        eq::set_fleet_target(&mut g.world, NationId::USA, "target-base", Some(6)).unwrap();
        eq::set_fleet_target(&mut g.world, NationId::USA, "target-next", Some(12)).unwrap();
        let saved = spheres_sim::save(&g.world);
        let board = targets_board(&g.world, NationId::USA);
        let next = board
            .iter()
            .find(|row| row["name"] == "target-next")
            .unwrap();
        let actions = next["actions"].as_array().unwrap();
        let production = actions
            .iter()
            .find(|a| a["command"]["kind"] == "equipment_produce")
            .unwrap();
        assert_eq!(production["command"]["quantity"], 12);
        assert_eq!(production["requires_preview"], true);
        let refit = actions
            .iter()
            .find(|a| a["command"]["kind"] == "equipment_refit")
            .unwrap();
        assert_eq!(refit["command"]["quantity"], 2);
        assert_eq!(refit["requires_preview"], true);
        assert!(refit["detail"].as_str().unwrap().contains("alternatives"));
        let stop = actions
            .iter()
            .find(|a| {
                a["command"]["kind"] == "equipment_target" && a["command"]["quantity"].is_null()
            })
            .unwrap();
        let quote = preview(
            &g.world,
            NationId::USA,
            &g.session_id,
            &json!({"command":stop["command"]}),
        )
        .unwrap();
        assert_eq!(quote["valid"], true);
        assert_eq!(quote["actions"][0]["label"], "Stop tracking target");
        assert_eq!(spheres_sim::save(&g.world), saved);
    }
}
