//! Materials & Manufacturing.
//!
//! OWNER: the materials domain author. This file is yours alone. Never edit
//! `tech/mod.rs`, and never touch another domain file — eight authors are
//! working in parallel and the merge is a straight concatenation.
//!
//! SCOPE: metallurgy, polymers, ceramics, composites, coatings, nanomaterials,
//! semiconductors as a material and a process, machine tools, industrial
//! robotics, additive manufacturing, process control and quality method.
//!
//! Every id here must begin with `matl_`. Prerequisites may name other `matl_`
//! ids in this file, or any `core_` id from the foundation set in `tech/mod.rs`.
//! Nothing else — you cannot see the other domains and they cannot see you.
//!
//! Every entry is a real technology with a real history, and carries a comment
//! naming the first deployment its year floor is read off. Past the present day
//! the comment opens with its bucket: `ROADMAP` where the date is somebody
//! else's published, dated, funded commitment, and `SPECULATIVE` where it is
//! not. `SPECULATIVE` is permitted only in the Frontier era, whose own
//! definition already requires it.
//!
//! WHAT THIS DOMAIN IS. Manufacturing is the domain where knowing a thing and
//! being able to make it are different problems, and the second one is the
//! expensive one. So the tree here is mostly chains rather than options: you
//! cannot polish a wafer flat until you can print on it, cannot lay copper into
//! a trench until you can polish, cannot immerse the lens until the wiring
//! underneath it will carry the clock. The lithography spine is the longest
//! chain in the file because it was the longest chain in the period — every
//! step of it was gated on the step before, and a nation that dropped out at
//! any point did not get back in.
//!
//! WHAT THE NUMBERS MEAN. Productivity here sums to about 0.0020 — two tenths
//! of a percentage point on trend growth for the whole of manufacturing, which
//! sounds small until you remember it compounds for fifty years. The domain
//! also carries most of the tree's `InvestmentEfficiency`, because that is
//! literally what a machine tool is: a thing that makes a dollar of plant go
//! further.

use super::TechDef;
#[allow(unused_imports)]
use super::{tech, Domain, Effect, Era};

/// The Materials: alloys, composites, semiconductor substrates and coatings.
///
/// Data only — the scoring engine lives in `tech::mod`.
pub fn techs() -> Vec<TechDef> {
    use Domain::*;
    use Effect::*;
    use Era::*;
    vec![
        // -------------------------------------------------------------------
        // The 1990 shop floor. Nothing here is new in 1990 — these are the
        // capabilities a serious industrial economy already had, and the ones a
        // poor one has to buy before anything else in the file is reachable.
        // -------------------------------------------------------------------

        // Mercury i-line steppers at 365nm came into wide use around 1990 and,
        // with off-axis illumination, printed below 0.35 micron. This is the
        // root of the longest chain in the tree: every later node in the
        // lithography spine is gated on being able to print at all.
        tech(
            "matl_semiconductor_lithography", "Sub-Half-Micron Lithography", Materials, Information,
            &["core_cmos_submicron"], 60.0, 1990,
            &[
                Productivity(0.00018),
                ResearchRate(0.03),
                CostReduction { domain: Materials, frac: 0.04 },
            ],
        ),
        // The foundation entry is the material; this is the industrial plant
        // that turns it into load-bearing structure — autoclaves, prepreg cold
        // stores, ultrasonic inspection, and the certification basis to fly a
        // part nobody can x-ray for cracks. In service on primary structure
        // through the 1980s, the AV-8B wing being the usual first citation.
        tech(
            "matl_composite_structures", "Composite Primary Structures", Materials, Information,
            &["core_carbon_composites"], 55.0, 1990,
            &[Productivity(0.00006), MilitaryEfficiency(0.02), EnergyEfficiency(0.01)],
        ),
        // The Unimate went to work at GM's Trenton plant in 1961; by 1990 the
        // six-axis AC-servo arm under a microprocessor controller was standard
        // equipment in every body shop worth the name. What it buys is not
        // cheaper labour so much as a repeatable weld.
        tech(
            "matl_industrial_robotics", "Industrial Robot Cells", Materials, Information,
            &[], 65.0, 1990,
            &[
                Productivity(0.00015),
                InvestmentEfficiency(0.03),
                CostReduction { domain: Materials, frac: 0.03 },
            ],
        ),
        // The Toyota Production System, and the discovery that it transfers:
        // NUMMI, from 1984, ran it with an American workforce in a plant GM had
        // closed, and MIT's IMVP published it as a method rather than a local
        // habit in 1990. The cheapest general-purpose technology in the file —
        // it costs almost nothing and it reorganises everything.
        tech(
            "matl_lean_production", "Lean Production System", Materials, Information,
            &[], 50.0, 1990,
            &[
                Productivity(0.00020),
                InvestmentEfficiency(0.04),
                CostReduction { domain: Materials, frac: 0.04 },
                Environment(0.01),
            ],
        ),
        // PWA 1480 entered commercial service in September 1982 in the JT9D-7R4
        // on the 767 — the first structural use of a single crystal anywhere.
        // Removing the grain boundaries is what let turbine inlet temperatures
        // keep climbing, which is most of the efficiency story in jet engines
        // and in the gas turbines that burn the same gas on the ground.
        tech(
            "matl_single_crystal_superalloy", "Single-Crystal Superalloy Blades", Materials, Information,
            &[], 70.0, 1990,
            &[MilitaryEfficiency(0.03), EnergyEfficiency(0.02), Productivity(0.00002)],
        ),
        // Nd2Fe14B, found independently by Sagawa at Sumitomo Special Metals and
        // Croat at General Motors in 1982-83; GM's Magnequench plant opened in
        // 1986. The strongest magnet anyone had ever sold, and therefore the
        // reason a motor could get small — disk drives first, traction motors
        // and direct-drive wind turbines much later.
        tech(
            "matl_rare_earth_magnet", "Sintered Rare-Earth Magnets", Materials, Information,
            &[], 55.0, 1990,
            &[Productivity(0.00003), EnergyEfficiency(0.02), ResourceYield(0.01)],
        ),
        // 3D Systems' SLA-1, the first commercial machine, shipped in 1987-88.
        // As a production method it was nearly worthless for twenty years; as a
        // way of finding out on Thursday whether Monday's design was wrong, it
        // took a cycle of engineering time from weeks to days.
        tech(
            "matl_stereolithography", "Stereolithography", Materials, Information,
            &[], 45.0, 1990,
            &[
                Productivity(0.00002),
                ResearchRate(0.02),
                CostReduction { domain: Materials, frac: 0.02 },
            ],
        ),
        // Five-axis simultaneous contouring — one setup for a turbine blade or
        // an impeller instead of six, and surfaces no three-axis machine can
        // reach. It rides the same closed-loop servo and controller stack the
        // robot arm does, which is why it is downstream of it here. The exact
        // year it stopped being exotic is not pinned; the floor is set at 1992.
        tech(
            "matl_five_axis_cnc", "Five-Axis CNC Machining", Materials, Information,
            &["matl_industrial_robotics"], 75.0, 1992,
            &[Productivity(0.00008), InvestmentEfficiency(0.03), MilitaryEfficiency(0.02)],
        ),

        // -------------------------------------------------------------------
        // The lithography spine begins. Each of these was gated on the last in
        // the literal sense that the process would not run without it.
        // -------------------------------------------------------------------

        // IBM developed chemical-mechanical planarization in the mid-1980s and
        // put it into CMOS production with the 4Mb DRAM; the first process data
        // went public at VMIC in 1991. Polishing the wafer flat after every
        // layer is what made stacking layers possible at all — without it the
        // topography defeats the stepper's depth of focus by the fourth mask.
        tech(
            "matl_chemical_mechanical_planarization", "Chemical-Mechanical Planarization", Materials, Information,
            &["matl_semiconductor_lithography"], 80.0, 1991,
            &[
                Productivity(0.00005),
                ResearchRate(0.01),
                CostReduction { domain: Materials, frac: 0.03 },
            ],
        ),
        // Electron-beam PVD yttria-stabilised zirconia on turbine airfoils, in
        // service on the PW2000 family. UNVERIFIED: I could not pin the first
        // production year, so the floor is set conservatively at 1994 rather
        // than invented. The coating is worth a couple of hundred degrees of
        // inlet temperature, on top of what the single crystal already bought.
        tech(
            "matl_thermal_barrier_coating", "Thermal-Barrier Coatings", Materials, Information,
            &["matl_single_crystal_superalloy"], 85.0, 1994,
            &[EnergyEfficiency(0.02), MilitaryEfficiency(0.02), Productivity(0.00002)],
        ),
        // EOS shipped the EOSINT M 250 in 1995, the first commercial machine to
        // sinter metal powder directly. It made tooling, not parts — a mould
        // insert in days instead of months — which is the whole of its economic
        // effect for the next twenty years.
        tech(
            "matl_direct_metal_laser_sintering", "Direct Metal Laser Sintering", Materials, Information,
            &["matl_stereolithography"], 80.0, 1995,
            &[
                Productivity(0.00002),
                InvestmentEfficiency(0.01),
                CostReduction { domain: Materials, frac: 0.03 },
            ],
        ),
        // Invented at TWI in 1991; first commercial use in November 1996, on
        // hollow aluminium panels at Sapa in Finspång. A solid-state joint in
        // alloys that cannot be fusion-welded without cracking, which is why it
        // ended up under launch vehicle tanks and rail car bodies.
        tech(
            "matl_friction_stir_welding", "Friction Stir Welding", Materials, Information,
            &["matl_five_axis_cnc"], 70.0, 1996,
            &[Productivity(0.00003), MilitaryEfficiency(0.01), ResourceYield(0.01)],
        ),
        // IBM announced CMOS 7S on 22 September 1997 and the copper-wired
        // PowerPC shipped in Apple's 400MHz Power Macintosh in March 1998 — a
        // third more clock out of the same transistors, because the wires
        // stopped being the limit. Damascene only works because CMP does: the
        // process is to fill the trench and polish the overburden away.
        tech(
            "matl_copper_damascene", "Copper Dual-Damascene Interconnect", Materials, Information,
            &["matl_chemical_mechanical_planarization", "core_duv_lithography"], 105.0, 1998,
            &[
                Productivity(0.00006),
                ResearchRate(0.02),
                CostReduction { domain: Materials, frac: 0.02 },
            ],
        ),

        // -------------------------------------------------------------------
        // 2000s. The process toolbox stops being about optics alone.
        // -------------------------------------------------------------------

        // Infineon released the first-generation CoolSiC Schottky barrier diode
        // in 2001 — the first commercial silicon-carbide power device. A wide
        // bandgap switches faster and hotter than silicon can, which is where
        // the last few points of efficiency in an inverter live, and eventually
        // where a traction inverter's range comes from.
        tech(
            "matl_silicon_carbide_power", "Silicon-Carbide Power Devices", Materials, Networked,
            &["matl_semiconductor_lithography"], 120.0, 2001,
            &[EnergyEfficiency(0.03), Productivity(0.00003)],
        ),
        // Micron began developing ALD high-κ films for DRAM capacitors in 2000
        // and they reached volume production around the 90nm node. UNVERIFIED:
        // the exact first-production year is not pinned, so the floor is 2004.
        // Depositing one atomic layer at a time is the only way to line a hole
        // with an aspect ratio of forty, which is what everything after this
        // needs — the gate stack, the vertical NAND string, the capacitor.
        tech(
            "matl_atomic_layer_deposition", "Atomic Layer Deposition", Materials, Networked,
            &["matl_semiconductor_lithography"], 135.0, 2004,
            &[Productivity(0.00004), CostReduction { domain: Materials, frac: 0.03 }],
        ),
        // Boeing completed the first full-scale one-piece 787 composite
        // fuselage barrel in January 2005, laid down by automated fibre
        // placement heads. Hand lay-up puts a ceiling on how much composite an
        // airframe can afford; AFP is what took it off.
        tech(
            "matl_automated_fibre_placement", "Automated Fibre Placement", Materials, Networked,
            &["matl_composite_structures", "matl_five_axis_cnc"], 155.0, 2005,
            &[Productivity(0.00003), InvestmentEfficiency(0.02), MilitaryEfficiency(0.01)],
        ),
        // Intel shipped 45nm Penryn on 12 November 2007 with a hafnium high-κ
        // dielectric and metal gates — the first real change to the CMOS
        // transistor's materials in forty years, and the thing that stopped
        // gate leakage from ending scaling outright.
        tech(
            "matl_high_k_metal_gate", "High-κ Metal Gate", Materials, Networked,
            &["matl_atomic_layer_deposition"], 185.0, 2007,
            &[Productivity(0.00005), ResearchRate(0.02), EnergyEfficiency(0.01)],
        ),
        // ASML's TWINSCAN XT:1700i was the first immersion scanner built for
        // volume production; it ramped at customers from late 2006 and was in
        // volume use through 2007. Putting water between the lens and the wafer
        // bought a numerical aperture above 1.0, and bought the industry the
        // decade it needed before EUV worked.
        tech(
            "matl_immersion_lithography", "193nm Immersion Lithography", Materials, Networked,
            &["matl_copper_damascene", "core_duv_lithography"], 195.0, 2007,
            &[
                Productivity(0.00010),
                ResearchRate(0.03),
                CostReduction { domain: Materials, frac: 0.04 },
            ],
        ),
        // Universal Robots sold a UR5 to Linatex in December 2008 and it was
        // installed alongside people rather than inside a cage. The cage was
        // always most of the cost of a robot — the integration, the floor space,
        // the fact that a cell cannot be moved once it is certified.
        tech(
            "matl_collaborative_robot", "Collaborative Robot", Materials, Networked,
            &["matl_industrial_robotics", "core_cmos_submicron"], 145.0, 2008,
            &[Productivity(0.00006), InvestmentEfficiency(0.03)],
        ),

        // -------------------------------------------------------------------
        // 2010s. Structure goes three-dimensional, in silicon and in airframes.
        // -------------------------------------------------------------------

        // The 787 entered service with ANA on 26 October 2011, roughly half its
        // airframe by weight in composite, including the fuselage. The weight
        // is the point, and the weight is fuel.
        tech(
            "matl_composite_airframe", "All-Composite Airframe", Materials, Platform,
            &["matl_automated_fibre_placement"], 250.0, 2011,
            &[Productivity(0.00003), EnergyEfficiency(0.02), MilitaryEfficiency(0.02)],
        ),
        // Intel's 22nm tri-gate went into volume production in 2011 and Ivy
        // Bridge went on sale in April 2012 — the first high-volume FinFET.
        // Standing the channel up on its edge is what let the gate keep control
        // of it below 25nm; planar transistors simply stop working there.
        tech(
            "matl_finfet_transistor", "FinFET Transistor", Materials, Platform,
            &["matl_immersion_lithography", "matl_high_k_metal_gate"], 300.0, 2012,
            &[
                Productivity(0.00010),
                ResearchRate(0.03),
                CostReduction { domain: Materials, frac: 0.03 },
            ],
        ),
        // Samsung announced mass production of 24-layer V-NAND on 6 August 2013.
        // When a flash cell cannot be made smaller, stack it: the cost curve
        // that had been driven by lithography for thirty years was handed over
        // to deposition and etch, which is why storage kept getting cheaper
        // after the transistor stopped.
        tech(
            "matl_vertical_nand", "Vertical NAND", Materials, Platform,
            &["matl_atomic_layer_deposition"], 275.0, 2013,
            &[Productivity(0.00006), ResearchRate(0.02)],
        ),
        // GE Aviation's Auburn plant began series production of the LEAP fuel
        // nozzle tip in 2015 — the first mass-manufacturing site for additive
        // flight hardware. Twenty brazed parts became one, five times more
        // durable, and the argument for printing production parts stopped being
        // hypothetical.
        tech(
            "matl_additive_production_parts", "Additive Production Parts", Materials, Platform,
            &["matl_direct_metal_laser_sintering"], 245.0, 2015,
            &[
                Productivity(0.00004),
                InvestmentEfficiency(0.02),
                CostReduction { domain: Materials, frac: 0.03 },
            ],
        ),
        // TSMC has had CoWoS silicon interposers in production since 2012, and
        // AMD's Fiji put memory stacks on an interposer in a shipping consumer
        // product in 2015. Once the die stopped shrinking on schedule, the way
        // to keep adding transistors was to stop building one die — which is a
        // packaging problem, and packaging is a materials problem. The
        // through-silicon via is a damascene copper fill in a hole a hundred
        // times deeper, lined by ALD, which is why it sits downstream of both.
        tech(
            "matl_advanced_packaging", "2.5D Advanced Packaging", Materials, Platform,
            &["matl_copper_damascene", "matl_atomic_layer_deposition"], 285.0, 2015,
            &[Productivity(0.00007), ResearchRate(0.02)],
        ),
        // The CFM LEAP's ceramic-matrix-composite turbine shroud flew
        // commercially on the A320neo from August 2016 — the first widely
        // deployed CMC product. A ceramic that does not shatter runs 2400°F
        // with far less cooling air than a nickel superalloy needs, and the
        // cooling air was always stolen from the thrust.
        tech(
            "matl_ceramic_matrix_composite", "Ceramic-Matrix Composites", Materials, Platform,
            &["matl_thermal_barrier_coating", "matl_composite_structures"], 265.0, 2016,
            &[EnergyEfficiency(0.02), MilitaryEfficiency(0.02), Productivity(0.00002)],
        ),
        // Cognex bought ViDi Systems in April 2017; ViDi was the first
        // ready-made deep-learning inspection package a factory could buy rather
        // than build. Rule-based machine vision could only find the defects
        // somebody had already described; this finds the ones nobody has.
        tech(
            "matl_machine_vision_inspection", "Deep-Learning Visual Inspection", Materials, Platform,
            &["matl_industrial_robotics", "core_gpu_deep_learning"], 235.0, 2017,
            &[
                Productivity(0.00006),
                ResourceYield(0.01),
                Environment(0.02),
                CostReduction { domain: Materials, frac: 0.02 },
            ],
        ),
        // Samsung started 7LPP wafer production with EUV in October 2018 and
        // TSMC's N7+ was the first EUV process in high-volume customer
        // production in Q2 2019. Thirteen nanometre light from a tin plasma,
        // in vacuum, off mirrors, after twenty years and tens of billions —
        // and afterwards there were three companies left who could print at the
        // leading edge and one who could sell the machine.
        tech(
            "matl_euv_lithography", "Extreme-Ultraviolet Lithography", Materials, Platform,
            &["matl_finfet_transistor", "matl_immersion_lithography"], 355.0, 2019,
            &[
                Productivity(0.00012),
                ResearchRate(0.04),
                CostReduction { domain: Materials, frac: 0.04 },
            ],
        ),

        // -------------------------------------------------------------------
        // 2020s.
        // -------------------------------------------------------------------

        // Tesla's IDRA Giga Press cast the Model Y rear underbody as a single
        // piece; cars carrying it were delivered from December 2020. Seventy
        // stampings and their welds became one casting, and it needed a new
        // aluminium alloy that would not need heat treatment — the die is the
        // product design, which is a different way to run a car company.
        tech(
            "matl_gigacasting", "Single-Piece Structural Casting", Materials, Intelligent,
            &["matl_lean_production", "matl_five_axis_cnc"], 330.0, 2020,
            &[Productivity(0.00003), InvestmentEfficiency(0.02), ResourceYield(0.01)],
        ),
        // Samsung began mass production of its 3nm MBCFET gate-all-around
        // process in June 2022, the first GAA in production anywhere. Wrapping
        // the gate the whole way round the channel is the last obvious move in
        // electrostatics; after this the gains come from stacking and wiring.
        tech(
            "matl_gate_all_around", "Gate-All-Around Transistor", Materials, Intelligent,
            &["matl_euv_lithography"], 480.0, 2022,
            &[Productivity(0.00007), ResearchRate(0.03)],
        ),
        // ASML shipped the first EXE:5000 to Intel in December 2023 as a
        // development platform, and the first commercial EXE:5200B went into an
        // Intel fab and began printing 14A silicon in 2026. A 0.55 numerical
        // aperture halves the pitch a single exposure can reach; it also costs
        // roughly twice what a low-NA machine does, which is the real filter on
        // who stays at the leading edge.
        tech(
            "matl_high_na_euv", "High-NA EUV Lithography", Materials, Intelligent,
            &["matl_gate_all_around"], 570.0, 2026,
            &[
                Productivity(0.00007),
                ResearchRate(0.03),
                CostReduction { domain: Materials, frac: 0.03 },
            ],
        ),

        // HISTORY. Deployed 2025-H2; transcribed 2026-09. Power delivered from
        // the back of the wafer instead of through the same stack as the
        // signals. Intel's PowerVia on the 18A process went into high-volume
        // manufacturing with Panther Lake at Fab 52 in Arizona in the second
        // half of 2025, the first backside power delivery in volume production
        // anywhere. It is a separate move from the gate two entries above: that
        // one is electrostatics, this is wiring, and past a point the wiring is
        // what is in the way.
        tech(
            "matl_backside_power_delivery", "Backside Power Delivery", Materials, Intelligent,
            &["matl_gate_all_around"], 540.0, 2025,
            &[Productivity(0.00005), EnergyEfficiency(0.02), ResearchRate(0.02)],
        ),
        // HISTORY. Deployed 2025; transcribed 2026-09. A humanoid robot doing
        // paid production work outside a fixture. BMW Group ran Figure 02 in the
        // body shop at Plant Spartanburg for ten months, loading sheet metal
        // across more than thirty thousand X3s, and has said Leipzig follows in
        // 2026. Cite BMW and nobody else: every fleet number circulating for the
        // other makers traces back to a marketing post. What the general-purpose
        // form buys over the arm above it is that the cell no longer has to be
        // built around the robot.
        tech(
            "matl_humanoid_work_cell", "Humanoid Robot in Industrial Service", Materials, Intelligent,
            &["matl_collaborative_robot", "matl_machine_vision_inspection"], 560.0, 2025,
            &[Productivity(0.00006), InvestmentEfficiency(0.02)],
        ),
        // ROADMAP. Not ours: TSMC's April 2026 roadmap update puts A16 into
        // volume production in 2027 — a year later than the 2025 version of the
        // same roadmap said — with A14 in 2028 and A13 in 2029, and Intel has
        // pulled 14A high-volume manufacturing forward to 2028 with risk
        // production in 2027 on an EXE:5200B that is installed and accepted. The
        // floor is 2028 because that is the first year both of the two companies
        // that can do this claim to be selling it. Note what the same roadmaps
        // say about the entry above: A13 and A12 are planned WITHOUT High-NA, so
        // it is not the critical path it looks like.
        tech(
            "matl_angstrom_class_node", "Angstrom-Class Logic Node", Materials, Intelligent,
            &["matl_high_na_euv"], 595.0, 2028,
            &[
                Productivity(0.00006),
                ResearchRate(0.03),
                CostReduction { domain: Materials, frac: 0.03 },
            ],
        ),
        // HISTORY. Deployed 2021-08-18; transcribed 2026-09. Iron reduced by
        // hydrogen in a shaft furnace instead of by carbon monoxide in a blast
        // furnace, and melted in an electric arc furnace afterwards. SSAB
        // rolled the first steel from the HYBRIT pilot at Luleå in July 2021
        // and delivered it to the Volvo Group on 18 August 2021; Volvo put it
        // into a load carrier that October. The floor is 2021 because that is
        // when the metal existed and changed hands — not when anybody made it
        // in quantity, and the quantity is the honest part of this entry. The
        // pilot made tonnes. Stegra's Boden works, financed to €1.4bn in a
        // rescue round led by the Wallenbergs and still advertising the second
        // half of 2026 while its own timeline is under review, is designed for
        // 2.5 million tonnes a year against a world that makes about 1.8
        // billion. NOTE THERE IS NO ResourceYield HERE: direct reduction wants
        // BETTER ore than a blast furnace does, not worse, and claiming a
        // yield gain would be the easy lie. And note the prerequisites, which
        // are none. Nothing in this file gates this. What gates fossil-free
        // iron is cheap electricity and the electrolyser that turns it into
        // hydrogen, and both of those live in a domain this file may not name.
        tech(
            "matl_hydrogen_direct_reduction", "Hydrogen Direct-Reduced Iron", Materials, Intelligent,
            &[], 420.0, 2021,
            &[Environment(0.04), Productivity(0.00002)],
        ),
        // HISTORY. Deployed 2022-03; transcribed 2026-09. Copper pads on two
        // dies pressed into direct contact and annealed — no solder, no bump,
        // no gap. AMD's Milan-X, an EPYC 7003 with a 64MB cache die bonded
        // face-to-face onto each compute die on TSMC's SoIC, was generally
        // available in March 2022, and the Ryzen 7 5800X3D put the same stack
        // in a desktop socket on 20 April 2022. The pitch is the whole point:
        // a microbump array is tens of microns apart, a hybrid bond is single
        // digits, and the energy it takes to move a bit across the joint falls
        // with it — which is why by 6 December 2023 AMD's MI300X was stacking
        // eight compute dies onto four I/O dies and quoting seventeen
        // terabytes a second of vertical bandwidth. The interposer entry above
        // put dies beside each other; this puts them on top of each other, and
        // after the die stopped growing it is how an accelerator gets built at
        // all.
        tech(
            "matl_hybrid_bonding", "Copper Hybrid Bonding", Materials, Intelligent,
            &["matl_advanced_packaging"], 450.0, 2022,
            &[Productivity(0.00004), ResearchRate(0.03), EnergyEfficiency(0.01)],
        ),
        // HISTORY. Deployed 2023-03; transcribed 2026-09. Silicon holds about
        // ten times the lithium per gram that graphite does and swells by
        // something like three hundred per cent doing it, which is why it took
        // thirty years to ship: the answer is silicon grown inside a porous
        // carbon scaffold that absorbs the expansion. Honor's Magic5 Ultimate
        // went on sale in China in March 2023 with a silicon-carbon cell —
        // 5,450mAh in the case that had held 5,100mAh of graphite, which Honor
        // put at 12.8% more energy by volume — and the other Chinese flagships
        // followed within two years. Automotive scale came later and is still
        // small: Sila opened Moses Lake on 23 September 2025 at an initial two
        // to five gigawatt-hours, Group14 opened BAM-3 in Korea in March 2026
        // at about ten. The floor is 2023 and the first customer was a
        // telephone, which is the usual order — the cell that goes in a car is
        // the cell that was proved in a pocket. The ResourceYield is graphite:
        // a silicon-dominant anode needs much less of a mineral one country
        // refines almost all of.
        tech(
            "matl_silicon_carbon_anode", "Silicon-Carbon Anode", Materials, Intelligent,
            &["core_lithium_ion_cell"], 360.0, 2023,
            &[Productivity(0.00003), EnergyEfficiency(0.02), ResourceYield(0.01)],
        ),
        // HISTORY. Deployed 2024-10-21; transcribed 2026-09. Shred the pack,
        // sort what falls out of it, then leach the black mass and take the
        // cobalt, nickel and lithium back out of solution as salts a cell maker
        // can buy. Mercedes-Benz opened the Kuppenheim plant on 21 October 2024
        // — the first carmaker anywhere running mechanical and
        // hydrometallurgical recovery end to end in its own works, on Primobius
        // process technology, claiming better than 96% recovery. It handles
        // 2,500 tonnes a year, which is a rounding error against what the world
        // builds, and that is exactly why this belongs here and not in the
        // Frontier block: the chemistry is proved and the capacity is not. It
        // hangs off the inspection entry because the front end is a sorting
        // problem — a shredded pack is a mixture, and what a hydrometallurgical
        // plant is paid for is separating it.
        //
        // NOTE WHAT THIS DOES TO THE FRONTIER ENTRY BELOW. matl_closed_loop_
        // recovery was authored as an extrapolation of precisely this —
        // sortation, per-part material identification, hydrometallurgy that
        // recovers elements rather than mixtures — and half of that is now
        // transcribable fact. Its 2033 floor is still a guess, but it is now a
        // guess about scale rather than about whether the chemistry works.
        tech(
            "matl_battery_hydrometallurgy", "Hydrometallurgical Battery Recovery", Materials, Intelligent,
            &["core_lithium_ion_cell", "matl_machine_vision_inspection"], 340.0, 2024,
            &[ResourceYield(0.02), Environment(0.02), Productivity(0.00002)],
        ),
        // ROADMAP. Not ours: Absolics, SKC's American subsidiary, took a $75
        // million CHIPS award announced in December 2024 against $343 million
        // of its own capital for a 120,000 square-foot glass substrate plant in
        // Covington, Georgia, and NIST's project page for that award says first
        // customer deliveries in 2025 with production capacity in 2027. The
        // company shipped mass-production samples and went into qualification
        // with AMD and AWS through 2025 and 2026, and now says volume by the
        // end of 2026.
        //
        // THE DISCOUNT IS TWO YEARS, AND HERE IS WHY. The same company's
        // original plan was mass production in the first half of 2024. It has
        // missed that by more than two years while doing everything else it
        // said it would, which is the ordinary shape of a substrate
        // qualification rather than a sign of failure. The floor is 2028 — one
        // year past NIST's 2027 and two past the company's own end-2026 —
        // because a substrate is qualified by its customer and not by its
        // maker, and AMD has not yet said yes in public. What the glass buys is
        // flatness and stiffness across a package far larger than an organic
        // laminate will hold still, which is the limit every accelerator
        // package is now up against. It sits on the polish entry as well as the
        // packaging one because a through-glass via is drilled, copper-filled
        // and planarised back — the damascene loop, run on a different
        // substrate.
        tech(
            "matl_glass_core_substrate", "Glass-Core Package Substrate", Materials, Intelligent,
            &["matl_advanced_packaging", "matl_chemical_mechanical_planarization"], 500.0, 2028,
            &[
                Productivity(0.00003),
                ResearchRate(0.02),
                CostReduction { domain: Materials, frac: 0.02 },
            ],
        ),

        // -------------------------------------------------------------------
        // Frontier. SPECULATIVE — past the present day, extrapolated from
        // trends that were visible by the mid-2020s and not from any
        // announcement. Floors are guesses and are meant to read as guesses.
        // -------------------------------------------------------------------

        // SPECULATIVE. Extrapolated: automated sortation and per-part material
        // identification, plus depolymerisation and hydrometallurgical routes
        // that recover elements rather than mixtures, closing the loop on the
        // metals and polymers a rich economy already holds above ground. The
        // trend is real — the year is not.
        tech(
            "matl_closed_loop_recovery", "Closed-Loop Material Recovery", Materials, Frontier,
            &["matl_machine_vision_inspection", "matl_additive_production_parts"], 700.0, 2033,
            &[ResourceYield(0.04), Environment(0.05), Productivity(0.00002)],
        ),
        // SPECULATIVE. Extrapolated: the uncaged arm, the inspection that
        // needs no fixture and the printer that needs no tooling, run together
        // long enough that the plant no longer stops when the shift does. The
        // last productivity claim in this file and the least certain one.
        tech(
            "matl_lights_out_fabrication", "Lights-Out Fabrication", Materials, Frontier,
            &[
                "matl_collaborative_robot",
                "matl_machine_vision_inspection",
                "matl_additive_production_parts",
            ],
            950.0, 2037,
            &[
                Productivity(0.00010),
                InvestmentEfficiency(0.05),
                CostReduction { domain: Materials, frac: 0.05 },
            ],
        ),
        // SPECULATIVE — no such node is in production and no company has said
        // it will sell one. KNOWN: imec demonstrated electrically functional
        // monolithic CFET devices on 18 June 2024 at the VLSI Symposium — an
        // n-device stacked directly over a p-device, 18nm gate length, 60nm
        // gate pitch, 50nm between the two tiers, with the bottom source/drain
        // contact formed from the wafer backside, a change that took top-device
        // survival from 11% to 79%. ASSUMED: that a device at 79% survival on a
        // research line becomes a node somebody sells. imec's 2026 roadmap puts
        // CFET at A7 and dates the nodes either side of it — A10 around 2030,
        // A5 in 2035 or 2036, A3 in 2038 — with A7 itself reported at 2034.
        // FALSIFIABLE, AND NOTE THE SLIP FIRST: a version of that same roadmap
        // presented as recently as 2024 ran to 2036 and got two generations
        // further, as far as A2. A roadmap that loses two generations in two
        // years is a forecast and not a schedule, which is why the floor here
        // is 2035 and not 2034. And if stacking nanosheets and moving the power
        // to the back keep delivering without it — the two entries above this
        // block are the evidence that they might — CFET stays a conference
        // paper and this entry simply stays unbuilt.
        tech(
            "matl_stacked_cfet", "Complementary FET", Materials, Frontier,
            &["matl_gate_all_around", "matl_high_na_euv"], 780.0, 2035,
            &[
                Productivity(0.00004),
                ResearchRate(0.03),
                CostReduction { domain: Materials, frac: 0.04 },
            ],
        ),
    ]
}
