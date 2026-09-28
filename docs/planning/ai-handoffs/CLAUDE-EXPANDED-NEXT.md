# Claude — six additional independent sections

Assigned 27 September 2026 by the user's request, “Give Claude more sections to work on.”
Base: `5979cf2fba78a6784a549d63223ca274950a2f7d`; fetch the latest
`origin/codex/campaign-certification` before claiming a packet.

Updated 28 September 2026 UTC after S22 closure and independent preparation reviews.
**Four bounded deliveries are complete:** C01-GAPS-01, C03-REVIEW-01,
C04-PREP-01 and S23-MATRIX-01. Follow their updated handoffs for accepted scope
and retained evidence. These completions do not close their parent content sessions.
S24 `5a23ebe0` and E05 `1fe45b2c` are now **ready_for_review** after the latest
fetch. Only their handoffs are mirrored; content remains unmerged and unaccepted.

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

SOURCE-05/06/17/26 are **complete** after independent bounded source/content review;
see their integration evidence. C01-23/24/25/27 remain `ready_for_review`;
their parent historical content is not yet accepted. C01-28 and C01-29 preserve
active Russia/Japan research claims. Do not duplicate those submissions or claims.
See [the research handoff](CLAUDE-C01-NEXT.md) and query the current task queue.

Codex has closed S19/S20/S22 and earned G4. It retains historical acceptance and
final integration. S23 remains planned pending C06; the next content slice can
advance an accepted Tonga research batch into reviewed identities and a small
cartoon batch. Human S26 playtests still require independent human participants.
