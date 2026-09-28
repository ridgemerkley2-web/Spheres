# CLAUDE-C01-32: Pan Africanist Congress presidents, 1990–2026

Owner: Claude. State: **claimed** (2026-09-28; in progress, not complete). Parent: C01 (incomplete).

Origin: part of the back-to-back C01 research pipeline the user asked for on 28 September 2026, taken from the
certified-country gap ledger (`docs/campaign-certification/C01/gap-ledger/ledger.md`) items `SouthAfrica/za_pac`. Pending Codex
acceptance; not registered in the task queue.

Branch: `claude/c01-za-32`. **Stacked on `claude/c01-za-30`** (a pending, unmerged packet that also edits `south-africa.json`), merged with current integration `032cd6a3`: merge that packet first. Claim commit: this record's first commit on the branch.

## Bounded deliverable

Add one party role, za_pac_president (President of the Pan Africanist Congress of Azania; kind party_leader), to the existing IEC observation za_iec_n2024_039. Research its presidents from 1 January 1990 to the cutoff — at most ten people (expected Clarence Makwetu, Stanley Mogoba, Motsoko Pheko, Letlapa Mphahlele, Alton Mphethi, Luthando Mbinda, Narius Moloto, Mzwanele Nyhontso). Leadership disputes and rival claimants, expulsions and court orders are distinct dated claims; a disputed presidency is recorded as disputed, never resolved by inference. Sources: the PAC's own records, IEC records, court judgments (SAFLII) and Parliament only where they record the party office.

Keep each distinct event (election or selection, appointment, assumption of office, acting or interim service,
resignation, removal, death, merger, renaming) as its own dated claim; acting service is claims only. Never infer an
end from a successor's start. Give a holder `from` or `until` only where a source states the day; otherwise record
`attested_on`. Party office and state office stay separate both ways. Organization identities, lifecycles and game
mappings stay unresolved; a name match to a simulation row is never a mapping. At most ten people. Primary sources
only; news and encyclopaedias are leads. The historical cutoff stays 7 September 2026.

## Allowed files and checks

Allowed files:

- this record;
- a new `docs/campaign-certification/C01/research/south-africa-pac-presidents-1990-2026-32.md`;
- `docs/campaign-certification/C01/research/south-africa.json` and specifically related new
  `docs/campaign-certification/C01/research/sources/south-africa-*-facts.json` extracts;
- focused tests under `tools/avatars/`: a new `test_south_africa_pac_presidents_c01_32.py`, and pinned counts or exact sets in `test_south_africa_research_s10h.py`, `test_south_africa_heads_of_state_c01_09.py`, `test_south_africa_anc_presidents_c01_16.py`, `test_south_africa_deputy_presidents_c01_21.py`, `test_south_africa_party_leaders_c01_30.py` updated to
  the new totals (none loosened).

Put generated `research-index.json` changes in a separate commit. Do not change the gap ledger, shared UI, the
roadmap, game data or other country packets.

Checks: research-index `--check`; `campaign_census.py --check`; the SouthAfrica, research and campaign Python tests
(census included); the atlas Node check; `workboard.py --check`; `git diff --check`.

Mark the packet `ready_for_review` when done. C01 and all parent gates stay open.
