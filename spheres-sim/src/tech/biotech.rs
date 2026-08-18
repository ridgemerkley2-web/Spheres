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
//! naming the first deployment its year floor is read off. Anything past the
//! present day is speculative and must say so in the comment.
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
    ]
}
