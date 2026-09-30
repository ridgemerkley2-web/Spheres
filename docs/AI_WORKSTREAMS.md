# Work assignments

**Updated 30 September 2026 · Integration branch: `codex/campaign-certification`.**

Start with [the roadmap](../ROADMAP.md) for priorities. S01–S22 are complete;
S23 is next and needs C06. G4 is earned; G5, CP1 and worldwide character coverage
remain open. Canonical session status lives in
[campaign-pathway.json](planning/campaign-pathway.json).

## Work happening now

| Owner | Work | Next action |
|---|---|---|
| Claude | Historical research and cartoon production | Continue existing claims; reconcile accepted Tonga research into dated identities, then a reviewed 6–8-cartoon batch. Repeat country batches toward C06. |
| Codex | 24-cell campaign matrix (`CODEX-S25-MATRIX-01`) | Collect all results of the fresh `68ba0622` attempt and run independent retained-evidence verification. No full-horizon pass is recorded. |
| Codex | Political calibration (`CODEX-S27-A1-01`) | Blocked pending new causal evidence or a reviewed model contract. Keep the failed A1 gate and its original limits. |
| Claude / Codex review | Russia research (`CLAUDE-C01-28`) | Twelve unavailable originals still hold 24 claims and four holder observations. Preserve the accessible-content review; import no unaccepted Russia research. |
| Human / Codex support | Independent playtests | Prepare S26's participant and task protocol; formal qualification follows S24. |

The [task queue](planning/ai-task-queue.json) records each bounded task's owner,
priority, state, dependencies and evidence. [Workstream assignments](planning/ai-workstreams.json)
cover the remaining canonical sessions. Use the query tool instead of copying
long task histories into the front page:

```sh
python tools/planning/workboard.py --tasks --owner Codex
python tools/planning/workboard.py --tasks --owner Claude
python tools/planning/workboard.py --session S23
python tools/planning/workboard.py --check
```

## Existing work to preserve

The branch inventory was checked on 30 September. These are the deliberate
exceptions to the one game integration branch:

| Branch | Purpose and disposition |
|---|---|
| `claude/c01-jp-31` | Active Komeito research claim at `52b23d59`; its packet is on that branch and is not yet in the integrated task queue. Continue the existing claim. |
| `claude/c01-ru-28` | Held Russia submission; source-access and review requirements remain open. |
| `claude/c01-gaps-01-fix` | Preserve the unmerged `1aa67047` gap-ledger follow-up until its exact disposition is reviewed. The earlier bounded gap-audit task is already accepted. |
| `dashboard` | Publishes the existing GitHub Pages status site and research-pipeline status. It is not game code. |

The original 145-branch inventory, recovery tags and ancestor comparisons are in
[the branch archive](archive/branches-2026-09-30.json). Archived branches are not
all literal Git merges: reviewed/cherry-picked work and older experiments retain
their exact tips without being imported into the game again.

## Handoffs and evidence

- [Research next tasks](planning/ai-handoffs/CLAUDE-C01-NEXT.md)
- [Character/art preparation](planning/ai-handoffs/CLAUDE-EXPANDED-NEXT.md)
- [Engineering next tasks](planning/ai-handoffs/CODEX-NEXT-ENGINEERING.md)
- [Engineering preparation scope](planning/ai-handoffs/CODEX-INDEPENDENT-ENGINEERING.md)
- [Complete campaign pathway](CERTIFIED_CAMPAIGN_PATHWAY.md)
- [Previous detailed workboard and acceptance history](archive/2026-09-30/AI_WORKSTREAMS.md)

Six of the earlier eight bounded engineering/review tasks are complete; A1 and
the full matrix remain open. Worldwide startup, controlled succession,
recovery and packaging preparation retain their accepted evidence. These bounded
results do not bypass canonical session dependencies.

## Integration discipline

Fetch the integration branch, use an isolated checkout, and claim one bounded
packet. Preserve others' claims, saves and uncommitted files. Coordinate edits to
shared runtime, save schemas, command dispatch, release workflows and central
planning records. Return exact commits, checks, evidence and known gaps.

Codex reviews and integrates the accepted result, then updates the relevant task
record and runs `workboard.py --check`. Never force-push the integration branch.
`ready_for_review` is not `complete`; only actual acceptance evidence closes a
session. Keep research, runtime installation, artwork, human testing and release
qualification as separate obligations.
