//! Daily military sustainment SERVICES over the existing freight network.
//!
//! A service unit is one modeled strength-day of transport, handling, food and
//! maintenance support. It is NOT a magazine, fuel barrel or equipment item.
//! One unit occupies 20 modeled freight tonnes (a game capacity assumption,
//! not a historical unit conversion). National magazines and actual inventory
//! remain owned and debited exactly once by operations::Snapshot/equipment.
//!
//! Existing funded force structure supplies at most 1.1 service units per
//! strength per day, stored centrally for at most 14 days. No new money, GDP,
//! goods or opening historical stock is granted. Combat intensity raises the
//! demand for this already-funded service. Dispatch moves finite service from
//! its national hub into saved cargo, arrival moves it into a local buffer,
//! and daily use removes it there. Seven days of local service smooth a cut;
//! creating a sector never creates reserves. Hub placement is explicitly a
//! largest-population-province proxy, fixed until lost, not a historical depot.

use std::{cmp::Reverse, collections::{BTreeMap, BTreeSet, BinaryHeap}};
use serde::{Deserialize, Serialize};
use crate::{arsenal::{self, Class}, clock, control, districts, logistics, statecraft,
    world::{NationId, WorldState}};

pub const TONNES_PER_SERVICE: f64 = 20.0;
const LOCAL_DAYS: f64 = 7.0;
const SOURCE_DAYS: f64 = 14.0;
const EPS: f64 = 1e-9;

#[derive(Clone, Debug)]
pub struct SupplyRequest {
    pub key: String,
    pub nation: NationId,
    pub conflict: u32,
    pub district: String,
    pub deployed: f64,
    /// Normalized magazine burn is an intensity signal ONLY, never tonnes.
    pub burn_monthly: f64,
    pub sea_escort: f64,
    /// Opponent interdiction mission, 0..1.
    pub sea_denial: f64,
}
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct SupplyDelivery {
    pub key: String,
    pub coverage: f64,
    pub days: u32,
    pub route: Vec<String>,
    pub reason: String,
}
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Source {
    pub district: String,
    pub service: f64,
}
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct LocalBuffer {
    pub nation: NationId,
    pub conflict: u32,
    pub district: String,
    pub service: f64,
    #[serde(default)]
    pub demand: f64,
}
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct ServiceCargo {
    pub key: String,
    pub nation: NationId,
    pub conflict: u32,
    pub district: String,
    pub service: f64,
    pub route: Vec<String>,
    pub segments: Vec<String>,
    pub at_sea: bool,
    pub dispatched_day: i32,
    pub due_day: i32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hold_reason: Option<String>,
}
#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct SupplyState {
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub sources: BTreeMap<NationId, Source>,
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub buffers: BTreeMap<String, LocalBuffer>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub cargo: Vec<ServiceCargo>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_day: Option<i32>,
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub deliveries: BTreeMap<String, SupplyDelivery>,
}
impl SupplyState {
    pub fn is_empty(&self) -> bool {
        self.sources.is_empty() && self.buffers.is_empty() && self.cargo.is_empty()
            && self.last_day.is_none() && self.deliveries.is_empty()
    }
}

struct Graph {
    nodes: Vec<logistics::CampaignNode>,
    edges: Vec<logistics::CampaignEdge>,
    index: BTreeMap<String, usize>,
    adj: Vec<Vec<usize>>,
    commercial_usage: BTreeMap<String,f64>,
    resistance: Vec<f64>,
}
impl Graph {
    fn new(w: &WorldState) -> Self {
        let (nodes, edges) = logistics::campaign_graph(w);
        let index = nodes.iter().enumerate().map(|(i,n)| (n.id.clone(),i)).collect();
        let mut adj = vec![vec![]; nodes.len()];
        for (i,e) in edges.iter().enumerate() { adj[e.a].push(i); adj[e.b].push(i); }
        let resistance=nodes.iter().map(|n|n.district.as_deref()
            .and_then(|d|w.campaign_peace.occupation.get(d).filter(|o|
                w.districts.get(d)==Some(&o.owner) && o.owner!=o.occupier
                    && control::controller(w,d)==Some(o.occupier)))
            .map_or(1.0,|o|1.0-0.50*fraction(o.resistance)*(1.0-fraction(o.coverage)))).collect();
        Self { nodes, edges, index, adj, commercial_usage:w.logistics.usage_tonnes.clone(),resistance }
    }
}
struct Route {
    nodes: Vec<String>,
    edges: Vec<usize>,
    days: u32,
    sea: bool,
    capacity: f64,
}

fn positive(x: f64) -> f64 { if x.is_finite() { x.max(0.0) } else { 0.0 } }
fn fraction(x: f64) -> f64 { positive(x).min(1.0) }

fn remaining_capacity(w:&WorldState,g:&Graph,ei:usize,sea:f64)->f64 {
    let e=&g.edges[ei];
    let commercial=g.commercial_usage.get(&e.key).copied().unwrap_or(0.0);
    let military=(w.logistics.usage_tonnes.get(&e.key).copied().unwrap_or(0.0)-commercial).max(0.0);
    // The weakest occupied endpoint constrains this edge once. Route capacity
    // is then the minimum edge, not an exponential penalty per province.
    let resistance=g.resistance[e.a].min(g.resistance[e.b]);
    ((e.capacity_tonnes-commercial).max(0.0)*resistance*if e.sea {sea} else {1.0}-military).max(0.0)
}

/// Permission is local to the actual host, never inferred from friendship,
/// commercial openness, a defence pact or merely sharing a theatre.
fn allowed(w: &WorldState, nation: NationId, district: &str) -> bool {
    let Some(host) = control::controller(w, district) else { return false };
    if host == nation { return w.nation_opt(nation).is_some_and(|n| n.alive); }
    w.nation_opt(host).is_some_and(|n| n.alive)
        && !statecraft::belligerents(w, nation, host)
        && w.access.iter().any(|a| a.host == host && a.seeker == nation)
}

/// A sea mission cannot manufacture lift. Serviceable naval inventory is
/// required; air inventory improves escort but cannot replace a port or ship.
fn sea_factor(w: &WorldState, r: &SupplyRequest) -> f64 {
    let Some(n) = w.nation_opt(r.nation) else { return 0.0 };
    let (mut navy, mut air, mut total) = (0.0, 0.0, 0.0);
    for h in &n.arsenal.held {
        let Some(d) = arsenal::DECK.get(h.kit as usize) else { continue };
        let value = positive(arsenal::combat_value(n,h));
        total += value;
        if d.class == Class::Naval { navy += value; }
        // The installed custom aviation slice is tactical strike only; it has
        // no convoy interception or escort role, even when its bombs are paid.
        let strike_only = h.design_id.as_deref().and_then(|id| crate::equipment::profile(n,id))
            .is_some_and(|profile| profile.aviation.is_some());
        if d.class == Class::Air && !strike_only { air += value; }
    }
    if navy <= EPS || total <= EPS { return 0.0; }
    let lift = (navy / (total * 0.15)).clamp(0.0,1.0);
    let cover = (air / (total * 0.20)).clamp(0.0,1.0);
    let protection = fraction(r.sea_escort) * (0.6 + 0.4 * cover);
    lift * (1.0 - fraction(r.sea_denial) * (1.0 - 0.65 * protection)).clamp(0.0,1.0)
}

fn route(w: &WorldState, g: &Graph, r: &SupplyRequest, start: &str) -> Result<Route,String> {
    let source = *g.index.get(start).ok_or("No mapped national supply hub.")?;
    let goal = *g.index.get(&r.district).ok_or("The sector has no mapped freight destination.")?;
    let pass: Vec<bool> = g.nodes.iter().map(|n| n.district.as_deref()
        .is_none_or(|d| allowed(w,r.nation,d))).collect();
    if !pass[source] || !pass[goal] { return Err("The supply endpoint is contested, hostile or lacks military access.".into()); }
    let sea = sea_factor(w,r);
    let mut distances = vec![u64::MAX;g.nodes.len()];
    let mut previous = vec![None;g.nodes.len()];
    let mut q = BinaryHeap::new();
    distances[source] = 0;
    q.push(Reverse((0_u64,source)));
    while let Some(Reverse((cost,node))) = q.pop() {
        if cost != distances[node] { continue; }
        if node == goal { break; }
        for &ei in &g.adj[node] {
            let e = &g.edges[ei];
            let next = if e.a == node { e.b } else { e.a };
            let remaining = remaining_capacity(w,g,ei,sea);
            if !pass[next] || remaining <= EPS || (e.sea && sea <= EPS) { continue; }
            let new = cost.saturating_add(e.travel_weight);
            if new < distances[next] { distances[next] = new; previous[next] = Some((node,ei)); q.push(Reverse((new,next))); }
        }
    }
    if distances[goal] == u64::MAX { return Err("No open military supply route with spare freight capacity and lift.".into()); }
    let mut nodes = vec![g.nodes[goal].id.clone()];
    let mut edges = vec![];
    let mut at = goal;
    while let Some((p,e)) = previous[at] { edges.push(e); nodes.push(g.nodes[p].id.clone()); at=p; }
    nodes.reverse(); edges.reverse();
    let mut has_sea = false;
    for &ei in &edges {
        let e = &g.edges[ei];
        has_sea |= e.sea;
    }
    let capacity = edges.iter().map(|&ei|remaining_capacity(w,g,ei,sea)).fold(f64::INFINITY,f64::min);
    Ok(Route { nodes, edges, sea:has_sea, capacity,
        days: if source == goal { 1 } else { ((distances[goal]+1679)/1680) as u32 + 2 } })
}

fn cargo_open(w: &WorldState, g: &Graph, r: &SupplyRequest, cargo: &ServiceCargo) -> bool {
    (!cargo.at_sea || sea_factor(w,r) > EPS) && cargo.route.iter().all(|id| {
        g.index.get(id).is_some_and(|&i| g.nodes[i].district.as_deref().is_none_or(|d| allowed(w,r.nation,d)))
    })
}

fn choose_hub(w: &WorldState, g: &Graph, nation: NationId) -> Option<String> {
    let mut best: Option<(f64,String)> = None;
    for (d,&owner) in &w.districts {
        if owner != nation || !g.index.contains_key(d) || !control::can_operate(w,nation,d) { continue; }
        let population = districts::population_of(w,d).unwrap_or(0.0);
        if best.as_ref().is_none_or(|(p,_)| population > *p) { best = Some((population,d.clone())); }
    }
    best.map(|(_,d)|d)
}

/// Pure movement feasibility quote over the same military-access graph. Node
/// IDs include sea/gateway geometry, not only districts. No dispatch, service,
/// force or edge reservation is created by asking for a route.
pub fn deployment_route(w:&WorldState,nation:NationId,district:&str)->Option<(Vec<String>,u32)> {
    let g=Graph::new(w);
    let hub=w.campaign_supply.sources.get(&nation).filter(|s|control::can_operate(w,nation,&s.district))
        .map(|s|s.district.clone()).or_else(||choose_hub(w,&g,nation))?;
    let request=SupplyRequest {key:String::new(),nation,conflict:0,district:district.into(),deployed:0.0,
        burn_monthly:0.0,sea_escort:0.0,sea_denial:0.0};
    let path=route(w,&g,&request,&hub).ok()?;
    Some((path.nodes,path.days))
}

/// Validate the not-yet-traversed part of a troop journey. Already departed
/// origins must be removed by its owner; their later loss cannot trap a
/// withdrawal indefinitely. This uses live control, never commercial access.
pub fn deployment_path_open(w:&WorldState,nation:NationId,path:&[String])->bool {
    if path.is_empty() {return false;}
    if path.iter().any(|id|logistics::campaign_district(id).is_none_or(|district|district.is_some_and(|d|!allowed(w,nation,d)))) {return false;}
    if path.iter().any(|id|id.starts_with("sea:")) {
        let request=SupplyRequest {key:String::new(),nation,conflict:0,district:String::new(),deployed:0.0,
            burn_monthly:0.0,sea_escort:0.0,sea_denial:0.0};
        if sea_factor(w,&request)<=EPS {return false;}
    }
    true
}

/// Called once before conflicts are taken out of WorldState, AFTER commercial
/// freight has opened this day's shared settlement. Never opens/reset that
/// ledger itself, and never debits national ammunition or fuel.
pub fn prepare(w: &mut WorldState, requests: &[SupplyRequest]) -> BTreeMap<String,SupplyDelivery> {
    if !clock::is_daily(w) || !w.rules.military_operations { return BTreeMap::new(); }
    let today = clock::absolute_day(w);
    if w.campaign_supply.last_day == Some(today) { return w.campaign_supply.deliveries.clone(); }
    if requests.is_empty() && w.campaign_supply.is_empty() { return BTreeMap::new(); }
    let g = Graph::new(w);
    let mut state = std::mem::take(&mut w.campaign_supply);
    let mut rows = BTreeMap::new();
    for r in requests { rows.entry(r.key.clone()).or_insert(r); }
    state.buffers.retain(|key,b| rows.get(key).is_some_and(|r|
        r.nation == b.nation && r.conflict == b.conflict && r.district == b.district));
    // Demobilized or relocated services expire. They never teleport to another
    // sector or become market stock. Same identity requires the saved route.
    state.cargo.retain(|c| rows.get(&c.key).is_some_and(|r|
        r.nation == c.nation && r.conflict == c.conflict && r.district == c.district));
    for r in rows.values() {
        state.buffers.entry(r.key.clone()).or_insert_with(|| LocalBuffer {
            nation:r.nation, conflict:r.conflict, district:r.district.clone(), service:0.0,demand:0.0 });
    }
    let nations: BTreeSet<_> = rows.values().map(|r|r.nation).collect();
    for nation in nations {
        let strength = w.nation_opt(nation).filter(|n| n.alive).map_or(0.0,|n|positive(n.mil_strength));
        if state.sources.get(&nation).is_some_and(|s| !control::can_operate(w,nation,&s.district)) {
            state.sources.remove(&nation); // A captured hub cannot teleport its reserve.
        }
        if !state.sources.contains_key(&nation) {
            if let Some(district) = choose_hub(w,&g,nation) { state.sources.insert(nation,Source {district,service:0.0}); }
        }
        if let Some(s) = state.sources.get_mut(&nation) {
            s.service = (positive(s.service) + strength * 1.1).min(strength * SOURCE_DAYS);
        }
    }
    let mut in_transit = vec![];
    for mut cargo in state.cargo.drain(..) {
        let r = rows[&cargo.key];
        if !cargo_open(w,&g,r,&cargo) {
            cargo.hold_reason = Some("The booked military corridor is closed; services remain in transit.".into());
            cargo.due_day = cargo.due_day.max(today).saturating_add(1);
            in_transit.push(cargo);
        } else if cargo.due_day <= today {
            state.buffers.get_mut(&cargo.key).unwrap().service += positive(cargo.service);
        } else { cargo.hold_reason=None; in_transit.push(cargo); }
    }
    state.cargo = in_transit;
    let settlement_open = logistics::enabled(w) && w.logistics.last_day == Some(today);
    let mut deliveries = BTreeMap::new();
    let mut sea_dispatched: BTreeMap<NationId,f64> = BTreeMap::new();
    for (key,r) in rows {
        let strength = w.nation_opt(r.nation).map_or(0.0,|n|positive(n.mil_strength));
        let deployed = positive(r.deployed);
        let intensity = if deployed > EPS { (positive(r.burn_monthly)*strength/deployed).min(2.0) } else { 0.0 };
        let demand = deployed * (0.65 + 0.70 * intensity);
        let accessible = allowed(w,r.nation,&r.district);
        let b = state.buffers.get_mut(&key).unwrap();
        b.demand=demand;
        // A lost/contested destination cannot feed friendly troops. Service
        // stock is not a capturable ammunition inventory.
        if !accessible { b.service=0.0; }
        let consumed = if accessible { positive(b.service).min(demand) } else { 0.0 };
        b.service = (positive(b.service)-consumed).min(demand*LOCAL_DAYS);
        let coverage = if demand > EPS { consumed/demand } else { 1.0 };
        // A stranded consignment cannot suppress fresh dispatch over a legal
        // alternate corridor. It may arrive later, subject to local storage.
        let pending:f64 = state.cargo.iter().filter(|c|c.key==key && c.hold_reason.is_none()).map(|c|c.service).sum();
        let local = b.service;
        let mut delivery = SupplyDelivery { key:key.clone(), coverage, days:0, route:vec![], reason:String::new() };
        let planned = state.sources.get(&r.nation).ok_or_else(|| "No controlled national supply hub.".to_string())
            .and_then(|s| route(w,&g,r,&s.district));
        match planned {
            Ok(path) => {
                delivery.days=path.days;
                delivery.route=path.nodes.clone();
                // Pipeline stock covers travel time; the local reserve target
                // alone would permanently starve every route longer than 7 days.
                let want = (demand*(LOCAL_DAYS+path.days as f64)-local-pending).max(0.0);
                let lift_left = if path.sea {
                    (strength*0.30*sea_factor(w,r)-sea_dispatched.get(&r.nation).copied().unwrap_or(0.0)).max(0.0)
                } else { f64::INFINITY };
                let source = state.sources.get_mut(&r.nation).unwrap();
                let dispatched = if settlement_open { want.min(source.service).min(path.capacity/TONNES_PER_SERVICE).min(lift_left) } else { 0.0 };
                if dispatched > EPS {
                    source.service = (source.service-dispatched).max(0.0);
                    if path.sea { *sea_dispatched.entry(r.nation).or_default() += dispatched; }
                    for &ei in &path.edges { *w.logistics.usage_tonnes.entry(g.edges[ei].key.clone()).or_default() += dispatched*TONNES_PER_SERVICE; }
                    state.cargo.push(ServiceCargo { key:key.clone(),nation:r.nation,conflict:r.conflict,district:r.district.clone(),
                        service:dispatched, route:path.nodes, segments:path.edges.iter().map(|&ei|g.edges[ei].key.clone()).collect(),
                        at_sea:path.sea, dispatched_day:today, due_day:today.saturating_add(path.days as i32),hold_reason:None });
                }
                delivery.reason = if !settlement_open { "Commercial freight settlement has not opened; local reserves only.".into() }
                    else if dispatched+EPS < want { "Shared freight capacity or national support service is strained.".into() }
                    else if coverage < 1.0-EPS { "The local service reserve is short; dispatched support has not arrived.".into() }
                    else { "Local reserves cover today's demand; support uses shared freight capacity.".into() };
            }
            Err(reason) => delivery.reason = format!("{reason} Local reserves cover {:.0}% of today's demand.",coverage*100.0),
        }
        if let Some(c) = state.cargo.iter().filter(|c|c.key==key && c.hold_reason.is_none()).min_by_key(|c|c.due_day) {
            delivery.days = (c.due_day-today).max(0) as u32;
            if delivery.route.is_empty() { delivery.route=c.route.clone(); }
        }
        deliveries.insert(key,delivery);
    }
    state.last_day=Some(today);
    state.deliveries=deliveries.clone();
    w.campaign_supply=state;
    deliveries
}

#[derive(Clone,Debug,Serialize)]
pub struct SupplyView {
    pub coverage:f64,
    pub status:String,
    pub reason:String,
    pub eta_days:Option<u32>,
}
/// Pure, own-force aggregate. Weight the realized daily service coverage by
/// demand, never expose another nation's route or stock through this view.
pub fn view(w:&WorldState,conflict:u32,viewer:NationId)->SupplyView {
    let mut demand=0.0; let mut served=0.0;
    let mut worst:Option<&SupplyDelivery>=None;
    for (key,b) in &w.campaign_supply.buffers {
        if key.starts_with("garrison:") || b.nation!=viewer || b.conflict!=conflict {continue;}
        if let Some(d)=w.campaign_supply.deliveries.get(key) {
            demand+=b.demand; served+=b.demand*d.coverage;
            if worst.is_none_or(|old|d.coverage<old.coverage){worst=Some(d);}
        }
    }
    let coverage=if demand>EPS {fraction(served/demand)} else {0.0};
    let today=clock::absolute_day(w);
    let eta_days=w.campaign_supply.cargo.iter().filter(|c|!c.key.starts_with("garrison:") && c.nation==viewer && c.conflict==conflict
        && c.hold_reason.is_none() && c.due_day>today && deployment_path_open(w,viewer,&c.route))
        .map(|c|(c.due_day-clock::absolute_day(w)).max(0) as u32).min();
    SupplyView {coverage,status:if demand<=EPS {"Preparing"} else if coverage>=0.9 {"Well supplied"}
        else if coverage>=0.5 {"Strained"} else if coverage>EPS {"Critical"} else {"Cut off"}.into(),
        reason:worst.map_or_else(||"Support deployment has not begun.".into(),|d|d.reason.clone()),eta_days}
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{init::world_1990, theatre::{Access,TheatreId}, world::{Conflict,GameRules}};

    fn fixture() -> (WorldState,SupplyRequest) {
        let mut w = world_1990(GameRules { daily_simulation:true, military_operations:true,
            resource_market:true, logistics_routes:true, physical_logistics:true, ..GameRules::default() });
        w.nation_mut(NationId::Germany).mil_strength=20.0;
        w.campaign_supply.sources.insert(NationId::Germany,Source{district:"DE-BE".into(),service:0.0});
        let r=SupplyRequest {key:"1:germany:DE-BB".into(),nation:NationId::Germany,conflict:1,
            district:"DE-BB".into(),deployed:1.0,burn_monthly:0.0,sea_escort:1.0,sea_denial:0.0};
        (w,r)
    }
    fn open(w: &mut WorldState) { logistics::begin_month(w); }
    fn next(w: &mut WorldState) { clock::advance_date(w); open(w); }
    fn total_service(w: &WorldState) -> f64 {
        w.campaign_supply.sources.values().map(|s|s.service).sum::<f64>()
            + w.campaign_supply.buffers.values().map(|b|b.service).sum::<f64>()
            + w.campaign_supply.cargo.iter().map(|c|c.service).sum::<f64>()
    }
    fn saturate(w: &mut WorldState) {
        for edge in Graph::new(w).edges { w.logistics.usage_tonnes.insert(edge.key,edge.capacity_tonnes); }
    }
    #[test]
    fn dispatch_arrival_and_consumption_are_conserved_and_same_day_idempotent() {
        let (mut w,r)=fixture(); open(&mut w);
        let munitions=w.nation(r.nation).munitions;
        let before=serde_json::to_string(&w.nation(r.nation).arsenal).unwrap();
        let first=prepare(&mut w,std::slice::from_ref(&r));
        assert_eq!(first[&r.key].coverage,0.0,"dispatch is not arrival");
        assert!(!w.campaign_supply.cargo.is_empty());
        assert!(!w.logistics.usage_tonnes.is_empty(),"domestic service reserves actual graph edges");
        assert!((total_service(&w)-22.0).abs()<1e-9,"only today's finite source was created");
        let saved=w.campaign_supply.clone(); let used=w.logistics.usage_tonnes.clone();
        assert_eq!(prepare(&mut w,std::slice::from_ref(&r)),first);
        assert_eq!(w.campaign_supply,saved); assert_eq!(w.logistics.usage_tonnes,used);
        let due=w.campaign_supply.cargo[0].due_day;
        while clock::absolute_day(&w)<due {
            next(&mut w); let got=prepare(&mut w,std::slice::from_ref(&r));
            if clock::absolute_day(&w)<due { assert_eq!(got[&r.key].coverage,0.0); }
            else { assert_eq!(got[&r.key].coverage,1.0); }
        }
        assert_eq!(w.nation(r.nation).munitions,munitions);
        assert_eq!(serde_json::to_string(&w.nation(r.nation).arsenal).unwrap(),before,"support services cannot debit equipment or ammunition");
    }
    #[test]
    fn commercial_usage_cannot_be_reset_or_overbooked_and_reserves_run_out() {
        let (mut w,r)=fixture(); open(&mut w);
        for _ in 0..20 { prepare(&mut w,std::slice::from_ref(&r)); next(&mut w); }
        assert!(w.campaign_supply.buffers[&r.key].service>0.0);
        let mut first=None; let mut last=1.0;
        for _ in 0..20 {
            saturate(&mut w); let ledger=w.logistics.usage_tonnes.clone();
            let got=prepare(&mut w,std::slice::from_ref(&r));
            first.get_or_insert(got[&r.key].coverage); last=got[&r.key].coverage;
            assert_eq!(w.logistics.usage_tonnes,ledger,"military cannot clear or exceed commerce's reservation");
            next(&mut w);
        }
        assert_eq!(first,Some(1.0),"a route interruption is absorbed by real existing reserves");
        assert_eq!(last,0.0,"neither next-day source generation nor sector recreation can refill an isolated buffer");
        assert_eq!(w.campaign_supply.buffers[&r.key].service,0.0);
    }
    #[test]
    fn military_transit_requires_explicit_host_access_and_contested_is_never_friendly() {
        let (mut w,r)=fixture();
        w.districts.insert(r.district.clone(),NationId::France);
        let g=Graph::new(&w);
        assert!(route(&w,&g,&r,"DE-BE").is_err(),"commercial openness/friendship grants no military transit");
        w.access.push(Access{theatre:TheatreId::CentralEurope,host:NationId::France,seeker:r.nation,since_year:1990,since_month:1});
        assert!(route(&w,&g,&r,"DE-BE").is_ok());
        w.conflicts.push(Conflict {id:99,theatre:TheatreId::CentralEurope,side_a:vec![NationId::France],side_b:vec![NationId::Iraq],
            posture:vec![],control:0.0,months:0,quiet_months:0,frozen_since:None,start_year:1990,start_month:1,
            origin_attacker:NationId::Iraq,invasion_declared:false,front:BTreeMap::from([(r.district.clone(),0.0)]),pockets:vec![],aim:None});
        assert!(route(&w,&g,&r,"DE-BE").is_err(),"even a consenting host's contested province is blocked");
    }
    #[test]
    fn saved_booked_cargo_holds_on_permission_revocation_then_arrives_once() {
        let (mut w,r)=fixture(); open(&mut w); prepare(&mut w,std::slice::from_ref(&r));
        let service=w.campaign_supply.cargo[0].service;
        let booked=w.campaign_supply.cargo[0].clone();
        // The original hub is now held by a neutral government; its booked
        // corridor closes. No route is silently rebooked from the new hub.
        w.districts.insert("DE-BE".into(),NationId::France);
        for _ in 0..5 { next(&mut w); saturate(&mut w); prepare(&mut w,std::slice::from_ref(&r)); }
        assert_eq!(w.campaign_supply.cargo[0].service,service);
        assert!(w.campaign_supply.cargo[0].hold_reason.is_some());
        assert_eq!(w.campaign_supply.cargo[0].segments,booked.segments);
        let text=serde_json::to_string(&w).unwrap();
        let mut replay:WorldState=serde_json::from_str(&text).unwrap(); replay.reindex();
        w.access.push(Access{theatre:TheatreId::CentralEurope,host:NationId::France,seeker:r.nation,since_year:1990,since_month:1});
        replay.access=w.access.clone();
        for _ in 0..8 {
            next(&mut w); next(&mut replay); saturate(&mut w); saturate(&mut replay);
            assert_eq!(prepare(&mut w,std::slice::from_ref(&r)),prepare(&mut replay,std::slice::from_ref(&r)));
            assert_eq!(w.campaign_supply,replay.campaign_supply);
        }
        assert!(w.campaign_supply.cargo.iter().all(|c|c.dispatched_day!=booked.dispatched_day),"old consignment arrived once");
    }
    #[test]
    fn source_and_edge_capacity_are_shared_across_requests_and_sea_needs_real_ships() {
        let (mut w,mut r)=fixture(); open(&mut w);
        r.deployed=1000.0;
        let mut second=r.clone(); second.key="2:germany:DE-BB".into();
        prepare(&mut w,&[second.clone(),r.clone()]);
        assert!(total_service(&w)<=22.0+EPS,"requests cannot duplicate the national source");
        assert!(w.campaign_supply.cargo.iter().map(|c|c.service).sum::<f64>()<=22.0+EPS);
        for edge in Graph::new(&w).edges {
            assert!(w.logistics.usage_tonnes.get(&edge.key).copied().unwrap_or(0.0)<=edge.capacity_tonnes+EPS);
        }
        w.nation_mut(r.nation).arsenal.held.retain(|h|arsenal::DECK[h.kit as usize].class!=Class::Naval);
        r.sea_escort=1.0;
        assert_eq!(sea_factor(&w,&r),0.0,"a mission button cannot invent naval lift");
    }
    #[test]
    fn request_order_and_sector_recreation_do_not_create_local_stock() {
        let (mut a,r)=fixture(); let mut b=a.clone(); open(&mut a); open(&mut b);
        let mut s=r.clone(); s.key="2:germany:DE-BB".into();
        assert_eq!(prepare(&mut a,&[r.clone(),s.clone()]),prepare(&mut b,&[s,r.clone()]));
        assert_eq!(a.campaign_supply,b.campaign_supply);
        next(&mut a); prepare(&mut a,&[]);
        next(&mut a); let got=prepare(&mut a,std::slice::from_ref(&r));
        assert_eq!(got[&r.key].coverage,0.0);
        assert_eq!(a.campaign_supply.buffers[&r.key].service,0.0);
    }
    #[test]
    fn garrison_coverage_recovers_real_occupied_corridor_capacity() {
        let (mut w,r)=fixture();
        w.districts.insert(r.district.clone(),NationId::France);
        w.conflicts.push(Conflict {id:1,theatre:TheatreId::CentralEurope,side_a:vec![NationId::Germany],side_b:vec![NationId::France],
            posture:vec![],control:0.0,months:0,quiet_months:0,frozen_since:None,start_year:1990,start_month:1,
            origin_attacker:NationId::Germany,invasion_declared:true,front:BTreeMap::from([(r.district.clone(),1.0)]),pockets:vec![],aim:None});
        let baseline=route(&w,&Graph::new(&w),&r,"DE-BE").unwrap().capacity;
        w.campaign_peace.occupation.insert(r.district.clone(),crate::campaign_peace::Occupation{
            district:r.district.clone(),conflict:1,occupier:r.nation,owner:NationId::France,days:90,resistance:1.0,coverage:0.0});
        let constrained=route(&w,&Graph::new(&w),&r,"DE-BE").unwrap().capacity;
        assert!((constrained-baseline*0.5).abs()<EPS,"uncovered resistance changes freight service, not just telemetry");
        w.campaign_peace.occupation.get_mut(&r.district).unwrap().coverage=1.0;
        assert_eq!(route(&w,&Graph::new(&w),&r,"DE-BE").unwrap().capacity,baseline);
    }
    #[test]
    fn blocked_or_already_due_cargo_does_not_promise_an_arrival_in_view() {
        let (mut w,r)=fixture();open(&mut w);prepare(&mut w,std::slice::from_ref(&r));
        assert!(view(&w,r.conflict,r.nation).eta_days.is_some());
        w.campaign_supply.cargo[0].hold_reason=Some("A booked crossing closed.".into());
        assert_eq!(view(&w,r.conflict,r.nation).eta_days,None);
        w.campaign_supply.cargo[0].hold_reason=None;
        w.districts.insert("DE-BE".into(),NationId::France);
        assert_eq!(view(&w,r.conflict,r.nation).eta_days,None,"a live closure is visible before next tick marks cargo held");
        w.districts.insert("DE-BE".into(),r.nation);
        w.campaign_supply.cargo[0].due_day=clock::absolute_day(&w);
        assert_eq!(view(&w,r.conflict,r.nation).eta_days,None,"no fictitious zero-day future arrival");
    }
    #[test]
    fn deployment_quote_is_pure_and_cannot_use_unconsented_foreign_ground() {
        let (mut w,r)=fixture();let saved=crate::save(&w);
        let path=deployment_route(&w,r.nation,&r.district).unwrap();
        assert_eq!(path.0.first().map(String::as_str),Some("DE-BE"));
        assert!(path.1>0);assert_eq!(crate::save(&w),saved);
        w.districts.insert(r.district.clone(),NationId::France);
        assert!(deployment_route(&w,r.nation,&r.district).is_none());
        assert!(!deployment_path_open(&w,r.nation,&path.0));
    }
    #[test]
    fn garrison_service_is_separate_from_frontline_forecast_coverage() {
        let (mut w,r)=fixture();open(&mut w);prepare(&mut w,std::slice::from_ref(&r));
        for _ in 0..6 {next(&mut w);prepare(&mut w,std::slice::from_ref(&r));}
        let front=view(&w,r.conflict,r.nation);assert_eq!(front.coverage,1.0);
        let key="garrison:1:germany:DE-BB".to_owned();
        let mut buffer=w.campaign_supply.buffers[&r.key].clone();buffer.demand=1000.0;buffer.service=0.0;
        w.campaign_supply.buffers.insert(key.clone(),buffer);
        w.campaign_supply.deliveries.insert(key.clone(),SupplyDelivery {key,coverage:0.0,days:0,route:vec![],reason:"Isolated garrison".into()});
        let displayed=view(&w,r.conflict,r.nation);
        assert_eq!(displayed.coverage,front.coverage);assert_eq!(displayed.reason,front.reason);
    }
    #[test]
    fn tactical_strike_aircraft_do_not_supply_unmodeled_convoy_air_cover() {
        let mut w=crate::equipment::ammunition_operation_tests::fixture("air_light_attack");
        let nation=NationId::USA;
        let kit=arsenal::DECK.iter().position(|d|d.class==Class::Naval).unwrap() as u16;
        w.nation_mut(nation).arsenal.held.push(arsenal::Holding {
            kit,design_id:None,units:1000.0,age:0.0,refit_reserved:0,loss_remainder:0.0,
        });
        let r=SupplyRequest {key:"convoy".into(),nation,conflict:1,district:"US-NY".into(),
            deployed:1.0,burn_monthly:0.0,sea_escort:1.0,sea_denial:1.0};
        let n=w.nation(nation);
        let total=n.arsenal.held.iter().map(|h|arsenal::combat_value(n,h)).sum::<f64>();
        let navy=n.arsenal.held.iter().filter(|h|arsenal::DECK[h.kit as usize].class==Class::Naval)
            .map(|h|arsenal::combat_value(n,h)).sum::<f64>();
        assert!(total>navy && navy>0.0);
        let expected=(navy/(total*0.15)).clamp(0.0,1.0)*0.65*0.6;
        assert!((sea_factor(&w,&r)-expected).abs()<EPS,
            "custom tactical bombers cannot create interception or transport roles");
    }
}
