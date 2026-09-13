# C01 — worldwide party and institution census

S10.b added a separate [source-backed discovery intake](research/README.md) and
[bounded research index](research-index.json). S10.c extends France's financial
discovery register to 635 identities and refreshes the artwork inventory after
one reviewed Tupou IV cartoon. Discovery observations do not imply completed
leadership histories or an exhaustive country census.

S10.d adds a bounded [Japan discovery packet](research/japan.json): 16 submitted
lists in the July 2025 proportional election and seven separately identified
House parliamentary groups in February 2026. Four party offices have five dated
holder observations. These do not establish continuous terms, group-to-party
mappings or new game candidates.

S10.e adds a bounded [India discovery packet](research/india.json): 82 dated
recognition observations from ECI notifications of 23 March 2024, officially
republished in the Kerala Gazette on 28 March 2024. These comprise six national
rows and 76 state-jurisdiction rows across 26 jurisdictions. They are not 82
distinct parties or people. Repeated names across states remain separate until
identity reconciliation; frozen-name and pending-court-order qualifications and
the source's skipped Kerala row number are retained. Recognition attestation
does not establish a party's lifespan or present status. This packet adds no
leader, term, game mapping or avatar.

S10.f adds a bounded [Brazil discovery packet](research/brazil.json): 31
organization observations comprising 29 labels in the TSE's 2024 election-funding
table, a later Missão registration decision and a separate PMB/Democrata naming
record. These are not 31 distinct parties. The funding table reports 27 dated
releases and two unknown cells; the June 2024 aggregate announcement does not
date those later rows. The naming record retains both the original discrepancy
and the February 2026 rectification. Five primary sources support 34 claims,
with no new leader, term, game mapping or avatar. Checked-in factual extracts
are hashed; original HTTP responses and an archived registry snapshot were not
obtained, so no original-response checksum or registry-as-of claim is made.

The discovery index now totals six country packets, 773 organization and 18
institution observations, 45 sources, 1,460 claims and 83 work batches. No
exhaustive country census is closed; the research cutoff remains unchanged.

**Partial inventory recorded; C01 remains incomplete.** This is a reproducible
prerequisite audit for S10, originally recorded at
`872442d7411d9986321248b41ccadf79e2851d1c` and refreshed in S10.c with the new
portrait manifest. It does not mark S10 or G2 complete. Leadership records,
gameplay eligibility and campaign saves remain unchanged by this inventory.

The existing research cutoff remains **7 September 2026**, with fictional
templates beginning **8 September 2026** and ending at the exclusive boundary
**1 January 2036**. This audit did not research events after that cutoff. The
cutoff needs explicit review before content acceptance; future templates are
authored gameplay fiction, never forecasts or evidence of actual succession.

## Recount from current inputs

| Inventory | Count | What the count establishes |
| --- | ---: | --- |
| Country identities | 160 | 137 starting identities and 23 successors |
| Simulation party rows | 624 | 148 countries have rows; 12 have none |
| Organization type recorded | 50 party, 3 coalition, 571 unknown | Game row identity does not establish an exhaustive real-party census |
| Registered components / historical name phases | 20 | Across five parent rows; they are not necessarily simultaneous coalition members |
| Historical people / term records | 590 / 393 | 311 leader, 44 acting and 38 co-leader records; 65 refer to components |
| Party history status | 53 partial, 571 gap | No party history is marked complete |
| Executive seed observations | 132 | Original executive identity observations, not complete terms or party-chair facts |
| Institution policy records / historical executive gameplay grants | 12 / 93 | Existing role-policy evidence; neither is a complete institution/office census |
| Future shared-seat policies | 2 | Separate fictional co-chair rules, not historical collective-office coverage |
| Lifecycle disclosures | 23 | 19 existing bounded registry records and four continuation disclosures; some overlap |
| Historical / fictional registered cartoon assets | 54 / 4 | Includes the separately reviewed S10.c Tupou IV image for 1990; the census itself does not visually validate images |
| Historical art jobs / unknown appearance-eligibility people | 704 / 126 | Known backlog only; missing organizations will add work |
| Fictional templates | 2,556 | Exact exported IDs for represented rows/components; templates are not finished characters |
| Exhaustive all-organization country censuses | 0 | Unrepresented organization counts are **unknown**, not zero |

The production board has been regenerated against its declared current inputs,
including the new portrait manifest. Fresh extraction of `POLITIES` and
`D4_POLITIES` still matches all 624 stored party identities exactly.
[census.json](census.json) records the current source hashes. The reviewed
Tupou IV window removes one opening-year art job, leaving 704 known jobs;
it does not establish any additional historical role or close country coverage.
The previous 705-job baseline remains preserved in the S10.a/S10.b evidence.

The four byte-hashed Python readers now have explicit LF checkout rules, matching
the existing JSON and political source conventions. Census source hashes were
regenerated after newline normalization; reader logic and catalogue contents
were unchanged. The seven census tests and reproducibility check still pass.

## Certified cases first

The approved pathway's eight cases use nine identity IDs because USSR → Russia
is a transition case. Every row below still needs an exhaustive organization
census, role/collective-seat review and explicit uncertainty register.

| Case / identity | Game party rows | Registered components | Partial party histories | Known terms | Fictional templates |
| --- | ---: | ---: | ---: | ---: | ---: |
| France | 5 | 11 | 5 | 54 | 60 |
| Japan | 5 | 5 | 5 | 42 | 32 |
| India | 4 | 0 | 4 | 30 | 16 |
| Brazil | 6 | 0 | 6 | 28 | 24 |
| South Africa | 7 | 0 | 7 | 18 | 28 |
| Tonga | 0 | 0 | 0 | 0 | 0 |
| Saudi Arabia | 0 | 0 | 0 | 0 | 0 |
| USSR | 3 | 0 | 0 | 0 | 12 |
| Russia | 5 | 0 | 0 | 0 | 20 |

Tonga and Saudi Arabia have explicit institution/discovery work orders despite
having no simulation party rows. The same applies to Lebanon, UAE, Qatar,
Oman, Bahrain, Libya, Brunei, Bhutan, Maldives and Swaziland. Absence from this
game's row catalogue is not evidence of historical absence or a completed census.

## What the records separate

[represented-organizations.json](represented-organizations.json) preserves
every game row and registered component, including identity notes, cited sources
and declared history gaps. It has **644 inventory records**, not a claim that
there were exactly 644 distinct real political organizations. A parent can group
historical name phases or multiple organizations. Unrepresented minor,
dissolved, successor and independent organizations have not yet been enumerated;
each country's discovery count remains `null` in [countries.json](countries.json).

[roles-and-lifecycle.json](roles-and-lifecycle.json) preserves exact party role
labels, acting/co-leader kinds, component association, date precision and existing
sources. It keeps executive seed observations and explicit gameplay eligibility
grants separate. A party chair is not silently converted into head of state or
government. Institution policy notes and future seat rules do not stand in for
historical collective institutions.

Existing dated formations/dissolutions and editorial continuation disclosures
remain distinct. In particular, the stored South African DP continuation is
`unverified`; NP, Canadian PC and Reform carry `ceased` disclosures. The notes
retain merger, renamed-phase and registration/dissolution distinctions from
their cited source records. This audit adds no new historical finding or
automatic successor mapping. Discoveries mentioned in notes still require
individually reviewed organization IDs and lifespan/relationship records.

## Numbered work orders

[work-orders.json](work-orders.json) contains **969 open work orders**. They
cover only known work plus initial country discovery assignments, so they are
not an estimate of all sessions needed to finish the world.

| Prefix / phase | Orders | Required output |
| --- | ---: | --- |
| `C01-[nation]-ORG-001` | 160 | Cited organization census, including unrepresented/minor/dissolved/merged/successor bodies and jurisdiction intervals |
| `C01-[nation]-ROLE-001` | 160 | Distinct leader, chair, parliamentary, executive and collective-seat roles and appointment rules |
| `CH-[nation]-B###` | 150 | Source review of known parent/component leadership chains |
| `CF-[nation]-B###` | 316 | Review exact fictional identities, biographies, institutions, continuation and physical cartoon art |
| `CA-[nation]-B###` | 183 | Resolve dated likeness eligibility and complete the 704 known historical cartoon jobs |

Known-record batches contain at most ten members and follow the certified-first
country order, then the worldwide roster. A long chain may need further sessions;
batch assignment does not mark all its people reviewed. The 126 people with
unknown appearance eligibility are listed separately in the role ledger and
must acquire sourced identity/life/office associations before art windows are
assigned. A person shared by countries retains all known associations but each
physical art job is assigned only once. New discoveries need new stable IDs and
further numbered orders; nothing should overwrite a reviewed historical batch.

## What remains before C01 can close

1. Independently enumerate actual organizations and institutions for all country
   identities and applicable jurisdiction periods. Record unknowns and research
   leads explicitly, including minor, dissolved, merged and successor bodies.
2. Resolve organization types, distinct offices, collective seats and coalition
   membership intervals before assigning people or illustrations to them.
3. Review the research cutoff and historical/future boundary, and bind the
   complete inventory and uniquely numbered work orders to reviewed sources.

C01 establishes the inventory and work assignments. Subsequent character
sessions finish historical chains, fictional editorial review and artwork.
Finishing all 704 known art jobs is neither required to perform this census
nor sufficient to establish worldwide coverage. S10/G2 retain their unchanged
C01 prerequisite while this census is open.

## Reproduce and check

From the repository root:

```powershell
python tools/avatars/campaign_census.py
python tools/avatars/campaign_census.py --check
python -m unittest discover -s tools/avatars -p test_campaign_census.py -v
```

The generator reads local catalogues, preserves cited source URLs without
re-verifying those external pages, and records SHA-256 input identities. It
writes only the five C01 JSON artifacts. `--check` compares without writing;
the seven focused checks cover unknown coverage, complete nonduplicated work
assignment, ten-record bounds, exact role/date retention, lifecycle uncertainty,
stale-audit disclosure, duplicate future IDs and missing component exports.
No game build, simulation, image generation, roster mutation or network access
is required. After a relevant source change, regenerate and review the changed
inventory; unchanged output is reproducible byte for byte apart from Git's
configured checkout newline conversion.

Existing research and policy context: [leadership production](../../LEADERSHIP_PRODUCTION_2035.md),
[executive eligibility](../../PARTY_EXECUTIVE_ELIGIBILITY.md), and
[approved campaign pathway](../../CERTIFIED_CAMPAIGN_PATHWAY.md).
