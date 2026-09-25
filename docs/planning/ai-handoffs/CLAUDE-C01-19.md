# CLAUDE-C01-19: Russian heads of government, 1991–2026

Owner: Claude. State: **claimed** (25 September 2026; in progress, not complete). Parent: C01 (incomplete).

Origin: a self-proposed follow-up packet, started on the user's 25 September 2026 instruction to start another
batch of five packets in parallel. It does not repeat accepted C01-01/02/03/04/07/08 or reclaim
the pending C01-05, C01-06 and C01-09 to C01-17. It is pending Codex acceptance and is not registered in `docs/planning/ai-workstreams.json`.

Branch: `claude/c01-ru-19`. **Stacked on CLAUDE-C01-14** (`claude/c01-ru-14` at `1d749e15`), which is
ready for review and not yet integrated, because both packets edit `russia.json`: merge CLAUDE-C01-14 first. Claim
commit: this record's first commit on the branch.

## Bounded deliverable

Add one executive institution, `ru_government` (Правительство Российской Федерации — Government of the Russian Federation, kind `executive_institution`), with one role, `ru_government_chairman` (Председатель Правительства — Chairman of the Government, kind `head_of_government`), and review at most ten observations between November 1991 and the cutoff:

1. 1991–1992: who headed the RSFSR/Russian government from November 1991, and any acting chairman in 1992, as claims.
2. December 1992: Viktor Chernomyrdin's appointment after the Congress of People's Deputies' consent, and his 1996 reappointment.
3. 1998: Chernomyrdin's dismissal and Sergei Kiriyenko's appointment after the State Duma's consent.
4. 1998: Kiriyenko's dismissal and Yevgeny Primakov's appointment.
5. 1999: Primakov's dismissal, Sergei Stepashin's appointment and dismissal, and Vladimir Putin's appointment.
6. 2000–2004: Mikhail Kasyanov's appointment and dismissal.
7. 2004–2007: Mikhail Fradkov's appointment, his 2004 reappointment and his 2007 resignation.
8. 2007–2008: Viktor Zubkov's appointment and Putin's 2008 appointment.
9. 2012–2020: Dmitry Medvedev's appointments of 2012 and 2018 and the government's 2020 resignation.
10. 2020–2026: Mikhail Mishustin's appointments of 2020 and 2024, and an official attestation of the holder in office before the cutoff.

Keep the President's nomination, the parliament's consent, the appointment decree, the government's resignation or dismissal, continued duties until a new government is formed, and an acting chairman as distinct dated claims. Acting and continuing service are claims only, never holders. The presidency holders from CLAUDE-C01-05 and C01-14 do not change. Names follow the packet's convention (Russian as printed). Portal searches must be date-restricted single-document cards, never unrestricted number searches whose results grow. Never infer an end from a successor's start unless a source states it. Give a holder `from` only where a source states the day office was assumed or took effect, and `until` only where a source states the day the office ended; otherwise record `attested_on`. Constitution and statute texts may establish procedure only, never a date. Retrospective lists are claims, never boundaries. News, encyclopaedias and history sites are leads only. The historical cutoff stays 7 September 2026.

Primary sources are required: the President of Russia's decrees and the Government's own records (kremlin.ru, government.ru, including archived pages), the official legal-information portal (pravo.gov.ru) and official gazettes, and the State Duma's resolutions and transcripts.

## Allowed files and checks

Allowed files:

- this record;
- a new `docs/campaign-certification/C01/research/russia-heads-of-government-1991-2026-19.md`;
- `docs/campaign-certification/C01/research/russia.json` and specifically related new
  `docs/campaign-certification/C01/research/sources/russia-*-facts.json` extracts;
- focused tests under `tools/avatars/`: a new `test_russia_heads_of_government_c01_19.py`, and pinned counts or exact sets in `test_russia_research_s10h.py`, `test_ussr_russia_transition_c01_05.py` and `test_russia_presidents_c01_14.py`
  updated to the new totals (none loosened).

Put generated `research-index.json` changes in a separate commit. Do not change shared UI, the roadmap, game
data or other country packets.

Checks: research-index `--check`; the Russia, research and campaign Python tests; the atlas Node check;
`workboard.py --check`; `git diff --check`.

Mark the packet `ready_for_review` when done. C01 and all parent gates stay open.
