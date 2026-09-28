# CLAUDE-C01-28: Russian party leaders, 1990–2026

Owner: Claude. State: **ready_for_review** (remote submission inspected 28 September 2026; not accepted). Parent: C01 (incomplete).

Origin: the first five items of the gap ledger's batch `GAP-USSR-Russia-B001`
(`docs/campaign-certification/C01/gap-ledger/ledger.md`), claimed on the user's 28 September 2026 instruction to
continue development. The five simulation party rows `Russia/ru_kprf`, `Russia/ru_ldpr`, `Russia/ru_yabloko`,
`Russia/ru_apr` and `Russia/ru_vybor` have no leader term at all in the census. No pending packet or repair edits
`russia.json` (SOURCE-26 edits `ussr.json` only). Pending Codex acceptance; not registered in the task queue.

Branch: `claude/c01-ru-28`. Base: `846df479` (current `codex/campaign-certification`); not stacked on a pending
packet. Claim commit: this record's first commit on the branch.

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

## Remote submission inventory — 28 September 2026

The preceding claim-registration paragraph records the earlier claim state. The
remote handoff now declares ready_for_review at
`16153784875006149a68e69c5e83b925e51ca294`, with original claim
`2c4d5bd725b84161fd742adec75febc6241b8c92` preserved. Codex inspected that
[remote handoff](https://github.com/ridgemerkley2-web/Spheres/blob/16153784875006149a68e69c5e83b925e51ca294/docs/planning/ai-handoffs/CLAUDE-C01-28.md)
for inventory only. Its source/test claims are not independently accepted; no
research files, mappings, history or art were imported by this update.
