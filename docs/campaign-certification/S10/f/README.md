# S10.f — Understand campaign succession

**S10.f complete. S10 and C01 remain in progress; G2 is unearned and S11 has not started.**

[Play the updated review build](http://127.0.0.1:7852/), then open **Government → Party leadership**.

Campaign leadership cards now explain a person's saved party office, current
candidate availability and eligibility for national office separately. When the
campaign records a death or a term-limit exclusion, the card shows the exact
saved date. A national-office term limit does not erase the person's separate
party leadership. Being eligible does not automatically appoint a leader.

Historical reference browsing now has an explicit independent context, even when
the selected reference date equals the campaign date. Campaign deaths can no
longer remove sourced historical candidates from that reference. Historical,
uncertain and fictional-template previews cannot display saved campaign
restrictions. Existing save formats and succession rules remain unchanged.
At narrow widths, portraits appear above the text so the status explanations
have the full card width.

## Bounded Brazil discovery

[Brazil's research packet](../../C01/research/brazil.json) adds 31 organization
observations supported by five official sources: 29 party labels in the 2024
election-funding table, a separate Missão registration decision and a separate
PMB/Democrata naming record. These are not 31 distinct parties or leaders.
Reported funding-release dates, unknown cells and later naming corrections
remain explicit. The live historical funding table is not presented as an
archived party registry at the research cutoff.

The discovery index now holds six country packets, 773 organization observations,
18 institution observations, 45 sources, 1,460 claims and 83 discovery batches.
No exhaustive country census, new leader mapping or portrait permission was
established. The historical cutoff remains 7 September 2026; access dates are
recorded separately.

## Verification

- Windows and Linux native web suites: **378 passed, 0 failed, 14 ignored on each**.
- Windows and Linux leadership suites: **49 passed on each**.
- Windows and Linux agency/succession integration: **12 passed on each**.
- Windows interface suite: **1,548 passed, 0 failed, 1 skipped**.
- Content: **57 tests** and five reproduction/allowlist checks passed in 11 commands.
- Two disclosed UK fixtures passed the actual campaign and historical-reference
  interface at 1440px, 390px and 320px, then named Save, cancelled Load, Load and
  Continue. Their 12 archive inspections and 10 exact native comparisons ignored
  no world fields. The complete saved event log and history also remained equal.
- The ordinary France journey separately passed second-tab staleness, refusal
  without mutation, a fresh government confirmation and Save/Load/Continue.
- Automated text visibility and hit tests were supplemented by manual inspection
  of the actual desktop and narrow-screen screenshots.

The UK fixtures deliberately invoke native succession events: a counterfactual
campaign death on 1 January 1990, and a fictional succession followed by a term
limit on 2 January 2027. The latter calendar is positioned as test setup. These
are not historical death dates, predicted elections or 37 years of simulated
play. **Both browser journeys advance zero days.** Full Linux interface tests,
the eight-country startup matrix, external old-save matrix, long-campaign
behavior, performance and Russia activation were not newly qualified here.

## Source and evidence

All game code and served assets use runtime `7d85b9f76a3e787b257613442fb0a3370e90f4c2`.
Executable SHA-256: `b99f84450805a518055eff0b67066ce800cbd30096eeeaf0d96d10b1ba9e118b`.

The authored browser driver has a separate recorded revision in the
[manifest](manifest.json). Its only source change after the runtime pin corrects
a false clipping assertion: visible serif glyph bounds may extend outside their
CSS line box. The final check tests the viewport, actual clipping boundaries and
hit points. The original failed browser run is development evidence, not a
passing qualification. No game source changed after the runtime pin.

The manifest binds [151 selected evidence files](evidence/inventory.json),
98,329,370 stored bytes. Large archives reconstruct exactly; compiled
binaries are identified by hash. All eight protected original files and both
source worktrees passed preservation checks. Previous review servers were left
in place. The new review server uses its own verified copy of the ordinary
France campaign. Execution stopped after S10.f.
