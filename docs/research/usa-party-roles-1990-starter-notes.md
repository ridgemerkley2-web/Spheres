# United States 1990 party-role audit

Checked September 7, 2026. Scope is the two existing simulation rows, `us_dem` and `us_rep`, at the 1990 start. This is not the full American party roster or a complete chronology through 2026.

`usa-party-roles-1990-starter.json` contains eight real people, two researched national committee chair terms, and six additional institutional role observations. The identity-only import added six people and enriched the existing `tom_foley` and `george_j_mitchell` identities. It preserved every existing person fact, source, party term, executive office link and portrait asset. The executive remains George H. W. Bush.

The two chair terms and six role observations remain research records only. The runtime's `eligible` and `on_succession_with` functions do not yet distinguish executive candidacy from the textual party-office role. Importing a DNC/RNC chair as a generic party leader could therefore make that person eligible to become president. Before those terms can enter the game, the model must separate national committee, House, Senate and executive offices, and test that committee office alone never grants presidential succession eligibility. The exact import and deferred term IDs are recorded in `usa-party-roles-ingestion-review.json`.

| Institution in January 1990 | Democratic role | Republican role |
| --- | --- | --- |
| National party organization | Ron Brown, DNC chair | Lee Atwater, RNC chair; Jeanie Austin, RNC cochair |
| House senior party leadership | Tom Foley, Speaker | Robert H. Michel, Minority Leader |
| House majority floor office | Dick Gephardt, Majority Leader | No majority floor office while Republicans were in the minority |
| Senate floor leadership | George J. Mitchell, Majority Leader | Bob Dole, Minority Leader |
| National executive | No Democratic president | George H. W. Bush, President |

The simulation's baseline explicitly uses the 1988 House popular vote to model congressional support. A national organization avatar should therefore say **DNC chair** or **RNC chair**; a congressional negotiation should identify the relevant chamber and office. Neither Brown nor Atwater held the Speaker/minority leader role, and a Republican president did not make House Republicans the majority. The [official House description](https://history.house.gov/People/Office/Minority-Leaders/) treats the minority leader as the counterpart to the Speaker, while the [majority floor leader](https://history.house.gov/People/Office/Majority-Leaders/) is a separate office.

## Primary evidence and boundary review

- Ron Brown: a [Congressional Record tribute](https://www.govinfo.gov/link/crec/142/H/3445) records his February 10, 1989 DNC election. The [official memorial volume](https://www.govinfo.gov/content/pkg/CDOC-104sdoc27/pdf/CDOC-104sdoc27.pdf) supplies biography and transition to Commerce in January 1993. The precise DNC handover day is unresolved here, so the end bound is a month.
- Lee Atwater: the [January 19, 1989 White House news summary, page A-6](https://www.reaganlibrary.gov/sites/default/files/2026-02/40-399-5730359-410-006-2025.pdf) reports election the preceding Wednesday. Presidential remarks on [January 2, 1990](https://www.presidency.ucsb.edu/documents/remarks-the-republican-national-committee) and [June 12, 1990](https://www.presidency.ucsb.edu/documents/remarks-reception-for-supporters-the-annual-republican-congressional-fundraising-dinner) confirm Atwater and Austin's different offices. Atwater's successor's [January 25, 1991 resignation letter](https://www.presidency.ucsb.edu/documents/letter-accepting-the-resignation-clayton-k-yeutter-secretary-agriculture) separates Yeutter's selection from his March 1 departure from Agriculture. The exact party handover, illness-period delegation and ceremonial roles need more evidence; 1991 remains unresolved, rather than inventing an exact termination day.
- Tom Foley: [House Speaker elections](https://history.house.gov/People/Office/Speakers-List/) record June 6, 1989. Each new Congress has a fresh election, so an exact office timeline should retain the intervening vacancies instead of assuming uninterrupted legal officeholding.
- Dick Gephardt: the [House Majority Leaders table](https://history.house.gov/People/Office/Majority-Leaders/) explicitly dates his election June 14, 1989.
- Robert H. Michel: the [House Minority Leaders table](https://history.house.gov/People/Office/Minority-Leaders/) lists the 97th through 103rd Congresses. Initial caucus election/effective dates were not checked in this starter audit.
- Mitchell and Dole: the [Senate's official leadership table](https://www.senate.gov/senators/majority-minority-leaders.htm) establishes their opposing floor offices during the 101st through 103rd Congresses. Exact caucus election/effective dates need separate review.

Additional biography sources are attached to each person. Jeanie Austin's April 2000 death uses a contemporary [Associated Press obituary](https://www.washingtonpost.com/archive/local/2000/04/26/jeanie-austin-dies-at-66/d7347710-f171-4b8a-ae00-a549c69b61a9/) as a secondary biographical source; it does not supply an independently verified full cochair tenure. Do not confuse her with the later librarian and author of the same name.

All terms are half-open and conservative. Month/year uncertainty describes incomplete research, not a historical vacancy. This identity-only import adds no US art, playable party terms, office assignments or succession behavior.
