# CLAUDE-C01-29: Social Democratic Party (Japan Socialist Party) chairs, 1990–2026

Owner: Claude. State: **claimed** (2026-09-27; in progress, not complete). Parent: C01 (incomplete).

Origin: part of the back-to-back C01 research pipeline the user asked for on 28 September 2026, taken from the
certified-country gap ledger (`docs/campaign-certification/C01/gap-ledger/ledger.md`) items `Japan/jp_jsp`, `Japan/jp_jsp/jp_jsp_1945`, `Japan/jp_jsp/jp_sdp_1996`. Pending Codex
acceptance; not registered in the task queue.

Branch: `claude/c01-jp-29`. Base: `f3514fc6` (current `codex/campaign-certification`); not stacked on a pending packet. Claim commit: this record's first commit on the branch.

## Bounded deliverable

Add one party role, jp_sdp_chair (委員長 / 党首 — chair of the Japan Socialist Party and, from its 1996 renaming, the Social Democratic Party; kind party_leader), to the existing 社会民主党 election-list observation jp_sangiin_pr_2025_10. Research the chairs from 1 January 1990 to the cutoff (expected 土井たか子, 田邊誠, 山花貞夫, 村山富市, then 土井たか子, 福島瑞穂, 吉田忠智, 又市征治 and 福島瑞穂 again): each party election or appointment, assumption, resignation and the January 1996 renaming as distinct dated claims. The renaming is a claim about the organization, never a merge of identities; the 2020 split of most members into another party is a claim only. Sources: the SDP's own records (sdp.or.jp and archived pages), Diet minutes only where a party officer's statement records the party office. Names in Japanese as printed.

Keep each distinct event (election or selection, appointment, assumption of office, acting or interim service,
resignation, removal, death, merger, renaming) as its own dated claim; acting service is claims only. Never infer an
end from a successor's start. Give a holder `from` or `until` only where a source states the day; otherwise record
`attested_on`. Party office and state office stay separate both ways. Organization identities, lifecycles and game
mappings stay unresolved; a name match to a simulation row is never a mapping. At most ten people. Primary sources
only; news and encyclopaedias are leads. The historical cutoff stays 7 September 2026.

## Allowed files and checks

Allowed files:

- this record;
- a new `docs/campaign-certification/C01/research/japan-sdp-chairs-1990-2026-29.md`;
- `docs/campaign-certification/C01/research/japan.json` and specifically related new
  `docs/campaign-certification/C01/research/sources/japan-*-facts.json` extracts;
- focused tests under `tools/avatars/`: a new `test_japan_sdp_chairs_c01_29.py`, and pinned counts or exact sets in `test_japan_research_s10d.py`, `test_japan_prime_ministers_c01_12.py`, `test_japan_prime_ministers_c01_13.py`, `test_japan_ldp_presidents_c01_18.py` updated to
  the new totals (none loosened).

Put generated `research-index.json` changes in a separate commit. Do not change the gap ledger, shared UI, the
roadmap, game data or other country packets.

Checks: research-index `--check`; `campaign_census.py --check`; the Japan, research and campaign Python tests
(census included); the atlas Node check; `workboard.py --check`; `git diff --check`.

Mark the packet `ready_for_review` when done. C01 and all parent gates stay open.

## Integration claim registration — 28 September 2026

Codex mirrored this existing claim into the central queue after S22. The original
claim above remains in progress; no research content or historical acceptance
was imported. Inspected remote head: `2d65b680`. Do not duplicate this work.
