# CLAUDE-C01-51: RSFSR President and Vice-President, 1991–1993

Owner: Claude. State: **ready_for_review** (submitted 1 October 2026 UTC; not complete). Parent: C01 (incomplete).

Origin: part of the back-to-back C01 research pipeline the user asked for on 28 September 2026, taken from the
certified-country gap ledger (`docs/campaign-certification/C01/gap-ledger/ledger.md`) items `ru_rsfsr_presidency#ru_rsfsr_president`, `ru_rsfsr_presidency#ru_rsfsr_vice_president`. Pending Codex
acceptance; not registered in the task queue.

Branch: `claude/c01-ru-51`. Base: `5ea4f8fc` (current `codex/campaign-certification`); not stacked on a pending packet. Claim commit: this record's first commit on the branch, `4b15a1d7`. Before the result
commits, `codex/campaign-certification` was fetched again and had not moved. Result commits: the packet commit and the
separate index commit at the head of `claude/c01-ru-51` at submission; to be recorded by the integrator.
Reviewer/integrator: Codex.

## Bounded deliverable

Fill the unresolved intervals of the existing roles ru_rsfsr_president and ru_rsfsr_vice_president (institution ru_rsfsr_presidency; one holder each) with dated primary attestations from the creation of the offices (12 June 1991 election; 10 July 1991 inauguration) to the end of the vice-presidency in 1993: Ельцин as President of the RSFSR / Russian Federation until the office is styled differently (coordinate with the existing ru_president role; never duplicate it), and Руцкой as Vice-President, including the decree of 1 September 1993 suspending him and the Supreme Soviet's 22 September 1993 'acting President' resolution (claims only). At most ten people. Keep existing holders unchanged. Sources: Vedomosti of the RSFSR Congress and Supreme Soviet, presidential decrees (pravo.gov.ru, kremlin.ru archives), stenograms; Russian as printed.

Keep each distinct event (election or selection, appointment, assumption of office, acting or interim service,
resignation, removal, death, merger, renaming) as its own dated claim; acting service is claims only. Never infer an
end from a successor's start. Give a holder `from` or `until` only where a source states the day; otherwise record
`attested_on`. Party office and state office stay separate both ways. Organization identities, lifecycles and game
mappings stay unresolved; a name match to a simulation row is never a mapping. At most ten people. Primary sources
only; news and encyclopaedias are leads. The historical cutoff stays 7 September 2026.

## Allowed files and checks

Allowed files:

- this record;
- a new `docs/campaign-certification/C01/research/russia-rsfsr-president-vice-president-1991-1993-51.md`;
- `docs/campaign-certification/C01/research/russia.json` and specifically related new
  `docs/campaign-certification/C01/research/sources/russia-*-facts.json` extracts;
- focused tests under `tools/avatars/`: a new `test_russia_rsfsr_presidency_c01_51.py`, and pinned counts or exact sets in `test_russia_research_s10h.py`, `test_ussr_russia_transition_c01_05.py`, `test_russia_presidents_c01_14.py`, `test_russia_heads_of_government_c01_19.py`, `test_russia_party_leaders_c01_28.py`, `test_russia_duma_faction_heads_c01_46.py` updated to
  the new totals (none loosened).

Put generated `research-index.json` changes in a separate commit. Do not change the gap ledger, shared UI, the
roadmap, game data or other country packets.

Checks: research-index `--check`; `campaign_census.py --check`; the Russia, research and campaign Python tests
(census included); the atlas Node check; `workboard.py --check`; `git diff --check`.

Mark the packet `ready_for_review` when done. C01 and all parent gates stay open.

## Submission (ready_for_review)

[Report](../../campaign-certification/C01/research/russia-rsfsr-president-vice-president-1991-1993-51.md):
`russia-rsfsr-president-vice-president-1991-1993-51.md`. One research pass by this packet; pending Codex acceptance.

Observation decisions:

- RU-RSP-01 accepted: decree 8 (19 July 1991) and order 98-рп (19 November 1991) signed "Президент РСФСР"; the
  Vice-President's own order 1-рв (29 July 1991) signed "Вице-президент РСФСР А. Руцкой" (`attested_on` only). The
  assignment of the Vice-President's duties (98-рп) names the office only and is a claim.
- RU-RSP-02 accepted, rulings requested: decree 316 (26 December 1991) is signed "Президент Российской Федерации" (a
  styling claim on `ru_rsfsr_president`); decrees 318 (26 December 1991) and 245-н (3 January 1992) are still signed
  "Президент РСФСР" (observations); the Vice-President signs "РСФСР" on 5 December 1991 and "Российской Федерации" on 16
  January 1992 (observation filed on `ru_rsfsr_vice_president`).
- RU-RSP-03 accepted: Supreme Soviet resolution 4825-I (16 April 1993) names "вице-президента Российской Федерации ... А.
  В. Руцкого".
- RU-RSP-04 accepted, one ruling requested: decree 1328 (1 September 1993) names the Vice-President while suspending him
  (attestation; the suspension is a claim, never an end); decree 1398 (18 September 1993) is procedure.
- RU-RSP-05 claims only: Presidium resolution 5779-I (21 September 1993), Supreme Soviet resolutions 5780-I and 5781-I (22
  September 1993) and decree 1410 (22 September 1993), five claims on `ru_president`; no holder, start or end anywhere.
- RU-RSP-06 accepted, pending a ruling: decree 1576 (3 October 1993) releases him from the office "с момента его
  подписания": `until` 1993-10-03 on the 1 September observation.

Holders appended after the unchanged C01-05 holders (9): Борис Николаевич Ельцин on `ru_rsfsr_president` (attested
1991-07-19, 1991-11-19, 1991-12-26, 1992-01-03); Александр Владимирович Руцкой on `ru_rsfsr_vice_president` (attested
1991-07-29, 1991-12-05, 1992-01-16, 1993-04-16; attested 1993-09-01 with until 1993-10-03). No `from` anywhere. Two people
as holders. `ru_president` gains five claims and no holder. Party offices untouched (CLAUDE-C01-28's roles are on its own
branch); no new organization, institution or role.

## Decisions for Codex

1. Decree 1576's "с момента его подписания" in a decree dated 3 October 1993 read as stating the effective day (`until`
   1993-10-03), as C01-19 read decrees 861 and 300 for `from`; fallback under review 1739eccb: no `until`.
2. Decree 1328's styling of the suspended Vice-President as an attestation (1993-09-01); fallback: claim only, with the end
   on the 16 April 1993 observation.
3. RSFSR-styled signatures after the 25 December 1991 renaming (decrees 318 and 245-н) as `ru_rsfsr_president`
   observations, outside the gap ledger's window for the role; fallback: styling claims.
4. Decree 316's new title as a claim on `ru_rsfsr_president`, leaving C01-Russia-PRES-002 (1991-1996 `ru_president`) open.
5. The 1992-1993 "Вице-президент Российской Федерации" observations on `ru_rsfsr_vice_president` (title unchanged; no
   new role).
6. The five September 1993 termination and acting-service claims on `ru_president` (claims only), which keeps C01-14's
   rule that `ru_president` shares no source with the RSFSR roles.
7. HTTP identities recorded (`source_response_url`), `source_url` the HTTPS packet URL, as in C01-14 and C01-19; HTTPS to
   pravo.gov.ru is unreachable from this environment, so `packet_check.py` cannot re-fetch `source_url` (see Checks).

Touched paths: this record; `docs/campaign-certification/C01/research/russia.json` (16 sources, 20 claims, 9 holder
observations, claims appended to the three presidency roles, three institution coverage notes and one packet coverage
note; append-only); sixteen new `docs/campaign-certification/C01/research/sources/russia-ips-*-facts.json` extracts; new
`docs/campaign-certification/C01/research/russia-rsfsr-president-vice-president-1991-1993-51.md`; new
`tools/avatars/test_russia_rsfsr_presidency_c01_51.py`; pins re-expressed exactly (none loosened) in
`tools/avatars/test_russia_research_s10h.py`, `tools/avatars/test_ussr_russia_transition_c01_05.py`,
`tools/avatars/test_russia_presidents_c01_14.py`, `tools/avatars/test_russia_heads_of_government_c01_19.py`,
`tools/avatars/test_russia_duma_faction_heads_c01_46.py` and `tools/avatars/test_ussr_government_supreme_soviet_c01_26.py`
(the last is outside the packet's pin list but in its test patterns: its hash of every Russia holder is kept for the base
holders and a third hash pins this packet's nine, as C01-46 did). `test_russia_party_leaders_c01_28.py` is not on this
base and is not touched. No existing source or extract is edited. Separate commit:
`docs/campaign-certification/C01/research-index.json` only (shared with the other parallel packets; regenerate on
integration).

Checks (1 October 2026 UTC, before the commits):

- `campaign_research.py` regenerated and `--check` passes (2,063 sources and 5,030 claims).
- `campaign_census.py --check` passes (exit 0).
- Russia tests (44, 7 of them new), USSR tests (48), research tests (79) and campaign tests (16, census included): OK.
- Atlas Node check (`check_leadership_research_review.cjs`): 11 pass, 0 fail.
- `workboard.py --check`: PASS (44 canonical markers) after adding `docs/campaign-certification/S26` to this worktree's
  sparse checkout (the base references `S26/preparation/RECRUITMENT.md`).
- `git diff --check`: clean.
- The new test's 26 mutations each fail as intended.
- A third HTTP download of every document frame and card (12:35-12:40Z, at least 49 minutes after the first) matched all
  16 sources byte for byte; three requests needed one retry after a timeout.
- `packet_check.py 51` is run after the push; HTTPS to pravo.gov.ru is unreachable from this environment, so its re-fetch
  of each extract's HTTPS `source_url` is expected to fail with HTTP 000 (see Decisions for Codex, item 7). Its summary is
  returned with the submission.

Known failures outside the suite, not fixed: `test_certified_gap_ledger.py` ("Source ru_rsfsr_ukaz_8_19910719 (Russia) has
no pinned attribution" until Codex classifies this commit) and `test_certified_boundary_matrix.py` (needs
`spheres-web/src/person_avatar_assets.rs`, absent from the sparse checkout).
