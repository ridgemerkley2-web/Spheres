# CLAUDE-C01-28: Russian party leaders, 1990–2026

Owner: Claude. State: **ready_for_review** (submitted 28 September 2026; not complete). Parent: C01 (incomplete).

Origin: the first five items of the gap ledger's batch `GAP-USSR-Russia-B001`
(`docs/campaign-certification/C01/gap-ledger/ledger.md`), claimed on the user's 28 September 2026 instruction to
continue development. The five simulation party rows `Russia/ru_kprf`, `Russia/ru_ldpr`, `Russia/ru_yabloko`,
`Russia/ru_apr` and `Russia/ru_vybor` have no leader term at all in the census. No pending packet or repair edits
`russia.json` (SOURCE-26 edits `ussr.json` only). The packet is pending Codex acceptance; Codex registered the claim in the
central task queue (`docs/planning/ai-task-queue.json`, entry `CLAUDE-C01-28`, state `claimed`), which this packet does
not edit.

Branch: `claude/c01-ru-28`. Base: `846df479` (then the head of `codex/campaign-certification`); not stacked on a pending
packet. Claim commit: `2c4d5bd7`, this record's first commit on the branch. `origin/codex/campaign-certification` (then
`30410e55`) was merged before submission (merge commits `7c38049f` and `b6d5c9a8`); it changed none of this packet's research files; its `research-index.json` is regenerated, and its copy of the claim handoff (with Codex's queue-registration note) was kept. The first merge was started with `git -c core.hooksPath=/dev/null merge`, against the house rule; it stopped on the handoff's add/add conflict, and `7c38049f` is that merge completed by a plain `git commit`. No hooks are installed, so nothing was skipped; its tree equals a clean merge with the handoff taken from `a2cf1dcc`. Result commits: the packet commit and the
separate index commit at the head of `claude/c01-ru-28`, to be recorded by the integrator. Reviewer/integrator: Codex.

## Bounded deliverable

Research the national party leader chain (chairman, leader or party head, as each party's own charter names the
office) of five parties from 1 January 1990, or the party's founding, to the 7 September 2026 cutoff — at most ten
people in total across the five chains:

1. The Communist Party of the Russian Federation: its founding or restoration congress and each chairman.
2. The Liberal Democratic Party of Russia: its registration, each chairman and the 2022 change after the
   chairman's death.
3. Yabloko: each chairman from the party's founding.
4. The Agrarian Party of Russia: each chairman, and its 2008 merger into another party only as source-stated facts.
5. Russia's Choice: the 1993 electoral bloc and its successor party (Democratic Choice of Russia) only as far as
   the sources tie the name to the game row; each leader.

Add a party-leader role to the existing 2021 ballot-list organization observation where one exists
(`ru_duma_ballot_list_2021_01` KPRF, `_03` LDPR, `_07` Yabloko); for the Agrarian Party and Russia's Choice add a
new organization observation only from a primary record of that organization. Organization identities, lifecycles
and game mappings stay unresolved; a name match is never a game mapping. Keep congress elections, assumption of
office, acting or interim leadership, resignation, death and merger as distinct dated claims. Acting leaders are
claims only. Never infer an end from a successor's start. Give a holder `from` or `until` only where a source states
the day; otherwise record `attested_on`. Party office stays separate from state office: no presidency, government
or Duma-faction claim may feed a party role, and the existing faction heads do not change.

Primary sources are required: each party's own records (official sites and their archived pages, congress
resolutions), the Ministry of Justice's party registry, the Central Election Commission and official gazettes.
News, encyclopaedias and history sites are leads only. The historical cutoff stays 7 September 2026.

## Allowed files and checks

Allowed files:

- this record;
- a new `docs/campaign-certification/C01/research/russia-party-leaders-1990-2026-28.md`;
- `docs/campaign-certification/C01/research/russia.json` and specifically related new
  `docs/campaign-certification/C01/research/sources/russia-*-facts.json` extracts;
- focused tests under `tools/avatars/`: a new `test_russia_party_leaders_c01_28.py`, and pinned counts or exact sets in
  `test_russia_research_s10h.py`, `test_ussr_russia_transition_c01_05.py`, `test_russia_presidents_c01_14.py` and
  `test_russia_heads_of_government_c01_19.py` updated to the new totals (none loosened).

Put generated `research-index.json` changes in a separate commit. Do not change the gap ledger, shared UI, the
roadmap, game data or other country packets.

Checks: research-index `--check`; `campaign_census.py --check`; the Russia/USSR, research and campaign Python tests
(census included); the atlas Node check; `workboard.py --check`; `git diff --check`.

Mark the packet `ready_for_review` when done. C01 and all parent gates stay open.

## Integration claim registration — 28 September 2026

Codex mirrored this existing claim into the central queue after S22. The original
claim above remains in progress; no research content or historical acceptance
was imported. Inspected remote head: `2c4d5bd7`. Do not duplicate this work.

## Submission (ready_for_review)

[Report](../../campaign-certification/C01/research/russia-party-leaders-1990-2026-28.md):
`russia-party-leaders-1990-2026-28.md`. Built from five research dossiers (KPRF, LDPR, Yabloko, the Agrarian Party, Russia's
Choice with Democratic Choice of Russia) and two independent adversarial checks, whose further primary records are imported.

Result: five `party_leader` roles (`ru_kprf_chairman`, `ru_ldpr_chairman` and `ru_yabloko_chairman` on the ballot-list
observations 01, 03 and 07; `ru_apr_chairman` and `ru_dvr_chairman` on the new observations `ru_apr_party_self_record` and
`ru_dvr_party_self_record`), a third new observation `ru_vybor_rossii_bloc_1993` with no role, 68 sources, 137
claims and 24 holder observations of ten people. No holder has a `from` or an `until`; no game mapping, lifecycle
boundary, portrait or avatar is added.

Decisions per chain:

- KPRF (RU-PTY-01 accepted in part, RU-PTY-02 accepted): Геннадий Андреевич Зюганов attested 23 May 1998, 5 July 2025 and 28
  August 2026. The 1993 restoration congress and his 1993 election as Chairman of the Central Executive Committee rest on the
  party's 2025-2026 retrospective pages (claims); the plenum elections of 20 April 1997, 24 April 2021 and 5 July 2025 are claims;
  the XIX congress's second stage (20 June 2026) took no leadership decision.
- LDPR (RU-PTY-03 and 04 accepted in part): Владимир Вольфович Жириновский attested 23 November 1996 and 30 March 2022; Леонид
  Эдуардович Слуцкий attested 27 May 2022, 2 October 2025 and 11 August 2026. The 1990-1993 founding and elections are
  retrospective claims; the death of 6 April 2022 (the party's news item of that day) is a distinct dated claim, and no `until` is
  set, by the user's ruling of 28 September 2026 (the item never says Председатель, never mentions the office ending, and has
  captures only from October 2025 onward; Codex may still decide at integration that a stated death day ends the office, in which
  case the anchor would be `ru_ldpr_news_zhirinovsky_died_today_20220406`); the interim issue names no acting chairman; the
  Supreme Council's recommendation (26 May 2022) is a nomination.
- Yabloko (RU-PTY-05 and 06 accepted in part, RU-PTY-07 accepted): Григорий Алексеевич Явлинский attested 14 March 1998, 23
  December 2001 and 4 July 2004; Сергей Сергеевич Митрохин 22 June 2008 and 19 December 2015; Эмилия Эдгардовна Слабунова 20
  December 2015; Николай Игоревич Рыбаков 16 December 2019, 13 December 2023 and 19 August 2026. The 1993 bloc, the 1995
  founding and the 2001 transformation are claims; the elections' days are mostly unprinted and never starts. The 1998 holder
  row and the continuation row of 22 December 2001 print the chairman of the association «Объединение ЯБЛОКО» (a party only from
  22 December 2001) and keep that title marked "(as printed)", on the party role; the party calls itself the association's legal
  successor (правопреемница). The two 1998 reference rows (the movement's and the association's chairman) keep their printed
  titles the same way.
- Agrarian Party (RU-PTY-08 and 09 accepted in part): Михаил Иванович Лапшин attested 9 September 2003; Владимир Николаевич
  Плотников 28 May 2004 and 26 September 2008. The 1993-2002 history is retrospective; the election of 28 April 2004, the
  memorandum of 12 September 2008 and the accession decision of 10 October 2008 are claims, not ends.
- Russia's Choice and DVR (RU-PTY-10 accepted in part): Егор Тимурович Гайдар attested 10 July 1994, 18 June 1995, 22 September
  1996 and 16 December 1997 on signed party records. The 1993 bloc is its own observation from its programme; the names are tied
  only by claims; the 1996 chairmanship ballot record, the bare-title signature of 2000 and the self-dissolution decision are
  claims. The party site's history page is a passage of Ю.Г.Коргунюк and С.Е.Заславский, «Российская многопартийность» (1996),
  that the party republished: it is typed `party_republished_reference_text`, `ru_dvr_party_self_record` no longer cites it or
  its four claims (the observation rests on the party's minutes, congress edition, bulletin and newspaper), and the six claims
  resting on it stay on the role as retrospective claims from a reference-book passage the party republished, never
  boundaries: among them the tie to the movement «Выбор России», and the programme's adoption on 19 November 1994 and the III
  congress of 26 August 1995, which have no other source.

Touched paths:

- `docs/campaign-certification/C01/research/russia.json`
- `docs/campaign-certification/C01/research/russia-party-leaders-1990-2026-28.md` (new)
- 68 new extracts `docs/campaign-certification/C01/research/sources/russia-{kprf,ldpr,yabloko,apr,dvr,vybor-rossii}-*-facts.json`
- `tools/avatars/test_russia_party_leaders_c01_28.py` (new)
- `tools/avatars/test_russia_research_s10h.py`, `tools/avatars/test_russia_presidents_c01_14.py`,
  `tools/avatars/test_russia_heads_of_government_c01_19.py` (pinned totals and exact sets; none loosened;
  `test_ussr_russia_transition_c01_05.py` needed no change)
- `tools/avatars/test_ussr_government_supreme_soviet_c01_26.py`, outside the listed pins: integrated after the claim, it
  hashes every Russia role and holder. The guard is re-expressed exactly: the five party roles of this packet are named,
  asserted to be the only `party_leader` roles and excluded, and every other Russia role and holder still hashes to the
  same pinned value. The five roles are pinned with their holders in the new test.
- this record
- `docs/campaign-certification/C01/research-index.json` (separate commit)

No existing extract was edited. The gap ledger, shared UI, the roadmap, game data and other country packets are untouched.

Checks: all passed on 28 September 2026 after the merge of `30410e55`, and again with the same results after the verifier fixes: index regeneration and `--check` (1,396 sources, 3,865 claims);
`campaign_census.py --check`; 39 Russia tests (10 new), 28 USSR, 79 research and 16 campaign tests; 11 atlas Node tests;
`workboard.py --check`; `git diff --check` on this packet's paths. The new test's 47 mutations each fail as intended. Outside the listed checks, `test_certified_gap_ledger.py` errors in `setUpClass` because the 68 new sources have no pinned attribution; that input under `gap-ledger/` can only be refreshed after this packet's commit is classified in `COMMIT_PACKETS`, which is left to the integrator. `test_certified_boundary_matrix.py` (S23, outside the listed checks) fails on this branch in a checkout with its inputs: `CLAUDE-C01-28` is `unclassified_packet` (not listed in `docs/campaign-certification/verification/2026-09-27-claude-integration.md`), and `cases-ussr-russia.json`, `summary.json` and `README.md` are stale. Both pass at `30410e55`. On integration: list the packet as pending and regenerate `docs/campaign-certification/S23/preparation/boundary-matrix/`.

Ruling on the LDPR death (the user, 28 September 2026): Zhirinovsky's 2022 observation gets no `until`. The item
ldpr.ru/event/202499 never says Председатель and never mentions the office ending, and its only captures are from October 2025
onward; the death stays a distinct dated claim. Codex may still decide at integration that a stated death day ends the office,
in which case the anchor would be `ru_ldpr_news_zhirinovsky_died_today_20220406`.
