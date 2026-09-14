use super::*;
use crate::{campaign, equipment::ammunition_operation_tests as ammo, programs};

const ATTACKER: NationId = NationId::USA;
const DEFENDER: NationId = NationId::Canada;
const SECOND: NationId = NationId::Mexico;
const CID: u32 = 400;
const SECOND_CID: u32 = 401;
const BOMBER: &str = "ammo-model";
const FIGHTER: &str = "s16-authored-fighter";
const BOMBS: &str = "air_bomb_unguided";
const MISSILES: &str = "air_missile_short_range";

fn near(a: f64, b: f64) {
    assert!(
        (a - b).abs() <= 1e-9 * (1.0 + a.abs().max(b.abs())),
        "{a} != {b}"
    );
}

fn certify(w: &mut WorldState, id: NationId, key: &str, platform: &str, quantity: u32) {
    // Native mission fixtures author certified starting hardware and the
    // researched integrations; the separate supplier test owns acquisition.
    let state = w
        .nation_mut(id)
        .equipment
        .get_or_insert_with(Default::default);
    state
        .learned
        .extend(equipment::RESEARCH.iter().map(|r| r.id.into()));
    let spec = equipment::default_spec(platform);
    let preview = equipment::design_preview(w, id, &spec);
    assert!(preview.valid, "{platform}: {:?}", preview.blockers);
    let day = clock::absolute_day(w);
    w.nation_mut(id)
        .equipment
        .as_mut()
        .unwrap()
        .revisions
        .insert(
            key.into(),
            equipment::DesignRevision {
                id: key.into(),
                name: key.into(),
                specification_key: equipment::specification_key(&spec),
                spec,
                profile: preview.profile.unwrap(),
                created_day: day,
                certified_day: Some(day),
            },
        );
    arsenal::deliver_design(w.nation_mut(id), key, quantity, 0.0).unwrap();
}

fn accounts(w: &mut WorldState, id: NationId) {
    let player = w.player;
    w.player = Some(id);
    w.nation_mut(id).treasury_bn = Some(1000.0);
    programs::set_construction_budget(w, id, 0.0).unwrap();
    equipment::set_maintenance_plan(w, id, 1.0).unwrap();
    w.player = player;
}

fn base(w: &mut WorldState, id: NationId, district: &str) {
    if airbases::base(w, district).is_some() {
        return;
    }
    w.airbases.as_mut().unwrap().bases.push(airbases::Airbase {
        id: district.into(),
        district: district.into(),
        name: "Authored defensive airfield".into(),
        sponsor: id,
        capacity_level: 1,
        support_level: 0,
        protection_level: 0,
        project: None,
        history: vec![],
    });
}

fn squadron(
    w: &mut WorldState,
    id: NationId,
    revision: &str,
    quantity: u32,
    district: &str,
) -> u32 {
    let player = w.player;
    w.player = Some(id);
    aviation::apply(
        w,
        id,
        &aviation::SquadronCommand::Create {
            name: format!("Authored {revision} flight"),
            revision: revision.into(),
            quantity,
        },
    )
    .unwrap();
    let sq = w
        .nation_mut(id)
        .aviation
        .as_mut()
        .unwrap()
        .squadrons
        .last_mut()
        .unwrap();
    sq.base = Some(district.into());
    let key = sq.id;
    w.player = player;
    key
}

fn queue(
    w: &mut WorldState,
    id: NationId,
    sq: u32,
    conflict: u32,
    kind: MissionKind,
    target: &str,
) -> u32 {
    let player = w.player;
    w.player = Some(id);
    let q = quote(w, id, sq, conflict, kind, target);
    assert!(q.valid, "{id:?} {kind:?}: {:?}", q.reason);
    apply(
        w,
        id,
        &MissionCommand::Queue {
            squadron: sq,
            conflict,
            kind,
            target: target.into(),
        },
    )
    .unwrap();
    let key = w.air_missions.as_ref().unwrap().orders.last().unwrap().id;
    w.player = player;
    key
}

fn close(w: &mut WorldState) {
    for id in [ATTACKER, DEFENDER, SECOND] {
        if w.nation(id).program_budget.is_some() {
            programs::stage_fiscal(w.nation_mut(id), 0.0, 0.0);
        }
    }
    programs::finish_day(w);
}

fn open_next(w: &mut WorldState) {
    clock::advance_date(w);
    programs::begin_day(w);
    equipment::settle_support(w);
    equipment::tick_air_support(w);
}

fn day(w: &mut WorldState) {
    open_next(w);
    crate::war::tick(w);
    close(w);
    validate(w).unwrap();
    aviation::validate(w).unwrap();
    for id in [ATTACKER, DEFENDER, SECOND] {
        equipment::validate_state(w.nation(id)).unwrap();
    }
}

fn stock(w: &WorldState, id: NationId, family: &str) -> f64 {
    w.nation(id)
        .equipment
        .as_ref()
        .and_then(|e| e.ammunition.as_ref())
        .and_then(|a| a.stocks.get(family))
        .copied()
        .unwrap_or(0.0)
}

fn remaining(w: &mut WorldState, id: NationId, family: &str, quantity: f64) {
    let a = w
        .nation_mut(id)
        .equipment
        .as_mut()
        .unwrap()
        .ammunition
        .as_mut()
        .unwrap();
    let total = a.stocks.get(family).copied().unwrap_or(0.0)
        + a.consumed.get(family).copied().unwrap_or(0.0);
    assert!(quantity >= 0.0 && quantity <= total);
    a.stocks.insert(family.into(), quantity);
    a.consumed.insert(family.into(), total - quantity);
}

fn report(w: &WorldState, id: u32) -> &MissionReport {
    w.air_missions
        .as_ref()
        .unwrap()
        .orders
        .iter()
        .find(|o| o.id == id)
        .unwrap()
        .report
        .as_ref()
        .unwrap()
}

fn holding<'a>(w: &'a WorldState, id: NationId, revision: &str) -> &'a arsenal::Holding {
    w.nation(id)
        .arsenal
        .held
        .iter()
        .find(|h| h.design_id.as_deref() == Some(revision))
        .unwrap()
}

fn fixture() -> (WorldState, String, String) {
    let (mut w, home, target) = super::tests::fixture();
    accounts(&mut w, DEFENDER);
    certify(&mut w, DEFENDER, FIGHTER, "air_fighter", 8);
    base(&mut w, DEFENDER, &target);
    squadron(&mut w, DEFENDER, FIGHTER, 4, &target);
    squadron(&mut w, DEFENDER, FIGHTER, 4, &target);
    equipment::seed_test_ammunition(&mut w, DEFENDER, MISSILES, 1000);
    open_next(&mut w);
    close(&mut w);
    aviation::validate(&w).unwrap();
    airbases::validate(&w).unwrap();
    (w, home, target)
}

fn ground_defense(w: &mut WorldState, id: NationId) -> String {
    certify(w, id, "s16-ground-defense", "ground_air_defense", 20);
    let family = equipment::ammunition_family(
        &w.nation(id).equipment.as_ref().unwrap().revisions["s16-ground-defense"].spec,
    )
    .unwrap()
    .to_string();
    equipment::seed_test_ammunition(w, id, &family, 100_000);
    let today = clock::absolute_day(w);
    w.nation_mut(id)
        .equipment
        .as_mut()
        .unwrap()
        .ammunition
        .as_mut()
        .unwrap()
        .active_from_day = Some(today);
    family
}

#[test]
fn s16_defend_skies_requires_fighter_friendly_geography_and_real_readiness() {
    let (mut w, home, target) = fixture();
    w.player = Some(DEFENDER);
    let before = crate::save(&w);
    let q = quote(&w, DEFENDER, 1, CID, MissionKind::DefendSkies, &target);
    assert!(q.valid, "{:?}", q.reason);
    assert_eq!(q.family, MISSILES);
    assert!(q.stores_required > 0.0);
    assert_eq!(crate::save(&w), before);
    assert!(!quote(&w, DEFENDER, 1, CID, MissionKind::StrikeTarget, &home).valid);
    assert!(!quote(&w, DEFENDER, 1, CID, MissionKind::DefendSkies, &home).valid);
    w.player = Some(ATTACKER);
    assert!(!quote(&w, ATTACKER, 1, CID, MissionKind::DefendSkies, &home).valid);
    w.player = Some(DEFENDER);
    w.nation_mut(DEFENDER).aviation.as_mut().unwrap().squadrons[0].service_days_left = 1;
    assert!(!quote(&w, DEFENDER, 1, CID, MissionKind::DefendSkies, &target).valid);
    w.nation_mut(DEFENDER).aviation.as_mut().unwrap().squadrons[0].service_days_left = 0;
    remaining(&mut w, DEFENDER, MISSILES, 0.0);
    equipment::seed_test_ammunition(&mut w, DEFENDER, BOMBS, 1000);
    let dry = quote(&w, DEFENDER, 1, CID, MissionKind::DefendSkies, &target);
    assert!(!dry.valid);
    assert!(dry.reason.unwrap().contains("compatible"));
}

#[test]
fn s16_paid_interception_changes_the_real_hostile_strike_and_depleted_defense_loses_it() {
    let (mut baseline, _, target) = fixture();
    let attack = queue(
        &mut baseline,
        ATTACKER,
        1,
        CID,
        MissionKind::StrikeTarget,
        &target,
    );
    let mut wet = baseline.clone();
    let defend = queue(
        &mut wet,
        DEFENDER,
        1,
        CID,
        MissionKind::DefendSkies,
        &target,
    );
    let mut dry = wet.clone();
    remaining(&mut dry, DEFENDER, MISSILES, 0.0);
    day(&mut baseline);
    day(&mut wet);
    day(&mut dry);
    assert!(report(&wet, attack).applied_power < report(&baseline, attack).applied_power);
    assert!(
        wet.nation(DEFENDER).mil_strength > baseline.nation(DEFENDER).mil_strength,
        "Interception must reduce casualties through the ordinary campaign resolver"
    );
    near(
        report(&dry, attack).applied_power,
        report(&baseline, attack).applied_power,
    );
    near(
        dry.nation(DEFENDER).mil_strength,
        baseline.nation(DEFENDER).mil_strength,
    );
    assert!(
        report(&wet, attack)
            .defense
            .as_ref()
            .unwrap()
            .prevented_power
            > 0.0
    );
    assert!(
        report(&wet, defend)
            .defense
            .as_ref()
            .unwrap()
            .prevented_power
            > 0.0
    );
    assert_eq!(
        report(&wet, defend)
            .defense
            .as_ref()
            .unwrap()
            .ground_defense_expected_loss,
        0.0
    );
    assert_eq!(report(&dry, defend).stores_used, 0.0);
    let receipt = wet
        .nation(DEFENDER)
        .equipment
        .as_ref()
        .unwrap()
        .ammunition
        .as_ref()
        .unwrap()
        .last_consumption
        .as_ref()
        .unwrap();
    near(receipt.used[MISSILES], report(&wet, defend).stores_used);
    near(
        stock(&wet, DEFENDER, MISSILES),
        1000.0 - report(&wet, defend).stores_used,
    );
}

#[test]
fn s16_fighter_and_ground_air_defense_have_distinct_paid_contributions() {
    let (mut source, _, target) = fixture();
    let aa_family = ground_defense(&mut source, DEFENDER);
    let attack = queue(
        &mut source,
        ATTACKER,
        1,
        CID,
        MissionKind::StrikeTarget,
        &target,
    );
    let mut results = vec![];
    for (fighter, aa) in [(false, false), (true, false), (false, true), (true, true)] {
        let mut w = source.clone();
        if fighter {
            queue(&mut w, DEFENDER, 1, CID, MissionKind::DefendSkies, &target);
        }
        if !aa {
            remaining(&mut w, DEFENDER, &aa_family, 0.0);
        }
        open_next(&mut w);
        prepare(&mut w);
        let c = w.conflict(CID).unwrap().clone();
        let firing = fire(&w, &c, &target, true).0;
        let snapshot = operations::Snapshot::new(&w);
        let ground = snapshot.rows[&(CID, DEFENDER)]
            .ammunition
            .unwrap()
            .air_defense;
        assert_eq!(ground > 0.0, aa);
        record_contact(&mut w, &c, &target, &[(DEFENDER, ground)]);
        snapshot.settle(&mut w);
        settle(&mut w);
        let r = report(&w, attack);
        let effect = r.defense.as_ref();
        let air_loss = effect.map_or(0.0, |e| e.air_combat_expected_loss);
        assert_eq!(air_loss > 0.0, fighter);
        results.push((
            firing,
            ground,
            w.air_missions
                .as_ref()
                .unwrap()
                .plans
                .iter()
                .find(|p| p.order == attack)
                .unwrap()
                .expected_loss,
            air_loss,
        ));
        validate(&w).unwrap();
    }
    near(results[0].0, results[2].0);
    near(results[1].0, results[3].0);
    assert!(
        results[1].0 < results[0].0,
        "Fighters reduce the incoming flight before ground defense"
    );
    assert!(
        results[2].2 > results[0].2,
        "Loaded ground defense has its own aircraft-loss contribution"
    );
    assert!(results[3].2 > results[1].2);
    near(results[1].3, results[3].3);
}

#[test]
fn s16_multiple_interceptors_and_ground_defense_cannot_write_off_one_target_twice() {
    let (mut w, _, target) = fixture();
    let attack = queue(&mut w, ATTACKER, 1, CID, MissionKind::StrikeTarget, &target);
    queue(&mut w, DEFENDER, 1, CID, MissionKind::DefendSkies, &target);
    queue(&mut w, DEFENDER, 2, CID, MissionKind::DefendSkies, &target);
    w.nation_mut(ATTACKER)
        .arsenal
        .held
        .iter_mut()
        .find(|h| h.design_id.as_deref() == Some(BOMBER))
        .unwrap()
        .loss_remainder = 0.999999;
    let parked = w.nation(ATTACKER).aviation.as_ref().unwrap().squadrons[1].clone();
    open_next(&mut w);
    prepare(&mut w);
    let saved = crate::save(&w);
    let mut resumed = crate::load(&saved).unwrap();
    for state in [&mut w, &mut resumed] {
        let c = state.conflict(CID).unwrap().clone();
        record_contact(state, &c, &target, &[(DEFENDER, 0.7)]);
        let contacted = crate::save(state);
        record_contact(state, &c, &target, &[(DEFENDER, 0.7)]);
        assert_eq!(crate::save(state), contacted);
        let p = state
            .air_missions
            .as_ref()
            .unwrap()
            .plans
            .iter()
            .find(|p| p.order == attack)
            .unwrap();
        let e = p.defense.as_ref().unwrap();
        assert_eq!(e.opposing_missions, 2);
        assert!(e.air_combat_expected_loss > 0.000001 && e.ground_defense_expected_loss > 0.0);
        assert!(p.expected_loss <= p.aircraft as f64);
        let mut snapshot = operations::Snapshot::new(state);
        snapshot.record_loss(CID, ATTACKER, 0.5);
        snapshot.settle(state);
        settle(state);
        let after = crate::save(state);
        settle(state);
        prepare(state);
        assert_eq!(crate::save(state), after);
        assert_eq!(report(state, attack).aircraft_lost, 1);
        assert_eq!(holding(state, ATTACKER, BOMBER).units, 3.0);
        assert_eq!(
            state.nation(ATTACKER).aviation.as_ref().unwrap().squadrons[0].assigned,
            1
        );
        assert_eq!(
            state.nation(ATTACKER).aviation.as_ref().unwrap().squadrons[1],
            parked
        );
        validate(state).unwrap();
        aviation::validate(state).unwrap();
    }
    assert_eq!(crate::save(&w), crate::save(&resumed));
}

fn second_front(w: &mut WorldState) -> (String, String) {
    // Authored diplomatic starting state: these native host grants permit
    // operations in both regions without bypassing mission access or range.
    for (host, seeker, theatre) in [
        (
            NationId::Brazil,
            ATTACKER,
            crate::theatre::TheatreId::LatinAmerica,
        ),
        (
            NationId::Canada,
            SECOND,
            crate::theatre::TheatreId::NorthAmerica,
        ),
    ] {
        w.access.push(crate::theatre::Access {
            host,
            seeker,
            theatre,
            since_year: w.year,
            since_month: w.month,
        });
        assert!(crate::theatre::has_access(w, seeker, theatre));
    }
    w.conflicts.push(ammo::conflict(w, SECOND_CID, SECOND));
    let snapshot = operations::Snapshot::new(w);
    let _ = campaign::prepare(w, &snapshot);
    let contested = front::contested_set(w, w.conflict(SECOND_CID).unwrap());
    let mut choices = vec![];
    for (target, (side_a, _)) in &contested.k {
        if *side_a {
            continue;
        }
        let Some(t) = airbases::district_location(target) else {
            continue;
        };
        // The second theatre has real timed army transfers rather than a
        // second migration grant. These Strike/Defend orders need geographic
        // bases and airborne opposition, not an already arrived army sector.
        for home in crate::districts::adj_of(target)
            .iter()
            .filter(|d| w.districts.get(*d) == Some(&ATTACKER))
        {
            let Some(b) = airbases::district_location(home) else {
                continue;
            };
            choices.push((airbases::distance_km(b, t), home.clone(), target.clone()));
        }
    }
    choices.sort_by(|a, b| {
        a.0.total_cmp(&b.0)
            .then_with(|| a.1.cmp(&b.1))
            .then_with(|| a.2.cmp(&b.2))
    });
    let (_, home, target) = choices
        .into_iter()
        .next()
        .expect("Native US/Mexico frontier contact");
    (home, target)
}

#[test]
fn s16_both_sides_and_two_conflicts_share_one_finite_national_store_plan() {
    let (mut w, north, canada) = fixture();
    let (south, mexico) = second_front(&mut w);
    accounts(&mut w, SECOND);
    base(&mut w, ATTACKER, &south);
    base(&mut w, SECOND, &mexico);
    w.nation_mut(ATTACKER).aviation.as_mut().unwrap().squadrons[1].base = Some(south.clone());
    certify(&mut w, ATTACKER, FIGHTER, "air_fighter", 8);
    let north_guard = squadron(&mut w, ATTACKER, FIGHTER, 4, &north);
    let south_guard = squadron(&mut w, ATTACKER, FIGHTER, 4, &south);
    certify(&mut w, SECOND, FIGHTER, "air_fighter", 4);
    let mexico_guard = squadron(&mut w, SECOND, FIGHTER, 4, &mexico);
    certify(
        &mut w,
        DEFENDER,
        "s16-canadian-bomber",
        "air_light_attack",
        2,
    );
    let canada_attack = squadron(&mut w, DEFENDER, "s16-canadian-bomber", 2, &canada);
    certify(&mut w, SECOND, "s16-mexican-bomber", "air_light_attack", 2);
    let mexico_attack = squadron(&mut w, SECOND, "s16-mexican-bomber", 2, &mexico);
    for id in [ATTACKER, SECOND] {
        equipment::seed_test_ammunition(&mut w, id, MISSILES, 1000);
    }
    for id in [DEFENDER, SECOND] {
        equipment::seed_test_ammunition(&mut w, id, BOMBS, 1000);
    }
    open_next(&mut w);
    close(&mut w);
    for (id, sq, cid, kind, target) in [
        (ATTACKER, 1, CID, MissionKind::StrikeTarget, canada.as_str()),
        (
            ATTACKER,
            2,
            SECOND_CID,
            MissionKind::StrikeTarget,
            mexico.as_str(),
        ),
        (DEFENDER, 1, CID, MissionKind::DefendSkies, canada.as_str()),
        (
            SECOND,
            mexico_guard,
            SECOND_CID,
            MissionKind::DefendSkies,
            mexico.as_str(),
        ),
        (
            DEFENDER,
            canada_attack,
            CID,
            MissionKind::StrikeTarget,
            north.as_str(),
        ),
        (
            SECOND,
            mexico_attack,
            SECOND_CID,
            MissionKind::StrikeTarget,
            south.as_str(),
        ),
        (
            ATTACKER,
            north_guard,
            CID,
            MissionKind::DefendSkies,
            north.as_str(),
        ),
        (
            ATTACKER,
            south_guard,
            SECOND_CID,
            MissionKind::DefendSkies,
            south.as_str(),
        ),
    ] {
        queue(&mut w, id, sq, cid, kind, target);
    }
    let mut requirements = BTreeMap::<String, f64>::new();
    for o in &w.air_missions.as_ref().unwrap().orders {
        if o.nation == ATTACKER {
            let mut view = w.clone();
            view.player = Some(ATTACKER);
            let q = quote_inner(
                &view, o.nation, o.squadron, o.conflict, o.kind, &o.target, false,
            );
            *requirements.entry(q.family).or_default() += q.stores_required;
        }
    }
    for (family, required) in &requirements {
        remaining(&mut w, ATTACKER, family, required * 0.5);
    }
    let before_bombs = stock(&w, ATTACKER, BOMBS);
    let before_missiles = stock(&w, ATTACKER, MISSILES);
    let mut reversed = w.clone();
    reversed.conflicts.reverse();
    let mut resumed = crate::load(&crate::save(&w)).unwrap();
    for state in [&mut w, &mut reversed, &mut resumed] {
        day(state);
        assert_eq!(state.air_missions.as_ref().unwrap().orders.len(), 8);
        assert!(state
            .air_missions
            .as_ref()
            .unwrap()
            .orders
            .iter()
            .all(|o| o.status == MissionStatus::Flown));
        for family in [BOMBS, MISSILES] {
            let used: f64 = state
                .air_missions
                .as_ref()
                .unwrap()
                .orders
                .iter()
                .filter(|o| o.nation == ATTACKER)
                .filter_map(|o| o.report.as_ref())
                .filter(|r| r.family == family)
                .map(|r| r.stores_used)
                .sum();
            let receipt = state
                .nation(ATTACKER)
                .equipment
                .as_ref()
                .unwrap()
                .ammunition
                .as_ref()
                .unwrap()
                .last_consumption
                .as_ref()
                .unwrap();
            near(used, receipt.used[family]);
            near(
                used,
                if family == BOMBS {
                    before_bombs
                } else {
                    before_missiles
                },
            );
            near(stock(state, ATTACKER, family), 0.0);
        }
        assert!(state
            .air_missions
            .as_ref()
            .unwrap()
            .plans
            .iter()
            .filter(|p| p.nation == ATTACKER)
            .all(|p| p.coverage > 0.0 && p.coverage < 1.0));
        assert!(state
            .air_missions
            .as_ref()
            .unwrap()
            .orders
            .iter()
            .filter(|o| o.kind == MissionKind::DefendSkies)
            .all(|o| o
                .report
                .as_ref()
                .unwrap()
                .defense
                .as_ref()
                .unwrap()
                .prevented_power
                > 0.0));
        state.conflicts.sort_by_key(|c| c.id);
    }
    assert_eq!(crate::save(&w), crate::save(&reversed));
    assert_eq!(crate::save(&w), crate::save(&resumed));
}

#[test]
fn s16_defensive_patrol_has_no_ground_attack_or_hostile_ground_defense_exposure() {
    let (mut w, _, target) = fixture();
    let aa = ground_defense(&mut w, ATTACKER);
    let defense = queue(&mut w, DEFENDER, 1, CID, MissionKind::DefendSkies, &target);
    open_next(&mut w);
    prepare(&mut w);
    let c = w.conflict(CID).unwrap().clone();
    assert_eq!(exposure(&w, DEFENDER, CID), 0.0);
    assert_eq!(fire(&w, &c, &target, false), (0.0, 0.0));
    let overview = equipment::ammunition_overview(&w, ATTACKER);
    assert_eq!(
        overview
            .families
            .iter()
            .find(|f| f.family == aa)
            .unwrap()
            .required,
        0.0
    );
    let snapshot = operations::Snapshot::new(&w);
    record_contact(&mut w, &c, &target, &[(ATTACKER, 0.35)]);
    snapshot.settle(&mut w);
    settle(&mut w);
    let r = report(&w, defense);
    assert_eq!(r.applied_power, 0.0);
    assert_eq!(r.aircraft_lost, 0);
    let effect = r.defense.as_ref().unwrap();
    assert_eq!(effect.opposing_missions, 0);
    assert_eq!(effect.prevented_power, 0.0);
    assert_eq!(effect.air_combat_expected_loss, 0.0);
    assert_eq!(effect.ground_defense_expected_loss, 0.0);
    validate(&w).unwrap();
}

#[test]
fn s16_saved_defense_effects_cannot_invent_contacts_suppression_or_losses() {
    let (mut w, _, target) = fixture();
    let attack = queue(&mut w, ATTACKER, 1, CID, MissionKind::StrikeTarget, &target);
    queue(&mut w, DEFENDER, 1, CID, MissionKind::DefendSkies, &target);
    open_next(&mut w);
    prepare(&mut w);
    let saved = crate::save(&w);
    let mut resumed = crate::load(&saved).unwrap();
    let mut forged = w.clone();
    forged.air_missions.as_mut().unwrap().plans[0].defense = Some(AirDefenseEffect {
        opposing_missions: 1,
        prevented_power: 1.0,
        air_combat_expected_loss: 1.0,
        ground_defense_expected_loss: 0.0,
    });
    assert!(validate(&forged).is_err());
    assert!(crate::load(&crate::save(&forged)).is_err());
    for state in [&mut w, &mut resumed] {
        let c = state.conflict(CID).unwrap().clone();
        let snapshot = operations::Snapshot::new(state);
        record_contact(state, &c, &target, &[(DEFENDER, 0.35)]);
        snapshot.settle(state);
        settle(state);
    }
    assert_eq!(crate::save(&w), crate::save(&resumed));
    for flaw in 0..5 {
        let mut wrong = w.clone();
        let r = wrong
            .air_missions
            .as_mut()
            .unwrap()
            .orders
            .iter_mut()
            .find(|o| o.id == attack)
            .unwrap()
            .report
            .as_mut()
            .unwrap();
        let e = r.defense.as_mut().unwrap();
        match flaw {
            0 => e.opposing_missions += 1,
            1 => e.prevented_power += 1.0,
            2 => e.air_combat_expected_loss += 1.0,
            3 => e.ground_defense_expected_loss += 1.0,
            _ => r.defense = None,
        }
        assert!(
            validate(&wrong).is_err(),
            "Forged interception report {flaw}"
        );
        assert!(crate::load(&crate::save(&wrong)).is_err());
    }
}
