# Claude push verification — 27 September 2026

## Result

All 17 pending, completed C01 submissions passed the seven applicable technical checks at their exact published heads: **119/119 command runs passed**. Four newer packets contain claims/plans only, so they are not completed deliverables. A second fetch at the end found no head changes.

No game-runtime changes were present in the reviewed branch deltas. The integration branch remains at `ffe54b028ac6343f7575f4b06ed52d845bf72297`; no submissions were merged, no acceptance gates were closed, and no original campaign save or server was changed. This report and its evidence are local review artifacts, not a GitHub push.

## Exact submissions

All rows below additionally passed research-index check, 79 research-pattern tests, 16 campaign-pattern tests, 11 Node atlas tests, the 44-marker workboard check, and branch-diff whitespace check. The packet-pattern column is the actual count run at that head, including inherited packet tests; these counts are not additive unique coverage. Campaign tests now include the seven census tests that Claude could not run in sparse checkouts.

| Packet | Published head | Scope | Packet-pattern tests | Result |
|---|---|---|---:|---|
| C01-05 | `1c698ed00cd3989e6c7dfc2c676fc7185c7b4c84` | The 1991 USSR/RSFSR executive transition | 27 | PASS |
| C01-06 | `7948ab987efeaadba2c9dca67d8ccec03db430ca` | Saudi Arabia's executive chronology 1990-2026 | 27 | PASS |
| C01-09 | `82a23f9da8660839731c1aa02a1e9b9baf8614e2` | South African heads of state, 1990–2024 | 68 | PASS |
| C01-10 | `73e5fd363769e589a09615e957b14c86f8756513` | Brazilian presidents, 1990–2026 | 68 | PASS |
| C01-11 | `538920f14c5784e16d910dc2df22766ccd72f59c` | Indian prime ministers, 1990–2026 | 69 | PASS |
| C01-12 | `da358dbff9ae536a6b83a012d36f98f0dd639e49` | Japanese prime ministers, 1990–2006 | 69 | PASS |
| C01-13 | `42f98dc8b3d2aae748cabd7f234e0ef7d1575eee` | Japanese prime ministers, 2006–2026 | 78 | PASS |
| C01-14 | `1d749e1520e3ee6a8b971e36b5027c723249b918` | Russian presidents, 1991–2026 | 77 | PASS |
| C01-15 | `dd58a610859f997a1ad2452012b464d305b7ba91` | Indian presidents, 1990–2026 | 78 | PASS |
| C01-16 | `2df4a0a6854a14796515d846d0d504edd785a83e` | ANC presidents, 1990–2026 | 76 | PASS |
| C01-17 | `7d71acef98e7a94781f48054b4574312bd9e7470` | Brazilian vice-presidents, 1990–2026 | 76 | PASS |
| C01-18 | `2120b697a1c51efc680d3747912115efede183af` | Liberal Democratic Party presidents, 1990–2009 | 87 | PASS |
| C01-19 | `6f4ef2c27c5c4293bb324ee4c8ad359fd155ad66` | Russian heads of government, 1991–2026 | 87 | PASS |
| C01-20 | `2ee3b14689e445486554604b69ff82dc1f9db9df` | Indian National Congress presidents, 1990–2026 | 88 | PASS |
| C01-21 | `32ea29336e204eafb52e68485e97c8844a96cd6b` | South African deputy presidents, 1994–2026 | 84 | PASS |
| C01-22 | `39bb64395273ea3cb7eef0a3fd1b10068a787002` | Workers' Party (PT) national presidents, 1990–2026 | 84 | PASS |
| C01-26 | `09f41dd4bbe05cec693e619eedd1740aec897d54` | Soviet heads of government and Supreme Soviet chairs, 1990–1991 | 96 | PASS |

Claim-only packets: C01-23 (French presidents), C01-24 (Tongan Speakers), C01-25 (Saudi councils), and C01-27 (BJP presidents). Their inherited parent changes do not constitute new completed work. Exact hashes and handoffs are in `inventory.json` and the saved handoff files.

Already integrated submissions C01-01/02/03/04/07/08 and S19 were classified as existing baseline work, not new deliveries. `claude/c01-saudi-03` is the old branch name for C01-06, not another packet.

## Combined integration rehearsal

In a dedicated detached worktree, assembled the changed paths from latest completed chain tips C01-06, C01-18, C01-20, C01-21, C01-22 and C01-26 onto integration. This was a content assembly for verification, not a merge into the active playset. Compared overlapping paths by Git blob identity: no conflicting path contents outside the shared generated research index. Regenerated that index in the scratch worktree only.

All eight combined commands passed: index generation and check; 79 research-pattern, 207 packet-pattern, 16 campaign-pattern and 11 Node tests; workboard and whitespace checks. Suite patterns overlap, so their sum is not a distinct-test count. The resulting index contains 1,329 sources and 3,731 claims; these are authored records, not a count of independently verified historical facts.

Recommended dependency order if acceptance is later granted:

- C01-05 → C01-14 → C01-19 → C01-26 (USSR/Russia).
- C01-06 (Saudi Arabia).
- C01-09 → C01-16 → C01-21 (South Africa).
- C01-10 → C01-17 → C01-22 (Brazil).
- C01-11 → C01-15 → C01-20 (India).
- C01-12 → C01-13 → C01-18 (Japan).

Regenerate `research-index.json` after combining countries; do not choose one branch's index as the final index.

## Source integrity and historical acceptance

An independent Git-byte audit found no local snapshot byte-count or SHA-256 mismatches. Its 1,369 source-file observations include repeated/inherited entries and a claim-only branch's inherited baseline; they are not 1,369 distinct new sources. The combined completed-submission tree has **1,169 distinct changed factual-extract files**, of which **23 do not record an original-response checksum**. Missing original-response identities are disclosed provenance limitations, not proof of invented facts. Local extract checksums do not prove that their paraphrases are accurate.

Fetched one checksum-bearing source response per completed submission (deterministically the first eligible changed extract). **14 of 17 matched both the recorded byte count and SHA-256; two returned different bytes and one timed out.** This is a small identity sample, not comprehensive source-content review or a statistically representative sample. The additional C01-25 inherited-source probe in the raw log is excluded from these 17 results.

Unverified live samples:

- C01-05: unavailable_not_verified; https://projects.rusarchives.ru/statehood/09-37-postanovlenie-vybory-prezident.shtml. Error: <urlopen error timed out>
- C01-06: changed_response_not_verified; https://www.bush41library.gov/digital-research-room/finding-aid/public-papers/address-nation-announcing-deployment-united-states. Recorded 31889 bytes, fetched 32782 bytes.
- C01-17: changed_response_not_verified; https://legis.senado.leg.br/diarios/BuscaPaginasDiario?codDiario=111711&download=true. Recorded 24950218 bytes, fetched 24950158 bytes.

Changed response bytes may reflect dynamic pages or document generation; they are not themselves evidence that the historical claim is wrong. These responses need content-level comparison or an independently verified archived record before acceptance. No comprehensive independent reread of all dated claims, source facsimiles, or likenesses was performed. The 7 September 2026 research cutoff and incomplete worldwide-coverage gates remain intact. C01-26 also broadens an earlier host exclusion to permit scanned Vedomosti PDFs while excluding HTML transcriptions; that source-policy change is explicitly documented and still needs acceptance review rather than being legitimized by its passing tests alone.

## Reproduction and evidence

`results.json` pins every head, exact command, exit status, duration and raw log checksum. `combined-inputs.json` identifies every assembled file by Git blob and records no path collisions. `combined-results.json`, `source-audit.json`, `combined-source-summary.json` and `source-identity-sample.json` preserve the details. All branch checks ran in clean detached snapshots with real `spheres-sim` source/data and `spheres-web/data` available. No full Rust runtime or visual browser suite was rerun for these research-only changes; atlas behavior received its targeted Node checks. Passing these checks does not change the previously recorded political A1 campaign-certification limitation.

The scratch combined tree remains at `work/campaign-certification/review-claude-20260927` for inspection, with uncommitted review-only assembly changes. The active integration checkout remains clean. The first assembly attempt exceeded Windows' command-line length; the review harness was corrected to restore paths in batches, and the complete rerun passed. No product change was needed.
