# Development roadmap

**Updated 30 September 2026 · Development branch: `codex/campaign-certification`.**

The first target is **CP1: a verified campaign from 1 January 1990 through
31 December 2035**, covering France, Japan, India, Brazil, South Africa, Tonga,
Saudi Arabia and a controlled USSR → Russia case.

**S01–S22 are complete. S23–S30 remain open.** The integrated game and supported
player journey are accepted through G4. G5 and CP1 are not yet earned.
These are milestone counts, not a percentage of remaining effort.

## Next actions

| Priority | Owner | Action | Completion requirement |
|---|---|---|---|
| 1 — release content dependency | Claude research/art; Codex review | Finish and integrate the eight country casts: accepted historical identities → dated role bindings → reviewed cartoons → fictional successors → country signoff. Continue existing claims before starting another batch. | C06 closes, then S23 passes its historical/date/art audit. Research receipts alone do not close a country. |
| 2 — active engineering | Codex | Collect and independently verify the fresh 24-cell long-campaign run; repair actual failures and repeat affected checks. | All eight countries × three seeds reach the full endpoint with valid save/resume and conservation evidence. Latest recorded attempt is pending. |
| 3 — blocked engineering | Codex; design review as needed | Resolve A1 political-outcome concentration from new causal evidence or a reviewed model change. | The original political-calibration gate passes. Existing diagnostics found no justified repair; its threshold remains unchanged. |
| 4 — player feedback | Human participants; Codex preparation | Prepare the newcomer protocol, then conduct independent opening and later-game sessions once S24 is qualified. | At least five first-time participants and eight sessions, meeting S26's recorded task-success criteria. |
| 5 — release | Codex | Fix remaining significant defects, freeze the candidate, qualify its packages, audit the evidence and publish. | S27–S30 close on the exact tested build. |

The [workboard](docs/AI_WORKSTREAMS.md) names existing packets and research
holds. Consult it before assigning new work. The [canonical session register](docs/planning/campaign-pathway.json)
owns status and dependencies; the [bounded task queue](docs/planning/ai-task-queue.json)
owns packet status. This page is the readable summary of those records.

## Completed foundation

| Milestones | Delivered |
|---|---|
| S01–S05 | Integrated economy, companies, warfare, compatible saves and command recovery |
| S06–S10 | Finance, construction/jobs, suppliers/imports, research/design, government and diplomacy |
| S11–S16 | Ground equipment, squadrons, airbases, funded support, tactical missions and fighters |
| S17–S21 | AI participation, campaign aircraft art, tutorial/advisor, navigation, goals and history |
| S22 | Art accounting and measured performance qualification on the recorded candidate/hardware |

Full acceptance criteria and source-specific evidence remain in the
[certified campaign pathway](docs/CERTIFIED_CAMPAIGN_PATHWAY.md).

## What still needs proof

- **Characters:** C01 and the broader character program remain unfinished. Eight
  completed country casts are required for S23; worldwide completion is a later,
  separately tracked milestone.
- **Campaign endurance:** the earlier full matrix failed from timeouts and a lost
  runner. A fresh attempt on `68ba0622` began 30 September. No full-matrix pass is
  recorded. [Run record](docs/campaign-certification/S25/preparation/full-matrix-20260930/README.md).
- **Political balance:** A1 still fails. [Diagnostic record](docs/campaign-certification/S27/preparation/a1-geography-20260928/README.md).
- **Human usability:** automated browser tests do not satisfy S26.
- **Final qualification:** worldwide startup, recovery, succession and packaging
  have completed preparation. Their canonical milestones still retain the
  content and preceding qualification dependencies.

On the last checked integration commit `ff01ee61`, Windows and Linux native,
JavaScript, town-browser and packaging jobs passed; political calibration and
the aggregate checks failed. [CI evidence](https://github.com/ridgemerkley2-web/Spheres/actions/runs/36707707542).

## After the first certified release

The wider plan retains reconnaissance/support aircraft, bombers/electronic warfare,
helicopters/drones, naval/carrier operations, deeper national companies and
worldwide character coverage. These remain in C07 and E01–E06 in the canonical
pathway. They do not displace the current CP1 priorities.

Earlier plans and long development journals live in the [archive](docs/archive/README.md).
Use this page for current direction, and system references for implementation details.
