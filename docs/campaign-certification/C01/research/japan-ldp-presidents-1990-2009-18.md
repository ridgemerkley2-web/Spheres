# LDP presidents 18: the party office, 1990-2009

Packet: **CLAUDE-C01-18**. State: **ready_for_review** (not complete).
Owner: Claude. Integrator/reviewer: Codex. Branch `claude/c01-jp-18`, **stacked on CLAUDE-C01-13** (`claude/c01-jp-13` at
`42f98dc8`, itself stacked on CLAUDE-C01-12 and based on `codex/campaign-certification` at `ffe54b02`); claim commit `30c507d7`.
Research access: 25 September 2026 (UTC). The historical cutoff stays **7 September 2026**.

This packet extends the existing party role `jp_ldp_party_president` (総裁 — party president, kind `party_leader`) on the
自由民主党 organization observation `jp_sangiin_pr_2025_13` in [japan.json](japan.json), which held only 石破茂 (2024) and
高市早苗 (2025). It reviews ten observations, from the President when the period opens (海部俊樹) to the September 2009
election won by 谷垣禎一, and adds 81 sources, 189 claims and fourteen holder observations, placed before the unchanged 2024 and
2025 observations, with a role scope-note addition, one organization coverage note and one packet coverage note. It adds no
organization, institution, role, game mapping, lifespan, portrait or avatar, and it changes no existing source, claim, holder or
extract: the thirty `jp_pm` holders of CLAUDE-C01-12 and CLAUDE-C01-13 and the 2024 and 2025 party observations stay as they were.
The parent scope (C01, C06, S23, WC1 and CP1) remains open.

The research was done in three parts (A: 1989-1995, LDP-PRES-01 to 04; B: 1998-2003, LDP-PRES-05 to 07; C: 2006-2009, LDP-PRES-08
to 10), and each part was checked independently before this packet was written. Every checker defect is applied or resolved
(see [Checker defects](#checker-defects)); of the 21 missing primary records the checks confirmed, 14 are imported as sources, one
is recorded as an alternate location, one is the recorded location of a source, four are leads with the reason, and one (the
contemporaneous party records of 1989-1995) could not be found online.

## Outcome

An observation is **Accepted** where every holder in it has a stated start and every presidency that ended before the cutoff has
a stated end; otherwise it is **Accepted in part**. No party record reviewed states the day a presidency of 1989-2009 ended, so
every observation is accepted in part.

| ID | Question | Decision |
|---|---|---|
| LDP-PRES-01 | The President when the period opens (海部俊樹), and his end if stated | **Accepted in part:** 海部俊樹, observed 14 May 1990 in his own Diet answer (私、自民党総裁としても); the party's records of his 1989 election and October 1989 re-selection and of his withdrawal from the October 1991 election are retrospective claims; nothing attests 1 January 1990 itself and no end is stated |
| LDP-PRES-02 | October 1991: the election won by 宮澤喜一 | **Accepted in part:** the 27 October 1991 election (285, 120, 87), the 29 October convention approval and the 31 October start are retrospective; 森政調会長 and 宮澤 himself refer to the election without its day; observed 30 November 1992 in his explicit '私は自由民主党の総裁でございます'; no start or end |
| LDP-PRES-03 | 1993: the election won by 河野洋平, never Prime Minister | **Accepted in part:** the 30 July 1993 joint plenary election (208, 159) and the second selection (notice 17 September, 57th convention 30 September 1993) are retrospective; 河野 recalls the resignations, his selection and his 総裁声明 without a day; observed 25 November 1994 ('自由民主党の総裁とはいえ'); no start or end |
| LDP-PRES-04 | 1995: the election won by 橋本龍太郎 | **Accepted in part:** the 22 September 1995 vote (304, 87), the 25 September convention and the 1 October start are retrospective; his own statement dates the convention's selection to 25 September (a selection, not a start); observed 2 October 1995; 橋本総裁 before the 1998 vote and 橋本前総裁 after it are claims; no end |
| LDP-PRES-05 | 1998 and 1999: 小渕恵三's election and re-election | **Accepted in part:** 24 July 1998 count and result (225, 102, 84; announced 15:13) and 小渕新総裁 after the decision (observed 24 July 1998); 21 September 1999 count (350 = 253 + 97) and announcement, the report to the 65th extraordinary convention and 小渕総裁's greeting and press conference on 22 September (observed 22 September 1999); no start or end |
| LDP-PRES-06 | 2000: 森喜朗's selection after 小渕恵三's incapacity | **Accepted in part:** from 5 April 2000, on his own statement at the joint plenary meeting that he has just now come to take office as the 19th president (ただ今…就任致すことになりました); the selection, 小渕前総裁, his 2001 early-election announcement and 森総裁 voting on 24 April 2001 are claims; no end |
| LDP-PRES-07 | 2001-2003: 小泉純一郎's election, reappointment and re-election | **Accepted in part:** three observations: 24 April 2001 (就任 greeting and first press conference after taking office), 10 August 2001 (reappointment greeting) and 20 September 2003 (press conference after re-election); notices, votes, results, reports and the list's reappointment decision are claims; no start or end |
| LDP-PRES-08 | 2006 and 2007: the elections won by 安倍晋三 and 福田康夫 | **Accepted in part:** 安倍晋三 observed 20 September 2006 (464 votes; address as 新総裁, first press conference since taking office), with a conflict on the term start (30 September expiry, 1 October list start); resignation announced 12 September 2007; 福田康夫 observed 23 September 2007 (330 to 197); no start or end for either |
| LDP-PRES-09 | 2008: the election won by 麻生太郎 | **Accepted in part:** 福田's resignation announced 1 September 2008; notice 10 September; 22 September 2008 vote (351), declaration by 臼井 and 麻生's address, press conference and appointments (observed 22 September 2008); no start or end |
| LDP-PRES-10 | 2009: the election won by 谷垣禎一, never Prime Minister | **Accepted in part:** 麻生太郎総裁's 8 September statement that he would resign with the cabinet on the 16th (prospective); notice 18 September; 28 September 2009 count (300 = 120 + 180), 野田毅's declaration, 谷垣新総裁's address and first press conference as president (observed 28 September 2009), 麻生太郎前総裁 the same day; conflicting term start and 麻生's end unresolved |

The resulting holder observations of `jp_ldp_party_president`, in date order (the last two are unchanged):

| Holder | `attested_on` | `from` | `until` | Cited claims |
|---|---|---|---|---|
| 海部俊樹 | 1990-05-14 | null | null | `jp_ldp_diet_kaifu_speaks_as_president_19900514` |
| 宮澤喜一 | 1992-11-30 | null | null | `jp_ldp_diet_miyazawa_is_president_19921130` |
| 河野洋平 | 1994-11-25 | null | null | `jp_ldp_diet_kono_is_president_19941125` |
| 橋本龍太郎 | 1995-10-02 | null | null | `jp_ldp_diet_hashimoto_speaks_as_president_19951002` |
| 小渕恵三 | 1998-07-24 | null | null | `jp_ldp98p3_obuchi_new_president_after_decision_19980724` |
| 小渕恵三 | 1999-09-22 | null | null | `jp_ldp99p9_obuchi_greeting_19990922`, `jp_ldp99aisatu_obuchi_greeting_as_president_19990922`, `jp_ldp99p10_obuchi_press_conference_after_election_19990922` |
| 森喜朗 | null | 2000-04-05 | null | `jp_ldpmori_stated_assumption_19th_president_20000405` |
| 小泉純一郎 | 2001-04-24 | null | null | `jp_ldp01d10_koizumi_inaugural_greeting_20010424`, `jp_ldp01d11_first_press_conference_after_assuming_20010424` |
| 小泉純一郎 | 2001-08-10 | null | null | `jp_ldpsainin_koizumi_reappointment_greeting_20010810` |
| 小泉純一郎 | 2003-09-20 | null | null | `jp_ldp03kaiken_press_conference_as_president_20030920` |
| 安倍晋三 | 2006-09-20 | null | null | `jp_ldp_abe_new_president_addresses_joint_plenary_20060920`, `jp_ldp_abe_first_press_conference_as_president_20060920`, `jp_ldp_joint_plenary_after_election_new_president_20060920` |
| 福田康夫 | 2007-09-23 | null | null | `jp_ldp_fukuda_new_president_address_20070923`, `jp_ldp_fukuda_first_press_conference_as_president_20070923` |
| 麻生太郎 | 2008-09-22 | null | null | `jp_ldp_aso_new_president_address_20080922`, `jp_ldp_aso_first_press_conference_as_president_20080922`, `jp_ldp_aso_appoints_officers_after_taking_office_20080922` |
| 谷垣禎一 | 2009-09-28 | null | null | `jp_ldp_sousai09_tanigaki_address_joint_plenary_20090928`, `jp_ldp_sousai09_tanigaki_first_press_conference_20090928` |
| 石破茂 | 2024-09-27 | null | null | unchanged (`jp_ishiba_president_20240927`) |
| 高市早苗 | 2025-10-04 | null | null | unchanged (`jp_takaichi_president_20251004`) |

### How a holder is dated

The packet follows the Japan rule of CLAUDE-C01-12 and CLAUDE-C01-13 and, for a party office kept apart from a state office,
the ANC presidents packet (CLAUDE-C01-16). A holder has `from` only where a source states the day office was assumed or took
effect, and `until` only where a source states the day it ended; otherwise it has `attested_on`. One holder observation is kept
per election or selection (as `jp_pm` splits a holder at each appointment and the ANC role keeps one per conference), so the 1999
re-election of 小渕恵三 and the 2001 reappointment and 2003 re-election of 小泉純一郎 have their own observations.

- **What dates an observation.** A same-day party record that names the person as the (new) president after the result is known
  (his address to the meeting as 新総裁, captions, his press conference as president, his appointments of officers), or the
  president's own Diet statement of the office. Each holder cites only such claims, each carrying exactly the holder's date.
- **What states a start.** Only a holder's own first-person statement, on a dated occasion, that he takes office now: 森喜朗's
  greeting of 5 April 2000 (ただ今…第１９代総裁に就任致すことになりました). 就任 wording in captions, headings and headlines
  (就任の挨拶, 就任後初の記者会見, 総裁就任記者会見, 就任後直ちに, 就任した谷垣禎一新総裁) says the office was held by that event
  but not the day it took effect. The party's own records show why this matters: in 2006 and 2009 its same-day pages call the new
  president 新総裁 and speak of 就任 on the vote day, while its election schedules list 30 September as the day the president's
  term expires and its later list starts the new terms on 1 October.
- **What never feeds a holder.** Party elections and selections (member and Diet-member votes, the joint plenary meetings of party
  Diet members held in place of a convention), notices (告示), candidacies, declarations and reports of results, index headlines,
  announced or prospective resignations, predecessor references (前総裁), withdrawals, continuation attestations (the incumbent
  before a vote, or a later attestation of the same observation), implicit references, and every retrospective list, history,
  chronology and narrative. Party rules and schedules are procedure or plans, never a date.
- **No end.** No party record or officer's statement reviewed states the day a 1989-2009 presidency ended. Announced
  resignations (12 September 2007, 1 September 2008), 麻生太郎総裁's statement of 8 September 2009 that he would resign with the
  cabinet on the 16th, the 前総裁 captions and references (24 July 1998, 5 April 2000, 28 September 2009), the schedules' 30
  September term expiries and the lists' printed ends are claims; no end is inferred from a successor's election.

Names follow the sources in Japanese, one holder string per person (海部俊樹, 宮澤喜一, 河野洋平, 橋本龍太郎, 小渕恵三, 森喜朗,
小泉純一郎, 安倍晋三, 福田康夫, 麻生太郎, 谷垣禎一). The party's web pages print 宮沢喜一 (沢); the Diet minutes and the party's
printed history print 宮澤喜一 (澤), the form already used for `jp_pm`, so every row carries 宮澤喜一 and the printed form stays in
the text (check defect A2). Where a source prints only a surname and title (海部総裁, 宮澤内閣総理大臣, 河野国務大臣, 小渕新総裁,
森総裁, 小泉内閣総理大臣, 安倍新総裁), the claim text keeps that form and the row carries the full name. A row's `holder_name` is
null where the source names no holder of this office: a notice, a filing, a schedule, an unnamed caption, a meeting or a
convention entry that names nobody. Romanized forms: 海部俊樹 Kaifu Toshiki, 宮澤喜一 Miyazawa Kiichi, 河野洋平 Kōno Yōhei, 橋本龍太郎
Hashimoto Ryūtarō, 小渕恵三 Obuchi Keizō, 森喜朗 Mori Yoshirō, 小泉純一郎 Koizumi Jun'ichirō, 安倍晋三 Abe Shinzō, 福田康夫 Fukuda
Yasuo, 麻生太郎 Asō Tarō and 谷垣禎一 Tanigaki Sadakazu.

### Party office and the prime-ministership

`jp_ldp_party_president` sits on the 自由民主党 organization observation; the prime-ministership stays in the `jp_prime_minister`
institution's `jp_pm` role. No `jp_pm` claim or source feeds the party role, and no party claim or source feeds `jp_pm`: every
claim and source id this packet adds begins `jp_ldp`, and none of `jp_pm`'s does. The Diet statements used here are those in
which a sitting party officer speaks of the party office (for Presidents who were also Prime Minister, only the party-office
wording is imported, and designations, cabinets and 首相 or 総理 wording are left to `jp_pm`). Two Presidents of this period,
河野洋平 and 谷垣禎一, were never Prime Minister and hold only the party office here. The same Diet issue of 10 April 2000 is used
by CLAUDE-C01-12 (speech 5, for `jp_pm`) and by this packet (speech 8, 森喜朗 on the party office): two different API responses,
recorded separately. The 2024 and 2025 holders, their sources and claims are unchanged.

### Date ledger

Each row is a separate dated fact with its own claim; two facts on one day stay two claims.

| Date | Events | Claims |
|---|---|---|
| 13 Dec 1989 | in office (pre-period reference) (海部俊樹) | `jp_ldp_diet_kaifu_refers_to_assuming_presidency_19891213` |
| 14 May 1990 | in office (dates the holder) (海部俊樹) | `jp_ldp_diet_kaifu_speaks_as_president_19900514` |
| 20 Feb 1992 | implicit reference to the office (宮澤喜一) | `jp_ldp_diet_miyazawa_president_standpoint_19920220` |
| 31 Mar 1992 | implicit reference to the office (宮澤喜一) | `jp_ldp_diet_miyazawa_president_standpoint_19920331` |
| 30 Nov 1992 | in office (dates the holder) (宮澤喜一) | `jp_ldp_diet_miyazawa_is_president_19921130` |
| 25 Nov 1994 | in office (dates the holder) (河野洋平) | `jp_ldp_diet_kono_is_president_19941125` |
| 31 Jan 1995 | in office (continuation) (河野洋平) | `jp_ldp_diet_kono_speaks_as_president_19950131` |
| 25 Sep 1995 | convention selection recalled (橋本龍太郎) | `jp_ldp_diet_hashimoto_chosen_at_convention_19950925` |
| 2 Oct 1995 | in office (dates the holder) (橋本龍太郎) | `jp_ldp_diet_hashimoto_speaks_as_president_19951002` |
| 3 Oct 1995 | in office (continuation) (橋本龍太郎) | `jp_ldp_diet_hashimoto_speaks_as_president_19951003` |
| 21 Jul 1998 | candidacies filed | `jp_ldp98idx_candidacies_filed_19980721` |
| 24 Jul 1998 | voting session listed; joint plenary meeting held; in office (continuation) (橋本龍太郎); result declared or announced (小渕恵三); predecessor called 前総裁 (橋本龍太郎); in office (dates the holder) (小渕恵三) | `jp_ldp98idx_vote_session_listed_19980724`, `jp_ldp98p1_joint_plenary_convened_19980724`, `jp_ldp98p1_hashimoto_greets_joint_plenary_19980724`, `jp_ldp98p3_result_declared_19980724`, `jp_ldp98p3_hashimoto_called_former_president_19980724`, `jp_ldp98p3_obuchi_new_president_after_decision_19980724` |
| 1 Jan 1999 | in office (continuation) (小渕恵三) | `jp_ldp_hatsugen_obuchi_signs_as_president_19990101` |
| 21 Sep 1999 | vote and count (小渕恵三); result declared or announced | `jp_ldp99idx_result_table_19990921`, `jp_ldp99p8_diet_member_vote_result_announced_19990921` |
| 22 Sep 1999 | extraordinary convention opened; result reported to the convention or joint plenary meeting; in office (dates the holder) (小渕恵三) | `jp_ldp99p9_congress_opened_19990922`, `jp_ldp99p9_result_reported_to_congress_19990922`, `jp_ldp99p9_obuchi_greeting_19990922`, `jp_ldp99aisatu_obuchi_greeting_as_president_19990922`, `jp_ldp99p10_obuchi_press_conference_after_election_19990922` |
| 10 Nov 1999 | in office (continuation) (小渕恵三) | `jp_ldp_diet_obuchi_decides_as_party_president_19991110` |
| 5 Apr 2000 | statement of assumption (start) (森喜朗); predecessor called 前総裁 (小渕恵三); selection by the joint plenary meeting (森喜朗) | `jp_ldpmori_stated_assumption_19th_president_20000405`, `jp_ldpmori_predecessor_called_former_president_20000405`, `jp_ldpmorisp_joint_plenary_elected_successor_20000405` |
| 10 Apr 2000 | in office (continuation) (森喜朗) | `jp_ldp_diet_mori_as_party_president_20000410` |
| 11 Apr 2001 | election notice (告示) | `jp_ldp01top_election_notice_20010411` |
| 24 Apr 2001 | vote and count (小泉純一郎); vote and count; in office (continuation) (森喜朗); in office (dates the holder) (小泉純一郎) | `jp_ldp01top_new_president_decided_koizumi_20010424`, `jp_ldp01d10_vote_closed_and_counted_20010424`, `jp_ldp01d10_mori_voting_as_president_20010424`, `jp_ldp01d10_koizumi_inaugural_greeting_20010424`, `jp_ldp01d11_first_press_conference_after_assuming_20010424` |
| 10 Aug 2001 | in office (dates the holder) (小泉純一郎) | `jp_ldpsainin_koizumi_reappointment_greeting_20010810` |
| 14 Sep 2001 | in office (continuation) (小泉純一郎) | `jp_ldp_diet_koizumi_as_party_president_20010914` |
| 8 Sep 2003 | election notice (告示); candidacies filed | `jp_ldp03_election_notice_20030908`, `jp_ldp03_candidacies_filed_20030908` |
| 20 Sep 2003 | vote and count (小泉純一郎); result reported to the convention or joint plenary meeting (小泉純一郎); in office (dates the holder) (小泉純一郎) | `jp_ldp03_koizumi_elected_399_20030920`, `jp_ldp03_winner_reported_approved_joint_plenary_20030920`, `jp_ldp03kaiken_press_conference_as_president_20030920` |
| 8 Sep 2006 | election notice (告示); candidacies filed; in office (continuation) (小泉純一郎) | `jp_ldp_2006_election_notice_20060908`, `jp_ldp_2006_candidacies_filed_20060908`, `jp_ldp_koizumi_incumbent_term_expiry_20060908` |
| 18 Sep 2006 | schedule listing the term expiry (prospective) | `jp_ldp_sousai06_schedule_term_expiry_20060930` |
| 20 Sep 2006 | vote and count (安倍晋三); in office (dates the holder) (安倍晋三) | `jp_ldp_abe_elected_21st_president_20060920`, `jp_ldp_abe_new_president_addresses_joint_plenary_20060920`, `jp_ldp_abe_first_press_conference_as_president_20060920`, `jp_ldp_joint_plenary_after_election_new_president_20060920` |
| 25 Sep 2006 | in office (continuation) (安倍晋三) | `jp_ldp_abe_president_decides_executive_20060925`, `jp_ldp_jiyuminshu_abe_21st_president_succeeds_koizumi_20060925` |
| 26 Sep 2006 | in office (continuation) (安倍晋三) | `jp_ldp_abe_president_addresses_joint_plenary_20060926` |
| 2 Oct 2006 | in office (continuation) (安倍晋三) | `jp_ldp_jiyuminshu_under_new_president_abe_20061002` |
| 12 Sep 2007 | resignation announced (安倍晋三); election method decided | `jp_ldp_abe_announces_resignation_20070912`, `jp_ldp_decides_joint_plenary_election_method_20070912` |
| 13 Sep 2007 | election schedule decided | `jp_ldp_2007_election_schedule_decided_20070913` |
| 15 Sep 2007 | candidacies filed | `jp_ldp_2007_fukuda_aso_file_candidacies_20070915` |
| 23 Sep 2007 | vote and count (福田康夫); in office (dates the holder) (福田康夫); election reported later (福田康夫) | `jp_ldp_fukuda_elected_22nd_president_joint_plenary_20070923`, `jp_ldp_fukuda_new_president_address_20070923`, `jp_ldp_fukuda_first_press_conference_as_president_20070923`, `jp_ldp_jiyuminshu_fukuda_22nd_president_20070923` |
| 24 Sep 2007 | in office (continuation) (福田康夫) | `jp_ldp_fukuda_president_nominates_officers_20070924` |
| 1 Sep 2008 | resignation announced (福田康夫) | `jp_ldp_fukuda_resignation_announcement_20080901` |
| 3 Sep 2008 | election schedule reported; in office (continuation) (福田康夫) | `jp_ldp_2008_election_schedule_reported_joint_plenary_20080903`, `jp_ldp_fukuda_last_request_as_president_20080903` |
| 10 Sep 2008 | election notice (告示); candidacies filed | `jp_ldp_2008_election_notice_20080910`, `jp_ldp_2008_candidacies_filed_20080910` |
| 22 Sep 2008 | vote and count (麻生太郎); result declared or announced (麻生太郎); in office (dates the holder) (麻生太郎) | `jp_ldp_aso_elected_23rd_president_joint_plenary_20080922`, `jp_ldp_aso_election_declared_by_committee_chair_20080922`, `jp_ldp_aso_new_president_address_20080922`, `jp_ldp_aso_first_press_conference_as_president_20080922`, `jp_ldp_aso_appoints_officers_after_taking_office_20080922` |
| 24 Sep 2008 | in office (continuation) (麻生太郎) | `jp_ldp_jiyuminshu_aso_23rd_president_20080924` |
| 8 Sep 2009 | election schedule approved; prospective resignation statement (麻生太郎); in office (continuation) (麻生太郎) | `jp_ldp_2009_election_schedule_approved_joint_plenary_20090908`, `jp_ldp_aso_states_will_resign_presidency_with_cabinet_20090908`, `jp_ldp_aso_president_addresses_joint_plenary_20090908` |
| 18 Sep 2009 | notice day referred to; election notice (告示); candidacies filed; index headline | `jp_ldp_2009_notice_day_proposals_to_candidates_20090918`, `jp_ldp_sousai09_election_notice_20090918`, `jp_ldp_sousai09_candidacies_filed_20090918`, `jp_ldp_index_2009_candidates_filed_20090918` |
| 21 Sep 2009 | schedule listing the term expiry (prospective) | `jp_ldp_sousai09_schedule_term_expiry_20090930` |
| 28 Sep 2009 | index headline (谷垣禎一); vote and count (谷垣禎一); result declared or announced (谷垣禎一); in office (dates the holder) (谷垣禎一); predecessor called 前総裁 (麻生太郎) | `jp_ldp_index_tanigaki_elected_24th_president_20090928`, `jp_ldp_index_tanigaki_president_first_press_conference_20090928`, `jp_ldp_sousai09_tanigaki_vote_count_20090928`, `jp_ldp_sousai09_result_declared_by_chair_20090928`, `jp_ldp_sousai09_tanigaki_address_joint_plenary_20090928`, `jp_ldp_sousai09_aso_called_former_president_20090928`, `jp_ldp_sousai09_tanigaki_first_press_conference_20090928` |
| 2 Oct 2009 | in office (continuation) (谷垣禎一) | `jp_ldp_tanigaki_president_visits_yamba_20091002` |
| (no structured date) | retrospective lists, histories and narratives; undated recollections and references; the 2001 election rules; the 2001 convention programme and address | `jp_ldp_list_kaifu_span_19890808_19911030`, `jp_ldp_list_kaifu_joint_plenary_election_19890808`, `jp_ldp_list_kaifu_result_19890808`, `jp_ldp_list_kaifu_sole_candidate_notice_19891006`, `jp_ldp_list_kaifu_convention_approval_19891031`, `jp_ldp_list_miyazawa_span_19911031_19930730`, `jp_ldp_list_miyazawa_public_election_19911027`, `jp_ldp_list_miyazawa_result_19911027`, `jp_ldp_list_miyazawa_convention_approval_19911029`, `jp_ldp_list_kono_span_19930730_19950930`, `jp_ldp_list_kono_joint_plenary_election_19930730`, `jp_ldp_list_kono_result_19930730`, `jp_ldp_list_kono_sole_candidate_notice_19930917`, `jp_ldp_list_kono_convention_decision_19930930`, `jp_ldp_list_hashimoto_span_19951001_19980724`, `jp_ldp_list_hashimoto_public_election_19950922`, `jp_ldp_list_hashimoto_result_19950922`, `jp_ldp_list_hashimoto_convention_report_19950925`, `jp_ldp_list_hashimoto_sole_candidate_notice_19970908`, `jp_ldp_list_hashimoto_joint_plenary_report_19970911`, `jp_ldp_list_obuchi_span_19980724_20000405`, `jp_ldp_list_obuchi_joint_plenary_decision_19980724`, `jp_ldp_list_obuchi_result_19980724`, `jp_ldp_list_obuchi_public_election_19990921`, `jp_ldp_list_obuchi_result_19990921`, `jp_ldp_list_obuchi_convention_report_19990922`, `jp_ldp_list_mori_span_20000405_20010424`, `jp_ldp_list_mori_joint_plenary_decision_20000405`, `jp_ldp_list_koizumi_span_20010424_20060930`, `jp_ldp_list_koizumi_joint_plenary_decision_20010424`, `jp_ldp_list_koizumi_result_20010424`, `jp_ldp_list_koizumi_reappointment_decided_20010810`, `jp_ldp_list_koizumi_public_election_20030920`, `jp_ldp_list_koizumi_result_20030920`, `jp_ldp_list_abe_span_20061001_20070923`, `jp_ldp_list_abe_public_election_20060920`, `jp_ldp_list_abe_result_20060920`, `jp_ldp_list_fukuda_span_20070923_20080922`, `jp_ldp_list_fukuda_joint_plenary_decision_20070923`, `jp_ldp_list_fukuda_result_20070923`, `jp_ldp_list_aso_span_20080922_20090930`, `jp_ldp_list_aso_joint_plenary_decision_20080922`, `jp_ldp_list_aso_result_20080922`, `jp_ldp_list_tanigaki_span_20091001_20120930`, `jp_ldp_list_tanigaki_public_election_20090928`, `jp_ldp_list_tanigaki_result_20090928`, `jp_ldp_kaifu_era_elected_joint_plenary_19890808`, `jp_ldp_kaifu_era_assumption_reference_19890808`, `jp_ldp_kaifu_era_reelection_notice_19891006`, `jp_ldp_kaifu_era_reappointment_decided_19891031`, `jp_ldp_kaifu_era_declines_candidacy_199110`, `jp_ldp_miyazawa_era_elected_19911027`, `jp_ldp_miyazawa_era_kono_assumed_19930730`, `jp_ldp_kono_era_miyazawa_resignation_referenced_1993`, `jp_ldp_kono_era_elected_19930730`, `jp_ldp_hashimoto_era_elected_19950922`, `jp_ldp_hashimoto_era_kono_withdrew_199508`, `jp_ldp_ayumi_chronology_kaifu_selected_19890808`, `jp_ldp_ayumi_chronology_kaifu_confirmed_19891031`, `jp_ldp_ayumi_chronology_miyazawa_elected_19911027`, `jp_ldp_ayumi_chronology_miyazawa_confirmed_19911029`, `jp_ldp_ayumi_chronology_kono_elected_19930730`, `jp_ldp_ayumi_chronology_57th_convention_19930930`, `jp_ldp_ayumi_chronology_hashimoto_elected_19950922`, `jp_ldp_ayumi_chronology_hashimoto_selected_convention_19950925`, `jp_ldphist_obuchi_elected_18th_president_19980724`, `jp_ldphist_obuchi_inherited_term_election_announced_19990909`, `jp_ldphist_obuchi_reelected_19990921`, `jp_ldphist_65th_extraordinary_congress_held_19990922`, `jp_ldphist_mori_chosen_19th_president_joint_plenary_20000405`, `jp_ldphist_mori_early_election_signalled_20010313`, `jp_ldphist_mori_statement_at_joint_plenary_20010411`, `jp_ldphist_koizumi_elected_20th_president_20010424`, `jp_ldphist_koizumi_reelected_unopposed_joint_plenary_20010810`, `jp_ldphist_koizumi_reelected_20030920`, `jp_ldp_diet_kaifu_recalls_campaign_as_president_stated_19900322`, `jp_ldp_diet_mori_miyazawa_chosen_15th_president_stated_19911111`, `jp_ldp_diet_miyazawa_states_elected_president_stated_19911112`, `jp_ldp_diet_miyazawa_states_elected_president_stated_19911113`, `jp_ldp_diet_miyazawa_recalls_election_oct_nov_1991_stated_19920220`, `jp_ldp_diet_kono_recalls_president_and_officers_resigned_stated_19941011`, `jp_ldp_diet_kono_recalls_own_selection_stated_19941011`, `jp_ldp_diet_kono_recalls_issuing_president_statement_stated_19941013`, `jp_ldpayumi01_obuchi_span_18th`, `jp_ldpayumi01_obuchi_elected_19980724`, `jp_ldpayumi01_obuchi_elected_19990921`, `jp_ldpayumi01_obuchi_result_reported_19990922`, `jp_ldpayumi01_mori_span_19th`, `jp_ldpayumi01_mori_selected_joint_plenary_20000405`, `jp_ldpayumi01_koizumi_open_span_20th`, `jp_ldpayumi01_koizumi_elected_20010424`, `jp_ldp_toutaikai67_programme_lists_president_address`, `jp_ldp_toutaikai67_mori_announces_early_election`, `jp_ldp01rules_joint_plenary_election_procedure`, `jp_ldp_diet_obuchi_states_won_party_presidency_19980821`, `jp_ldp_diet_koizumi_states_became_president_20010522`, `jp_ldp_diet_koizumi_states_reelected_20030930`, `jp_ldp_history_tanigaki_era_election_narrative` |

## Observations

### LDP-PRES-01 — 海部俊樹, the President when the period opens

Evidence: the party's list of its presidents shows 第14代 海部 俊樹 from 平成元年8月8日 to 平成3年10月30日
(`jp_ldp_list_kaifu_span_19890808_19911030`), chosen by the joint plenary meeting in place of a convention on 8 August 1989 with
279 votes (`jp_ldp_list_kaifu_joint_plenary_election_19890808`, `jp_ldp_list_kaifu_result_19890808`); its second entry records
the October 1989 selection with one candidate, notified on 6 October (`jp_ldp_list_kaifu_sole_candidate_notice_19891006`) and
approved by the 51st extraordinary convention on 31 October 1989 (`jp_ldp_list_kaifu_convention_approval_19891031`). The history
page for his presidency keeps apart the 8 August election (`jp_ldp_kaifu_era_elected_joint_plenary_19890808`), his becoming the
14th president (`jp_ldp_kaifu_era_assumption_reference_19890808`), the 6 October notice and election
(`jp_ldp_kaifu_era_reelection_notice_19891006`) and the 31 October report and reappointment
(`jp_ldp_kaifu_era_reappointment_decided_19891031`), and closes with his declining to stand in the October 1991 election on the
expiry of his term (`jp_ldp_kaifu_era_declines_candidacy_199110`); the party history's chronology lists the 8 August selection and
the 31 October confirmation (`jp_ldp_ayumi_chronology_kaifu_selected_19890808`, `jp_ldp_ayumi_chronology_kaifu_confirmed_19891031`).
In the Diet, 海部 referred on 13 December 1989 to the time before he assumed the presidency
(`jp_ldp_diet_kaifu_refers_to_assuming_presidency_19891213`, pre-period), recalled campaigning as party president in the February
1990 general election (`jp_ldp_diet_kaifu_recalls_campaign_as_president_stated_19900322`, undated) and on 14 May 1990 answered
'as LDP president' (私、自民党総裁としても; `jp_ldp_diet_kaifu_speaks_as_president_19900514`).

Decision: accepted in part. The holder is dated 14 May 1990, the first dated statement of the office found after 1 January 1990,
with no start or end.

Limits: nothing attests 1 January 1990 itself; every party record of 1989-1991 reviewed is retrospective (no contemporaneous
announcement of 1989-1995 was found online); no source states the day his presidency ended, and the withdrawal from the 1991
election is not an end.

### LDP-PRES-02 — October 1991: 宮澤喜一

Evidence: the party's list records the public election of 27 October 1991 (宮沢 喜一 285, 渡辺 美智雄 120, 三塚 博 87), approval
by the 54th extraordinary convention on 29 October and the period from 31 October 1991 to 30 July 1993 (four claims beginning
`jp_ldp_list_miyazawa_`); the history page dates the election in the eighth-floor hall of party headquarters to 27 October
(`jp_ldp_miyazawa_era_elected_19911027`) and the chronology lists the election and the 29 October confirmation
(`jp_ldp_ayumi_chronology_miyazawa_elected_19911027`, `jp_ldp_ayumi_chronology_miyazawa_confirmed_19911029`). In the Diet, 森喜朗,
whom the same sitting's reply addresses as 森政調会長, said on 11 November 1991 that 宮澤総理 had been chosen as the 15th president
(`jp_ldp_diet_mori_miyazawa_chosen_15th_president_stated_19911111`); 宮澤 said on 12 and 13 November 1991 that he had been elected
in an open election (`jp_ldp_diet_miyazawa_states_elected_president_stated_19911112`,
`jp_ldp_diet_miyazawa_states_elected_president_stated_19911113`) and on 20 February 1992 referred to the October-November 1991
election (`jp_ldp_diet_miyazawa_recalls_election_oct_nov_1991_stated_19920220`), all without the day. On 20 February and 31 March
1992 he spoke implicitly of his standpoint as president (`jp_ldp_diet_miyazawa_president_standpoint_19920220`,
`jp_ldp_diet_miyazawa_president_standpoint_19920331`), and on 30 November 1992 he said 'I am the president of the Liberal
Democratic Party' (私は自由民主党の総裁でございますから; `jp_ldp_diet_miyazawa_is_president_19921130`).

Decision: accepted in part. The holder is dated 30 November 1992, his only explicit statement of the office found (check defect
A8); the implicit references of February and March 1992 are claims.

Limits: the day he took office is stated only retrospectively (31 October 1991 in the list); no source states the day it ended:
the 河野 history page and 河野's own 1994 statement refer to his 1993 resignation without a day.

### LDP-PRES-03 — 1993: 河野洋平, a President who was never Prime Minister

Evidence: the party's list records the joint plenary election of 30 July 1993 (河野 洋平 208, 渡辺 美智雄 159), a second entry
with one candidate notified on 17 September 1993 and decided by the 57th extraordinary convention on 30 September 1993, and the
period from 30 July 1993 to 30 September 1995 (five claims beginning `jp_ldp_list_kono_`). The history page for his presidency
refers to the resignation of 宮沢喜一総裁, without a day, as the occasion of the 30 July election
(`jp_ldp_kono_era_miyazawa_resignation_referenced_1993`, `jp_ldp_kono_era_elected_19930730`); the 宮沢 page says 河野洋平氏 took
office on 30 July (`jp_ldp_miyazawa_era_kono_assumed_19930730`, retrospective, never a from); the chronology lists the 30 July
election and the 57th convention of 30 September, which it links to nobody (`jp_ldp_ayumi_chronology_kono_elected_19930730`,
`jp_ldp_ayumi_chronology_57th_convention_19930930`). In the Diet on 11 October 1994 河野 recalled that after the 1993 general
election the president and the three senior officers resigned together and that he was then chosen and took the president's
seat (`jp_ldp_diet_kono_recalls_president_and_officers_resigned_stated_19941011`,
`jp_ldp_diet_kono_recalls_own_selection_stated_19941011`); on 13 October 1994 he confirmed that he had issued the president's
statement (総裁声明) that the questioner dated 14 December 1993 (`jp_ldp_diet_kono_recalls_issuing_president_statement_stated_19941013`,
a record the part A check found); on 25 November 1994 he spoke 'although LDP president' (自由民主党の総裁とはいえ;
`jp_ldp_diet_kono_is_president_19941125`) and on 31 January 1995 'as president of the Liberal Democratic Party'
(`jp_ldp_diet_kono_speaks_as_president_19950131`, continuation). The 橋本 history page says 河野洋平前総裁 had withdrawn at the end
of August 1995 (`jp_ldp_hashimoto_era_kono_withdrew_199508`).

Decision: accepted in part. The holder is dated 25 November 1994, in the term from the second selection, with no start or end.

Limits: no in-office statement dated between 30 July and 30 September 1993 was found; the party's records of 1993-1995 are
retrospective; his statements of 19 October 1995 as foreign minister, after he had left the party office, are leads (check A1).

### LDP-PRES-04 — 1995: 橋本龍太郎

Evidence: the party's list records the public election of 22 September 1995 (橋本 龍太郎 304, 小泉 純一郎 87), a report to the 60th
extraordinary convention on 25 September and the period from 1 October 1995 to 24 July 1998; its second entry records the 1997
selection with one candidate, notified on 8 September 1997 and reported to the joint plenary meeting on 11 September 1997
(claims only; six claims beginning `jp_ldp_list_hashimoto_`). The history page dates the vote to 22 September
(`jp_ldp_hashimoto_era_elected_19950922`) and the chronology lists the election and the convention's 選任 of 25 September
(`jp_ldp_ayumi_chronology_hashimoto_elected_19950922`, `jp_ldp_ayumi_chronology_hashimoto_selected_convention_19950925`). In the Diet
橋本 said on 2 October 1995 that he would do his utmost as president of the Liberal Democratic Party
(`jp_ldp_diet_hashimoto_speaks_as_president_19951002`), again on 3 October (`jp_ldp_diet_hashimoto_speaks_as_president_19951003`,
continuation), and on 26 October 1995 that the party convention chose him on 25 September
(`jp_ldp_diet_hashimoto_chosen_at_convention_19950925`, a selection dated by the day it states, never a start). On 24 July 1998 the
party's captions show 橋本総裁 greeting the joint plenary meeting before the vote
(`jp_ldp98p1_hashimoto_greets_joint_plenary_19980724`) and, after the decision, 橋本前総裁
(`jp_ldp98p3_hashimoto_called_former_president_19980724`).

Decision: accepted in part. The holder is dated 2 October 1995, with no start or end.

Limits: no source states the day he took office (the list's 1 October is retrospective; the convention's 25 September is a
selection); the term that followed the 1997 re-selection is not reviewed and has no holder observation; the 1998 前総裁 caption
states no end.

### LDP-PRES-05 — 1998 and 1999: 小渕恵三

Evidence, 1998: the party's 平成１０年総裁選挙情報 index names the candidates, the filing of 21 July and the voting and count of 24
July, and supplies the year that the caption pages do not print (`jp_ldp98idx_candidacies_filed_19980721`,
`jp_ldp98idx_vote_session_listed_19980724`, a record the part B check asked to import). The caption pages show the venue and the
joint plenary meeting (`jp_ldp98p1_joint_plenary_convened_19980724`, no holder name), the count from 14:52 and the result announced
at 15:13 (梶山静六 102, 小泉純一郎 84, 小渕恵三 225; `jp_ldp98p3_result_declared_19980724`) and, after the decision, 小渕新総裁 at
15:15, 小渕新総裁挨拶 at 15:24 and the new president's press conference at 16:33 (`jp_ldp98p3_obuchi_new_president_after_decision_19980724`).
His New Year message of 1 January 1999 is signed 自由民主党 総裁 小渕恵三 (`jp_ldp_hatsugen_obuchi_signs_as_president_19990101`,
continuation, a record the check found); on 21 August 1998 he told the Diet he had won the presidency (undated recollection,
`jp_ldp_diet_obuchi_states_won_party_presidency_19980821`). Evidence, 1999: the party history says the election was notified on 9
September because he had inherited his predecessor's term (retrospective); the party's 1999 page gives the count (350 = 253 Diet
+ 97 member-derived) with the counting sessions of 21 September (`jp_ldp99idx_result_table_19990921`); a caption page shows 谷川
announcing the result of the Diet members' vote at 17:00 on 21 September (`jp_ldp99p8_diet_member_vote_result_announced_19990921`,
no holder name); on 22 September the convention opened at 13:01 (`jp_ldp99p9_congress_opened_19990922`), 谷川 reported the result at
13:10 (`jp_ldp99p9_result_reported_to_congress_19990922`, no holder name) and 小渕総裁 gave his greeting at 13:18
(`jp_ldp99p9_obuchi_greeting_19990922`); the party's text of that greeting, dated 平成１１年９月２２日, says the party rules give him
a two-year term (`jp_ldp99aisatu_obuchi_greeting_as_president_19990922`), and a caption page shows his first press conference
after the election at 14:30 (`jp_ldp99p10_obuchi_press_conference_after_election_19990922`); both records were found by the part B
check. On 10 November 1999 he told the Diet he had that day made a decision as LDP president
(`jp_ldp_diet_obuchi_decides_as_party_president_19991110`, continuation). The party's list, its 2001 election list and its printed
history repeat the 1998 and 1999 elections and the report to the 65th convention (retrospective).

Decision: accepted in part. Two holders: 24 July 1998 and 22 September 1999, with no start or end.

Limits: no source states the day either term took effect; no source states the day his presidency ended (the lists' 5 April
2000 and 森's 前総裁 reference of that day are claims). The 1998 caption pages are the party's 2001 web copies of the 1998 record
(Last-Modified 28 February 2001), which print month and day only (check B6).

### LDP-PRES-06 — 2000: 森喜朗, chosen after 小渕恵三's incapacity

Evidence: the party's record of 森総裁's greeting, headed 平成１２年４月５日 午前１１時〜 両院議員総会, has him say that just now
(ただ今), having been recommended by the members of both Houses, he has come to take office as the 19th president
(`jp_ldpmori_stated_assumption_19th_president_20000405`), and refers to 小渕前総裁 (`jp_ldpmori_predecessor_called_former_president_20000405`);
a party feature page says the joint plenary meeting on the morning of the 5th elected him the successor president
(`jp_ldpmorisp_joint_plenary_elected_successor_20000405`); the list, the 2001 list and the history repeat the selection
(retrospective, the history adding 'unanimously'). On 10 April 2000 he told the Diet that as LDP president too he would carry out
party reform (`jp_ldp_diet_mori_as_party_president_20000410`, continuation). The programme of the 67th party convention sets it for
平成13年3月13日 and lists 総裁挨拶 by 森喜朗総裁 (`jp_ldp_toutaikai67_programme_lists_president_address`), and his address, given as
party president, names bringing forward the presidential election planned for the autumn
(`jp_ldp_toutaikai67_mori_announces_early_election`; both records found by the part B check, undated because the address page prints
no date); the history dates that announcement to 13 March 2001 and his statement about stepping down to 11 April 2001
(retrospective). On 24 April 2001 the party captions 森総裁 voting at 13:23 (`jp_ldp01d10_mori_voting_as_president_20010424`,
continuation).

Decision: accepted in part: from 5 April 2000, the packet's only stated start, with no end. The reviewer may read
就任致すことになりました as prospective; the day would be the same (check B8).

Limits: no source states the day his presidency ended; the early-election announcement, the statement of 11 April 2001 and the
lists' 24 April 2001 end are claims.

### LDP-PRES-07 — 2001-2003: 小泉純一郎 and his re-elections

Evidence, April 2001: the party's 2001 election page lists in its timetable (平成１３年４月１２日現在) the notice of 11 April
(`jp_ldp01top_election_notice_20010411`) and gives the result 小泉純一郎 298, 橋本龍太郎 155, 麻生太郎 31
(`jp_ldp01top_new_president_decided_koizumi_20010424`); the rules for the election at the joint plenary meeting are procedure only
(`jp_ldp01rules_joint_plenary_election_procedure`); a caption page shows the meeting opened at 13:01, the voting and the count
result at 13:57 (`jp_ldp01d10_vote_closed_and_counted_20010424`, no holder name) and at 14:00 就任の挨拶をする小泉新総裁
(`jp_ldp01d10_koizumi_inaugural_greeting_20010424`); another shows his first press conference after taking office at 17:30
(`jp_ldp01d11_first_press_conference_after_assuming_20010424`). He told the Diet on 22 May 2001 that party members and Diet members
chose him (undated recollection). August 2001: the party's record of 小泉総裁再任挨拶, dated 10 August 2001, has him thank the
members for their unanimous recommendation and say the officers would basically stay
(`jp_ldpsainin_koizumi_reappointment_greeting_20010810`); the list and the history record the joint plenary meeting's reappointment
decision without a vote (retrospective); on 14 September 2001 小泉内閣総理大臣 told the Diet he was trying to act as president
(`jp_ldp_diet_koizumi_as_party_president_20010914`, continuation). 2003: the election was notified at 11:00 on 8 September 2003 by
谷川 and four candidates filed (`jp_ldp03_election_notice_20030908`, `jp_ldp03_candidacies_filed_20030908`, found by the part B
check); on 20 September he won 399 (194 + 205) (`jp_ldp03_koizumi_elected_399_20030920`), the result was reported to and approved by
the joint plenary meeting (`jp_ldp03_winner_reported_approved_joint_plenary_20030920`), and the party's transcript headed
総裁就任記者会見 records 小泉純一郎総裁 at 18:00 (`jp_ldp03kaiken_press_conference_as_president_20030920`); he told the Diet on 30
September 2003 that he had been re-elected (undated recollection). The 2006 notice describes the election as held on the expiry of
the term of 小泉純一郎総裁 (`jp_ldp_koizumi_incumbent_term_expiry_20060908`, continuation).

Decision: accepted in part. Three holders: 24 April 2001, 10 August 2001 and 20 September 2003, with no start or end.

Limits: the 就任 captions and heading name the occasion but do not state the day office took effect; the list's end of 30
September 2006 and the 2006 schedule's term expiry are claims; no source states the day his presidency ended.

### LDP-PRES-08 — 2006 and 2007: 安倍晋三 and 福田康夫

Evidence, 2006: the election was notified at 11:00 on 8 September 2006 by 臼井日出男 (`jp_ldp_2006_election_notice_20060908`) and
安倍晋三, 谷垣禎一 and 麻生太郎 filed (`jp_ldp_2006_candidacies_filed_20060908`); the party's schedule as of 18 September lists the
vote and count on 20 September and 30 September as the day the president's term expires
(`jp_ldp_sousai06_schedule_term_expiry_20060930`, found by the part C check). On 20 September 安倍晋三 won 464 (267 + 197)
(`jp_ldp_abe_elected_21st_president_20060920`), addressed the joint plenary meeting as 安倍新総裁
(`jp_ldp_abe_new_president_addresses_joint_plenary_20060920`), held his first press conference since taking office
(`jp_ldp_abe_first_press_conference_as_president_20060920`), and the meeting was held before the newly chosen 安倍晋三総裁
(`jp_ldp_joint_plenary_after_election_new_president_20060920`). On 25 September 安倍晋三総裁 decided the executive and the party
organ's notice called him the 21st president succeeding 小泉純一郎総裁; he addressed the joint plenary meeting as 総裁 on 26
September; the notice of 2 October spoke of 安倍新総裁 (four continuation claims). The list gives the span 1 October 2006 to 23
September 2007. Evidence, 2007: on 12 September 2007 安倍晋三総理・総裁 announced his resignation
(`jp_ldp_abe_announces_resignation_20070912`) and the party chose, under 党則第６条第２項 (procedure), an election at the joint
plenary meeting (`jp_ldp_decides_joint_plenary_election_method_20070912`); the schedule was decided on 13 September and 福田康夫 and
麻生太郎 filed on 15 September. On 23 September the joint plenary meeting elected 福田康夫 330 to 197
(`jp_ldp_fukuda_elected_22nd_president_joint_plenary_20070923`); 福田新総裁 addressed it (`jp_ldp_fukuda_new_president_address_20070923`)
and held his first press conference since taking office (`jp_ldp_fukuda_first_press_conference_as_president_20070923`); he
nominated the party officers on 24 September (continuation); the party organ's notice of 25 September dates his election to the
23rd (`jp_ldp_jiyuminshu_fukuda_22nd_president_20070923`).

Decision: accepted in part: 安倍晋三 observed 20 September 2006 and 福田康夫 observed 23 September 2007, with no start or end.

Limits: the party's own records conflict on when the 2006 term took effect (新総裁 and 就任 on 20 September; 30 September as the
previous term's expiry; 1 October in the later list); none is a from, and the conflict is left unresolved. The announced
resignation of 12 September 2007 is not an end; no source states the day either presidency ended.

### LDP-PRES-09 — 2008: 麻生太郎

Evidence: the party's report of 3 September 2008 says 福田康夫総理・総裁 announced his resignation on the 1st
(`jp_ldp_fukuda_resignation_announcement_20080901`) and that the joint plenary meeting was told the schedule
(`jp_ldp_2008_election_schedule_reported_joint_plenary_20080903`); 福田 said he had announced his intention to resign (辞意表明) and
made his 'last request as president' (`jp_ldp_fukuda_last_request_as_president_20080903`, continuation). The election was notified at
11:00 on 10 September by 臼井日出男 (`jp_ldp_2008_election_notice_20080910`) and five candidates filed
(`jp_ldp_2008_candidacies_filed_20080910`). On 22 September the joint plenary meeting elected 麻生太郎 with 351 votes
(`jp_ldp_aso_elected_23rd_president_joint_plenary_20080922`), 臼井 announced his election
(`jp_ldp_aso_election_declared_by_committee_chair_20080922`), and 麻生新総裁 said he had just now been chosen
(`jp_ldp_aso_new_president_address_20080922`), held his first press conference since taking office
(`jp_ldp_aso_first_press_conference_as_president_20080922`) and appointed the party officers immediately after taking office
(`jp_ldp_aso_appoints_officers_after_taking_office_20080922`). The party organ's notice of 24 September speaks of 麻生新総裁
(continuation); the list gives the span 22 September 2008 to 30 September 2009.

Decision: accepted in part: observed 22 September 2008, with no start or end.

Limits: 就任後初めて and 就任後直ちに place the assumption before those events but do not state its day; no source states the day
福田康夫's or 麻生太郎's presidency ended.

### LDP-PRES-10 — 2009: 谷垣禎一, a President who was never Prime Minister

Evidence: the joint plenary meeting of 8 September 2009 approved the schedule (notice 18 September, vote 28 September, with 300
party-member votes under the main rule; `jp_ldp_2009_election_schedule_approved_joint_plenary_20090908`), and 麻生太郎総裁 said he
would also resign as party president with the cabinet's resignation on the morning of the 16th
(`jp_ldp_aso_states_will_resign_presidency_with_cabinet_20090908`, prospective) and addressed it as president (continuation). On 18
September 野田毅 declared the notice at 9:30 and 河野太郎, 谷垣禎一 and 西村康稔 filed (`jp_ldp_sousai09_election_notice_20090918`,
`jp_ldp_sousai09_candidacies_filed_20090918`, found by the part C check); the proposals handed to the candidates that day and the
index headline are supplementary. The schedule as of 21 September lists the vote on 28 September and 30 September as the day the
president's term expires (`jp_ldp_sousai09_schedule_term_expiry_20090930`). On 28 September the count gave 谷垣禎一 300 (120 + 180)
(`jp_ldp_sousai09_tanigaki_vote_count_20090928`), 野田毅 declared him elected (`jp_ldp_sousai09_result_declared_by_chair_20090928`),
谷垣新総裁 addressed the joint plenary meeting that followed (`jp_ldp_sousai09_tanigaki_address_joint_plenary_20090928`), 麻生太郎前総裁
went up on the platform (`jp_ldp_sousai09_aso_called_former_president_20090928`), and 谷垣禎一新総裁, having taken office as the 24th
president, held his first press conference as president that evening (`jp_ldp_sousai09_tanigaki_first_press_conference_20090928`);
these two articles, archived on the party's 2009 election site, were found by the part C check and replace the index headlines as
evidence. On 2 October 2009 谷垣禎一総裁 visited the 八ツ場ダム site (continuation). The list gives the span 1 October 2009 to 30
September 2012, and the era page narrates the election (retrospective).

Decision: accepted in part: observed 28 September 2009, with no start or end.

Limits: the term start conflicts (就任 on 28 September; 30 September as the previous term's expiry; 1 October in the list); 麻生太郎's
end stays unresolved (his prospective 16 September; 前総裁 on 28 September; the schedule's 30 September; the list's 30 September),
and no party record of his actually resigning on 16 September 2009 was found.

## Sources added

| Source ID | What | Provenance |
|---|---|---|
| `jp_ldp_history_presidents_list` | 自民党の歴史 / 自民党について / 自由民主党 (歴代総裁 list) | capture 2026-05-09 of www.jimin.jp |
| `jp_ldp_history_kaifu_era` | 海部俊樹総裁時代 / 歴代総裁 / 党のあゆみ / 自民党について / 自由民主党 | capture 2026-02-08 of www.jimin.jp |
| `jp_ldp_history_miyazawa_era` | 宮沢喜一総裁時代 / 歴代総裁 / 党のあゆみ / 自民党について / 自由民主党 | capture 2026-02-08 of www.jimin.jp |
| `jp_ldp_history_kono_era` | 河野洋平総裁時代 / 歴代総裁 / 党のあゆみ / 自民党について / 自由民主党 | capture 2026-02-08 of www.jimin.jp |
| `jp_ldp_history_hashimoto_era` | 橋本龍太郎総裁時代 / 歴代総裁 / 党のあゆみ / 自民党について / 自由民主党 | capture 2026-02-08 of www.jimin.jp |
| `jp_ldp_ayumi_2015` | 自由民主党のあゆみ (party history PDF; まえがき dated 平成二十七年十一月) | capture 2026-02-08 of storage2.jimin.jp |
| `jp_ldp_diet_hc_kessan_19891213` | 第116回国会 参議院 決算委員会 第8号 (1989-12-13) | Diet minutes API, whole meeting record |
| `jp_ldp_diet_hr_yosan_19900322` | 第118回国会 衆議院 予算委員会 第3号 (1990-03-22) | Diet minutes API, whole meeting record |
| `jp_ldp_diet_hc_yosan_19900514` | 第118回国会 参議院 予算委員会 第6号 (1990-05-14) | Diet minutes API, whole meeting record |
| `jp_ldp_diet_hr_honkaigi_19911111` | 第122回国会 衆議院 本会議 第3号 (1991-11-11) | Diet minutes API, whole meeting record |
| `jp_ldp_diet_hc_honkaigi_19911112` | 第122回国会 参議院 本会議 第3号 (1991-11-12) | Diet minutes API, whole meeting record |
| `jp_ldp_diet_hc_honkaigi_19911113` | 第122回国会 参議院 本会議 第4号 (1991-11-13) | Diet minutes API, whole meeting record |
| `jp_ldp_diet_hr_yosan_19920220` | 第123回国会 衆議院 予算委員会 第5号 (1992-02-20) | Diet minutes API, whole meeting record |
| `jp_ldp_diet_hc_yosan_19920331` | 第123回国会 参議院 予算委員会 第10号 (1992-03-31) | Diet minutes API, whole meeting record |
| `jp_ldp_diet_hr_koshokusen_19921130` | 第125回国会 衆議院 公職選挙法改正に関する調査特別委員会 第2号 (1992-11-30) | Diet minutes API, whole meeting record |
| `jp_ldp_diet_hr_yosan_19941011` | 第131回国会 衆議院 予算委員会 第1号 (1994-10-11) | Diet minutes API, whole meeting record |
| `jp_ldp_diet_hr_yosan_19941013` | 第131回国会 衆議院 予算委員会 第3号 (1994-10-13) | Diet minutes API, whole meeting record |
| `jp_ldp_diet_hr_wto_19941125` | 第131回国会 衆議院 世界貿易機関設立協定等に関する特別委員会 第7号 (1994-11-25) | Diet minutes API, whole meeting record |
| `jp_ldp_diet_hr_yosan_19950131` | 第132回国会 衆議院 予算委員会 第5号 (1995-01-31) | Diet minutes API, whole meeting record |
| `jp_ldp_diet_hr_honkaigi_19951002` | 第134回国会 衆議院 本会議 第2号 (1995-10-02) | Diet minutes API, whole meeting record |
| `jp_ldp_diet_hc_honkaigi_19951003` | 第134回国会 参議院 本会議 第2号 (1995-10-03) | Diet minutes API, whole meeting record |
| `jp_ldp_diet_hr_yosan_19951026` | 第134回国会 衆議院 予算委員会 第5号 (1995-10-26) | Diet minutes API, whole meeting record |
| `jp_ldp_sousai_ayumi_20011109` | 自由民主党総裁選挙のあゆみ (自由民主党情報) | capture 2001-11-09 of www.jimin.or.jp |
| `jp_ldp_sousai98_index` | 平成１０年総裁選挙情報 (第１８代自由民主党総裁選挙情報) | capture 2001-03-09 of www.jimin.or.jp |
| `jp_ldp_sousai98_vote_page1_19980724` | 第18代自由民主党総裁選挙情報: 総裁選挙投票・開票の模様（7月24日午後2時〜）［1/3］ | capture 2001-07-01 of www.jimin.or.jp |
| `jp_ldp_sousai98_vote_page3_19980724` | 第18代自由民主党総裁選挙情報: 総裁選挙投票・開票の模様（7月24日午後2時〜）［3/3］ | capture 2001-07-19 of www.jimin.or.jp |
| `jp_ldp_hatsugen_obuchi_new_year_19990101` | 党役員発言: 明けましておめでとうございます (自由民主党総裁 小渕恵三, 平成１１年１月１日) | capture 2001-07-01 of www.jimin.or.jp |
| `jp_ldp_sousai99_index` | 平成11年総裁選挙情報 (自由民主党総裁選挙情報) | capture 2000-06-05 of www.jimin.or.jp |
| `jp_ldp_sousai99_diet_member_vote_19990921` | 自由民主党総裁選挙情報: 議員投票の開票の模様（9月21日午後4時〜） | capture 2000-01-18 of www.jimin.or.jp |
| `jp_ldp_sousai99_extraordinary_congress_19990922` | 自由民主党総裁選挙情報: 臨時党大会の模様（9月22日午後1時〜） | capture 2000-01-18 of www.jimin.or.jp |
| `jp_ldp_sousai99_obuchi_greeting_19990922` | 臨時党大会における小渕総裁挨拶（要旨）（平成１１年９月２２日） | capture 2000-01-18 of www.jimin.or.jp |
| `jp_ldp_sousai99_press_conference_19990922` | 自由民主党総裁選挙情報: 新総裁記者会見の模様（9月22日午後2時30分〜） | capture 2000-01-18 of www.jimin.or.jp |
| `jp_ldp_mori_greeting_joint_plenary_20000405` | 森総裁挨拶（平成12年4月5日 午前11時〜 両院議員総会） | capture 2000-12-13 of www.jimin.or.jp |
| `jp_ldp_mori_special_20000414` | 日本新生 森新内閣スタート (自由民主 web edition, /jimin/120413/morispe/) | capture 2000-05-10 of www.jimin.or.jp |
| `jp_ldp_toutaikai67_programme_20010313` | 第67回 自由民主党大会 進行プログラム | capture 2001-04-30 of www.jimin.or.jp |
| `jp_ldp_toutaikai67_mori_address_20010313` | 第67回自由民主党大会 総裁挨拶 森喜朗総裁 | capture 2002-09-08 of www.jimin.jp |
| `jp_ldp_sousai01_top_20010428` | 総裁選情報: 新総裁決定 小泉純一郎 (2001 presidential election page) | capture 2001-04-28 of www.jimin.or.jp |
| `jp_ldp_sousai01_rules_2001` | 党大会に代わる両院議員総会における総裁選挙の実施要領 (2001) | capture 2001-04-16 of www.jimin.or.jp |
| `jp_ldp_sousai01_joint_plenary_20010424` | 総裁選情報: 党大会に代わる両院議員総会（投開票） | capture 2001-06-11 of www.jimin.or.jp |
| `jp_ldp_sousai01_press_conference_20010424` | 総裁選情報: 新総裁記者会見（4月24日午後5時30分〜） | capture 2001-04-28 of www.jimin.or.jp |
| `jp_ldp_koizumi_reappointment_greeting_20010810` | 小泉総裁再任挨拶（平成13年8月10日）(党役員発言) | capture 2002-02-21 of www.jimin.jp |
| `jp_ldp_sousai03_notice_20030908` | 平成15年 自由民主党 総裁選挙: 総裁選挙が告示。小泉、藤井、亀井、高村の４候補が立候補【平成15年9月8日】 | capture 2003-12-16 of www.jimin.jp |
| `jp_ldp_sousai03_result_20030920` | 平成15年 自由民主党 総裁選挙: 小泉純一郎候補が３９９票獲得し当選。【平成15年9月20日】 | capture 2003-09-22 of www.jimin.jp |
| `jp_ldp_sousai03_press_conference_20030920` | 平成15年 自由民主党 総裁選挙: 総裁就任記者会見【平成15年9月20日】18:00〜 | capture 2003-09-22 of www.jimin.jp |
| `jp_ldp_diet_sangiin_budget_obuchi_19980821` | 参議院 予算委員会 第143回国会 第3号 (1998-08-21), speech 160 | Diet minutes API, one speech |
| `jp_ldp_diet_joint_budget_obuchi_19991110` | 両院 予算委員会合同審査会 第146回国会 第1号 (1999-11-10), speech 27 | Diet minutes API, one speech |
| `jp_ldp_diet_shugiin_plenary_mori_20000410` | 衆議院 本会議 第147回国会 第22号 (2000-04-10), speech 8 | Diet minutes API, one speech |
| `jp_ldp_diet_sangiin_budget_koizumi_20010522` | 参議院 予算委員会 第151回国会 第15号 (2001-05-22), speech 335 | Diet minutes API, one speech |
| `jp_ldp_diet_shugiin_budget_koizumi_20010914` | 衆議院 予算委員会 第152回国会 第2号 (2001-09-14), speech 75 | Diet minutes API, one speech |
| `jp_ldp_diet_sangiin_plenary_koizumi_20030930` | 参議院 本会議 第157回国会 第2号 (2003-09-30), speech 3 | Diet minutes API, one speech |
| `jp_ldp_news_2006_election_notice_20060908` | 総裁選挙が告示　安倍、谷垣、麻生の３氏が立候補 | capture 2009-01-05 of www.jimin.jp |
| `jp_ldp_sousai06_schedule_20060918` | 自由民主党 総裁選２００６ スケジュール（平成１８年９月１８日現在） | capture 2007-02-28 of www.jimin.jp |
| `jp_ldp_news_abe_elected_20060920` | 安倍晋三氏が新総裁に選出 | capture 2009-01-05 of www.jimin.jp |
| `jp_ldp_news_abe_first_press_conference_20060920` | 「期待、支援に応え、責任果たす」　安倍新総裁が初の記者会見 | capture 2009-01-05 of www.jimin.jp |
| `jp_ldp_news_joint_plenary_20060920` | 衆議院補欠選挙候補者が必勝をアピール！　党大会に代わる両院議員総会 | capture 2009-01-05 of www.jimin.jp |
| `jp_ldp_news_abe_executive_20060925` | 新執行部が決定――幹事長に中川秀直、総務会長に丹羽雄哉、政調会長は中川昭一の各氏が就任 | capture 2009-01-05 of www.jimin.jp |
| `jp_ldp_jiyuminshu_2247_notice_20060925` | 機関紙「自由民主」２２４７号（平成１８年１０月３日号）発行のおしらせ | capture 2009-01-05 of www.jimin.jp |
| `jp_ldp_news_abe_president_joint_plenary_20060926` | 臨時国会が召集　安倍総裁を第９０代総理に選出 | capture 2009-01-05 of www.jimin.jp |
| `jp_ldp_jiyuminshu_2248_notice_20061002` | 機関紙「自由民主」２２４８号（平成１８年１０月１０日号）発行のおしらせ | capture 2007-08-16 of www.jimin.jp |
| `jp_ldp_news_abe_resignation_election_method_20070912` | 総裁選の日程を協議　総裁選管理委員会 | capture 2009-07-04 of www.jimin.jp |
| `jp_ldp_news_2007_election_schedule_20070913` | 総裁選の日程決まる　１４日告示、２３日投開票 | capture 2009-07-04 of www.jimin.jp |
| `jp_ldp_news_2007_candidates_filed_20070915` | 総裁選スタート　候補者届出と共同記者会見 | capture 2009-07-04 of www.jimin.jp |
| `jp_ldp_news_fukuda_elected_20070923` | 第２２代総裁に福田康夫元官房長官を選出　両院議員総会 | capture 2008-02-07 of www.jimin.jp |
| `jp_ldp_news_fukuda_first_press_conference_20070923` | 「国民の信頼回復に全力を尽くしたい」　福田新総裁が就任後初の記者会見 | capture 2009-07-04 of www.jimin.jp |
| `jp_ldp_news_fukuda_party_officers_20070924` | 選挙対策委員長に格上げ、党４役体制がスタート | capture 2009-04-25 of www.jimin.jp |
| `jp_ldp_jiyuminshu_2293_notice_20070925` | 機関紙「自由民主」２２９３号（平成１９年１０月２日号）発行のおしらせ | capture 2009-09-10 of www.jimin.jp |
| `jp_ldp_news_joint_plenary_schedule_20080903` | 総裁選「今月１０日告示、２２日投開票」を報告　両院議員総会 | capture 2008-09-09 of www.jimin.jp |
| `jp_ldp_news_2008_election_notice_20080910` | 総裁選挙告示、５候補が立候補 | capture 2008-09-17 of www.jimin.jp |
| `jp_ldp_news_aso_elected_20080922` | 第２３代総裁に麻生太郎幹事長を選出　両院議員総会 | capture 2008-09-24 of www.jimin.jp |
| `jp_ldp_news_aso_first_press_conference_20080922` | 国民の不安解消に全力を尽くす―麻生新総裁が就任後初の記者会見 | capture 2008-09-24 of www.jimin.jp |
| `jp_ldp_news_aso_party_officers_20080922` | 新党４役決定　幹事長に細田博之氏が就任 | capture 2008-09-24 of www.jimin.jp |
| `jp_ldp_jiyuminshu_2339_notice_20080924` | 機関紙「自由民主」２３３９号（平成２０年９月３０日号）発行のおしらせ | capture 2008-09-27 of www.jimin.jp |
| `jp_ldp_news_joint_plenary_schedule_20090908` | 総裁選「今月１８日告示、２８日投開票」を了承　両院議員総会 | capture 2009-09-12 of www.jimin.jp |
| `jp_ldp_news_2009_notice_day_20090918` | 細田博之幹事長が党再生へ向けた提言を総裁選候補者に手渡す | capture 2009-10-01 of www.jimin.jp |
| `jp_ldp_sousai09_notice_20090918` | 総裁選スタート 候補者届出と所見発表演説会（平成２１年９月１８日） | capture 2009-09-24 of www.jimin.jp |
| `jp_ldp_sousai09_schedule_20090921` | 自由民主党｜スケジュール（総裁選挙、平成２１年９月２１日現在） | capture 2009-09-23 of www.jimin.jp |
| `jp_ldp_news_index_200909_20091001` | ニュース (LDP news index, September 2009) | capture 2009-10-01 of www.jimin.jp |
| `jp_ldp_sousai09_tanigaki_elected_20090928` | 第２４代自由民主党総裁に谷垣禎一氏を選出（平成２１年９月２８日） | capture 2009-10-03 of www.jimin.jp |
| `jp_ldp_sousai09_tanigaki_press_conference_20090928` | 谷垣禎一総裁が初の記者会見（平成２１年９月２８日） | capture 2009-10-03 of www.jimin.jp |
| `jp_ldp_news_tanigaki_yamba_20091002` | 谷垣禎一総裁が八ツ場ダム建設予定地を視察　地元住民とも意見交換 | capture 2009-10-11 of www.jimin.jp |
| `jp_ldp_history_tanigaki_era` | 谷垣禎一総裁時代 / 歴代総裁 / 党のあゆみ / 自民党について / 自由民主党 | capture 2026-02-08 of www.jimin.jp |

## Response identities and stability checks

The reviewer re-downloads every recorded response and compares its byte count and SHA-256, so every identity here was
downloaded at least twice, at least 30 minutes apart, with identical bytes, and each extract records a `fetch_recipe`: fetch the
recorded `url` exactly, with **no Accept-Encoding request header and no automatic decoding** (`curl -s -o FILE URL`, not
`--compressed`), and hash the bytes as received. No identity is a page generated per request. How each was established:

- **Researchers and checks.** Part A's researcher downloaded each response two or three times on 25 September 2026 (12:07Z-12:50Z,
  the Diet API responses also with a cache-busting query) and its check twice more (12:58Z-12:59Z and 13:40Z-13:42Z). Part B's
  researcher downloaded each at least twice, 30 minutes or more apart, and its check twice more (13:14Z-13:23Z and 13:57Z-13:59Z).
  Part C's researcher downloaded each once (the dossier's stability notes are replaced, check defect C8) and its check again 33-62
  minutes later (13:04Z-13:27Z) with `Accept-Encoding: identity`. The records the checks found were each downloaded twice by the
  check, 25 minutes or more apart. Each extract's provenance note gives that source's own times.
- **This packet (25 September).** Every one of the 81 identities, and the alternate copy of the 森 greeting, was downloaded again
  plain at 14:15Z-14:20Z and again at 14:52Z-14:56Z, at least 30 minutes later, and the 22 Diet API responses also with a cache-busting query at
  14:56Z-14:57Z. Every byte count and SHA-256 matched.

Reproducibility traps met and avoided:

- **Compression.** The Internet Archive sends `Content-Encoding: gzip` for many captures when asked; every recorded identity is the
  identity-encoded body. The party's list of presidents has three captures made before the cutoff: 20260825202443 is stored and
  served gzip-encoded (15,636 bytes) whatever the request, and 20260208090814 differs from the others in two header links; neither
  is recorded. The recorded capture, 20260509234631, is a Common Crawl record the archive replays uncompressed (the crawler stored
  the decoded body), and its 159,872 bytes equal the decoded body of the gzip capture and the live page on 25 September 2026
  (check defects A13, B16 and C9).
- **Edge caches.** Every direct download of the party-history PDF from storage2.jimin.jp after 12:00:13Z on 25 September was a
  CloudFront copy of one origin response, whatever the query or Origin header (check defect B15). The PDF is therefore recorded as
  the raw capture 20260208085555, whose SHA-1 equals the CDX digest of captures of this PDF from January 2023 to February 2026; the
  live file is its alternate location.
- **Old ARC digests.** The SHA-1 of the 森 greeting capture (20001213045900) does not equal its 2000-era ARC digest; its length
  equals the archived Content-Length and ETag, and the relocated copy (weekly_bn, capture 20010311194932) is byte-identical and
  recorded as the alternate location (check defect B17).
- **Growing and listing pages.** The party's list of presidents grows with each new president, so only a fixed capture is
  recorded. The September 2009 news index is a fixed, month-restricted capture of 1 October 2009, used only for supplementary
  headlines; the articles themselves are recorded from the party's 2009 election site. No search page, open listing or CDX query
  is a source.
- **The Diet minutes site is a JavaScript application.** Only the public API is recorded (`/api/meeting?issueID=…` or
  `/api/speech?issueID=…&speechNumber=…`, always with `recordPacking=json`); its responses carry `Cache-Control: no-cache,
  no-store` and no timestamp, token or request echo. Speaker searches were used for discovery only and are not recorded.

| Source | Bytes | SHA-256 | Kind |
|---|---|---|---|
| `jp_ldp_history_presidents_list` | 159,872 | `dcbd1425ba61…c1d7b4` | raw capture |
| `jp_ldp_history_kaifu_era` | 12,460 | `627457550f2e…ab1de4` | raw capture |
| `jp_ldp_history_miyazawa_era` | 18,264 | `d6c60c8997cb…3438a9` | raw capture |
| `jp_ldp_history_kono_era` | 18,492 | `9b2d9bcd7e7e…8edbab` | raw capture |
| `jp_ldp_history_hashimoto_era` | 19,673 | `ade2763655de…7fd567` | raw capture |
| `jp_ldp_ayumi_2015` | 3,313,141 | `2924c7dd5ba6…c0f482` | raw capture (PDF) |
| `jp_ldp_diet_hc_kessan_19891213` | 444,330 | `93db3b6c193e…e42817` | API meeting record |
| `jp_ldp_diet_hr_yosan_19900322` | 259,890 | `17fe1ecb11fd…16ee12` | API meeting record |
| `jp_ldp_diet_hc_yosan_19900514` | 430,898 | `06ba7990ea37…dda60e` | API meeting record |
| `jp_ldp_diet_hr_honkaigi_19911111` | 140,695 | `805ceb503014…36a469` | API meeting record |
| `jp_ldp_diet_hc_honkaigi_19911112` | 136,088 | `e8fb61813f4a…50b313` | API meeting record |
| `jp_ldp_diet_hc_honkaigi_19911113` | 195,182 | `872f18013cd6…023162` | API meeting record |
| `jp_ldp_diet_hr_yosan_19920220` | 570,013 | `6341943c7f3d…7f6482` | API meeting record |
| `jp_ldp_diet_hc_yosan_19920331` | 412,347 | `a5a65c742d60…9f1573` | API meeting record |
| `jp_ldp_diet_hr_koshokusen_19921130` | 391,976 | `04f49975390f…2d80a3` | API meeting record |
| `jp_ldp_diet_hr_yosan_19941011` | 594,581 | `45b4d370add2…8d7115` | API meeting record |
| `jp_ldp_diet_hr_yosan_19941013` | 274,055 | `5b3959405770…b359d7` | API meeting record |
| `jp_ldp_diet_hr_wto_19941125` | 352,687 | `f5a58ae98ff9…c6d280` | API meeting record |
| `jp_ldp_diet_hr_yosan_19950131` | 535,128 | `3c02d4da0aa7…007fe9` | API meeting record |
| `jp_ldp_diet_hr_honkaigi_19951002` | 114,052 | `47c45682e2af…784863` | API meeting record |
| `jp_ldp_diet_hc_honkaigi_19951003` | 173,253 | `94f7c6df5f7e…a011da` | API meeting record |
| `jp_ldp_diet_hr_yosan_19951026` | 521,450 | `098ac5caf4cc…474bb7` | API meeting record |
| `jp_ldp_sousai_ayumi_20011109` | 20,913 | `c6caae50dec8…f48c3e` | raw capture |
| `jp_ldp_sousai98_index` | 7,704 | `cdd05a24e17b…ca2a3f` | raw capture |
| `jp_ldp_sousai98_vote_page1_19980724` | 3,262 | `5e1bd2dbbd7a…950b20` | raw capture |
| `jp_ldp_sousai98_vote_page3_19980724` | 4,677 | `c976c2e7503e…d85492` | raw capture |
| `jp_ldp_hatsugen_obuchi_new_year_19990101` | 1,400 | `764ae3c474cf…4910a5` | raw capture |
| `jp_ldp_sousai99_index` | 14,505 | `259da6b4b953…d55df8` | raw capture |
| `jp_ldp_sousai99_diet_member_vote_19990921` | 5,530 | `18a0f64b3591…b690c0` | raw capture |
| `jp_ldp_sousai99_extraordinary_congress_19990922` | 5,397 | `a0cb20294212…3f92c7` | raw capture |
| `jp_ldp_sousai99_obuchi_greeting_19990922` | 4,132 | `a601c30f818c…8bc441` | raw capture |
| `jp_ldp_sousai99_press_conference_19990922` | 5,784 | `3614f3e6d4d6…91613e` | raw capture |
| `jp_ldp_mori_greeting_joint_plenary_20000405` | 4,270 | `c52bf4725103…70aa5f` | raw capture |
| `jp_ldp_mori_special_20000414` | 4,592 | `2a623445749f…1f876f` | raw capture |
| `jp_ldp_toutaikai67_programme_20010313` | 6,524 | `adc8eda49de0…16b6c1` | raw capture |
| `jp_ldp_toutaikai67_mori_address_20010313` | 8,889 | `57842b32c905…bb82c1` | raw capture |
| `jp_ldp_sousai01_top_20010428` | 12,026 | `089f3476d1ea…654097` | raw capture |
| `jp_ldp_sousai01_rules_2001` | 10,188 | `38b91baa3d27…8d7f0b` | raw capture |
| `jp_ldp_sousai01_joint_plenary_20010424` | 6,779 | `2fecf5c7b210…07a3dd` | raw capture |
| `jp_ldp_sousai01_press_conference_20010424` | 2,843 | `e3e64b57bac7…66527a` | raw capture |
| `jp_ldp_koizumi_reappointment_greeting_20010810` | 1,758 | `c3cf6e8daf4d…a7c095` | raw capture |
| `jp_ldp_sousai03_notice_20030908` | 13,043 | `1cad6d55ea0e…9a2226` | raw capture |
| `jp_ldp_sousai03_result_20030920` | 13,094 | `51b5bdbd5ae6…ac693a` | raw capture |
| `jp_ldp_sousai03_press_conference_20030920` | 24,948 | `be50bc13d5a2…1563f5` | raw capture |
| `jp_ldp_diet_sangiin_budget_obuchi_19980821` | 4,628 | `d4182c28479f…77c804` | API speech |
| `jp_ldp_diet_joint_budget_obuchi_19991110` | 2,359 | `7771c162bfe1…470431` | API speech |
| `jp_ldp_diet_shugiin_plenary_mori_20000410` | 15,191 | `5c7afccd514d…572a2b` | API speech |
| `jp_ldp_diet_sangiin_budget_koizumi_20010522` | 1,780 | `97c78b04c72d…86e376` | API speech |
| `jp_ldp_diet_shugiin_budget_koizumi_20010914` | 2,915 | `15346ccff638…e48d0d` | API speech |
| `jp_ldp_diet_sangiin_plenary_koizumi_20030930` | 25,199 | `eae14fb36cb3…7d72f0` | API speech |
| `jp_ldp_news_2006_election_notice_20060908` | 31,058 | `724770376043…19207f` | raw capture |
| `jp_ldp_sousai06_schedule_20060918` | 14,859 | `74df327a08ae…b037b5` | raw capture |
| `jp_ldp_news_abe_elected_20060920` | 31,440 | `81f2fd2a1089…48c352` | raw capture |
| `jp_ldp_news_abe_first_press_conference_20060920` | 31,167 | `ab9c9b65d25e…ccda58` | raw capture |
| `jp_ldp_news_joint_plenary_20060920` | 31,271 | `a439ef39de53…6a0fda` | raw capture |
| `jp_ldp_news_abe_executive_20060925` | 30,948 | `5a297bf4531d…5714c1` | raw capture |
| `jp_ldp_jiyuminshu_2247_notice_20060925` | 31,011 | `399217708262…531289` | raw capture |
| `jp_ldp_news_abe_president_joint_plenary_20060926` | 31,192 | `512e9f4b17f3…7237be` | raw capture |
| `jp_ldp_jiyuminshu_2248_notice_20061002` | 37,752 | `d3e1a01ce27e…1596e9` | raw capture |
| `jp_ldp_news_abe_resignation_election_method_20070912` | 29,063 | `2d7818a848f3…36c933` | raw capture |
| `jp_ldp_news_2007_election_schedule_20070913` | 29,390 | `461fb04bd1e5…f6a6a1` | raw capture |
| `jp_ldp_news_2007_candidates_filed_20070915` | 30,557 | `0cefee039588…3cce8c` | raw capture |
| `jp_ldp_news_fukuda_elected_20070923` | 28,615 | `638239c17e4f…a1ec75` | raw capture |
| `jp_ldp_news_fukuda_first_press_conference_20070923` | 29,407 | `369c33ee0706…9310f8` | raw capture |
| `jp_ldp_news_fukuda_party_officers_20070924` | 29,533 | `6dba64c4ea78…da7429` | raw capture |
| `jp_ldp_jiyuminshu_2293_notice_20070925` | 29,433 | `c07ce04cdf2b…395fd2` | raw capture |
| `jp_ldp_news_joint_plenary_schedule_20080903` | 27,588 | `d8b87c67f6b3…8fc521` | raw capture |
| `jp_ldp_news_2008_election_notice_20080910` | 25,194 | `dd33bfd974e1…fad9ef` | raw capture |
| `jp_ldp_news_aso_elected_20080922` | 29,124 | `5bc47cf1a3d8…14bb27` | raw capture |
| `jp_ldp_news_aso_first_press_conference_20080922` | 27,170 | `8d1a1f958bfe…704b49` | raw capture |
| `jp_ldp_news_aso_party_officers_20080922` | 26,978 | `90cf884b6de8…8ec5cf` | raw capture |
| `jp_ldp_jiyuminshu_2339_notice_20080924` | 25,552 | `b301771ad6fe…7f0bdb` | raw capture |
| `jp_ldp_news_joint_plenary_schedule_20090908` | 26,822 | `d162ceb56cc3…aca6da` | raw capture |
| `jp_ldp_news_2009_notice_day_20090918` | 26,739 | `6f731e44cf6c…15bd6a` | raw capture |
| `jp_ldp_sousai09_notice_20090918` | 8,088 | `aabfb292795c…a5fb23` | raw capture |
| `jp_ldp_sousai09_schedule_20090921` | 11,987 | `f25547d79cac…e0a6ab` | raw capture |
| `jp_ldp_news_index_200909_20091001` | 27,053 | `422b8ee220b8…33fced` | raw capture |
| `jp_ldp_sousai09_tanigaki_elected_20090928` | 8,172 | `bfc6602c4a23…7a3ffa` | raw capture |
| `jp_ldp_sousai09_tanigaki_press_conference_20090928` | 7,959 | `8808b85e05e5…2775be` | raw capture |
| `jp_ldp_news_tanigaki_yamba_20091002` | 27,753 | `5be1abc555ae…1b75ae` | raw capture |
| `jp_ldp_history_tanigaki_era` | 17,970 | `751add7caa71…033fe9` | raw capture |

## Leads not imported

- Diet statements by LDP members whose party office the record does not identify: 平井卓志, 6 March 1990 (111815254X00319900306 #9,
  海部's 1989 selection); 野田毅, 9 May 1990 (111805261X01719900509 #39); 北修二, 12 November 1991 (#10 of the imported record);
  下稲葉耕吉, 8 October 1993 (112815261X00319931008 #339, 宮澤総裁's resignation); 鹿野道彦, 19 October 1993
  (112804573X00419931019 #96, '私どもの河野総裁', the earliest LDP-member attestation of 河野 found; also 5 November 1993);
  町村信孝, 26 August 1993 (112705254X00619930826 #3); 小渕恵三, 20 July 1994 (113005254X00219940720 #10). Only party officers are
  used.
- 河野洋平's statements of 19 October 1995 in the House of Councillors Foreign Affairs Committee (113413968X00119951019 #178, with
  #104 and #180): he completed his term and decided not to stand, and 橋本さん now leads the party. He spoke as foreign minister and
  former president, holding no party office, so these are leads (check A1); the withdrawal is a claim from the party's own page.
- The closing sentence of the 河野 history page (the 1995 upper-house result leading to the autumn change of president): an undated
  retrospective reference, not imported (check A4, optional).
- The party's per-president pages for 1998-2009 (https://www.jimin.jp/aboutus/history/18.html, 19.html and 20.html; the archived
  aboutus/history/21.html, 22.html and 23.html of 8 February 2026, 15,325, 15,747 and 13,261 bytes): the same text as the imported
  printed history (18-20) or the contemporaneous records (21-23), with 首相 wording for 2007 and 2009.
- 1998 and 2001 election pages: vote page 2/3 (sousai98/pics-42.html, 5,258 bytes; 谷川 opened the vote at 14:08); the 2001 page
  before the vote (sousai01/index.html) and the transcript of the 24 April 2001 press conference (sousai01/ss_kaiken.html, 20,816
  bytes, where 新総裁就任 is a reporter's word); the 2003 press-conference news item (sousai03/news/20/150920b.shtml), the 2003
  member-vote table (sousai03/kenren/kekka.html) and the 2003 pre-vote schedule (sousai03/schedule.html); the index of party
  officers' statements (hatsugen/index.html) and the 自由民主 pages of August 2001 (/jimin/130810/).
- The party page on 小渕恵三前総理 in hospital (120413/obu/index.html) and the party history's passage on 小渕首相's stroke of 2 April
  2000 (withdrawn claim `jp_ldphist_obuchi_stroke_reported_20000402`): prime-ministerial context; CLAUDE-C01-12 records the stroke
  for `jp_pm` (check B14).
- Further in-office statements not downloaded: 小泉純一郎, 9 October 2003 (115724293X00120031009 #13, 'あと三年間、自民党総裁として
  任期を与えられました'); 小渕恵三, 28 August 1998 (114304056X00419980828 #282); and the opposition speaker 千葉景子 on 30 September 2003
  (speech 2 of 115715254X00220030930), not a party officer.
- 2006-2009 records the part C check listed as optional or procedure only, each downloaded twice by the check with a stable identity:
  the 2009 party-member vote table by prefecture (https://web.archive.org/web/20091003234013id_/http://www.jimin.jp:80/jimin/jimin/sousai09/kekka.html,
  10,152 bytes, SHA-256 9f0aa66246e1…6d0932), the 2006 results table (sousai/data_01/kekka.html, 34,333 bytes, 4813040eec69…8411d7) and
  the 2006 member-vote PDF (sousai/data_01/pdf/kekka.pdf, 13,270 bytes, 9c035ec223dc…0fcc8): redundant corroboration of the totals the
  same-day articles already carry; the 2008 election rules (sousai08/data_01/youryou.html, 17,124 bytes, d1e7ecac3a17…a5bf):
  procedure only, never a date.
- Other 2006-2009 party pages reviewed and not imported: the 2 September 2008 notice (daily/08_09/02/200902a.shtml, 25,429 bytes;
  福田康夫総理's 退陣, prime-ministerial wording); the pages on the Diet designations and cabinet formations of 2007 and 2008
  (07_09/25/190925a, 07_09/26/190926a, 08_09/24/200924b and 200924d; `jp_pm` matters); the October 2009 index and the 5 October 2009
  notice (09_10index.shtml, 211005a.shtml; redundant attestations of 谷垣禎一総裁); the English translation of the election rules
  (e-presidentRule; procedure only).
- Diet speeches of October-November 2009 in which LDP officers refer to 谷垣総裁 (大島理森, 林芳正): not needed; 谷垣禎一's own Diet
  speeches of 28 September to 31 December 2009 contain no 総裁 self-reference.
- The two other captures of the party's list (20260825202443, gzip-encoded; 20260208090814), used by parts B and C: the same rows
  are recorded once from the capture of 9 May 2026 (checks B16 and C9).
- The party rules (党則, 総裁公選規程) and the Diet designations are procedure or `jp_pm` matters; no news, encyclopaedia or history
  site is used.

## Sources attempted

- Contemporaneous LDP records of 1989-1995 (announcements, convention minutes, result declarations and the party organs 自由新報 and
  月刊自由民主): none found online. Internet Archive CDX queries for 1990s captures of www.jimin.or.jp and www.jimin.jp timed out or
  returned 'Temporarily Offline' or HTTP 503 (12:00Z-13:15Z on 25 September 2026); an NDL Search catalogue query found no openly
  available digitised party organ (the part A check's third missing record).
- https://www.jimin.jp/aboutus/president/: 404; the list is part of /aboutus/history/.
- The daily-news URLs of the 2009 election articles (www.jimin.jp/jimin/daily/09_09/28/210928a.shtml, 210928b.shtml and
  09_09/18/210918b.shtml) have no capture; the same articles are archived under /sousai09/news/ and are recorded from there. No
  capture exists for 09_09/16/ (where a record of 麻生太郎's resignation on 16 September 2009 would appear), for 07_09/14/ or
  08_09/01/, or for 210929a and 210929b.
- The jimin.or.jp address of the 2001 reappointment greeting has only a redirect capture; the jimin.jp capture of the same page is
  recorded.
- NDL WARP returned 403 Forbidden to curl; it was not retried or bypassed. The built-in browser was not used.
- The Internet Archive refused connections, returned HTTP 429 or served empty bodies at times on 25 September 2026 while several
  agents shared the address; requests were retried one at a time, and no recorded identity is an error page.
- No terms, logins, CAPTCHAs or access controls were met or bypassed.

## Checker defects

| # | Defect | Outcome |
|---|---|---|
| A1 | 河野's 19 October 1995 statements were made as foreign minister, not as a party officer | **Applied**: both moved to leads with speeches #104, #178 and #180; the source dropped; the withdrawal rests on the party's own page |
| A2 | Six rows carry holder_name 宮沢喜一 | **Applied**: every row carries 宮澤喜一; the printed 宮沢 stays in the texts and uncertainties |
| A3 | Wrong locator for 河野's assumption on the 宮沢 page | **Applied**: body paragraph 26 of 27, 'second-to-last body paragraph' in the text |
| A4 | False statement that the 河野 page ends with the Gulf crisis | **Applied**: the visual review corrected; the closing sentence on the 1995 change of president is a lead |
| A5 | List rows merge the notice with the convention act (1989, 1993) | **Applied**: split into a notice claim (no holder name) and a convention claim for 1989 and 1993; the 1997 橋本 entry is imported in the same split form |
| A6 | The 海部 re-election narrative merges four events | **Applied**: split into the 6 October notice and one-candidate election and the 31 October report and reappointment |
| A7 | The 8 August 1989 narrative merges election and assumption, and misreads 宇野内閣の退陣表明 | **Applied**: a separate retrospective assumption claim; the text says the 宇野 cabinet's announced resignation |
| A8 | The 宮澤 holder rests on an implicit statement | **Applied**: dated 30 November 1992, his explicit statement; the 20 February 1992 record imported and it and 31 March 1992 kept as implicit references |
| A9 | 'then LDP president' wording and missing 'no structured date' | **Applied**: the text attributes the selection to his own words; the uncertainty says no structured date is stored |
| A10 | Continuation claims labelled in_office_attestation | **Applied**: 31 January 1995 and 3 October 1995 are `in_office_continuation_attestation`; 30 November 1992 now dates the 宮澤 holder |
| A11 | Party-role texts carry the prime-ministerial designation | **Applied**: the designation clauses removed; the uncertainty says they belong to `jp_pm` |
| A12 | Selection and seat-taking merged (11 October 1994) | **Applied**: the uncertainty says they are reported together, without a day, and neither is a boundary |
| A13 | The list capture's provenance omits the Common Crawl source and the growing-list caveat | **Applied**: WARC name, crawler decoding, the fixed capture and the two other captures recorded |
| A14 | Dossier shape does not match the house shape | **Applied**: claim and row keys, access methods and rights notes as in CLAUDE-C01-13; printed and statement dates folded into the texts; `observation_id` `jp_sangiin_pr_2025_13` and the role title on every row |
| B1 | 森 and 小泉 holders set both attested_on and from | **Applied**: 森喜朗 has `from` only; 小泉純一郎 has `attested_on` only, because a 就任 caption is not a stated start |
| B2 | Holders cite elections, reports, retrospective rows and other days' claims | **Applied**: each holder cites only same-day in-office attestations (森: his statement of assumption), each carrying the holder's day; pinned in the test |
| B3 | Re-elections folded into the first holder; 就任 read two ways | **Applied**: separate observations for 1999, August 2001 and 2003; one rule for 就任 wording stated and pinned (never a start) |
| B4 | Retrospective list rows carry structured dates | **Applied**: moved to the one recorded list capture, undated, with `retrospective_*` kinds |
| B5 | The printed history's claims carry structured dates | **Applied**: undated `retrospective_*` claims, never cited by a holder |
| B6 | Caption pages print no year | **Applied**: each claim says where the year comes from; the 1998 index imported as the year anchor |
| B7 | published_date taken from Last-Modified | **Applied**: `published_date` is null for every source; the header is in the provenance note only |
| B8 | 森's greeting paraphrase drops ただ今 | **Applied**: ただ今 quoted and the clause rendered as a present assumption; from 5 April 2000, flagged for the reviewer |
| B9 | 2001 top-page locator; the 11 April notice not claimed | **Applied**: the 17:30 press conference located in the event-page list; the 11 April 2001 notice imported as its own claim (the preliminary counts, optional, are not) |
| B10 | Reappointment greeting locator | **Applied**: paragraphs 1, 2 and 5 |
| B11 | The 1999 convention page merges three events | **Applied**: convention opened and result reported (no holder name) and 小渕総裁's greeting as three claims |
| B12 | The 1998 page 1 claim mixes the venue with 橋本総裁's greeting | **Applied**: the meeting (no holder name) and 橋本総裁's greeting (continuation, LDP-PRES-04) as two claims; pics-41 and pics-43 imported once |
| B13 | Romanized names in claim texts | **Applied**: the printed forms (小渕内閣, 森首相, 森総裁, 小泉内閣総理大臣) |
| B14 | The stroke claim is prime-ministerial | **Resolved by removal**: withdrawn and listed as a lead; CLAUDE-C01-12 records it for `jp_pm` |
| B15 | The PDF's later downloads were CloudFront copies | **Applied**: recorded as the raw capture 20260208085555, with the live file as alternate location and the caveat in the provenance |
| B16 | The gzip-encoded list capture | **Resolved**: not recorded; its rows are recorded from the identity-encoded capture 20260509234631, whose bytes equal its decoded body |
| B17 | The 森 greeting body does not match its CDX digest | **Applied**: the byte-identical weekly_bn copy recorded as alternate location, with the ARC digest explanation |
| B18 | Extract rows lack integration fields | **Applied**: `observation_id`, `review_observation`, `role_title` and a `fetch_recipe` in every extract |
| B19 | Summary wording on the 1998 pages and the PDF | **Applied**: the report says the 1998 pages are 2001 web copies and the PDF's later downloads were cache copies |
| C1 | 谷垣禎一's holder rests on index headlines | **Applied**: 210928a and 210928b imported; the count, the declaration, the address, 麻生太郎前総裁 and the press conference as separate claims; the holder cites the address and the press conference; headlines supplementary; unresolved notes rewritten |
| C2 | The 2009 notice and nominations rest on a side mention and a headline | **Applied**: 210918b imported; notice and candidacies separate; the side mention and headline supplementary |
| C3 | The term-expiry schedules were missed | **Applied**: the 2006 and 2009 schedules imported as prospective claims with no holder name, never a from or until |
| C4 | The 谷垣 era page carries a structured date | **Applied**: undated, and its id no longer carries the date |
| C5 | 小泉政権 wording in a party claim | **Applied**: removed; the claim is a continuation on LDP-PRES-07 with no end; no duplicate in part B |
| C6 | 2006 and 2008 notices merged with nominations | **Applied**: split into a notice and a candidacies claim each |
| C7 | 福田's 辞意表明 read as a resignation | **Applied**: 'announced his intention to resign (辞意表明)' |
| C8 | Stability notes lack a 30-minute repeat | **Applied**: every provenance note gives the researcher's, the check's and this packet's download times |
| C9 | The list's 'non-matching attempt' was another capture | **Resolved**: the list is recorded once from the capture of 9 May 2026; the provenance names both other captures |
| C10 | Sources and rows lack integration fields | **Applied**: rights notes, source types, scope notes, snapshots, role titles and `observation_id` everywhere |

Missing primary records the checks found: part A's three: the 20 February 1992 and 13 October 1994 Diet records imported; the
contemporaneous party records of 1989-1995 not found (Sources attempted). Part B's nine: seven imported as sources (the 1999
convention greeting, the 1 January 1999 message, the 67th convention address and programme, the 2003 notice, the 1998 index and the
1999 press-conference page), the relocated 森 greeting recorded as an alternate location, and the PDF capture recorded as that
source's location. Part C's nine: five imported (the 28 September 2009 election and press-conference articles, the 18 September
2009 notice and the 2006 and 2009 schedules) and four listed as leads (the 2009 and 2006 result tables and the 2006 member-vote PDF,
redundant with the same-day articles; the 2008 rules, procedure only). This packet downloaded each imported record again, as for
every source.

Other changes made to fit the packet's rules rather than a numbered defect:

- One source per response: the three parts' copies of the party's list and its printed history are each recorded once
  (`jp_ldp_history_presidents_list`, `jp_ldp_ayumi_2015`); part B's and part C's list rows and part B's history claims moved there,
  and the list rows of all eleven Presidents are split into span, election and result claims as part A's are.
- Every added id begins `jp_ldp` (`jp_diet_` renamed `jp_ldp_diet_`), so the separation from `jp_pm` is pinned by id; renamed
  claims include `jp_ldp03kaiken_press_conference_as_president_20030920`, `jp_ldp_aso_new_president_address_20080922`,
  `jp_ldp_2009_notice_day_proposals_to_candidates_20090918` and `jp_ldp_history_tanigaki_era_election_narrative`.
- Recollections that do not state the day of the event are undated (as in CLAUDE-C01-13); those that do (橋本's 25 September 1995,
  the party organ's 23 September 2007) are dated by the day they state. Party-organ notices that speak of the new president are
  continuation attestations dated by the notice.
- `published_date` is null for every new source.

## Suggested next work orders

These are proposals for the integrator. They are not created in `work-orders.json`.

- `C01-Japan-LDP-002`: LDP presidents from October 2009 to September 2024 (谷垣禎一's term, 安倍晋三 2012-2020, 菅義偉 and 岸田文雄), to
  join this packet to the 2024 and 2025 observations.
- `C01-Japan-LDP-003`: contemporaneous party records of 1989-1995 (自由新報, 月刊自由民主, convention records) through lawful access,
  to date the 1989-1995 elections and assumptions contemporaneously.
- `C01-Japan-LDP-004`: the term that followed 橋本龍太郎's 1997 re-selection, and whether any party record states the day a
  president's term takes effect or ends (the 2006 and 2009 conflicts; 麻生太郎's end in 2009).
- `C01-Japan-LDP-005`: the vice presidents and secretaries-general as separate party roles (outside this packet).

## Integration notes (outside this packet's file boundary)

- **Stack:** this branch holds the claim commit `30c507d7` on top of `claude/c01-jp-13` at `42f98dc8` (CLAUDE-C01-13, ready for
  review and not integrated, itself stacked on `claude/c01-jp-12`), and this packet's two commits. Before the work
  `origin/claude/c01-jp-13` was fetched and had no commit missing from this branch, so no merge was needed. Merge CLAUDE-C01-12,
  then CLAUDE-C01-13, then this packet: `japan.json` and the three Japan tests are shared with them.
- `japan.json` changes are additions only: 81 sources appended after CLAUDE-C01-13's; on the 自由民主党 organization observation, its
  `sources` and `claim_ids` and the role's `sources` and `claim_ids` extended, fourteen holders inserted before the 2024 and 2025
  ones (unchanged), a sentence appended to the role's `scope_note` and one coverage note appended to the organization; one packet
  coverage note appended after CLAUDE-C01-13's. No existing extract is edited.
- `research-index.json` is regenerated in a **separate commit** and is the only file shared with other pending packets (the India,
  Brazil, Russia and South Africa packets add their own sources and claims); regenerate it after they merge. New Japan totals: 418
  sources and 685 claims (CLAUDE-C01-13: 337 and 496); institutions, roles (5), `mapping_pending` (24) and the Japan work orders
  (10, 10 and 4 members) are unchanged.
- Pinned tests, none loosened and no assertion removed:
  - `test_japan_research_s10d.py`: counts (entries, sources, claims, roles) are now (24, 418, 685, 5), with this packet's 81 sources
    and 189 claims pinned beside CLAUDE-C01-12's and CLAUDE-C01-13's; the party holders' count is 19 and the guard that no party
    holder has a boundary is re-expressed exactly (the one stated start is 森喜朗's 2000-04-05, and no holder has an until); the LDP
    holders' dates are pinned as the new exact list; the access-date guard is split exactly (CLAUDE-C01-13: 24 and 25 September;
    this packet: 25 September); the extract count is 2 + 119 + 211 + 81.
  - `test_japan_prime_ministers_c01_12.py`: its exact source list is extended with this packet's sources (imported from
    `test_japan_ldp_presidents_c01_18.py`), and the packet coverage notes it pins are now 11 in all, with CLAUDE-C01-12's at index 8
    and CLAUDE-C01-13's at index 9 unchanged.
  - `test_japan_prime_ministers_c01_13.py`: its rows are loaded only from its own 211 sources; the source order and total are its own
    followed by this packet's; the LDP holders it pins are the new exact list (the 2024 and 2025 observations unchanged) and party
    office still never feeds `jp_pm`; its packet coverage note is pinned at index 9, followed only by this packet's.
- `test_campaign_census` needs `spheres-sim/data`, which is absent from this sparse worktree; it was not run, and the sparse
  checkout was not widened.
- The atlas (`tools/ui/leadership-research-review.js`) shows 'Observed on' for a holder with `attested_on`; thirteen of the new
  holders have it and 森喜朗 has a `from`. No UI code changed.
- New extract fields beside CLAUDE-C01-13's: `source_response_sha1_base32` and `fetch_recipe` (as in the ANC packet), and
  `alternate_location` on two extracts.
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
