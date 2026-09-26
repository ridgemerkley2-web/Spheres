# CLAUDE-C01-26: Soviet heads of government and Supreme Soviet chairs, 1990–1991

Owner: Claude. State: **claimed** (25 September 2026; in progress, not complete). Parent: C01 (incomplete).

Origin: a self-proposed follow-up packet, started on the user's 25 September 2026 instruction to start another
batch of five packets in parallel. It does not repeat accepted C01-01/02/03/04/07/08 or reclaim
the pending C01-05, C01-06 and C01-09 to C01-22. It is pending Codex acceptance and is not registered in `docs/planning/ai-workstreams.json`.

Branch: `claude/c01-su-26`. **Stacked on CLAUDE-C01-19** (`claude/c01-ru-19` at `6f4ef2c2`), which is
ready for review and not yet integrated, because both packets edit `ussr.json`: merge CLAUDE-C01-19 first. Claim
commit: this record's first commit on the branch.

## Bounded deliverable

Add one executive institution, `su_government` (the Council of Ministers of the USSR and, from 1991, the Cabinet of Ministers, kind `executive_institution`) with one role, `su_government_head` (head of the Union government, kind `head_of_government`), and extend the existing `su_supreme_soviet_chair` role. Review at most ten observations between 1 January 1990 and 26 December 1991:

1. The Chairman of the Council of Ministers when the period opens (Nikolai Ryzhkov).
2. March 1990: the election of a new Chairman of the Supreme Soviet after the Chairman became President.
3. The 1990–1991 constitutional change creating the Cabinet of Ministers (procedure only).
4. Ryzhkov's end, only as a source states it.
5. January 1991: Valentin Pavlov's appointment as Prime Minister.
6. August 1991: Pavlov's dismissal.
7. August–December 1991: the committee that took over the government's functions and its chairman.
8. The Chairman of the Supreme Soviet's end in 1991, only as a source states it.
9. The September 1991 reorganization of the Supreme Soviet and its later chairs.
10. The last records of both offices in December 1991.

Keep the Supreme Soviet's election or consent, the appointment decree, dismissal, resignation, arrest and acting service as distinct dated claims; acting and interim service are claims only. The C01-05 USSR presidency holders and all Russia holders (C01-05, C01-14, C01-19) do not change. Never infer an end from a successor's start unless a source states it. Give a holder `from` only where a source states the day office was assumed or took effect, and `until` only where a source states the day the office ended; otherwise record `attested_on`. Constitution and statute texts may establish procedure only, never a date. Retrospective lists are claims, never boundaries. News, encyclopaedias and history sites are leads only. The historical cutoff stays 7 September 2026.

Primary sources are required: the records of the USSR Congress of People's Deputies and Supreme Soviet (Vedomosti, stenograms), decrees of the President of the USSR, Izvestia and Pravda only as official publication of an act, and archival official records.

## Allowed files and checks

Allowed files:

- this record;
- a new `docs/campaign-certification/C01/research/ussr-government-and-supreme-soviet-1990-1991-26.md`;
- `docs/campaign-certification/C01/research/ussr.json` and specifically related new
  `docs/campaign-certification/C01/research/sources/ussr-*-facts.json` extracts;
- focused tests under `tools/avatars/`: a new `test_ussr_government_supreme_soviet_c01_26.py`, and pinned counts or exact sets in `test_ussr_research_s10h.py` and `test_ussr_russia_transition_c01_05.py`
  updated to the new totals (none loosened).

Put generated `research-index.json` changes in a separate commit. Do not change shared UI, the roadmap, game
data or other country packets.

Checks: research-index `--check`; the USSR, research and campaign Python tests; the atlas Node check;
`workboard.py --check`; `git diff --check`.

Mark the packet `ready_for_review` when done. C01 and all parent gates stay open.
