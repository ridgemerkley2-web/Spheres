# CLAUDE-C01-49: USSR heads of government and President: dated attestations, 1990–1991

Owner: Claude. State: **claimed** (2026-10-01; in progress, not complete). Parent: C01 (incomplete).

Origin: part of the back-to-back C01 research pipeline the user asked for on 28 September 2026, taken from the
certified-country gap ledger (`docs/campaign-certification/C01/gap-ledger/ledger.md`) items `su_government#su_government_head`, `su_presidency#su_president`. Pending Codex
acceptance; not registered in the task queue.

Branch: `claude/c01-su-49`. Base: `5ea4f8fc` (current `codex/campaign-certification`); not stacked on a pending packet. Claim commit: this record's first commit on the branch.

## Bounded deliverable

Fill the unresolved intervals of the existing roles su_government_head (Совет Министров / Кабинет Министров СССР; holders from CLAUDE-C01-26) and su_president (Presidency of the USSR) with dated primary attestations from 1 January 1990 to 25 December 1991: Рыжков, Павлов, the 1991 interim arrangements (claims only) and Горбачёв as President, including the Congress election of 14-15 March 1990, the August 1991 events (Янаев's 'acting' claim is claims only) and the 25 December 1991 statement. At most ten people. Keep existing holders unchanged; add dated observations alongside them. Sources: USSR Supreme Soviet and Congress records (Vedomosti, stenograms), Pravda and Izvestia facsimiles, archive scans; non-official hosts disclosed as CLAUDE-C01-26/41 did. Russian as printed.

Keep each distinct event (election or selection, appointment, assumption of office, acting or interim service,
resignation, removal, death, merger, renaming) as its own dated claim; acting service is claims only. Never infer an
end from a successor's start. Give a holder `from` or `until` only where a source states the day; otherwise record
`attested_on`. Party office and state office stay separate both ways. Organization identities, lifecycles and game
mappings stay unresolved; a name match to a simulation row is never a mapping. At most ten people. Primary sources
only; news and encyclopaedias are leads. The historical cutoff stays 7 September 2026.

## Allowed files and checks

Allowed files:

- this record;
- a new `docs/campaign-certification/C01/research/ussr-government-president-attestations-1990-1991-49.md`;
- `docs/campaign-certification/C01/research/ussr.json` and specifically related new
  `docs/campaign-certification/C01/research/sources/ussr-*-facts.json` extracts;
- focused tests under `tools/avatars/`: a new `test_ussr_government_president_c01_49.py`, and pinned counts or exact sets in `test_ussr_research_s10h.py`, `test_ussr_russia_transition_c01_05.py`, `test_ussr_government_supreme_soviet_c01_26.py`, `test_ussr_democratic_russia_soyuz_c01_35.py`, `test_ussr_cpsu_general_secretary_c01_41.py` updated to
  the new totals (none loosened).

Put generated `research-index.json` changes in a separate commit. Do not change the gap ledger, shared UI, the
roadmap, game data or other country packets.

Checks: research-index `--check`; `campaign_census.py --check`; the USSR, research and campaign Python tests
(census included); the atlas Node check; `workboard.py --check`; `git diff --check`.

Mark the packet `ready_for_review` when done. C01 and all parent gates stay open.
