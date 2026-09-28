# A1 observer: complete portable diagnostic evidence

This packet preserves the complete fixed development diagnostic at observer candidate **6818e4f0d94b01c86d7a9acc4252260947d13504**, plus Codex `/root/review_gap_submission`'s read-only analysis. It is **not an A1 calibration pass, reserved-holdout result, campaign qualification, or task closeout**. No source, coefficient, criterion, seed, or simulation was changed or executed while packaging.

The actual retained run used seed 0 and 252 ordinary legacy monthly ticks. It reports 252 exact full-native-world/RNG/headline comparisons between unobserved and observed legs, 146,525 snapshots, and 10 elected-government coups. The observer test passed; that demonstrates observer equivalence in this run, not acceptable coup frequency or geographic spread.

The first focused build failed before tests because two research files referenced by `include_str!` were absent from the sparse checkout. Its original log is retained. The second build completed and all **3 focused observer tests passed**. The separately invoked ignored observer diagnostic then reports **1 test passed** (1,114 filtered out). These are existing coordinator executions, not new peer-review test runs.

The complete 240,585,752-byte observations stream is retained as a 15,387,211-byte gzip. Both complete final worlds are also retained as gzip, with identical decoded size 2,137,811 and SHA-256 `8ff7ce538fc47b0a22a13d25e6af73c04bb35cdc416e710fd82bfe1600150431`. All three use deterministic gzip headers: mtime 0, empty filename, compression level 9. Every compressed and decoded size/hash is in the manifest; all round trips were verified against the untouched originals. No data is omitted under the 50 MB observation limit, and no original files were removed.

## Contents and identity

- `original-run/`: verbatim plan/result and lossless gzip of all three original large files.
- `execution/`: original failed and passing focused logs, ignored-run log, and frozen `execution.json` source/binary binding.
- `analysis/`: the entire original analysis packet, byte for byte, including its unchanged historical README and manifest.
- `source/`: the six reviewed source files, byte for byte, with raw and Git-normalized pins retained in the analysis. They match the candidate checkout; no generated source edits are bundled.
- `verify_portable.py`: standalone, source-path-independent integrity check and optional create-only restoration of the five original run files.

The coordinator's `execution.json` binds the exact source revision to frozen executable SHA-256 `27f282ad02daeaf9fcd527682387b0615c833a9a326a521cc5c029432ef78071` before and after the successful run. Packaging independently rehashed the frozen executable (51,475,482 bytes) and matched that binding. The executable is an **external hash reference**, not included in this evidence packet. This is coordinator build/execution provenance, not a reproducible-build attestation or an independent recompilation by the reviewer.

The earlier analysis README states that this source/binary binding was absent from the diagnostic directory supplied for its review. That original limitation is retained unchanged. This additive packet now supplies the separate frozen execution record; it does not retroactively rewrite the peer review or claim that the reviewer executed the native test. Intermediate monthly worlds were not separately retained, so the reviewer checked the 252 native comparison records and test code, independently matched the two final worlds, and recomputed recorded walk/funding/trigger formulas—not all intermediate simulations.

## Verify or restore

From any location with Python 3:

```powershell
python -B -X utf8 <packet>/verify_portable.py
python -B -X utf8 <packet>/verify_portable.py --restore-run <NEW-ABSOLUTE-DIRECTORY>
```

Verification reads only the packet, checks every manifest file, decompresses and hashes all three gzip streams, compares the final pair, and matches original snapshot stage counts and firing cases. It neither executes nor needs the native binary. Restoration is optional, requires a new destination outside this packet with an existing parent, and creates only the five original run files; it never overwrites a directory or source. Restoration needs about 245 MB free. The packaging verification used streaming decompression; **a full destination restoration was not executed**, and no claim of one is made.

The bounded review found no additional implementation defect: all eligible recorded funding commands applied, nine firing cases lacked fiscal headroom throughout their preceding year, and the remaining case still had hostile actual loyalty while recovering toward a paid target. That explanation does not establish that the political model meets A1. A1 remains open.
