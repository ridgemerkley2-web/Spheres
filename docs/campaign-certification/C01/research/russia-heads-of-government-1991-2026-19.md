# Russian heads of government 1991-2026 19: the Chairman of the Government from the 1991 reorganisation to the 2024 appointment

Packet: **CLAUDE-C01-19**. State: **ready_for_review** (not complete).
Owner: Claude. Integrator/reviewer: Codex. Branch `claude/c01-ru-19`; claim commit `cb64061a` on `1d749e15`, the head of
`claude/c01-ru-14`. **Stacked on CLAUDE-C01-14** (itself stacked on CLAUDE-C01-05): merge that packet first. Research
access: 25 September 2026 (UTC); this packet's two re-download passes ran at 2026-09-25T14:18:26Z-14:25:24Z and
2026-09-25T14:52:07Z-14:59:23Z (UTC). The historical cutoff stays **7 September 2026**.

This packet reviews ten observations, RU-GOV-01 to RU-GOV-10, in [russia.json](russia.json). It adds one institution,
`ru_government` ("Правительство Российской Федерации — Government of the Russian Federation", kind
`executive_institution`, lifecycle `unknown`), with one role, `ru_government_chairman` ("Председатель Правительства —
Chairman of the Government", kind `head_of_government`), fifteen holder observations, 102 sources and 153 claims: 151 on
the new sources and two added to the CLAUDE-C01-14 source of decree 1146 (the only existing extract edited). It does not
change the presidency holders of CLAUDE-C01-05 and C01-14, the presidency's institution-level records or `ussr.json`,
and it adds no organization, game mapping, portrait or avatar. The parent scope (C01, C06, S23, WC1 and CP1) remains
open.

Three research dossiers (parts A, B and C) and an independent adversarial check of each were prepared before this
packet was written. Every checker defect is applied, applied in part, resolved by removal or declined with a reason (see
[Checker defects](#checker-defects)). Every missing primary record the checks found is imported, except four State
Duma website news items (press-service reports, recorded as leads) and one Government order downloaded only once. The
stenogram fragment of 10 May 2024 that check C found also dates the 2024 submission, which neither the dossier nor the
check had noticed.

## Outcome

| ID | Question | Decision |
|---|---|---|
| RU-GOV-01 | Who headed the RSFSR/Russian Government from November 1991, and any acting Chairman in 1992 | **Accepted:** decrees 171 and 172 (6 Nov 1991, on Congress resolution 1830-I) put the Government under the President, who heads it; its listed composition has no Chairman; resolution 8 is signed "Б. ЕЛЬЦИН". Decree 633 (15 Jun 1992) entrusts the Chairman's duties to Гайдар, resolution 457 is signed "Е. Гайдар", decree 1570 (15 Dec 1992) releases him, and decree 1569 states that the post was vacant until 14 Dec 1992. All claims; no holder |
| RU-GOV-02 | December 1992: Chernomyrdin's appointment after the Congress's decision, and his 1996 reappointment | **Accepted in part:** Congress resolutions 4063-I, 4079-I and 4088-I ("утвердить", 14 Dec 1992) and decree 1567 (holder 1, `attested_on` 14 Dec 1992); decree 1146 (9 Aug 1996), the President's letter as presented and the 314-85-3 vote (10 Aug), resolution 624-II ГД and decree 1152 (holder 2, `attested_on` 10 Aug 1996). The dates of the President's letters and the 9 Dec 1992 ballot result were not found |
| RU-GOV-03 | 1998: Chernomyrdin's dismissal and Kiriyenko's appointment | **Accepted in part:** decree 281 (the Government's resignation announced), 287 and 288 (Кириенко acting); three presentations (10, 17, 24 Apr), two failed votes and the 251-25 secret ballot; resolutions 2375-II, 2402-II and 2421-II ГД; decree 436 (holder 3, `attested_on` 24 Apr 1998). No source names Chernomyrdin at the dismissal or states his end |
| RU-GOV-04 | 1998: Kiriyenko's dismissal and Primakov's appointment | **Accepted in part:** decree 983 (resignation announced; Черномырдин acting) and decree 987 (members continue); the presentations and failed votes of 31 Aug (94-251) and 7 Sep (138-273) and resolutions 2898-II and 2928-II ГД; the presentation and 315-63-15 vote of 11 Sep, resolution 2961-II ГД and decree 1087 (holder 4, `attested_on` 11 Sep 1998). No source states Kiriyenko's end |
| RU-GOV-05 | 1999: Primakov's dismissal, Stepashin's appointment and dismissal, Putin's appointment | **Accepted in part:** decree 580 and the Duma's statement 3961-II (12 May); Степашин acting (580; resolution 528); consent 3965-II and decree 611 (holder 5, `attested_on` 19 May 1999, no effect clause); resolution 905 signed by him as Chairman (6 Aug); decree 1012 (9 Aug; Путин acting; resolution 923); consent 4276-II and decree 1052 (holder 6, `from` 16 Aug 1999). The nomination dates and Primakov's and Stepashin's ends are not stated |
| RU-GOV-06 | 2000-2004: Kasyanov's appointment and dismissal | **Accepted:** decrees 834 and 836 and order 647-r (7 May 2000); the nomination (10 May), consent 363-III, decree 861 (holder 7, `from` 17 May 2000) and the certificate; decree 264, the President's statement, Христенко acting (264; resolution 104) and the "former Chairman" styling, all claims; no `until` |
| RU-GOV-07 | 2004-2007: Fradkov's appointment, reappointment and resignation | **Accepted in part:** the 1 Mar proposal, consent 162-IV and decree 300 (holder 8, `from` 5 Mar 2004); order 608-r (signed by him as Chairman) and decree 585 (7 May); acting signatures of 8 and 11 May; the nomination, consent 489-IV and decree 610 (holder 9, `from` 12 May 2004); the resignation request and acceptance, decree 1184 and resolution 586 (12-13 Sep 2007). No `until`: an integrator ruling is requested on the 2007 acceptance; the March 2004 letter was not found |
| RU-GOV-08 | 2007-2008: Zubkov's appointment and Putin's 2008 appointment | **Accepted:** nomination (12 Sep 2007), consent 5066-4 and decree 1202 (holder 10, `from` 14 Sep 2007); order 680-r signed by Зубков as Chairman and decree 717 (7 May 2008); the nomination, the Kremlin's 392-56 report, consent 458-5 and decree 723 (holder 11, `from` 8 May 2008) |
| RU-GOV-09 | 2012-2020: Medvedev's appointments and the Government's 2020 resignation | **Accepted:** order 760-r signed by acting Chairman "В.Зубков" and decree 607 (7 May 2012); the nomination, the 299-144 report, consent 323-6 and decree 612 (holder 12, `from` 8 May 2012); order 875-r and decree 202 (7 May 2018); nomination, consent 3894-7 and decree 209 (holder 13, `from` 8 May 2018); the 15 Jan 2020 proposal and decree 14 (resignation announced; Медведев acting). No `until` |
| RU-GOV-10 | 2020-2026: Mishustin's appointments and an attestation before the cutoff | **Accepted:** nomination (15 Jan 2020), consent 7565-7 and decree 17 (holder 14, `from` 16 Jan 2020); order 1121-r and decree 306 (7 May 2024); the submission of 9 May 2024 (stated at the Duma), the acting styling, the 375-0-57 vote, resolution 6061-8 ("утвердить") and decree 319 (holder 15, `from` 10 May 2024); resolution 1125 (3 Sep 2026) attests him in office |

### Holders on `ru_government_chairman`

| # | Name | `attested_on` | `from` | `until` | Claims |
|---|---|---|---|---|---|
| 1 | Виктор Степанович Черномырдин | 1992-12-14 | null | null | `ru_ukaz_1567_chernomyrdin_appointed_19921214` |
| 2 | Виктор Степанович Черномырдин | 1996-08-10 | null | null | `ru_ukaz_1152_chernomyrdin_appointed_19960810` |
| 3 | Сергей Владиленович Кириенко | 1998-04-24 | null | null | `ru_ukaz_436_kiriyenko_appointed_19980424` |
| 4 | Евгений Максимович Примаков | 1998-09-11 | null | null | `ru_ukaz_1087_primakov_appointed_19980911` |
| 5 | Сергей Вадимович Степашин | 1999-05-19 | null | null | `ru_ukaz_611_stepashin_appointed_19990519`, `ru_gov_res_905_stepashin_signs_as_chairman_19990806` |
| 6 | Владимир Владимирович Путин | null | 1999-08-16 | null | `ru_ukaz_1052_putin_appointed_19990816` |
| 7 | Михаил Михайлович Касьянов | null | 2000-05-17 | null | `ru_ukaz_861_kasyanov_appointed_20000517` |
| 8 | Михаил Ефимович Фрадков | null | 2004-03-05 | null | `ru_ukaz_300_fradkov_appointed_20040305`, `ru_gov_608r_fradkov_signs_as_chairman_20040507` |
| 9 | Михаил Ефимович Фрадков | null | 2004-05-12 | null | `ru_ukaz_610_fradkov_appointed_20040512` |
| 10 | Виктор Алексеевич Зубков | null | 2007-09-14 | null | `ru_ukaz_1202_zubkov_appointed_20070914`, `ru_gov_680r_zubkov_signs_as_chairman_20080507` |
| 11 | Владимир Владимирович Путин | null | 2008-05-08 | null | `ru_ukaz_723_putin_appointed_20080508` |
| 12 | Дмитрий Анатольевич Медведев | null | 2012-05-08 | null | `ru_pub_ukaz_612_medvedev_appointed_20120508`, `ru_pub_gov_875r_medvedev_signs_as_chairman_20180507` |
| 13 | Дмитрий Анатольевич Медведев | null | 2018-05-08 | null | `ru_pub_ukaz_209_medvedev_appointed_20180508` |
| 14 | Михаил Владимирович Мишустин | null | 2020-01-16 | null | `ru_pub_ukaz_17_mishustin_appointed_20200116`, `ru_pub_gov_1121r_mishustin_signs_as_chairman_20240507` |
| 15 | Михаил Владимирович Мишустин | null | 2024-05-10 | null | `ru_pub_ukaz_319_mishustin_appointed_20240510`, `ru_pub_gov_res_1125_mishustin_signs_as_chairman_20260903` |

A `from` rests on an appointment decree that states "Настоящий Указ вступает в силу со дня его подписания" (in force on
signing), dated that day: decrees 1052 (1999) to 319 (2024). Decrees 1567, 1152, 436, 1087 and 611 have no effect clause,
so those five holders carry `attested_on` only. The Congress's approval, the Duma's consent or approval, the vote and the
nomination are separate claims and never the start. The second claim of six holders is an in-office signature as
"Председатель Правительства" (resolution 905, orders 608-r, 680-r, 875-r and 1121-r, resolution 1125): an attestation,
never a boundary.

**No holder has an `until`.** No reviewed source states the day a Chairman's office ended: the decrees announcing the
Government's resignation (281, 983, 580, 1012, 264, 1184, 14) name no outgoing Chairman or entrust him with acting duties;
the Government's orders laying down its powers concern the body; the Duma's statement of 1999, kremlin.ru's "former
Chairman" styling of 2004 (a page last updated in 2015) and the President's acceptance of Fradkov's resignation in 2007
state no end; and no end is taken from a successor's appointment. The dossiers' proposed ends (Kasyanov 24 Feb 2004,
Fradkov 7 May 2004 and 12 Sep 2007, Medvedev 15 Jan 2020) are therefore not set, one rule for every holder; an integrator
ruling is requested for 12 Sep 2007 (see [Suggested next work orders](#suggested-next-work-orders)). In the atlas the ten
decree-dated observations read "Reported interval: <day> → Not established" and the five others show their attested day.

Names follow the stacked packet's convention: `holder_name` is the canonical given-name, patronymic, surname form (the
C01-14 form for Путин and Медведев, so the same person has one spelling across roles), and each extract row keeps the
printed form and case in `printed_name` (for example "Путина Владимира Владимировича (accusative)", "С.Степашин",
"Е.М.Примаковым (instrumental; initials and surname only)"). Христенко is printed only as "Христенко В.Б." and
"В.Христенко", so no fuller form is given. Rows that name nobody carry `holder_name` null and no `printed_name`.

### Date ledger

Each row is a separate dated fact with its own claim. The nomination (as dated by the President's website or stated at a
sitting), its presentation, the parliament's consent, approval or refusal and the vote, the appointment decree, the
Government's resignation, dismissal or laying down of powers, continued duties, acting service and attestations are not
merged, and different dates for them are not inconsistencies. `attested_on` is the date of the act or of the event the
source dates.

| Date | Events | Holder field and claims |
|---|---|---|
| 1 Nov 1991 | reorganisation authorised | `ru_cpd_1830i_president_reorganizes_executive_19911101` |
| 6 Nov 1991 | President heads the Government; continued duties | `ru_ukaz_171_president_heads_government_19911106`, `ru_ukaz_171_council_of_ministers_continues_19911106`, `ru_ukaz_172_president_heads_government_19911106` |
| 15 Nov 1991 | Government act signed | `ru_gov_res_8_signed_yeltsin_19911115` |
| 15 Jun 1992 | acting designation | `ru_ukaz_633_gaidar_acting_chairman_19920615` |
| 1 Jul 1992 | Government act signed | `ru_gov_res_457_signed_gaidar_19920701` |
| 9 Dec 1992 | ballot results approved | `ru_cpd_4063i_secret_ballot_results_approved_19921209` |
| 12 Dec 1992 | selection procedure | `ru_cpd_4079i_chairman_selection_procedure_19921212` |
| 14 Dec 1992 | approval; **appointment** | holder 1 `attested_on`; `ru_cpd_4088i_chernomyrdin_approved_19921214`, `ru_ukaz_1567_chernomyrdin_appointed_19921214` |
| 15 Dec 1992 | vacancy statement; continued duties; release from acting service | `ru_ukaz_1569_chairman_post_vacant_19921215`, `ru_ukaz_1569_current_government_continues_19921215`, `ru_ukaz_1570_gaidar_released_from_acting_19921215` |
| 9 Aug 1996 | Government's resignation accepted; continued duties | `ru_ukaz_1146_resignation_statement_accepted_19960809`, `ru_ukaz_1146_government_continues_19960809` |
| 10 Aug 1996 | nomination presented; vote; consent; **appointment** | holder 2 `attested_on`; `ru_duma_steno_nomination_letter_presented_19960810`, `ru_duma_steno_consent_vote_19960810`, `ru_gd_624ii_consent_chernomyrdin_19960810`, `ru_ukaz_1152_chernomyrdin_appointed_19960810` |
| 23 Mar 1998 | Government's resignation announced; continued duties; acting designation; acting designation revoked | `ru_ukaz_281_government_dismissed_19980323`, `ru_ukaz_281_members_continue_19980323`, `ru_ukaz_281_president_assumes_acting_duties_19980323`, `ru_ukaz_287_acting_point_repealed_19980323`, `ru_ukaz_288_kiriyenko_acting_19980323` |
| 10 Apr 1998 | nomination presented; vote; consent refused | `ru_duma_steno_kiriyenko_submitted_19980410`, `ru_duma_steno_kiriyenko_consent_vote_failed_19980410`, `ru_gd_2375ii_kiriyenko_rejected_19980410` |
| 17 Apr 1998 | nomination presented; vote; nomination status; consent refused | `ru_duma_steno_kiriyenko_resubmitted_19980417`, `ru_duma_steno_kiriyenko_consent_vote_failed_19980417`, `ru_duma_steno_third_submission_not_received_19980417`, `ru_gd_2402ii_kiriyenko_rejected_19980417` |
| 24 Apr 1998 | nomination presented; vote; consent; **appointment** | holder 3 `attested_on`; `ru_duma_steno_kiriyenko_third_submission_presented_19980424`, `ru_duma_steno_kiriyenko_secret_ballot_result_19980424`, `ru_gd_2421ii_consent_kiriyenko_19980424`, `ru_ukaz_436_kiriyenko_appointed_19980424` |
| 23 Aug 1998 | Government's resignation announced; acting designation | `ru_ukaz_983_government_dismissed_19980823`, `ru_ukaz_983_chernomyrdin_acting_19980823` |
| 25 Aug 1998 | continued duties | `ru_ukaz_987_members_continue_19980825` |
| 31 Aug 1998 | nomination presented; vote; consent refused | `ru_duma_steno_chernomyrdin_first_submission_presented_19980831`, `ru_duma_steno_chernomyrdin_consent_vote_failed_19980831`, `ru_gd_2898ii_chernomyrdin_rejected_19980831` |
| 7 Sep 1998 | nomination presented; acting attestation; vote; consent refused | `ru_duma_steno_chernomyrdin_resubmitted_19980907`, `ru_duma_steno_chernomyrdin_styled_acting_19980907`, `ru_duma_steno_chernomyrdin_consent_vote_failed_19980907`, `ru_gd_2928ii_chernomyrdin_rejected_19980907` |
| 11 Sep 1998 | nomination presented; vote; consent; **appointment** | holder 4 `attested_on`; `ru_duma_steno_primakov_submitted_19980911`, `ru_duma_steno_primakov_consent_vote_19980911`, `ru_gd_2961ii_consent_primakov_19980911`, `ru_ukaz_1087_primakov_appointed_19980911` |
| 12 May 1999 | Government's resignation announced; acting designation; continued duties; dismissal reference | `ru_ukaz_580_government_dismissal_announced_19990512`, `ru_ukaz_580_stepashin_acting_19990512`, `ru_ukaz_580_members_continue_19990512`, `ru_duma_3961ii_primakov_government_dismissed_19990512` |
| 13 May 1999 | acting attestation | `ru_gov_res_528_stepashin_signs_as_acting_19990513` |
| 19 May 1999 | consent; **appointment** | holder 5 `attested_on`; `ru_duma_3965ii_consent_stepashin_19990519`, `ru_ukaz_611_stepashin_appointed_19990519` |
| 6 Aug 1999 | in-office attestation | `ru_gov_res_905_stepashin_signs_as_chairman_19990806` |
| 9 Aug 1999 | Government's resignation announced; acting designation; continued duties | `ru_ukaz_1012_government_dismissal_announced_19990809`, `ru_ukaz_1012_putin_acting_19990809`, `ru_ukaz_1012_members_continue_19990809` |
| 10 Aug 1999 | acting attestation | `ru_gov_res_923_putin_signs_as_acting_19990810` |
| 16 Aug 1999 | consent; **appointment** | holder 6 `from`; `ru_duma_4276ii_consent_putin_19990816`, `ru_ukaz_1052_putin_appointed_19990816` |
| 7 May 2000 | powers laid down (Government order); acting attestation; acting designation; powers laid down (decree recital); continued duties | `ru_gov_647r_government_surrenders_powers_20000507`, `ru_gov_647r_kasyanov_signs_as_acting_20000507`, `ru_ukaz_834_kasyanov_acting_20000507`, `ru_ukaz_836_government_surrendered_powers_20000507`, `ru_ukaz_836_government_to_continue_20000507` |
| 10 May 2000 | nomination | `ru_kremlin_putin_nominates_kasyanov_20000510` |
| 17 May 2000 | consent; **appointment**; certificate | holder 7 `from`; `ru_duma_363iii_consent_kasyanov_20000517`, `ru_ukaz_861_kasyanov_appointed_20000517`, `ru_kremlin_kasyanov_certificate_presented_20000517` |
| 24 Feb 2004 | Government's resignation announced; acting designation; continued duties; decision statement; nomination intention; former-holder styling | `ru_ukaz_264_government_dismissal_announced_20040224`, `ru_ukaz_264_khristenko_acting_20040224`, `ru_ukaz_264_government_to_continue_20040224`, `ru_kremlin_putin_statement_dismissal_decision_20040224`, `ru_kremlin_putin_statement_nomination_intention_20040224`, `ru_kremlin_putin_statement_government_continue_20040224`, `ru_kremlin_kasyanov_styled_former_chairman_20040224` |
| 25 Feb 2004 | acting attestation | `ru_gov_res_104_khristenko_signs_as_acting_20040225` |
| 1 Mar 2004 | nomination proposal | `ru_kremlin_putin_proposes_fradkov_20040301` |
| 5 Mar 2004 | consent; **appointment** | holder 8 `from`; `ru_duma_162iv_consent_fradkov_20040305`, `ru_ukaz_300_fradkov_appointed_20040305` |
| 7 May 2004 | powers laid down (Government order); in-office attestation; powers laid down (decree recital); continued duties; powers laid down (report); continued duties (report); nomination intention; nomination | `ru_gov_608r_government_surrenders_powers_20040507`, `ru_gov_608r_fradkov_signs_as_chairman_20040507`, `ru_ukaz_585_government_surrendered_powers_20040507`, `ru_ukaz_585_government_to_continue_20040507`, `ru_kremlin_premier_order_received_20040507`, `ru_kremlin_decree_585_signing_reported_20040507`, `ru_kremlin_putin_to_submit_fradkov_20040507`, `ru_kremlin_putin_nominates_fradkov_20040507` |
| 8 May 2004 | acting attestation | `ru_gov_res_232_fradkov_signs_as_acting_20040508` |
| 11 May 2004 | acting attestation | `ru_gov_res_233_fradkov_signs_as_acting_20040511` |
| 12 May 2004 | consent; **appointment** | holder 9 `from`; `ru_duma_489iv_consent_fradkov_20040512`, `ru_ukaz_610_fradkov_appointed_20040512` |
| 12 Sep 2007 | Government's resignation announced; acting designation; continued duties; resignation request; resignation acceptance; request to act; nomination | `ru_ukaz_1184_government_resignation_announced_20070912`, `ru_ukaz_1184_fradkov_acting_20070912`, `ru_ukaz_1184_government_to_continue_20070912`, `ru_kremlin_fradkov_requests_resignation_20070912`, `ru_kremlin_putin_accepts_resignation_20070912`, `ru_kremlin_putin_asks_fradkov_to_act_20070912`, `ru_kremlin_steno_fradkov_asks_resignation_20070912`, `ru_kremlin_steno_putin_accepts_resignation_20070912`, `ru_kremlin_steno_putin_asks_fradkov_to_act_20070912`, `ru_kremlin_42319_zubkov_nominated_20070912` |
| 13 Sep 2007 | acting attestation | `ru_gov_res_586_fradkov_signs_as_acting_20070913` |
| 14 Sep 2007 | consent; **appointment**; consent (report); appointment (report) | holder 10 `from`; `ru_duma_5066_4_consent_zubkov_20070914`, `ru_ukaz_1202_zubkov_appointed_20070914`, `ru_kremlin_42319_duma_consent_reported_20070914`, `ru_kremlin_42319_appointment_decree_signed_20070914` |
| 7 May 2008 | powers laid down (Government order); in-office attestation; powers laid down (decree recital); continued duties; nomination | `ru_gov_680r_government_powers_laid_down_20080507`, `ru_gov_680r_zubkov_signs_as_chairman_20080507`, `ru_ukaz_717_government_powers_laid_down_20080507`, `ru_ukaz_717_government_to_continue_20080507`, `ru_kremlin_6_putin_nominated_20080507` |
| 8 May 2008 | vote (report); consent; **appointment** | holder 11 `from`; `ru_kremlin_20_duma_vote_putin_20080508`, `ru_duma_458_5_consent_putin_20080508`, `ru_ukaz_723_putin_appointed_20080508` |
| 7 May 2012 | powers laid down (Government order); acting attestation; powers laid down (decree recital); continued duties; nomination | `ru_gov_760r_government_powers_laid_down_20120507`, `ru_gov_760r_zubkov_signs_as_acting_20120507`, `ru_pub_ukaz_607_government_powers_laid_down_20120507`, `ru_pub_ukaz_607_government_to_continue_20120507`, `ru_kremlin_15230_medvedev_nominated_20120507` |
| 8 May 2012 | vote (report); consent; **appointment** | holder 12 `from`; `ru_kremlin_15266_duma_vote_medvedev_20120508`, `ru_pub_duma_323_6_consent_medvedev_20120508`, `ru_pub_ukaz_612_medvedev_appointed_20120508` |
| 7 May 2018 | powers laid down (Government order); in-office attestation; powers laid down (decree recital); continued duties; nomination | `ru_pub_gov_875r_government_powers_laid_down_20180507`, `ru_pub_gov_875r_medvedev_signs_as_chairman_20180507`, `ru_pub_ukaz_202_government_powers_laid_down_20180507`, `ru_pub_ukaz_202_government_to_continue_20180507`, `ru_kremlin_57422_medvedev_nominated_20180507` |
| 8 May 2018 | consent; **appointment** | holder 13 `from`; `ru_pub_duma_3894_7_consent_medvedev_20180508`, `ru_pub_ukaz_209_medvedev_appointed_20180508` |
| 15 Jan 2020 | resignation proposed; continued duties; Government's resignation announced; acting designation; nomination | `ru_kremlin_62585_medvedev_proposes_resignation_20200115`, `ru_kremlin_62585_putin_asks_duties_continue_20200115`, `ru_pub_ukaz_14_government_resignation_announced_20200115`, `ru_pub_ukaz_14_medvedev_acting_chairman_20200115`, `ru_pub_ukaz_14_government_to_continue_20200115`, `ru_kremlin_62586_mishustin_nominated_20200115` |
| 16 Jan 2020 | consent; **appointment** | holder 14 `from`; `ru_pub_duma_7565_7_consent_mishustin_20200116`, `ru_pub_ukaz_17_mishustin_appointed_20200116` |
| 7 May 2024 | powers laid down (Government order); in-office attestation; powers laid down (decree recital); continued duties | `ru_pub_gov_1121r_government_powers_laid_down_20240507`, `ru_pub_gov_1121r_mishustin_signs_as_chairman_20240507`, `ru_pub_ukaz_306_government_powers_laid_down_20240507`, `ru_pub_ukaz_306_government_to_continue_20240507` |
| 9 May 2024 | nomination | `ru_duma_steno_mishustin_submitted_20240509` |
| 10 May 2024 | acting attestation; nomination reference; vote; approval; **appointment** | holder 15 `from`; `ru_kremlin_74009_mishustin_styled_acting_20240510`, `ru_kremlin_74009_nomination_submitted_20240510`, `ru_duma_steno_mishustin_approval_vote_20240510`, `ru_pub_duma_6061_8_approval_mishustin_20240510`, `ru_pub_ukaz_319_mishustin_appointed_20240510` |
| 3 Sep 2026 | in-office attestation | `ru_pub_gov_res_1125_mishustin_signs_as_chairman_20260903` |

### Identifier changes from the dossiers

| Dossier | This packet | Why |
|---|---|---|
| `ru_ukaz_1146_government_resignation_accepted_19960809` | `ru_ukaz_1146_resignation_statement_accepted_19960809` | The dossier's ID is a stale CLAUDE-C01-14 identifier that test_russia_presidents_c01_14 forbids |
| claims of the live `ru_duma_steno_19960810` (C01-14) | the same claim IDs on the new source `ru_duma_steno_ia_19960810` | Frozen capture instead of the live page, so the C01-14 extract is not edited (A1, A10) |
| `http://web.archive.org/...` URLs (part A) | `https://web.archive.org/web/<ts>id_/...` | A2 |
| `ru_kremlin_premier_order_government_surrender_20040507` | `ru_kremlin_premier_order_received_20040507` and `ru_kremlin_decree_585_signing_reported_20040507` | Collapsed events (B10) |
| `ru_kremlin_putin_accepts_resignation_20070912`, `ru_kremlin_steno_putin_accepts_resignation_20070912` | the same IDs (acceptance only) plus `ru_kremlin_putin_asks_fradkov_to_act_20070912` and `ru_kremlin_steno_putin_asks_fradkov_to_act_20070912` | Collapsed events (B9) |
| `ru_gov_608r_government_surrenders_powers_20040507` | the same ID (the body's act) plus `ru_gov_608r_fradkov_signs_as_chairman_20040507` | The signature is an in-office attestation, separate from the laying down of powers |
| printed `holder_name` with `holder_name_normalized` (A); surname-first `holder_name` (B, C) | canonical `holder_name` and `printed_name` | A3, B4, C6 |
| event kinds `nomination` (sitting-day presentations), `acting_government_act_signature`, `appointment_decree`, `continuation`, `government_dismissal`, `government_resignation`, `government_powers_surrendered`, `resignation_tendered`, `parliamentary_consent_vote`, and the Kremlin reports' act kinds | one vocabulary: `nomination_presented`, `government_act_signature`, `appointment`, `continuation_of_duties`, `government_resignation_announced`, `government_powers_laid_down` (orders) and `government_powers_laid_down_recital` (decrees), `resignation_proposed`, `parliamentary_vote`, `appointment_reported`, `parliamentary_consent_reported`, `parliamentary_vote_reported` | A6, A7, C8, C9 |
| (none) | `ru_ukaz_987_*`, `ru_duma_steno_19980424`, `ru_duma_steno_19980831`, resolutions 528, 905, 923, 104, 232, 233 and 586, orders 680-r, 760-r, 875-r and 1121-r, `ru_duma_steno_20240510` | Primary records the checks found (A8, A9, B2, B15, C1, C2, C4) |

## Observations

### RU-GOV-01 — The 1991 reorganisation and the acting Chairman of 1992

Evidence (all original-edition texts on the official legal portal):

- Congress resolution 1830-I of 1 November 1991 lets the President of the RSFSR decide on his own the reorganisation of
  the highest executive bodies until the law on the Council of Ministers is adopted (point 2), in force on adoption and
  until 1 December 1992; it prints no ordinal for the Congress (A4).
- Decree 171 of 6 November 1991: for the period of the economic reform "Правительство РСФСР возглавляет Президент РСФСР";
  the Council of Ministers continues until the new Government is formed. Decree 172 of the same day forms the Government
  under the President, who conducts its sittings and signs its decisions; the listed composition begins with the First
  Deputy Chairman and has no Chairman. Both are signed "Президент РСФСР Б. ЕЛЬЦИН".
- Government resolution 8 of 15 November 1991 is signed "Б. ЕЛЬЦИН" with no printed capacity.
- Decree 633 of 15 June 1992 entrusts the performance of the Chairman's duties to "Гайдара Егора Тимуровича";
  resolution 457 of 1 July 1992 is signed "Е. Гайдар" with no printed title (a neutral kind, A6); decree 1570 of 15
  December 1992 releases him from the post of First Deputy Chairman "исполняющего обязанности Председателя
  Правительства".
- Decree 1569 of 15 December 1992 recites that "до 14 декабря 1992 г. должность Председателя Совета Министров -
  Правительства Российской Федерации оставалась вакантной".

Decision: accepted, as claims only. The President headed the Government ex officio and no one held the title of
Chairman; Gaidar only acted. The vacancy recital is a statement made on 15 December about the past, not a statement that
anyone assumed office on 14 December. None of these is a holder, and none dates or ends one.

Limits: when the 1991 arrangement ended is not stated (the portal cards mark decrees 171 and 172 "Утратил силу" without
the repealing act); other resolutions signed by Yeltsin or Gaidar are not surveyed.

### RU-GOV-02 — Chernomyrdin, December 1992 and August 1996

Evidence:

- Congress resolution 4063-I (9 December 1992) approves the results of the secret ballot on the Chairman question and
  names nobody; 4079-I (12 December) sets the procedure for the President's candidates (a rating vote on 14 December and
  approval of one of the top three); 4088-I (14 December) resolves "Утвердить Председателем Совета Министров Российской
  Федерации Черномырдина Виктора Степановича". Decree 1567 of the same day appoints him Chairman of the Council of
  Ministers - Government; decree 1569 (15 December) keeps the current composition acting.
- Decree 1146 of 9 August 1996 (a CLAUDE-C01-14 source; two new rows) accepts the Government's statement resigning its
  powers and instructs it to continue acting until a new Government is formed.
- The State Duma stenogram of 10 August 1996, read from the frozen capture of 28 May 2024: G. N. Seleznev reads the
  President's letter "вношу предложение о кандидатуре Черномырдина Виктора Степановича ... и прошу дать согласие на его
  назначение"; E. B. Mizulina says less than a day had passed since the submission; the secret vote at 16:21:54 is 314
  for, 85 against, 3 abstaining, "Результат: принято"; Chernomyrdin speaks of what follows "после того как будет указ о
  моем назначении".
- Resolution 624-II ГД gives consent; decree 1152 of the same day appoints him Chairman of the Government.

Decision: **accepted in part**. Holders 1 and 2, Виктор Степанович Черномырдин, `attested_on` 14 December 1992 and 10
August 1996, each on its appointment decree (no effect clause, so no `from`). The 1992 act of the Congress is an approval
("утвердить"), kept separate from the decree of the same day. The two observations are not joined and neither has an
end: decree 1146 names no Chairman, and the 1996 reappointment is not used as an end of the 1992 one.

Limits: the President's letters of 1992 and 1996 and their dates, the candidate and result of the 9 December 1992 ballot,
the 14 December rating vote and a Congress stenogram were not found; the 1993 change of title is not reviewed.

### RU-GOV-03 — March-April 1998: Chernomyrdin's dismissal and Kiriyenko

Evidence:

- Decree 281 of 23 March 1998 announces the resignation of the Government ("Объявляю об отставке Правительства"), in
  force on signing; point 2 asks the members to continue; point 3 of the original edition has the President assume the
  Chairman's temporary duties himself. Decree 287 of the same day voids point 3; decree 288 entrusts the duties to First
  Deputy Chairman "Кириенко Сергею Владиленовичу", in force on signing.
- Stenograms (raw captures) of 10 and 17 April 1998: A. A. Kotenkov presents the first and second submissions; the
  consent votes fail (143-186-5 by secret ballot; 115-271-11 by open vote); on 17 April the Chairman says the third letter
  had not yet arrived. Resolutions 2375-II and 2402-II ГД reject the candidate.
- The live stenogram of 24 April 1998 (check find, A9): Kotenkov presents the third submission under Article 111(3);
  Counting Commission protocol No. 4 records 315 ballots issued, 276 found, "за - 251, против - 25", approved at
  16:09:29 (342-9-1). Resolution 2421-II ГД gives consent by secret ballot; decree 436 appoints him.

Decision: **accepted in part**. Holder 3, Сергей Владиленович Кириенко, `attested_on` 24 April 1998 on decree 436. The
three presentations are `nomination_presented` claims (the day of each letter is not given, A7), the votes and the
resolutions are separate, and his acting service is claims only. Decree 281 names no Chairman, so the 1996 Chernomyrdin
observation has no end.

Limits: the President's letters, the order of signing of decrees 281, 287 and 288, and any end of Chernomyrdin's office.
The 24 April page carries the live footer year (see [Response identities](#response-identities-and-stability-checks)).

### RU-GOV-04 — August-September 1998: Kiriyenko's dismissal and Primakov

Evidence:

- Decree 983 of 23 August 1998 announces the Government's resignation and entrusts the Chairman's temporary duties to
  "Черномырдина Виктора Степановича", in force on signing; decree 987 of 25 August (check find, A8) instructs the deputy
  chairmen and federal ministers to continue until a new Government is formed.
- The live stenogram of 31 August (check find): Kotenkov presents Chernomyrdin's first 1998 submission (citing "пунктом
  'г' статьи 83" as printed); the open vote at 18:58:22 is 94 for, 251 against. Resolution 2898-II ГД rejects him.
- The captured stenogram of 7 September: the second submission; Kotenkov calls Chernomyrdin "исполняющего обязанности
  Председателя Правительства"; the vote at 19:16:32 is 138-273-1. Resolution 2928-II ГД rejects him.
- The captured stenogram of 11 September: after the break, with Г.Н. Селезнев in the chair (A5), Kotenkov presents
  Primakov's candidacy; the vote at 18:48:33 is 315-63-15. Resolution 2961-II ГД consents; decree 1087 appoints him.

Decision: **accepted in part**. Holder 4, Евгений Максимович Примаков, `attested_on` 11 September 1998. Chernomyrdin's
acting service (from 23 August, attested on 7 September) is claims only and gives no holder. Decree 983 names no
Chairman, so Kiriyenko's observation has no end.

Limits: the President's letters; the end of Chernomyrdin's acting service; any end of Kiriyenko's office.

### RU-GOV-05 — 1999: Primakov, Stepashin and Putin

Evidence:

- Decree 580 of 12 May 1999 announces the Government's resignation, entrusts the Chairman's duties to First Deputy
  Chairman "Степашина Сергея Вадимовича" and asks the members to continue. The State Duma's statement of the same day
  (resolution 3961-II) objects that the President dismissed the Government "во главе с Е.М.Примаковым". Government
  resolution 528 of 13 May (check find) is signed "И. о. Председателя Правительства Российской Федерации С.Степашин".
- Resolution 3965-II ГД consents on 19 May; decree 611 appoints him (no effect clause). Resolution 905 of 6 August (check
  find, B15) is signed "Председатель Правительства Российской Федерации С.Степашин".
- Decree 1012 of 9 August announces the Government's resignation and entrusts the duties to First Deputy Chairman
  "Путина Владимира Владимировича"; resolution 923 of 10 August is signed "И. о. ... В.Путин". Resolution 4276-II ГД
  consents on 16 August; decree 1052 appoints him and enters into force on signing.

Decision: **accepted in part**. Holder 5, Сергей Вадимович Степашин, `attested_on` 19 May 1999 (with resolution 905 as
an in-office attestation); holder 6, Владимир Владимирович Путин, `from` 16 August 1999. Primakov's observation has no
end: pairing decree 580 (which names nobody) with the Duma's political statement would be an inferred end (B14).
Stepashin's has none either (decree 1012 names no Chairman). Acting service is claims only.

Limits: the nomination dates (the Duma stenograms of 19 May and 16 August 1999 were behind bot protection) and the ends.

### RU-GOV-06 — 2000-2004: Kasyanov

Evidence: decree 834 of 7 May 2000 ("В связи с вступлением в должность Президента") entrusts the Chairman's duties to
"Касьянова Михаила Михайловича"; decree 836 recites the Government's surrender of powers and keeps it acting; the
Government's order 647-r of the same day lays down its powers and is signed "И. о. Председателя Правительства Российской
Федерации М.Касьянов"; kremlin.ru news 38132 (10 May, 14:40) reports the letter submitting his candidacy; resolution
363-III ГД consents on 17 May and decree 861 appoints him, in force on signing; news 38171 reports the certificate
handed over that evening. On 24 February 2004 decree 264 announces the Government's resignation and entrusts the duties
to "Христенко В.Б."; the President's statement (transcript 22362) says he decided on the Government's resignation under
Article 117 and intends to submit a candidate (it names no election day, B5); news 30424 styles Kasyanov "бывшим
Председателем Правительства"; resolution 104 of 25 February (check find) is signed by "В.Христенко" as temporary acting
Chairman.

Decision: accepted. Holder 7, Михаил Михайлович Касьянов, `from` 17 May 2000. **No `until`** (B1): decree 264 names no
Chairman, and the "former Chairman" styling, on a page last updated on 20 March 2015, states no day on which his office
ended. The 1999 Putin observation has no end either (decrees 834 and 836 and order 647-r name no outgoing Chairman).

### RU-GOV-07 — 2004-2007: Fradkov

Evidence: kremlin.ru news 45963 (1 March 2004) reports the President proposing "Михаила Фрадкова" to the Duma
majority's leaders and describing him as Russia's representative to the Commission of the European Communities (B8);
resolution 162-IV ГД consents on 5 March and decree 300 appoints him, in force on signing. On 7 May 2004 the
Government's order 608-r lays down its powers and is signed "Председатель Правительства Российской Федерации М.Фрадков";
decree 585 recites the surrender (without the dossier's constitutional gloss, B6) and keeps the Government acting;
kremlin.ru news 30891 reports the order received and the decree signed (two claims, B10) and news 30892 the letter
submitting his candidacy; resolutions 232 (8 May) and 233 (11 May) are signed "И. о. ... М.Фрадков" (check finds, B2);
resolution 489-IV ГД consents on 12 May and decree 610 appoints him. On 12 September 2007 news 42294 and transcript 24530
record his request to resign and the President's acceptance ("Принимая Вашу отставку") with a request to keep performing
his duties (split, B9); decree 1184 announces the Government's resignation and entrusts the duties to "Фрадкова М.Е." (no
gloss, B7); resolution 586 of 13 September (check find) is signed by him as temporary acting Chairman.

Decision: **accepted in part**. Holders 8 and 9, Михаил Ефимович Фрадков, `from` 5 March 2004 and 12 May 2004, separate
observations. **No `until`** on either (B2, B3): order 608-r and decree 585 concern the Government as a body, and the
acting signatures of 8 and 11 May show continued service, not an end; the 2007 acceptance of a personal resignation is
the strongest candidate for a stated end, but no source says his powers as Chairman ceased that day, and he continued as
acting Chairman. An integrator ruling is requested; if the acceptance is accepted as a stated end, it should rest on
`ru_kremlin_steno_putin_accepts_resignation_20070912` alone and the same rule must then be applied to every holder.

Limits: the formal submission letter of March 2004. The kremlin.ru items carry 2014-2015 last-update stamps (B11); no
contemporaneous capture was found.

### RU-GOV-08 — 2007-2008: Zubkov and Putin

Evidence: kremlin.ru news 42319 (text version, 14 September 2007) states that the candidacy was submitted on Wednesday
12 September and reports the consent and the decree (report kinds, C9); resolution 5066-4 ГД consents and decree 1202
appoints "Зубкова Виктора Алексеевича", in force on signing. On 7 May 2008 the Government's order 680-r (check find, C2),
signed "Председатель Правительства Российской Федерации В.Зубков", lays down its powers; decree 717 recites it and keeps
the Government acting; news 6 reports Medvedev's letter submitting Putin's candidacy. On 8 May news 20 reports the
392-56 vote and adds that 448 deputies took part in the sitting (C7); resolution 458-5 ГД consents; decree 723 appoints
"Путина Владимира Владимировича", in force on signing.

Decision: accepted. Holder 10, Виктор Алексеевич Зубков, `from` 14 September 2007 (order 680-r as an in-office
attestation); holder 11, Владимир Владимирович Путин, `from` 8 May 2008, not merged with his `ru_president` holders.
Neither has an end.

### RU-GOV-09 — 2012-2020: Medvedev

Evidence: on 7 May 2012 the Government's order 760-r (check find, C1) lays down its powers and is signed "Исполняющий
обязанности Председателя Правительства Российской Федерации В.Зубков", which answers the dossier's open question of who
signed as acting Chairman that day; decree 607 recites the laying down and keeps the Government acting; news 15230
reports the letter submitting Medvedev's candidacy. On 8 May news 15266 reports the 299-144 vote, resolution 323-6 ГД
consents and decree 612 appoints him. On 7 May 2018 order 875-r (check find), signed "Д.Медведев" as Chairman, lays down
the powers; decree 202 recites it; news 57422 reports the submission; on 8 May resolution 3894-7 ГД consents and decree
209 appoints him. On 15 January 2020, at a meeting at Government House reported on kremlin.ru, Medvedev proposes that
the Government resign (`resignation_proposed`, C8) and the President asks its members to continue; decree 14 announces
the Government's resignation under Article 117(2), entrusts the Chairman's duties to "Медведева Д.А." and keeps the
Government acting.

Decision: accepted. Holders 12 and 13, Дмитрий Анатольевич Медведев, `from` 8 May 2012 and 8 May 2018. **No `until`** on
the 2018 observation (C3): decree 14 concerns the Government as a body and makes him acting Chairman; it does not state
that his office ended, and the dossier itself refused ends for the parallel events of 2008, 2012, 2018 and 2024. Zubkov's
acting signature of 7 May 2012 is a claim only; no act assigning the duties was found.

### RU-GOV-10 — 2020-2026: Mishustin and the attestation before the cutoff

Evidence: news 62586 (15 January 2020) reports the submission of Mishustin's candidacy; resolution 7565-7 ГД consents on
16 January and decree 17 appoints him, in force on signing. On 7 May 2024 order 1121-r (check find), signed
"Председатель Правительства Российской Федерации М.Мишустин" with an electronic-signature stamp, lays down the
Government's powers; decree 306 recites it. On 10 May news 74009 (12:40) styles him acting Chairman and has the President
say the proposal had been submitted. The official stenogram fragment of sitting No. 210 (capture of 20 May 2024, check
find, C4) has the President's representative Г. В. Минх say that the President "9 мая текущего года внёс в
Государственную Думу на утверждение кандидатуру Мишустина Михаила Владимировича", and records the open vote on draft
resolution 622328-8 at 16:20:28: 375 for, 0 against, 57 abstaining, "Результат: принято". Resolution 6061-8 ГД approves
("утвердить", the post-2020 wording) and decree 319 appoints him, in force on signing. Government resolution 1125 of 3
September 2026 is signed "Председатель Правительства Российской Федерации М.Мишустин" and was officially published on 4
September 2026 (publication No. 0001202609040021, from the hashed document card).

Decision: accepted. Holders 14 and 15, Михаил Владимирович Мишустин, `from` 16 January 2020 and 10 May 2024; order 1121-r
and resolution 1125 are in-office attestations, not boundaries. The 2024 submission is dated 9 May by the President's
representative (`ru_duma_steno_mishustin_submitted_20240509`), which narrows the dossier's open question; it is never a
holder date.

## Sources added

102 sources, each with a checked-in derived factual extract under [sources/](sources/) (`russia-*-facts.json`, LF,
format `spheres-c01-derived-factual-table/v1`, with its own checksum in the packet; 542,598 bytes in all). Each
extract records the original response's URL, byte count and SHA-256, the attached responses, a stability record and one
row per claim (claim_id, observation, role, `holder_name` and the printed form, role title, event kind, date, text,
locator). Original pages and PDFs are not checked in; no emblem, seal, signature image or photograph is republished.
"Check find" marks a primary record found by an independent check. One existing extract,
`russia-ips-ukaz-1146-19960809-facts.json` (CLAUDE-C01-14), gains two rows and a widened scope (see
[Integration notes](#integration-notes-outside-this-packets-file-boundary)).

| Source ID | What | Response identity (bytes, SHA-256) and read path |
|---|---|---|
| `ru_rsfsr_cpd_res_1830i_19911101` | [Congress resolution 1830-I, 1 Nov 1991](https://pravo.gov.ru/proxy/ips/?doc_itself=&nd=102012920&page=1&rdk=0) | HTTP, 25,524 bytes, `a65284a5…734c8e`; card 3,121 `acb45728` |
| `ru_rsfsr_ukaz_171_19911106` | [Decree 171, 6 Nov 1991](https://pravo.gov.ru/proxy/ips/?doc_itself=&nd=102012981&page=1&rdk=0) | HTTP, 20,304 bytes, `9dcc2f76…b63b83`; card 2,873 `b7ee2052` |
| `ru_rsfsr_ukaz_172_19911106` | [Decree 172, 6 Nov 1991](https://pravo.gov.ru/proxy/ips/?doc_itself=&nd=102012988&page=1&rdk=0) | HTTP, 36,493 bytes, `e748a13a…e81fca`; card 3,078 `2ce25a0c` |
| `ru_rsfsr_gov_res_8_19911115` | [Government resolution 8, 15 Nov 1991](https://pravo.gov.ru/proxy/ips/?doc_itself=&nd=102013105&page=1&rdk=0) | HTTP, 8,523 bytes, `b8c9282e…74a31e`; card 2,964 `2a3c6f99` |
| `ru_ukaz_633_19920615` | [Decree 633, 15 Jun 1992](https://pravo.gov.ru/proxy/ips/?doc_itself=&nd=102016829&page=1&rdk=0) | HTTP, 23,687 bytes, `3331989e…c569ec`; card 3,134 `161b2881` |
| `ru_gov_res_457_19920701` | [Government resolution 457, 1 Jul 1992](https://pravo.gov.ru/proxy/ips/?doc_itself=&nd=102017184&page=1&rdk=0) | HTTP, 11,718 bytes, `6e6f7ef1…d6183e`; card 3,120 `a889f0d8` |
| `ru_cpd_res_4063i_19921209` | [Congress resolution 4063-I, 9 Dec 1992](https://pravo.gov.ru/proxy/ips/?doc_itself=&nd=102020211&page=1&rdk=0) | HTTP, 7,516 bytes, `b2ca2b01…a626a1`; card 3,213 `c553d257` |
| `ru_cpd_res_4079i_19921212` | [Congress resolution 4079-I, 12 Dec 1992](https://pravo.gov.ru/proxy/ips/?doc_itself=&nd=102020277&page=1&rdk=0) | HTTP, 11,520 bytes, `6cca2d3e…1d5151`; card 3,134 `f1cda79e` |
| `ru_cpd_res_4088i_19921214` | [Congress resolution 4088-I, 14 Dec 1992](https://pravo.gov.ru/proxy/ips/?doc_itself=&nd=102020284&page=1&rdk=0) | HTTP, 7,235 bytes, `fb54c920…af308a`; card 3,152 `6f8f5533` |
| `ru_ukaz_1567_19921214` | [Decree 1567, 14 Dec 1992](https://pravo.gov.ru/proxy/ips/?doc_itself=&nd=102020298&page=1&rdk=0) | HTTP, 23,684 bytes, `d33bfee7…cb7b6d`; card 3,263 `4db75021` |
| `ru_ukaz_1569_19921215` | [Decree 1569, 15 Dec 1992](https://pravo.gov.ru/proxy/ips/?doc_itself=&nd=102020327&page=1&rdk=0) | HTTP, 24,638 bytes, `12352d48…1fce06`; card 3,250 `5d0f1509` |
| `ru_ukaz_1570_19921215` | [Decree 1570, 15 Dec 1992](https://pravo.gov.ru/proxy/ips/?doc_itself=&nd=102020328&page=1&rdk=0) | HTTP, 23,571 bytes, `937cc54e…bae424`; card 3,209 `4efeb841` |
| `ru_duma_steno_ia_19960810` | [State Duma stenogram (capture), 10 Aug 1996](https://web.archive.org/web/20240528152653id_/http://transcript.duma.gov.ru/node/2898/) | IA 20240528152653, 503,153 bytes, `c2bdcb97…448a31` |
| `ru_gd_res_624ii_19960810` | [State Duma resolution 624-II, 10 Aug 1996](https://pravo.gov.ru/proxy/ips/?doc_itself=&nd=102042901&page=1&rdk=0) | HTTP, 8,180 bytes, `82293a19…ab580d`; card 3,211 `32777fbc` |
| `ru_ukaz_1152_19960810` | [Decree 1152, 10 Aug 1996](https://pravo.gov.ru/proxy/ips/?doc_itself=&nd=102042896&page=1&rdk=0) | HTTP, 23,652 bytes, `916f7483…e01db4`; card 3,059 `24be7622` |
| `ru_ukaz_281_19980323` | [Decree 281, 23 Mar 1998](https://pravo.gov.ru/proxy/ips/?doc_itself=&nd=102052192&page=1&rdk=0) | HTTP, 23,883 bytes, `a3e71c31…0e4254`; card 3,115 `38c29453` |
| `ru_ukaz_287_19980323` | [Decree 287, 23 Mar 1998](https://pravo.gov.ru/proxy/ips/?doc_itself=&nd=102052188&page=1&rdk=0) | HTTP, 24,048 bytes, `93a5ac88…1be57b`; card 3,164 `352dba2d` |
| `ru_ukaz_288_19980323` | [Decree 288, 23 Mar 1998](https://pravo.gov.ru/proxy/ips/?doc_itself=&nd=102052187&page=1&rdk=0) | HTTP, 23,794 bytes, `71d0e6ef…c37d13`; card 3,153 `f4db87e8` |
| `ru_duma_steno_19980410` | [State Duma stenogram, 10 Apr 1998](https://web.archive.org/web/20240727075703id_/http://transcript.duma.gov.ru/node/2581/) | IA 20240727075703, 351,530 bytes, `e2aaba02…8bc896` |
| `ru_gd_res_2375ii_19980410` | [State Duma resolution 2375-II, 10 Apr 1998](https://pravo.gov.ru/proxy/ips/?doc_itself=&nd=102052472&page=1&rdk=0) | HTTP, 8,248 bytes, `b68b6183…dd7b17`; card 3,191 `b3a94a0f` |
| `ru_duma_steno_19980417` | [State Duma stenogram, 17 Apr 1998](https://web.archive.org/web/20240528152617id_/http://transcript.duma.gov.ru/node/2572/) | IA 20240528152617, 500,314 bytes, `817e7534…12519c` |
| `ru_gd_res_2402ii_19980417` | [State Duma resolution 2402-II, 17 Apr 1998](https://pravo.gov.ru/proxy/ips/?doc_itself=&nd=102052549&page=1&rdk=0) | HTTP, 8,273 bytes, `c6f2ae81…f6b24c`; card 3,191 `c931f120` |
| `ru_duma_steno_19980424` | [State Duma stenogram, 24 Apr 1998](https://transcript.duma.gov.ru/node/2568/) (check find) | HTTP (live page), 392,612 bytes, `8293a8c9…35b778` |
| `ru_gd_res_2421ii_19980424` | [State Duma resolution 2421-II, 24 Apr 1998](https://pravo.gov.ru/proxy/ips/?doc_itself=&nd=102052658&page=1&rdk=0) | HTTP, 28,358 bytes, `459f5515…bde7fc`; card 3,279 `09743f71` |
| `ru_ukaz_436_19980424` | [Decree 436, 24 Apr 1998](https://pravo.gov.ru/proxy/ips/?doc_itself=&nd=102052676&page=1&rdk=0) | HTTP, 20,085 bytes, `3ef6f580…1779f1`; card 3,128 `950c6b1b` |
| `ru_ukaz_983_19980823` | [Decree 983, 23 Aug 1998](https://pravo.gov.ru/proxy/ips/?doc_itself=&nd=102055100&page=1&rdk=0) | HTTP, 23,827 bytes, `8ee3465c…30f438`; card 3,115 `b391b582` |
| `ru_ukaz_987_19980825` | [Decree 987, 25 Aug 1998](https://pravo.gov.ru/proxy/ips/?doc_itself=&nd=102055106&page=1&rdk=0) (check find) | HTTP, 20,533 bytes, `8d49ad29…aeaff9`; card 3,148 `a030e203` |
| `ru_duma_steno_19980831` | [State Duma stenogram, 31 Aug 1998](https://transcript.duma.gov.ru/node/2517/) (check find) | HTTP (live page), 263,526 bytes, `9b41c8f2…02408f` |
| `ru_gd_res_2898ii_19980831` | [State Duma resolution 2898-II, 31 Aug 1998](https://pravo.gov.ru/proxy/ips/?doc_itself=&nd=102055136&page=1&rdk=0) | HTTP, 8,274 bytes, `a846bf2a…4bd10b`; card 3,191 `8840dd4a` |
| `ru_duma_steno_19980907` | [State Duma stenogram, 7 Sep 1998](https://web.archive.org/web/20220620072458id_/http://transcript.duma.gov.ru/node/2510/) | IA 20220620072458, 227,423 bytes, `2a7fdbb6…1ff344` |
| `ru_gd_res_2928ii_19980907` | [State Duma resolution 2928-II, 7 Sep 1998](https://pravo.gov.ru/proxy/ips/?doc_itself=&nd=102055315&page=1&rdk=0) | HTTP, 28,360 bytes, `10da8b9f…2ef815`; card 3,191 `ca4018a7` |
| `ru_duma_steno_19980911` | [State Duma stenogram, 11 Sep 1998](https://web.archive.org/web/20220615161227id_/http://transcript.duma.gov.ru/node/2506/) | IA 20220615161227, 578,566 bytes, `592cea0a…140477` |
| `ru_gd_res_2961ii_19980911` | [State Duma resolution 2961-II, 11 Sep 1998](https://pravo.gov.ru/proxy/ips/?doc_itself=&nd=102055400&page=1&rdk=0) | HTTP, 8,293 bytes, `eef235a0…4de262`; card 3,209 `7f953cd1` |
| `ru_ukaz_1087_19980911` | [Decree 1087, 11 Sep 1998](https://pravo.gov.ru/proxy/ips/?doc_itself=&nd=102055453&page=1&rdk=0) | HTTP, 23,636 bytes, `88e46448…d0bfa0`; card 3,059 `5003881f` |
| `ru_ukaz_580_19990512` | [Decree 580, 12 May 1999](https://pravo.gov.ru/proxy/ips/?doc_itself=&nd=102059649&page=1&rdk=0) | HTTP, 24,115 bytes, `0988a28c…af3568`; card 3,128 `a87cbcab` |
| `ru_duma_3961ii_19990512` | [State Duma resolution 3961-II, 12 May 1999](https://pravo.gov.ru/proxy/ips/?doc_itself=&nd=102059639&page=1&rdk=0) | HTTP, 31,216 bytes, `85bb8bf4…3eaca2`; card 3,171 `22bfeac2` |
| `ru_gov_res_528_19990513` | [Government resolution 528, 13 May 1999](https://pravo.gov.ru/proxy/ips/?doc_itself=&nd=102059661&page=1&rdk=0) (check find) | HTTP, 31,708 bytes, `01d20d17…f34923`; card 3,036 `01236efb` |
| `ru_duma_3965ii_19990519` | [State Duma resolution 3965-II, 19 May 1999](https://pravo.gov.ru/proxy/ips/?doc_itself=&nd=102059766&page=1&rdk=0) | HTTP, 28,371 bytes, `b7823540…d9a585`; card 3,207 `a834d1af` |
| `ru_ukaz_611_19990519` | [Decree 611, 19 May 1999](https://pravo.gov.ru/proxy/ips/?doc_itself=&nd=102059754&page=1&rdk=0) | HTTP, 23,628 bytes, `00f03014…7de8d5`; card 3,058 `e3a6c1bb` |
| `ru_gov_res_905_19990806` | [Government resolution 905, 6 Aug 1999](https://pravo.gov.ru/proxy/ips/?doc_itself=&nd=102061269&page=1&rdk=0) (check find) | HTTP, 31,777 bytes, `c74956a7…5c9a87`; card 3,113 `b6ffc114` |
| `ru_ukaz_1012_19990809` | [Decree 1012, 9 Aug 1999](https://pravo.gov.ru/proxy/ips/?doc_itself=&nd=102061311&page=1&rdk=0) | HTTP, 20,522 bytes, `fd660c0d…6e14d4`; card 3,129 `d63d5af9` |
| `ru_gov_res_923_19990810` | [Government resolution 923, 10 Aug 1999](https://pravo.gov.ru/proxy/ips/?doc_itself=&nd=102061319&page=1&rdk=0) (check find) | HTTP, 7,808 bytes, `0a4d7e5e…82d635`; card 3,110 `76d0ce95` |
| `ru_duma_4276ii_19990816` | [State Duma resolution 4276-II, 16 Aug 1999](https://pravo.gov.ru/proxy/ips/?doc_itself=&nd=102061352&page=1&rdk=0) | HTTP, 8,297 bytes, `83dcb530…e15809`; card 3,210 `f2b6d00d` |
| `ru_ukaz_1052_19990816` | [Decree 1052, 16 Aug 1999](https://pravo.gov.ru/proxy/ips/?doc_itself=&nd=102061345&page=1&rdk=0) | HTTP, 23,754 bytes, `71c634af…4857a1`; card 3,129 `f52cb606` |
| `ru_gov_rasp_647r_20000507` | [Government order 647-r, 7 May 2000](https://pravo.gov.ru/proxy/ips/?doc_itself=&nd=102065738&page=1&rdk=0) | HTTP, 31,940 bytes, `49e80cb0…9c840c`; card 3,022 `f68aed24` |
| `ru_ukaz_834_20000507` | [Decree 834, 7 May 2000](https://pravo.gov.ru/proxy/ips/?doc_itself=&nd=102065739&page=1&rdk=0) | HTTP, 23,925 bytes, `80c52749…e34d26`; card 3,153 `896f756c` |
| `ru_ukaz_836_20000507` | [Decree 836, 7 May 2000](https://pravo.gov.ru/proxy/ips/?doc_itself=&nd=102065741&page=1&rdk=0) | HTTP, 24,049 bytes, `9a8bab10…d04468`; card 3,201 `4a781f04` |
| `ru_kremlin_news_38132_20000510` | [Kremlin news 38132, 10 May 2000](https://web.archive.org/web/20260513105913id_/http://kremlin.ru/events/president/news/38132) | IA 20260513105913, 34,798 bytes, `e2f76cc3…719136` |
| `ru_duma_363iii_20000517` | [State Duma resolution 363-IIi, 17 May 2000](https://pravo.gov.ru/proxy/ips/?doc_itself=&nd=102065777&page=1&rdk=0) | HTTP, 8,290 bytes, `9b3b03c3…9d91db`; card 3,197 `09c112d8` |
| `ru_ukaz_861_20000517` | [Decree 861, 17 May 2000](https://pravo.gov.ru/proxy/ips/?doc_itself=&nd=102065790&page=1&rdk=0) | HTTP, 23,705 bytes, `3ae314db…ebda25`; card 3,128 `864f0b2b` |
| `ru_kremlin_news_38171_20000517` | [Kremlin news 38171, 17 May 2000](https://web.archive.org/web/20250425232300id_/http://kremlin.ru/events/president/news/38171) | IA 20250425232300, 36,843 bytes, `a2ad8116…f8ce35` |
| `ru_ukaz_264_20040224` | [Decree 264, 24 Feb 2004](https://pravo.gov.ru/proxy/ips/?doc_itself=&nd=102085491&page=1&rdk=0) | HTTP, 20,097 bytes, `e9464945…9d45d4`; card 3,043 `723e3bd4` |
| `ru_kremlin_transcript_22362_20040224` | [Kremlin transcript 22362, 24 Feb 2004](https://web.archive.org/web/20260615170001id_/http://www.kremlin.ru/events/president/transcripts/22362) | IA 20260615170001, 42,078 bytes, `3a2873e2…35f2db` |
| `ru_kremlin_news_30424_20040224` | [Kremlin news 30424, 24 Feb 2004](https://web.archive.org/web/20260616154745id_/http://kremlin.ru/events/president/news/30424) | IA 20260616154745, 37,301 bytes, `2384295f…1b61d2` |
| `ru_gov_res_104_20040225` | [Government resolution 104, 25 Feb 2004](https://pravo.gov.ru/proxy/ips/?doc_itself=&nd=102085550&page=1&rdk=0) (check find) | HTTP, 29,317 bytes, `e77a8a3f…d53c29`; card 3,279 `4886ac9c` |
| `ru_kremlin_news_45963_20040301` | [Kremlin news 45963, 1 Mar 2004](https://web.archive.org/web/20260410175830id_/http://kremlin.ru/events/president/news/45963) | IA 20260410175830, 41,738 bytes, `8c8daeaa…efc34b` |
| `ru_duma_162iv_20040305` | [State Duma resolution 162-IV, 5 Mar 2004](https://pravo.gov.ru/proxy/ips/?doc_itself=&nd=102085648&page=1&rdk=0) | HTTP, 8,231 bytes, `55c22021…ba7dce`; card 3,192 `fb4ea9e1` |
| `ru_ukaz_300_20040305` | [Decree 300, 5 Mar 2004](https://pravo.gov.ru/proxy/ips/?doc_itself=&nd=102085647&page=1&rdk=0) | HTTP, 23,703 bytes, `025dc0f0…548a9a`; card 3,057 `f291b26d` |
| `ru_gov_rasp_608r_20040507` | [Government order 608-r, 7 May 2004](https://pravo.gov.ru/proxy/ips/?doc_itself=&nd=102086677&page=1&rdk=0) | HTTP, 27,540 bytes, `6d58f37b…cd9608`; card 3,040 `fb0ae3cf` |
| `ru_ukaz_585_20040507` | [Decree 585, 7 May 2004](https://pravo.gov.ru/proxy/ips/?doc_itself=&nd=102086675&page=1&rdk=0) | HTTP, 24,307 bytes, `7607b52c…7c4060`; card 3,084 `c4652207` |
| `ru_kremlin_news_30891_20040507` | [Kremlin news 30891, 7 May 2004](https://web.archive.org/web/20250521055537id_/http://kremlin.ru/events/president/news/30891) | IA 20250521055537, 36,430 bytes, `22151444…2fca37` |
| `ru_kremlin_news_30892_20040507` | [Kremlin news 30892, 7 May 2004](https://web.archive.org/web/20240510083530id_/http://kremlin.ru/events/president/news/30892/print) | IA 20240510083530, 38,383 bytes, `9407866e…3392b4` |
| `ru_gov_res_232_20040508` | [Government resolution 232, 8 May 2004](https://pravo.gov.ru/proxy/ips/?doc_itself=&nd=102086682&page=1&rdk=0) (check find) | HTTP, 8,076 bytes, `7a1e27d7…25ea0a`; card 3,204 `6d37def8` |
| `ru_gov_res_233_20040511` | [Government resolution 233, 11 May 2004](https://pravo.gov.ru/proxy/ips/?doc_itself=&nd=102086705&page=1&rdk=0) (check find) | HTTP, 32,704 bytes, `1c78b338…888267`; card 3,210 `1db411a6` |
| `ru_duma_489iv_20040512` | [State Duma resolution 489-IV, 12 May 2004](https://pravo.gov.ru/proxy/ips/?doc_itself=&nd=102086710&page=1&rdk=0) | HTTP, 8,173 bytes, `27be0dc6…638aae`; card 3,266 `e45d2827` |
| `ru_ukaz_610_20040512` | [Decree 610, 12 May 2004](https://pravo.gov.ru/proxy/ips/?doc_itself=&nd=102086708&page=1&rdk=0) | HTTP, 23,702 bytes, `0ed6d5a3…a26b73`; card 3,128 `4058fac6` |
| `ru_ukaz_1184_20070912` | [Decree 1184, 12 Sep 2007](https://pravo.gov.ru/proxy/ips/?doc_itself=&nd=102116591&page=1&rdk=0) | HTTP, 23,958 bytes, `74f07e57…7581f3`; card 3,131 `5f208b66` |
| `ru_kremlin_news_42294_20070912` | [Kremlin news 42294, 12 Sep 2007](https://web.archive.org/web/20251216052618id_/http://www.kremlin.ru/events/president/news/42294) | IA 20251216052618, 38,808 bytes, `1160b90a…711006` |
| `ru_kremlin_transcript_24530_20070912` | [Kremlin transcript 24530, 12 Sep 2007](https://web.archive.org/web/20260610191047id_/http://kremlin.ru/events/president/transcripts/24530) | IA 20260610191047, 44,045 bytes, `0ea1a722…62c976` |
| `ru_gov_res_586_20070913` | [Government resolution 586, 13 Sep 2007](https://pravo.gov.ru/proxy/ips/?doc_itself=&nd=102116640&page=1&rdk=0) (check find) | HTTP, 20,673 bytes, `30a898ad…e1c088`; card 3,315 `4721e79b` |
| `ru_duma_res_5066_4_gd_20070914` | [State Duma resolution 5066-4, 14 Sep 2007](https://pravo.gov.ru/proxy/ips/?doc_itself=&nd=102116607&page=1&rdk=0) | HTTP, 32,541 bytes, `5215f20d…f00110`; card 3,194 `a2d2bafb` |
| `ru_ukaz_1202_20070914` | [Decree 1202, 14 Sep 2007](https://pravo.gov.ru/proxy/ips/?doc_itself=&nd=102116627&page=1&rdk=0) | HTTP, 23,709 bytes, `d26dd646…5a9173`; card 3,129 `6915f285` |
| `ru_kremlin_news_42319_20070914` | [Kremlin news 42319, 14 Sep 2007](https://web.archive.org/web/20251210060946id_/http://kremlin.ru/events/president/news/copy/42319) | IA 20251210060946, 1,600 bytes, `576dfbaa…643fae` |
| `ru_gov_rasp_680r_20080507` | [Government order 680-r, 7 May 2008](https://pravo.gov.ru/proxy/ips/?doc_itself=&nd=102121745&page=1&rdk=0) (check find) | HTTP, 21,754 bytes, `65b22ef2…874295`; card 3,022 `5e435ef9` |
| `ru_ukaz_717_20080507` | [Decree 717, 7 May 2008](https://pravo.gov.ru/proxy/ips/?doc_itself=&nd=102121789&page=1&rdk=0) | HTTP, 24,317 bytes, `56194f58…769168`; card 3,066 `1128d36e` |
| `ru_kremlin_news_6_20080507` | [Kremlin news 6, 7 May 2008](https://web.archive.org/web/20260421233600id_/http://kremlin.ru/events/president/news/6) | IA 20260421233600, 36,559 bytes, `9335792f…667962` |
| `ru_kremlin_news_20_20080508` | [Kremlin news 20, 8 May 2008](https://web.archive.org/web/20260520144350id_/http://kremlin.ru/events/president/news/20) | IA 20260520144350, 42,709 bytes, `d5daa1fe…7fdb8e` |
| `ru_duma_res_458_5_gd_20080508` | [State Duma resolution 458-5, 8 May 2008](https://pravo.gov.ru/proxy/ips/?doc_itself=&nd=102121651&page=1&rdk=0) | HTTP, 32,519 bytes, `748ee307…74cb0e`; card 3,269 `c8f62fa2` |
| `ru_ukaz_723_20080508` | [Decree 723, 8 May 2008](https://pravo.gov.ru/proxy/ips/?doc_itself=&nd=102121791&page=1&rdk=0) | HTTP, 19,788 bytes, `3742103b…d35da0`; card 3,200 `04fbbbbe` |
| `ru_gov_rasp_760r_20120507` | [Government order 760-r, 7 May 2012](https://pravo.gov.ru/proxy/ips/?doc_itself=&nd=102156287&page=1&rdk=0) (check find) | HTTP, 19,957 bytes, `ae197235…76e665`; card 3,022 `2e26d137` |
| `ru_pub_ukaz_607_20120507` | [Decree 607, 7 May 2012](https://publication.pravo.gov.ru/file/pdf?eoNumber=0001201205070027) | HTTP (PDF), 39,220 bytes, `ae7b8880…3d4733`; card 3,209 `0e46e68b`; IPS text 24,298 `168d27da` |
| `ru_kremlin_news_15230_20120507` | [Kremlin news 15230, 7 May 2012](https://web.archive.org/web/20260520130324id_/http://www.kremlin.ru/events/president/news/15230) | IA 20260520130324, 39,222 bytes, `20963d15…411a88` |
| `ru_kremlin_news_15266_20120508` | [Kremlin news 15266, 8 May 2012](https://web.archive.org/web/20260729233942id_/http://www.kremlin.ru/events/president/news/15266) | IA 20260729233942, 67,777 bytes, `ec4a7762…2461ee` |
| `ru_pub_duma_res_323_6_gd_20120508` | [State Duma resolution 323-6, 8 May 2012](https://publication.pravo.gov.ru/file/pdf?eoNumber=0001201205080001) | HTTP (PDF), 48,243 bytes, `85a8103b…16f4a4`; card 3,412 `5e0e2540`; IPS text 24,457 `506a9d9b` |
| `ru_pub_ukaz_612_20120508` | [Decree 612, 8 May 2012](https://publication.pravo.gov.ru/file/pdf?eoNumber=0001201205080002) | HTTP (PDF), 33,114 bytes, `1f0d6bae…e4f71b`; card 3,270 `c7b51f86`; IPS text 19,785 `b8df7269` |
| `ru_pub_gov_rasp_875r_20180507` | [Government order 875-r, 7 May 2018](https://publication.pravo.gov.ru/file/pdf?eoNumber=0001201805070016) (check find) | HTTP (PDF), 24,640 bytes, `d853a5a2…a98115`; card 3,165 `4f03448b`; IPS text 17,916 `aa4e962d` |
| `ru_pub_ukaz_202_20180507` | [Decree 202, 7 May 2018](https://publication.pravo.gov.ru/file/pdf?eoNumber=0001201805070024) | HTTP (PDF), 38,932 bytes, `d8457ead…7803b3`; card 3,209 `3bd41658`; IPS text 24,293 `ea14b4a4` |
| `ru_kremlin_news_57422_20180507` | [Kremlin news 57422, 7 May 2018](https://web.archive.org/web/20250829021035id_/http://www.kremlin.ru/events/president/news/copy/57422) | IA 20250829021035, 1,704 bytes, `a9134e21…64a5d6` |
| `ru_pub_duma_res_3894_7_gd_20180508` | [State Duma resolution 3894-7, 8 May 2018](https://publication.pravo.gov.ru/file/pdf?eoNumber=0001201805080026) | HTTP (PDF), 222,756 bytes, `baf9f3e8…7eedbe`; card 3,340 `8ed9de95`; IPS text 20,579 `f26edfce` |
| `ru_pub_ukaz_209_20180508` | [Decree 209, 8 May 2018](https://publication.pravo.gov.ru/file/pdf?eoNumber=0001201805080028) | HTTP (PDF), 31,157 bytes, `b4444681…dbdfd3`; card 3,270 `f1dba388`; IPS text 18,517 `76329d07` |
| `ru_kremlin_news_62585_20200115` | [Kremlin news 62585, 15 Jan 2020](https://web.archive.org/web/20210513233827id_/http://www.kremlin.ru/events/president/news/copy/62585) | IA 20210513233827, 7,497 bytes, `f2fbced0…9f5ae0` |
| `ru_pub_ukaz_14_20200115` | [Decree 14, 15 Jan 2020](https://publication.pravo.gov.ru/file/pdf?eoNumber=0001202001150020) | HTTP (PDF), 43,942 bytes, `585d9eca…526b46`; card 3,186 `bdb229aa`; IPS text 17,995 `c6a51abb` |
| `ru_kremlin_news_62586_20200115` | [Kremlin news 62586, 15 Jan 2020](https://web.archive.org/web/20230731173211id_/http://kremlin.ru/events/president/news/copy/62586) | IA 20230731173211, 1,642 bytes, `44e824a1…73011e` |
| `ru_pub_duma_res_7565_7_gd_20200116` | [State Duma resolution 7565-7, 16 Jan 2020](https://publication.pravo.gov.ru/file/pdf?eoNumber=0001202001160027) | HTTP (PDF), 39,429 bytes, `09e5c355…781988`; card 3,340 `af2f606c`; IPS text 18,306 `91e27220` |
| `ru_pub_ukaz_17_20200116` | [Decree 17, 16 Jan 2020](https://publication.pravo.gov.ru/file/pdf?eoNumber=0001202001160035) | HTTP (PDF), 32,565 bytes, `a177fb99…255c6e`; card 3,269 `bfa6f3b4`; IPS text 17,649 `2c1b1d05` |
| `ru_pub_gov_rasp_1121r_20240507` | [Government order 1121-r, 7 May 2024](https://publication.pravo.gov.ru/file/pdf?eoNumber=0001202405070002) (check find) | HTTP (PDF), 107,660 bytes, `5254fdde…989393`; card 3,166 `9918b6d8`; IPS text 23,862 `d62d1bce` |
| `ru_pub_ukaz_306_20240507` | [Decree 306, 7 May 2024](https://publication.pravo.gov.ru/file/pdf?eoNumber=0001202405070003) | HTTP (PDF), 41,213 bytes, `17c1e119…2f8891`; card 3,209 `dfbde56a`; IPS text 24,340 `bf31b202` |
| `ru_kremlin_news_74009_20240510` | [Kremlin news 74009, 10 May 2024](https://web.archive.org/web/20240512211122id_/http://kremlin.ru/events/president/news/copy/74009) | IA 20240512211122, 5,947 bytes, `16c34183…c423cd` |
| `ru_duma_steno_20240510` | [State Duma stenogram, 10 May 2024](https://web.archive.org/web/20240520233218id_/http://transcript.duma.gov.ru/api_search/?kodz=2125&kodvopr=2) (check find) | IA 20240520233218, 261,112 bytes, `dfdff77f…f72e3e` |
| `ru_pub_duma_res_6061_8_gd_20240510` | [State Duma resolution 6061-8, 10 May 2024](https://publication.pravo.gov.ru/file/pdf?eoNumber=0001202405100001) | HTTP (PDF), 35,692 bytes, `7b4448a5…b5f6cc`; card 3,295 `f983584b`; IPS text 24,321 `b9913956` |
| `ru_pub_ukaz_319_20240510` | [Decree 319, 10 May 2024](https://publication.pravo.gov.ru/file/pdf?eoNumber=0001202405100015) | HTTP (PDF), 32,384 bytes, `3747d35f…67d64b`; card 3,272 `c066263a`; IPS text 23,749 `4e118998` |
| `ru_pub_gov_res_1125_20260903` | [Government resolution 1125, 3 Sep 2026](https://publication.pravo.gov.ru/file/pdf?eoNumber=0001202609040021) | HTTP (PDF), 184,530 bytes, `972865e6…cc060d`; card 20,324 `f860a58c` |

Read paths. The legal portal, its official-publication section and the State Duma transcripts server refused HTTPS from
this environment (port 443), so their responses were read over HTTP; the packet URL gives the same path on HTTPS, the
only scheme the register accepts, and each extract records the HTTP URL read (as CLAUDE-C01-05 and C01-14 did).
kremlin.ru pages and six Duma stenograms (one of them a single-question fragment) are read from raw Internet Archive
captures (`id_` form, all made before the cutoff, now fetched over HTTPS, A2); the packet URL is the capture and
`original_url` the captured page.

## Response identities and stability checks

Every recorded response was downloaded at least three times, most of them four or more, and each identity is the
response of a stored page or file, not one generated per request:

- the dossier's first download and a delayed re-download 30-53 minutes later (for records a check found, the check's two
  downloads, 31-48 minutes apart, except resolutions 528, 905 and 923, whose check downloads were 7-8 minutes apart,
  and the 2024 stenogram capture, whose check record gives a single matching time, 14:06:39Z);
- the independent check (A: 13:21Z-13:25Z; B: two rounds, the second with a cache-busting query on every portal URL; C:
  two passes, the second with a cache-busting query on every official-publication PDF);
- this packet, twice, 32-37 minutes apart for each response: 193 downloads at 2026-09-25T14:18:26Z-14:25:24Z and 194 at
  2026-09-25T14:52:07Z-14:59:23Z: the recorded responses, every portal and publication card and every corroborating IPS text
  (the 1125 publication card, added by this packet, was first fetched twice at 14:26Z). Every one returned identical
  bytes and SHA-256; two connection failures in the second pass succeeded on an immediate retry.

Points a reviewer needs:

- **Two identities expire on 1 January 2027.** The live stenograms of 24 April and 31 August 1998 print the current year
  in their footer ("© Государственная Дума Федерального Собрания Российской Федерации, 2026"); no usable capture made
  before the cutoff exists. The server sometimes answers with a 307 redirect to a bot-protection path; it was never
  followed, and a plain retry returned the page. CLAUDE-C01-14's live stenogram of 10 August 1996 has the same footer;
  this packet cites the frozen capture of that sitting (20240528152653), which differs from the live page in that one
  byte.
- Internet Archive captures were requested without an Accept-Encoding header and none was served compressed, so each
  identity is the uncompressed body; captures replayed gzip-encoded were rejected by the dossiers.
- The kremlin.ru full pages carry "Последнее обновление материала 20 марта 2015 года" (42294: "28 мая 2014 года"); this is
  recorded in each provenance note and in the claims that rest on a styling (B11). The text versions (`/copy/`) carry no
  stamp. Live kremlin.ru pages are not recorded at all (B13, C10).
- Official-publication PDFs are stored files: their CreationDate is a fixed past value (the 6061-8 scan carries a 2014
  scanner clock; 3894-7 has none), and cache-busting downloads returned identical bytes. Each has its IPS original-edition
  text attached as a hashed corroborating response, except resolution 1125, whose publication number and date rest on
  the hashed official-publication document card this packet added.
- Portal cards are single-document searches restricted to one day (`a7from` = `a7to`, one hit); the per-date lists used
  for discovery are not recorded. The 2024 stenogram is a single sitting-and-question fragment of the transcripts search
  service, frozen in its capture, not a growing search.

## Leads not imported

- **State Duma website news items** (press-service reports, not the Duma's vote record; check C offered them as
  optional): [duma.gov.ru/news/26901](https://web.archive.org/web/20180509013520id_/http://www.duma.gov.ru/news/26901/)
  (8 May 2018, "«За» проголосовали 374 депутата"; 357,281 bytes, `308696a4…`),
  [duma.gov.ru/news/47525](https://web.archive.org/web/20200116124831id_/http://duma.gov.ru/news/47525/) (16 January
  2020, 383 for, none against, 41 abstaining; 382,914 bytes, `729daec1…`),
  [duma.gov.ru/news/59261](https://web.archive.org/web/20240510120450id_/http://duma.gov.ru/news/59261/) (10 May 2024,
  05:20, Volodin: the President had submitted Mishustin's candidacy; 239,589 bytes, `7a9314ff…`) and
  [duma.gov.ru/news/59264](https://web.archive.org/web/20240510231612id_/http://duma.gov.ru/news/59264/) (10 May 2024;
  427,522 bytes, `5e0f65ec…`). The 2024 stenogram carries both the submission day and the vote.
- **Government order 610-r of 8 May 2004** (nd=102086684, 29,156 bytes, `c2f05b78…`): signed "И. о. ... М.Фрадков" like
  resolution 232 of the same day; downloaded once by check B.
- **Government order 2416-r of 7 September 2026** (publication 0001202609070021, 98,135 bytes, `9b47a971…`): a second
  attestation of Mishustin as Chairman; resolution 1125 suffices.
- **Off-role or duplicate acts on the portal:** decree 286 of 23 March 1998 (Kiriyenko First Deputy Chairman,
  nd=102052189), decree 282 (an award to Chernomyrdin, nd=102052191), Government resolution 441 of 15 June 1992 (signed
  "Б. ЕЛЬЦИН", nd=102016822), laws 1827-I (nd=102012917) and 4061-I (constitutional amendments, procedure only), decree
  175 of 6 November 1991 (nd=102012985), decree 1011 of 9 August 1999 (Putin First Deputy Chairman, nd=102061314), decree
  299 of 5 March 2004 (nd=102085636) and decree 1183 of 12 September 2007 (an award).
- **kremlin.ru items duplicating imported records:** news/38168, news/30420, news/30421, news/30422, news/30505,
  news/30924, transcripts/22460, 22372, 22364, 18 and 19, news/57431, news/57415, news/62584, news/73976 and the
  document-bank copies (acts/bank/13000, acts/bank/35274); the date pages (`by-date`) list a moving window of items.
- **government.ru**: live pages rotate content and its history pages are retrospective lists; `/history/` returned 404.
- **Portal per-date lists** used for discovery (for example the list for 23.03.1998) and the Internet Archive CDX
  listings of transcript nodes.

## Sources attempted

- `https://pravo.gov.ru/…`, `https://publication.pravo.gov.ru/…`, `https://kremlin.ru/…`, `https://government.ru/…` and
  `https://transcript.duma.gov.ru/…`: port 443 failed; the same paths over HTTP worked (kremlin.ru is used only through
  captures).
- `transcript.duma.gov.ru` and `duma.gov.ru` over HTTP: intermittent 307 redirects to `/DDoS01/…` (bot protection), never
  followed; the stenograms of 24 April and 31 August 1998 answered plain retries. The stenograms of 19 May and 16 August
  1999, 17 May 2000, 5 March and 12 May 2004, 14 September 2007, 8 May 2008, 8 May 2012, 8 May 2018 and 16 January 2020
  were not read. `sozd.duma.gov.ru` did not connect.
- kremlin.ru's document bank (IDs 35277-35280) answered 403 (rate limit) and was not retried; its search returns no
  server-side results.
- Internet Archive: HTTPS was refused during the first downloads (it later answered); captures replayed gzip-encoded
  (news/6 of 4 June 2026, copy/62586 of 13 February 2026, the 30892 print view of 26 August 2024) were rejected; no
  capture exists of news/30892 itself or of the 24 April (only a 404) and 31 August 1998 nodes; 1999 captures of the
  Government's and the Duma's sites stop in April 1999.

## Checker defects

| # | Defect | Outcome |
|---|---|---|
| A1 | The 1996 stenogram identity is the live page with a 2026 footer | **Applied**: new source `ru_duma_steno_ia_19960810` (capture 20240528152653, 503,153 bytes, `c2bdcb97…`) carries the two claims; the C01-14 extract is not edited; its "no capture exists" sentence and live identity are flagged in the integration notes |
| A2 | Archive URLs recorded over `http://web.archive.org` | **Applied**: `https://web.archive.org/web/<ts>id_/…` as packet URL and response URL; this packet downloaded them over HTTPS |
| A3 | Field mapping (`holder_name_normalized`, free-text role titles) | **Applied**: canonical `holder_name`, `printed_name`, and the role title or the one acting title on every row |
| A4 | "The Fifth Congress" in 1830-I | **Applied**: "The Congress of People's Deputies of the RSFSR"; the ordinal is attributed to decree 171 |
| A5 | Primakov submission located in the morning session | **Applied**: after the break, chaired by Г.Н. Селезнев |
| A6 | Resolution 457 kind assumed an acting capacity | **Applied**: `government_act_signature`; the acting context stays in the uncertainty |
| A7 | Sitting-day presentations labelled `nomination` | **Applied**: `nomination_presented`, each saying the submission day is not given and never dates a holder |
| A8 | Decree 983 note wrong; decree 987 missing | **Applied**: decree 987 imported (a continuation claim); 983's uncertainty corrected |
| A9 | The 24 April and 31 August 1998 stenograms are obtainable | **Applied**: both imported (four claims), with their footer-year expiry recorded |
| A10 | Editing C01-14 extracts | **Applied**: the stenogram edit avoided (A1); decree 1146's extract edited (two rows, widened scope), listed as a touched path |
| B1 | Kasyanov `until` 2004-02-24 inferred | **Applied**: `until` null; the styling claim records the 2015 stamp |
| B2 | Fradkov `until` 2004-05-07 inferred | **Applied**: `until` null; resolutions 232 and 233 imported as acting attestations |
| B3 | Fradkov `until` 2007-09-12 needs a ruling | **Applied**: `until` null by default; the ruling is requested and its single anchor named |
| B4 | Holder names inconsistent | **Applied**: one canonical name per person, the C01-14 form for Путин and Медведев |
| B5 | Transcript 22362 said to name the election | **Applied**: reworded to the statement's words |
| B6 | Decree 585 constitutional gloss | **Applied**: removed; the recital is quoted |
| B7 | Decree 1184 constitutional gloss | **Applied**: removed |
| B8 | "European Communities" | **Applied**: "the Commission of the European Communities" |
| B9 | 12 September 2007 acceptance claims merged events | **Applied**: split into acceptance and request-to-act claims; the decree signing and Fradkov's reply removed from the acceptance |
| B10 | 7 May 2004 Kremlin claim merged events | **Applied**: split into the order received and the decree-signing report |
| B11 | kremlin.ru last-update stamps undisclosed | **Applied**: in every full-page provenance note and in the claims that rest on a styling |
| B12 | Acting claims lacked "never a holder"; no role titles | **Applied**: every acting claim says it; acting rows carry the acting title |
| B13 | Live alternates not reproducible | **Resolved by removal**: live kremlin.ru pages are not recorded |
| B14 | Primakov end suggested by pairing | **Applied**: no `until`; 3961-II's uncertainty says pairing would be an inferred end |
| B15 | Resolutions 905, 528 and 923 missed | **Applied**: imported (in-office and acting attestations), with 104, 232, 233 and 586 |
| C1 | Order 760-r missing | **Applied**: imported; acting signatory "В.Зубков", a claim only; 607's uncertainty rewritten |
| C2 | The Government's orders of 2008, 2018 and 2024 missing | **Applied**: 680-r (IPS), 875-r and 1121-r (PDF with IPS text) imported; each laying down of powers and each signature is its own claim |
| C3 | Medvedev `until` 2020-01-15 inferred | **Applied**: `until` null, as for every holder |
| C4 | Duma records exist as captures | **Applied in part**: the 10 May 2024 stenogram fragment imported (the vote, and the 9 May submission it states); the Duma website news items of 2018 and 2020 are press reports and stay leads |
| C5 | News item 59261 (2024 submission) | **Declined**: a press report; the stenogram now dates the submission to 9 May 2024 |
| C6 | Surname-first names | **Applied**: canonical names with `printed_name` |
| C7 | News/20: 448 took part in the sitting | **Applied** |
| C8 | 62585 kind `resignation_tendered` | **Applied**: `resignation_proposed`; decree 14 relies on Article 117(2) |
| C9 | Kremlin reports reused act kinds | **Applied**: `appointment_reported`, `parliamentary_consent_reported`, `parliamentary_vote_reported` |
| C10 | 57422 live-copy caveat | **Resolved by removal**: live copies are not recorded |
| C11 | Two errors in the researcher's summary | **Applied**: 32-53 minutes; the 2020 meeting was at Government House (as stated here) |

Missing primary records: all four found by check A, all seven with hashes found by check B, and the order texts and the
stenogram fragment found by check C are imported (with the 1125 publication card added by this packet). Not imported:
the four Duma website news items (press reports) and order 610-r of 2004 (a single download, redundant with resolution
232).

## Suggested next work orders

These are proposals for the integrator. They are not created in `work-orders.json`.

- `C01-Russia-GOV-001`: an integrator ruling on ends. Is the President's acceptance of a personal resignation (Fradkov,
  12 September 2007) or a decree announcing the Government's resignation that makes the outgoing Chairman acting (decree
  14, 2020) a stated end of the Chairman's office? Whatever the ruling, apply it to all fifteen holders at once.
- `C01-Russia-GOV-002`: the State Duma stenograms of 19 May and 16 August 1999, 17 May 2000, 5 March and 12 May 2004, 14
  September 2007, 8 May 2008, 8 May 2012, 8 May 2018 and 16 January 2020 (nomination letters, vote figures) when the
  transcripts server is reachable, and frozen captures for 24 April and 31 August 1998 (and C01-14's 10 August 1996
  page) before 1 January 2027.
- `C01-Russia-GOV-003`: the President's nomination letters of 1992-2020; the Congress's record of the 9 December 1992
  ballot and the 14 December rating vote; the 1993 retitling from "Председатель Совета Министров - Правительства".
- `C01-Russia-GOV-004`: the act ending the 1991 arrangement (the repeal of decrees 171 and 172) and any act assigning
  acting duties on 7 May 2012 and 7-10 May 2024.
- Deputy chairmen, ministers, the Government's composition and the Security Council are outside this packet.

## Integration notes (outside this packet's file boundary)

- **Stacking.** This branch is stacked on CLAUDE-C01-14 (`claude/c01-ru-14` at `1d749e15`), itself stacked on
  CLAUDE-C01-05. `claude/c01-ru-14` was fetched again before this work and had no commits missing from this branch. Both
  packets edit `russia.json` and the Russia tests, so CLAUDE-C01-14 must be integrated first; after that, this packet's
  diff is additive except for one edited existing extract.
- **Edited existing file:** `docs/campaign-certification/C01/research/sources/russia-ips-ukaz-1146-19960809-facts.json`
  (CLAUDE-C01-14) gains two rows (`ru_ukaz_1146_resignation_statement_accepted_19960809`,
  `ru_ukaz_1146_government_continues_19960809`, role `ru_government_chairman`), a widened `bounded_scope`, one sentence in
  `scope_note` and one in `stability_check`; its response identity is unchanged. The packet's `ru_ukaz_1146_19960809`
  source gains the same two claims and a new snapshot checksum. No other existing record changes.
- **C01-14's live stenogram.** `russia-duma-stenogram-19960810-facts.json` records the live page, whose footer prints the
  current year, so its identity will stop reproducing on 1 January 2027, and its sentence "No Internet Archive capture of
  this node exists" is wrong: capture 20240528152653 exists (byte-identical except for the footer year). It is not edited
  here; Codex may re-point it to the capture. The same expiry applies to this packet's stenograms of 24 April and 31
  August 1998.
- `research-index.json` is regenerated in a **separate commit**; it is the only file this packet shares with other
  pending packets (apart from the files it shares with CLAUDE-C01-14 and C01-05 through the stack). New totals: 336
  sources and 2,046 claims (previously 234 and 1,893); 29 institution observations (28); 93 discovery batches (92).
  Russia's entries go from 20 to 21, `mapping_pending` from 20 to 21, batches from [10, 10] to [10, 10, 1], role
  observations from 8 to 9 and source claims from 142 to 295. If another packet lands first, regenerate the index rather
  than merging it.
- `research/README.md`, the C01 README totals and `docs/planning/ai-workstreams.json` are left for the integrator; this
  handoff is self-proposed and not registered there.
- Existing tests updated, none loosened:
  - `test_russia_research_s10h.py`: totals 68→170 sources, 142→295 claims, 20→21 entries and 8→9 roles; institutions
    6→7 with the new kind `executive_institution`; the non-faction institutions pinned to exactly `ru_rsfsr_presidency`
    and `ru_government`; the source hosts unchanged (this packet uses only hosts already pinned); access dates pinned per
    packet (2 original, 13 C01-05, 53 C01-14, 102 C01-19); `mapping_pending` 21, role observations 9 and work orders
    [10, 10, 1].
  - `test_ussr_russia_transition_c01_05.py`: the non-faction Russia institutions pinned to the same two.
  - `test_russia_presidents_c01_14.py`: the same institution pin; the C01-14 source slice re-expressed as
    `sources[15:68]`; entries and roles (21, 9); index totals (9, 295); the two C01-19 rows on decree 1146 are named
    exactly (`C01_19_ROWS_1146`), pinned to their role and position, and excluded from the C01-14 claim and row
    classification by ID, so every C01-14 guard still applies to all 94 of its claims.
- The new test `test_russia_heads_of_government_c01_19.py` pins the institution, the role and the fifteen holders, every
  claim's date, kind and observation, every response identity and attached response, the extracts, the one edited
  extract, the separation from the presidency and the USSR packet, and 44 mutations.
- In the atlas, Russia gains a second executive institution, "Правительство Российской Федерации — Government of the
  Russian Federation", with one office and fifteen holder observations. No UI code changed; the Node check passes and no
  browser review was run.
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

All passed on 25 September 2026 (UTC): the exact index regeneration and check (336 sources, 2,046 claims, 29 institution
observations, 93 discovery batches); 29 Russia tests (10 of them new) and 18 USSR tests; 79 research tests; 9 campaign
research tests, with `test_campaign_census.py` erroring in setup because the sparse worktree has no `spheres-sim/data`
(expected; not widened); 11 atlas Node tests; the workboard check (44 markers); `git diff --check` on this packet's
paths. The new test's 44 mutations each fail as intended: successor starts, a reappointment, dismissals and resignation
decrees, the acceptance of a resignation, the laying down of powers, a former-holder styling, the Duma's 1999 statement
and the latest attestation used as ends; nomination and consent dates used as starts; a start set where the decree has
no effect clause; a presentation date used as the attested day; acting service (Gaidar 1992, Medvedev 2020, Zubkov 2012)
and the President's 1991 headship added as holders; acting, continuation and consent claims cited by holders; any
`until`; cross-role holders in both directions, a cross-role claim, a holder merged with the presidency and a
cross-institution holder on the USSR Presidency; a second head-of-government role; the role or the institution removed;
nominations re-dated to the sitting; a consent after its appointment; a lifecycle start; checksum mismatches (including
the edited C01-14 extract); beyond-cutoff dates; a reversed interval; a claim from an uncited source; and an HTTP source
URL.
