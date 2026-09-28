# CLAUDE-C01-33: Janata Dal presidents, 1990–2026

Owner: Claude. State: **ready_for_review** (28 September 2026; submitted, not accepted). Parent: C01 (incomplete).
Report: [india-janata-dal-presidents-1990-2026-33.md](../../campaign-certification/C01/research/india-janata-dal-presidents-1990-2026-33.md).

Origin: part of the back-to-back C01 research pipeline the user asked for on 28 September 2026, taken from the
certified-country gap ledger (`docs/campaign-certification/C01/gap-ledger/ledger.md`) items `India/in_jd`. Pending Codex
acceptance; not registered in the task queue.

Branch: `claude/c01-in-33`. Claimed while **stacked on `claude/c01-in-27`** (then a pending, unmerged packet that also edits `india.json`), merged with integration `3ec6e155`; CLAUDE-C01-27 has since been integrated, and this branch has merged current integration `032cd6a3` (at `a266ebcb`), so nothing needs merging first. Claim commit: `89decb6a` (this record only).

## Bounded deliverable

Research the national presidents of the Janata Dal from 1 January 1990 to its splits and eventual successors, as far as sources tie them to the Janata Dal name — at most ten people. The Janata Dal has no current ECI national-party observation, so add a new organization observation only from a primary record of the Janata Dal itself (ECI party lists or notifications naming it, or its own records), with one role in_jd_president (kind party_leader). Splits (Janata Dal (Secular), Janata Dal (United) and others) are claims about the organization only, never merged identities or inherited leaders. Sources: ECI records, Gazette notifications, Lok Sabha records, party records.

Keep each distinct event (election or selection, appointment, assumption of office, acting or interim service,
resignation, removal, death, merger, renaming) as its own dated claim; acting service is claims only. Never infer an
end from a successor's start. Give a holder `from` or `until` only where a source states the day; otherwise record
`attested_on`. Party office and state office stay separate both ways. Organization identities, lifecycles and game
mappings stay unresolved; a name match to a simulation row is never a mapping. At most ten people. Primary sources
only; news and encyclopaedias are leads. The historical cutoff stays 7 September 2026.

## Allowed files and checks

Allowed files:

- this record;
- a new `docs/campaign-certification/C01/research/india-janata-dal-presidents-1990-2026-33.md`;
- `docs/campaign-certification/C01/research/india.json` and specifically related new
  `docs/campaign-certification/C01/research/sources/india-*-facts.json` extracts;
- focused tests under `tools/avatars/`: a new `test_india_janata_dal_presidents_c01_33.py`, and pinned counts or exact sets in `test_india_research_s10e.py`, `test_india_prime_ministers_c01_11.py`, `test_india_presidents_c01_15.py`, `test_india_inc_presidents_c01_20.py`, `test_india_bjp_presidents_c01_27.py` updated to
  the new totals (none loosened).

Put generated `research-index.json` changes in a separate commit. Do not change the gap ledger, shared UI, the
roadmap, game data or other country packets.

Checks: research-index `--check`; `campaign_census.py --check`; the India, research and campaign Python tests
(census included); the atlas Node check; `workboard.py --check`; `git diff --check`.

Mark the packet `ready_for_review` when done. C01 and all parent gates stay open.

## Result

Submitted `ready_for_review` on 28 September 2026 (UTC). Claim commit: `89decb6a` (this record only), made while the
branch was stacked on the then-pending packet CLAUDE-C01-27 (`claude/c01-in-27`) merged with integration `3ec6e155` at
`6fdb950e`. CLAUDE-C01-27 has since been integrated, and the branch merged current integration `032cd6a3` at `a266ebcb`
(only the generated research index conflicted; it was taken from integration and regenerated), so the base is
`codex/campaign-certification` at `032cd6a3` and nothing needs merging first. CLAUDE-C01-27's records and its test are
unchanged apart from the re-expressed pins and the comment listed below. The independent verifier's fixes and the user's
rulings on them were applied on 28 September 2026 (commit `Apply verifier fixes to CLAUDE-C01-33`). Touched paths (nothing
else):

- `docs/planning/ai-handoffs/CLAUDE-C01-33.md` (this record);
- `docs/campaign-certification/C01/research/india-janata-dal-presidents-1990-2026-33.md` (new report);
- `docs/campaign-certification/C01/research/india.json` (additions only: 22 sources after the C01-27 sources; one new
  organization observation, `in_eci_19980110_np_06` (Janata Dal, row 6 of the Election Commission's national-party table
  of 10 January 1998), appended after the 82 recognition rows, with the role `in_jd_president`; one packet coverage note
  after the C01-27 note);
- 22 new extracts `docs/campaign-certification/C01/research/sources/india-*-facts.json` (no existing extract edited);
- `tools/avatars/test_india_janata_dal_presidents_c01_33.py` (new), and `tools/avatars/test_india_research_s10e.py`,
  `tools/avatars/test_india_prime_ministers_c01_11.py`, `tools/avatars/test_india_presidents_c01_15.py`,
  `tools/avatars/test_india_inc_presidents_c01_20.py` and `tools/avatars/test_india_bjp_presidents_c01_27.py` (pinned
  counts, exact sets, source lists, hosts, access dates, mapping_pending and work-order sizes only, re-expressed exactly;
  none loosened and no assertion removed; in the C01-11, C01-15 and C01-27 tests the comment on the C01-33 pins says the
  packet was claimed while stacked and is now based on integration);
- `docs/campaign-certification/C01/research-index.json` (regenerated, separate commit; the only file shared with other
  pending packets).

The packet gains 22 sources and 41 claims (14 about the organisation, with `role_id` null; 27 about the office), one
organization observation and one party role (`in_jd_president`, President of the Janata Dal, `party_leader`). The
observation's lifecycle is `unresearched`, its coverage `reporting_identity_only`, its game mapping empty, and no successor
is merged into it. The role has two holder observations, none with a stated start or end: S. R. Bommai (observed 14 July
1990, V. P. Singh's letter as the Janata Dal's leader to 'Shri S. R. Bommai, President, Janata Dal' on party business,
printed in the Rajya Sabha's answer of 28 August 1990, which decided nothing about the office) and Laloo Prasad Yadav
(observed 15 July 1996, 'the party President', Lok Sabha, in English, by a member who is not a Janata Dal officer, in a
passage naming 'the ruling Janata Dal'; disclosed). Sharad Yadav has no holder observation: neither Lok Sabha statement of
29 July 1997 ('party president, Shri Sharad Yadav'; 'Democratically elected president Shri Sharad Yadav') names the Janata
Dal, one names no party office and neither speaker is identified as a Janata Dal officer, so both are leads
(`party_not_named_lead`) that never feed a holder; he stays in the packet through his claims. V. P. Singh is attested only
before the period (28 December 1989), and the working presidency (Sharad Yadav, 17 March 1997), the rival claims of Shri
Ajit Singh (5 February 1992) and Shri H. D. Deve Gowda (1999), the removal claimed on 21 July 1999, the endorsement claimed on
29 July 1999, the Commission's statement of its records (7 August 1999), continuations, lists, recollections and spans are
claims only. The Election Commission's 1993 and 1999 dispute orders and table entries, and the Janata Dal (A), (Secular)
and (United) groups, are claims about the organisation, never merged identities or inherited leaders. The prime-minister,
president, Congress-president and BJP-president holders are unchanged, and no claim or source is shared with any other
role or institution.

Decisions for the reviewer:

- The new observation is taken from the Commission's table of 10 January 1998 because the Janata Dal has no row in the
  2024 notifications; the earlier table entries (1993, 1996) are organisation claims, and 'Janta Dal' (1996) is kept as
  printed.
- Lists under the heading 'Presidents of all Major National Political Parties' (1996, 1997) and a Lok Sabha statement of
  1991 describing the 1990 composition of the National Integration Council are claims, never observations.
- Evidence standard for party offices in parliamentary records: CLAUDE-C01-29 (Japan) admitted Diet minutes only where a
  party officer's statement records the party office, while India's integrated CLAUDE-C01-20 and CLAUDE-C01-27 accept
  other members' statements naming a party president (Kesri, Rao, Rajiv Gandhi, Joshi). This packet keeps Bommai (party
  letter) and Laloo Prasad Yadav (a non-member's statement naming the party) under the India practice and demotes Sharad
  Yadav (party not named). A single cross-country ruling is for the integrator.
- Four Lok Sabha files are Internet Archive copies of Parliament Digital Library PDFs (mirror-only; the eparlib hosts could
  not be reached and their official URLs were never captured) and the fifth is the raw Internet Archive capture of its
  official URL (2 December 2021); two Gazette issues (1998, 1999) are Internet Archive copies of eGazette files, the Goa Gazette of 9
  September 1993 is an Internet Archive copy of the Goa Government's file, and the Commission's order of 7 August 1999 is a
  copy in the IFES Election Judgments database (the Commission's host answered HTTP 406). Each is disclosed as such.

Observation decisions:

| ID | Decision |
|---|---|
| JD-PRES-01 | Declined: V. P. Singh attested 28 Dec 1989 (before the period); the 1990 recollection names nobody; no holder |
| JD-PRES-02 | Accepted in part: Bommai observed 14 Jul 1990; continuations 28 Aug 1990 and 29 Mar 1991; a 1990 composition list; his 1989 recollection |
| JD-PRES-03 | Accepted in part: claims only; Ajit Singh's rival claim (5 Feb 1992); the Commission's 1993 freeze, interim groups and final order of 22 Jul 1993 |
| JD-PRES-04 | Accepted in part: Laloo Prasad Yadav observed 15 Jul 1996 (a non-member's statement naming the ruling Janata Dal; disclosed); continuations 14 Oct 1996 and 22 Apr 1997; RJD office by 29 Jul 1997; Bommai's 1990-1996 span retrospective |
| JD-PRES-05 | Accepted in part: claims only; Sharad Yadav working President 17 Mar 1997; the statements of 29 Jul 1997 do not name the party (leads, never a holder); heading list 29 Sep 1997 |
| JD-PRES-06 | Accepted in part: claims only; the 1999 dispute (removal claimed, rival election, the Commission's records statement, ad hoc recognition of both groups) |
| JD-PRES-07 | Accepted: the new organization observation from row 6 of the table of 10 Jan 1998 |
| JD-PRES-08 | Accepted in part: the name and symbol under dispute (9 Aug 1999) and the Janata Dal (Secular) and (United) rows; no holder tied to the name after 7 Aug 1999 |

Checks: see the report's Checks section. research-index `--check`, the India (57), research (79) and campaign (16) tests,
the atlas Node check (11), `workboard.py --check` and `git diff --check` pass. `campaign_census.py --check` fails on a stale
`spheres-sim/src/government.rs` hash inherited from integration `032cd6a3` (changed by integration commit `262d5f61`,
after `4a3d0572`, without regenerating `census.json`); this packet touches no census input. `tools/avatars/test_certified_gap_ledger.py` fails on this branch until Codex
classifies the new commits (disclosed, not fixed; the gap ledger is untouched). The S23 boundary-matrix test needs
`spheres-web/src`, which is absent from this sparse checkout; **Codex must regenerate that matrix when it merges**.
