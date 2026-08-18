<!-- Produced 2026-08-18 by an 18-agent swarm: six equipment decks each with an
adversarial verifier, five design agents, one synthesis. Saved verbatim because
it overturned the first implementation of arsenal.rs and the reasoning is worth
more than the conclusion. Applied progressively; see the checklist at the end of
ORDER OF WORK for what has landed. -->

# SPHERES PROCUREMENT LAYER — INTEGRATION PACKAGE

Read before applying: `spheres-sim/src/arsenal.rs` (300 lines, all of it), `war.rs:44-97` and `war.rs:343-372`, `data/mod.rs:540-580`, `tech/mod.rs:120-141, 545-620, 1442-1480`, `tech/aerospace.rs`, `politics.rs:205-245, 275-295, 505-530, 875-895`, `lib.rs:140-210, 500-560, 2090-2260`, `spheres-web/src/main.rs:500-575, 905-930`, `spheres-web/ui/index.html:703, 824-850, 974-1000, 1033-1080, 2406-2420`.

---

## 0. THE FIVE DECISIONS THAT RESOLVE THE DISAGREEMENTS

The designers and verifiers contradict each other in five places. Every one of them collapses once you notice a single fact:

> **`arsenal::strength_of` is the wrong quantity to give `war.rs`. `book_value` is the right one.**

**Decision 1 — The arsenal enters `war.rs` as a money-against-money ratio, never as a strength addend, and never through the `quality` column.**
Five separate verifiers found the `quality` column wrong by between 5× and 10⁴×, in opposite directions, in five different classes. That is not a fixable data problem; it is a signal that no scalar built out of authored `quality` judgements can be trusted to set the scale of `mil_strength`. `unit_cost` is transcribed money under iron rule 4 and is the only column in the deck that is *fact*. So the war model reads `book_value` (Σ units × unit_cost × condition, in $bn) against what a properly-funded force *would* hold, and the ratio is bounded. **The entire quality column becomes non-load-bearing for the war model.** Every "off by 10³" finding — D2 in the audit, the Missile deck's whole-deck problem, the Armour deck's out-produce break, the JDAM 16,667 monoculture — stops being a calibration hazard the moment the deck stops setting scale.

**Decision 2 — Reject the war-model designer's two-term `coverage × composition`. Ship coverage only.**
`composition` re-imports every quality-column error through the back door, and the designer's own §4 C3 admits every AI nation would max it for free. Coverage alone is defensible, checkable against the BEA gross stock of national-defence equipment, and does the job. `quality` survives to do exactly two things: order kits *within a class* in `plan()`, and render a number in the UI. **No code compares quality across classes.** State this in `EquipmentDef`'s doc so no future author tries.

**Decision 3 — Seed on money, not on strength.** The seeding designer's §1(c) found a 630× spread in the required strength-per-dollar across the roster and concluded no global deck fits. That is true and it is fatal to `strength_of(n) == n.mil_strength`. Seeding to *book value* has no such constraint: `book_value == want` by construction for every nation, so `equipped_fraction == 1.0` exactly at t=0 for all 137, with zero per-nation tuning. The 630× problem does not exist in money space. **This is the single most important consequence of Decision 1 and it is why the Gulf tests survive.**

**Decision 4 — `lead_months` is the programme span from decision to first delivery. `floor + lead` may land earlier than the real in-service date, and that is correct.**
Several verifiers applied a rule ("floor + lead must land near the real service entry") and then flagged su35s, kc46, starshield, seawolf_ssn, virginia_ssn, type212a and qec_cv as anachronisms. The rule is wrong: it charges the delay twice, once at the technology floor and once in the lead. The tech floor says *when the enabling technology first existed anywhere*. The lead says *how long a government that decides today waits*. A nation that decides in 1990 to build a boom-refuelling tanker gets one before Boeing did, because it decided twenty years earlier. That is the game working. The only hard error is a designation fielded before its enabling technology existed, which the floor makes impossible. Where the real programme was late because of false starts, put the false starts in the lead (KC-46 at 276 months, not 96) — that is honest and it is what those justifications already describe.

**Decision 5 — Cap NEW technologies at six for the whole package, and price the four Information-era ones above the existing Information ceiling.**
The five decks proposed ~73 new `aero_` techs. That would take Aerospace from 32 to 105 entries, a 228% enlargement of one of eight domains, moving every knowledge-share and convergence denominator in the tree and putting `china_growth_miracle` (median 11.16× against a floor of 11.0×, a 1.5% margin) at real risk. Existing Aerospace Information entries top out at cost 106. **Every new Information-era tech is priced 107–110.** Since `pick_focus` is cheapest-first within a domain, that guarantees all thirteen existing Information entries are researched in exactly the order they are today before any new one is eligible to be picked — the 1990-99 research sequence is preserved by construction, which is what protects `gulf_war_emerges` and `desert_storm_is_quick_when_they_stand_and_fight`. This kills the "four cheap 1990 roots reorder research and compress the US:Iraq quality ratio" finding outright. Everything else is re-gated onto the existing 32 `aero_` ids, the `core_` foundation set, or `comm_` ids (cross-domain gating works — `tech::index_of` searches the whole registry; `falcon9`'s `core_reusable_booster` is the precedent).

Also decided, briefly:
- **Tech-domain split (8→11): NOT in this package.** The split's own author recommended after, and is right: `available()` is domain-blind, so the equipment layer ships unmodified against the single Aerospace domain. Land only the three-line `ensure_shape` guard (§7 of that design) — it never fires on a current save, moves no hash, and permanently removes the class of bug where an 8-element `focus`/`progress` vector is resized into an 11-domain world and silently spends Biotech money on an Air project.
- **Launch vehicles (titan_iv, delta_ii, falcon9, starship): CUT.** A rocket is expended on one flight; `arsenal.rs` has no expenditure path; nothing gates a satellite order on holding a booster. Four entries that add permanent strength for free and model a consumable as a 25-year asset. Cut them and drop `aero_expendable_launch_vehicle` and `aero_fully_reusable_heavy_lift` with them.
- **Individual 1990 infantry kit (PASGT vest, PVS-7B, TOW-2A, Stinger RMP): CUT.** These are not national procurement decisions at the level this model runs at; they live inside a brigade set. Cutting them removes four zero-prereq 1990 techs and the entire "1990 strength-per-dollar ceiling rises 7.6×" finding.

---

## 1. THE FINAL DECK

### 1.1 Schema changes to `EquipmentDef` (required before any of this pastes)

```rust
/// What a piece of kit is *for*. The war model reads different classes of thing
/// differently: a frigate and a JDAM tail kit are both defence equipment and
/// both belong in the books, but only one of them is force structure.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum Role {
    /// Counts as force structure. Ships, aircraft, formations, launchers, radars.
    Line,
    /// Reach, not combat power. Tankers, airlifters, ro-ro hulls, amphibious
    /// shipping. In the books, out of `strength_of` — a KC-135R is worth more to
    /// a war than a Strike Eagle and it wins no engagement whatsoever. When
    /// `war.rs` gains a lift multiplier this is the term it reads.
    Lift,
    /// Deterrence. A missile boat contributes nothing to a conventional fight and
    /// `Nation.nuclear` plus the taboo mechanic already carry what it does.
    Deterrent,
    /// Expendable. Rounds, bombs, one-way drones. `war.rs` already models this as
    /// `n.munitions` with `BURN_BY_RUNG` and `MAGAZINE_REBUILD`; these entries
    /// feed that stock, never `strength_of`, or the sim double-counts BIBLE §6's
    /// second stock as an infinite non-depleting asset.
    Magazine,
}

pub struct EquipmentDef {
    pub id: &'static str,
    pub name: &'static str,
    pub class: Class,
    pub role: Role,
    /// The technology that permits it. `None` means the legacy tier: what a
    /// 1990 government already knew how to build, and what a state with no
    /// research programme at all still buys.
    pub tech: Option<&'static str>,
    /// Strength per unit held. **Only ever compared within a class** — `plan()`
    /// orders kits by `quality / unit_cost` inside one class and nothing
    /// anywhere compares a tank to a frigate. Cross-class quality ratios in this
    /// table are meaningless by construction and must stay that way.
    pub quality: f64,
    /// $bn per unit, 1990 dollars. The only transcribed column, and the only one
    /// the war model reads (through `book_value`).
    pub unit_cost: f64,
    pub lead_months: u32,
    /// Months from delivery to residual value. NOT a warranty period — see the
    /// new `condition()`.
    pub service_months: u32,
    /// The largest share of a nation's monthly procurement line one programme can
    /// absorb. A yard is a yard: the United States cannot put its whole defence
    /// budget through Newport News. Spill goes to the next kit in the class.
    pub max_share: f64,
}
```

`kit(...)` gains `role`, `tech` becomes `Option`, and gains `max_share`. Below, `L`/`F`/`D`/`M` abbreviate `Role::Line`/`Lift`/`Deterrent`/`Magazine` for legibility; write them out in source.

### 1.2 Legacy tier — ungated, `tech: None` (18 entries)

**This tier is the precondition for everything else.** `TechState::new` sets `known: vec![]`, so on 1 Jan 1990 `available()` returns empty for all 137 nations, `pick()` returns `None`, and nothing in the current DECK is orderable by anybody. Without this tier there is nothing to seed 1990 with and no nation can buy anything until it researches its first `aero_` tech — which most of the roster never would.

A **unit is a formation block**, not an airframe. That is what keeps the arsenal screen legible and what makes 137 nations' worth of holdings ~10 rows each rather than 40,000 aeroplanes. Names are cohort labels, not fabricated national designations: the data does not record which tank Chad has, and a generation label is the truthful representation of what is known. Name the exemplars in the doc comment to satisfy BIBLE §3.1 (`arm_gen3` = M1A1 / Leopard 2 / T-80U / Challenger 1).

```rust
// ---- Legacy: what a 1990 government already knew how to build ----
kit("inf_light",     "Light Infantry Brigade Set",         Infantry, L, None,  8.0,  0.90,  24, 216, 0.35),
kit("inf_mech",      "Mechanised Infantry Brigade Set",    Infantry, L, None, 30.0,  2.40,  36, 240, 0.35),
kit("arm_gen2",      "Second-Generation Tank Regiment",    Armour,   L, None,  8.0,  1.10,  36, 240, 0.35),
kit("arm_gen3",      "Third-Generation Tank Regiment",     Armour,   L, None, 44.0,  3.40,  48, 360, 0.35),
kit("air_gen2",      "Second-Generation Fighter Squadron", Air,      L, None,  4.5,  0.70,  36, 300, 0.30),
kit("air_gen3",      "Third-Generation Fighter Squadron",  Air,      L, None, 19.0,  1.90,  48, 330, 0.30),
kit("air_gen4",      "Fourth-Generation Fighter Squadron", Air,      L, None, 64.0,  4.60,  72, 420, 0.30),
kit("air_rotary",    "Army Aviation Regiment",             Air,      L, None, 16.0,  1.60,  48, 300, 0.25),
kit("air_lift",      "Tactical Transport Squadron",        Air,      F, None, 12.0,  2.20,  60, 480, 0.20),
kit("nav_patrol",    "Patrol and Fast-Attack Flotilla",    Naval,    L, None,  2.0,  0.35,  36, 300, 0.25),
kit("nav_escort",    "Frigate and Destroyer Squadron",     Naval,    L, None, 34.0,  4.20,  84, 420, 0.25),
kit("nav_ssk",       "Diesel-Electric Submarine Flotilla", Naval,    L, None, 22.0,  2.60,  96, 420, 0.20),
kit("nav_blue",      "Blue-Water Task Group",              Naval,    L, None,160.0, 17.00, 120, 540, 0.15),
kit("nav_lift",      "Amphibious and Sealift Group",       Naval,    F, None, 16.0,  3.00,  72, 480, 0.15),
kit("msl_sam",       "Strategic Air-Defence Belt",         Missile,  L, None, 14.0,  1.50,  36, 300, 0.25),
kit("msl_brm",       "Ballistic Missile Brigade",          Missile,  L, None,  7.0,  0.90,  36, 300, 0.20),
kit("msl_deterrent", "Strategic Deterrent Force",          Missile,  D, None, 60.0, 14.00, 120, 420, 0.15),
kit("spc_recon",     "Photoreconnaissance Constellation",  Space,    L, None, 20.0,  3.80,  60, 144, 0.10),
```

Legacy lead times (24–120 months) are deliberately far shorter than `f35a`'s 168 or `sixthgen`'s 216. **This sharpens the thesis rather than weakening it:** you *can* reorder T-72s in three years. What you cannot do is start a fifth-generation programme when the war arrives. The lead-time cliff lives between the legacy tier and the tech-gated tier, which is exactly where it belongs.

### 1.3 Armour (16)

```rust
kit("trophy",      "Trophy Active Protection",   Armour, L, Some("aero_active_protection_system"),   0.15, 0.0005,  48, 300, 0.20),
kit("m1a1",        "M1A1 Abrams",                Armour, L, Some("aero_third_generation_armour"),    1.00, 0.0043,  48, 480, 0.25),
kit("m2a2",        "M2A2 Bradley",               Armour, L, Some("aero_third_generation_armour"),    0.60, 0.0032,  48, 480, 0.25),
kit("m109a6",      "M109A6 Paladin",             Armour, L, Some("aero_third_generation_armour"),    0.70, 0.0025,  96, 540, 0.20),
kit("t90",         "T-90",                       Armour, L, Some("aero_third_generation_armour"),    0.85, 0.0022,  60, 420, 0.25),
kit("leopard2a5",  "Leopard 2A5",                Armour, L, Some("aero_third_generation_armour"),    1.20, 0.0050,  84, 540, 0.25),
kit("challenger2", "Challenger 2",               Armour, L, Some("aero_third_generation_armour"),    1.10, 0.0065, 144, 420, 0.20),
kit("m1a2sep",     "M1A2 SEP Abrams",            Armour, L, Some("aero_tactical_datalink"),          1.35, 0.0087,  72, 480, 0.25),
kit("matv",        "M-ATV",                      Armour, L, Some("aero_third_generation_armour"),    0.10, 0.0013,  24, 180, 0.20),
kit("m1150",       "M1150 Assault Breacher Veh", Armour, L, Some("aero_third_generation_armour"),    0.50, 0.0037, 228, 480, 0.15),
kit("merkava4m",   "Merkava Mk 4M",              Armour, L, Some("aero_active_protection_system"),   1.40, 0.0065,  72, 480, 0.25),
kit("type99a",     "ZTZ-99A (Type 99A)",         Armour, L, Some("aero_third_generation_armour"),    1.20, 0.0035, 264, 420, 0.25),
kit("k2",          "K2 Black Panther",           Armour, L, Some("aero_active_protection_system"),   1.50, 0.0085, 180, 480, 0.25),
kit("t14",         "T-14 Armata",                Armour, L, Some("aero_active_protection_system"),   1.40, 0.0060, 120, 420, 0.25),
kit("archer",      "Archer FH77 BW L52",         Armour, L, Some("aero_gps_guided_munition"),        1.10, 0.0050, 168, 420, 0.20),
kit("mgcs",        "Main Ground Combat System",  Armour, L, Some("aero_autonomous_targeting"),       2.20, 0.0400, 240, 600, 0.15),
```

Fixes applied vs the submitted deck: `trophy` rebased to 0.15/$0.0005bn (it was worth exactly as much as the entire Merkava it bolts onto, at ~10× the real Trophy unit price); `matv` cut to 0.10 (the entry written to prove buying lead time down is expensive was simultaneously the entry that made it the optimal play); `m1150` restored to its true 228 months (the author's self-declared falsification was made only to avoid out-running the sixth-gen fighter, and nothing in the code compares lead times across entries); `m109a6` 54→96, `archer` 120→168, `type99a` 120→264, `challenger2` 132→144 (four leads that were roughly half what their own justifications stated, on a module whose entire thesis is lead time); `m1a2sep` re-gated off `aero_network_centric_warfare` (2003 floor for a vehicle fielded 1999) onto `aero_tactical_datalink`; `merkava4m` lead 108→72 so it lands 2017 against a real 2009; unit costs reconciled to their own prose (m109a6 0.0027→0.0025, challenger2 0.0068→0.0065, archer 0.0048→0.0050, leopard2a5 0.0055→0.0050). Cut: `cv9040`, `rcv` (RCV was terminated outright in the 2025 Army Transformation Initiative, not restructured), `xm30`, `m1e3` (all needed dedicated new techs the budget does not have).

### 1.4 Air (31 — 13 edited in place, 18 added)

```rust
// existing, edited in place
kit("f15e",     "F-15E Strike Eagle",              Air, L, Some("aero_pulse_doppler_radar"),          2.40, 0.090,  60, 480, 0.30),
kit("f117",     "F-117 Nighthawk",                 Air, L, Some("aero_stealth_shaping"),              2.80, 0.110,  96, 360, 0.25),
kit("e3",       "E-3 Sentry AWACS",                Air, L, Some("aero_airborne_battle_management"),   3.60, 0.280,  84, 540, 0.15),
kit("b2",       "B-2 Spirit",                      Air, L, Some("aero_flying_wing_stealth"),          6.00, 1.100, 144, 540, 0.15),
kit("predator", "RQ-1 Predator",                   Air, L, Some("aero_unmanned_aircraft"),            0.70, 0.020,  36, 240, 0.20),
kit("mq1b",     "MQ-1B Predator",                  Air, L, Some("aero_armed_uav"),                    1.10, 0.025,  36, 240, 0.20),
kit("f22",      "F-22 Raptor",                     Air, L, Some("aero_stealth_air_superiority"),      5.20, 0.200, 180, 480, 0.30),
kit("ea18g",    "EA-18G Growler",                  Air, L, Some("aero_electronic_attack"),            3.10, 0.100,  84, 420, 0.20),
kit("rq170",    "RQ-170 Sentinel",                 Air, L, Some("aero_stealth_uav"),                  1.90, 0.045,  72, 240, 0.15),
kit("f35a",     "F-35A Lightning II",              Air, L, Some("aero_stealth_multirole"),            4.60, 0.090, 168, 480, 0.30),
kit("cca",      "Collaborative Combat Aircraft",   Air, L, Some("aero_collaborative_combat_aircraft"),2.20, 0.045,  96, 300, 0.25),
kit("sixthgen", "Sixth-Generation Fighter",        Air, L, Some("aero_sixth_gen_air_dominance"),      7.50, 0.300, 216, 540, 0.30),
kit("aesa",     "AESA Radar Refit",                Air, L, Some("aero_aesa_radar"),                   1.30, 0.030,  48, 300, 0.20),
// added
kit("f16c",     "F-16C Block 50 Fighting Falcon",  Air, L, Some("aero_pulse_doppler_radar"),          1.90, 0.035,  48, 480, 0.30),
kit("su27s",    "Su-27S Flanker-B",                Air, L, Some("aero_pulse_doppler_radar"),          2.50, 0.045, 120, 480, 0.30),
kit("fa18ef",   "F/A-18E/F Super Hornet",          Air, L, Some("aero_pulse_doppler_radar"),          2.60, 0.060, 120, 420, 0.30),
kit("gripen",   "JAS 39A Gripen",                  Air, L, Some("aero_pulse_doppler_radar"),          2.00, 0.040, 132, 420, 0.25),
kit("rafale",   "Dassault Rafale",                 Air, L, Some("aero_pulse_doppler_radar"),          3.50, 0.080, 180, 480, 0.30),
kit("typhoon",  "Eurofighter Typhoon",             Air, L, Some("aero_pulse_doppler_radar"),          3.60, 0.090, 192, 480, 0.30),
kit("su35s",    "Su-35S Flanker-E",                Air, L, Some("aero_pulse_doppler_radar"),          3.30, 0.060, 288, 420, 0.30),
kit("ah64d",    "AH-64D Apache Longbow",           Air, L, Some("aero_pulse_doppler_radar"),          2.20, 0.045,  84, 480, 0.25),
kit("b1b",      "B-1B Lancer",                     Air, L, Some("aero_stealth_shaping"),              4.20, 0.280,  66, 540, 0.15),
kit("e7",       "E-7A Wedgetail",                  Air, L, Some("aero_airborne_battle_management"),   4.00, 0.350, 144, 480, 0.15),
kit("j20",      "J-20 Mighty Dragon",              Air, L, Some("aero_stealth_air_superiority"),      4.30, 0.110, 156, 480, 0.30),
kit("mq9",      "MQ-9A Reaper",                    Air, L, Some("aero_armed_uav"),                    1.60, 0.030,  48, 240, 0.20),
kit("b21",      "B-21 Raider",                     Air, L, Some("aero_stealth_multirole"),            6.20, 0.600, 132, 600, 0.15),
kit("gcap",     "Global Combat Air Programme",     Air, L, Some("aero_sixth_gen_air_dominance"),      6.90, 0.240,  24, 540, 0.30),
kit("kc135r",   "KC-135R Stratotanker",            Air, F, Some("aero_strategic_mobility"),           0.80, 0.050,  60, 480, 0.15),
kit("kc46",     "KC-46A Pegasus",                  Air, F, Some("aero_strategic_mobility"),           1.00, 0.140, 276, 600, 0.15),
kit("c17a",     "C-17A Globemaster III",           Air, F, Some("aero_strategic_mobility"),           1.00, 0.220, 156, 540, 0.15),
kit("v22",      "MV-22B Osprey",                   Air, F, Some("aero_strategic_mobility"),           0.90, 0.070, 300, 420, 0.15),
```

Fixes: `kc135r` from quality 2.6 (above the F-15E anchor) and 720 service months down to 0.8/480 and made `Role::Lift` — it was the single best out-produce-a-war lever in the whole package and it was a tanker; `c17a`/`kc46`/`v22` likewise repriced to Lift and to honest combat values, which is possible now that `Lift` exists as a role rather than being solved by lying about combat value; `kc46` lead 96→276 (the three cancelled KC-X competitions 1996–2011 belong in the lead, not in a fictional second tech floor); `su35s` lead 144→288; `gripen` renamed to the A-model whose dates the justification actually gives, lead 156→132; `typhoon` quality 3.4→3.6 so it is not strictly dominated by the Rafale on every axis; `gcap` lead 204→24 and cost 0.28→0.24 (the 2037 floor already encodes the seventeen-year concept-to-service delay; re-applying it delivered a 2054 aircraft that `plan()` would never pick); `b21` 0.38→0.60 and `mq9` 0.012→0.017→0.030 (system, not airframe) to match their own citations; `predator`/`mq1b`/`rq170`/`cca`/`aesa` repriced from bare airframe to system-with-ground-station, which is what a nation actually buys and what pulls them into the class's density band. `b1b` keeps `aero_stealth_shaping` with a TODO comment naming `aero_low_level_penetration` as the correct gate — the deck's own justification admits variable geometry and terrain-following, not low observability, is the B-1's defining engineering. Cut: `bwb`, `mq25`.

### 1.5 Naval (18 — 2 edited, 1 deleted, 15 added)

```rust
kit("la_ssn",       "Los Angeles-class SSN",        Naval, L, Some("aero_quiet_submarine"),           5.40, 0.900, 108, 396, 0.20),
kit("laws",         "Shipboard Directed-Energy Mount",Naval,L,Some("aero_directed_energy_laser"),     1.00, 0.060,  72, 300, 0.15),
kit("seawolf_ssn",  "Seawolf-class SSN",            Naval, L, Some("aero_quiet_submarine"),           8.60, 2.450, 168, 480, 0.15),
kit("virginia_ssn", "Virginia-class SSN (Block III)",Naval,L, Some("aero_quiet_submarine"),           6.80, 1.450, 132, 396, 0.20),
kit("type212a",     "Type 212A",                    Naval, L, Some("aero_air_independent_submarine"), 3.40, 0.380,  96, 420, 0.20),
kit("ohio_ssbn",    "Ohio-class SSBN",              Naval, D, Some("aero_naval_nuclear_propulsion"),  7.50, 2.200, 132, 504, 0.15),
kit("columbia_ssbn","Columbia-class SSBN",          Naval, D, Some("aero_naval_nuclear_propulsion"), 10.00, 4.200, 288, 504, 0.15),
kit("nimitz_cvn",   "Nimitz-class CVN",             Naval, L, Some("aero_naval_nuclear_propulsion"), 22.00, 3.600, 126, 600, 0.12),
kit("ford_cvn",     "Gerald R. Ford-class CVN",     Naval, L, Some("aero_naval_nuclear_propulsion"), 30.00, 7.000, 300, 600, 0.12),
kit("burke_ddg",    "Arleigh Burke-class DDG (Fl I)",Naval,L, Some("aero_pulse_doppler_radar"),       4.50, 0.900, 132, 480, 0.20),
kit("fremm_ffg",    "FREMM multipurpose frigate",   Naval, L, Some("aero_pulse_doppler_radar"),       3.60, 0.480, 132, 360, 0.20),
kit("type45_ddg",   "Type 45 Daring-class",         Naval, L, Some("aero_aesa_radar"),                5.00, 0.950, 156, 360, 0.20),
kit("burke_f3",     "Arleigh Burke-class Flight III",Naval,L, Some("aero_aesa_radar"),                6.20, 1.050, 168, 480, 0.20),
kit("zumwalt_ddg",  "Zumwalt-class DDG",            Naval, L, Some("aero_aesa_radar"),                7.00, 2.300, 192, 480, 0.15),
kit("qec_cv",       "Queen Elizabeth-class",        Naval, L, Some("aero_stealth_air_superiority"),  14.00, 2.100, 180, 600, 0.12),
kit("america_lha",  "America-class LHA",            Naval, F, Some("aero_stealth_air_superiority"),   9.00, 1.850, 156, 480, 0.12),
kit("wasp_lhd",     "Wasp-class LHD",               Naval, F, Some("aero_strategic_mobility"),        6.00, 0.750, 108, 480, 0.12),
kit("lmsr_sealift", "Bob Hope-class LMSR",          Naval, F, Some("aero_strategic_mobility"),        2.20, 0.240,  84, 480, 0.15),
```

`aip_ssk` is **deleted** — `type212a` supersedes it, both hung off the same tech, and keeping both leaves `aip_ssk` as permanently unreachable dead data. `la_ssn` is **edited in place**, not re-declared: a second `la_ssn` would be silently unreachable through `index_of` (which uses `position()`) while still live in `available()` and `plan()`. The whole submitted naval tech block is gone: `aero_naval_nuclear_propulsion` is the one entry that survives (its proposed prereq "core-level nuclear engineering" does not exist and would have panicked `tree_is_well_formed`; it takes `prereqs: &[]` like `aero_quiet_submarine`); `aero_stovl_carrier_aviation` (floor 2011, prereq floor 2016 — a guaranteed panic) is replaced by gating `qec_cv` and `america_lha` on `aero_stealth_air_superiority`; `aero_uncrewed_undersea_vehicle` (floor 2019 outside the Intelligent window 2020–2029, *and* a prereq year violation) dies with `ssnx`; `aero_pump_jet_propulsor` was two-thirds verbatim inside `aero_quiet_submarine`'s existing comment and is dropped, with `seawolf_ssn` hung directly off `aero_quiet_submarine` — the 168-month lead was always the real gate; `aero_electromagnetic_launch`'s nuclear prereq is falsified by Fujian and is dropped with the node. `ohio_ssbn` raised to 2.20 (early-1980s dollars inflate *up* to 1990 at CPI 130.7/90.9 = 1.44; it was the one cost in the whole package leaning the wrong way, and it flattered the deck's own argument about deterrence being poor value). `wasp_lhd` to 0.75 and lead 108 to match its own prose. `ohio_ssbn`, `columbia_ssbn` → `Role::Deterrent` and `lmsr_sealift`, `wasp_lhd`, `america_lha` → `Role::Lift`: this is the fix for "a boomer is a better conventional combatant than a Virginia" and "an unarmed ro-ro ferry sums at 92% of an F-15E", and it is the fix the naval verifier asked for by name. Cut: `ssnx`, `ddgx`, `vpm_ssn`.

### 1.6 Infantry (11 — 6 edited, 5 added)

```rust
kit("rq11b",           "RQ-11B Raven",                   Infantry, L, Some("aero_small_uas"),                0.12, 0.00006,  24,  96, 0.20),
kit("switchblade_300", "Switchblade 300",                Infantry, M, Some("aero_loitering_munition"),       0.06, 0.00006,  24, 120, 0.15),
kit("cuas",            "Counter-UAS Battery",            Infantry, L, Some("aero_counter_uas_layered"),      1.00, 0.00500,  36, 240, 0.20),
kit("link16",          "Tactical Data Link Fit",         Infantry, L, Some("aero_tactical_datalink"),        1.50, 0.00800,  48, 360, 0.20),
kit("c4isr",           "Networked C4ISR",                Infantry, L, Some("aero_network_centric_warfare"),  2.60, 0.03000,  72, 360, 0.20),
kit("atr",             "Autonomous Target Recognition",  Infantry, L, Some("aero_autonomous_targeting"),     2.30, 0.02000,  84, 300, 0.20),
kit("javelin",         "FGM-148 Javelin",                Infantry, M, Some("aero_precision_munitions"),      0.08, 0.00012, 120, 240, 0.15),
kit("nlaw",            "NLAW (MBT LAW)",                 Infantry, M, Some("aero_precision_munitions"),      0.05, 0.00003, 228, 240, 0.15),
kit("prc117g",         "AN/PRC-117G Manpack Radio",      Infantry, L, Some("aero_network_centric_warfare"),  0.09, 0.00005,  72, 180, 0.20),
kit("ngsri",           "Next Generation Short Range Interceptor", Infantry, L, Some("aero_counter_uas_layered"), 0.10, 0.00040, 48, 240, 0.20),
kit("sbmc_ivas",       "Soldier Borne Mission Command (IVAS)",    Infantry, L, Some("aero_autonomous_targeting"), 0.22, 0.00040, 72,  96, 0.20),
```

`raven` and `switchblade` are **renamed in place** to `rq11b` and `switchblade_300` — appending the new ids would leave DECK carrying "RQ-11 Raven" and "RQ-11B Raven" simultaneously. All fourteen proposed infantry techs are gone: the four 1990 roots' contents are absorbed by `inf_light`/`inf_mech`, `aero_predicted_line_of_sight` had two hard test failures (a 2009 floor outside the Platform window, and a prereq `core_mems_inertial` that is not one of the fourteen `core_` ids), `prc117g` is re-gated off `aero_tactical_datalink` (1994 — a thirteen-year anachronism for a 2009 software-defined radio) onto `aero_network_centric_warfare` with a 72-month lead, and `nlaw` gets a 228-month lead on `aero_precision_munitions` to land 2009, its real British in-service year. Nothing in this class is below 24 months: nine of twenty submitted entries delivered inside eighteen, which would have exempted half the class from the module's thesis, and the deck's own cited evidence (2022 Javelin reorder leads of 24–32 months, Stinger restart 30–48) contradicts the short figures. Anti-armour/AA qualities divided by 5–8 as the verifier asked — a Javelin at 0.55 exceeded the whole Switchblade, and a Stinger at 0.30 equalled a Shahed-136 with 1,000 km of range.

### 1.7 Missile (21 — 8 edited/renamed in place, 13 added)

```rust
kit("paveway",      "GBU-24 Paveway III",           Missile, M, Some("aero_precision_munitions"),        0.35, 0.00040,  24, 300, 0.15),
kit("jdam",         "GBU-31(V)1/B JDAM",            Missile, M, Some("aero_gps_guided_munition"),        0.10, 0.00003,  24, 300, 0.15),
kit("tomahawk",     "BGM-109 Tomahawk Block III",   Missile, M, Some("aero_cruise_missile"),             1.20, 0.00160,  36, 360, 0.15),
kit("kalibr",       "3M-14 Kalibr",                 Missile, M, Some("aero_cruise_missile"),             1.10, 0.00130,  42, 300, 0.15),
kit("jassm",        "AGM-158A JASSM",               Missile, M, Some("aero_cruise_missile"),             1.30, 0.00080, 180, 360, 0.15),
kit("storm_shadow", "Storm Shadow / SCALP EG",      Missile, M, Some("aero_cruise_missile"),             1.40, 0.00120, 156, 360, 0.15),
kit("atacms",       "MGM-140A ATACMS Block I",      Missile, M, Some("aero_tactical_ballistic_missile"), 1.00, 0.00090,  42, 360, 0.15),
kit("iskander",     "9K720 Iskander-M",             Missile, M, Some("aero_tactical_ballistic_missile"), 2.40, 0.00300, 180, 360, 0.15),
kit("himars",       "M142 HIMARS with M31 GMLRS",   Missile, L, Some("aero_precision_rocket_artillery"), 1.80, 0.00600,  48, 360, 0.20),
kit("owa",          "One-Way Attack Drone",         Missile, M, Some("aero_attritable_strike_drone"),    0.30, 0.00005,  24, 120, 0.15),
kit("x51",          "Scramjet Test Vehicle",        Missile, L, Some("aero_scramjet_propulsion"),        1.60, 0.05000, 132, 240, 0.15),
kit("avangard",     "Avangard (15Yu71)",            Missile, M, Some("aero_hypersonic_glide_vehicle"),   4.10, 0.09000, 144, 300, 0.15),
kit("lrhw",         "LRHW Dark Eagle",              Missile, M, Some("aero_hypersonic_glide_vehicle"),   3.60, 0.06000, 132, 300, 0.15),
kit("kinzhal",      "Kh-47M2 Kinzhal",              Missile, M, Some("aero_hypersonic_glide_vehicle"),   3.00, 0.01000,  24, 300, 0.15),
kit("patriot_pac2", "MIM-104C Patriot PAC-2",       Missile, L, Some("aero_theater_missile_defense"),    2.10, 0.02000,  60, 420, 0.20),
kit("s300pmu",      "S-300PMU (SA-10C Grumble)",    Missile, L, Some("aero_theater_missile_defense"),    2.30, 0.00700,  60, 420, 0.20),
kit("patriot_pac3", "MIM-104F Patriot PAC-3",       Missile, L, Some("aero_hit_to_kill_interceptor"),    3.20, 0.09000,  72, 420, 0.20),
kit("thaad",        "Terminal High Altitude Area Defense", Missile, L, Some("aero_hit_to_kill_interceptor"), 4.20, 0.32000, 96, 420, 0.15),
kit("s400",         "S-400 Triumf",                 Missile, L, Some("aero_aesa_radar"),                 3.40, 0.06500,  84, 420, 0.20),
kit("sm3_1b",       "RIM-161B SM-3 Block IB",       Missile, L, Some("aero_midcourse_defense"),          2.00, 0.01200,  84, 360, 0.20),
kit("gbi",          "Ground-Based Interceptor",     Missile, L, Some("aero_midcourse_defense"),          3.40, 0.07000, 120, 480, 0.15),
```

`patriot` → `patriot_pac2` and `hgv` → `avangard` are **renames in place**; `paveway`, `jdam` and `tomahawk` are **edits in place** (the deck claimed these three were "re-emitted byte-identical" — two of the three change name, and a naive append would leave duplicate ids where `index_of`'s `position()` resolves to the old row while the new names sit unreachable and `available()` returns both, showing two Tomahawk lines in the UI). `patriot_pac3` renamed off "MSE" (production 2015) to the CRI-era designation its 2001 gate actually names, per BIBLE §3.1. `thaad` 0.085→0.32 and `patriot_pac3` 0.045→0.09: both were priced below their own ammunition — the THAAD entry's own figures give $100M of interceptors on a $85M fire unit, and its own cited Saudi package works out at ~$340M per launcher-with-eight-rounds. `s300pmu` 0.015→0.007 and `s400` 0.03→0.065: both justifications stated arithmetic that produced a different number than the field beside it, in opposite directions. `lrhw` 4.4→3.6 (it rated a conventional intermediate-range weapon above a nuclear-armed ICBM-boosted glide vehicle). `jdam` 0.5→0.10 to stop it dominating its own class under `plan()`. **The Magazine role is what actually fixes this class**: nine of these are rounds, `war.rs` already models rounds as `n.munitions` with a burn rate and a refill rate, and the submitted deck double-modelled them as infinite non-depleting strength. Cut: `ngi`, `brilliant_pebbles`.

### 1.8 Space (14 — 1 edited, 13 added)

```rust
kit("kh11",         "KH-11 KENNEN",                 Space, L, Some("aero_recon_satellite"),       3.20, 0.350,  96, 180, 0.10),
kit("lacrosse",     "Lacrosse/Onyx",                Space, L, Some("aero_recon_satellite"),       3.00, 0.550,  84, 150, 0.10),
kit("dsp",          "Defense Support Program Satellite", Space, L, Some("aero_recon_satellite"),  3.00, 0.300,  72, 144, 0.10),
kit("worldview3",   "WorldView-3 Imagery Access",   Space, L, Some("aero_recon_satellite"),       1.50, 0.030,  54,  60, 0.10),
kit("gps_iia",      "NAVSTAR GPS Block IIA",        Space, L, Some("core_gnss"),                  1.50, 0.045, 144, 180, 0.10),
kit("milstar",      "Milstar",                      Space, L, Some("aero_tactical_datalink"),     3.80, 3.500, 156, 168, 0.10),
kit("aehf",         "AEHF",                         Space, L, Some("aero_network_centric_warfare"),4.50, 2.400, 120, 168, 0.10),
kit("sbirs",        "SBIRS GEO",                    Space, L, Some("aero_midcourse_defense"),     4.40, 0.900,  84, 144, 0.10),
kit("gssap",        "GSSAP",                        Space, L, Some("aero_aesa_radar"),            1.70, 0.110, 144, 120, 0.10),
kit("ccs",          "Counter Communications System",Space, L, Some("aero_electronic_attack"),     0.80, 0.020,  42, 180, 0.10),
kit("iceye",        "ICEYE SAR Microsatellite",     Space, L, Some("comm_smallsat_bus"),          0.45, 0.012,  30,  60, 0.10),
kit("gps_iii",      "GPS III",                      Space, L, Some("comm_gnss_sovereign"),        2.60, 0.200,  24, 180, 0.10),
kit("galileo",      "Galileo FOC Satellite",        Space, L, Some("comm_gnss_sovereign"),        1.80, 0.650,  24, 156, 0.10),
kit("sda_tracking", "SDA Tracking Layer Satellite", Space, L, Some("comm_leo_broadband_constellation"), 1.10, 0.035, 33, 60, 0.10),
```

`kh11` **edited in place** at its existing DECK position (service 300→180, well sourced off USA-129, Dec 1996 to 2014). Six proposed space techs deleted as duplicates of nodes already committed in `communications.rs`: `aero_sovereign_gnss` vs `comm_gnss_sovereign` (same 15 Dec 2016 Galileo date, same sovereignty argument, near-verbatim), `aero_fully_reusable_heavy_lift` vs `comm_full_reuse_heavy_lift`, `aero_smallsat_constellation` vs `comm_smallsat_bus`, `aero_proliferated_leo_architecture` vs `comm_leo_broadband_constellation` + `comm_laser_intersatellite_link`. `aero_counterspace_electronic_warfare` (floor 2004, prereq `aero_electronic_attack` at 2009) would have panicked `tree_is_well_formed`; `ccs` gates directly on `aero_electronic_attack` instead. The recommendation buried in the `dsp` justification — make a new missile-warning tech a *prereq of* `aero_theater_missile_defense` — is **rejected**: Patriot in Desert Storm is load-bearing for both Gulf tests, and `pick_focus` researches cheapest-first, so the delay would land twice. `milstar` 0.85→3.50, `galileo` 0.07→0.65, `aehf` 1.05→2.40: one convention, programme-unit cost, which is what a government actually appropriates and is what `b2` at 1.10 already uses. `worldview3` recast as an imagery *contract* at $30M and a 60-month term, which is what its own justification describes ("renting a queue position on somebody else's satellite") and which makes it the most interesting entry here — a middle power buying overhead reconnaissance it could never build. Cut: `titan_iv`, `delta_ii`, `falcon9`, `starship`, `starshield`, `ess`.

### 1.9 The six NEW technologies — the complete diff to `tech/aerospace.rs`

Paste into the Information and Networked blocks. **Registry 253 → 259; update `the_registry_is_the_size_this_source_says_it_is` in the same commit and say so in the commit message**, or the next person reads the canary as catching a stale binary (iron rule 6).

```rust
// --- Information block ---

// Composite and reactive armour and the sight that fights at night. Chobham on
// Challenger 1 from 1983, Kontakt-5 on Soviet tanks from 1988, and the M1A1HA
// with the 120mm gun and the depleted-uranium package that deployed to Saudi
// Arabia in the autumn of 1990. The M1's thermal imaging sight is why Desert
// Storm was fought in the dark, and why the range at which a tank can be killed
// stopped being the range at which it can be seen.
tech(
    "aero_third_generation_armour", "Third-Generation Armour", Aerospace, Information,
    &[], 108.0, 1990,
    &[MilitaryEfficiency(0.03), MilitaryStrength(1.5)],
),
// Force projection logistics: the flying boom (KC-135, 1957), the outsize
// airlifter (C-5A, 1970) and the roll-on/roll-off sealift hull. REFORGER and the
// build-up to Desert Shield in the autumn of 1990 are the same capability twice.
// This is what separates an army that exists from an army that can arrive.
tech(
    "aero_strategic_mobility", "Strategic Mobility", Aerospace, Information,
    &[], 110.0, 1990,
    &[MilitaryEfficiency(0.03), MilitaryStrength(1.0)],
),
// Pressurised-water naval reactors. USS Nautilus got under way on nuclear power
// on 17 January 1955; USS Enterprise commissioned in 1961 with eight reactors;
// Nimitz in 1975 with two. Endurance without refuelling is what makes a fleet
// global rather than regional.
tech(
    "aero_naval_nuclear_propulsion", "Naval Nuclear Propulsion", Aerospace, Information,
    &[], 107.0, 1990,
    &[MilitaryEfficiency(0.02), MilitaryStrength(1.5)],
),
// ATACMS Block I fired its first rounds in Desert Storm on 18 January 1991: a
// corps commander with a 165 km precision ballistic weapon that answers to him
// rather than to an air tasking order.
tech(
    "aero_tactical_ballistic_missile", "Army Tactical Ballistic Missile", Aerospace, Information,
    &["aero_precision_munitions"], 109.0, 1991,
    &[MilitaryEfficiency(0.02), MilitaryStrength(1.0)],
),

// --- Networked block ---

// PAC-3 CRI: first hit-to-kill intercept in 1999, fielded 2001. A round that
// kills by striking rather than by fragmenting is the only thing that reliably
// destroys a ballistic warhead, and Patriot's actual Desert Storm record is the
// argument that made it necessary.
tech(
    "aero_hit_to_kill_interceptor", "Hit-to-Kill Interceptor", Aerospace, Networked,
    &["aero_theater_missile_defense", "aero_aesa_radar"], 158.0, 2001,
    &[MilitaryEfficiency(0.03), MilitaryStrength(1.0)],
),
// GMLRS fired its first rounds in Iraq in September 2005: a 70 km rocket with a
// GPS/INS package that turned divisional artillery from an area weapon into a
// precision one, and made the launcher itself the thing worth hunting.
tech(
    "aero_precision_rocket_artillery", "Precision Rocket Artillery", Aerospace, Networked,
    &["aero_gps_guided_munition"], 146.0, 2005,
    &[MilitaryEfficiency(0.03), MilitaryStrength(0.5)],
),
```

Every one carries **zero `Productivity`**. I summed the file: Aerospace Productivity is at exactly 0.00200 across 22 effects, which is precisely the domain budget with zero slack. Any Productivity on a new node has to be taken off an existing one, and none of these six has an obvious civil twin. This also makes the change growth-neutral through the productivity channel by construction, leaving only the small upward `absorption_rate` effect of six more learnable technologies — which pushes `china_growth_miracle` *away* from the floor it is nearest.

Prereq-year and era-window checks, all six: 1990/1990/1990 with `&[]`; 1991 on a 1990 prereq; 2001 on 2000 and 1991 prereqs; 2005 on a 1999 prereq. Cost bands: 107/108/109/110 inside Information (40–110); 146/158 inside Networked (90–200). `tree_is_well_formed` passes.

---

## 2. THE WAR INTEGRATION

### 2.1 Why the arsenal is a multiplier and never an addend

`war.rs:47`:
```rust
((n.gdp * n.mil_spend_gdp) / n.mil_strength.max(1.0) / 3.0).clamp(0.0, 1.2)
```
`mil_strength` is the **denominator** of `capital_intensity`, and `capital_intensity` feeds `quality()`, `deployable_fraction()` and the magazine refill. Inflating `mil_strength` collapses quality; deflating it inflates quality. And the 1.2 clamp breaks the symmetry: the USA sits at 1.096, just under it, and Iraq at 0.154. Any *uniform* shrink of `mil_strength` pushes the USA into the clamp where it stops gaining while Iraq gains proportionally forever — i.e. shrinking everyone's strength compresses the quality gap in favour of the poor. That is backwards. `sustained = ... + strength_of(n)` and `mil_strength = strength_of(n)` are both unshippable for this reason before you even reach the scale problem.

This also disposes of the audit's D11.2 double-counting objection: `war.rs` sustains from the full budget while `arsenal.rs` takes 20% of the same money, so an *additive* arsenal would buy strength twice. A multiplier that is normalised to 1.0 at the reference procurement share buys it once.

### 2.2 `arsenal.rs` — the new `condition`

```rust
/// How much of its original value a platform still carries at this age.
///
/// A straight decline from the day it is delivered to `RESIDUAL` at the end of
/// its service life. The old shape — flat to `service_months`, then a ramp over
/// the same span again — meant nothing in this deck decayed at all inside the
/// century the game runs: an F-15E bought in 1990 is at 0.51 in 2060 and a B-2
/// is still at 1.0 in 2035, so lapsing procurement for thirty years cost
/// approximately nothing and the module's own thesis was unimplemented.
/// `service_months` now means "months to residual", which is how every number in
/// DECK was actually chosen: 480 for an M1A1 is a full forty-year life, not a
/// warranty period.
pub fn condition(def: &EquipmentDef, age: f64) -> f64 {
    let life = def.service_months.max(1) as f64;
    (1.0 - (1.0 - RESIDUAL) * (age / life)).max(RESIDUAL)
}
```

### 2.3 `arsenal.rs` — the two new functions

```rust
/// Months of the reference procurement line that a fully-equipped force is worth.
///
/// Derived, not chosen: a steady stream of L $bn/month into kit of service life S
/// settles at a book value of L·S·(1+RESIDUAL)/2 = 0.675·L·S. At S = 296 months,
/// the middling service life in DECK, that is 200 months of the line.
///
/// It is also the one number here that can be checked against the world. At the
/// transcribed 1990 figures this makes a fully-equipped United States worth
/// $1,096bn of equipment against a BEA gross stock of national-defence equipment
/// near $1.1tn; the Soviet Union $640bn; Iraq $40bn against roughly 5,500 tanks
/// and 700 aircraft; Kuwait $3.0bn against 250 tanks and 35 aircraft. Nobody
/// typed any of those — they fall out of budget × share × horizon.
pub const EQUIP_HORIZON: f64 = 200.0;

/// What a force structure is worth with no procurement programme behind it at
/// all: conscripts, rifles, bases, and whatever the last government left. Not
/// zero — an unequipped army is the Iraqi army of 2003, not an empty field.
pub const BARE_FORCE: f64 = 0.55;

/// The most a nation can be over-equipped relative to its current budget. A
/// government that has just halved defence spending coasts on inherited
/// equipment; it does not become twice as strong.
pub const ADEQUACY_CAP: f64 = 1.30;

/// The replacement value still standing in the books, $bn.
///
/// Deliberately in the same units as the procurement budget. The arsenal enters
/// the war model as a ratio of money to money, so the deck's `quality` column —
/// which is authored judgement, and which five separate reviews found wrong by
/// between 5x and 10^4x in different classes — cannot set the scale of anything.
/// Only `unit_cost`, which is transcribed money under iron rule 4, can.
pub fn book_value(n: &Nation) -> f64 {
    n.arsenal
        .held
        .iter()
        .filter_map(|h| DECK.get(h.kit as usize).map(|d| h.units * d.unit_cost * condition(d, h.age)))
        .sum()
}

/// What share of the force structure its budget describes a nation has actually
/// equipped. The only quantity `war.rs` reads out of this module.
///
/// Money against money: what is in the books, against what a force funded at the
/// reference share for the reference horizon would hold. This is what makes lead
/// time bite, and it survives every balance error in the deck's `quality` column
/// because it never reads that column.
///
/// The upper bound is BIBLE §5's answer written as a number. Doubling the
/// military budget in the month war arrives multiplies sustained strength by
/// 1.414 through the square root and by 0.775 through the coverage it has just
/// halved: a net 1.096. The other ninety per cent of the money is on the order
/// book, due in 2010. You cannot out-produce a war. You can only have failed to
/// out-produce the twenty years before it.
pub fn equipped_fraction(n: &Nation) -> f64 {
    let want = (n.gdp * n.mil_spend_gdp * PROCUREMENT_SHARE / 12.0 * EQUIP_HORIZON).max(1e-9);
    let adequacy = (book_value(n) / want).min(ADEQUACY_CAP);
    BARE_FORCE + (1.0 - BARE_FORCE) * adequacy.max(0.0)
}
```

Note the denominator uses the **constant** `PROCUREMENT_SHARE`, not the nation's own share. That is the load-bearing choice: it is what makes cutting the procurement share a slow, unfixable knife, while cutting the *budget* is correctly forgiving (an inherited arsenal against a smaller army is a larger army, for a while).

`strength_of` stays, with its lying doc comment replaced and a role filter:

```rust
/// Notional line-combat value of what a nation holds. Force structure only —
/// `Lift`, `Deterrent` and `Magazine` are in the books and out of this sum,
/// because a tanker wins no engagement, a missile boat fights no conventional
/// war, and rounds are already modelled as `Nation.munitions`.
///
/// NOT on the same scale as `mil_strength` and never added to it. The war model
/// reads `equipped_fraction`, which is a money ratio; this exists for the
/// interface and for whatever composition term a future pass calibrates.
pub fn strength_of(n: &Nation) -> f64 { /* filter(|d| d.role == Role::Line) */ }
```

### 2.4 The exact edit

**`spheres-sim/src/war.rs:361-362`**, inside `tick()`. Current:
```rust
        let sustained = (budget * 0.30).max(0.0).sqrt() * 8.0 * crate::tech::military_multiplier(n)
            + crate::tech::military_floor(n);
```
Becomes:
```rust
        // What the budget could sustain, times how much of it procurement
        // actually bought. Multiplicative and bounded rather than additive:
        // `mil_strength` is the denominator of `capital_intensity`, so anything
        // that moves its scale moves quality, lift and magazine refill with it.
        let sustained = (budget * 0.30).max(0.0).sqrt()
            * 8.0
            * crate::tech::military_multiplier(n)
            * crate::arsenal::equipped_fraction(n)
            + crate::tech::military_floor(n);
```
`n` is `&mut Nation` at line 354 and reborrows to `&Nation` for the call. `arsenal::tick` already precedes `war::tick` in `SYSTEMS` (`lib.rs:526`), so a delivery shows up the same month it lands.

**The floor stays outside the multiplier.** `military_floor` is knowledge, not hardware, and it is what stops a bankrupt nuclear state reaching zero. It is 0.0 for every nation in 1990 anyway.

**`spheres-sim/src/statecraft.rs:133` takes the same term in the same commit.** It holds a second, independent copy of the sustained formula for patronised clients; if `war.rs` gains the arsenal term and this does not, the two loops pull a client toward different equilibria every month:
```rust
let sustained = ((client_gdp * client_mil_share + annual) * 0.30).sqrt() * 8.0
    * crate::arsenal::equipped_fraction(w.nation(f.client));
```
Leave its missing `military_multiplier` alone — that is a pre-existing divergence and fixing it in this commit confounds the reading.

### 2.5 What the player experiences

| State | adequacy | `equipped_fraction` |
|---|---|---|
| Steady at the reference share | 1.000 | **1.000** |
| Procurement share cut 0.20 → 0.05, ten years on | 0.662 | **0.848** |
| …twenty years on | 0.476 | **0.764** |
| …held forever (equilibrium) | 0.250 | **0.663** |
| Bought nothing at all since 1990, measured in 2010 | 0.300 | **0.685** |
| Military budget doubled *this month* | 0.500 | **0.775** |
| Military budget quartered this month | ≥1.30 | **1.135** |

- **Surging `mil_spend_gdp` at the moment of crisis:** `sqrt(2) × 0.775 = 1.096`. Nine per cent. Quadrupling gives `2.0 × 0.6625 = 1.325`. The money is not wasted — it is on the order book, due in four to twenty years.
- **Cutting the budget is never an exploit.** Halving gives at most `0.707 × 1.135 = 0.803`; quartering `0.5 × 1.135 = 0.568`. Always a loss.
- **A share cut is the decade-long knife.** Restoring the share in month 120 changes nothing for `lead_months` (24–300), and then rebuilds on a 200-month time constant. Unfixable in the month it matters, which is the requirement.

---

## 3. THE SEEDING

### 3.1 Where it runs, and why nowhere else

**`spheres-sim/src/data/mod.rs:568`** — replace `arsenal: Default::default(),` with:
```rust
arsenal: crate::arsenal::inheritance(self),
```
Body in `arsenal.rs`, taking `&NationRecord`.

- It is the **only** constructor from transcribed data. A post-load pass in `init::world_1990` misses every caller that goes through `data::load_world` directly — the CLI, `spheres-web`, any future mod loader — and those worlds open with empty arsenals and no error.
- It has **no `&mut WorldState` and therefore no RNG**. This is not incidental: if the vintage spread were drawn from `w.rng`, every downstream draw in the game shifts and all ~15 emergent-history tests re-roll. Seeding must consume zero RNG draws, and putting it here makes that a type-level guarantee rather than a discipline.
- It runs before the first `state_hash` is taken, so `the_1990_start_is_pinned` sees the seeded world rather than an empty one that fills in in February.

Do **not** put it in `arsenal::tick` behind a first-tick flag: that makes the t=0 hash blind to it, breaks `save_load_roundtrip_continuity` for a world saved before its first tick, and misses successor states.

### 3.2 The algorithm

```rust
pub fn inheritance(r: &crate::data::NationRecord) -> Arsenal {
    let line = (r.economy.gdp_bn * r.economy.mil_spend_gdp * PROCUREMENT_SHARE / 12.0).max(0.0);
    if line <= 0.0 || r.military.strength <= 0.0 { return Arsenal::default(); }
    let want = line * EQUIP_HORIZON;                       // $bn of equipment

    // capital_intensity, recomputed from the record — no WorldState exists yet.
    let k = ((r.economy.gdp_bn * r.economy.mil_spend_gdp)
             / r.military.strength.max(1.0) / 3.0).clamp(0.0, 1.2);
    let sat = |x: f64| 1.0 - crate::exact::exp(-x);        // exact::exp, NEVER f64::exp
    ...
}
```

**1. Class shares — one doctrine vector, no per-nation table.**
```rust
let cmd   = matches!(r.system, EconomySystem::Command) as u8 as f64;
let coast = if LANDLOCKED.binary_search(&r.id.code()).is_ok() { 0.0 } else { 1.0 };
let nuke  = r.military.nuclear as u8 as f64;
let budget_abs = r.economy.gdp_bn * r.economy.mil_spend_gdp;   // $bn/yr

let w = [
    1.10,                                        // Infantry — everybody has one
    1.30 * sat(k / 0.12) * (1.0 + 0.45 * cmd),   // Armour   — command economies overbuild it
    1.75 * sat(k / 0.32),                        // Air      — needs sustained money
    1.30 * sat(k / 0.50) * coast,                // Naval    — money and a coast
    0.35 * sat(k / 0.40) * (1.0 + nuke),         // Missile
    0.35 * ((budget_abs - 100.0) / 250.0).clamp(0.0, 1.0),  // Space
];  // normalise to sum 1.0
```
Space keys on **absolute budget**, not `k`, because a reconnaissance constellation is an absolute-scale capability. At the transcribed 1990 figures that threshold selects **exactly the USA ($328.9bn) and the USSR ($192.0bn)** and nobody else — which is the truth about military space in 1990, and it is emergent, not typed. China crosses $100bn on its own growth curve mid-game and joins them.

**2. Vintage — the mechanism, not decoration.**
```
mean_age = 132.0 + 240.0 * crate::exact::exp(-k / 0.40)          // months
tranches = [(0.30, -84.0), (0.45, 0.0), (0.25, +108.0)]          // weight, offset
```
USA → 12.3y mean (5.3 / 12.3 / 21.3y); France → 19.4y; Iraq → 24.6y; Chad → 30.6y.

**3. Generation.** `modernity = sat(k / 0.35)`, stepped down 0.25 per tranche: `g = modernity − 0.25·i`. Thresholds:

| Class | tiers | `g` cut points |
|---|---|---|
| Infantry | `inf_light`, `inf_mech` | 0.55 |
| Armour | `arm_gen2`, `arm_gen3` | 0.50 |
| Air | `air_gen2`, `air_gen3`, `air_gen4` | 0.30, 0.60 |
| Naval | `nav_patrol`, `nav_escort`, `nav_blue` | 0.30, 0.72 |
| Missile | `msl_sam`, `msl_brm`, (`msl_deterrent`) | 0.35, 0.70 |
| Space | `spc_recon` | — |

**The deterrent tier is gated on `r.military.nuclear`, and a nuclear state's newest tranche is always `msl_deterrent`.** Without the gate, Japan seeds a Strategic Deterrent Force and France seeds none.

**4. Units — solved from money, so the identity is exact.**
```rust
let age = (mean_age + offset).clamp(0.0, 2.0 * def.service_months as f64);
let units = want * share * weight / (def.unit_cost * condition(def, age));
```
Therefore `book_value == want` to floating-point, therefore `adequacy == 1.0`, therefore **`equipped_fraction == 1.0` for every one of the 137 nations at t=0, exactly, with no per-nation tuning.** This is why the Gulf tests survive: `sustained` at tick one is unchanged to ~1e-16.

Note this dissolves the seeding designer's own §1(c) blocker. Their 630× spread in required strength-per-dollar across the roster exists only if you seed to `strength_of(n) == n.mil_strength`. In money space there is no such constraint, and no global deck is needed to fit anything.

**5. Merge.** Same-kit rows across tranches merge with a **units-weighted mean age** (see §4 D9 — `Holding.age` becomes `f64` and there is one row per kit). Because `condition` is now *linear* in age, a units-weighted mean age gives exactly the right condition, so the merge is lossless up to the point where part of a fleet reaches the residual floor.

**Landlocked gate.** All 21 landlocked start nations verified present in the roster:
`Afghanistan, Austria, Bhutan, Bolivia, Botswana, CentralAfricanRepublic, Chad, Czechoslovakia, Hungary, Laos, Lesotho, Luxembourg, Malawi, Mongolia, Nepal, Paraguay, Swaziland, Switzerland, Uganda, Zambia, Zimbabwe`
This is the one new transcribed fact the design needs — geography, not invention, the same kind of thing `NationRow::neighbours` already carries. Its right home is a tenth field on `NationRow`, but that is a 160-row edit to a `const fn row(...)`; a sorted `const LANDLOCKED: &[&str]` in `arsenal.rs` is the cheap version. **Ethiopia is coastal in 1990** (Assab and Massawa; it had a navy until 1991) and is correctly absent — revisit if Eritrean secession is ever modelled.

**Do not seed `orders`.** What a nation had on order in January 1990 is not in the data, the pipeline refills itself within one lead time, and the order book is where save bloat lives.

### 3.3 What comes out

```
USA   strength 100.0  k=1.10  modernity 0.96  mean fleet age 12.3y  book $1,096bn
  Infantry inf_mech 63mo | inf_mech 147mo | inf_light 255mo
  Armour   arm_gen3 63mo | arm_gen3 147mo | arm_gen2  255mo
  Air      air_gen4 63mo | air_gen4 147mo | air_gen3  255mo
  Naval    nav_blue 63mo | nav_escort 147mo | nav_escort 255mo
  Missile  msl_deterrent 63mo | msl_deterrent 147mo | msl_brm 255mo
  Space    spc_recon 63mo | spc_recon 147mo | spc_recon 255mo

Iraq  strength 26.0  k=0.15  modernity 0.36  mean fleet age 24.6y  book $40bn
  31% infantry, 38% armour, 19% air, 10% naval, 3% missile — a mass conscript
  army with a very large tank park and a mediocre air force. Nothing typed.

Chad  strength 1.3  k=0.01  → 88% light infantry at 23-40 years old, 7% gen-2
  armour, a trace of gen-2 aircraft, no navy. Already at the residual floor in 1990.

Costa Rica, Samoa — zero holdings. Both transcribe strength 0.0, and Costa Rica
abolished its army in 1949. Emergent, not special-cased.
```

World aggregate: Infantry 41.0%, Armour 25.9%, Air 18.3%, Naval 9.8%, Missile 4.4%, Space 0.6%.

**What a procurement lapse costs** — `book_value` as % of 1990 if a nation orders nothing:

| | 1995 | 2000 | 2010 | 2020 | 2030 |
|---|---|---|---|---|---|
| USA | 94 | 89 | **75** | 60 | **46** |
| USSR | 94 | 88 | 72 | 56 | 43 |
| France | 87 | 79 | 62 | 48 | 39 |
| Iraq | 76 | 67 | 52 | 41 | 36 |
| Chad | 41 | 38 | 36 | 35 | 35 |

Per class for the USA (1990/2000/2010/2020/2030): Space 100/77/44/35/35 — Infantry 100/93/69/41/35 — Missile 100/99/87/61/40 — Armour 100/100/96/81/55 — Air 100/100/99/92/73 — Naval 100/100/100/97/86.

**This is the answer to BIBLE §5.** Space dies in a decade, foot mobility in fifteen years, hulls last forty. A player who stops buying loses reconnaissance first and finds out in the one month it matters. Chad cannot lapse because Chad already has. **If every holding were seeded at `age: 0`, all six rows would read 100 until 2030 and the system would have no consequence inside the playable window. The seeded vintage spread is the whole mechanism.**

### 3.4 Successor states and conquest

`politics.rs:237` (Russia), `:286` (Ukraine), `:524` (the eleven republics), `:889` (Yugoslav successors) all build with `arsenal: Default::default()`. The largest arsenal on earth would evaporate in December 1991, and every in-flight order with it. Add:

```rust
/// A successor takes its share of the depots and every year already on them.
pub fn inherit(parent: &Arsenal, share: f64) -> Arsenal
```
Scale every `Holding::units` and `Order::units` by `share`; **copy `age` unchanged.** The share is already computed at each site (`strength * 0.65` for Russia, `* 0.15` for Ukraine, `r.army` for the republics, `m` for Yugoslavia), so no new figure is authored. Preserving age is the point: Russia in 1992 flies 1970s airframes, which is why its arsenal collapses through the decade even while its `mil_strength` number still looks respectable. `TechState::inherit` already exists for exactly this reason — "the institutes and the engineers do not vanish with the flag" — and hulls in the water should not be treated as if they do.

`war.rs:876-919`: annexation (`:887-890`) sets `alive = false` and leaves the arsenal on the dead row, serialized forever and invisible to `tick`'s `alive` filter. Transfer `held` to the winner at 0.5 (war damage and capture rate), drop `orders` (an annexed state's contracts die with its treasury), then clear. Subjugation (`:903-908`) does `l.mil_strength *= 0.4` and leaves the arsenal wholly intact — scale `held` units by 0.4 to match.

---

## 4. DEFECTS IN `arsenal.rs`, BY SEVERITY

**D1 — `pick()` produces a global JDAM monoculture, and it is structurally correct to do so.**
`arsenal.rs:295-299`. `units = budget / unit_cost` (`:266`) and `strength = units × quality` (`:211`), so strength is exactly linear in money at rate `quality/unit_cost` with no diminishing return, no capacity limit and no class requirement. The argmax of that ratio is the optimal allocation at every budget, and `pick()` finds it. JDAM is 16,667/$bn against the B-2's 5.45 — 3,058×. Twenty-seven of thirty-two existing entries are decorative; no nation ever buys one, in any century. `Class` is documented at `:28-29` as existing "so an arsenal can be lopsided in a way that matters" and **nothing reads it**. Every arsenal in the world is 100% Missile.
**Fix — replace `pick()` with `plan()`:**
```rust
/// What procurement buys this month, and where it puts the money.
///
/// Not an argmax. `units = budget / unit_cost` makes strength exactly linear in
/// money at rate quality/unit_cost, so a single global argmax is the *correct*
/// answer to the model as specified and it is a monoculture. The fix is not a
/// better heuristic, it is a budget that is divided before it is spent.
pub fn plan(n: &Nation) -> Vec<(u16, f64)>   // (kit index, $bn this month)
```
1. Class shares from the **same `profile()` the seeder uses** — one function, two callers, so the 1990 seed and the 2005 buy are the same claim rather than two authored tables.
2. Within each class, order available kits by `quality/unit_cost` descending; fill each up to `max_share × line` until the class's money is spent.
3. `preference`, if set and available, takes its own `max_share` first out of its own class's money.
4. Class residue spills in `Class` declaration order; final residue banks (D8).
Deterministic: DECK index breaks every tie, no `HashMap`, no RNG.
This is also the fix for "eight of twenty air-defence entries are strictly dominated", "`cuas`/`link16`/`c4isr` become dead data", and "the seeded mix is erased within a decade".

**D2 — No starting inventory. Iron rule 4 breach, and the module's own thesis is unimplemented.**
`data/mod.rs:568` plus `politics.rs:237, 286, 524, 889`. Every nation begins 1990 with zero equipment; the doc at `arsenal.rs:13-14` says the arsenal is "mostly an inheritance from governments that are gone" and there is no inheritance. In January 1991 the United States would fight Desert Storm with nothing. 1990 order-of-battle is transcribable public data and it has been invented as zero. **Fix: §3.** This is a hard precondition for §2's `war.rs` edit — landing them in the wrong order turns a calibration test red for a reason nobody will find quickly.

**D3 — Nothing in the DECK is orderable by anybody, ever, at t=0.**
`TechState::new` (`tech/mod.rs:636`) sets `known: vec![]` and every entry is tech-gated, so `available()` returns `[]` for all 137 nations and `pick()` returns `None` until a nation researches its first `aero_` tech. Procurement is inert in both directions. **Fix: `tech: Option<&'static str>`, `None` always available, and the 18-entry legacy tier (§1.2).** Without it, adequacy decays as `exp(-t/200)` for everybody — 0.549 by 2000, 0.301 by 2010 — and the sim quietly loses a third of its militaries in the decade nobody is watching.

**D4 — DECK indices are serialized to disk. Silent corruption, and a panic.**
`Holding.kit`/`Order.kit`/`Arsenal.preference` are `u16` indices into a `pub const DECK`, all serialized (`world.rs:233`). Insert one entry and every existing save reinterprets its JDAMs as GBIs — no error, no version bump. Worse, `strength_of` (`:210`) and `tick` (`:265`) index unchecked, so a save written against a longer DECK **panics on load-and-tick**, not at `load()` where it could be reported. The codebase already knows this is wrong and says so twice: `TechState.known` is "written to disk as stable ids — see `known_serde`" (`tech/mod.rs:548-549`), and `Command::EnactStratagem` "carries the stratagem's stable id, **never an index into the deck**" (`lib.rs:51-52`).
**Fix: `kit_serde`, a straight copy of `known_serde`, on both `kit` fields; `preference` becomes `Option<String>` on the wire. Harden `DECK[..]` to `DECK.get(..)` everywhere.** This must land in the same commit as the deck expansion. **It also dissolves four separate verifier findings at a stroke** — with id-based serialization, the "append at the end of DECK or every save silently re-points" constraint disappears entirely and the deck can stay grouped by class.

**D5 — The module is dead code and the player decision it exists for does not exist.**
`strength_of`, `condition`, `available`, `registry`, `index_of`, `Class::parse` have zero callers outside `arsenal.rs`. `war.rs:361` still uses the flat technology floor, so the doc at `:203-204` ("Read by `war.rs`") is **false as of this commit**. `Arsenal::preference` has no `Command` variant, so it can never be non-`None` — iron rule 2 is not so much violated as unimplemented.
**Fix: two commands.**
```rust
/// Direct the procurement line at one thing, or hand the choice back to the
/// staff with `None`. A standing order, not a purchase: every month's money goes
/// to it until it is changed, and changing it recalls nothing already on the water.
SetProcurement { nation: NationId, kit: Option<String> },
/// What share of the military budget buys new equipment rather than paying
/// people and running what is already owned. The decision this module exists
/// for, and the one whose cost does not arrive for fifteen years.
SetProcurementShare { nation: NationId, share: f64 },
```
Prices in `command_price`: `SetProcurement` **8.0, REFUSABLE** — slightly above `SetResearchFocus` (6.0) and far below a national programme (30.0); redirecting an industrial base is an ordinary act of government and the expensive part is the fifteen years, which the model charges in months rather than in standing. `SetProcurementShare` **`swing(before, after, 200.0)`, REFUSABLE** — cutting equipment spending to pay soldiers is the popular half of a defence budget and the model should let a government do it cheaply and regret it later.
Resolve the kit by `arsenal::index_of(id).ok_or_else(...)` and verify membership in `available(n)`, erroring with the gating technology's name — the same shape `SetResearchFocus` uses.

**D6 — The lapse penalty the module exists to deliver does not exist.**
`condition()` (`:193-199`) is flat until `service_months`, then declines to 0.35 over the same span again. Service lives are 240–600 months, so nothing bought in 1990 falls below 0.35 before 2060 — that needs age ≥ 2× service, i.e. 80 years for most air platforms. Stop procurement in 1995, spend nothing for thirty years, and in 2025 you still hold 100% of what you had. **Fix: the straight-line curve in §2.2.** The frontier-relative obsolescence term (`0.75^generations`) that one review proposed is correct in spirit and **deferred**: it couples a per-nation function to a global `tech::frontier_known` read, and `military_multiplier` already scores knowing new things, so it would double-charge. Revisit after the calibration is measured.

**D7 — Consumables and platforms share one additive pool, and consumables are never consumed. `war.rs` already models munitions.**
`war.rs` has `n.munitions` (0..1), `BURN_BY_RUNG` (0.090/mo at rung 6), `MAGAZINE_REBUILD × capital_intensity`, and `MAX_SUSTAINABLE_DRY` forcing a dry belligerent down to rung 5 — BIBLE §6's second stock, and the whole reason a poor state's arsenal is a one-shot weapon. `arsenal.rs` silently double-models it as an infinite non-depleting asset: 30,000 bombs are worth more than the bomber that drops them, in perpetuity, having been dropped. **Fix: `Role::Magazine`, excluded from `strength_of`. Phase 4 wires Magazine deliveries into `munitions` capacity and refill rate.** Until then a Magazine holding is book value and nothing else, which is honest.

**D8 — Procurement money leaks silently whenever `pick()` returns `None`.**
`:239, 264-273`. Budget is computed unconditionally and then, if `choice` is `None` — every nation in the world through 1990 and beyond — never mentioned again. No bank, no headline, no record. `economy.rs:284` has already charged `mil_spend_gdp` to the fiscal balance, so it is not a double-spend against the treasury; it is years of funding charged to the economy that produce nothing, with no player-visible signal. **Fix: `pub banked: f64` on `Arsenal`, capped at 24 months of the line so it cannot be hoarded for a century, spent by `plan()` when something becomes orderable.**

**D9 — `held` grows without bound; `retain` at `:276` is dead code; `held` rows explode.**
Arrivals merge only while `h.age < 12`, so a new `Holding` is created every twelve months per kit — ~70 per kit over a 1990–2060 run. `units` only ever increases (`:258, :259`), so `retain(|h| h.units > 1e-6)` can never fire and the comment at `:275` ("Retire what has nothing left to give") describes something the module does not do. Arsenal value is monotonically non-decreasing for the entire game.
**Fix, three parts:** (a) `Holding.age` becomes `f64` and there is **one row per kit**, merged with a units-weighted mean age — legitimate because `condition` is now linear in age, so the mean gives exactly the right value until part of a fleet clamps at residual; (b) retire past twice service life — `if h.age > 2.0 * def.service_months as f64 { h.units *= 0.98; }`, a ~22%/yr write-off, which makes `retain` live; (c) the order-merge predicate at `:268` (`o.due == def.lead_months`) is likewise dead, because `:250` decrements before it runs — with `plan()` returning several kits it becomes live and correct.

**D10 — `PROCUREMENT_SHARE = 0.20` is applied identically to all 137 nations and contradicts its own comment.**
`:170-175` says most of the world sits below 15% and that this is what makes their arsenals age out, then hands everyone 20%. **Fix: `Arsenal.share` with `#[serde(default = "default_share")]`, seeded from the record, and `tick` line 237 becomes `(n.gdp * n.mil_spend_gdp * n.arsenal.share / 12.0).max(0.0)`.** `PROCUREMENT_SHARE` remains the *reference* the coverage denominator measures against — a government that drops below it is paying its soldiers with its grandchildren's air force and will not find out for fifteen years.

**D11 — `+inf` GDP produces an infinite order and an infinite arsenal.**
`.max(0.0)` at `:238` handles negative GDP (which this tree has demonstrably produced — commit `ff77690`) and NaN (because `f64::max` returns the non-NaN operand), but not `+inf`. **Fix: `.clamp(0.0, 1e6)`.** One line.

**D12 — Nothing asserts that a kit's tech id resolves.**
`available()` uses `is_some_and` (`:221`), so an unresolved `tech` yields `false`: the entry is silently never orderable, with no panic, no failed test and no warning. `tree_is_well_formed` catches unresolved *prereqs* loudly; nothing does the equivalent for `DECK.tech`. With six new techs landing beside 129 deck entries this is a real hazard. **Fix: `every_kit_names_a_technology_that_exists` — one loop, one assert.**

### What I checked and found clean — do not re-audit

Determinism: `tick` (`:233`) collects ids in `w.nations` vector order; `available` iterates `DECK` by index; `held`/`orders` are insertion-ordered `Vec`s; `tech::index_of` and `knows_index` are binary searches over a sorted `OnceLock` table. No `HashMap` iteration, no RNG, no wall clock. `pick()`'s comparator `.then(b.cmp(a))` is a strict total order and never returns `Equal` for distinct indices, so `max_by`'s last-wins rule cannot bite — that subtlety is correct and the same discipline must be kept in `plan()`. Order/delivery timing is right: the decrement at `:250` runs before the placement at `:270`, so an order placed in month M with `due = L` arrives in M+L, not M+L−1. `mil_spend_gdp == 0` gives `budget = 0` → `units = 0` → no order, no NaN. `Nation.arsenal` carries `#[serde(default)]` so pre-procurement saves load. All 32 existing `EquipmentDef.tech` ids resolve against `tech/aerospace.rs`; I verified every one.

---

## 5. THE UI

### 5.0 A bug that must be fixed first, or the screen is broken on arrival

`render()` ends with `if (selected) openNation(selected, true);`. `selected` (`ui/index.html:703`) is only ever set by `openNation`. **`openTechTree` never sets it, so the tech-tree sheet goes stale the moment you advance time** — it silently shows last month's costs. For the arsenal that is fatal: the whole point is watching `due` count down, and a player advancing five years with the sheet open would see the order book frozen.

```js
let sheetMode = null;      // "nation" | "tree" | "arsenal"
let selected  = null;      // nation id, meaningful only when sheetMode === "nation"

// end of render():
if      (sheetMode === "nation" && selected) openNation(selected, true);
else if (sheetMode === "tree")    openTechTree(treeDomain, true);
else if (sheetMode === "arsenal") openArsenal(arsSel, true);

// closeSheet() (line 2415):
function closeSheet() { sheetMode = null; selected = null; /* ...as now... */ }
```
Both reopen paths take a `keepScroll` argument and pass the sheet's `scrollTop` through `openSheetEl(true, scroll)` (`:2406`), which already supports it. **Land this on its own; it fixes the tech tree regardless.**

### 5.1 The one idea

Every other strategy game's procurement screen is a shopping list sorted by price. **This one is sorted by the calendar.** The eye should land on a year before it lands on a number, and the year should be shocking. Three devices carry the whole thesis:

1. **The wall.** A hatched vertical band over the next N years of the horizon chart, N = the shortest lead time of anything currently orderable, labelled in words: `NOTHING ORDERED TODAY ARRIVES BEFORE JULY 1994`. Everything left of it is inheritance and is not a decision.
2. **The lapse line.** A dotted projection of book value assuming zero further orders. It always falls. The distance between it and the stacked area is exactly what previous governments bought you.
3. **The divergence pip.** Select a kit in the catalogue and a ghost line joins the chart — *identical to the lapse line for its entire lead time*, then lifting. A circle at the departure point reads `first effect · Mar 2005 · fifteen years`. The player watches the thing they just chose do nothing for fifteen years. That animation is the module's argument and it is cheaper to build than any of the tables.

### 5.2 Headline numbers — and the one thing the screen must not claim

Do **not** print `strength_of` beside `mil_strength`. They are not on the same scale and never will be; a screen reading "arsenal 1,204 · fielded strength 100" is lying about which number decides a war. The headline is:

> **Equipped 100% · $1,096bn of equipment in the books · $5.48bn a month into procurement**

`equipped_fraction` as a percentage is the number the war model actually reads, it is bounded 55%–113.5%, and it is directly actionable. `book_value` in $bn is checkable against the world. `strength_of` appears only inside the arsenal sheet, per class, labelled "line combat value", never compared to `mil_strength`.

### 5.3 Left column: `#arsCard`

Placed immediately after `#resCard` in `renderLeft()` (`:996`, between `${researchHtml(m)}` and `${stratagemsHtml(m)}`) — research and procurement are the two decade-clock cards and adjacency teaches that. Keyboard **`A`**, added to the `keydown` switch (`:824-850`) and to `#keys`.

Six lines, one control:
- `Equipped` — the percentage, in `--amber`, with a bar.
- `.arsstack` — a 6px flex bar of class shares of book value, six colours, `title` tooltips.
- **`.arsnext`** — the loudest element on the card, bigger than the equipped figure: a date in `--cyan` at 20px, `MAR 2005`, with `F-22 Raptor · 26 units` small beside it and `ordered Mar 1994 · eleven years still to run` smaller still. The card's job is to keep a year the player cannot influence permanently in the corner of their eye.
- `On order` — count and $bn committed.
- `Past service life` — share of book value, in `.down`.
- **`.arsstanding`** — the standing order, clickable.

When there are no orders, do not print "0 orders". Print the thesis: *"Nothing on order. Anything decided today arrives 1994 at the earliest."* When `at_war`, a red rule and one line: *"Deliveries during this war: none."*

### 5.4 The sheet: `#arsenal`

Opened by `window.openArsenal(kit)`, structured exactly like `window.openTechTree` (`:1033`): fetch a dedicated route, write `$("#sheet").innerHTML`, call `openSheetEl(false, 0)`, wire. Five blocks in this order — the chart first, because the tables are evidence for it:

**(a) The budget line, stated once, plainly.** `$328.9bn × 20% = $5.48bn a month into procurement`, with the note: *"The rest pays people and runs what is already owned. Raising military spending raises this line — and nothing it buys arrives for at least four years."* The `20%` is a slider, wired to `SetProcurementShare`, with the second note: *"Cutting this is the cheapest thing on this screen and the most expensive thing you will ever do. It takes about seventeen years to show up."*

**(b) The horizon chart.** See §5.5.

**(c) On order** — rows sorted by `due_t` ascending. `MAR 2005` / `15 years away` on the left; kit, units, $bn, `ordered Mar 1994` in the middle; a `.run` progress bar that has visibly barely started; `+$5.3bn to the books` on the right. **No cancel button** — `arsenal.rs` has no cancellation and a control asserting a feature that does not exist is worse than the omission.

**(d) Held** — grouped by class, `.past` rows (age > service_months) get a red left rule and read `past service life since 2026 · worth 71% of new`. The value here is showing that a class ages out *together*, because it was bought together.

**(e) What you could order** — sorted by **when it arrives**, never by value for money. Three states matching the tech tree's vocabulary: `open` (cyan border, real month and year), `locked` (`research 11 years + build 14 years = 2015`, linking into the tech tree at the right domain — the deepest expression of the thesis and worth the sim helper it needs), `impossible` (bare year). Button verb: **`MAKE THIS THE STANDING ORDER`**, deliberately echoing `MAKE THIS THE PROJECT` (`:1112`). Under it on first open: *"Procurement is a standing direction, not a purchase. Every month's money goes to this until you change it — and changing it does not recall what is already on the water."*

### 5.5 `horizonSvg(h)`

**Do not reuse `chart()` (`:1947`).** It is hard-bound to the recorded past — `N = HIST.t.length`, `labelAt(i)` reads `HIST.labels[i]`, `wireCharts` indexes hover positions into that same month array. A future axis cannot be expressed in it. Write a separate function with a local hover closure.

`W=980, H=310, PADL 58, PADR 132, PADT 16, PADB 28`. X in fractional years. **Y is always base-zero** — an arsenal is a stock, and a cropped baseline lies about how much of it is inheritance. Annual resolution, 41 points. Paint order matters:

1. Year grid every 5 years.
2. Optional `past` stack, 1990→today at 55% opacity.
3. **The stack** — six filled `<path>` areas, held + already-ordered book value by class, `fill-opacity:.42`.
4. **The wall** — `<rect>` from `X(now)` to `X(wall_year)`, 45° hatch `<pattern>` in `#233042`, label in `--dim` at 11px. If narrower than ~70px, label above the axis.
5. **Now line** — 2px `--amber`, `TODAY`.
6. **The lapse line** — 1.6px `--dim`, `stroke-dasharray="4 3"`, right-hand label `if you order nothing more`, annotated where it crosses half of today's value: `half of what you have now · 2014`.
7. **The standing line** — 2px `--cyan`, solid, only when a preference is set.
8. **The ghost** — 2px in the selected kit's class colour, `stroke-dasharray="2 4"`. **Identical to the lapse line until `diverges` by construction** — do not smooth it, do not let a spline lift it early; the flatness is the message. `<circle r="4">` at the divergence year.
9. **Delivery pips** — 6px triangles at `PADT+2` per outstanding order, class-coloured, with `<title>`. A cluster in 2005 next to an empty 1990s is the order book seen from above.
10. Right-hand series labels, pushed apart by the same 12px rule `chart()` already uses.

Hover writes into `#ars-ro`: `2007 · in the books $940bn · on order $210bn · if you lapse $780bn · equipped 86%`.

### 5.6 Server contract

**On `state_json` (`main.rs:500-563`)**, beside `"research"` and `"stratagems"`, `null` when `w.player` is `None`:
```jsonc
"arsenal": {
  "equipped": 1.00, "book_bn": 1096.4, "procurement_bn_month": 5.48,
  "procurement_share": 0.20,
  "classes": [ { "class": "Air", "book_bn": 200.7, "share": 0.183 } ],  // all six, always
  "orders": 4, "committed_bn": 21.9,
  "next": { "kit": "f22", "name": "F-22 Raptor", "units": 26.4,
            "due": "Mar 2005", "due_t": 183, "years": 11.0, "placed": "Mar 1994" },
  "ageing": { "past_service_share": 0.34,
              "next_out": { "name": "Third-Generation Tank Regiment", "year": 2026 } },
  "standing": { "kit": "f35a", "name": "F-35A Lightning II", "delivers": "Jan 2004" },
  "wall_year": 1994, "wall_label": "Nothing ordered today arrives before Jul 1994"
}
```

**New route `GET /api/arsenal[?project=<kit id>]`**, beside `/api/tech` (`:915`), because the catalogue is 129 rows and the state payload is fetched on every advance. It carries `budget`, `classes`, `held`, `orders`, `orders_total`, `catalogue`, `preference` and `horizon{ t0, span, wall_*, labels, stack[6], total, lapse, standing, past, counterfactual }`. Generalise `nation_param` (`:221`) to a named-key lookup rather than writing a second percent-decoder.

**Four rules the payload must obey**, all of them lessons already in this codebase:
- **Every date is a preformatted string from the server.** `month_name(month, year)` exists at `main.rs:150`. The browser must never do calendar arithmetic — it got growth wrong three ways doing arithmetic it should not have.
- **Every projection is computed sim-side.** `horizon.*` is the only place the counterfactual can be right, because only `arsenal.rs` knows `condition()`'s shape and `RESIDUAL`.
- **Kits travel as `&'static str` ids, never `u16`.** `EnactStratagem`'s comment already sets the precedent.
- **All six classes always present**, zeros included, so the legend and stack do not reorder between frames.

`parse_command` (`:697`): `"procure" => Command::SetProcurement { nation: me, kit: v.get("kit").and_then(|x| x.as_str()).filter(|s| !s.is_empty()).map(String::from) }` and `"procure_share" => Command::SetProcurementShare { nation: me, share: num()? }`. Sent immediately via `POST /api/command` then `adopt(r, false)`, same as `setFocus` — it is a decision and the sim charges for it the moment it lands.

**One sim helper the locked rows need:**
```rust
/// Months of research before this nation could hold `idx`, following the whole
/// unheld prerequisite closure at the current domain rate. `None` when the domain
/// has no rate, or a prerequisite is not yet possible for anyone.
pub fn tech::months_to(w: &WorldState, me: NationId, idx: u16) -> Option<u32>
```
Approximate — it assumes the player keeps redirecting that domain — and the row must say so: `about 11 years at your present research budget`. Return `None` rather than guessing when the closure crosses domains; the row then reads `not being researched` and links to the tree.

**Optional, highest value per line of code:** `Order.placed_by: Option<String>` with `#[serde(default)]`, filled from `government.rs` at order time. It makes the module's own doc comment literal the day a 2005 delivery is labelled with a party that lost power in 1996. Nothing else here produces that feeling as cheaply.

**Rival arsenals** (`openNation`): held book value by class only, **never the order book**. What a rival fields is observable; what it has committed to for 2005 is not. One payload field, and it is the strategic tension of the whole module.

---

## 6. ORDER OF WORK, AND WHAT BREAKS AT EACH STEP

Export a per-worktree `CARGO_TARGET_DIR` before any of this (iron rule 6). If a result surprises you in either direction, confirm the binary is yours before believing it.

**Step 0 — `sheetMode` fix (§5.0) and `TechState::ensure_shape` clear-not-resize.** Both land alone. No Rust behaviour change, no hash movement, `ensure_shape`'s new branch never fires on a current save. Expect: nothing breaks. This is the only thing worth landing before the deck is written.

**Step 1 — `arsenal.rs` structural repair, no deck change, no `war.rs` change.** `Role`, `tech: Option`, `max_share`, `kit_serde`, `DECK.get`, `Holding.age: f64` with one row per kit, retirement at 2×service, `banked`, `Arsenal.share`, the new `condition`, `book_value`, `equipped_fraction`, `strength_of`'s role filter and its corrected doc, `plan()` replacing `pick()`, `profile()`, `inherit()`. `war.rs:361` **untouched**.
**Expect to break:** `golden_hash_of_a_known_run` (`arsenal::tick` is already in `SYSTEMS` at `lib.rs:526`, so `plan()` and the new merge write different `orders`/`held` from the first month any nation knows an `aero_` tech — re-pin, cause known and legitimate). `the_1990_start_is_pinned` should **not** move yet — nothing is seeded. If it does, you have a serde default that is not defaulting. Everything else green.

**Step 2 — the six technologies.** Registry canary 253 → 259 in the same commit, with the reason in the commit message.
**Expect to break:** `the_registry_is_the_size_this_source_says_it_is` (deliberate, one line). `golden_hash_of_a_known_run` re-pins. **Watch `china_growth_miracle`** — median 11.16× against a floor of 11.0×, a 1.5% margin, and this is the test that decides whether the commit ships. Run all ten seeds before and after. Two forces act: +6 registry entries lift `absorption_rate` slightly (up, good) and two mid-band Networked entries slightly slow other 2000s Aerospace research (neutral for growth — Aerospace Productivity is 0.0020 in total). Also check `a_poor_nation_still_picks_up_what_everyone_has` (floor 5, currently 6..11 across twelve seeds with two seeds at 6). **The Gulf tests should be unmoved at zero order**, because every new Information entry is priced above the existing Information ceiling of 106 and `pick_focus` is cheapest-first — the 1990-99 research sequence is preserved exactly. If either Gulf test moves here, that guarantee is broken and something is mispriced; check the costs before touching anything else.

**Step 3 — the deck.** All 129 entries, `aip_ssk` deleted, renames applied in place. Add `every_kit_names_a_technology_that_exists` and `deck_ids_are_unique`.
**Expect to break:** `golden_hash_of_a_known_run` (re-pin). Nothing else — `strength_of` still has no war-model caller and `equipped_fraction` is still not read. **This is the last commit at which a deck error is cheap to find, so measure here:** run a century and print, per nation, `book_value`, `equipped_fraction` and the class mix. Nothing asserts yet; you are reading instruments.

**Step 4 — seeding.** `data/mod.rs:568`, the four `politics.rs` sites, the two `war.rs:876-919` sites, `LANDLOCKED`.
**Expect to break:** `the_1990_start_is_pinned` (`0xf28a574a2efdd179` → new) and `golden_hash_of_a_known_run`. Both re-pin **once**, with the paragraph the file demands and both checks done: `git diff -- spheres-sim/data/` is **empty** — this design changes no nation file, every input is already transcribed — and not one figure moved. The movement is the struct gaining content, the same story as the `munitions`/`theatres` and `TechState::priority` entries already in that comment. Note the trap explicitly in that comment: **seeded `units` are solved from `unit_cost` and `service_months`, so every future DECK tuning pass moves the 1990 hash**, and a plausible-looking data edit is exactly what the pin exists to catch. Add here: `every_nation_opens_equipped` (`(equipped_fraction(n) - 1.0).abs() < 1e-9` for every alive nation), `seeding_draws_no_randomness` (two worlds, different seeds, byte-identical arsenals), `nobody_opens_with_a_brand_new_arsenal` (no holding at age 0; roster mean age > 120 months), `only_two_powers_hold_military_space_in_1990`, `a_landlocked_state_has_no_navy`, `costa_rica_has_no_army`.

**Step 5 — `war.rs:361` and `statecraft.rs:133`.** The three-line edit. **This is the commit that can move a war test, and it must contain nothing else, or a red has three candidate causes.**
**Expect:**
- `determinism_same_seed_same_world` — **green, must not move.** No RNG, no `HashMap`, no wall clock; `equipped_fraction` is `+ - * /` only.
- `save_load_roundtrip_continuity` — **green.** `Arsenal.share` and `banked` carry `#[serde(default)]`; holdings round-trip; f64 is exact through serde_json.
- Both hashes — **move, re-pin.** `equipped_fraction` is 1.0 to ~1e-16 at t=0, not bit-exactly, and the sim is chaotic; tick one is identical to twelve digits and the timeline resamples.
- **`gulf_war_emerges` (≥5/10) — the one to watch.** `mil_strength` at t=0 is unchanged to 1e-16 and the invasion fires around months 7–30, by which point adequacy has drifted <2% and *symmetrically*, because everyone starts at 1.0 and adequacy is a ratio against each nation's own budget. The exposure is `dyads.rs:262`: `strength_ratio.powi(8)` below parity, with Iraq/Kuwait at ≈0.87, so a 3% **asymmetric** drift is a ~25% change in the monthly hazard. **If this moves, the cause is `dyads.rs`, not `war.rs` — check `strength_ratio` before touching anything here.**
- **`desert_storm_is_quick_when_they_stand_and_fight` (≥6/8 inside 36 months) — expect green with margin.** It calls `declare_war` directly and bypasses `dyads` entirely. Within 36 months adequacy drifts <5%, and the second-order effect is *favourable*: the USA at `capital_intensity` 1.096 sits just under the 1.2 clamp, so any small downward move in its `mil_strength` pushes it into the clamp and raises US quality, while Iraq at 0.154 moves proportionally — the ratio widens from 2.06 to 2.14 at f = 0.9. Only a *large* uniform shrink hurts (at f = 0.55 the ratio narrows to 1.91), which is exactly the case step 4 exists to prevent.
- `afghanistan_does_not_end` — green. Staged, 60 months; Pakistan's `capital_intensity` is 0.052, far from the clamp; all three assertions have visible margin against a <5% drift.
- `ussr_collapses_in_the_nineties` (≥6/10) — **watch.** Not a strength test, but the USSR's `mil_spend_gdp` of 0.12 is the highest of the majors, so its arsenal decisions move `militarisation` and `dyads` more than anyone's.
- `nuclear_taboo_holds`, `china_growth_miracle` — green, resampled.

**Step 6 — commands and the web surface.** `SetProcurement`, `SetProcurementShare`, `arsenal_card_json`, `#arsCard`. Shippable and visible on turn one.
**Expect:** hashes hold (commands are player-initiated; no AI issues them yet). `command_price` gains two arms — check nothing else in `lib.rs` matches exhaustively on `Command` without a wildcard.

**Step 7 — `/api/arsenal`, the sheet, `tech::months_to`, `horizonSvg`, the ghost.** Pure surface, no sim change, no hash movement. Ship 1–4 of the UI build order together (they already make lead time legible through dates and countdowns); the horizon chart is where it becomes the point; the counterfactual ghost is last, because it is the best part and everything else has to be right first.

**Deferred, deliberately, each its own commit later:**
- Magazine deliveries feeding `n.munitions` (D7 phase 2) — the correct home for nine Missile and two Infantry entries.
- `Role::Lift` feeding `deployable_fraction` — then transports stop needing to be lied about and `air_lift`/`nav_lift`/`c17a`/`lmsr_sealift` become worth buying for the right reason.
- Combat attrition into `book_value`. **Do not do this in step 5.** `structure_hits` (`war.rs:571`) cuts `mil_strength` by up to 50%/month; routing that into the arsenal closes a positive-feedback loop — lose strength, lose arsenal, lose more strength — that needs its own floor argued, and it would move `desert_storm` and `afghanistan_does_not_end` in the same commit as everything else.
- Frontier-relative obsolescence in `condition` (D6's second half).
- The tech-domain split, 8 → 11, after the equipment layer is calibrated — with `coastline` added to `NationRow` as its own prior commit under full roster hash discipline, and the `domain_weights` military bloc computed with the *identical* old Aerospace expression before subdivision, so the denominator and every civil share are unchanged to the digit.