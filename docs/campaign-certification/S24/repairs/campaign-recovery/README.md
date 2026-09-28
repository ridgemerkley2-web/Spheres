# Campaign recovery — completed engineering repair

**CODEX-S24-RECOVERY-01 is complete.** This repairs three reproduced storage
defects and verifies recovery during five kinds of active work. It does not
complete S24's worldwide startup/character qualification or award CP1.

## What changed

- Campaigns lists a surviving backup even when the primary save is missing.
  It shows the backup's date and an explanation outside the selector, including
  at 390px. Ordinary Load is disabled for a missing/unreadable primary; explicit
  backup loading still performs complete validation.
- Repeating an unchanged save preserves the previous recovery point. Only the
  native envelope's terminal wall-clock timestamp is ignored. A first repeated
  save can establish a backup; changed world/history/log/journey still rotates it.
- Failed exclusive creation no longer deletes a temporary file that this save
  operation did not create. Backup promotion failures preserve the primary.

## Verification

| Check | Result |
|---|---|
| Native web suite at `910c5bda` | 440 passed, 0 failed, 26 existing opt-in tests ignored; asset-dimensions executable: 1 passed |
| Focused shipped menu/session/dialog UI | 64 passed, 0 failed |
| Ordinary France save/backup browser | Passed at 1440px and 390px; actual committed save reply lost, retry preserves older backup; missing primary recovered through Campaigns; old-session save refused |
| Five active-work browser cases | 5/5 passed, with 20 complete archive comparisons and desktop/narrow retry controls |
| Independent evidence review | All 46 matrix snapshots/hashes, 20 comparisons, five source/manifest pins and 12 screenshots checked; zero page errors |
| Portable packet verification | 93 original artifacts restored exactly from 1,779 shared chunks |

The final browser executable is candidate
`de50bc5c1897a39cf3cd358b47486e58349da9a4`. The five-boundary driver includes the
`34d6f2b3` transmission-context correction. The production storage code is the
same as the native passing run; later changes improve the menu explanation and
test drivers. Every result retains executable, driver and embedded UI hashes.

The actual active-work transitions were:

| Boundary | Observed progress on the lost-response day |
|---|---|
| Construction | Paid foundation work increased from $0.002bn / day 1 to $0.004bn / day 2 |
| Delivery | Two paid aircraft arrived; holdings increased by exactly two |
| Refit | Settled conversion work advanced from 7 to 8 days |
| Rebasing | Four assigned aircraft arrived at the consenting British base |
| Missions | Both queued orders flew on day 814, consuming 2.8891 and 2.1897 stores |

For each case, the full committed campaign stayed identical after browser
reload, receipt retry, named Load and stale-request refusal, excluding only the
terminal `saved_unix` value. Normal continuation then settled one further day.
The initial S11/S15 campaigns are explicitly authored fixtures with disclosed
opening conditions; no new grants, date patches or hand-edited state were used.
The ordinary France backup test starts through New Campaign instead.

## Retained failures and restoration

The packet includes the initial test compilation mistake, four reproduced
storage failures, and test-fixture corrections: legacy/default rules and an
isolated GDP mutation triggered legitimate load migration/debt-ratio repair.
The fixtures now use current playable initialization and an independent political
stock; exact comparisons were retained. Native attempt 01's three fixture
failures remain visible beside the complete passing rerun.

The first matrix browser attempt failed because its driver compared a stored
turn to a transmitted body without accounting for the normal API wrapper's
`player_context` field. The corrected driver still compares the actual original
and retry HTTP bodies exactly. Both attempts are retained. The first passing
backup run prompted the additional visible narrow-screen explanation.

Large artifacts use lossless binary chunks; even original gzip streams restore
byte-for-byte. From the repository root:

```powershell
python -B tools/campaign/verify_recovery_evidence.py docs/campaign-certification/S24/repairs/campaign-recovery/artifacts
python -B tools/campaign/verify_recovery_evidence.py docs/campaign-certification/S24/repairs/campaign-recovery/artifacts --restore ../recovery-restored-review
```

The second command requires a new directory. Its `recovery-browser-02/result.json`
and `recovery-matrix-02/result.json` contain the passing receipts. Original
absolute paths identify the disposable run; the packet manifest maps every
retained file to its portable restoration path. Five-boundary inputs and their
provenance remain pinned in `tools/ui/recovery-boundaries.json`.

No user campaign, historical roster, game balance, economic formula or company
research changed. Git was checked after recovery; no newer Claude heads appeared.
