# CLAUDE-C01-29 — independent review in progress

Reviewed 28 September 2026 UTC. **Not merged or accepted.** Parent C01, C06,
S23 and CP1 remain open. The historical cutoff remains 7 September 2026.

The submitted Japanese Socialist / Social Democratic Party chair packet passed
the technical review. Original-source review is incomplete because 19 recorded
Internet Archive responses could not be retrieved. Keep the task
`ready_for_review`; preserve its submission and resume this review, rather than
repeating the completed technical work or commissioning duplicate research.

## Exact revisions and scope

- Original submission tested: `e0bb7a01930b08a9b8b6d3b78caf3fc93af199d7`.
- Latest inspected submission: `3b30478562ec78c2392d0472061e773e23cb25dd`.
  Its only changes are six documentation replacements: clarifying the packet
  author's internal research integration and correcting the handoff's Diet
  response count from 22 to 15. Sources, extracts, claims and tests are identical
  to the tested revision.
- Integration baseline for this review: `b949f64ea5ba371e037b8ebb234e4f41c66cabbf`.
- Research-only additions: 54 sources, 111 claims, one party role, sixteen
  holder observations of seven people. No runtime data, portraits or gameplay
  changes are proposed or installed.

The recursive source comparison preserves all existing Japan values and limits
changes to the six declared appended ranges. Earlier Japan tests retain their
exact prior sets and counts and explicitly add guards for the new records;
shared research generators are unchanged. See
[isolation receipt](evidence/jp29-technical-isolation.log).

## Technical validation

| Check | Independent result |
|---|---|
| Japan Python tests | 44 passed |
| Research Python tests | 79 passed |
| Campaign Python tests | 16 passed |
| Leadership research atlas Node tests | 11 passed |
| Research-index exact regeneration check | Passed |
| Campaign census check | Passed |
| Workboard validation | Passed |
| Submitted diff whitespace check | Passed |

These suites recorded 150 passing test executions; discovery patterns overlap,
so this is not a count of unique cases. Original logs are retained
under [evidence](evidence/). The first campaign/census attempts failed because
the disposable sparse review checkout lacked their simulation/web input files;
after hydrating those existing tracked inputs, both checks passed. The failed
attempts remain alongside the `-02` passing logs. No submitted file was changed
to make those checks pass.

The initial broad review checkout also exhausted the available disk while
copying historical evidence. Only that newly created, disposable review
worktree was removed; the successful narrow checkout preserved the integration
tree, campaign saves, prior evidence and all existing worktrees.

## Original-response verification

[retrieval.json](evidence/retrieval.json) records every attempted URL, timestamp,
expected identity, returned identity or connection error. The independent
[retrieval script](evidence/jp29_source_retrieval.py) requested identity encoding,
without automatic response decompression, and compared both byte count and
SHA-256 against the submitted extracts.

- **35 of 54 responses reproduced exactly:** all 15 Diet API responses and
  20 archived party responses.
- **19 archived party responses remain unverified:** connection refused
  (`WinError 10061`). A targeted repeat of the first failed URL also refused the
  connection. The errors do not establish that the historical claims are false.
- An independent browser-text check of the party's live
  [8 April 2026 press-conference report](https://sdp.or.jp/sdp-paper/toushusen-6/)
  supports Fukushima's in-office attestation on that day. It does not reproduce
  the missing archived response identity. The live July 2026 continuation page
  and the archived April page were unavailable through that browser-text tool.

Downloaded response bodies remain local review inputs; no original HTML,
photographs or Diet speech texts are republished in this evidence folder.
The `body` fields in the receipt refer to that local retrieval directory, not
to checked-in attachments. Byte equality alone is not historical acceptance;
claim-level content review is a separate step.

## Content review and bounded repair

The [content review](evidence/content-review.md) and
[source-by-source record](evidence/content-review.json) independently read eleven
retrieved original bodies and corroborated selected claims on five live party
pages. Checked 1990/1991/1993/1994 party-office attestations, the 2003 assumption,
the 2013 resignation and Yoshida attestation, and the retrospective rename claim
are supported within their stated scope. Live 2023/2026 corroboration remains
separate from archived-body verification; the remaining claims are not accepted
by implication.

One finding, **JP29-CONTENT-01**, concerns
`jp_sdp_fukushima_tenure_record_2003_2013`: the English wording can imply five
additional re-elections. The original reports five consecutive uncontested wins,
while the contemporaneous 2009/2012 reports call those her fourth/fifth wins.
Neutral election-win wording and an explicit ambiguity note are needed; no
holder date changes follow from this correction. This is not a finding that the
original source itself has been proved false.

The tested correction is commit
[`c98d1d5668870e184c320a3e03f477bd7f89a12c`](https://github.com/ridgemerkley2-web/Spheres/commit/c98d1d5668870e184c320a3e03f477bd7f89a12c)
on `codex/review-c01-jp29`, directly above Claude's `3b304785`. It updates the
claim/extract mirrors, their local snapshot/index identities and report wording,
with a regression that rejects the original paraphrase. **45 Japan and 79
research test executions pass**, along with exact index and diff checks. The
initial failing regression and stale-index check are retained beside the passing
logs. Holder dates and original-response URLs/hashes are unchanged. This review
branch is published for the next reviewer; it is not integrated or accepted.

The central queue now records Japan as submitted, while Russia remains claimed.
The regenerated gap ledger preserves both exclusions and its unaccepted state.
Its original test still expected both claims to be `claimed`; that failure is
retained. Updating the exact expected Japan state to `ready_for_review` preserves
the separate Russia and non-acceptance guards; **26 gap-ledger tests pass** and
the regenerated output check passes. This changes no research coverage totals.

## Resume conditions

1. Retry the 19 entries with `exact: false` in the retained receipt; retain
   subsequent attempts separately. If originals remain unavailable, document
   a supported alternative and its limits without replacing the old identity.
2. Finish claim-to-source content review, particularly the 2018/2020 leadership
   observations, the 2022/2023 re-elections and the 2026 contest/continuations.
   Preserve uncertain tenure dates and acting-role/organization distinctions.
3. If accepted, merge the exact reviewed submission including the tested
   `c98d1d56` wording correction into the then-current
   integration branch, regenerate affected S23 boundary and gap reports, and
   run their checks plus the relevant combined research checks before pushing.

No merge, country-wide history acceptance, art approval or campaign qualification
was granted by this review checkpoint. Russia's separate active claim remains
untouched at `2c4d5bd725b84161fd742adec75febc6241b8c92`.
