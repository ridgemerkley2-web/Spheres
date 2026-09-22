# CLAUDE-C01-08: Tonga prime ministers, 1990–2019

Owner: Claude. State: **ready_for_review** (submitted 22 September 2026; not complete). Parent: C01 (incomplete).

Origin: this is a self-proposed follow-up packet, started on the user's 22 September 2026 instruction to
continue Claude's section. It is pending Codex acceptance and is not yet registered in
`docs/planning/ai-workstreams.json`.

Branch: `claude/c01-tonga-08`. Base: `e6f9fa41`, the head of `claude/c01-tonga-07`, which itself sits on
CLAUDE-C01-04. Both of those packets are ready for review and not yet integrated. Claim commit: `dac0cc35`.

CLAUDE-C01-04 and CLAUDE-C01-07 are now accepted and integrated, and this branch has merged integration
`9d352f03`. It has also merged CLAUDE-C01-03 (`claude/c01-tonga-03` at `387e4526`), resolving the shared
`to_pm` holder list (seven 1990-2018 holders, the 2019 and 2021 ones, Sovaleni's stated end, Eke) and the
exact test pins. **Stacked: merge CLAUDE-C01-03 first**; against it, this branch contains CLAUDE-C01-08 only. The earlier conflict note no longer applies: the two packets are merged here.

Result commit: the head of `claude/c01-tonga-08` at submission (the separate index commit); to be recorded by
the integrator. Reviewer and integrator: Codex.

[Report](../../campaign-certification/C01/research/tonga-prime-ministers-1990-2019-08.md): TO-PM90-03, 05, 06
and 07 accepted; TO-PM90-01 accepted at year precision; TO-PM90-04 and 08 accepted in part (the 2006
substantive appointment day and the day of Pohiva's death unresolved); TO-PM90-02 (the 1991 succession date)
unresolved. All thirteen checker defects applied, including the addition of the MEIDECC release of
7 September 2017 that the checker located.

Touched paths:

- this record;
- `docs/campaign-certification/C01/research/tonga.json`: 39 sources, 47 claims, seven `to_pm` holders —
  Fatafehi Tu'ipelehake (`attested_period` 1990) and Baron Vaea (`attested_period` 1992-1998) at year
  precision; Prince 'Ulukalala Lavaka Ata from 2000-01-03 until 2006-02-11 (stated commencement; accepted
  resignation); Feleti Sevele (attested 2006-04-07); Lord Tu'ivakano (attested 2010-12-22); Samuela 'Akilisi
  Pohiva (attested 2014-12-30; and from 2018-01-02, no end) — a `to_pm` scope note, and `to_prime_minister`
  and packet coverage notes; acting service is recorded as claims only;
- 39 new `docs/campaign-certification/C01/research/sources/tonga-{pmo,mic,ipu,palace,assembly,gazette}-*-facts.json`
  extracts;
- new `docs/campaign-certification/C01/research/tonga-prime-ministers-1990-2019-08.md`;
- new `tools/avatars/test_tonga_pm_1990_2019_c01_08.py`;
- pinned counts and exact holder sets updated, none loosened and no assertion removed:
  `tools/avatars/test_tonga_research_s10g.py`, `tools/avatars/test_tonga_transition_c01_02.py`,
  `tools/avatars/test_tonga_dpfi_c01_04.py` and `tools/avatars/test_tonga_crown_c01_07.py` (the report's
  integration notes give each change);
- separate commit: `docs/campaign-certification/C01/research-index.json` only.

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

## Result

Checks run on 22 September 2026 in this sparse worktree (`docs/campaign-certification/C01`, `docs/planning`,
`tools/avatars`, `tools/planning`, `tools/ui`; not widened):

- `python -X utf8 tools/avatars/campaign_research.py` then `--check`: exact regeneration passes; 9 packets,
  841 organization and 27 institution observations, 145 sources, 1,735 claims, 92 open batches.
- Tonga tests (`-p "test_tonga_*.py"`): 58 pass (9 of them new in `test_tonga_pm_1990_2019_c01_08.py`).
- Research tests (`-p "test_*research*.py"`): 79 pass.
- Campaign tests (`-p "test_campaign*.py"`): the 9 `test_campaign_research` tests pass; `test_campaign_census`
  errors in `setUpClass` because `spheres-sim/data/party_leaders.json` is not in this sparse worktree. It was not
  run, and the sparse checkout was not widened. These suites overlap; their counts are not summed.
- `node --test tools/ui/check_leadership_research_review.cjs`: 11 pass.
- `python tools/planning/workboard.py --check`: passes (44 markers).
- `git diff --check`: clean for this packet's paths.
- The new test rejects hand-made regressions: a selection, oath or release date used as the 2018 start;
  Vaea's end inferred from Lavaka Ata's start; the 2014 appointment ended by the 2018 re-appointment; a death
  day, a leave audience or the retrospective timeline date used for a holder; a year range turned into an
  interval; an acting Prime Minister added as a holder or cited by one; a selection cited by a holder; the 2010
  ballot collapsed into the appointment; the IPU 1990 sentence given a date; the funeral programme turned into
  a death window; a second prime-minister role; and checksum, path and beyond-cutoff mutations.

C01 and all parent gates (C06, S23, WC1, CP1) stay open. No installed leader, avatar, portrait, campaign rule or
save schema changed.
