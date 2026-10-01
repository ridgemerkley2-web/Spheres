# CLAUDE-C01-49: USSR heads of government and President: dated attestations, 1990–1991

> Independent review, 1 October 2026: accept six sources, 17 claims and **four Presidential holder observations**. Withhold the proposed Pavlov observation of 22 January 1991: the title between two names in the printed signature block is ambiguous. It remains an `ambiguous_signature_block` claim with no identified holder. The unchanged submission below records the author's original proposal, including its five-observation count, and is superseded on that point by this review. The existing three government observations are unchanged. See the C01-49 independent review receipt; no canonical qualification is granted.


Owner: Claude. State: **ready_for_review** (submitted 1 October 2026 UTC; not complete). Parent: C01 (incomplete).

Origin: part of the back-to-back C01 research pipeline the user asked for on 28 September 2026, taken from the
certified-country gap ledger (`docs/campaign-certification/C01/gap-ledger/ledger.md`) items `su_government#su_government_head`, `su_presidency#su_president`. Pending Codex
acceptance; not registered in the task queue.

Branch: `claude/c01-su-49`. Base: `5ea4f8fc` (current `codex/campaign-certification`); not stacked on a pending packet. Claim commit: this record's first commit on the branch.

## Bounded deliverable

Fill the unresolved intervals of the existing roles su_government_head (Совет Министров / Кабинет Министров СССР; holders from CLAUDE-C01-26) and su_president (Presidency of the USSR) with dated primary attestations from 1 January 1990 to 25 December 1991: Рыжков, Павлов, the 1991 interim arrangements (claims only) and Горбачёв as President, including the Congress election of 14-15 March 1990, the August 1991 events (Янаев's 'acting' claim is claims only) and the 25 December 1991 statement. At most ten people. Keep existing holders unchanged; add dated observations alongside them. Sources: USSR Supreme Soviet and Congress records (Vedomosti, stenograms), Pravda and Izvestia facsimiles, archive scans; non-official hosts disclosed as CLAUDE-C01-26/41 did. Russian as printed.

Keep each distinct event (election or selection, appointment, assumption of office, acting or interim service,
resignation, removal, death, merger, renaming) as its own dated claim; acting service is claims only. Never infer an
end from a successor's start. Give a holder `from` or `until` only where a source states the day; otherwise record
`attested_on`. Party office and state office stay separate both ways. Organization identities, lifecycles and game
mappings stay unresolved; a name match to a simulation row is never a mapping. At most ten people. Primary sources
only; news and encyclopaedias are leads. The historical cutoff stays 7 September 2026.

## Allowed files and checks

Allowed files:

- this record;
- a new `docs/campaign-certification/C01/research/ussr-government-president-attestations-1990-1991-49.md`;
- `docs/campaign-certification/C01/research/ussr.json` and specifically related new
  `docs/campaign-certification/C01/research/sources/ussr-*-facts.json` extracts;
- focused tests under `tools/avatars/`: a new `test_ussr_government_president_c01_49.py`, and pinned counts or exact sets in `test_ussr_research_s10h.py`, `test_ussr_russia_transition_c01_05.py`, `test_ussr_government_supreme_soviet_c01_26.py`, `test_ussr_democratic_russia_soyuz_c01_35.py`, `test_ussr_cpsu_general_secretary_c01_41.py` updated to
  the new totals (none loosened).

Put generated `research-index.json` changes in a separate commit. Do not change the gap ledger, shared UI, the
roadmap, game data or other country packets.

Checks: research-index `--check`; `campaign_census.py --check`; the USSR, research and campaign Python tests
(census included); the atlas Node check; `workboard.py --check`; `git diff --check`.

Mark the packet `ready_for_review` when done. C01 and all parent gates stay open.

## Submission (ready_for_review)

[Report](../../campaign-certification/C01/research/ussr-government-president-attestations-1990-1991-49.md):
`ussr-government-president-attestations-1990-1991-49.md`. Branch `claude/c01-su-49`, claim commit `b2ae720b` on `5ea4f8fc`;
**not stacked**. Result commits: the packet commit `Add CLAUDE-C01-49: USSR heads of government and President: dated
attestations, 1990–1991`, the separate commit `Regenerate the C01 research index for CLAUDE-C01-49` and a final commit recording
the pipeline check in this record, at the head of `claude/c01-su-49` at submission; to be recorded by the integrator. Research
access 1 October 2026 (UTC). One research pass, no sub-agents.

Decisions per observation (institutions `su_presidency` and `su_government`, existing roles `su_president` and
`su_government_head`):

- SU-PRES-01 accepted: the Congress's stenographic report (seventh sitting, 15 March 1990) records the ballot (14 March, from
  'вчера'), the result (1329 for, 495 against), the protocol and resolution, the oath and the presiding officer's declaration that
  Михаил Сергеевич Горбачев assumed the office; holder **from 1990-03-15** (attested_on null), resting on the oath and the declared
  assumption only.
- SU-PRES-02 accepted: signed decrees of 22 January 1991 (Pravda No. 20) and 5 October 1991 (УП-2668, Vedomosti No. 41); holders
  attested_on 1991-01-22 and 1991-10-05.
- SU-PRES-03 accepted in part: the Vice-President's decree of 18 August 1991, the statement of the 'Soviet leadership' and the
  acting styling (Izvestia No. 197) and the Presidium's resolution 2352-I of 21 August are claims only; decree УП-2443 (22 August
  1991) gives a holder attested_on 1991-08-22. No acting holder, break or resumption day.
- SU-PRES-04 not resolved: no Soviet primary text of the 25 December 1991 statement (Pravda No. 302 has only news); no until.
- SU-GOV-11 claims only: the Supreme Soviet's approval of 14 January 1991 as printed in Pravda No. 13, and the note headed
  'Премьер-министр СССР'; no from.
- SU-GOV-12 accepted: Валентин Сергеевич Павлов attested_on 1991-01-22 (the government's resolution of that day, Pravda No. 20);
  the statement of 18 August 1991 listing him as Premier is a claim; an acting Premier after 22 August 1991 remains unsourced.

The existing holders (two CLAUDE-C01-05 presidency observations, three CLAUDE-C01-26 government observations) are unchanged and
keep their indexes; the five new observations follow. No holder has an until. Party and state offices stay separate; nothing is
added to `russia.json`. Two holders; seven people named in rows.

## Decisions for Codex

1. **from 1990-03-15 for the President.** Set from the stenogram's oath and the presiding officer's declaration 'вступил в
   должность', with the law's rule (entry into office at the oath, existing claim `su_first_president_congress_rule_19900314`),
   following the CLAUDE-C01-05 Yeltsin precedent. A ruling could reduce it to attested_on 1990-03-15. This required re-expressing
   three existing guards that assumed the oath was unsourced (S10.h "no holder has a from", C01-26 "no presidency from" and C01-26
   NEVER_BOUNDARY, which contains 15 March 1990 as the election resolution's claim day); each now names exactly this holder.
2. **Pravda (the CPSU's organ, private Internet Archive uploads) as the printed text of state acts.** The 22 January 1991 holders
   of both roles and both SU-GOV-11 claims rest on it. The spec allows Pravda facsimiles; a ruling could make them leads.
3. **The Premier's signature block in Pravda No. 20** prints 'В. ПАВЛОВ / Премьер-министр / М. ШКАБАРДНЯ' (title line between the
   names); read as his signature as Premier.
4. **The approval of 14 January 1991 as a claim, not a holder.** Under the CLAUDE-C01-37 rule an appointing act could give
   attested_on 1991-01-14 (as CLAUDE-C01-26's withdrawn observation did); kept as a claim to follow the role's scope note
   (observations rest on in-office signatures).
5. **The Presidium's resolution 2352-I (21 August 1991)** names 'Президента СССР М. С. Горбачева'; kept as a claim, not an
   attestation of the office on 21 August.
6. **Izvestia No. 197 recorded as the live static file** (no Internet Archive capture exists; Last-Modified 2016).
7. **Name forms.** New observations use 'Михаил Сергеевич Горбачев'; the CLAUDE-C01-05 'Mikhail Gorbachev' observations are not
   reconciled. The existing notes calling Павлов's 19 August observation his "only" one and the government scope note's
   earliest-and-latest wording are left unchanged (no edits to existing records).
8. **Tests outside the listed pins.** `test_russia_presidents_c01_14.py` and `test_russia_heads_of_government_c01_19.py` pinned
   the su_president names as exactly two; both are re-expressed (the two stay first; appended ones rest only on this packet's
   sources). The SSSR.SU host ruling of CLAUDE-C01-26 (C12) applies to four sources.

Touched paths: this record; `docs/campaign-certification/C01/research/ussr.json` (6 sources, 17 claims, five holder observations
appended to the two existing roles after the existing holders, a scope note on `su_president`, three `su_presidency` and two
`su_government` coverage items and one packet coverage item; no existing source, claim, holder, role title or kind, scope note or
lifecycle changed); 6 new `docs/campaign-certification/C01/research/sources/ussr-*-facts.json` extracts (no existing extract
edited); the new `docs/campaign-certification/C01/research/ussr-government-president-attestations-1990-1991-49.md`; the new
`tools/avatars/test_ussr_government_president_c01_49.py`; pinned counts, exact sets and guards re-expressed (none loosened beyond
naming the appended records, which the new test pins) in `tools/avatars/test_ussr_research_s10h.py`,
`tools/avatars/test_ussr_russia_transition_c01_05.py`, `tools/avatars/test_ussr_government_supreme_soviet_c01_26.py`,
`tools/avatars/test_ussr_democratic_russia_soyuz_c01_35.py`, `tools/avatars/test_ussr_cpsu_general_secretary_c01_41.py`,
`tools/avatars/test_russia_presidents_c01_14.py` and `tools/avatars/test_russia_heads_of_government_c01_19.py`; `russia.json`
unchanged. Separate commit: `docs/campaign-certification/C01/research-index.json` only.

Integration notes: this packet runs in parallel with other C01 packets in other country files; `research-index.json` is the only
shared file; regenerate it rather than merging if another packet lands first. The sparse worktree lacked
`docs/campaign-certification/S26/`, so `workboard.py --check` reported a missing handoff; the directory was added to the local
sparse checkout (no file changed) and the check passes.

Checks (1 October 2026 UTC, sparse worktree, on `5ea4f8fc`, which `codex/campaign-certification` still pointed to when fetched
before committing): the research index regenerated and `--check` passes (2,053 sources, 5,027 claims, 844 organization and 36
institution observations, 93 discovery batches); `campaign_census.py --check` passes; the USSR tests (57, 9 of them new) and the
Russia tests (37) pass; the research tests (79) and the campaign tests (16) pass; the atlas Node check passes (11);
`workboard.py --check` passes (44 markers; after adding `docs/campaign-certification/S26/` to the local sparse checkout); `git diff
--check` is clean. The new test's 16 mutations each fail on the rule they break. `packet_check.py 49` (head `558af167`, base `5ea4f8fc`): 18 files changed; 6 new sources, all 6 re-downloaded responses match their recorded byte counts and SHA-256; every check passes (PASS); it lists the re-expressed pinned-test lines for review (Russia C01-19 and C01-14: 1 each; C01-41: 2; C01-35: 2; C01-26: 6; S10.h: 4). A later `packet_check.py 49` run (head `0c2a3ab7`) reported Izvestia No. 197 as 33,680,543 bytes (`2d85c149ae56`); that was a transfer cut off by its 120-second curl limit, since the first 33,680,543 bytes of the recorded body hash to that value. The recorded identity was kept and no packet data changed: a full re-download matched it, and the re-run `packet_check.py 49 --base origin/codex/campaign-certification` (head `0c2a3ab7`, base `5ea4f8fc`) found all 6 re-downloaded responses matching and every check passing (PASS). Known failures outside these checks,
not fixed: `test_certified_gap_ledger.py` reports "no pinned attribution" for this packet's sources until Codex classifies its
commit; `test_certified_boundary_matrix.py` (S23) needs `spheres-web/src`, absent from the sparse checkout, and Codex regenerates
the boundary matrix on integration.

C01 and all parent gates stay open.
