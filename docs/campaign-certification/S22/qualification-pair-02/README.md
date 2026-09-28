# S22 qualification pair02 — 18/18 passed

Both complete, predeclared rounds **passed all 18 cells** on candidate
`5d11dd6dae436eeca105dbaf0d02732cd768b35b`. This archive retains every original
artifact, including the six native windows and twelve browser runs. No cell was
retried, omitted or substituted, and no threshold changed. The separate
[failed pair01](../qualification-pair-01/README.md) remains unchanged.

The authoritative [verdict](records/verdict.json), [runner summary](records/run-summary.json)
and [frozen manifest](records/frozen-manifest.json) are preserved byte for byte.
The runner completed with exit 0. A second reviewer reran the existing frozen
evidence verifier on the original paths before storage cleanup: exit 0, all 18
cells passed, all 83 identity files rehashed, unchanged runtime tree. Its
[independent verdict](records/independent-verdict.json) matches the original
except for the verification timestamp; it did not rerun the game measurements.
The [independent review](records/independent-review.md) and
[derived compact observations](records/independent-summary.json) are also retained.

The manifest was frozen on 28 September 2026 at 11:35:28.248 UTC. Measurements
ran from 11:35:48.236 to 12:11:19.665 UTC in manifest order without overlapping
cell windows. The authoritative verdict was written at 12:11:25.676 UTC and the
runner summary at 12:12:05.272 UTC. Independent verification completed at
12:13:21.662 UTC. Complete command, hardware, source, binary, input and lineage
pins remain in the frozen manifest and archive mapping.

## Native and browser results

| Round / actual input | Simulation/history p95 ms | Whole-turn p95 ms | Whole-turn maximum ms | Private / OS peak bytes |
| --- | ---: | ---: | ---: | ---: |
| Initial / 1999 | 182.2451 | 243.0344 | 243.8629 | 458,428,416 / 416,329,728 |
| Initial / 2015 | 257.4512 | 314.1968 | 458.5752 | 852,783,104 / 781,721,600 |
| Initial / 2035 | 251.3132 | 344.3258 | 444.8095 | 902,717,440 / 898,027,520 |
| Confirmation / 1999 | 204.7656 | 252.4781 | 278.2701 | 457,187,328 / 402,300,928 |
| Confirmation / 2015 | 255.5951 | 312.0435 | 658.3375 | 978,538,496 / 840,454,144 |
| Confirmation / 2035 | 237.4000 | 323.1374 | 409.3736 | 901,349,376 / 891,039,744 |

Table values are displayed to four decimal places; the original raw observations
retain full precision. Each native window contains 31 consecutive actual days,
passes all unchanged 300/400/750 ms limits and both 1 GiB observed memory limits,
and matches ordinary/batch final facts. Input hashes remain unchanged. Same-case
final fingerprints match across rounds: `5e2885a23e03ba8e` (1999),
`5c486d0c186f921e` (2015), and `3c3f21d9d67bd7b8` (2035).

All six map cells pass the eight Standard/Low world/national/regional/city views
and both ordered 31-click trusted control sequences. The minimum measured map
rate is **32.0651270975245 FPS**; the largest control p95 is **51.5 ms**. All six
renderer cells pass released-fighter orbit performance, with rates from
**99.91333622207631 to 99.99750006246119 FPS**. Each renderer cell records six
visits to the 200,446-triangle fighter and 47 lifecycle observations.

All twelve browser runs retain cold/load observations, hashed nonempty Chrome
traces, screenshots, complete process/CDP/GL memory observations and unchanged
normalized campaign saves. They pass 390×844 and 3440×1440 functional profiles,
keyboard/touch and focus return, scrolling/framing, CityMesh eviction/revisits,
Cities Off/Low detail, context recovery, lazy card mount/removal and zero viewer
buffer/texture counts after each close. The actual recorded GPU is RTX 5070 via
ANGLE/D3D11.

The preceding [validation packet](../pair02-validation/README.md) separately
retains the fresh final-candidate offline art audit, 1,963 release tests, nine
focused tests, eight actual 31-day original-path comparisons and passing native
preflights, along with every earlier failed validation attempt. Those preparatory
results were not substituted for any declared pair02 cell.

## Acceptance scope

This is the complete passing measurement pair for the declared workstation,
browser configuration, France/seed 1990 and three actual campaign anchors. It
does not establish performance for all countries or hardware, or the 1990
startup. The browser late anchor is 30 November 2035; native windows advance to
31 December. The local control endpoint is paint opportunity plus GL completion,
not physical display/scanout. GL buffer and declared texture payload accounting
is not physical VRAM; sampled private bytes can miss instantaneous peaks.

Recorded lineage hashes and dates were independently reviewed, not independently
replayed for every intervening year. The released fighter inspection does not
imply campaign ownership of that design. Cold loading has separate observations
without an invented ceiling. This packet does not award G5, CP1, worldwide
historical/portrait coverage or human-playtest completion. Canonical closeout
decisions are recorded separately from these immutable measurement artifacts.

## Complete lossless preservation

[manifest.json](manifest.json) maps every original absolute artifact path, raw
size and SHA-256 to exact encoded payloads. It covers **all 636 original pair
files (6,456,235,037 bytes)**, final runner records, frozen config/manifest/shared
registry snapshot, hardware, exact harness/probe/UI sources, reviewed lineage
and prerequisite provenance. Unique gzip bytes, all twelve browser traces,
screenshots, profiles, memory CSV, observations, logs and save copies remain
recoverable. The shared registry is the exact pair02-completion snapshot.

The complete mapping has **782 original-path records**, **559 unique payloads**
and **565 chunks**, totalling **1,504,014,691 encoded bytes**.
Objects are deduplicated only when their encoded bytes are identical. Large
payloads are split into ordered chunks of at most 90 MiB, below GitHub's file
limit. The manifest records each chunk hash, combined encoded hash and decoded
original hash. Metadata-distinct saves retain their distinct raw bytes.

After the parent explicitly confirmed independent verification had finished,
the authorized preservation operation removed exactly 42 redundant raw copies:
six native inputs, twelve browser inputs and twenty-four browser before/after
saves. Every gzip was fully decompressed and matched against the original raw
SHA-256 and size. A complete restore plan, original inventory and frozen records
were written before removal, followed by a fresh raw hash/size check and a
resolved-path check for each individual PowerShell `Remove-Item -LiteralPath`.
All selected paths were inside the exact pair02 directory.

The operation reclaimed **4,737,930,442 bytes**, raising free space from
4,448,030,720 to 9,185,087,488 bytes. The manifest preserves all individual
receipts, original timestamps and gzip restore paths under
`s22-pair02-preservation-01`. Canonical sources, executable pins, unique compressed
files, prior failed pair01 and all other raw artifacts were untouched.

Four canonical input bodies reference previously verified immutable Git gzip
archives and were decompressed/hash-checked again during packaging. Two frozen
runtime executable bodies remain external with exact size/hash pins; they are
explicitly excluded from Git and from archive-body verification.

## Verification and restoration

From a checkout containing this packet:

```text
node docs/campaign-certification/S22/qualification-pair-02/archive.cjs
```

The command verifies every chunk, encoded payload, decoded original, canonical
gzip reference, full original pair inventory and readable record mirror without
restoring raw files. The [archive verification](records/archive-verification.json)
records its exact counts and manifest hash: all 782 records, 587 distinct
decoded encodings and all 636 original pair artifacts verified. Every packet Git blob is additionally
read and SHA-256 matched to the working bytes before commit. These are byte
integrity checks, not fresh game qualification runs.

To reconstruct the artifacts into a **new, nonexistent** directory:

```text
node docs/campaign-certification/S22/qualification-pair-02/archive.cjs --restore D:/s22-pair02-restored
```

The helper validates the entire packet first, then writes create-only outputs
beneath that destination. `C:\original\path` maps to
`D:/s22-pair02-restored/C/original/path`; `restore-map.json` lists every mapping.
It never writes live original paths or restores excluded executable bodies.
Allow at least 9 GiB of free space. Full disk restoration has not been executed;
every would-be output's decoded bytes have been verified independently.

The frozen verifier requires the original recorded absolute paths and pinned
external binaries. For a later verifier read, stage exact verified bytes at those
paths in an isolated environment; do not rewrite the frozen manifest or overwrite
current project files. Its retained expected result is passed. The restore
helper reports that recorded result separately from its own byte-integrity check.
