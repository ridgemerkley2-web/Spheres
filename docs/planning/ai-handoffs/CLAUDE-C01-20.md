# CLAUDE-C01-20: Indian National Congress presidents, 1990–2026

Owner: Claude. State: **claimed** (25 September 2026; in progress, not complete). Parent: C01 (incomplete).

Origin: a self-proposed follow-up packet, started on the user's 25 September 2026 instruction to start another
batch of five packets in parallel. It does not repeat accepted C01-01/02/03/04/07/08 or reclaim
the pending C01-05, C01-06 and C01-09 to C01-17. It is pending Codex acceptance and is not registered in `docs/planning/ai-workstreams.json`.

Branch: `claude/c01-in-20`. **Stacked on CLAUDE-C01-15** (`claude/c01-in-15` at `dd58a610`), which is
ready for review and not yet integrated, because both packets edit `india.json`: merge CLAUDE-C01-15 first. Claim
commit: this record's first commit on the branch.

## Bounded deliverable

Add one party role, `in_inc_president` (President of the Indian National Congress, kind `party_leader`), to the existing `Indian National Congress` recognition observation (`in_eci_20240323_np_05`), and review at most ten observations between 1 January 1990 and the cutoff:

1. The Congress President when the period opens (Rajiv Gandhi), and the vacancy after his death in May 1991 only as source-stated facts.
2. 1991–1992: P. V. Narasimha Rao's selection and any confirmation by the All India Congress Committee.
3. 1996: Sitaram Kesri's selection.
4. 1998: Sonia Gandhi's selection by the Congress Working Committee and its ratification.
5. Sonia Gandhi's organisational elections and extensions (2000 onward).
6. 2017: Rahul Gandhi's election and assumption of the office.
7. 2019: Rahul Gandhi's resignation and Sonia Gandhi's appointment as interim President.
8. 2022: the organisational election's result and Mallikarjun Kharge's assumption of the office.
9. Any stated end of a presidency where a source states it.
10. An attestation by the party of its President before the cutoff.

Keep a Working Committee decision, an All India Congress Committee ratification, the Central Election Authority's declaration of results, the assumption of charge, a resignation and interim arrangements as distinct dated claims. Party office and government office stay separate: no `in_prime_minister` or `in_presidency` claim may feed the party role or the reverse. The recognition observation's identity, lifecycle and game mapping do not change. Never infer an end from a successor's start unless a source states it. Give a holder `from` only where a source states the day office was assumed or took effect, and `until` only where a source states the day the office ended; otherwise record `attested_on`. Constitution and statute texts may establish procedure only, never a date. Retrospective lists are claims, never boundaries. News, encyclopaedias and history sites are leads only. The historical cutoff stays 7 September 2026.

Primary sources are required: the Indian National Congress's own records (inc.in and aicc.org.in, including archived pages, press releases, resolutions and Central Election Authority notices), and official Parliament or Election Commission records only where they record the party office.

## Allowed files and checks

Allowed files:

- this record;
- a new `docs/campaign-certification/C01/research/india-inc-presidents-1990-2026-20.md`;
- `docs/campaign-certification/C01/research/india.json` and specifically related new
  `docs/campaign-certification/C01/research/sources/india-*-facts.json` extracts;
- focused tests under `tools/avatars/`: a new `test_india_inc_presidents_c01_20.py`, and pinned counts or exact sets in `test_india_research_s10e.py`, `test_india_prime_ministers_c01_11.py` and `test_india_presidents_c01_15.py`
  updated to the new totals (none loosened).

Put generated `research-index.json` changes in a separate commit. Do not change shared UI, the roadmap, game
data or other country packets.

Checks: research-index `--check`; the India, research and campaign Python tests; the atlas Node check;
`workboard.py --check`; `git diff --check`.

Mark the packet `ready_for_review` when done. C01 and all parent gates stay open.
