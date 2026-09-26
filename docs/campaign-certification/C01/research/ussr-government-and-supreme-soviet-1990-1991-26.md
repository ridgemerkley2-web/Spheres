# USSR government and Supreme Soviet 1990-1991 26: the Union head of government and the Chairman of the Supreme Soviet from 1 January 1990 to December 1991

Packet: **CLAUDE-C01-26**. State: **ready_for_review** (not complete).
Owner: Claude. Integrator/reviewer: Codex. Branch `claude/c01-su-26`; claim commit `e4fec74d` on `6f4ef2c2`, the head of
`claude/c01-ru-19`. **Stacked on CLAUDE-C01-19** (itself stacked on CLAUDE-C01-14 and C01-05): merge that packet first.
Research access: 26 September 2026 (UTC; 25 September local, UTC-7); every recorded download and both of this packet's
re-download passes (2026-09-26T03:52:14Z-03:54:51Z and 2026-09-26T04:25:49Z-04:27:36Z) fall on that UTC day. The historical cutoff stays **7 September 2026**.

This packet reviews ten observations, SU-GOV-01 to SU-GOV-10, in [ussr.json](ussr.json). It adds one institution,
`su_government` ("Совет Министров СССР / Кабинет Министров СССР — Council of Ministers of the USSR (from 1991 the Cabinet of
Ministers of the USSR)", kind `executive_institution`, lifecycle `unknown`), with one role, `su_government_head`
("Председатель Совета Министров СССР / Премьер-министр СССР — Head of the Union government", kind `head_of_government`), and
four holder observations. It extends the existing `su_supreme_soviet_chair` role with two holder observations and
26 claims, and it adds one election claim to the role claims of `su_president`, whose holders are unchanged. In
all it adds 31 sources and 77 claims, each source with a checked-in derived extract. It does not change the CLAUDE-C01-05
presidency holders, the existing chair observation of 14 March 1990, any existing extract or `russia.json`, and it adds no
organization, game mapping, portrait or avatar. No holder has a `from` or an `until`. The parent scope (C01, C06, S23, WC1 and
CP1) remains open.

Three research dossiers (parts A, B and C) and an independent adversarial check of each were prepared before this packet was
written. Every checker defect is applied, applied in part or declined with a reason (see [Checker defects](#checker-defects)).
Every obtainable missing primary record the checks found is imported; the ones not imported are either unobtainable
(Vedomosti 1991 Nos. 4, 39, 45, 48 and 52, behind a paywall or absent), an optional book page, or an Izvestia issue that does
not print the act. This packet also found three primary records on the official legal portal that neither the dossiers nor the
checks had: the Council of Ministers' resolution of 24 November 1990 signed by the Chairman, its resolution of 10 January 1991
signed by a Deputy Chairman, and the Cabinet of Ministers' order of 19 August 1991 in which the Premier entrusts the general
direction of the Cabinet to the First Deputy Premier.

## Outcome

| ID | Question | Decision |
|---|---|---|
| SU-GOV-01 | Who was Chairman of the Council of Ministers when the period opens? | **Accepted in part:** signatures of "Н. Рыжков" on joint USSR-RSFSR resolutions of 24 Dec 1989 and 12 Jan 1990 (RSFSR government gazette) and on resolution 525 (26 May 1990); holder 1 `attested_on` 12 Jan 1990. No record dated 1 Jan 1990 |
| SU-GOV-02 | March 1990: who became Chairman of the Supreme Soviet after the Chairman became President? | **Accepted in part:** the GARF original of 1362-I (15 Mar 1990) elects the President and is signed "(А.Лукьянов)" as Chairman; holder `attested_on` 15 Mar 1990; later signatures of 27 Dec 1990 (1870-I) and 14 Jan 1991 are claims. The election resolution 1367-I is a lead |
| SU-GOV-03 | The 1990-1991 constitutional change creating the Cabinet of Ministers (procedure only) | **Accepted in part:** Law 1861-I (26 Dec 1990) renames the chapter and rewrites Arts. 128-136, sets the Premier's approval and release (Arts. 113(3), 127-3(6)) and no confidence (Art. 130), and changes the chair's place in Art. 127-7; Law 1862-I keeps the Council of Ministers' powers until the new bodies are formed. Procedure only; Law 2033-I is a lead |
| SU-GOV-04 | Ryzhkov's end, only as a source states it | **Accepted in part:** his last reviewed signature (24 Nov 1990, holder 2); the heart attack reported on 26 Dec 1990 and the Congress's telegram; the Minister of Finance's report for the government and a health report (27 Dec); a Deputy Chairman's signature of 10 Jan 1991. Claims only; no source states an end or names an acting Chairman; no `until` |
| SU-GOV-05 | January 1991: Pavlov's appointment as Premier | **Accepted:** the Supreme Soviet's resolution of 14 Jan 1991, published in Izvestia No. 13, recites the President's submission and resolves "Утвердить премьер-министром СССР товарища Павлова Валентина Сергеевича"; holder 3 `attested_on` 14 Jan 1991, no `from` |
| SU-GOV-06 | August 1991: Pavlov's dismissal and the Cabinet's | **Accepted in part:** his last signature and the interim direction entrusted to the First Deputy Premier (order 943р, 19 Aug; holder 4); decree УП-2443 (22 Aug) releasing him and referring the decision to the Supreme Soviet; УП-2444; the agenda acts; 2366-I agreeing (28 Aug); the confidence question (УП-2461) and no confidence (2367-I); the session report; 2371-I's retrospective styling. No `until`: an integrator ruling is requested (22 or 28 Aug) |
| SU-GOV-07 | August-December 1991: the committee that took over the government's functions and its head | **Accepted in part:** УП-2461 and 2367-I create the Committee; УП-2528 gives it the Cabinet's current-management functions; Law 2392-I Art. 5 creates the Inter-republican Economic Committee; seven committee acts of September-November signed "И. Силаев" as head or Chairman, from 28 Nov as "Премьер-министр Экономического сообщества". Claims on the institution only; no holder |
| SU-GOV-08 | The Chairman of the Supreme Soviet's end in 1991, only as a source states it | **Accepted:** signatures of 19, 21 and 22 Aug (holder `attested_on` 22 Aug); the Presidium's removal from chairing (22 Aug, by reference), the failed and the adopted approval, the chamber chairmen presiding, the suspension (26 Aug, 2361-I), his statement of 24 Aug, the consent to arrest (29 Aug), the Congress's vote (1702-20-78) and resolution 2389-I releasing him (4 Sep). No `until`: 2389-I states no effective moment; an integrator ruling is requested |
| SU-GOV-09 | The September 1991 reorganisation of the Supreme Soviet and its later chairs | **Accepted in part:** Law 2392-I (5 Sep 1991; gazette and GARF original) makes two chambers and creates no Chairman of the Supreme Soviet; the Presidium under the chamber chairmen (5 Sep); УП-2663 moves the first sitting to 21 Oct 1991. The chamber chairmen elected in October are leads |
| SU-GOV-10 | The last records of both offices in December 1991 | **Accepted in part:** ГС-13 abolishes the Union ministries from 1 Dec (reproduced 23 Nov); the latest Interstate Economic Committee and Committee acts (17 and 19 Dec); RSFSR decree 299 (19 Dec) abolishes the Committee; the RSFSR recalls its deputy groups (12 Dec). No lifecycle end; the chambers' acts of 26 Dec remain leads |

### Holders

`su_government_head` (new role):

| # | Name | `attested_on` | `from` | `until` | Claim |
|---|---|---|---|---|---|
| 1 | Николай Иванович Рыжков | 1990-01-12 | null | null | `su_sprsfsr_60_ryzhkov_signs_as_chairman_19900112` |
| 2 | Николай Иванович Рыжков | 1990-11-24 | null | null | `su_ips_cm_1177_ryzhkov_signs_as_chairman_19901124` |
| 3 | Валентин Сергеевич Павлов | 1991-01-14 | null | null | `su_izv13_vs_approves_pavlov_premier_19910114` |
| 4 | Валентин Сергеевич Павлов | 1991-08-19 | null | null | `su_km_943r_pavlov_signs_as_premier_19910819` |

`su_supreme_soviet_chair` (existing role; the CLAUDE-C01-05 observation is row 1, unchanged):

| # | Name | `attested_on` | `from` | `until` | Claim |
|---|---|---|---|---|---|
| 1 | Mikhail Gorbachev (signature: M. Gorbachev) | 1990-03-14 | null | null | `su_gorbachev_chair_signature_19900314` |
| 2 | Анатолий Иванович Лукьянов | 1990-03-15 | null | null | `su_garf_1362i_lukyanov_signs_as_chairman_19900315` |
| 3 | Анатолий Иванович Лукьянов | 1991-08-22 | null | null | `su_lukyanov_signs_presidium_2353i_2354i_19910822` |

One rule applies to every holder in both roles. `from` would need a source that states the day office was assumed or took
effect, and `until` one that states the day it ended; none does, so every observation carries `attested_on` only. Each person
has an observation at the earliest and at the latest reviewed attestation in the period, and each observation rests on one
claim dated that day. Intermediate and pre-period signatures (24 December 1989 and 26 May 1990 for Ryzhkov; 27 December 1990,
14 January 1991, 19 and 21 August 1991 for Lukyanov) are role claims. The Supreme Soviet's approval, the President's
submission, the release decree, the agreement with it, the suspension, the resignation statement, the consent to arrest, the
Congress's vote and release, illness reports and retrospective stylings are dated claims and never boundaries, and no end is
taken from a successor's approval or signature.

Names. `holder_name` in the extracts and `name` in `ussr.json` are the canonical given-name, patronymic, surname form, taken
from a form printed in full in this packet's sources (the CLAUDE-C01-19 convention): "Николай Иванович Рыжков" (the stenogram
of 26 December 1990), "Валентин Сергеевич Павлов" (the 14 January 1991 resolution and decree УП-2443), "Анатолий Иванович
Лукьянов" (2368-I, 2389-I) and "Михаил Сергеевич Горбачев" (1362-I). Each row keeps the printed form and case in
`printed_name` (for example "Н. Рыжков (initial and surname only)", "А.ЛУК'ЯНОВ (initial and surname only; Ukrainian
text)"). Claim texts name people only as printed. People who hold no role in the row (the chamber chairmen, the First Deputy
Premier, a Deputy Chairman, the head of the interim committees) appear only in `persons_named`, with `holder_name` null;
Силаев is printed only with initials, so no fuller form is given. The CLAUDE-C01-05 holders keep their English names
("Mikhail Gorbachev"); reconciling the two conventions in `ussr.json` is left to the integrator.

### Date ledger

Each row is a separate dated fact with its own claim; different dates for the approval, the release decree, the agreement with
it, the suspension, the consent to arrest and the release are not inconsistencies. `attested_on` is the date of the act or of
the event the source dates.

| Date | Events | Holder field and claims |
|---|---|---|
| 24 Dec 1989 | in-office signature | `su_sprsfsr_59_ryzhkov_signs_as_chairman_19891224` |
| 12 Jan 1990 | in-office signature | head-of-government holder `attested_on`; `su_sprsfsr_60_ryzhkov_signs_as_chairman_19900112` |
| 15 Mar 1990 | election of the President; in-office signature | chair holder `attested_on`; `su_garf_1362i_gorbachev_elected_president_19900315`, `su_garf_1362i_lukyanov_signs_as_chairman_19900315` |
| 26 May 1990 | in-office signature | `su_ips_cm_525_ryzhkov_signs_as_chairman_19900526` |
| 24 Nov 1990 | in-office signature | head-of-government holder `attested_on`; `su_ips_cm_1177_ryzhkov_signs_as_chairman_19901124` |
| 26 Dec 1990 | procedure; illness reported; the Congress's message; continuation of powers | `su_rada_1861i_cabinet_chapter_19901226`, `su_rada_1861i_supreme_soviet_chair_provisions_19901226`, `su_steno4_ryzhkov_heart_attack_reported_19901226`, `su_steno4_congress_telegram_to_ryzhkov_19901226`, `su_law_1861i_premier_appointment_release_procedure_19901226`, `su_law_1861i_no_confidence_procedure_19901226`, `su_law_1862i_council_of_ministers_retains_powers_19901226` |
| 27 Dec 1990 | report for the government; illness reported; in-office signature | `su_steno4_pavlov_reports_for_government_19901227`, `su_steno4_ryzhkov_health_report_19901227`, `su_rada_1870i_lukyanov_signs_as_chairman_19901227` |
| 10 Jan 1991 | a Deputy Chairman's signature | `su_ips_cm_27_deputy_chairman_signs_19910110` |
| 14 Jan 1991 | the President's submission recited; the Supreme Soviet's approval; in-office signature | head-of-government holder `attested_on`; `su_izv13_president_submission_recited_19910114`, `su_izv13_vs_approves_pavlov_premier_19910114`, `su_izv13_lukyanov_signs_as_vs_chair_19910114` |
| 19 Aug 1991 | in-office signature; interim direction entrusted | head-of-government holder `attested_on`; `su_km_943r_pavlov_signs_as_premier_19910819`, `su_km_943r_general_direction_entrusted_19910819`, `su_lukyanov_signs_res_2350i_19910819` |
| 21 Aug 1991 | in-office signature | `su_lukyanov_signs_presidium_2352i_19910821` |
| 21-22 Aug 1991 | Presidium under the chamber chairmen | `su_ved35_presidium_under_chamber_chairs_19910821` |
| 22 Aug 1991 | request to the President; removal from chairing (by reference); in-office signature; agenda proposed; release decree; criminal case stated; referral to the Supreme Soviet; ГКЧП members removed | chair holder `attested_on`; `su_sten_request_to_president_reference_19910822`, `su_ss_res_2361i_presidium_removal_reference_19910822`, `su_lukyanov_signs_presidium_2353i_2354i_19910822`, `su_ss_presidium_2354i_government_changes_agenda_19910822`, `su_ukaz_up2443_pavlov_released_19910822`, `su_ukaz_up2443_criminal_case_stated_19910822`, `su_ukaz_up2443_referred_to_vs_session_19910822`, `su_ukaz_up2444_gkchp_members_removed_19910822` |
| 24 Aug 1991 | resignation statement; confidence question; committee created | `su_sten_lukyanov_statement_19910824`, `su_ukaz_up2461_cabinet_confidence_question_19910824`, `su_ukaz_up2461_committee_created_19910824` |
| 26 Aug 1991 | chamber chairman presiding; statement read out; approval not voted (no quorum); agenda adopted; removal approved; chamber chairmen to preside; resignation statement referred; suspension | `su_sten_laptev_presiding_19910826`, `su_sten_statement_read_out_19910826`, `su_sten2_laptev_presiding_evening_19910826`, `su_sten2_presidium_removal_not_approved_no_quorum_19910826`, `su_ss_res_2360i_cabinet_confidence_agenda_19910826`, `su_ss_res_2361i_approves_presidium_removal_19910826`, `su_ss_res_2361i_chamber_chairs_preside_19910826`, `su_ss_res_2361i_resignation_statement_reference_19910826`, `su_ss_res_2361i_suspension_19910826` |
| 28 Aug 1991 | agreement with the release; no confidence; committee created; styling reported; agreement reported; no confidence reported | `su_vs_2366i_agrees_to_pavlov_release_19910828`, `su_vs_2367i_no_confidence_in_cabinet_19910828`, `su_vs_2367i_committee_until_new_cabinet_19910828`, `su_ved36_lukyanov_styled_chairman_19910828`, `su_ved36_consent_to_pavlov_release_reported_19910828`, `su_ved36_no_confidence_reported_19910828` |
| 29 Aug 1991 | consent to arrest; retrospective styling | `su_ss_res_2368i_lukyanov_arrest_consent_19910829`, `su_vs_2371i_pavlov_named_among_organizers_19910829` |
| 31 Aug 1991 | chamber chairman closes the session | `su_ved36_laptev_closes_session_19910831` |
| 4 Sep 1991 | the Congress's vote; committee act signed; release; release reported | `su_snd5_lukyanov_release_vote_19910904`, `su_kou_23r_silayev_signs_as_committee_head_19910904`, `su_cpd_res_2389i_lukyanov_released_19910904`, `su_ved37_release_reported_19910904` |
| 5 Sep 1991 | procedure; Presidium under the chamber chairmen | `su_garf_law_2392i_signed_original_19910905`, `su_law_2392i_bicameral_structure_19910905`, `su_law_2392i_mek_chairman_procedure_19910905`, `su_law_2392i_transition_and_entry_into_force_19910905`, `su_ved37_presidium_under_chamber_chairs_19910905` |
| 6 Sep 1991 | former Cabinet apparatus continues; Cabinet's functions assigned | `su_kou_25r_former_cabinet_apparatus_continues_19910906`, `su_ukaz_up2528_committee_performs_cabinet_functions_19910906` |
| 3 Oct 1991 | first sitting moved | `su_ukaz_up2663_new_ss_first_sitting_moved_19911003` |
| 5 Oct 1991 | acts for the Government of the USSR | `su_kou_62r_guest_of_ussr_government_19911005` |
| 10 Oct 1991 | committee act signed | `su_mek_2r_silayev_signs_as_chairman_19911010` |
| 12 Nov 1991 | committee act signed | `su_mek_6r_silayev_signs_as_chairman_19911112` |
| 14 Nov 1991 | Union ministries abolished | `su_gs13_abolition_of_union_ministries_19911114` |
| 15 Nov 1991 | committee act signed | `su_mgek_7r_silayev_signs_as_chairman_19911115` |
| 23 Nov 1991 | abolition announced | `su_kou_53_announces_gs13_19911123` |
| 28 Nov 1991 | committee act signed | `su_mgek_7_signed_premier_of_economic_community_19911128` |
| 12 Dec 1991 | deputy groups recalled | `su_rsfsr_2017i_recalls_deputy_groups_19911212` |
| 17 Dec 1991 | committee act signed | `su_mgek_23r_latest_signature_19911217` |
| 19 Dec 1991 | committee act signed; Committee abolished (RSFSR) | `su_kou_212r_latest_signature_19911219`, `su_rsfsr_ukaz_299_abolishes_committee_19911219` |

### Identifier changes from the dossiers

| Dossier | This packet | Why |
|---|---|---|
| `su_garf_exhibit_referendum_res_19901224`, `su_garf_referendum_res_lukyanov_autograph_19901224` (A) | a lead; replaced by `su_rada_res_1870i_19901227` / `su_rada_1870i_lukyanov_signs_as_chairman_19901227` | Caption-only attestation (A1) |
| `su_rada_1861i_cabinet_of_ministers_created_19901226` (A) | `su_rada_1861i_cabinet_chapter_19901226` | The law renames the chapter; it does not say it creates the Cabinet (A4) |
| `su_rada_1861i_prime_minister_procedure_19901226` (A) | not imported; the same items are `su_law_1861i_premier_appointment_release_procedure_19901226` and `su_law_1861i_no_confidence_procedure_19901226`, from the Russian text in the stenographic report | One claim per provision; the officially published Russian text is preferred for wording |
| `su_ved_1991_35_scan`, `su_ved_1991_36_scan` (B) and `su_ved_1991_35`, `su_ved_1991_36` (C) | `su_ved_1991_35`, `su_ved_1991_36`, recorded as their pre-cutoff captures | One source per gazette issue (C1) |
| `su_ukaz_up2461_confidence_question_19910824`, `su_ukaz_up2461_committee_headed_by_silayev_19910824`, `su_ss_res_2367i_cabinet_no_confidence_19910828`, `su_ss_res_2367i_committee_interim_19910828` (C) | part B's `su_ukaz_up2461_cabinet_confidence_question_19910824`, `su_ukaz_up2461_committee_created_19910824`, `su_vs_2367i_no_confidence_in_cabinet_19910828`, `su_vs_2367i_committee_until_new_cabinet_19910828` | Duplicates of the same articles (C1, C2) |
| `su_vs_session_report_confidence_item_19910828` (B) | `su_ved36_consent_to_pavlov_release_reported_19910828` and `su_ved36_no_confidence_reported_19910828` | Two events in one claim (B6) |
| `su_ss_res_2361i_presidium_removal_reference_19910822` (C) | the same ID (the 22 August reference only) plus `su_ss_res_2361i_approves_presidium_removal_19910826` | Two events in one claim (C3) |
| event kinds `dismissal`, `acting_service_designation`, `committee_head_designation`, `resignation_request_reference`, `in_office_signature`, `parliamentary_approval`, `session_record`, `government_act_signature` | `release_decree_referred_to_legislature`, `committee_created`, `request_reference`, `in_office_attestation`, `appointment_approval`, `session_record_consent_to_release` / `session_record_no_confidence`, `acts_in_name_of_union_government` | B7, C2, C4 and one vocabulary |
| holder keys `holder_name`, `supporting_claims`, `role_title_printed` (A) | `name`, `sources`, `claim_ids`, `note`, `uncertainty` in `ussr.json`; `holder_name` with `printed_name` or `persons_named` in extract rows | A6 |
| (none) | `su_sprsfsr_1990_8_art59_19891224`, `su_sprsfsr_1990_8_art60_19900112`, `su_rada_res_1870i_19901227`, `su_sten_vs_bulletin2_19910826`, `su_snd5_bulletin5_19910904`, and five claims on recorded sources (the telegram, 2354-I and 2360-I agenda items, the 2361-I approval, the split session report) | Primary records the checks found (A1, A2, B10, B11, C3 and the checks' missing records) |
| (none) | `su_ips_cm_res_1177_19901124`, `su_ips_cm_res_27_19910110`, `su_km_rasp_943r_19910819` | Found by this packet |

## Observations

### SU-GOV-01 — The Chairman of the Council of Ministers when the period opens

Evidence:

- The official gazette of the RSFSR Government, *Собрание постановлений Правительства РСФСР* 1990 No. 8 (page scans in the
  Russian Historical Society's library; check find, A2), prints two joint resolutions of the USSR and RSFSR Councils of
  Ministers signed "Председатель Совета Министров СССР Н. Рыжков" and "Председатель Совета Министров РСФСР А. Власов": art. 59,
  "Москва, 24 декабря 1989 г. № 1139" (printed pp. 194-202), and art. 60, "Москва, 12 января 1990 г. № 37" (printed pp.
  203-214). This packet viewed all four page images.
- USSR Council of Ministers resolution No. 525 of 26 May 1990 on the legal portal, signed "Председатель Совета Министров СССР
  Н.РЫЖКОВ".

Decision: **accepted in part**. Holder 1, Николай Иванович Рыжков, `attested_on` 12 January 1990 (art. 60). The 24 December
1989 signature is a pre-period role claim and the 26 May 1990 signature an intermediate one; neither is a holder. No source
dated 1 January 1990 names the Chairman, and neither signature proves office on that day.

Limits: the gazette pages are the library's public small renditions (about 328 x 480 px, readable); the library's document
pages change on every request and are locators only. His 1989 appointment and the Council of Ministers' own gazette were not
reviewed.

### SU-GOV-02 — The Chairman of the Supreme Soviet from March 1990

Evidence:

- The GARF original of Congress resolution 1362-I of 15 March 1990 (Rosarkhiv exhibit 09-30, raw captures of December 2019;
  facsimile viewed at full size): "Избрать Президентом Союза Советских Социалистических Республик товарища ГОРБАЧЕВА Михаила
  Сергеевича", signed over "Председатель Верховного Совета СССР (А.Лукьянов)" with an autograph and the Congress's seal.
- Congress resolution 1870-I of 27 December 1990 (the Verkhovna Rada database's Ukrainian text, raw capture of 31 May 2025;
  check find, A1), signed "Голова Верховної Ради СРСР А.ЛУК'ЯНОВ".
- The Supreme Soviet's resolution of 14 January 1991 on the Premier, signed "Председатель Верховного Совета СССР А. ЛУКЬЯНОВ"
  (see SU-GOV-05).

Decision: **accepted in part**. Holder, Анатолий Иванович Лукьянов, `attested_on` 15 March 1990: he signs as Chairman the
original of an act of that date; the date of signature is not stated (A5), and 1362-I is numbered before the resolution that
elected him, so he signed after its adoption. The 1870-I and 14 January 1991 signatures are role claims. The election of the
President is filed as a role claim on `su_president`; its holders are unchanged. The existing observation of 14 March 1990
keeps no end: neither 1362-I nor the successor's signature states that the earlier chairmanship ended.

Limits: Congress resolution 1367-I electing him, the vote and the moment he took office are known only from a commercial
database (a lead). Rosarkhiv exhibit 09-32 (24 December 1990) rests on a caption only; its facsimile was never archived (a lead).

### SU-GOV-03 — The constitutional change creating the Cabinet of Ministers (procedure only)

Evidence:

- Law 1861-I of 26 December 1990 in the Verkhovna Rada database (Ukrainian text, raw capture of 31 May 2025): item 19 renames
  Chapter 16 "Кабінет Міністрів СРСР"; items 20-28 rewrite Articles 128-136 (the Cabinet subordinate to the President; composed
  of the Prime Minister, his deputies and ministers; Article 134 deleted; competence left to a USSR law); items 2, 4, 10, 11
  and 29 substitute the Cabinet in Articles 77, 96, 122, 124 and 140. Item 17 names the Vice-President and then the Chairman of
  the Supreme Soviet in renumbered Article 127-7, where the article had named the Chairman of the Supreme Soviet and then the
  Chairman of the Council of Ministers; item 18 lets the Chairman attend the Federation Council.
- The same law as printed in Russian in the Fourth Congress stenographic report, vol. III (Издание Верховного Совета СССР,
  1991): new Article 113(3), the Supreme Soviet "по представлению Президента СССР утверждает Премьер-министра, дает согласие на
  сессии либо отклоняет кандидатуры членов Кабинета Министров СССР и членов Совета безопасности СССР, дает согласие на
  освобождение от должности указанных лиц"; new Article 127-3(6), the President forms the Cabinet, presents the candidate for
  Premier and, by agreement with the Supreme Soviet, releases the Premier; Article 130, a two-thirds no-confidence vote "влечет
  его отставку".
- Law 1862-I of the same day (same volume) puts 1861-I into force on adoption and keeps the Council of Ministers' powers "впредь
  до сформирования в соответствии с названным Законом государственных органов и назначения должностных лиц".

Decision: **accepted in part**, as procedure and continuation claims only. The law renames and rewrites; it does not say it
creates the Cabinet (A4), and neither law dates the end of the Council of Ministers or the Cabinet's formation, so
`su_government` has no lifecycle dates and one identity under both titles. Whether "указанных лиц" in Article 113(3) includes
the Premier is not decided; the Premier's release rests on Article 127-3(6) (B4).

Limits: the Law "О Кабинете Министров СССР" of 20 March 1991 (2033-I) and its enactment resolution were not found on an
official host or in a facsimile (leads). The Rada text is a Ukrainian translation; its footer's 2022 status note is not used.

### SU-GOV-04 — Ryzhkov's end

Evidence:

- Council of Ministers resolution 1177 of 24 November 1990 (legal portal; packet find), signed "Председатель Совета
  Министров СССР Н.РЫЖКОВ": his latest reviewed signature.
- The Fourth Congress stenographic report, vol. III: on 26 December 1990 (Lukyanov presiding) Gorbachev reports "вчера ночью у
  Николая Ивановича Рыжкова произошел сердечный приступ - инфаркт"; the Congress sends a telegram to "Николаю Ивановичу
  Рыжкову" wishing him a recovery (check find, B11); on 27 December the presiding officer says "было предложено В. С. Павлову
  доложить от имени правительства, потому что Н. И. Рыжков болен", Pavlov's speaker line reads "Павлов В. С., Министр финансов
  СССР", and Gorbachev answers a question "О состоянии здоровья Н. И. Рыжкова".
- Council of Ministers resolution 27 of 10 January 1991 (legal portal; packet find), signed "Зам. Председателя Совета
  Министров СССР Л. ВОРОНИН".

Decision: **accepted in part**. Holder 2, Николай Иванович Рыжков, `attested_on` 24 November 1990, a separate observation from
12 January 1990. Everything else is a claim. No reviewed source states the day his chairmanship ended, a dismissal, resignation
or retirement, or names an acting Chairman: the report "on behalf of the government" was given by the Minister of Finance, and
a Deputy Chairman signed as deputy. No `until`; Pavlov's approval of 14 January 1991 is not used as an end.

Limits: Supreme Soviet resolution 1907-I of 15 January 1991 on his pension (Vedomosti 1991 No. 4, art. 84) is known only from a
commercial database, whose text also states no end date; the Vedomosti scan is behind a paywall. The stenogram's four passages
print no office title for him, and their rows say so.

### SU-GOV-05 — Pavlov's approval as Premier

Evidence: Izvestia No. 13 (23279) of 15 January 1991, Moscow evening edition, page 1, prints the Supreme Soviet's resolution "О
премьер-министре СССР", "Москва, Кремль. 14 января 1991 г.": "Рассмотрев представление Президента СССР, Верховный Совет СССР
постановляет: Утвердить премьер-министром СССР товарища Павлова Валентина Сергеевича", signed "Председатель Верховного Совета
СССР А. ЛУКЬЯНОВ". It is read from the OCR embedded in the Yandex archive page (where the two columns are interleaved) in the
order of the page image; Izvestia is used only as the act's official publication.

Decision: **accepted**. Holder 3, Валентин Сергеевич Павлов, `attested_on` 14 January 1991, on the approval (event kind
`appointment_approval`): under Article 113(3) the Supreme Soviet's approval is the appointing step on the constitutional text,
and no presidential appointment decree was found. The recital of the President's submission is a separate claim. No `from`:
the resolution has no effect clause and states no day on which he took office.

Limits: the recorded identity is a dynamic rendering (see [Response identities and stability
checks](#response-identities-and-stability-checks)). Izvestia prints no act number; 1900-I and Vedomosti 1991 No. 4, art. 80
come from leads. The only compilation of presidential acts searched is non-official and lacks УП-1307 to 1309 (11-15 January
1991), so a presidential act cannot be excluded (B8). The President's submission, the vote (279-75 in Izvestia's news report,
a lead) and the Supreme Soviet stenogram of 14 January were not obtained.

### SU-GOV-06 — Pavlov's release and the Cabinet's

Evidence:

- Cabinet of Ministers order 943р of 19 August 1991 (legal portal; packet find): "В связи с невозможностью выполнения мною из-за
  болезни обязанностей Премьер-министра СССР временно возложить на первого заместителя Премьер-министра СССР т. Догужиева В. Х.
  общее руководство работой Кабинета Министров СССР", signed "Премьер-министр СССР В. Павлов".
- Vedomosti 1991 No. 35: Presidium resolution 2354-I (22 August) proposes agenda item 5 "Об изменениях в составе Правительства
  СССР и других государственных органов" (check find); decree УП-2443 (22 August) releases "Павлова Валентина Сергеевича от
  обязанностей Премьер-министра СССР" on the ground of a criminal case and refers the decision to the Supreme Soviet; УП-2444
  removes all ГКЧП members from their posts, naming nobody; УП-2461 (24 August) puts the question of confidence in the Cabinet;
  resolution 2360-I (26 August) puts "О доверии Кабинету Министров СССР" on the agenda (check find).
- Vedomosti 1991 No. 36: resolution 2366-I (28 August), "Согласиться с Указом Президента СССР от 22 августа 1991 года";
  2367-I (28 August) expresses no confidence in the Cabinet; the session report's 28 August paragraph records the agreement and
  the no-confidence resolution (two claims, B6); resolution 2371-I (29 August) lists "Премьер-министр СССР Павлов В. С." among
  the organisers of the coup.

Decision: **accepted in part**. Holder 4, Валентин Сергеевич Павлов, `attested_on` 19 August 1991, his latest reviewed
signature. The interim direction entrusted to the First Deputy Premier is a claim only: it gives no acting title, names him
only as "т. Догужиева В. Х." and is never a holder. **No `until`**: УП-2443 has no effect clause and refers the decision to the
Supreme Soviet, and 2366-I agrees without a day of effect (B7). An integrator ruling is requested on whether 22 or 28 August is
a stated end; whatever it is must apply to every holder. The Cabinet-level acts (the confidence question, the agenda items, no
confidence and its report) are claims on `su_government`, not on the role. No reviewed primary source states Pavlov's arrest.

### SU-GOV-07 — The Committee that took over the government's functions

Evidence: УП-2461 point 2 (24 August) creates a Committee "во главе с Силаевым И. С. (руководитель)"; 2367-I point 2 (28
August) creates the Committee for Operational Management of the National Economy "во главе с тов. Силаевым И. С." until a new
Cabinet is formed; УП-2528 (6 September) has it perform the functions of current management "возлагавшиеся ранее на Кабинет
Министров СССР"; Law 2392-I, Article 5 (5 September), creates the Inter-republican Economic Committee, whose Chairman the
President appoints with the State Council's consent. On the legal portal: order 23р (4 September) signed "Руководитель
Комитета И. Силаев"; order 25р (6 September) keeps the apparatus of "бывшего Кабинета Министров СССР" at work; order 62р (5
October) receives a foreign Prime Minister "в качестве гостя Правительства СССР"; orders 2-р (10 October) and 6-р (12
November) of the Inter-republican Economic Committee and 7р (15 November) of the Interstate Economic Committee, signed
"Председатель Комитета И. Силаев"; the Interstate Economic Committee's resolution 7 (28 November), signed "Председатель
Комитета - Премьер-министр Экономического сообщества И. СИЛАЕВ", which appoints "т. Родникову О. В." (C8).

Decision: **accepted in part**, as claims on `su_government` only. No primary source calls the Committee the Union government
or its head the head of the Union government; the arrangement is interim, and the committee rows name nobody as holder (B5,
C2). No holder on `su_government_head`. Whether the committees count as the Union government is a modelling decision for the
integrator.

Limits: the decrees appointing Силаев Chairman of the Inter-republican Economic Committee (Vedomosti 1991 No. 39) and of the
Interstate Economic Committee and Premier of the Economic Community (No. 48) were seen only in an index transcription (leads).

### SU-GOV-08 — The Chairman of the Supreme Soviet's end

Evidence:

- Vedomosti 1991 No. 35: resolution 2350-I (19 August) and Presidium resolutions 2352-I (21 August), 2353-I and 2354-I (22
  August), each signed "Председатель Верховного Совета СССР А. ЛУКЬЯНОВ"; the gazette's report that the Presidium met on 21-22
  August under the chamber chairmen "И. Д. Лаптева и Р. Н. Нишанова" and entrusted the chairing of the session to them in turn;
  resolution 2361-I (26 August), which cites the Presidium's resolution of 22 August removing him from conducting sittings,
  approves it and entrusts the sittings to the chamber chairmen (point 1), refers his "Заявление об отставке" to agenda item 3
  (point 2) and suspends his exercise of the Chairman's duties (point 3).
- The stenographic bulletins of 26 August: the morning sitting opens under "Председатель Совета Союза ... И. Д. Лаптев", and
  the statement signed "А. Лукьянов, 24.08.1991 года" is read ("считаю для себя невозможным оставаться в настоящее время на посту
  Председателя Верховного Совета СССР"); in the evening (check find) a proposal to approve the Presidium's removal cannot be
  voted for lack of a quorum (290 registered) before a break to 20:30.
- Vedomosti 1991 No. 36: the 28 August report styles him "Председатель Верховного Совета СССР А. И. Лукьянов" giving an
  explanation; resolution 2368-I (29 August) consents to the prosecution and arrest of "народного депутата СССР Лукьянова
  Анатолия Ивановича"; Laptev sums up the session on 31 August.
- The Fifth Congress's bulletin No. 5 (4 September, Gorbachev presiding; check find): "Постановление об освобождении А. И.
  Лукьянова принято", 1702 for, 20 against, 78 abstaining; Vedomosti 1991 No. 37 prints resolution 2389-I, "Освободить
  Лукьянова Анатолия Ивановича от обязанностей Председателя Верховного Совета СССР", and reports the release; the Congress's own
  acts publication prints the same text (a corroborating response).

Decision: **accepted**. Holder, Анатолий Иванович Лукьянов, `attested_on` 22 August 1991 (2353-I and 2354-I), his latest
reviewed signatures; those of 19 and 21 August are role claims. Every other event is its own dated claim, and the chamber
chairmen presiding is recorded with no holder. The report that the Presidium met under the chamber chairmen on 21-22 August sits
uneasily with acts of those days printed with his signature; both are recorded, not reconciled. **No `until`**: 2389-I and the
vote state no effective moment. An integrator ruling is requested on whether the Congress's release is a stated end; if it is,
it should rest on `su_cpd_res_2389i_lukyanov_released_19910904`, and the same rule must apply to the Premier's release. His
arrest has no primary source.

### SU-GOV-09 — The September 1991 reorganisation

Evidence: Law 2392-I of 5 September 1991 (Vedomosti 1991 No. 37 and the GARF original, Rosarkhiv exhibit 09-42): in the
transitional period the Supreme Soviet consists of two independent chambers, the Council of Republics and the Council of the
Union (Articles 1-2); the existing Supreme Soviet keeps its powers until the new one begins, its session convened by the chamber
chairmen (Article 7); the law enters into force on publication, except Article 2 (Article 9). The Presidium met under the
chamber chairmen on 5 September. Decree УП-2663 (3 October) moves the new Supreme Soviet's first sitting to 21 October 1991.

Decision: **accepted in part**, as procedure and body-level claims. The law creates no Chairman of the Supreme Soviet, so
`su_supreme_soviet_chair` has no later holder, and the chamber chairmanships are separate offices, not holders of this role.

Limits: the October elections of the chamber chairmen (Vedomosti 1991 No. 45, arts. 1249-1254, "А. Т. Алимжанов" and "К. Д.
Лубенченко") were seen only in an index transcription and in Izvestia news; whether to add chamber roles is an integrator
decision. The publication day that brought Law 2392-I into force is not verified.

### SU-GOV-10 — The last records of both offices in December 1991

Evidence: State Council resolution ГС-13 of 14 November 1991, reproduced in the Committee's resolution 53 of 23 November,
abolishes the Union ministries and the bodies under the Cabinet, their management functions ceasing from 1 December 1991; the
Interstate Economic Committee's order 23р of 17 December and the Committee's order 212р of 19 December, the latest the portal
lists, are signed by Силаев; RSFSR decree 299 of 19 December abolishes the Committee and ends the Interstate Economic
Committee's activity on RSFSR territory; RSFSR resolution 2017-I of 12 December recalls the RSFSR deputy groups from both
chambers.

Decision: **accepted in part**, as claims on the two institutions. The two RSFSR acts are acts of a republic body, filed in
`ussr.json` as claims about Union bodies; `russia.json` is not changed and no mapping is made. No lifecycle end is set for
`su_government` or `su_supreme_soviet`.

Limits: Declaration 142-N of the Council of Republics and order 141-N (26 December) remain leads (SURU-TR91-08).

## Sources added

31 sources, each with a checked-in derived factual extract under [sources/](sources/) (`ussr-*-facts.json`, LF, format
`spheres-c01-derived-factual-table/v1`, with its own checksum in the packet; 196,601 bytes in all). Each extract
records the original response's URL, byte count and SHA-256, the attached responses, a stability record and one row per claim
(claim_id, observation, role, `holder_name` with the printed form or `persons_named`, role title, event kind, date, text,
locator). Original pages, PDFs and images are not checked in; no emblem, seal, signature image or photograph is republished.
"Check find" marks a primary record found by an independent check, "packet find" one found by this packet.

| Source ID | What | Response identity (bytes, SHA-256) and read path |
|---|---|---|
| `su_sprsfsr_1990_8_art59_19891224` | [Собрание постановлений Правительства РСФСР 1990 No. 8, art. 59 (24 Dec 1989), pp. 202 and 194](https://docs.historyrussia.org/system/pages/016/529/17/images/small/2deefa51e6a9fe2dc161dca537b10f3eed802f18.jpg?1716491941) (check find) | static image, 30,424 bytes, `d795d3dd…91a236`; image 37,089 `55bda52a` |
| `su_sprsfsr_1990_8_art60_19900112` | [Собрание постановлений Правительства РСФСР 1990 No. 8, art. 60 (12 Jan 1990), pp. 214 and 203](https://docs.historyrussia.org/system/pages/016/529/29/images/small/08390f990d8fd8b5127cefd8c1c749b501710421.jpg?1716491978) (check find) | static image, 31,779 bytes, `20bd55b2…78d65f`; image 38,198 `3ab7bcd6` |
| `su_garf_exhibit_res_1362i_19900315` | [Rosarkhiv 09-30: Congress resolution 1362-I, 15 Mar 1990 (GARF original)](https://web.archive.org/web/20191207080438id_/http://projects.rusarchives.ru/statehood/09-30-postanovlenie-prezident.shtml) | IA 20191207080438, 9,423 bytes, `e2ab870b…13ad95`; image 157,538 `543c9efb` |
| `su_ips_cm_res_525_19900526` | [Council of Ministers resolution 525, 26 May 1990](https://pravo.gov.ru/proxy/ips/?doc_itself=&nd=102010231&page=1&rdk=0) | HTTP, 52,511 bytes, `889da9f7…6c14c9`; card 2,936 `93cfc7cb` |
| `su_ips_cm_res_1177_19901124` | [Council of Ministers resolution 1177, 24 Nov 1990](https://pravo.gov.ru/proxy/ips/?doc_itself=&nd=102084027&page=1&rdk=0) (packet find) | HTTP, 52,149 bytes, `6ed687d8…5b0348` |
| `su_rada_law_1861i_19901226` | [Law 1861-I, 26 Dec 1990 (Rada database, Ukrainian text)](https://web.archive.org/web/20250531192616id_/https://zakon.rada.gov.ua/laws/show/v1861400-90/print) | IA 20250531192616, 42,384 bytes, `c7919fd2…705b35` |
| `su_snd4_steno_vol3` | [Fourth Congress stenographic report, vol. III (26-27 Dec 1990)](https://web.archive.org/web/20250718143642id_/https://snd.sssr.su/IV/III.pdf) | IA 20250718143642, 12,653,342 bytes, `398d2fa5…49b897`; live file identical |
| `su_rada_res_1870i_19901227` | [Congress resolution 1870-I, 27 Dec 1990 (Rada database, Ukrainian text)](https://web.archive.org/web/20250531195947id_/https://zakon.rada.gov.ua/laws/show/v1870400-90/print) (check find) | IA 20250531195947, 11,254 bytes, `a78b05ae…e6dc62` |
| `su_ips_cm_res_27_19910110` | [Council of Ministers resolution 27, 10 Jan 1991](https://pravo.gov.ru/proxy/ips/?doc_itself=&nd=102010316&page=1&rdk=0) (packet find) | HTTP, 20,265 bytes, `09ae8fe9…9f123f` |
| `su_izvestia_19910115_no13_p1` | [Izvestia No. 13, 15 Jan 1991, page 1 (Yandex archive)](https://yandex.ru/archive/catalog/91392bb5-0841-4352-8d7f-55d76c362683/1) | dynamic HTML (recorded request), 171,140 bytes, `db9ec251…ea6847`; preview 161,461 `cf0204c6` |
| `su_km_rasp_943r_19910819` | [Cabinet of Ministers order 943р, 19 Aug 1991](https://pravo.gov.ru/proxy/ips/?doc_itself=&nd=102012314&page=1&rdk=0) (packet find) | HTTP, 6,925 bytes, `b967d16d…85e53a` |
| `su_sten_vs_bulletin1_19910826` | [Supreme Soviet stenographic bulletin No. 1, 26 Aug 1991 (morning)](https://sten.vs.sssr.su/12/6/1.pdf) | live static PDF, 2,252,696 bytes, `849b9dae…39776a` |
| `su_sten_vs_bulletin2_19910826` | [Supreme Soviet stenographic bulletin No. 2, 26 Aug 1991 (evening)](https://sten.vs.sssr.su/12/6/2.pdf) (check find) | live static PDF, 1,530,044 bytes, `22562b5b…3fd5bd` |
| `su_ved_1991_35` | [Vedomosti 1991 No. 35 (28 Aug 1991)](https://web.archive.org/web/20211204065955id_/https://vedomosti.sssr.su/1991/35.pdf) | IA 20211204065955, 618,372 bytes, `5a8c0630…c55a63`; live file identical |
| `su_ved_1991_36` | [Vedomosti 1991 No. 36 (4 Sep 1991)](https://web.archive.org/web/20250820135020id_/https://vedomosti.sssr.su/1991/36.pdf) | IA 20250820135020, 1,303,663 bytes, `87abb4c1…075b6f`; live file identical |
| `su_snd5_bulletin5_19910904` | [Fifth Congress bulletin No. 5, 4 Sep 1991](https://web.archive.org/web/20240901125154id_/https://snd.sssr.su/V/5.pdf) (check find) | IA 20240901125154, 1,620,327 bytes, `9ea2c893…227a65`; live file identical; acts publication 544,323 `236b00bd` |
| `su_kou_rasp_23r_19910904` | [Committee order 23р, 4 Sep 1991](https://pravo.gov.ru/proxy/ips/?doc_itself=&nd=102012407&page=1&rdk=0) | HTTP, 7,876 bytes, `62e4006a…35bedd` |
| `su_garf_exhibit_law_2392i` | [Rosarkhiv 09-42: Law 2392-I, 5 Sep 1991 (GARF original)](https://web.archive.org/web/20260415035300id_/https://projects.rusarchives.ru/statehood/09-42-postanovlenie-organy-gosvlasti.shtml) | IA 20260415035300, 14,839 bytes, `f6a4f3ea…50c2e9`; image 157,151 `c32c9177`; image 117,260 `044b8701` |
| `su_kou_rasp_25r_19910906` | [Committee order 25р, 6 Sep 1991](https://pravo.gov.ru/proxy/ips/?doc_itself=&nd=102012430&page=1&rdk=0) | HTTP, 7,340 bytes, `d29cc413…6629eb` |
| `su_ved_1991_37` | [Vedomosti 1991 No. 37 (11 Sep 1991)](https://vedomosti.sssr.su/1991/37.pdf) | live static PDF, 999,248 bytes, `3e77c34e…b603b0` |
| `su_kou_rasp_62r_19911005` | [Committee order 62р, 5 Oct 1991](https://pravo.gov.ru/proxy/ips/?doc_itself=&nd=102012665&page=1&rdk=0) | HTTP, 8,464 bytes, `00a8eed4…0f8e06` |
| `su_ved_1991_41` | [Vedomosti 1991 No. 41 (9 Oct 1991)](https://vedomosti.sssr.su/1991/41.pdf) | live static PDF, 556,781 bytes, `91cf6571…ff902e` |
| `su_mek_rasp_2r_19911010` | [Inter-republican Economic Committee order 2-р, 10 Oct 1991](https://pravo.gov.ru/proxy/ips/?doc_itself=&nd=102119002&page=1&rdk=0) | HTTP, 6,843 bytes, `cbf946f6…e5f378` |
| `su_mek_rasp_6r_19911112` | [Inter-republican Economic Committee order 6-р, 12 Nov 1991](https://pravo.gov.ru/proxy/ips/?doc_itself=&nd=102119005&page=1&rdk=0) | HTTP, 7,853 bytes, `f149257e…5b11d7` |
| `su_mgek_rasp_7r_19911115` | [Interstate Economic Committee order 7р, 15 Nov 1991](https://pravo.gov.ru/proxy/ips/?doc_itself=&nd=102013103&page=1&rdk=0) | HTTP, 7,150 bytes, `24434819…9a09ac` |
| `su_kou_post_53_19911123` | [Committee resolution 53, 23 Nov 1991 (reproducing ГС-13 of 14 Nov)](https://pravo.gov.ru/proxy/ips/?doc_itself=&nd=102013213&page=1&rdk=0) | HTTP, 21,408 bytes, `222e799e…b71431` |
| `su_mgek_post_7_19911128` | [Interstate Economic Committee resolution 7, 28 Nov 1991](https://pravo.gov.ru/proxy/ips/?doc_itself=&nd=102013298&page=1&rdk=0) | HTTP, 7,557 bytes, `4a9457c3…76ced8` |
| `su_rsfsr_res_2017i_19911212` | [RSFSR Supreme Soviet resolution 2017-I, 12 Dec 1991](https://pravo.gov.ru/proxy/ips/?doc_itself=&nd=102013526&page=1&rdk=0) | HTTP, 25,236 bytes, `4bfe7135…cfc1dc` |
| `su_mgek_rasp_23r_19911217` | [Interstate Economic Committee order 23р, 17 Dec 1991](https://pravo.gov.ru/proxy/ips/?doc_itself=&nd=102013609&page=1&rdk=0) | HTTP, 6,923 bytes, `168225eb…accdeb` |
| `su_kou_rasp_212r_19911219` | [Committee order 212р, 19 Dec 1991](https://pravo.gov.ru/proxy/ips/?doc_itself=&nd=102119006&page=1&rdk=0) | HTTP, 8,608 bytes, `e09fff16…d165ab` |
| `su_rsfsr_ukaz_299_19911219` | [RSFSR presidential decree 299, 19 Dec 1991](https://pravo.gov.ru/proxy/ips/?doc_itself=&nd=102013665&page=1&rdk=0) | HTTP, 24,105 bytes, `b73a9d55…24bf7e` |

Read paths. The legal portal refused HTTPS from this environment (port 443), so its texts were read over HTTP; the packet URL
gives the same path on HTTPS and each extract records the HTTP URL read (as CLAUDE-C01-05, C01-14 and C01-19 did). Rosarkhiv's
exhibition pages and images and the Verkhovna Rada database pages are read through raw Internet Archive captures (`id_`), all
made before the cutoff; the packet URL is the capture and `original_url` the captured page. Vedomosti Nos. 35 and 36, the
Fourth Congress volume and the Fifth Congress bulletin are recorded as their pre-cutoff captures, each byte-identical to the
live static file (`live_file_response`); Vedomosti Nos. 37 and 41 and the two Supreme Soviet bulletins have no capture and are
recorded as the live static files.

Hosting. The Vedomosti, stenograms and Congress bulletins are page-image facsimiles of official publications served by the
non-official SSSR.SU project (vedomosti.sssr.su, sten.vs.sssr.su, snd.sssr.su), and the 1989-1990 gazette pages come from the
Russian Historical Society's library. CLAUDE-C01-05 named "a gazette facsimile" as sufficient for acts of this kind
(SURU-TR91-08) while declining the same host's HTML transcription; this packet uses only the facsimile PDFs, and every such
source says so in its publisher. An integrator ruling on these hosts is requested (C12). If it goes against them, SU-GOV-04
keeps resolutions 1177 and 27 and SU-GOV-06 keeps order 943р (all on the legal portal), SU-GOV-01 keeps resolution 525 and
SU-GOV-02 keeps the GARF original, but SU-GOV-08 and most of SU-GOV-06, 07 and 09 lose their claims.

## Response identities and stability checks

Every recorded response was downloaded at least three times, and each identity is the response of a stored page or file, not
one generated per request, with one disclosed exception (Izvestia, below):

- the dossier's downloads, including a delayed re-download 30 minutes or more after the first and, for the static files,
  cache-busting requests;
- the independent check's downloads (for records a check found, its two or three downloads);
- this packet, twice for every recorded and attached response: 44 downloads at 2026-09-26T03:52:14Z-03:54:51Z and 44 at
  2026-09-26T04:25:49Z-04:27:36Z, 32-33 minutes apart for each response. Every one returned identical bytes and SHA-256. Three
  Internet Archive requests in the first pass failed to connect and succeeded on an immediate retry.

For the three records found by this packet, this packet's discovery download (03:49Z-03:51Z) is a third identical download; no
independent check has downloaded them.

Points a reviewer needs:

- **Izvestia No. 13 is a dynamic rendering** (B1). The Yandex archive page is served "no-store", sets tracking cookies and
  embeds the site's front-end build id; it reproduces only with the request recorded in `request_note` (a desktop Chrome
  User-Agent, no cookies, no Accept-Encoding). curl's default User-Agent and python-requests receive a 3,375-byte anti-robot
  page (HTTP 403). A redeploy of the site will change the bytes without changing the text, and no pre-cutoff capture exists.
  The preview image (161,461 bytes, immutable) is attached as a supplementary response; the full-resolution page image is
  served only through a time-limited signed URL and is not recorded (B2). Replacing this source with the Vedomosti 1991 No. 4
  facsimile is the first next work order.
- Internet Archive captures were requested without an Accept-Encoding header and none was served compressed; each identity is
  the uncompressed body. A replay that accepts gzip is a different body (the Law 1861-I capture came back as 14,596 gzip bytes).
- The RSFSR gazette page images carry the file's upload timestamp as their query (`?1716491978`); it is not a cache-buster, and
  the check's cache-busting request returned the same bytes. The library's document pages (`/ru/nodes/418646`, `418647`) change
  on every request and are recorded as locators only.
- The one portal card (resolution 525) is a single-document query restricted to one number and one day. The portal's date and
  organ lists used for discovery are not recorded.

## Leads not imported

- **Congress resolution 1367-I of 15 March 1990 electing the Chairman of the Supreme Soviet**, Garant
  (https://base.garant.ru/6334493/, commercial; its signatory "В. Воротников" unconfirmed), and resolution 1362-I on Garant
  (https://base.garant.ru/6334489/). The GARF original of 1362-I is used instead.
- **Rosarkhiv exhibit 09-32** (the Congress's resolution of 24 December 1990 on the referendum; caption "Подпись-автограф
  председателя Верховного Совета СССР А.И. Лукьянова"): caption only, its facsimile never archived (A1); replaced by 1870-I.
- **Law 2033-I "О Кабинете Министров СССР" (20 March 1991)**: https://ru.wikisource.org/wiki/Закон_СССР_от_20.03.1991_№_2033-I
  (crowd-sourced) and economics.kiev.ua; not on the portal or in the Rada database (404).
- **Supreme Soviet resolutions 1900-I (Pavlov's approval) and 1907-I (Ryzhkov's pension), 14-15 January 1991**:
  https://base.garant.ru/6336271/ and https://base.garant.ru/6336275/ (commercial; Vedomosti 1991 No. 4, arts. 80 and 84).
- **Non-official HTML transcriptions of the Vedomosti** on the same host as the scans: https://vedomosti.sssr.su/1991/52/ (the
  annual index citing Nos. 4, 39, 45, 48 and 49; Declaration 142-N and order 141-N), https://vedomosti.sssr.su/1990/52/ and
  https://vedomosti.sssr.su/1991/1/; the retyped bulletin of the Supreme Soviet members' meeting of 12 December 1991
  (https://sten.vs.sssr.su/13/1/5/) and the Council of Republics transcription (https://sten.sr.vs.sssr.su/13/1/23/).
- **Compilations by G. V. Belonuchkin** (non-official): https://aprel.org/d04up2.pdf (presidential acts of 1991; lacks
  УП-1307 to 1309), https://aprel.org/d08km1.pdf (Cabinet acts) and https://aprel.org/d09pp.pdf; and https://sssr.su/1991-12.pdf.
- **Izvestia news pages**: No. 13, page 2, and No. 14 (15-16 January 1991: the 279-75 vote, the pension debate, an editorial);
  Nos. 226, 258, 259, 295, 299 and 307 (Silayev's interview, the chamber chairmen); No. 200 (the Presidium of 21 August, TASS).
  Izvestia No. 201 of 23 August 1991 (https://izvestija.sssr.su/1991/201mv.pdf, 32,909,140 bytes, `64c01cdb…`) prints only a
  TASS report of the 22 August Presidium sitting, not УП-2443 or the removal resolution.
- **Legal-portal records read but not imported**: Supreme Soviet resolution 1906-I of 15 January 1991 (nd=102010350, a further
  signature of the Chairman), Cabinet resolution 274 of 22 May 1991 (nd=102011538, a further signature of the Premier), Cabinet
  orders 1р of 18 January 1991 (nd=102010393, signed by a deputy) and 940р of 20 August 1991 (nd=102012325, signed "Зам.
  Премьер-министра СССР В. Догужиев"), the Council of Ministers' order 2042р of 5 December 1990 and the Committee's other acts.
  Each would repeat an attestation or a signature pattern already recorded.
- **Optional check find, not imported**: a documentary-edition page (Музыка вместо сумбура, 2013) printing a minister's letter
  of 17 January 1990 to "т. Рыжкову Н.И.": a published edition and an addressee attestation only.
- **Encyclopaedias and history sites**: Wikipedia (Ryzhkov's term to 14 January 1991; Pavlov's arrest on 23 August 1991; the
  Committee; Alimzhanov's election on 29 October 1991), bigenc.ru and a cyberleninka article: leads only; no date is taken from them.
- **Paid or closed**: the naukaprava.ru scans of Vedomosti 1990 No. 12 and 1991 Nos. 4, 39, 45, 48 and 52; the Presidential
  Library's Council of Ministers volumes (reading room only).

## Sources attempted

- `https://pravo.gov.ru/…` on port 443: timed out; the same paths over HTTP worked. The portal holds no USSR act for 15 March
  1990 and no text of 1367-I, 1861-I, 1862-I, 2033-I, 1900-I, 1907-I or 2366-I.
- `projects.rusarchives.ru` and `guides.rusarchives.ru`: refused or timed out; Rosarkhiv exhibits were read from captures. The
  capture of leaf 46 of exhibit 09-42 timed out.
- `zakon.rada.gov.ua`: the identifiers for 1362-I, 1367-I, 1862-I, 2033-I and 2392-I return 404; the live pages embed
  per-request tokens.
- `vedomosti.sssr.su`: PDF scans exist only for 1991 Nos. 35-38 and 41; Nos. 4, 39, 40 and 42-52 return 404.
  `sten.vs.sssr.su`: no scans of the sittings of 27-31 August (12/6/3.pdf to 5.pdf) or October-December 1991;
  `sten.sr.vs.sssr.su`: no scans. `izvestija.sssr.su`: 1991 issues 187-208 (August 1991) only; its directory listing returns 403.
- naukaprava.ru: purchase or login required (not attempted). rusneb.ru: 403 ("Отключите VPN"), not bypassed. The Presidential
  Library's search and the UN Digital Library: bot challenges, not attempted. adilet.zan.kz: JavaScript only.
  history.state.gov (FRUS): no 1990 document naming Ryzhkov or Lukyanov. search.rsl.ru: script-rendered results.
- Yandex archive: full-resolution page images need the viewer's signed URL (HTTP 403 otherwise); not fetched outside the viewer.
- Internet Archive: the CDX index was intermittently offline or rate-limited (HTTP 429) during the dossiers and checks, and three
  of this packet's first-pass requests failed to connect; availability lookups and captures were retried.

## Checker defects

| # | Defect | Outcome |
|---|---|---|
| A1 | The 24 Dec 1990 claim for the Chairman of the Supreme Soviet rests only on Rosarkhiv's caption | **Applied**: exhibit 09-32 is a lead; the check's primary replacement, 1870-I of 27 Dec 1990 (Rada capture), is a role claim with the printed name "А.ЛУК'ЯНОВ" |
| A2 | SU-GOV-01 missed earlier attestations in the RSFSR government gazette | **Applied**: arts. 59 and 60 imported (four page images, viewed); holder 1 `attested_on` 1990-01-12; 24 Dec 1989 is a pre-period claim; the dossier's "nothing dated in January 1990" is withdrawn |
| A3 | Holder names inconsistent | **Applied**: canonical names, with `printed_name` on every holder row; no "И." is taken from a caption |
| A4 | "Creates the Cabinet" overstates Law 1861-I | **Applied**: claim renamed `su_rada_1861i_cabinet_chapter_19901226`; the claim, the observation and the institution notes say it renames and rewrites |
| A5 | "In the chair on 15 March" goes further than the facsimile | **Applied**: `attested_on` kept as the act's date; the claim, the holder note and this report say the date of signature is not stated |
| A6 | Proposed holders do not match the `ussr.json` shape; attaching the election row changes C01-05 pins | **Applied**: holders use `name`, `sources` and `claim_ids`; the election row is a `su_president` role claim with the holders unchanged; `su_president` and the C01-05 test are named in the integration notes |
| B1 | The Izvestia page reproduces only with a browser User-Agent | **Applied**: `request_note` records the exact request; the provenance note calls it a dynamic rendering and names its build id; its replacement is next work |
| B2 | The full-resolution image hash cannot be reproduced | **Applied**: it is in no response field; only the immutable preview is attached |
| B3 | Names not in the house form | **Applied**: "Валентин Сергеевич Павлов", "Николай Иванович Рыжков", "Анатолий Иванович Лукьянов"; Силаев only in `persons_named`, as printed |
| B4 | Art. 113(3) paraphrased too narrowly | **Applied**: quoted in full; the Premier's release rests on Art. 127-3(6) |
| B5 | The Силаев committee rows are filed under the head-of-government role with his name | **Applied**: every committee row is a claim on `su_government`, with no role and no holder name |
| B6 | The 28 Aug session-report claim bundles two events | **Applied**: split into the agreement (on the role) and the no-confidence report (on the institution); the date is attributed to the paragraph's "28 августа" |
| B7 | The 22 Aug decree is labelled "dismissal" | **Applied**: `release_decree_referred_to_legislature`; no `until` |
| B8 | The "no presidential decree appointed Pavlov" finding is too strong | **Applied**: reworded in the claim, the unresolved item and this report, naming the compilation's gap (УП-1307 to 1309) |
| B9 | No unresolved item on who, if anyone, acted as Premier after 22 Aug | **Applied**: an unresolved item on `su_government`; order 943р (a packet find) records the interim direction of 19 Aug as a claim only. The first deputy Premiers' styling of 28 Aug stays in the agreement claim's `persons_named`, not a separate claim, because they hold no role here |
| B10 | Resolutions 2354-I and 2360-I (agenda items on the Government and the Cabinet) left out of a recorded source | **Applied**: both imported as body-level claims on `su_government` |
| B11 | The Congress's telegram on Ryzhkov's illness left out of a recorded source | **Applied**: imported (`congress_message_on_illness`), never a boundary |
| B12 | `accessed_date` convention | **Applied**: 2026-09-26 (UTC) for all 31 sources, matching the download timestamps; stated in the header |
| B13 | Rows lack `role_title` and `observation_id`; the stenogram prints no title for Ryzhkov | **Applied**: every row has both; the four Ryzhkov stenogram rows carry `role_title_note` "The passage prints no office title for him." |
| C1 | Part C duplicates part B (the same PDFs, restated claims) | **Applied**: one source per issue, recorded as the pre-cutoff capture with the live file attached; C's four duplicate claims dropped for B's |
| C2 | `acting_service_designation` overstates 2367-I | **Applied**: `committee_created`, on the institution, with no holder |
| C3 | The 2361-I claim collapses the 22 Aug removal and the 26 Aug approval | **Applied**: `su_ss_res_2361i_approves_presidium_removal_19910826` added; the reference claim is restricted to 22 Aug |
| C4 | `resignation_request_reference` overreads the "analogous request" | **Applied**: `request_reference`; the uncertainty says the text does not say it concerned a resignation |
| C5 | The 28 Aug styling claim: locator and a bundled 29 Aug sentence | **Applied**: locator printed pp. 1468-1469 (PDF pp. 38-39); the 29 Aug consent sentence dropped (2368-I carries it) |
| C6 | The 21-22 Aug report bundles the 26 Aug suspension | **Applied**: the 26 Aug sentence dropped (2361-I carries it); the claim keeps its two-day period |
| C7 | Law 2392-I Art. 4 locator | **Applied**: printed pp. 1487-1488 (PDF pp. 17-18) |
| C8 | Resolution 7 of 28 Nov leaves out its appointee | **Applied**: "т. Родникову О. В." and "т. Кулика Г. В." quoted as printed |
| C9 | The Chairman's name differs across parts | **Applied**: "Анатолий Иванович Лукьянов" (printed in full in 2368-I and 2389-I), with `printed_name` on every row; holders use `name` in `ussr.json` and rows `holder_name` |
| C10 | Acts about the bodies sit on the role claim lists | **Applied**: body-level acts are on `su_government.claim_ids` or `su_supreme_soviet.claim_ids`; the role lists hold only office acts |
| C11 | `accessed_date` is the local date | **Applied**: 2026-09-26 (UTC) |
| C12 | A ruling is needed on the sssr.su facsimiles | **Applied in part**: the ruling is requested (Sources added, next work); the scans are used, with the host in every publisher and the dependent observations named; the C01-05 host guard is re-expressed to allow only scanned issue PDFs |

Numbering follows each check's `defects` list in order: A1-A6, B1-B13 and C1-C12.

Missing primary records the checks found:

- Check A: the RSFSR government gazette pages (arts. 59 and 60) and Congress resolution 1870-I are imported; the optional
  documentary-edition page is a lead.
- Check B: part C's records in Vedomosti Nos. 35 and 36 are imported; Vedomosti 1991 No. 4 (paywall) and Law 2033-I (no
  official copy) remain unavailable; Izvestia No. 201 was checked and does not print УП-2443, so it is a lead.
- Check C: the Fifth Congress bulletin No. 5 (the vote) and the Supreme Soviet's bulletin No. 2 of 26 August are imported, and
  the acts of the Fifth Congress are attached to the bulletin as a corroborating response; Vedomosti 1991 Nos. 39, 45, 48 and
  52 and the Presidium's resolution of 22 August remain unavailable.

## Suggested next work orders

These are proposals for the integrator. They are not created in `work-orders.json`.

- `C01-USSR-GOV-001`: an official facsimile of Vedomosti 1991 No. 4 (arts. 80 and 84, resolutions 1900-I and 1907-I), to replace
  the Izvestia rendering and to test whether 1907-I states when the 1990 Chairman's office ended.
- `C01-USSR-GOV-002`: an integrator ruling on ends. Is a release decree referred to the legislature (УП-2443), the legislature's
  agreement (2366-I) or the Congress's release (2389-I) a stated end? Apply one rule to every holder, here and in CLAUDE-C01-19.
- `C01-USSR-GOV-003`: an integrator ruling on the SSSR.SU and Russian Historical Society facsimiles.
- `C01-USSR-GOV-004`: the Congress's resolution 1367-I and vote of 15 March 1990 (Vedomosti 1990 No. 12, or GARF F. R-9654, Op.
  1, D. 121); Law 2033-I and its enactment resolution (Vedomosti 1991 No. 14, arts. 400-401).
- `C01-USSR-GOV-005`: the chamber chairmen's elections (Vedomosti 1991 No. 45) and a decision on chamber roles; Declaration
  142-N and order 141-N (No. 52; SURU-TR91-08).
- `C01-USSR-GOV-006`: Силаев's appointments (Vedomosti 1991 Nos. 39 and 48) and a modelling decision on the interim committees.

## Integration notes (outside this packet's file boundary)

- **Stacking.** This branch is stacked on CLAUDE-C01-19 (`claude/c01-ru-19` at `6f4ef2c2`), itself stacked on CLAUDE-C01-14 and
  C01-05. `claude/c01-ru-19` was fetched again before this work and had no commits missing from this branch. Both packets
  change the USSR and Russia tests, so CLAUDE-C01-19 must be integrated first.
- **Existing records changed.** No existing extract is edited. In `ussr.json`, `su_president` gains one role claim and one
  source (`su_garf_1362i_gorbachev_elected_president_19900315`, `su_garf_exhibit_res_1362i_19900315`), its holders unchanged,
  and `su_presidency` one coverage item; `su_supreme_soviet` gains sources, body-level claims and four coverage items; its role
  `su_supreme_soviet_chair` gains sources, claims, a `scope_note` and two holders after the unchanged CLAUDE-C01-05 holder; the
  top-level coverage gains one item. All other existing content is unchanged.
- **A test outside the handoff's list.** `test_russia_heads_of_government_c01_19.py` pinned `ru_government_chairman` as the
  only head-of-government role across both packets. That guard is re-expressed exactly as `[ROLE, 'su_government_head']`, the
  only change to the file, because this packet's handoff requires a `head_of_government` role in `ussr.json`.
- **The C01-05 host guard.** `test_ussr_russia_transition_c01_05.py` excluded the host vedomosti.sssr.su outright to keep its
  non-official HTML transcription out. It now allows only scanned issue PDFs on that host (`/1991/<issue>.pdf`) and still
  forbids the transcription (`/1991/52`); the other excluded hosts are unchanged.
- Existing tests updated, none loosened:
  - `test_ussr_research_s10h.py`: totals (4, 10, 21, 5) → (5, 41, 98, 6); institutions 3 → 4; the extract comparison keeps the
    per-claim check for the first ten extracts and pins the 31 table extracts to their rows; the PDF-page set adds the eight
    scanned PDFs; access dates {13, 21, 26 September}, with the 26 September set pinned to the 31 new sources; index
    institution observations 3 → 4, source claims 21 → 98, `mapping_pending` 4 → 5.
  - `test_ussr_russia_transition_c01_05.py`: the USSR institution set adds `su_government`; the host guard as above.
- `research-index.json` is regenerated in a **separate commit**; it is the only file this packet shares with other pending
  packets (apart from those shared with CLAUDE-C01-19, C01-14 and C01-05 through the stack). New totals: 367 sources and 2,123
  claims (previously 336 and 2,046); 30 institution observations (29); 93 discovery batches (unchanged). USSR's entries go from
  4 to 5, `mapping_pending` from 4 to 5, role observations from 5 to 6 and source claims from 21 to 98; its one batch now has
  five members. If another packet lands first, regenerate the index rather than merging it.
- `research/README.md`, the C01 README totals and `docs/planning/ai-workstreams.json` are left for the integrator; this handoff
  is self-proposed and not registered there.
- The new test `test_ussr_government_supreme_soviet_c01_26.py` pins the institution, the role, the six new holders, every
  claim's date, kind, observation, role and institution, every response identity and attached response, the extracts, the
  separation from the presidency and from `russia.json` (a digest of every Russia holder), and 41 mutations.
- In the atlas, the USSR gains "Совет Министров СССР / Кабинет Министров СССР — Council of Ministers of the USSR (from 1991 the
  Cabinet of Ministers of the USSR)" with one office and four observations, and the Supreme Soviet's chair shows two more
  observations. No UI code changed; the Node check passes and no browser review was run.
- The sparse worktree lacks `spheres-sim/data`, so `test_campaign_census.py` errors in setup; the worktree was not widened.

## Checks

```text
python -X utf8 tools/avatars/campaign_research.py
python -X utf8 tools/avatars/campaign_research.py --check
python -X utf8 -m unittest discover -s tools/avatars -p "test_ussr*.py"
python -X utf8 -m unittest discover -s tools/avatars -p "test_russia*.py"
python -X utf8 -m unittest discover -s tools/avatars -p "test_*research*.py"
python -X utf8 -m unittest discover -s tools/avatars -p "test_campaign*.py"
node --test tools/ui/check_leadership_research_review.cjs
python tools/planning/workboard.py --check
git diff --check (this packet's paths)
```

All passed on 26 September 2026 (UTC): the exact index regeneration and check (367 sources, 2,123 claims, 30 institution
observations, 93 discovery batches); 27 USSR tests (9 of them new) and 29 Russia tests; 79 research tests; 9 campaign research
tests, with `test_campaign_census.py` erroring in setup because the sparse worktree has no `spheres-sim/data` (expected; not
widened); 11 atlas Node tests; the workboard check (44 markers); `git diff --check` on this packet's paths, including the new
files once staged. The new test's 41 mutations each fail as intended: a successor's approval or signature, an illness report,
the release decree, the agreement with it, the suspension, the consent to arrest and the Congress's release used as ends; the
approval and election days used as starts; a nomination day used as the attested day; interim direction, a deputy's
signature, the interim committee and a chamber chairman added as holders; intermediate and pre-period signatures promoted to
holders; cross-role and cross-institution holders and claims; a body-level claim moved onto the role; the presidency and the
existing chair holder changed; any until; a second Union head-of-government role; the role or the institution removed; a
lifecycle end; a successor mapping; a holder dated by another day's claim; events out of order; a changed Russia holder or a
Russia role citing this packet; checksum mismatches, beyond-cutoff dates, a reversed interval, a claim from an uncited source
and an HTTP source URL.
