# CLAUDE-C01-36: Tongan Deputy Prime Ministers, 1990–2026

Owner: Claude. State: **ready_for_review** (29 September 2026 UTC; submitted, not accepted). Parent: C01 (incomplete).
Report: [tonga-deputy-prime-ministers-1990-2026-36.md](../../campaign-certification/C01/research/tonga-deputy-prime-ministers-1990-2026-36.md).

Origin: part of the back-to-back C01 research pipeline the user asked for on 28 September 2026, taken from the
certified-country gap ledger (`docs/campaign-certification/C01/gap-ledger/ledger.md`) items `to_cabinet#to_deputy_pm`. Pending Codex
acceptance; not registered in the task queue.

Branch: `claude/c01-to-36`. Base: `44098c5a` (current `codex/campaign-certification`); not stacked on a pending packet. Claim commit: this record's first commit on the branch.

## Bounded deliverable

Extend the existing to_deputy_pm role of to_cabinet (which holds holders from 2021 onward) with the Deputy Prime Ministers from 1 January 1990 to 2021 — at most ten people. Royal appointments, Cabinet announcements, acting service and resignations are distinct dated claims; acting Prime Minister service by a Deputy is a claim on to_pm only, never a to_pm holder. Existing holders do not change. Sources: the PMO, Palace Office, Government Gazette, Legislative Assembly records; IPU only as corroboration.

Keep each distinct event (election or selection, appointment, assumption of office, acting or interim service,
resignation, removal, death, merger, renaming) as its own dated claim; acting service is claims only. Never infer an
end from a successor's start. Give a holder `from` or `until` only where a source states the day; otherwise record
`attested_on`. Party office and state office stay separate both ways. Organization identities, lifecycles and game
mappings stay unresolved; a name match to a simulation row is never a mapping. At most ten people. Primary sources
only; news and encyclopaedias are leads. The historical cutoff stays 7 September 2026.

## Allowed files and checks

Allowed files:

- this record;
- a new `docs/campaign-certification/C01/research/tonga-deputy-prime-ministers-1990-2026-36.md`;
- `docs/campaign-certification/C01/research/tonga.json` and specifically related new
  `docs/campaign-certification/C01/research/sources/tonga-*-facts.json` extracts;
- focused tests under `tools/avatars/`: a new `test_tonga_deputy_prime_ministers_c01_36.py`, and pinned counts or exact sets in `test_tonga_research_s10g.py`, `test_tonga_*_c01_*.py` updated to
  the new totals (none loosened).

Put generated `research-index.json` changes in a separate commit. Do not change the gap ledger, shared UI, the
roadmap, game data or other country packets.

Checks: research-index `--check`; `campaign_census.py --check`; the Tonga, research and campaign Python tests
(census included); the atlas Node check; `workboard.py --check`; `git diff --check`.

Mark the packet `ready_for_review` when done. C01 and all parent gates stay open.

## Result

Submitted `ready_for_review` on 29 September 2026 (UTC; 28 September local) on `claude/c01-to-36` (claim `e8821f40`, base
`44098c5a`, not stacked). Report:
[tonga-deputy-prime-ministers-1990-2026-36.md](../../campaign-certification/C01/research/tonga-deputy-prime-ministers-1990-2026-36.md).
Touched paths (nothing else):

- `docs/planning/ai-handoffs/CLAUDE-C01-36.md` (this record);
- `docs/campaign-certification/C01/research/tonga-deputy-prime-ministers-1990-2026-36.md` (new report);
- `docs/campaign-certification/C01/research/tonga.json` (additions only: 39 sources and 53 claims appended after every
  earlier packet's; on `to_cabinet` and `to_deputy_pm`, the 45 Deputy Prime Minister claims and their sources appended; on
  `to_prime_minister` and `to_pm`, the 8 acting-premiership claims and their sources appended; ten holder observations inserted
  after the unchanged string observation `to_cabinet_appointment` and before the three unchanged existing holders; a
  `to_deputy_pm` `scope_note`; six coverage notes appended on `to_cabinet`, one on `to_prime_minister` and one on the packet);
- 39 new extracts `docs/campaign-certification/C01/research/sources/tonga-*-facts.json` (no existing extract edited);
- `tools/avatars/test_tonga_deputy_prime_ministers_c01_36.py` (new); `tools/avatars/test_tonga_research_s10g.py`,
  `tools/avatars/test_tonga_speakers_c01_24.py`, `tools/avatars/test_tonga_transition_c01_02.py`,
  `tools/avatars/test_tonga_transition_c01_03.py`, `tools/avatars/test_tonga_pm_1990_2019_c01_08.py` and
  `tools/avatars/test_tonga_dpfi_c01_04.py` (pins re-expressed exactly, none loosened);
- `docs/campaign-certification/C01/research-index.json` (regenerated, separate commit).

The `to_deputy_pm` role gains ten holder observations of nine people, in order: Langi Kavaliku (attested 2000-11-06); Tevita
Poasi Tupou (from 2001-01-24, until 2001-09-28, the royal consent's stated effective day and the day his resignation was made
and accepted); James Cecil Cocker (attested 2002-05-13); Viliami Ta'u Tangi (attested 2006-08-08); Samiu Kuita Vaipulu (from
2011-01-04, the Cabinet's stated commencement); Siaosi Sovaleni (attested 2014-12-31, the Cabinet list; the effective day is only a forecast); Lord Ma'afu (from
2017-09-01, the royal endorsement's stated effect); Semisi Kioa Lafu Sika (from 2018-01-05); Sione Vuna Fa'otusia (from
2019-10-09); and Lord Ma'afu (attested 2021-04-21, until 2021-12-12, his death in office as the PMO states). The existing
holders Poasi Mataele Tei, Samiu Vaipulu (2024) and Taniela Likuohihifo Fusimalohi are unchanged. Recommendations, reports of
appointments, the day a royal endorsement was conveyed, oaths, the 2004 reshuffle confirmation, the 2000 retirement
announcement, the 2017 revocation recommendation, retrospective statements and Assembly lists, the office named without its
holder in June 2006, acting Deputy Prime Ministers (Edwards 2001-2002, Paunga 2002; claims on `to_deputy_pm`) and Deputies
acting as Prime Minister (2005-2017; claims on `to_pm`) are claims only. No organization, institution or role is added.

Observation decisions:

| ID | Decision |
|---|---|
| TO-DPM-01 | Unresolved: the Deputy Prime Minister on 1 January 1990 (no 1990-1991 primary record; "appointed ... in 1991" is year-only); no holder |
| TO-DPM-02 | Accepted in part: Kavaliku attested 6 Nov 2000; the retirement due 11 Nov 2000 is an announcement, not an end |
| TO-DPM-03 | Accepted: Tupou from 24 Jan 2001 until 28 Sep 2001, both stated; Edwards acting from 28 Sep 2001 (claim) |
| TO-DPM-04 | Accepted in part: Cocker attested 13 May 2002 to 8 Mar 2006; appointment day and end unresolved; acting Deputies and his acting premiership are claims |
| TO-DPM-05 | Accepted in part: Tangi attested 8 Aug 2006 to 30 Dec 2010; "In May 2006" retrospective; no end |
| TO-DPM-06 | Accepted in part: Vaipulu from 4 Jan 2011; no end; acting premierships are claims |
| TO-DPM-07 | Accepted in part: Sovaleni attested 31 Dec 2014 to 30 Aug 2017; the 2014 effective day is a forecast and the 2017 revocation only recommended, so no start or end |
| TO-DPM-08 | Accepted in part: Lord Ma'afu from 1 Sep 2017; Sika from 5 Jan 2018; no end for either |
| TO-DPM-09 | Accepted in part: Fa'otusia from 9 Oct 2019; resignation "in 2020" only; no end |
| TO-DPM-10 | Accepted in part: Lord Ma'afu attested 21 Apr 2021, until his death on 12 Dec 2021 |

Decisions for the reviewer:

- Every recorded response is a raw Internet Archive capture made before the cutoff, fetched with `Accept-Encoding: identity`
  and curl's default user agent, and was downloaded twice for this packet at least 30 minutes apart with identical bytes and
  SHA-256. None was returned gzip-encoded.
- Sovaleni has neither `from` nor `until`: the PMO release of 31 December 2014 states the Prime Minister-elect's recommendation
  and that the appointments 'will be effective as of 31st December, 2014' (IPU gives 19 January 2015), and the release of 6
  September 2017 gives the revocation's recommended effective day (1 September 2017) while the royal endorsement it reports
  covers the new appointments; CLAUDE-C01-08's claim of the King's consent to the removal is reserved by that packet's test and
  is named, not cited. Setting either boundary would need the integrator to accept a recommendation, or that pairing, as the
  royal act.
- Kavaliku's retirement "due ... commencing on the 11th November 2000" is treated as a prospective announcement, not an end.
- Holder names are printed without honorifics, as on the existing holders of the role. Three of them (Langi Kavaliku, Viliami
  Ta'u Tangi, Semisi Kioa Lafu Sika) carry names that CLAUDE-C01-08's and CLAUDE-C01-04's accepted tests kept out of every
  holder, to stop acting Prime Ministers and court parties becoming holders; those guards now exempt only these exact
  `to_deputy_pm` observations and assert that no other holder carries the names.
- The Tongan version of the 22 January 2018 release is imported for Sika's effective day because the English version is
  CLAUDE-C01-08's source and its claim is reserved; both are separate responses.

Independent check: fourteen defects (D1-D14); twelve applied (the second download round, Sovaleni's start removed because it
rested on a recommendation, the death-notice styling undated, 33 claims rewritten with short quotations, three locators, the
2006 relative date, an unsourced note, quotation characters, two scope and coverage wordings), one declined (D13, access-date
convention) and one declined in part (D14, companion pages read in the research). See the report's Checker defects.

Checks run on 29 September 2026 (UTC) in this sparse worktree (`docs/campaign-certification/C01`, `docs/planning`,
`spheres-sim/src`, `spheres-sim/data`, `spheres-web/data`, `tools/avatars`, `tools/planning`, `tools/ui`; not widened), with
`PYTHONDONTWRITEBYTECODE=1`:

- `python -X utf8 tools/avatars/campaign_research.py` then `--check`: exact regeneration passes; 9 packets, 841 organization
  and 35 institution observations, 1,610 sources, 4,235 claims, 93 open batches.
- `python -X utf8 tools/avatars/campaign_census.py --check`: **fails (exit 1) on the integration base itself**, not because
  of this packet. Integration commit `262d5f61` changed `spheres-sim/src/government.rs` (848,551 to 849,546 bytes; SHA-256
  `0e2dbdef…a2b73f` to `3f846b4b…a8d202`) without regenerating `docs/campaign-certification/C01/census.json`; regenerating
  into a scratch directory shows that the stale `government.rs` entry (with `all_declared_inputs_current` and
  `stale_inputs`) is the only difference and the other four census files are byte-identical. Left for the integrator.
- Tonga tests (`-p "test_tonga*.py"`): 89 pass (10 of them in the new `test_tonga_deputy_prime_ministers_c01_36.py`).
- Research tests (`-p "test_*research*.py"`): 79 pass.
- Campaign tests (`-p "test_campaign*.py"`): all 16 pass, the census tests included. These suites overlap; their counts are
  not summed.
- `node --test tools/ui/check_leadership_research_review.cjs`: 11 pass.
- `python tools/planning/workboard.py --check`: passes (44 markers).
- `git diff --check` on this packet's paths: clean.
- Known failures outside these checks, disclosed and not fixed: `tools/avatars/test_certified_gap_ledger.py` errors on this
  packet's sources ("Source to_pmo_2000_press_releases (Tonga) has no pinned attribution") until Codex classifies the
  packet's commit in `COMMIT_PACKETS`; `tools/avatars/test_certified_boundary_matrix.py` (S23) needs
  `spheres-web/src/person_avatar_assets.rs`, which the sparse checkout lacks, and in a full checkout would report the packet
  as `unclassified_packet` with stale boundary-matrix files, so Codex must list the packet and regenerate
  `docs/campaign-certification/S23/preparation/boundary-matrix/` on integration.
- The new test rejects hand-made regressions by rule and against the pinned lists: a successor's start, a retirement
  announcement, a revocation recommendation or a month-only list used as an end; a report date, the Palace letter, an oath, a
  retrospective month or IPU's Cabinet day used as a start; an acting Deputy added as a holder; a Deputy's acting premiership
  added as a to_pm holder or cited by a holder; recommendation, list and other packets' claims cited by a holder; a Deputy
  claim cited by the Prime Minister's holder; an acting premiership moved onto the Deputy role; an existing holder changed;
  the string observation moved; holders out of order; a year-only claim given a date; a second Deputy role; an exempt name on
  another role; and checksum, path, beyond-cutoff, reversed-interval, foreign-source claim, unknown-claim and invalid-date
  mutations.

C01 and all parent gates (C06, S23, WC1, CP1) stay open. No installed leader, avatar, portrait, campaign rule or save schema
changed.
