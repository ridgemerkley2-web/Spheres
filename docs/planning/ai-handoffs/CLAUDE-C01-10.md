# CLAUDE-C01-10: Brazilian presidents, 1990–2026

Owner: Claude. State: **claimed** (23 September 2026; in progress, not complete). Parent: C01 (incomplete).

Origin: a self-proposed follow-up packet, started on the user's 23 September 2026 instruction to continue
development. The workboard asks for a distinct bounded packet that does not repeat accepted
C01-01/02/03/04/07/08 or reclaim the pending C01-05 (USSR/RSFSR), C01-06 (Saudi Arabia) and C01-09 (South
Africa). This packet touches only the Brazil research packet, which none of those change. It is pending Codex
acceptance and is not registered in `docs/planning/ai-workstreams.json`.

Branch: `claude/c01-br-10`. Base: `ffe54b02` (current `codex/campaign-certification`). Claim commit: this
record's first commit on the branch.

## Bounded deliverable

The Brazil packet has party observations only. Add one executive institution, `br_presidency`, with one role,
`br_president` (President of the Federative Republic of Brazil, kind `head_of_state`), and review at most ten
observations between 1 January 1990 and the 7 September 2026 cutoff:

1. The holder when the period opens (José Sarney) and the end of that term only if a source states it.
2. The inauguration (posse) of 15 March 1990 before the National Congress.
3. 1992: the Chamber of Deputies' authorization, the Senate's opening of the trial and the suspension of the
   President, the Vice-President's exercise of the office, the resignation, the Vice-President's posse as
   President, and the Senate's judgment — each as its own dated claim.
4. The posses of 1 January 1995 and 1999.
5. The posses of 1 January 2003 and 2007.
6. The posses of 1 January 2011 and 2015.
7. 2016: the Chamber's authorization, the Senate's opening of the trial and suspension, the Vice-President's
   interim exercise, the Senate's judgment removing the President, and the Vice-President's posse as President.
8. The posse of 1 January 2019.
9. The posse of 1 January 2023.
10. An official attestation of the holder in office before the cutoff (2026), without extending any term.

Keep election, diplomação (certification by the electoral court), posse (oath before Congress), transfer of
the sash, suspension, interim or acting exercise, resignation, removal by judgment and any loss of political
rights as distinct dated claims. Never infer an end from a successor's posse unless a source states it. Give a
holder `from` only where a source states the day office was assumed, and `until` only where a source states the
day a term, resignation or removal took effect; otherwise record `attested_on`. A Vice-President exercising the
office is recorded as claims, never as a holder of `br_president`. Vice-presidents, ministers and party offices
are otherwise outside this packet.

Primary sources are required: the Presidency (Planalto, including the Presidency Library's records of former
presidents), the National Congress, Senate and Chamber of Deputies (session records, the Diário do Congresso
Nacional, resolutions and their official news agencies' reports of their own proceedings), the Diário Oficial
da União, the Superior Electoral Court (diplomação) and the Supreme Federal Court. Constitution texts may
establish procedure only, never a date. News, encyclopaedias and history sites are leads only. The historical
cutoff stays 7 September 2026.

## Allowed files and checks

Allowed files:

- this record;
- a new `docs/campaign-certification/C01/research/brazil-presidents-1990-2026-10.md`;
- `docs/campaign-certification/C01/research/brazil.json` and specifically related new
  `docs/campaign-certification/C01/research/sources/brazil-*-facts.json` extracts;
- focused tests under `tools/avatars/`: a new `test_brazil_presidents_c01_10.py`, and pinned counts or exact sets
  in `test_brazil_research_s10f.py` updated to the new totals (none loosened).

Put generated `research-index.json` changes in a separate commit. Do not change shared UI, the roadmap, game
data or other country packets.

Checks:

- research-index `--check`;
- the Brazil, research and campaign Python tests;
- the atlas Node check;
- `workboard.py --check`;
- `git diff --check`.

Mark the packet `ready_for_review` when done. C01 and all parent gates stay open.
