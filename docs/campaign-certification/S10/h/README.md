# S10.h — Political research atlas and certified-country discovery

Status: **S10.h complete; S10 and C01 remain open.** G2 is unearned and
S11 is planned. Execution stopped after this increment.

Open the [political research atlas](http://127.0.0.1:7854/tools/ui/leadership-research-review.html?country=SouthAfrica).
It is also linked from the existing Government character review and cartoon
production pages. The game remains available on port 7853 with its unchanged
S10.g runtime; the atlas is a separate read-only reference.

## Review behavior

All 160 country identities remain selectable, with the nine certification
identities listed first. Select a country, search an organization, office or
person, and filter organizations, institutions or entries with recorded offices.
Each entry shows its identity, lifecycle uncertainty, distinct offices, holder
observations, attributed source claims and unresolved research. Source panels
retain access methods and qualifications as well as publication/access dates.

The atlas reads the current research register and verifies country/packet bytes
against its recorded SHA-256 identities. It clears the previous country while
loading, ignores superseded responses and refuses altered source files.
Missing packets show unknown coverage without erasing existing game party rows.
Large registers load 25 observations at a time; requesting more preserves open
evidence. Keyboard focus remains visible on cards and Enter opens their details.

The atlas does not load a campaign, issue a game command, appoint a person or
add a portrait. Date observations are not promoted to complete office terms;
unresolved game mappings do not imply an organization is absent from the game.

## New historical discovery

| Packet | Organization observations | Institutions | Sources | Claims |
| --- | ---: | ---: | ---: | ---: |
| South Africa | 52 | 1 | 5 | 109 |
| Russia | 14 | 5 | 2 | 24 |
| USSR | 1 | 3 | 3 | 8 |

South Africa preserves all 52 labels in the IEC 2024 National Ballot results
and separately sourced seat allocations for those labels. Six independent
candidate rows are accounted for outside the party intake. DA leader and chair
offices remain distinct; a presidential election observation is separate from
inauguration and party leadership.

Russia preserves the 14 federal ballot lists in the CEC's 16 August 2021
resolution and five separate Duma factions with parliamentary-leader observations
dated 12 October 2021. Ballot names do not automatically establish faction,
party or game-identity equivalence.

USSR separates the CPSU from the Presidency, Congress and Supreme Soviet.
The Presidency's creation on 14 March 1990 is distinct from personal appointment
or oath. July party observations retain month precision and the source's
unresolved spelling. USSR and Russia remain separate research jurisdictions.

Original IEC/CEC PDF pages and Soviet archival facsimiles were visually reviewed.
Only derived factual extracts are checked in. Downloaded-original hashes are
separate from extract hashes; unavailable Duma and Japanese page response bytes
remain explicitly unknown. Original source bodies and review images remain in
the separate source-work directories. The detailed [research notes](../../C01/research/README.md)
record each method and limitation. No source artwork is reused.

The generated register now contains nine partial packets, 841 organization
observations, 27 institutions, 58 sources, 1,606 claims and 92 open discovery
batches. All nine identities used by the eight certification cases have a
partial packet. There are still zero exhaustive censuses, and 151 other
identities have no new discovery packet. The cutoff remains 7 September 2026.
This increment adds no runtime historical term, game mapping or character art.

## Qualification and evidence

The [manifest](manifest.json) pins source revision
`233b13201f1d489b2a073c6bb4b6730f0ebabcdc`. The unchanged game runtime remains
`0f4616477671bc01936fcc9f2877ae172b13e348`.

- 102 content tests pass across research, census, importer and portrait checks.
- Five inventory/asset reproduction checks pass.
- The full Windows interface suite passes 1,565 tests, with one skipped.
- The actual browser checks all nine packets, search, filters, pagination,
  source claims, missing-country coverage, deep-link reload and keyboard use.
- Layout checks pass at 1440, 390 and 320 pixels. Four final screenshots were
  manually inspected, including the visible keyboard outline.
- Two explicitly authored error cases verify altered-packet refusal and an
  older country's delayed response. They are separate from the real-data checks.

Qualification ran on a clean, unchanged source revision. The browser uses only
GET requests against static research files and has no game API. No campaign,
Linux or new performance qualification is claimed. Simulation code, save
schemas, historical role grants and physical artwork are unchanged; the earlier
S10.g runtime evidence retains its original scope.

The [evidence inventory](evidence/inventory.json) contains 30 selected files and
806,131 stored bytes, excluding its own inventory file. It retains all declared
logs, browser results and screenshots, the manual review, exact runners, launch
proof and preservation checks. Stored bytes and Git index bytes are verified.
The preservation audit passes for eight protected files and two worktrees;
earlier saves, review servers and evidence remain intact.
