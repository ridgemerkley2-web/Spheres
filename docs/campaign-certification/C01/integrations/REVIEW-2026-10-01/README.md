# Independent research reviews, 1 October 2026

This integration accepts three bounded research packets and retains one held
review. It also records a new delivery awaiting source-content review. Research
acceptance does not install identities or avatars in the game, complete a country
cast, or close C01/C03/C06/S23/CP1.

| Packet | Decision | Evidence |
|---|---|---|
| C01-38 France PMs | Accepted: 22 exact originals, 33 claims, twelve dated appointment observations. Same-day interval prose corrected; effective terms remain unknown. | [Review](../CLAUDE-C01-38/README.md) |
| C01-40 CPI(M) | Accepted: eighteen exact originals, 24 claims, four holder observations. Five locators and unsupported opening-holder prose corrected. | [Review](../CLAUDE-C01-40/README.md) |
| C01-41 CPSU | Accepted: twelve exact originals, 26 claims, five holder observations. Reportage attribution and decree annotation scope corrected. The extra original-edition comparison is supplementary evidence, not a thirteenth source. | [Review](../CLAUDE-C01-41/README.md) |
| C01-31 Komeito | Held: 24/31 originals and 50/64 claims reviewed; seven originals, fourteen claims and five holder dependencies remain unresolved. Only the review receipt is integrated. | [Held review and conservative boundary patch](../../reviews/CLAUDE-C01-31/README.md) |
| C01-39 DA | Newly delivered at `9cb02c20`. Scope triage confirms 24 new sources, 29 claims and eleven added observations, preserving 245 prior sources and two prior DA holder objects. All new sources use Archive. Original-source review and independent tests remain pending. No new DA research is imported. | [Registered delivery](../../../../planning/ai-handoffs/CLAUDE-C01-39.md) |

The three accepted packets add **52 sources, 83 claims and 21 holder observations**.
Original-response identity and content review are separate steps. Reviewers read
the original HTML/XML passages and cited PDF pages; passing parsers or hashes alone
did not establish acceptance. Original failures, uncertainties, source-host
qualifications and unchanged prior research remain recorded in each receipt.
Full original response bytes stay outside Git; receipts pin their identities.

Accepted source branches are merged to preserve the first-source import commits
used by the gap ledger. Japan's receipt alone is cherry-picked; neither its source
import nor its boundary correction is installed while the packet remains held.
The task queue, provenance classifications, research index, gap ledger and S23
preparation matrix are refreshed together. Runtime code, game data, art, saves and
the user's servers are unchanged by this review integration.

The campaign-leader art direction remains in the [Claude handoff](../../../../planning/ai-handoffs/CLAUDE-CAMPAIGN-LEADER-ART.md):
use the actual dated campaign leader and the matching reviewed cartoon, never a
timeless national representative. Next content work resolves held sources, maps
Tonga's accepted identities, then produces a reviewed 6–8-character Tonga batch
before repeating the remaining certified-country casts.

Combined validation passed **627 avatar/research tests, 69 planning tests and
eleven Node atlas tests** (707 test executions). Research-index, census, gap-ledger,
date-boundary matrix, cartoon-review, workboard and whitespace checks also passed.
The queue has 44 bounded tasks and the pathway retains 44 canonical markers;
Claude has 25 complete bounded tasks and three submissions awaiting acceptance.
The first combined attempt is retained: India's decision sentence was not in the
parser's established format, the matrix had not yet regenerated, and its expected
accepted-packet set needed the three newly reviewed entries. The repair adds the
explicit declaration, repins that README only, and updates exact expected packet
sets; it does not loosen a parser, test limit or historical criterion. Both failed
and successful logs are in [validation](validation/).

The original C01-40 review remains pinned at `6ee593de`; the precise declaration
change and before/after hashes are in [the repair record](acceptance-declaration-repair.json).
The [scope audit](scope-audit.json) verifies unchanged game code/data/assets and
six untouched country packets. All prior sources in France, India and the USSR
remain unchanged. It also checks the retained USSR review's 61 input/receipt pins.
An initial checkout-only pin check exposed Git's CRLF expansion of source text;
affected inputs are checked against exact merged Git blobs with explicit LF text
equivalence. Original pins remain unchanged; byte-preserved receipt files still
require exact checkout bytes. Per-file verification methods are recorded.
The [review revisions](review-revisions.json) preserve exact remote submissions,
accepted source imports and excluded held-research commits. Full native builds,
campaign simulations and human playtests were not repeated for this research and
planning-only integration; no new gameplay qualification is asserted.

Run `python -B verify_receipt.py` to check this retained receipt. Its manifest and
logs establish integrity of the reported result, not a repeated historical review.
