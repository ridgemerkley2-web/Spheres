# Source-backed discovery intake

This is additional research for S10.b. The earlier C01 inventory remains a
frozen recount of the playable catalogue. These packets record discoveries and
explicit gaps; they do not change game parties, leader eligibility or artwork.
The historical cutoff stays **7 September 2026**. Research access dates can be
later, but historical observations must remain inside that boundary.

The generated [research index](../research-index.json) adds a separate set of
numbered discovery batches, each containing at most ten exact identities. All
country censuses and the C01/G2 prerequisites remain open. An empty game mapping
means **unreconciled**, not proof that an organization is absent from the game.

## France: official financial reporting identities

[france.json](france.json) contains 575 unique CNCCFP codes and their published
names from the [official 2024 accounts release](https://www.data.gouv.fr/datasets/comptes-des-partis-et-groupements-politiques).
CNCCFP retains an identifier when a name changes without a change in legal
personality. The [publication notice](https://cnccfp.fr/publication-de-lavis-de-la-cnccfp-sur-les-comptes-des-partis-politiques-exercice-2024/)
reports 635 entities subject to the reporting obligation; this CSV does not
enumerate the other 60. Neither reporting universe is an exhaustive census of
French political organizations from 1990 to 2026.

The importer preserves the original names and identifiers. It does not turn an
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
are retained. The importer reads only identity/name/year columns; the game's
economy does not consume these financial accounts.

## Maintaining a packet

[Tonga](tonga.json) adds three provisional political-organization identities and
five institutions from parliamentary, constitutional and government records.
[Saudi Arabia](saudi-arabia.json) adds six differently classified organizations
and six institutions. The Saudi organizations include a self-declared party, a
historical opposition group and human-rights associations; these are distinct
categories, not six newly verified electoral parties. The 2022 prime-minister
appointment is separated from the King's office and Cabinet-chairing exception.

Together the three packets contain 584 organization observations and 11
institution observations, supported by 615 claims across 26 cited sources.
The index assigns them to 61 research batches. None establishes an exhaustive
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
```

Use the importer without `--check` only to reproduce France's packet from the
existing pinned snapshot. A different release requires an explicit source and
scope update. No tool here accesses the network or writes campaign saves.
