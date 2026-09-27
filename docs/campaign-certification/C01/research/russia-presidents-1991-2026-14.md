# Russian presidents 1991-2026 14: the President of the Russian Federation from the retitling to the 2024 term

Packet: **CLAUDE-C01-14**. State: **ready_for_review** (not complete).
Owner: Claude. Integrator/reviewer: Codex. Branch `claude/c01-ru-14`; claim commit `9673a99f` on `0c9b5f19`,
which merges CLAUDE-C01-05 (`claude/c01-ussr-05` at `1c698ed0`) with integration `ffe54b02`. **Stacked on
CLAUDE-C01-05**: merge that packet first. Research access: 24 September 2026 (local); final re-downloads
2026-09-25T00:31Z-00:43Z (UTC). The historical cutoff stays **7 September 2026**.

This packet reviews ten observations, RU-PRES-01 to RU-PRES-10, in [russia.json](russia.json). It adds one role,
`ru_president` ("Президент Российской Федерации — President of the Russian Federation", kind `head_of_state`), to
the existing institution `ru_rsfsr_presidency`, with seven holder observations, 53 sources and 94 claims. It does
not change the C01-05 holders (Yeltsin and Rutskoi on the two RSFSR roles), the institution's lifecycle, its
institution-level sources and claims, or `ussr.json`. It adds no organization, institution, game mapping, portrait
or avatar. The parent scope (C01, C06, S23, WC1 and CP1) remains open.

Three research dossiers (parts A, B and C) and an independent adversarial check of each were prepared before this
packet was written. Every checker defect is applied below, one in part and one by removing the source; every
missing primary record the checks found is imported, except three: the Rossiyskaya Gazeta copy of 4-SF, which is
not the publication the portal cites, and two commercial-database titles, which are not official copies (see
[Checker defects](#checker-defects) and [Leads not imported](#leads-not-imported)).

## Outcome

| ID | Question | Decision |
|---|---|---|
| RU-PRES-01 | How were the state and the President's title renamed (1991-1992), and does a source tie the renamed office to Yeltsin? | **Accepted:** Law 2708-I (21 Apr 1992) retitles the office in the Constitution; determination 134-O (1998) states that the President of the RSFSR "стал именоваться" President of the Russian Federation and names Yeltsin; 1992-1993 records use the new title. All claims; no 1991-1996 holder on `ru_president`, no merge across roles |
| RU-PRES-02 | When did the 1993 Constitution enter into force, and what effect on the incumbent does a source state? | **Accepted:** Constitution text is procedure only (undated claims); 134-O states entry into force on 25 Dec 1993 and the incumbent's continuation from that day (a claim only) |
| RU-PRES-03 | 1996: the election result and Yeltsin's inauguration and oath | **Accepted in part:** calling (15 Nov 1995), both votes and both CEC protocols, the correction of 19 Jul, the ceremony, and 134-O's statement that he took the oath and so assumed office on 9 Aug 1996 (holder `from`). The CEC resolution of 9 Jul 1996 and a contemporaneous oath record were not found |
| RU-PRES-04 | 1999: resignation and acting service | **Accepted:** decree 1761 ends his exercise of powers at 12:00 on 31 Dec 1999 (holder `until`); address, news, decree 1762 and 4-SF are separate claims; acting service from 12:00 is claims only, with no stated end |
| RU-PRES-05 | 2000: election result and inauguration | **Accepted:** calling, vote, results table, signed protocol and result resolution (98/1110-3 in the gazette; 97/1110-3 in the CEC list) of 5 Apr, correction of 7 Jul; Baglai's declaration on 7 May 2000 that Putin assumed office (holder `from`) |
| RU-PRES-06 | 2004: election result and inauguration | **Accepted:** calling, vote, declaration of 23 Mar and three later corrections; Zorkin's declaration on 7 May 2004 (holder `from`); no end |
| RU-PRES-07 | 2008: Medvedev's election result and inauguration | **Accepted:** calling, vote, declaration and its gazette publication with figures; Zorkin's declaration on 7 May 2008 (holder `from`); Putin's pre-oath farewell is a claim, not an end |
| RU-PRES-08 | 2012: election result and inauguration | **Accepted:** calling, vote (Kremlin, 4 Mar), declaration and publication; Zorkin's declaration on 7 May 2012 (holder `from`); Medvedev's pre-oath farewell is a claim, so his observation has no `until` |
| RU-PRES-09 | 2018: election result and inauguration | **Accepted:** calling, vote (Kremlin, 18 Mar), declaration, publication and figures; assumption of office on 7 May 2018 (holder `from`) |
| RU-PRES-10 | 2024: election result, inauguration and an attestation before the cutoff | **Accepted:** calling, voting under way on 15 Mar, the signed resolution of 21 Mar (facsimile with annex); assumption of office on 7 May 2024 (holder `from`); decree 636 of 4 Sep 2026 attests him in office (not a boundary) |

### Holders on `ru_president`

| # | Name | `attested_on` | `from` | `until` | Claims |
|---|---|---|---|---|---|
| 1 | Борис Николаевич Ельцин | null | 1996-08-09 | 1999-12-31 | 134-O assumption and oath; decree 1761 |
| 2 | Владимир Владимирович Путин | null | 2000-05-07 | null | Baglai's declaration; oath |
| 3 | Владимир Владимирович Путин | null | 2004-05-07 | null | Zorkin's declaration; oath |
| 4 | Дмитрий Анатольевич Медведев | null | 2008-05-07 | null | Zorkin's declaration and oath (transcript); the Kremlin item's statement and oath |
| 5 | Владимир Владимирович Путин | null | 2012-05-07 | null | Zorkin's declaration; oath |
| 6 | Владимир Владимирович Путин | null | 2018-05-07 | null | Kremlin statement of assumption; oath |
| 7 | Владимир Владимирович Путин | null | 2024-05-07 | null | Kremlin statement of assumption; oath; decree 636 (attestation) |

Every `from` rests on a source that states the day office was assumed ("вступил в должность"); the oath is always a
separate claim and never the start. The only `until` rests on decree 1761, which states the day and hour the
exercise of powers ended. No end is taken from a successor's start, a re-election, a pre-oath farewell, Article
92(1) of the Constitution, or the latest attestation. In the atlas the six Putin and Medvedev observations read
"Reported interval: <day> → Not established" and Yeltsin's "1996-08-09 → 1999-12-31".

### Date ledger

Each row is a separate dated fact with its own claim or field. Calling, voting, protocol, declaration, correction,
publication, oath, stated assumption of office, ceremony, resignation, acting service and outgoing statements are
not merged, and different dates for them are not inconsistencies. `attested_on` is the date of the act or state the
source dates; the date a document was issued is `document_date` or `published_date`.

| Date | Event | Claim or field |
|---|---|---|
| 25 Dec 1991 | Law 2094-I renames the state (CLAUDE-C01-05, institution level; response re-verified) | `ru_law_2094i_rename_19911225` (unchanged) |
| 21 Apr 1992 | Law 2708-I amends the Constitution; the office reads "Президент Российской Федерации"; in force from publication (day unverified); published signature "Президент Российской Федерации Б.ЕЛЬЦИН" | `ru_law_2708i_*` (four claims) |
| 30 Nov 1992 | Ruling 9-P styles the author of the 1991 acts "Президент Российской Федерации" | `ru_ks_9p_decrees_styled_president_rf_19921130` |
| 23 Mar 1993 | Conclusion names "Президент Российской Федерации Б.Н.Ельцин" | `ru_ks_yeltsin_styled_president_rf_19930323` |
| 25 Dec 1993 | Constitution in force and the first term continues (134-O) | `ru_ks_134o_continuation_new_constitution_19931225` (claim only) |
| none | Constitution procedure (Section Two points 1 and 3, Articles 80-82, 92); 134-O's retitling statement | six undated claims |
| 15 Nov 1995 | Federation Council 697-I SF calls the election for 16 Jun 1996 | `ru_sf_697i_election_called_19951115` |
| 16 Jun 1996 | First-round voting | `ru_cec_first_round_voting_19960616` |
| 20 Jun 1996 | First-round protocol compiled | `ru_cec_first_round_protocol_figures_19960620` |
| 3 Jul 1996 | Run-off voting | `ru_cec_runoff_voting_19960703` |
| 9 Jul 1996 | Run-off protocol compiled; CEC resolution declaring the result (known from 134-O only) | `ru_cec_runoff_protocol_figures_19960709`, `ru_ks_134o_cec_declared_second_term_19960709` |
| 19 Jul 1996 | CEC 112/845-II revises the protocol | `ru_cec_protocol_amended_dagestan_19960719` |
| 5 Aug 1996 | Decree 1138 on the ceremony's symbols (procedure) | `ru_ukaz_1138_inauguration_symbols_rules_19960805` |
| 9 Aug 1996 | Ceremony; oath; stated assumption of office (134-O); Duma deputies next day: "вчера"; decree 1146 signed as President | holder 1 `from`; `ru_ks_134o_*_19960809`, `ru_duma_steno_inauguration_held_19960809`, `ru_ukaz_1146_signed_as_president_19960809` |
| 5 Nov 1998 | Determination 134-O and the separate opinion | `ru_ks_134o_determination_terms_19981105`, `ru_ks_134o_dissent_identity_question_19981105` |
| 31 Dec 1999, 12:00 | Decree 1761: Yeltsin ceases to exercise the powers; the Chairman of the Government acts; decree 1762: Putin acting from 12:00; the address; news at 12:30 | holder 1 `until`; resignation, acting and announcement claims |
| 5 Jan 2000 | 4-SF recites the resignation and calls the early election for 26 Mar | `ru_sf_4sf_*` |
| 26 Mar 2000 | Voting; CEC results table | `ru_cec_election_voting_20000326`, `ru_cec_results_table_20000326` |
| 5 Apr 2000 | Protocol signed; result resolution (98/1110-3 in the gazette, 97/1110-3 in the CEC list); certificate resolution 97/1111-3 | `ru_cec_protocol_2000_figures_20000405`, `ru_rg_cec_98_1110_3_*`, `ru_cec_res_97_*` |
| 6 May 2000 | Putin styled "И. о. Президента России" | `ru_kremlin_putin_styled_acting_president_20000506` (acting, claim only) |
| 7 May 2000 | CEC chair's announcement; oath; Baglai: assumed office; news at 13:00 | holder 2 `from`; `ru_kremlin_20000507_*`, `ru_kremlin_news_20000507_inauguration_noon` |
| 7 Jul 2000 | CEC 106/1149-3 corrects the protocol (Putin 39,740,467) | `ru_cec_106_1149_3_result_correction_20000707` |
| 10 Dec 2003 | 337-SF calls the election for 14 Mar 2004 | `ru_sf_337sf_election_scheduled_20031210` |
| 14 Mar 2004 | Voting (CEC wording of 29 Oct 2004; broadcast narration) | `ru_cec_125_902_4_voting_20040314`, `ru_kremlin_20040507_narration_vote_20040314` |
| 23 Mar 2004 | CEC 99/799-4 declares the result (published 24 Mar) | `ru_cec_res_99_799_4_*`; source `published_date` |
| 7 May 2004 | Oath; Zorkin: assumed office; news at 13:30 | holder 3 `from`; `ru_kremlin_20040507_*`, `ru_kremlin_news_20040507_inauguration_ceremony` |
| 29 Oct 2004; 20 Jan 2005; 25 Apr 2006 | Corrections 125/902-4, 135/935-4, 175/1128-4 (Putin 49,560,546, then 49,558,328) | `ru_cec_*_result_correction_*` |
| 26 Nov 2007 | 550-SF calls the election for 2 Mar 2008 | `ru_fc_550sf_election_called_20071126` |
| 2 Mar 2008 | Voting (Kremlin item of 7 May 2008) | `ru_kremlin_election_held_20080302` |
| 7 Mar 2008 | CEC 104/777-5 declares Medvedev elected; figures | `ru_cec_104_777_5_medvedev_elected_20080307`, `ru_rg_104_777_5_candidate_figures_20080307` |
| 8 Mar 2008 | Official publication, RG No. 4608 | `ru_rg_104_777_5_published_20080308` |
| 7 May 2008 | Putin's farewell before the oath; Medvedev's oath; Zorkin: took office; nuclear-forces handover | holder 4 `from`; `ru_kremlin_steno_*`, `ru_kremlin_medvedev_*` |
| 25 Nov 2011 | 442-SF calls the election for 4 Mar 2012 | `ru_fc_442sf_election_called_20111125` |
| 4 Mar 2012 | Voting (Kremlin news 14680) | `ru_kremlin_medvedevs_voted_20120304` |
| 7 Mar 2012 | CEC 112/893-6 declares Putin elected | `ru_cec_112_893_6_putin_elected_20120307` |
| 8 Mar 2012 | Official publication, RG No. 5724 | `ru_rg_112_893_6_published_20120308` |
| 7 May 2012 | Medvedev's farewell before the oath; Putin's oath; Zorkin: took office | holder 5 `from`; `ru_kremlin_medvedev_concluding_presidency_20120507` (no end) |
| 15 Dec 2017 | 528-SF calls the election for 18 Mar 2018 | `ru_fc_528sf_election_called_20171215` |
| 18 Mar 2018 | Voting (Kremlin news 57083) | `ru_kremlin_putin_voted_20180318` |
| 23 Mar 2018 | CEC 152/1255-7; figures | `ru_cec_152_1255_7_putin_elected_20180323`, `ru_rg_152_1255_7_candidate_figures_20180323` |
| 24 Mar 2018 | Official publication in RG | `ru_rg_152_1255_7_published_20180324` |
| 7 May 2018 | Oath; assumption of office | holder 6 `from` |
| 7 Dec 2023 | 678-SF calls the election for 17 Mar 2024 | `ru_fc_678sf_election_called_20231207` |
| 15 Mar 2024 | Voting under way (15-17 Mar stated prospectively) | `ru_kremlin_voting_15_17_march_20240315` |
| 21 Mar 2024 | CEC 163/1291-8 (signed facsimile); figures | `ru_cec_163_1291_8_putin_elected_20240321`, `ru_rg_163_1291_8_candidate_figures_20240321` |
| 7 May 2024 | Oath; assumption of office | holder 7 `from` |
| 4 Sep 2026 | Decree 636 signed "Президент Российской Федерации В.Путин", published that day | `ru_decree_636_putin_signs_as_president_20260904` (attestation, not a boundary) |

### Identifier changes from the dossiers

Source and claim IDs follow the dossiers except where a check required a change:

| Dossier | This packet | Why |
|---|---|---|
| `ru_ks_134o_oath_assumed_office_19960809` | `ru_ks_134o_oath_of_office_19960809` and `ru_ks_134o_assumption_of_office_19960809` | The oath and the stated assumption were one claim (A1) |
| `ru_ukaz_1146_government_resignation_accepted_19960809` | `ru_ukaz_1146_signed_as_president_19960809` | The decree accepts the Government's resignation, not a presidency event (A3) |
| `ru_rg_constitution_19931225`, `ru_rg_constitution_published_19931225` | lead | Its header lines are site template fields (A7) |
| event kinds `inauguration`, `acceptance`, `oath_and_stated_assumption_of_office` (A); `oath`, `election_scheduled`, `resignation` (B); `stated_assumption_of_office`, `oath_inauguration`, `attestation_in_office`, `stated_end_of_office` (C) | one vocabulary: `inauguration_ceremony`, `in_office_signature`, `oath_of_office`, `assumption_of_office`, `in_office_attestation`, `outgoing_holder_statement`, `election_called`, `resignation_reference` and the rest | A1, A3, A5, C1, C2, C9 |
| `ru_kremlin_voting_15_17_march_20240315` with `attested_period` 15-17 Mar | the same ID with `attested_on` 2024-03-15 | 16 and 17 March were prospective (C3) |
| `http://` URLs of the portal sources (part B) | `https://` packet URL, HTTP in `source_response_url` | B10 |
| (none) | `ru_sf_697i_*`, `ru_cec_first_round_*`, `ru_rg_cec_98_1110_3_*`, `ru_cec_protocol_2000_*`, `ru_cec_106_1149_3_*`, `ru_cec_125_902_4_*`, `ru_cec_135_935_4_*`, `ru_cec_175_1128_4_*`, `ru_kremlin_news_20040507_inauguration_ceremony`, `ru_rg_104_777_5_*`, `ru_kremlin_medvedevs_voted_20120304`, `ru_rg_112_893_6_published_20120308`, `ru_kremlin_putin_voted_20180318`, `ru_rg_152_1255_7_*`, `ru_rg_163_1291_8_candidate_figures_20240321` | Claims from the primary records the checks found (A12, B5-B8, C7, C8) |

## Observations

### RU-PRES-01 — The renaming and the retitled office (1991-1992)

Evidence (all from the official legal portal unless noted):

- Law 2094-I of 25 December 1991, already in the packet from CLAUDE-C01-05, renames the state; its response was
  re-downloaded and still matches C01-05's 20,875 bytes and hash. Its three claims stay at institution level.
- Law 2708-I of 21 April 1992 (`ru_rf_law_2708i_19920421`) amends the RSFSR Constitution "in connection with the
  change of the state's name": in listed chapters and articles, including Articles 102-123 (which hold 121-1 to
  121-11 on the President), "РСФСР" becomes "Российская Федерация". Point 50 restates Article 121-8 in terms of the
  "Президент Российской Федерации"; point 63 puts the law into effect from publication (day unverified); the
  published text is signed "Президент Российской Федерации Б.ЕЛЬЦИН".
- Ruling 9-P of 30 November 1992 (`ru_ks_post_9p_19921130`): in its own voice the Court calls the author of the 1991
  decrees the "Президент Российской Федерации" ("издал 20 июля 1991 года Указ"); "Президент РСФСР" appears only in
  quoted titles of other bodies' acts.
- The Court's conclusion of 23 March 1993 (`ru_ks_conclusion_19930323`) names "Президент Российской Федерации
  Б.Н.Ельцин".
- Determination 134-O of 5 November 1998 (`ru_ks_det_134o_19981105`) recounts that Yeltsin was elected the first
  President of the RSFSR on 12 June 1991 and that with the 25 December 1991 law and the constitutional amendments
  "Президент РСФСР стал именоваться Президентом Российской Федерации". Judge Morshchakova's separate opinion
  records that experts disagreed on whether the presidencies under the two constitutions are one institution.

Decision: accepted, as claims on `ru_president`. A source does tie the renamed office to Yeltsin, but only
retrospectively (134-O), and the separate opinion records the identity question. No `ru_president` holder is
recorded for 1991-1996: the 1992-1993 stylings predate the head-of-state definition of 1993, and the 25 December 1993
continuation is a claim only. The C01-05 holders on `ru_rsfsr_president` and `ru_rsfsr_vice_president` are
unchanged, no holder or claim is shared between those roles and `ru_president`, and the institution's name,
lifecycle and institution-level lists are unchanged.

Limits: the publication that brought 2708-I into force, its signed original, and whether the heading of Chapter 13-1
was retitled are unverified. Whether to add a dated 1992-1993 observation, or to treat the two roles as one retitled
office, is left to the integrator (coverage and next work).

### RU-PRES-02 — The 1993 Constitution

Evidence: the Constitution's original edition (`ru_const_1993_original`): Section Two point 1 (in force on official
publication after the popular vote), point 3 (the President elected under the old Constitution exercises the new
powers until his term expires), Article 80 (head of state), and Articles 81, 82 and 92 (term, oath, start and end of
powers). The portal lists *Российские вести* and *Российская газета* of 25.12.1993 as publications. Determination
134-O states that the Constitution entered into force on 25 December 1993, that the President then in his first term
exercised its powers from that day, and that the first term was neither interrupted nor restarted; its point 1
characterises the terms.

Decision: accepted. The five Constitution and portal claims carry no structured date: constitution text is procedure
only, and Article 92(1)'s rule that powers cease at the successor's oath is never used to infer an end. The 25
December 1993 date rests on the Court's statement, recorded as a continuation claim, never a holder or a start.
Morshchakova's separate opinion restates the State Duma's view that he exercised the new powers "с 1994 года"; the
Court's own date stands. Rossiyskaya Gazeta's 2004 web reproduction of the Constitution is a lead (A7).

Limits: the printed issues of 25 December 1993 were not inspected.

### RU-PRES-03 — 1996: the election, the result and the second-term oath

Evidence:

- Federation Council resolution 697-I SF of 15 November 1995 calls the election for 16 June 1996 (check find).
- The CEC protocol of the first round (`ru_cec_protocol_first_round_19960620`, CEC archive Word file, check find):
  headed 16 June 1996; 75,744,549 took part; Yeltsin 26,665,495, Zyuganov 24,211,686, Lebed 10,974,736; "Протокол
  составлен '20' июня 1996 года". The file was re-saved in 2008 and names no one as advancing.
- The CEC protocol of the run-off (`ru_cec_protocol_runoff_19960709`): headed 3 July 1996; 74,815,898 took part;
  Yeltsin 40,208,384, Zyuganov 30,113,306; compiled 9 July 1996; it does not state who is elected.
- CEC resolution 112/845-II of 19 July 1996 revises the protocol (Yeltsin 40,203,948) and says the result is
  unaffected.
- 134-O: a CEC resolution of 9 July 1996 officially announced him elected for a second term; at the ceremony of 9
  August 1996 the CEC chairman handed him the certificate, and "избранный Президент Российской Федерации принес
  присягу народу и, таким образом, вступил в должность на второй срок подряд".
- The State Duma stenogram of 10 August 1996: deputies speak of "Борис Николаевич Ельцин, инаугурация которого
  состоялась вчера".
- Decree 1138 of 5 August 1996 sets the ceremony's symbols (procedure); decree 1146 of 9 August 1996 is signed by him
  as President (it accepts the Government's resignation).

Decision: **accepted in part**. Holder 1, Борис Николаевич Ельцин, `from` 1996-08-09 on 134-O's statement of
assumption, citing that claim and the oath claim. The calling, the votes, the protocols, the declaration, the
correction, the ceremony, the Duma remarks and decree 1146 are role claims only. The start rests on a 1998 judicial
statement; if the reviewer does not accept a later court statement as a start, the fallback is `attested_on`
1996-08-09 with `from` null (recorded in the holder's uncertainty). This follows CLAUDE-C01-09, where a later
Presidency statement ("assumed his second term in office") dates Zuma's 2014 start.

Limits: the CEC resolution of 9 July 1996 and CEC resolution 105/825-II of 20 June 1996 (first-round result and
run-off) were not found in an official copy, and no contemporaneous official record of the oath was found. The CEC's
later corrections of August 1996 to November 1997 were not reviewed, so final figures are not established.

### RU-PRES-04 — 1999: resignation and acting service

Evidence:

- Decree 1761 of 31 December 1999 (`ru_pravo_ukaz_1761_19991231`), signed "Президент Российской Федерации Б.Ельцин":
  under Article 92(2) he ceases to exercise the powers of the President from 12:00 on 31 December 1999; under Article
  92(3) the Chairman of the Government exercises them from the same moment.
- Decree 1762 of the same day, signed "Исполняющий обязанности Президента Российской Федерации В.Путин", cites
  Yeltsin's resignation ("отставка") and states that Putin began the temporary exercise of the powers from 12:00.
- Yeltsin's statement (kremlin.ru transcript) and the site's news item of 12:30; Federation Council resolution 4-SF
  of 5 January 2000 recites the cessation by resignation; a kremlin.ru item of 6 May 2000 styles Putin "И. о.
  Президента России".

Decision: accepted. Holder 1's `until` is 1999-12-31 on decree 1761, which states the day and hour; it is attached
to the 1996 observation (not a second Yeltsin observation) and not to the C01-05 holder. The address, the news item,
1762's reference and 4-SF's recital are separate claims, not the end. Acting service is claims only (four claims,
role title "Исполняющий обязанности Президента Российской Федерации"); it has a sourced start and a dated attestation
on 6 May 2000, no source states when it ended, and none is inferred.

### RU-PRES-05 — 2000: the early election and the first inauguration

Evidence:

- 4-SF calls the early election for 26 March 2000; the CEC's results page is headed with that day (Putin 39,740,434,
  52.94 per cent).
- The CEC protocol, "Протокол подписан 5 апреля 2000 года" (check find), prints the same figures and 75,181,071
  voters taking part.
- The result resolution of 5 April 2000 in Rossiyskaya Gazeta's official section (check find), numbered
  98/1110-3, declares the election valid and Putin elected; its annex gives the eleven candidates' votes. The CEC's
  own April 2000 list numbers the same resolution 97/1110-3, and lists 97/1111-3 on his certificate.
- CEC resolution 106/1149-3 of 7 July 2000 corrects the protocol (Putin 39,740,467) and names the election of 26
  March and the protocol of 5 April in the CEC's words.
- The kremlin.ru transcript of 7 May 2000: the CEC chairman announces the result; Constitutional Court Chairman
  Baglai invites the oath; Putin takes it; Baglai declares that "Владимир Владимирович Путин" has assumed the office
  of President of the Russian Federation. The site's news item of 13:00 that day reports the ceremony.

Decision: accepted (upgraded from the dossier's "in part" now that the result text and the signed protocol are in
the packet). Holder 2 `from` 2000-05-07 on Baglai's declaration; no end. The transcript's participation figure
"57 миллионов 181 тысяча 71" swaps the first two digits of the CEC's printed 75,181,071 and is not used. The 97/98
numbering difference is recorded as printed and left open. Yeltsin's styling as "первый Президент Российской
Федерации" in the same records is retrospective and is not a tie between the two roles.

### RU-PRES-06 — 2004: re-election and the second inauguration

Evidence: 337-SF calls the election for 14 March 2004; CEC resolution 99/799-4 of 23 March 2004, as officially
published in Rossiyskaya Gazeta on 24 March, declares Putin elected (49,565,238, 71.31 per cent) with per-candidate
data; CEC resolutions 125/902-4 (29 October 2004), 135/935-4 (20 January 2005) and 175/1128-4 (25 April 2006) amend
the protocol (Putin 49,560,546, then 49,558,328) and name the election of 14 March and the protocol of 23 March; the
kremlin.ru broadcast transcript of 7 May 2004 has the oath and Zorkin's declaration that "Владимир Владимирович
Путин" has assumed office; news item 30888, stamped 7 May 2004 13:30, reports the ceremony (check find).

Decision: accepted. Holder 3 `from` 2004-05-07; no end (Putin's farewell of 7 May 2008 was spoken before
Medvedev's oath). The broadcast page has only a header date, so its `published_date` is null. The captured page of
135/935-4 repeats 125/902-4's date and number in its closing block; the heading is taken as its identity, since
175/1128-4 cites 135/935-4 of 20 January 2005.

### RU-PRES-07 — 2008: Medvedev

Evidence: 550-SF calls the election for 2 March 2008; CEC resolution 104/777-5 of 7 March 2008 declares Medvedev
elected (52,530,712, 70.28 per cent); Rossiyskaya Gazeta No. 4608 publishes it on 8 March 2008 with the per-candidate
table (check find); the kremlin.ru item of 7 May 2008 (12:20) states the vote of 2 March and that Zorkin announced
Medvedev's assumption of office; the broadcast transcript has Putin's farewell, the oath, and Zorkin's "Дмитрий
Анатольевич Медведев вступил в должность Президента России"; a later item has control of the strategic nuclear forces
transferred to "President of Russia" Medvedev.

Decision: accepted. Holder 4 `from` 2008-05-07, citing both statements of assumption and both oath records. Putin's
"Сейчас, слагая с себя полномочия главы государства", spoken before Medvedev's oath, is an outgoing-holder statement:
it does not state that his office had ended, so it gives holder 3 no end. The nuclear-forces handover is a role claim.

### RU-PRES-08 — 2012

Evidence: 442-SF calls the election for 4 March 2012 (its official-publication facsimile viewed); kremlin.ru news
14680 has the President voting on 4 March 2012 (check find); CEC resolution 112/893-6 of 7 March 2012 declares Putin
elected (45,602,075, 63.60 per cent) with its annex; Rossiyskaya Gazeta No. 5724 publishes it on 8 March 2012 (check
find); the kremlin.ru item of 7 May 2012 with its transcript: Medvedev speaks first ("Завершая работу на посту главы
государства"; "сегодня Президентом станет Владимир Владимирович Путин"), then the oath and Zorkin's declaration
that Putin has taken office.

Decision: accepted. Holder 5 `from` 2012-05-07. Medvedev's words are present-participle and future-tense and were
spoken before the handover, so they state no end; the dossier's proposed `until` 2012-05-07 is not set (C1). Unlike
Mandela's "since this morning" (CLAUDE-C01-09), nothing here says the office had already ended, and the successor's
oath is never used as an end.

### RU-PRES-09 — 2018

Evidence: 528-SF calls the election for 18 March 2018 (facsimile viewed); kremlin.ru news 57083 has Putin voting on
18 March 2018 (check find); CEC resolution 152/1255-7 of 23 March 2018 declares him elected (56,430,712, 76.69 per
cent), and its own annex (docx) and Rossiyskaya Gazeta's publication of 24 March 2018 (check find) give identical
per-candidate figures; the kremlin.ru item of 7 May 2018 states the Article 82 oath and that Zorkin announced his
assumption of office.

Decision: accepted. Holder 6 `from` 2018-05-07; no end of holder 5 is stated or inferred.

### RU-PRES-10 — 2024 and the attestation before the cutoff

Evidence: 678-SF calls the election for 17 March 2024 (facsimile viewed); the kremlin.ru item of 15 March 2024
(19:10) has the head of state voting and says the election is being held from 15 to 17 March; CEC resolution
163/1291-8 of 21 March 2024 declares Putin elected (76,277,708, 87.28 per cent), read in Rossiyskaya Gazeta's official
publication and in the signed facsimile with its annex (Davankov 3,362,484; Putin 76,277,708; Slutsky 2,795,629;
Kharitonov 3,768,470); the kremlin.ru item of 7 May 2024 states the oath and that Zorkin announced his assumption of
office; decree No. 636 of 4 September 2026, signed "Президент Российской Федерации В.Путин" and officially published
that day (publication No. 0001202609040002), attests him in office three days before the cutoff.

Decision: accepted. Holder 7 `from` 2024-05-07, citing the statement, the oath and the decree; the decree is an
attestation, not a boundary. The voting claim is dated 15 March only (C3). The CEC's own 2024 page is unavailable
(archived only as HTTP 403).

## Sources added

53 sources, each with a checked-in derived factual extract under [sources/](sources/) (`russia-*-facts.json`, LF,
format `spheres-c01-derived-factual-table/v1`, with its own checksum in the packet; 283,171 bytes in all). Each extract
records the original response's URL, byte count and SHA-256, the attached responses, a stability record and one row
per claim (claim_id, observation, role, `holder_name` and the printed form, role title, event kind, date, text,
locator). Original pages, PDFs and images are not checked in; no emblem, signature image or photograph is
republished. "Check find" marks a primary record found by an independent check.

| Source ID | What | Response identity (bytes, SHA-256) and read path |
|---|---|---|
| `ru_rf_law_2708i_19920421` | [Law 2708-I, 21 Apr 1992](https://pravo.gov.ru/proxy/ips/?doc_itself=&nd=102015844&page=1&rdk=0) | HTTP, 104,315 bytes, `0df2faec…0306ba`; card 3,163 `afa68972` |
| `ru_ks_post_9p_19921130` | [Constitutional Court ruling 9-P, 30 Nov 1992](https://pravo.gov.ru/proxy/ips/?doc_itself=&nd=102020039&page=1&rdk=0) | HTTP, 371,762 bytes, `bd71ec4f…0ebfc6`; card 3,445 `040ea51d` |
| `ru_ks_conclusion_19930323` | [Constitutional Court conclusion, 23 Mar 1993](https://pravo.gov.ru/proxy/ips/?doc_itself=&nd=102022316&page=1&rdk=0) | HTTP, 55,038 bytes, `bd3f3ddc…29885e`; card 9,452 `c3902c4e` |
| `ru_ks_det_134o_19981105` | [Constitutional Court determination 134-O, 5 Nov 1998](https://pravo.gov.ru/proxy/ips/?doc_itself=&nd=102056287&page=1&rdk=0) | HTTP, 36,602 bytes, `2c45f919…5d464f`; card 3,240 `52120d18` |
| `ru_const_1993_original` | [Constitution of 12 Dec 1993 (original edition)](https://pravo.gov.ru/proxy/ips/?doc_itself=&nd=102027595&page=1&rdk=0) | HTTP, 117,303 bytes, `5b49db4a…d5e5ed`; card 10,151 `019a9adf` |
| `ru_sf_res_697i_19951115` | [Federation Council 697-I SF, 15 Nov 1995](https://pravo.gov.ru/proxy/ips/?doc_itself=&nd=102038135&page=1&rdk=0) (check find) | HTTP, 7,748 bytes, `d173929b…2a78b5`; card 135,597 `ca463b87` |
| `ru_cec_protocol_first_round_19960620` | [CEC protocol, first round of 16 Jun 1996](https://web.archive.org/web/20160401180417id_/http://www.cikrf.ru/banners/vib_arhiv/president/1996/files/1/1996-1-Protokol_CIK.doc) (check find) | IA 20160401180417, 79,360 bytes, `6e587cc3…34f917` |
| `ru_cec_protocol_runoff_19960709` | [CEC protocol, run-off of 3 Jul 1996](https://web.archive.org/web/20160401153401id_/http://www.cikrf.ru/banners/vib_arhiv/president/1996/files/2/1996-2-Protokol_CIK.doc) | IA 20160401153401, 15,872 bytes, `0d1a31c3…e6043f`; index 15,137 `b8d50e1f` |
| `ru_cec_res_112_845ii_19960719` | [CEC 112/845-II, 19 Jul 1996](https://web.archive.org/web/20160401213450id_/http://www.cikrf.ru/law/decree_of_cec/1996/07/19/post_845_pr_1996.html) | IA 20160401213450, 22,132 bytes, `0d10bf75…d70c56` |
| `ru_ukaz_1138_19960805` | [Decree 1138, 5 Aug 1996](https://pravo.gov.ru/proxy/ips/?doc_itself=&nd=102042830&page=1&rdk=0) | HTTP, 9,749 bytes, `4ecae803…fe90ca`; card 5,322 `91aed157` |
| `ru_duma_steno_19960810` | [State Duma stenogram, 10 Aug 1996](https://transcript.duma.gov.ru/node/2898/) | HTTP, 503,153 bytes, `c6a8530f…e65f74` |
| `ru_ukaz_1146_19960809` | [Decree 1146, 9 Aug 1996](https://pravo.gov.ru/proxy/ips/?doc_itself=&nd=102042895&page=1&rdk=0) | HTTP, 24,028 bytes, `82667085…53228d`; card 21,917 `1f61933b` |
| `ru_pravo_ukaz_1761_19991231` | [Decree 1761, 31 Dec 1999](https://pravo.gov.ru/proxy/ips/?doc_itself=&nd=102063835&page=1&rdk=0) | HTTP, 20,095 bytes, `28c0900f…fed2ed`; card 3,205 `2e458ea8`; capture 7,429 `7fc6d35f` |
| `ru_pravo_ukaz_1762_19991231` | [Decree 1762, 31 Dec 1999](https://pravo.gov.ru/proxy/ips/?doc_itself=&nd=102063834&page=1&rdk=0) | HTTP, 19,902 bytes, `5612fd3a…abbdbc`; card 3,214 `4788164f` |
| `ru_kremlin_yeltsin_statement_19991231` | [Yeltsin's statement, 31 Dec 1999](https://web.archive.org/web/20260212101412id_/http://www.kremlin.ru/events/president/transcripts/24080) | IA 20260212101412, 44,320 bytes, `12d7d838…2c0358` |
| `ru_kremlin_news_resignation_19991231` | [Kremlin news 37381, 31 Dec 1999](https://web.archive.org/web/20260405070524id_/http://kremlin.ru/events/president/news/37381) | IA 20260405070524, 10,557 bytes, `c796d164…8d7b1f`; decoded 42,935 `6c08885a` |
| `ru_pravo_sf_4sf_20000105` | [Federation Council 4-SF, 5 Jan 2000](https://pravo.gov.ru/proxy/ips/?doc_itself=&nd=102063946&page=1&rdk=0) | HTTP, 8,365 bytes, `9f7ded63…55bdf2`; card 3,179 `ece7d1cb` |
| `ru_kremlin_news_acting_president_20000506` | [Kremlin news 38128, 6 May 2000](https://web.archive.org/web/20260611174508id_/http://www.kremlin.ru/events/president/news/38128) | IA 20260611174508, 36,393 bytes, `a579074b…66f058` |
| `ru_fci_cec_results_20000326` | [CEC results table, 26 Mar 2000](https://web.archive.org/web/20000511054425id_/http://www.fci.ru:80/prez2000/oitog26/pr_r00.htm) | IA 20000511054425, 5,207 bytes, `53f135dc…9e93d5` |
| `ru_fci_cec_resolutions_april_2000` | [CEC list of April 2000 resolutions](https://web.archive.org/web/20000831231210id_/http://www.fci.ru:80/prez2000/pnorm_akt/pnorm_akta_s.htm) | IA 20000831231210, 5,647 bytes, `01e52664…0e430a` |
| `ru_rg_cec_res_98_1110_3_20000405` | [CEC 98/1110-3 (RG official section), 5 Apr 2000](https://web.archive.org/web/20010218034958id_/http://www.rg.ru:80/oficial/doc/vybor_99/98.htm) (check find) | IA 20010218034958, 5,684 bytes, `af2e5bf0…a94981`; annex 4,893 `8279818e` |
| `ru_cec_protocol_2000_20000405` | [CEC protocol signed 5 Apr 2000](https://web.archive.org/web/20140802013752id_/http://cikrf.ru:80/banners/vib_arhiv/president/2000/files/2000-Protokol_CIK.doc) (check find) | IA 20140802013752, 57,344 bytes, `2815f790…4cb226`; index 10,968 `6809b850` |
| `ru_kremlin_inauguration_stenogram_20000507` | [Kremlin transcript 21410, 7 May 2000](https://web.archive.org/web/20260208102524id_/http://kremlin.ru/events/president/transcripts/21410) | IA 20260208102524, 64,522 bytes, `3d6f0277…8d4da6` |
| `ru_kremlin_news_inauguration_20000507` | [Kremlin news 38089, 7 May 2000](https://web.archive.org/web/20260514112545id_/http://www.kremlin.ru/events/president/news/38089) | IA 20260514112545, 56,816 bytes, `94ea965d…acb205` |
| `ru_cec_res_106_1149_3_20000707` | [CEC 106/1149-3, 7 Jul 2000](https://web.archive.org/web/20140801070526id_/http://cikrf.ru:80/banners/vib_arhiv/president/2000/post_1149_pr_2000.html) (check find) | IA 20140801070526, 32,209 bytes, `b4752f24…f75358` |
| `ru_pravo_sf_337sf_20031210` | [Federation Council 337-SF, 10 Dec 2003](https://pravo.gov.ru/proxy/ips/?doc_itself=&nd=102084559&page=1&rdk=0) | HTTP, 7,843 bytes, `698bc387…cd4cd1`; card 3,246 `8254367c` |
| `ru_rg_cec_res_99_799_4_20040323` | [CEC 99/799-4 (RG), 23 Mar 2004](https://web.archive.org/web/20041021110403id_/http://www.rg.ru:80/2004/03/24/cik-dok-printable.html) | IA 20041021110403, 5,991 bytes, `0616440b…277cb5` |
| `ru_kremlin_inauguration_broadcast_20040507` | [Kremlin supplement 1924, 7 May 2004](https://web.archive.org/web/20260519104156id_/http://kremlin.ru/supplement/1924) | IA 20260519104156, 62,383 bytes, `77ce3743…e902e1` |
| `ru_kremlin_news_inauguration_20040507` | [Kremlin news 30888, 7 May 2004](https://web.archive.org/web/20251206040020id_/http://www.kremlin.ru/events/president/news/30888) (check find) | IA 20251206040020, 43,710 bytes, `34c36670…b67412` |
| `ru_cec_res_125_902_4_20041029` | [CEC 125/902-4, 29 Oct 2004](https://web.archive.org/web/20111119154354id_/http://cikrf.ru/law/decree_of_cec/2004/10/29/post_902_pr_2004.html) (check find) | IA 20111119154354, 33,983 bytes, `31e36b14…10c1a0`; index 10,287 `c908538b` |
| `ru_cec_res_135_935_4_20050120` | [CEC 135/935-4, 20 Jan 2005](https://web.archive.org/web/20111119155204id_/http://cikrf.ru:80/banners/vib_arhiv/president/2004/post_935_pr_2004.html) (check find) | IA 20111119155204, 14,959 bytes, `760ab1e2…827fbe` |
| `ru_cec_res_175_1128_4_20060425` | [CEC 175/1128-4, 25 Apr 2006](https://web.archive.org/web/20111119155112id_/http://cikrf.ru/law/decree_of_cec/2006/04/25/zp061128.html) (check find) | IA 20111119155112, 32,055 bytes, `b2d68d5f…78d9b2` |
| `ru_fc_res_550sf_20071126` | [Federation Council 550-SF, 26 Nov 2007](https://pravo.gov.ru/proxy/ips/?doc_itself=&nd=102118390&page=1&rdk=0) | HTTP, 32,327 bytes, `1d0ca88f…6e2615`; card 3,246 `e1184174` |
| `ru_cec_res_104_777_5_20080307` | [CEC 104/777-5, 7 Mar 2008](https://web.archive.org/web/20080313193604id_/http://www.cikrf.ru:80/postancik/Zp080777.jsp) | IA 20080313193604, 15,657 bytes, `eee45922…e2274d` |
| `ru_rg_cec_res_104_777_5_20080308` | [CEC 104/777-5 in RG No. 4608, 8 Mar 2008](https://web.archive.org/web/20080310004304id_/http://www.rg.ru:80/2008/03/08/cik-president-dok.html) (check find) | IA 20080310004304, 41,974 bytes, `9d871e47…2734d9`; annex 11,240 `25a076d2` |
| `ru_kremlin_news_4_20080507` | [Kremlin news 4, 7 May 2008](https://web.archive.org/web/20260312204311id_/http://www.kremlin.ru/events/president/news/4) | IA 20260312204311, 57,494 bytes, `3410c4de…eceacd` |
| `ru_kremlin_transcript_3_20080507` | [Kremlin transcript 3 (text version), 7 May 2008](https://web.archive.org/web/20220526201821id_/http://kremlin.ru/events/president/transcripts/copy/3) | IA 20220526201821, 38,354 bytes, `b2c728d1…eb29f5`; capture 77,737 `ac1a5e24` |
| `ru_kremlin_news_11_20080507` | [Kremlin news 11, 7 May 2008](https://web.archive.org/web/20260517065034id_/http://www.kremlin.ru/events/president/news/11) | IA 20260517065034, 37,548 bytes, `21d22773…41418d` |
| `ru_fc_res_442sf_20111125` | [Federation Council 442-SF, 25 Nov 2011](https://pravo.gov.ru/proxy/ips/?doc_itself=&nd=102152264&page=1&rdk=0) | HTTP, 24,040 bytes, `35c52f74…524776`; card 3,408 `21e3aa98`; facsimile 45,191 `768c128e` |
| `ru_kremlin_news_14680_20120304` | [Kremlin news 14680, 4 Mar 2012](https://web.archive.org/web/20230202123957id_/http://kremlin.ru/events/president/news/14680) (check find) | IA 20230202123957, 46,512 bytes, `e1423d45…6636a0` |
| `ru_cec_res_112_893_6_20120307` | [CEC 112/893-6, 7 Mar 2012](https://web.archive.org/web/20120311220745id_/http://www.cikrf.ru:80/law/decree_of_cec/2012/03/07/Zp12893.html) | IA 20120311220745, 12,676 bytes, `e40cc334…0baf48` |
| `ru_rg_cec_res_112_893_6_20120308` | [CEC 112/893-6 in RG No. 5724, 8 Mar 2012](https://web.archive.org/web/20120311050014id_/http://rg.ru:80/2012/03/08/cik-vibory-dok.html) (check find) | IA 20120311050014, 45,730 bytes, `847bc03a…989006` |
| `ru_kremlin_news_15224_20120507` | [Kremlin news 15224 (text version), 7 May 2012](https://web.archive.org/web/20251012044258id_/http://kremlin.ru/events/president/news/copy/15224) | IA 20251012044258, 16,063 bytes, `1c0d7d4e…6348ef` |
| `ru_fc_res_528sf_20171215` | [Federation Council 528-SF, 15 Dec 2017](https://pravo.gov.ru/proxy/ips/?doc_itself=&nd=102454764&page=1&rdk=0) | HTTP, 20,353 bytes, `effab7f8…9f5e3c`; card 3,263 `6b3e1a41`; facsimile 65,007 `2fa290f5` |
| `ru_kremlin_news_57083_20180318` | [Kremlin news 57083, 18 Mar 2018](https://web.archive.org/web/20180320095741id_/http://kremlin.ru:80/events/president/news/57083) (check find) | IA 20180320095741, 42,708 bytes, `6df21b99…640963` |
| `ru_cec_res_152_1255_7_20180323` | [CEC 152/1255-7, 23 Mar 2018](https://web.archive.org/web/20180324211630id_/http://cikrf.ru:80/activity/docs/postanovleniya/39429/) | IA 20180324211630, 40,311 bytes, `50e3a6fa…8a9188`; annex 18,560 `21e55f09` |
| `ru_rg_cec_res_152_1255_7_20180324` | [CEC 152/1255-7 in RG, 24 Mar 2018](https://web.archive.org/web/20180324135637id_/https://rg.ru/2018/03/23/postanovlenie-site-dok.html) (check find) | IA 20180324135637, 143,832 bytes, `322f1a53…4f126c` |
| `ru_kremlin_news_57416_20180507` | [Kremlin news 57416 (text version), 7 May 2018](https://web.archive.org/web/20251229124214id_/http://www.kremlin.ru/events/president/news/copy/57416) | IA 20251229124214, 20,506 bytes, `e0ac877d…cab8d7` |
| `ru_fc_res_678sf_20231207` | [Federation Council 678-SF, 7 Dec 2023](https://pravo.gov.ru/proxy/ips/?doc_itself=&nd=606168232&page=1&rdk=0) | HTTP, 24,272 bytes, `50613a83…dd2915`; card 3,315 `1924dcd4`; facsimile 123,828 `37be57dc` |
| `ru_kremlin_news_73658_20240315` | [Kremlin news 73658, 15 Mar 2024](https://web.archive.org/web/20260513124847id_/http://kremlin.ru/events/president/news/73658) | IA 20260513124847, 37,431 bytes, `3dffdcc9…5c9c86` |
| `ru_rg_cec_res_163_1291_8_20240321` | [CEC 163/1291-8 in RG, 21 Mar 2024](https://web.archive.org/web/20240405043140id_/https://rg.ru/documents/2024/03/21/cik-post-resultaty2024-site-dok.html) | IA 20240405043140, 92,949 bytes, `e7ba12c9…0a937f`; facsimile 85,147 `6ff16d38` |
| `ru_kremlin_news_73981_20240507` | [Kremlin news 73981 (text version), 7 May 2024](https://web.archive.org/web/20250504092901id_/http://www.kremlin.ru/events/president/news/copy/73981) | IA 20250504092901, 13,029 bytes, `a55150b9…1af56c` |
| `ru_pub_decree_636_20260904` | [Decree 636, 4 Sep 2026 (official publication PDF)](https://publication.pravo.gov.ru/file/pdf?eoNumber=0001202609040002) | HTTP, 32,911 bytes, `986757f8…269c80`; card 20,071 `1acd4fc5` |

Read paths. The legal portal, its official-publication section and the State Duma transcripts server refused HTTPS
from this environment (port 443), so their responses were read over HTTP; the packet URL gives the same path on
HTTPS, the only scheme the register accepts, and each extract records the HTTP URL read (as CLAUDE-C01-05 did).
kremlin.ru, cikrf.ru, fci.ru and rg.ru pages are read from raw Internet Archive captures (`id_` form, all made before
the cutoff); the packet URL is the capture, `original_url` is the exact captured page (with `www` and `/copy/` text
version paths kept, `:80` removed), and `archive_capture_utc` records the timestamp.

## Response identities and stability checks

Every recorded response was downloaded at least three times, and each identity is the response of a stored page or
file, not one generated per request:

- the dossier's first download and a delayed re-download 28-60 minutes later (portal responses also with a
  cache-busting query);
- the independent check, 38-75 minutes after the first downloads (part C also with cache-busting queries on the
  portal and the publication section); records the checks found were hashed and re-downloaded 30-44 minutes later;
- this packet, 2026-09-25T00:31:53Z-00:43:51Z: 97 of 100 recorded identities (every primary response, every portal
  card, CEC index, annex, facsimile and corroborating capture) came back with identical bytes and SHA-256, several
  hours after the first downloads. The three that did not are live kremlin.ru text versions kept only as alternates
  (connections refused). The live full pages of news 4 and news 15224, which carry rotating blocks and current titles,
  are no longer recorded. After that check the portal cards of 134-O, 9-P, 2708-I and 442/528/550/678-SF were
  replaced by date-restricted one-hit searches, because the unrestricted number searches also list later acts with
  the same number and can grow; each replacement returned identical bytes on the review's downloads at
  2026-09-25T01:31Z and 02:03Z and on this revision's at 02:14Z, and with a cache-busting query at 02:16Z-02:17Z.

Points a reviewer needs:

- `ru_kremlin_news_resignation_19991231` is stored gzip-compressed: the identity is the 10,557-byte body as
  transferred (curl without `--compressed`); the decoded 42,935-byte body (`6c08885a…`) is recorded too.
- Live kremlin.ru pages carry the current footer year, menus and a promoted block; they are recorded only as
  `alternate_responses`, never as identities. The captured text versions (`/copy/`) of news 15224 and 57416 were
  byte-identical to the live text versions; the live text versions of transcript 3 and news 73981 differ only in a
  build hash, which is why the captures are the identities.
- The decree PDF (CreationDate 2026-09-04) and the three facsimile PDFs or ZIP (CreationDate or member date on the
  publication day) are stored files; cache-busting downloads returned identical bytes.
- The State Duma page is sent with no-store headers and a session cookie, but its body carries no token: immediate,
  cache-busting and later downloads were identical.
- The CEC's 2018 annex docx and the RG 2000 pages have Internet Archive digests identical across later captures.

## Leads not imported

- **kremlin.ru biography of Yeltsin**, [capture 20260720093348](https://web.archive.org/web/20260720093348id_/http://www.kremlin.ru/catalog/persons/6/biography):
  a retrospective timeline (elected "first President of the Russian Federation" on 12 June 1991, re-elected 3 July
  1996, resignation decree 31 December 1999). Identities: as served (gzip) 8,748 bytes, `85624723…d316e65`; decoded
  32,528 bytes, `79f5320a…11166a1`. Retrospective and undocumented; it would retro-title the 1991 election.
- **Rossiyskaya Gazeta's web page of the Constitution**, [capture 20040326195037](https://web.archive.org/web/20040326195037id_/http://www.rg.ru:80/1993/12/25/konstituciya.html)
  (rg.ru/1993/12/25/konstituciya, 296,556 bytes, `9d204c1b…a267e`): its "Опубликовано 25 декабря 1993 г." is followed
  by "Вступает в силу с момента подписания", which contradicts the Constitution, so both are site template fields;
  the text is consolidated to 2001 (A7).
- **CEC resolution 105/825-II of 20 June 1996** and a page titled "О результатах выборов Президента Российской
  Федерации": titles only in the commercial Kodeks database (docs.cntd.ru/document/600333 and 901758223), which timed
  out; not official copies.
- **CEC summary table of the 1996 run-off** (1996-2-Svodnaya_CIK.xls, 49,664 bytes, `3547e42d…`): created in 2008;
  the protocol is used. The later 1996-1997 CEC corrections (post_850_pr_1996 and three others) were not reviewed.
- **Rossiyskaya Gazeta's official-section copy of 4-SF** (vybor_99/2000_4.htm, capture 20000712205338, 4,308 bytes,
  `24945dfb…`): a copy, not the publication the portal cites (Parlamentskaya Gazeta; Sobranie zakonodatelstva).
- Federal Assembly resolutions on the bill on the assumption of office (1996-2000): legislative procedure on a bill.
- kremlin.ru items redundant with imported records or outside the questions: acts/bank/14857 (decree 1763), news/37383
  and news/37410 (31 December 1999), transcripts/22280 (the Acting President's New Year address), news/38099 (decree
  835, 7 May 2000), news/30885 and transcripts/22452 (2004), transcripts/1 and 2 (2008 addresses, also in transcript
  3), news/copy/73692 (21 March 2024), news/15221 (2012 schedule notice), news/57420 and news/73983 (the Government
  told to continue, 2018 and 2024), news/57091 (foreign congratulations). The document-bank copies of decrees 1761 and
  1762 are recorded as alternate responses.
- The broadcast commentators' remarks in transcript 3 (2008), Rossiyskaya Gazeta's news reports (2004/03/24/cik.html
  and the 2024 editorial links), the CEC's 2000 section menu (text_pr.htm) and its 2003-2004 index (_1/doc_6_1.htm).
- Other decrees signed "В.Путин" and published 31 August - 7 September 2026, and the Eastern Economic Forum item of
  3 September 2026: alternative attestations; one decree is used.

## Sources attempted

- `https://pravo.gov.ru/…`, `https://publication.pravo.gov.ru/…` and `https://transcript.duma.gov.ru/…`: port 443
  failed; the same paths over HTTP worked.
- `https://www.kremlin.ru/`: port 443 failed; kremlin.ru later refused ports 80 and 443 for parts of the check window.
  Guessed inauguration paths and `/events/president/by-date/…` returned 404.
- cikrf.ru, izbirkom.ru and ksrf.ru: no connection over HTTP or HTTPS. The CEC's later "1996 presidential election"
  page is archived as a 404 page; the 2004 election page's captures stop before the result; its 2024 resolution pages
  are archived only as HTTP 403.
- rg.ru: HTTP 401 (bot protection), not bypassed. Presidential Library search: a challenge page, not bypassed.
- The CEC's own texts of 97/1110-3 and 97/1111-3 (fci.ru Zp001110.htm, Zp001111.htm): not archived. Internet
  Archive CDX searches for Rossiyskaya Gazeta 2000 pages returned 504 or nothing (the official-section copy was found
  another way).
- The legal portal's date lists for 20 June and 9-12 July 1996 and for 26 March - 8 April 2000 hold no CEC acts; its
  legislation base does not carry CEC result resolutions.
- The Internet Archive was intermittently "Temporarily Offline" or refused connections; every capture used was
  eventually downloaded and re-verified.

## Checker defects

| # | Defect | Outcome |
|---|---|---|
| A1 | 134-O's oath and stated assumption of office were one claim | **Applied**: split into `ru_ks_134o_oath_of_office_19960809` (`oath_of_office`) and `ru_ks_134o_assumption_of_office_19960809` (`assumption_of_office`); `from` rests on the assumption claim |
| A2 | The 1996 holder cited ceremony claims and decree 1146 | **Applied**: the holder cites the assumption and oath claims and decree 1761's stated end; the certificate, the Duma remarks and 1146 are role claims only |
| A3 | Decree 1146 labelled "acceptance" | **Applied**: renamed `ru_ukaz_1146_signed_as_president_19960809`, kind `in_office_signature`; its uncertainty says the acceptance concerns the Government |
| A4 | The 1996 re-election and the 9 July declaration were one claim | **Applied**: the claim keeps the declaration only, with no holder name; the re-election sentence is not imported (the protocols date both rounds) |
| A5 | The certificate claim bundled events under a non-standard kind | **Applied**: kind `inauguration_ceremony`, context only; the CEC chairman's announcement was not split out (optional in the check) |
| A6 | Ruling 9-P paraphrased unfairly; converter-dependent locator | **Applied**: reworded to the Court's own usage; locator "reasoning (установил), part I" |
| A7 | The Rossiyskaya Gazeta page was read selectively | **Resolved by removal**: moved to leads with both header lines; 25 December 1993 rests on 134-O |
| A8 | The "с 1994 года" variant was unrecorded | **Applied**: in the continuation claim's uncertainty |
| A9 | A procedure claim (decree 1138) carried a holder name | **Applied**: holder name null |
| A10 | Non-standard `statement_date` key | **Applied**: dropped; the claim is undated and says "Stated in the determination of 5 November 1998" |
| A11 | One Constitution claim was filed under two observations | **Applied**: RU-PRES-02 only |
| A12 | The first round and the calling of the 1996 election were missing | **Applied**: 697-I SF and the first-round protocol imported (three claims); 105/825-II recorded as unresolved and a lead |
| A13 | The Kremlin lead lacked its decoded identity | **Applied**: both identities in Leads |
| B1 | Locator of 97/1110-3 | **Applied**: "Last list entry" |
| B2 | Locator of 97/1111-3 | **Applied**: "Last-but-one list entry" |
| B3 | Locator of the address passage | **Applied**: "paragraph beginning 'В соответствии с Конституцией, уходя в отставку' (fourth from the end)" |
| B4 | The turnout "conflict" rested on the researcher's own sum | **Applied**: the printed 75,181,071 (protocol and 98/1110-3) is cited; the transcript swaps two digits; the item is closed |
| B5 | The 2000 result text and protocol exist | **Applied**: RG 98/1110-3 with its annex and the CEC protocol with its index imported; both numbers recorded as printed; `published_date` null; RU-PRES-05 upgraded to accepted |
| B6 | Correction 106/1149-3 was missing | **Applied**: imported as a `result_correction` claim of 7 July 2000 |
| B7 | The three 2004 corrections were missing | **Applied**: imported as three sources with separate claims, plus a voting-day claim from 125/902-4; the footer inconsistency of 135/935-4 is recorded |
| B8 | Supplement 1924 has no publication stamp | **Applied**: `published_date` null; news 30888 imported as a ceremony claim |
| B9 | Settle on `from` for 2000 and 2004 | **Applied in part**: `from` settled and the fallback wording removed; news 38089 and 30888 support the day in the holder notes and on the role, not in the holders' claim IDs, because ceremony claims never feed a holder (A2) |
| B10 | Portal URLs were `http://` | **Applied**: `https://` packet URLs with the HTTP form in `source_response_url` |
| B11 | `original_url` lacked `www` | **Applied**: every `original_url` is derived from the capture URL |
| B12 | Live kremlin.ru alternates could not be re-verified | **Applied**: labelled as alternates, never identities, with this packet's retrieval result on each |
| B13 | "первый Президент Российской Федерации" stylings passed over | **Applied**: recorded in scope notes and the 2000 assumption claim as retrospective, not a tie or merge |
| B14 | Holder names normalised declined printed forms | **Applied**: rows keep the normalised `holder_name` and the printed form in `printed_name` |
| C1 | Medvedev's `until` rested on a pre-oath farewell | **Applied**: `until` null; the claim is an `outgoing_holder_statement` |
| C2 | Putin's 2008 farewell offered as an end for 2004 | **Applied**: relabelled; holder 3 has no `until` |
| C3 | The 15-17 March 2024 period was partly prospective | **Applied**: `attested_on` 2024-03-15 |
| C4 | `original_url` did not match the captured text versions | **Applied**: exact captured URLs |
| C5 | Decree 636's publication date rested on an unhashed card | **Applied**: the card is a hashed `card_response` and the claim's locator |
| C6 | "The portal offers no facsimile" was wrong for 442/528/678-SF | **Applied**: three facsimile responses, rendered and viewed; wording corrected |
| C7 | The 4 March 2012 voting day has a primary record | **Applied**: news 14680 imported, and news 57083 for 18 March 2018 |
| C8 | Annexes and gazette publications exist | **Applied**: RG publications of 2008, 2012 and 2018, the signed 2024 facsimile with its annex, and the CEC's 2018 annex; figures claims for 2008, 2018 and 2024 |
| C9 | Event-kind names differed from the model packet | **Applied**: `in_office_attestation`, `oath_of_office`, `assumption_of_office`; no end label on pre-oath remarks |

Missing primary records: all twelve found by check C, ten of the eleven found by check B (three of them as attached
index or annex responses) and both downloadable records found by check A are imported. Not imported: the
Rossiyskaya Gazeta copy of 4-SF (a copy, not the cited publication) and the Kodeks titles of 105/825-II and of a
result resolution (commercial database, no official copy).

## Suggested next work orders

These are proposals for the integrator. They are not created in `work-orders.json`.

- `C01-Russia-PRES-001`: the CEC resolution of 9 July 1996 declaring the run-off result, CEC resolution 105/825-II of
  20 June 1996, and a contemporaneous record of the 9 August 1996 oath (Rossiyskaya Gazeta of 10 August 1996, the CEC
  Vestnik, the Presidential Library). Could replace 134-O as the basis of the 1996 start.
- `C01-Russia-PRES-002`: an integrator ruling on the 1991-1996 period of `ru_president`: a dated observation from
  1992-1993 records, or a documented decision that `ru_rsfsr_president` and `ru_president` are one retitled office.
  The publication that brought Law 2708-I into force and its signed original belong here.
- `C01-Russia-PRES-003`: Rossiyskaya Gazeta of 25 December 1993 as printed, and the gazette pages cited on portal
  cards (Sobranie zakonodatelstva, Parlamentskaya Gazeta) for the decrees and Federation Council resolutions.
- `C01-Russia-PRES-004`: the 97/98 numbering of the result resolution of 5 April 2000 (CEC Vestnik 2000).
- `C01-Russia-PRES-005`: a source stating when the 1999-2000 acting service ended, and the minute of each oath.
- The Vice-President after 1993, the Chairman of the Government and Security Council offices are outside this packet.

## Integration notes (outside this packet's file boundary)

- **Stacking.** This branch is stacked on CLAUDE-C01-05 (`claude/c01-ussr-05` at `1c698ed0`, merged with integration
  `ffe54b02` at `0c9b5f19`). `claude/c01-ussr-05` was fetched again before this work and had no commits missing
  from this branch. Both packets edit `russia.json` and the two Russia tests, so CLAUDE-C01-05 must be integrated
  first; after that, this packet's diff is additive (a new role, 53 sources, seven institution coverage notes and one
  packet note).
- `research-index.json` is regenerated in a **separate commit**; it is the only file this packet shares with other
  pending packets (apart from the files it shares with CLAUDE-C01-05 through the stack). New totals: 234 sources and
  1,893 claims (previously 181 and 1,799). Russia's role observations go from 7 to 8 and its source claims from 48 to
  142; its entries (20), `mapping_pending` (20) and batches ([10, 10]) are unchanged. If another packet lands first,
  regenerate the index rather than merging it.
- `research/README.md`, the C01 README totals and `docs/planning/ai-workstreams.json` are left for the integrator;
  this handoff is self-proposed and not registered there.
- Existing tests updated, none loosened:
  - `test_russia_research_s10h.py`: totals 15→68 sources, 48→142 claims and 7→8 roles; role observations 7→8; source
    hosts pinned to the new exact set (adding `web.archive.org`, `transcript.duma.gov.ru` and
    `publication.pravo.gov.ru`); access dates pinned per packet (2 original, 13 C01-05, 53 C01-14); the cutoff check on
    claim dates now names the exact six undated claims instead of assuming every claim is dated.
  - `test_ussr_russia_transition_c01_05.py`: the "no end" guard is re-expressed to cover exactly the roles it was
    written for (every USSR Presidency role and the two RSFSR roles), because `ru_president` now has a source-stated
    end pinned by the new test; the role list is pinned to the three roles, with the two RSFSR roles still
    `institutional_office` and `ru_president` `head_of_state`.
- The new test `test_russia_presidents_c01_14.py` pins the holders, the claim classification, every claim's date,
  kind and observation, every response identity, the extracts, the separation from the C01-05 roles and the USSR
  packet, and 32 mutations.
- In the atlas, the RSFSR presidency gains a third office, "Президент Российской Федерации — President of the Russian
  Federation", with seven holder observations. No UI code changed; the Node check passes and no browser review was run.
- The sparse worktree lacks `spheres-sim/data`, so `test_campaign_census.py` errors in setup; the worktree was not
  widened.

## Checks

```text
python -X utf8 tools/avatars/campaign_research.py
python -X utf8 tools/avatars/campaign_research.py --check
python -X utf8 -m unittest discover -s tools/avatars -p "test_russia*.py"
python -X utf8 -m unittest discover -s tools/avatars -p "test_ussr*.py"
python -X utf8 -m unittest discover -s tools/avatars -p "test_*research*.py"
python -X utf8 -m unittest discover -s tools/avatars -p "test_campaign*.py"
node --test tools/ui/check_leadership_research_review.cjs
python tools/planning/workboard.py --check
git diff --check (this packet's paths)
```

All passed on 24 September 2026 (local): the exact index regeneration and check (234 sources, 1,893 claims); 19
Russia tests (9 of them new) and 18 USSR tests; 79 research tests; 9 campaign research tests, with
`test_campaign_census.py` erroring in setup because the sparse worktree has no `spheres-sim/data` (expected; not
widened); 11 atlas Node tests; the workboard check (44 markers); `git diff --check` on this packet's staged paths.
The new test's 32 mutations each fail as intended: successor starts, a re-election and the latest attestation used
as ends; election, declaration and designation dates used as starts; acting service and the 1993 continuation added
as holders; acting, ceremony, election and outgoing-statement claims cited by holders; cross-role and
cross-institution holders; the 1999 end moved onto the C01-05 holder; a cross-role claim; an undated claim given a
date; an oath re-dated to the declaration; a second head-of-state role; checksum mismatches; beyond-cutoff dates; an
HTTP source URL; and a reversed interval.
