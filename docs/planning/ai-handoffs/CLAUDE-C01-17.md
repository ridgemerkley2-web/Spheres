# CLAUDE-C01-17: Brazilian vice-presidents, 1990–2026

Owner: Claude. State: **claimed** (24 September 2026; in progress, not complete). Parent: C01 (incomplete).

Origin: a self-proposed follow-up packet, started on the user's 24 September 2026 instruction to continue
development faster with several packets in parallel. It does not repeat accepted C01-01/02/03/04/07/08 or reclaim
the pending C01-05 (USSR/RSFSR), C01-06 (Saudi Arabia), C01-09 (South Africa), C01-10 (Brazil), C01-11 (India) and C01-12 (Japan, 1990-2006). It is pending Codex acceptance and is not registered in `docs/planning/ai-workstreams.json`.

Branch: `claude/c01-br-17`. **Stacked on CLAUDE-C01-10** (`claude/c01-br-10` at `73e5fd36`), which is
ready for review and not yet integrated, because both packets edit `brazil.json`: merge CLAUDE-C01-10 first. Claim
commit: this record's first commit on the branch.

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
  `docs/campaign-certification/C01/research/sources/brazil-*-facts.json` extracts;
- focused tests under `tools/avatars/`: a new `test_brazil_vice_presidents_c01_17.py`, and pinned counts or exact sets in `test_brazil_research_s10f.py` and `test_brazil_presidents_c01_10.py`
  updated to the new totals (none loosened).

Put generated `research-index.json` changes in a separate commit. Do not change shared UI, the roadmap, game
data or other country packets.

Checks: research-index `--check`; the Brazil, research and campaign Python tests; the atlas Node check;
`workboard.py --check`; `git diff --check`.

Mark the packet `ready_for_review` when done. C01 and all parent gates stay open.
