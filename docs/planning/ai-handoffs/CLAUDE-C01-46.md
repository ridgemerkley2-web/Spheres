# CLAUDE-C01-46: State Duma faction heads, 2021–2026

Owner: Claude. State: **ready_for_review** (submitted 1 October 2026 UTC, 30 September local; not complete). Parent:
C01 (incomplete).

Origin: part of the back-to-back C01 research pipeline the user asked for on 28 September 2026, taken from the
certified-country gap ledger (`docs/campaign-certification/C01/gap-ledger/ledger.md`) items `ru_duma_faction_20211012_er#ru_duma_faction_20211012_er_head`, `ru_duma_faction_20211012_kprf#ru_duma_faction_20211012_kprf_head`, `ru_duma_faction_20211012_ldpr#ru_duma_faction_20211012_ldpr_head`, `ru_duma_faction_20211012_nl#ru_duma_faction_20211012_nl_head`, `ru_duma_faction_20211012_srzp#ru_duma_faction_20211012_srzp_head`. Pending Codex
acceptance; not registered in the task queue.

Branch: `claude/c01-ru-46`. Base: `a81d2486` (current `codex/campaign-certification`); not stacked on a pending packet. Claim commit: this record's first commit on the branch, `3ee3357f`. Before the result
commits, `codex/campaign-certification` had moved to `d0c6676b` and was merged into the branch (no conflict; none of
its files are this packet's). Result commits: the packet commit and the separate index commit at the head of
`claude/c01-ru-46` at submission; to be recorded by the integrator. Reviewer/integrator: Codex.

## Bounded deliverable

Extend the five existing roles ru_duma_faction_20211012_{er,kprf,ldpr,nl,srzp}_head (Руководитель фракции; one holder each) for the 8th State Duma from 12 October 2021 to the cutoff: dated attestations of each faction head, and any change of head (e.g. the LDPR faction after Жириновский's death in April 2022) — at most ten people. Keep the existing holders unchanged. The State Duma's own records (duma.gov.ru faction pages and resolutions, the Duma's stenograms, sozd.duma.gov.ru) are primary for a Duma faction office; party office stays separate (a party chairman is not thereby faction head, and vice versa; do not edit CLAUDE-C01-28's party roles). A change of head is a dated claim; `from`/`until` only where a Duma record states the effective day. Russian names as printed.

Keep each distinct event (election or selection, appointment, assumption of office, acting or interim service,
resignation, removal, death, merger, renaming) as its own dated claim; acting service is claims only. Never infer an
end from a successor's start. Give a holder `from` or `until` only where a source states the day; otherwise record
`attested_on`. Party office and state office stay separate both ways. Organization identities, lifecycles and game
mappings stay unresolved; a name match to a simulation row is never a mapping. At most ten people. Primary sources
only; news and encyclopaedias are leads. The historical cutoff stays 7 September 2026.

## Allowed files and checks

Allowed files:

- this record;
- a new `docs/campaign-certification/C01/research/russia-duma-faction-heads-2021-2026-46.md`;
- `docs/campaign-certification/C01/research/russia.json` and specifically related new
  `docs/campaign-certification/C01/research/sources/russia-*-facts.json` extracts;
- focused tests under `tools/avatars/`: a new `test_russia_duma_faction_heads_c01_46.py`, and pinned counts or exact sets in `test_russia_research_s10h.py`, `test_ussr_russia_transition_c01_05.py`, `test_russia_presidents_c01_14.py`, `test_russia_heads_of_government_c01_19.py`, `test_russia_party_leaders_c01_28.py` updated to
  the new totals (none loosened).

Put generated `research-index.json` changes in a separate commit. Do not change the gap ledger, shared UI, the
roadmap, game data or other country packets.

Checks: research-index `--check`; `campaign_census.py --check`; the Russia, research and campaign Python tests
(census included); the atlas Node check; `workboard.py --check`; `git diff --check`.

Mark the packet `ready_for_review` when done. C01 and all parent gates stay open.

## Submission (ready_for_review)

[Report](../../campaign-certification/C01/research/russia-duma-faction-heads-2021-2026-46.md):
`russia-duma-faction-heads-2021-2026-46.md`. One research pass by this packet; pending Codex acceptance. The user chose
to start this batch (C01-42 to C01-46) before Codex's "continue existing claims first" roadmap line.

Observation decisions:

- RU-FH-01 claims only: news 52394 (11 October 2021) reports the faction elections: Васильев elected on 7 October 2021,
  Миронов elected, Зюганов, Жириновский and Нечаев reported as heads ("станут"). Never holder dates.
- RU-FH-02 accepted: news 53098 (22 December 2021) attests Зюганов, Жириновский, Нечаев and Васильев as "Руководитель
  фракции".
- RU-FH-03 accepted, one ruling requested: news 53988 (6 April 2022, 15:58), subtitled "Руководитель фракции ЛДПР ушел
  из жизни…", quotes the faction's "Сегодня … скончался": Жириновский's December 2021 observation gets `until`
  2022-04-06 (fallback: a death claim only). News 53987 (13:10) states no day and is a claim. The same item attests
  Васильев, Зюганов, Миронов and Нечаев.
- RU-FH-04 claims only: news 54314 (18 May 2022): Слуцкий elected head of the ЛДПР faction at its sitting of 18 May
  (no `from`; ruling requested) and earlier acting head (claims only).
- RU-FH-05 and RU-FH-06 accepted: news 54910 (7 July 2022) and 63980 (27 July 2026) attest all five heads.

Holders appended after the five unchanged originals (18): Васильев, Зюганов and Нечаев (attested 2021-12-22,
2022-04-06, 2022-07-07, 2026-07-27); Миронов (2022-04-06, 2022-07-07, 2026-07-27); Жириновский (attested 2021-12-22,
until 2022-04-06); Слуцкий (2022-07-07, 2026-07-27). No `from` anywhere. Six people. Party offices untouched (CLAUDE-C01-28's
roles are on its own branch); no new organization, institution or role.

Rulings requested: (1) Жириновский's `until` from the Duma's subtitle plus the quoted faction statement's "Сегодня"
(which names the party office); (2) whether a faction's own election of its head on a stated day gives `from` for a
parliamentary faction office; (3) separate holder observations per attestation day (as here, the C01-34 style) or one
observation per tenure with later claims (the C01-19 style).

Touched paths: this record; `docs/campaign-certification/C01/research/russia.json` (7 sources, 27 claims, 18 holder
observations on the five faction roles, one coverage note per faction and one packet coverage note; append-only); seven new
`docs/campaign-certification/C01/research/sources/russia-duma-news-*-facts.json` extracts; new
`docs/campaign-certification/C01/research/russia-duma-faction-heads-2021-2026-46.md`; new
`tools/avatars/test_russia_duma_faction_heads_c01_46.py`; pins re-expressed exactly (none loosened) in
`tools/avatars/test_russia_research_s10h.py`, `tools/avatars/test_russia_heads_of_government_c01_19.py`,
`tools/avatars/test_russia_presidents_c01_14.py` and `tools/avatars/test_ussr_government_supreme_soviet_c01_26.py`
(the last is outside the packet's pin list: its hash of every Russia holder is kept for the base holders and a second
hash pins this packet's 18). No existing source or extract is edited. Separate commit:
`docs/campaign-certification/C01/research-index.json` only (shared with the other pending packets; regenerate on
integration).

Checks (1 October 2026 UTC): research-index regeneration and `--check` (1,976 sources, 4,901 claims);
`campaign_census.py --check` passes (exit 0); the Russia tests (36, 7 of them new), USSR tests (48), research tests
(79) and campaign tests (16, census included); the atlas Node check (11); `workboard.py --check` (44 markers) after
adding `docs/campaign-certification/S26` to the sparse checkout (the merged head references
`S26/preparation/RECRUITMENT.md`); `git diff --check`. The new test's 23 mutations each fail as intended.
Known failures outside the suite, not fixed: `test_certified_gap_ledger.py` (no pinned attribution until Codex
classifies this commit) and `test_certified_boundary_matrix.py` (needs `spheres-web/src`, absent from the sparse
checkout). `packet_check.py 46` is run after the push; its summary is returned with the submission.
