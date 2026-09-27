# Claude research integration — 27 September 2026

Merged C01-05/06/09–22/26 into `codex/campaign-certification` on the user's explicit instruction. All 17 reviewed submission heads are now ancestors of integration. Research is integrated; historical acceptance and C01 completion remain pending. Claim-only C01-23/24/25/27 were not merged.

Six merge commits retain the country-chain history. Shared research-index conflicts were resolved by regeneration from all integrated packets, never by dropping another country's records. No other merge conflicts occurred. Every nongenerated incoming file matches the blob from the previously tested combined snapshot.

## Validation

Merged research head: `af49b6e579eea5ece8322c9b77b1115a7c44f831`.

All eight final checks passed: index generation/check, 79 research-pattern tests, 207 packet-pattern tests, 16 campaign-pattern tests (including the previously skipped census suite), 11 atlas Node tests, the 44-marker workboard check, and whitespace validation. Patterns overlap; counts are not unique test totals. No gameplay source or saved campaign was changed. Runtime/browser suites were not rerun for research-only changes. Existing campaign-certification limitations remain.

## Source-review limits

The premerge audit passed all 119 branch check runs. Local source-extract identities had no mismatches. Of 17 live response spot-checks, 14 matched, two changed and one timed out; these require historical content review rather than automatic acceptance. The combined tree contains 1,169 distinct changed factual extracts, with 23 lacking original-response checksums. This does not certify all dates, people, roles or likenesses. C01-26's allowance for scanned Vedomosti PDFs also remains part of the pending source-policy review. The historical cutoff remains 7 September 2026.

The linked premerge report describes the earlier verification-only state; this integration record supersedes its statement that nothing was merged. See [raw evidence](evidence/2026-09-27-claude-integration/merged-results.json), [premerge report](evidence/2026-09-27-claude-integration/premerge-REVIEW.md), and [source sample](evidence/2026-09-27-claude-integration/premerge-source-identity-sample.json).

## Integrated submissions

| Packet | Reviewed head |
|---|---|
| C01-05 | `1c698ed00cd3989e6c7dfc2c676fc7185c7b4c84` |
| C01-06 | `7948ab987efeaadba2c9dca67d8ccec03db430ca` |
| C01-09 | `82a23f9da8660839731c1aa02a1e9b9baf8614e2` |
| C01-10 | `73e5fd363769e589a09615e957b14c86f8756513` |
| C01-11 | `538920f14c5784e16d910dc2df22766ccd72f59c` |
| C01-12 | `da358dbff9ae536a6b83a012d36f98f0dd639e49` |
| C01-13 | `42f98dc8b3d2aae748cabd7f234e0ef7d1575eee` |
| C01-14 | `1d749e1520e3ee6a8b971e36b5027c723249b918` |
| C01-15 | `dd58a610859f997a1ad2452012b464d305b7ba91` |
| C01-16 | `2df4a0a6854a14796515d846d0d504edd785a83e` |
| C01-17 | `7d71acef98e7a94781f48054b4574312bd9e7470` |
| C01-18 | `2120b697a1c51efc680d3747912115efede183af` |
| C01-19 | `6f4ef2c27c5c4293bb324ee4c8ad359fd155ad66` |
| C01-20 | `2ee3b14689e445486554604b69ff82dc1f9db9df` |
| C01-21 | `32ea29336e204eafb52e68485e97c8844a96cd6b` |
| C01-22 | `39bb64395273ea3cb7eef0a3fd1b10068a787002` |
| C01-26 | `09f41dd4bbe05cec693e619eedd1740aec897d54` |
