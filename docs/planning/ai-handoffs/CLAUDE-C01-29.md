# CLAUDE-C01-29: Social Democratic Party (Japan Socialist Party) chairs, 1990–2026

Owner: Claude. State: **ready_for_review** (28 September 2026; not complete, pending Codex acceptance). Parent: C01 (incomplete).

Origin: part of the back-to-back C01 research pipeline the user asked for on 28 September 2026, taken from the
certified-country gap ledger (`docs/campaign-certification/C01/gap-ledger/ledger.md`) items `Japan/jp_jsp`, `Japan/jp_jsp/jp_jsp_1945`, `Japan/jp_jsp/jp_sdp_1996`. Pending Codex
acceptance; not registered in the task queue.

Branch: `claude/c01-jp-29`. Base: `f3514fc6` (current `codex/campaign-certification`); not stacked on a pending packet. Claim commit: this record's first commit on the branch.

## Bounded deliverable

Add one party role, jp_sdp_chair (委員長 / 党首 — chair of the Japan Socialist Party and, from its 1996 renaming, the Social Democratic Party; kind party_leader), to the existing 社会民主党 election-list observation jp_sangiin_pr_2025_10. Research the chairs from 1 January 1990 to the cutoff (expected 土井たか子, 田邊誠, 山花貞夫, 村山富市, then 土井たか子, 福島瑞穂, 吉田忠智, 又市征治 and 福島瑞穂 again): each party election or appointment, assumption, resignation and the January 1996 renaming as distinct dated claims. The renaming is a claim about the organization, never a merge of identities; the 2020 split of most members into another party is a claim only. Sources: the SDP's own records (sdp.or.jp and archived pages), Diet minutes only where a party officer's statement records the party office. Names in Japanese as printed.

Keep each distinct event (election or selection, appointment, assumption of office, acting or interim service,
resignation, removal, death, merger, renaming) as its own dated claim; acting service is claims only. Never infer an
end from a successor's start. Give a holder `from` or `until` only where a source states the day; otherwise record
`attested_on`. Party office and state office stay separate both ways. Organization identities, lifecycles and game
mappings stay unresolved; a name match to a simulation row is never a mapping. At most ten people. Primary sources
only; news and encyclopaedias are leads. The historical cutoff stays 7 September 2026.

## Allowed files and checks

Allowed files:

- this record;
- a new `docs/campaign-certification/C01/research/japan-sdp-chairs-1990-2026-29.md`;
- `docs/campaign-certification/C01/research/japan.json` and specifically related new
  `docs/campaign-certification/C01/research/sources/japan-*-facts.json` extracts;
- focused tests under `tools/avatars/`: a new `test_japan_sdp_chairs_c01_29.py`, and pinned counts or exact sets in `test_japan_research_s10d.py`, `test_japan_prime_ministers_c01_12.py`, `test_japan_prime_ministers_c01_13.py`, `test_japan_ldp_presidents_c01_18.py` updated to
  the new totals (none loosened).

Put generated `research-index.json` changes in a separate commit. Do not change the gap ledger, shared UI, the
roadmap, game data or other country packets.

Checks: research-index `--check`; `campaign_census.py --check`; the Japan, research and campaign Python tests
(census included); the atlas Node check; `workboard.py --check`; `git diff --check`.

Mark the packet `ready_for_review` when done. C01 and all parent gates stay open.

## Integration claim registration — 28 September 2026

Codex mirrored this existing claim into the central queue after S22. The original
claim above remains in progress; no research content or historical acceptance
was imported. Inspected remote head: `2d65b680`. Do not duplicate this work.

## Result (ready_for_review)

Claim commit `2d65b680` (this record only) on base `f3514fc6`; not stacked. Before the packet commits,
`origin/codex/campaign-certification` was fetched and merged at `e6ee9f7c` (merge commit `fa6c3de2`; the only conflict was an
add/add in this record, where the integration branch's copy is the claim record plus the claim-registration note above, so it
was taken unchanged and this section appended) and again at `3efac1b9` (merge commit `41f8b976`, no conflict). Then the packet commit and the research-index commit on `claude/c01-jp-29`.
Report: [japan-sdp-chairs-1990-2026-29.md](../../campaign-certification/C01/research/japan-sdp-chairs-1990-2026-29.md).

Touched paths (all inside this record's allowed files):

- `docs/planning/ai-handoffs/CLAUDE-C01-29.md` (this record: state line and this section);
- `docs/campaign-certification/C01/research/japan-sdp-chairs-1990-2026-29.md` (new report);
- `docs/campaign-certification/C01/research/japan.json` (additions only: 54 sources and 111 claims appended after CLAUDE-C01-18's;
  on the 社会民主党 organization observation `jp_sangiin_pr_2025_10`, the new role `jp_sdp_chair` (its `roles` was empty), its
  `sources` and `claim_ids` extended and one coverage note appended; one packet coverage note appended; no existing text changed);
- 54 new `docs/campaign-certification/C01/research/sources/japan-*-facts.json` extracts, one per new source (no existing extract
  edited);
- `tools/avatars/test_japan_sdp_chairs_c01_29.py` (new);
- `tools/avatars/test_japan_research_s10d.py`, `tools/avatars/test_japan_prime_ministers_c01_12.py`,
  `tools/avatars/test_japan_prime_ministers_c01_13.py` and `tools/avatars/test_japan_ldp_presidents_c01_18.py` (pinned counts, exact
  source lists, holder lists, access dates, role counts and coverage-note positions re-expressed; the no-end and one-start guards
  re-expressed as exact pins; none loosened, no assertion removed);
- `docs/campaign-certification/C01/research-index.json` (regenerated, separate commit).

The packet adds the party role `jp_sdp_chair` (委員長 / 党首, kind `party_leader`) with sixteen holder observations of seven people
and no new organization or institution; `jp_pm`, the LDP presidency and every other role are unchanged. Holders, in order: 土井たか子
observed 6 April 1990 (the party secretary-general 山口鶴男's Diet statement); 田邊誠 observed 20 August 1991 (the secretary-general
山花貞夫's Diet statement); 山花貞夫 observed 25 January 1993 (his own statement); 村山富市 observed 13 October 1994 (his own statement,
party-office wording only, while Prime Minister); 土井たか子 observed 30 November 1996, 21 January 1998 and 21 January 2000 (party
records); 福島瑞穂 from 15 November 2003 (her own statement of 26 November 2003 of the day she became leader); 福島瑞穂 observed 9
December 2009, and observed 24 January 2012 until 25 July 2013 (her words 本日で辞任する as the party reported them); 吉田忠智 observed
24 October 2013 (his own statement); 又市征治 observed 25 February 2018 (his inaugural address as 党首); 福島瑞穂 observed 28 February
2020, 14 January 2022, 1 December 2023 and 8 April 2026 (party records). Only 福島瑞穂 has a stated start and a stated end. Each holder
cites only same-day party records or a party officer's own Diet statement of the office; elections, notices, filings, counts,
declarations, convention confirmations, resignations, the acting leader (又市征治, 1 August 2013), predecessor references,
continuations, the 1996 renaming, the 2020 convention decision and retrospective records are claims only. The renaming and the 2020
decision are claims about the organization: no identity is merged or mapped.

Observation decisions:

| ID | Decision |
|---|---|
| SDP-CHAIR-01 | Accepted in part: 土井たか子 observed 6 April 1990; her 1986 assumption and July 1991 resignation are month-only retrospective claims; no 1991 resignation record found |
| SDP-CHAIR-02 | Accepted in part: 田邊誠 observed 20 August 1991; selection and resignation not attested |
| SDP-CHAIR-03 | Accepted in part: 山花貞夫 observed 25 January 1993 (就任 without a day is not a start); still chair 24 September 1993; election and resignation not dated |
| SDP-CHAIR-04 | Accepted in part: 村山富市 observed 13 October 1994, continued to 11 October 1995; September 1993 election not attested |
| SDP-CHAIR-05 | Accepted in part (organization claims only): the renaming is retrospective (19 January 1996 printed in 2016; January 1996 in 2025) and the new name is in use on 24 January 1996; no holder as 党首 found |
| SDP-CHAIR-06 | Accepted in part: 土井たか子 observed 30 November 1996, 21 January 1998 and 21 January 2000; succession day not stated (retrospectives conflict); 2002 not found; resignation announced 13 November and stated 15 November 2003; no end |
| SDP-CHAIR-07 | Accepted in part: 福島瑞穂 from 15 November 2003; observed 9 December 2009 and 24 January 2012; until 25 July 2013; 2005 and 2007 not found |
| SDP-CHAIR-08 | Accepted in part: acting leader 1 August 2013 (claim); the 2013 election (count 14 October); 吉田忠智 observed 24 October 2013; his recalled 1 November 2013 takeover conflicts and is never a start; no end |
| SDP-CHAIR-09 | Accepted in part: 又市征治 uncontested 26 January 2018, observed 25 February 2018; no end |
| SDP-CHAIR-10 | Accepted in part: 福島瑞穂 elected 22 February 2020, observed 28 February 2020, 14 January 2022 and 1 December 2023; the 2020 convention decision is an organization claim |
| SDP-CHAIR-11 | Accepted in part: the 2026 election (notice 4 March, inconclusive count 23 March, run-off count 6 April); 福島瑞穂 observed 8 April 2026; continuations 29 April and 29 July 2026 |

Integration decisions (the report's table gives each): the Prime Minister's replies naming the chair are leads, not party
officers' statements, so 土井たか子 and 田邊誠 are dated by two secretary-general statements the integrator found and 村山富市 by his own
statement; 山花貞夫's statements of 7 October 1993, made as a minister holding no party office, are leads (CLAUDE-C01-18's rule for
河野洋平); a recollection printing no year has no structured date; the 2020 observation is dated by the party's news items of 28
February 2020, not by an address given at the end of the convention; 吉田忠智's recalled takeover day is never a start. Every
recorded identity was downloaded again by this packet on 28 September 2026 (all 54 at 15:34Z-15:37Z and again at 16:09Z-16:13Z,
the 22 Diet responses also with a cache-busting query), with the same byte count and SHA-256 each time; three captures' SHA-1 differ
from their CDX digests (two chunked records and an early ARC record), with stable served bytes. Nothing was blocked; no terms,
logins or CAPTCHAs were met. The central task-queue entry (`claimed`) is Codex's to update.

Checks run on 28 September 2026 in this sparse worktree (not widened), with `PYTHONDONTWRITEBYTECODE=1`, after the merge:

- `python -X utf8 tools/avatars/campaign_research.py` then `--check`: exact regeneration passes; 9 packets, 841 organization and 34
  institution observations, 1,382 sources, 3,839 claims, 93 open batches (Japan: 472 sources, 799 claims, 6 role observations).
- `python -X utf8 tools/avatars/campaign_census.py --check`: exit 0.
- Japan tests (`-p "test_japan*.py"`): 44 pass (9 in the new `test_japan_sdp_chairs_c01_29.py`, whose mutation test rejects 41 rule
  mutations, 8 validator mutations, 5 row mutations and 4 event re-datings; 9 in `test_japan_ldp_presidents_c01_18.py`, 9 in
  `test_japan_prime_ministers_c01_13.py`, 9 in `test_japan_prime_ministers_c01_12.py` and 8 in `test_japan_research_s10d.py`).
- Research tests (`-p "test_*research*.py"`): 79 pass. Campaign tests (`-p "test_campaign*.py"`): all 16 pass.
- `node --test tools/ui/check_leadership_research_review.cjs`: 11 pass.
- `python tools/planning/workboard.py --check`: PASS.
- `git diff --check` on the packet's paths: clean.
- Not run: `tools/avatars/certified_boundary_matrix.py --check` (S23) needs `spheres-web/src`, which is outside this sparse checkout;
  its Japan cases read `japan.json` and the research reports, so the integrator should regenerate the S23 matrix after merging (a file
  outside this packet's boundary). The gap ledger was not touched.
