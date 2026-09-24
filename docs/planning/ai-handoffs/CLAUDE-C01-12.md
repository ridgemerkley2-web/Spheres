# CLAUDE-C01-12: Japanese prime ministers, 1990–2006

Owner: Claude. State: **ready_for_review** (24 September 2026; submitted, not accepted). Parent: C01
(incomplete). Report: [japan-prime-ministers-1990-2006-12.md](../../campaign-certification/C01/research/japan-prime-ministers-1990-2006-12.md).

Origin: a self-proposed follow-up packet, started on the user's 24 September 2026 instruction to continue
development. The workboard asks for a distinct bounded packet that does not repeat accepted
C01-01/02/03/04/07/08 or reclaim the pending C01-05 (USSR/RSFSR), C01-06 (Saudi Arabia), C01-09 (South Africa),
C01-10 (Brazil) and C01-11 (India). This packet touches only the Japan research packet, which none of those
change. It is pending Codex acceptance and is not registered in `docs/planning/ai-workstreams.json`.

Branch: `claude/c01-jp-12`. Base: `ffe54b02` (current `codex/campaign-certification`). Claim commit: `c2aafd22`
(this record only).

## Bounded deliverable

The Japan packet has election-list, parliamentary-group and party-office observations only. Add one executive
institution, `jp_prime_minister`, with one role, `jp_pm` (内閣総理大臣 — Prime Minister of Japan, kind
`head_of_government`), and review at most ten observations between 1 January 1990 and the end of the Koizumi
cabinets in September 2006. The 2006–2026 prime ministers are left to a later packet.

1. The holder when the period opens (Toshiki Kaifu), and his re-designation and new cabinet after the
   February 1990 general election.
2. 1991: the Kaifu cabinet's resignation en masse and Kiichi Miyazawa's designation and appointment.
3. 1993: the Miyazawa cabinet's resignation and Morihiro Hosokawa's designation and appointment.
4. 1994: Tsutomu Hata's designation and appointment.
5. 1994: Tomiichi Murayama's designation and appointment.
6. 1996: Ryūtarō Hashimoto's designation and appointment, and his November 1996 re-designation.
7. 1998: Keizō Obuchi's designation and appointment.
8. 2000: Obuchi's incapacity, the acting prime minister (臨時代理), the cabinet's resignation, and Yoshirō
   Mori's designation and appointment, and his July 2000 re-designation.
9. 2001: Jun'ichirō Koizumi's designation and appointment, and his re-designations of 2003 and 2005.
10. 2006: the Koizumi cabinet's resignation, the date the office passed, and the successor's appointment only
    as the closing boundary of this packet.

Keep the Diet's designation (each House's vote, and a joint-committee or House of Representatives resolution
where they differ), the Imperial appointment ceremony (親任式), a cabinet's resignation en masse (総辞職),
continued performance of duties until a successor is appointed, and an acting prime minister as distinct dated
claims. Never infer an end from a successor's appointment unless a source states it. Give a holder `from` only
where a source states the day of appointment or assumption of office, and `until` only where a source states the
day the office ended; otherwise record `attested_on`. Continued performance of duties and an acting prime
minister are recorded as claims, never as holders. Follow the packet's existing convention for names (Japanese
as printed by the source), with a romanized form where the report needs one. Ministers, deputy prime ministers
and party offices are outside this packet; the existing LDP president observations stay separate and unchanged.

Primary sources are required: the Prime Minister's Office and Cabinet (kantei.go.jp, including archived pages),
the minutes of the National Diet (kokkai.ndl.go.jp) and the two Houses' own records, the Official Gazette (官報)
where accessible without restriction, and the Imperial Household Agency. Constitution texts may establish
procedure only, never a date. Retrospective lists of cabinets are claims, never boundaries. News, encyclopaedias
and history sites are leads only. The historical cutoff stays 7 September 2026.

## Allowed files and checks

Allowed files:

- this record;
- a new `docs/campaign-certification/C01/research/japan-prime-ministers-1990-2006-12.md`;
- `docs/campaign-certification/C01/research/japan.json` and specifically related new
  `docs/campaign-certification/C01/research/sources/japan-*-facts.json` extracts;
- focused tests under `tools/avatars/`: a new `test_japan_prime_ministers_c01_12.py`, and pinned counts or exact
  sets in `test_japan_research_s10d.py` updated to the new totals (none loosened).

Put generated `research-index.json` changes in a separate commit. Do not change shared UI, the roadmap, game
data or other country packets.

Checks:

- research-index `--check`;
- the Japan, research and campaign Python tests;
- the atlas Node check;
- `workboard.py --check`;
- `git diff --check`.

Mark the packet `ready_for_review` when done. C01 and all parent gates stay open.

## Result

Submitted `ready_for_review` on 24 September 2026. Claim commit: `c2aafd22` (this record only). Report:
[japan-prime-ministers-1990-2006-12.md](../../campaign-certification/C01/research/japan-prime-ministers-1990-2006-12.md).
Touched paths (nothing else):

- `docs/planning/ai-handoffs/CLAUDE-C01-12.md` (this record);
- `docs/campaign-certification/C01/research/japan-prime-ministers-1990-2006-12.md` (new report);
- `docs/campaign-certification/C01/research/japan.json`;
- 119 new extracts `docs/campaign-certification/C01/research/sources/japan-*-facts.json`;
- `tools/avatars/test_japan_prime_ministers_c01_12.py` (new) and `tools/avatars/test_japan_research_s10d.py`;
- `docs/campaign-certification/C01/research-index.json` (regenerated, separate commit).

The packet gains 119 sources and 174 claims, one institution (`jp_prime_minister`, `executive_institution`, lifecycle
`unknown`, appended after the seven parliamentary groups) with one role (`jp_pm`, 内閣総理大臣 — Prime Minister of Japan,
`head_of_government`) and fourteen holder observations. Stated starts, each from a same-day Kantei statement or account
that names the holder (and in 2005 the Imperial Household Agency's photo page): 橋本龍太郎 (11 January 1996), 森喜朗
(5 April and 4 July 2000) and 小泉純一郎 (26 April 2001, 19 November 2003 and 21 September 2005). No stated end. Every other
holder is dated by `attested_on`: 海部俊樹 (19 January and 2 March 1990), 宮澤喜一 (8 November 1991), 細川護煕 (23 August
1993), 羽田孜 (10 May 1994), 村山富市 (18 July 1994), 橋本龍太郎 (8 November 1996) and 小渕恵三 (31 July 1998). Each House's
designation vote, the 1998 joint committee and the House of Representatives' prevailing resolution, the Imperial
appointment ceremonies, cabinet formations, resignations en masse, the acting prime ministers (橋本龍太郎 on 12 January
1990; 青木幹雄 from 3 April 2000), unnamed statements and ceremony records, members' references, recollections and the
Kantei's and the House of Councillors' retrospective lists are claims only; no successor's appointment is used as an end,
and no continued performance of duties under art. 71 was found. 安倍晋三's appointment of 26 September 2006 is recorded
only as the closing boundary; holders from then to the cutoff are outside this packet.

Observation decisions:

| ID | Decision |
|---|---|
| JP-PM-01 | Accepted in part: 海部俊樹 observed 19 Jan 1990 (acting Prime Minister's signature 12 Jan, a claim); resignation notice and both designations 27 Feb 1990; observed again 2 Mar 1990; no appointment day or end |
| JP-PM-02 | Accepted in part: notice 5 Nov 1991 (09:22 and 09:25) and both designations; 宮澤喜一 observed 8 Nov 1991; no appointment day or end |
| JP-PM-03 | Accepted in part: notice 5 Aug 1993, designations 6 Aug; 細川護煕 observed 23 Aug 1993; recollections of 9 and 13 Aug are claims |
| JP-PM-04 | Accepted in part: notice and designations 25 Apr 1994; 羽田孜 observed 10 May 1994 |
| JP-PM-05 | Accepted in part: Hata cabinet resigned 25 Jun 1994 (receipt 11:56); designations 29 Jun; 村山富市 observed 18 Jul 1994 |
| JP-PM-06 | Accepted in part: 橋本龍太郎 from 11 Jan 1996; re-designated 7 Nov 1996, whose statement prints no name, so observed 8 Nov 1996 (check B6, second option) |
| JP-PM-07 | Accepted in part: designations differed (小渕恵三, 菅直人), joint committee failed, art. 67(2) on 30 Jul 1998; 小渕恵三 observed 31 Jul 1998 |
| JP-PM-08 | Accepted: acting Prime Minister 青木幹雄 from 09:00 on 3 Apr 2000 (claims); art. 70 resignation 4 Apr; 森喜朗 from 5 Apr and 4 Jul 2000 |
| JP-PM-09 | Accepted: 小泉純一郎 from 26 Apr 2001, 19 Nov 2003 and 21 Sep 2005 |
| JP-PM-10 | Accepted in part: in office 25 Sep 2006; resignation notices and 安倍晋三's appointment 26 Sep 2006 (closing boundary); Koizumi's end not stated |

All 31 checker defects (A1-A11, B1-B11, C1-C9) are applied; B6 is resolved by its second option and C2 is applied in an
adapted form that keeps one holder string per person (check A4); the report's Checker defects table gives each outcome.
All confirmed missing primary records are imported (five of part A's six, with the relocated table recorded as the
capture's alternate location; all six of part B's; all sixteen of part C's). Each recorded identity was downloaded again by
this packet three times on 24 September 2026 (plain at 20:17Z-20:21Z, cache-busting on the live hosts at 20:38Z-20:42Z and plain at
20:53Z-20:56Z) with the same byte count and SHA-256, and the SHA-1 of every one of the 49 captures equals the
Internet Archive's CDX digest for that capture. The Official Gazette of 1990-2006 stayed unavailable without restriction;
nothing was bypassed.

Checks run on 24 September 2026 in this sparse worktree (`docs/campaign-certification/C01`, `docs/planning`,
`tools/avatars`, `tools/planning`, `tools/ui`; not widened), with `PYTHONDONTWRITEBYTECODE=1`:

- `python -X utf8 tools/avatars/campaign_research.py` then `--check`: exact regeneration passes; 9 packets,
  841 organization and 28 institution observations, 280 sources, 1,936 claims, 92 open batches.
- Japan tests (`-p "test_japan*.py"`): 17 pass (9 of them in the new `test_japan_prime_ministers_c01_12.py`, whose
  mutation test rejects 38 rule mutations, 10 validator mutations, 3 row mutations and 4 event collapses).
- Research tests (`-p "test_*research*.py"`): 79 pass.
- Campaign tests (`-p "test_campaign*.py"`): the 9 `test_campaign_research` tests pass; `test_campaign_census` errors in
  `setUpClass` because `spheres-sim/data/party_leaders.json` is not in this sparse worktree. It was not run, and the
  sparse checkout was not widened. These suites overlap; their counts are not summed.
- `node --test tools/ui/check_leadership_research_review.cjs`: 11 pass.
- `python tools/planning/workboard.py --check`: passes (44 markers).
- `git diff --check`: clean for this packet's paths.
- The new test rejects hand-made regressions both by rule and against the pinned list: a successor's appointment,
  ceremony or designation used or cited as an end (Koizumi 2005, Mori 2000, Kaifu 1990, Hosokawa); a same-person
  reappointment used as an end (Hashimoto 1996); a resignation notice or statement, the art. 70 resignation, a
  retrospective span or the last in-office interview used as an end; a designation date, a designation, the prevailing
  resolution or a retrospective list used or cited as a start; a member's recollection or a dated photo caption used as
  an observation; an unnamed statement or ceremony record used or cited as a start; a named start left off its holder;
  the acting Prime Minister (Aoki, or Hashimoto in 1990), continued duties (Hosokawa 1994) or the successor (Abe) added
  as a holder, or acting service or a member's reference cited by one; a span or list given a structured date or
  period; a second role, a second institution, the institution moved before the groups, a head-of-government role on an
  organization and holders out of order; acting, unnamed and boundary rows given a holder name; collapsed event dates;
  and checksum, path, reversed-interval and beyond-cutoff mutations.

C01 and all parent gates (C06, S23, WC1, CP1) stay open. No installed leader, avatar, portrait, campaign rule or save
schema changed.
