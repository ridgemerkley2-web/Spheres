# Tonga Deputy Prime Ministers 36: the Deputy Prime Minister, 1990-2021

Packet: **CLAUDE-C01-36**. State: **ready_for_review** (not complete).
Owner: Claude. Integrator/reviewer: Codex. Branch `claude/c01-to-36`; claim commit `e8821f40` on base `44098c5a` (then the
head of `codex/campaign-certification`); not stacked on a pending packet. Research access: 28 September 2026 (local; 29
September UTC), when every recorded response was downloaded twice for this packet. The historical cutoff stays **7 September
2026**.

This packet reviews ten observations about the existing Deputy Prime Minister role (`to_deputy_pm`) of the existing
`to_cabinet` institution in [tonga.json](tonga.json), from 1 January 1990 to the existing holder observation of Poasi Mataele
Tei (from 28 December 2021): the office-holder when the period opens, and each later Deputy Prime Minister's appointment,
attestations, resignation, removal or death, with acting service. It adds 39 sources and 53 claims (45 on `to_deputy_pm` and
`to_cabinet`, and 8 on `to_pm` and `to_prime_minister` for a Deputy's acting premiership), ten `to_deputy_pm` holder
observations of nine people placed after the existing string observation `to_cabinet_appointment` (which stays first) and
before the three existing holders (which are unchanged), a `to_deputy_pm` scope note, and coverage notes on `to_cabinet`,
`to_prime_minister` and the packet. It adds no organization, institution or role and changes no existing source, claim,
extract, holder or coverage entry. The parent scope (C01, C06, S23, WC1 and CP1) remains open.

The research was done in two parts (1990-2006 and 2006-2021) and then checked independently against the saved responses; every
checker defect is applied or declined with a reason (see [Checker defects](#checker-defects)).

## Outcome

| ID | Question | Decision |
|---|---|---|
| TO-DPM-01 | Who was Deputy Prime Minister on 1 January 1990, and when was the 1991 appointment made? | **Unresolved:** no 1990-1991 primary record names the office-holder; the PMO's retrospective "appointed ... in 1991" gives the year only; no holder |
| TO-DPM-02 | Langi Kavaliku, 1991-2000 | **Accepted in part:** attested 6 Nov 2000 (PMO index); retirement announced as due to commence 11 Nov 2000, not used as an end |
| TO-DPM-03 | Tevita Poasi Tupou, 2001 | **Accepted:** royal consent in Privy Council with effect from 24 Jan 2001; resignation accepted 28 Sep 2001; Clive Edwards acting from that day (claim only) |
| TO-DPM-04 | James Cecil Cocker, 2002-2006 | **Accepted in part:** attested 13 May 2002 to 8 Mar 2006, with the 2004 reshuffle confirmed with effect from 24 Aug 2004; appointment day and end unresolved; acting Deputies and his acting premiership are claims |
| TO-DPM-05 | Viliami Ta'u Tangi, 2006-2010 | **Accepted in part:** attested 8 Aug 2006 to 30 Dec 2010; "In May 2006" is retrospective only; no end |
| TO-DPM-06 | Samiu Kuita Vaipulu, 2011-2014 | **Accepted in part:** from the Cabinet's commencement on 4 Jan 2011; attested to 5 Sep 2014; no end; acting premierships are claims |
| TO-DPM-07 | Siaosi Sovaleni, 2014-2017, and his removal | **Accepted in part:** attested from the Cabinet list of 31 Dec 2014 (its effective day only forecast) to 30 Aug 2017; the revocation is stated only as recommended "effective from 1st September, 2017"; no start or end |
| TO-DPM-08 | Lord Ma'afu (2017) and Semisi Kioa Lafu Sika (2018-2019) | **Accepted in part:** Lord Ma'afu from 1 Sep 2017 (royal endorsement, conveyed 5 Sep); Sika from 5 Jan 2018, attested to 8 Nov 2018; no end for either |
| TO-DPM-09 | Sione Vuna Fa'otusia, 2019-2020 | **Accepted in part:** from 9 Oct 2019; sworn 28 Oct 2019; attested 16 Apr 2020; resigned "in 2020" (year only), so no end |
| TO-DPM-10 | Lord Ma'afu, 2020-2021 | **Accepted in part:** attested 21 Apr 2021; until his death on 12 Dec 2021 (stated); start known only as "mei Tisema 2020" |

The resulting `to_deputy_pm` holders, after the existing string observation `to_cabinet_appointment` (the Cabinet effective 31
December 2025), which stays first, and before the three existing holders:

| Holder | `attested_on` | `from` | `until` | Basis |
|---|---|---|---|---|
| Langi Kavaliku | 2000-11-06 | null | null | PMO "News in Brief" index row styling him Deputy Prime Minister |
| Tevita Poasi Tupou | null | 2001-01-24 | 2001-09-28 | PM's address: royal consent "with effect from 24th January, 2001"; Tongan release: resignation made and accepted 28 Sep 2001 |
| James Cecil Cocker | 2002-05-13 | null | null | PMO: Paunga acting "in lieu of Deputy Prime Minister, Hon. Cecil Cocker who is abroad", 13 May 2002; also 3 Sep 2002, 24 Aug 2004 (confirmation) and 8 Mar 2006 |
| Viliami Ta'u Tangi | 2006-08-08 | null | null | PMO release of 9 Aug 2006 styling him at the broadcast of the night before; also 14 Jan 2009, 24 Jun 2010 and 30 Dec 2010 |
| Samiu Kuita Vaipulu | null | 2011-01-04 | null | PMO media release: "The Cabinet Ministers commence today, Tuesday 4th. January, 2011"; also 3 Jan 2011 (report), 13 Feb 2012 and 5 Sep 2014 |
| Siaosi Sovaleni | 2014-12-31 | null | null | PMO media release's Cabinet list of 31 Dec 2014; also 12 Feb 2015, 4 Sep 2016 and 30 Aug 2017 |
| Lord Ma'afu | null | 2017-09-01 | null | PMO media release: endorsed "with the Royal Sign Manual ... with effect from 1st September, 2017"; letter received 5 Sep 2017 |
| Semisi Kioa Lafu Sika | null | 2018-01-05 | null | Tongan release: appointed with effect from 5 Jan 2018; also 15 Jun and 8 Nov 2018 |
| Sione Vuna Fa'otusia | null | 2019-10-09 | null | PMO media release: appointed "with effect from 9th October, 2019"; also 16 Apr 2020 |
| Lord Ma'afu | 2021-04-21 | null | 2021-12-12 | Assembly member page as captured; PMO: styled Deputy Prime Minister at his death on Sunday 12 Dec 2021 |
| Poasi Mataele Tei (existing) | null | 2021-12-28 | null | unchanged |
| Samiu Vaipulu (existing) | 2024-12-09 | null | null | unchanged |
| Taniela Likuohihifo Fusimalohi (existing) | null | 2025-01-28 | null | unchanged |

Five holders have a `from`, each the effective or commencement day a source states for a royal act or a Cabinet's
commencement; two have an `until`, Tupou's accepted resignation and Lord Ma'afu's death in office. No boundary comes from a
recommendation or a forecast (Sovaleni, 2014 and 2017), a successor's appointment, a retirement announcement, a year- or
month-only statement, an Assembly list, a stale government roster or a change of government. No acting Deputy Prime Minister,
and no Deputy acting as Prime Minister, is a holder of either office. Holder names are as printed in a cited claim, without
honorifics, as on the existing holders of this role; Lord Ma'afu's two deputy premierships are separate observations, and the
2011 Vaipulu observation is separate from the existing 2024 one.

### Date ledger

Each row is a separate dated fact with its own claim. A Prime Minister's recommendation, the report of an appointment, a royal
appointment or endorsement, its stated effective day, the day a Palace letter was received, an oath, a resignation and its
acceptance, a death and acting service are never merged, even where two fall on the same day. Claims of other packets named in
this ledger are existing records that this packet names in text only and does not cite.

| Date | Event | Claim or field |
|---|---|---|
| 1991 (year; retrospective) | Kavaliku "appointed the Deputy Prime Minister in 1991" | `to_dpm_kavaliku_appointed_1991_retro` (no structured date) |
| 6 Nov 2000 | PMO index row: "Dr. Langi Kavaliku, Deputy Prime Minister retires" | `to_dpm_kavaliku_listed_20001106`; Kavaliku `attested_on` |
| 11 Nov 2000 (prospective) | Retirement "due ... commencing on the 11th November 2000" | `to_dpm_kavaliku_retirement_announced_2000` (no structured date; not an end) |
| 24 Jan 2001 | Royal consent in Privy Council "with effect from 24th January, 2001": Tupou Deputy Prime Minister | `to_dpm_tupou_appointment_effective_20010124`; Tupou `from` |
| Jan 2001 (month; retrospective) | Appointed Deputy Prime Minister in January 2001; nine months in the office | `to_dpm_tupou_appointed_jan2001_retro` (no structured date) |
| 28 Sep 2001, 16:00 | Tupou resigns; the Princess Regent accepts | `to_dpm_tupou_resignation_accepted_20010928`; Tupou `until` |
| 28 Sep 2001 | Clive Edwards appointed Acting Deputy Prime Minister | `to_acting_dpm_edwards_appointed_20010928` (claim, not holder) |
| 11 Jan 2002 | Statement "By the Acting Deputy Prime Minister, Hon. William Clive Edwards" | `to_acting_dpm_edwards_20020111` (claim, not holder) |
| 13 May 2002 | Paunga Acting Deputy Prime Minister while Deputy Prime Minister Cocker is abroad | `to_acting_dpm_paunga_20020513` (claim), `to_dpm_cocker_abroad_20020513`; Cocker `attested_on` |
| 3 Sep 2002 | Statement by "JAMES CECIL COCKER DEPUTY PRIME MINISTER" at the WSSD | `to_dpm_cocker_wssd_statement_20020903` |
| 24 Aug 2004 (stated in the release of 9 Sep) | Reshuffle confirmed "with effect from 24th August, 2004": Cocker Deputy Prime Minister | `to_dpm_cocker_appointment_confirmed_20040824` (not a start) |
| 10 Apr 2005 | Acting Prime Minister Cocker's public notice | `to_acting_pm_cocker_20050410` (to_pm claim) |
| 8 Mar 2006 | Cocker introduced as Deputy Prime Minister at the concert | `to_dpm_cocker_styled_20060308` |
| May 2006 (month; retrospective) | Tangi "appointed ... as Deputy Prime Minister and Minister of Health" | `to_dpm_tangi_appointed_may2006_retro` (no structured date) |
| 9 Jun 2006 | "the Deputy Prime Minister and Minister of Health" (not named) | `to_dpm_office_with_health_20060609` (names nobody) |
| 8 Aug 2006 | Tangi styled Deputy Prime Minister at a broadcast "last night" (release of 9 Aug) | `to_dpm_tangi_styled_20060808`; Tangi `attested_on` |
| 2009 (undated) | Tangi styled Acting Prime Minister (Australia Day speech) | `to_acting_pm_tangi_australia_day_2009` (to_pm claim; no structured date) |
| 14 Jan 2009 | Tangi to represent the Government at a funeral as Deputy Prime Minister | `to_dpm_tangi_represents_government_20090114` |
| 13 Jun 2009 | Tangi named Acting Prime Minister | existing prime-minister claim `to_pmo_vaea_tribute_acting_pm_20090613` (named, not cited) |
| 24 Jun 2010 | The Deputy Prime Minister's reply to the Speech from the Throne | `to_dpm_tangi_throne_reply_20100624` |
| 22 Dec 2010 | Lord Tu'ivakano's appointment audience as Prime Minister | existing prime-minister claim `to_tuivakano_royal_appointment_20101222` (named, not cited; not an end) |
| 30 Dec 2010 | Life peerage to Tangi as "interim Health Minister and Deputy Prime Minister" | `to_dpm_tangi_styled_life_peer_20101230` |
| 3 Jan 2011 | Report that Lord Tu'ivakano "has appointed" Vaipulu Deputy Prime Minister | `to_dpm_vaipulu_appointment_reported_20110103` (not a start) |
| 4 Jan 2011 | "The Cabinet Ministers commence today": Vaipulu Deputy Prime Minister | `to_dpm_vaipulu_cabinet_commences_20110104`; Vaipulu `from` |
| 13 Feb 2012 | Speech by the Deputy Prime Minister | `to_dpm_vaipulu_styled_20120213` |
| 22 Apr 2013; 18 Jun 2014 | Vaipulu Acting Prime Minister | `to_acting_pm_vaipulu_20130422`, `to_acting_pm_vaipulu_20140618` (to_pm claims) |
| 5 Sep 2014 | "the Acting Prime Minister, Deputy Prime Minister and Minister for Environment" | `to_dpm_vaipulu_styled_20140905`, `to_acting_pm_vaipulu_20140905` (to_pm claim) |
| 29 Dec 2014 | "caretaker Deputy Prime Minister" Vaipulu nominated for Prime Minister | existing prime-minister claim `to_pm_2014_two_nominations_20141229` (named, not cited) |
| 31 Dec 2014 | Prime Minister-elect's recommendation; appointments to be "effective as of 31st December, 2014"; Cabinet list with Sovaleni Deputy Prime Minister | `to_dpm_sovaleni_appointment_recommended_20141231` (not a start), `to_dpm_sovaleni_cabinet_list_20141231`; Sovaleni `attested_on` |
| 19 Jan 2015 (IPU) | The Cabinet took office | existing claim `to_ipu_2014_cabinet_took_office_20150119` (named, not cited; conflicts) |
| 12 Feb 2015; 4 Sep 2016 | Sovaleni styled Deputy Prime Minister | `to_dpm_sovaleni_profile_20150212`, `to_dpm_sovaleni_styled_20160904` |
| 12-13 Nov 2015; 4 Apr 2017 | Sovaleni Acting Prime Minister | `to_acting_pm_sovaleni_20151112`, `to_acting_pm_sovaleni_20170404` (to_pm claims) |
| 24 Aug 2017 | Assembly dissolved | existing prime-minister claim `to_dissolution_instrument_20170824` (named, not cited) |
| 30 Aug 2017 | Sovaleni styled Deputy Prime Minister at a launch "yesterday evening" | `to_dpm_sovaleni_styled_20170830` |
| 1 Sep 2017 | Prime Minister recommends revoking Sovaleni's appointment "effective from 1st September, 2017" | `to_dpm_sovaleni_revocation_recommended_20170901` (not an end) |
| 1 Sep 2017 | Prime Minister recommends Lord Ma'afu; royal endorsement with effect from that day; Lord Ma'afu Acting Prime Minister | `to_dpm_maafu_appointment_recommended_20170901`, `to_dpm_maafu_royal_endorsement_effective_20170901` (Lord Ma'afu `from`), `to_acting_pm_maafu_20170901` (to_pm claim) |
| 5 Sep 2017 | Palace letter received by the PMO; the King's consent to Sovaleni's removal conveyed to him | `to_dpm_maafu_endorsement_conveyed_20170905`; existing prime-minister claim `to_pohiva_conveys_deputy_pm_removal_20170905` (named, not cited) |
| 5 Jan 2018 | Sika appointed Deputy Prime Minister with effect from that day | `to_dpm_sika_appointment_effective_20180105`; Sika `from` (the English release is CLAUDE-C01-08's `to_pohiva_appointment_repeated_20180122`, not cited) |
| 15 Jun 2018; 8 Nov 2018 | Sika styled Deputy Prime Minister | `to_dpm_sika_styled_20180615`, `to_dpm_sika_styled_20181108` |
| Jan 2018 - Sep 2019 (months; retrospective) | Assembly list: Sika "Tokoni Palemia (Sanuali 2018 – Sepitema 2019)" | `to_la_report_sika_dpm_jan2018_sep2019` (no structured date) |
| 14 and 16 Sep 2019 | Sika Acting Prime Minister | existing prime-minister claims `to_sika_acting_pm_20190914_programme`, `to_sika_acting_pm_20190916_gazette` (named, not cited) |
| 9 Oct 2019 | Ministers appointed "with effect from 9th October, 2019": Fa'otusia Deputy Prime Minister | `to_dpm_faotusia_appointment_effective_20191009`; Fa'otusia `from` |
| 28 Oct 2019 | Cabinet sworn in the Assembly | `to_dpm_faotusia_oath_news_20191028` (not a start) |
| 16 Apr 2020 | PMO clarification naming the Deputy Prime Minister, Fa'otusia | `to_dpm_faotusia_styled_20200416` |
| Oct 2019 - 2020; 2020 (retrospective) | Assembly list "('Okatopa 2019 – 2020)"; "until his resignation in 2020" | `to_la_report_faotusia_dpm_oct2019_2020`, `to_dpm_faotusia_resignation_2020_retro` (no structured date) |
| Dec 2020 (month; retrospective) | Assembly list: Lord Ma'afu "Tokoni Palemia (mei Tisema 2020)" | `to_la_report_maafu_dpm_from_dec2020` (no structured date) |
| 21 Apr 2021 (capture) | Assembly member page: Lord Ma'afu "Deputy Prime Minister" | `to_dpm_maafu_member_page_20210421`; Lord Ma'afu (2021) `attested_on` |
| 12 Dec 2021 | Lord Ma'afu dies in Auckland, styled Deputy Prime Minister in the PMO's release of 13 Dec | `to_dpm_maafu_death_20211212`, `to_dpm_maafu_styled_at_death_20211213` (no structured date); Lord Ma'afu (2021) `until` |
| 28 Dec 2021 | The Sovaleni Cabinet effective; Tei Deputy Prime Minister | existing `to_sovaleni_cabinet_effective_20211228` (unchanged) |

Date conventions follow CLAUDE-C01-08 and C01-24. `attested_on` is the date of the observed event or state as the source
dates it; a page's issue date is `published_date`. "Today", "yesterday", "last night", "this morning" and "last week" are read
from the item's own date line, and a day printed without a year takes the year of that date line; where the relative word
leaves the day open ("last week"), the item's date is kept and the claim says so. A retrospective statement or list that
gives only a year or month carries no structured date and is never a holder date. A capture date dates what a site presented,
not the day of any underlying event.

## Observations

### TO-DPM-01 — The Deputy Prime Minister on 1 January 1990

Evidence: the PMO release announcing Dr S. Langi Kavaliku's retirement in November 2000 says he "was appointed the Deputy
Prime Minister in 1991" (`to_dpm_kavaliku_appointed_1991_retro`).

Decision: unresolved; no holder. The statement is retrospective and gives the year only; it does not say who held the office
on 1 January 1990 or on what day Kavaliku was appointed.

Limits: the PMO's web site was first archived in April 2001, the AGO's gazette files archived on line begin in 2010, and no
1990-1991 record naming the Deputy Prime Minister was found. Encyclopaedia leads name Baron Vaea as the deputy until August
1991 and give Kavaliku a day in August 1991; they are leads only (see [Leads not imported](#leads-not-imported)).

### TO-DPM-02 — Langi Kavaliku, 1991-2000

Evidence: the PMO's "NEWS IN BRIEF" index lists, against "6/11/00", the release "Dr. Langi Kavaliku, Deputy Prime Minister
retires after thirty-four (34) years of Service" (`to_dpm_kavaliku_listed_20001106`). The release itself says that "Dr. S.
Langi Kavaliku" is "due for retirement, commencing on the 11th November 2000" (`to_dpm_kavaliku_retirement_announced_2000`).

Decision: accepted in part. The holder is event-dated to 6 November 2000, the only contemporaneous dated record of him in
office found. The retirement is a prospective announcement, recorded as a claim; its day is not used as an end.

Limits: no record of 1991-1999 naming him Deputy Prime Minister was found. The government's cabinet and Assembly pages of
2001-2002 still list "Hon. Dr. S. Langi Hu'akavameiliku" as Deputy Prime Minister after Tupou's appointment (the PMO's 2008
tribute, a lead, gives Kavaliku that title); they are stale and are not used (see [Leads not imported](#leads-not-imported)).
No record names a Deputy Prime Minister between 11 November 2000 and 24 January 2001.

### TO-DPM-03 — Tevita Poasi Tupou, 2001

Evidence: in his address to the nation on Wednesday 24 January 2001 the Prime Minister announced that the King "in - Privy
Council today" consented to ministerial appointments "with effect from 24th January, 2001", the second being "Hon. Tevita P.
Tupou" as Deputy Prime Minister, Attorney General and Minister of Justice (`to_dpm_tupou_appointment_effective_20010124`). The
government's Tongan release of 28 September 2001 says that at 4 p.m. that day Tevita Poasi Tupou resigned those offices and
that the Princess Regent accepted the resignation (`to_dpm_tupou_resignation_accepted_20010928`); it adds that he had been
appointed Deputy Prime Minister in January 2001 and had held the office for nine months (`to_dpm_tupou_appointed_jan2001_retro`),
and that the Princess Regent appointed the Minister of Police, Hon. Clive Edwards, Acting Deputy Prime Minister
(`to_acting_dpm_edwards_appointed_20010928`).

Decision: accepted. The holder runs from 24 January 2001, the stated effective day of the royal consent, until 28 September
2001, the day the resignation was made and accepted. Edwards's acting service is a claim only.

Limits: neither instrument was seen. The Princess Regent's act is recorded as printed; no regency is observed here (the Crown
packet owns regencies).

### TO-DPM-04 — James Cecil Cocker, 2002-2006

Evidence: a statement dated 11 January 2002 is issued "By the Acting Deputy Prime Minister, Hon. William Clive Edwards"
(`to_acting_dpm_edwards_20020111`). A PMO media statement says that Dr Giulio Paunga was appointed Acting Deputy Prime Minister
"in lieu of Deputy Prime Minister, Hon. Cecil Cocker who is abroad on official duties", with effect from 13 May 2002
(`to_dpm_cocker_abroad_20020513`, `to_acting_dpm_paunga_20020513`). A PMO-published statement headed as by "JAMES CECIL COCKER
DEPUTY PRIME MINISTER" is dated 3 September 2002 (`to_dpm_cocker_wssd_statement_20020903`). A release of 9 September 2004 says
that the King "has accordingly confirmed the following appointments, with effect from 24th August, 2004", Cocker as Deputy
Prime Minister among them (`to_dpm_cocker_appointment_confirmed_20040824`). On 10 April 2005 "the Acting Prime Minister, Hon.
James Cecil Cocker, issued a public notice" (`to_acting_pm_cocker_20050410`). A PMO item of 10 March 2006 has the concert
organisers introduced on 8 March to the Acting Prime Minister and to "the Deputy Prime Minister, Hon. James Cecil Cocker"
(`to_dpm_cocker_styled_20060308`).

Decision: accepted in part. The holder is event-dated to 13 May 2002, the earliest dated record of him in the office. The 2004
confirmation of a reshuffle, while he already held the office, is a supporting claim and not a start. The acting Deputies of
2001-2002 and his acting premiership are claims.

Limits: the day of his appointment (after 28 September 2001 and on or before 13 May 2002) was not found; the government's
portfolio pages suggest a reshuffle in early January 2002 but do not name the Deputy Prime Minister. His end is not stated;
the retrospective "In May 2006" appointment of Tangi is not used as an end.

### TO-DPM-05 — Viliami Ta'u Tangi, 2006-2010

Evidence: a PMO release of 9 June 2006 refers to "the Deputy Prime Minister and Minister of Health" without naming him
(`to_dpm_office_with_health_20060609`). The PMO release of 9 August 2006 says that the Prime Minister "and the Deputy Prime
Minister Dr. Viliami Tangi" made a plea in a television programme broadcast "last night" (`to_dpm_tangi_styled_20060808`); on 14
January 2009 "The Deputy Prime Minister, Dr the Hon Viliami Ta'u Tangi" is to represent the Government at a funeral
(`to_dpm_tangi_represents_government_20090114`); the Deputy Prime Minister's reply to the Speech from the Throne is dated 24
June 2010 (`to_dpm_tangi_throne_reply_20100624`); and on 30 December 2010 the King granted a life peerage "to Dr. Viliami Ta'u
Tangi, interim Health Minister and Deputy Prime Minister" (`to_dpm_tangi_styled_life_peer_20101230`). The same item says that
"In May 2006 His Majesty King Taufa'ahau Tupou IV appointed Dr. Tangi as Deputy Prime Minister and Minister of Health"
(`to_dpm_tangi_appointed_may2006_retro`). A PMO speeches page styles him Acting Prime Minister for Australia Day 2009, without a
day (`to_acting_pm_tangi_australia_day_2009`).

Decision: accepted in part. The holder is event-dated to 8 August 2006, the broadcast at which the first dated record names
him in the office; the June 2006 reference names nobody and the May 2006 statement is retrospective and month-only, so neither
is cited by the holder. His acting premiership is a claim on `to_pm`.

Limits: no appointment release of 2006 was found among the PMO's archived releases of February to June 2006. His end is not
stated: the 30 December 2010 styling falls after Lord Tu'ivakano's appointment audience and before the new Cabinet commenced,
and neither supplies an end. Whether "interim" also qualifies "Deputy Prime Minister" is not stated.

### TO-DPM-06 — Samiu Kuita Vaipulu, 2011-2014

Evidence: a government portal item of 3 January 2011 reports that Lord Tu'ivakano "has appointed the Hon. Samiu Kuita Vaipulu,
as his Deputy Prime Minister" (`to_dpm_vaipulu_appointment_reported_20110103`). The PMO media release of 4 January 2011 says
that the Prime Minister recommended the new ministers to King George Tupou V and that "The Cabinet Ministers commence today,
Tuesday 4th. January, 2011", listing Vaipulu as Deputy Prime Minister (`to_dpm_vaipulu_cabinet_commences_20110104`). He is
styled Deputy Prime Minister for a speech of 13 February 2012 (`to_dpm_vaipulu_styled_20120213`) and on 5 September 2014
(`to_dpm_vaipulu_styled_20140905`), and Acting Prime Minister on 22 April 2013, 18 June 2014 and 5 September 2014
(`to_acting_pm_vaipulu_20130422`, `to_acting_pm_vaipulu_20140618`, `to_acting_pm_vaipulu_20140905`).

Decision: accepted in part. The holder starts on 4 January 2011, the stated day the Cabinet ministers commenced; the report of
3 January is not a start. No end is stated. His acting premierships are claims on `to_pm`.

Limits: the royal instrument was not seen, and the ministers' oath of 14 January 2011 is not imported. CLAUDE-C01-08's record of
him as "caretaker Deputy Prime Minister" on 29 December 2014 is named in the ledger and not cited, and no end is inferred from
the Cabinet of 31 December 2014. This observation is separate from the existing 2024 observation of "Samiu Vaipulu".

### TO-DPM-07 — Siaosi Sovaleni, 2014-2017, and his removal

Evidence: the PMO media release of 31 December 2014 says that the Prime Minister-elect "has recommended to His Majesty King Tupou
VI" the new ministers, whose appointment "will be effective as of 31st December, 2014" (`to_dpm_sovaleni_appointment_recommended_20141231`),
and its Cabinet list names "Hon. Siaosi'Ofakivahafolau Sovaleni- Deputy Prime Minister" (`to_dpm_sovaleni_cabinet_list_20141231`).
He is styled Deputy Prime Minister on 12 February 2015, 4 September 2016 and 30 August 2017 (`to_dpm_sovaleni_profile_20150212`,
`to_dpm_sovaleni_styled_20160904`, `to_dpm_sovaleni_styled_20170830`) and Acting Prime Minister on 12-13 November 2015 and 4
April 2017 (`to_acting_pm_sovaleni_20151112`, `to_acting_pm_sovaleni_20170404`). The PMO media release of 6 September 2017 says
that on 1 September 2017 the Prime Minister recommended to the King, under clause 51(3)(a) of the Constitution, "to revoke the
appointments of Hon. Siaosi Sovaleni, Deputy Prime Minister" and of the Minister for Finance, "effective from 1st September,
2017" (`to_dpm_sovaleni_revocation_recommended_20170901`).

Decision: accepted in part. The holder is event-dated to 31 December 2014, the date of the Cabinet list; it has no `from`,
because the 2014 release states only a recommendation and the effective day it forecast, and no `until`, because the 2017
release states only the recommendation of the revocation and the effective day it proposed, while the royal endorsement it
reports covers the new appointments. CLAUDE-C01-08's MEIDECC claim that the King's consent to the removal was conveyed to him
on 5 September (`to_pohiva_conveys_deputy_pm_removal_20170905`) is reserved by that packet's test and is named, not cited.
Lord Ma'afu's start is not used as an end.

Limits: IPU's record that the Cabinet took office on 19 January 2015 conflicts with the PMO's forecast; IPU is corroboration
only. The item of 30 August 2017 falls after the dissolution of 24 August; its address reads "caretaker-dpm" but its text does
not, and no caretaker status is asserted. Neither instrument was seen.

### TO-DPM-08 — Lord Ma'afu (2017) and Semisi Kioa Lafu Sika (2018-2019)

Evidence: the release of 6 September 2017 says that the Prime Minister recommended Lord Ma'afu's appointment as Deputy Prime
Minister "with effect from 1st September, 2017" (`to_dpm_maafu_appointment_recommended_20170901`), that the King "has duly
endorsed with the Royal Sign Manual" that appointment with effect from that day (`to_dpm_maafu_royal_endorsement_effective_20170901`),
that the Lord Chamberlain's letter conveying the endorsement was received on 5 September 2017
(`to_dpm_maafu_endorsement_conveyed_20170905`), and that "Lord Ma'afu, Deputy Prime Minister shall be Acting Prime Minister"
from 1 September while the Prime Minister attended the Pacific Islands Forum (`to_acting_pm_maafu_20170901`). The Tongan
version of the release of 22 January 2018 says the King appointed the Cabinet ministers with effect from 5 January 2018, the
first being "Hon. Semisi Kioa Lafu Sika" as Deputy Prime Minister (`to_dpm_sika_appointment_effective_20180105`). Sika is styled
Deputy Prime Minister on 15 June and 8 November 2018 (`to_dpm_sika_styled_20180615`, `to_dpm_sika_styled_20181108`), and the
Assembly's report of 2017-2021 lists him "Tokoni Palemia (Sanuali 2018 – Sepitema 2019)" (`to_la_report_sika_dpm_jan2018_sep2019`).

Decision: accepted in part. Lord Ma'afu's holder starts on 1 September 2017, the stated effective day of the royal endorsement;
the recommendation and the day the letter arrived are separate claims. Sika's holder starts on 5 January 2018. Neither has an
end: the 2018 list names Lord Ma'afu Minister of Lands only, which is not stated as an end, and Sika's months are retrospective.

Limits: no later attestation of Lord Ma'afu's 2017-2018 deputy premiership was found. The English version of the 22 January
2018 release is CLAUDE-C01-08's `to_pohiva_appointment_repeated_20180122` and is not cited; the Tongan version is a separate
response. Sika's acting premiership of September 2019 is recorded by CLAUDE-C01-08 and not cited.

### TO-DPM-09 — Sione Vuna Fa'otusia, 2019-2020

Evidence: the PMO release of 10 October 2019 says the King "has appointed the New Ministers of Government, with effect from 9th
October, 2019", listing "Hon. Sione Vuna Fa'otusia" as Deputy Prime Minister and Minister for Justice and Prison
(`to_dpm_faotusia_appointment_effective_20191009`). The Assembly reports that the Prime Minister and Cabinet "were sworn in at the
Legislative Assembly today", 28 October 2019 (`to_dpm_faotusia_oath_news_20191028`). The PMO's Tongan release of 16 April 2020
names the Deputy Prime Minister, "Hon. Sione Vuna Fa'otusia" (`to_dpm_faotusia_styled_20200416`). The Assembly's condolence
item of 1 September 2021 says that "He also served as the Deputy Prime Minister of Tonga until his resignation in 2020"
(`to_dpm_faotusia_resignation_2020_retro`), and its 2017-2021 report lists him "Tokoni Palemia ('Okatopa 2019 – 2020)"
(`to_la_report_faotusia_dpm_oct2019_2020`).

Decision: accepted in part. The holder starts on 9 October 2019; the oath is not a start. No end is set: the resignation is
known only by year.

Limits: no PMO post of 5-17 December 2020 was archived, so no release on his resignation was found (see
[Sources attempted](#sources-attempted)).

### TO-DPM-10 — Lord Ma'afu, 2020-2021

Evidence: the Assembly's 2017-2021 report lists Lord Ma'afu "Tokoni Palemia (mei Tisema 2020)"
(`to_la_report_maafu_dpm_from_dec2020`). The Assembly's member page, as captured on 21 April 2021, gives his offices as "Deputy
Prime Minister" and Minister of Defence and of Lands (`to_dpm_maafu_member_page_20210421`). The PMO media release of 13 December
2021 announces the death of "the Lord Ma'afu", styling him Deputy Prime Minister among his offices
(`to_dpm_maafu_styled_at_death_20211213`), and says that he "passed away in the early hours of Sunday, 12 December 2021, at the
Auckland City Hospital" (`to_dpm_maafu_death_20211212`).

Decision: accepted in part. The holder is event-dated to the capture of 21 April 2021 and ends on 12 December 2021, the stated
day of his death in office. The December 2020 start is month-only and retrospective. The member page's portfolios match his
2019 Cabinet offices, not those of 2017, so its Deputy Prime Minister line is not a remnant of his first deputy premiership.

Limits: the appointment release of December 2020 was not found. Who, if anyone, held or acted in the office from 12 to 27
December 2021 is unresolved; the existing Tei observation from 28 December 2021 is unchanged.

## Sources added

| Source ID | What | Retained provenance |
|---|---|---|
| `to_pmo_2000_press_releases` | Government Press Release (PMO page of 2000-2001 releases, archived) | 53,628 bytes, `d31a98c9…9a739a`; capture 2001-12-30 |
| `to_pmo_news_in_brief_2001` | News in Brief - Government Press Release (PMO index, archived) | 31,877 bytes, `dd163157…eb04f4`; capture 2001-07-08 |
| `to_pmo_20010124_pm_address` | Translation of the Address delivered to the Nation by H.R.H. Prince 'Ulukalala Lavaka Ata, The Prime Minister, 24 January 2001 (archived) | 11,567 bytes, `d89cb254…ad4f6d`; capture 2001-04-30 |
| `to_pmo_20010928_tupou_resignation_to` | Ongoongo 'ae Pule'anga: Fakafisi 'ae 'Eiki Tokoni Palemia ... Hon. Tevita Poasi Tupou pea mo e Fakanofo 'o e 'Eiki Tokoni Palemia Le'oleo, Hon. Clive Edwards (Tongan release, 28 September 2001, archived) | 4,898 bytes, `958ebb0a…0aa653`; capture 2001-12-30 |
| `to_pmo_20020111_acting_dpm_statement` | Statement by the Acting Deputy Prime Minister, Hon. William Clive Edwards, on the detention of three vessels on the Tonga ship registry, 11 January 2002 (archived) | 22,721 bytes, `7d8c1fb1…a3167f`; capture 2002-02-10 |
| `to_pmo_20020514_acting_dpm_paunga` | Tonga Government Media Statement: Appointment of Dr. Giulio Masasso Paunga as Acting Deputy Prime Minister (May 2002, archived) | 11,566 bytes, `87e55949…a82e65`; capture 2002-08-16 |
| `to_pmo_20020903_wssd_statement` | Statement by the Honourable James Cecil Cocker, Deputy Prime Minister of the Kingdom of Tonga, at the World Summit for Sustainable Development, 3 September 2002 (archived) | 25,681 bytes, `a10dbe01…506bfa`; capture 2002-12-19 |
| `to_pmo_20040909_confirmation_of_appointments` | Government of Tonga Press Release: Reshuffling of Ministerial Portfolios in the Government of Tonga (9 September 2004, archived) | 19,494 bytes, `c8aa0ca0…4e3476`; capture 2004-11-05 |
| `to_pmo_20050411_pope_homage` | Tonga Pays Homage to the Pope, John Paul II (government media release, April 2005, archived) | 19,686 bytes, `76df4a30…34ab7d`; capture 2005-10-18 |
| `to_pmo_20060310_ub40_concert` | Royalty, Government and the People of Tonga welcomes the UB40 Band with immense applause (PMO item, 10 March 2006, archived) | 13,311 bytes, `7924a594…3a6f3d`; capture 2006-04-26 |
| `to_pmo_20060609_pohiva_statements` | Akilisi Should Stop Making Unjustified Statements (PMO press release, 9 June 2006, archived) | 14,317 bytes, `50a84552…5c3a7d`; capture 2006-06-15 |
| `to_pmo_20060809_common_sense` | Government Pleads For Common Sense (PMO press release, 9 August 2006, archived) | 12,953 bytes, `cf7cfbe2…f2d7b8`; capture 2006-08-22 |
| `to_pmo_20090114_hauofa_funeral` | Deputy Prime Minister to Represent Government at Professor 'Epeli Hau'ofa's Funeral in Suva (PMO, 14 January 2009, archived) | 19,408 bytes, `c5174de6…ca3ebe`; capture 2009-01-22 |
| `to_pmo_2009_australia_day_speech` | Australian Day 2009, Speech by Dr. The Hon. Viliami T. Tangi, Acting Prime Minister (PMO speeches page, archived) | 23,691 bytes, `d5f8460a…954638`; capture 2009-03-26 |
| `to_pmo_20100624_throne_reply` | Tali 'o e Folofola 'e he 'Eiki Tokoni Palemia, Hon. Dr. Viliami Tau Tangi (government speeches page, 24 June 2010, archived) | 36,849 bytes, `9b299f69…17d2ff`; capture 2011-11-30 |
| `to_mic_20101230_lord_tangi` | Lord Tangi of Vaonukonuka (government portal item, 30 December 2010, archived) | 35,793 bytes, `edab13cf…31ebfc`; capture 2014-01-02 |
| `to_mic_20110103_vaipulu_appointed` | Hon. Samiu Vaipulu - Deputy PM / Justice & Transport Minister (government portal item, 3 January 2011, archived) | 31,190 bytes, `69ef8960…48f9a4`; capture 2017-10-24 |
| `to_mic_20110104_cabinet_named` | Prime Minister Announces new Cabinet Ministers (media release, 4 January 2011, archived) | 30,867 bytes, `52173170…608289`; capture 2017-10-24 |
| `to_mic_20120213_foi_speech` | Speech by the Deputy Prime Minister Hon. Samiu Kuita Vaipulu at the Tonga National FOI Policy Consultation Workshop on Monday 13 February 2012 (government portal video page, archived) | 29,486 bytes, `ecb526ea…9a37e0`; capture 2017-09-06 |
| `to_mic_20130424_rti_speech` | Rights to Information Workshop: Speech by the Hon. Samiu Vaipulu (government portal, April 2013, archived) | 44,363 bytes, `c7af0f2a…eac770`; capture 2017-10-24 |
| `to_mic_20140619_half_mast` | National Flags to be flown at half-mast for HSH Prince Tu'ipelehake (government release, 19 June 2014, archived) | 33,061 bytes, `94f91e69…7d5a97`; capture 2020-06-11 |
| `to_mic_20140909_toloa` | Restoration of Toloa Rainforest (government release, 9 September 2014, archived) | 30,803 bytes, `b46af3a7…32a9a3`; capture 2017-01-27 |
| `to_pmo_20141231_new_cabinet` | Prime Minister announces new Cabinet (Government of Tonga media release, 31 December 2014, archived) | 47,371 bytes, `6ff556ad…9ceb2f`; capture 2015-10-02 |
| `to_mic_20150212_sovaleni_profile` | PROFILE: Hon. Siaosi Sovaleni (government portal, 12 February 2015, archived) | 32,128 bytes, `279eaf8c…c256f3`; capture 2017-10-24 |
| `to_mic_20151116_acting_pm_itu` | Acting Prime Minister Meets with Director for Telecommunication Development Bureau (BDT) of the ITU (government release, 16 November 2015, archived) | 35,533 bytes, `354e629b…df7a66`; capture 2015-12-06 |
| `to_mic_20160907_renewable` | Hon. Siaosi Sovaleni- Deputy Prime Minister and Minister for MEIDECC announced Tonga's commitment to a 100% Renewable Energy by 2035 (government release, 7 September 2016, archived) | 31,933 bytes, `71286caa…375f78`; capture 2016-09-16 |
| `to_mic_20170404_acting_pm_whales` | Tonga's Acting PM opens the first regional conference on whales (government release, 4 April 2017, archived) | 36,681 bytes, `22156a02…32eaa3`; capture 2017-04-07 |
| `to_mic_20170831_macbio` | DPM Officially launched the two MACBIO Reports for Tonga (government release, 31 August 2017, archived) | 33,077 bytes, `2d78ccd0…f76d98`; capture 2017-09-03 |
| `to_pmo_20170906_ministerial_appointments` | Ministerial Appointments (Government of Tonga media release, 6 September 2017, archived) | 41,468 bytes, `72b063fb…26b323`; capture 2017-09-10 |
| `to_mic_20180122_cabinet_to` | Fakanofo 'e he 'Ene 'Afio 'a e 'Eiki Palēmiá mo e Hou'eiki Minisitā 'o e Kapineti 'a 'Ene 'Afió (Tongan version of the government release, 22 January 2018, archived) | 34,581 bytes, `f97d7a8f…c7c30c`; capture 2018-10-30 |
| `to_mic_20180615_dpm_gita` | Tongan Deputy PM surveys Cyclone Gita recovery with ADF (government release, 15 June 2018, archived) | 30,732 bytes, `934102be…90f5a8`; capture 2018-10-30 |
| `to_la_news_20181108_auditor` | 'Auditor general independent to carry out audit work' according to Lord Speaker (Assembly news, 8 November 2018, archived) | 40,794 bytes, `5ecaa853…0dd878`; capture 2019-07-23 |
| `to_la_term_report_2017_2021` | Fakamatala 'o e Ngāue 'a e Fale Alea 'o Tongá, To'u Fale Alea Nōvema 2017 - Sepitema 2021 (Assembly report, archived PDF) | 15,526,342 bytes, `37d85723…a76f87`; capture 2021-11-24; PDF pages 206, 208, 209 viewed |
| `to_pmo_20191010_new_cabinet` | 'Prime Minister Announces New Cabinet Ministers' (Government of Tonga media release, 10 October 2019, archived) | 48,927 bytes, `81d5b448…b71bf7`; capture 2019-10-11 |
| `to_la_news_20191028_oath` | Prime Minister & Cabinet Members took oath in Parliament (Assembly news, 28 October 2019, archived) | 37,376 bytes, `19e943fd…c9f4a1`; capture 2019-11-22 |
| `to_pmo_20200416_dpm_clarification` | Ongoongo Tukuatu: 'Fakama'ala'ala e ongoongo fekau'aki mo e Tokoni Palēmia' (PMO release image, 16 April 2020, archived) | 88,720 bytes, `ab551fc7…e59331`; capture 2023-08-02; release image viewed |
| `to_la_news_20210901_faotusia` | The Lord Speaker and the Members of the Legislative Assembly are deeply saddened by the passing of the late Hon Sione Vuna Fa'otusia (Assembly news, 1 September 2021, archived) | 41,553 bytes, `ec4ae5f7…604f5c`; capture 2021-09-02 |
| `to_la_member_maafu_2021` | Lord Ma'afu (Legislative Assembly members' page, archived) | 40,129 bytes, `b9c6399c…ca7136`; capture 2021-04-21 |
| `to_pmo_20211213_maafu_death` | Media Release (13th December, 2021): 'Public Statement-Office of the Prime Minister' (PMO release image, archived) | 258,072 bytes, `3adfa677…18978a`; capture 2025-04-15; release image viewed |

All 39 sources are raw Internet Archive captures (`id_` form) of original `pmo.gov.to`, `www.pmo.gov.to`, `mic.gov.to`,
`www.mic.gov.to` and `parliament.gov.to` responses, all made before the cutoff: 36 HTML pages, two PMO release images (one PNG,
one JPEG) and one Assembly PDF. Each records the capture URL as `url` and the original address, without the archive's `:80`, as
`original_url`, and each extract records the capture time. Each new source has a checked-in derived factual extract under
[sources/](sources/) (39 in all) in the `spheres-c01-derived-factual-table/v1` format, with rows keyed by `claim_id` that repeat
the packet's claims exactly and add `observation_id` (`to_cabinet`, or `to_prime_minister` for a Deputy's acting premiership),
`review_observation` (TO-DPM-01 to 10), `role_id`, `holder_name` (null where the source names nobody), `role_title` and
`event_kind`. Each extract's own checksum is in the packet, separate from the original-response identity. Original pages,
images and the PDF are not checked in; no seal, coat of arms, signature or photograph is republished. The two release images
were viewed at full size and the PDF's three cited pages were rendered and checked; HTML pages were read as text.

Source types: `primary_government_release_archived` (PMO, Chief Secretary's and government-portal releases),
`primary_government_reference_list_archived` (the PMO's release index), `primary_government_address_archived` (the Prime
Minister's address, speech pages and the WSSD statement), `primary_government_office_profile_archived` (a ministerial profile),
`primary_government_public_announcement_image_archived` (the PMO release images), `primary_legislature_news_notice_archived`,
`primary_legislature_member_profile_archived` and `primary_legislature_report_archived` (Assembly pages and report).

## Identities and stability checks

Every recorded response was downloaded twice for this packet, at least 35 minutes apart, and each pair matched in byte count and
SHA-256:

- first round: 2026-09-29T03:32:09Z to 2026-09-29T03:40:41Z (UTC);
- second round: 2026-09-29T04:13:11Z to 2026-09-29T04:15:53Z (UTC).

The research downloads of the same day, two per source at least 30 minutes apart, had already recorded the same identities:

- research downloads: 2026-09-29T01:27:56Z to 2026-09-29T03:35:46Z (UTC).

Each extract's `stability_check` gives its download times.

- Internet Archive captures were fetched with `Accept-Encoding: identity` and curl's default user agent. None was returned
  gzip-encoded, so every recorded identity is the body as received.
- No source is a page generated per request, a search or listing page, or a live page. The Assembly's and the government
  portal's pages carry hit counters and "Who's Online" blocks; only pre-cutoff captures are used, and the counters and sidebars
  frozen in them are not used as facts. The two release images are captures of static uploads.
- Tongan text is quoted as printed (straight apostrophes for the glottal stop; macrons where the source prints them); English
  renderings are this packet's reading.

## Leads not imported

- Encyclopaedias: [wikipedia.org](https://en.wikipedia.org/wiki/Langi_Kavaliku) (Kavaliku Deputy Prime Minister from 22 August
  1991, Baron Vaea deputy until then), and the articles on Lord Ma'afu Tuku'i'aulahi (Deputy Prime Minister 16 December 2020 -
  12 December 2021) and Sione Vuna Fa'otusia (resignation in December 2020). Leads only; no primary record found gives those
  days.
- Stale government rosters: the PMO's `executive_in_government.htm` and `legislative_assembly.htm` captures. The 30 April 2001
  cabinet page (6,939 bytes, `d2c9821d…71094d`) and the Assembly page of 18 August 2002 (12,654 bytes, `c007510c…8cffca`) still
  list "Hon. Dr. S. Langi Hu'akavameiliku" as Deputy Prime Minister after Tupou's appointment; the cabinet pages of 24 August
  2001 and 18 August 2002 (6,890 bytes, `d70d7580…1ac7f5`; 6,862 bytes, `cc9657a4…ee45f8`) still list Tupou after his
  resignation. Later revisions listing Cocker (for example the cabinet page of 8 April 2003, 6,856 bytes, `db7ad76b…cfd2dd`)
  repeat what the imported releases state. The Assembly-page captures of 8 July 2003 and 12 October 2004 have the same bytes as
  CLAUDE-C01-24's `to_pmo_legislative_assembly_page_20030408` and `to_pmo_legislative_assembly_page_20041226` and are not
  duplicated.
- PMO `gpr19july.html` (captured 17 April 2002; 12,618 bytes, `95d7176a…e8732e`) and its index `PR2001.htm`: Tupou styled Deputy
  Prime Minister on 19 July 2001; inside his stated interval, and the page prints no date of its own.
- Tonga Law Reports 2004 ([download=1562:2004_tlr](https://ago.gov.to/cms/ago-materials/publications/tonga-law-reports.html?download=1562:2004_tlr);
  2,119,807 bytes, `ff354bfb…dfeb20`): Tupou v Saulala says the plaintiff was Deputy Prime Minister "When the writs were issued
  in April 2001"; month only, inside the stated interval.
- The PMO's tribute to Hu'akavameiliku of December 2008 (`356-a-tribute-to-dr-the-hon-huakavameiliku...`; 18,969 bytes,
  `79ba6971…9d8b6b`): mentions his "Deputy Prime Ministership" without dates.
- Tongan versions and copies of imported releases: the Tongan address of 24 January 2001 (`pmspeech.html`), the Tongan
  confirmation of 2004 (printing Cocker as "Semisi Sesolo Koka"), the Tongan Hau'ofa item (dated 2008 as printed), the Tongan
  PMO item of 12 June 2006 (`article_123`), the PMO Tongan and government-portal copies of the 6 September 2017 release
  (`fakanofo-o-e-houeiki-minisita`, `6895-ministerial-appointments`, `6897-fakanofo`) and the Tongan image of 13 December 2021
  (`ongoongo-tukuatu.jpg`).
- Further acting service not imported: Edwards as Acting Deputy Prime Minister on 6 December 2002 (`gpr6dec02`) and 18 February
  2004 (`Announcement_of_the sad occasion`); Cocker as Acting Prime Minister in May 2002 and on 21 March 2005 (a prospective
  item); Vaipulu as Acting Prime Minister in October-November 2013 and August 2014. Acting arrangements were not surveyed
  exhaustively.
- Assembly item `538-pohiva-vaipulu-are-2-nominations-of-pm` (captured 30 December 2014; 40,967 bytes, `9de302d2…daf130`): the
  same article as CLAUDE-C01-08's `to_la_20141229_nominations` (item 293), which already records "caretaker Deputy Prime
  Minister" Vaipulu; not duplicated.
- The government portal's "Deputy Prime Minister of Tonga" who's-who page (`4051-deputy-prime-minister-of-tonga-...`):
  captures of 22 April 2013 (Vaipulu; 33,256 bytes, `8557949a…4a114f`), 24 October 2017 (still Sovaleni; 29,979 bytes,
  `5bf3e4da…f4fae6`), 15 February 2018 and 15 August 2020 (Sika, still listed after Fa'otusia's appointment; 27,917 bytes,
  `27574421…37a24b`). Undated and stale; not used.
- Further attestations not imported, to keep the packet bounded: PMO items of 16 August 2006, 17 December 2009, 1 April 2010
  and 20 October 2010 (the last naming no one), the PMO cabinet page `cabinet.html` of 2010, the government portal's profile
  of Vaipulu (18 January 2011) and items of 19 January 2011, 9 March 2011, 20 January 2012, 11 October 2012 and July 2014, the
  PMO's shorter item `prime-minister-announced-new-cabinet` of 31 December 2014 and its 2015 cabinet page, items of 29 June 2015,
  22 June 2017 and 26 June 2018, and the Assembly's 2016 member page for Sovaleni.
- The Assembly's item of 14 October 2019 on the new Cabinet (`king-appoints-new-pm-and-cabinet-ministers-effective-last-week`)
  is a lead that CLAUDE-C01-08's accepted test keeps out of the packet; the PMO release of 10 October 2019 is imported instead.
  The Assembly's minutes of 14 January 2011 (a `miniti-fika` download link) print the ministers' oaths without the Deputy
  Prime Minister's title and are not imported.
- A PMO release of 2 March 2018 accepting Lord Ma'afu's resignation from the Lands and Armed Forces portfolios (a government
  portal repost), and the Assembly's item of 7 January 2021 calling Sika "former Deputy Prime Minister": context and
  retrospective statements only.

## Sources attempted

- **Gazette.** The AGO's gazettes-by-year page (`gazettes-by-year.html?view=gazettes_by_year`) selects a year by a POST form,
  which was not submitted. Internet Archive listings of `ago.gov.to/cms/images/LEGISLATION/GAZETTES/` hold no gazette for
  1990-2009 and few for 2011-2019; no gazette notice of a Deputy Prime Minister's appointment, revocation or resignation was
  found.
- **Assembly minutes behind a download form.** The current Assembly minutes are served only by POSTing a Phoca Download form
  with `license_agree=1` and a session token; as in CLAUDE-C01-08 and C01-24, the form was not submitted. Only minutes that the
  archive captured as plain downloads in 2012 were seen.
- **PacLII** (`paclii.org`) was not queried for this packet; earlier Tonga packets recorded that its live pages answer with a
  Cloudflare challenge, which is not bypassed.
- **PMO posts of December 2020.** The archive holds no PMO post between 5 and 17 December 2020 (`pmo.gov.to/index.php/2020/12/`
  lists only posts of 4 and 18 December), and no government-portal copy was found, so the releases on Fa'otusia's resignation
  and Lord Ma'afu's appointment were not found.
- **1990-1991.** The PMO's web site was first archived in April 2001, and no earlier government record naming the Deputy Prime
  Minister was found on line; IPU's 1999 summary is a lead that CLAUDE-C01-08's test keeps out of the packet, and IPU is
  corroboration only.
- **Live sites.** The live `pmo.gov.to` answers curl with a JavaScript redirect; it was not used. The Internet Archive refused
  connections, answered HTTP 429 and 503, or timed out for periods on 29 September (UTC); every recorded download was retried
  after pacing, and nothing depends on a failed request.

## Checker defects

An independent check compared every claim, quotation, date, locator, gloss, event kind and holder decision with the saved
responses (HTML read as text, the two release images viewed, the PDF's three pages rendered). It confirmed that the quotations
appear in their sources as printed (apart from D10), that the capture timestamps match the archive headers, that the base
packet's 168 sources and the three existing holders are unchanged, and that the eight acting-premiership claims sit only on
`to_pm`. It raised fourteen defects:

| # | Defect | Outcome |
|---|---|---|
| D1 | The second download round had not been run, so every `stability_check` was pending | **Applied**: the second round ran 35 minutes or more after the first; all 39 identities matched and each extract records both times |
| D2 | Sovaleni's `from` (31 December 2014) rested on a recommendation and a forecast ("will be effective"), inconsistent with refusing the 2017 recommendation as an end | **Applied**: the claim is split into the recommendation (`to_dpm_sovaleni_appointment_recommended_20141231`, cited by no holder) and the Cabinet list (`to_dpm_sovaleni_cabinet_list_20141231`); the holder has no `from` and is attested 2014-12-31 |
| D3 | The styling in the death notice was dated 13 December 2021 as an in-office observation, after the death | **Applied**: `to_dpm_maafu_styled_at_death_20211213` carries no structured date and the event kind `styled_at_death` |
| D4 | Many claims quoted whole sentences or paragraphs | **Applied**: 33 claims rewritten to paraphrase with short quotations of the load-bearing words |
| D5 | Locator of Edwards's 2001 acting appointment | **Applied**: paragraphs 6 and 10 |
| D6 | Locator of Sika's November 2018 styling; the remarks were "last week" | **Applied**: paragraph 6; the uncertainty says the remarks are dated only as last week and the item's date is kept |
| D7 | Locator of the unnamed Deputy Prime Minister of June 2006 | **Applied**: paragraph 5 |
| D8 | Tangi's 2006 styling dated by the release although the broadcast was "last night", unlike the other relative dates | **Applied**: dated to the broadcast, 8 August 2006 (`to_dpm_tangi_styled_20060808`); the holder is attested 2006-08-08 |
| D9 | The Kavaliku holder note gave a noble title no imported source states | **Applied**: removed; the report attributes the title to the PMO's 2008 tribute, a lead |
| D10 | An en dash and inserted slashes inside quotations | **Applied**: quotations split at the list breaks and started after the en dash |
| D11 | The concert item's scope note said CLAUDE-C01-08 records Sevele's acting premiership from that item | **Applied**: "recorded by CLAUDE-C01-08 from other sources" |
| D12 | "Sovaleni from 12 November 2015" in the prime-minister coverage note read as a start | **Applied**: "on 12-13 November 2015" |
| D13 | The access date (28 September) differs from the UTC download date (29 September) | **Declined**: the house convention (CLAUDE-C01-24) records the local access date and gives the UTC times in each `stability_check` |
| D14 | Scope-note details about companion pages could not be checked from the saved responses | **Declined in part**: the PMO post of 16 April 2020, the shorter 2014 item, the 2017 copies and the Tongan Hau'ofa item were downloaded in the research and are listed under [Leads not imported](#leads-not-imported); the 2020 scope note now names the post capture |

On the holder questions it put, the check agreed with keeping Kavaliku's and Sovaleni's `until` null, with Tupou's two stated
boundaries, with dating Cocker to 13 May 2002 and not starting him from the 2004 confirmation, with Lord Ma'afu's 2017 start,
and with the 2021 member-page attestation (its portfolios match the 2019 Cabinet) and the end at his death; it found no holder
that should be split or merged.

## Suggested next work orders

These are proposals for the integrator. They are not created in `work-orders.json`.

- `C01-Tonga-DPM-002`: 1990-1991 — a Gazette, Privy Council record or Tonga Chronicle notice naming the Deputy Prime Minister on
  1 January 1990 and dating Kavaliku's 1991 appointment and his 2000 retirement (National Archives or a library holding).
- `C01-Tonga-DPM-003`: 2001-2006 — the day Cocker was appointed (between 28 September 2001 and 13 May 2002) and the end of his
  office; the instrument appointing Tangi in May 2006.
- `C01-Tonga-DPM-004`: the royal instruments or gazette notices of 2011-2019, including the effective day of Sovaleni's
  revocation as enacted and any end of Lord Ma'afu's 2017-2018 and Sika's 2018-2019 deputy premierships.
- `C01-Tonga-DPM-005`: December 2020 and December 2021 — Fa'otusia's resignation, Lord Ma'afu's second appointment, and who held
  or acted in the office from 12 to 27 December 2021.
- `C01-Tonga-DPM-006`: a survey of acting Deputy Prime Ministers and of Deputies acting as Prime Minister, 1990-2026.

## Integration notes (outside this packet's file boundary)

- **Base and stack:** claim commit `e8821f40` on base `44098c5a`, the head of `codex/campaign-certification` when the packet was
  claimed; not stacked on a pending packet.
- `research-index.json` is regenerated in a **separate commit**. New totals: 1,610 sources and 4,235 claims across 9 country
  packets (previously 1,571 and 4,182), 841 organization and 35 institution observations, 93 open discovery batches. Tonga keeps
  nine entries, 15 role observations and one open batch, `C01-Tonga-DISC-B001`. `research-index.json` is the only file shared
  with other pending packets.
- `research/README.md`, the C01 README totals and `docs/planning/ai-workstreams.json` are left for the integrator; this handoff
  is self-proposed and not registered there.
- Pinned tests, none loosened and no assertion removed:
  - `test_tonga_research_s10g.py`: totals 207 sources and 319 claims (from 168 and 266), with the ownership comment.
  - `test_tonga_speakers_c01_24.py`: its check that CLAUDE-C01-24's 49 sources are the last in the file becomes the same check
    on their fixed position, `sources[119:168]`, and its check that its packet note is the last coverage note becomes the same
    check at its fixed position, `unresolved[13]`, because this packet's sources and note follow them.
  - `test_tonga_transition_c01_03.py`: its exact `to_deputy_pm` holder list gains this packet's ten names ahead of Tei, Vaipulu
    and Fusimalohi, unchanged.
  - `test_tonga_transition_c01_02.py`: its mutation of "the first dict Deputy Prime Minister holder" now selects Tei by name,
    since this packet's holders come first; the expected error is unchanged.
  - `test_tonga_pm_1990_2019_c01_08.py`: its guard that no holder anywhere names Sika, Tangi or Kavaliku (the acting Prime
    Ministers of its packet) now exempts only this packet's exact `to_deputy_pm` observations of those three substantive
    Deputy Prime Ministers, and asserts that they are the only holders anywhere carrying those names; the acting and
    never-holder claims it lists still feed no holder.
  - `test_tonga_dpfi_c01_04.py`: its exact sets of stated starts and ends gain this packet's five starts and two ends; its guard
    that no holder names Sika exempts only the `to_deputy_pm` observation "Semisi Kioa Lafu Sika" and asserts that it is the
    only one.
- Existing text is not edited: no existing source, claim, extract, holder or coverage entry changes; new coverage entries are
  appended, and the `to_deputy_pm` role gains a `scope_note`, as `to_king`, `to_pm` and `to_speaker` have. Claims of other
  packets that bear on the office (CLAUDE-C01-08's September 2017 letter, January 2018 release, 2014 nomination, acting
  premierships of 2009 and 2019, and IPU's 2015 Cabinet date) are named in the ledger and notes but not cited.
- New fields: none in `tonga.json`. The extract rows use the `rows` layout of the later C01 packets; the two image extracts add
  `facsimile_pages_one_based` to `visual_review`, as CLAUDE-C01-03's image extract does. No UI code changed and no browser
  review was run.
- **Known failures outside this packet (disclosed, not fixed):**
  - `tools/avatars/campaign_census.py --check` fails on the integration base itself: integration commit `262d5f61` changed
    `spheres-sim/src/government.rs` (848,551 to 849,546 bytes; SHA-256 `0e2dbdef…a2b73f` to `3f846b4b…a8d202`) without
    regenerating `docs/campaign-certification/C01/census.json`. Regenerating into a scratch directory shows that this is the only
    difference: `census.json`'s `all_declared_inputs_current`, `stale_inputs` and the `government.rs` input entry; the other four
    census files are byte-identical. This packet does not touch the census.
  - `tools/avatars/test_certified_gap_ledger.py` errors on this packet's new sources ("no pinned attribution") until Codex
    classifies the packet's commit in `COMMIT_PACKETS` at integration.
  - `tools/avatars/test_certified_boundary_matrix.py` (S23) needs `spheres-web/src`, which this sparse checkout lacks; in a full
    checkout it would report the packet as `unclassified_packet` with stale boundary-matrix files, so Codex must list the packet
    and regenerate `docs/campaign-certification/S23/preparation/boundary-matrix/` on integration.

## Checks

```text
Run on 29 September 2026 (UTC; 28 September local) from the worktree root with PYTHONDONTWRITEBYTECODE=1; the sparse
checkout includes spheres-sim/src, spheres-sim/data and spheres-web/data (not widened).

python -X utf8 tools/avatars/campaign_research.py            # regenerate
python -X utf8 tools/avatars/campaign_research.py --check    # pass: 9 packets, 841 organization and 35 institution
                                                             # observations, 1,610 sources, 4,235 claims, 93 batches
python -X utf8 tools/avatars/campaign_census.py --check      # FAILS (exit 1) on the integration base itself:
                                                             # government.rs changed at 262d5f61, census.json not
                                                             # regenerated (see Integration notes); not touched here
python -X utf8 -m unittest discover -s tools/avatars -p "test_tonga*.py"      # 89 pass (10 new)
python -X utf8 -m unittest discover -s tools/avatars -p "test_*research*.py"  # 79 pass (overlaps the Tonga run)
python -X utf8 -m unittest discover -s tools/avatars -p "test_campaign*.py"   # 16 pass, census tests included
node --test tools/ui/check_leadership_research_review.cjs                     # 11 pass
python tools/planning/workboard.py --check                                    # pass (44 markers)
git diff --check -- <this packet's paths>                                     # clean

Known failures outside the listed checks (disclosed, not fixed):
test_certified_gap_ledger.py      # error: "Source to_pmo_2000_press_releases (Tonga) has no pinned attribution"
test_certified_boundary_matrix.py # error: "Required input is missing: spheres-web/src/person_avatar_assets.rs"
```
