//! W3/W5 settlement boundaries. The fronts, vote support, opening appropriation
//! and completed Arms Plant are disclosed technical prerequisites, not simulated
//! victories, election forecasts or completed construction. All proposals,
//! consent, company formation, public payment and settlement use actual APIs.
use spheres_sim::world::{
    Belligerent, Conflict, GameRules, NationId as N, Objective, WorldState, BUDGET_DEFENSE as D,
};
use spheres_sim::{
    apply_command,
    campaign_peace::{self, PeaceOrder, Terms},
    companies, control, districts, government,
    init::world_1990,
    load, party_leadership, population, production, programs, resources, save, tick_day, war,
    Command,
};

fn world(winner: N, loser: N) -> WorldState {
    let mut w = world_1990(GameRules {
        daily_simulation: true,
        military_operations: true,
        operational_warfare: 1,
        production_system: true,
        manufacturing_system: true,
        resource_market: true,
        ideology_blocs: true,
        ai_aggression: 0.0,
        crisis_intensity: 0.0,
        ..Default::default()
    });
    w.conflicts.clear();
    w.player = Some(loser);
    for id in [winner, loser] {
        w.nation_mut(id).political_capital = 1000.0;
    }
    government::ensure_all(&mut w);
    party_leadership::enable_campaign(&mut w).unwrap();
    w
}
fn front(w: &mut WorldState, winner: N, loser: N, held: &[String]) {
    w.conflicts.push(Conflict {
        id: 1,
        theatre: war::theatre_between(w, winner, loser),
        side_a: vec![winner],
        side_b: vec![loser],
        posture: vec![
            Belligerent::new(winner, 8, Objective::Seize),
            Belligerent::new(loser, 8, Objective::Hold),
        ],
        control: 0.0,
        months: 12,
        quiet_months: 0,
        frozen_since: None,
        start_year: 1990,
        start_month: 1,
        origin_attacker: winner,
        invasion_declared: true,
        front: held.iter().map(|d| (d.clone(), 1.0)).collect(),
        pockets: vec![],
        aim: None,
    });
}
fn order(w: &mut WorldState, n: N, order: PeaceOrder) -> Result<(), String> {
    apply_command(w, &Command::WarDiplomacy { nation: n, order })
}
fn propose(w: &mut WorldState, n: N, terms: Terms) -> u64 {
    order(w, n, PeaceOrder::Propose { conflict: 1, terms }).unwrap();
    w.campaign_peace.offers.last().unwrap().id
}
fn respond(w: &mut WorldState, n: N, offer: u64) -> Result<(), String> {
    order(
        w,
        n,
        PeaceOrder::Respond {
            offer,
            accept: true,
        },
    )
}
fn canonical(w: &WorldState) -> WorldState {
    let bytes = save(w);
    let loaded = load(&bytes).unwrap();
    assert_eq!(
        save(&loaded),
        bytes,
        "loading a pending settlement must not execute it"
    );
    loaded
}
fn near(a: f64, b: f64) {
    assert!((a - b).abs() < 1e-8, "{a} != {b}");
}
fn money(w: &WorldState, n: N) -> (Option<f64>, Option<f64>) {
    (w.nation(n).treasury_bn, w.nation(n).debt_bn)
}
fn assert_once(w: &mut WorldState, loser: N, offer: u64) {
    let settled = save(w);
    assert!(respond(w, loser, offer).is_err());
    assert_eq!(
        save(w),
        settled,
        "replayed consent cannot repeat settlement"
    );
    campaign_peace::tick(w);
    let once = save(w);
    campaign_peace::tick(w);
    assert_eq!(
        save(w),
        once,
        "same-day peace tick cannot repeat settlement"
    );
    assert_eq!(
        w.campaign_peace
            .history
            .iter()
            .filter(|r| r.offer.id == offer && r.outcome == "accepted")
            .count(),
        1
    );
}

#[test]
fn consented_cession_transfers_one_district_and_residents_without_acquiring_its_company() {
    let (winner, loser) = (N::France, N::Germany);
    let mut w = world(winner, loser);
    let d = w
        .districts
        .iter()
        .find(|(_, n)| **n == loser)
        .unwrap()
        .0
        .clone();
    // Existing completed plant and prior fiscal authority: neither is granted by peace.
    if let Some(site) = w.production.provinces.iter_mut().find(|p| p.district == d) {
        site.arms_plants = 1;
    } else {
        w.production
            .provinces
            .push(production::ProvinceCapabilities {
                district: d.clone(),
                arms_plants: 1,
                infrastructure: 0,
                civilian_industry: 0,
                power_grid: 0,
                research_centers: 0,
            });
        w.production
            .provinces
            .sort_by(|a, b| a.district.cmp(&b.district));
    }
    programs::set_construction_budget(&mut w, loser, 0.0).unwrap();
    resources::tick(&mut w);
    for n in [winner, loser] {
        let nation = w.nation_mut(n);
        nation.treasury_bn = Some(100.0);
        nation.debt_bn = Some(0.0);
        nation.debt_gdp = 0.0;
    }
    apply_command(&mut w, &Command::EnableConnectedEconomy { nation: loser }).unwrap();
    apply_command(&mut w, &Command::EnableCompanies { nation: loser }).unwrap();
    w.nation_mut(loser)
        .program_budget
        .as_mut()
        .unwrap()
        .available_bn[D][3] = 10.0;
    let q = companies::establishment_quote(&w, loser, "Peace Fixture Works", &d, 1.0);
    assert!(q.valid, "{:?}", q.reason);
    apply_command(
        &mut w,
        &Command::Company {
            nation: loser,
            order: companies::CompanyOrder::Establish {
                name: "Peace Fixture Works".into(),
                district: d.clone(),
                capitalization_bn: 1.0,
                quote: q.token,
            },
        },
    )
    .unwrap();
    // One ordinary daily settlement converts the paid capital receivable into cash.
    tick_day(&mut w, &[]);
    let company = w
        .companies
        .firms
        .iter()
        .find(|c| c.nation == loser)
        .unwrap()
        .id;
    assert_eq!(
        companies::company(&w, loser, company)
            .unwrap()
            .capital_received_bn,
        1.0
    );
    assert!(companies::company(&w, loser, company).unwrap().cash_bn > 0.0);
    front(&mut w, winner, loser, &[d.clone()]);
    assert_eq!(control::controller(&w, &d), Some(winner));
    assert_eq!(w.districts[&d], loser, "occupation alone is not cession");
    let offer = propose(
        &mut w,
        winner,
        Terms::Cede {
            districts: vec![d.clone()],
        },
    );
    let before = canonical(&w);
    let mut resumed = canonical(&w);
    let mut transferred = before.clone();
    districts::transfer_district(&mut transferred, loser, winner, &d).unwrap();
    let companies_before = w.companies.clone();
    let sites = serde_json::to_value(&w.production.provinces).unwrap();
    let arsenal = [winner, loser].map(|n| serde_json::to_value(&w.nation(n).arsenal).unwrap());
    let cohort = w.population_system.provinces[&d].clone();
    let global_population: f64 = w.nations.iter().map(|n| n.population).sum();
    let global_gdp: f64 = w.nations.iter().map(|n| n.gdp).sum();
    for state in [&mut w, &mut resumed] {
        respond(state, loser, offer).unwrap();
        assert!(state.conflict(1).is_none());
        assert_eq!(state.districts[&d], winner);
        assert_eq!(control::controller(state, &d), Some(winner));
        assert_eq!(
            state.districts, transferred.districts,
            "only the consented district can move"
        );
        assert!(districts::deltas(state).contains(&(d.clone(), winner)));
        assert_eq!(
            state.companies, companies_before,
            "cession cannot nationalize company books"
        );
        assert_eq!(
            serde_json::to_value(&state.production.provinces).unwrap(),
            sites
        );
        let firm = companies::company(state, loser, company).unwrap();
        assert_eq!(firm.district, d);
        assert_eq!(firm.nation, loser);
        assert!(companies::facility_blocker(state, firm).is_some());
        assert!(companies::company(state, winner, company).is_none());
        for (i, n) in [winner, loser].into_iter().enumerate() {
            near(state.nation(n).population, transferred.nation(n).population);
            near(state.nation(n).gdp, transferred.nation(n).gdp);
            near(state.nation(n).oil_mbd, transferred.nation(n).oil_mbd);
            assert_eq!(money(state, n), money(&before, n));
            assert_eq!(
                serde_json::to_value(&state.nation(n).arsenal).unwrap(),
                arsenal[i]
            );
        }
        near(
            state.nations.iter().map(|n| n.population).sum(),
            global_population,
        );
        near(state.nations.iter().map(|n| n.gdp).sum(), global_gdp);
        let moved = &state.population_system.provinces[&d];
        assert_eq!(moved.last_owner, winner);
        assert_eq!(moved.children, cohort.children);
        assert_eq!(moved.working, cohort.working);
        assert_eq!(moved.courses, cohort.courses);
        assert_eq!(moved.retirees_m, cohort.retirees_m);
        population::validate(state).unwrap();
        assert_once(state, loser, offer);
        canonical(state);
    }
    assert_eq!(save(&w), save(&resumed));
    tick_day(&mut w, &[]);
    tick_day(&mut resumed, &[]);
    assert_eq!(save(&w), save(&resumed));
    assert_eq!(
        companies::company(&w, loser, company)
            .unwrap()
            .capital_received_bn,
        1.0
    );
}

#[test]
fn consented_political_transition_uses_reviewed_executive_identity_and_preserves_party_incumbents()
{
    // Japan's existing reviewed JSP contender is allowed; Germany's historical
    // SPD chair is party-only. Neither party may be promoted merely by its title.
    for (loser, party, named) in [(N::Japan, "jp_jsp", true), (N::Germany, "de_spd", false)] {
        let winner = N::France;
        let mut w = world(winner, loser);
        assert!(!w.nation(loser).nuclear);
        let g = w
            .governments
            .states
            .iter_mut()
            .find(|g| g.nation == loser)
            .unwrap();
        assert_ne!(g.leader(), Some(party));
        assert!(g.support.iter().any(|(id, _)| id == party));
        for (id, value) in &mut g.support {
            *value = if id == party { 1.0 } else { 0.0 };
        }
        front(&mut w, winner, loser, &[]);
        let book = w.party_leadership.as_ref().unwrap();
        let assignments = book.assignments.clone();
        let holders: Vec<_> = book
            .assignments
            .iter()
            .filter(|a| a.nation == loser && a.party == party)
            .flat_map(|a| a.holders.iter())
            .cloned()
            .collect();
        assert!(
            !holders.is_empty(),
            "fixture needs actual sourced incumbents"
        );
        let policy = party_leadership::executive_eligibility::policy().unwrap();
        let authorized: Vec<_> = holders
            .iter()
            .filter(|h| {
                policy.historical_grants.iter().any(|g| {
                    g.nation == loser
                        && g.party == party
                        && g.term == h.term
                        && g.person == h.person
                })
            })
            .collect();
        assert_eq!(authorized.len(), usize::from(named));
        let prior_person = party_leadership::executive_person(&w, loser).map(|p| p.id.clone());
        assert!(prior_person.is_some());
        let auth = w.nation(loser).authoritarianism;
        let offer = propose(&mut w, winner, Terms::Transition);
        assert_eq!(
            party_leadership::executive_person(&w, loser).map(|p| p.id.clone()),
            prior_person
        );
        let mut resumed = canonical(&w);
        for state in [&mut w, &mut resumed] {
            respond(state, loser, offer).unwrap();
            assert!(state.conflict(1).is_none());
            assert_eq!(state.nation(loser).authoritarianism, (auth - 0.25).max(0.0));
            assert_eq!(
                state
                    .governments
                    .states
                    .iter()
                    .find(|g| g.nation == loser)
                    .unwrap()
                    .leader(),
                Some(party)
            );
            let book = state.party_leadership.as_ref().unwrap();
            assert_eq!(book.assignments.len(), assignments.len());
            for (after, before) in book.assignments.iter().zip(&assignments) {
                assert_eq!(
                    (&after.nation, &after.party, &after.component),
                    (&before.nation, &before.party, &before.component)
                );
                assert!(
                    serde_json::to_value(&after.holders).unwrap()
                        == serde_json::to_value(&before.holders).unwrap(),
                    "national election changed incumbent identity/date for {}:{}",
                    after.nation.code(),
                    after.party
                );
                if !before.holders.is_empty() || before.nation != loser || before.party != party {
                    assert!(
                        serde_json::to_value(after).unwrap()
                            == serde_json::to_value(before).unwrap(),
                        "national election changed an occupied or unrelated party assignment"
                    );
                } else {
                    // An empty target-party component is reviewed for a vacancy.
                    // Its reason/date may update; no incumbent or future identity
                    // may be manufactured (the exact holder check above is empty).
                    assert!(after.reason == before.reason || after.reason == "election");
                }
            }
            if named {
                let executive = book.executives.iter().find(|e| e.nation == loser).unwrap();
                assert_eq!(executive.party, party);
                assert_eq!(executive.reason, "election");
                assert_eq!(executive.holder.person, authorized[0].person);
                assert_eq!(executive.holder.term, authorized[0].term);
                assert_eq!(
                    party_leadership::executive_person(state, loser).unwrap().id,
                    authorized[0].person
                );
                assert_ne!(Some(executive.holder.person.clone()), prior_person);
            } else {
                assert!(!book.executives.iter().any(|e| e.nation == loser));
                assert!(
                    party_leadership::executive_person(state, loser).is_none(),
                    "party-only chair must not become an invented named national executive"
                );
            }
            party_leadership::validate_state(state).unwrap();
            assert_once(state, loser, offer);
            canonical(state);
        }
        assert_eq!(save(&w), save(&resumed));
        tick_day(&mut w, &[]);
        tick_day(&mut resumed, &[]);
        assert_eq!(save(&w), save(&resumed));
    }
}

#[test]
fn nuclear_and_last_district_limits_refuse_proposals_and_changed_pending_terms_atomically() {
    let (winner, loser) = (N::France, N::Germany);
    for transition in [false, true] {
        let mut w = world(winner, loser);
        let d = w
            .districts
            .iter()
            .find(|(_, n)| **n == loser)
            .unwrap()
            .0
            .clone();
        front(&mut w, winner, loser, &[d.clone()]);
        let terms = if transition {
            Terms::Transition
        } else {
            Terms::Cede { districts: vec![d] }
        };
        // Counterfactual deterrent tests the rule, not historical German weapons.
        w.nation_mut(loser).nuclear = true;
        let before = save(&w);
        let err = order(
            &mut w,
            winner,
            PeaceOrder::Propose {
                conflict: 1,
                terms: terms.clone(),
            },
        )
        .unwrap_err();
        assert!(err.contains("Nuclear"), "{err}");
        assert_eq!(save(&w), before);
        w.nation_mut(loser).nuclear = false;
        let offer = propose(&mut w, winner, terms);
        w.nation_mut(loser).nuclear = true;
        let mut resumed = canonical(&w);
        let before = save(&resumed);
        let err = respond(&mut resumed, loser, offer).unwrap_err();
        assert!(err.contains("Nuclear"), "{err}");
        assert_eq!(save(&resumed), before);
    }
    // Ceding all five real Singapore districts fits the six-district proposal
    // limit but would include its last district. Do not invent a smaller map.
    let (winner, loser) = (N::Malaysia, N::Singapore);
    let mut w = world(winner, loser);
    let held: Vec<_> = w
        .districts
        .iter()
        .filter(|(_, n)| **n == loser)
        .map(|(d, _)| d.clone())
        .collect();
    assert_eq!(held.len(), 5, "use the actual Singapore district set");
    front(&mut w, winner, loser, &held);
    let before = save(&w);
    let err = order(
        &mut w,
        winner,
        PeaceOrder::Propose {
            conflict: 1,
            terms: Terms::Cede { districts: held },
        },
    )
    .unwrap_err();
    assert!(err.contains("last district"), "{err}");
    assert_eq!(save(&w), before);
}
