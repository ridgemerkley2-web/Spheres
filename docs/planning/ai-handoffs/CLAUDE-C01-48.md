# CLAUDE-C01-48: BSP, AAP and NPP national leaders, 1990–2026

Owner: Claude. State: **ready_for_review** (1 October 2026 UTC; submitted, not accepted). Parent: C01 (incomplete).
Report: [india-bsp-aap-npp-leaders-1990-2026-48.md](../../campaign-certification/C01/research/india-bsp-aap-npp-leaders-1990-2026-48.md).

Origin: part of the back-to-back C01 research pipeline the user asked for on 28 September 2026, taken from the
certified-country gap ledger (`docs/campaign-certification/C01/gap-ledger/ledger.md`) items `in_eci_20240323_np_02`, `in_eci_20240323_np_01`, `in_eci_20240323_np_06`. Pending Codex
acceptance; not registered in the task queue.

Branch: `claude/c01-in-48`. Base: `5ea4f8fc` (current `codex/campaign-certification`); not stacked on a pending packet. Claim commit: this record's first commit on the branch.

## Bounded deliverable

Add one party_leader role to each of three existing ECI 2024 national-party observations: in_bsp_national_president on in_eci_20240323_np_02 (Bahujan Samaj Party; expected Kanshi Ram, Mayawati), in_aap_national_convenor on in_eci_20240323_np_01 (Aam Aadmi Party; expected Arvind Kejriwal) and in_npp_national_president on in_eci_20240323_np_06 (National People's Party; expected P. A. Sangma, Conrad Sangma) — from each party's founding (or 1 January 1990) to the cutoff, at most ten people. Elections, assumptions, deaths in office (only where the office and the day are named) and acting arrangements (claims only) are distinct dated claims. Party records (official sites and archived pages) and ECI records where they record the party office are primary; news are leads. india.json is 1.2 MB: read it only through tools/slice.py.

Keep each distinct event (election or selection, appointment, assumption of office, acting or interim service,
resignation, removal, death, merger, renaming) as its own dated claim; acting service is claims only. Never infer an
end from a successor's start. Give a holder `from` or `until` only where a source states the day; otherwise record
`attested_on`. Party office and state office stay separate both ways. Organization identities, lifecycles and game
mappings stay unresolved; a name match to a simulation row is never a mapping. At most ten people. Primary sources
only; news and encyclopaedias are leads. The historical cutoff stays 7 September 2026.

## Allowed files and checks

Allowed files:

- this record;
- a new `docs/campaign-certification/C01/research/india-bsp-aap-npp-leaders-1990-2026-48.md`;
- `docs/campaign-certification/C01/research/india.json` and specifically related new
  `docs/campaign-certification/C01/research/sources/india-*-facts.json` extracts;
- focused tests under `tools/avatars/`: a new `test_india_bsp_aap_npp_leaders_c01_48.py`, and pinned counts or exact sets in `test_india_research_s10e.py`, `test_india_prime_ministers_c01_11.py`, `test_india_presidents_c01_15.py`, `test_india_inc_presidents_c01_20.py`, `test_india_bjp_presidents_c01_27.py`, `test_india_janata_dal_presidents_c01_33.py`, `test_india_cpim_general_secretaries_c01_40.py` updated to
  the new totals (none loosened).

Put generated `research-index.json` changes in a separate commit. Do not change the gap ledger, shared UI, the
roadmap, game data or other country packets.

Checks: research-index `--check`; `campaign_census.py --check`; the India, research and campaign Python tests
(census included); the atlas Node check; `workboard.py --check`; `git diff --check`.

Mark the packet `ready_for_review` when done. C01 and all parent gates stay open.

## Result

Submitted `ready_for_review` on 1 October 2026 (UTC). Commits on `claude/c01-in-48` (not stacked): the claim commit
`e2623e3e` (this record only) on base `5ea4f8fc`; `Add CLAUDE-C01-48: BSP, AAP and NPP national leaders, 1990–2026`
(everything below except the index); and `Regenerate the C01 research index for CLAUDE-C01-48` (`research-index.json`
only). `codex/campaign-certification` had not moved from `5ea4f8fc` when the packet was committed, so no merge was needed.
Touched paths (nothing else):

- `docs/planning/ai-handoffs/CLAUDE-C01-48.md` (this record);
- `docs/campaign-certification/C01/research/india-bsp-aap-npp-leaders-1990-2026-48.md` (new report);
- `docs/campaign-certification/C01/research/india.json` (additions only: 19 sources after the C01-40 sources; one role on
  each of the existing observations `in_eci_20240323_np_01` (`in_aap_national_convenor`), `in_eci_20240323_np_02`
  (`in_bsp_national_president`) and `in_eci_20240323_np_06` (`in_npp_national_president`), with each observation's
  sources, claims and one coverage note appended after its existing entries; one note at the end of the packet coverage);
- 19 new extracts `docs/campaign-certification/C01/research/sources/india-{aap,bsp,npp,eci}-*-facts.json` (no existing
  extract edited);
- `tools/avatars/test_india_bsp_aap_npp_leaders_c01_48.py` (new), and the pinned tests `test_india_research_s10e.py`,
  `test_india_prime_ministers_c01_11.py`, `test_india_presidents_c01_15.py`, `test_india_inc_presidents_c01_20.py`,
  `test_india_bjp_presidents_c01_27.py`, `test_india_janata_dal_presidents_c01_33.py` and
  `test_india_cpim_general_secretaries_c01_40.py` (totals, source order, the party-leader list, coverage-note positions,
  hosts and access dates re-expressed exactly; none loosened and no assertion removed);
- `docs/campaign-certification/C01/research-index.json` (regenerated, separate commit; the only file shared with the
  parallel C01 packets: on a conflict, continue existing claims first and regenerate it from the merged inputs).

Holders: Arvind Kejriwal (observed 7 October 2013; a second entry from 27 April 2016, the effective day stated in the
National Executive's certified minutes filed with the Election Commission), Mayawati (observed 16 July 2012, the party's
signed letter to the Chief Election Commissioner) and Conrad K. Sangma (observed 11 July 2020, the party's press release).
No end is stated. Kanshi Ram and Purno Agitok Sangma, the founders, are claims only. 19 sources (16 raw Internet Archive
captures, 3 WordPress REST records of the AAP's own sites), 24 claims, each source downloaded at least twice 30 or more
minutes apart with identical bytes and SHA-256.

Decisions for Codex (detailed in the report): (1) the AAP minutes' 'W.E.F. 27.04.2016' used as `from`; (2) the 2016
term as a second holder entry for the same person; (3) the BSP profile's retrospective 'Assumed the office' entry of
18 September 2003 kept as a claim, not a start; (4) Kanshi Ram's death recalled as 'founder President' not treated as a
death in office; (5) Election Commission catalogue entries naming the office kept as continuation claims; (6) party
letters published by the Commission treated as the parties' own records; (7) the NPP press release of 11 July 2020
selected over the signed notification of 4 February 2022; (8) the AAP NRI-wing release treated as a party record; (9) the
BSP press note read from its legacy-font text layer.

Checks: research-index regenerate and `--check` pass; `campaign_census.py --check` passes; `test_india*.py` 84 OK, `test_*research*.py` 79 OK, `test_campaign*.py` 16 OK; the atlas Node check 11/11; `git diff --check` clean; `workboard.py --check` exits 1 only on 'Missing task handoff: docs/campaign-certification/S26/preparation/RECRUITMENT.md', a file outside this worktree's sparse checkout (same on the base here). `packet_check.py 48` runs after the commits.

Known failures outside the suite (disclosed, not fixed): `test_certified_gap_ledger.py` reports no pinned attribution
for this packet until Codex classifies its commit; `test_certified_boundary_matrix.py` needs `spheres-web/src`, which
the sparse checkout lacks. C01 and every parent gate stay open.
