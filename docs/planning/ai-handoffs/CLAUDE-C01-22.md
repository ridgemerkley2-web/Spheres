# CLAUDE-C01-22: Workers' Party (PT) national presidents, 1990–2026

Owner: Claude. State: **claimed** (25 September 2026; in progress, not complete). Parent: C01 (incomplete).

Origin: a self-proposed follow-up packet, started on the user's 25 September 2026 instruction to start another
batch of five packets in parallel. It does not repeat accepted C01-01/02/03/04/07/08 or reclaim
the pending C01-05, C01-06 and C01-09 to C01-17. It is pending Codex acceptance and is not registered in `docs/planning/ai-workstreams.json`.

Branch: `claude/c01-br-22`. **Stacked on CLAUDE-C01-17** (`claude/c01-br-17` at `7d71acef`), which is
ready for review and not yet integrated, because both packets edit `brazil.json`: merge CLAUDE-C01-17 first. Claim
commit: this record's first commit on the branch.

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
