# CLAUDE-C01-13: Japanese prime ministers, 2006–2026

Owner: Claude. State: **ready_for_review** (25 September 2026; not complete, pending Codex acceptance). Parent: C01 (incomplete).

Origin: a self-proposed follow-up packet, started on the user's 24 September 2026 instruction to continue
development faster with several packets in parallel. It does not repeat accepted C01-01/02/03/04/07/08 or reclaim
the pending C01-05 (USSR/RSFSR), C01-06 (Saudi Arabia), C01-09 (South Africa), C01-10 (Brazil), C01-11 (India) and C01-12 (Japan, 1990-2006). It is pending Codex acceptance and is not registered in `docs/planning/ai-workstreams.json`.

Branch: `claude/c01-jp-13`. **Stacked on CLAUDE-C01-12** (`claude/c01-jp-12` at `87e1da53`), which is
ready for review and not yet integrated, because both packets edit `japan.json`: merge CLAUDE-C01-12 first. Claim
commit: this record's first commit on the branch. The claim commit is `3bcd6e6c`; `claude/c01-jp-12` up to `da358dbf` (its report-only verifier fixes) was merged into this branch in `b0e3c5b5` before the packet was written.

## Bounded deliverable

Continue CLAUDE-C01-12 on the same `jp_prime_minister` institution and `jp_pm` role (内閣総理大臣), from the appointment of 安倍晋三 on 26 September 2006 that C01-12 records only as its closing boundary, to the cutoff. Review at most ten observations:

1. 2006–2007: 安倍晋三's designation and appointment, and his cabinet's resignation.
2. 2007–2008: 福田康夫's designation, appointment and resignation.
3. 2008–2009: 麻生太郎's designation, appointment and resignation.
4. 2009–2010: 鳩山由紀夫's designation, appointment and resignation.
5. 2010–2011: 菅直人's designation, appointment and resignation.
6. 2011–2012: 野田佳彦's designation, appointment and resignation.
7. 2012–2020: 安倍晋三's designation and appointment of December 2012, his re-designations of 2014 and 2017, and his 2020 resignation.
8. 2020–2021: 菅義偉's designation, appointment and resignation.
9. 2021–2024: 岸田文雄's designation and appointment of October 2021, his November 2021 re-designation, and his 2024 resignation.
10. 2024–2026: 石破茂's designations of October and November 2024, 高市早苗's designation and appointment of October 2025, and an official attestation of the holder in office before the cutoff.

Keep each House's designation vote, the Imperial appointment ceremony (親任式), the cabinet's formation, resignation en masse (総辞職) and continued performance of duties as distinct dated claims; continued duties and any acting prime minister are claims, never holders. The existing LDP president observations (石破茂, 高市早苗) stay separate and unchanged; party office never feeds `jp_pm`. Names follow the packet's convention (Japanese as printed). Never infer an end from a successor's start unless a source states it. Give a holder `from` only where a source states the day office was assumed or took effect, and `until` only where a source states the day the office ended; otherwise record `attested_on`. Constitution and statute texts may establish procedure only, never a date. Retrospective lists are claims, never boundaries. News, encyclopaedias and history sites are leads only. The historical cutoff stays 7 September 2026.

Primary sources are required: the Prime Minister's Office and Cabinet (kantei.go.jp, including archived pages), the minutes of the National Diet (kokkai.ndl.go.jp) and the two Houses' own records, the Official Gazette where accessible without restriction, and the Imperial Household Agency.

## Allowed files and checks

Allowed files:

- this record;
- a new `docs/campaign-certification/C01/research/japan-prime-ministers-2006-2026-13.md`;
- `docs/campaign-certification/C01/research/japan.json` and specifically related new
  `docs/campaign-certification/C01/research/sources/japan-*-facts.json` extracts;
- focused tests under `tools/avatars/`: a new `test_japan_prime_ministers_c01_13.py`, and pinned counts or exact sets in `test_japan_research_s10d.py` and `test_japan_prime_ministers_c01_12.py`
  updated to the new totals (none loosened).

Put generated `research-index.json` changes in a separate commit. Do not change shared UI, the roadmap, game
data or other country packets.

Checks: research-index `--check`; the Japan, research and campaign Python tests; the atlas Node check;
`workboard.py --check`; `git diff --check`.

Mark the packet `ready_for_review` when done. C01 and all parent gates stay open.

## Result (ready_for_review)

Claim commit `3bcd6e6c` (this record only); stacked on `claude/c01-jp-12` up to `da358dbf` (merged, no conflict); packet
commit and research-index commit on `claude/c01-jp-13`. Report:
[japan-prime-ministers-2006-2026-13.md](../../campaign-certification/C01/research/japan-prime-ministers-2006-2026-13.md).

Touched paths (all inside this record's allowed files):

- `docs/planning/ai-handoffs/CLAUDE-C01-13.md` (this record);
- `docs/campaign-certification/C01/research/japan-prime-ministers-2006-2026-13.md` (new report);
- `docs/campaign-certification/C01/research/japan.json` (additions only: 211 sources, 293 claims, sixteen `jp_pm` holders, a
  scope-note addition and one packet coverage note appended, and three institution coverage notes inserted before C01-12's closing 'Keep executive office distinct from party leadership' note, which stays last; no existing text changed);
- 211 new `docs/campaign-certification/C01/research/sources/japan-*-facts.json` extracts, one per new source;
- `tools/avatars/test_japan_prime_ministers_c01_13.py` (new);
- `tools/avatars/test_japan_prime_ministers_c01_12.py` and `tools/avatars/test_japan_research_s10d.py` (pinned counts, exact
  lists, hosts, access dates and markers re-expressed for the extension; none loosened, no assertion removed);
- `docs/campaign-certification/C01/research-index.json` (regenerated, separate commit).

The packet extends `jp_prime_minister` / `jp_pm` (内閣総理大臣) with no new institution or role. Holders added, each with a
stated start: 安倍晋三 from 26 September 2006; 福田康夫 from 26 September 2007; 麻生太郎 from 24 September 2008; 鳩山由紀夫
from 16 September 2009; 菅直人 from 8 June 2010; 野田佳彦 from 2 September 2011; 安倍晋三 from 26 December 2012, from
24 December 2014 and from 1 November 2017 until 16 September 2020; 菅義偉 from 16 September 2020 until 4 October 2021; 岸田文雄
from 4 October 2021 until 10 November 2021 and from 10 November 2021 until 1 October 2024; 石破茂 from 1 October 2024 until
11 November 2024 and from 11 November 2024 until 21 October 2025; 高市早苗 from 21 October 2025 until 18 February 2026 and from
18 February 2026 (in office on 27 July and 4 September 2026). Every end is stated by the Official Gazette's notice of loss of
office (内閣総理大臣及び国務大臣退官); no end is inferred from a successor's start, and the 2006-2014 appointments have no end
because the Gazette of those years is not available without restriction. Each House's designation vote, the joint committees
of 2007 and 2008, the Imperial appointment ceremony (親任式), the cabinet's formation, the resignation en masse (総辞職) and the
continued performance of duties are separate dated claims; continued duties and the art. 9 acting orders are claims only. The
LDP presidency observations (石破茂, 高市早苗) are unchanged and never feed `jp_pm`. The 2006 start rests on the stacked
packet's three closing-boundary responses re-recorded under new ids with named rows (a deliberate repeated identity, pinned
and left to the integrator's ruling).

Observation decisions:

| ID | Decision |
|---|---|
| JP-PM06-01 | Accepted in part: 安倍晋三 from 26 Sep 2006, in office 24 Sep 2007; announcement 12 Sep, hospital 13 Sep, no acting Prime Minister, resignation 25 Sep 2007; no stated end |
| JP-PM06-02 | Accepted in part: art. 67(2) after the joint committee failed on 25 Sep 2007; 福田康夫 from 26 Sep 2007 (08:30); resignation 24 Sep 2008; no stated end |
| JP-PM06-03 | Accepted in part: art. 67(2) on 24 Sep 2008; 麻生太郎 from that day; resignation 16 Sep 2009; no stated end |
| JP-PM06-04 | Accepted in part: 鳩山由紀夫 from 16 Sep 2009; announcement 2 Jun 2010, resignation 4 Jun 2010; the end conflict (4 June against continued duties to 8 June) left unresolved |
| JP-PM06-05 | Accepted in part: 菅直人 designated 4 Jun, from 8 Jun 2010; resignation 30 Aug 2011, continued duties to 2 Sep 2011 (claims); no stated end |
| JP-PM06-06 | Accepted in part: 野田佳彦 designated 30 Aug, from 2 Sep 2011; resignation 26 Dec 2012; no stated end |
| JP-PM06-07 | Accepted in part: 安倍晋三 from 26 Dec 2012, 24 Dec 2014 and 1 Nov 2017, the last until 16 Sep 2020 (Gazette); no stated end for 2012 and 2014 |
| JP-PM06-08 | Accepted: 菅義偉 from 16 Sep 2020 until 4 Oct 2021 |
| JP-PM06-09 | Accepted: 岸田文雄 from 4 Oct 2021 until 10 Nov 2021, and from 10 Nov 2021 until 1 Oct 2024 |
| JP-PM06-10 | Accepted: 石破茂 1 Oct 2024-11 Nov 2024 and 11 Nov 2024-21 Oct 2025; 高市早苗 21 Oct 2025-18 Feb 2026 and from 18 Feb 2026, in office before the cutoff |

All 34 checker defects (A1-A15, B1-B6, C1-C13) are applied, A4 resolved as described; the report's Checker defects table
gives each outcome. Of the 59 missing primary records the checks confirmed, 40 are imported as sources (with one 2020
capture replaced by a later capture of the same bytes), two live pages are recorded as alternate locations, and seventeen
redundant ones (thirteen cabinet agendas duplicated by the imported minutes, and four Chief Cabinet Secretary pages) are leads
with their reasons. This packet downloaded every recorded identity again on 25 September 2026 (plain at 01:30Z-01:41Z, the 67 Diet API responses and 31 stored official files with a cache-busting query at 01:41Z-01:42Z, and plain again at 02:13Z-02:21Z, at least 30 minutes after the first) with the same
byte count and SHA-256, and found the SHA-1 of all 113 captures equal to the Internet Archive's CDX digest. The Official Gazette
of 2006-2017 stayed unavailable without restriction; NDL WARP returned 403; nothing was bypassed.

Checks run on 25 September 2026 in this sparse worktree (`docs/campaign-certification/C01`, `docs/planning`,
`tools/avatars`, `tools/planning`, `tools/ui`; not widened), with `PYTHONDONTWRITEBYTECODE=1`:

- `python -X utf8 tools/avatars/campaign_research.py` then `--check`: exact regeneration passes; 9 packets, 841 organization and 28 institution observations, 491 sources, 2,229 claims, 92 open batches (Japan: 337 sources, 496 claims).
- Japan tests (`-p "test_japan*.py"`): 26 pass (9 in the new `test_japan_prime_ministers_c01_13.py`, whose mutation test rejects 34 rule mutations, 8 validator mutations, 4 row mutations and 4 event collapses; 9 in `test_japan_prime_ministers_c01_12.py` and 8 in `test_japan_research_s10d.py`).
- Research tests (`-p "test_*research*.py"`): 79 pass.
- Campaign tests (`-p "test_campaign*.py"`): the 9 `test_campaign_research` tests pass; `test_campaign_census` errors in `setUpClass` because `spheres-sim/data/party_leaders.json` is not in this sparse worktree. It was not run, and the sparse checkout was not widened. These suites overlap; their counts are not summed.
- `node --test tools/ui/check_leadership_research_review.cjs`: 11 pass.
- `python tools/planning/workboard.py --check`: passes (44 markers).
- `git diff --check`: clean for this packet's paths.
- The new test rejects hand-made regressions both by rule and against the pinned list: a successor's start, ceremony or
  appointment used or cited as an end (麻生太郎, 安倍晋三 2006, 福田康夫), or the cutoff used as an end (高市早苗 2026); a resignation notice, a departure, a
  retrospective span, the Chief Cabinet Secretary's span or the 'today' remarks used as an end (菅直人, 鳩山由紀夫); a Gazette
  end dropped while the until stays, or an until dropped while the Gazette end stays; a designation date or designation, the
  prevailing resolution, an unnamed statement or an unnamed schedule row used or cited as a start; a named start left off its
  holder; an acting designation or continued duties added as a holder or cited by one; a party-president claim cited by
  `jp_pm`, a `jp_pm` holder moved onto the party presidency, a parliamentary-group claim or an earlier packet's claim cited by
  a holder; a second role or institution; spans given a structured date or period; holders out of order; a beyond-cutoff
  end; acting and unnamed rows given a holder name; a span relabelled as an end; collapsed event dates; and checksum, path,
  reversed-interval, cited-source and beyond-cutoff validator mutations.

C01 and all parent gates (C06, S23, WC1, CP1) stay open. No installed leader, avatar, portrait, campaign rule or save
schema changed.
