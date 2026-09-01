//! Communications & Space.
//!
//! OWNER: the communications domain author. This file is yours alone. Never edit
//! `tech/mod.rs`, and never touch another domain file — eight authors are
//! working in parallel and the merge is a straight concatenation.
//!
//! SCOPE: fixed and mobile networks, optical transport, satellite communications
//! and earth observation, civil launch and orbital infrastructure, navigation,
//! broadcasting, and the internet as infrastructure rather than as software.
//!
//! Every id here must begin with `comm_`. Prerequisites may name other `comm_`
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
//! SHAPE. Four chains run the length of this file and they are what the domain
//! is: glass, radio, orbit, and position. The glass chain starts with a national
//! trunk and ends with the wavelength; the radio chain starts with one analogue
//! cell and ends with a handset talking to a satellite; the orbit chain starts
//! with one expensive geostationary bird and ends with launch cheap enough that
//! the constellation is the network; the position chain starts with a receiver
//! that knows where it is and ends with a state that owns the timing signal its
//! grid and its missiles depend on. Nothing here jumps a chain. A nation that
//! never built the trunk does not get the wavelength.
//!
//! PROPORTION. Communications is a diffusion domain before it is a productivity
//! one. Its whole Productivity budget is 0.0020 — two tenths of a point of trend
//! growth — but it carries most of the tree's `DiffusionSpeed`, because what a
//! telephone network does to an economy is mostly to let it find out what
//! everyone else already knows.

use super::TechDef;
#[allow(unused_imports)]
use super::{tech, Domain, Effect, Era};

/// The Communications: switching, fibre, cellular and the networks over them.
///
/// Data only — the scoring engine lives in `tech::mod`.
pub fn techs() -> Vec<TechDef> {
    use Domain::*;
    use Effect::*;
    use Era::*;
    vec![
        // -------------------------------------------------------------------
        // Information era: the wires and the first cells
        // -------------------------------------------------------------------

        // Stored-program digital exchanges and out-of-band signalling. AT&T's
        // 5ESS entered service in 1982 and SS7 signalling was carrying the US
        // network by the late 1980s, which is why 1990 is the floor rather than
        // the date: a country that has not converted its exchanges in 1990 is
        // late, not early. What it buys is not calls but everything a call can
        // now be — a leased line set up in minutes instead of weeks.
        // (Date not re-verified this session; the floor is the era minimum, so
        // nothing in the simulation turns on the precise year.)
        tech(
            "comm_digital_switching", "Digital Switching and Common-Channel Signalling",
            Communications, Information,
            &[], 45.0, 1990,
            &[Productivity(0.00008), InvestmentEfficiency(0.01)],
        ),
        // A nationwide trunk network with glass all the way through it. Sprint
        // finished cutting the last traffic off copper in May 1988 and was the
        // first carrier anywhere that was one hundred percent digital fibre;
        // AT&T and MCI had caught up by the early nineties. This is the thing
        // every later transport entry is an upgrade to, and the reason a country
        // that skipped it spends the next thirty years buying capacity abroad.
        tech(
            "comm_fibre_optic_backbone", "National Fibre Backbone",
            Communications, Information,
            &["core_single_mode_fiber"], 60.0, 1990,
            &[Productivity(0.00010), DiffusionEmission(0.03)],
        ),
        // Geostationary communications satellites have been commercial since
        // Early Bird in 1965, and by 1990 the Intelsat V and VI series carried
        // the long-haul traffic that had no cable under it. Expensive per bit
        // and always will be, but it reaches everywhere at once, which is what
        // makes it the root of the orbital chain and not a dead end.
        // (Pre-1990 date not re-verified this session; the floor is the era
        // minimum regardless.)
        tech(
            "comm_geostationary_comsat", "Geostationary Communications Satellite",
            Communications, Information,
            &[], 55.0, 1990,
            &[Productivity(0.00003), DiffusionSpeed(0.02), MilitaryEfficiency(0.02)],
        ),
        // First-generation cellular: NMT in the Nordics from 1981 and AMPS in
        // Chicago from October 1983. Analogue, expensive and capacity-starved,
        // and the reason it belongs in the tree at all is that it is what
        // teaches a country to build cell sites, backhaul them and bill for
        // them — none of which the digital network gets to skip.
        // (Dates not re-verified this session; floor is the era minimum.)
        tech(
            "comm_cellular_analogue", "Analogue Cellular Telephony",
            Communications, Information,
            &["comm_digital_switching"], 50.0, 1990,
            &[Productivity(0.00004), DiffusionSpeed(0.01)],
        ),
        // Second-generation digital cellular at national scale. The first GSM
        // call was placed in Finland in July 1991 and the German and Finnish
        // networks opened to subscribers through 1992. Digital is what turns the
        // car telephone into the thing everyone has: eight times the users per
        // cell, and a handset that costs what a household will pay.
        tech(
            "comm_cellular_digital", "Digital Cellular Rollout",
            Communications, Information,
            &["comm_cellular_analogue", "core_digital_cellular"], 80.0, 1992,
            &[Productivity(0.00011), DiffusionSpeed(0.02)],
        ),
        // Civil positioning as an industry rather than a military programme. GPS
        // reached initial operational capability with 24 satellites in December
        // 1993; full capability followed in April 1995. What it changes is not
        // navigation, which mostly worked, but surveying, fleet dispatch,
        // container yards and artillery — every job where knowing exactly where
        // something is was previously a specialist's week of work.
        tech(
            "comm_satellite_navigation", "Civil Satellite Positioning",
            Communications, Information,
            &["core_gnss", "comm_geostationary_comsat"], 90.0, 1993,
            &[Productivity(0.00008), InvestmentEfficiency(0.02), MilitaryEfficiency(0.04)],
        ),
        // The erbium-doped fibre amplifier: light in, brighter light out, with
        // no conversion to electricity in between. The first long-haul amplified
        // systems went in during 1995 and the first trans-Pacific cable using
        // them followed in 1996. It is the single largest cost collapse in the
        // history of transmission, because it deleted the regenerator hut every
        // fifty kilometres — and, more importantly, it amplifies every colour at
        // once, which is what makes the next entry possible at all.
        tech(
            "comm_erbium_amplifier", "Erbium-Doped Fibre Amplifier",
            Communications, Information,
            &["comm_fibre_optic_backbone"], 75.0, 1995,
            &[Productivity(0.00005), CostReduction { domain: Communications, frac: 0.03 }],
        ),
        // Dense wavelength-division multiplexing. Ciena shipped the 16-channel
        // MultiWave 1600 in March 1996 and Sprint announced in June 1996 that it
        // had raised the capacity of its national network sixteen-fold without
        // laying any new glass. From here on the cost of carrying a bit falls
        // faster than demand for bits rises, which is the fact the whole
        // internet economy is built on top of.
        tech(
            "comm_wdm_transport", "Dense Wavelength-Division Multiplexing",
            Communications, Information,
            &["comm_erbium_amplifier"], 95.0, 1996,
            &[Productivity(0.00008), CostReduction { domain: Communications, frac: 0.04 }],
        ),
        // Broadband over the copper pair that is already in the wall. Bell
        // Atlantic sold commercial ADSL in selected US markets from 1997; the
        // mass-market rollouts came with Korea's carriers in 1999 and BT in
        // 2000. It matters more than fibre does for twenty years, because the
        // last mile is the expensive mile and this one is already built.
        tech(
            "comm_dsl_broadband", "Digital Subscriber Line",
            Communications, Information,
            &["comm_digital_switching", "comm_fibre_optic_backbone"], 105.0, 1997,
            &[Productivity(0.00014), DiffusionSpeed(0.02)],
        ),
        // Wireless local-area networking. Apple's AirPort, launched in July 1999
        // on the 802.11b draft, was the first mass-market Wi-Fi product. The
        // network stops being a place you go and becomes a property of the
        // building, and every portable machine bought afterwards assumes it.
        tech(
            "comm_wifi_wlan", "Wireless Local-Area Networking",
            Communications, Information,
            &["core_packet_internetworking", "core_cmos_submicron"], 85.0, 1999,
            &[Productivity(0.00008), DiffusionSpeed(0.02)],
        ),
        // Caching content at the edge instead of hauling it from origin. Akamai
        // began selling FreeFlow in 1999. Unglamorous and structural: it is what
        // lets a service in one country be usable in another without that
        // country having a transit path worth the name, and it is therefore one
        // of the strongest diffusion channels in the tree.
        // (1999 launch not re-verified this session — see the unverified list.)
        tech(
            "comm_content_delivery_network", "Content Delivery Network",
            Communications, Information,
            &["comm_fibre_optic_backbone", "core_packet_internetworking"], 100.0, 1999,
            &[Productivity(0.00006), DiffusionSpeed(0.02), DiffusionEmission(0.03)],
        ),
        // Commercial imagery at the resolution that used to be classified.
        // IKONOS launched on 24 September 1999 and sold one-metre panchromatic
        // imagery to anyone from January 2000. A crop survey, a mine survey, a
        // deforestation audit and an order of battle all become things a
        // government can buy rather than things it must launch for.
        tech(
            "comm_earth_observation_imaging", "Commercial Earth Observation",
            Communications, Information,
            &["comm_geostationary_comsat", "core_cmos_submicron"], 105.0, 1999,
            &[ResourceYield(0.02), Environment(0.04), MilitaryEfficiency(0.03)],
        ),

        // -------------------------------------------------------------------
        // Networked era: the ocean, the household, and data on the handset
        // -------------------------------------------------------------------

        // A world wired rather than a set of countries with cables. FLAG opened
        // in 1997 and SEA-ME-WE 3 — the longest cable ever laid, thirty-nine
        // landing points — was completed in 2000. Capacity between continents
        // stops being rationed, and a national economy's distance from the
        // centre of the world becomes a matter of price rather than of physics.
        // (Completion dates not re-verified this session — see unverified list.)
        tech(
            "comm_transoceanic_cable_grid", "Transoceanic Cable Grid",
            Communications, Networked,
            &["comm_wdm_transport"], 155.0, 2000,
            &[Productivity(0.00010), DiffusionSpeed(0.03), DiffusionEmission(0.03)],
        ),
        // Packet data over the cellular network, billed by the megabyte rather
        // than the minute. BT Cellnet launched the world's first commercial GPRS
        // service on 22 June 2000. Slow enough to be a disappointment and
        // important anyway: it is the month the mobile network stops being a
        // telephone and starts being an access network.
        tech(
            "comm_mobile_packet_data", "Mobile Packet Data",
            Communications, Networked,
            &["comm_cellular_digital", "core_packet_internetworking"], 115.0, 2000,
            &[Productivity(0.00004), DiffusionSpeed(0.01)],
        ),
        // Third-generation mobile. NTT DoCoMo opened FOMA, the first commercial
        // W-CDMA service, on 1 October 2001, and Europe spent roughly a hundred
        // billion euros on the spectrum licences before anyone had a handset to
        // sell. Expensive lesson, real capability: data on the move at a rate
        // that a general population will actually use.
        // (October 2001 launch not re-verified this session — see unverified.)
        tech(
            "comm_third_generation", "Third-Generation Mobile",
            Communications, Networked,
            &["comm_mobile_packet_data"], 145.0, 2001,
            &[Productivity(0.00008), DiffusionSpeed(0.02)],
        ),
        // Glass all the way to the door. NTT East and West launched B FLET'S in
        // 2001 and Japan spent the decade at speeds the rest of the rich world
        // did not reach until the 2010s. It is a capital project rather than a
        // clever idea — trenching, ducts and a regulator willing to let someone
        // earn a return on them — which is why its effect is on what investment
        // buys rather than on productivity alone.
        tech(
            "comm_fibre_to_the_home", "Fibre to the Home",
            Communications, Networked,
            &["comm_dsl_broadband", "comm_wdm_transport"], 180.0, 2001,
            &[Productivity(0.00010), InvestmentEfficiency(0.02)],
        ),
        // A standard satellite the size of a shoebox. Six CubeSats flew as
        // secondary payloads on a Rockot from Plesetsk on 30 June 2003. The
        // technical content is modest; what it changes is who is allowed to fly
        // — a university, a start-up, a small state — because the bus, the
        // deployer and the ride are now catalogue items.
        tech(
            "comm_smallsat_bus", "Standardised Small-Satellite Bus",
            Communications, Networked,
            &["comm_geostationary_comsat", "core_cmos_submicron"], 130.0, 2003,
            &[CostReduction { domain: Communications, frac: 0.04 }, ResearchRate(0.02)],
        ),
        // Correcting the satellite signal against known ground stations until
        // metres become centimetres. The FAA commissioned WAAS for instrument
        // flight on 10 July 2003. Precision positioning is the input a great
        // many other industries quietly need: the tractor that drives itself
        // down the row, the dozer that cuts to grade, the shell that lands where
        // the map said.
        tech(
            "comm_gnss_augmentation", "GNSS Augmentation and Precise Positioning",
            Communications, Networked,
            &["comm_satellite_navigation"], 125.0, 2003,
            &[ResourceYield(0.02), InvestmentEfficiency(0.02), MilitaryEfficiency(0.03)],
        ),
        // Value stored and moved on a telephone account. Safaricom launched
        // M-PESA in Kenya on 6 March 2007 and reached two million customers
        // inside a year. This is the entry in the file that does most for the
        // poorest nation that gets it: a country that never built retail
        // banking acquires one over an existing cellular network, and the
        // savings, the remittance and the small trader's float all arrive with
        // it.
        tech(
            "comm_mobile_money", "Mobile Money Transfer",
            Communications, Networked,
            &["comm_cellular_digital"], 120.0, 2007,
            &[Productivity(0.00010), Stability(0.10), InvestmentEfficiency(0.02)],
        ),
        // Fourth-generation mobile: an all-packet radio network with no circuit
        // left in it. TeliaSonera switched on the world's first commercial LTE
        // service in Stockholm and Oslo on 14 December 2009. This is the point
        // at which the mobile network becomes most people's only network, and
        // for whole countries it becomes the fixed network they never built.
        tech(
            "comm_lte_mobile_broadband", "LTE Mobile Broadband",
            Communications, Networked,
            &["comm_third_generation", "comm_fibre_optic_backbone"], 190.0, 2009,
            &[Productivity(0.00014), DiffusionSpeed(0.02)],
        ),
        // Coherent detection: recover the phase of the light and let a digital
        // signal processor undo what the fibre did to it. Verizon lit the first
        // commercial 100G coherent wave, Paris to Frankfurt, on 14 December
        // 2009. Every cable already in the ground gets several times its
        // capacity back from a change of terminal equipment, which is why the
        // transoceanic grid never had to be re-laid.
        tech(
            "comm_coherent_optical", "Coherent Optical Transmission",
            Communications, Networked,
            &["comm_transoceanic_cable_grid"], 195.0, 2009,
            &[Productivity(0.00008), CostReduction { domain: Communications, frac: 0.04 }],
        ),

        // -------------------------------------------------------------------
        // Platform era: orbit gets cheap and position gets political
        // -------------------------------------------------------------------

        // Satellites talking to each other on laser links instead of waiting to
        // fly over a ground station. ESA and Airbus launched EDRS-A, the first
        // SpaceDataHighway node, on 29 January 2016. It is the difference
        // between imagery that arrives in ninety minutes and imagery that
        // arrives now, and it is what later makes a constellation a network
        // rather than a set of individually useful objects.
        tech(
            "comm_laser_intersatellite_link", "Laser Inter-Satellite Relay",
            Communications, Platform,
            &["comm_smallsat_bus", "comm_geostationary_comsat"], 240.0, 2016,
            &[MilitaryEfficiency(0.04), ResearchRate(0.02)],
        ),
        // A positioning constellation of one's own. The European Commission
        // declared Galileo initial services on 15 December 2016 with fifteen
        // satellites; Russia had rebuilt GLONASS and China finished BeiDou soon
        // after. The engineering is a repeat of 1993; the point is sovereign —
        // a power whose grid, ports and guided weapons run on a timing signal
        // another government can degrade at will is not, in the end, sovereign.
        tech(
            "comm_gnss_sovereign", "Sovereign Navigation Constellation",
            Communications, Platform,
            &["comm_gnss_augmentation"], 275.0, 2016,
            &[MilitaryEfficiency(0.05), Stability(0.10)],
        ),
        // Not landing a booster — flying one again. SpaceX re-flew a recovered
        // first stage with SES-10 on 30 March 2017, the first reflight of an
        // orbital-class rocket. Landing was the demonstration; reflight is the
        // economics, and it is the reason the following decade's satellites are
        // designed around what is cheap to launch rather than what is cheap to
        // build.
        tech(
            "comm_reusable_launch_economics", "Reusable Launch Economics",
            Communications, Platform,
            &["core_reusable_booster", "comm_smallsat_bus"], 300.0, 2017,
            &[
                Productivity(0.00004),
                CostReduction { domain: Communications, frac: 0.06 },
                InvestmentEfficiency(0.02),
            ],
        ),
        // The firms generating the traffic start owning the cable. MAREA —
        // Microsoft, Facebook and Telxius, Virginia Beach to Bilbao — was
        // completed in September 2017 at 160 terabits per second, laid in under
        // two years. Intercontinental capacity stops being a carrier consortium's
        // scarce asset and becomes a private input a handful of firms provision
        // for themselves, which is very good for how fast their services spread
        // and does nothing at all for anyone else's transit bill.
        tech(
            "comm_hyperscale_private_cable", "Hyperscale Private Cable",
            Communications, Platform,
            &["comm_coherent_optical"], 255.0, 2017,
            &[Productivity(0.00005), DiffusionEmission(0.04)],
        ),
        // Fifth-generation mobile. South Korea's three carriers switched on
        // consumer 5G together late on 3 April 2019, two days early, because
        // Verizon looked like going first. The consumer gain is thinner than the
        // marketing claimed; the industrial one is real — dense low-latency
        // radio inside a plant, a port or a mine, with the fibre and the
        // coherent transport behind it doing the actual work.
        tech(
            "comm_fifth_generation", "Fifth-Generation Mobile",
            Communications, Platform,
            &["comm_lte_mobile_broadband", "comm_coherent_optical"], 340.0, 2019,
            &[Productivity(0.00011), DiffusionSpeed(0.02)],
        ),

        // -------------------------------------------------------------------
        // Intelligent era: the constellation is the network
        // -------------------------------------------------------------------

        // Broadband from low orbit, at a latency that makes it a substitute for
        // a wire rather than a consolation for having none. SpaceX opened the
        // Starlink public beta on 27 October 2020 at ninety-nine dollars a month
        // with roughly nine hundred satellites up. Coverage stops depending on
        // whether anyone was ever willing to trench to the valley — and the
        // stability term is negative on purpose, because it is the first
        // consumer network in this file that a ministry of communications
        // cannot switch off, licence, or route through its own building.
        tech(
            "comm_leo_broadband_constellation", "LEO Broadband Constellation",
            Communications, Intelligent,
            &["comm_reusable_launch_economics", "comm_laser_intersatellite_link"], 430.0, 2020,
            &[Productivity(0.00010), DiffusionSpeed(0.04), Stability(-0.10)],
        ),
        // A satellite pretending to be a cell tower, so an ordinary handset with
        // no special hardware connects to it. SpaceX launched the first six
        // direct-to-cell Starlink satellites on 2 January 2024 and exchanged the
        // first text messages six days later. The coverage map stops being a map
        // of where anyone built anything, which is worth a great deal to a large
        // empty country and is one more thing a state cannot take away from a
        // province it is losing.
        tech(
            "comm_direct_to_cell_satellite", "Direct-to-Handset Satellite Service",
            Communications, Intelligent,
            &["comm_leo_broadband_constellation", "comm_fifth_generation"], 480.0, 2024,
            &[Productivity(0.00004), DiffusionSpeed(0.03), Stability(-0.05)],
        ),

        // HISTORY. Deployed 2020-02-25; transcribed 2026-09. Northrop
        // Grumman's Mission Extension Vehicle-1 docked with Intelsat 901 at
        // 07:15 UTC on 25 February 2020: the first docking of two commercial
        // spacecraft, and the first docking with a satellite nobody had
        // designed to be docked with. MEV-1 then flew the pair as a bolted-on
        // engine for five years, MEV-2 did the same for Intelsat 10-02 from
        // April 2021, and the Mission Robotic Vehicle — two seven-degree-of-
        // freedom arms rather than a docking probe — launched on 21 July
        // 2026. A geostationary satellite is a fifteen-year asset whose life
        // ends when the tank does, and that, rather than any failure of the
        // electronics, is why the fleet turns over as fast as it does.
        // Refuelling, moving or repairing one turns a consumable into plant,
        // which is what the investment term is for; the military term is
        // there because an arm that can service an object it did not build
        // can also inspect, grapple and move somebody else's. The
        // prerequisite is the satellite being serviced rather than anything
        // about the servicer, because the honest gate is autonomous
        // rendezvous with an uncooperative target and this file has no entry
        // for that — the smallsat bus is named for the avionics, not the
        // size. INERT BEFORE 2025 BY PRICE, NOT BY YEAR FLOOR: 380 points
        // against a measured Jan-2025 Communications tie price of 180.
        tech(
            "comm_orbital_servicing", "On-Orbit Servicing and Life Extension",
            Communications, Intelligent,
            &["comm_geostationary_comsat", "comm_smallsat_bus"], 380.0, 2020,
            &[
                CostReduction { domain: Communications, frac: 0.03 },
                InvestmentEfficiency(0.02),
                MilitaryEfficiency(0.02),
            ],
        ),
        // HISTORY. Deployed by 2025-09; transcribed 2026-09. Light in air
        // instead of in glass: a double-nested antiresonant nodeless fibre
        // guides the signal down a hollow centre, so it arrives about a third
        // sooner and stops obeying the loss floor of silica that has bounded
        // every entry above this one. Microsoft bought Lumenisity to get it,
        // committed at Ignite in November 2024 to fifteen thousand kilometres
        // across the Azure network, and in September 2025 published 0.091 dB
        // per kilometre measured across 1,200 km — the lowest attenuation
        // ever recorded in an optical fibre, and measured on cable in the
        // ground carrying live traffic rather than on a spool in Southampton.
        // It contracted Corning and Heraeus to manufacture the fibre on 23
        // September 2025 and reported more than 1,280 km live with no field
        // failures by early 2026. The month it first carried revenue traffic
        // is not published, which is the only sense in which this is
        // UNVERIFIED, and it is why the floor is 2025 rather than 2024. Note
        // what this is and is not: one buyer's interconnect between its own
        // data centres, not a public network, so it is priced as a transport
        // cost improvement in the line of the amplifier and the wavelength —
        // and its diffusion term, like the private cable above, is emission
        // rather than speed, because it does nothing whatever for anyone
        // else's transit bill.
        tech(
            "comm_hollow_core_fibre", "Hollow-Core Fibre Transport",
            Communications, Intelligent,
            &["comm_coherent_optical", "comm_hyperscale_private_cable"], 470.0, 2025,
            &[
                Productivity(0.00004),
                CostReduction { domain: Communications, frac: 0.04 },
                DiffusionEmission(0.03),
            ],
        ),
        // HISTORY. Deployed 2017; floor dated 2025-03-19; transcribed 2026-09.
        // READ THE FLOOR AGAINST THIS FILE'S OWN STANDARD FIRST, because it
        // deliberately fails it, and fails it in the safe direction. The
        // operational deployment is eight years older than the year in the
        // argument list. China has run a quantum-key backbone in fibre between
        // Beijing and Shanghai since 2017 — two thousand kilometres through
        // thirty-two trusted nodes, carrying government, bank and insurance
        // traffic — and joined it to the Micius satellite in 2020 to reach
        // 4,600 km, the first space-and-ground quantum network. Micius weighs
        // 640 kg and there is one of it. What is dated here is the cheap
        // version. Jinan-1, a 96 kg microsatellite launched on 27 July 2022 with
        // a 23 kg payload, was shown in Nature on 19 March 2025 running
        // real-time key distribution to portable ground stations of about a
        // hundred kilograms, and relaying a key between Beijing and Stellenbosch
        // — 12,900 km, and the first quantum link into the southern hemisphere.
        // Ten times lighter and forty-five times cheaper than Micius is the
        // whole of the news: it is the same step the shoebox satellite was,
        // applied to the one traffic a state most minds being read. So the 2025
        // floor is read off the demonstration that the cheap version works and
        // not off a first deployment, and it is therefore LATE by up to eight
        // years rather than early — the error withholds this from hands that
        // historically held it, and never grants it to hands that did not. The
        // productivity term is nearly zero on purpose — nobody's output rises
        // because a ministry's cables cannot be recorded today and broken in
        // 2040 — and almost all the value is military and political. INERT
        // BEFORE 2025 BY PRICE AND PREREQUISITE: 520 points against a measured
        // Jan-2025 Communications tie price of 180, behind a 2016 laser-relay
        // entry that almost nobody holds.
        tech(
            "comm_satellite_quantum_key", "Satellite Quantum Key Distribution",
            Communications, Intelligent,
            &["comm_smallsat_bus", "comm_laser_intersatellite_link"], 520.0, 2025,
            &[MilitaryEfficiency(0.04), Stability(0.05), Productivity(0.00002)],
        ),
        // HISTORY. Deployed 2025-03-02; transcribed 2026-09. Intuitive
        // Machines' Odysseus touched down near Malapert A at 23:23:53 UTC on
        // 22 February 2024, the first American soft landing since Apollo 17
        // and the first by a company — and it arrived fast onto a twelve-
        // degree slope with its laser altimeters inhibited, broke a leg and
        // tipped to thirty degrees, which is why the floor is read off the
        // next one and not off that. Firefly's Blue Ghost landed upright in
        // Mare Crisium at 08:34 UTC on 2 March 2025, worked a full lunar day
        // and five hours into the night, and returned ten NASA payloads: the
        // first fully successful commercial landing on another body. Both
        // flew on Falcon 9 under fixed-price task orders rather than as
        // national programmes, and that is the entry. A landing stops being
        // something a state mounts and becomes something it buys, which is
        // why the effects are on research and on the cost of everything else
        // in this domain rather than on output. The prestige term is small
        // because nobody has ever held a parade for a robot. INERT BEFORE
        // 2025 BY PRICE AND PREREQUISITE: 500 points against a measured
        // Jan-2025 Communications tie price of 180, behind the 2017 reflight
        // entry that gates the ride.
        tech(
            "comm_commercial_lunar_lander", "Commercial Robotic Lunar Landing",
            Communications, Intelligent,
            &["comm_reusable_launch_economics", "comm_smallsat_bus"], 500.0, 2025,
            &[
                ResearchRate(0.03),
                CostReduction { domain: Communications, frac: 0.03 },
                Stability(0.05),
            ],
        ),
        // ROADMAP. Not ours: three parties have hardware and money against a
        // date, and the interesting thing is that they disagree about who
        // should own it. The US Space Force awarded four Resilient GPS design
        // agreements — Astranis, Axient, L3Harris, Sierra Space — in September
        // 2024 under the Quick Start authority of section 229 of the FY2024
        // defence act, for up to eight small satellites broadcasting the core
        // GPS signals by 2028. ESA's Celeste pair went up on a Rocket Lab
        // Electron in March 2026 and ESTEC had the first navigation signal on
        // 8 April 2026, dual-band L and S from 510 km. Xona ordered eight more
        // Pulsar satellites from Aerospacelab in January 2026 for launch from
        // late 2026 and service in 2027. The floor is 2029 and not 2028
        // because the only one of the three with a government behind it is
        // also the only one that has not flown anything, and R-GPS's 2028 is
        // a design-phase target rather than a delivery date. What this buys
        // is not accuracy — the medium-orbit constellations already deliver
        // that, and the augmentation entry above sharpened it twenty years
        // ago — but a signal roughly a hundred times stronger arriving from a
        // direction the jammer is not pointed at. That is the entire reason
        // the money appeared, and it is why the terms are military and
        // capital rather than resource.
        tech(
            "comm_leo_navigation_layer", "Low-Orbit Navigation Layer",
            Communications, Intelligent,
            &["comm_gnss_sovereign", "comm_leo_broadband_constellation"], 560.0, 2029,
            &[MilitaryEfficiency(0.05), InvestmentEfficiency(0.02), Stability(0.05)],
        ),

        // -------------------------------------------------------------------
        // Frontier era: SPECULATIVE. Nothing below has been deployed.
        // -------------------------------------------------------------------

        // SPECULATIVE. Sixth-generation mobile: the standards bodies have 6G on
        // a schedule that puts a first specification around 2030, and the
        // pattern of the previous four generations says the first commercial
        // network follows the specification by a year or two rather than
        // preceding it. Extrapolated from the ten-year cadence 1G→5G and from
        // the terrestrial and satellite radio here converging into one access
        // network; the capability set is a guess and is priced as one.
        tech(
            "comm_sixth_generation", "Sixth-Generation Mobile",
            Communications, Frontier,
            &["comm_fifth_generation", "comm_direct_to_cell_satellite"], 700.0, 2030,
            &[Productivity(0.00011), DiffusionSpeed(0.03)],
        ),
        // SPECULATIVE. Full reuse: both stages recovered and re-flown on a short
        // turnaround, taking the cost of a kilogram to orbit down by another
        // order of magnitude. Vehicles intended to do this are flying test
        // articles now; routine operational reuse of an upper stage has not been
        // demonstrated by anyone, and the year here is an extrapolation of the
        // 2017 first-stage reflight, not a schedule. If it does not arrive, the
        // entry simply stays unbuilt.
        tech(
            "comm_full_reuse_heavy_lift", "Fully Reusable Heavy Lift",
            Communications, Frontier,
            &["comm_reusable_launch_economics", "comm_leo_broadband_constellation"], 800.0, 2032,
            &[
                Productivity(0.00006),
                CostReduction { domain: Communications, frac: 0.06 },
                InvestmentEfficiency(0.03),
            ],
        ),
        // ROADMAP. Not ours: a crewed landing on the Moon before 2030 is a
        // standing commitment of the Chinese state, and the milestones under it
        // are observed rather than announced — the Lanyue lander flew a landing
        // and lift-off verification on 6 August 2025, the Long March 10 flew a
        // low-altitude demonstration on 11 February 2026, and the Mengzhou pad
        // abort is complete. The transport half is proven elsewhere: Artemis II
        // carried four people round the Moon between 1 and 11 April 2026, the
        // first crew beyond low orbit since 1972. The floor is 2030 and not
        // 2029 because a programme date is not a fact — see HS2, whose funded
        // and legislated 2033 opening is now, on the Department for Transport's
        // own May 2026 report to Parliament, somewhere between May 2036 and
        // October 2039.
        tech(
            "comm_crewed_lunar_return", "Crewed Lunar Return",
            Communications, Frontier,
            &["comm_reusable_launch_economics"], 780.0, 2030,
            &[
                Productivity(0.00003),
                ResearchRate(0.03),
                InvestmentEfficiency(0.02),
                Stability(0.15),
            ],
        ),
        // ROADMAP. Not ours: the European Commission signed a twelve-year
        // concession for IRIS² with SpaceRISE — SES, Eutelsat and Hispasat —
        // on 16 December 2024, and closed the governance and financing
        // negotiations with an implementation agreement on 6 August 2026. The
        // constellation is 348 satellites, 330 in low orbit and 18 in medium,
        // paid for out of the EU's 2028-2034 budget and out of member states
        // that have put their own money on the table: Poland 656 million
        // euros, Hungary 500 million, Spain between one and a half and two
        // billion. First launches are targeted for 2029 with services
        // "progressively from 2029 onwards". The floor is 2032, and note
        // carefully why. This is the only programme in this file whose
        // published date has moved EARLIER rather than later — the 2024
        // contract said services from the beginning of 2030, and the 2026
        // reinforcement says 2029 while adding sixty-six satellites — and a
        // schedule that accelerates as its scope grows is a schedule to
        // discount harder, not softer. Twenty months passed between signing
        // the concession and agreeing how it would be paid for. What the
        // thing is FOR is the stability term: the commercial constellation
        // above carries a negative one because it is the first consumer
        // network a ministry of communications cannot switch off, licence or
        // route through its own building, and this is a state buying that
        // capability back rather than regulating it back.
        tech(
            "comm_sovereign_broadband_constellation", "Sovereign Broadband Constellation",
            Communications, Frontier,
            &["comm_leo_broadband_constellation", "comm_gnss_sovereign"], 720.0, 2032,
            &[DiffusionSpeed(0.03), Stability(0.10), MilitaryEfficiency(0.03)],
        ),
        // ROADMAP. Not ours, and the date has already slipped twice in
        // writing. Moving hundreds of tonnes of cryogenic methane and oxygen
        // between two docked vehicles is the single unproven step between a
        // cheap ride to low orbit and a cheap ride anywhere else: the lunar
        // lander architecture NASA has bought needs roughly ten tanker
        // flights to fill one depot, and nothing downstream of that works
        // without it. Conventional propellants have been transferred in orbit
        // since 1978; at this scale and at these temperatures they have not.
        // The dates are NASA's own and they are moving. The inspector general
        // records a vehicle-to-vehicle test planned for March 2025 slipping
        // to March 2026; the budget documents put an uncrewed landing demo in
        // Q3 2026 and the crewed flight in June 2027; the GAO's July 2026
        // assessment still lists both propellant tests as 2026 milestones;
        // and through the thirteenth Starship flight in July 2026 the
        // transfer had not been attempted. The floor is 2033, six years past
        // the crewed date NASA is currently publishing, for two reasons. The
        // GAO's own confidence analysis put a thirty per cent chance on the
        // lander alone running past February 2028, and — the larger reason —
        // this entry is not the demonstration. It is the depot flown often
        // enough that the tenth tanker costs what the first did, and one
        // successful transfer would not settle it. It depends on the
        // servicing entry as much as on the vehicle, because autonomous
        // docking in orbit is the part that has already been done once.
        tech(
            "comm_orbital_propellant_transfer", "Orbital Propellant Transfer",
            Communications, Frontier,
            &["comm_full_reuse_heavy_lift", "comm_orbital_servicing"], 850.0, 2033,
            &[
                CostReduction { domain: Communications, frac: 0.06 },
                InvestmentEfficiency(0.03),
                ResearchRate(0.02),
            ],
        ),
        // SPECULATIVE — nothing at this scale exists, and this entry is here
        // because the anchor underneath it is real and dated rather than
        // because the trend line is pretty. KNOWN: Starcloud flew an Nvidia
        // H100 to a 325 km orbit on 2 November 2025 in a 60 kg satellite, ran
        // inference on it against a Gemma model, and trained a small model in
        // orbit that December — a data-centre part working outside the
        // atmosphere, which had not been done. Google contracted Planet to
        // fly two Suncatcher prototypes carrying its own accelerators in
        // early 2027, linked by free-space optics, and published its own
        // estimate that space-based clusters become economically sensible
        // around 2035. ASSUMED: everything between one refrigerator and a
        // cluster — that the optical mesh in this file's laser-relay entry
        // scales to hundreds of terabits between platforms flying in tight
        // formation, that radiators can reject megawatts, and above all that
        // launch falls to where the full-reuse entry above assumes it falls.
        // The case for orbit is power and cooling and not computing:
        // continuous sun in a dawn-dusk orbit and a sky to radiate into,
        // against a terrestrial grid that has become the binding constraint
        // on building anything. FALSIFIABLE, and cheaply: if fully reusable
        // heavy lift does not arrive the arithmetic never closes and this
        // entry simply stays unbuilt. It is the same bet as the entry it
        // depends on, taken a second time and at longer odds. The floor is
        // 2036, one year past Google's own number, and Google's number came
        // from a research blog and not from a schedule. The environment term
        // is deliberately small and is net of putting the mass up there.
        tech(
            "comm_orbital_compute_constellation", "Orbital Compute Constellation",
            Communications, Frontier,
            &["comm_laser_intersatellite_link", "comm_full_reuse_heavy_lift"], 900.0, 2036,
            &[Productivity(0.00006), ResearchRate(0.05), Environment(0.03)],
        ),
    ]
}
