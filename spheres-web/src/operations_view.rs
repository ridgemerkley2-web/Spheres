//! Read-only links between the physical ground fleet and the shared operations model.
use serde_json::{json, Value};
use spheres_sim::{arsenal, clock, equipment as eq, operations, world::*};

fn day_label(day: i32) -> String {
    let (year, month, date) = clock::date_from_day(day);
    format!("{year:04}-{month:02}-{date:02}")
}

pub(crate) fn view(w: &WorldState, me: NationId) -> Value {
    let n = w.nation(me);
    let mut data = serde_json::to_value(operations::view(w, me)).unwrap();
    let state = n.equipment.as_ref();
    let mut models = Vec::new();
    for h in &n.arsenal.held {
        let Some(revision) = h
            .design_id
            .as_deref()
            .and_then(|id| state.and_then(|s| s.revisions.get(id)))
        else {
            continue;
        };
        if revision.profile.aviation.is_some() || h.units <= 0.0 {
            continue;
        }
        let available = arsenal::available_design_units(h);
        let reference = available as f64 * revision.profile.reference_weight_bn;
        let supported = if reference > 0.0 {
            (arsenal::combat_value(n, h) / reference).clamp(0.0, 1.0)
        } else {
            0.0
        };
        models.push(json!({"revision":revision.id,"name":revision.name,
            "platform":revision.spec.platform,"role":eq::platform_role(&revision.spec.platform),
            "delivered":h.units,"available":available,"reserved":h.refit_reserved,
            "supported_fraction":supported,"ammunition_family":eq::ammunition_family(&revision.spec),
            "ammunition_name":eq::ammunition_family(&revision.spec).and_then(eq::ammo_def).map(|ammo|ammo.name)}));
    }
    models.sort_by(|a, b| a["revision"].as_str().cmp(&b["revision"].as_str()));
    let recorded = state.and_then(|s| {
        s.maintenance_plan
            .as_ref()
            .and_then(|p| p.receipt.as_ref())
            .map(|r| r.day)
            .or(s.last_tick_day)
    });
    let last_report = state
        .and_then(|s| s.ground_operations_receipt.as_ref())
        .map(|receipt| {
            let mut report = serde_json::to_value(receipt).unwrap();
            report["day_label"] = json!(day_label(receipt.day));
            if let Some(rows) = report["revisions"].as_array_mut() {
                for row in rows {
                    if let Some(revision) = row["revision_id"]
                        .as_str()
                        .and_then(|id| state.and_then(|s| s.revisions.get(id)))
                    {
                        row["name"] = json!(revision.name);
                        row["platform"] = json!(revision.spec.platform);
                        row["role"] = json!(eq::platform_role(&revision.spec.platform));
                    }
                }
            }
            report
        });
    data["ground_fleet"] = json!({
        "title":"Ground equipment in service",
        "detail":"Delivered tanks and specialist vehicles support one shared national force. Refit reservations leave service. Maintenance and age affect their contribution; compatible stores determine firing. Force allocations are commitments, while the operation board records arrivals.",
        "models":models,
        "maintenance":{"coverage":state.map(|s|s.maintenance_fraction),"recorded_day":recorded,"day_label":recorded.map(day_label)},
        "actions":[{"label":"Service, refit and retirement","tab":"service"},
            {"label":"Compatible ammunition","tab":"ammunition"},{"label":"Buy from suppliers","tab":"companies"}],
        "last_report":last_report
    });
    data
}

#[cfg(test)]
mod tests {
    use super::*;
    const ME: NationId = NationId::France;

    fn fixture() -> WorldState {
        let mut w = spheres_sim::init::world_1990(GameRules {
            military_operations: true,
            daily_simulation: true,
            ..GameRules::default()
        });
        w.player = Some(ME);
        for platform in [
            "tank_standard",
            "ground_ifv",
            "ground_recon",
            "ground_artillery",
            "ground_air_defense",
            "air_light_attack",
        ] {
            let spec = eq::default_spec(platform);
            let preview = eq::design_preview(&w, ME, &spec);
            assert!(preview.valid, "{platform}: {:?}", preview.blockers);
            let day = clock::absolute_day(&w);
            w.nation_mut(ME)
                .equipment
                .get_or_insert_with(Default::default)
                .revisions
                .insert(
                    platform.into(),
                    eq::DesignRevision {
                        id: platform.into(),
                        name: format!("Test {platform}"),
                        specification_key: eq::specification_key(&spec),
                        spec,
                        profile: preview.profile.unwrap(),
                        created_day: day,
                        certified_day: Some(day),
                    },
                );
            arsenal::deliver_design(w.nation_mut(ME), platform, 10, 0.0).unwrap();
        }
        w
    }

    #[test]
    fn ground_panel_preserves_native_operations_and_excludes_aircraft_and_refit_reservations() {
        let mut w = fixture();
        arsenal::reserve_refit(w.nation_mut(ME), "ground_ifv", 4).unwrap();
        w.nation_mut(ME)
            .equipment
            .as_mut()
            .unwrap()
            .maintenance_fraction = 0.5;
        let before = spheres_sim::save(&w);
        let mut rendered = view(&w, ME);
        let panel = rendered
            .as_object_mut()
            .unwrap()
            .remove("ground_fleet")
            .unwrap();
        assert_eq!(
            rendered,
            serde_json::to_value(operations::view(&w, ME)).unwrap()
        );
        let rows = panel["models"].as_array().unwrap();
        assert_eq!(rows.len(), 5);
        let ifv = rows.iter().find(|r| r["revision"] == "ground_ifv").unwrap();
        assert_eq!(ifv["delivered"], 10.0);
        assert_eq!(ifv["available"], 6);
        assert_eq!(ifv["reserved"], 4);
        assert_eq!(ifv["supported_fraction"], 0.5);
        assert!(panel["maintenance"]["recorded_day"].is_null());
        assert_eq!(
            spheres_sim::save(&w),
            before,
            "reading does not appoint equipment or consume supplies"
        );
    }

    #[test]
    fn empty_fleet_and_unfunded_stock_do_not_invent_support_or_loss_reports() {
        let w = spheres_sim::init::world_1990(GameRules::default());
        let panel = view(&w, ME)["ground_fleet"].clone();
        assert_eq!(panel["models"], json!([]));
        assert!(panel["last_report"].is_null());
        let mut w = fixture();
        w.nation_mut(ME)
            .equipment
            .as_mut()
            .unwrap()
            .maintenance_fraction = 0.0;
        let data = view(&w, ME);
        assert!(data["ground_fleet"]["models"]
            .as_array()
            .unwrap()
            .iter()
            .all(|r| r["supported_fraction"] == 0.0));
        assert_eq!(
            data["capabilities"],
            serde_json::to_value(operations::capabilities(w.nation(ME))).unwrap()
        );
    }
}
