//! Agriculture & Environment.
//!
//! OWNER: the agriculture domain author. This file is yours alone. Never edit
//! `tech/mod.rs`, and never touch another domain file — eight authors are
//! working in parallel and the merge is a straight concatenation.
//!
//! SCOPE: crop and livestock breeding including transgenics, fertiliser and
//! pest control, irrigation and water supply, fisheries and aquaculture, soil
//! and land management, forestry, pollution control, climate measurement and
//! mitigation, waste and recycling.
//!
//! Every id here must begin with `agri_`. Prerequisites may name other `agri_`
//! ids in this file, or any `core_` id from the foundation set in `tech/mod.rs`.
//! Nothing else — you cannot see the other domains and they cannot see you.
//!
//! Every entry is a real technology with a real history, and carries a comment
//! naming the first deployment its year floor is read off. Anything past the
//! present day is speculative and must say so in the comment.
//!
//! SHAPE. This domain is unusual in that most of what it does was already
//! invented by 1990 and simply had not reached most of the people who needed
//! it. Diffusion, not invention, is the story: a plastic tunnel, a drip line
//! and a clean banana sucker are cheap, ancient and still absent from most of
//! the world's farms. So the tree is front-loaded and the cheap entries are the
//! generous ones — that is what makes this the branch a poor agrarian state can
//! actually climb. The frontier end of it (autonomous machinery, gene editing,
//! air capture) is expensive and buys comparatively little, which is also true.

use super::TechDef;
#[allow(unused_imports)]
use super::{tech, Domain, Effect, Era};

pub fn techs() -> Vec<TechDef> {
    use Domain::*;
    use Effect::*;
    use Era::*;
    vec![
        // -------------------------------------------------------------------
        // Water. The binding constraint on more arable land than soil is.
        // -------------------------------------------------------------------

        // Drip irrigation: Netafim commercialised the pressure-compensating
        // emitter in the 1960s, and by 1990 it was standard practice across
        // Israel and the American southwest. Delivering water to the root
        // instead of the field roughly halves what a hectare drinks, which is
        // why it is the first thing a water-short state buys.
        tech(
            "agri_drip_irrigation", "Drip Irrigation", Agriculture, Information,
            &[], 45.0, 1990,
            &[ResourceYield(0.03), Productivity(0.00009), Environment(0.03)],
        ),
        // Thin-film composite polyamide membranes: John Cadotte's FT-30
        // chemistry, developed at FilmTec at the turn of the 1980s, is still
        // the barrier layer in the great majority of the world's desalination
        // and water-reuse plants. Everything downstream in this branch is a
        // building wrapped around this membrane.
        tech(
            "agri_composite_membrane", "Thin-Film Composite Membrane", Agriculture, Information,
            &[], 55.0, 1990,
            &[ResourceYield(0.01), Health(0.05), Productivity(0.00003)],
        ),

        // -------------------------------------------------------------------
        // Soil, inputs and the field itself.
        // -------------------------------------------------------------------

        // Zero tillage: 670,000 hectares across the Mercosur countries in 1987,
        // over thirty million by 2002. Not ploughing keeps the soil where it is
        // — Brazilian erosion fell from several tonnes a hectare to under half
        // of one — and it is one of the few agronomic changes that costs a poor
        // farmer less rather than more.
        tech(
            "agri_conservation_tillage", "Conservation Tillage", Agriculture, Information,
            &[], 42.0, 1990,
            &[ResourceYield(0.01), Environment(0.06), Productivity(0.00007)],
        ),
        // Integrated pest management: Indonesia banned 57 formulations of
        // broad-spectrum insecticide on rice by presidential decree in 1986 and
        // ran a national IPM programme from 1989 to 1999 that put more than a
        // million rice farmers through Farmer Field Schools. Brown planthopper
        // stopped being a national emergency. Spraying less is the rare measure
        // that raises yield and lowers cost at once.
        tech(
            "agri_integrated_pest_management", "Integrated Pest Management", Agriculture, Information,
            &[], 50.0, 1990,
            &[ResourceYield(0.01), Environment(0.05), Health(0.05), Productivity(0.00005)],
        ),
        // Protected cultivation: Almería's plastic went from 45 hectares in
        // 1970 to tens of thousands by the 1990s, and turned the driest corner
        // of Spain into the winter vegetable garden of Europe. A polythene
        // tunnel is the cheapest way ever found to move a crop's calendar and
        // cut its water use at the same time.
        tech(
            "agri_protected_cultivation", "Protected Cultivation", Agriculture, Information,
            &[], 52.0, 1990,
            &[ResourceYield(0.01), Productivity(0.00007)],
        ),

        // -------------------------------------------------------------------
        // Breeding, from clean planting material to edited genomes.
        // -------------------------------------------------------------------

        // Micropropagation: Taiwan's Banana Research Institute was selling
        // tissue-cultured, disease-free planting material commercially by 1982,
        // and Israeli laboratories were shipping millions of plantlets to
        // Central American plantations in the early 1990s. Most of the yield
        // gap in vegetatively propagated staples — banana, cassava, potato — is
        // virus carried in the planting material, not genetics.
        tech(
            "agri_tissue_culture_propagation", "Micropropagated Planting Material", Agriculture, Information,
            &[], 44.0, 1990,
            &[ResourceYield(0.01), Health(0.03), Productivity(0.00006)],
        ),
        // Marker-assisted selection: RFLP and then PCR-based markers let a
        // breeder select on a genotype in a seedling instead of waiting for the
        // phenotype in a field, which cuts a breeding cycle from a decade to a
        // few years. Commercial maize and rice programmes were using it by the
        // mid-1990s.
        tech(
            "agri_marker_assisted_selection", "Marker-Assisted Selection", Agriculture, Information,
            &["core_pcr"], 72.0, 1994,
            &[ResourceYield(0.01), ResearchRate(0.02), Productivity(0.00008)],
        ),
        // Transgenic crops: the Flavr Savr tomato cleared the FDA in May 1994,
        // and 1996 was the year it became an industry — herbicide-tolerant
        // soybean and Bt cotton and maize on 1.7 million hectares, almost all
        // of it American. Regenerating a transformed cell into a plant is a
        // tissue-culture problem before it is a genetics one.
        tech(
            "agri_transgenic_crops", "Transgenic Crops", Agriculture, Information,
            &["agri_tissue_culture_propagation", "core_pcr"], 100.0, 1996,
            &[ResourceYield(0.03), Productivity(0.00016), Environment(0.02)],
        ),

        // -------------------------------------------------------------------
        // Fisheries and aquaculture: the protein the bottom of the table eats.
        // -------------------------------------------------------------------

        // Marine net-pen aquaculture: Norway industrialised salmon farming
        // through the 1980s and by 1990 it was a national export industry.
        // The same cages, cheaper, are what put farmed fish into Asian and
        // African markets.
        tech(
            "agri_marine_aquaculture", "Marine Net-Pen Aquaculture", Agriculture, Information,
            &[], 48.0, 1990,
            &[ResourceYield(0.02), Health(0.04), Productivity(0.00006)],
        ),
        // GIFT: ICLARM, the Norwegian institute and Philippine partners ran a
        // selective breeding programme on Nile tilapia from 1988, and by the
        // end of it in 1997 six generations had produced a strain growing some
        // 85% faster than the founders. Seed went to the Philippines,
        // Bangladesh, China, Thailand and Vietnam. Ordinary quantitative
        // genetics, no laboratory required, and it fed hundreds of millions.
        tech(
            "agri_selective_bred_tilapia", "Selectively Bred Farmed Fish", Agriculture, Information,
            &["agri_marine_aquaculture"], 66.0, 1997,
            &[ResourceYield(0.02), Health(0.06), Productivity(0.00007)],
        ),

        // -------------------------------------------------------------------
        // Pollution control.
        // -------------------------------------------------------------------

        // Flue-gas desulphurisation: scrubbers were in service in Japan and
        // West Germany through the 1980s, and Title IV of the 1990 Clean Air
        // Act Amendments made them general — a permanent ten-million-ton cut in
        // American sulphur dioxide, with the first phase binding on 110 power
        // stations from January 1995. Acid rain stopped being a diplomatic
        // problem between neighbours.
        tech(
            "agri_flue_gas_desulphurisation", "Flue-Gas Desulphurisation", Agriculture, Information,
            &[], 60.0, 1990,
            &[Environment(0.14), Health(0.06), Stability(0.03), Productivity(0.00002)],
        ),

        // -------------------------------------------------------------------
        // Precision agriculture: the field stops being one field.
        // -------------------------------------------------------------------

        // Ag Leader's Yield Monitor 2000 shipped in 1992: the first combine
        // instrument accurate enough to say what each pass of the header
        // actually produced. Nothing else in this branch is possible until a
        // farmer can measure the thing he is trying to vary.
        tech(
            "agri_yield_monitor", "Combine Yield Monitoring", Agriculture, Information,
            &["core_cmos_submicron"], 58.0, 1992,
            &[ResourceYield(0.01), Productivity(0.00005)],
        ),
        // Precision farming: a yield monitor, a GPS receiver and grid soil
        // sampling, and the fertiliser rate changes across the field instead of
        // being averaged over it. The package came together in the American
        // Midwest in the mid-1990s as GPS reached full operational capability.
        // Less nitrogen, more grain, and the difference goes into the river
        // rather than the crop only when nobody is measuring.
        tech(
            "agri_precision_farming", "Precision Farming", Agriculture, Information,
            &["agri_yield_monitor", "core_gnss"], 95.0, 1995,
            &[ResourceYield(0.02), Productivity(0.00018), Environment(0.04)],
        ),

        // -------------------------------------------------------------------
        // Controlled environment.
        // -------------------------------------------------------------------

        // The Dutch Venlo glasshouse under computer climate control, with CO2
        // dosing and soilless root zones, was standard practice in the
        // Netherlands by the mid-1990s and made a country the size of a county
        // the world's second agricultural exporter. The exact year the climate
        // computer became general is not one I could pin down; the floor here
        // is calibrated to it being ordinary Dutch practice, not to a launch.
        tech(
            "agri_controlled_environment", "Controlled-Environment Agriculture", Agriculture, Information,
            &["agri_protected_cultivation", "core_cmos_submicron"], 92.0, 1997,
            &[ResourceYield(0.01), Productivity(0.00009), EnergyEfficiency(0.01)],
        ),

        // -------------------------------------------------------------------
        // Fisheries management: the technology is knowing where the boat is.
        // -------------------------------------------------------------------

        // Satellite vessel monitoring: Council Regulation 686/97 put VMS
        // transponders on Community vessels, third-country vessels in
        // Community waters came under it from 1 January 2000, and the size
        // threshold has ratcheted down ever since. Paired with transferable
        // quota it is the only thing that has ever reliably stopped a fishery
        // being fished to nothing — a commons problem solved by a receiver.
        tech(
            "agri_vessel_monitoring", "Satellite Vessel Monitoring", Agriculture, Information,
            &["core_gnss"], 70.0, 1998,
            &[ResourceYield(0.02), Environment(0.04), Stability(0.02), Productivity(0.00004)],
        ),

        // -------------------------------------------------------------------
        // Networked era: the field seen from orbit, the water used twice.
        // -------------------------------------------------------------------

        // Satellite crop monitoring: MODIS began returning data from Terra in
        // February 2000, and the USDA's Foreign Agricultural Service has used
        // its vegetation index to call the world's harvests ever since. A
        // government that can see a drought forming in its own provinces three
        // months early is a government that imports grain before the price
        // moves rather than after the riot.
        tech(
            "agri_satellite_crop_monitoring", "Satellite Crop Monitoring", Agriculture, Networked,
            &["agri_precision_farming", "core_packet_internetworking"], 125.0, 2000,
            &[ResourceYield(0.01), Stability(0.04), Productivity(0.00008)],
        ),
        // Reclaimed water: Singapore's Bedok and Kranji NEWater plants opened
        // in January 2003 — microfiltration, reverse osmosis, ultraviolet — and
        // a city with no watershed stopped being hostage to one. Reuse only
        // pays where the water goes somewhere efficient afterwards, which in
        // practice means under drip.
        tech(
            "agri_wastewater_reuse", "Reclaimed Water Reuse", Agriculture, Networked,
            &["agri_composite_membrane", "agri_drip_irrigation"], 135.0, 2003,
            &[ResourceYield(0.02), Health(0.06), Environment(0.05), Productivity(0.00005)],
        ),
        // Affordable smallholder drip: IDE's low-cost kits, scaled down to a
        // tenth of a hectare and priced at a fifth of the commercial systems,
        // reached over a million Indian households through the 2000s. The
        // engineering is trivial and the effect is not: it is irrigation for
        // people who will never own a pump, on plots too small for anything
        // else to reach.
        tech(
            "agri_smallholder_drip_kit", "Affordable Smallholder Drip", Agriculture, Networked,
            &["agri_drip_irrigation"], 98.0, 2003,
            &[ResourceYield(0.03), Productivity(0.00013), Stability(0.03)],
        ),
        // Large-scale seawater reverse osmosis: Ashkelon came on line in 2005
        // at 330,000 cubic metres a day, then the largest in the world, and did
        // it at a price that made desalinated water an ordinary municipal
        // input rather than an emergency one. Israel stopped arguing about the
        // Jordan's flow shortly afterwards.
        tech(
            "agri_seawater_desalination", "Large-Scale Seawater Desalination", Agriculture, Networked,
            &["agri_composite_membrane"], 180.0, 2005,
            &[ResourceYield(0.03), Health(0.04), Stability(0.03), Productivity(0.00007)],
        ),
        // Controlled-release and stabilised nitrogen: polymer-coated urea moved
        // out of golf courses and into commodity agriculture in the mid-2000s —
        // Agrium's ESN was registered for Canadian food crops in mid-2006 —
        // alongside urease inhibitors for surface-applied urea. Surface
        // application is exactly what no-till forces on you, so the two
        // technologies arrived together by necessity. Half of the nitrogen a
        // poor country imports is currently lost to the air.
        tech(
            "agri_controlled_release_fertiliser", "Controlled-Release Fertiliser", Agriculture, Networked,
            &["agri_conservation_tillage"], 115.0, 2006,
            &[ResourceYield(0.02), Environment(0.06), Productivity(0.00007)],
        ),
        // Biofortification: Uganda released the first HarvestPlus-facilitated
        // biofortified variety, a vitamin-A orange sweet potato, in 2007, and
        // iron beans, zinc wheat and vitamin-A cassava followed. Nutrition
        // built into the staple rather than bought as a supplement, which is
        // the only delivery mechanism that reaches a subsistence household.
        tech(
            "agri_biofortified_staple", "Biofortified Staple Crops", Agriculture, Networked,
            &["agri_marker_assisted_selection"], 120.0, 2007,
            &[Health(0.14), Productivity(0.00005)],
        ),

        // -------------------------------------------------------------------
        // Platform era.
        // -------------------------------------------------------------------

        // Drought tolerance: Monsanto's MON 87460 was deregulated in the United
        // States in 2011 and sold as DroughtGard from 2013, and the Water
        // Efficient Maize for Africa programme had adapted DroughtTEGO hybrids
        // in African trials over 2011-2013. The measured effect is modest — a
        // few points of yield saved in a moderate drought — but a few points is
        // the difference between a thin year and a famine year.
        tech(
            "agri_drought_tolerant_maize", "Drought-Tolerant Cereals", Agriculture, Platform,
            &["agri_transgenic_crops", "agri_biofortified_staple"], 250.0, 2013,
            &[ResourceYield(0.02), Productivity(0.00009), Stability(0.03)],
        ),
        // Machine-vision agronomy: PlantVillage's Nuru went free on Android in
        // 2018, diagnosing cassava mosaic and brown streak from a phone camera
        // with no network connection, and tested about twice as accurate as the
        // human extension agents it was compared against. An extension service
        // that costs nothing per farmer is a different kind of institution from
        // one that costs a salary per village.
        tech(
            "agri_ai_crop_advisory", "Machine-Vision Crop Advisory", Agriculture, Platform,
            &["agri_satellite_crop_monitoring", "core_gpu_deep_learning"], 275.0, 2018,
            &[ResourceYield(0.01), Productivity(0.00011), DiffusionSpeed(0.03)],
        ),
        // Genome editing in crops: Calyxt's high-oleic soybean oil went on sale
        // in the United States in February 2019, the first gene-edited food to
        // reach a consumer — two genes switched off, nothing inserted, and in
        // most jurisdictions therefore not a transgenic at all. The regulatory
        // asymmetry is most of why it matters.
        tech(
            "agri_genome_edited_crop", "Genome-Edited Crops", Agriculture, Platform,
            &["agri_marker_assisted_selection", "core_crispr_editing"], 310.0, 2019,
            &[ResourceYield(0.01), Productivity(0.00009), ResearchRate(0.02)],
        ),

        // -------------------------------------------------------------------
        // Intelligent era.
        // -------------------------------------------------------------------

        // Direct air capture: Climeworks' Orca started up in Iceland in
        // September 2021 — eight collector containers, four thousand tonnes of
        // CO2 a year, mineralised underground on geothermal heat. Four thousand
        // tonnes is nothing against forty billion, and that is the point: it is
        // the same sorbent-and-contactor engineering as flue-gas scrubbing,
        // arriving at a price nobody can yet justify. Scored accordingly.
        tech(
            "agri_direct_air_capture", "Direct Air Capture", Agriculture, Intelligent,
            &["agri_flue_gas_desulphurisation"], 480.0, 2021,
            &[Environment(0.10), Productivity(0.00001)],
        ),
        // Autonomous field machinery: John Deere showed a driverless 8R at CES
        // in January 2022 and called it ready for large-scale production — six
        // stereo camera pairs, a neural network classifying every pixel in a
        // tenth of a second, and a geofence. It removes the last labour input
        // from arable farming in the countries that already have the least of
        // it, which is why it does less for the bottom of the table than a
        // drip kit does.
        tech(
            "agri_field_robotics", "Autonomous Field Machinery", Agriculture, Intelligent,
            &["agri_ai_crop_advisory", "agri_precision_farming"], 430.0, 2022,
            &[
                ResourceYield(0.01),
                Productivity(0.00013),
                InvestmentEfficiency(0.02),
            ],
        ),
    ]
}
