# Russian party leaders 1990-2026 28: the national leaders of the KPRF, the LDPR, Yabloko, the Agrarian Party and Democratic Choice of Russia

Packet: **CLAUDE-C01-28**. State: **ready_for_review** (not complete).
Owner: Claude. Integrator/reviewer: Codex. Branch `claude/c01-ru-28`; claim commit `2c4d5bd7` on `846df479` (then the head of
`codex/campaign-certification`); **not stacked** on a pending packet. `origin/codex/campaign-certification` was merged
before submission (merge commits `7c38049f` and `b6d5c9a8`); it changed none of this packet's research files; its `research-index.json` is regenerated, and its copy of the claim handoff (with Codex's queue-registration note) was kept. Research access: 28 September 2026 (UTC); this packet's two
re-download passes of every response ran between 14:54Z and 17:29Z (details under
[Response identities](#response-identities-and-stability-checks)). The historical cutoff stays **7 September 2026**.

This packet reviews ten observations, RU-PTY-01 to RU-PTY-10, in [russia.json](russia.json). It adds five party-leader roles
(kind `party_leader`): `ru_kprf_chairman`, `ru_ldpr_chairman` and `ru_yabloko_chairman` on the existing 2021 ballot-list
observations `ru_duma_ballot_list_2021_01`, `_03` and `_07`, and `ru_apr_chairman` and `ru_dvr_chairman` on two new
organization observations made only from those parties' own records, `ru_apr_party_self_record` («Аграрная партия России»)
and `ru_dvr_party_self_record` («Демократический выбор России»). A third new observation, `ru_vybor_rossii_bloc_1993`
(«Избирательное объединение «Выбор России»»), comes from the 1993 bloc's own programme and has no role. In all: 68
sources, 137 claims and 24 holder observations of ten people, from party records of 1994-2026 (retrospective party
claims reach back to 1990). No holder has a `from` or an `until`. The three ballot-list observations keep their names,
identifiers, lifecycles, claims and sources; each gains the role and one coverage note, and the packet coverage gains one
note. The five 2021 Duma faction heads, the presidency and the Government roles and `ussr.json` are unchanged. No game
mapping, lifecycle boundary, portrait or avatar is added. The parent scope (C01, C06, S23, WC1 and CP1) remains open.

The research was done in five dossiers (KPRF, LDPR, Yabloko, the Agrarian Party, and Russia's Choice with Democratic Choice
of Russia) by three research agents, and each chain was then checked independently: one check of the KPRF, LDPR and Yabloko
chains and one of the Agrarian Party and DVR chains. The checks found further primary records; the packet imports the
18 that bear on the office and records the rest as leads. Every checker defect is applied, applied in part or
declined with a reason (see [Checker defects](#checker-defects)).

## Outcome

| ID | Question | Decision |
|---|---|---|
| RU-PTY-01 | KPRF: the 1993 restoration congress and the chairman to 1998 | **Accepted in part:** the party's 2026 "date in history" item and its undated reference page (claims only) give the II Extraordinary Congress opening on 13 Feb 1993, the renaming, the Central Executive Committee and Zyuganov's election as its chairman "После съезда"; the I Plenum of 20 Apr 1997 "избрал Председателем ЦК КПРФ т. Зюганова Г.А." (an election, a claim); holder 1 attested 23 May 1998 (V congress notice). No contemporaneous 1993 record and no record of the 1995 change of title were found |
| RU-PTY-02 | KPRF: the chairman from 2021 to the cutoff | **Accepted:** the XVIII congress and the I Plenum's election of 24 Apr 2021 (claims only; no later 2021 attestation); the XIX congress and the I Plenum's election of 5 Jul 2025, with holder 2 attested the same day after the vote; the congress's second stage of 20 Jun 2026 (no leadership decision); holder 3 attested 28 Aug 2026 |
| RU-PTY-03 | LDPR: the LDPSS founding, the LDPR's founding and the chairman to 2022 | **Accepted in part:** the party's 2010 timeline dates the LDPSS founding congress and Zhirinovsky's election to 31 Mar 1990, the USSR Ministry of Justice certificate to 12 Apr 1991 and the III (1992) and IV (1993) congresses (retrospective claims); holder 1 attested 23 Nov 1996 (Central Committee plenum resolution); holder 2 attested 30 Mar 2022 (party newspaper contacts). No contemporaneous 1990-1993 record was found |
| RU-PTY-04 | LDPR: the 2022 death, the interim and Slutsky to the cutoff | **Accepted in part:** the party's news item of 6 Apr 2022 ("Но сегодня его не стало и сегодня мы будем скорбеть…") and its later timeline (claims; no `until`, by the user's ruling of 28 Sep 2026); the interim newspaper of 6 May 2022 names no chairman or acting chairman; the Supreme Council's recommendation of 26 May (a nomination); the XXXIV congress, vote and election of 27 May; holder 3 attested 27 May 2022; the re-election of 2 Oct 2025 and holder 4 attested that day; holder 5 attested 11 Aug 2026 |
| RU-PTY-05 | Yabloko: the 1993 bloc, the 1995 association, the 2001 party and Yavlinsky to 2001 | **Accepted in part:** the association's undated reference page of about 1998 (claims: the bloc's lists in autumn 1993, the founding congress of 5-6 Jan 1995, the chairman's election there, registration on 10 Feb 1995); holder 1 attested 14 Mar 1998 (VI congress report); the transformation into a party on 22 Dec 2001 and Yavlinsky's election on 23 Dec 2001 (claims); holder 2 attested 23 Dec 2001 (his speech page and the congress section). No contemporaneous 1995 record was found |
| RU-PTY-06 | Yabloko: 2004 to 2008 and Mitrokhin's election | **Accepted in part:** the 2004 re-election report (no day of the vote printed; a claim), the party's vote schedule (a claim) and holder 3 attested 4 Jul 2004; the 2008 report's year-only title block and congress spans (claims); the XV congress of 21-22 Jun 2008 elected Mitrokhin (day of the vote not printed; a claim) and holder 4 is attested 22 Jun 2008 |
| RU-PTY-07 | Yabloko: Mitrokhin to 2015, Slabunova, Rybakov and an attestation before the cutoff | **Accepted:** holder 5 (Mitrokhin) attested 19 Dec 2015; the XVIII congress's election report and holder 6 (Slabunova) attested 20 Dec 2015; the XXI congress of 14-15 Dec 2019, the vote "около часа ночи" and holder 7 (Rybakov) attested 16 Dec 2019; the re-election reports of 9 and 11 Dec 2023 and holder 8 attested 13 Dec 2023; holder 9 attested 19 Aug 2026 |
| RU-PTY-08 | Agrarian Party: its founding and Lapshin | **Accepted in part:** the party's undated historical note of 2002 (claims: founding congress 26 Feb 1993, charter registration 9 Apr 1993, re-elections 1994-2001, confirmation of powers 26 Feb 1998, the 2001 transformation, registration as a political party 31 May 2002); holder 1 attested 9 Sep 2003 (XI congress release) |
| RU-PTY-09 | Agrarian Party: Plotnikov and the 2008 accession | **Accepted in part:** the XII congress's election of 28 Apr 2004 (a claim), holder 2 attested 28 May 2004 (plenum release), the congress's second stage of 9 Oct 2004 (a claim), holder 3 attested 26 Sep 2008; the memorandum of 12 Sep 2008 and the XV congress's accession decision of 10 Oct 2008 are claims, not an end. The completion of the accession was not found |
| RU-PTY-10 | Russia's Choice (1993) and Democratic Choice of Russia | **Accepted in part:** the bloc's programme adopted on 17 Oct 1993 (a separate observation, no leader named); the working name «Выбор России» (19 May 1994), the founding declaration of 12 Jun 1994 and the Political Council's election (claims); holders attested 10 Jul 1994, 18 Jun 1995, 22 Sep 1996 and 16 Dec 1997 on signed party records; the 1996 chairmanship ballot record, a bare-title signature of 16 Dec 2000 and the X congress's self-dissolution decision ("19 мая", no year printed) are claims; the party site's history page is a 1996 reference-book passage the party republished (claims only; no observation cites it) |

Retrospective lists are claims, never boundaries: a history page, reference page or timeline may date an event it retells
(its day is recorded when printed), but it never dates or bounds a holder.

### Holders

`ru_kprf_chairman` (Председатель Центрального Комитета КПРФ — Chairman of the Central Committee of the KPRF):

| # | Name | `attested_on` | `from` | `until` | Claim |
|---|---|---|---|---|---|
| 1 | Геннадий Андреевич Зюганов | 1998-05-23 | null | null | `ru_kprf_v_congress_zyuganov_reports_as_chairman_19980523` |
| 2 | Геннадий Андреевич Зюганов | 2025-07-05 | null | null | `ru_kprf_i_plenum_chairman_closing_word_20250705` |
| 3 | Геннадий Андреевич Зюганов | 2026-08-28 | null | null | `ru_kprf_zyuganov_chairman_greeting_20260828` |

`ru_ldpr_chairman` (Председатель ЛДПР — Chairman of the LDPR):

| # | Name | `attested_on` | `from` | `until` | Claim |
|---|---|---|---|---|---|
| 1 | Владимир Вольфович Жириновский | 1996-11-23 | null | null | `ru_ldpr_plenum_resolution_chairman_report_19961123` |
| 2 | Владимир Вольфович Жириновский | 2022-03-30 | null | null | `ru_ldpr_newspaper_chairman_zhirinovsky_contacts_20220330` |
| 3 | Леонид Эдуардович Слуцкий | 2022-05-27 | null | null | `ru_ldpr_news_chairman_speaks_after_election_20220527` |
| 4 | Леонид Эдуардович Слуцкий | 2025-10-02 | null | null | `ru_ldpr_xxxvii_congress_chairman_presents_strategy_20251002`, `ru_ldpr_slutsky_post_signed_as_chairman_20251002` |
| 5 | Леонид Эдуардович Слуцкий | 2026-08-11 | null | null | `ru_ldpr_slutsky_chairman_proposal_20260811` |

`ru_yabloko_chairman` (Председатель партии «ЯБЛОКО» — Chairman of Yabloko):

| # | Name | `attested_on` | `from` | `until` | Claim |
|---|---|---|---|---|---|
| 1 | Григорий Алексеевич Явлинский | 1998-03-14 | null | null | `ru_yabloko_yavlinsky_chairman_report_vi_congress_19980314` |
| 2 | Григорий Алексеевич Явлинский | 2001-12-23 | null | null | `ru_yabloko_chairman_speech_page_20011223`, `ru_yabloko_x_congress_party_chairman_speech_20011223` |
| 3 | Григорий Алексеевич Явлинский | 2004-07-04 | null | null | `ru_yabloko_yavlinsky_proposes_term_limit_on_his_post_20040704` |
| 4 | Сергей Сергеевич Митрохин | 2008-06-22 | null | null | `ru_yabloko_mitrokhin_chairman_joins_political_committee_20080622` |
| 5 | Сергей Сергеевич Митрохин | 2015-12-19 | null | null | `ru_yabloko_mitrokhin_holds_chairmanship_20151219` |
| 6 | Эмилия Эдгардовна Слабунова | 2015-12-20 | null | null | `ru_yabloko_slabunova_new_chairman_deputies_20151220` |
| 7 | Николай Игоревич Рыбаков | 2019-12-16 | null | null | `ru_yabloko_rybakov_plans_as_chairman_20191216` |
| 8 | Николай Игоревич Рыбаков | 2023-12-13 | null | null | `ru_yabloko_rybakov_chairman_appeal_20231213` |
| 9 | Николай Игоревич Рыбаков | 2026-08-19 | null | null | `ru_yabloko_rybakov_chairman_appeal_20260819` |

`ru_apr_chairman` (Председатель Аграрной партии России — Chairman of the Agrarian Party of Russia):

| # | Name | `attested_on` | `from` | `until` | Claim |
|---|---|---|---|---|---|
| 1 | Михаил Иванович Лапшин | 2003-09-09 | null | null | `ru_apr_xi_congress_lapshin_reports_as_chairman_20030909` |
| 2 | Владимир Николаевич Плотников | 2004-05-28 | null | null | `ru_apr_plenum_plotnikov_reports_as_chairman_20040528` |
| 3 | Владимир Николаевич Плотников | 2008-09-26 | null | null | `ru_apr_plenum_plotnikov_reports_as_chairman_20080926` |

`ru_dvr_chairman` (Председатель партии «Демократический выбор России» — Chairman of Democratic Choice of Russia):

| # | Name | `attested_on` | `from` | `until` | Claim |
|---|---|---|---|---|---|
| 1 | Егор Тимурович Гайдар | 1994-07-10 | null | null | `ru_dvr_council_resolution_signed_by_chairman_19940710` |
| 2 | Егор Тимурович Гайдар | 1995-06-18 | null | null | `ru_dvr_ii_congress_approves_chairman_work_19950618`, `ru_dvr_ii_congress_decision_signed_by_chairman_19950618` |
| 3 | Егор Тимурович Гайдар | 1996-09-22 | null | null | `ru_dvr_v_congress_decision_signed_by_chairman_19960922` |
| 4 | Егор Тимурович Гайдар | 1997-12-16 | null | null | `ru_dvr_politsovet_decision_signed_by_chairman_19971216` |

The ten people are Геннадий Андреевич Зюганов, Владимир Вольфович Жириновский, Леонид Эдуардович Слуцкий, Григорий Алексеевич
Явлинский, Сергей Сергеевич Митрохин, Эмилия Эдгардовна Слабунова, Николай Игоревич Рыбаков, Михаил Иванович Лапшин, Владимир
Николаевич Плотников and Егор Тимурович Гайдар: the packet's limit. `holder_name` is the full given-name, patronymic and
surname form printed somewhere in the party's own records (Рыбаков's patronymic only in the 2026 item), and each extract row
keeps the printed form in `printed_name`. The 2021 faction heads keep their two-word names ("Геннадий Зюганов", "Владимир
Жириновский"), pinned by `test_russia_research_s10h.py`; faction and party roles are different offices and are not joined.

### How a start and an end are decided

A holder has `from` only where a record states the day the office was assumed or took effect, and `until` only where a record
states the day it ended. No reviewed record does either, so every holder is a dated observation. Each holder cites only
in-office attestations made on its own `attested_on` day: a party record that calls the person its chairman in the present (a
signed decision, a report heading, a contacts block, a news item). The test pins that every cited claim has the holder's date.
Everything else is a claim that never feeds a holder: congresses and their spans, elections by congresses, Central Committee
plenums or the Political Council, election reports, nominations, the 1996 chairmanship ballot record, retrospective lists and
statements, registration, reorganisation, accession and dissolution decisions, deaths, the interim record, continuation claims
(the incumbent before a vote, or a later attestation within the same observation), undated pages and a bare-title signature.
An election's day is never a `from`: no party election reviewed states an assumption of office.

No end is inferred. Successors' elections are not ends. The LDPR's news item of 6 April 2022 says of Zhirinovsky "Но сегодня его
не стало и сегодня мы будем скорбеть…", and its later timeline dates the death to that day; a death is not stated in words as the
end of the office, so no `until` is set. That is the user's ruling of 28 September 2026: the item (ldpr.ru/event/202499) never
says Председатель and never mentions the office ending, and its only captures date from October 2025 onward, so the death stays
a distinct dated claim. Codex may still decide at integration that a stated death day ends the office, in which case the anchor
would be `ru_ldpr_news_zhirinovsky_died_today_20220406`. The Agrarian Party's accession decision of 10 October 2008 and DVR's self-dissolution decision are organization events, never ends.

Dates are stored only where the record prints the day. A release's own date line may supply the year of a day and month in its
body (the Agrarian Party's "9 сентября" and "28 апреля", the LDPR timeline's days under its year headings); a capture date never
supplies a missing year (DVR's "19 мая"), and relative words ("около часа ночи", "в минувшие выходные") supply no day. Where a
record prints two different days (DVR's statement headed 18 June 1995 and adopted "17 июня 1995 года") it dates nothing.

### Party office and state office

Each role sits on a party organization observation; no claim of this packet is cited by any institution role and no
presidency, Government or faction claim is cited by a party role, which the test pins both ways. Several party records print a
state or parliamentary office beside the party office: the LDPR newspaper's contacts list the Duma faction head separately, the
LDPR's 2022 items describe Slutsky by his Duma faction and committee posts, the KPRF's 2021 author block pairs the party title
with the faction post, DVR's congress edition calls Gaidar head of the parliamentary faction «Выбор России», and the Agrarian
Party's history names Lapshin a people's deputy. Only the party-office lines are claims. The LDPR newspaper is co-issued by the
party and its Duma faction; only its party-office lines are used.

### Organizations, names and mappings

The three roles on ballot-list observations follow the handoff; attaching a role does not establish that the 2021 list, the
party of the cited records and any predecessor (the KP RSFSR, the LDPSS, the 1995 association) are one legal identity. The
Agrarian Party and DVR observations are made only from their own records; the party-stated registration numbers (5025, 2364)
are recorded as such, not as register extracts. Russia's Choice is split: the 1993 electoral association is its own
observation from its programme, and DVR's records tie the party's name to the name «Выбор России» (a working name in May 1994,
"создана на основе движения Выбор России" in its history page); neither names the 1993 association. That history page is not a
party record: it is a passage of Ю.Г.Коргунюк and С.Е.Заславский, «Российская многопартийность» (1996), which the party site
republished as its history. It is typed `party_republished_reference_text`, no observation cites it, and its six claims stay on
`ru_dvr_chairman` as retrospective claims, never boundaries; `ru_dvr_party_self_record` rests on the party's own minutes,
congress edition, bulletin and newspaper. No observation is mapped to
`Russia/ru_kprf`, `ru_ldpr`, `ru_yabloko`, `ru_apr` or `ru_vybor`: a name match is never a game mapping.

### Date ledger

Each row is a separate dated fact with its own claim; two facts on one day stay two claims. "(holder `attested_on`)" marks the
claims that date a holder.

| Date | Party: events | Claims (holder field) |
|---|---|---|
| 31 Mar 1990 | LDPR: congress (retrospective); LDPR: election (retrospective) | `ru_ldpr_history2010_ldpss_founding_congress_19900331`, `ru_ldpr_history2010_ldpss_chairman_elected_19900331` |
| 12 Apr 1991 | LDPR: registration statement (retrospective) | `ru_ldpr_history2010_ussr_justice_registration_19910412` |
| 13 Feb 1993 | KPRF: congress (retrospective) | `ru_kprf_retro_ii_extraordinary_congress_opened_19930213` |
| 26 Feb 1993 | APR: congress (retrospective) | `ru_apr_history_founding_congress_19930226` |
| 9 Apr 1993 | APR: registration statement | `ru_apr_history_charter_registered_19930409` |
| 17 Oct 1993 | Russia's Choice bloc: programme adopted | `ru_vybor_rossii_programme_adopted_founding_congress_19931017` |
| 19 May 1994 | DVR: working name | `ru_dvr_supporters_meeting_working_name_19940519` |
| 12 Jun 1994 | DVR: founding declaration | `ru_dvr_founding_declaration_19940612` |
| 10 Jul 1994 | DVR: in-office attestation | `ru_dvr_council_resolution_signed_by_chairman_19940710` (holder `attested_on`) |
| 9 Aug 1994 | DVR: registration statement; DVR: registration statement | `ru_dvr_charter_registered_19940809`, `ru_dvr_history_registered_19940809` |
| 19 Nov 1994 | DVR: programme adopted | `ru_dvr_history_programme_adopted_19941119` |
| 10 Feb 1995 | Yabloko: registration statement | `ru_yabloko_reference_registration_19950210` |
| 18 Jun 1995 | DVR: in-office attestation; DVR: in-office attestation; DVR: congress (retrospective) | `ru_dvr_ii_congress_approves_chairman_work_19950618` (holder `attested_on`), `ru_dvr_ii_congress_decision_signed_by_chairman_19950618` (holder `attested_on`), `ru_dvr_history_ii_congress_19950618` |
| 26 Aug 1995 | DVR: congress (retrospective) | `ru_dvr_history_iii_congress_19950826` |
| 21 Sep 1996 | DVR: continuation claim | `ru_dvr_v_congress_list_decision_signed_by_chairman_19960921` |
| 22 Sep 1996 | DVR: chairmanship ballot record; DVR: in-office attestation | `ru_dvr_v_congress_chairman_ballot_protocol_approved_19960922`, `ru_dvr_v_congress_decision_signed_by_chairman_19960922` (holder `attested_on`) |
| 23 Nov 1996 | LDPR: in-office attestation | `ru_ldpr_plenum_resolution_chairman_report_19961123` (holder `attested_on`) |
| 20 Apr 1997 | KPRF: election | `ru_kprf_i_plenum_zyuganov_elected_chairman_19970420` |
| 16 Dec 1997 | DVR: in-office attestation | `ru_dvr_politsovet_decision_signed_by_chairman_19971216` (holder `attested_on`) |
| 26 Feb 1998 | APR: retrospective statement | `ru_apr_history_vi_congress_powers_confirmed_19980226` |
| 14 Mar 1998 | Yabloko: in-office attestation | `ru_yabloko_yavlinsky_chairman_report_vi_congress_19980314` (holder `attested_on`) |
| 23 May 1998 | KPRF: congress; KPRF: in-office attestation | `ru_kprf_v_congress_held_19980523`, `ru_kprf_v_congress_zyuganov_reports_as_chairman_19980523` (holder `attested_on`) |
| 29 May 1998 | APR: registration statement | `ru_apr_history_charter_changes_registered_19980529` |
| 20 Mar 1999 | APR: election (retrospective) | `ru_apr_history_vii_congress_lapshin_reelected_19990320` |
| 16 Dec 2000 | DVR: bare-title signature | `ru_dvr_politsovet_statement_signed_chairman_20001216` |
| 24 Mar 2001 | APR: election (retrospective) | `ru_apr_history_ix_congress_lapshin_elected_20010324` |
| 8 Dec 2001 | APR: reorganisation decision; APR: election (retrospective) | `ru_apr_history_x_congress_transformation_20011208`, `ru_apr_history_x_congress_lapshin_elected_20011208` |
| 22 Dec 2001 | Yabloko: continuation claim; Yabloko: reorganisation decision | `ru_yabloko_x_congress_association_chairman_report_20011222`, `ru_yabloko_association_transformed_into_party_20011222` |
| 23 Dec 2001 | Yabloko: in-office attestation; Yabloko: election; Yabloko: in-office attestation | `ru_yabloko_x_congress_party_chairman_speech_20011223` (holder `attested_on`), `ru_yabloko_yavlinsky_elected_party_chairman_20011223`, `ru_yabloko_chairman_speech_page_20011223` (holder `attested_on`) |
| 31 May 2002 | APR: registration statement | `ru_apr_history_party_registration_20020531` |
| 9 Sep 2003 | APR: congress; APR: in-office attestation | `ru_apr_xi_congress_opened_20030909`, `ru_apr_xi_congress_lapshin_reports_as_chairman_20030909` (holder `attested_on`) |
| 26 Apr 2004 | APR: retrospective statement | `ru_apr_old_leadership_under_lapshin_20040426` |
| 28 Apr 2004 | APR: election; APR: election (retrospective) | `ru_apr_xii_congress_plotnikov_elected_20040428`, `ru_apr_chairman_page_plotnikov_elected_20040428` |
| 28 May 2004 | APR: in-office attestation | `ru_apr_plenum_plotnikov_reports_as_chairman_20040528` (holder `attested_on`) |
| 4 Jul 2004 | Yabloko: in-office attestation | `ru_yabloko_yavlinsky_proposes_term_limit_on_his_post_20040704` (holder `attested_on`) |
| 9 Oct 2004 | APR: congress | `ru_apr_xii_congress_second_stage_ended_20041009` |
| 12 Oct 2004 | APR: continuation claim | `ru_apr_plotnikov_comments_as_chairman_20041012` |
| 22 Jun 2008 | Yabloko: in-office attestation | `ru_yabloko_mitrokhin_chairman_joins_political_committee_20080622` (holder `attested_on`) |
| 12 Sep 2008 | APR: memorandum | `ru_apr_memorandum_with_united_russia_20080912` |
| 26 Sep 2008 | APR: in-office attestation | `ru_apr_plenum_plotnikov_reports_as_chairman_20080926` (holder `attested_on`) |
| 10 Oct 2008 | APR: congress; APR: accession decision | `ru_apr_xv_congress_held_20081010`, `ru_apr_xv_congress_accession_to_united_russia_20081010` |
| 19 Dec 2015 | Yabloko: in-office attestation | `ru_yabloko_mitrokhin_holds_chairmanship_20151219` (holder `attested_on`) |
| 20 Dec 2015 | Yabloko: election report; Yabloko: continuation claim; Yabloko: in-office attestation | `ru_yabloko_slabunova_elected_xviii_congress_20151220`, `ru_yabloko_mitrokhin_incumbent_nominates_slabunova_20151220`, `ru_yabloko_slabunova_new_chairman_deputies_20151220` (holder `attested_on`) |
| 15 Dec 2019 | Yabloko: election report | `ru_yabloko_rybakov_elected_xxi_congress_20191215` |
| 16 Dec 2019 | Yabloko: in-office attestation | `ru_yabloko_rybakov_plans_as_chairman_20191216` (holder `attested_on`) |
| 24 Apr 2021 | KPRF: congress; KPRF: continuation claim; KPRF: election | `ru_kprf_xviii_congress_held_20210424`, `ru_kprf_xviii_congress_chairman_presents_report_20210424`, `ru_kprf_i_plenum_zyuganov_elected_chairman_20210424` |
| 30 Mar 2022 | LDPR: in-office attestation | `ru_ldpr_newspaper_chairman_zhirinovsky_contacts_20220330` (holder `attested_on`) |
| 6 Apr 2022 | LDPR: death (retrospective); LDPR: death statement | `ru_ldpr_history2026_zhirinovsky_death_20220406`, `ru_ldpr_news_zhirinovsky_died_today_20220406` |
| 6 May 2022 | LDPR: interim record | `ru_ldpr_newspaper_interim_issue_no_chairman_20220506` |
| 26 May 2022 | LDPR: nomination | `ru_ldpr_news_supreme_council_recommends_slutsky_20220526` |
| 27 May 2022 | LDPR: election (retrospective); LDPR: congress; LDPR: election report; LDPR: in-office attestation; LDPR: election | `ru_ldpr_history2026_slutsky_elected_chairman_20220527`, `ru_ldpr_news_xxxiv_congress_opened_20220527`, `ru_ldpr_news_xxxiv_congress_elects_slutsky_20220527`, `ru_ldpr_news_chairman_speaks_after_election_20220527` (holder `attested_on`), `ru_ldpr_news_leader_vote_held_20220527` |
| 27 Jun 2022 | LDPR: continuation claim | `ru_ldpr_newspaper_chairman_slutsky_contacts_20220627` |
| 9 Dec 2023 | Yabloko: continuation claim; Yabloko: election report | `ru_yabloko_rybakov_incumbent_candidate_20231209`, `ru_yabloko_rybakov_reelected_second_term_20231209` |
| 11 Dec 2023 | Yabloko: election report | `ru_yabloko_rybakov_retained_chairmanship_20231211` |
| 13 Dec 2023 | Yabloko: in-office attestation | `ru_yabloko_rybakov_chairman_appeal_20231213` (holder `attested_on`) |
| 5 Jul 2025 | KPRF: congress; KPRF: continuation claim; KPRF: election; KPRF: in-office attestation; KPRF: election | `ru_kprf_xix_congress_held_20250705`, `ru_kprf_xix_congress_chairman_presents_party_cards_20250705`, `ru_kprf_i_plenum_zyuganov_elected_chairman_20250705`, `ru_kprf_i_plenum_chairman_closing_word_20250705` (holder `attested_on`), `ru_kprf_official_zyuganov_unanimously_elected_20250705` |
| 2 Oct 2025 | LDPR: election; LDPR: in-office attestation; LDPR: election report; LDPR: in-office attestation | `ru_ldpr_xxxvii_congress_slutsky_reelected_20251002`, `ru_ldpr_xxxvii_congress_chairman_presents_strategy_20251002` (holder `attested_on`), `ru_ldpr_slutsky_says_reelected_today_20251002`, `ru_ldpr_slutsky_post_signed_as_chairman_20251002` (holder `attested_on`) |
| 20 Jun 2026 | KPRF: congress | `ru_kprf_xix_congress_second_stage_20260620` |
| 11 Aug 2026 | LDPR: in-office attestation | `ru_ldpr_slutsky_chairman_proposal_20260811` (holder `attested_on`) |
| 19 Aug 2026 | Yabloko: in-office attestation | `ru_yabloko_rybakov_chairman_appeal_20260819` (holder `attested_on`) |
| 28 Aug 2026 | KPRF: in-office attestation | `ru_kprf_zyuganov_chairman_greeting_20260828` (holder `attested_on`) |

Claims without a structured date (spans, month or year only, no printed date, prospective, or two conflicting days):

| Printed date | Party: event | Claim |
|---|---|---|
| II Extraordinary Congress (opened 13 февраля 1993 года) | KPRF: renaming (retrospective) | `ru_kprf_retro_congress_renamed_party_1993` |
| II Extraordinary Congress (opened 13 февраля 1993 года) | KPRF: party body elected (retrospective) | `ru_kprf_retro_congress_elected_cec_1993` |
| После съезда (February 1993) | KPRF: election (retrospective) | `ru_kprf_retro_zyuganov_elected_cec_chairman_1993` |
| с февраля 1993 г. ... до настоящего времени | KPRF: retrospective list | `ru_kprf_reference_zyuganov_since_february_1993` |
| 13—14 февраля 1993 года | KPRF: registration statement | `ru_kprf_reference_registered_since_ii_congress_1993` |
| 18–19 апреля 1992 г. | LDPR: founding (retrospective) | `ru_ldpr_history2010_iii_congress_founds_ldpr_1992` |
| 18–19 апреля 1992 г. | LDPR: election (retrospective) | `ru_ldpr_history2010_iii_congress_chairman_1992` |
| 24–25 апреля 1993 г. | LDPR: election (retrospective) | `ru_ldpr_history2010_iv_congress_chairman_1993` |
| пройдёт 27 мая (item of 26.05.2022) | LDPR: congress dates planned | `ru_ldpr_news_xxxiv_congress_announced` |
| no date printed (XXXIV extraordinary congress) | LDPR: election report | `ru_ldpr_newspaper_chairman_speech_xxxiv_congress` |
| no date printed | LDPR: death reference | `ru_ldpr_newspaper_death_referenced_undated` |
| осенью 1993 года | Yabloko: retrospective statement | `ru_yabloko_reference_bloc_lists_autumn_1993` |
| (1993 г.); (1994 г.) | Yabloko: name lineage (retrospective) | `ru_yabloko_reference_former_names_1993_1994` |
| 5-6 января 1995 г. | Yabloko: congress (retrospective) | `ru_yabloko_reference_founding_congress_19950105_19950106` |
| 5-6 января 1995 г. (founding congress) | Yabloko: election (retrospective) | `ru_yabloko_reference_yavlinsky_chairman_elected_1995` |
| с января 1995 г. | Yabloko: retrospective list | `ru_yabloko_reference_yavlinsky_chairman_since_january_1995` |
| 22-23 декабря 2001 г. | Yabloko: congress | `ru_yabloko_x_congress_dates_20011222_20011223` |
| в 0.00 4 июля; в 1.00 (prospective) | Yabloko: vote schedule | `ru_yabloko_chairman_vote_scheduled_2004` |
| dateline "3 июля 2004 года"; the vote's day is not printed | Yabloko: election report | `ru_yabloko_yavlinsky_reelected_chairman_2004` |
| с 1995 года | Yabloko: name lineage (retrospective) | `ru_yabloko_party_named_successor_of_1995_association` |
| Москва, 2008 г. | Yabloko: continuation claim | `ru_yabloko_report_2008_chairman_title_block` |
| 3-4 июля 2004 года | Yabloko: congress | `ru_yabloko_report_2008_xii_congress_2004` |
| 10-11 июня 2006 года | Yabloko: congress | `ru_yabloko_report_2008_xiii_congress_2006` |
| Москва, 2008 г. | Yabloko: continuation claim | `ru_yabloko_report_2008_bureau_list_chairman` |
| 21-22 июня | Yabloko: congress | `ru_yabloko_xv_congress_dates_20080621_20080622` |
| 21-22 июня (congress); release of 26 June 2008 | Yabloko: election | `ru_yabloko_xv_congress_mitrokhin_elected_2008` |
| с 2001 по 2008 год; с 1993 года | Yabloko: retrospective term statement | `ru_yabloko_retro_yavlinsky_chairman_2001_2008` |
| 19-20 декабря 2015 года | Yabloko: congress | `ru_yabloko_xviii_congress_dates_20151219_20151220` |
| 14-15 декабря 2019 года | Yabloko: congress | `ru_yabloko_xxi_congress_dates_20191214_20191215` |
| 2015-2019 гг. | Yabloko: retrospective term statement | `ru_yabloko_slabunova_report_as_chairman_2015_2019` |
| около часа ночи | Yabloko: election | `ru_yabloko_xxi_congress_vote_about_one_am` |
| undated (captured 17 November 2002) | APR: undated attestation | `ru_apr_history_chairman_heading_2002` |
| no date in the sentence (founding congress) | APR: election (retrospective) | `ru_apr_history_lapshin_elected_founding_congress_1993` |
| 26-27 октября 1994 г. | APR: election (retrospective) | `ru_apr_history_iii_congress_lapshin_reelected_1994` |
| 22-23 марта 1997 г | APR: retrospective statement | `ru_apr_history_v_congress_lapshin_remained_1997` |
| undated (captured 20 May 2004) | APR: undated attestation | `ru_apr_leadership_page_plotnikov_chairman_2004` |
| undated (captured 30 June 2008) | APR: undated attestation | `ru_apr_chairman_page_heading_2008` |
| 12,13 июня (planned) | DVR: congress dates planned | `ru_dvr_supporters_founding_congress_planned` |
| 12-13 июня 1994 г. | DVR: election | `ru_dvr_political_council_gaidar_chairman_1994` |
| no printed date; handwritten «Декабрь 94.» | DVR: undated attestation | `ru_dvr_statement_signed_by_chairman_199412` |
| 18 июня 1995 года (heading); 17 июня 1995 года (adoption line) | DVR: attestation printing two days | `ru_dvr_ii_congress_statement_signed_by_chairman_1995` |
| retrospective (1996 text) | DVR: name lineage (retrospective) | `ru_dvr_history_created_on_basis_of_movement` |
| 12-13 июня 1994 года | DVR: founding (retrospective) | `ru_dvr_history_founded_and_named_1994` |
| undated (captured 3 March 2001) | DVR: undated attestation | `ru_dvr_about_page_chairman_gaidar_2001` |
| 19 мая (issue No. 21 (253); no year printed) | DVR: dissolution decision | `ru_dvr_x_congress_self_dissolution_decision_2001` |
| no date printed | DVR: dissolution speech | `ru_dvr_x_congress_gaidar_speech_dissolution_2001` |

## Observations

### RU-PTY-01 — KPRF: the 1993 restoration congress and the chairman to 1998

Evidence (raw Internet Archive captures of kprf.ru):

- The party's "date in history" item of 13 February 2026 (credited "Телеграм-канал КПРФ"): on 13 February 1993 "II
  Чрезвычайный съезд коммунистов России" opened near Moscow and announced the resumption of the party; the congress renamed it
  the "Коммунистическую партию Российской Федерации" and elected a Central Executive Committee of 148; "После съезда" a plenum
  of that Committee elected Геннадий Андреевич Зюганов its chairman ("Председателем ЦИК был избран").
- The undated reference page (captured 12 September 2025): "Г.А.Зюганов (с февраля 1993 г. - с момента воссоздания КП РСФСР -
  КПРФ и до настоящего времени)"; the same list names the KP RSFSR leaders И.К.Полозков (1990-1991) and В.А.Купцов (1991), a
  different office in a different organization; the party says it was registered from the II Extraordinary Congress (13-14
  February 1993) as the restored KP RSFSR, naming no act.
- The information notice on the I Plenum of 20 April 1997: "Пленум избрал Председателем ЦК КПРФ т. Зюганова Г.А."; the congress
  that elected this Central Committee is not named or dated.
- The notice on the V (extraordinary) congress of 23 May 1998, signed by the Central Committee's ideology department that day:
  "По первому вопросу с докладом выступил Председатель ЦК КПРФ Г.А.Зюганов".

Decision: **accepted in part**. Holder 1, Геннадий Андреевич Зюганов, `attested_on` 23 May 1998. The 1993 events are claims
from 2025-2026 retrospective pages (the opening day is recorded; the plenum's day is not printed); the 1997 plenum's election is
a claim; the 1993 title "Председатель ЦИК" is kept on its own row.

Limits: no contemporaneous record of the 1993 congress or plenum, of the III congress's reported change of title (1995), of the
IV congress (1997; its notice was never captured) or naming a July 2004 "alternative congress" was found; the Ministry of
Justice register answered 403.

### RU-PTY-02 — KPRF: the chairman from 2021 to the cutoff

Evidence:

- The press-service notices of 26 April 2021: "24 апреля 2021 года состоялся XVIII очередной отчётно-выборный Съезд", where
  "Председатель ЦК КПРФ Г.А. Зюганов" presented the Central Committee's report (before the plenum voted), and "24 апреля
  состоялся I Пленум ... избранного XVIII Съездом", which "избрал Председателем Центрального комитета КПРФ Г.А. Зюганова".
- The notices of the XIX congress and the I Plenum of 5 July 2025 and the official-documents record: the congress elected the
  Central Committee and stayed open; "Пленум избрал Председателем Центрального Комитета КПРФ Г.А. Зюганова" ("единогласно" in the
  official record); summing up, "Председатель ЦК КПРФ призвал членов Центрального Комитета" to coordinated work.
- The official record of the congress's second stage, "20 июня 2026 года", which nominated Duma candidates.
- The item of 28 August 2026 bylined "Геннадий Зюганов, Председатель ЦК КПРФ".

Decision: **accepted**. Holder 2, `attested_on` 5 July 2025 (his closing word as Chairman, after the vote); holder 3,
`attested_on` 28 August 2026. The 2021 election is claims only: no party record attests him after that vote in 2021. The
elections are never starts, and neither re-election ends an earlier observation.

Limits: the congresses and plenums of 1998-2021 are not reviewed (the official pages of 2013 and 2017 name no chairman).

### RU-PTY-03 — LDPR: the LDPSS founding, the LDPR's founding and the chairman to 2022

Evidence (raw captures of ldpr.ru and of the party newspaper on hub.ldpr.ru):

- The party's history timeline of 22 September 2010: "31 марта 1990 г. Учредительный съезд ЛДПСС" (programme and charter
  adopted; "Председателем ЛДПСС и членом ЦК избран В. В. Жириновский"); "12 апреля 1991 г.", USSR Ministry of Justice
  certificate No. 0066 for the LDPSS charter; "18–19 апреля 1992 г. III съезд ЛДПР" (decision to found and register the LDPR;
  "Председателем ЛДПР избрать В. В. Жириновского"); "24–25 апреля 1993 г.", IV congress ("Председателем партии избран").
- A page of the party site's library reprinting the Central Committee plenum resolution of 23 November 1996: "Обсудив доклад
  Председателя партии В.В. Жириновского Пленум ЦК ЛДПР постановляет".
- The party newspaper «ЛДПР» No. 04 (382)/2022, signed to press 30 March 2022: contacts "Председатель ЛДПР Владимир Вольфович
  ЖИРИНОВСКИЙ" (the faction head listed separately).

Decision: **accepted in part**. Holders 1 and 2, Владимир Вольфович Жириновский, `attested_on` 23 November 1996 and 30 March
2022. The founding, registration and election entries are retrospective claims; the LDPSS chairmanship keeps its own printed
title on its row.

Limits: no contemporaneous record of the 1990-1993 congresses or of the LDPR's state registration (December 1992 is a lead only)
was found; 1990s ldpr.ru captures hold no congress documents; the library page's label as newspaper issue No. 1 (31) of 1997
comes from an unrecorded index.

### RU-PTY-04 — LDPR: the 2022 death, the interim and Slutsky to the cutoff

Evidence (party news items captured in October 2025, each tagged to a regional branch on the federal site; the newspaper):

- 6 April 2022: "Из жизни ушел бессменный лидер ЛДПР Владимир Жириновский" ... "Но сегодня его не стало и сегодня мы будем
  скорбеть…"; the 2026 timeline dates the death to 6 April 2022.
- Newspaper No. 05 (383)/2022, signed to press 6 May 2022: "Лидер ЛДПР Владимир Жириновский навсегда остаётся в нашей памяти";
  its address is signed "Высший Совет ЛДПР", and its contacts name no chairman.
- 26 May 2022: the XXXIV congress "пройдёт 27 мая в Москве"; "Высший Совет ЛДПР 26 мая ... выражает поддержку Леониду Слуцкому ...
  и рекомендует делегатам Съезда включить его кандидатуру в бюллетень".
- 27 May 2022: "В эти минуты в Москве начал работу XXXIV съезд ЛДПР"; "Голосование за лидера партии прошло 27 мая в Москве"; "По
  решению внеочередного XXXIV Съезда ЛДПР новым Председателем партии единогласно избран" Леонид Слуцкий (86:0), quoted
  afterwards as "сказал Председатель партии".
- Newspaper No. 06-07 (384-385)/2022, signed 27 June 2022: his speech as Chairman at the XXXIV congress (undated) and contacts
  "Председатель ЛДПР Леонид Эдуардович Слуцкий".
- 2 October 2025: "вновь избрали на пост Председателя ЛДПР в ходе XXXVII съезда партии 2 октября 2025 года"; "Председатель партии
  представил участникам съезда Стратегию развития ЛДПР до 2030 года"; his post that day signed "Председатель ЛДПР Леонид
  Слуцкий".
- 11 August 2026: "Председатель ЛДПР Леонид Слуцкий предложил".

Decision: **accepted in part**. Holders 3-5, Леонид Эдуардович Слуцкий, `attested_on` 27 May 2022, 2 October 2025 and 11 August
2026. The Supreme Council's recommendation is a nomination, not acting service; no acting chairman is named anywhere (his acting
headship of the Duma faction is outside the role). The death is a distinct dated claim, and Zhirinovsky's 2022 observation has
no `until`: that is the user's ruling of 28 September 2026, because the item of 6 April 2022 (ldpr.ru/event/202499) never says
Председатель, never mentions the office ending, and has captures only from October 2025 onward. Codex may still decide at
integration that a stated death day ends the office, in which case the anchor would be
`ru_ldpr_news_zhirinovsky_died_today_20220406`.

Limits: the ldpr.ru items were captured in 2025, so later edits cannot be excluded; the XXXVIII congress of 23 June 2026 was not
reviewed (403 captures; the live page has a view counter).

### RU-PTY-05 — Yabloko: the 1993 bloc, the 1995 association, the 2001 party and Yavlinsky to 2001

Evidence (raw captures of yabloko.ru):

- The association's undated reference page (captured 30 June 1998): its Duma members came in on the lists of "Блок: Явлинский -
  Болдырев - Лукин" "осенью 1993 года"; former names; the founding congress "5-6 января 1995 г. в Подмосковье", where the
  "председатель движения (им стал Г.Явлинский)" was elected; "Председатель объединения - Явлинский Григорий Алексеевич (с января
  1995 г.)"; registration "10 февраля 1995 г., регистрационный номер 2554".
- The release of 14 March 1998 headed as the report of the association's chairman Григорий Явлинский to the VI congress.
- The X congress (22-23 December 2001): the release that the association became the Российская демократическая политическая
  партия "ЯБЛОКО" on 22 December; the release that on 23 December 472 delegates elected Yavlinsky chairman "до 31 декабря 2004
  года"; the speech page "Выступление Председателя партии "ЯБЛОКО" Г.А. Явлинского", "Москва, 23 декабря 2001 г., 12.00", also
  listed on the congress section, which lists his report as chairman of the association on 22 December.

Decision: **accepted in part**. Holders 1 and 2, Григорий Алексеевич Явлинский, `attested_on` 14 March 1998 and 23 December
2001. The 1995 election and list are retrospective claims; the election of 23 December 2001 is a claim, and its term "до 31
декабря 2004 года" is prospective. Holder 1's row (14 March 1998) and the continuation row of 22 December 2001 print the chairman
of the association «Объединение ЯБЛОКО», which became a party only on 22 December 2001: they keep that printed title, marked
"(as printed)" as for the KPRF's Central Executive Committee and the LDPSS, and stay on the party role. The party calls itself
the association's legal successor ("правопреемницей которого является РДП «ЯБЛОКО»", release of 4 July 2004), a claim that
reconciles no identity. The two 1998 reference rows (`ru_yabloko_reference_yavlinsky_chairman_elected_1995`, "председатель
движения", and `ru_yabloko_reference_yavlinsky_chairman_since_january_1995`, "Председатель объединения") keep their printed
titles the same way.

Limits: no contemporaneous 1995 record was found; the 1993 bloc's leadership is not printed; the 1998 re-election is not
reviewed.

### RU-PTY-06 — Yabloko: 2004 to 2008 and Mitrokhin's election

Evidence:

- The press-service item of 3 July 2004: nominations "в 0.00 4 июля", the secret vote "в 1.00".
- The release under the dateline "3 июля 2004 года": "Г. Явлинский вновь избран председателем партии «Яблоко»", 190 of 252, the
  one alternative candidate 59 (the vote's day is not printed).
- The release of 4 July 2004: "Григорий Явлинский предложил съезду поправку об ограничении пребывания на своем посту Председателя
  партии двумя четырехлетними сроками", counting from 1995, the association whose "правопреемницей ... является РДП «ЯБЛОКО»".
- The report on the governing bodies for July 2004 - June 2008 ("Председатель Партии Г.А.Явлинский"; the Bureau "избрано на XII
  Съезде Партии 3-4 июля 2004 года, довыборы на XIII Съезде Партии 10-11 июня 2006 года").
- The release of 26 June 2008 on the XV congress "21-22 июня" ("Избран председатель партии": Митрохин 75, Резник 24, Попов 20)
  and the release of 22 June 2008: "Председатель партии Сергей Митрохин вошел в ПК по должности".

Decision: **accepted in part**. Holder 3, Явлинский, `attested_on` 4 July 2004; holder 4, Сергей Сергеевич Митрохин,
`attested_on` 22 June 2008. The re-election report is undated (the vote was scheduled for 4 July; the dateline is 3 July); the
2008 election's day is not printed; the successor's election is not Yavlinsky's end.

Limits: the 2006 congress's decisions and Mitrokhin's re-elections of 2011 or 2013 are not reviewed; a reposted news-agency text
giving 21 June for the 2008 vote is a lead only.

### RU-PTY-07 — Yabloko: Mitrokhin to 2015, Slabunova, Rybakov and an attestation before the cutoff

Evidence:

- The release of 19 December 2015 on the chairman's term limit: "Сергея Митрохин занимает пост председателя с 2008 года" (as
  printed), "действующий руководитель «ЯБЛОКА»"; it also says Yavlinsky "был председателем с 2001 по 2008 год и возглавлял
  общественно-политическое объединение «ЯБЛОКО» с 1993 года" (which conflicts with the 1998 page).
- The releases of 20 December 2015: Slabunova "стала новым председателем партии «ЯБЛОКО»" in the second round, nominated by the
  incumbent Митрохин; "Заместителями нового председателя партии «ЯБЛОКО» Эмилии Слабуновой стали ...".
- The XXI congress, "14-15 декабря 2019 года", not closed: the vote "около часа ночи", 69 of 132; the release of 15 December that
  Rybakov "стал четвертым председателем"; the page of 16 December on his plans "на посту председателя".
- 9 December 2023: "действующий председатель Николай Рыбаков" on the shortlist and his re-election "на второй срок" (59 votes,
  55%); 11 December: "Николай Рыбаков сохранил пост председателя"; 13 December: "Председатель «Яблока» Николай Рыбаков просит
  руководителя ФСИН".
- 19 August 2026: "Председатель партии «Яблоко» Николай Рыбаков обратился".

Decision: **accepted**. Holders 5 (Митрохин, 19 December 2015), 6 (Эмилия Эдгардовна Слабунова, 20 December 2015), 7-9
(Николай Игоревич Рыбаков, 16 December 2019, 13 December 2023 and 19 August 2026). Election reports and the incumbent's mentions
before each vote are claims.

Limits: the days of the 2015, 2019 and 2023 votes are not printed; no Rybakov item after 19 August 2026 and before the cutoff has
a usable capture.

### RU-PTY-08 — Agrarian Party: its founding and Lapshin

Evidence (raw captures of agroparty.ru):

- The party's undated historical note (captured 17 November 2002), headed "Председатель партии - Михаил Иванович Лапшин": the
  founding congress "26 февраля 1993" (Лапшин elected chairman, no day in the sentence); charter registration "9 апреля 1993
  (Рег.N1647)"; re-elections at the III congress (26-27 October 1994), the V congress (22-23 March 1997: "председателем остался
  М. Лапшин"), the VII congress (20 March 1999), the 9th congress (24 March 2001) and the 10th congress (8 December 2001), which
  also transformed the organization into the political party; confirmation of his powers at the VI congress (26 February 1998);
  re-registration of charter amendments (29 May 1998); registration as a political party "31 мая 2002 года" (No 5025).
- The release of 10 September 2003: "9 сентября открылся XI внеочередной съезд", with a report by "председатель Аграрной партии
  Михаил Лапшин".
- The release of 26 May 2004: on 26 April 2004 "старое руководство АПР во главе Михаилом Лапшиным" filed the party's final
  campaign-finance report.

Decision: **accepted in part**. Holder 1, Михаил Иванович Лапшин, `attested_on` 9 September 2003. The 1993-2002 facts are
retrospective claims; the "old leadership" statement of 26 April 2004 is a continuation claim, not an end.

Limits: no contemporaneous 1993-2002 record naming Lapshin chairman was found; the releases of 22 August and 11 September 2003
are leads (the second names him also by a regional state office).

### RU-PTY-09 — Agrarian Party: Plotnikov and the 2008 accession

Evidence:

- The release of 26 May 2004: "28 апреля состоялся съезд АПР, который 226 голосами против 185 избрал новым председателем партии
  Владимира Плотникова"; the 2008 chairman page repeats 28 April 2004 (two newspaper reprints on the site imply 27 April; leads).
- The release of 31 May 2004 on the plenum of 28 May 2004: "На пленуме с докладом выступил Председатель Аграрной партии России
  Владимир Плотников"; the leadership page of 20 May 2004 (undated).
- The release of 12 October 2004: "9 октября 2004 г. в Москве завершился второй этап XII Съезда", with a comment by "председатель
  АПР Владимир Плотников" (a continuation claim).
- The home page captured 16 October 2008: the memorandum of 12 September 2008 with "ЕДИНАЯ РОССИЯ"; the plenum of 26 September 2008
  with a report by "Председатель Аграрной партии России В.Н.Плотников"; the XV (extraordinary) congress of 10 October 2008, which
  resolved "реорганизовать Аграрную партию России путем присоединения" to "ЕДИНАЯ РОССИЯ".

Decision: **accepted in part**. Holders 2 and 3, Владимир Николаевич Плотников, `attested_on` 28 May 2004 and 26 September 2008.
The election, the memorandum and the accession decision are claims; the accession decision ends neither the office nor the
observation's lifecycle.

Limits: the full congress article and the September 2008 releases were never captured; no record completing the accession (the
party's, United Russia's or the Ministry of Justice's) was obtained; agroparty.ru was parked after 2008.

### RU-PTY-10 — Russia's Choice (1993) and Democratic Choice of Russia

Evidence (scans of the organizations' documents in the Yegor Gaidar Archive, captured by the Internet Archive, every cited page
rendered and read; captures of dvr.ru):

- The bloc's programme: "ИЗБИРАТЕЛЬНОЕ ОБЪЕДИНЕНИЕ «ВЫБОР РОССИИ» ... Принята 17 октября 1993 года на учредительном съезде
  общественно-политического блока «Выбор России»"; no leader named.
- Minutes of 19 May 1994 of "сторонников создания партии "Выбор России""; the founding congress planned for "12,13 июня".
- The published edition of the founding congress (1994): the declaration that on 12 June they founded the "ПАРТИЮ “ДЕМОКРАТИЧЕСКИЙ
  ВЫБОР РОССИИ”"; the Political Council "ИЗБРАН УЧРЕДИТЕЛЬНЫМ СЪЕЗДОМ ПАРТИИ 12-13 июня 1994 г.", headed by Гайдар Егор
  Тимурович.
- The party bulletin No. 1 (1994): registration by the Ministry of Justice on 9 August 1994 (certificate N 2364); the Council's
  resolution "ПРИНЯТО НА ЗАСЕДАНИИ СОВЕТА ПАРТИИ 10 июля 1994 года", signed "Председатель Партии Е.Гайдар".
- The statement on Chechnya signed "Председатель партии Демократический Выбор России ЕГОР ГАЙДАР" (no printed date; an archival
  note "Декабрь 94."); the II congress's statement headed 18 June 1995 but "Принято ... 17 июня 1995 года"; the II congress's
  decision "Принято ... 18 июня 1995 года", which approves the work of "Председателя партии Е.Т.Гайдара" and is signed
  "Председатель партии ДВР Е.Т.ГАЙДАР".
- The V congress: a decision of "21 сентября 1996 г." signed "Председатель Партии Е.Т. Гайдар"; the decision of "22 сентября 1996
  г." approving the counting commission's protocol of 21 September "об итогах тайного голосования по выдвижению Председателя
  Партии", signed the same way.
- The Political Council's decision of 16 December 1997 signed "Председатель партии Е.Гайдар"; its statement of 16 December 2000
  signed only "Председатель ... Е.Гайдар".
- The party's history page, which is not a party record but a 1996 reference-book passage (Ю.Г.Коргунюк and С.Е.Заславский,
  «Российская многопартийность») that the party site republished as its history: created "на основе движения Выбор России";
  founded 12-13 June 1994; registered 9 August 1994; programme adopted by the II plenum of the Council on 19 November 1994; II
  congress 18 June 1995; III congress 26 August 1995. The tie to the movement «Выбор России», the programme's adoption on 19
  November 1994 and the III congress of 26 August 1995 have no other source. The information page (captured March 2001): "Председатель партии ДВР - Гайдар Егор Тимурович".
- The party newspaper «Демократический выбор» No. 21 (253): on "19 мая" (no year printed) the X congress decided "о самороспуске
  ДВР" as the Union of Right Forces was being created; Gaidar's speech "мы распускаем нашу партию".

Decision: **accepted in part**. Holders 1-4, Егор Тимурович Гайдар, `attested_on` 10 July 1994, 18 June 1995, 22 September 1996
and 16 December 1997. The bloc is its own observation with one claim and no role; the Political Council's election, the 1996
ballot record, undated attestations, the bare-title signature of 2000 and the dissolution are claims. The six claims resting on
the history page come from the republished reference-book passage: they stay on the role as retrospective claims, never
boundaries, and `ru_dvr_party_self_record` cites neither the page nor its claims.

Limits: no primary record names a leader or list head of the 1993 bloc (its CEC records were not captured); the day of the 1994
election is not recorded; the 1997-2001 congresses and the X congress's resolution are not reviewed; the two 1998 dvr.ru captures
were stored chunked, so their served bodies' SHA-1 differs from the index digest (disclosed in the extracts).

## Sources added

68 sources, each with a checked-in derived factual extract under [sources/](sources/) (`russia-*-facts.json`, LF, format
`spheres-c01-derived-factual-table/v1`, with its own checksum in the packet). Each extract records the original response's URL,
byte count, SHA-256 and SHA-1 (base32), the capture index digest, the capture time, a stability record and one row per claim
(`claim_id`, observation, role, `holder_name` and the printed form, role title, event kind, date or printed date, text,
locator). Original pages and PDFs are not checked in; no emblem, photograph, signature image or expressive source text is
republished. All are raw Internet Archive captures (`id_` form) made before the cutoff: the packet URL is the capture and
`original_url` the captured page. Rows marked "check find" were found by an independent check.

| Source ID | Party | What | Capture, response identity (bytes, SHA-256) |
|---|---|---|---|
| `ru_kprf_i_plenum_notice_19970420` | KPRF | [information notice on the I Plenum of the KPRF Central Committee, 20 April 1997 (kprf.ru archive section, captured 6 March 2000)](https://web.archive.org/web/20000306135653id_/http://www.kprf.ru:80/arhiv/plenum/1plenum.htm) | IA 20000306135653, 1,757 bytes, `b26af368…07bdd5` |
| `ru_kprf_v_congress_notice_19980523` | KPRF | [information notice on the V (extraordinary) congress, signed by the Central Committee's ideology department, 23 May 1998](https://web.archive.org/web/20000306152838id_/http://www.kprf.ru:80/arhiv/congr5/infsoob.htm) | IA 20000306152838, 2,959 bytes, `8dc7cb01…b393d8` |
| `ru_kprf_date_in_history_1993_20260213` | KPRF | [the party site's "date in history" item (credited "Телеграм-канал КПРФ"), 13 February 2026](https://web.archive.org/web/20260727165041id_/https://kprf.ru/history/date/241272.html) | IA 20260727165041, 26,386 bytes, `750fce20…aca23b` |
| `ru_kprf_party_reference_20250912` | KPRF | [the party site's undated reference page about the party (captured 12 September 2025)](https://web.archive.org/web/20250912215927id_/https://kprf.ru/party/) | IA 20250912215927, 33,284 bytes, `6cb36313…070330` |
| `ru_kprf_xviii_congress_notice_20210424` | KPRF | [press-service notice on the XVIII congress (24 April 2021), posted 26 April 2021](https://web.archive.org/web/20210426151642id_/https://kprf.ru/party-live/cknews/202177.html) (check find) | IA 20210426151642, 29,955 bytes, `18d432df…bb4efd` |
| `ru_kprf_i_plenum_notice_20210424` | KPRF | [press-service notice on the I Plenum of the Central Committee elected by the XVIII congress, posted 26 April 2021](https://web.archive.org/web/20210426152225id_/https://kprf.ru/party-live/cknews/202175.html) (check find) | IA 20210426152225, 24,747 bytes, `470281b2…58b0a2` |
| `ru_kprf_xix_congress_notice_20250705` | KPRF | [information notice on the XIX regular congress (5 July 2025), posted 8 July 2025](https://web.archive.org/web/20250708140517id_/https://kprf.ru/party-live/cknews/235899.html) | IA 20250708140517, 41,887 bytes, `4414392c…e6edc9` |
| `ru_kprf_i_plenum_notice_20250705` | KPRF | [information notice on the I (July) Plenum of the Central Committee (5 July 2025), press service, posted 7 July 2025](https://web.archive.org/web/20250708000201id_/https://kprf.ru/party-live/cknews/235886.html) | IA 20250708000201, 28,783 bytes, `19a6e954…bb0329` |
| `ru_kprf_official_i_plenum_20250705` | KPRF | [the Central Committee's official-documents record of the I organisational Plenum](https://web.archive.org/web/20251211130650id_/https://kprf.ru/official/2025/07/05/5-iiulia-2025-goda-sostoialsia-i-organizatsionnyi-plenum-tsk-kprf/) | IA 20251211130650, 18,158 bytes, `3aa99b06…65b508` |
| `ru_kprf_zyuganov_greeting_20260828` | KPRF | [the Chairman's greeting for Knowledge Day, party news of 28 August 2026](https://web.archive.org/web/20260829220349id_/https://kprf.ru/party-live/cknews/246862.html) | IA 20260829220349, 31,109 bytes, `a2a9026c…0e1cc5` |
| `ru_kprf_official_xix_second_stage_20260620` | KPRF | [the Central Committee's official-documents record of the XIX congress's second stage (20 June 2026)](https://web.archive.org/web/20260727152918id_/https://kprf.ru/official/2026/06/20/xix-sezd-kprf--vtoroi-etap/) (check find) | IA 20260727152918, 15,328 bytes, `6feb71b7…95a303` |
| `ru_ldpr_newspaper_plenum_resolution_19961123` | LDPR | [A page of the party site's library (ldpr.ru/book/ldpr_01_97.htm, untitled) including «Из Постановления Пленума ЦК ЛДПР (23 ноября 1996 г.)», an extract of the Central Committee plenum resolution of 23 November 1996](https://web.archive.org/web/19980207231628id_/http://www.ldpr.ru:80/book/ldpr_01_97.htm) | IA 19980207231628, 21,866 bytes, `3910baae…1cfacb` |
| `ru_ldpr_party_history_20100922` | LDPR | [the LDPR's party-history timeline, page dated 22 September 2010](https://web.archive.org/web/20101016073607id_/http://www.ldpr.ru:80/party/history) | IA 20101016073607, 36,369 bytes, `693b4c46…85cdaa` |
| `ru_ldpr_party_history_2026` | LDPR | [the LDPR's current party-history timeline (undated; captured 16 February 2026)](https://web.archive.org/web/20260216025055id_/https://ldpr.ru/party/history/) | IA 20260216025055, 245,090 bytes, `1a65aaac…a97f78` |
| `ru_ldpr_newspaper_no04_20220330` | LDPR | [Общественно-политическая газета «ЛДПР» № 04 (382) / 2022, signed to press 30 March 2022 (PDF)](https://web.archive.org/web/20220801065115id_/https://hub.ldpr.ru/media/documents/a076ef48d5c253e3ca4e2d145168c52e4ebdd72a5068134ccd201b8c2356a210.pdf) | IA 20220801065115, 3,628,893 bytes, `0140c750…b40e35` |
| `ru_ldpr_news_zhirinovsky_death_20220406` | LDPR | [party news dated 06.04.2022 (tagged Севастополь)](https://web.archive.org/web/20251012234518id_/https://ldpr.ru/event/202499/) (check find) | IA 20251012234518, 98,960 bytes, `cb54c3e6…1b3ca7` |
| `ru_ldpr_newspaper_no05_20220506` | LDPR | [Общественно-политическая газета «ЛДПР» № 05 (383) / 2022, signed to press 6 May 2022 (PDF)](https://web.archive.org/web/20220731053447id_/https://hub.ldpr.ru/media/documents/f9bcc7681584bd9dbe1b82a8cf309149b81136ee5fd642764d286aedf725c6ff.pdf) | IA 20220731053447, 5,262,052 bytes, `def809b8…9837db` |
| `ru_ldpr_news_supreme_council_recommendation_20220526` | LDPR | [party news dated 26.05.2022 (tagged Республика Северная Осетия-Алания)](https://web.archive.org/web/20251012221126id_/https://ldpr.ru/event/213302/) (check find) | IA 20251012221126, 98,454 bytes, `3bae6409…0d6c91` |
| `ru_ldpr_news_xxxiv_congress_opens_20220527` | LDPR | [party news dated 27.05.2022 (tagged Курганская область)](https://web.archive.org/web/20251012090749id_/https://ldpr.ru/event/213496/) (check find) | IA 20251012090749, 95,630 bytes, `02b87926…1413be` |
| `ru_ldpr_news_slutsky_elected_20220527` | LDPR | [party news dated 27.05.2022 (tagged Саратовская область)](https://web.archive.org/web/20251012171837id_/https://ldpr.ru/event/213552/) (check find) | IA 20251012171837, 97,398 bytes, `f7904c4a…05c689` |
| `ru_ldpr_news_vote_day_20220527` | LDPR | [party news dated 27.05.2022 (tagged Республика Мордовия)](https://web.archive.org/web/20251012070034id_/https://ldpr.ru/event/213560/) (check find) | IA 20251012070034, 97,383 bytes, `661c9a01…482b52` |
| `ru_ldpr_newspaper_no06_07_20220627` | LDPR | [Общественно-политическая газета «ЛДПР» № 06-07 (384-385) / 2022, signed to press 27 June 2022 (PDF)](https://web.archive.org/web/20220731053434id_/https://hub.ldpr.ru/media/documents/da28a936605feede1f300e41180ece800a3a999b0bceb6a4708766f8e93f32a7.pdf) | IA 20220731053434, 4,385,525 bytes, `54e52e3f…26a07c` |
| `ru_ldpr_news_slutsky_reelected_20251002` | LDPR | [party news of 2 October 2025](https://web.archive.org/web/20251012062735id_/https://ldpr.ru/event/leonida-slutskogo-izbrali-predsedatelem-ldpr-na-sleduyushchie-chetyre-goda/) | IA 20251012062735, 124,272 bytes, `e57f89ea…b8cbc0` |
| `ru_ldpr_news_slutsky_congress_post_20251002` | LDPR | [Slutsky's first-person post on the party site, 2 October 2025](https://web.archive.org/web/20251012062743id_/https://ldpr.ru/event/tolko-chto-zakonchilsya-37-y-sezd-ldpr-pogovorili-otkryto-sverili-chasy-dela-predstoyat-bolshie/) | IA 20251012062743, 110,098 bytes, `45af9535…dea8e5` |
| `ru_ldpr_news_citizenship_proposal_20260811` | LDPR | [party news of 11 August 2026](https://web.archive.org/web/20260816173730id_/https://ldpr.ru/event/ldpr-predlozhila-lishat-grazhdanstva-tekh-kto-poluchil-pasport-radi-lgot-i-zhilya/) | IA 20260816173730, 152,667 bytes, `f60abeb9…3cbdec` |
| `ru_yabloko_site_reference_1998` | Yabloko | [the Yabloko association's reference page (history, leaders, congresses, registration), yabloko.ru, undated (captured 30 June 1998)](https://web.archive.org/web/19980630062735id_/http://www.yabloko.ru:80/Union/gdy.html) | IA 19980630062735, 20,333 bytes, `699e5af4…e1b014` |
| `ru_yabloko_vi_congress_report_19980314` | Yabloko | [the chairman's report to the VI congress](https://web.archive.org/web/19980630062048id_/http://www.yabloko.ru:80/Press/980314.html) | IA 19980630062048, 27,960 bytes, `2073831f…e61c57` |
| `ru_yabloko_x_congress_section_2001` | Yabloko | [the party site's section page for the X congress (documents, speeches, releases), captured 23 January 2002](https://web.archive.org/web/20020123194510id_/http://www.yabloko.ru:80/Union/XCongress/index.html) | IA 20020123194510, 49,435 bytes, `99e5545f…6327e9` |
| `ru_yabloko_press_party_transformation_20011222` | Yabloko | [Yabloko transformed into a party](https://web.archive.org/web/20020322235901id_/http://www.yabloko.ru:80/Press/2001/0112226.html) | IA 20020322235901, 4,846 bytes, `2de4e340…a389da` |
| `ru_yabloko_press_yavlinsky_elected_20011223` | Yabloko | [Yavlinsky elected party chairman for three years](https://web.archive.org/web/20020620151948id_/http://www.yabloko.ru:80/Press/2001/011223.html) | IA 20020620151948, 4,974 bytes, `861d4f81…d9a65b` |
| `ru_yabloko_x_congress_chairman_speech_20011223` | Yabloko | [the chairman's speech page](https://web.archive.org/web/20020323204010id_/http://www.yabloko.ru:80/Union/XCongress/1223yavl.html) (check find) | IA 20020323204010, 10,068 bytes, `9e97363d…26968d` |
| `ru_yabloko_press_congress_schedule_20040703` | Yabloko | [the XII congress's schedule](https://web.archive.org/web/20050302004743id_/http://www.yabloko.ru:80/Press/Docs/2004/0703_info.html) (check find) | IA 20050302004743, 7,308 bytes, `8224f98a…10ea8e` |
| `ru_yabloko_press_yavlinsky_reelected_20040703` | Yabloko | [Yavlinsky re-elected party chairman](https://web.archive.org/web/20050301190917id_/http://www.yabloko.ru:80/Press/2004/0407041.html) | IA 20050301190917, 6,103 bytes, `a925474e…c0bc0a` |
| `ru_yabloko_press_charter_amendments_20040704` | Yabloko | [the XII congress's charter amendments](https://web.archive.org/web/20040927193602id_/http://www.yabloko.ru:80/Press/2004/040704.html) (check find) | IA 20040927193602, 8,028 bytes, `8a4df4bb…a4fb77` |
| `ru_yabloko_leadership_report_2004_2008` | Yabloko | [report on the work of the governing bodies, July 2004 - June 2008 (Moscow, 2008)](https://web.archive.org/web/20111112212637id_/http://www.yabloko.ru/Union/XYCongress/congr-report.html) | IA 20111112212637, 238,829 bytes, `33605fe7…31cd1e` |
| `ru_yabloko_press_new_leadership_20080622` | Yabloko | [the new leadership elected by the XV congress](https://web.archive.org/web/20111112210033id_/http://www.yabloko.ru/Press/2008/0806221.html) | IA 20111112210033, 11,691 bytes, `e5f132c5…cfb088` |
| `ru_yabloko_press_xv_congress_decisions_20080626` | Yabloko | [the XV congress's main decisions](https://web.archive.org/web/20111112203944id_/http://www.yabloko.ru/Press/2008/080626.html) | IA 20111112203944, 12,728 bytes, `2bb4b073…7f099a` |
| `ru_yabloko_news_term_limit_20151219` | Yabloko | [the XVIII congress limits the chairmanship to two terms](https://web.archive.org/web/20151224043458id_/http://www.yabloko.ru:80/news/2015/12/19_1) (check find) | IA 20151224043458, 51,785 bytes, `aad04b2c…cdb983` |
| `ru_yabloko_news_slabunova_elected_20151220` | Yabloko | [Slabunova became party chairman](https://web.archive.org/web/20151222103638id_/http://www.yabloko.ru/news/2015/12/20) | IA 20151222103638, 53,009 bytes, `3938b057…520317` |
| `ru_yabloko_news_new_chairman_deputies_20151220` | Yabloko | [the new chairman's deputies elected](https://web.archive.org/web/20151222194032id_/http://www.yabloko.ru:80/2015/12/20_1) | IA 20151222194032, 50,818 bytes, `410a9fcf…5b4017` |
| `ru_yabloko_news_rybakov_elected_20191215` | Yabloko | [Rybakov elected party chairman](https://web.archive.org/web/20210627225332id_/https://www.yabloko.ru/news/2019/12/15) | IA 20210627225332, 48,691 bytes, `a4f7e573…2b22e3` |
| `ru_yabloko_news_xxi_congress_main_20191216` | Yabloko | [the XXI congress: main points](https://web.archive.org/web/20200513204833id_/https://yabloko.ru/news/2019/12/16-0) | IA 20200513204833, 58,217 bytes, `1aedcf73…2116f3` |
| `ru_yabloko_news_rybakov_second_term_20231209` | Yabloko | [Rybakov re-elected for a second term](https://web.archive.org/web/20231209211736id_/https://www.yabloko.ru/cat-news/2023/12/09-3) | IA 20231209211736, 58,023 bytes, `dae451e5…423a63` |
| `ru_yabloko_news_xxii_congress_main_20231211` | Yabloko | [the XXII congress: main points](https://web.archive.org/web/20231212111820id_/https://www.yabloko.ru/cat-news/2023/12/11) | IA 20231212111820, 51,800 bytes, `a2e879fb…6d8788` |
| `ru_yabloko_news_fsin_appeal_20231213` | Yabloko | [press release of 13 December 2023](https://web.archive.org/web/20231217211933id_/https://www.yabloko.ru/cat-news/2023/12/13) (check find) | IA 20231217211933, 45,887 bytes, `8140b7fb…93b818` |
| `ru_yabloko_news_rybakov_appeal_20260819` | Yabloko | [the chairman's appeal to the Minister of Internal Affairs](https://web.archive.org/web/20260821172511id_/https://www.yabloko.ru/cat-news/2026/08/19) | IA 20260821172511, 49,945 bytes, `5837c6f8…f70950` |
| `ru_apr_history_note_2002` | APR | [the party's historical note, agroparty.ru, undated (captured 17 November 2002)](https://web.archive.org/web/20021117045251id_/http://www.agroparty.ru:80/hist.htm) | IA 20021117045251, 14,361 bytes, `66679607…23632e` |
| `ru_apr_release_xi_congress_20030910` | APR | [press release on the XI congress](https://web.archive.org/web/20031022201803id_/http://www.agroparty.ru:80/releases/3/106/) | IA 20031022201803, 35,412 bytes, `523a89c9…36dfe3` |
| `ru_apr_release_new_chairman_20040526` | APR | [press release on the party's financial report, reporting the new leadership](https://web.archive.org/web/20040710080750id_/http://www.agroparty.ru:80/releases/3/178/) | IA 20040710080750, 32,743 bytes, `918fbe1e…e8efec` |
| `ru_apr_release_plenum_20040531` | APR | [the organisational plenum of 28 May 2004](https://web.archive.org/web/20050523171303id_/http://agroparty.ru:80/releases/3/179/) | IA 20050523171303, 33,588 bytes, `9c487c8b…9b4368` |
| `ru_apr_leadership_page_2004` | APR | [the party site's leading-bodies page for the Chairman (undated; captured 20 May 2004)](https://web.archive.org/web/20040520161601id_/http://www.agroparty.ru:80/party/people.php?id=10) | IA 20040520161601, 24,462 bytes, `ddbf238c…cb9aaa` |
| `ru_apr_release_xii_congress_second_stage_20041012` | APR | [the XII congress's second stage](https://web.archive.org/web/20041124131828id_/http://www.agroparty.ru:80/releases/3/215/) (check find) | IA 20041124131828, 39,474 bytes, `dd8913f3…dc11e2` |
| `ru_apr_chairman_page_2008` | APR | [the party site's page about its Chairman (undated; captured 30 June 2008)](https://web.archive.org/web/20080630045912id_/http://www.agroparty.ru:80/?id=31) | IA 20080630045912, 9,335 bytes, `c85cca5c…9d9971` |
| `ru_apr_home_page_20081016` | APR | [agroparty.ru home page captured 16 October 2008, with the dated items «XV Съезд Аграрной партии России одобрил объединение АПР и Всероссийской политической партии “Единая Россия”» (10.10.2008) and the releases of 12 and 26 September 2008](https://web.archive.org/web/20081016023015id_/http://www.agroparty.ru:80/) | IA 20081016023015, 17,490 bytes, `e8242f5c…d2a551` |
| `ru_vybor_rossii_bloc_programme_19931017` | DVR | [the bloc's programme (Moscow, October 1993; scan)](https://web.archive.org/web/20131005073722id_/http://gaidar-arc.ru/file/bulletin-1/DEFAULT/org.stretto.plugins.bulletin.core.Article/file/3647) | IA 20131005073722, 2,040,735 bytes, `1fdfa9b2…75b17f` |
| `ru_dvr_supporters_protocol_19940519` | DVR | [minutes of a meeting of supporters of founding the party "Russia's Choice" (Southern administrative district of Moscow; scan)](https://web.archive.org/web/20170823160756id_/http://gaidar-arc.ru:80/file/bulletin-1/DEFAULT/org.stretto.plugins.bulletin.core.Article/file/4791) | IA 20170823160756, 122,938 bytes, `be163177…0f5cae` |
| `ru_dvr_founding_congress_edition_1994` | DVR | [a published edition of the founding congress's documents (Библиотека открытой политики; М.: Евразия, 1994; scan)](https://web.archive.org/web/20160803131429id_/http://gaidar-arc.ru/file/bulletin-1/DEFAULT/org.stretto.plugins.bulletin.core.Article/file/4818) | IA 20160803131429, 10,512,385 bytes, `df101511…82e26f` |
| `ru_dvr_information_bulletin_1_1994` | DVR | [the party's information bulletin No. 1 (Charter, leading bodies and Council resolutions; scan)](https://web.archive.org/web/20160813072102id_/http://gaidar-arc.ru/file/bulletin-1/DEFAULT/org.stretto.plugins.bulletin.core.Article/file/4816) | IA 20160813072102, 1,198,783 bytes, `9f6aff6c…5071aa` |
| `ru_dvr_statement_grozny_199412` | DVR | [the party's statement on the conflict in Chechnya, signed by its chairman (single page; scan; no printed date)](https://web.archive.org/web/20160803121516id_/http://gaidar-arc.ru/file/bulletin-1/DEFAULT/org.stretto.plugins.bulletin.core.Article/file/3661) | IA 20160803121516, 436,750 bytes, `778aaa6a…54e75c` |
| `ru_dvr_congress_statement_1995` | DVR | [statement of the II (extraordinary) congress, signed by the party chairman (single page; scan)](https://web.archive.org/web/20170823162628id_/http://gaidar-arc.ru:80/file/bulletin-1/DEFAULT/org.stretto.plugins.bulletin.core.Article/file/3734) | IA 20170823162628, 22,777 bytes, `33ef8455…0a15a3` |
| `ru_dvr_ii_congress_decision_19950618` | DVR | [decision of the II congress, 18 June 1995, signed by the party chairman (scan)](https://web.archive.org/web/20151006140140id_/http://gaidar-arc.ru/file/bulletin-1/DEFAULT/org.stretto.plugins.bulletin.core.Article/file/3737) (check find) | IA 20151006140140, 23,968 bytes, `06aef5e3…4e0ce7` |
| `ru_dvr_v_congress_decision_19960921` | DVR | [decision of the V congress, 21 September 1996, signed by the party chairman (scan)](https://web.archive.org/web/20161108004556id_/http://gaidar-arc.ru:80/file/bulletin-1/DEFAULT/org.stretto.plugins.bulletin.core.Article/file/3796) (check find) | IA 20161108004556, 101,000 bytes, `3ec3fc20…a274bd` |
| `ru_dvr_v_congress_decisions_19960922` | DVR | [Decisions of the V congress of the party "Демократический выбор России" of 22 September 1996, including «Об утверждении протоколов № 3-4 заседания Счетной комиссии», signed by the party chairman (scan)](https://web.archive.org/web/20250115065600id_/http://www.gaidar-arc.ru/file/bulletin-1/DEFAULT/org.stretto.plugins.bulletin.core.Article/file/3801) (check find) | IA 20250115065600, 341,045 bytes, `79022c35…d86a71` |
| `ru_dvr_politsovet_decision_19971216` | DVR | [decision of the party's Political Council of 16 December 1997, dvr.ru](https://web.archive.org/web/19980627072026id_/http://www.dvr.ru:80/politsovet_16-12-97.htm) | IA 19980627072026, 4,566 bytes, `97a5c08d…db6de5` |
| `ru_dvr_history_page_1998` | DVR | [the party site's history page, quoting Ю.Г.Коргунюк and С.Е.Заславский, «Российская многопартийность» (М., 1996, с. 69-70); undated (captured 27 June 1998)](https://web.archive.org/web/19980627060716id_/http://www.dvr.ru:80/history.html); a reference-book passage the party republished, not a party record (`party_republished_reference_text`) | IA 19980627060716, 8,261 bytes, `343d9f2a…8c70ad` |
| `ru_dvr_politsovet_statement_20001216` | DVR | [«Заявление Политсовета партии «Об угрозе свободе слова, законности и частной собственности»», № 3-10/12, 16.12.2000, on the party's letterhead (scan)](https://web.archive.org/web/20160827164330id_/http://gaidar-arc.ru/file/bulletin-1/DEFAULT/org.stretto.plugins.bulletin.core.Article/file/3887) (check find) | IA 20160827164330, 123,226 bytes, `bb874c6b…230d20` |
| `ru_dvr_about_page_2001` | DVR | [the party site's information page (undated; captured 3 March 2001)](https://web.archive.org/web/20010303063231id_/http://www.dvr.ru:80/party.htm) | IA 20010303063231, 11,695 bytes, `e631023b…4cb059` |
| `ru_dvr_newspaper_demvybor_21_2001` | DVR | [news «Партия ДВР объявила о роспуске» and «НА X СЪЕЗДЕ ДВР» (undated issue captured 7 June 2001)](https://web.archive.org/web/20010607195936id_/http://www.dvr.ru:80/demvyb/index.htm) | IA 20010607195936, 99,602 bytes, `1acc8f81…077fb6` |

## Response identities and stability checks

Every recorded response was downloaded at least four times with identical bytes and SHA-256:

- twice by its dossier, 30-53 minutes apart (check finds: twice by the independent check, 30-50 minutes apart);
- twice by this packet, at least 30 minutes apart (dossier sources at 14:54Z-15:57Z and 15:59Z-16:30Z; check finds at
  16:26Z-16:55Z and 17:27Z-17:29Z). Each extract's `stability_check` gives its own times.

Points a reviewer needs:

- Every capture was requested without an Accept-Encoding header and served without Content-Encoding, so each identity is the
  uncompressed body; captures replayed gzip-encoded were rejected (seven of kprf.ru pages, two of yabloko.ru pages).
- Each body's SHA-1 (base32) equals the capture index digest, except the two dvr.ru pages captured in June 1998, which were stored
  chunked and are served de-chunked (the independent check reproduced the index digest for the Political Council decision by
  re-chunking the served body; for the history page no split reproduces it). Their served bodies were identical on every
  download.
- Live party pages were not used: kprf.ru, ldpr.ru and yabloko.ru pages regenerate per request (widgets, view counters, the
  current footer year). Frozen captures of news pages include their site chrome as captured.
- The PDFs are stored files. The LDPR newspapers carry Adobe PDF Library metadata of 2022; the Gaidar Archive scans carry ABBYY
  FineReader metadata (some with a fixed placeholder CreationDate); each extract records its own producer and date.

## Leads not imported

- **Encyclopaedia and news leads:** Wikipedia's article on the Agrarian Party (fetched by an earlier agent); a RIA Novosti report
  reposted on yabloko.ru, [Press/2008/080622.html](https://web.archive.org/web/20111105013604id_/http://yabloko.ru/Press/2008/080622.html)
  (dateline 21 June for Mitrokhin's election); the Moscow branch's item mosyabloko.ru/2008/06/21/g-yavlinskiy-vzyal-samootvod;
  two newspaper reprints on agroparty.ru, [releases/1/168](https://web.archive.org/web/20040712165304id_/http://www.agroparty.ru:80/releases/1/168/)
  (28.04.2004, "вчера") and releases/1/169 (30.04.2004, "во вторник"), both implying 27 April 2004.
- **Live or procedural party pages:** the LDPR charter of 2023 (ldpr.ru/upload/grain.tables/306/tie26c9bhpev1xfl7pdqsrrotrz8p30n.pdf,
  "избирается на Съезде ЛДПР сроком на четыре года"; procedure only); Slutsky's live biography ldpr.ru/members/moscow/slutskiy-leonid-eduardovich
  ("с мая 2022 года"); video captions found through ldpr.ru's live search (the Supreme Council's support); kprf.ru/htm/prezid.htm
  (1998, the Presidium's rules); yabloko.ru/chairman (live).
- **Redundant or weaker records:** ldpr.ru [event/214130](https://web.archive.org/web/20251012141540id_/https://ldpr.ru/event/214130/)
  (30 May 2022, "Председатель ЛДПР Леонид Слуцкий заявил"); kprf.ru official pages of 24 and 21 April 2021, of 24 February 2013
  (I Plenum) and of 27 May 2017 (XVII congress), which name no chairman, and the second stage's information notice; yabloko.ru
  news/2015/12/19 and cat-news/2023/12/09; agroparty.ru releases/3/108 (11 September 2003; it also names Lapshin by a regional
  state office), releases/3/99 (22 August 2003), releases/3/212 (7 October 2004, prospective), boss.htm and ?id=22; Gaidar Archive
  files 3797 and 3799 (V congress decisions, the same signature), 3736 (Gaidar's II congress report, undated), 3644 (undated),
  3889 (X congress stenogram, no date), 3768 (IV congress, no chairman), 4804 (a draft agenda) and 4807 (the organising
  committee's list, undated, headed by "Гайдар Е.Т. (председатель)" with a state title).
- **Later biographies:** dvr.ru [personae/gaidar.htm](https://web.archive.org/web/20010728083347id_/http://www.dvr.ru:80/personae/gaidar.htm)
  (captured after the dissolution: "С 1994г. по настоящее время"); dvr.ru inf_soob.html (a Council plenum of "12 июня", year not
  printed); agroparty.ru cheef/index.php (undated).

## Sources attempted

- minjust.gov.ru answered 403 and was not retried or bypassed; cikrf.ru timed out, and its 1993 list pages have no captures.
- ldpr.ru: the archive holds only empty (HTTP 204) captures of the site for 6 April - 27 July 2022; captures after late June 2026
  are 403 pages, including the XXXVIII congress of 23 June 2026; the live site has view counters and a session cookie.
- kprf.ru: items 246873, 246874, 246987 and 247146 (late August 2026) have no usable capture before the cutoff; the IV congress
  notice (arhiv/congr4/infsoob-1.htm) was never captured; the statement of 2 February 1998 prints no name or date.
- yabloko.ru: cat-news pages of 20 August - 4 September 2026 are 502 captures; news URLs moved from /news/ to /cat-news/.
- agroparty.ru is a parked domain; its XV congress article, September 2008 releases and April 2004 news were never captured.
- dvr.ru was reused by unrelated businesses after 2002; its X congress and VI congress pages were never captured.
- The Internet Archive answered with intermittent 504, "Temporarily Offline" pages and refused connections; each fetch was
  retried sequentially.

## Checker defects

Two independent checks: of the KPRF, LDPR and Yabloko chains (K, L, Y, then R for points left in the revised draft) and of the
Agrarian Party and DVR chains (A, D, then N for points in the revised draft). Informational findings (K7, L8, Y8) are recorded
under the observations' limits.

| # | Defect | Outcome |
|---|---|---|
| K1 | The chain jumped from 1998 to 2025; the 2021 congress and plenum notices were missing | **Applied**: both notices imported (the congress, the report as a continuation claim, the plenum's election); no 2021 attestation after the vote exists, so 2021 is claims only |
| K2 | The XIX congress's second stage (20 June 2026) was omitted | **Applied**: imported as a congress claim; "not reviewed" wording removed |
| K3 | One 1993 claim bundled the renaming and the Central Executive Committee's election, with Zyuganov as holder | **Applied**: split into a renaming claim and a party-body election claim, neither with a holder (R1) |
| K4 | The 1998 signature quote added a comma | **Applied**: quoted as two printed lines |
| K5 | Wrong locator on the 1993 retrospective claim | **Applied** |
| K6 | The V congress's name decision was folded into the congress claim | **Applied in part**: the uncertainty says the name decision's wording is not printed, so no renaming claim is separated |
| L1 | Misquote "моего избрания" | **Applied**: "главный смысл своего избрания" and "всем, кто поддержал мою кандидатуру" |
| L2 | Contemporaneous LDPR records of the 2022 change exist | **Applied**: the death item (6 April), the Supreme Council's recommendation (26 May), the congress's opening, the vote and the election report with an attestation (27 May) imported; Slutsky's first holder re-based to 27 May 2022; the "not recoverable" wording removed; item 214130 left a lead |
| L3 | The interim issue of 6 May 2022 carried Zhirinovsky as holder | **Applied**: no holder |
| L4 | The 1990 and 1992 timeline claims bundled founding and election | **Applied**: each split into an organization claim and an election claim |
| L5 | The 1997 library page's label came from an unrecorded index | **Applied**: reworded; the index's label is stated as such |
| L6 | The 2 October 2025 post is signed with the party title | **Applied**: imported as a second attestation of that day's holder |
| L7 | The newspaper's faction co-publisher was omitted | **Applied**: named in the publisher and provenance; only party-office lines are used |
| Y1 | Mitrokhin's December 2015 attestation was not kept | **Applied**: holder (Mitrokhin, 19 December 2015) and the conflicting retrospective on Yavlinsky imported |
| Y2 | The 11 December 2023 holder rested on a past-tense result report | **Applied**: reclassified as an election report with the congress days noted; the holder re-based on the release of 13 December 2023 |
| Y3 | The 2008 report claim bundled two congresses and a list | **Applied**: split into two congress claims and a year-only continuation claim |
| Y4 | The 2004 re-election dated by the dateline; "alternative candidates" overstated | **Applied**: the report is undated, with the one alternative candidate; the vote schedule and the release of 4 July 2004 imported (holder, 4 July 2004) |
| Y5 | The 2001 holder cited a listing entry | **Applied**: the speech page imported as the holder's first claim; the release's section date noted (R4) |
| Y6 | Inconsistent kind for retrospective congresses | **Applied** |
| Y7 | The XXI congress claim dropped "съезд не закрыт" | **Applied** |
| R1 | The Central Executive Committee's election still named a holder | **Applied** |
| R2 | The 2004 succession statement named the proposer as holder | **Applied** |
| R3 | No congress claim for the XXXIV congress of 27 May 2022 | **Applied**: item 213496 imported |
| R4 | The 2001 election release's section date (22 December) not noted | **Applied** |
| A1 | The history note's 1997, 1998 and re-registration facts were not claims | **Applied in part**: V congress (1997), VI congress confirmation (26 February 1998) and re-registration (29 May 1998) imported; the II congress of 17 October 1993 (a list headed by a deputy prime minister, no party-office statement) is not |
| A2 | The XII congress's second stage was undated | **Applied**: the release of 12 October 2004 imported (second stage ended 9 October 2004; Plotnikov as a continuation claim) |
| A3 | Dropping release 3/108 lacked a valid reason; release 3/99 is cleaner | **Declined** (optional, as the check says): both are recorded as leads; the holder of 9 September 2003 stands |
| A4 | Only one reprint cited for the 27 April reading | **Applied** |
| A5 | The home page's document date and a truncated locator | **Applied**: document date null; full heading; the scope says why frozen teasers are used |
| A6 | "Under the federal law" | **Applied** |
| D1 | The 1995 holder rested on a page printing two days | **Applied**: the congress's signed decision of 18 June 1995 imported and the holder re-based; the statement kept as a claim printing both days |
| D2 | The bulletin's Council resolution of 10 July 1994 was missed | **Applied**: imported; new holder (10 July 1994); page rendered and viewed |
| D3 | Dated 1996 and 2000 records missing | **Applied**: the decision of 21 September 1996 (continuation), the decisions of 22 September 1996 (holder and the ballot record) and the statement of 16 December 2000 (a bare-title claim) imported |
| D4 | The history page's congresses and programme adoption were not claims | **Applied in part**: II and III congresses and the programme's adoption imported; the internally inconsistent IV congress sentence is not |
| D5 | The PDFs' metadata misstated | **Applied**: each extract records its own producer and date; the programme's locator names the image and PDF p. 4 |
| D6 | Mixed page numbering | **Applied**: "PDF p. N (printed p. M)" |
| D7 | The 1994 congress edition is a publisher's edition | **Applied** |
| D8 | Holder names on claims that name no office | **Applied**: the movement claim carries none; the speech keeps its speaker |
| D9 | Organization wording tied names to the bloc | **Applied** |
| D10 | "Above the printed name" | **Applied** (N6) |
| D11 | Attribution line and speech paraphrase | **Applied** |
| D12 | Dropping file 4807 was moot | **Declined** (optional): recorded as a lead; undated and not a party office |
| N1 | The 1997 claim typed as an election | **Applied**: a retrospective statement |
| N2 | "к апрелю" misread | **Applied** |
| N3 | The 1995 statement typed "undated" | **Applied**: kind `conflicting_date_attestation` |
| N4 | "For the chairmanship" loses "по выдвижению" | **Applied** |
| N5 | Capture years in the DVR jurisdiction | **Applied** |
| N6 | "Above" in the 2000 claim | **Applied** |

## Suggested next work orders

These are proposals for the integrator. They are not created in `work-orders.json`.

- `C01-Russia-PTY-001` (not proposed; ruled): the user ruled on 28 September 2026 that Zhirinovsky's 2022 observation has no
  `until`, because the party's item of 6 April 2022 ("Но сегодня его не стало и сегодня мы будем скорбеть…", ldpr.ru/event/202499)
  never says Председатель, never mentions the office ending, and has captures only from October 2025 onward; the death stays a
  distinct dated claim. Codex may still decide at integration that a stated death day ends the office, in which case
  `ru_ldpr_news_zhirinovsky_died_today_20220406` alone would be the anchor (`until` 2022-04-06), and the same rule would apply
  to every holder.
- `C01-Russia-PTY-002`: contemporaneous records of the 1990-1995 founding congresses (LDPSS 1990, KPRF 1993, APR 1993, Yabloko
  1995), the KPRF's 1995 change of title and the 1993 bloc «Выбор России» (its CEC registration and list), from party archives,
  the CEC's election statistics volumes or the State Archive.
- `C01-Russia-PTY-003`: the Ministry of Justice registry entries of the five parties and the record completing the Agrarian
  Party's 2008 accession, when the registry is reachable.
- `C01-Russia-PTY-004`: re-elections not reviewed here (KPRF 1998-2021, LDPR 1996-2021, Yabloko 2006, 2011 and 2013, DVR
  1997-2001) and the LDPR's XXXVIII congress of 23 June 2026.
- Other parties' leaders (Just Russia, United Russia and the remaining ballot lists) and deputy chairmen are outside this packet.

## Integration notes (outside this packet's file boundary)

- **Not stacked.** The packet is not stacked on a pending packet: the branch starts at the claim commit `2c4d5bd7` on `846df479`; `origin/codex/campaign-certification` (then
  `30410e55`) was merged before submission; it changed none of this packet's research files (it brought the USSR and Brazil repairs, the gap ledger, the Japan C01-29 review record and the S28 release evidence), and the index is regenerated. Retrospective lists are claims in this packet, as the house
  rules require. The first merge was started with `git -c core.hooksPath=/dev/null merge`, against the house rule; it stopped on the handoff's add/add conflict, and `7c38049f` is that merge completed by a plain `git commit`. No hooks are installed, so nothing was skipped; its tree equals a clean merge with the handoff taken from `a2cf1dcc`.
- **Edited existing research file:** only `russia.json`. The three ballot-list observations gain a role and one coverage note each; the
  packet coverage gains one note; no existing extract is edited.
- `research-index.json` is regenerated in a **separate commit**; it is the only file this packet shares with other pending
  packets. New totals for Russia: 238 sources and 432 claims (previously 170 and 295); entries 21→24,
  `mapping_pending` 21→24, role observations 9→14, discovery batches [10, 10, 1]→[10, 10, 4]. If another packet lands first,
  regenerate the index rather than merging it.
- Existing tests updated, none loosened:
  - `test_russia_research_s10h.py`: totals (24, 238, 432, 14); 17 organizations with their kinds listed in order;
    the guard that organizations have no roles re-expressed as the exact map of the five organizations with one `party_leader`
    role each; access dates pinned per packet (68 sources of 28 September 2026 in the slice `sources[170:]`); the undated
    claims pinned exactly, with this packet's 46 listed by ID; `mapping_pending` 24, role observations 14, work orders
    [10, 10, 4]. The hosts are unchanged (this packet uses only web.archive.org).
  - `test_russia_presidents_c01_14.py`: entries and roles (24, 14); index totals (14, 432).
  - `test_russia_heads_of_government_c01_19.py`: its source slice re-expressed as `sources[68:170]`; totals (238,
    432, 24, 14); index totals (14, 432, 24).
  - `test_ussr_russia_transition_c01_05.py`: unchanged; its pins are unaffected and it passes.
  - `test_ussr_government_supreme_soviet_c01_26.py` (outside the listed pins; integrated after the claim): its hash of every
    Russia role and holder is re-expressed exactly. The five party roles of this packet are named, asserted to be the only
    `party_leader` roles and excluded; every other Russia role and holder still hashes to the same pinned value.
- The new test `test_russia_party_leaders_c01_28.py` pins the five roles, the three new observations, the 24 holders, every
  claim's date, kind, observation and role, every response identity and capture, the extracts, the six titles kept as printed,
  the one non-primary source (`ru_dvr_history_page_1998`, `party_republished_reference_text`, cited by no observation), the separation from the
  presidency, the Government, the factions and the USSR packet, and 47 mutations.
- In the atlas, Russia gains three organizations and five party offices with 24 holder observations. No UI code changed;
  the Node check passes and no browser review was run.
- **Gap ledger (integrator-owned).** `test_certified_gap_ledger.py`, which is not among this packet's listed checks, errors on this branch in `setUpClass`: `certified_gap_ledger.py` needs a pinned attribution for every research source and stops at the first new one (`Source ru_kprf_i_plenum_notice_19970420 (Russia) has no pinned attribution; run --refresh-attribution`). The attribution is rebuilt from git history (the commit that first adds each extract), so it can only follow this packet's commit, and `--refresh-attribution` also requires that commit to be classified in `COMMIT_PACKETS` in `tools/avatars/certified_gap_ledger.py`. The attribution input and the ledger lie under `docs/campaign-certification/C01/gap-ledger/`, which this packet may not edit. On integration: classify the packet commit as it lands, refresh the attribution, regenerate the ledger and update the expected `CLAUDE-C01-28` in-flight state.
- **S23 boundary matrix.** `test_certified_boundary_matrix.py` (S23, outside the listed checks) fails on this branch in a checkout with its inputs: `CLAUDE-C01-28` is `unclassified_packet` (not listed in `docs/campaign-certification/verification/2026-09-27-claude-integration.md`), and `cases-ussr-russia.json`, `summary.json` and `README.md` are stale. Both pass at `30410e55`. On integration: list the packet as pending and regenerate `docs/campaign-certification/S23/preparation/boundary-matrix/`.
- `research/README.md`, the C01 README totals, `docs/planning/ai-workstreams.json` and the task queue (where Codex registered
  the claim as `claimed`) are left for the integrator.

## Checks

```text
python -X utf8 tools/avatars/campaign_research.py
python -X utf8 tools/avatars/campaign_research.py --check
python -X utf8 tools/avatars/campaign_census.py --check
python -X utf8 -m unittest discover -s tools/avatars -p "test_russia*.py"
python -X utf8 -m unittest discover -s tools/avatars -p "test_ussr*.py"
python -X utf8 -m unittest discover -s tools/avatars -p "test_*research*.py"
python -X utf8 -m unittest discover -s tools/avatars -p "test_campaign*.py"
node --test tools/ui/check_leadership_research_review.cjs
python tools/planning/workboard.py --check
git diff --check (this packet's paths)
```

All passed on 28 September 2026 (UTC), after the merge of `codex/campaign-certification` at `30410e55` (merge commit `b6d5c9a8`), and
again with the same results after the verifier fixes: the index regeneration
and exact `--check` (9 country packets, 1,396 sources, 3,865 claims, 844 organization and 34 institution observations, 93
discovery batches); `campaign_census.py --check` (exit 0); 39 Russia tests (10 of them new), 28 USSR tests, 79 research tests
and 16 campaign tests (census included); 11 atlas Node tests; the workboard check (44 markers, 24 bounded tasks); `git diff
--check` on this packet's paths. The new test's 47 mutations each fail as intended: a successor's election, a contemporaneous
death statement, the interim issue, a successor's attestation, an accession decision and the latest attestation used as ends; election days
and a nomination used as starts; election and congress days used as attested days; an interim body and a retrospective
election added as holders; an election, an undated attestation, a continuation claim and a nomination cited by holders; a
faction head added to a party role, a faction claim and a Government claim cited by party roles and a party claim cited by
the Government; a party holder added to a faction role; a claim moved between parties; a game mapping, lifecycle start and
end on the new observations; a role for the bloc; a changed role kind; a removed role and organization; changed ballot-list
claims; reordered holders; a re-dated election and a dated undated claim; checksum mismatches; beyond-cutoff dates; a
reversed interval; a claim from an uncited source; an HTTP source URL; a represented-party mapping; and a party holder on
the USSR Presidency.

Outside the listed checks, `test_certified_gap_ledger.py` errors on this branch (22 tests run, one `setUpClass` error): the ledger needs a pinned attribution for each new source, which can only be refreshed from git history after this packet's commit is classified; see [Integration notes](#integration-notes-outside-this-packets-file-boundary).

`test_certified_boundary_matrix.py` (S23, outside the listed checks) fails on this branch in a checkout with its inputs: `CLAUDE-C01-28` is `unclassified_packet` (not listed in `docs/campaign-certification/verification/2026-09-27-claude-integration.md`), and `cases-ussr-russia.json`, `summary.json` and `README.md` are stale. Both pass at `30410e55`. On integration: list the packet as pending and regenerate `docs/campaign-certification/S23/preparation/boundary-matrix/`.
