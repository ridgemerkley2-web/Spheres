# CLAUDE-C01-50: Saudi prime ministers, 1990–2026

Owner: Claude. State: **claimed** (2026-10-01; in progress, not complete). Parent: C01 (incomplete).

Origin: part of the back-to-back C01 research pipeline the user asked for on 28 September 2026, taken from the
certified-country gap ledger (`docs/campaign-certification/C01/gap-ledger/ledger.md`) items `sa_prime_minister#sa_pm`. Pending Codex
acceptance; not registered in the task queue.

Branch: `claude/c01-sa-50`. Base: `5ea4f8fc` (current `codex/campaign-certification`); not stacked on a pending packet. Claim commit: this record's first commit on the branch.

## Bounded deliverable

Add holder observations to the existing role sa_pm (institution sa_prime_minister; no dict holders yet) from 1 January 1990 to the cutoff: the kings as Prime Minister (Fahd, Abdullah, Salman; their styling as رئيس مجلس الوزراء in Council of Ministers records and royal orders) and Mohammed bin Salman from the royal order of 27 September 2022 — at most ten people. Royal orders that appoint or reconstitute the Council of Ministers give `from` only where they state the effective day; Council of Ministers sessions chaired by the PM give attested_on. Keep existing holder entries unchanged. SPA and Umm al-Qura (Arabic, quoted as printed; Hijri and Gregorian dates as printed) are primary; prefer raw Internet Archive captures made before 2026-09-07.

Keep each distinct event (election or selection, appointment, assumption of office, acting or interim service,
resignation, removal, death, merger, renaming) as its own dated claim; acting service is claims only. Never infer an
end from a successor's start. Give a holder `from` or `until` only where a source states the day; otherwise record
`attested_on`. Party office and state office stay separate both ways. Organization identities, lifecycles and game
mappings stay unresolved; a name match to a simulation row is never a mapping. At most ten people. Primary sources
only; news and encyclopaedias are leads. The historical cutoff stays 7 September 2026.

## Allowed files and checks

Allowed files:

- this record;
- a new `docs/campaign-certification/C01/research/saudi-arabia-prime-ministers-1990-2026-50.md`;
- `docs/campaign-certification/C01/research/saudi-arabia.json` and specifically related new
  `docs/campaign-certification/C01/research/sources/saudi-arabia-*-facts.json` extracts;
- focused tests under `tools/avatars/`: a new `test_saudi_prime_ministers_c01_50.py`, and pinned counts or exact sets in `test_saudi_executive_c01_06.py`, `test_saudi_shura_allegiance_c01_25.py`, `test_saudi_kings_crown_princes_c01_45.py` updated to
  the new totals (none loosened).

Put generated `research-index.json` changes in a separate commit. Do not change the gap ledger, shared UI, the
roadmap, game data or other country packets.

Checks: research-index `--check`; `campaign_census.py --check`; the SaudiArabia, research and campaign Python tests
(census included); the atlas Node check; `workboard.py --check`; `git diff --check`.

Mark the packet `ready_for_review` when done. C01 and all parent gates stay open.
