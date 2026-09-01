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
//! naming the first deployment its year floor is read off. Past the present day
//! the comment opens with its bucket: `ROADMAP` where the date is somebody
//! else's published, dated, funded commitment, and `SPECULATIVE` where it is
//! not. `SPECULATIVE` is permitted only in the Frontier era, whose own
//! definition already requires it.
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

        // HISTORY. Deployed 2024-01-31; transcribed 2026-09. Six Magura V5
        // boats sank the Russian missile corvette Ivanovets off Crimea
        // overnight on 31 January 2024. Tsezar Kunikov, a four-thousand-tonne
        // landing ship, followed on 14 February and the patrol ship Sergey
        // Kotov on 5 March: three warships in five weeks, sunk by a
        // belligerent with no navy of its own, and the Black Sea Fleet moved
        // out of Sevastopol.
        // The escalation is the part worth transcribing, because it is what
        // makes this a capability rather than a raid. On 31 December 2024 a
        // Magura carrying R-73 air-to-air missiles shot down an Mi-8 near Cape
        // Tarkhankut, and on 2 May 2025 the V7 variant, on Sidewinder rails,
        // shot down two Su-30s sent to hunt it — the first combat aircraft in
        // history downed by a boat with nobody in it. The floor is 2024 and
        // not 2025 because the thing that mattered was demonstrated in
        // January: a hull costing a fraction of its target sinks the target.
        // What 2025 added is that the hull now defends itself, which is
        // remarkable but is not what the floor is read off. It descends from
        // the one-way attack drone rather than from anything in the submarine
        // chain above, because the argument is identical and only the medium
        // changed: cheap hull, satellite fix, one direction, and a defender
        // whose cheapest available answer costs more than the attack.
        tech(
            "aero_uncrewed_surface_attack_craft", "Uncrewed Surface Attack Craft", Aerospace, Intelligent,
            &["aero_attritable_strike_drone", "aero_gps_guided_munition"], 395.0, 2024,
            &[MilitaryEfficiency(0.06), MilitaryStrength(1.5)],
        ),
        // HISTORY. Deployed 2024-04-29; transcribed 2026-09. Finnair suspended
        // its Helsinki-Tartu service from 29 April to 31 May 2024 after two
        // aircraft turned back for satellite-navigation interference — the
        // first time a scheduled international air service was stopped by
        // peacetime jamming. Estonian air navigation services logged more than
        // six hundred pilot reports that month and said the jammers ran
        // essentially around the clock; Vilnius logged over eight hundred in
        // the last quarter of 2024 against a hundred and twenty-four in the
        // same quarter a year before, and a Ryanair flight diverted to Warsaw
        // in January 2025. EASA and IATA published a joint mitigation plan on
        // 18 June 2025 and EASA and EUROCONTROL an action plan after it, and
        // by July 2026 EASA's thirty-day ranking of affected airspace named
        // Warsaw, Tallinn, Riga, Helsinki, Vilnius, Ankara and Istanbul
        // alongside Tehran, Baghdad, Beirut, Cairo and the Gulf.
        // The floor is 2025 and not 2024 because one route for one month is an
        // incident; a year of continuous denial across two theatres, answered
        // by regulators rather than by air forces, is the deployment.
        // This is the file's own argument turned back on itself. The tree runs
        // through precision, and precision after 1999 means a satellite fix; a
        // state that can deny the fix over a theatre has disarmed a great deal
        // of somebody else's inventory without destroying any of it, at a cost
        // per hour that is not comparable to anything it degrades. Note what
        // is NOT in this file: the answer. Quantum and magnetic-anomaly
        // navigation are in DARPA-funded flight trials and on an X-37B, and
        // none of it is fielded, so none of it is here. MilitaryStrength,
        // small, because a jamming network is masts, transmitters and a
        // spectrum staff — fixed plant that goes on working whether or not
        // this year's budget renews, which is this file's test for that
        // channel.
        tech(
            "aero_gnss_denial", "Theatre-Scale Satellite Navigation Denial", Aerospace, Intelligent,
            &["aero_electronic_attack", "core_gnss"], 415.0, 2025,
            &[MilitaryEfficiency(0.05), MilitaryStrength(1.0)],
        ),
        // HISTORY. First kills April 2025, fielded at scale from 2026;
        // transcribed 2026-09. Wild Hornets showed the Sting interceptor in
        // October 2024; the first widely circulated footage of one killing a
        // Shahed was April 2025 and thermal footage followed in May. Ukraine
        // announced serial production of the Octopus interceptor on 14
        // November 2025, the design handed to three manufacturers with eleven
        // more standing up lines, and on 30 November a Sting brought down a
        // jet-powered Geran-3. Ukraine's National Security and Defence Council
        // reports a hundred thousand interceptors built in the year to 2026,
        // more than twenty firms building them, a mission success rate above
        // sixty per cent, and a cost per Shahed downed more than twenty-five
        // times below a Western interceptor missile.
        // The floor is 2026 because 2025 was demonstrations and 2026 is the
        // year this became the principal means of defending a city rather than
        // a supplement to it.
        // This is the 2024 entry above finishing its own sentence. That entry
        // says the interceptors cannot be afforded; this is the interceptor
        // becoming affordable, and it is the first time anywhere in this file
        // since 1990 that defence got cheaper than attack. Whether it stays
        // cheaper is the open question, and the Geran-3 — jet-powered, faster,
        // higher — is the reason to doubt it.
        // No MilitaryStrength: an interceptor is spent the moment it works,
        // and buys no arsenal that survives the night. What it buys is a
        // capital that is still there in the morning, and that is Stability.
        tech(
            "aero_interceptor_drone", "Attritable Interceptor Drone", Aerospace, Intelligent,
            &["aero_counter_uas_layered", "aero_attritable_strike_drone"], 405.0, 2026,
            &[MilitaryEfficiency(0.05), Stability(0.08)],
        ),
        // HISTORY. Deployed 2025-12-28; transcribed 2026-09. Israel's Ministry
        // of Defence declared Iron Beam operational in September 2025 and
        // Rafael handed the first system to the air force on 28 December 2025:
        // a hundred-kilowatt class ground laser in a container with its own
        // generator, with the laser source built by Elbit, effective to
        // something like seven to ten kilometres against rockets, mortars and
        // drones. It is the first high-energy laser any state has fielded as
        // part of a standing national air defence rather than as a ship's
        // self-defence trial, and the ministry's own line was that the
        // delivery marked the transition from development to serial
        // production. The system was first unveiled in 2014 and took eleven
        // years to get here. Two things stop this being a triumphalist entry,
        // and both belong in it. The reported first combat use on 2 March
        // 2026, against projectiles from Lebanon, is reported and not
        // confirmed. And in August 2026, eight months after delivery, a senior
        // Israeli defence official told Ynet the system was still not in full
        // operational use. The floor is 2026 because the delivery on 28
        // December 2025 is a fact and full operational use is not yet one; the
        // entry sits on the fact and not on the claim.
        // This is not a replacement for the 2014 shipboard laser above but its
        // power class, which is the whole difference between a point-defence
        // curiosity and an air-defence layer. Productivity is small and real:
        // the emitter is a high-power fibre laser, and industrial cutting and
        // welding is the same component with a different aiming problem.
        tech(
            "aero_laser_air_defense", "High-Energy Laser Air Defence", Aerospace, Intelligent,
            &["aero_directed_energy_laser", "aero_counter_uas_layered"], 515.0, 2026,
            &[MilitaryEfficiency(0.05), MilitaryStrength(1.5), Productivity(0.00003)],
        ),
        // HISTORY. Deployed 2025-09-10; transcribed 2026-09. A Falcon 9 put
        // the first twenty-one operational Tranche 1 Transport Layer
        // satellites, built by York Space Systems, into low orbit from
        // Vandenberg on 10 September 2025 — the first operational spacecraft
        // of the US Space Development Agency's Proliferated Warfighter Space
        // Architecture. Tranche 1 is a hundred and fifty-four vehicles: a
        // hundred and twenty-six transport, twenty-eight tracking, four
        // missile-defence demonstrators, carrying Link 16 down from orbit and
        // tracking manoeuvring threats that the handful of geostationary
        // warning satellites were never built to see. The argument is not
        // capability per satellite, which is modest; it is that a
        // constellation of a hundred and fifty cheap satellites has no single
        // node worth shooting at.
        // The deflation is from the same agency. SDA paused Tranche 1 launches
        // after on-orbit checkout of that first plane found things needing
        // software correction, and SDA's Sandhoo said in April 2026 that
        // launches would resume in May or June; SDA's own date for initial
        // warfighting capability through the architecture is 2027, not 2025.
        // The floor is 2026 and not 2025 because twenty-one satellites out of
        // a hundred and fifty-four is one orbital plane, not a constellation,
        // and the entry should not be cheaper than the thing. It hangs off the
        // reusable booster rather than the reconnaissance satellite, because
        // what changed is not that anyone learned to build small satellites.
        // It is that putting a hundred and fifty of them up stopped being the
        // expensive part, and that is the 2016 entry's doing.
        tech(
            "aero_proliferated_military_constellation", "Proliferated Military Satellite Constellation", Aerospace, Intelligent,
            &["aero_network_centric_warfare", "core_reusable_booster"], 545.0, 2026,
            &[MilitaryEfficiency(0.05), MilitaryStrength(1.5), Productivity(0.00005)],
        ),
        // ROADMAP. Not ours, and the anchor is a vehicle under contract rather
        // than anybody's remark. The B-21 first flew on 10 November 2023. Two
        // aircraft are flying at Edwards, built on production tooling so they
        // can be converted to combat configuration, and one of those two is the
        // aircraft that goes to Ellsworth Air Force Base; the Air Force has
        // agreed a twenty-five per cent increase in production capacity with
        // Northrop Grumman and wants at least a hundred at an eventual seven a
        // year. The service's date for the first operational aircraft on the
        // ramp at Ellsworth is 2027, first given in February 2026 and held
        // since. Gen. Dale White restated it at Life Cycle Industry Days in
        // Dayton on 30 July 2026, which corroborates the service's date and is
        // not what this entry is anchored on — an officer's remark is no more
        // admissible here than a chief executive's, and the tooling and the
        // contract are.
        // The floor is 2029 and not 2027, and the discount is arithmetic rather
        // than mood. The service's own date is three years and two months from
        // that first flight; this file's own B-2 entry puts that aircraft's
        // initial operational capability at 1 April 1997, seven years and eight
        // months after its first flight in July 1989. A floor of 2029 is five
        // years and two months, still a third faster than the only comparable
        // aeroplane anyone has built, and it may still be optimistic. A
        // programme date is not a fact even when the programme is early.
        // MilitaryStrength rather than efficiency, because a bomber force is
        // exactly what this file reserves that channel for: it exists whether or
        // not next year's budget renews.
        tech(
            "aero_penetrating_bomber", "Penetrating Strike Bomber", Aerospace, Intelligent,
            &["aero_flying_wing_stealth", "aero_stealth_multirole"], 585.0, 2029,
            &[MilitaryStrength(2.0), MilitaryEfficiency(0.04)],
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
        // ROADMAP. Not ours: the Missile Defense Agency's Glide Phase
        // Interceptor is under contract to Northrop Grumman as prime, took a
        // $475 million acceleration out of the 2025 reconciliation act, and as
        // of April 2026 is working to a preliminary design review in 2028 and
        // a delivery in 2031. It fires from the Aegis vertical launch cells
        // that are already at sea, which is the single best reason to believe
        // any part of it.
        // The floor is 2035 and not 2031, for two reasons both on the record.
        // A delivery date is a hardware date and not a fielding date — the
        // entry above on midcourse defence took from the first silo
        // emplacement on 22 July 2004 to a system whose effectiveness is still
        // argued about twenty years later. And this programme's schedule moves
        // with an appropriation rather than with engineering: Defense News
        // reported reduced funding slowing it in May 2025 and a reconciliation
        // bill restoring it eleven months afterwards, which is a date that can
        // be un-set by the next Congress as easily as this one set it. Four
        // years is the discount, and it is not generous.
        // A glide vehicle manoeuvres in the upper atmosphere precisely in
        // order to be somewhere the midcourse interceptor is not. This is the
        // answer to that, and it is the first entry in this file that exists
        // to cancel another entry in this file rather than to extend one.
        // Stability, positive, on exactly the argument the 2004 entry makes:
        // what a defence changes first is not what survives an attack but how
        // a small nuclear state reasons about its own deterrent.
        tech(
            "aero_hypersonic_interceptor", "Hypersonic Glide-Phase Interceptor", Aerospace, Frontier,
            &["aero_hypersonic_glide_vehicle", "aero_midcourse_defense"], 725.0, 2035,
            &[MilitaryStrength(2.0), MilitaryEfficiency(0.03), Stability(0.12)],
        ),
        // SPECULATIVE — no state has ever fielded one, and one state has
        // already cancelled the same idea once.
        // KNOWN: the Space Force awarded other-transaction agreements worth up
        // to $3.2 billion across twenty contracts to twelve companies —
        // Lockheed Martin, Northrop Grumman, Raytheon and General Dynamics
        // alongside SpaceX, Anduril, True Anomaly, Turion and others — to
        // build and demonstrate space-based interceptor prototypes for Golden
        // Dome. In August 2026 Gen. Michael Guetlein said all twelve had
        // passed the first of four milestones, with interceptor flight tests
        // from 2027, an initial capability demonstration in 2028 and actual
        // intercept demonstrations in June 2029.
        // ASSUMED: everything after the demonstration. In the same remarks
        // Guetlein said the department has placed no operational production
        // request and may not pursue large interceptor constellations at all
        // if they cannot be made affordable. That sentence is why this entry
        // says SPECULATIVE and not ROADMAP: there is a funded demonstration
        // and there is no funded deployment, and the difference between those
        // two is the whole of this convention. The floor of 2040 is eleven
        // years past the intercept demonstration, on the argument that a
        // constellation dense enough to hold a boost-phase shot over a
        // continent — hundreds of vehicles, replaced on a schedule — is a
        // different programme from a prototype that hits one target on a
        // range.
        // FALSIFIABLE: if the June 2029 demonstration fails, or succeeds at a
        // price per interceptor that makes the constellation arithmetic
        // impossible, the entry simply stays unbuilt. That has already
        // happened to precisely this idea inside this game's own span:
        // Brilliant Pebbles was proposed in 1987, had development contracts
        // let to Martin Marietta and TRW in June 1991 — the first funded
        // ballistic missile defence production since Safeguard — and was
        // stopped by a stop-work order on 1 December 1993. Nothing about the
        // current attempt makes it immune to the same three sentences.
        tech(
            "aero_space_based_interceptor", "Space-Based Missile Interceptor", Aerospace, Frontier,
            &["aero_hypersonic_interceptor", "aero_proliferated_military_constellation"], 905.0, 2040,
            &[MilitaryStrength(2.5), MilitaryEfficiency(0.03), Stability(0.12)],
        ),
    ]
}
