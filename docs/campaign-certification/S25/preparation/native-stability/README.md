# Paired campaign stability runner and pilot

Bounded task: **CODEX-S25-RUNNER-01 complete**. This delivers the runner and
the declared annual-boundary pilot. It does **not** complete canonical S25,
the 24-cell 1990–2035 matrix, the controlled USSR-to-Russia case, G5 or CP1.

## Actual campaign evidence

The clean native test executable was compiled from
`0106ecedfb4968bf3b10d55768a5ece08713882b`; its SHA-256 is
`cdde22f6b845845e654fe2d7e940c3214d2d27d4f38e8afa781bfa459f1b9968`.
The original Python runner is frozen inside the evidence with its own hash.
Both games in each pair start through the ordinary new-game path, adopt
economic competition through a legal command and renew existing budgets.
There are no cash grants, date edits or patched campaign states.

| Country / seed | Daily advances per leg | Final native date | Complete archive comparisons | Result |
|---|---:|---|---:|---|
| France / 1990 | 398 | 3 February 1991 | 30 | Passed |
| Tonga / 7 | 398 | 3 February 1991 | 30 | Passed |

Together these are 1,592 actual daily advances, 26 monthly reloads, four
terminal reloads, 1,592 daily invariant checks and 120 full native-validator
checks. The continuous and resumed campaigns match in every serialized byte
except the envelope's terminal wall-clock `saved_unix` field. Finance,
history, event logs, leadership, equipment and campaign metadata remain part
of the comparison. January renewal and the day after each monthly reload are
checked explicitly. The terminal save must also survive loading unchanged.

The [original run](evidence/pilot-01/result.json) retains the frozen request,
runner, per-cell process output, command and comparison records, binary pins,
final saves and last monthly recovery pair. Eight gzip files restore the exact
original native archive bytes. The [independent retained-evidence check](evidence/retained-verification.json)
rehashes the retained inputs, decompresses archives as streams, recomputes both
SHA-256 and native fingerprints, and repeats the report and coverage checks.
It does not launch a simulator or substitute generated fixtures for this run.

## Verification and reuse

From the repository root:

```text
python -B -m unittest discover -s tools/campaign -p test_stability_matrix.py
python -B tools/campaign/stability_matrix.py --verify docs/campaign-certification/S25/preparation/native-stability/evidence/pilot-01
```

Add `--restore <new-directory>` to the second command for a complete restored
bundle. Existing destinations are refused; retained evidence is not changed.

To execute another pilot, first compile the native release test executable from
a clean committed candidate, then use its actual path and full commit:

```text
python -B tools/campaign/stability_matrix.py --binary <native-test-executable> --revision <40-character-commit> --plan tools/campaign/stability-pilot.json --out <new-directory>
```

The full plan is `tools/campaign/stability-full.json`: eight countries times
three seeds, inclusive through 31 December 2035, followed by ordinary horizon
acknowledgment and a sandbox day. It is a separate, lengthy execution and has
**not been run here**. The verifier rejects subsets, duplicate cases, changed
identity, shortened dates, missing comparisons and zero-test successes.

There are 35 synthetic runner/retention tests and five native runner regressions.
The full UI rerun passed 1,780 tests with one optional browser check skipped.
The earlier failed UI attempt is retained: its standalone save-list fixture
omitted the shipped label helper; the repaired fixture loads the real helper
and adds a backup-only regression. The passing rerun does not erase that attempt.

The existing political calibration A1 concentration failure remains open under
its unchanged limit, independently of this deterministic save/resume pilot.
S23 and the canonical S24/S25 prerequisites also remain unchanged.
