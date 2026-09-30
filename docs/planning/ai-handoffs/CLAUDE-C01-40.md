# CLAUDE-C01-40: CPI(M) general secretaries, 1990–2026

Owner: Claude. State: **claimed** (2026-09-30; in progress, not complete). Parent: C01 (incomplete).

Origin: part of the back-to-back C01 research pipeline the user asked for on 28 September 2026, taken from the
certified-country gap ledger (`docs/campaign-certification/C01/gap-ledger/ledger.md`) items `India/in_cpm`. Pending Codex
acceptance; not registered in the task queue.

Branch: `claude/c01-in-40`. Base: `02d2c5a2` (current `codex/campaign-certification`); not stacked on a pending packet. Claim commit: this record's first commit on the branch.

## Bounded deliverable

Add one party role, in_cpm_general_secretary (kind party_leader), to the existing ECI 2024 national-party observation in_eci_20240323_np_04 (Communist Party of India (Marxist)). Research the General Secretaries from 1 January 1990 to the cutoff: expected E. M. S. Namboodiripad, Harkishan Singh Surjeet, Prakash Karat, Sitaram Yechury (died 12 September 2024), the interim coordinator arrangement after his death (claims only) and M. A. Baby (elected April 2025) — at most ten people. Each Party Congress election is its own dated claim. The party's own records (cpim.org and archived pages, People's Democracy, Polit Bureau statements) are primary; ECI records only where they record the party office; news are leads.

Keep each distinct event (election or selection, appointment, assumption of office, acting or interim service,
resignation, removal, death, merger, renaming) as its own dated claim; acting service is claims only. Never infer an
end from a successor's start. Give a holder `from` or `until` only where a source states the day; otherwise record
`attested_on`. Party office and state office stay separate both ways. Organization identities, lifecycles and game
mappings stay unresolved; a name match to a simulation row is never a mapping. At most ten people. Primary sources
only; news and encyclopaedias are leads. The historical cutoff stays 7 September 2026.

## Allowed files and checks

Allowed files:

- this record;
- a new `docs/campaign-certification/C01/research/india-cpim-general-secretaries-1990-2026-40.md`;
- `docs/campaign-certification/C01/research/india.json` and specifically related new
  `docs/campaign-certification/C01/research/sources/india-*-facts.json` extracts;
- focused tests under `tools/avatars/`: a new `test_india_cpim_general_secretaries_c01_40.py`, and pinned counts or exact sets in `test_india_research_s10e.py`, `test_india_prime_ministers_c01_11.py`, `test_india_presidents_c01_15.py`, `test_india_inc_presidents_c01_20.py`, `test_india_bjp_presidents_c01_27.py`, `test_india_janata_dal_presidents_c01_33.py` updated to
  the new totals (none loosened).

Put generated `research-index.json` changes in a separate commit. Do not change the gap ledger, shared UI, the
roadmap, game data or other country packets.

Checks: research-index `--check`; `campaign_census.py --check`; the India, research and campaign Python tests
(census included); the atlas Node check; `workboard.py --check`; `git diff --check`.

Mark the packet `ready_for_review` when done. C01 and all parent gates stay open.
