//! Biotech & Medicine.
//!
//! OWNER: the biotech domain author. This file is yours alone. Never edit
//! `tech/mod.rs`, and never touch another domain file — eight authors are
//! working in parallel and the merge is a straight concatenation.
//!
//! SCOPE: pharmaceuticals and how they are found, vaccines, surgery, imaging,
//! diagnostics, genomics and cell therapy as medicine, immunology, public
//! health and its delivery, medical devices, the biology of ageing.
//!
//! Every id here must begin with `bio_`. Prerequisites name other `bio_` ids
//! in this file or `core_` ids from the foundation set in `tech/mod.rs`. The
//! cross-domain dependencies this file genuinely has — silicon, packet
//! networking, lithography, deep learning — are all taken through the
//! foundation anchors rather than through another author's file, so this
//! domain resolves on its own and cannot be broken by anyone else's merge.
//!
//! Every entry is a real technology with a real history, and carries a comment
//! naming the first deployment its year floor is read off. Past the present day
//! the comment opens with its bucket: `ROADMAP` where the date is somebody
//! else's published, dated, funded commitment, and `SPECULATIVE` where it is
//! not. `SPECULATIVE` is permitted only in the Frontier era, whose own
//! definition already requires it.
//!
//! WHAT THIS DOMAIN IS ABOUT. Medicine is the one branch of the tree whose
//! output is measured in people rather than output. Two channels carry most of
//! it: `Health`, which is mortality retreating, and `Fertility`, which is
//! people choosing to have fewer children once they can be confident the ones
//! they have will live. They pull against each other on purpose — the whole
//! domain moves population growth by a fraction of a point a year, which over
//! fifty years is the demographic transition and nothing louder. The
//! productivity that medicine buys is real but modest: a workforce that is not
//! sick, and a laboratory whose instruments got a thousand times cheaper.
//!
//! THE SHAPE. Four trunks, and they converge at the end.
//!   * Recombinant protein → monoclonal antibody → humanised antibody →
//!     biosimilar manufacture, which is the industrialisation of biologics and
//!     is what makes gene therapy and the metabolic drugs affordable at all.
//!   * PCR → automated sequencing → high-throughput sequencing → the
//!     thousand-dollar genome → pathogen genomics, the cost collapse that
//!     turned reading DNA from a national programme into a bench operation.
//!   * Public health delivery: immunisation, oral rehydration, outbreak
//!     surveillance. Cheap, unglamorous, and the largest single block of
//!     mortality this file removes.
//!   * Rational drug design → antiretrovirals → targeted oncology, chemistry
//!     done against a known structure rather than against a screening library.
//!
//! The vectors and the pathogen genomics meet at the mRNA platform; the
//! structure prediction and the editing tools meet past it.

use super::TechDef;
#[allow(unused_imports)]
use super::{tech, Domain, Effect, Era};

/// The Biotechnology: sequencing, drug design, vaccines and the mRNA platform.
///
/// Data only — the scoring engine lives in `tech::mod`.
pub fn techs() -> Vec<TechDef> {
    use Domain::*;
    use Effect::*;
    use Era::*;
    vec![
        // -------------------------------------------------------------------
        // Information era: what the world already had in hand in 1990.
        // -------------------------------------------------------------------

        // Humulin, recombinant human insulin, was approved by the FDA on 30
        // October 1982 — the first drug made by a genetically engineered
        // organism. It is the root of this file because everything downstream
        // that is a protein rather than a small molecule is made this way.
        tech(
            "bio_recombinant_pharma", "Recombinant Protein Pharmaceuticals", Biotech, Information,
            &[], 45.0, 1990,
            &[Health(0.03), Productivity(0.00010), ResearchRate(0.02)],
        ),
        // The WHO/UNICEF Expanded Programme on Immunization pushed DTP3
        // coverage from a few percent to roughly three quarters of the world's
        // infants by 1990. No technology in this file removes more death per
        // dollar, and none is more a matter of logistics than of laboratory.
        tech(
            "bio_universal_immunisation", "Universal Childhood Immunisation", Biotech, Information,
            &[], 40.0, 1990,
            &[Health(0.10), Fertility(-0.05), Stability(0.30)],
        ),
        // Oral rehydration salts: introduced through WHO's diarrhoeal disease
        // programme from 1979 and the centrepiece of UNICEF's child survival
        // push from 1982. Five cents a sachet against the leading cause of
        // child death in 1980. Parents who expect their children to live have
        // fewer of them, which is why this carries a fertility term.
        tech(
            "bio_oral_rehydration", "Oral Rehydration Therapy", Biotech, Information,
            &[], 40.0, 1990,
            &[Health(0.08), Fertility(-0.03), Stability(0.15)],
        ),
        // Norplant, the levonorgestrel implant, was approved by the FDA in
        // December 1990 after twenty-five years of development: five years of
        // contraception that does not depend on remembering anything. The
        // productivity term is not the implant, it is the hours of a woman's
        // working life that stop being spent on unintended pregnancy.
        tech(
            "bio_hormonal_contraception", "Long-Acting Hormonal Contraception", Biotech, Information,
            &[], 50.0, 1991,
            &[Fertility(-0.25), Health(0.02), Productivity(0.00006), Stability(0.10)],
        ),
        // Mouret took out a gallbladder through a laparoscope in March 1987 and
        // Dubois followed in Paris in April 1988; within four years of reaching
        // the United States the keyhole operation was ninety percent of all
        // cholecystectomies. The gain the economy sees is the hospital bed and
        // the six weeks of convalescence that stop being necessary.
        tech(
            "bio_laparoscopic_surgery", "Minimally Invasive Surgery", Biotech, Information,
            &[], 55.0, 1991,
            &[Health(0.03), Productivity(0.00008)],
        ),
        // Kalender's spiral CT prototype ran in 1989 and Siemens shipped the
        // SOMATOM Plus-S, the first commercial spiral volume scanner, in 1990;
        // MRI was already in hospitals. Both are reconstruction problems before
        // they are physics problems, which is why they wait on the silicon.
        tech(
            "bio_cross_sectional_imaging", "Helical CT and MRI", Biotech, Information,
            &["core_cmos_submicron"], 65.0, 1990,
            &[Health(0.05), Productivity(0.00012)],
        ),
        // Conjugating a bacterial polysaccharide to a carrier protein makes an
        // infant's immune system able to see it. The FDA licensed HbOC for
        // infants in October 1990 and PedvaxHIB on 13 December 1990, and
        // Haemophilus influenzae type b meningitis effectively disappeared from
        // every country that adopted them.
        tech(
            "bio_conjugate_vaccine", "Polysaccharide Conjugate Vaccine", Biotech, Information,
            &["bio_universal_immunisation"], 55.0, 1990,
            &[Health(0.06)],
        ),
        // Orthoclone OKT3 (muromonab-CD3) was approved in 1986, the first
        // therapeutic monoclonal antibody: a drug specified by what it binds to
        // rather than found by screening. Wholly murine, so patients raised
        // antibodies against it — the limit the next entry exists to solve.
        tech(
            "bio_monoclonal_antibody", "Monoclonal Antibody Therapeutics", Biotech, Information,
            &["bio_recombinant_pharma"], 60.0, 1990,
            &[Health(0.03), Productivity(0.00006), ResearchRate(0.01)],
        ),
        // The Applied Biosystems 370A shipped in 1987: four fluorescent dyes,
        // one gel lane, a laser and a printout instead of a graduate student
        // reading an autoradiograph. Sequencing stopped being a publication and
        // became a measurement.
        tech(
            "bio_automated_sequencing", "Automated Fluorescent Sequencing", Biotech, Information,
            &["core_pcr"], 70.0, 1990,
            &[ResearchRate(0.03), Health(0.01), Productivity(0.00006)],
        ),
        // Lovastatin was approved in 1987, but the Scandinavian Simvastatin
        // Survival Study published on 19 November 1994 is the date that matters:
        // the first trial to show a lipid-lowering drug cutting all-cause
        // mortality, by thirty percent. Cardiovascular disease is the largest
        // single killer in every developed country in this simulation.
        tech(
            "bio_statin_therapy", "Statin Cardiovascular Therapy", Biotech, Information,
            &[], 75.0, 1995,
            &[Health(0.05), Productivity(0.00008)],
        ),
        // Saquinavir, approved 6 December 1995, was designed against the
        // crystal structure of HIV protease rather than found in a screening
        // library. Chemistry against a known target is cheaper chemistry, which
        // is what the cost reduction is doing here.
        tech(
            "bio_rational_drug_design", "Structure-Based Drug Design", Biotech, Information,
            &["bio_recombinant_pharma", "core_cmos_submicron"], 90.0, 1995,
            &[
                ResearchRate(0.03),
                CostReduction { domain: Biotech, frac: 0.05 },
                Health(0.03),
                Productivity(0.00010),
            ],
        ),
        // The XI International AIDS Conference at Vancouver in July 1996 is
        // where triple therapy stopped being a hypothesis. A disease that had
        // been killing working-age adults at the peak of their earnings became
        // a chronic condition, which is why the productivity and stability
        // terms are as large as they are for a drug regimen.
        tech(
            "bio_antiretroviral_therapy", "Combination Antiretroviral Therapy", Biotech, Information,
            &["bio_rational_drug_design"], 100.0, 1996,
            &[Health(0.07), Productivity(0.00012), Stability(0.20)],
        ),
        // Rituximab was approved on 26 November 1997: a chimeric mouse-human
        // antibody, the first approved against a cancer, and the proof that
        // swapping the murine constant region for a human one turned a
        // one-shot drug into one a patient could be given repeatedly.
        tech(
            "bio_humanised_antibody", "Humanised Therapeutic Antibody", Biotech, Information,
            &["bio_monoclonal_antibody"], 105.0, 1998,
            &[Health(0.04), Productivity(0.00006)],
        ),

        // -------------------------------------------------------------------
        // Networked era: genomics becomes an instrument, and delivery catches
        // up with discovery.
        // -------------------------------------------------------------------

        // Imatinib was approved on 10 May 2001, ten weeks after filing: a drug
        // aimed at a kinase that exists only because of a specific chromosomal
        // translocation. The first time a cancer therapy was chosen by the
        // patient's genotype rather than by the organ it started in.
        tech(
            "bio_targeted_oncology", "Targeted Kinase Oncology", Biotech, Networked,
            &["bio_rational_drug_design", "bio_automated_sequencing"], 130.0, 2001,
            &[Health(0.04), Productivity(0.00006)],
        ),
        // Cipla's Triomune put three antiretrovirals in one pill for under a
        // dollar a day in 2001; WHO prequalified the generic fixed-dose
        // combination and PEPFAR stood up in 2003. This is a manufacturing and
        // procurement technology, not a molecular one, and in the countries
        // where prevalence was highest it did more than the science did.
        tech(
            "bio_arv_scale_up", "Generic Antiretroviral Scale-Up", Biotech, Networked,
            &["bio_antiretroviral_therapy", "bio_universal_immunisation"], 110.0, 2003,
            &[Health(0.08), Productivity(0.00012), Stability(0.35)],
        ),
        // The 454 Genome Sequencer 20 shipped in October 2005, the first
        // instrument to abandon one-read-per-lane for hundreds of thousands of
        // reads at once. The Human Genome Project cost a nation; this made a
        // genome cost a grant. Sequencing consortia publish, hence the emission.
        tech(
            "bio_genome_sequencing", "High-Throughput Sequencing", Biotech, Networked,
            &["bio_automated_sequencing", "core_genome_sequencing"], 165.0, 2005,
            &[
                ResearchRate(0.05),
                CostReduction { domain: Biotech, frac: 0.05 },
                Health(0.03),
                Productivity(0.00010),
                DiffusionEmission(0.03),
            ],
        ),
        // WHO stood up the Global Outbreak Alert and Response Network in 2000,
        // and the World Health Assembly adopted the revised International
        // Health Regulations on 23 May 2005 — which, for the first time, let
        // WHO act on reports a government had not made itself. What made that
        // possible was that the reports no longer had to come from the
        // government: they came over the wire. A state that can see an epidemic
        // coming is a state that survives it.
        tech(
            "bio_outbreak_surveillance", "Global Outbreak Surveillance", Biotech, Networked,
            &["bio_universal_immunisation", "core_packet_internetworking"], 115.0, 2005,
            &[Health(0.04), Stability(0.25), Productivity(0.00008)],
        ),
        // Gardasil was approved on 8 June 2006: recombinant capsid proteins
        // that self-assemble into empty virus-like particles, carrying no
        // genome at all. The first vaccine against a cancer, and the payoff
        // arrives thirty years after the injection, which is why the health
        // term is small for something so important.
        tech(
            "bio_hpv_vaccine", "Recombinant Virus-Like-Particle Vaccine", Biotech, Networked,
            &["bio_recombinant_pharma", "bio_conjugate_vaccine"], 130.0, 2006,
            &[Health(0.03)],
        ),
        // Omnitrope was authorised by the EMA on 12 April 2006, the first
        // biosimilar: proof that a protein drug made in living cells could be
        // copied to a regulator's satisfaction. Biologics stopped being priced
        // as monopolies, which is the only reason the cell and gene therapies
        // further down this file are affordable by anybody.
        tech(
            "bio_biosimilar_manufacture", "Biosimilar Manufacturing", Biotech, Networked,
            &["bio_humanised_antibody"], 145.0, 2006,
            &[
                CostReduction { domain: Biotech, frac: 0.06 },
                Health(0.03),
                Productivity(0.00010),
            ],
        ),

        // -------------------------------------------------------------------
        // Platform era: reading and rewriting biology at industrial cost.
        // -------------------------------------------------------------------

        // Ipilimumab was approved on 25 March 2011, the first drug to extend
        // survival in metastatic melanoma. It does nothing to the tumour: it
        // blocks CTLA-4 and takes the brake off the patient's own T cells.
        tech(
            "bio_checkpoint_immunotherapy", "Immune Checkpoint Blockade", Biotech, Platform,
            &["bio_humanised_antibody", "bio_targeted_oncology"], 250.0, 2011,
            &[Health(0.04)],
        ),
        // Glybera received EU marketing authorisation on 25 October 2012, the
        // first gene therapy approved in the West: an adeno-associated virus
        // carrying a working copy of a gene into muscle. Commercially it was a
        // failure and was withdrawn in 2017. As a vector platform it is the
        // ancestor of everything after it.
        tech(
            "bio_gene_therapy_vector", "Adeno-Associated Viral Vector", Biotech, Platform,
            &["bio_genome_sequencing", "bio_biosimilar_manufacture"], 265.0, 2012,
            &[Health(0.03)],
        ),
        // Illumina announced the HiSeq X Ten on 14 January 2014: ten machines,
        // ten million dollars, and a thirty-fold human genome for a thousand.
        // Reading a genome became cheaper than the clinic visit that ordered
        // it, and everything downstream in this file assumes that. It is a
        // sensor and a compute problem more than a chemistry one — the imaging
        // sensor and the basecalling both ride the same fab that the logic does.
        tech(
            "bio_sequencing_cost_collapse", "Thousand-Dollar Genome", Biotech, Platform,
            &["bio_genome_sequencing", "core_duv_lithography"], 235.0, 2014,
            &[
                ResearchRate(0.05),
                CostReduction { domain: Biotech, frac: 0.06 },
                Health(0.03),
                Productivity(0.00010),
            ],
        ),
        // In April 2015 a nanopore sequencing laboratory went to Guinea in two
        // pieces of airline luggage and returned Ebola genomes inside
        // twenty-four hours of a sample. Surveillance stopped being a report
        // filed after the outbreak and became a map drawn during it.
        tech(
            "bio_pathogen_genomics", "Field Pathogen Genomics", Biotech, Platform,
            &["bio_sequencing_cost_collapse", "bio_outbreak_surveillance"], 245.0, 2015,
            &[Health(0.03), Stability(0.15), Productivity(0.00008)],
        ),
        // Semaglutide was approved on 5 December 2017 for type 2 diabetes; the
        // obesity indications followed. A recombinant peptide, manufactured at
        // biologics scale, against the metabolic disease that underwrites most
        // of the chronic-disease burden of a rich country.
        tech(
            "bio_glp1_agonist", "GLP-1 Metabolic Therapy", Biotech, Platform,
            &["bio_recombinant_pharma", "bio_biosimilar_manufacture"], 250.0, 2018,
            &[Health(0.06), Productivity(0.00012)],
        ),
        // Onpattro was approved on 10 August 2018: an interfering RNA wrapped
        // in an ionisable lipid nanoparticle that survives the bloodstream and
        // unloads inside a cell. The drug treated a rare amyloidosis. The
        // wrapper turned out to be the important part.
        tech(
            "bio_lipid_nanoparticle", "Lipid Nanoparticle Delivery", Biotech, Platform,
            &["bio_gene_therapy_vector"], 280.0, 2018,
            &[Health(0.02)],
        ),

        // -------------------------------------------------------------------
        // Intelligent era: the sequence, the structure and the edit.
        // -------------------------------------------------------------------

        // The MHRA authorised BNT162b2 on 2 December 2020, the first mRNA
        // vaccine anywhere. What it demonstrated is not one vaccine but a
        // manufacturing pattern: publish a pathogen's sequence and the same
        // lipid particle and the same production line make the countermeasure.
        // That is why it needs both the delivery vehicle and the surveillance.
        tech(
            "bio_mrna_platform", "mRNA Vaccine Platform", Biotech, Intelligent,
            &["bio_lipid_nanoparticle", "bio_pathogen_genomics"], 430.0, 2020,
            &[Health(0.08), Productivity(0.00012), ResearchRate(0.02), Stability(0.25)],
        ),
        // AlphaFold2 took CASP14 in November 2020 by a margin no method had
        // ever taken it by, and the structure database opened in July 2021 with
        // whole proteomes in it, free. A fifty-year bottleneck in structural
        // biology was removed, and removed publicly — hence the emission term.
        tech(
            "bio_protein_structure_prediction", "Learned Protein Structure Prediction", Biotech, Intelligent,
            &["core_gpu_deep_learning", "bio_rational_drug_design"], 400.0, 2021,
            &[
                ResearchRate(0.08),
                CostReduction { domain: Biotech, frac: 0.08 },
                Health(0.02),
                Productivity(0.00010),
                DiffusionEmission(0.04),
            ],
        ),
        // Casgevy was authorised by the MHRA on 16 November 2023, the first
        // approved gene-editing therapy: a patient's own stem cells cut at the
        // BCL11A enhancer so they make fetal haemoglobin again. It cures sickle
        // cell disease and costs about two million dollars, which is the honest
        // reason the health term is small.
        tech(
            "bio_crispr_therapeutic", "Gene-Edited Cell Therapy", Biotech, Intelligent,
            &["core_crispr_editing", "bio_gene_therapy_vector"], 490.0, 2023,
            &[Health(0.04), Productivity(0.00004)],
        ),

        // -------------------------------------------------------------------
        // Intelligent era, second pass (authored 2026-09). Every entry below
        // carries its bucket as the first word of its comment: HISTORY where a
        // dated deployment can be cited, ROADMAP where the date is somebody
        // else's published commitment and the floor is set later than it, and
        // SPECULATIVE only in Frontier.
        //
        // ON INERTNESS, because several floors here predate the authoring date
        // and a reader will ask. None of these can move the 1990-2025 output.
        // The learn gate refuses a technology before its floor, and selection
        // is gated on price: a brand-new entry has an adopter share of zero, so
        // its effective price is cost x own, and the highest raw cost that
        // could tie any nation's incumbent Biotech project at January 2025 was
        // measured this session at 245, across twelve seeds. Era::Intelligent's
        // band floor is 300 and Era::Frontier's is 550, so nothing authored in
        // band can displace anybody's focus at the golden checkpoint. Three of
        // the entries below are additionally prerequisite-gated — the 2020
        // surveillance network, the bespoke base editor and the oral incretin,
        // whose prerequisite sets no nation ever completes before January 2025
        // across twelve seeds. The other five are held by price alone, and the
        // cheapest of those is 380 against the measured 245. That is a
        // measurement and not an argument; re-run it before believing it.
        // -------------------------------------------------------------------

        // HISTORY. Deployed 2020-09; transcribed 2026-09. The CDC stood up the
        // National Wastewater Surveillance System in September 2020 with 209
        // sampling sites, and it passed 1,500 sites covering roughly 47% of the
        // United States population by December 2022. The method is older than
        // that — polio environmental surveillance has worked this way for
        // decades — but a standing national network reporting weekly is new,
        // and it does the one thing no clinical system does: it counts the
        // people who never present. New York found poliovirus in sewage in 2022
        // and Colorado found measles in it in August 2025, in both cases before
        // the case that would have raised the alarm walked into a clinic. The
        // floor is 2020 because that is when the first national network was
        // stood up, not when it was finished. Cheap, unglamorous, and it
        // belongs to this file's delivery trunk rather than its laboratory one.
        // The emission term is not incidental: the dashboards are public, which
        // is why every other country copied the design instead of inventing one.
        tech(
            "bio_wastewater_surveillance", "Wastewater Pathogen Surveillance", Biotech, Intelligent,
            &["bio_outbreak_surveillance", "bio_pathogen_genomics"], 340.0, 2020,
            &[Health(0.03), Stability(0.05), DiffusionEmission(0.02)],
        ),
        // HISTORY. Deployed 2023; transcribed 2026-09. Two products in one year
        // against one virus. GSK announced FDA approval of Arexvy on 3 May
        // 2023 — the first RSV vaccine anyone had licensed, fifty-seven years
        // after a formalin-inactivated candidate made the disease worse and
        // killed two children — at 82.6% efficacy against lower respiratory
        // disease in adults over sixty. Nirsevimab followed on 17 July 2023:
        // not a vaccine but a single long-acting antibody given to an infant,
        // which is the only prophylaxis that works in someone whose immune
        // system will not answer one. Galicia immunised more than nine in ten
        // eligible infants in the 2023-24 season and reported an 82% fall in
        // RSV hospitalisations under six months in The Lancet, which is the
        // deployment this floor is read off rather than the trial. Both rest on
        // the same trick — the fusion protein pinned in its prefusion shape by
        // designed mutations — and that is structure-based design applied to an
        // antigen instead of to a drug, which is why this hangs off the design
        // chain and not off the immunisation one.
        tech(
            "bio_rsv_immunisation", "Respiratory Syncytial Virus Immunisation", Biotech, Intelligent,
            &["bio_rational_drug_design", "bio_humanised_antibody"], 420.0, 2023,
            &[Health(0.05), Productivity(0.00003)],
        ),
        // HISTORY. Deployed 2024-07-15; transcribed 2026-09. WHO recommended
        // R21/Matrix-M on 2 October 2023 and said plainly why: demand for the
        // RTS,S vaccine already far exceeded supply, and a second vaccine at
        // two to four dollars a dose was the only route to enough of it.
        // UNVERIFIED: Cameroon is indexed as putting a malaria vaccine into
        // routine childhood immunisation in January 2024, the first country to
        // do so outside the pilot programmes, but only the abstract was seen and
        // the page would not open, so the floor is read off the fielding that
        // WAS checked — Côte d'Ivoire, first to deploy R21, on 15 July 2024, per
        // Oxford. The floor is 2024 either way and nothing turns on which.
        // Efficacy is 75% against symptomatic malaria over twelve months when
        // the three doses land ahead of a seasonal peak and 66% on an age-based
        // schedule: good, not a cure, and it is given alongside the bed nets
        // rather than instead of them. It is a recombinant particle vaccine,
        // which is why it hangs off the virus-like-particle entry, and it is a
        // cold chain and a procurement problem, which is why it also hangs off
        // the immunisation one. The fertility term is the one oral rehydration
        // carries, for the same reason.
        tech(
            "bio_malaria_vaccine", "Paediatric Malaria Vaccine", Biotech, Intelligent,
            &["bio_hpv_vaccine", "bio_universal_immunisation"], 400.0, 2024,
            &[Health(0.05), Fertility(-0.03), Productivity(0.00004)],
        ),
        // HISTORY. Deployed 2025-03-25; transcribed 2026-09. Gepotidacin, as
        // Blujepa: a triazaacenaphthylene that jams both bacterial type II
        // topoisomerases at once, approved by the FDA on 25 March 2025 for
        // uncomplicated urinary tract infection — the first new oral antibiotic
        // class for it in about thirty years — and on 11 December 2025 for
        // uncomplicated urogenital gonorrhoea, where the EAGLE-1 trial put an
        // oral tablet level with injected ceftriaxone plus oral azithromycin,
        // 92.6% against 91.2%. The gonorrhoea indication is the one that
        // matters: ceftriaxone is the last reliable drug against an organism
        // that has defeated every previous one, and this is the first oral
        // alternative in a generation. Size the effect for what it is. This is
        // a holding action against resistance, not a disease retreating, and
        // the tree has no other antibiotic in it precisely because for thirty
        // years there was nothing to transcribe. Structure-based chemistry is
        // the gate and is the entry it depends on.
        tech(
            "bio_novel_class_antibiotic", "First-in-Class Oral Antibiotic", Biotech, Intelligent,
            &["bio_rational_drug_design"], 380.0, 2025,
            &[Health(0.03), Productivity(0.00002)],
        ),
        // HISTORY. Deployed 2025-06-18; transcribed 2026-09. Lenacapavir, as
        // Yeztugo: a capsid inhibitor given under the skin twice a year,
        // approved by the FDA on 18 June 2025 for pre-exposure prophylaxis.
        // PURPOSE-1 recorded no infections in 2,134 women and PURPOSE-2 two in
        // 2,179. A prophylaxis that must be taken daily is a prophylaxis most
        // people stop taking; this is the first one that need not be, and it is
        // the largest public-health effect in this file after 2020.
        tech(
            "bio_long_acting_prophylaxis", "Long-Acting Injectable Prophylaxis", Biotech, Intelligent,
            &["bio_antiretroviral_therapy", "bio_rational_drug_design"], 470.0, 2025,
            &[Health(0.10), Productivity(0.00004), Stability(0.05)],
        ),
        // HISTORY. Deployed 2025-02; transcribed 2026-09. A base editor designed
        // for one patient. KJ Muldoon was born with CPS1 deficiency and dosed at
        // Children's Hospital of Philadelphia in February 2025, about six months
        // from diagnosis to a bespoke editor delivered to his liver in a lipid
        // nanoparticle; the result was presented on 15 May 2025. What is new is
        // not the edit — that is the entry above — but the PIPELINE: design,
        // manufacture and review fast enough that a sick infant outlives it.
        tech(
            "bio_in_vivo_base_editing", "Bespoke In-Vivo Base Editing", Biotech, Intelligent,
            &["core_crispr_editing", "bio_lipid_nanoparticle"], 560.0, 2025,
            &[Health(0.05), ResearchRate(0.03), Productivity(0.00003)],
        ),
        // HISTORY. Deployed 2025-05-16; transcribed 2026-09. The Lumipulse
        // pTau217 to beta-amyloid 1-42 plasma ratio was cleared by the FDA on
        // 16 May 2025, the first blood test for the diagnosis of Alzheimer's
        // disease. It replaces a lumbar puncture or a PET scanner with a
        // venepuncture, which is the difference between diagnosing a disease in
        // a memory clinic and diagnosing it in a country.
        tech(
            "bio_plasma_neuro_biomarker", "Plasma Neurodegeneration Assay", Biotech, Intelligent,
            &["bio_monoclonal_antibody"], 400.0, 2025,
            &[Health(0.04), Productivity(0.00003)],
        ),
        // HISTORY. Deployed 2026-04-01; transcribed 2026-09. Orforglipron, as
        // Foundayo: the first once-daily oral incretin with no food or water
        // restriction, approved on 1 April 2026, with mean weight loss above
        // twelve per cent at the top dose in ATTAIN-1. The molecule is not a
        // peptide, which is the whole point — it is made by ordinary chemistry
        // rather than in a bioreactor, so what limits how many people can be
        // treated stops being manufacturing capacity.
        tech(
            "bio_oral_incretin", "Oral Incretin Therapy", Biotech, Intelligent,
            &["bio_glp1_agonist"], 520.0, 2026,
            &[Health(0.06), Productivity(0.00008)],
        ),
        // ROADMAP. Not ours: Merck and Moderna announced on 19 August 2026 that
        // the phase 3 INTerpath-001 trial met its primary endpoint of
        // recurrence-free survival and its key secondary of distant
        // metastasis-free survival — 1,137 patients with completely resected
        // stage IIB-IV melanoma, intismeran autogene on top of pembrolizumab
        // against pembrolizumab alone. That is the first positive phase 3 for
        // an individualised neoantigen therapy and for any mRNA cancer therapy.
        // Note what the anchor is and is not. A pivotal trial that has read out
        // is better evidence than a filing date, and there is no filing date:
        // the sponsors say only that they will engage with regulators. THE
        // FLOOR IS 2029 AND NOT 2027 BECAUSE APPROVAL IS NOT THE GATE HERE.
        // Each dose is synthetic mRNA coding up to thirty-four neoantigens read
        // off one patient's own tumour, so every course is its own
        // manufacturing run, and this file already carries that lesson three
        // entries up: Casgevy was authorised in November 2023 and still takes
        // the better part of a year per patient. It depends on the checkpoint
        // blockade because the trial tested it as an addition to one and never
        // alone, and on the mRNA platform because that is what makes a bespoke
        // batch a production step rather than a research project.
        tech(
            "bio_neoantigen_cancer_vaccine", "Individualised Neoantigen Therapy", Biotech, Intelligent,
            &["bio_mrna_platform", "bio_checkpoint_immunotherapy"], 570.0, 2029,
            &[Health(0.03), Productivity(0.00002)],
        ),

        // -------------------------------------------------------------------
        // Frontier era: SPECULATIVE. Past the present day, so extrapolated.
        // -------------------------------------------------------------------

        // SPECULATIVE — no such therapy exists. Extrapolated from work that
        // does: senolytic trials, the TAME metformin trial, and partial
        // reprogramming in animals. Assumes a drug that moves the ageing
        // process itself rather than one disease of it, deployable in the
        // mid-2030s. Note the negative stability: a state whose people live
        // much longer is a state whose pension and care promises come due
        // against a workforce that did not grow to match.
        tech(
            "bio_geroscience", "Geroprotective Therapy", Biotech, Frontier,
            &["bio_protein_structure_prediction", "bio_crispr_therapeutic"], 800.0, 2035,
            &[Health(0.10), Productivity(0.00014), Fertility(-0.05), Stability(-0.10)],
        ),
        // ROADMAP. Not ours: the Gates Medical Research Institute opened the
        // phase 3 trial of M72/AS01E in March 2024 and reached full enrolment
        // of about 20,000 participants in April 2025, eleven months early,
        // across 54 sites in South Africa, Kenya, Malawi, Zambia and Indonesia,
        // funded by the Gates Foundation with GSK and Wellcome. Participants
        // are followed roughly four years, until 110 of them have pulmonary
        // tuberculosis. The sponsor anticipates results in late 2028 and
        // regulatory submissions from 2029, and the Serum Institute of India
        // has agreed to manufacture it, saying it is putting more than
        // US$100 million of its own money into the capacity, contingent on the
        // trial. It would be the first new tuberculosis vaccine in more than a
        // century, against the infectious disease that still kills more adults
        // than any other, and the phase 2b that justified the programme showed
        // 54% protection in already-infected adults — which is the number to
        // hold in mind, not a better one.
        // THE FLOOR IS 2032, THREE YEARS PAST THE SPONSOR'S OWN SUBMISSION
        // DATE, because a submission is not an approval, an approval is not a
        // WHO prequalification, and a prequalification is not a dose in a
        // child. The malaria entry in this file is the measurement of that
        // gap: WHO recommended in October 2023 and one country had it in
        // routine use by January 2024, but only because the pilots had already
        // run four years by then, and this programme has run no pilots. It
        // depends on the generic scale-up entry rather than on the vaccine
        // chain alone, because what gates a tuberculosis vaccine is not the
        // antigen — it is whether Serum Institute can make it at a price Gavi
        // will pay.
        tech(
            "bio_tuberculosis_vaccine", "Tuberculosis Subunit Vaccine", Biotech, Frontier,
            &["bio_recombinant_pharma", "bio_universal_immunisation", "bio_arv_scale_up"], 700.0, 2032,
            &[Health(0.08), Productivity(0.00005), Stability(0.10)],
        ),
        // SPECULATIVE, and the downgrade is the point. The floor here used to be
        // discounted off a chief executive's remark that she would file in the
        // second half of 2028 and hope for approval in 2029, with 2030 the
        // reasonable expectation. A chief executive's remark is not an
        // admissible anchor in this file, and a floor set by discounting a date
        // the comment itself rules out is a forecast wearing a roadmap's
        // clothes. So this says SPECULATIVE and the 2033 floor is this file's
        // own number, not somebody else's commitment.
        // KNOWN, and dated: the FDA cleared United Therapeutics' investigational
        // new drug application for the UKidney on 3 February 2025, and the first
        // transplant in the EXPAND study was performed at NYU Langone Health and
        // announced on 3 November 2025 — a pig kidney carrying ten gene edits,
        // six human genes in and four pig genes out, into a patient with
        // end-stage renal disease. The trial starts at six patients across two
        // centres, expands towards fifty on an independent monitoring
        // committee's say-so, and is explicitly built to support a biologics
        // licence application. Those are dated facts and they are hardware.
        // ASSUMED, and published by nobody: that a xenograft and its recipient
        // survive each other for years, and that a six-patient study becomes a
        // licence. What is being tested is not a drug's efficacy but that first
        // question, and it cannot be answered faster than it takes to answer.
        // FALSIFIABLE: graft loss or chronic rejection in the EXPAND cohort, in
        // which case the entry simply stays unbuilt.
        // It depends on the editing anchor for the ten edits and on the
        // humanised antibody chain because the immunosuppression that keeps the
        // graft is itself a monoclonal. If it works, what it removes is less the
        // mortality on a waiting list than the dialysis: three days a week, for
        // life.
        tech(
            "bio_xenotransplant_organ", "Gene-Edited Xenotransplant Organ", Biotech, Frontier,
            &["core_crispr_editing", "bio_humanised_antibody"], 800.0, 2033,
            &[Health(0.05), Productivity(0.00003)],
        ),
        // SPECULATIVE — no drug discovered this way has been approved anywhere.
        // KNOWN: Insilico Medicine's rentosertib is a TNIK inhibitor whose
        // target was chosen by a model and whose molecule was drawn by one; its
        // phase 2a in idiopathic pulmonary fibrosis was published in Nature
        // Medicine on 3 June 2025 (+98.4 mL forced vital capacity at 60 mg
        // against -20.3 mL on placebo, 71 patients), and its phase 3 opened on
        // 7 July 2026. The company reports its candidates took twelve to
        // eighteen months and between 60 and 200 synthesised molecules to reach
        // preclinical nomination, against an industry norm of two and a half to
        // four years.
        // ASSUMED: that this survives the part of drug development that
        // actually costs money. Discovery is the cheap end. The expensive end
        // is phase 3, and nothing in the record above says anything at all
        // about phase 3, because nothing has finished one. The 2035 floor
        // assumes several such molecules clear it and that the method becomes
        // how medicines are found rather than one company's method.
        // FALSIFIABLE, and cheaply: if rentosertib's phase 3 fails, or if
        // AI-originated molecules fail phase 3 at the ordinary rate, then this
        // is a saving on the cheapest tenth of a pipeline and the numbers here
        // are two or three times too large. That is why the terms are
        // ResearchRate and CostReduction and not Health — searching chemical
        // space faster is not by itself a cure for anything. It sits on the
        // structure prediction entry because that one settled what a target
        // LOOKS like; this is the further claim that a model can pick the
        // target and draw the molecule to fit it. If it does not arrive, the
        // entry simply stays unbuilt.
        tech(
            "bio_ai_originated_therapeutic", "AI-Originated Drug Discovery", Biotech, Frontier,
            &["bio_protein_structure_prediction", "bio_rational_drug_design"], 880.0, 2035,
            &[
                ResearchRate(0.05),
                CostReduction { domain: Biotech, frac: 0.05 },
                Productivity(0.00003),
            ],
        ),
    ]
}
