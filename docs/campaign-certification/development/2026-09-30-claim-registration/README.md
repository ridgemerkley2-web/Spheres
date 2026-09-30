# Komeito claim registration and ledger refresh

The 30 September branch check found a new authored claim at
`52b23d5961ea13355ab1e3edd497be0920b6bea6` on `claude/c01-jp-31`, based on
`ff01ee61`. It contains only a handoff, with no research delivery or executed
source checks. The integration checkout was first synchronized to `aba3902b`,
preserving the independently completed repository cleanup and four-cell matrix
audit. This follow-up was prepared in a separate task checkout.

C01-31 is registered as **claimed**, with priority 24. The authored handoff is
preserved as an exact text prefix with an explicit registration addendum. The
gap ledger reserves `Japan/jp_komeito`, `Japan/jp_komeito/jp_komei_1994` and
`Japan/jp_komeito/jp_komeito_1998`, removing exactly those three targets from
unclaimed next batches. The total next-item count changes from 97 to 94 because
work is reserved, not because historical coverage improved. All previous 39
queue rows, completed intakes, source-repair records and coverage totals remain
unchanged. No sources, leaders, runtime mappings or portraits are accepted here.

The initial gap-ledger check on `aba3902b` failed because the matrix task's
updated next-action text changed the recorded whole-queue byte count and SHA.
Only that input record differed in a fresh generation. The original failure is
retained in `validation/baseline-ledger-check.json`. Regeneration repairs it
while retaining the current complete-input provenance contract.

## Preserved gap-ledger follow-up

The unmerged `claude/c01-gaps-01-fix` tip
`1aa670479758a1320701639db28807fb3553adba` was independently reviewed and is
**not adopted**. It is an optional selective-hashing optimization based on
`846df479`, not part of the earlier accepted `b4d95396` / `12ee48f1` / `157aac55`
gap-audit chain. Its projection records only task id, state and branch; current
completion logic also reads review revisions. This is an incomplete description
of today's consumed inputs, not a demonstrated false-freshness defect: changed
review revisions also affect other generated output. Its old generated ledger
and handoff snapshot are obsolete. Keep the complete queue pin and ordinary
regeneration. A future selective projection would need an updated field contract
and tests. The exact branch tip is preserved; none of its five files is imported.

## Checks

- **65 focused metadata tests and 69 planning tests pass.**
- Workboard, claim query, gap-ledger, research-index and boundary-matrix checks
  pass. The board now contains forty bounded tasks and the same 44 canonical
  markers. S23 preparation remains 8,127 cases; it is not historical acceptance.
- Independent review confirms the exact three-target reservation, prior-record
  preservation, explicit claimed/ready-for-review distinction and unchanged
  rejection of unaccepted or duplicate work. No tests were weakened.
- All 48 retired Claude tips from the preceding remote inventory match exact
  recovery tags. The three remaining research branches were reread from origin;
  Russia C01-28 remains source-held.

The live four-worker campaign and its already armed verifier continue unchanged.
Six of the original eight bounded tasks remain complete; the full matrix and A1
remain open. No canonical session or certification status is changed here.

`validation/` retains the commands and their outputs; `manifest.json` pins this
packet's payloads. This record accepts claim bookkeeping, not Claude's eventual
research submission.
