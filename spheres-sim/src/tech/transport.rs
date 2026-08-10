//! Transport.
//!
//! OWNER: the transport domain author. This file is yours alone. Never edit
//! `tech/mod.rs`, and never touch another domain file — eight authors are
//! working in parallel and the merge is a straight concatenation.
//!
//! SCOPE: road vehicles and their powertrains, rail and high-speed rail,
//! shipping, ports and containerisation, civil aviation and air traffic,
//! logistics and freight, traffic management, vehicle autonomy, urban mobility.
//!
//! Every id here must begin with `tran_`. Prerequisites may name other `tran_`
//! ids in this file, or any `core_` id from the foundation set in `tech/mod.rs`.
//! Nothing else — you cannot see the other domains and they cannot see you.
//!
//! Every entry is a real technology with a real history, and carries a comment
//! naming the first deployment its year floor is read off. Anything past the
//! present day is speculative and must say so in the comment.
//!
//! The shape of the domain, since a reader will otherwise see thirty siblings.
//! Transport in 1990 is not waiting on an invention; the box was standardised in
//! the 1960s and the wide-body jet flew in 1969. What it is waiting on is
//! *knowing where everything is*. So the spine of this file is one chain — the
//! box, then a document format for the box, then a radio that says where the box
//! is, then software that plans around the answer, then a retail promise built
//! on the software, then robots inside the building that keeps the promise. A
//! second chain runs through the powertrain: injection timing, then a hybrid,
//! then a battery car, then the chargers that make the battery car a car, then a
//! truck. A third runs through the hull, a fourth through the airframe, and a
//! fifth through autonomy, which is the only one that had to wait for a domain
//! other than its own.
//!
//! Effects are deliberately small. Transport is roughly a tenth of output and a
//! quarter of the oil, so it belongs in `EnergyEfficiency` and
//! `InvestmentEfficiency` more than it belongs in raw `Productivity`: what
//! logistics does is make a dollar of somebody else's plant go further.

use super::TechDef;
#[allow(unused_imports)]
use super::{tech, Domain, Effect, Era};

pub fn techs() -> Vec<TechDef> {
    use Domain::Transport as T;
    use Effect::*;
    use Era::*;
    vec![
        // -------------------------------------------------------------------
        // The box, and everything that had to be built around it
        // -------------------------------------------------------------------

        // Containerisation itself predates the start date by a generation; what
        // was still arriving in 1990 was the land half of it. APL ran the first
        // all-double-stack train from Los Angeles on 30 March 1984, and the well
        // car roughly halved the cost of moving a box inland. Intermodal is the
        // root of this file because everything downstream is an attempt to find
        // out where a particular box has got to.
        tech(
            "tran_container_intermodal", "Double-Stack Intermodal Freight", T, Information,
            &[], 50.0, 1990,
            &[Productivity(0.00022), InvestmentEfficiency(0.02), DiffusionEmission(0.02)],
        ),
        // A hull too wide for the Panama Canal is a bet that the Pacific matters
        // more than the isthmus. APL's C10 class, lead ship President Truman,
        // was delivered in 1988 as the first containership to break the 32.2m
        // canal beam, at 4,500 TEU. Everyone said it was a $100m ship that could
        // not go where ships go.
        tech(
            "tran_post_panamax_ship", "Post-Panamax Hull", T, Information,
            &["tran_container_intermodal"], 60.0, 1990,
            &[Productivity(0.00006), EnergyEfficiency(0.01), ResourceYield(0.01)],
        ),
        // A box that nobody can describe in a message is a box that has to be
        // described on paper by hand, at every border it crosses. UN/EDIFACT was
        // approved as ISO 9735 in 1987 and gave the manifest, the customs entry
        // and the invoice one syntax. Unglamorous, and the precondition for
        // every piece of logistics software written since.
        tech(
            "tran_electronic_freight_documents", "Electronic Freight Documentation", T, Information,
            &["tran_container_intermodal"], 45.0, 1990,
            &[Productivity(0.00009), InvestmentEfficiency(0.01), DiffusionSpeed(0.01)],
        ),
        // Not new in 1990 either: the Shinkansen opened in 1964 and the TGV
        // Sud-Est in 1981. What it does to a nation is take the short-haul air
        // market away from itself and put two cities inside one labour market,
        // which is why the effect is as much environmental as economic.
        tech(
            "tran_high_speed_rail", "High-Speed Passenger Rail", T, Information,
            &[], 95.0, 1990,
            &[Productivity(0.00007), EnergyEfficiency(0.02), Environment(0.03)],
        ),
        // ECT's Delta/Sea-Land terminal at Rotterdam opened on 25 June 1993: the
        // first container terminal in the world to run automated guided vehicles
        // and automated stacking cranes. The AGVs had to be invented for it —
        // the ones on offer were factory machines that had never carried forty
        // tonnes outdoors in salt air.
        tech(
            "tran_port_terminal_automation", "Automated Container Terminal", T, Information,
            &["tran_container_intermodal", "core_cmos_submicron"], 85.0, 1993,
            &[
                Productivity(0.00007),
                InvestmentEfficiency(0.01),
                CostReduction { domain: T, frac: 0.03 },
            ],
        ),
        // Qualcomm's OmniTRACS went commercial in 1988 as a satellite text link
        // to a truck cab, and became a position report once GPS was up. It is
        // the first time a dispatcher could answer "where is it" without
        // telephoning a truck stop, which is the whole of modern freight.
        tech(
            "tran_fleet_telematics", "Fleet Telematics", T, Information,
            &["tran_electronic_freight_documents", "core_gnss"], 90.0, 1996,
            &[Productivity(0.00010), InvestmentEfficiency(0.01), EnergyEfficiency(0.01)],
        ),

        // -------------------------------------------------------------------
        // The powertrain, on the road and in the air
        // -------------------------------------------------------------------

        // The GE90 entered service on a British Airways 777 on 17 November 1995,
        // taking a single engine past 90,000lb of thrust with a composite fan.
        // Two engines instead of four is what made the long thin route pay, and
        // the four-engine wide-body has been dying ever since.
        tech(
            "tran_high_bypass_turbofan", "Large High-Bypass Turbofan", T, Information,
            &["core_carbon_composites"], 100.0, 1995,
            &[Productivity(0.00005), EnergyEfficiency(0.03), Environment(0.02)],
        ),

        // Denso put the ECD-U2 common-rail system on the Hino Rising Ranger in
        // 1995, the first in general sale. Decoupling injection pressure from
        // engine speed is what let the diesel be quiet and clean enough for a
        // passenger car, and Europe spent the next fifteen years buying them.
        tech(
            "tran_common_rail_injection", "Common-Rail Diesel Injection", T, Information,
            &["core_cmos_submicron"], 70.0, 1995,
            &[Productivity(0.00005), EnergyEfficiency(0.03), Environment(0.02)],
        ),
        // The Prius went on sale in Japan on 10 December 1997, the first
        // mass-produced hybrid. The battery was the least of it: what Toyota
        // actually shipped was a power inverter and a control law that could
        // hand torque between two sources several times a second, and that is
        // the part every electric vehicle since has inherited.
        tech(
            "tran_hybrid_electric_drive", "Hybrid Electric Drivetrain", T, Information,
            &["core_cmos_submicron"], 105.0, 1997,
            &[Productivity(0.00004), EnergyEfficiency(0.03), Environment(0.03)],
        ),

        // -------------------------------------------------------------------
        // Networked: the software layer arrives on top of the freight layer
        // -------------------------------------------------------------------

        // Advanced planning and scheduling — i2's Rhythm, Manugistics, and the
        // ERP vendors' answers to them — came of age in the late 1990s, once
        // there was enough electronic freight data to plan against. It is the
        // step that turns a warehouse from a buffer into a decision, and it is
        // the single largest productivity entry in this file.
        tech(
            "tran_supply_chain_management", "Supply-Chain Planning Systems", T, Networked,
            &["tran_electronic_freight_documents", "tran_fleet_telematics",
              "core_packet_internetworking"],
            130.0, 2000,
            &[
                Productivity(0.00020),
                InvestmentEfficiency(0.03),
                CostReduction { domain: T, frac: 0.04 },
            ],
        ),
        // Amazon launched Prime on 2 February 2005: unlimited two-day delivery
        // for a flat $79. The technology is not the website, it is the network
        // of buildings and the routing that has to exist before that promise can
        // be made at all, and every retailer on earth then had to match it.
        tech(
            "tran_parcel_fulfilment_network", "Parcel Fulfilment Network", T, Networked,
            &["tran_supply_chain_management", "core_packet_internetworking"], 165.0, 2005,
            &[Productivity(0.00014), InvestmentEfficiency(0.01), DiffusionSpeed(0.02)],
        ),
        // Wal-Mart required its top 100 suppliers to tag pallets and cases from
        // January 2005. Roughly half of them managed it, which is the honest
        // measure of the technology: item-level visibility turned out to be
        // worth less than the tags cost, and the value that stuck was in the
        // pallet and the shipping container rather than the razor blade.
        tech(
            "tran_rfid_item_visibility", "RFID Consignment Tracking", T, Networked,
            &["tran_supply_chain_management", "core_cmos_submicron"], 120.0, 2005,
            &[Productivity(0.00005), InvestmentEfficiency(0.01), ResourceYield(0.01)],
        ),
        // The Tesla Roadster was delivered from February 2008, the first
        // highway-legal production car on lithium-ion cells — 6,831 of them,
        // laptop cells wired into a pack. What it proved was not range but that
        // the cell chemistry the consumer-electronics industry had already
        // scaled could move a vehicle.
        tech(
            "tran_electric_drivetrain", "Lithium Traction Drivetrain", T, Networked,
            &["tran_hybrid_electric_drive", "core_lithium_ion_cell"], 190.0, 2008,
            &[Productivity(0.00006), EnergyEfficiency(0.02), Environment(0.03)],
        ),

        // -------------------------------------------------------------------
        // Platform: each of the five chains reaches scale
        // -------------------------------------------------------------------

        // The Nissan Leaf was delivered from December 2010, the first affordable
        // mass-market electric car since the 1900s. A sports car for enthusiasts
        // is a demonstration; a hatchback with a warranty and a dealer network
        // is an industry deciding to retool.
        tech(
            "tran_mass_market_ev", "Mass-Market Electric Car", T, Platform,
            &["tran_electric_drivetrain"], 230.0, 2010,
            &[Productivity(0.00005), EnergyEfficiency(0.03), Environment(0.05)],
        ),
        // Beijing–Shanghai opened on 30 June 2011: 1,318km at 350km/h, the
        // largest single construction project in the country's history. One line
        // is a railway; a national network built in a decade is a different
        // thing, and it is the only entry here that plausibly moves stability,
        // because it is what a state builds to be visibly one country.
        tech(
            "tran_dedicated_hsr_network", "National High-Speed Rail Network", T, Platform,
            &["tran_high_speed_rail"], 300.0, 2011,
            &[Productivity(0.00006), EnergyEfficiency(0.02), Environment(0.03), Stability(0.02)],
        ),
        // ANA flew the first revenue 787 from Narita to Hong Kong on 26 October
        // 2011, three years late — the first airliner with a composite primary
        // fuselage rather than composite control surfaces. The delay is the
        // point: nobody had built an aircraft that way and the supply chain had
        // to be rebuilt around it.
        tech(
            "tran_composite_airliner", "Composite-Fuselage Airliner", T, Platform,
            &["tran_high_bypass_turbofan", "core_carbon_composites"], 330.0, 2011,
            &[Productivity(0.00005), EnergyEfficiency(0.02), Environment(0.02)],
        ),
        // Tesla opened six Supercharger stations in California on 24 September
        // 2012, sited so a Model S could cross the state. A car with nowhere to
        // charge is a second car; the network is what makes the battery car a
        // vehicle rather than a commuting appliance, and it had to be built by
        // the manufacturer because no fuel retailer would.
        tech(
            "tran_fast_charging_network", "High-Power Charging Network", T, Platform,
            &["tran_mass_market_ev"], 200.0, 2012,
            &[Productivity(0.00004), EnergyEfficiency(0.02), Environment(0.03)],
        ),
        // UberX launched in July 2012, putting ordinary cars and non-professional
        // drivers behind a phone. A licence to carry passengers had been a scarce
        // asset in every city on earth for a century, and the small negative on
        // stability is the trade that lost it — the taxi medallion holders and
        // the fights they took to city halls.
        tech(
            "tran_ride_hailing_platform", "Ride-Hailing Platform", T, Platform,
            &["core_digital_cellular", "core_gnss", "core_packet_internetworking"], 185.0, 2012,
            &[Productivity(0.00006), InvestmentEfficiency(0.01), Stability(-0.01)],
        ),
        // Mærsk Mc-Kinney Møller entered service in July 2013 at 18,270 TEU,
        // burning a fifth less fuel per box than anything before her. Ships this
        // size only pay if the terminal can turn them round, which is why this
        // one waits on the automated quay rather than on the shipyard.
        tech(
            "tran_ultra_large_containership", "Ultra-Large Containership", T, Platform,
            &["tran_post_panamax_ship", "tran_port_terminal_automation"], 260.0, 2013,
            &[Productivity(0.00007), EnergyEfficiency(0.02), Environment(0.02)],
        ),
        // Amazon bought Kiva Systems in 2012 and had the drives running across
        // the network by 2014, taking the click-to-ship cycle from an hour and a
        // quarter to a quarter of an hour. The insight is that the shelf should
        // walk to the picker; the consequence is that a fulfilment centre is now
        // a machine rather than a building with people in it.
        tech(
            "tran_warehouse_mobile_robots", "Goods-to-Person Warehouse Robotics", T, Platform,
            &["tran_parcel_fulfilment_network"], 245.0, 2014,
            &[
                Productivity(0.00010),
                InvestmentEfficiency(0.02),
                CostReduction { domain: T, frac: 0.03 },
            ],
        ),
        // Tesla pushed software 7.0 with Autosteer on 14 October 2015 — lane
        // centring and traffic-aware cruise on a car already sold, delivered
        // overnight. This is the first entry in the file that had to wait for
        // another domain: the perception stack is convolutional networks on a
        // GPU, and it did not exist before 2012.
        tech(
            "tran_driver_assistance_autonomy", "Driver-Assistance Autonomy", T, Platform,
            &["core_gpu_deep_learning", "tran_fleet_telematics"], 340.0, 2015,
            &[Productivity(0.00005), Health(0.03)],
        ),

        // -------------------------------------------------------------------
        // Intelligent
        // -------------------------------------------------------------------

        // Waymo opened fully driverless Waymo One rides to the public in Phoenix
        // on 8 October 2020 — no safety driver, a real fare, an ordinary
        // customer. Eleven years after the programme started, in one suburb, in
        // good weather, which is the correct scale for the claim.
        tech(
            "tran_robotaxi_service", "Driverless Ride Service", T, Intelligent,
            &["tran_driver_assistance_autonomy", "tran_ride_hailing_platform"], 480.0, 2020,
            &[Productivity(0.00008), Health(0.05), EnergyEfficiency(0.01)],
        ),
        // Tesla delivered the first Semis to PepsiCo on 1 December 2022, five
        // years after the unveiling. Heavy road freight is the hardest thing to
        // electrify and the largest single piece of transport diesel, so the
        // energy effect here is the biggest in the file even though the
        // productivity effect is not.
        tech(
            "tran_battery_electric_truck", "Battery-Electric Heavy Truck", T, Intelligent,
            &["tran_fast_charging_network", "tran_mass_market_ev"], 430.0, 2022,
            &[Productivity(0.00006), EnergyEfficiency(0.04), Environment(0.05)],
        ),
        // Laura Maersk entered service in July 2023 and was named in Copenhagen
        // that September: the first containership running on methanol. Shipping
        // is about 3% of world emissions and had no answer at all until a
        // dual-fuel engine let an owner order a hull before the green fuel to
        // run it existed.
        tech(
            "tran_methanol_dual_fuel_ship", "Dual-Fuel Methanol Ship", T, Intelligent,
            &["tran_ultra_large_containership"], 380.0, 2023,
            &[Productivity(0.00002), EnergyEfficiency(0.01), Environment(0.06)],
        ),

        // -------------------------------------------------------------------
        // Frontier — speculative, past the present day
        // -------------------------------------------------------------------

        // SPECULATIVE. Extrapolated, not observed: driverless trucking confined
        // to instrumented long-haul corridors with human drayage at each end,
        // which is the deployment shape every operator has been converging on
        // since the mid-2020s. Assumes both the perception problem and the
        // battery-weight problem are solved first, which is why it sits on
        // robotaxi and the electric truck rather than on either alone. If it
        // arrives it is the largest single change to freight cost since the box.
        tech(
            "tran_autonomous_freight_corridor", "Autonomous Freight Corridor", T, Frontier,
            &["tran_robotaxi_service", "tran_battery_electric_truck",
              "tran_supply_chain_management"],
            800.0, 2032,
            &[
                Productivity(0.00012),
                InvestmentEfficiency(0.02),
                CostReduction { domain: T, frac: 0.05 },
                Health(0.04),
            ],
        ),
        // SPECULATIVE. Extrapolated, not observed: a battery or hybrid-electric
        // regional airliner in scheduled service. The physics is unforgiving —
        // jet fuel carries some forty times the energy per kilogram of a good
        // cell — so this is confined to short sectors and small airframes for as
        // long as the cell is what it is. Needs the composite airframe for mass
        // and the traction drive for the motor.
        tech(
            "tran_electric_regional_aircraft", "Electric Regional Airliner", T, Frontier,
            &["tran_composite_airliner", "tran_electric_drivetrain"], 900.0, 2035,
            &[Productivity(0.00004), EnergyEfficiency(0.02), Environment(0.04)],
        ),
    ]
}
