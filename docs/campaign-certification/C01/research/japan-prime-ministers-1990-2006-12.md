# Japanese prime ministers 12: holders and transitions, 1990-2006

Packet: **CLAUDE-C01-12**. State: **ready_for_review** (not complete).
Owner: Claude. Integrator/reviewer: Codex. Branch `claude/c01-jp-12`, based on `codex/campaign-certification`
at `ffe54b02`; claim commit `c2aafd22`. Research access: 24 September 2026 (UTC). The historical cutoff stays
**7 September 2026**.

This packet adds the prime-ministership to [japan.json](japan.json), which until now held 16 election-list organizations,
7 parliamentary-group institutions, 4 party-office roles and 7 sources, and no executive office. It reviews ten
observations from 1 January 1990 to the Koizumi cabinet's resignation on 26 September 2006: the Prime Minister when the
period opens and his re-designation after the February 1990 general election, the 1991 and 1993 transitions, Hata and
Murayama in 1994, Hashimoto's two appointments of 1996, Obuchi in 1998, Obuchi's incapacity with the acting Prime
Minister and Mori's two appointments in 2000, Koizumi's three appointments of 2001, 2003 and 2005, and the 2006
resignation with the successor's appointment as the closing boundary. It adds 119 sources and 174 claims,
one institution (`jp_prime_minister`, an `executive_institution` with lifecycle `unknown`) with one role (`jp_pm`,
"内閣総理大臣 — Prime Minister of Japan", kind `head_of_government`), fourteen holder observations, a role scope note,
institution coverage and a bounded note in the packet coverage. It adds no organization, parliamentary group, party
role, game mapping, lifespan, portrait or avatar, and it changes none of the 23 existing observations, the four party
roles (the LDP presidency observations included) or the seven original sources. Holders from 26 September 2006 to the
cutoff are outside this packet. The parent scope (C01, C06, S23, WC1 and CP1) remains open.

The research was done in three parts (A: 1990-1993; B: 1994-1998; C: 2000-2006), and each part was checked
independently before this packet was written. Every checker defect is applied (see [Checker defects](#checker-defects)),
and every missing primary record the checks confirmed is imported.

## Outcome

| ID | Question | Decision |
|---|---|---|
| JP-PM-01 | The holder when the period opens (海部俊樹), and his re-designation and new cabinet after the February 1990 general election | **Accepted in part:** observed on 19 Jan 1990 signing a written answer (an acting Prime Minister signed one on 12 Jan); the resignation notice and both Houses' designations of 27 Feb 1990, with no joint committee; observed again on 2 Mar 1990 after his reappointment; no appointment day and no end found |
| JP-PM-02 | 1991: the Kaifu cabinet's resignation en masse and 宮澤喜一's designation and appointment | **Accepted in part:** notice received on 5 Nov 1991 (House of Representatives 09:22, House of Councillors 09:25) and both Houses' designations that day; observed on 8 Nov 1991; no appointment day and no end found |
| JP-PM-03 | 1993: the Miyazawa cabinet's resignation and 細川護煕's designation and appointment | **Accepted in part:** notice received on 5 Aug 1993; both designations on 6 Aug; observed on 23 Aug 1993; later remarks recall his acts on 9 and 13 Aug (claims); no appointment day and no end found |
| JP-PM-04 | 1994: 羽田孜's designation and appointment | **Accepted in part:** the Hosokawa cabinet's notice and both designations on 25 Apr 1994; observed on 10 May 1994; the 28 Apr formation rests on retrospective claims |
| JP-PM-05 | 1994: 村山富市's designation and appointment | **Accepted in part:** the Hata cabinet decided to resign on 25 Jun 1994 (House of Councillors receipt 11:56); designated on 29 Jun (the House of Representatives in a runoff); observed on 18 Jul 1994; 30 Jun rests on his own recollection and the Kantei list |
| JP-PM-06 | 1996: 橋本龍太郎's designation and appointment, and his November re-designation | **Accepted in part:** from 11 Jan 1996 (a Kantei statement titled with his name); re-designated on 7 Nov 1996, when the statement of assumption prints no name, so the second holder is observed on 8 Nov 1996; no end |
| JP-PM-07 | 1998: 小渕恵三's designation and appointment | **Accepted in part:** the Houses designated 小渕恵三 and 菅直人; the joint committee reached no agreement and the House of Representatives' designation prevailed under art. 67(2) on 30 Jul 1998; observed on 31 Jul 1998 |
| JP-PM-08 | 2000: Obuchi's incapacity, the acting Prime Minister, the resignation and 森喜朗's two appointments | **Accepted:** 青木幹雄 acting Prime Minister from 09:00 on 3 Apr 2000 (claims only); the art. 70 resignation decided on 4 Apr; 森喜朗 from 5 Apr 2000 and from 4 Jul 2000 |
| JP-PM-09 | 2001-2005: the Mori cabinet's resignation and 小泉純一郎's three appointments | **Accepted:** from 26 Apr 2001, 19 Nov 2003 and 21 Sep 2005, each from a named same-day Kantei account of the Imperial appointment ceremony (and in 2005 the Imperial Household Agency's photo page) |
| JP-PM-10 | 2006: the Koizumi cabinet's resignation, the day the office passed and the successor's appointment as the closing boundary | **Accepted in part:** in office on 25 Sep 2006; resignation notices on 26 Sep; 安倍晋三 designated and appointed that day (the closing boundary, never an end); no source states the day Koizumi's office ended |

The resulting holder observations of `jp_pm`, in date order:

| Holder | `attested_on` | `from` | `until` | Basis |
|---|---|---|---|---|
| 海部俊樹 | 1990-01-19 | null | null | written answers signed 内閣総理大臣 海部俊樹, House of Councillors 19 Jan and House of Representatives 23 Jan 1990 |
| 海部俊樹 | 1990-03-02 | null | null | policy speeches of 2 Mar 1990 in both Houses: 再び内閣総理大臣に任命され (no date) |
| 宮澤喜一 | 1991-11-08 | null | null | policy speeches of 8 Nov 1991 in both Houses: 内閣総理大臣に任命され (no date) |
| 細川護煕 | 1993-08-23 | null | null | policy speeches of 23 Aug 1993 in both Houses: 内閣総理大臣に任命され (no date) |
| 羽田孜 | 1994-05-10 | null | null | policy speeches of 10 May 1994 in both Houses: 内閣総理大臣に任命されました (no date) |
| 村山富市 | 1994-07-18 | null | null | speaks as 内閣総理大臣 in the policy speech of 18 Jul 1994 |
| 橋本龍太郎 | null | 1996-01-11 | null | Kantei statement 橋本内閣総理大臣談話 of 11 Jan 1996: 本日、内閣総理大臣の重責を担うことになりました |
| 橋本龍太郎 | 1996-11-08 | null | null | press conference of 8 Nov 1996 (the 7 Nov statement of assumption prints no name) |
| 小渕恵三 | 1998-07-31 | null | null | press conference record 小渕内閣総理大臣記者会見録, 31 Jul 1998 |
| 森喜朗 | null | 2000-04-05 | null | Kantei statement (本日、内閣総理大臣に任命され), Kantei account of the 親任式 that night, profile entry |
| 森喜朗 | null | 2000-07-04 | null | Kantei account 第２次森内閣発足: 森総理 appointed at the 親任式 that night |
| 小泉純一郎 | null | 2001-04-26 | null | Kantei account 小泉内閣発足: 小泉純一郎 attended the Prime Minister's 親任式 that night |
| 小泉純一郎 | null | 2003-11-19 | null | Kantei account (appointed at the 親任式 as the 88th Prime Minister) and his statement (本日、再び…重責を担う) |
| 小泉純一郎 | null | 2005-09-21 | null | Kantei account (89th Prime Minister), his statement (本日、三度…), Imperial Household Agency photo page 親任式（小泉内閣総理大臣）; in office 25 Sep 2006 |

### How a start and an end are decided

One rule covers all three parts. A holder has `from` only where a same-day source that names the holder states the day of
appointment or assumption of office, and `until` only where a source states the day the office ended; otherwise the
holder is dated by `attested_on`, the earliest same-day act in office found on a byte-stable record. For 1990-1998 no
record of an appointment was accessible: the Official Gazette (官報) of those years is not available without
restriction, the Imperial Household Agency's website begins in late 1999, and the Diet minutes never mention the
appointment ceremony (親任式). Those holders are dated by their own acts in office: signed written answers, policy speeches
in which each says he has been appointed (任命され) without giving a day, and press conferences. From 1996 the Kantei
published the Prime Minister's statement (内閣総理大臣談話) of the day, from 2000 its accounts of the ceremony, and from
2000 the Imperial Household Agency's schedules date the ceremony. These give the six stated starts.

Three refinements apply the same way to every part:

- **A statement or record that prints no name never makes a start by itself.** It is a claim on the role whose row
  carries no holder name. The Kantei statement of 7 November 1996 ("本日、再び内閣総理大臣の重責を担うこととなりました")
  prints no name, and the only record identifying its speaker is the next day's press conference, so the second
  橋本龍太郎 holder is observed on 8 November 1996 (check B6, the second option). The unnamed statements of 4 July 2000
  and 26 April 2001 and the Imperial Household Agency's schedule rows, which name no appointee, corroborate starts that a
  named same-day Kantei account already states, and each holder's note cites them.
- **A spoken "today" that its own record contradicts is not an assumption statement.** Obuchi told the press on
  31 July 1998 that "today" he had come to bear responsibility as Prime Minister, and in the same opening remarks that the
  new cabinet was launched the previous day (check B8). The written statements used for starts (11 January 1996, 5 April
  2000, 19 November 2003 and 21 September 2005) are dated, issued on the day and name no other day.
- **"このたび…任命され" in a policy speech states an appointment without a day;** it dates an observation, never a start.

Proposed ends that are refused:

- **A notice or record of a cabinet's resignation en masse (総辞職).** Constitution art. 71 (procedure only) keeps the
  outgoing cabinet performing its functions until a new Prime Minister is appointed, but no source reviewed states that
  any outgoing Prime Minister did so, so no continuation claim is recorded and no end is dated from a resignation. Diet
  searches for 職務執行内閣, 七十一条 and 引き続き職務 around every transition found nothing.
- **A successor's designation, appointment or ceremony, or the same person's re-designation and reappointment.** None is
  used as an end; no source in this packet states the day any office ended, so no holder has `until`.
- **Retrospective lists and spans.** The Kantei's 歴代内閣 pages and cabinet lists and the House of Councillors'
  compiled table give formation days and spans; they are claims with no structured date and never a boundary.
- **Obuchi's incapacity.** The cabinet judged on 4 April 2000 that he was "lacking" (欠けたとき) under art. 70 and
  resigned en masse; that reading is the cabinet's, and no source states the day his office ended.

Two acting prime ministers (臨時代理) appear: 橋本龍太郎 signs a written answer of 12 January 1990 (the reason and span are
not stated), and 青木幹雄 served from 09:00 on 3 April 2000. Both are claims only, never holders, and their rows carry no
holder name.

Names follow the sources in Japanese, with one holder string per person. The House of Councillors' table and the Houses'
written answers of August-September 1993 print 細川護熙 (熙, U+7199); the Diet minutes and the Kantei print 細川護煕
(煕, U+7155), the form used (check A4). Where a source prints only a surname or a title (森総理, 小泉総理,
青木内閣総理大臣臨時代理), the claim text keeps that form. A row's `holder_name` is the holder's full name when the source
names that holder, and null when it names no holder of this role in this packet: an unnamed statement, agenda or
ceremony record, an acting Prime Minister, the House of Councillors' 1998 designee 菅直人 or the successor 安倍晋三.
Romanized forms: 海部俊樹 Kaifu Toshiki, 宮澤喜一 Miyazawa Kiichi, 細川護煕 Hosokawa Morihiro, 羽田孜 Hata Tsutomu,
村山富市 Murayama Tomiichi, 橋本龍太郎 Hashimoto Ryūtarō, 小渕恵三 Obuchi Keizō, 森喜朗 Mori Yoshirō and 小泉純一郎
Koizumi Jun'ichirō.

### Date ledger

Each row is a separate dated fact with its own claim; two facts on one day stay two claims.

| Date | Event | Claim or field |
|---|---|---|
| 12 Jan 1990 | Written answer signed by the acting Prime Minister (臨時代理) 橋本龍太郎 | `jp_hashimoto_acting_pm_signs_hr_answer_19900112` |
| 19 Jan 1990 | Written answer signed 内閣総理大臣 海部俊樹 (House of Councillors) | `jp_kaifu_signs_hc_written_answer_as_pm_19900119` (海部俊樹 `attested_on`) |
| 23 Jan 1990 | Written answer signed 内閣総理大臣 海部俊樹 (House of Representatives) | `jp_kaifu_signs_hr_written_answer_as_pm_19900123` (海部俊樹 (support)) |
| 27 Feb 1990 | Notice of the Kaifu cabinet's resignation en masse received by the House of Councillors (09:45); both Houses designate 海部俊樹 (House of Councillors in a runoff) | `jp_kaifu_notifies_cabinet_resignation_hc_19900227`, `jp_hr_designates_kaifu_19900227`, `jp_hc_designates_kaifu_runoff_19900227` |
| 28 Feb 1990 | Second Kaifu cabinet formed, recalled by a member on 9 April 1990 | `jp_member_states_second_kaifu_cabinet_formed_19900228` |
| 2 Mar 1990 | Kaifu tells both Houses he has been appointed again | `jp_kaifu_policy_speech_states_reappointment_19900302` (海部俊樹 `attested_on`), `jp_kaifu_hc_policy_speech_states_reappointment_19900302` (海部俊樹 `attested_on`) |
| 5 Nov 1991 | Notice of the Kaifu cabinet's resignation en masse (House of Representatives 09:22, House of Councillors 09:25; reported in both plenaries); both Houses designate 宮澤喜一; Miyazawa cabinet launched, recalled by a minister on 17 December 1991 | `jp_kaifu_notifies_cabinet_resignation_hr_19911105`, `jp_hc_receives_kaifu_resignation_notice_19911105`, `jp_hr_speaker_reports_kaifu_resignation_notice_19911105`, `jp_hr_designates_miyazawa_19911105`, `jp_hc_president_reports_kaifu_resignation_notice_19911105`, `jp_hc_designates_miyazawa_runoff_19911105`, `jp_minister_states_miyazawa_cabinet_formed_19911105` |
| 8 Nov 1991 | Miyazawa tells both Houses he has been appointed | `jp_miyazawa_policy_speech_states_appointment_19911108` (宮澤喜一 `attested_on`), `jp_miyazawa_hc_policy_speech_states_appointment_19911108` (宮澤喜一 `attested_on`) |
| 5 Aug 1993 | Notice of the Miyazawa cabinet's resignation en masse received by the House of Councillors (09:53; reported on 6 August) | `jp_miyazawa_notifies_cabinet_resignation_19930805` |
| 6 Aug 1993 | Both Houses designate 細川護煕 (the House of Representatives after a repeated roll call) | `jp_hr_designates_hosokawa_19930806`, `jp_hc_designates_hosokawa_19930806` |
| 9 Aug 1993 | Hosokawa cabinet comes into being and the Prime Minister gives a minister a special instruction, both recalled by the minister on 24 August 1993 | `jp_minister_states_hosokawa_cabinet_formed_19930809`, `jp_minister_recalls_pm_instruction_19930809` |
| 13 Aug 1993 | Prime Minister Hosokawa visits the Kagoshima disaster area, recalled on 24 August 1993 | `jp_member_recalls_pm_disaster_visit_19930813` |
| 23 Aug 1993 | Hosokawa tells both Houses he has been appointed | `jp_hosokawa_policy_speech_states_appointment_19930823` (細川護煕 `attested_on`), `jp_hosokawa_hc_policy_speech_states_appointment_19930823` (細川護煕 `attested_on`) |
| 25 Apr 1994 | Notice of the Hosokawa cabinet's resignation en masse read in both Houses; both designate 羽田孜 | `jp_hr_receives_hosokawa_cabinet_resignation_notice_19940425`, `jp_hr_designates_hata_19940425`, `jp_hc_hosokawa_cabinet_resolves_resignation_19940425`, `jp_hc_designates_hata_19940425` |
| 28 Apr 1994 | Hata cabinet launched, recalled by a member on 27 May 1994 | `jp_member_recalls_hata_cabinet_launch_19940428` |
| 10 May 1994 | Hata tells both Houses he has been appointed | `jp_hata_states_appointed_pm_19940510` (羽田孜 `attested_on`), `jp_hata_hr_policy_speech_states_appointment_19940510` (羽田孜 `attested_on`) |
| 25 Jun 1994 | Hata cabinet decides to resign en masse (House of Councillors receipt 11:56; read on 29 June) | `jp_hc_receives_hata_resignation_notice_19940625`, `jp_hr_receives_hata_cabinet_resignation_notice_19940625`, `jp_hc_hata_cabinet_resolves_resignation_19940625` |
| 29 Jun 1994 | House of Representatives: first ballot without a majority, then runoff designating 村山富市; House of Councillors designates 村山富市 | `jp_hr_first_ballot_no_majority_19940629`, `jp_hr_designates_murayama_runoff_19940629`, `jp_hc_designates_murayama_19940629` |
| 30 Jun 1994 | Murayama forms the cabinet, recalled by himself on 9 November 1994 | `jp_murayama_recalls_cabinet_formed_19940630` |
| 6 Jul 1994 | Members refer to the Murayama cabinet and 村山総理 in the agriculture committees of both Houses | `jp_hc_member_refers_to_murayama_cabinet_19940706`, `jp_hr_member_refers_to_murayama_cabinet_19940706`, `jp_hr_member_refers_to_prime_minister_murayama_19940706` |
| 18 Jul 1994 | Murayama speaks as 内閣総理大臣 in his policy speech | `jp_murayama_speaks_as_pm_19940718` (村山富市 `attested_on`) |
| 11 Jan 1996 | Notice of the Murayama cabinet's resignation en masse in both Houses; both designate 橋本龍太郎; Kantei statement 橋本内閣総理大臣談話 that he took on the office that day; first cabinet meeting (説示); Hashimoto's own recollection of 8 November 1996 | `jp_hr_receives_murayama_cabinet_resignation_notice_19960111`, `jp_hr_designates_hashimoto_19960111`, `jp_hc_murayama_cabinet_resolves_resignation_19960111`, `jp_hc_designates_hashimoto_19960111`, `jp_kantei_hashimoto_assumes_office_19960111` (橋本龍太郎 `from`), `jp_kantei_first_cabinet_meeting_19960111`, `jp_kantei_hashimoto_recalls_appointment_19960111` |
| 7 Nov 1996 | Notice of the first Hashimoto cabinet's resignation en masse received by the House of Councillors (09:17); Kantei agenda (extraordinary cabinet meeting on the resignation; the Prime Minister's statement); both Houses re-designate 橋本龍太郎; unnamed Kantei statement that the Prime Minister took on the office again; Hashimoto's next-day recollection | `jp_hc_receives_hashimoto_cabinet_resignation_notice_19961107`, `jp_kantei_extraordinary_cabinet_resignation_item_19961107`, `jp_kantei_cabinet_statement_item_19961107`, `jp_hr_designates_hashimoto_19961107`, `jp_hc_designates_hashimoto_19961107`, `jp_kantei_pm_assumes_office_again_19961107`, `jp_kantei_hashimoto_designated_again_continues_19961107` |
| 8 Nov 1996 | Press conference of 橋本総理 after forming the second Hashimoto cabinet | `jp_kantei_hashimoto_press_conference_in_office_19961108` (橋本龍太郎 `attested_on`) |
| 30 Jul 1998 | Notice of the Hashimoto cabinet's resignation en masse in both Houses; Kantei agenda and Hashimoto's statement; House of Representatives designates 小渕恵三, House of Councillors 菅直人 (after a first ballot without a majority); joint committee without agreement; art. 67(2) declared in both Houses; dated photo caption 小渕内閣; Obuchi's next-day recollection of the cabinet's launch | `jp_hr_receives_hashimoto_cabinet_resignation_notice_19980730`, `jp_hr_designates_obuchi_19980730`, `jp_hr_joint_committee_requested_19980730`, `jp_hr_resolution_prevails_obuchi_19980730`, `jp_hc_hashimoto_cabinet_resolves_resignation_19980730`, `jp_hc_first_ballot_no_majority_19980730`, `jp_hc_designates_kan_runoff_19980730`, `jp_hc_hr_resolution_prevails_19980730`, `jp_joint_committee_no_agreement_19980730`, `jp_kantei_extraordinary_cabinet_resignation_item_19980730`, `jp_kantei_hashimoto_cabinet_resigned_19980730`, `jp_kantei_diary_obuchi_cabinet_19980730`, `jp_kantei_obuchi_new_cabinet_launched_19980730` |
| 31 Jul 1998 | Kantei 初閣議 agenda; unnamed statement ('この度'); press conference 小渕内閣総理大臣記者会見録 | `jp_kantei_first_cabinet_meeting_19980731`, `jp_kantei_obuchi_statement_bears_office_19980731`, `jp_kantei_obuchi_press_conference_in_office_19980731` (小渕恵三 `attested_on`) |
| 2 Apr 2000 | Obuchi admitted to hospital, moved to intensive care and in a coma; his 19:00 instruction to 青木 (both recounted later) | `jp_obuchi_hospitalised_and_comatose_20000402`, `jp_obuchi_instruction_to_aoki_1900_20000402` |
| 3 Apr 2000 | 青木幹雄 acting Prime Minister from 09:00 (question and answer in the House of Councillors that day; later statements); acting designation reported at the 12:40 cabinet meeting and notified to both Speakers | `jp_aoki_acting_pm_from_0900_announced_20000403`, `jp_aoki_reports_acting_designation_notified_20000403`, `jp_member_asks_aoki_acting_pm_0900_20000403`, `jp_aoki_takes_acting_pm_0900_20000403`, `jp_aoki_states_acting_pm_legally_from_0900_20000403`, `jp_aoki_commenced_acting_duties_0900_20000403`, `jp_acting_designation_notice_sent_20000403` |
| 4 Apr 2000 | Cabinet judges the art. 70 case and decides to resign en masse at 19:00; notices signed by the acting Prime Minister (House of Councillors receipt 19:26) | `jp_obuchi_cabinet_resignation_decided_20000404`, `jp_obuchi_cabinet_resignation_art70_20000404`, `jp_sangiin_receives_resignation_notice_from_acting_pm_20000404`, `jp_shugiin_rules_reads_acting_pm_resignation_notice_20000404`, `jp_shugiin_receives_resignation_notice_from_acting_pm_20000404`, `jp_sangiin_notice_cabinet_resigns_art70_20000404` |
| 5 Apr 2000 | Both Houses designate 森喜朗; Kantei statement that he was appointed that day and account of the ceremony that night; Imperial Household Agency schedule records the 親任式; profile entry; cabinet launched | `jp_shugiin_designates_mori_20000405`, `jp_sangiin_designates_mori_20000405`, `jp_mori_appointed_pm_statement_20000405` (森喜朗 `from`), `jp_mori_shinninshiki_appointed_20000405` (森喜朗 `from`), `jp_mori_cabinet_formed_20000405`, `jp_kunaicho_ceremony_20000405`, `jp_mori_profile_pm_career_entry_20000405` (森喜朗 `from`) |
| 4 Jul 2000 | Notice of the Mori cabinet's resignation en masse received by the House of Councillors (09:25); both Houses re-designate 森喜朗; unnamed statement; Kantei account of the ceremony; Imperial Household Agency schedule; second cabinet launched | `jp_sangiin_receives_mori_cabinet_resignation_notice_20000704`, `jp_shugiin_designates_mori_20000704`, `jp_sangiin_designates_mori_20000704`, `jp_mori_resumes_pm_statement_20000704`, `jp_mori_shinninshiki_appointed_20000704` (森喜朗 `from`), `jp_mori_second_cabinet_formed_20000704`, `jp_kunaicho_ceremony_20000704` |
| 26 Apr 2001 | Notice of the Mori cabinet's resignation en masse (House of Representatives 09:21, House of Councillors 09:23; read in both plenaries); Mori's statement; both Houses designate 小泉純一郎; unnamed appointment statement; Kantei account of the ceremony; Imperial Household Agency schedule; cabinet launched | `jp_shugiin_rules_receives_mori_resignation_notice_20010426`, `jp_sangiin_rules_receives_mori_resignation_notice_20010426`, `jp_shugiin_receives_mori_cabinet_resignation_notice_20010426`, `jp_shugiin_designates_koizumi_20010426`, `jp_sangiin_notice_mori_cabinet_resigns_20010426`, `jp_sangiin_designates_koizumi_20010426`, `jp_mori_cabinet_resigned_statement_20010426`, `jp_koizumi_appointed_pm_statement_20010426`, `jp_koizumi_shinninshiki_20010426` (小泉純一郎 `from`), `jp_koizumi_cabinet_formed_20010426`, `jp_kunaicho_ceremony_20010426` |
| 19 Nov 2003 | Notice of the Koizumi cabinet's resignation en masse received by the House of Councillors (09:16); both Houses re-designate 小泉純一郎; his statement; Kantei account of the ceremony; Imperial Household Agency schedule; second cabinet launched | `jp_sangiin_receives_koizumi_cabinet_resignation_notice_20031119`, `jp_shugiin_designates_koizumi_20031119`, `jp_sangiin_designates_koizumi_20031119`, `jp_koizumi_resumes_pm_statement_20031119` (小泉純一郎 `from`), `jp_koizumi_shinninshiki_appointed_20031119` (小泉純一郎 `from`), `jp_koizumi_second_cabinet_formed_20031119`, `jp_kunaicho_ceremony_20031119` |
| 21 Sep 2005 | Notice received by the House of Councillors (09:14); cabinet resigns at a morning meeting; both Houses re-designate 小泉純一郎; his statement; Kantei account of the ceremony; Imperial Household Agency schedule and photo page naming 小泉内閣総理大臣; third cabinet launched | `jp_sangiin_receives_koizumi_cabinet_resignation_notice_20050921`, `jp_shugiin_designates_koizumi_20050921`, `jp_sangiin_designates_koizumi_20050921`, `jp_koizumi_resumes_pm_statement_20050921` (小泉純一郎 `from`), `jp_koizumi_cabinet_resigned_morning_20050921`, `jp_koizumi_shinninshiki_appointed_20050921` (小泉純一郎 `from`), `jp_koizumi_third_cabinet_formed_20050921`, `jp_kunaicho_ceremony_20050921`, `jp_kunaicho_ceremony_koizumi_20050921` (小泉純一郎 `from`) |
| 25 Sep 2006 | Koizumi gives the last interview of his term at the Prime Minister's Office | `jp_koizumi_pm_final_interview_20060925` (小泉純一郎 (support)) |
| 26 Sep 2006 | Notice of the Koizumi cabinet's resignation en masse (House of Representatives 09:16, House of Councillors 09:19; read in both plenaries); Koizumi's statement; both Houses designate 安倍晋三; his appointment statement, the Kantei account and the Imperial Household Agency's schedule and photo page of the ceremony (the closing boundary) | `jp_shugiin_rules_receives_koizumi_resignation_notice_20060926`, `jp_sangiin_rules_receives_koizumi_resignation_notice_20060926`, `jp_shugiin_receives_koizumi_cabinet_resignation_notice_20060926`, `jp_shugiin_designates_abe_20060926`, `jp_sangiin_notice_koizumi_cabinet_resigns_20060926`, `jp_sangiin_designates_abe_20060926`, `jp_koizumi_cabinet_resigned_statement_20060926`, `jp_abe_appointed_pm_statement_20060926`, `jp_abe_shinninshiki_20060926`, `jp_abe_cabinet_formed_20060926`, `jp_kunaicho_ceremony_20060926`, `jp_kunaicho_ceremony_abe_20060926` |
| undated | Kantei 歴代内閣 spans and cabinet lists, the House of Councillors' table and one month-precision recollection | 30 claims with no structured date: `jp_kantei_first_kaifu_span_19890810_19900228`, `jp_hc_precedents_hc_designates_kaifu_19900227`, `jp_hc_precedents_hr_designates_kaifu_19900227`, `jp_hc_precedents_kaifu_cabinet_resignation_19900227`, `jp_hc_precedents_second_kaifu_cabinet_formed_19900228`, `jp_hc_precedents_hc_designates_miyazawa_19911105`, `jp_hc_precedents_hr_designates_miyazawa_19911105`, `jp_hc_precedents_kaifu_cabinet_resignation_19911105`, `jp_hc_precedents_miyazawa_cabinet_formed_19911105`, `jp_hc_precedents_hc_designates_hosokawa_19930806`, `jp_hc_precedents_hr_designates_hosokawa_19930806`, `jp_hc_precedents_miyazawa_cabinet_resignation_19930805`, `jp_hc_precedents_hosokawa_cabinet_formed_19930809`, `jp_kantei_second_kaifu_cabinet_formed_19900228`, `jp_kantei_second_kaifu_span_19900228_19911105`, `jp_kantei_miyazawa_cabinet_formed_19911105`, `jp_kantei_miyazawa_span_19911105_19930809`, `jp_kantei_hosokawa_cabinet_formed_19930809`, `jp_kantei_hosokawa_span_19930809_19940428`, `jp_kantei_list_hata_cabinet_formed_19940428`, `jp_kantei_list_murayama_cabinet_formed_19940630`, `jp_kantei_list_hashimoto_cabinet_1_formed_19960111`, `jp_kantei_list_hashimoto_cabinet_2_formed_19961107`, `jp_kantei_hashimoto_recalls_taking_office_199601`, `jp_kantei_list_obuchi_cabinet_formed_19980730`, `jp_kantei_span_mori_85_20000405_20000704`, `jp_kantei_span_mori_86_20000704_20010426`, `jp_kantei_span_koizumi_87_20010426_20031119`, `jp_kantei_span_koizumi_88_20031119_20050921`, `jp_kantei_span_koizumi_89_20050921_20060926` |

Date conventions follow CLAUDE-C01-07 to C01-11: `attested_on` is the day of the observed event as the source dates it
(a notice by the day it was received or decided, a recollection by the day it recalls). No page's issue date is stored:
`published_date` is null for every new source, because a statement's dateline is not its publication (the 1996 statement's
original Last-Modified is six days later). Retrospective Kantei pages and the House of Councillors' compiled table carry no
structured date (check B3, applied to every retrospective list), nor does Hashimoto's month-precision recollection
(check B2). Clock times are kept in the text only.

## Observations

### JP-PM-01 — The holder when the period opens, and the 1990 re-designation

Evidence: the Cabinet's written answer No. 1 of the 117th session to the House of Councillors, dated 19 January 1990, is
signed 内閣総理大臣 海部俊樹 (`jp_kaifu_signs_hc_written_answer_as_pm_19900119`; a scan, PDF page 1 rendered), and answer
No. 2 to the House of Representatives of 23 January 1990 likewise (`jp_kaifu_signs_hr_written_answer_as_pm_19900123`).
An answer of 12 January 1990 is signed by 橋本龍太郎 as acting Prime Minister (内閣総理大臣臨時代理)
(`jp_hashimoto_acting_pm_signs_hr_answer_19900112`; check A5). The Kantei's retrospective page gives the 76th Prime
Minister's period as 平成元年8月10日～平成2年2月28日 (`jp_kantei_first_kaifu_span_19890810_19900228`; checks A6 and A7).
On 27 February 1990, the day the 118th special session convened, the House of Councillors received at 09:45 the notice,
addressed to its President, of the Cabinet's decision to resign en masse (`jp_kaifu_notifies_cabinet_resignation_hc_19900227`;
check A2); the House of Representatives designated 海部俊樹 with 286 of 508 votes (`jp_hr_designates_kaifu_19900227`) and
the House of Councillors in a runoff, 111 to 91 with 44 blank (`jp_hc_designates_kaifu_runoff_19900227`), so no joint
committee arose. The House of Councillors' table records both designations in separate columns, the resignation
decision and 2.2.28 第二次海部内閣成立 (`jp_hc_precedents_hc_designates_kaifu_19900227`,
`jp_hc_precedents_hr_designates_kaifu_19900227`, `jp_hc_precedents_kaifu_cabinet_resignation_19900227`,
`jp_hc_precedents_second_kaifu_cabinet_formed_19900228`; check A3), as do the Kantei's 77th page
(`jp_kantei_second_kaifu_cabinet_formed_19900228`) and a member questioning a minister on 9 April 1990
(`jp_member_states_second_kaifu_cabinet_formed_19900228`; check A1). On 2 March 1990 Kaifu told both Houses that after the
election he had been appointed Prime Minister again (`jp_kaifu_policy_speech_states_reappointment_19900302`,
`jp_kaifu_hc_policy_speech_states_reappointment_19900302`).

Decision: accepted in part. The first holder is observed on 19 January 1990, the earliest 1990 attestation pinned to a
byte-stable response, and the second on 2 March 1990. Neither has a start or an end.

Limits: no record dated 1 January 1990 itself (Kaifu is attested on 13 December 1989, a lead); no appointment instrument
or ceremony record of 1990; no source states the day the first appointment ended or that Kaifu performed the duties
between 27 February and his reappointment; the reason and span of the 12 January acting designation are not stated.

### JP-PM-02 — 1991: Kaifu to Miyazawa

Evidence: on 5 November 1991, the day the 122nd extraordinary session convened, the House of Representatives' Committee
on Rules and Administration heard that at 09:22 Kaifu (海部内閣総理大臣) had notified the Speaker in writing that the
Cabinet had decided to resign en masse that day (`jp_kaifu_notifies_cabinet_resignation_hr_19911105`), and the House of
Councillors' committee that the House had received the notice at 09:25 (`jp_hc_receives_kaifu_resignation_notice_19911105`,
a record added by the part A check); both presiding officers reported it (`jp_hr_speaker_reports_kaifu_resignation_notice_19911105`,
`jp_hc_president_reports_kaifu_resignation_notice_19911105`). The House of Representatives designated 宮澤喜一 with 276 of
492 (`jp_hr_designates_miyazawa_19911105`) and the House of Councillors in a runoff, 115 to 85 with 44 blank
(`jp_hc_designates_miyazawa_runoff_19911105`); the table records both, the resignation decision and 3.11.5 宮澤内閣成立
(`jp_hc_precedents_hc_designates_miyazawa_19911105`, `jp_hc_precedents_hr_designates_miyazawa_19911105`,
`jp_hc_precedents_kaifu_cabinet_resignation_19911105`, `jp_hc_precedents_miyazawa_cabinet_formed_19911105`). On 8 November
Miyazawa told both Houses that he had now been appointed Prime Minister (`jp_miyazawa_policy_speech_states_appointment_19911108`,
`jp_miyazawa_hc_policy_speech_states_appointment_19911108`). A minister said on 17 December 1991 that the cabinet was
launched on 5 November (`jp_minister_states_miyazawa_cabinet_formed_19911105`), and the Kantei's pages give the spans and
the formation (`jp_kantei_second_kaifu_span_19900228_19911105`, `jp_kantei_miyazawa_cabinet_formed_19911105`).

Decision: accepted in part. Miyazawa is observed on 8 November 1991; Kaifu's second appointment has no end.

Limits: the appointment day and ceremony, the day Kaifu's office ended and any continuation between 5 November and the
appointment are not stated.

### JP-PM-03 — 1993: Miyazawa to Hosokawa

Evidence: the House of Councillors received the notice of the Miyazawa cabinet's resignation en masse at 09:53 on
5 August 1993, the day the 127th special session convened (reported on 6 August; `jp_miyazawa_notifies_cabinet_resignation_19930805`).
On 6 August the House of Councillors designated 細川護煕 on the first ballot, 132 of 240 (`jp_hc_designates_hosokawa_19930806`),
and the House of Representatives, after a first roll call was abandoned, 262 of 503 (`jp_hr_designates_hosokawa_19930806`);
the table records both (printing 細川護熙), the resignation decision and 5.8.9 細川内閣成立
(`jp_hc_precedents_hc_designates_hosokawa_19930806`, `jp_hc_precedents_hr_designates_hosokawa_19930806`,
`jp_hc_precedents_miyazawa_cabinet_resignation_19930805`, `jp_hc_precedents_hosokawa_cabinet_formed_19930809`). On
23 August Hosokawa told both Houses that he had now been appointed (`jp_hosokawa_policy_speech_states_appointment_19930823`,
`jp_hosokawa_hc_policy_speech_states_appointment_19930823`). In committee on 24 August a minister said that the Hosokawa
cabinet came into being on 9 August, when he received the Prime Minister's special instruction on the Kagoshima
disaster, and a member said that Prime Minister Hosokawa entered the disaster area on the 13th
(`jp_minister_states_hosokawa_cabinet_formed_19930809`, `jp_minister_recalls_pm_instruction_19930809`,
`jp_member_recalls_pm_disaster_visit_19930813`; the last two found by the part A check in an existing source). The Kantei
pages give Miyazawa's span to 平成5年8月9日 and the Hosokawa cabinet's formation that day
(`jp_kantei_miyazawa_span_19911105_19930809`, `jp_kantei_hosokawa_cabinet_formed_19930809`,
`jp_kantei_hosokawa_span_19930809_19940428`).

Decision: accepted in part. Hosokawa is observed on 23 August 1993. The remarks about 9 and 13 August are claims dated by
the day they recall; they are later recollections and do not date the holder.

Limits: the appointment day and ceremony, Miyazawa's end and any continuation from 5 to 9 August are not stated.

### JP-PM-04 — 1994: Hata

Evidence: on 25 April 1994 both Houses received the notice that the Hosokawa cabinet had decided that day to resign en
masse (`jp_hr_receives_hosokawa_cabinet_resignation_notice_19940425`, `jp_hc_hosokawa_cabinet_resolves_resignation_19940425`)
and designated 羽田孜 (House of Representatives 274 of 502; House of Councillors 127 of 247; `jp_hr_designates_hata_19940425`,
`jp_hc_designates_hata_19940425`). The Kantei's cabinet list heads the 80th cabinet 羽田内閣−平成６年４月28日成立
(`jp_kantei_list_hata_cabinet_formed_19940428`), and a member recalled on 27 May the cabinet's launch on 28 April
(`jp_member_recalls_hata_cabinet_launch_19940428`). On 10 May 1994 Hata told both Houses that he had been appointed
(`jp_hata_states_appointed_pm_19940510`, `jp_hata_hr_policy_speech_states_appointment_19940510`; the House of
Representatives' record, added by the part B check, misprints the label as 内目総理大臣).

Decision: accepted in part. Hata is observed on 10 May 1994.

Limits: 28 April 1994 is not confirmed by a same-day record; no record shows Hosokawa performing the duties from 25 to
28 April; no end.

### JP-PM-05 — 1994: Murayama

Evidence: the Hata cabinet decided on 25 June 1994 to resign en masse; the House of Councillors received the notice at
11:56 that day (`jp_hc_receives_hata_resignation_notice_19940625`, added by the part B check), and it was reported on
29 June in the House of Representatives' Rules Committee and read in the House of Councillors' plenary
(`jp_hr_receives_hata_cabinet_resignation_notice_19940625`, `jp_hc_hata_cabinet_resolves_resignation_19940625`); the House
of Representatives' plenary does not repeat it. On 29 June the House of Representatives' first ballot gave no majority
(村山富市 241, 海部俊樹 220; `jp_hr_first_ballot_no_majority_19940629`) and the runoff designated 村山富市, 261 to 214
(`jp_hr_designates_murayama_runoff_19940629`); the House of Councillors designated him with 148 of 244
(`jp_hc_designates_murayama_19940629`). The Kantei list gives 平成６年６月30日成立 (`jp_kantei_list_murayama_cabinet_formed_19940630`),
and Murayama recalled on 9 November 1994 forming the cabinet on 30 June (`jp_murayama_recalls_cabinet_formed_19940630`).
On 6 July members of both Houses' agriculture committees referred to "the 81st Murayama cabinet", "today's Murayama
cabinet" and 村山総理 (`jp_hc_member_refers_to_murayama_cabinet_19940706`, `jp_hr_member_refers_to_murayama_cabinet_19940706`,
`jp_hr_member_refers_to_prime_minister_murayama_19940706`; added by the part B check). On 18 July he spoke as 内閣総理大臣
in his policy speech (`jp_murayama_speaks_as_pm_19940718`).

Decision: accepted in part. Murayama is observed on 18 July 1994; the members' references of 6 July are claims and, as
the check proposed, do not move the observation.

Limits: 30 June 1994 is not confirmed by a same-day record; no record shows Hata performing the duties from 25 to 30 June;
no end.

### JP-PM-06 — 1996: Hashimoto's two appointments

Evidence: on 11 January 1996 both Houses received the Murayama cabinet's resignation notice
(`jp_hr_receives_murayama_cabinet_resignation_notice_19960111`, `jp_hc_murayama_cabinet_resolves_resignation_19960111`)
and designated 橋本龍太郎 (288 of 489; 158 of 251; `jp_hr_designates_hashimoto_19960111`, `jp_hc_designates_hashimoto_19960111`).
The Kantei's statement titled 橋本内閣総理大臣談話 and dated that day opens "I have today taken on the heavy responsibility
of Prime Minister" (`jp_kantei_hashimoto_assumes_office_19960111`); the same page carries his instructions at the first
cabinet meeting of that day (`jp_kantei_first_cabinet_meeting_19960111`; check B9), and its original Last-Modified is
17 January 1996. On 7 November 1996 the House of Councillors received the resignation notice at 09:17
(`jp_hc_receives_hashimoto_cabinet_resignation_notice_19961107`), the Kantei's agenda lists an extraordinary cabinet
meeting on the resignation and the Prime Minister's statement (`jp_kantei_extraordinary_cabinet_resignation_item_19961107`,
`jp_kantei_cabinet_statement_item_19961107`), both Houses re-designated him (262 of 498; 145 of 248;
`jp_hr_designates_hashimoto_19961107`, `jp_hc_designates_hashimoto_19961107`), and the unnamed statement of that day says
the Prime Minister again took on the office that day (`jp_kantei_pm_assumes_office_again_19961107`). At the press
conference of 8 November, 橋本総理 said he had received the Diet's designation the previous day and continued to bear the
heavy responsibility, set out the second cabinet's priorities as Prime Minister, and recalled his appointment on
11 January (`jp_kantei_hashimoto_designated_again_continues_19961107`, `jp_kantei_hashimoto_press_conference_in_office_19961108`,
`jp_kantei_hashimoto_recalls_appointment_19960111`; check B4). The Kantei lists and his 1998 recollection are claims
(`jp_kantei_list_hashimoto_cabinet_1_formed_19960111`, `jp_kantei_list_hashimoto_cabinet_2_formed_19961107`,
`jp_kantei_hashimoto_recalls_taking_office_199601`; check B2).

Decision: accepted in part (check B6). The first holder starts on 11 January 1996. The second is observed on 8 November
1996: the statement of 7 November gives the day but prints no name, and the only record identifying its speaker is the
next day's, so it does not make a start.

Limits: no record of either ceremony; no end for either holder; the House of Representatives' own record of the
7 November notice was not found (its Rules Committee of that day does not mention it).

### JP-PM-07 — 1998: Obuchi

Evidence: on 30 July 1998 both Houses received the Hashimoto cabinet's resignation notice
(`jp_hr_receives_hashimoto_cabinet_resignation_notice_19980730`, `jp_hc_hashimoto_cabinet_resolves_resignation_19980730`);
the Kantei's agenda lists the resignation and Hashimoto's statement says the cabinet resigned that day
(`jp_kantei_extraordinary_cabinet_resignation_item_19980730`, `jp_kantei_hashimoto_cabinet_resigned_19980730`). The House of
Representatives designated 小渕恵三 with 268 of 497 (`jp_hr_designates_obuchi_19980730`); the House of Councillors' first
ballot gave no majority (小渕恵三 103, 菅直人 98) and the runoff designated 菅直人, 142 to 103
(`jp_hc_first_ballot_no_majority_19980730`, `jp_hc_designates_kan_runoff_19980730`). The joint committee requested at 15:33
voted 9 and 10, neither two-thirds of those present, and reached no agreement (`jp_hr_joint_committee_requested_19980730`,
`jp_joint_committee_no_agreement_19980730`); both Houses declared that under art. 67(2) the House of Representatives'
designation became the Diet's resolution, and the Speaker of the House of Representatives alone added that it would be
reported to the Emperor forthwith (`jp_hr_resolution_prevails_obuchi_19980730`, `jp_hc_hr_resolution_prevails_19980730`;
check B10). The Kantei's photo diary captions 小渕内閣 −平成10年7月30日 (`jp_kantei_diary_obuchi_cabinet_19980730`; check B5).
On 31 July the Kantei published the 初閣議 agenda, an unnamed statement "この度…重責を担うことになりました" and the record
小渕内閣総理大臣記者会見録, in which he says the new cabinet was launched the previous day
(`jp_kantei_first_cabinet_meeting_19980731`, `jp_kantei_obuchi_statement_bears_office_19980731`,
`jp_kantei_obuchi_press_conference_in_office_19980731`, `jp_kantei_obuchi_new_cabinet_launched_19980730`).

Decision: accepted in part. Obuchi is observed on 31 July 1998. The caption and his recollection make 30 July likely but
state no appointment, and his spoken "today" is not read as an assumption statement (check B8).

Limits: no record of the ceremony; his end is not stated (see JP-PM-08).

### JP-PM-08 — 2000: the acting Prime Minister and Mori

Evidence: Chief Cabinet Secretary 青木幹雄 told the House of Representatives on 10 April 2000 that Obuchi was admitted to
hospital about 01:00 on 2 April, that before 21:00 he was told Obuchi had been moved to intensive care, and that Obuchi fell
into a coma about 23:30 (`jp_obuchi_hospitalised_and_comatose_20000402`; check C3); that at about 19:00 Obuchi had told him
to take care of everything if anything happened (`jp_obuchi_instruction_to_aoki_1900_20000402`); that he accepted the acting
prime ministership at 09:00 on 3 April, announced at 11:00, and reported the acting designation at a 12:40 cabinet
meeting, notifying both Speakers and the Gazette (`jp_aoki_acting_pm_from_0900_announced_20000403`,
`jp_aoki_reports_acting_designation_notified_20000403`; check C4); and that at 19:00 on 4 April the cabinet, judging
Obuchi's condition the art. 70 case, decided to resign en masse (`jp_obuchi_cabinet_resignation_decided_20000404`). On
3 April itself a member asked in the House of Councillors to confirm that 青木 had become acting Prime Minister at nine that
morning, and the Deputy Chief Cabinet Secretary said it was so (`jp_member_asks_aoki_acting_pm_0900_20000403`,
`jp_aoki_takes_acting_pm_0900_20000403`; check C7). Aoki said on 26 April that the legal time was 09:00 on the 3rd
(`jp_aoki_states_acting_pm_legally_from_0900_20000403`), and a government answer of 23 May repeated the start of the
acting duties, the notice of the designation and the art. 70 decision (`jp_aoki_commenced_acting_duties_0900_20000403`,
`jp_acting_designation_notice_sent_20000403`, `jp_obuchi_cabinet_resignation_art70_20000404`). The notices signed by the
acting Prime Minister were received by the House of Councillors at 19:26 on 4 April and read in both Houses on 5 April
(`jp_sangiin_receives_resignation_notice_from_acting_pm_20000404`, `jp_shugiin_rules_reads_acting_pm_resignation_notice_20000404`,
`jp_shugiin_receives_resignation_notice_from_acting_pm_20000404`, `jp_sangiin_notice_cabinet_resigns_art70_20000404`). On
5 April both Houses designated 森喜朗 (335 of 488; 137 of 244; `jp_shugiin_designates_mori_20000405`,
`jp_sangiin_designates_mori_20000405`); his statement of that day, under the heading 森内閣総理大臣演説等, says he was
appointed that day (`jp_mori_appointed_pm_statement_20000405`); the Kantei's account says 森総理 was appointed at the
Imperial appointment ceremony that night as the 85th Prime Minister (`jp_mori_shinninshiki_appointed_20000405`,
`jp_mori_cabinet_formed_20000405`); the Imperial Household Agency's schedule records 親任式・認証官任命式（19名） that day
(`jp_kunaicho_ceremony_20000405`; check C1); and the April 2000 cabinet list's profile gives 平成１２年４月５日内閣総理大臣
(`jp_mori_profile_pm_career_entry_20000405`). On 4 July 2000 the House of Councillors received the resignation notice at
09:25 (`jp_sangiin_receives_mori_cabinet_resignation_notice_20000704`), both Houses re-designated him (284 of 479; 133 of
242; `jp_shugiin_designates_mori_20000704`, `jp_sangiin_designates_mori_20000704`), the Kantei's account says 森総理 was
appointed at the ceremony that night as the 86th Prime Minister (`jp_mori_shinninshiki_appointed_20000704`,
`jp_mori_second_cabinet_formed_20000704`), his unnamed statement says he again took on the office that day
(`jp_mori_resumes_pm_statement_20000704`), and the agency's schedule records the ceremony (`jp_kunaicho_ceremony_20000704`).
The Kantei's 85th and 86th pages give spans (`jp_kantei_span_mori_85_20000405_20000704`, `jp_kantei_span_mori_86_20000704_20010426`;
check C5).

Decision: accepted. Mori's holders start on 5 April and 4 July 2000. Acting service is claims only, and Obuchi's holder
gets no end.

Limits: no source states when Aoki's acting service ended, the day Obuchi's office ended or the day either Mori
appointment ended; the Gazette notices of the acting designation and the appointments were not accessible.

### JP-PM-09 — 2001-2005: Koizumi's three appointments

Evidence: on 26 April 2001 the House of Representatives' Rules Committee reported the notice received at 09:21 and the
House of Councillors' at 09:23 (`jp_shugiin_rules_receives_mori_resignation_notice_20010426`,
`jp_sangiin_rules_receives_mori_resignation_notice_20010426`; check C6); both plenaries reported or read it
(`jp_shugiin_receives_mori_cabinet_resignation_notice_20010426`, `jp_sangiin_notice_mori_cabinet_resigns_20010426`), and
Mori's statement says the Mori cabinet resigned that day (`jp_mori_cabinet_resigned_statement_20010426`). Both Houses
designated 小泉純一郎 (287 of 478; 138 of 246; `jp_shugiin_designates_koizumi_20010426`, `jp_sangiin_designates_koizumi_20010426`).
The Kantei's account 小泉内閣発足 names 小泉純一郎 and says that night he attended the Prime Minister's appointment
ceremony and then issued the Prime Minister's statement, which it quotes (`jp_koizumi_shinninshiki_20010426`,
`jp_koizumi_cabinet_formed_20010426`); the statement itself, "本日、内閣総理大臣に任命され", prints no name
(`jp_koizumi_appointed_pm_statement_20010426`); the Imperial Household Agency's schedule records the ceremony
(`jp_kunaicho_ceremony_20010426`). On 19 November 2003 the House of Councillors received the notice at 09:16
(`jp_sangiin_receives_koizumi_cabinet_resignation_notice_20031119`), both Houses re-designated him (281 of 479; 136 of 240;
`jp_shugiin_designates_koizumi_20031119`, `jp_sangiin_designates_koizumi_20031119`), his statement says he again took on the
office that day (`jp_koizumi_resumes_pm_statement_20031119`), the Kantei's account says he was appointed at the ceremony as
the 88th Prime Minister (`jp_koizumi_shinninshiki_appointed_20031119`, `jp_koizumi_second_cabinet_formed_20031119`), and the
agency's schedule records the ceremony (`jp_kunaicho_ceremony_20031119`). On 21 September 2005 the House of Councillors
received the notice at 09:14 and the cabinet resigned at a morning meeting (`jp_sangiin_receives_koizumi_cabinet_resignation_notice_20050921`,
`jp_koizumi_cabinet_resigned_morning_20050921`); both Houses re-designated him (340 of 479; 134 of 236;
`jp_shugiin_designates_koizumi_20050921`, `jp_sangiin_designates_koizumi_20050921`); his statement says that for the third
time he took on the office that day (`jp_koizumi_resumes_pm_statement_20050921`); the Kantei's account says he was appointed
at the ceremony as the 89th Prime Minister (`jp_koizumi_shinninshiki_appointed_20050921`, `jp_koizumi_third_cabinet_formed_20050921`);
and the agency's schedule and photo page record the ceremony, the photo page as 親任式（小泉内閣総理大臣）
(`jp_kunaicho_ceremony_20050921`, `jp_kunaicho_ceremony_koizumi_20050921`). The Kantei's 87th and 88th pages give spans
(`jp_kantei_span_koizumi_87_20010426_20031119`, `jp_kantei_span_koizumi_88_20031119_20050921`).

Decision: accepted. Koizumi's holders start on 26 April 2001, 19 November 2003 and 21 September 2005; none has an end.

Limits: no source states the day Mori's office or either of Koizumi's first two appointments ended; the Kantei account of
2001 says he attended the ceremony rather than that he was appointed, and the appointment itself is stated by the unnamed
statement it quotes.

### JP-PM-10 — 2006: the closing boundary

Evidence: on 25 September 2006 the Kantei recorded 小泉総理's last interview of his term at the Prime Minister's Office
(`jp_koizumi_pm_final_interview_20060925`). On 26 September the House of Representatives' Rules Committee reported the
notice received at 09:16 and the House of Councillors' at 09:19 (`jp_shugiin_rules_receives_koizumi_resignation_notice_20060926`,
`jp_sangiin_rules_receives_koizumi_resignation_notice_20060926`; check C6); both plenaries reported or read it
(`jp_shugiin_receives_koizumi_cabinet_resignation_notice_20060926`, `jp_sangiin_notice_koizumi_cabinet_resigns_20060926`);
Koizumi's statement says the Koizumi cabinet resigned that day (`jp_koizumi_cabinet_resigned_statement_20060926`). Both Houses
designated 安倍晋三 (339 of 476; 136 of 240; `jp_shugiin_designates_abe_20060926`, `jp_sangiin_designates_abe_20060926`); the
statement 安倍内閣総理大臣談話 says he was appointed that day (`jp_abe_appointed_pm_statement_20060926`); the Kantei's account
dates the ceremony to that night (`jp_abe_shinninshiki_20060926`, `jp_abe_cabinet_formed_20060926`); and the Imperial
Household Agency's schedule and photo page record 親任式（安倍内閣総理大臣） (`jp_kunaicho_ceremony_20060926`,
`jp_kunaicho_ceremony_abe_20060926`). The Kantei's 89th page gives Koizumi's span to 26 September 2006
(`jp_kantei_span_koizumi_89_20050921_20060926`).

Decision: accepted in part. Koizumi's 2005 holder is in office on 25 September 2006 and has no end. 安倍晋三's appointment
is recorded only as the closing boundary: seven claims whose rows carry no holder name, never used as Koizumi's end.

Limits: no contemporaneous source states the day Koizumi's office ended; the only dated end is the Kantei's retrospective
span, and no source states that he performed the duties until the successor's appointment.

## Sources added

| Source ID | What | Provenance |
|---|---|---|
| `jp_hr_written_answer_19900112` | Cabinet written answer No. 1 of the 117th session to the House of Representatives (内閣衆質一一七第一号), 12 Jan 1990, signed by the acting Prime Minister | official file, www.shugiin.go.jp; PDF page 1 viewed |
| `jp_hc_written_answer_19900119` | Cabinet written answer No. 1 of the 117th session to the House of Councillors (内閣参質一一七第一号), 19 Jan 1990, signed 内閣総理大臣 海部俊樹 | official file, www.sangiin.go.jp; PDF page 1 viewed |
| `jp_hr_written_answer_19900123` | Cabinet written answer No. 2 of the 117th session to the House of Representatives (内閣衆質一一七第二号), 23 Jan 1990, signed 内閣総理大臣 海部俊樹 | official file, www.shugiin.go.jp; PDF page 1 viewed |
| `jp_kantei_rekidai_076` | 第76代 海部 俊樹 / 歴代内閣 / 首相官邸ホームページ | capture 2026-01-28 of www.kantei.go.jp |
| `jp_hc_rules_committee_19900227` | 第118回国会 参議院 議院運営委員会 第1号（平成2年2月27日） | Diet minutes API, whole meeting record |
| `jp_hr_plenary_19900227` | 第118回国会 衆議院 本会議 第1号（平成2年2月27日） | Diet minutes API, whole meeting record |
| `jp_hc_plenary_19900227` | 第118回国会 参議院 本会議 第1号（平成2年2月27日） | Diet minutes API, whole meeting record |
| `jp_hc_precedents_pm_designations` | 参議院先例諸表 一一 内閣総理大臣の指名一覧表（令和5年版） | capture 2024-04-19 of www.sangiin.go.jp; PDF pages 15, 16 viewed; live copy at the relocated URL byte-identical |
| `jp_kantei_rekidai_077` | 第77代 海部 俊樹 / 歴代内閣 / 首相官邸ホームページ | capture 2025-12-21 of www.kantei.go.jp |
| `jp_hr_plenary_19900302` | 第118回国会 衆議院 本会議 第3号（平成2年3月2日） | Diet minutes API, whole meeting record |
| `jp_hc_plenary_19900302` | 第118回国会 参議院 本会議 第2号（平成2年3月2日） | Diet minutes API, whole meeting record |
| `jp_hr_budget_committee_19900409` | 第118回国会 衆議院 予算委員会 第6号（平成2年4月9日） | Diet minutes API, whole meeting record |
| `jp_hr_rules_committee_19911105` | 第122回国会 衆議院 議院運営委員会 第1号（平成3年11月5日） | Diet minutes API, whole meeting record |
| `jp_hc_rules_committee_19911105` | 第122回国会 参議院 議院運営委員会 第1号（平成3年11月5日） | Diet minutes API, whole meeting record |
| `jp_hr_plenary_19911105` | 第122回国会 衆議院 本会議 第1号（平成3年11月5日） | Diet minutes API, whole meeting record |
| `jp_hc_plenary_19911105` | 第122回国会 参議院 本会議 第1号（平成3年11月5日） | Diet minutes API, whole meeting record |
| `jp_hr_plenary_19911108` | 第122回国会 衆議院 本会議 第2号（平成3年11月8日） | Diet minutes API, whole meeting record |
| `jp_hc_plenary_19911108` | 第122回国会 参議院 本会議 第2号（平成3年11月8日） | Diet minutes API, whole meeting record |
| `jp_hc_cabinet_committee_19911217` | 第122回国会 参議院 内閣委員会 第2号（平成3年12月17日） | Diet minutes API, whole meeting record |
| `jp_kantei_rekidai_078` | 第78代 宮澤 喜一 / 歴代内閣 / 首相官邸ホームページ | capture 2026-06-13 of www.kantei.go.jp |
| `jp_hc_rules_committee_19930806` | 第127回国会 参議院 議院運営委員会 第2号（平成5年8月6日） | Diet minutes API, whole meeting record |
| `jp_hr_plenary_19930806` | 第127回国会 衆議院 本会議 第2号（平成5年8月6日） | Diet minutes API, whole meeting record |
| `jp_hc_plenary_19930806` | 第127回国会 参議院 本会議 第2号（平成5年8月6日） | Diet minutes API, whole meeting record |
| `jp_hr_plenary_19930823` | 第127回国会 衆議院 本会議 第4号（平成5年8月23日） | Diet minutes API, whole meeting record |
| `jp_hc_plenary_19930823` | 第127回国会 参議院 本会議 第4号（平成5年8月23日） | Diet minutes API, whole meeting record |
| `jp_hr_disaster_committee_19930824` | 第127回国会 衆議院 災害対策特別委員会 第2号（平成5年8月24日） | Diet minutes API, whole meeting record |
| `jp_kantei_rekidai_079` | 第79代 細川 護煕 / 歴代内閣 / 首相官邸ホームページ | capture 2025-12-07 of www.kantei.go.jp |
| `jp_hr_plenary_19940425` | 衆議院本会議録 第129回国会 第15号 (25 April 1994): 内閣総理大臣の指名 | Diet minutes API, whole meeting record |
| `jp_hc_plenary_19940425` | 参議院本会議録 第129回国会 第13号 (25 April 1994): 内閣総理大臣の指名 | Diet minutes API, whole meeting record |
| `jp_kantei_rekidai_hata_cabinet` | 首相官邸 歴代内閣: 羽田内閣 (第80代) cabinet list | capture 2025-11-18 of www.kantei.go.jp |
| `jp_hc_plenary_19940510_hata_speech` | 参議院本会議録 第129回国会 第15号 (10 May 1994), speech 2: 羽田孜 policy speech | Diet minutes API, single speech |
| `jp_hr_plenary_19940510_hata_speech` | 衆議院本会議録 第129回国会 第17号 (10 May 1994), speech 3: 羽田孜 policy speech | Diet minutes API, single speech |
| `jp_hr_audit_subcommittee_19940527` | 衆議院決算委員会第三分科会議録 第129回国会 第2号 (27 May 1994), speech 11 | Diet minutes API, single speech |
| `jp_hc_rules_committee_19940629` | 参議院議院運営委員会会議録 第129回国会 第26号 (29 June 1994), speech 2 | Diet minutes API, single speech |
| `jp_hr_rules_committee_19940629` | 衆議院議院運営委員会議録 第129回国会 第32号 (29 June 1994) | Diet minutes API, whole meeting record |
| `jp_hr_plenary_19940629` | 衆議院本会議録 第129回国会 第32号 (29 June 1994): 内閣総理大臣の指名 | Diet minutes API, whole meeting record |
| `jp_hc_plenary_19940629` | 参議院本会議録 第129回国会 第26号 (29 June 1994): 内閣総理大臣の指名 | Diet minutes API, whole meeting record |
| `jp_kantei_rekidai_murayama_cabinet` | 首相官邸 歴代内閣: 村山内閣 (第81代) cabinet list | capture 2024-08-17 of www.kantei.go.jp |
| `jp_hc_agriculture_committee_19940706` | 参議院農林水産委員会会議録 第129回国会閉会後 第1号 (6 July 1994), speech 27 | Diet minutes API, single speech |
| `jp_hr_agriculture_committee_19940706_s081` | 衆議院農林水産委員会議録 第129回国会 第12号 (6 July 1994), speech 81 | Diet minutes API, single speech |
| `jp_hr_agriculture_committee_19940706_s116` | 衆議院農林水産委員会議録 第129回国会 第12号 (6 July 1994), speech 116 | Diet minutes API, single speech |
| `jp_hr_plenary_19940718_murayama_speech` | 衆議院本会議録 第130回国会 第1号 (18 July 1994), speech 17: 村山富市 policy speech | Diet minutes API, single speech |
| `jp_hr_tax_committee_19941109_murayama` | 衆議院税制改革に関する特別委員会議録 第131回国会 第9号 (9 November 1994), speech 83 | Diet minutes API, single speech |
| `jp_hr_plenary_19960111` | 衆議院本会議録 第135回国会 第1号 (11 January 1996): 内閣総理大臣の指名 | Diet minutes API, whole meeting record |
| `jp_hc_plenary_19960111` | 参議院本会議録 第135回国会 第1号 (11 January 1996): 内閣総理大臣の指名 | Diet minutes API, whole meeting record |
| `jp_kantei_hashimoto_statement_19960111` | 首相官邸: 橋本内閣総理大臣談話 (平成八年一月十一日) | capture 1997-01-06 of www.kantei.go.jp |
| `jp_kantei_rekidai_hashimoto_cabinet_1` | 首相官邸 歴代内閣: 第一次橋本内閣 (第82代) cabinet list | capture 2025-12-09 of www.kantei.go.jp |
| `jp_hc_rules_committee_19961107` | 参議院議院運営委員会会議録 第138回国会 第1号 (7 November 1996) | Diet minutes API, whole meeting record |
| `jp_kantei_cabinet_agenda_19961107` | 首相官邸: 閣議案件（項目）の公表 (kakugi-1107, 7 November 1996) | capture 1997-01-06 of www.kantei.go.jp |
| `jp_hr_plenary_19961107` | 衆議院本会議録 第138回国会 第1号 (7 November 1996): 内閣総理大臣の指名 | Diet minutes API, whole meeting record |
| `jp_hc_plenary_19961107` | 参議院本会議録 第138回国会 第1号 (7 November 1996): 内閣総理大臣の指名 | Diet minutes API, whole meeting record |
| `jp_kantei_pm_statement_19961107` | 首相官邸: 内閣総理大臣談話 (平成八年十一月七日) | capture 1997-01-06 of www.kantei.go.jp |
| `jp_kantei_press_conference_19961108` | 首相官邸: 第二次橋本内閣組閣後記者会見 (平成８年１１月８日) | capture 1997-01-06 of www.kantei.go.jp |
| `jp_kantei_rekidai_hashimoto_cabinet_2` | 首相官邸 歴代内閣: 第二次橋本内閣 (第83代) cabinet list | capture 2025-02-11 of www.kantei.go.jp |
| `jp_hr_plenary_19980730` | 衆議院本会議録 第143回国会 第1号 (30 July 1998): 内閣総理大臣の指名, 両院協議会 | Diet minutes API, whole meeting record |
| `jp_hc_plenary_19980730` | 参議院本会議録 第143回国会 第1号 (30 July 1998): 内閣総理大臣の指名, 両院協議会 | Diet minutes API, whole meeting record |
| `jp_joint_committee_19980730` | 内閣総理大臣の指名両院協議会会議録 第143回国会 第1号 (30 July 1998) | Diet minutes API, whole meeting record |
| `jp_kantei_cabinet_agenda_19980730` | 首相官邸: 閣議案件（項目）の公表 (kakugi-1998073010, 30 July 1998) | capture 2000-04-22 of www.kantei.go.jp |
| `jp_kantei_hashimoto_resignation_statement_19980730` | 首相官邸: 内閣総辞職に当たっての内閣総理大臣談話 (平成十年七月三十日) | capture 1999-01-27 of www0.kantei.go.jp |
| `jp_kantei_diary_19980730` | 首相官邸 官邸ダイアリー 7月 (1998): 小渕内閣 photo entry | capture 2003-04-24 of www.kantei.go.jp |
| `jp_kantei_cabinet_agenda_19980731` | 首相官邸: 閣議案件 初閣議 (kakugi-1998073111, 31 July 1998) | capture 2000-04-22 of www.kantei.go.jp |
| `jp_kantei_obuchi_statement_19980731` | 首相官邸: 内閣総理大臣談話 (平成十年七月三十一日) | capture 1999-01-27 of www0.kantei.go.jp |
| `jp_kantei_obuchi_press_conference_19980731` | 首相官邸: 小渕内閣総理大臣記者会見録 (平成１０年７月３１日) | capture 1999-01-28 of www0.kantei.go.jp |
| `jp_kantei_rekidai_obuchi_cabinet` | 首相官邸 歴代内閣: 小渕内閣 (第84代) cabinet list | capture 2025-11-20 of www.kantei.go.jp |
| `jp_shugiin_honkaigi_20000410_aoki` | House of Representatives plenary No. 22, 10 Apr 2000, 147th Diet: Chief Cabinet Secretary Aoki's account of 2-4 April 2000 (speech 5) | Diet minutes API, single speech |
| `jp_sangiin_gyoseikanshi_20000403_s35` | House of Councillors, Administrative Oversight Committee (行政監視委員会) No. 5, 3 Apr 2000, 147th Diet: speech 35 (江田五月's question) | Diet minutes API, single speech |
| `jp_sangiin_gyoseikanshi_20000403_s36` | House of Councillors, Administrative Oversight Committee (行政監視委員会) No. 5, 3 Apr 2000, 147th Diet: speech 36 (the Deputy Chief Cabinet Secretary's answer) | Diet minutes API, single speech |
| `jp_shugiin_naikaku_20000426_aoki` | House of Representatives, Cabinet Committee (内閣委員会) No. 5, 26 Apr 2000, 147th Diet: Chief Cabinet Secretary Aoki (speech 11) | Diet minutes API, single speech |
| `jp_sangiin_homu_20000523_matsutani` | House of Councillors, Judicial Affairs Committee (法務委員会) No. 16, 23 May 2000, 147th Diet: Deputy Chief Cabinet Secretary Matsutani's answer (speech 32) | Diet minutes API, single speech |
| `jp_sangiin_giun_20000405` | House of Councillors, Committee on Rules and Administration (議院運営委員会) No. 13, 5 Apr 2000: Secretary General's report (speech 2) | Diet minutes API, single speech |
| `jp_shugiin_giun_20000405` | House of Representatives, Committee on Rules and Administration (議院運営委員会) No. 21, 5 Apr 2000, 147th Diet: chairman's report (speech 1) | Diet minutes API, single speech |
| `jp_shugiin_honkaigi_20000405` | House of Representatives plenary No. 20, 5 Apr 2000, 147th Diet: notice of resignation and designation of the Prime Minister | Diet minutes API, whole meeting record |
| `jp_sangiin_honkaigi_20000405` | House of Councillors plenary No. 12, 5 Apr 2000, 147th Diet: notice of resignation and designation of the Prime Minister | Diet minutes API, whole meeting record |
| `jp_kantei_mori_danwa_20000405` | Kantei: 内閣総理大臣談話 (Statement by the Prime Minister), 5 Apr 2000 | capture 2002-02-11 of www.kantei.go.jp |
| `jp_kantei_mori_hossoku_200004` | Kantei: 森総理の動き・トピックス（1）[森喜朗内閣発足], April 2000 | capture 2002-06-21 of www.kantei.go.jp |
| `jp_kunaicho_schedule_h12_q2` | 宮内庁: 天皇皇后両陛下のご日程（平成１２年４月～６月） | capture 2001-03-06 of www.kunaicho.go.jp |
| `jp_kantei_mori_profile_200004` | Kantei: 森内閣閣僚名簿 profile page 内閣総理大臣 森 喜朗 (April 2000 cabinet list) | capture 2000-05-10 of www.kantei.go.jp |
| `jp_sangiin_giun_20000704` | House of Councillors, Committee on Rules and Administration No. 1, 4 Jul 2000, 148th (special) Diet: Secretary General's report (speech 2) | Diet minutes API, single speech |
| `jp_shugiin_honkaigi_20000704` | House of Representatives plenary No. 1, 4 Jul 2000, 148th (special) Diet: designation of the Prime Minister | Diet minutes API, whole meeting record |
| `jp_sangiin_honkaigi_20000704` | House of Councillors plenary No. 1, 4 Jul 2000, 148th (special) Diet: designation of the Prime Minister | Diet minutes API, whole meeting record |
| `jp_kantei_mori_danwa_20000704` | Kantei: 内閣総理大臣談話, 4 Jul 2000 | capture 2002-06-13 of www.kantei.go.jp |
| `jp_kantei_mori_hossoku_200007` | Kantei: 第２次森内閣発足, 4 Jul 2000 | capture 2002-08-18 of www.kantei.go.jp |
| `jp_kunaicho_schedule_h12_q3` | 宮内庁: 天皇皇后両陛下のご日程（平成１２年７月～９月） | capture 2001-03-06 of www.kunaicho.go.jp |
| `jp_kantei_rekidai_085` | Kantei 歴代内閣: 第85代 森 喜朗 (retrospective page) | capture 2026-01-29 of www.kantei.go.jp |
| `jp_kantei_rekidai_086` | Kantei 歴代内閣: 第86代 森 喜朗 (retrospective page) | capture 2026-02-13 of www.kantei.go.jp |
| `jp_shugiin_giun_20010426` | House of Representatives, Committee on Rules and Administration No. 26, 26 Apr 2001, 151st Diet: chairman's report (speech 1) | Diet minutes API, single speech |
| `jp_sangiin_giun_20010426` | House of Councillors, Committee on Rules and Administration No. 20, 26 Apr 2001, 151st Diet: Secretary General's report (speech 2) | Diet minutes API, single speech |
| `jp_shugiin_honkaigi_20010426` | House of Representatives plenary No. 26, 26 Apr 2001, 151st Diet: notice of resignation and designation of the Prime Minister | Diet minutes API, whole meeting record |
| `jp_sangiin_honkaigi_20010426` | House of Councillors plenary No. 20, 26 Apr 2001, 151st Diet: notice of resignation and designation of the Prime Minister | Diet minutes API, whole meeting record |
| `jp_kantei_mori_jisyoku_danwa_20010426` | Kantei: 内閣総辞職に当たっての内閣総理大臣談話 (Statement on the cabinet's resignation), 26 Apr 2001 | capture 2001-12-27 of www.kantei.go.jp |
| `jp_kantei_koizumi_danwa_20010426` | Kantei: 内閣総理大臣談話, 26 Apr 2001 | capture 2001-05-01 of www.kantei.go.jp |
| `jp_kantei_koizumi_hossoku_20010426` | Kantei: 小泉総理の動き — 小泉内閣発足, 26 Apr 2001 | capture 2001-05-01 of www.kantei.go.jp |
| `jp_kunaicho_schedule_h13_q2` | 宮内庁: 天皇皇后両陛下のご日程（平成１３年４月～） | capture 2001-06-09 of www.kunaicho.go.jp |
| `jp_kantei_rekidai_087` | Kantei 歴代内閣: 第87代 小泉 純一郎 (retrospective page) | capture 2026-06-05 of www.kantei.go.jp |
| `jp_sangiin_giun_20031119` | House of Councillors, Committee on Rules and Administration No. 1, 19 Nov 2003, 158th (special) Diet: Secretary General's report (speech 2) | Diet minutes API, single speech |
| `jp_shugiin_honkaigi_20031119` | House of Representatives plenary No. 1, 19 Nov 2003, 158th (special) Diet: designation of the Prime Minister | Diet minutes API, whole meeting record |
| `jp_sangiin_honkaigi_20031119` | House of Councillors plenary No. 1, 19 Nov 2003, 158th (special) Diet: designation of the Prime Minister | Diet minutes API, whole meeting record |
| `jp_kantei_koizumi_danwa_20031119` | Kantei: 内閣総理大臣談話, 19 Nov 2003 | capture 2003-11-21 of www.kantei.go.jp |
| `jp_kantei_koizumi_sokaku_20031119` | Kantei: 小泉総理の動き — 第二次小泉内閣の発足, 19 Nov 2003 | capture 2003-12-09 of www.kantei.go.jp |
| `jp_kunaicho_schedule_h15_q4` | 宮内庁: 天皇皇后両陛下のご日程（平成１５年１０月～１２月） | capture 2004-02-16 of www.kunaicho.go.jp |
| `jp_kantei_rekidai_088` | Kantei 歴代内閣: 第88代 小泉 純一郎 (retrospective page) | capture 2026-04-19 of www.kantei.go.jp |
| `jp_sangiin_giun_20050921` | House of Councillors, Committee on Rules and Administration No. 1, 21 Sep 2005, 163rd (special) Diet: Secretary General's report (speech 2) | Diet minutes API, single speech |
| `jp_shugiin_honkaigi_20050921` | House of Representatives plenary No. 1, 21 Sep 2005, 163rd (special) Diet: designation of the Prime Minister | Diet minutes API, whole meeting record |
| `jp_sangiin_honkaigi_20050921` | House of Councillors plenary No. 1, 21 Sep 2005, 163rd (special) Diet: designation of the Prime Minister | Diet minutes API, whole meeting record |
| `jp_kantei_koizumi_danwa_20050921` | Kantei: 内閣総理大臣談話, 21 Sep 2005 | capture 2005-10-28 of www.kantei.go.jp |
| `jp_kantei_koizumi_sokaku_20050921` | Kantei: 小泉総理の動き — 第３次小泉内閣の発足, 21 Sep 2005 | capture 2005-11-27 of www.kantei.go.jp |
| `jp_kunaicho_schedule_2005_h2` | 宮内庁: 天皇皇后両陛下のご日程（平成17年7月～） | capture 2005-11-22 of www.kunaicho.go.jp |
| `jp_kunaicho_photo_20050921` | 宮内庁: 天皇皇后両陛下のご日程 写真 平成17年9月21日 親任式・認証官任命式 | capture 2005-12-19 of www.kunaicho.go.jp |
| `jp_kantei_koizumi_interview_20060925` | Kantei: 小泉総理の動き — 小泉総理インタビュー, 25 Sep 2006 | capture 2006-10-04 of www.kantei.go.jp |
| `jp_shugiin_giun_20060926` | House of Representatives, Committee on Rules and Administration No. 1, 26 Sep 2006, 165th Diet: chairman's report (speech 1) | Diet minutes API, single speech |
| `jp_sangiin_giun_20060926` | House of Councillors, Committee on Rules and Administration No. 1, 26 Sep 2006, 165th Diet: Secretary General's report (speech 2) | Diet minutes API, single speech |
| `jp_shugiin_honkaigi_20060926` | House of Representatives plenary No. 1, 26 Sep 2006, 165th (extraordinary) Diet: notice of resignation and designation of the Prime Minister | Diet minutes API, whole meeting record |
| `jp_sangiin_honkaigi_20060926` | House of Councillors plenary No. 1, 26 Sep 2006, 165th (extraordinary) Diet: notice of resignation and designation of the Prime Minister | Diet minutes API, whole meeting record |
| `jp_kantei_koizumi_sojishoku_danwa_20060926` | Kantei: 内閣総辞職に当たっての内閣総理大臣談話, 26 Sep 2006 | capture 2006-10-04 of www.kantei.go.jp |
| `jp_kantei_abe_danwa_20060926` | Kantei: 安倍内閣総理大臣談話, 26 Sep 2006 | capture 2006-10-04 of www.kantei.go.jp |
| `jp_kantei_abe_hossoku_20060926` | Kantei: 安倍総理の動き — 安倍内閣の発足, 26 Sep 2006 | capture 2006-10-04 of www.kantei.go.jp |
| `jp_kunaicho_schedule_2006_h2` | 宮内庁: 天皇皇后両陛下のご日程（平成18年7月～） | capture 2006-10-10 of www.kunaicho.go.jp |
| `jp_kunaicho_photo_20060926` | 宮内庁: 天皇皇后両陛下のご日程 写真 平成18年9月26日 親任式・認証官任命式 | capture 2006-10-10 of www.kunaicho.go.jp |
| `jp_kantei_rekidai_089` | Kantei 歴代内閣: 第89代 小泉 純一郎 (retrospective page) | capture 2026-05-13 of www.kantei.go.jp |

Forty-nine sources are raw Internet Archive captures (`id_` form) made before the cutoff: 40 Kantei pages, eight
Imperial Household Agency pages and the House of Councillors' table. Each records the capture URL as `url`, the address
inside it (without `:80`) as `original_url`, and in its extract the capture time, the archived original's Last-Modified
header and the character encoding (Shift_JIS, cp932, EUC-JP, ISO-2022-JP or UTF-8, with any declaration found). Sixty-seven
are responses of the National Diet Library's Diet minutes API (44 whole meeting records and 23 single
speeches), recorded by the exact API URL whose bytes were hashed, with the readable minutes page named in the extract. Three
are the Houses' static written-answer PDFs.

Each new source has a derived factual extract under [sources/](sources/) in the packet's format
(`spheres-c01-derived-factual-table/v1`): one row per claim, keyed by `claim_id`, with `observation_id`
`jp_prime_minister`, `review_observation`, `role_id` `jp_pm`, `holder_name`, `role_title` "内閣総理大臣 — Prime Minister of
Japan", `event_kind`, `attested_on`, the claim's text and locator. The extract's own checksum is in the packet, separate from
the response hash. Original pages, PDFs, JSON bodies, images and renders are not checked in, and no photograph, emblem,
seal or signature is republished.

Source types: `primary_diet_minutes_api_json`, `primary_house_precedents_table_pdf_archived`, `primary_imperial_household_photo_page_archived`, `primary_imperial_household_schedule_archived`, `primary_kantei_account_archived`, `primary_kantei_cabinet_agenda_archived`, `primary_kantei_cabinet_list_profile_archived`, `primary_kantei_page_archived_retrospective`, `primary_kantei_photo_caption_archived`, `primary_kantei_press_conference_archived`, `primary_kantei_statement_archived` and `primary_written_answer_pdf`.

## Response identities and stability checks

The reviewer re-downloads every recorded response and compares its byte count and SHA-256, so every identity here was
downloaded at least twice, at least 30 minutes apart, with identical bytes, and no identity is a page generated per
request. How each was established:

- **Part A.** The researcher downloaded each response three or four times (18:06Z-18:25Z, again at 18:57Z, at least
  30 minutes after round 1, and with a cache-busting query on the live hosts; the Lotus Domino server of shugiin.go.jp
  rejects a bare `?_cb=` with HTTP 400, so it took `?OpenElement&_cb=`), and the check three more times (18:59Z, with a
  cache-busting query at 19:00Z, and plain at 19:29Z, more than 30 minutes after every earlier download).
- **Part B.** The researcher downloaded each response at 18:06Z-18:26Z and again at 19:00Z-19:01Z; the check at
  19:02Z-19:03Z (the Diet API parameters reordered as a cache-bust) and at 19:32Z-19:33Z from the exact recorded URLs.
- **Part C.** The researcher downloaded each response at 18:28Z-18:38Z, with a cache-busting query on the Diet API, and
  again at 19:10Z-19:12Z; the check at 19:13Z-19:15Z, with a cache-busting query on every API response, and at
  19:42Z-19:44Z. The part C summary counted 21 API responses and 19 captures; the dossier holds 19 API responses and
  21 captures (check C9).
- **Records added by the checks.** Part A's check downloaded its five candidates three times (19:18Z-19:29Z), part B's its
  six twice (19:29Z and, with the parameters reordered, 19:39Z), and part C's its sixteen two or three times (19:21Z-20:08Z,
  the API ones also with a cache-busting query).
- **This packet (24 September).** Every one of the 119 identities, and the House of Councillors' table at its
  relocated live URL, was downloaded three more times: plain at 20:17Z-20:21Z, with a cache-busting query on the live hosts at
  20:38Z-20:42Z (captures plain), and plain again at 20:53Z-20:56Z, at least 30 minutes after the first. Every byte count and SHA-256
  matched. Each extract's provenance note gives that source's own download times.

Reproducibility traps met and avoided:

- **Compression.** The Diet API and the Internet Archive send `Content-Encoding: gzip` when a client asks for it, and the
  transferred bytes then differ (speech `113104583X00919941109_083` is 1,304 bytes gzip against 2,518; the `danwa-1107`
  capture 1,360 against 1,953). Every recorded identity is the identity-encoded body, requested without
  `Accept-Encoding`; each extract records `source_response_content_encoding` `identity` (check B1).
- **The Diet minutes site is a JavaScript application.** Only the public API is recorded (`/api/meeting?issueID=…`,
  `/api/speech?speechID=…` or `/api/speech?issueID=…&speechNumber=…`, always with `recordPacking=json`). Its responses carry
  `Cache-Control: no-cache, no-store`, CloudFront reported cache misses on the cache-busting requests, and the JSON has no
  timestamp, token or request echo. A full-text search URL (`any=`) depends on the search index, so part C's search URL
  for 3 April 2000 is replaced by the two record-pinned speeches (check C7); speech 36 is byte-identical to the
  researcher's first download.
- **Static House files.** The written answers have fixed `Last-Modified` and `ETag` headers and PDF CreationDates of 2002
  and 2010, not the access date; the recorded URLs are plain.
- **A relocated file.** The House of Councillors' table returns 404 at its old URL; the same bytes are live at
  `https://www.sangiin.go.jp/jpn/shiryo/houki/09senrei/pdf/r5se-s-11.pdf`, recorded as the extract's
  `alternate_location`, while the 2024 capture stays the recorded identity (check A9).
- **A capture whose digest does not match.** The 2000 capture of the 1998 photo diary replays 407 bytes whose SHA-1 is not
  its CDX digest; the 2003 capture has the same bytes and a matching digest and is recorded (check B5). On 24 September this packet compared the SHA-1 of every recorded capture with the Internet Archive's CDX digest for that capture: all 49 of 49 match, including the two part B's check could not compare while the CDX server was offline (kakuryo/83 and kakugi-1998073111).
- The Internet Archive returned transient connection errors and CDX 503/504 replies at times; those were retried one
  request at a time, and no recorded identity is an error page.

| Source ID | Bytes | SHA-256 | Recorded response |
|---|---|---|---|
| `jp_hr_written_answer_19900112` | 78,598 | `dbe70268afc7…54410d` | official file |
| `jp_hc_written_answer_19900119` | 23,985 | `66623a6ed289…399f24` | official file |
| `jp_hr_written_answer_19900123` | 89,159 | `e3980687d480…92a7e6` | official file |
| `jp_kantei_rekidai_076` | 16,232 | `2f9bad3a615a…37d069` | raw capture |
| `jp_hc_rules_committee_19900227` | 26,192 | `02d7740f47d2…7aadb4` | API meeting record |
| `jp_hr_plenary_19900227` | 81,054 | `e69f7ed0eeca…470de0` | API meeting record |
| `jp_hc_plenary_19900227` | 35,670 | `ee46738e4f20…7ef399` | API meeting record |
| `jp_hc_precedents_pm_designations` | 1,071,345 | `9b828b599e4c…0874fb` | raw capture; relocated live file byte-identical |
| `jp_kantei_rekidai_077` | 25,031 | `f007a6ae4edd…cdaac2` | raw capture |
| `jp_hr_plenary_19900302` | 104,570 | `22eb38e54c64…76bfda` | API meeting record |
| `jp_hc_plenary_19900302` | 103,067 | `ea174d7abab9…2d5662` | API meeting record |
| `jp_hr_budget_committee_19900409` | 555,162 | `28a60fff8bab…141a64` | API meeting record |
| `jp_hr_rules_committee_19911105` | 37,436 | `f708bd84a699…f4c34f` | API meeting record |
| `jp_hc_rules_committee_19911105` | 21,160 | `a4b37f784f25…c45991` | API meeting record |
| `jp_hr_plenary_19911105` | 41,252 | `b6b656d97dce…e31960` | API meeting record |
| `jp_hc_plenary_19911105` | 14,936 | `323355a254ab…962706` | API meeting record |
| `jp_hr_plenary_19911108` | 29,831 | `8c7fb41e3bae…225405` | API meeting record |
| `jp_hc_plenary_19911108` | 23,813 | `c3f2d2b22410…b15122` | API meeting record |
| `jp_hc_cabinet_committee_19911217` | 273,120 | `6edbbbdf914e…a1bea9` | API meeting record |
| `jp_kantei_rekidai_078` | 26,936 | `c3959417817a…27687c` | raw capture |
| `jp_hc_rules_committee_19930806` | 10,406 | `7b073e9b3ad3…b59608` | API meeting record |
| `jp_hr_plenary_19930806` | 39,537 | `913974c160a6…d9334e` | API meeting record |
| `jp_hc_plenary_19930806` | 6,164 | `057e07b4ccaf…f3b560` | API meeting record |
| `jp_hr_plenary_19930823` | 27,736 | `89a4f7737875…52061e` | API meeting record |
| `jp_hc_plenary_19930823` | 32,382 | `3edd3e3dc3dc…dd3cdf` | API meeting record |
| `jp_hr_disaster_committee_19930824` | 443,166 | `b8233337d062…b4c980` | API meeting record |
| `jp_kantei_rekidai_079` | 15,624 | `8ce1e7bd93c7…f0354d` | raw capture |
| `jp_hr_plenary_19940425` | 6,323 | `e588f50213e3…0c30ca` | API meeting record |
| `jp_hc_plenary_19940425` | 6,923 | `07d2650e4f57…1d8e46` | API meeting record |
| `jp_kantei_rekidai_hata_cabinet` | 3,274 | `dc6ba2267919…7f7264` | raw capture |
| `jp_hc_plenary_19940510_hata_speech` | 24,236 | `a19d3c4ee6c6…a4ab2b` | API speech |
| `jp_hr_plenary_19940510_hata_speech` | 24,524 | `2a0079926319…5e88c5` | API speech |
| `jp_hr_audit_subcommittee_19940527` | 10,551 | `79444bfde2aa…211b94` | API speech |
| `jp_hc_rules_committee_19940629` | 1,008 | `0194e0da014d…bef044` | API speech |
| `jp_hr_rules_committee_19940629` | 29,428 | `37e090775d64…059f39` | API meeting record |
| `jp_hr_plenary_19940629` | 29,240 | `a847d19e1f8f…9155b9` | API meeting record |
| `jp_hc_plenary_19940629` | 11,694 | `19f910c7227a…95b7b3` | API meeting record |
| `jp_kantei_rekidai_murayama_cabinet` | 3,379 | `b75ee146e858…4e71e0` | raw capture |
| `jp_hc_agriculture_committee_19940706` | 5,184 | `c9c9262d2aa0…703e70` | API speech |
| `jp_hr_agriculture_committee_19940706_s081` | 3,099 | `ea81a73a5836…e7d7cb` | API speech |
| `jp_hr_agriculture_committee_19940706_s116` | 3,396 | `2bb389f3ce37…1fc922` | API speech |
| `jp_hr_plenary_19940718_murayama_speech` | 24,932 | `d5816bdaa334…10752f` | API speech |
| `jp_hr_tax_committee_19941109_murayama` | 2,518 | `d04a33b2397a…cdffe4` | API speech |
| `jp_hr_plenary_19960111` | 14,221 | `6fe9ef7adaad…5e7e80` | API meeting record |
| `jp_hc_plenary_19960111` | 17,116 | `064bc1475871…b48a07` | API meeting record |
| `jp_kantei_hashimoto_statement_19960111` | 3,939 | `b2f8fb68b7fa…e09b87` | raw capture |
| `jp_kantei_rekidai_hashimoto_cabinet_1` | 2,807 | `f71b842cd4d0…755cdd` | raw capture |
| `jp_hc_rules_committee_19961107` | 28,209 | `35640ad7959f…797a0a` | API meeting record |
| `jp_kantei_cabinet_agenda_19961107` | 1,749 | `3e5d423f9f56…129e02` | raw capture |
| `jp_hr_plenary_19961107` | 27,726 | `53ce81381b49…786f6e` | API meeting record |
| `jp_hc_plenary_19961107` | 14,333 | `e560ca9e7db3…c960c0` | API meeting record |
| `jp_kantei_pm_statement_19961107` | 1,953 | `1f226807c657…129f44` | raw capture |
| `jp_kantei_press_conference_19961108` | 43,397 | `ba669228bb0f…aaef27` | raw capture |
| `jp_kantei_rekidai_hashimoto_cabinet_2` | 2,741 | `9c289bbf1ff5…2f5393` | raw capture |
| `jp_hr_plenary_19980730` | 20,854 | `e947da3919d5…32a4b7` | API meeting record |
| `jp_hc_plenary_19980730` | 30,579 | `ea863bf0530d…47254e` | API meeting record |
| `jp_joint_committee_19980730` | 22,992 | `d605c74d9b02…d2e163` | API meeting record |
| `jp_kantei_cabinet_agenda_19980730` | 1,880 | `58691d472c36…e17ca6` | raw capture |
| `jp_kantei_hashimoto_resignation_statement_19980730` | 2,197 | `d29d4d97cd6b…3b65c5` | raw capture |
| `jp_kantei_diary_19980730` | 407 | `544b92f11d6b…62c893` | raw capture; 2003 capture (CDX digest matches) |
| `jp_kantei_cabinet_agenda_19980731` | 2,051 | `a447be612fae…c924fe` | raw capture |
| `jp_kantei_obuchi_statement_19980731` | 2,019 | `af0c6a3832de…7f659e` | raw capture |
| `jp_kantei_obuchi_press_conference_19980731` | 33,398 | `519c9042ef49…783c8f` | raw capture |
| `jp_kantei_rekidai_obuchi_cabinet` | 3,446 | `fd40dab55937…e2b055` | raw capture |
| `jp_shugiin_honkaigi_20000410_aoki` | 9,073 | `1eec231c38f4…bc74e6` | API speech |
| `jp_sangiin_gyoseikanshi_20000403_s35` | 1,028 | `54f47c0d3849…d74006` | API speech |
| `jp_sangiin_gyoseikanshi_20000403_s36` | 1,149 | `8236caf46b05…0bd6f1` | API speech |
| `jp_shugiin_naikaku_20000426_aoki` | 948 | `17d029526771…f331ff` | API speech |
| `jp_sangiin_homu_20000523_matsutani` | 3,727 | `6cb96b67f063…a00342` | API speech |
| `jp_sangiin_giun_20000405` | 1,039 | `8ec976741cfd…1957c6` | API speech |
| `jp_shugiin_giun_20000405` | 1,326 | `ea433f37b67e…d789ce` | API speech |
| `jp_shugiin_honkaigi_20000405` | 22,329 | `e6b38e37bf07…dc657d` | API meeting record |
| `jp_sangiin_honkaigi_20000405` | 5,985 | `ca76c989b804…94a8ac` | API meeting record |
| `jp_kantei_mori_danwa_20000405` | 1,622 | `254b53736186…6beee6` | raw capture |
| `jp_kantei_mori_hossoku_200004` | 3,221 | `fd019a93fafe…00611c` | raw capture |
| `jp_kunaicho_schedule_h12_q2` | 18,043 | `de92452f2eb2…d64bc8` | raw capture |
| `jp_kantei_mori_profile_200004` | 1,588 | `b225d2d0dc57…ff155b` | raw capture |
| `jp_sangiin_giun_20000704` | 1,023 | `81686506d891…1b1afa` | API speech |
| `jp_shugiin_honkaigi_20000704` | 74,622 | `e01df453cfca…424ce0` | API meeting record |
| `jp_sangiin_honkaigi_20000704` | 23,235 | `8d52148a789c…902106` | API meeting record |
| `jp_kantei_mori_danwa_20000704` | 2,459 | `12a00e19615b…7a6b87` | raw capture |
| `jp_kantei_mori_hossoku_200007` | 1,669 | `cb24a6763d0d…bd0ebb` | raw capture |
| `jp_kunaicho_schedule_h12_q3` | 15,296 | `88ce38259793…009659` | raw capture |
| `jp_kantei_rekidai_085` | 15,218 | `6a8a7a23ea99…82f462` | raw capture |
| `jp_kantei_rekidai_086` | 34,418 | `324077a348a3…200baf` | raw capture |
| `jp_shugiin_giun_20010426` | 1,296 | `d3a7e58f21cd…01b2dc` | API speech |
| `jp_sangiin_giun_20010426` | 1,030 | `0c6a08900c54…557958` | API speech |
| `jp_shugiin_honkaigi_20010426` | 22,344 | `512a469c0699…2ca193` | API meeting record |
| `jp_sangiin_honkaigi_20010426` | 7,439 | `9f917b903f92…4d987b` | API meeting record |
| `jp_kantei_mori_jisyoku_danwa_20010426` | 2,033 | `b477e37b125f…736435` | raw capture |
| `jp_kantei_koizumi_danwa_20010426` | 4,011 | `c0f2f802afc6…35f0f5` | raw capture |
| `jp_kantei_koizumi_hossoku_20010426` | 3,391 | `ffb7aa82d8cd…9934f5` | raw capture |
| `jp_kunaicho_schedule_h13_q2` | 20,777 | `5a313b9a4567…ffc82e` | raw capture |
| `jp_kantei_rekidai_087` | 34,119 | `dc8766f8213e…0bf788` | raw capture |
| `jp_sangiin_giun_20031119` | 1,029 | `b554e96ce026…a8a6b6` | API speech |
| `jp_shugiin_honkaigi_20031119` | 77,918 | `896775625887…b6d41b` | API meeting record |
| `jp_sangiin_honkaigi_20031119` | 17,648 | `6b6f97e85854…9118fd` | API meeting record |
| `jp_kantei_koizumi_danwa_20031119` | 2,871 | `fcbdfea90e21…8b0b77` | raw capture |
| `jp_kantei_koizumi_sokaku_20031119` | 3,402 | `953f02c21c97…38a242` | raw capture |
| `jp_kunaicho_schedule_h15_q4` | 36,347 | `0023768004df…9e40a5` | raw capture |
| `jp_kantei_rekidai_088` | 25,446 | `ad807f920c91…59b327` | raw capture |
| `jp_sangiin_giun_20050921` | 1,029 | `a167c14524b5…20cb13` | API speech |
| `jp_shugiin_honkaigi_20050921` | 69,001 | `b1a742f442d2…001bd4` | API meeting record |
| `jp_sangiin_honkaigi_20050921` | 22,529 | `4aadcfe35db7…cd2732` | API meeting record |
| `jp_kantei_koizumi_danwa_20050921` | 4,115 | `9b31f5db4ef4…8505dc` | raw capture |
| `jp_kantei_koizumi_sokaku_20050921` | 4,098 | `1dcd06ccc171…aba390` | raw capture |
| `jp_kunaicho_schedule_2005_h2` | 46,090 | `cfe20d3e06fd…48129e` | raw capture |
| `jp_kunaicho_photo_20050921` | 1,557 | `8b6b000453f9…0604f9` | raw capture |
| `jp_kantei_koizumi_interview_20060925` | 4,954 | `8424abce518c…fae96c` | raw capture |
| `jp_shugiin_giun_20060926` | 1,507 | `cd1afdccf12a…db43b1` | API speech |
| `jp_sangiin_giun_20060926` | 1,029 | `7462608f59ed…8f24a6` | API speech |
| `jp_shugiin_honkaigi_20060926` | 26,138 | `adf156a277b0…ea2373` | API meeting record |
| `jp_sangiin_honkaigi_20060926` | 8,956 | `be1aa433233b…fce34b` | API meeting record |
| `jp_kantei_koizumi_sojishoku_danwa_20060926` | 3,182 | `68367c250409…00cb02` | raw capture |
| `jp_kantei_abe_danwa_20060926` | 4,910 | `8ae3f73ecc88…44d886` | raw capture |
| `jp_kantei_abe_hossoku_20060926` | 5,538 | `370ae14d206d…7f8bbf` | raw capture |
| `jp_kunaicho_schedule_2006_h2` | 49,890 | `c490d66f9c43…6e7554` | raw capture |
| `jp_kunaicho_photo_20060926` | 1,399 | `416637aeddc3…e64136` | raw capture |
| `jp_kantei_rekidai_089` | 25,746 | `7e877396999b…ff2942` | raw capture |

## Leads not imported

- House of Councillors research committee of 18 January 1990 (https://kokkai.ndl.go.jp/txt/111714330X00119900118): a
  member's reference to the sitting 海部内閣; superseded by the Cabinet's signed answer of 19 January 1990.
- House of Councillors plenary of 13 December 1989 (https://kokkai.ndl.go.jp/txt/111615254X01219891213): Kaifu answers as
  内閣総理大臣; the last in-office record found before 1 January 1990, outside the period.
- Further written answers: Kaifu's of 30 January and 6 February 1990 (House of Councillors t117002-t117004, e.g.
  https://www.sangiin.go.jp/japanese/joho1/kousei/syuisyo/117/touh/t117003.htm) and of 22 October 1991 (t121018),
  宮澤喜一's first of 3 December 1991 (t122001) and 細川護熙's of 27 August 1993 (t127001; also House of Representatives
  b127001): later corroboration, not needed.
- Ministers' statements of their own appointments on 28 February 1990 and 9 August 1993 (e.g.
  https://kokkai.ndl.go.jp/txt/111804006X00119900313): ministerial appointments are out of scope.
- The Kantei's 1998-1999 pages rekidai/kakuryo/76-79 (e.g.
  https://web.archive.org/web/19980128152510id_/http://www.kantei.go.jp:80/jp/rekidai/kakuryo/77.html) and souri/76, 78
  and 79: retrospective duplicates. The capture 19990422184900 of souri/76.html is EUC-JP, not Shift_JIS, and reads
  平元. 8.10〜平 2. 2.28 (203日); the misprint first reported does not reproduce (check A10). souri/78 (19990422190902) and
  souri/79 (19990117015734) are Shift_JIS.
- The Imperial Household Agency's 親任式 page (https://www.kunaicho.go.jp/learn/gokomu-kyuchu/shinninshiki/index.html):
  procedure only, and its dated list starts in 2022.
- The Kantei-hosted tables of contents of the Official Gazette of 7-8 November 1996 (km1107ee, km1107g1, km1108ee,
  km1108g1; e.g. https://web.archive.org/web/19970106204626id_/http://www.kantei.go.jp:80/jp/kanpo/nov.1/km1107ee.html):
  contents only, with no appointment text.
- 内閣総理大臣説示 of 7 November 1996 (setuji-1107) and 31 July 1998 (980731setuji): unnamed instructions, redundant with the
  agenda records.
- The Kantei's copies of Murayama's 18 July 1994 speech (murayama.html) and Hashimoto's 22 January 1996 speech
  (danwa-122), and the undated 小渕内閣閣僚名簿 (9807kakuryo/index.html): duplicates or undated.
- kaiken-1107 (7 November 1996, https://web.archive.org/web/19970106192630id_/http://www.kantei.go.jp:80/jp/kaiken-1107.html):
  橋本総理's congratulatory messages to President Clinton, an in-office act on 7 November that cannot be placed before or
  after the reappointment, so it dates neither Hashimoto holder.
- Rules and Administration Committee receipt times of the notices of 25 April 1994 (House of Representatives 10:21, House of
  Councillors 10:23; e.g. https://kokkai.ndl.go.jp/api/speech?speechID=112904024X01519940425_001&recordPacking=json),
  11 January 1996 (9:35 and 9:38) and 30 July 1998 (9:16 and 9:18): the plenary readings of the same notices are imported and
  no clock time is stored; the receipt records the checks confirmed (25 June 1994, 2001 and 2006) are imported.
- Ministers' and a member's recollections of 30 June and 28 April 1994 (found by full-text search): redundant with the
  holder's own statement.
- House of Representatives Agriculture Committee of 27 April 1994 (112905007X00319940427, speech 7): no ministers decided two
  days after the designation; context only, not a record of Hosokawa performing the duties.
- The House of Councillors' opening ceremony of 11 January 1996, listing 内閣総理大臣その他の国務大臣 among those present
  (inside `jp_hc_plenary_19960111`): names no one, so it is not recorded as a continuation claim.
- Obuchi's policy speech of 7 August 1998 (114305254X00319980807, speech 5): a later in-office attestation.
- The live 歴代内閣 pages 085-089: recorded only as captures made before the cutoff.
- The Kantei's 説示 of 5 April and 4 July 2000 (0405setuji, 0704setuji) and the cabinet lists of April and July 2000
  (0004-1kakuryo/index.html, 0007-1kakuryo/index.html): nothing beyond the imported accounts and profile.
- Aoki's House of Representatives answer of 11 April 2000 (114705254X02320000411, speech 16), the Cabinet Legislation Bureau's
  reading of art. 70 (114715261X01420000425, speech 435) and Aoki's House of Councillors answer of 17 May 2000
  (114715254X02520000517, speech 26): duplicates or procedure.
- The Imperial Household Agency's yearly attestation tables
  (https://web.archive.org/web/20061011022926id_/http://www.kunaicho.go.jp/04/d04-13-ninsyokan.html and d04-15-ninsyokan.html):
  monthly counts only; the dated schedules are imported instead (check C1).
- The Kantei's press conferences of 19 November 2003, 21 September 2005 and 26 September 2006 (19press, 21press and
  26press): not needed once the same-day statements and accounts were imported.
- The Constitution (arts. 6, 67, 70 and 71) and the Diet Act (arts. 64 and 65) frame the procedure; constitution text
  establishes procedure only, never a date. No news, encyclopaedia or history site is used.

## Sources attempted

- The Official Gazette (官報), 1990-2006: not available without restriction. An NDL Digital Collections search (title 官報,
  volume numbers included, every access level) returns 332 items for 1950, a positive control, and no gazette issue for
  1990-1993; post-1952 issues are restricted there, and kanpo.go.jp is free only for the latest 90 days. The paid
  官報情報検索サービス and the NDL's registered-user services were not used. The NDL Search OpenSearch query first recorded
  (`dpid=digidepo`) returns 0 for every year, 1930-1952 included, so it proves nothing (check A8). No appointment notice
  (叙任及び辞令) was read.
- The Imperial Household Agency: its website begins in late 1999, and `/page/gonittei/` returns 404; the dated schedules of
  2000-2006 were found through the agency's own index (gonitei/gonitei-00.html) in the Internet Archive (check C1); nothing
  was found for 1990-1998.
- The Diet minutes API, searched for 親任式, 親任, 任命, 認証式, 職務執行内閣, 七十一条, 引き続き職務 and 臨時代理 with ending
  words around every transition: no record of an appointment ceremony in 1990-1993, of continued performance of duties
  under art. 71, of the end of Aoki's acting service or of the day any office ended; no House of Representatives record of
  the 7 November 1996 notice (its Rules Committee of that day, 113804024X00119961107, does not mention it); no earlier Diet
  attestation of Hata (26 April-9 May 1994) or Obuchi (30 July-6 August 1998).
- The House of Councillors' table at its old URL (under `/japanese/…/houki/09senrei/`): 404 since the site moved; the
  capture and the relocated file are identical.
- The old Kantei list (rekidai/ichiran.html): redirects to an index of NDL WARP archives of past Kantei sites; its 1998
  capture failed once and was not needed.
- The House of Representatives' written-question list for the 117th session (kaiji117_l.htm): 404; the answers were
  fetched by identifier.
- The Kantei archive: no page of April 2000 on the acting Prime Minister, no photo page of 26 September 2006 for Koizumi and
  no 1998 Gazette capture for 30-31 July.
- The Internet Archive's CDX index: wildcard listings timed out (504) and were narrowed; some captures failed once with a
  connection error and succeeded on a single retry.
- No terms, logins, CAPTCHAs or access controls were met or bypassed.

## Checker defects

| # | Defect | Outcome |
|---|---|---|
| A1 | The 9 April 1990 remark was addressed to a minister, not the Prime Minister | **Applied**: reworded; 市川雄一 questions Minister 深谷隆司 (speech 34) and the Prime Minister answers next; still a member's retrospective remark, never a start |
| A2 | The 27 February 1990 notice was received by the House, addressed to the President | **Applied**: "the House received a notice ... addressed to the President"; the 09:45 time tied to the receipt; the same wording used in every Secretary General's report of 1991-2006 |
| A3 | The House of Councillors' table folds both Houses' designations into one claim | **Applied**: each of the three rows split into two claims by the table's 参議院の指名 and 衆議院の指名 columns (a `column` locator; kinds `retrospective_designation_record_house_of_councillors` and `..._house_of_representatives`) |
| A4 | Two rows name 細川護熙 (熙) while the holder is 細川護煕 (煕) | **Applied**: every row naming him carries 細川護煕; the printed 熙 stays in the text and uncertainty; one holder string, tested |
| A5 | The 1990 acting Prime Minister's row would carry a holder name | **Applied**: `holder_name` is null on every acting row (1990 and 2000); 橋本龍太郎 is named in the text; tested |
| A6 | The first Kaifu Kantei claim merges the span with a pre-period formation | **Applied**: the formation half dropped; likewise the formation lines of the 85th-89th pages, whose formations the contemporaneous Kantei accounts date |
| A7 | The span claims carry `attested_on: null` | **Applied**: no `attested_on` key on all nine spans (and on every retrospective list and table row); each uncertainty says no structured date is stored |
| A8 | The NDL Search query is not evidence of absence | **Applied**: replaced by the NDL Digital Collections search with its 1950 positive control (Sources attempted); nothing hashed |
| A9 | The table's "404" is a relocation | **Applied**: the relocated live file recorded as `alternate_location` with the same bytes and SHA-256, downloaded again by this packet plain and with a cache-busting query; the capture stays the recorded identity |
| A10 | The 1998 Kantei lead's misprint does not reproduce | **Applied**: the remark dropped; the lead names capture 19990422184900 (EUC-JP) and the Shift_JIS pages |
| A11 | The holder shape differs from the C01-11 model | **Applied**: holders have exactly `name`, `attested_on`, `from`, `until`, `sources`, `claim_ids`, `note`, `uncertainty`; romanized forms are in this report; supporting claims stay on the role |
| B1 | Hashes are of identity-encoded bodies, not stated | **Applied**: every extract records `source_response_content_encoding` `identity`, and its provenance note says the body was requested without `Accept-Encoding`; every capture records `archive_capture_utc` |
| B2 | A claim carries the withdrawn `attested_period` | **Applied**: removed; the recollection has no structured date; tested |
| B3 | The Kantei list headings carry `attested_on` | **Applied**: no structured date on the five cabinet-list claims, and, for the same reason, on the part A 歴代内閣 formation lines and the House of Councillors' table rows |
| B4 | "reappointment_recalled_by_holder" says more than the press conference | **Applied**: kind `designation_recalled_by_holder`; the text quotes the designation, continuing in office and forming the cabinet |
| B5 | The photo-diary kind and capture | **Applied**: kind `dated_photo_caption`; recorded from the 2003 capture (20030424005116), byte-identical with a matching CDX digest |
| B6 | Hashimoto's second start rests on an unnamed statement | **Resolved** by the second option: the second holder is observed on 8 November 1996 from a new in-office claim on the named press conference (`jp_kantei_hashimoto_press_conference_in_office_19961108`); the 7 November statement is a claim with no holder name; JP-PM-06 accepted in part |
| B7 | The holder claim sets break the C01-11 pattern | **Applied**: holders cite only same-day claims that name them (starts and attestations); lists, recollections, designations, resignations, unnamed statements and non-holders stay on the role; the two 菅直人 rows carry no holder name; one event-kind vocabulary, pinned in the new test |
| B8 | Obuchi's spoken "today" treated differently from Hashimoto's statement without a reason | **Applied**: the reason is in the claim's uncertainty, the holder's note and this report; no date changes |
| B9 | The 11 January 1996 page's first cabinet meeting was not recorded | **Applied**: `jp_kantei_first_cabinet_meeting_19960111` added on the role; the page's original Last-Modified (17 January 1996) cited |
| B10 | The JP-PM-07 summary overstates the report to the Emperor | **Applied**: only the House of Representatives' Speaker announced 奏上; stated in the claim and this report |
| B11 | Extract phrases the C01-11 tests require | **Applied**: every provenance note carries "not checked into this repository", "derived factual extract", "same byte count and SHA-256" and, for captures, "Raw Internet Archive capture", with the character encoding recorded |
| C1 | The Imperial Household Agency's dated schedules were missed | **Applied**: eight captures imported as `imperial_appointment_ceremony` claims; the named 2005 photo page is cited by the 2005 Koizumi holder; the unnamed schedule rows and the Abe photo page stay on the role (a holder claim names its holder, check B7) and each holder's note cites them; the "no dated record" statements removed |
| C2 | Nine claims give a full name the source does not print | **Applied, adapted**: each text now shows the printed form (森内閣総理大臣演説等, 森総理, 小泉総理 and so on) and the uncertainty says where the full name comes from; where a source prints no name at all the row's `holder_name` is null; where it prints a surname the row keeps the holder's single full-name string, as check A4 and the C01-11 rule that every non-null row names a holder require |
| C3 | The incapacity claim folds in the 19:00 instruction | **Applied**: `jp_obuchi_instruction_to_aoki_1900_20000402` split out (claim only); the intensive-care wording corrected |
| C4 | Two acting claims each hold two events | **Applied**: the notices of the acting designation split out (`jp_aoki_reports_acting_designation_notified_20000403`, `jp_acting_designation_notice_sent_20000403`) |
| C5 | Spans use `retrospective_span` with a `stated_span` object | **Applied**: `retrospective_term_span`, no structured date and no `stated_span` |
| C6 | Receipt times missing for 2001 and 2006 | **Applied**: the four Rules and Administration Committee records imported (09:21 and 09:23; 09:16 and 09:19), times in the text only |
| C7 | A full-text search URL is recorded | **Applied**: replaced by speeches 35 and 36 as two sources, the claim split into the question and the answer |
| C8 | The 087-089 pages' header also declares UTF-8 | **Applied**: every extract records the declarations found (meta tag and archived header; 078 and 087-089 declare UTF-8 in both) |
| C9 | The part C summary swaps the source counts | **Applied**: 19 API responses and 21 captures, as the dossier holds |

Missing primary records the checks found: of part A's six, five are imported (the House of Councillors' Rules Committee of
5 November 1991, the House of Councillors' records of the policy speeches of 2 March 1990, 8 November 1991 and 23 August
1993, and the two remarks of 24 August 1993 as claims in an existing source) and the sixth, the relocated table, is recorded
as the capture's alternate location. Part B's six are imported: the three agriculture-committee records of 6 July 1994, the
House of Representatives' record of 10 May 1994, the House of Councillors' receipt of 25 June 1994, and the 2003 diary
capture in place of the 2000 capture. Part C's sixteen are imported: the eight Imperial Household Agency captures, the four
Rules and Administration Committee records of 2001 and 2006, the House of Representatives' reading of the 4 April 2000
notice, Aoki's answer of 26 April 2000, and speeches 35 and 36 of 3 April 2000 in place of the search URL. This packet
downloaded each of them again three times (plain, cache-busting and plain at least 30 minutes after the first), matching
the checks' identities.

Other changes made to fit the packet's rules rather than a numbered defect:

- One event-kind vocabulary for all parts: notices and readings of a resignation en masse are `cabinet_resignation_notice`,
  cabinet accounts of the decision `cabinet_resignation_decided`, the Prime Ministers' statements
  `cabinet_resignation_statement` and agenda items `cabinet_resignation_agenda_item`; each House's vote is
  `designation_vote_house_of_…` and a first ballot without a majority `designation_ballot_no_majority_house_of_…`; Kantei
  statements of the day are `appointment_statement` or `assumption_statement` (the C01-11 name) whether or not they name the
  speaker; acting service is `acting_prime_minister_…`; recollections end in `_recalled` or `_recalled_by_holder`.
- Members' contemporaneous references to the sitting Prime Minister (6 July 1994) are `in_office_reference_by_member`,
  claims only.
- Resignation uncertainties say that art. 71 is procedure only and that no source states a continuation, instead of
  asserting that the cabinet continued.
- `published_date` is null for every new source (see the date conventions).

## Suggested next work orders

These are proposals for the integrator. They are not created in `work-orders.json`.

- `C01-Japan-PM-002`: the Imperial appointment ceremonies and Official Gazette appointment notices (叙任及び辞令) of
  1990-2006, from lawful access to the post-1952 Gazette (an NDL reading room or the paid Gazette search).
- `C01-Japan-PM-003`: the ends of office and any continued performance of duties under art. 71, 1990-2006.
- `C01-Japan-PM-004`: prime ministers from 26 September 2006 to the 7 September 2026 cutoff.
- `C01-Japan-PM-005`: acting prime ministers (臨時代理) and the designated order of acting ministers, 1990-2026.

## Integration notes (outside this packet's file boundary)

- **Base and claim:** based on `ffe54b02` (`codex/campaign-certification`); claim commit `c2aafd22` holds only the
  handoff. No other pending packet touches `japan.json`.
- `research-index.json` is regenerated in a **separate commit**. New totals against `ffe54b02`: 280 sources and 1,936 claims
  (previously 161 and 1,762), 28 institution observations (previously 27). Japan now has eight institutions, five role
  observations and 24 entries pending mapping; its three discovery batches become 10, 10 and 4 members, so the total of 92
  batches is unchanged. The pending India (CLAUDE-C01-11), Brazil (CLAUDE-C01-10) and South Africa (CLAUDE-C01-09) packets
  add their own sources and claims to other packets; the index must be regenerated after they merge.
- Pinned tests in `test_japan_research_s10d.py`, none loosened and no assertion removed:
  - counts (entries, sources, claims, roles) are now (24, 126, 203, 5), from (23, 7, 29, 4), and the original seven sources
    and 29 claims are pinned by id;
  - the seven parliamentary-group assertions apply to exactly the seven group ids, and the institution set is pinned to
    those seven plus `jp_prime_minister`, whose one role is `jp_pm`;
  - the 2026-09-13 access date still applies to the original seven sources by id, and the 119 new sources are pinned to
    2026-09-24 (their exact identities are pinned in `test_japan_prime_ministers_c01_12.py`);
  - the extract count is 2 plus 119, `mapping_pending` is 24 (from 23) and the Japan work orders are 10, 10 and 4 members
    (from 3), with the last batch pinned.
- `test_campaign_census` needs `spheres-sim/data`, which is absent from this sparse worktree; it was not run, and the sparse
  checkout was not widened.
- The atlas (`tools/ui/leadership-research-review.js`) shows only "Observed on" when a holder has `attested_on`; no holder
  here has both `attested_on` and `until`, so no end is hidden. No UI code changed.
- New fields on Japan sources: `original_url`, `source_type` and `scope_note`; extracts carry `review_observation`,
  `archive_capture_utc`, `diet_record` (issue, House, meeting, sitting date and readable page),
  `source_response_content_encoding`, `source_character_encoding`, `alternate_location` (the table) and `visual_review`. The
  event-kind vocabulary and every row's holder name are pinned in the new test.
- `research/README.md`, the C01 README totals and `docs/planning/ai-workstreams.json` are left for the integrator; this
  handoff is self-proposed and not registered there.

## Checks

```text
python -X utf8 tools/avatars/campaign_research.py
python -X utf8 tools/avatars/campaign_research.py --check
python -X utf8 -m unittest discover -s tools/avatars -p "test_japan*.py"
python -X utf8 -m unittest discover -s tools/avatars -p "test_*research*.py"
python -X utf8 -m unittest discover -s tools/avatars -p "test_campaign*.py"
node --test tools/ui/check_leadership_research_review.cjs
python tools/planning/workboard.py --check
git diff --check
```

Results are recorded in the handoff.
