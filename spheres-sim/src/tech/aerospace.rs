//! Aerospace & Military.
//!
//! OWNER: the aerospace domain author. This file is yours alone. Never edit
//! `tech/mod.rs`, and never touch another domain file — eight authors are
//! working in parallel and the merge is a straight concatenation.
//!
//! SCOPE: combat aircraft and their engines, stealth, precision munitions,
//! ballistic and cruise missiles and the defences against them, naval and
//! submarine systems, armour, sensors and electronic warfare, uncrewed systems,
//! military space, nuclear weapons engineering and its verification.
//!
//! Every id here must begin with `aero_`. Prerequisites may name other `aero_`
//! ids in this file, or any `core_` id from the foundation set in `tech/mod.rs`.
//! Nothing else — you cannot see the other domains and they cannot see you.
//!
//! Every entry is a real technology with a real history, and carries a comment
//! naming the first deployment its year floor is read off. Anything past the
//! present day is speculative and must say so in the comment.
//!
//! ---
//!
//! The shape of this domain is one argument made four times. A weapon that hits
//! what it is aimed at is worth more than a hundred that do not, so the whole
//! tree runs through precision: first the seeker, then the satellite fix, then
//! the loitering airframe that carries its own seeker and waits. A sensor that
//! sees further is worth more than armour, so the tree runs through radar:
//! mechanical scan, electronic scan, the jammer that blinds it, the laser that
//! is only a very well-aimed sensor. A picture is worth nothing until it is on
//! somebody's screen, so the tree runs through the data link. And an aircraft
//! nobody has to bring home changes what may be risked, so the tree runs from
//! the reconnaissance drone to the cheap one built by the thousand.
//!
//! Almost none of this is free. `MilitaryEfficiency` is where most of the
//! spending goes — it is a multiplier on what a defence budget buys, which is
//! exactly what these technologies are. `MilitaryStrength` is reserved for the
//! arsenal that exists whether or not the budget is renewed: hulls, silos,
//! bombers. `Productivity` is thin and deliberately so; the civilian dividend
//! from military research is real but it is a leak, not a purpose, and only the
//! entries with an obvious commercial twin — radar, satellite imagery, the
//! network, the small drone, machine vision — carry much of it.

use super::TechDef;
#[allow(unused_imports)]
use super::{tech, Domain, Effect, Era};

/// The Aerospace: airframes, engines, orbit and what it takes to reach it.
///
/// Data only — the scoring engine lives in `tech::mod`.
pub fn techs() -> Vec<TechDef> {
    use Domain::*;
    use Effect::*;
    use Era::*;
    vec![
        // ------------------------------------------------------------------
        // Information (1990-1999) — the arsenal the Gulf War was fought with.
        // Everything in this block was already in service somewhere in 1990,
        // which is why the floors sit at the bottom of the window. What stops
        // the 1990 United States being absurd is not the floor; it is that a
        // nation starts the game knowing none of it and has to buy it out of
        // its own research, one entry at a time.
        // ------------------------------------------------------------------

        // Semi-active laser guidance. Paveway I went to war over North Vietnam
        // in 1968 and dropped the Thanh Hoa bridge on 13 May 1972 after seven
        // years of unguided attempts had not. That arithmetic — one aircraft,
        // one bomb, one bridge — is the whole century of air power rewritten.
        tech(
            "aero_precision_munitions", "Laser-Guided Munitions", Aerospace, Information,
            &[], 52.0, 1990,
            &[MilitaryEfficiency(0.05), Productivity(0.00005)],
        ),
        // Pulse-Doppler and phased-array radar: the F-15's APG-63 gave a
        // look-down, shoot-down picture from 1976, and the Aegis SPY-1 went to
        // sea in USS Ticonderoga in 1983 tracking a hundred targets at once
        // with no moving parts. Both wait on the signal processing.
        tech(
            "aero_pulse_doppler_radar", "Pulse-Doppler and Phased-Array Radar", Aerospace, Information,
            &["core_cmos_submicron"], 60.0, 1990,
            &[MilitaryEfficiency(0.03), Productivity(0.00010)],
        ),
        // Faceted low-observable shaping. The F-117A reached initial operational
        // capability in October 1983 and flew its first combat over Panama in
        // December 1989. Radar cross-section, not speed or altitude, becomes the
        // thing an air defence has to be built against.
        tech(
            "aero_stealth_shaping", "Low-Observable Airframe Shaping", Aerospace, Information,
            &[], 78.0, 1990,
            &[MilitaryEfficiency(0.05), MilitaryStrength(1.0)],
        ),
        // Acoustic quieting: raft-mounted machinery, anechoic tile and shrouded
        // propulsors. The improved Los Angeles boats from 1988 and the Soviet
        // Akula from 1986 were the point at which a submarine stopped being
        // findable by simply listening for it, and deterrence went back to sea.
        tech(
            "aero_quiet_submarine", "Acoustically Quieted Submarine", Aerospace, Information,
            &[], 72.0, 1990,
            &[MilitaryEfficiency(0.02), MilitaryStrength(1.5)],
        ),
        // Electro-optical reconnaissance satellite. KH-11 KENNEN first launched
        // 19 December 1976 and returned imagery down a radio link instead of in
        // a film bucket, which turned overhead photography from a monthly
        // archive into something a duty officer could ask for.
        tech(
            "aero_recon_satellite", "Electro-Optical Reconnaissance Satellite", Aerospace, Information,
            &["core_cmos_submicron"], 92.0, 1990,
            &[MilitaryEfficiency(0.03), Productivity(0.00010)],
        ),
        // Terrain-contour-matching cruise missile. Tomahawk reached IOC in 1983
        // and opened Desert Storm on 17 January 1991. A defended target could
        // now be struck without putting a crew over it, which changes the
        // political price of the first shot more than the military one.
        tech(
            "aero_cruise_missile", "Long-Range Cruise Missile", Aerospace, Information,
            &["aero_precision_munitions"], 82.0, 1991,
            &[MilitaryEfficiency(0.03), MilitaryStrength(1.5), Productivity(0.00005)],
        ),
        // Theatre ballistic missile defence. Patriot PAC-2 engaged Scuds over
        // Israel and Saudi Arabia in January 1991. The interception record is
        // still argued over; what is not is that a government which believes it
        // has a defence behaves differently from one that knows it has none.
        tech(
            "aero_theater_missile_defense", "Theatre Ballistic Missile Defence", Aerospace, Information,
            &["aero_pulse_doppler_radar"], 88.0, 1991,
            &[MilitaryStrength(1.0), Stability(0.10)],
        ),
        // Airborne early warning and control. The E-3 Sentry began delivery in
        // 1977 and put the radar picture and the man giving the orders in the
        // same aeroplane, above the horizon that had hidden low-level attack
        // since radar was invented.
        tech(
            "aero_airborne_battle_management", "Airborne Early Warning and Control", Aerospace, Information,
            &["aero_pulse_doppler_radar"], 70.0, 1992,
            &[MilitaryEfficiency(0.04)],
        ),
        // Jam-resistant tactical data link. JTIDS terminals went onto USAF E-3s
        // from the mid-1980s and Link 16 was used operationally by the US Navy
        // for the first time in 1994. Every platform now sees what every other
        // platform sees, which is worth more than any of the platforms.
        tech(
            "aero_tactical_datalink", "Jam-Resistant Tactical Data Link", Aerospace, Information,
            &["aero_airborne_battle_management", "core_packet_internetworking"], 96.0, 1994,
            &[MilitaryEfficiency(0.05), Productivity(0.00010)],
        ),
        // Long-endurance reconnaissance UAV. The Predator's first operational
        // deployment was Nomad Vigil, out of Gjader in Albania, from July 1995.
        // Endurance measured in days rather than hours is what makes persistent
        // surveillance a thing a staff can plan around.
        tech(
            "aero_unmanned_aircraft", "Long-Endurance Reconnaissance UAV", Aerospace, Information,
            &["core_gnss", "aero_recon_satellite"], 86.0, 1995,
            &[MilitaryEfficiency(0.04), Productivity(0.00010)],
        ),
        // Radar-absorbent flying wing. The B-2A reached initial operational
        // capability on 1 April 1997: curved low-observable shaping, which needs
        // far more computing than faceting did, on a composite airframe with no
        // vertical surfaces to reflect anything.
        tech(
            "aero_flying_wing_stealth", "Radar-Absorbent Flying Wing", Aerospace, Information,
            &["aero_stealth_shaping", "core_carbon_composites"], 106.0, 1997,
            &[MilitaryEfficiency(0.03), MilitaryStrength(1.5), Productivity(0.00008)],
        ),
        // Satellite-guided bomb. B-2s opened Operation Allied Force on 24 March
        // 1999 with GPS-aided munitions, the direct ancestor of the GBU-31
        // JDAM. A laser bomb needs someone to hold the spot and a clear sky; a
        // satellite bomb needs neither, so precision stops being a special
        // mission and becomes the default way ordnance is dropped.
        tech(
            "aero_gps_guided_munition", "Satellite-Guided Munitions", Aerospace, Information,
            &["aero_precision_munitions", "core_gnss"], 100.0, 1999,
            &[MilitaryEfficiency(0.06), Productivity(0.00004)],
        ),

        // ------------------------------------------------------------------
        // Networked (2000-2009) — the sensors get solid-state and the picture
        // gets shared. This is where the reconnaissance-strike complex stops
        // being a Soviet theoretical term and becomes an American standing
        // capability.
        // ------------------------------------------------------------------

        // Active electronically scanned array. Mitsubishi's J/APG-1 entered
        // service on the F-2 in September 2000 and the APG-63(V)2 went
        // operational on Elmendorf's F-15Cs in December 2000. A thousand
        // solid-state transmit/receive modules instead of one big tube: more
        // reliable, harder to jam, and the same gallium-arsenide module the
        // mobile phone industry was learning to make by the million.
        tech(
            "aero_aesa_radar", "Active Electronically Scanned Array", Aerospace, Networked,
            &["aero_pulse_doppler_radar", "core_duv_lithography"], 150.0, 2000,
            &[MilitaryEfficiency(0.05), ResearchRate(0.02), Productivity(0.00013)],
        ),
        // Armed uncrewed strike aircraft. A Predator carrying Hellfires was over
        // Afghanistan from October 2001 and killed Mohammed Atef on 14 November.
        // The find and the kill collapse into one orbit and one aircraft, and
        // the time between them falls from days to minutes.
        tech(
            "aero_armed_uav", "Armed Uncrewed Strike Aircraft", Aerospace, Networked,
            &["aero_unmanned_aircraft", "aero_precision_munitions"], 128.0, 2001,
            &[MilitaryEfficiency(0.06)],
        ),
        // Networked C4ISR. Blue Force Tracking went into Iraq in March 2003 and
        // for the first time a commander could see where his own units were
        // without asking them. The dividend is civilian too: fleet telematics is
        // the same problem with cheaper radios.
        tech(
            "aero_network_centric_warfare", "Networked C4ISR", Aerospace, Networked,
            &["aero_tactical_datalink", "core_gnss"], 168.0, 2003,
            &[MilitaryEfficiency(0.08), Productivity(0.00016)],
        ),
        // Exo-atmospheric midcourse defence. The first ground-based interceptor
        // was emplaced in its silo at Fort Greely on 22 July 2004. Whether it
        // works against a determined arsenal is disputed; that it changes how a
        // small nuclear state reasons about its own deterrent is not.
        tech(
            "aero_midcourse_defense", "Exo-Atmospheric Midcourse Defence", Aerospace, Networked,
            &["aero_theater_missile_defense", "aero_recon_satellite"], 192.0, 2004,
            &[MilitaryStrength(2.0), Stability(0.15)],
        ),
        // Air-independent propulsion. U-31, lead boat of the Type 212A, was
        // commissioned on 19 October 2005 with Siemens PEM fuel cells: three
        // weeks submerged without snorkelling, which gives a middle power a
        // submarine that can hold a strait without a reactor programme.
        tech(
            "aero_air_independent_submarine", "Air-Independent Propulsion Submarine", Aerospace, Networked,
            &["aero_quiet_submarine"], 136.0, 2005,
            &[MilitaryStrength(1.5), Productivity(0.00006)],
        ),
        // Stealth air superiority. The F-22A reached IOC in December 2005 —
        // low observability, supercruise and an AESA in one airframe, and the
        // first fighter designed on the assumption that it would shoot at things
        // that could not see it.
        tech(
            "aero_stealth_air_superiority", "Stealth Air-Superiority Fighter", Aerospace, Networked,
            &["aero_flying_wing_stealth", "aero_aesa_radar"], 198.0, 2005,
            &[MilitaryEfficiency(0.06), MilitaryStrength(1.5)],
        ),
        // Airborne electronic attack. The EA-18G reached IOC with VAQ-132 in
        // September 2009. Once every weapon depends on a radar or a satellite
        // fix, denying the spectrum is a way of disarming an enemy without
        // destroying anything.
        tech(
            "aero_electronic_attack", "Airborne Electronic Attack", Aerospace, Networked,
            &["aero_aesa_radar", "aero_tactical_datalink"], 174.0, 2009,
            &[MilitaryEfficiency(0.05)],
        ),

        // ------------------------------------------------------------------
        // Platform (2010-2019) — cheap airframes from the consumer supply chain
        // at one end, hypersonics and directed energy at the other, and the
        // gap between a superpower's arsenal and a militia's narrowing at the
        // bottom while widening at the top.
        // ------------------------------------------------------------------

        // Vehicle active protection. Trophy went into service on the Merkava IV
        // in 2009 and made the first recorded hard-kill intercept in combat in
        // March 2011, destroying an RPG round in flight. Armour stops being
        // mass and starts being a radar and a reaction time.
        tech(
            "aero_active_protection_system", "Vehicle Active Protection System", Aerospace, Platform,
            &["aero_pulse_doppler_radar"], 186.0, 2011,
            &[MilitaryEfficiency(0.03), MilitaryStrength(1.0)],
        ),
        // Small rotary-wing UAS. The DJI Phantom shipped in January 2013 and
        // put a stabilised camera on a GPS-holding airframe for a few hundred
        // dollars. Militarily it is a platoon's own reconnaissance; civilly it
        // is survey, inspection and mapping, and the civil dividend here is
        // larger than anything else in this file.
        tech(
            "aero_small_uas", "Small Rotary-Wing UAS", Aerospace, Platform,
            &["aero_unmanned_aircraft", "core_lithium_ion_cell"], 176.0, 2013,
            &[
                MilitaryEfficiency(0.04),
                Productivity(0.00022),
                CostReduction { domain: Aerospace, frac: 0.05 },
            ],
        ),
        // Low-observable uncrewed aircraft. The X-47B made the first arrested
        // landing by a tailless autonomous aircraft on a carrier deck aboard
        // USS George H.W. Bush on 10 July 2013 — stealth and autonomy on the
        // one airframe, and no crew to lose over a defended coast.
        tech(
            "aero_stealth_uav", "Low-Observable Uncrewed Aircraft", Aerospace, Platform,
            &["aero_stealth_shaping", "aero_unmanned_aircraft"], 296.0, 2013,
            &[MilitaryEfficiency(0.04), MilitaryStrength(1.0), Productivity(0.00004)],
        ),
        // Scramjet propulsion. The X-51A WaveRider flew 210 seconds of powered
        // air-breathing hypersonic flight on 1 May 2013, the longest anyone had
        // managed. The engine is half the problem; the thermal structure that
        // survives the flight is the other half, and it is the half that
        // everything else hypersonic inherits.
        tech(
            "aero_scramjet_propulsion", "Scramjet Propulsion", Aerospace, Platform,
            &["aero_cruise_missile", "core_carbon_composites"], 288.0, 2013,
            &[MilitaryEfficiency(0.02), Productivity(0.00006)],
        ),
        // Shipboard directed energy. The 30kW Laser Weapon System was declared
        // operational aboard USS Ponce in the Persian Gulf in December 2014,
        // with the commanding officer authorised to fire it in self-defence.
        // A magazine limited by generator output rather than by rounds is the
        // first answer anyone has had to cheap saturation attack.
        tech(
            "aero_directed_energy_laser", "Shipboard Directed-Energy Weapon", Aerospace, Platform,
            &["aero_aesa_radar", "aero_electronic_attack"], 314.0, 2014,
            &[MilitaryEfficiency(0.05), ResearchRate(0.02), Productivity(0.00008)],
        ),
        // Loitering munition. Azerbaijani Harops flew the first loitering-
        // munition combat missions of any interstate war in the April 2016
        // Four-Day War in Nagorno-Karabakh. A weapon that can wait over the
        // target area collapses the sensor-to-shooter loop into one object.
        tech(
            "aero_loitering_munition", "Loitering Munition", Aerospace, Platform,
            &["aero_armed_uav", "aero_gps_guided_munition"], 226.0, 2016,
            &[MilitaryEfficiency(0.06)],
        ),
        // Stealth multirole strike. The F-35A was declared combat-ready on
        // 2 August 2016. Its point is not the airframe but that it is a sensor
        // node with weapons attached, which is why it costs what it costs and
        // why the software has taken longer than the aeroplane.
        tech(
            "aero_stealth_multirole", "Stealth Multirole Strike Fighter", Aerospace, Platform,
            &["aero_stealth_air_superiority", "aero_network_centric_warfare"], 350.0, 2016,
            &[MilitaryEfficiency(0.07), MilitaryStrength(2.0), Productivity(0.00008)],
        ),
        // Hypersonic glide vehicle. The first Russian regiment armed with
        // Avangard went on combat duty on 27 December 2019. A warhead that
        // manoeuvres in the upper atmosphere is not faster than a ballistic
        // one — it is simply somewhere the midcourse interceptor is not, which
        // is why it exists at all.
        tech(
            "aero_hypersonic_glide_vehicle", "Hypersonic Glide Vehicle", Aerospace, Platform,
            &["aero_scramjet_propulsion", "aero_midcourse_defense"], 356.0, 2019,
            &[MilitaryStrength(2.0), Stability(0.08), Productivity(0.00004)],
        ),

        // ------------------------------------------------------------------
        // Intelligent (2020-2029) — machine vision closes the loop, and the
        // decisive munition of the decade costs twenty thousand dollars and is
        // built on a production line, not in an arsenal.
        // ------------------------------------------------------------------

        // Autonomous target recognition. A UN Panel of Experts reported Kargu-2
        // munitions engaging retreating fighters in Libya in 2020 without an
        // operator in the loop, and Project Maven put machine vision onto
        // operational imagery in the same years. The scarce thing stops being
        // the sensor and becomes the analyst, and this is what replaces him.
        tech(
            "aero_autonomous_targeting", "Autonomous Target Recognition", Aerospace, Intelligent,
            &["aero_loitering_munition", "core_gpu_deep_learning"], 430.0, 2021,
            &[MilitaryEfficiency(0.08), Productivity(0.00018)],
        ),
        // Mass one-way attack drone. Shahed-136 airframes, redesignated
        // Geran-2, were first identified in Ukraine on 13 September 2022. A
        // moped engine, a plywood-and-composite wing and a satellite fix, made
        // by the thousand — the defender's interceptor costs a hundred times
        // more than the thing it is shooting down, and that ratio is the whole
        // argument.
        tech(
            "aero_attritable_strike_drone", "Mass-Produced One-Way Attack Drone", Aerospace, Intelligent,
            &["aero_small_uas", "aero_loitering_munition"], 384.0, 2022,
            &[
                MilitaryEfficiency(0.05),
                MilitaryStrength(1.5),
                Productivity(0.00010),
                CostReduction { domain: Aerospace, frac: 0.06 },
            ],
        ),
        // Layered counter-UAS. On the night of 13-14 April 2024 an Israeli and
        // allied layered defence engaged some three hundred drones and missiles
        // in a single raid, and the cost exchange of doing it that way became a
        // public argument in every defence ministry. Guns, jammers and lasers
        // underneath the interceptors, because the interceptors alone cannot
        // be afforded.
        tech(
            "aero_counter_uas_layered", "Layered Counter-UAS Defence", Aerospace, Intelligent,
            &["aero_directed_energy_laser", "aero_autonomous_targeting"], 462.0, 2024,
            &[MilitaryEfficiency(0.04), MilitaryStrength(1.5), Stability(0.10), Productivity(0.00005)],
        ),

        // ------------------------------------------------------------------
        // Frontier (2030+) — SPECULATIVE. Everything below this line is
        // extrapolation from programmes that exist and dates that have been
        // announced but not met. Nothing here has been deployed.
        // ------------------------------------------------------------------

        // SPECULATIVE. Collaborative combat aircraft: uncrewed wingmen flying
        // with a crewed fighter, cheap enough to lose. The USAF's Increment 1
        // prototypes flew in 2025 and fielding is planned before the end of the
        // decade; a floor of 2032 assumes the usual slip between a first flight
        // and a squadron that can be sent somewhere.
        tech(
            "aero_collaborative_combat_aircraft", "Collaborative Combat Aircraft", Aerospace, Frontier,
            &["aero_stealth_uav", "aero_autonomous_targeting"], 660.0, 2032,
            &[MilitaryEfficiency(0.08), MilitaryStrength(2.0), Productivity(0.00010)],
        ),
        // SPECULATIVE. Sixth-generation air dominance: the crewed centre of a
        // formation that is mostly uncrewed, with adaptive-cycle propulsion and
        // an open mission-systems architecture. The American, European and
        // Japanese programmes all advertise the mid-2030s, and defence
        // programmes of this size have never once been early.
        tech(
            "aero_sixth_gen_air_dominance", "Sixth-Generation Air Dominance", Aerospace, Frontier,
            &["aero_stealth_multirole", "aero_collaborative_combat_aircraft"], 900.0, 2037,
            &[MilitaryEfficiency(0.09), MilitaryStrength(2.5), Productivity(0.00008)],
        ),
    ]
}
