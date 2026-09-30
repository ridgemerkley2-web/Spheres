# CLAUDE-C01-38: French prime ministers, 2014–2026

Owner: Claude. State: **claimed** (2026-09-30; in progress, not complete). Parent: C01 (incomplete).

Origin: part of the back-to-back C01 research pipeline the user asked for on 28 September 2026, taken from the
certified-country gap ledger (`docs/campaign-certification/C01/gap-ledger/ledger.md`) items `fr_prime_minister#fr_pm`. Pending Codex
acceptance; not registered in the task queue.

Branch: `claude/c01-fr-38`. Base: `02d2c5a2` (current `codex/campaign-certification`); not stacked on a pending packet. Claim commit: this record's first commit on the branch.

## Bounded deliverable

Extend the existing role fr_pm (institution fr_prime_minister, added by CLAUDE-C01-37) with the Prime Ministers appointed from 31 March 2014 to the cutoff (C01-37's next work FR-PM-12): expected Manuel Valls, Bernard Cazeneuve, Édouard Philippe, Jean Castex, Élisabeth Borne, Gabriel Attal, Michel Barnier, François Bayrou and Sébastien Lecornu (appointed twice in 2025) — at most ten people. Apply C01-37's rulings exactly: the appointment decree naming the person gives `from`; the decree ending the Government's functions gives `until` on its signing day; resignation letters, publication dates and caretaker periods are separate claims; countersignatures give attested_on only. Additions go only through supplements/france.json and the importer (tools/avatars/import_cnccfp_census.py). Sources: JORF texts; Légifrance may refuse scripted access — use raw Internet Archive captures made before the cutoff, as C01-37 did.

Keep each distinct event (election or selection, appointment, assumption of office, acting or interim service,
resignation, removal, death, merger, renaming) as its own dated claim; acting service is claims only. Never infer an
end from a successor's start. Give a holder `from` or `until` only where a source states the day; otherwise record
`attested_on`. Party office and state office stay separate both ways. Organization identities, lifecycles and game
mappings stay unresolved; a name match to a simulation row is never a mapping. At most ten people. Primary sources
only; news and encyclopaedias are leads. The historical cutoff stays 7 September 2026.

## Allowed files and checks

Allowed files:

- this record;
- a new `docs/campaign-certification/C01/research/france-prime-ministers-2014-2026-38.md`;
- `docs/campaign-certification/C01/research/france.json` and specifically related new
  `docs/campaign-certification/C01/research/sources/france-*-facts.json` extracts, plus `docs/campaign-certification/C01/research/supplements/france.json` and a regenerated `france.json` (via `tools/avatars/import_cnccfp_census.py`);
- focused tests under `tools/avatars/`: a new `test_france_prime_ministers_c01_38.py`, and pinned counts or exact sets in `test_campaign_research.py`, `test_import_cnccfp_census.py`, `test_france_presidents_c01_23.py`, `test_france_prime_ministers_c01_37.py` updated to
  the new totals (none loosened).

Put generated `research-index.json` changes in a separate commit. Do not change the gap ledger, shared UI, the
roadmap, game data or other country packets.

Checks: research-index `--check`; `campaign_census.py --check`; the France, research and campaign Python tests
(census included); the atlas Node check; `workboard.py --check`; `git diff --check`.

Mark the packet `ready_for_review` when done. C01 and all parent gates stay open.
