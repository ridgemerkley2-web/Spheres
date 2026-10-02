# Work assignments

**Updated 2 October 2026 · Integration branch: `codex/campaign-certification`.**

Start with [the roadmap](../ROADMAP.md) for priorities. S01–S22 are complete;
S23 is next and needs C06. G4 is earned; G5, CP1 and worldwide character coverage
remain open. Canonical session status lives in
[campaign-pathway.json](planning/campaign-pathway.json).

## CP1 execution priority

The user authorized the [parallel CP1 plan](planning/ai-handoffs/CP1-ACCELERATION.md).
Finish Tonga's full country cast first. Codex owns the stable task
`CLAUDE-C06-TONGA-01`. The [integrated cast](campaign-certification/C06/production/tonga/integration-20261001/README.md)
provides 42 appearances for 32 real people, four fictional civilians and separate
Crown/premier behavior. The remaining likeness, political chains and country
acceptance checks are recorded explicitly. Native crash diagnosis, political
balance and [real-player recruitment](campaign-certification/S26/preparation/RECRUITMENT.md)
remain separate tasks.
The final Russia and DA reviews preserve all earlier source-access failures.
Worldwide expansion and new systems remain after CP1; qualification criteria are unchanged.

The [Tonga PM follow-up](campaign-certification/C06/production/tonga/pm-transition-review-20261001/README.md)
adds reviewed 1991 appointment/retirement events and separates Vaea's 1999–2000
resignation report from his leave statement. Four existing portrait/date mappings
pass; no effective term boundaries or additional holder observations are installed.
Fatai Helu's targeted likeness follow-up remains unresolved. Those fourteen country requirements stay open; France registration adds a current-source browser refresh, bringing the current total to fifteen. Source findings do not count as completed role chains.

## Work happening now

| Owner | Work | Next action |
|---|---|---|
| Claude / Codex review | Historical research and cast batches | **All 38 earlier bounded research tasks remain complete. France’s seven-portrait batch is now accepted.** Six further batches have 31 generated portraits awaiting Claude review and registration. These do not close country or campaign gates. |
| Claude / Codex rendering | France registration and six cast batches | **France accepted; 31 active jobs rendered.** [2 October return package](campaign-certification/C06/render-return-20261002/README.md) contains unchanged originals, all attempts, full SHA-256 and exact source revisions. Brazil/Japan era replacements were used; superseded jobs were skipped. Claude now owns likeness/era review and coordinated registration for the six batches. |
| Codex | Tonga country cast (`CLAUDE-C06-TONGA-01`, stable task ID) | **In progress; fifteen requirements remain.** Tonga’s records and native acceptance are unchanged. [Previous browser evidence](campaign-certification/C06/production/tonga/ci-browser-acceptance-20261001/README.md) remains historical; the shared portrait manifest changed with France registration, so current-source browser refresh is pending. Fatai Helu likeness, eleven historical chains, final visual review and country signoff also remain open. |
| Codex | 24-cell campaign matrix (`CODEX-S25-MATRIX-01`) | [30 September terminal failure](campaign-certification/S25/preparation/local-matrix-20260930/README.md): 4 revalidated passes, 4 abnormal exits, 16 not started; verifier failed. The [runner journal repair](campaign-certification/S25/preparation/journal-diagnosis-20261001/README.md) is accepted with 105 tooling tests and one existing skip. Native access violations remain unexplained. The reviewed [Japan / seed 7 diagnostic](campaign-certification/S25/diagnostics/japan-7-20261001-01/README.md) started at 03:26 UTC on 1 October with dump capture and resource guards; read its external terminal result for current status. No replacement full matrix or verifier has been launched. |
| Codex | Political calibration (`CODEX-S27-A1-01`) | [Two correctness repairs](campaign-certification/S27/preparation/political-repairs-20260930/README.md) pass 1,990 ordinary native tests; A1 still fails at 0.571429. Urgent-response policy rejected. The [timing/A1 follow-up](campaign-certification/S27/preparation/a1-followup-20261001/README.md) records a 0.0623 ms/month resource assertion pass; quiet confirmation is still pending because of background app Git scans. Residual concentration remains unresolved; retain all original limits. |
| Claude / Codex review | Russia research (`CLAUDE-C01-28`) | **Accepted bounded intake.** All 68 originals, 137 claims and 24 holder observations are materially reviewed and the scoped research is imported. [Final missing-only review](campaign-certification/C01/reviews/CLAUDE-C01-28/2026-10-01-final-missing-only/README.md) preserves earlier failures and content decisions. This does not establish complete party histories or runtime identities. |
| Claude / Codex review | DA research (`CLAUDE-C01-39`) | **Accepted bounded intake.** All 24 originals, 29 claims and 12 proposed holder observations from latest `b66f8c07` are materially reviewed and the corrected scoped research is imported. [Final review](campaign-certification/C01/reviews/CLAUDE-C01-39/2026-10-01-accepted-03/README.md) preserves the earlier `9cb02c20` held receipt and all failed/unattempted checkpoints. No full chronology, runtime identity or country acceptance is implied. |
| Human / Codex support | Independent playtests | The [invitation and eight-session plan](campaign-certification/S26/preparation/RECRUITMENT.md) are ready. Recruit five actual independent first-time players; none are yet recorded. Formal qualification follows S24. |

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
| `claude/c01-za-39` | Latest `b66f8c07` is independently reviewed and accepted as a bounded intake with the corrections stated in the [final receipt](campaign-certification/C01/reviews/CLAUDE-C01-39/2026-10-01-accepted-03/README.md). All 24 originals, 29 claims and 12 proposed holder observations were read. The earlier `9cb02c20` held review and follow-up triage remain immutable; claimed user rulings in branch prose are not authorization. |
| `claude/c01-fr-38`, `claude/c01-in-40`, `claude/c01-su-41` | Accepted scoped imports and independent review receipts are integrated. India follow-up `d7e1b9d2` adds two verified originals and conservatively reselects two observations; its [separate amendment review](campaign-certification/C01/reviews/CLAUDE-C01-40-amendment-20261001/README.md) preserves all earlier evidence and unknown office boundaries. Exact source, correction and acceptance revisions are in the [Claude handoff](planning/ai-handoffs/CLAUDE-CAMPAIGN-LEADER-ART.md) and queue. Fetch current integration before further work; no complete country cast is claimed. |
| `claude/c01-ru-28` | Submission `03141c43` is accepted as bounded research after all 68 originals, 137 claims and 24 holder observations were reviewed. The [final missing-only receipt](campaign-certification/C01/reviews/CLAUDE-C01-28/2026-10-01-final-missing-only/README.md) resolves the three original-source holds while preserving every earlier attempt, correction and uncertainty. No complete-country or runtime mapping is granted. |
| `claude/c01-gaps-01-fix` | Follow-up `1aa67047` reviewed and not adopted: retain current full-queue provenance and regenerate its metadata. Its older projection and generated payload are not imported. Preserve the exact tip; the earlier bounded gap-audit task remains accepted. |
| `dashboard` | Publishes the existing GitHub Pages status site and research-pipeline status. It is not game code. |

The original 145-branch inventory, recovery tags and ancestor comparisons are in
[the branch archive](archive/branches-2026-09-30.json). Archived branches are not
all literal Git merges: reviewed/cherry-picked work and older experiments retain
their exact tips without being imported into the game again.

The next 1 October pass accepted Japan C01-42 at `94426852`, Brazil C01-43
at `68fb8f86`, and the resumed Tonga C01-44 packet at `61100640`. All original
content and proposed observations were reviewed; Tonga’s earlier failed receipt
remains intact. Saudi follow-up `5f083d7f` contributed only a stronger date guard,
with no historical change. The [current handoff](planning/ai-handoffs/CLAUDE-CAMPAIGN-LEADER-ART.md)
records exact source and review commits. These completions resolve the five
registered C01-42–46 claims as bounded research, without changing CP1 qualification. The
[final combined review](campaign-certification/C01/integrations/REVIEW-20261001-03/README.md)
also checks later Japan/Tonga follow-ups and excludes Brazil's proposed sixth
observation pending office-identity evidence; the five accepted observations remain.

The later 1 October pass accepts C01-47–51 as bounded research: **80 originals,
126 claims and 41 holder observations**. One ambiguous USSR Premier observation
and three Saudi meeting-only holder uses are withheld; their literal source facts
remain claims. France's exact-day assumptions and one locator are corrected,
and India's prior cutoff guards are restored. The [current handoff](planning/ai-handoffs/CLAUDE-CAMPAIGN-LEADER-ART.md)
pins all five submitted tips, first source imports and independent receipts.
At that checkpoint, the queue had 59 tasks and 36 completed Claude tasks;
all 44 canonical markers and the two older source holds were unchanged. Those
research intakes did not accept the separate Tonga runtime/art candidate or close
C01, C06, S23, S25, G5 or CP1. The final source reviews above supersede only the
two held-delivery statuses; Tonga production remains separately owned by Codex.

## Handoffs and evidence

- [Final 1 October delivery review: all 38 Claude-owned bounded tasks complete](campaign-certification/C01/integrations/REVIEW-20261001-06/README.md)

- [1 October CP1 preparation: two bounded tasks complete; production and recruitment queued](campaign-certification/development/2026-10-01-cp1-acceleration/README.md)
- [Earlier 1 October checkpoint: Komeito accepted; Russia and DA then held](campaign-certification/C01/reviews/PENDING-2026-10-01/README.md)
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

## Latest art handback, 2 October 2026

[France registration](campaign-certification/C06/render-return-20261002/france-registration-review.json) at `09cbb583` passed 776 checks. Seven additive registrations preserve all previous portraits and historical records; five derived exports have been regenerated. The [six-batch return](campaign-certification/C06/render-return-20261002/README.md) now supersedes earlier render-waiting status: Brazil 7, South Africa 4, Japan 7, India 8, Saudi Arabia 4, USSR / Russia 1. All 31 await Claude’s visual review and registration; they do not replace Tonga as the CP1 priority.

India, Saudi Arabia and USSR / Russia preparation pushes are now reviewed and rendered. Current tips, excluded reserves, source limits and source-licence obligations are preserved in their country return records. The earlier [Tonga PM2006 follow-up](campaign-certification/C06/production/tonga/pm2006-followup-20261002/README.md) remains a bounded source lead with no effective-date import or requirement closure.
