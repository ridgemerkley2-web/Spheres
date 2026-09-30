# CLAUDE-C01-31: Komeito representatives, 1990–2026

Owner: Claude. State: **claimed** (2026-09-30; in progress, not complete). Parent: C01 (incomplete).

Origin: part of the back-to-back C01 research pipeline the user asked for on 28 September 2026, taken from the
certified-country gap ledger (`docs/campaign-certification/C01/gap-ledger/ledger.md`) items `Japan/jp_komeito`, `Japan/jp_komeito/jp_komei_1994`, `Japan/jp_komeito/jp_komeito_1998`. Pending Codex
acceptance; not registered in the task queue.

Branch: `claude/c01-jp-31`. Base: `ff01ee61` (current `codex/campaign-certification`); not stacked on a pending packet. Claim commit: this record's first commit on the branch.

## Bounded deliverable

Add one party role, jp_komeito_representative (代表 / 委員長; kind party_leader), to the existing 公明党 election-list observation jp_sangiin_pr_2025_15. Research the party's leaders from 1 January 1990 to the cutoff (expected 石田幸四郎; the 1994 division into 公明 and the New Frontier Party and its 1998 re-formation as claims about the organization only; then 神崎武法, 太田昭宏, 山口那津男, 石井啓一, 斉藤鉄夫) — at most ten people. Each party-congress election, assumption and resignation is a distinct dated claim; the 1994-1998 organizations are never merged into one identity. Sources: Komeito's own records (komei.or.jp and archived pages); Diet minutes only where a party officer's statement records the party office. Names in Japanese as printed.

Keep each distinct event (election or selection, appointment, assumption of office, acting or interim service,
resignation, removal, death, merger, renaming) as its own dated claim; acting service is claims only. Never infer an
end from a successor's start. Give a holder `from` or `until` only where a source states the day; otherwise record
`attested_on`. Party office and state office stay separate both ways. Organization identities, lifecycles and game
mappings stay unresolved; a name match to a simulation row is never a mapping. At most ten people. Primary sources
only; news and encyclopaedias are leads. The historical cutoff stays 7 September 2026.

## Allowed files and checks

Allowed files:

- this record;
- a new `docs/campaign-certification/C01/research/japan-komeito-representatives-1990-2026-31.md`;
- `docs/campaign-certification/C01/research/japan.json` and specifically related new
  `docs/campaign-certification/C01/research/sources/japan-*-facts.json` extracts;
- focused tests under `tools/avatars/`: a new `test_japan_komeito_representatives_c01_31.py`, and pinned counts or exact sets in `test_japan_research_s10d.py`, `test_japan_prime_ministers_c01_12.py`, `test_japan_prime_ministers_c01_13.py`, `test_japan_ldp_presidents_c01_18.py`, `test_japan_sdp_chairs_c01_29.py` updated to
  the new totals (none loosened).

Put generated `research-index.json` changes in a separate commit. Do not change the gap ledger, shared UI, the
roadmap, game data or other country packets.

Checks: research-index `--check`; `campaign_census.py --check`; the Japan, research and campaign Python tests
(census included); the atlas Node check; `workboard.py --check`; `git diff --check`.

Mark the packet `ready_for_review` when done. C01 and all parent gates stay open.
