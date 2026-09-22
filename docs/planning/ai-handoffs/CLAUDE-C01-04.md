# CLAUDE-C01-04 — Tonga DPFI/PTOA identity, leadership and the 2025 record

Owner: Claude. State: **ready_for_review** (submitted 21 September 2026; not complete). Parent: C01 (incomplete).
Origin: self-proposed follow-up packet authorized by the user's 21 September 2026 instruction for six further
autonomous sessions; pending Codex acceptance. It is not yet registered in `docs/planning/ai-workstreams.json`.
Predecessor: [CLAUDE-C01-02](CLAUDE-C01-02.md), ready for review and not yet integrated.
Branch `claude/c01-tonga-04`; base `b6767837` (head of `claude/c01-tonga-02`, itself based on integration
`5b46e40a`). **Stacked:** merge CLAUDE-C01-02 first; against `b6767837` this branch contains CLAUDE-C01-04 only.
Result commit: the head of `claude/c01-tonga-04` at submission (the separate index commit); to be recorded by the integrator.
Reviewer/integrator: Codex. Separate from the ready S19 gameplay packet.
[Report](../../campaign-certification/C01/research/tonga-dpfi-04.md): TO-DPFI-01, 02, 05 and 06 accepted;
TO-DPFI-03 (founding), 04 (legal form) and 07 (party offices after September 2019) unresolved with sources
attempted; TO-DPFI-08 (a 2025 result of no seats) not accepted. All fifteen checker defects applied.
Touched paths: this record; `research/tonga.json` (eleven sources, 22 claims, a second `to_dpfi_leader` holder
attested 2014-11-27, new role `to_dpfi_president` with one holder attested 2022-08-29, six name observations);
eleven new `research/sources/tonga-{sc,ca,ipu,assembly,mcctil,tec}-*-facts.json` extracts; new
`research/tonga-dpfi-04.md`; `test_tonga_research_s10g.py` (totals updated) and `test_tonga_reconciliation_c01.py`
(DPFI leader holder pin updated to the exact new list), none loosened; new `test_tonga_dpfi_c01_04.py`.
Separate commit: `research-index.json` only.

## Bounded deliverable

Review at most eight observations about the Democratic Party of the Friendly Islands (`to_dpfi`) and the PTOA
label: primary records connecting PTOA to DPFI, the reported September 2010 founding, the registered or
incorporated form, leadership before and after 'Akilisi Pohiva's death in 2019 through the cutoff, and the
reported 2025 result of no seats. Reuse the existing IDs (`to_dpfi`, `to_dpfi_leader`, the IPU 2010, 2017 and
2021 claims and TO-REC-02, 05 and 08).

Keep selection, endorsement, Cabinet, death, vacancy, win and oath as separate dated claims. Do not infer a
term end from a successor's start, a founding date from a launch report, or a party banner from a personal
result. Holder observations carry `from`/`until` only where a source states them; otherwise null with
`attested_on`. Secondary and tertiary accounts are leads in the report, never claims. Apply every defect in
the independent check, or explain in the report why one does not apply. Preserve the frozen 7 September 2026
cutoff; later access dates are fine, later observations are not.

## Allowed files and handoff

- This record and a new `research/tonga-dpfi-04.md` under `docs/campaign-certification/C01/`.
- The existing `research/tonga.json` and specifically related new factual extracts in `research/sources/`,
  with source IDs, source types, locators, access dates, response identities and uncertainty.
- Focused Tonga validation under `tools/avatars/`, and pinned totals in existing Tonga tests where the new
  records change a count. No assertion may be removed or loosened.

Generated `research-index.json` changes must remain a separate commit. Do not edit other country packets,
shared atlas/UI code, the central roadmap, `ai-workstreams.json`, installed character data, portraits, game
schemas or simulation rules. Do not create a second DPFI organization or a second prime-minister office.

Run the research-index regeneration and check, all Tonga, research and campaign tests, the new test, the atlas
Node checks, the workboard check and `git diff --check`. Return the branch with exact base and result commits,
source evidence, resolved and open observation IDs and test results. Mark `ready_for_review`; C01 and all
parent gates stay open.
