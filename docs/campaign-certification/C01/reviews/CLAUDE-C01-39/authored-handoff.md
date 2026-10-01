# CLAUDE-C01-39: Democratic Alliance federal leaders, 2000–2026

Owner: Claude. State: **ready_for_review** (30 September 2026; submitted, not accepted). Parent: C01 (incomplete).
Report: [south-africa-da-federal-leaders-2000-2026-39.md](../../campaign-certification/C01/research/south-africa-da-federal-leaders-2000-2026-39.md).

Origin: part of the back-to-back C01 research pipeline the user asked for on 28 September 2026, taken from the
certified-country gap ledger (`docs/campaign-certification/C01/gap-ledger/ledger.md`) items `za_iec_n2024_027#za_da_federal_leader`, `SouthAfrica/za_dp`. Pending Codex
acceptance; not registered in the task queue.

Branch: `claude/c01-za-39`. Base: `02d2c5a2` (then current `codex/campaign-certification`), merged with the moved integration tips `79ef97ec` (merge `0183c1e1`) and `f3e18e83` (merge `b0b9006c`); not stacked on a pending packet. Claim commit: this record's first commit on the branch (`03cd0bb9`).

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

## Result

Submitted `ready_for_review` on 30 September 2026. Touched paths (nothing else):

- `docs/planning/ai-handoffs/CLAUDE-C01-39.md` (this record);
- `docs/campaign-certification/C01/research/south-africa-da-federal-leaders-2000-2026-39.md` (new report);
- `docs/campaign-certification/C01/research/south-africa.json`;
- 24 new extracts, `docs/campaign-certification/C01/research/sources/south-africa-da-*-facts.json` (21) and
  `south-africa-dp-*-facts.json` (3); no existing extract is edited;
- `tools/avatars/test_south_africa_da_federal_leaders_c01_39.py` (new), and pinned values in
  `tools/avatars/test_south_africa_research_s10h.py`, `tools/avatars/test_south_africa_heads_of_state_c01_09.py`,
  `tools/avatars/test_south_africa_anc_presidents_c01_16.py`, `tools/avatars/test_south_africa_deputy_presidents_c01_21.py`,
  `tools/avatars/test_south_africa_party_leaders_c01_30.py` and `tools/avatars/test_south_africa_pac_presidents_c01_32.py`;
- `docs/campaign-certification/C01/research-index.json` (regenerated, separate commit; the only file shared with the
  parallel packets CLAUDE-C01-38, 40 and 41).

The packet gains 24 sources and 29 claims and no role: it extends the existing `party_leader` role
`za_da_federal_leader` on the DEMOCRATIC ALLIANCE observation (`za_iec_n2024_027`). The S10h holders (John Steenhuisen
2023-04-03, Geordin Hill-Lewis 2026-04-12) are unchanged, object for object; eleven holder observations of four people are
added, each dated by a DA in-office attestation, with no start and no end, because no record found states the day any
Leader assumed or left the office. In date order (13 observations, five people):

- Tony Leon: 2000-10-14, 2000-11-22 (DA speech pages, "Position Leader of the Democratic Alliance").
- Helen Zille: 2010-10-15, 2012-11-04, 2014-05-09 (DA newsroom bylines "Helen Zille, Leader of the Democratic Alliance").
- Mmusi Maimane: 2016-04-25, 2018-04-07, 2018-04-08, 2019-10-04 (DA posts and statements).
- John Steenhuisen: 2021-03-10, 2023-04-03 (S10h, unchanged), 2026-03-04 (DA statements issued by him as Leader).
- Geordin Hill-Lewis: 2026-04-12 (S10h, unchanged).

Decisions:

- **ZA-DA-01 and ZA-DA-02, claims only:** the Democratic Party (1989-2000) has no observation. Its formation ("April 8,
  1989", an undated retrospective page, stored undated), its leaders (Zach de Beer, retrospective; Tony Leon on 22 May
  1997, in an undated page and "In 1994", year only), the 2000 DP/NNP alliance ("Mr Leon lei die DP en ek lei die NNP in
  die DA in", 14 October 2000) and the DA's formation by the DP, NNP and Federal Alliance (undated) sit on the DA
  observation with `role_id` null: never holders, starts, ends, merged identities or predecessor links.
- **Elections and results never feed a holder:** Zille's acceptance speech of 6 May 2007, the results of 10 May 2015 and
  1 November 2020 and the congress span of 9 and 10 May 2015 are claims. The S10h Hill-Lewis observation, which rests on a
  result announcement, is left as recorded.
- **Interim service is claims only:** the interim election set for 17 November 2019 and Steenhuisen's congratulation as
  "Interim Federal Leader" (23 November 2019) never feed a holder; the interim title is kept as its own `role_title`.
- **Ruling requested (October 2019):** the Federal Council Chairperson's statements of 24 and 25 October 2019 date
  Maimane's decision to resign and the vacancy only "on Wednesday"; no calendar day of an accepted or effective
  resignation is printed, so no `until` is given. Codex: does a vacancy stated by weekday in a dated party statement give
  `until` on that weekday?
- **Never an end:** the "Former Leader" styling of 2007 (undated) and the tribute of 4 February 2026 ("six years of
  determined leadership as Federal Leader"); the DA styles Steenhuisen its Leader again on 4 March 2026.
- **Separation:** "MP", the "Leader of the Official Opposition" heading label of 2007, the "Parliamentary Leader" styling
  of 2016, the President of the Republic (2012), the 2014 national election and the ministry named in 2026 are never
  used; the Federal Chairperson, Chairperson of the Federal Council, Federal Finance Chairperson, interim Federal
  Chairperson and the 2000 Deputy Leader are other offices. No claim or source is shared with any other role.
- **Declared edit:** the DA role's scope note is rewritten (it said "Two discrete attestations"); the role's and the
  organization's `sources` and `claim_ids` are extended at the end. No existing holder, claim, source or extract changes.
- **Sources:** 21 DA pages (www.da.org.za, 2000-2026) and 3 Democratic Party pages (www.dp.org.za, 1999-2000 captures),
  all raw Internet Archive captures made before the cutoff, each downloaded twice at least 30 minutes apart with the same
  identity and matching the archive's CDX digest. News, Wikipedia and history sites are leads only.

The packet-level coverage note is inserted after the CLAUDE-C01-32 note and before the CLAUDE-C01-21 and CLAUDE-C01-16
notes, which their tests pin as the last two entries.

Integration: the branch is not stacked. Before committing, `codex/campaign-certification` had moved from `02d2c5a2` to
`79ef97ec` (simulation, campaign-matrix and planning records; no research file) and then to `f3e18e83` (campaign-leader
art, selector checks and a regenerated `census.json`; no research file); both were merged without conflict (merges
`0183c1e1` and `b0b9006c`). The user chose to start this batch (CLAUDE-C01-38 to 41) before Codex's roadmap line
"continue existing claims first"; this packet was started on the user's instruction. The claim is not registered in the task queue; that is
left for Codex. New South Africa totals: 269 sources, 500 claims, 11 role observations; `mapping_pending` stays 53 and
the six discovery batches stay open. `research-index.json` (separate commit) is the only file shared with CLAUDE-C01-38,
40 and 41: regenerate it when integrating.

## Checks

Run from the worktree with `PYTHONDONTWRITEBYTECODE=1` and `python -X utf8`:

- `tools/avatars/campaign_research.py` (regenerated), then `--check`: passed. 9 country packets, 1,910 sources, 4,756
  claims, 93 discovery batches; South Africa 269 sources, 500 claims, 11 role observations.
- `tools/avatars/campaign_census.py --check`: passed after the merge of `f3e18e83`, which regenerated `census.json` (it
  had failed between the two merges because `spheres-sim/src/government.rs` changed at `79ef97ec` without a census
  regeneration; this packet touches no census input).
- `-m unittest discover -s tools/avatars -p "test_south_africa*.py"`: 58 tests OK (the new test's 8 included, with its 7
  validator and 33 invariant mutation cases all rejected).
- `-p "test_*research*.py"`: 79 tests OK. `-p "test_campaign*.py"`: 16 tests OK (including `test_campaign_census`).
- `node --test tools/ui/check_leadership_research_review.cjs`: 11 passed.
- `python tools/planning/workboard.py --check`: PASS.
- `git diff --check`: clean. The gap ledger was not touched.
- `packet_check.py 39`: run on the final tree before commit (`0183c1e1` plus the working tree): 24 new sources re-downloaded, 23 matching their recorded identities and one rate-limited (HTTP 429), which was re-downloaded by hand at 00:58Z with the same 145,135 bytes and SHA-256; every suite check passed except `census --check`, which the later merge of `f3e18e83` fixed (see above). It is run again after the push; that run's summary line is returned with this submission.
- Known failures outside the listed checks, not fixed: `tools/avatars/test_certified_gap_ledger.py` reports "no pinned
  attribution" for this packet's sources until Codex classifies its commit in `COMMIT_PACKETS`;
  `tools/avatars/test_certified_boundary_matrix.py` (S23) needs `spheres-web/src`, which the sparse checkout lacks, and
  Codex regenerates `docs/campaign-certification/S23/preparation/boundary-matrix/` on integration.
