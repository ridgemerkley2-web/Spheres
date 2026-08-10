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
//! naming the first deployment its year floor is read off. Anything past the
//! present day is speculative and must say so in the comment.
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
//! PROPORTION. The Productivity values here sum to 0.0020 — two tenths of a
//! percentage point of annual trend growth for the whole of computing, spread
//! over thirty-five years. That is deliberately less than the popular story of
//! the period claims and about what the productivity statistics of the period
//! actually show.

use super::TechDef;
#[allow(unused_imports)]
use super::{tech, Domain, Effect, Era};

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

        // ---------------------------------------------------------------
        // Frontier.
        // ---------------------------------------------------------------

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
    ]
}
