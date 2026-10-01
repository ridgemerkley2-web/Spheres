# CLAUDE-C01-40: CPI(M) general secretaries, 1990–2026

## Integration amendment, 1 October 2026

The original submission below was accepted with corrections at `6ee593de`.
Claude subsequently submitted `5054de7b` / `d7e1b9d2`: two additional official
records, three newly-elected styling classifications and two revised selected
observations. See the [independent amendment review](../../campaign-certification/C01/reviews/CLAUDE-C01-40-amendment-20261001/README.md).
Earlier claims and corrected citation anchors remain intact. No effective starts,
runtime mappings, portraits or parent-gate completion are granted. The submitted
attribution of policy rulings to Ridge is not treated as verified authorization.
The historical original submission record follows.

Owner: Claude. State: **ready_for_review** (1 October 2026 UTC; submitted, not accepted). Parent: C01 (incomplete).
Report: [india-cpim-general-secretaries-1990-2026-40.md](../../campaign-certification/C01/research/india-cpim-general-secretaries-1990-2026-40.md).

Origin: part of the back-to-back C01 research pipeline the user asked for on 28 September 2026, taken from the
certified-country gap ledger (`docs/campaign-certification/C01/gap-ledger/ledger.md`) items `India/in_cpm`. Pending Codex
acceptance; not registered in the task queue.

Branch: `claude/c01-in-40`. Base: `02d2c5a2` (current `codex/campaign-certification`); not stacked on a pending packet. Claim commit: this record's first commit on the branch.

## Bounded deliverable

Add one party role, in_cpm_general_secretary (kind party_leader), to the existing ECI 2024 national-party observation in_eci_20240323_np_04 (Communist Party of India (Marxist)). Research the General Secretaries from 1 January 1990 to the cutoff: expected E. M. S. Namboodiripad, Harkishan Singh Surjeet, Prakash Karat, Sitaram Yechury (died 12 September 2024), the interim coordinator arrangement after his death (claims only) and M. A. Baby (elected April 2025) — at most ten people. Each Party Congress election is its own dated claim. The party's own records (cpim.org and archived pages, People's Democracy, Polit Bureau statements) are primary; ECI records only where they record the party office; news are leads.

Keep each distinct event (election or selection, appointment, assumption of office, acting or interim service,
resignation, removal, death, merger, renaming) as its own dated claim; acting service is claims only. Never infer an
end from a successor's start. Give a holder `from` or `until` only where a source states the day; otherwise record
`attested_on`. Party office and state office stay separate both ways. Organization identities, lifecycles and game
mappings stay unresolved; a name match to a simulation row is never a mapping. At most ten people. Primary sources
only; news and encyclopaedias are leads. The historical cutoff stays 7 September 2026.

## Allowed files and checks

Allowed files:

- this record;
- a new `docs/campaign-certification/C01/research/india-cpim-general-secretaries-1990-2026-40.md`;
- `docs/campaign-certification/C01/research/india.json` and specifically related new
  `docs/campaign-certification/C01/research/sources/india-*-facts.json` extracts;
- focused tests under `tools/avatars/`: a new `test_india_cpim_general_secretaries_c01_40.py`, and pinned counts or exact sets in `test_india_research_s10e.py`, `test_india_prime_ministers_c01_11.py`, `test_india_presidents_c01_15.py`, `test_india_inc_presidents_c01_20.py`, `test_india_bjp_presidents_c01_27.py`, `test_india_janata_dal_presidents_c01_33.py` updated to
  the new totals (none loosened).

Put generated `research-index.json` changes in a separate commit. Do not change the gap ledger, shared UI, the
roadmap, game data or other country packets.

Checks: research-index `--check`; `campaign_census.py --check`; the India, research and campaign Python tests
(census included); the atlas Node check; `workboard.py --check`; `git diff --check`.

Mark the packet `ready_for_review` when done. C01 and all parent gates stay open.

## Result

Submitted `ready_for_review` on 1 October 2026 (UTC). Commits on `claude/c01-in-40` (not stacked): the claim commit
`c152e35e` (this record only) on base `02d2c5a2`; `c70c10eb` `Add CLAUDE-C01-40: CPI(M) general secretaries, 1990–2026`
(everything below except the index); `04019fb3` `Regenerate the C01 research index for CLAUDE-C01-40`
(`research-index.json` only); the merge `14250447` of the moved integration `79ef97ec` (no conflict; the index needed no
change); and a last commit recording that merge in this record and the report. The base is now
`codex/campaign-certification` at `79ef97ec`.
Touched paths (nothing else):

- `docs/planning/ai-handoffs/CLAUDE-C01-40.md` (this record);
- `docs/campaign-certification/C01/research/india-cpim-general-secretaries-1990-2026-40.md` (new report);
- `docs/campaign-certification/C01/research/india.json` (additions only: 18 sources after the C01-33 sources; one role,
  `in_cpm_general_secretary`, on the existing observation `in_eci_20240323_np_04`, with the observation's sources, claims
  and one coverage note appended after its existing entries; one note at the end of the packet coverage);
- 18 new extracts `docs/campaign-certification/C01/research/sources/india-cpim-*-facts.json` and
  `india-pd-*-facts.json` (no existing extract edited);
- `tools/avatars/test_india_cpim_general_secretaries_c01_40.py` (new), and `tools/avatars/test_india_research_s10e.py`,
  `test_india_prime_ministers_c01_11.py`, `test_india_presidents_c01_15.py`, `test_india_inc_presidents_c01_20.py`,
  `test_india_bjp_presidents_c01_27.py` and `test_india_janata_dal_presidents_c01_33.py` (pinned counts, exact sets,
  source order, party-leader list, coverage-note positions, hosts and access dates re-expressed exactly; none loosened and
  no assertion removed);
- `docs/campaign-certification/C01/research-index.json` (regenerated, separate commit; the only file shared with the
  parallel packets CLAUDE-C01-38, -39 and -41).

The role has four holder observations from the party's own records (its website and its weekly organ People's
Democracy), none with a stated start: Harkishan Singh Surjeet (observed 2 March 2004), Prakash Karat (11 April 2005),
Sitaram Yechury (19 April 2015, until 12 September 2024 from the Polit Bureau's statement naming the office and the day of
death) and M. A. Baby (12 May 2025). EMS Namboodiripad has no in-office attestation in the period (one retrospective claim).
The interim coordinator arrangement (Prakash Karat, decided 29 September 2024, 'Coordinator' on 2 April 2025) is claims
only. Party Congress elections are separate claims: 11 October 1998 (recalled), the 18th Congress (no day), 19 April 2015
(names nobody) with the undated Polit Bureau list, 22 April 2018, 10 April 2022 and 6 April 2025. 18 sources, 24 claims;
ten raw Internet Archive captures and eight WordPress REST records of the party website's posts, each downloaded at least
twice 30 or more minutes apart with identical bytes.

Decisions for a ruling: (1) whether a Central Committee election recorded on its day should give `from` (kept as a claim,
as in CLAUDE-C01-27); (2) whether the party website's WordPress REST record is an acceptable identity where the HTML page
differs per request; (3) whether a 'newly elected general secretary' styling at the rally closing a Congress is an
in-office attestation of that day (used for Karat and Yechury).

Integration notes: the user chose to start this batch (CLAUDE-C01-38 to -41) before Codex's
"continue existing claims first" roadmap line; `research-index.json` must be regenerated after merging any of the
parallel packets;
`test_certified_gap_ledger.py` reports 'no pinned attribution' until Codex classifies these commits, and
`test_certified_boundary_matrix.py` (S23) needs `spheres-web/src`, absent from the sparse checkout; both are disclosed, not
fixed. C01 and every parent gate stay open.

Checks (1 October 2026, before committing on `02d2c5a2`, rerun after merging `79ef97ec`): research-index regenerate and
`--check` passed (9 country packets, 1,904 sources, 4,751 claims); `campaign_census.py --check` passed on `02d2c5a2` but
fails after the merge, inherited from integration ('C01 evidence differs: census.json': integration commits `434abd50`
and `7c6f112c` changed `spheres-sim/src/government.rs` without regenerating `census.json`; outside this packet's
boundary, not fixed); `test_india*.py` 67 passed (the new
test's 10 included), `test_*research*.py` 79 passed, `test_campaign*.py` 16 passed; the atlas Node check 11 passed;
`workboard.py --check` passed; `git diff --check` clean; `packet_check.py 40 --no-tests` re-downloaded all 18 sources and
every one matched. The full `packet_check.py 40` is run again after pushing.
