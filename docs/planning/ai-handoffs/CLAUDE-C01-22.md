# CLAUDE-C01-22: Workers' Party (PT) national presidents, 1990–2026

Owner: Claude. State: **ready_for_review** (25 September 2026; submitted, not accepted). Parent: C01 (incomplete).
Report: [brazil-pt-presidents-1990-2026-22.md](../../campaign-certification/C01/research/brazil-pt-presidents-1990-2026-22.md).

Origin: a self-proposed follow-up packet, started on the user's 25 September 2026 instruction to start another
batch of five packets in parallel. It does not repeat accepted C01-01/02/03/04/07/08 or reclaim
the pending C01-05, C01-06 and C01-09 to C01-17. It is pending Codex acceptance and is not registered in `docs/planning/ai-workstreams.json`.

Branch: `claude/c01-br-22`. **Stacked on CLAUDE-C01-17** (`claude/c01-br-17` at `7d71acef`), which is
ready for review and not yet integrated, because both packets edit `brazil.json`: merge CLAUDE-C01-17 first. Claim
commit: `45821428` (this record only).

## Bounded deliverable

Add one party role, `br_pt_president` (Presidente Nacional do Partido dos Trabalhadores, kind `party_leader`), to the existing `PT` funding observation (`br_tse_fefc_2024_party_03`), and review at most ten observations between 1 January 1990 and the cutoff:

1. The national President when the period opens (Luiz Inácio Lula da Silva), and the end of his presidency only if a source states it.
2. 1994–1995: the national presidents who followed, and José Dirceu's election.
3. José Dirceu's re-elections and his departure in 2002.
4. 2002–2005: José Genoino's presidency and resignation.
5. 2005: Tarso Genro's interim presidency and Ricardo Berzoini's election.
6. 2006–2010: interim arrangements and Berzoini's re-election, and José Eduardo Dutra's election.
7. 2011–2017: Rui Falcão's presidency.
8. 2017–2025: Gleisi Hoffmann's presidency and its end.
9. 2025: the interim President and Edinho Silva's election and assumption of the office.
10. An attestation by the party or the electoral court of the national President before the cutoff.

Keep a national meeting's or direct election's result (Processo de Eleições Diretas), the assumption of the office, a resignation or leave, and interim arrangements as distinct dated claims. Party office and the Presidency of the Republic stay separate: no `br_presidency` claim may feed the party role or the reverse. The PT observation's identity, lifecycle and game mapping, and the C01-10 and C01-17 holders, do not change. Never infer an end from a successor's start unless a source states it. Give a holder `from` only where a source states the day office was assumed or took effect, and `until` only where a source states the day the office ended; otherwise record `attested_on`. Constitution and statute texts may establish procedure only, never a date. Retrospective lists are claims, never boundaries. News, encyclopaedias and history sites are leads only. The historical cutoff stays 7 September 2026.

Primary sources are required: the Partido dos Trabalhadores's own records (pt.org.br and its archived pages, resolutions and national directorate notices), the Superior Electoral Court's records of the party's national directorate, and Congress records only where they record the party office.

## Allowed files and checks

Allowed files:

- this record;
- a new `docs/campaign-certification/C01/research/brazil-pt-presidents-1990-2026-22.md`;
- `docs/campaign-certification/C01/research/brazil.json` and specifically related new
  `docs/campaign-certification/C01/research/sources/brazil-*-facts.json` extracts;
- focused tests under `tools/avatars/`: a new `test_brazil_pt_presidents_c01_22.py`, and pinned counts or exact sets in `test_brazil_research_s10f.py`, `test_brazil_presidents_c01_10.py` and `test_brazil_vice_presidents_c01_17.py`
  updated to the new totals (none loosened).

Put generated `research-index.json` changes in a separate commit. Do not change shared UI, the roadmap, game
data or other country packets.

Checks: research-index `--check`; the Brazil, research and campaign Python tests; the atlas Node check;
`workboard.py --check`; `git diff --check`.

Mark the packet `ready_for_review` when done. C01 and all parent gates stay open.

## Result

Submitted `ready_for_review` on 25 September 2026 on `claude/c01-br-22`, stacked on `claude/c01-br-17` at `7d71acef`
(fetched that day: no commits beyond the stack point, so no merge was needed). Touched paths (nothing else):

- `docs/planning/ai-handoffs/CLAUDE-C01-22.md` (this record);
- `docs/campaign-certification/C01/research/brazil-pt-presidents-1990-2026-22.md` (new report);
- `docs/campaign-certification/C01/research/brazil.json` (additions only: 108 sources and 177 claims appended after
  CLAUDE-C01-17's; on the PT observation `br_tse_fefc_2024_party_03`, the one new role and the new ids appended to its
  sources and claims; one coverage note on the PT observation and one on the packet);
- 108 new extracts `docs/campaign-certification/C01/research/sources/brazil-*-facts.json` (no existing extract edited);
- `tools/avatars/test_brazil_pt_presidents_c01_22.py` (new); `tools/avatars/test_brazil_research_s10f.py`,
  `tools/avatars/test_brazil_presidents_c01_10.py` and `tools/avatars/test_brazil_vice_presidents_c01_17.py` (pins
  re-expressed exactly, none loosened);
- `docs/campaign-certification/C01/research-index.json` (regenerated, separate commit; the only file shared with the
  other pending packets).

The PT observation gains the party role `br_pt_president` (Presidente Nacional do Partido dos Trabalhadores,
`party_leader`) with eighteen holder observations: Luiz Inácio Lula da Silva from 3 June 1990 (his stated assumption);
Rui Falcão observed 25 November 1994; José Dirceu observed 24 August 1995, 17 September 1997, 2 December 1999 and
4 October 2001; José Genoino observed 18 March 2003; Tarso Genro observed 10 July 2005; Ricardo Berzoini from
22 October 2005 and observed 13 November 2009; José Eduardo Dutra from 19 February 2010 until 29 April 2011 (the party's
stated resignation day); Rui Falcão observed 8 May 2011 and 18 December 2013; Gleisi Hoffmann from 5 July 2017 and
observed 1 April 2020; Humberto Costa from 20 March 2025 (his stated change from interim to effective President); Edinho
Silva from 3 August 2025 and observed 15 August 2026. Luiz Gushiken (President when the period opens) has no holder: no
record gives a single day. Acting and interim service (Genoino 2001 and 2002-2003, Marco Aurélio Garcia 2006-2007, Rui
Falcão 2011, Humberto Costa 7-19 March 2025), elections, selections, PED ballots and results, leaves, handovers,
resignations without a day, Berzoini's stated last day (10 February 2010), registry periods and retrospective records
are claims only. The organization's identity, lifecycle, game mapping and funding record, the `br_presidency`
institution and the CLAUDE-C01-10 and CLAUDE-C01-17 holders are unchanged; no presidency claim or source feeds the party
role or the reverse.

Observation decisions:

| ID | Decision |
|---|---|
| PT-PRES-01 | Accepted in part: Gushiken, not Lula, on 1 January 1990 (no holder: no single day); Lula from 3 June 1990; no stated end in 1994 |
| PT-PRES-02 | Accepted in part: Rui Falcão observed 25 November 1994 (interim status open); December 1994-August 1995 unresolved; Dirceu observed 24 August 1995 |
| PT-PRES-03 | Accepted in part: Dirceu observed 17 September 1997, 2 December 1999, 4 October 2001; leaves, acting service and a two-day resignation meeting are claims |
| PT-PRES-04 | Accepted in part: Genoino elected 15 March 2003 after interim service, observed 18 March 2003; the 2005 handover-or-leave is a claim |
| PT-PRES-05 | Accepted in part: Tarso Genro observed 10 July 2005; PED 2005 ballots and declaration; Berzoini from 22 October 2005 |
| PT-PRES-06 | Accepted in part: Garcia interim (claims); Berzoini observed 13 November 2009; Dutra from 19 February 2010; Berzoini's stated last day a claim |
| PT-PRES-07 | Accepted in part: Dutra until 29 April 2011; Falcão acting (claim), observed 8 May 2011 and 18 December 2013; the 2015 hint unsupported |
| PT-PRES-08 | Accepted in part: Gleisi Hoffmann from 5 July 2017, observed 1 April 2020; departure announced 7 March 2025, no stated end |
| PT-PRES-09 | Accepted in part: Humberto Costa interim 7-19 March, from 20 March 2025; Edinho Silva from 3 August 2025 (the prospective 4 August recorded for Codex) |
| PT-PRES-10 | Accepted: the PT's Diretório page archived 15 August 2026; the live SGIP record of the organ in force is a lead |

Checker defects: all 40 (A1-A12, B1-B14, C1-C14) are applied, C2 in part (the start kept on the stated posse, with
the alternative recorded for Codex) and B10 declined (the Chamber diary of 19 December 2002: other deputies' remarks on
an OCR scan during the interim period; a lead with its identity). Of the 25 missing primary records the checks found,
24 are imported; one further portal item (18 March 2003) was located for this packet. Rulings left to Codex are listed
in the report's integration notes.

Checks run on 25 September 2026 in this sparse worktree (`docs/campaign-certification/C01`, `docs/planning`,
`tools/avatars`, `tools/planning`, `tools/ui`; not widened), with `PYTHONDONTWRITEBYTECODE=1`:

- `python -X utf8 tools/avatars/campaign_research.py` then `--check`: exact regeneration passes; 9 packets,
  841 organization and 28 institution observations, 328 sources, 2,136 claims, 92 open batches.
- Brazil tests (`-p "test_brazil*.py"`): 33 pass (8 of them in the new `test_brazil_pt_presidents_c01_22.py`).
- Research tests (`-p "test_*research*.py"`): 79 pass.
- Campaign tests (`-p "test_campaign*.py"`): the 9 `test_campaign_research` tests pass; `test_campaign_census`
  errors in `setUpClass` because `spheres-sim/data/party_leaders.json` is not in this sparse worktree. It was not
  run, and the sparse checkout was not widened. These suites overlap; their counts are not summed.
- `node --test tools/ui/check_leadership_research_review.cjs`: 11 pass.
- `python tools/planning/workboard.py --check`: passes (44 markers).
- `git diff --check`: clean for this packet's paths.
- The new test rejects hand-made regressions both by rule and against the pinned lists: a successor's styling, start,
  selection or posse used as an end; Berzoini's stated last day, a two-day resignation meeting, a leave or a registry
  period used as an end; an election, result, nomination, Diretório election, prospective assumption, registry start or
  in-office observation used as a start; an election, continuation or interim-period claim cited by a holder; acting or
  interim service (Genoino 2001 and 2003, Humberto Costa, Marco Aurélio Garcia, Rui Falcão 2011) added as a holder; a
  Presidency of the Republic holder added to the party role or a party holder added to `br_president`; a presidency
  claim moved onto the party role or the reverse; the party role copied onto another organization or onto the
  presidency institution; a second party role or no role; a lifecycle start; a retrospective span given a date; a
  printed civil name as the holder name; holders out of order; collapsed event dates; and checksum, path,
  reversed-interval, unknown-claim, foreign-mapping and beyond-cutoff mutations.

C01 and all parent gates (C06, S23, WC1, CP1) stay open. No installed leader, avatar, portrait, campaign rule or
save schema changed.
