# Portable January 1994 diagnostic evidence

This packet retains the **complete immutable input** and original records from the actual France diagnostic at frozen revision `ae8084e853a8eb01ef4d834017af3e94c8e2cc82`. It is performance-attribution evidence, not a new simulation run, performance qualification, completed S25 stability matrix, or campaign acceptance decision.

The original input is the real 1 January 1994 monthly save from the **interrupted** France/1990 full-matrix attempt:

`D:/spheres-offload/codex-next-20260928/matrix-full-ssd-02-interrupted/cells/france-1990/native/resumed/saves/monthly.json`

The diagnostic itself ran from a byte-identical private input copy, through 31 actual native days, and ended on 1 February 1994. Its independent observed schedule and ordinary `Game` schedule recorded equal complete native-world serializations and returned headlines on every day. The diagnostic is not equality of two complete campaign envelopes: only the ordinary `Game` owns the campaign history/log wrapper. The broader S25 harness has its own monthly, next-day, mandatory and terminal comparison boundaries.

## Contents

- `input/france-1994-01-01.json.gz`: the **entire** original 135,739,703-byte save, losslessly compressed with gzip mtime zero and no original filename header. Its decoded SHA-256 is `5480ccc296c2fe67ef2234871d1ef5fc9f098184ca1c83d885b07010a1368b20`. No cohorts, histories, fields or numerical values were omitted or normalized.
- `original-run/`: exact original `execution-start.json`, `execution.json`, `profile.json`, `process.csv`, `stdout.log` and `stderr.log`.
- `profile-1994.ps1`: the exact original Windows execution script. It retains its historical absolute paths and is included as provenance, **not** invoked by verification.
- `review/`: the entire original independent review packet, byte-for-byte, including the corrected reviewer-only monotonic-history assertion attempt. Its absolute paths are historical provenance; portable verification uses the local copies. The review's statement that the input remains external applies to that earlier compact packet; this outer packet now includes the whole compressed input.
- `input-provenance/world.rs`: exact frozen source explaining the omitted native `day` field's `first_day() = 1` default. No native loader is executed by the verifier.
- `build-record.json`: source/copy before-and-after SHA-256 pins, decoded round-trip pin, exact script source pin, compressor version and creation receipt.
- `manifest.json`: SHA-256 and length inventory of **every payload file**, including this README and verifier. The only self-exclusions are `manifest.json` and its detached `manifest.sha256`.
- `verify_portable.py`: standalone Python-standard-library, read-only verifier. It never invokes a game binary, Cargo, browser, PowerShell execution script or network request; it does not extract an uncompressed file or write a report.

The frozen executable is **not included or executed**. Its reference identity is 353,133,206 bytes, SHA-256 `e90b8f0cb34603f5290c8a19b106a0d6dc31e3ea7ad293eb0135199fe1fd59e2`. The original run receipt and embedded revision bind that executable to the declared source; this packet is not a reproducible-build attestation.

## Verify after copying the directory

Use Python 3.9 or newer. Obtain the expected manifest SHA-256 from the sender's separate message or sibling `native-profile-1994-portable-01.manifest.sha256` file through the trusted transfer channel. The copy of that hash inside the packet is a consistency check, not an independent trust anchor.

```text
python -B /path/to/packet/verify_portable.py /path/to/packet --manifest-sha256 EXPECTED_64_HEX_DIGEST
```

On Windows the same command accepts ordinary quoted Windows paths. The verifier prints JSON and exits 0 only on success; failures print `passed: false` and exit 1. It checks:

1. The manifest against the externally supplied digest and every payload's exact bytes, including the original source/script/review copies; absent, extra, duplicate or linked payload paths fail.
2. The actual entire decoded gzip stream against its 135,739,703-byte/SHA-256 pin, then ordinary JSON input identity, seed, calendar and retained history/log counts. Gzip metadata must have zero mtime and no optional original filename.
3. Original execution start/final receipts, binary/input reference pins, exact test invocation, zero process exit/no timeout, ordinary budget renewal and required-capability settings, and retained passed test stdout.
4. All 31 recorded equality flags, lengths, real dates, null comparison/validation errors and corresponding stderr confirmations, plus process-sample chronology.

Successful verification proves integrity and consistency of the **recorded observations**, not an independent replay of the worlds. Output explicitly reports `native_worlds_independently_replayed: false`, `binary_executed: false` and `qualification: false`.

## Interpretation limits

Ordinary 2–30 January native ticks averaged 87.340 ms; annual 1 January was 166.406 ms and month-end 31 January was 180.826 ms. The diagnostic compares both complete worlds **every day**, spending 16.193 seconds in comparison clocks and a further 23.841 seconds outside the reported clocks. The 45.919-second outer wall time must **not** be extrapolated to S25 matrix ETA, because S25 uses different comparison/save/reload cadence and full campaign envelopes. Read `review/README.md` for exclusive versus nested subsystem timings and observed-process memory/CPU limits.

The 172 process samples concern a diagnostic holding two evolving worlds and comparison buffers. They do not qualify a one-world production server's memory or hardware-wide utilization. No new policy, numerical coefficient, acceptance threshold, historical claim, gameplay change or native performance measurement was introduced while creating this portable packet.
