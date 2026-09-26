# Resource lookup optimization: selected stage1

The shipped change is only a three-line early return in `dyads::last_resort_with`: an all-false resource-stall mask returns `None` before per-target cover/table lookups. With no stalled commodity, the original loop could only return `None`. Nonempty masks execute the original statements unchanged. Arithmetic, iteration order, RNG, event rules, save schemas and test assertions are unchanged.

Selected raw `dyads.rs` SHA-256: `a9bc6689a343d1957130a3b38af4f020e2d22b4d8c2c9dfe77db4b3f89c2a98b`. All other 1,931 recorded native inputs match checkpoint26. The parent source-input manifest is preserved in the neighbouring `checkpoint27-rejected-model/` archive. This folder's external receipts retain their original machine-specific paths.

## Results and selection

| Measured source | Native workspace | Appetite ms/month | Total ms/month | Decision |
| --- | --- | --- | --- | --- |
| Restored checkpoint26 | Previously 1,886 passes; fresh Algeria and timing checks pass | 0.0208 | 0.0629 | Baseline |
| Stage1: empty-mask return | 1,886 passed, zero failed; 89 ignored, one timing filter; 66 targets | 0.0160 | 0.0573 | Selected |
| Stage2: also check mask before tracked-table lookup | 1,886 passed, zero failed; same ignored/filter counts | 0.0162 | 0.0584 | Removed: no additional measured gain |

Each timing row is one execution of the unchanged isolated best-of-three, 1,200-month harness. All pass its **0.15 ms hard limit**; all miss its **0.05 advisory target**. Stage1's observed total reduction is about 9%; this is a local measurement, not a cross-hardware speed guarantee. The small stage2 difference is not called a semantic or performance regression. It was removed to retain the smaller evidenced optimization; both its passing tests and timing are preserved.

After restoring stage1 byte for byte, a fresh build passes all eight selected resource, deterministic-campaign and Algeria chronology checks. The complete original political suite records **10 passes and one failure, A1**: median seven coups and median top-three share 0.59. The independent final review confirms all ten printed metrics and all twenty unique printed seed rows match checkpoint26 exactly. This is not campaign certification. No coefficient, acceptance threshold, historical input, test fixture, development cohort or reserved holdout was changed.

## Other verification

All seven metadata commands pass, with generated files unchanged (`tooling/`). The Node applicability audit (`node-applicability/`) rechecks the runner, all 107 tests, the test list, UI/assets/generated inputs and broad source manifest; the 1,701-pass checkpoint26 result remains applicable to those unchanged inputs. It is not presented as a new Node run or a new browser run.

The preservation verifier passes for all eight protected files and original preview servers. The selected-result receipt records the fresh native executable identities separately from earlier mutable build paths. Existing checkpoint26 archive, latency and browser evidence remains separately identified; this optimization does not claim to have rerun those unchanged interfaces.

Raw logs, prospective applications, independent reviews, both trial outcomes and the final selection are retained. `manifest.json` hashes every other file under this folder, including nested verification archives. The original political A1 failure and the advisory timing miss remain explicit.
