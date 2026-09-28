# CLAUDE-C01-24: Tongan Speakers of the Legislative Assembly, 1990–2026

Owner: Claude. State: **ready_for_review** (27 September 2026; submitted, not accepted). Parent: C01 (incomplete).
Report: [tonga-speakers-1990-2026-24.md](../../campaign-certification/C01/research/tonga-speakers-1990-2026-24.md).

Origin: a self-proposed follow-up packet, started on the user's 25 September 2026 instruction to start another
batch of five packets in parallel. It does not repeat accepted C01-01/02/03/04/07/08 or reclaim
the pending C01-05, C01-06 and C01-09 to C01-22. It is pending Codex acceptance and is not registered in `docs/planning/ai-workstreams.json`.

Branch: `claude/c01-to-24`. Base: `ffe54b02` (then `codex/campaign-certification`); not stacked on another pending packet. Claim
commit: `a91a8249` (this record only). Integration `a33a8987` was merged into the branch at `f1230bbb` before the packet
was completed; `codex/campaign-certification` advanced past it at 16:44 -0700 on 27 September and was `e41aa18d` by 19:18 -0700
that day (eleven commits touching S19, planning, UI and simulation files, none of this packet's paths); merging it is left to
the integrator.

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

## Result

Submitted `ready_for_review` on 27 September 2026 on `claude/c01-to-24` (claim `a91a8249`, base `ffe54b02`, integration
`a33a8987` merged at `f1230bbb`). Touched paths (nothing else):

- `docs/planning/ai-handoffs/CLAUDE-C01-24.md` (this record);
- `docs/campaign-certification/C01/research/tonga-speakers-1990-2026-24.md` (new report);
- `docs/campaign-certification/C01/research/tonga.json` (additions only: 49 sources and 82 claims appended after every
  earlier packet's; on `to_legislative_assembly`, the new ids appended to its sources and claims; on `to_speaker`, the new
  ids appended to its sources and claims, eleven holder observations appended after the unchanged string observation
  `to_speakers_appointment`, and a `scope_note`; five coverage notes appended on `to_legislative_assembly` and one on the
  packet);
- 49 new extracts `docs/campaign-certification/C01/research/sources/tonga-*-facts.json` (no existing extract edited);
- `tools/avatars/test_tonga_speakers_c01_24.py` (new); `tools/avatars/test_tonga_research_s10g.py`,
  `tools/avatars/test_tonga_dpfi_c01_04.py`, `tools/avatars/test_tonga_transition_c01_02.py` and
  `tools/avatars/test_tonga_pm_1990_2019_c01_08.py` (pins re-expressed exactly, none loosened);
- `docs/campaign-certification/C01/research-index.json` (regenerated, separate commit).

The `to_speaker` role gains eleven holder observations, in order: Fusitu'a (attested 1996-11-14); Hon. Veikune (attested
2001-04-30); Hon. Tu'ivakano (attested 2003-04-08); Hon. Veikune (attested 2005-03-22, until 2006-01-25, the stated end);
Lord Tu'ilakepa (attested 2008-06-02); Lord Lasike (attested 2011-01-14, until 2012-07-17, the revocation stated as
effective immediately); Lord Fakafanua (attested 2012-07-19); Lord Tu'ivakano (attested 2015-02-24); Lord Fakafanua
(attested 2018-03-05 and, separately, 2022-02-17); and Lord Vaea (attested 2026-05-19). No holder has a `from`; the 2025
appointment effective 18 December 2025 stays the existing string observation, which is not duplicated. Royal
appointments, the Assembly's elections and recommendations, oaths, resignations, the 2012 revocation's grounds, acting
Speakers (1995, 2000, 2012-2013, 2022, 2026), Interim Speakers (2010, 2025), notices, procedure and retrospective lists are
claims only. No organization, institution or role is added; `to_deputy_speaker` and every other Tonga holder are unchanged.

Observation decisions:

| ID | Decision |
|---|---|
| TO-SPK-01 | Unresolved: the Speaker on 1 January 1990 (only conflicting retrospective lists); no holder |
| TO-SPK-02 | Accepted in part: Fusitu'a (court records, 1996) and Veikune (2001) attested; acting service of 1995 and 2000 claims only |
| TO-SPK-03 | Accepted in part: Tu'ivakano attested 2003 (2002 dates contested); Veikune 22 Mar 2005 to 25 Jan 2006; Tu'iha'angana claims only; Tu'ilakepa attested 2008 |
| TO-SPK-04 | Accepted: the 2010 procedure, procedure only |
| TO-SPK-05 | Accepted in part: Lasike selected 21 Dec 2010, attested 14 Jan 2011; appointment day unresolved |
| TO-SPK-06 | Accepted in part: Lasike's revocation effective 17 Jul 2012; Fakafanua appointed 19 Jul 2012; acting service claims only |
| TO-SPK-07 | Accepted in part: Tu'ivakano attested 24 Feb 2015 |
| TO-SPK-08 | Accepted in part: Fakafanua sworn 18 Jan 2018, attested 5 Mar 2018 |
| TO-SPK-09 | Accepted in part: Fakafanua elected on or before 15 Dec 2021, attested 17 Feb 2022 |
| TO-SPK-10 | Accepted: Vaea elected (16 Dec 2025 item), appointed effective 18 Dec 2025 (string observation), sworn 22 Jan 2026, attested 19 May 2026 |

Decisions for the reviewer:

- Every recorded response was downloaded twice for this packet at least 30 minutes apart and matched; Internet Archive
  captures were fetched with `Accept-Encoding: identity`, and three gzip-stored captures record their decoded body with the
  transfer identity alongside.
- The five Assembly minutes the research had used are served only through a Phoca Download form that posts
  `license_agree=1` and a session token; the C01-08 reviewer declined that form, and this packet does not submit it. They
  are not sources; the Assembly's own news items cover their facts where a pre-cutoff record exists.
- Tu'iha'angana (2006-2008) has no holder: his only contemporaneous primary attestation is the Crown packet's Gazette No. 19
  claim, which CLAUDE-C01-07's accepted test reserves to the Crown entry. Existing Speaker-related claims of other packets
  are named in the report's ledger, not cited.
- Lord Fakafanua's 2012 holder is event-dated to 19 July 2012 as the Assembly's 23 July item states, although its Tongan item
  of 17:23 that day still called the appointment pending; 20 July is recorded as the conservative alternative.

Checker defects: of the 48 (A1-A21, B1-B14, C1-C13), 42 are applied, two are declined (A3 and C3, for the guard reason
above) and four are superseded because the minutes they concern are no longer sources (C4, C7, C8, C12). Of the nine
primary records the checks located, eight are imported and one optional further attestation is not; six more Assembly news
items were located for this packet.

Checks run on 28 September 2026 (UTC) in this sparse worktree (`docs/campaign-certification/C01`, `docs/planning`,
`spheres-sim/src`, `spheres-sim/data`, `spheres-web/data`, `tools/avatars`, `tools/planning`, `tools/ui`; not widened), with
`PYTHONDONTWRITEBYTECODE=1`:

- `python -X utf8 tools/avatars/campaign_research.py` then `--check`: exact regeneration passes; 9 packets, 841 organization
  and 34 institution observations, 1,378 sources, 3,813 claims, 93 open batches.
- `python -X utf8 tools/avatars/campaign_census.py --check`: passes (exit code 0).
- Tonga tests (`-p "test_tonga*.py"`): 79 pass (10 of them in the new `test_tonga_speakers_c01_24.py`).
- Research tests (`-p "test_*research*.py"`): 79 pass.
- Campaign tests (`-p "test_campaign*.py"`): all 16 pass, the 7 census tests included. These suites overlap; their counts
  are not summed.
- `node --test tools/ui/check_leadership_research_review.cjs`: 11 pass.
- `python tools/planning/workboard.py --check`: passes (44 markers).
- `git diff --check` on this packet's paths: clean.
- The new test rejects hand-made regressions both by rule and against the pinned lists: an acting Speaker, an Interim Speaker
  or Tu'iha'angana from IPU added as a holder; a duplicate holder of the 2025 appointment; the Assembly's selection, an oath or
  the first presiding day used as a start; the reported appointment day, a contested 2002 date or a retrospective list used
  as a holder date; a successor's appointment, a conviction or an expected successor used as an end; acting, election or list
  claims cited by a holder; the string observation moved or edited; the Deputy Speaker role extended; a second Speaker role;
  a year-only claim given a date; holders out of order; and checksum, path, beyond-cutoff, reversed-interval, foreign-source
  claim, unknown-claim and invalid-date mutations.

C01 and all parent gates (C06, S23, WC1, CP1) stay open. No installed leader, avatar, portrait, campaign rule or
save schema changed.
