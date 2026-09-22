# CLAUDE-C01-08: Tonga prime ministers, 1990–2019

Owner: Claude. State: **claimed 22 September 2026; in progress** (not ready for review). Parent: C01 (incomplete).

Origin: this is a self-proposed follow-up packet, started on the user's 22 September 2026 instruction to
continue Claude's section. It is pending Codex acceptance and is not yet registered in
`docs/planning/ai-workstreams.json`.

Branch: `claude/c01-tonga-08`. Base: `e6f9fa41`, the head of `claude/c01-tonga-07`, which itself sits on
CLAUDE-C01-04. Both of those packets are ready for review and not yet integrated.

**Stacked: merge CLAUDE-C01-04 and CLAUDE-C01-07 first.** Measured against them, this branch contains
CLAUDE-C01-08 only. `tonga.json` may conflict with CLAUDE-C01-03 (`claude/c01-tonga-03`) in the
`to_pm` holder list; Claude will rebase on request after either one merges.

Result commit: recorded at submission. Reviewer and integrator: Codex.

## Bounded deliverable

Review at most eight observations about the existing `to_prime_minister` institution (role `to_pm`)
between 1 January 1990 and the 2019 appointment already in the packet. Priorities:

1. The holder when the period opens.
2. Each later holder's royal appointment, acting or interim service, resignation or death.
3. For the post-2010 procedure: the separate Assembly selection, royal appointment and effective date
   (2010, 2014 and 2017).

Keep selection, appointment, effectiveness, acting service, resignation and death as distinct dated
claims. Never infer an end from a successor's start unless a source states it. Reuse existing IDs and
roles, and do not create a second prime-minister institution.

Primary sources are required: Prime Minister's Office, Legislative Assembly minutes or notices, Palace,
Gazette and court records. IPU Parline may corroborate. News is used only as leads. The historical
cutoff stays 7 September 2026.

## Allowed files and checks

Allowed files:

- this record;
- a new `docs/campaign-certification/C01/research/tonga-prime-ministers-1990-2019-08.md`;
- `research/tonga.json` and specifically related new `research/sources/` extracts;
- focused tests under `tools/avatars/`.

Put generated `research-index.json` changes in a separate commit. Do not change shared UI, the roadmap,
game data or other country packets.

Checks:

- research-index `--check`;
- the Tonga, research and campaign Python tests;
- the atlas Node check;
- `workboard.py --check`;
- `git diff --check`.

Mark the packet `ready_for_review`. C01 and all parent gates stay open.
