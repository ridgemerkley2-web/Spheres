# CLAUDE-C01-41: CPSU General Secretary and Deputy General Secretary, 1990–1991

Owner: Claude. State: **claimed** (2026-09-30; in progress, not complete). Parent: C01 (incomplete).

Origin: part of the back-to-back C01 research pipeline the user asked for on 28 September 2026, taken from the
certified-country gap ledger (`docs/campaign-certification/C01/gap-ledger/ledger.md`) items `USSR/su_cpsu`. Pending Codex
acceptance; not registered in the task queue.

Branch: `claude/c01-su-41`. Base: `02d2c5a2` (current `codex/campaign-certification`); not stacked on a pending packet. Claim commit: this record's first commit on the branch.

## Bounded deliverable

Date the existing su_cpsu roles su_cpsu_general_secretary and su_cpsu_deputy_general_secretary from 1 January 1990 to the party's end, adding dated holder observations without renaming or deleting the existing undated ones (the existing deputy holder's source spelling 'Ivashkov' needs identity care: note it, never reconcile it). Expected: Gorbachev as General Secretary (XXVIII Congress election of July 1990; his statement of 24 August 1991 giving up the office) and Ивашко as Deputy General Secretary (elected July 1990), with his acting service after 24 August 1991 as claims only. The suspension of the party's activity (29 August 1991) and the RSFSR presidential decree of 6 November 1991 are organization claims only. At most ten people. Sources: CPSU records (Pravda, the XXVIII Congress stenogram and resolutions), USSR Supreme Soviet Vedomosti, archive facsimiles and raw Internet Archive captures; news are leads.

Keep each distinct event (election or selection, appointment, assumption of office, acting or interim service,
resignation, removal, death, merger, renaming) as its own dated claim; acting service is claims only. Never infer an
end from a successor's start. Give a holder `from` or `until` only where a source states the day; otherwise record
`attested_on`. Party office and state office stay separate both ways. Organization identities, lifecycles and game
mappings stay unresolved; a name match to a simulation row is never a mapping. At most ten people. Primary sources
only; news and encyclopaedias are leads. The historical cutoff stays 7 September 2026.

## Allowed files and checks

Allowed files:

- this record;
- a new `docs/campaign-certification/C01/research/ussr-cpsu-general-secretary-1990-1991-41.md`;
- `docs/campaign-certification/C01/research/ussr.json` and specifically related new
  `docs/campaign-certification/C01/research/sources/ussr-*-facts.json` extracts;
- focused tests under `tools/avatars/`: a new `test_ussr_cpsu_general_secretary_c01_41.py`, and pinned counts or exact sets in `test_ussr_research_s10h.py`, `test_ussr_russia_transition_c01_05.py`, `test_ussr_government_supreme_soviet_c01_26.py`, `test_ussr_democratic_russia_soyuz_c01_35.py` updated to
  the new totals (none loosened).

Put generated `research-index.json` changes in a separate commit. Do not change the gap ledger, shared UI, the
roadmap, game data or other country packets.

Checks: research-index `--check`; `campaign_census.py --check`; the USSR, research and campaign Python tests
(census included); the atlas Node check; `workboard.py --check`; `git diff --check`.

Mark the packet `ready_for_review` when done. C01 and all parent gates stay open.
