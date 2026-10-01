# CLAUDE-C01-39: resumed original review, one source still held

**The packet remains unaccepted. This is an additive review receipt only; no DA
country research, holder observation, correction or generated index is imported.**

Reviewed submission: `b66f8c074431d7e1a8c9bfca228576c7d5134655`, against canonical
base `4aaf5f05b422485d32ce6687df3bc1c55b109547`. The [initial held receipt](../README.md)
continues to describe `9cb02c20`; its HTTP 429 response and all earlier failed
checks remain intact. Alleged user rulings in the authored follow-up are not
approval evidence for this independent review.

## Original responses and material decisions

After the coordinating reviewer released the Archive lane, one ordinary curl
pass requested the 23 missing recorded URLs, with at least 15 seconds after each
response, no automatic retry, and a stop rule for HTTP 429/503. No rate-limit
response occurred. Twenty-two originals matched their submitted byte counts and
SHA-256 exactly. The prior 24 October 2019 original was verified and re-read from
retained bytes without another request.

One request did not reproduce its recorded source. The [requested 11 March 2021
capture](https://web.archive.org/web/20210311042629id_/https://www.da.org.za/2021/03/da-leader-john-steenhuisen-visits-and-supports-afrikaans-students-in-stellenbosch)
returned HTTP 302 to a 15 March 2023 capture and then HTTP 200. The returned
148,735-byte body has SHA-256
`dcec199a9516c8c25becf01419f6600b19820b97ac0ba207223e826cf5b7cde9`, whereas the
submitted original requires 134,450 bytes and
`730846b64891bac5bcd0fe512300a18b2413d9e8274fce780ff7027d04d5e32c`.
Both responses' headers and the returned body are retained. The later capture is
not substituted. Claim `za_da_statement_federal_leader_steenhuisen_20210310` and
the proposed John Steenhuisen observation on 10 March 2021 remain held.

Cumulative material review now supports **23 of 24 original identities, 28 of 29
claims, and 11 of 12 newly proposed holder observations**. These counts describe
reviewed support, not integration acceptance. The two pre-existing S10h holders
are preserved exactly and are not newly certified by this review.

The recovered [25 October 2019 statement](https://web.archive.org/web/20191028070115id_/https://www.da.org.za/2019/10/fedco-to-elect-interim-leadership-on-sunday-17-november-2019)
explicitly dates the vacancy to Wednesday. Its Friday dateline, corroborated by
the retained Thursday statement, resolves that weekday to **23 October 2019**.
This supports the proposed Maimane end as a calendar-derived vacancy boundary;
it is stronger than a report of an intention to resign. No clock time, tenure
start, interim-holder start or successor-derived end is supplied.

The recovered [Zille acceptance speech](https://web.archive.org/web/20070518152213id_/http://www.da.org.za/DA/Site/Eng/News/Article.asp?ID=7578)
supports the **6 May 2007 day attestation** through her own acceptance and address
to DA delegates as their elected leader. Her assumption date remains unknown;
the page's separate parliamentary-opposition label is not the party-office
anchor. Election results, interim service, undated former-leader profiles and
the February 2026 tribute remain distinct claims. DP/NNP formation statements
create no DA predecessor mapping or lifecycle boundary.

## Corrections and scope

Three precise passage locators need repair when the held packet can be imported:
the October vacancy is in the third paragraph, its election scheduling spans
the first two paragraphs, and the February 2026 ministerial-focus wording is in
the penultimate paragraph. [Proposed corrections](proposed-locator-corrections.json)
identify both country-claim and derived-row edits, with required snapshot hash
refreshes. They are not applied to canonical research, and original-response
identities must remain unchanged.

[Scope review](scope-review.json) confirms preservation of all 245 prior sources,
the two S10h holders, other organizations and roles, and the presidency.
[Checker review](checker-review.json) distinguishes the existing authored-handoff
binding repair from the later exact Zille/Maimane guards. The latter remains
bounded to named tuples; it does not authorize arbitrary ends or party/state
crossovers. A future import must preserve the newer central registration and the
immutable earlier authored handoff instead of overwriting either.

## Verification and resumption

`verify_receipt.py` verifies committed Git blobs rather than checkout newline
bytes. It pins the exact reviewed submission/base and optionally checks all 98
external original/header/error/reading-aid files. It does not repeat historical
judgment or turn a held packet into an accepted one. No native run, newly
installed packet test suite, runtime mapping, art permission or country-cast
approval is claimed. C01/C06/S23/WC1/CP1 remain open.

```powershell
python -B -X utf8 docs/campaign-certification/C01/reviews/CLAUDE-C01-39/2026-10-01-resumed-02/verify_receipt.py
python -B -X utf8 docs/campaign-certification/C01/reviews/CLAUDE-C01-39/2026-10-01-resumed-02/verify_receipt.py --verify-external
```

Resume only the exact missing 2021 original, its claim and dependent holder.
Preserve this redirect attempt, the original 429, and all prior material findings.
External evidence is in
`D:/spheres-offload/codex-next-20260928/claude-review-20261001-05/da39-evidence`;
the prior seven files remain under `c01-39-review-evidence-20261001b`.
