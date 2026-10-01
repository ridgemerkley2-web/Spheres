# C01-43 independent Brazil PRN / PTC / Agir review

**Decision: accepted as bounded research intake, with narrow corrections.**

Independently retrieved and read **14/14 submitted originals**, **21/21 claims**
and **5/5 selected Daniel Tourinho observations**. No source, claim or observation
is held. This is research acceptance; the integrator owns task registration and
combined generated records.

Submission: `68fb8f863398247ba1515a1ff3f413a444a6d7f5`; substantive author commit:
`bf858a1ecf97a80afb96c118d42dc7a8167c852e`. Review base:
`13367c99a6ee708e2324ce67f079e6c80148b854`. Authored import:
`0a6b571dc1c8aefe331c7bbfde9ba1ebf1b458a8`; correction:
`878a48bcaea12a3ef89037a7dfb2028297fea3c6`.

## Original evidence and decisions

One coordinated sequential pass retrieved the fourteen exact archived URLs on
1 October 2026, with fifteen-second waits between requests and no retries,
redirects, alternate endpoints or credentials. All returned HTTP 200 and matched
the submitted raw-body pins; both gzip decoded pins also match. The archive is
custodian, not the author of the court or party records. Full bodies, headers,
curl metadata, decoded files and renderings remain outside Git. Exact times and
commands are retained in [source-attempts.json](source-attempts.json).

Hash equality was not treated as content review. The reviewer separately read
every claimed HTML section with its surrounding article/table, and visually read
all four pages of the three PDFs. The second minutes page contains notarial
signature recognitions, not another leadership claim; no personal administrative
fields or signature images are copied into Git. [Visual review](visual-review.json)
identifies the external renders. No SGIP request was made.

- The electoral court's registry and glossary directly support the PRN's 1990
  definitive registration, PRN-to-PTC renaming in 2001 and PTC-to-Agir renaming in
  2022. This supports one research office across those names on the existing
  funding observation. It does not reconcile a game identity, infer a continuous
  lifecycle or conflate the unrelated rejected PTC(1988) glossary entry.
- The five selected observations are supported on **2014-05-16, 2018-07-25,
  2020-08-07, 2021-07-23 and 2022-11-11**. The 2018 signed scan explicitly prints
  the national title. The 2020 resolution's signature day is distinct from its
  20 August publication; its effectiveness clause concerns the resolution, not
  Tourinho's assumption of office. Every `from` and `until` remains null.
- The 2015 convention article genuinely styles Tourinho national president. It
  does not identify a newly elected president. The 2022 article separately
  reports the preceding day's meeting and styles him national president. Keeping
  those as claims rather than extra selected observations is conservative
  selection, not proof that event-day office evidence is invalid.
- The 3 July 2018 minutes use President in national-executive context. They do not
  say he was merely the meeting chair. The 19 July notice explicitly uses President
  of the National Directorate and announces a future convention; its exact title
  remains a claim without silently equating offices or inventing a result.
- The party-hosted historical narrative is republished court-voice reference
  text, not a contemporaneous original court record. Undated rosters/listings and
  the undated plenary article remain undated. Capture dates supply no office day.
- The PDS/PDC-to-PPR fusion is a supported court-reference claim only. No PDS
  research entry, office, holder or game mapping is added.

[Source decisions](source-review.json), [claim decisions](claim-review.json) and
[holder decisions](holder-review.json) preserve the individual rulings. Repeated
observations do not establish continuous service between them. No claim beyond
the submitted packet was added from incidental content in the originals.

## Corrections and preservation

The generic `agir_rules` no longer rejects otherwise-supported office evidence
solely because its day matches a submitted event. The date veto remains in
`agir_invariants`, which pins this exact historical fixture. Three synthetic
same-day cases failed before the correction and pass afterward; exact historical
pins still reject changed observations, and substituting event-only claims still
fails the generic evidence rule. Synthetic cases are not historical acceptance.

Three claim uncertainty strings and the report now describe conservative
selection and the minutes' actual wording. Claim text, locators, dates, event
kinds, all original/extract pins and all five new holder objects are unchanged.
The report's claim of user priority is qualified as an author report, not an
instruction. Existing author check reports remain attributed to the author.

The [scope audit](scope-review.json) verifies the full prior Brazil object after
removing only declared appends: all **248 old sources, 535 claims, 32 entries,
five old roles and 51 old holder observations** remain unchanged. Accepted C01-34
and SOURCE-17 corrections remain intact. A second reviewer independently checked
those prior records/extracts and the two July scans at correction `878a48bc`, with
no further blocking issue. No other country, runtime or art file is changed.

## Checks and reproduction

Raw commands, UTC, exit codes and output hashes are in [validation/](validation/).

| Check | Result |
|---|---|
| Same-day guard before correction | 1 test, 3 expected failing subcases retained |
| Guard / exact fixture / adverse mutations after correction | 3 tests passed |
| Final focused Brazil suite | 52 passed |
| Final campaign research suite | 9 passed |
| Temporary research index generation and `--check` | Passed: 2,010 sources / 4,954 claims on this isolated base |
| `git diff --check` | Passed |

The temporary generated index was restored and is not in these commits. Run
`python -B -X utf8 verify.py` here to verify receipt payloads, reviewed Git inputs
and external files offline. Use `--external-root <directory>` for relocated
originals or `--skip-external` to explicitly omit external-byte verification.
The verifier proves integrity, not historical meaning. The preserved retrieval
script does not run as part of verification.

This does not complete Brazil's historical party census, succession eligibility,
portrait permissions, likeness approval, C01, C06, S23, WC1, CP1 or any other parent
gate. No native campaign, full avatar suite or qualification run was performed.
