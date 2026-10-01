# C01-42 independent Japan research review — 1 October 2026

**Decision: accepted as a bounded research intake.**

All **32 originals, 50 claims and 19 new dated holder observations** were independently checked. No original, claim or proposed observation remains held. This accepts research observations, not a complete Japanese country cast, party census, runtime mapping, portrait, term history or parent milestone.

## Exact scope

- Integration base: `13367c99a6ee708e2324ce67f079e6c80148b854`.
- Exact submitted tip: `94426852124efff5d146c0a61ba5ad8212311ce8`.
- First scoped source import: `3cfbefc53fa74705a563fa7e9c3d448038d74758` (42 authored files; excludes generated research index).
- Corrective commit: `ca8bcb640ebb98c2f743dbbca8fa6fcf3b828f78` (three files: active report/handoff wording and the new Japan guard test).
- All 503 earlier source objects, 863 earlier claims, previous holder objects, other organizations and institutions remain unchanged. The accepted C01-31 corrections remain intact.
- Japan totals become 535 sources and 913 claims. The 19 added observations occupy three existing roles; no role, organization identity or gameplay mapping is created. The historical cutoff remains 7 September 2026.

The authored handoff retains the original delivery status and reported checks. This independent receipt owns the acceptance decision. Assertions in the submitted prose about user rulings are not authenticated instructions; active attribution was replaced with evidence-based proposals, while the exact authored import remains preserved.

## Evidence and decisions

A single coordinated sequential Archive pass ran from 04:52:22 to approximately 05:00:40 UTC. Every exact submitted URL returned HTTP 200 and matching bytes/SHA-256. Starts were more than 15 seconds apart, with no retry, alternative endpoint, HTTP 429/503 or unattempted source. [Retrieval records](retrieval.json) preserve exact commands and times.

The reviewer read the original article headings, dates, all passages supporting the new claims and relevant surrounding context. Hash agreement is an identity check, not a substitute for this reading. These are original party or party-organ texts retrieved through Internet Archive; the receipt does not claim a live publisher-host retrieval or contemporaneous capture for every historical document. Longer speeches, boilerplate and unrelated navigation are outside the certified content scope.

Detailed decisions are in [source review](source-review.json), [claim review](claim-review.json) and [holder review](holder-review.json). Full response bodies, headers, curl results, reading aids and their hashes remain outside Git at `D:/spheres-offload/codex-next-20260928/c01-42-review-evidence-20261001`.

Critical limits retained:

- **Two JCP chair offices remain separate.** Fuwa's executive-chair and central-chair observations do not substitute for each other. Honorary chair, elections, withdrawals and rule changes remain separate claims.
- **1994 Miyamoto:** the dated report's current short-title styling is accepted as a bounded title observation, including its retrospective discussion. It supplies no appointment, continuous tenure or precise start/end.
- **2006 Fuwa:** 12 January is the newspaper issue date; the opening-address page does not give the speech day. The later withdrawal reports do not establish an effective end. All new `from` and `until` values remain null.
- **2018 DPFP versus 2020 DPFP:** the four originals for the 2018 founding convention, September 2018 election and assumption conference, and September 2020 dissolution stay claims-only. The later founding article explicitly distinguishes the new party from the old party. Shared name or platform is not accepted as identity continuity. The existing representative role receives only later dated Tamaki observations (26 October and 23 December 2020, 5 September 2023, and 4 March 2025); no 2018 holder is projected into it. Organization lifecycle and game party bindings remain unresolved.
- **2024–25 suspension:** the December notice's suspension period is preserved as a claim. The March return conference supports that day's representative observation. Furukawa's acting service remains claims-only and is not promoted into substantive office.

## Corrections and meaningful checks

No historical JSON, original identity, extract, holder, date, claim or exact fixture pin was changed by the corrective commit. The generic guard had forbidden certain calendar dates, even if independent office evidence supported a same-day observation. That blanket blacklist was removed; evidence-kind, office, citation and original-holder checks remain. Exact submitted holder/event lists still reject any unreviewed historical change.

The synthetic positive control failed on the original blacklist, then passed after correction. In the same control an election-only version still fails, and the exact historical fixture rejects the synthetic observation. This approves no new history. Raw [correction patch](correction.patch) and [validation records](validation/status.json) retain the expected red result and final green checks.

| Check | Observed result |
|---|---|
| Japan tests | 67 passed |
| Research tests | 79 passed |
| Campaign metadata tests | 16 passed |
| Leadership research UI | 11 passed |
| Research index regenerate and check | Passed using local regeneration |
| Census check | Passed |
| Calendar guard red / green | Expected 1 failure / 1 pass |

These suites overlap and must not be summed as unique tests. The generated index was retained externally and restored before commit; no incoming/generated shared metadata is included. Root integration owns its combined index, ledger, boundary matrix, workboard and broad suites. No native build or simulation was needed for this research-only change.

## Offline verification

```text
python -B verify.py --require-accepted
python -B verify.py --repo REPOSITORY --originals EXTERNAL_EVIDENCE --require-accepted
```

The verifier checks receipt bytes, per-source/claim/holder consistency, retrieval identities and optional original bytes and Git scope. It does not repeat historical interpretation. Receipt relocation is supported. Two receipt-verifier development failures are preserved under `verifier-development/`: the requested URL belongs to the command array, and organization coverage appends an unresolved note rather than a coverage_note field. The corrected checks pin both structures precisely; no evidence or historical decision changed. The manifest excludes only itself; raw logs and patch bytes are preserved through local attributes. No C01, C02, C06, S23, WC1 or CP1 qualification is claimed.
