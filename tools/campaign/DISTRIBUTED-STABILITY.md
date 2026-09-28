# Distributed full campaign evidence

The fixed `stability-full.json` still contains all 24 country/seed cells, each
settled through 31 December 2035 and the subsequent sandbox day. Distribution
changes the execution location only. It does not change gameplay, simulation
history, comparison schedules, invariants, seeds or acceptance thresholds.

`.github/workflows/stability-full.yml` runs only on deliberate `codex/s25-run-*`
branch pushes or manual dispatch. It builds one clean native test executable,
freezes its actual bytes with the whole plan and both Python helpers, then runs
one assigned cell on each standard Ubuntu runner. Every assignment is bound to
that exact central manifest SHA, including its workflow attempt identity. A
retry needs a new full batch; successful cells from separate batches cannot be
combined. Failed or interrupted cells retain their artifacts and remain failed.

Each shard's ordinary matrix `passed` and `full_matrix_passed` stay **false**.
`selected_cell_passed` reports only its single assigned case. Twenty-four green
jobs alone do not establish a verified matrix. The independent aggregate step
must read all retained files and decompress every complete campaign archive.
It rejects missing/extra/retried cells, changed archives, different builds,
different plans, different frozen helpers and mixed batches.

Download the central `full-stability-frozen-ATTEMPT` artifact into `frozen/` and
each `full-stability-cell-ID-ATTEMPT` artifact into `shards/ID/`. Preserve every
file. Verify the central manifest against the SHA reported by the freeze job,
then run the **downloaded frozen helper**:

```text
python -B frozen/distributed_stability.py aggregate --bundle frozen --manifest-sha256 SHA_FROM_FREEZE_JOB --shards shards --out NEW-aggregate-result.json
```

The output path must be new. Keep the artifact ZIP hashes, GitHub run/attempt
URL, build log, original archives and aggregate receipt. The workflow retains
artifacts for seven days; copy them to durable local evidence storage before
expiry. The local aggregate reads archives without requiring another giant raw
restore copy. Original save hashes and restore mappings remain available.

Native execution has a five-hour per-cell timeout inside a six-hour runner job,
leaving time to retain failed evidence. A timeout is an incomplete cell, never
a shortened pass. If this workload exceeds those bounds, preserve the attempt
and adjust execution resources under a new freeze. The optional local managed
scratch runner provides a separate storage option; do not mix its results into
an existing distributed batch.

Synthetic unit tests validate orchestration only. A passing full aggregate is
engineering evidence, not S25 or CP1 qualification. The controlled USSR-to-Russia
proof, political A1 gate and all canonical campaign prerequisites stay separate.
