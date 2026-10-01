# CLAUDE-C01-45: Saudi kings and crown princes: Saudi primary attestations, and the Allegiance Commission secretary, 1990–2026

Owner: Claude. State: **claimed** (2026-09-30; in progress, not complete). Parent: C01 (incomplete).

Origin: part of the back-to-back C01 research pipeline the user asked for on 28 September 2026, taken from the
certified-country gap ledger (`docs/campaign-certification/C01/gap-ledger/ledger.md`) items `sa_crown#sa_king`, `sa_crown#sa_crown_prince`, `sa_succession_commission#sa_succession_secretary`. Pending Codex
acceptance; not registered in the task queue.

Branch: `claude/c01-sa-45`. Base: `509bd289` (current `codex/campaign-certification`); not stacked on a pending packet. Claim commit: this record's first commit on the branch.

## Bounded deliverable

Fill the unresolved intervals of the existing roles sa_king and sa_crown_prince (institution sa_crown; holders from CLAUDE-C01-06) with dated Saudi primary attestations — Royal Court statements and royal orders published by the Saudi Press Agency (SPA) or Umm al-Qura, read in Arabic and quoted as printed — for King Fahd (1990–2005), King Abdullah, King Salman and the crown princes Abdullah, Sultan, Nayef, Salman, Muqrin, Mohammed bin Nayef and Mohammed bin Salman; and research the Secretary General of the Allegiance (Succession) Commission (sa_succession_secretary, no holder yet). At most ten people. Keep existing holders unchanged; add dated observations alongside them. Accessions, pledges of allegiance (bay'a), royal orders appointing or relieving a crown prince (from/until only where an order states its effective day), and deaths (until only where the statement names the office and the day) are distinct dated claims. Foreign records (as in C01-06) are leads for this packet.

Keep each distinct event (election or selection, appointment, assumption of office, acting or interim service,
resignation, removal, death, merger, renaming) as its own dated claim; acting service is claims only. Never infer an
end from a successor's start. Give a holder `from` or `until` only where a source states the day; otherwise record
`attested_on`. Party office and state office stay separate both ways. Organization identities, lifecycles and game
mappings stay unresolved; a name match to a simulation row is never a mapping. At most ten people. Primary sources
only; news and encyclopaedias are leads. The historical cutoff stays 7 September 2026.

## Allowed files and checks

Allowed files:

- this record;
- a new `docs/campaign-certification/C01/research/saudi-arabia-kings-crown-princes-1990-2026-45.md`;
- `docs/campaign-certification/C01/research/saudi-arabia.json` and specifically related new
  `docs/campaign-certification/C01/research/sources/saudi-arabia-*-facts.json` extracts;
- focused tests under `tools/avatars/`: a new `test_saudi_kings_crown_princes_c01_45.py`, and pinned counts or exact sets in `test_saudi_executive_c01_06.py`, `test_saudi_shura_allegiance_c01_25.py` updated to
  the new totals (none loosened).

Put generated `research-index.json` changes in a separate commit. Do not change the gap ledger, shared UI, the
roadmap, game data or other country packets.

Checks: research-index `--check`; `campaign_census.py --check`; the SaudiArabia, research and campaign Python tests
(census included); the atlas Node check; `workboard.py --check`; `git diff --check`.

Mark the packet `ready_for_review` when done. C01 and all parent gates stay open.
