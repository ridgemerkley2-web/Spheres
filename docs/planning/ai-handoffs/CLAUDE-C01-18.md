# CLAUDE-C01-18: Liberal Democratic Party presidents, 1990–2009

Owner: Claude. State: **claimed** (25 September 2026; in progress, not complete). Parent: C01 (incomplete).

Origin: a self-proposed follow-up packet, started on the user's 25 September 2026 instruction to start another
batch of five packets in parallel. It does not repeat accepted C01-01/02/03/04/07/08 or reclaim
the pending C01-05, C01-06 and C01-09 to C01-17. It is pending Codex acceptance and is not registered in `docs/planning/ai-workstreams.json`.

Branch: `claude/c01-jp-18`. **Stacked on CLAUDE-C01-13** (`claude/c01-jp-13` at `42f98dc8`), which is
ready for review and not yet integrated, because both packets edit `japan.json`: merge CLAUDE-C01-13 first. Claim
commit: this record's first commit on the branch.

## Bounded deliverable

Extend the existing party role `jp_ldp_party_president` (総裁) on the `自由民主党` organization observation, which today holds only 石破茂 (2024) and 高市早苗 (2025), with the presidents from 1 January 1990 to the September 2009 presidential election. Review at most ten observations:

1. The President when the period opens (海部俊樹) and the end of his presidency only if a source states it.
2. 1991: the presidential election won by 宮澤喜一.
3. 1993: the presidential election won by 河野洋平, a President who was never Prime Minister.
4. 1995: the presidential election won by 橋本龍太郎.
5. 1998: the presidential election won by 小渕恵三.
6. 2000: 森喜朗's selection as President after 小渕恵三's incapacity.
7. 2001: the presidential election won by 小泉純一郎, and his re-elections.
8. 2006 and 2007: the presidential elections won by 安倍晋三 and 福田康夫.
9. 2008: the presidential election won by 麻生太郎.
10. 2009: the presidential election won by 谷垣禎一, a President who was never Prime Minister.

Keep a party election or selection (votes by members and Diet members, or a joint plenary meeting of party Diet members), the declaration of results, the start of the term and a resignation as distinct dated claims. Party office and the prime-ministership stay separate: no `jp_pm` claim may feed the LDP role or the reverse, and the existing 2024 and 2025 LDP holders do not change. Names follow the packet's convention (Japanese as printed). Never infer an end from a successor's start unless a source states it. Give a holder `from` only where a source states the day office was assumed or took effect, and `until` only where a source states the day the office ended; otherwise record `attested_on`. Constitution and statute texts may establish procedure only, never a date. Retrospective lists are claims, never boundaries. News, encyclopaedias and history sites are leads only. The historical cutoff stays 7 September 2026.

Primary sources are required: the Liberal Democratic Party's own records (jimin.jp and its archived pages, party histories and announcements), and Diet minutes (kokkai.ndl.go.jp) only where a party officer's statement records the party office.

## Allowed files and checks

Allowed files:

- this record;
- a new `docs/campaign-certification/C01/research/japan-ldp-presidents-1990-2009-18.md`;
- `docs/campaign-certification/C01/research/japan.json` and specifically related new
  `docs/campaign-certification/C01/research/sources/japan-*-facts.json` extracts;
- focused tests under `tools/avatars/`: a new `test_japan_ldp_presidents_c01_18.py`, and pinned counts or exact sets in `test_japan_research_s10d.py`, `test_japan_prime_ministers_c01_12.py` and `test_japan_prime_ministers_c01_13.py`
  updated to the new totals (none loosened).

Put generated `research-index.json` changes in a separate commit. Do not change shared UI, the roadmap, game
data or other country packets.

Checks: research-index `--check`; the Japan, research and campaign Python tests; the atlas Node check;
`workboard.py --check`; `git diff --check`.

Mark the packet `ready_for_review` when done. C01 and all parent gates stay open.
