# Source-backed discovery intake

This is additional research begun in S10.b through S10.h and continued in bounded
Claude/Codex handoffs, separate from the C01
recount of the playable catalogue. These packets record discoveries and
explicit gaps; they do not change game parties, leader eligibility or artwork.
The historical cutoff stays **7 September 2026**. Research access dates can be
later, but historical observations must remain inside that boundary.

The generated [research index](../research-index.json) adds a separate set of
numbered discovery batches, each containing at most ten exact identities. All
country censuses and the C01/G2 prerequisites remain open. An empty game mapping
means **unreconciled**, not proof that an organization is absent from the game.

The [political research atlas](../../../../tools/ui/leadership-research-review.html)
provides country selection, text search and organization, institution and office
filters. Expand an observation to review its dated claims, source provenance and
open questions. All 160 identities remain selectable; the 151 without a new
packet show missing research explicitly. This reference neither loads a campaign
nor grants a character an office.

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

[Tonga](tonga.json) now contains four provisional political-organization
identities and five institutions. S10.g adds three primary sources and five
claims: the [dialogue organizer's account](https://www.idcpc.org.cn/english2023/bzhd/202105/t20210531_160418.html)
identifies Pohiva Tu'i'onetoa as Leader of Tonga People's Party on 28 May 2021;
the Court of Appeal describes People's Party campaigning at a 1 October 2021
event. At that increment the court's PATOA/PTOA spellings remained explicit and unreconciled. These
observations establish neither a party's founding date nor a continuous
leadership term, and do not merge it with the People's Democratic Party.

The PMO appointment release separately records the Assembly's recommendation
on 27 September 2019 and the King's appointment on 8 October. Neither is a
party-leadership election. Court PDF pages 1, 8, 9, 12 and 15 and PMO PDF page 1
were rendered and visually reviewed. Three derived factual extracts retain
hashes and byte counts of the actual downloaded responses; original bodies
are not checked in. The extracts have their own checked-in checksums. No source
artwork is reused. Access on 13 September 2026 leaves the fixed historical
cutoff unchanged; party histories, game mappings and Tonga's discovery batch
remain open.

The [first Tonga reconciliation](tonga-reconciliation-01.md) is now accepted through
the [Codex integration review](../integrations/CLAUDE-C01-01/README.md). It adds three
sources and seven claims while keeping the four organizations and five institutions.
The printed PATOA/PTOA variants are reconciled provisionally; society offices remain
separate from party leadership, and Assembly selection remains separate from royal
appointment. No full term, installed leader, avatar or country completion is added.

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

[Brazil](brazil.json) adds 31 organization observations, not 31 distinct parties:
29 exact labels from the [TSE 2024 FEFC table](https://www.tse.jus.br/eleicoes/eleicoes-2024-content/prestacao-de-contas/fundo-especial-de-financiamento-de-campanha-fefc),
one later Missão registration decision and one separate PMB/Democrata naming
record. The complete HTML funding table was read. It reports 27 release dates
in 2024; PCB and PMB show dashes. Those two dates stay unknown, with no inferred
refusal, nonpayment or inactivity. Labels and process-reference punctuation are
retained rather than expanded into guessed legal identities. The June 2024
announcement's aggregate total does not backdate the later release rows.

The naming record preserves the TSE article's Democrata and the initial official
notice's O DEMOCRATA, followed by the [TRE-RJ notice](https://www.tre-rj.jus.br/servicos-judiciais/comunicados)
reporting a 12 February 2026 rectification to DEMOCRATA. These are decision
observations, not automatic name intervals or a merged game identity. The
underlying decisions, statutory compliance and subsequent status remain to be
reviewed. Missão's registration and number 14 do not grant an earlier party's
identity or establish a leader.

Five primary sources support 34 claims. Five derived factual extracts have
checked-in checksums, while original HTTP-response byte counts and hashes are
explicitly unknown. Direct downloads were unavailable; no image or linked PDF
was visually reviewed. Official HTML and indexed article text provide the
stated observations, not an archived registry as of the cutoff. The live
leadership registry was not used to backdate officeholders. Access on
13 September 2026 does not extend the fixed historical cutoff. All game
mappings, leadership roles, lifecycle boundaries and portrait eligibility stay
unknown; Brazil's four discovery batches remain open.

[USSR](ussr.json) adds one CPSU observation and three separate state institutions:
the USSR Presidency, Congress of People's Deputies and Supreme Soviet. Three
sources support eight claims. The 14 March 1990 law creates the Presidency and
changes Article 6; its enactment and Gorbachev's Supreme Soviet chair signature
do not establish his presidential election or oath. A 20 March diplomatic letter
provides a separate presidential observation. Japan's contemporary diplomatic
report provides July party-office observations with month precision; it is not
an original CPSU election protocol. Its Ivashkov spelling remains unresolved.

The law's Russian transcription was checked against four GARF JPG facsimiles
(pages 1, 3, 13 and 14), including the signed date block. Downloaded law-page,
facsimile and letter response hashes are separate from the factual extracts.
The Japanese page was read through web retrieval, while its direct download
returned 403; no raw byte count or response hash is invented. All personal term
boundaries remain unknown. The Presidency has a sourced creation date and an
unknown end; other lifespans and the 1991 transitions remain unresolved. No USSR
institution or party is automatically mapped to Russia or an RSFSR counterpart.

[Russia](russia.json) adds 14 federal ballot-list observations from CEC resolution
42/337-8 of 16 August 2021 and five separately classified Duma factions recorded
on 12 October 2021. Two sources support 24 claims. The five dated faction leaders
are parliamentary-role observations, not party leadership or executive-office
terms. Reported faction memberships are retained without forcing a chamber total
or converting them into election results. Ballot labels and faction names are
not merged into game identities merely because they match.

CEC bulletin pages 1, 144 and 145 were downloaded and visually reviewed. The
resolution date is known; the bulletin's exact publication day is not. The
historical Duma article body was read through search retrieval after direct
fetches timed out; dynamic linked biographies were excluded. The CEC original's
checksum and the checked-in extract's checksum are distinct; the unavailable
Duma raw response has no asserted checksum. These observations do not certify a
2026 roster, full party history or continuity from the USSR.

[South Africa](south-africa.json) adds the 52 party rows in the IEC's dated 2024
National Ballot results report and separate seat allocations for those same
labels. These are 52 organization observations, not 104 parties. The seat report
has six additional independent-candidate rows outside this organization intake;
National Ballot vote shares alone do not define the overall seat formula. A
complete extraction of this bounded results table is not a complete country
party register. Names containing Alliance, Congress or Movement do not establish
coalition composition or organizational form.

Five sources support 109 claims and one separate Presidency institution. DA
statements provide distinct Federal Leader, Federal Chairperson and Federal
Council Chairperson observations; the 2023 acknowledgment and April 2026 election
results do not establish uninterrupted terms. Parliament's 14 June 2024 election
of Cyril Ramaphosa as President-elect is separate from inauguration and party
leadership. Original PDF pages and official article bodies were reviewed;
downloaded response hashes remain distinct from the five checked-in factual
extracts. No source artwork, game mapping, complete term or portrait eligibility
is added. Access on 14 September 2026 leaves the historical cutoff unchanged.

Together the nine packets contain 841 organization observations and 27
institution observations, supported by 1,613 claims across 61 cited sources.
The index assigns them to 92 open research batches. All nine certification
identity IDs now have partial discovery packets: the eight campaign cases
include a USSR → Russia transition with two separate jurisdictions. None
establishes an exhaustive country roster or supplies a new finished character;
151 other country identities still have no new discovery packet.

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
python -X utf8 -m unittest discover -s tools/avatars -p "test_brazil_research_s10f.py"
python -X utf8 -m unittest discover -s tools/avatars -p "test_tonga_research_s10g.py"
python -X utf8 -m unittest discover -s tools/avatars -p "test_ussr_research_s10h.py"
python -X utf8 -m unittest discover -s tools/avatars -p "test_russia_research_s10h.py"
python -X utf8 -m unittest discover -s tools/avatars -p "test_south_africa_research_s10h.py"
node --test tools/ui/check_leadership_research_review.cjs
```

Use the importer without `--check` only to reproduce France's packet from the
existing pinned snapshot. A different release requires an explicit source and
scope update. No tool here accesses the network or writes campaign saves.
