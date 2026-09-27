# Brazilian presidents 10: holders and transitions, 1990-2026

Packet: **CLAUDE-C01-10**. State: **ready_for_review** (not complete).
Owner: Claude. Integrator/reviewer: Codex. Branch `claude/c01-br-10`, based on `codex/campaign-certification`
at `ffe54b02`; claim commit `0d177d7d`. Research access: 23 September 2026 (local). The historical cutoff stays
**7 September 2026**.

This packet adds the Brazilian presidency to [brazil.json](brazil.json), which until now held only 31 party
observations and no institution. It reviews ten observations from 1 January 1990 to the cutoff: the President when
the period opens, Fernando Collor's diplomação and posse, the 1992 impeachment, resignation and succession, the posses
of 1995, 1999, 2003, 2007, 2011 and 2015, the 2016 impeachment and succession, the posses of 2019 and 2023, and one
official attestation of the President in office in 2026. It adds 48 sources and 112 claims, one institution
(`br_presidency`, an `executive_institution` with lifecycle `unknown`) with one role (`br_president`, "President of
the Federative Republic of Brazil", kind `head_of_state`), twelve holder observations, a role scope note, institution
coverage and a bounded note in the packet coverage. It adds no organization, game mapping, lifespan, portrait or
avatar, and it changes none of the 31 organization observations. The parent scope (C01, C06, S23, WC1 and CP1) remains
open.

The research was done in three parts (A: 1989-1992; B: 1995-2015; C: 2016-2026), and each part was checked
independently before this packet was written. Every checker defect is applied or explained below (see
[Checker defects](#checker-defects)).

## Outcome

| ID | Question | Decision |
|---|---|---|
| BR-PRES-01 | Who was President when the period opens, and does a source state the end of that term? | **Accepted in part:** José Sarney observed in office on 5 Jan 1990 (Decree No. 98.797) and still on 13 Mar 1990; no primary source states the day his term ended |
| BR-PRES-02 | Fernando Collor's diplomação and his posse of 15 March 1990 before the National Congress | **Accepted:** ballots of 15 Nov and 17 Dec 1989; TSE diploma of 30 Dec 1989; declared invested on 15 Mar 1990 for a period beginning that day (Collor's start) |
| BR-PRES-03 | 1992: authorization, trial opening, suspension, the Vice-President's exercise, resignation, posse and judgment | **Accepted:** Chamber 441-38 on 29 Sep; Senate opens the trial on 1 Oct; Collor suspended on receiving the summons at 10:20 on 2 Oct; Itamar Franco exercising from 2 Oct (claims only); resignation and vacancy on 29 Dec (Collor's end); Itamar Franco's posse that day (his start); disqualification for eight years voted 76-3 on 30 Dec |
| BR-PRES-04 | The posses of 1 January 1995 and 1999, and Itamar Franco's end only if a source states it | **Accepted in part:** both posses state a period beginning that day (Cardoso's two starts); Itamar Franco's end is stated only in Cardoso's thanks, a successor's statement that is not a boundary |
| BR-PRES-05 | The posses of 1 January 2003 and 2007 | **Accepted:** both posses state a period beginning that day (Lula's two starts); no term end is stated |
| BR-PRES-06 | The posses of 1 January 2011 and 2015 | **Accepted:** both posses state a period beginning that day (Rousseff's two starts), with her and the President of Congress's statements that she assumes office |
| BR-PRES-07 | 2016: authorization, trial opening, suspension, the Vice-President's exercise, removal and posse | **Accepted:** Chamber 367-137 on 17 Apr; Senate admits 55-22 on 12 May and Rousseff signs the intimation that day; Temer exercising from 12 May (claims only); loss of office 61-20 and Resolution No. 35 in force on publication on 31 Aug (Rousseff's end); Temer's posse that day (his start) |
| BR-PRES-08 | The posse of 1 January 2019, and Temer's end only if a source states it | **Accepted in part:** Bolsonaro declared invested for a period beginning that day, with the signed termo and the sash-ceremony address; Temer's end is not stated |
| BR-PRES-09 | The posse of 1 January 2023, and Bolsonaro's end only if a source states it | **Accepted in part:** Lula declared invested for a period beginning that day; TSE diploma of 12 Dec 2022; the Vice-President exercised the office on 30-31 Dec 2022 (claims only); Bolsonaro's end is not stated |
| BR-PRES-10 | An official attestation of the President in office before the cutoff, without extending any term | **Accepted:** MP 1.388 of 24 Aug 2026, signed by Lula as President |

The resulting holder observations of `br_president`, in date order:

| Holder | `attested_on` | `from` | `until` | Basis |
|---|---|---|---|---|
| José Sarney | 1990-01-05 | null | null | Decree No. 98.797, signed as President (Planalto, archived) |
| Fernando Collor de Mello | null | 1990-03-15 | 1992-12-29 | posse declared for the period beginning that day (DCN 8/1990); letter resigning "nesta data", vacancy declared and "renúncia ... ocorrida nesta data" (DCN 70/1992) |
| Itamar Franco | null | 1992-12-29 | null | posse declared for the period beginning that day (DCN 70/1992) |
| Fernando Henrique Cardoso | null | 1995-01-01 | null | posse declaration and termo (DCN 1/1995) |
| Fernando Henrique Cardoso | null | 1999-01-01 | null | posse declaration and termo (DCN 1/1999) |
| Luiz Inácio Lula da Silva | null | 2003-01-01 | null | posse declaration and termo (DCN 1/2003) |
| Luiz Inácio Lula da Silva | null | 2007-01-01 | null | posse declaration and termo (DCN 1/2007) |
| Dilma Rousseff | null | 2011-01-01 | null | posse declaration and termo (DCN 1/2011) |
| Dilma Rousseff | null | 2015-01-01 | 2016-08-31 | posse declaration and termo (DCN 1/2015); Resolution No. 35, in force on publication in the DOU extra edition of 31 Aug 2016 |
| Michel Temer | null | 2016-08-31 | null | posse declaration and termo "em virtude de vacância ocorrida" that day (DCN 15/2016) |
| Jair Bolsonaro | null | 2019-01-01 | null | posse declaration and signed termo (DCN 1/2019) |
| Luiz Inácio Lula da Silva | null | 2023-01-01 | null | posse declaration (DCN 1/2023); MP 1.388 of 24 Aug 2026 attests him in office |

### How a start and an end are decided

This packet applies one rule to all three parts. A holder has `from` only where a declaration of posse or a termo de
posse states that the person was invested for a period beginning that day ("Declaro empossado ... para o período de
..." and "foram solenemente empossados"; from 1999 the openings also say "para o período a iniciar-se nesta data"),
and `until` only where a source states the day a resignation or removal took effect; otherwise the holder is dated by
`attested_on`. The South Africa packet (CLAUDE-C01-09) set almost no starts because its oath records state no first
day of office; the Brazilian posse record does state one, so the start rests on it and not on the oath, which is
recorded as its own claim and is never a start. Statements that a person "assumes today" support a start but never
make one.

Three kinds of statement were proposed as ends and are refused:

- **Periods declared at a posse.** Every posse record prints a period (for example 31 August 2016 to 31 December 2018
  for Temer). It is prospective: Rousseff's own 2015 posse declared a period to 31 December 2018 and her term ended on
  31 August 2016. So no printed period end is an `until`; the text stays in the posse claim, and Lula's printed end of
  4 January 2027, which falls after the cutoff, is never a structured date (check C1-C3).
- **A successor's or third party's statement at the successor's posse.** Cardoso's thanks to Itamar Franco "No
  momento em que deixa o Governo" (1 January 1995) and Rousseff's "Hoje o presidente Lula deixa o governo" (1 January
  2011) are claims (`predecessor_departure_statement`), not ends, as CLAUDE-C01-09 treated a third party on Mbeki and a
  successor on Motlanthe (check B1, B2).
- **A successor's posse, "ex-Presidente" styling, and retrospective Presidency Library spans.** None is used as an
  end; the library spans are claims with no structured date.

The two stated ends are Collor's resignation (his letter, the vacancy declared by the President of Congress and
Itamar Franco's termo that the resignation "occurred" that day) and Rousseff's removal (Resolution No. 35 enters into
force on publication and was published in the DOU extra edition of 31 August 2016). Both are day-only. Suspensions
(Collor from 2 October 1992; Rousseff from 12 May 2016) are claims and do not split either holder, and a
Vice-President exercising the office is never a holder.

### Date ledger

Each row is a separate dated fact with its own claim; two facts on one day stay two claims.

| Date | Event | Claim or field |
|---|---|---|
| 17 Dec 1989 | Collor elected at the ballots of 15 Nov and 17 Dec 1989 | `br_collor_elected_ballots_19891217`, `br_collor_second_round_recited_19891217` (claims) |
| 30 Dec 1989 | TSE diploma; diplomação ceremony; the posse record's recital | `br_collor_tse_diploma_issued_19891230`, `br_tse_diplomacao_ceremony_catalogued_19891230`, `br_collor_diplomacao_recited_19891230` |
| 5 Jan 1990 | Sarney signs Decree No. 98.797 | `br_sarney_signs_decreto_98797_19900105`; Sarney `attested_on` |
| 13 Mar 1990 | Sarney signs Decree No. 99.167 | `br_sarney_signs_decreto_99167_19900313` (context) |
| 15 Mar 1990 | Collor declared invested; termo at 10:00; "acabo de assumir a Presidência" | `br_collor_posse_declared_congress_19900315`, `br_collor_termo_de_posse_19900315`, `br_collor_assumes_presidency_statement_19900315`; Collor `from` |
| 29 Sep 1992 | Chamber authorizes the process, 441-38 | `br_chamber_authorizes_impeachment_process_19920929` |
| 30 Sep 1992 | The Chamber's letter read in the Senate | `br_chamber_authorization_read_in_senate_19920930` |
| 1 Oct 1992 | Senate approves Parecer nº 302; Mesa minutes (12:00); summons; message to the Vice-President; Collor signs Decree No. 663 | `br_senate_approves_process_opening_opinion_19921001`, `br_senate_mesa_formalizes_summons_19921001`, `br_senate_summons_mandado_suspension_terms_19921001`, `br_vice_president_instructed_to_assume_19921001`, `br_collor_signs_decreto_663_19921001` |
| 2 Oct 1992 | Collor receives the summons at 10:20 (suspended); Itamar Franco's exercise dated from that day | `br_collor_receives_summons_suspended_19921002`, `br_itamar_states_exercise_from_19921002` (claims, not holders) |
| 7 Oct 1992 | Law No. 8.471 by the Vice-President in exercise | `br_itamar_exercising_signs_lei_8471_19921007` |
| 29 Dec 1992 | Letter read to the Senate (9:00-9:43); letter read in Congress and vacancy declared (11:30-11:50); Itamar Franco's posse (12:30); vote to continue the trial 71-8 (18:17) | `br_collor_resignation_letter_read_senate_19921229`, `br_collor_resignation_letter_read_congress_19921229`, `br_presidency_vacancy_declared_19921229`, `br_collor_resignation_occurred_19921229`, `br_collor_resignation_letter_printed_19921229` (Collor `until`); `br_itamar_posse_declared_congress_19921229`, `br_itamar_termo_de_posse_19921229` (Itamar Franco `from`); `br_senate_votes_to_continue_trial_19921229` |
| 30 Dec 1992 | Disqualification vote 76-3 (04:21); sentence; Resolução nº 101, first printed that day | `br_senate_disqualification_vote_19921230`, `br_senate_impeachment_sentence_19921230`, `br_senate_resolution_101_first_printed_19921230`, `br_senate_resolution_101_disqualification_19921230` (reprint of 31 Dec) |
| 3 Oct 1994 / 17 Dec 1994 | Cardoso elected; diplomado | `br_fhc_elected_first_round_19941003`, `br_fhc_diplomado_tse_19941217` |
| 1 Jan 1995 | Oath; posse declared; termo; "que assumo hoje"; thanks to Itamar Franco as he "deixa o Governo" | `br_fhc_oath_before_congress_19950101`, `br_fhc_posse_declared_19950101`, `br_fhc_termo_de_posse_19950101`, `br_fhc_assumes_position_today_19950101` (Cardoso `from`); `br_itamar_leaves_government_fhc_address_19950101` (claim only) |
| 4 Oct 1998 / 12 Dec 1998 | Cardoso re-elected; diplomado | `br_fhc_reelected_first_round_19981004`, `br_fhc_diplomado_tse_19981212` |
| 1 Jan 1999 | Oath; posse declared; termo; second mandate "que hoje se inicia" | `br_fhc_oath_before_congress_19990101`, `br_fhc_posse_declared_19990101`, `br_fhc_termo_de_posse_19990101`, `br_fhc_second_mandate_assumed_acm_address_19990101` (Cardoso `from`) |
| 27 Oct 2002 / 14 Dec 2002 | Lula elected; diplomado | `br_lula_elected_20021027`, `br_lula_diplomado_tse_20021214` |
| 1 Jan 2003 | Oath; posse declared; termo; "hoje assume a Presidência"; recalled by Lula in 2007 | `br_lula_oath_before_congress_20030101`, `br_lula_posse_declared_20030101`, `br_lula_termo_de_posse_20030101`, `br_lula_assumes_presidency_first_secretary_20030101`, `br_lula_assumption_recalled_20070101` (Lula `from`) |
| 29 Oct 2006 / 14 Dec 2006 | Lula re-elected; diplomado | `br_lula_reelected_20061029`, `br_lula_diplomado_tse_20061214` |
| 1 Jan 2007 | Oath; posse declared; termo; "dia inaugural de meu novo mandato" | `br_lula_oath_before_congress_20070101`, `br_lula_posse_declared_20070101`, `br_lula_termo_de_posse_20070101`, `br_lula_new_mandate_inaugural_day_20070101` (Lula `from`) |
| 31 Oct 2010 / 17 Dec 2010 | Rousseff elected; diplomada | `br_dilma_elected_20101031`, `br_dilma_diplomada_tse_20101217` |
| 1 Jan 2011 | Oath; posse declared; termo; sash announced for "today"; Sarney's address; Parlatório address; "Hoje o presidente Lula deixa o governo" | `br_dilma_oath_before_congress_20110101`, `br_dilma_posse_declared_20110101`, `br_dilma_termo_de_posse_20110101`, `br_dilma_assumes_responsibility_sarney_address_20110101`, `br_dilma_assumes_government_parlatorio_20110101` (Rousseff `from`); `br_dilma_sash_announced_congress_address_20110101`, `br_lula_leaves_government_dilma_parlatorio_20110101` (claims only) |
| 26 Oct 2014 / 18 Dec 2014 | Rousseff re-elected; diplomada | `br_dilma_reelected_20141026`, `br_dilma_diplomada_tse_20141218` |
| 1 Jan 2015 | Oath; posse declared; termo; "Assumo meu segundo mandato" | `br_dilma_oath_before_congress_20150101`, `br_dilma_posse_declared_20150101`, `br_dilma_termo_de_posse_20150101`, `br_dilma_assumes_second_mandate_parlatorio_20150101` (Rousseff `from`) |
| 17 Apr 2016 | Chamber authorizes the process, 367-137 | `br_camara_authorizes_senate_trial_rousseff_20160417` |
| 12 May 2016 | Senate admits the denúncia 55-22 (06:30-06:33); mandado read; Rousseff signs the contrafé; notifications served that morning; Temer notified; MP 726 by the Vice-President in exercise | `br_senate_admits_denuncia_rousseff_20160512`, `br_senate_intimation_order_suspension_terms_20160512`, `br_agsen_senate_opens_process_0634_20160512`, `br_rousseff_receives_intimation_suspended_20160512`, `br_radio_senado_notifications_served_20160512`, `br_senate_record_intimation_receipt_signed_20160512`, `br_sentence_records_suspension_20160512`, `br_senate_record_vp_notified_20160512`, `br_temer_vp_in_exercise_mpv726_20160512` (claims, not holders) |
| 31 Aug 2016 | Loss of office 61-20 (13:32-13:34); disqualification 42-36-3, not imposed (14:06-14:08); sentence; Resolution No. 35 and its DOU publication; Mensagem 144; Temer's posse (16:41-16:52) | `br_senate_vote_loss_of_office_20160831`, `br_senate_vote_disqualification_20160831`, `br_sentence_loss_of_office_61_20_20160831`, `br_sentence_disqualification_not_imposed_20160831`, `br_senate_resolution35_removal_rousseff_20160831`, `br_dou_publishes_resolution35_20160831` (Rousseff `until`), `br_dou_masthead_temer_vp_in_exercise_20160831`, `br_mensagem144_to_vp_in_exercise_20160831`, `br_dcn_vp_in_exercise_at_posse_opening_20160831`, `br_temer_posse_declared_20160831`, `br_temer_termo_de_posse_20160831` (Temer `from`) |
| 28 Oct 2018 / 10 Dec 2018 | Bolsonaro elected; diplomado | `br_bolsonaro_elected_20181028`, `br_bolsonaro_diplomado_tse_20181210` |
| 1 Jan 2019 | Posse declared; signed termo (15:00); MP 870; sash ceremony; "ex-Presidente Michel Temer" | `br_bolsonaro_posse_declared_20190101`, `br_bolsonaro_signed_termo_de_posse_20190101`, `br_bolsonaro_signs_as_president_mpv870_20190101` (Bolsonaro `from`); `br_bolsonaro_sash_ceremony_speech_20190101`, `br_temer_styled_ex_president_20190101` (claims only) |
| 30 Oct 2022 / 12 Dec 2022 | Lula elected; TSE diploma; Congress's recital | `br_lula_elected_20221030`, `br_lula_diplomado_tse_20221212`, `br_lula_diplomacao_recited_20221212` |
| 30-31 Dec 2022 | Decrees No. 11.322 and 11.324 by the Vice-President in exercise | `br_mourao_vp_in_exercise_d11322_20221230`, `br_mourao_vp_in_exercise_d11324_20221231` (claims, not holders) |
| 1 Jan 2023 | Posse declared; MP 1.154 | `br_lula_posse_declared_20230101`, `br_lula_signs_as_president_mpv1154_20230101` (Lula `from`) |
| 24 Aug 2026 | MP 1.388 signed by Lula as President | `br_lula_signs_as_president_mpv1388_20260824` |
| undated | Presidency Library spans for Sarney, Collor and Itamar Franco | three `br_bibpr_*` claims (no structured date) |

Date conventions follow CLAUDE-C01-07 to C01-09: `attested_on` is the day of the observed event as the source dates
it, and a page's issue date is `published_date`. A retrospective statement is dated by the day it recalls
(`br_lula_assumption_recalled_20070101` carries 1 January 2003). No clock time is stored.

## Observations

### BR-PRES-01 — The President when the period opens

Evidence: the Presidency's archived legislation pages show Decree No. 98.797 of 5 January 1990 and Decree No. 99.167
of 13 March 1990, each issued by the President of the Republic and signed José Sarney
(`br_sarney_signs_decreto_98797_19900105`, `br_sarney_signs_decreto_99167_19900313`). The Presidency Library's 2016
gallery page gives his period of government as 15.03.1985 to 15.03.1990
(`br_bibpr_sarney_government_period_19850315_19900315`).

Decision: accepted in part. Sarney's holder is dated 5 January 1990, the earliest 1990 attestation pinned to a
byte-stable response (check A7), with no start or end. The 13 March act is a claim on the role, kept off the holder so
that it cannot be read as an end.

Limits: no primary source reviewed states the day Sarney's term ended; the library span is retrospective, the
Constitution's transitional article is procedure only, and Collor's posse is not used as the end. Earlier January 1990
acts exist (Decree No. 98.785 of 4 January), but their archived captures are bot-check pages.

### BR-PRES-02 — Collor's diplomação and posse

Evidence: the TSE digital library's photograph of the diploma, dated Brasília 30 December 1989 and signed by the
Court's President, Francisco Rezek, and the other ministers, certifies Collor's election at the ballots of
15 November and 17 December 1989 (`br_collor_tse_diploma_issued_19891230`, `br_collor_elected_ballots_19891217`); the
TSE catalogue dates the diplomação ceremony to 30 December 1989 (`br_tse_diplomacao_ceremony_catalogued_19891230`).
The Diário do Congresso Nacional nº 8 of 16 March 1990 records the joint session of 15 March 1990: the President of
Congress declared Collor invested for the period 15 March 1990 to 1 January 1995
(`br_collor_posse_declared_congress_19900315`), the termo records the posse at 10:00 and recites the election and
diplomação (`br_collor_termo_de_posse_19900315`, `br_collor_diplomacao_recited_19891230`,
`br_collor_second_round_recited_19891217`), and Collor says in his address that he "acabo de assumir a Presidência da
República" (`br_collor_assumes_presidency_statement_19900315`; check A8).

Decision: accepted. Collor's `from` is 15 March 1990. The election and diplomação dates are claims, never holder dates;
the printed end of the period (1 January 1995) is prospective and not used.

Limits: the photographed diploma is the TSE's collection exemplar, acquired in 1995 (check A5); the TSE's own minutes
of the diplomação session were not found. No Presidency record of the 15 March 1990 sash transfer was found.

### BR-PRES-03 — 1992: impeachment, resignation and succession

Evidence: on 29 September 1992 the Chamber approved the Special Committee's opinion by 441 to 38
(`br_chamber_authorizes_impeachment_process_19920929`), and its letter was read in the Senate on 30 September
(`br_chamber_authorization_read_in_senate_19920930`). On 1 October the Senate approved Parecer nº 302 to institute the
process (`br_senate_approves_process_opening_opinion_19921001`); the Mesa met at 12:00 with the President of the
Supreme Federal Court (`br_senate_mesa_formalizes_summons_19921001`) and signed the summons, which suspends the
President from its receipt (`br_senate_summons_mandado_suspension_terms_19921001`), and a message telling the
Vice-President to assume (`br_vice_president_instructed_to_assume_19921001`). Collor signed Decree No. 663 that day
(`br_collor_signs_decreto_663_19921001`). His contra-fé records receipt of the summons at 10:20 on 2 October at the
Palácio do Planalto (`br_collor_receives_summons_suspended_19921002`). Itamar Franco's handwritten statement of
29 December dates his exercise as Vice-President from 2 October (`br_itamar_states_exercise_from_19921002`), and he
signed Law No. 8.471 on 7 October as "Vice-Presidente da República no exercício do cargo"
(`br_itamar_exercising_signs_lei_8471_19921007`).

On 29 December the judgment session opened at 9:00; defence counsel read Collor's letter to the Senate, and the
session was suspended at 9:43 so that Congress could declare the vacancy (`br_collor_resignation_letter_read_senate_19921229`;
check A3, A4). Congress met at 11:30: the letter, dated 29 December and resigning "nesta data e por este instrumento",
was read, the office was declared vacant and the session rose at 11:50 (`br_collor_resignation_letter_read_congress_19921229`,
`br_presidency_vacancy_declared_19921229`; the letter is reprinted in `br_collor_resignation_letter_printed_19921229`).
At 12:30 Itamar Franco was declared invested for the period 29 December 1992 to 1 January 1995, and the termo says the
office fell to him by the resignation "ocorrida nesta data" (`br_itamar_posse_declared_congress_19921229`,
`br_itamar_termo_de_posse_19921229`, `br_collor_resignation_occurred_19921229`). The Senate voted 71 to 8 at 18:17 to
continue the trial for the disqualification sanction (`br_senate_votes_to_continue_trial_19921229`) and 76 to 3 at 04:21
on 30 December to impose eight years' disqualification (`br_senate_disqualification_vote_19921230`); the sentence and
Resolução nº 101 are dated 30 December (`br_senate_impeachment_sentence_19921230`,
`br_senate_resolution_101_first_printed_19921230`, `br_senate_resolution_101_disqualification_19921230`). The
Presidency Library's spans are claims (`br_bibpr_collor_first_phase_19900315_19921002`,
`br_bibpr_itamar_phase_19921002_period_19921229`).

Decision: accepted. Collor's `until` is 29 December 1992 and Itamar Franco's `from` is the same day. The suspension of
2 October is a claim; Collor remained the holder, suspended, and the holder is not split. The Vice-President's exercise
is claims only. Resolução nº 101 imposes disqualification from public functions ("inabilitação") and says nothing of
political rights (check A1); it was first printed in the 30 December supplement, and the 31 December issue is a reprint
(check A2).

Limits: no source states the hour the resignation took effect (it reached the Senate between 9:00 and 9:43 and was read
in Congress between 11:30 and 11:50). The special DCN edition of 29 December is not in the archive, and the DOU pages
sit behind a CAPTCHA.

### BR-PRES-04 — The posses of 1995 and 1999

Evidence: the DCN nº 1 of 1995 records Cardoso's election in the first round on 3 October 1994 and his diplomação on
17 December 1994 (`br_fhc_elected_first_round_19941003`, `br_fhc_diplomado_tse_19941217`), his oath
(`br_fhc_oath_before_congress_19950101`), the declaration of posse for 1 January 1995 to 31 December 1998 and the termo
(`br_fhc_posse_declared_19950101`, `br_fhc_termo_de_posse_19950101`), and his "posição que assumo hoje"
(`br_fhc_assumes_position_today_19950101`). In the same address he thanks Itamar Franco "No momento em que deixa o
Governo" (`br_itamar_leaves_government_fhc_address_19950101`). The DCN nº 1 of 1999 reproduces the TSE diploma of
12 December 1998 and records the re-election of 4 October 1998, the oath, the declaration of posse for the period
beginning that day, the termo at 17:00 and the Congress President's words on the mandate "que hoje se inicia"
(`br_fhc_reelected_first_round_19981004`, `br_fhc_diplomado_tse_19981212`, `br_fhc_oath_before_congress_19990101`,
`br_fhc_posse_declared_19990101`, `br_fhc_termo_de_posse_19990101`, `br_fhc_second_mandate_assumed_acm_address_19990101`).

Decision: accepted in part. Cardoso's two holders start on 1 January 1995 and 1 January 1999, with no ends. Itamar
Franco's end is not set: the only statement is a successor's thanks, which this packet records as a claim and never as
a boundary (check B1, B2).

Limits: no primary record of the 1995 sash transfer was found; the end of Cardoso's first term is not dated ("o mandato
que se findou" gives no day).

### BR-PRES-05 — The posses of 2003 and 2007

Evidence: the DCN nº 1 of 2003 reproduces the TSE diploma of 14 December 2002, signed by Minister Nelson Jobim as
President of the Court with the other members (check B5), and records the election of 27 October 2002, the oath, the
declaration of posse for the period beginning 1 January 2003, the signed termo and the First Secretary's statement that
Lula "hoje assume a Presidência" (`br_lula_elected_20021027`, `br_lula_diplomado_tse_20021214`,
`br_lula_oath_before_congress_20030101`, `br_lula_posse_declared_20030101`, `br_lula_termo_de_posse_20030101`,
`br_lula_assumes_presidency_first_secretary_20030101`). The DCN nº 1 of 2007 does the same for the re-election of
29 October 2006 and the diploma of 14 December 2006 (`br_lula_reelected_20061029`, `br_lula_diplomado_tse_20061214`,
`br_lula_oath_before_congress_20070101`, `br_lula_posse_declared_20070101`, `br_lula_termo_de_posse_20070101`,
`br_lula_new_mandate_inaugural_day_20070101`), and in it Lula recalls assuming the Presidency in that chamber on a
1 January four years earlier (`br_lula_assumption_recalled_20070101`; check B7). Rousseff's 2011 Parlatório address says
"Hoje o presidente Lula deixa o governo" (`br_lula_leaves_government_dilma_parlatorio_20110101`).

Decision: accepted. Lula's two holders start on 1 January 2003 and 1 January 2007. The 2007 holder has no end: the
successor's statement is a claim only, and it differs by a day from the prospective period end (31 December 2010)
declared at the 2007 posse (check B1).

Limits: the termo clock times precede the recorded session openings and are not used; no Presidency record of the 2003
sash transfer could be retrieved (the Senate agency's report is a lead).

### BR-PRES-06 — The posses of 2011 and 2015

Evidence: the DCN nº 1 of 2011 reproduces the TSE diploma of 17 December 2010 and records the election of 31 October
2010, the oath, José Sarney's declaration of posse for the period beginning 1 January 2011, the signed termo, her
statement that the sash would be placed on a woman "today", and Sarney's closing words that through the commitment "a
Presidente eleita assume a responsabilidade" (`br_dilma_elected_20101031`, `br_dilma_diplomada_tse_20101217`,
`br_dilma_oath_before_congress_20110101`, `br_dilma_posse_declared_20110101`, `br_dilma_termo_de_posse_20110101`,
`br_dilma_sash_announced_congress_address_20110101`, `br_dilma_assumes_responsibility_sarney_address_20110101`; check
B6). The Presidency Library's text of her Parlatório address says "eu assumo hoje o governo do meu país"
(`br_dilma_assumes_government_parlatorio_20110101`). The DCN nº 1 of 2015 and the Portal Planalto's address of
1 January 2015 do the same for the second mandate (`br_dilma_reelected_20141026`, `br_dilma_diplomada_tse_20141218`,
`br_dilma_oath_before_congress_20150101`, `br_dilma_posse_declared_20150101`, `br_dilma_termo_de_posse_20150101`,
`br_dilma_assumes_second_mandate_parlatorio_20150101`).

Decision: accepted. Rousseff's two holders start on 1 January 2011 and 1 January 2015; the 2015 holder's end is set in
BR-PRES-07.

Limits: the sash transfer of 2011 is recorded only prospectively; the Senate and Chamber agencies' reports of it are
leads.

### BR-PRES-07 — 2016: impeachment and succession

Evidence: the Chamber approved the authorization on 17 April 2016 by 367 to 137
(`br_camara_authorizes_senate_trial_rousseff_20160417`). The Senate admitted Denúncia nº 1/2016 by 55 to 22 in a vote
from 06:30 to 06:33 on 12 May, and its President read the Mandado de Intimação, under which the process is instituted
and the President suspended from its receipt (`br_senate_admits_denuncia_rousseff_20160512`,
`br_senate_intimation_order_suspension_terms_20160512`, `br_agsen_senate_opens_process_0634_20160512`). The Senate's
published image of her contrafé reads that Dilma Vana Rousseff received the mandado "nesta data", Brasília 12 May 2016
(`br_rousseff_receives_intimation_suspended_20160512`; check C7); Rádio Senado reports that the First Secretary
notified her at the Planalto that morning and then Temer at the Jaburu (`br_radio_senado_notifications_served_20160512`);
the case record files both (`br_senate_record_intimation_receipt_signed_20160512`, `br_senate_record_vp_notified_20160512`),
and the sentence says she was summoned and suspended on 12 May (`br_sentence_records_suspension_20160512`). Temer
adopted MP 726 that day as "Vice-Presidente da República, no exercício do cargo de Presidente da República"
(`br_temer_vp_in_exercise_mpv726_20160512`). On 31 August the Senate voted 61 to 20 for loss of office (13:32-13:34) and
42 to 36 with 3 abstentions on disqualification, short of two-thirds (14:06-14:08)
(`br_senate_vote_loss_of_office_20160831`, `br_senate_vote_disqualification_20160831`,
`br_sentence_loss_of_office_61_20_20160831`, `br_sentence_disqualification_not_imposed_20160831`). Resolution No. 35
imposes loss of office and enters into force on publication, and the DOU extra edition of 31 August publishes it
(`br_senate_resolution35_removal_rousseff_20160831`, `br_dou_publishes_resolution35_20160831`). The DOU masthead,
Mensagem 144 and the opening of the posse session still style Temer as the Vice-President in exercise
(`br_dou_masthead_temer_vp_in_exercise_20160831`, `br_mensagem144_to_vp_in_exercise_20160831`,
`br_dcn_vp_in_exercise_at_posse_opening_20160831`). Congress then declared Temer invested for the period beginning
31 August 2016 "tendo em vista a vacância do cargo", and the signed termo records the posse "em virtude de vacância
ocorrida" that day (`br_temer_posse_declared_20160831`, `br_temer_termo_de_posse_20160831`).

Decision: accepted. Rousseff's 2015 holder ends on 31 August 2016 and Temer's starts that day. The suspension of 12 May
is a claim and does not split her holder; Temer's exercise as Vice-President is claims only. The printed period end
(31 December 2018) is not an end (check C2).

Limits: no hour is stated for the receipt of the intimation or for the resolution's effect. The Imprensa Nacional
original of the DOU extra edition was not fetched (a CAPTCHA field); the Senate's filed copy is used.

### BR-PRES-08 — The posse of 2019

Evidence: the DCN nº 1 of 2019 records the election of 28 October 2018 (`br_bolsonaro_elected_20181028`; check C4), the
TSE diploma of 10 December 2018 (`br_bolsonaro_diplomado_tse_20181210`), the declaration of posse for the period
beginning 1 January 2019 (`br_bolsonaro_posse_declared_20190101`) and, in the complete issue, the signed termo at 15:00
(`br_bolsonaro_signed_termo_de_posse_20190101`; check C10). Bolsonaro signed MP 870 as President that day
(`br_bolsonaro_signs_as_president_mpv870_20190101`). The Presidency's page for his address at the "cerimônia de
Recebimento da Faixa Presidencial" was published at 16h45 that day, with its dateline misprinted "2018"
(`br_bolsonaro_sash_ceremony_speech_20190101`; check C8). The President of Congress referred to "o ex-Presidente Michel
Temer" in the same session (`br_temer_styled_ex_president_20190101`).

Decision: accepted in part. Bolsonaro's holder starts on 1 January 2019. Temer's end is not stated: the period declared
at his 2016 posse is prospective, the styling is context, and Bolsonaro's posse is not used.

Limits: the sash page does not name who handed over the sash.

### BR-PRES-09 — The posse of 2023

Evidence: the DCN nº 1 of 2023 records the election of 30 October 2022, the diplomação of 12 December 2022 and the
declaration of posse for "o período de 1º de janeiro de 2023 a 4 de janeiro de 2027" (`br_lula_elected_20221030`,
`br_lula_diplomacao_recited_20221212`, `br_lula_posse_declared_20230101`), and its pages 18-19 reproduce the TSE diploma
signed by Minister Alexandre de Moraes (`br_lula_diplomado_tse_20221212`; check C9). Lula signed MP 1.154 as President
that day (`br_lula_signs_as_president_mpv1154_20230101`). Vice-President Hamilton Mourão signed Decrees No. 11.322 and
11.324 on 30 and 31 December 2022 "no exercício do cargo de Presidente da República"
(`br_mourao_vp_in_exercise_d11322_20221230`, `br_mourao_vp_in_exercise_d11324_20221231`).

Decision: accepted in part. Lula's 2023 holder starts on 1 January 2023. Bolsonaro's end is not stated; the
Vice-President's exercise does not end his term and is claims only. The printed end of 2027 lies beyond the cutoff and
is kept in the claim's text only (check C1).

Limits: no record of the 1 January 2023 sash ceremony was found, and the decrees do not say why the Vice-President
exercised the office.

### BR-PRES-10 — An attestation before the cutoff

Evidence: the Presidency's page for Medida Provisória nº 1.388 of 24 August 2026, captured on 26 August 2026, shows it
adopted by "O PRESIDENTE DA REPÚBLICA" and signed by Luiz Inácio Lula da Silva
(`br_lula_signs_as_president_mpv1388_20260824`).

Decision: accepted. The claim is cited by Lula's 2023 holder as an in-office attestation; it sets no end and extends
nothing.

## Sources added

| Source ID | What | Response identity and provenance |
|---|---|---|
| `br_planalto_decreto_98797_19900105` | Planalto: Decree No. 98.797, signed by Sarney (5 Jan 1990) | 4,920 bytes, `d87cf71b…8d62d8`; capture 2012-06-24 |
| `br_planalto_decreto_99167_19900313` | Planalto: Decree No. 99.167, signed by Sarney (13 Mar 1990) | 14,155 bytes, `28fe2304…2ec69f`; capture 2015-02-21 |
| `br_bibpr_sarney_page_2016` | Presidency Library: Sarney gallery page (retrospective span) | 45,127 bytes, `4f1f6d96…b0b34d`; capture 2016-08-14 |
| `br_tse_diploma_collor_19891230` | TSE digital library: photograph of Collor's diploma (30 Dec 1989) | 3,529,917 bytes, `9db4c926…a75427`; TSE repository file |
| `br_tse_catalogue_diplomacao_1989` | TSE digital library: catalogue record of the 1989 diplomação ceremony | 5,194 bytes, `ade97072…eda4b0`; TSE repository file |
| `br_dcn_08_19900316` | DCN nº 8/1990: joint session of 15 Mar 1990 (Collor's posse) | 1,255,764 bytes, `838abf03…7b6645`; Senate file; PDF pages 1, 3, 4 viewed |
| `br_dcd_161_19920930` | DCN Seção I (Chamber) nº 161/1992: the Chamber's 441-38 vote of 29 Sep 1992 | 11,721,640 bytes, `20b17a8b…63aed6`; Chamber file; PDF pages 1, 57 viewed |
| `br_dsf_163_19921001` | DCN Seção II (Senate) nº 163/1992: the Chamber's letter read on 30 Sep 1992 | 3,889,678 bytes, `d1d0273d…8fb281`; Senate file; PDF pages 1, 3 viewed |
| `br_dsf_164_19921002` | DCN Seção II (Senate) nº 164/1992: Parecer nº 302 approved on 1 Oct 1992 | 4,417,869 bytes, `3df14147…3d89c7`; Senate file; PDF page 15 viewed |
| `br_planalto_decreto_663_19921001` | Planalto: Decree No. 663, signed by Collor (1 Oct 1992) | 14,428 bytes, `9da119b4…46147e`; capture 2008-10-26 |
| `br_senado_autos_impeachment_vol1` | Senate case file DIV 12/1992, vol. I: summons, message to the Vice-President, contra-fé | 47,475,991 bytes, `3e4ad563…7d83be`; Senate file; PDF pages 1, 786, 787, 788, 791 viewed |
| `br_planalto_lei_8471_19921007` | Planalto: Law No. 8.471, signed by the Vice-President in exercise (7 Oct 1992) | 5,010 bytes, `39e80d34…2928b1`; capture 2008-10-14 |
| `br_dcn_70_19921230` | DCN nº 70/1992: resignation letter, vacancy and Itamar Franco's posse (29 Dec 1992) | 397,945 bytes, `dfe8e27a…020d33`; Senate file; PDF pages 3, 6 viewed |
| `br_dsf_223_suplemento_19921230` | DCN Seção II (Órgão Judiciário), 30 Dec 1992 (Suplemento Único to DSF 223): judgment session and Resolução nº 101 | 24,080,741 bytes, `563c6a98…ffdf74`; Senate file; PDF pages 1, 12, 235, 236 viewed; located by the check |
| `br_dsf_224_19921231` | DCN Seção II nº 224/1992: letter, sentence and Resolução nº 101 (reprint) | 1,087,012 bytes, `f6d9477c…c07147`; Senate file; PDF pages 1, 2, 3, 4 viewed |
| `br_bibpr_collor_page_2016` | Presidency Library: Collor gallery page (retrospective span) | 42,045 bytes, `e0ad5fdb…cbf49d`; capture 2016-08-12 |
| `br_bibpr_itamar_page_2016` | Presidency Library: Itamar Franco gallery page (retrospective span) | 43,437 bytes, `9f67a1a4…e2ac22`; capture 2016-08-13 |
| `br_dcn_1_1995_posse_19950101` | DCN nº 1/1995: Cardoso's posse (1 Jan 1995) | 845,143 bytes, `fb7ff244…abc5c5`; Senate file; PDF pages 1, 5, 6, 7 viewed |
| `br_dcn_1_1999_posse_19990101` | DCN nº 1/1999: Cardoso's second posse (1 Jan 1999) | 939,787 bytes, `b7c0283e…2f2abb`; Senate file; PDF pages 3, 4, 6, 9, 13 viewed |
| `br_dcn_1_2003_posse_20030101` | DCN nº 1/2003: Lula's posse (1 Jan 2003) | 1,212,635 bytes, `86f2d392…bca32d`; Senate file; PDF pages 3, 4, 6, 7, 13 viewed |
| `br_dcn_1_2007_posse_20070101` | DCN nº 1/2007: Lula's second posse (1 Jan 2007) | 1,829,092 bytes, `395e155d…d28822`; Senate file; PDF pages 4, 8, 9 viewed |
| `br_dcn_1_2011_posse_20110101` | DCN nº 1/2011: Rousseff's posse (1 Jan 2011) | 506,362 bytes, `b75d4e99…baccbb`; Senate file; PDF pages 1, 4, 8, 9 viewed |
| `br_planalto_library_dilma_parlatorio_20110101` | Presidency Library: Rousseff's Parlatório address (1 Jan 2011) | 37,635 bytes, `3476ba07…94272a`; capture 2026-01-23 |
| `br_dcn_1_2015_posse_20150101` | DCN nº 1/2015: Rousseff's second posse (1 Jan 2015) | 8,131,269 bytes, `8f588186…861787`; Senate file; PDF pages 1, 4, 6, 7, 10, 11 viewed |
| `br_planalto_dilma_parlatorio_20150101` | Portal Planalto: Rousseff's Parlatório address (1 Jan 2015) | 63,136 bytes, `2cbd5e75…25947a`; capture 2015-01-11 |
| `br_camara_dcd_20160418` | Diário da Câmara nº 56/2016: the Chamber's 367-137 vote of 17 Apr 2016 | 2,515,996 bytes, `266a43a0…4a37c2`; Chamber file; PDF pages 1, 120 viewed |
| `br_senado_dsf64_20160512_p166_171` | DSF nº 64/2016 (complete issue; pp. 166-171): admission 55-22 and the Mandado de Intimação (12 May 2016) | 27,973,809 bytes, `c98dafbb…7b8c5d`; Senate file; PDF pages 167, 168 viewed |
| `br_agsen_20160512_senado_abre_processo` | Agência Senado: the Senate opens the process (12 May 2016) | 121,960 bytes, `61689374…13b0be`; capture 2016-05-13 |
| `br_senado_rousseff_contrafe_20160512` | Senate: Rousseff's signed contrafé of the intimation (12 May 2016) | 311,449 bytes, `41f9c9d8…a41b97`; Senate file; located by the check |
| `br_radio_senado_notifications_20160512` | Rádio Senado: notifications served on 12 May 2016 | 103,027 bytes, `8fa440b7…0f86dd`; capture 2016-05-13; located by the check |
| `br_senado_den1_2016_materia_20260508` | Senate matter page DEN 1/2016: case record | 1,097,044 bytes, `0880869d…5ee7c3`; capture 2026-05-08 |
| `br_planalto_mpv726_20160512` | Planalto: MP 726, signed by the Vice-President in exercise (12 May 2016) | 144,547 bytes, `32c843c5…876c84`; capture 2016-05-17 |
| `br_senado_den1_2016_vote_loss_of_office` | Senate vote list: loss of office 61-20 (31 Aug 2016) | 156,974 bytes, `7c0e3097…a33e24`; Senate file; PDF pages 1, 3 viewed |
| `br_senado_den1_2016_vote_disqualification` | Senate vote list: disqualification 42-36-3 (31 Aug 2016) | 151,651 bytes, `a5538be7…0e1152`; Senate file; PDF pages 1, 3 viewed |
| `br_senado_den1_2016_resolucao35` | Senate Resolution No. 35/2016 (signed original) | 37,070 bytes, `6c0e65e9…2c03fe`; Senate file; PDF page 1 viewed |
| `br_senado_den1_2016_dou_20160831_extra` | DOU extra edition 168-A of 31 Aug 2016 (Senate's filed copy): Resolution 35 and the sentence | 192,778 bytes, `0096f80f…a24dd8`; Senate file; PDF pages 1, 2 viewed |
| `br_senado_den1_2016_mensagem144` | Senate: Ofício 1.117 and Mensagem 144 to the Vice-President in exercise (31 Aug 2016) | 61,472 bytes, `ad1913ec…3f0535`; Senate file; PDF pages 1, 2 viewed |
| `br_cn_dcn15_20160901` | DCN nº 15/2016: Temer's posse (31 Aug 2016) | 8,585,228 bytes, `60e2c85c…824195`; Senate file; PDF pages 4, 6 viewed |
| `br_cn_dcn1_20190102_p5_11` | DCN nº 1/2019 (complete issue; pp. 5-11): Bolsonaro's posse (1 Jan 2019) | 49,257,639 bytes, `2d8a63d5…0b24eb`; Senate file (same response as the complete-issue row); PDF pages 6, 7 viewed |
| `br_cn_dcn1_20190102_full` | DCN nº 1/2019, complete issue: TSE diploma and signed termo de posse | 49,257,639 bytes, `2d8a63d5…0b24eb`; Senate file; PDF pages 14, 20 viewed |
| `br_planalto_mpv870_20190101` | Planalto: MP 870, signed by Bolsonaro (1 Jan 2019) | 241,409 bytes, `98d89a08…00103e`; capture 2019-01-03 |
| `br_planalto_bolsonaro_sash_speech_20190101` | Planalto: Bolsonaro's address at the sash ceremony (1 Jan 2019) | 192,253 bytes, `7e6995ac…2fa19d`; capture 2022-03-05; located by the check |
| `br_planalto_d11322_20221230` | Planalto: Decree No. 11.322, signed by the Vice-President in exercise (30 Dec 2022) | 16,059 bytes, `582e464c…f44e34`; capture 2022-12-31 |
| `br_planalto_d11324_20221231` | Planalto: Decree No. 11.324, signed by the Vice-President in exercise (31 Dec 2022) | 15,704 bytes, `b791adf5…f3bd16`; capture 2023-01-01 |
| `br_cn_dcn1_20230102_p1_8` | DCN nº 1/2023 (complete issue; pp. 1-8): Lula's posse (1 Jan 2023) | 24,950,218 bytes, `d6c9c275…c854ee`; Senate file; PDF pages 1, 6, 7 viewed |
| `br_cn_dcn1_20230102_p18_19` | DCN nº 1/2023 (complete issue; pp. 18-19): Lula's TSE diploma (12 Dec 2022) | 24,950,218 bytes, `d6c9c275…c854ee`; Senate file (same response as the row above); PDF pages 18, 19 viewed; located by the check |
| `br_planalto_mpv1154_20230101` | Planalto: MP 1.154, signed by Lula (1 Jan 2023) | 314,530 bytes, `61b3b641…ea1315`; capture 2023-01-02 |
| `br_planalto_mpv1388_20260824` | Planalto: MP 1.388, signed by Lula (24 Aug 2026) | 45,429 bytes, `8b9eeda3…6ed3f5`; capture 2026-08-26 |

Nineteen sources are raw Internet Archive captures (`id_` form) made before the cutoff; each records the capture URL as
`url`, the address inside it (without `:80`) as `original_url` and the capture time in its extract. Twenty-nine are
official files: Diário do Congresso Nacional, Senate and Chamber diaries and Senate case documents from the Senate's
diary store and document service (`legis.senado.leg.br`) and the Chamber's image server (`imagem.camara.leg.br`), two
TSE digital-library responses and the Senate's published contrafé image (`www12.senado.leg.br`). The Senate diary
viewer's page-range form (seqPaginaInicial/seqPaginaFinal) is rebuilt by Aspose.PDF on every request with a new
CreationDate and document /ID, so the four records first taken as page ranges (DSF 64/2016 pp. 166-171; DCN 1/2019 pp.
5-11; DCN 1/2023 pp. 1-8 and 18-19) record the complete stored issue (download=true) instead, whose PDF page numbers
equal the printed ones; the DCN 1/2019 record shares the identity of `br_cn_dcn1_20190102_full`, and the two DCN 1/2023
records share one identity. All 48 recorded identities were re-downloaded and matched by the source verification on 23
September 2026. The Suplemento Único to DSF 223 came back 1,390 bytes short on the check's first transfer; the next two
transfers were identical and are the recorded identity.

Each new source has a derived factual extract under [sources/](sources/) in the packet's format
(`spheres-c01-derived-factual-table/v1`): one row per claim, keyed by `claim_id`, with `observation_id`
`br_presidency`, `review_observation`, `role_id` `br_president`, `holder_name`, `role_title`, `event_kind`,
`attested_on`, the claim's text and locator. The extract's checksum is in the packet, separate from the response hash.
Original pages, PDFs, images and renders are not checked in, and no seal, coat of arms, signature, diploma image or
photograph is republished.

Retrospective Presidency pages (the Library's 2016 gallery pages) are labelled as such and bound nothing. The TSE
diploma photograph is the TSE's collection exemplar (catalogue record bdtse/1942, acquired in 1995), stated in its scope
note. The DOU extra edition of 2016 is the Senate's filed copy of the Imprensa Nacional publication.

Source types: `primary_presidency_legislation_page_archived`, `primary_presidency_library_page_archived`,
`primary_presidency_speech_text_archived`, `primary_electoral_court_repository_image`,
`primary_electoral_court_catalogue_record`, `primary_congress_session_record_pdf`, `primary_chamber_session_record_pdf`,
`primary_senate_session_record_pdf`, `primary_senate_case_file_pdf`, `primary_senate_case_record_page_archived`,
`primary_senate_news_agency_report_archived`, `primary_senate_published_document_image` and
`primary_official_gazette_copy_filed_by_senate`.

## Leads not imported

- Agência Senado, "Transmissão da faixa ocorreu no parlatório" (1 January 2003): Lula received the sash from Fernando
  Henrique Cardoso at the Parlatório at 17h05, after the posse in Congress. Capture 20251009142606
  (https://web.archive.org/web/20251009142606id_/https://www12.senado.leg.br/noticias/materias/2003/01/01/transmissao-da-faixa-ocorreu-no-parlatorio;
  44,603 bytes, `2a7e93a4…05de`, stable) and an earlier capture 20210927124126 (46,684 bytes, `b6b326c9…a7e7`) whose
  dateline shows 31/12/2002 23h00. The Senate's agency reporting a ceremony at the Planalto, outside its own
  proceedings: a lead only (check B8).
- Agência Senado, "Dilma recebe a faixa de Lula no Parlatório" (1 January 2011, 16h51;
  https://www12.senado.leg.br/noticias/materias/2011/01/01/dilma-recebe-a-faixa-de-lula-no-parlatorio) and Agência
  Câmara, "Dilma recebe a faixa presidencial" (1 January 2011;
  https://www.camara.leg.br/noticias/208597-dilma-recebe-a-faixa-presidencial): live pages only, with dynamic sidebars
  (44,376 bytes, `c58ef151…c399`; 55,072 bytes, `e5eaab49…5d19`; not reproducible) reporting a ceremony outside the
  agencies' own proceedings. Leads only.
- Agência Câmara's retrospective chronology (https://www.camara.leg.br/noticias/22024-cronologia-do-impeachment/),
  the Planalto's current Vice-President profile of Itamar Franco and the current Presidency Library gallery pages:
  retrospective and live; not fetched.
- The Senate matter page for DIV 12/1992 (https://www25.senado.leg.br/web/atividade/materias/-/materia/2390): a dynamic
  finding aid for the case-file volumes.
- Planalto live pages of Decree No. 99.177 of 15 March 1990 (Collor; https://www.planalto.gov.br/ccivil_03/decreto/1990-1994/d99177.htm)
  and Decree No. 667 of 15 October 1992 (Vice-President in exercise; .../d0667.htm): read live, not byte-stable.
- The Chamber diary of 3 October 1992 (DCD03OUT1992.pdf), DSF nº 165 of 3 October 1992 (codDiario=6431; 413,928 bytes,
  `9c19846f…ced2dd`) and DSF nº 166 of 6 October 1992 (codDiario=6432): speeches about the Vice-President's assumption,
  no act. DSF nº 222 of 29 December 1992 (codDiario=6487): a message of 23 December signed by Itamar Franco, not needed.
- Senate case-file volumes II (dm=3692516; 57,493,521 bytes, `0ccf0ee6…3b46d1`) and III (dm=3692525; 46,196,632 bytes,
  `52629743…00d47a`): no dated step used here.
- The archived Planalto decrees 664-666 of 1 October 1992 (all Collor's) and the Presidency Library's 1990 speech list
  for Sarney (capture 20240222081438), which ends on 7 February 1990 with no statement of his end.
- The Chamber's partial three-page copy of the 2007 issue (https://imagem.camara.leg.br/Imagem/d/pdf/DCN02JAN2007.pdf;
  177,847 bytes, `8ffe08dc…55e2`); the Presidency Library's Lula biography
  (https://web.archive.org/web/20181116101103id_/http://www.biblioteca.presidencia.gov.br/presidencia/ex-presidentes/luiz-inacio-lula-da-silva/biografia-periodo-presidencial;
  38,811 bytes, `4fcfec31…fa67`); the Library's transcripts of Lula's 2003 and 2007 speeches (76,400 bytes,
  `621ada9c…9191`; 49,819 bytes, `d7608926…3bed`), which duplicate the DCN or add no dated fact; the Library's copies of
  Cardoso's 1995 and 1998 speeches and the Portal Planalto copies of Rousseff's speeches in Congress; the 2015
  presidential agenda (prospective).
- Agência Senado, 12 May 2016 07h11 (https://web.archive.org/web/20160513113821id_/http://www12.senado.leg.br/noticias/materias/2016/05/12/vicentinho-alves-levara-mandado-de-intimacao-a-dilma-rousseff;
  109,975 bytes, `a09acf0c…dad3b`): a prospective account of the intimation.
- DSF nº 64/2016's Supplement A (codDiario=20398; 4,973,871 bytes, `7c2646ee…fe58a`), DSF nº 65/2016 pp. 1-8
  (codDiario=20405) and the sentence as a separate scan (dm=4654044): about the Senate's own organization or duplicated
  by the DOU copy.
- The earlier capture of the Planalto sash-ceremony page (20190903092728; 47,704 bytes, `1a5a583b…d8e2`): same text,
  no publication stamp.
- Planalto, "Discurso do presidente Lula no Parlatório do Palácio do Planalto" (published 6 January 2023; capture
  20240716071744 of https://www.gov.br/planalto/pt-br/acompanhe-o-planalto/discursos-e-pronunciamentos/2023/discurso-do-presidente-lula-no-parlatorio-do-palacio-do-planalto;
  153,484 bytes, `7241c67a…f1ea`): no dateline and no mention of the sash; adds no dated fact.
- The contrafé's wrapper page (https://www12.senado.leg.br/noticias/infograficos/2016/05/comprovante-de-recebimento-de-dilma):
  dynamic; the image it links is imported.
- Law No. 15.490 of 17 August 2026 (capture of .../2026/lei/l15490.htm; 36,153 bytes, `edbe09c5…b5abd6`) and veto
  message 541 of 17 June 2026: further 2026 attestations, not needed. The live MP 726 page (later annotations) and the
  Planalto's news item on the Vice-President's 2023 posse (geraldo-alckmin-toma-posse-como-vice-presidente-da-republica;
  out of scope).
- The Presidency Library's biographies of Rousseff, Temer and Bolsonaro: retrospective, not downloaded.
- The Constitution (https://www.planalto.gov.br/ccivil_03/Constituicao/ConstituicaoCompilado.htm): ADCT article 4 and
  articles 51, 52, 78, 79, 82 and 86 frame the procedure; constitution text establishes procedure only, never a date.

## Sources attempted

- Senate digital library (BDSF: https://www2.senado.leg.br/bdsf/handle/id/50540 and a 2011 news item, handle id/48010):
  a "Verificação de segurança" JavaScript proof-of-work challenge, not bypassed.
- Diário Oficial da União viewer (pesquisa.in.gov.br, including page 18979 of 31 December 1992 and the issue of
  2 October 1992): the viewer carries a CAPTCHA, which was not attempted; the Imprensa Nacional original of the 2016
  extra edition was not fetched for the same reason.
- Internet Archive: HTTP 429 rate limiting for several minutes in all three parts, and "Temporarily Offline" CDX
  responses; every capture used was fetched after spaced retries. Reviewers should space re-downloads.
- The 2021 captures of Planalto Decrees No. 98.785 and 99.176 are F5 bot-check pages (4,740 bytes); live Planalto pages
  carry a per-request script token and are not byte-stable.
- Presidency Library: the live site returned "Esta página está em manutenção"; the 2022 capture of Collor's 1990 speech
  list, the 2024 capture of Rousseff's 2015 Parlatório page, the 2023 capture of the 2011 sash photograph
  (5,191 bytes, `c37b47c4…f522`) and the 2025 capture of Lula's 2003 Parlatório file (5,593 bytes, `714dc880…8045`) are
  JavaScript challenge pages. Not bypassed.
- Chamber image server: HTTP 403 (a 2,328-byte firewall page) for the 1995, 1999, 2003, 2011 and 2015 DCN issues; the
  Senate copies are used.
- Senate diary archive: no Senate diary of 16 March 1990 (the posse is in the Congress diary); no special DCN edition of
  29 December 1992; no separate item for the 8 October 1992 Órgão Judiciário supplement (read in the case file); code
  20698 is misfiled; the old search endpoint returned 404; one 2016 download timed out part-way and later succeeded.
- DCN 1/2019 page 14 alone: the `seqPaginaInicial=14&seqPaginaFinal=14` form broke with a chunked-encoding error; the
  complete issue is the recorded identity.
- Senate diary viewer page ranges (`seqPaginaInicial`/`seqPaginaFinal`: DSF 64/2016 pp. 166-171, DCN 1/2019 pp. 5-11,
  DCN 1/2023 pp. 1-8 and 18-19): the PDF is rebuilt by Aspose.PDF with a new /CreationDate, /ModDate and /ID each time
  the server generates it (a cached copy may be served in between), so a later download keeps the byte count but not the
  SHA-256. Not used as identities; the complete issues (`download=true`) are recorded.
- TSE digital library: no minutes or resolution of the 30 December 1989 diplomação session; only photograph items.
- Internet Archive CDX searches found no Planalto statement on 12 May 2016, no Planalto news item on Lula's 2023 posse
  or sash ceremony, and nothing relevant under the 2019 and December 2022 Planalto prefixes.

## Checker defects

| # | Defect | Outcome |
|---|---|---|
| A1 | Resolução nº 101 labelled `loss_of_political_rights` | **Applied**: event kind `disqualification_from_public_function`; the uncertainty says it does not mention political rights |
| A2 | Resolução nº 101 said to be published in DSF 224 of 31 Dec | **Applied**: first printing in the 30 Dec supplement (`br_senate_resolution_101_first_printed_19921230`); DSF 224 described as a reprint |
| A3 | Judgment-session transcript (Suplemento Único to DSF 223, 24 MB) skipped "because of disk limits" | **Applied**: imported as `br_dsf_223_suplemento_19921230` with four claims (the letter read to the Senate, the 18:17 vote to continue, the 04:21 disqualification vote, the resolution's first printing) |
| A4 | Resignation timing could be narrowed | **Applied**: letter presented to the Senate between 9:00 and 9:43 and read in Congress between 11:30 and 11:50; `until` stays day-only |
| A5 | TSE diploma photograph not identified as a collection object | **Applied**: catalogue record bdtse/1942 (acquired in 1995) stated in the scope note and uncertainty; the date is unaffected |
| A6 | Case-file volume I pages reviewed did not line up | **Applied**: PDF pages 1, 786-788 and 791 rendered; 789-790 read from the text layer; printed page mapping stated |
| A7 | Sarney holder note "first 1990 primary attestation" | **Applied**: "the earliest 1990 attestation pinned to a byte-stable response" |
| A8 | Optional: Collor's "acabo de assumir a Presidência" | **Applied**: `br_collor_assumes_presidency_statement_19900315`, support for his `from` |
| B1 | Lula's `until` 2011-01-01 written as a value while Itamar Franco's was conditional | **Applied**: both ends null under one written rule; both statements kept as claims |
| B2 | `end_of_term_statement` overstates Cardoso's thanks | **Applied**: `predecessor_departure_statement` for both the 1995 and 2011 statements |
| B3 | Holder-dating kinds included `oath_of_office` | **Applied**: starts only from `posse_declared` and `posse_record`; every oath claim says it is never a start |
| B4 | Locator for "Hoje o presidente Lula deixa o governo" | **Applied**: paragraphs 2, 3 and 5 counted from the salutation |
| B5 | 2003 diploma "headed by" Jobim | **Applied**: "signed by Minister Nelson Jobim as President of the Court, with the other members of the Court" |
| B6 | Sarney's 2011 statement missing | **Applied**: `br_dilma_assumes_responsibility_sarney_address_20110101`; the "substituir um grande Presidente" passage noted as giving no day |
| B7 | Optional: Lula's 2007 recall of assuming office in 2003 | **Applied**: `br_lula_assumption_recalled_20070101`, retrospective, support only |
| B8 | Sash-transfer leads missing | **Applied**: the Senate and Chamber agency reports listed under leads with their identities; the unresolved note reworded |
| B9 | No rights notes; extract fields missing | **Applied**: every source has a rights note; extracts carry `archive_capture_utc` where archived and `source_response_checked_in: false` |
| C1 | Lula's 2027 period end in a structured field would fail `--check` | **Resolved by removal**: the period claim is merged into `br_lula_posse_declared_20230101`; 4 January 2027 is text only |
| C2 | Temer's and Bolsonaro's ends taken from declared periods | **Applied**: both `until` null; the periods stay in the posse claims' text |
| C3 | Declared-period claims malformed and filed under two observations | **Resolved by removal**: the three `*_declared_mandate_period_*` claims are merged into the posse declarations, as in part B |
| C4 | Bolsonaro's election claim folded in the diplomação | **Applied**: `br_bolsonaro_elected_20181028` states the election only |
| C5 | Event-kind vocabulary differed from part B | **Applied**: one vocabulary for all parts (`posse_declared`, `posse_record`, `in_office_attestation`, ...); the optional split of a separate oath claim for 1990, 1992, 2016, 2019 and 2023 is not made, because those oaths sit inside the posse declarations and an oath is never a boundary |
| C6 | Institution, role and holder shape differed from `za_presidency` | **Applied**: the accepted shape (lifecycle note, `represented_party_ids`, `reconciled_organization_id`, one `claim_ids` list with `sources`, `note` and `uncertainty` per holder, role `scope_note`) |
| C7 | Rousseff's receipt inferred from the case record | **Applied**: her signed contrafé imported (`br_rousseff_receives_intimation_suspended_20160512`) with the Rádio Senado report; only the hour stays open |
| C8 | "No sash record for 2019" | **Applied**: `br_bolsonaro_sash_ceremony_speech_20190101`, with the misprinted dateline and no named giver |
| C9 | The 2022 diplomação rested only on Congress's statement | **Applied**: DCN 1/2023 pp. 18-19 imported (`br_lula_diplomado_tse_20221212`) |
| C10 | Bolsonaro's posse lacked the signed termo | **Applied**: `br_bolsonaro_signed_termo_de_posse_20190101` from page 20 of the complete issue |

Missing primary records found by the checks: the Suplemento Único to DSF 223 (part A); Rousseff's contrafé, the Rádio
Senado report, the 2022 capture of the Planalto sash-ceremony page and DCN 1/2023 pp. 18-19 (part C) are imported. The
Senate and Chamber agency reports of the 2003 and 2011 sash transfers (part B) are leads because they report a
ceremony outside their own proceedings, and two of them are not reproducible. The 2019 capture of the sash page (no
publication stamp) and Lula's undated 2023 Parlatório page are leads. The Library's 2011 sash photograph, its 2003
Parlatório file and the BDSF item are blocked by challenge pages and were not bypassed.

Other changes made to fit the packet's rules rather than a numbered defect:

- Part C claim IDs were renamed to part B's pattern (`br_temer_posse_declared_20160831`, `br_temer_termo_de_posse_20160831`,
  `br_bolsonaro_posse_declared_20190101`, `br_bolsonaro_diplomado_tse_20181210`, `br_lula_posse_declared_20230101`,
  `br_lula_elected_20221030`, `br_lula_diplomacao_recited_20221212`); none of the old IDs remains.
- Acts signed shortly before an event (Sarney on 13 March 1990, Collor on 1 October 1992) are context claims on the role
  and are kept off the holders so that they cannot be read as ends.
- The Vice-President's exercise rows carry the role title "Vice-President of the Republic in exercise of the office of
  President".

## Suggested next work orders

These are proposals for the integrator. They are not created in `work-orders.json`.

- `C01-Brazil-PRES-002`: 1990 — a primary record of the day José Sarney's term ended (DOU of 15 March 1990, the
  Congress record or a Presidency act).
- `C01-Brazil-PRES-003`: ends of terms — a record stating the day each term ended (Itamar Franco 1995, each first term,
  Cardoso 2002, Lula 2010, Temer 2018, Bolsonaro 2022), if one exists beyond the posse records.
- `C01-Brazil-PRES-004`: the TSE's own diplomação minutes or resolutions, 1989-2022.
- `C01-Brazil-PRES-005`: sash transfers, 1990-2023, from Presidency records once the Library is out of maintenance.
- `C01-Brazil-PRES-006`: DOU originals through an access route without a CAPTCHA.
- `C01-Brazil-PRES-007`: vice-presidents and acting presidents during absences, 1990-2026 (outside this packet).

## Integration notes (outside this packet's file boundary)

- **Base and claim:** based on `ffe54b02` (`codex/campaign-certification`); claim commit `0d177d7d` holds only the
  handoff. No other pending packet touches `brazil.json`.
- `research-index.json` is regenerated in a **separate commit**. New totals against `ffe54b02`: 209 sources and 1,874
  claims (previously 161 and 1,762), 28 institution observations (previously 27). Brazil now has one institution, one
  role observation and 32 entries pending mapping; its four open discovery batches become 10, 10, 10 and 2 members, so
  the total of 92 batches is unchanged. The pending South Africa packet (CLAUDE-C01-09) adds its own sources and claims
  to a different packet; the index must be regenerated after both merge.
- Pinned tests in `test_brazil_research_s10f.py`, none loosened and no assertion removed:
  - counts (entries, sources, claims, roles) are now (32, 53, 146, 1), from (31, 5, 34, 0), and the 31 organizations
    are pinned separately;
  - `institutions == []` is replaced by exactly one institution, `br_presidency`, with exactly one role, `br_president`;
  - the two hosts, the 2026-09-13 access date and the null original-response identities still apply to the original
    five sources by id; the 48 new sources are pinned to five hosts and the 2026-09-23 access date with recorded
    identities (exact bytes and SHA-256 in `test_brazil_presidents_c01_10.py`);
  - `mapping_pending` is 32 (from 31) and the Brazil work orders are 10, 10, 10 and 2 members (from 1), with the last
    batch pinned.
- `test_campaign_census` needs `spheres-sim/data`, which is absent from this sparse worktree; it was not run here, and
  the sparse checkout was not widened.
- The atlas (`tools/ui/leadership-research-review.js`) shows only "Observed on" when a holder has `attested_on`, and
  would hide an end on such a holder. No holder here has both an observation date and an end: Sarney is dated by
  observation only, and the eleven posse holders show "Reported interval" from their start (Collor and Rousseff 2015
  with their ends). If an end is later added to an observation-dated holder, its note must carry the end. No UI code
  changed.
- New fields on Brazil sources: `original_url`, `source_type`, `rights_note` and `scope_note`; extracts carry
  `review_observation`, `archive_capture_utc` and `visual_review`. The event-kind vocabulary is pinned in the new test.
- `research/README.md`, the C01 README totals and `docs/planning/ai-workstreams.json` are left for the integrator; this
  handoff is self-proposed and not registered there.

## Checks

```text
python -X utf8 tools/avatars/campaign_research.py
python -X utf8 tools/avatars/campaign_research.py --check
python -X utf8 -m unittest discover -s tools/avatars -p "test_brazil*.py"
python -X utf8 -m unittest discover -s tools/avatars -p "test_*research*.py"
python -X utf8 -m unittest discover -s tools/avatars -p "test_campaign*.py"
node --test tools/ui/check_leadership_research_review.cjs
python tools/planning/workboard.py --check
git diff --check
```

Results are recorded in the handoff.
