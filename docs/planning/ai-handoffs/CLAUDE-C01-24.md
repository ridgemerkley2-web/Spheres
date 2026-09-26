# CLAUDE-C01-24: Tongan Speakers of the Legislative Assembly, 1990–2026

Owner: Claude. State: **claimed** (25 September 2026; in progress, not complete). Parent: C01 (incomplete).

Origin: a self-proposed follow-up packet, started on the user's 25 September 2026 instruction to start another
batch of five packets in parallel. It does not repeat accepted C01-01/02/03/04/07/08 or reclaim
the pending C01-05, C01-06 and C01-09 to C01-22. It is pending Codex acceptance and is not registered in `docs/planning/ai-workstreams.json`.

Branch: `claude/c01-to-24`. Base: `ffe54b02` (current `codex/campaign-certification`); not stacked on another pending packet. Claim
commit: this record's first commit on the branch.

## Bounded deliverable

Extend the existing `to_speaker` role (Speaker) of `to_legislative_assembly`, which holds only the string observation `to_speakers_appointment`, with the Speakers from 1 January 1990 to the cutoff. Review at most ten observations:

1. The Speaker when the period opens, and his appointment only if a source dates it.
2. Each royal appointment of a Speaker under the pre-2010 constitution in the 1990s.
3. Appointments in the 2000s.
4. The 2010 constitutional reform's procedure for electing the Speaker (procedure only).
5. The Speaker of the first Assembly after the 2010 election.
6. The Speakers of 2012–2014.
7. The Speaker after the 2014 election.
8. The Speaker after the 2017 election.
9. The Speaker after the 2021 election.
10. The Speaker after the 2025 election, and an official attestation of the holder before the cutoff.

Keep royal appointment, the Assembly's election of its Speaker, the oath, a resignation and an acting Speaker as distinct dated claims; an acting Speaker or the Deputy Speaker presiding is a claim only. The existing `to_speakers_appointment` observation and all other Tonga holders do not change. Never infer an end from a successor's start unless a source states it. Give a holder `from` only where a source states the day office was assumed or took effect, and `until` only where a source states the day the office ended; otherwise record `attested_on`. Constitution and statute texts may establish procedure only, never a date. Retrospective lists are claims, never boundaries. News, encyclopaedias and history sites are leads only. The historical cutoff stays 7 September 2026.

Primary sources are required: the Legislative Assembly of Tonga (parliament.gov.to, including archived pages, minutes and releases), the Palace Office, the Prime Minister's Office, the Government Gazette and court records.

## Allowed files and checks

Allowed files:

- this record;
- a new `docs/campaign-certification/C01/research/tonga-speakers-1990-2026-24.md`;
- `docs/campaign-certification/C01/research/tonga.json` and specifically related new
  `docs/campaign-certification/C01/research/sources/tonga-*-facts.json` extracts;
- focused tests under `tools/avatars/`: a new `test_tonga_speakers_c01_24.py`, and pinned counts or exact sets in `test_tonga_research_s10g.py` and the existing `test_tonga_*_c01_*.py` tests
  updated to the new totals (none loosened).

Put generated `research-index.json` changes in a separate commit. Do not change shared UI, the roadmap, game
data or other country packets.

Checks: research-index `--check`; the Tonga, research and campaign Python tests; the atlas Node check;
`workboard.py --check`; `git diff --check`.

Mark the packet `ready_for_review` when done. C01 and all parent gates stay open.
