//! Physical freight routes over the game's own geography.
//!
//! The network is baked by `tools/logistics/build_network.py` from the same
//! Natural Earth district paths and coastline the globe draws. It claims no
//! historical port, rail or road count: district centroids, modeled coastal
//! gateways and a coarse ocean mesh are routing geometry. Throughput is a game
//! capacity shared by every commodity; completed province infrastructure is
//! the only player-built multiplier. The annual ministry gap is deliberately
//! absent here (CLAUDE iron rule 8: it already owns extraction).
//!
//! Freight settles daily in daily simulations, monthly in legacy audits.
//! Dispatch removes no cargo stock. Assigned terminal operators charge their
//! government for actual handling through the ordinary fiscal channel; the
//! resource market owns those ledgers. It merely reserves the narrowest route,
//! records cargo and returns the quantity the market may remove from the
//! seller. `begin_month` returns arrivals for that market to credit. A closure
//! holds goods in transit instead of destroying them.

use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::collections::{BTreeMap, BTreeSet, BinaryHeap};
use std::sync::OnceLock;

use crate::districts;
use crate::production;
use crate::resources::{self, Commodity, ShipmentSource};
use crate::statecraft;
use crate::world::{NationId, WorldState};

pub const EMBEDDED_NETWORK: &str = include_str!("../data/logistics_network.json");
const EPS: f64 = 1e-9;
/// Two optional alternate attempts can each settle at most this many nodes.
/// The nominal route remains available when the bounded search cannot improve it.
const CONGESTION_SEARCH_NODE_LIMIT: usize = 1024;

#[derive(Clone, Copy, Debug, Default, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "snake_case")]
pub enum RoutePolicy {
    #[default]
    Fastest,
    LandOnly,
    AvoidChokepoints,
}

impl RoutePolicy {
    pub fn parse(s: &str) -> Option<Self> {
        Some(
            match s
                .trim()
                .to_ascii_lowercase()
                .replace([' ', '-'], "_")
                .as_str()
            {
                "fastest" => Self::Fastest,
                "land_only" | "land" => Self::LandOnly,
                "avoid_chokepoints" | "avoid_chokes" => Self::AvoidChokepoints,
                _ => return None,
            },
        )
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct RouteNode {
    pub id: String,
    pub name: String,
    pub kind: String,
    pub lon: f64,
    pub lat: f64,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct RoutePlan {
    pub mode: String,
    pub nodes: Vec<RouteNode>,
    pub distance_km: u32,
    pub estimated_days: u32,
    pub months: u32,
    pub capacity_tonnes: f64,
    pub bottleneck: String,
    pub chokepoints: Vec<String>,
    /// Stable edge keys. Public for save continuity; clients may ignore it.
    pub segments: Vec<String>,
    /// A dispatch-only explanation; absent from nominal routes and old saves.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dispatch_note: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Cargo {
    pub id: u64,
    pub seller: NationId,
    pub buyer: NationId,
    pub commodity: Commodity,
    pub quantity: f64,
    pub source: ShipmentSource,
    pub contract: Option<u32>,
    pub route: RoutePlan,
    pub dispatched_month: i32,
    pub due_month: i32,
    /// Absolute playable departure/arrival dates. Old monthly saves acquire
    /// these lazily when the daily scheduler first visits their cargo.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dispatched_day: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub due_day: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hold_reason: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Logistics {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub cargo: Vec<Cargo>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub arrivals: Vec<Cargo>,
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub policies: BTreeMap<NationId, RoutePolicy>,
    /// Tonnes reserved this settlement, keyed by stable graph edge.
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub usage_tonnes: BTreeMap<String, f64>,
    #[serde(default, skip_serializing_if = "is_zero_u64")]
    pub next_id: u64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_month: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_day: Option<i32>,
    /// A settlement-local acceleration only. It is never truth and never saved.
    #[serde(skip)]
    route_cache: BTreeMap<(NationId, NationId, RoutePolicy), RoutePlan>,
}

impl Default for Logistics {
    fn default() -> Self {
        Self {
            cargo: vec![],
            arrivals: vec![],
            policies: BTreeMap::new(),
            usage_tonnes: BTreeMap::new(),
            next_id: 0,
            last_month: None,
            last_day: None,
            route_cache: BTreeMap::new(),
        }
    }
}

impl PartialEq for Logistics {
    fn eq(&self, other: &Self) -> bool {
        self.cargo == other.cargo
            && self.arrivals == other.arrivals
            && self.policies == other.policies
            && self.usage_tonnes == other.usage_tonnes
            && self.next_id == other.next_id
            && self.last_month == other.last_month
            && self.last_day == other.last_day
    }
}

fn is_zero_u64(v: &u64) -> bool {
    *v == 0
}

impl Logistics {
    pub fn is_empty(&self) -> bool {
        self.cargo.is_empty()
            && self.arrivals.is_empty()
            && self.policies.is_empty()
            && self.usage_tonnes.is_empty()
            && self.next_id == 0
            && self.last_month.is_none()
            && self.last_day.is_none()
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Dispatch {
    pub quantity: f64,
    pub route: Option<RoutePlan>,
    pub reason: Option<String>,
}

#[derive(Deserialize)]
struct NetworkFile {
    nodes: Vec<Node>,
    edges: Vec<Edge>,
}
#[derive(Clone, Deserialize)]
struct Node {
    id: String,
    name: String,
    kind: String,
    lon: f64,
    lat: f64,
    #[allow(dead_code)]
    district: Option<String>,
}
#[derive(Clone, Deserialize)]
struct Edge {
    a: String,
    b: String,
    km: u32,
    kind: String,
    chokepoint: Option<String>,
}
struct Network {
    nodes: Vec<Node>,
    index: BTreeMap<String, usize>,
    edges: Vec<Edge>,
    edge_index: BTreeMap<String, usize>,
    adj: Vec<Vec<usize>>,
    /// Integer endpoints preserve authored edge order without repeated string
    /// tree lookups at every Dijkstra relaxation.
    endpoints: Vec<(usize, usize)>,
    edge_keys: Vec<String>,
}

fn network() -> &'static Network {
    static NETWORK: OnceLock<Network> = OnceLock::new();
    NETWORK.get_or_init(|| {
        let f: NetworkFile =
            serde_json::from_str(EMBEDDED_NETWORK).expect("data/logistics_network.json must parse");
        let index: BTreeMap<String, usize> = f
            .nodes
            .iter()
            .enumerate()
            .map(|(i, n)| (n.id.clone(), i))
            .collect();
        assert_eq!(index.len(), f.nodes.len(), "duplicate logistics node");
        let mut adj = vec![vec![]; f.nodes.len()];
        let mut edge_index = BTreeMap::new();
        let mut endpoints = Vec::with_capacity(f.edges.len());
        for (i, e) in f.edges.iter().enumerate() {
            let a = *index.get(&e.a).expect("logistics edge a must exist");
            let b = *index.get(&e.b).expect("logistics edge b must exist");
            endpoints.push((a, b));
            adj[a].push(i);
            adj[b].push(i);
            assert!(
                edge_index.insert(edge_key(e), i).is_none(),
                "duplicate logistics edge"
            );
        }
        for a in &mut adj {
            a.sort_unstable();
        }
        Network {
            edge_keys: f.edges.iter().map(edge_key).collect(),
            nodes: f.nodes,
            index,
            edges: f.edges,
            edge_index,
            adj,
            endpoints,
        }
    })
}

pub fn enabled(w: &WorldState) -> bool {
    w.rules.resource_gates
        && w.rules.resource_market
        && w.rules.logistics_routes
        && w.rules.physical_logistics
}

pub fn policy_for(w: &WorldState, nation: NationId) -> RoutePolicy {
    w.logistics
        .policies
        .get(&nation)
        .copied()
        .unwrap_or_default()
}

pub fn set_policy(w: &mut WorldState, nation: NationId, policy: RoutePolicy) -> Result<(), String> {
    if !w.nation_opt(nation).is_some_and(|n| n.alive) {
        return Err("That government is not on the board.".into());
    }
    if policy == RoutePolicy::Fastest {
        w.logistics.policies.remove(&nation);
    } else {
        w.logistics.policies.insert(nation, policy);
    }
    w.logistics
        .route_cache
        .retain(|(_, buyer, _), _| *buyer != nation);
    Ok(())
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct Visit {
    cost: u64,
    node: usize,
}
impl Ord for Visit {
    fn cmp(&self, o: &Self) -> Ordering {
        o.cost.cmp(&self.cost).then_with(|| o.node.cmp(&self.node))
    }
}
impl PartialOrd for Visit {
    fn partial_cmp(&self, o: &Self) -> Option<Ordering> {
        Some(self.cmp(o))
    }
}

fn edge_key(e: &Edge) -> String {
    format!("{}|{}", e.a, e.b)
}

fn owner_passable(w: &WorldState, owner: NationId, seller: NationId, buyer: NationId) -> bool {
    if owner == seller || owner == buyer {
        return true;
    }
    w.nation_opt(owner).is_some_and(|n| n.alive)
        && !w.is_sanctioning(owner, seller)
        && !w.is_sanctioning(seller, owner)
        && !w.is_sanctioning(owner, buyer)
        && !w.is_sanctioning(buyer, owner)
        && !statecraft::belligerents(w, owner, seller)
        && !statecraft::belligerents(w, owner, buyer)
}

fn effective_owner(w: &WorldState, district: &str, owner: NationId) -> NationId {
    for c in &w.conflicts {
        let Some(&hold) = c.front.get(district) else {
            continue;
        };
        let controller = if hold as f64 > crate::front::HELD_BAND {
            c.side_a.first().copied()
        } else if (hold as f64) < -crate::front::HELD_BAND {
            c.side_b.first().copied()
        } else {
            None
        };
        if let Some(controller) = controller {
            if c.side_of(owner) != c.side_of(controller) {
                return controller;
            }
        }
    }
    owner
}

fn district_passable(w: &WorldState, node: &Node, seller: NationId, buyer: NationId) -> bool {
    if node.kind != "district" {
        return true;
    }
    let Some(owner) = w.districts.get(node.id.as_str()).copied() else {
        return false;
    };
    if w.rules.military_operations {
        return crate::control::controller(w, &node.id)
            .is_some_and(|controller| owner_passable(w, controller, seller, buyer));
    }
    owner_passable(w, effective_owner(w, &node.id, owner), seller, buyer)
}

fn freight_controller(w: &WorldState, district: &str) -> Option<NationId> {
    if w.rules.military_operations { crate::control::controller(w, district) }
    else { w.districts.get(district).map(|&owner| effective_owner(w, district, owner)) }
}

fn route_open(
    w: &WorldState,
    seller: NationId,
    buyer: NationId,
    route: &RoutePlan,
) -> Result<(), String> {
    if !w.nation_opt(seller).is_some_and(|n| n.alive)
        || !w.nation_opt(buyer).is_some_and(|n| n.alive)
    {
        return Err("A route endpoint no longer has a living government.".into());
    }
    if w.is_sanctioning(seller, buyer) || w.is_sanctioning(buyer, seller) {
        return Err("Bilateral sanctions closed the route.".into());
    }
    if statecraft::belligerents(w, seller, buyer) {
        return Err("War closed the route.".into());
    }
    if w.rules.military_operations && (route.nodes.first().is_none_or(|n| !crate::control::can_operate(w, seller, &n.id))
        || route.nodes.last().is_none_or(|n| !crate::control::can_operate(w, buyer, &n.id))) {
        return Err("A booked freight endpoint is no longer under its sponsoring government's ownership and control.".into());
    }
    let mut permissions = vec![None; if w.rules.military_operations { crate::nations::nation_count() } else { 0 }];
    for n in &route.nodes {
        let Some(i) = network().index.get(&n.id) else {
            return Err("The saved route no longer exists.".into());
        };
        let node = &network().nodes[*i];
        let passable = if w.rules.military_operations && node.kind == "district" {
            freight_controller(w, &node.id).is_some_and(|owner| {
                if let Some(allowed) = permissions[owner.index()] { allowed }
                else {
                    let allowed = owner_passable(w, owner, seller, buyer);
                    permissions[owner.index()] = Some(allowed);
                    allowed
                }
            })
        } else { district_passable(w, node, seller, buyer) };
        if !passable {
            return Err(format!("Transit through {} is closed.", n.name));
        }
    }
    Ok(())
}

/// Only mapped gateways can receive a player-built terminal upgrade. This is
/// routing geometry, not an assertion that a historical port existed here.
pub fn has_terminal(district: &str) -> bool {
    network().edges.iter().any(|e| e.kind == "terminal" && (e.a == district || e.b == district))
}

fn segment_capacity(w: &WorldState, e: &Edge) -> (f64, String) {
    let net = network();
    let (a, b) = (&net.nodes[net.index[&e.a]], &net.nodes[net.index[&e.b]]);
    let (monthly, label) = match e.kind.as_str() {
        "sea" => (
            75_000.0,
            e.chokepoint
                .clone()
                .unwrap_or_else(|| "Open-sea lift".into()),
        ),
        "terminal" => (
            45_000.0 * (1.0 + production::level(w,
                if a.kind == "district" { &a.id } else { &b.id },
                production::ProjectKind::FreightTerminal) as f64 * 0.25),
            format!(
                "{} freight terminal",
                if a.kind == "district" {
                    &a.name
                } else {
                    &b.name
                }
            ),
        ),
        _ => {
            let level = [a, b]
                .into_iter()
                .filter(|n| n.kind == "district")
                .map(|n| production::province_capabilities(w, &n.id).infrastructure)
                .min()
                .unwrap_or(0) as f64;
            let terrain = [a, b]
                .into_iter()
                .filter(|n| n.kind == "district")
                .map(|n| districts::terrain_of(&n.id))
                .fold(1.0_f64, |m, t| {
                    m.min(match t {
                        districts::TerrainClass::Mountain => 0.55,
                        districts::TerrainClass::Highland => 0.78,
                        districts::TerrainClass::Desert => 0.82,
                        districts::TerrainClass::Wetland => 0.72,
                        districts::TerrainClass::Tundra => 0.65,
                        districts::TerrainClass::Lowland => 1.0,
                    })
                });
            (
                25_000.0 * (1.0 + level * 0.20) * terrain,
                format!("{} land corridor", a.name),
            )
        }
    };
    let operator = terminal_company(w, e).map_or(1.0, |(owner, target)| {
        crate::companies::modifiers(w, owner, &target).work_rate
    });
    (monthly * crate::clock::month_fraction(w) * operator, label)
}

fn terminal_company(w: &WorldState, e: &Edge) -> Option<(NationId, crate::companies::CompanyTarget)> {
    if w.companies.is_empty() || !crate::clock::is_daily(w) || e.kind != "terminal" { return None; }
    let district = [&e.a, &e.b].into_iter().find(|d| w.districts.contains_key(*d))?;
    let owner = *w.districts.get(district)?;
    if !w.nation_opt(owner).is_some_and(|n| n.alive)
        || (w.rules.military_operations && !crate::control::can_operate(w, owner, district)) {
        return None;
    }
    Some((owner, crate::companies::CompanyTarget::Facility {
        district: district.clone(), sector: crate::companies::CompanySector::Logistics,
    }))
}

/// MODEL handling basis: $10 per tonne at each contracted terminal. The
/// contractor's quoted percentage is billed only when cargo actually moves.
pub const COMPANY_HANDLING_BN_PER_TONNE: f64 = 0.00000001;
fn record_terminal_work(w: &mut WorldState, route: &RoutePlan, tonnes: f64) {
    if w.companies.is_empty() || tonnes <= 0.0 || !crate::clock::is_daily(w) { return; }
    let mut seen = BTreeSet::new();
    for segment in &route.segments {
        let Some(&ei) = network().edge_index.get(segment) else { continue; };
        let e = &network().edges[ei];
        let Some((owner, target)) = terminal_company(w, e) else { continue; };
        if !seen.insert(target.clone()) { continue; }
        crate::companies::record_sector_activity(w, owner, crate::companies::CompanySector::Logistics, tonnes);
        let modifiers = crate::companies::modifiers(w, owner, &target);
        if modifiers.fee_rate <= 0.0 && modifiers.work_rate == 1.0 { continue; }
        let used_after = w.logistics.usage_tonnes.get(segment).copied().unwrap_or(0.0);
        let ordinary_capacity = segment_capacity(w, e).0 / modifiers.work_rate.max(0.1);
        let bonus = (used_after - ordinary_capacity).max(0.0)
            - (used_after - tonnes - ordinary_capacity).max(0.0);
        let fee = tonnes * COMPANY_HANDLING_BN_PER_TONNE * modifiers.fee_rate;
        if fee > 0.0 {
            let share = fee / w.nation(owner).gdp.max(0.1);
            crate::economy::charge(w, owner, fee, share);
        }
        crate::companies::record_work(w, owner, &target, tonnes, bonus, fee);
    }
}

pub fn plan(w: &WorldState, seller: NationId, buyer: NationId) -> Result<RoutePlan, String> {
    plan_impl(w, seller, buyer, true)
}

/// Ephemeral search trees for ONE spot clearing only. That caller changes
/// freight usage and cash/stock ledgers, never ownership, transit permissions,
/// infrastructure, route policies or the calendar. Nothing is persisted.
///
/// A tree is shared only for the identical source, policy and complete owner
/// permission vector. The original heap order and strict relaxation remain
/// unchanged. The first settled buyer goal is exactly where the pair search
/// would have stopped; later visits cannot rewrite its predecessor chain.
pub(crate) struct ClearingRoutes {
    owned_nodes: Vec<Vec<usize>>,
    effective_owners: Vec<Option<NationId>>,
    trees: BTreeMap<(NationId, RoutePolicy, Vec<bool>), SearchTree>,
    /// Infrastructure and calendar cannot change during one spot clearing.
    capacities: Vec<f64>,
    /// A deterministic gameplay search budget for one complete spot clearing.
    /// Budget exhaustion keeps the nominal route; it never changes capacity.
    search_nodes_left: usize,
}

struct SearchTree {
    dist: Vec<u64>,
    prev: Vec<Option<(usize, usize)>>,
    settled: Vec<usize>,
    queue: BinaryHeap<Visit>,
    next_rank: usize,
}

impl ClearingRoutes {
    pub(crate) fn new(w: &WorldState) -> Self {
        let net = network();
        let mut owned_nodes = vec![vec![]; crate::nations::nation_count()];
        // Preserve the exact BTreeMap district/start ordering of plan_impl.
        for (district, owner) in &w.districts {
            if let Some(&i) = net.index.get(district) {
                owned_nodes[owner.index()].push(i);
            }
        }
        let effective_owners = net.nodes.iter().map(|n| freight_controller(w, &n.id)).collect();
        let capacities = if w.rules.military_operations { net.edges.iter().map(|e| segment_capacity(w, e).0).collect() } else { vec![] };
        Self { owned_nodes, effective_owners, trees: BTreeMap::new(), capacities, search_nodes_left: 32_768 }
    }

    fn plan(&mut self, w: &WorldState, seller: NationId, buyer: NationId) -> Result<RoutePlan, String> {
        // Keep all endpoint errors, including their precedence, on the
        // original path. This also avoids indexing an invalid nation id.
        if seller == buyer || !w.nation_opt(seller).is_some_and(|n| n.alive)
            || !w.nation_opt(buyer).is_some_and(|n| n.alive)
            || w.is_sanctioning(seller, buyer) || w.is_sanctioning(buyer, seller)
            || statecraft::belligerents(w, seller, buyer)
        { return plan(w, seller, buyer); }
        let net = network();
        let policy = policy_for(w, buyer);
        let mut owner_pass = vec![false; crate::nations::nation_count()];
        for &owner in crate::nations::all_nations() {
            owner_pass[owner.index()] = owner_passable(w, owner, seller, buyer);
        }
        let passable = |i: usize| net.nodes[i].kind != "district"
            || self.effective_owners[i].is_some_and(|owner| owner_pass[owner.index()]);
        let starts: Vec<_> = self.owned_nodes[seller.index()].iter().copied().filter(|&i| passable(i)).collect();
        let goals: BTreeSet<_> = self.owned_nodes[buyer.index()].iter().copied().filter(|&i| passable(i)).collect();
        if starts.is_empty() || goals.is_empty() {
            return Err("No mapped freight gateway exists for one endpoint.".into());
        }
        let key = (seller, policy, owner_pass.clone());
        if !self.trees.contains_key(&key) {
            // Bound retained source searches, including their resumable heaps.
            // More distinct masks use the original search, never a world cache.
            if self.trees.len() >= 128 { return plan(w, seller, buyer); }
            let mut dist = vec![u64::MAX; net.nodes.len()];
            let mut queue = BinaryHeap::new();
            for s in starts { dist[s] = 0; queue.push(Visit { cost: 0, node: s }); }
            self.trees.insert(key.clone(), SearchTree {
                dist, prev: vec![None; net.nodes.len()], settled: vec![usize::MAX; net.nodes.len()],
                queue, next_rank: 0,
            });
        }
        let tree = self.trees.get_mut(&key).unwrap();
        let mut finish = goals.iter().copied().filter(|&i| tree.settled[i] != usize::MAX)
            .min_by_key(|&i| tree.settled[i]);
        // Extend precisely the same Dijkstra traversal only as far as this
        // buyer requires. Already settled goals keep their original rank and
        // predecessor chain. Expand the stopping goal before pausing so a
        // later destination can resume without losing any outgoing edge.
        while finish.is_none() {
            let Some(Visit { cost, node }) = tree.queue.pop() else { break; };
            if cost != tree.dist[node] { continue; }
            tree.settled[node] = tree.next_rank;
            tree.next_rank += 1;
            for &ei in &net.adj[node] {
                let e = &net.edges[ei];
                let (a, b) = net.endpoints[ei];
                let next = if a == node { b } else { a };
                if policy == RoutePolicy::LandOnly
                    && (e.kind == "sea" || net.nodes[next].kind == "gateway") { continue; }
                if !passable(next) { continue; }
                let penalty = if policy == RoutePolicy::AvoidChokepoints && e.chokepoint.is_some() {
                    10_000_000
                } else { 0 };
                let travel = e.km as u64 * if e.kind == "sea" { 3 } else { 4 };
                let nc = cost + travel + penalty;
                if nc < tree.dist[next] {
                    tree.dist[next] = nc;
                    tree.prev[next] = Some((node, ei));
                    tree.queue.push(Visit { cost: nc, node: next });
                }
            }
            if goals.contains(&node) { finish = Some(node); }
        }
        let finish = finish.ok_or_else(|| match policy {
                RoutePolicy::LandOnly => "No open all-land route exists.".to_string(),
                _ => "No open physical route exists.".to_string(),
            })?;
        assemble_plan(w, finish, &tree.prev)
    }
}

fn plan_impl(w: &WorldState, seller: NationId, buyer: NationId, memoize: bool) -> Result<RoutePlan, String> {
    plan_search(w, seller, buyer, memoize, None, None)
}

fn plan_search(w: &WorldState, seller: NationId, buyer: NationId, memoize: bool,
    capacity: Option<(&BTreeMap<String, f64>, f64, Option<&[f64]>)>,
    mut search_budget: Option<&mut usize>) -> Result<RoutePlan, String> {
    if seller == buyer {
        return Err("Domestic freight does not need an international route.".into());
    }
    if !w.nation_opt(seller).is_some_and(|n| n.alive)
        || !w.nation_opt(buyer).is_some_and(|n| n.alive)
    {
        return Err("Both route endpoints must be living governments.".into());
    }
    if w.is_sanctioning(seller, buyer) || w.is_sanctioning(buyer, seller) {
        return Err("Bilateral sanctions closed the route.".into());
    }
    if statecraft::belligerents(w, seller, buyer) {
        return Err("War closed the route.".into());
    }
    let policy = policy_for(w, buyer);
    let net = network();
    // A node may be considered from several neighboring edges; a government's
    // transit decision is identical at all its uncontested districts. These
    // two lazily populated arrays are private to this immutable path search.
    // No world cache, invalidation or heap/edge ordering changes are needed.
    let mut node_pass = vec![None; net.nodes.len()];
    let mut owner_pass = vec![None; crate::nations::nation_count()];
    let mut passable = |i: usize| {
        if !memoize { return district_passable(w, &net.nodes[i], seller, buyer); }
        if let Some(result) = node_pass[i] { return result; }
        let node = &net.nodes[i];
        let result = if node.kind != "district" { true }
        else if let Some(owner) = freight_controller(w, &node.id) {
            if let Some(result) = owner_pass[owner.index()] { result }
            else {
                let result = owner_passable(w, owner, seller, buyer);
                owner_pass[owner.index()] = Some(result);
                result
            }
        } else { false };
        node_pass[i] = Some(result);
        result
    };
    let starts: Vec<usize> = w
        .districts
        .iter()
        .filter(|(_, o)| **o == seller)
        .filter_map(|(d, _)| net.index.get(d).copied())
        .filter(|&i| passable(i))
        .collect();
    let goals: BTreeSet<usize> = w
        .districts
        .iter()
        .filter(|(_, o)| **o == buyer)
        .filter_map(|(d, _)| net.index.get(d).copied())
        .filter(|&i| passable(i))
        .collect();
    if starts.is_empty() || goals.is_empty() {
        return Err("No mapped freight gateway exists for one endpoint.".into());
    }
    let mut dist = vec![u64::MAX; net.nodes.len()];
    let mut prev: Vec<Option<(usize, usize)>> = vec![None; net.nodes.len()];
    let mut q = BinaryHeap::new();
    for s in starts {
        dist[s] = 0;
        q.push(Visit { cost: 0, node: s });
    }
    let mut finish = None;
    let mut settled_nodes = 0;
    while let Some(Visit { cost, node }) = q.pop() {
        if cost != dist[node] {
            continue;
        }
        if let Some(left) = search_budget.as_mut() {
            if **left == 0 { break; }
            **left -= 1;
        }
        if goals.contains(&node) {
            finish = Some(node);
            break;
        }
        if capacity.is_some() {
            settled_nodes += 1;
            if settled_nodes >= CONGESTION_SEARCH_NODE_LIMIT { break; }
        }
        for &ei in &net.adj[node] {
            let e = &net.edges[ei];
            let (a, b) = if memoize { net.endpoints[ei] } else { (net.index[&e.a], net.index[&e.b]) };
            if let Some((used, minimum, capacities)) = capacity {
                let limit = capacities.map_or_else(|| segment_capacity(w, e).0, |v| v[ei]);
                let remaining = limit - used.get(&net.edge_keys[ei]).copied().unwrap_or(0.0);
                if remaining < minimum { continue; }
            }
            let next = if a == node { b } else { a };
            if policy == RoutePolicy::LandOnly
                && (e.kind == "sea" || net.nodes[next].kind == "gateway")
            {
                continue;
            }
            if !passable(next) {
                continue;
            }
            let penalty = if policy == RoutePolicy::AvoidChokepoints && e.chokepoint.is_some() {
                10_000_000
            } else {
                0
            };
            // Divide this fixed-point time by 1,680 for days: sea moves 560
            // km/day and land 420. Fastest therefore means fastest, without a
            // runtime float comparison deciding graph order.
            let travel = e.km as u64 * if e.kind == "sea" { 3 } else { 4 };
            let nc = cost + travel + penalty;
            if nc < dist[next] {
                dist[next] = nc;
                prev[next] = Some((node, ei));
                q.push(Visit {
                    cost: nc,
                    node: next,
                });
            }
        }
    }
    let at = finish.ok_or_else(|| match policy {
        RoutePolicy::LandOnly => "No open all-land route exists.".to_string(),
        _ => "No open physical route exists.".to_string(),
    })?;
    assemble_plan(w, at, &prev)
}

fn assemble_plan(w: &WorldState, mut at: usize, prev: &[Option<(usize, usize)>]) -> Result<RoutePlan, String> {
    let net = network();
    let mut ids = vec![at];
    let mut edge_ids = vec![];
    while let Some((p, e)) = prev[at] {
        ids.push(p);
        edge_ids.push(e);
        at = p
    }
    ids.reverse();
    edge_ids.reverse();
    let distance_km = edge_ids.iter().map(|i| net.edges[*i].km).sum();
    let has_sea = edge_ids.iter().any(|i| net.edges[*i].kind == "sea");
    let has_land = edge_ids.iter().any(|i| net.edges[*i].kind == "land");
    let mode = match (has_land, has_sea) {
        (true, true) => "mixed",
        (false, true) => "sea",
        _ => "land",
    }
    .to_string();
    let travel_weight: u64 = edge_ids
        .iter()
        .map(|i| {
            let e = &net.edges[*i];
            e.km as u64 * if e.kind == "sea" { 3 } else { 4 }
        })
        .sum();
    let estimated_days = ((travel_weight + 1_679) / 1_680) as u32 + 2;
    let months = ((estimated_days + 29) / 30).max(1);
    let mut bottleneck = (f64::INFINITY, "Unconstrained".to_string());
    let mut chokes = BTreeSet::new();
    let mut segments = vec![];
    for ei in edge_ids {
        let e = &net.edges[ei];
        let (cap, name) = segment_capacity(w, e);
        if cap < bottleneck.0 {
            bottleneck = (cap, name)
        }
        if let Some(c) = &e.chokepoint {
            chokes.insert(c.clone());
        }
        segments.push(edge_key(e));
    }
    let nodes = ids
        .into_iter()
        .map(|i| {
            let n = &net.nodes[i];
            RouteNode {
                id: n.id.clone(),
                name: n.name.clone(),
                kind: n.kind.clone(),
                lon: n.lon,
                lat: n.lat,
            }
        })
        .collect();
    Ok(RoutePlan {
        mode,
        nodes,
        distance_km,
        estimated_days,
        months,
        capacity_tonnes: bottleneck.0,
        bottleneck: bottleneck.1,
        chokepoints: chokes.into_iter().collect(),
        segments,
        dispatch_note: None,
    })
}

#[path = "freight_routing.rs"]
mod freight_routing;

fn tonnes_per_unit(c: Commodity) -> f64 {
    match c.unit() {
        "kg" => 0.001,
        "kt" => 1000.0,
        "bcf" => 20_000.0,
        "kb/d" => 4_080.0,
        _ => 1.0,
    }
}
fn floor9(v: f64) -> f64 {
    (v.max(0.0) * 1e9).floor() / 1e9
}

/// Reserve the same edge ledger used by raw commodities for a separately typed
/// cargo owner. This creates no raw cargo, transfers no stock and pays nobody.
/// The caller owns its cargo lifecycle. Never opens the settlement: resources
/// must first post its arrivals, otherwise opening here would lose raw cargo.
pub fn reserve_freight(
    w: &mut WorldState,
    seller: NationId,
    buyer: NationId,
    requested: f64,
    tonnes_per_unit: f64,
) -> Dispatch {
    let refused = |reason: &str| Dispatch { quantity: 0.0, route: None, reason: Some(reason.into()) };
    if !enabled(w) || !crate::clock::is_daily(w) {
        return refused("Physical daily freight must be enabled.");
    }
    if !requested.is_finite() || requested < 0.0 || !tonnes_per_unit.is_finite() || tonnes_per_unit <= 0.0 {
        return refused("Freight quantity and unit weight must be finite and valid.");
    }
    if w.logistics.last_day != Some(crate::clock::absolute_day(w)) {
        return refused("The freight settlement has not begun for this day.");
    }
    let key = (seller, buyer, policy_for(w, buyer));
    let route = w.logistics.route_cache.get(&key).cloned()
        .filter(|r| route_open(w, seller, buyer, r).is_ok())
        .map(Ok).unwrap_or_else(|| plan(w, seller, buyer));
    let route = match route {
        Ok(route) => route,
        Err(reason) => return refused(&reason),
    };
    w.logistics.route_cache.insert(key, route.clone());
    let (route, alternate) = freight_routing::select(w, seller, buyer, route,
        requested * tonnes_per_unit, &w.logistics.usage_tonnes, None, None);
    let remaining = route.segments.iter().map(|segment| {
        (route_segment_capacity(w, segment).unwrap_or(route.capacity_tonnes)
            - w.logistics.usage_tonnes.get(segment).copied().unwrap_or(0.0)).max(0.0)
    }).fold(f64::INFINITY, f64::min);
    // An exactly fitting final lot must not strand sub-quantum dust forever.
    let quantity = if requested * tonnes_per_unit <= remaining { requested }
        else { floor9((remaining / tonnes_per_unit).min(requested)) };
    for segment in &route.segments {
        *w.logistics.usage_tonnes.entry(segment.clone()).or_default() += quantity * tonnes_per_unit;
    }
    record_terminal_work(w, &route, quantity * tonnes_per_unit);
    let reason = freight_routing::dispatch_reason(w, alternate, quantity < requested, &route);
    Dispatch { quantity, route: Some(route), reason }
}

/// Saved manufactured cargo follows exactly the raw-cargo permission checks.
pub fn freight_route_open(w: &WorldState, seller: NationId, buyer: NationId, route: &RoutePlan) -> Result<(), String> {
    route_open(w, seller, buyer, route)
}

pub fn dispatch(
    w: &mut WorldState,
    seller: NationId,
    buyer: NationId,
    commodity: Commodity,
    requested: f64,
    source: ShipmentSource,
    contract: Option<u32>,
) -> Dispatch {
    dispatch_impl(w, seller, buyer, commodity, requested, source, contract, None, None)
}

pub(crate) fn dispatch_in_clearing(
    w: &mut WorldState, seller: NationId, buyer: NationId, commodity: Commodity,
    requested: f64, routes: &mut ClearingRoutes,
) -> Dispatch {
    dispatch_impl(w, seller, buyer, commodity, requested, ShipmentSource::Spot, None, Some(routes), None)
}

fn dispatch_impl(
    w: &mut WorldState, seller: NationId, buyer: NationId, commodity: Commodity,
    requested: f64, source: ShipmentSource, contract: Option<u32>, mut routes: Option<&mut ClearingRoutes>,
    preplanned: Option<Result<RoutePlan, String>>,
) -> Dispatch {
    if !enabled(w) {
        return Dispatch {
            quantity: requested.max(0.0),
            route: None,
            reason: None,
        };
    }
    if !requested.is_finite() || requested < 0.0 {
        return Dispatch {
            quantity: 0.0,
            route: None,
            reason: Some("Requested freight must be finite and non-negative.".into()),
        };
    }
    let now = resources::month_abs(w);
    let current = if crate::clock::is_daily(w) {
        w.logistics.last_day == Some(crate::clock::absolute_day(w))
    } else { w.logistics.last_month == Some(now) };
    if !current {
        return Dispatch {
            quantity: 0.0,
            route: None,
            reason: Some("The freight settlement has not begun for this tick.".into()),
        };
    }
    let policy = policy_for(w, buyer);
    let key = (seller, buyer, policy);
    let frozen = preplanned.is_some();
    let route = preplanned.unwrap_or_else(|| w
        .logistics
        .route_cache
        .get(&key)
        .cloned()
        .filter(|r| route_open(w, seller, buyer, r).is_ok())
        .map(Ok)
        .unwrap_or_else(|| match routes.as_mut() {
            Some(routes) => routes.plan(w, seller, buyer),
            None => plan(w, seller, buyer),
        }));
    let route = match route {
        Ok(route) => route,
        Err(reason) => return Dispatch {
            quantity: 0.0,
            route: None,
            reason: Some(reason),
        },
    };
    if !frozen { w.logistics.route_cache.insert(key, route.clone()); }
    let (route, alternate) = if frozen { (route, false) } else {
        let (capacities, budget) = match routes.as_mut() {
            Some(r) if !r.capacities.is_empty() => (Some(r.capacities.as_slice()), Some(&mut r.search_nodes_left)),
            _ => (None, None),
        };
        freight_routing::select(w, seller, buyer, route, requested * tonnes_per_unit(commodity), &w.logistics.usage_tonnes,
            capacities, budget)
    };
    if requested == 0.0 {
        return Dispatch {
            quantity: 0.0,
            route: Some(route),
            reason: None,
        };
    }
    let capacities = routes.as_ref().filter(|r| !r.capacities.is_empty()).map(|r| r.capacities.as_slice());
    let remaining_tonnes = route
        .segments
        .iter()
        .map(|s| {
            let cap = route_segment_capacity_with(w, s, capacities).unwrap_or(route.capacity_tonnes);
            (cap - w.logistics.usage_tonnes.get(s).copied().unwrap_or(0.0)).max(0.0)
        })
        .fold(f64::INFINITY, f64::min);
    let quantity = floor9(requested.min(remaining_tonnes / tonnes_per_unit(commodity)));
    if quantity <= 0.0 {
        return Dispatch {
            quantity: 0.0,
            route: Some(route),
            reason: Some(if w.rules.military_operations {
                "No available freight route was found for this lot; the remaining goods wait for the next settlement."
            } else { "Route capacity is fully committed for this tick." }.into()),
        };
    }
    let tonnes = quantity * tonnes_per_unit(commodity);
    for s in &route.segments {
        *w.logistics.usage_tonnes.entry(s.clone()).or_default() += tonnes;
    }
    record_terminal_work(w, &route, tonnes);
    let id = w.logistics.next_id;
    w.logistics.next_id = w.logistics.next_id.saturating_add(1);
    let dispatched_day = crate::clock::is_daily(w).then(|| crate::clock::absolute_day(w));
    w.logistics.cargo.push(Cargo {
        id,
        seller,
        buyer,
        commodity,
        quantity,
        source,
        contract,
        route: route.clone(),
        dispatched_month: now,
        due_month: now + route.months as i32,
        dispatched_day,
        due_day: dispatched_day.map(|day| day + route.estimated_days.max(1) as i32),
        hold_reason: None,
    });
    Dispatch {
        quantity,
        route: Some(route.clone()),
        reason: if w.rules.military_operations {
            freight_routing::dispatch_reason(w, alternate, quantity + EPS < requested, &route)
        } else if quantity + EPS < requested {
            Some("The route moved only what its shared freight capacity could carry.".into())
        } else {
            None
        },
    }
}

/// Execute an atomic contract service fraction against frozen route choices.
/// The resource ledger owns stock/cash; this function reserves capacity only.
pub(crate) fn dispatch_bundle(w: &mut WorldState,
    legs: &[(NationId, NationId, Commodity, f64)], stock_fraction: f64, contract: u32,
) -> (f64, Vec<Dispatch>) {
    let bundle = freight_routing::prepare_bundle(w, legs, &w.logistics.usage_tonnes);
    let service = stock_fraction.clamp(0.0, 1.0).min(bundle.ratio);
    let mut dispatches = Vec::with_capacity(legs.len());
    for (&(seller, buyer, commodity, quantity), route) in legs.iter().zip(bundle.routes) {
        let alternate = route.as_ref().is_ok_and(|r| r.dispatch_note.is_some());
        let mut dispatched = dispatch_impl(w, seller, buyer, commodity, quantity * service,
            ShipmentSource::Contract, Some(contract), None, Some(route));
        if alternate && dispatched.quantity > 0.0 {
            dispatched.reason = dispatched.route.as_ref().and_then(|route|
                freight_routing::dispatch_reason(w, true, service < 1.0, route));
        }
        dispatches.push(dispatched);
    }
    (bundle.ratio, dispatches)
}

fn route_segment_capacity(w: &WorldState, key: &str) -> Option<f64> {
    route_segment_capacity_with(w, key, None)
}

/// Frozen capacities are scoped to one spot clearing, whose infrastructure
/// and calendar cannot change. Live usage is still subtracted at every call.
fn route_segment_capacity_with(w: &WorldState, key: &str, capacities: Option<&[f64]>) -> Option<f64> {
    network()
        .edge_index
        .get(key)
        .map(|&i| capacities.map_or_else(|| segment_capacity(w, &network().edges[i]).0, |v| v[i]))
}

/// Fraction of all legs a physical bundle can dispatch together. Demand is
/// aggregated by shared edge before the ratio is chosen, so two legs using the
/// same bridge cannot each reserve the whole bridge in a preview. No state is
/// mutated; the caller dispatches every leg at this one ratio or none of them.
pub fn bundle_capacity_ratio(w: &WorldState, legs: &[(NationId, NationId, Commodity, f64)]) -> f64 {
    if !enabled(w) {
        return 1.0;
    }
    if w.rules.military_operations {
        return freight_routing::prepare_bundle(w, legs, &w.logistics.usage_tonnes).ratio;
    }
    let mut demand: BTreeMap<String, f64> = BTreeMap::new();
    for &(seller, buyer, commodity, quantity) in legs {
        if !quantity.is_finite() || quantity < 0.0 {
            return 0.0;
        }
        if quantity == 0.0 {
            continue;
        }
        let Ok(route) = plan(w, seller, buyer) else {
            return 0.0;
        };
        let tonnes = quantity * tonnes_per_unit(commodity);
        for segment in &route.segments {
            *demand.entry(segment.clone()).or_default() += tonnes;
        }
    }
    demand
        .into_iter()
        .map(|(segment, wanted)| {
            let capacity = route_segment_capacity(w, &segment).unwrap_or(0.0);
            let used = w
                .logistics
                .usage_tonnes
                .get(&segment)
                .copied()
                .unwrap_or(0.0);
            ((capacity - used).max(0.0) / wanted.max(EPS)).clamp(0.0, 1.0)
        })
        .fold(1.0, f64::min)
}

/// Pure next-settlement capacity preview for recurring contract bundles.
/// Every tuple is `(contract_id, physical_legs)` and the ids are served in
/// stable order, matching contract clearing. Unlike [`bundle_capacity_ratio`]
/// this starts with fresh edge use: a forecast made after today's dispatches
/// must not charge tomorrow for today's freight reservations. Each bundle is
/// atomic, and earlier bundles reserve their serviced share in the local
/// preview only; `WorldState` is never mutated.
pub fn fresh_contract_capacity_ratios(
    w: &WorldState,
    bundles: &[(u32, Vec<(NationId, NationId, Commodity, f64)>)],
) -> BTreeMap<u32, f64> {
    if !enabled(w) {
        return bundles.iter().map(|(id, _)| (*id, 1.0)).collect();
    }
    if w.rules.military_operations { return freight_routing::fresh_contract_ratios(w, bundles); }
    let mut ordered = bundles.to_vec();
    ordered.sort_by_key(|(id, _)| *id);
    let mut used: BTreeMap<String, f64> = BTreeMap::new();
    let mut result = BTreeMap::new();
    for (id, legs) in ordered {
        let mut demand: BTreeMap<String, f64> = BTreeMap::new();
        let mut valid = true;
        for (seller, buyer, commodity, quantity) in legs {
            if !quantity.is_finite() || quantity < 0.0 {
                valid = false;
                break;
            }
            if quantity == 0.0 {
                continue;
            }
            let Ok(route) = plan(w, seller, buyer) else {
                valid = false;
                break;
            };
            let tonnes = quantity * tonnes_per_unit(commodity);
            for segment in route.segments {
                *demand.entry(segment).or_default() += tonnes;
            }
        }
        let ratio = if !valid {
            0.0
        } else {
            demand
                .iter()
                .map(|(segment, wanted)| {
                    let capacity = route_segment_capacity(w, segment).unwrap_or(0.0);
                    ((capacity - used.get(segment).copied().unwrap_or(0.0)).max(0.0)
                        / wanted.max(EPS))
                    .clamp(0.0, 1.0)
                })
                .fold(1.0, f64::min)
        };
        if valid && ratio > 0.0 {
            for (segment, wanted) in demand {
                *used.entry(segment).or_default() += wanted * ratio;
            }
        }
        result.insert(id, ratio);
    }
    result
}

pub fn begin_month(w: &mut WorldState) -> Vec<Cargo> {
    if !enabled(w) {
        return vec![];
    }
    let now = resources::month_abs(w);
    let daily = crate::clock::is_daily(w);
    let today = crate::clock::absolute_day(w);
    if if daily { w.logistics.last_day == Some(today) }
        else { w.logistics.last_month == Some(now) } {
        return vec![];
    }
    w.logistics.last_month = Some(now);
    if daily { w.logistics.last_day = Some(today); }
    w.logistics.usage_tonnes.clear();
    w.logistics.arrivals.clear();
    w.logistics.route_cache.clear();
    let old = std::mem::take(&mut w.logistics.cargo);
    let mut keep = vec![];
    let mut arrivals = vec![];
    // Repeated consignments may share a booked path. Control and permissions
    // do not change during this arrival pass, so one exact path check suffices.
    let mut open_routes: BTreeMap<(NationId, NationId, Vec<String>), Result<(), String>> = BTreeMap::new();
    for mut c in old {
        if daily && c.due_day.is_none() {
            // Legacy freight arrived at the END of due_month: preserve that
            // known boundary, rather than adding months to the load date.
            // Its exact dispatch day was never stored and stays unknown.
            let year = 1990 + c.due_month.div_euclid(12);
            let month = c.due_month.rem_euclid(12) as u32 + 1;
            c.due_day = Some(crate::clock::date_day(year, month,
                crate::world::days_in_month(year, month)));
        }
        let in_transit = if daily { c.due_day.is_some_and(|due| due > today) }
            else { c.due_month > now };
        if in_transit {
            if w.rules.military_operations {
                let key = (c.seller, c.buyer, c.route.nodes.iter().map(|n| n.id.clone()).collect());
                c.hold_reason = open_routes.entry(key).or_insert_with(|| route_open(w, c.seller, c.buyer, &c.route)).clone().err();
            }
            keep.push(c);
            continue;
        }
        match route_open(w, c.seller, c.buyer, &c.route) {
            Ok(()) => {
                c.hold_reason = None;
                arrivals.push(c)
            }
            Err(reason) => {
                c.hold_reason = Some(reason);
                keep.push(c)
            }
        }
    }
    arrivals.sort_by_key(|c| c.id);
    keep.sort_by_key(|c| c.id);
    w.logistics.cargo = keep;
    w.logistics.arrivals = arrivals.clone();
    arrivals
}

pub fn pending(w: &WorldState, buyer: NationId, c: Commodity) -> f64 {
    w.logistics
        .cargo
        .iter()
        .filter(|x| x.buyer == buyer && x.commodity == c)
        .map(|x| x.quantity)
        .sum()
}

/// Paid cargo that can actually arrive within `days` from the current
/// simulation date. Presently closed routes are not secured supply; a stale
/// held receipt is ignored once that same route is open again, matching the
/// next arrival pass which clears the hold.
/// Old monthly cargo has no absolute due day, so its remaining due-month
/// distance is conservatively compared on the policy calendar (12/365).
pub fn pending_within_days(
    w: &WorldState,
    buyer: NationId,
    c: Commodity,
    days: i32,
) -> f64 {
    if days <= 0 {
        return 0.0;
    }
    let forecast_start = resources::forecast_start_day(w);
    let forecast_end = forecast_start.saturating_add(days.saturating_sub(1));
    w.logistics
        .cargo
        .iter()
        .filter(|cargo| cargo.buyer == buyer && cargo.commodity == c)
        .filter(|cargo| route_open(w, cargo.seller, cargo.buyer, &cargo.route).is_ok())
        .filter(|cargo| match cargo.due_day {
            Some(due) => due <= forecast_end,
            None => {
                // Legacy cargo arrived at the end of its due month. Recreate
                // that known conservative boundary without inventing an
                // original dispatch date.
                let year = 1990 + cargo.due_month.div_euclid(12);
                let month = cargo.due_month.rem_euclid(12) as u32 + 1;
                let due = crate::clock::date_day(
                    year,
                    month,
                    crate::world::days_in_month(year, month),
                );
                due <= forecast_end
            }
        })
        .map(|cargo| cargo.quantity)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::init::world_1990;
    use crate::world::GameRules;
    #[test]
    #[ignore = "observer-only Materials freight-capacity microprofile"]
    fn materials_segment_capacity_profile() {
        use std::{hint::black_box, time::Instant};
        let mut w = world_1990(GameRules {
            seed: 42, daily_simulation: true, economic_competition: true,
            production_system: true, resource_market: true, manufacturing_system: true,
            physical_logistics: true, logistics_routes: true, ..GameRules::default()
        });
        crate::starting_industry::enable_new_world(&mut w).unwrap();
        crate::province_economy::enable(&mut w);
        for _ in 0..31 { crate::tick_day(&mut w, &[]); }
        // Observe the actual day-32 settlement before date advance, so the
        // month fraction and edge reservations are those used by dispatch.
        if w.day <= 1 { w.headlines.clear(); }
        w.reindex();
        crate::province_economy::begin_day(&mut w);
        crate::programs::begin_day(&mut w);
        crate::production::tick_day(&mut w);
        for (name, tick) in crate::SYSTEMS {
            tick(&mut w);
            if *name == "arsenal" { break; }
        }
        let today = crate::clock::absolute_day(&w);
        let dispatches = w.logistics.cargo.iter()
            .filter(|c| c.dispatched_day == Some(today)).count();
        // Include all planned routes, including zero-capacity attempts that
        // never create cargo; successful fills alone undercount the hot path.
        let routes: Vec<_> = w.logistics.route_cache.values().collect();
        assert!(!routes.is_empty());
        let segments: usize = routes.iter().map(|r| r.segments.len()).sum();
        let before = crate::save(&w);
        let start = Instant::now();
        let cold_plans: Vec<_> = w.logistics.route_cache.keys().map(|&(seller, buyer, _)|
            plan(black_box(&w), seller, buyer).unwrap()).collect();
        let all_cold_plans_seconds = start.elapsed().as_secs_f64();
        for (cold, cached) in cold_plans.iter().zip(&routes) { assert_eq!(cold, *cached); }
        let repetitions = 100;
        let start = Instant::now();
        let mut checksum = 0.0;
        for _ in 0..repetitions {
            for route in &routes {
                checksum += route.segments.iter().map(|s| {
                    let cap = route_segment_capacity(black_box(&w), black_box(s))
                        .unwrap_or(route.capacity_tonnes);
                    (cap - w.logistics.usage_tonnes.get(s).copied().unwrap_or(0.0)).max(0.0)
                }).fold(f64::INFINITY, f64::min);
            }
        }
        black_box(checksum);
        let uncached_seconds = start.elapsed().as_secs_f64();
        // This is a diagnostic local immutable cache only, not production
        // behavior. Every live usage lookup and subtraction still occurs.
        let start = Instant::now();
        let capacities: BTreeMap<_, _> = routes.iter().flat_map(|r| &r.segments)
            .map(|s| (s.as_str(), route_segment_capacity(&w, s))).collect();
        let cache_build_seconds = start.elapsed().as_secs_f64();
        let start = Instant::now();
        let mut cached_checksum = 0.0;
        for _ in 0..repetitions {
            for route in &routes {
                cached_checksum += route.segments.iter().map(|s| {
                    let cap = capacities[s.as_str()].unwrap_or(route.capacity_tonnes);
                    (cap - w.logistics.usage_tonnes.get(s).copied().unwrap_or(0.0)).max(0.0)
                }).fold(f64::INFINITY, f64::min);
            }
        }
        black_box(cached_checksum);
        let cached_seconds = start.elapsed().as_secs_f64();
        assert_eq!(checksum.to_bits(), cached_checksum.to_bits());
        assert_eq!(before, crate::save(&w));
        eprintln!("MATERIALS_SEGMENT_PROFILE routes={} actual_dispatches={} segment_visits={} unique_segments={} repetitions={} uncached_s={:.6} cached_s={:.6} cache_build_s={:.6} uncached_one_pass_ms={:.6} cached_one_pass_ms={:.6} all_cold_plans_s={:.6}",
            routes.len(), dispatches, segments, capacities.len(), repetitions, uncached_seconds,
            cached_seconds, cache_build_seconds, uncached_seconds * 1000.0 / repetitions as f64,
            cached_seconds * 1000.0 / repetitions as f64, all_cold_plans_seconds);
    }
    #[test]
    fn clearing_tree_pauses_resumes_and_keeps_first_settled_goal() {
        let mut w = world();
        w.rules.military_operations = true;
        // One stable permission mask lets these destinations share a search.
        w.sanctions.clear();
        w.conflicts.clear();
        let before = crate::save(&w);
        let mut trees = ClearingRoutes::new(&w);
        let seller = NationId::Netherlands;
        let near = NationId::Belgium;
        let first = trees.plan(&w, seller, near).unwrap();
        assert_eq!(first, plan_impl(&w, seller, near, false).unwrap());
        assert_eq!(trees.trees.len(), 1);
        let tree = trees.trees.values().next().unwrap();
        let paused_rank = tree.next_rank;
        assert!(paused_rank < network().nodes.len(), "a nearby destination cannot exhaust the whole graph");
        assert!(!tree.queue.is_empty(), "later destinations retain a resumable frontier");
        let previous = tree.prev.clone();
        let settled = tree.settled.clone();
        // Japan forces a resume; USA/France have multiple owned goal nodes;
        // the final Belgium request must return its original earliest goal.
        for buyer in [NationId::Japan, NationId::USA, NationId::France, near] {
            assert_eq!(trees.plan(&w, seller, buyer), plan_impl(&w, seller, buyer, false));
        }
        assert_eq!(trees.trees.len(), 1);
        let tree = trees.trees.values().next().unwrap();
        assert!(tree.next_rank > paused_rank);
        for (i, rank) in settled.into_iter().enumerate().filter(|(_, rank)| *rank != usize::MAX) {
            assert_eq!(tree.settled[i], rank);
            assert_eq!(tree.prev[i], previous[i], "resuming cannot rewrite a settled predecessor");
        }
        assert_eq!(trees.plan(&w, seller, near).unwrap(), first);
        assert_eq!(crate::save(&w), before);
    }

    #[test]
    fn clearing_tree_exhaustion_preserves_unreachable_and_blocked_results() {
        let mut w = world();
        w.rules.military_operations = true;
        w.sanctions.clear();
        w.conflicts.clear();
        for buyer in [NationId::Japan, NationId::Belgium] {
            w.logistics.policies.insert(buyer, RoutePolicy::LandOnly);
        }
        let mut trees = ClearingRoutes::new(&w);
        let seller = NationId::Netherlands;
        let unavailable = trees.plan(&w, seller, NationId::Japan);
        assert!(unavailable.is_err(), "Japan has no all-land route from the Netherlands");
        assert_eq!(unavailable, plan_impl(&w, seller, NationId::Japan, false));
        assert_eq!(trees.trees.len(), 1);
        assert!(trees.trees.values().next().unwrap().queue.is_empty());
        let exhausted_rank = trees.trees.values().next().unwrap().next_rank;
        assert_eq!(trees.plan(&w, seller, NationId::Belgium), plan_impl(&w, seller, NationId::Belgium, false));
        assert_eq!(trees.plan(&w, seller, NationId::Japan), unavailable);
        assert_eq!(trees.trees.values().next().unwrap().next_rank, exhausted_rank);
        w.sanctions.push((seller, NationId::Belgium));
        let mut blocked = ClearingRoutes::new(&w);
        assert_eq!(blocked.plan(&w, seller, NationId::Belgium), plan_impl(&w, seller, NationId::Belgium, false));
        assert!(blocked.trees.is_empty(), "blocked endpoints preserve the original early refusal");
    }

    #[test]
    fn clearing_frozen_capacities_preserve_live_usage_and_dispatches() {
        let mut w = world();
        w.rules.daily_simulation = true;
        w.rules.military_operations = true;
        w.sanctions.clear();
        w.conflicts.clear();
        begin_month(&mut w);
        let mut cached_routes = ClearingRoutes::new(&w);
        for (seller, buyer) in [(NationId::Germany, NationId::France), (NationId::USA, NationId::Japan)] {
            for (commodity, quantity) in [(Commodity::Iron, 10.0), (Commodity::Coal, 1e8), (Commodity::Copper, 1.0)] {
                let nominal = plan(&w, seller, buyer).unwrap();
                // Keep the same real usage, search budget, heap order and
                // input lot. Only the frozen capacity reads differ.
                let mut cached_budget = cached_routes.search_nodes_left;
                let mut plain_budget = cached_budget;
                let a = freight_routing::select(&w, seller, buyer, nominal.clone(), quantity * tonnes_per_unit(commodity),
                    &w.logistics.usage_tonnes, Some(&cached_routes.capacities), Some(&mut cached_budget));
                let b = freight_routing::select(&w, seller, buyer, nominal, quantity * tonnes_per_unit(commodity),
                    &w.logistics.usage_tonnes, None, Some(&mut plain_budget));
                assert_eq!(a, b);
                assert_eq!(cached_budget, plain_budget);
                for edge in &a.0.segments {
                    assert_eq!(route_segment_capacity_with(&w, edge, Some(&cached_routes.capacities)),
                        route_segment_capacity(&w, edge));
                }
                dispatch_in_clearing(&mut w, seller, buyer, commodity, quantity, &mut cached_routes);
            }
        }
        assert!(!w.logistics.usage_tonnes.is_empty());
    }

    #[test]
    fn clearing_trees_match_original_pair_routes_across_policy_and_ownership() {
        let ids = [NationId::USA, NationId::Japan, NationId::Netherlands, NationId::Belgium,
            NationId::Germany, NationId::France, NationId::India, NationId::Brazil,
            NationId::Iraq, NationId::Kuwait, NationId::Tonga, NationId::Canada];
        let mut total = 0;
        for seed in [42, 71] {
            let mut w = world_1990(GameRules {
                seed, daily_simulation: true, resource_market: true,
                logistics_routes: true, physical_logistics: true, ..GameRules::default()
            });
            for scenario in 0..3 {
                if scenario == 1 {
                    w.sanctions.extend([(NationId::India, NationId::Germany),
                        (NationId::Germany, NationId::Japan), (NationId::France, NationId::USA)]);
                }
                if scenario == 2 {
                    crate::war::declare_war(&mut w, NationId::Iraq, NationId::Kuwait).unwrap();
                    let district = w.districts.iter().find(|(_, owner)| **owner == NationId::Kuwait).unwrap().0.clone();
                    let conflict = w.conflict_between(NationId::Iraq, NationId::Kuwait).unwrap().id;
                    w.conflict_mut(conflict).unwrap().front.insert(district, 1.0);
                    let ceded = w.districts.iter().find(|(_, owner)| **owner == NationId::France).unwrap().0.clone();
                    w.districts.insert(ceded, NationId::Germany);
                    w.nation_mut(NationId::Belgium).alive = false;
                }
                for policy in [RoutePolicy::Fastest, RoutePolicy::LandOnly, RoutePolicy::AvoidChokepoints] {
                    for &id in &ids { w.logistics.policies.insert(id, policy); }
                    let before = crate::save(&w);
                    let mut trees = ClearingRoutes::new(&w);
                    for &seller in &ids {
                        for &buyer in &ids {
                            assert_eq!(trees.plan(&w, seller, buyer), plan_impl(&w, seller, buyer, false),
                                "seed={seed} scenario={scenario} policy={policy:?} {seller:?}->{buyer:?}");
                            total += 1;
                        }
                    }
                    assert!(trees.trees.len() <= 128);
                    assert_eq!(crate::save(&w), before, "route search cannot mutate any saved state");
                }
            }
        }
        assert_eq!(total, 2 * 3 * 3 * ids.len() * ids.len());
    }

    #[test]
    fn clearing_tree_limit_falls_back_without_changing_routes() {
        let w = world();
        let mut trees = ClearingRoutes::new(&w);
        let ids: Vec<_> = crate::nations::all_nations().iter().copied()
            .filter(|&id| w.nation_opt(id).is_some_and(|n| n.alive)).collect();
        for seller in ids {
            let buyer = if seller == NationId::Japan { NationId::USA } else { NationId::Japan };
            assert_eq!(trees.plan(&w, seller, buyer), plan(&w, seller, buyer));
        }
        assert_eq!(trees.trees.len(), 128, "fixture reaches bounded-cache fallback");
    }

    #[test]
    fn indexed_lazy_route_search_is_identical_to_original_graph_walk() {
        let mut w = world();
        w.rules.daily_simulation = true;
        let pairs = [(NationId::USA,NationId::Japan),(NationId::Netherlands,NationId::Belgium),
            (NationId::Germany,NationId::France),(NationId::Japan,NationId::USA),
            (NationId::India,NationId::Germany),(NationId::Brazil,NationId::Japan)];
        for policy in [RoutePolicy::Fastest,RoutePolicy::LandOnly,RoutePolicy::AvoidChokepoints] {
            for &(seller,buyer) in &pairs {
                set_policy(&mut w,buyer,policy).unwrap();
                assert_eq!(plan_impl(&w,seller,buyer,true),plan_impl(&w,seller,buyer,false));
                w.sanctions.push((buyer,seller));
                assert_eq!(plan_impl(&w,seller,buyer,true),plan_impl(&w,seller,buyer,false));
                w.sanctions.pop();
            }
        }
        let a=NationId::Iraq;
        let b=NationId::Kuwait;
        crate::war::declare_war(&mut w,a,b).unwrap();
        let district=w.districts.iter().find(|(_,owner)|**owner==b).unwrap().0.clone();
        let conflict=w.conflict_between(a,b).unwrap().id;
        w.conflict_mut(conflict).unwrap().front.insert(district,1.0);
        for buyer in [a,b,NationId::India] {
            assert_eq!(plan_impl(&w,NationId::USA,buyer,true),plan_impl(&w,NationId::USA,buyer,false));
        }
        w.nation_mut(NationId::Germany).alive=false;
        assert_eq!(plan_impl(&w,NationId::Germany,NationId::France,true),plan_impl(&w,NationId::Germany,NationId::France,false));
    }
    fn world() -> WorldState {
        world_1990(GameRules {
            resource_market: true,
            logistics_routes: true,
            physical_logistics: true,
            ..GameRules::default()
        })
    }
    fn advance_month(w: &mut WorldState) {
        if w.month == 12 {
            w.month = 1;
            w.year += 1;
        } else {
            w.month += 1;
        }
    }
    #[test]
    fn daily_capacity_is_prorated_shared_and_resets_on_the_next_day() {
        let monthly = world();
        let month_capacity = plan(&monthly, NationId::Germany, NationId::France).unwrap().capacity_tonnes;
        let mut w = monthly;
        w.rules.daily_simulation = true;
        let day_capacity = plan(&w, NationId::Germany, NationId::France).unwrap().capacity_tonnes;
        assert!((day_capacity * 31.0 - month_capacity).abs() < 1e-8);
        begin_month(&mut w);
        let coal = dispatch(&mut w, NationId::Germany, NationId::France,
            Commodity::Coal, 1e8, ShipmentSource::Spot, None);
        assert!(coal.quantity > 0.0);
        let iron = dispatch(&mut w, NationId::Germany, NationId::France,
            Commodity::Iron, 1e8, ShipmentSource::Spot, None);
        assert!(iron.quantity < 0.000002, "coal and iron share one corridor, with only floor9 dust left");
        let usage = w.logistics.usage_tonnes.clone();
        begin_month(&mut w);
        assert_eq!(usage, w.logistics.usage_tonnes, "a retry cannot reset capacity");
        crate::clock::advance_date(&mut w);
        begin_month(&mut w);
        assert!(w.logistics.usage_tonnes.is_empty());
        assert!(dispatch(&mut w, NationId::Germany, NationId::France,
            Commodity::Iron, 1.0, ShipmentSource::Spot, None).quantity > 0.0);
    }

    #[test]
    fn daily_cargo_arrives_on_its_exact_day_and_only_once() {
        let mut w = world();
        w.rules.daily_simulation = true;
        w.year = 1992;
        w.month = 2;
        w.day = 27;
        begin_month(&mut w);
        let dispatched = crate::clock::absolute_day(&w);
        dispatch(&mut w, NationId::Japan, NationId::USA,
            Commodity::Iron, 10.0, ShipmentSource::Spot, None);
        let due = w.logistics.cargo[0].due_day.unwrap();
        assert_eq!(due - dispatched, w.logistics.cargo[0].route.estimated_days as i32);
        while crate::clock::absolute_day(&w) + 1 < due {
            crate::clock::advance_date(&mut w);
            assert!(begin_month(&mut w).is_empty());
        }
        let mut loaded = crate::load(&crate::save(&w)).unwrap();
        crate::clock::advance_date(&mut w);
        crate::clock::advance_date(&mut loaded);
        assert_eq!(crate::clock::absolute_day(&w), due);
        let arrived = begin_month(&mut w);
        assert_eq!(arrived, begin_month(&mut loaded));
        assert_eq!(arrived.len(), 1);
        assert_eq!(arrived[0].quantity, 10.0);
        assert!(w.logistics.cargo.is_empty());
        assert!(begin_month(&mut w).is_empty());
    }

    #[test]
    fn daily_cargo_is_held_then_resumes_without_losing_its_paid_quantity() {
        let mut w = world();
        w.rules.daily_simulation = true;
        begin_month(&mut w);
        dispatch(&mut w, NationId::Germany, NationId::France,
            Commodity::Copper, 10.0, ShipmentSource::Contract, Some(991));
        let due = w.logistics.cargo[0].due_day.unwrap();
        w.sanctions.push((NationId::France, NationId::Germany));
        while crate::clock::absolute_day(&w) < due {
            crate::clock::advance_date(&mut w);
            assert!(begin_month(&mut w).is_empty());
        }
        assert!(w.logistics.cargo[0].hold_reason.is_some());
        assert_eq!(pending(&w, NationId::France, Commodity::Copper), 10.0);
        w.sanctions.clear();
        crate::clock::advance_date(&mut w);
        let arrivals = begin_month(&mut w);
        assert_eq!(arrivals.len(), 1);
        assert_eq!(arrivals[0].quantity, 10.0);
        assert_eq!(pending(&w, NationId::France, Commodity::Copper), 0.0);
    }

    #[test]
    fn old_monthly_cargo_migrates_its_remaining_term_without_dispatching_again() {
        let mut w = world();
        begin_month(&mut w);
        dispatch(&mut w, NationId::Germany, NationId::France,
            Commodity::Copper, 10.0, ShipmentSource::Spot, None);
        assert_eq!(w.logistics.cargo[0].due_day, None);
        let old_id = w.logistics.cargo[0].id;
        w.rules.daily_simulation = true;
        begin_month(&mut w);
        assert_eq!(w.logistics.cargo.len(), 1);
        assert_eq!(w.logistics.cargo[0].id, old_id);
        assert_eq!(w.logistics.cargo[0].quantity, 10.0);
        assert_eq!(w.logistics.cargo[0].due_day, Some(58));
        assert_eq!(w.logistics.cargo[0].dispatched_day, None);
        assert_eq!(w.logistics.next_id, old_id + 1);
    }
    #[test]
    fn network_is_real_and_connected() {
        let n = network();
        assert!(n.nodes.len() > 5_000);
        assert!(n.edges.len() > 15_000);
        assert!(n.nodes.iter().any(|x| x.id == "US-TX"));
        assert!(n.nodes.iter().any(|x| x.kind == "chokepoint"));
    }
    #[test]
    fn completed_freight_terminal_increases_only_its_real_gateway_capacity() {
        let mut w = world();
        w.rules.daily_simulation = true;
        let e = network().edges.iter().find(|e| e.kind == "terminal").unwrap();
        let district = if network().nodes[network().index[&e.a]].kind == "district" { &e.a } else { &e.b };
        assert!(has_terminal(district));
        assert!(!has_terminal("not-a-mapped-province"));
        let before = segment_capacity(&w,e).0;
        production::complete_capability(&mut w,district,production::ProjectKind::FreightTerminal);
        assert!((segment_capacity(&w,e).0-before*1.25).abs()<1e-8);
        let sea = network().edges.iter().find(|e|e.kind=="sea").unwrap();
        assert!((segment_capacity(&w,sea).0-75_000.0*crate::clock::month_fraction(&w)).abs()<1e-8);
    }
    #[test]
    fn company_terminal_capacity_and_fees_follow_actual_dispatch() {
        use crate::companies::{self, CompanySector, CompanyTarget};
        let mut w = world();
        w.rules.daily_simulation = true;
        companies::enable(&mut w);
        let buyer = NationId::USA;
        let seller = NationId::Japan;
        let route = plan(&w, seller, buyer).unwrap();
        let edge = route.segments.iter().filter_map(|s| network().edge_index.get(s))
            .map(|i| &network().edges[*i]).find(|e| e.kind == "terminal"
                && [&e.a,&e.b].iter().any(|d| w.districts.get(*d) == Some(&buyer))).unwrap();
        let district = [&edge.a,&edge.b].into_iter().find(|d| w.districts.get(*d) == Some(&buyer)).unwrap().clone();
        production::complete_capability(&mut w, &district, production::ProjectKind::FreightTerminal);
        let baseline = segment_capacity(&w, edge).0;
        let company = w.companies.roster.iter().find(|c| c.nation == buyer && c.sector == CompanySector::Logistics).unwrap().clone();
        let target = CompanyTarget::Facility {district,sector:CompanySector::Logistics};
        companies::assign(&mut w,buyer,company.id,target.clone()).unwrap();
        assert!((segment_capacity(&w,edge).0-baseline*company.modifiers().work_rate).abs()<1e-8);
        let sea = network().edges.iter().find(|e| e.kind == "sea").unwrap();
        assert!((segment_capacity(&w,sea).0-75_000.0*crate::clock::month_fraction(&w)).abs()<1e-8);
        assert_eq!(w.companies.roster.iter().find(|c|c.id==company.id).unwrap().total_fees_bn,0.0);
        begin_month(&mut w);
        w.nation_mut(buyer).treasury_bn=Some(100.0);
        w.nation_mut(buyer).debt_bn=Some(0.0);
        let dispatched=dispatch_impl(&mut w,seller,buyer,Commodity::Iron,100.0,ShipmentSource::Spot,None,None,Some(Ok(route)));
        assert!(dispatched.quantity>0.0);
        let receipt=w.companies.assignments.iter().find(|a|a.company_id==company.id).unwrap();
        let fee=dispatched.quantity*tonnes_per_unit(Commodity::Iron)*COMPANY_HANDLING_BN_PER_TONNE*company.fee_rate;
        assert!((receipt.fees_today_bn-fee).abs()<1e-14);
        assert!((w.nation(buyer).treasury_bn.unwrap()-(100.0-fee)).abs()<1e-12);
        let copy=crate::save(&w);
        let _=plan(&w,seller,buyer);
        assert_eq!(copy,crate::save(&w),"capacity previews never charge fees");
        companies::unassign(&mut w,buyer,&target).unwrap();
        assert!((segment_capacity(&w,edge).0-baseline).abs()<1e-8);
    }
    #[test]
    fn land_only_is_a_real_policy() {
        let mut w = world();
        set_policy(&mut w, NationId::France, RoutePolicy::LandOnly).unwrap();
        let p = plan(&w, NationId::Germany, NationId::France).unwrap();
        assert_eq!(p.mode, "land");
        assert!(!p.nodes.is_empty());
    }
    #[test]
    fn ocean_freight_takes_time_and_is_idempotent() {
        let mut w = world();
        assert!(begin_month(&mut w).is_empty());
        let d = dispatch(
            &mut w,
            NationId::Japan,
            NationId::USA,
            Commodity::Iron,
            10.,
            ShipmentSource::Spot,
            None,
        );
        assert!(d.quantity > 0.);
        assert!(pending(&w, NationId::USA, Commodity::Iron) > 0.);
        assert!(begin_month(&mut w).is_empty());
        assert_eq!(w.logistics.cargo.len(), 1);
    }
    #[test]
    fn every_commodity_shares_the_same_capacity() {
        let mut w = world();
        assert!(begin_month(&mut w).is_empty());
        let a = dispatch(
            &mut w,
            NationId::Germany,
            NationId::France,
            Commodity::Coal,
            1e9,
            ShipmentSource::Spot,
            None,
        );
        let b = dispatch(
            &mut w,
            NationId::Germany,
            NationId::France,
            Commodity::Iron,
            1e9,
            ShipmentSource::Spot,
            None,
        );
        assert!(a.quantity > 0.);
        assert_eq!(b.quantity, 0.);
    }
    #[test]
    fn bilateral_sanctions_close_a_route() {
        let mut w = world();
        w.sanctions.push((NationId::USA, NationId::Japan));
        assert!(plan(&w, NationId::Japan, NationId::USA).is_err());
    }
    #[test]
    fn overseas_land_only_is_unreachable() {
        let mut w = world();
        set_policy(&mut w, NationId::USA, RoutePolicy::LandOnly).unwrap();
        assert!(plan(&w, NationId::Japan, NationId::USA).is_err());
    }
    #[test]
    fn built_infrastructure_grows_land_capacity() {
        let mut w = world();
        set_policy(&mut w, NationId::Belgium, RoutePolicy::LandOnly).unwrap();
        let before = plan(&w, NationId::Netherlands, NationId::Belgium)
            .unwrap()
            .capacity_tonnes;
        let districts: Vec<String> = w
            .districts
            .iter()
            .filter(|(_, owner)| **owner == NationId::Netherlands || **owner == NationId::Belgium)
            .map(|(district, _)| district.clone())
            .collect();
        for district in districts {
            w.production
                .provinces
                .push(production::ProvinceCapabilities {
                    district,
                    infrastructure: 5,
                    civilian_industry: 0,
                    power_grid: 0,
                    research_centers: 0,
                    arms_plants: 0,
                });
        }
        w.production
            .provinces
            .sort_by(|a, b| a.district.cmp(&b.district));
        let after = plan(&w, NationId::Netherlands, NationId::Belgium)
            .unwrap()
            .capacity_tonnes;
        assert!(after > before, "{before} -> {after}");
    }
    #[test]
    fn held_cargo_survives_save_and_arrives_once_after_reopening() {
        let mut w = world();
        begin_month(&mut w);
        assert!(
            dispatch(
                &mut w,
                NationId::Germany,
                NationId::France,
                Commodity::Copper,
                10.0,
                ShipmentSource::Contract,
                Some(9),
            )
            .quantity
                > 0.0
        );
        w.sanctions.push((NationId::France, NationId::Germany));
        advance_month(&mut w);
        assert!(begin_month(&mut w).is_empty());
        assert!(w.logistics.cargo[0].hold_reason.is_some());
        let mut loaded = crate::load(&crate::save(&w)).unwrap();
        assert_eq!(loaded.logistics, w.logistics);
        loaded.sanctions.clear();
        advance_month(&mut loaded);
        assert_eq!(begin_month(&mut loaded).len(), 1);
        assert!(begin_month(&mut loaded).is_empty());
        assert!(loaded.logistics.cargo.is_empty());
    }
    #[test]
    fn disabled_world_stays_empty() {
        let mut w = world_1990(GameRules::default());
        let d = dispatch(
            &mut w,
            NationId::Japan,
            NationId::USA,
            Commodity::Iron,
            3.,
            ShipmentSource::Spot,
            None,
        );
        assert_eq!(d.quantity, 3.);
        assert!(w.logistics.is_empty());
    }

    #[test]
    fn military_routing_uses_an_alternate_after_the_fastest_corridor_fills() {
        let mut w = world();
        w.rules.daily_simulation = true;
        w.rules.military_operations = true;
        set_policy(&mut w, NationId::France, RoutePolicy::LandOnly).unwrap();
        begin_month(&mut w);
        let first = dispatch(&mut w, NationId::Germany, NationId::France,
            Commodity::Coal, 1e8, ShipmentSource::Spot, None);
        let second = dispatch(&mut w, NationId::Germany, NationId::France,
            Commodity::Iron, 1.0, ShipmentSource::Spot, None);
        assert!(second.quantity > 0.5, "an open alternate corridor must carry freight, got {}", second.quantity);
        assert_ne!(first.route.unwrap().segments, second.route.unwrap().segments);
        for (edge, used) in &w.logistics.usage_tonnes {
            assert!(*used <= route_segment_capacity(&w, edge).unwrap() + EPS);
        }
    }

    #[test]
    fn military_routing_all_search_paths_refuse_a_contested_endpoint() {
        let mut w = world();
        w.rules.military_operations = true;
        crate::war::declare_war(&mut w, NationId::Iraq, NationId::Kuwait).unwrap();
        let cid = w.conflict_between(NationId::Iraq, NationId::Kuwait).unwrap().id;
        let districts: Vec<_> = w.districts.iter().filter(|(_,n)| **n == NationId::Kuwait).map(|(d,_)| d.clone()).collect();
        for district in districts { w.conflict_mut(cid).unwrap().front.insert(district, 0.0); }
        let original = plan_impl(&w, NationId::USA, NationId::Kuwait, false);
        assert!(original.is_err());
        assert_eq!(plan(&w, NationId::USA, NationId::Kuwait), original);
        let mut trees = ClearingRoutes::new(&w);
        assert_eq!(trees.plan(&w, NationId::USA, NationId::Kuwait), original);
    }

    #[test]
    fn military_routing_alternates_replay_after_save_and_keep_actual_arrival_dates() {
        let mut w = world();
        w.rules.daily_simulation = true;
        w.rules.military_operations = true;
        set_policy(&mut w, NationId::France, RoutePolicy::LandOnly).unwrap();
        begin_month(&mut w);
        dispatch(&mut w, NationId::Germany, NationId::France, Commodity::Coal, 1e8, ShipmentSource::Spot, None);
        let mut resumed = crate::load(&crate::save(&w)).unwrap();
        // The live world's nominal route cache is populated; a save has none.
        let live = dispatch(&mut w, NationId::Germany, NationId::France, Commodity::Iron, 1.0, ShipmentSource::Spot, None);
        let replay = dispatch(&mut resumed, NationId::Germany, NationId::France, Commodity::Iron, 1.0, ShipmentSource::Spot, None);
        assert_eq!(live, replay);
        assert!(live.route.as_ref().unwrap().dispatch_note.is_some());
        assert_eq!(crate::save(&w), crate::save(&resumed));
        let cargo = w.logistics.cargo.last().unwrap();
        assert_eq!(cargo.due_day.unwrap() - cargo.dispatched_day.unwrap(), cargo.route.estimated_days as i32);
        let saved_note = cargo.route.dispatch_note.clone();
        let due = cargo.due_day.unwrap();
        let id = cargo.id;
        while crate::clock::absolute_day(&w) < due {
            crate::clock::advance_date(&mut w);
            let arrivals = begin_month(&mut w);
            if crate::clock::absolute_day(&w) < due { assert!(arrivals.iter().all(|c| c.id != id)); }
            else { assert_eq!(arrivals.iter().find(|c| c.id == id).unwrap().route.dispatch_note, saved_note); }
        }
        assert!(begin_month(&mut w).is_empty());
    }

    #[test]
    fn military_routing_contract_forecasts_and_frozen_barter_legs_share_capacity() {
        let mut w = world();
        w.rules.daily_simulation = true;
        w.rules.military_operations = true;
        set_policy(&mut w, NationId::France, RoutePolicy::LandOnly).unwrap();
        begin_month(&mut w);
        let legs = vec![(NationId::Germany, NationId::France, Commodity::Iron, 100_000.0),
            (NationId::Germany, NationId::France, Commodity::Copper, 50_000.0)];
        let before = crate::save(&w);
        let forecast = fresh_contract_capacity_ratios(&w, &[(2, legs.clone()), (1, legs.clone())]);
        assert_eq!(crate::save(&w), before, "forecast is read-only");
        for id in 1..=2 {
            let ratio = bundle_capacity_ratio(&w, &legs);
            let (actual, dispatches) = dispatch_bundle(&mut w, &legs, 1.0, id);
            assert!((actual - ratio).abs() < 1e-12);
            assert!((forecast[&id] - actual).abs() < 1e-9);
            assert!(actual > 0.0 && actual < 1.0);
            for (d, leg) in dispatches.iter().zip(&legs) {
                assert!((d.quantity / leg.3 - actual).abs() < 1e-12, "barter legs have one service fraction");
            }
        }
        for (edge, used) in &w.logistics.usage_tonnes {
            assert!(*used <= route_segment_capacity(&w, edge).unwrap() + 1e-8);
        }
        assert!(w.logistics.cargo.iter().any(|c| c.route.dispatch_note.is_some()));
    }

    #[test]
    fn military_routing_goods_use_alternates_and_an_exhausted_graph_moves_nothing() {
        let mut w = world();
        w.rules.daily_simulation = true;
        w.rules.military_operations = true;
        set_policy(&mut w, NationId::France, RoutePolicy::LandOnly).unwrap();
        begin_month(&mut w);
        dispatch(&mut w, NationId::Germany, NationId::France, Commodity::Coal, 1e8, ShipmentSource::Spot, None);
        let goods = reserve_freight(&mut w, NationId::Germany, NationId::France, 1.0, 1.0);
        assert_eq!(goods.quantity, 1.0);
        assert!(goods.reason.unwrap().contains("alternate"));
        assert!(goods.route.unwrap().dispatch_note.is_some());
        for (edge, used) in &w.logistics.usage_tonnes {
            assert!(*used <= route_segment_capacity(&w, edge).unwrap() + EPS);
        }
        for edge in &network().edges { w.logistics.usage_tonnes.insert(edge_key(edge), segment_capacity(&w, edge).0); }
        let before = crate::save(&w);
        let blocked = dispatch(&mut w, NationId::Germany, NationId::France, Commodity::Iron, 1.0, ShipmentSource::Spot, None);
        assert_eq!(blocked.quantity, 0.0);
        assert!(blocked.reason.unwrap().contains("next settlement"));
        assert_eq!(crate::save(&w), before, "no goods, money or cargo are created");
        w.sanctions.push((NationId::Germany, NationId::France));
        assert!(dispatch(&mut w, NationId::Germany, NationId::France, Commodity::Iron, 1.0, ShipmentSource::Spot, None).route.is_none());
        set_policy(&mut w, NationId::USA, RoutePolicy::LandOnly).unwrap();
        assert!(reserve_freight(&mut w, NationId::Japan, NationId::USA, 1.0, 1.0).route.is_none());
    }

    #[test]
    fn military_routing_warns_of_a_hold_before_due_without_rebooking_paid_cargo() {
        let mut w = world();
        w.rules.daily_simulation = true;
        w.rules.military_operations = true;
        begin_month(&mut w);
        dispatch(&mut w, NationId::Japan, NationId::USA, Commodity::Iron, 1.0, ShipmentSource::Spot, None);
        let booked = w.logistics.cargo[0].clone();
        assert!(booked.route.estimated_days > 2);
        w.sanctions.push((NationId::USA, NationId::Japan));
        crate::clock::advance_date(&mut w);
        assert!(begin_month(&mut w).is_empty());
        assert!(w.logistics.cargo[0].hold_reason.as_ref().unwrap().contains("sanctions"));
        assert_eq!(w.logistics.cargo[0].route, booked.route);
        assert_eq!(w.logistics.cargo[0].due_day, booked.due_day);
        w.sanctions.clear();
        crate::clock::advance_date(&mut w);
        assert!(begin_month(&mut w).is_empty());
        assert!(w.logistics.cargo[0].hold_reason.is_none());
        assert_eq!(w.logistics.cargo[0].id, booked.id);
        assert_eq!(w.logistics.cargo[0].quantity, booked.quantity);
    }

    #[test]
    fn military_routing_replans_a_cached_departure_after_endpoint_cession() {
        let mut w = world();
        w.rules.daily_simulation = true;
        w.rules.military_operations = true;
        begin_month(&mut w);
        let original = dispatch(&mut w, NationId::Germany, NationId::France, Commodity::Iron, 1.0, ShipmentSource::Spot, None).route.unwrap();
        let source = original.nodes.first().unwrap().id.clone();
        w.districts.insert(source.clone(), NationId::France);
        assert!(route_open(&w, NationId::Germany, NationId::France, &original).is_err());
        let next = dispatch(&mut w, NationId::Germany, NationId::France, Commodity::Iron, 1.0, ShipmentSource::Spot, None);
        assert!(next.quantity > 0.0);
        assert_ne!(next.route.unwrap().nodes.first().unwrap().id, source);
        crate::clock::advance_date(&mut w);
        begin_month(&mut w);
        assert!(w.logistics.cargo.iter().find(|c| c.route == original).unwrap().hold_reason.is_some());
    }

    #[test]
    #[ignore = "bounded observer of the actual daily world; no timing threshold"]
    fn military_routing_daily_market_profile() {
        let days = std::env::var("SPHERES_MILITARY_PROFILE_DAYS").ok()
            .and_then(|v| v.parse::<u32>().ok()).unwrap_or(3).clamp(1, 45);
        for modern in [false, true] {
            let mut w = world();
            w.rules.daily_simulation = true;
            w.rules.military_operations = modern;
            w.rules.production_system = true;
            w.rules.manufacturing_system = true;
            w.rules.economic_competition = true;
            crate::starting_industry::enable_new_world(&mut w).unwrap();
            crate::province_economy::enable(&mut w);
            let started = std::time::Instant::now();
            for _ in 0..days { crate::tick_day(&mut w, &[]); }
            eprintln!("military_operations={modern} days={days} seconds={:.3} cargo={}", started.elapsed().as_secs_f64(), w.logistics.cargo.len());
        }
    }

    #[test]
    fn military_routing_search_budget_exhaustion_preserves_nominal_capacity() {
        let mut w = world();
        w.rules.daily_simulation = true;
        w.rules.military_operations = true;
        begin_month(&mut w);
        let nominal = plan(&w, NationId::Germany, NationId::France).unwrap();
        for edge in &nominal.segments {
            w.logistics.usage_tonnes.insert(edge.clone(), route_segment_capacity(&w, edge).unwrap());
        }
        let mut routes = ClearingRoutes::new(&w);
        routes.search_nodes_left = 1;
        let before = crate::save(&w);
        let d = dispatch_in_clearing(&mut w, NationId::Germany, NationId::France, Commodity::Iron, 1.0, &mut routes);
        assert_eq!(routes.search_nodes_left, 0);
        assert_eq!(d.quantity, 0.0);
        assert_eq!(d.route, Some(nominal));
        assert_eq!(crate::save(&w), before);
        let retry = dispatch_in_clearing(&mut w, NationId::Germany, NationId::France, Commodity::Iron, 1.0, &mut routes);
        assert_eq!(retry, d);
        assert_eq!(routes.search_nodes_left, 0);
    }
}
