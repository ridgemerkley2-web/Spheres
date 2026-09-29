# CLAUDE-C01-36: Tongan Deputy Prime Ministers, 1990–2026

Owner: Claude. State: **claimed** (2026-09-28; in progress, not complete). Parent: C01 (incomplete).

Origin: part of the back-to-back C01 research pipeline the user asked for on 28 September 2026, taken from the
certified-country gap ledger (`docs/campaign-certification/C01/gap-ledger/ledger.md`) items `to_cabinet#to_deputy_pm`. Pending Codex
acceptance; not registered in the task queue.

Branch: `claude/c01-to-36`. Base: `44098c5a` (current `codex/campaign-certification`); not stacked on a pending packet. Claim commit: this record's first commit on the branch.

## Bounded deliverable

Extend the existing to_deputy_pm role of to_cabinet (which holds holders from 2021 onward) with the Deputy Prime Ministers from 1 January 1990 to 2021 — at most ten people. Royal appointments, Cabinet announcements, acting service and resignations are distinct dated claims; acting Prime Minister service by a Deputy is a claim on to_pm only, never a to_pm holder. Existing holders do not change. Sources: the PMO, Palace Office, Government Gazette, Legislative Assembly records; IPU only as corroboration.

Keep each distinct event (election or selection, appointment, assumption of office, acting or interim service,
resignation, removal, death, merger, renaming) as its own dated claim; acting service is claims only. Never infer an
end from a successor's start. Give a holder `from` or `until` only where a source states the day; otherwise record
`attested_on`. Party office and state office stay separate both ways. Organization identities, lifecycles and game
mappings stay unresolved; a name match to a simulation row is never a mapping. At most ten people. Primary sources
only; news and encyclopaedias are leads. The historical cutoff stays 7 September 2026.

## Allowed files and checks

Allowed files:

- this record;
- a new `docs/campaign-certification/C01/research/tonga-deputy-prime-ministers-1990-2026-36.md`;
- `docs/campaign-certification/C01/research/tonga.json` and specifically related new
  `docs/campaign-certification/C01/research/sources/tonga-*-facts.json` extracts;
- focused tests under `tools/avatars/`: a new `test_tonga_deputy_prime_ministers_c01_36.py`, and pinned counts or exact sets in `test_tonga_research_s10g.py`, `test_tonga_*_c01_*.py` updated to
  the new totals (none loosened).

Put generated `research-index.json` changes in a separate commit. Do not change the gap ledger, shared UI, the
roadmap, game data or other country packets.

Checks: research-index `--check`; `campaign_census.py --check`; the Tonga, research and campaign Python tests
(census included); the atlas Node check; `workboard.py --check`; `git diff --check`.

Mark the packet `ready_for_review` when done. C01 and all parent gates stay open.
