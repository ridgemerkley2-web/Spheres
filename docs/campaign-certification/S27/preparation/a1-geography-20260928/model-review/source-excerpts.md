# Read-only source and contract excerpts

Exact numbered text extracted from the Git objects pinned in source-pins.json. Source comments describe the design; historical assertions in archived design notes were not independently researched in this review.

## spheres-sim/src/government.rs:8484-8499 at 6818e4f0d94b01c86d7a9acc4252260947d13504

```text
8484: pub fn army_operating_allowance(income_per_head: f64) -> f64 {
8485:     ARMY_IMPORTED_OPERATING_ALLOWANCE + ARMY_LOCAL_OPERATING_MULTIPLE * income_per_head.max(0.0)
8486: }
8487: 
8488: pub fn army_resource_loyalty_target(resources: f64, income_per_head: f64, exhaustion: f64) -> f64 {
8489:     let funded = (resources / (2.0 * army_operating_allowance(income_per_head))).clamp(0.0, 1.0);
8490:     0.20 + funded * 0.65 - exhaustion * 0.45
8491: }
8492: 
8493: /// Military resources and obedience to civilian office are distinct. These
8494: /// design coefficients model an institution with executive-removal leverage
8495: /// losing confidence in sustained civilian crisis; they are not coup estimates.
8496: /// Known historical leverage replaces the old authoritarianism proxy only in
8497: /// this political channel. Unknown assessments retain the original proxy.
8498: pub const ARMY_CRISIS_CONFIDENCE_WEIGHT: f64 = 0.65;
8499: pub const ARMY_PROGRAMME_VETO_WEIGHT: f64 = 0.45;
```

## spheres-sim/src/government.rs:8501-8515 at 6818e4f0d94b01c86d7a9acc4252260947d13504

```text
8501: fn civilian_army_executive_leverage(w: &WorldState, id: NationId) -> f64 {
8502:     if !w.rules.ideology_blocs || !is_electoral(w, id)
8503:         || state(w, id).is_none_or(|g| !g.pillars.iter().any(|(p, _)| *p == Pillar::Army))
8504:     { return 0.0; }
8505:     crate::army_authority::current_leverage(w, id).unwrap_or_else(||
8506:         ((w.nation(id).authoritarianism - 0.20) / 0.40).clamp(0.0, 1.0))
8507: }
8508: 
8509: fn established_civilian_record(w: &WorldState, id: NationId) -> Option<&PoliticalRecord> {
8510:     let g = state(w, id)?;
8511:     let record = g.political_record.as_ref()?;
8512:     if record.months < 6.0 || !record.performance.is_finite()
8513:         || record.government != format!("party:{}", g.leader()?) { return None; }
8514:     Some(record)
8515: }
```

## spheres-sim/src/government.rs:8556-8594 at 6818e4f0d94b01c86d7a9acc4252260947d13504

```text
8556: /// A paid force can still lose confidence in a persistently failing civilian
8557: /// government. This lowers its gradual loyalty target, never its equipment or
8558: /// strength. A fresh administration, a quiet country, an absent army, and
8559: /// strong civilian control have no such penalty. Existing loyalty smoothing
8560: /// and coup pressure retain their time requirements.
8561: /// Popular consent limits this particular political grievance; it does not
8562: /// erase underfunding, war exhaustion or an army's prospective programme veto.
8563: /// Powell (2012), pp. 1020-1021, distinguishes public legitimacy from military
8564: /// corporate grievances. The linear outside-constituency multiplier here is an
8565: /// explicit game assumption, not a coefficient estimated by that study.
8566: pub fn army_civilian_confidence_penalty(w: &WorldState, id: NationId) -> f64 {
8567:     let leverage = civilian_army_executive_leverage(w, id);
8568:     if leverage <= 0.0 { return 0.0; }
8569:     let Some(record) = established_civilian_record(w, id) else { return 0.0; };
8570:     let discontent = crate::blocs::discontent(w, id);
8571:     if discontent < ELECTORAL_COUP_DISCONTENT { return 0.0; }
8572:     let sustained_crisis = 0.5 * discontent + 0.5 * (-record.performance).clamp(0.0, 1.0);
8573:     ARMY_CRISIS_CONFIDENCE_WEIGHT * leverage * sustained_crisis * (1.0 - civilian_public_mandate(w, id))
8574: }
8575: 
8576: /// Read prospective acceptance of a new radical programme at a completed
8577: /// election. An institution with removal leverage can resist a mandate that
8578: /// threatens its existing programme even when its material bills were paid.
8579: /// This is separate from the gradual crisis confidence already in the pillar.
8580: /// The existing monarchy and authoritarianism guards still apply. The economic
8581: /// discontent guard belongs to the separate material-grievance route: an
8582: /// institutional conflict does not require invented bad inflation or output.
8583: /// No extra voters or foreign backing are invented here.
8584: pub fn army_programme_veto_loyalty(w: &WorldState, id: NationId, winner: &str) -> f64 {
8585:     let loyalty = crate::blocs::effective_army_loyalty(w, id);
8586:     let leverage = civilian_army_executive_leverage(w, id);
8587:     if leverage <= 0.0 || established_civilian_record(w, id).is_none()
8588:         || !matches!(bloc_of(id, winner), Bloc::Communist | Bloc::Islamist)
8589:         || crate::blocs::ruling_bloc(w, id) == Some(bloc_of(id, winner))
8590:     { return loyalty; }
8591:     let mandate = state(w, id).and_then(|g| g.seats.iter().find(|(p, _)| p == winner))
8592:         .map_or(0.0, |(_, share)| share.clamp(0.0, 1.0));
8593:     loyalty - ARMY_PROGRAMME_VETO_WEIGHT * leverage * mandate
8594: }
```

## spheres-sim/src/government.rs:8705-8719 at 6818e4f0d94b01c86d7a9acc4252260947d13504

```text
8705:     for pillar in pillars.iter().copied() {
8706:         let t = match pillar {
8707:             // Generals are bought with budgets and lost in wars that go badly.
8708:             // The floor is deliberately low: the first draft started the army at
8709:             // 0.35 and added the budget on top, which put an entirely unpaid army
8710:             // at 0.357 — a hair above the 0.35 line at which pressure starts to
8711:             // build. Twenty years of a defence budget cut to a tenth of a percent
8712:             // of GDP produced no coup at all, because the model could not express
8713:             // an army that had been abandoned.
8714:             Pillar::Army => match w.rules.ideology_blocs.then(|| army_resources_per_member(w, id)).flatten() {
8715:                 Some(resources) => army_resource_loyalty_target(resources, income_per_head, exhaustion)
8716:                     - army_civilian_confidence_penalty(w, id),
8717:                 None => army_loyalty_target(mil, exhaustion, None, ARMY_PAY_WEIGHT)
8718:                     - army_civilian_confidence_penalty(w, id),
8719:             },
```

## spheres-sim/src/government.rs:8765-8779 at 6818e4f0d94b01c86d7a9acc4252260947d13504

```text
8765: /// Loyalty walks toward its target: slow to buy and quick to lose, like
8766: /// everything else in this game that is worth having.
8767: fn walk_pillars(w: &mut WorldState, id: NationId, targets: Vec<(Pillar, f64)>) {
8768:     let loss_rate = crate::clock::blend(w, 0.10);
8769:     let gain_rate = crate::clock::blend(w, 0.045);
8770:     let g = match state_mut(w, id) {
8771:         Some(g) => g,
8772:         None => return,
8773:     };
8774:     for (pillar, target) in targets {
8775:         if let Some(e) = g.pillars.iter_mut().find(|(p, _)| *p == pillar) {
8776:             let rate = if target < e.1 { loss_rate } else { gain_rate };
8777:             e.1 += (target - e.1) * rate;
8778:             e.1 = e.1.clamp(0.0, 1.0);
8779:         }
```

## spheres-sim/src/government.rs:8832-8845 at 6818e4f0d94b01c86d7a9acc4252260947d13504

```text
8832: pub fn electoral_coup_settled_months(w: &WorldState, id: NationId) -> u32 {
8833:     let Some(g) = state(w, id) else { return 0; };
8834:     if !w.rules.ideology_blocs { return g.months_in_office; }
8835:     if g.awaiting_first_election { return 0; }
8836:     let observed = g.leader().and_then(|party| {
8837:         g.political_record.as_ref().filter(|record|
8838:             record.government == format!("party:{party}")
8839:                 && record.months.is_finite() && record.months >= 0.0)
8840:     });
8841:     observed.map_or(g.months_in_office, |record| {
8842:         // Match the existing office clock's calendar rounding tolerance.
8843:         g.months_in_office.max((record.months + 1e-12).floor() as u32)
8844:     })
8845: }
```

## spheres-sim/src/government.rs:8926-8962 at 6818e4f0d94b01c86d7a9acc4252260947d13504

```text
8926: fn maybe_electoral_coup(w: &mut WorldState, id: NationId) -> bool {
8927:     if !w.rules.ideology_takeover {
8928:         #[cfg(test)]
8929:         a1_observer::record(w, id, "trigger_takeover_disabled");
8930:         return false;
8931:     }
8932:     let (pressure, has_army) = match state(w, id) {
8933:         Some(g) => (g.coup_pressure, g.pillars.iter().any(|(p, _)| *p == Pillar::Army)),
8934:         None => {
8935:             #[cfg(test)]
8936:             a1_observer::record(w, id, "trigger_no_government");
8937:             return false;
8938:         },
8939:     };
8940:     let settled = electoral_coup_settled_months(w, id);
8941:     if !has_army || settled < ELECTORAL_COUP_SETTLED
8942:         || (w.rules.ideology_blocs && state(w, id).is_some_and(|g| g.awaiting_first_election))
8943:     {
8944:         #[cfg(test)]
8945:         a1_observer::record(w, id, "trigger_unsettled_interim_or_no_army");
8946:         return false;
8947:     }
8948:     // Pressure records past grievances; it cannot substitute for the trigger
8949:     // still being live. A paid army or a resolved crisis gets a chance to cool
8950:     // the gauge instead of staging a coup after its reason to move has gone.
8951:     if crate::blocs::effective_army_loyalty(w, id) >= ELECTORAL_COUP_ARMY
8952:         || crate::blocs::discontent(w, id) < ELECTORAL_COUP_DISCONTENT
8953:     {
8954:         #[cfg(test)]
8955:         a1_observer::record(w, id, "trigger_live_conditions_inactive");
8956:         return false;
8957:     }
8958:     if pressure < 1.0 / w.rules.crisis_intensity.max(0.1) {
8959:         #[cfg(test)]
8960:         a1_observer::record(w, id, "trigger_pressure_not_ready");
8961:         return false;
8962:     }
```

## spheres-sim/src/government.rs:9359-9393 at 6818e4f0d94b01c86d7a9acc4252260947d13504

```text
9359: /// A legacy AI's affordable operating allocation for its actual Army pillar.
9360: /// This is a budget priority, not free loyalty: resources still have to be
9361: /// paid through SetMilSpend and the normal fiscal settlement. The same floor
9362: /// is read by fiscal consolidation so the two AIs cannot buy and cut the
9363: /// identical appropriation every month. Explicit/player budgets have their
9364: /// own authority and never use this fallback policy.
9365: pub fn ai_army_funding_floor(w: &WorldState, id: NationId) -> Option<f64> {
9366:     if !w.rules.ideology_blocs || Some(id) == w.player { return None; }
9367:     let n = w.nation_opt(id).filter(|n| n.alive)?;
9368:     if n.on_the_books() || n.annual_budget.is_some() || crate::programs::enrolled(w, id)
9369:         || crate::fiscal_recovery::enabled(w) || !n.gdp.is_finite() || n.gdp <= 0.0
9370:         || !n.population.is_finite() || n.population <= 0.0
9371:     { return None; }
9372:     let g = state(w, id)?;
9373:     if !g.pillars.iter().any(|(p, _)| *p == Pillar::Army) { return None; }
9374:     let personnel = army_personnel_assessment(w, id)?.members;
9375:     if !personnel.is_finite() || personnel <= 0.0 { return None; }
9376:     // Invert the CURRENT Army pillar target, including its existing war and
9377:     // civilian-confidence losses. Funding the raw .40 resource term while
9378:     // ignoring those losses could declare the budget adequate at .22 actual
9379:     // loyalty. The .40 target retains the modest margin above the .35 line;
9380:     // it does not buy away a prospective programme veto or change loyalty
9381:     // directly. Beyond a full resource basket, further spending cannot help.
9382:     let funded = ((0.40 - 0.20 + n.war_exhaustion * 0.45
9383:         + army_civilian_confidence_penalty(w, id)) / 0.65).clamp(0.0, 1.0);
9384:     let resources = funded * 2.0 * army_operating_allowance(n.gdp * 1000.0 / n.population);
9385:     let needed = resources * personnel / (n.gdp * 1_000_000_000.0);
9386:     let conditions = crate::economy::Conditions::of(w, id);
9387:     let terms = crate::economy::growth_terms(n, n.state_invest_gdp, n.interest_rate, &conditions);
9388:     let fiscal = crate::economy::Fiscal::of(n, &terms);
9389:     // Retain all existing civilian expenditure and debt service. No new
9390:     // borrowing or invented treasury is authorized by this response.
9391:     let affordable = (fiscal.balance_gdp + n.mil_spend_gdp).max(0.0);
9392:     Some(needed.min(affordable).min(0.35).max(0.0))
9393: }
```

## spheres-sim/src/government.rs:9395-9427 at 6818e4f0d94b01c86d7a9acc4252260947d13504

```text
9395: /// AI regimes pay their bills. A government that will not spend on the people
9396: /// who could remove it is a government that gets removed, and the AI reaching
9397: /// for the same command the player has is the only way that stays fair.
9398: fn ai_government(w: &mut WorldState) {
9399:     let ids: Vec<NationId> = w
9400:         .nations
9401:         .iter()
9402:         .filter(|n| n.alive && Some(n.id) != w.player)
9403:         .map(|n| n.id)
9404:         .collect();
9405:     for id in ids {
9406:         #[cfg(test)]
9407:         a1_observer::record(w, id, "ai_before_funding");
9408:         // Retain the annual review, with a month-end emergency review when
9409:         // actual loyalty has fallen below the existing .40 funded margin.
9410:         // A confidence loss emerging midyear must not wait until next January
9411:         // when an affordable, useful appropriation is already available.
9412:         // Hysteresis avoids buying tiny changes; the same paid command and
9413:         // fiscal floor still protect civilian spending and debt service.
9414:         if crate::clock::month_end(w)
9415:             && state(w, id).is_some_and(|g|
9416:                 g.loyalty(Pillar::Army) < if w.month == 1 { 0.50 } else { 0.40 })
9417:         {
9418:             if let Some(share) = ai_army_funding_floor(w, id) {
9419:                 if share >= w.nation(id).mil_spend_gdp + 0.001 {
9420:                     let command = crate::Command::SetMilSpend { nation: id, share };
9421:                     if crate::affordable(w, &command) { let _ = crate::apply_command(w, &command); }
9422:                 }
9423:             }
9424:         }
9425:         #[cfg(test)]
9426:         a1_observer::record(w, id, "ai_after_funding");
9427:         if is_electoral(w, id) {
```

## spheres-sim/src/government.rs:12537-12569 at 6818e4f0d94b01c86d7a9acc4252260947d13504

```text
12537:     fn a_new_accountable_party_gets_coup_grace_without_rewriting_the_election_clock() {
12538:         let id = NationId::Pakistan;
12539:         let mut w = twice_reelected_snap_government();
12540:         for month in 13..=18 {
12541:             (w.year, w.month) = add_months(1990, 1, month);
12542:             tick(&mut w);
12543:         }
12544:         for (party, share) in &mut state_mut(&mut w, id).unwrap().support {
12545:             *share = if party == "pk_iji" { 0.90 } else { 0.05 };
12546:         }
12547:         let held = w.nation(id).political_capital;
12548:         crate::apply_command(&mut w, &crate::Command::CallElection { nation: id }).unwrap();
12549:         assert_eq!(w.nation(id).political_capital, held - 25.0);
12550:         assert_eq!(state(&w, id).unwrap().leader(), Some("pk_iji"));
12551:         assert_eq!(electoral_coup_settled_months(&w, id), 0, "outgoing party history cannot settle its successor");
12552:         let election_date = state(&w, id).unwrap().next_election;
12553:         set_live_snap_army_crisis(&mut w);
12554:         w = crate::load(&crate::save(&w)).unwrap();
12555:         for month in 1..=12 {
12556:             let watch = crate::blocs::takeover_readout(&w, id).coup;
12557:             assert!(!watch.open, "new authority lost its genuine first-year grace at month {month}");
12558:             assert!(watch.armed, "the risk gauges remain visibly live during grace");
12559:             assert!(watch.reason.contains("first year"));
12560:             (w.year, w.month) = add_months(1991, 7, month);
12561:             tick(&mut w);
12562:             if month < 12 {
12563:                 assert!(is_electoral(&w, id));
12564:                 assert_eq!(state(&w, id).unwrap().next_election, election_date);
12565:                 assert_eq!(electoral_coup_settled_months(&w, id), month);
12566:             }
12567:         }
12568:         assert!(!is_electoral(&w, id), "the same live crisis remains actionable when the genuine grace expires");
12569:     }
```

## spheres-sim/src/government.rs:12625-12655 at 6818e4f0d94b01c86d7a9acc4252260947d13504

```text
12625:     fn electoral_coup_requires_a_current_hostile_army_and_crisis() {
12626:         let id = NationId::Pakistan;
12627:         let mut staged = world_1990(roads_rules(7));
12628:         {
12629:             let n = staged.nation_mut(id);
12630:             n.inflation = 0.03;
12631:             n.growth_last = 0.01;
12632:             n.war_exhaustion = 0.0;
12633:             n.separatism = 0.0;
12634:             n.stability = 25.0;
12635:             let g = state_mut(&mut staged, id).unwrap();
12636:             g.months_in_office = 24;
12637:             g.coup_pressure = 1.5;
12638:             for (p, loyalty) in &mut g.pillars {
12639:                 *loyalty = if *p == Pillar::Army { 0.20 } else { 0.80 };
12640:             }
12641:         }
12642:         for (army, stability) in [(0.80, 25.0), (ELECTORAL_COUP_ARMY, 25.0), (0.20, 60.0)] {
12643:             let mut recovered = staged.clone();
12644:             recovered.nation_mut(id).stability = stability;
12645:             state_mut(&mut recovered, id).unwrap().pillars.iter_mut()
12646:                 .find(|(p, _)| *p == Pillar::Army).unwrap().1 = army;
12647:             let before = crate::save(&recovered);
12648:             recovered = crate::load(&before).unwrap();
12649:             assert!(!maybe_electoral_coup(&mut recovered, id));
12650:             assert_eq!(crate::save(&recovered), before);
12651:         }
12652:         assert!(maybe_electoral_coup(&mut staged, id), "the still-live trigger must remain reachable");
12653:         assert!(!is_electoral(&staged, id));
12654:         assert_eq!(state(&staged, id).unwrap().coup_pressure, 0.0);
12655:     }
```

## spheres-sim/src/government.rs:13572-13625 at 6818e4f0d94b01c86d7a9acc4252260947d13504

```text
13572:     fn army_confidence_responds_to_sustained_civilian_failure_without_changing_resources() {
13573:         let id = NationId::Pakistan;
13574:         let mut crisis = world_1990(roads_rules(7));
13575:         // This existing causal fixture explicitly exercises the unknown-source
13576:         // authoritarianism proxy; sourced leverage has separate controls below.
13577:         state_mut(&mut crisis, id).unwrap().army_authority = None;
13578:         {
13579:             let n = crisis.nation_mut(id);
13580:             n.gdp = 100.0;
13581:             n.mil_spend_gdp = 0.03;
13582:             n.authoritarianism = 0.55;
13583:             n.stability = 5.0;
13584:             n.inflation = 0.30;
13585:             n.growth_last = -0.10;
13586:             n.war_exhaustion = 0.0;
13587:             n.separatism = 0.0;
13588:         }
13589:         let party = state(&crisis, id).unwrap().leader().unwrap().to_string();
13590:         state_mut(&mut crisis, id).unwrap().political_record = Some(PoliticalRecord {
13591:             government: format!("party:{party}"), months: 24.0, performance: -0.80,
13592:         });
13593:         let resources = army_resources_per_member(&crisis, id).unwrap();
13594:         let penalty = army_civilian_confidence_penalty(&crisis, id);
13595:         assert!(penalty > 0.30, "sustained failure can cost an autonomous army's confidence");
13596:         let mut quiet = crisis.clone();
13597:         {
13598:             let n = quiet.nation_mut(id);
13599:             n.stability = 85.0;
13600:             n.inflation = 0.02;
13601:             n.growth_last = 0.03;
13602:         }
13603:         assert_eq!(army_civilian_confidence_penalty(&quiet, id), 0.0);
13604:         assert_eq!(army_resources_per_member(&quiet, id), Some(resources));
13605:         let mut fresh = crisis.clone();
13606:         state_mut(&mut fresh, id).unwrap().political_record.as_mut().unwrap().months = 5.0;
13607:         assert_eq!(army_civilian_confidence_penalty(&fresh, id), 0.0, "no instant crisis penalty");
13608:         let mut open = crisis.clone();
13609:         open.nation_mut(id).authoritarianism = 0.20;
13610:         assert_eq!(army_civilian_confidence_penalty(&open, id), 0.0);
13611:         let mut off = crisis.clone();
13612:         off.rules.ideology_blocs = false;
13613:         assert_eq!(army_civilian_confidence_penalty(&off, id), 0.0);
13614:         let loaded = crate::load(&crate::save(&crisis)).unwrap();
13615:         assert_eq!(army_civilian_confidence_penalty(&loaded, id), penalty);
13616:         for _ in 0..48 {
13617:             electoral_army_tick(&mut crisis, id);
13618:             electoral_army_tick(&mut quiet, id);
13619:         }
13620:         assert!(state(&crisis, id).unwrap().loyalty(Pillar::Army) < ELECTORAL_COUP_ARMY);
13621:         assert!(state(&crisis, id).unwrap().coup_pressure > 0.0);
13622:         assert!(state(&quiet, id).unwrap().loyalty(Pillar::Army) > 0.60);
13623:         assert_eq!(state(&quiet, id).unwrap().coup_pressure, 0.0);
13624:         assert_eq!(army_resources_per_member(&crisis, id), Some(resources), "confidence does not consume equipment");
13625:     }
```

## spheres-sim/src/government.rs:14450-14488 at 6818e4f0d94b01c86d7a9acc4252260947d13504

```text
14450:     #[test]
14451:     fn ai_army_appropriation_targets_actual_loyalty_without_spending_past_useful_resources() {
14452:         let id = NationId::Pakistan;
14453:         let mut w = world_1990(roads_rules(7));
14454:         // This existing causal fixture explicitly exercises the unknown-source
14455:         // authoritarianism proxy; sourced leverage has separate controls below.
14456:         state_mut(&mut w, id).unwrap().army_authority = None;
14457:         {
14458:             let n = w.nation_mut(id);
14459:             n.gdp = 100.0;
14460:             n.tax_rate = 0.50;
14461:             n.oil_mbd = 0.0;
14462:             n.mil_spend_gdp = 0.001;
14463:             n.state_invest_gdp = 0.03;
14464:             n.political_capital = 100.0;
14465:             // A moderate-autonomy crisis can be funded to the desired margin
14466:             // throughout the declared confidence sensitivity range. The old
14467:             // .55 fixture becomes politically unreachable at weight .90;
14468:             // separate controls below require the AI to respect that limit.
14469:             n.authoritarianism = 0.35;
14470:             n.stability = 5.0;
14471:             n.inflation = 0.30;
14472:             n.growth_last = -0.10;
14473:             n.war_exhaustion = 0.10;
14474:         }
14475:         let party = state(&w, id).unwrap().leader().unwrap().to_string();
14476:         state_mut(&mut w, id).unwrap().political_record = Some(PoliticalRecord {
14477:             government: format!("party:{party}"), months: 24.0, performance: -0.50,
14478:         });
14479:         let fiscal = |world: &WorldState| {
14480:             let n = world.nation(id);
14481:             let terms = crate::economy::growth_terms(n, n.state_invest_gdp, n.interest_rate,
14482:                 &crate::economy::Conditions::of(world, id));
14483:             crate::economy::Fiscal::of(n, &terms)
14484:         };
14485:         let penalty = army_civilian_confidence_penalty(&w, id);
14486:         assert!(penalty > 0.15, "fixture needs a real established civilian crisis");
14487:         assert!(0.85 - w.nation(id).war_exhaustion * 0.45 - penalty > 0.40,
14488:             "positive inversion fixture must be reachable within useful resources");
```

## spheres-sim/src/government.rs:14513-14562 at 6818e4f0d94b01c86d7a9acc4252260947d13504

```text
14513:         let command = crate::Command::SetMilSpend { nation: id, share: floor };
14514:         let price = crate::price_of(&w, &command).unwrap();
14515:         let loyalty = state(&w, id).unwrap().loyalty(Pillar::Army);
14516:         crate::apply_command(&mut w, &command).unwrap();
14517:         assert!((w.nation(id).political_capital - (100.0 - price)).abs() < 1e-9);
14518:         assert_eq!(state(&w, id).unwrap().loyalty(Pillar::Army), loyalty,
14519:             "appropriation does not grant immediate loyalty");
14520:         assert!(fiscal(&w).balance_gdp >= -1e-12);
14521:         let actual = pillar_targets(&w, id, &[Pillar::Army])[0].1;
14522:         assert!(actual > before_target && (actual - 0.40).abs() < 1e-12,
14523:             "the paid appropriation must invert the real target, got {actual}");
14524: 
14525:         // Political opposition alone can exceed what provisioning can repair.
14526:         // No war loss is needed, and throwing extra money beyond a fully
14527:         // equipped force cannot buy away its independent political grievance.
14528:         let mut political = w.clone();
14529:         political.nation_mut(id).authoritarianism = 0.59;
14530:         political.nation_mut(id).war_exhaustion = 0.0;
14531:         state_mut(&mut political, id).unwrap().political_record.as_mut().unwrap().performance = -1.0;
14532:         assert!(is_electoral(&political, id));
14533:         assert!(army_civilian_confidence_penalty(&political, id) > 0.50,
14534:             "even the full material target must remain below the coup line");
14535:         let n = political.nation(id);
14536:         let full_resources = 2.0 * army_operating_allowance(n.gdp * 1000.0 / n.population);
14537:         let useful_share = full_resources * army_personnel_assessment(&political, id).unwrap().members
14538:             / (n.gdp * 1_000_000_000.0);
14539:         let political_floor = ai_army_funding_floor(&political, id).unwrap();
14540:         assert!((political_floor - useful_share).abs() < 1e-12);
14541:         assert!(political_floor < fiscal(&political).balance_gdp + n.mil_spend_gdp,
14542:             "this control is politically limited, not cash limited");
14543:         crate::apply_command(&mut political, &crate::Command::SetMilSpend { nation: id, share: political_floor }).unwrap();
14544:         assert!(pillar_targets(&political, id, &[Pillar::Army])[0].1 < 0.35);
14545:         assert!((army_resources_per_member(&political, id).unwrap() - full_resources).abs() < 1e-9);
14546:         assert_eq!(ai_army_funding_floor(&political, id), Some(political_floor));
14547: 
14548:         let mut exhausted = w.clone();
14549:         exhausted.nation_mut(id).war_exhaustion = 1.0;
14550:         let n = exhausted.nation(id);
14551:         let full_basket = 2.0 * army_operating_allowance(n.gdp * 1000.0 / n.population);
14552:         let full_share = full_basket * army_personnel_assessment(&exhausted, id).unwrap().members
14553:             / (n.gdp * 1_000_000_000.0);
14554:         assert!(full_share < fiscal(&exhausted).balance_gdp + n.mil_spend_gdp);
14555:         let capped = ai_army_funding_floor(&exhausted, id).unwrap();
14556:         assert!((capped - full_share).abs() < 1e-12);
14557:         crate::apply_command(&mut exhausted, &crate::Command::SetMilSpend { nation: id, share: capped }).unwrap();
14558:         let capped_target = pillar_targets(&exhausted, id, &[Pillar::Army])[0].1;
14559:         assert!(capped_target < 0.35, "money cannot erase overwhelming war and political losses");
14560:         assert!((army_resources_per_member(&exhausted, id).unwrap() - full_basket).abs() < 1e-9);
14561:         assert_eq!(ai_army_funding_floor(&exhausted, id), Some(capped),
14562:             "the AI does not chase an unreachable margin past the full basket");
```

## spheres-sim/src/politics.rs:145-185 at 6818e4f0d94b01c86d7a9acc4252260947d13504

```text
145:         // the inflation an economy settles on when demand is at potential. The
146:         // half-point disagreement closed a loop with a fixed point at
147:         // g* = +0.00108, and every nation on the board collected it as permanent
148:         // free growth. See `economy::INFLATION_ANCHOR` for the algebra and for
149:         // why it is this half that moved.
150:         let target = crate::economy::INFLATION_ANCHOR;
151:         let desired = (0.025 + n.inflation + (n.inflation - target) * 0.6).clamp(0.0, 0.45);
152:         n.interest_rate += (desired - n.interest_rate) * bank_rate;
153:     }
154: 
155:     // ---- Fiscal AI: consolidate when debt runs hot ----
156:     for id in &ids {
157:         // An explicit plan or open books has its own fiscal authority. An
158:         // enrolled opponent uses priced commands through economic_ai; legacy
159:         // aggregate writes must not bypass either owner's plan or price.
160:         if Some(*id) == w.player || w.nation(*id).on_the_books()
161:             || w.nation(*id).annual_budget.is_some() || crate::programs::enrolled(w, *id)
162:             || crate::fiscal_recovery::enabled(w) {
163:             continue;
164:         }
165:         let army_floor = crate::government::ai_army_funding_floor(w, *id);
166:         let n = w.nation_mut(*id);
167:         if n.debt_gdp > 0.85 {
168:             // Bounds limit the direction of this policy; they are not targets
169:             // that authorize tax cuts or spending increases during austerity.
170:             n.tax_rate = (n.tax_rate + 0.002 * dt).min(0.55_f64.max(n.tax_rate));
171:             // A funded Army's operating appropriation is a shared priority
172:             // with government AI. This only limits a cut; it never raises
173:             // spending implicitly. The .01 legacy floor also only limits cuts.
174:             let floor = army_floor.map_or(0.01, |v| v.max(0.01)).min(n.mil_spend_gdp);
175:             n.mil_spend_gdp = (n.mil_spend_gdp * fiscal_cut).max(floor);
176:             n.state_invest_gdp = (n.state_invest_gdp * fiscal_cut)
177:                 .max(0.02_f64.min(n.state_invest_gdp));
178:         } else if n.debt_gdp < 0.3 {
179:             if n.tax_rate > 0.30 {
180:                 n.tax_rate = (n.tax_rate - 0.001 * dt).max(0.30);
181:             }
182:             // CONSOLIDATION IS REVERSIBLE, and it was not. The branch above
183:             // cuts public investment 0.5% a month while debt runs hot and
184:             // nothing ever gave it back, so a nation that consolidated its way
185:             // out of a debt crisis carried the cut for the rest of the run. That
```

## spheres-sim/tests/bloc_census.rs:985-993 at 6818e4f0d94b01c86d7a9acc4252260947d13504

```text
985: fn a1_coups_against_elected_governments_are_spread_not_three_micro_polities() {
986:     let rows = census(12, MONTHS);
987:     let coups: Vec<f64> = rows.iter().map(|r| r.coup_el as f64).collect();
988:     let m = median(&coups);
989:     let top3 = median(&rows.iter().map(|r| r.top3_share).collect::<Vec<_>>());
990:     eprintln!("A1: median coups {m} top-3 share median {top3:.2}\n{}", seed_summary(&rows));
991:     assert!((4.0..=14.0).contains(&m), "median coups against elected governments {m}");
992:     assert!(top3 < 0.5, "three nations hold {top3:.2} of a seed's coups");
993: }
```

## SPEC.md:82-84 at 6818e4f0d94b01c86d7a9acc4252260947d13504

```text
82: - **The political arm — movements, foreign backing, the five levers, the four roads, succession and mortality (built 2026-09-06, stages S3-S4 of the same design, on Ridge's approval of all six decisions at their recommendation; shipped after three skeptic passes and five repairs, recorded in BUGS R-1..R-10).** Everything below is behind `rules.ideology_blocs` (the lens; `play_rules` turns it on in the browser) and the roads behind `rules.ideology_takeover`, which nothing in the tree turns on: the browser reads every road `calibration pending` with live gauges until the S5 census calibrates them. Every coefficient is INVENTED, labelled at its definition, filed in BUGS with what would calibrate it. **Movements** (`government::drift_movements`, after `regime_tick` in the non-electoral branch, the same shape as `drift_support` so the two cannot disagree in kind): record = 0.35 − (0.90·prices + 0.90·growth + 1.10·war) off `pains()`; the ruling bloc moves by record·0.006·dt, the transfer bounded 0.015·dt; the outflow goes to present non-ruling blocs in proportion to the MEAN of `appeal()` over the bloc's member families in the dormant table (representatives where none: Western = Liberal/Social-Democratic/Conservative, Non-Aligned = Big-Tent), never the max; then a blend(0.005) reversion toward the flat seed, floor 0.002, normalise; no RNG. The liberalisation seam reseeds party support from the movements when the branch schedules a regime's first free elections (S_B × table weight within the bloc; a bloc with a movement and no party is dropped and named in the headline). A non-ruling movement crossing 0.30 upward prints "The {bloc} movement in {X} passes a third of the country" once, latched, cleared under 0.25; the latch is seeded closed at the 1990 seed, on a programme, on any break and on an uprising. **Foreign backing** (`CovertOp::BackBloc(Bloc)`, parse `back:<bloc>`, the same 5 PC REFUSABLE command, sovereignty gate, service charge gdp·0.0008, `success_p` / `expose_p` (now `statecraft::covert_odds`, the one place, quoted on the dossier as "works p% / exposed q%"), heat +0.18 and two `chance` rolls in the same order — no third draw): FIXED +0.06 per clean op, per-sponsor cap 0.12, per-bloc cap 0.25, in `Statecraft.backing` sorted; refused when the bloc is the target's ruling bloc ("You cannot back a government covertly — send aid"); decays 0.006·dt a month, retained while > 0, gated on the lens; exposure halves every entry of that sponsor in that target and taints the bloc −0.02 of support (electoral: from its parties by size; regime: from the movement), the headline naming sponsor and bloc. Patronage gravity is a VIEW: each live `AidFlow` counts 0.10·min(1, infusion_share/0.10) of the patron's ruling bloc toward F_B inside the 0.25 cap. Influence I_B = S_B + F_B; F_B never leaks into support; effective army loyalty = loyalty(Army) − F_Nationalist. D3: `NationDef.ideological_sponsor` on Saudi Arabia, Iran, Pakistan (Islamist), Libya (Nationalist), Cuba (Communist), read only by the gated AI covert arm (`politics.rs`: a present non-ruling bloc at I_B ≥ 0.15 that is the sponsor's own colour or sits in a rival's client, the draw after the choice), never by `patrons()`. The Security Crackdown gains one gated arm: F_B halved for every non-ruling bloc. **The five levers** (`Command::SuspendConstitution` 40 PC, `BanParty` 18, `LegalizeParty` 12, `DeclareProgramme` 35, `ConveneRoundTable` 30, all REFUSABLE, refused before any state read — "This world does not model ideological movements" with the lens off — each built as refusal → plan → arm → effects off ONE plan struct, so the card and the world take the same clamped numbers, rule 8): a suspension needs an electoral polity, stability < 45 or government seats < 0.5, authoritarianism ≥ 0.20, and writes authoritarianism max(auth + 0.30, 0.65), the incumbent's colour as `regime_bloc`, the cabinet kept dormant, movements seeded from the bloc sums with the ruler +0.10, stability −6, −10 with every democracy (auth < 0.30); a ban needs authoritarianism ≥ 0.30 or a pariah, never the coalition leader, and takes seats not voters (`seats_from_legal`, a gated wrapper over an untouched `seats_from`; support kept and counted in I), authoritarianism +0.04, stability −3 when the party holds ≥ 0.15, −4 with democracies — and NO cabinet arm (R-4); legalising reverses it at −0.02 / +3; a programme needs a regime with the bloc's movement ≥ 0.25 or the strongest pillar's colour, and writes the colour, stability −5, Clergy +0.15 if Islamist else −0.10, Army +0.10 if Nationalist, Party −0.15 leaving Communist, −15 with every patron of the old colour and +10 with every patron of the new, the movement +0.10 then normalise; a round table needs discontent ≥ 0.40 and the largest non-ruling movement ≥ 0.25, and writes authoritarianism min(auth − 0.25, 0.59), stability +8, every ban lifted, first free elections in six months. The AI (`government::ai_lever`, pure) asks for a suspension when electoral, stability < 30, auth ≥ 0.25, 55 PC; a ban on the largest party of the strongest non-ruling non-Western bloc at I ≥ 0.35, auth ≥ 0.40, 60 PC; a programme toward the strongest pillar's colour when the ruling movement < 0.30 at 70 PC; a round table when non-electoral, discontent ≥ 0.50, the largest movement ≥ 0.35, the armed mean loyalty < 0.50, 60 PC — and acts on the deck's ONE 0.02 monthly draw in `stratagems::ai_stratagems`, the lever before the card, one decision a month (R-3); the government module draws no RNG in any state. **The roads** (all behind `ideology_takeover`; D = discontent, R = the ruling bloc, W = argmax of I_B over the present non-ruling winnable blocs, ties in enum order, never Regionalist-only, never Western without a party table; crisis = `crisis_intensity.max(0.1)`). Route 2, a coup against an elected government: electoral polities whose state holds an Army walk the Army and Security lines of `pillar_targets` (regime_tick's formulas, factored) and accrue pressure 0.30·(2·(0.35 − eff_army) + (D − 0.25))·dt while eff_army < 0.35 and D ≥ 0.25, cap 1.5, else −0.03·dt; at pressure ≥ 1/crisis after 12 months in office, before the fragile branch, `regime_break` — `maybe_coup`'s block, ONE function shared by the regime's own coup, route 2 and the annulment: stability −16 floor 5, gdp ×0.97, political capital reseated, the mover 0.90 and the rest 0.72, the office clock to zero — with authoritarianism max(auth + 0.25, 0.65), the Nationalist colour, the cabinet dormant, the movements seeded from the party bloc sums; "COUP IN {NATION}: {army} removes the elected government." The regime's own coup keeps its colour under the lens alone and rules in the mover's colour only under the roads, the deposed movement latched (R-2). The annulment, inside `hold_election` between the seats and the formation: a live Army, a Communist or Islamist would-be leader, authoritarianism ≥ 0.35, D ≥ 0.25, no court → the break with every party of the winner's bloc banned, "COUP IN X: the army annuls the election {party} won." Route 3, the uprising: the `politics.rs` collapse chain gains an arm at the same site with the same `monthly_chance(0.10·crisis)` draw (the off world's code and draw untouched), firing on stability < 12 or [D ≥ 0.45 and I_W ≥ 0.45 and coercion fails: regime — weakest armed loyalty < 0.35 or mean < 0.35 (CALIBRATED 2026-09-06, S5, from the design's 0.50 / 0.55: both now quote the pillar model's unpaid line, `regime_tick`'s pressure and `ai_government`'s purchase — anchor A3, BUGS S5-2; before → after at N=60, Communist non-ballot takeovers 60/60 seeds at 1/3/4 a seed → 58/60 at 0/2/3, Iraq's Nationalist/Non-Aligned flip-flop gone); electoral — only if W is non-Western or authoritarianism ≥ 0.40]; the crown goes to W only where the MOVEMENT armed the collapse (`blocs::uprising_armed` at the firing site — CALIBRATED 2026-09-06, S5, anchor A3, BUGS S5-2: a stability < 12 collapse the movement did not arm runs the pre-arm collapse block verbatim, its random authoritarianism shift included, "the old regime falls"; Peru 60/60 → 0, Georgia 60/60 → 0, A3 per seed 3/5/6 → 1/3/4); stability 45, gdp ×0.93, no random auth shift under the switch, the winner's authoritarianism (Western min(auth, 0.40), Communist max(auth, 0.80), Nationalist max(auth, 0.72), Islamist max(auth, 0.80), Non-Aligned max(auth, 0.65), clamp 0.05..0.95), the cabinet cleared, `regime_bloc` = W, W +0.15 then normalise, the home pillar 0.80 and the rest 0.55; "Revolution in {}: the {bloc} movement takes power." Route 4, the round table: non-electoral, I_Western ≥ 0.40, stability 30..70, loyalty(Party) < 0.55 (the weakest armed where there is no Party) → authoritarianism −0.01·dt a month to a floor of 0.55, the only authoritarianism drift; crossing 0.60 makes the state electoral and the branch schedules first free elections. The foreign payoff on any non-ballot takeover (`statecraft::takeover_payoff`, one plan): every sponsor holding ≥ 0.10 behind the winner +40 with the new regime and −25 with the target's previous top patron, exposed on the spot with the existing costs at covert heat ≥ 0.50; democracies −8 with a new Communist/Nationalist/Islamist regime and +8 with a Western one; patrons whose own colour was the loser's −10. **Succession** (D2, `government::Succession` / `seat_office`, on every change of government, gated on the lens): the leader row becomes `Office::Emergent {described, party, pillar, since}` — the same leading party keeps the transcribed person; a new leading party seats "the {party} government"; a coup or a Nationalist/Non-Aligned takeover seats the polity's real pillar name; an Islamist/Communist takeover the bloc's largest party or the Clergy/Party pillar; a reached `must_leave_by` "a new {party} president"; a monarch or party-state leader removed by a Party coup or a programme the transcribed heir ONCE then "the ruling house"; a removed incumbent never returns by name; NO NAME is written for a date after 1 January 1990 but the heir. A vote never unseats a transcribed holder tied to a pillar — a crown, a court, a party-state chief — the chamber's winner is the government of the day beside the row (R-1). **Mortality** (D1, `politics::mortality`, last in `SYSTEMS`, gated on the lens): one draw per living transcribed leader per game year in sorted NationId order on the one RNG, annual hazard q(age) = 0.015·2^((age − 65)/7) — INVENTED, a Gompertz curve fitted by eye to 1990 male life tables — "{name} dies in office.", the office seated by the leader's own party or pillar. **D4** (Nepal May 1991, Haiti December 1990) is transcribed beside the table as `D4_POLITIES` and NOT wired: wiring it moves the 1990 start hash with the switch off (P-8, S4-10), Ridge's call. **The web** (`spheres-web`): the government screen serves the five levers with `price_of`, the refusal prose and the effects list off the plan; the covert card on the TARGET's dossier (`GET /api/covert?nation=`) carries the three operations and "Back the {bloc} movement" per present bloc with "works p% / exposed q%" from `covert_odds`; the bar hatches each bloc's F_B "backed from abroad", the sponsor named only after exposure (the server never sends an unexposed one); the takeover watch reads every road with its served reason; the political stems are filed and promoted as event cards; the header chip pulses and the dock names the road when any gauge passes half. Nothing is recomputed in JavaScript; `ideology_takeover` stays off in the browser.
83: - **The political arm — S5, the census, the first pins and the switch decision (2026-09-06, the pin-and-wire pass on `feat/ideology-census`).** `spheres-sim/tests/bloc_census.rs` measures the design's anchors A1-A10 per seed (both switches on, monthly, 252 months or `SPHERES_CENSUS_MONTHS`, seeds from `SPHERES_CENSUS_SEEDS`, no player), and its bars sit beside the scan reading the same code. The N=200 reading of this tree, per seed min/med/max: A1 coups against elected governments 4/6/9 (mean 6.35, sd 0.85) — inside 4..14 and degenerate, Sao Tome 545, Philippines 460, Comoros 200 of 1269; A2 Islamist takeover without a ballot by 2000 0/200 seeds; A3 Communist takeover without a ballot 197/200 seeds, 0/2/4 a seed (Belarus 181, Cambodia 142, Ukraine 91); A4 1990 regimes electoral by end-1996 1/2/3; A5 ex-communist return by ballot 0/200; A6 route events in 1990 democracies 0 in 200/200 (0 in 60/60 at 420 months, Argentina's pre-existing collapse beside in 59/60); A7 ballot bloc flips / non-ballot takeovers 0.16 median; A8 8 of 8 big regimes kept in 200/200; A9 3718/3900 = 0.953 of coups in nations under $8,000 a head; A10 discontent lead 0.27/0.33/0.38. FOUR BARS ARE PINNED, each with its seed count derived from that sample (iron rule 7; the variance, the n and a power statement beside each): A6 as an invariant over 420 months on 12 seeds (a power budget: sees a road touching one democracy in five seeds at 0.93); A8 on 40 seeds, 24 must keep six of eight (sees the eight's survival falling to a coin toss at 0.87; decorative against one regime falling — said so); A9 on 12 seeds pooled ≥ 0.80 (sees the rich-nation coup share rising ×4, not ×2; NOT red with route 2 thrown open, 0.878 — it guards the transcription of which polities carry an Army, recorded as decorative against the road constants); A10 on 12 seeds, median ≥ 0.25 (sees the lead falling by 0.10 at 0.92). Each was watched red against a mutation of the lines it guards, the readings in the file. SIX ARE WRITTEN AND `#[ignore]`d at their measured reading, never widened: A1's concentration arm (top-3 share median 1.00), A2 (0/200), A3 (0.985 against ≤ 0.10), A4 (median 2 against 15..40), A5 (0/200), A7 (0.16 against ≥ 3); the reasons are BUGS S5-5..S5-8 and S6-3. THE SWITCH DECISION: `ideology_takeover` stays OFF in browser play (`play_rules`) and the watch keeps reading `calibration pending`, because five anchors and one arm are outside their bands (BUGS S6-5 carries the list and the distances); it turns on only when every anchor is inside. D4 (above) was wired under the lens switch by that pass and REVERTED by the ship pass (BUGS S7-3): `D4_POLITIES` stays beside the table, unread, Ridge's call. The ship pass also repaired A6's attribution — a route event's uprising is the crowned headline, the firing site's own clause, and not the pre-tick flag (S7-1) — recorded that two of A5's six named returns are unreachable by transcription (S7-5), and re-read the census and every watched red on the tree it ships (S7-2, S7-4).
84: - **The political arm — the history pass and the calibration pass (2026-09-06, `feat/ideology-history` off 2e164ae, on Ridge's determinations, quoted: "Make determinations for these and continue. I want it based on historical data.").** Every constant below carries its basis at its definition; the default path is byte-identical throughout (both golden actuals, both market-OFF digests, the government module draws no RNG), and everything is behind `rules.ideology_blocs`. **M1, presence through backing** (`blocs::PRESENCE_BACKING` = 0.05, the ruling's number): a bloc is present in a regime when foreign backing behind it (`blocs::backing`, stock plus gravity) is at or over the line, so `bloc_present` = `bloc_in_table || bloc_backed`; a backed non-Western bloc can win an uprising; the AI sponsor (`politics::ai_back_bloc_choice`) backs its OWN bloc in a target where that bloc is absent from the TABLE when the target is hostile (`HOSTILE_TARGET` = −30, `best_covert_target`'s line, named) or a rival's client — the first clean operation creates the presence (anchor: the Peshawar parties, the NIF, the contras). The ideological sponsors' block acts once a calendar month whenever its choice stands and the channel has cooled to `SPONSOR_PROGRAMME_HEAT` — DERIVED, 0.18 − 0.012·(`BACKING_STEP`/`BACKING_DECAY`) = 0.06, the heat left when the last operation's money has run out (Operation Cyclone was funded every fiscal year 1980-92) — and draws nothing itself; the patron arm keeps its 0.022 draw (BUGS H-1). **R2, the annulment needs a HOSTILE army**: `annulment_check` returns `None` where `effective_army_loyalty >= ELECTORAL_COUP_ARMY` (0.35, no new constant; Algeria's ANP of January 1992, Jordan's never-annulled 1989 chamber, Turkey's 1997 memorandum); the Jordan assertion of the Algeria test re-expressed to it (H-2). **M2, pay per soldier in the Army target** (`government::army_pay_ratio` = `mil_spend_gdp · population / personnel_1990`, the ruled (milex/personnel)/(gdp/population) with the output cancelled; `army_loyalty_target(mil, exhaustion, pay_ratio, weight)`: `None` is the share arm's literal bit for bit, `Some(r)` blends `0.20 + 0.65·((1 − w)·share + w·pay) − 0.45·exhaustion` with pay the ratio's clamped position between `ARMY_PAY_UNPAID_AT` 1.0 — the ruling's "national average income" — and `ARMY_PAY_FULL_AT` 2.0, INVENTED and labelled; `ARMY_PAY_WEIGHT_RULED` 0.80 derived from the pillar line, `ARMY_PAY_WEIGHT` 0.0 SIZED BY MEASUREMENT: on the fetched 1990 figures the ruled ratio reads Sudan the best-paid army of the nineteen and Algeria's ANP as paid at any positive weight, because dividing by a poor country's income inverts Londregan and Poole's gradient; Powell 2012 and Londregan and Poole 1990 cited at the function; the arm applies only under the lens, the one place the lens changes a target; BUGS H-3 tables Powell's own variable for Ridge). **R1, D4 wired** (`government::polity_in` serves `D4_POLITIES` — Nepal May 1991, Haiti December 1990 — under the lens and `POLITIES` otherwise; the inertness test's government clause narrowed for those two nations as ruled, H-4). **R3(d), the successor flag** (`PartySpec.successor_of_ruling_party`, 24 sourced rows, `every_successor_flag_has_a_source`; A5 re-expressed to it). **R3(e), 1990 armed forces** (`military.personnel_1990` in 125 files, `milex_1990_usd_bn` in 118, World Bank MS.MIL.TOTL.P1 / MS.MIL.XPND.CD with the fallback year named, else the CIA Factbook; 12 nations refused to the share-only line, H-5). R3(a), (b), (c) — the successors' 1990 tables, Algeria's 1987 table, the seven regimes' families — are transcribed under `docs/political-arm/*-pending*` and NOT landed: each moves a golden or a pinned vector, three re-pin questions for Ridge (H-5). **THE CALIBRATION PASS (BUGS H-6) MOVED NO CONSTANT.** The census on this tree, N=200, 252 months, per seed min/med/max: A1 coups against elected governments 5/6/9 (top-3 share 1.00; Sao Tome 541, Philippines 447, Comoros 200 of 1252), annulments 0/1/1 (Algeria 105/200 alone — Belarus's and Ukraine's 200/200 are gone with R2); A2 1/200 by 2000 (the Islamist bloc is present in Kabul from month 0 and stops at I <= 0.383 against the 0.45 literal, coercion never failing at a 15% defence share); A3 117/200 = 0.585, Cambodia alone (2e164ae 197/200); A4 1/2/3; A5 0/200; A6 0 in 200/200 and 0 in 60/60 at 420 months; A7 0.148; A8 8/8 in 200/200; A9 3472/3472 = 1.000; A10 0.26/0.28/0.32. Every permitted move for an out-of-band anchor is either pinned by an existing test — the flat seed and the drift constants by `a_regime_s_movements_move_with_the_pains_and_revert` (0.5988, 0.65..0.80), the AI's round-table lines by `the_ai_takes_each_lever_only_under_its_thresholds`, the road lines by `every_road_reads_closed_while_takeover_is_off` — or short of the band by arithmetic on those pins; the historical bases fetched for the moves drafted (Benin 1989-90: "riots broke out when the regime did not have enough money to pay its army"; Zambia 1990-91: Kaunda's 24% against Chiluba's 75%; Cambodia 5 July 1997) are quoted in H-6 for the re-expression that would admit them. The four pinned bars' n and power are re-derived from this sample (A9 pooled 1.000, sd 0, sees a quarter of coups moving to rich nations and not a fifth; A10 0.279, sd 0.0103, n = 0.68, sees a seventh of the lead going and not a fourteenth) and re-watched red on a fresh build; the six ignored bars carry this tree's readings. **`ideology_takeover` stays OFF** — out: A1's concentration (1.00 against < 0.5), A2 (1/200 against a majority), A3 (0.585 against <= 0.10), A4 (2 against 15..40), A5 (0/200), A7 (0.148 against >= 3); inside: A1's band, A2's annulment sub-anchor, A6, A8, A9, A10, A12, A13.
```

## BUGS.md:3085-3095 at 6818e4f0d94b01c86d7a9acc4252260947d13504

```text
3085: ## Filed 2026-09-06 by the S4 shipper (the roads, succession, mortality; branch `feat/ideology-roads`)
3086: 
3087: Stage S4 of the approved design, eight commits on part two's 712de08: `regime_break` shared (d8df9b6); route 2 (79c2ea8); the annulment (9a6885b); route 3 (0386136); route 4 (6f4a6d8); the foreign payoff (e218372); succession (bea4513); mortality (ca41545). Every road is behind `rules.ideology_takeover`, which nothing in the tree turns on — the browser keeps it off and every road reads `calibration pending` there — and mortality and succession are behind `rules.ideology_blocs`. Every coefficient below is INVENTED, labelled at its definition, and filed with what would calibrate it. The measured values are this run's.
3088: 
3089: ### S4-1 — route 2's electoral pressure: `0.30·(2·(0.35 − eff) + (D − 0.25))·dt`, cap 1.5, cooling 0.03·dt, twelve months settled
3090: 
3091: `spheres-sim/src/government.rs`, `electoral_army_tick` / `maybe_electoral_coup`, `ELECTORAL_COUP_*`. Measured on Pakistan with its budget at 0.001 and stability held at 35 (its transcribed 52 reads discontent 0.147, under the 0.25 line — at Pakistan's own numbers the road never arms): the Pakistan Army crosses 0.35 in month 11 and removes the elected government in month 30, not the design's "about eighteen"; the bar is 12..=36 with the measurement recorded beside it. **What would calibrate it:** Pakistan's own October 1999 (a paid army, a government at 0.60 of the chamber, no pressure of this shape at all — the coup was the army's, not the street's), Thailand's February 1991, Haiti's September 1991 (eight months after the vote: faster than any setting of this rate reaches from 0.65), and Turkey's 1980 (two years of street war before the generals moved). The rate looks slow by Haiti and the 12-month honeymoon short by Turkey.
3092: 
3093: ### S4-2 — the annulment's lines: authoritarianism `0.35`, discontent `0.25`, Communist or Islamist only
3094: 
3095: `government::annulment_check`. Algeria at its transcribed numbers (stability 40, inflation 16.7%, held — left to the model with Algiers in the player's seat, inflation falls to 0.1% and discontent to 0.219 by October 1990, under the line) annuls the December 1991 vote the FIS won; Jordan under its court at 0.55 does not, and at 0.38 does. **What would calibrate them:** the one clean case is Algeria's own January 1992 (FIS 47.3% of the first round, annulled between rounds: the model annuls AFTER the seats and bans the bloc, which is the right order of events and the wrong month); Burma's May 1990 (the NLD's 80% of seats annulled by a junta that never let it sit — a Western winner, which the line excludes); Turkey's 1997 "post-modern coup" against the Welfare Party (Islamist, in office a year, forced out rather than annulled). Burma says the bloc restriction is too narrow.
```

## docs/political-arm/2026-09-22-calibration-repairs.md:8-117 at 6818e4f0d94b01c86d7a9acc4252260947d13504

```text
8: ## Defence resources, local costs, and political confidence
9: 
10: The old defence-share calculation made a small force with a small national
11: budget permanently hostile, regardless of what that force could actually buy.
12: An intermediate universal dollar threshold had the opposite data problem:
13: inexpensive conscript forces in poor countries were treated as chronically
14: unpaid. Resources now use an explicit operating-cost model:
15: 
16: | Quantity | Definition |
17: | --- | --- |
18: | Annual defence resources per recorded force member | `defence GDP share × GDP in billions × 1e9 / personnel` |
19: | Annual operating basket per member | `$2,500 + 1.5 × local GDP per head`, in model 1990 dollars |
20: | Material loyalty target | `.20 + .65 × clamp(resources / (2 × basket), 0, 1) − .45 × war exhaustion` |
21: | Local-cost proxy | `GDP in billions × 1000 / population in millions` |
22: 
23: The fixed basket component represents imported equipment and maintenance; the
24: local component represents personnel and locally purchased support. A second
25: basket represents full provisioning, reserves, and modernisation. **The dollar
26: amount, multipliers, and loyalty endpoints are game-design assumptions, not
27: historical salaries or estimates from an empirical regression.** Defence
28: expenditure includes personnel, operations, and equipment; it cannot be read
29: as a soldier's wage. [World Bank expenditure indicator definition](https://databank.worldbank.org/metadataglossary/world-development-indicators/series/MS.MIL.XPND.GD.ZS).
30: The denominator is the existing total armed-forces personnel series, including
31: qualifying paramilitaries; it is not a land-Army headcount. The Army pillar
32: represents the conventional military institution, while this broader denominator
33: remains a resource proxy. Readouts should say **defence resources per recorded
34: force member**, retain `Sourced1990` versus `ModelEstimate`, and never label
35: that quantity salary or Army pay. [Personnel indicator definition](https://databank.worldbank.org/metadataglossary/world-development-indicators/series/MS.MIL.TOTL.TF.ZS).
36: The direction of the resource effect follows expenditure per member as an
37: organisational-resource proxy in [Powell (2012), pp. 1026–1029](https://jonathanmpowell.com/wp-content/uploads/2025/10/powell-2012jcr-determinants-of-the-attempting-and-outcome-of-coups-detat.pdf).
38: That paper does not supply this game's cost basket or causal loyalty formula.
39: 
40: The legacy AI can buy an annual military appropriation through the existing,
41: priced `SetMilSpend` action. Iteration 10 corrects its inverse to target `.40`
42: in the actual current Army pillar, including the existing war and civilian
43: confidence losses. The required funded fraction is
44: `clamp((.40 − .20 + .45 × exhaustion + confidence penalty) / .65, 0, 1)`.
45: It stops at full provisioning (two operating baskets per member); an unreachable political margin cannot
46: justify spending beyond useful resources. The allocation is limited by actual
47: revenue after civilian expenditure and debt service, and does not authorise
48: additional deficit borrowing. The separate prospective programme veto is not
49: priced into this current-target calculation. The fiscal
50: consolidation routine respects that same affordable allocation. The annual
51: January review remains. After the candidate-11 diagnostic, an emergency
52: calendar-month-end review also responds when actual Army loyalty falls below
53: the existing `.40` desired margin. The unchanged `.001` minimum spending
54: increase avoids repeated purchases of tiny changes. Player budgets, open fiscal books, programme budgets, and the
55: fiscal-recovery mode retain their own authority.
56: 
57: The [midyear response trace](../campaign-certification/verification/evidence/2026-09-22-completion/army-funding/emergency-review.md)
58: records a civilian confidence shortfall emerging after January while a useful
59: appropriation was already affordable. The emergency review uses that current
60: need and the ordinary paid action; it grants no pressure reset or direct
61: loyalty and does not forecast future political losses. Its exact extracted
62: regression passes; restoring annual-only timing fails. Its compiled regression
63: passes in iterations 12 and 13. The combined campaign readings below still
64: fail A1 concentration; this correctness repair is not full certification.
65: 
66: The [thirty-event diagnostic and regression proof](../campaign-certification/verification/evidence/2026-09-22-completion/army-funding/README.md)
67: retain the distinction between genuine fiscal shortages and later affordable
68: targets missed by the old inverse. The new test reaches `.40` through a paid
69: command with no immediate loyalty gift; insufficient funds remain capped and
70: overwhelming losses remain below `.35` even at full resources. Restoring the
71: old inverse in an external copy makes that regression fail. The new inverse
72: and successor-reference tests pass in the iteration-10 focused run; full
73: calibration and the final whole-workspace checks remain pending.
74: 
75: Resources do not guarantee political obedience. An actual Army pillar in an
76: electoral state can lose confidence after at least six months of the same
77: government's recorded performance. The confidence penalty is:
78: 
79: ```text
80: autonomy = clamp((authoritarianism − .20) / .40, 0, 1)
81: crisis = .5 × current discontent + .5 × clamp(−remembered performance, 0, 1)
82: target penalty = .65 × autonomy × crisis × (1 − public mandate)
83: ```
84: 
85: This channel is zero below `.25` discontent, without the requisite performance
86: record, without a real Army pillar, outside electoral rule, or with the lens
87: off. Civilian control at authoritarianism `.20` or below blocks it. It changes
88: the gradual loyalty target, not equipment, military strength, or the fiscal
89: balance. The existing loyalty lag and accumulated coup-pressure rules still
90: apply. These coefficients are explicit design assumptions.
91: 
92: The iteration-9 refinement measures a public mandate only for a completed,
93: unrestricted elected coalition. It distributes each bloc's national constituency
94: among the parties that carry it, then counts the known, represented governing
95: parties. A single offered party carrying 20% of the country supplies a 20%
96: buffer even if its conditional ballot or seats total 100%. Opposition, foreign
97: backing and unrepresented movements add no governing consent. Missing history,
98: bans and an unvoted interim supply no buffer. This multiplier only limits the
99: gradual crisis-confidence penalty; material shortfalls, war exhaustion, prior
100: loyalty and the separate prospective programme veto remain effective. Powell's
101: discussion of legitimacy on pp. 1020–1021 motivates this distinction, not the
102: linear coefficient. [The same-state diagnostic](../campaign-certification/verification/evidence/2026-09-22-completion/public-mandate/README.md)
103: compares the proposed formula without claiming that the old trajectory used it.
104: Iteration 9 measured these combined effects; its unresolved failures and the
105: larger development cohort are recorded below.
106: 
107: At a completed election, prospective acceptance of a **new** Communist or
108: Islamist programme also reads institutional autonomy and that winner's actual
109: seat share. The same established-government requirement applies;
110: the additional veto term is `.45 × autonomy × winner seat share`. Strong army
111: loyalty, civilian control, or retention of the incumbent
112: programme resists that term. The material-grievance route retains its existing
113: `.25` discontent requirement; a credible institutional veto independently
114: constitutes a political crisis even when prices and output have recovered.
115: The `.35` loyalty threshold, monarchy exception, and authoritarianism guard
116: remain. A financial appropriation is not an automatic cure for political
117: opposition to civilian authority.
```

## docs/planning/ai-handoffs/CODEX-NEXT-ENGINEERING.md:39-70 at 032cd6a3b1a01149fc84d731fc649c3ba2319f62

```text
39: ## 1. Political calibration: CODEX-S27-A1-01
40: 
41: Reproduce and diagnose the remaining A1 concentration failure on a pinned
42: candidate. Retain the existing twelve-seed cohort, 252-month horizon, median
43: coup band of 4–14 and strict median top-three share below 0.50. Do not change
44: thresholds, substitute selected seeds, suppress inconvenient outcomes or label
45: passing packaging CI as a passing aggregate gate.
46: 
47: Use the existing `spheres-sim/tests/bloc_census.rs` diagnostics and
48: [calibration history](../../political-arm/2026-09-22-calibration-repairs.md).
49: Implement the smallest justified political correction, with focused tests and
50: the unchanged A1–A10 checks. Preserve failed attempts and explain any gameplay
51: change. Completion requires a reviewed correction and retained passing gate
52: evidence, not a change to the acceptance test. This task cannot close S27.
53: 
54: The [28 September evidence](../../campaign-certification/S27/preparation/a1-20260928/README.md)
55: retains the failed baseline and rejected trial 01. Both pass nine of ten outcome
56: gates plus attribution; the trial worsened the displayed A1 concentration from
57: 0.59 to 0.67. Source is restored, with the unrebuilt candidate executable boundary
58: recorded. No justified additional runtime correction was established, and no
59: reserved seed was consumed. A1 stays blocked; independent startup work follows
60: this attempted first task without claiming political or aggregate qualification.
61: 
62: The later [exact firing observer](../../campaign-certification/S27/preparation/a1-firing-observer-20260928/README.md)
63: closes the observation gap on the already-used seed 0, with 252 exact monthly
64: world/RNG/headline comparisons, 146,525 retained snapshots and ten actual firings.
65: All 151 eligible funding commands applied correctly. Nine firing cases lacked
66: fiscal headroom; the remaining case had a recovered target while actual loyalty
67: was still low. No further functional defect was established. Test-only hooks and
68: the complete compressed diagnostic data are retained; coefficients, acceptance
69: tests and reserved cohorts are unchanged. A1 remains blocked pending a justified
70: causal policy/model correction rather than another ungrounded coefficient trial.
```

## docs/campaign-certification/S27/preparation/a1-20260928/README.md:1-62 at 032cd6a3b1a01149fc84d731fc649c3ba2319f62

```text
1: # A1 political calibration: failed baseline and rejected trial 01
2: 
3: `CODEX-S27-A1-01` remains **blocked and incomplete**. Source baseline:
4: `30410e55217c7461937ad42564678c59d34f53b1`. The 28 September 2026 attempt
5: preserves the existing A1-A10 outcome definitions, cohorts and thresholds.
6: No holdout was run; no political calibration, S27, G5 or CP1 closure is earned.
7: 
8: ## Actual results
9: 
10: | Run | Outcome gates | Separate attribution control | A1 median elected coups | Displayed median top-three share |
11: |---|---|---|---|---|
12: | Fresh baseline | 9/10 pass; A1 fails | Pass | 7 | 0.59 |
13: | Trial 01 | 9/10 pass; A1 fails | Pass | 12 | 0.67 |
14: 
15: Both complete runs report **10 passed, 1 failed, 1 filtered**: ten historical
16: outcome gates plus one attribution regression ran; the measurement-only census
17: was filtered. A1 requires median coups in **4..14 inclusive** and median
18: per-seed top-three share **strictly below 0.50** over seeds 0..11 and 252 months.
19: Shares above are the two-decimal log displays, not higher-precision measurements.
20: The trial worsened the failed concentration reading while retaining the count
21: band. Logs retain all eleven outcomes and the original failing exit.
22: 
23: The [prospective plan](evidence/trial-01-plan.md) declared a single AI policy
24: revision: remove the civilian-confidence penalty from the Army funding inverse,
25: funding material and war needs only. This was a policy experiment, not correction
26: of an accidental arithmetic bug. The [exact patch](evidence/trial-01-candidate.patch)
27: changed no historical source, cohort or outcome assertion. The candidate failed
28: its first acceptance stage and was rejected; no candidate full-regression,
29: development N200 or reserved-seed validation is claimed.
30: 
31: ## Restoration and evidence
32: 
33: The [result](evidence/trial-01-result.json) and [source receipt](evidence/trial-01-source.json)
34: pin the candidate, restored government source and unchanged acceptance test.
35: During packaging, the complete restored government bytes were compared with the
36: captured pre-trial body, and the test hash was checked against the result. The
37: redundant 848 KB pre-trial source body is omitted; the baseline Git revision and
38: raw source hash identify it. [manifest.json](manifest.json) hashes every retained
39: original artifact. Local attributes preserve captured bytes across checkouts.
40: 
41: **Executable boundary:** at rejection, restoring source did not rebuild the
42: candidate executable. Its recorded SHA identifies the rejected trial, not a
43: restored-source binary. Later tests or preflights must rebuild and record their
44: own source/executable identity; this packet does not claim that rebuild occurred.
45: 
46: ## Next work without an unearned pass
47: 
48: The existing country-count and seed-0 diagnostics distinguish limited geographic
49: spread, repeat events, fiscal constraints and loyalty recovery. They demonstrate
50: no further concrete runtime defect requiring correction. In particular, removing
51: repeats from five or six distinct coup countries cannot by itself satisfy the
52: strict top-three share rule. Pooled shares and inverse-HHI are different measures
53: and are not substitutions for A1. The
54: [earlier calibration history](../../../../political-arm/2026-09-22-calibration-repairs.md)
55: retains rejected confidence trials and their Algeria/A2 regressions.
56: 
57: A1 therefore awaits a justified causal hypothesis or an explicitly reviewed game
58: design decision. No new coefficient search, manufactured event quota, weakened
59: assertion or use of reserved seeds is implied. The user's ordered work continues
60: with the independent startup engineering preflight after this recorded attempt;
61: A1 remains failed and excluded from any claim of aggregate CI or campaign
62: qualification. All canonical session dependencies remain unchanged.
```

## docs/campaign-certification/verification/evidence/2026-09-22-completion/checkpoint27-rejected-model/README.md:1-22 at 032cd6a3b1a01149fc84d731fc649c3ba2319f62

```text
1: # Rejected confidence reassessment 27
2: 
3: Candidate 27 changed only `ARMY_CRISIS_CONFIDENCE_WEIGHT` from 0.65 to 1.20 on runtime checkpoint `9f7faed5f541e14545e01caf1616ba08c6ad8ff9`; its evidence-only parent revision was `12278ea4c66863200fd10948350ae69391954cfa`. The single change is preserved in `iteration-27-candidate.patch`. The candidate was rejected and the coefficient restored to 0.65. It is not an accepted model or a certification result.
4: 
5: ## Unchanged tests and result
6: 
7: - Sim library: **1,004 passed, 1 failed, 25 ignored, 1 filtered**. The existing Algeria annulment chronology failed after an earlier elected-government coup. No fixture or assertion was changed to admit the candidate.
8: - Ordinary bloc integration target: **5 passed, 0 failed, 7 ignored**.
9: - Explicit original A1–A10 and attribution suite: **9 passed, 2 failed, 0 ignored, 1 filtered**. A1 recorded median 12 electoral coups but median top-three concentration 0.50, which fails the strict `< 0.5` requirement. A2 recorded Islamist takeovers by 2000 in 6/12 seeds, which fails the unchanged `> 6 && <= 10` requirement. The eight other outcome gates and the attribution regression passed.
10: - The result records no N200 development cohort and no independent holdout run. The candidate already violated the predeclared correctness stop rule; the original outcome suite was completed for diagnosis. There was no further parameter search in this trial.
11: 
12: The raw logs, prospective plan, precision review, application receipt, build receipt and final result retain their original bytes. The independent review clarifies two statements in the plan without changing implementation: a target exactly at 0.35 is not below the strict loyalty line, and pending-first-ballot protection belongs to coup eligibility rather than an unconditional zero raw confidence penalty.
13: 
14: ## Provenance and restoration boundary
15: 
16: The parent result records all **1,932** baseline native inputs restored and a clean worktree at `2026-09-22T23:14:05.947771+00:00`. The independent audit verified the restored government bytes against both the preserved external baseline and the parent revision's Git blob. Replacing the one coefficient declaration reconstructs the candidate SHA exactly. The large duplicate baseline government body and executable files are not copied here; the Git revision, one-line patch, input manifest and binary identities identify them.
17: 
18: The independent audit happened after a separately authorized three-line empty-stall optimization in `spheres-sim/src/dyads.rs`. At that later audit, **1,931/1,932** current inputs still matched the baseline raw hashes, with only that recorded later edit differing. The original dyads Git blob, reconstructed using its existing CRLF checkout convention, matches the baseline manifest. This distinction preserves the recorded clean restoration without claiming the later working tree was clean. The audit did not write runtime source or execute candidate binaries.
19: 
20: Candidate binary hashes are the contemporaneous build receipt's recorded identities. Those mutable paths were subsequently used for a restored-baseline rebuild; this archive does not claim to preserve or independently rehash the rejected executables after replacement. Baseline correctness and remaining A1 work are separate from this rejected trial.
21: 
22: `manifest.json` lists SHA-256 and byte sizes for every other archived file. `iteration-27-independent-verification.json` records the independent receipt/log/source checks and their limitations. The verification script is an external audit snapshot with machine-specific paths, not a runtime or automatic CI entrypoint.
```
