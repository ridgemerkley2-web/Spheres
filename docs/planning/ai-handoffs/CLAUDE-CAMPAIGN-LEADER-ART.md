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

The [CP1 acceleration assignment](CP1-ACCELERATION.md) keeps Tonga as the first
country-cast priority. Production task `CLAUDE-C06-TONGA-01` is now owned by
**Codex, in progress**; the stable ID preserves its original assignment history.
Candidate `328ff914` on `codex/tonga-country-cast-20261001` remains isolated,
with CI run `36876063103` running at this registration checkpoint. It is not live,
accepted or a completed country cast. Preserve the accepted identity proposal
and C01-44 research; coordinate any Claude assistance with Codex before changing
production identities, artwork or shared runtime. The Russia/DA source holds
remain separate from this candidate's validation.

Updated 1 October 2026 against the live remote tips and completed independent
reviews. The central queue contains 36 completed bounded Claude tasks and two
source-held submissions still awaiting acceptance: Russia C01-28 and DA C01-39.
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

The earlier 1 October review completed the existing C01-42–46 claims as bounded
research. At that checkpoint, the queue recorded 31 completed Claude tasks and
two source-held submissions: Russia C01-28 and DA C01-39. The production ownership
and candidate status above supersede that checkpoint’s queued Tonga assignment.

| Packet | Exact checked tip | Current decision |
|---|---|---|
| [C01-42](CLAUDE-C01-42.md) | `94426852124efff5d146c0a61ba5ad8212311ce8` | **Accepted bounded intake**: 32 originals, 50 claims, 19 new holder observations. First import `3cfbefc5`; [review `a113a1f4`](../../campaign-certification/C01/reviews/CLAUDE-C01-42-20261001/README.md). |
| [C01-43](CLAUDE-C01-43.md) | `68fb8f863398247ba1515a1ff3f413a444a6d7f5` | **Accepted bounded intake**: 14 originals, 21 claims, 5 new holder observations. First import `0a6b571d`; [review `5a0fc340`](../../campaign-certification/C01/reviews/CLAUDE-C01-43-20261001/README.md). |
| [C01-44](CLAUDE-C01-44.md) | `6110064071a513d21bdcee7a8d2d081e90099359` | **Accepted bounded intake**: 5 originals, 6 claims, 0 new holder observations. First import `c19f86c0`; [review `43cafce6`](../../campaign-certification/C01/reviews/CLAUDE-C01-44-resumed-20261001/README.md). |
| [C01-45](CLAUDE-C01-45.md) | `5f083d7fff505189a01cc69aef2e13e1d70d8066` | **Accepted research preserved**: 18 originals, 30 claims, 18 observations. First import `d23f0bf2`, correction `4a98bc10`, historical receipt `42046e51`. [Follow-up](../../campaign-certification/C01/reviews/CLAUDE-C01-45-followup-20261001/README.md) adopts only a stronger date guard at `7aaf39e8`; extra reported user rulings are not imported. |
| [C01-46](CLAUDE-C01-46.md) | `64fec76b8a61e7f65f0f98f7bf3a93699ea80d72` | **Accepted bounded intake**: 7 originals, 27 claims, 18 observations. First import `91a1dbe1`, correction `c1056321`, receipt `6f88cd1b`. C01-28 party research remains held. |

The coordinated resumed pass retrieved only Tonga’s missing original, then the
new Japan and Brazil originals, one request per URL. Earlier failed responses
remain preserved; no Russia/DA source retries occurred. No research packet accepts
a runtime identity, cartoon, whole country or parent gate. Tonga cast production
and campaign-stability work remain the CP1 priorities.

Final follow-up review before publication: Japan `6d845541` adds wording and
comments only; Tonga `e788b4ee` duplicates the integrated Helu correction. Neither
needs another import. Brazil `8e3a72d3` proposes a sixth observation on 19 July
2018: the original supports President of the National Directorate, but equivalence
with the national-president research role is not established. Keep that exact-title
claim and the five accepted observations; the additional promotion is excluded
pending office-identity evidence. Only the report's article/meeting date explanation
was corrected at `1f268515`. Preserve the independent generic-guard repairs.
The table above pins accepted historical submissions, not these later unadopted
changes. [Combined review and follow-up decisions](../../campaign-certification/C01/integrations/REVIEW-20261001-03/README.md)
retain exact tips, raw deltas and evidence. No new original was requested for this triage.


## C01-47–51 independent review

The later 1 October intake adds **80 originals, 126 claims and 41 accepted holder
observations** from 45 proposed uses. All material claims and proposed holder uses
were read. Matching hashes and passing tests alone are not historical acceptance.
The four withheld uses remain attributed facts rather than accepted officeholders.
Earlier evidence, the fixed 7 September 2026 cutoff, and organization/game mapping
uncertainty remain intact. Shared generated metadata is regenerated on the combined
integration after the exact accepted source imports, not imported from packet tips.

| Packet | Exact checked tip | Bounded decision and provenance |
|---|---|---|
| [C01-47](CLAUDE-C01-47.md) | `8226c419fb4c3c1aa0073800f8f3bf80d0663f71` | **Accepted bounded intake**: 24 originals, 47 claims, 13 observations. First import `b94146eb`; [review `32eb9977`](../../campaign-certification/C01/reviews/CLAUDE-C01-47-20261001/README.md). Two date-precision repairs remove unsupported exact days for the 2017 relinquishment and 2025 ratification; one locator is corrected. All thirteen holder objects and earlier source records are preserved. The CNCCFP party observation remains unreconciled with game identities. |
| [C01-48](CLAUDE-C01-48.md) | `bb0f86cded9fb0d196f32e385cbe7f30d3cecded` | **Accepted bounded intake**: 19 originals, 24 claims, 4 observations. First import `72d3d80d`; [review `ee884dfc`](../../campaign-certification/C01/reviews/CLAUDE-C01-48-20261001/README.md). The review restores earlier CPI(M) cutoff guards accidentally weakened by the new test branch and corrects one locator. Four holder objects remain unchanged, including the explicitly effective 27 April 2016 AAP start. Current REST representations retain their historical-capture limitation. |
| [C01-49](CLAUDE-C01-49.md) | `c601d991df0692a3426cf4967abb57d9edbfa56e` | **Accepted bounded intake**: 6 originals, 17 claims, 4 observations. First import `8c891fd5`; [review `9ec1583a`](../../campaign-certification/C01/reviews/CLAUDE-C01-49-20261001/README.md). Four Gorbachev Presidential observations are accepted, including the explicit 15 March 1990 oath/assumption. A fifth proposed observation from an ambiguous Premier signature block is withheld and remains claims-only. Competing authority assertions grant no acting holder or term break; partial-download and host-provenance limits are retained. |
| [C01-50](CLAUDE-C01-50.md) | `8b1a3c975749087fc6ed4bf48c788386fdecfa39` | **Accepted bounded intake**: 15 originals, 18 claims, 11 observations. First import `fc0fad30`; [review `6248bde6`](../../campaign-certification/C01/reviews/CLAUDE-C01-50-20261001/README.md). Eleven holder uses survive: four named records and seven additional observations. Three meeting-only uses are withheld as institution events, and Fahd is anchored by explicit PM styling on 3 October 2004. Standing Council orders remain distinct from session chairing; no new effective tenure boundary is inferred. |
| [C01-51](CLAUDE-C01-51.md) | `2fe2b752e05125e204832f5cd1f60b4c640af28d` | **Accepted bounded intake**: 16 originals, 20 claims, 9 observations. First import `90f7f54c`; [review `1299cfd8`](../../campaign-certification/C01/reviews/CLAUDE-C01-51-20261001/README.md). Nine observations retain no effective starts. The 3 October 1993 end records the decree-stated release, not constitutional legitimacy or an interval of active service. Postrename signatures do not extend entity lifetimes; competing September declarations remain claims-only. The report separates retained originals from an unverified broader author-reported portal sweep. |

The queue has 59 bounded tasks and 36 completed Claude tasks. Russia C01-28 and
DA C01-39 retain their exact held state; no missing-original retry or held research
import is included. C01, C06, S23, S25, G5 and CP1 remain open. These receipts
do not qualify the separate Codex Tonga production candidate.

Continue existing claims without duplicating their ownership. Codex owns the
remaining original-source/content reviews for C01-28 and C01-39. Both Archive
passes stopped at HTTP 429 without a Retry-After header; preserve those responses
and coordinate any future missing-only pass when normal service permits. Claude's
next content sequence is:

1. **Support the existing Codex Tonga production claim only by coordination.**
   Preserve the accepted source and identity proposals; validate or repair exact
   requested evidence or artwork without duplicating candidate implementation.
2. **Continue country casts toward C06 after Tonga's signoff:** France, Japan,
   India, Brazil, South Africa, Saudi Arabia and the USSR/Russia case. Claim exact
   reviewed identities and appearance windows before producing cartoons. Resolve
   dated roles and clearly fictional successors, then obtain each country review.
   Worldwide expansion follows the first certified-country casts.
3. **Resolve the held Russia and DA source gaps as a separate work lane.**
   Preserve unavailable and unattempted distinctions and every failed response.
   Review actual passages when the remaining originals become accessible.
   Repeated rate-limit retries do not establish available evidence. Komeito is
   accepted; do not restart it or undo its conservative corrections.

The source repairs are existing bounded queue records. Tonga's production task
remains in progress under Codex; candidate validation does not imply integration,
country acceptance or parent qualification.

Query the queue and check live branches before claiming a batch. The six earlier
expanded tool/preparation packets are already accepted; do not redo them. S19
guidance is maintenance for reproduced defects. E05 company mechanics remain
after CP1. No research or art batch alone closes C03, C06, S23 or certification.
