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
//! naming the first deployment its year floor is read off. Past the present day
//! the comment opens with its bucket: `ROADMAP` where the date is somebody
//! else's published, dated, funded commitment, and `SPECULATIVE` where it is
//! not. `SPECULATIVE` is permitted only in the Frontier era, whose own
//! definition already requires it.
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

/// The Energy: extraction, generation, storage and the grid that moves it.
///
/// Data only — the scoring engine lives in `tech::mod`.
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
        // The 2020s, transcribed 2026-09
        //
        // Everything down to the roadmap block has a floor at or below 2026 and
        // is therefore history rather than expectation. One note for whoever
        // audits this later: none of it can move the 1990-2025 record. The
        // cheapest entry in this block costs 330 research points against a
        // measured worst-case Energy tie-price of 165 at January 2025 — the raw
        // cost at which a new entry would exactly tie the project a nation is
        // already focused on — so no nation can be diverted onto one of these
        // before the golden checkpoint, whatever it already knows.
        // -------------------------------------------------------------------

        // HISTORY. Deployed 2020-Q4; transcribed 2026-09. Not one deployment
        // but a displacement, like the seismic entry at the top of this file:
        // pumping two or three wells at once off a single spread instead of
        // alternating between them. Rystad Energy counted it at under 1% of US
        // onshore completions in 2019, 4% across 2020 — some 250 wells of about
        // six thousand — and 8% in the fourth quarter alone, with the stages
        // going down 60% faster than zipper fracturing and Ovintiv reporting
        // $400,000 off the average well. Halliburton did roughly a third of the
        // 2020 jobs, and on 19 October 2021 put the power supply under it on a
        // multi-year footing: an all-electric spread of 5,000-horsepower pumps
        // for Chesapeake in the Marcellus, running on 25 MW generated from the
        // pad's own gas. The floor is 2021 because that is when the technique
        // and the power plant to run it were both under contract rather than on
        // trial. Triple-well completions are the norm now. The displaced diesel
        // is real and is not scored: a fleet burning field gas to frack faster
        // is a cheaper barrel, not a cleaner grid.
        tech(
            "ener_simultaneous_fracturing", "Simultaneous Multi-Well Fracturing", Energy, Intelligent,
            &["ener_pad_drilling"], 340.0, 2021,
            &[
                OilYield(0.02),
                Productivity(0.00003),
                CostReduction { domain: Energy, frac: 0.04 },
            ],
        ),
        // HISTORY. Deployed 2023-12-06; transcribed 2026-09. Shidao Bay-1, the
        // HTR-PM: two 250 MWt helium-cooled pebble-bed modules driving one
        // 210 MWe turbine, owned by China Huaneng, engineered by CNNC and
        // designed at Tsinghua, entered commercial operation on 6 December 2023.
        // What distinguishes it from the passive-safety entry above is that the
        // safety is not a system at all. In August and September 2023 the
        // operators cut power to both modules at full load with no emergency
        // cooling of any kind and let them sit; each settled at a stable
        // temperature within about 35 hours. Tsinghua published the results in
        // Joule in July 2024, the first loss-of-cooling test on a
        // commercial-scale high-temperature reactor. A reactor that cannot melt
        // is licensable in places a reactor that must not melt is not, and its
        // 500 C steam goes somewhere a light-water plant's cannot.
        tech(
            "ener_high_temperature_reactor", "High-Temperature Gas-Cooled Reactor", Energy, Intelligent,
            &["ener_nuclear_gen_iii_plus"], 470.0, 2023,
            &[
                EnergyEfficiency(0.02),
                Productivity(0.00004),
                Environment(0.03),
                Stability(0.02),
            ],
        ),
        // HISTORY. Deployed 2023-11-07; transcribed 2026-09. Fuel enriched
        // between five and twenty percent, which nearly every advanced reactor
        // design needs and which no Western plant made. Centrus began enriching
        // at the American Centrifuge Plant in Piketon, Ohio on 11 October 2023
        // and delivered the first 20 kilograms of HALEU hexafluoride to the
        // Department of Energy on 7 November 2023; the contract's second phase,
        // a full year at a 900 kg/yr rate, closed out in June 2025. Twenty
        // kilograms is a rounding error against a reactor core and the number is
        // not the point — the point is that the bottleneck under every entry in
        // this block was a fuel nobody outside Russia was licensed to produce.
        // The prerequisite is the reactor and not the centrifuge, because the
        // centrifuge is not in this file and because nobody enriches to twenty
        // percent for a market that does not exist yet.
        tech(
            "ener_haleu_enrichment", "High-Assay Low-Enriched Fuel", Energy, Intelligent,
            &["ener_small_modular_reactor"], 330.0, 2023,
            &[
                EnergyEfficiency(0.01),
                Productivity(0.00002),
                CostReduction { domain: Energy, frac: 0.03 },
            ],
        ),
        // HISTORY. Deployed 2024-06-30; transcribed 2026-09. Datang's Qianjiang
        // station in Hubei entered operation with 50 MW and 100 MWh of
        // sodium-ion cells — HiNa 185 Ah, forty-two bays — as the first phase
        // of 200 MWh. The chemistry is worse than lithium on every axis except
        // the one a grid cares about: it is made of sodium and aluminium, so a
        // country with no lithium and no cobalt can still build storage. INERT
        // BEFORE 2025 BY PREREQUISITE AND NOT BY THE 2024 FLOOR: measured
        // monthly across twelve seeds to January 2025, no nation ever holds the
        // grid battery entry this hangs off, so it never enters a candidate set
        // at all in the years the golden hashes cover.
        tech(
            "ener_sodium_ion_storage", "Sodium-Ion Grid Storage", Energy, Intelligent,
            &["ener_grid_battery_storage"], 360.0, 2024,
            &[
                EnergyEfficiency(0.02),
                ResourceYield(0.01),
                Environment(0.02),
                Productivity(0.00003),
            ],
        ),
        // HISTORY. Deployed 2024-08-12; transcribed 2026-09. Chevron started
        // production from Anchor in Green Canyon, about 140 miles off Louisiana,
        // on 12 August 2024: seven subsea wells in 5,000 ft of water reaching
        // reservoirs 34,000 ft below sea level, on equipment rated to 20,000 psi
        // where every deepwater development before it stopped at 15,000. A third
        // more pressure than anyone had drilled sounds incremental and is not —
        // wellheads, risers, blowout preventers and trees all had to be
        // requalified, which took from the 2019 sanction to first oil. It opens
        // the lower tertiary reservoirs that were mapped decades ago and left
        // where they were. It moves recovery and it moves nothing else, which is
        // why there are two terms here and not four.
        tech(
            "ener_ultra_high_pressure_completion", "Ultra-High-Pressure Completion", Energy, Intelligent,
            &["ener_deepwater_subsea"], 430.0, 2024,
            &[OilYield(0.03), Productivity(0.00003)],
        ),
        // HISTORY. Deployed 2025-08-25; transcribed 2026-09. Northern Lights,
        // owned equally by Equinor, Shell and TotalEnergies, injected its first
        // CO2 on 25 August 2025 — captured at Heidelberg Materials' cement works
        // at Brevik, shipped as a liquid to Oygarden, piped 100 km and put
        // 2,600 m under the seabed into the Aurora reservoir. It is the first
        // open-access CO2 transport and storage network anyone can buy into, and
        // that is the whole of its significance: capture has been demonstrated
        // for decades and had nowhere to send the product. Phase 1 is 1.5 Mt a
        // year and is fully booked; phase 2 takes it past 5 Mt from 2028.
        // Against annual emissions counted in the tens of billions of tonnes
        // that is nothing, and the effect is sized as what it is — a disposal
        // route for the industries that cannot electrify, priced at a level
        // only a carbon market sustains. It hangs off the offshore and the LNG
        // chains because it is made of both: a subsea injection well and a
        // refrigerated cargo ship.
        tech(
            "ener_co2_transport_storage", "Offshore CO2 Storage Network", Energy, Intelligent,
            &["ener_deepwater_subsea", "ener_lng_mega_train"], 450.0, 2025,
            &[Environment(0.05), Productivity(0.00002)],
        ),
        // ROADMAP. Not ours: Fervo's Cape Station in Utah, financed by a $462m
        // Series E closed in December 2025 on about $1.5bn raised since 2017,
        // with 100 MW stated for October 2026 and a further 400 MW for 2028.
        // The wells are the evidence and not the press release — Sawtooth 7,
        // spudded to 19,448 ft with a 7,500 ft lateral in 460 F rock in 21 days
        // in July 2026, against a first Cape well that took nearly four times
        // as long. That learning curve is the entry: the pilot two lines up
        // proved the reservoir works, and this proves it can be drilled at the
        // price of a gas plant. The floor is 2029, a year past Fervo's own date
        // for the larger tranche, because the 2028 wells are not drilled yet
        // and because a schedule with a month in it has never yet been the
        // month. Note that if October 2026 lands, the first half of this comment
        // is history and the label above it is stale — read it again.
        tech(
            "ener_utility_scale_geothermal", "Utility-Scale Enhanced Geothermal", Energy, Intelligent,
            &["ener_enhanced_geothermal", "ener_pad_drilling"], 540.0, 2029,
            &[
                EnergyEfficiency(0.03),
                Productivity(0.00005),
                Environment(0.03),
                CostReduction { domain: Energy, frac: 0.03 },
            ],
        ),
        // ROADMAP. Not ours, and the two parties disagree, which is the useful
        // part. Toyota holds a METI certification issued in September 2024 to
        // manufacture all-solid-state cells in Japan, contracted Sumitomo Metal
        // Mining for cathode material on 8 October 2025 and Idemitsu Kosan for
        // the sulphide electrolyte, and says a first vehicle in 2027 or 2028.
        // THE ELECTROLYTE IS THE HARDER HALF AND IT IS THE BETTER ANCHOR:
        // Idemitsu took a final investment decision on a lithium sulphide plant
        // at Chiba, broke ground in January 2026 with Chiyoda building it, and
        // expects completion in 2027 at about a thousand tonnes a year. A plant
        // under construction with a named builder and a completion year is
        // firmer evidence than a certification, which is a permission to build
        // and not a line running.
        // CATL, which makes more cells than anyone, says otherwise: its chairman
        // Robin Zeng told the World Economic Forum meeting in Dalian on 24 June
        // 2026 that the technology sits at level four of a nine-step readiness
        // scale, that the inflection is not before 2030, and that commercial
        // viability is unproven. The floor is 2029 — a year past the later of
        // Toyota's two dates, two years past the electrolyte plant's own
        // completion year, and a year before CATL's earliest — because when the
        // largest manufacturer in a field publicly discounts its rival's
        // schedule that discount is data. This is the cell and not the car:
        // solid electrolyte on the plant the lithium-ion entry already built.
        tech(
            "ener_solid_state_battery", "Solid-State Battery Cell", Energy, Intelligent,
            &["ener_lithium_ion_cell"], 560.0, 2029,
            &[EnergyEfficiency(0.03), Productivity(0.00005), Environment(0.02)],
        ),

        // -------------------------------------------------------------------
        // Past the present day
        // -------------------------------------------------------------------

        // ROADMAP. Not ours, and distinct from the pilot plant below: a device
        // that returns more energy than the plasma was given, which is a physics
        // result and not a power station. Commonwealth Fusion's SPARC has its
        // cryostat base in place and the first of eighteen toroidal-field
        // magnets installed, with first plasma advertised for 2026 and Q>1 for
        // 2027; ITER re-baselined in July 2024 to Start of Research Operation in
        // 2034 and deuterium-tritium in 2039. Both are funded, dated and under
        // construction, and both have slipped before. The floor is 2030: the
        // earlier of the two claims, plus the margin every fusion schedule in
        // this file's lifetime has needed. The effects are small on purpose —
        // this entry is the option and the pilot plant is the exercise of it.
        tech(
            "ener_net_energy_fusion", "Net-Energy Fusion Device", Energy, Frontier,
            &["ener_small_modular_reactor"], 690.0, 2030,
            &[ResearchRate(0.05), Productivity(0.00002), Environment(0.01)],
        ),
        // ROADMAP. Not ours: the NRC issued a construction permit to TerraPower
        // for Kemmerer Unit 1 in Wyoming on 4 March 2026 — the first ever issued
        // for a commercial-scale advanced reactor — and TerraPower began nuclear
        // construction on 23 April 2026, having poured the non-nuclear side
        // since June 2024 under a Department of Energy cost share. It is a
        // 345 MWe sodium-cooled fast reactor whose heat goes into molten salt
        // before it goes into a turbine, so the plant can hold output and sell
        // 500 MW into a peak. TerraPower says complete in 2030. The floor is
        // 2032 and not 2030 because a first-of-a-kind nuclear schedule is the
        // least reliable number in this file — the Generation III+ entry above
        // was eight years late in the West — and because sodium is a coolant
        // that has closed more reactors than it has opened. There is no resource
        // term here on purpose: this is a fast reactor that does not breed, and
        // burns the fuel of the entry above rather than making its own.
        tech(
            "ener_sodium_fast_reactor", "Sodium-Cooled Fast Reactor", Energy, Frontier,
            &["ener_small_modular_reactor", "ener_haleu_enrichment"], 660.0, 2032,
            &[
                EnergyEfficiency(0.03),
                Productivity(0.00008),
                Environment(0.04),
                Stability(0.02),
            ],
        ),
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
