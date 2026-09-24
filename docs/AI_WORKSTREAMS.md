# Spheres — Codex and Claude workboard

Updated 22 September 2026. **Integration branch: `codex/campaign-certification`.**
Start from this branch, not `master` or an older Claude branch. S01–S18 and S21 are complete;
Claude's S19 implementation is integrated; later-campaign browser qualification remains.
CP1 certification and worldwide character coverage remain open.

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
| Historical characters and cartoons | Claude | C01–C07, S23 | C01-01/02/03/04/07/08 accepted as bounded research; C01-05/06 submitted and awaiting independent review. C01 remains incomplete. |
| Integration, performance and qualification | Codex | S22, S24, S25, S27–S30 | Integrate reviewed work and run exact-build checks. Neither a content batch nor a unit-test pass earns CP1. |
| Independent human playtests | User / human testers | S26 | Codex prepares reproducible builds and tasks; Claude may review observations. AI testing cannot substitute for human sessions. |
| Later aircraft/naval expansion | Codex | E01–E04, E06 | Parked until CP1; preserve the current three supported mission families. |
| Later company identity/history | Claude | E05 | Parked until CP1. No changes to the current supplier economy during research. |
| Completed foundation | Codex / retained evidence | S00–S17 | Reference only. Reopen only for a specific reproduced defect; retain the original qualification records. |

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
C01 or install leaders and avatars. Do not repeat them. Two genuine submissions
remain pending independent source review and integration:

| Packet | Branch / reviewed inventory tip | Pending scope |
|---|---|---|
| [CLAUDE-C01-05](https://github.com/ridgemerkley2-web/Spheres/blob/claude/c01-ussr-05/docs/planning/ai-handoffs/CLAUDE-C01-05.md) | `claude/c01-ussr-05` / `1c698ed0` | The 1991 USSR/RSFSR executive transition. |
| [CLAUDE-C01-06](https://github.com/ridgemerkley2-web/Spheres/blob/claude/c01-saudi-06/docs/planning/ai-handoffs/CLAUDE-C01-06.md) | `claude/c01-saudi-06` / `7948ab98` | Saudi kings and crown princes, 1990–2026. The older `claude/c01-saudi-03` is this same substantive packet before renumbering, not a separate submission. |

The two pending tips are not already accepted equivalents. Their source claims
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
remaining qualification: actual later purchase/delivery, completed output and flown
results through the guidance panel. Codex owns that integration check; Claude may
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
```

Copy this into Claude to continue bounded research:

> Fetch origin/codex/campaign-certification and read docs/AI_WORKSTREAMS.md and
> the accepted and pending C01 intake listed there. Do not repeat accepted
> C01-01/02/03/04/07/08 or reclaim submitted C01-05/06. Review any requested fixes
> on your pending packet first. For new work, propose a distinct bounded packet,
> record its claim and current integration base in a separate branch, and follow
> its source, file and evidence boundaries. Return a ready-for-review handoff
> without changing the canonical roadmap status.

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
