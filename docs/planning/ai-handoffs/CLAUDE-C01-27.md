# CLAUDE-C01-27: Bharatiya Janata Party presidents, 1990–2026

Owner: Claude. State: **ready_for_review** (28 September 2026; submitted, not accepted). Parent: C01
(incomplete). Report: [india-bjp-presidents-1990-2026-27.md](../../campaign-certification/C01/research/india-bjp-presidents-1990-2026-27.md).

Origin: a self-proposed follow-up packet, started on the user's 25 September 2026 instruction to start another
batch of five packets in parallel. It does not repeat accepted C01-01/02/03/04/07/08 or reclaim
the pending C01-05, C01-06 and C01-09 to C01-22. It is pending Codex acceptance and is not registered in `docs/planning/ai-workstreams.json`.

Branch: `claude/c01-in-27`. Claimed while **stacked on CLAUDE-C01-20** (`claude/c01-in-20` at `2ee3b146`), because
both packets edit `india.json`; CLAUDE-C01-20 has since been integrated into `codex/campaign-certification`, and this
branch has merged current integration (`da49f9c0`, `26a07bb8` and `29f1937e`, the last at `e41aa18d`), so nothing needs
merging first. Claim commit: `27c52dc4` (this record only).

## Bounded deliverable

Add one party role, `in_bjp_president` (National President of the Bharatiya Janata Party, kind `party_leader`), to the existing `Bharatiya Janata Party` recognition observation, and review at most ten observations between 1 January 1990 and the cutoff:

1. The President when the period opens (L. K. Advani) and the 1991 change.
2. Murli Manohar Joshi's presidency.
3. Advani's return (1993).
4. Kushabhau Thakre (1998) and Bangaru Laxman (2000).
5. K. Jana Krishnamurthi (2001) and M. Venkaiah Naidu (2002).
6. Advani (2004) and Rajnath Singh (2005).
7. Nitin Gadkari (2009) and Rajnath Singh (2013).
8. Amit Shah (2014, re-elected 2016).
9. J. P. Nadda (working President 2019, President 2020, extensions).
10. The President at the cutoff, and a party attestation before it.

Keep the National Council's or organisational election's result, the assumption of charge, working or interim presidencies, resignations and extensions as distinct dated claims; a working president is a claim only unless a source states a substantive presidency. Party office and the prime-ministership and presidency stay separate both ways, and the INC role from C01-20 does not change. Never infer an end from a successor's start unless a source states it. Give a holder `from` only where a source states the day office was assumed or took effect, and `until` only where a source states the day the office ended; otherwise record `attested_on`. Constitution and statute texts may establish procedure only, never a date. Retrospective lists are claims, never boundaries. News, encyclopaedias and history sites are leads only. The historical cutoff stays 7 September 2026.

Primary sources are required: the BJP's own records (bjp.org, including archived pages, press releases and National Council records), and official Parliament or Election Commission records only where they record the party office.

## Allowed files and checks

Allowed files:

- this record;
- a new `docs/campaign-certification/C01/research/india-bjp-presidents-1990-2026-27.md`;
- `docs/campaign-certification/C01/research/india.json` and specifically related new
  `docs/campaign-certification/C01/research/sources/india-*-facts.json` extracts;
- focused tests under `tools/avatars/`: a new `test_india_bjp_presidents_c01_27.py`, and pinned counts or exact sets in `test_india_research_s10e.py`, `test_india_prime_ministers_c01_11.py`, `test_india_presidents_c01_15.py` and `test_india_inc_presidents_c01_20.py`
  updated to the new totals (none loosened).

Put generated `research-index.json` changes in a separate commit. Do not change shared UI, the roadmap, game
data or other country packets.

Checks: research-index `--check`; the India, research and campaign Python tests; the atlas Node check;
`workboard.py --check`; `git diff --check`.

Mark the packet `ready_for_review` when done. C01 and all parent gates stay open.

## Result

Submitted `ready_for_review` on 28 September 2026 (UTC). Claim commit: `27c52dc4` (this record only). Base:
`codex/campaign-certification` at `e41aa18d`, merged into the branch at `29f1937e` (earlier merges: `da49f9c0` at
`a33a8987` and `26a07bb8` at `01d5217c`). CLAUDE-C01-20 is integrated there, so no pending packet needs merging first.
Touched paths (nothing else):

- `docs/planning/ai-handoffs/CLAUDE-C01-27.md` (this record);
- `docs/campaign-certification/C01/research/india-bjp-presidents-1990-2026-27.md` (new report);
- `docs/campaign-certification/C01/research/india.json` (additions only: 74 sources after the C01-20 sources; on the
  Bharatiya Janata Party recognition observation `in_eci_20240323_np_03` the role `in_bjp_president`, its 74 sources and
  167 claim ids and one coverage note; one packet coverage note after the C01-20 note);
- 74 new extracts `docs/campaign-certification/C01/research/sources/india-*-facts.json` (no existing extract edited);
- `tools/avatars/test_india_bjp_presidents_c01_27.py` (new), and `tools/avatars/test_india_research_s10e.py`,
  `tools/avatars/test_india_prime_ministers_c01_11.py`, `tools/avatars/test_india_presidents_c01_15.py` and
  `tools/avatars/test_india_inc_presidents_c01_20.py` (pinned counts, exact sets, source lists, hosts and access dates
  only, re-expressed exactly; none loosened and no assertion removed);
- `docs/campaign-certification/C01/research-index.json` (regenerated, separate commit; the only file shared with other
  pending packets).

The packet gains 74 sources and 167 claims and one party role (`in_bjp_president`, National President of the Bharatiya
Janata Party, `party_leader`) on the existing recognition observation, whose identity, recognition row, lifecycle,
coverage status and empty game mapping are unchanged. The role has nineteen holder observations: L. K. Advani (observed
8 January 1991, 16 July 1997 and 20 October 2004), Murli Manohar Joshi (10 December 1991), Kushabhau Thakre (27 February
1999), Bangaru Laxman (observed 31 August 2000; stated end 14 March 2001, when the office bearers accepted his
resignation 'with immediate effect'), M. Venkaiah Naidu (stated start 1 July 2002, from the party journal), Rajnath
Singh (2 January 2006 and 2 March 2013), Nitin Gadkari (24 December 2009 and 22 January 2013), Amit Shah (9 July 2014, 2
February 2016 and 17 June 2019), J. P. Nadda (stated start 20 January 2020, from the party organ; observed 17 January
2023 and 15 December 2025) and Nitin Nabin (stated start 20 January 2026, from the party's same-day release and the
party organ; observed 17 August 2026). National Council and National Executive resolutions under multi-day headings,
election steps (schedules, nominations, scrutiny, sole-candidate announcements, results, declarations, certificates),
President-elect stylings, acceptances, endorsements and entrustments, an appointment awaiting ratification, the acting
presidency of March 2001, the working presidencies of 2019 and 2025, a term extension, handover statements, farewells,
'outgoing', 'former' and 'Ex' stylings, resignations and their consideration, acceptance without a day of effect,
rejection or recording, retrospective spans, lists and recollections, and addresses that name no speaker are claims
only. The thirteen prime-minister, eight president and ten Congress-president holders are unchanged, and no claim or
source is shared between the party role and any other role or institution.

Decisions for the reviewer:

- K. Jana Krishnamurthi's service from 14 March 2001 stays claims only (check B6). The office bearers made him acting
  president on 14 March 2001. On 24 March 2001 the party heads his address 'National President' and he says the
  Executive has 'entrusted the responsibility of party presidentship' to him, but the same address places him in the
  Chair because of the office bearers' request, and no source states that the acting arrangement ended or that he was
  elected or appointed President; like the INC's interim presidency of 2019-2022, his service is claims only.
- Two holders (Advani and Joshi in 1991) are dated by English Rajya Sabha records, because the party's own records of
  1990-1992 print multi-day meeting headings or name no speaker; the undated Rajya Sabha page of 8 January 1991 is dated
  by the store file of the same printed sheet, recorded as a date anchor.
- Starts declined: Advani 2004 ('two days ago', a recollection), Rajnath Singh 2006 (present-tense acceptance, undated
  takeover), Thakre 1998 ('I assume this office', no printed day). Ends declined: every handover, farewell, predecessor
  reference, resignation accepted without a day of effect (Naidu 2004) and decision not to seek a second term (Gadkari
  2013).
- The 2016 compilations' section headings never name a speaker, because they contradict each other for the same 1991
  farewell.

Observation decisions:

| ID | Decision |
|---|---|
| BJP-PRES-01 | Accepted in part: Advani observed 8 Jan 1991 (Rajya Sabha, English); 1990 resolutions under multi-day headings; the 1991 change has no day |
| BJP-PRES-02 | Accepted in part: Joshi observed 10 Dec 1991 (Rajya Sabha, English); continuations 1992-1993; 'Ex-President' by December 1993; no election record |
| BJP-PRES-03 | Accepted in part: unnamed 1993 and 1995 election statements; Advani observed 16 Jul 1997; handover announced 2 May 1998 for the next day |
| BJP-PRES-04 | Accepted in part: Thakre elected unanimously and President-elect 2 May 1998, observed 27 Feb 1999; Laxman observed 31 Aug 2000, stated end 14 Mar 2001 |
| BJP-PRES-05 | Accepted in part: Jana Krishnamurthi acting from 14 Mar 2001, styled National President 24 Mar 2001 and 24 Jun 2002 (claims only); Naidu stated start 1 Jul 2002 |
| BJP-PRES-06 | Accepted in part: Naidu's resignation accepted 18 Oct 2004 (no day of effect); Advani observed 20 Oct 2004; endorsement 27 Oct 2004; Rajnath Singh observed 2 Jan 2006 |
| BJP-PRES-07 | Accepted in part: Gadkari observed 24 Dec 2009 and 22 Jan 2013; Rajnath Singh declared 23 Jan 2013 and observed 2 Mar 2013 |
| BJP-PRES-08 | Accepted in part: Amit Shah observed 9 Jul 2014, 2 Feb 2016 and 17 Jun 2019; re-election, continuation and handover are claims |
| BJP-PRES-09 | Accepted in part: Nadda Working President 17 Jun 2019 (claims), stated start 20 Jan 2020, observed 17 Jan 2023 and 15 Dec 2025 |
| BJP-PRES-10 | Accepted: Nabin Working President 14-15 Dec 2025 (claims), stated start 20 Jan 2026, observed 17 Aug 2026 |

All 65 checker defects (A1-A17, B1-B16, C1-C32) have an outcome in the report's Checker defects table: 62 applied and
three resolved by removal (A10, the Calcutta address that says nothing about the office; B13, the misread encoding notes;
C14, the post-cutoff Kamal Sandesh post). Of the 38 confirmed missing primary records, 24 are imported (among them the
Vol-4 and Vol-6 resolutions, the Hindi Rajya Sabha answer of 4 March 1992, BJP Today's editorial of July 2002 and report
of November 2004, the 1999 and 2000 statements that date Thakre and Laxman, the National Returning Officer's notice and
statement of January 2026, the party's release of 20 January 2026, four Kamal Sandesh issues stored on www.bjp.org and
the pre-cutoff captures that replace the live WordPress API responses) and 14 are declined with reasons (a record naming
nobody, finding aids whose articles are unarchived, redundant recollections and continuations, and API captures
superseded by the stored issues). Every recorded response was downloaded again twice by this packet on 28 September
2026 (between 01:39Z and 02:55Z), each pair at least 30 minutes apart, and the date anchor three times, always with the
recorded byte count and SHA-256 and no Content-Encoding. The live www.bjp.org, the Parliament Digital Library and the Library of
Congress web archive stayed unreachable or challenged; nothing was bypassed, no terms were accepted and no login was
used.

Checks run on 28 September 2026 in this sparse worktree (`docs/campaign-certification/C01`, `docs/planning`,
`spheres-sim/data`, `spheres-sim/src`, `spheres-web/data`, `tools/avatars`, `tools/planning`, `tools/ui`; not widened),
with `PYTHONDONTWRITEBYTECODE=1`:

- `python -X utf8 tools/avatars/campaign_research.py` then `--check`: exact regeneration passes; 9 packets, 841
  organization and 34 institution observations, 1,403 sources, 3,898 claims, 93 discovery batches (India: four role
  observations, 558 claims, 84 entries pending mapping).
- `python -X utf8 tools/avatars/campaign_census.py --check`: passes (exit 0, `"check": true`).
- India tests (`-p "test_india*.py"`): 46 pass (10 of them new in `test_india_bjp_presidents_c01_27.py`).
- Research tests (`-p "test_*research*.py"`): 79 pass.
- Campaign tests (`-p "test_campaign*.py"`): 16 pass, `test_campaign_census` included. These suites overlap; their
  counts are not summed.
- `node --test tools/ui/check_leadership_research_review.cjs`: 11 pass.
- `python tools/planning/workboard.py --check`: passes (44 canonical markers).
- `git diff --check`: clean for this packet's paths.
- The new test rejects hand-made regressions both by rule and against the pinned list: a successor's observation or
  start used as an end; an end invented at the cutoff; a handover statement, an 'Ex' styling, a farewell, an announced
  handover, a resignation accepted without a day of effect, a rejected resignation, a decision not to seek a second
  term, a 'former' or 'outgoing' styling or the tender of a resignation used as an end; a stated end removed or dropped;
  a declaration, a recollection, an appointment, a profile span, an undated assumption or a retrospective list used as a
  start; a stated start replaced by a continuation; a President-elect or 'Newly Elected' styling used as an
  observation; an election result, a continuation or an extension cited by a holder; a multi-day meeting or an unnamed
  address added as a holder; acting or working service added as a holder or used as a start; a prime minister, a
  President of India or a Congress President added as a BJP President, a BJP President moved into the
  prime-ministership or added to the presidency or the Congress role, a prime-ministership claim cited by a party
  President, a party claim moved onto the prime-ministership or the Congress role, the role copied to another party or
  an institution, a second role, a changed role kind, a lifecycle start, a game mapping or an upgraded coverage status on
  the observation; a span or a multi-day meeting given a structured date or a period; holders out of order; collapsed
  event dates; and checksum, path, reversed-interval, unknown-claim, cited-source, foreign-mapping and beyond-cutoff
  mutations.

C01 and all parent gates (C06, S23, WC1, CP1) stay open. No installed leader, avatar, portrait, campaign rule or save
schema changed.
