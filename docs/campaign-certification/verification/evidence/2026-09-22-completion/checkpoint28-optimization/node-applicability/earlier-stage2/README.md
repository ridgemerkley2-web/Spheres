# Node result applicability after final dyads optimization

This is a read-only comparison, not a rerun. The recorded checkpoint26 result remains 1,701 passed, zero failed/skipped across 107 test files on Node v24.12.0. The runner and all 107 tests remain raw-byte identical; the selected test list is unchanged.

The entire 1,933-file source manifest was compared against current raw files (825,374,461 baseline bytes). Only spheres-sim/src/dyads.rs differs; all 1,932 other files, including UI assets and generated data, match their recorded SHA-256 values. Additional tools/ui and tools/arsenal tracked content is unchanged against 9f7faed5.

The runner and Node test subprocesses do not run Cargo or a live native simulation, and no test source references dyads. Reusing the Node result is therefore justified for these unchanged inputs. Native optimization correctness, timing, browser, and political results require their own receipts. No tests, generators, repository inputs or source were changed by this review.
