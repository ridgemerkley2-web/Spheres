# Claude — six additional independent sections

Assigned 27 September 2026 by the user's request, “Give Claude more sections to work on.”
Base: `5979cf2fba78a6784a549d63223ca274950a2f7d`; fetch the latest
`origin/codex/campaign-certification` before claiming a packet.

Updated 28 September 2026 UTC after S19 closure and a fresh Git fetch.
C01-GAPS-01 is **complete**, accepted with repairs in `68a38eb4`. The remaining five
sections are **claimed**, not available for duplicate work: C03 `28b3c577`, C04
`f1547abd`, S23 `b4c021ab`, S24 `328db128`, and E05 `1966fb06`. Their remote handoffs
were inspected; these claim commits do not contain accepted deliverables.

| Order | Task / handoff | Concrete deliverable |
|---|---|---|
| 1 | [CLAUDE-C01-GAPS-01](CLAUDE-C01-GAPS-01.md) | Reproducible eight-country party/institution gap ledger and numbered next research batches. |
| 2 | [CLAUDE-C03-REVIEW-01](CLAUDE-C03-REVIEW-01.md) | Working cartoon contact-sheet reviewer with small-card previews, identity/era checks and asset validation. |
| 3 | [CLAUDE-C04-PREP-01](CLAUDE-C04-PREP-01.md) | Eight explicitly fictional successor proposals for France/Tonga, with sourced institutional constraints and validation. |
| 4 | [CLAUDE-S23-MATRIX-01](CLAUDE-S23-MATRIX-01.md) | Executable historical/date/art coverage audit across the certified country set. |
| 5 | [CLAUDE-S24-SUCCESSORS-01](CLAUDE-S24-SUCCESSORS-01.md) | Successor identity inventory and isolated activation/load/UI test harness. |
| 6 | [CLAUDE-E05-RESEARCH-01](CLAUDE-E05-RESEARCH-01.md) | Eight sourced French/Japanese company dossiers and proposed catalog mappings. |

## Working contract for all six

- Query `python tools/planning/workboard.py --tasks --owner Claude` and read the
  selected handoff. Claim it in that handoff with branch, exact base, touched paths
  and next checkpoint. Use a separate `claude/...` branch from current integration;
  do not stack on unreviewed research. Prefer this order for one worker; each packet
  can proceed independently. Do not wait for unrelated source acceptance to build tools.
- Own only the paths named in that packet and its handoff. Existing production data,
  shared avatar/leadership manifests, research index, campaign shell, commands,
  save schemas, supplier runtime and CI workflows are read-only. Propose a focused
  integration patch if a shared change is necessary. Do not weaken an existing check.
- New tools must run against current integration without requiring another new
  packet. Record input revisions/hashes. Pending research stays visibly pending;
  missing data is a reported gap, never invented history or a passing coverage cell.
- Return implementation/data, meaningful tests, exact commands/results, screenshots
  for visual work, provenance/license notes and limitations in the handoff. Mark
  `ready_for_review`; Codex merges and updates the central queue after review.
- These are bounded preparation/research/tool deliveries. They do not complete
  C03/C04/S23/S24/E05 or bypass their canonical dependencies. Production installation,
  exact-build qualification and post-CP1 company mechanics remain gated. Do not edit
  `campaign-pathway.json` or award a campaign certificate.

## Current submissions: skip duplicate work

SOURCE-05 and SOURCE-06 are **complete** after independent source/content review;
see their integration evidence. SOURCE-17 `efeac546`, SOURCE-26 `c1f537f2`, C01-23
`9c0f5c14`, C01-24 `a7a9e39c`, C01-25 `c06839c1`, and C01-27 `31035e25` declare
`ready_for_review`. These pending submissions are not independent verification or
merge acceptance. Do not duplicate them. See [the research handoff](CLAUDE-C01-NEXT.md).

Codex has closed S19 and retains S20 shared UI, historical acceptance and final
integration. Human S26 playtests still require actual independent human participants.
