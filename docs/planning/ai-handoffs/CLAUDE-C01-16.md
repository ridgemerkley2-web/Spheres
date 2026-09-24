# CLAUDE-C01-16: ANC presidents, 1990–2026

Owner: Claude. State: **claimed** (24 September 2026; in progress, not complete). Parent: C01 (incomplete).

Origin: a self-proposed follow-up packet, started on the user's 24 September 2026 instruction to continue
development faster with several packets in parallel. It does not repeat accepted C01-01/02/03/04/07/08 or reclaim
the pending C01-05 (USSR/RSFSR), C01-06 (Saudi Arabia), C01-09 (South Africa), C01-10 (Brazil), C01-11 (India) and C01-12 (Japan, 1990-2006). It is pending Codex acceptance and is not registered in `docs/planning/ai-workstreams.json`.

Branch: `claude/c01-za-16`. **Stacked on CLAUDE-C01-09** (`claude/c01-za-09` at `82a23f9d`), which is
ready for review and not yet integrated, because both packets edit `south-africa.json`: merge CLAUDE-C01-09 first. Claim
commit: this record's first commit on the branch.

## Bounded deliverable

Add one party role, `za_anc_president` (President of the African National Congress, kind `party_leader`), to the existing `AFRICAN NATIONAL CONGRESS` organization observation, and review at most ten observations between 1 January 1990 and the cutoff:

1. The ANC President when the period opens, and any 1990 acting or deputy arrangement recorded as claims.
2. July 1991: the 48th National Conference's election of the President.
3. December 1994: the 49th National Conference.
4. December 1997: the 50th National Conference.
5. December 2002: the 51st National Conference.
6. December 2007: the 52nd National Conference.
7. December 2012: the 53rd National Conference.
8. December 2017: the 54th National Conference.
9. December 2022: the 55th National Conference.
10. An attestation by the ANC of its President before the cutoff.

Keep a conference's election or declaration of results, the handover or assumption of the party office and any acting arrangement as distinct dated claims. Party office and the Presidency of the Republic stay separate: no `za_presidency` claim may feed the ANC role or the reverse, and the ANC's organizational lifecycle, identity and game mapping stay unresolved. The C01-09 presidency holders do not change. Never infer an end from a successor's start unless a source states it. Give a holder `from` only where a source states the day office was assumed or took effect, and `until` only where a source states the day the office ended; otherwise record `attested_on`. Constitution and statute texts may establish procedure only, never a date. Retrospective lists are claims, never boundaries. News, encyclopaedias and history sites are leads only. The historical cutoff stays 7 September 2026.

Primary sources are required: the ANC's own records (its official websites, including archived pages, conference reports and resolutions, statements and ANC Today), and official Parliament or government records only where they record the party office.

## Allowed files and checks

Allowed files:

- this record;
- a new `docs/campaign-certification/C01/research/south-africa-anc-presidents-1990-2026-16.md`;
- `docs/campaign-certification/C01/research/south-africa.json` and specifically related new
  `docs/campaign-certification/C01/research/sources/south-africa-*-facts.json` extracts;
- focused tests under `tools/avatars/`: a new `test_south_africa_anc_presidents_c01_16.py`, and pinned counts or exact sets in `test_south_africa_research_s10h.py` and `test_south_africa_heads_of_state_c01_09.py`
  updated to the new totals (none loosened).

Put generated `research-index.json` changes in a separate commit. Do not change shared UI, the roadmap, game
data or other country packets.

Checks: research-index `--check`; the SouthAfrica, research and campaign Python tests; the atlas Node check;
`workboard.py --check`; `git diff --check`.

Mark the packet `ready_for_review` when done. C01 and all parent gates stay open.
