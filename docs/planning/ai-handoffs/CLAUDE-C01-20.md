# CLAUDE-C01-20: Indian National Congress presidents, 1990–2026

Owner: Claude. State: **ready_for_review** (25 September 2026; submitted, not accepted). Parent: C01
(incomplete). Report: [india-inc-presidents-1990-2026-20.md](../../campaign-certification/C01/research/india-inc-presidents-1990-2026-20.md).

Origin: a self-proposed follow-up packet, started on the user's 25 September 2026 instruction to start another
batch of five packets in parallel. It does not repeat accepted C01-01/02/03/04/07/08 or reclaim
the pending C01-05, C01-06 and C01-09 to C01-17. It is pending Codex acceptance and is not registered in `docs/planning/ai-workstreams.json`.

Branch: `claude/c01-in-20`. **Stacked on CLAUDE-C01-15** (`claude/c01-in-15` at `dd58a610`), which is
ready for review and not yet integrated, because both packets edit `india.json`: merge CLAUDE-C01-15 first. Claim
commit: `d19f756b` (this record only).

## Bounded deliverable

Add one party role, `in_inc_president` (President of the Indian National Congress, kind `party_leader`), to the existing `Indian National Congress` recognition observation (`in_eci_20240323_np_05`), and review at most ten observations between 1 January 1990 and the cutoff:

1. The Congress President when the period opens (Rajiv Gandhi), and the vacancy after his death in May 1991 only as source-stated facts.
2. 1991–1992: P. V. Narasimha Rao's selection and any confirmation by the All India Congress Committee.
3. 1996: Sitaram Kesri's selection.
4. 1998: Sonia Gandhi's selection by the Congress Working Committee and its ratification.
5. Sonia Gandhi's organisational elections and extensions (2000 onward).
6. 2017: Rahul Gandhi's election and assumption of the office.
7. 2019: Rahul Gandhi's resignation and Sonia Gandhi's appointment as interim President.
8. 2022: the organisational election's result and Mallikarjun Kharge's assumption of the office.
9. Any stated end of a presidency where a source states it.
10. An attestation by the party of its President before the cutoff.

Keep a Working Committee decision, an All India Congress Committee ratification, the Central Election Authority's declaration of results, the assumption of charge, a resignation and interim arrangements as distinct dated claims. Party office and government office stay separate: no `in_prime_minister` or `in_presidency` claim may feed the party role or the reverse. The recognition observation's identity, lifecycle and game mapping do not change. Never infer an end from a successor's start unless a source states it. Give a holder `from` only where a source states the day office was assumed or took effect, and `until` only where a source states the day the office ended; otherwise record `attested_on`. Constitution and statute texts may establish procedure only, never a date. Retrospective lists are claims, never boundaries. News, encyclopaedias and history sites are leads only. The historical cutoff stays 7 September 2026.

Primary sources are required: the Indian National Congress's own records (inc.in and aicc.org.in, including archived pages, press releases, resolutions and Central Election Authority notices), and official Parliament or Election Commission records only where they record the party office.

## Allowed files and checks

Allowed files:

- this record;
- a new `docs/campaign-certification/C01/research/india-inc-presidents-1990-2026-20.md`;
- `docs/campaign-certification/C01/research/india.json` and specifically related new
  `docs/campaign-certification/C01/research/sources/india-*-facts.json` extracts;
- focused tests under `tools/avatars/`: a new `test_india_inc_presidents_c01_20.py`, and pinned counts or exact sets in `test_india_research_s10e.py`, `test_india_prime_ministers_c01_11.py` and `test_india_presidents_c01_15.py`
  updated to the new totals (none loosened).

Put generated `research-index.json` changes in a separate commit. Do not change shared UI, the roadmap, game
data or other country packets.

Checks: research-index `--check`; the India, research and campaign Python tests; the atlas Node check;
`workboard.py --check`; `git diff --check`.

Mark the packet `ready_for_review` when done. C01 and all parent gates stay open.

## Result

Submitted `ready_for_review` on 25 September 2026 (UTC). Claim commit: `d19f756b` (this record only). Stack: branch
`claude/c01-in-20` is stacked on `claude/c01-in-15` at `dd58a610` (CLAUDE-C01-15, ready for review, not yet integrated),
which is stacked on `claude/c01-in-11` at `538920f1`; `origin/claude/c01-in-15` had no commits beyond `dd58a610`, so no
merge was needed. Merge CLAUDE-C01-11 and CLAUDE-C01-15 first. Touched paths (nothing else):

- `docs/planning/ai-handoffs/CLAUDE-C01-20.md` (this record);
- `docs/campaign-certification/C01/research/india-inc-presidents-1990-2026-20.md` (new report);
- `docs/campaign-certification/C01/research/india.json` (additions only: 73 sources after the C01-15 sources; on the
  Indian National Congress recognition observation `in_eci_20240323_np_05` the role `in_inc_president`, its 73 sources
  and 113 claim ids and one coverage note; one packet coverage note after the C01-15 note);
- 73 new extracts `docs/campaign-certification/C01/research/sources/india-*-facts.json` (no existing extract edited);
- `tools/avatars/test_india_inc_presidents_c01_20.py` (new), and `tools/avatars/test_india_research_s10e.py`,
  `tools/avatars/test_india_prime_ministers_c01_11.py` and `tools/avatars/test_india_presidents_c01_15.py` (pinned values
  only, none loosened);
- `docs/campaign-certification/C01/research-index.json` (regenerated, separate commit; the only file shared with other
  pending packets).

The packet gains 73 sources and 113 claims and one party role (`in_inc_president`, President of the Indian National
Congress, `party_leader`) on the existing recognition observation, whose identity, recognition row, lifecycle, coverage
status and empty game mapping are unchanged. The role has ten holder observations, each dated by a same-day in-office
attestation that names the holder, and none has a start or an end, because no record reviewed states the day any
President assumed or left the office: Rajiv Gandhi (29 August 1990), P. V. Narasimha Rao (15 July 1996), Sitaram Kesri
(11 April 1997), Sonia Gandhi (15 May 1999, 26 November 2000 and 29 April 2017), Rahul Gandhi (16 December 2017 and 25 May
2019) and Mallikarjun Kharge (26 October 2022 and 3 September 2026). Working Committee decisions and schedules, AICC
sessions and the recollection of a ratification, the Central Election Authority's notices, polls, counts and
declarations, certificates, President-elect stylings, acceptances, handover statements, resignations and their
rejection or withdrawal, farewells, predecessor references, a death, posthumous references, retrospective spans, lists and
histories, and records naming nobody are claims only. Sonia Gandhi's service from 10 August 2019 to 26 October 2022 was the
Working Committee's interim arrangement and is recorded as claims only. The thirteen CLAUDE-C01-11 prime-minister holders
and eight CLAUDE-C01-15 president holders are unchanged, and no claim or source is shared between the party role and
either institution.

Observation decisions:

| ID | Decision |
|---|---|
| INC-PRES-01 | Accepted in part: Rajiv Gandhi observed 29 Aug 1990 (Rajya Sabha, English); death 21 May 1991 stated; no vacancy, acting President or end stated |
| INC-PRES-02 | Accepted in part: no record of Rao's 1991 selection or an AICC confirmation; retrospective span and session list; observed 15 Jul 1996 |
| INC-PRES-03 | Accepted in part: Rao still in office 10 Sep 1996; Kesri's 1996 selection year-only; Kesri observed 11 Apr 1997 |
| INC-PRES-04 | Accepted in part: no record of the March 1998 Working Committee selection; unnamed AICC remarks of 6 Apr 1998; Sonia Gandhi observed 15 May 1999 |
| INC-PRES-05 | Accepted in part: 2000 election and declaration; observed 26 Nov 2000 and 29 Apr 2017; 2005 and 2010 elections claims only; 2015-2016 extensions not found |
| INC-PRES-06 | Accepted in part: schedules, declaration 11 Dec, President-elect 13 Dec; observed 16 Dec 2017; no stated assumption |
| INC-PRES-07 | Accepted in part: Rahul Gandhi observed 25 May 2019; resignation stated without a day; interim President requested and accepted 10 Aug 2019 (claims) |
| INC-PRES-08 | Accepted in part: 2021-2022 schedules, notification, scrutiny, final list, poll, count and declaration; Kharge observed 26 Oct 2022; handover statements are claims; no ratification found |
| INC-PRES-09 | Unresolved: no stated end; resignations, withdrawal, farewell, predecessor references, the 2022 handover of interim service and a plenary resolution are claims only |
| INC-PRES-10 | Accepted: Kharge styled Congress President 3 Sep 2026; an unnamed act 7 Sep 2026 |

All 44 checker defects (A1-A11, B1-B19, C1-C14) have an outcome in the report's Checker defects table: 39 applied, three
applied in part (A2 and A4: some confirmed Lok Sabha copies declined; B3: no exception for 2005 and 2010) and two resolved
by removal (B2, the 2017 Sonia Gandhi holder; C3, the post-cutoff page data). Of the 30 confirmed missing primary records,
23 are imported (among them the Rajya Sabha store copies that replace the dossier's Internet Archive copies, the English
Rajya Sabha records that date Rajiv Gandhi and Rao, the Lok Sabha pages that date and recall Kesri, the INC's 2017, 2020,
2021 and 2022 records and the 85th Plenary resolution) and seven are declined with reasons (Lok Sabha translations or
Internet Archive copies that add only continuations, one record naming nobody, and one ambiguous Rajya Sabha passage). This
packet re-downloaded every recorded response between 14:47Z and 15:00Z on 25 September 2026 with identical bytes, the new
records at least 30 minutes after their check's first download. The party's 2022 biography capture of Rajiv Gandhi
answered 404 on six attempts and is a lead. The Parliament Digital Library, the Lok Sabha and Rajya Sabha sites and the
PIB archive stayed unreachable; nothing was bypassed, no terms were accepted and no login was used.

Checks run on 25 September 2026 in this sparse worktree (`docs/campaign-certification/C01`, `docs/planning`,
`tools/avatars`, `tools/planning`, `tools/ui`; not widened), with `PYTHONDONTWRITEBYTECODE=1`:

- `python -X utf8 tools/avatars/campaign_research.py` then `--check`: exact regeneration passes; 9 packets, 841
  organization and 29 institution observations, 360 sources, 2,071 claims, 92 open batches.
- India tests (`-p "test_india*.py"`): 36 pass (10 of them new in `test_india_inc_presidents_c01_20.py`).
- Research tests (`-p "test_*research*.py"`): 79 pass.
- Campaign tests (`-p "test_campaign*.py"`): the 9 `test_campaign_research` tests pass; `test_campaign_census` errors in
  `setUpClass` because `spheres-sim/data/party_leaders.json` is not in this sparse worktree. It was not run, and the
  sparse checkout was not widened. These suites overlap; their counts are not summed.
- `node --test tools/ui/check_leadership_research_review.cjs`: 11 pass.
- `python tools/planning/workboard.py --check`: passes (44 markers).
- `git diff --check`: clean for this packet's paths.
- The new test rejects hand-made regressions both by rule and against the pinned list: a successor's observation or
  claim used as an end; an end invented at the cutoff; a death, a farewell, a resignation, a resignation statement, a
  stepping-down resolution or a retrospective span used as an end; an election, a declaration, a retrospective sketch's
  candidate start, a recollection or a handover statement used as a start; a President-elect styling, a certificate or a
  session list used as an observation; an election result, an unnamed record or a continuation claim cited by a holder;
  interim service (with or without an end) or a continuation attestation added as a holder; a prime minister or a
  President of India added as a party President, a party President moved into the prime-ministership or the presidency, a
  prime-ministership claim cited by a party President, a party claim moved onto the prime-ministership, the role copied to
  another party or an institution, a second role, a changed role kind, a lifecycle start, a game mapping or an upgraded
  coverage status on the observation; a span given a structured date or a period; holders out of order; collapsed event
  dates; and checksum, path, reversed-interval and beyond-cutoff mutations.

C01 and all parent gates (C06, S23, WC1, CP1) stay open. No installed leader, avatar, portrait, campaign rule or save
schema changed.
