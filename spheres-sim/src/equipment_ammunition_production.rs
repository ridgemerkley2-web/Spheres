// Conventional ammunition for custom ground equipment and tactical aviation. Prices, recipes,
// factory rates and firing rates are GAME assumptions, never starting stocks.
pub const MAX_AMMO_ORDER: u32 = 1_000_000;
/// Concurrent unfinished batches, including paused work. Closed receipts stay
/// in the ledger indefinitely to prove stock and consumption conservation.
pub const MAX_AMMO_ORDERS: usize = 128;

#[derive(Clone, Debug, Serialize)]
pub struct AmmoDef {
    pub id: &'static str,
    pub name: &'static str,
    pub unit: &'static str,
    pub fabrication_bn: f64,
    pub rounds_per_day: f64,
    pub recipe: [f64; 12],
    pub rounds_per_vehicle_month: f64,
}
const fn ammunition_recipe(iron: f64, copper: f64, coal: f64) -> [f64; 12] {
    let mut r = [0.0; 12];
    r[crate::resources::Commodity::Iron as usize] = iron;
    r[crate::resources::Commodity::Copper as usize] = copper;
    r[crate::resources::Commodity::Coal as usize] = coal;
    r
}
macro_rules! ammo {
    ($id:literal,$name:literal,$cost:expr,$rate:expr,$iron:expr,$copper:expr,$coal:expr,$burn:expr) => {
        AmmoDef {
            id: $id,
            name: $name,
            unit: "rounds",
            fabrication_bn: $cost / 1_000_000_000.0,
            rounds_per_day: $rate,
            recipe: ammunition_recipe($iron, $copper, $coal),
            rounds_per_vehicle_month: $burn,
        }
    };
}
pub const AMMUNITION_CATALOG: &[AmmoDef] = &[
    ammo!(
        "tank_90_mixed",
        "90 mm mixed-purpose",
        450.0,
        100.0,
        0.008,
        0.0015,
        0.000002,
        150.0
    ),
    ammo!(
        "tank_90_penetrator",
        "90 mm penetrator",
        750.0,
        80.0,
        0.010,
        0.002,
        0.000003,
        150.0
    ),
    ammo!(
        "tank_90_support",
        "90 mm support",
        350.0,
        120.0,
        0.008,
        0.001,
        0.000002,
        150.0
    ),
    ammo!(
        "tank_105_mixed",
        "105 mm mixed-purpose",
        600.0,
        90.0,
        0.012,
        0.002,
        0.000003,
        150.0
    ),
    ammo!(
        "tank_105_penetrator",
        "105 mm penetrator",
        1000.0,
        70.0,
        0.014,
        0.0025,
        0.000004,
        150.0
    ),
    ammo!(
        "tank_105_support",
        "105 mm support",
        450.0,
        110.0,
        0.012,
        0.0015,
        0.000003,
        150.0
    ),
    ammo!(
        "tank_120_mixed",
        "120 mm mixed-purpose",
        900.0,
        75.0,
        0.016,
        0.003,
        0.000004,
        150.0
    ),
    ammo!(
        "tank_120_penetrator",
        "120 mm penetrator",
        1500.0,
        55.0,
        0.018,
        0.0035,
        0.000005,
        150.0
    ),
    ammo!(
        "tank_120_support",
        "120 mm support",
        650.0,
        95.0,
        0.016,
        0.002,
        0.000004,
        150.0
    ),
    ammo!(
        "tank_125_mixed",
        "125 mm mixed-purpose",
        1000.0,
        70.0,
        0.018,
        0.003,
        0.000004,
        150.0
    ),
    ammo!(
        "tank_125_penetrator",
        "125 mm penetrator",
        1650.0,
        50.0,
        0.020,
        0.0035,
        0.000005,
        150.0
    ),
    ammo!(
        "tank_125_support",
        "125 mm support",
        750.0,
        90.0,
        0.018,
        0.002,
        0.000004,
        150.0
    ),
    ammo!(
        "autocannon_25",
        "25 mm autocannon",
        18.0,
        6000.0,
        0.00012,
        0.00006,
        0.0000001,
        1800.0
    ),
    ammo!(
        "autocannon_35",
        "35 mm autocannon",
        30.0,
        4500.0,
        0.0003,
        0.0001,
        0.0000002,
        1800.0
    ),
    ammo!(
        "mg_127",
        "12.7 mm machine gun",
        3.0,
        25000.0,
        0.000025,
        0.000025,
        0.00000002,
        4000.0
    ),
    ammo!(
        "howitzer_122_he",
        "122 mm high explosive",
        650.0,
        90.0,
        0.018,
        0.002,
        0.000004,
        600.0
    ),
    ammo!(
        "howitzer_122_guided",
        "122 mm guided",
        9000.0,
        20.0,
        0.018,
        0.005,
        0.000005,
        600.0
    ),
    ammo!(
        "howitzer_155_he",
        "155 mm high explosive",
        1100.0,
        60.0,
        0.032,
        0.003,
        0.000006,
        600.0
    ),
    ammo!(
        "howitzer_155_guided",
        "155 mm guided",
        12000.0,
        15.0,
        0.032,
        0.007,
        0.000007,
        600.0
    ),
    ammo!(
        "aa_cannon",
        "Air-defense cannon",
        40.0,
        4000.0,
        0.00035,
        0.0001,
        0.0000003,
        2000.0
    ),
    ammo!(
        "aa_missile",
        "Short-range air-defense missile",
        45000.0,
        3.0,
        0.020,
        0.015,
        0.000005,
        24.0
    ),
    AmmoDef {
        id: "air_bomb_unguided",
        name: "Unguided aircraft stores",
        unit: "stores",
        fabrication_bn: 0.000004,
        rounds_per_day: 60.0,
        recipe: ammunition_recipe(0.18, 0.001, 0.00004),
        // Aviation uses the immutable aircraft's sortie rate and payload.
        rounds_per_vehicle_month: 0.0,
    },
    AmmoDef {
        id: "air_bomb_guided",
        name: "Guided aircraft stores",
        unit: "stores",
        fabrication_bn: 0.000025,
        rounds_per_day: 12.0,
        recipe: ammunition_recipe(0.18, 0.006, 0.00004),
        rounds_per_vehicle_month: 0.0,
    },
];
pub fn ammo_catalog() -> &'static [AmmoDef] {
    AMMUNITION_CATALOG
}
pub fn ammo_def(id: &str) -> Option<&'static AmmoDef> {
    AMMUNITION_CATALOG.iter().find(|d| d.id == id)
}
pub fn ammunition_def(id: &str) -> Option<&'static AmmoDef> {
    ammo_def(id)
}
pub fn ammunition_family(spec: &DesignSpec) -> Option<&'static str> {
    if matches!(
        spec.platform.as_str(),
        "air_light_attack" | "air_tactical_strike"
    ) {
        return match spec.components.get("air_payload").map(String::as_str) {
            Some("air_payload_unguided") => Some("air_bomb_unguided"),
            Some("air_payload_guided") => Some("air_bomb_guided"),
            _ => None,
        };
    }
    let gun = spec.components.get("armament")?.as_str();
    let load = spec.components.get("ammunition").map(String::as_str);
    let id = match (gun, load) {
        ("armament_standard", _) => "tank_105_mixed",
        ("armament_heavy", _) => "tank_120_mixed",
        ("gun_90", Some("ammo_mixed")) => "tank_90_mixed",
        ("gun_90", Some("ammo_penetrator")) => "tank_90_penetrator",
        ("gun_90", Some("ammo_support")) => "tank_90_support",
        ("gun_105", Some("ammo_mixed")) => "tank_105_mixed",
        ("gun_105", Some("ammo_penetrator")) => "tank_105_penetrator",
        ("gun_105", Some("ammo_support")) => "tank_105_support",
        ("gun_120", Some("ammo_mixed")) => "tank_120_mixed",
        ("gun_120", Some("ammo_penetrator")) => "tank_120_penetrator",
        ("gun_120", Some("ammo_support")) => "tank_120_support",
        ("gun_125", Some("ammo_mixed")) => "tank_125_mixed",
        ("gun_125", Some("ammo_penetrator")) => "tank_125_penetrator",
        ("gun_125", Some("ammo_support")) => "tank_125_support",
        ("ground_gun_25", Some("ground_ammo_autocannon")) => "autocannon_25",
        ("ground_gun_35", Some("ground_ammo_autocannon")) => "autocannon_35",
        ("ground_mg_127", Some("ground_ammo_ball")) => "mg_127",
        ("ground_howitzer_122", Some("ground_ammo_he")) => "howitzer_122_he",
        ("ground_howitzer_122", Some("ground_ammo_guided")) => "howitzer_122_guided",
        ("ground_howitzer_155", Some("ground_ammo_he")) => "howitzer_155_he",
        ("ground_howitzer_155", Some("ground_ammo_guided")) => "howitzer_155_guided",
        ("ground_aa_gun", Some("ground_ammo_aa")) => "aa_cannon",
        ("ground_aa_missiles", Some("ground_ammo_missiles")) => "aa_missile",
        _ => return None,
    };
    Some(id)
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct AmmoConsumptionReceipt {
    pub day: i32,
    pub required: BTreeMap<String, f64>,
    pub used: BTreeMap<String, f64>,
}
#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct AmmunitionState {
    pub stocks: BTreeMap<String, f64>,
    pub consumed: BTreeMap<String, f64>,
    pub orders: Vec<AmmoOrder>,
    pub next_id: u32,
    pub active_from_day: Option<i32>,
    pub last_work_day: Option<i32>,
    pub last_consumption: Option<AmmoConsumptionReceipt>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub supplier_receipts: Vec<AmmoSupplierReceipt>,
}
/// A company delivery is a distinct paid source, never a fabricated public
/// production order. The world validator reconciles this receipt to its sale.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct AmmoSupplierReceipt {
    pub delivery: u32,
    pub company: u32,
    pub product: u32,
    pub family: String,
    pub quantity: u32,
    pub delivered_day: i32,
}
pub(crate) fn receive_supplier_ammunition(
    n: &mut Nation,
    r: AmmoSupplierReceipt,
) -> Result<(), String> {
    validate_ammunition(n)?;
    if n.equipment
        .as_ref()
        .and_then(|s| s.maintenance_plan.as_ref())
        .is_none()
        || ammo_def(&r.family).is_none()
        || r.quantity == 0
        || r.quantity > MAX_AMMO_ORDER
        || n.equipment
            .as_ref()
            .and_then(|s| s.ammunition.as_ref())
            .is_some_and(|a| a.supplier_receipts.iter().any(|x| x.delivery == r.delivery))
    {
        return Err("Ammunition arrival requires an actual maintenance plan and one unique supported supplier receipt.".into());
    }
    let a = state_mut(n).ammunition.get_or_insert_with(Default::default);
    *a.stocks.entry(r.family.clone()).or_default() += r.quantity as f64;
    a.supplier_receipts.push(r);
    Ok(())
}
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct AmmoOrder {
    pub id: u32,
    pub family: String,
    pub district: String,
    pub quantity: u32,
    pub completed_rounds: u32,
    pub work_rounds: f64,
    pub cost_bn: f64,
    pub spent_bn: f64,
    pub daily_limit_bn: f64,
    pub recipe_per_round: [f64; 12],
    pub resources_used: [f64; 12],
    pub rounds_per_day: f64,
    pub started_day: i32,
    pub completed_day: Option<i32>,
    pub last_day: Option<i32>,
    pub last_spent_bn: f64,
    pub paused: bool,
    pub status: ProjectStatus,
    pub reason: String,
}
#[derive(Clone, Debug, Serialize)]
pub struct AmmoOrderQuote {
    pub valid: bool,
    pub reason: Option<String>,
    pub family: String,
    pub quantity: u32,
    pub cost_bn: f64,
    pub minimum_days: u32,
    pub daily_limit_bn: f64,
    pub recipe: [f64; 12],
}
pub fn ammunition_active(n: &Nation, day: i32) -> bool {
    n.equipment
        .as_ref()
        .and_then(|s| s.ammunition.as_ref())
        .and_then(|a| a.active_from_day)
        .is_some_and(|d| day >= d)
}
fn ammunition_ended(p: &AmmoOrder) -> bool {
    matches!(p.status, ProjectStatus::Complete | ProjectStatus::Cancelled)
}
pub fn reserved_ammunition_slots(n: &Nation, district: &str) -> usize {
    n.equipment
        .as_ref()
        .and_then(|s| s.ammunition.as_ref())
        .map_or(0, |a| {
            a.orders
                .iter()
                .filter(|p| p.district == district && !ammunition_ended(p))
                .count()
        })
}
pub(crate) fn ammo_actor_refusal(w: &WorldState, id: NationId) -> Option<String> {
    actor_refusal(w,id).or_else(||(!(w.rules.production_system&&w.rules.resource_market)).then(||"Ammunition fabrication requires physical production and the raw-resource market.".into()))
        .or_else(||w.nation(id).equipment.as_ref().and_then(|s|s.maintenance_plan.as_ref()).is_none().then(||"Set an actual fleet maintenance plan before ordering ammunition. Both use Defense maintenance and supply authority.".into()))
}
pub fn ammo_order_quote(
    w: &WorldState,
    id: NationId,
    family: &str,
    district: &str,
    quantity: u32,
    limit: f64,
) -> AmmoOrderQuote {
    let def = ammo_def(family);
    let mut reason = ammo_actor_refusal(w, id).or_else(|| budget_refusal(limit));
    if reason.is_none() {
        reason = validate_state(w.nation(id)).err();
    }
    if reason.is_none() {
        reason = if def.is_none() {
            Some("Choose a supported ammunition family.".into())
        } else if !(1..=MAX_AMMO_ORDER).contains(&quantity) {
            Some(format!("Choose one to {MAX_AMMO_ORDER} rounds."))
        } else {
            None
        };
    }
    if reason.is_none() {
        reason = site_refusal(w, id, district);
    }
    if reason.is_none() {
        let s = w.nation(id).equipment.as_ref().unwrap();
        if !s
            .revisions
            .values()
            .any(|r| r.certified_day.is_some() && ammunition_family(&r.spec) == Some(family))
        {
            reason=Some("Certify a compatible weapon configuration before producing this ammunition family.".into());
        } else if s.ammunition.as_ref().is_some_and(|a| {
            a.orders.iter().filter(|p| !ammunition_ended(p)).count() >= MAX_AMMO_ORDERS
        }) {
            reason = Some(format!("At most {MAX_AMMO_ORDERS} unfinished ammunition batches may be active at once, including paused work. Finish or cancel an existing batch."));
        } else if s.ammunition.as_ref().is_some_and(|a| a.next_id == u32::MAX) {
            reason = Some("No further ammunition order identifiers are available.".into());
        }
    }
    AmmoOrderQuote {
        valid: reason.is_none(),
        reason,
        family: family.into(),
        quantity,
        cost_bn: def.map_or(0.0, |d| d.fabrication_bn * quantity as f64),
        minimum_days: def.map_or(0, |d| {
            (quantity as f64 / d.rounds_per_day).ceil().max(1.0) as u32
        }),
        daily_limit_bn: limit,
        recipe: def.map_or([0.0; 12], |d| d.recipe.map(|v| v * quantity as f64)),
    }
}
pub fn start_ammo_order(
    w: &mut WorldState,
    id: NationId,
    family: &str,
    district: &str,
    quantity: u32,
    limit: f64,
) -> Result<u32, String> {
    let q = ammo_order_quote(w, id, family, district, quantity, limit);
    if let Some(r) = q.reason {
        return Err(r);
    }
    let def = ammo_def(family).unwrap();
    let today = clock::absolute_day(w);
    let a = state_mut(w.nation_mut(id))
        .ammunition
        .get_or_insert_with(Default::default);
    let number = a.next_id;
    a.next_id += 1;
    a.orders.push(AmmoOrder {
        id: number,
        family: family.into(),
        district: district.into(),
        quantity,
        completed_rounds: 0,
        work_rounds: 0.0,
        cost_bn: q.cost_bn,
        spent_bn: 0.0,
        daily_limit_bn: limit,
        recipe_per_round: def.recipe,
        resources_used: [0.0; 12],
        rounds_per_day: def.rounds_per_day,
        started_day: today,
        completed_day: None,
        last_day: None,
        last_spent_bn: 0.0,
        paused: false,
        status: ProjectStatus::Working,
        reason: "Awaiting the first funded work date; no rounds exist yet.".into(),
    });
    Ok(number)
}
fn ammo_order_index(w: &WorldState, id: NationId, job: u32) -> Result<usize, String> {
    if let Some(r) = actor_refusal(w, id) {
        return Err(r);
    }
    validate_state(w.nation(id))?;
    w.nation(id)
        .equipment
        .as_ref()
        .and_then(|s| s.ammunition.as_ref())
        .and_then(|a| {
            a.orders
                .iter()
                .position(|p| p.id == job && !ammunition_ended(p))
        })
        .ok_or("This ammunition order is missing or closed.".into())
}
pub fn set_ammo_funding(
    w: &mut WorldState,
    id: NationId,
    job: u32,
    limit: f64,
) -> Result<(), String> {
    if let Some(r) = budget_refusal(limit) {
        return Err(r);
    }
    let i = ammo_order_index(w, id, job)?;
    state_mut(w.nation_mut(id))
        .ammunition
        .as_mut()
        .unwrap()
        .orders[i]
        .daily_limit_bn = limit;
    Ok(())
}
pub fn pause_ammo_order(
    w: &mut WorldState,
    id: NationId,
    job: u32,
    paused: bool,
) -> Result<(), String> {
    let i = ammo_order_index(w, id, job)?;
    let p = &mut state_mut(w.nation_mut(id))
        .ammunition
        .as_mut()
        .unwrap()
        .orders[i];
    p.paused = paused;
    p.status = if paused {
        ProjectStatus::Paused
    } else {
        ProjectStatus::Working
    };
    p.reason = if paused {
        "Paused; completed rounds remain in stock and prior work stays paid."
    } else {
        "Awaiting the next funded work date."
    }
    .into();
    Ok(())
}
pub fn cancel_ammo_order(w: &mut WorldState, id: NationId, job: u32) -> Result<(), String> {
    let i = ammo_order_index(w, id, job)?;
    let today = clock::absolute_day(w);
    let p = &mut state_mut(w.nation_mut(id))
        .ammunition
        .as_mut()
        .unwrap()
        .orders[i];
    p.status = ProjectStatus::Cancelled;
    p.completed_day = Some(today);
    p.reason="Cancelled. Completed rounds remain in stock; prior fabrication and raw inputs are not refunded.".into();
    Ok(())
}
pub fn activate_ammunition(w: &mut WorldState, id: NationId) -> Result<(), String> {
    if let Some(r) = ammo_actor_refusal(w, id) {
        return Err(r);
    }
    validate_state(w.nation(id))?;
    if !w.nation(id).equipment.as_ref().is_some_and(|s| {
        s.revisions
            .values()
            .any(|r| r.certified_day.is_some() && ammunition_family(&r.spec).is_some())
    }) {
        return Err(
            "Certify a compatible custom ground model before activating physical ammunition."
                .into(),
        );
    }
    let from = clock::absolute_day(w).saturating_add(1);
    let a = state_mut(w.nation_mut(id))
        .ammunition
        .get_or_insert_with(Default::default);
    if a.active_from_day.is_some() {
        return Err("Physical ammunition has already been activated.".into());
    }
    a.active_from_day = Some(from);
    Ok(())
}

fn ammo_site_blocker(w: &WorldState, id: NationId, job: &AmmoOrder, day: i32) -> Option<String> {
    if let Some(r) = crate::control::blocker(w, id, &job.district) {
        return Some(r);
    }
    if w.districts.get(&job.district) != Some(&id) {
        return Some(
            "The ammunition factory province is no longer owned by this government.".into(),
        );
    }
    let n = w.nation(id);
    // Existing equipment/directed manufacturing retains its places after a
    // capacity loss. Ammunition uses the remaining places in stable order id.
    // A job completing today has already used today's factory work place.
    let prior = crate::manufacturing::lines_for(w, id)
        .filter(|p| p.district == job.district)
        .count()
        + n.equipment.as_ref().map_or(0, |s| {
            s.projects
                .iter()
                .filter(|p| {
                    p.district.as_deref() == Some(job.district.as_str())
                        && (!matches!(p.status, ProjectStatus::Complete | ProjectStatus::Cancelled)
                            || p.status == ProjectStatus::Complete && p.completed_day == Some(day))
                })
                .count()
        });
    let mut jobs: Vec<_> = n
        .equipment
        .as_ref()?
        .ammunition
        .as_ref()?
        .orders
        .iter()
        .filter(|p| {
            p.district == job.district
                && (!ammunition_ended(p)
                    || p.status == ProjectStatus::Complete && p.completed_day == Some(day))
        })
        .collect();
    jobs.sort_by_key(|p| p.id);
    let place = jobs.iter().position(|p| p.id == job.id)?;
    (place + prior >= crate::manufacturing::plant_slots(w, &job.district) as usize)
        .then(|| "Existing equipment lines occupy the remaining completed arms-plant slots.".into())
}
fn ammo_work_payment(job: &AmmoOrder, advance: f64) -> f64 {
    if advance >= job.quantity as f64 - job.work_rounds {
        (job.cost_bn - job.spent_bn).max(0.0)
    } else {
        advance * (job.cost_bn / job.quantity as f64)
    }
}
fn ammo_work_inputs(job: &AmmoOrder, advance: f64) -> [f64; 12] {
    let next = (job.work_rounds + advance).min(job.quantity as f64);
    let target = ammo_material_target(job, next);
    std::array::from_fn(|i| (target[i] - job.resources_used[i]).max(0.0))
}
fn ammo_material_target(job: &AmmoOrder, rounds: f64) -> [f64; 12] {
    let batch = job.recipe_per_round.map(|v| v * job.quantity as f64);
    crate::resources::scale_bundle(&batch, rounds / job.quantity.max(1) as f64)
}
fn ammo_next_advance(job: &AmmoOrder, available: f64) -> f64 {
    let unit = job.cost_bn / job.quantity as f64;
    (job.quantity as f64 - job.work_rounds)
        .max(0.0)
        .min(job.rounds_per_day)
        .min(job.daily_limit_bn / unit)
        .min(available / unit)
}
fn ammo_set_reason(w: &mut WorldState, id: NationId, index: usize, reason: String) {
    let p = &mut state_mut(w.nation_mut(id))
        .ammunition
        .as_mut()
        .unwrap()
        .orders[index];
    p.status = if p.paused {
        ProjectStatus::Paused
    } else {
        ProjectStatus::Blocked
    };
    p.reason = reason;
}
fn tick_ammunition_work(w: &mut WorldState, id: NationId) {
    let day = clock::absolute_day(w);
    let n = w.nation(id);
    if !maintenance_plan_on(n, day) || validate_ammunition(n).is_err() {
        return;
    }
    let Some(a) = n.equipment.as_ref().and_then(|s| s.ammunition.as_ref()) else {
        return;
    };
    if a.last_work_day.is_some_and(|d| d >= day)
        || n.program_budget
            .as_ref()
            .is_none_or(|p| p.day != Some(day) || p.settled_day == Some(day))
    {
        return;
    }
    let mut jobs: Vec<_> = a
        .orders
        .iter()
        .enumerate()
        .filter(|(_, p)| !ammunition_ended(p))
        .map(|(i, p)| (p.id, i))
        .collect();
    jobs.sort();
    {
        let a = state_mut(w.nation_mut(id)).ammunition.as_mut().unwrap();
        a.last_work_day = Some(day);
        for p in &mut a.orders {
            p.last_day = Some(day);
            p.last_spent_bn = 0.0;
        }
    }
    for (_, i) in jobs {
        let job = w
            .nation(id)
            .equipment
            .as_ref()
            .unwrap()
            .ammunition
            .as_ref()
            .unwrap()
            .orders[i]
            .clone();
        if day <= job.started_day {
            continue;
        }
        let reason = if job.paused {
            Some("Paused by the government.".into())
        } else if job.daily_limit_bn <= 0.0 {
            Some("Daily ammunition funding is zero.".into())
        } else {
            ammo_site_blocker(w, id, &job, day)
        };
        if let Some(r) = reason {
            ammo_set_reason(w, id, i, r);
            continue;
        }
        let available = crate::programs::available_bn(w, id, crate::world::BUDGET_DEFENSE, 2);
        let desired = ammo_next_advance(&job, available);
        let mut advance =
            desired * crate::resources::bundle_throughput(w, id, &ammo_work_inputs(&job, desired));
        if advance <= 0.0 || job.work_rounds + advance == job.work_rounds {
            ammo_set_reason(w,id,i,if available<=0.0{"No maintenance and supply authority remains after fleet servicing."}else{"Waiting for the physical raw inputs or enough funding for the next work increment."}.into());
            continue;
        }
        if advance > job.quantity as f64 - job.work_rounds {
            advance = job.quantity as f64 - job.work_rounds;
        }
        let payment = ammo_work_payment(&job, advance);
        let raw = ammo_work_inputs(&job, advance);
        if payment > available || payment > job.daily_limit_bn + 1e-12 {
            ammo_set_reason(
                w,
                id,
                i,
                "The exact ammunition payment exceeds remaining daily authority.".into(),
            );
            continue;
        }
        let opening = w.nation(id).program_budget.clone();
        if let Err(r) =
            crate::programs::spend_operating(w, id, crate::world::BUDGET_DEFENSE, 2, payment)
        {
            ammo_set_reason(w, id, i, r);
            continue;
        }
        if let Err((c, need, have)) = crate::resources::consume_stockpile_atomic(w, id, &raw) {
            w.nation_mut(id).program_budget = opening;
            ammo_set_reason(
                w,
                id,
                i,
                format!(
                    "{} needs {:.6} {}, but only {:.6} is available.",
                    c.name(),
                    need,
                    c.unit(),
                    have
                ),
            );
            continue;
        }
        let next = (job.work_rounds + advance).min(job.quantity as f64);
        let completed = (next.floor() as u32).min(job.quantity);
        let a = state_mut(w.nation_mut(id)).ammunition.as_mut().unwrap();
        let p = &mut a.orders[i];
        p.work_rounds = next;
        p.spent_bn += payment;
        p.completed_rounds = completed;
        p.last_spent_bn = payment;
        for (used, input) in p.resources_used.iter_mut().zip(raw) {
            *used += input;
        }
        let finished = completed == p.quantity;
        p.status = if finished {
            ProjectStatus::Complete
        } else {
            ProjectStatus::Working
        };
        p.reason = if finished {
            "Complete. Manufactured rounds are now in the national ammunition store."
        } else {
            "Paid fabrication is progressing; only completed whole rounds enter stock."
        }
        .into();
        if finished {
            p.completed_day = Some(day);
        }
        if completed > job.completed_rounds {
            *a.stocks.entry(job.family).or_default() += (completed - job.completed_rounds) as f64;
        }
    }
}

pub fn validate_ammunition(n: &Nation) -> Result<(), String> {
    let Some(s) = &n.equipment else {
        return Ok(());
    };
    let Some(a) = &s.ammunition else {
        return Ok(());
    };
    let fail =
        || "Invalid custom ammunition inventory, production or consumption ledger.".to_string();
    if s.version < 5
        || s.maintenance_plan.is_none()
        || a.orders.iter().filter(|p| !ammunition_ended(p)).count() > MAX_AMMO_ORDERS
    {
        return Err(fail());
    }
    let plan = s.maintenance_plan.as_ref().unwrap();
    let budget_day = n.program_budget.as_ref().and_then(|p| p.day);
    let command_day = budget_day.map_or(plan.from_day.saturating_sub(1), |d| d.saturating_add(1));
    if a.active_from_day
        .is_some_and(|d| d < plan.from_day || d > command_day.saturating_add(1))
    {
        return Err(fail());
    }
    let mut ids = BTreeSet::new();
    let mut completed: BTreeMap<&str, f64> = BTreeMap::new();
    let close = |x: f64, y: f64| (x - y).abs() <= 1e-8 + 1e-12 * x.abs().max(y.abs());
    let money_close = |x: f64, y: f64| (x - y).abs() <= 1e-12 + 1e-12 * x.abs().max(y.abs());
    for p in &a.orders {
        let def = ammo_def(&p.family).ok_or_else(fail)?;
        let material_target = ammo_material_target(p, p.work_rounds);
        if !ids.insert(p.id)
            || p.id >= a.next_id
            || ammo_def(&p.family).is_none()
            || p.district.is_empty()
            || !(1..=MAX_AMMO_ORDER).contains(&p.quantity)
            || p.completed_rounds > p.quantity
            || p.work_rounds < 0.0
            || p.work_rounds > p.quantity as f64
            || p.completed_rounds != (p.work_rounds.floor() as u32)
            || budget_refusal(p.daily_limit_bn).is_some()
            || p.rounds_per_day <= 0.0
            || p.cost_bn <= 0.0
            || [
                p.work_rounds,
                p.cost_bn,
                p.spent_bn,
                p.rounds_per_day,
                p.last_spent_bn,
            ]
            .iter()
            .any(|v| !v.is_finite() || *v < 0.0)
            || p.spent_bn > p.cost_bn + 1e-12
            || p.last_spent_bn > p.spent_bn + 1e-12
            || p.resources_used
                .iter()
                .chain(p.recipe_per_round.iter())
                .any(|v| !v.is_finite() || *v < 0.0)
            || !money_close(p.cost_bn, def.fabrication_bn * p.quantity as f64)
            || p.recipe_per_round != def.recipe
            || p.rounds_per_day != def.rounds_per_day
            || !money_close(p.spent_bn, p.cost_bn * p.work_rounds / p.quantity as f64)
            || p.resources_used
                .iter()
                .zip(material_target)
                .any(|(used, target)| (*used - target).abs() > 1e-12 + target.abs() * 1e-12)
            || (p.status == ProjectStatus::Complete) != (p.completed_rounds == p.quantity)
            || p.started_day > command_day
            || ammunition_ended(p) != p.completed_day.is_some()
            || p.completed_day.is_some_and(|d| {
                d < p.started_day
                    || d > command_day
                    || p.status == ProjectStatus::Complete && budget_day.is_none_or(|last| d > last)
            })
            || p.last_day.is_some_and(|d| {
                d < p.started_day
                    || a.last_work_day.is_none_or(|last| d > last)
                    || budget_day.is_none_or(|last| d > last)
            })
        {
            return Err(fail());
        }
        *completed.entry(&p.family).or_default() += p.completed_rounds as f64;
    }
    let mut supplier_ids = BTreeSet::new();
    for r in &a.supplier_receipts {
        if r.delivery == 0
            || r.company == 0
            || r.product == 0
            || !supplier_ids.insert(r.delivery)
            || ammo_def(&r.family).is_none()
            || !(1..=MAX_AMMO_ORDER).contains(&r.quantity)
            || r.delivered_day < plan.from_day
            || r.delivered_day > command_day
            || !s.revisions.values().any(|revision| {
                revision.certified_day.is_some_and(|d| d <= r.delivered_day)
                    && ammunition_family(&revision.spec) == Some(r.family.as_str())
            })
        {
            return Err(fail());
        }
        *completed.entry(&r.family).or_default() += r.quantity as f64;
    }
    for (family, value) in a.stocks.iter().chain(a.consumed.iter()) {
        if ammo_def(family).is_none() || !value.is_finite() || *value < 0.0 {
            return Err(fail());
        }
    }
    for def in ammo_catalog() {
        let stock = a.stocks.get(def.id).copied().unwrap_or(0.0);
        let used = a.consumed.get(def.id).copied().unwrap_or(0.0);
        if !close(stock + used, completed.get(def.id).copied().unwrap_or(0.0)) {
            return Err(fail());
        }
    }
    if let Some(r) = &a.last_consumption {
        let ground_active = a.active_from_day.is_some_and(|d| r.day >= d);
        let aviation_known = s.revisions.values().any(|revision| {
            revision.profile.aviation.is_some()
                && revision.certified_day.is_some_and(|d| d <= r.day)
        });
        if (!ground_active && !aviation_known)
            || r.day < plan.from_day && r.used.values().any(|v| *v > 0.0)
            || n.program_budget
                .as_ref()
                .and_then(|p| p.day)
                .is_none_or(|d| r.day > d)
        {
            return Err(fail());
        }
        for (family, v) in r.required.iter().chain(r.used.iter()) {
            if ammo_def(family).is_none() || !v.is_finite() || *v < 0.0 {
                return Err(fail());
            }
            if !ground_active
                && *v > 0.0
                && !matches!(family.as_str(), "air_bomb_unguided" | "air_bomb_guided")
            {
                return Err(fail());
            }
        }
        for (family, used) in &r.used {
            if *used > r.required.get(family).copied().unwrap_or(0.0) + 1e-8
                || *used > a.consumed.get(family).copied().unwrap_or(0.0) + 1e-8
            {
                return Err(fail());
            }
        }
    }
    if a.last_work_day.is_some_and(|d| {
        n.program_budget
            .as_ref()
            .and_then(|p| p.day)
            .is_none_or(|day| d > day)
    }) {
        return Err(fail());
    }
    Ok(())
}

/// Funded ammunition demand shares D2 once, after the current fleet's modeled
/// maintenance invoice. This is independent of D3 procurement authority: the
/// forecast must never subtract an ammunition bill from legacy vehicle funding.
pub fn ammunition_supply_demand(w: &WorldState, id: NationId) -> EquipmentRawDemand {
    ammunition_raw_plan(w, id, crate::economic_ai::RAW_HORIZON_DAYS[2])
}
/// Next-date reporting only. It authorizes no automatic material purchase.
pub fn ammunition_next_work(w: &WorldState, id: NationId) -> [f64; 12] {
    ammunition_raw_plan(w, id, 1).next_work
}
fn ammunition_raw_plan(w: &WorldState, id: NationId, horizon: i32) -> EquipmentRawDemand {
    let mut out = EquipmentRawDemand::default();
    let Some(n) = w.nation_opt(id).filter(|n| n.alive) else {
        return out;
    };
    let Some(s) = &n.equipment else {
        return out;
    };
    let Some(a) = &s.ammunition else {
        return out;
    };
    let Some(plan) = &s.maintenance_plan else {
        return out;
    };
    let Some(budget) = &n.program_budget else {
        return out;
    };
    if !clock::is_daily(w) {
        return out;
    }
    let start = crate::resources::forecast_start_day(w)
        .max(a.last_work_day.map_or(i32::MIN, |d| d.saturating_add(1)));
    let mut jobs: Vec<_> = a
        .orders
        .iter()
        .filter(|p| {
            !ammunition_ended(p)
                && !p.paused
                && p.daily_limit_bn > 0.0
                && ammo_site_blocker(w, id, p, start).is_none()
        })
        .cloned()
        .collect();
    jobs.sort_by_key(|p| p.id);
    for job in &jobs {
        let total = ammo_material_target(job, job.quantity as f64);
        for i in 0..12 {
            out.remaining[i] += (total[i] - job.resources_used[i]).max(0.0);
        }
    }
    let d = crate::world::BUDGET_DEFENSE;
    let mut prepaid = budget.prepaid_bn[d][2];
    let mut authority_year = clock::date_from_day(start).0;
    let mut funds = prepaid
        + if budget.authority_year == authority_year {
            budget.available_bn[d][2]
        } else {
            0.0
        };
    let requirement = fleet_maintenance_requirement(n) + legacy_maintenance_requirement(n);
    let mut used = [0.0; 12];
    for offset in 0..horizon {
        let day = start.saturating_add(offset);
        let (year, month, _) = clock::date_from_day(day);
        if year != authority_year {
            funds = prepaid;
            authority_year = year;
        }
        if day >= plan.from_day && year == budget.fiscal_year && budget.day != Some(day) {
            funds += n.budget_for(year).allocations[d] * n.gdp * budget.departments[d][2] as f64
                / 10_000.0
                / 12.0
                / crate::world::days_in_month(year, month) as f64;
        }
        if day >= plan.from_day {
            if plan.receipt.as_ref().is_none_or(|r| r.day < day) {
                let payment = requirement.min(plan.daily_limit_bn).min(funds).max(0.0);
                funds = (funds - payment).max(0.0);
                prepaid = (prepaid - payment).max(0.0);
            }
            for job in &mut jobs {
                if ammunition_ended(job) || day <= job.started_day {
                    continue;
                }
                let advance = ammo_next_advance(job, funds);
                if advance <= 0.0 || job.work_rounds + advance == job.work_rounds {
                    continue;
                }
                let payment = ammo_work_payment(job, advance);
                if payment > funds || payment > job.daily_limit_bn + 1e-12 {
                    continue;
                }
                let raw = ammo_work_inputs(job, advance);
                funds = (funds - payment).max(0.0);
                prepaid = (prepaid - payment).max(0.0);
                for i in 0..12 {
                    used[i] += raw[i];
                    job.resources_used[i] += raw[i];
                }
                job.work_rounds = (job.work_rounds + advance).min(job.quantity as f64);
                job.spent_bn += payment;
                job.completed_rounds = job.work_rounds.floor() as u32;
                if job.completed_rounds == job.quantity {
                    job.status = ProjectStatus::Complete;
                }
            }
        }
        if offset == 0 {
            out.next_work = used;
        }
        for (h, days) in crate::economic_ai::RAW_HORIZON_DAYS.iter().enumerate() {
            if offset + 1 == *days {
                for i in 0..12 {
                    out.horizons[i][h] = used[i].min(out.remaining[i]);
                }
            }
        }
    }
    out
}

#[cfg(test)]
pub(crate) fn seed_test_ammunition(w: &mut WorldState, id: NationId, family: &str, quantity: u32) {
    let def = ammo_def(family).unwrap();
    let today = clock::absolute_day(w);
    let district = w
        .districts
        .iter()
        .find(|(_, owner)| **owner == id)
        .unwrap()
        .0
        .clone();
    let a = state_mut(w.nation_mut(id))
        .ammunition
        .get_or_insert_with(Default::default);
    let number = a.next_id;
    a.next_id += 1;
    a.orders.push(AmmoOrder {
        id: number,
        family: family.into(),
        district,
        quantity,
        completed_rounds: quantity,
        work_rounds: quantity as f64,
        cost_bn: def.fabrication_bn * quantity as f64,
        spent_bn: def.fabrication_bn * quantity as f64,
        daily_limit_bn: 1.0,
        recipe_per_round: def.recipe,
        resources_used: crate::resources::scale_bundle(
            &def.recipe.map(|v| v * quantity as f64),
            1.0,
        ),
        rounds_per_day: def.rounds_per_day,
        started_day: today - 1,
        completed_day: Some(today),
        last_day: None,
        last_spent_bn: 0.0,
        paused: false,
        status: ProjectStatus::Complete,
        reason: "Paid, completed test-fixture batch.".into(),
    });
    *a.stocks.entry(family.into()).or_default() += quantity as f64;
}

#[cfg(test)]
mod ammunition_production_tests {
    use super::*;
    use crate::{
        programs,
        resources::{self, Commodity},
        world::{GameRules, BUDGET_DEFENSE as D},
    };
    const USA: NationId = NationId::USA;
    fn fixture() -> (WorldState, String) {
        let mut w = crate::init::world_1990(GameRules {
            daily_simulation: true,
            military_operations: true,
            production_system: true,
            manufacturing_system: true,
            resource_market: true,
            resource_gates: true,
            ..Default::default()
        });
        w.player = Some(USA);
        programs::set_construction_budget(&mut w, USA, 0.0).unwrap();
        let spec = default_spec("ground_apc");
        assert_eq!(ammunition_family(&spec), Some("mg_127"));
        let profile = design_preview(&w, USA, &spec).profile.unwrap();
        let day = clock::absolute_day(&w);
        state_mut(w.nation_mut(USA)).revisions.insert(
            "ammo-fixture".into(),
            DesignRevision {
                id: "ammo-fixture".into(),
                name: "Ammunition fixture".into(),
                specification_key: specification_key(&spec),
                spec,
                profile,
                created_day: day,
                certified_day: Some(day),
            },
        );
        set_maintenance_plan(&mut w, USA, 0.0).unwrap();
        let district = w
            .districts
            .iter()
            .find(|(_, owner)| **owner == USA)
            .unwrap()
            .0
            .clone();
        for _ in 0..3 {
            crate::production::complete_capability(
                &mut w,
                &district,
                crate::production::ProjectKind::ArmsPlant,
            );
        }
        for c in [Commodity::Iron, Commodity::Copper, Commodity::Coal] {
            resources::set_stockpile_for_test(&mut w, USA, c, 10.0);
        }
        w.nation_mut(USA).treasury_bn = Some(1000.0);
        (w, district)
    }
    fn state(w: &WorldState) -> &AmmunitionState {
        w.nation(USA)
            .equipment
            .as_ref()
            .unwrap()
            .ammunition
            .as_ref()
            .unwrap()
    }
    fn job(w: &WorldState, id: u32) -> &AmmoOrder {
        state(w).orders.iter().find(|p| p.id == id).unwrap()
    }
    fn open_next(w: &mut WorldState) {
        clock::advance_date(w);
        programs::begin_day(w);
    }
    fn next_work(w: &mut WorldState) {
        open_next(w);
        settle_support(w);
        programs::finish_day(w);
    }
    fn near(a: f64, b: f64, tolerance: f64) {
        assert!((a - b).abs() <= tolerance, "{a:.16e} != {b:.16e}");
    }
    #[test]
    fn review_order_and_activation_never_grant_rounds_or_spend_an_open_day() {
        let (mut w, district) = fixture();
        programs::begin_day(&mut w);
        let before = crate::save(&w);
        let q = ammo_order_quote(&w, USA, "mg_127", &district, 123, 0.1);
        assert!(q.valid, "{:?}", q.reason);
        assert_eq!(crate::save(&w), before);
        let fiscal = w.nation(USA).program_budget.clone();
        let cash = w.nation(USA).treasury_bn;
        let id = start_ammo_order(&mut w, USA, "mg_127", &district, 123, 0.1).unwrap();
        activate_ammunition(&mut w, USA).unwrap();
        assert!(!ammunition_active(w.nation(USA), clock::absolute_day(&w)));
        settle_support(&mut w);
        assert_eq!(job(&w, id).spent_bn, 0.0);
        assert_eq!(job(&w, id).completed_rounds, 0);
        assert!(state(&w).stocks.is_empty());
        assert_eq!(w.nation(USA).program_budget, fiscal);
        assert_eq!(w.nation(USA).treasury_bn, cash);
        let saved = crate::save(&w);
        assert!(activate_ammunition(&mut w, USA).is_err());
        assert_eq!(crate::save(&w), saved);
        open_next(&mut w);
        assert!(ammunition_active(w.nation(USA), clock::absolute_day(&w)));
        assert!(validate_state(w.nation(USA)).is_ok());
    }
    #[test]
    fn fabrication_pays_and_consumes_once_then_fiscal_owner_posts_cash_once() {
        let (mut w, district) = fixture();
        let id = start_ammo_order(&mut w, USA, "mg_127", &district, 123, 0.1).unwrap();
        let opening: [f64; 12] =
            std::array::from_fn(|i| resources::stockpile(&w, USA, resources::ALL[i]));
        open_next(&mut w);
        let d3 = w.nation(USA).program_budget.as_ref().unwrap().available_bn[D][3];
        let cash = w.nation(USA).treasury_bn;
        settle_support(&mut w);
        let p = job(&w, id);
        assert_eq!(p.status, ProjectStatus::Complete);
        assert_eq!(state(&w).stocks["mg_127"], 123.0);
        near(p.spent_bn, p.cost_bn, 1e-15);
        near(
            w.nation(USA)
                .program_budget
                .as_ref()
                .unwrap()
                .spent_today_bn[D][2],
            p.cost_bn,
            1e-15,
        );
        for i in [
            Commodity::Iron.idx(),
            Commodity::Copper.idx(),
            Commodity::Coal.idx(),
        ] {
            near(
                opening[i] - resources::stockpile(&w, USA, resources::ALL[i]),
                p.resources_used[i],
                2e-15,
            );
        }
        assert_eq!(w.nation(USA).treasury_bn, cash);
        assert_eq!(
            w.nation(USA).program_budget.as_ref().unwrap().available_bn[D][3],
            d3
        );
        assert!(validate_state(w.nation(USA)).is_ok());
        let paid = crate::save(&w);
        settle_support(&mut w);
        assert_eq!(crate::save(&w), paid);
        programs::stage_fiscal(w.nation_mut(USA), 0.0, 0.0);
        let bill: f64 = w
            .nation(USA)
            .program_budget
            .as_ref()
            .unwrap()
            .spent_today_bn
            .iter()
            .flatten()
            .sum();
        programs::finish_day(&mut w);
        near(
            cash.unwrap() - w.nation(USA).treasury_bn.unwrap(),
            bill,
            1e-12,
        );
        let saved = crate::save(&w);
        programs::finish_day(&mut w);
        settle_support(&mut w);
        assert_eq!(crate::save(&w), saved);
    }
    #[test]
    fn tiny_daily_raw_inputs_accumulate_on_the_warehouse_quantum_and_finish_fully_paid() {
        let (mut w, district) = fixture();
        let unit = ammo_def("mg_127").unwrap().fabrication_bn;
        let id = start_ammo_order(&mut w, USA, "mg_127", &district, 1, unit * 0.01).unwrap();
        let opening = resources::stockpile(&w, USA, Commodity::Coal);
        for _ in 0..110 {
            next_work(&mut w);
            assert!(validate_state(w.nation(USA)).is_ok(), "{:?}", job(&w, id));
            if job(&w, id).status == ProjectStatus::Complete {
                break;
            }
        }
        let p = job(&w, id);
        assert_eq!(
            p.completed_rounds, 1,
            "fractional work must not strand the final whole round: {p:?}"
        );
        near(p.spent_bn, unit, 1e-20);
        near(
            p.resources_used[Commodity::Coal.idx()],
            ammo_def("mg_127").unwrap().recipe[Commodity::Coal.idx()],
            1e-15,
        );
        near(
            opening - resources::stockpile(&w, USA, Commodity::Coal),
            p.resources_used[Commodity::Coal.idx()],
            2e-15,
        );
        assert_eq!(state(&w).stocks["mg_127"], 1.0);
    }
    #[test]
    fn shared_d2_authority_is_used_after_maintenance_and_never_twice_for_parallel_orders() {
        let (mut w, district) = fixture();
        let unit = ammo_def("mg_127").unwrap().fabrication_bn;
        let first = start_ammo_order(&mut w, USA, "mg_127", &district, 100, 1.0).unwrap();
        let second = start_ammo_order(&mut w, USA, "mg_127", &district, 100, 1.0).unwrap();
        set_maintenance_plan(&mut w, USA, unit * 2.0).unwrap();
        open_next(&mut w);
        {
            let b = w.nation_mut(USA).program_budget.as_mut().unwrap();
            b.available_bn[D][2] = unit * 3.0;
            b.prepaid_bn[D][2] = 0.0;
        }
        settle_support(&mut w);
        let receipt = w
            .nation(USA)
            .equipment
            .as_ref()
            .unwrap()
            .maintenance_plan
            .as_ref()
            .unwrap()
            .receipt
            .as_ref()
            .unwrap();
        near(
            receipt.custom_paid_bn + receipt.legacy_paid_bn,
            unit * 2.0,
            1e-18,
        );
        near(job(&w, first).spent_bn, unit, 1e-18);
        near(job(&w, second).spent_bn, 0.0, 1e-18);
        near(
            w.nation(USA)
                .program_budget
                .as_ref()
                .unwrap()
                .spent_today_bn[D][2],
            unit * 3.0,
            1e-18,
        );
        assert!(validate_state(w.nation(USA)).is_ok());
    }
    #[test]
    fn zero_paused_and_cancelled_orders_preserve_paid_work_and_free_only_closed_slots() {
        let (mut w, district) = fixture();
        let unit = ammo_def("mg_127").unwrap().fabrication_bn;
        let id = start_ammo_order(&mut w, USA, "mg_127", &district, 100, unit * 1.5).unwrap();
        next_work(&mut w);
        assert_eq!(job(&w, id).completed_rounds, 1);
        let paid = job(&w, id).spent_bn;
        let used = job(&w, id).resources_used;
        pause_ammo_order(&mut w, USA, id, true).unwrap();
        next_work(&mut w);
        assert_eq!(job(&w, id).spent_bn, paid);
        assert_eq!(reserved_ammunition_slots(w.nation(USA), &district), 1);
        pause_ammo_order(&mut w, USA, id, false).unwrap();
        set_ammo_funding(&mut w, USA, id, 0.0).unwrap();
        next_work(&mut w);
        assert_eq!(job(&w, id).spent_bn, paid);
        cancel_ammo_order(&mut w, USA, id).unwrap();
        assert_eq!(reserved_ammunition_slots(w.nation(USA), &district), 0);
        assert_eq!(state(&w).stocks["mg_127"], 1.0);
        assert_eq!(job(&w, id).resources_used, used);
        let saved = crate::save(&w);
        assert!(cancel_ammo_order(&mut w, USA, id).is_err());
        assert!(set_ammo_funding(&mut w, USA, id, 1.0).is_err());
        assert_eq!(crate::save(&w), saved);
        assert!(validate_state(w.nation(USA)).is_ok());
    }
    #[test]
    fn ordinary_equipment_and_ammunition_reserve_the_same_finite_factory_slots() {
        let (mut w, district) = fixture();
        let slots = crate::manufacturing::plant_slots(&w, &district);
        assert!(slots >= 3);
        start_production(&mut w, USA, "ammo-fixture", &district, 1, 0.1).unwrap();
        for _ in 1..slots {
            start_ammo_order(&mut w, USA, "mg_127", &district, 100, 0.01).unwrap();
        }
        assert_eq!(
            crate::manufacturing::used_slots(&w, USA, &district),
            slots as usize
        );
        let saved = crate::save(&w);
        assert!(start_ammo_order(&mut w, USA, "mg_127", &district, 100, 0.01).is_err());
        assert_eq!(crate::save(&w), saved);
        assert!(start_production(&mut w, USA, "ammo-fixture", &district, 1, 0.1).is_err());
        assert_eq!(crate::save(&w), saved);
    }
    #[test]
    fn missing_raw_inputs_or_loss_of_factory_control_prevent_paid_fabrication() {
        let (mut w, district) = fixture();
        let id = start_ammo_order(&mut w, USA, "mg_127", &district, 100, 0.1).unwrap();
        resources::set_stockpile_for_test(&mut w, USA, Commodity::Copper, 0.0);
        next_work(&mut w);
        assert_eq!(job(&w, id).spent_bn, 0.0);
        assert_eq!(job(&w, id).work_rounds, 0.0);
        assert!(state(&w).stocks.is_empty());
        assert!(
            raw_supply_demand(&w, USA).horizons[Commodity::Copper.idx()][0] > 0.0,
            "a shortage must retain demand for separately paid material purchases"
        );
        resources::set_stockpile_for_test(&mut w, USA, Commodity::Copper, 10.0);
        w.districts.insert(district.clone(), NationId::Canada);
        next_work(&mut w);
        assert_eq!(job(&w, id).spent_bn, 0.0);
        assert_eq!(ammunition_supply_demand(&w, USA).remaining, [0.0; 12]);
        w.districts.insert(district, USA);
        next_work(&mut w);
        assert_eq!(job(&w, id).completed_rounds, 100);
        assert!(validate_state(w.nation(USA)).is_ok());
    }
    #[test]
    fn completing_a_batch_cannot_reuse_a_lost_factory_slot_on_the_same_work_date() {
        let (mut w, district) = fixture();
        let first = start_ammo_order(&mut w, USA, "mg_127", &district, 1, 0.1).unwrap();
        let second = start_ammo_order(&mut w, USA, "mg_127", &district, 1, 0.1).unwrap();
        w.production
            .provinces
            .iter_mut()
            .find(|p| p.district == district)
            .unwrap()
            .arms_plants = 1;
        next_work(&mut w);
        assert_eq!(job(&w, first).status, ProjectStatus::Complete);
        assert_eq!(
            job(&w, second).spent_bn,
            0.0,
            "a completed batch used the only factory place for this date"
        );
        assert_eq!(state(&w).stocks["mg_127"], 1.0);
        next_work(&mut w);
        assert_eq!(job(&w, second).status, ProjectStatus::Complete);
        assert_eq!(state(&w).stocks["mg_127"], 2.0);
    }
    #[test]
    fn completing_an_ordinary_vehicle_contract_keeps_its_factory_place_until_tomorrow() {
        let (mut w, district) = fixture();
        state_mut(w.nation_mut(USA)).finance_from_day = clock::absolute_day(&w);
        for c in [Commodity::Iron, Commodity::Copper, Commodity::Coal] {
            resources::set_stockpile_for_test(&mut w, USA, c, 10_000.0);
        }
        let normal = start_production(&mut w, USA, "ammo-fixture", &district, 1, 1.0).unwrap();
        let ammo = start_ammo_order(&mut w, USA, "mg_127", &district, 1, 0.1).unwrap();
        w.production
            .provinces
            .iter_mut()
            .find(|p| p.district == district)
            .unwrap()
            .arms_plants = 1;
        let minimum = w
            .nation(USA)
            .equipment
            .as_ref()
            .unwrap()
            .projects
            .iter()
            .find(|p| p.id == normal)
            .unwrap()
            .minimum_days;
        let mut done = false;
        for _ in 0..minimum + 5 {
            open_next(&mut w);
            tick_day(&mut w);
            settle_support(&mut w);
            programs::finish_day(&mut w);
            assert_eq!(
                job(&w, ammo).spent_bn,
                0.0,
                "ordinary work has priority and keeps today's used place even when finishing"
            );
            done = w
                .nation(USA)
                .equipment
                .as_ref()
                .unwrap()
                .projects
                .iter()
                .any(|p| p.id == normal && p.status == ProjectStatus::Complete);
            if done {
                break;
            }
        }
        assert!(done);
        next_work(&mut w);
        assert_eq!(job(&w, ammo).status, ProjectStatus::Complete);
        assert!(validate_state(w.nation(USA)).is_ok());
    }
    #[test]
    fn malformed_orders_and_forged_conservation_or_dates_are_rejected() {
        let (mut w, district) = fixture();
        let saved = crate::save(&w);
        for (family, quantity, limit) in [
            ("missing", 1, 0.1),
            ("aa_missile", 1, 0.1),
            ("mg_127", 0, 0.1),
            ("mg_127", MAX_AMMO_ORDER + 1, 0.1),
            ("mg_127", 1, -1.0),
            ("mg_127", 1, f64::NAN),
        ] {
            assert!(start_ammo_order(&mut w, USA, family, &district, quantity, limit).is_err());
            assert_eq!(crate::save(&w), saved);
        }
        start_ammo_order(&mut w, USA, "mg_127", &district, 100, 0.01).unwrap();
        activate_ammunition(&mut w, USA).unwrap();
        next_work(&mut w);
        for flaw in 0..7 {
            let mut bad = w.clone();
            let a = bad
                .nation_mut(USA)
                .equipment
                .as_mut()
                .unwrap()
                .ammunition
                .as_mut()
                .unwrap();
            match flaw {
                0 => *a.stocks.get_mut("mg_127").unwrap() += 1.0,
                1 => a.orders[0].spent_bn = 0.0,
                2 => a.orders[0].recipe_per_round = [0.0; 12],
                3 => a.orders[0].rounds_per_day *= 2.0,
                4 => a.orders[0].completed_day = a.orders[0].completed_day.map(|d| d + 10),
                5 => a.active_from_day = a.active_from_day.map(|d| d - 10),
                _ => a.orders[0].resources_used[Commodity::Coal.idx()] = 0.0,
            }
            assert!(validate_state(bad.nation(USA)).is_err(), "forgery {flaw}");
            assert!(crate::load(&crate::save(&bad)).is_err());
        }
    }
    #[test]
    fn save_roundtrip_resumes_fractional_work_without_repaying_or_duplicating_stock() {
        let (mut w, district) = fixture();
        let unit = ammo_def("mg_127").unwrap().fabrication_bn;
        start_ammo_order(&mut w, USA, "mg_127", &district, 10, unit * 1.1).unwrap();
        activate_ammunition(&mut w, USA).unwrap();
        next_work(&mut w);
        let saved = crate::save(&w);
        let mut loaded = crate::load(&saved).unwrap();
        assert_eq!(crate::save(&loaded), saved);
        for _ in 0..12 {
            next_work(&mut w);
            next_work(&mut loaded);
        }
        assert_eq!(crate::save(&w), crate::save(&loaded));
        assert_eq!(state(&w).stocks["mg_127"], 10.0);
        assert_eq!(state(&w).orders[0].status, ProjectStatus::Complete);
    }
    #[test]
    fn finite_raw_forecast_uses_d2_after_maintenance_and_retains_no_parallel_d3_claim() {
        let (mut w, district) = fixture();
        let unit = ammo_def("mg_127").unwrap().fabrication_bn;
        start_ammo_order(&mut w, USA, "mg_127", &district, 30, unit).unwrap();
        start_ammo_order(&mut w, USA, "mg_127", &district, 30, unit).unwrap();
        let saved = crate::save(&w);
        let demand = ammunition_supply_demand(&w, USA);
        assert_eq!(crate::save(&w), saved);
        let copper = Commodity::Copper.idx();
        assert!(demand.horizons[copper][0] > 0.0);
        assert!(demand.horizons[copper][0] <= demand.remaining[copper]);
        assert_eq!(demand.procurement_claim_bn, [0.0; 3]);
        assert_eq!(demand.procurement_months, [0.0; 3]);
        assert!(!raw_supply_demand(&w, USA).procurement_calendar);
        near(
            raw_supply_demand(&w, USA).horizons[copper][0],
            demand.horizons[copper][0],
            1e-15,
        );
        w.year = 1991;
        w.month = 1;
        w.day = 1;
        programs::begin_day(&mut w);
        {
            let b = w.nation_mut(USA).program_budget.as_mut().unwrap();
            b.available_bn[D][2] = 0.0;
            b.prepaid_bn[D][2] = unit;
        }
        let expired = ammunition_supply_demand(&w, USA);
        near(
            expired.horizons[copper][2],
            ammo_def("mg_127").unwrap().recipe[copper],
            1e-12,
        );
        set_maintenance_plan(&mut w, USA, 1.0).unwrap();
        let maintenance_first = ammunition_supply_demand(&w, USA);
        assert_eq!(maintenance_first.horizons[copper], [0.0; 3]);
    }
    #[test]
    fn completed_and_cancelled_history_never_prevents_later_paid_replenishment() {
        let (mut w, district) = fixture();
        let unit = ammo_def("mg_127").unwrap().fabrication_bn;
        let batches = MAX_AMMO_ORDERS / 2 + 1;
        for _ in 0..batches {
            let finished = start_ammo_order(&mut w, USA, "mg_127", &district, 1, 0.1).unwrap();
            let stopped =
                start_ammo_order(&mut w, USA, "mg_127", &district, 2, unit * 1.5).unwrap();
            next_work(&mut w);
            assert_eq!(job(&w, finished).status, ProjectStatus::Complete);
            assert_eq!(job(&w, stopped).completed_rounds, 1);
            cancel_ammo_order(&mut w, USA, stopped).unwrap();
        }
        assert!(state(&w).orders.len() > MAX_AMMO_ORDERS);
        assert_eq!(
            state(&w)
                .orders
                .iter()
                .filter(|p| p.status == ProjectStatus::Complete)
                .count(),
            batches
        );
        assert_eq!(
            state(&w)
                .orders
                .iter()
                .filter(|p| p.status == ProjectStatus::Cancelled)
                .count(),
            batches
        );
        assert_eq!(state(&w).stocks["mg_127"], (batches * 2) as f64);
        let historic_paid: f64 = state(&w).orders.iter().map(|p| p.spent_bn).sum();
        near(historic_paid, batches as f64 * unit * 2.5, 1e-18);
        let saved = crate::save(&w);
        let mut loaded = crate::load(&saved).unwrap();
        assert_eq!(crate::save(&loaded), saved);
        let q = ammo_order_quote(&loaded, USA, "mg_127", &district, 2, 0.1);
        assert!(q.valid, "{:?}", q.reason);
        assert_eq!(crate::save(&loaded), saved);
        let new = start_ammo_order(&mut loaded, USA, "mg_127", &district, 2, 0.1).unwrap();
        next_work(&mut loaded);
        assert_eq!(job(&loaded, new).status, ProjectStatus::Complete);
        assert_eq!(state(&loaded).stocks["mg_127"], (batches * 2 + 2) as f64);
        assert_eq!(state(&loaded).orders.len(), batches * 2 + 1);
        assert!(validate_state(loaded.nation(USA)).is_ok());
        near(
            state(&loaded)
                .orders
                .iter()
                .map(|p| p.spent_bn)
                .sum::<f64>(),
            historic_paid + unit * 2.0,
            1e-18,
        );
    }
    #[test]
    fn unfinished_limit_counts_paused_batches_and_preserves_identifier_exhaustion() {
        let (mut w, _) = fixture();
        let districts: Vec<_> = w
            .districts
            .iter()
            .filter(|(_, owner)| **owner == USA)
            .map(|(d, _)| d.clone())
            .collect();
        for district in &districts {
            for _ in 0..crate::production::MAX_PROVINCE_LEVEL {
                crate::production::complete_capability(
                    &mut w,
                    district,
                    crate::production::ProjectKind::ArmsPlant,
                );
            }
        }
        let places: Vec<_> = districts
            .iter()
            .flat_map(|d| {
                std::iter::repeat_n(d.clone(), crate::manufacturing::plant_slots(&w, d) as usize)
            })
            .collect();
        assert!(
            places.len() > MAX_AMMO_ORDERS,
            "fixture needs spare physical capacity to isolate the concurrent batch cap"
        );
        for (i, district) in places.iter().take(MAX_AMMO_ORDERS).enumerate() {
            let id = start_ammo_order(&mut w, USA, "mg_127", district, 1, 0.1).unwrap();
            if i % 2 == 0 {
                pause_ammo_order(&mut w, USA, id, true).unwrap();
            }
        }
        assert_eq!(
            state(&w).orders.iter().filter(|p| p.paused).count(),
            MAX_AMMO_ORDERS / 2
        );
        let spare = &places[MAX_AMMO_ORDERS];
        let saved = crate::save(&w);
        let q = ammo_order_quote(&w, USA, "mg_127", spare, 1, 0.1);
        assert!(q.reason.unwrap().contains("unfinished"));
        assert!(start_ammo_order(&mut w, USA, "mg_127", spare, 1, 0.1).is_err());
        assert_eq!(crate::save(&w), saved);
        let mut malformed = w.clone();
        let a = state_mut(malformed.nation_mut(USA))
            .ammunition
            .as_mut()
            .unwrap();
        let mut extra = a.orders[0].clone();
        extra.id = a.next_id;
        a.next_id += 1;
        a.orders.push(extra);
        assert!(validate_state(malformed.nation(USA)).is_err());
        assert!(crate::load(&crate::save(&malformed)).is_err());
        cancel_ammo_order(&mut w, USA, 0).unwrap();
        let replacement = start_ammo_order(&mut w, USA, "mg_127", &places[0], 1, 0.1).unwrap();
        assert!(state(&w).orders.len() > MAX_AMMO_ORDERS);
        assert!(validate_state(w.nation(USA)).is_ok());
        cancel_ammo_order(&mut w, USA, replacement).unwrap();
        state_mut(w.nation_mut(USA))
            .ammunition
            .as_mut()
            .unwrap()
            .next_id = u32::MAX;
        let exhausted = crate::save(&w);
        let q = ammo_order_quote(&w, USA, "mg_127", &places[0], 1, 0.1);
        assert!(q.reason.unwrap().contains("identifiers"));
        assert!(start_ammo_order(&mut w, USA, "mg_127", &places[0], 1, 0.1).is_err());
        assert_eq!(crate::save(&w), exhausted);
    }
}
