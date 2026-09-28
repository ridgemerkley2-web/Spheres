# CLAUDE-E05-RESEARCH-01 bounded integration review

Decision: accept the eight-company research preparation package and repaired offline validator for integration. This does **not** certify all 195 factual claims, complete E05, install real companies, grant any intellectual-property licence, or approve runtime opening balances, capacity, stock or production.

Reviewed Claude commit `1fe45b2cae22538240a58ad2407f31d0c0fa213e`; scoped 12-file import `13a6b5b4c648698cacfb693851488d0181bdca56` preserves its author. Repair `8c2b3e1d235c1dfea223e5e7e4d27641ce6775b2` changes only the catalog explanation, validator and tests. No global handoff, workboard or runtime edits belong to this packet.

## Findings and validation

- Legal rename periods were incorrectly inclusive at an exact end date. The prior name is now rejected on a precise rename day; the new name is accepted. `last_observed` remains inclusive, and year-only dates retain uncertainty.
- A source declaring a changed response could omit its recheck hash/byte count. Every recheck now needs a SHA-256 and a positive integer byte count, rejecting booleans and numeric strings.
- The catalog overstated anchor verification. The offline checker validates recorded metadata and declared date coverage; it never downloads or reads a source body.
- The expanded 61-test suite exposed 9 failing subcases before these repairs and passes 61/61 after them. The actual eight-dossier checker passes against pinned game inputs. Exact commands and result logs are in this packet. The checker summary's 65 stable / 34 changed / 10 inaccessible are **author-declared catalog counts**, not the independent retrieval totals below.

## Independent source observations

On 28 September 2026, ordinary default-User-Agent HTTP requests obtained 68 of 99 source bodies: 54 exactly matched the original recorded response and 14 differed. Thirty requests returned 403 and one ANAO request timed out. No access control was bypassed, and there was no alternate-user-agent retry. Full external bodies are retained locally under the evidence root recorded in retrievals.json; this Git packet distributes response identities/statuses and review metadata, not the PDFs/HTML or any art rights.

All 150 recorded anchors associated with the 68 returned bodies were located. Four first-pass misses were local extraction limitations, corrected without another request: three BODACC records contain nested JSON strings; one Japanese MOD page declares Shift_JIS in HTML. Original first-pass records/logs remain unchanged and corrections are separate. Matching an anchor is not a factual review.

Every one of 195 claims has an explicit content status in claim-review.json: **20 bounded direct content checks, 26 partial direct checks, 10 primary web-text checks without exact body reproduction, and 139 not independently content-verified.** Changed bodies were separately read at selected claim locations: corporate timeline prose, registry identity/address entries and Toyota releases. The per-claim notes distinguish statement checks from partial context checks. Dates missing from a spot check and broader negative historical claims are not promoted merely because the anchor occurs. The file lists all 45 claims without a direct body and all 165 claims not fully content-checked. None of these status counts is a completed historical-certification gate.

Public web-tool views of four original institutional URLs corroborated ten selected claims despite failed direct retrievals. [MHI's Sagamihara history](https://www.mhi.com/jp/company/location/sagamiharaw/history) distinguishes tank production dates from service entry; [MHI's annual report](https://www.mhi.com/jp/finance/library/financial/pdf/2025/2025_04_all.pdf) gives the reviewed corporate-history entries and submission cover. [Naval Group's history](https://www.naval-group.com/en/history) dates the company transition to 2003; [MOD's 2022 APC decision](https://www.mod.go.jp/atla/pinup/pinup041209.pdf) identifies the selection and preceding cancellation. These are text-only observations, not reproduced historical response hashes or independently rendered PDF-page checks. They do not alter the 31 direct access limitations.

## Company and game boundaries

The pilot contains France's KNDS France, Dassault Aviation, Naval Group and Renault, plus Japan's MHI, Kawasaki Heavy Industries, Komatsu and Toyota Motor. Identity spot checks distinguish company numbers, observed names, subsidiaries and namesakes. The dossier design explicitly separates GIAT holding versus Nexter/KNDS legal persons, the DCN state-directorate/company transition, Renault's alliance versus ownership, MHI versus Mitsubishi Motors, KHI versus unrelated Kawasaki companies, and Komatsu's separately numbered namesake. These are reviewed modeling boundaries, not certification of every associated date or percentage.

The checker resolves all 32 proposed targets against existing game IDs and records eight input hashes. These remain proposed mappings with confidence and consumer gaps, not installed suppliers. Naval Group's `msl_deterrent` proposal is a generic carrier/force-tier reference and does not establish missile manufacture or ship stock; ships await the missing consumer. Existing Establish/Capitalize/Develop/Inventory/Purchase flows remain the intended paid sequence. This review grants no platform, technology, contract, licence, warehouse input, free stock or opening balance.

Remaining gates include the explicitly unverified claims, independent historical conflict resolution, full product-to-capability validation, licensing/art approvals and any future runtime integration review. Announced/cancelled products remain distinct from delivered equipment. The 1990–2026-09-07 historical window and fictional-future policy are unchanged.

## Reproduce

From the repository root:

```powershell
python -m unittest discover -s tools/planning -p test_company_pilot.py
python tools/planning/check_company_pilot.py
python docs/campaign-certification/E05/integrations/CLAUDE-E05-RESEARCH-01/verify_packet.py
```

The packet verifier checks frozen evidence bytes and exact claim/source membership. It performs no network requests, makes no historical truth inference and does not require the separately retained copyrighted original bodies. Frozen reviewed-input copies avoid later checkout line-ending or source changes altering this review. The retrieval script records the ordinary original request/extraction method; running it again would be a new observation, not reproduction of historical live-page bytes. Its initial syntax failure is retained; no requests occurred on that failed invocation.
