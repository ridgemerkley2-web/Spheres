# S10.g — Clear sanctions and relationship decisions

Status: **S10.g complete; S10 and C01 remain in progress.** G2 is unearned,
S11 is planned, and execution stopped after this increment.

Open the [review build](http://127.0.0.1:7853/) and choose **Decisions →
Sanctions & relations**. It contains the ordinary France campaign used for
the Government browser check, dated 1 January 1990. The authored sanctions
fixture is retained separately as test evidence.

## What changed

The sanctions desk separates restrictions imposed by your government from
restrictions against it. Choose a country to review sanctions, lifting your
sanctions, or improving relations. Only the imposing government can lift its
own restriction, and selecting a country does not submit an order.

Reviews keep the current and proposed bilateral conditions visible: relations,
sanctions in both directions, other sanctioning countries, conflict, saved
trade agreements, freight routes and the sanctions component of growth drag.
Removing your restriction can leave other barriers in force. A saved agreement
can remain present immediately after a decision and be removed at a later
settlement. Route availability does not book cargo or guarantee delivery, and
the displayed growth component is not a total growth forecast.

Improving relations at the limit now explicitly shows that the relationship
cannot increase while the political-capital cost still applies. Refused reviews
retain current conditions without an invented after-state or confirmation
token. Repeated sanctions and empty lift orders disclose their native no-op
behavior. Existing stale-review, pending-receipt and foreign-authority guards
remain active. No simulation rule or save schema changed.

## Qualification

All runs used clean runtime and browser-driver revision
`0f4616477671bc01936fcc9f2877ae172b13e348`, with no runtime/driver delta.
The Windows game binary SHA-256 is
`dc0b66412cbe8b25553c9e91435df419fdf7e33d9e56baa40f1e888476a46a72`.
The [manifest](manifest.json) records each result and its scope.

| Check | Result |
| --- | --- |
| Windows native web | 387 passed, 16 ignored |
| Linux native web | 387 passed, 16 ignored |
| Windows interface | 1,555 passed, 1 skipped |
| Windows agency/succession | 12 passed |
| Linux agency/succession | 12 passed |
| Content | 66 tests and 5 reproduction checks, 12 commands |
| Authored sanctions browser | 13 native inspections, 9 exact world comparisons, 9 envelope comparisons |
| Ordinary France browser | 8 native inspections, 5 exact world comparisons |
| Native open-route review timing | 244.99 ms p95; 274.78 ms maximum |

The authored France fixture deliberately starts with 100 political capital,
France–Japan relations of 20, reciprocal sanctions, India→Japan and
Brazil→France sanctions, and a saved France–Japan trade agreement of depth
0.425. These are authored test conditions. Three reviewed commands—lift,
sanction and improve—match independently applied native commands exactly.
Four previews include a canceled initial lift. Reverse and third-party
restrictions persist, while political capital progresses 100→97→91→89 and
relations progress 20→20→5→11.

The browser checks cover cancellation, foreign inspection, Save, canceled
Load, completed Load and Continue at desktop and narrow widths. Whole-world
comparisons ignore no paths; envelope comparisons retain the full history and
validate save timestamps separately. The ordinary Government journey also
checks a stale review from a real second tab and a fresh confirmed decision.
Both journeys advance **zero campaign days**. Ten actual screenshots were
manually inspected alongside the automated visibility and hit-testing checks.

The separate performance test uses the existing 134,238,603-byte Tonga save
dated 29 October 1995 and reviews improving relations with the United States.
Both directions have actual native freight routes before and after the preview.
After three warmups, 21 measured samples meet the predefined 300 ms p95 and
750 ms maximum limits. Timing includes the cloned native command, four route
checks and complete-world review token; it excludes HTTP, rendering and target
discovery. No political-capital floor was needed. Source bytes, benchmark world,
log and history remain unchanged, and no time advances. The existing S08
compressed source object is hash-bound and reconstructed for verification,
without duplicating the source archive in this checkpoint.

This increment does not repeat the full Linux interface suite, eight-country
startup matrix, external old-save matrix or long-campaign/Russia activation
checks. It does not certify a campaign through 2035.

## Bounded Tonga discovery

The [Tonga packet](../../C01/research/tonga.json) adds three primary sources and
five factual claims, including one provisional People's Party organization
observation and an event-host observation of its leader on 28 May 2021.
Assembly selection and royal appointment as prime minister remain separate
from party leadership. The court's PATOA/PTOA spelling discrepancy is retained
without reconciliation. Original court and appointment pages were visually
reviewed; only derived factual extracts are checked in, with downloaded-response
hashes recorded separately from extract hashes.

The research index now contains six country packets, 774 organization
observations, 18 institution observations, 48 sources, 1,465 claims and 83 open
discovery batches. There are zero exhaustive country censuses. These counts
do not establish distinct-party totals, complete leadership terms, artwork
permission or additional gameplay candidates. The research cutoff remains
7 September 2026.

## Retained evidence and preservation

The [inventory](evidence/inventory.json) describes 156 logical files stored in
157 files, totaling 98,259,482 stored bytes excluding the inventory itself.
It retains native and interface logs, content checks, four authored oracles,
browser archives, screenshots, launch verification, performance results and
the exact runners. Original selected bytes, compressed reconstruction and Git
index bytes are verified before publication. Compiled executables are identified
by hash and are not bundled.

The preservation check passes for eight protected files and two worktrees.
Previous saves, evidence and review servers remain intact. The new review
server uses its own copied ordinary save and matches the final browser world's
native canonical bytes.
