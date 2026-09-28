use super::*;
use crate::sector_contractors::{self as companies, CompanySector, CompanyTarget};
use std::cell::Cell;

thread_local! {
    pub(super) static ORIGINAL_TERMINAL_ASSEMBLY: Cell<bool> = const { Cell::new(false) };
    pub(super) static REUSED_TERMINAL_ASSEMBLY: Cell<u64> = const { Cell::new(0) };
}

fn original<T>(run: impl FnOnce() -> T) -> T {
    struct Reset(bool);
    impl Drop for Reset {
        fn drop(&mut self) { ORIGINAL_TERMINAL_ASSEMBLY.with(|flag| flag.set(self.0)); }
    }
    let _reset = Reset(ORIGINAL_TERMINAL_ASSEMBLY.with(|flag| flag.replace(true)));
    run()
}

fn fixture() -> (WorldState, NationId, u32, Vec<usize>) {
    let mut w = crate::init::world_1990(crate::world::GameRules {
        daily_simulation: true, resource_gates: true, resource_market: true,
        logistics_routes: true, physical_logistics: true, military_operations: true,
        ..Default::default()
    });
    w.conflicts.clear(); w.sanctions.clear();
    companies::enable(&mut w);
    let mut ports: BTreeMap<NationId, BTreeMap<String, usize>> = BTreeMap::new();
    for (index, edge) in network().edges.iter().enumerate() {
        if let Some((owner, CompanyTarget::Facility { district, .. })) = terminal_company(&w, edge) {
            ports.entry(owner).or_default().entry(district).or_insert(index);
        }
    }
    let (owner, ports) = ports.into_iter().find(|(owner, ports)| ports.len() >= 2
        && w.sector_contractors.roster.iter().any(|c| c.nation == *owner && c.sector == CompanySector::Logistics))
        .expect("authored world must contain two terminals served by one domestic logistics company");
    let company = w.sector_contractors.roster.iter_mut()
        .find(|c| c.nation == owner && c.sector == CompanySector::Logistics).unwrap();
    company.capacity = company.capacity.max(2);
    company.work_bonus = 0.2; company.fee_rate = 0.02;
    let id = company.id;
    let mut edges = vec![];
    for (district, index) in ports.into_iter().take(2) {
        production::complete_capability(&mut w, &district, production::ProjectKind::FreightTerminal);
        companies::assign(&mut w, owner, id, CompanyTarget::Facility {
            district, sector: CompanySector::Logistics,
        }).unwrap();
        edges.push(index);
    }
    w.nation_mut(owner).treasury_bn = Some(100.0);
    w.nation_mut(owner).debt_bn = Some(0.0);
    (w, owner, id, edges)
}

fn terminal_route(w: &WorldState, edge: usize,
    reads: Option<&mut BTreeMap<usize, TerminalAssemblyRead>>) -> RoutePlan {
    let (a, b) = network().endpoints[edge];
    let mut previous = vec![None; network().nodes.len()];
    previous[b] = Some((a, edge));
    assemble_plan_with_reads(w, b, &previous, None, reads).unwrap()
}

#[test]
fn terminal_identity_reads_match_original_bits_and_first_duplicate_selection() {
    let (base, owner, company, edges) = fixture();
    for case in 0..12 {
        let mut w = base.clone();
        match case {
            1 => { w.sector_contractors.enabled = false; },
            2 => { w.nation_mut(owner).alive = false; },
            3 => { w.sector_contractors.assignments.clear(); },
            4 => {
                // The first assignment remains authoritative, even when a
                // later duplicate resolves to a valid real contractor.
                let mut missing = w.sector_contractors.assignments.iter()
                    .find(|a| a.nation == owner && a.company_id == company).unwrap().clone();
                missing.company_id = u32::MAX;
                w.sector_contractors.assignments.insert(0, missing);
            },
            5 => {
                // A same-ID row in another sector must not hide the original
                // first row satisfying modifiers' complete roster predicate.
                let mut wrong = w.sector_contractors.roster.iter().find(|c| c.id == company).unwrap().clone();
                wrong.sector = CompanySector::Mining; wrong.work_bonus = 0.9;
                w.sector_contractors.roster.insert(0, wrong);
            },
            6 => {
                let mut duplicate = w.sector_contractors.roster.iter().find(|c| c.id == company).unwrap().clone();
                duplicate.work_bonus = 0.7; duplicate.experience = 540.0;
                w.sector_contractors.roster.insert(0, duplicate);
            },
            7 => {
                let first = w.sector_contractors.roster.iter_mut().find(|c| c.id == company).unwrap();
                first.work_bonus = f64::NAN; first.fee_rate = f64::INFINITY;
                first.experience = f64::NAN;
            },
            8 => {
                let first = w.sector_contractors.roster.iter_mut().find(|c| c.id == company).unwrap();
                first.work_bonus = -0.8; first.experience = f64::INFINITY;
            },
            9 => { w.production.industry.sites.clear(); },
            10 => {
                let district = terminal_company(&w, &network().edges[edges[0]]).unwrap().1;
                if let CompanyTarget::Facility { district, .. } = district {
                    let other = if owner == NationId::France { NationId::USA } else { NationId::France };
                    w.districts.insert(district, other);
                }
            },
            11 => { w.rules.daily_simulation = false; w.month = 2; },
            _ => {},
        }
        let before = crate::save(&w);
        let mut reads = BTreeMap::new();
        for _ in 0..2 { for &edge in &edges {
            let original = segment_capacity(&w, &network().edges[edge]);
            let resolved = TerminalAssemblyRead::new(&w, &network().edges[edge]);
            assert_eq!(resolved.capacity(&w).to_bits(), original.0.to_bits(), "case {case}");
            assert_eq!(resolved.label, original.1);
            let actual = terminal_route(&w, edge, Some(&mut reads));
            let expected = terminal_route(&w, edge, None);
            assert_eq!(actual.capacity_tonnes.to_bits(), expected.capacity_tonnes.to_bits());
            assert_eq!(actual, expected, "all route nodes/keys/labels case {case}");
        }}
        assert_eq!(crate::save(&w), before, "assembly is a pure read");
        assert_eq!(reads.len(), 2);
    }
}

#[test]
fn terminal_identity_reads_observe_shared_company_xp_and_exact_paid_work() {
    for boundary in [180.0, 540.0] {
        let (mut actual, owner, company, edges) = fixture();
        let row = actual.sector_contractors.roster.iter_mut().find(|c| c.id == company).unwrap();
        row.experience = boundary - 0.5; row.experience_day = None; row.experience_today = 0.0;
        let mut expected = actual.clone();
        let mut reads = BTreeMap::new();
        let opening: Vec<_> = edges.iter().map(|&e| terminal_route(&actual, e, Some(&mut reads))).collect();
        let indices: Vec<_> = reads.values().map(|read| read.company_index.unwrap()).collect();
        assert_eq!(indices[0], indices[1], "both terminals resolve the same live company row");
        // Synthetic single-edge receipts isolate each real terminal's paid
        // work; the existing full-route dispatch regression also exercises
        // this memo across cargo/admission and both XP thresholds.
        for &edge in &edges {
            let a = terminal_route(&actual, edge, Some(&mut reads));
            let b = terminal_route(&expected, edge, None);
            assert_eq!(a, b);
            for (world, route) in [(&mut actual, &a), (&mut expected, &b)] {
                *world.logistics.usage_tonnes.entry(network().edge_keys[edge].clone()).or_default() += 100.0;
                record_terminal_work(world, route, 100.0);
            }
            assert_eq!(crate::save(&actual), crate::save(&expected), "exact usage, fees, counters and XP");
            for (i, &other) in edges.iter().enumerate() {
                let a = terminal_route(&actual, other, Some(&mut reads));
                let b = terminal_route(&expected, other, None);
                assert_eq!(a.capacity_tonnes.to_bits(), b.capacity_tonnes.to_bits());
                assert_eq!(a, b);
                assert!(a.capacity_tonnes > opening[i].capacity_tonnes, "both terminals see shared live XP");
            }
        }
        let row = actual.sector_contractors.roster.iter().find(|c| c.id == company).unwrap();
        assert_eq!(row.experience.to_bits(), (boundary + 0.5_f64).to_bits(), "daily XP cap stays live");
        assert!(row.total_fees_bn > 0.0);
        assert!(actual.nation(owner).treasury_bn.unwrap() < 100.0);
    }
}

#[test]
fn terminal_identity_cache_drops_before_assignment_roster_and_calendar_changes() {
    let (mut w, owner, company, edges) = fixture();
    let mut routes = ClearingRoutes::new(&w);
    let (a, b) = network().endpoints[edges[0]];
    let mut previous = vec![None; network().nodes.len()]; previous[b] = Some((a, edges[0]));
    assemble_plan_with_reads(&w, b, &previous, None, Some(&mut routes.assembly_terminals)).unwrap();
    assert!(!routes.assembly_terminals.is_empty());
    let mut pool = NominalRoutePool::default(); routes.return_to_pool(&mut pool);
    w.sector_contractors.assignments.retain(|a| !(a.nation == owner && a.company_id == company));
    w.sector_contractors.roster.reverse(); w.month = 2; w.day = 1;
    let mut next = ClearingRoutes::with_pool(&w, &mut pool);
    assert!(next.assembly_terminals.is_empty());
    let actual = assemble_plan_with_reads(&w, b, &previous, None, Some(&mut next.assembly_terminals)).unwrap();
    assert_eq!(actual, assemble_plan(&w, b, &previous).unwrap());
    assert!(next.assembly_terminals[&edges[0]].company_index.is_none());
}

#[test]
#[ignore = "actual immutable 2015/2035 checkpoint, original terminal assembly for 31 days; no timing claims"]
fn terminal_identity_reuse_matches_actual_checkpoint_for_31_complete_days() {
    let path = std::env::var("SPHERES_S22_CHECKPOINT").expect("actual checkpoint path required");
    let source = std::fs::read_to_string(&path).unwrap();
    let mut value: serde_json::Value = serde_json::from_str(&source).unwrap();
    let world = if value["format"] == "spheres-campaign" { value["world"].take() } else { value.take() };
    drop(value);
    let mut actual = crate::load_value(world).unwrap();
    assert!(actual.year == 2015 || actual.year == 2035);
    assert!(enabled(&actual) && actual.rules.military_operations && actual.sector_contractors.enabled);
    let mut expected = actual.clone();
    let before = REUSED_TERMINAL_ASSEMBLY.with(|count| count.get());
    for day in 0..31 {
        let a = crate::tick_day(&mut actual, &[]);
        let b = original(|| crate::tick_day(&mut expected, &[]));
        assert_eq!(a, b, "returned headlines day {day}");
        assert_eq!(actual.headlines, expected.headlines, "retained headlines day {day}");
        assert!(crate::save(&actual) == crate::save(&expected), "complete native world day {day}");
    }
    let reused = REUSED_TERMINAL_ASSEMBLY.with(|count| count.get()) - before;
    assert!(reused > 0, "actual checkpoint must reuse terminal identity reads");
    assert_eq!(std::fs::read_to_string(path).unwrap(), source, "immutable actual source");
    eprintln!("31 complete native days match original terminal assembly; {reused} terminal identities reused with live modifiers.");
}
