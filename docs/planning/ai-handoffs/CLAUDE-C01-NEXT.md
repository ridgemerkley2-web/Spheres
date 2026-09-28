# Claude — next bounded research tasks

Updated 27 September 2026. Fetch the latest `codex/campaign-certification`; runtime
checkpoint `2cb1da4a` follows the original assignment base `474df63e`.
The machine-readable task queue is `docs/planning/ai-task-queue.json`; query it with
`python tools/planning/workboard.py --tasks --owner Claude` or `--task TASK_ID`.
This is a work assignment, not a claim that Claude has begun a repair.

SOURCE-06 was submitted at `93467faaef254a71b4a776d00e56c16dda963eca` on
`claude/c01-source-06` and is **ready for Codex review**, not merged or accepted.
Do not duplicate that submission. SOURCE-05/17/26 remain claimed on their existing
branches; continue those repairs, then the existing C01-23/24/25/27 claims below.

## First: resolve source-review follow-ups

1. **CLAUDE-C01-SOURCE-05:** reproduce the Russian archive response for the 19 June
   1991 CEC resolution, or provide an accessible primary facsimile/archive with a
   content-level comparison. The Codex spot-check timed out; do not infer a wrong
   claim from that timeout. Preserve the unresolved leaf-number discrepancy.
2. **CLAUDE-C01-SOURCE-06:** compare the Bush Presidential Library's 8 August 1990
   address against the recorded extract. Identify dynamic-page changes versus
   changed factual content. Preserve the title-only observation; it does not
   establish Fahd's accession or uninterrupted tenure.
3. **CLAUDE-C01-SOURCE-17:** compare the Brazilian Senate diary download response
   against its recorded extract. Record the exact pages, publication identity and
   factual agreement/disagreement rather than replacing a checksum blindly.
4. **CLAUDE-C01-SOURCE-26:** document provenance and legibility for the Vedomosti
   issue scans permitted by C01-26. Explain why the PDFs qualify as primary
   facsimiles while the same host's HTML transcriptions remain excluded. Codex
   independently decides acceptance; a test allowlist is not historical proof.

The exact URLs, expected/fetched hashes and limitations are preserved in
[source identity evidence](../../campaign-certification/verification/evidence/2026-09-27-claude-integration/premerge-source-identity-sample.json).
The integrated source policy is documented in C01-26's research report.

Use one focused branch per repair. Limit changes to the affected country source
records/extracts, report, relevant tests and repair handoff. Return exact commits,
original-response identity when available, content locators and remaining gaps.
Keep historical dates unchanged unless the cited evidence supports a correction.
Do not convert all 23 missing-response identities into fabricated checksums;
retain disclosed limitations and propose further bounded review separately.

## Then: finish existing claims

| Packet | Existing branch | Bounded scope |
|---|---|---|
| C01-23 | `claude/c01-fr-23` | French presidents, 1990–2026 |
| C01-24 | `claude/c01-to-24` | Tongan Speakers, 1990–2026 |
| C01-25 | `claude/c01-sa-25` | Saudi Shura Council / Allegiance Commission chairs |
| C01-27 | `claude/c01-in-27` | BJP presidents, 1990–2026 |

These were claim-only at the 27 September fetch. Preserve each existing claim;
fetch and merge current integration into its branch before completing it. Do not
restart these as new packet IDs. Follow their existing bounded deliverables.
Regenerate the shared research index separately; run tests with actual game data
available so campaign-census setup is not skipped. Return `ready_for_review` with
exact source commits, tests, uncertainties and any unresolved source access.

## Boundaries

C01-01/02/03/04/07/08 are accepted bounded intake. C01-05/06/09–22/26 are integrated
research with historical acceptance pending; they are not unstarted work. No new
country expansion or cartoon installation is assigned in this queue. C01, G2,
G4 and CP1 remain open. The research cutoff stays 7 September 2026; successors
after that are explicitly fictional. S19 later-outcome qualification belongs to
Codex; Claude supplies targeted fixes only when a reproduced defect is handed off.
