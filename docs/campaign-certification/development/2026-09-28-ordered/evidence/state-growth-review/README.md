# Read-only campaign save-growth review

Reviewer: Codex /root/review_gap_submission. Source inspected at integration revision 8d338dd217af0e4c83725f999812eef37e55bbb7. No Cargo, simulation execution, source modification, input rewrite or pruning was performed. Parent confirmed all owned attempt02 processes stopped before the first file read; all file handles were closed and read completion was reported before parent moves the attempt.

## Exact observed input

Original path: C:/Users/ridge/Documents/Codex/2026-09-05/pick-up-the-spheres-game-on/work/campaign-certification/evidence/codex-next-matrix-ssd-02/cells/france-1990/native/resumed/saves/monthly.json

135,739,703 bytes; SHA256 5480ccc296c2fe67ef2234871d1ef5fc9f098184ca1c83d885b07010a1368b20. Hash independently matched on first and last read. The retained envelope date is **1 Jan 1994**, although the latest externally observed progress line was Dec1993. Two early profile filenames retain the initial date estimate; their contents refer to this exact hash, not a separate December save.

## What grew

Exact raw JSON value-span accounting: world envelope 107,103,841 bytes (78.904% of file), history28,452,641 (20.961%), dispatch log182,999 (0.135%). Within the native world: population_system71,837,269; fiscal_recovery15,607,885; logistics9,944,763; economic_ai2,979,326; starting_industry1,563,111; nations1,532,218; province_economy1,189,825.

The population contains2,584 province rows and **496,309 province Course records**, plus265 unallocated courses. Course field payload is approximately61,420,931 bytes when independently compact-JSON encoded by Python; this estimate is explicitly distinct from the exact raw span totals because number formatting may differ from Rust. There are258 duplicate exact course keys (0.052%), no zero-person records, no completed retained courses, and only8 records at or below the already-existing1e-14 people_m pruning threshold. These small sets cannot explain or materially repair the growth.

ST-P alone contains48,545 course records across93 course/start groups. One group contains1,092 distinct funded-day values spanning985.9867421635334 to1095.0; the smallest adjacent gap is0.0835193853729379days. This is not just floating-point roundoff producing identical records. Other leading districts include GQ-AN44,764; GY-BA41,387; SR-BR40,705; TT-ARI37,518. A few other groups do contain near-ULP differences, but the dominant groups represent distinct accumulated funding progress.

The fiscal journal has7,110 day records, exactly45 per158 nation rows; fiscal observations are bounded to12. Presentation history contains1,110 points between opening1990-01-01 and1994-01-01 and already compacts after its three-year daily window. Logistics contains3,537 cargo with unique IDs (937 held), plus174 unique current arrivals. No duplicate cargo-ID growth was found. Held cargo is paid, undelivered state; discarding it is not justified by this size audit.

## Causal source review

- population.rs1116–1142: every existing course advances according to local education funding and teacher staffing; completed or already-negligible entries are removed.
- population.rs1148–1180: at most four new route cohorts per province per monthly intake.
- population.rs1475–1534: daily household migration carries proportional students and their exact schooling progress. A source cohort repeatedly arriving at a destination with different funding accumulates a genuinely different funded-day history on each arrival day.
- population.rs1545–1555 and1590–1627: moved people are scaled and course merging retains exact first-match keys/order. Distinct progress is intentionally not conflated. Updating progress can incidentally cause a small number of exact keys to converge later, explaining some duplicate keys; changing accumulation order is not automatically bit-exact.
- spheres-web/src/history.rs86–145: first/lifespan milestones plus three-year daily, twenty-year monthly and older annual points. This is existing explicit retention, not an unbounded daily snapshot bug.
- fiscal_journal.rs8 sets45 retained days; fiscal_recovery.rs412 bounds monthly observations.
- spheres-web/src/s25_stability_tests.rs536–539 uses one monthly slot and native prior backup. It does not intentionally leave one file per month.

## Recommendation

No confirmed population-duplication or stale-history bug explains the dominant growth. Do not truncate history, discard students, round funding progress, change migration cadence, or weaken complete-archive comparisons to make the run fit. Such changes alter gameplay and require a separately justified model design.

Treat this observed failure first as an execution-resource planning problem: reduce simultaneous cells according to measured memory, use transparent byte-preserving scratch compression where supported, verify final native archives before gzip/offload, and preserve original raw hashes/restoration mappings. Keep failed attempts and all complete state comparisons. A future lossless storage encoding can be considered separately, but it does not itself bound live course memory. Current observations cannot certify the2035 peak or establish that a four-worker run will fit; monitor real working sets and remaining storage throughout the complete horizon.

No performance, long-horizon stability or certification pass is claimed.
