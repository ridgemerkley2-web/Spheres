# CLAUDE-C01-GAPS-01 — certified-country research gap ledger

Owner: Claude; reviewer/integrator: Codex. State: **complete — accepted with repairs**.
Parent C01 remains incomplete. Submitted head `b4d95396`; repair `12ee48f1`;
review `157aac55`. Original branch `claude/c01-gaps-01`, base `76f8ac6c`.
[Independent review and limits](../../campaign-certification/C01/integrations/CLAUDE-C01-GAPS-01/README.md).
This accepts the bounded planning audit, not pending historical research or a complete census.
Follow the [expanded working contract](CLAUDE-EXPANDED-NEXT.md).

## Build

Create an executable, deterministic report joining the existing campaign census,
C01 research packets and their acceptance records for France, Japan, India, Brazil,
South Africa, Tonga, Saudi Arabia and USSR/Russia. Preserve distinct country,
organization, coalition-component, institution and role IDs; executive observations
must not count as party-leader coverage. Separate represented parties, documented
unrepresented/minor/dissolved organizations, accepted claims, pending submissions,
unresearched intervals and institutional exceptions.

Produce a prioritized ledger of the next bounded research batches, at most ten
chains/people each, with source leads and exact missing fields. Do not repeat
existing submissions or accepted intake. Classify each packet only from its
current explicit integration decision; bounded acceptance does not establish
complete history. This is a gap audit of known inputs, not proof that all
real-world organizations have been discovered. Record
the frozen 2026-09-07 cutoff and explicit worldwide discovery limitations.

## Owned paths

New `tools/avatars/certified_gap_ledger.py`, `test_certified_gap_ledger.py`, and
`docs/campaign-certification/C01/gap-ledger/`; this handoff. Read existing
`campaign_census.py`, `campaign_research.py`, research packets and integration records.
Do not change their generators, country packets or generated research index.

## Acceptance

Deliver JSON and readable Markdown generated from pinned inputs, with a `--check`
mode. Test role separation, alias collisions, missing dates, pending versus accepted
evidence, coalition components and deterministic regeneration. Include all eight
cases; disclose uncertain mappings without counting them as covered. Provide a
reproducible next-work list that another researcher can claim without duplicating work.

## Result

Submitted `ready_for_review` on 27 September 2026. Touched paths (nothing else):

- `tools/avatars/certified_gap_ledger.py` (new generator; `--check`; `--refresh-attribution`);
- `tools/avatars/test_certified_gap_ledger.py` (new, 19 tests);
- `docs/campaign-certification/C01/gap-ledger/` (new): `ledger.json`, `ledger.md`, `README.md` and the pinned input
  `inputs/source-attribution.json`;
- this record.

No generator, country packet, research index, census output, production data or shared UI changed.

**Inputs.** `ledger.json` records the path, byte count and SHA-256 of every input: the five census outputs, the
research index, the nine certified research packets, the 27 September integration record, its premerge source
audit and `docs/planning/ai-task-queue.json`, plus the pinned attribution (built at `53246145`). Attribution covers all
1,329 certified sources: 1,300 by the commit that first added their extract, 29 extract-less sources by the first
commit whose packet mentions their id. Commits are classified by hash, because the Saudi packet's first commit still
carries its pre-renumbering label `CLAUDE-C01-03`.

**Ledger (current integration).** 8 cases; 35 represented party rows and 16 components or name phases, of which 50
chains have unresolved days or declared registry gaps; 58 research offices, 49 with unresolved days; 12 numbered next
batches holding 91 items (49 party-leader chains, 41 research office chains, 1 missing institution: the French Prime
Minister, from the census policy record). Batches: France B001-B002, Japan B001-B002, India B001, Brazil B001, South
Africa B001-B002, Tonga B001, Saudi Arabia B001, USSR-Russia B001-B002.

**What it keeps apart.** Party chains use census term records only: executive observations (seed executives,
gameplay grants, research executive offices) never count as party-leader coverage, and people named in both
families are listed as cross-role people. Research organizations are never mapped to party rows; name matches (for
example the IEC ANC row, the LDP, the INC and the PT) are uncertain candidates, not counted, and their role evidence
is folded into the represented row's item. Components never fill a parent chain. Holders carry evidence classes
(`c01_accepted` for C01-01/02/03/04/07/08, `c01_integrated_pending` for C01-05/06/09-22/26, `s10_discovery_intake`,
and `production_registry` for census terms). In-flight C01-23/24/25/27 and the four source repairs come from the task
queue: their targets (the French presidency, the Tongan Speaker, the two Saudi chairs, the BJP chain) are excluded from
new batches and never labelled accepted; a drift between the in-flight table and the queue fails the build.
Applicability windows are pinned only where a checked-in claim anchors them (USSR roles through the recorded
25 December 1991 resignation; the RSFSR and Russian presidencies; South Africa's 1994 offices) and by the census's own
founded/dissolved disclosures; elsewhere the span before a first observation is flagged as of unknown applicability.

**Checks** (27 September 2026, sparse worktree with game data; `PYTHONDONTWRITEBYTECODE=1`):

- `python -X utf8 tools/avatars/certified_gap_ledger.py --check`: passes; regeneration is byte-identical.
- `python -X utf8 -m unittest discover -s tools/avatars -p "test_certified_gap_ledger.py"`: 19 pass (a fixture tree
  covering role separation, alias collisions, missing and imprecise dates, pending versus accepted versus in-flight
  evidence, coalition components, windows and exceptions, bounded batches, deterministic regeneration and stale
  detection, and loud failures; plus checks against the committed ledger). Five deliberate mutations (a candidate
  counted, in-flight not excluded, a collision merged, acting counted as substantive, a component filling its parent)
  each turn the fixture suite red.
- Research index `--check` (1,329 sources, 3,731 claims), `campaign_census.py --check` (exit 0), `test_*research*.py`
  79 pass, `test_campaign*.py` 16 pass (census included), `test_*c01*.py` 207 pass, the atlas Node check 11 pass,
  `workboard.py --check` passes, `git diff --check` clean.

**Limitations.** A gap audit of checked-in inputs, not proof that all real organizations have been discovered; the
cutoff stays 2026-09-07. Coverage statuses describe evidence, not truth: an attested day or an open-ended start never
becomes an interval, and a day with no evidence is not a vacancy. Component and name-phase windows are not bounded in
the inputs, so their unresolved spans may partly fall outside their lifetimes (each such item says so). The RSFSR
vice-presidency has no anchored end in the inputs and is audited to the cutoff. Any new integration, queue change or
packet makes `--check` report the ledger stale until it is regenerated (and the attribution refreshed when sources are
added). C01, G4 and CP1 remain open.
