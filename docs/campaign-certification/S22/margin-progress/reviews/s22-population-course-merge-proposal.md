# Population course-merge source review and workload counts

Prepared by Codex subagent `/root/s20_preflight`, against source `697448c1`.
This is an offline read-only JSON inspection and source review, not a native
timing run, qualification result, or claim of measured savings.

Sources were read from the existing immutable preparation checkpoints:

- `s22-preparation-active-02/campaigns/saves/s22-2015-01-01.json`
- `s22-preparation-active-03/campaigns/saves/s22-2035-11-30.json`

The files were not changed. Their hashes and preparation provenance remain in
the canonical preparation evidence; this count inspection did not rerun hashing.

| Snapshot | Province rows | Retained province courses | Snapshot-only candidate merges | Incoming × existing comparison upper bound |
| --- | ---: | ---: | ---: | ---: |
| 2015-01-01 | 2,584 | 329,982 | 6 | 13,501,721 |
| 2035-11-30 | 2,584 | 438,538 | 4 | 17,373,912 |

Largest candidate source/destination course-vector lengths:

- 2015: Libya `LY-BA` → `LY-AJ`: 65 → 66,225; Iraq `IQ-AR` →
  `IQ-AN`: 66 → 49,451; Algeria `DZ-02` → `DZ-01`: 65 → 48,655;
  Israel `IL-HA` → `IL-D`: 65 → 42,562.
- 2035: Sao Tome `ST-S` → `ST-P`: 90 → 72,625; Libya `LY-BA` →
  `LY-AJ`: 66 → 79,256; Algeria `DZ-02` → `DZ-01`: 66 → 43,181;
  Iraq `IQ-AR` → `IQ-AN`: 66 → 41,770.

Candidate estimates applied the existing vacancy, unemployment-gap and room
conditions to the saved rows using a 1/365 year fraction. They did not perform
the preceding native demography, schooling, matching or migration mutations.
They therefore demonstrate substantial retained vectors and possible work,
not exact future tick call counts or actual comparison totals. Original
linear searches can also end before reaching each upper bound.

The measured parent-owned `s22-subsystem-2035-pass-local-01` diagnosis attributes
21.18 ms mean to population and 24.30 ms mean to industry. It does not separately
attribute course merging. Source inspection found the exact-key course merge
to be the smallest isolated quadratic operation worth addressing. Industry
was left unchanged because repeated contractor reads can cross actual XP,
stock, fiscal and power mutations.

The proposed implementation indexes only keys in the current incoming batch,
then scans the destination once to record each key's first existing position.
It preserves the original `(from, to, started_day, required_days.to_bits(),
funded_days.to_bits())` match; existing and new duplicate-first behavior;
every incoming row; append order; and each original `people_m +=` operation.
No course is truncated, coalesced beyond the existing rule, sorted, or cached
across merges. All other resident and household arithmetic is unchanged.

Required evidence before integration qualification: literal old-path tests
for the full population row and key bit patterns, full-world/native-day
equivalence, and separate actual 31-day 2015 and 2035 old-path oracles requiring
nonzero eliminated repeated scans. These tests make no timing assertions.
Root coordinates compilation, execution and any later isolated timing.
