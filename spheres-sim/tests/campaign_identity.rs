//! Operational conflict identities survive closure and save/load. Legacy
//! worlds retain their calibrated reuse of max-live conflict identifiers.
use spheres_sim::{
    campaign::{self, AirMission, Approach, NavalMission, OperationOrder},
    campaign_peace::{self, OccupationPolicy, PeaceOrder, Terms, WarAim},
    clock, commitment,
    init::world_1990,
    war,
    world::{GameRules, NationId as N, Objective, WorldState},
};

fn world() -> WorldState {
    let mut w = world_1990(GameRules {
        daily_simulation: true,
        military_operations: true,
        operational_warfare: 1,
        ai_aggression: 0.0,
        ..Default::default()
    });
    w.player = Some(N::Iraq);
    for n in &mut w.nations {
        n.political_capital = 100.0;
    }
    w
}
fn open(w: &mut WorldState, enemy: N) -> u32 {
    let theatre = war::theatre_between(w, N::Iraq, enemy);
    commitment::open_conflict(w, N::Iraq, enemy, theatre).unwrap()
}
fn configure(w: &mut WorldState, id: u32) {
    let c = w.conflict_mut(id).unwrap();
    for b in &mut c.posture {
        b.rung = 8;
    }
    war::join_side(c, N::Syria, false, 8, Objective::Seize);
    campaign::set_order(
        w,
        &OperationOrder {
            conflict: id,
            nation: N::Iraq,
            target: None,
            approach: Approach::Hold,
            reserve_bp: 4000,
            air: AirMission::None,
            naval: NavalMission::None,
        },
    )
    .unwrap();
    campaign_peace::apply(
        w,
        N::Iraq,
        &PeaceOrder::SetAim {
            conflict: id,
            aim: WarAim::Concession,
        },
    )
    .unwrap();
    campaign_peace::apply(
        w,
        N::Iraq,
        &PeaceOrder::Garrison {
            conflict: id,
            share_bp: 2000,
            policy: OccupationPolicy::Security,
        },
    )
    .unwrap();
}
fn close(w: &mut WorldState, id: u32) {
    campaign_peace::apply(
        w,
        N::Iraq,
        &PeaceOrder::Propose {
            conflict: id,
            terms: Terms::Ceasefire,
        },
    )
    .unwrap();
    let offer = w.campaign_peace.offers.last().unwrap().clone();
    for n in offer.participants.into_iter().filter(|n| *n != N::Iraq) {
        campaign_peace::apply(
            w,
            n,
            &PeaceOrder::Respond {
                offer: offer.id,
                accept: true,
            },
        )
        .unwrap();
    }
    assert!(w.conflict(id).is_none());
}

#[test]
fn immediate_redeclaration_cannot_inherit_previous_coalition_orders_or_deployed_force() {
    for next_enemy in [N::Iran, N::SaudiArabia] {
        let mut w = world();
        let old = open(&mut w, N::Iran);
        configure(&mut w, old);
        war::tick(&mut w);
        assert!(w
            .campaign
            .sectors
            .values()
            .any(|s| s.conflict == old && s.nation == N::Iraq));
        close(&mut w, old);
        // Exercise the other real creation path before any cleanup tick.
        war::declare_war(&mut w, N::Iraq, next_enemy).unwrap();
        let new = w.conflict_between(N::Iraq, next_enemy).unwrap().id;
        assert!(
            new > old,
            "a new operational war must never reuse the closed identity"
        );
        if next_enemy == N::Iran {
            war::join_side(
                w.conflict_mut(new).unwrap(),
                N::Syria,
                false,
                8,
                Objective::Seize,
            );
        }
        let view = campaign::view(&w, new, N::Iraq).unwrap();
        assert_eq!(
            view.fielded, 0.0,
            "old deployed troops must finish repatriation/reassignment"
        );
        assert_eq!(
            view.in_transit, 0.0,
            "old dated transfers cannot become a new deployment"
        );
        assert_ne!(view.order.approach, Approach::Hold);
        assert!(view.last_report.is_none());
        assert_eq!(view.enemy.observed_day, -1);
        let peace = campaign_peace::view(&w, w.conflict(new).unwrap(), N::Iraq);
        assert_eq!(peace["aim"]["kind"], "expel");
        assert_eq!(peace["garrison"]["share_bp"], 0);
        assert!(peace["proposals"].as_array().unwrap().is_empty());
        clock::advance_date(&mut w);
        war::tick(&mut w);
        assert!(w.campaign.sectors.values().all(|s| s.conflict != old));
        let retired = format!("{old}:");
        assert!(w
            .campaign_peace
            .aims
            .keys()
            .all(|key| !key.starts_with(&retired)));
        assert!(w
            .campaign_peace
            .garrisons
            .keys()
            .all(|key| !key.starts_with(&retired)));
        assert!(w
            .campaign_peace
            .last_proposal
            .keys()
            .all(|key| !key.starts_with(&retired)));
        assert!(
            w.campaign_peace
                .history
                .iter()
                .any(|r| r.offer.conflict == old),
            "closed intent cleanup must preserve accepted history"
        );
        assert!(
            w.campaign
                .transfers
                .iter()
                .any(|t| t.nation == N::Iraq && t.conflict.is_none()),
            "closure must retain real force on its dated return journey"
        );
        assert!(
            w.campaign
                .transfers
                .iter()
                .filter(|t| t.nation == N::Iraq && t.conflict.is_none())
                .all(|t| t.departed_day == clock::absolute_day(&w)
                    && t.arrives_day == clock::absolute_day(&w) + 14),
            "retired identity must not shorten the existing repatriation rule"
        );
        assert!(
            (campaign::accounted_force(&w, N::Iraq) - w.nation(N::Iraq).mil_strength).abs() < 1e-7
        );
    }
}

#[test]
fn upgrading_a_save_seeds_identity_from_closed_history_and_persists_reservations() {
    let mut w = world();
    let old = open(&mut w, N::Iran);
    configure(&mut w, old);
    close(&mut w, old);
    // An already-enabled pre-fix save has retained peace metadata but no
    // monotonic counter. Remove only that new field to exercise migration.
    let mut saved = serde_json::to_value(&w).unwrap();
    saved["campaign"]
        .as_object_mut()
        .unwrap()
        .remove("conflict_id_high_water");
    let mut migrated: WorldState = serde_json::from_value(saved).unwrap();
    migrated.reindex();
    let next = open(&mut migrated, N::SaudiArabia);
    assert!(
        next > old,
        "old live IDs are insufficient once only peace history remains"
    );
    migrated.conflicts.clear();
    migrated.campaign_peace = Default::default();
    migrated.campaign.orders.clear();
    // A reservation survives even when its war closes before the first tick.
    let mut resumed = spheres_sim::load(&spheres_sim::save(&migrated)).unwrap();
    let third = open(&mut resumed, N::Iran);
    assert!(
        third > next,
        "synchronous reservations must survive empty worlds and saves"
    );
}

#[test]
fn legacy_identity_and_default_serialization_remain_unchanged() {
    let mut legacy = world_1990(GameRules::default());
    let first = open(&mut legacy, N::Iran);
    legacy.conflicts.clear();
    assert_eq!(open(&mut legacy, N::SaudiArabia), first);
    assert!(!spheres_sim::save(&legacy).contains("conflict_id_high_water"));
    assert!(!spheres_sim::save(&legacy).contains("\"campaign\""));
}

#[test]
fn previews_and_exhausted_creation_commands_are_atomic() {
    let mut w = world();
    w.campaign.conflict_id_high_water = u32::MAX;
    let before = spheres_sim::save(&w);
    assert_eq!(w.next_conflict_id(), u32::MAX);
    assert_eq!(
        spheres_sim::save(&w),
        before,
        "preview cannot reserve an identity"
    );
    for command in [
        spheres_sim::Command::DeclareWar {
            attacker: N::Iraq,
            defender: N::Iran,
        },
        spheres_sim::Command::OpenConflict {
            opener: N::Iraq,
            target: N::SaudiArabia,
            theatre: war::theatre_between(&w, N::Iraq, N::SaudiArabia),
        },
    ] {
        let refusal = spheres_sim::refusal_of(&w, &command)
            .expect("unavailable IDs must be refused before charging");
        assert!(refusal.contains("identifiers"));
        assert!(spheres_sim::apply_command(&mut w, &command).is_err());
        assert_eq!(
            spheres_sim::save(&w),
            before,
            "refusal cannot charge capital, reserve IDs, add access or change commitments"
        );
    }
}

#[test]
fn migration_adopts_old_counter_and_garrison_ids_before_cleanup() {
    let mut w = world();
    w.daily.counters.insert("war:81:age".into(), 0.25);
    w.campaign_peace.garrisons.insert(
        "82:Iraq".into(),
        campaign_peace::GarrisonPolicy {
            share_bp: 2500,
            policy: OccupationPolicy::Security,
        },
    );
    campaign::enroll(&mut w);
    w.daily.counters.clear();
    w.campaign_peace = Default::default();
    let mut resumed = spheres_sim::load(&spheres_sim::save(&w)).unwrap();
    assert_eq!(
        open(&mut resumed, N::Iran),
        83,
        "old namespace IDs remain reserved after their ephemeral state disappears"
    );
}

#[test]
fn an_agency_only_old_save_cannot_reapply_a_previous_wars_call_to_arms() {
    for pending in [true, false] {
        let mut w = world();
        let old = spheres_sim::agency::Offer {
            id: 1,
            from: N::Iran,
            to: N::Syria,
            kind: spheres_sim::agency::OfferKind::CallToArms {
                conflict: 91,
                attacker: N::Iraq,
                year: w.year,
                month: w.month,
                rung: 8,
                guaranteed: false,
            },
            issued_day: clock::absolute_day(&w),
            expires_day: clock::absolute_day(&w) + 7,
        };
        if pending {
            w.agency.offers.push(old);
        } else {
            w.agency.history.push(spheres_sim::agency::Decision {
                offer: old,
                resolved_day: clock::absolute_day(&w),
                outcome: "expired".into(),
            });
        }
        w.agency.next_id = 1;
        let mut resumed = spheres_sim::load(&spheres_sim::save(&w)).unwrap();
        // DeclareWar historically lacked OpenConflict's retire-conflict hook.
        war::declare_war(&mut resumed, N::Iraq, N::Iran).unwrap();
        assert_eq!(resumed.conflicts[0].id, 92);
        if pending {
            resumed.player = Some(N::Syria);
            assert!(spheres_sim::agency::respond(&mut resumed, N::Syria, 1, true).is_err());
            assert!(!resumed.conflicts[0].involves(N::Syria));
        }
    }
}
