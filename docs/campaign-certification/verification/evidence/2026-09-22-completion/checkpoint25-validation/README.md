# Checkpoint25 validation and subsequent archive repair

Validated source: `7b8a3dbb90f191ee5520705a8233409274d9d752`.

The locked release workspace passed 1,884 tests, with zero failures, 89 ignored and one separately measured timing check. That isolated resource check passed its unchanged 0.15 ms/month bar at 0.0631; the advisory 0.05 target was not met. This is not an all-political-gates pass.

Three of four genuine archived-save checks passed. The unchanged pinned-master test failed because load added an Army pillar to an existing government. All 32 original fixture/provenance hashes stayed unchanged. The failure is preserved under `archives/`; `repair/` documents the subsequent source fix, whose rebuilt validation is pending.

Both mature native-preview latency checks passed unchanged 300ms p95 / 750 ms maximum bars: decision 12.7586 / 12.9232 ms, government 11.9048 / 12.0682 ms, reciprocal-route 188.0559 / 189.2093 ms. Both original input archives, binaries and clean source stayed unchanged. These do not measure browser or HTTP latency.

`preceding-hosted24/` records the older b51b7333 run: four jobs passed and six failed. Both native platforms reported exactly the same three test failures repaired by25; both political jobs failed solely A1; aggregate checks correctly failed closed. Its red results are not replaced by the later local successes. Hosted25 is separate and still in progress when this archive is written.

The original A1 concentration rule remains unchanged and failing; its separately archived original12 comparison does not authorize another criterion. No independent held-out cohort has run. These receipts are not campaign certification.
