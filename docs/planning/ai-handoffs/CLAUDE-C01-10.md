# CLAUDE-C01-10: Brazilian presidents, 1990–2026

Owner: Claude. State: **ready_for_review** (23 September 2026; submitted, not accepted). Parent: C01
(incomplete). Report: [brazil-presidents-1990-2026-10.md](../../campaign-certification/C01/research/brazil-presidents-1990-2026-10.md).

Origin: a self-proposed follow-up packet, started on the user's 23 September 2026 instruction to continue
development. The workboard asks for a distinct bounded packet that does not repeat accepted
C01-01/02/03/04/07/08 or reclaim the pending C01-05 (USSR/RSFSR), C01-06 (Saudi Arabia) and C01-09 (South
Africa). This packet touches only the Brazil research packet, which none of those change. It is pending Codex
acceptance and is not registered in `docs/planning/ai-workstreams.json`.

Branch: `claude/c01-br-10`. Base: `ffe54b02` (current `codex/campaign-certification`). Claim commit: `0d177d7d`
(this record only).

## Bounded deliverable

The Brazil packet has party observations only. Add one executive institution, `br_presidency`, with one role,
`br_president` (President of the Federative Republic of Brazil, kind `head_of_state`), and review at most ten
observations between 1 January 1990 and the 7 September 2026 cutoff:

1. The holder when the period opens (José Sarney) and the end of that term only if a source states it.
2. The inauguration (posse) of 15 March 1990 before the National Congress.
3. 1992: the Chamber of Deputies' authorization, the Senate's opening of the trial and the suspension of the
   President, the Vice-President's exercise of the office, the resignation, the Vice-President's posse as
   President, and the Senate's judgment — each as its own dated claim.
4. The posses of 1 January 1995 and 1999.
5. The posses of 1 January 2003 and 2007.
6. The posses of 1 January 2011 and 2015.
7. 2016: the Chamber's authorization, the Senate's opening of the trial and suspension, the Vice-President's
   interim exercise, the Senate's judgment removing the President, and the Vice-President's posse as President.
8. The posse of 1 January 2019.
9. The posse of 1 January 2023.
10. An official attestation of the holder in office before the cutoff (2026), without extending any term.

Keep election, diplomação (certification by the electoral court), posse (oath before Congress), transfer of
the sash, suspension, interim or acting exercise, resignation, removal by judgment and any loss of political
rights as distinct dated claims. Never infer an end from a successor's posse unless a source states it. Give a
holder `from` only where a source states the day office was assumed, and `until` only where a source states the
day a term, resignation or removal took effect; otherwise record `attested_on`. A Vice-President exercising the
office is recorded as claims, never as a holder of `br_president`. Vice-presidents, ministers and party offices
are otherwise outside this packet.

Primary sources are required: the Presidency (Planalto, including the Presidency Library's records of former
presidents), the National Congress, Senate and Chamber of Deputies (session records, the Diário do Congresso
Nacional, resolutions and their official news agencies' reports of their own proceedings), the Diário Oficial
da União, the Superior Electoral Court (diplomação) and the Supreme Federal Court. Constitution texts may
establish procedure only, never a date. News, encyclopaedias and history sites are leads only. The historical
cutoff stays 7 September 2026.

## Allowed files and checks

Allowed files:

- this record;
- a new `docs/campaign-certification/C01/research/brazil-presidents-1990-2026-10.md`;
- `docs/campaign-certification/C01/research/brazil.json` and specifically related new
  `docs/campaign-certification/C01/research/sources/brazil-*-facts.json` extracts;
- focused tests under `tools/avatars/`: a new `test_brazil_presidents_c01_10.py`, and pinned counts or exact sets
  in `test_brazil_research_s10f.py` updated to the new totals (none loosened).

Put generated `research-index.json` changes in a separate commit. Do not change shared UI, the roadmap, game
data or other country packets.

Checks:

- research-index `--check`;
- the Brazil, research and campaign Python tests;
- the atlas Node check;
- `workboard.py --check`;
- `git diff --check`.

Mark the packet `ready_for_review` when done. C01 and all parent gates stay open.

## Result

Submitted `ready_for_review` on 23 September 2026. Touched paths (nothing else):

- `docs/planning/ai-handoffs/CLAUDE-C01-10.md` (this record);
- `docs/campaign-certification/C01/research/brazil-presidents-1990-2026-10.md` (new report);
- `docs/campaign-certification/C01/research/brazil.json`;
- 48 new extracts `docs/campaign-certification/C01/research/sources/brazil-*-facts.json`;
- `tools/avatars/test_brazil_presidents_c01_10.py` (new) and `tools/avatars/test_brazil_research_s10f.py`;
- `docs/campaign-certification/C01/research-index.json` (regenerated, separate commit).

The packet gains 48 sources and 112 claims, one institution (`br_presidency`, `executive_institution`, lifecycle
`unknown`) with one role (`br_president`, President of the Federative Republic of Brazil, `head_of_state`) and twelve
holder observations: José Sarney observed on 5 January 1990, and eleven holders with a start stated by a posse record
(Collor 15 March 1990; Itamar Franco 29 December 1992; Cardoso 1995 and 1999; Lula 2003 and 2007; Rousseff 2011 and
2015; Temer 31 August 2016; Bolsonaro 2019; Lula 2023). Only two ends are stated: Collor's resignation on 29 December
1992 and Rousseff's removal on 31 August 2016 (Resolution No. 35, in force on publication that day). Periods declared
at a posse, successors' statements that the predecessor "deixa o governo", a successor's posse and retrospective
Presidency Library spans are never ends; elections, diplomações, oaths, sash ceremonies, suspensions and the
Vice-President's exercise of the office are claims only.

Observation decisions:

| ID | Decision |
|---|---|
| BR-PRES-01 | Accepted in part: Sarney observed 5 Jan 1990 (and 13 Mar 1990); his end not stated |
| BR-PRES-02 | Accepted: election 17 Dec 1989, diplomação 30 Dec 1989, posse 15 Mar 1990 (start) |
| BR-PRES-03 | Accepted: authorization 29 Sep, trial opened 1 Oct, suspension 2 Oct, resignation and Itamar Franco's posse 29 Dec, disqualification 30 Dec 1992 |
| BR-PRES-04 | Accepted in part: posses of 1995 and 1999 (starts); Itamar Franco's end stated only by a successor, not set |
| BR-PRES-05 | Accepted: posses of 2003 and 2007 (starts) |
| BR-PRES-06 | Accepted: posses of 2011 and 2015 (starts) |
| BR-PRES-07 | Accepted: authorization 17 Apr, admission and suspension 12 May, removal 31 Aug 2016 (Rousseff's end); Temer's posse 31 Aug 2016 (start) |
| BR-PRES-08 | Accepted in part: posse of 1 Jan 2019 (start); Temer's end not stated |
| BR-PRES-09 | Accepted in part: posse of 1 Jan 2023 (start); Bolsonaro's end not stated |
| BR-PRES-10 | Accepted: MP 1.388 of 24 Aug 2026 signed by Lula as President |

All 27 checker defects (A1-A8, B1-B9, C1-C10) are applied, two of them (C1, C3) by merging the declared-period
claims into the posse declarations; the report's Checker defects table gives each outcome. Five missing primary
records found by the checks are imported; the Senate and Chamber agency reports of the 2003 and 2011 sash transfers
are leads, and three challenge-blocked items were not bypassed.

Checks run on 23 September 2026 in this sparse worktree (`docs/campaign-certification/C01`, `docs/planning`,
`tools/avatars`, `tools/planning`, `tools/ui`; not widened), with `PYTHONDONTWRITEBYTECODE=1`:

- `python -X utf8 tools/avatars/campaign_research.py` then `--check`: exact regeneration passes; 9 packets,
  841 organization and 28 institution observations, 209 sources, 1,874 claims, 92 open batches.
- Brazil tests (`-p "test_brazil*.py"`): 17 pass (8 of them new in `test_brazil_presidents_c01_10.py`).
- Research tests (`-p "test_*research*.py"`): 79 pass.
- Campaign tests (`-p "test_campaign*.py"`): the 9 `test_campaign_research` tests pass; `test_campaign_census`
  errors in `setUpClass` because `spheres-sim/data/party_leaders.json` is not in this sparse worktree. It was not
  run, and the sparse checkout was not widened. These suites overlap; their counts are not summed.
- `node --test tools/ui/check_leadership_research_review.cjs`: 11 pass.
- `python tools/planning/workboard.py --check`: passes (44 markers).
- `git diff --check`: clean for this packet's paths.
- The new test rejects hand-made regressions both by rule and against the pinned list: a successor's posse used as an
  end (Sarney, Itamar Franco, Temer, Bolsonaro); a successor's statement cited as an end (Itamar Franco, Lula 2007); a
  re-election used as an end; a declared period used as an end (Temer, Bolsonaro); a suspension or judgment used as an
  end; an election or diplomação date used as a start (Collor, Bolsonaro, Lula 2023); the Vice-President's exercise
  date used as a start; the Vice-President's interim exercise added as a holder (Itamar Franco, Temer, Mourão) or
  cited by one; an election, oath or sash claim cited by a holder; a library span given a date; a declared period
  stored as a structure; an observation date without an in-office act; a second role, a second institution or a
  head-of-state role on an organization; holders out of order; collapsed event dates; and checksum, path, reversed
  interval and beyond-cutoff mutations (including a structured 2027 period end).

C01 and all parent gates (C06, S23, WC1, CP1) stay open. No installed leader, avatar, portrait, campaign rule or
save schema changed.
