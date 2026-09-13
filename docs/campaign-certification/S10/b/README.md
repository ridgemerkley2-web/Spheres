# S10.b - Source-backed discovery and country government checks

**S10.b complete; S10, C01 and G2 remain open.** This increment adds research
intake code, sourced discovery packets and qualification coverage. It does not
change the simulation, playable party roster, portraits or saved campaign data.
S11 has not started.

## Research delivered

The [C01 discovery intake](../../C01/research/README.md) contains 584 organization
observations and 11 institution observations for France, Tonga and Saudi Arabia.
Its 615 claims cite 26 primary sources. The generated index assigns every exact
identity once to 61 batches of at most ten, while leaving unrepresented totals
unknown. These are research observations, not 595 verified political parties or
complete leader histories.

The France importer pins the original official CNCCFP release and checks its
checksum. The validator checks source/claim ownership, country and game-row
references, historical dates, role separation and unresolved coverage. It cannot
establish source truth or exhaustiveness. The cutoff stays 7 September 2026;
later research access does not extend a historical term or fictional eligibility.

## Government journeys verified

All eight independent fresh campaigns passed on the unchanged S10 runtime. Each
used ordinary menus to inspect its own government and a foreign government,
browse the year-2000 leadership reference, review and cancel a legal decision,
confirm it once, then use named Save, Load and Continue.

Whole native-world comparisons include dates, saved incumbents, finances, RNG
and all other serialized fields. Viewing/cancelling remained pure; confirmed
costs and every quoted effect matched the native outcome. Each confirmed action
produced a dated result. The matrix advanced **zero campaign days** and granted
no resources. It is startup coverage, not a 1990-2035 campaign qualification or
a new Russia-activation test.

| Country | Confirmed native decision | PC spent | Public payment, $ million |
| --- | --- | ---: | ---: |
| France | Invite French Communist Party | 17.1386 | 0.000 |
| Japan | Invite Komeito | 17.9196 | 0.000 |
| India | Invite Janata Dal | 15.7723 | 0.000 |
| Brazil | Invite Democratic Labour Party | 17.1386 | 0.000 |
| South Africa | Secure the South African Defence Force | 14.0000 | 1008.000 |
| Tonga | Secure the thirty-three hereditary nobles | 14.0000 | 0.912 |
| Saudi Arabia | Secure the Al Saud family council | 14.0000 | 936.000 |
| Soviet Union | Secure the Soviet Army | 14.0000 | 12800.000 |

Each case includes desktop and 390px screenshots. The first driver attempt's
four electoral cases passed; four institutional cases completed their commands
but failed a checker assumption about the electoral standing-description text.
The corrected checker reads institutional standing from the saved pillar values
and retains every effect assertion. A second attempt exposed its own JSON-copy
conversion of negative zero to zero when comparing the resumed board; direct
Playwright value transport now preserves that value. Native whole-save equality
already passed in that attempt. Both original failures are preserved alongside
the successful rerun; no runtime fix or relaxed threshold was needed.

## Qualification and provenance

- Linux: 1625 native tests passed, 0 failed,
  80 explicitly ignored, across 66
  completed targets; the full Node suite and three external archive checks passed.
- All 16 census/intake tests passed on Windows and Linux. Original census,
  pinned France import and research index reproduce exactly on both platforms.
- Windows runtime tests are the existing [S10.a proof](../manifest.json).
  The entire runtime source tree remains byte-equivalent after Git newline
  normalization; the served browser assets also matched checkout bytes exactly.
- Protected original saves, fixture binaries and source worktrees retained their
  recorded hashes. Matrix servers were disposable and stopped after their cases.

Runtime: `7aaa4517fbcc1c256481c2eefc5ef5e1c4153303`. Research and Linux source: `4643bd97d01864bf6e28c29592088e2daa735052`.
Final browser driver: `871c7eb3fffd76cfa6c077bd22cd365100a6c478`.
The [manifest](manifest.json) and [evidence inventory](evidence/inventory.json)
bind the precise logs, native snapshots, screenshots and source revisions.

## Remaining work

Complete the country organization censuses, reconcile jurisdiction and game
identities, and source distinct party/legislative/executive histories before new
characters or succession rules are accepted. The eight startup checks do not
establish complete historical leadership or succession coverage. Longer campaign qualification remains part of the broader pathway.

Visual follow-up remains: narrow review tables require horizontal scrolling to
see After values, some captures show sticky tabs/toasts overlapping scrolled
content, and Tonga still has a portrait placeholder. These findings are recorded,
not counted as finished artwork or polished narrow-layout acceptance.
