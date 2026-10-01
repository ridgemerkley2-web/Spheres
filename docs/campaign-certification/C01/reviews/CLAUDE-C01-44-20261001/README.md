# Partial independent review held — CLAUDE-C01-44

Decision: **held**. Only this review receipt may be integrated. No C01-44 country
research, source records, holders, runtime mapping or art is accepted for integration.

Reviewed submission: `206df90553337f4fbeb4ae34819cb2b01c6e8a21`, against integration
`229df2105f1ca46bc55873ea4ab0ada63255ced3`, on 1 October 2026 UTC. Codex `/root`
retrieved and materially read the accessible originals; `/root/nation_art_scope`
checked preservation, guards, the retained legislature text and this receipt.

## What was verified

**Four of five original responses match the submitted bytes and SHA-256 and were
materially read. They support five of six claims, within the limits below.** There
are **zero new holder observations**. The packet remains held as a whole; these
partial findings do not authorize importing its accessible subset.

| Original | Claim decision and limits |
|---|---|
| Assembly's 2013 PGA award article | Supports an attributed September 2010 founding statement. The article explicitly republishes PGA Forum material; official hosting does not make that text primary authority for a founding day or party office. |
| 2014 Tonga Law Reports, PDF pages 31 and 33 | Supports the dated judicial background recital identifying Pohiva as leader of the **Tonga Democratic Party**. It does not establish that name as an alias of DPFI, an effective term boundary or a DPFI holder. |
| Court of Appeal AC 9/2015, PDF pages 1, 3 and 21 | Supports the legal-form/capacity recital and, separately, the report of Tongasat's argument about party funds. The latter is not a court finding that the party lacked funds. Neither establishes party office or registry status. |
| Assembly member profile captured in 2016 | Supports a retrospective establishment statement at year precision, 2010. It supplies no exact founding date or leader-office evidence. |
| PMO reply attributed to March 2020 | **Held, unread.** Its Tongan wording, English rendering, release date and attributed legal-status assertion have not been independently verified. |

[source-review.json](source-review.json) and [claim-review.json](claim-review.json)
pin every decision. No materially accessible claim was deferred. These findings
do not resolve the disputed party-name mappings or create lifecycle boundaries.

The PMO request at **03:54:27 UTC** returned **HTTP 429**, not the source article.
Its retained 117-byte body hashes to
`ae138caf8767f7be2fe6f47f1663b0e2e28d903264707aa9b6f73bb7b223902c`.
There was **no Retry-After header**. The coordinated Archive pass stopped; no
original was retried or fetched by this follow-up. Exactly one source/claim is
unverified; the rate limit does not show that the claim is false. Raw originals,
headers, failure body, reading aids and PDF page images remain outside Git under
`D:\spheres-offload\codex-next-20260928\c01-44-review-evidence-20261001`, with pins in
[external-files.json](external-files.json).

## Isolated import, correction and checks

- Original scoped import: `087b92eadc162d6ae465a10273a6b9e52ebed92d` — exactly ten
  authored paths from the submitted tip; generated research index excluded.
- Prose correction: `9955de17c16f3c13c0e042487b604fd8a5960433` — the report now says
  no **additional** primary record was found by this packet and explicitly
  preserves the existing **2022 Fatai Helu presidency** evidence. The original
  blanket statement contradicted that retained observation. [Exact repair](report-correction.patch).
- **Do not integrate either commit while held.** The separately committed receipt
  is the only authorized publication. The isolated review branch is
  `codex/review-c01-44-20261001`.

[scope-review.json](scope-review.json) verifies that all **207 prior sources, 319
claims, 194 extract blobs, roles and holders** are unchanged. Additions are five
sources, six organization-level claims and unresolved coverage notes. No old test
assertion was weakened: S10g changes only its totals/comment; the eleven new test
methods include ten semantic negative mutations and checksum rejection.

Focused checks on the isolated corrected tree passed: **101 Tonga tests, 31
country-cast tests and 9 shared research tests** (141 total). The research builder
also validated the candidate in memory; its generated index stays outside Git.
[Commands and log pins](validation.json) retain the exact outcomes. No native,
full-avatar, live campaign or art generation run was performed. Green tests and
matching hashes are technical checks, not historical approval. C01, C06, S23,
G5 and CP1 remain open.

Run [verify.py](verify.py) offline to recheck compact payload hashes, exact authored
scope and prior-data preservation using the pinned Git objects. Add
`--external-root D:\spheres-offload\codex-next-20260928\c01-44-review-evidence-20261001`
to rehash retained originals and failed retrieval evidence. The verifier makes
no network calls and does not approve historical claims.

Resume only the single missing PMO original after a coordinated later source
retrieval. Read its content and resolve its one claim before reconsidering the
packet; preserve this failed attempt and all existing claim boundaries.
