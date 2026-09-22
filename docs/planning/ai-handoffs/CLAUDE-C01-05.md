# CLAUDE-C01-05 — The 1991 USSR/RSFSR executive transition

Owner: Claude. State: **ready_for_review** (submitted 21 September 2026; not complete). Parent: C01 (incomplete).
Origin: self-proposed follow-up packet authorized by the user's 21 September 2026 instruction for six further
autonomous sessions; pending Codex acceptance. It is not yet registered in `docs/planning/ai-workstreams.json`.
Predecessors: [CLAUDE-C01-01](CLAUDE-C01-01.md) and [CLAUDE-C01-02](CLAUDE-C01-02.md), accepted as bounded research.
Independent of the Tonga (CLAUDE-C01-04) and Saudi packets; it touches only the USSR and Russia packets.
Branch `claude/c01-ussr-05`; base `04bc99a6` (current integration). No separate claim commit.
Result commit: the head of `claude/c01-ussr-05` at submission (the separate index commit); to be recorded by the integrator.
Reviewer/integrator: Codex. Separate from the ready S19 gameplay packet.
[Report](../../campaign-certification/C01/research/ussr-russia-transition-1991-05.md): SURU-TR91-01 to 05 accepted;
SURU-TR91-06 accepted for the day of Gorbachev's announced cessation only (no `until`); SURU-TR91-07 (end of the
USSR Presidency as an institution) and 08 (Council of Republics, 26 December) unresolved, with sources attempted.
All thirteen checker defects applied (D5 in a stricter form).
Touched paths: this record; `research/russia.json` (thirteen sources, 24 claims, new institution
`ru_rsfsr_presidency` with roles `ru_rsfsr_president` and `ru_rsfsr_vice_president`, holders Yeltsin and Rutskoi
from 1991-07-10 with no end, `lifecycle.from` null); `research/ussr.json` (seven sources, 13 claims, a second
`su_president` holder attested 1991-12-25 with no `until`, coverage notes on `su_presidency` and `su_supreme_soviet`;
`lifecycle.until` stays null); twenty new `research/sources/{russia,ussr}-*-1991*-facts.json` extracts; new
`research/ussr-russia-transition-1991-05.md`; `test_ussr_research_s10h.py` and `test_russia_research_s10h.py`
(totals and exact sets updated, faction loops scoped to the five factions), none loosened; new
`test_ussr_russia_transition_c01_05.py`. Separate commit: `research-index.json` only.

## Bounded deliverable

Review at most eight observations about the 1991 executive transition: the creation of the RSFSR presidency,
its first election and Yeltsin's oath; the Belovezha (8 December) and Alma-Ata (21 December) records; Gorbachev's
cessation as USSR President; and the end of the USSR Presidency. Reuse the existing USSR IDs (`su_presidency`,
`su_president`, `su_supreme_soviet`, `su_congress_peoples_deputies`). Add a Russian presidency institution only
if `russia.json` lacks one and primary sources support it. Never map a USSR institution to Russia automatically;
membership continuity, recognition or a transfer of authority is not institutional succession.

Keep adoption, entry into force, approval, election, result approval, oath, signature, ratification, renaming
and cessation as separate dated claims. In particular, Law 1098-I's adoption (24 April 1991) is not its entry
into force (from publication). Holder observations carry `from`/`until` only where a source states them;
otherwise null with `attested_on`. Secondary editions and non-official transcriptions (the BSB edition of the
25 December address, the vedomosti.sssr.su transcription) are leads, never claims. Apply every defect in the
independent check, or explain in the report why one does not apply. Preserve the frozen 7 September 2026
cutoff; later access dates are fine, later observations are not.

## Allowed files and handoff

- This record and a new `research/ussr-russia-transition-1991-05.md` under `docs/campaign-certification/C01/`.
- The existing `research/ussr.json` and `research/russia.json`, and specifically related new factual extracts in
  `research/sources/`, with source IDs, source types, locators, access dates, response identities and uncertainty.
- Focused validation under `tools/avatars/`, and pinned totals in existing USSR and Russia tests where the new
  records change a count or an exact set. No assertion may be removed or loosened.

Generated `research-index.json` changes must remain a separate commit. Do not edit other country packets,
shared atlas/UI code, the central roadmap, `ai-workstreams.json`, installed character data, portraits, game
schemas or simulation rules. Do not widen the sparse worktree.

Run the research-index regeneration and check, the research, USSR and Russia tests, the new test, the atlas
Node checks, the workboard check and `git diff --check`. `test_campaign_census.py` needs `spheres-sim/data`,
which the sparse worktree lacks; it is noted, not run. Return the branch with exact base and result commits,
source evidence, resolved and open observation IDs and test results. Mark `ready_for_review`; C01 and all
parent gates stay open.
