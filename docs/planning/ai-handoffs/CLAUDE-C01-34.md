# CLAUDE-C01-34: PFL/DEM, PDT and PMDB/MDB national presidents, 1990–2026

Owner: Claude. State: **claimed** (2026-09-28; in progress, not complete). Parent: C01 (incomplete).

Origin: part of the back-to-back C01 research pipeline the user asked for on 28 September 2026, taken from the
certified-country gap ledger (`docs/campaign-certification/C01/gap-ledger/ledger.md`) items `Brazil/br_pfl`, `Brazil/br_pdt`, `Brazil/br_pmdb`. Pending Codex
acceptance; not registered in the task queue.

Branch: `claude/c01-br-34`. Base: `032cd6a3` (current `codex/campaign-certification`); not stacked on a pending packet. Claim commit: this record's first commit on the branch.

## Bounded deliverable

Add party-leader roles to the existing FEFC observations for MDB and PDT (and for DEM/União Brasil only if a source ties that label to the PFL line; otherwise record the PFL only as claims) and research each party's national president from 1 January 1990 to the cutoff — at most ten people across the chains. Renamings (PFL to DEM, PMDB to MDB) and mergers (DEM into União Brasil) are claims about the organization, never merged identities. Sources: the parties' own records, the TSE's records of national directorates (SGIP), Congress records only where they record the party office.

Keep each distinct event (election or selection, appointment, assumption of office, acting or interim service,
resignation, removal, death, merger, renaming) as its own dated claim; acting service is claims only. Never infer an
end from a successor's start. Give a holder `from` or `until` only where a source states the day; otherwise record
`attested_on`. Party office and state office stay separate both ways. Organization identities, lifecycles and game
mappings stay unresolved; a name match to a simulation row is never a mapping. At most ten people. Primary sources
only; news and encyclopaedias are leads. The historical cutoff stays 7 September 2026.

## Allowed files and checks

Allowed files:

- this record;
- a new `docs/campaign-certification/C01/research/brazil-party-presidents-1990-2026-34.md`;
- `docs/campaign-certification/C01/research/brazil.json` and specifically related new
  `docs/campaign-certification/C01/research/sources/brazil-*-facts.json` extracts;
- focused tests under `tools/avatars/`: a new `test_brazil_party_presidents_c01_34.py`, and pinned counts or exact sets in `test_brazil_research_s10f.py`, `test_brazil_presidents_c01_10.py`, `test_brazil_vice_presidents_c01_17.py`, `test_brazil_pt_presidents_c01_22.py` updated to
  the new totals (none loosened).

Put generated `research-index.json` changes in a separate commit. Do not change the gap ledger, shared UI, the
roadmap, game data or other country packets.

Checks: research-index `--check`; `campaign_census.py --check`; the Brazil, research and campaign Python tests
(census included); the atlas Node check; `workboard.py --check`; `git diff --check`.

Mark the packet `ready_for_review` when done. C01 and all parent gates stay open.
