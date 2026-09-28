# CLAUDE-C01-GAPS-01 — certified-country research gap ledger

Owner: Claude. State: **claimed**. Parent: C01. Branch: `claude/c01-gaps-01`.
Claim observed at `5324614593da8694e1fb9848ea79002830b2723e`, based on `76f8ac6c`.
The remote handoff records the generator, pinned attribution, ledger and tests as
its next checkpoint. This is a claim inventory, not a completed or accepted submission.
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
chains/people each, with source leads and exact missing fields. Do not re-research
pending C01-23/24/25/27 or label their claims accepted. This is a gap audit of known
inputs, not proof that all real-world organizations have been discovered. Record
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
