# A1 historical basis and taxonomy audit — 2026-09-22

Read-only research review. No simulation, test, threshold or acceptance-status change. This is not evidence that the current failing A1 passed.

## What A1 actually counts

`spheres-sim/tests/bloc_census.rs:389–401` counts the emitted Army-removes-elected-government route, separately from annulment and regime coups. Its 252 monthly ticks cover 1990–2010. The concentration statistic is calculated within each seed and then its median is compared with 0.5. The first arm requires a median count from 4 through 14 inclusive. This instrument is neither a count of every democratic breakdown nor a direct implementation of the Powell–Thyne event definition: it is restricted to one game route and does not independently validate seven days of post-coup control. The route's event updates are genuine government changes; the issue here is which historical category its design examples justify.

## Primary-source definition and reproducible historical count

Powell and Thyne's original paper (2011), Journal of Peace Research 48(2), pp. 251–253, distinguishes illegal overt executive-removal attempts by military/state elites from legal intervention, self-coups, plots and outside insurgencies. Success uses at least seven days of control. It does not limit the target to an elected government. The paper explicitly excludes self-coups on p.253. The author's current project page retains the core definition and explains current versioning.

Paper: https://jonathanmpowell.com/wp-content/uploads/2022/11/powell-thyne-2011jpr-global-instances-of-coups.pdf
Project: https://jonathanmpowell.com/coups/

The directly retrieved author CSV, version V2026.08.29, has 497 total rows. Filtering `1990 <= year <= 2010` yields 94 attempts, 35 coded successful and 59 unsuccessful. The 35 successes occur in 26 countries. Niger and Sierra Leone have three each; the third highest count is two (several countries tied), making pooled top-three share 8/35 = 0.228571. These are independently computed observations, NOT A1's calibration target. They include unelected targets, nonmilitary state elites, and countries outside the game roster. One historical realization's pooled concentration cannot identify the median of a counterfactual simulation's per-seed concentration. No elected-only/roster-filtered historical series was produced in this bounded audit.

CSV: https://jonathanmpowell.com/wp-content/uploads/2026/08/pt_20260829.csv
Raw bytes: 20,089; SHA256: 1e8f933b22f4b9a77d0ba5eb59bf0d8158adcaab27746adcf345195be87a9012.
Exact raw data and filtered records are retained beside this report. The current author's release is revisionable, not the immutable original 2011 release; the page warns of prospective updates. The separately linked UKy codebook returned HTTP403, so its present contents were not read. Definitions above were verified in the original author-hosted paper instead.

## Critical examples

**Peru, 5 April 1992 — does not fit A1's asserted executive-removal category.** The IACHR's contemporary 1993 country report, chapter III paragraphs 42 and 52–53, describes Fujimori's dissolution of Congress and transfer of legislative functions to the executive. The IACHR's subsequent findings reproduce Inter-American Court judgment facts 89.1–89.6: Fujimori remained President after the 1992 institutional break and was later reelected. It was a military-backed incumbent self-coup, not an Army replacement of him. The author CSV contains a FAILED Peru attempt dated 13 November 1992 and another failed attempt in October 2000, with no successful Peru event in 1990–2010. Peru supports a broader democratic-breakdown requirement, not this precise count.

Sources: https://cidh.oas.org/countryrep/Peru93eng/iii.htm ; https://www.oas.org/en/iachr/decisions/court/12214fondoen.pdf (pp.7–8, Commission proven-facts section; header is an unfinalized report template, so use it as corroboration of reproduced Court findings, not a dated final report).

**Algeria, 11 January 1992 — a genuine removal plus electoral interruption, not simply one or the other.** UN Special Rapporteur report E/CN.4/2003/66/Add.1 paragraphs 15–16 states that Chadli was forced to resign on 11 January and a High Council of State was formed on 14 January. CRS 98-219, 18 August 1998, p.2, independently describes the army forcing resignation and cancelling the unfinished parliamentary election. Powell–Thyne codes 11 January as successful. But the removed incumbent was Bendjedid, not an already-installed FIS government: the competitive parliamentary process had not completed. CRS p.3 dates the first multi-candidate presidential contest to 1995. Thus this is not clean evidence of overthrowing a freely elected FIS executive. A game's mutually exclusive Annul route can reasonably stay separate, but the historical example cannot then be presented as an unqualified observation in A1's elected-removal sample.

Sources: https://documents.un.org/doc/undoc/gen/g03/101/14/pdf/g0310114.pdf ; https://www.everycrsreport.com/reports/98-219.html (primary CRS publication via a third-party republication).

**Pakistan's dismissals versus 12 October 1999 — distinguish the events.** The National Assembly's own parliamentary history records presidential dissolutions under Article58(2)(b) on 6 August1990 and 18 April1993; the Supreme Court restored the latter Assembly on 26 May, followed by dissolution on the PM's advice on 18 July. It separately records General Musharraf taking over Nawaz Sharif's government on 12 October1999 and subsequent suspension of assemblies. The current Powell–Thyne series has only the 1999 successful coup for Pakistan during this window. The 1990/1993 dissolutions cannot simply be added as successful military coups; legality, military coercion, executive identity and constitutional proceedings need case-specific coding. This audit does not claim every presidential dismissal was legitimate.

Source: https://na.gov.pk/en/content.php?id=75 (browser reader failed, but the full official HTML was directly fetched and checked; raw response SHA256 is in the receipt).

## Are the numbers sourced?

The A1 source comment at lines962–978 explicitly identifies top-three-under-half as a session's numerical expression of a qualitative design sentence that had no number. It reserves re-expression to the user. SPEC.md's S5/history paragraphs and BUGS.md S5–S6/H6 repeat 4..14 as an existing design band; no event-level historical derivation of those endpoints was located. Bootstrap calculations estimate how many simulation seeds reliably reproduce an observed simulation median; they do not establish that the acceptance interval matches historical frequency. This is a bounded finding about the inspected repository material, not proof that no earlier user discussion ever justified the band.

The comment also lists Sierra Leone, Gambia and Niger; BUGS S6 notes those nations have no polity row. Those cases may motivate a broad world-history aspiration but cannot be treated as opportunities in the game without an explicit coverage crosswalk. Haiti's opening regime status is not proof its later elected government cannot be toppled.

## Strongest case for retaining the test

A transparent game-design requirement can be valid without being a statistical estimate from history. Requiring political instability to arise across the world, rather than recur in a few mechanically disadvantaged small countries, protects believable campaign variety. Broad historical spread provides directional support. The current independent iteration12 audit found six countries generated 97.1% of electoral coups, so this is not merely a rare small-denominator anomaly. The test has already exposed real funding, coverage and lifecycle defects. Keeping it unchanged preserves an independent predeclared guard against tuning away an inconvenient failure. No source found here proves <0.5 unreasonable or proves the current build meets it.

## Actual contract defects / limitations

The historical rationale mixes executive removals, an incumbent self-coup, election interruption and presidential dismissals. It also mixes global history with a reduced game roster. Neither numerical arm has a located empirical event-to-instrument derivation; the concentration arm expressly originated as a design choice. The strict share also imposes a stronger repetition constraint at low totals: with 1–6 events it cannot pass; with 7–8 it requires every event in a different country. Those are properties to disclose, not permission to alter results. The simulation's all-zero share sentinel is irrelevant to iteration12, which has no zero-count seeds. Any future historical calibration would first need an explicit elected-incumbent definition, roster/date crosswalk and route taxonomy, while preserving the original guard and its current failure as a separate recorded result. No alternative numerical threshold is recommended here.
