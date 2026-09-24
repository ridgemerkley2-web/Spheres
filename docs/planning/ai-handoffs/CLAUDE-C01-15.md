# CLAUDE-C01-15: Indian presidents, 1990–2026

Owner: Claude. State: **claimed** (24 September 2026; in progress, not complete). Parent: C01 (incomplete).

Origin: a self-proposed follow-up packet, started on the user's 24 September 2026 instruction to continue
development faster with several packets in parallel. It does not repeat accepted C01-01/02/03/04/07/08 or reclaim
the pending C01-05 (USSR/RSFSR), C01-06 (Saudi Arabia), C01-09 (South Africa), C01-10 (Brazil), C01-11 (India) and C01-12 (Japan, 1990-2006). It is pending Codex acceptance and is not registered in `docs/planning/ai-workstreams.json`.

Branch: `claude/c01-in-15`. **Stacked on CLAUDE-C01-11** (`claude/c01-in-11` at `538920f1`), which is
ready for review and not yet integrated, because both packets edit `india.json`: merge CLAUDE-C01-11 first. Claim
commit: this record's first commit on the branch.

## Bounded deliverable

Add a second executive institution, `in_presidency`, with one role, `in_president` (President of India, kind `head_of_state`), separate from C01-11's `in_prime_minister`, and review at most ten observations between 1 January 1990 and the cutoff:

1. The holder when the period opens (R. Venkataraman) and the end of his term only if a source states it.
2. 1992: the election result and Shankar Dayal Sharma's oath and assumption of office.
3. 1997: the election result and K. R. Narayanan's oath and assumption of office.
4. 2002: the election result and A. P. J. Abdul Kalam's oath and assumption of office.
5. 2007: the election result and Pratibha Patil's oath and assumption of office.
6. 2012: the election result and Pranab Mukherjee's oath and assumption of office.
7. 2017: the election result and Ram Nath Kovind's oath and assumption of office.
8. 2022: the election result and Droupadi Murmu's oath and assumption of office.
9. The stated ends of terms, where a source states them.
10. An official attestation of the holder in office before the cutoff.

Keep the Returning Officer's or Election Commission's declaration of the result, the oath administered by the Chief Justice, a stated assumption of office and a stated end as distinct dated claims. Vice-presidents acting as President are claims only. Do not change the C01-11 prime-minister holders. Never infer an end from a successor's start unless a source states it. Give a holder `from` only where a source states the day office was assumed or took effect, and `until` only where a source states the day the office ended; otherwise record `attested_on`. Constitution and statute texts may establish procedure only, never a date. Retrospective lists are claims, never boundaries. News, encyclopaedias and history sites are leads only. The historical cutoff stays 7 September 2026.

Primary sources are required: the President's Secretariat (Rashtrapati Bhavan), the Election Commission of India, the Press Information Bureau (including its archive), notifications in the Gazette of India, and the Lok Sabha and Rajya Sabha records.

## Allowed files and checks

Allowed files:

- this record;
- a new `docs/campaign-certification/C01/research/india-presidents-1990-2026-15.md`;
- `docs/campaign-certification/C01/research/india.json` and specifically related new
  `docs/campaign-certification/C01/research/sources/india-*-facts.json` extracts;
- focused tests under `tools/avatars/`: a new `test_india_presidents_c01_15.py`, and pinned counts or exact sets in `test_india_research_s10e.py` and `test_india_prime_ministers_c01_11.py`
  updated to the new totals (none loosened).

Put generated `research-index.json` changes in a separate commit. Do not change shared UI, the roadmap, game
data or other country packets.

Checks: research-index `--check`; the India, research and campaign Python tests; the atlas Node check;
`workboard.py --check`; `git diff --check`.

Mark the packet `ready_for_review` when done. C01 and all parent gates stay open.
