# CLAUDE-C01-15: Indian presidents, 1990–2026

Owner: Claude. State: **ready_for_review** (25 September 2026; submitted, not accepted). Parent: C01
(incomplete). Report: [india-presidents-1990-2026-15.md](../../campaign-certification/C01/research/india-presidents-1990-2026-15.md).

Origin: a self-proposed follow-up packet, started on the user's 24 September 2026 instruction to continue
development faster with several packets in parallel. It does not repeat accepted C01-01/02/03/04/07/08 or reclaim
the pending C01-05 (USSR/RSFSR), C01-06 (Saudi Arabia), C01-09 (South Africa), C01-10 (Brazil), C01-11 (India) and C01-12 (Japan, 1990-2006). It is pending Codex acceptance and is not registered in `docs/planning/ai-workstreams.json`.

Branch: `claude/c01-in-15`. **Stacked on CLAUDE-C01-11** (`claude/c01-in-11` at `538920f1`), which is
ready for review and not yet integrated, because both packets edit `india.json`: merge CLAUDE-C01-11 first. Claim
commit: `f755794c` (this record only).

## Bounded deliverable

Add a second executive institution, `in_presidency`, with one role, `in_president` (President of India, kind `head_of_state`), separate from C01-11's `in_prime_minister`, and review at most ten observations between 1 January 1990 and the cutoff:

1. The holder when the period opens (R. Venkataraman) and the end of his term only if a source states it.
2. 1992: the election result and Shankar Dayal Sharma's oath and assumption of office.
3. 1997: the election result and K. R. Narayanan's oath and assumption of office.
4. 2002: the election result and A. P. J. Abdul Kalam's oath and assumption of office.
5. 2007: the election result and Pratibha Patil's oath and assumption of office.
6. 2012: the election result and Pranab Mukherjee's oath and assumption of office.
7. 2017: the election result and Ram Nath Kovind's oath and assumption of office.
8. 2022: the election result and Droupadi Murmu's oath and assumption of office.
9. The stated ends of terms, where a source states them.
10. An official attestation of the holder in office before the cutoff.

Keep the Returning Officer's or Election Commission's declaration of the result, the oath administered by the Chief Justice, a stated assumption of office and a stated end as distinct dated claims. Vice-presidents acting as President are claims only. Do not change the C01-11 prime-minister holders. Never infer an end from a successor's start unless a source states it. Give a holder `from` only where a source states the day office was assumed or took effect, and `until` only where a source states the day the office ended; otherwise record `attested_on`. Constitution and statute texts may establish procedure only, never a date. Retrospective lists are claims, never boundaries. News, encyclopaedias and history sites are leads only. The historical cutoff stays 7 September 2026.

Primary sources are required: the President's Secretariat (Rashtrapati Bhavan), the Election Commission of India, the Press Information Bureau (including its archive), notifications in the Gazette of India, and the Lok Sabha and Rajya Sabha records.

## Allowed files and checks

Allowed files:

- this record;
- a new `docs/campaign-certification/C01/research/india-presidents-1990-2026-15.md`;
- `docs/campaign-certification/C01/research/india.json` and specifically related new
  `docs/campaign-certification/C01/research/sources/india-*-facts.json` extracts;
- focused tests under `tools/avatars/`: a new `test_india_presidents_c01_15.py`, and pinned counts or exact sets in `test_india_research_s10e.py` and `test_india_prime_ministers_c01_11.py`
  updated to the new totals (none loosened).

Put generated `research-index.json` changes in a separate commit. Do not change shared UI, the roadmap, game
data or other country packets.

Checks: research-index `--check`; the India, research and campaign Python tests; the atlas Node check;
`workboard.py --check`; `git diff --check`.

Mark the packet `ready_for_review` when done. C01 and all parent gates stay open.

## Result

Submitted `ready_for_review` on 25 September 2026 (UTC). Claim commit: `f755794c` (this record only). Stack: branch
`claude/c01-in-15` is stacked on `claude/c01-in-11` at `538920f1` (CLAUDE-C01-11, ready for review, not yet integrated);
`origin/claude/c01-in-11` had no commits beyond `538920f1`, so no merge was needed. Merge CLAUDE-C01-11 first. Touched
paths (nothing else):

- `docs/planning/ai-handoffs/CLAUDE-C01-15.md` (this record);
- `docs/campaign-certification/C01/research/india-presidents-1990-2026-15.md` (new report);
- `docs/campaign-certification/C01/research/india.json` (additions only: 56 sources, the `in_presidency` institution
  after `in_prime_minister`, and one packet coverage note after the C01-11 note);
- 56 new extracts `docs/campaign-certification/C01/research/sources/india-*-facts.json`;
- `tools/avatars/test_india_presidents_c01_15.py` (new), `tools/avatars/test_india_research_s10e.py` and
  `tools/avatars/test_india_prime_ministers_c01_11.py` (pinned values only);
- `docs/campaign-certification/C01/research-index.json` (regenerated, separate commit; the only file shared with other
  pending packets).

The packet gains 56 sources and 84 claims, one institution (`in_presidency`, `executive_institution`, lifecycle
`unknown`) with one role (`in_president`, President of India, `head_of_state`) and eight holder observations. The
thirteen C01-11 prime-minister holders are unchanged. Holders, in order: R. Venkataraman (`attested_on` 13 January 1990,
his own Order); Shankar Dayal Sharma (`from` 25 July 1992), K. R. Narayanan (`from` 25 July 1997), Pratibha Devisingh
Patil (`from` 25 July 2007), Pranab Mukherjee (`from` 25 July 2012), Ram Nath Kovind (`from` 25 July 2017) and Droupadi
Murmu (`from` 25 July 2022), each from the Ministry of Home Affairs resolution that the President takes his or her seat
and its proclamation that he or she has entered upon the office; and A. P. J. Abdul Kalam (`from` 25 July 2002, his own
same-day speech on assumption of office, with the 2002 corrigendum's recital as corroboration). No holder has `until`: no
primary source reviewed states the day any President's office ended. The Election Commission's election notices and
term-expiry notices, the Returning Officers' declarations and their publication, the ceremony programmes, farewells,
same-day stylings of a predecessor as former President, retrospective spans and recollections, the Lok Sabha seat notice
of 2012, the PMO's report of the 2022 oath and the name corrigenda are claims only; no successor's assumption of office is
used as an end. The oath administered by the Chief Justice stays a distinct claim (M.H. Kania 1992, J.S. Khehar 2017,
N.V. Ramana 2022; unnamed in 2002 and 2012; recalled only later in 2007; not recorded in 1997).

Observation decisions:

| ID | Decision |
|---|---|
| IN-PRES-01 | Accepted in part: Venkataraman observed 13 Jan 1990; term due to expire 24 Jul 1992 (prospective); no stated end |
| IN-PRES-02 | Accepted: poll dates 10 Jun, declaration 16 Jul (published 17 Jul), programme 24 Jul, oath by M.H. Kania and resolution and proclamation 25 Jul 1992 |
| IN-PRES-03 | Accepted in part: poll dates 9 Jun, declaration 17 Jul (published 22 Jul), programme 24 Jul, resolution and proclamation 25 Jul 1997; the oath itself not recorded |
| IN-PRES-04 | Accepted in part: declaration 18 Jul 2002 (89.58 per cent); own address on assumption 25 Jul 2002; S.O. 788(E), the Chief Justice and the hour not found |
| IN-PRES-05 | Accepted in part: declaration 21 Jul, programme 24 Jul, resolution and proclamation 25 Jul 2007; the oath only in a February 2008 caption |
| IN-PRES-06 | Accepted: declaration 22 Jul, programme 24 Jul, resolution, proclamation, address and oath caption 25 Jul 2012 |
| IN-PRES-07 | Accepted: declaration 20 Jul, programme 24 Jul, resolution and proclamation, and oath by J.S. Khehar 25 Jul 2017; name corrigendum 4 Aug 2017 |
| IN-PRES-08 | Accepted: declaration 21 Jul (published 22 Jul), programme 24 Jul, resolution and proclamation, and oath by N.V. Ramana 25 Jul 2022 |
| IN-PRES-09 | Unresolved: no stated end; seven term-expiry notices, three farewells, the eve of demitting office and three stylings as former President are claims only |
| IN-PRES-10 | Accepted: Murmu signs an ordinance 5 Jun 2026 and addresses the nation 14 Aug 2026 |

All 33 checker items (A1-A10, B1-B10, C1-C13) have an outcome in the report's Checker defects table: 27 applied, one
resolved by removal (A1, the statute-text claim), one applied in part (C1: the 29 July 2022 recollection is off the holder
and renamed, but keeps the day it recalls as the stacked packet does for recollections), one already recorded (C11, the
1992 notice) and three declined (A10, the pre-period 1987 Gazette; B9 and B10, the Election Commission's 2002 finding-aid
page and its copy, which carry no claim). Of the confirmed missing primary records, six are imported (the 25 July 1997
Lok Sabha Debates, the 1997 notice O.N. 65(E), the 2017 corrigendum, PIB's 2017 photo gallery, the Secretariat's 2022
photo gallery and PIB's release of Kovind's 2022 farewell); this packet re-downloaded each of them at least 30 minutes after
the check's first download with identical bytes (the five besides the corrigendum also at least 30 minutes after the
check's last). The Election Commission site (HTTP 406), the Parliament Digital Library and
the PIB archive stayed unreachable; nothing was bypassed, no terms were accepted and no login was used.

Checks run on 25 September 2026 in this sparse worktree (`docs/campaign-certification/C01`, `docs/planning`,
`tools/avatars`, `tools/planning`, `tools/ui`; not widened), with `PYTHONDONTWRITEBYTECODE=1`:

- `python -X utf8 tools/avatars/campaign_research.py` then `--check`: exact regeneration passes; 9 packets, 841
  organization and 29 institution observations, 287 sources, 1,958 claims, 92 open batches.
- India tests (`-p "test_india*.py"`): 26 pass (9 of them new in `test_india_presidents_c01_15.py`).
- Research tests (`-p "test_*research*.py"`): 79 pass.
- Campaign tests (`-p "test_campaign*.py"`): the 9 `test_campaign_research` tests pass; `test_campaign_census` errors in
  `setUpClass` because `spheres-sim/data/party_leaders.json` is not in this sparse worktree. It was not run, and the
  sparse checkout was not widened. These suites overlap; their counts are not summed.
- `node --test tools/ui/check_leadership_research_review.cjs`: 11 pass.
- `python tools/planning/workboard.py --check`: passes (44 markers).
- `git diff --check`: clean for this packet's paths.
- The new test rejects hand-made regressions both by rule and against the pinned list: a successor's assumption of office
  used as an end for every holder, including with the successor's resolution or oath cited; a term "due to expire", a
  farewell, the eve of demitting office, a styling as former President or a retrospective span used as an end; an end
  invented at the cutoff; a declaration, a Gazette publication, a ceremony programme or an election schedule used as a
  start or an observation, or cited by a holder; the Lok Sabha seat notice, the corrigendum's recital, a recollection, a
  later caption, a member's statement, the PMO's oath report or a name corrigendum cited by a holder; a Vice-President
  acting as President, service on the eve of demitting office, a farewell or a continuation after an expiry notice added
  as a holder; a prime minister added as a president, a prime-ministership claim cited by a president, a president moved
  into the prime-ministership, a second head-of-state role on the prime-ministership or an organization; a second role or
  presidency; a span given a structured date or period; holders out of order; collapsed event dates; and checksum, path,
  reversed-interval and beyond-cutoff mutations.

C01 and all parent gates (C06, S23, WC1, CP1) stay open. No installed leader, avatar, portrait, campaign rule or save
schema changed.
