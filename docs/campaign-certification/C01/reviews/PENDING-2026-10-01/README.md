# Pending-acceptance review

User request: independently check and verify the three submissions still awaiting
acceptance after integration `509bd289`: C01-28 Russia, C01-31 Komeito and C01-39
Democratic Alliance. Original failures and prior accessible-content reviews remain
part of each packet's evidence. A successful source download is followed by
claim and holder review; passing tests alone does not grant acceptance.

## Related follow-up checks

The same remote check found two follow-ups on already accepted packets:

- France C01-38 at `abc8ef4a368ae8dd9df7fe3e3f5953f1e36d7379` changes only its report
  and handoff. The same-day interval repair is already integrated. Effective
  boundaries remain unknown and the four DILA exports were independently retained
  in the prior review. No new source or holder is delivered. The further prose is
  not imported here; the independent acceptance receipt remains authoritative.
- USSR C01-41 at `8a3a041154377ea187313d48d8489bc2bf10002c` tightens two tests in
  addition to changing report/handoff prose. The two exact test changes are adopted:
  CPSU role sources must match ordered lists, and the old-holder preservation guard
  may exclude only new observations within the two CPSU roles whose citations all
  belong to that packet. Two mutation regressions expose invented extra state and
  party holders that the old broad filter hid; both fail before the repair and
  pass after it. All 48 USSR tests pass. No source, holder or date is changed.

The remote prose includes assertions about separate user rulings. Those assertions
are not used as approval evidence here. The retained independent review decisions
and their stated limits remain the basis for acceptance. The exact remote tips are
preserved; this review does not merge the remote branches wholesale or alter their
unfinished work.

The before/after regression logs are in `followup-validation/`.

## Pending packet decisions

| Packet | Decision | Evidence and remaining work |
|---|---|---|
| C01-31 Komeito | Accepted bounded research | All 31 originals, 64 claims and 15 holder observations reviewed. Seven originals recovered; three passage locators repaired. Takeya remains an attestation without an inferred start; Ota's explicit assumption is retained. [Acceptance](../../integrations/CLAUDE-C01-31/README.md), [resumed receipt](../CLAUDE-C01-31-resumed-20261001/README.md). |
| C01-28 Russia | Held | Nine more originals and 20 more claims checked: cumulative 65/68 originals, 133/137 claims and all 24 holder observations. Three originals/four claims remain held. One request returned HTTP 429 and two later URLs were not attempted. [Additive receipt](../CLAUDE-C01-28/2026-10-01-missing-only/README.md). |
| C01-39 Democratic Alliance | Held | Review of `9cb02c20` reproduced one original and checked one claim. The second request returned HTTP 429; 22 originals were not attempted. The remaining 23 originals/28 claims and all eleven then-proposed holder observations stay held. [Immutable initial receipt](../CLAUDE-C01-39/README.md). A later twelve-observation proposal is separately identified below. |

Komeito source import `b74f4fa565429b63052d94cc8ec10337b0b573ed`, boundary
correction `988d537934b4d36e2a251c4b5ca498f19dc7cf06`, locator correction
`aed12059c63545c3e6d6c5cacaa696a8db53c918` and acceptance receipt
`d4d3542b2c7750a83d27564f99cc0428536c8852` are integrated together at
`a69627e76cf0131b7b5b25d5ab7999737a919001`.

Only the additive Russia receipt `6133f79b5b00126d4733453fe42839c1ca7390c4`
(integrated as `c58ee025`) and DA receipt
`d1818a48a1b117d408dafb49d7943d35f3f7c2da` (integrated as `2d31003a`)
are imported from those isolated reviews. Their research, corrections, tests and
generated indexes remain isolated. Earlier failures and content decisions remain
unchanged. Neither archive stop supplied a Retry-After header; no further source
requests were made after the DA stop. A rate limit is an access failure, not a
finding that a historical claim is false.

The queue now records **26 completed bounded Claude tasks and two held
submissions**. Acceptance does not grant runtime mappings, organization
equivalence, portrait rights, complete country histories or country-cast signoff.
C01, C06, S23, G5 and CP1 remain open.

## DA follow-up discovered during final fetch

The final fetch found `b66f8c074431d7e1a8c9bfca228576c7d5134655`, comprising
correction `9e3d0b3c` and a separate generated-index commit. The earlier held
receipt continues to describe exactly `9cb02c20`; it is not rewritten to imply
review of later content.

The follow-up retains the same 24 original-source identities and 29 claims. It
adds a Helen Zille observation on 6 May 2007, proposes a Maimane end on
23 October 2019 using the next-day vacancy statement, changes the acceptance
claim's event kind and repairs a quotation's apostrophe. These produce twelve
proposed new holder observations. Their supporting originals remain unverified,
so all twelve are held alongside the same 23 originals and 28 claims. The one
read statement still supports the reported resignation/vacancy context; its
changed interpretation is not automatically accepted. This is diff and dependency
triage, not a fresh historical-content review or execution of the new tests.

Claims of separate user rulings in the submission do not replace independent
source review. No follow-up source, observation, date, test or generated index is
imported. The [scope audit](scope-audit/scope-audit.json) and accompanying
`newtip-triage.json` pin the differences. The queue identifies the latest
submission and the exact earlier review separately.

## Validation

The combined accepted tree passes **640 avatar/research tests, 69 planning tests
and 11 Node atlas tests: 720 tests, no failures**. Research index, census, gap
ledger, boundary matrix, cartoon inventory, workboard and whitespace checks also
pass. Exact commands, durations and log hashes are in
[the combined validation record](validation/validation-01/results.json).

After registering the newly discovered held DA tip, generation and affected
ledger/boundary/planning checks are repeated without changing the accepted
research or runtime. Their separate logs retain the chronology; they are not
added to the 720-test total. The earlier 48-test USSR run overlaps the combined
suite, as do the packet-specific review checks. No native campaign, performance,
artwork or human-playtest gate is rerun or newly awarded by this research review.

Post-copy evidence checks reproduce the Komeito receipt and all 31 retained
originals, the DA receipt and seven retained evidence files, and the Russia
receipt's 23 payloads plus 76 external files. The
[Russia verification result](russia-verification/verification-result.json) also
proves its country JSON and 170 source files are unchanged from `509bd289`.
The final scope audit verifies preservation of other country research and game
code/assets. Full source bodies remain external; compact receipts, metadata,
checksums and limited findings are committed.
