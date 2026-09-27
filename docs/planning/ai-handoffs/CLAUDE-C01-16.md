# CLAUDE-C01-16: ANC presidents, 1990–2026

Owner: Claude. State: **ready_for_review** (24 September 2026; submitted, not accepted). Parent: C01 (incomplete).
Report: [south-africa-anc-presidents-1990-2026-16.md](../../campaign-certification/C01/research/south-africa-anc-presidents-1990-2026-16.md).

Origin: a self-proposed follow-up packet, started on the user's 24 September 2026 instruction to continue
development faster with several packets in parallel. It does not repeat accepted C01-01/02/03/04/07/08 or reclaim
the pending C01-05 (USSR/RSFSR), C01-06 (Saudi Arabia), C01-09 (South Africa), C01-10 (Brazil), C01-11 (India) and C01-12 (Japan, 1990-2006). It is pending Codex acceptance and is not registered in `docs/planning/ai-workstreams.json`.

Branch: `claude/c01-za-16`. **Stacked on CLAUDE-C01-09** (`claude/c01-za-09` at `82a23f9d`), which is
ready for review and not yet integrated, because both packets edit `south-africa.json`: merge CLAUDE-C01-09 first. Claim
commit: `6e9a489b` (this record only). CLAUDE-C01-09 is itself based on `ffe54b02` (`codex/campaign-certification`).

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

## Result

Submitted `ready_for_review` on 24 September 2026. Before the work, `claude/c01-za-09` was fetched from the remote; it
had no commit missing from this branch, so nothing was merged. Touched paths (nothing else):

- `docs/planning/ai-handoffs/CLAUDE-C01-16.md` (this record);
- `docs/campaign-certification/C01/research/south-africa-anc-presidents-1990-2026-16.md` (new report);
- `docs/campaign-certification/C01/research/south-africa.json`;
- 54 new extracts `docs/campaign-certification/C01/research/sources/south-africa-anc*-facts.json`;
- `tools/avatars/test_south_africa_anc_presidents_c01_16.py` (new), `tools/avatars/test_south_africa_research_s10h.py`
  and `tools/avatars/test_south_africa_heads_of_state_c01_09.py`;
- `docs/campaign-certification/C01/research-index.json` (regenerated, separate commit; the only file shared with other
  pending packets).

The packet gains 54 sources and 73 claims and the role `za_anc_president` on the `AFRICAN NATIONAL CONGRESS`
observation (`za_iec_n2024_014`), whose IEC identity, unknown lifecycle and empty game mapping are unchanged. The role
has ten holder observations, one per reviewed observation, each dated by in-office attestations of its own day and none
with a start or an end, because no ANC record reviewed states one: Oliver Tambo (1990-01-08), Nelson Mandela
(1991-07-18, 1994-12-22), Thabo Mbeki (1997-12-20, 2002-12-20), Jacob Zuma (2007-12-20, 2012-12-20) and Cyril Ramaphosa
(2017-12-20, 2023-01-08, 2026-05-15). Conference and NEC elections, result lists and publications, election
references, acceptances, nominations, handover speeches, departure statements, predecessor references, the Deputy
President and National Chairman claims and continuation attestations are claims only. No `za_presidency` claim or
source feeds the ANC role, none of its claims feeds `za_presidency`, and the CLAUDE-C01-09 presidency holders are
unchanged.

Observation decisions:

| ID | Decision |
|---|---|
| ZA-ANC-01 | Accepted in part: Tambo observed 8 Jan 1990; no acting President named; Mandela elected Deputy President by the NEC (claim only); no start or end |
| ZA-ANC-02 | Accepted in part: election reported for the conference of 2-7 (or 2-6) Jul 1991; Mandela observed 18 Jul 1991; election day not stated |
| ZA-ANC-03 | Accepted in part: NEC list "December 1994"; Mandela observed 22 Dec 1994; election day not stated |
| ZA-ANC-04 | Accepted in part: office bearers elected "on the second day of conference" (not stored as a date); Mbeki observed 20 Dec 1997; no end for Mandela |
| ZA-ANC-05 | Accepted in part: NEC list as elected (16-20 Dec 2002); Mbeki observed 20 Dec 2002 |
| ZA-ANC-06 | Accepted in part: vote 18 Dec, results published under 19 Dec, acceptance 20 Dec 2007; Zuma observed 20 Dec 2007; no end for Mbeki |
| ZA-ANC-07 | Accepted in part: election referred to on 18 and 19 Dec 2012; Zuma observed 20 Dec 2012 |
| ZA-ANC-08 | Accepted in part: elected 18 Dec 2017 (election reference); Ramaphosa observed 20 Dec 2017; no end for Zuma |
| ZA-ANC-09 | Accepted in part: Nasrec session 16-20 Dec 2022, concluded 5 Jan 2023; Ramaphosa observed 8 Jan 2023 |
| ZA-ANC-10 | Accepted: NEC support for its President, statement of 15 May 2026; Ramaphosa observed 15 May 2026 |

All 45 checker defects (A1-A11, B1-B16, C1-C18) are resolved: 42 applied, A7 and B9 applied in part (a low-value
capture and a homepage without a day left as leads) and C2 resolved by removing the 31 August 2026 officials page, whose
content dates from February 2023. Eleven missing primary records found by the checks are imported; seven are leads, for
the reasons in the report's Checker defects and Leads sections.

Checks run on 24 September 2026 in this sparse worktree (`docs/campaign-certification/C01`, `docs/planning`,
`tools/avatars`, `tools/planning`, `tools/ui`; not widened), with `PYTHONDONTWRITEBYTECODE=1`:

- `python -X utf8 tools/avatars/campaign_research.py` then `--check`: exact regeneration passes; 9 packets,
  841 organization and 27 institution observations, 265 sources, 1,913 claims, 92 open batches.
- South Africa tests (`-p "test_south_africa*.py"`): 26 pass (8 of them new in
  `test_south_africa_anc_presidents_c01_16.py`).
- Research tests (`-p "test_*research*.py"`): 79 pass.
- Campaign tests (`-p "test_campaign*.py"`): the 9 `test_campaign_research` tests pass; `test_campaign_census`
  errors in `setUpClass` because `spheres-sim/data/party_leaders.json` is not in this sparse worktree. It was not
  run, and the sparse checkout was not widened. These suites overlap; their counts are not summed.
- `node --test tools/ui/check_leadership_research_review.cjs`: 11 pass.
- `python tools/planning/workboard.py --check`: passes (44 markers).
- `git diff --check`: clean for this packet's paths.
- The new test rejects hand-made regressions: a successor's observation used as an end (Tambo, Mbeki 2002, Zuma 2012)
  or a re-election used as one (Mandela 1991); an election day, election reference, derived conference day,
  declaration date or in-office observation used as a start; an election reference or a result publication used as an
  observation date; deputy service or a continuation attestation added as a holder; a continuation or election claim
  cited by a holder; a presidency holder added to the ANC role and an ANC holder added to the Presidency; a presidency
  claim moved onto the ANC role and the reverse; the ANC role copied to another party, doubled or removed; a lifecycle
  start; a month-only list given a date; the holder order changed; and checksum, cited-source, unknown-claim, game
  mapping and beyond-cutoff mutations.

C01 and all parent gates (C06, S23, WC1, CP1) stay open. No installed leader, avatar, portrait, campaign rule or
save schema changed.
