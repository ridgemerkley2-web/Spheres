# PT national presidents 22: the party office, 1990-2026

Packet: **CLAUDE-C01-22**. State: **ready_for_review** (not complete).
Owner: Claude. Integrator/reviewer: Codex. Branch `claude/c01-br-22`, **stacked on CLAUDE-C01-17**
(`claude/c01-br-17` at `7d71acef`, itself stacked on CLAUDE-C01-10; both ready for review and not yet integrated);
claim commit `45821428`. Research access: 25 September 2026 (UTC). The historical cutoff stays **7 September 2026**.

This packet adds one party role, `br_pt_president` (Presidente Nacional do Partido dos Trabalhadores, kind
`party_leader`), to the existing `PT` funding observation (`br_tse_fefc_2024_party_03`) in [brazil.json](brazil.json),
and reviews ten observations from 1 January 1990 to the cutoff: the President when the period opens and the end of
Lula's presidency, 1994-1995, José Dirceu's re-elections and departure, José Genoino's presidency, Tarso Genro and
Ricardo Berzoini in 2005, the 2006-2010 interim arrangements and José Eduardo Dutra, Rui Falcão, Gleisi Hoffmann, the
2025 interim President and Edinho Silva, and a party attestation before the cutoff. It adds 108 sources and 177 claims,
the role with eighteen holder observations, a role scope note, one coverage note on the organization and one packet
coverage note. The organization's funding identity, unknown lifecycle, empty game mapping and funding record are
unchanged, and so are the `br_presidency` institution, CLAUDE-C01-10's twelve `br_president` holders and
CLAUDE-C01-17's nine `br_vice_president` holders. It adds no organization, institution, game mapping, lifespan,
portrait or avatar. The parent scope (C01, C06, S23, WC1 and CP1) remains open.

The research was done in three parts (A: 1988-2003; B: 2003-2010; C: 2011-2026), and each part was checked
independently before this packet was written. Every checker defect is applied or explained below (see
[Checker defects](#checker-defects)); 24 of the 25 missing primary records the checks found are imported, and one more
record was located for this packet.

## Outcome

| ID | Question | Decision |
|---|---|---|
| PT-PRES-01 | The PT's national President when the period opens, and the end of Lula's presidency | **Accepted in part:** the hint (Lula) is wrong for 1 January 1990: party records show Luiz Gushiken as President (chosen at the Diretório Nacional meeting of 10-11 December 1988; styled President in February, March and May 1990 without a day), so he has no holder; Lula's own closing speech of Sunday 3 June 1990 states that he assumes the Presidency that day (his start); no source states the day or the reason his presidency ended in 1994 |
| PT-PRES-02 | 1994-1995: the presidents who followed Lula, and José Dirceu's election | **Accepted in part:** Rui Falcão is observed on 25 November 1994 (the party's December 1994 special edition, found by the check), with no start, no end and no statement whether he was interim; December 1994 to August 1995 is unresolved; Dirceu won the 10th Encontro's vote (215 to 183, day not printed) and is observed on 24 August 1995 |
| PT-PRES-03 | Dirceu's re-elections (1997, 1999, 2001) and his departure | **Accepted in part:** observed on 17 September 1997, 2 December 1999 and 4 October 2001; his leave of 17 July 2001 and José Genoino's acting service are claims; he took leave on 7 December 2002 with Genoino interim (claims) and resigned at a two-day meeting of 15-16 March 2003, so no end is recorded |
| PT-PRES-04 | 2002-2005: José Genoino's presidency and its end | **Accepted in part:** elected by the Diretório Nacional on 15 March 2003 after interim service from 7 December 2002; observed on 18 March 2003 (a portal item located for this packet); his declaration of 9 July 2005 both hands the office over and calls the act a leave, so no end is recorded |
| PT-PRES-05 | 2005: Tarso Genro, and Ricardo Berzoini's election and assumption | **Accepted in part:** Tarso Genro was selected on 9 July and is observed on 10 July 2005 (no start; no PT record calls him interim); PED ballots of 18 September and 9 October, declaration of 13 October; Berzoini's posse on 22 October 2005 (his start) |
| PT-PRES-06 | 2006-2010: interim arrangements, Berzoini's re-election and José Eduardo Dutra's election and assumption | **Accepted in part:** Marco Aurélio Garcia's interim service (November 2006 to January 2007) is claims only; no PT record of the PED 2007 result was found; Berzoini is observed on 13 November 2009 (the 2008/2009 executive); Dutra was declared on 25 November and 2 December 2009 and invested on 19 February 2010 (his start); Berzoini's statement that 10 February 2010 was his last day is a claim |
| PT-PRES-07 | 2011-2017: Rui Falcão's presidency | **Accepted in part:** Dutra resigned on 29 April 2011 (his stated end); Falcão was acting on 26 April and is observed on 8 May 2011 and, after the PED 2013 posse during the 5th Congress, on 18 December 2013; no start or end is stated, and the hint of a 2015 re-election is unsupported |
| PT-PRES-08 | 2017-2025: Gleisi Hoffmann's presidency and its end | **Accepted in part:** elected on 3 June 2017, invested on 5 July 2017 (her start); re-elected on 24 November 2019 and observed on 1 April 2020; her departure was announced on 7 March 2025, but no source states the day her office ended |
| PT-PRES-09 | 2025: the interim President and Edinho Silva's election and assumption | **Accepted in part:** Humberto Costa was interim from 7 March 2025 (claims) and states that he became effective President on 20 March 2025 (his start); Edinho Silva's PED vote of 6 July, results of 7 and 15 July, and posse on 3 August 2025 (his start), against a same-day announcement that he would officially assume on 4 August (a prospective claim for Codex to weigh) |
| PT-PRES-10 | An attestation of the national President before the cutoff | **Accepted:** the PT's Diretório Nacional page archived on 15 August 2026 heads its list 'Presidente Nacional do PT - Edinho Silva'; the TSE's live registry record of the organ in force is a lead |

The resulting holder observations of `br_pt_president`, in date order:

| Holder | `attested_on` | `from` | `until` | Basis |
|---|---|---|---|---|
| Luiz Inácio Lula da Silva | null | 1990-06-03 | null | closing speech of the VII Encontro: 'Volto a assumir a Presidência', receiving the baton 'hoje' (Boletim Nacional nº 51) |
| Rui Falcão | 1994-11-25 | null | null | 'O presidente nacional do PT, deputado Rui Falcão, abriu o Seminário' held on 25 November (Boletim Nacional special edition, Dezembro/94) |
| José Dirceu | 1995-08-24 | null | null | a national secretariat's ofício addressed to 'José Dirceu, Presidente do Diretório Nacional' |
| José Dirceu | 1997-09-17 | null | null | 'o presidente nacional do PT, José Dirceu' at a meeting in Vitória 'No dia 17' (PT Notícias nº 52) |
| José Dirceu | 1999-12-02 | null | null | letter of the national presidency signed 'José Dirceu, Presidente Nacional do Partido dos Trabalhadores' |
| José Dirceu | 2001-10-04 | null | null | official note of 4 October 2001 signed as 'presidente nacional do PT' (PT Notícias nº 109) |
| José Genoino | 2003-03-18 | null | null | Portal do PT: a ceremony attended by 'o presidente do PT, José Genoino', three days after his election by the Diretório Nacional |
| Tarso Genro | 2005-07-10 | null | null | Portal do PT: the executive committee's decisions 'segundo informou o presidente Tarso Genro' |
| Ricardo Berzoini | null | 2005-10-22 | null | 'O Diretório Nacional deu posse, neste sábado (22), ao presidente eleito, Ricardo Berzoini' |
| Ricardo Berzoini | 2009-11-13 | null | null | Portal do PT page 'Integrantes da CEN para o biênio 2008/2009' (capture date of an undated page) |
| José Eduardo Dutra | null | 2010-02-19 | 2011-04-29 | invested on the night of 19 February 2010; 'A renúncia do presidente do PT José Eduardo Dutra no dia 29 de abril' (Portal do PT, 8 May 2011) |
| Rui Falcão | 2011-05-08 | null | null | the secretary-general on 'o novo presidente nacional do PT, Rui Falcão' |
| Rui Falcão | 2013-12-18 | null | null | the leaders 'empossado durante o 5º Congresso', headed 'Rui Goethe da Costa Falcão, Presidente' |
| Gleisi Hoffmann | null | 2017-07-05 | null | the new Diretório and presidenta 'está sendo empossado hoje' (post of Wednesday 5 July 2017) |
| Gleisi Hoffmann | 2020-04-01 | null | null | the Diretório Nacional page 'Presidenta Nacional do PT - Gleisi Hoffmann' (capture date) |
| Humberto Costa | null | 2025-03-20 | null | 'agora saio da condição de presidente interino para presidente efetivo', after the Diretório's vote that day |
| Edinho Silva | null | 2025-08-03 | null | 'empossado neste domingo (3)' at the close of the 17th Encontro Nacional |
| Edinho Silva | 2026-08-15 | null | null | the Diretório Nacional page 'Presidente Nacional do PT - Edinho Silva' (capture date, before the cutoff) |

### How a start and an end are decided

The packet applies the rule of the stacked Brazil packets to a party office. A holder has `from` only where a source
states the day the office was assumed or took effect: a party report that the person was invested (`posse_reported`:
Berzoini 2005, Dutra 2010, Gleisi Hoffmann 2017, Edinho Silva 2025) or the holder's own statement that the office was
assumed that day (`stated_assumption_of_office`: Lula on 3 June 1990, Humberto Costa on 20 March 2025). It has `until`
only where a source states the day the office ended: only Dutra's resignation 'no dia 29 de abril' (2011) does. Every
other holder is dated by `attested_on`, and, as in the ANC packet (CLAUDE-C01-16), each such holder cites only
in-office attestations made on its own day; the test pins that every cited claim carries exactly the holder's date.
Days of selection, election and declaration are never holder dates.

Everything else is a claim that never feeds a holder: Encontro and Congresso elections, Diretório and
executive-committee selections, PED ballots, partial counts and declarations, scheduled posses, published posse
speeches, leaves and returns, handovers, departure announcements, resignations dated only by a two-day meeting,
in-office stylings on other days (continuation), stylings dated only by an issue span or a month, acting and interim
service, TSE registry exercise periods and retrospective lists, counts, spans and biographies. Party statutes and the
PED regulations appear only as procedure (the 2005 and 2009 amendments on the posse), never as a date. No end is
inferred from a successor's selection, election or posse.

Five kinds of statement were proposed or could be read as boundaries and are refused:

- **Stated last days and departures.** Berzoini told the Chamber on 10 February 2010 that 'hoje é o último dia do meu
  mandato', but the party reported on 19 February 2010 that he 'deixa a presidência' at Dutra's posse, and the PED
  2009 rule set the posse for 20 February 2010; the statement is a claim, and 10 February 2010 is pinned as never a
  boundary (the independent check recommended claims only; Codex to rule).
- **Leaves and handovers.** Dirceu's leaves of 17 July 2001 and 7 December 2002, and Genoino's declaration of
  9 July 2005 (which both hands the office to the Diretório Nacional and calls the act 'uma licença'), end nothing.
- **Resignations without a day.** Dirceu's resignation is dated only by the Diretório Nacional meeting 'em 15 e
  16/03/2003' in a retrospective compilation citing minutes not seen; Gleisi Hoffmann's resignation is cited only as
  the reason for the designation of 7 March 2025, and the PT's pages of 13 and 20 March say Humberto Costa took over
  early in the following week.
- **Prospective days.** The posse window of 28 November-10 December 2013, the posse scheduled for the night of
  12 December 2013, the PED 2009 rule's 20 February 2010, the posse announced for 3 August 2025 and the official
  assumption announced for Monday 4 August 2025 are text only.
- **Registry periods.** The TSE's SGIP records give exercise periods that follow each organ's registered term (for
  example Rui Falcão to 8 September 2017, after Gleisi Hoffmann's posse, and Edinho Silva from 23 August 2025); they
  are claims only.

### Acting and interim service

Acting (`em exercício`) and interim (`interino`) service is recorded only as claims, with the role title
'Presidente Nacional do Partido dos Trabalhadores em exercício ou interino', and never makes, splits or ends a holder:
José Genoino during Dirceu's campaign leave (July-October 2001) and after Dirceu's leave of 7 December 2002 (the party
compilation's 'presidente interino' to 15 March 2003); Marco Aurélio Garcia from November 2006 to January 2007; Rui
Falcão on 26 April 2011; and Humberto Costa from 7 to 19 March 2025. Because the party compilation records Genoino's
service from 7 December 2002 to 15 March 2003 as interim, the Portal do PT's stylings of 8 and 13 January 2003 are
claims of kind `styled_president_during_interim_period`, and his holder is dated after his election by the Diretório
Nacional on 15 March 2003. Humberto Costa's service is interim only until 20 March 2025, when the Diretório ratified
his choice and he said he left 'a condição de presidente interino para presidente efetivo'; from then on he is a
holder, and the stylings of 7 and 16 July 2025 are continuation claims.

### Party office and the Presidency of the Republic

`br_pt_president` sits on the PT funding observation; the Presidency of the Republic stays in `br_presidency`. No
`br_presidency` claim or source feeds the party role, and no party claim or source feeds `br_presidency`: every claim
and source id of this packet begins `br_pt_`, and none of the presidency's does. The CLAUDE-C01-10 and CLAUDE-C01-17
holders are pinned unchanged by this packet's test, and Lula's `br_president` holders (2003, 2007, 2023) never touch his
party holder of 1990. Ministerial appointments (Dirceu to the Casa Civil, Gleisi Hoffmann to Institutional Relations)
and candidacies for the Presidency of the Republic are state-office context and never start, end or split a party
holder. The existing `test_brazil_research_s10f.py` guard that no organization has roles is re-expressed as an exact
pin: only the PT observation has a role, and it is exactly `br_pt_president`.

### Date ledger

Each row is a separate dated fact with its own claim; two facts on one day stay two claims. Holder fields are marked where a claim dates a holder. Claims with no structured date are listed by observation at the end.

| Date | Events | Claims and holder fields |
|---|---|---|
| 3 Jun 1990 | Lula styled President in office; Lula states that the office is assumed that day; handover from Gushiken | `br_pt_bn051_lula_reconducted_closing_speech_19900603`, `br_pt_bn051_lula_states_he_assumes_presidency_19900603`, `br_pt_bn051_lula_receives_baton_from_gushiken_19900603`; Lula `from` |
| 15 Jul 1990 | executive committee elected, headed by Lula | `br_pt_fpa_cen_elected_presidente_lula_19900715` |
| 13 Jul 1992 | executive committee altered, headed by Lula | `br_pt_fpa_cen_altered_presidente_lula_19920713` |
| 13 Jun 1993 | executive committee elected, headed by Lula | `br_pt_fpa_cen_elected_presidente_lula_19930613` |
| 25 Nov 1994 | Rui Falcão styled President in office | `br_pt_bnee94_rui_falcao_opens_seminar_as_president_19941125`; Rui Falcão `attested_on` |
| 24 Aug 1995 | Dirceu styled President in office | `br_pt_sncr_oficio_addressed_to_president_dirceu_19950824`; Dirceu `attested_on` |
| 31 Aug 1997 | vote for the presidency won by Dirceu | `br_pt_ptn051_presidential_vote_dirceu_284_temer_256_19970831` |
| 17 Sep 1997 | Dirceu styled President in office | `br_pt_ptn052_dirceu_national_president_at_vitoria_meeting_19970917`; Dirceu `attested_on` |
| 28 Nov 1999 | Dirceu declared elected | `br_pt_ptn087_result_known_dirceu_reelected_19991128` |
| 2 Dec 1999 | Dirceu styled President in office | `br_pt_presidencia_letter_signed_dirceu_19991202`; Dirceu `attested_on` |
| 17 Jul 2001 | Dirceu on leave | `br_pt_ptn105_dirceu_leave_to_run_for_reelection_20010717` |
| 3 Aug 2001 | Genoino acting (em exercício) | `br_pt_ptn106_genoino_signs_note_as_acting_president_20010803` |
| 16 Aug 2001 | Genoino acting (em exercício) | `br_pt_ptn107_genoino_signs_note_as_acting_president_20010816` |
| 1 Sep 2001 | Genoino acting (em exercício) | `br_pt_ptn108_genoino_acting_national_president_20010901` |
| 16 Sep 2001 | direct-election (PED) vote | `br_pt_ptn109_ped_vote_held_20010916` |
| 20 Sep 2001 | totalisation note | `br_pt_ped_commission_note_totalization_failures_20010920` |
| 27 Sep 2001 | Dirceu declared elected | `br_pt_ptn109_ped_totalization_dirceu_elected_20010927` |
| 1 Oct 2001 | Dirceu resumes after the leave | `br_pt_ptn109_dirceu_resumed_party_leadership_20011001` |
| 4 Oct 2001 | Dirceu styled President in office | `br_pt_ptn109_dirceu_signs_note_as_president_20011004`; Dirceu `attested_on` |
| 6 Oct 2001 | Dirceu styled President (continuation) | `br_pt_ptn109_dirceu_signs_note_as_president_20011006` |
| 10 Oct 2001 | Dirceu styled President (continuation) | `br_pt_ptn109_dirceu_signs_official_note_as_president_20011010` |
| 13 Dec 2001 | Dirceu styled President (continuation) | `br_pt_ptn_ee12_dirceu_presidente_nacional_at_cultural_opening_20011213` |
| 7 Dec 2002 | Genoino interim substitute; Diretório acclaims Genoino | `br_pt_fpa_dn_genoino_substitutes_dirceu_interim_20021207`, `br_pt_ptn124_dn_acclaims_genoino_replacing_dirceu_20021207` |
| 8 Jan 2003 | Genoino styled President during the interim period | `br_pt_genoino_styled_president_interim_period_20030108` |
| 13 Jan 2003 | Genoino styled President during the interim period | `br_pt_genoino_signs_official_note_interim_period_20030113` |
| 15 Mar 2003 | Diretório elects Genoino | `br_pt_fpa_dn_genoino_elected_national_president_20030315` |
| 18 Mar 2003 | Genoino styled President in office | `br_pt_genoino_attends_affiliation_as_pt_president_20030318`; Genoino `attested_on` |
| 9 Jul 2005 | Genoino announces his departure; Genoino hands the office to the Diretório; Genoino calls it a leave; reactions to Genoino's request; Diretório approves Tarso Genro; 'novos empossados' until the PED (names nobody); Tarso Genro styled new President on the selection day | `br_pt_genoino_departure_announcement_reported_20050709`, `br_pt_genoino_hands_office_to_dn_20050709`, `br_pt_genoino_describes_act_as_leave_20050709`, `br_pt_genoino_departure_request_reported_20050709`, `br_pt_dn_approves_tarso_genro_for_presidency_20050709`, `br_pt_empossados_until_ped_statement_20050709`, `br_pt_tarso_styled_new_president_press_conference_20050709` |
| 10 Jul 2005 | Tarso Genro styled President in office | `br_pt_tarso_reports_as_president_after_cen_meeting_20050710`; Tarso Genro `attested_on` |
| 18 Sep 2005 | PED ballot | `br_pt_ped2005_first_round_ballot_20050918` |
| 19 Sep 2005 | Tarso Genro styled President (continuation); PED partial count; PED rule on the posse amended | `br_pt_tarso_styled_president_releasing_ped_bulletin_20050919`, `br_pt_ped2005_first_round_partial_berzoini_leads_20050919`, `br_pt_cen_amends_ped_rule_on_posse_20050919` |
| 9 Oct 2005 | PED ballot | `br_pt_ped2005_second_round_ballot_20051009` |
| 10 Oct 2005 | PED partial count | `br_pt_ped2005_second_round_first_partial_20051010` |
| 11 Oct 2005 | Berzoini called president-elect | `br_pt_ped2005_coordinator_says_berzoini_president_elect_20051011` |
| 13 Oct 2005 | Berzoini declared or proclaimed elected; assumption scheduled (prospective); Tarso Genro styled President (continuation) | `br_pt_ped2005_berzoini_elected_declared_20051013`, `br_pt_ped2005_leadership_to_take_office_20051022_prospective_20051013`, `br_pt_tarso_styled_president_berzoini_virtual_elect_20051013` |
| 14 Oct 2005 | Tarso Genro styled President (continuation) | `br_pt_tarso_styled_national_president_20051014` |
| 22 Oct 2005 | posse of Berzoini; Berzoini styled President in office | `br_pt_dn_gives_posse_to_berzoini_20051022`, `br_pt_new_cen_lists_berzoini_presidency_20051022`; Berzoini `from` |
| 28 Apr 2006 | Berzoini styled President (continuation) | `br_pt_berzoini_styled_national_president_20060428` |
| 27 Nov 2006 | Marco Aurélio Garcia interim President | `br_pt_marco_aurelio_garcia_interim_president_listed_20061127` |
| 26 Dec 2006 | Marco Aurélio Garcia interim President | `br_pt_dn_guidelines_name_president_marco_aurelio_garcia_20061226` |
| 3 Jan 2007 | Marco Aurélio Garcia interim President | `br_pt_marco_aurelio_garcia_interim_president_listed_20070103` |
| 5 Mar 2007 | Berzoini styled President (continuation) | `br_pt_berzoini_styled_national_president_20070305` |
| 13 Mar 2007 | Berzoini styled President (continuation) | `br_pt_berzoini_listed_president_cen_page_20070313` |
| 16 Dec 2007 | Berzoini styled President (continuation); PED ballot | `br_pt_berzoini_styled_president_and_candidate_ped2007_20071216`, `br_pt_ped2007_second_round_ballot_20071216` |
| 19 Dec 2007 | executive salutes PED participation | `br_pt_cen_salutes_ped2007_participation_20071219` |
| 13 Nov 2009 | Berzoini styled President in office | `br_pt_berzoini_listed_president_cen_biennium_2008_2009_20091113`; Berzoini `attested_on` |
| 22 Nov 2009 | PED ballot | `br_pt_ped2009_ballot_20091122` |
| 24 Nov 2009 | Berzoini styled President (continuation); PED partial count | `br_pt_berzoini_styled_national_president_20091124`, `br_pt_ped2009_partial_count_dutra_leads_20091124` |
| 25 Nov 2009 | Dutra declared or proclaimed elected; assumption scheduled (prospective) | `br_pt_dutra_declared_president_elect_20091125`, `br_pt_dutra_posse_scheduled_february_2010_20091125` |
| 2 Dec 2009 | Dutra final result | `br_pt_ped2009_final_result_dutra_20091202` |
| 7 Dec 2009 | PED rule on the posse amended | `br_pt_cen_amends_ped2009_posse_rule_20091207` |
| 10 Feb 2010 | Berzoini: last day of his mandate (claim only); Berzoini's farewell speech reported | `br_pt_berzoini_states_last_day_of_mandate_20100210`, `br_pt_portal_reports_berzoini_farewell_speech_20100210` |
| 19 Feb 2010 | assumption scheduled (prospective); posse of Dutra; Berzoini 'deixa a presidência' | `br_pt_iv_congress_posse_scheduled_evening_20100219`, `br_pt_dutra_empossado_president_20100219`, `br_pt_berzoini_leaves_presidency_statement_20100219`; Dutra `from` |
| 20 Feb 2010 | Dutra styled President (continuation) | `br_pt_new_cen_lists_dutra_president_20100220` |
| 26 Apr 2011 | Rui Falcão acting (em exercício); Dutra styled President (continuation) | `br_pt_falcao_styled_acting_president_20110426`, `br_pt_dutra_styled_president_pending_decision_20110426` |
| 29 Apr 2011 | Dutra resigns | `br_pt_dutra_resignation_20110429`; Dutra `until` |
| 8 May 2011 | Rui Falcão styled President in office | `br_pt_falcao_styled_new_national_president_20110508`; Rui Falcão `attested_on` |
| 15 Jun 2011 | Rui Falcão styled President (continuation) | `br_pt_falcao_styled_president_20110615` |
| 10 Nov 2013 | direct-election (PED) vote | `br_pt_ped2013_vote_held_20131110` |
| 12 Nov 2013 | Rui Falcão announced winner on a partial count; Rui Falcão styled President (continuation) | `br_pt_ped2013_falcao_reelection_announced_20131112`, `br_pt_falcao_styled_national_president_20131112` |
| 18 Nov 2013 | posse window resolved | `br_pt_dn_resolution_posse_window_20131118` |
| 10 Dec 2013 | posse scheduled (prospective) | `br_pt_guia_schedules_president_posse_20131210` |
| 11 Dec 2013 | executive committee elected, headed by Rui Falcão | `br_pt_dn_elects_and_inducts_cen_falcao_president_20131211` |
| 12 Dec 2013 | Rui Falcão posse speech published | `br_pt_falcao_posse_speech_published_20131212` |
| 18 Dec 2013 | Rui Falcão styled President in office | `br_pt_dn_composition_lists_falcao_president_20131218`; Rui Falcão `attested_on` |
| 12 Jun 2015 | Rui Falcão styled President (continuation) | `br_pt_falcao_styled_national_president_5th_congress_20150612` |
| 5 Aug 2016 | Rui Falcão styled President (continuation) | `br_pt_dn_page_falcao_presidente_nacional_20160805` |
| 3 Jun 2017 | Congress elects Gleisi Hoffmann | `br_pt_gleisi_elected_6th_congress_20170603` |
| 5 Jul 2017 | posse of Gleisi Hoffmann; Gleisi Hoffmann styled President in office | `br_pt_gleisi_posse_20170705`, `br_pt_gleisi_posse_reported_lula_20170705`; Gleisi Hoffmann `from` |
| 25 Feb 2018 | Gleisi Hoffmann styled President (continuation) | `br_pt_dn_page_gleisi_presidenta_20180225` |
| 24 Nov 2019 | Congress re-elects Gleisi Hoffmann | `br_pt_gleisi_reelected_7th_congress_20191124`, `br_pt_gleisi_reelected_text_20191124`, `br_pt_7th_congress_reelects_gleisi_20191124` |
| 1 Apr 2020 | Gleisi Hoffmann styled President in office | `br_pt_dn_page_gleisi_presidenta_20200401`; Gleisi Hoffmann `attested_on` |
| 28 Feb 2025 | Gleisi Hoffmann styled President (continuation) | `br_pt_gleisi_styled_presidenta_nacional_20250228` |
| 7 Mar 2025 | Gleisi Hoffmann's departure announced; Humberto Costa designated interim; Humberto Costa's interim designation recited | `br_pt_gleisi_departure_announced_20250307`, `br_pt_humberto_interim_designated_executive_20250307`, `br_pt_dn_page_humberto_temporary_designation_20250307`, `br_pt_humberto_interim_confirmed_recited_20250307` |
| 8 Mar 2025 | Humberto Costa interim President | `br_pt_dn_page_humberto_heading_20250308` |
| 12 Mar 2025 | Humberto Costa interim President | `br_pt_humberto_styled_national_president_interim_20250312` |
| 13 Mar 2025 | Humberto Costa interim President | `br_pt_humberto_styled_interim_national_president_20250313` |
| 20 Mar 2025 | Diretório elects Humberto Costa; Diretório ratifies Humberto Costa; Humberto Costa states that the office is assumed that day | `br_pt_dn_elects_humberto_20250320`, `br_pt_dn_ratifies_humberto_choice_20250320`, `br_pt_humberto_states_interim_to_effective_20250320`; Humberto Costa `from` |
| 7 Jul 2025 | Edinho Silva announced winner on a partial count; Humberto Costa styled President (continuation) | `br_pt_ped2025_result_announced_20250707`, `br_pt_humberto_styled_president_at_announcement_20250707` |
| 15 Jul 2025 | Edinho Silva totalization | `br_pt_ped2025_totalization_20250715` |
| 16 Jul 2025 | Edinho Silva styled president-elect; Humberto Costa styled President (continuation) | `br_pt_edinho_styled_president_elect_20250716`, `br_pt_humberto_styled_current_president_20250716` |
| 23 Jul 2025 | posse scheduled (prospective) | `br_pt_posse_scheduled_notice_20250723` |
| 3 Aug 2025 | Encontro confirms Edinho Silva; Edinho Silva to assume officially on 4 August (prospective); posse of Edinho Silva; tribute to Humberto Costa | `br_pt_encontro_confirms_edinho_20250803`, `br_pt_edinho_official_assumption_announced_for_20250804`, `br_pt_edinho_empossado_20250803`, `br_pt_edinho_speech_thanks_humberto_20250803`; Edinho Silva `from` |
| 17 Sep 2025 | Edinho Silva styled President (continuation) | `br_pt_dn_page_edinho_presidente_20250917` |
| 15 Aug 2026 | Edinho Silva styled President in office | `br_pt_dn_page_edinho_presidente_nacional_20260815`; Edinho Silva `attested_on` |
| undated (PT-PRES-01) | spans, month or issue dates, meetings of several days, registry periods and retrospective statements (no structured date) | `br_pt_fpa_cen_altered_presidente_gushiken_19881210_11`, `br_pt_fpa_dn5_elected_7th_encontro_presidente_lula_19900601_03`, `br_pt_fpa_dn6_elected_8th_encontro_presidente_lula_19930611_13`, `br_pt_bn049_gushiken_presidente_nacional_meeting_19900216_18`, `br_pt_bn049_gushiken_signs_official_note_haiti_199003`, `br_pt_bn050_gushiken_presidente_nacional_theses_199005`, `br_pt_bn051_vii_encontro_reconducts_lula_19900531_0603`, `br_pt_csbh_resumo_vii_encontro_elects_5th_dn_19900531_0603`, `br_pt_bn085_lula_styled_presidente_do_pt_199404`, `br_pt_bn091_gushiken_former_national_president_199410`, `br_pt_ptn124_retrospective_fifth_president_count_200212`, `br_pt_camara_bio_gushiken_presidente_dn_1988_1990` |
| undated (PT-PRES-02) | spans, month or issue dates, meetings of several days, registry periods and retrospective statements (no structured date) | `br_pt_fpa_dn7_elected_10th_encontro_presidente_dirceu_19950818_20`, `br_pt_fpa_cen_approved_presidente_dirceu_19951028_29`, `br_pt_bn085_rui_falcao_vice_president_199404`, `br_pt_csbh_resumo_ix_encontro_19940429_0501`, `br_pt_bn087_editorial_signed_rui_falcao_presidente_nacional_199405`, `br_pt_bn091_editorial_signed_rui_falcao_presidente_nacional_199410`, `br_pt_site_bio_rui_falcao_president_in_1994`, `br_pt_csbh_resumo_x_encontro_presidential_vote_19950818_20` |
| undated (PT-PRES-03) | spans, month or issue dates, meetings of several days, registry periods and retrospective statements (no structured date) | `br_pt_fpa_cen_altered_early_1997_presidente_dirceu`, `br_pt_fpa_dn8_elected_11th_encontro_presidente_dirceu_19970828_30`, `br_pt_fpa_cen_elected_presidente_dirceu_19970920_21`, `br_pt_fpa_cen_1999_2001_presidente_dirceu`, `br_pt_fpa_dn10_elected_12th_encontro_presidente_dirceu_20011214_16`, `br_pt_fpa_dn_dirceu_resigns_national_presidency_20030315_16`, `br_pt_fpa_cen_dirceu_to_federal_government_printed_20020110`, `br_pt_csbh_resumo_xi_encontro_19970828_30`, `br_pt_ptn051_xi_encontro_reconducts_dirceu_19970829_31`, `br_pt_ptn051_dirceu_proclaimed_national_president_199708`, `br_pt_ptn051_masthead_presidente_nacional_dirceu_19970910_16`, `br_pt_ptn052_masthead_presidente_nacional_dirceu_19970920_26`, `br_pt_ptn087_ii_congresso_held_19991124_28`, `br_pt_ptn087_dirceu_assumed_third_term_during_congresso_199911`, `br_pt_ptn087_masthead_presidente_nacional_dirceu_19991216_20000105`, `br_pt_ptn105_masthead_genoino_em_exercicio_20010720_0803`, `br_pt_ptn107_dirceu_signs_as_president_on_leave_200108`, `br_pt_ptn108_masthead_genoino_em_exercicio_20010910_25`, `br_pt_ptn109_masthead_genoino_em_exercicio_20011010_25`, `br_pt_ptn109_interview_dirceu_reassumed_presidency_200110`, `br_pt_ptn124_dirceu_on_leave_for_lula_government_200212`, `br_pt_camara_bio_dirceu_presidente_pt_1995_1997_1998_2002` |
| undated (PT-PRES-04) | spans, month or issue dates, meetings of several days, registry periods and retrospective statements (no structured date) | `br_pt_genoino_recalls_30_months_in_presidency` |
| undated (PT-PRES-05) | spans, month or issue dates, meetings of several days, registry periods and retrospective statements (no structured date) | `br_pt_berzoini_recalls_leading_first_interim_2005` |
| undated (PT-PRES-06) | spans, month or issue dates, meetings of several days, registry periods and retrospective statements (no structured date) | `br_pt_deputy_aparte_berzoini_president_2005_2007_2007_2010`, `br_pt_dutra_posse_speech_lists_past_pt_presidents` |
| undated (PT-PRES-07) | spans, month or issue dates, meetings of several days, registry periods and retrospective statements (no structured date) | `br_pt_falcao_elected_unanimously_2011`, `br_pt_tse_sgip_falcao_dn_registered_exercise_20140219_20170908`, `br_pt_tse_sgip_falcao_cen_registered_exercise_20140219_20171009`, `br_pt_falcao_balance_retrospective_span` |
| undated (PT-PRES-08) | spans, month or issue dates, meetings of several days, registry periods and retrospective statements (no structured date) | `br_pt_tse_sgip_gleisi_registered_exercise_20170909_20200117`, `br_pt_tse_sgip_gleisi_registered_exercise_20200117_20250307`, `br_pt_dn_page_gleisi_resignation_referenced` |
| undated (PT-PRES-09) | spans, month or issue dates, meetings of several days, registry periods and retrospective statements (no structured date) | `br_pt_tse_sgip_humberto_registered_president_20250307_20250823`, `br_pt_humberto_assumed_interim_early_in_week`, `br_pt_humberto_assumed_previous_week_recital`, `br_pt_dn_page_edinho_ped_election_recited` |

Date conventions follow CLAUDE-C01-10 and CLAUDE-C01-17: `attested_on` is the day of the observed event as the source
dates it; a day printed without a month or year takes them from the issue or item that prints it, and the claim says
so; a newspaper issue's span, a month, a meeting of several days and a retrospective statement carry no structured
date (the printed words stay in the claim text); an undated web page is dated by its archive capture only where it is
the attestation itself; no clock time is stored.

## Observations

### PT-PRES-01 — The President when the period opens, and the end of Lula's presidency

Evidence: the party's compilation of directorate compositions (Fundação Perseu Abramo) records the national executive
committee altered at the Diretório Nacional meeting of 10-11 December 1988 with Luiz Gushiken as President
(`br_pt_fpa_cen_altered_presidente_gushiken_19881210_11`). The Boletim Nacional, organ of the national executive
committee, calls 'Luís Gushiken' the national president at a meeting of 16-18 February 1990, signing an official note
(March 1990) and presenting the theses for the VII Encontro (May 1990)
(`br_pt_bn049_gushiken_presidente_nacional_meeting_19900216_18`, `br_pt_bn049_gushiken_signs_official_note_haiti_199003`,
`br_pt_bn050_gushiken_presidente_nacional_theses_199005`). The VII Encontro (31 May-3 June 1990) returned ('reconduzir')
Lula to the presidency (`br_pt_bn051_vii_encontro_reconducts_lula_19900531_0603`,
`br_pt_fpa_dn5_elected_7th_encontro_presidente_lula_19900601_03`, `br_pt_csbh_resumo_vii_encontro_elects_5th_dn_19900531_0603`).
In his closing speech on the night of Sunday 3 June 1990, printed by Boletim Nacional nº 51, Lula says 'Volto a assumir
a Presidência do Partido dos Trabalhadores' and addresses Gushiken, 'de quem eu recebo o bastão hoje'
(`br_pt_bn051_lula_states_he_assumes_presidency_19900603`, `br_pt_bn051_lula_receives_baton_from_gushiken_19900603`);
the standfirst calls him 'reconduzido à presidência do PT' (`br_pt_bn051_lula_reconducted_closing_speech_19900603`).
The executive committees of 15 July 1990, 13 July 1992 and 13 June 1993 are headed by him
(`br_pt_fpa_cen_elected_presidente_lula_19900715`, `br_pt_fpa_cen_altered_presidente_lula_19920713`,
`br_pt_fpa_dn6_elected_8th_encontro_presidente_lula_19930611_13`, `br_pt_fpa_cen_elected_presidente_lula_19930613`), and
the bulletin still calls him 'o presidente do PT' in the first fortnight of April 1994
(`br_pt_bn085_lula_styled_presidente_do_pt_199404`). Retrospective records: Gushiken 'foi presidente nacional do PT'
(1994), the Chamber's biography ('Presidente, 1988-1990') and the party newspaper's count of December 2002, which
names Lula, Olívio Dutra, Gushiken, Dirceu and Genoino as its five presidents (`br_pt_bn091_gushiken_former_national_president_199410`,
`br_pt_camara_bio_gushiken_presidente_dn_1988_1990`, `br_pt_ptn124_retrospective_fifth_president_count_200212`; the
count also bears on PT-PRES-02 and PT-PRES-03).

Decision: accepted in part. The hint that Lula was President on 1 January 1990 is not supported: Gushiken was, but no
record gives a single day of his presidency, so he has no holder. Lula's holder starts on 3 June 1990 on his own
stated assumption of that day; if Codex does not accept a speech as a stated assumption, the fallback is attested_on
1990-06-03 on the same claims. No end: no party record states the day or the reason his presidency ended in 1994.

Limits: no party record dated 1 January 1990 names the President (Boletim Nacional nº 48, February 1990, is a lead);
Gushiken's baton speech is a handover statement, never an end for him.

### PT-PRES-02 — 1994-1995: Rui Falcão and José Dirceu's election

Evidence: in the first fortnight of April 1994 the bulletin places contacts with the PSTU 'a cargo dos
vice-presidentes do PT, Rui Falcão e Luiz Eduardo Greenhalgh' (`br_pt_bn085_rui_falcao_vice_president_199404`; check
A9). The 9th Encontro met on 29 April-1 May 1994 (`br_pt_csbh_resumo_ix_encontro_19940429_0501`). Rui Falcão signs the
bulletin's editorials as 'Presidente Nacional do PT' in the second fortnight of May and in October 1994
(`br_pt_bn087_editorial_signed_rui_falcao_presidente_nacional_199405`,
`br_pt_bn091_editorial_signed_rui_falcao_presidente_nacional_199410`). The party's special edition of December 1994,
which has no text layer and was found by the check in page renders, reports that the Seminário Nacional de Avaliação
da Campanha Lula-94 'reuniu no dia 25 de novembro' and that 'O presidente nacional do PT, deputado Rui Falcão, abriu o
Seminário' (`br_pt_bnee94_rui_falcao_opens_seminar_as_president_19941125`; check A2). The party's 2017 biography says
'em 1994, presidente do PT Nacional' (`br_pt_site_bio_rui_falcao_president_in_1994`), and its 2011 portal says he 'já
foi presidente do partido' (`br_pt_falcao_elected_unanimously_2011`). The 10th Encontro (Guarapari, 18-20 August 1995)
held the first contested vote for the presidency: José Dirceu 215, Hamilton Pereira 183, 16 blank, day not printed
(`br_pt_csbh_resumo_x_encontro_presidential_vote_19950818_20`, `br_pt_fpa_dn7_elected_10th_encontro_presidente_dirceu_19950818_20`).
An ofício of the national secretariat against racism dated 24 August 1995 is addressed 'Ao Sr. José Dirceu, Presidente
do Diretório Nacional' (`br_pt_sncr_oficio_addressed_to_president_dirceu_19950824`), and the executive committee of
28-29 October 1995 is headed by him (`br_pt_fpa_cen_approved_presidente_dirceu_19951028_29`).

Decision: accepted in part. Rui Falcão is a holder observed on 25 November 1994, the first day-dated party record of
his presidency: the party's organ styles him Presidente Nacional without 'interino' or 'em exercício'. He has no start
and no end, and whether his service was interim is open (he was a vice-president weeks before; the December 2002 count
omits him; the 2011 and 2017 party records count him). Dirceu is a holder observed on 24 August 1995.

Limits: who led the party from December 1994 to August 1995 is unresolved (Brasil Agora, a company newspaper, is a lead
only); the day of the 1995 vote is not printed.

### PT-PRES-03 — Dirceu's re-elections and departure

Evidence, 1997: PT Notícias nº 51 dates the 11th Encontro 29-31 August 1997 and the vote to the 31st (Dirceu 284, Milton
Temer 256), after which he was 'proclamado presidente nacional do PT' (day not printed); the compilation and the summary
sheet say 28-30 August (`br_pt_ptn051_xi_encontro_reconducts_dirceu_19970829_31`,
`br_pt_ptn051_presidential_vote_dirceu_284_temer_256_19970831`, `br_pt_ptn051_dirceu_proclaimed_national_president_199708`,
`br_pt_fpa_dn8_elected_11th_encontro_presidente_dirceu_19970828_30`, `br_pt_csbh_resumo_xi_encontro_19970828_30`). The
issue's masthead lists 'Presidente Nacional do PT: José Dirceu' (`br_pt_ptn051_masthead_presidente_nacional_dirceu_19970910_16`),
and PT Notícias nº 52 reports him as 'o presidente nacional do PT' at a meeting in Vitória 'No dia 17' of September
1997 (`br_pt_ptn052_dirceu_national_president_at_vitoria_meeting_19970917`; check A4).

1999: at the II Congresso (24-28 November 1999) the result was known on Sunday 28 November (496 votes); he 'assumed his
third term during the Congresso' (no day), and signed a letter as 'Presidente Nacional do Partido dos Trabalhadores' on
2 December 1999 (`br_pt_ptn087_ii_congresso_held_19991124_28`, `br_pt_ptn087_result_known_dirceu_reelected_19991128`,
`br_pt_ptn087_dirceu_assumed_third_term_during_congresso_199911`, `br_pt_presidencia_letter_signed_dirceu_19991202`).

2001: Dirceu 'licenciou-se no dia 17 da presidência nacional do PT para poder disputar a reeleição', announced at the
launch of the direct-election process on 13 July (`br_pt_ptn105_dirceu_leave_to_run_for_reelection_20010717`; check A3);
José Genoino is 'em exercício' in the mastheads of nº 105, 108 and 109 and signs notes as 'presidente em exercício' on
3 and 16 August, and a candidate's letter of 1 September is addressed to him as acting president (claims only), while
Dirceu signs as 'presidente nacional do PT licenciado' (`br_pt_ptn107_dirceu_signs_as_president_on_leave_200108`). The
PED vote took place on 16 September, the totalisation ended on 27 September (113,713 votes, 55.55%), and 'Dirceu
retomou suas atividades na direção do partido no dia 1º de outubro' (`br_pt_ptn109_ped_vote_held_20010916`,
`br_pt_ped_commission_note_totalization_failures_20010920`, `br_pt_ptn109_ped_totalization_dirceu_elected_20010927`,
`br_pt_ptn109_dirceu_resumed_party_leadership_20011001`). He signs dated notes as president on 4, 6 and 10 October
2001 (`br_pt_ptn109_dirceu_signs_note_as_president_20011004`, `br_pt_ptn109_dirceu_signs_note_as_president_20011006`,
`br_pt_ptn109_dirceu_signs_official_note_as_president_20011010`; check A1), and is styled national president at the
12th Encontro's cultural opening on 13 December 2001 (`br_pt_ptn_ee12_dirceu_presidente_nacional_at_cultural_opening_20011213`).

Departure: at the Diretório Nacional meeting of 7 December 2002 José Genoino replaced Dirceu, who took leave to join
the President-elect's government; PT Notícias calls Genoino the new president, and the compilation 'presidente
interino' for 07/12/2002 to 15/03/2003 (`br_pt_ptn124_dn_acclaims_genoino_replacing_dirceu_20021207`,
`br_pt_ptn124_dirceu_on_leave_for_lula_government_200212`, `br_pt_fpa_dn_genoino_substitutes_dirceu_interim_20021207`).
The compilation records Dirceu's resignation of the national presidency at the meeting 'em 15 e 16/03/2003'
(`br_pt_fpa_dn_dirceu_resigns_national_presidency_20030315_16`). The Chamber's biography gives 'Presidente do PT,
1995-1997 e 1998-2002' (`br_pt_camara_bio_dirceu_presidente_pt_1995_1997_1998_2002`; it also bears on PT-PRES-02).

Decision: accepted in part. Three holders: observed on 17 September 1997, 2 December 1999 and 4 October 2001 (the first
fully dated act after the 2001 election; the resumption of 1 October follows a leave and is a claim). The leaves, the
acting and interim service, and the resignation dated only by a two-day meeting are claims; no until.

Limits: nº 109's masthead still lists Genoino 'em exercício' in October 2001, a conflict recorded rather than resolved;
the compilation's '10/01/2002' row listing Dirceu as Casa Civil minister is probably a misprint for 2003.

### PT-PRES-04 — José Genoino's presidency and its end

Evidence: the Portal do PT styles 'o presidente nacional do PT, deputado José Genoíno' on 8 January 2003, and an
official PT note of 13 January 2003 is signed 'José Genoino, Presidente nacional do PT'
(`br_pt_genoino_styled_president_interim_period_20030108`, `br_pt_genoino_signs_official_note_interim_period_20030113`);
both fall in the period the party compilation records as his interim substitution. The compilation records his
election by the Diretório Nacional on 15 March 2003 ('Presidente eleito pela R/DN 15/03/2003')
(`br_pt_fpa_dn_genoino_elected_national_president_20030315`; split from the resignation by check A6). A Portal do PT
item of 18 March 2003, located for this packet among the portal's archived items of 17-26 March 2003, reports a ceremony
'que contou com a presença do presidente do PT, José Genoino' and quotes 'O presidente nacional do PT'
(`br_pt_genoino_attends_affiliation_as_pt_president_20030318`). On 9 July 2005 the portal reports his announcement of
his 'afastamento' and reproduces his declaration: he hands the office to the Diretório Nacional ('eu entrego o cargo';
'eu entreguei o meu cargo'), 'numa licença da condição de presidente do PT', after 'nesses 30 meses'
(`br_pt_genoino_departure_announcement_reported_20050709`, `br_pt_genoino_hands_office_to_dn_20050709`,
`br_pt_genoino_describes_act_as_leave_20050709`, `br_pt_genoino_recalls_30_months_in_presidency`,
`br_pt_genoino_departure_request_reported_20050709`).

Decision: accepted in part. Genoino's holder is observed on 18 March 2003, three days after his election; the January
2003 stylings are claims only, because the party's own compilation (citing the Diretório minutes) records that service
as interim. No start (the election is a claim) and no end (the 2005 act is both a handover and a leave, and no act
accepting a resignation or declaring a vacancy was found).

Limits: the portal's December 2002 items were never archived; the Chamber diary of 19 December 2002, in which other
deputies address him as President of the PT, is a lead (check B10).

### PT-PRES-05 — 2005: Tarso Genro and Ricardo Berzoini

Evidence: on the night of 9 July 2005 the Diretório Nacional approved Tarso Genro's name for the national presidency
(52-0, 21 abstentions), and the same day's portal items call him the new president
(`br_pt_dn_approves_tarso_genro_for_presidency_20050709`, `br_pt_tarso_styled_new_president_press_conference_20050709`);
a sentence 'Os novos empossados devem permanecer nos cargos até 18 de setembro' follows the paragraph on three new
secretaries and names nobody (`br_pt_empossados_until_ped_statement_20050709`; check B1). On 10 July 2005 the portal
reports the executive committee's decisions 'segundo informou o presidente Tarso Genro'
(`br_pt_tarso_reports_as_president_after_cen_meeting_20050710`); he is styled national president again on 19 September,
13 October and 14 October 2005 (`br_pt_tarso_styled_president_releasing_ped_bulletin_20050919`,
`br_pt_tarso_styled_president_berzoini_virtual_elect_20051013`, `br_pt_tarso_styled_national_president_20051014`). The
PED 2005 first round closed at 17h on Sunday 18 September and the second at 17h on Sunday 9 October
(`br_pt_ped2005_first_round_ballot_20050918`, `br_pt_ped2005_second_round_ballot_20051009`; check B11); partial counts,
the coordinator's announcement of 11 October and the party's declaration of 13 October 2005 (51.6%) follow
(`br_pt_ped2005_first_round_partial_berzoini_leads_20050919`, `br_pt_ped2005_second_round_first_partial_20051010`,
`br_pt_ped2005_coordinator_says_berzoini_president_elect_20051011`, `br_pt_ped2005_berzoini_elected_declared_20051013`,
`br_pt_ped2005_leadership_to_take_office_20051022_prospective_20051013`), with the executive's rule of 19 September on
the posse (`br_pt_cen_amends_ped_rule_on_posse_20050919`). 'O Diretório Nacional deu posse, neste sábado (22), ao
presidente eleito, Ricardo Berzoini', and the new executive lists 'Presidência - Ricardo Berzoini'
(`br_pt_dn_gives_posse_to_berzoini_20051022`, `br_pt_new_cen_lists_berzoini_presidency_20051022`). In 2010 Berzoini
recalled leading the party in 2005 'primeiro interinamente, à frente da Secretaria-Geral'
(`br_pt_berzoini_recalls_leading_first_interim_2005`).

Decision: accepted in part. Tarso Genro is a holder observed on 10 July 2005 (the selection day is kept out of holder
dates; no source states a posse or calls him interim); no end is stated, and Berzoini's posse is not used as one.
Berzoini's holder starts on 22 October 2005.

Limits: whether Tarso Genro's presidency was formally interim is open; no final certified count of the 2005 second round
was found.

### PT-PRES-06 — 2006-2010: interim arrangements, Berzoini and José Eduardo Dutra

Evidence: Berzoini is styled national president on 28 April 2006 (`br_pt_berzoini_styled_national_president_20060428`).
The executive committee page lists 'Marco Aurélio Garcia - Presidente (interino)' in captures of 27 November 2006 and
3 January 2007, and the Diretório's guidelines for the III Congresso, published on 26 December 2006, name 'o presidente
Marco Aurélio Garcia' (`br_pt_marco_aurelio_garcia_interim_president_listed_20061127`,
`br_pt_dn_guidelines_name_president_marco_aurelio_garcia_20061226`, `br_pt_marco_aurelio_garcia_interim_president_listed_20070103`).
Berzoini is again styled and listed President on 5 and 13 March 2007 and voted as incumbent and candidate in the PED 2007
second round on 16 December 2007 (`br_pt_berzoini_styled_national_president_20070305`,
`br_pt_berzoini_listed_president_cen_page_20070313`, `br_pt_berzoini_styled_president_and_candidate_ped2007_20071216`,
`br_pt_ped2007_second_round_ballot_20071216`); the executive's resolution of 19 December 2007 names no winner
(`br_pt_cen_salutes_ped2007_participation_20071219`). The portal lists him President of the executive 'para o biênio
2008/2009' (capture of 13 November 2009) and styles him on 24 November 2009
(`br_pt_berzoini_listed_president_cen_biennium_2008_2009_20091113`, `br_pt_berzoini_styled_national_president_20091124`).
PED 2009: ballot 22 November, Dutra declared president-elect on 25 November and final result released on 2 December
2009 (`br_pt_ped2009_ballot_20091122`, `br_pt_ped2009_partial_count_dutra_leads_20091124`,
`br_pt_dutra_declared_president_elect_20091125`, `br_pt_dutra_posse_scheduled_february_2010_20091125`,
`br_pt_ped2009_final_result_dutra_20091202`); the executive set the posse for 20 February 2010
(`br_pt_cen_amends_ped2009_posse_rule_20091207`). In the Chamber on 10 February 2010 Berzoini said 'hoje é o último dia
do meu mandato como Presidente Nacional do Partido dos Trabalhadores', and a deputy called him President 'de 2005 a
2007 e de 2007 a 2010'; the portal reported the farewell the next day (`br_pt_berzoini_states_last_day_of_mandate_20100210`,
`br_pt_deputy_aparte_berzoini_president_2005_2007_2007_2010`, `br_pt_portal_reports_berzoini_farewell_speech_20100210`;
check B8). On the night of 19 February 2010 'foram empossados o novo presidente José Eduardo Dutra' and the new
Diretório, when Berzoini 'deixa a presidência'; Dutra's posse speech named the party's past presidents; the new
executive of 20 February lists 'Presidente: José Eduardo Dutra' (`br_pt_iv_congress_posse_scheduled_evening_20100219`,
`br_pt_dutra_empossado_president_20100219`, `br_pt_berzoini_leaves_presidency_statement_20100219`,
`br_pt_dutra_posse_speech_lists_past_pt_presidents`, `br_pt_new_cen_lists_dutra_president_20100220`).

Decision: accepted in part. Garcia's interim service is claims only and neither splits nor ends Berzoini's 2005 holder.
A second Berzoini holder is observed on 13 November 2009 (the 2008/2009 term; no PT record of the 2007 result or a 2008
posse). Berzoini's stated last day conflicts with the party's report of 19 February, so it is a claim and never an
until. Dutra's holder starts on 19 February 2010 and ends on 29 April 2011 (PT-PRES-07).

Limits: the day Berzoini took leave in 2006 and resumed in 2007, and the PED 2007 result, are open.

### PT-PRES-07 — 2011-2017: Rui Falcão

Evidence: on 26 April 2011 the portal calls Rui Falcão 'presidente em exercício do PT' while Dutra, on health leave,
is still 'presidente do PT' (`br_pt_falcao_styled_acting_president_20110426`,
`br_pt_dutra_styled_president_pending_decision_20110426`). On 8 May 2011 the portal says 'A renúncia do presidente do
PT José Eduardo Dutra no dia 29 de abril foi muito sentida no partido', reports Falcão's election by unanimity (no day)
and quotes the secretary-general on 'o novo presidente nacional do PT, Rui Falcão' (`br_pt_dutra_resignation_20110429`,
`br_pt_falcao_elected_unanimously_2011`, `br_pt_falcao_styled_new_national_president_20110508`; check C4); he is styled
President on 15 June 2011 (`br_pt_falcao_styled_president_20110615`). PED 2013: vote on 10 November, re-election
announced on 12 November (`br_pt_ped2013_vote_held_20131110`, `br_pt_ped2013_falcao_reelection_announced_20131112`,
`br_pt_falcao_styled_national_president_20131112`); the Diretório set a posse window of 28 November-10 December
(`br_pt_dn_resolution_posse_window_20131118`), the congress guide scheduled the posse of the national President for the
night of 12 December (`br_pt_guia_schedules_president_posse_20131210`), the Diretório elected and inducted the new
executive under 'Presidente Rui Falcão' on 11 December (`br_pt_dn_elects_and_inducts_cen_falcao_president_20131211`),
the party published his posse speech on 12 December (`br_pt_falcao_posse_speech_published_20131212`), and on
18 December listed the leaders 'empossado durante o 5º Congresso', headed 'Rui Goethe da Costa Falcão, Presidente'
(`br_pt_dn_composition_lists_falcao_president_20131218`; check C3). He is styled President in June 2015 and August
2016 (`br_pt_falcao_styled_national_president_5th_congress_20150612`, `br_pt_dn_page_falcao_presidente_nacional_20160805`);
the TSE registry gives registry periods from 19 February 2014 (`br_pt_tse_sgip_falcao_dn_registered_exercise_20140219_20170908`,
`br_pt_tse_sgip_falcao_cen_registered_exercise_20140219_20171009`); a 2017 balance says 'entre abril de 2011 e maio de
2017' (`br_pt_falcao_balance_retrospective_span`).

Decision: accepted in part. Dutra's until is 29 April 2011, the day the party states he resigned. Falcão's two holders
are observed on 8 May 2011 and 18 December 2013; the acting styling of 26 April 2011 is a claim; no start or end is
stated. The 2015 re-election in the hint is not supported.

Limits: the PT's own article on Falcão's 2011 election has no usable capture; the Diretório meeting is dated 29 or
30 April 2011.

### PT-PRES-08 — 2017-2025: Gleisi Hoffmann

Evidence: the 6th Congress elected Gleisi Hoffmann on 3 June 2017 (61.89%) (`br_pt_gleisi_elected_6th_congress_20170603`);
on 5 July 2017 the PT reported that the new Diretório 'está sendo empossado hoje' with the new presidenta, and a
same-day report speaks of her posse (`br_pt_gleisi_posse_20170705`, `br_pt_gleisi_posse_reported_lula_20170705`). She
heads the Diretório page in February 2018 (`br_pt_dn_page_gleisi_presidenta_20180225`) and the TSE registry records
2017-2020 and 2020-2025 periods (`br_pt_tse_sgip_gleisi_registered_exercise_20170909_20200117`,
`br_pt_tse_sgip_gleisi_registered_exercise_20200117_20250307`). The 7th Congress re-elected her on 24 November 2019
(`br_pt_gleisi_reelected_7th_congress_20191124`, `br_pt_gleisi_reelected_text_20191124`,
`br_pt_7th_congress_reelects_gleisi_20191124`; check C missing records); the Diretório page of 1 April 2020 heads its
list 'Presidenta Nacional do PT - Gleisi Hoffmann' (`br_pt_dn_page_gleisi_presidenta_20200401`), and on 28 February
2025 the party calls her 'a presidenta nacional do partido' (`br_pt_gleisi_styled_presidenta_nacional_20250228`). On
7 March 2025 the executive met after her departure was announced (`br_pt_gleisi_departure_announced_20250307`), and
the Diretório page cites her 'renúncia' as the reason for Humberto Costa's designation that day
(`br_pt_dn_page_gleisi_resignation_referenced`).

Decision: accepted in part. Her holder starts on 5 July 2017; the 2019 term is observed on 1 April 2020. No end: no
source states the day her office ended, and the reviewer option of an until on 7 March 2025 is withdrawn (check C5).

Limits: no posse day of the 2019 term was found.

### PT-PRES-09 — 2025: the interim President and Edinho Silva

Evidence: on Friday 7 March 2025 the executive named Humberto Costa, a vice-president, for the interim command until
the PED (`br_pt_humberto_interim_designated_executive_20250307`, `br_pt_dn_page_humberto_temporary_designation_20250307`,
`br_pt_humberto_interim_confirmed_recited_20250307`); he is 'presidente interino' on 8, 12 and 13 March and assumed
'no início da semana' (`br_pt_dn_page_humberto_heading_20250308`, `br_pt_humberto_styled_national_president_interim_20250312`,
`br_pt_humberto_styled_interim_national_president_20250313`, `br_pt_humberto_assumed_interim_early_in_week`,
`br_pt_humberto_assumed_previous_week_recital`). On Thursday 20 March 2025 the Diretório elected him (63-0-5) and
ratified the choice, and he said 'agora saio da condição de presidente interino para presidente efetivo'
(`br_pt_dn_elects_humberto_20250320`, `br_pt_dn_ratifies_humberto_choice_20250320`,
`br_pt_humberto_states_interim_to_effective_20250320`; check C1); he is 'presidente do PT' on 7 and 16 July 2025
(`br_pt_humberto_styled_president_at_announcement_20250707`, `br_pt_humberto_styled_current_president_20250716`), and
the TSE registry records a registry period (`br_pt_tse_sgip_humberto_registered_president_20250307_20250823`). PED 2025:
the vote of 6 July (recited), the winner announced on a partial count on 7 July, the totalization of 15 July ('180
EDINHO', 73.1%) and the president-elect styling of 16 July (`br_pt_dn_page_edinho_ped_election_recited`,
`br_pt_ped2025_result_announced_20250707`, `br_pt_ped2025_totalization_20250715`, `br_pt_edinho_styled_president_elect_20250716`);
the posse was announced for 3 August (`br_pt_posse_scheduled_notice_20250723`). On 3 August 2025 the PT reported at
13h27 that the Encontro 'confirmou o nome de Edinho Silva' and that he 'assume oficialmente o cargo na próxima
segunda-feira (4)' (`br_pt_encontro_confirms_edinho_20250803`, `br_pt_edinho_official_assumption_announced_for_20250804`;
check C2), and at 19h53 that he 'foi empossado neste domingo (3)' (`br_pt_edinho_empossado_20250803`); in his speech he
thanked Humberto Costa (`br_pt_edinho_speech_thanks_humberto_20250803`). The Diretório page of 17 September 2025 heads
its list with him (`br_pt_dn_page_edinho_presidente_20250917`).

Decision: accepted in part. Humberto Costa's interim service (7-19 March 2025) is claims only; from 20 March 2025 he is a
holder, on his statement of that day (Codex may prefer attested_on 2025-03-20). Edinho Silva's holder starts on the
stated posse of 3 August 2025; the same-day announcement of an official assumption on Monday 4 August is prospective
and unconfirmed, recorded as a claim and pinned as never a boundary. If Codex prefers it, the fallback is attested_on
2025-09-17. No end for Humberto Costa is stated.

Limits: the TSE registry's 23 August 2025 dates follow the organ's term, not the posse.

### PT-PRES-10 — An attestation before the cutoff

Evidence: the PT's Diretório Nacional page, archived on 15 August 2026, heads its list 'Presidente Nacional do PT -
Edinho Silva - São Paulo' (`br_pt_dn_page_edinho_presidente_nacional_20260815`).

Decision: accepted. A holder observed on 15 August 2026, a second observation of the term begun on 3 August 2025. No
interim arrangement, leave or vacancy was found between 3 August 2025 and the cutoff.

Limits: the TSE's SGIP record of the national organ in force (570664; last altered 8 July 2026, listing him as
PRESIDENTE, Ativo) is a live record that will change on its next annotation, so it is a lead, not a recorded identity
(check C14).

## Sources added

All 108 sources are new records, appended after CLAUDE-C01-17's in `brazil.json`; no existing source record or extract
is edited. The columns give the source id, what it is, the observations its claims serve and the recorded response
identity (bytes and the first and last characters of the SHA-256; the full values are in each extract and pinned in the
test).

| Source ID | What | PT-PRES | Response identity and provenance |
|---|---|---|---|
| `br_pt_fpa_dn4_composition` | 4º Diretório Nacional (DN) and Comissão Executiva Nacional compositions, 1987-1989 (PT/FPA compilation) | 01 | 73,201 bytes, `5dd0c538…0de694`; party foundation PDF |
| `br_pt_fpa_dn5_composition` | 5º Diretório Nacional (DN) and Comissão Executiva Nacional compositions, 1990-1993 (PT/FPA compilation) | 01 | 24,224 bytes, `ac9d36d6…3b3e0c`; party foundation PDF |
| `br_pt_fpa_dn6_composition` | 6º Diretório Nacional (DN) and Comissão Executiva Nacional compositions, 1993-1995 (PT/FPA compilation) | 01 | 66,843 bytes, `d636ec6d…94a2ed`; party foundation PDF |
| `br_pt_fpa_dn7_composition` | 7º Diretório Nacional (DN) and Comissão Executiva Nacional compositions, 1995-1997 (PT/FPA compilation) | 02, 03 | 80,176 bytes, `e1740c23…117fb5`; party foundation PDF |
| `br_pt_fpa_dn8_composition` | 8º Diretório Nacional (DN) and Comissão Executiva Nacional compositions, 1997-1999 (PT/FPA compilation) | 03 | 66,311 bytes, `2ec679d8…7f8cdf`; party foundation PDF |
| `br_pt_fpa_dn9_composition` | Membros do Diretório Nacional do PT, Gestão 1999/2001, and Comissão Executiva Nacional (1999/2001) (PT/FPA compilation) | 03 | 21,991 bytes, `a2f0a3eb…58eb23`; party foundation PDF |
| `br_pt_fpa_dn10_composition` | 10º Diretório Nacional (DN), Comissão Executiva Nacional (gestão 2002/2005) and recorded changes (PT/FPA compilation) | 03, 04 | 112,487 bytes, `aaf8e614…11cb75`; party foundation PDF |
| `br_pt_bn049_199003` | Boletim Nacional do PT, nº 49 (March 1990) | 01 | 5,871,191 bytes, `8c842d38…f6eac2`; party archive (SIAC) PDF |
| `br_pt_bn050_199005` | Boletim Nacional do PT, nº 50 (May 1990) | 01 | 8,076,668 bytes, `c91b27e1…37bfee`; party archive (SIAC) PDF |
| `br_pt_bn051_199007` | Boletim Nacional do PT, nº 51 (cover 'Julho'; inner page headers 'junho de 1990'), VII Encontro Nacional special | 01 | 6,234,294 bytes, `1674cdef…891c8b`; party archive (SIAC) PDF |
| `br_pt_csbh_resumo_7_encontro_1990` | Resumo do VII Encontro Nacional (ficha-resumo, 31 May-3 June 1990), CSBH dossier '1990 - Dossiê VII Encontro Nacional do PT' | 01 | 201,799 bytes, `df83aaa5…b8b600`; party archive (SIAC) PDF |
| `br_pt_bn085_199404` | Boletim Nacional do PT, nº 85 (1st fortnight of April 1994) | 01, 02 | 1,134,723 bytes, `2cb7d3d1…21dca0`; party archive (SIAC) PDF |
| `br_pt_csbh_resumo_9_encontro_1994` | Resumo do IX Encontro Nacional (29 April-1 May 1994), CSBH dossier '1994 - Dossiê IX Encontro Nacional do PT' | 02 | 188,888 bytes, `291b85bc…47bd6a`; party archive (SIAC) PDF |
| `br_pt_bn087_199405` | Boletim Nacional do PT, nº 87 (2nd fortnight of May 1994) | 02 | 2,387,248 bytes, `6ff517df…87d3b4`; party archive (SIAC) PDF |
| `br_pt_bn091_199410` | Boletim Nacional do PT, nº 91 (October 1994) | 01, 02 | 1,835,767 bytes, `3428336f…cbb4a8`; party archive (SIAC) PDF |
| `br_pt_bn_edicao_especial_199412` | Boletim Nacional, Edição Especial (Dezembro/94), a special publication of the PT's Secretaria Nacional de Comunicação | 02 | 7,589,722 bytes, `4f13a575…b38bec`; party archive (SIAC) PDF; **found by the checks** |
| `br_pt_site_rui_falcao_biography_20170617` | Rui Falcão (PT site biography page, www.pt.org.br/rui-falcao/, Internet Archive capture of 17 June 2017) | 02 | 53,135 bytes, `c2ea5610…908d50`; capture 2017-06-17; **found by the checks** |
| `br_pt_csbh_resumo_10_encontro_1995` | Resumo do 10º Encontro Nacional (18-20 August 1995), CSBH dossier '1995 - Dossiê X Encontro Nacional do PT' | 02 | 196,092 bytes, `e14102e9…859fa7`; party archive (SIAC) PDF |
| `br_pt_sncr_oficio_001_19950824` | Ofício nº 001/95 of the Secretaria Nacional de Combate ao Racismo (provisional collective) to José Dirceu, 24 August 1995 | 02 | 894,821 bytes, `ee797a76…17d085`; party archive (SIAC) PDF |
| `br_pt_csbh_resumo_11_encontro_1997` | Resumo do 11º Encontro Nacional (August 1997), CSBH dossier '1997 - Dossiê XI Encontro Nacional do PT' | 03 | 192,624 bytes, `5571ab78…8c7d07`; party archive (SIAC) PDF |
| `br_pt_noticias_051_19970910` | PT Notícias nº 51, ano II (10-16 September 1997) | 03 | 12,285,885 bytes, `e91bc278…790b37`; party archive (SIAC) PDF |
| `br_pt_noticias_052_19970920` | PT Notícias nº 52, ano II (20-26 September 1997) | 03 | 11,754,811 bytes, `b673c856…08111d`; party archive (SIAC) PDF; **found by the checks** |
| `br_pt_noticias_087_19991216` | PT Notícias nº 87, ano III (16 December 1999 - 5 January 2000) | 03 | 9,375,590 bytes, `4434183f…b68ad5`; party archive (SIAC) PDF |
| `br_pt_presidencia_comunicacao_19991202` | Comunicação do Presidente Nacional do PT aos integrantes da antiga Comissão Executiva Nacional, Secretários e Membros do DN (São Paulo, 2 December 1999) | 03 | 473,863 bytes, `fd1e18d7…e3008d`; party archive (SIAC) PDF |
| `br_pt_noticias_105_20010720` | PT Notícias nº 105, ano V (20 July - 3 August 2001) | 03 | 4,815,117 bytes, `9e68759d…c85f20`; party archive (SIAC) PDF; **found by the checks** |
| `br_pt_noticias_106_20010804` | PT Notícias nº 106 (4-18 August 2001) | 03 | 6,038,454 bytes, `3bf47b0f…a57ab0`; party archive (SIAC) PDF; **found by the checks** |
| `br_pt_noticias_107_20010819` | PT Notícias nº 107 (19 August - 4 September 2001) | 03 | 16,049,882 bytes, `8065054b…45b397`; party archive (SIAC) PDF; **found by the checks** |
| `br_pt_noticias_108_20010910` | PT Notícias nº 108, ano 5 (10-25 September 2001) | 03 | 9,061,147 bytes, `f57c7a0c…7aa1f3`; party archive (SIAC) PDF |
| `br_pt_noticias_109_20011010` | PT Notícias nº 109, ano 5 (10-25 October 2001) | 03 | 5,198,077 bytes, `9710168f…51a497`; party archive (SIAC) PDF |
| `br_pt_noticias_ee12_200201` | PT Notícias, ano VI, edição especial 12º Encontro Nacional (January 2002) | 03 | 8,801,111 bytes, `52c7550e…7e5e02`; party archive (SIAC) PDF |
| `br_pt_noticias_124_20021216` | PT Notícias nº 124, ano 6 (16-31 December 2002) | 01, 03 | 7,539,258 bytes, `4e42a824…5164ef`; party archive (SIAC) PDF |
| `br_pt_camara_bio_gushiken_20250807` | Biografia do(a) Deputado(a) Federal LUIZ GUSHIKEN - Portal da Câmara dos Deputados (Internet Archive capture of 7 August 2025) | 01 | 39,743 bytes, `1281ac6f…404734`; capture 2025-08-07 |
| `br_pt_camara_bio_dirceu_20251108` | Biografia do(a) Deputado(a) Federal JOSÉ DIRCEU - Portal da Câmara dos Deputados (Internet Archive capture of 8 November 2025) | 03 | 40,033 bytes, `af0740c7…96e40c`; capture 2025-11-08 |
| `br_pt_portal_genoino_congress_talks_20030108` | Genoíno discutirá eleição no Congresso com PMDB, PSDB e PFL (Portal do PT news item, 08/01/2003) | 04 | 21,287 bytes, `8fa5b50f…026afc`; capture 2005-01-25 |
| `br_pt_official_note_lauro_campos_20030113` | Leia nota divulgada por Genoino sobre Lauro Campos (Portal do PT, 13/01/2003; 'Nota oficial do PT') | 04 | 20,306 bytes, `9671c1a0…19ed5a`; capture 2005-01-25 |
| `br_pt_portal_genoino_governor_affiliation_20030318` | PT amplia seu quadro de governadores (Portal do PT news item, 18/03/2003) | 04 | 22,950 bytes, `07397ece…08e658`; capture 2005-01-04; **located for this packet** |
| `br_pt_portal_genoino_leaves_presidency_20050709` | Genoino deixa a presidência do partido (Portal do PT, 09/07/2005, with the text of his 'Declaração política') | 04 | 32,460 bytes, `57489335…f1d0d2`; capture 2005-09-28 |
| `br_pt_portal_reactions_genoino_departure_20050709` | Petistas ressaltam grandeza e dignidade de José Genoino (Portal do PT, 09/07/2005) | 04 | 26,028 bytes, `4d9c918a…ef037d`; capture 2005-09-28 |
| `br_pt_portal_tarso_genro_new_president_20050709` | Tarso Genro é novo presidente do PT (Portal do PT, 09/07/2005) | 05 | 26,893 bytes, `acda68ac…d0c3ed`; capture 2005-09-28 |
| `br_pt_portal_tarso_first_press_conference_20050709` | Tarso reafirma compromisso de prestar contas à sociedade (Portal do PT, 09/07/2005) | 05 | 28,683 bytes, `ba66c4cb…2177f0`; capture 2005-09-28 |
| `br_pt_portal_tarso_after_cen_meeting_20050710` | Tarso Genro: Vamos reconstruir nossa densidade ética (Portal do PT, 10/07/2005) | 05 | 31,370 bytes, `7e136a44…cfd168`; capture 2005-09-28 |
| `br_pt_portal_ped2005_voting_closed_20050918` | PT encerra votação em todo o Brasil (Portal do PT, 18/09/2005) | 05 | 22,879 bytes, `e0804b14…e407a6`; capture 2005-09-28; **found by the checks** |
| `br_pt_portal_ped2005_first_partial_bulletin_20050919` | Confira o 1º boletim parcial do PED nacional (Portal do PT, 19/09/2005) | 05 | 59,110 bytes, `e2b97f4d…94c668`; capture 2005-09-28 |
| `br_pt_portal_posse_date_anticipated_20050919` | Data da posse de novos dirigentes nacionais será antecipada (Portal do PT, 19/09/2005) | 05 | 22,790 bytes, `815b0e34…26a3e8`; capture 2005-09-28 |
| `br_pt_portal_ped2005_second_round_closed_20051009` | PT encerra votação em todo o Brasil (Portal do PT, 09/10/2005) | 05 | 23,334 bytes, `5c0061d8…92ab32`; capture 2005-10-13; **found by the checks** |
| `br_pt_portal_ped2005_second_round_first_partial_20051010` | PED: 1ª parcial aponta disputa acirrada (Portal do PT, 10/10/2005) | 05 | 36,655 bytes, `604527cf…e0265b`; capture 2005-10-12 |
| `br_pt_portal_berzoini_new_president_20051011` | Ricardo Berzoini é o novo presidente do PT (Portal do PT, 11/10/2005) | 05 | 36,620 bytes, `8a4b934b…d178d2`; capture 2005-10-12 |
| `br_pt_portal_ped2005_berzoini_elected_20051013` | Eleições internas reforçam vitalidade e democracia do Partido dos Trabalhadores (Portal do PT, 13/10/2005) | 05 | 38,217 bytes, `702c93c0…72a071`; capture 2005-10-20 |
| `br_pt_portal_tarso_announces_final_results_20051013` | Tarso anuncia hoje resultados finais do PED em coletiva às 17h (Portal do PT, 13/10/2005) | 05 | 23,435 bytes, `ad125fce…998b27`; capture 2005-10-20; **found by the checks** |
| `br_pt_portal_tarso_meets_pcdob_psb_20051014` | Tarso de reúne hoje com PCdoB e PSB (Portal do PT, 14/10/2005) | 05 | 21,821 bytes, `6898d30c…052371`; capture 2005-10-20; **found by the checks** |
| `br_pt_portal_dn_posse_new_executive_20051022` | Confira a nova Executiva Nacional do PT (Portal do PT, 22/10/2005) | 05 | 33,668 bytes, `e0b5abcc…ece5f4`; capture 2005-11-04 |
| `br_pt_resolution_campaign_coordination_20060428` | Resolução da CEN sobre Coordenação Política da Campanha Presidencial (Portal do PT documents, 28/04/2006) | 06 | 22,565 bytes, `91223796…2782e0`; capture 2010-10-07 |
| `br_pt_site_cen_members_20061127` | Comissão Executiva Nacional (CEN) - Integrantes da CEN (PT site page site/conheca/executiva_nac.asp, capture of 27 November 2006) | 06 | 40,667 bytes, `2e43f266…dca2de`; capture 2006-11-27 |
| `br_pt_dn_guidelines_3rd_congress_20061226` | Conheça as diretrizes gerais para o regulamento aprovado para o 3º Congresso (PT documents, 26/12/2006) | 06 | 26,168 bytes, `c42aaf59…3bf657`; capture 2010-10-07; **found by the checks** |
| `br_pt_site_cen_members_20070103` | Comissão Executiva Nacional (CEN) - Integrantes da CEN (PT site page, capture of 3 January 2007) | 06 | 40,727 bytes, `adbaa2b7…e54f01`; capture 2007-01-03 |
| `br_pt_portal_berzoini_weekly_interview_20070305` | Entrevista da 2ª: Berzoini discute reforma política, composição de governo e visita de Bush (Portal do PT, 05/03/2007) | 06 | 63,862 bytes, `50d90211…3f3b49`; capture 2007-03-14 |
| `br_pt_site_cen_members_20070313` | Comissão Executiva Nacional (CEN) - Integrantes da CEN (PT site page, capture of 13 March 2007) | 06 | 40,131 bytes, `e2c35c49…5c2298`; capture 2007-03-13 |
| `br_pt_portal_berzoini_votes_ped2007_20071216` | Berzoini: Lula demonstra compromisso com o PT ao votar no PED (Portal do PT, Secretaria Geral news; body dated 16/12/2007) | 06 | 22,541 bytes, `14e38550…206d57`; capture 2010-10-07 |
| `br_pt_cen_resolution_ped2007_result_20071219` | Resolução da CEN sobre resultado do PED e fim da CPMF (Portal do PT documents, 19/12/2007) | 06 | 23,056 bytes, `3ebe5a41…727b49`; capture 2010-10-07 |
| `br_pt_portal_cen_members_2008_2009_20091113` | Integrantes da Executiva Nacional - Integrantes da CEN para o biênio 2008/2009 (Portal do PT page, capture of 13 November 2009) | 06 | 19,440 bytes, `4935adf2…4f2244`; capture 2009-11-13 |
| `br_pt_portal_berzoini_press_conference_ped2009_20091124` | Berzoini concede coletiva nesta quarta-feira para falar sobre resultados do PED (Portal do PT, 24/11/2009) | 06 | 21,947 bytes, `fbc58c10…deb270`; capture 2009-11-28 |
| `br_pt_portal_dutra_elected_20091125` | Dutra é eleito presidente do PT; quase meio milhão de petistas participaram do PED (Portal do PT, 25/11/2009, updated 28/11) | 06 | 21,925 bytes, `59300480…144f27`; capture 2009-11-29 |
| `br_pt_portal_ped2009_final_result_20091202` | Lição de democracia: eleições internas do PT levaram 518.912 filiados às urnas (Portal do PT, 02/12/2009) | 06 | 25,425 bytes, `964ba1e8…431945`; capture 2009-12-05 |
| `br_pt_portal_ped2009_posse_rule_20100111` | Prazo para posse: CEN aprova alteração no artigo 47 do Regulamento do PED 2009 (Portal do PT, Secretaria de Organização, 11/01/2010) | 06 | 20,479 bytes, `c9cd883d…9afa73`; capture 2010-10-07; **found by the checks** |
| `br_pt_camara_dcd_berzoini_farewell_20100211` | Diário da Câmara dos Deputados, ano LXV, nº 013 (11 February 2010): session of 10 February 2010, Ricardo Berzoini's farewell to the PT national presidency | 05, 06 | 6,444,492 bytes, `48098bf9…1dd9a8`; Chamber stored diary; **found by the checks** |
| `br_pt_portal_berzoini_farewell_reported_20100211` | Berzoini: PT é patrimônio da militância e referência de compromisso com o povo (Portal do PT, 11/02/2010) | 06 | 33,841 bytes, `e33ad62e…c2cd61`; capture 2010-10-07; **found by the checks** |
| `br_pt_portal_iv_congress_posse_programme_20100219` | IV Congresso debate tática, diretrizes do programa de 2010 e empossa nova direção (Portal do PT, 19/02/2010 08:34) | 06 | 22,090 bytes, `03e5d8cd…ebf6fe`; capture 2010-02-23 |
| `br_pt_portal_dn_posse_dutra_20100219` | O PT tem lado e sabe o que o quer para o Brasil, diz Lula na posse do novo DN (Portal do PT, 19/02/2010 22:15) | 06 | 25,662 bytes, `9eaa13b5…2e5f4e`; capture 2010-02-23 |
| `br_pt_portal_dn_defines_new_cen_20100220` | DN reúne e define a nova Comissão Executiva Nacional do PT (Portal do PT, 20/02/2010) | 06 | 22,646 bytes, `41d50b70…d66b6d`; capture 2010-02-23 |
| `br_pt_portal_dutra_dn_meeting_20110426` | José Eduardo Dutra confirma presença em reunião do Diretório Nacional do PT (Portal do PT, Geral) | 07 | 23,735 bytes, `1aea07b6…3d5c38`; capture 2011-04-30 |
| `br_pt_portal_novo_comando_20110508` | Novo comando do PT aponta para unidade da militância de olho nas eleições municipais (Portal do PT, Institucional, 08/05/2011) | 07 | 25,987 bytes, `a5599021…46ebd8`; capture 2011-05-11; **found by the checks** |
| `br_pt_portal_falcao_president_20110615` | Rui Falcão destaca Eleições 2012, Brasil Sem Miséria e ativismo na internet (Portal do PT, Institucional) | 07 | 25,552 bytes, `1c440eef…595413`; capture 2011-06-20 |
| `br_pt_ped2013_falcao_reelected_20131112` | PED 2013: Rui Falcão é reeleito presidente nacional do PT (Portal do PT) | 07 | 26,772 bytes, `8ec431d6…73b9e1`; capture 2013-11-15 |
| `br_pt_dn_resolution_ped2013_posse_20131118` | Resolução sobre posse de dirigentes eleitos no PED-2013 (Diretório Nacional do PT) | 07 | 25,649 bytes, `2e7aea03…6c3e30`; capture 2013-11-22 |
| `br_pt_portal_guia_5th_congress_20131210` | 5º Congresso Nacional do PT: Guia do Congressista (Portal do PT, 10/12/13) | 07 | 26,097 bytes, `a902f4c5…f57e14`; capture 2013-12-11; **found by the checks** |
| `br_pt_portal_dn_elects_cen_20131211` | PT: Diretório Nacional elege e empossa nova Comissão Executiva Nacional (Portal do PT, 11/12/13) | 07 | 26,619 bytes, `8c495bc5…5eacd3`; capture 2013-12-19; **found by the checks** |
| `br_pt_falcao_posse_speech_20131212` | 5º Congresso: Discurso de posse do presidente nacional do PT, Rui Falcão (Portal do PT) | 07 | 44,714 bytes, `d068ba43…4be0d7`; capture 2013-12-19 |
| `br_pt_portal_dn_composition_20131218` | Composição atual do Diretório Nacional do Partido dos Trabalhadores (Portal do PT, 18/12/13) | 07 | 28,165 bytes, `f7c0e3cc…6d9d77`; capture 2013-12-23; **found by the checks** |
| `br_pt_tse_sgip_dn_2014_2017` | SGIP: PT national organ 70953 (Órgão definitivo, vigência 19/02/2014-08/09/2017) with members | 07 | 41,754 bytes, `4b7a8986…de601b`; TSE SGIP JSON, closed organ |
| `br_pt_tse_sgip_cen_2014_2017` | SGIP: PT national organ 70954 (Comissão executiva, vigência 19/02/2014-09/10/2017) with members | 07 | 18,797 bytes, `723f11fd…f982ca`; TSE SGIP JSON, closed organ |
| `br_pt_agencia_5th_congress_falcao_20150612` | 5º Congresso do PT foi totalmente bem-sucedido, avalia Rui Falcão (Agência PT de Notícias) | 07 | 55,662 bytes, `dced8c74…674c49`; capture 2015-06-15 |
| `br_pt_dn_page_falcao_20160805` | Diretório Nacional | Partido dos Trabalhadores (composition page) | 07 | 96,335 bytes, `3efa4283…e92e6b`; capture 2016-08-05 |
| `br_pt_falcao_balance_20170527` | Conheça o balanço da gestão de Rui Falcão a frente do PT (Agência PT de Notícias) | 07 | 65,582 bytes, `bb3b8bf7…6d6381`; capture 2018-10-11 |
| `br_pt_gleisi_elected_6th_congress_20170603` | Gleisi é eleita a primeira mulher presidenta nacional do PT (Agência PT de Notícias) | 08 | 67,962 bytes, `d50070c0…245448`; capture 2017-06-05 |
| `br_pt_new_directorate_gleisi_posse_20170705` | Novo Diretório e Gleisi Hoffmann tomam posse no PT (Agência PT de Notícias) | 08 | 62,459 bytes, `cda5ae16…06ac5b`; capture 2017-07-07 |
| `br_pt_lula_at_gleisi_posse_20170705` | Lula garante apoio incondicional à Gleisi na presidência do PT (Agência PT de Notícias; URL slug 'valeu-a-pena-insistir-nas-mulheres-diz-lula-em-posse-de-gleisi') | 08 | 68,205 bytes, `e72dc623…73a883`; capture 2017-07-07 |
| `br_pt_tse_sgip_dn_2017_2020` | SGIP: PT national organ 232052 (Órgão definitivo, vigência 09/09/2017-17/01/2020) with members | 08 | 43,974 bytes, `c132f132…7317a3`; TSE SGIP JSON, closed organ |
| `br_pt_dn_page_gleisi_20180225` | Diretório Nacional | Partido dos Trabalhadores (composition page) | 08 | 182,595 bytes, `95dd6e10…2726ac`; capture 2018-02-25 |
| `br_pt_gleisi_reelected_7th_congress_20191124` | Assista: Gleisi Hoffmann é reeleita presidenta Nacional do PT (pt.org.br) | 08 | 62,315 bytes, `61dd6ddb…dbb4c1`; capture 2019-12-21 |
| `br_pt_gleisi_reelected_text_20191124` | Gleisi Hoffmann é reeleita presidenta do Partido dos Trabalhadores (pt.org.br, 24/11/2019) | 08 | 71,267 bytes, `f5d73d2d…277a17`; capture 2019-12-20; **found by the checks** |
| `br_pt_7th_congress_new_leadership_20191126` | 7º Congresso Nacional elege nova direção do PT (pt.org.br, 26/11/2019) | 08 | 66,164 bytes, `c958459b…44b371`; capture 2019-12-21; **found by the checks** |
| `br_pt_tse_sgip_dn_2020_2025` | SGIP: PT national organ 308884 (Órgão definitivo, vigência 17/01/2020-23/08/2025) with members | 08, 09 | 65,506 bytes, `d53c2fdf…8bad62`; TSE SGIP JSON, closed organ |
| `br_pt_dn_page_gleisi_20200401` | Diretório Nacional | Partido dos Trabalhadores (composition page) | 08 | 174,032 bytes, `40e6c715…d9f8bd`; capture 2020-04-01 |
| `br_pt_portal_gleisi_sri_congratulations_20250228` | PT parabeniza Gleisi por nomeação para Secretaria de Relações Institucionais (pt.org.br, 28/02/2025) | 08 | 98,654 bytes, `bb6c575d…4cd576`; capture 2025-03-01; **found by the checks** |
| `br_pt_humberto_interim_executive_20250307` | Com Gleisi ministra, Humberto Costa assume interinamente presidência do PT (pt.org.br) | 08, 09 | 98,815 bytes, `bfdf4e5a…7e19ea`; capture 2025-03-08 |
| `br_pt_dn_page_humberto_20250308` | Diretório Nacional | Atribuições | Partido dos Trabalhadores (composition page) | 08, 09 | 46,464 bytes, `a223ce2b…c1e77c`; capture 2025-03-08, gzip-stored |
| `br_pt_humberto_priorities_20250312` | Presidente nacional do PT, Humberto Costa lista prioridades do novo desafio (pt.org.br, from PT no Senado) | 09 | 97,522 bytes, `6c10728b…fc5834`; capture 2025-03-13 |
| `br_pt_portal_humberto_ped_mobilization_20250313` | Humberto Costa: PED deve ser processo de mobilização nacional para 2026 (pt.org.br, 13/03/2025) | 09 | 95,733 bytes, `86ee3963…dab56a`; capture 2025-03-14; **found by the checks** |
| `br_pt_dn_elects_humberto_20250320` | Em reunião do Diretório, PT elege Humberto Costa como presidente (pt.org.br) | 09 | 96,231 bytes, `2c38caf5…85f7d5`; capture 2025-03-21 |
| `br_pt_portal_humberto_reiterates_unity_20250320` | PED 2025: Humberto Costa reitera trabalho pela união nacional do PT (pt.org.br, 20/03/2025) | 09 | 99,562 bytes, `4e732da3…57784f`; capture 2025-03-21; **found by the checks** |
| `br_pt_edinho_elected_ped_20250707` | Edinho Silva é eleito novo presidente nacional do PT (Rede PT de Comunicação) | 09 | 108,049 bytes, `f4df9d15…553a7d`; capture 2026-06-08 |
| `br_pt_ped2025_totalization_20250715` | PED 2025 - Totalização dos votos (PDF, 4 pages) | 09 | 263,758 bytes, `220aaa11…bb51fe`; capture 2025-07-17 |
| `br_pt_ped2025_press_conference_20250716` | Em coletiva, PT divulga balanço do Processo de Eleição Direta 2025 (Rede PT de Comunicação) | 09 | 125,726 bytes, `ca88b20c…b97e73`; capture 2026-08-15 |
| `br_pt_encontro_posse_notice_20250723` | PT faz encontro nacional e empossa nova direção no início de agosto (pt.org.br) | 09 | 91,839 bytes, `abcdce0e…a6bebe`; capture 2025-10-14 |
| `br_pt_portal_edinho_speech_report_20250803` | “Quero ser presidente de um partido vivo e comprometido com o povo”, diz Edinho Silva (pt.org.br, 03/08/2025 13h27) | 09 | 101,233 bytes, `4b68321f…ec2425`; capture 2025-08-04; **found by the checks** |
| `br_pt_edinho_posse_speech_20250803` | Leia a íntegra do discurso de Edinho Silva, novo presidente do PT (pt.org.br) | 09 | 116,902 bytes, `38bde2b4…abf228`; capture 2025-10-15 |
| `br_pt_dn_page_edinho_20250917` | Diretório Nacional | Atribuições | Partido dos Trabalhadores (composition page) | 09 | 40,639 bytes, `c179b431…cbffdb`; capture 2025-09-17, gzip-stored |
| `br_pt_dn_page_edinho_20260815` | Diretório Nacional – Partido dos Trabalhadores (composition page) | 10 | 159,613 bytes, `37fedb84…c0657b`; capture 2026-08-15 |

The sources are the party's own records and two public record-keepers of the party office: the party compilation of
directorate compositions (7 born-digital PDFs of the Fundação Perseu Abramo), the party archive's (SIAC, Centro Sérgio
Buarque de Holanda) scans of the Boletim Nacional (organ of the national executive committee), PT Notícias (organ of the
Diretório Nacional), four national-meeting summary sheets and two dated letters; raw Internet Archive captures of the
party's own site (www.pt.org.br, 2003-2026); the Chamber of Deputies' biographies of Gushiken and Dirceu and its stored
diary of 11 February 2010 (Congress records of the party office); and four closed national organs in the Superior
Electoral Court's SGIP registry. News (Brasil Agora, press), encyclopaedias and a state legislature's news item are
leads only.

Each source has a derived factual extract under [sources/](sources/) in the house format
(`spheres-c01-derived-factual-table/v1`): one row per claim, keyed by `claim_id`, with `observation_id`
`br_tse_fefc_2024_party_03`, `review_observation`, `role_id` `br_pt_president`, a normalised `holder_name`, `role_title`
(the office, the acting or interim office, or the vice-presidency), `event_kind`, `attested_on`, the claim's text and
locator. Each extract records the original response's byte count and SHA-256, its content encoding, a fetch recipe, the
stability check and a provenance note saying that the original is not checked into this repository. Original PDFs,
pages, registry records and renders are not checked in, and no photograph, logo, signature or personal registry field is
reproduced: the SGIP extracts carry only the organ's dates and the President's name, office, exercise dates and status
(the test asserts that no CPF, voter-registration, contact or password field is present).

New source types: `primary_party_directorate_compilation_pdf`, `primary_party_newspaper_scan_pdf`,
`primary_party_archive_summary_pdf`, `primary_party_letter_scan_pdf`, `primary_party_web_page_archived`,
`primary_party_results_pdf_archived`, `primary_congress_biography_archived` and
`primary_electoral_court_party_registry_json`; the Chamber diary reuses CLAUDE-C01-10's
`primary_congress_session_record_pdf`.

## Response identities and stability checks

Every recorded response was downloaded at least twice, at least 30 minutes apart, with the same byte count and SHA-256;
the time of each download is in the extract's `stability_check`.

| Responses | Downloads (UTC, 25 September 2026) | Stability basis |
|---|---|---|
| FPA compilation PDFs (7) | researcher 12:02 (Cloudflare HIT) and 12:39 (cache-busted, MISS); check 13:10-13:11 (cache-busted) | origin fetches 37 and 68 minutes apart; the alternative URLs of the DN4, DN5, DN7 and DN8 files serve identical bytes |
| SIAC PDFs of part A (18) | researcher 12:09-12:28 and 12:42-13:00 (cache-busted); check 13:10-13:11 (cache-busted) | plain nginx origin, no CDN; 30+ minutes |
| SIAC PDFs found by check A (5) | check 13:17-13:27 and 13:54 (cache-busted); this packet 16:01 for the special edition and nº 52 | 30+ minutes; this packet's downloads 2.5 hours after the first |
| Internet Archive captures of part A (3) | researcher 12:33-12:35 and 13:06; check 13:10-13:11 and 13:54 | raw id_ captures without Accept-Encoding; the recorded identity is the uncompressed body |
| Internet Archive captures of part B (27) | researcher 12:08-13:25 and 13:32-13:56 (curl, identity); check 13:58-14:00 and 14:31-14:33 | raw id_ captures, no Content-Encoding; 30+ minutes after both earlier downloads |
| Candidates found by check B (7) and the Chamber diary | check 14:06-14:13 and 14:43; the diary also by this packet at 16:07 (cache-busted) | 30+ minutes; the diary is a stored file with a fixed Last-Modified (12 April 2010) |
| Portal item of 18 March 2003 (located for this packet) | 16:00:37 and 16:31:02 | raw id_ capture, 30 minutes apart |
| Internet Archive captures of part C (25) | researcher 13:27-14:20 and 14:12-14:52; check 14:54-14:57 or 15:23-15:25 | raw id_ captures; two are stored gzip-encoded (see below) |
| Candidates found by check C (10) | check 15:03-15:19 and 15:49 | 30+ minutes |
| SGIP records of closed organs (4) | researcher 12:59 and 13:31; check 14:54-14:55 | JSON generated per request but without any timestamp, token or session value; byte-identical over two hours |

Reproducibility notes for the reviewer. The Internet Archive serves most raw captures uncompressed to a client that
sends no Accept-Encoding header; a gzip-accepting client can receive a gzip body instead (the Dirceu biography capture
returns 10,369 compressed bytes), so fetch with curl without `--compressed`. Two captures of the PT's Diretório page
(8 March 2025 and 17 September 2025) are stored gzip-encoded and are served with `Content-Encoding: gzip` even to an
identity request: their recorded identity is the gzip body as served (46,464 and 40,639 bytes), and the decoded HTML
identities (209,451 and 190,467 bytes) are recorded beside it; do not let the client decode them. A cache-busting query
may be added to the FPA and SIAC URLs to reach the origin; it is not part of any recorded URL. The SGIP records are
closed organs (Não Vigente) and are not expected to change, but a later annotation by the court would change their
bytes; they carry members' personal data, which must not be copied. No search or listing page, live party page, viewer
page-range PDF (the Chamber's `dc_20b.asp` or `montaPdf.asp`) or cache-busting query is a recorded identity.

## Leads not imported

- The TSE's SGIP record of the national organ in force,
  https://sgip3.tse.jus.br/sgip3-consulta/api/v1/orgaoPartidario/comAnotacoesEMembros?idOrgaoPartidario=570664&isMembrosAtivos=false
  (42,872 bytes, `6019d226…8f4171`, byte-identical at 12:59, 13:31 and 14:54 UTC): it lists EDSON ANTONIO EDINHO DA
  SILVA as PRESIDENTE, Ativo, with a registry period from 23 August 2025 and a last alteration of 8 July 2026, but the
  organ is in force (Vigente) and its bytes will change on its next annotation, so it is not a reproducible identity
  (check C14). The SGIP index of PT national organs (2,381 bytes, `7c933018…`) was used only to find the organ ids.
- The Chamber's diary of 19 December 2002, https://imagem.camara.leg.br/Imagem/d/pdf/DCD19DEZ2002.pdf (51,232,685 bytes,
  `973be2e1…a6ca6d`, identical at 14:09 and 14:43 UTC): other deputies address Genoino as President of the PT during his
  farewell of 18 December 2002. Declined (check B10): third parties' remarks on an OCR-only scan, during the period the
  party compilation records as interim; they would not change any holder.
- Brasil Agora nº 73 (https://siac.fpabramo.org.br/uploads/acervo/J_BA_1995_0073.pdf; 8,640,762 bytes, `b0c55694…2aee1`)
  and nº 59 (.../J_BA_1994_0059.pdf): a newspaper published by a company (Editora Brasil Agora Ltda.), so news; nº 73
  calls Dirceu the new president and Lula both 'ex-presidente nacional' and 'presidente nacional' in August 1995, a lead
  for the December 1994-August 1995 interval.
- Boletim Nacional special edition of the VII Encontro (theses; .../J_BN_1990_ee_7_ENCONTRO_NACIONAL.pdf; 28,113,991
  bytes, `19b1614c…f728d3`): Gushiken's undated presentation; nº 48 (February 1990; .../J_BN_1990_0048.pdf; 7,248,322
  bytes, `c683e4c0…77e8e`): names no president; nº 41 (December 1988/January 1989; .../J_BN_1988-1989_0041.pdf), the
  source of Gushiken's 1988 selection, pre-period and not downloaded.
- Boletim Nacional nº 86 and nº 88-90 (April-September 1994; .../J_BN_1994_0086.pdf etc.): their text layers were
  searched without finding Lula's leave or Rui Falcão's assumption. (The researcher's note also listed the December 1994
  special edition here; it has no text layer and was not searched, and the check found and imported its page 4.)
- The CSBH item '5º Diretório Nacional Eleito em 15 de Julho de 1990' (SIAC catalogue item 6031): a 1991 list whose title
  conflicts with the compilation; not downloaded.
- PT Notícias nº 109, the 'União das esquerdas' column on the 12th Encontro's opening session: undated (check A7).
- PT Notícias nº 130 (5-25 April 2003; .../J_PT_NOTICIAS_2003_0130.pdf; 9,058,336 bytes, `bb06413a…a49e30ab`), read for
  this packet: it styles Genoino national president, but by issue span and in a meeting dated only by a span of days;
  the portal item of 18 March 2003 is used instead.
- Portal do PT items of 9 July 2005: 'Petistas discutem propostas' (cod=36604; 37,489 bytes, `479c6b4e…ef52e45`), 'Novo
  presidente do PT quer repactuação' (cod=36615; 23,689 bytes, `f9c84d40…2e45`) and the secretaries' items
  (cod=36611-36614): context or duplicates of imported stylings.
- Portal do PT 13 October 2005 (cod=39223; 31,715 bytes, `9ec2597e…fdfb`) and 11 October 2005 (cod=39181): duplicates of
  the imported declaration and stylings.
- The PED site's 2005 results table (http://ped.pt.org.br/.../2005_Presidente.htm, capture 20091016174424; 12,065 bytes,
  `5ea023c2…c44c`): an undated first-round table, retrospective. Its 2007 page reads 'Em construção'.
- Portal do PT 30 November 2009 (97% counted; 21,000 bytes, `2c3814f2…714b`): duplicates the 25 November and 2 December
  items.
- The Diretório's resolution of 9 February 2008 (documentos/...-210.html; 41,802 bytes, `c2faa65d…77c8`) and an undated
  item on the first meeting of the new Diretório after the PED 2007 (9-10 February 2008): they name no president.
- The executive's note of 31 July 2007 on Marco Aurélio Garcia as '1º vice-presidente', its resolution of 6 October 2006
  on the 'dossiê' case, and the Portal do PT item of 30 September 2006 on Garcia as campaign coordinator: other offices
  or no president named.
- The PT site's stale late-1990s executive list captured in January 2003 (presidente.pt.org.br/execut.htm): not
  evidence of the office in 2003.
- The PED 2005 list of candidates (site/ped2005/presidentes/presidentes_nac.asp) and the PT site's resolution page of
  the 19 September 2005 rule (resolucoes_int.asp?cod=15): a candidacy list and a duplicate of the imported rule.
- The PT's own article on Rui Falcão's 2011 election
  (portalpt/noticias/institucional-3/rui-falcao-e-eleito-por-unanimidade-para-presidir-partido-dos-trabalhadores-58511.html):
  its only capture (2021) returns 404.
- The Portal do PT item of 5 May 2011 on the Diretório's political-reform resolution
  (…diretorio-nacional-do-pt-aprova-resolucao-sobre-reforma-politica-58721.html; 27,666 bytes, `4e946894…b53a`): it
  dates the meeting to Saturday 30 April but does not mention the President's election.
- The executive committee page of August 2011 (loaded by script; no member list in the archived body).
- The PT page 'gleisi-hoffmann-assumira-secretaria-de-relacoes-institucionais' (28 February 2025; capture 20250318125334;
  22,286 bytes, gzip-stored, `a67c8468…52ef`): a companion of the imported 28 February item.
- A search-engine title for 'em-reuniao-do-diretorio-pt-elege-humberto-costa-como-presidente-interino': no capture
  before the cutoff; the archived '-2' address is used.
- The party foundation's republications (fpabramo.org.br/focusbrasil/2025/08/04/... and fpabramo.org.br/2019/11/25/...):
  duplicates of the PT's own pages.
- News: poder360.com.br and otempo.com.br on the 3 August 2025 posse, em.com.br on the 2013 re-election, and the São
  Paulo Legislative Assembly's news item (al.sp.gov.br/noticia/?id=305399) on Falcão's 2011 election.
- Encyclopaedias: en.wikipedia.org's articles on Rui Falcão (terms '2011-2013, 2013-2015 and 2015-2017', the likely
  origin of the unsupported 2015 hint) and Gleisi Hoffmann.
- The Chamber's live biographies of Rui Falcão (73604), Genoino (73540) and Berzoini (74793): they list no party
  presidency.
- The PT statute and the PED regulations as such: procedure only.

## Sources attempted

- pt.org.br (live site, search, API, sitemaps, article pages and the 2025 results PDF): a GoCache security-check page
  (HTTP 403, 'Verificação de segurança'); not bypassed. Pre-cutoff Internet Archive captures are used instead.
- www.tse.jus.br and dadosabertos.tse.jus.br: Akamai 'Access Denied' (403); not bypassed. The SGIP consultation host
  answered normally.
- The CSBH AtoM browse listing (acervo.fpabramo.org.br): HTTP 503; its item pages are regenerated per request with a
  print date and were used only for navigation.
- The TSE SGIP history of PT national organs begins on 19 February 2014: nothing is recorded there for 1990-2013.
- The Internet Archive: domain-wide CDX listings timed out or returned 504; replays refused connections or went offline
  for stretches in all three parts; requests were retried with pauses. The Portal do PT's news items of December 2002
  and of September 2006, and the sitept items of December 2007, were never archived; the 21 November 2025 capture of the
  Dirceu biography is stored gzip-compressed (8,963 bytes) and was not used.
- The Chamber's open-data speeches API (dadosabertos.camara.leg.br/api/v2/deputados/.../discursos): a live API, not a
  recorded identity.

## Checker defects

| # | Defect | Outcome |
|---|---|---|
| A1 | PT Notícias nº 109: only page 1 read; the page 2 masthead ('José Genoino (em exercício)') conflicts with the 1 October resumption; three dated notes signed by Dirceu left out | **Applied**: pages 1, 2, 3 and 7 recorded; the masthead is an acting-service claim with no structured date; the notes of 4, 6 and 10 October 2001 and the page 3 'reassumiu' standfirst are claims; the 2001 holder moves to 4 October 2001 (the first fully dated act) and records the conflict |
| A2 | The December 1994 special edition, said to be searched, has no text layer; its page 4 dates Rui Falcão as national president on 25 November 1994 | **Applied**: imported (`br_pt_bn_edicao_especial_199412`; pages 2 and 4 rendered for this packet); Rui Falcão's holder is observed on 25 November 1994, with his interim status left open; the lead text is corrected; the unresolved interval is now December 1994 to August 1995 |
| A3 | Genoino's 2001 acting presidency: the claim said no source gives when or why it began; nº 105 does, and nº 108's masthead was missing | **Applied**: nº 105 imported (Dirceu's leave 'no dia 17' of July 2001, announced at the 13 July launch; the masthead); nº 108's masthead is a claim; nº 106 (note of 3 August 2001) and nº 107 (note of 16 August 2001; Dirceu 'licenciado') imported; the uncertainty is rewritten |
| A4 | The 1997 holder was dated by the 31 August vote day | **Applied**: nº 51's masthead is a claim, nº 52 is imported, and the holder is observed on 17 September 1997; the vote stays an election claim |
| A5 | The 1993 claim merged the directorate's and the executive committee's elections | **Applied**: split into `br_pt_fpa_dn6_elected_8th_encontro_presidente_lula_19930611_13` (no structured date) and `br_pt_fpa_cen_elected_presidente_lula_19930613` |
| A6 | The resignation claim also carried Genoino's election; its id suggested a single day | **Applied**: split; `br_pt_fpa_dn_genoino_elected_national_president_20030315` (PT-PRES-04) and `br_pt_fpa_dn_dirceu_resigns_national_presidency_20030315_16` (no structured date) |
| A7 | The 13 December 2001 claim merged a dated box with an undated column | **Applied**: limited to the box; the column is a lead |
| A8 | Dirceu's leave of December 2002 was dated by the later issue span | **Applied**: no structured date; the uncertainty says the leave is reported with the 7 December 2002 meeting |
| A9 | Rui Falcão styled vice-president in nº 85 (April 1994) not recorded | **Applied**: `br_pt_bn085_rui_falcao_vice_president_199404`, titled as the vice-presidency, never a holder |
| A10 | Internet Archive notes should say that a gzip-accepting client receives a compressed body (10,369 bytes for the Dirceu biography) | **Applied**: every capture's provenance note says so, and the Dirceu biography's gives the 10,369-byte figure |
| A11 | Claim ids ended in a single day where the source prints only a span | **Applied**: the eight ids are renamed to their spans (for example `…_19900601_03`, `…_19951028_29`, `…_20030315_16`) |
| A12 | Two claims sat in two observations, but a row carries one | **Applied**: one review observation per row (the December 2002 count PT-PRES-01, the Dirceu biography PT-PRES-03); the other observations cite them in this report and in the claims' uncertainty |
| B1 | Tarso Genro's `from` 2005-07-09 had no source: 'os novos empossados' names nobody | **Applied**: no start; the claim is reworded and re-kinded `prospective_tenure_statement`; the holder is observed on 10 July 2005 (the selection day kept out of holder dates, as in the ANC packet) |
| B2 | The Diretório's selection claim was cited as Tarso Genro's holder support | **Applied**: the holder cites only the in-office attestation of 10 July 2005 |
| B3 | Genoino's departure announcement was cited as holder support | **Applied**: not cited; a claim only |
| B4 | Genoino's holder name did not match a supporting claim ('Genoíno') | **Applied**: holder names are normalised ('José Genoino'; the printed 'Genoíno' stays in the text); the holder is dated by the portal item of 18 March 2003 printed 'José Genoino', because the party compilation records his service from 7 December 2002 to 15 March 2003 as interim, so the January 2003 stylings are claims |
| B5 | Locators of the 9 July 2005 declaration were one paragraph off | **Applied**: paragraphs 1 and 5 for the handover, paragraph 1 for the leave, paragraph 6 for '30 meses' |
| B6 | `leave_statement` prejudged resignation or leave | **Applied**: split into `office_handover_statement` and `leave_statement`, both claims |
| B7 | Two retrospective claims carried a structured date | **Applied**: `br_pt_genoino_recalls_30_months_in_presidency` and `br_pt_dutra_posse_speech_lists_past_pt_presidents` carry none, and the test forbids a date on every retrospective kind |
| B8 | A Chamber record states Berzoini's last day (10 February 2010) | **Applied**: the diary of 11 February 2010 and the portal item of 11 February 2010 are imported; the statement is a claim, never an until, and the conflict with the 19 February report is recorded for Codex |
| B9 | Retrospective statements bearing on Tarso Genro's status and the second Berzoini term | **Applied**: imported as retrospective claims with no structured date; the second Berzoini holder (13 November 2009) is kept as an observation |
| B10 | The Chamber diary of 19 December 2002 styles Genoino President of the PT | **Declined**: other deputies' remarks on an OCR-only scan, in the period the party compilation records as interim; they could not date a holder. Recorded as a lead with its identity |
| B11 | The PED 2005 ballot days were in no claim; 'Ontem' misread | **Applied**: cod=38515 and cod=39123 imported as ballot claims; the uncertainty of the 10 October partial is corrected |
| B12 | The 8 January 2003 paraphrase changed the order of events | **Applied** |
| B13 | `ped_result_resolution` for a resolution naming no result | **Applied**: re-kinded `ped_participation_resolution` |
| B14 | The 24 November 2009 styling was cited by a holder of another day | **Applied**: a continuation claim; holders cite only same-day attestations |
| C1 | Humberto Costa should be a holder from 20 March 2025 ('agora saio da condição de presidente interino para presidente efetivo') | **Applied**: the 20 March page is imported; his holder starts on 20 March 2025 on that stated change (Codex may prefer attested_on); the Diretório vote is a directorate election; the July stylings are continuation claims; the SGIP period is a registry period; 7-19 March stays interim |
| C2 | Edinho Silva's start is contested: the same day's page says he 'assume oficialmente o cargo na próxima segunda-feira (4)' | **Applied in part**: the 13h27 page is imported with two claims; the start stays on the stated posse of 3 August 2025 (option a), the 4 August announcement is a prospective claim pinned as never a boundary, and option b (attested_on 2025-09-17) is recorded for Codex |
| C3 | Rui Falcão's 2013 holder rested on a posse-speech claim; three December 2013 pages missed | **Applied**: the pages of 10, 11 and 18 December 2013 are imported; the holder is observed on 18 December 2013; the speech is a claim |
| C4 | The 2011 handover is recorded: Dutra's resignation 'no dia 29 de abril', Falcão 'o novo presidente nacional' | **Applied**: the 8 May 2011 page is imported; Dutra's until is 29 April 2011; Falcão's holder is observed on 8 May 2011; the 58511 article is recorded as attempted |
| C5 | Withdraw the until option of 7 March 2025 for Gleisi Hoffmann | **Applied**: no until; the resignation claim has no structured date; the 13 March page is imported |
| C6 | The 7 March 2025 paraphrase reversed who thanked whom | **Applied** |
| C7 | Wrong locator of the 2011 acting styling | **Applied**: second paragraph, first sentence |
| C8 | Wrong locator of the speech paragraphs | **Applied**: third and fourth paragraphs |
| C9 | The totalization claim did not print the names as the PDF does | **Applied**: '180 EDINHO', '150 ROMENIO PEREIRA', '130 RUI FALCAO', '120 VALTER POMAR' |
| C10 | The Diretório page's recital of the 6 July election carried a structured date | **Applied**: retrospective, no structured date |
| C11 | The 2017 posse note wrongly said the post does not name her | **Applied** |
| C12 | `holder_name` gave four people nine spellings | **Applied**: normalised on every row; printed civil names stay in the texts |
| C13 | The two gzip-stored captures needed an explicit provenance note | **Applied**: each records the gzip identity as served and the decoded identity |
| C14 | The live SGIP record of the organ in force will change; the records carry personal data | **Applied**: organ 570664 is a lead; the four closed organs are claims only; the extracts carry no personal field, and the test asserts it |

Missing primary records found by the checks:

| # | Record | Outcome |
|---|---|---|
| MA1 | Boletim Nacional special edition, December 1994 | **Imported** (`br_pt_bn_edicao_especial_199412`) |
| MA2 | PT Notícias nº 105 (July 2001) | **Imported** (`br_pt_noticias_105_20010720`) |
| MA3 | PT Notícias nº 52 (September 1997) | **Imported** (`br_pt_noticias_052_19970920`) |
| MA4 | PT Notícias nº 106 (August 2001) | **Imported** (`br_pt_noticias_106_20010804`) |
| MA5 | PT Notícias nº 107 (August-September 2001) | **Imported** (`br_pt_noticias_107_20010819`) |
| MA6 | The PT site's Rui Falcão biography (capture of 17 June 2017) | **Imported** (`br_pt_site_rui_falcao_biography_20170617`), retrospective |
| MB1 | Chamber diary of 11 February 2010 | **Imported** (`br_pt_camara_dcd_berzoini_farewell_20100211`) |
| MB2 | Portal do PT, 11 February 2010 | **Imported** (`br_pt_portal_berzoini_farewell_reported_20100211`) |
| MB3 | Chamber diary of 19 December 2002 | Lead (B10) |
| MB4 | Portal do PT cod=38515 (18 September 2005) | **Imported** (`br_pt_portal_ped2005_voting_closed_20050918`) |
| MB5 | Portal do PT cod=39123 (9 October 2005) | **Imported** (`br_pt_portal_ped2005_second_round_closed_20051009`) |
| MB6 | Portal do PT cod=39227 (14 October 2005) | **Imported** (`br_pt_portal_tarso_meets_pcdob_psb_20051014`) |
| MB7 | Portal do PT cod=39190 (13 October 2005) | **Imported** (`br_pt_portal_tarso_announces_final_results_20051013`) |
| MB8 | Diretório guidelines for the III Congresso (26 December 2006) | **Imported** (`br_pt_dn_guidelines_3rd_congress_20061226`) |
| MB9 | PED 2009 rule, Article 47 | **Imported** (`br_pt_portal_ped2009_posse_rule_20100111`), procedure only |
| MC1 | The 20 March 2025 'presidente efetivo' page | **Imported** (`br_pt_portal_humberto_reiterates_unity_20250320`) |
| MC2 | The 3 August 2025 13h27 page | **Imported** (`br_pt_portal_edinho_speech_report_20250803`) |
| MC3 | The 8 May 2011 'Novo comando do PT' page | **Imported** (`br_pt_portal_novo_comando_20110508`) |
| MC4 | The 13 March 2025 'presidente nacional interino' page | **Imported** (`br_pt_portal_humberto_ped_mobilization_20250313`) |
| MC5 | Guia do Congressista, 10 December 2013 | **Imported** (`br_pt_portal_guia_5th_congress_20131210`) |
| MC6 | The Diretório's election and posse of the executive, 11 December 2013 | **Imported** (`br_pt_portal_dn_elects_cen_20131211`) |
| MC7 | The Diretório composition, 18 December 2013 | **Imported** (`br_pt_portal_dn_composition_20131218`) |
| MC8 | 'Gleisi Hoffmann é reeleita presidenta', 24 November 2019 | **Imported** (`br_pt_gleisi_reelected_text_20191124`) |
| MC9 | '7º Congresso Nacional elege nova direção do PT', 26 November 2019 | **Imported** (`br_pt_7th_congress_new_leadership_20191126`) |
| MC10 | 'PT parabeniza Gleisi', 28 February 2025 | **Imported** (`br_pt_portal_gleisi_sri_congratulations_20250228`) |

Other changes made to fit the packet's rules rather than a numbered defect:

- The part C record of the SGIP organ in force (570664) and its two claims are withdrawn (C14); PT-PRES-10 rests on
  the party's archived Diretório page.
- The dossiers' `attested_period` values are not stored anywhere: a claim dated only by a span, an issue or a month has
  no structured date, as in the stacked packets.
- All ids are prefixed `br_pt_` so that the separation from `br_presidency` is testable, and every cross-reference in
  the texts uses the new ids.
- The portal item of 18 March 2003 was located for this packet (among the archived portal items of 17-26 March 2003) to
  date Genoino's holder after his election; it was downloaded twice, 30 minutes apart.

## Suggested next work orders

These are proposals for the integrator. They are not created in `work-orders.json`.

- `C01-Brazil-PT-002`: the day Lula's presidency ended in 1994 and the form of Rui Falcão's 1994 presidency (the Boletim
  Nacional of May-June 1994 and the Diretório minutes), and who led the party from December 1994 to August 1995.
- `C01-Brazil-PT-003`: the Diretório Nacional minutes of 7 December 2002, 15-16 March 2003 and 9 July 2005 (the source
  of the compilation's notes), to date Genoino's designation, Dirceu's resignation and the form of Genoino's 2005 act.
- `C01-Brazil-PT-004`: the PED 2007 result and any 2008 posse, and the 2006 leave and return of Berzoini.
- `C01-Brazil-PT-005`: the day each later term ended (Tarso Genro 2005, Falcão 2017, Gleisi Hoffmann 2025, Humberto
  Costa 2025) and a record of Edinho Silva's assumption on 3 or 4 August 2025, once pt.org.br is reachable without a
  security check.
- `C01-Brazil-PT-006`: other PT party offices (secretary-general, vice-presidents, caucus leaders) as separate roles if
  wanted.

## Integration notes (outside this packet's file boundary)

- **Stack, base and claim:** `claude/c01-br-22` is stacked on `claude/c01-br-17` at `7d71acef` (itself stacked on
  `claude/c01-br-10`); the stacked branch was fetched on 25 September 2026 and had no commits beyond the stack point, so
  no merge was needed. Claim commit `45821428` holds only the handoff. **Merge CLAUDE-C01-10, then CLAUDE-C01-17, then
  this packet.** This packet extends `brazil.json` additively: 108 source records appended after CLAUDE-C01-17's; on
  the PT observation, `roles` set to the one new role and the new ids appended to its `sources` and `claim_ids`; one
  coverage note appended to the PT observation and one to the packet. No existing source record or extract is edited,
  and no CLAUDE-C01-10 or CLAUDE-C01-17 record changes.
- `research-index.json` is regenerated in a **separate commit**, and it is **the only file shared with other pending
  packets** (the parallel C01 packets touch other countries' packets). New totals against `7d71acef`: 328 sources and
  2,136 claims (previously 220 and 1,959); 28 institution observations (unchanged); Brazil has one institution, three
  role observations (previously two), 408 claims and 32 entries pending mapping, and its four discovery batches stay 10,
  10, 10 and 2 members (92 batches in all). Regenerate the index after the other packets merge.
- Pinned tests re-expressed, none loosened and no assertion removed:
  - `test_brazil_research_s10f.py`: counts (entries, sources, claims, roles) are now (32, 172, 408, 3) from
    (32, 64, 231, 2); the 108 new sources are pinned to five hosts and the 2026-09-25 access date; the source count,
    the order of CLAUDE-C01-17's 11 sources and of the last 108 are pinned; the guard that no organization has roles
    becomes an exact pin (the PT observation has exactly `br_pt_president`, every other organization none);
    `mapping_pending` (32) and the work-order sizes are unchanged. It imports `NEW_SOURCES` from the new test.
  - `test_brazil_presidents_c01_10.py`: the full source list is this packet's 48, CLAUDE-C01-17's 11, then
    CLAUDE-C01-22's 108; entries and roles (32, 3); the rows loaded after the original sources are CLAUDE-C01-10's,
    CLAUDE-C01-17's and CLAUDE-C01-22's claims, exactly, and the event map stays exact for CLAUDE-C01-10's 112 claims;
    `role_observations` is 3. Every holder, identity and page pin is untouched. It imports the new test module.
  - `test_brazil_vice_presidents_c01_17.py`: counts (32, 172, 408, 3); its 11 sources are pinned at their position after
    CLAUDE-C01-10's, the total is pinned with CLAUDE-C01-22's 108 (all `br_pt_`); the packet coverage note is pinned at
    `[-2]` and CLAUDE-C01-22's at `[-1]`; `role_observations` is 3. Its stale-id guard on the ModDate-derived day
    `20250715` is re-expressed exactly: that day must not appear anywhere in the packet outside CLAUDE-C01-22's records,
    and within them only on the PED 2025 totalization of 15 July 2025 and the one claim that cites it. It does not
    import the new module (the new module imports it).
- `test_campaign_census` needs `spheres-sim/data`, which is absent from this sparse worktree; it was not run here, and
  the sparse checkout was not widened.
- The atlas (`tools/ui/leadership-research-review.js`) will show the PT observation with one role and eighteen holders;
  six have a start, one an end. No UI code changed.
- New vocabulary: the role kind `party_leader` was already in `ROLE_KINDS`; the source types and event kinds listed in
  the test (`FROM_KINDS`, `UNTIL_KINDS`, `ACTING_KINDS`, `ELECTION_KINDS`, `DEPARTURE_KINDS`, `RETROSPECTIVE_KINDS`)
  are new and pinned.
- Rulings for Codex: Lula's start on a speech (fallback attested_on 1990-06-03); Rui Falcão as a 1994 holder; Humberto
  Costa's start on his spoken statement (fallback attested_on 2025-03-20); Edinho Silva's start on 3 August 2025 against
  the prospective 4 August (fallback attested_on 2025-09-17); Dutra's until on the party's stated resignation day; the
  second Berzoini holder on a capture-dated page; Berzoini's stated last day as a claim only.
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
