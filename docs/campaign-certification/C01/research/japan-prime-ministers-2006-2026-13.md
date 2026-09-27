# Japanese prime ministers 13: holders and transitions, 2006-2026

Packet: **CLAUDE-C01-13**. State: **ready_for_review** (not complete).
Owner: Claude. Integrator/reviewer: Codex. Branch `claude/c01-jp-13`, **stacked on CLAUDE-C01-12** (`claude/c01-jp-12` up to `da358dbf`, merged into this branch in `b0e3c5b5`), which is based on `codex/campaign-certification` at `ffe54b02`; claim commit
`3bcd6e6c`. Research access: 24-25 September 2026 (UTC). The historical cutoff stays **7 September 2026**.

This packet continues CLAUDE-C01-12 on the same institution and role in [japan.json](japan.json): `jp_prime_minister` and
`jp_pm` (内閣総理大臣 — Prime Minister of Japan). It adds no institution, role, organization, game mapping, lifespan,
portrait or avatar, and it changes no existing observation, holder, claim or source: the fourteen 1990-2006 holders, the
LDP presidency observations (石破茂, 高市早苗) and the original sources stay as they were. It reviews ten observations from
安倍晋三's appointment of 26 September 2006, which C01-12 records only as its closing boundary, to the cutoff, and adds
211 sources, 293 claims and sixteen holder observations, with a scope-note addition, three institution coverage
notes and one packet coverage note. The parent scope (C01, C06, S23, WC1 and CP1) remains open.

The research was done in three parts (A: 2006-2010; B: 2010-2020; C: 2020-2026), and each part was checked independently
before this packet was written. Every checker defect is applied or resolved (see [Checker defects](#checker-defects)), and
every missing primary record the checks confirmed is imported, except seventeen redundant ones that are listed as leads with
the reason.

## Outcome

An observation is **Accepted** where every holder in it has a stated start and every appointment that ended before the
cutoff has a stated end; otherwise it is **Accepted in part**.

| ID | Question | Decision |
|---|---|---|
| JP-PM06-01 | 2006-2007: 安倍晋三's designation and appointment, and his cabinet's resignation | **Accepted in part:** from 26 Sep 2006 (a named Kantei statement, the Kantei account of the 親任式 and the agency's photo page, re-recorded with named rows); resignation announced 12 Sep 2007, hospital 13 Sep, no acting Prime Minister designated, resignation en masse 25 Sep 2007; no source states the day his office ended |
| JP-PM06-02 | 2007-2008: 福田康夫's designation, appointment and resignation | **Accepted in part:** the House of Councillors designated 小沢一郎, the joint committee failed and the House of Representatives' designation prevailed on 25 Sep 2007; from 26 Sep 2007 (ceremony at 08:30); resignation en masse 24 Sep 2008; no stated end |
| JP-PM06-03 | 2008-2009: 麻生太郎's designation, appointment and resignation | **Accepted in part:** the same art. 67(2) sequence on 24 Sep 2008 and from that day (ceremony 深夜); resignation en masse 16 Sep 2009 (only the House of Councillors' receipt is recorded); no stated end |
| JP-PM06-04 | 2009-2010: 鳩山由紀夫's designation, appointment and resignation | **Accepted in part:** both Houses designated him on 16 Sep 2009 and from that day; resignation announced 2 Jun 2010 (the Chief Cabinet Secretary's same-day record), resignation en masse 4 Jun 2010; the end conflicts (4 June against continued duties to 8 June) and is left unresolved |
| JP-PM06-05 | 2010-2011: 菅直人's designation, appointment and resignation | **Accepted in part:** designated 4 Jun 2010, from 8 Jun 2010; resignation en masse 30 Aug 2011 and continued duties to 2 Sep 2011 (claims only); no stated end |
| JP-PM06-06 | 2011-2012: 野田佳彦's designation, appointment and resignation | **Accepted in part:** designated 30 Aug 2011 (the House of Councillors in a runoff), from 2 Sep 2011; resignation en masse 26 Dec 2012; no stated end |
| JP-PM06-07 | 2012-2020: 安倍晋三's appointment of December 2012, his re-designations of 2014 and 2017 and his 2020 resignation | **Accepted in part:** from 26 Dec 2012, 24 Dec 2014 and 1 Nov 2017; the 2017 appointment ended on 16 Sep 2020 (Official Gazette); no stated end for the 2012 and 2014 appointments |
| JP-PM06-08 | 2020-2021: 菅義偉's designation, appointment and resignation | **Accepted:** from 16 Sep 2020 and until 4 Oct 2021, both stated by the Official Gazette's notices |
| JP-PM06-09 | 2021-2024: 岸田文雄's appointment of October 2021, his November 2021 re-designation and his 2024 resignation | **Accepted:** from 4 Oct 2021 until 10 Nov 2021, and from 10 Nov 2021 until 1 Oct 2024 |
| JP-PM06-10 | 2024-2026: 石破茂's two designations, 高市早苗's designation and appointment, and the holder in office before the cutoff | **Accepted:** 石破茂 from 1 Oct 2024 until 11 Nov 2024 and from 11 Nov 2024 until 21 Oct 2025; 高市早苗 from 21 Oct 2025 until 18 Feb 2026 and from 18 Feb 2026, in office on 27 Jul and 4 Sep 2026 |

The resulting holder observations of `jp_pm` added here, in date order (they follow C01-12's fourteen):

| Holder | `attested_on` | `from` | `until` | Basis |
|---|---|---|---|---|
| 安倍晋三 | null | 2006-09-26 | null | 安倍内閣総理大臣談話 (本日、内閣総理大臣に任命され), the Kantei account of the 親任式 and the agency photo page, re-recorded with named rows; in office 24 Sep 2007 |
| 福田康夫 | null | 2007-09-26 | null | Kantei account and Chief Cabinet Secretary record (ceremony 08:30), agency photo page 親任式（福田内閣総理大臣） |
| 麻生太郎 | null | 2008-09-24 | null | Kantei account (ceremony 深夜), agency photo page 親任式（麻生内閣総理大臣） |
| 鳩山由紀夫 | null | 2009-09-16 | null | Kantei account (ceremony that night), agency photo page 親任式（鳩山内閣総理大臣） |
| 菅直人 | null | 2010-06-08 | null | agency photo page 親任式（菅内閣総理大臣）, Kantei account; in office 11 Jun 2010 |
| 野田佳彦 | null | 2011-09-02 | null | agency photo page, Kantei account, his press conference (本日…正式に内閣総理大臣に就任); in office 13 Sep 2011 |
| 安倍晋三 | null | 2012-12-26 | null | agency photo page, Kantei account, statement and press conference (本日…拝命); in office 28 Jan 2013 |
| 安倍晋三 | null | 2014-12-24 | null | agency photo page, Kantei account, press conference (本日…引き続き…重責を担う); in office 12 Feb 2015 |
| 安倍晋三 | null | 2017-11-01 | 2020-09-16 | agency photo page, Kantei account, press conference; until: Gazette No. 99 p. 2 (退官) |
| 菅義偉 | null | 2020-09-16 | 2021-10-04 | Gazette No. 99 p. 2, Kantei account, agency entry, first cabinet meeting minutes; until: Gazette No. 83 p. 2 |
| 岸田文雄 | null | 2021-10-04 | 2021-11-10 | Kantei account, agency entry, Gazette No. 83, minutes; until: Gazette No. 88 p. 2 |
| 岸田文雄 | null | 2021-11-10 | 2024-10-01 | Kantei account, agency entry, Gazette No. 88, minutes; until: Gazette No. 45 p. 2 |
| 石破茂 | null | 2024-10-01 | 2024-11-11 | Gazette No. 45, Kantei account, minutes, press conference; until: Gazette No. 52 p. 2 |
| 石破茂 | null | 2024-11-11 | 2025-10-21 | Kantei account, Gazette No. 52, minutes, press conference; until: Gazette No. 28 p. 2 |
| 高市早苗 | null | 2025-10-21 | 2026-02-18 | Kantei account, Gazette No. 28, minutes, press conference; until: Gazette No. 9 p. 2 |
| 高市早苗 | null | 2026-02-18 | null | Kantei account, Gazette No. 9, minutes, press conference; in office 27 Jul and 4 Sep 2026 |

The part C researcher proposed the second 高市早苗 appointment (18 February 2026), which the handoff did not list. It is
included in JP-PM06-10 because the holder in office at the cutoff holds that appointment, and the handoff asks for the holder
in office before the cutoff.

### How a start and an end are decided

C01-12's rule applies unchanged. A holder has `from` only where a same-day source that names the holder states the day of
appointment or assumption of office, and `until` only where a source states the day the office ended; otherwise a holder is
dated by `attested_on`. Every holder here has a stated start: a Kantei account of the Imperial appointment ceremony that names
the holder, the Imperial Household Agency's photo page or entry captioned 親任式（…内閣総理大臣）, a same-day statement or
press conference ("本日…"), and from 2020 the Official Gazette's appointment notice and the Cabinet's minutes of that night's
first meeting, which name the Prime Minister present and record his or her statement that the appointment was made that day.

Ends come only from the Official Gazette. Each special extra issue of 2020-2026 prints, under ○内閣総理大臣及び国務大臣退官,
that on the day of the month it states (本月X日) the new Prime Minister was appointed and the outgoing Prime Minister and the
ministers each lost their office (それぞれその地位を失った). That is a source stating the day an office ended, not an end
inferred from a successor's start, so seven holders have `until`: 安倍晋三 (16 September 2020), 菅義偉 (4 October 2021),
岸田文雄 (10 November 2021 and 1 October 2024), 石破茂 (11 November 2024 and 21 October 2025) and 高市早苗 (18 February
2026). Where the outgoing and incoming Prime Minister are the same person (2021, 2024 and 2026), the notice names both, and
the earlier appointment ends on the day of the new one. The Gazette of 2006-2017 is not available without restriction, so the
earlier appointments have no end.

Refinements that apply as in C01-12:

- **A statement or record that prints no name never makes a start by itself.** The Kantei's statements of 26 September 2007
  and of 2020-2026 print no name; their rows carry no holder name. The minutes of the first cabinet meetings of 2020-2026 name
  the Prime Minister and record the same statements, and those named rows are the ones the holders cite. The Imperial
  Household Agency's schedule rows and its 2024-2026 entries that print 親任式（内閣総理大臣） without a name are likewise
  claims on the role.
- **A spoken "today" that its own records contradict is not an end.** Hatoyama's remarks at the last cabinet meeting of
  4 June 2010 say that "today" he leaves the post, and the Chief Cabinet Secretary gave the cabinet's span as ending "today,
  4 June"; but a member's question and the successor's answer of 15 June 2010 say the resigned cabinet continued its
  functions until the ceremony of 8 June, and the Kantei's retrospective span runs to 8 June. The conflict is recorded and no
  end is taken.
- **"拝命" or "指名" spoken before the ceremony is not a start.** 福田康夫's press remark of 25 September 2007 and 菅直人's of
  8 June 2010 were made before the ceremony; they are a designation recollection and a prospective statement.
- **"このたび…任命され" in a policy speech states an appointment without a day;** it dates an in-office attestation, never a
  start.

Refused as ends: resignations en masse and their notices, announced intentions to resign (12 September 2007, 1 September
2008, 2 June 2010, 26 August 2011, 28 August 2020), 安倍晋三's "tomorrow" of 24 September 2007, 福田康夫's unnamed "I have
resigned" and 麻生太郎's "leaving office" (no day), departures from the Prime Minister's Office (2 September 2011, 26 December
2012, 16 September 2020), continued performance of duties, the Chief Cabinet Secretary's span of 4 June 2010, and the
Kantei's retrospective spans. Continued performance of duties (4-8 June 2010 and 30 August-2 September 2011, and the
prospective emergency arrangements of 2008, 2010, 2021, 2024 and 2025) and the Cabinet Act art. 9 orders of acting ministers
(2007 and 2008) are claims only, and no acting Prime Minister served in this period: 安倍晋三 stated on 24 September 2007 that
none was designated during his hospitalisation.

Names follow the sources in Japanese, one holder string per person (安倍晋三, 福田康夫, 麻生太郎, 鳩山由紀夫, 菅直人, 野田佳彦,
菅義偉, 岸田文雄, 石破茂, 高市早苗). Where a source prints only a surname and title (安倍総理, 鳩山内閣総理大臣, 菅内閣総理大臣),
the claim text keeps that form and the row carries the holder's full name, as C01-12 did; 菅直人 and 菅義偉 share the surname 菅
and are told apart by date and by the full names printed in the same day's records. A row's `holder_name` is null where the
source names no holder of this role: an unnamed statement or schedule row, the House of Councillors' designee 小沢一郎 (2007
and 2008), a designation of acting ministers or the statement that none was designated, or the Chief Cabinet
Secretary's 2008 emergency arrangement. Romanized forms:
安倍晋三 Abe Shinzō, 福田康夫 Fukuda Yasuo, 麻生太郎 Asō Tarō, 鳩山由紀夫 Hatoyama Yukio, 菅直人 Kan Naoto, 野田佳彦 Noda
Yoshihiko, 菅義偉 Suga Yoshihide, 岸田文雄 Kishida Fumio, 石破茂 Ishiba Shigeru and 高市早苗 Takaichi Sanae.

### The 2006 appointment and the stacked packet

C01-12 records the three same-day records of 安倍晋三's appointment (the statement 安倍内閣総理大臣談話, the Kantei account and
the agency's photo page) as its closing boundary, with rows that carry no holder name, and its extracts are outside this
packet's file boundary. This packet re-records the same three responses under new source ids
(`jp_kantei_abe_danwa_named_20060926`, `jp_kantei_abe_hossoku_named_20060926`, `jp_kunaicho_photo_abe_named_20060926`),
with the same URLs, byte counts and SHA-256, so that rows naming the holder can support the 2006 start; the C01-12 claims
stay on the role and are never cited by a holder. This is the only repeated response identity in the packet and is pinned in
the test. The integrator may instead amend C01-12's three rows to name 安倍晋三 and drop the three re-recorded sources (see
the integration notes).

### Date ledger

Each row is a separate dated fact with its own claim; two facts on one day stay two claims.

| Date | Events | Claims |
|---|---|---|
| 26 Sep 2006 | statement of appointment (安倍晋三); Imperial appointment ceremony (親任式) (安倍晋三) | `jp_abe_named_statement_appointed_pm_20060926`, `jp_abe_named_shinninshiki_20060926`, `jp_kunaicho_ceremony_abe_named_20060926` |
| 12 Sep 2007 | intention to resign announced (安倍晋三); announcement recalled (安倍晋三) | `jp_abe_announces_resignation_decision_20070912`, `jp_member_recalls_abe_resignation_announcement_20070912` |
| 13 Sep 2007 | hospital examination reported (安倍晋三); hospitalisation recalled (安倍晋三) | `jp_ccs_reports_abe_hospital_examination_20070913`, `jp_abe_hospitalised_since_20070913` |
| 24 Sep 2007 | in office (安倍晋三); no acting Prime Minister designated; 'tomorrow' remark (安倍晋三) | `jp_abe_in_office_press_conference_20070924`, `jp_abe_no_acting_pm_designated_20070924`, `jp_abe_states_he_will_leave_office_tomorrow_20070924` |
| 25 Sep 2007 | resignation statement (安倍晋三); resignation en masse decided (安倍晋三); notice of resignation en masse (安倍晋三); House of Representatives designates (福田康夫); joint committee requested; House of Representatives' designation prevails (art. 67(2)) (福田康夫); House of Councillors first ballot without a majority (福田康夫); House of Councillors designates; House of Representatives' designation prevails (art. 67(2)); joint committee: no agreement; designation recalled by the designee (福田康夫); ceremony scheduled | `jp_abe_cabinet_resigned_statement_20070925`, `jp_abe_cabinet_resignation_decided_20070925`, `jp_shugiin_rules_receives_abe_resignation_notice_20070925`, `jp_sangiin_rules_receives_abe_resignation_notice_20070925`, `jp_shugiin_receives_abe_cabinet_resignation_notice_20070925`, `jp_shugiin_designates_fukuda_20070925`, `jp_shugiin_receives_joint_committee_request_20070925`, `jp_shugiin_hr_resolution_prevails_fukuda_20070925`, `jp_sangiin_notice_abe_cabinet_resigns_20070925`, `jp_sangiin_first_ballot_no_majority_20070925`, `jp_sangiin_designates_ozawa_runoff_20070925`, `jp_sangiin_joint_committee_requested_20070925`, `jp_sangiin_hr_resolution_prevails_20070925`, `jp_joint_committee_no_agreement_20070925`, `jp_ccs_reports_abe_cabinet_resignation_decided_20070925`, `jp_fukuda_says_appointed_on_eve_20070925`, `jp_ccs_schedules_fukuda_ceremony_20070925` |
| 26 Sep 2007 | Imperial appointment ceremony (親任式) (福田康夫); cabinet launched (福田康夫); statement of appointment; Imperial appointment ceremony (親任式); art. 9 acting order designated | `jp_fukuda_shinninshiki_appointed_20070926`, `jp_fukuda_cabinet_formed_20070926`, `jp_kantei_pm_statement_appointed_20070926`, `jp_kunaicho_ceremony_20070926`, `jp_kunaicho_ceremony_fukuda_20070926`, `jp_ccs_reports_fukuda_ceremony_0830_20070926`, `jp_ccs_reports_fukuda_acting_order_designated_20070926` |
| 1 Sep 2008 | intention to resign announced (福田康夫) | `jp_fukuda_announces_resignation_decision_20080901` |
| 24 Sep 2008 | resignation statement; resignation en masse decided (福田康夫); notice of resignation en masse (福田康夫); House of Representatives designates (麻生太郎); joint committee requested; House of Representatives' designation prevails (art. 67(2)) (麻生太郎); House of Councillors first ballot without a majority (麻生太郎); House of Councillors designates; House of Representatives' designation prevails (art. 67(2)); continued-duties arrangement; joint committee: no agreement; Imperial appointment ceremony (親任式) (麻生太郎); cabinet launched (麻生太郎); Imperial appointment ceremony (親任式); formation recalled by the holder (麻生太郎) | `jp_kantei_pm_statement_resigned_20080924`, `jp_fukuda_cabinet_resignation_decided_20080924`, `jp_shugiin_rules_receives_fukuda_resignation_notice_20080924`, `jp_sangiin_rules_receives_fukuda_resignation_notice_20080924`, `jp_shugiin_receives_fukuda_cabinet_resignation_notice_20080924`, `jp_shugiin_designates_aso_20080924`, `jp_shugiin_receives_joint_committee_request_20080924`, `jp_shugiin_hr_resolution_prevails_aso_20080924`, `jp_sangiin_notice_fukuda_cabinet_resigns_20080924`, `jp_sangiin_first_ballot_no_majority_20080924`, `jp_sangiin_designates_ozawa_runoff_20080924`, `jp_sangiin_joint_committee_requested_20080924`, `jp_sangiin_hr_resolution_prevails_20080924`, `jp_ccs_reports_fukuda_cabinet_resignation_decided_20080924`, `jp_ccs_arranges_emergency_ministers_until_attestation_20080924`, `jp_joint_committee_no_agreement_20080924`, `jp_aso_shinninshiki_appointed_20080924`, `jp_aso_cabinet_formed_20080924`, `jp_kunaicho_ceremony_20080924`, `jp_kunaicho_ceremony_aso_20080924`, `jp_aso_recalls_cabinet_launched_20080924` |
| 25 Sep 2008 | art. 9 acting order designated | `jp_ccs_reports_aso_acting_order_designated_20080925` |
| 16 Sep 2009 | resignation statement (麻生太郎); resignation en masse decided (麻生太郎); notice of resignation en masse (麻生太郎); House of Representatives designates (鳩山由紀夫); House of Councillors designates (鳩山由紀夫); Imperial appointment ceremony (親任式) (鳩山由紀夫); cabinet launched (鳩山由紀夫); Imperial appointment ceremony (親任式); ceremony scheduled (鳩山由紀夫) | `jp_aso_states_cabinet_resigned_20090916`, `jp_aso_cabinet_resignation_decided_20090916`, `jp_sangiin_rules_receives_aso_resignation_notice_20090916`, `jp_shugiin_designates_hatoyama_20090916`, `jp_sangiin_designates_hatoyama_20090916`, `jp_hatoyama_shinninshiki_appointed_20090916`, `jp_hatoyama_cabinet_formed_20090916`, `jp_kunaicho_ceremony_20090916`, `jp_kunaicho_ceremony_hatoyama_20090916`, `jp_ccs_schedules_hatoyama_ceremony_20090916` |
| 2 Jun 2010 | intention to resign reported (鳩山由紀夫); continued-duties arrangement (鳩山由紀夫) | `jp_ccs_reports_hatoyama_resignation_announcement_20100602`, `jp_ccs_government_to_continue_until_successor_20100602`, `jp_hatoyama_transition_instruction_20100602` |
| 4 Jun 2010 | last cabinet meeting remarks (鳩山由紀夫); statement index (鳩山由紀夫); resignation statement (鳩山由紀夫); resignation en masse decided (鳩山由紀夫); notice of resignation en masse (鳩山由紀夫); House of Representatives designates (菅直人); House of Councillors designates (菅直人); designation recalled by a member (菅直人); Kantei account of the designation (菅直人) | `jp_hatoyama_last_cabinet_meeting_remarks_20100604`, `jp_kantei_index_lists_hatoyama_resignation_items_20100604`, `jp_hatoyama_cabinet_resigned_statement_20100604`, `jp_hatoyama_cabinet_resignation_decided_20100604`, `jp_shugiin_rules_receives_hatoyama_resignation_notice_20100604`, `jp_sangiin_rules_receives_hatoyama_resignation_notice_20100604`, `jp_shugiin_receives_hatoyama_cabinet_resignation_notice_20100604`, `jp_shugiin_designates_kan_20100604`, `jp_sangiin_notice_hatoyama_cabinet_resigns_20100604`, `jp_sangiin_designates_kan_20100604`, `jp_ccs_reports_hatoyama_cabinet_resignation_decided_20100604`, `jp_ccs_reports_hatoyama_final_cabinet_meeting_remarks_20100604`, `jp_member_recalls_kan_designation_20100604`, `jp_kantei_kan_designated_94th_20100604` |
| 8 Jun 2010 | presiding officers' report to the Emperor; Imperial appointment ceremony (親任式); Imperial appointment ceremony (親任式) (菅直人); to take office that evening (prospective) (菅直人); cabinet launched (菅直人); ceremony scheduled (菅直人) | `jp_kunaicho_speakers_report_20100608`, `jp_kunaicho_ceremony_20100608`, `jp_kunaicho_ceremony_kan_20100608`, `jp_kan_press_conference_to_assume_office_that_evening_20100608`, `jp_kan_shinninshiki_appointed_20100608`, `jp_kan_cabinet_formed_20100608`, `jp_ccs_schedules_kan_ceremony_20100608` |
| 11 Jun 2010 | in office (菅直人) | `jp_kan_policy_speech_bears_office_20100611` |
| 26 Aug 2011 | intention to resign announced (菅直人) | `jp_kan_announces_intent_to_resign_as_pm_20110826` |
| 30 Aug 2011 | notice of resignation en masse (菅直人); House of Representatives designates (野田佳彦); House of Councillors first ballot without a majority (野田佳彦); House of Councillors designates (野田佳彦); resignation statement (菅直人); resignation en masse decided (菅直人) | `jp_shugiin_giun_receives_kan_resignation_notice_20110830`, `jp_shugiin_honkaigi_kan_resignation_notice_20110830`, `jp_shugiin_designates_noda_20110830`, `jp_sangiin_giun_receives_kan_resignation_notice_20110830`, `jp_sangiin_honkaigi_kan_resignation_notice_20110830`, `jp_sangiin_first_ballot_no_majority_20110830`, `jp_sangiin_designates_noda_runoff_20110830`, `jp_kan_cabinet_resigned_statement_20110830`, `jp_kantei_kan_cabinet_resignation_decided_20110830` |
| 1 Sep 2011 | continued duties (act) (菅直人); continued-duties act recalled (菅直人) | `jp_kan_leads_disaster_drill_continued_duties_20110901`, `jp_kan_instructs_ministers_continued_duties_20110901` |
| 2 Sep 2011 | leaves the Prime Minister's Office (菅直人); formation recalled by a member (野田佳彦); formation recalled by the holder (野田佳彦); presiding officers' report to the Emperor; Imperial appointment ceremony (親任式); Imperial appointment ceremony (親任式) (野田佳彦); cabinet launched (野田佳彦); statement of assumption (野田佳彦); first cabinet meeting | `jp_kan_leaves_kantei_20110902`, `jp_member_states_noda_cabinet_first_meeting_20110902`, `jp_noda_states_cabinet_formation_completed_20110902`, `jp_kunaicho_speakers_report_20110902`, `jp_kunaicho_ceremony_20110902`, `jp_kunaicho_ceremony_noda_20110902`, `jp_noda_shinninshiki_appointed_20110902`, `jp_noda_cabinet_formed_20110902`, `jp_noda_states_assumed_office_today_20110902`, `jp_kantei_first_cabinet_meeting_instruction_20110902` |
| 13 Sep 2011 | in office (野田佳彦) | `jp_noda_policy_speech_states_appointment_20110913` |
| 26 Dec 2012 | notice of resignation en masse (野田佳彦); resignation en masse decided (野田佳彦); leaves the Prime Minister's Office (野田佳彦); resignation statement (野田佳彦); House of Representatives designates (安倍晋三); House of Councillors first ballot without a majority (安倍晋三); House of Councillors designates (安倍晋三); presiding officers' report to the Emperor; Imperial appointment ceremony (親任式); Imperial appointment ceremony (親任式) (安倍晋三); statement of appointment (安倍晋三); cabinet launched (安倍晋三); Kantei account of the designation (安倍晋三) | `jp_sangiin_giun_receives_noda_resignation_notice_20121226`, `jp_noda_cabinet_resignation_decided_20121226`, `jp_noda_leaves_kantei_20121226`, `jp_noda_cabinet_resigned_statement_20121226`, `jp_shugiin_designates_abe_20121226`, `jp_sangiin_first_ballot_no_majority_20121226`, `jp_sangiin_designates_abe_runoff_20121226`, `jp_kunaicho_speakers_report_20121226`, `jp_kunaicho_ceremony_20121226`, `jp_kunaicho_ceremony_abe_20121226`, `jp_abe_statement_appointed_again_20121226`, `jp_abe_shinninshiki_appointed_20121226`, `jp_abe_second_cabinet_formed_20121226`, `jp_abe_press_conference_appointed_96th_today_20121226`, `jp_kantei_abe_designated_96th_20121226` |
| 28 Jan 2013 | in office (安倍晋三) | `jp_abe_policy_speech_states_96th_pm_20130128` |
| 24 Dec 2014 | notice of resignation en masse (安倍晋三); House of Representatives designates (安倍晋三); House of Councillors designates (安倍晋三); presiding officers' report to the Emperor; Prime Minister's report to the Emperor (unnamed); Imperial appointment ceremony (親任式); Imperial appointment ceremony (親任式) (安倍晋三); cabinet launched (安倍晋三); statement of assumption (安倍晋三); resignation en masse decided (安倍晋三); Kantei account of the designation (安倍晋三) | `jp_sangiin_giun_receives_abe_resignation_notice_20141224`, `jp_shugiin_designates_abe_20141224`, `jp_sangiin_designates_abe_20141224`, `jp_kunaicho_speakers_report_20141224`, `jp_kunaicho_pm_report_20141224`, `jp_kunaicho_ceremony_20141224`, `jp_kunaicho_ceremony_abe_20141224`, `jp_abe_shinninshiki_appointed_20141224`, `jp_abe_third_cabinet_formed_20141224`, `jp_abe_press_conference_continues_office_today_20141224`, `jp_kantei_second_abe_cabinet_resigned_20141224`, `jp_kantei_abe_designated_97th_20141224` |
| 12 Feb 2015 | in office (安倍晋三) | `jp_abe_policy_speech_continues_office_20150212` |
| 1 Nov 2017 | notice of resignation en masse (安倍晋三); House of Representatives designates (安倍晋三); House of Councillors designates (安倍晋三); presiding officers' report to the Emperor; Prime Minister's report to the Emperor (unnamed); Imperial appointment ceremony (親任式); Imperial appointment ceremony (親任式) (安倍晋三); cabinet launched (安倍晋三); statement of assumption (安倍晋三); resignation en masse decided (安倍晋三); Kantei account of the designation (安倍晋三) | `jp_sangiin_giun_receives_abe_resignation_notice_20171101`, `jp_shugiin_designates_abe_20171101`, `jp_sangiin_designates_abe_20171101`, `jp_kunaicho_speakers_report_20171101`, `jp_kunaicho_pm_report_20171101`, `jp_kunaicho_ceremony_20171101`, `jp_kunaicho_ceremony_abe_20171101`, `jp_abe_shinninshiki_appointed_20171101`, `jp_abe_fourth_cabinet_formed_20171101`, `jp_abe_press_conference_98th_pm_today_20171101`, `jp_kantei_third_abe_cabinet_resigned_20171101`, `jp_kantei_abe_designated_98th_20171101` |
| 17 Nov 2017 | in office (安倍晋三) | `jp_abe_policy_speech_continues_office_20171117` |
| 28 Aug 2020 | intention to resign announced (安倍晋三) | `jp_abe_announces_intent_to_resign_as_pm_20200828` |
| 16 Sep 2020 | notice of resignation en masse (安倍晋三); House of Representatives designates (菅義偉); House of Councillors designates (菅義偉); resignation en masse decided (安倍晋三); leaves the Prime Minister's Office (安倍晋三); resignation statement (安倍晋三); Official Gazette appointment notice (菅義偉); Official Gazette: office lost (退官) (安倍晋三); Kantei account of the designation (菅義偉); Imperial appointment ceremony (親任式) (菅義偉); cabinet launched (菅義偉); statement of appointment; in office (菅義偉); statement of appointment (菅義偉) | `jp_shugiin_giun_receives_abe_resignation_notice_20200916`, `jp_shugiin_honkaigi_abe_resignation_notice_20200916`, `jp_shugiin_designates_suga_20200916`, `jp_sangiin_giun_receives_abe_resignation_notice_20200916`, `jp_sangiin_honkaigi_abe_resignation_notice_20200916`, `jp_sangiin_designates_suga_20200916`, `jp_abe_cabinet_resignation_decided_20200916`, `jp_abe_leaves_kantei_20200916`, `jp_abe_cabinet_resigns_statement_20200916`, `jp_kanpo_suga_appointed_pm_20200916`, `jp_kanpo_abe_office_lost_20200916`, `jp_kantei_suga_designated_99th_20200916`, `jp_kantei_suga_shinninshiki_20200916`, `jp_kantei_suga_cabinet_formed_20200916`, `jp_kantei_pm_statement_assumes_office_20200916`, `jp_kantei_suga_press_conference_as_pm_20200916`, `jp_kunaicho_ceremony_suga_20200916`, `jp_cabinet_minutes_suga_appointment_statement_20200916` |
| 4 Oct 2021 | notice of resignation en masse (菅義偉); House of Representatives designates (岸田文雄); House of Councillors designates (岸田文雄); resignation en masse decided (菅義偉); resignation statement (菅義偉); Official Gazette: office lost (退官) (菅義偉); continued-duties arrangement (菅義偉); Kantei account of the designation (岸田文雄); Imperial appointment ceremony (親任式) (岸田文雄); cabinet launched (岸田文雄); statement of appointment; Official Gazette appointment notice (岸田文雄); statement of appointment (岸田文雄) | `jp_suga_notifies_cabinet_resignation_hr_20211004`, `jp_sangiin_receives_suga_resignation_notice_20211004`, `jp_shugiin_receives_suga_cabinet_resignation_notice_20211004`, `jp_shugiin_designates_kishida_20211004`, `jp_sangiin_notice_suga_cabinet_resigns_20211004`, `jp_sangiin_designates_kishida_20211004`, `jp_kantei_suga_cabinet_resigned_20211004`, `jp_kantei_suga_cabinet_resignation_statement_20211004`, `jp_kanpo_suga_office_lost_20211004`, `jp_cabinet_minutes_suga_cabinet_resignation_decided_20211004`, `jp_cabinet_minutes_suga_cabinet_emergency_arrangement_20211004`, `jp_kantei_kishida_designated_100th_20211004`, `jp_kantei_kishida_shinninshiki_20211004`, `jp_kantei_kishida_cabinet_formed_20211004`, `jp_kantei_pm_statement_assumes_office_20211004`, `jp_kunaicho_ceremony_kishida_20211004`, `jp_kanpo_kishida_appointed_pm_20211004`, `jp_cabinet_minutes_kishida_appointment_statement_20211004` |
| 10 Nov 2021 | notice of resignation en masse (岸田文雄); House of Representatives designates (岸田文雄); House of Councillors designates (岸田文雄); resignation en masse decided (岸田文雄); Kantei account of the designation (岸田文雄); Imperial appointment ceremony (親任式) (岸田文雄); cabinet launched (岸田文雄); statement of assumption; Official Gazette appointment notice (岸田文雄); Official Gazette: office lost (退官) (岸田文雄); statement of assumption (岸田文雄) | `jp_sangiin_receives_kishida_resignation_notice_20211110`, `jp_shugiin_designates_kishida_20211110`, `jp_sangiin_designates_kishida_20211110`, `jp_kantei_kishida_cabinet_resigned_morning_20211110`, `jp_kantei_kishida_designated_101st_20211110`, `jp_kantei_kishida_shinninshiki_20211110`, `jp_kantei_kishida_second_cabinet_formed_20211110`, `jp_kantei_pm_statement_assumes_office_again_20211110`, `jp_kunaicho_ceremony_kishida_20211110`, `jp_kanpo_kishida_appointed_pm_20211110`, `jp_kanpo_kishida_office_lost_20211110`, `jp_cabinet_minutes_kishida_cabinet_resignation_decided_20211110`, `jp_cabinet_minutes_kishida_assumption_statement_20211110` |
| 1 Oct 2024 | notice of resignation en masse (岸田文雄); House of Representatives designates (石破茂); House of Councillors designates (石破茂); resignation en masse decided (岸田文雄); resignation statement (岸田文雄); Official Gazette appointment notice (石破茂); Official Gazette: office lost (退官) (岸田文雄); continued-duties arrangement (岸田文雄); Kantei account of the designation (石破茂); Imperial appointment ceremony (親任式) (石破茂); cabinet launched (石破茂); statement of appointment; in office (石破茂); Imperial appointment ceremony (親任式); statement of appointment (石破茂) | `jp_kishida_notifies_cabinet_resignation_hr_20241001`, `jp_sangiin_receives_kishida_resignation_notice_20241001`, `jp_shugiin_receives_kishida_cabinet_resignation_notice_20241001`, `jp_shugiin_designates_ishiba_20241001`, `jp_sangiin_notice_kishida_cabinet_resigns_20241001`, `jp_sangiin_designates_ishiba_20241001`, `jp_kantei_kishida_cabinet_resigned_20241001`, `jp_kantei_kishida_cabinet_resignation_statement_20241001`, `jp_kanpo_ishiba_appointed_pm_20241001`, `jp_kanpo_kishida_office_lost_20241001`, `jp_cabinet_minutes_kishida_cabinet_resignation_decided_20241001`, `jp_cabinet_minutes_kishida_cabinet_emergency_arrangement_20241001`, `jp_kantei_ishiba_designated_102nd_20241001`, `jp_kantei_ishiba_shinninshiki_20241001`, `jp_kantei_ishiba_cabinet_formed_20241001`, `jp_kantei_pm_statement_assumes_office_20241001`, `jp_kantei_ishiba_press_conference_as_pm_20241001`, `jp_kunaicho_ceremony_20241001`, `jp_cabinet_minutes_ishiba_appointment_statement_20241001` |
| 11 Nov 2024 | notice of resignation en masse (石破茂); House of Representatives first ballot without a majority (石破茂); House of Representatives designates (石破茂); House of Councillors designates (石破茂); Kantei account of the designation (石破茂); Imperial appointment ceremony (親任式) (石破茂); cabinet launched (石破茂); in office (石破茂); Imperial appointment ceremony (親任式); Official Gazette appointment notice (石破茂); Official Gazette: office lost (退官) (石破茂); resignation en masse decided (石破茂); statement of assumption (石破茂); statement of assumption | `jp_sangiin_receives_ishiba_resignation_notice_20241111`, `jp_shugiin_first_ballot_no_majority_20241111`, `jp_shugiin_designates_ishiba_runoff_20241111`, `jp_sangiin_designates_ishiba_20241111`, `jp_kantei_ishiba_designated_103rd_20241111`, `jp_kantei_ishiba_shinninshiki_20241111`, `jp_kantei_ishiba_second_cabinet_formed_20241111`, `jp_kantei_ishiba_press_conference_as_pm_20241111`, `jp_kunaicho_ceremony_20241111`, `jp_kanpo_ishiba_appointed_pm_20241111`, `jp_kanpo_ishiba_office_lost_20241111`, `jp_cabinet_minutes_ishiba_cabinet_resignation_decided_20241111`, `jp_cabinet_minutes_ishiba_assumption_statement_20241111`, `jp_kantei_pm_statement_assumes_office_again_20241111` |
| 21 Oct 2025 | notice of resignation en masse (石破茂); House of Representatives designates (高市早苗); House of Councillors first ballot without a majority (高市早苗); House of Councillors designates (高市早苗); resignation en masse decided (石破茂); resignation statement (石破茂); Kantei account of the designation (高市早苗); Imperial appointment ceremony (親任式) (高市早苗); cabinet launched (高市早苗); statement of appointment; in office (高市早苗); Imperial appointment ceremony (親任式); Official Gazette appointment notice (高市早苗); Official Gazette: office lost (退官) (石破茂); continued-duties arrangement (石破茂); statement of appointment (高市早苗) | `jp_ishiba_notifies_cabinet_resignation_hr_20251021`, `jp_sangiin_receives_ishiba_resignation_notice_20251021`, `jp_shugiin_receives_ishiba_cabinet_resignation_notice_20251021`, `jp_shugiin_designates_takaichi_20251021`, `jp_sangiin_notice_ishiba_cabinet_resigns_20251021`, `jp_sangiin_first_ballot_no_majority_20251021`, `jp_sangiin_designates_takaichi_runoff_20251021`, `jp_kantei_ishiba_cabinet_resigned_20251021`, `jp_kantei_ishiba_cabinet_resignation_statement_20251021`, `jp_kantei_takaichi_designated_104th_20251021`, `jp_kantei_takaichi_shinninshiki_20251021`, `jp_kantei_takaichi_cabinet_formed_20251021`, `jp_kantei_pm_statement_assumes_office_20251021`, `jp_kantei_takaichi_press_conference_as_pm_20251021`, `jp_kunaicho_ceremony_20251021`, `jp_kanpo_takaichi_appointed_pm_20251021`, `jp_kanpo_ishiba_office_lost_20251021`, `jp_cabinet_minutes_ishiba_cabinet_resignation_decided_20251021`, `jp_cabinet_minutes_ishiba_cabinet_emergency_arrangement_20251021`, `jp_cabinet_minutes_takaichi_appointment_statement_20251021` |
| 18 Feb 2026 | notice of resignation en masse (高市早苗); House of Representatives designates (高市早苗); House of Councillors first ballot without a majority (高市早苗); House of Councillors designates (高市早苗); Kantei account of the designation (高市早苗); Imperial appointment ceremony (親任式) (高市早苗); cabinet launched (高市早苗); statement of assumption; in office (高市早苗); Imperial appointment ceremony (親任式); Official Gazette appointment notice (高市早苗); Official Gazette: office lost (退官) (高市早苗); resignation en masse decided (高市早苗); statement of assumption (高市早苗) | `jp_sangiin_receives_takaichi_resignation_notice_20260218`, `jp_shugiin_designates_takaichi_20260218`, `jp_sangiin_first_ballot_no_majority_20260218`, `jp_sangiin_designates_takaichi_runoff_20260218`, `jp_kantei_takaichi_designated_105th_20260218`, `jp_kantei_takaichi_shinninshiki_20260218`, `jp_kantei_takaichi_second_cabinet_formed_20260218`, `jp_kantei_pm_statement_assumes_office_again_20260218`, `jp_kantei_takaichi_press_conference_as_pm_20260218`, `jp_kunaicho_ceremony_20260218`, `jp_kanpo_takaichi_appointed_pm_20260218`, `jp_kanpo_takaichi_office_lost_20260218`, `jp_cabinet_minutes_takaichi_cabinet_resignation_decided_20260218`, `jp_cabinet_minutes_takaichi_assumption_statement_20260218` |
| 27 Jul 2026 | in office (高市早苗) | `jp_takaichi_attends_hr_budget_committee_as_pm_20260727` |
| 4 Sep 2026 | in office (高市早苗) | `jp_kantei_takaichi_in_office_20260904` |
| (no structured date) | retrospective spans, undated recollections of continued duties and of taking office, the Chief Cabinet Secretary's 262-day span | `jp_kantei_span_abe_90_20060926_20070926`, `jp_fukuda_recalls_taking_office_20070926`, `jp_kantei_span_fukuda_91_20070926_20080924`, `jp_kantei_span_aso_92_20080924_20090916`, `jp_ccs_states_hatoyama_cabinet_span_262_days_20100604`, `jp_member_states_hatoyama_caretaker_cabinet_20100604_20100608`, `jp_pm_kan_confirms_caretaker_cabinet_20100615`, `jp_kantei_span_hatoyama_93_20090916_20100608`, `jp_kan_recalls_taking_office_20100608`, `jp_member_states_kan_cabinet_continued_duties_20110830_20110902`, `jp_noda_states_kan_asked_to_continue_duties_20110830_20110902`, `jp_kantei_span_kan_94_20100608_20110902`, `jp_noda_recalls_continued_duties_period_20110928`, `jp_kantei_span_noda_95_20110902_20121226`, `jp_kantei_span_abe_96_20121226_20141224`, `jp_kantei_span_abe_97_20141224_20171101`, `jp_kantei_span_abe_98_20171101_20200916` |

## Observations

### JP-PM06-01 — 2006-2007: 安倍晋三

Evidence: the Kantei statement titled 安倍内閣総理大臣談話 of 26 September 2006 says he was appointed that day
(`jp_abe_named_statement_appointed_pm_20060926`), the Kantei account dates the 親任式 to that night
(`jp_abe_named_shinninshiki_20060926`) and the Imperial Household Agency's photo page captions 親任式（安倍内閣総理大臣）
(`jp_kunaicho_ceremony_abe_named_20060926`); both Houses' designations that day stay C01-12's claims. On 12 September 2007 he
announced that he had resolved to resign (`jp_abe_announces_resignation_decision_20070912`; recalled in the joint committee
of 25 September, `jp_member_recalls_abe_resignation_announcement_20070912`). The Chief Cabinet Secretary reported on the
morning of 13 September that he had gone to a hospital in Tokyo for an examination at about 10:45
(`jp_ccs_reports_abe_hospital_examination_20070913`, a record added by the part A check); on 24 September he spoke as
Prime Minister (`jp_abe_in_office_press_conference_20070924`), said he had been in hospital since the 13th
(`jp_abe_hospitalised_since_20070913`), that no acting Prime Minister had been designated
(`jp_abe_no_acting_pm_designated_20070924`) and that he would leave office "tomorrow"
(`jp_abe_states_he_will_leave_office_tomorrow_20070924`). On 25 September 2007 the cabinet decided to resign en masse
(`jp_abe_cabinet_resignation_decided_20070925`, `jp_ccs_reports_abe_cabinet_resignation_decided_20070925`); the Houses
received the notice at 09:17 and 09:20 and read it in plenary (`jp_shugiin_rules_receives_abe_resignation_notice_20070925`,
`jp_sangiin_rules_receives_abe_resignation_notice_20070925`, `jp_shugiin_receives_abe_cabinet_resignation_notice_20070925`,
`jp_sangiin_notice_abe_cabinet_resigns_20070925`), and his statement says the cabinet resigned that day
(`jp_abe_cabinet_resigned_statement_20070925`). The Kantei's 90th page gives his span to 26 September 2007
(`jp_kantei_span_abe_90_20060926_20070926`).

Decision: accepted in part. The holder is from 26 September 2006, in office on 24 September 2007, with no end.

Limits: no source states the day his office ended, and no record states that he performed the duties on 25-26 September 2007.

### JP-PM06-02 — 2007-2008: 福田康夫

Evidence: on 25 September 2007 the House of Representatives designated 福田康夫 (338 of 477; `jp_shugiin_designates_fukuda_20070925`);
the House of Councillors' first ballot gave no majority (`jp_sangiin_first_ballot_no_majority_20070925`) and in the runoff it
designated 小沢一郎 (`jp_sangiin_designates_ozawa_runoff_20070925`, no holder name); both Houses recorded the joint committee
(`jp_sangiin_joint_committee_requested_20070925`, `jp_shugiin_receives_joint_committee_request_20070925`), which reached no
agreement (`jp_joint_committee_no_agreement_20070925`), and both declared the House of Representatives' designation the
Diet's resolution (`jp_shugiin_hr_resolution_prevails_fukuda_20070925`, `jp_sangiin_hr_resolution_prevails_20070925`). That
evening the incoming Chief Cabinet Secretary scheduled the ceremony for 08:30 the next morning
(`jp_ccs_schedules_fukuda_ceremony_20070925`), and 福田康夫 spoke of having received the appointment (拝命) before it
(`jp_fukuda_says_appointed_on_eve_20070925`). On 26 September 2007 the Kantei account (`jp_fukuda_shinninshiki_appointed_20070926`),
the Chief Cabinet Secretary's record (ceremony from 08:30; `jp_ccs_reports_fukuda_ceremony_0830_20070926`) and the agency's
photo page (`jp_kunaicho_ceremony_fukuda_20070926`) date the ceremony; the schedule row names no appointee
(`jp_kunaicho_ceremony_20070926`); the unnamed statement (`jp_kantei_pm_statement_appointed_20070926`) and the art. 9
acting order (`jp_ccs_reports_fukuda_acting_order_designated_20070926`) are claims; the cabinet was launched that day
(`jp_fukuda_cabinet_formed_20070926`). He announced his resignation on 1 September 2008
(`jp_fukuda_announces_resignation_decision_20080901`, where he also recalls taking office on 26 September,
`jp_fukuda_recalls_taking_office_20070926`). On 24 September 2008 the cabinet resigned en masse
(`jp_fukuda_cabinet_resignation_decided_20080924`, `jp_ccs_reports_fukuda_cabinet_resignation_decided_20080924`), the Houses
received the notice at 09:21 and 09:19 and read it (four notice claims), the unnamed statement says he has resigned
(`jp_kantei_pm_statement_resigned_20080924`), and the Chief Cabinet Secretary kept the ministers responsible for emergencies
in Tokyo until the attestation ceremony (`jp_ccs_arranges_emergency_ministers_until_attestation_20080924`). The Kantei's 91st
page gives the span (`jp_kantei_span_fukuda_91_20070926_20080924`).

Decision: accepted in part: from 26 September 2007, with no end.

Limits: no source states the day the office ended.

### JP-PM06-03 — 2008-2009: 麻生太郎

Evidence: on 24 September 2008 the House of Representatives designated 麻生太郎 (`jp_shugiin_designates_aso_20080924`); the
House of Councillors, after a first ballot without a majority (`jp_sangiin_first_ballot_no_majority_20080924`), designated
小沢一郎 in the runoff (`jp_sangiin_designates_ozawa_runoff_20080924`); the joint committee was requested and failed
(`jp_sangiin_joint_committee_requested_20080924`, `jp_shugiin_receives_joint_committee_request_20080924`,
`jp_joint_committee_no_agreement_20080924`) and the House of Representatives' designation prevailed
(`jp_shugiin_hr_resolution_prevails_aso_20080924`, `jp_sangiin_hr_resolution_prevails_20080924`). The Kantei account dates the
ceremony to late that night (`jp_aso_shinninshiki_appointed_20080924`), the agency's photo page captions 親任式（麻生内閣総理大臣）
(`jp_kunaicho_ceremony_aso_20080924`; the schedule row, `jp_kunaicho_ceremony_20080924`, names no one), and the cabinet was
launched that day (`jp_aso_cabinet_formed_20080924`). The art. 9 acting order was designated at the first cabinet meeting on
25 September (`jp_ccs_reports_aso_acting_order_designated_20080925`). On 16 September 2009 the cabinet resigned en masse
(`jp_aso_cabinet_resignation_decided_20090916`); the House of Councillors received the notice at 09:25
(`jp_sangiin_rules_receives_aso_resignation_notice_20090916`); 麻生太郎 said the cabinet had resigned and he was leaving office
(`jp_aso_states_cabinet_resigned_20090916`, also recalling the launch of 24 September 2008,
`jp_aso_recalls_cabinet_launched_20080924`). The 92nd page gives the span (`jp_kantei_span_aso_92_20080924_20090916`).

Decision: accepted in part: from 24 September 2008, with no end.

Limits: the House of Representatives' records of 16 September 2009 (its first sitting after the election) contain no report of
the notice; no source states the day the office ended.

### JP-PM06-04 — 2009-2010: 鳩山由紀夫

Evidence: on 16 September 2009 both Houses designated 鳩山由紀夫 at the first ballot (`jp_shugiin_designates_hatoyama_20090916`,
`jp_sangiin_designates_hatoyama_20090916`); the Chief Cabinet Secretary scheduled the ceremony for 19:00
(`jp_ccs_schedules_hatoyama_ceremony_20090916`); the Kantei account dates it to that night (`jp_hatoyama_shinninshiki_appointed_20090916`)
and the agency's photo page captions 親任式（鳩山内閣総理大臣） (`jp_kunaicho_ceremony_hatoyama_20090916`; the schedule row,
`jp_kunaicho_ceremony_20090916`, names no one); the cabinet was launched that day (`jp_hatoyama_cabinet_formed_20090916`). On
2 June 2010 the Chief Cabinet Secretary reported that at 10:00 a message from 鳩山代表 to the party's joint meeting had
announced his resignation as Prime Minister and party leader (`jp_ccs_reports_hatoyama_resignation_announcement_20100602`),
that the government would carry on until the next head was chosen (`jp_ccs_government_to_continue_until_successor_20100602`),
and that at the 13:30 extraordinary cabinet meeting 鳩山総理 asked for crisis management without a gap until the new cabinet
(`jp_hatoyama_transition_instruction_20100602`); these records were found by the part A check. On 4 June 2010 the cabinet
resigned en masse (`jp_hatoyama_cabinet_resignation_decided_20100604`, `jp_ccs_reports_hatoyama_cabinet_resignation_decided_20100604`),
the Houses received the notice at 09:35 and 09:37 and read it (four notice claims), his statement says the cabinet resigned
that day (`jp_hatoyama_cabinet_resigned_statement_20100604`), and his remarks at the last cabinet meeting say that "today" he
leaves the post (`jp_hatoyama_last_cabinet_meeting_remarks_20100604`), attributed and dated by the Chief Cabinet Secretary's
record and the statement index (`jp_ccs_reports_hatoyama_final_cabinet_meeting_remarks_20100604`,
`jp_kantei_index_lists_hatoyama_resignation_items_20100604`). The Chief Cabinet Secretary gave the cabinet's span as 16
September of the previous year to "today, 4 June", 262 days (`jp_ccs_states_hatoyama_cabinet_span_262_days_20100604`). On
15 June 2010 a member stated that the former Hatoyama cabinet continued as a continued-duties cabinet (職務執行内閣) until the
ceremony of 8 June (`jp_member_states_hatoyama_caretaker_cabinet_20100604_20100608`), and 菅直人 answered that he had been deputy
prime minister in it (`jp_pm_kan_confirms_caretaker_cabinet_20100615`). The 93rd page gives the span to 8 June 2010
(`jp_kantei_span_hatoyama_93_20090916_20100608`).

Decision: accepted in part: from 16 September 2009, with no end.

Limits: the end conflicts between the same-day "today" and the 262-day span on one side and the continued duties to 8 June
and the Kantei's span on the other; it is left unresolved, and the Gazette of 2010, which would state the day the office
was lost, is not available without restriction.

### JP-PM06-05 — 2010-2011: 菅直人

Evidence: on 4 June 2010 both Houses designated 菅直人 (`jp_shugiin_designates_kan_20100604`, `jp_sangiin_designates_kan_20100604`),
reported by the Kantei (`jp_kantei_kan_designated_94th_20100604`) and recalled by a member on 15 June
(`jp_member_recalls_kan_designation_20100604`). On 8 June 2010 the presiding officers reported to the Emperor
(`jp_kunaicho_speakers_report_20100608`); the Chief Cabinet Secretary scheduled the ceremony for 18:15
(`jp_ccs_schedules_kan_ceremony_20100608`); 菅直人 said he would take office that evening
(`jp_kan_press_conference_to_assume_office_that_evening_20100608`); the Kantei account (`jp_kan_shinninshiki_appointed_20100608`)
and the agency's photo page 親任式（菅内閣総理大臣） (`jp_kunaicho_ceremony_kan_20100608`; schedule row `jp_kunaicho_ceremony_20100608`)
date the ceremony, and the cabinet was launched (`jp_kan_cabinet_formed_20100608`). He spoke as Prime Minister on 11 June
(`jp_kan_policy_speech_bears_office_20100611`) and later recalled taking office on 8 June (`jp_kan_recalls_taking_office_20100608`).
On 26 August 2011 he announced that he would resign after a new party leader was chosen
(`jp_kan_announces_intent_to_resign_as_pm_20110826`); on 30 August 2011 the cabinet resigned en masse
(`jp_kantei_kan_cabinet_resignation_decided_20110830`, `jp_kan_cabinet_resigned_statement_20110830`; notices at 10:20 and 10:23 and
the plenary readings). On 1 September he led the disaster drill (`jp_kan_leads_disaster_drill_continued_duties_20110901`) and
instructed the ministers on Typhoon No. 12 (`jp_kan_instructs_ministers_continued_duties_20110901`); a member and 野田佳彦 later
described the interval as a continued-duties cabinet (`jp_member_states_kan_cabinet_continued_duties_20110830_20110902`,
`jp_noda_states_kan_asked_to_continue_duties_20110830_20110902`). He left the Prime Minister's Office on 2 September
(`jp_kan_leaves_kantei_20110902`). The 94th page gives the span (`jp_kantei_span_kan_94_20100608_20110902`, a record added by
the part B check).

Decision: accepted in part: from 8 June 2010, with no end.

Limits: no source states the day the office ended; continued duties are claims only.

### JP-PM06-06 — 2011-2012: 野田佳彦

Evidence: on 30 August 2011 the House of Representatives designated 野田佳彦 (`jp_shugiin_designates_noda_20110830`); the House of
Councillors' first ballot gave no majority (`jp_sangiin_first_ballot_no_majority_20110830`) and the runoff designated him
(`jp_sangiin_designates_noda_runoff_20110830`). On 2 September 2011 the presiding officers reported to the Emperor
(`jp_kunaicho_speakers_report_20110902`), the agency's photo page captions 親任式（野田内閣総理大臣）
(`jp_kunaicho_ceremony_noda_20110902`; schedule row `jp_kunaicho_ceremony_20110902`), the Kantei account dates the ceremony
(`jp_noda_shinninshiki_appointed_20110902`), his press conference says he formally took office that day
(`jp_noda_states_assumed_office_today_20110902`), the cabinet was launched (`jp_noda_cabinet_formed_20110902`) and the Prime
Minister's instruction at the first cabinet meeting was published unnamed (`jp_kantei_first_cabinet_meeting_instruction_20110902`);
later a member and he recalled that day (`jp_member_states_noda_cabinet_first_meeting_20110902`,
`jp_noda_states_cabinet_formation_completed_20110902`), and in speech 119 of 28 September 2011 he dated the designation to
29 August, which both Houses' minutes contradict (`jp_noda_recalls_continued_duties_period_20110928`). He spoke as Prime Minister
on 13 September 2011 (`jp_noda_policy_speech_states_appointment_20110913`). On 26 December 2012 the cabinet resigned en masse
(`jp_noda_cabinet_resignation_decided_20121226`, `jp_noda_cabinet_resigned_statement_20121226`; the House of Councillors' receipt
at 09:17, `jp_sangiin_giun_receives_noda_resignation_notice_20121226`) and he left the Prime Minister's Office
(`jp_noda_leaves_kantei_20121226`). The 95th page gives the span (`jp_kantei_span_noda_95_20110902_20121226`).

Decision: accepted in part: from 2 September 2011, with no end.

Limits: the House of Representatives' records of 26 December 2012 carry no report of the notice; no source states the day the
office ended.

### JP-PM06-07 — 2012-2020: 安倍晋三's second to fourth appointments

Evidence: on 26 December 2012 both Houses designated 安倍晋三 (`jp_shugiin_designates_abe_20121226`; the House of Councillors in a
runoff, `jp_sangiin_first_ballot_no_majority_20121226`, `jp_sangiin_designates_abe_runoff_20121226`), reported by the Kantei
(`jp_kantei_abe_designated_96th_20121226`); the presiding officers reported to the Emperor (`jp_kunaicho_speakers_report_20121226`);
the agency's photo page (`jp_kunaicho_ceremony_abe_20121226`), the Kantei account (`jp_abe_shinninshiki_appointed_20121226`), his
statement (`jp_abe_statement_appointed_again_20121226`) and his press conference (`jp_abe_press_conference_appointed_96th_today_20121226`)
date the appointment; the cabinet was launched (`jp_abe_second_cabinet_formed_20121226`); in office on 28 January 2013
(`jp_abe_policy_speech_states_96th_pm_20130128`). On 24 December 2014 the second Abe cabinet resigned at a morning cabinet meeting
(`jp_kantei_second_abe_cabinet_resigned_20141224`; the House of Councillors' receipt at 09:22), both Houses re-designated him
(`jp_shugiin_designates_abe_20141224`, `jp_sangiin_designates_abe_20141224`, `jp_kantei_abe_designated_97th_20141224`), and the
ceremony, the launch and his press conference followed (`jp_kunaicho_ceremony_abe_20141224`, `jp_abe_shinninshiki_appointed_20141224`,
`jp_abe_third_cabinet_formed_20141224`, `jp_abe_press_conference_continues_office_today_20141224`), with the Prime Minister's
report to the Emperor unnamed (`jp_kunaicho_pm_report_20141224`); in office on 12 February 2015. On 1 November 2017 the same
sequence (`jp_kantei_third_abe_cabinet_resigned_20171101`, `jp_shugiin_designates_abe_20171101`, `jp_sangiin_designates_abe_20171101`,
`jp_kantei_abe_designated_98th_20171101`, `jp_kunaicho_ceremony_abe_20171101`, `jp_abe_shinninshiki_appointed_20171101`,
`jp_abe_fourth_cabinet_formed_20171101`, `jp_abe_press_conference_98th_pm_today_20171101`); in office on 17 November 2017. On
28 August 2020 he announced that he would resign (`jp_abe_announces_intent_to_resign_as_pm_20200828`); on 16 September 2020 the
cabinet resigned en masse (`jp_abe_cabinet_resignation_decided_20200916`, `jp_abe_cabinet_resigns_statement_20200916`; notices at
09:24 and 09:26 and both plenary readings) and he left the Prime Minister's Office (`jp_abe_leaves_kantei_20200916`). Page 2 of
the Official Gazette No. 99 states that he, Prime Minister of the fourth Abe cabinet, lost his office on the 16th
(`jp_kanpo_abe_office_lost_20200916`). The 96th-98th pages give the spans (claims only).

Decision: accepted in part: three holders from 26 December 2012, 24 December 2014 and 1 November 2017; the third until
16 September 2020.

Limits: no source reviewed states the day the 2012 and 2014 appointments ended (their Gazette issues are not available without
restriction); the re-designations are never used as those ends.

### JP-PM06-08 — 2020-2021: 菅義偉

Evidence: on 16 September 2020 both Houses designated 菅義偉 (`jp_shugiin_designates_suga_20200916`, `jp_sangiin_designates_suga_20200916`,
`jp_kantei_suga_designated_99th_20200916`); the Kantei account dates the ceremony (`jp_kantei_suga_shinninshiki_20200916`), the
agency's entry records 親任式（菅内閣総理大臣） (`jp_kunaicho_ceremony_suga_20200916`), page 2 of the Official Gazette No. 99 states
his appointment on the 16th (`jp_kanpo_suga_appointed_pm_20200916`), and the minutes of that night's first cabinet meeting, which
he attended as Prime Minister, record his statement "本日，私は，内閣総理大臣を拝命し"
(`jp_cabinet_minutes_suga_appointment_statement_20200916`; the unnamed Kantei copy is `jp_kantei_pm_statement_assumes_office_20200916`);
the cabinet was launched and he held a press conference (`jp_kantei_suga_cabinet_formed_20200916`,
`jp_kantei_suga_press_conference_as_pm_20200916`). On 4 October 2021 the cabinet decided at 09:04-09:20 to resign en masse
(`jp_cabinet_minutes_suga_cabinet_resignation_decided_20211004`, `jp_kantei_suga_cabinet_resigned_20211004`,
`jp_kantei_suga_cabinet_resignation_statement_20211004`; notices at 09:21 and 09:23 and both plenary readings), and the Chief
Cabinet Secretary told the ministers that the present cabinet must respond to emergencies until the new cabinet was launched
(`jp_cabinet_minutes_suga_cabinet_emergency_arrangement_20211004`). Page 2 of the Gazette No. 83 states that he lost his office on
the 4th (`jp_kanpo_suga_office_lost_20211004`).

Decision: accepted: from 16 September 2020 until 4 October 2021.

Limits: page 1 of the Gazette No. 99 is not retrievable (its only capture returns 404); page 2 states the appointment.

### JP-PM06-09 — 2021-2024: 岸田文雄

Evidence: on 4 October 2021 both Houses designated 岸田文雄 (`jp_shugiin_designates_kishida_20211004`, `jp_sangiin_designates_kishida_20211004`,
`jp_kantei_kishida_designated_100th_20211004`); the Kantei account (`jp_kantei_kishida_shinninshiki_20211004`), the agency's entry
(`jp_kunaicho_ceremony_kishida_20211004`), the Gazette No. 83 (`jp_kanpo_kishida_appointed_pm_20211004`) and the minutes of that
night's first cabinet meeting (`jp_cabinet_minutes_kishida_appointment_statement_20211004`) date the appointment. On 10 November
2021, the special session having convened, the cabinet resigned under Constitution art. 70 at 09:01-09:15
(`jp_cabinet_minutes_kishida_cabinet_resignation_decided_20211110`, `jp_kantei_kishida_cabinet_resigned_morning_20211110`; the
House of Councillors' receipt at 09:19), both Houses re-designated him, and the Kantei account, the agency's entry, the Gazette
No. 88 and the minutes (本日，引き続き，内閣総理大臣の重責を担う) date the second appointment. Page 2 of the Gazette No. 88 states
that he, Prime Minister of the Kishida cabinet, lost his office on the 10th (`jp_kanpo_kishida_office_lost_20211110`). On 1 October
2024 the cabinet resigned en masse at 09:02-09:23 (`jp_cabinet_minutes_kishida_cabinet_resignation_decided_20241001`, with the
emergency arrangement `jp_cabinet_minutes_kishida_cabinet_emergency_arrangement_20241001`; the Kantei account and statement;
notices at 09:24 and 09:26 and both plenary readings), and page 2 of the Gazette No. 45 states that he lost his office on the 1st
(`jp_kanpo_kishida_office_lost_20241001`).

Decision: accepted: from 4 October 2021 until 10 November 2021, and from 10 November 2021 until 1 October 2024.

Limits: the House of Representatives' minutes of 10 November 2021 do not mention the resignation notice.

### JP-PM06-10 — 2024-2026: 石破茂 and 高市早苗

Evidence: on 1 October 2024 both Houses designated 石破茂; the Gazette No. 45, the Kantei account, the minutes of that night's first
cabinet meeting (naming him in full) and his press conference date the appointment (`jp_kanpo_ishiba_appointed_pm_20241001`,
`jp_kantei_ishiba_shinninshiki_20241001`, `jp_cabinet_minutes_ishiba_appointment_statement_20241001`,
`jp_kantei_ishiba_press_conference_as_pm_20241001`); the agency's entry names no appointee (`jp_kunaicho_ceremony_20241001`). On
11 November 2024 the cabinet resigned under art. 70 at 08:21-08:33 (`jp_cabinet_minutes_ishiba_cabinet_resignation_decided_20241111`;
the House of Councillors' receipt at 08:38), the House of Representatives re-designated him in a runoff
(`jp_shugiin_first_ballot_no_majority_20241111`, `jp_shugiin_designates_ishiba_runoff_20241111`) and the House of Councillors at the
first ballot, and the Kantei account captured that day, the Gazette No. 52, the minutes (私は、本日、再び…) and his press conference
date the second appointment; the unnamed statement of that day, missing from the dossier, is imported
(`jp_kantei_pm_statement_assumes_office_again_20241111`). Page 2 of the Gazette No. 52 states that he lost the first appointment on
the 11th (`jp_kanpo_ishiba_office_lost_20241111`). On 21 October 2025 the Ishiba cabinet resigned en masse at 08:51-09:05, with the
emergency arrangement (`jp_cabinet_minutes_ishiba_cabinet_resignation_decided_20251021`,
`jp_cabinet_minutes_ishiba_cabinet_emergency_arrangement_20251021`; notices at 09:10 and 09:11 and both plenary readings); the House
of Representatives designated 高市早苗 at the first ballot and the House of Councillors in a runoff; the Kantei account captured that
day, the Gazette No. 28, the minutes of that night's first meeting and her press conference date her appointment; page 2 of the
Gazette No. 28 states that 石破茂 lost his office on the 21st (`jp_kanpo_ishiba_office_lost_20251021`). On 18 February 2026, after
the general election, the cabinet resigned under art. 70 at 09:00-09:10 (`jp_cabinet_minutes_takaichi_cabinet_resignation_decided_20260218`;
the House of Councillors' receipt at 09:17), both Houses re-designated her (the House of Councillors in a runoff), the Kantei
account, the Gazette No. 9, the minutes (本日、再び…) and her press conference date the second appointment, and page 2 of the
Gazette No. 9 states that she lost the first appointment on the 18th (`jp_kanpo_takaichi_office_lost_20260218`). She is listed as
内閣総理大臣 高市早苗君 at the House of Representatives Budget Committee's recess (閉会中審査) sitting of 27 July 2026
(`jp_takaichi_attends_hr_budget_committee_as_pm_20260727`) and shown as 高市総理 at the Kumamoto earthquake recovery headquarters on
4 September 2026 (`jp_kantei_takaichi_in_office_20260904`).

Decision: accepted: 石破茂 from 1 October 2024 until 11 November 2024 and from 11 November 2024 until 21 October 2025; 高市早苗 from
21 October 2025 until 18 February 2026 and from 18 February 2026, in office before the cutoff.

Limits: the House of Representatives' minutes of 11 November 2024 and 18 February 2026 do not mention the resignation notices; a
Kantei page dated 17 September 2026 is after the cutoff and excluded. The LDP presidency observations of 石破茂 and 高市早苗 stay
separate and unchanged.

## Sources added

| Source ID | What | Provenance |
|---|---|---|
| `jp_kantei_abe_danwa_named_20060926` | Kantei: 安倍内閣総理大臣談話, 26 Sep 2006 (re-recorded with the holder named) | capture 2006-10-04 of www.kantei.go.jp |
| `jp_kantei_abe_hossoku_named_20060926` | Kantei: 安倍総理の動き — 安倍内閣の発足, 26 Sep 2006 (re-recorded with the holder named) | capture 2006-10-04 of www.kantei.go.jp |
| `jp_kunaicho_photo_abe_named_20060926` | 宮内庁: 天皇皇后両陛下のご日程 写真 平成18年9月26日 親任式（安倍内閣総理大臣） (re-recorded with the holder named) | capture 2006-10-10 of www.kunaicho.go.jp |
| `jp_kantei_abe_kaiken_20070912` | 安倍内閣総理大臣記者会見 (12 Sep 2007) | capture 2007-09-14 of www.kantei.go.jp |
| `jp_kantei_ccs_press_20070913_am` | Kantei: 官房長官記者発表 平成１９年９月１３日（木）午前 (安倍内閣総理大臣の検診について) | capture 2007-11-20 of www.kantei.go.jp |
| `jp_kantei_abe_kaiken_20070924` | 安倍内閣総理大臣記者会見 (24 Sep 2007) | capture 2007-10-13 of www.kantei.go.jp |
| `jp_kantei_abe_sojishoku_danwa_20070925` | 内閣総辞職に当たっての内閣総理大臣談話 (25 Sep 2007, 安倍総理の演説・記者会見等) | capture 2007-10-13 of www.kantei.go.jp |
| `jp_kantei_abe_jisyoku_20070925` | 安倍総理の動き - 安倍内閣総辞職 (25 Sep 2007) | capture 2007-11-19 of www.kantei.go.jp |
| `jp_shugiin_giun_20070925` | 衆議院 議院運営委員会 第3号 (25 Sep 2007) | Diet minutes API, whole meeting record |
| `jp_sangiin_giun_20070925` | 参議院 議院運営委員会 第2号 (25 Sep 2007) | Diet minutes API, whole meeting record |
| `jp_shugiin_honkaigi_20070925` | 衆議院 本会議 第2号 (25 Sep 2007) | Diet minutes API, whole meeting record |
| `jp_sangiin_honkaigi_20070925` | 参議院 本会議 第2号 (25 Sep 2007) | Diet minutes API, whole meeting record |
| `jp_ryoin_kyogikai_20070925` | 両院 内閣総理大臣の指名両院協議会 第1号 (25 Sep 2007) | Diet minutes API, whole meeting record |
| `jp_kantei_ccs_press_20070925_am` | Kantei: 官房長官発表 平成１９年９月２５日（火）午前 (閣議の概要) | capture 2007-11-20 of www.kantei.go.jp |
| `jp_kantei_rekidai_090` | 歴代内閣 第90代 安倍 晋三 (retrospective) | capture 2026-06-10 of www.kantei.go.jp |
| `jp_kantei_fukuda_kaiken_20070925` | 福田内閣総理大臣記者会見 (25 Sep 2007) | capture 2007-10-11 of www.kantei.go.jp |
| `jp_kantei_ccs_press_20070925_pm` | Kantei: 官房長官発表 平成１９年９月２５日（火）午後 (閣僚名簿の発表) | capture 2007-11-20 of www.kantei.go.jp |
| `jp_kantei_fukuda_hossoku_20070926` | 福田総理の動き - 福田内閣の発足 (26 Sep 2007) | capture 2007-10-13 of www.kantei.go.jp |
| `jp_kantei_fukuda_danwa_20070926` | 内閣総理大臣談話 (26 Sep 2007, 福田総理の演説・記者会見等) | capture 2007-10-12 of www.kantei.go.jp |
| `jp_kunaicho_schedule_2007_h2` | 宮内庁 天皇皇后両陛下のご日程 (July-December 2007) | capture 2007-10-26 of www.kunaicho.go.jp |
| `jp_kunaicho_photo_20070926` | 宮内庁 photo page: 親任式（福田内閣総理大臣）(26 Sep 2007) | capture 2011-03-22 of www.kunaicho.go.jp |
| `jp_kantei_ccs_press_20070926_am` | Kantei: 官房長官記者発表 平成１９年９月２６日（水）午前 (初閣議の概要) | capture 2007-11-20 of www.kantei.go.jp |
| `jp_kantei_fukuda_kaiken_20080901` | 福田内閣総理大臣記者会見 (1 Sep 2008) | capture 2008-09-05 of www.kantei.go.jp |
| `jp_kantei_fukuda_sojishoku_danwa_20080924` | 内閣総辞職に当たっての内閣総理大臣談話 (24 Sep 2008, 福田総理の演説・記者会見等) | capture 2008-09-26 of www.kantei.go.jp |
| `jp_kantei_fukuda_jisyoku_20080924` | 福田総理の動き - 福田内閣総辞職 (24 Sep 2008) | capture 2008-09-27 of www.kantei.go.jp |
| `jp_shugiin_giun_20080924` | 衆議院 議院運営委員会 第1号 (24 Sep 2008) | Diet minutes API, whole meeting record |
| `jp_sangiin_giun_20080924` | 参議院 議院運営委員会 第1号 (24 Sep 2008) | Diet minutes API, whole meeting record |
| `jp_shugiin_honkaigi_20080924` | 衆議院 本会議 第1号 (24 Sep 2008) | Diet minutes API, whole meeting record |
| `jp_sangiin_honkaigi_20080924` | 参議院 本会議 第1号 (24 Sep 2008) | Diet minutes API, whole meeting record |
| `jp_kantei_ccs_press_20080924_am` | Kantei: 官房長官記者発表 平成20年9月24日(水)午前 (閣議の概要) | capture 2008-10-13 of www.kantei.go.jp |
| `jp_kantei_rekidai_091` | 歴代内閣 第91代 福田 康夫 (retrospective) | capture 2026-04-19 of www.kantei.go.jp |
| `jp_ryoin_kyogikai_20080924` | 両院 内閣総理大臣の指名両院協議会 第1号 (24 Sep 2008) | Diet minutes API, whole meeting record |
| `jp_kantei_aso_hossoku_20080924` | 麻生総理の動き - 麻生内閣の発足 (24 Sep 2008) | capture 2008-09-27 of www.kantei.go.jp |
| `jp_kunaicho_schedule_2008_h2` | 宮内庁 天皇皇后両陛下のご日程 (July-December 2008) | capture 2008-10-14 of www.kunaicho.go.jp |
| `jp_kunaicho_photo_20080924` | 宮内庁 photo page: 親任式（麻生内閣総理大臣）(24 Sep 2008) | capture 2008-10-13 of www.kunaicho.go.jp |
| `jp_kantei_ccs_press_20080925` | Kantei: 官房長官記者発表 平成20年9月25日(木) (初閣議の概要) | capture 2008-10-13 of www.kantei.go.jp |
| `jp_kantei_aso_kaiken_20090916` | 麻生内閣総理大臣記者会見 (16 Sep 2009) | capture 2009-10-05 of www.kantei.go.jp |
| `jp_kantei_aso_jisyoku_20090916` | 麻生総理の動き - 麻生内閣総辞職 (16 Sep 2009) | capture 2009-09-24 of www.kantei.go.jp |
| `jp_sangiin_giun_20090916` | 参議院 議院運営委員会 第1号 (16 Sep 2009) | Diet minutes API, whole meeting record |
| `jp_kantei_rekidai_092` | 歴代内閣 第92代 麻生 太郎 (retrospective) | capture 2026-06-04 of www.kantei.go.jp |
| `jp_shugiin_honkaigi_20090916` | 衆議院 本会議 第1号 (16 Sep 2009) | Diet minutes API, whole meeting record |
| `jp_sangiin_honkaigi_20090916` | 参議院 本会議 第1号 (16 Sep 2009) | Diet minutes API, whole meeting record |
| `jp_kantei_hatoyama_hossoku_20090916` | 鳩山総理の動き - 鳩山内閣の発足 (16 Sep 2009) | capture 2009-09-23 of www.kantei.go.jp |
| `jp_kunaicho_schedule_2009_h2` | 宮内庁 天皇皇后両陛下のご日程 平成21年（7月～） | capture 2010-01-10 of www.kunaicho.go.jp |
| `jp_kunaicho_photo_20090916` | 宮内庁 photo page: 親任式（鳩山内閣総理大臣）(16 Sep 2009) | capture 2011-03-22 of www.kunaicho.go.jp |
| `jp_kantei_ccs_press_20090916_pm` | Kantei: 官房長官記者発表(速報) 平成21年9月16日(水)午後 (閣僚名簿の発表) | capture 2009-09-25 of www.kantei.go.jp |
| `jp_kantei_ccs_press_20100602_am` | Kantei: 官房長官記者発表 平成22年6月2日（水）午前 (臨時閣議の開催について) | capture 2010-06-14 of www.kantei.go.jp |
| `jp_kantei_ccs_press_20100602_pm` | Kantei: 官房長官記者発表 平成22年6月2日（水）午後 (臨時閣議の概要について) | capture 2010-06-14 of www.kantei.go.jp |
| `jp_kantei_hatoyama_hatsugen_20100604` | 内閣総理大臣発言 (4 Jun 2010, PDF, 鳩山内閣最後の閣議) | capture 2010-06-14 of www.kantei.go.jp; PDF pages 1, 2, 3 viewed |
| `jp_kantei_hatoyama_statement_index_201006` | 鳩山総理の演説・記者会見等 (index, captured 14 Jun 2010) | capture 2010-06-14 of www.kantei.go.jp |
| `jp_kantei_hatoyama_sojishoku_danwa_20100604` | 内閣総辞職に当たっての内閣総理大臣談話 (4 Jun 2010, 鳩山総理の演説・記者会見等) | capture 2010-06-07 of www.kantei.go.jp |
| `jp_kantei_hatoyama_soujisyoku_20100604` | 鳩山総理の動き - 鳩山内閣総辞職 (4 Jun 2010) | capture 2010-06-07 of www.kantei.go.jp |
| `jp_shugiin_giun_20100604` | 衆議院 議院運営委員会 第37号 (4 Jun 2010) | Diet minutes API, whole meeting record |
| `jp_sangiin_giun_20100604` | 参議院 議院運営委員会 第26号 (4 Jun 2010) | Diet minutes API, whole meeting record |
| `jp_shugiin_honkaigi_20100604` | 衆議院 本会議 第34号 (4 Jun 2010) | Diet minutes API, whole meeting record |
| `jp_sangiin_honkaigi_20100604` | 参議院 本会議 第26号 (4 Jun 2010) | Diet minutes API, whole meeting record |
| `jp_kantei_ccs_press_20100604_am` | Kantei: 官房長官記者発表 平成22年6月4日（金）午前 (閣議の概要; 官房長官からの一言) | capture 2010-06-07 of www.kantei.go.jp |
| `jp_sangiin_honkaigi_20100615_s004` | 参議院 本会議 第28号 (15 Jun 2010), speech 4 (林芳正) | Diet minutes API, single speech |
| `jp_sangiin_honkaigi_20100615_s005` | 参議院 本会議 第28号 (15 Jun 2010), speech 5 (菅直人, 内閣総理大臣) | Diet minutes API, single speech |
| `jp_kantei_rekidai_093` | 歴代内閣 第93代 鳩山 由紀夫 (retrospective) | capture 2025-10-10 of www.kantei.go.jp |
| `jp_kantei_kan_shimei_20100604` | Kantei: 菅総理の動き — 内閣総理大臣の指名, 4 Jun 2010 | capture 2010-06-11 of www.kantei.go.jp |
| `jp_kunaicho_schedule_2010_q2` | 宮内庁: 天皇皇后両陛下のご日程：平成22年（4月～6月） | stored official file, www.kunaicho.go.jp |
| `jp_kunaicho_photo_20100608` | 宮内庁: ご日程 写真 平成22年6月8日 親任式・認証官任命式（大臣等20名）（宮殿） | stored official file, www.kunaicho.go.jp |
| `jp_kantei_kan_kaiken_20100608` | Kantei: 菅内閣総理大臣記者会見, 8 Jun 2010 | capture 2010-06-14 of www.kantei.go.jp |
| `jp_kantei_kan_hossoku_20100608` | Kantei: 菅総理の動き — 菅内閣の発足, 8 Jun 2010 | capture 2010-06-14 of www.kantei.go.jp |
| `jp_kantei_ccs_press_20100608_pm` | Kantei: 官房長官記者発表 平成22年6月8日（火）午後 (閣僚名簿の発表) | capture 2010-06-14 of www.kantei.go.jp |
| `jp_shugiin_honkaigi_kan_speech_20100611` | House of Representatives plenary No. 35, 11 Jun 2010, 174th Diet, speech 9: 菅直人's policy speech (所信表明演説) | Diet minutes API, single speech |
| `jp_kantei_kan_kaiken_20110826` | Kantei: 菅内閣総理大臣記者会見, 26 Aug 2011 | capture 2011-09-01 of www.kantei.go.jp |
| `jp_shugiin_giun_20110830` | House of Representatives, Committee on Rules and Administration No. 42, 30 Aug 2011, 177th Diet (衆議院 議院運営委員会 第42号) | Diet minutes API, whole meeting record |
| `jp_shugiin_honkaigi_20110830` | House of Representatives plenary No. 40, 30 Aug 2011, 177th Diet: notice of resignation and designation of the Prime Minister (衆議院 本会議 第40号) | Diet minutes API, whole meeting record |
| `jp_sangiin_giun_20110830` | House of Councillors, Committee on Rules and Administration No. 37, 30 Aug 2011, 177th Diet (参議院 議院運営委員会 第37号) | Diet minutes API, whole meeting record |
| `jp_sangiin_honkaigi_20110830` | House of Councillors plenary No. 37, 30 Aug 2011, 177th Diet: notice of resignation and designation of the Prime Minister (参議院 本会議 第37号) | Diet minutes API, whole meeting record |
| `jp_kantei_kan_sojishoku_danwa_20110830` | Kantei: 内閣総辞職にあたっての内閣総理大臣談話, 30 Aug 2011 | capture 2011-09-01 of www.kantei.go.jp |
| `jp_kantei_kan_bousai_20110901` | Kantei: 菅総理の動き — 平成２３年度総合防災訓練, 1 Sep 2011 | capture 2011-09-08 of www.kantei.go.jp |
| `jp_kantei_kan_sojishoku_20110902` | Kantei: 菅総理の動き — 菅内閣総辞職, dated 2 Sep 2011 | capture 2011-09-08 of www.kantei.go.jp |
| `jp_sangiin_yosan_20110928_s112` | House of Councillors, Budget Committee No. 2, 28 Sep 2011, 178th Diet, speech 112: question by 世耕弘成 | Diet minutes API, single speech |
| `jp_sangiin_yosan_20110928_s113` | House of Councillors, Budget Committee No. 2, 28 Sep 2011, 178th Diet, speech 113: answer by 野田佳彦 (内閣総理大臣) | Diet minutes API, single speech |
| `jp_sangiin_yosan_20110928_s115` | House of Councillors, Budget Committee No. 2, 28 Sep 2011, 178th Diet, speech 115: answer by 野田佳彦 (内閣総理大臣) | Diet minutes API, single speech |
| `jp_kantei_rekidai_094` | Kantei 歴代内閣: 第94代 菅直人 (retrospective page) | capture 2025-07-25 of www.kantei.go.jp |
| `jp_kunaicho_schedule_2011_q3` | 宮内庁: 天皇皇后両陛下のご日程：平成23年（7月～9月） | stored official file, www.kunaicho.go.jp |
| `jp_kunaicho_photo_20110902` | 宮内庁: ご日程 写真 平成23年9月2日 親任式・認証官任命式（大臣等20名）（宮殿） | stored official file, www.kunaicho.go.jp |
| `jp_kantei_noda_hossoku_20110902` | Kantei: 野田総理の動き — 野田内閣の発足, 2 Sep 2011 | capture 2011-09-08 of www.kantei.go.jp |
| `jp_kantei_noda_kaiken_20110902` | Kantei: 野田内閣総理大臣記者会見, 2 Sep 2011 | capture 2011-09-08 of www.kantei.go.jp |
| `jp_kantei_noda_siji_20110902` | Kantei: 総理指示・談話など — 平成23年9月2日 内閣総理大臣指示（第３次補正予算編成について） | capture 2012-06-11 of www.kantei.go.jp |
| `jp_shugiin_honkaigi_noda_speech_20110913` | House of Representatives plenary No. 1, 13 Sep 2011, 178th Diet, speech 22: 野田佳彦's policy speech (所信表明演説) | Diet minutes API, single speech |
| `jp_sangiin_yosan_20110928_s119` | House of Councillors, Budget Committee No. 2, 28 Sep 2011, 178th Diet, speech 119: answer by 野田佳彦 (内閣総理大臣) | Diet minutes API, single speech |
| `jp_sangiin_giun_20121226` | House of Councillors, Committee on Rules and Administration No. 1, 26 Dec 2012, 182nd (special) Diet (参議院 議院運営委員会 第1号) | Diet minutes API, whole meeting record |
| `jp_kantei_noda_sojishoku_20121226` | Kantei: 総理の一日 — 平成24年12月26日 野田内閣総辞職 | capture 2013-01-02 of www.kantei.go.jp |
| `jp_kantei_noda_sojishoku_danwa_20121226` | Kantei: 総理指示・談話など — 平成24年12月26日 内閣総辞職に当たっての内閣総理大臣談話 | capture 2013-01-04 of www.kantei.go.jp |
| `jp_kantei_rekidai_095` | Kantei 歴代内閣: 第95代 野田佳彦 (retrospective page) | capture 2026-06-08 of www.kantei.go.jp |
| `jp_shugiin_honkaigi_20121226` | House of Representatives plenary No. 1, 26 Dec 2012, 182nd (special) Diet: designation of the Prime Minister (衆議院 本会議 第1号) | Diet minutes API, whole meeting record |
| `jp_sangiin_honkaigi_20121226` | House of Councillors plenary No. 1, 26 Dec 2012, 182nd (special) Diet: designation of the Prime Minister (参議院 本会議 第1号) | Diet minutes API, whole meeting record |
| `jp_kunaicho_schedule_2012_q4` | 宮内庁: 天皇皇后両陛下のご日程：平成24年（10月～12月） | stored official file, www.kunaicho.go.jp |
| `jp_kunaicho_photo_20121226` | 宮内庁: ご日程 写真 平成24年12月26日 親任式，認証官任命式（大臣等21名）（宮殿） | stored official file, www.kunaicho.go.jp |
| `jp_kantei_abe_danwa_20121226` | Kantei: 総理指示・談話など — 内閣総理大臣談話 (page dated 27 Dec 2012; statement dated 26 Dec 2012, 閣議決定) | capture 2013-01-05 of www.kantei.go.jp |
| `jp_kantei_abe_hossoku_20121226` | Kantei: 総理の一日 — 平成24年12月26日 第２次安倍内閣の発足 | capture 2012-12-31 of www.kantei.go.jp |
| `jp_kantei_abe_kaiken_20121226` | Kantei: 総理の演説・記者会見など — 平成24年12月26日 安倍内閣総理大臣就任記者会見 | capture 2012-12-31 of www.kantei.go.jp |
| `jp_kantei_abe_designation_20121226` | Kantei: 総理の一日 — 平成24年12月26日 内閣総理大臣の指名 | capture 2012-12-30 of www.kantei.go.jp |
| `jp_shugiin_honkaigi_abe_speech_20130128` | House of Representatives plenary No. 1, 28 Jan 2013, 183rd Diet, speech 11: 安倍晋三's policy speech (所信表明演説) | Diet minutes API, single speech |
| `jp_sangiin_giun_20141224` | House of Councillors, Committee on Rules and Administration No. 1, 24 Dec 2014, 188th (special) Diet (参議院 議院運営委員会 第1号) | Diet minutes API, whole meeting record |
| `jp_shugiin_honkaigi_20141224` | House of Representatives plenary No. 1, 24 Dec 2014, 188th (special) Diet: designation of the Prime Minister (衆議院 本会議 第1号) | Diet minutes API, whole meeting record |
| `jp_sangiin_honkaigi_20141224` | House of Councillors plenary No. 1, 24 Dec 2014, 188th (special) Diet: designation of the Prime Minister (参議院 本会議 第1号) | Diet minutes API, whole meeting record |
| `jp_kunaicho_schedule_2014_q4` | 宮内庁: 天皇皇后両陛下のご日程：平成26年（10月～12月） | stored official file, www.kunaicho.go.jp |
| `jp_kunaicho_photo_20141224` | 宮内庁: ご日程 写真 平成26年12月24日 親任式・認証官任命式（大臣等21名）（宮殿） | stored official file, www.kunaicho.go.jp |
| `jp_kantei_abe_hossoku_20141224` | Kantei: 総理の一日 — 平成26年12月24日 第３次安倍内閣の発足 | capture 2015-03-24 of www.kantei.go.jp |
| `jp_kantei_abe_kaiken_20141224` | Kantei: 総理の演説・記者会見など — 平成26年12月24日 安倍内閣総理大臣記者会見 | capture 2015-02-02 of www.kantei.go.jp |
| `jp_kantei_abe_designation_20141224` | Kantei: 総理の一日 — 平成26年12月24日 内閣総理大臣の指名 | capture 2015-01-11 of www.kantei.go.jp |
| `jp_shugiin_honkaigi_abe_speech_20150212` | House of Representatives plenary No. 5, 12 Feb 2015, 189th Diet, speech 3: 安倍晋三's policy speech (施政方針演説) | Diet minutes API, single speech |
| `jp_sangiin_giun_20171101` | House of Councillors, Committee on Rules and Administration No. 1, 1 Nov 2017, 195th (special) Diet (参議院 議院運営委員会 第1号) | Diet minutes API, whole meeting record |
| `jp_shugiin_honkaigi_20171101` | House of Representatives plenary No. 1, 1 Nov 2017, 195th (special) Diet: designation of the Prime Minister (衆議院 本会議 第1号) | Diet minutes API, whole meeting record |
| `jp_sangiin_honkaigi_20171101` | House of Councillors plenary No. 1, 1 Nov 2017, 195th (special) Diet: designation of the Prime Minister (参議院 本会議 第1号) | Diet minutes API, whole meeting record |
| `jp_kunaicho_schedule_2017_q4` | 宮内庁: 天皇皇后両陛下のご日程：平成29年（10月～12月） | stored official file, www.kunaicho.go.jp |
| `jp_kunaicho_photo_20171101` | 宮内庁: ご日程 写真 平成29年11月1日 親任式・認証官任命式（大臣等22名）（宮殿） | stored official file, www.kunaicho.go.jp |
| `jp_kantei_abe_hossoku_20171101` | Kantei: 総理の一日 — 平成29年11月1日 第４次安倍内閣の発足 | capture 2020-08-29 of www.kantei.go.jp |
| `jp_kantei_abe_kaiken_20171101` | Kantei: 総理の演説・記者会見など — 平成29年11月1日 安倍内閣総理大臣記者会見 | capture 2017-11-02 of www.kantei.go.jp |
| `jp_kantei_abe_designation_20171101` | Kantei: 総理の一日 — 平成29年11月1日 内閣総理大臣の指名 | capture 2019-07-17 of www.kantei.go.jp |
| `jp_shugiin_honkaigi_abe_speech_20171117` | House of Representatives plenary No. 4, 17 Nov 2017, 195th Diet, speech 7: 安倍晋三's policy speech (所信表明演説) | Diet minutes API, single speech |
| `jp_kantei_abe_kaiken_20200828` | Kantei: 総理の演説・記者会見など — 令和2年8月28日 安倍内閣総理大臣記者会見 | capture 2020-08-29 of www.kantei.go.jp |
| `jp_shugiin_giun_20200916` | House of Representatives, Committee on Rules and Administration No. 1, 16 Sep 2020, 202nd (extraordinary) Diet (衆議院 議院運営委員会 第1号) | Diet minutes API, whole meeting record |
| `jp_shugiin_honkaigi_20200916` | House of Representatives plenary No. 1, 16 Sep 2020, 202nd (extraordinary) Diet: notice of resignation and designation of the Prime Minister (衆議院 本会議 第1号) | Diet minutes API, whole meeting record |
| `jp_sangiin_giun_20200916` | House of Councillors, Committee on Rules and Administration No. 1, 16 Sep 2020, 202nd (extraordinary) Diet (参議院 議院運営委員会 第1号) | Diet minutes API, whole meeting record |
| `jp_sangiin_honkaigi_20200916` | House of Councillors plenary No. 1, 16 Sep 2020, 202nd (extraordinary) Diet: notice of resignation and designation of the Prime Minister (参議院 本会議 第1号) | Diet minutes API, whole meeting record |
| `jp_kantei_abe_sojishoku_20200916` | Kantei: 総理の一日 — 令和2年9月16日 安倍内閣総辞職 | capture 2020-09-30 of www.kantei.go.jp |
| `jp_kantei_abe_sojishoku_danwa_20200916` | Kantei: 総理の指示・談話など — 令和2年9月16日 内閣総辞職に当たっての内閣総理大臣談話 | capture 2020-09-30 of www.kantei.go.jp |
| `jp_kanpo_gogai_toku99_20200916_p2` | 官報 令和2年9月16日（水曜日）号外特第99号 2頁 | capture 2020-09-17 of kanpou.npb.go.jp; PDF page 1 viewed |
| `jp_kantei_rekidai_096` | Kantei 歴代内閣: 第96代 安倍晋三 (retrospective page) | capture 2026-04-21 of www.kantei.go.jp |
| `jp_kantei_rekidai_097` | Kantei 歴代内閣: 第97代 安倍晋三 (retrospective page) | capture 2026-04-19 of www.kantei.go.jp |
| `jp_kantei_rekidai_098` | Kantei 歴代内閣: 第98代 安倍晋三 (retrospective page) | capture 2026-05-13 of www.kantei.go.jp |
| `jp_kantei_suga_designation_20200916` | Kantei 総理の一日: 内閣総理大臣の指名（令和2年9月16日） | capture 2020-09-30 of www.kantei.go.jp |
| `jp_kantei_suga_cabinet_launch_20200916` | Kantei 総理の一日: 菅内閣の発足（令和2年9月16日） | capture 2020-09-18 of www.kantei.go.jp |
| `jp_kantei_pm_statement_20200916` | Kantei 総理の指示・談話など: 内閣総理大臣談話（令和2年9月16日、閣議決定） | capture 2020-09-30 of www.kantei.go.jp |
| `jp_kantei_suga_press_conference_20200916` | Kantei 総理の演説・記者会見など: 菅内閣総理大臣記者会見（令和2年9月16日） | capture 2020-09-17 of www.kantei.go.jp |
| `jp_kunaicho_schedule_entry_20200916` | 宮内庁 天皇ご一家のご日程：親任式・認証官任命式（大臣等23名）（宮殿）（令和2年9月16日） | stored official file, www.kunaicho.go.jp |
| `jp_kantei_cabinet_minutes_20200916_first` | Kantei: 初閣議及び閣僚懇談会議事録 令和２年９月１６日（水） | capture 2024-08-15 of www.kantei.go.jp; PDF pages 1, 2 viewed |
| `jp_shugiin_giun_20211004` | 第205回国会 衆議院 議院運営委員会 第1号（令和3年10月4日） | Diet minutes API, whole meeting record |
| `jp_sangiin_giun_20211004` | 第205回国会 参議院 議院運営委員会 第1号（令和3年10月4日） | Diet minutes API, whole meeting record |
| `jp_shugiin_honkaigi_20211004` | 第205回国会 衆議院 本会議 第1号（令和3年10月4日） | Diet minutes API, whole meeting record |
| `jp_sangiin_honkaigi_20211004` | 第205回国会 参議院 本会議 第1号（令和3年10月4日） | Diet minutes API, whole meeting record |
| `jp_kantei_suga_cabinet_resignation_20211004` | Kantei 総理の一日: 菅内閣総辞職（令和3年10月4日） | capture 2021-10-04 of www.kantei.go.jp |
| `jp_kantei_suga_resignation_statement_20211004` | Kantei 総理の指示・談話など: 内閣総辞職に当たっての内閣総理大臣談話（令和3年10月4日、閣議決定） | capture 2021-10-04 of www.kantei.go.jp |
| `jp_kanpo_gogai_toku83_20211004_p2` | 官報 令和3年10月4日（月曜日）号外特第83号 2頁 | capture 2021-10-05 of kanpou.npb.go.jp; PDF page 1 viewed |
| `jp_kantei_cabinet_minutes_20211004_resignation` | Kantei: 臨時閣議及び閣僚懇談会議事録 令和３年１０月４日（月） | capture 2022-05-19 of www.kantei.go.jp; PDF pages 1, 2, 3, 5 viewed |
| `jp_kantei_kishida_designation_20211004` | Kantei 総理の一日: 内閣総理大臣の指名（令和3年10月4日） | capture 2021-10-04 of www.kantei.go.jp |
| `jp_kantei_kishida_cabinet_launch_20211004` | Kantei 総理の一日: 岸田内閣の発足（令和3年10月4日） | capture 2021-10-04 of www.kantei.go.jp |
| `jp_kantei_pm_statement_20211004` | Kantei 総理の指示・談話など: 内閣総理大臣談話（令和3年10月4日、閣議決定） | capture 2021-10-04 of www.kantei.go.jp |
| `jp_kunaicho_schedule_entry_20211004` | 宮内庁 天皇ご一家のご日程：親任式・認証官任命式（大臣等23名）（宮殿）（令和3年10月4日） | stored official file, www.kunaicho.go.jp |
| `jp_kanpo_gogai_toku83_20211004` | 官報 令和3年10月4日（月曜日）号外特第83号 1頁 | capture 2021-10-04 of kanpou.npb.go.jp; PDF page 1 viewed |
| `jp_kantei_cabinet_minutes_20211004_first` | Kantei: 初閣議及び閣僚懇談会議事録 令和３年１０月４日（月） | capture 2022-05-19 of www.kantei.go.jp; PDF pages 1, 2 viewed |
| `jp_sangiin_giun_20211110` | 第206回国会 参議院 議院運営委員会 第1号（令和3年11月10日） | Diet minutes API, whole meeting record |
| `jp_shugiin_honkaigi_20211110` | 第206回国会 衆議院 本会議 第1号（令和3年11月10日） | Diet minutes API, whole meeting record |
| `jp_sangiin_honkaigi_20211110` | 第206回国会 参議院 本会議 第1号（令和3年11月10日） | Diet minutes API, whole meeting record |
| `jp_kantei_kishida_designation_20211110` | Kantei 総理の一日: 内閣総理大臣の指名（令和3年11月10日） | capture 2021-11-10 of www.kantei.go.jp |
| `jp_kantei_kishida_cabinet_launch_20211110` | Kantei 総理の一日: 第２次岸田内閣の発足（令和3年11月10日） | capture 2021-11-10 of www.kantei.go.jp |
| `jp_kantei_pm_statement_20211110` | Kantei 総理の指示・談話など: 内閣総理大臣談話（令和3年11月10日、閣議決定） | capture 2021-11-10 of www.kantei.go.jp |
| `jp_kunaicho_schedule_entry_20211110` | 宮内庁 天皇ご一家のご日程：親任式・認証官任命式（大臣等23名）（宮殿）（令和3年11月10日） | stored official file, www.kunaicho.go.jp |
| `jp_kanpo_gogai_toku88_20211110` | 官報 令和3年11月10日（水曜日）号外特第88号 1頁 | capture 2021-11-11 of kanpou.npb.go.jp; PDF page 1 viewed |
| `jp_kanpo_gogai_toku88_20211110_p2` | 官報 令和3年11月10日（水曜日）号外特第88号 2頁 | capture 2021-11-11 of kanpou.npb.go.jp; PDF page 1 viewed |
| `jp_kantei_cabinet_minutes_20211110_resignation` | Kantei: 閣議及び閣僚懇談会議事録 令和３年１１月１０日（水） | capture 2022-05-19 of www.kantei.go.jp; PDF pages 1, 2, 3 viewed |
| `jp_kantei_cabinet_minutes_20211110_first` | Kantei: 初閣議及び閣僚懇談会議事録 令和３年１１月１０日（水） | capture 2022-05-19 of www.kantei.go.jp; PDF pages 1, 2 viewed |
| `jp_shugiin_giun_20241001` | 第214回国会 衆議院 議院運営委員会 第1号（令和6年10月1日） | Diet minutes API, whole meeting record |
| `jp_sangiin_giun_20241001` | 第214回国会 参議院 議院運営委員会 第1号（令和6年10月1日） | Diet minutes API, whole meeting record |
| `jp_shugiin_honkaigi_20241001` | 第214回国会 衆議院 本会議 第1号（令和6年10月1日） | Diet minutes API, whole meeting record |
| `jp_sangiin_honkaigi_20241001` | 第214回国会 参議院 本会議 第1号（令和6年10月1日） | Diet minutes API, whole meeting record |
| `jp_kantei_kishida_cabinet_resignation_20241001` | Kantei 総理の一日: 岸田内閣総辞職（令和6年10月1日） | capture 2024-10-01 of www.kantei.go.jp |
| `jp_kantei_kishida_resignation_statement_20241001` | Kantei 総理の指示・談話など: 内閣総辞職に当たっての内閣総理大臣談話（令和6年10月1日、閣議決定） | capture 2024-10-01 of www.kantei.go.jp |
| `jp_kanpo_gogai_toku45_20241001` | 官報 令和6年10月1日（火曜日）号外特第45号 1-2頁 | capture 2024-10-01 of kanpou.npb.go.jp; PDF pages 1, 2 viewed |
| `jp_kantei_cabinet_minutes_20241001_resignation` | Kantei: 閣議及び閣僚懇談会議事録 令和６年１０月１日（火） | stored official file, www.kantei.go.jp; PDF pages 1, 2, 3 viewed |
| `jp_kantei_ishiba_designation_20241001` | Kantei 総理の一日: 内閣総理大臣の指名（令和6年10月1日） | capture 2024-10-01 of www.kantei.go.jp |
| `jp_kantei_ishiba_cabinet_launch_20241001` | Kantei 総理の一日: 石破内閣の発足（令和6年10月1日） | capture 2024-10-01 of www.kantei.go.jp |
| `jp_kantei_pm_statement_20241001` | Kantei 総理の指示・談話など: 内閣総理大臣談話（令和6年10月1日、閣議決定） | capture 2024-10-01 of www.kantei.go.jp |
| `jp_kantei_ishiba_press_conference_20241001` | Kantei 総理の演説・記者会見など: 石破内閣総理大臣記者会見（令和6年10月1日） | stored official file, www.kantei.go.jp |
| `jp_kunaicho_schedule_entry_20241001` | 宮内庁 天皇ご一家のご日程：親任式（内閣総理大臣）、認証官任命式（大臣等22名）（宮殿）（令和6年10月1日） | stored official file, www.kunaicho.go.jp |
| `jp_kantei_cabinet_minutes_20241001_first` | Kantei: 初閣議及び閣僚懇談会議事録 令和６年１０月１日（火） | stored official file, www.kantei.go.jp; PDF pages 1, 2 viewed |
| `jp_sangiin_giun_20241111` | 第215回国会 参議院 議院運営委員会 第1号（令和6年11月11日） | Diet minutes API, whole meeting record |
| `jp_shugiin_honkaigi_20241111` | 第215回国会 衆議院 本会議 第1号（令和6年11月11日） | Diet minutes API, whole meeting record |
| `jp_sangiin_honkaigi_20241111` | 第215回国会 参議院 本会議 第1号（令和6年11月11日） | Diet minutes API, whole meeting record |
| `jp_kantei_ishiba_designation_20241111` | Kantei 総理の一日: 内閣総理大臣の指名（令和6年11月11日） | capture 2024-11-11 of www.kantei.go.jp |
| `jp_kantei_ishiba_cabinet_launch_20241111` | Kantei 総理の一日: 第２次石破内閣の発足（令和6年11月11日） | capture 2024-11-11 of www.kantei.go.jp |
| `jp_kantei_ishiba_press_conference_20241111` | Kantei 総理の演説・記者会見など: 石破内閣総理大臣記者会見（令和6年11月11日） | stored official file, www.kantei.go.jp |
| `jp_kunaicho_schedule_202411` | 宮内庁 天皇ご一家のご日程 令和6年（11月） | stored official file, www.kunaicho.go.jp |
| `jp_kanpo_gogai_toku52_20241111` | 官報 令和6年11月11日（月曜日）号外特第52号 1-2頁 | capture 2024-11-11 of kanpou.npb.go.jp; PDF pages 1, 2 viewed |
| `jp_kantei_cabinet_minutes_20241111_resignation` | Kantei: 臨時閣議及び閣僚懇談会議事録 令和６年１１月１１日（月） | stored official file, www.kantei.go.jp; PDF pages 1, 2, 3 viewed |
| `jp_kantei_cabinet_minutes_20241111_first` | Kantei: 初閣議及び閣僚懇談会議事録 令和６年１１月１１日（月） | stored official file, www.kantei.go.jp; PDF pages 1, 2 viewed |
| `jp_kantei_pm_statement_20241111` | Kantei: 総理の指示・談話など — 内閣総理大臣談話（令和6年11月11日、閣議決定） | stored official file, www.kantei.go.jp |
| `jp_shugiin_giun_20251021` | 第219回国会 衆議院 議院運営委員会 第1号（令和7年10月21日） | Diet minutes API, whole meeting record |
| `jp_sangiin_giun_20251021` | 第219回国会 参議院 議院運営委員会 第1号（令和7年10月21日） | Diet minutes API, whole meeting record |
| `jp_shugiin_honkaigi_20251021` | 第219回国会 衆議院 本会議 第1号（令和7年10月21日） | Diet minutes API, whole meeting record |
| `jp_sangiin_honkaigi_20251021` | 第219回国会 参議院 本会議 第1号（令和7年10月21日） | Diet minutes API, whole meeting record |
| `jp_kantei_ishiba_cabinet_resignation_20251021` | Kantei 総理の一日: 石破内閣総辞職（令和7年10月21日） | capture 2025-10-21 of www.kantei.go.jp |
| `jp_kantei_ishiba_resignation_statement_20251021` | Kantei 総理の指示・談話など: 内閣総辞職に当たっての内閣総理大臣談話（令和7年10月21日、閣議決定） | capture 2025-10-21 of www.kantei.go.jp |
| `jp_kantei_takaichi_designation_20251021` | Kantei 総理の一日: 内閣総理大臣の指名（令和7年10月21日） | capture 2025-10-21 of www.kantei.go.jp |
| `jp_kantei_takaichi_cabinet_launch_20251021` | Kantei 総理の一日: 高市内閣の発足（令和7年10月21日） | capture 2025-10-21 of www.kantei.go.jp |
| `jp_kantei_pm_statement_20251021` | Kantei 総理の指示・談話など: 内閣総理大臣談話（令和7年10月21日、閣議決定） | capture 2025-10-21 of www.kantei.go.jp |
| `jp_kantei_takaichi_press_conference_20251021` | Kantei 総理の演説・記者会見など: 高市内閣総理大臣記者会見（令和7年10月21日） | stored official file, www.kantei.go.jp |
| `jp_kunaicho_schedule_entry_20251021` | 宮内庁 天皇ご一家のご日程：親任式（内閣総理大臣）、認証官任命式（大臣等21名）（宮殿）（令和7年10月21日） | stored official file, www.kunaicho.go.jp |
| `jp_kanpo_gogai_toku28_20251021` | 官報 令和7年10月21日（火曜日）号外特第28号 1-2頁 | capture 2025-10-21 of www.kanpo.go.jp; PDF pages 1, 2 viewed |
| `jp_kantei_cabinet_minutes_20251021_resignation` | Kantei: 閣議及び閣僚懇談会議事録 令和７年１０月２１日（火） | stored official file, www.kantei.go.jp; PDF pages 1, 2, 4, 5 viewed |
| `jp_kantei_cabinet_minutes_20251021_first` | Kantei: 初閣議及び閣僚懇談会議事録 令和７年１０月２１日（火） | stored official file, www.kantei.go.jp; PDF pages 1, 2 viewed |
| `jp_sangiin_giun_20260218` | 第221回国会 参議院 議院運営委員会 第1号（令和8年2月18日） | Diet minutes API, whole meeting record |
| `jp_shugiin_honkaigi_20260218` | 第221回国会 衆議院 本会議 第1号（令和8年2月18日） | Diet minutes API, whole meeting record |
| `jp_sangiin_honkaigi_20260218` | 第221回国会 参議院 本会議 第1号（令和8年2月18日） | Diet minutes API, whole meeting record |
| `jp_kantei_takaichi_designation_20260218` | Kantei 総理の一日: 内閣総理大臣の指名（令和8年2月18日） | capture 2026-02-18 of www.kantei.go.jp |
| `jp_kantei_takaichi_cabinet_launch_20260218` | Kantei 総理の一日: 第２次高市内閣の発足（令和8年2月18日） | capture 2026-02-18 of www.kantei.go.jp |
| `jp_kantei_pm_statement_20260218` | Kantei 総理の指示・談話など: 内閣総理大臣談話（令和8年2月18日、閣議決定） | capture 2026-02-18 of www.kantei.go.jp |
| `jp_kantei_takaichi_press_conference_20260218` | Kantei 総理の演説・記者会見など: 高市内閣総理大臣記者会見（令和8年2月18日） | stored official file, www.kantei.go.jp |
| `jp_kunaicho_schedule_entry_20260218` | 宮内庁 天皇ご一家のご日程：親任式（内閣総理大臣）、認証官任命式（大臣等21名）（宮殿）（令和8年2月18日） | stored official file, www.kunaicho.go.jp |
| `jp_kanpo_gogai_toku9_20260218` | 官報 令和8年2月18日（水曜日）号外特第9号 1-2頁 | capture 2026-02-18 of www.kanpo.go.jp; PDF pages 1, 2 viewed |
| `jp_kantei_cabinet_minutes_20260218_resignation` | Kantei: 臨時閣議及び閣僚懇談会議事録 令和８年２月１８日（水） | stored official file, www.kantei.go.jp; PDF pages 1, 2 viewed |
| `jp_kantei_cabinet_minutes_20260218_first` | Kantei: 初閣議及び閣僚懇談会議事録 令和８年２月１８日（水） | stored official file, www.kantei.go.jp; PDF pages 1, 2 viewed |
| `jp_shugiin_budget_committee_20260727` | 第221回国会 衆議院 予算委員会 第17号（令和8年7月27日、閉会中審査） — 会議録情報 | Diet minutes API, single speech |
| `jp_kantei_takaichi_recovery_hq_20260904` | Kantei 総理の一日: 令和８年熊本地震非常災害復旧復興本部（令和8年9月4日） | stored official file, www.kantei.go.jp |

113 sources are raw Internet Archive captures (`id_` form) made before the cutoff: Kantei pages, the Chief Cabinet
Secretary's press records, the Cabinet's 2020-2021 minutes, the Imperial Household Agency's 2006-2009 pages and the Official
Gazette's special extra issues of 2020-2026. Each records the capture URL as `url`, the address inside it (without `:80`) as
`original_url`, and in its extract the capture time, the archived original's Last-Modified header where one was archived and
the character encoding. 67 are responses of the National Diet Library's Diet minutes API (55 whole meeting
records and 12 single speeches), recorded by the exact API URL whose bytes were hashed, with the readable minutes page
named in the extract. 31 are stored official files on the Kantei's and the Imperial Household Agency's own hosts (the
agency's 2010-2026 schedule and photo pages, which the Internet Archive does not hold or could not serve, the Kantei's
2024-2026 press conferences, minutes and one statement, and its account of 4 September 2026); each has a Last-Modified header
before the cutoff and matched a cache-busting download.

Each new source has a derived factual extract under [sources/](sources/) in the packet's format
(`spheres-c01-derived-factual-table/v1`): one row per claim, keyed by `claim_id`, with `observation_id` `jp_prime_minister`,
`review_observation`, `role_id` `jp_pm`, `holder_name`, `role_title` "内閣総理大臣 — Prime Minister of Japan", `event_kind`,
`attested_on`, the claim's text and locator. The extract's own checksum is in the packet, separate from the response hash.
Original pages, PDFs, JSON bodies, images and renders are not checked in, and no photograph, emblem, seal or signature is
republished.

Source types: `primary_cabinet_minutes_pdf_archived`, `primary_cabinet_minutes_pdf_official`, `primary_diet_minutes_api_json`, `primary_imperial_household_photo_page_archived`, `primary_imperial_household_photo_page_official`, `primary_imperial_household_schedule_archived`, `primary_imperial_household_schedule_official`, `primary_kantei_account_archived`, `primary_kantei_account_official`, `primary_kantei_chief_cabinet_secretary_press_archived`, `primary_kantei_index_archived`, `primary_kantei_page_archived_retrospective`, `primary_kantei_press_conference_archived`, `primary_kantei_press_conference_official`, `primary_kantei_statement_archived`, `primary_kantei_statement_official`, `primary_kantei_statement_pdf_archived`, `primary_official_gazette_pdf_archived`.

## Response identities and stability checks

The reviewer re-downloads every recorded response and compares its byte count and SHA-256, so every identity here was
downloaded at least twice, at least 30 minutes apart, with identical bytes, and no identity is a page generated per request.
How each was established:

- **Part A.** The researcher downloaded each response three or four times on 24 September 2026, the first and last 30 to 76
  minutes apart, the Diet API responses also with a cache-busting query; the check downloaded all 50 again at 23:03Z-23:08Z,
  the 19 API responses with a cache-busting query at 23:09Z (every request a cache miss) and all 50 again at 23:41Z-23:51Z.
  The check's candidate records were each downloaded two or three times, at least 30 minutes apart.
- **Part B.** The researcher downloaded each response at least twice, 30 minutes or more apart, the API responses also with the
  parameters reordered and the agency's pages with a cache-busting query; the check matched all 60 from about 23:35Z (its plain
  download of the agency's pages was a CloudFront cache hit, so it repeated them with a cache-busting query, which reached the
  server and matched).
- **Part C.** The researcher downloaded each response at least twice, 30 minutes or more apart, the API and stored official files
  also with a cache-busting query; the check matched all 69 at 00:09Z-00:14Z, 00:10Z-00:11Z (cache-busting) and 00:40Z-00:46Z on
  25 September, and its candidates twice (00:17Z-00:58Z and 01:02Z-01:18Z).
- **This packet (25 September).** Every one of the 211 identities was downloaded again plain at 01:30Z-01:41Z, the
  67 API responses and 31 stored official files again with a cache-busting query at 01:41Z-01:42Z, and plain again
  at 02:13Z-02:21Z (211 of them), at least 30 minutes after the first. Every byte count and SHA-256 matched. Each
  extract's provenance note gives that source's own download times.

Reproducibility traps met and avoided:

- **Compression.** The Diet API and the Internet Archive send `Content-Encoding: gzip` when asked; every recorded identity is the
  identity-encoded body, requested without `Accept-Encoding`, and each extract records `source_response_content_encoding`
  `identity`.
- **The Diet minutes site is a JavaScript application.** Only the public API is recorded (`/api/meeting?issueID=…`,
  `/api/speech?speechID=…` or `/api/speech?issueID=…&speechNumber=…`, always with `recordPacking=json`); its responses carry
  `Cache-Control: no-cache, no-store` and no timestamp, token or request echo. Full-text search URLs (`any=`) were used for
  discovery only and are not recorded.
- **Edge caches.** The agency's host sits behind CloudFront, and a quick second download can be a cache hit; the stored official
  files were therefore downloaded again with a cache-busting query and matched.
- **A capture that stopped replaying.** The capture 20200916021157 of the Kantei's 2020 resignation statement, which the part B
  check recorded, redirected on 25 September 2026 (`x-archive-redirect-reason: found capture at 20200930160518`) to a later
  capture; that capture replays the same bytes directly and is recorded instead (`jp_kantei_abe_sojishoku_danwa_20200916`).
  The part B researcher similarly recorded later captures for two 2020 pages whose earliest captures redirect or do not
  replay their own digest.
- **Later live versions.** The live copies of the second Ishiba and first Takaichi Kantei accounts and of the 94th 歴代内閣 page
  (all Last-Modified 14 March 2026) are later versions, not byte-identical to the captures; they are recorded as each extract's
  `alternate_location`, with the capture kept as the identity.
- **PDFs.** The Gazette PDFs' CreationDate is the evening of the issue day and the minutes' are days after the meeting, not the
  access date; no PDF here is generated per request.
- **CDX digests.** For captures, the SHA-1 of the recorded bytes was compared with the Internet Archive's CDX digest by the part A
  and B researchers, by the checks for their candidates, and by this packet for all 113 captures on 25 September 2026
  (the CDX server refused connections for part of the run, and failed queries were retried); each extract says who compared
  it and gives the SHA-1.

| Source ID | Bytes | SHA-256 | Recorded response |
|---|---|---|---|
| `jp_kantei_abe_danwa_named_20060926` | 4,910 | `8ae3f73ecc88…44d886` | raw capture; same response as the 1990-2006 packet's source |
| `jp_kantei_abe_hossoku_named_20060926` | 5,538 | `370ae14d206d…7f8bbf` | raw capture; same response as the 1990-2006 packet's source |
| `jp_kunaicho_photo_abe_named_20060926` | 1,399 | `416637aeddc3…e64136` | raw capture; same response as the 1990-2006 packet's source |
| `jp_kantei_abe_kaiken_20070912` | 11,978 | `eaacc123071a…4d7b8d` | raw capture |
| `jp_kantei_ccs_press_20070913_am` | 2,416 | `47cc2b8fadc4…eae123` | raw capture |
| `jp_kantei_abe_kaiken_20070924` | 10,129 | `99ffe0e22edf…2555ce` | raw capture |
| `jp_kantei_abe_sojishoku_danwa_20070925` | 2,657 | `13832132ab7a…f79b57` | raw capture |
| `jp_kantei_abe_jisyoku_20070925` | 2,902 | `c42391d81405…a05514` | raw capture |
| `jp_shugiin_giun_20070925` | 13,216 | `d4b000ea0928…d3e135` | API meeting record |
| `jp_sangiin_giun_20070925` | 19,740 | `c2c668fc7c15…f16e19` | API meeting record |
| `jp_shugiin_honkaigi_20070925` | 30,262 | `a92d9fc2613b…2b5c45` | API meeting record |
| `jp_sangiin_honkaigi_20070925` | 15,388 | `41400cdb4154…e5e0ca` | API meeting record |
| `jp_ryoin_kyogikai_20070925` | 21,704 | `15d18d73dfd3…155625` | API meeting record |
| `jp_kantei_ccs_press_20070925_am` | 2,042 | `8da4dcc5306f…6a3d13` | raw capture |
| `jp_kantei_rekidai_090` | 28,838 | `3cf8487d3613…b89cbe` | raw capture |
| `jp_kantei_fukuda_kaiken_20070925` | 16,992 | `ef685ca58301…7e7d70` | raw capture |
| `jp_kantei_ccs_press_20070925_pm` | 2,406 | `a750b12c7219…9e6d47` | raw capture |
| `jp_kantei_fukuda_hossoku_20070926` | 4,282 | `0a4787142bcb…e2bfb0` | raw capture |
| `jp_kantei_fukuda_danwa_20070926` | 3,629 | `a03099ffe7d4…61596a` | raw capture |
| `jp_kunaicho_schedule_2007_h2` | 49,199 | `55f08b6f6d65…066d34` | raw capture |
| `jp_kunaicho_photo_20070926` | 6,368 | `425306b9ab54…845ecc` | raw capture |
| `jp_kantei_ccs_press_20070926_am` | 5,350 | `acd3970d9488…306d1f` | raw capture |
| `jp_kantei_fukuda_kaiken_20080901` | 12,859 | `f3b6f2cd913c…4b659d` | raw capture |
| `jp_kantei_fukuda_sojishoku_danwa_20080924` | 3,116 | `4d9425a5d215…8ad8e6` | raw capture |
| `jp_kantei_fukuda_jisyoku_20080924` | 2,545 | `be08035349c4…57a198` | raw capture |
| `jp_shugiin_giun_20080924` | 34,524 | `566ef27c2b80…f2ac02` | API meeting record |
| `jp_sangiin_giun_20080924` | 30,512 | `ae0f5586094f…977189` | API meeting record |
| `jp_shugiin_honkaigi_20080924` | 34,785 | `b8021c5f421d…223844` | API meeting record |
| `jp_sangiin_honkaigi_20080924` | 22,134 | `652c9632b1db…506700` | API meeting record |
| `jp_kantei_ccs_press_20080924_am` | 4,661 | `238f390a0927…eca014` | raw capture |
| `jp_kantei_rekidai_091` | 26,041 | `519c2c756fb4…445148` | raw capture |
| `jp_ryoin_kyogikai_20080924` | 20,729 | `e46fc671f999…1da0fd` | API meeting record |
| `jp_kantei_aso_hossoku_20080924` | 4,596 | `6d02091e465e…c06af1` | raw capture |
| `jp_kunaicho_schedule_2008_h2` | 50,261 | `bcdbed9038e6…f55f50` | raw capture |
| `jp_kunaicho_photo_20080924` | 1,374 | `afc7578a08ad…58cc2e` | raw capture |
| `jp_kantei_ccs_press_20080925` | 4,896 | `5c031e141fc2…e52cda` | raw capture |
| `jp_kantei_aso_kaiken_20090916` | 8,072 | `06f1e46a55b4…1f507f` | raw capture |
| `jp_kantei_aso_jisyoku_20090916` | 5,848 | `29996fb87746…cad51c` | raw capture |
| `jp_sangiin_giun_20090916` | 27,105 | `84ff5f02fcd6…25046d` | API meeting record |
| `jp_kantei_rekidai_092` | 19,314 | `bfcdce324760…dde82d` | raw capture |
| `jp_shugiin_honkaigi_20090916` | 69,025 | `a7591492e2e5…0191a9` | API meeting record |
| `jp_sangiin_honkaigi_20090916` | 21,124 | `a09db976b256…1d4c62` | API meeting record |
| `jp_kantei_hatoyama_hossoku_20090916` | 7,876 | `7386aac30ea6…905a0f` | raw capture |
| `jp_kunaicho_schedule_2009_h2` | 53,863 | `328d85d3cce0…57cbbc` | raw capture |
| `jp_kunaicho_photo_20090916` | 6,689 | `395bad47d1f6…f51ebb` | raw capture |
| `jp_kantei_ccs_press_20090916_pm` | 4,594 | `3d15f88d33de…850bf8` | raw capture |
| `jp_kantei_ccs_press_20100602_am` | 20,726 | `50e1736be735…953988` | raw capture |
| `jp_kantei_ccs_press_20100602_pm` | 22,082 | `fd8e24eac00a…a28602` | raw capture |
| `jp_kantei_hatoyama_hatsugen_20100604` | 116,842 | `fd493fffe4a7…6740cb` | raw capture |
| `jp_kantei_hatoyama_statement_index_201006` | 16,349 | `6b961423c3d8…aa6234` | raw capture |
| `jp_kantei_hatoyama_sojishoku_danwa_20100604` | 4,641 | `8a1d43d763a6…4d4b13` | raw capture |
| `jp_kantei_hatoyama_soujisyoku_20100604` | 33,665 | `7fd14d3b3817…12f26e` | raw capture |
| `jp_shugiin_giun_20100604` | 6,481 | `5661133c5283…976e58` | API meeting record |
| `jp_sangiin_giun_20100604` | 7,995 | `3be7ff85858e…a444e7` | API meeting record |
| `jp_shugiin_honkaigi_20100604` | 20,798 | `a8d6b302e097…f291d5` | API meeting record |
| `jp_sangiin_honkaigi_20100604` | 6,008 | `92025d9c9a7d…e84fb5` | API meeting record |
| `jp_kantei_ccs_press_20100604_am` | 26,843 | `499a3e64716a…8eabb7` | raw capture |
| `jp_sangiin_honkaigi_20100615_s004` | 33,991 | `6baa619bd7c4…8c51eb` | API speech |
| `jp_sangiin_honkaigi_20100615_s005` | 28,869 | `1c7a80829164…a77ca6` | API speech |
| `jp_kantei_rekidai_093` | 20,961 | `8b82f66c2726…21a616` | raw capture |
| `jp_kantei_kan_shimei_20100604` | 12,851 | `7946fef10bd0…09ea71` | raw capture |
| `jp_kunaicho_schedule_2010_q2` | 92,768 | `7877d61a685f…cc115d` | stored official file |
| `jp_kunaicho_photo_20100608` | 6,848 | `c6efd730446c…0e898f` | stored official file |
| `jp_kantei_kan_kaiken_20100608` | 37,934 | `4f556ce00846…a3d572` | raw capture |
| `jp_kantei_kan_hossoku_20100608` | 17,434 | `f1b653fb2d91…017d2a` | raw capture |
| `jp_kantei_ccs_press_20100608_pm` | 21,626 | `34a6ecba1fbf…f22563` | raw capture |
| `jp_shugiin_honkaigi_kan_speech_20100611` | 1,838 | `d12c6b39420f…ab65de` | API speech |
| `jp_kantei_kan_kaiken_20110826` | 26,303 | `48b2aa965738…8c9f04` | raw capture |
| `jp_shugiin_giun_20110830` | 6,980 | `86f4b11bfcd8…ee15b0` | API meeting record |
| `jp_shugiin_honkaigi_20110830` | 21,275 | `522c09dd8f68…5af50e` | API meeting record |
| `jp_sangiin_giun_20110830` | 8,022 | `2c2d0af7d8b6…f2cbcd` | API meeting record |
| `jp_sangiin_honkaigi_20110830` | 9,135 | `8a239d08ea27…3b979d` | API meeting record |
| `jp_kantei_kan_sojishoku_danwa_20110830` | 3,633 | `d78770727ca1…edf675` | raw capture |
| `jp_kantei_kan_bousai_20110901` | 59,181 | `0c8d3df84c20…5b87b1` | raw capture |
| `jp_kantei_kan_sojishoku_20110902` | 56,136 | `1392b41c292a…39859d` | raw capture |
| `jp_sangiin_yosan_20110928_s112` | 2,147 | `f29f84f547d1…be671b` | API speech |
| `jp_sangiin_yosan_20110928_s113` | 1,636 | `f5295a72a0b0…1484dc` | API speech |
| `jp_sangiin_yosan_20110928_s115` | 1,200 | `c37e04da8418…b51ff3` | API speech |
| `jp_kantei_rekidai_094` | 43,705 | `cb3d28f2479e…58fb91` | raw capture; live copy is a later version |
| `jp_kunaicho_schedule_2011_q3` | 91,105 | `45fe2de56875…3932fa` | stored official file |
| `jp_kunaicho_photo_20110902` | 6,873 | `6b166201772c…dd966f` | stored official file |
| `jp_kantei_noda_hossoku_20110902` | 16,888 | `583e8de4d2fd…3d571d` | raw capture |
| `jp_kantei_noda_kaiken_20110902` | 36,456 | `135937c14e00…37d808` | raw capture |
| `jp_kantei_noda_siji_20110902` | 22,460 | `9ae7737d0810…335a71` | raw capture |
| `jp_shugiin_honkaigi_noda_speech_20110913` | 3,606 | `53172a0b9a33…680432` | API speech |
| `jp_sangiin_yosan_20110928_s119` | 1,338 | `44a42e4f2ded…e58e2f` | API speech |
| `jp_sangiin_giun_20121226` | 30,068 | `b643ae687f64…dcb34b` | API meeting record |
| `jp_kantei_noda_sojishoku_20121226` | 16,317 | `dd2eabb7cf9e…e0ff1c` | raw capture |
| `jp_kantei_noda_sojishoku_danwa_20121226` | 15,799 | `5b0af54e4218…4a7926` | raw capture |
| `jp_kantei_rekidai_095` | 58,157 | `95a458c91735…e91caf` | raw capture |
| `jp_shugiin_honkaigi_20121226` | 69,616 | `ae6c621d5abf…b64f34` | API meeting record |
| `jp_sangiin_honkaigi_20121226` | 27,489 | `a0cfc74d6b9c…92247d` | API meeting record |
| `jp_kunaicho_schedule_2012_q4` | 113,891 | `5b5eef31a0bb…ab9426` | stored official file |
| `jp_kunaicho_photo_20121226` | 6,858 | `5bfe99768d3c…663b6e` | stored official file |
| `jp_kantei_abe_danwa_20121226` | 19,811 | `83bcd2fd9f0a…14d89f` | raw capture |
| `jp_kantei_abe_hossoku_20121226` | 24,333 | `814d31c1a93f…2427d6` | raw capture |
| `jp_kantei_abe_kaiken_20121226` | 43,441 | `af9f9f46e876…fc0ea7` | raw capture |
| `jp_kantei_abe_designation_20121226` | 21,392 | `9ebdc605f11b…e2663a` | raw capture |
| `jp_shugiin_honkaigi_abe_speech_20130128` | 15,385 | `b17196d4b46e…4c8e3f` | API speech |
| `jp_sangiin_giun_20141224` | 26,007 | `298d5b66515a…2c1ed3` | API meeting record |
| `jp_shugiin_honkaigi_20141224` | 69,311 | `74a5b662a324…74ada3` | API meeting record |
| `jp_sangiin_honkaigi_20141224` | 11,613 | `783d31221ee9…480ebf` | API meeting record |
| `jp_kunaicho_schedule_2014_q4` | 105,206 | `344fcb463831…ae952e` | stored official file |
| `jp_kunaicho_photo_20141224` | 6,845 | `735f834f6704…e32915` | stored official file |
| `jp_kantei_abe_hossoku_20141224` | 16,738 | `b23ba888225a…966ba2` | raw capture |
| `jp_kantei_abe_kaiken_20141224` | 33,673 | `315cf95d110c…c279ae` | raw capture |
| `jp_kantei_abe_designation_20141224` | 13,763 | `9e33a5c14344…6676b9` | raw capture |
| `jp_shugiin_honkaigi_abe_speech_20150212` | 38,562 | `6f627d9bdc85…4cd8df` | API speech |
| `jp_sangiin_giun_20171101` | 21,659 | `04b451138b94…6f7286` | API meeting record |
| `jp_shugiin_honkaigi_20171101` | 69,250 | `0511b93adcf4…76edf5` | API meeting record |
| `jp_sangiin_honkaigi_20171101` | 11,246 | `1f7912f9a82e…adb5dc` | API meeting record |
| `jp_kunaicho_schedule_2017_q4` | 107,791 | `cfb8239c6a55…6f5417` | stored official file |
| `jp_kunaicho_photo_20171101` | 6,850 | `81e9cac165dd…916deb` | stored official file |
| `jp_kantei_abe_hossoku_20171101` | 13,255 | `358055449888…9a86fc` | raw capture |
| `jp_kantei_abe_kaiken_20171101` | 30,141 | `8b8b96ea141d…10d710` | raw capture |
| `jp_kantei_abe_designation_20171101` | 9,378 | `5d86abb9c8b5…aeef54` | raw capture |
| `jp_shugiin_honkaigi_abe_speech_20171117` | 11,605 | `f98ba62851cf…5996eb` | API speech |
| `jp_kantei_abe_kaiken_20200828` | 62,923 | `1bee7ee383dc…ed31c7` | raw capture |
| `jp_shugiin_giun_20200916` | 18,970 | `e48a623493bd…5dacba` | API meeting record |
| `jp_shugiin_honkaigi_20200916` | 21,827 | `5e0b66751c83…698cdd` | API meeting record |
| `jp_sangiin_giun_20200916` | 30,559 | `8e6465ec3bdb…e9e024` | API meeting record |
| `jp_sangiin_honkaigi_20200916` | 17,519 | `c0d20dbb3ec6…f1cbca` | API meeting record |
| `jp_kantei_abe_sojishoku_20200916` | 8,674 | `c3a8db1bf67f…57f437` | raw capture |
| `jp_kantei_abe_sojishoku_danwa_20200916` | 8,106 | `aa2e117a5029…6ff993` | raw capture |
| `jp_kanpo_gogai_toku99_20200916_p2` | 308,013 | `8f7729b13815…e0a744` | raw capture |
| `jp_kantei_rekidai_096` | 85,785 | `afeb71fde9c1…183fab` | raw capture |
| `jp_kantei_rekidai_097` | 169,110 | `a5037eed2a14…988a49` | raw capture |
| `jp_kantei_rekidai_098` | 159,667 | `f433d6442ccb…222e5f` | raw capture |
| `jp_kantei_suga_designation_20200916` | 10,667 | `55065f6ce5f8…9383d1` | raw capture |
| `jp_kantei_suga_cabinet_launch_20200916` | 15,329 | `fe8bd4514b05…457b9e` | raw capture |
| `jp_kantei_pm_statement_20200916` | 7,104 | `90455bbeb1dd…ba0d32` | raw capture |
| `jp_kantei_suga_press_conference_20200916` | 33,531 | `c9d509ccbcd8…ce637c` | raw capture |
| `jp_kunaicho_schedule_entry_20200916` | 7,702 | `3aefa0e939fd…6d3903` | stored official file |
| `jp_kantei_cabinet_minutes_20200916_first` | 365,264 | `79133cfc8cfd…27c0b3` | raw capture |
| `jp_shugiin_giun_20211004` | 19,535 | `b321d64999b3…bb1793` | API meeting record |
| `jp_sangiin_giun_20211004` | 28,076 | `e98e6bb98ea5…f5bb29` | API meeting record |
| `jp_shugiin_honkaigi_20211004` | 22,423 | `2f5816ef662b…2e4cd8` | API meeting record |
| `jp_sangiin_honkaigi_20211004` | 16,543 | `83e3000441c2…e915bb` | API meeting record |
| `jp_kantei_suga_cabinet_resignation_20211004` | 8,608 | `38e701ed3a37…487574` | raw capture |
| `jp_kantei_suga_resignation_statement_20211004` | 10,268 | `c9257e57563c…5a2b34` | raw capture |
| `jp_kanpo_gogai_toku83_20211004_p2` | 297,975 | `695e627611fd…0cde02` | raw capture |
| `jp_kantei_cabinet_minutes_20211004_resignation` | 353,635 | `8b347a6b6367…d7b2af` | raw capture |
| `jp_kantei_kishida_designation_20211004` | 10,588 | `5170affd0037…405960` | raw capture |
| `jp_kantei_kishida_cabinet_launch_20211004` | 12,716 | `46ee6cbe65f1…d41a9d` | raw capture |
| `jp_kantei_pm_statement_20211004` | 6,738 | `80e4198136da…4de009` | raw capture |
| `jp_kunaicho_schedule_entry_20211004` | 7,706 | `ff7246283679…b946e2` | stored official file |
| `jp_kanpo_gogai_toku83_20211004` | 250,230 | `324e48c8d0bd…7aacde` | raw capture |
| `jp_kantei_cabinet_minutes_20211004_first` | 413,376 | `c4bcebc79569…681e9e` | raw capture |
| `jp_sangiin_giun_20211110` | 30,637 | `dfbb3b21e9b5…c73572` | API meeting record |
| `jp_shugiin_honkaigi_20211110` | 67,903 | `22474ba4b073…2e2441` | API meeting record |
| `jp_sangiin_honkaigi_20211110` | 19,518 | `f0cb15dc80af…7dfb30` | API meeting record |
| `jp_kantei_kishida_designation_20211110` | 10,792 | `68f88b365538…274dc3` | raw capture |
| `jp_kantei_kishida_cabinet_launch_20211110` | 11,264 | `a4290d124625…d6fc89` | raw capture |
| `jp_kantei_pm_statement_20211110` | 7,407 | `f7de9e3b3085…bb196b` | raw capture |
| `jp_kunaicho_schedule_entry_20211110` | 7,707 | `712dd8a13e78…bdf4ab` | stored official file |
| `jp_kanpo_gogai_toku88_20211110` | 257,370 | `a8aa962c9f3d…b5562e` | raw capture |
| `jp_kanpo_gogai_toku88_20211110_p2` | 306,712 | `4185c6b54abb…d8c328` | raw capture |
| `jp_kantei_cabinet_minutes_20211110_resignation` | 242,978 | `0370d1ea8355…b91159` | raw capture |
| `jp_kantei_cabinet_minutes_20211110_first` | 367,143 | `693d6a5b5937…66234f` | raw capture |
| `jp_shugiin_giun_20241001` | 51,168 | `262571856802…60b470` | API meeting record |
| `jp_sangiin_giun_20241001` | 40,603 | `632ff7359336…b95039` | API meeting record |
| `jp_shugiin_honkaigi_20241001` | 34,514 | `438c99bebd79…3ccc98` | API meeting record |
| `jp_sangiin_honkaigi_20241001` | 39,904 | `da8875048275…add626` | API meeting record |
| `jp_kantei_kishida_cabinet_resignation_20241001` | 12,961 | `217ddea14d6a…ac58ee` | raw capture |
| `jp_kantei_kishida_resignation_statement_20241001` | 12,639 | `fc1b3ff22ecd…58082e` | raw capture |
| `jp_kanpo_gogai_toku45_20241001` | 517,482 | `e4db13ba6a83…6dddf1` | raw capture |
| `jp_kantei_cabinet_minutes_20241001_resignation` | 165,404 | `48cb136c5551…b7c761` | stored official file |
| `jp_kantei_ishiba_designation_20241001` | 13,950 | `c92bf4001e52…225e2f` | raw capture |
| `jp_kantei_ishiba_cabinet_launch_20241001` | 16,982 | `c37bc56180e6…3e5355` | raw capture |
| `jp_kantei_pm_statement_20241001` | 8,548 | `48e63a195354…0d3a3a` | raw capture |
| `jp_kantei_ishiba_press_conference_20241001` | 53,768 | `edeed5f7bc70…d3aec5` | stored official file |
| `jp_kunaicho_schedule_entry_20241001` | 7,847 | `ac9e70484413…76d9ad` | stored official file |
| `jp_kantei_cabinet_minutes_20241001_first` | 285,754 | `65c4a182b738…2a65c4` | stored official file |
| `jp_sangiin_giun_20241111` | 32,129 | `6064ebf5cfc4…2a9df5` | API meeting record |
| `jp_shugiin_honkaigi_20241111` | 86,948 | `acb7c5e8007e…ee2cd9` | API meeting record |
| `jp_sangiin_honkaigi_20241111` | 37,390 | `0b4d7a395ab2…1cf214` | API meeting record |
| `jp_kantei_ishiba_designation_20241111` | 14,102 | `9cf429657d3b…5306ce` | raw capture |
| `jp_kantei_ishiba_cabinet_launch_20241111` | 15,416 | `9b586e745e9d…88eb25` | raw capture; live copy is a later version |
| `jp_kantei_ishiba_press_conference_20241111` | 51,315 | `e1037a8013f9…d8d254` | stored official file |
| `jp_kunaicho_schedule_202411` | 78,199 | `79e8e33ffa8c…9a686a` | stored official file |
| `jp_kanpo_gogai_toku52_20241111` | 519,828 | `76bd3b3d1901…9b4beb` | raw capture |
| `jp_kantei_cabinet_minutes_20241111_resignation` | 190,589 | `812690497cc1…1736fe` | stored official file |
| `jp_kantei_cabinet_minutes_20241111_first` | 268,401 | `0777e5fcc9e4…b072dc` | stored official file |
| `jp_kantei_pm_statement_20241111` | 7,961 | `6ae0b7e2085c…cadd0b` | stored official file |
| `jp_shugiin_giun_20251021` | 36,580 | `c5175bb0d3c3…f95f57` | API meeting record |
| `jp_sangiin_giun_20251021` | 39,246 | `851d9f253aac…1c2cb4` | API meeting record |
| `jp_shugiin_honkaigi_20251021` | 31,062 | `1dfcf5da41e8…d40853` | API meeting record |
| `jp_sangiin_honkaigi_20251021` | 36,653 | `eed5fdf37c3e…80609d` | API meeting record |
| `jp_kantei_ishiba_cabinet_resignation_20251021` | 14,605 | `120a9720c4f3…025548` | raw capture |
| `jp_kantei_ishiba_resignation_statement_20251021` | 15,801 | `ca4bd3cdd8c4…958a02` | raw capture |
| `jp_kantei_takaichi_designation_20251021` | 14,440 | `eb85c976ef63…ba19a6` | raw capture |
| `jp_kantei_takaichi_cabinet_launch_20251021` | 16,509 | `0750e51b6a8c…fd5f92` | raw capture; live copy is a later version |
| `jp_kantei_pm_statement_20251021` | 8,019 | `64005495794b…9ad308` | raw capture |
| `jp_kantei_takaichi_press_conference_20251021` | 51,266 | `c48bfa07bd96…4e91c3` | stored official file |
| `jp_kunaicho_schedule_entry_20251021` | 7,848 | `29a52012284a…aca51f` | stored official file |
| `jp_kanpo_gogai_toku28_20251021` | 231,567 | `6f72a1d1ba17…b7ae85` | raw capture |
| `jp_kantei_cabinet_minutes_20251021_resignation` | 288,323 | `f4e6176c8d08…ffc8b8` | stored official file |
| `jp_kantei_cabinet_minutes_20251021_first` | 282,368 | `2875316d6027…ead753` | stored official file |
| `jp_sangiin_giun_20260218` | 26,539 | `9656ba58ed9d…575604` | API meeting record |
| `jp_shugiin_honkaigi_20260218` | 67,561 | `87f77bebda52…5c2d75` | API meeting record |
| `jp_sangiin_honkaigi_20260218` | 24,218 | `1ba084282bff…168abf` | API meeting record |
| `jp_kantei_takaichi_designation_20260218` | 15,150 | `4d10fa6a7e8b…662707` | raw capture |
| `jp_kantei_takaichi_cabinet_launch_20260218` | 15,491 | `b790e6790c09…acde87` | raw capture |
| `jp_kantei_pm_statement_20260218` | 8,363 | `544e03d2b56f…51fbb1` | raw capture |
| `jp_kantei_takaichi_press_conference_20260218` | 41,640 | `7396028759bb…ca82dc` | stored official file |
| `jp_kunaicho_schedule_entry_20260218` | 7,860 | `7f22216f8a54…cd32ce` | stored official file |
| `jp_kanpo_gogai_toku9_20260218` | 224,239 | `a25c3b3ad597…91e3ec` | raw capture |
| `jp_kantei_cabinet_minutes_20260218_resignation` | 117,991 | `2470987c925c…406b64` | stored official file |
| `jp_kantei_cabinet_minutes_20260218_first` | 206,407 | `34ccbadc58f5…a11827` | stored official file |
| `jp_shugiin_budget_committee_20260727` | 9,337 | `4b8d890afb79…47c141` | API speech |
| `jp_kantei_takaichi_recovery_hq_20260904` | 16,218 | `a1a8963358e7…3e67d2` | stored official file |

## Leads not imported

- Kantei press conferences of 24 September 2008 (https://web.archive.org/web/20080926193401id_/http://www.kantei.go.jp/jp/asospeech/2008/09/24kaiken.html)
  and 16 September 2009 (hatoyama/statement/200909/16kaiken.html): designation remarks before the ceremony, redundant with the Diet
  records and the Kantei accounts.
- The Kantei's cabinet agendas (閣議案件) of the transition days of 2020-2026 (e.g. https://www.kantei.go.jp/jp/kakugi/2024/kakugi-2024100101.html,
  kakugi-2021100401 and kakugi-2026021803): thirteen pages the part C check listed; each lists the items (内閣総辞職について, the
  statement, 内閣総理大臣を任命することについて) that the imported minutes of the same meetings record with the times and the
  attendance, so they are redundant and not imported.
- The Chief Cabinet Secretary's handout copy of the 4 June 2010 remarks (https://web.archive.org/web/20100613053201id_/http://www.kantei.go.jp:80/jp/tyoukanpress/201006/__icsFiles/afieldfile/2010/06/04/hatsugen_souri20100604.pdf;
  82,250 bytes, SHA-256 8b418eb12311…14b6b4): the same text as the recorded statement-folder PDF.
- The Chief Cabinet Secretary's records of the afternoon of 3 June 2010 (tyoukanpress/201006/3_p.html; 21,838 bytes, SHA-256
  b0c7740ddbbe…ae331cd; the Emperor's availability for 4 June, context only), of the afternoon of 13 September 2007
  (rireki/2007/09/13_p.html; 2,576 bytes, 54dcc3ded525…944ed0; the doctors' briefing, corroboration) and the index of June 2010
  (tyoukanpress/201006/index.html; 27,133 bytes, 3c22f6535caf…3a462; navigation).
- House of Representatives Rules and Administration Committee records of 26 December 2012 (118204024X00120121226), 24 December 2014
  (118804024X00120141224), 1 November 2017 (119504024X00120171101), 10 November 2021 (120604024X00120211110), 11 November 2024
  (121504024X00120241111) and 18 February 2026 (122104024X00120260218): read, and none mentions the resignation notice.
- The Imperial Household Agency's 2009 schedule index (https://web.archive.org/web/20090917023210id_/http://www.kunaicho.go.jp/activity/gonittei/01/gonittei01.html),
  which links schedules back to 平成2年 (1990): a lead for the 1990-2006 packet, which reported that the agency's website
  begins in late 1999; and the 2009-site copy of the 2007 schedule (h19/gonittei-1-2007-7.html) and an earlier 2009 capture
  (h21/gonittei-1-2009-3.html), used only to find pages.
- Hatoyama's activity index (hatoyama/actions/index.html), Abe's policy speech of 10 September 2007 (10syosin.html) and the
  agenda of the unheld sitting of 12 September 2007 printed in the 25 September record: navigation or context.
- Kishida's press conferences of 4 October 2021 (100_kishida/statement/2021/1004kaiken.html, a placeholder capture without the
  transcript; 6,793 bytes) and 10 November 2021 (1110kaiken.html, not retrieved).
- The Kantei's account of 1 September 2026 (https://www.kantei.go.jp/jp/105/actions/202609/01bousai.html; 24,256 bytes, stable):
  the 4 September account is used as the latest attestation. The account 17takaichinaikaku2.html is dated 17 September 2026, after
  the cutoff, and was not read.
- Budget Committee speech 5 of 27 July 2026 (api/speech?issueID=122105261X01720260727&speechNumber=5; 2,168 bytes): not imported; the
  attestation rests on speech 0 (会議録情報) alone.
- The Kantei's 歴代内閣 pages for the 99th-105th Prime Ministers (https://www.kantei.go.jp/jp/rekidainaikaku/099.html to 105.html):
  not reviewed; retrospective spans would be claims only (next work).
- Full-text Diet searches for 職務執行内閣, 第七十一条 and 内閣総理大臣臨時代理 in 2006-2026: discovery only; besides the imported
  speeches of 15 June 2010 and 28 September 2011 they found no record of continued duties or acting service.
- The Constitution (arts. 6, 67, 70 and 71), the Diet Act (arts. 64 and 65) and the Cabinet Act (art. 9) frame the procedure;
  statute text establishes procedure only, never a date. No news, encyclopaedia or history site is used, and the Democratic
  Party's materials on the 2 June 2010 meeting were not used.

## Sources attempted

- The Official Gazette (官報), 2006-2017: not located in an openly accessible, byte-stable form (the official site is free only for
  the latest 90 days and its past-gazette page is script-rendered; the 1990-2006 packet found the NDL copies restricted). No paid
  or registered service was used. Page 1 of issue No. 99 (16 September 2020; https://web.archive.org/web/20200917033216id_/https://kanpou.npb.go.jp/20200916/20200916t00099/pdf/20200916t000990001.pdf)
  returns the Wayback 404 page; its page 2 is recorded. The live issues of 2025 and 2026 on www.kanpo.go.jp return 404.
- NDL WARP copies of the Kan, Noda and Suga Kantei sites: CloudFront 403 on every request; not retried or bypassed.
- Live Kantei pages of past administrations: /jp/kan/ and /jp/rekidai/ redirect (301) to the archive index, /jp/noda/ is a stub,
  /jp/96_abe/ to /jp/98_abe/ and the Suga and Kishida pages return 404; captures are recorded instead.
- Kantei pages with no capture (Wayback 404): Aso's statements of 24 September 2008 and 16 September 2009, Hatoyama's of 16 September
  2009, the guessed June 2010 statement and activity pages, and Fukuda's 2008 photo page; the agency's pre-2009 schedule paths for
  2009-2010 and a guessed quarter URL on its redesigned site.
- The Kantei's 歴代内閣 captures nearest August 2026 for the 91st-93rd pages are archived 404 pages; earlier captures are recorded.
- The Internet Archive refused connections, returned 429 or reported itself temporarily offline at times on 24-25 September 2026
  while several agents shared the address; requests were retried one at a time, and no recorded identity is an error page.
- No terms, logins, CAPTCHAs or access controls were met or bypassed.

## Checker defects

| # | Defect | Outcome |
|---|---|---|
| A1 | Three press-conference URLs end in `press.html`, which the stacked test bans packet-wide | **Applied**: the sources are kept (they are the primary records); in `test_japan_prime_ministers_c01_12.py` the lead markers apply unchanged to that packet's own sources, and for this packet's sources the generic `press.html` and `kanpo` markers are re-expressed as the exact leads they excluded there (the 2003, 2005 and 2006 press conferences and the Kantei-hosted Gazette contents) |
| A2 | Three claims carry the withdrawn `attested_period` | **Applied**: removed everywhere; the hospitalisation recollection is dated by the day it recalls (13 September, now backed by the same-day record), the no-acting-Prime-Minister statement by its record (24 September), and the continued-duties statement is undated; tested as a stale id |
| A3 | A holder's recollection carries `attested_on` | **Applied**: `jp_fukuda_recalls_taking_office_20070926` and 菅直人's recollection are undated (`assumption_recalled_by_holder`); 麻生太郎's recollection of the launch stays dated by the day recalled, as the stacked packet does for that kind |
| A4 | The new holders and the 2006 rows collide with the stacked test, and the 2006 rows repeat that packet's responses | **Resolved**: the stacked test's rules and pins are scoped to its own records and holders (those dated before 26 September 2006, with `2006-09-26` kept in its never-a-holder-date set for them) and its exact lists are extended with this packet's pinned lists; the 2006 start rests on the three responses re-recorded under new ids, a deliberate repeated identity pinned in the test and left to the integrator's ruling |
| A5 | Hatoyama statement locator | **Applied**: first and seventh paragraphs |
| A6 | Abe 12 September 2007 locator | **Applied**: first and seventh paragraphs |
| A7 | Hatoyama 2009 account locator and surname-only naming | **Applied**: paragraphs 1-2, the press-conference paragraph and the one beginning 夜には; the uncertainty says the page prints only 鳩山総理 |
| A8 | "172nd special session after the general election" is not in the cited speech | **Applied**: the clause removed |
| A9 | The 2 June 2010 resignation announcement exists | **Applied**: both Chief Cabinet Secretary records of 2 June imported (the announcement reported, the government to carry on, Hatoyama's transition instruction); the unresolved item removed |
| A10 | The 4 June 2010 record and its 262-day span | **Applied**: imported; it attributes and dates the remarks, and the span is a claim with no structured date in the end conflict; the PDF title and timing note are in the remarks' uncertainty |
| A11 | Continuation arrangements, the 2007 ceremony time, the 2009 schedule and the art. 9 orders | **Applied**: all imported as claims (the 2008 emergency arrangement; the 08:30 ceremony of 2007 and its schedule; the 19:00 schedule of 2009; the 2010 schedule; the acting orders of 2007 and 2008 with no holder name) |
| A12 | The 13 September 2007 hospital record | **Applied**: the morning record imported; the afternoon record is a lead (corroboration only) |
| A13 | The unnamed statements of 2007 and 2008 | **Applied**: their rows keep no holder name; the uncertainties cite the Chief Cabinet Secretary's same-day records |
| A14 | Ceremony rows restate the designation | **Applied**: each ceremony claim now states the ceremony and each formation claim the launch sentence; designation context is kept only as a subordinate clause where the account runs them together |
| A15 | Corroborating kinds must stay out of holders | **Applied**: holders cite only named start, end and in-office rows; every other kind stays on the role; tested |
| B1 | Six claims duplicated with part A | **Applied**: one source per response; part A's claim ids kept, with 鳩山由紀夫 as the row holder of the two continued-duties remarks; part B's Kan rows kept in the same extracts |
| B2 | Four continued-duties claims use `attested_period` | **Applied**: undated, with the interval in the text; `period` is not used either (the stacked rules forbid it on this office) |
| B3 | Formation claims describe the first cabinet meeting | **Applied**: each formation claim states the account's launch sentence; the ceremony claims no longer say "formally launching"; the first cabinet meeting is mentioned in the uncertainty only |
| B4 | The 歴代内閣 pages 094-098 were skipped for a wrong reason | **Applied**: imported as retrospective spans with no structured date; the 94th capture's later live version is its alternate location |
| B5 | Designation accounts and resignation statements left out | **Applied**: the 2012, 2014 and 2017 designation accounts (with the 2014 and 2017 morning resignations) and the 2012 and 2020 resignation statements imported |
| B6 | Speech 119 not downloaded | **Applied**: imported as an undated continued-duties recollection whose 29 August date both Houses' minutes contradict |
| C1 | The Gazette states six ends | **Applied**: the loss-of-office notices of Nos. 83, 88, 45, 52, 28 and 9 are claims (`end_of_office_stated`) and give those holders `until`; No. 99 gives a seventh, 安倍晋三 16 September 2020; the resignation uncertainties no longer say no source states an end |
| C2 | Gazette page 2 unread and "no other date" wrong | **Applied**: page 2 read from renders in Nos. 45, 52, 28 and 9 (pages 1 and 2); the uncertainties give the closing (以上…) line |
| C3 | Page 2 of Nos. 83 and 88 | **Applied**: imported as sources with the loss-of-office claims |
| C4 | Suga's appointment is on page 2 of No. 99 | **Applied**: imported; one source carries 菅義偉's appointment (JP-PM06-08) and 安倍晋三's loss of office (JP-PM06-07) |
| C5 | Continuation instructions in the 2021, 2024 and 2025 minutes | **Applied**: imported as dated claims only; the minutes of 10 November 2021, 11 November 2024 and 18 February 2026 record none, as their claims say |
| C6 | The live pages do carry the ceremony sentence | **Applied**: the notes corrected; the live copies are the alternate locations |
| C7 | "again" in the 10 November 2021 statement | **Applied**: removed |
| C8 | House-neutral first-ballot kind | **Applied**: the House-specific kinds |
| C9 | New kinds differ from the stacked vocabulary | **Applied**: press conferences are `in_office_attestation`, unnamed statements `appointment_statement` or `assumption_statement` with no holder name, Kantei resignation accounts `cabinet_resignation_decided`, runoffs the base designation kinds |
| C10 | The 27 July 2026 sitting was a recess sitting | **Applied**: 閉会中審査 in the title and text |
| C11 | Gazette publisher anachronism | **Applied**: the National Printing Bureau for the 2020-2024 issues, the Cabinet Office for 2025-2026 |
| C12 | No statement claim for 11 November 2024 | **Applied**: the Kantei statement imported, with no holder name |
| C13 | Holder rows use `holder_name` | **Applied**: holders use the packet's `name` shape; `holder_name` stays in extract rows only |

Missing primary records the checks found: part A's fifteen: eleven imported (the Chief Cabinet Secretary's records of 13 and
25 September 2007 morning, 25 September 2007 evening, 26 September 2007, 24 and 25 September 2008, 16 September 2009, 2 June
2010 morning and afternoon, 4 June 2010 and 8 June 2010) and four listed as leads (3 June 2010, the June 2010 index, the handout
PDF and 13 September 2007 afternoon). Part B's twelve: all imported (the 2020 statement from a later capture of the same bytes).
Part C's thirty-two: seventeen imported as sources (page 2 of Gazette Nos. 99, 83 and 88, the thirteen minutes and the 11 November
2024 statement), two live Kantei pages recorded as alternate locations, and the thirteen agenda pages listed as leads, redundant
with the minutes. This packet downloaded each imported record again, as for every source.

Other changes made to fit the packet's rules rather than a numbered defect:

- One event-kind vocabulary with C01-12: `cabinet_resignation_notice`, `cabinet_resignation_decided`,
  `cabinet_resignation_statement`, the House-specific designation and first-ballot kinds, `appointment_statement` and
  `assumption_statement`; new kinds only where C01-12 has no counterpart: `appointment_notice_official_gazette`,
  `end_of_office_stated` (which C01-12's rules already reserve for a stated end), `designation_account`,
  `presiding_officers_report_to_emperor`, `prime_minister_report_to_emperor`, `resignation_intent_announced`,
  `resignation_intent_reported`, `resignation_intent_recalled`, `prospective_resignation_statement`, `final_cabinet_meeting_remarks`,
  `departure_from_prime_ministers_office_recorded`, `continued_performance_of_duties` (with `_arranged`, `_recalled` and
  `_act_recalled`), `acting_prime_minister_not_designated`, `acting_prime_minister_order_designated`,
  `imperial_appointment_ceremony_scheduled`, `prospective_assumption_statement`, `hospital_visit_reported`,
  `hospitalisation_recalled_by_holder`, `statement_index_listing` and `cabinet_span_stated_by_chief_cabinet_secretary`; all pinned.
- The formation halves of the 090-095 retrospective pages are dropped, as C01-12's check A6 did for its pages.
- The part B decisions ("accepted") for JP-PM06-05 to 07 become "accepted in part" under the outcome rule above; nothing in them
  changes.
- `published_date` is null for every new source.

## Suggested next work orders

These are proposals for the integrator. They are not created in `work-orders.json`.

- `C01-Japan-PM-006`: the Official Gazette's appointment and loss-of-office notices of 2006-2017, from lawful access (an NDL reading
  room or the paid Gazette search), to state the ends of the 2006-2014 appointments and settle the Hatoyama conflict.
- `C01-Japan-PM-007`: the Kantei's 歴代内閣 pages 099-105 as retrospective claims.
- `C01-Japan-PM-005` (proposed by C01-12): acting prime ministers and the art. 9 orders, 1990-2026; this packet adds the 2007 and
  2008 orders.

## Integration notes (outside this packet's file boundary)

- **Stack:** this branch holds the claim commit `3bcd6e6c`, a merge of `claude/c01-jp-12` up to `da358dbf` (the stacked packet's
  report-only verifier fixes), and this packet's two commits. Merge CLAUDE-C01-12 first; `japan.json` and the two Japan tests are
  shared with it; this packet adds to `japan.json` without changing existing text (its three institution coverage notes sit before C01-12's closing 'Keep executive office distinct from party leadership' note, which stays last) and re-scopes the two tests' pins as listed below.
- `research-index.json` is regenerated in a **separate commit** and is the only file shared with other pending packets (the
  India, Brazil and South Africa packets add their own sources and claims); regenerate it after they merge. New Japan totals: 337
  sources and 496 claims (C01-12: 126 and 203); institutions, roles, `mapping_pending` (24) and the Japan work orders (10, 10 and
  4 members) are unchanged.
- **The 2006 ruling:** if the integrator prefers, amend C01-12's rows `jp_abe_appointed_pm_statement_20060926`,
  `jp_abe_shinninshiki_20060926` and `jp_kunaicho_ceremony_abe_20060926` to name 安倍晋三 (and C01-12's test pins), cite them from
  the 2006 holder, and drop the three re-recorded sources; this packet's test pins the repeated identities so the change is
  visible either way.
- Pinned tests, none loosened and no assertion removed:
  - `test_japan_research_s10d.py`: counts (entries, sources, claims, roles) are now (24, 337, 496, 5), with this packet's 211
    sources and 293 claims pinned beside C01-12's 119 and 174; the 2026-09-24 access date still applies to C01-12's sources, and
    this packet's (2026-09-24 and 2026-09-25) are pinned per source in its own test; the extract count is 2 + 119 + 211.
  - `test_japan_prime_ministers_c01_12.py`: its rows are loaded from its own sources; its holder rules apply to its own holders
    (those dated before its closing boundary, 26 September 2006, which stays in its never-a-holder-date set), while chronological
    order is checked across all holders and a later holder may never cite its claims; its exact source, claim and holder lists are
    its own followed by this packet's pinned lists (imported from `test_japan_prime_ministers_c01_13.py`); its "no end" checks
    apply to its own holders; the packet coverage note it pins is at index 8 of 10; its archive-only rule for Kantei and agency
    pages applies to its own sources, with this packet's stored official pages pinned exactly; and its lead markers apply to its
    own sources unchanged and to this packet's sources with `press.html` and `kanpo` re-expressed as the exact leads.
- `test_campaign_census` needs `spheres-sim/data`, which is absent from this sparse worktree; it was not run, and the sparse
  checkout was not widened.
- The atlas (`tools/ui/leadership-research-review.js`) shows only "Observed on" when a holder has `attested_on`; no holder here
  has `attested_on`, and seven have both `from` and `until`. No UI code changed.
- New fields on Japan sources and extracts are those of C01-12, plus `alternate_location` on three extracts.
- `research/README.md`, the C01 README totals and `docs/planning/ai-workstreams.json` are left for the integrator; this handoff is
  self-proposed and not registered there.

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
