# CLAUDE-C01-42: Research JCP chairs and DPFP representatives, 1990–2026

Owner: Claude. State: **ready_for_review** (1 October 2026; not complete, pending Codex acceptance). Registered by Codex on 1 October 2026.
Parent C01 remains incomplete. Branch: `claude/c01-jp-42`.

Exact claim: `601078190e76bfa3ae578a6b467073620dff1ce6`. Exact checked tip: `601078190e76bfa3ae578a6b467073620dff1ce6`.
The [authored claim/delivery](https://github.com/ridgemerkley2-web/Spheres/blob/601078190e76bfa3ae578a6b467073620dff1ce6/docs/planning/ai-handoffs/CLAUDE-C01-42.md)
is preserved on its own branch. Its assertions about user instructions are not independent authorization.

Existing Claude claim only. Preserve its three Japanese party-office targets and old holder records. C01-31 is now accepted; refresh from current integration without undoing its corrections. No C01-42 delivery or acceptance is recorded.

Keep the historical cutoff at 7 September 2026, separate party and state offices,
and preserve unknown dates. No runtime, portrait, country-cast or CP1 acceptance
is implied. Fetch current integration before a follow-up, use the existing claim,
and keep source recovery separate from the ready Tonga production assignment.

## Result (ready_for_review)

Claim commit `60107819` (this record only) on `861cd268`, the merge of the integration head `509bd289` into the then-pending
`claude/c01-jp-31` (CLAUDE-C01-31), on which this packet was stacked. Before the packet commits, `origin/codex/campaign-certification`
and `origin/claude/c01-jp-31` were fetched: the stacked branch had not moved, but integration had moved to `13367c99`, which
accepted CLAUDE-C01-31 with Codex's corrections and registered this record. It was merged (merge commit `c478d66e`). Every
conflict lay between the pre-review `claude/c01-jp-31` copy and the accepted integration copy of CLAUDE-C01-31's own files
(`japan.json`, its report, its test, four of its extracts, its handoff, `research-index.json` and the S10d test) or Codex's
registration of this record; none held content of this packet, so the integration copy was taken unchanged in each and the merged
tree equals `13367c99`. Ridge confirmed this merge (ruling (d) below). The packet is **no longer stacked**: Codex accepted
CLAUDE-C01-31 into integration (`13367c99`), which is the packet's base (ruling (e)), and `packet_check.py 42` is run with
`--base origin/codex/campaign-certification`. Then the packet commit and the research-index commit on `claude/c01-jp-42`,
followed by the checker-fix commit(s) under 'Checker fixes' below. Report:
[japan-jcp-chairs-dpfp-representatives-1990-2026-42.md](../../campaign-certification/C01/research/japan-jcp-chairs-dpfp-representatives-1990-2026-42.md).

The user chose to start this batch of research packets before Codex's roadmap line 'continue existing claims first'.

Touched paths (all inside this record's allowed files):

- `docs/planning/ai-handoffs/CLAUDE-C01-42.md` (this record: state line and this section);
- `docs/campaign-certification/C01/research/japan-jcp-chairs-dpfp-representatives-1990-2026-42.md` (new report);
- `docs/campaign-certification/C01/research/japan.json` (additions only: 32 sources and 50 claims appended after CLAUDE-C01-31's; on
  `jp_sangiin_pr_2025_01` and `jp_sangiin_pr_2025_07`, the existing roles `jp_jcp_executive_committee_chair`,
  `jp_jcp_central_committee_chair` and `jp_dpfp_representative` gain 19 holders placed in date order before the unchanged originals,
  appended `sources` and `claim_ids` and one appended scope-note sentence each; each organization's `sources` and `claim_ids` extended
  and one coverage note appended; one packet coverage note appended; no existing text changed);
- 32 new `docs/campaign-certification/C01/research/sources/japan-*-facts.json` extracts, one per new source (no existing extract
  edited);
- `tools/avatars/test_japan_jcp_dpfp_leaders_c01_42.py` (new);
- `tools/avatars/test_japan_research_s10d.py`, `tools/avatars/test_japan_prime_ministers_c01_12.py`,
  `tools/avatars/test_japan_prime_ministers_c01_13.py`, `tools/avatars/test_japan_ldp_presidents_c01_18.py`,
  `tools/avatars/test_japan_sdp_chairs_c01_29.py` and `tools/avatars/test_japan_komeito_representatives_c01_31.py` (pinned counts,
  exact source lists, holder lists, access dates and coverage-note positions re-expressed exactly; the original JCP and DPFP holders
  are pinned as the last of their roles instead of the first; none loosened, no assertion removed);
- `docs/campaign-certification/C01/research-index.json` (regenerated, separate commit; the only file shared with the packets running
  in parallel, so regenerate it on integration).

Holders added (all `attested_on`; none has a `from` or `until`):

- 幹部会委員長: 不破哲三 1994-07-19 (his bylined report to the 20th Congress), 1997-09-26 (first plenum communiqué and the congress
  diary), 2000-11-20 (opening address); 志位和夫 2001-05-29 (second plenum report), 2004-01-17, 2006-01-14, 2010-01-16, 2014-01-18,
  2017-01-18 and 2020-01-18 (summations and closing addresses on each congress's final day); then the unchanged 田村智子 2024-01-18.
- 中央委員会議長: 宮本顕治 1994-07-19 (宮本議長 in the chair's own report) and 1997-09-23 (congress diary: 宮本議長欠席); 不破哲三
  2000-11-24 (closing address: 私は中央委員会議長の任にあたることになりました, read as attested), 2004-01-17 (first plenum communiqué)
  and 2006-01-12 (issue date; his own 日本共産党中央委員会議長の不破哲三でございます); then the unchanged 志位和夫 2024-01-18.
- 代表 (国民民主党): 玉木雄一郎 2020-10-26, 2020-12-23, 2023-09-05 and 2025-03-04 (same-day party records); then the unchanged
  2026-09-06 observation.

Claims only: plenum and convention elections, 'new' stylings, the 1997 rule change making the Central Committee chair optional
(procedure), 宮本顕治's approval as 名誉議長, 不破哲三's withdrawal of January 2006, every record of the 国民民主党 of May 2018 to
September 2020 (its founding, co-representatives 大塚耕平 and 玉木雄一郎, its representative from 4 September 2018 and its dissolution),
the new party's founding (15 September 2020), the 2024-2025 suspension from posts and 古川元久's acting service as 代表代行.

Observation decisions:

| ID | Decision |
|---|---|
| JCP-01 | Accepted in part: both chairs observed 19 July 1994; no 1990-1993 party record found |
| JCP-02 | Accepted in part: 宮本顕治 observed 23 September 1997; rule change, 名誉議長 and an election naming nobody are claims; 不破哲三 observed 26 September 1997 |
| JCP-03 | Accepted in part: 不破哲三 委員長 20 November 2000 and 議長 24 November 2000; 志位和夫's election and 'new chair' styling are claims |
| JCP-04 | Accepted in part: 志位和夫 29 May 2001 and 17 January 2004; 不破哲三 議長 17 January 2004 |
| JCP-05 | Accepted in part: 不破哲三 議長 12 January 2006 (issue date); 志位和夫 14 January 2006; withdrawal claims only, no until |
| JCP-06 | Accepted in part: 志位和夫 2010, 2014, 2017 and 2020; the 2017 and 2020 elections are claims |
| DPFP-01 | Accepted in part (claims only): the earlier 国民民主党 and its co-representatives (7 May 2018) |
| DPFP-02 | Accepted in part (claims only): its representative election and assumption heading (4 September 2018) and its dissolution (11 September 2020) |
| DPFP-03 | Accepted in part: new party founded 15 September 2020; 玉木雄一郎 observed 26 October and 23 December 2020 |
| DPFP-04 | Accepted in part: elected 2 September 2023 (claim); observed 5 September 2023 |
| DPFP-05 | Accepted in part: suspension 4 December 2024 to 3 March 2025 and 古川元久's acting service (claims only); observed 4 March 2025 |

Ruling questions for Codex, decided by Ridge on 1 October 2026 (Codex may still decide otherwise):

- (a) The 国民民主党 of 2018-2020 stays claims only; work order `C01-Japan-DPFP-002` is proposed to base it on its own record (the
  founding-convention page of 7 May 2018, `jp_dpfp_2018_founding_convention_20180507`).
- (b) 不破哲三 gets no `until` for 2006: the closing address of 14 January 2006 reports the new Central Committee's confirmation that
  he withdraws as chair, but no source states the day the withdrawal took effect (Codex's strict rule, review 1739eccb); it stays a
  dated claim, and the 24th Congress first-plenum communiqué (しんぶん赤旗, 15 January 2006) is listed as a lead.
- (c) 宮本顕治's observation of 19 July 1994 stands: '宮本議長' in the Executive Committee chair's own report is the incumbent's
  current short title.
- (d) Merge commit `c478d66e` is confirmed: its tree equals integration `13367c99`.
- (e) The packet is no longer stacked: Codex accepted CLAUDE-C01-31 into integration (`13367c99`).

Every recorded identity is a raw Internet Archive capture made before the cutoff, downloaded at least twice by this packet at least
30 minutes apart with identical bytes and SHA-256 (curl's default User-Agent, no Accept-Encoding); the Internet Archive answered
with long runs of HTTP 429 and 503, and every request was made one at a time with a growing back-off. No live party page is
recorded.

Checks:

Run on the packet tree before committing (1 October 2026, UTC):

- `python -X utf8 tools/avatars/campaign_research.py` (regenerates `research-index.json`) and `--check`: pass (exit 0).
- `python -X utf8 tools/avatars/campaign_census.py --check`: pass (exit 0); it does not fail on this branch.
- `python -X utf8 -m unittest discover -s tools/avatars -p "test_japan*.py"`: 66 tests, OK.
- `python -X utf8 -m unittest discover -s tools/avatars -p "test_*research*.py"`: 79 tests, OK.
- `python -X utf8 -m unittest discover -s tools/avatars -p "test_campaign*.py"`: 16 tests, OK.
- `node --test tools/ui/check_leadership_research_review.cjs`: 11 pass, 0 fail.
- `python tools/planning/workboard.py --check`: PASS (44 canonical markers, 54 bounded tasks), after adding
  `docs/campaign-certification/S26/preparation` to this worktree's sparse checkout: the merged integration's workboard names a
  handoff there, and without it the check reports 'Missing task handoff' (a checkout limit, not a repository fault).
- `git diff --check`: clean.
- `python -X utf8 D:/spheres-scratch/c01-pipeline/tools/packet_check.py 42 --base origin/codex/campaign-certification` runs
  after the push (it needs a clean, pushed head) and re-downloads all 32 identities a third time; its summary is returned with
  this handoff.

Known failures outside the listed checks (not fixed): `tools/avatars/test_certified_gap_ledger.py` reports 'no pinned attribution' for this packet's sources until Codex classifies
  its commit in `COMMIT_PACKETS`; `tools/avatars/test_certified_boundary_matrix.py` (S23) needs `spheres-web/src`, which the
  sparse checkout lacks, so Codex lists the packet and regenerates
  `docs/campaign-certification/S23/preparation/boundary-matrix/` on integration. `campaign_census.py --check` passes on this
  branch.

## Checker fixes

Commit 'Apply checker fixes to CLAUDE-C01-42' on `7fc1d21b` (integration had not moved from `13367c99`), touching `japan.json`,
the report, this handoff and one extract (`japan-dpfp-representative-press-conference-20250304-facts.json`):

1. `jp_dpfp_2018_representative_elected_20180904` title: '【臨時党大会】（３）「国民のための政治をともに作っていこう」玉木新代表が就任あいさつ',
   as the capture's `<title>` and heading print it (verified on a fresh download with the recorded bytes and SHA-256).
2. `jp_dpfp_representative_election_20201218` title: '【代表選】臨時党大会が開催 新代表決定' (が; verified the same way).
3. `jp_dpfp_tamaki_in_office_20250304` claim text, identical in `japan.json` and the extract: "The record of the representative's
   regular press conference of 4 March 2025, tagged '代表', '玉木雄一郎' and '記者会見', has 玉木雄一郎 call it his return press
   conference ('今日復帰会見ですので')." The extract's snapshot bytes and SHA-256 are updated.
4. `jp_jcp_central_committee_chair` scope note: the clause on the plenums of 2006-2020 replaced by: the 2006 personnel report
   records the decision to have no chair ('議長をおかずに') and the 2017 and 2020 first-plenum lists name no Central Committee chair;
   no first-plenum record naming the officers was found for 2010 or 2014.
5. Ridge's rulings (a)-(e) recorded here and in the report.

The extracts carry no title field, so fixes 1 and 2 change `japan.json` and the report only. No holder, date, source identity,
claim id or count changes; no pinned test changes.

Checks re-run on the fixed tree (1 October 2026, UTC):

- `python -X utf8 tools/avatars/campaign_research.py` (regenerates `research-index.json`; only `japan.json`'s bytes and SHA-256
  change, committed separately as 'Regenerate the C01 research index for CLAUDE-C01-42 checker fixes') and `--check`: pass (exit 0).
- `python -X utf8 tools/avatars/campaign_census.py --check`: pass (exit 0).
- `python -X utf8 -m unittest discover -s tools/avatars -p "test_japan*.py"`: 66 tests, OK.
- `python -X utf8 -m unittest discover -s tools/avatars -p "test_*research*.py"`: 79 tests, OK.
- `python -X utf8 -m unittest discover -s tools/avatars -p "test_campaign*.py"`: 16 tests, OK.
- `node --test tools/ui/check_leadership_research_review.cjs`: 11 pass, 0 fail.
- `python tools/planning/workboard.py --check`: PASS (44 canonical markers, 54 bounded tasks).
- `git diff --check`: clean.
- `python -X utf8 D:/spheres-scratch/c01-pipeline/tools/packet_check.py 42 --base origin/codex/campaign-certification --no-fetch`
  runs after the push: no source identity was added or changed (two titles and one claim text only; the three affected captures
  were re-downloaded for verification with the recorded bytes and SHA-256), so it does not re-download the 32 identities.

C01 and all parent gates stay open.
