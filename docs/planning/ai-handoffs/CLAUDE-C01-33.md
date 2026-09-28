# CLAUDE-C01-33: Janata Dal presidents, 1990–2026

Owner: Claude. State: **claimed** (2026-09-28; in progress, not complete). Parent: C01 (incomplete).

Origin: part of the back-to-back C01 research pipeline the user asked for on 28 September 2026, taken from the
certified-country gap ledger (`docs/campaign-certification/C01/gap-ledger/ledger.md`) items `India/in_jd`. Pending Codex
acceptance; not registered in the task queue.

Branch: `claude/c01-in-33`. **Stacked on `claude/c01-in-27`** (a pending, unmerged packet that also edits `india.json`), merged with current integration `3ec6e155`: merge that packet first. Claim commit: this record's first commit on the branch.

## Bounded deliverable

Research the national presidents of the Janata Dal from 1 January 1990 to its splits and eventual successors, as far as sources tie them to the Janata Dal name — at most ten people. The Janata Dal has no current ECI national-party observation, so add a new organization observation only from a primary record of the Janata Dal itself (ECI party lists or notifications naming it, or its own records), with one role in_jd_president (kind party_leader). Splits (Janata Dal (Secular), Janata Dal (United) and others) are claims about the organization only, never merged identities or inherited leaders. Sources: ECI records, Gazette notifications, Lok Sabha records, party records.

Keep each distinct event (election or selection, appointment, assumption of office, acting or interim service,
resignation, removal, death, merger, renaming) as its own dated claim; acting service is claims only. Never infer an
end from a successor's start. Give a holder `from` or `until` only where a source states the day; otherwise record
`attested_on`. Party office and state office stay separate both ways. Organization identities, lifecycles and game
mappings stay unresolved; a name match to a simulation row is never a mapping. At most ten people. Primary sources
only; news and encyclopaedias are leads. The historical cutoff stays 7 September 2026.

## Allowed files and checks

Allowed files:

- this record;
- a new `docs/campaign-certification/C01/research/india-janata-dal-presidents-1990-2026-33.md`;
- `docs/campaign-certification/C01/research/india.json` and specifically related new
  `docs/campaign-certification/C01/research/sources/india-*-facts.json` extracts;
- focused tests under `tools/avatars/`: a new `test_india_janata_dal_presidents_c01_33.py`, and pinned counts or exact sets in `test_india_research_s10e.py`, `test_india_prime_ministers_c01_11.py`, `test_india_presidents_c01_15.py`, `test_india_inc_presidents_c01_20.py`, `test_india_bjp_presidents_c01_27.py` updated to
  the new totals (none loosened).

Put generated `research-index.json` changes in a separate commit. Do not change the gap ledger, shared UI, the
roadmap, game data or other country packets.

Checks: research-index `--check`; `campaign_census.py --check`; the India, research and campaign Python tests
(census included); the atlas Node check; `workboard.py --check`; `git diff --check`.

Mark the packet `ready_for_review` when done. C01 and all parent gates stay open.
