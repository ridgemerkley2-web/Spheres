# CLAUDE-C01-42: Japanese Communist Party chairs and DPFP representatives, 1990–2026

Owner: Claude. State: **claimed** (2026-09-30; in progress, not complete). Parent: C01 (incomplete).

Origin: part of the back-to-back C01 research pipeline the user asked for on 28 September 2026, taken from the
certified-country gap ledger (`docs/campaign-certification/C01/gap-ledger/ledger.md`) items `Japan/jp_jcp`, `jp_sangiin_pr_2025_01#jp_jcp_executive_committee_chair`, `jp_sangiin_pr_2025_01#jp_jcp_central_committee_chair`, `jp_sangiin_pr_2025_07#jp_dpfp_representative`. Pending Codex
acceptance; not registered in the task queue.

Branch: `claude/c01-jp-42`. **Stacked on `claude/c01-jp-31`** (a pending, unmerged packet that also edits `japan.json`), merged with current integration `509bd289`: merge that packet first. Claim commit: this record's first commit on the branch.

## Bounded deliverable

Extend the existing roles on jp_sangiin_pr_2025_01 (日本共産党): jp_jcp_executive_committee_chair (幹部会委員長; expected 不破哲三, 志位和夫, 田村智子) and jp_jcp_central_committee_chair (中央委員会議長; expected 宮本顕治 to 1997, the vacancy, 不破哲三, 志位和夫), and jp_dpfp_representative on jp_sangiin_pr_2025_07 (国民民主党; expected 玉木雄一郎, the 2018 co-representative arrangement, and any acting representative during his 2024–2025 suspension as claims only) — at most ten people. Each party-congress or Central Committee plenum election, assumption, acting arrangement and departure is its own dated claim. Keep the existing holders unchanged. Party records (jcp.or.jp, しんぶん赤旗 akahata, new-kokumin.jp and archived pages) are primary; Diet minutes only as a party officer's own statement (C01-29). Names in Japanese as printed.

Keep each distinct event (election or selection, appointment, assumption of office, acting or interim service,
resignation, removal, death, merger, renaming) as its own dated claim; acting service is claims only. Never infer an
end from a successor's start. Give a holder `from` or `until` only where a source states the day; otherwise record
`attested_on`. Party office and state office stay separate both ways. Organization identities, lifecycles and game
mappings stay unresolved; a name match to a simulation row is never a mapping. At most ten people. Primary sources
only; news and encyclopaedias are leads. The historical cutoff stays 7 September 2026.

## Allowed files and checks

Allowed files:

- this record;
- a new `docs/campaign-certification/C01/research/japan-jcp-chairs-dpfp-representatives-1990-2026-42.md`;
- `docs/campaign-certification/C01/research/japan.json` and specifically related new
  `docs/campaign-certification/C01/research/sources/japan-*-facts.json` extracts;
- focused tests under `tools/avatars/`: a new `test_japan_jcp_dpfp_leaders_c01_42.py`, and pinned counts or exact sets in `test_japan_research_s10d.py`, `test_japan_prime_ministers_c01_12.py`, `test_japan_prime_ministers_c01_13.py`, `test_japan_ldp_presidents_c01_18.py`, `test_japan_sdp_chairs_c01_29.py`, `test_japan_komeito_representatives_c01_31.py` updated to
  the new totals (none loosened).

Put generated `research-index.json` changes in a separate commit. Do not change the gap ledger, shared UI, the
roadmap, game data or other country packets.

Checks: research-index `--check`; `campaign_census.py --check`; the Japan, research and campaign Python tests
(census included); the atlas Node check; `workboard.py --check`; `git diff --check`.

Mark the packet `ready_for_review` when done. C01 and all parent gates stay open.
