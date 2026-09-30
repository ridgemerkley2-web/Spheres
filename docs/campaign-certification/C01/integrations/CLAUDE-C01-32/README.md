# CLAUDE-C01-32 independent review — 30 September 2026

**Recommendation: intake the bounded PAC research packet with the attribution correction below.** This is a source/technical review receipt, not a merge or parent-session completion. Reviewer: Codex `/root/s20_preflight`, independent of the Claude submission author.

Reviewed `origin/claude/c01-za-32` at `df4707462903b893916b5ef2d56a875a82f4030f`. The own-packet delta is measured from `4fe2f1ba13a3595ac2c4affb9975132fcd666429`: 48 files, 38 new source records and 65 claims, one PAC party role and 14 dated holder observations. The inherited C01-30 packet was reviewed separately; do not blanket merge this stacked branch. Preserve that review's later prose correction when applying the country record.

## Finding and correction

The PAC's 1 September 2019 post reproduces the opening text of [SABC News, “PAC re-elects Mzwanele Nyhontso as president”](https://www.sabcnews.com/sabcnews/pac-re-elects-mzwanele-nyhontso-as-president/), by Makgala Masiteng, without credit. The posted report was incorrectly typed as a primary party statement. Fix `8d545b1925b6de125586f0ae6ba7a4d28562507d` classifies it as `party_republished_news_text`, attributes the original and adds two negative mutations. The original article was consulted on 30 September only to identify copied authorship; it is not a new historical source or holder attestation. The single result-publication claim was already claims-only and remains excluded from holders. The raw response pins, claim count, holder list and every office date remain unchanged.

The index-only follow-up is `f2559b84a04f1ae88c82272090e818dce3ef702e`. Claude's authorship, original retrieval records and earlier verifier rulings remain intact. [The attribution comparison](sabc-attribution.json) records the independently fetched publisher identity and a normalized whole-body comparison without copying the article into Git.

## Source and scope review

All 38 recorded raw archive responses were independently retrieved and matched their submitted byte length, SHA-256 and recorded base32 SHA-1. [The source ledger](source-verification.json) records every response, and [all retrieval attempts](source-attempts.json) retain 21 initial connection failures and one later retry failure before complete recovery. These are body-byte checks; no new CDX query is claimed. No source remains held for failed retrieval.

[All 65 claim records](claim-review.json) were reviewed against the cited original text and locator, with publication, event, edit-stamp and court-order dates distinguished. The scanned Moloto statement was visually read on both pages; the appeal PDF was visually reviewed on pages 1, 3, 4 and 10. Poppler emitted two missing-font warnings while rendering the appeal PDF; the relevant text remained readable and agreed with its text layer. Two literal quote checks differed only in apostrophe typography; the scanned statement had no text layer. Those exceptions were manually checked, not silently counted as exact text matches.

The existing 14 holder observations remain discrete observations of six people; all `from` and `until` values remain null. Six observations keep explicit dispute markers. Acting service, rival claims, elections, expulsions, suspensions, results, orders, edit stamps and other offices do not become holder boundaries. The existing unknown party lifecycle and empty game mapping remain unchanged. [The scope audit](scope-audit.json) verifies that the earlier source prefix and institutions are unchanged, and the PAC is the sole changed organization in the original delta. The review correction changes no organization object at all.

## Checks and retained limits

Corrected-run logs are under [validation/corrected](validation/corrected/results.json): 50 South Africa Python tests, 79 research Python tests, 16 campaign Python tests and 11 leadership-atlas Node tests pass. The 145 Python count is a sum of overlapping test executions, not a unique-test count. Research-index, workboard and diff checks pass. The PAC method count remains eight; its packet-invariant mutations increase from 45 to 47, with the existing seven validator mutations retained.

The [baseline census limitation](baseline-limit.json) remains: `campaign_census.py --check` exits 1 because its government source pin records 848,551 bytes while the actual unchanged input is 849,546 bytes. Both the government file and stale census have identical Git blobs before and after this packet. This review did not regenerate an out-of-scope census. Gap-ledger packet classification and the S23 boundary matrix must be refreshed at integration; they were not claimed as passing in this sparse review.

Initial sparse-checkout missing-input failures, a reviewer console-encoding error, the first attribution-test draft failure and the pre-existing census failure remain recorded separately. No Cargo, runtime simulation, game/art change or whole-avatar-suite pass is claimed. C01, C06, S23, WC1 and CP1 remain incomplete. This does not approve uninterrupted historical terms, new game mappings, portraits, likeness reuse or exhaustive South African coverage.

## Retained evidence

`manifest.json` pins this compact review bundle by exact bytes; the local `.gitattributes` preserves those bytes across checkouts. Original response bodies, headers, private text extraction and rendered pages remain at `D:/spheres-offload/codex-next-20260928/c01-32-review-evidence-01`, with their paths/pins in the receipt. They are not republished in Git. A manifest check confirms receipt integrity, not a fresh historical-content review.

Run `python -B verify_receipt.py` from this directory to verify the checked-in receipt, or supply `--external-evidence D:/spheres-offload/codex-next-20260928/c01-32-review-evidence-01` to additionally rehash all 38 retained raw bodies. Neither mode downloads sources or independently repeats the human-readable content review.
