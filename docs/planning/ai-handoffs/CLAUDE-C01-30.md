# CLAUDE-C01-30: ACDP, Freedom Front and IFP leaders, 1990–2026

Owner: Claude. State: **claimed** (2026-09-28; in progress, not complete). Parent: C01 (incomplete).

Origin: part of the back-to-back C01 research pipeline the user asked for on 28 September 2026, taken from the
certified-country gap ledger (`docs/campaign-certification/C01/gap-ledger/ledger.md`) items `SouthAfrica/za_acdp`, `SouthAfrica/za_ff`, `SouthAfrica/za_ifp`. Pending Codex
acceptance; not registered in the task queue.

Branch: `claude/c01-za-30`. Base: `b949f64e` (current `codex/campaign-certification`); not stacked on a pending packet. Claim commit: this record's first commit on the branch.

## Bounded deliverable

Add one party-leader role (kind party_leader) to each of three existing IEC observations: za_iec_n2024_008 (AFRICAN CHRISTIAN DEMOCRATIC PARTY), za_iec_n2024_051 (VRYHEIDSFRONT PLUS, the Freedom Front's later name) and za_iec_n2024_034 (INKATHA FREEDOM PARTY). Research each party's national leader or president from 1 January 1990 (or the party's founding) to the cutoff — at most ten people in total (expected Kenneth Meshoe; Constand Viljoen, Pieter Mulder, Pieter Groenewald; Mangosuthu Buthelezi, Velenkosini Hlabisa). The Freedom Front's renaming to Freedom Front Plus and mergers are claims about the organization, never merged identities; Buthelezi's death in 2023 and any transition are distinct dated claims. Party office stays separate from state office (the IFP leader's ministerial posts never feed the party role). Sources: each party's own records (official sites, archived pages, congress resolutions and statements), IEC records and Parliament only where they record the party office.

Keep each distinct event (election or selection, appointment, assumption of office, acting or interim service,
resignation, removal, death, merger, renaming) as its own dated claim; acting service is claims only. Never infer an
end from a successor's start. Give a holder `from` or `until` only where a source states the day; otherwise record
`attested_on`. Party office and state office stay separate both ways. Organization identities, lifecycles and game
mappings stay unresolved; a name match to a simulation row is never a mapping. At most ten people. Primary sources
only; news and encyclopaedias are leads. The historical cutoff stays 7 September 2026.

## Allowed files and checks

Allowed files:

- this record;
- a new `docs/campaign-certification/C01/research/south-africa-acdp-ff-ifp-leaders-1990-2026-30.md`;
- `docs/campaign-certification/C01/research/south-africa.json` and specifically related new
  `docs/campaign-certification/C01/research/sources/south-africa-*-facts.json` extracts;
- focused tests under `tools/avatars/`: a new `test_south_africa_party_leaders_c01_30.py`, and pinned counts or exact sets in `test_south_africa_research_s10h.py`, `test_south_africa_heads_of_state_c01_09.py`, `test_south_africa_anc_presidents_c01_16.py`, `test_south_africa_deputy_presidents_c01_21.py` updated to
  the new totals (none loosened).

Put generated `research-index.json` changes in a separate commit. Do not change the gap ledger, shared UI, the
roadmap, game data or other country packets.

Checks: research-index `--check`; `campaign_census.py --check`; the SouthAfrica, research and campaign Python tests
(census included); the atlas Node check; `workboard.py --check`; `git diff --check`.

Mark the packet `ready_for_review` when done. C01 and all parent gates stay open.
