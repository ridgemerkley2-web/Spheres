# Spheres — Codex and Claude workboard

Updated 28 September 2026 UTC. **Integration branch: `codex/campaign-certification`.**
Start from this branch, not `master` or an older Claude branch. S01–S21 are complete.
Codex closed S19 and S20 with actual campaign evidence. [G4 is earned](campaign-certification/G4/README.md); [S22 performance qualification](campaign-certification/S22/README.md) is in progress.
CP1 certification and worldwide character coverage remain open.

## Latest checkpoint — 28 September

| Area | Verified state | Next owner / action |
|---|---|---|
| S19 tutorial/advisors | **Complete.** Actual budget, paid construction/output, company purchases/delivery and a supported flown mission are recognized; save/load/Continue and fresh-campaign isolation pass. | [Codex closeout](campaign-certification/S19/integration/CLOSEOUT.md), runtime `c8a59bfd`. |
| S20 shared interface | **Complete.** Native keyboard/touch journey, retries, focus, campaign isolation and full save integrity passed. | [S20 closeout](campaign-certification/S20/README.md); 1,717 UI and 420 native tests passed (21 existing native tests ignored). |
| S22 art and performance | **In progress.** All three actual inputs are ready. Full 9823b070 release regression: 1,922 passed, 0 failed, 96 ignored. Both isolated dated preflights pass memory but fail latency; later sovereignty repair passes bounded actual-2035 parity. | Codex: [current evidence](campaign-certification/S22/README.md), [import checks/failures](campaign-certification/S22/idle-import-progress/README.md), [diplomacy diagnosis](campaign-certification/S22/diplomacy-diagnosis-progress/README.md). Next: finish validation of integrated contract/campaign route repairs, then measure the combined candidate; no threshold changes. |
| C01 gap audit | **CLAUDE-C01-GAPS-01 complete**, accepted with provenance and portable-hash repairs. | [Review](campaign-certification/C01/integrations/CLAUDE-C01-GAPS-01/INTEGRATION.md). Historical C01 coverage remains open. |
| C01 source repairs | **SOURCE-05 and SOURCE-06 complete**, with independently reproduced primary bodies/content; **CODEX-C01-SOURCE-06-REVIEW complete**. | [Russian archive review](campaign-certification/C01/integrations/CLAUDE-C01-SOURCE-05/README.md), [Bush Library review](campaign-certification/C01/integrations/CLAUDE-C01-SOURCE-06/README.md). Other claims are not automatically accepted. |
| Pending Claude work | SOURCE-17/26 and C01-23/24/25/27 await review. All five remaining expanded sections are now claimed on their own branches; remote handoffs inspected after S19 closure; C01-27 refreshed to `0765c590` after S20 closure. | Query the bounded task queue; new remote heads require independent review before acceptance. |

S19 closure does not award CP1. The flight proof records an actual launch and store
consumption with no opposing target contact, so it does not claim combat damage.

S22's isolated 9823b070 preflights remain failed: 2015 simulation/whole-turn p95
**356.2455 / 446.365 ms**, maximum **679.2651 ms**; 2035 p95 **452.9066 /
554.1808 ms**, maximum **13,964.7566 ms**. Both memory limits pass. The newer
8de6f2a0 contract/campaign route implementation is integrated with build/tests
ongoing at this snapshot. No authoritative qualification pair has started;
S22, G5 and CP1 remain unearned. Source review and bounded parity checks cannot
replace the two complete qualifying rounds.

The [campaign pathway](CERTIFIED_CAMPAIGN_PATHWAY.md) defines the approved game scope.
[campaign-pathway.json](planning/campaign-pathway.json) owns session status, dependencies,
acceptance criteria and evidence. [ai-workstreams.json](planning/ai-workstreams.json)
assigns work; it deliberately does not duplicate completion status. This allocation is a
handoff plan, not a claim that Claude has started work or permission to run every session.

## Who owns what

| Workstream | Owner | Session markers | Boundary and next step |
|---|---|---|---|
| Flight and player journey | Codex | S18, S20, S21 | S18–S21 complete. G4 earned; S22 performance qualification is in progress. |
| Tutorial and advisors | Claude | S19 | Submission `7de62539` integrated at `2400800b`; Codex completed ordinary later-outcome qualification; S19 is closed. |
| Historical characters and cartoons | Claude | C01–C07, S23 | C01-01/02/03/04/07/08 accepted as bounded research; C01-05/06/09–22/26 integrated after technical verification, historical acceptance pending. C01-23/24/25/27 are submitted for review. C01 remains incomplete. |
| Integration, performance and qualification | Codex | S22, S24, S25, S27–S30 | Codex is executing the frozen S22 protocol; no timing threshold is relaxed. Claude owns a bounded S24 successor-fixture preparation packet; Codex owns final qualification. Neither a content batch nor a unit-test pass earns CP1. |
| Independent human playtests | User / human testers | S26 | Codex prepares reproducible builds and tasks; Claude may review observations. AI testing cannot substitute for human sessions. |
| Later aircraft/naval expansion | Codex | E01–E04, E06 | Parked until CP1; preserve the current three supported mission families. |
| Later company identity/history | Claude | E05 | Eight-company France/Japan research pilot claimed. Runtime expansion stays after CP1; current supplier economy stays with Codex. |
| Completed foundation | Codex / retained evidence | S00–S17 | Reference only. Reopen only for a specific reproduced defect; retain the original qualification records. |

## Six additional Claude sections — claims synchronized

The user authorized this expanded assignment on 27 September. These are six new
bounded deliveries, **not six completed roadmap sessions**. They can proceed
independently of remaining C01 source repairs. Read the
[expanded handoff](planning/ai-handoffs/CLAUDE-EXPANDED-NEXT.md) for branch, file ownership,
review and submission rules. Suggested order for one worker:

| Task | Build / research deliverable | Release boundary |
|---|---|---|
| [CLAUDE-C01-GAPS-01](planning/ai-handoffs/CLAUDE-C01-GAPS-01.md) | Eight-country party/institution gap ledger and numbered next research batches. | No invented coverage or duplicate pending research. |
| [CLAUDE-C03-REVIEW-01](planning/ai-handoffs/CLAUDE-C03-REVIEW-01.md) | Cartoon review workbench, small-card previews, asset checks and six-to-eight portrait review. | Tooling and existing-art audit; no production portrait replacements. |
| [CLAUDE-C04-PREP-01](planning/ai-handoffs/CLAUDE-C04-PREP-01.md) | Eight fictional France/Tonga successor proposals with institutional research and validation. | Explicit fiction, eligibility through 2035; no automatic appointments or runtime installation. |
| [CLAUDE-S23-MATRIX-01](planning/ai-handoffs/CLAUDE-S23-MATRIX-01.md) | Executable historical handover/date/role/art coverage matrix. | Preparation; S20 complete, C06 and final content qualification still required. |
| [CLAUDE-S24-SUCCESSORS-01](planning/ai-handoffs/CLAUDE-S24-SUCCESSORS-01.md) | 23-successor inventory and isolated activation/load/UI harness. | Authored fixtures labeled; Codex retains full S24 qualification. |
| [CLAUDE-E05-RESEARCH-01](planning/ai-handoffs/CLAUDE-E05-RESEARCH-01.md) | Eight sourced French/Japanese company dossiers and catalog mappings. | Research now; company mechanics remain after S30/CP1. |

Each task has its own allowed files, deliverables and acceptance checks. Claude has
completed C01-GAPS-01, accepted with repairs in `68a38eb4`; the other five new
sections are claimed: C03 `28b3c577`, C04 `f1547abd`, S23 `b4c021ab`, S24 `328db128`, and E05 `1966fb06`. Remote handoffs were read; these claim commits contain no accepted implementation. This bounded audit does not complete historical coverage. A parent session may still have unmet dependencies: only
its named independent preparation is authorized here. The canonical roadmap and
44 unique session assignments remain unchanged. Codex keeps active supplier runtime,
S19 later-outcome evidence, S20 shared interface, historical acceptance and integration.

## Historical research: six bounded Tonga packets integrated

Claude submitted the [first Tonga packet](https://github.com/ridgemerkley2-web/Spheres/tree/claude/c01-tonga-01)
at `426f0ddb17a93c3d8b9a809c0129b441b29d3fe6`, based on `3d422da`.
Its [handoff](planning/ai-handoffs/CLAUDE-C01-01.md) is **accepted and integrated**,
qualified at `fdb6d2c` with [source review and validation evidence](campaign-certification/C01/integrations/CLAUDE-C01-01/README.md).
The [second packet](planning/ai-handoffs/CLAUDE-C01-02.md), submitted at `b6767837`,
is also accepted, with [independent source and atlas review](campaign-certification/C01/integrations/CLAUDE-C01-02/README.md).
The [C01-04 party and C01-07 Crown packets](campaign-certification/C01/integrations/CLAUDE-C01-04-07/README.md)
are also accepted and integrated, at `48d8bfef` and `8d8e4b41` respectively.
The [C01-08 prime-minister packet](campaign-certification/C01/integrations/CLAUDE-C01-08/README.md)
was reviewed at `14f01c5a` and merged at `bd24d577`. Its independent audit reproduced
34 original source response identities; four source contents remain explicitly
unverified. The merged 58 Tonga tests and exact research-index check pass.

The [C01-03 transition packet](campaign-certification/C01/integrations/CLAUDE-C01-03/README.md)
was reviewed at `3291facf` (including `387e4526`) and merged at `5a63d7b6`.
Its seven critical original responses reproduced exactly; 85 Python tests passed
in isolation, with unresolved instruments and office dates retained.

These six accepted packets are bounded research intake. They do not complete
C01 or install leaders and avatars. Do not repeat them. On 27 September, the user
authorized merging all 17 technically verified submissions: C01-05/06/09–22/26.
They are now integrated as research; historical acceptance remains pending.
The [integration record](campaign-certification/verification/2026-09-27-claude-integration.md)
pins the reviewed heads, test evidence and remaining source-review limitations.
Do not reimplement these submissions or confuse integration with acceptance.
The two oldest submissions are:

| Packet | Branch / reviewed inventory tip | Pending scope |
|---|---|---|
| [CLAUDE-C01-05](https://github.com/ridgemerkley2-web/Spheres/blob/claude/c01-ussr-05/docs/planning/ai-handoffs/CLAUDE-C01-05.md) | `claude/c01-ussr-05` / `1c698ed0` | The 1991 USSR/RSFSR executive transition. |
| [CLAUDE-C01-06](https://github.com/ridgemerkley2-web/Spheres/blob/claude/c01-saudi-06/docs/planning/ai-handoffs/CLAUDE-C01-06.md) | `claude/c01-saudi-06` / `7948ab98` | Saudi kings and crown princes, 1990–2026. The older `claude/c01-saudi-03` is this same substantive packet before renumbering, not a separate submission. |

These integrated tips are not already accepted equivalents. Their source claims
have not been independently accepted by this workboard update; a submitted packet
must not be reclaimed as unstarted work. Read its remote handoff and coordinate
bounded repairs with the integrator. The [C01-08 review inventory](campaign-certification/C01/integrations/CLAUDE-C01-08/README.md#other-pending-submissions-inventory-only)
records the earlier comparison against integration; C01-03 was subsequently
accepted as recorded above.

Claim one bounded C01 batch rather than attempting
the entire world at once. C02 batches contain at most ten leadership chains/people;
C03 art batches contain six to eight physically reviewed cartoons. Real people extend
through the frozen, researched present-day cutoff; later candidates are explicitly
fictional through 2035. The existing cutoff is 7 September 2026 until a sourced
change is reviewed. Do not silently advance it to the current date.

Claude’s gameplay packet [CLAUDE-S19-01](planning/ai-handoffs/CLAUDE-S19-01.md) was
submitted at `7de62539` and is integrated at `2400800b` after an independent complete
first-hour browser rerun, 1,672 UI tests and 414 native web tests. The
[integration review](campaign-certification/S19/integration/README.md) retains the earlier
checkpoints. The [S19 closeout](campaign-certification/S19/integration/CLOSEOUT.md)
now completes actual later construction/output, paid purchases/delivery, supported
flight and guidance save/resume qualification. S19 and S20 are complete; G4 is earned.
Use the integrated routes instead of restarting or independently rewriting them.

## Query each AI’s own work

From the repository root, these commands only read the checked-in workboard:

```text
python tools/planning/workboard.py --owner Claude
python tools/planning/workboard.py --owner Codex
python tools/planning/workboard.py --session S19
python tools/planning/workboard.py --session C01
python tools/planning/workboard.py --check
python tools/planning/workboard.py --tasks --owner Claude
python tools/planning/workboard.py --tasks --owner Codex
python tools/planning/workboard.py --task CLAUDE-C01-SOURCE-05
```

The [bounded task queue](planning/ai-task-queue.json) records priority, owner,
packet state and dependencies separately from canonical session status.
[Claude's expanded task list](planning/ai-handoffs/CLAUDE-EXPANDED-NEXT.md) adds six
independent sections. The [existing research list](planning/ai-handoffs/CLAUDE-C01-NEXT.md)
tracks the remaining source/content submissions. SOURCE-05/06 and C01-GAPS-01
are closed as bounded tasks; their parent C01 remains incomplete. Codex completed
[S19 later outcomes](planning/ai-handoffs/CODEX-S19-LATER-01.md) and has completed
S20's combined map, province and room navigation. S22 performance qualification is in progress.

Copy this into Claude to resume its claimed sections:

> Fetch origin/codex/campaign-certification. Read docs/AI_WORKSTREAMS.md and
> docs/planning/ai-handoffs/CLAUDE-EXPANDED-NEXT.md, then run
> `python tools/planning/workboard.py --tasks --owner Claude`. Start with
> one of your five existing claimed expanded sections. C01-GAPS-01 is complete; do not repeat it.
> Read that task's handoff, record your branch/base and claim, and build its bounded
> deliverables with the required checks. The six new sections do not wait on unrelated
> source repairs. Keep existing claims, skip completed SOURCE-05/06, and do not duplicate pending submissions.
> Return exact ready-for-review commits and evidence. Do not edit shared runtime,
> install unaccepted content or change canonical roadmap status.

To ask either assistant for a status check:

> Read the latest GitHub workboard. Report only your assigned sections: marker,
> current status, unmet dependencies, evidence, remaining work and next bounded task.
> Verify the files and commits; do not infer completion from a previous chat.

## Working together without losing changes

1. Fetch the integration branch before starting. Use a separate checkout/branch per
   packet (`claude/c01-tonga-01`, for example). Never work directly in another AI’s
   uncommitted checkout. The local game running on a port is not evidence of GitHub state.
2. Claim one marker in that packet’s handoff record: owner, branch, base commit,
   current state, touched paths and next checkpoint. Other packets keep their own
   records; the central status JSON is updated by the integrator after review.
3. Respect the packet’s file boundary. `spheres-web/src/main.rs`,
   `spheres-web/ui/index.html`, shared save schemas, native command dispatch,
   `equipment-*`, release workflows and the central roadmap require an integration
   handoff if another owner is working there. Describe the required change or supply
   a focused patch instead of independently rewriting those files.
4. Return exact source commits, meaningful test commands/results, browser/art
   captures when relevant, new dependencies/licenses, save implications and known
   gaps. `ready_for_review` is not `complete`. Uncommitted screenshots or a claim
   of passing tests without a source revision are not release evidence.
5. Codex compares against current integration, resolves overlap, runs affected
   checks and records the merge/evidence. Preserve original saves, older evidence
   and prior work. Never force-push the integration branch.
6. Full-campaign, historical coverage, performance, human-playtest and packaging
   gates retain their own acceptance criteria. Each owner may report its own
   evidence; nobody self-awards the full campaign certificate from a partial task.

When scope or ownership changes, update this workboard and the relevant packet,
then run `workboard.py --check`. All markers continue to refer to the same original
campaign pathway; this is a work split, not a replacement roadmap.

Codex completed the independently ready [S21 campaign journey](campaign-certification/S21/README.md) while S19 remained claimed. It adds campaign goals/history/continuation and repairs first-day successor saves. S19 and S20 are now complete, and the dated G4 decision records the earned player-journey gate.

Codex completed [CODEX-S22-PREP-01](planning/ai-handoffs/CODEX-S22-PREP-01.md): repaired art accounting, restored compiled equipment self-shadows, and validated the isolated renderer. Its original 42-overrun finding remains in the historical preparation record. The [current art audit](art/P0_BUDGETS.md) now grades 245 configurations with 166 passes, 79 advisory density notes and no ceiling or required-quality-floor failures. The [adaptive town renderer](art/TOWN_SCENE_RENDERING.md) preserves the original close meshes and measures actual submissions within the unchanged scene ceiling; raw full-block overages remain explicit diagnostics. This is retained independent preparation. Codex has now claimed S22 and frozen its measurement protocol; current offline accounting passes while full campaign performance and human qualification remain open. See the [current completion follow-up](campaign-certification/verification/2026-09-22-completion.md) for exact local and hosted validation status.

The [six-session development journal](campaign-certification/development/2026-09-21-six-sessions/README.md)
records the subsequent equipment picker, province activity overview, local timing
export, Congo selection repair, 137-country preflight and reviewed Claude integration.
The latest separate preview is <http://127.0.0.1:7866/>. These six development
checkpoints do not automatically close six canonical markers.
