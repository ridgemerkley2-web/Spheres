# CLAUDE-C01-09: South African heads of state, 1990–2024

Owner: Claude. State: **ready_for_review** (22 September 2026; submitted, not accepted). Parent: C01
(incomplete). Report: [south-africa-heads-of-state-1990-2024-09.md](../../campaign-certification/C01/research/south-africa-heads-of-state-1990-2024-09.md).

Origin: a self-proposed follow-up packet, started on the user's 22 September 2026 instruction to continue
development of Claude's section. The workboard asks for a distinct bounded packet that does not repeat accepted
C01-01/02/03/04/07/08 or reclaim the pending C01-05 (USSR/RSFSR) and C01-06 (Saudi Arabia). This packet touches
only the South Africa research packet, which none of those change. It is pending Codex acceptance and is not
registered in `docs/planning/ai-workstreams.json`.

Branch: `claude/c01-za-09`. Base: `ffe54b02` (current `codex/campaign-certification`). Claim commit: `0e6bbf94`
(this record only).

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

## Result

Submitted `ready_for_review` on 22 September 2026. Touched paths (nothing else):

- `docs/planning/ai-handoffs/CLAUDE-C01-09.md` (this record);
- `docs/campaign-certification/C01/research/south-africa-heads-of-state-1990-2024-09.md` (new report);
- `docs/campaign-certification/C01/research/south-africa.json`;
- 50 new extracts `docs/campaign-certification/C01/research/sources/south-africa-*-facts.json`;
- `tools/avatars/test_south_africa_heads_of_state_c01_09.py` (new) and `tools/avatars/test_south_africa_research_s10h.py`;
- `docs/campaign-certification/C01/research-index.json` (regenerated, separate commit).

The packet gains 50 sources and 78 claims, the role `za_state_president` in `za_presidency` (one holder:
F. W. de Klerk, observed 2 February 1990), and nine `za_president_election` holders around the unchanged
14 June 2024 one. Only stated boundaries are set: Zuma's start on 24 May 2014 ("assumed his second term") and the
ends of Mandela (16 June 1999, his own statement), Mbeki (25 September 2008, the Assembly's resolution) and Zuma
(14 February 2018, President Act No. 24). Oath and swearing-in days date observations and are never starts;
retrospective span lists and directories are claims, never boundaries; acting service and the ANC recalls are
claims or context, never holders.

Observation decisions:

| ID | Decision |
|---|---|
| ZA-HOS-01 | Accepted in part: de Klerk observed 2 Feb 1990; acting from 15 Aug 1989; election, oath and end not found |
| ZA-HOS-02 | Accepted in part: nomination 9 May 1994 at the sitting fixed for the election; oath 10 May 1994; de Klerk's end not stated |
| ZA-HOS-03 | Accepted in part: election 14 Jun and inauguration 16 Jun 1999; Mandela's end 16 Jun 1999; no start for Mbeki |
| ZA-HOS-04 | Accepted: election 23 Apr 2004; oath 27 Apr 2004 |
| ZA-HOS-05 | Accepted in part: resignation effective 25 Sep 2008; Motlanthe elected and sworn in 25 Sep 2008; time and acting service unresolved |
| ZA-HOS-06 | Accepted in part: election 6 May and oath 9 May 2009; Motlanthe's end not set |
| ZA-HOS-07 | Accepted: election 21 May 2014; second term assumed 24 May 2014 |
| ZA-HOS-08 | Accepted: resignation with immediate effect 14 Feb 2018; acting service, election and oath 15 Feb 2018 |
| ZA-HOS-09 | Accepted: election 22 May 2019; oath 25 May 2019 |
| ZA-HOS-10 | Accepted: oath 19 Jun 2024; the existing 14 Jun 2024 election observation unchanged |

All 34 checker defects (A1-A12, B1-B11, C1-C11) are applied, three of them (A4, A5, A6) by not importing the source;
the report's Checker defects table gives each outcome.

Checks run on 22 September 2026 in this sparse worktree (`docs/campaign-certification/C01`, `docs/planning`,
`tools/avatars`, `tools/planning`, `tools/ui`; not widened):

- `python -X utf8 tools/avatars/campaign_research.py` then `--check`: exact regeneration passes; 9 packets,
  841 organization and 27 institution observations, 211 sources, 1,840 claims, 92 open batches.
- South Africa tests (`-p "test_south_africa*.py"`): 18 pass (8 of them new in
  `test_south_africa_heads_of_state_c01_09.py`).
- Research tests (`-p "test_*research*.py"`): 79 pass.
- Campaign tests (`-p "test_campaign*.py"`): the 9 `test_campaign_research` tests pass; `test_campaign_census`
  errors in `setUpClass` because `spheres-sim/data/party_leaders.json` is not in this sparse worktree. It was not
  run, and the sparse checkout was not widened. These suites overlap; their counts are not summed.
- `node --test tools/ui/check_leadership_research_review.cjs`: 11 pass.
- `python tools/planning/workboard.py --check`: passes (44 markers).
- `git diff --check`: clean for this packet's paths.
- The new test rejects hand-made regressions: a successor's start or oath used as an end (Motlanthe, de Klerk,
  Ramaphosa 2018, Mbeki 1999); an election date used as a start or an observation date; an oath used as a start;
  Mandela's end used as Mbeki's start; an announcement, a list date or a receipt used as an end; de Klerk's acting
  date used as a start; the State President added as a holder of `za_president_election` or his claim moved onto
  it; acting service added as a holder or cited by one; an election or ceremony claim cited by a holder; a list row
  given a date; the 2004 election collapsed into the oath; a second presidency institution; the State President
  role removed; and checksum, path, reversed-interval and beyond-cutoff mutations.

C01 and all parent gates (C06, S23, WC1, CP1) stay open. No installed leader, avatar, portrait, campaign rule or
save schema changed.
