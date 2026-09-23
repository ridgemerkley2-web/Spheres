# CLAUDE-C01-09: South African heads of state, 1990–2024

Owner: Claude. State: **claimed** (22 September 2026; in progress, not complete). Parent: C01 (incomplete).

Origin: a self-proposed follow-up packet, started on the user's 22 September 2026 instruction to continue
development of Claude's section. The workboard asks for a distinct bounded packet that does not repeat accepted
C01-01/02/03/04/07/08 or reclaim the pending C01-05 (USSR/RSFSR) and C01-06 (Saudi Arabia). This packet touches
only the South Africa research packet, which none of those change. It is pending Codex acceptance and is not
registered in `docs/planning/ai-workstreams.json`.

Branch: `claude/c01-za-09`. Base: `ffe54b02` (current `codex/campaign-certification`). Claim commit: this
record's first commit on the branch.

## Bounded deliverable

Review at most ten observations about the existing `za_presidency` institution between 1 January 1990 and the
14 June 2024 National Assembly election already in the packet (claim `za_ramaphosa_president_elect_20240614`).

1. The head of state when the period opens: the State President under the 1983 Constitution.
2. The 1994 transition: the National Assembly's election of the first President under the 1993 Constitution,
   the inauguration or oath, and the State President's end only if a source states it.
3. Each later presidential election by the National Assembly (1999, 2004, 2009, 2014, 2019) and the
   corresponding oath or inauguration.
4. The 2008 resignation (its announcement and its stated effective time), the election and oath of the
   successor, and the same for the 2018 resignation.
5. The 2024 assumption of office that the existing claim leaves unresolved.

Keep National Assembly election, oath or assumption of office, inauguration ceremony, resignation announcement,
resignation effective time and acting service as distinct dated claims. A party decision (for example a recall)
is not an office event. Never infer an end from a successor's start unless a source states it. Give a holder
`from` only where a source states the day office was assumed or took effect, and `until` only where a source
states the day a resignation or term ended; otherwise record `attested_on`.

Reuse the existing `za_presidency` institution and its `za_president_election` role for the President of the
Republic. Record the pre-1994 State President as its own role in `za_presidency`, because its constitutional
basis differs; never as a holder of `za_president_election`. Do not create a second presidency institution.
Deputy presidents, acting presidents during travel, ministers, and party offices are outside this packet.

Primary sources are required: the Presidency, the South African Government's statement and speech archives
(current and archived official sites), Parliament's releases, Hansard and minutes, the Constitutional Court or
Chief Justice where they record an oath, and the Government Gazette. The texts of the 1983, 1993 and 1996
Constitutions may establish procedure only, never a date. An archival copy of an identified official
document may be used when its limitation is stated. News, encyclopaedias and history sites are leads only.
The historical cutoff stays 7 September 2026.

## Allowed files and checks

Allowed files:

- this record;
- a new `docs/campaign-certification/C01/research/south-africa-heads-of-state-1990-2024-09.md`;
- `docs/campaign-certification/C01/research/south-africa.json` and specifically related new
  `docs/campaign-certification/C01/research/sources/south-africa-*-facts.json` extracts;
- focused tests under `tools/avatars/`: a new `test_south_africa_heads_of_state_c01_09.py`, and pinned counts or
  exact sets in `test_south_africa_research_s10h.py` updated to the new totals (none loosened).

Put generated `research-index.json` changes in a separate commit. Do not change shared UI, the roadmap, game
data or other country packets.

Checks:

- research-index `--check`;
- the South Africa, research and campaign Python tests;
- the atlas Node check;
- `workboard.py --check`;
- `git diff --check`.

Mark the packet `ready_for_review` when done. C01 and all parent gates stay open.
