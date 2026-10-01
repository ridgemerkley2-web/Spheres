# C01-28 Russia: missing-original review continuation

**Decision: held pending three original responses.** This additive receipt is
eligible for documentation-only integration. The Russia research packet and its
previous precision repairs remain isolated and are not accepted for import.

Codex reviewed the unchanged submission
`03141c43c5663e35d21eebc631aaf5eec4e909aa` on 1 October 2026 UTC in
`codex/review-c01-28-20261001`, based on the preserved review tip
`0dec078f747575f12eb851b15dc97602e2a06bfe`. The task's integration snapshot was
`509bd2890f71850c97215d304ff16b757c7d49c2`.

| Evidence | Previously reviewed | Newly checked | Cumulative | Still held |
|---|---:|---:|---:|---:|
| Original response bodies | 56 | 9 | 65 of 68 | 3 |
| Material claim passages | 113 | 20 | 133 of 137 | 4 |
| Dated holder observations | 20 | 4 | 24 of 24 | 0 |

The previous 56 responses were rehashed without requesting them again. The
[30 September available-content review](../2026-09-30-available-content/README.md),
its 113 claim findings, and the [initial held receipt at its immutable commit](https://github.com/ridgemerkley2-web/Spheres/blob/a074a0f9f290acefab6f782d7854722436a68967/docs/campaign-certification/C01/integrations/CLAUDE-C01-28/README.md)
remain unchanged. The cumulative totals reuse those checks; this continuation
does not represent a fresh review of every previously read passage.

## Retrieval and stopping condition

After the shared Archive slot was released, one missing-only pass used the exact
recorded URLs, serial plain curl requests, ten seconds between a completed
response and the next request, a 45-second timeout and at most three redirects.
The first nine responses were HTTP 200 and matched their recorded lengths,
SHA-256 values and served-body SHA-1 values.

The tenth response, the DVR Political Council statement of 16 December 2000,
returned HTTP 429 at `2026-10-01T01:36:26.122206Z`. It contained no `Retry-After`
header. The pass stopped immediately; the two later sources were not requested.
There were no further Russia retries, alternate URLs, host substitutions or
encoding-based identity substitutions. All earlier failures remain preserved.
[Retrieval receipt](retrieval.json), retained response headers, and the
[exact retrieval recipe](recipes/retrieve-missing.py) record the pass.

| Missing source | Exact held claims | Holder observations depending on it |
|---|---|---:|
| `ru_dvr_politsovet_statement_20001216` — HTTP 429 | `ru_dvr_politsovet_statement_signed_chairman_20001216` | 0 |
| `ru_dvr_about_page_2001` — not attempted after 429 | `ru_dvr_about_page_chairman_gaidar_2001` | 0 |
| `ru_dvr_newspaper_demvybor_21_2001` — not attempted after 429 | `ru_dvr_x_congress_self_dissolution_decision_2001`; `ru_dvr_x_congress_gaidar_speech_dissolution_2001` | 0 |

The [remaining-holds record](remaining-holds.json) retains their original URLs
and claim locators. An inaccessible source is not evidence that its document or
assertion does not exist. Zero remaining holder dependencies does not satisfy
the four unresolved claims or accept the packet as a whole.

## Content findings

The nine recovered sources comprise five HTML pages and four scanned PDF
documents. All seven pages of those four PDFs were rendered and visually read;
OCR was used only for navigation. The scan warnings about fonts did not obscure
the printed name, title or date passages. [Visual-review pins](visual-review.json)
retain each rendered page's identity.

- Yabloko's 2008 decisions distinguish the 21–22 June congress from the 26 June
  release and do not establish a precise chairmanship start. The December 2015
  pages distinguish the incumbent's nomination, the reported election, a
  congress span and retrospective year ranges. Their datelines print day and
  month; the dated paths, capture context and related congress heading supply
  the year. No overlapping tenures or new exact boundaries are inferred.
- The 1995 DVR statement genuinely has an 18 June heading and 17 June adoption
  line. Its null attestation is retained. A separate decision explicitly adopted
  on 18 June names Gaidar as party chairman and supports that day's holder
  observation without resolving the conflicting statement.
- The 1996 congress records separate council nominations and counting-protocol
  approval from the printed chairman signatures. The 22 September signature is
  an in-office observation; the preceding ballot-process wording is not a named
  election result or proof of a term start.
- The 1997 Political Council page prints its 16 December adoption and Gaidar's
  party-chair title. The 1998 capture is not a document date. The other 1998
  page explicitly credits a 1996 reference book by Korgunyuk and Zaslavsky.
  Its party-site republication remains attributed retrospective narrative,
  separate from a contemporary registry or founding instrument. Its inconsistent
  IV-congress chronology remains outside the imported claims.

Private archive hosting is distinct from authorship of the party records. A
reproduced scan does not independently authenticate the paper original, a
signature, legal continuity or complete biographies. No image or portrait rights
are granted by these sources.

The two 1998 DVR responses now reproduce the recorded served-body SHA-1 values,
which differ from their separately recorded Archive index digests. The retrieved
headers confirm original chunked transfer. This pass did not fetch CDX or prove
that chunking caused the digest differences; it does not replace either identity
or describe the index digest as independently verified. Acceptance of the read
passages here is bounded to the identified served response and its attribution.

[Per-source rulings](source-content-review.json), [20 claim rulings](claim-review.json)
and [four holder rulings](holder-review.json) give the exact scope. No new
substantive contradiction requiring a packet edit was found in those passages.

## Preservation and validation

The isolated Russia tests passed: **39 tests**, observed on this review checkout.
All 56 prior bodies and all ten new response/header pairs, including the 429,
were rehashed. The old receipts, Russia data, USSR data and inherited SOURCE-05
are byte-identical to the review base; the earlier two chronology qualifiers at
`35ea76f65723f2213c893bf8338d1b146508dd0a` remain preserved. No research, extract,
test, index, mapping, runtime, artwork or central queue file is changed by this
receipt. See [preservation pins](preservation.json) and [validation](validation.json).

Original response bodies, full text and page renders remain outside Git in
`D:\spheres-offload\codex-next-20260928\ru28-originals-20261001`; the earlier
responses remain in the retained `ru28-review` directory. Their compact identities
are committed here. No Cargo, game, browser, campaign or artwork qualification is
claimed. C01, C06, S23 and CP1 remain open.

Only this new receipt directory should be cherry-picked while the packet is
held. Do not merge the continued review branch or use its old generated index
as current integration data.
