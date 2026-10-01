# CLAUDE-C01-48: BSP, AAP and NPP national leaders, 1990–2026

Owner: Claude. State: **claimed** (2026-10-01; in progress, not complete). Parent: C01 (incomplete).

Origin: part of the back-to-back C01 research pipeline the user asked for on 28 September 2026, taken from the
certified-country gap ledger (`docs/campaign-certification/C01/gap-ledger/ledger.md`) items `in_eci_20240323_np_02`, `in_eci_20240323_np_01`, `in_eci_20240323_np_06`. Pending Codex
acceptance; not registered in the task queue.

Branch: `claude/c01-in-48`. Base: `5ea4f8fc` (current `codex/campaign-certification`); not stacked on a pending packet. Claim commit: this record's first commit on the branch.

## Bounded deliverable

Add one party_leader role to each of three existing ECI 2024 national-party observations: in_bsp_national_president on in_eci_20240323_np_02 (Bahujan Samaj Party; expected Kanshi Ram, Mayawati), in_aap_national_convenor on in_eci_20240323_np_01 (Aam Aadmi Party; expected Arvind Kejriwal) and in_npp_national_president on in_eci_20240323_np_06 (National People's Party; expected P. A. Sangma, Conrad Sangma) — from each party's founding (or 1 January 1990) to the cutoff, at most ten people. Elections, assumptions, deaths in office (only where the office and the day are named) and acting arrangements (claims only) are distinct dated claims. Party records (official sites and archived pages) and ECI records where they record the party office are primary; news are leads. india.json is 1.2 MB: read it only through tools/slice.py.

Keep each distinct event (election or selection, appointment, assumption of office, acting or interim service,
resignation, removal, death, merger, renaming) as its own dated claim; acting service is claims only. Never infer an
end from a successor's start. Give a holder `from` or `until` only where a source states the day; otherwise record
`attested_on`. Party office and state office stay separate both ways. Organization identities, lifecycles and game
mappings stay unresolved; a name match to a simulation row is never a mapping. At most ten people. Primary sources
only; news and encyclopaedias are leads. The historical cutoff stays 7 September 2026.

## Allowed files and checks

Allowed files:

- this record;
- a new `docs/campaign-certification/C01/research/india-bsp-aap-npp-leaders-1990-2026-48.md`;
- `docs/campaign-certification/C01/research/india.json` and specifically related new
  `docs/campaign-certification/C01/research/sources/india-*-facts.json` extracts;
- focused tests under `tools/avatars/`: a new `test_india_bsp_aap_npp_leaders_c01_48.py`, and pinned counts or exact sets in `test_india_research_s10e.py`, `test_india_prime_ministers_c01_11.py`, `test_india_presidents_c01_15.py`, `test_india_inc_presidents_c01_20.py`, `test_india_bjp_presidents_c01_27.py`, `test_india_janata_dal_presidents_c01_33.py`, `test_india_cpim_general_secretaries_c01_40.py` updated to
  the new totals (none loosened).

Put generated `research-index.json` changes in a separate commit. Do not change the gap ledger, shared UI, the
roadmap, game data or other country packets.

Checks: research-index `--check`; `campaign_census.py --check`; the India, research and campaign Python tests
(census included); the atlas Node check; `workboard.py --check`; `git diff --check`.

Mark the packet `ready_for_review` when done. C01 and all parent gates stay open.
