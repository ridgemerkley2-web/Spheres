# CLAUDE-C01-14: Russian presidents, 1991–2026

Owner: Claude. State: **claimed** (24 September 2026; in progress, not complete). Parent: C01 (incomplete).

Origin: a self-proposed follow-up packet, started on the user's 24 September 2026 instruction to continue
development faster with several packets in parallel. It does not repeat accepted C01-01/02/03/04/07/08 or reclaim
the pending C01-05 (USSR/RSFSR), C01-06 (Saudi Arabia), C01-09 (South Africa), C01-10 (Brazil), C01-11 (India) and C01-12 (Japan, 1990-2006). It is pending Codex acceptance and is not registered in `docs/planning/ai-workstreams.json`.

Branch: `claude/c01-ru-14`. **Stacked on CLAUDE-C01-05** (`claude/c01-ussr-05` at `1c698ed0`, merged with integration `ffe54b02` at `0c9b5f19`), which is
ready for review and not yet integrated, because both packets edit `russia.json`: merge CLAUDE-C01-05 first. Claim
commit: this record's first commit on the branch.

## Bounded deliverable

CLAUDE-C01-05 added `ru_rsfsr_presidency` with the role `ru_rsfsr_president` (Yeltsin from 10 July 1991, no end). Add a role `ru_president` (Президент Российской Федерации — President of the Russian Federation, kind `head_of_state`) to that institution and review at most ten observations to the cutoff:

1. The renaming of the RSFSR as the Russian Federation and of the office's title (1991–1992), recorded as claims; state whether any source ties the renamed office to the existing holder, and never merge holders across the two roles without one.
2. The 1993 Constitution's entry into force, as procedure only, and any source-stated effect on the incumbent's office.
3. 1996: the election result and Yeltsin's inauguration and oath.
4. 1999: Yeltsin's resignation (address and decree) and Putin's service as acting President (claims only).
5. 2000: the election result and Putin's inauguration.
6. 2004: the election result and inauguration.
7. 2008: Medvedev's election result and inauguration.
8. 2012: Putin's election result and inauguration.
9. 2018: the election result and inauguration.
10. 2024: the election result and inauguration, and an official attestation of the holder in office before the cutoff.

Keep the Central Election Commission's result, the inauguration and oath, any stated assumption of office, a resignation, acting service and a term's stated end as distinct dated claims. Acting service is claims only, never a holder. Do not change the C01-05 holders or the USSR packet. Names follow the packet's convention. Never infer an end from a successor's start unless a source states it. Give a holder `from` only where a source states the day office was assumed or took effect, and `until` only where a source states the day the office ended; otherwise record `attested_on`. Constitution and statute texts may establish procedure only, never a date. Retrospective lists are claims, never boundaries. News, encyclopaedias and history sites are leads only. The historical cutoff stays 7 September 2026.

Primary sources are required: the President of Russia's official site (kremlin.ru, including archived pages), the official legal-information portal (pravo.gov.ru) and official gazettes, the Central Election Commission (cikrf.ru), the Constitutional Court, the State Duma and the Federation Council.

## Allowed files and checks

Allowed files:

- this record;
- a new `docs/campaign-certification/C01/research/russia-presidents-1991-2026-14.md`;
- `docs/campaign-certification/C01/research/russia.json` and specifically related new
  `docs/campaign-certification/C01/research/sources/russia-*-facts.json` extracts;
- focused tests under `tools/avatars/`: a new `test_russia_presidents_c01_14.py`, and pinned counts or exact sets in `test_russia_research_s10h.py` and `test_ussr_russia_transition_c01_05.py`
  updated to the new totals (none loosened).

Put generated `research-index.json` changes in a separate commit. Do not change shared UI, the roadmap, game
data or other country packets.

Checks: research-index `--check`; the Russia, research and campaign Python tests; the atlas Node check;
`workboard.py --check`; `git diff --check`.

Mark the packet `ready_for_review` when done. C01 and all parent gates stay open.
