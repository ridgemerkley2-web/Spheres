# Brazilian vice-presidents 17: holders, exercise and succession, 1990-2026

Packet: **CLAUDE-C01-17**. State: **ready_for_review** (not complete).
Owner: Claude. Integrator/reviewer: Codex. Branch `claude/c01-br-17`, **stacked on CLAUDE-C01-10**
(`claude/c01-br-10` at `73e5fd36`, ready for review and not yet integrated); claim commit `774af12d`. Research access:
24 September 2026 (local; some downloads fall after midnight UTC on 25 September). The historical cutoff stays
**7 September 2026**.

This packet adds a second role to CLAUDE-C01-10's `br_presidency` institution in [brazil.json](brazil.json):
`br_vice_president` ("Vice-President of the Federative Republic of Brazil", kind `institutional_office`). It reviews
ten observations from 1 January 1990 to the cutoff: the office when the period opens, Itamar Franco's posse of
15 March 1990, his exercise of the Presidency and succession in 1992, the posses of 1995, 1999, 2003, 2007, 2011 and
2015, Michel Temer's interim exercise and succession in 2016, the posses of 2019 and 2023, and an official attestation
of the Vice-President in office in 2026. It adds 11 sources and 85 claims (61 of them appended to 13 source records
whose responses CLAUDE-C01-10 already records), cites 13 CLAUDE-C01-10 claims on the new role, and records nine
holder observations, a role scope note, two institution coverage notes and one packet coverage note. CLAUDE-C01-10's
`br_president` role, its twelve holders and its 112 claims are unchanged; no Vice-President exercising the Presidency
becomes a holder of either role. It adds no organization, game mapping, lifespan, portrait or avatar. The parent scope
(C01, C06, S23, WC1 and CP1) remains open.

The research was done in three parts (A: 1985-1992; B: 1995-2015; C: 2016-2026), and each part was checked
independently before this packet was written. Every checker defect is applied or explained below (see
[Checker defects](#checker-defects)).

## Outcome

| ID | Question | Decision |
|---|---|---|
| BR-VP-01 | The office on 1 January 1990: any holder, or a source-stated vacancy | **Unresolved:** no source attests a Vice-President in office that day and none states that the office was vacant; the 1985 record declares only the Presidency vacant; no holder and no vacancy is recorded |
| BR-VP-02 | Itamar Franco's election, diplomação and posse of 15 March 1990 | **Accepted:** ballots of 15 Nov and 17 Dec 1989; TSE diploma of 30 Dec 1989; declared invested on 15 Mar 1990 for a period beginning that day (his start) |
| BR-VP-03 | 1992: the Vice-President's exercise of the Presidency and any vacancy after his posse as President | **Accepted in part:** exercise attested on 11 Dec 1991 and from 2 Oct 1992 (claims only); announced notification and succession to the Presidency on 29 Dec 1992 (claims only); no source states that the Vice-Presidency ended or became vacant |
| BR-VP-04 | Marco Maciel's posses of 1 January 1995 and 1999 | **Accepted:** both posses state a period beginning that day (two starts); no end is stated |
| BR-VP-05 | José Alencar's posses of 1 January 2003 and 2007 | **Accepted:** both posses state a period beginning that day (two starts); Rousseff's 2011 tribute is a claim only |
| BR-VP-06 | Michel Temer's posses as Vice-President of 1 January 2011 and 2015 | **Accepted:** both posses state a period beginning that day (two starts); his `br_president` holder is unchanged |
| BR-VP-07 | 2016: the interim exercise, the posse as President and any stated vacancy that followed | **Accepted in part:** notification of 12 May 2016 and its signed filing, exercise from 12 May and succession on 31 Aug (claims only); no source states that the Vice-Presidency ended or became vacant |
| BR-VP-08 | Hamilton Mourão's posse of 1 January 2019 | **Accepted:** declared invested for a period beginning that day (his start); his exercise of the Presidency on 30-31 Dec 2022 is claims only |
| BR-VP-09 | Geraldo Alckmin's posse of 1 January 2023 | **Accepted:** declared invested for a period beginning that day (his start); the printed 2027 end is text only |
| BR-VP-10 | An official attestation of the Vice-President in office before the cutoff | **Accepted:** Laws No. 15.434 (16 Jun 2026) and 15.436 (17 Jun 2026) signed by Alckmin as Vice-President in exercise of the Presidency, recorded as role claims (acting service never feeds a holder) |

The resulting holder observations of `br_vice_president`, in date order:

| Holder | `attested_on` | `from` | `until` | Basis |
|---|---|---|---|---|
| Itamar Franco | null | 1990-03-15 | null | posse declared "para o período de 15 de março de 1990" and the signed termo (DCN 8/1990) |
| Marco Maciel | null | 1995-01-01 | null | posse declaration and joint termo (DCN 1/1995) |
| Marco Maciel | null | 1999-01-01 | null | posse declaration and joint termo (DCN 1/1999) |
| José Alencar | null | 2003-01-01 | null | posse declaration and signed termo (DCN 1/2003) |
| José Alencar | null | 2007-01-01 | null | posse declaration and signed termo (DCN 1/2007) |
| Michel Temer | null | 2011-01-01 | null | posse declaration and signed termo (DCN 1/2011) |
| Michel Temer | null | 2015-01-01 | null | posse declaration and signed termo (DCN 1/2015) |
| Hamilton Mourão | null | 2019-01-01 | null | posse declaration, termo read into the record and signed termo (DCN 1/2019) |
| Geraldo Alckmin | null | 2023-01-01 | null | posse declaration, termo read into the record and signed termo (DCN 1/2023) |

### How a start and an end are decided

The packet applies CLAUDE-C01-10's rule to the second role. A holder has `from` only where a declaration of posse or a
termo de posse states that the person was invested as Vice-President for a period beginning that day, and `until` only
where a source states the day the vice-presidential office ended; otherwise the holder is dated by `attested_on`.
Same-session styling after the posse ("Vice-Presidente da República") supports a start but never makes one. Every
holder here has a stated start, and none has an end, because no reviewed primary source states the day a
vice-presidential term ended or that the Vice-Presidency became vacant.

Five kinds of statement were proposed or could be read as ends and are refused:

- **The Vice-President's posse as President.** The 1992 and 2016 termos record the Vice-President invested as President
  under Article 79 by virtue of the resignation (1992) or the vacancy (2016) of the office of President. They are claims
  of kind `vice_president_succeeds_to_presidency` on this role; neither says that the vice-presidential office ended or
  that the Vice-Presidency became vacant, so neither is an end.
- **The vacancy of the office of President.** CLAUDE-C01-10's `br_presidency_vacancy_declared_19921229` and the 2016
  vacancy stated in DCN 15/2016 are the Presidency's; the first is not cited on this role at all (check A4), and a test
  pins that no Presidency vacancy claim is cited here.
- **Periods declared at a posse.** Every posse prints a period (for example 1 January 2015 to 31 December 2018 for
  Temer); they are prospective, and Alckmin's printed end of 4 January 2027 falls after the cutoff and is text only.
- **Retrospective spans and chronologies.** The Vice-Presidency's 2024 publication gives each Vice-President a span
  (Itamar Franco 15/03/1990-29/12/1992; Temer 01/01/2011-31/08/2016; Mourão 01/01/2019-31/12/2022) and draws unlabelled
  grey segments in its chronology; the Presidency Library's Collor page gives Itamar Franco's vice-presidency as
  15.03.1990-29.12.1992. They are claims with no structured date.
- **A successor's posse, re-election, the incumbent's styling before a new posse and a successor's tribute.** None is
  used as an end (Maciel 1999 and Alencar 2007 are styled Vice-President before their second posses; Rousseff pays
  tribute to Alencar in 2011).

The Vice-President's exercise of the Presidency (1985, 1991, 1992, 2016, 2022 and 2026) is claims only. It never makes
the Vice-President a holder of `br_president`, never a separate holder of `br_vice_president`, and it neither splits nor
ends a holder. The claims that record it carry the role title "Vice-President of the Republic in exercise of the office
of President", the title CLAUDE-C01-10 gives its ten equivalent rows. Those ten rows keep `role_id` `br_president`
(they were recorded before this role existed) and are cited by this role; the new exercise rows carry `role_id`
`br_vice_president` (check A5).

### Date ledger

Each row is a separate dated fact with its own claim; two facts on one day stay two claims. Claims marked (C01-10) are
CLAUDE-C01-10's, cited on this role.

| Date | Event | Claim or field |
|---|---|---|
| 15 Mar 1985 | Sarney's oath as Vice-President, and his exercise of the Presidency from that day, as recited on 22 Apr 1985 | `br_sarney_vp_oath_recited_19850315`, `br_sarney_vp_exercise_from_recited_19850315` (context, outside the window) |
| 21 Apr 1985 | Mensagem nº 232: Sarney continues "agora na qualidade de sucessor" | `br_sarney_mensagem232_continues_as_successor_19850421` |
| 22 Apr 1985 | Congress declares the office of President vacant; the Vice-President succeeds under Article 77 | `br_presidency_vacant_vp_sarney_succeeds_19850422` |
| 17 Dec 1989 | Itamar Franco elected at the ballots of 15 Nov and 17 Dec 1989 | `br_itamar_vp_elected_ballots_19891217`, `br_itamar_vp_second_round_recited_19891217` |
| 30 Dec 1989 | TSE diploma; the posse record's recital; diplomação ceremony | `br_itamar_vp_tse_diploma_issued_19891230`, `br_itamar_vp_diplomacao_recited_19891230`, `br_tse_diplomacao_ceremony_catalogued_19891230` (C01-10) |
| 15 Mar 1990 | Oath; posse declared; termo | `br_itamar_vp_oath_before_congress_19900315`, `br_itamar_vp_posse_declared_19900315`, `br_itamar_vp_termo_de_posse_19900315`; Itamar Franco `from` |
| 11 Dec 1991 | Decree signed by the Vice-President in exercise | `br_itamar_vp_exercising_dnn428_19911211` (claim only) |
| 1 Oct 1992 | Senate message telling the Vice-President to assume | `br_vice_president_instructed_to_assume_19921001` (C01-10) |
| 2 Oct 1992 | Exercise dated from this day by Itamar Franco's own statement | `br_itamar_states_exercise_from_19921002` (C01-10) |
| 5 Oct 1992 | Law No. 8.469 and the ministers' posse address, as Vice-President in exercise | `br_itamar_vp_exercising_lei_8469_19921005`, `br_itamar_vp_in_exercise_speech_19921005` |
| 7 Oct 1992 | Law No. 8.471, as Vice-President in exercise | `br_itamar_exercising_signs_lei_8471_19921007` (C01-10) |
| 29 Dec 1992 | Notification announced after the Presidency is declared vacant; the Vice-President invested as President under Article 79 | `br_itamar_vp_notification_announced_19921229`, `br_itamar_vp_invested_as_president_19921229` (claims only, never an end) |
| 3 Oct 1994 / 17 Dec 1994 | Maciel elected; diplomado (Congress record and TSE diploma) | `br_maciel_elected_first_round_19941003`, `br_maciel_diplomado_tse_19941217`, `br_maciel_tse_diploma_issued_19941217` |
| 1 Jan 1995 | Oath; posse declared; termo; saluted at the close | `br_maciel_oath_before_congress_19950101`, `br_maciel_posse_declared_19950101`, `br_maciel_termo_de_posse_19950101`, `br_maciel_saluted_vice_president_closing_19950101`; Maciel `from` |
| 4 Oct 1998 / 12 Dec 1998 | Maciel re-elected; TSE diploma | `br_maciel_reelected_first_round_19981004`, `br_maciel_diplomado_tse_19981212` |
| 1 Jan 1999 | Styled incumbent at the opening; oath; posse declared; termo; saluted | `br_maciel_styled_incumbent_opening_19990101` (claim only), `br_maciel_oath_before_congress_19990101`, `br_maciel_posse_declared_19990101`, `br_maciel_termo_de_posse_19990101`, `br_maciel_saluted_vice_president_cardoso_address_19990101`; Maciel `from` |
| 27 Oct 2002 / 14 Dec 2002 | Alencar elected; TSE diploma | `br_alencar_elected_20021027`, `br_alencar_diplomado_tse_20021214` |
| 1 Jan 2003 | Oath; posse declared; termo; saluted | `br_alencar_oath_before_congress_20030101`, `br_alencar_posse_declared_20030101`, `br_alencar_termo_de_posse_20030101`, `br_alencar_saluted_vice_president_lula_address_20030101`; Alencar `from` |
| 29 Oct 2006 / 14 Dec 2006 | Alencar re-elected; TSE diploma | `br_alencar_reelected_20061029`, `br_alencar_diplomado_tse_20061214` |
| 1 Jan 2007 | Styled incumbent; oath with signed termo de compromisso; posse declared; termo; saluted | `br_alencar_styled_incumbent_opening_20070101` (claim only), `br_alencar_oath_before_congress_20070101`, `br_alencar_posse_declared_20070101`, `br_alencar_termo_de_posse_20070101`, `br_alencar_saluted_vice_president_lula_address_20070101`; Alencar `from` |
| 31 Oct 2010 / 17 Dec 2010 | Temer elected; TSE diploma | `br_temer_vp_elected_20101031`, `br_temer_vp_diplomado_tse_20101217` |
| 1 Jan 2011 | Oath; posse declared; termo; styled after the posse; Rousseff's tribute to Alencar | `br_temer_vp_oath_before_congress_20110101`, `br_temer_vp_posse_declared_20110101`, `br_temer_vp_termo_de_posse_20110101`, `br_temer_vp_styled_after_posse_20110101`; Temer `from`; `br_alencar_tribute_rousseff_address_20110101` (claim only) |
| 26 Oct 2014 / 18 Dec 2014 | Temer re-elected; TSE diploma | `br_temer_vp_reelected_20141026`, `br_temer_vp_diplomado_tse_20141218` |
| 1 Jan 2015 | Oath; posse declared; termo; styled after the posse | `br_temer_vp_oath_before_congress_20150101`, `br_temer_vp_posse_declared_20150101`, `br_temer_vp_termo_de_posse_20150101`, `br_temer_vp_styled_after_posse_20150101`; Temer `from` |
| 12 May 2016 | Mandado de Notificação; its signed copy filed; notification served (report); MP 726 by the Vice-President in exercise; the matter page's record | `br_temer_notified_to_assume_interim_20160512`, `br_temer_vp_notification_filed_signed_20160512`, `br_radio_senado_notifications_served_20160512` (C01-10), `br_temer_vp_in_exercise_mpv726_20160512` (C01-10), `br_senate_record_vp_notified_20160512` (C01-10) (claims only) |
| 31 Aug 2016 | DOU masthead, Mensagem 144 and the posse session's opening name the Vice-President in exercise; termo of his posse as President under Article 79 | `br_dou_masthead_temer_vp_in_exercise_20160831`, `br_mensagem144_to_vp_in_exercise_20160831`, `br_dcn_vp_in_exercise_at_posse_opening_20160831` (C01-10), `br_temer_vp_succeeds_art79_termo_20160831` (claims only, never an end) |
| 28 Oct 2018 / 10 Dec 2018 | Mourão elected; TSE diploma | `br_mourao_elected_20181028`, `br_mourao_diplomado_tse_20181210` |
| 1 Jan 2019 | Oath and signed termo de compromisso; posse declared; termo read and signed; saluted | `br_mourao_oath_before_congress_20190101`, `br_mourao_termo_de_compromisso_20190101`, `br_mourao_posse_declared_20190101`, `br_mourao_termo_read_20190101`, `br_mourao_signed_termo_de_posse_20190101`, `br_mourao_saluted_vice_president_bolsonaro_address_20190101`; Mourão `from` |
| 30-31 Dec 2022 | Decrees No. 11.322 and 11.324 by the Vice-President in exercise | `br_mourao_vp_in_exercise_d11322_20221230`, `br_mourao_vp_in_exercise_d11324_20221231` (C01-10; claims only) |
| 30 Oct 2022 / 12 Dec 2022 | Alckmin elected; TSE diploma | `br_alckmin_elected_20221030`, `br_alckmin_diplomado_tse_20221212` |
| 1 Jan 2023 | Oath and signed termo de compromisso; posse declared; termo read and signed; saluted | `br_alckmin_oath_before_congress_20230101`, `br_alckmin_termo_de_compromisso_20230101`, `br_alckmin_posse_declared_20230101`, `br_alckmin_termo_read_20230101`, `br_alckmin_signed_termo_de_posse_20230101`, `br_alckmin_saluted_vice_president_lula_address_20230101`; Alckmin `from` |
| 16-17 Jun 2026 | Laws No. 15.434 and 15.436 by the Vice-President in exercise | `br_alckmin_vp_exercising_l15434_20260616`, `br_alckmin_vp_exercising_l15436_20260617` (claims only) |
| undated | Retrospective spans and chronology: the Vice-Presidency's publication (vacancies "em dez ocasiões"; spans for Sarney, Itamar Franco, Temer, Mourão and Alckmin; a grey 2016-2019 segment) and the Presidency Library's Collor and Itamar Franco pages | `br_vpbio_*` (seven claims), `br_bibpr_collor_page_vp_itamar_span_19900315_19921229`, `br_bibpr_itamar_phase_19921002_period_19921229` (C01-10) (no structured date) |

Date conventions follow CLAUDE-C01-10: `attested_on` is the day of the observed event as the source dates it, a page's
issue date is `published_date`, a recital is dated by the day it recalls, and no clock time is stored.

## Observations

### BR-VP-01 — The office on 1 January 1990

Evidence: the Diário do Congresso Nacional nº 36 of 23 April 1985 records the joint session of 22 April 1985. The
President of Congress recalled that Vice-President José Sarney took the constitutional oath before Congress on
15 March 1985 and had since then been exercising the Presidency because of the President-elect's impediment
(`br_sarney_vp_oath_recited_19850315`, `br_sarney_vp_exercise_from_recited_19850315`), and declared that, Tancredo
Neves having died and the office of President being vacant, it fell to the Vice-President, as successor, to exercise it
under Article 77 (`br_presidency_vacant_vp_sarney_succeeds_19850422`). Sarney's Mensagem nº 232 of 21 April says he
continues to exercise the office "agora na qualidade de sucessor"
(`br_sarney_mensagem232_continues_as_successor_19850421`). The Vice-Presidency's 2024 publication gives Sarney's
vice-presidential span as 15/03/1985 to 21/04/1985 and says that the office was left vacant on ten occasions, and its
chronology draws a grey segment between 1985 and 1990 (`br_vpbio_sarney_vp_span_19850315_19850421`,
`br_vpbio_vacancy_chronology_undated`).

Decision: unresolved. No source attests any Vice-President in office on 1 January 1990, and none states that the office
was vacant that day; the 1985 record declares only the Presidency vacant. No holder and no vacancy is recorded. The
1985 claims are pre-period context and feed nothing.

Limits: the retrospective publication does not list or date its ten vacancies, and its grey segments are unlabelled.
The Presidency Library's Sarney page (CLAUDE-C01-10's `br_bibpr_sarney_page_2016`) leaves its "Vice-presidente" box
empty; an absence, not recorded as a claim. The Senate's full-text diary search, which could look for a statement that
the office was vacant, is behind a security check.

### BR-VP-02 — Itamar Franco's election, diplomação and posse

Evidence: the TSE digital library's photograph of the diploma (item bdtse/5202), dated Brasília 30 December 1989 and
signed by Minister Francisco Rezek and the other ministers, certifies that Itamar Augusto Cautiero Franco was elected
Vice-President at the ballots of 15 November and 17 December 1989 (`br_itamar_vp_tse_diploma_issued_19891230`,
`br_itamar_vp_elected_ballots_19891217`); the TSE catalogue of the diplomação ceremony is CLAUDE-C01-10's
`br_tse_diplomacao_ceremony_catalogued_19891230`, cited here (check A6). The DCN nº 8/1990 records the opening recital
of the second-round election and diplomação (`br_itamar_vp_second_round_recited_19891217`), the oath of the President
and Vice-President (`br_itamar_vp_oath_before_congress_19900315`), the declaration "Declaro empossados, para o período
de 15 de março de 1990 a 1º de janeiro de 1995, como Presidente e Vice-Presidente ..."
(`br_itamar_vp_posse_declared_19900315`), and the signed termo under Article 78 with its recital of the TSE diplomação
of 30 December (`br_itamar_vp_termo_de_posse_19900315`, `br_itamar_vp_diplomacao_recited_19891230`).

Decision: accepted. Itamar Franco's `from` as Vice-President is 15 March 1990. The election, diplomação and oath are
claims, never holder dates; the printed period end is prospective.

Limits: the diploma photograph is the TSE's collection exemplar and states no acquisition date; the TSE's own minutes
of the session were not found.

### BR-VP-03 — 1991-1992: exercise of the Presidency and succession

Evidence: an unnumbered decree of 11 December 1991 is issued by "O VICE-PRESIDENTE DA REPÚBLICA, no exercício do cargo
de PRESIDENTE DA REPÚBLICA" and signed Itamar Franco (`br_itamar_vp_exercising_dnn428_19911211`), the only primary act
found for him between March 1990 and October 1992. In 1992 the Senate's message of 1 October told him to assume the
Presidency immediately (`br_vice_president_instructed_to_assume_19921001`), his own declaration of 29 December dates his
exercise from 2 October (`br_itamar_states_exercise_from_19921002`), and he signed Law No. 8.469 on 5 October and Law
No. 8.471 on 7 October as Vice-President in exercise (`br_itamar_vp_exercising_lei_8469_19921005`,
`br_itamar_exercising_signs_lei_8471_19921007`); the Presidency Library's transcript of his 5 October address at the
ministers' posse carries the same styling (`br_itamar_vp_in_exercise_speech_19921005`). On 29 December, after the
office of President was declared vacant, the President of Congress said he would notify ("irei cientificar") the
Vice-President to appear at 12:30 to take the oath as President (`br_itamar_vp_notification_announced_19921229`), and
the termo records "o Sr. Dr. Itamar Augusto Cautiero Franco, Vice-Presidente da República" invested under Article 79 by
virtue of the resignation (`br_itamar_vp_invested_as_president_19921229`). The Presidency Library's retrospective
pages and the Vice-Presidency's profile give his vice-presidency as ending on 29 December 1992
(`br_bibpr_collor_page_vp_itamar_span_19900315_19921229`, `br_vpbio_itamar_vp_span_19900315_19921229`,
`br_bibpr_itamar_phase_19921002_period_19921229`).

Decision: accepted in part. The exercise of the Presidency is recorded as claims only; his `br_president` holder
(CLAUDE-C01-10, from 29 December 1992) is unchanged and no exercise becomes a holder. Not accepted: no source reviewed
states that the office of Vice-President became vacant or the day his vice-presidential office ended, so his
`br_vice_president` holder has no `until` and no vacancy is recorded. The 29 December vacancy is the Presidency's.

Limits: no act dated 2-4 October 1992 was found (the Senate's 1992 legislation index lists none; the Planalto index of
unnumbered decrees is behind a bot check). The notification of 29 December is announced, not recorded.

### BR-VP-04 — Marco Maciel's posses of 1995 and 1999

Evidence: the DCN nº 1/1995 records the first-round election of 3 October 1994, the diplomação of 17 December 1994,
Maciel's oath, the declaration that Cardoso and Marco Antônio de Oliveira Maciel are invested as President and
Vice-President for 1 January 1995 to 31 December 1998, the joint termo and the closing salutation
(`br_maciel_elected_first_round_19941003`, `br_maciel_diplomado_tse_19941217`, `br_maciel_oath_before_congress_19950101`,
`br_maciel_posse_declared_19950101`, `br_maciel_termo_de_posse_19950101`,
`br_maciel_saluted_vice_president_closing_19950101`). Because that issue reproduces no diploma, the TSE's photograph of
the diploma dated 17 December 1994 (bdtse/5204) is imported (`br_maciel_tse_diploma_issued_19941217`). The DCN nº
1/1999 reproduces the diploma of 12 December 1998 and records the re-election of 4 October 1998, the oath, the
declaration for the period beginning that day, the termo and Cardoso's salutation
(`br_maciel_reelected_first_round_19981004`, `br_maciel_diplomado_tse_19981212`, `br_maciel_oath_before_congress_19990101`,
`br_maciel_posse_declared_19990101`, `br_maciel_termo_de_posse_19990101`,
`br_maciel_saluted_vice_president_cardoso_address_19990101`); at its opening he is already seated as Vice-President
(`br_maciel_styled_incumbent_opening_19990101`).

Decision: accepted. Two holders start on 1 January 1995 and 1 January 1999; neither has an end. The styling before the
1999 posse is continuation context and neither ends the first holder nor starts the second.

Limits: the 2003 record does not name Maciel; no end of either term is stated.

### BR-VP-05 — José Alencar's posses of 2003 and 2007

Evidence: the DCN nº 1/2003 reproduces the TSE diploma of 14 December 2002 and records the election of 27 October 2002,
the oath, the declaration of posse for the period beginning 1 January 2003, the signed termo and Lula's salutation
(`br_alencar_elected_20021027`, `br_alencar_diplomado_tse_20021214`, `br_alencar_oath_before_congress_20030101`,
`br_alencar_posse_declared_20030101`, `br_alencar_termo_de_posse_20030101`,
`br_alencar_saluted_vice_president_lula_address_20030101`). The DCN nº 1/2007 does the same for the re-election of
29 October 2006 and the diploma of 14 December 2006, with his signed termo de compromisso
(`br_alencar_reelected_20061029`, `br_alencar_diplomado_tse_20061214`, `br_alencar_oath_before_congress_20070101`,
`br_alencar_posse_declared_20070101`, `br_alencar_termo_de_posse_20070101`,
`br_alencar_saluted_vice_president_lula_address_20070101`), and seats him as Vice-President before the new posse
(`br_alencar_styled_incumbent_opening_20070101`). In her 2011 address Rousseff pays tribute to "nosso querido
Vice-Presidente José Alencar" (`br_alencar_tribute_rousseff_address_20110101`).

Decision: accepted. Two holders start on 1 January 2003 and 1 January 2007; neither has an end. The tribute is a
successor administration's statement with no day, a claim only.

Limits: the 2003 issue also calls him "Senador José Alencar" and lists him as vice-president of a Senate council; both
are other offices and are not imported.

### BR-VP-06 — Michel Temer's posses as Vice-President, 2011 and 2015

Evidence: the DCN nº 1/2011 and nº 1/2015 reproduce the TSE diplomas of 17 December 2010 and 18 December 2014, his
signed termos de compromisso, the declarations of posse for periods beginning 1 January 2011 and 1 January 2015, the
signed joint termos and the salutations after the posse (`br_temer_vp_elected_20101031`,
`br_temer_vp_diplomado_tse_20101217`, `br_temer_vp_oath_before_congress_20110101`, `br_temer_vp_posse_declared_20110101`,
`br_temer_vp_termo_de_posse_20110101`, `br_temer_vp_styled_after_posse_20110101`, `br_temer_vp_reelected_20141026`,
`br_temer_vp_diplomado_tse_20141218`, `br_temer_vp_oath_before_congress_20150101`, `br_temer_vp_posse_declared_20150101`,
`br_temer_vp_termo_de_posse_20150101`, `br_temer_vp_styled_after_posse_20150101`).

Decision: accepted. Two holders start on 1 January 2011 and 1 January 2015; the 2016 events are BR-VP-07. His
`br_president` holder (CLAUDE-C01-10, from 31 August 2016) is unchanged.

Limits: the printed period ends are prospective; the 2015 diary's typed termo says "eleitos" and its signed facsimile
"reeleitos".

### BR-VP-07 — 2016: interim exercise and succession

Evidence: the Senate's complete case file of Denúncia nº 1/2016, volume 48, files the Mandado de Notificação of
12 May 2016 addressed to "MICHEL TEMER, Vice-Presidente da República Federativa do Brasil", signed by the President of
the Senate, telling him to assume the Presidency "imediata e interinamente" under Article 79
(`br_temer_notified_to_assume_interim_20160512`), and the Termo de Juntada of the same day certifying that the copy was
"assinado por S.Exa. no anverso" (`br_temer_vp_notification_filed_signed_20160512`; check C2). CLAUDE-C01-10's claims
record the service of the notification, the matter page's entry, MP 726 adopted as Vice-President in exercise, and on
31 August the DOU masthead, Mensagem 144 and the posse session's opening styling him Vice-President in exercise
(`br_radio_senado_notifications_served_20160512`, `br_senate_record_vp_notified_20160512`,
`br_temer_vp_in_exercise_mpv726_20160512`, `br_dou_masthead_temer_vp_in_exercise_20160831`,
`br_mensagem144_to_vp_in_exercise_20160831`, `br_dcn_vp_in_exercise_at_posse_opening_20160831`); they are cited, not
duplicated. The termo read into DCN nº 15/2016 records "o Senhor Michel Temer, Vice-Presidente da República" invested as
President under Article 79 by virtue of the vacancy that occurred on 31 August 2016
(`br_temer_vp_succeeds_art79_termo_20160831`); in the minutes the declaration of posse as President precedes the reading
(check C5). The Vice-Presidency's profile gives his span as 01/01/2011 to 31/08/2016 and its chronology draws an
unlabelled grey segment from 2016 to 2019 (`br_vpbio_temer_vp_span_20110101_20160831`,
`br_vpbio_chronology_grey_segment_2016_2019`).

Decision: accepted in part. The notification, exercise and succession are claims only: they neither split nor end his
2015 holder and never make him a `br_president` holder (his `br_president` start of 31 August 2016 is CLAUDE-C01-10's).
Not accepted: no primary source states that the Vice-Presidency ended or became vacant; the vacancy stated on
31 August 2016 is the Presidency's, and the retrospective span and grey segment are claims.

Limits: no source states the hour Temer began exercising the Presidency on 12 May 2016. A law of 1 September 2016 signed
by the President of the Chamber in exercise of the Presidency and the Chamber diaries of 1-2 September state no vacancy
of the Vice-Presidency (leads).

### BR-VP-08 — Hamilton Mourão's posse of 2019

Evidence: the DCN nº 1/2019 (the complete stored issue CLAUDE-C01-10 records) records the election of 28 October 2018
(`br_mourao_elected_20181028`), his oath and signed termo de compromisso (`br_mourao_oath_before_congress_20190101`,
`br_mourao_termo_de_compromisso_20190101`), the TSE diploma of 10 December 2018 signed by Minister Rosa Weber
(`br_mourao_diplomado_tse_20181210`), the declaration that Bolsonaro and Antonio Hamilton Martins Mourão are invested for
"o período de 1º de janeiro de 2019 a 31 de dezembro de 2022" in a session called "para o período a iniciar-se nesta
data" (`br_mourao_posse_declared_20190101`), the termo read and signed (`br_mourao_termo_read_20190101`,
`br_mourao_signed_termo_de_posse_20190101`) and Bolsonaro's salutation of "Sr. Hamilton Mourão"
(`br_mourao_saluted_vice_president_bolsonaro_address_20190101`). His exercise of the Presidency on 30 and 31 December
2022 is CLAUDE-C01-10's `br_mourao_vp_in_exercise_d11322_20221230` and `br_mourao_vp_in_exercise_d11324_20221231`, cited.

Decision: accepted. Mourão's holder starts on 1 January 2019 with no end; the decrees are claims only (check C9). The
Vice-Presidency's span (`br_vpbio_mourao_vp_span_20190101_20221231`) is retrospective.

Limits: an early 2019 decree in exercise (No. 9.690) could not be read: its captures are bot-check pages.

### BR-VP-09 — Geraldo Alckmin's posse of 2023

Evidence: the DCN nº 1/2023 records the election of 30 October 2022 (`br_alckmin_elected_20221030`), his oath and signed
termo de compromisso (`br_alckmin_oath_before_congress_20230101`, `br_alckmin_termo_de_compromisso_20230101`), the TSE
diploma of 12 December 2022 signed by Minister Alexandre de Moraes (`br_alckmin_diplomado_tse_20221212`), the declaration
of posse of Lula and Geraldo José Rodrigues Alckmin Filho "para o período de 1º de janeiro de 2023 a 4 de janeiro de
2027" (`br_alckmin_posse_declared_20230101`), the termo read and signed (`br_alckmin_termo_read_20230101`,
`br_alckmin_signed_termo_de_posse_20230101`) and Lula's salutation (`br_alckmin_saluted_vice_president_lula_address_20230101`).
The Vice-Presidency's profile says he is Vice-President "desde 1º de janeiro de 2023"
(`br_vpbio_alckmin_vp_since_20230101`). The diploma, signed termo and termo de compromisso (pages 20-26) are a new
page-scoped record of the same stored issue, because CLAUDE-C01-10's two records of it are scoped to pages 1-8 and 18-19.

Decision: accepted. Alckmin's holder starts on 1 January 2023 with no end; the 2027 end is prospective, after the cutoff
and text only.

Limits: the minutes print his spoken oath without "manter", while the signed termo de compromisso has the full formula.

### BR-VP-10 — An attestation before the cutoff

Evidence: Law No. 15.434 of 16 June 2026 (capture of 20 June 2026) and Law No. 15.436 of 17 June 2026 (capture of
16 August 2026) are each enacted by the Vice-President "no exercício do cargo de PRESIDENTE DA REPÚBLICA" and signed
Geraldo José Rodrigues Alckmin Filho (`br_alckmin_vp_exercising_l15434_20260616`,
`br_alckmin_vp_exercising_l15436_20260617`). Law No. 15.436 prints the formula with an en dash; the independent check
found it after the researcher's hyphen-only scan missed it, and re-scanned the 63 captured acts dated 9 June to
3 September 2026 by signatory: every later act is signed by Lula (check C1).

Decision: accepted. The two laws attest Alckmin as Vice-President in office on 16 and 17 June 2026, before the cutoff.
They are recorded as role claims and are not cited by his holder, because an act signed in exercise of the Presidency is
acting service, which never feeds a holder (check C9). They extend nothing.

Limits: an act not captured by the Internet Archive may exist; Law No. 15.435 of 17 June 2026 has no capture and its
live page is regenerated per request.

## Sources added

| Source ID | What | Response identity and provenance |
|---|---|---|
| `br_dcn_36_19850423` | DCN nº 36/1985: joint session of 22 Apr 1985 (vacancy of the Presidency; Sarney succeeds) | 237,692 bytes, `0a2f326e…b5278d`; Senate stored issue; PDF pages 1, 3 viewed |
| `br_tse_diploma_itamar_19891230` | TSE digital library: photograph of Itamar Franco's diploma (30 Dec 1989) | 677,161 bytes, `cc83b742…dd73af`; TSE repository file (ETag = MD5) |
| `br_planalto_dnn428_19911211` | Planalto: decree of 11 Dec 1991, signed by the Vice-President in exercise | 5,934 bytes, `02f18ec0…193612`; capture 2024-07-24; located by the check |
| `br_planalto_lei_8469_19921005` | Planalto: Law No. 8.469, signed by the Vice-President in exercise (5 Oct 1992) | 5,013 bytes, `5843b0cb…500d57`; capture 2008-10-14; located by the check |
| `br_bibpr_itamar_speech_19921005` | Presidency Library: address as Vice-President in exercise at the ministers' posse (5 Oct 1992) | 1,616,993 bytes, `4639b08f…2b2ab0`; capture 2025-04-21; PDF page 2 viewed |
| `br_vpr_biografias_vice_presidentes_2024` | Vice-Presidency: biographies and succession chronology (2024; retrospective) | 2,129,103 bytes, `ae0c5e07…93327f`; capture 2025-12-07; PDF pages 6, 7, 47, 49, 55, 57, 59 viewed |
| `br_tse_diploma_maciel_19941217` | TSE digital library: photograph of Maciel's diploma (17 Dec 1994) | 678,470 bytes, `120ec1e7…4ba51c`; TSE repository file (ETag = MD5) |
| `br_senado_den1_2016_autos_vol48` | Senate case file DEN 1/2016, vol. 48: Mandado de Notificação and Termo de Juntada (12 May 2016) | 29,934,149 bytes, `72e1b1e6…6f4b90`; Senate file; PDF pages 16, 17 viewed; located by the check |
| `br_cn_dcn1_20230102_p20_26` | DCN nº 1/2023 (complete issue; pp. 20-26): Alckmin's diploma, signed termo and termo de compromisso | 24,950,218 bytes, `d6c9c275…c854ee`; Senate file (same response as CLAUDE-C01-10's two DCN 1/2023 records); PDF pages 20, 23, 26 viewed |
| `br_planalto_lei15434_20260616` | Planalto: Law No. 15.434, signed by the Vice-President in exercise (16 Jun 2026) | 17,609 bytes, `07451520…68fb89`; capture 2026-06-20 |
| `br_planalto_lei15436_20260617` | Planalto: Law No. 15.436, signed by the Vice-President in exercise (17 Jun 2026) | 58,054 bytes, `d06c3098…ace246`; capture 2026-08-16; located by the check |

Existing source records gaining claims. Each is a response CLAUDE-C01-10 already records; its URL, identity,
`accessed_date` (23 September 2026), title and scope note are unchanged, and the new claims are appended after
CLAUDE-C01-10's with rows keyed to `br_vice_president`:

| Source ID | Claims added | Response identity | Pages rendered for these rows |
|---|---|---|---|
| `br_dcn_08_19900316` | 5 (BR-VP-02) | 1,255,764 bytes, `838abf03…7b6645` | 3, 4 |
| `br_dcn_70_19921230` | 2 (BR-VP-03) | 397,945 bytes, `dfe8e27a…020d33` | 3, 6 |
| `br_bibpr_collor_page_2016` | 1 (BR-VP-03) | 42,045 bytes, `e0ad5fdb…cbf49d` | HTML |
| `br_dcn_1_1995_posse_19950101` | 6 (BR-VP-04) | 845,143 bytes, `fb7ff244…abc5c5` | 6, 9 |
| `br_dcn_1_1999_posse_19990101` | 7 (BR-VP-04) | 939,787 bytes, `b7c0283e…2f2abb` | 3, 5, 6, 7, 8, 9 |
| `br_dcn_1_2003_posse_20030101` | 6 (BR-VP-05) | 1,212,635 bytes, `86f2d392…bca32d` | 3, 5, 6, 7 |
| `br_dcn_1_2007_posse_20070101` | 7 (BR-VP-05) | 1,829,092 bytes, `395e155d…d28822` | 3, 5, 7, 9 |
| `br_dcn_1_2011_posse_20110101` | 7 (BR-VP-05, BR-VP-06) | 506,362 bytes, `b75d4e99…baccbb` | 3, 5, 7, 8, 9 |
| `br_dcn_1_2015_posse_20150101` | 6 (BR-VP-06) | 8,131,269 bytes, `8f588186…861787` | 4, 5, 6, 7, 9, 10, 11 |
| `br_cn_dcn15_20160901` | 1 (BR-VP-07) | 8,585,228 bytes, `60e2c85c…824195` | 6 |
| `br_cn_dcn1_20190102_p5_11` | 5 (BR-VP-08) | 49,257,639 bytes, `2d8a63d5…0b24eb` | 6 |
| `br_cn_dcn1_20190102_full` | 3 (BR-VP-08) | 49,257,639 bytes, `2d8a63d5…0b24eb` | 15, 18, 20 |
| `br_cn_dcn1_20230102_p1_8` | 5 (BR-VP-09) | 24,950,218 bytes, `d6c9c275…c854ee` | 6 |

In each of these thirteen extracts CLAUDE-C01-10's `visual_review.pdf_pages_one_based` is unchanged; the pages rendered
for the Vice-President rows are recorded separately as `visual_review.c01_17_pdf_pages_one_based`, the text-layer pages
are named in the method text, the `bounded_scope` now names both packets' rows, and the provenance note records the
24 September re-downloads. Their snapshot bytes and SHA-256 in the packet are updated.

Six sources are raw Internet Archive captures (`id_` form) made before the cutoff; each records the capture URL as
`url`, the address inside it as `original_url` and the capture time in its extract. Five are official files: two
Senate stored diary issues (download=true), the Senate's stored case-file volume and two TSE repository bitstreams.
The DCN 1/2023 page-scoped record shares the stored issue's identity with CLAUDE-C01-10's records, in the pattern
CLAUDE-C01-10 uses for DCN 1/2019 and 1/2023. Each new source has a derived factual extract under [sources/](sources/)
in the packet's format (`spheres-c01-derived-factual-table/v1`): one row per claim, keyed by `claim_id`, with
`observation_id` `br_presidency`, `review_observation`, `role_id` `br_vice_president`, `holder_name`, `role_title`,
`event_kind`, `attested_on`, the claim's text and locator. Original pages, PDFs, images and renders are not checked in,
and no seal, coat of arms, signature, diploma image or photograph is republished.

The Vice-Presidency's publication and the Presidency Library's pages are retrospective and bound nothing. The two TSE
diploma photographs are the Court's collection exemplars; nothing shows that either is the exemplar handed to the
holder. The Library's speech transcript is a later printed text, supported by Law No. 8.469 of the same day.

New source types: `primary_vice_presidency_publication_archived`; the others reuse CLAUDE-C01-10's
(`primary_congress_session_record_pdf`, `primary_electoral_court_repository_image`,
`primary_presidency_legislation_page_archived`, `primary_presidency_speech_text_archived`,
`primary_senate_case_file_pdf`).

## Response identities and stability checks

Every recorded response was downloaded again by the researcher and the independent check of its part on 24 September
2026, and the late imports again for this packet; all match the recorded byte count and SHA-256.

| Response | Downloads (UTC) | Stability basis |
|---|---|---|
| DCN 36/1985 (codDiario 15364) | 22:01, 22:32 (+ cache-busted), check 22:43-22:44 and 23:04 (cache-busted) | stored issue, 30+ minutes and cache-busted |
| TSE bitstream, Itamar Franco diploma | 21:59, 22:32; check 22:44, 22:45, 23:05 (two cache-busted) | ETag and repository checksum = MD5 of the bytes |
| Decree of 11 Dec 1991 (capture 20240724055151) | check 22:58, 23:07; this packet 00:57 on 25 Sep | raw capture, two hours apart |
| Law No. 8.469 (capture 20081014095729) | check 22:57, 23:06; this packet 00:57 on 25 Sep | raw capture, two hours apart |
| Library speech PDF (capture 20250421205127) | 21:53, 22:32; check 22:45, 23:04 | raw capture, 30+ minutes |
| Vice-Presidency PDF (capture 20251207025501) | 21:50, 22:32, 22:56; check 22:45, 23:04 | raw capture, 30+ minutes |
| TSE bitstream, Maciel diploma | 21:54, 22:05 (cache-busted), 22:26; check 22:37, 23:04 (cache-busted) | ETag and repository checksum = MD5; no-store |
| Senate case file vol. 48 (dm=7749877) | check 00:10, 00:11 (cache-busted), 00:43 on 25 Sep; this packet 01:05 (cache-busted, Age 0) | stored scan with a fixed 2018 CreationDate; 33+ minutes and cache-busted |
| DCN 1/2023 complete issue (codDiario 111711) | CLAUDE-C01-10's identity of 23 Sep; 22:39 and the check 74-85 minutes later | stored issue, across days |
| Law No. 15.434 (capture 20260620021856) | 23:16, 23:30, 23:47; check 35-57 minutes after the first | raw capture, 31+ minutes |
| Law No. 15.436 (capture 20260816171933) | check 00:15, 00:20, 00:44, 00:50 on 25 Sep; this packet 00:57 | raw capture, 35 minutes |
| The six DCN posse issues 1995-2015 | CLAUDE-C01-10's identities of 23 Sep; check's cache-busted downloads 22:37:51-22:38:51 (Age 0) | stored issues, across days, from the origin |
| DCN 8/1990 and 70/1992 | CLAUDE-C01-10's identities; cache-busted 22:33, 22:43, 23:05 | stored issues, across days |
| DCN 15/2016 and 1/2019 | CLAUDE-C01-10's identities; 22:36-22:43 and the check 74-85 minutes later | stored issues, across days |
| Collor Library page (capture 20160812222130) | CLAUDE-C01-10's identity; 22:05 and check 22:45 | raw capture, across days |

Reproducibility notes for the reviewer. The Senate diary store serves `max-age=1800` and the document service
`max-age=3600`: plain re-downloads within that window can be edge-cache copies (the check saw `Age` of about 3,050 s), so
add a cache-busting query or wait. Several Senate transfers came back short with HTTP 200 (DCN 1/2015 by 1,744 bytes;
volume 48 by 1,046 and 1,854 bytes): compare the byte count with the Content-Length first and retry. The Internet
Archive timed out, refused connections and served "Temporarily Offline" at times during all three parts; every capture
used was fetched after spaced retries. No per-request page is recorded: no viewer page-range PDF (seqPaginaInicial), no
live Planalto or gov.br page and no cache-busting query is part of any recorded URL.

## Leads not imported

- The Senate's published image of the same Mandado de Notificação
  (https://www12.senado.leg.br/noticias/infograficos/2016/05/mandado-de-notificacao-michel-temer/temer.jpg; 362,353 bytes,
  `bdeae2d3…b954`, identical across cache-busted fetches at 22:57 and 23:52 UTC) and its dynamic wrapper page
  (39,281 bytes): the image has no Internet Archive capture, and the stored case-file copy (volume 48, fl. 18.146) is the
  recorded identity instead (check C2).
- Law No. 15.435 of 17 June 2026 (https://www.planalto.gov.br/ccivil_03/_ato2023-2026/2026/lei/l15435.htm), also signed
  by Alckmin in exercise: no Internet Archive capture, and its live page is always 18,220 bytes but has a new SHA-256 on
  each request (a per-request F5 token). Not reproducible.
- Decree No. 12.958 of 7 May 2026 (https://web.archive.org/web/20260508185121id_/https://www.planalto.gov.br/ccivil_03/_ato2023-2026/2026/decreto/d12958.htm;
  85,232 bytes, `8a240405…304c`, stable over 30 minutes) and Decree No. 12.939 of 16 April 2026 (capture 20260803155902
  of .../decreto/d12939.htm; 50,907 bytes, `c99bd0b8…5e80`), with Decrees 12.936-12.940 and 12.851-12.853: earlier 2026
  attestations in the same form; the two June laws are imported instead.
- Law No. 8.470 of 5 October 1992 (https://web.archive.org/web/20081010194528id_/http://www.planalto.gov.br/ccivil_03/Leis/L8470.htm;
  5,120 bytes, `f1a329a5…9c49`): same day, form and signatures as Law No. 8.469, a duplicate attestation.
- Decree No. 363 (https://web.archive.org/web/20081201211354id_/http://www.planalto.gov.br/ccivil_03/decreto/1990-1994/D0363.htm;
  3,716 bytes, `f3c095e9…82d0`): signed by the Vice-President in exercise, but fetched once by the check and its page
  dates it 12 December 1991 against a DOU of 11 December; the decree of 11 December 1991 is used instead.
- The Chamber diary of 3 October 1992 (https://imagem.camara.leg.br/Imagem/d/pdf/DCD03OUT1992.pdf; 3,465,016 bytes,
  `88742949…0b4f`) and DSF nº 165/1992 (codDiario=6431, a CLAUDE-C01-10 lead): members' speeches about the
  Vice-President's assumption, not an act.
- Chamber history pages (https://www2.camara.leg.br/a-camara/conheca/historia/Ex_presidentesCD_Republica/paes.html and
  .../inocencio.html): the President of the Chamber exercising the Presidency in 1989-1990 and 1993-1994; live history
  pages that state no vacancy of the Vice-Presidency.
- The Presidency Library's 1992 speech list (capture 20240222080132; 29,654 bytes, `e6ebd08d…e077`), used only to locate
  the 5 October transcript; its 16 December 1992 address has no archived PDF.
- The Vice-Presidency's HTML profile of Itamar Franco (https://www.gov.br/planalto/pt-br/vice-presidencia/acesso-a-informacao/institucional/biografia-dos-vice-presidentes-da-republica/itamar-franco):
  duplicates the imported publication.
- CLAUDE-C01-10's Sarney Library page: an empty "Vice-presidente" box and a list of legal substitutes; an absence, not a
  statement of vacancy.
- Six TSE originals of Vice-President diplomas, each byte-stable over a cache-busted second download with MD5 equal to
  the repository checksum: bdtse/5200 Maciel 1998 (bitstream 23f606e7-67c2-4b20-af1a-517213a328f9; 742,691 bytes,
  `e14c2516…5449`), bdtse/5203 Alencar 2002 (bitstream 1a12c506-d2c2-4f15-b496-4ae3b8e7fabb; 17,201,788 bytes,
  `574bc98d…41a4`), bdtse/5196 Alencar 2006 (bitstream d74dd68a-582f-4a04-9f68-8c83042edd6f; 868,878 bytes,
  `16679b32…86dc`), bdtse/5199 Temer 2010 (item https://bibliotecadigital.tse.jus.br/server/api/core/items/0d83a50c-3eb2-4041-ae8c-076a2f18a49f;
  bitstream 19f3eb3f-9ab1-440a-b0b7-594316559657; 184,242 bytes, `f224971d…a7cd`), and bdtse/5408 Temer 2014 front
  (bitstream 1062d3d7-3a67-4d85-83e3-509bc1455de8; 146,118 bytes, `06eaffd1…6332`) and back (bitstream
  77548c34-578b-4233-9ff5-54f95107c799; 84,712 bytes, `19d65de9…5acd`). They duplicate the diplomas reproduced in the
  Congress records, which already date each diplomação; recorded as leads (see the missing-records table).
- TSE ceremony photograph records (bdtse/533, 540, 1108, 541, 681, 682): archival descriptions, not session minutes.
- In DCN 1/2003, "o Senador José Alencar" and his vice-presidency of a Senate council: other offices, excluded.
- Law No. 13.332 of 1 September 2016 (https://web.archive.org/web/20160904005503id_/http://www.planalto.gov.br/ccivil_03/_Ato2015-2018/2016/Lei/L13332.htm;
  37,935 bytes, `e4e0bf43…9ca1`), signed by Rodrigo Maia as President of the Chamber in exercise of the Presidency: it
  states no reason and no vacancy of the Vice-Presidency; recording one would be an inference.
- The Chamber diaries of 1 and 2 September 2016 (https://imagem.camara.leg.br/Imagem/d/pdf/DCD0020160901001520000.PDF,
  7,022,668 bytes, `2d65320d…a9dc`; .../DCD0020160902001530000.PDF, 2,770,906 bytes, `877b5789…b76e`): searched, silent
  on a vacancy of the Vice-Presidency.
- The Planalto news item "Geraldo Alckmin toma posse como vice-presidente da República" (capture 20230105134541 of
  .../noticias/2023/01/geraldo-alckmin-toma-posse-como-vice-presidente-da-republica): news of a ceremony held in Congress.
- DCN nº 1/2026 (https://legis.senado.leg.br/diarios/BuscaPaginasDiario?codDiario=123478&download=true; 131,367,701
  bytes, not downloaded because of the disk limit): pages 1-20 read only through the regenerated page-range viewer (not
  an identity); the Vice-President is not recorded at the 2 February 2026 session.
- The Internet Archive CDX listing of 2026 Planalto acts used for the BR-VP-10 scan (390 rows, 323 act pages).
- pt.wikipedia.org's list of Vice-Presidents and its article on the line of succession: encyclopaedia, not fetched.
- The Constitution (Articles 77-81; https://www.planalto.gov.br/ccivil_03/Constituicao/ConstituicaoCompilado.htm):
  procedure only, never a date.

## Sources attempted

- gov.br Vice-Presidency pages (the live biographies PDF, /vice-presidencia, /conheca-a-vice-presidencia,
  /galeria-de-ex-vice-presidentes): 47-48 KB F5 JavaScript challenge pages (HTTP 200); the July 2026 captures of the PDF
  URL are also challenge pages. Not bypassed; the December 2025 capture of the PDF is used.
- The former Vice-Presidency domain (http://www.vicepresidencia.gov.br/): the host does not resolve in DNS (curl exit 6)
  (check B6).
- The Planalto index of unnumbered decrees (portal-legis, decretos-nao-numerados1): F5 challenge page; the CDX listing of
  the 1992 unnumbered-decree prefix was truncated at 500 rows. No act of 2-4 October 1992 was pinned.
- The Senate's full-text diary search: a 9,048-byte "Verificação de segurança" challenge page, not bypassed.
- The Diário Oficial da União viewer: a CAPTCHA field, not attempted.
- The Presidency Library: no archived PDF of the 16 December 1992 address; no capture of the Sarney government's
  "Substitutos Legais" subpage.
- The Chamber's copies of the DCN issues (DCN02JAN1995.pdf, DCN02JAN2011.pdf): HTTP 403 firewall pages.
- Decree No. 9.690 of 23 January 2019: its captures are 5,833-byte JavaScript challenge pages (`e97ec49a…4b5f`).
- CDX searches for a Planalto page of the interim President's speeches of 12 May 2016: nothing usable.
- The Internet Archive: timeouts, refused connections (part B could not reach the CDX at all) and "Temporarily Offline"
  responses; the availability API listed no pre-2016 capture of the Vice-Presidency's posse pages.

## Checker defects

| # | Defect | Outcome |
|---|---|---|
| A1 | Sarney's 1985 exercise stored as `attested_period` to 22 Apr 1985 | **Applied**: `br_sarney_vp_exercise_from_recited_19850315`, `attested_on` 1985-03-15, kind `vice_president_exercise_start`; no period is stored |
| A2 | Rows used the printed name "Itamar Augusto Cautiero Franco"; `name_as_printed` key on the holder | **Applied**: every row of a person carries one holder name (Itamar Franco, Marco Maciel, José Alencar, Michel Temer, Hamilton Mourão, Geraldo Alckmin); printed forms stay in the texts and notes; the holder keeps the exact key list |
| A3 | Existing CLAUDE-C01-10 sources restamped and its exact guards broken by the new rows | **Applied**: `accessed_date` 2026-09-23 and CLAUDE-C01-10's `pdf_pages_one_based` kept; the new rendered pages go in `c01_17_pdf_pages_one_based`; every affected guard re-expressed as an exact pin (see Integration notes); snapshots updated |
| A4 | The Presidency's vacancy cited on the Vice-Presidency | **Applied**: `br_presidency_vacancy_declared_19921229` is not cited on `br_vice_president`; a test pins that no Presidency vacancy claim is cited there and that no holder has an end |
| A5 | Exercise claims on two roles; the 5 October attestation rests on a later transcript | **Applied**: one convention stated in the role scope note and tested (CLAUDE-C01-10's rows stay `br_president` and are cited; new exercise rows are `br_vice_president`, both with the exercise title); Law No. 8.469 imported as the primary act of 5 October |
| A6 | BR-VP-02 omitted the cited diplomação-ceremony claim | **Applied**: `br_tse_diplomacao_ceremony_catalogued_19891230` cited under BR-VP-02, never a holder date |
| A7 | `br_vpbio_vacancy_chronology_20250715` dated by the PDF's ModDate | **Applied**: renamed `br_vpbio_vacancy_chronology_undated`; no date in any `br_vpbio_*` id comes from file metadata |
| A8 | "Summoned" overstated an announced notification | **Applied**: kind `vice_president_notification_announced`, id `br_itamar_vp_notification_announced_19921229`; the uncertainty says the notification itself is not recorded |
| B1 | Mistyped TSE item identifier for bdtse/5199 | **Applied**: the lead gives the corrected item URL (`0d83a50c-3eb2-…`) |
| B2 | Stability notes rested on edge-cache copies | **Applied**: the extracts and this report rest on the check's cache-busted origin downloads (Age 0) and name the cache window and the short-transfer retry |
| B3 | Knock-on changes to CLAUDE-C01-10 not listed | **Applied**: scope texts widened, snapshots updated and every pin re-expressed exactly (see Integration notes) |
| B4 | "Seal and signatures are not reproduced here" read as absent | **Applied**: "The arms and signatures are visible in the image and are not reproduced in the packet (rights note)." |
| B5 | Reused diaries restamped 2026-09-24 | **Applied**: CLAUDE-C01-10's date and identity kept; the 24 September re-checks are in the provenance notes and this report |
| B6 | Wrong failure reason for vicepresidencia.gov.br | **Applied**: recorded as a DNS failure (curl exit 6) |
| C1 | BR-VP-10 missed Laws No. 15.435 and 15.436 of 17 June 2026 | **Applied**: Law No. 15.436 imported as the latest attestation; Law No. 15.434's uncertainty, the observation and the method corrected; Law No. 15.435 is a lead (no capture, live page per request) |
| C2 | The stored case-file copy of the notification was not used | **Applied**: volume 48 imported (fl. 18.146 and the Termo de Juntada at fl. 18.147); the lower signature is identified; the live image is a lead |
| C3 | A new notification kind duplicating CLAUDE-C01-10's and merging issue and service | **Applied**: the mandado is `vice_president_notified_to_exercise`; the filing is `vice_president_notification_receipt_filed`; the service report is CLAUDE-C01-10's `summons_served_report`, cited |
| C4 | Rádio Senado claim: a news item, and "removal" for "afastamento" | **Resolved by removal**: the duplicate claim is withdrawn and CLAUDE-C01-10's claim is cited; the same wording in CLAUDE-C01-10's own claim is left to that packet (see Integration notes) |
| C5 | Temer's succession claim put the declaration after the termo | **Applied**: the claim records the termo only (pdf pp. 4-5 read, p. 6 signed) and says the declaration preceded it |
| C6 | Chronology footnote on the wrong page | **Applied**: locators give pdf pp. 6-7 with footnote (1) on p. 7 |
| C7 | Election claims also stated the diplomação | **Applied**: both trimmed to the election |
| C8 | Salutation locator outside its record's pages | **Applied**: the p. 13 reference is dropped |
| C9 | Holders cited exercise rows; holder names did not match rows | **Applied**: no holder cites an exercise, notification or succession claim; one holder name per person |
| C10 | Text-layer pages listed as rendered | **Applied**: only rendered pages are listed; text-layer pages are named in the method text |
| C11 | Decrees No. 11.322 and 11.324 labelled UTF-8 | **Applied**: they gain no claims here (CLAUDE-C01-10's are cited); their windows-1252 bytes are recorded in this report, and every new capture's scope note records its encoding |
| C12 | The image's stability rested on an edge-cache copy | **Resolved by removal**: the image is no longer a recorded identity; volume 48 rests on cache-busted origin downloads |
| C13 | CLAUDE-C01-10 extract scope texts contradicted by the new rows | **Applied**: each touched extract's `bounded_scope` names both packets' rows and its snapshot is updated |

Missing primary records found by the checks:

| Record | Outcome |
|---|---|
| Law No. 8.469 of 5 Oct 1992 (part A) | **Imported** (`br_planalto_lei_8469_19921005`) |
| Law No. 8.470 of 5 Oct 1992 (part A) | Lead: same day, form and signatures as Law No. 8.469 |
| Decree of 11 Dec 1991, Dnn428 (part A) | **Imported** (`br_planalto_dnn428_19911211`) |
| Decree No. 363 (part A) | Lead: fetched once, inconsistent dates; the decree of 11 Dec 1991 is preferred, as the check advised |
| Six TSE Vice-President diploma originals, 1998-2014 (part B) | Leads with their identities: each duplicates the diploma reproduced in the stored Congress issue already recorded, and the check called them optional corroboration; one is 17 MB |
| gov.br Vice-Presidency pages (part B) | Blocked by a bot check; not bypassed |
| Senate case file DEN 1/2016, vol. 48 (part C) | **Imported** (`br_senado_den1_2016_autos_vol48`) |
| Law No. 15.436 of 17 Jun 2026 (part C) | **Imported** (`br_planalto_lei15436_20260617`) |

Other changes made to fit the packet's rules rather than a numbered defect:

- Six part C claims duplicated CLAUDE-C01-10 claims on the same responses with the same event kind and person
  (`br_temer_vp_notified_jaburu_20160512`, `br_temer_vp_exercising_presidency_mpv726_20160512`,
  `br_temer_vp_exercising_dou_expedient_20160831`, `br_temer_vp_in_exercise_at_posse_opening_20160831`,
  `br_mourao_vp_exercising_d11322_20221230`, `br_mourao_vp_exercising_d11324_20221231`). They are withdrawn, and the
  CLAUDE-C01-10 claims are cited on this role instead. A new claim is added only where the person or the event kind
  differs from the existing claim (for example the Vice-President's posse in a diary whose existing claim is the
  President's, or his succession recorded in the President's termo).
- Part C's `br_senado_temer_mandado_notificacao_20160512` source and part C's Decree No. 12.958 source are not imported
  (C2; the June laws are later attestations); `br_temer_notified_to_assume_interim_20160512` now rests on volume 48.
- `br_vpbio_chronology_segment_2016_2019_20250715` is renamed `br_vpbio_chronology_grey_segment_2016_2019` (the same
  file-metadata date as A7).
- Retrospective and spans claims carry no `attested_on` key, as in CLAUDE-C01-10; every oath uncertainty says it is
  never a start or a holder date.
- The part proposals to cite President-role records on this role (the posse declarations and termos of 1992, 2016,
  2019 and 2023, the 2016 intimation order, the 1992 summons and suspension, the Presidency vacancy, Lula's 2022
  diploma) are not taken: they are `br_president` records, and the Vice-President's own claims name them in their
  uncertainty where relevant.

## Suggested next work orders

These are proposals for the integrator. They are not created in `work-orders.json`.

- `C01-Brazil-VP-002`: a contemporaneous record of the Vice-Presidency between April 1985 and March 1990 (vacant or
  held), through the Senate diary search or the DOU once reachable without a challenge.
- `C01-Brazil-VP-003`: the day each vice-presidential term ended, and any statement that the office was vacant after
  29 December 1992 or 31 August 2016.
- `C01-Brazil-VP-004`: the Vice-Presidency's own records on gov.br and their pre-cutoff captures, once reachable
  without a bot check.
- `C01-Brazil-VP-005`: acting presidents other than the Vice-President (Presidents of the Chamber and Senate), 1990-2026,
  as claims on a separate role if wanted.

## Integration notes (outside this packet's file boundary)

- **Stack, base and claim:** `claude/c01-br-17` is stacked on `claude/c01-br-10` at `73e5fd36`; the base branch was
  fetched on 24 September 2026 and had no commits beyond the stack point, so no merge was needed. Claim commit
  `774af12d` holds only the handoff. **Merge CLAUDE-C01-10 first.** This packet extends CLAUDE-C01-10's files:
  `brazil.json`, thirteen of its extracts, `test_brazil_presidents_c01_10.py` and `test_brazil_research_s10f.py`. Every
  change to them is additive (claims, rows and array entries appended; one role appended; two institution and one packet
  coverage notes inserted; snapshot values updated), so a CLAUDE-C01-10 fix made during review can be merged by keeping
  both sides and re-running the checks.
- `research-index.json` is regenerated in a **separate commit**, and it is **the only file shared with other pending
  packets** (the parallel C01 packets touch other countries' packets). New totals against `73e5fd36`: 220 sources and
  1,959 claims (previously 209 and 1,874); 28 institution observations (unchanged); Brazil has one institution, two role
  observations (previously one) and 32 entries pending mapping, and its four discovery batches stay 10, 10, 10 and 2
  members (92 batches in all). Regenerate the index after the other packets merge.
- Pinned tests re-expressed, none loosened and no assertion removed:
  - `test_brazil_presidents_c01_10.py`: "exactly one role" becomes an exact pin of the two roles with their titles and
    kinds; the "second role" mutation becomes a third-role mutation and still fails; entries and roles (32, 2); the
    source list is this packet's 48 then CLAUDE-C01-17's 11; the institution's claims and sources are CLAUDE-C01-10's
    then CLAUDE-C01-17's, exactly; `br_president`'s own claims and sources are unchanged; each CLAUDE-C01-10 source's
    claims are its own followed by exactly the CLAUDE-C01-17 claims pinned for it; every row's role is `br_president`
    except exactly the CLAUDE-C01-17 rows (`br_vice_president`); the event map is exact for CLAUDE-C01-10's 112 claims
    and the other rows are exactly CLAUDE-C01-17's; `role_observations` is 2. The 48 response identities, the page pins,
    the access date and every holder pin are untouched. The module imports the CLAUDE-C01-17 pins from
    `test_brazil_vice_presidents_c01_17.py` so that both packets' sets are pinned once.
  - `test_brazil_research_s10f.py`: counts (entries, sources, claims, roles) are now (32, 64, 231, 2) from
    (32, 53, 146, 1); the roles are exactly `br_president` and `br_vice_president`; the 11 new sources are pinned to three
    hosts and the 2026-09-24 access date (CLAUDE-C01-10's keep 2026-09-23); the source count and the order of the last 11
    are pinned; `mapping_pending` (32) and the work-order sizes are unchanged.
- `test_campaign_census` needs `spheres-sim/data`, which is absent from this sparse worktree; it was not run here, and
  the sparse checkout was not widened.
- The atlas (`tools/ui/leadership-research-review.js`) will show the two roles on `br_presidency`; every
  Vice-President holder has a start and no observation date, so each shows a "Reported interval" from its start. No UI
  code changed.
- New fields and vocabulary: `visual_review.c01_17_pdf_pages_one_based` in the thirteen shared extracts; source type
  `primary_vice_presidency_publication_archived`; claim-only event kinds `oath_recital`, `succession_statement`,
  `vice_president_succeeds_to_presidency`, `vice_president_notification_announced`,
  `vice_president_notification_receipt_filed`, `styled_incumbent_before_posse`, `predecessor_tribute_statement`,
  `retrospective_vacancy_statement` and `retrospective_chronology_depiction`, all pinned in the new test.
- For CLAUDE-C01-10 (not changed here): its claim `br_radio_senado_notifications_served_20160512` renders the source's
  "afastamento do cargo por até 180 dias" as "removal from office"; it was a suspension of up to 180 days. Correcting it
  would change that extract's checksum and CLAUDE-C01-10's pins, so it is left to that packet.
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
