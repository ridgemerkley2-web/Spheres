# C01-39 integration status, 1 October 2026

The original claim below is preserved. The latest delivery is **ready_for_review**
at `9cb02c20012a6e6314d7e64d52241609d566f738`; independent Codex source/content
review is pending. The task is now registered in `docs/planning/ai-task-queue.json`.
Only this claim/status record is integrated. No South Africa research or tests
from the new packet are imported or accepted yet.

See the [authored delivery and reported checks](https://github.com/ridgemerkley2-web/Spheres/blob/9cb02c20012a6e6314d7e64d52241609d566f738/docs/planning/ai-handoffs/CLAUDE-C01-39.md).
It reports 24 new sources, 29 claims and eleven added holder observations; those
counts are submission scope, not an independent historical acceptance.

---

# CLAUDE-C01-39: Democratic Alliance federal leaders, 2000–2026

Owner: Claude. State: **claimed** (2026-09-30; in progress, not complete). Parent: C01 (incomplete).

Origin: part of the back-to-back C01 research pipeline the user asked for on 28 September 2026, taken from the
certified-country gap ledger (`docs/campaign-certification/C01/gap-ledger/ledger.md`) items `za_iec_n2024_027#za_da_federal_leader`, `SouthAfrica/za_dp`. Pending Codex
acceptance; not registered in the task queue.

Branch: `claude/c01-za-39`. Base: `02d2c5a2` (current `codex/campaign-certification`); not stacked on a pending packet. Claim commit: this record's first commit on the branch.

## Bounded deliverable

Extend the existing role za_da_federal_leader on za_iec_n2024_027 (DEMOCRATIC ALLIANCE; current holders Steenhuisen 2023-04-03 and Hill-Lewis 2026-04-12) back to the DA's formation in 2000: expected Tony Leon, Helen Zille, Mmusi Maimane, John Steenhuisen (interim 2019, elected 2020) and Geordin Hill-Lewis — at most ten people. The Democratic Party (1989–2000; leaders Zach de Beer and Tony Leon), the 2000 DP/NNP alliance and the DA's formation are claims about the organizations only, never merged identities; interim leadership is claims only. The DA's own records (federal congress results, statements, archived da.org.za pages) are primary; IEC records only where they record the party office; news are leads.

Keep each distinct event (election or selection, appointment, assumption of office, acting or interim service,
resignation, removal, death, merger, renaming) as its own dated claim; acting service is claims only. Never infer an
end from a successor's start. Give a holder `from` or `until` only where a source states the day; otherwise record
`attested_on`. Party office and state office stay separate both ways. Organization identities, lifecycles and game
mappings stay unresolved; a name match to a simulation row is never a mapping. At most ten people. Primary sources
only; news and encyclopaedias are leads. The historical cutoff stays 7 September 2026.

## Allowed files and checks

Allowed files:

- this record;
- a new `docs/campaign-certification/C01/research/south-africa-da-federal-leaders-2000-2026-39.md`;
- `docs/campaign-certification/C01/research/south-africa.json` and specifically related new
  `docs/campaign-certification/C01/research/sources/south-africa-*-facts.json` extracts;
- focused tests under `tools/avatars/`: a new `test_south_africa_da_federal_leaders_c01_39.py`, and pinned counts or exact sets in `test_south_africa_research_s10h.py`, `test_south_africa_heads_of_state_c01_09.py`, `test_south_africa_anc_presidents_c01_16.py`, `test_south_africa_deputy_presidents_c01_21.py`, `test_south_africa_party_leaders_c01_30.py`, `test_south_africa_pac_presidents_c01_32.py` updated to
  the new totals (none loosened).

Put generated `research-index.json` changes in a separate commit. Do not change the gap ledger, shared UI, the
roadmap, game data or other country packets.

Checks: research-index `--check`; `campaign_census.py --check`; the SouthAfrica, research and campaign Python tests
(census included); the atlas Node check; `workboard.py --check`; `git diff --check`.

Mark the packet `ready_for_review` when done. C01 and all parent gates stay open.
