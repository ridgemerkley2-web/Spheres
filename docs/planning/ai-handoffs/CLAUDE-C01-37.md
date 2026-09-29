# CLAUDE-C01-37: French prime ministers, 1990–2026

Owner: Claude. State: **claimed** (2026-09-28; in progress, not complete). Parent: C01 (incomplete).

Origin: part of the back-to-back C01 research pipeline the user asked for on 28 September 2026, taken from the
certified-country gap ledger (`docs/campaign-certification/C01/gap-ledger/ledger.md`) items `institution:fr_prime_minister`. Pending Codex
acceptance; not registered in the task queue.

Branch: `claude/c01-fr-37`. Base: `44098c5a` (current `codex/campaign-certification`); not stacked on a pending packet. Claim commit: this record's first commit on the branch.

## Bounded deliverable

Add the French Prime Minister as a new institution fr_prime_minister with role fr_pm (kind head_of_government) through the hand-maintained supplement docs/campaign-certification/C01/research/supplements/france.json introduced by CLAUDE-C01-23 (never hand-edit the generated france.json; regenerate it with tools/avatars/import_cnccfp_census.py). Research the Prime Ministers from 1 January 1990 (Michel Rocard) to the cutoff — at most ten people per packet; if more than ten held office, cover 1990 onward up to the tenth and record the rest as the next batch. Each appointment decree (Journal officiel), the government's resignation and its acceptance, and continued handling of current affairs are distinct dated claims. Sources: the Journal officiel (Légifrance or raw archive captures), gouvernement.fr, elysee.fr and the Assemblée nationale.

Keep each distinct event (election or selection, appointment, assumption of office, acting or interim service,
resignation, removal, death, merger, renaming) as its own dated claim; acting service is claims only. Never infer an
end from a successor's start. Give a holder `from` or `until` only where a source states the day; otherwise record
`attested_on`. Party office and state office stay separate both ways. Organization identities, lifecycles and game
mappings stay unresolved; a name match to a simulation row is never a mapping. At most ten people. Primary sources
only; news and encyclopaedias are leads. The historical cutoff stays 7 September 2026.

## Allowed files and checks

Allowed files:

- this record;
- a new `docs/campaign-certification/C01/research/france-prime-ministers-1990-2026-37.md`;
- `docs/campaign-certification/C01/research/france.json` and specifically related new
  `docs/campaign-certification/C01/research/sources/france-*-facts.json` extracts, plus `docs/campaign-certification/C01/research/supplements/france.json` and a regenerated `france.json` (via `tools/avatars/import_cnccfp_census.py`);
- focused tests under `tools/avatars/`: a new `test_france_prime_ministers_c01_37.py`, and pinned counts or exact sets in `test_campaign_research.py`, `test_import_cnccfp_census.py`, `test_france_presidents_c01_23.py` updated to
  the new totals (none loosened).

Put generated `research-index.json` changes in a separate commit. Do not change the gap ledger, shared UI, the
roadmap, game data or other country packets.

Checks: research-index `--check`; `campaign_census.py --check`; the France, research and campaign Python tests
(census included); the atlas Node check; `workboard.py --check`; `git diff --check`.

Mark the packet `ready_for_review` when done. C01 and all parent gates stay open.
