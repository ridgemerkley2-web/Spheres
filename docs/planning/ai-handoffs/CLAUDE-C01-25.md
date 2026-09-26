# CLAUDE-C01-25: Saudi Shura Council and Allegiance Commission chairs, 1990–2026

Owner: Claude. State: **claimed** (25 September 2026; in progress, not complete). Parent: C01 (incomplete).

Origin: a self-proposed follow-up packet, started on the user's 25 September 2026 instruction to start another
batch of five packets in parallel. It does not repeat accepted C01-01/02/03/04/07/08 or reclaim
the pending C01-05, C01-06 and C01-09 to C01-22. It is pending Codex acceptance and is not registered in `docs/planning/ai-workstreams.json`.

Branch: `claude/c01-sa-25`. **Stacked on CLAUDE-C01-06** (`claude/c01-saudi-06` at `7948ab98`, merged with integration `ffe54b02` at `8fdc2e6e`), which is
ready for review and not yet integrated, because both packets edit `saudi-arabia.json`: merge CLAUDE-C01-06 first. Claim
commit: this record's first commit on the branch.

## Bounded deliverable

Fill two existing empty roles: `sa_shura_chair` (Chairman of the Shura Council) of `sa_shura`, and `sa_succession_chair` (Chairman of the Allegiance Commission) of `sa_succession_commission`. Review at most ten observations between 1 January 1990 and the cutoff:

1. The Shura Council Law of 1992 and the council's formation, as procedure only.
2. The first Chairman's appointment (the council's first term).
3. The Chairman's reappointments across terms.
4. The 2009 appointment of a new Chairman.
5. Later appointments (2013 onward).
6. The Chairman at the council's current term.
7. The Allegiance Commission Law of 2006 and the commission's formation, as procedure only.
8. The Allegiance Commission's first Chairman.
9. Later Allegiance Commission Chairmen.
10. Official attestations of both chairs before the cutoff.

Keep a royal order of appointment, its stated effective day, the start of a council term, a relief from office and a death as distinct dated claims. The C01-06 king, crown-prince and prime-minister holders do not change. Never infer an end from a successor's start unless a source states it. Give a holder `from` only where a source states the day office was assumed or took effect, and `until` only where a source states the day the office ended; otherwise record `attested_on`. Constitution and statute texts may establish procedure only, never a date. Retrospective lists are claims, never boundaries. News, encyclopaedias and history sites are leads only. The historical cutoff stays 7 September 2026.

Primary sources are required: the Saudi Press Agency (spa.gov.sa, including archived pages), the Shura Council (shura.gov.sa), the Bureau of Experts at the Council of Ministers (laws.boe.gov.sa) and Umm al-Qura, the official gazette.

## Allowed files and checks

Allowed files:

- this record;
- a new `docs/campaign-certification/C01/research/saudi-shura-allegiance-chairs-1990-2026-25.md`;
- `docs/campaign-certification/C01/research/saudi-arabia.json` and specifically related new
  `docs/campaign-certification/C01/research/sources/saudi-arabia-*-facts.json` extracts;
- focused tests under `tools/avatars/`: a new `test_saudi_shura_allegiance_c01_25.py`, and pinned counts or exact sets in `test_saudi_executive_c01_06.py` and any other test pinning the Saudi packet
  updated to the new totals (none loosened).

Put generated `research-index.json` changes in a separate commit. Do not change shared UI, the roadmap, game
data or other country packets.

Checks: research-index `--check`; the SaudiArabia, research and campaign Python tests; the atlas Node check;
`workboard.py --check`; `git diff --check`.

Mark the packet `ready_for_review` when done. C01 and all parent gates stay open.
