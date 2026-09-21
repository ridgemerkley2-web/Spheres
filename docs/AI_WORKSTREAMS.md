# Spheres — Codex and Claude workboard

Updated 21 September 2026. **Integration branch: `codex/campaign-certification`.**
Start from this branch, not `master` or an older Claude branch. S01–S17 are complete;
Codex is implementing S18. CP1 certification and worldwide character coverage remain open.

The [campaign pathway](CERTIFIED_CAMPAIGN_PATHWAY.md) defines the approved game scope.
[campaign-pathway.json](planning/campaign-pathway.json) owns session status, dependencies,
acceptance criteria and evidence. [ai-workstreams.json](planning/ai-workstreams.json)
assigns work; it deliberately does not duplicate completion status. This allocation is a
handoff plan, not a claim that Claude has started work or permission to run every session.

## Who owns what

| Workstream | Owner | Session markers | Boundary and next step |
|---|---|---|---|
| Flight and player journey | Codex | S18, S20, S21 | Finish S18’s live flight pages and aircraft. Then navigation and campaign goals. |
| Tutorial and advisors | Claude | S19 | Design/review now; implementation starts after S18 is merged. Recognize real outcomes, keep advice optional. |
| Historical characters and cartoons | Claude | C01–C07, S23 | Start with one bounded C01 research packet. Continuous terms, cartoons, fictional future candidates and country sign-offs follow in order. |
| Integration, performance and qualification | Codex | S22, S24, S25, S27–S30 | Integrate reviewed work and run exact-build checks. Neither a content batch nor a unit-test pass earns CP1. |
| Independent human playtests | User / human testers | S26 | Codex prepares reproducible builds and tasks; Claude may review observations. AI testing cannot substitute for human sessions. |
| Later aircraft/naval expansion | Codex | E01–E04, E06 | Parked until CP1; preserve the current three supported mission families. |
| Later company identity/history | Claude | E05 | Parked until CP1. No changes to the current supplier economy during research. |
| Completed foundation | Codex / retained evidence | S00–S17 | Reference only. Reopen only for a specific reproduced defect; retain the original qualification records. |

## Claude can start here: `CLAUDE-C01-01`

Read [the C01 handoff](planning/ai-handoffs/CLAUDE-C01-01.md). Reconcile one small Tonga
party/institution research packet, record exact sources and unresolved questions, and
return a reviewable commit. This can run alongside Codex’s S18 because it changes
research records and their validation only. It must not turn uncertain observations
into installed leaders, continuous terms or finished avatars.

After that packet, request/review the next bounded C01 batch rather than attempting
the entire world at once. C02 batches contain at most ten leadership chains/people;
C03 art batches contain six to eight physically reviewed cartoons. Real people extend
through the frozen, researched present-day cutoff; later candidates are explicitly
fictional through 2035. The existing cutoff is 7 September 2026 until a sourced
change is reviewed. Do not silently advance it to the current date.

Claude’s next gameplay packet is [CLAUDE-S19-01](planning/ai-handoffs/CLAUDE-S19-01.md).
It waits for S18. S20 integration must wait for S19’s shared-navigation handoff so
both assistants do not independently edit the campaign shell.

## Query each AI’s own work

From the repository root, these commands only read the checked-in workboard:

```text
python tools/planning/workboard.py --owner Claude
python tools/planning/workboard.py --owner Codex
python tools/planning/workboard.py --session S19
python tools/planning/workboard.py --session C01
python tools/planning/workboard.py --check
```

Copy this into Claude:

> Fetch GitHub and start from origin/codex/campaign-certification. Read
> docs/AI_WORKSTREAMS.md and run python tools/planning/workboard.py --owner Claude.
> Review your current markers, then work only on CLAUDE-C01-01. Follow its file
> boundaries and checks. Put the result on a separate branch and report the commit,
> changed files, sources, tests and unresolved gaps. Do not mark all of C01 complete.

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
