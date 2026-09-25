# CLAUDE-C01-21: South African deputy presidents, 1994–2026

Owner: Claude. State: **claimed** (25 September 2026; in progress, not complete). Parent: C01 (incomplete).

Origin: a self-proposed follow-up packet, started on the user's 25 September 2026 instruction to start another
batch of five packets in parallel. It does not repeat accepted C01-01/02/03/04/07/08 or reclaim
the pending C01-05, C01-06 and C01-09 to C01-17. It is pending Codex acceptance and is not registered in `docs/planning/ai-workstreams.json`.

Branch: `claude/c01-za-21`. **Stacked on CLAUDE-C01-16** (`claude/c01-za-16` at `2df4a0a6`), which is
ready for review and not yet integrated, because both packets edit `south-africa.json`: merge CLAUDE-C01-16 first. Claim
commit: this record's first commit on the branch.

## Bounded deliverable

Add one role, `za_deputy_president` (Deputy President of the Republic of South Africa, kind `institutional_office`), to the existing `za_presidency` institution, and review at most ten observations between 10 May 1994 and the cutoff:

1. 1994: the appointments of Thabo Mbeki and F. W. de Klerk as Executive Deputy Presidents.
2. 1996: de Klerk's withdrawal from the Government of National Unity and the end of his office.
3. 1999: Jacob Zuma's appointment.
4. 2005: Zuma's release from office and Phumzile Mlambo-Ngcuka's appointment.
5. 2008: Mlambo-Ngcuka's resignation and Baleka Mbete's appointment.
6. 2009: Kgalema Motlanthe's appointment.
7. 2014: Cyril Ramaphosa's appointment.
8. 2018 and 2019: David Mabuza's appointments.
9. 2023: Mabuza's resignation and Paul Mashatile's appointment.
10. 2024: Mashatile's reappointment, and an official attestation of the holder in office before the cutoff.

Keep the President's announcement, the appointment, the oath before the Chief Justice, a resignation or release from office and its stated effective day as distinct dated claims. The Deputy President acting as President is a claim only and never a holder of `za_president_election`; the C01-09 presidency holders and the C01-16 ANC holders do not change. Never infer an end from a successor's start unless a source states it. Give a holder `from` only where a source states the day office was assumed or took effect, and `until` only where a source states the day the office ended; otherwise record `attested_on`. Constitution and statute texts may establish procedure only, never a date. Retrospective lists are claims, never boundaries. News, encyclopaedias and history sites are leads only. The historical cutoff stays 7 September 2026.

Primary sources are required: the Presidency (thepresidency.gov.za), the South African Government's statement archives (gov.za, including archived official sites), Parliament's records, and the Government Gazette.

## Allowed files and checks

Allowed files:

- this record;
- a new `docs/campaign-certification/C01/research/south-africa-deputy-presidents-1994-2026-21.md`;
- `docs/campaign-certification/C01/research/south-africa.json` and specifically related new
  `docs/campaign-certification/C01/research/sources/south-africa-*-facts.json` extracts;
- focused tests under `tools/avatars/`: a new `test_south_africa_deputy_presidents_c01_21.py`, and pinned counts or exact sets in `test_south_africa_research_s10h.py`, `test_south_africa_heads_of_state_c01_09.py` and `test_south_africa_anc_presidents_c01_16.py`
  updated to the new totals (none loosened).

Put generated `research-index.json` changes in a separate commit. Do not change shared UI, the roadmap, game
data or other country packets.

Checks: research-index `--check`; the SouthAfrica, research and campaign Python tests; the atlas Node check;
`workboard.py --check`; `git diff --check`.

Mark the packet `ready_for_review` when done. C01 and all parent gates stay open.
