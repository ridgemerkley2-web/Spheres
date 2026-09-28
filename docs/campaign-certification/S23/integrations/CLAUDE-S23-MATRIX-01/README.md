# CLAUDE-S23-MATRIX-01 — preparation review

**Decision: accepted for integration as a bounded preparation audit, with the
corrections below.** Reviewer: Codex `/root/review_gap_submission`, 28 September
2026. S23 remains planned pending C06. This review accepts neither historical
coverage, portrait likeness, a country, nor campaign certification.

Claude's submission is `8c9f6ce1d533504143419af483e4435cde21835b` on
`origin/claude/s23-matrix-01`. Its twelve owned-file Git blobs were imported
unchanged in `cd9d50669ba4a842375163723ac60ff723e22868`, based on integration
`75626a137d5efd1ca1cca0d6f76c4d8502909bee`. The submitted handoff is retained
as [original-handoff.md](original-handoff.md); the global handoff was not edited.
[submitted-blobs.json](submitted-blobs.json) records exact original/imported
blob identities. The submitted generated coverage gaps remain recoverable from
that import commit, rather than being rewritten in its history.

The reviewed implementation and regenerated preparation outputs are pinned at
`e39dd476c758d049dd3e1cc581f9fdaf68caf88b`. No runtime, research, production
schema, art asset, or global workboard changed in these commits.

## Review corrections

- Require an explicit acceptance decision. A mention of “not accepted,” another
  packet's acceptance, or a quoted decision cannot accept a numbered packet.
  Bounded SOURCE repairs remain distinct from parent packet acceptance.
- Treat an unclaimed source as unattributed discovery intake. A missing source
  table entry does not prove S10 provenance; accepted plus unattributed evidence
  is explicitly mixed, rather than wholly accepted.
- Keep year/month/multi-day observations as possible period observations, not
  exact-day holders or continuous terms. Retain source uncertainty and notes.
  Sample their endpoints without calling those endpoints handovers.
- Include paired office handovers and attestations in executive cases, and
  exact recorded deaths of retained original executives.
- Match the served selector's exact Gregorian portrait dates. Count fictional
  portraits only where the selector actually binds, distinguish simulation
  future references from the web historical endpoint, and report a missing
  checksum on shared art instead of crashing.
- Preserve saved executive precedence and avoid legacy identity fallback when
  historical mode has no saved leadership book. Pin supplied raw/gzip save
  bytes and decoded bytes; future historical dates remain inapplicable.
- Respect known dissolution even when party kind is unknown. Flag portraits
  covering an exact death day, including open-ended portrait windows.
- Reject changed research cutoffs and duplicate research identities. Clarify
  that this is a source-derived runtime mirror, not native execution or visual
  inspection. No equivalence is asserted for unread S10 saves.

## Validation

The original 23 tests passed before changes; their original log is preserved in
[original-tests.log](original-tests.log). [original-tests-record.json](original-tests-record.json)
states the retrospective provenance limits of that initial command.

The final source passes **36 tests: the original 23 and 13 added regressions**,
plus matrix `--check`, census `--check`, and research `--check`. Exact commands,
Python identity, source pins, timings and log hashes are in
[validation/validation.json](validation/validation.json).

The added regressions were also run against the exact original tool from Git.
All thirteen exposed an issue; the retained negative-control log records
**25 failing assertions/subtests and 3 errors** across thirteen test methods.
This expected failure is separate from the original passing 23-test suite and
the reviewed passing 36-test suite. Both logs list the exact test names.

The read-only audit against actual integration
`01c68fe17b03391edc18b244932c3b56080d1f26` produced all ten output files
byte-identical to the reviewed checkout. Only the reviewed tool's own source
bytes were overlaid for self-hashing because it had not been integrated there;
all data, research, Rust, portrait and acceptance inputs came from the integration
directory and were rehashed unchanged afterward. Exact compressed outputs,
decoded hashes and method are in
[validation/current-integration-audit.json](validation/current-integration-audit.json).

Actual immutable saves dated **1999-05-13** and **2035-11-30** were read without
native loading or replay. Their original and decoded hashes are pinned in the
two `actual-campaign-*.json` reports and they remained byte-identical afterward.
The 1999 report retains twelve divergent party incumbencies as expected gameplay
outcomes. The 2035 report does not invent a historical comparison after the
cutoff. No save bodies were duplicated into this packet.

## Findings retained

The corrected matrix contains **7,095 cases over 115 roles**, covering eight CP1
cases/nine identities and annual samples from 1990 through 2035. The historical
cutoff remains 2026-09-07; authored fiction begins 2026-09-08 and ends exclusively
on 2036-01-01. All 26 inspected bound/referenced assets are available, which is
an availability result, not a likeness review.

Only the existing six numbered Tonga packet acceptances remain accepted.
The other numbered research packets stay pending. France still lacks an office
research chain; USSR/Russia and many minor-party intervals remain incomplete.
Correctly separating period evidence reduces Tonga's exact-date yearly research
identification from 23 to **15/555**, with eight period-only observations.
Only four of 192 fictional candidates have served portraits. Research-to-runtime
identity reconciliation and C06 art acceptance remain open. Passing the checker
does not turn any of these gaps into a coverage pass.

## Regeneration after integration

After combining all accepted submissions, run from the combined repository:

```text
py -X utf8 tools/avatars/certified_boundary_matrix.py
py -X utf8 tools/avatars/certified_boundary_matrix.py --check
py -X utf8 -m unittest discover -s tools/avatars -p test_certified_boundary_matrix.py -v
```

New acceptance-record directories can legitimately change the pinned summary
and README. Review those changes; do not promote their numbered parent packet
unless its own explicit acceptance decision exists. Regenerate the current
preparation output, never this immutable review evidence.

`review.py` can reproduce the review in a **new** `--output` directory using
`--root`, optional read-only `--integration`, and repeatable `--campaign` paths.
The negative control reads the original submission from Git, so that object
must remain reachable. Decompress any `validation/current-integration/*.gz`
with standard gzip; compare decoded byte counts and SHA-256 to the audit record.
