# CLAUDE-C01-43: PRN / PTC / Agir national presidents, 1990–2026

Owner: Claude. State: **claimed** (2026-09-30; in progress, not complete). Parent: C01 (incomplete).

Origin: part of the back-to-back C01 research pipeline the user asked for on 28 September 2026, taken from the
certified-country gap ledger (`docs/campaign-certification/C01/gap-ledger/ledger.md`) items `Brazil/br_prn`, `Brazil/br_pds`. Pending Codex
acceptance; not registered in the task queue.

Branch: `claude/c01-br-43`. Base: `509bd289` (current `codex/campaign-certification`); not stacked on a pending packet. Claim commit: this record's first commit on the branch.

## Bounded deliverable

Add one party role, br_agir_president (kind party_leader), to the existing TSE 2024 observation br_tse_fefc_2024_party_07 (AGIR). Research the national presidents of the Partido da Reconstrução Nacional (PRN), renamed Partido Trabalhista Cristão (PTC) and then Agir, from 1 January 1990 to the cutoff — at most ten people. The renamings (PRN to PTC, PTC to Agir) are organization claims, recorded with the TSE's own decisions, never merged identities beyond what the TSE registry states (same registration/CNPJ, as C01-34 ruled for PMDB to MDB). The Partido Democrático Social (PDS, 1980–1993, merged into PPR) has no research observation: its national presidents are claims only. Party records (archived party pages, convention editais) and TSE/SGIP registry records are primary; never copy SGIP personal data (hash, then delete raw bodies); news are leads.

Keep each distinct event (election or selection, appointment, assumption of office, acting or interim service,
resignation, removal, death, merger, renaming) as its own dated claim; acting service is claims only. Never infer an
end from a successor's start. Give a holder `from` or `until` only where a source states the day; otherwise record
`attested_on`. Party office and state office stay separate both ways. Organization identities, lifecycles and game
mappings stay unresolved; a name match to a simulation row is never a mapping. At most ten people. Primary sources
only; news and encyclopaedias are leads. The historical cutoff stays 7 September 2026.

## Allowed files and checks

Allowed files:

- this record;
- a new `docs/campaign-certification/C01/research/brazil-prn-ptc-agir-presidents-1990-2026-43.md`;
- `docs/campaign-certification/C01/research/brazil.json` and specifically related new
  `docs/campaign-certification/C01/research/sources/brazil-*-facts.json` extracts;
- focused tests under `tools/avatars/`: a new `test_brazil_prn_agir_presidents_c01_43.py`, and pinned counts or exact sets in `test_brazil_research_s10f.py`, `test_brazil_presidents_c01_10.py`, `test_brazil_vice_presidents_c01_17.py`, `test_brazil_pt_presidents_c01_22.py`, `test_brazil_party_presidents_c01_34.py` updated to
  the new totals (none loosened).

Put generated `research-index.json` changes in a separate commit. Do not change the gap ledger, shared UI, the
roadmap, game data or other country packets.

Checks: research-index `--check`; `campaign_census.py --check`; the Brazil, research and campaign Python tests
(census included); the atlas Node check; `workboard.py --check`; `git diff --check`.

Mark the packet `ready_for_review` when done. C01 and all parent gates stay open.
