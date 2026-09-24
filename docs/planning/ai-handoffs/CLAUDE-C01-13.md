# CLAUDE-C01-13: Japanese prime ministers, 2006–2026

Owner: Claude. State: **claimed** (24 September 2026; in progress, not complete). Parent: C01 (incomplete).

Origin: a self-proposed follow-up packet, started on the user's 24 September 2026 instruction to continue
development faster with several packets in parallel. It does not repeat accepted C01-01/02/03/04/07/08 or reclaim
the pending C01-05 (USSR/RSFSR), C01-06 (Saudi Arabia), C01-09 (South Africa), C01-10 (Brazil), C01-11 (India) and C01-12 (Japan, 1990-2006). It is pending Codex acceptance and is not registered in `docs/planning/ai-workstreams.json`.

Branch: `claude/c01-jp-13`. **Stacked on CLAUDE-C01-12** (`claude/c01-jp-12` at `87e1da53`), which is
ready for review and not yet integrated, because both packets edit `japan.json`: merge CLAUDE-C01-12 first. Claim
commit: this record's first commit on the branch.

## Bounded deliverable

Continue CLAUDE-C01-12 on the same `jp_prime_minister` institution and `jp_pm` role (内閣総理大臣), from the appointment of 安倍晋三 on 26 September 2006 that C01-12 records only as its closing boundary, to the cutoff. Review at most ten observations:

1. 2006–2007: 安倍晋三's designation and appointment, and his cabinet's resignation.
2. 2007–2008: 福田康夫's designation, appointment and resignation.
3. 2008–2009: 麻生太郎's designation, appointment and resignation.
4. 2009–2010: 鳩山由紀夫's designation, appointment and resignation.
5. 2010–2011: 菅直人's designation, appointment and resignation.
6. 2011–2012: 野田佳彦's designation, appointment and resignation.
7. 2012–2020: 安倍晋三's designation and appointment of December 2012, his re-designations of 2014 and 2017, and his 2020 resignation.
8. 2020–2021: 菅義偉's designation, appointment and resignation.
9. 2021–2024: 岸田文雄's designation and appointment of October 2021, his November 2021 re-designation, and his 2024 resignation.
10. 2024–2026: 石破茂's designations of October and November 2024, 高市早苗's designation and appointment of October 2025, and an official attestation of the holder in office before the cutoff.

Keep each House's designation vote, the Imperial appointment ceremony (親任式), the cabinet's formation, resignation en masse (総辞職) and continued performance of duties as distinct dated claims; continued duties and any acting prime minister are claims, never holders. The existing LDP president observations (石破茂, 高市早苗) stay separate and unchanged; party office never feeds `jp_pm`. Names follow the packet's convention (Japanese as printed). Never infer an end from a successor's start unless a source states it. Give a holder `from` only where a source states the day office was assumed or took effect, and `until` only where a source states the day the office ended; otherwise record `attested_on`. Constitution and statute texts may establish procedure only, never a date. Retrospective lists are claims, never boundaries. News, encyclopaedias and history sites are leads only. The historical cutoff stays 7 September 2026.

Primary sources are required: the Prime Minister's Office and Cabinet (kantei.go.jp, including archived pages), the minutes of the National Diet (kokkai.ndl.go.jp) and the two Houses' own records, the Official Gazette where accessible without restriction, and the Imperial Household Agency.

## Allowed files and checks

Allowed files:

- this record;
- a new `docs/campaign-certification/C01/research/japan-prime-ministers-2006-2026-13.md`;
- `docs/campaign-certification/C01/research/japan.json` and specifically related new
  `docs/campaign-certification/C01/research/sources/japan-*-facts.json` extracts;
- focused tests under `tools/avatars/`: a new `test_japan_prime_ministers_c01_13.py`, and pinned counts or exact sets in `test_japan_research_s10d.py` and `test_japan_prime_ministers_c01_12.py`
  updated to the new totals (none loosened).

Put generated `research-index.json` changes in a separate commit. Do not change shared UI, the roadmap, game
data or other country packets.

Checks: research-index `--check`; the Japan, research and campaign Python tests; the atlas Node check;
`workboard.py --check`; `git diff --check`.

Mark the packet `ready_for_review` when done. C01 and all parent gates stay open.
