# C01-46 independent State Duma faction-head review

**Decision: accepted as bounded research intake, with two corrections.**

Independently retrieved and read **7/7 original archived Duma articles**, all
**27/27 submitted claims** and **18/18 appended holder observations**. No source,
claim or holder remains held in this packet. This acceptance is a research review;
the coordinating integrator still owns import and task registration.

The exact submission is `64fec76b8a61e7f65f0f98f7bf3a93699ea80d72`, including
substantive commit `8a6cbe0c5f8c48b1e80b8a1da6f6c475a68d20dd` and claim
`3ee3357f55a26a4587a62e56c0cc4c62110e54df`. Review base:
`229df2105f1ca46bc55873ea4ab0ada63255ced3`. First-source authored import:
`91a1dbe1` (full identity in [scope-review.json](scope-review.json)). Independent
correction: `c1056321ec2330b7acb27510a5f7c1e3b4043d53`.

## What the originals establish

These are State Duma website articles preserved by the Internet Archive, not
Archive-authored government records. On 1 October 2026, one coordinated sequential
pass retrieved all seven exact submitted URLs with at least ten seconds between
requests and no retries, alternate hosts, browser impersonation or credentials.
Every request returned HTTP 200 and matched the submitted raw-body identity. The
last response is gzip as served; its decoded identity also matches. Headers, curl
metadata, original bytes, decoded HTML and text aids remain outside Git. The
first pass ran from 03:52:07 through 03:53:15 UTC; exact times and commands are in
[source-attempts.json](source-attempts.json). No rate-limit response occurred.

Hash equality did not establish acceptance. The reviewer separately read all
seven complete article bodies, their datelines, headline/subtitle context and
relevant captions. HTML inspection distinguished embedded hover biographies from
the printed article names. There are no PDFs or facsimiles in this packet.
[Source decisions](source-review.json), [all claim decisions](claim-review.json)
and [all holder decisions](holder-review.json) retain the individual outcomes.

- News 52394 reports the October faction elections and selections, with a
  future-tense subtitle. Only Vasilyev's election states its own day. These five
  claims remain separate events; no holder commencement is inferred.
- News 53098 explicitly names four faction heads in a dated sitting report.
  Delyagin speaks for SRZP but is not styled faction head. The report does not
  establish that Mironov never spoke; it simply supplies no Mironov attestation.
- News 53987 reports Zhirinovsky's death but gives no death day beyond its
  dateline. It remains a death-report claim, not boundary evidence by itself.
- News 53988 explicitly identifies the deceased as LDPR faction head in the
  subtitle and quotes the faction saying he died that day. Its 6 April 2022
  dateline resolves that relative day. This supports the submitted `until`
  **2022-04-06** on the new December observation, without inferring it from
  Slutsky's succession. The quotation's party-chair title creates no party role.
  The four other heads have directly printed faction-office titles.
- News 54314 states Slutsky's election on 18 May and earlier acting service,
  without a separately stated assumption day or acting interval. The two claims
  stay claims-only; no start is fabricated. This bounded decision does not ban
  future independently supported office evidence on the same calendar day.
- News 54910 and 63980 directly name all five faction heads. Their datelines
  support observations, not complete tenures or separately stated meeting days.
  Committee titles and party affiliation do not substitute for faction office.

Repeated observations are evidence points, not separate inferred tenures. The
five original October 2021 holders remain byte-for-field unchanged. All 18 new
observations retain null `from`; only the directly supported death end is set.
No latest attestation or successor event supplies an end. No post-cutoff record,
faction lifecycle, government-party mapping or party leadership is inferred.

## Corrections and preservation

1. The dated death quotation is in **article paragraph 2**, after a paragraph
   describing colleagues' reaction. Both the claim and factual extract said
   paragraph 1. Corrected the two locators and repinned that extract. The new
   regression failed against the submitted locator and passed after correction;
   original response identities, claim text, event kind and dates are unchanged.
2. Narrowed the report's unsupported statement that Mironov did not speak to the
   supported fact that the December item provides no Mironov attestation.

[Scope audit](scope-review.json) compares the complete country data to the base:
all 170 old sources, 295 old claims, 14 organizations, original faction holders
and other Russia fields are preserved after removing the declared appends.
USSR data and accepted SOURCE-26/CPSU corrections remain unchanged. Russia
C01-28 party research stays held and is not imported. The authored pin updates
retain the old USSR C26 holder hash and separately pin the 18 new observations.
Author assertions about user priorities or precedent rulings were not treated as
user instructions.

## Checks and reproducibility

Raw commands, UTC, exit codes and hashed output are in [validation/](validation/).
All checks ran on this isolated reviewed tree; none launched a native campaign.

| Check | Result |
|---|---|
| Locator regression before correction | 1 expected failure retained |
| Locator regression after correction | 1 passed |
| `test_russia*.py` | 37 passed, including the existing 23 adverse packet mutations |
| `test_ussr*.py` | 48 passed, including the accepted prior-holder guards |
| `test_campaign_research.py` | 9 passed |
| Research index regeneration and `--check` | Passed: 1,978 sources / 4,903 claims on this review base |
| `git diff --check` | Passed |

The generated index was used only for the check and restored; it is not committed
in this branch. The integrator must regenerate the combined index and related
central records. No full avatar suite, native tests, historical matrix or campaign
qualification was run by this review.

Run `python -B -X utf8 verify.py` in this directory to rehash all receipt payloads,
the reviewed Git inputs at the correction commit and the 42 external source files.
For relocated originals pass `--external-root <directory>`; the default is
`D:/spheres-offload/codex-next-20260928/c01-46-review-evidence-20261001`.
`--skip-external` explicitly verifies only the available repository evidence.
The verifier is offline and proves integrity, not source content. The retrieval
and check-driver copies preserve the executed review procedure; they include the
original worktree paths and do not automatically rerun when verifying.

## Limits

This packet adds research observations for six people in five existing faction
roles. It does not prove continuous 2021–2026 office histories, exhaustive parties,
identity reconciliation, portrait permission, licensing or likeness approval.
No original article, photograph or symbol is republished. No runtime, art,
country-cast, C01, C06, S23, CP1 or other parent gate is completed by this receipt.
Older source-access holds remain separate.
