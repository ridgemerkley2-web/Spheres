# Work assignments

**Updated 30 September 2026 · Integration branch: `codex/campaign-certification`.**

Start with [the roadmap](../ROADMAP.md) for priorities. S01–S22 are complete;
S23 is next and needs C06. G4 is earned; G5, CP1 and worldwide character coverage
remain open. Canonical session status lives in
[campaign-pathway.json](planning/campaign-pathway.json).

## Work happening now

| Owner | Work | Next action |
|---|---|---|
| Claude | Historical research and cartoon production | Komeito C01-31 is delivered for review; Russia remains held. New remote claims C01-38–41 cover French PMs, DA, CPI(M) and CPSU leadership. [Current art direction, exact remote tips and next content sequence](planning/ai-handoffs/CLAUDE-CAMPAIGN-LEADER-ART.md): use actual dated campaign leaders; national representative figures are retired. Next production: reconcile Tonga identities, then a reviewed 6–8-cartoon batch and further C06 country casts. |
| Codex | 24-cell campaign matrix (`CODEX-S25-MATRIX-01`) | [30 September snapshot](campaign-certification/S25/preparation/local-matrix-20260930/README.md): 4 revalidated passes, 4 running, 16 queued. Collect the existing run and dependent verifier's final results. Full-matrix pass remains pending. |
| Codex | Political calibration (`CODEX-S27-A1-01`) | [Two correctness repairs](campaign-certification/S27/preparation/political-repairs-20260930/README.md) pass 1,990 ordinary native tests; A1 still fails at 0.571429. Urgent-response policy rejected. Next complete standalone timing and review residual concentration; retain all original limits. |
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
| `claude/c01-jp-31` | Komeito delivered for review, latest checked tip `696937ba`; the integrated queue still records its original claim. Keep the three organizational targets reserved. Independent Codex review remains pending; no acceptance or runtime installation is implied. |
| `claude/c01-fr-38`, `claude/c01-za-39`, `claude/c01-in-40`, `claude/c01-su-41` | New claim-only handoffs for French PMs, DA, CPI(M) and CPSU leadership, respectively. Exact tips and scope are in the [current Claude handoff](planning/ai-handoffs/CLAUDE-CAMPAIGN-LEADER-ART.md). Central registration and content delivery are still pending. Do not duplicate these claims. |
| `claude/c01-ru-28` | Held Russia submission; source-access and review requirements remain open. |
| `claude/c01-gaps-01-fix` | Follow-up `1aa67047` reviewed and not adopted: retain current full-queue provenance and regenerate its metadata. Its older projection and generated payload are not imported. Preserve the exact tip; the earlier bounded gap-audit task remains accepted. |
| `dashboard` | Publishes the existing GitHub Pages status site and research-pipeline status. It is not game code. |

The original 145-branch inventory, recovery tags and ancestor comparisons are in
[the branch archive](archive/branches-2026-09-30.json). Archived branches are not
all literal Git merges: reviewed/cherry-picked work and older experiments retain
their exact tips without being imported into the game again.

## Handoffs and evidence

- [30 September claim registration, ledger refresh and review checks](campaign-certification/development/2026-09-30-claim-registration/README.md)
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
