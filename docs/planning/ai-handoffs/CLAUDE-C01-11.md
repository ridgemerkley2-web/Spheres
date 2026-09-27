# CLAUDE-C01-11: Indian prime ministers, 1990–2026

Owner: Claude. State: **ready_for_review** (24 September 2026; submitted, not accepted). Parent: C01
(incomplete). Report: [india-prime-ministers-1990-2026-11.md](../../campaign-certification/C01/research/india-prime-ministers-1990-2026-11.md).

Origin: a self-proposed follow-up packet, started on the user's 23 September 2026 instruction to continue
development. The workboard asks for a distinct bounded packet that does not repeat accepted
C01-01/02/03/04/07/08 or reclaim the pending C01-05 (USSR/RSFSR), C01-06 (Saudi Arabia), C01-09 (South Africa)
and C01-10 (Brazil). This packet touches only the India research packet, which none of those change. It is
pending Codex acceptance and is not registered in `docs/planning/ai-workstreams.json`.

Branch: `claude/c01-in-11`. Base: `ffe54b02` (current `codex/campaign-certification`). Claim commit: `38f36fa2`
(this record only).

## Bounded deliverable

The India packet has party-recognition observations only. Add one executive institution,
`in_prime_minister`, with one role, `in_pm` (Prime Minister of India, kind `head_of_government`), and review at
most ten observations between 1 January 1990 and the 7 September 2026 cutoff:

1. The holder when the period opens (V. P. Singh), the 1990 confidence vote and his resignation.
2. The swearing-in of November 1990, the 1991 resignation and any request to continue in office.
3. The swearing-in of June 1991 and the 1996 resignation.
4. 1996: the May swearing-in, the resignation that followed, and the June swearing-in.
5. 1997–1998: the April 1997 confidence vote and resignation, the April 1997 swearing-in, the November 1997
   resignation and any continuation in office until March 1998.
6. 1998–1999: the March 1998 swearing-in, the April 1999 confidence vote, any continuation in office, and the
   October 1999 swearing-in.
7. 2004: the resignation and the May swearing-in.
8. 2009: the May swearing-in.
9. 2014 and 2019: the resignation and the two swearings-in.
10. 2024: the June swearing-in, and an official attestation of the holder in office before the cutoff, without
    extending any term.

Keep the President's appointment, the oath or swearing-in, any stated effective date, a confidence vote, a
resignation, the President's acceptance of it, a request to continue in office until other arrangements are
made, and the end of such continuation as distinct dated claims. Never infer an end from a successor's
swearing-in unless a source states it. Give a holder `from` only where a source states the day the appointment
took effect or office was assumed, and `until` only where a source states the day the office ended (for example
a resignation accepted with effect from a stated day); otherwise record `attested_on`. Service after a
resignation at the President's request is recorded as claims, never as a separate holder. Deputy prime
ministers, ministers, the President and party offices are outside this packet.

Primary sources are required: the President's Secretariat (Rashtrapati Bhavan communiqués), the Prime
Minister's Office, the Press Information Bureau (including its archive), Cabinet Secretariat notifications in
the Gazette of India, and the Lok Sabha and Rajya Sabha debates and records. Constitution texts may establish
procedure only, never a date. Retrospective lists of former prime ministers are claims, never boundaries. News,
encyclopaedias and history sites are leads only. The historical cutoff stays 7 September 2026.

## Allowed files and checks

Allowed files:

- this record;
- a new `docs/campaign-certification/C01/research/india-prime-ministers-1990-2026-11.md`;
- `docs/campaign-certification/C01/research/india.json` and specifically related new
  `docs/campaign-certification/C01/research/sources/india-*-facts.json` extracts;
- focused tests under `tools/avatars/`: a new `test_india_prime_ministers_c01_11.py`, and pinned counts or exact
  sets in `test_india_research_s10e.py` updated to the new totals (none loosened).

Put generated `research-index.json` changes in a separate commit. Do not change shared UI, the roadmap, game
data or other country packets.

Checks:

- research-index `--check`;
- the India, research and campaign Python tests;
- the atlas Node check;
- `workboard.py --check`;
- `git diff --check`.

Mark the packet `ready_for_review` when done. C01 and all parent gates stay open.

## Result

Submitted `ready_for_review` on 24 September 2026. Claim commit: `38f36fa2` (this record only). Touched paths
(nothing else):

- `docs/planning/ai-handoffs/CLAUDE-C01-11.md` (this record);
- `docs/campaign-certification/C01/research/india-prime-ministers-1990-2026-11.md` (new report);
- `docs/campaign-certification/C01/research/india.json`;
- 70 new extracts `docs/campaign-certification/C01/research/sources/india-*-facts.json`;
- `tools/avatars/test_india_prime_ministers_c01_11.py` (new) and `tools/avatars/test_india_research_s10e.py`;
- `docs/campaign-certification/C01/research-index.json` (regenerated, separate commit).

The packet gains 70 sources and 112 claims, one institution (`in_prime_minister`, `executive_institution`, lifecycle
`unknown`) with one role (`in_pm`, Prime Minister of India, `head_of_government`) and thirteen holder observations.
Stated starts, all from Ministry of Home Affairs notifications in the Gazette of India: P. V. Narasimha Rao (21 June
1991), Atal Bihari Vajpayee (16 May 1996, 19 March 1998, 13 October 1999) and I. K. Gujral (21 April 1997). Stated ends,
all from Gazette acceptances "with effect from" a day: Chandra Shekhar (21 June 1991), Rao (16 May 1996), H. D. Deve
Gowda (21 April 1997) and Vajpayee (13 October 1999). Every other holder is dated by `attested_on`: V. P. Singh
(12 March 1990), Chandra Shekhar (16 November 1990), Deve Gowda (11 June 1996), Manmohan Singh (22 May 2004 and 2009) and
Narendra Modi (26 May 2014, 30 May 2019 and 9 June 2024, with attestations of 23 June, 25 July and 30 August 2026).
Confidence votes, resignation announcements and letters, acceptances without an effective day, requests to continue,
service during a continuation, oath announcements, appointments of a Prime Minister-designate, the council's oaths and
retrospective PMO and PIB spans are claims only; no successor's swearing-in is used as an end.

Observation decisions:

| ID | Decision |
|---|---|
| IN-PM-01 | Accepted in part: V. P. Singh observed 12 Mar 1990; confidence vote lost 7 Nov 1990; no resignation, acceptance or end found |
| IN-PM-02 | Accepted in part: Chandra Shekhar observed 16 Nov 1990; resignation, acceptance and request to continue 6 Mar 1991; in office 4 Jun 1991; until 21 Jun 1991; appointment day not found |
| IN-PM-03 | Accepted in part: Rao from 21 Jun 1991 until 16 May 1996; the 1996 resignation letter and any continuation not found |
| IN-PM-04 | Accepted in part: Vajpayee from 16 May 1996, resignation announced 28 May; Deve Gowda observed 11 Jun 1996; no 1996 resignation instrument or Deve Gowda appointment record |
| IN-PM-05 | Accepted in part: Deve Gowda until 21 Apr 1997; Gujral from 21 Apr 1997; Gujral's end not stated (the printed 19 Mar 1998 date was withdrawn by corrigendum) |
| IN-PM-06 | Accepted: Vajpayee from 19 Mar 1998 until 13 Oct 1999, and from 13 Oct 1999; the 1999 vote, resignation and continuation as claims |
| IN-PM-07 | Accepted in part: Vajpayee's 13 May 2004 resignation and continuation; Manmohan Singh observed 22 May 2004; the 19 May 2004 appointment is not byte-stable |
| IN-PM-08 | Accepted in part: resignation and continuation 18 May 2009; appointment 20 May; observed 22 May 2009 |
| IN-PM-09 | Accepted in part: Modi observed 26 May 2014 and 30 May 2019; the 2014 and 2019 resignations and continuations as claims |
| IN-PM-10 | Accepted in part: resignation and continuation 5 Jun 2024; observed 9 Jun 2024; attested in office 23 Jun, 25 Jul and 30 Aug 2026 |

All 33 checker defects (A1-A14, B1-B9, C1-C10) are applied, one of them (A1) by withdrawing a claim that paraphrased
beyond its source; the report's Checker defects table gives each outcome. Of the confirmed missing primary records, 21 are
imported (three of them the 2019 captures that replace live pages); two redundant PIB captions and the 24 July 2026 council list are leads, and PIB relid 1734 (19 May 2004) is a
lead because its live page changes SHA-256 on every request and it has no capture. The PIB archive, the eGazette search
and the 1990 Gazette Nos. 15-16 stayed unreachable; nothing was bypassed.

Checks run on 24 September 2026 in this sparse worktree (`docs/campaign-certification/C01`, `docs/planning`,
`tools/avatars`, `tools/planning`, `tools/ui`; not widened), with `PYTHONDONTWRITEBYTECODE=1`:

- `python -X utf8 tools/avatars/campaign_research.py` then `--check`: exact regeneration passes; 9 packets,
  841 organization and 28 institution observations, 231 sources, 1,874 claims, 92 open batches.
- India tests (`-p "test_india*.py"`): 17 pass (9 of them new in `test_india_prime_ministers_c01_11.py`).
- Research tests (`-p "test_*research*.py"`): 79 pass.
- Campaign tests (`-p "test_campaign*.py"`): the 9 `test_campaign_research` tests pass; `test_campaign_census` errors in
  `setUpClass` because `spheres-sim/data/party_leaders.json` is not in this sparse worktree. It was not run, and the
  sparse checkout was not widened. These suites overlap; their counts are not summed.
- `node --test tools/ui/check_leadership_research_review.cjs`: 11 pass.
- `python tools/planning/workboard.py --check`: passes (44 markers).
- `git diff --check`: clean for this packet's paths.
- The new test rejects hand-made regressions both by rule and against the pinned list: a successor's swearing-in or
  appointment used as an end (V. P. Singh, Vajpayee 1996, Gujral, Vajpayee 1999, Manmohan Singh 2004 and 2009, Modi
  2019), including with the successor's oath cited; a resignation announcement, letter, acceptance or communique without
  an effective day used as an end (Chandra Shekhar, Vajpayee 1996, Deve Gowda, Gujral, Vajpayee 2004, Modi 2019); the
  superseded 1998 Gazette text used as Gujral's end; a lost confidence vote used as an end; a retrospective span used as
  a start or an end; a member's recollection used as an observation; a caretaker continuation added as a holder (Chandra
  Shekhar, Gujral, Vajpayee 1999, Modi 2024) or its request, attestation or a confidence vote cited by a holder; a
  swearing-in report or appointment communique used as a start; a Prime Minister-designate's appointment, an oath
  announcement, a council oath or a recollection cited by a holder; an invitation used as a start; a span given a
  structured date or period; a second role, a second institution or a head-of-government role on an organization; holders
  out of order; collapsed event dates; and checksum, path, reversed-interval and beyond-cutoff mutations.

C01 and all parent gates (C06, S23, WC1, CP1) stay open. No installed leader, avatar, portrait, campaign rule or save
schema changed.
