# CLAUDE-C01-18: Liberal Democratic Party presidents, 1990–2009

Owner: Claude. State: **ready_for_review** (25 September 2026; not complete, pending Codex acceptance). Parent: C01 (incomplete).

Origin: a self-proposed follow-up packet, started on the user's 25 September 2026 instruction to start another
batch of five packets in parallel. It does not repeat accepted C01-01/02/03/04/07/08 or reclaim
the pending C01-05, C01-06 and C01-09 to C01-17. It is pending Codex acceptance and is not registered in `docs/planning/ai-workstreams.json`.

Branch: `claude/c01-jp-18`. **Stacked on CLAUDE-C01-13** (`claude/c01-jp-13` at `42f98dc8`), which is
ready for review and not yet integrated, because both packets edit `japan.json`: merge CLAUDE-C01-13 first. Claim
commit: this record's first commit on the branch. The claim commit is `30c507d7`; `origin/claude/c01-jp-13` was fetched before the
work and had no commit missing from this branch, so no merge was needed.

## Bounded deliverable

Extend the existing party role `jp_ldp_party_president` (総裁) on the `自由民主党` organization observation, which today holds only 石破茂 (2024) and 高市早苗 (2025), with the presidents from 1 January 1990 to the September 2009 presidential election. Review at most ten observations:

1. The President when the period opens (海部俊樹) and the end of his presidency only if a source states it.
2. 1991: the presidential election won by 宮澤喜一.
3. 1993: the presidential election won by 河野洋平, a President who was never Prime Minister.
4. 1995: the presidential election won by 橋本龍太郎.
5. 1998: the presidential election won by 小渕恵三.
6. 2000: 森喜朗's selection as President after 小渕恵三's incapacity.
7. 2001: the presidential election won by 小泉純一郎, and his re-elections.
8. 2006 and 2007: the presidential elections won by 安倍晋三 and 福田康夫.
9. 2008: the presidential election won by 麻生太郎.
10. 2009: the presidential election won by 谷垣禎一, a President who was never Prime Minister.

Keep a party election or selection (votes by members and Diet members, or a joint plenary meeting of party Diet members), the declaration of results, the start of the term and a resignation as distinct dated claims. Party office and the prime-ministership stay separate: no `jp_pm` claim may feed the LDP role or the reverse, and the existing 2024 and 2025 LDP holders do not change. Names follow the packet's convention (Japanese as printed). Never infer an end from a successor's start unless a source states it. Give a holder `from` only where a source states the day office was assumed or took effect, and `until` only where a source states the day the office ended; otherwise record `attested_on`. Constitution and statute texts may establish procedure only, never a date. Retrospective lists are claims, never boundaries. News, encyclopaedias and history sites are leads only. The historical cutoff stays 7 September 2026.

Primary sources are required: the Liberal Democratic Party's own records (jimin.jp and its archived pages, party histories and announcements), and Diet minutes (kokkai.ndl.go.jp) only where a party officer's statement records the party office.

## Allowed files and checks

Allowed files:

- this record;
- a new `docs/campaign-certification/C01/research/japan-ldp-presidents-1990-2009-18.md`;
- `docs/campaign-certification/C01/research/japan.json` and specifically related new
  `docs/campaign-certification/C01/research/sources/japan-*-facts.json` extracts;
- focused tests under `tools/avatars/`: a new `test_japan_ldp_presidents_c01_18.py`, and pinned counts or exact sets in `test_japan_research_s10d.py`, `test_japan_prime_ministers_c01_12.py` and `test_japan_prime_ministers_c01_13.py`
  updated to the new totals (none loosened).

Put generated `research-index.json` changes in a separate commit. Do not change shared UI, the roadmap, game
data or other country packets.

Checks: research-index `--check`; the Japan, research and campaign Python tests; the atlas Node check;
`workboard.py --check`; `git diff --check`.

Mark the packet `ready_for_review` when done. C01 and all parent gates stay open.

## Result (ready_for_review)

Claim commit `30c507d7` (this record only), on top of `claude/c01-jp-13` at `42f98dc8`; packet commit and research-index commit
on `claude/c01-jp-18`. Report:
[japan-ldp-presidents-1990-2009-18.md](../../campaign-certification/C01/research/japan-ldp-presidents-1990-2009-18.md).

Touched paths (all inside this record's allowed files):

- `docs/planning/ai-handoffs/CLAUDE-C01-18.md` (this record);
- `docs/campaign-certification/C01/research/japan-ldp-presidents-1990-2009-18.md` (new report);
- `docs/campaign-certification/C01/research/japan.json` (additions only: 81 sources and 192 claims appended after CLAUDE-C01-13's;
  on the 自由民主党 organization observation `jp_sangiin_pr_2025_13`, its `sources` and `claim_ids` and the `jp_ldp_party_president`
  role's `sources` and `claim_ids` extended, fourteen holders inserted before the unchanged 2024 and 2025 holders, a sentence appended
  to the role's `scope_note`, one organization coverage note and one packet coverage note appended; no existing text changed);
- 81 new `docs/campaign-certification/C01/research/sources/japan-*-facts.json` extracts, one per new source (no existing extract
  edited);
- `tools/avatars/test_japan_ldp_presidents_c01_18.py` (new);
- `tools/avatars/test_japan_research_s10d.py`, `tools/avatars/test_japan_prime_ministers_c01_12.py` and
  `tools/avatars/test_japan_prime_ministers_c01_13.py` (pinned counts, exact source lists, LDP holder lists, access dates and the
  coverage-note positions re-expressed for the extension; none loosened, no assertion removed);
- `docs/campaign-certification/C01/research-index.json` (regenerated, separate commit).

The packet extends `jp_ldp_party_president` (総裁) with no new institution, organization or role, and `jp_pm` is unchanged. Holders
added, in order: 海部俊樹 observed 14 May 1990; 宮澤喜一 observed 30 November 1992; 河野洋平 observed 25 November 1994; 橋本龍太郎
observed 2 October 1995; 小渕恵三 observed 24 July 1998 and 22 September 1999; 森喜朗 from 5 April 2000 (his own statement at the
joint plenary meeting that he took office that day, ただ今…就任致すことになりました; flagged for the reviewer); 小泉純一郎 observed
24 April 2001, 10 August 2001 and 20 September 2003; 安倍晋三 observed 20 September 2006; 福田康夫 observed 23 September 2007;
麻生太郎 observed 22 September 2008; 谷垣禎一 observed 28 September 2009. No holder has an `until`: no party record or officer's
statement reviewed states the day a 1989-2009 presidency ended, and no end is inferred from a successor's election. Each holder cites
only same-day party records or the president's own Diet statement naming him as president; party elections and selections, notices,
candidacies, declarations and reports of results, index headlines, announced or prospective resignations, 前総裁 references,
continuation attestations and retrospective lists and histories are claims only. 就任 wording in captions, headings and headlines
is never a start, because the party's own 2006 and 2009 records conflict on when a term took effect (vote-day 新総裁, 30 September
term expiry, 1 October list start). Party office and the prime-ministership stay separate both ways: every added id begins
`jp_ldp`, no `jp_pm` claim or source is cited by the party role, and the 2024 and 2025 LDP holders are unchanged.

Observation decisions:

| ID | Decision |
|---|---|
| LDP-PRES-01 | Accepted in part: 海部俊樹 observed 14 May 1990; the 1989 election, the October 1989 re-selection and the 1991 withdrawal are retrospective claims; no start or end |
| LDP-PRES-02 | Accepted in part: 宮澤喜一 observed 30 November 1992 (explicit); the 27 October 1991 election and 29 October convention are retrospective; implicit references of February and March 1992 are claims |
| LDP-PRES-03 | Accepted in part: 河野洋平 observed 25 November 1994; the 30 July 1993 election and 30 September 1993 second selection are retrospective; his recollections undated |
| LDP-PRES-04 | Accepted in part: 橋本龍太郎 observed 2 October 1995; the 25 September 1995 convention selection is a claim, not a start; the 1997 re-selection is claims only |
| LDP-PRES-05 | Accepted in part: 小渕恵三 observed 24 July 1998 and 22 September 1999; votes, declarations and the convention report are claims |
| LDP-PRES-06 | Accepted in part: 森喜朗 from 5 April 2000 (the packet's only stated start); no end |
| LDP-PRES-07 | Accepted in part: 小泉純一郎 observed 24 April 2001, 10 August 2001 and 20 September 2003; no start or end |
| LDP-PRES-08 | Accepted in part: 安倍晋三 observed 20 September 2006 (term-start conflict unresolved); 福田康夫 observed 23 September 2007 |
| LDP-PRES-09 | Accepted in part: 麻生太郎 observed 22 September 2008 |
| LDP-PRES-10 | Accepted in part: 谷垣禎一 observed 28 September 2009 (term-start conflict and 麻生太郎's end unresolved) |

All 43 checker defects (A1-A14, B1-B19, C1-C10) are applied or resolved (B14 by removal; B16 and C9 by recording the party's list once
from its identity-encoded capture); the report's Checker defects table gives each outcome. Of the 21 missing primary records the
checks confirmed, 14 are imported as sources, one is an alternate location (the relocated 森 greeting), one is the recorded location
of a source (the party-history PDF's raw capture), four are leads with the reason (three redundant result tables and the 2008 rules,
procedure only), and one (contemporaneous party records of 1989-1995) was not found online. The three parts' copies of the party's
list and printed history are recorded once each. This packet downloaded every recorded identity again on 25 September 2026 (plain at
14:15Z-14:20Z and again at 14:52Z-14:56Z, and the 22 Diet API responses also with a cache-busting query at 14:56Z-14:57Z), with the same byte count and
SHA-256 each time. NDL WARP returned 403; nothing was bypassed, and no terms, logins or CAPTCHAs were met.

Checks run on 25 September 2026 in this sparse worktree (`docs/campaign-certification/C01`, `docs/planning`, `tools/avatars`,
`tools/planning`, `tools/ui`; not widened), with `PYTHONDONTWRITEBYTECODE=1`:

- `python -X utf8 tools/avatars/campaign_research.py` then `--check`: exact regeneration passes; 9 packets, 841 organization and 28
  institution observations, 572 sources, 2,421 claims, 92 open batches (Japan: 418 sources, 688 claims).
- Japan tests (`-p "test_japan*.py"`): 35 pass (9 in the new `test_japan_ldp_presidents_c01_18.py`, whose mutation test rejects 39
  rule mutations, 8 validator mutations, 4 row mutations and 4 event re-datings; 9 in `test_japan_prime_ministers_c01_13.py`, 9 in
  `test_japan_prime_ministers_c01_12.py` and 8 in `test_japan_research_s10d.py`).
- Research tests (`-p "test_*research*.py"`): 79 pass.
- Campaign tests (`-p "test_campaign*.py"`): the 9 `test_campaign_research` tests pass; `test_campaign_census` errors in `setUpClass`
  because `spheres-sim/data/party_leaders.json` is not in this sparse worktree. It was not run, and the sparse checkout was not
  widened. These suites overlap; their counts are not summed.
- `node --test tools/ui/check_leadership_research_review.cjs`: 11 pass.
- `python tools/planning/workboard.py --check`: passes (44 markers).
- `git diff --check`: clean for this packet's paths.
- The new test rejects hand-made regressions both by rule and against the pinned lists: a successor's election, start or
  attestation used or cited as an end (海部俊樹, 橋本龍太郎, 森喜朗, 福田康夫, 麻生太郎); a predecessor reference, a term-expiry
  schedule, an announced or prospective resignation or a retrospective span used as an end; an election, convention, list start,
  nomination or 就任 caption date used as a start; an election, declaration, nomination, retrospective row, unnamed caption or index
  headline cited by a holder; a named attestation left off its holder; an interim holder in the 2000 vacancy, a continuation, an
  incumbent-before-the-vote or an implicit reference added as a holder; a `jp_pm` claim or holder moved onto the party role or the
  reverse; the role copied to an institution or another party, a second role or a new institution; the 2024 observation changed;
  holders out of order; a retrospective span given a date; a beyond-cutoff date; checksum mismatches and escaped snapshot paths.
