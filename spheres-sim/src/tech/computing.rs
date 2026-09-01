//! Computing & Software.
//!
//! OWNER: the computing domain author. This file is yours alone. Never edit
//! `tech/mod.rs`, and never touch another domain file — eight authors are
//! working in parallel and the merge is a straight concatenation.
//!
//! SCOPE: microprocessors and their architectures, memory and storage,
//! operating systems, databases, programming practice and tooling, the web as
//! software, cryptography and security, machine learning as a discipline.
//!
//! Every id here must begin with `comp_`. Prerequisites may name other `comp_`
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
//! SHAPE. The domain is four chains that cross rather than thirty siblings.
//! Silicon runs microprocessor → superscalar → multi-core → FinFET → tensor
//! accelerator, and each step is the one that made the next affordable. System
//! software runs protected memory → virtualization → cloud → containers.
//! Networking runs the IP stack → the web → encryption → commerce and search →
//! rich clients → mobile. Learning hangs off all three, because the deep
//! learning era needed the silicon, the fleet and the corpus at once, and got
//! none of them before roughly 2012.
//!
//! PROPORTION. The Productivity values here were authored to sum to 0.0020 —
//! two tenths of a percentage point of annual trend growth for the whole of
//! computing, spread over thirty-five years. That is deliberately less than the
//! popular story of the period claims and about what the productivity
//! statistics of the period actually show. The MEASURED sum is now 0.00242:
//! the 2026-09 Intelligent and Frontier shelf added ten entries and took
//! nothing back, because every incumbent in this file has a floor inside the
//! window the golden hashes cover, so reducing any one of them would move a
//! golden. The budget is held by writing less, not by trading.

use super::TechDef;
#[allow(unused_imports)]
use super::{tech, Domain, Effect, Era};

/// The Computing: process nodes, architectures, and the software that rides them.
///
/// Data only — the scoring engine lives in `tech::mod`.
pub fn techs() -> Vec<TechDef> {
    use Domain::*;
    use Effect::*;
    use Era::*;
    vec![
        // ---------------------------------------------------------------
        // Silicon: the substrate everything else in the tree is billed to.
        // ---------------------------------------------------------------

        // The 32-bit microprocessor as an article of commerce rather than a
        // part number. Intel began volume production of the 80386 in June 1986
        // and Compaq shipped the first machine built round it, the Deskpro 386,
        // in September 1986 — the first time a desk had the addressing and the
        // arithmetic of a minicomputer on it. By 1990 it is what a business
        // buys without thinking about it, which is the condition every later
        // entry in this file assumes.
        tech(
            "comp_microprocessor", "Commodity 32-Bit Microprocessor", Computing, Information,
            &["core_cmos_submicron"], 50.0, 1990,
            &[Productivity(0.00017), ResearchRate(0.03), InvestmentEfficiency(0.02)],
        ),
        // Superscalar RISC: issue more than one instruction per cycle and
        // reorder them when the data is not ready yet. IBM announced the
        // RISC System/6000 on 15 February 1990 with the POWER1, the first
        // commercial part to do register renaming and out-of-order issue —
        // techniques that had belonged to mainframes. Everything after this
        // buys speed by finding parallelism rather than by raising the clock.
        tech(
            "comp_risc_superscalar", "Superscalar RISC Processor", Computing, Information,
            &["comp_microprocessor"], 60.0, 1990,
            &[Productivity(0.00003), ResearchRate(0.03)],
        ),
        // Synchronous DRAM. Samsung's KM48SL2000 arrived in 1992 and went to
        // mass production in 1993, the year JEDEC standardised the interface.
        // Clocking the memory to the bus rather than handshaking with it is
        // dull, and it is also the reason the processors above were ever fed.
        tech(
            "comp_synchronous_dram", "Synchronous DRAM", Computing, Information,
            &["comp_microprocessor"], 50.0, 1993,
            &[Productivity(0.00002), CostReduction { domain: Computing, frac: 0.03 }],
        ),
        // NAND flash. Toshiba invented it in 1987 and had product on the market
        // in 1989, but it was the SmartMedia card of 1995 that made non-volatile
        // storage something with no moving parts, no seek time and no power draw
        // at rest. Nothing that has to survive being carried in a pocket is
        // possible before it.
        tech(
            "comp_nand_flash_storage", "NAND Flash Storage", Computing, Information,
            &["comp_microprocessor", "core_cmos_submicron"], 60.0, 1995,
            &[Productivity(0.00003), EnergyEfficiency(0.01)],
        ),

        // ---------------------------------------------------------------
        // System software: what turns a fast part into a machine anyone can
        // be trusted with.
        // ---------------------------------------------------------------

        // Relational storage with a query language over it. Oracle shipped the
        // first commercial RDBMS in 1979 and IBM put DB2 on MVS in 1983; by 1990
        // SQL is a standard and every institution that keeps records keeps them
        // this way. The dull half of the computing revolution, and the half that
        // actually reorganised firms.
        tech(
            "comp_relational_database", "Relational Database Management", Computing, Information,
            &["comp_microprocessor"], 45.0, 1990,
            &[Productivity(0.00008), InvestmentEfficiency(0.02)],
        ),
        // Pre-emptive multitasking with memory protection on commodity hardware:
        // one program's mistake stops being every program's mistake. OS/2 2.0
        // shipped in April 1992 and Windows NT 3.1 on 27 July 1993; the Unixes
        // had had it for a decade on hardware nobody could afford. This is what
        // makes a computer something a clerk can be handed.
        tech(
            "comp_protected_memory_os", "Protected-Memory Operating System", Computing, Information,
            &["comp_microprocessor"], 55.0, 1993,
            &[Productivity(0.00005), InvestmentEfficiency(0.02)],
        ),

        // ---------------------------------------------------------------
        // The network, and then the network as a place of business.
        // ---------------------------------------------------------------

        // TCP/IP stops being a research network's protocol and becomes a
        // component of the operating system. 4.2BSD shipped the reference stack
        // and the sockets interface in August 1983; the commodity machines got
        // there through third-party stacks in the early nineties and then for
        // free, when Windows 95 shipped TCP/IP and Winsock in the box in August
        // 1995 and killed the market for selling it. A nation with this can put
        // any two of its machines in touch for the cost of the wire.
        tech(
            "comp_internet_protocol", "Internet Protocol Stack", Computing, Information,
            &["core_packet_internetworking", "comp_protected_memory_os"], 60.0, 1993,
            &[Productivity(0.00009), DiffusionSpeed(0.05), DiffusionEmission(0.05)],
        ),
        // The web. CERN placed the source in the public domain on 30 April 1993
        // and declined to charge for it, which is the decision that mattered
        // more than the invention: an open standard nobody owned meant every
        // later entry could assume it. By the end of that year there were more
        // than five hundred servers.
        tech(
            "comp_world_wide_web", "World Wide Web", Computing, Information,
            &["comp_internet_protocol"], 70.0, 1993,
            &[
                Productivity(0.00014),
                ResearchRate(0.04),
                DiffusionSpeed(0.05),
                DiffusionEmission(0.08),
            ],
        ),
        // Open-source development as a method rather than a hobby: publish the
        // source, take the patches, and let the maintenance bill fall on
        // everyone who benefits. Linux 1.0 was released on 14 March 1994 and
        // Apache began the following year. Its effect in this model is mostly on
        // other nations — it is the single largest reason knowledge leaks.
        tech(
            "comp_open_source_development", "Open-Source Development", Computing, Information,
            &["comp_protected_memory_os", "comp_internet_protocol"], 55.0, 1994,
            &[
                Productivity(0.00003),
                CostReduction { domain: Computing, frac: 0.06 },
                DiffusionSpeed(0.04),
                DiffusionEmission(0.10),
            ],
        ),
        // Transport-layer encryption. Netscape bundled SSL 2.0 with Navigator
        // 1.1 in early 1995 and submitted it to the IETF that April. Weak by
        // any later standard, and enough: it is the moment a stranger's network
        // becomes a place where a contract can be signed.
        tech(
            "comp_transport_encryption", "Transport-Layer Encryption", Computing, Information,
            &["comp_world_wide_web"], 75.0, 1995,
            &[Productivity(0.00002), Stability(0.15), MilitaryEfficiency(0.02)],
        ),
        // Electronic commerce: a catalogue, an inventory system and a payment
        // that a customer will complete without speaking to anybody. Amazon
        // opened on 16 July 1995 and eBay on 3 September 1995. The productivity
        // is in the retail and wholesale margin, not in the website.
        tech(
            "comp_ecommerce_platform", "Electronic Commerce", Computing, Information,
            &["comp_transport_encryption", "comp_relational_database"], 85.0, 1995,
            &[Productivity(0.00007), InvestmentEfficiency(0.03)],
        ),
        // Search that ranks rather than merely matches. AltaVista put a full-text
        // index of the web up on 15 December 1995; Google was founded in 1998 on
        // the observation that a link is a vote. An index nobody can rank is a
        // library with the catalogue burnt, so this is where the web stops being
        // a curiosity and becomes infrastructure for finding things out.
        tech(
            "comp_web_search", "Web Search Ranking", Computing, Information,
            &["comp_world_wide_web", "comp_relational_database"], 95.0, 1998,
            &[Productivity(0.00008), ResearchRate(0.05), DiffusionSpeed(0.04)],
        ),

        // ---------------------------------------------------------------
        // The fleet: computing sold by the hour instead of the box.
        // ---------------------------------------------------------------

        // Bare-metal virtualization on commodity x86. VMware ESX 1.0 was
        // released on 27 March 2001 and let sixteen machines' worth of work sit
        // on one machine's worth of hardware. Server rooms had been running at
        // ten per cent utilisation because a workload could not be trusted to
        // share; this is what let the capital be used.
        tech(
            "comp_server_virtualization", "Server Virtualization", Computing, Networked,
            &["comp_protected_memory_os", "comp_risc_superscalar"], 110.0, 2001,
            &[Productivity(0.00006), InvestmentEfficiency(0.04)],
        ),
        // Two processors on one die. IBM's POWER4 was announced in October 2001
        // and shipped that year, the first commercial multi-core part; the desks
        // followed in 2005. Clock speed had run into the wall of what a package
        // can dissipate, and after this every further gain in throughput has to
        // be taken as parallelism whether the software wants it or not.
        tech(
            "comp_multicore_processor", "Multi-Core Processor", Computing, Networked,
            &["comp_risc_superscalar", "core_duv_lithography"], 125.0, 2001,
            &[Productivity(0.00007), ResearchRate(0.04), EnergyEfficiency(0.02)],
        ),
        // Web pages that behave like applications. Microsoft shipped
        // XMLHttpRequest in Internet Explorer 5 in 1999 to serve its own mail
        // client; Google used it through 2004 in Gmail and Maps, and the rest of
        // the industry copied it within a year. What it changes is distribution:
        // software stops being installed.
        tech(
            "comp_web_application_platform", "Rich Web Application Platform", Computing, Networked,
            &["comp_world_wide_web", "comp_ecommerce_platform"], 120.0, 2004,
            &[Productivity(0.00006), DiffusionSpeed(0.04)],
        ),
        // Computation over data too large for any one machine, on machines cheap
        // enough to expect to fail. Google described the file system in 2003 and
        // MapReduce in 2004; Hadoop put the same design in the open in 2006.
        // Analysis stops being something done to a sample.
        tech(
            "comp_distributed_data_processing", "Distributed Data Processing", Computing, Networked,
            &["comp_relational_database", "comp_open_source_development"], 145.0, 2004,
            &[
                Productivity(0.00007),
                ResearchRate(0.05),
                CostReduction { domain: Computing, frac: 0.04 },
            ],
        ),
        // The social platform. Facebook dropped its university-only registration
        // on 26 September 2006 and let anyone over thirteen join. It is a modest
        // contributor to output and a large one to how fast a claim travels,
        // which cuts both ways: a state's own account of itself now competes
        // with everyone else's, inside its own borders, in real time.
        tech(
            "comp_social_platform", "Social Networking Platform", Computing, Networked,
            &["comp_web_application_platform"], 115.0, 2006,
            &[Productivity(0.00003), DiffusionEmission(0.08), Stability(-0.35)],
        ),
        // Public cloud. Amazon opened S3 on 14 March 2006 and the EC2 beta on
        // 25 August 2006, and a firm that wanted a hundred servers for a week
        // could have them without a purchase order. The effect is on the cost of
        // trying something, which is why it shows up as investment efficiency
        // rather than as a large productivity number.
        tech(
            "comp_cloud_computing", "Public Cloud Computing", Computing, Networked,
            &["comp_server_virtualization", "comp_distributed_data_processing"], 165.0, 2006,
            &[
                Productivity(0.00011),
                ResearchRate(0.05),
                InvestmentEfficiency(0.05),
                CostReduction { domain: Computing, frac: 0.05 },
            ],
        ),
        // Graphics hardware turned into a general array processor. NVIDIA
        // released CUDA 1.0 on 23 June 2007, which is the point at which a
        // physicist could use a games card without writing his problem as a
        // shader. It buys nothing in an office and a great deal in a laboratory.
        tech(
            "comp_gpgpu_computing", "General-Purpose GPU Computing", Computing, Networked,
            &["comp_multicore_processor", "comp_synchronous_dram"], 150.0, 2007,
            &[Productivity(0.00004), ResearchRate(0.06)],
        ),
        // Mobile computing: a machine with the network, the storage and the
        // interface all in a pocket. The iPhone shipped on 29 June 2007 and its
        // imitators within two years. For most of the world's population this,
        // and not the desktop, is the first computer — which is why it carries
        // more diffusion than productivity.
        tech(
            "comp_mobile_computing", "Mobile Computing", Computing, Networked,
            &[
                "comp_nand_flash_storage",
                "comp_multicore_processor",
                "comp_web_application_platform",
                "core_digital_cellular",
            ],
            180.0, 2007,
            &[Productivity(0.00008), DiffusionSpeed(0.05), DiffusionEmission(0.05)],
        ),
        // The application store: a distribution channel, a payment rail and a
        // review process in one, opened on 10 July 2008. It is the reason the
        // handset became a platform other people could build a business on
        // rather than a telephone with extras.
        tech(
            "comp_mobile_app_ecosystem", "Mobile Application Ecosystem", Computing, Networked,
            &["comp_mobile_computing", "comp_ecommerce_platform"], 155.0, 2008,
            &[Productivity(0.00005), InvestmentEfficiency(0.03)],
        ),

        // ---------------------------------------------------------------
        // Platform era: the network as a weapon, and the silicon that made
        // learning affordable.
        // ---------------------------------------------------------------

        // Software as ordnance. Stuxnet was uncovered in June 2010 after
        // wrecking on the order of a thousand centrifuges at Natanz — the first
        // time code destroyed hardware as an act of state. It buys reach a navy
        // cannot, and it is cheap, which is the whole problem with it.
        tech(
            "comp_offensive_cyber_operations", "Offensive Cyber Operations", Computing, Platform,
            &["comp_internet_protocol", "comp_transport_encryption"], 190.0, 2010,
            &[Productivity(0.00001), MilitaryStrength(4.0), MilitaryEfficiency(0.04)],
        ),
        // The transistor goes vertical. Intel's 22nm tri-gate Ivy Bridge shipped
        // on 23 April 2012, the first high-volume FinFET part; the planar
        // transistor had stopped switching cleanly and this is what kept the
        // industry's cost curve going for another decade.
        tech(
            "comp_finfet_processor", "FinFET Processor", Computing, Platform,
            &["comp_multicore_processor", "core_duv_lithography"], 235.0, 2012,
            &[
                Productivity(0.00006),
                ResearchRate(0.05),
                EnergyEfficiency(0.03),
                CostReduction { domain: Computing, frac: 0.04 },
            ],
        ),
        // Containers and something to schedule them with. Docker was announced
        // in March 2013 and Kubernetes reached 1.0 on 21 July 2015. What it
        // removes is the week between a program working and a program running:
        // deployment stops being an event and becomes a routine.
        tech(
            "comp_container_orchestration", "Container Orchestration", Computing, Platform,
            &["comp_cloud_computing", "comp_open_source_development"], 205.0, 2013,
            &[
                Productivity(0.00005),
                InvestmentEfficiency(0.03),
                CostReduction { domain: Computing, frac: 0.03 },
            ],
        ),
        // The framework that made a neural network something an engineer could
        // write rather than derive. Theano and Torch came first; TensorFlow was
        // released in November 2015 and put automatic differentiation and a GPU
        // backend behind an ordinary programming interface. The method had
        // existed for years and this is what made it usable at a normal salary.
        tech(
            "comp_deep_learning_framework", "Deep-Learning Framework", Computing, Platform,
            &["core_gpu_deep_learning", "comp_gpgpu_computing", "comp_distributed_data_processing"],
            255.0, 2015,
            &[Productivity(0.00007), ResearchRate(0.06)],
        ),
        // Silicon designed for one arithmetic. Google had its first tensor
        // processing unit running in production datacentres in 2015 and
        // disclosed it on 18 May 2016. Once the workload is worth its own chip,
        // the chip is an order of magnitude better at it, and the general-purpose
        // processor stops setting the price of a computation.
        tech(
            "comp_tensor_accelerator", "Tensor Accelerator", Computing, Platform,
            &["comp_finfet_processor", "comp_deep_learning_framework"], 285.0, 2016,
            &[Productivity(0.00005), ResearchRate(0.06), EnergyEfficiency(0.02)],
        ),
        // The transformer. Eight authors at Google posted "Attention Is All You
        // Need" on 12 June 2017; BERT was in Google's production search ranking
        // by October 2019. It is an architecture that scales cleanly with
        // hardware, which is the property that turned a research programme into
        // a capital expenditure.
        tech(
            "comp_transformer_model", "Transformer Model", Computing, Platform,
            &["comp_deep_learning_framework"], 310.0, 2017,
            &[Productivity(0.00007), ResearchRate(0.06), DiffusionSpeed(0.03)],
        ),
        // A programmable quantum processor. Google's 53-qubit Sycamore ran a
        // sampling problem in 200 seconds in October 2019 and claimed a
        // classical machine would need far longer; IBM disputed the margin
        // within days. Nothing useful has been computed on one yet, so the
        // productivity here is nearly nothing and the value is the option.
        tech(
            "comp_quantum_processor", "Programmable Quantum Processor", Computing, Platform,
            &["comp_finfet_processor", "comp_cloud_computing"], 270.0, 2019,
            &[Productivity(0.00001), ResearchRate(0.03)],
        ),

        // ---------------------------------------------------------------
        // Intelligent era.
        // ---------------------------------------------------------------

        // The large language model as a product. OpenAI opened the GPT-3 API on
        // 11 June 2020 and ChatGPT reached the public on 30 November 2022. The
        // productivity is real and narrower than the claims made for it; the
        // stability cost is that the marginal price of a plausible falsehood
        // went to zero in the same year.
        tech(
            "comp_large_language_model", "Large Language Model", Computing, Intelligent,
            &["comp_transformer_model", "comp_tensor_accelerator"], 430.0, 2020,
            &[
                Productivity(0.00013),
                ResearchRate(0.10),
                DiffusionSpeed(0.04),
                Stability(-0.20),
            ],
        ),
        // Learned models turned on the laboratory itself. AlphaFold 2 won CASP14
        // in November 2020 with accuracy the assessors called equivalent to
        // experiment, and the structure database opened on 22 July 2021 with the
        // human proteome in it. This is the entry that makes research cheaper
        // for everyone else, which is why almost all of its weight is there.
        tech(
            "comp_ai_scientific_discovery", "Machine Learning for Discovery", Computing, Intelligent,
            &["comp_deep_learning_framework", "comp_tensor_accelerator"], 400.0, 2021,
            &[Productivity(0.00007), ResearchRate(0.10), Health(0.04)],
        ),
        // Models that call tools instead of only answering. OpenAI shipped
        // function calling in June 2023 and renamed it to a general tool
        // interface later that year; every provider had one within a year. The
        // difference from the model alone is that the output is an action taken
        // in a system of record rather than a paragraph somebody still has to act on.
        tech(
            "comp_ai_agent_tooling", "Autonomous Agent Tooling", Computing, Intelligent,
            &["comp_large_language_model", "comp_container_orchestration"], 520.0, 2023,
            &[Productivity(0.00007), ResearchRate(0.08), InvestmentEfficiency(0.04)],
        ),

        // HISTORY. Deployed 2022-12-11; transcribed 2026-09. Making the
        // commonest class of security defect unrepresentable instead of
        // catching it afterwards. Rust support landed in the mainline Linux
        // kernel with 6.1 on 11 December 2022; Google reported the same month
        // that Android 13 was the first release in which a majority of new code
        // was written in a memory-safe language, with 21% of new native code in
        // Rust, and by September 2024 reported memory-safety defects down from
        // 76% of Android's vulnerabilities in 2019 to 24% in 2024 — 223 reports
        // a year to under fifty. The first Rust drivers were accepted in
        // December 2023 and the kernel maintainers made Rust permanent for new
        // code at the 2025 summit. What it does not do is fix the installed
        // base: the C that is already deployed is still deployed, and will be
        // for decades, which is why this is a slow pull on stability rather
        // than a step change. It hangs off open-source development because both
        // artefacts that carry the claim are open projects and because the
        // compiler that enforces it is one, which is also why the effect
        // arrives everywhere at once instead of being anybody's advantage.
        tech(
            "comp_memory_safe_systems", "Memory-Safe Systems Programming", Computing, Intelligent,
            &["comp_protected_memory_os", "comp_open_source_development"], 420.0, 2022,
            &[Productivity(0.00003), Stability(0.06), MilitaryEfficiency(0.02)],
        ),
        // HISTORY. Deployed 2024-03; transcribed 2026-09. Replacing the key
        // exchange this whole file rests on, before the machine that breaks it
        // exists. NIST published FIPS 203, 204 and 205 on 13 August 2024,
        // ending an eight-year competition; Chrome had already turned hybrid
        // post-quantum key agreement on by default on the desktop in March 2024
        // and moved to the standardised ML-KEM in Chrome 131 that November,
        // Firefox and Chrome for Android followed in November 2024, and Apple's
        // platforms in October 2025. Cloudflare measured the majority of
        // human-initiated HTTPS traffic on its network as post-quantum in the
        // last week of October 2025. Note how much of it is not done: the same
        // measurement put support at the origin at 3.7% and found no public
        // post-quantum certificates in use at all, so the key exchange has
        // moved and the signatures have not. OMB's July 2024 report to Congress
        // priced the migration of federal civilian systems alone at $7.1bn
        // across 2025-2035 and warned in the same document that its own figure
        // was a rough order of magnitude. The floor is 2024 because that is the
        // year a browser most of the world uses started doing this without
        // being asked. It hangs off the quantum processor and not off the
        // encryption entry alone because the threat is the machine and not the
        // mathematics — and that prerequisite is also what keeps this out of
        // any candidate set in the years the golden hashes cover.
        tech(
            "comp_post_quantum_cryptography", "Post-Quantum Cryptography", Computing, Intelligent,
            &["comp_transport_encryption", "comp_quantum_processor"], 465.0, 2024,
            &[Productivity(0.00002), Stability(0.08), MilitaryEfficiency(0.03)],
        ),
        // HISTORY. Deployed 2024-09; transcribed 2026-09. Memory sold by the
        // stack rather than by the module. SK hynix began volume production of
        // the first twelve-layer HBM3E in September 2024; JEDEC published the
        // HBM4 standard as JESD270-4 in April 2025, SK hynix had its HBM4
        // production line in place that September, and HBM4 went to mass
        // production in February 2026 against a part it was already late for.
        // The stacking itself is a packaging problem and belongs to the
        // materials file, which already claims it; what belongs here is what
        // the bandwidth is for. A trained model is a matrix that must be read
        // once per token, so the accelerator four entries above has been
        // starved rather than slow since roughly 2016, and this is the entry
        // that feeds it. The cost reduction is on a unit of computation and not
        // on a part — HBM is the most expensive memory anyone has ever sold,
        // and it is also the reason a run that would have needed ten machines
        // needs one.
        tech(
            "comp_high_bandwidth_memory", "High-Bandwidth Memory Systems", Computing, Intelligent,
            &["comp_synchronous_dram", "comp_tensor_accelerator"], 480.0, 2024,
            &[
                Productivity(0.00004),
                ResearchRate(0.05),
                CostReduction { domain: Computing, frac: 0.03 },
            ],
        ),
        // HISTORY. Deployed 2024-06-18; transcribed 2026-09. Inference that
        // happens in the hand rather than in somebody else's datacentre.
        // Microsoft set a floor of 40 TOPS of neural processing unit for the
        // Copilot+ class of machine and the first of them, on Qualcomm's
        // Snapdragon X, shipped on 18 June 2024; Apple put an on-device
        // foundation model in front of iPhone owners with iOS 18.1 on 28
        // October 2024. Almost all of the weight here is diffusion and not
        // output: a nation with no datacentre and an intermittent link now runs
        // the same class of model as one with both, and that is the mechanism
        // by which this capability reaches places that could never have paid
        // for the training of it. The energy term is the smallest in this file
        // and it is real — a query answered in the handset is a query that did
        // not cross a network and did not spin a rack — and it is a rounding
        // error beside what the model cost somebody else to make.
        tech(
            "comp_on_device_inference", "On-Device Neural Inference", Computing, Intelligent,
            &["comp_mobile_computing", "comp_large_language_model"], 395.0, 2024,
            &[Productivity(0.00004), EnergyEfficiency(0.01), DiffusionSpeed(0.04)],
        ),
        // HISTORY. Deployed 2024-09-12; transcribed 2026-09. Spending compute
        // at answer time rather than only at training time. OpenAI put
        // o1-preview in front of users on 12 September 2024 and o1 on 5
        // December; DeepSeek released R1 under an open licence on 20 January
        // 2025, 671B parameters with 37B active, reasoning at the level of the
        // closed models it was compared against. Mechanically this is not the
        // scale entry above it — the model is no larger and the gain is bought
        // with post-training and with seconds of thought — and the emission
        // term is R1 rather than o1: the near-frontier became a download, which
        // is the largest single leak of capability in this file. INERT BEFORE
        // 2025 BY PREREQUISITE AND NOT BY THE 2024 FLOOR: measured monthly
        // across twelve seeds to January 2025, no nation ever holds the large
        // language model entry this hangs off, so it never enters a candidate
        // set at all in the years the golden hashes cover.
        tech(
            "comp_reasoning_model", "Inference-Time Reasoning Model", Computing, Intelligent,
            &["comp_large_language_model", "comp_tensor_accelerator"], 545.0, 2024,
            &[Productivity(0.00008), ResearchRate(0.10), DiffusionEmission(0.08)],
        ),
        // HISTORY. Deployed 2025-02; transcribed 2026-09. The rack becomes the
        // unit of computation. NVIDIA's GB200 NVL72 puts seventy-two
        // accelerators and thirty-six processors into one liquid-cooled 120 kW
        // coherent domain; the first systems reached hyperscalers in December
        // 2024 and CoreWeave and HPE had them running at scale in February
        // 2025. What changes is not the part but the reach of a memory
        // reference: a model that no longer fits a machine still fits a rack.
        tech(
            "comp_rack_scale_accelerator", "Rack-Scale Accelerated Computing", Computing, Intelligent,
            &["comp_tensor_accelerator", "comp_cloud_computing"], 500.0, 2025,
            &[
                Productivity(0.00005),
                ResearchRate(0.08),
                CostReduction { domain: Computing, frac: 0.04 },
            ],
        ),
        // HISTORY. Deployed 2024-11; transcribed 2026-09. Finding the flaw is
        // the expensive half of an intrusion, and it stopped being done by
        // hand. Google's Big Sleep agent found its first real vulnerability, a
        // stack buffer underflow in SQLite, in November 2024 and before
        // release; in July 2025 it caught CVE-2025-6965 in the same library
        // while an actor was preparing to use it. DARPA closed its two-year AI
        // Cyber Challenge at DEF CON on 8-9 August 2025: seven autonomous
        // systems, 143 hours with nobody at a keyboard, 54 of 63 planted
        // vulnerabilities found and 43 patched, plus 18 real ones nobody had
        // planted and 11 patches for those. Then the same capability pointed
        // the other way, dated and catalogued rather than merely claimed —
        // Anthropic reported on 14 November 2025 that an actor it tracks as
        // GTG-1002 had run a largely autonomous espionage campaign that
        // September against roughly thirty organisations and breached a
        // handful; MITRE ATT&CK carries it as campaign C0062. The strength
        // number is deliberately half what the Stuxnet entry above carries:
        // this multiplies a capability a state already has rather than
        // conferring a new one. The emission term is DARPA's doing and not an
        // accident — every finalist system had to be released under an
        // OSI-approved licence, so the state of the art in this became a
        // download in the same month it was demonstrated.
        tech(
            "comp_machine_vulnerability_research", "Machine Vulnerability Research", Computing, Intelligent,
            &["comp_offensive_cyber_operations", "comp_ai_agent_tooling"], 555.0, 2025,
            &[
                Productivity(0.00001),
                MilitaryStrength(2.0),
                MilitaryEfficiency(0.04),
                DiffusionEmission(0.04),
            ],
        ),
        // ROADMAP. Not ours: OpenAI, Oracle, Crusoe and SoftBank publish site
        // capacities and dates for the Stargate campuses, and Epoch AI's
        // site-by-site survey checks each of them against what is actually
        // built. Abilene, Texas is furthest along — about 0.3 GW energised
        // against a stated 1.2 GW by Q4 2026, four of eight buildings running
        // Blackwell parts, powered by a mix of on-site gas and the Texas grid.
        // Five further sites — Shackelford, Doña Ana, Milam, Port Washington,
        // Saline Township — carry stated Q4 2028 dates, are between them at
        // zero today, and three of them are to run on natural-gas microgrids of
        // their own. The floor is 2028 and not 2026 because this programme has
        // already reversed one of its own published numbers: the Abilene
        // expansion from 1.2 GW to 2.1 GW was announced and then cancelled,
        // and the capacity redirected. What a campus of this size buys is the
        // ability to train something nobody else can, which is research rate.
        // It does not buy investment efficiency — this is the least efficient
        // capital in the file, and the entry says so by omitting the term. The
        // environment number is negative, which only one other entry in this
        // tree is, and it is earned: this is the first technology here whose
        // artefact is a power station with computers attached to it.
        tech(
            "comp_gigawatt_compute_campus", "Gigawatt-Scale Compute Campus", Computing, Intelligent,
            &["comp_cloud_computing", "comp_tensor_accelerator"], 590.0, 2028,
            &[Productivity(0.00005), ResearchRate(0.06), Environment(-0.02)],
        ),

        // ---------------------------------------------------------------
        // Frontier.
        // ---------------------------------------------------------------

        // ROADMAP. Not ours: IBM published a dated path to a fault-tolerant
        // machine on 10 June 2025 — Starling, 200 logical qubits running 100
        // million operations, to be built in the IBM Quantum Data Center at
        // Poughkeepsie and delivered to clients in 2029 — and the milestones
        // under it are being met rather than merely restated. Loon, the chip
        // carrying the longer-range couplers the qLDPC code needs, was unveiled
        // in November 2025 on schedule. The floor is 2031 and not 2029 for two
        // reasons written on the same roadmap. IBM's earlier plan put a
        // 4,158-qubit Kookaburra in 2025; the June 2025 revision put a
        // 1,386-qubit Kookaburra in 2026. And Condor, the 1,121-qubit part that
        // closed the 2020 roadmap on time in December 2023, was never made
        // available to anybody — a programme that builds its hardware and then
        // does not ship it has delivered a milestone and not a machine. Note
        // what this does NOT buy: two hundred logical qubits will not factor
        // anything, so there is no military term here and the one on the entry
        // below stands where it is. The classical prerequisite is not
        // decoration — the syndrome decoder is an ordinary computer that has to
        // keep up in real time, and nobody owns one of these outright; they are
        // reached over somebody else's network, like everything else in this era.
        tech(
            "comp_error_corrected_qubit", "Error-Corrected Logical Qubit", Computing, Frontier,
            &["comp_quantum_processor", "comp_cloud_computing"], 660.0, 2031,
            &[Productivity(0.00002), ResearchRate(0.06)],
        ),
        // SPECULATIVE — no such machine exists. The anchor is real: Google's
        // Willow chip demonstrated surface-code error correction below threshold
        // in December 2024, so logical error falls as the code grows rather than
        // rising. Everything past that is extrapolation. A machine with enough
        // logical qubits to run Shor's algorithm at scale would break the public
        // key cryptography this file's own encryption entry rests on, and would
        // make simulating a molecule a calculation rather than an experiment.
        // The floor is set at 2035 on the industry's own million-qubit roadmaps,
        // which have slipped before and may again.
        tech(
            "comp_fault_tolerant_quantum", "Fault-Tolerant Quantum Computing", Computing, Frontier,
            &["comp_quantum_processor", "comp_ai_scientific_discovery"], 850.0, 2035,
            &[Productivity(0.00003), ResearchRate(0.10), MilitaryEfficiency(0.03)],
        ),
        // SPECULATIVE — this is a claim about 2042 and there is no artefact.
        // KNOWN: machines already write and land repairs unattended, and the
        // strongest measurement of it is a refereed public one rather than a
        // vendor's. At DARPA's AI Cyber Challenge final on 8-9 August 2025,
        // seven systems ran 143 hours with nobody at a keyboard and produced 43
        // accepted patches for planted defects and 11 for real ones. On the
        // SWE-bench Verified leaderboard, the share of real repository issues
        // closed with no person in the loop went from a couple of per cent in
        // late 2023 to above ninety by 2026. ASSUMED: that this extends from a
        // patch to a codebase — that a system of record can be specified,
        // built, changed and run for years with nobody who has read it. Nothing
        // demonstrated so far survives that sentence. FALSIFIABLE, and cheaply:
        // half the evidence is a leaderboard, leaderboards leak, and
        // contamination in this one was being argued in early 2026. If the
        // maintenance bill lands back on people, this stays unbuilt and the
        // agent-tooling entry above is the whole of the story. The stability
        // term is negative and it is the honest price: a state whose critical
        // software nobody has read is a state that cannot audit itself, which
        // is the same shape of cost the large language model entry already
        // carries and for the same reason.
        tech(
            "comp_machine_authored_software", "Machine-Authored Software", Computing, Frontier,
            &["comp_ai_agent_tooling", "comp_machine_vulnerability_research"], 780.0, 2042,
            &[
                Productivity(0.00008),
                ResearchRate(0.04),
                CostReduction { domain: Computing, frac: 0.05 },
                Stability(-0.05),
            ],
        ),
    ]
}
