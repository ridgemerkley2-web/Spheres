# CLAUDE-C01-07 — Tonga Crown and reign chronology, 1990-2026

Owner: Claude. State: **ready_for_review** (submitted 21 September 2026; not complete). Parent: C01 (incomplete).
Origin: self-proposed follow-up packet authorized by the user's 21 September 2026 instruction for six further
autonomous sessions; pending Codex acceptance. It is not yet registered in `docs/planning/ai-workstreams.json`.
Predecessor: [CLAUDE-C01-04](CLAUDE-C01-04.md), ready for review and not yet integrated.
Branch `claude/c01-tonga-07`; base `62690e61` (head of `claude/c01-tonga-04`, which has already merged current
integration `04bc99a6`). **Stacked: merge CLAUDE-C01-04 first**; against it, this branch contains CLAUDE-C01-07 only.
Result commit: the head of `claude/c01-tonga-07` at submission (the separate index commit); to be recorded by the
integrator. Reviewer/integrator: Codex. Separate from the ready S19 gameplay packet.
[Report](../../campaign-certification/C01/research/tonga-crown-07.md): TO-CROWN-01 to TO-CROWN-08 accepted;
TO-CROWN-02 (regencies) only as dated observations, with instruments, starts and ends unresolved; the two
coronation oaths and a ceremony record for the 2015 coronation unresolved. All eleven checker defects applied,
including the major one (Tupou IV's death is 00:34 on 11 September 2006, Tongan time, per Gazette No. 20).
Touched paths: this record; `research/tonga.json` (27 sources, 44 claims of which seven are on reused sources,
three `to_king` holders — Taufa'ahau Tupou IV until 2006-09-11, George Tupou V 2006-09-11 to 2012-03-18, Tupou VI
from 2012-03-18 — a `to_king` scope note, and `to_crown` and packet coverage notes); 27 new
`research/sources/tonga-{act,tlr,pmo,gazette,nrbt,un,iha}-*-facts.json` extracts; new
`research/tonga-crown-07.md`; `test_tonga_research_s10g.py` (totals updated) and `test_tonga_dpfi_c01_04.py`
(stated `from`/`until` pins updated to the exact new sets), none loosened; new `test_tonga_crown_c01_07.py`.
Separate commit: `research-index.json` only.

## Bounded deliverable

Review at most eight observations about the Crown of Tonga (`to_crown`, role `to_king`) from 1990 to the
cutoff: the holder when the period opens, any formally recorded Prince or Princess Regent arrangement before
Taufa'ahau Tupou IV's death, and the accession, proclamation, traditional installation, coronation and death of
each monarch. Reuse the existing IDs (`to_crown`, `to_king`, `to_tupou_vi_2025`, `to_constitution_2020`,
`to_constitution_older`, `to_ipu_2008`, `to_ipu_2010`) and the existing prime-minister and Cabinet roles.

Keep accession, proclamation, traditional installation, coronation, coronation oath, death, funeral and mourning
as separate dated claims. Accession by devolution of the Crown on a monarch's death is a stated start; a
proclamation, installation or coronation is not. Never infer a term end from a successor's start unless a source
states it. Regencies are not reigns. Holder observations carry `from`/`until` only where a source states them.
Secondary and tertiary accounts are leads in the report, never claims. Apply every defect in the independent
check, or explain in the report why one does not apply. Preserve the frozen 7 September 2026 cutoff; later
access dates are fine, later observations are not.

## Allowed files and handoff

- This record and a new `research/tonga-crown-07.md` under `docs/campaign-certification/C01/`.
- The existing `research/tonga.json` and specifically related new factual extracts in `research/sources/`,
  with source IDs, source types, locators, access dates, response identities and uncertainty.
- Focused Tonga validation under `tools/avatars/`, and pinned totals in existing Tonga tests where the new
  records change a count. No assertion may be removed or loosened.

Generated `research-index.json` changes must remain a separate commit. Do not edit other country packets,
shared atlas/UI code, the central roadmap, `ai-workstreams.json`, `roles-and-lifecycle.json`, installed character
data, portraits, game schemas or simulation rules. Do not create a second Crown institution or a regent role to
work around unknown dates.

## Result

Checks run on 21 September 2026 at the submission head:

- `python -X utf8 tools/avatars/campaign_research.py` then `--check`: exact regeneration passes; 9 packets,
  841 organization and 27 institution observations, 106 sources, 1,688 claims, 92 open batches.
- Tonga tests (`-p "test_tonga_*.py"`): 49 pass (11 of them new in `test_tonga_crown_c01_07.py`).
- Research tests (`-p "test_*research*.py"`): 79 pass. Campaign tests (`-p "test_campaign*.py"`): 16 pass,
  `test_campaign_census` included (this worktree is not sparse). These suites overlap; their counts are not summed.
- `node --test tools/ui/check_leadership_research_review.cjs`: 11 pass.
- `python tools/planning/workboard.py --check`: passes (44 markers).
- `git diff --check`: clean for this packet's paths.
- The new test was also run against hand-made regressions (a coronation or installation used as the start, a
  proclamation used as the start, the New Zealand date used as the end, an inferred end for Tupou VI, a regent
  added as a reign or a role, a death collapsed into its proclamation, a relative date converted, the pleading
  given a month); each is rejected.

C01 and all parent gates (C06, S23, WC1, CP1) stay open. No installed leader, avatar, portrait, campaign rule or
save schema changed.
