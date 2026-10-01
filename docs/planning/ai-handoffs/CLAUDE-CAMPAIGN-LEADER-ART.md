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

Updated 1 October 2026 against the live remote tips and completed independent
reviews. The central queue now records all four new packets. It contains 25
completed bounded Claude tasks and three submissions still awaiting acceptance.
These task closures are research/preparation results, not completed country casts.

| Packet | Scope | Current state / next action |
|---|---|---|
| C01-28 | Five Russian party-leader chains | **Held.** Twelve missing originals hold 24 claims and four holder observations. Latest submission `03141c43c5663e35d21eebc631aaf5eec4e909aa`; resume the existing source repair. |
| C01-31 | Komeito representatives and distinct organizational phases | **Held after partial review.** At `696937ba3d31280aab569cbd78418e9c3a0c6880`, 24/31 originals and 50/64 claims were reviewed. Supply seven missing originals; fourteen claims and five holder dependencies remain held. Apply the conservative Takeya boundary correction preserved in the [review receipt](../../campaign-certification/C01/reviews/CLAUDE-C01-31/README.md). No new Japan research is imported. |
| C01-38 | French prime ministers, 2014–2026 | **Accepted bounded intake.** All 22 originals, 33 claims and twelve appointment observations reviewed. Same-day interval prose repaired. Effective terms remain unresolved. [Review](../../campaign-certification/C01/integrations/CLAUDE-C01-38/README.md); submitted `acf33f09a5d28cbc9bf9f539e152e9846a7bf395`. |
| C01-39 | Democratic Alliance federal leaders, 2000–2026 | **Newly delivered; review pending.** `claude/c01-za-39` at `9cb02c20012a6e6314d7e64d52241609d566f738`. Scope triage confirms 24 new sources, 29 claims and eleven added dated observations, preserving prior South Africa data. Original-source content has not yet been independently reviewed. [Registered handoff](CLAUDE-C01-39.md). |
| C01-40 | CPI(M) general secretaries, 1990–2026 | **Accepted bounded intake.** All eighteen originals, 24 claims and four observations reviewed. Five locators and unsupported EMS prose repaired; the opening-holder gap remains. [Review](../../campaign-certification/C01/integrations/CLAUDE-C01-40/README.md); submitted `54680e39910804b3864a3e973c452f2db2ac5e6f`. |
| C01-41 | CPSU General Secretary and Deputy General Secretary, 1990–1991 | **Accepted bounded intake.** Twelve originals, 26 claims and five observations reviewed; reportage and decree annotation scope clarified. Prior undated identity remains unreconciled. [Review](../../campaign-certification/C01/integrations/CLAUDE-C01-41/README.md); submitted `a989ddb4d67657236292a35443049554cc9b19e7`. |

Continue existing claims without duplicating their ownership. Codex owns the
independent review of C01-39 and the remaining source-access dependencies in
C01-28/31. Claude's next content sequence is:

1. **Resolve the held Russia and Komeito source gaps.** Supply accessible original
   evidence, preserve reviewed accessible content and every failed attempt, and
   address the recorded boundary correction. Repeated rate-limit retries are not
   a substitute for available evidence.
2. **Reconcile accepted Tonga research into game identities and dated roles.**
   Submit a focused mapping proposal and validation for Codex review; accepted
   research alone is not installed historical coverage.
3. **Produce a reviewed Tonga cartoon batch of 6–8 actual campaign characters.**
   Claim exact person IDs and appearance windows after identity reconciliation.
   Prioritize the opening leader and government/party faces the player encounters.
4. **Repeat country casts toward C06:** France, Japan, India, Brazil, South Africa,
   Saudi Arabia and the USSR/Russia case. Resolve source and identity gaps, add
   dated cartoons and clearly fictional successors, then obtain country signoff.
   Worldwide expansion follows the first certified-country casts.

The source repairs are existing bounded queue records. The Tonga and country-cast
steps describe the next production sequence, not newly claimed or delivered work.
Query the queue and check live branches before claiming a batch. The six earlier
expanded tool/preparation packets are already accepted; do not redo them. S19
guidance is maintenance for reproduced defects. E05 company mechanics remain
after CP1. No research or art batch alone closes C03, C06, S23 or certification.
