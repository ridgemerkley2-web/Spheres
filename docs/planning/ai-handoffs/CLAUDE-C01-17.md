# CLAUDE-C01-17: Brazilian vice-presidents, 1990–2026

Owner: Claude. State: **ready_for_review** (24 September 2026; submitted, not accepted). Parent: C01 (incomplete).
Report: [brazil-vice-presidents-1990-2026-17.md](../../campaign-certification/C01/research/brazil-vice-presidents-1990-2026-17.md).

Origin: a self-proposed follow-up packet, started on the user's 24 September 2026 instruction to continue
development faster with several packets in parallel. It does not repeat accepted C01-01/02/03/04/07/08 or reclaim
the pending C01-05 (USSR/RSFSR), C01-06 (Saudi Arabia), C01-09 (South Africa), C01-10 (Brazil), C01-11 (India) and C01-12 (Japan, 1990-2006). It is pending Codex acceptance and is not registered in `docs/planning/ai-workstreams.json`.

Branch: `claude/c01-br-17`. **Stacked on CLAUDE-C01-10** (`claude/c01-br-10` at `73e5fd36`), which is
ready for review and not yet integrated, because both packets edit `brazil.json`: merge CLAUDE-C01-10 first. Claim
commit: `774af12d` (this record only).

## Bounded deliverable

Add a second role, `br_vice_president` (Vice-President of the Federative Republic of Brazil, kind `institutional_office`), to C01-10's `br_presidency` institution, and review at most ten observations between 1 January 1990 and the cutoff:

1. The office on 1 January 1990: any holder, or a source-stated vacancy.
2. The posse of 15 March 1990 (Itamar Franco).
3. 1992: the Vice-President's exercise of the Presidency and the vacancy that followed his posse as President, recorded as source-stated facts only.
4. The posses of 1 January 1995 and 1999 (Marco Maciel).
5. The posses of 1 January 2003 and 2007 (José Alencar).
6. The posses of 1 January 2011 and 2015 (Michel Temer).
7. 2016: the Vice-President's interim exercise and his posse as President, and any stated vacancy that followed.
8. The posse of 1 January 2019 (Hamilton Mourão).
9. The posse of 1 January 2023 (Geraldo Alckmin).
10. An official attestation of the holder in office before the cutoff.

Keep election, diplomação, posse, exercise of the Presidency (as substitute or interim), vacancy and any stated end as distinct dated claims. A Vice-President exercising the Presidency never becomes a `br_president` holder, and C01-10's president holders do not change. A vacancy is recorded only where a source states it, never inferred. Never infer an end from a successor's start unless a source states it. Give a holder `from` only where a source states the day office was assumed or took effect, and `until` only where a source states the day the office ended; otherwise record `attested_on`. Constitution and statute texts may establish procedure only, never a date. Retrospective lists are claims, never boundaries. News, encyclopaedias and history sites are leads only. The historical cutoff stays 7 September 2026.

Primary sources are required: the Presidency and Vice-Presidency (Planalto, gov.br, including archived pages), the National Congress, Senate and Chamber of Deputies (session records, the Diário do Congresso Nacional as stored complete issues, never regenerated page-range PDFs), the Diário Oficial da União, and the Superior Electoral Court.

## Allowed files and checks

Allowed files:

- this record;
- a new `docs/campaign-certification/C01/research/brazil-vice-presidents-1990-2026-17.md`;
- `docs/campaign-certification/C01/research/brazil.json` and specifically related new
  `docs/campaign-certification/C01/research/sources/brazil-*-facts.json` extracts, and the existing extracts of the
  CLAUDE-C01-10 sources whose recorded responses gain Vice-President claims (rows appended);
- focused tests under `tools/avatars/`: a new `test_brazil_vice_presidents_c01_17.py`, and pinned counts or exact sets in `test_brazil_research_s10f.py` and `test_brazil_presidents_c01_10.py`
  updated to the new totals (none loosened).

Put generated `research-index.json` changes in a separate commit. Do not change shared UI, the roadmap, game
data or other country packets.

Checks: research-index `--check`; the Brazil, research and campaign Python tests; the atlas Node check;
`workboard.py --check`; `git diff --check`.

Mark the packet `ready_for_review` when done. C01 and all parent gates stay open.

## Result

Submitted `ready_for_review` on 24 September 2026 on `claude/c01-br-17`, stacked on `claude/c01-br-10` at `73e5fd36`
(fetched that day: no commits beyond the stack point, so no merge was needed). Touched paths (nothing else):

- `docs/planning/ai-handoffs/CLAUDE-C01-17.md` (this record);
- `docs/campaign-certification/C01/research/brazil-vice-presidents-1990-2026-17.md` (new report);
- `docs/campaign-certification/C01/research/brazil.json` (additions only: 11 sources, 85 claims, the second role and
  its nine holders, coverage notes; snapshot values of the 13 sources that gain claims);
- 11 new extracts `docs/campaign-certification/C01/research/sources/brazil-*-facts.json`, and 13 existing CLAUDE-C01-10
  extracts with Vice-President rows appended (`brazil-congress-dcn-collor-posse-19900316`, `-resignation-posse-19921230`,
  `-cardoso-posse-19950102`, `-cardoso-posse-19990102`, `-lula-posse-20030102`, `-lula-posse-20070102`,
  `-rousseff-posse-20110101`, `-rousseff-posse-20150102`, `-temer-posse-20160901`, `-bolsonaro-posse-20190102`,
  `-bolsonaro-diploma-termo-20190102`, `-lula-posse-20230102` and `brazil-presidency-library-collor-20160812`);
- `tools/avatars/test_brazil_vice_presidents_c01_17.py` (new), `tools/avatars/test_brazil_presidents_c01_10.py` and
  `tools/avatars/test_brazil_research_s10f.py` (pins re-expressed exactly, none loosened);
- `docs/campaign-certification/C01/research-index.json` (regenerated, separate commit; the only file shared with the
  other pending packets).

The presidency institution gains the role `br_vice_president` (Vice-President of the Federative Republic of Brazil,
`institutional_office`) with nine holder observations, each starting on a stated posse and none with an end: Itamar
Franco 15 March 1990; Marco Maciel 1 January 1995 and 1999; José Alencar 2003 and 2007; Michel Temer 2011 and 2015;
Hamilton Mourão 2019; Geraldo Alckmin 2023. No reviewed primary source states the end of a vice-presidential term or a
vacancy of the Vice-Presidency, so no end and no vacancy is recorded; the vacancies declared on 29 December 1992 and
31 August 2016 are the Presidency's. The Vice-President's exercise of the Presidency (1985, 1991, 1992, 2016, 2022,
2026), his succession to it, elections, diplomações, oaths and retrospective spans are claims only. CLAUDE-C01-10's
`br_president` role, twelve holders and 112 claims are unchanged; 13 of its claims are cited on the new role.

Observation decisions:

| ID | Decision |
|---|---|
| BR-VP-01 | Unresolved: no holder and no source-stated vacancy on 1 January 1990 |
| BR-VP-02 | Accepted: election 17 Dec 1989, diplomação 30 Dec 1989, posse 15 Mar 1990 (start) |
| BR-VP-03 | Accepted in part: exercise 11 Dec 1991 and from 2 Oct 1992, succession 29 Dec 1992 (claims only); no end or vacancy stated |
| BR-VP-04 | Accepted: Maciel's posses of 1995 and 1999 (starts) |
| BR-VP-05 | Accepted: Alencar's posses of 2003 and 2007 (starts) |
| BR-VP-06 | Accepted: Temer's posses of 2011 and 2015 (starts) |
| BR-VP-07 | Accepted in part: notification 12 May 2016, exercise, succession 31 Aug 2016 (claims only); no end or vacancy stated |
| BR-VP-08 | Accepted: Mourão's posse of 1 Jan 2019 (start) |
| BR-VP-09 | Accepted: Alckmin's posse of 1 Jan 2023 (start) |
| BR-VP-10 | Accepted: Laws No. 15.434 and 15.436 of 16-17 Jun 2026 signed by Alckmin as Vice-President in exercise (role claims) |

All 27 checker defects (A1-A8, B1-B6, C1-C13) are applied, two of them (C4, C12) by removing the duplicate Rádio Senado
claim and the live image identity. Four missing primary records are imported (Law No. 8.469, the decree of 11 December
1991, the Senate case file volume 48, Law No. 15.436); Law No. 8.470, Decree No. 363 and the six optional TSE diploma
originals are leads with their identities, and the gov.br Vice-Presidency pages stayed behind a bot check (not
bypassed).

Checks run on 24 September 2026 in this sparse worktree (`docs/campaign-certification/C01`, `docs/planning`,
`tools/avatars`, `tools/planning`, `tools/ui`; not widened), with `PYTHONDONTWRITEBYTECODE=1`:

- `python -X utf8 tools/avatars/campaign_research.py` then `--check`: exact regeneration passes; 9 packets,
  841 organization and 28 institution observations, 220 sources, 1,959 claims, 92 open batches.
- Brazil tests (`-p "test_brazil*.py"`): 25 pass (8 of them new in `test_brazil_vice_presidents_c01_17.py`).
- Research tests (`-p "test_*research*.py"`): 79 pass.
- Campaign tests (`-p "test_campaign*.py"`): the 9 `test_campaign_research` tests pass; `test_campaign_census`
  errors in `setUpClass` because `spheres-sim/data/party_leaders.json` is not in this sparse worktree. It was not
  run, and the sparse checkout was not widened. These suites overlap; their counts are not summed.
- `node --test tools/ui/check_leadership_research_review.cjs`: 11 pass.
- `python tools/planning/workboard.py --check`: passes (44 markers).
- `git diff --check`: clean for this packet's paths.
- The new test rejects hand-made regressions both by rule and against the pinned lists: a successor's posse, the
  Vice-President's posse as President, the Presidency's vacancy, a succession record, a retrospective span, a
  re-election, a last act before a posse, a declared period or a successor's tribute used or cited as an end; an
  election or diplomação date or the interim exercise date used as a start; an election, oath or styled-incumbent
  claim cited by a holder; interim or acting service added as a Vice-President holder or as a `br_president` holder,
  or cited by either; a Vice-President holder moved onto `br_president` or citing the President's posse or a
  CLAUDE-C01-10 exercise claim; the Vice-Presidency copied onto an organization or a second institution; a third role
  or swapped roles; the Presidency's vacancy recorded on the Vice-Presidency; a retrospective span given a date; an
  exercise stored as a period; a printed name used as the holder name; holders out of order; collapsed event dates;
  and checksum, path, reversed-interval, unknown-claim and beyond-cutoff mutations (including a structured 2027
  period end).

C01 and all parent gates (C06, S23, WC1, CP1) stay open. No installed leader, avatar, portrait, campaign rule or
save schema changed.
