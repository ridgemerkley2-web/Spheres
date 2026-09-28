# S22 idle import repair and isolated preflights

Candidate **`9823b070efea06a4f6e63e955d8ae23445108054`** passed its complete
release workspace regression and actual-2015 import equivalence check. Both
subsequent isolated native preflights **failed latency requirements**. Their
observed memory values passed the existing limits. **Qualification is false;
S22, G5 and CP1 remain open.**

This append-only packet contains the original outputs and provenance. It makes
no claim of a frozen qualification round or performance improvement. Packaging
overlapped a later diagnostic; the two preflights had already finished. That
separate diagnostic was still incomplete at this packet's cutoff and is not
included.

## Regression and repair evidence

- `cargo test --locked --release --workspace -- --test-threads=2`: **1,922
  passed, 0 failed, 96 ignored across 66 suite summaries**. Counts were
  independently summed from the retained test log during packaging.
- Actual-2015 import oracle: **1 passed**, covering 31 complete simulation days
  against the original import path, including world/headline/financial-bit
  equality and successful guarded purchases. This run uses complete simulation
  ticks without player annual budget renewal, and is functional equivalence
  evidence rather than a benchmark. Its provenance records overlap with the
  workspace regression.
- The original compilation failure is retained: the added fixture referred to
  an unavailable `enabled` helper. The corrected build and final regression
  logs are separate files; the failed build was not overwritten or counted as
  successful coverage.

`runtime/provenance.json` records the immutable native-test and server copies.
The native test binary SHA256 is
`7b4039626cfe75fa151b7f46a28053bb9f90e2a2b648d8ae5ad0bcdefe0ada07`.
The actual-import oracle's simulation binary hash is recorded separately in its
own provenance. No executable is duplicated in this packet.

## Completed isolated native preflights

| Actual input | Simulation/history p95 | Whole-turn p95 | Whole-turn maximum | Verdict |
| --- | ---: | ---: | ---: | --- |
| 2015-01-01 | 356.2455 ms | 446.365 ms | 679.2651 ms | Both p95 limits failed |
| 2035-11-30 | 452.9066 ms | 554.1808 ms | 13,964.7566 ms | Both p95 limits and maximum failed |

The unchanged limits are 300 ms simulation/history p95, 400 ms whole-turn p95
and 750 ms whole-turn maximum. The table rounds only the displayed floating
representation; exact raw values remain in the profiles, runner results and
manifest. Both attempts contain all 31 consecutive daily samples. Raw sample
arithmetic was checked against their retained summaries during packaging.

| Actual input | Maximum sampled private bytes | Observed OS working-set peak | Memory limit result |
| --- | ---: | ---: | --- |
| 2015-01-01 | 862,003,200 | 858,652,672 | Both below 1,073,741,824 bytes |
| 2035-11-30 | 904,888,320 | 900,075,520 | Both below 1,073,741,824 bytes |

These are the isolated native process envelope observations, including load and
report work. They are not browser memory, driver VRAM or a claim about steady
server memory. Sampling gaps and failed sample counts remain in the raw reports.

Each native test exited zero and its profile reports functional completion:
unchanged source, real daily advancement, and matching final facts from the
separately loaded batch. **The wrapper still correctly reports `passed:false`
and `numerical_acceptance:false`** because latency failed. Profile completion
does not override that failure.

## Reproduction and preservation

`preflights/2015/` and `preflights/2035/` each contain the original invocation,
runner result, complete profile, stdout/stderr, wrapper console and memory CSV.
The two CSVs are small (25,635 and 45,736 bytes), so their exact original bytes
are retained without compression. Empty stderr files are retained too.

Inputs are referenced, not duplicated:

- [Actual 2015 input manifest](../stock-fix-progress/manifest.json):
  uncompressed SHA256
  `e786ffb67a26c6ae28bd917d5d8dc26f857e441dd6a58d60adc1a1107937ca11`.
- [Actual 2035 input manifest](../late-input-progress/inputs/reviewed-late-input-manifest.json):
  uncompressed SHA256
  `67aadca2f55280abc0e4ad97944a654ec7d173ffb65cb7d71dd59090e10503cb`.
- [Actual-2015 extraction record](../route-ui-progress/actual-transfer-parity/extraction.json)
  connects the oracle's integrated-world input to the preserved campaign.

The prior compressed input hashes and extraction-record bytes were verified.
`manifest.json` lists each captured artifact's original absolute path, byte
count and SHA256. `.gitattributes` prevents newline conversion. No original
input, failed attempt, existing packet or canonical session status was changed.
