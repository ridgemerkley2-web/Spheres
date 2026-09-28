# Certified-country research gap ledger

CLAUDE-C01-GAPS-01. A deterministic gap audit that joins the C01 campaign census, the C01 research packets and
their acceptance/integration records for the eight certified cases (France, Japan, India, Brazil, South Africa,
Tonga, Saudi Arabia and USSR → Russia), and lists numbered next research batches of at most ten chains each.

| File | What it is |
|---|---|
| `ledger.json` | The machine-readable ledger (generated). |
| `ledger.md` | The readable ledger and next-batch list (generated). |
| `inputs/source-attribution.json` | Pinned input: each certified research source attributed to the packet that added it. |

```bash
python -X utf8 tools/avatars/certified_gap_ledger.py            # regenerate ledger.json and ledger.md
python -X utf8 tools/avatars/certified_gap_ledger.py --check    # fail if either is stale
python -X utf8 tools/avatars/certified_gap_ledger.py --refresh-attribution   # rebuild the pinned attribution (needs git history)
python -X utf8 -m unittest discover -s tools/avatars -p "test_certified_gap_ledger.py"
```

## What it separates

- **Represented parties** (simulation rows) and their **coalition components or name phases**, each with its own
  party-leader chain from the census term records only. A component never fills its parent's chain.
- **Documented research organizations** (registers, filings, lists): counted and linked to the existing discovery
  work orders, never mapped to a party row. A name match is an *uncertain candidate* and is not counted.
- **Research offices** (institution and organization roles) with every holder's evidence class.
- **Executive observations** (census seed executives, executive gameplay grants): listed apart; they never count as
  party-leader coverage, and research executive offices never count toward a party chain.
- **Evidence classes**: `production_registry`, `s10_discovery_intake`, `c01_accepted` (CLAUDE-C01-01/02/03/04/07/08)
  and `c01_integrated_pending` (C01-05/06/09–22/26). Claimed or submitted packets that are not integrated appear
  only as **in-flight work**, taken from `docs/planning/ai-task-queue.json`; their targets are excluded from new
  batches and never labelled accepted.
- **Institutional exceptions**: countries with no party rows, hereditary offices, collective seats, institutions
  observed without office roles, census institution-policy facts, editorial continuation disclosures, and
  applicability windows (pinned only where a checked-in claim anchors the boundary; see `APPLICABILITY`).
- **Alias collisions**: distinct IDs sharing a normalized name, reported and never merged.

## Coverage statuses

Each chain is audited day by day over its window. `definite` requires both ends stated (trimmed to the certain part
of a month- or year-precision boundary); `acting_definite` is kept apart; `boundary_imprecise`, `open_end` (a stated
start with no stated end), `attested_day`, `attested_within_period` and `no_evidence` are unresolved. An isolated
attestation or an open-ended start never becomes an interval, and a day with no evidence is not evidence of a vacancy.

## Attribution

Each source is attributed to the commit that first added its extract, or, for the 29 extract-less sources, the first
commit whose packet file mentions its id. Commits are classified by hash in `COMMIT_PACKETS` (not by message: the
Saudi packet's first commit still carries its pre-renumbering label). An unattributed source, an unclassified
commit or a drift between the in-flight table and the task queue fails the build.

Only the task-queue rows the ledger reads (Claude's C01 packet and source-repair tasks: id, state and branch) are
recorded and hashed as its queue input, so unrelated queue changes, such as another session's closure, do not make
the ledger stale.

An accepted packet requires an explicit accepted decision in its integration `README.md`; creating a directory
does not accept research. Those decision records are hashed among the ledger's inputs, so a review scope change
makes the generated ledger stale even when packet contents and evidence-class totals are unchanged.
Markdown input byte counts and hashes explicitly use UTF-8 text with LF line endings, independent of Git checkout
conversion on Windows. Other input hashes remain over the exact file bytes.

## Limitations

This audits checked-in inputs; it does not show that every real organization, office or holder has been found.
The research cutoff is frozen at 2026-09-07. Passing its tests does not mean coverage passes: every unresolved span
in `ledger.md` is open work. It changes no generator, packet, research index, census output or production data.
