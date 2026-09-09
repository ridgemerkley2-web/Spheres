# Australia party leadership research

Checked through **7 September 2026**. Staging only: 42 real people, 49 office windows, four exact represented party rows and four inspected visual references. This is a partial historical catalogue, not a claim that every Australian party or every acting appointment is covered. Fictional successors belong after 7 September 2026 and require explicit fictional metadata.

## Opening identities and roles

| Exact row | Opening person ID | Office represented |
| --- | --- | --- |
| `au_alp` | `bob_hawke` | Federal parliamentary Labor leader |
| `au_lib` | `andrew_peacock` | Federal parliamentary Liberal leader |
| `au_nat` | `charles_blunt` | Federal parliamentary Nationals leader |
| `au_dem` | `janine_haines` | Federal parliamentary Democrats leader |

The Liberal and National parties are separate existing rows, not one shared coalition chair. Existing Bob Hawke name, native name and birth are preserved exactly. Other life facts are supplied only when the linked personal parliamentary or archival source was checked. An absent death date is unknown, not evidence that someone remains alive or eligible.

**Integration prerequisite:** new terms remain party-only. The Governor-General's appointment and House-confidence basis for prime minister require separate executive eligibility review. Preserve existing seated executive bindings. Democrats national president and parliamentary leader are separate offices under the [party constitution](https://www.democrats.org.au/constitution/); Leonie Green's current organisational presidency must not be relabelled as parliamentary leadership or an automatic prime-minister mandate. The [House practice](https://www.aph.gov.au/About_Parliament/House_of_Representatives/Powers_practice_and_procedure/Practice7/HTML/Chapter2/The_Ministry) describes ministerial appointment.

## Chronology quality and remaining gaps

The primary source set combines Parliament's individual biographies, original Hansard/journals, the Senate Biographical Dictionary, National Archives, government transcripts and current party announcements. The JSON carries sources beside each person, office and gap. Original party-role records take precedence over summaries that conflate office types or give inconsistent dates.

- **Labor:** the original 19 December 1991 Keating acceptance transcript establishes the party transition; a parliamentary quick guide's 16 December date is not used. Keating's March 1996 departure remains month precision rather than his prime-ministerial endpoint. Crean's 2 December 2003 office endpoint follows his biography, not the earlier resignation announcement. The Latham–Beazley acting interval in January 2005 remains a gap. Bowen's party acting role begins before his formal Opposition office. Albanese's party election is 27 May 2019, not his 2022 prime-minister appointment.
- **Liberals:** Peacock–Hewson is 3 April 1990. Downer's biography gives 30 January 1995, not the earlier announcement. Howard and Morrison final inclusive days are converted to next-day exclusive ends; the short subsequent vacancies remain explicit. Dutton's party biography gives the May 2025 endpoint despite losing his seat earlier. Ley's replacement by Angus Taylor is supported by the party's 13 February 2026 acceptance press conference.
- **Nationals:** the archived Blunt biography, dated 24 April 1997, ends his office on 6 April 1990; Fischer's personal biography begins 9 April, while a summary says 10 April. This transition deliberately retains April precision and a gap disclosure. Fischer–Anderson is 1 July 1999 for party leadership, distinct from the 20 July ministerial change. Joyce's 2017 court-ineligibility interruption is preserved, with the precise Scullion acting mandate unfilled. Littleproud's 10 March 2026 statement announced an intention to resign; its immediate effect is not assumed. Canavan's parliamentary biography and contemporary party statement establish 11 March 2026.
- **Democrats:** Haines' March 1990 exit and Coulter's October 1991 regular confirmation have limited precision. Some short acting appointments are not safely established. Original Senate announcements support **observed-office clips** for Greig on 26 August 2002 and Bartlett's return on 10 February 2004; these are not claimed as the original appointment/resumption days. November 2004 Hansard explicitly preserves existing leadership until the membership ballot, supporting Allison's 13 December start over the earlier announcement. The post-June 2008 organisational sequence and deregistration/re-registration continuity remain a substantial explicit gap. Leonie Green is observed as president by the party's 29 January 2026 interview post; her own statement says she was elected late in 2025, without an exact verified day.

All date intervals use **[from, until)**. Incoming holders own a verified shared transition day. A statement of an inclusive final day without a shared incoming transition becomes the next day as the exclusive endpoint. January 1990 starts are research scope clips, not asserted original election dates. Month/year dates deliberately retain uncertainty and must not be silently converted into invented precise events.

The modern Greens, One Nation, Country Liberal Party, Queensland LNP identity, United Australia, Katter's Australian Party, Centre Alliance and other omitted parties need separately reviewed IDs, affiliations and lifecycle rules. Their omission from the opening simulation is not a claim that they did not exist later; no opening support has been fabricated.

## Visual provenance

Each local path, source URL, page number, rights note and appearance observation is retained in `portrait_reference_audit` in the JSON. References stay outside shipped game assets. Copyrighted reference photographs are not marked as freely licensed artwork.

| Person | Inspected reference | Useful cues and limits |
| --- | --- | --- |
| Bob Hawke | [ANU Press, *Australia Goes to Washington*](https://press.anu.edu.au/downloads/press/n2237/pdf/book.pdf), PDF p141, 1986 White House photograph, NAA A8746, KN8/5/86/2 | Hawke is right of Reagan. Silver swept quiff, broad smile, clean-shaven, no glasses. The book is ©2016 ANU Press, all rights reserved; individual NAA image licence unreviewed. This is a 1986 reference for a 1990 interpretation. |
| Andrew Peacock | [Museum of Australian Democracy portrait on the 1990 speech page](https://www.moadoph.gov.au/explore/democracy/election-speeches/andrew-peacock-1990) | Side-parted dark hair, long face, serious mouth, striped tie, no glasses. Photo exposure is undated: neither the 1990 speech nor the 2026 media upload dates the photograph. |
| Charles Blunt | [*Milestone: A Centenary of Achievement*](https://www.page.org.au/wp-content/uploads/2020/01/Milestone-A-Centenary-of-Achievement-Final-small-size.pdf), PDF p76 / printed p70, 1987 parliamentary group | Caption identifies him second row, sixth from left. Short dark side part, long face, suit and tie; face is very small, so finer likeness and eye-colour details are not established. ©2020 Paul Davey, National Party publisher; photograph credited Federal Secretariat. |
| Janine Haines | [*Nuovo Paese*, March 1990](https://filefaustralia.org/wp-content/uploads/2025/02/1990-Marzo-Nuovo-Paese.pdf), PDF p8 / printed p6 | Curly dark bouffant/shag, short fringe and conspicuous large pale double-rim glasses; light collared jacket. March 1990 publication is established, camera date is unknown. Photographer licence not identified. The adjacent PDF p9 portrays Gordon Bilney and is not her reference. |

The live Nationals PDF endpoint returned 404; its archived 2024 capture was truncated and unusable. The intact 110-page original publication was retrieved from the Page Research Centre mirror, with the original publisher/copyright page intact. An unrelated museum interview image showing microphones was inspected and rejected as a Haines portrait. These failed or rejected files are not claimed as usable likeness evidence.

## Validation and handoff

The staged file passes the existing strict `people_from_registry`, `life` and `validate_row` validators: 42 people, 4 parties, 49 terms, 14 gap disclosures and 4 portrait audits. Existing Hawke identity facts compare equal. All referenced person IDs exist in this staging set, office and life bounds use the repository's tagged date schema, and the obsolete unverified `snpf_005.pdf` URL was removed. No runtime registry, portrait asset, save file, office link, server process or Git commit was changed by this research handoff.
