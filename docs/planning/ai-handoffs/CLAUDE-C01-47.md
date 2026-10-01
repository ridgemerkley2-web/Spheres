# CLAUDE-C01-47: Socialist Party first secretaries, 1990–2026

Owner: Claude. State: **claimed** (2026-10-01; in progress, not complete). Parent: C01 (incomplete).

Origin: part of the back-to-back C01 research pipeline the user asked for on 28 September 2026, taken from the
certified-country gap ledger (`docs/campaign-certification/C01/gap-ledger/ledger.md`) items `France/fr_ps`. Pending Codex
acceptance; not registered in the task queue.

Branch: `claude/c01-fr-47`. Base: `5ea4f8fc` (current `codex/campaign-certification`); not stacked on a pending packet. Claim commit: this record's first commit on the branch.

## Bounded deliverable

Add one party role, fr_ps_first_secretary (Premier secrétaire; kind party_leader), to the existing CNCCFP observation fr_cnccfp_76 (PARTI SOCIALISTE), through supplements/france.json and the importer. Research the First Secretaries from 1 January 1990 to the cutoff — at most ten people (expected Pierre Mauroy, Laurent Fabius, Michel Rocard, Henri Emmanuelli, Lionel Jospin, François Hollande, Martine Aubry, Harlem Désir, Jean-Christophe Cambadélis, Olivier Faure; an interim (e.g. 2017-2018) is claims only; if more than ten, cover the first ten and list the rest as next work). Each Congress or Conseil national vote, assumption and departure is its own dated claim. The party's own records (parti-socialiste.fr and archived pages, congress results) are primary; JORF only where it records the party office; news are leads.

Keep each distinct event (election or selection, appointment, assumption of office, acting or interim service,
resignation, removal, death, merger, renaming) as its own dated claim; acting service is claims only. Never infer an
end from a successor's start. Give a holder `from` or `until` only where a source states the day; otherwise record
`attested_on`. Party office and state office stay separate both ways. Organization identities, lifecycles and game
mappings stay unresolved; a name match to a simulation row is never a mapping. At most ten people. Primary sources
only; news and encyclopaedias are leads. The historical cutoff stays 7 September 2026.

## Allowed files and checks

Allowed files:

- this record;
- a new `docs/campaign-certification/C01/research/france-ps-first-secretaries-1990-2026-47.md`;
- `docs/campaign-certification/C01/research/france.json` and specifically related new
  `docs/campaign-certification/C01/research/sources/france-*-facts.json` extracts, plus `docs/campaign-certification/C01/research/supplements/france.json` and a regenerated `france.json` (via `tools/avatars/import_cnccfp_census.py`);
- focused tests under `tools/avatars/`: a new `test_france_ps_first_secretaries_c01_47.py`, and pinned counts or exact sets in `test_campaign_research.py`, `test_import_cnccfp_census.py`, `test_france_presidents_c01_23.py`, `test_france_prime_ministers_c01_37.py`, `test_france_pm_boundaries_review.py`, `test_france_prime_ministers_c01_38.py` updated to
  the new totals (none loosened).

Put generated `research-index.json` changes in a separate commit. Do not change the gap ledger, shared UI, the
roadmap, game data or other country packets.

Checks: research-index `--check`; `campaign_census.py --check`; the France, research and campaign Python tests
(census included); the atlas Node check; `workboard.py --check`; `git diff --check`.

Mark the packet `ready_for_review` when done. C01 and all parent gates stay open.
