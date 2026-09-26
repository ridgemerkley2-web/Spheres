# CLAUDE-C01-27: Bharatiya Janata Party presidents, 1990–2026

Owner: Claude. State: **claimed** (25 September 2026; in progress, not complete). Parent: C01 (incomplete).

Origin: a self-proposed follow-up packet, started on the user's 25 September 2026 instruction to start another
batch of five packets in parallel. It does not repeat accepted C01-01/02/03/04/07/08 or reclaim
the pending C01-05, C01-06 and C01-09 to C01-22. It is pending Codex acceptance and is not registered in `docs/planning/ai-workstreams.json`.

Branch: `claude/c01-in-27`. **Stacked on CLAUDE-C01-20** (`claude/c01-in-20` at `2ee3b146`), which is
ready for review and not yet integrated, because both packets edit `india.json`: merge CLAUDE-C01-20 first. Claim
commit: this record's first commit on the branch.

## Bounded deliverable

Add one party role, `in_bjp_president` (National President of the Bharatiya Janata Party, kind `party_leader`), to the existing `Bharatiya Janata Party` recognition observation, and review at most ten observations between 1 January 1990 and the cutoff:

1. The President when the period opens (L. K. Advani) and the 1991 change.
2. Murli Manohar Joshi's presidency.
3. Advani's return (1993).
4. Kushabhau Thakre (1998) and Bangaru Laxman (2000).
5. K. Jana Krishnamurthi (2001) and M. Venkaiah Naidu (2002).
6. Advani (2004) and Rajnath Singh (2005).
7. Nitin Gadkari (2009) and Rajnath Singh (2013).
8. Amit Shah (2014, re-elected 2016).
9. J. P. Nadda (working President 2019, President 2020, extensions).
10. The President at the cutoff, and a party attestation before it.

Keep the National Council's or organisational election's result, the assumption of charge, working or interim presidencies, resignations and extensions as distinct dated claims; a working president is a claim only unless a source states a substantive presidency. Party office and the prime-ministership and presidency stay separate both ways, and the INC role from C01-20 does not change. Never infer an end from a successor's start unless a source states it. Give a holder `from` only where a source states the day office was assumed or took effect, and `until` only where a source states the day the office ended; otherwise record `attested_on`. Constitution and statute texts may establish procedure only, never a date. Retrospective lists are claims, never boundaries. News, encyclopaedias and history sites are leads only. The historical cutoff stays 7 September 2026.

Primary sources are required: the BJP's own records (bjp.org, including archived pages, press releases and National Council records), and official Parliament or Election Commission records only where they record the party office.

## Allowed files and checks

Allowed files:

- this record;
- a new `docs/campaign-certification/C01/research/india-bjp-presidents-1990-2026-27.md`;
- `docs/campaign-certification/C01/research/india.json` and specifically related new
  `docs/campaign-certification/C01/research/sources/india-*-facts.json` extracts;
- focused tests under `tools/avatars/`: a new `test_india_bjp_presidents_c01_27.py`, and pinned counts or exact sets in `test_india_research_s10e.py`, `test_india_prime_ministers_c01_11.py`, `test_india_presidents_c01_15.py` and `test_india_inc_presidents_c01_20.py`
  updated to the new totals (none loosened).

Put generated `research-index.json` changes in a separate commit. Do not change shared UI, the roadmap, game
data or other country packets.

Checks: research-index `--check`; the India, research and campaign Python tests; the atlas Node check;
`workboard.py --check`; `git diff --check`.

Mark the packet `ready_for_review` when done. C01 and all parent gates stay open.
