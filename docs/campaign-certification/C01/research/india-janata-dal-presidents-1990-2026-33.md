# Janata Dal presidents 33: the party office, 1990-2026

Packet: **CLAUDE-C01-33**. State: **ready_for_review** (not complete).
Owner: Claude. Integrator/reviewer: Codex. Branch `claude/c01-in-33`; claim commit `89decb6a`, made while the packet was
**stacked on the then-pending packet CLAUDE-C01-27** (`claude/c01-in-27`, which also edits `india.json`) merged with
integration `3ec6e155` at `6fdb950e`. CLAUDE-C01-27 has since been integrated into `codex/campaign-certification`, and this
branch has merged current integration `032cd6a3` at `a266ebcb`, so it is based on `codex/campaign-certification` at
`032cd6a3` and nothing needs merging first. Research access: 28 September 2026 (UTC); every recorded
response was downloaded on that day, between 19:27:23Z and 20:51:03Z, which is each source's `accessed_date`. The
historical cutoff stays **7 September 2026**.

This packet adds to [india.json](india.json) one organization observation, `in_eci_19980110_np_06` ("Janata Dal", kind
`political_party_national_recognition_observation`), taken from row 6 of the Election Commission's national-party table
of 10 January 1998 (O.N. 18(E)), because the Janata Dal has no row in the March 2024 notifications the packet was built
from. The observation carries one party role, `in_jd_president` ("President of the Janata Dal", kind `party_leader`). The
packet adds 22 sources and 41 claims (14 about the organisation, whose rows carry `role_id` null, and 27 about the office),
three holder observations with no stated start or end, a role scope note, an observation coverage note and one bounded
note in the packet coverage. The observation's lifecycle stays `unresearched`, its coverage `reporting_identity_only` and
its game mapping empty; no successor is merged into it, and the 2024 rows of the Janata Dal (Secular), the Janata Dal
(United) and the Rashtriya Janata Dal are untouched. It changes nothing that CLAUDE-C01-11, CLAUDE-C01-15, CLAUDE-C01-20
or CLAUDE-C01-27 added: the thirteen prime-minister, eight president, ten Congress-president and nineteen BJP-president
holders are unchanged, and no existing extract is edited. The party office and the `in_prime_minister` and `in_presidency`
institutions stay separate both ways, and `in_inc_president` and `in_bjp_president` do not change. The parent scope (C01,
C06, S23, WC1 and CP1) remains open.

## Outcome

| ID | Question | Decision |
|---|---|---|
| JD-PRES-01 | The President when the period opens (V. P. Singh) | **Declined:** the Rajya Sabha record of 28 Dec 1989 ('He is the President of the Janata Dal') is before the period; on 18 May 1990 a member recalls that 'The Prime Minister in his capacity as President of the Janata dal' wrote to the Election Commissioner, an undated act that names nobody; no record attests V. P. Singh in the office on a single day from 1 Jan 1990, and no record of the day he left it or of the day S. R. Bommai took it over was found |
| JD-PRES-02 | S. R. Bommai (1990-1991) | **Accepted in part:** observed on 14 Jul 1990 (the Prime Minister's letter of that day to 'Shri S. R. Bommai, President, Janata Dal', printed in the Rajya Sabha's written answer of 28 Aug 1990); styled again in that answer and in the Ministry of Welfare's resolution of 29 Mar 1991; a Lok Sabha statement of 29 Aug 1991 lists him from the National Integration Council 'last reconstituted in 1990' (a list of an earlier date); his own 1997 recollection of a 1989 report made 'As Janata Dal President' is kept as printed; no start |
| JD-PRES-03 | The split of 1992-1993 (Ajit Singh) | **Accepted in part:** claims only. Bommai styled President in the Welfare resolution of 14 Feb 1992 and, in a pleading recalled by the Speaker's decision of 1 Jun 1993, as the President who expelled Shri Ajit Singh (26 Dec 1991) and four other members (19 Jul 1992); the Ajit Singh group's claim that he was endorsed as President on 5 Feb 1992; the Election Commission froze the name and symbol on 14 Jan 1993, recognised the groups headed by Ajit Singh and Bommai ad hoc as the Janata Dal (A) and (B), and on 22 Jul 1993 recognised the group represented by Bommai as the Janata Dal; no holder changes |
| JD-PRES-04 | Laloo Prasad Yadav (1996) and Bommai's end | **Accepted in part:** observed on 15 Jul 1996 ('The Janata Dal Party Supremo, the party President, the Bihar Chief Minister', Lok Sabha, in English); styled again under the heading 'Presidents of all Major National Political Parties' on 14 Oct 1996 and as 'your president and kingmaker' on 22 Apr 1997; President of the Rashtriya Janata Dal, another organisation, by 29 Jul 1997; the 2007 obituary synopsis gives Bommai 'from 1990 to 1996' (retrospective); no record of Bommai's end or of Laloo Prasad Yadav's election, start or end |
| JD-PRES-05 | Sharad Yadav (1997) | **Accepted in part:** 'the working President of Janta Dal' on 17 Mar 1997 (a claim only); observed on 29 Jul 1997 ('party president, Shri Sharad Yadav' and 'Democratically elected president Shri Sharad Yadav', Lok Sabha, official translation); styled again under the Presidents heading on 29 Sep 1997; an unnamed 'President of the JD' on 5 Aug 1997; no record of the day he was elected or assumed the office |
| JD-PRES-06 | The dispute of 1999 (Sharad Yadav and H. D. Deve Gowda) | **Accepted in part:** claims only. The Political Affairs Committee is claimed to have removed Sharad Yadav on 21 Jul 1999 and elected Deve Gowda; Deve Gowda applied on 22 Jul 1999 'as the President'; the National Executive's endorsement of Sharad Yadav is claimed for 29 Jul 1999; on 7 Aug 1999 the Commission recorded Sharad Yadav as President 'As per the Commission's records', found the party split vertically and recognised both groups ad hoc; no end for Sharad Yadav and no holder for Deve Gowda |
| JD-PRES-07 | The organisation observation | **Accepted:** new observation `in_eci_19980110_np_06` from row 6 of the Commission's national-party table of 10 Jan 1998 ('Janata Dal', Chakra (Wheel), 7, Jantar Mantar Road); the table entries of 23 Jul 1993 and 5 Feb 1996 ('Janta Dal', so printed) are organisation claims |
| JD-PRES-08 | The name after the 1999 split | **Accepted in part:** the table of 9 Aug 1999 keeps entry 6 'Janata Dal' as 'The name of the party and the symbol under dispute' and adds 6(A) Janata Dal (Secular) and 6(B) Janata Dal (United); the compilation's note on the Commission's subsequent order of 7 Aug 1999; both groups are claims about the organisation, never successors or inherited leaders; no holder is tied to the Janata Dal name after 7 Aug 1999 |

The resulting holder observations of `in_jd_president`, in date order:

| Holder | `attested_on` | `from` | `until` | Basis |
|---|---|---|---|---|
| S. R. Bommai | 1990-07-14 | null | null | the Prime Minister's letter of 14 Jul 1990 to 'Shri S. R. Bommai, President, Janata Dal', printed in the Rajya Sabha's written answer of 28 Aug 1990 (official debates store) |
| Laloo Prasad Yadav | 1996-07-15 | null | null | Lok Sabha, 15 Jul 1996: Shri Sriballav Panigrahi, in English, 'The Janata Dal Party Supremo, the party President, the Bihar Chief Minister' (Internet Archive copy of the Lok Sabha Secretariat's file) |
| Sharad Yadav | 1997-07-29 | null | null | Lok Sabha, 29 Jul 1997: Shri Ram Naik, 'party president, Shri Sharad Yadav', and Shri Anandrao Vithoba Adsul, 'Democratically elected president Shri Sharad Yadav' (official translation; Internet Archive copy) |

### How a start and an end are decided

The rule of the stacked India packets and of the party-leader packets (CLAUDE-C01-16, CLAUDE-C01-20 and CLAUDE-C01-27)
applies. A holder has `from` only where a source states the day the office was assumed, and `until` only where a source
states the day it ended; otherwise the holder is dated by `attested_on`, from a same-day in-office attestation that names
the holder and styles the office. No record reviewed states either day for any President of the Janata Dal, so the three
holders are dated observations only, each citing only in-office attestations of its own day.

Kept as claims that never feed a holder: later attestations of a holder already observed (`in_office_continuation_attestation`),
lists under a category heading, a list describing an earlier composition, the working presidency, recollections of acts
(dated or not), retrospective statements and spans, an attestation before the period, rival claims to the office, a
removal claimed, an endorsement claimed, the Commission's statement of its records, a passage that names nobody, and the
presidency of another organisation (the Rashtriya Janata Dal). Claims about the organisation (the Commission's table rows,
its 1993 and 1999 dispute orders and the rows of the Janata Dal (Secular) and (United)) carry `role_id` null and
`holder_name` null and never feed the office. Parliament records and Government resolutions are used only where they
record the party office.

Rulings on the questions the research left open:

- **A new observation, not a borrowed one.** The Janata Dal has no row in the Commission's notifications of 23 March 2024,
  so the scope allows a new observation only from a primary record naming it. Row 6 of the national-party table of 10
  January 1998 is used: a complete Table I of the Commission's own notification, before the 1999 split, printing the name
  'Janata Dal'. The entries of 17 January 1993 (under dispute), 23 July 1993 and 5 February 1996 ('Janta Dal', so printed)
  are organisation claims on the same observation; no spelling is reconciled from them.
- **Organisation rows name groups, not offices.** The Commission's orders speak of the groups 'headed by' Shri Ajit Singh
  and Shri S. R. Bommai (1993), the group 'represented by' Shri S. R. Bommai (the final order of 22 July 1993) and the
  groups 'led by' Shri Sharad Yadav and Shri H.D. Deve Gowda (1999); none styles an office, so these rows carry
  `holder_name` null and the names stay in the text.
- **A heading is not an observation.** The resolutions of 14 October 1996 and 29 September 1997 list Laloo Prasad Yadav
  and Sharad Yadav under 'Presidents of all Major National Political Parties', a heading that also covers two General
  Secretaries; both are continuations of holders already observed on single days in Parliament.
- **A list of an earlier date attests nothing on its answer's day.** The Lok Sabha statement of 29 August 1991 lists the
  National Integration Council 'last reconstituted in 1990' and still names Rajiv Gandhi, who had died in May 1991, as
  Congress President; its Bommai entry has no structured date.
- **Pleadings are claims.** The expulsions dated 26 December 1991 and 19 July 1992 and the Ajit Singh group's claim of 5
  February 1992 are the parties' pleadings as summarised by the Speaker, who decided membership of the Lok Sabha and of the
  Parliamentary Party, not the party office.
- **The 1990 change has no day.** V. P. Singh is attested in the office on 28 December 1989, before the period; the Meham
  recollection of 18 May 1990 says only 'the Prime Minister' and has no day; Bommai is first attested on 14 July 1990. His
  own statement of 26 August 1997 that he gave a report 'As Janata Dal President' in 1989 conflicts with those records and
  is kept as printed, with no structured date. The header of the Meham record prints '[13 MAY 1990]' where the Secretariat
  dates the sitting 18 May 1990; neither is used as a date.
- **Translations are the official report.** The Lok Sabha statements of 17 March, 22 April and 29 July 1997 were made in
  Hindi; the report prints the official English translation, which is quoted. The observation of 15 July 1996 is an
  English speech.
- **No end in 1999.** The Political Affairs Committee's removal of Sharad Yadav on 21 July 1999 is one side's claim,
  contested and not decided; the Commission's order of 7 August 1999 recorded him as President 'As per the Commission's
  records' while leaving the presidency undecided and recognising both groups ad hoc. Neither is an end, and Deve Gowda's
  claimed election is not a holder.
- **A two-language conflict stores no date.** The English text of O.N. 8(E) dates the further order naming the Janata Dal
  (A) and (B) 17 January 1993 and the Hindi text 16 January 1993; that claim has no structured date.
- **Mirror-only and repository copies are disclosed.** Four Lok Sabha files are Internet Archive copies of the Parliament
  Digital Library's PDFs (the eparlib hosts could not be reached, and no capture of their official URLs exists) and the
  fifth is a 2021 capture of its official URL, the 1998 and 1999 Gazette files are Internet Archive
  copies of eGazette files (eGazette serves no file for those issues), and the Commission's 1999 order is a copy in the
  IFES Election Judgments database (the Commission's host answered HTTP 406). Each is recorded as such, with its item or
  entity; the compiler's summary and note in the 1999 copy are kept apart from the order's text.

### Date ledger

Each row is a separate dated fact with its own claim; two facts on one day stay two claims. "Observed" marks a holder's
`attested_on` claim, "organisation" a claim about the observation; every other claim never feeds a holder.

| Date | Events | Claims |
|---|---|---|
| 28 Dec 1989 | V. P. Singh: in office before the period | `in_rs_anand_sharma_vp_singh_president_of_janata_dal_19891228` (claim) |
| 14 Jul 1990 | Bommai: in office | `in_rs_pm_letter_to_bommai_president_janata_dal_19900714` (observed) |
| 28 Aug 1990 | Bommai: in office again | `in_rs_pm_answer_styles_bommai_president_janata_dal_19900828` (claim) |
| 29 Mar 1991 | Bommai: in office again | `in_gazette_welfare_resolution_bommai_president_janata_dal_19910329` (claim) |
| 26 Dec 1991 | Bommai: expulsion recalled | `in_ls_speaker_decision_ajit_singh_expelled_by_bommai_recalled_19911226` (claim) |
| 5 Feb 1992 | Ajit Singh: rival claim | `in_ls_speaker_decision_ajit_singh_endorsed_as_president_claimed_19920205` (claim) |
| 14 Feb 1992 | Bommai: in office again | `in_gazette_welfare_resolution_bommai_president_janata_dal_19920214` (claim) |
| 19 Jul 1992 | Bommai: expulsions recalled | `in_ls_speaker_decision_four_members_expelled_by_bommai_recalled_19920719` (claim) |
| 14 Jan 1993 | organisation: name and symbol frozen; organisation: two groups recognised ad hoc | `in_eci_janata_dal_symbol_and_name_frozen_order_19930114` (organisation), `in_eci_janata_dal_two_groups_interim_recognition_order_19930114` (organisation) |
| 17 Jan 1993 | organisation: table entry under dispute | `in_eci_on8e_table_entry_janata_dal_dispute_pending_19930117` (organisation) |
| 22 Jul 1993 | organisation: final order (Bommai group is the Janata Dal); organisation: Janata Dal (A) withdrawn | `in_eci_final_order_bommai_group_recognised_as_janata_dal_19930722` (organisation), `in_eci_final_order_janata_dal_a_interim_recognition_withdrawn_19930722` (organisation) |
| 23 Jul 1993 | organisation: table entry | `in_eci_notification_56_93_6_table_entry_janata_dal_19930723` (organisation) |
| 5 Feb 1996 | organisation: table row ('Janta Dal') | `in_eci_19960205_np_row_05_janta_dal` (organisation) |
| 15 Jul 1996 | Laloo Prasad Yadav: in office | `in_ls_panigrahi_laloo_prasad_yadav_party_president_19960715` (observed) |
| 14 Oct 1996 | Laloo Prasad Yadav: in office again (heading) | `in_gazette_hrd_resolution_laloo_under_presidents_of_major_parties_19961014` (claim) |
| 17 Mar 1997 | Sharad Yadav: working President | `in_ls_ram_naik_sharad_yadav_working_president_19970317` (claim) |
| 22 Apr 1997 | Laloo Prasad Yadav: in office again | `in_ls_sushma_swaraj_your_president_laloo_prasad_yadav_19970422` (claim) |
| 29 Jul 1997 | Sharad Yadav: in office; Sharad Yadav: in office; Laloo Prasad Yadav: Rashtriya Janata Dal office | `in_ls_ram_naik_party_president_sharad_yadav_19970729` (observed), `in_ls_adsul_democratically_elected_president_sharad_yadav_19970729` (observed), `in_ls_virendra_kumar_singh_laloo_rjd_party_president_19970729` (claim) |
| 5 Aug 1997 | unnamed: President of the JD | `in_rs_som_pal_president_of_jd_charge_framed_19970805` (claim) |
| 29 Sep 1997 | Sharad Yadav: in office again (heading) | `in_gazette_hrd_resolution_sharad_yadav_under_presidents_of_major_parties_19970929` (claim) |
| 10 Jan 1998 | organisation: table row 6 (the observation) | `in_eci_19980110_np_row_06` (organisation) |
| 21 Jul 1999 | Sharad Yadav: removal claimed | `in_eci_dispute_1999_pac_removal_of_sharad_yadav_claimed_19990721` (claim) |
| 22 Jul 1999 | Deve Gowda: rival claim (application) | `in_eci_dispute_1999_deve_gowda_application_as_president_19990722` (claim) |
| 29 Jul 1999 | Sharad Yadav: endorsement claimed | `in_eci_dispute_1999_national_executive_endorsement_claimed_19990729` (claim) |
| 7 Aug 1999 | organisation: recognised National party; Sharad Yadav: the Commission's records; organisation: both groups recognised ad hoc; organisation: compiler's note naming JD(U) and JD(S) | `in_eci_dispute_1999_janata_dal_recognised_national_party_chakra_19990807` (organisation), `in_eci_dispute_1999_commission_records_sharad_yadav_president_19990807` (claim), `in_eci_dispute_1999_ad_hoc_recognition_of_both_groups_19990807` (organisation), `in_eci_dispute_1999_compiler_note_jdu_and_jds_named` (organisation) |
| 9 Aug 1999 | organisation: name and symbol under dispute; organisation: JD(S) and JD(U) rows added | `in_eci_on28e_janata_dal_name_and_symbol_under_dispute_19990809` (organisation), `in_eci_on28e_janata_dal_secular_and_united_rows_19990809` (organisation) |
| undated | unnamed: recalled act (the Prime Minister as President); Bommai: 1990 composition list; organisation: Janata Dal (A) and (B) named (16 or 17 Jan); Laloo Prasad Yadav: recalled act; Bommai: own recollection (1989); Deve Gowda: rival election claimed; Bommai: retrospective span 1990-1996 | `in_rs_raj_mohan_gandhi_pm_wrote_as_janata_dal_president_recalled_1990`, `in_ls_nic_statement_lists_bommai_president_janata_dal_1990_composition`, `in_eci_janata_dal_a_and_b_named_further_order_199301`, `in_rs_joyanta_roy_laloo_as_president_of_janata_dal_cmp_recalled`, `in_rs_bommai_as_janata_dal_president_report_1989_recalled`, `in_eci_dispute_1999_pac_elected_deve_gowda_president_claimed`, `in_rs_obituary_bommai_president_all_india_janta_dal_1990_to_1996` (no structured date) |

Date conventions follow the stacked India packets: `attested_on` is the day of the observed event as the source dates it,
a Gazette's issue date is `published_date`, and a recollection that recalls a day is dated by that day; spans, year-only
statements, lists of an earlier composition, undated recollections and a day printed differently in two language texts
carry no structured date.

## Observations

### JD-PRES-01 — The President when the period opens

Evidence: on 28 December 1989, in the Rajya Sabha's debate on the President's Address, Shri Anand Sharma said of Shri
Vishwanath Pratap Singh: 'Today he; is the Prime Minister of the country. He is the President of the Janata Dal. He is the
Convenor of the; National Front.' (so printed) On 18 May 1990, in the reference on the countermanding of the Meham bye-election, Shri Raj Mohan Gandhi
said that 'The Prime Minister in his capacity as President of the Janata dal, wrote to the Election Commissioner after the
first terrible incident took place'. The first record is before the period; the second recalls an undated act and names
nobody. No Gazette, Rajya Sabha or Lok Sabha record found attests the Janata Dal's President on a single day between 1
January and 13 July 1990, or states the day the office passed to S. R. Bommai.

Decision: **Declined.** V. P. Singh has no holder observation in the period. The prime-ministership is a separate office and
never supplies a name to this role.

### JD-PRES-02 — S. R. Bommai

Evidence: answering Unstarred Question 2406 on 28 August 1990, the Prime Minister enclosed his two letters of 14 July 1990,
one addressed to 'Shri S. R. Bommai, President, Janata Dal', asking him 'to call a meeting of the National Front
Parliamentary Party to elect a new leader'. The Ministry of Welfare's resolution of 29 March 1991 lists 'Shri S. R. Bommai,
President, Janata Dal' among the members of the Ambedkar centenary committee (and V. P. Singh without any party office). The
Lok Sabha statement of 29 August 1991 on the National Integration Council lists him from the 1990 composition. On 26 August
1997 Bommai recalled a 1989 report made 'As Janata Dal President'.

Decision: **Accepted in part.** Observed on 14 July 1990, the day of the letter; the later records are continuations or a
list of an earlier date, and his 1997 recollection is a claim. No start.

### JD-PRES-03 — The split of 1992-1993

Evidence: the Welfare resolution of 14 February 1992 lists 'Shri S.R. Bommai, President Janata Dal'. The Speaker's decision
of 1 June 1993 (Lok Sabha Secretariat notification S.O. 350(E)) summarises the written statement that Shri Ajit Singh was
expelled 'By Shri S. R. Bommai, President of the Janata Dal' on 26-12-1991 and four members 'on 19-7-1992', and the
respondents' claim that 'on 5-2-1992, the Janata Dal was split and Shri Ajit Singh was endorsed as the President of the
Party'. The Commission's O.N. 8(E) of 17 January 1993 records its order of 14 January 1993 freezing the symbol 'Chakra
(Wheel)' and the name, recognising the two groups 'headed by S/Shri Ajit Singh and S. R. Bommai' ad hoc, and the further
order naming them the Janata Dal (A) and (B). Its notification No. 56/93(6) of 23 July 1993, republished in the Goa
Gazette of 9 September 1993, records the final order of 22 July 1993: 'the group represented by Shri S. R. Bommai which was
granted interim recognition as Janata Dal (B) is hereby recognised as the Janata Dal', and the Janata Dal (A)'s interim
recognition withdrawn.

Decision: **Accepted in part.** Claims only; the office's holder observations do not change. The Commission's orders are
claims about the organisation; the rival claim of 1992 never feeds the role.

### JD-PRES-04 — Laloo Prasad Yadav, and Bommai's end

Evidence: on 15 July 1996 Shri Sriballav Panigrahi, speaking in English in the Lok Sabha, quoted 'the Janata Dal Supremo,
the Bihar Chief Minister Shri Laloo Prasad Yadav' and said 'The Janata Dal Party Supremo, the party President, the Bihar
Chief Minister had taken the Prime Minister to task'. The Human Resource Development resolution of 14 October 1996 lists
'Shri Laloo Prasad Yadav, Janata Dal' under 'Presidents of all Major National Political Parties'. On 22 April 1997 Shrimati
Sushma Swaraj spoke of 'your president and kingmaker Shri Laloo Prasad Yadav'. On 29 July 1997 a member said that the
'Rashtriya Janata Dal is functioning under the leadership of Laloo Yadav ji', its 'party president'. On 5
August 1997 a Rajya Sabha member recalled that 'as President of the Janata Dal' he had helped to form the United Front's
Common Minimum Programme. The Rajya Sabha's obituary synopsis of 15 November 2007 says Bommai 'was President of the All
India Janta Dal from 1990 to 1996'.

Decision: **Accepted in part.** Observed on 15 July 1996. No start: no record of his election or of the day he took the
office; no end, and his Rashtriya Janata Dal presidency is another organisation's office. Bommai's end is not recorded:
the span is retrospective and Laloo Prasad Yadav's observation is not used as it.

### JD-PRES-05 — Sharad Yadav

Evidence: on 17 March 1997 Shri Ram Naik said (translation) that 'Shri Sharad Yadav is the working President of Janta Dal'.
On 29 July 1997 Shri Ram Naik spoke of 'the Leader of the House, Shri Ram Vilas Paswan and party president, Shri Sharad
Yadav', and Shri Anandrao Vithoba Adsul of 'Democratically elected president Shri Sharad Yadav' (translation). On 5 August
1997 Shri Som Pal said in the Rajya Sabha that 'the President of the JD has been formally charge-framed by the court',
without a name. The Human Resource Development resolution of 29 September 1997 lists 'Shri Sharad Yadav, Janata Dal' under
'PRESIDENTS OF ALL MAJOR NATIONAL POLITICAL PARTIES'.

Decision: **Accepted in part.** Observed on 29 July 1997, from the two same-day attestations. The working presidency is a
claim only, never a holder, and 'Democratically elected' dates no election. No start.

### JD-PRES-06 — The dispute of 1999

Evidence: the Commission's order of 7 August 1999 in Dispute Case No. 1 of 1999: 'As per the Commission's records, Shri
Sharad Yadav is the President of the party'; Shri Deve Gowda's application of 22.7.99 for the symbol for 'the group of the
JD represented by him as the President'; the Political Affairs Committee's claimed decision of 21.7.99 'to remove Shri
Yadav from the post of party President' and its claimed election of Shri Deve Gowda; the claimed endorsement of Sharad
Yadav by the National Executive on 29 July 1999; and the Commission's finding that 'the party has split vertically', with
'provisional and ad-hoc recognition to both the rival groups as National parties' and neither group allowed the name or
the symbol until further orders.

Decision: **Accepted in part.** Claims only. The Commission decided nothing about the office: Sharad Yadav has no end and
Deve Gowda no holder observation.

### JD-PRES-07 — The organisation observation

Evidence: Table I of O.N. 18(E) of 10 January 1998 lists '6. Janata Dal, Chakra (Wheel), 7, Jantar Mantar Road, New
Delhi-110001'. Table I of O.N. 11(E) of 5 February 1996 lists '5. Janta Dal' (so printed) with the same symbol and address,
and the notification of 23 July 1993 restores the single entry '5. Janata Dal Chakra (Wheel)'.

Decision: **Accepted.** The new observation `in_eci_19980110_np_06` is a recognition-row observation of 10 January 1998
only: it asserts no founding, dissolution, lifespan, successor, leadership or game identity.

### JD-PRES-08 — The name after the 1999 split

Evidence: the Commission's O.N. 28(E) of 9 August 1999 keeps entry 6, 'Janata Dal, Chakra (Wheel)', with the note 'The name
of the party and the symbol under dispute - symbol not to be allotted until further orders', and adds '6(A) Janata Dal
(Secular), Kisan Driving Tractor' and '6(B) Janata Dal (United), Arrow'. The compilation printing the order of 7 August
1999 notes that 'By subsequent order dated 07.08.1999' the Commission recognised the group led by Sharad Yadav as the
Janata Dal (United) and the group led by Deve Gowda as the Janata Dal (Secular).

Decision: **Accepted in part.** Both groups are separately recognised organisations and claims about this one only; no
identity, lifecycle or leader passes between them, and no holder of this office is tied to the Janata Dal name after 7
August 1999. Later orders on the frozen name were not found.

## Sources added

| Source ID | What | Provenance |
|---|---|---|
| `in_rs_debate_19891228_motion_of_thanks` | Rajya Sabha Debates, 28 December 1989 (Session 152): Motion of Thanks on the President's Address (contd.), cols 202-356 (store file ID_152_28121989_01_p202-356_1.pdf) | official file, Rajya Sabha debates store (handle 123456789/258711); PDF page 10 rendered or read |
| `in_rs_debate_19900518_meham_countermanding` | Rajya Sabha Debates, 18 May 1990 (Session 154): Reference to the countermanding of bye-election in Meham due to murder of a candidate, cols 170-242 (store file ID_154_18051990_01_p170-242_1.pdf) | official file, Rajya Sabha debates store (handle 123456789/251375); PDF pages 18, 19 rendered or read |
| `in_rs_written_answers_19900828_usq2406_pm_letters` | Rajya Sabha Debates, 28 August 1990 (Session 155): Written Answers, Unstarred Question 2406 (letters of 14 July 1990), cols 181-183 (store file IQ_155_28081990_U2406_p181-183.pdf) | official file, Rajya Sabha debates store (handle 123456789/254970); PDF pages 1, 2 rendered or read |
| `in_gazette_welfare_resolution_19910329` | The Gazette of India, Extraordinary, Part I - Section 1, No. 76, New Delhi, Friday 29 March 1991: Ministry of Welfare (Centenary Cell) Resolution No. 1/3/91-CC.I of 29 March 1991 (National Committee for the Centenary Celebrations of Baba Saheb Dr. Bhim Rao Ambedkar) | official file, eGazette (equal to the Internet Archive item `in.gazette.central.e.1991-03-29.21565`); PDF pages 1, 9, 10 rendered or read |
| `in_ls_written_answers_19910829_usq5063_nic` | Lok Sabha Debates, 29 August 1991 (Tenth Lok Sabha): Written Answers, Unstarred Question 5063 'National Integration Council', cols 305-307 (Parliament Digital Library file 10_I_29081991_p158_p162_t116.pdf) | mirror-only copy of a Parliament Digital Library file, Internet Archive item `eparlib.nic.in.17679`; PDF page 1 rendered or read |
| `in_gazette_welfare_resolution_19920214` | The Gazette of India, Extraordinary, Part I - Section 1, No. 33, New Delhi, Friday 14 February 1992: Ministry of Welfare (Centenary Cell) Resolution No. 1/3/91-CC.I of 14 February 1992 | official file, eGazette (equal to the Internet Archive item `in.gazette.central.e.1992-02-14.19183`); PDF pages 6, 9 rendered or read |
| `in_eci_on8e_19930117_janata_dal_dispute` | The Gazette of India, Extraordinary, No. 8, New Delhi, Monday 18 January 1993: Election Commission of India notification O.N. 8(E) of 17 January 1993, No. 56/93(1) (dispute relating to Janata Dal) | official file, eGazette (equal to the Internet Archive item `in.gazette.central.e.1993-01-18.17812`); PDF pages 1, 2 rendered or read |
| `in_gazette_ls_speaker_decision_19930601` | The Gazette of India, Extraordinary, Part II - Section 3 - Sub-section (ii), No. 320, New Delhi, Tuesday 1 June 1993: Lok Sabha Secretariat notification S.O. 350(E), decision of the Speaker, Lok Sabha, of 1 June 1993 under the Tenth Schedule (Janata Dal Legislature Party) | official file, eGazette (equal to the Internet Archive item `in.gazette.central.e.1993-06-01.18126`); PDF pages 21, 23, 33 rendered or read |
| `in_goa_gazette_eci_notification_56_93_6_19930909` | Official Gazette, Government of Goa, Series I No. 24, Panaji, 9 September 1993, pp. 465-466: Law Department notification 3-1-87/ELEC-Vol. II of 24 August 1993 republishing Election Commission of India Notification No. 56/93(6) of 23 July 1993 (dispute relating to the Janata Dal) | stored Gazette copy, Internet Archive item `in.goa.egaz.9394-24.SI`; PDF pages 1, 7, 8 rendered or read |
| `in_eci_on11e_19960205_national_parties` | The Gazette of India, Extraordinary, Part II - Section 3 - Sub-section (iii), No. 4, New Delhi, Monday 5 February 1996: Election Commission of India notification O.N. 11(E) of 5 February 1996 (No. 56/96/Jud.II), Table I, National Parties | official file, eGazette (equal to the Internet Archive item `in.gazette.central.e.1996-02-05.11878`); PDF page 34 read from the text layer |
| `in_ls_debate_19960715_petroleum_prices` | Lok Sabha Debates, 15 July 1996 (Eleventh Lok Sabha): Discussion under Rule 193, steep pre-budget hike in the administered prices of petrol, LPG, diesel and other petroleum products (contd.), cols 263-264 (Parliament Digital Library file 11_II_15071996_p126_p145_t195.pdf) | mirror-only copy of a Parliament Digital Library file, Internet Archive item `eparlib.nic.in.10951`; PDF page 11 rendered or read |
| `in_gazette_hrd_resolution_19961014` | The Gazette of India, Extraordinary, Part I - Section 1, New Delhi, Friday 25 October 1996: Ministry of Human Resource Development (Department of Culture) Resolution F. No. 38-1/95-C & M of 14 October 1996 (National Committee for the commemoration of the 50th Anniversary of India's Independence) | official file, eGazette (equal to the Internet Archive item `in.gazette.central.e.1996-10-25.10568`); PDF pages 5, 7 rendered or read |
| `in_ls_debate_19970317_environment_appellate_authority` | Lok Sabha Debates, 17 March 1997 (Eleventh Lok Sabha): Statutory Resolution re disapproval of the National Environment Appellate Authority Ordinance, 1997 and the National Environment Appellate Authority Bill, 1997, cols 257-258 (Parliament Digital Library file 11_IV_17031997_p129_p138_t282.pdf) | mirror-only copy of a Parliament Digital Library file, Internet Archive item `eparlib.nic.in.10363`; PDF page 5 read from the text layer |
| `in_ls_debate_19970422_confidence_motion` | Lok Sabha Debates, 22 April 1997 (Eleventh Lok Sabha): Motion of Confidence in the Council of Ministers (contd.), cols 17-18 (Parliament Digital Library file 11_IV_22041997_p9_p23_t14.pdf) | raw Internet Archive capture (2 December 2021) of the official eparlib.nic.in file; PDF page 6 rendered or read |
| `in_ls_debate_19970729_bihar_situation` | Lok Sabha Debates, 29 July 1997 (Eleventh Lok Sabha): Situation arising out of the recent developments in Bihar (contd.), cols 443-446 (Parliament Digital Library file 11_V_29071997_p204_p237_270.pdf) | mirror-only copy of a Parliament Digital Library file, Internet Archive item `eparlib.nic.in.8795`; PDF pages 23, 24 rendered or read |
| `in_rs_debate_19970805_bihar_situation` | Rajya Sabha Debates, 5 August 1997 (Session 181): Short Duration Discussion on the prevailing situation in Bihar (contd.), cols 239-310 (store file ID_181_05081997_01_p239-310_1.pdf) | official file, Rajya Sabha debates store (handle 123456789/134942); PDF pages 5, 24 read from the text layer |
| `in_rs_debate_19970826_human_development_discussion` | Rajya Sabha Debates, 26 August 1997 (Session 181): Discussion on Human Development and Science and Technology, cols 72-220 (store file ID_181_26081997_01_p72-220_1.pdf) | official file, Rajya Sabha debates store (handle 123456789/135374); PDF pages 40, 41 read from the text layer |
| `in_gazette_hrd_resolution_19970929` | The Gazette of India, Extraordinary, Part I - Section 1, No. 194, New Delhi, Wednesday 1 October 1997: Ministry of Human Resource Development (Department of Culture) Resolution No. F. 29-2/97-C&M of 29 September 1997 (National Committee to observe the 50th Anniversary of the Martyrdom of Mahatma Gandhi) | official file, eGazette (equal to the Internet Archive item `in.gazette.central.e.1997-10-01.8273`); PDF pages 4, 5 read from the text layer |
| `in_eci_on18e_19980110_national_parties` | The Gazette of India, Extraordinary, Part II - Section 3 - Sub-section (iii), New Delhi, Thursday 15 January 1998: Election Commission of India notification O.N. 18(E) of 10 January 1998, Table I, National Parties | stored Gazette copy, Internet Archive item `in.gazette.central.e.1998-01-15.7285`; PDF pages 82, 83 rendered or read |
| `in_eci_dispute_case_1_of_1999_order_19990807` | Election Commission of India, order of 7 August 1999 in Dispute Case No. 1 of 1999 (application of Shri H.D. Deve Gowda under paragraph 15 of the Election Symbols (Reservation and Allotment) Order, 1968), compilation pages 329-338 (IFES Election Judgments file 16008740057920ye3h4iaksq.pdf) | third-party repository copy (IFES Election Judgments) of an Election Commission order; PDF pages 1, 2, 3, 4, 5, 6, 7, 8, 9, 10 read from the text layer |
| `in_eci_on28e_19990809_janata_dal_dispute` | The Gazette of India, Extraordinary, Part II - Section 3 - Sub-section (iii), No. 24, New Delhi, Monday 9 August 1999: Election Commission of India notification O.N. 28(E) of 9 August 1999 (No. 56/99/Jud.-III) | stored Gazette copy, Internet Archive item `in.gazette.central.e.1999-08-09.4826`; PDF pages 1, 2, 3 rendered or read |
| `in_rs_synopsis_20071115_obituary_bommai` | Rajya Sabha, Synopsis of Debates, 15 November 2007 (Session 212): Obituary references (Shri S.R. Bommai) | official file, Rajya Sabha CMS; PDF page 5 read from the text layer |

## Response identities and stability checks

The reviewer re-downloads every recorded response and compares its byte count and SHA-256, so every identity here was
downloaded at least three times with identical bytes, the first and last at least 30 minutes apart, and no identity is a page
generated per request, a growing listing or a search. Every download used curl with its default User-Agent and the request
header `Accept-Encoding: identity`, and every body was served with no Content-Encoding, so each identity is the
uncompressed body. How each was established:

- **The Rajya Sabha Secretariat's debates store (5).** Static objects on `bucketapi.rajyasabha.digital`, the host the
  stacked India packets already record; each ETag equals the MD5 of the body, and the query sets only the served content
  type and file name. They were located through the Secretariat's data service, whose year listings are not identities.
- **The Rajya Sabha CMS (1).** The synopsis of 15 November 2007, a static file with a fixed ETag.
- **eGazette (7).** Static `egazette.gov.in` files (Last-Modified 12 March 2013, fixed ETag, PDF CreationDate in 2013 and no
  ModDate); each file's MD5 equals the stored file of the Internet Archive item of the same issue (the
  `in.gazette.central.e.*` copies, not the different `csl_extraordinary` derivatives).
- **Internet Archive stored copies (7).** `archive.org/download/<item>/<file>` redirects to a storage node, and the final
  body is the recorded identity; each MD5 equals the item's file listing. Two are Gazette of India issues of 15 January 1998
  and 9 August 1999 (Public.Resource.Org copies of eGazette files, which eGazette no longer serves), one is the Goa Gazette
  of 9 September 1993, and four are Parliament Digital Library files of the Lok Sabha debates (uploads of July 2025 whose
  metadata names the eparlib record), recorded with mirror-only provenance because the eparlib hosts could not be reached;
  three of those four PDFs were made with ReportLab on 29 January 2024, so they may be later renderings of the official
  files, whose bytes could not be compared.
- **A raw Internet Archive capture (1).** The Lok Sabha file of 22 April 1997 is the `id_` capture of its official
  eparlib.nic.in URL made on 2 December 2021, fetched with `Accept-Encoding: identity` and without `--compressed`; the
  Internet Archive item of the same record holds a different 15,195,535-byte rendering, which is not used. Its first
  download of the capture was later than the other sources' first downloads, so its times are its own.
- **IFES Election Judgments (1).** The Commission's order of 7 August 1999 as a static PDF (CreationDate 10 July 2002),
  uploaded on 23 September 2020; a third-party repository copy, disclosed as such.
- **Times.** Every recorded response was downloaded on 28 September 2026 at the times below, the first and last at least 30
  minutes apart. The large copies were deleted from scratch after hashing.

| Source | Bytes | SHA-256 | Downloads with identical bytes (28 Sep 2026) |
|---|---:|---|---|
| `in_rs_debate_19891228_motion_of_thanks` | 3,544,025 | `3fdddf91d8e022da393d663f2061e738227ad6fb48a71103d46c8b1bc61644c2` | 19:29:03Z, 20:00:37Z, 20:33:28Z |
| `in_rs_debate_19900518_meham_countermanding` | 815,707 | `6dd6c4707d9adb96ed96251d70c9c4ab32f46d8971aac9e1c916f1da11774e50` | 19:29:13Z, 20:00:43Z, 20:33:38Z |
| `in_rs_written_answers_19900828_usq2406_pm_letters` | 105,687 | `94845f4a2e27e15432cdb79e9a69dfe532554ef0a584254407725ec0f8840788` | 19:29:11Z, 20:00:42Z, 20:33:36Z |
| `in_gazette_welfare_resolution_19910329` | 663,246 | `db48f4f7c1d139680cb394cdd8cf0df75ac3c519bd97f171f9b65a4440783fd5` | 19:28:01Z, 19:59:30Z, 20:32:09Z |
| `in_ls_written_answers_19910829_usq5063_nic` | 312,005 | `ee5547287325ffcad6b34ec2acab1e78ff4fe162050879f93859780e2077ddf1` | 19:28:48Z, 20:00:23Z, 20:33:13Z |
| `in_gazette_welfare_resolution_19920214` | 424,598 | `59b93fa4f3702717f3246ca28b74ba7c61d6cbfe4211f7d5b53a4d094edb5f79` | 19:28:10Z, 19:59:38Z, 20:32:19Z |
| `in_eci_on8e_19930117_janata_dal_dispute` | 104,268 | `1d0a0b07556664d8c2cdca5cd2024d1374a8b269949f8900c6014fd7d45a7b32` | 19:27:23Z, 19:58:47Z, 20:31:33Z |
| `in_gazette_ls_speaker_decision_19930601` | 2,879,963 | `d0ceaea1a826bd8e4f0808ca458be7eaf6d866c9941ad176a03d833e8698d59f` | 19:28:16Z, 19:59:45Z, 20:32:25Z |
| `in_goa_gazette_eci_notification_56_93_6_19930909` | 549,444 | `27990864cd1bf4f60d8df443c297ffa8b7f8763a7826275fb22e72e0888e463a` | 19:27:25Z, 19:58:49Z, 20:31:38Z |
| `in_eci_on11e_19960205_national_parties` | 2,694,875 | `0189f4e8e71fde7fcd11bdc1f2a86818e79d7d804542405e511a4c27f83677c8` | 19:27:27Z, 19:58:52Z, 20:31:40Z |
| `in_ls_debate_19960715_petroleum_prices` | 13,860,933 | `ca995bb48dc8b52a998694c8454ea1850afbd4f80162f324203b23985fa8bf59` | 19:28:49Z, 20:00:24Z, 20:33:14Z |
| `in_gazette_hrd_resolution_19961014` | 435,535 | `11eab2c61a70181f0120f195210c8e33c3d2c81f936018fc1de6764e16aeb07f` | 19:28:38Z, 20:00:11Z, 20:33:05Z |
| `in_ls_debate_19970317_environment_appellate_authority` | 8,195,221 | `f8ce7f3ea2f80086fc8bf4a7ff5a9a46e8fba0d7417b80331ffde6c0f09a664c` | 19:28:51Z, 20:00:27Z, 20:33:18Z |
| `in_ls_debate_19970422_confidence_motion` | 937,125 | `1cb7dcbb11e90f68af7cf8109ef4459e9483963676f627b22a2a170e97c3f160` | 20:20:07Z, 20:33:20Z, 20:51:03Z |
| `in_ls_debate_19970729_bihar_situation` | 30,276,934 | `735dc385eb052fcb14cafb895da33eddd50d48bddf52995b968565e01898be65` | 19:28:57Z, 20:00:31Z, 20:33:21Z |
| `in_rs_debate_19970805_bihar_situation` | 449,285 | `19d3c3f03f9fba7ba4753b7c52889ca9ddeff5f1143b0fbfccda237a3e279192` | 19:29:26Z, 20:00:47Z, 20:33:41Z |
| `in_rs_debate_19970826_human_development_discussion` | 753,976 | `8ff848856e1583368e978ab29200306c7a0ffc38dd6e8540a00d4b9fdb5e4fe2` | 19:29:22Z, 20:00:49Z, 20:33:42Z |
| `in_gazette_hrd_resolution_19970929` | 186,543 | `eac780ae235103cfb1c2e917b7b166cf21827ffbb731e182eafc71399404ec6e` | 19:28:44Z, 20:00:19Z, 20:33:11Z |
| `in_eci_on18e_19980110_national_parties` | 4,063,921 | `2e68126f22e1540315405b8b5ee978c35e6bcc4a7f2f23629c236b2d2e5b7744` | 19:27:54Z, 19:59:23Z, 20:32:03Z |
| `in_eci_dispute_case_1_of_1999_order_19990807` | 28,440 | `2fa846930f971347601f0f0719954f2d49bc717ad5d54e68e6349fb92e1cb404` | 19:28:00Z, 19:59:29Z, 20:32:08Z |
| `in_eci_on28e_19990809_janata_dal_dispute` | 107,430 | `2c2f57244d870630f325d3e3ec287e76209cc4b0f798c495e64b63f234150164` | 19:27:57Z, 19:59:27Z, 20:32:05Z |
| `in_rs_synopsis_20071115_obituary_bommai` | 103,683 | `712056ec84df5a2623fa2b46b4f6c8293139ca83ff2411055e67a25decd3e4c7` | 19:29:01Z, 20:00:35Z, 20:33:26Z |

## Leads not imported

- `sikhheritageeducation.com/vp-to-give-up-party-post/` (the gap ledger's lead for the 1990 handover): a news clipping, a
  lead only.
- `business-standard.com` article of 1 July 1997, 'Gujral may take over as Dal president' (the gap ledger's lead): news, a
  lead only.
- Rajya Sabha, 4 October 1990 (store file `ID_155_04101990_01_p89-212_1.pdf`): the Internet Archive's own OCR of this record
  (item rsdebate.nic.in.250440) reads 'The ruling party President, Mr. Bommai said that he would do padyatra in Kashmir',
  but the official file's text layer does not contain the passage (a Hindi page) and it was not rendered; not imported.
- Rajya Sabha, 15 July 1996 (`ID_178_15071996_01_p180-202_1.pdf`): 'The Working President of the Janata Dal has already
  condemned the statement', naming nobody; the named working presidency of 17 March 1997 is recorded instead.
- Rajya Sabha, 9 April 1990 (`ID_153_09041990_01_p10-66_1.pdf`, 'the then Janata Dal President, Mr. V. P. Singh', about
  1988) and 29 August 1990 (`ID_155_29081990_01_p315-378_1.pdf`, 'Mr. V. P. Singh, as President of the Janata Dal ... made
  various remarks', about the time before he became Prime Minister): recollections of periods before 1990.
- Gazette of India, 23 December 1996 (`E-0297-1996-0220-10496`): the resolution of 14 December 1996 listing 'Shri Sharad
  Yadav, Janata Dal' among the 'Leaders of the Political Parties / Groups in Lok Sabha', a parliamentary office outside
  this role.
- Gazette of India, 6 May 1992 (`E-0492-1992-0085-19235`): the Welfare resolution extending the committee, which repeats
  'Shri S. R. Bommai, President Janata Dal' from the list of 14 February 1992; a duplicate continuation.
- Lok Sabha, 25 July 1996 (`eparlib.nic.in.5888`): a member reports that the Bihar Chief Minister said in the Vidhan Sabha
  'Day before yesterday' that he was the National President of the Janata Dal; second-hand, a relative day, and names
  nobody.
- Lok Sabha, 11 April 1997 (`eparlib.nic.in.9786`): 'inquiry is being conducted against the President of Janata Dal',
  naming nobody.
- Lok Sabha, 27 February 1992 (`eparlib.nic.in.3579`, the National Integration Council list with 'Shri S. R. Bommai
  President, Janata Dal') and 16 July 1992 (`eparlib.nic.in.3670`, 'alongwith Shri Bommai, President of my party'):
  continuations of a holder already observed, in large mirror-only files; not imported.
- Lok Sabha, 27 May 1996 (`eparlib.nic.in.6036`): 'the president of Janata Dal Shri S.R. Bomai' in a translated list of
  names from the hawala diaries, whose tense is unclear.
- Lok Sabha Secretariat, 'Parliament of India: Tenth Lok Sabha, 1991-1996' (`eparlib.nic.in.56238`, 1997): a retrospective
  account of 'two letters from the President of the Janata Dal, Shri S.R. Bommai, intimating the expulsion'.
- Gazette of India, 15 February 2001 (`in.gazette.central.e.2001-02-15.113003`): 'Shri Sharad Yadav, President, Janata Dal
  (United)' and 'Shri Laloo Prasad Yadav, President, Rashtriya Janata Dal', offices of other organisations.
- The Commission's principal notification No. 56/99/Jud.III of 30 July 1999 (Internet Archive item
  in.gazette.central.e.1999-07-30.4818), which O.N. 28(E) amends: not downloaded; the amended entry is recorded.
- The `csl_extraordinary` Internet Archive items of the same Gazette issues: different renderings, not used.
- The Internet Archive item copy of the Lok Sabha file of 22 April 1997 (`archive.org/download/eparlib.nic.in.6598`,
  15,195,535 bytes, SHA-256 `6509b2cc6a13...`, made with ReportLab in 2024): a different rendering of the record whose 2021
  capture of the official URL is recorded instead.
- The Internet Archive's mirrors of the Rajya Sabha debates (`rsdebate.nic.in.*` items) and of the Lok Sabha debates were
  used only to find records by full-text search; the Rajya Sabha records are recorded from the Secretariat's own store.
- Wikipedia and news articles were not used.

## Sources attempted

- The Parliament Digital Library (`eparlib.sansad.in`, `eparlib.nic.in`) and `loksabhadocs.nic.in`: connection timeouts on
  28 September 2026, so no Lok Sabha file is recorded from an official host.
- `www.eci.gov.in`: HTTP 406 to curl's default request; not retried with another User-Agent. No official copy of the
  Commission's 1999 order or of its later orders on the frozen name was found.
- The Wayback Machine (`web.archive.org`, including its CDX index): 'Temporarily Offline' (HTTP 503) for much of 28
  September 2026. When it answered, CDX queries for the five eparlib files found a capture only for record 6598 (2 December
  2021), which is recorded; records 17679, 10363, 10951 and 8795 have no capture on `eparlib.nic.in` or
  `eparlib.sansad.in`.
- eGazette: no file for the 1998 and 1999 issues at any `WriteReadData`/`writereaddata` path tried (HTTP 404); the Internet
  Archive's stored copies are recorded.
- The Government Printing Press, Goa (`goaprintingpress.gov.in` gazette record 4417): HTTP 301, not followed; the Goa
  Gazette is recorded from the Internet Archive item that links it.
- Discovery only, never a recorded identity: the Rajya Sabha data service's year listings (8,810 debate files of 1990-1999
  were downloaded, their text layers searched and the files deleted), the Internet Archive's advanced and full-text search,
  and the IFES database's search API.

No site terms, licences or cookie banners were accepted, no CAPTCHA or challenge was attempted, and no login was used.

## Suggested next work orders

1. When the Parliament Digital Library is reachable, record the Lok Sabha files from the official host and search the
   debates of January-July 1990 for V. P. Singh's presidency and the day it passed to S. R. Bommai.
2. The Election Commission's own copies of the order of 7 August 1999 and of its separate order naming the Janata Dal
   (United) and the Janata Dal (Secular), and of later orders on the frozen name and symbol.
3. The Janata Dal's own records (National Executive and National Council resolutions, election returns) for the elections
   of 1996 and 1997 and the days each President took and left the office.
4. The split of November 1990 (the Janata Dal (Socialist)): the Speaker's decision and the Commission's records.
5. Separate observations and leadership roles for the Janata Dal (Secular), the Janata Dal (United) and the Rashtriya
   Janata Dal, from their own rows, without inheriting this office.

## Integration notes (outside this packet's file boundary)

- **Stacking.** The branch was claimed at `89decb6a` on `claude/c01-in-27` merged with integration `3ec6e155` (merge commit
  `6fdb950e`). CLAUDE-C01-27 has since been integrated, and the branch merged current integration `032cd6a3` at `a266ebcb`
  (the only conflict was the generated research index, taken from integration and then regenerated), so nothing needs
  merging first; this packet adds only its own records on top of CLAUDE-C01-27's, whose content and test are unchanged
  apart from the re-expressed pins below.
- **Existing files edited.** `india.json` is edited by appending only: 22 sources after CLAUDE-C01-27's, one organization
  observation after the 82 recognition rows, and one note at the end of the packet coverage. No existing extract is edited.
  The pinned tests are re-expressed, never loosened: `test_india_research_s10e.py` (packet counts 85/297/599/5, 83
  organizations, a third organization role, C01-33 hosts and access date, the source order ending with the new
  observation's sources, mapping_pending 85 and a last India work order of five members including `in_eci_19980110_np_06`),
  `test_india_prime_ministers_c01_11.py`, `test_india_presidents_c01_15.py`, `test_india_inc_presidents_c01_20.py` and
  `test_india_bjp_presidents_c01_27.py` (the same counts, the source order extended by the new observation's sources, the
  party-leader list extended by `in_jd_president`, the BJP coverage note at index 10 with the new note after it, 12
  coverage notes and 5 roles; in the C01-11 test the set of other packets' rows also includes the new observation's).
- **Shared generated file.** `docs/campaign-certification/C01/research-index.json` is the only file shared with other
  pending packets; it is regenerated in its own commit and should be regenerated again after merging.
- **Gap ledger.** `tools/avatars/test_certified_gap_ledger.py` fails on this branch until Codex classifies the new commits
  and pins their attribution, as on every new packet branch (here it stops at the stacked CLAUDE-C01-27 source
  `in_bjp_elib_party_document_vol5_political_resolutions`, 'has no pinned attribution'); it is disclosed, not fixed, and
  `docs/campaign-certification/C01/gap-ledger/` is untouched.
- **S23 boundary matrix.** The S23 boundary-matrix test (`tools/avatars/test_certified_boundary_matrix.py`) needs
  `spheres-web/src`, which is absent from this sparse checkout (it stops at `spheres-web/src/person_avatar_assets.rs`), so
  it could not pass here; Codex must regenerate that matrix when it merges, because the India packet gains an organization
  observation and a role.
- **Campaign tests.** `test_campaign_census` and the other campaign tests run and pass (see Checks).

## Checks

Run from the worktree with `PYTHONDONTWRITEBYTECODE=1` on 28 September 2026, after merging integration `032cd6a3`:

```text
python -X utf8 tools/avatars/campaign_research.py
python -X utf8 tools/avatars/campaign_research.py --check
python -X utf8 tools/avatars/campaign_census.py --check
python -X utf8 -m unittest discover -s tools/avatars -p "test_india*.py"
python -X utf8 -m unittest discover -s tools/avatars -p "test_*research*.py"
python -X utf8 -m unittest discover -s tools/avatars -p "test_campaign*.py"
node --test tools/ui/check_leadership_research_review.cjs
python tools/planning/workboard.py --check
git diff --check
```

| Check | Result |
|---|---|
| `campaign_research.py`, then `--check` | passed: 9 country packets, 842 organization and 35 institution observations, 1,593 sources, 4,223 claims, 93 open discovery batches |
| `campaign_census.py --check` | **fails, inherited from integration:** 'C01 evidence differs: census.json', because integration commits `262d5f61` and `032cd6a3` (after `4a3d0572`) changed `spheres-sim/src/government.rs` without regenerating `docs/campaign-certification/C01/census.json`, whose production snapshot pins that file's SHA-256 (`0e2dbdef...`, now `3f846b4b...`). The other four C01 outputs regenerate identically; this packet touches no census input, and the check passed on the branch before that merge. Not fixed here (outside the file boundary); Codex should regenerate the census at integration. |
| `test_india*.py` | 57 passed, including the 11 tests of `test_india_janata_dal_presidents_c01_33.py` (its mutation test rejects 12 validator cases, 52 rule cases and 5 collapsed events) |
| `test_*research*.py` | 79 passed |
| `test_campaign*.py` | 16 passed |
| `check_leadership_research_review.cjs` | 11 passed |
| `workboard.py --check` | passed |
| `git diff --check` | clean |
| `test_certified_gap_ledger.py` (not required; disclosed) | fails until Codex classifies the new commits and pins their attribution ('has no pinned attribution'); not fixed, and the gap ledger is untouched |
| `test_certified_boundary_matrix.py` (S23; not required; disclosed) | cannot pass in this sparse checkout: 'Required input is missing: spheres-web/src/person_avatar_assets.rs'; Codex must regenerate the S23 boundary matrix when it merges |
