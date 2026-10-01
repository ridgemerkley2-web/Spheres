# DA Federal Leaders 39: the Federal Leader of the Democratic Alliance, 2000-2026

Packet: **CLAUDE-C01-39**. State: **ready_for_review** (not accepted).
Owner: Claude. Integrator/reviewer: Codex. Branch `claude/c01-za-39`, based on `codex/campaign-certification` at
`02d2c5a2`, **not stacked** on a pending packet; claim commit `03cd0bb9`. Research access: 30 September 2026. The
historical cutoff stays **7 September 2026**.

This packet extends the existing party-leader role `za_da_federal_leader` (Federal Leader) on the IEC organization
observation `DEMOCRATIC ALLIANCE` (`za_iec_n2024_027`) in [south-africa.json](south-africa.json) back to the DA's
formation in 2000. It reviews 10 observations (ZA-DA-01..10) and adds 24 sources and 29 claims, eleven
holder observations of four people (Tony Leon, Helen Zille, Mmusi Maimane and John Steenhuisen), a rewritten role scope
note, and coverage notes on the organization and the packet. The S10h intake's two holder observations (John Steenhuisen
2023-04-03 and Geordin Hill-Lewis 2026-04-12), their sources and claims, and the Federal Chairperson and Chairperson of
the Federal Council roles are unchanged; the role now holds thirteen observations of five people in date order, every one
with `from` and `until` null. The IEC identity, its unknown lifecycle and empty game mapping are unchanged, and so is
every role, claim and holder of `za_presidency`, the ANC, the CLAUDE-C01-30 parties and the PAC. It adds no organization,
institution, game mapping, lifespan, portrait or avatar. The parent scope (C01, C06, S23, WC1 and CP1) remains open.

The Democratic Party (1989-2000) has no observation in this file. Its formation, its leaders (Zach de Beer and Tony Leon)
and the 2000 DP/NNP alliance that formed the DA are recorded as claims about organizations only, on the DA observation
with no role (`role_id` null), never as holders and never as a merged identity or predecessor link. The interim
leadership of November 2019 to November 2020 is claims only. Six people are named as leaders of either party, within the
limit of ten; other people appear only in other offices (the NNP's leader as the DA's Deputy Leader in 2000, the Federal
Chairperson in 2019, the interim Federal Chairperson, the Chairperson of the Federal Council and another party's leader).

## Outcome

| ID | Question | Decision |
|---|---|---|
| ZA-DA-01 | The Democratic Party, 1989-2000: claims about another organization | **Claims only:** a DP record of 22 May 1997 names "Tony Leon Leader of the Democratic Party"; the DP's undated pages give its formation on "April 8, 1989" by merger, "DP leader Zach de Beer" (retrospective), the heading "DP Leader; Spokesperson on Labour and the Presidency Tony Leon" and his election to the DP's leadership "In 1994" (year only); no DP observation, role or holder |
| ZA-DA-02 | 2000: the DP, the NNP and the Federal Alliance form the DA | **Claims only:** the DA's Deputy Leader on 14 October 2000: "Mr Leon lei die DP en ek lei die NNP in die DA in"; the DA's undated introduction page: the DP, NNP and Federal Alliance "formed the Democratic Alliance"; no formation day, merger of identities or lifecycle |
| ZA-DA-03 | Tony Leon in office, 2000 | **Accepted:** DA speech pages of 14 October and 22 November 2000 give "Position Leader of the Democratic Alliance" |
| ZA-DA-04 | May 2007: Helen Zille's election and Leon as former Leader | **Accepted in part:** Zille's acceptance speech of 6 May 2007 ("Today you have elected me as your leader") is an election reference; the undated 2007 profile styles Leon "Former Leader" (retrospective); neither is a start or an end |
| ZA-DA-05 | Helen Zille in office, 2010-2014 | **Accepted:** DA newsroom items bylined "Helen Zille, Leader of the Democratic Alliance" on 15 October 2010, 4 November 2012 and 9 May 2014 |
| ZA-DA-06 | May 2015: the sixth Federal Congress and Mmusi Maimane's election | **Accepted in part:** the DA's announcement of 10 May 2015 lists "Leader Mr Mmusi Maimane MP" (result publication) after the congress of "9 and 10 May 2015" (span); no start |
| ZA-DA-07 | Mmusi Maimane in office, 2016-2019 | **Accepted:** DA records of 25 April 2016, 7 April 2018, 8 April 2018 and 4 October 2019 |
| ZA-DA-08 | October-November 2019: Maimane's resignation and the interim leadership | **Accepted in part:** the FedEx was informed "On Wednesday" of his decision to resign (24 October 2019) and the positions "became vacant on Wednesday" (25 October 2019): claims, no end (ruling question); the interim election set for 17 November 2019 and the congratulation of John Steenhuisen as "Interim Federal Leader" (23 November 2019) are claims only |
| ZA-DA-09 | 2020-2023: John Steenhuisen's election and the office | **Accepted in part:** the Presiding Officers' results of 1 November 2020 ("Federal Leader: John Steenhuisen") are a result publication; Steenhuisen observed 10 March 2021; the S10h observation of 3 April 2023 is unchanged |
| ZA-DA-10 | 2026: the tribute, the last attestation and Geordin Hill-Lewis's election | **Accepted in part:** the tribute of 4 February 2026 ("six years of determined leadership as Federal Leader") is a claim, never an end; Steenhuisen observed 4 March 2026; the S10h observation of Hill-Lewis on 12 April 2026 is unchanged |

The resulting holder observations, in date order (every `from` and `until` is null; the two S10h rows are unchanged):

| Holder | `attested_on` | Basis |
|---|---|---|
| Tony Leon | 2000-10-14 | speech page: "Author Tony Leon MP", "Position Leader of the Democratic Alliance" |
| Tony Leon | 2000-11-22 | speech page: "Author Tony Leon MP", "Position Leader of the Democratic Alliance" |
| Helen Zille | 2010-10-15 | newsroom item bylined "Helen Zille, Leader of the Democratic Alliance 15 October 2010" |
| Helen Zille | 2012-11-04 | press release bylined "Helen Zille, Leader of the Democratic Alliance 4 November 2012" |
| Helen Zille | 2014-05-09 | speech item bylined "Helen Zille, Leader of the Democratic Alliance 9 May 2014" |
| Mmusi Maimane | 2016-04-25 | post headed "DA Leader unveils Local Government Election posters in Tshwane", styled "Federal Leader of the Democratic Alliance" |
| Mmusi Maimane | 2018-04-07 | post: "On Saturday, 7 April, DA Leader Mmusi Maimane confronted racist remarks" |
| Mmusi Maimane | 2018-04-08 | post: the speech "was delivered by the DA Federal Leader, Mmusi Maimane, to the party’s Federal Congress delegates" |
| Mmusi Maimane | 2019-10-04 | statement headed "DA finds no financial wrongdoing committed by Federal Leader Mmusi Maimane" |
| John Steenhuisen | 2021-03-10 | statement "Issued by John Steenhuisen MP – DA Federal Leader 10 Mar 2021" |
| John Steenhuisen | 2023-04-03 | S10h (unchanged): DA KwaZulu-Natal statement on his re-election as Federal Leader |
| John Steenhuisen | 2026-03-04 | statement "Issued by John Steenhuisen MP – Leader of the Democratic Alliance 04 Mar 2026" |
| Geordin Hill-Lewis | 2026-04-12 | S10h (unchanged): Presiding Officers' announcement of the Federal Congress 2026 results |

### How a start and an end are decided

The packet applies the CLAUDE-C01-30 and CLAUDE-C01-32 rule unchanged. A holder has `from` only where a source states the
day the office was assumed or took effect, and `until` only where a source states the day it ended. No record reviewed
states either, so every added holder is a dated observation citing only an in-office attestation made on its own
`attested_on` day; the test pins that every cited claim carries exactly the holder's date. Everything else is a claim
that never feeds a holder: elections and election references (6 May 2007), result publications (10 May 2015, 1 November
2020), congress sessions (9 and 10 May 2015), the resignation decision and the vacancy of October 2019, the interim
election and interim service, the retrospective "Former Leader" styling, and the tribute of 4 February 2026.

No end is inferred from a successor's election or first attestation. The S10h holders keep their own basis: the
Hill-Lewis observation of 12 April 2026 rests on an election result announcement, as the S10h intake recorded it, and is
left unchanged; the added holders follow the later rule that result publications never feed a holder. Interim service is
claims only, so Steenhuisen's added observations come after his election of November 2020 was published, and none rests
on the interim period. One added observation (Steenhuisen, 4 March 2026) falls between the two S10h observations; it is
inserted in date order and changes neither of them.

### The October 2019 resignation (ruling question)

The DA's Federal Council Chairperson stated on 24 October 2019 that "On Wednesday" the Federal Executive "was informed of
the personal decision of its Federal Leader, Mmusi Maimane, and Federal Chairperson, Athol Trollip to resign from their
positions", and on 25 October 2019 that "The Federal Leader and Federal Chairperson positions became vacant on
Wednesday". Both statements date the event only by a weekday; neither prints a calendar day of an accepted or effective
resignation. The rulings give `until` for a resignation "accepted or effective on a stated day"; the packet does not
convert "Wednesday" into a calendar day and leaves Maimane's last observation (4 October 2019) without an end. A ruling
is requested on whether a vacancy stated "on Wednesday" in a dated party statement gives `until` on the weekday it names.

### Party office, state office and the other roles

The role sits on the DA observation; the Presidency of the Republic stays in `za_presidency`, and the ANC, ACDP, Freedom
Front, IFP and PAC roles and the DA's two chair roles are unchanged. No other role's claim or source feeds this role, and
none of this role's feeds any other: every new claim and source id begins `za_da_`, and no other entry cites one. Offices
of the state, the legislature and the caucus printed in the same documents are never used: "MP" in the speech and
statement headers, the heading label "DA spokesperson on Leader of the Official Opposition" on Zille's acceptance speech
of 2007, the "Parliamentary Leader of the Democratic Alliance" styling beside "Federal Leader" in 2016, the President of
the Republic named in 2012, the 2014 national election, and Steenhuisen's ministry named in the tribute of 2026. The
Federal Chairperson, the Chairperson of the Federal Council (Zille in 2019), the Federal Finance Chairperson, the interim
Federal Chairperson and the DA's Deputy Leader of 2000 are other offices, never holders. The interim office ("Interim
Federal Leader") is recorded with its own title and never feeds a holder.

### Organizations: the Democratic Party, the DP/NNP alliance and the DA's formation

The DP and formation claims sit on the DA observation with `role_id` null and set no lifecycle, identity, predecessor
link or game mapping. The DP claims are about another organization: its formation "on April 8, 1989" by merger of the
Progressive Federal Party, the Independent party and the National Democratic Movement (an undated retrospective page, so
no structured date), "DP leader Zach de Beer" (retrospective), a DP speech of 22 May 1997 "by Tony Leon Leader of the
Democratic Party", the heading "DP Leader; Spokesperson on Labour and the Presidency Tony Leon" and his election to the
DP's leadership "In 1994" (year only). The 2000 alliance appears in the DA's Deputy Leader's speech of 14 October 2000
("Mr Leon lei die DP en ek lei die NNP in die DA in", Mr Leon leads the DP, and I the NNP, into the DA) and on the DA's
undated introduction page (the DP, NNP and Federal Alliance "formed the Democratic Alliance"); the 2007 profile says "The
formation of the Democratic Alliance, under Tony Leon's leadership, is the first step in that consolidation process".
None gives the DA's formation day, and the DP's Leader is never a holder of the DA's office, even where the same person
holds both.

### Date ledger

Each row is a separate dated fact with its own claim; two facts on one day stay two claims.

| Date | Event | Claim or field |
|---|---|---|
| "April 8, 1989" (undated page) | DP formed by merger (organization claim, retrospective) | `za_da_dp_formed_by_merger_retrospective` |
| undated | "DP leader Zach de Beer" (retrospective) | `za_da_dp_leader_de_beer_retrospective` |
| 1994 | Leon elected to the DP's leadership (year only, retrospective) | `za_da_dp_whos_who_leon_elected_dp_leadership_1994` |
| 22 May 1997 | "Tony Leon Leader of the Democratic Party" (DP record; claims only) | `za_da_dp_speech_leader_of_dp_leon_19970522` |
| undated (capture 1999) | "DP Leader; ... Tony Leon" (DP page) | `za_da_dp_whos_who_dp_leader_leon_1999` |
| 14 October 2000 | DP and NNP led "in die DA in" (organization claim) | `za_da_speech_dp_and_nnp_led_into_da_20001014` |
| 14 October 2000 | Leon, "Position Leader of the Democratic Alliance" | `za_da_speech_leader_leon_20001014`; Leon `attested_on` |
| 22 November 2000 | Leon, "Position Leader of the Democratic Alliance" | `za_da_speech_leader_leon_20001122`; Leon `attested_on` |
| undated (capture 2001) | DP, NNP and Federal Alliance "formed the Democratic Alliance" | `za_da_formed_by_dp_nnp_fa_retrospective` |
| 6 May 2007 | Zille: "Today you have elected me as your leader" (election reference) | `za_da_zille_elected_leader_acceptance_20070506` |
| undated (capture 2007) | Leon styled "Former Leader of the Democratic Alliance"; formation "under Tony Leon's leadership" | `za_da_profile_leon_former_leader_2007`, `za_da_profile_formation_under_leon_leadership` |
| 15 October 2010 | "Helen Zille, Leader of the Democratic Alliance" | `za_da_sa_today_leader_zille_20101015`; Zille `attested_on` |
| 4 November 2012 | "Helen Zille, Leader of the Democratic Alliance" | `za_da_press_release_leader_zille_20121104`; Zille `attested_on` |
| 9 May 2014 | "Helen Zille, Leader of the Democratic Alliance" | `za_da_speech_leader_zille_20140509`; Zille `attested_on` |
| 9 and 10 May 2015 | Sixth Federal Congress (span) | `za_da_sixth_federal_congress_9_10_may_2015` |
| 10 May 2015 | "Leader Mr Mmusi Maimane MP" elected (result publication) | `za_da_leader_maimane_elected_published_20150510` |
| 25 April 2016 | "DA Leader unveils ..." (Maimane) | `za_da_news_leader_maimane_20160425`; Maimane `attested_on` |
| 7 April 2018 | "DA Leader Mmusi Maimane" | `za_da_news_leader_maimane_20180407`; Maimane `attested_on` |
| 8 April 2018 | "the DA Federal Leader, Mmusi Maimane" (closing speech to the Federal Congress) | `za_da_closing_speech_federal_leader_maimane_20180408`; Maimane `attested_on` |
| 4 October 2019 | "Federal Leader Mmusi Maimane" | `za_da_statement_federal_leader_maimane_20191004`; Maimane `attested_on` |
| 24 October 2019 | FedEx informed "On Wednesday" of the decision to resign (claim) | `za_da_fedex_informed_maimane_resignation_decision_20191024` |
| 25 October 2019 | Positions "became vacant on Wednesday" (claim; no end) | `za_da_federal_leader_position_vacant_20191025` |
| 25 October 2019 | Interim election set for "Sunday, 17 November 2019" (prospective; interim) | `za_da_fedco_interim_leader_election_scheduled_20191025` |
| 23 November 2019 | Steenhuisen congratulated as "Interim Federal Leader" (interim; claims only) | `za_da_fedex_congratulates_interim_leader_steenhuisen_20191123` |
| 1 November 2020 | "Federal Leader: John Steenhuisen, with 80% of votes cast" (result publication) | `za_da_leader_steenhuisen_elected_published_20201101` |
| 10 March 2021 | "John Steenhuisen MP – DA Federal Leader" | `za_da_statement_federal_leader_steenhuisen_20210310`; Steenhuisen `attested_on` |
| 3 April 2023 | S10h: re-elected Federal Leader (DA KZN) | `za_da_steenhuisen_attested_20230403` (unchanged) |
| 4 February 2026 | Tribute: "six years of determined leadership as Federal Leader" (claim; never an end) | `za_da_tribute_steenhuisen_six_years_federal_leader_20260204` |
| 4 March 2026 | "John Steenhuisen MP – Leader of the Democratic Alliance" | `za_da_statement_leader_steenhuisen_20260304`; Steenhuisen `attested_on` |
| 12 April 2026 | S10h: Hill-Lewis elected Federal Leader | `za_da_leader_elected_20260412` (unchanged) |

## Observations

### ZA-DA-01 — The Democratic Party, 1989-2000: claims about another organization

Evidence: a DP website archive item (captured 17 November 1999) is headed "Speech 22 May 97 by Tony Leon Leader of the
Democratic Party" (`za_da_dp_speech_leader_of_dp_leon_19970522`). The DP's "Party Info" page (captured 13 April 2000,
undated) says "The Democratic Party was formed on April 8, 1989, when the former Progressive Federal Party, Independent
party and National Democratic Movement merged" and "DP leader Zach de Beer was chosen as the first Management Committee
Chairman of Codesa". Its "Who's Who" page (captured 20 February 1999, undated) heads its first entry "DP Leader;
Spokesperson on Labour and the Presidency Tony Leon" and says "In 1994 he was again elected to Parliament and then to the
leadership of the DP".

Decision: claims only. The DP has no observation in this file; the claims sit on the DA observation with no role and are
never holders of the DA's office, starts, ends or a merged identity. The formation day is printed only in an undated
retrospective page and is not stored as a structured date.

Limits: no dated DP record of de Beer in office, of the DP's 1994 congress or of the DP's end was found; the DP's website
dates from 1997.

### ZA-DA-02 — 2000: the DP, the NNP and the Federal Alliance form the DA

Evidence: the speech of the DA's "Deputy Leader of the Democratic Alliance" dated "10/14/00" says "Mr Leon lei die DP en
ek lei die NNP in die DA in, waar ons saam sterker gaan wees" (`za_da_speech_dp_and_nnp_led_into_da_20001014`). The DA's
introduction page (captured 22 November 2001, undated) says "The Democratic Party , New National Party and Federal
Alliance formed the Democratic Alliance" (`za_da_formed_by_dp_nnp_fa_retrospective`).

Decision: claims only, about organizations. In October 2000 the DP and the NNP are described as parties led into the DA,
so the three stay separate organizations here; no formation day, merger of identities, predecessor link or lifecycle is
set.

Limits: no DA record of its founding day or of the NNP's later departure was found in the captures reviewed.

### ZA-DA-03 — Tony Leon in office, 2000

Evidence: DA speech pages dated "10/14/00" and "11/22/00" give "Author Tony Leon MP" and "Position Leader of the
Democratic Alliance" (`za_da_speech_leader_leon_20001014`, `za_da_speech_leader_leon_20001122`).

Decision: accepted. Two dated observations; "MP" is a state office and never used.

Limits: the DA's later speech and news pages (2001-2007) label authors with a code ("(ldr)", also used for other
spokespeople) or as "Leader of the Official Opposition", a parliamentary office, and its weekly "SA Today" letters are
signed with an image; none was used, so no Leon observation after 2000 was found.

### ZA-DA-04 — May 2007: Helen Zille's election and Leon as former Leader

Evidence: "Helen Zille`s acceptance speech", dated "Sunday, May 06, 2007", says "I accept nomination as your leader" and
"Today you have elected me as your leader" (`za_da_zille_elected_leader_acceptance_20070506`); its heading label reads "DA
spokesperson on Leader of the Official Opposition". The DA profile page captured 7 September 2007 is headed "Tony Leon :
Former Leader of the Democratic Alliance" (`za_da_profile_leon_former_leader_2007`).

Decision: accepted in part. The acceptance speech is an election reference that names "your leader", not the office, and
states no day she assumed it: never a start or a holder observation. The heading label is a parliamentary office and
feeds nothing. The "Former Leader" styling is undated and is never Leon's end.

Limits: no DA record of the 2007 Federal Congress result, of Leon's last day or of Zille in office from 2007 to 2009 was
found in the captures reviewed.

### ZA-DA-05 — Helen Zille in office, 2010-2014

Evidence: DA newsroom items bylined "Helen Zille, Leader of the Democratic Alliance" on 15 October 2010 (SA Today), 4
November 2012 (press release) and 9 May 2014 (speech).

Decision: accepted. Three dated observations.

Limits: the 2010 and 2012 Federal Congress results and her decision not to stand in 2015 were not found in the material
reviewed.

### ZA-DA-06 — May 2015: the sixth Federal Congress and Mmusi Maimane's election

Evidence: the DA post dated "May 10, 2015" says "The Democratic Alliance held its sixth Federal Congress this weekend, 9
and 10 May 2015" and lists "Leader Mr Mmusi Maimane MP" among those elected
(`za_da_sixth_federal_congress_9_10_may_2015`, `za_da_leader_maimane_elected_published_20150510`).

Decision: accepted in part. The congress span and the result publication are claims; neither is a start, and neither is
Zille's end.

### ZA-DA-07 — Mmusi Maimane in office, 2016-2019

Evidence: the post of 25 April 2016 headed "DA Leader unveils Local Government Election posters in Tshwane", styled to
Maimane as "Federal Leader of the Democratic Alliance"; the post of 7 April 2018 ("DA Leader Mmusi Maimane"); the closing
speech to the Federal Congress of 8 April 2018 ("delivered by the DA Federal Leader, Mmusi Maimane"); and the statement of
4 October 2019 by the Federal Finance Chairperson ("Federal Leader Mmusi Maimane", "Democratic Alliance (DA) Leader, Mmusi
Maimane").

Decision: accepted. Four dated observations. The "Parliamentary Leader" styling beside "Federal Leader" in 2016 is a
separate caucus office and feeds nothing; the 2018 closing speech states no election result.

### ZA-DA-08 — October-November 2019: Maimane's resignation and the interim leadership

Evidence: the statement "Issued by Helen Zille – DA Federal Council Chairperson 24 Oct 2019" says "On Wednesday" the
FedEx was informed of the decision of "its Federal Leader, Mmusi Maimane" to resign; the statement of 25 October 2019 says
the positions "became vacant on Wednesday" and that the Federal Council will "elect an Interim Federal Leader" on "Sunday,
17 November 2019"; the National Spokesperson's statement of 23 November 2019 says the FedEx began "by congratulating John
Steenhuisen and Ivan Meyer on their election as Interim Federal Leader and Interim Federal Chairperson respectively".

Decision: accepted in part. All four are claims: the resignation decision and the vacancy are dated by weekday only and
give no end (see [the ruling question](#the-october-2019-resignation-ruling-question)); the interim election and the
interim office are claims only and never a start.

### ZA-DA-09 — 2020-2023: John Steenhuisen's election and the office

Evidence: the Presiding Officers' announcement of 1 November 2020 gives "Federal Leader: John Steenhuisen, with 80% of votes
cast" (`za_da_leader_steenhuisen_elected_published_20201101`); his statement "Issued by John Steenhuisen MP – DA Federal
Leader 10 Mar 2021" (`za_da_statement_federal_leader_steenhuisen_20210310`). The S10h observation of 3 April 2023 is
unchanged.

Decision: accepted in part. The result publication is a claim; the 2021 statement dates a holder observation.

### ZA-DA-10 — 2026: the tribute, the last attestation and Geordin Hill-Lewis's election

Evidence: the National Spokesperson's statement of 4 February 2026 "extends our sincere gratitude to John Steenhuisen for
six years of determined leadership as Federal Leader of the party" as he "now focuses fully on his responsibilities as
Minister of Agriculture"; his statement "Issued by John Steenhuisen MP – Leader of the Democratic Alliance 04 Mar 2026".
The S10h observation of Hill-Lewis on 12 April 2026 is unchanged.

Decision: accepted in part. The tribute states no end day and is never an end (the DA styles him its Leader again a month
later); the ministry is a state office. The 4 March 2026 statement dates a holder observation.

## Sources added

| Source ID | What | Retained provenance |
|---|---|---|
| `za_da_dp_speech_taalbeleid_19970522` | Die DP se Taalbeleid (DP website archive item: speech of 22 May 1997 by Tony Leon, Leader of the Democratic Party) | 22,392 bytes, `e998f741…2fac40`; capture 1999-11-17 |
| `za_da_dp_party_info_page_2000` | The Democratic Party of South Africa -- Party Info (DP website history page) | 24,997 bytes, `1fa2a429…00d5c4`; capture 2000-04-13 |
| `za_da_dp_whos_who_page_1999` | The Democratic Party of South Africa -- Who's Who (DP website page) | 25,299 bytes, `164b275f…1c3bdc`; capture 1999-02-20 |
| `za_da_speech_campaign_launch_deputy_leader_20001014` | DEMOCRATIC ALLIANCE - A PARTY FOR ALL THE PEOPLE (DA speech, National Election Campaign Launch, Cape Town, 14 October 2000) | 7,920 bytes, `afb6df11…3dfaa7`; capture 2001-05-06 |
| `za_da_about_introduction_page_2001` | Democratic Alliance: About the DA, Introduction (DA website page) | 2,348 bytes, `ff35358d…f2b7da`; capture 2001-11-22 |
| `za_da_speech_local_elections_launch_20001014` | Launch of the DA's Local Elections Campaign (DA speech, Cape Town Civic Center, 14 October 2000) | 15,073 bytes, `532864af…bdd922`; capture 2001-05-01 |
| `za_da_speech_kzn_coalition_20001122` | The ANC/IFP KZN coalition: a cozy cabal of power, perks and privileges (DA speech, Durban, 22 November 2000) | 11,474 bytes, `3ed5f3fa…e9f9ff`; capture 2001-05-02 |
| `za_da_zille_acceptance_speech_20070506` | Helen Zille`s acceptance speech (DA news article, 6 May 2007) | 55,578 bytes, `4c069e13…4171c0`; capture 2007-05-18 |
| `za_da_leon_former_leader_profile_2007` | Tony Leon : Former Leader of the Democratic Alliance (DA website leader profile) | 34,802 bytes, `1b157954…06d92c`; capture 2007-09-07 |
| `za_da_sa_today_provinces_20101015` | The drive to destroy the provinces is purely political (DA newsroom, SA Today, 15 October 2010) | 23,610 bytes, `e0bc603e…9d5b41`; capture 2010-10-19 |
| `za_da_press_release_nkandla_20121104` | Zuma has 72 hours to answer for Nkandla (DA press release, 4 November 2012) | 20,704 bytes, `1a8c3466…16d007`; capture 2012-11-08 |
| `za_da_speech_growth_victory_20140509` | DA's growth is a victory for all South Africans (DA speech, 9 May 2014) | 21,843 bytes, `ac490800…ce70e5`; capture 2014-06-03 |
| `za_da_federal_leadership_announcement_20150510` | Announcement of DA Federal Leadership (DA news, 10 May 2015) | 69,610 bytes, `67a5d71e…dd6cd7`; capture 2015-07-15 |
| `za_da_news_election_posters_20160425` | DA Leader unveils Local Government Election posters in Tshwane (DA news, 25 April 2016) | 75,816 bytes, `a8f9b17b…9ff6c2`; capture 2016-04-26 |
| `za_da_news_puppet_allegations_20180407` | Maimane confronts ‘puppet’ allegations head on (DA news, 7 April 2018) | 78,240 bytes, `3ce2f52b…8ac6a3`; capture 2018-04-07 |
| `za_da_closing_speech_congress_20180408` | Closing Speech Congress 2018 (DA news and speeches, 8 April 2018) | 86,722 bytes, `60cd833b…fb5d25`; capture 2018-04-08 |
| `za_da_finance_chair_statement_20191004` | DA finds no financial wrongdoing committed by Federal Leader Mmusi Maimane (DA statement, 4 October 2019) | 95,560 bytes, `d2a31ff9…30cce0`; capture 2019-10-06 |
| `za_da_fedcouncil_chair_leadership_vacancies_20191024` | DA working to fill Federal Leadership vacancies as a matter of urgency (DA statement, 24 October 2019) | 145,135 bytes, `28b3452b…5a76c0`; capture 2021-11-08 |
| `za_da_fedcouncil_chair_interim_election_20191025` | FedCo to elect Interim Leadership on Sunday, 17 November 2019 (DA statement, 25 October 2019) | 118,278 bytes, `521125be…23de64`; capture 2019-10-28 |
| `za_da_fedex_statement_interim_leadership_20191123` | DA is rebuilding, refocusing and reconnecting with the people of South Africa (DA statement, 23 November 2019) | 112,207 bytes, `3251192c…a01b85`; capture 2019-11-28 |
| `za_da_leadership_election_results_20201101` | DA Leadership Election Results (DA Federal Congress Presiding Officers, 1 November 2020) | 140,775 bytes, `3a94aa39…e562c3`; capture 2020-11-01 |
| `za_da_statement_stellenbosch_visit_20210310` | DA leader John Steenhuisen visits and supports Afrikaans students in Stellenbosch (DA statement, 10 March 2021) | 134,450 bytes, `730846b6…d5e32c`; capture 2021-03-11 |
| `za_da_statement_thanks_steenhuisen_20260204` | DA thanks John Steenhuisen for six years of service as Federal Leader (DA statement, 4 February 2026) | 125,751 bytes, `ab6172e0…ac3c49`; capture 2026-02-04 |
| `za_da_statement_condolences_cope_20260304` | DA Leader Steenhuisen sends condolences to COPE following the passing of Co-founder and Leader Mosiuoa Lekota (DA statement, 4 March 2026) | 121,009 bytes, `11cb414d…5d34ca`; capture 2026-04-10 |

Source types: `primary_party_speech_archived`, `primary_party_statement_archived`, `party_web_page_archived`,
`party_history_page_archived` and `party_biography_archived`. No source republishes third-party text.

## Response identities and stability checks

Codex re-downloads every recorded response and compares its byte count and SHA-256, so each extract carries the recipe
the identities depend on: fetch the recorded `url` exactly, with **no `Accept-Encoding` request header and no automatic
decoding** (for example `curl -s -o FILE URL`, not `--compressed`), and hash the bytes as received. All 24
captures are served without content encoding, and for every one the base32 SHA-1 of the bytes equals the Internet
Archive's CDX digest for the capture (recorded in each extract as `source_response_sha1_base32`).

Stability: every response was downloaded at least twice, the second at least 30 minutes after the first, with the same
byte count and SHA-256; each extract's `stability_check` gives the times (first downloads 2026-09-30T23:25:08Z to 2026-10-01T00:17:06Z, second downloads
2026-10-01T00:04:47Z to 2026-10-01T00:48:25Z). `packet_check.py` downloaded every response again before commit (one rate-limited response was re-downloaded by hand;
see Checks). Two DP pages were switched
to digest-matching captures: the "Who's Who" capture of 2 December 1998 serves the same bytes as the recorded capture of
20 February 1999 under a CDX digest that does not match them, and the "Party Info" captures of 3 December 1998 and 10
February 1999 serve a 24,990-byte version whose digest does not match; the capture of 13 April 2000, whose digest
matches, is recorded. The 2026 condolence statement's first capture (5 March 2026) redirects (HTTP 302) to the recorded
capture of 10 April 2026, which serves HTTP 200 directly.

Per-request traits: no response is generated per request. The DA's 2000-2008 pages are ASP pages and its 2008-2014
newsroom items carry query strings, but the archive replays the bytes fixed at capture; the WordPress posts of 2015-2026
carry widgets and banners fixed at capture (the capture of 8 November 2021 of the 24 October 2019 statement carries a
later site banner). No search, feed, tag, category, listing, API or cache-busting URL is used as a source; CDX queries and
archived listing pages were used for discovery only.

## Leads not imported

- DA captures reviewed and not used: the 2002 and 2004 "National Leadership" pages (undated rosters whose layout leaves the
  Leader and Deputy Leader cells ambiguous); speech and news pages of 2001-2007 labelled "(ldr)" (a code also used for
  other spokespeople, e.g. `DA/Site/Eng/Speeches/Speech.asp?ID=393`) or "Leader of the Official Opposition" (a
  parliamentary office, e.g. `Speech.asp?ID=1190`); the weekly "SA Today" letters of 2004-2008 (`print_satoday.asp`),
  signed with an image; the 2007 Federal Congress page `da/Site/Eng/congress/congress.asp.htm` (candidate profiles, no
  result); a 2007 speech by the Parliamentary Leader saying "Helen Zille was elected national head" (`Speech.asp?ID=1468`,
  another member's remark without the office); the 2010 "national-leaders" profiles (undated).
- DA posts of 2016-2026 left out to keep the set compact or because they name candidates who never held the office: the
  candidate lists of 31 October 2019 (interim), October 2020 and 31 March 2026; the parliamentary-leader announcement of
  October 2019 (`2019/10/da-pleased-to-announce-john-steenhuisen-as-the-new-parliamentary-leader`, another office); the
  February 2026 nominations notice (served gzip-encoded); the post `2016/01/maimane-launches-stand-up-speak-out-initiative/`
  (a further attestation).
- DP pages: the 1997 roster `old/leader.htm` ("Leader: Tony Leon MP", undated) and the press-release listings of 2000 (a
  database error page and a growing list).
- History sites, encyclopaedias and news, leads only: https://en.wikipedia.org/wiki/Democratic_Alliance_(South_Africa),
  https://en.wikipedia.org/wiki/Democratic_Party_(South_Africa), sahistory.org.za party pages, and news reports of the
  2007, 2010, 2012, 2015, 2018, 2020, 2023 and 2026 congresses and of Maimane's resignation (news24, dailymaverick,
  iol.co.za, politicsweb republishing DA statements). The commonly reported days 24 June 2000 (the DA's launch), 23
  October 2019 (Maimane's resignation) and 17 November 2019 (the interim election) are pinned as never-holder dates.

## Sources attempted

- Internet Archive: heavy rate limiting (HTTP 429 and refused connections) during the session, with other packets running
  in parallel; every capture used was retrieved and re-downloaded after waiting, and no limit was bypassed.
- The DA's 2000-2001 speech archive listing (`site/asp/SpeechArchive.asp`) and CDX listings were used for discovery only.
- IEC: no IEC record of the party office was used; the archived IEC registered-party pages reviewed for CLAUDE-C01-32
  record only a party contact person, and the S10h ballot sources record no leader.
- Not found in the captures reviewed: a DA record of its founding day; the Federal Congress results of 2000-2006, 2010 and
  2012; a DA record of Leon after 2000 that names the party office; Zille in office in 2007-2009 and 2015; Maimane's own
  resignation statement on the DA site.

## Suggested next work orders

These are proposals for the integrator. They are not created in `work-orders.json`.

- `C01-SouthAfrica-DA-002`: DA records of 2001-2009: Leon's congresses and statements naming the party office, the NNP's
  departure, and the 2007 Federal Congress result.
- `C01-SouthAfrica-DA-003`: the 2010 and 2012 Federal Congress results and Zille's announcement of April 2015.
- `C01-SouthAfrica-DP-001`: a Democratic Party observation (1989-2000), if the integrator wants the DP's own leaders as
  holders: dated DP records of Zach de Beer and of the 1994 congress.

## Integration notes (outside this packet's file boundary)

- **Base, stack and claim:** the branch is based on `codex/campaign-certification` at `02d2c5a2` and is **not stacked**;
  the claim commit `03cd0bb9` holds only the handoff. Before committing,
  `codex/campaign-certification` had moved to `79ef97ec` (simulation, campaign-matrix and planning records; no research
  file) and then to `f3e18e83` (campaign-leader art, selector checks and a regenerated `census.json`; no research file);
  both were merged without conflict (merge commits `0183c1e1` and `b0b9006c`).
- **Roadmap:** the user chose to start this batch (CLAUDE-C01-38 to 41) before Codex's roadmap line
  "continue existing claims first"; this packet was started on the user's instruction.
- `research-index.json` is regenerated in a **separate commit** and is the only file this packet shares with the parallel
  packets CLAUDE-C01-38, 40 and 41 (other country files); regenerate it when integrating. New totals for South Africa:
  269 sources and 500 claims (previously 245 and 471); role observations stay 11; organization, institution, packet
  and batch counts are unchanged, `mapping_pending` stays 53 and South Africa keeps six open batches.
- The DA role's scope note is rewritten (declared edit): the S10h note said "Two discrete attestations"; the new note keeps
  its rule (do not fill the interval or infer the outgoing holder's last day; no state or parliamentary office) and states
  the packet's rules. The role's `sources` and `claim_ids` and the organization's lists are extended at the end; no
  existing holder, claim, source or extract changes.
- The packet-level coverage note is inserted after the CLAUDE-C01-32 note and before the CLAUDE-C01-21 and CLAUDE-C01-16
  notes, which their own tests pin as the last two entries; no existing note changes.
- Pinned tests, none loosened and no assertion removed:
  - `test_south_africa_research_s10h.py`: counts (entries, sources, claims, roles) are now (53, 269, 500, 11);
    the exact DA holder list is re-expressed as this packet's thirteen holders (the two S10h holders included); the exact source list, response pins, hosts and access dates (2026-09-30 for this packet) and the undated
    claim count (7 + 31 + 13 + 21 + 30 + 8) are extended exactly.
  - `test_south_africa_heads_of_state_c01_09.py`: the exact source list is extended.
  - `test_south_africa_anc_presidents_c01_16.py` and `test_south_africa_deputy_presidents_c01_21.py`: the exact source
    order after their own sources and the index totals (11, 500).
  - `test_south_africa_party_leaders_c01_30.py` and `test_south_africa_pac_presidents_c01_32.py`: the exact source order,
    the index totals (11, 500) and the coverage-note positions (each earlier note one place further from the end,
    the same order re-expressed, with this packet's note third from last).
- The new test `test_south_africa_da_federal_leaders_c01_39.py` pins the other roles by their holder counts and the role
  map literally rather than importing them, so the earlier tests can import its response pins without a circular import.
- New extract fields match CLAUDE-C01-32 (`source_response_sha1_base32`, `source_response_content_encoding`,
  `fetch_recipe`, `stability_check`, `review_observation`, `printed_range`); rows of organization claims carry `role_id`
  null, and the DP rows carry the printed DP title in `role_title` with `role_id` null.
- `research/README.md`, the C01 README totals, the task queue and `docs/planning/ai-workstreams.json` are left for the
  integrator. No existing extract, UI code, game data or gap ledger is changed.

## Checks

```text
python -X utf8 tools/avatars/campaign_research.py
python -X utf8 tools/avatars/campaign_research.py --check
python -X utf8 tools/avatars/campaign_census.py --check
python -X utf8 -m unittest discover -s tools/avatars -p "test_south_africa*.py"
python -X utf8 -m unittest discover -s tools/avatars -p "test_*research*.py"
python -X utf8 -m unittest discover -s tools/avatars -p "test_campaign*.py"
node --test tools/ui/check_leadership_research_review.cjs
python tools/planning/workboard.py --check
git diff --check
python -X utf8 D:/spheres-scratch/c01-pipeline/tools/packet_check.py 39
```

Results are recorded in the handoff: the index regeneration and `--check`, `campaign_census.py --check` (after the merge
of `f3e18e83`, which regenerated `census.json`), the South Africa (58 tests), research (79) and campaign (16) Python
tests, the Node check (11) and the workboard check pass, and `git diff --check` is clean. `packet_check.py 39`:
run on the final tree before commit (`0183c1e1` plus the working tree): 24 new sources re-downloaded, 23 matching their recorded identities and one rate-limited (HTTP 429), which was re-downloaded by hand at 00:58Z with the same 145,135 bytes and SHA-256; every suite check passed except `census --check`, which the later merge of `f3e18e83` fixed (see above). It is run again after the push; that run's summary line is returned with this submission.

Access dates are the session's local date (30 September 2026, UTC-7); the two DP captures switched for their CDX
digests were first downloaded at 00:16-00:17Z on 1 October UTC. Known failures outside the listed checks, not fixed:
`tools/avatars/test_certified_gap_ledger.py` reports "no pinned attribution" for a new packet until Codex classifies its
commit in `COMMIT_PACKETS`; `tools/avatars/test_certified_boundary_matrix.py` (S23) needs `spheres-web/src`, which the
sparse checkout lacks, so Codex regenerates `docs/campaign-certification/S23/preparation/boundary-matrix/` on
integration.
