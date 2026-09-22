# CLAUDE-C01-06 — Saudi Arabia's executive chronology 1990-2026

Renumbered from CLAUDE-C01-03, first pushed on `claude/c01-saudi-03`, so it does not collide with
Codex's CLAUDE-C01-03 (Tonga 2024–2025 succession). The current branch is `claude/c01-saudi-06`.

Owner: Claude. State: **ready_for_review** (submitted 21 September 2026; not complete). Parent: C01 (incomplete).
Authority: self-proposed follow-up packet authorized by the user's 21 September 2026 instruction for six further autonomous sessions; pending Codex acceptance.
Related: [CLAUDE-C01-01](CLAUDE-C01-01.md) (integrated and qualified at `fdb6d2c`) and
[CLAUDE-C01-02](CLAUDE-C01-02.md) (Tonga). This packet is independent of the Tonga packets.
Branch `claude/c01-saudi-03`; base `857da24d` (current integration). The sparse worktree
contains `docs/campaign-certification/C01`, `docs/planning`, `tools/avatars`, `tools/planning`
and `tools/ui` only, and it was not widened.
Result commit: the head of `claude/c01-saudi-03` at submission (the separate index commit); to be recorded by the integrator.
Reviewer/integrator: Codex. Separate from the ready S19 gameplay packet.
[Report](../../campaign-certification/C01/research/saudi-executive-chronology-06.md): SA-EXEC-01 to 08
accepted. SA-EXEC-01 is accepted with limits: the 1990 record is foreign, and King Fahd's
death was announced on 1 August 2005 with no stated date of death, so no end date is recorded.
Touched paths: this record; `research/saudi-arabia.json` (22 sources, 33 claims, ten dated
holders on `sa_king` and `sa_crown_prince`, three claim-ID holders for 13 August 2026; the
existing `sa_spa_pm_2022` re-read from a downloaded original, claims unchanged); 23 new
`research/sources/saudi-arabia-*-facts.json` extracts; new
`research/saudi-executive-chronology-06.md`; new `tools/avatars/test_saudi_executive_c01_06.py`.
No existing test pinned Saudi totals, so none changed. Separate commit: `research-index.json` only.

## Bounded deliverable

Review at most eight observations on Saudi Arabia's kings and crown princes from 1990 to the
7 September 2026 cutoff: Fahd, Abdullah and Salman as kings, and Abdullah, Sultan, Nayef,
Salman, Muqrin, Mohammed bin Nayef and Mohammed bin Salman as crown princes. Check them
against the packet's existing 27 September 2022 prime-minister exception. Build from the
completed research dossier and apply every defect fix from its independent checker, or explain
why one does not apply.

Keep each death, family pledge, citizens' pledge (scheduled or held), royal-order selection,
appointment, relief and release filing time as a separate dated claim. Record a term end only
where a source states death while in office, a relief, or the holder's own accession. Never
infer an end from a successor's start. A holder uses `from`/`until` only where a source states
them; otherwise it is event-dated with `attested_on`. News, encyclopedias and non-government
archives are leads in the report, not claims. No observation later than the cutoff; later
access dates are metadata. Reuse existing IDs. Do not create a Deputy Crown Prince or a second
prime-minister role to work around unknown dates.

## Allowed files and handoff

- This record and a new `research/saudi-executive-chronology-06.md` under
  `docs/campaign-certification/C01/`.
- The existing `research/saudi-arabia.json` and specifically related new derived factual
  extracts in `research/sources/`, with source IDs, locators, access dates, provenance and
  uncertainty.
- Focused Saudi validation under `tools/avatars/`. Existing tests change only where they pin
  counts, and are never loosened.

Generated `research-index.json` changes must remain a separate commit. Do not edit other
country packets, READMEs, shared atlas/UI code, the central roadmap, installed character data,
portraits, game schemas or simulation rules.

## Checks and results

```text
python -X utf8 tools/avatars/campaign_research.py            # regenerate: 83 sources, 1,646 claims
python -X utf8 tools/avatars/campaign_research.py --check    # pass
python -X utf8 -m unittest discover -s tools/avatars -p "test_*research*.py"   # 79 pass
python -X utf8 -m unittest discover -s tools/avatars -p "test_tonga_*.py"      # 18 pass
python -X utf8 -m unittest discover -s tools/avatars -p "test_campaign*.py"    # 9 pass, census setUpClass error (sparse; see below)
python -X utf8 -m unittest discover -s tools/avatars -p "test_saudi_executive_c01_06.py"  # 8 pass
node --test tools/ui/check_leadership_research_review.cjs    # 11 pass
python tools/planning/workboard.py --check                   # pass, 44 markers
git diff --check                                             # clean
```

`test_campaign_census.py` needs `spheres-sim/data/party_leaders.json`, which is outside the
sparse checkout. It errors identically at base `857da24d`. It was run in a `git archive` export
of its inputs (the `spheres-sim` and `spheres-web` data, `tools/avatars` and `C01`) at the
result commit, where all 16 campaign tests passed.

Return the branch with base/result commits, source evidence, observation decisions, checker
defects applied or declined, and test results. Marked `ready_for_review`; C01 and all parent
gates stay open.
