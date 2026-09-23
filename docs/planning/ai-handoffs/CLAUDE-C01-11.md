# CLAUDE-C01-11: Indian prime ministers, 1990–2026

Owner: Claude. State: **claimed** (23 September 2026; in progress, not complete). Parent: C01 (incomplete).

Origin: a self-proposed follow-up packet, started on the user's 23 September 2026 instruction to continue
development. The workboard asks for a distinct bounded packet that does not repeat accepted
C01-01/02/03/04/07/08 or reclaim the pending C01-05 (USSR/RSFSR), C01-06 (Saudi Arabia), C01-09 (South Africa)
and C01-10 (Brazil). This packet touches only the India research packet, which none of those change. It is
pending Codex acceptance and is not registered in `docs/planning/ai-workstreams.json`.

Branch: `claude/c01-in-11`. Base: `ffe54b02` (current `codex/campaign-certification`). Claim commit: this
record's first commit on the branch.

## Bounded deliverable

The India packet has party-recognition observations only. Add one executive institution,
`in_prime_minister`, with one role, `in_pm` (Prime Minister of India, kind `head_of_government`), and review at
most ten observations between 1 January 1990 and the 7 September 2026 cutoff:

1. The holder when the period opens (V. P. Singh), the 1990 confidence vote and his resignation.
2. The swearing-in of November 1990, the 1991 resignation and any request to continue in office.
3. The swearing-in of June 1991 and the 1996 resignation.
4. 1996: the May swearing-in, the resignation that followed, and the June swearing-in.
5. 1997–1998: the April 1997 confidence vote and resignation, the April 1997 swearing-in, the November 1997
   resignation and any continuation in office until March 1998.
6. 1998–1999: the March 1998 swearing-in, the April 1999 confidence vote, any continuation in office, and the
   October 1999 swearing-in.
7. 2004: the resignation and the May swearing-in.
8. 2009: the May swearing-in.
9. 2014 and 2019: the resignation and the two swearings-in.
10. 2024: the June swearing-in, and an official attestation of the holder in office before the cutoff, without
    extending any term.

Keep the President's appointment, the oath or swearing-in, any stated effective date, a confidence vote, a
resignation, the President's acceptance of it, a request to continue in office until other arrangements are
made, and the end of such continuation as distinct dated claims. Never infer an end from a successor's
swearing-in unless a source states it. Give a holder `from` only where a source states the day the appointment
took effect or office was assumed, and `until` only where a source states the day the office ended (for example
a resignation accepted with effect from a stated day); otherwise record `attested_on`. Service after a
resignation at the President's request is recorded as claims, never as a separate holder. Deputy prime
ministers, ministers, the President and party offices are outside this packet.

Primary sources are required: the President's Secretariat (Rashtrapati Bhavan communiqués), the Prime
Minister's Office, the Press Information Bureau (including its archive), Cabinet Secretariat notifications in
the Gazette of India, and the Lok Sabha and Rajya Sabha debates and records. Constitution texts may establish
procedure only, never a date. Retrospective lists of former prime ministers are claims, never boundaries. News,
encyclopaedias and history sites are leads only. The historical cutoff stays 7 September 2026.

## Allowed files and checks

Allowed files:

- this record;
- a new `docs/campaign-certification/C01/research/india-prime-ministers-1990-2026-11.md`;
- `docs/campaign-certification/C01/research/india.json` and specifically related new
  `docs/campaign-certification/C01/research/sources/india-*-facts.json` extracts;
- focused tests under `tools/avatars/`: a new `test_india_prime_ministers_c01_11.py`, and pinned counts or exact
  sets in `test_india_research_s10e.py` updated to the new totals (none loosened).

Put generated `research-index.json` changes in a separate commit. Do not change shared UI, the roadmap, game
data or other country packets.

Checks:

- research-index `--check`;
- the India, research and campaign Python tests;
- the atlas Node check;
- `workboard.py --check`;
- `git diff --check`.

Mark the packet `ready_for_review` when done. C01 and all parent gates stay open.
