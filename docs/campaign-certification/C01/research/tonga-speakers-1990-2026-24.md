# Tonga Speakers 24: the Speaker of the Legislative Assembly, 1990-2026

Packet: **CLAUDE-C01-24**. State: **ready_for_review** (not complete).
Owner: Claude. Integrator/reviewer: Codex. Branch `claude/c01-to-24`; claim commit `a91a8249` on base `ffe54b02`
(then the head of `codex/campaign-certification`); current integration `a33a8987` was merged into the branch at
`f1230bbb` before this packet was completed, and the fetch of 27 September 2026 found nothing newer. Research
access: 25-26 September 2026 (research and independent checks) and 27 September 2026 (local; 28 September UTC), when
every recorded response was downloaded again for this packet. The historical cutoff stays **7 September 2026**.

This packet reviews ten observations about the existing Speaker role (`to_speaker`) of the existing
`to_legislative_assembly` institution in [tonga.json](tonga.json), from 1 January 1990 to the cutoff: the holder when the
period opens, the royal appointments before the 2010 reform, the reform's procedure, and the Speaker of each Assembly
elected in 2010, 2014, 2017, 2021 and 2025. It adds 49 sources and 82 claims, eleven `to_speaker` holder observations
placed after the existing string observation `to_speakers_appointment` (which stays first and unchanged), a
`to_speaker` scope note, and coverage notes on `to_legislative_assembly` and on the packet. It adds no organization,
institution or role, does not extend `to_deputy_speaker`, and changes no other Tonga holder or existing extract. The
parent scope (C01, C06, S23, WC1 and CP1) remains open.

The research was done in three parts (1990-2010; the 2010 procedure and 2010-2014; 2014-2026), each with an independent
adversarial check. Every checker defect is applied, declined with a reason, or superseded by a source change (see
[Checker defects](#checker-defects)).

## Outcome

| ID | Question | Decision |
|---|---|---|
| TO-SPK-01 | Who was Speaker on 1 January 1990, and when was he appointed? | **Unresolved:** only the Assembly's retrospective lists name Speakers for 1987-1990, and they conflict with its own 2006 list and 2014 news item; no holder |
| TO-SPK-02 | Royal appointments of the 1990s | **Accepted in part:** Fusitu'a attested by court records in 1996; Veikune attested in 2001; the Acting Speakers of 1995 and 2000 are claims only; no appointment is dated |
| TO-SPK-03 | Appointments from 2000 to 2010 | **Accepted in part:** Tu'ivakano attested from 2003 (appointment date contested); Veikune appointed 22 Mar 2005, ended 25 Jan 2006 (stated); Tu'iha'angana claims only; Tu'ilakepa attested 2008 and 2010 |
| TO-SPK-04 | The 2010 reform's procedure for the Speaker | **Accepted:** procedure only (Acts 20 and 21 of 2010; the Interim Speaker's note); no Speaker date comes from it |
| TO-SPK-05 | The Speaker of the first Assembly after the 2010 election | **Accepted in part:** Lord Lasike, selected 21 Dec 2010 and attested from 14 Jan 2011; the royal appointment day is unresolved |
| TO-SPK-06 | The Speakers of 2012-2014 | **Accepted in part:** Lasike's appointment revoked effective 17 Jul 2012 (stated end); Lord Fakafanua elected and appointed 19 Jul 2012; acting service claims only; no end for Fakafanua |
| TO-SPK-07 | The Speaker after the 2014 election | **Accepted in part:** Lord Tu'ivakano attested 24 Feb 2015; no appointment or oath dated |
| TO-SPK-08 | The Speaker after the 2017 election | **Accepted in part:** Lord Fakafanua sworn 18 Jan 2018 and attested 5 Mar 2018; no appointment dated |
| TO-SPK-09 | The Speaker after the 2021 election | **Accepted in part:** Lord Fakafanua elected on or before 15 Dec 2021 and attested 17 Feb 2022; acting service on 11 Jan 2022 a claim only; no appointment dated |
| TO-SPK-10 | The Speaker after the 2025 election, and an official attestation before the cutoff | **Accepted:** Lord Vaea elected (item of 16 Dec 2025), appointed effective 18 Dec 2025 (existing string observation), sworn 22 Jan 2026 and attested 19 May 2026 |

The resulting `to_speaker` holders, after the existing string observation `to_speakers_appointment` (Lord Vaea's
appointment effective 18 December 2025), which stays first:

| Holder | `attested_on` | `from` | `until` | Basis |
|---|---|---|---|---|
| Fusitu'a | 1996-11-14 | null | null | court records: the Speaker's warrant of 20 Sep 1996; "at all material times the Speaker" on 14 Nov 1996 |
| Hon. Veikune | 2001-04-30 | null | null | government Assembly page presenting him as Speaker (also its 14 Dec 2001 revision) |
| Hon. Tu'ivakano | 2003-04-08 | null | null | government Assembly page (8 Apr 2003); Assembly home page (6 Sep 2004); government page (19 Sep 2004) |
| Hon. Veikune | 2005-03-22 | null | 2006-01-25 | Assembly: appointed by the King on 22 Mar 2005; PMO: position ended 25 Jan 2006, confirmed for the Palace 3 Feb 2006 |
| Lord Tu'ilakepa | 2008-06-02 | null | null | his signed reply to the 2008 Speech from the Throne; presiding on 29 Jun 2010 |
| Lord Lasike | 2011-01-14 | null | 2012-07-17 | administering members' oaths; revocation "effective immediately", letter of 17 Jul 2012 |
| Lord Fakafanua | 2012-07-19 | null | null | Assembly: "officially APPOINTED ... last Thursday, July 19th" |
| Lord Tu'ivakano | 2015-02-24 | null | null | Assembly news styling him Lord Speaker |
| Lord Fakafanua | 2018-03-05 | null | null | Assembly news: the Speaker addressing the first sitting at the Tonga National Centre |
| Lord Fakafanua | 2022-02-17 | null | null | Assembly news styling him Speaker |
| Lord Vaea | 2026-05-19 | null | null | Assembly press release: delegation to China led by the Speaker; the latest attestation before the cutoff |

Only two holders have an end, each a day the source states: Veikune's position "ended on 25th January, 2006", and Lasike's
appointment was revoked "effective immediately" by a letter of 17 July 2012. No holder has a `from`: no source states the
day a Speaker's office took effect, except the 2025 appointment, which the existing string observation already records and
which is not duplicated. No end comes from a successor's appointment, a conviction, an expected successor, a retrospective
span, a description as "former Speaker" or a dissolution. No Acting Speaker, Interim Speaker or Deputy Speaker presiding is
a holder, and Tu'iha'angana (2006-2008) has claims but no holder (see TO-SPK-03).

### Date ledger

Each row is a separate dated fact with its own claim. A royal appointment, the Assembly's election or recommendation of its
Speaker, the report to the King, an oath, a resignation, a revocation and acting or interim service are never merged, even
where two fall on the same day. Claims of other packets named in this ledger are existing records that this packet names in
text only and does not cite.

| Date | Event | Claim or field |
|---|---|---|
| 1987-1989; 1990-1998 (years) | Assembly's 2011 lists: Malupo; Fusitu'a | `to_la_list_malupo_1987_1989_20111130`, `to_la_list_fusitua_1990_1998_20111130` (observed 2011-11-30) |
| 1995-1998 (years) | Assembly's 2006 list: Fusitu'a (nothing for 1979-1994) | `to_la_archives_fusitua_1995_1998_20060327` (observed 2006-03-27) |
| 1991 (year) | Assembly news of 2014: Fusitu'a appointed; "three terms from 1991-1998" | `to_la_obituary_fusitua_appointed_1991_20140505` (observed 2014-05-05) |
| 2 Aug 1995 | Acting Speaker "the Honourable Lasike" presiding (court record) | `to_acting_speaker_lasike_19950802` (claim, not holder) |
| 20 Sep 1996 | The Speaker's committal warrant signed "Fusitu'a / Chairman of the Legislative Assembly" | `to_speaker_warrant_fusitua_19960920` |
| 14 Nov 1996 | Interview of the Speaker; "at all material times the Speaker" | `to_speaker_fusitua_interview_19961114`, `to_speaker_fusitua_material_times_19961114`; Fusitu'a `attested_on` |
| 2000 (year) | Hon. Malupo "Acting Speaker ... in 2000" | `to_la_ministers_page_malupo_acting_2000` (no structured date; claim, not holder) |
| 30 Apr 2001; 14 Dec 2001 | Government page lists "Hon. Veikune (Speaker of the House)" | `to_speaker_veikune_listed_20010430`, `to_speaker_veikune_listed_20011214`; Veikune (first) `attested_on` |
| about 23 May 2002 | Tu'ivakano "embraced his appointment last week" | `to_speaker_tuivakano_appointment_reported_20020523` (date from file name and metadata) |
| 1 Jul 2002 (retrospective) | Assembly: "appointed Speaker by His Majesty on 1st July 2002" | `to_la_ministers_page_tuivakano_appointed_20020701`, `to_la_profile_tuivakano_appointed_20020701` (never a holder date) |
| 8 Apr 2003; 6 Sep 2004; 26 Dec 2004 | "Hon. Tu'ivakano (Speaker of the House)" | `to_speaker_tuivakano_listed_20030408`, `..._20040906`, `..._20041226`; Tu'ivakano `attested_on` |
| 22 Mar 2005 | Veikune appointed by King Taufa'ahau Tupou IV (Assembly page) | `to_speaker_veikune_royal_appointment_20050322`; Veikune (second) `attested_on` |
| 23 Mar 2005 | IPU: Speaker Veikune issues the by-election order | `to_ipu_2005_speaker_veikune_order_20050323` (corroboration) |
| 25 Jan 2006 | Veikune's position as Speaker ended (stated) | `to_speaker_veikune_end_20060125`; Veikune (second) `until` |
| 3 Feb 2006 | For the Palace, the Acting Private Secretary confirms the end | `to_palace_confirms_veikune_speaker_end_20060203` |
| 9 Feb 2006 | "A new Speaker of the House is expected to be appointed soon" | `to_speaker_successor_expected_20060209` (not a vacancy date) |
| 10 Feb 2006 (IPU only) | Tu'iha'angana's appointment "effective from 10 February 2006" | `to_ipu_2005_tuihaangana_appointment_effective_20060210` (not a holder) |
| 11 Sep 2006 | "Speaker Tu'iha'angana" among the proclaimers of Gazette No. 19 | existing Crown claim `to_crown_gtv_devolution_proclamation_20060911` (named, not cited) |
| 13 Dec 2007 | Land Court recites Veikune's removal as Speaker (no day) | `to_speaker_veikune_removal_recited_20071213` (retrospective) |
| 2 May 2008 (IPU only) | Tu'ilakepa appointed | existing Crown claim `to_ipu_2008_gtv_speaker_20080502` (named, not cited) |
| 2 Jun 2008 | Reply signed "Tu'ilakepa / Sea 'o e Fale Alea" (refers to the opening of 29 May 2008) | `to_speaker_tuilakepa_reply_signed_20080602`; Tu'ilakepa `attested_on` |
| 29 Jun 2010 | Tu'ilakepa presides; petition addressed to him as Speaker | `to_speaker_tuilakepa_presides_20100629` |
| 24 Nov 2010 (printed) | Assent printed on Acts 20 and 21 of 2010 | `to_act20_2010_royal_assent_printed`, `to_la_act21_2010_royal_assent_printed` (no structured date) |
| 3 Dec 2010 | Interim Speaker Lord Tevita Tupou's press conference | `to_interim_speaker_tupou_styled_20101203` (claim, not holder) |
| 21 Dec 2010 | Assembly selects Lord Lasike unopposed; royal appointment to follow | `to_lasike_assembly_selection_unopposed_20101221`, `to_lasike_royal_appointment_announced_20101221` |
| undated (Dec 2010 or Jan 2011) | Speaker Lord Lasike announces the first sitting; oaths "starting with the Speakers" | `to_lasike_announces_first_sitting_2011` (no structured date) |
| 14 Jan 2011 | Lasike administers the members' oaths | `to_lasike_speaker_administers_oaths_20110114`; Lasike `attested_on` |
| 4 Apr 2011 | Lasike styled Speaker | `to_lasike_speaker_styled_20110404` |
| 9 Jul 2012 | Lasike convicted; his seat lost from that date | `to_lasike_supreme_court_conviction_20120709`, `to_lasike_ceased_elected_representative_20120709` (not the end) |
| 17 Jul 2012 | Lasike's appointment revoked "effective immediately" | `to_lasike_speaker_appointment_revoked_20120717`; Lasike `until` |
| 19 Jul 2012 | Assembly elects Fakafanua 17-8; Tu'iha'ateiho acting during the proceedings | `to_fakafanua_assembly_election_20120719`, `to_tuihaateiho_acting_speaker_20120719` |
| 19 Jul 2012, 17:23 | Assembly's Tongan item: recommended; the appointment "toki" follows | `to_fakafanua_assembly_recommendation_20120719`, `to_fakafanua_royal_appointment_pending_20120719` |
| 19 Jul 2012 (stated 23 Jul) | "officially APPOINTED ... last Thursday, July 19th" | `to_fakafanua_royal_appointment_20120719`; Fakafanua (2012) `attested_on` |
| 20 Jul 2012 | Appointment reported as done; adjournment to inform the King (undated) | `to_fakafanua_royal_appointment_reported_20120720`, `to_fakafanua_adjourned_to_inform_king_2012` |
| 23 Jul 2012 | Fakafanua starts presiding "this morning" | `to_fakafanua_starts_as_speaker_20120723` (not a start) |
| 27-28 Aug 2013 | Tu'iha'ateiho announces a resignation, resumes as Acting Speaker; the Speaker abroad | `to_tuihaateiho_resignation_announced_20130827`, `to_tuihaateiho_resumed_acting_speaker_20130828`, `to_tuihaateiho_acting_speaker_20130828`, `to_fakafanua_speaker_abroad_20130828` |
| 4 Sep 2013 | Acting Speaker Tu'iha'ateiho on the quorum | `to_tuihaateiho_acting_speaker_20130904` |
| 14 Aug 2014 | Speaker Fakafanua in the constitutional debate | `to_fakafanua_speaker_20140814` |
| 29 Dec 2014 | House elects Lord Tu'ivakano Speaker | existing prime-minister claim `to_pohiva_declared_pm_elect_pmo_20141229` (named, not cited) |
| 24 Feb 2015 | Lord Speaker Tu'ivakano styled | `to_speaker_tuivakano_styled_20150224`; Tu'ivakano (2015) `attested_on` |
| 12 Sep 2017 | The Speaker, Lord Tu'ivakano, issues a release | existing prime-minister claim `to_pohiva_styled_pm_20170912` (named, not cited) |
| 17 Nov 2017 | Interim Speaker Lord Tangi appointed | existing prime-minister claim `to_interim_speaker_tangi_20171117` (named, not cited) |
| 18 Dec 2017 | Fakafanua and Tu'ilakepa elected Speaker and Deputy Speaker | existing prime-minister claim `to_pohiva_assembly_reselection_20171218` (named, not cited) |
| 18 Jan 2018 | "the Lord Speaker Fakafanua" sworn in | `to_speaker_fakafanua_sworn_news_20180118` (not a start) |
| 5 Mar 2018 | The Speaker addresses the first sitting at the Tonga National Centre | `to_speaker_fakafanua_styled_20180305`; Fakafanua (2018) `attested_on` |
| 12 Sep 2019 (sitting) | Acting Speaker Lord Tu'ilakepa adjourns the House | existing prime-minister claim `to_la_20190912_pm_prayers_adjourned` (named, not cited) |
| 20 Nov 2021 | Interim Speaker Lord Tangi appointed | existing 2021-transition claim `to_interim_speaker_tangi_20211120` (named, not cited) |
| on or before 15 Dec 2021 | Assembly elects Fakafanua and Tu'iha'angana ("Yesterday") | `to_speaker_fakafanua_assembly_election_20211215` (item date only) |
| 11 Jan 2022 | Acting Speaker Tu'iha'angana at the opening | `to_acting_speaker_tuihaangana_20220111` (claim, not holder) |
| 17 Feb 2022 | Speaker Fakafanua styled | `to_speaker_fakafanua_styled_20220217`; Fakafanua (2022) `attested_on` |
| 9-10 Dec 2024; 31 Jan 2025 | Speaker Lord Fakafanua addressed, issuing invitations, countersigning | existing 2024-transition claims `to_palace_acceptance_letter_20241209`, `to_pm_nominations_invited_20241210`, `to_eke_oath_20250131` (named, not cited) |
| 12 Dec 2025 | Notice: Speaker's election at the meeting of Monday 15 Dec 2025 | `to_speaker_election_meeting_notice_20251212` (prospective) |
| 16 Dec 2025 (item) | Assembly elects Lord Vaea at the Special Sitting presided over by Interim Speaker Lord Tangi | `to_speaker_vaea_assembly_election_20251216`, `to_interim_speaker_tangi_presided_20251216` |
| 18 Dec 2025 | Royal appointment effective (English and Tongan releases of 19 Dec) | existing `to_speakers_appointment` (string holder); `to_speaker_vaea_royal_appointment_to_20251218` |
| 22 Jan 2026 | The Lord Speaker and members sworn in | `to_speaker_vaea_oath_news_20260122` (not a start) |
| 19 May 2026 | Delegation to China led by the Speaker, Lord Vaea | `to_speaker_vaea_led_delegation_20260519`; Vaea `attested_on` |
| 18 Aug 2026 | Acting Speaker Tu'iha'angana welcomes three ministers | `to_acting_speaker_tuihaangana_20260818` (claim, not holder) |

Date conventions follow CLAUDE-C01-07 and C01-08. `attested_on` is the date of the observed event or state as the source
dates it; a page's issue date is `published_date`. "Today", "yesterday" and "last Thursday" in a dated item are read from
that item's own date; where the item's date and a relative word leave the day uncertain (15 December 2021) the claim keeps
the item's date and says so. A retrospective statement that gives a day keeps that day as `attested_on` and is labelled
retrospective (as C01-08 did for a retrospective timeline date); one that gives only years is dated by its observation. A
capture date dates what a site presented, not necessarily the day of the underlying event.

## Observations

### TO-SPK-01 — The Speaker on 1 January 1990

Evidence: the Assembly's retrospective "Speakers of the House" list, as captured in February and November 2011, gives
"Nopele Malupo 1987 - 1989" and "Nopele Fusitu'a 1990 - 1998" (`to_la_list_malupo_1987_1989_20111130`,
`to_la_list_fusitua_1990_1998_20111130`, `to_la_list_tuihaangana_2006_2007_20110203`). Its "Archives" page of 2005-2006
jumps from "Hon. Ma'atu Tukui'aulahi 1959-1978" to "Hon. Fusitu'a 1995-1998" (`to_la_archives_fusitua_1995_1998_20060327`),
and its news item of 5 May 2014 says Fusitu'a was appointed "in 1991" and "served for three terms from 1991-1998"
(`to_la_obituary_fusitua_appointed_1991_20140505`). The Assembly's 2005 ministers page says Hon. Malupo "began his
political career in 1993" (`to_la_ministers_page_malupo_acting_2000`).

Decision: unresolved; no holder. The lists conflict with each other and with the 2014 item, and the ministers page suggests
that "Nopele Malupo 1987 - 1989" may name a title rather than the man who held it in the 1990s.

Limits: no 1989-1990 record naming the Speaker was found. The AGO gazette archive starts in 2010, the AGO law reports on
line start in 1996, and PacLII's live pages answered with a bot challenge that was not attempted (see
[Sources attempted](#sources-attempted)).

### TO-SPK-02 — Royal appointments in the 1990s

Evidence: the Court of Appeal in Edwards v Pohiva quotes the committal warrant of 20 September 1996, which "The Speaker of
the Legislative Assembly issued", signed "Fusitu'a / Chairman of the Legislative Assembly" (`to_speaker_warrant_fusitua_19960920`).
The Court of Appeal in 'Akau'ola v Attorney General says the journalist "interviewed the Speaker of the Legislative
Assembly" on 14 November 1996 and quotes the article "In a statement by Fusitu'a [i.e. the Speaker]"
(`to_speaker_fusitua_interview_19961114`), and the Supreme Court found that "Fusitu'a was at all material times the Speaker
of the Supreme Law making Body of Tonga" (`to_speaker_fusitua_material_times_19961114`). In Pohiva v Lasike the Supreme
Court records that on 2nd August 1995 "the Acting Speaker, the Honourable Lasike" presided
(`to_acting_speaker_lasike_19950802`). The government's page on the Assembly lists "Hon. Veikune (Speaker of the House)"
under "1999-2002 Legislative Assembly Membership" when captured on 30 April 2001 and in its revision of 14 December 2001
(`to_speaker_veikune_listed_20010430`, `to_speaker_veikune_listed_20011214`). The Assembly's ministers page says Hon.
Malupo was "Acting Speaker of the Legislative Assembly in 2000" (`to_la_ministers_page_malupo_acting_2000`), and its 2005
Speaker page says Veikune "was the Speaker from 1999 to 2002" (`to_speaker_veikune_prior_term_retro_20070625`); the lists
give 1998-2001 and 1999-2001.

Decision: accepted in part. Fusitu'a is event-dated to 14 November 1996, the day of the court's explicit finding; the
warrant of 20 September is an earlier attestation cited by the same holder. Veikune's first term is event-dated to 30
April 2001. The Acting Speakers of 1995 and 2000 are claims only.

Limits: no appointment instrument, day or end was found for either man; the courts name Fusitu'a only by his noble title.
The government roster could lag: a capture of 18 August 2002, after Tu'ivakano's appointment had been reported, still
marks Veikune as Speaker (a lead, listed below). The 1996 volume's report of Moala v Minister of Police places the 1995
incident "in October 1995" (a lead); the appellate date is kept and the conflict recorded. No identity is asserted between
"the Honourable Lasike" of 1995 and the Lord Lasike of 2010-2012.

### TO-SPK-03 — Appointments from 2000 to 2010

Evidence: a government media statement, dated about 23 May 2002 by its file name and embedded metadata, says that the Hon.
Tu'ivakano "embraced his appointment last week as Speaker ... from 2002 to 2004" and "succeeds the Hon. Veikune"
(`to_speaker_tuivakano_appointment_reported_20020523`); the Assembly's 2005 ministers page and 2011 profile say he "was
appointed Speaker by His Majesty on 1st July 2002" (`to_la_ministers_page_tuivakano_appointed_20020701`,
`to_la_profile_tuivakano_appointed_20020701`). The government page's revision of 8 April 2003, the Assembly's home page
captured on 6 September 2004 and the government page's revision of 19 September 2004 list "Hon. Tu'ivakano (Speaker of the
House)" (`to_speaker_tuivakano_listed_20030408`, `..._20040906`, `..._20041226`). The Assembly's Speaker page says Hon.
Veikune "was appointed the Speaker of the Legislative Assembly by His Majesty King Taufa'ahau Tupou IV on the 22nd of March
2005, to replace Hon. Tu'ivakano" (`to_speaker_veikune_royal_appointment_20050322`); IPU has him issue a by-election order
on 23 March 2005 (`to_ipu_2005_speaker_veikune_order_20050323`). The PMO release of 9 February 2006 states that "his position
as the Speaker of the House ended on 25th January, 2006", as the Acting Private Secretary to His Majesty confirmed on 3
February (`to_speaker_veikune_end_20060125`, `to_palace_confirms_veikune_speaker_end_20060203`), and that "A new Speaker of
the House is expected to be appointed soon" (`to_speaker_successor_expected_20060209`). IPU says the King consented to Mr.
Tu'iha'angana's appointment "(effective from 10 February 2006)" (`to_ipu_2005_tuihaangana_appointment_effective_20060210`);
the Assembly's lists give him 2006-2007 and 2006-2008. The Speaker's reply to the Princess Regent's Speech from the Throne,
dated 2 June 2008, is signed "Tu'ilakepa / Sea 'o e Fale Alea" (`to_speaker_tuilakepa_reply_signed_20080602`), and the
government-published transcript of 29 June 2010 has "'Eiki Nopele Tu'ilakepa" presiding and a petition addressed to "Lord
Tu'ilakepa, 'Eiki Sea 'o e Fale Alea" (`to_speaker_tuilakepa_presides_20100629`); his 2010 profile says he was appointed
Speaker in 2008 (`to_la_profile_tuilakepa_appointed_2008_20100915`). The Land Court in 2007 recited that Veikune "has been
removed as the Speaker" (`to_speaker_veikune_removal_recited_20071213`).

Decision: accepted in part. Tu'ivakano is event-dated to 8 April 2003, the earliest attestation after the two conflicting
appointment dates, neither of which is used. Veikune's second term is event-dated to the stated appointment day, 22 March
2005, and ends on the stated day, 25 January 2006. Tu'ilakepa is event-dated to 2 June 2008. Tu'iha'angana has no holder.

Limits: no royal instrument or effective date was found for any of these appointments. Tu'ivakano's end is not stated (he
became Minister of Works; Veikune's appointment "to replace" him is not an end). "Last week" may attach to the appointment or
to its acceptance. The only contemporaneous primary attestation of Tu'iha'angana found is the Crown packet's reproduction of
Gazette No. 19 of 11 September 2006, which names "Speaker Tu'iha'angana" among the proclaimers; CLAUDE-C01-07's accepted test
reserves that claim to the Crown entry and forbids any other role's holder from citing it, so it is named here and not
cited, and no Tu'iha'angana holder rests on IPU or the lists alone. Tu'ilakepa's appointment day (2 May 2008) is IPU-only and
is held by the Crown packet; his end is not stated. The PMO release numbers between 9 and 13 February 2006, where an
appointment notice would be expected, were not archived.

### TO-SPK-04 — The 2010 reform's procedure (procedure only)

Evidence: Act No. 20 of 2010 replaces clause 61 of the Constitution: within 5 days after a post-election Prime Minister is
appointed, the King appoints one of the elected representatives of the nobles "on the recommendation of the Legislative
Assembly" as Speaker; the Speaker stays in office until an Interim Speaker is appointed after the next election, until
revocation, or until death, resignation or revocation after ceasing to be an elected nobles' representative; and a vacancy
is filled within 7 days (`to_act20_2010_clause61_speaker_appointment_procedure`,
`to_act20_2010_clause61_speaker_tenure_and_vacancy`). Its Schedule creates an Interim Speaker, "not a candidate", who holds
office "until a Speaker is next appointed under clause 61" (`to_act20_2010_schedule_interim_speaker_procedure`). Act No. 21
of 2010 repeats the appointment rule in section 15 of the Legislative Assembly Act and has the Deputy Speaker preside when
the Speaker does not (`to_la_act21_2010_section15_speaker_procedure`, `to_la_act21_2010_section16_deputy_presides_procedure`).
Both print the assent "GEORGE TUPOU V, 24th November 2010" (`to_act20_2010_royal_assent_printed`,
`to_la_act21_2010_royal_assent_printed`). The Assembly's note of December 2010 styles Lord Tevita Tupou Interim Speaker and
says the interim office ceases when a Speaker is appointed (`to_interim_speaker_tupou_styled_20101203`,
`to_interim_speaker_office_ceases_rule_2010`).

Decision: accepted as procedure only. No Speaker's selection, appointment, start or end is dated from a statute. The assent
lines are kept in the claim text without a structured date; they agree with the existing `to_reform_assent` and
`to_reform_passage`.

Limits: neither Act prints a commencement provision, and no consolidation after 2010 was checked, so the procedure is not
read as the text in force at any later date. The existing `to_reform_act_2010` (a different AGO copy with a blank assent
block) is not edited.

### TO-SPK-05 — The Speaker of the first Assembly after the 2010 election

Evidence: the Assembly reported at 15:05 on 21 December 2010 that "Lord Lasike has become the 17 th Speaker ... after he was
nominated unopposed this afternoon" and that he "will beformally appointed by the King" [sic]
(`to_lasike_assembly_selection_unopposed_20101221`, `to_lasike_royal_appointment_announced_20101221`). An undated Assembly
item says the first session of 13 January 2011 "was announced by the Speaker of the House, Lord Lasike", with oaths "starting
with the Speakers" (`to_lasike_announces_first_sitting_2011`). On 14 January 2011 "The Speaker of the Legislative Assembly,
Lord Lasike administered the oaths of office" to the members (`to_lasike_speaker_administers_oaths_20110114`); he is styled
Speaker on 4 April 2011 (`to_lasike_speaker_styled_20110404`) and told the Supreme Court "that he is the Speaker"
(`to_lasike_testimony_speaker_cr285`). The Assembly's 2011 list gives "Lord Lasike 2010" (`to_la_list_lasike_2010_20111130`).

Decision: accepted in part. The holder is event-dated to 14 January 2011, the first dated record of him acting as Speaker.
The selection and the announced appointment are claims, not a start.

Limits: the day and instrument of the royal appointment were not found; clause 61(1) would put it within 5 days of the
Prime Minister's appointment, but a statute is never used to date it. No Speaker's oath was found, though one was planned.

### TO-SPK-06 — The Speakers of 2012-2014

Evidence: the Supreme Court convicted "Noble Lasike" on 9 July 2012 (`to_lasike_supreme_court_conviction_20120709`). The
Office of the Legislative Assembly's release of 17 July 2012 states that King Tupou VI "under Clause 61(2) (c) of the
Constitution has revoked Lord Lasike's appointment as the Speaker ... effective immediately", conveyed by a letter of 17 July
2012; in its own voice it adds that under clause 23 "the decision was effective from the date of his conviction and he is no
longer an elected representative" (`to_lasike_speaker_appointment_revoked_20120717`,
`to_lasike_ceased_elected_representative_20120709`). MIC reports that on 19 July 2012 the Assembly elected Lord Fakafanua
17-8 over Lord Tu'ilakepa, with Lord Tu'iha'ateiho "appointed as Acting Speaker during the proceedings"
(`to_fakafanua_assembly_election_20120719`, `to_tuihaateiho_acting_speaker_20120719`). The Assembly's own Tongan item of
17:23 that day records the recommendation and that the appointment "'oku toki fakahoko" by the King
(`to_fakafanua_assembly_recommendation_20120719`, `to_fakafanua_royal_appointment_pending_20120719`). Its item of 20 July
reports that the King "has appointed" him and that the House adjourned "to inform His Majesty ... on the outcome of the
ballot" (`to_fakafanua_royal_appointment_reported_20120720`, `to_fakafanua_adjourned_to_inform_king_2012`); its item of 23
July says he "was officially APPOINTED ... last Thursday, July 19th" and "started in his new role ... this morning"
(`to_fakafanua_royal_appointment_20120719`, `to_fakafanua_starts_as_speaker_20120723`). In August-September 2013 Lord
Tu'iha'ateiho is Acting Speaker while "Speaker Lord Fakafanua" is abroad; he announces a resignation on 27 August and
"resumed duty as the Acting Speaker after resignation" (`to_tuihaateiho_acting_speaker_20130828`,
`to_tuihaateiho_resignation_announced_20130827`, `to_tuihaateiho_resumed_acting_speaker_20130828`,
`to_fakafanua_speaker_abroad_20130828`, `to_tuihaateiho_acting_speaker_20130904`). The Speaker is attested on 14 August 2014
(`to_fakafanua_speaker_20140814`).

Decision: accepted in part. Lasike's `until` is 17 July 2012, the revocation stated as effective immediately. The Fakafanua
holder is event-dated to 19 July 2012, the appointment day the Assembly states, with no `from`. All acting service is
claims only.

Limits: the revocation instrument was not seen. The 23 July dating of the appointment is in tension with the Tongan item's
"toki" at 17:23 on 19 July; both hold only if the appointment followed that evening, and 20 July (reported as done) is the
conservative alternative, recorded in the holder's uncertainty. The resignation's post is not named in the source. When the
2012 Speakership ended is not stated.

### TO-SPK-07 — The Speaker after the 2014 election

Evidence: the Assembly's news item of 24 February 2015 styles "Lord Speaker of the Legislative Assembly of Tonga, Lord
Tu'ivakano" (`to_speaker_tuivakano_styled_20150224`). The House's election of him on 29 December 2014 and his styling as
Speaker on 12 September 2017 are existing claims of the prime-minister packet (see the ledger).

Decision: accepted in part: holder event-dated to 24 February 2015, without start or end.

Limits: no royal appointment or oath for this Assembly is dated by any source retrieved. The Assembly's own item of 29
December 2014, already a source of the prime-minister packet, reports the 15-11 Speaker vote in a sentence its extract does
not import; importing it would edit an existing extract, which this packet does not do.

### TO-SPK-08 — The Speaker after the 2017 election

Evidence: on 18 January 2018 "MEMBERS of Parliament including the Lord Speaker Fakafanua ... were sworn into office today"
(`to_speaker_fakafanua_sworn_news_20180118`). On 5 March 2018 "THE Speaker of the Legislative Assembly", Lord Fakafanua,
addressed the first sitting at the Tonga National Centre (`to_speaker_fakafanua_styled_20180305`). The Assembly's election of
18 December 2017 and the minutes of his oath are existing claims of the prime-minister packet.

Decision: accepted in part: holder event-dated to 5 March 2018, separate from the oath and without start or end.

Limits: no royal appointment for this Assembly is dated. No continuity with, or gap after, the 2012-2014 holder is asserted.

### TO-SPK-09 — The Speaker after the 2021 election

Evidence: the Assembly's item created on 15 December 2021 says "Yesterday Parliament elected Lord Fakafanua and Lord
Tu'iha'angana as the Speaker and Deputy Speaker, respectively" (`to_speaker_fakafanua_assembly_election_20211215`). At the
opening on 11 January 2022 "the Acting Speaker Lord Tu'iha'angana" named the reply committee
(`to_acting_speaker_tuihaangana_20220111`). On 17 February 2022 "The Speaker of the Legislative Assembly of Tonga, Lord
Fakafanua" is styled (`to_speaker_fakafanua_styled_20220217`). Claims of the 2024-2025 transition packet style him Speaker
in December 2024 and January 2025.

Decision: accepted in part: holder event-dated to 17 February 2022; the election and acting service are claims.

Limits: the day of the Speaker's election is not fixed ("Yesterday" in an item of 15 December, while the Interim Speaker's
notice scheduled it for the 15 December meeting), and no royal appointment for this Assembly is dated. The minutes of 13
January and 2 June 2022 (the Speaker's apology and his oath) were not used (see Sources attempted).

### TO-SPK-10 — The Speaker after the 2025 election, and an attestation before the cutoff

Evidence: the Office of the Interim Speaker announced on 12 December 2025 a meeting on Monday 15 December at which "The
election of the Speaker and Deputy Speaker ... will be held" (`to_speaker_election_meeting_notice_20251212`). The Assembly's
item of 16 December 2025 reports that at the Special Sitting "presided over by Interim Speaker Lord Tangi, Parliament elected
... Lord Vaea as Speaker of Parliament" (`to_speaker_vaea_assembly_election_20251216`,
`to_interim_speaker_tangi_presided_20251216`). The Chief Clerk's Tongan release of 19 December 2025 says the King appointed
"'Eiki Nōpele Vaea" Speaker with instruments effective 18 December 2025 (`to_speaker_vaea_royal_appointment_to_20251218`),
as the existing English release already records (`to_speakers_appointment`). On 22 January 2026 "The Lord Speaker and all
Members ... were sworn in today" (`to_speaker_vaea_oath_news_20260122`). The Assembly's press release of 19 May 2026 says the delegation to China "was led by
the Speaker of the Legislative Assembly of Tonga, Lord Vaea" (`to_speaker_vaea_led_delegation_20260519`). On 18 August 2026
"The Acting Speaker of Parliament, Lord Tu'iha'angana" welcomed three new ministers (`to_acting_speaker_tuihaangana_20260818`).

Decision: accepted. The appointment remains the existing string observation, which stays first and unchanged and is not
duplicated; the Vaea holder is event-dated to 19 May 2026, the latest official attestation found before the cutoff.

Limits: the day of the Special Sitting is not printed (scheduled for 15 December); the election, appointment, oath and
acting service stay separate claims. The minutes of 22 January, 13 August and 18 August 2026 were not used (see Sources
attempted); their facts are covered by the Assembly's news items.

## Sources added

| Source ID | What | Retained provenance |
|---|---|---|
| `to_tlr_1997` | Tonga Law Reports 1997 ([download](https://ago.gov.to/cms/ago-materials/publications/tonga-law-reports.html?download=1572:1997_tlr)): Pohiva v Lasike; 'Akau'ola v AG; AG v Fusitu'a | 4,010,541 bytes, `7a4726f0…42e71c`; PDF pages 27, 30, 135, 136 viewed |
| `to_tlr_2003` | Tonga Law Reports 2003 ([download](https://ago.gov.to/cms/ago-materials/publications/tonga-law-reports.html?download=1563:2003_tlr)): Edwards v Pohiva (warrant of 20 Sep 1996) | 1,804,479 bytes, `02cbec45…0c8e3e`; PDF page 235 viewed |
| `to_pmo_legislative_assembly_page_2001` | Government page 'The Legislative Assembly' (archived) | 12,527 bytes, `83f4ab7d…7ae84e`; capture 2001-04-30 |
| `to_pmo_legislative_assembly_page_20011214` | Same page, revision of 14 Dec 2001 (archived; checker-located) | 12,470 bytes, `76eb2f55…2455a4`; capture 2001-12-14 |
| `to_la_ministers_page_2005` | Assembly ministers page ministers1.htm, content of July 2005 (archived; checker-located) | 22,559 bytes, `ffae9887…692f5f`; capture 2007-03-26 |
| `to_pmo_20020523_tuivakano_speaker` | Government media statement on Tu'ivakano's appointment, about 23 May 2002 (archived) | 6,996 bytes, `d37af6cc…d2e796`; capture 2002-06-03 |
| `to_pmo_legislative_assembly_page_20030408` | Government Assembly page, revision of 8 Apr 2003 (archived; checker-located) | 13,397 bytes, `a941aa7c…1cd610`; capture 2003-04-08 |
| `to_la_home_2004` | Assembly home page (archived) | 17,824 bytes, `1256f7ef…cde43e`; capture 2004-09-06 |
| `to_pmo_legislative_assembly_page_20041226` | Government Assembly page, revision of 19 Sep 2004 (archived; checker-located) | 12,802 bytes, `c230a530…5b98f5`; capture 2004-12-26 |
| `to_la_speaker_page_2005` | Assembly page 'The Speaker of the House' speaker1.htm (archived) | 4,845 bytes, `99afe647…7beb5f`; capture 2007-06-25 |
| `to_ipu_2005` | IPU PARLINE summary of the 2005 elections (archived) | 18,665 bytes, `4c8d5ad5…f5bb62`; capture 2024-11-07 |
| `to_pmo_20060209_veikune_directives` | PMO release on Veikune after his conviction, 9 Feb 2006 (archived) | 12,195 bytes, `9a70ba0e…ef40e4`; capture 2007-09-30 |
| `to_la_archives_2006` | Assembly 'Archives' page with a Speakers list (archived) | 17,818 bytes, `d97ed973…1edc02`; capture 2006-03-27 |
| `to_tlr_2007` | Tonga Law Reports 2007 ([download](https://ago.gov.to/cms/ago-materials/publications/tonga-law-reports.html?download=1559:2007_tlr)): Veikune v Kingdom of Tonga | 1,556,139 bytes, `e3f6737a…fe847c`; PDF page 230 viewed |
| `to_pmo_20080602_speaker_reply` | Speaker's reply to the 2008 Speech from the Throne, PMO speeches page (archived; checker-located) | 27,356 bytes, `3808da91…5197c4`; capture 2009-03-26 |
| `to_pmo_20100629_assembly_transcript` | Government-published Assembly transcript of 29 Jun 2010 (archived; checker-located) | 137,572 bytes, `410e1c66…4c6ad5`; capture 2011-11-30 |
| `to_la_profile_tuilakepa_2011` | Assembly profile of Lord Tu'ilakepa, 15 Sep 2010 (archived) | 20,258 bytes, `51ee9f61…39327c`; capture 2011-02-03 |
| `to_la_profile_tuivakano_2011` | Assembly profile of Lord Tu'ivakano (archived) | 19,855 bytes, `011cb558…977eab`; capture 2011-02-03 |
| `to_la_speakers_list_20110203` | Assembly 'Speakers of the House' list, February 2011 (archived) | 38,500 bytes, `c75402b5…c45d11`; capture 2011-02-03 |
| `to_la_speakers_list_20111130` | Assembly 'Speakers of the House' list, November 2011 (archived) | 59,808 bytes, `c205e15e…e3b22c`; capture 2011-11-30 |
| `to_la_obituary_fusitua_20140505` | Assembly news item on Lord Fusitu'a's funeral, 5 May 2014 (archived) | 44,098 bytes, `fcd92941…133392`; capture 2022-01-14 |
| `to_act20_2010_amending_copy` | [Act No. 20 of 2010](https://ago.gov.to/cms/images/LEGISLATION/AMENDING/2010/2010-0020/ActofConstitutionofTongaAmendmentNo.2Act2010.pdf), AGO amending copy | 90,497 bytes, `e4e9001f…f7db5b`; PDF pages 5, 12, 17 viewed |
| `to_la_amendment_no2_act21_2010` | [Act No. 21 of 2010](https://ago.gov.to/cms/images/LEGISLATION/AMENDING/2010/2010-0021/LegislativeAssemblyAmendmentNo.2Act2010.pdf), AGO amending copy | 43,541 bytes, `ddd1ce78…0b849e`; PDF pages 5, 6, 7, 8 viewed |
| `to_la_20101206_interim_note_archived` | Assembly explanatory note by the Interim Speaker, December 2010 (archived old-site copy) | 50,753 bytes, `ab47cbb7…b888f9`; capture 2011-11-30 |
| `to_la_20101221_lasike_elected` | Assembly news, Lord Lasike selected Speaker, 21 Dec 2010 (archived) | 19,549 bytes, `96a30037…870653`; capture 2011-05-18 |
| `to_la_2011_sitting_announced` | Assembly news, first sitting announced by the Speaker, undated (archived; checker-located) | 45,940 bytes, `7de3abfe…1a7670`; capture 2011-11-30 |
| `to_la_20110114_oaths` | Assembly news, members' oaths administered by the Speaker, 14 Jan 2011 (archived) | 19,349 bytes, `feca9a69…b5b3c9`; capture 2011-05-18 |
| `to_la_20110404_knesset_visit` | Assembly news, Knesset Speaker's visit, 4 Apr 2011 (archived) | 20,625 bytes, `5bb4d019…b0205b`; capture 2011-05-18 |
| `to_sc_r_v_lasike_cr285_2011` | [R v Noble Lasike, CR 285 of 2011](https://ago.gov.to/cms/judgements/supreme-court-criminal/category/69-cr-2012.html?download=820:r-v-lasike-unreported-supreme-court-of-tonga-cr-285-11-9-july-2012-cj), 9 Jul 2012 | 3,738,281 bytes, `dc116598…7a1746`; PDF pages 7, 16 viewed |
| `to_la_20120717_speaker_revoked` | Office of the Legislative Assembly release on the revocation, 17 Jul 2012 (archived) | 31,363 bytes, `86e849be…9c1638`; capture 2013-04-11 |
| `to_mic_20120719_fakafanua_elected` | MIC release, Lord Fakafanua elected Speaker, 19 Jul 2012 (archived) | 33,685 bytes, `25a28ad7…ba0e60`; capture 2013-04-22 |
| `to_la_20120719_fili_sea` | Assembly Tongan item 'Fili Sea', 19 Jul 2012 17:23 (archived; checker-located) | 14,274 bytes, `31da815f…d4794e`; capture 2012-10-30 |
| `to_la_20120720_fakafanua_appointed` | Assembly news, appointment reported, 20 Jul 2012 (archived) | 15,678 bytes, `7b4fecd8…f79e98`; capture 2012-10-30 |
| `to_la_20120723_new_speaker_starts` | Assembly news, new Speaker starts, 23 Jul 2012 (archived) | 14,950 bytes, `cb7ac334…b943d6`; capture 2012-10-30 |
| `to_la_20130828_acting_speaker` | Assembly news, Deputy Speaker remains Acting Speaker, 28 Aug 2013 (archived) | 18,359 bytes, `2fd5e321…fff9a9`; capture 2013-08-30 |
| `to_la_20130905_acting_speaker_quorum` | Assembly news, Acting Speaker on quorum, 5 Sep 2013 (archived) | 35,848 bytes, `533663b5…0dd2ff`; capture 2014-10-08 |
| `to_la_20140815_speaker_judiciary` | Assembly news, Speaker on judicial appointments, 15 Aug 2014 (archived) | 37,839 bytes, `85dd999b…9b9ec4`; capture 2014-10-08 |
| `to_la_news_speaker_china_20150224` | Assembly news, Lord Speaker Tu'ivakano's China trip, 24 Feb 2015 (archived) | 43,468 bytes, `73c01ed5…c7f82f`; capture 2015-07-02 |
| `to_la_news_mps_sworn_20180118` | Assembly news, MPs and the Lord Speaker sworn in, 18 Jan 2018 (archived) | 36,585 bytes, `64fc85aa…0cc0b5`; capture 2018-03-02 |
| `to_la_news_speaker_gita_20180305` | Assembly news, Speaker after Cyclone Gita, 5 Mar 2018 (archived; located here) | 37,874 bytes, `0cd4e36b…d10198`; capture 2018-03-22 |
| `to_la_news_pm_designate_20211215` | Assembly news, Prime Minister and Speaker elected, 15 Dec 2021 (archived) | 41,088 bytes, `74074abe…10adef`; capture 2021-12-16 |
| `to_la_news_opening_20220111` | Assembly news, 2022 session opened; Acting Speaker, 11 Jan 2022 (archived; located here) | 39,214 bytes, `0d5fa34e…c7de74`; capture 2022-01-11 |
| `to_la_news_speaker_summit_20220217` | Assembly news, Speaker at World Summit 2022, 17 Feb 2022 (archived; located here) | 40,352 bytes, `70dcd37b…3d2835`; capture 2022-02-17 |
| `to_la_notice_pm_meeting_20251212` | Office of the Interim Speaker release, meeting of 15 Dec 2025, 12 Dec 2025 (archived; checker-located) | 57,442 bytes, `74b01559…5258dc`; capture 2025-12-12 |
| `to_la_news_pm_designate_20251216` | Assembly news, Prime Minister-Designate and Speaker elected, 16 Dec 2025 (archived, gzip-stored) | 56,646 bytes, `5172843b…579105`; capture 2025-12-16; decoded (gzip transfer 15,626 bytes) |
| `to_la_release_speaker_appointment_to_20251219` | Chief Clerk's Tongan release on the appointments, 19 Dec 2025 (archived) | 57,823 bytes, `a1898f95…43f57c`; capture 2025-12-19 |
| `to_la_news_speaker_sworn_20260122` | Assembly news, Lord Speaker and MPs sworn in, 22 Jan 2026 (archived, gzip-stored; located here) | 54,760 bytes, `fe079867…5c9b4e`; capture 2026-01-22; decoded (gzip transfer 15,148 bytes) |
| `to_la_news_china_visit_20260519` | Assembly press release, delegation to China led by the Speaker, 19 May 2026 (archived; located here) | 60,118 bytes, `2bade793…4c0776`; capture 2026-05-19 |
| `to_la_news_ministers_oath_20260818` | Assembly news, ministers' oaths before the Acting Speaker, 18 Aug 2026 (archived, gzip-stored; located here) | 58,734 bytes, `c5847147…a566f4`; capture 2026-08-18; decoded (gzip transfer 16,119 bytes) |

Forty-three of the sources are raw Internet Archive captures (`id_` form) of original `parliament.gov.to`, `pmo.gov.to`,
`mic.gov.to` and `data.ipu.org` pages, all made before the cutoff; each records the capture URL as `url` and the original
address as `original_url`, and each extract records the capture time. The other six are PDFs served by `ago.gov.to`: two
static files and four Phoca Download links that serve a stored file as an attachment. Each new source has a checked-in
derived factual extract under [sources/](sources/) (49 in all) in the `spheres-c01-derived-factual-table/v1` format, with rows
keyed by `claim_id` that repeat the packet's claims exactly and add `observation_id` (`to_legislative_assembly`),
`review_observation` (TO-SPK-01 to 10), `role_id`, `holder_name` (null where the source names nobody), `role_title` and
`event_kind`. Each extract's own checksum is in the packet, separate from the original-response identity. Original pages,
PDFs and renders are not checked in; no seal, coat of arms, signature or photograph is republished. Pages listed as viewed
were rendered and visually checked for this packet; HTML pages were read as text.

Source types: `primary_court_record` (Tonga Law Reports; CR 285 of 2011), `primary_legislation_print` (Acts 20 and 21 of
2010), `primary_government_release_archived` (PMO and MIC releases), `primary_government_reference_list_archived` (the
government's Assembly page), `primary_legislature_reference_page_archived`, `primary_legislature_reference_list_archived`,
`primary_legislature_member_profile_archived`, `primary_legislature_news_notice_archived`,
`primary_legislature_release_archived`, `primary_legislature_address_archived`,
`primary_legislature_debate_transcript_archived` (Assembly pages, lists, profiles, news, releases, the Speaker's reply and
a transcript) and `interparliamentary_election_record` (IPU, corroboration only).

## Identities and stability checks

Every recorded response was downloaded twice for this packet, with at least 30 minutes between the downloads, and each pair
matched in byte count and SHA-256:

- first round: 2026-09-28T00:02:32Z to 2026-09-28T00:20:08Z (UTC);
- second round: 2026-09-28T00:53:10Z to 2026-09-28T01:18:34Z (UTC).

For the 43 sources recorded by the research or located by the checks on 25-26 September, the identities also match the
values recorded then; for the six sources located for this packet, a first probe download before the two rounds gave the
same identity. Each extract's `stability_check` gives its two download times.

- Internet Archive captures were fetched with `Accept-Encoding: identity` and no automatic decoding. Three captures made in
  2025-2026 (`to_la_news_pm_designate_20251216`, `to_la_news_speaker_sworn_20260122` and `to_la_news_ministers_oath_20260818`)
  are stored gzip-encoded and are returned with `Content-Encoding: gzip` even when identity is requested; their recorded
  identity is the decoded body, and the compressed transfer (identical in both rounds) is recorded separately in the extract.
- `ago.gov.to` answers some user-agent strings with a 403 HTML page; the recorded PDFs were fetched with curl's default user
  agent and no Accept-Encoding header. The four Phoca Download links (three law-report volumes and the CR 285 judgment) are
  plain GET links that serve a stored file; the two Acts are static files.
- No source is a page generated per request, a search or listing page, or a live page with a hit counter or rotating blocks:
  the Assembly's live pages carry hit counters, so their pre-cutoff captures are used, and the counters, sidebars and headlines
  frozen in those captures are not used as facts. IPU's live page embeds a per-request Cloudflare script; its pre-cutoff
  capture, byte-identical to the static archive.ipu.org copy, is used instead.
- The Assembly's minutes, which the research had downloaded, are not used (see Sources attempted).

## Leads not imported

- Encyclopaedias: [wikipedia.org](https://en.wikipedia.org/wiki/Legislative_Assembly_of_Tonga) (a Speaker table with "April
  1999", "1 July 2002", "10 February 2006-April 2008" and "2 May 2008"), and the articles on Fusitu'a, Fakafanua and Lasike,
  including a "29 Dec 2014" end for Fakafanua's first Speakership that no primary source supports. Leads only.
- News: [rnz.co.nz](https://www.rnz.co.nz/international/pacific-news/160410/tongan-king-appoints-new-speaker) (20 February
  2006, on Tu'iha'angana's appointment), RNZI items of 21 December 2010 and 18 July 2012, and a 2025 newspaper obituary of Lord
  Lasike; not fetched. The Assembly's repost of a Xinhua story (`247-chinas-top-legislator-meets-tongan-parliament-speaker`,
  October 2012) is news authorship and a lead only.
- MIC Tongan release `3919-fokotuu-a-lord-fakafanua-ko-e-sea-e-fale-alea-o-tonga` (19 July 2012): a republication of the
  Assembly's "Fili Sea" item, which is imported instead (capture of 22 June 2015: 28,931 bytes, SHA-256 `6920ccb3…122ddbe`,
  re-downloaded twice for this packet).
- Assembly item `156-deputy-speaker-resigns-to-take-up-new-ministerial-post` (2 July 2012): Lord Tu'iha'ateiho appointed
  Deputy Speaker that day, which conflicts with the 19 July releases; it concerns the Deputy Speakership, outside this role
  (capture of 30 October 2012: 15,063 bytes, SHA-256 `6d492fb2…7f5f871a`, re-downloaded twice for this packet).
- The government's Assembly page captured on 18 August 2002, still marking "Hon. Veikune (Speaker of the House)" after
  Tu'ivakano's appointment had been reported: evidence that the roster lagged (12,654 bytes, SHA-256 `c007510c…f1cffca`,
  re-downloaded twice for this packet).
- Tonga Law Reports 1996 ([download=1573:1996_tlr](https://ago.gov.to/cms/ago-materials/publications/tonga-law-reports.html?download=1573:1996_tlr)):
  Moala v Minister of Police dates the 1995 book-throwing "in October 1995" with "the then Acting or Deputy Speaker";
  Fotofili v Siale (1986) is outside the period. Lasike v R, Court of Appeal AC 11 of 2012
  ([download=771](https://ago.gov.to/cms/judgements/court-of-appeal/category/35-ac-2012.html?download=771:lasike-v-r-unreported-court-of-appeal-of-tonga-ac-11-12-cr-285-12-12-october-2012-salmon-j-moore-j-handley-j)),
  which set the conviction aside on 12 October 2012, never mentions the Speakership.
- The Assembly's per-Speaker print pages of November 2011 (for example `lorem-ipsum-ii/speakers-of-the-house/216-nopele-fusitua`)
  and member profiles only repeat the list's spans; the 2005 Assembly brochure repeats the Speaker page; the MIC "List of
  Speakers" of May 2011 (a `timeline-box` page) repeats the Assembly's list; the MIC "Who's Who" profile of 10 January 2011 says
  Lasike "will replace" Tu'ilakepa, with boilerplate about the House of Lords; the Assembly's nobles' page of 2005 says
  Tu'iha'angana had acted as Acting Speaker, undated.
- Live Assembly pages with hit counters: the 2025 item `parliament-pays-tribute-to-former-speaker-the-late-lord-lasike`, items
  of May and August 2011 styling Lasike, and other items styling Tu'ivakano, Fakafanua and Vaea (one or two attestations per
  term were imported, to keep the packet bounded). The current Rules of Procedure (edition date unknown) were not fetched.
- The Assembly's Tongan item of 23 February 2011 (the Crown Prince's special meeting with "Lord Lasike"), located by the
  check: a further attestation between 14 January and 4 April 2011, not imported (see the table of located records).
- IPU summaries for 1990-2002 name no Speaker; IPU 2008 (Tu'ilakepa appointed 2 May 2008) is the Crown packet's existing
  `to_ipu_2008`.

## Sources attempted

- **Assembly minutes behind a download form.** The research downloaded five sets of minutes that would have supported
  TO-SPK-09 and TO-SPK-10: `83-miniti-fika-01-aho-13-sanuali-2022`, `84-miniti-fika-02-aho-2-sune-2022`,
  `343-miniti-fika-1-fale-alea-aho-22-o-sanuali-2026`, `379-miniti-fika-15-aho-13-o-akosi-2026` and
  `380-miniti-fika-16-aho-18-o-akosi-2026` on `parliament.gov.to/en/parliament-business/hansards-debates/`. Each PDF is served
  only by POSTing the file page's Phoca Download form, whose fields include `license_agree=1` and a session token. Although
  the page shows no licence text, the reviewer declined to submit this form for CLAUDE-C01-08 and could not reproduce those
  responses, and this packet does not accept site licences or submit such forms, so the five minutes were not downloaded again
  and are not sources. Their facts are covered, where a pre-cutoff primary record exists, by the Assembly's own news items
  (11 January and 17 February 2022; 22 January, 19 May and 18 August 2026). Lost: the Speaker's apology and the statement
  that the offices were filled (13 January 2022), the Speaker's oath of 2 June 2022, and the presiding record of 13 August 2026.
- PacLII (`www.paclii.org/to/cases/`): the live site answers with a Cloudflare challenge, which was not attempted. The check
  read pre-cutoff raw captures of its 1989-1991 Supreme Court indexes and of Vaikona v Fuko (No 2) [1990]; none names the
  Speaker (the 1990 petition refers only to "the Speaker"). No claim.
- AGO gazettes by year: nothing for 1990, 1995, 2000, 2002 or 2004-2009; the 2010-2012, 2014-2015, 2017-2018, 2021-2022 and
  2025-2026 lists hold no Speaker appointment or revocation notice.
- PMO: release numbers between the 9 February 2006 directives (article 85) and the 13 February 2006 release (article 87), and
  articles 89 and 91, were not archived; the 2006-2008 category indexes list no Speaker appointment; the live `pmo.gov.to`
  answers curl with a JavaScript redirect (403) and was not pursued. Palace Office captures of 2008-2009 hold no Speaker item.
- The Assembly's site search returns 404, and growing search or listing pages are unsuitable anyway. The minutes of 19 January
  2015 survive only as an archived file page without the PDF. A second Assembly item on the December 2021 meeting (`880`) was
  not needed.
- The Internet Archive's CDX API and captures answered HTTP 429 and refused connections for periods on 26 and 28 September
  (UTC); every recorded download was retried after pacing, and nothing depends on a failed request.

## Checker defects

Part A (TO-SPK-01 to 03), part B (TO-SPK-04 to 06) and part C (TO-SPK-07 to 10) were each checked independently.

| # | Defect | Outcome |
|---|---|---|
| A1 | Tu'ilakepa holder offered as optional and year-only, although PMO-published records attest him | **Applied**: the 2 June 2008 reply and the 29 June 2010 transcript added; holder attested 2008-06-02; the profile's uncertainty reworded; IPU's 2 May 2008 not cited |
| A2 | Tu'ivakano holder dated to the contested 23 May 2002 statement | **Applied**: holder attested 2003-04-08 (government page revision), with the 2004 attestations; both 2002 dates kept as claims |
| A3 | Tu'iha'angana holder resting on the Crown packet's Gazette No. 19 claim | **Declined**: CLAUDE-C01-07's accepted test reserves that claim to the Crown entry and forbids other roles' holders from citing it; no Tu'iha'angana holder; the attestation is named in the ledger and the unresolved notes (the checker's corrected premise, that the gazette extract carries no role, is recorded) |
| A4 | Veikune's first-term roster can lag | **Applied**: caveat and Last-Modified date in the claim; the 14 December 2001 revision added; the lagging August 2002 capture listed as a lead |
| A5 | Veikune end claim misattributed and mislocated | **Applied**: reworded to the Acting Private Secretary's confirmation; locator paragraphs 1-3 |
| A6 | Palace confirmation locator | **Applied**: paragraph 3 (first sentence) |
| A7 | "Vacancy" is an inference | **Applied**: renamed `to_speaker_successor_expected_20060209`, event kind `successor_expected`, the implied vacancy stated |
| A8 | "Speaker of the Legislative Assembly" substituted for the printed words | **Applied**: quoted as printed, with a gloss |
| A9 | Name not as printed on the 2004 home page | **Applied**: "Hon. Tu'ivakano" |
| A10 | Removal dated to the decision day | **Applied**: renamed `to_speaker_veikune_removal_recited_20071213`, event kind `retrospective_statement` |
| A11 | Profile locator, date of text and event kind | **Applied**: locator paragraphs 1-4; the 2005 ministers page added; event kind `retrospective_statement`, the stated day kept as `attested_on` as C01-08 did for a retrospective timeline date, never a holder date |
| A12 | Obituary locator; the appointing King and 1991-1998 omitted | **Applied** |
| A13 | Inconsistent paragraph numbering on the Speaker page | **Applied** |
| A14 | February 2011 list's dated entries | **Applied**: only the 1875-1958 entries are stamped |
| A15 | IPU "consent" dated to the effective date | **Applied**: renamed `to_ipu_2005_tuihaangana_appointment_effective_20060210`, event kind `appointment_effective_reported` |
| A16 | 2002 statement's date basis | **Applied**: `published_date` 2002-05-23 with its basis in the scope note; the "last week" ambiguity stated |
| A17 | Title-only names not flagged; "Lasike" without the printed honorific | **Applied**: notes added; holder_name "the Honourable Lasike" |
| A18 | Malupo caveat and his 2000 acting service | **Applied**: `to_la_ministers_page_malupo_acting_2000`; caveat on the 1987-1989 list entry |
| A19 | No source types | **Applied** |
| A20 | Request method for the law reports | **Applied**: each extract's `fetch_recipe`; the 403 user-agent page described |
| A21 | PacLII archive route not recorded | **Applied**: recorded under Sources attempted; no claim |
| B1 | Resignation claim over-read and incomplete | **Applied** |
| B2 | Resumption of acting service not recorded | **Applied**: `to_tuihaateiho_resumed_acting_speaker_20130828` |
| B3 | MIC 3919 is a republication | **Applied**: the Assembly's "Fili Sea" item imported with its 17:23 time; MIC 3919 kept as a lead |
| B4 | Appointment claim locator | **Applied**: paragraphs 3-5 |
| B5 | "Starts" claim locator | **Applied**: paragraphs 1, 2 and 6 |
| B6 | Reported-appointment locator and the Tu'ilakepa link | **Applied** |
| B7 | Ballot claim overstated | **Applied**: renamed `to_fakafanua_adjourned_to_inform_king_2012`; "Legislative Act 2010 Act" [sic] |
| B8 | Revocation locator | **Applied** |
| B9 | Clause 23 sentence attributed to the letter | **Applied** |
| B10 | "beformally" quoted without [sic] | **Applied** |
| B11 | Act 20 assent given a structured date | **Applied**: renamed `to_act20_2010_royal_assent_printed`, no `attested_on` |
| B12 | Act 21 assent given a structured date | **Applied**: renamed `to_la_act21_2010_royal_assent_printed`, no `attested_on` |
| B13 | 2 July 2012 Deputy Speaker item left out of the acting claim | **Applied**: the conflict stated; the item is a lead |
| B14 | Uncertainty resting on the unimported MIC "Who's Who" profile | **Applied**: reference removed; the profile is a lead |
| C1 | A second Vaea holder duplicating the appointment | **Applied**: no appointment-based holder; Vaea attested 2026-05-19 |
| C2 | The 12 December 2025 notice missed | **Applied**: `to_speaker_election_meeting_notice_20251212` |
| C3 | Existing Interim Speaker and Speaker claims not linked | **Declined**: CLAUDE-C01-07's and C01-08's accepted tests reserve their claims to their own entries, and `tonga.json` keeps every role claim on its entry; the claims are named in the ledger and holder notes instead of being cited |
| C4 | Minutes download method not reproducible as written | **Superseded**: the minutes are not used (see Sources attempted) |
| C5 | 2015 trip dates said to be unstated | **Applied** |
| C6 | Tongan release names not as printed | **Applied**: "'Eiki Nōpele Vaea" |
| C7 | Oath claim cites a clause 83 that the 2026 minutes do not print | **Superseded**: the oath claim now rests on the Assembly's news item of 22 January 2026 |
| C8 | The acting-service claim of 13 January 2022 packs several events | **Superseded**: the acting claim now rests on the news item of 11 January 2022 |
| C9 | 2021 quotation attributed wrongly | **Applied** |
| C10 | 2018 holder name rests on another source | **Applied**: the 2018 holder rests on the item of 5 March 2018, which prints "Lord Fakafanua" |
| C11 | 2022 holder dated to a roster with an apology | **Applied**: holder attested 2022-02-17 from a styling |
| C12 | Minutes' published dates inconsistent | **Superseded**: the minutes are not used |
| C13 | Extract layout | **Applied**: rows with `observation_id`, `review_observation`, `role_title` and `event_kind` |

Primary records the checks located:

| Located record | Outcome |
|---|---|
| Speaker's reply of 2 June 2008 (PMO) | added (`to_pmo_20080602_speaker_reply`) |
| Assembly transcript of 29 June 2010 (PMO) | added (`to_pmo_20100629_assembly_transcript`) |
| Government Assembly page, revisions of 8 April 2003 and 19 September 2004 | added |
| Assembly ministers page (2005) | added (`to_la_ministers_page_2005`) |
| Government Assembly page, revision of 14 December 2001 (optional) | added |
| Assembly "Fili Sea" item, 19 July 2012 | added, replacing MIC 3919 |
| Assembly item on the first sitting of 13 January 2011 | added (`to_la_2011_sitting_announced`) |
| Assembly Tongan item on 23 February 2011 (optional) | not added: a further attestation between two already imported, deciding nothing |
| Office of the Interim Speaker release, 12 December 2025 | added |

## Suggested next work orders

These are proposals for the integrator. They are not created in `work-orders.json`.

- `C01-Tonga-SPK-002`: 1989-1991 — a Gazette, Privy Council record or Assembly journal naming the Speaker in 1989-1990 and
  dating Fusitu'a's appointment (National Archives, or a library holding of the official notices).
- `C01-Tonga-SPK-003`: 1998-2008 — the royal instruments for Veikune (1998 or 1999; 2005), Tu'ivakano (2002) and Tu'iha'angana
  (2006); a primary record of Tu'iha'angana as Speaker other than the Crown packet's gazette; Tu'ilakepa's 2008 appointment.
- `C01-Tonga-SPK-004`: the post-2010 royal appointments of the Speaker (2010, 2012, 2014, 2017, 2021) and any gazette notice.
- `C01-Tonga-SPK-005`: if the reviewer accepts the Assembly's minutes form for reproduction, the minutes of 13 January and 2
  June 2022 and of 13 and 18 August 2026.
- An integrator decision on whether the Crown packet's gazette claim may be cited by a Speaker holder, which would let a
  Tu'iha'angana holder be added without new research.

## Integration notes (outside this packet's file boundary)

- **Base and merge:** claim commit `a91a8249` on base `ffe54b02`; current integration `a33a8987` merged at `f1230bbb`; the fetch
  of 27 September 2026 found no newer integration commit, so no further merge was needed.
- `research-index.json` is regenerated in a **separate commit**. New totals: 1,378 sources and 3,813 claims across 9 country packets (previously 1,329 and 3,731), 841 organization and 34 institution observations, 93 open discovery batches. Tonga keeps nine entries, 15
  role observations and one open batch, `C01-Tonga-DISC-B001`; organization, institution, packet and batch counts are unchanged.
- `research/README.md`, the C01 README totals and `docs/planning/ai-workstreams.json` are left for the integrator; this handoff
  is self-proposed and not registered there.
- Pinned tests, none loosened and no assertion removed:
  - `test_tonga_research_s10g.py`: totals 168 sources and 266 claims (from 119 and 184), with the ownership comment.
  - `test_tonga_dpfi_c01_04.py` and `test_tonga_transition_c01_02.py`: their exact `to_speaker` holder list
    (`['to_speakers_appointment']`) is re-expressed exactly: the one string observation stays first and alone among strings,
    the dict holders are this packet's eleven, in order, and none cites a claim of the pinning packet.
  - `test_tonga_pm_1990_2019_c01_08.py`: its check that C01-08's 39 sources are the last 39 in the file becomes the same check
    on their fixed position, `sources[80:119]`, because this packet's 49 sources now follow them.
- Existing text is not edited: no existing source, claim, extract, holder or coverage entry changes; the new coverage entries
  are appended, and the `to_speaker` role gains a `scope_note`, as `to_king` and `to_pm` have.
- Claims of other packets that bear on the Speaker (the Crown packet's Gazette No. 19 and Gazette Extraordinary No. 9 of 2012,
  IPU 2008, and the prime-minister and transition packets' Interim Speaker and Speaker claims) are named in the ledger and
  holder notes but not cited, so no accepted packet's guard changes.
- New fields: none in `tonga.json`. The extract rows use the `rows` layout of the later C01 packets rather than the `claims`
  layout of the earlier Tonga extracts. No UI code changed and no browser review was run.

## Checks

```text
Run on 28 September 2026 (UTC) from the worktree root with PYTHONDONTWRITEBYTECODE=1; the sparse checkout includes
spheres-sim/src, spheres-sim/data and spheres-web/data, so the census runs (not widened further).

python -X utf8 tools/avatars/campaign_research.py            # regenerate
python -X utf8 tools/avatars/campaign_research.py --check    # pass: 9 packets, 841 organization and 34 institution
                                                             # observations, 1,378 sources, 3,813 claims, 93 batches
python -X utf8 tools/avatars/campaign_census.py --check      # pass (exit code 0)
python -X utf8 -m unittest discover -s tools/avatars -p "test_tonga*.py"      # 79 pass (10 new)
python -X utf8 -m unittest discover -s tools/avatars -p "test_*research*.py"  # 79 pass (overlaps the Tonga run)
python -X utf8 -m unittest discover -s tools/avatars -p "test_campaign*.py"   # 16 pass, census included
node --test tools/ui/check_leadership_research_review.cjs                     # 11 pass
python tools/planning/workboard.py --check                                    # pass (44 markers)
git diff --check -- <this packet's paths>                                     # clean
```
