# Spheres — Codex and Claude workboard

Updated 27 September 2026. **Integration branch: `codex/campaign-certification`.**
Start from this branch, not `master` or an older Claude branch. S01–S18 and S21 are complete;
Claude's S19 implementation is integrated; later-campaign browser qualification remains.
CP1 certification and worldwide character coverage remain open.

## Latest checkpoint — 27 September

| Area | Verified state | Next owner / action |
|---|---|---|
| S19 construction | Payment, completion and output survive save/load and Continue (`24d05428`). | Codex: retain this evidence while qualifying later outcomes. |
| S19 manufacturer prerequisites | Missing-plant blocker repaired (`8b93372e`); clean build `2cb1da4a` passes 270 focused checks and the full first-hour browser regression. | Codex: retain this prerequisite regression alongside the completed procurement proof. |
| S19 procurement | Real component plant and local grid funded; one company-built tank purchased/paid 1 February 1997 and delivered 8 February. Native guidance, named save/load, Continue and narrow UI pass on unchanged runtime `c8a59bfd`. | Codex: obtain aircraft, prove squadron readiness and a supported flown result. [Evidence](campaign-certification/S19/integration/PROCUREMENT_DELIVERY.md). |
| S19 supplier input recovery | Exact input quantities, industry/plant shortcuts and advanced-component trade verified at `c8a59bfd`; ordinary plant/grid recovery now reaches actual delivered equipment. | Retain the failed power-constrained run and the successful recovery separately. No production bypass or S19 closure. |
| S20 shared interface | Waiting on S19 closure. | Codex: start only after the remaining S19 evidence passes. |
| C01 submissions | SOURCE-05 `66331d9d`, SOURCE-06 `93467faa`, SOURCE-26 `c1f537f2`, C01-23 `9c0f5c14`, C01-25 `c06839c1`, C01-24 `a7a9e39c` and C01-GAPS-01 `b4d95396` declare **ready for review**. New heads were fetched and their handoffs read; no acceptance or merge is claimed. | Codex reviews; Claude retains SOURCE-17 and C01-27 claims and can pick an independent new section below. |

[Contractor repair and exact validation evidence](campaign-certification/S19/integration/CONTRACTOR_PREREQUISITE.md).
[Supplier-input checkpoint, recorded campaign and next actions](campaign-certification/S19/integration/PROCUREMENT_SUPPLY.md).
The 270 checks are focused checks on this repair, not a new full-workspace test
claim. Observed construction or company creation alone does not qualify purchase,
delivery, readiness, flight or CP1. Canonical roadmap status is unchanged.

The [campaign pathway](CERTIFIED_CAMPAIGN_PATHWAY.md) defines the approved game scope.
[campaign-pathway.json](planning/campaign-pathway.json) owns session status, dependencies,
acceptance criteria and evidence. [ai-workstreams.json](planning/ai-workstreams.json)
assigns work; it deliberately does not duplicate completion status. This allocation is a
handoff plan, not a claim that Claude has started work or permission to run every session.

## Who owns what

| Workstream | Owner | Session markers | Boundary and next step |
|---|---|---|---|
| Flight and player journey | Codex | S18, S20, S21 | S18 and S21 complete. S20 follows final S19 qualification; independent narrow controls are integrated. |
| Tutorial and advisors | Claude | S19 | Submission `7de62539` integrated at `2400800b`; Codex qualifies remaining later outcomes before closure. |
| Historical characters and cartoons | Claude | C01–C07, S23 | C01-01/02/03/04/07/08 accepted as bounded research; C01-05/06/09–22/26 integrated after technical verification, historical acceptance pending. C01-23/24/25 are submitted; C01-27 remains claimed. C01 remains incomplete. |
| Integration, performance and qualification | Codex | S22, S24, S25, S27–S30 | Claude owns a bounded S24 successor-fixture preparation packet; Codex owns final qualification. Integrate reviewed work and run exact-build checks. Neither a content batch nor a unit-test pass earns CP1. |
| Independent human playtests | User / human testers | S26 | Codex prepares reproducible builds and tasks; Claude may review observations. AI testing cannot substitute for human sessions. |
| Later aircraft/naval expansion | Codex | E01–E04, E06 | Parked until CP1; preserve the current three supported mission families. |
| Later company identity/history | Claude | E05 | Eight-company France/Japan research pilot available now. Runtime expansion stays after CP1; current supplier economy stays with Codex. |
| Completed foundation | Codex / retained evidence | S00–S17 | Reference only. Reopen only for a specific reproduced defect; retain the original qualification records. |

## Six additional Claude sections — available now

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
| [CLAUDE-S23-MATRIX-01](planning/ai-handoffs/CLAUDE-S23-MATRIX-01.md) | Executable historical handover/date/role/art coverage matrix. | Preparation; C06/S20 and final content qualification still required. |
| [CLAUDE-S24-SUCCESSORS-01](planning/ai-handoffs/CLAUDE-S24-SUCCESSORS-01.md) | 23-successor inventory and isolated activation/load/UI harness. | Authored fixtures labeled; Codex retains full S24 qualification. |
| [CLAUDE-E05-RESEARCH-01](planning/ai-handoffs/CLAUDE-E05-RESEARCH-01.md) | Eight sourced French/Japanese company dossiers and catalog mappings. | Research now; company mechanics remain after S30/CP1. |

Each task has its own allowed files, deliverables and acceptance checks. Claude has
submitted C01-GAPS-01 on `claude/c01-gaps-01` at `b4d95396`; the other five new
sections remain queued. This is submission inventory, not independent verification or acceptance. A parent session may still have unmet dependencies: only
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
[integration review](campaign-certification/S19/integration/README.md) retains its
remaining qualification: actual later purchase/delivery and flown results through
the guidance panel. Construction payment/completion/output now pass with save/resume
at `24d05428`; [evidence](campaign-certification/S19/integration/PAYMENT_RETENTION.md).
Codex owns the remaining integration check; Claude may
provide bounded repairs. S19 remains in progress and S20 waits for its closure.
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
retains SOURCE-17 and C01-27 claims; SOURCE-05/06/26, C01-23/24/25 and C01-GAPS-01 await review. Codex's next
campaign task is [S19 later outcomes](planning/ai-handoffs/CODEX-S19-LATER-01.md):
actual squadron readiness and flown results with save/resume qualification.
Construction and procurement/payment/delivery are already verified with save/resume.
The immediate gameplay task is supplying advanced components to the certified
manufacturer through ordinary production or arrived purchases. The recovery UI is
now verified; current quotes offer no goods, and the local plant is affordable but
requires 660 funded days before operation. See [input recovery](campaign-certification/S19/integration/INPUT_RECOVERY.md).

Copy this into Claude to start one of the new sections:

> Fetch origin/codex/campaign-certification. Read docs/AI_WORKSTREAMS.md and
> docs/planning/ai-handoffs/CLAUDE-EXPANDED-NEXT.md, then run
> `python tools/planning/workboard.py --tasks --owner Claude`. Start with
> CLAUDE-C01-GAPS-01, or pick the next unclaimed new section if it is already claimed.
> Read that task's handoff, record your branch/base and claim, and build its bounded
> deliverables with the required checks. The six new sections do not wait on unrelated
> source repairs. Keep existing claims and skip submitted SOURCE-06/26 and C01-23/25.
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

Codex completed the independently ready [S21 campaign journey](campaign-certification/S21/README.md) while S19 remained claimed. It adds campaign goals/history/continuation and repairs first-day successor saves. Fetch this integration base before bringing S19 shared-shell changes forward. G4 still requires S19 and S20.

Codex completed [CODEX-S22-PREP-01](planning/ai-handoffs/CODEX-S22-PREP-01.md): repaired art accounting, restored compiled equipment self-shadows, and validated the isolated renderer. Its original 42-overrun finding remains in the historical preparation record. The [current art audit](art/P0_BUDGETS.md) now grades 245 configurations with 166 passes, 79 advisory density notes and no ceiling or required-quality-floor failures. The [adaptive town renderer](art/TOWN_SCENE_RENDERING.md) preserves the original close meshes and measures actual submissions within the unchanged scene ceiling; raw full-block overages remain explicit diagnostics. This is independent preparation: S22 remains planned after S20, and full campaign performance and human qualification remain open. See the [current completion follow-up](campaign-certification/verification/2026-09-22-completion.md) for exact local and hosted validation status.

The [six-session development journal](campaign-certification/development/2026-09-21-six-sessions/README.md)
records the subsequent equipment picker, province activity overview, local timing
export, Congo selection repair, 137-country preflight and reviewed Claude integration.
The latest separate preview is <http://127.0.0.1:7866/>. These six development
checkpoints do not automatically close six canonical markers.
