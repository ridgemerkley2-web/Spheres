# Claude — campaign leader art and country selection

User direction, 30 September 2026:
> Take out country selector figures totally and replace it with the existing campaign leader for the time.

This replaces the earlier selection of deceased national representatives for
country cards. Do not create or reinstate timeless national mascots. The old
selector figure files and receipts are archival provenance, not active character
assignments. Their presence in Git never makes them a valid fallback portrait.

## Which person to draw

- A new-campaign country card uses that campaign's opening date and the same
  primary leader that the game resolves for its government. An existing campaign
  uses its current saved office holder and current date. Follow the authoritative
  game identity; do not substitute whoever historically held office after the
  player's campaign has diverged.
- Match an exact authored `person_id`, not a country name, spelling similarity,
  cultural association or image filename. Keep person identity, office tenure,
  party leadership and a portrait's appearance interval separate.
- Party cards still need their own dated leaders, including small parties. A
  head of government portrait must not stand in for every party in that country.
  The intended coverage remains 1990–2035 across the game's country/party roster;
  current gaps must stay visible until researched identities and art are reviewed.
- Use that person's reviewed cartoon only when its appearance interval covers
  the displayed date. A missing identity or portrait gets an explicit named or
  unavailable fallback; never borrow a national representative or another person.
  Reuse existing reviewed portraits for covered dates; claim missing
  person/appearance windows before generating additional art.
- The existing project research cutoff is 7 September 2026. Later candidates
  are explicitly fictional gameplay successors, not asserted future office
  holders. Do not extend the historical cutoff without a separate sourced update.

## Art requirements

Use the approved fixed 2D person-cartoon anchor:
`spheres-web/ui/person-portraits/margaret-thatcher-cartoon-1990-v3.png`.
Retain bold outlines, clear simplified faces, full-body composition, readable
small-card silhouettes and a quiet opaque dark teal background. The previous
Lincoln-based national-selector rendering is no longer the production target.

Each delivered portrait needs the exact person ID, reviewed depiction interval,
dated likeness reference, source/creator/license/credit, retained reference bytes,
exact generation prompt, physical PNG, SHA-256 and dimensions, and actual
identity/likeness/era/visual review. Record artistic age, color or standing-body
interpretation honestly. Label the reviewer; never imply that an automated check
or Codex review is human approval. Follow `tools/avatars/person_art_pipeline.py`.

Codex has produced the five opening portraits for
`francois_mitterrand`, `jose_sarney`, `v_p_singh`,
`fahd_bin_abdulaziz_al_saud` and `mikhail_gorbachev`, and replaced the selector's
old figure resolver. Do not duplicate these files or edit that implementation
without coordination. See [the bounded handoff](CODEX-C03-OPENING-01.md).

## Claude's current work

The [CP1 acceleration assignment](CP1-ACCELERATION.md) makes Tonga the next
complete country cast. `CLAUDE-C06-TONGA-01` is ready to claim after the accepted
Codex identity proposal: reuse King IV, source the two existing 1990 identities,
and resolve the five later identity checks before completing Tonga's cast. This is an
assignment, not a claim that Claude has begun production. Keep current C01-44
party research isolated and preserve its ownership. Do not let the Russia/DA
archive holds prevent work on a ready Tonga batch.

Updated 1 October 2026 against the live remote tips and completed independent
reviews. The central queue records all four new packets. It contains 26
completed bounded Claude tasks and two submissions still awaiting acceptance.
These task closures are research/preparation results, not completed country casts.

| Packet | Scope | Current state / next action |
|---|---|---|
| C01-28 | Five Russian party-leader chains | **Held after resumed review.** 65/68 originals, 133/137 claims and all 24 holder observations are materially checked. Three originals / four claims remain unresolved; there are no remaining holder-row dependencies. [Review](../../campaign-certification/C01/reviews/CLAUDE-C01-28/2026-10-01-missing-only/README.md); submission `03141c43c5663e35d21eebc631aaf5eec4e909aa`. No Russia research is imported. |
| C01-31 | Komeito representatives and distinct organizational phases | **Accepted bounded intake.** All 31 originals, 64 claims and fifteen holder observations reviewed; no source holds remain. Exact submission `696937ba3d31280aab569cbd78418e9c3a0c6880`, first import `b74f4fa5`, [acceptance receipt `d4d3542b`](../../campaign-certification/C01/reviews/CLAUDE-C01-31-resumed-20261001/README.md). The conservative Takeya boundary correction, explicit Ota assumption, three locator repairs and earlier held evidence are preserved. |
| C01-38 | French prime ministers, 2014–2026 | **Accepted bounded intake.** All 22 originals, 33 claims and twelve appointment observations reviewed. Same-day interval prose repaired. Effective terms remain unresolved. [Review](../../campaign-certification/C01/integrations/CLAUDE-C01-38/README.md); submitted `acf33f09a5d28cbc9bf9f539e152e9846a7bf395`. |
| C01-39 | Democratic Alliance federal leaders, 2000–2026 | **Held; latest follow-up unaccepted.** Latest `b66f8c074431d7e1a8c9bfca228576c7d5134655` follows the reviewed submission `9cb02c20012a6e6314d7e64d52241609d566f738`. The [original receipt `d1818a48`](../../campaign-certification/C01/reviews/CLAUDE-C01-39/README.md) records 1/24 originals and 1/29 claims reviewed, with eleven submitted holder observations held. The follow-up has twelve added observations, all held; the same 23 originals and 28 claims remain unresolved. One request returned HTTP 429; 22 were not attempted. No DA research or isolated test repair is imported. |
| C01-40 | CPI(M) general secretaries, 1990–2026 | **Accepted bounded intake with reviewed amendment.** The [initial review](../../campaign-certification/C01/integrations/CLAUDE-C01-40/README.md) retains eighteen originals and five locator repairs. The [amendment](../../campaign-certification/C01/reviews/CLAUDE-C01-40-amendment-20261001/README.md) independently checks two added originals and re-reads three rally descriptions: 20 sources, 26 claims, four observations, no new effective boundaries. Submitted `d7e1b9d286f68b7864f2d014c49e42c4296e66e5`; scoped import `3c4a3abc`. Current REST limitations and the opening-holder gap remain. |
| C01-41 | CPSU General Secretary and Deputy General Secretary, 1990–1991 | **Accepted bounded intake.** Twelve originals, 26 claims and five observations reviewed; reportage and decree annotation scope clarified. Prior undated identity remains unreconciled. [Review](../../campaign-certification/C01/integrations/CLAUDE-C01-41/README.md); submitted `a989ddb4d67657236292a35443049554cc9b19e7`. |

The [combined review](../../campaign-certification/C01/reviews/PENDING-2026-10-01/README.md)
separately records the latest DA triage. Correction `9e3d0b3c` adds a Zille 2007
observation, proposes Maimane's effective end as 23 October 2019 and repairs a quote
apostrophe while retaining the same 24 recorded original identities. This is not
completed content review or acceptance; the earlier source receipt stays unchanged.
Review the added observation and proposed boundary against the originals before
integration. No second source pass was made.

The combined review also records follow-ups on accepted France and USSR packets. Only two USSR test
tightenings were adopted; all 48 USSR tests pass, with two failing-before/passing-after
mutation checks. Source and holder scope is unchanged. The additional France and
USSR prose is not imported; the independent acceptance limits remain authoritative.

The later 1 October review and branch check registered these existing claims.
The queue now records 28 completed bounded Claude tasks, three source-held
submissions, one additional unreviewed delivery and the remaining Japan claim.

| Packet | Exact checked tip | Current decision |
|---|---|---|
| [C01-42](CLAUDE-C01-42.md) | `601078190e76bfa3ae578a6b467073620dff1ce6` | Claim only; preserve ownership and current accepted C01-31 corrections. |
| [C01-43](CLAUDE-C01-43.md) | `68fb8f863398247ba1515a1ff3f413a444a6d7f5` | Newly submitted: 14 proposed originals / 21 claims. Not retrieved, reviewed or imported in this pass. |
| [C01-44](CLAUDE-C01-44.md) | `206df90553337f4fbeb4ae34819cb2b01c6e8a21` | **Held**: 4/5 originals and 5/6 claims read, no new holders. PMO returned HTTP 429; one original/claim held. Receipt only, no data import. |
| [C01-45](CLAUDE-C01-45.md) | `d7db2cbcde25594d794d6c2dbbb309cd7d6cbf49` | **Accepted bounded intake**: 18 originals, 30 claims, 18 observations. First import `d23f0bf2`, correction `4a98bc10`, receipt `42046e51`. |
| [C01-46](CLAUDE-C01-46.md) | `64fec76b8a61e7f65f0f98f7bf3a93699ea80d72` | **Accepted bounded intake**: 7 originals, 27 claims, 18 observations. First import `91a1dbe1`, correction `c1056321`, receipt `6f88cd1b`. C01-28 party research remains held. |

The source pass is closed after Tonga’s rate-limit response. Preserve all failed
attempts and resume only missing originals in a later permitted pass. No new
research packet accepts a runtime identity, cartoon, whole country or parent gate.
The ready Tonga cast and campaign-stability work remain the CP1 priorities.

Continue existing claims without duplicating their ownership. Codex owns the
remaining original-source/content reviews for C01-28 and C01-39. Both Archive
passes stopped at HTTP 429 without a Retry-After header; preserve those responses
and coordinate any future missing-only pass when normal service permits. Claude's
next content sequence is:

1. **Reconcile accepted Tonga research into game identities and dated roles.**
   Submit a focused mapping proposal and validation for Codex review; accepted
   research alone is not installed historical coverage.
2. **Produce a reviewed Tonga cartoon batch of 6–8 actual campaign characters.**
   Claim exact person IDs and appearance windows after identity reconciliation.
   Prioritize the opening leader and government/party faces the player encounters.
3. **Finish Tonga, then repeat country casts toward C06:** France, Japan, India, Brazil, South Africa,
   Saudi Arabia and the USSR/Russia case. Resolve source and identity gaps, add
   dated cartoons and clearly fictional successors, then obtain country signoff.
   Worldwide expansion follows the first certified-country casts.
4. **Resolve the held Russia and DA source gaps as a separate work lane.**
   Preserve the reviewed content, unavailable and unattempted distinctions, and
   every failed response. Review actual passages when the remaining originals
   become accessible. Do not delay an independent ready country batch for these
   holds. Repeated rate-limit retries do not establish available evidence.
   Komeito is accepted; do not restart it or undo its conservative corrections.

The source repairs are existing bounded queue records. Tonga now has a queued
production assignment and an active Codex identity proposal; no Claude production
claim or completed delivery is implied by that assignment.
Query the queue and check live branches before claiming a batch. The six earlier
expanded tool/preparation packets are already accepted; do not redo them. S19
guidance is maintenance for reproduced defects. E05 company mechanics remain
after CP1. No research or art batch alone closes C03, C06, S23 or certification.
