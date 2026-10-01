# CLAUDE-C01-44: Tongan opposition party leaders (DPFI and PDP), 1990–2026

Owner: Claude. State: **claimed** (2026-09-30; in progress, not complete). Parent: C01 (incomplete).

Origin: part of the back-to-back C01 research pipeline the user asked for on 28 September 2026, taken from the
certified-country gap ledger (`docs/campaign-certification/C01/gap-ledger/ledger.md`) items `to_dpfi#to_dpfi_leader`, `to_dpfi#to_dpfi_president`, `to_pdp#to_pdp_leader`. Pending Codex
acceptance; not registered in the task queue.

Branch: `claude/c01-to-44`. Base: `509bd289` (current `codex/campaign-certification`); not stacked on a pending packet. Claim commit: this record's first commit on the branch.

## Bounded deliverable

Extend the existing roles to_dpfi_leader and to_dpfi_president (Democratic Party of the Friendly Islands, organization to_dpfi; one holder each from CLAUDE-C01-04) and to_pdp_leader (People's Democratic Party, to_pdp; no holder yet) from each party's founding to the cutoff — at most ten people. Keep the existing holders unchanged. Party conventions, elections of leaders and presidents, deaths in office (only where the office and the day are named), acting leadership (claims only) and splits/foundings are distinct dated claims; organization identities stay unresolved. Party records and the Tongan government's own releases (PMO, Legislative Assembly, Gazette) only where they record the party office are primary; Matangi Tonga and other news are leads; party and state office stay separate.

Keep each distinct event (election or selection, appointment, assumption of office, acting or interim service,
resignation, removal, death, merger, renaming) as its own dated claim; acting service is claims only. Never infer an
end from a successor's start. Give a holder `from` or `until` only where a source states the day; otherwise record
`attested_on`. Party office and state office stay separate both ways. Organization identities, lifecycles and game
mappings stay unresolved; a name match to a simulation row is never a mapping. At most ten people. Primary sources
only; news and encyclopaedias are leads. The historical cutoff stays 7 September 2026.

## Allowed files and checks

Allowed files:

- this record;
- a new `docs/campaign-certification/C01/research/tonga-dpfi-pdp-leaders-1990-2026-44.md`;
- `docs/campaign-certification/C01/research/tonga.json` and specifically related new
  `docs/campaign-certification/C01/research/sources/tonga-*-facts.json` extracts;
- focused tests under `tools/avatars/`: a new `test_tonga_dpfi_pdp_leaders_c01_44.py`, and pinned counts or exact sets in `test_tonga_research_s10g.py`, `test_tonga_*_c01_*.py`, `test_tonga_deputy_prime_ministers_c01_36.py`, `test_tonga_dpfi_c01_04.py` updated to
  the new totals (none loosened).

Put generated `research-index.json` changes in a separate commit. Do not change the gap ledger, shared UI, the
roadmap, game data or other country packets.

Checks: research-index `--check`; `campaign_census.py --check`; the Tonga, research and campaign Python tests
(census included); the atlas Node check; `workboard.py --check`; `git diff --check`.

Mark the packet `ready_for_review` when done. C01 and all parent gates stay open.
