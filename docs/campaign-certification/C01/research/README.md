# Source-backed discovery intake

This is additional research for S10.b through S10.e, separate from the C01
recount of the playable catalogue. These packets record discoveries and
explicit gaps; they do not change game parties, leader eligibility or artwork.
The historical cutoff stays **7 September 2026**. Research access dates can be
later, but historical observations must remain inside that boundary.

The generated [research index](../research-index.json) adds a separate set of
numbered discovery batches, each containing at most ten exact identities. All
country censuses and the C01/G2 prerequisites remain open. An empty game mapping
means **unreconciled**, not proof that an organization is absent from the game.

## France: official financial reporting identities

[france.json](france.json) contains 635 unique CNCCFP codes: the 575 filers from
the [official 2024 accounts release](https://www.data.gouv.fr/datasets/comptes-des-partis-et-groupements-politiques)
plus the 60 non-filers in the [official publication table](https://liste.cnccfp.fr/publications/comptes_partis_2024.html).
CNCCFP retains an identifier when a name changes without a change in legal
personality. The [publication notice](https://cnccfp.fr/publication-de-lavis-de-la-cnccfp-sur-les-comptes-des-partis-politiques-exercice-2024/)
reports 635 entities subject to the reporting obligation. All 60 additions also
match the Journal officiel annex by their assigned numbers. This is complete
for that financial reporting universe; it is not an exhaustive census of
French political organizations from 1990 to 2026.

The importer preserves the original names and identifiers. Twelve name
differences between the CSV and publication table remain explicit unresolved
observations under the same assigned code; no name-only identity matching is used.
It does not turn an
accounting year into an active-party interval, assign a leader, merge similarly
named bodies or assume that every reporting identity belongs in the France
campaign. Overseas and local jurisdictions require review. Earlier organizations,
non-reporting groups, coalition membership and independently sourced office
histories remain work to do.

The 298,078-byte [original CSV](sources/cnccfp-2024.csv) was downloaded on
13 September 2026 from the immutable resource URL in the packet. Its SHA-256 is
`6ce50ac58fe95308b21995463a38a2cc83852980fde19cdd110002e8582c49f2`.
Publisher: **CNCCFP**, release dated 10 February 2026; attribution and
[Licence Ouverte](https://www.etalab.gouv.fr/licence-ouverte-open-licence/) metadata
are retained. The publication JSON, page, loader, column instructions and
Journal officiel PDF are also byte-pinned. The importer extracts identities,
reporting status and provenance; the game's economy does not consume these accounts.

## Maintaining a packet

[Tonga](tonga.json) adds three provisional political-organization identities and
five institutions from parliamentary, constitutional and government records.
[Saudi Arabia](saudi-arabia.json) adds six differently classified organizations
and six institutions. The Saudi organizations include a self-declared party, a
historical opposition group and human-rights associations; these are distinct
categories, not six newly verified electoral parties. The 2022 prime-minister
appointment is separated from the King's office and Cabinet-chairing exception.

[Japan](japan.json) adds the 16 party/other-organization lists submitted for the
July 2025 proportional election and seven House parliamentary groups attested
in February 2026. The election and parliamentary universes remain distinct even
when names match. Four party offices carry five dated observations, without
inferred term boundaries or national-office eligibility. Two factual extracts
retain source URLs and downloaded-response hashes. The live upper-house roster
dated 13 September 2026 was excluded because it exceeds the fixed cutoff.

[India](india.json) adds 82 recognition observations from the ECI's
23 March 2024 national- and state-party notifications, officially republished in
[Kerala Gazette No. 1197](https://www.ceo.kerala.gov.in/ceokerala/ceo-cms/uploads/newsupdates/gazette-1197-nationalparties-20240329181824311796.pdf)
and [No. 1198](https://www.ceo.kerala.gov.in/ceokerala/ceo-cms/uploads/newsupdates/gazette-1198-el7-state-parties-20240329182048526666.pdf)
on 28 March 2024. All six national rows and 76 state-jurisdiction rows across
26 jurisdictions were visually reviewed. These are not 82 unique parties or
people. Repeated names across states remain separate observations pending
identity reconciliation; no party, alliance or parliamentary group is merged
by name alone.

The India extracts preserve five frozen-name or pending-court-order
qualifications and Kerala's printed row numbers 1, 2, 3, 4 and 6. They do not
invent a missing row 5 or infer a dispute's resolution. Two factual extracts
retain the source PDF response hashes and page/row locators. Recognition on
23 March 2024 is an attestation, not a founding date, lifespan or current-status
claim; later amendments are not consolidated here. Registered unrecognized
parties, alliances, earlier organizations and separately dated office histories
remain outside this intake. All game mappings, leadership terms and lifecycle
boundaries stay unknown. No avatar or portrait eligibility is added, and access
on 13 September 2026 does not move the historical cutoff.

Together the five packets contain 742 organization observations and 18
institution observations, supported by 1,426 claims across 40 cited sources.
The index assigns them to 79 research batches. None establishes an exhaustive
country roster or supplies a new finished character.

Each source needs a public URL, publisher, access date and individually identified
claims. Each organization or institution cites the exact relevant claims and
records unresolved work. Party, parliamentary and executive roles remain
separate. A holder observation may reference a source claim directly when exact
term boundaries are not established; that observation is not a completed term.

The offline validator checks unique IDs, nation and game-row references, source
and claim ownership, structured historical dates, unresolved coverage, and any
local snapshot's path and checksum. These checks establish record consistency;
they cannot establish that a source is accurate or that a census is exhaustive.
Human/source review remains part of each discovery batch.

From the repository root:

```text
python -X utf8 tools/avatars/import_cnccfp_census.py --check
python -X utf8 tools/avatars/campaign_research.py
python -X utf8 tools/avatars/campaign_research.py --check
python -X utf8 -m unittest discover -s tools/avatars -p "test_campaign*.py"
python -X utf8 -m unittest discover -s tools/avatars -p "test_import_cnccfp_census.py"
python -X utf8 -m unittest discover -s tools/avatars -p "test_japan_research_s10d.py"
python -X utf8 -m unittest discover -s tools/avatars -p "test_india_research_s10e.py"
```

Use the importer without `--check` only to reproduce France's packet from the
existing pinned snapshot. A different release requires an explicit source and
scope update. No tool here accesses the network or writes campaign saves.
