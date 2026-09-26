# Checkpoint25 regression repairs

Checkpoint24 (`b51b7333`) passed 1,881 workspace tests and failed three. Its full original log and receipt remain in the adjacent political-calibration evidence. These repairs do not change production simulation or server behavior.

- Move all seven existing stratagem unit tests into the standard `stratagems/tests.rs` child module. The production prefix is byte-identical, and the treasury audit still scans the entire production file with its original forbidden-write assertions. Its false positive was the unit fixture assigning initial debt.
- Keep the actual player dissolution, required pause and explicit observer command. Compare eleven subsequent advances against independently settled simulation months, including the exact legitimate interrupt, date and saved world. An incidental minimum elapsed span is no longer mistaken for the no-repeated-death invariant.
- Establish two-party and three-party quarrels through paid public commands, then execute the real dissolution phase. Assert both disappearance of an empty-sided conflict and retention of its living peers, while serving the view leaves the complete world unchanged. Keep the full 360-month payload invariant sweep.

All seven metadata/provenance commands pass without source hash or mtime changes, and all generated outputs remain byte-identical. The full rebuilt tests are pending when this repair packet is recorded. A1 political concentration remains a separate unresolved acceptance gate; its cutoff and cohorts are unchanged.
