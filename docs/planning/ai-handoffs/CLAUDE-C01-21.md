# CLAUDE-C01-21: South African deputy presidents, 1994–2026

Owner: Claude. State: **ready_for_review** (25 September 2026; submitted, not accepted). Parent: C01 (incomplete).
Report: [south-africa-deputy-presidents-1994-2026-21.md](../../campaign-certification/C01/research/south-africa-deputy-presidents-1994-2026-21.md).

Origin: a self-proposed follow-up packet, started on the user's 25 September 2026 instruction to start another
batch of five packets in parallel. It does not repeat accepted C01-01/02/03/04/07/08 or reclaim
the pending C01-05, C01-06 and C01-09 to C01-17. It is pending Codex acceptance and is not registered in `docs/planning/ai-workstreams.json`.

Branch: `claude/c01-za-21`. **Stacked on CLAUDE-C01-16** (`claude/c01-za-16` at `2df4a0a6`), which is
ready for review and not yet integrated, because both packets edit `south-africa.json`: merge CLAUDE-C01-16 first. Claim
commit: `e2d1a1ec` (this record only). CLAUDE-C01-16 is itself stacked on CLAUDE-C01-09 (`claude/c01-za-09`), which is
based on `ffe54b02` (`codex/campaign-certification`).

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

## Result

Submitted `ready_for_review` on 25 September 2026. Before the work, `claude/c01-za-16` was fetched from the remote; it
had no commit missing from this branch, so nothing was merged. Touched paths (nothing else):

- `docs/planning/ai-handoffs/CLAUDE-C01-21.md` (this record);
- `docs/campaign-certification/C01/research/south-africa-deputy-presidents-1994-2026-21.md` (new report);
- `docs/campaign-certification/C01/research/south-africa.json`;
- 59 new extracts `docs/campaign-certification/C01/research/sources/south-africa-*-facts.json` (no existing extract is
  edited: the four responses CLAUDE-C01-09 already cites have separate source records and extracts);
- `tools/avatars/test_south_africa_deputy_presidents_c01_21.py` (new), and pinned values in
  `tools/avatars/test_south_africa_research_s10h.py`, `tools/avatars/test_south_africa_heads_of_state_c01_09.py` and
  `tools/avatars/test_south_africa_anc_presidents_c01_16.py`;
- `docs/campaign-certification/C01/research-index.json` (regenerated, separate commit; the only file shared with other
  pending packets).

The packet gains 59 sources and 82 claims and the role `za_deputy_president` (Deputy President of the Republic of South
Africa, `institutional_office`) in `za_presidency`, after the unchanged President and State President roles. The role has
twelve holder observations. Only Jacob Zuma's has a start (17 June 1999, the effective day General Notice 1392 of 1999
states) and none has an end: F. W. de Klerk (1994-05-10), Thabo Mbeki (1994-05-25), Jacob Zuma (from 1999-06-17),
Phumzile Mlambo-Ngcuka (2005-06-22), Baleka Mbete (2008-10-21), Kgalema Motlanthe (2009-05-11), Cyril Ramaphosa
(2014-05-30), David Mabuza (2018-02-27, 2019-05-30) and Paul Mashatile (2023-03-07, 2024-07-04, 2026-08-30).
Announcements, intentions, appointment notices, scheduled oaths, releases, resignations and their stated effect,
Assembly-seat events, retrospective lists and continuation attestations are claims only. No acting service is a holder;
the CLAUDE-C01-09 presidency holders and the CLAUDE-C01-16 ANC holders are unchanged, and no claim or source is shared
between this role and theirs.

Observation decisions:

| ID | Decision |
|---|---|
| ZA-DP-01 | Accepted in part: de Klerk observed 10 May 1994, Mbeki 25 May 1994 (UN record with the Foreign Affairs copy); no designation or oath record, no start |
| ZA-DP-02 | Accepted in part: withdrawal communicated 9 May 1996; no stated end for de Klerk (30 June 1996 only in a retrospective list) |
| ZA-DP-03 | Accepted: Zuma from 17 June 1999 (General Notice 1392 of 1999, published 2 July 1999) |
| ZA-DP-04 | Accepted in part: release announced and accepted 14 June 2005 with no effective day; Mlambo-Ngcuka observed 22 June 2005, no start (oath scheduled 23 June, not recorded) |
| ZA-DP-05 | Accepted in part: resignation effective only by reference, no end; Mbete designated 25 September 2008, observed 21 October 2008 |
| ZA-DP-06 | Accepted: Motlanthe sworn in 11 May 2009 (observation, not a start) |
| ZA-DP-07 | Accepted in part: announced 25 May 2014; Ramaphosa observed 30 May 2014; no oath record |
| ZA-DP-08 | Accepted in part: Mabuza sworn in 27 February 2018 and 30 May 2019 (Presidency profile) |
| ZA-DP-09 | Accepted in part: Assembly seat resigned as of 28 February 2023, term ended without a stated day; Mashatile sworn in 7 March 2023 |
| ZA-DP-10 | Accepted in part: reappointment announced 30 June 2024, oath not recorded; Mashatile observed 4 July 2024 and 30 August 2026 |

All 30 checker defects (A1-A9, B1-B11, C1-C10) are applied; B11 needed no change. Thirteen missing primary records found
by the checks are imported; five are declined as leads (members' remarks, papers naming no one, and a duplicate copy),
for the reasons in the report. The part B researcher's two requested rulings are decided: no `until` of 25 September
2008 and no `from` of 22 June 2005 for Mlambo-Ngcuka.

Checks run on 25 September 2026 in this sparse worktree (`docs/campaign-certification/C01`, `docs/planning`,
`tools/avatars`, `tools/planning`, `tools/ui`; not widened), with `PYTHONDONTWRITEBYTECODE=1`:

- `python -X utf8 tools/avatars/campaign_research.py` then `--check`: exact regeneration passes; 9 packets, 841 organization and 27 institution observations, 324 sources, 1,995 claims, 92 open batches.
- South Africa tests (`-p "test_south_africa*.py"`): 34 pass (8 of them new in
  `test_south_africa_deputy_presidents_c01_21.py`).
- Research tests (`-p "test_*research*.py"`): 79 pass.
- Campaign tests (`-p "test_campaign*.py"`): the 9 `test_campaign_research` tests pass;
  `test_campaign_census` errors in `setUpClass` because `spheres-sim/data/party_leaders.json` is not in this sparse
  worktree. It was not run, and the sparse checkout was not widened. These suites overlap; their counts are not summed.
- `node --test tools/ui/check_leadership_research_review.cjs`: 11 pass.
- `python tools/planning/workboard.py --check`: passes (44 canonical markers).
- `git diff --check`: clean for this packet's paths.
- Every recorded response was downloaded again for this packet (at 15:28-15:31Z on 25 September 2026), more than 30 minutes after the
  earlier downloads, and matched its recorded byte count and SHA-256. The independent source verification then
  replaced the 1 March 2023 statement's gzip-encoded capture with the earlier, uncompressed capture 20230302084617,
  downloaded at 15:54Z, 15:57Z and 16:25Z with the same identity.
- The new test rejects hand-made regressions: a successor's start, observation or oath used as an end (Mbeki, Zuma,
  Mlambo-Ngcuka, Mbete, Motlanthe, Ramaphosa, Mabuza) or a reappointment used as one (Mashatile 2023); a retrospective
  list end, a release announcement, an end by reference and an Assembly-seat resignation used as ends; announcements, an
  intention, a scheduled oath, a conflicting stated assumption, an oath, a list start and a publication date used as
  starts, and a designation day used as an observation; acting service and an interim designation added as holders; a
  continuation claim or a scheduled oath cited by a holder; a Deputy President added to `za_president_election`, a
  Deputy President claim moved onto the President's role, a President or ANC holder added to this role, the role copied
  to the ANC, moved to a second institution, given a head-of-state kind, doubled or removed; a list row or a month-only
  profile given a date; a receipt collapsed into the letter; the holder order changed; and checksum, snapshot-path,
  cited-source, unknown-claim and beyond-cutoff mutations.

C01 and all parent gates (C06, S23, WC1, CP1) stay open. No installed leader, avatar, portrait, campaign rule or
save schema changed.
