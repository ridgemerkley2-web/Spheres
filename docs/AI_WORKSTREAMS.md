# Work assignments

**Updated 1 October 2026 · Integration branch: `codex/campaign-certification`.**

Start with [the roadmap](../ROADMAP.md) for priorities. S01–S22 are complete;
S23 is next and needs C06. G4 is earned; G5, CP1 and worldwide character coverage
remain open. Canonical session status lives in
[campaign-pathway.json](planning/campaign-pathway.json).

## Work happening now

| Owner | Work | Next action |
|---|---|---|
| Claude | Historical research and cartoon production | Komeito C01-31, French PMs C01-38, CPI(M) C01-40 and CPSU C01-41 are accepted bounded research intake. The queue records 26 completed bounded Claude tasks and two held submissions: Russia C01-28 and DA C01-39. [Current art direction, exact remote tips and next content sequence](planning/ai-handoffs/CLAUDE-CAMPAIGN-LEADER-ART.md): use actual dated campaign leaders; national representative figures are retired. Next production: reconcile Tonga identities, then a reviewed 6–8-cartoon batch and further C06 country casts. |
| Codex | 24-cell campaign matrix (`CODEX-S25-MATRIX-01`) | [30 September terminal failure](campaign-certification/S25/preparation/local-matrix-20260930/README.md): 4 revalidated passes, 4 abnormal exits, 16 not started; verifier failed. Diagnose crashes and journal-write failure before a newly declared complete attempt. |
| Codex | Political calibration (`CODEX-S27-A1-01`) | [Two correctness repairs](campaign-certification/S27/preparation/political-repairs-20260930/README.md) pass 1,990 ordinary native tests; A1 still fails at 0.571429. Urgent-response policy rejected. Next complete standalone timing and review residual concentration; retain all original limits. |
| Claude / Codex review | Russia research (`CLAUDE-C01-28`) | [Resumed review](campaign-certification/C01/reviews/CLAUDE-C01-28/2026-10-01-missing-only/README.md): 65/68 originals, 133/137 claims and all 24 holder observations materially checked. Three originals and four claims remain held; no Russia research is imported. |
| Claude / Codex review | DA research (`CLAUDE-C01-39`) | [Held review of `9cb02c20`](campaign-certification/C01/reviews/CLAUDE-C01-39/README.md): 1/24 originals and 1/29 claims reviewed; all eleven submitted holder observations held. Latest follow-up `b66f8c07` adds a twelfth observation and proposes an effective end; its content remains unaccepted. The same 23 originals / 28 claims and all twelve latest holder observations remain held. One request returned HTTP 429; 22 were not attempted. |
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

The branch inventory was checked on 1 October. These are the deliberate
exceptions to the one game integration branch:

| Branch | Purpose and disposition |
|---|---|
| `claude/c01-jp-31` | Komeito at `696937ba` is accepted as bounded intake: all 31 originals, 64 claims and fifteen holder observations reviewed. First import `b74f4fa5` and [acceptance receipt `d4d3542b`](campaign-certification/C01/reviews/CLAUDE-C01-31-resumed-20261001/README.md) preserve the Takeya boundary correction, three locator repairs and earlier failed checkpoints. Do not duplicate the accepted packet. |
| `claude/c01-za-39` | Latest `b66f8c07` follows the reviewed `9cb02c20` submission. Correction `9e3d0b3c` adds a Zille 2007 observation, proposes Maimane's end as 23 October 2019 and repairs a quote apostrophe; these changes are not accepted. [Follow-up triage](campaign-certification/C01/reviews/PENDING-2026-10-01/README.md) is separate from the preserved original review. No DA research or test repair is imported. |
| `claude/c01-fr-38`, `claude/c01-in-40`, `claude/c01-su-41` | Accepted scoped imports and independent review receipts are integrated. Exact source, correction and acceptance revisions are in the [Claude handoff](planning/ai-handoffs/CLAUDE-CAMPAIGN-LEADER-ART.md) and queue. Fetch current integration before further work; no complete country cast is claimed. |
| `claude/c01-ru-28` | Held Russia submission at `03141c43`; three originals / four claims remain unresolved, with no remaining holder-row dependency. Preserve all prior content review and failed attempts; import no held research. |
| `claude/c01-gaps-01-fix` | Follow-up `1aa67047` reviewed and not adopted: retain current full-queue provenance and regenerate its metadata. Its older projection and generated payload are not imported. Preserve the exact tip; the earlier bounded gap-audit task remains accepted. |
| `dashboard` | Publishes the existing GitHub Pages status site and research-pipeline status. It is not game code. |

The original 145-branch inventory, recovery tags and ancestor comparisons are in
[the branch archive](archive/branches-2026-09-30.json). Archived branches are not
all literal Git merges: reviewed/cherry-picked work and older experiments retain
their exact tips without being imported into the game again.

## Handoffs and evidence

- [1 October pending-acceptance review: Komeito accepted; Russia and DA held](campaign-certification/C01/reviews/PENDING-2026-10-01/README.md)
- [Earlier 1 October checkpoint: accepted research, then-held Japan review and queue registration](campaign-certification/C01/integrations/REVIEW-2026-10-01/README.md)

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
