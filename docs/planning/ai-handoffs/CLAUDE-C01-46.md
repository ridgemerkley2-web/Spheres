# CLAUDE-C01-46: State Duma faction heads, 2021–2026

Owner: Claude. State: **claimed** (2026-09-30; in progress, not complete). Parent: C01 (incomplete).

Origin: part of the back-to-back C01 research pipeline the user asked for on 28 September 2026, taken from the
certified-country gap ledger (`docs/campaign-certification/C01/gap-ledger/ledger.md`) items `ru_duma_faction_20211012_er#ru_duma_faction_20211012_er_head`, `ru_duma_faction_20211012_kprf#ru_duma_faction_20211012_kprf_head`, `ru_duma_faction_20211012_ldpr#ru_duma_faction_20211012_ldpr_head`, `ru_duma_faction_20211012_nl#ru_duma_faction_20211012_nl_head`, `ru_duma_faction_20211012_srzp#ru_duma_faction_20211012_srzp_head`. Pending Codex
acceptance; not registered in the task queue.

Branch: `claude/c01-ru-46`. Base: `a81d2486` (current `codex/campaign-certification`); not stacked on a pending packet. Claim commit: this record's first commit on the branch.

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
