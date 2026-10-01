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
without coordination. This implementation is reserved on
`codex/opening-leader-art-20260930`. The five-portrait and selector changes have
passed scoped checks and await full-regression integration; this documentation
update does not publish that implementation.

## Claude's current work

Remote snapshot checked 30 September 2026 at 23:50 UTC against the authored
branch handoffs and live remote tips. The integrated queue has not yet recorded
the new remote deliveries/claims below. A branch claim is not reviewed content.

| Packet | Scope | Current state / exact remote tip |
|---|---|---|
| C01-31 | Komeito representatives and distinct organizational phases | Delivered for Codex review, not accepted: `claude/c01-jp-31` at `696937ba3d31280aab569cbd78418e9c3a0c6880` (wording clarifications and refreshed index after the initial delivery). The central queue still says claimed. |
| C01-28 | Five Russian party-leader chains | Review held on original-source access; `claude/c01-ru-28` at `03141c43`. |
| C01-38 | French prime ministers, 2014–2026 | Claim only: `claude/c01-fr-38` at `f1420ba1265f6b3d193548eac6f596864c50c2c9`. |
| C01-39 | Democratic Alliance federal leaders, 2000–2026 | Claim only: `claude/c01-za-39` at `03cd0bb9bc6989e47e3da0ff1ca6373320daf30b`. |
| C01-40 | CPI(M) general secretaries, 1990–2026 | Claim only: `claude/c01-in-40` at `c152e35e3cf184a6d7cbde87236a43d8c924fc86`. |
| C01-41 | CPSU General Secretary and Deputy General Secretary, 1990–1991 | Claim only: `claude/c01-su-41` at `7778be5b829587f6361d3935bc5aa50d9181e753`. |

Continue these existing research claims without duplicating their ownership.
Codex must independently review C01-31 and register new claims before treating
them as accepted or installed. The next content sequence is:

1. **Resolve the held Russia packet C01-28.** Supply accessible original evidence
   for the twelve missing originals; 24 claims and four holder observations remain
   held. Preserve the reviewed accessible evidence and original failures.
2. **Reconcile accepted Tonga research into the game identities and dated roles.**
   Submit a focused mapping proposal and validation for Codex review; accepted
   research alone is not installed historical coverage.
3. **Produce a reviewed Tonga cartoon batch of 6–8 actual campaign characters.**
   Claim exact person IDs and appearance windows after the identity reconciliation.
   Prioritize the opening leader and government/party faces the player encounters.
4. **Repeat country casts toward C06:** France, Japan, India, Brazil, South Africa,
   Saudi Arabia and the USSR/Russia case. Resolve source and identity gaps, add
   dated cartoons and clearly fictional successors, then obtain country signoff.
   Worldwide expansion follows the first certified-country casts.

Russia is an existing bounded queue record; the Tonga and country-cast steps are
the next content sequence, not new claims or completed deliveries. Query the queue
and check the live branches above before claiming any batch. The six earlier
expanded tool/preparation packets are already
accepted; do not redo them. S19 guidance is maintenance for reproduced defects.
E05 company mechanics remain after CP1. No research or art batch alone closes
C03, C06, S23 or campaign certification.
