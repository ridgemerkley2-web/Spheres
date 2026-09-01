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
//! naming the first deployment its year floor is read off. Past the present day
//! the comment opens with its bucket: `ROADMAP` where the date is somebody
//! else's published, dated, funded commitment, and `SPECULATIVE` where it is
//! not. `SPECULATIVE` is permitted only in the Frontier era, whose own
//! definition already requires it.
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

/// The Agriculture: yield, inputs, mechanisation and the end of famine as policy.
///
/// Data only — the scoring engine lives in `tech::mod`.
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
        //
        // Every entry from here down costs at least 300, the Intelligent band
        // floor, against a measured Agriculture focus-displacement price of 115
        // at January 2025. So none of them can take a nation's research focus in
        // the years the golden hashes cover, whatever its year floor says. That
        // is the inertness argument for the entries below whose floor is 2026 or
        // earlier, and it is worth writing down because it is price and not the
        // floor that does the work: a 2020 floor does not make an entry quiet, a
        // cost of 340 against a tie price of 115 does.
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
        // HISTORY. In force 2020-01-01; transcribed 2026-09. MARPOL Annex VI
        // Regulation 14 dropped the global ceiling on fuel-oil sulphur from
        // 3.50% to 0.50% on 1 January 2020, and the fleet complied by burning
        // very-low-sulphur fuel oil or by fitting the same wet scrubbers the
        // entry above put on power stations — which is why it hangs off that
        // entry and not off anything to do with ships. The IMO forecast a 77%
        // cut in shipping's sulphur oxides, 8.5 million tonnes a year, and cites
        // a 2016 Finnish submission to its own Marine Environment Protection
        // Committee putting the cost of not acting at more than 570,000
        // additional premature deaths over 2020-2025. Note what else it did.
        // Ship sulphur was seeding marine cloud, and taking it out removed a
        // cooling nobody had counted: Yuan et al. put the forcing at about
        // 0.2 W/m2 over the global ocean, Gettelman et al. and Jordan et al.
        // both published in 2024 that the rule brought global warming forward by
        // something like two to three years. The cleanest air regulation of the
        // decade is also the largest accidental geoengineering experiment ever
        // run, and this file scores the health and leaves the climate ledger
        // alone because it has no channel that can hold both signs at once.
        tech(
            "agri_low_sulphur_marine_fuel", "Low-Sulphur Marine Fuel", Agriculture, Intelligent,
            &["agri_flue_gas_desulphurisation"], 340.0, 2020,
            &[Environment(0.07), Health(0.05), Stability(0.02)],
        ),
        // HISTORY. Registered 2023-12-22; transcribed 2026-09. The EPA
        // registered ledprona — GreenLight Biosciences' Calantha, EPA Reg. No.
        // 94614-E — on 22 December 2023, the first sprayable double-stranded RNA
        // allowed to be applied to a crop anywhere. It silences the gene for the
        // PSMB5 proteasome subunit in Colorado potato beetle and does nothing to
        // anything that does not carry that sequence, which is the whole
        // argument for it: a pesticide whose selectivity is written in the
        // target's own genome rather than inferred from its physiology. The
        // plant is not modified, so most jurisdictions regulate the spray and
        // not the potato. Two cautions, both on the record. The registration ran
        // three years, not indefinitely. And GreenLight was taken private by
        // Fall Line Capital in 2023 for $45.5 million after a SPAC listing that
        // did not work, and closed a $25 million Series C in April 2025 to keep
        // Calantha selling — a first-in-class approval and a company that size
        // are not the same news. The prerequisites are the antisense lineage
        // (Flavr Savr was an RNA-silencing construct before anyone called it
        // that) and the discipline that decides whether you spray at all.
        tech(
            "agri_rnai_biopesticide", "RNA Interference Biopesticide", Agriculture, Intelligent,
            &["agri_transgenic_crops", "agri_integrated_pest_management"], 370.0, 2023,
            &[Environment(0.04), Health(0.02), ResourceYield(0.01), Productivity(0.00002)],
        ),
        // HISTORY. Launched 2024-08-16; transcribed 2026-09. Tanager-1 — Carbon
        // Mapper, Planet and a JPL imaging spectrometer — went up from
        // Vandenberg on 16 August 2024 and in its first year published 5,392
        // methane and 1,234 carbon dioxide plumes from 3,563 distinct sources,
        // resolved to the individual facility and in some cases the individual
        // piece of equipment. On 9 October 2024 it found a Permian pipeline
        // venting about 7,000 kg of methane an hour; the operator was told,
        // repaired it, and a later pass showed no plume. That loop — see it,
        // name it, watch it stop — is the technology, and it is why the entry
        // pairs orbital imaging with the classifier that turns a spectrum into a
        // notification rather than into a research paper. The honest ledger is
        // mixed: MethaneSAT, the Environmental Defense Fund's own satellite,
        // launched 4 March 2024, lost power on 20 June 2025 after fifteen months
        // and was not recoverable. A capability that depends on one spacecraft
        // is not yet a capability, which is the argument for the constellation
        // and against scoring this any higher than it is scored.
        tech(
            "agri_orbital_plume_detection", "Orbital Plume Detection", Agriculture, Intelligent,
            &["agri_satellite_crop_monitoring", "core_gpu_deep_learning"], 420.0, 2024,
            &[Environment(0.05), Stability(0.02), Productivity(0.00001)],
        ),
        // HISTORY. Approved 2025-04-30; transcribed 2026-09. The FDA approved
        // PIC's gene edit on 30 April 2025: exon 7 of CD163 deleted, so the
        // receptor porcine reproductive and respiratory syndrome virus uses to
        // enter a macrophage is not there, and the pig cannot be infected. It is
        // the first edit approved for a commercial food animal in the United
        // States. The disease it removes is the largest single cost in the
        // industry — Holtkamp's Iowa State group put it at $1.2 billion a year
        // in lost American production over 2016-2020, up 80% on his own
        // 2006-2010 estimate — and a study of 97 meat quality and composition
        // measures found nothing different about the pork but the resistance.
        // Approval is not sale: PIC said it would not commercialise in the
        // United States before 2026 and is collecting determinations elsewhere
        // first, Colombia and Brazil already given, Canada in January 2026,
        // because a pig that cannot be exported is not worth breeding. Health is
        // scored small and is not the pig's: sick herds are dosed herds, and the
        // human interest in this is antimicrobial resistance.
        tech(
            "agri_gene_edited_livestock", "Gene-Edited Livestock", Agriculture, Intelligent,
            &["core_crispr_editing", "agri_genome_edited_crop"], 500.0, 2025,
            &[ResourceYield(0.02), Health(0.02), Productivity(0.00004)],
        ),
        // HISTORY. Data opened 2026-01-26; transcribed 2026-09. ESA's Biomass
        // launched on a Vega-C from Kourou on 29 April 2025 carrying the first
        // P-band synthetic aperture radar ever flown, on a twelve-metre
        // deployable reflector. P-band goes through the canopy to the trunks and
        // the large branches, which is where the carbon actually is; everything
        // before it measured the top of the forest and inferred the rest. The
        // floor is 2026 and not 2025 because a launch is not a capability: the
        // satellite spent eight months in commissioning and the archive went
        // open to all on 26 January 2026, and it is the open archive rather than
        // the spacecraft that changes what a forestry ministry or a timber buyer
        // can know. The prerequisite is the file's remote-sensing trunk for the
        // same reason satellite crop monitoring hangs off precision farming: the
        // instrument is worthless to a state that has never learned to act on a
        // picture of its own territory.
        tech(
            "agri_forest_biomass_radar", "Forest Biomass Radar", Agriculture, Intelligent,
            &["agri_satellite_crop_monitoring"], 360.0, 2026,
            &[Environment(0.04), ResourceYield(0.01), Productivity(0.00001)],
        ),
        // ROADMAP. Not ours: the Copernicus CO2M mission, three satellites built
        // by OHB System under a prime contract ESA signed in 2020 and has since
        // authorised into production, the first two planned for launch in 2027
        // and the third delivered in 2029. It is the first instrument designed
        // to separate the anthropogenic share of a country's carbon dioxide from
        // the natural one at national scale, which makes a Paris inventory an
        // audited number rather than a declared one, and EUMETSAT will fly it
        // for the global stocktake. The floor is 2029 and not 2027, for two
        // reasons that are both on the record. ESA's own schedule delivers the
        // first platform to the launcher by the end of 2027 and the third in
        // 2029, and a delivery is not a data product — Biomass, the entry above,
        // launched in April 2025 and its archive did not open until January
        // 2026, eight months later, on a mission that went well. And one
        // satellite is not a verification system: the capability arrives when
        // the constellation does. The prerequisites are the two halves of a
        // national inventory that a state cannot fake — the fossil plumes and
        // the standing biomass — and neither on its own audits anybody. No
        // Productivity: a treaty verified is not an economy improved.
        tech(
            "agri_emission_inventory_verification", "National Emissions Verification",
            Agriculture, Intelligent,
            &["agri_orbital_plume_detection", "agri_forest_biomass_radar"], 560.0, 2029,
            &[Environment(0.05), Stability(0.03)],
        ),

        // -------------------------------------------------------------------
        // Frontier era: SPECULATIVE. Past the present day, so extrapolated, and
        // written to read as extrapolation. Each comment states what is known,
        // what is assumed, and what would falsify it, in that order. Note the
        // shape the file's own header predicted: the frontier of this domain is
        // expensive and buys comparatively little, and BOTH entries below would
        // be beaten, in effect per research point, by a drip kit — 45 points for
        // ResourceYield 0.03, Environment 0.03 and Productivity 0.00009, against
        // 880 and 900 here for less of each.
        // -------------------------------------------------------------------

        // SPECULATIVE — nothing at this scale exists or is funded. Known: the
        // largest plant ever built, 1PointFive's STRATOS in Ector County, Texas,
        // holds the first EPA Class VI permits ever issued for sequestering
        // direct-air-captured CO2, granted 7 April 2025, and is designed for
        // 500,000 tonnes a year — a hundred and twenty-five times the Orca plant
        // this file's Intelligent entry is read off. It began a phased ramp in
        // early 2026 and had not reached capacity by the middle of that year,
        // held up in commissioning by a component that was not part of the
        // capture process. Known and less comfortable: Climeworks' Mammoth,
        // nameplate 36,000 tonnes a year, captured 105 tonnes in 2024 — three
        // tenths of one per cent — and the company laid off a tenth of its staff
        // in mid-2025 having raised over $800 million. Assumed: that the cost
        // per tonne falls by roughly an order of magnitude and that somebody
        // keeps paying for a service with no product, for long enough to reach
        // a scale that registers against forty billion tonnes a year. The floor
        // is 2038 and is a guess, set late on purpose. The second prerequisite
        // is the one that matters and is not obvious: a tonne removed that
        // nobody independently verified is a tonne nobody will buy twice, which
        // is why this hangs off the verification entry and not off capture
        // alone. Environment is scored above the existing direct-air-capture
        // entry's 0.10 because this is three orders of magnitude larger; that
        // the difference is only 0.02 is a fair reading of how generous the
        // earlier entry was to four thousand tonnes. What would falsify it:
        // STRATOS running for a full year well under nameplate, the way Mammoth
        // did. That is a measurement that will exist, and it should be checked.
        tech(
            "agri_gigatonne_carbon_removal", "Gigatonne Carbon Removal", Agriculture, Frontier,
            &["agri_direct_air_capture", "agri_emission_inventory_verification"], 880.0, 2038,
            &[Environment(0.12), Stability(0.02)],
        ),
        // SPECULATIVE — no cereal has ever been made to nodulate, and this is
        // the largest thing in this file that has not happened. Known: on
        // 5 November 2025 an Aarhus-led group published in Nature that two amino
        // acid residues in the juxtamembrane region of a receptor kinase decide
        // whether the plant reads a bacterium as an infection or as a partner,
        // and that changing them converts an immunity receptor into a symbiotic
        // one — demonstrated in Lotus japonicus, and shown to work with a
        // receptor taken from barley. Known and separate: Pivot Bio sells
        // gene-edited free-living soil bacteria that have replaced an average of
        // 33 lb of nitrogen an acre across millions of American maize acres, so
        // the microbial half of the problem is already a product. Assumed: that
        // a signalling result becomes an organ — a nodule is a structure a
        // cereal has no genetic programme for, and reprogramming the receptor is
        // the first of several steps nobody has taken; and that the ENSA
        // consortium's second phase, which is where the nodule organogenesis
        // work sits, gets there. The floor is 2040, is mine, and is deliberately
        // late. If it arrives it is worth more than anything else in this
        // domain: nitrogen fertiliser is the largest cash cost a poor cereal
        // farmer has and the reason ammonia is near 2% of world carbon dioxide,
        // and a maize that makes its own is both of those problems gone at once,
        // which is why Stability is positive and why this is the file's most
        // expensive entry. The prerequisites are the two techniques the route
        // actually needs, an edit to the receptor and an inserted programme for
        // the nodule. What would falsify it: nodule organogenesis failing to
        // transfer, which is the outcome forty years of trying says to expect.
        // If it does not arrive, the entry simply stays unbuilt.
        tech(
            "agri_nitrogen_fixing_cereal", "Nitrogen-Fixing Cereals", Agriculture, Frontier,
            &["agri_genome_edited_crop", "agri_transgenic_crops"], 900.0, 2040,
            &[ResourceYield(0.03), Environment(0.06), Productivity(0.00006), Stability(0.03)],
        ),
    ]
}
