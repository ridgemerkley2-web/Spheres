# CLAUDE-C01-41: CPSU General Secretary and Deputy General Secretary, 1990–1991

Owner: Claude. State: **ready_for_review** (submitted 1 October 2026 UTC, 30 September local; not complete). Parent: C01 (incomplete).

Origin: part of the back-to-back C01 research pipeline the user asked for on 28 September 2026, taken from the
certified-country gap ledger (`docs/campaign-certification/C01/gap-ledger/ledger.md`) items `USSR/su_cpsu`. Pending Codex
acceptance; not registered in the task queue.

Branch: `claude/c01-su-41`. Base: `02d2c5a2` (current `codex/campaign-certification`); not stacked on a pending packet. Claim commit: this record's first commit on the branch.

## Bounded deliverable

Date the existing su_cpsu roles su_cpsu_general_secretary and su_cpsu_deputy_general_secretary from 1 January 1990 to the party's end, adding dated holder observations without renaming or deleting the existing undated ones (the existing deputy holder's source spelling 'Ivashkov' needs identity care: note it, never reconcile it). Expected: Gorbachev as General Secretary (XXVIII Congress election of July 1990; his statement of 24 August 1991 giving up the office) and Ивашко as Deputy General Secretary (elected July 1990), with his acting service after 24 August 1991 as claims only. The suspension of the party's activity (29 August 1991) and the RSFSR presidential decree of 6 November 1991 are organization claims only. At most ten people. Sources: CPSU records (Pravda, the XXVIII Congress stenogram and resolutions), USSR Supreme Soviet Vedomosti, archive facsimiles and raw Internet Archive captures; news are leads.

Keep each distinct event (election or selection, appointment, assumption of office, acting or interim service,
resignation, removal, death, merger, renaming) as its own dated claim; acting service is claims only. Never infer an
end from a successor's start. Give a holder `from` or `until` only where a source states the day; otherwise record
`attested_on`. Party office and state office stay separate both ways. Organization identities, lifecycles and game
mappings stay unresolved; a name match to a simulation row is never a mapping. At most ten people. Primary sources
only; news and encyclopaedias are leads. The historical cutoff stays 7 September 2026.

## Allowed files and checks

Allowed files:

- this record;
- a new `docs/campaign-certification/C01/research/ussr-cpsu-general-secretary-1990-1991-41.md`;
- `docs/campaign-certification/C01/research/ussr.json` and specifically related new
  `docs/campaign-certification/C01/research/sources/ussr-*-facts.json` extracts;
- focused tests under `tools/avatars/`: a new `test_ussr_cpsu_general_secretary_c01_41.py`, and pinned counts or exact sets in `test_ussr_research_s10h.py`, `test_ussr_russia_transition_c01_05.py`, `test_ussr_government_supreme_soviet_c01_26.py`, `test_ussr_democratic_russia_soyuz_c01_35.py` updated to
  the new totals (none loosened).

Put generated `research-index.json` changes in a separate commit. Do not change the gap ledger, shared UI, the
roadmap, game data or other country packets.

Checks: research-index `--check`; `campaign_census.py --check`; the USSR, research and campaign Python tests
(census included); the atlas Node check; `workboard.py --check`; `git diff --check`.

Mark the packet `ready_for_review` when done. C01 and all parent gates stay open.

## Submission (ready_for_review)

[Report](../../campaign-certification/C01/research/ussr-cpsu-general-secretary-1990-1991-41.md):
`ussr-cpsu-general-secretary-1990-1991-41.md`. Branch `claude/c01-su-41`, claim commit `7778be5b` on `02d2c5a2`; **not stacked**.
Result commits: the packet commit `Add CLAUDE-C01-41: CPSU General Secretary and Deputy General Secretary, 1990–1991` and the
separate commit `Regenerate the C01 research index for CLAUDE-C01-41` at the head of `claude/c01-su-41` at submission; to be
recorded by the integrator. Research access 30 September 2026 (UTC; the second download pass ran on 1 October UTC). One research
pass, no sub-agents.

Decisions per observation (`su_cpsu`, existing roles `su_cpsu_general_secretary` and `su_cpsu_deputy_general_secretary`):

- SU-CPSU-01 accepted: Михаил Сергеевич Горбачев attested 1990-02-05 (the Central Committee's plenum communique in Pravda No. 37).
- SU-CPSU-02 accepted: the XXVIII Congress's ballot, vote, election and protocol approval on 10 July 1990 are claims; holder
  attested 1990-07-10 (the communique styles him "Генеральный секретарь ЦК КПСС, Президент СССР" after the election). No `from`.
- SU-CPSU-03 accepted: the deputy's ballot and vote (11 July), election announced and approved (12 July; vote figures in Pravda's
  report; the biography says 11 July) are claims; Владимир Антонович Ивашко attested 1990-07-13 (the Congress's programme
  commission list, Pravda No. 195). No `from`.
- SU-CPSU-04 accepted in part: Горбачев attested 1991-08-22 (the Secretariat's undated statement, Pravda No. 201); Ивашко attested
  1991-08-21 (Pravda's report of А. Дзасохов's press conference; ruling requested); the journal masthead of 10 July 1991 is a claim.
- SU-CPSU-05 accepted in part: his own words of 26 August 1991 that he laid down the duties and a deputy's reference of 3 September
  are claims with no day; the statement of 24 August 1991 is a lead; no `until`; the deputy's acting service was not found in a
  primary record (no acting holder).
- SU-CPSU-06 accepted in part: УП-2460 (24 Aug), the proposal of self-dissolution (26 Aug), the suspension 2371-I (29 Aug), the
  statement that the Central Committee did not decide to dissolve itself (3 Sep) and RSFSR decree No. 169 (6 Nov 1991) are
  organization claims only; no lifecycle end.

The S10.h observations ("Mikhail Gorbachev"; "Vladimir Ivashkov (source spelling; identity reconciliation pending)") are unchanged
and stay first in each role; the Russian names are never reconciled with them. No holder has a `from` or an `until`. Party and
state offices stay separate; nothing is added to `russia.json`. Two people are researched.

Rulings requested: (1) Internet Archive items uploaded by private accounts (Pravda, Известия ЦК КПСС) as hosts of the party's own
records: all five new holders rest on them; (2) Pravda's report of a Central Committee secretary's press conference as attestation
of the deputy's office (21 August 1991); (3) whether his own words of 26 August 1991, with the statement's date only in leads,
should ever give `until` 1991-08-24 (none set); (4) whether a congress re-election on a stated day gives `from` (none set, as in
CLAUDE-C01-26). The SSSR.SU host ruling of CLAUDE-C01-26 (C12) also applies to four sources.

Touched paths: this record; `docs/campaign-certification/C01/research/ussr.json` (12 sources, 26 claims, five holder observations
appended to the two existing roles after the S10.h holders, a scope note per role, four `su_cpsu` coverage items and one packet
coverage item; no existing source, claim, holder, role title or kind, or lifecycle changed); 12 new
`docs/campaign-certification/C01/research/sources/ussr-*-facts.json` extracts (no existing extract edited); the new
`docs/campaign-certification/C01/research/ussr-cpsu-general-secretary-1990-1991-41.md`; the new
`tools/avatars/test_ussr_cpsu_general_secretary_c01_41.py`; pinned counts, exact sets and access dates re-expressed (none
loosened) in `tools/avatars/test_ussr_research_s10h.py`, `tools/avatars/test_ussr_government_supreme_soviet_c01_26.py` and
`tools/avatars/test_ussr_democratic_russia_soyuz_c01_35.py`; `test_ussr_russia_transition_c01_05.py` and `russia.json` unchanged.
Separate commit: `docs/campaign-certification/C01/research-index.json` only.

Integration notes: the user chose to start this batch (CLAUDE-C01-38 to C01-41, run in parallel in other country files) before
Codex's roadmap line "continue existing claims first". `research-index.json` is the only file shared with the parallel packets;
regenerate it rather than merging if another packet lands first.

Checks (1 October 2026 UTC, sparse worktree, after merging `codex/campaign-certification` at `79ef97ec`): the research index
regenerated and `--check` passes (1,898 sources, 4,753 claims, 844 organization and 36 institution observations, 93 discovery
batches); the USSR tests (46, 9 of them new) and the Russia tests (29) pass; the research tests (79) and the campaign tests (16)
pass; the atlas Node check passes (11); `workboard.py --check` passes (44 markers); `git diff --check` on this packet's paths is
clean. The new test's 19 mutations each fail on the rule they break. `packet_check.py 41` (head `befa7c36`, base `79ef97ec`):
12 new sources, all 12 re-downloaded responses match their recorded byte counts and SHA-256; every check passes except
`census --check` (below); it lists the re-expressed pinned-test lines for review. Known failures outside these checks, not fixed: `campaign_census.py --check` exits 1 on the
integration head `79ef97ec` itself, because commits there (`ace1f233`, `434abd50`, `7c6f112c`) changed
`spheres-sim/src/government.rs` without regenerating `census.json` (regenerating to a scratch directory shows that `census.json`'s
record of that input is the only difference: 849,546 → 851,150 bytes, SHA-256 `3f846b4b…` → `4b0b82db…`; the other four outputs
are identical); `test_certified_gap_ledger.py` reports "no pinned attribution" for this packet's sources until Codex classifies
its commit; `test_certified_boundary_matrix.py` (S23) needs `spheres-web/src`, absent from the sparse checkout, and Codex
regenerates the boundary matrix on integration.

C01 and all parent gates stay open.
