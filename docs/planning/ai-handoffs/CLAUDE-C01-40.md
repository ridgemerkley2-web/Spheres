# CLAUDE-C01-40: CPI(M) general secretaries, 1990–2026

Owner: Claude. State: **ready_for_review** (1 October 2026 UTC; resubmitted with the checker fixes, not accepted as fixed). Parent: C01 (incomplete).
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

Submitted `ready_for_review` on 1 October 2026 (UTC) at `54680e39` and resubmitted `ready_for_review` the same day with
the checker fixes. Commits on `claude/c01-in-40` (not stacked): the claim commit `c152e35e` (this record only) on base
`02d2c5a2`; `c70c10eb` `Add CLAUDE-C01-40: CPI(M) general secretaries, 1990–2026` (everything below except the index);
`04019fb3` `Regenerate the C01 research index for CLAUDE-C01-40` (`research-index.json` only); the merge `14250447` of
the moved integration `79ef97ec` (no conflict; the index needed no change); `54680e39`, recording that merge here and in
the report; `Apply checker fixes to CLAUDE-C01-40` (everything the fixes touch except the index); and `Regenerate the
C01 research index for CLAUDE-C01-40 checker fixes` (`research-index.json` only). The base stays
`codex/campaign-certification` at `79ef97ec`; the integration has since moved to `509bd289`, but merging it conflicts
outside the index, so it was not merged (see the integration notes).
Touched paths (nothing else):

- `docs/planning/ai-handoffs/CLAUDE-C01-40.md` (this record);
- `docs/campaign-certification/C01/research/india-cpim-general-secretaries-1990-2026-40.md` (new report);
- `docs/campaign-certification/C01/research/india.json` (additions only: 20 sources after the C01-33 sources; one role,
  `in_cpm_general_secretary`, on the existing observation `in_eci_20240323_np_04`, with the observation's sources,
  claims and one coverage note appended after its existing entries; one note at the end of the packet coverage);
- 20 new extracts `docs/campaign-certification/C01/research/sources/india-cpim-*-facts.json` and `india-pd-*-facts.json`
  (no extract of another packet edited; the checker fixes re-kind one row in each of three of this packet's own
  extracts);
- `tools/avatars/test_india_cpim_general_secretaries_c01_40.py` (new), and `tools/avatars/test_india_research_s10e.py`,
  `test_india_prime_ministers_c01_11.py`, `test_india_presidents_c01_15.py`, `test_india_inc_presidents_c01_20.py`,
  `test_india_bjp_presidents_c01_27.py` and `test_india_janata_dal_presidents_c01_33.py` (pinned counts, now 20 sources
  and 26 claims for this role, exact sets, source order, party-leader list, coverage-note positions, hosts and access
  dates re-expressed exactly; none loosened and no assertion removed; the access dates are now also asserted as an exact
  set);
- `docs/campaign-certification/C01/research-index.json` (regenerated, separate commits; the only file shared with the
  parallel packets CLAUDE-C01-38, -39 and -41).

The role has four holder observations from the party's own records (its website and its weekly organ People's
Democracy), none with a stated start: Harkishan Singh Surjeet (observed 2 March 2004, his letter of that day), Prakash
Karat (10 September 2011, the party's release of his note to the National Integration Council 'today'), Sitaram Yechury
(6 May 2015, his letter of that day signed as General Secretary and released on 8 May; until 12 September 2024 from the
Polit Bureau's statement naming the office and the day of death) and M. A. Baby (12 May 2025). EMS Namboodiripad has no
in-office attestation in the period (one retrospective claim). The 'newly elected general secretary' stylings at the
rallies closing the 18th (11 April 2005), 20th (9 April 2012) and 21st (19 April 2015) Congresses are election claims.
The interim coordinator arrangement (Prakash Karat, decided 29 September 2024, 'Coordinator' on 2 April 2025) is claims
only. Party Congress elections are separate claims: 11 October 1998 (recalled), the 18th Congress (no day), 19 April
2015 (names nobody) with the undated Polit Bureau list, 22 April 2018, 10 April 2022 and 6 April 2025. 20 sources, 26
claims; ten raw Internet Archive captures and ten WordPress REST records of the party website's posts, each downloaded
at least twice 30 or more minutes apart with identical bytes.

Decisions (rulings decided by Ridge on 1 October 2026 after the checker's review; Codex may still decide otherwise at
integration):

- (a) A 'newly elected general secretary' styling on or for the election is an election claim, never a holder date and
  never a start (the accepted CLAUDE-C01-27 precedent: Rajnath Singh's 'Newly Elected President' of 23 January 2013).
  Karat's observation of 11 April 2005 and Yechury's of 19 April 2015 are demoted to claims, and Karat's 'newly elected
  general secretary' of 9 April 2012 is re-kinded with them (`newly_elected_styling`). Karat is observed instead on 10
  September 2011 (cpim.org post 1411, the earliest party record found whose printed dateline agrees with its publication
  date); his styling as General Secretary at the inauguration of the 21st Congress on 14 April 2015 qualifies too but is
  later, so it stays a later attestation. Yechury is observed on 6 May 2015 from the lead post 4346: the release of 8
  May 2015 names him General Secretary on its dateline and prints his letter of 6 May signed '(Sitaram Yechury) General
  Secretary', and the observation takes the letter's own date, as written (CLAUDE-C01-35), as Surjeet's takes the day of
  his letter; his `until` of 12 September 2024 from the Polit Bureau statement (post 11608, which names the office and
  the day of death; CLAUDE-C01-34) is unchanged.
- (b) A Central Committee election recorded on its day gives no `from` (CLAUDE-C01-27; CLAUDE-C01-36 Sovaleni): the
  elections of 22 April 2018, 10 April 2022 and 6 April 2025 stay claims.
- (c) The party website's WordPress REST record of a post is accepted as the identity, with the post's publication date
  (never its modification date) as the dateline; Codex should prefer a pre-cutoff raw capture of the same JSON if one
  exists and should re-verify the live records.
- (d) The 1996 HRD resolution naming 'Shri Harkishan Singh Surjeet, Genl.Secy, CPI (M)' stays a lead only because
  importing it would edit an existing CLAUDE-C01-33 extract; the earlier note that it would be 'a claim in any case' is
  corrected (the entry names the party and the office), and it is listed as next work.

Integration notes: the user chose to start this batch (CLAUDE-C01-38 to -41) before Codex's
"continue existing claims first" roadmap line; `research-index.json` must be regenerated after merging any of the
parallel packets. After this packet was submitted at `54680e39`, `codex/campaign-certification` (now `509bd289`)
imported it (`5d5935c3`), corrected five claim locators in four extracts and the report's opening-era and REST wording
(`d21357db`, with `test_india_cpim_original_review_c01_40.py`), and accepted it as a bounded intake with the original
holders, Karat observed on 11 April 2005 and Yechury on 19 April 2015 (`6ee593de`,
`docs/campaign-certification/C01/integrations/CLAUDE-C01-40/`). Merging that integration into this branch conflicts in
`india.json`, the report and four extracts (content and add/add conflicts against those corrected copies) besides the
index, so it was not merged: the checker fixes are committed on top of the exact submission Codex reviewed. The
integrator needs to carry the fix commit onto its corrected copy (two new sources and claims, three re-kinded rows in
`india-pd-rally-18th-congress-20050417`, `india-pd-rally-20th-congress-20120415` and
`india-pd-join-to-bring-forth-change-20150426`, the last two of which Codex also corrected, the two changed holders, the
notes and the pinned counts) and to decide whether rulings (a)-(d) replace the holder observations it accepted.
`test_certified_gap_ledger.py` reports 'no pinned attribution' until Codex classifies these commits, and
`test_certified_boundary_matrix.py` (S23) needs `spheres-web/src`, absent from the sparse checkout; both are disclosed,
not fixed. `campaign_census.py --check` fails on this branch with 'C01 evidence differs: census.json', inherited, not
fixed: integration commits after `262d5f61` (`ace1f233`, `434abd50`, `7c6f112c`) changed `spheres-sim/src/government.rs`
(849,546 to 851,150 bytes) without regenerating `docs/campaign-certification/C01/census.json`, and the check fails on
the integration head this branch is based on (`79ef97ec`) itself; this packet touches neither file. Codex reports the
check passing on its later integration base. C01 and every parent gate stay open.

Checks (1 October 2026, after the checker fixes, on this branch): research-index regenerate and `--check` passed (9
country packets, 844 organization and 36 institution observations, 1,906 sources, 4,753 claims, 93 open discovery
batches); `campaign_census.py --check` fails, inherited as above; `test_india*.py` 67 passed (the new test's 10
included), `test_*research*.py` 79 passed, `test_campaign*.py` 16 passed; the atlas Node check 11 passed; `workboard.py
--check` passed; `git diff --check` clean; `packet_check.py 40 --no-tests` with fetching, before committing: 20 of 20
sources match their recorded byte count and SHA-256 (downloaded again at 01:56-01:57Z on 1 October 2026); the only
problem reported was the then-uncommitted worktree, and the full run is repeated after pushing.
