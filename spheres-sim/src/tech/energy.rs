//! Energy.
//!
//! OWNER: the energy domain author. This file is yours alone. Never edit
//! `tech/mod.rs`, and never touch another domain file — eight authors are
//! working in parallel and the merge is a straight concatenation.
//!
//! SCOPE: oil and gas exploration, extraction and refining, coal, nuclear
//! fission and fusion, hydro, wind, solar, geothermal, grids and transmission,
//! grid-scale and domestic storage, end-use efficiency.
//!
//! Every id here begins with `ener_`, the short code the cross-domain list
//! publishes for this domain. Prerequisites name other `ener_` ids in this
//! file, or a `core_` id from the foundation set in `tech/mod.rs`. Nothing
//! else: the foundation anchors are the only ids that are guaranteed to exist
//! while the eight files are still being written separately, and
//! `tree_is_well_formed` panics on a prerequisite it cannot resolve.
//!
//! Every entry is a real technology with a real history, and carries a comment
//! naming the first deployment its year floor is read off. Anything past the
//! present day is speculative and must say so in the comment.
//!
//! Two things in here are read straight through by `economy.rs` and are the
//! reason this domain is weighted the way it is. `OilYield` is recovery, not
//! geology: it is the fraction of a field that comes out of the ground, and it
//! is worked into producing wells over years rather than granted. It is what
//! turned the United States from a declining producer into the largest one
//! without a single new basin being found. `EnergyEfficiency` is the other
//! side of the same ledger — how much a barrel has to do — and it is what
//! decides whether an importer is wrecked by a price shock or merely annoyed
//! by one. The rest of the tree is generation and grid: cheaper electricity
//! reads through to productivity slowly and to the environment quickly.
//!
//! The shape is four chains that only meet at the end. Drilling runs from
//! seeing the rock (3-D seismic) to steering through it to breaking it, and
//! the shale revolution is the marriage of the last two rather than either
//! alone. Generation runs from the combined-cycle turbine through the reactor
//! fleet. The renewables chain runs from a screen-printed cell and a
//! pitch-controlled blade to plants that undercut fuel. Storage and
//! transmission run from a cell in a camcorder to the thing that makes the
//! first three dispatchable. Enhanced geothermal, at the end, is the drilling
//! chain arriving in the generation one — which is exactly how it happened.

use super::TechDef;
#[allow(unused_imports)]
use super::{tech, Domain, Effect, Era};

pub fn techs() -> Vec<TechDef> {
    use Domain::*;
    use Effect::*;
    use Era::*;
    vec![
        // -------------------------------------------------------------------
        // Seeing the rock, and steering through it
        // -------------------------------------------------------------------

        // 3-D seismic. Not one deployment but a displacement: a hundred-odd
        // surveys had been shot by the late 1980s, and through 1990-96 the
        // major companies went from 3-D as a special case to 3-D over the
        // majority of their offshore acreage, two to three hundred surveys a
        // year. It cut finding costs and reopened fields written off as
        // depleted, which is why it sits at the root of everything else here.
        tech(
            "ener_seismic_3d_imaging", "Three-Dimensional Seismic Imaging", Energy, Information,
            &[], 55.0, 1990,
            &[OilYield(0.03), ResourceYield(0.01), Productivity(0.00004)],
        ),
        // Horizontal drilling. Elf's Rospo Mare wells in the early 1980s proved
        // it offshore; the Austin Chalk campaign of 1986-90 proved it paid.
        // One wellbore in contact with hundreds of metres of pay instead of
        // tens is the single largest change in what a reservoir will give up.
        tech(
            "ener_horizontal_drilling", "Horizontal Drilling", Energy, Information,
            &[], 60.0, 1990,
            &[OilYield(0.04), Productivity(0.00006)],
        ),
        // Rotary steerable systems: Baker Hughes INTEQ and Agip fielded the
        // first-generation AutoTrak rotary closed-loop system in 1997, with
        // Schlumberger's PowerDrive alongside it. Steering while the whole
        // string keeps rotating is what made a long lateral routine rather
        // than an achievement, and it needed a downhole computer to do it.
        tech(
            "ener_rotary_steerable_drilling", "Rotary Steerable Drilling", Energy, Information,
            &["ener_horizontal_drilling", "core_cmos_submicron"], 95.0, 1997,
            &[OilYield(0.02), Productivity(0.00003)],
        ),
        // Slickwater fracturing. Mitchell Energy's Nick Steinsberger pumped the
        // first slickwater treatment in the Barnett in May 1997 and the company
        // made it standard practice in September 1998 — water and sand with a
        // friction reducer instead of gel, at a fraction of the cost. Proved in
        // vertical wells; the marriage with the lateral came later, which is
        // why this hangs off the seismic and not the drill.
        tech(
            "ener_slickwater_fracturing", "Slickwater Hydraulic Fracturing", Energy, Information,
            &["ener_seismic_3d_imaging"], 90.0, 1998,
            &[OilYield(0.03), Productivity(0.00003)],
        ),
        // Deepwater floating production: Shell's Auger tension-leg platform
        // began producing in April 1994 in 2,860 ft of water, the first TLP to
        // open the deepwater Gulf of Mexico. Everything past the shelf edge
        // dates from it.
        tech(
            "ener_deepwater_subsea", "Deepwater Floating Production", Energy, Information,
            &["ener_seismic_3d_imaging"], 95.0, 1994,
            &[OilYield(0.03), Productivity(0.00003)],
        ),

        // -------------------------------------------------------------------
        // Burning gas better, and splitting atoms more safely
        // -------------------------------------------------------------------

        // F-class combined cycle. The frame machine of the early 1990s took a
        // gas plant past 50% thermal efficiency and could be built in two
        // years rather than ten. It is the reason a country with gas stopped
        // building coal, and the reason electricity got cheaper in the decade
        // when nothing else did.
        tech(
            "ener_gas_turbine_combined_cycle", "F-Class Combined-Cycle Plant", Energy, Information,
            &["core_combined_cycle_turbine"], 70.0, 1990,
            &[EnergyEfficiency(0.04), Productivity(0.00012), Environment(0.04)],
        ),
        // Generation III: Kashiwazaki-Kariwa Unit 6, the world's first ABWR,
        // entered commercial operation on 7 November 1996 at 1,356 MWe. The
        // significance is not the megawatts but that a reactor could be
        // licensed to a standard design instead of built one at a time.
        tech(
            "ener_nuclear_gen_iii", "Generation III Reactor", Energy, Information,
            &[], 105.0, 1996,
            &[EnergyEfficiency(0.02), Productivity(0.00007), Environment(0.05), Stability(0.02)],
        ),

        // -------------------------------------------------------------------
        // Cells, blades and the beginnings of the cost curve
        // -------------------------------------------------------------------

        // Screen-printed crystalline-silicon modules. The process was in volume
        // production through the 1980s and Germany's 1,000 Roofs programme,
        // launched in 1990, put it on grid-connected houses for the first time
        // at any scale. Trivial as generation in 1990; it is the root of the
        // steepest cost curve in the history of manufactured goods.
        tech(
            "ener_crystalline_silicon_module", "Crystalline-Silicon PV Module", Energy, Information,
            &[], 50.0, 1990,
            &[EnergyEfficiency(0.01), Productivity(0.00003), Environment(0.02)],
        ),
        // Variable-speed, electronically controlled wind turbines: Enercon
        // built the first E-40 direct-drive machine in 1993 and sold over a
        // thousand of them by 1999. Letting the rotor change speed and the
        // power electronics deal with the grid is what took wind from a
        // subsidy curiosity to a machine that could be financed.
        tech(
            "ener_variable_speed_turbine", "Variable-Speed Wind Turbine", Energy, Information,
            &["core_cmos_submicron"], 70.0, 1993,
            &[EnergyEfficiency(0.01), Productivity(0.00003), Environment(0.02)],
        ),
        // Megawatt-class turbines. The mid-1990s German and Danish machines
        // (NEG Micon's 1.5 MW, Enercon's E-66) crossed the megawatt line, and
        // scale is the whole economics of wind: energy goes with the square of
        // the blade. UNVERIFIED — the exact first megawatt-class unit is not
        // firmly established here, only that it was the mid-1990s, so the
        // floor is deliberately late rather than early.
        tech(
            "ener_megawatt_wind_turbine", "Megawatt-Class Wind Turbine", Energy, Information,
            &["ener_variable_speed_turbine", "core_carbon_composites"], 100.0, 1996,
            &[EnergyEfficiency(0.02), Productivity(0.00006), Environment(0.03)],
        ),
        // Lithium-ion in volume. Sony commercialised the cell in 1991; what
        // matters for a nation rather than a camcorder is being able to make
        // them by the million, and the manufacturing base built in the 1990s
        // is what everything electrified afterwards was built on.
        tech(
            "ener_lithium_ion_cell", "Lithium-Ion Cell Production", Energy, Information,
            &["core_lithium_ion_cell"], 65.0, 1991,
            &[EnergyEfficiency(0.02), Productivity(0.00006)],
        ),

        // -------------------------------------------------------------------
        // The 2000s: unconventional oil, and renewables that scale
        // -------------------------------------------------------------------

        // SAGD: Foster Creek, piloted from 1997, became the first commercial
        // steam-assisted gravity drainage oil sands project in 2001. A pair of
        // stacked horizontal wells, steam down one and bitumen up the other,
        // turned several hundred billion barrels of tar into a reserve. It is
        // also among the most carbon-intensive barrels anyone produces.
        tech(
            "ener_oil_sands_sagd", "Steam-Assisted Gravity Drainage", Energy, Networked,
            &["ener_horizontal_drilling"], 130.0, 2001,
            &[OilYield(0.03), Environment(-0.03), Productivity(0.00002)],
        ),
        // The shale revolution proper: Devon bought Mitchell Energy in 2002 and
        // drilled the first commercially successful horizontal Barnett well,
        // the Blakely Estate D2, that year — Mitchell's slickwater recipe run
        // down Devon's lateral. Neither half was worth much alone. Cheap gas
        // out of this reorganised American industry and pushed coal off the
        // grid, which is why it carries both a yield and an environment gain.
        tech(
            "ener_shale_gas_extraction", "Horizontal Slickwater Shale Gas", Energy, Networked,
            &["ener_slickwater_fracturing", "ener_rotary_steerable_drilling"], 175.0, 2002,
            &[OilYield(0.05), EnergyEfficiency(0.02), Productivity(0.00020), Environment(0.02)],
        ),
        // Tight oil: EOG drilled the Parshall discovery in the North Dakota
        // Bakken in May 2006 and the field ran from under 100,000 b/d in 2007
        // to a million by 2014. The same technique aimed at oil instead of gas,
        // and the reason a large importer stopped being one.
        tech(
            "ener_tight_oil_production", "Tight Oil Production", Energy, Networked,
            &["ener_shale_gas_extraction"], 190.0, 2006,
            &[OilYield(0.06), Productivity(0.00008), Stability(0.03)],
        ),
        // Utility-scale offshore wind: Vindeby in 1991 was the first offshore
        // farm at all, eleven 450 kW machines, but Horns Rev in 2002 was the
        // first at grid scale — 160 MW, four times the largest before it. The
        // floor is read off the one that mattered.
        tech(
            "ener_offshore_wind_farm", "Utility-Scale Offshore Wind", Energy, Networked,
            &["ener_megawatt_wind_turbine"], 160.0, 2002,
            &[EnergyEfficiency(0.02), Productivity(0.00003), Environment(0.03)],
        ),
        // H-class: GE's 9H entered service at Baglan Bay in September 2003, the
        // first machine designed for 60% combined-cycle efficiency. A fifth
        // less fuel for the same electricity, on plant that already dominated
        // new build.
        tech(
            "ener_h_class_turbine", "H-Class Gas Turbine", Energy, Networked,
            &["ener_gas_turbine_combined_cycle"], 175.0, 2003,
            &[EnergyEfficiency(0.02), Productivity(0.00007), Environment(0.02)],
        ),
        // Utility-scale photovoltaics: the 10 MW Bavaria Solarpark, contracted
        // June 2004 and operational June 2005, was the first plant built as a
        // power station rather than a roof. Fields of modules on trackers, sold
        // to a grid at a tariff — the format every gigawatt since has copied,
        // and the volume that started the module price down its curve.
        tech(
            "ener_photovoltaic_scale", "Utility-Scale Photovoltaics", Energy, Networked,
            &["ener_crystalline_silicon_module"], 165.0, 2005,
            &[
                EnergyEfficiency(0.03),
                Productivity(0.00016),
                Environment(0.05),
                CostReduction { domain: Energy, frac: 0.05 },
            ],
        ),
        // Lithium iron phosphate: A123's nanophosphate cells reached commercial
        // products in 2006, in DeWalt's 36 V tool packs. Less energy per kilo
        // than cobalt chemistry and it does not burn, which is the trade a grid
        // wants and a laptop does not.
        tech(
            "ener_lfp_cell", "Lithium Iron Phosphate Cell", Energy, Networked,
            &["ener_lithium_ion_cell"], 140.0, 2006,
            &[EnergyEfficiency(0.01), Productivity(0.00004)],
        ),
        // Solid-state lighting: Cree commercialised the first LED retrofit
        // downlight in 2008 and the LRP-38 screw-in lamp in May 2009, at about
        // a hundred dollars each. Lighting was a fifth of electricity demand
        // and this cut it by four-fifths — the largest single end-use
        // efficiency gain available to anyone in this period.
        tech(
            "ener_solid_state_lighting", "Solid-State Lighting", Energy, Networked,
            &[], 120.0, 2009,
            &[EnergyEfficiency(0.03), Productivity(0.00008), Environment(0.02)],
        ),
        // LNG mega-trains: Qatargas II's 7.8 mtpa trains started up in 2009,
        // half again larger than any liquefaction unit outside Qatar and driven
        // by the same industrial gas turbines the power plants use. Gas stopped
        // being a regional commodity chained to a pipeline and started being a
        // cargo, which is what let an importer choose a supplier.
        tech(
            "ener_lng_mega_train", "LNG Mega-Train", Energy, Networked,
            &["ener_gas_turbine_combined_cycle"], 185.0, 2009,
            &[
                EnergyEfficiency(0.02),
                Productivity(0.00005),
                InvestmentEfficiency(0.01),
                Stability(0.02),
            ],
        ),

        // -------------------------------------------------------------------
        // The 2010s: moving it, storing it, and the cost curve biting
        // -------------------------------------------------------------------

        // UHVDC: the +/-800 kV Xiangjiaba-Shanghai link was commissioned in July
        // 2010, 7,200 MW over 1,980 km, the first ultra-high-voltage DC line in
        // commercial service. It makes generation and demand independent of
        // each other's geography, which is the precondition for building solar
        // where the sun is rather than where the load is.
        tech(
            "ener_uhvdc_transmission", "Ultra-High-Voltage DC Transmission", Energy, Platform,
            &["core_cmos_submicron"], 260.0, 2010,
            &[
                EnergyEfficiency(0.02),
                Productivity(0.00008),
                InvestmentEfficiency(0.02),
                Stability(0.02),
            ],
        ),
        // Grid-scale batteries: AES put 32 MW of A123 lithium iron phosphate on
        // Laurel Mountain in West Virginia in 2011, the largest lithium-ion
        // installation on any grid at the time, selling frequency regulation
        // into PJM. A battery that earns money from a market rather than a
        // subsidy is the thing that made the rest of them get built.
        tech(
            "ener_grid_battery_storage", "Grid-Scale Battery Storage", Energy, Platform,
            &["ener_lfp_cell"], 240.0, 2011,
            &[
                EnergyEfficiency(0.02),
                Productivity(0.00007),
                CostReduction { domain: Energy, frac: 0.04 },
            ],
        ),
        // Multi-well pad drilling. By 2011 over 83% of Marcellus wells were
        // drilled from shared pads, and the first rig that could walk itself
        // between wellheads was built for Range Resources. Unglamorous, and it
        // is most of where shale's cost collapse actually came from: the rig
        // stops being demobilised and the play becomes a factory.
        tech(
            "ener_pad_drilling", "Multi-Well Pad Drilling", Energy, Platform,
            &["ener_tight_oil_production"], 210.0, 2011,
            &[
                OilYield(0.03),
                Productivity(0.00004),
                CostReduction { domain: Energy, frac: 0.05 },
            ],
        ),
        // PERC. Suntech's Pluto line, the first PERC-derived cell sold
        // commercially, began sales in mid-2009, but the floor here is read off
        // volume manufacture in the early 2010s — the passivation equipment had
        // to get cheap first — and by the second half of 2014 most new capacity
        // being built anywhere was PERC. A point or two of absolute efficiency
        // on the same wafer, across an industry, compounds into everything.
        tech(
            "ener_perc_cell", "Passivated-Emitter Rear Cell", Energy, Platform,
            &["ener_photovoltaic_scale"], 250.0, 2012,
            &[
                EnergyEfficiency(0.02),
                Productivity(0.00008),
                Environment(0.03),
                CostReduction { domain: Energy, frac: 0.05 },
            ],
        ),
        // Floating offshore wind: Hywind Scotland, five 6 MW machines on spar
        // buoys off Peterhead, started generating in October 2017 and has run
        // at the highest capacity factor of any UK offshore farm since. It
        // takes wind off the continental shelf, which is where most of the
        // world's coastline actually is.
        tech(
            "ener_floating_offshore_wind", "Floating Offshore Wind", Energy, Platform,
            &["ener_offshore_wind_farm", "core_carbon_composites"], 290.0, 2017,
            &[EnergyEfficiency(0.01), Productivity(0.00003), Environment(0.02)],
        ),
        // Generation III+: Sanmen 1, the first AP1000, entered commercial
        // operation on 21 September 2018. Passive safety — cooling that works
        // by gravity and evaporation with the power off — is the answer to
        // Fukushima, and the reason a reactor can be argued for again.
        tech(
            "ener_nuclear_gen_iii_plus", "Generation III+ Passive-Safety Reactor", Energy, Platform,
            &["ener_nuclear_gen_iii"], 330.0, 2018,
            &[EnergyEfficiency(0.02), Productivity(0.00004), Environment(0.04), Stability(0.02)],
        ),

        // -------------------------------------------------------------------
        // The 2020s: firming, hydrogen, and the drill arriving in the heat
        // -------------------------------------------------------------------

        // Small modular reactors: the Akademik Lomonosov, two 35 MWe KLT-40S
        // units on a barge at Pevek, entered commercial operation in May 2020.
        // A reactor small enough to be built in a yard and delivered is a
        // different industrial proposition from one poured in place over a
        // decade, whatever one thinks of putting it on a boat.
        tech(
            "ener_small_modular_reactor", "Small Modular Reactor", Energy, Intelligent,
            &["ener_nuclear_gen_iii_plus"], 420.0, 2020,
            &[EnergyEfficiency(0.02), Productivity(0.00005), Environment(0.03), Stability(0.02)],
        ),
        // Solar plus four-hour storage: Vistra's Moss Landing came online on
        // 11 December 2020 at 300 MW and 1,200 MWh, an order of magnitude past
        // anything before it. Storage at that duration is what turns a solar
        // plant from an energy source into a dispatchable one, and it is the
        // point at which the renewables chain and the storage chain stop being
        // separate arguments.
        tech(
            "ener_utility_storage_hybrid", "Firmed Renewable Generation", Energy, Intelligent,
            &["ener_grid_battery_storage", "ener_photovoltaic_scale"], 380.0, 2020,
            &[EnergyEfficiency(0.02), Productivity(0.00005), Environment(0.03)],
        ),
        // Enhanced geothermal: Fervo's Project Red in Nevada drilled the first
        // horizontal well pair in an EGS reservoir, flowed it for thirty days
        // in 2023 and reached commercial operation on 30 October 2023 at
        // 3.5 MW. It is the shale toolkit — horizontal laterals, staged
        // fracturing, fibre downhole — pointed at heat instead of hydrocarbons,
        // which is why it depends on the drilling chain rather than the
        // generation one.
        tech(
            "ener_enhanced_geothermal", "Enhanced Geothermal Systems", Energy, Intelligent,
            &["ener_rotary_steerable_drilling", "ener_slickwater_fracturing"], 450.0, 2023,
            &[EnergyEfficiency(0.02), Productivity(0.00004), Environment(0.03)],
        ),
        // Green hydrogen at industrial scale: Sinopec's Kuqa project in
        // Xinjiang started up in July 2023 with 260 MW of electrolysers on
        // dedicated solar, more installed capacity than all of Europe had at
        // the time, feeding 20,000 t/yr into a refinery. It took until 2025 to
        // approach its rated output, which is the honest state of the
        // technology: it exists, it works, and it is not cheap.
        tech(
            "ener_green_hydrogen_electrolysis", "Industrial Green Hydrogen", Energy, Intelligent,
            &["ener_photovoltaic_scale", "ener_uhvdc_transmission"], 470.0, 2023,
            &[EnergyEfficiency(0.02), Productivity(0.00004), Environment(0.03)],
        ),
        // Perovskite-silicon tandem: Oxford PV shipped the first commercial
        // tandem modules on 5 September 2024, 72 cells at 24.5% module
        // efficiency, to a US utility-scale customer. A hundred kilowatts is
        // nothing; a second junction stacked on the silicon the world already
        // knows how to make is the first credible route past the limit that
        // silicon alone runs into.
        tech(
            "ener_perovskite_tandem", "Perovskite-Silicon Tandem Cell", Energy, Intelligent,
            &["ener_perc_cell"], 400.0, 2024,
            &[EnergyEfficiency(0.02), Productivity(0.00005), Environment(0.02)],
        ),

        // -------------------------------------------------------------------
        // Past the present day
        // -------------------------------------------------------------------

        // SPECULATIVE. No fusion device has produced net electricity for a
        // grid. What exists is a facility-level scientific milestone — NIF's
        // December 2022 shot released more energy than the laser delivered to
        // the target, which is not the same as more than the plant consumed —
        // and a set of pilot plants (ITER, SPARC, STEP) whose schedules are
        // measured in decades. The 2040 floor is a guess at the earliest a
        // pilot plant could plausibly sell power, deliberately late, and it
        // guarantees nothing to anyone. The effect is sized as a
        // general-purpose technology because that is what it would be.
        tech(
            "ener_fusion_pilot_plant", "Fusion Pilot Plant", Energy, Frontier,
            &["ener_small_modular_reactor", "ener_uhvdc_transmission"], 850.0, 2040,
            &[
                EnergyEfficiency(0.04),
                Productivity(0.00015),
                Environment(0.06),
                Stability(0.03),
                ResearchRate(0.03),
                CostReduction { domain: Energy, frac: 0.06 },
            ],
        ),
    ]
}
