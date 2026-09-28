# S22 qualification pair01 — complete, failed

**S22 remains open.** All 18 predeclared measurements completed on candidate
`d50f7ee1a3440b00c8968b530e8e8b45aa0899fd`; 17 passed. The initial full round
passed. The confirmation failed its end-2035 native cell: simulation/history
p95 **323.6552 ms exceeds 300 ms**, and whole-turn p95 **406.1905 ms exceeds
400 ms**. No cell was retried, omitted or replaced. No budget was widened.

The exact [verdict](records/verdict.json), [runner summary](records/run-summary.json)
and [frozen manifest](records/frozen-manifest.json) remain unchanged. The final
verifier checked identity, all 18 cells and both round memberships; its failed
cell retains the original assertion message. The native runner's two explicit
numerical failures are also indexed in the archive manifest. Runner exit was 1.
Measurement started 28 September 2026 at 09:59:30.085 UTC and the last successful
browser cell ended at 10:35:00.510 UTC. The verdict was written at 10:35:05.866 UTC;
the final runner summary at 10:35:43.574 UTC.

## Results

| Round / input | Simulation/history p95 ms | Whole-turn p95 ms | Whole-turn maximum ms | Private / OS peak bytes | Result |
| --- | ---: | ---: | ---: | ---: | --- |
| Initial / 1999 | 179.7859 | 224.7219 | 246.6188 | 456,105,984 / 415,330,304 | Pass |
| Initial / 2015 | 259.2079 | 326.8361 | 493.4156 | 858,337,280 / 854,269,952 | Pass |
| Initial / 2035 | 273.8758 | 365.3191 | 435.4161 | 906,424,320 / 901,955,584 | Pass |
| Confirmation / 1999 | 193.7636 | 252.0204 | 264.7847 | 455,168,000 / 414,130,176 | Pass |
| Confirmation / 2015 | 258.5614 | 314.6700 | 457.4270 | 854,745,088 / 834,170,880 | Pass |
| Confirmation / 2035 | **323.6552** | **406.1905** | 646.8852 | 908,431,360 / 903,606,272 | **Fail: both p95 limits** |

Table values are displayed to four decimal places; unchanged artifacts retain
the original numeric values. Every native memory check passed both unchanged
1 GiB limits. The failed cell's whole-turn maximum passed the 750 ms bound.
All inputs remained unchanged and ordinary/batch final-state checks matched.
Both rounds end with the same world fingerprints: 1999 `5e2885a23e03ba8e`,
2015 `5c486d0c186f921e`, and 2035 `3c3f21d9d67bd7b8`.

All **six map and six renderer cells passed**. Each map cell includes the eight
Standard/Low views and 62 ordered trusted controls. The lowest observed map rate
was 33.1293960960586 FPS; the largest control p95 was 51.30000000447035 ms.
The six fighter orbit rates range from 99.91333622213834 to 99.9983333611479 FPS.
The released fighter inspection was 200,446 triangles on each of six visits per
renderer cell. Native layouts, cold/load checks, cache limits, context recovery,
viewer disposal and read-only campaign checks passed in both rounds.

Browser memory observations retain process/CDP, queried GL buffer sizes and
API-declared resident texture texel payloads, including explicit unknown counts.
Those GL payloads are not physical VRAM measurements. Native memory samples
cover the complete isolated test process, including loading and report work;
sampled private bytes and the OS working-set high-water mark remain distinct.

The [prior validation packet](../margin-progress/README.md) records the same
candidate's 1,954 release tests, seven focused tests, six actual 31-day parity
checks and passing isolated preflights. They do not override this failed pair.
Further CPU work requires new correctness evidence and a new complete pair.
S22, G5 and CP1 are not awarded.

## Lossless archive and storage operation

[manifest.json](manifest.json) maps every original absolute path, byte count
and SHA-256 to its encoded payload. It includes **all 636 original pair files
(6,455,426,397 bytes)**, final runner output, frozen config/manifest/registry
snapshot, hardware, exact harness/probe/UI sources, reviewed lineage and source
provenance. All traces, screenshots, observation JSON, profiles, memory CSV,
stdout/stderr, invocations and save copies remain recoverable. The registry is
the exact snapshot at pair01 completion, not a replacement for future entries.

The complete packet has **768 original-path records**, **545 unique payloads**
and **551 ordered chunks**, totalling **1,502,596,171 encoded bytes**. Files
larger than 90 MiB are split into ordered chunks below the GitHub file limit.
Chunk, complete encoded-payload and decoded-original hashes are all retained.
Compressed traces and distinct gzip encodings are preserved byte for byte.
Deduplication only shares identical encoded bytes; metadata-distinct save copies
retain distinct original hashes and exact restore payloads.

With explicit authorization, 42 redundant raw save copies were removed only
after complete gzip decompression reproduced their exact original size and
SHA-256, followed by a fresh raw-file check immediately before each removal.
They comprised six native inputs, twelve browser inputs and twenty-four browser
before/after saves. All 42 exact paths, original timestamps, raw/gzip hashes,
verification and individual removal receipts are preserved under the manifest's
`s22-pair01-preservation-01` paths. The operation freed **4,737,930,442 bytes**.
Only those task-created raw copies were removed; canonical 1999/2015/2035 and
S19 sources, unique gzip files and all reports remained unchanged.

Three additional small records under `s22-preflight-raw-dedup-03` document a
separate authorized removal of 15 older, nonqualification raw input duplicates,
freeing 1,979,413,207 bytes after canonical hash/size checks. Those records do not
alter pair01's 636-file inventory or 42-copy operation.

The four canonical input bodies are referenced through their existing immutable
Git gzip archives and were independently decompressed and checked during this
archive operation. Two immutable d50 runtime executables are external references
with exact size/SHA pins; executable bodies are not included in Git. Their
absence is explicit and never reported as archive-body verification.

## Verification and restoration

Run from a checkout containing this packet:

```text
node docs/campaign-certification/S22/qualification-pair-01/archive.cjs
```

This read-only command verifies every chunk, encoded payload, original decoded
file, canonical gzip reference, complete original pair inventory and readable
record mirror. It does not launch the game, replay a campaign or requalify S22.
The completed [archive verification](records/archive-verification.json) reports
768 records, 573 distinct decoded encodings, all 636 pair files and zero writes
to restored paths. Git staging is separately checked for exact blob bytes.

To reconstruct the retained original artifacts into a **new, nonexistent**
directory on a drive with enough free space:

```text
node docs/campaign-certification/S22/qualification-pair-01/archive.cjs --restore D:/s22-pair01-restored
```

The helper verifies the packet first, then writes create-only files beneath the
new destination. `C:\original\path` becomes `D:/s22-pair01-restored/C/original/path`;
`restore-map.json` records each mapping. It never overwrites live original paths,
changes frozen records or restores executable bodies. Allow at least 9 GiB for
the decoded pair and captured prerequisites. Full disk restoration has not been
executed here; every would-be output's decoded bytes were independently verified.

The original qualification verifier expects the original absolute paths and
external binaries. To use it on a restored snapshot, stage those exact verified
bytes in an isolated environment at the recorded paths and provide the pinned
binaries. Do not rewrite the frozen manifest or overwrite current source files
to accommodate restoration. Its expected result remains **failed**. The archive
helper's success only proves retained byte integrity and recoverability.
