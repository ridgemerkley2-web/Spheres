# PDT, PMDB/MDB and PFL/DEM national presidents 34: three party offices, 1990-2026

Packet: **CLAUDE-C01-34**. State: **ready_for_review** (not complete).
Owner: Claude. Integrator/reviewer: Codex. Branch `claude/c01-br-34`, based on `032cd6a3` (current
`codex/campaign-certification`), **not stacked** on any pending packet; claim commit `c2ff9396`. Research access:
28 September 2026 (UTC). The historical cutoff stays **7 September 2026**.

This packet adds two party roles to existing 2024 funding observations in [brazil.json](brazil.json):
`br_pdt_president` (Presidente Nacional do Partido Democrático Trabalhista, kind `party_leader`) on the `PDT`
observation (`br_tse_fefc_2024_party_02`) and `br_mdb_president` (Presidente Nacional do PMDB / MDB, kind
`party_leader`) on the `MDB` observation (`br_tse_fefc_2024_party_01`). It records the national presidency of the
PFL, renamed Democratas (DEM) in 2007 and extinguished by its fusion with the PSL into União Brasil in 2022, **as
claims only**: no role, holder or organization observation cites those claims, and the `UNIÃO` observation
(`br_tse_fefc_2024_party_27`) is unchanged. It adds 76 sources (27 PDT, 31 PMDB/MDB, 18 PFL/DEM) and 127 claims
(41, 59 and 27), thirteen holder observations (eight for the PDT, five for the MDB), two role scope notes, one
coverage note on each of the two organizations and one packet coverage note. The organizations' funding identities,
unknown lifecycles, empty game mappings and funding records are unchanged, and so are the `br_presidency` institution,
its twelve `br_president` and nine `br_vice_president` holders and CLAUDE-C01-22's eighteen `br_pt_president`
holders. It adds no organization, institution, game mapping, lifespan, portrait or avatar. The parent scope (C01, C06,
S23, WC1 and CP1) remains open.

**At most ten people.** The three chains together have about eighteen national presidents from 1990 to the cutoff,
so this packet researches ten: Leonel Brizola and Carlos Lupi (PDT, the whole period); Jader Barbalho, Michel Temer,
Romero Jucá and Baleia Rossi (PMDB/MDB from 1998); and Jorge Bornhausen, Rodrigo Maia, José Agripino and ACM Neto
(PFL/DEM from 1999, claims only). The PMDB presidents of 1990-1998 (Ulysses Guimarães, Orestes Quércia, Luiz Henrique
da Silveira, Paes de Andrade) and the PFL presidents before 1999 (including Hugo Napoleão) are outside the packet and
are proposed as next work. Other people appear only as they are printed in claims: acting or interim service (André
Figueiredo and Vieira da Cunha for the PDT; Iris de Araújo and Valdir Raupp for the PMDB), Maguito Vilela in the
PMDB's retrospective list, and offices of União Brasil.

The research was done in three chains. Two helpers each assembled a dossier for one chain (PDT; PMDB/MDB), and the
PFL/DEM chain and the TSE registry records were researched for this packet directly. Every recorded response was then
downloaded again for this packet at least twice from the recorded URL, the first and last at least 30 minutes apart,
and every single-quoted passage in a claim text was checked by script against the decoded bytes (PDF text via
pdftotext); the one exception is the image table in the MDB directorate's minutes of 5 October 2023, read visually.
Two independent checks then read every claim and holder against the bodies; all their must-fix defects and most of
their suggestions are applied (see [Checker defects](#checker-defects)).

## Outcome

| ID | Question | Decision |
|---|---|---|
| PDT-PRES-01 | The PDT's President when the period opens, and Leonel Brizola's presidency to his death | **Accepted in part:** no contemporaneous day-dated PDT record of 1990-1996 was found, and the party's 2019 history ties Brizola's 1992 'recondução' to the death of Doutel de Andrade (7 January 1991), so who presided on 1 January 1990 is open; Brizola is observed on 11 April 1997, 26 August 1999 and 2 June 2004, and the party's home page of 21 June 2004 reports that its national President died that evening (his stated end); his Tijolaço column printed '06.05.2004' is undated, its own text placing it in early June 2004 |
| PDT-PRES-02 | The succession of 2004 | **Accepted in part:** Carlos Lupi is observed on 6 July 2004 with no start: the undated 2004 profile says he assumed 'automaticamente' on Brizola's death, the party's 2019 history says 'interinamente' at the executive's meeting of 28 June 2004 (never archived), and the 2017 'Líderes Históricos' page (a lead) says 21 June 2004; the roster captured 22 July 2004 lists him as President with no first vice-president, and the item of 21 March 2005 has him 'confirmado na presidência' |
| PDT-PRES-03 | Lupi's presidency, 2005-2022: conventions, re-elections and the leave of 2008-2009 | **Accepted in part:** re-elections of 21 March 2005, 9 March 2007 (retrospective), 6 March 2009 (while on leave, with Vieira da Cunha acting), 18 March 2017 and 18 March 2019 and two closed SGIP organs are claims; Lupi is observed on 9 February 2007 and 21 December 2021; no contemporaneous party record of 2011-2016 and no result of the convention of 21 January 2022 were found |
| PDT-PRES-04 | 2023-2026: Lupi's ministerial leave, André Figueiredo's acting service, the return and an attestation before the cutoff | **Accepted in part:** the leave and Figueiredo's acting service (2 February 2023) and the return decided on 20 May 2025 are claims; Lupi is observed on 21 May 2025 and 4 September 2026 |
| MDB-PRES-01 | Jader Barbalho's presidency (1998-2001) and its end | **Accepted in part:** his election on 15 September 1998 is printed on an undated party page and his mandate to 15 May 2001 only in the retrospective list; no day-dated party attestation was found, so he has no holder, and how his presidency ended is open; the same list prints Maguito Vilela's service of 15 May-9 September 2001 as a 'Mandato', not a 'Mandato Interino' (outside the ten people researched) |
| MDB-PRES-02 | Michel Temer, 2001-2009: election, attestation and the leave of 2009 | **Accepted in part:** observed on 2 July 2003 (an official note signed 'Michel Temer Presidente do PMDB'); the start of his mandate on 9 September 2001 is only in the retrospective list; the leave of 10 March 2009 with Iris de Araújo's interim service is a claim |
| MDB-PRES-03 | Temer, 2010-2016: leaves, acting service, re-elections and the conflicting records | **Accepted in part:** the posse of the executive on 10 March 2010, printed on an undated roster, is the posse of an executive and not taken as his start; re-elections of 2 March 2013 and 12 March 2016 are claims; a party item of 14 May 2014 styles Valdir Raupp national President against the party's retrospective list; Temer is observed on 29 March 2016 |
| MDB-PRES-04 | 2016-2019: Temer's leave, Romero Jucá's service and the renaming PMDB to MDB | **Accepted in part:** Temer took leave on 5 April 2016 and Jucá's service is recorded as acting (the party's roster, its retrospective list and the TSE registry say 'em exercício' or 'interina'), so his unqualified stylings, including the signed edital of 23 November 2017, are claims; the convention of 19 December 2017 voted the renaming, filed with the TSE on 31 January 2018 and approved by the TSE on 15 May 2018 |
| MDB-PRES-05 | Baleia Rossi, 2019 to the cutoff | **Accepted in part:** elected on 6 October 2019 (no posse day printed), extended on 23 February 2021, re-elected on 5 October 2023 and extended to 15 May 2027 on 16 July 2025 (claims); observed on 17 October 2019, 16 July 2025 and 7 June 2026 |
| PFL-PRES-01 | The PFL's President before the renaming: Jorge Bornhausen | **Claims only:** the executive elected and invested at the convention of 7 May 1999 is headed by Bornhausen; he is styled President on 7 March 2003 and signs the convocation of 13 March 2007 |
| PFL-PRES-02 | The renaming of the PFL as Democratas and Rodrigo Maia's presidency | **Claims only:** the convention called for 28 March 2007 was to adopt a new name; an undated PFL page announces the transformation into DEMOCRATAS; on 29 March 2007 the 'Comissão Provisória Nacional do Democratas' acts under a resolution signed by Rodrigo Maia as 'Presidente Nacional do Democratas', who is still styled President on 15 February and 14 March 2011 |
| PFL-PRES-03 | José Agripino's presidency | **Claims only:** the single slate agreed on 16 February 2011 and the convention announced for 15 March 2011; the election by acclamation is reported in an item of 16 March 2011; Agripino signs as President on 24 March 2011; the SGIP registry period of 2015-2018 |
| PFL-PRES-04 | ACM Neto's presidency and the fusion into União Brasil | **Claims only:** the executive elected at the convention of 8 March 2018 is headed by ACM Neto; he is styled President on 5 October 2021 and 12 January 2022; the fusion convention of 6 October 2021 and the TSE's registration of União Brasil on 8 February 2022 are organization claims, and the registry gives União Brasil a CNPJ other than the Democratas' |

The resulting holder observations of `br_pdt_president`, in date order:

| Holder | `attested_on` | `from` | `until` | Basis |
|---|---|---|---|---|
| Leonel Brizola | 1997-04-11 | null | null | The PDT home page updated that day says 'o presidente nacional Leonel Brizola acompanhou todas as intervenções'. |
| Leonel Brizola | 1999-08-26 | null | null | A PDT press item of that day says 'O presidente do PDT, Leonel Brizola, encerrou seu discurso'. |
| Leonel Brizola | 2004-06-02 | null | 2004-06-21 | A PDT item of that day says 'Decisão se deu em reunião das bancadas com o Presidente Nacional do Partido, Leonel Brizola (Brasilia - 02/06/04)'. Until 21 June 2004: the PDT home page updated at 22h15m that day reports that 'O presidente nacional do PDT' Leonel de Moura Brizola 'faleceu ás 21h29m'. |
| Carlos Lupi | 2004-07-06 | null | null | The PDT home page updated that day heads 'Perfil de Carlos Lupi, presidente nacional do PDT'. |
| Carlos Lupi | 2007-02-09 | null | null | Resolução nº 001/07 of that day closes 'CARLOS LUPI Presidente Nacional'. |
| Carlos Lupi | 2021-12-21 | null | null | A PDT item of that day says that 'A resolução normativa 003/2021 é assinada pelo presidente nacional da legenda, Carlos Lupi.' |
| Carlos Lupi | 2025-05-21 | null | null | The PDT item of that day, reporting his return decided 'nesta terça-feira (20)', attributes his statement to 'o presidente nacional do PDT'. |
| Carlos Lupi | 2026-09-04 | null | null | A PDT item of that day says 'O presidente nacional do PDT, Carlos Lupi, reforça o papel estratégico do partido'. |

The resulting holder observations of `br_mdb_president`, in date order:

| Holder | `attested_on` | `from` | `until` | Basis |
|---|---|---|---|---|
| Michel Temer | 2003-07-02 | null | null | An official note of the national executive headed 'Brasília, 02 de julho de 2003' is signed 'Michel Temer Presidente do PMDB'. |
| Michel Temer | 2016-03-29 | null | null | A PMDB item of that day styles 'O vice-presidente da República e presidente nacional do PMDB, Michel Temer'. |
| Baleia Rossi | 2019-10-17 | null | null | An MDB item of that day says 'O novo presidente do MDB, deputado Baleia Rossi (SP) reuniu ontem em Brasília' the new executive's officers. |
| Baleia Rossi | 2025-07-16 | null | null | The signed minutes of the national executive record that 'o Presidente Nacional do Partido e Deputado Federal Baleia Rossi (MDB-SP) iniciou a reunião'. |
| Baleia Rossi | 2026-06-07 | null | null | A decision reproduced by the MDB's item of 8 June 2026 closes 'Brasília, 7 de junho de 2026. Baleia Rossi Presidente Nacional do MDB'. |

### How a start and an end are decided

The packet applies the rule of the integrated Brazil packets (CLAUDE-C01-10, -17 and -22) to two party offices. A
holder has `from` only where a source states the day the office was assumed or took effect, and `until` only where a
source states the day it ended. No record found here states a day of assumption unambiguously, so no holder has a
start. One record states a day of ending: the PDT's home page updated at 22h15m on 21 June 2004 reports that 'O
presidente nacional do PDT' Leonel de Moura Brizola died that evening; the packet takes that same-day statement of a
death in office as his `until` (kind `death_in_office_stated`). It is a death, not a successor's start; if Codex does
not accept a death as a stated end, the fallback is a death claim only and no `until`. Every other holder is dated by
`attested_on` and, as in the PT and ANC packets, cites only in-office attestations made on its own day; the test pins
that every cited claim carries exactly the holder's date. Days of conventions, elections, re-elections, extensions and
declarations are never holder dates.

Everything else is a claim that never feeds a holder: conventions and executive elections, re-elections, mandate
extensions, the posse of an executive printed on an undated roster, leaves and returns, stylings on the day of an
election, in-office stylings on other days of a term already observed (continuation), stylings dated only by a month,
an undated page or a year not printed, acting and interim service, SGIP registry exercise periods, retrospective lists,
histories and profiles, renamings and the fusion. No end is inferred from a successor's election, first styling or
registry start.

Five kinds of statement were proposed or could be read as boundaries and are refused:

- **Assumption days that are not printed or are retrospective.** Lupi's 2004 profile gives the mechanism
  ('automaticamente ... com a morte de Leonel Brizola') without a day; the party's 2019 history ('interinamente ... em
  28 de junho de 2004') and its 2017 'Líderes Históricos' page ('em 21 de junho de 2004', a lead) are retrospective.
- **The posse of an executive.** The PMDB roster captured in May 2011 prints 'Data da posse: 10 DE MARÇO DE 2010'
  over 'Presidente: MICHEL TEMER (SP)'; it is the posse of the executive he heads, printed on an undated page, not a
  stated assumption of the office by him, and the retrospective list already has him back in office from 27 January
  2010, so 10 March 2010 is a claim (Codex may accept it as the start of a new term). The same page's 'Licenciado'
  status is a separate undated claim.
- **Returns from leave.** The PDT executive's decision of Tuesday 20 May 2025 that Lupi 'está de volta à presidência'
  states the decision's day, not a day of resumption; he is observed on 21 May 2025.
- **Registry periods.** Most SGIP exercise periods follow their organ's registered term (for example Lupi from
  18/03/2017 and Baleia Rossi from 06/10/2019, both convention days); Temer's period as 'PRESIDENTE LICENCIADO'
  (16/07/2014-05/04/2016) is his own and ends on the day the party reports his leave, and Jucá's combined period runs
  from the convention of 12 March 2016. All are claims only.
- **Leaves and renamings.** Temer's leaves (10 March 2009, 5 April 2016), Lupi's leaves (2008, 2 February 2023), the
  renaming of 19 December 2017 and its approval of 15 May 2018 end nothing.

### Acting and interim service

Acting (`em exercício`) and interim (`interino`) service is recorded only as claims, with the role title 'Presidente
Nacional do ... em exercício ou interino', and never makes, splits or ends a holder: Vieira da Cunha for the PDT after
the convention of 6 March 2009 (and, by the party's 2019 history, from 12 March 2008); André Figueiredo from 2 February
2023; Iris de Araújo for the PMDB from 10 March 2009; Valdir Raupp in the roster captured in May 2011 and the
retrospective list (15 June 2010 to 11 March 2014); and Romero Jucá from 5 April 2016. Jucá is the hard case. The
party printed him 'presidente nacional do PMDB, em exercício' on 7 April 2016, its roster captured in January 2019
lists '1º Vice-Presidente: ROMERO JUCÁ (em exercício)' under 'Presidente: MICHEL TEMER (licenciado)', its retrospective
list says 'Assume a direção interina do Partido em 05/04/2016', and the TSE registry records 'PRESIDENTE EM EXERCÍCIO |
PRIMEIRO VICE-PRESIDENTE' from 12/03/2016 to 06/10/2019. The party also printed him without a qualifier, most strongly
in the convocation edital signed 'ROMERO JUCÁ Presidente Nacional do PMDB' on 23 November 2017, and on 19 December 2017,
22 February 2018 and 17 June 2019; no election of Jucá as President was found. Following the Genoino precedent of
CLAUDE-C01-22, those stylings are claims of kind `styled_president_during_acting_period` and Jucá has no holder (Codex
to rule). A party item of 14 May 2014 styling Valdir Raupp 'presidente nacional da legenda' without a qualifier is kept
as a conflicting record, never a holder.

### Party office, state office and the other party offices

`br_pdt_president` and `br_mdb_president` sit on the PDT and MDB funding observations; the Presidency and
Vice-Presidency of the Republic stay in `br_presidency`, and the PT office stays on the PT observation. No claim or
source of either new role feeds `br_presidency` or `br_pt_president`, and none of theirs feeds the new roles: every claim
and source id of the PDT role begins `br_pdt_`, of the MDB role `br_mdb_`, and of the PFL/DEM chain `br_pfl_`. The test
pins the presidency, vice-presidency and PT holders unchanged and rejects cross-role mutations. State offices named in
the same sentence as the party office are flagged in the claims and never start, end or split a holder: Temer as Vice-
President of the Republic (29 March and 5 April 2016), Lupi as Minister (2009, 2023, 2025), and the senators, deputies,
governors and mayors styled by their seats. The existing `test_brazil_research_s10f.py` guard that only the PT
observation has a role is re-expressed as an exact pin of the three party roles.

### The PFL/DEM chain: claims only, and the União Brasil question

The scope allows a role for DEM/União Brasil only if a source ties that label to the PFL line. Sources do tie them: an
undated PFL page announces the PFL's 'transformação em DEMOCRATAS', the Democratas act under that name from 29 March
2007, and the TSE registered on 8 February 2022 'o pedido de registro do estatuto e do programa partidário do União
Brasil', an 'agremiação política resultante da fusão do Democratas (DEM) com o Partido Social Liberal (PSL)'. The tie
is a fusion that created a new party with its own registration, number (44) and registry identity: the SGIP records of
the DEM organs carry CNPJ 01.633.510/0001-69, and the first União Brasil organ carries CNPJ 44.551.496/0001-67. Placing
the PFL/DEM presidency as a role on the `UNIÃO` observation would merge two organizations' identities, which the scope
forbids, so the packet records the PFL/DEM chain only as claims (sources `br_pfl_*`, rows with `observation_id` and
`role_id` null) and leaves `UNIÃO` unchanged. The PMDB/MDB case differs: the TSE approved on 15 May 2018 'a mudança do
nome e da sigla' of the same party, and the SGIP records the PMDB-era organ of 2013-2019 and the MDB organ of 2019-2023
under the same CNPJ (00.676.213/0001-38), so the PMDB presidents sit on the MDB role, tied by the recorded change of
name and never by a name match. The registry's party record attached to the DEM organs names the party 'DEMOCRATAS
(extinto por fusão com PSL, originando o UNIÃO)', situation 'Unido a outro partido por fusão'; the packet records it as
an organization claim without a date. The two União Brasil sources and the UNIÃO registry claim carry the `br_pfl_`
prefix and review observation PFL-PRES-04 only as a filing convention for the fusion question; the prefix asserts no
lineage and no identity.

### Date ledger

Each row is a separate dated fact with its own claim; two facts on one day stay two claims. Holder fields are marked
where a claim dates a holder. Claims with no structured date are listed by observation at the end.

| Date | Events | Claims and holder fields |
|---|---|---|
| 11 Apr 1997 | Brizola styled President in office | `br_pdt_brizola_presidente_nacional_home_19970411`; Brizola `attested_on` |
| 15 Sep 1998 | Jader Barbalho elected | `br_mdb_jader_elected_19980915` |
| 7 May 1999 | executive elected and invested at the convention (Bornhausen President) (PFL/DEM, claims only) | `br_pfl_cen_elected_and_invested_bornhausen_presidente_19990507` |
| 26 Aug 1999 | Brizola styled President in office | `br_pdt_brizola_styled_presidente_do_pdt_19990826`; Brizola `attested_on` |
| 7 Mar 2003 | Bornhausen styled President in office (PFL/DEM, claims only) | `br_pfl_bornhausen_styled_presidente_do_pfl_20030307` |
| 2 Jul 2003 | Temer styled President in office | `br_mdb_temer_signs_official_note_as_presidente_20030702`; Temer `attested_on` |
| 2 Jun 2004 | Brizola styled President in office | `br_pdt_brizola_meets_caucuses_as_presidente_nacional_20040602`; Brizola `attested_on` |
| 21 Jun 2004 | Brizola dies in office (stated the same day); Brizola's death reported the next day | `br_pdt_brizola_dies_as_presidente_nacional_20040621`, `br_pdt_curitiba_reports_death_of_presidente_nacional`; Brizola `until` |
| 6 Jul 2004 | Lupi styled President in office | `br_pdt_lupi_presidente_nacional_home_20040706`; Lupi `attested_on` |
| 28 Feb 2005 | Lupi styled President (continuation); convention called (prospective) | `br_pdt_lupi_signs_convocation_as_presidente_nacional_20050228`, `br_pdt_convention_convoked_for_20050321_20050228` |
| 21 Mar 2005 | convention held; Lupi styled President on the day of his re-election; Lupi re-elected | `br_pdt_convention_held_20050321`, `br_pdt_lupi_styled_atual_presidente_on_convention_day_20050321`, `br_pdt_lupi_reelected_convention_20050321` |
| 9 Feb 2007 | Lupi styled President in office | `br_pdt_lupi_signs_resolution_001_07_as_presidente_nacional_20070209`; Lupi `attested_on` |
| 13 Mar 2007 | Bornhausen styled President in office (PFL/DEM, claims only); convention called (prospective) | `br_pfl_bornhausen_signs_convocation_as_presidente_20070313`, `br_pfl_convention_called_for_20070328_prospective_20070313` |
| 29 Mar 2007 | party acts as the Democratas; Rodrigo Maia styled President in office (PFL/DEM, claims only) | `br_pfl_resolution_006_issued_in_name_of_democratas_20070329`, `br_pfl_maia_signs_resolution_006_as_presidente_nacional_20070329` |
| 6 Mar 2009 | Lupi re-elected while on leave; Vieira da Cunha acting (em exercício or interim) | `br_pdt_lupi_reelected_while_licenciado_20090306`, `br_pdt_vieira_da_cunha_keeps_command_20090306` |
| 10 Mar 2009 | Iris de Araújo interim; Temer on leave | `br_mdb_iris_assumes_interim_presidency_20090310`, `br_mdb_temer_leave_20090310` |
| 20 Jan 2010 | convention called (prospective) | `br_mdb_convention_set_for_20100206_prospective_20100120` |
| 21 Jan 2010 | Temer signs as President on leave | `br_mdb_temer_signs_note_as_presidente_licenciado_20100121` |
| 10 Mar 2010 | posse of the executive printed on an undated roster (Temer "Licenciado") | `br_mdb_executive_posse_20100310` |
| 15 Feb 2011 | Rodrigo Maia styled President in office (PFL/DEM, claims only) | `br_pfl_maia_signs_resolution_112_as_presidente_nacional_20110215` |
| 16 Feb 2011 | single-slate agreement announced | `br_pfl_single_slate_agreement_announced_20110216` |
| 14 Mar 2011 | Rodrigo Maia styled President in office (PFL/DEM, claims only); convention announced (prospective) (PFL/DEM, claims only) | `br_pfl_maia_styled_atual_presidente_20110314`, `br_pfl_convention_to_formalize_agripino_prospective_20110314` |
| 24 Mar 2011 | José Agripino styled President in office (PFL/DEM, claims only) | `br_pfl_agripino_signs_resolution_113_as_presidente_nacional_20110324` |
| 2 Mar 2013 | convention held; Temer re-elected | `br_mdb_convention_held_20130302`, `br_mdb_temer_reelected_20130302` |
| 14 May 2014 | Valdir Raupp styled national President (conflicting record) | `br_mdb_raupp_styled_presidente_nacional_20140514` |
| 12 Mar 2016 | convention held; Temer re-elected | `br_mdb_convention_held_20160312`, `br_mdb_temer_reelected_20160312` |
| 29 Mar 2016 | Temer styled President in office | `br_mdb_temer_styled_presidente_nacional_20160329`; Temer `attested_on` |
| 5 Apr 2016 | Temer on leave; Jucá acting (em exercício or interim) | `br_mdb_temer_leave_20160405`, `br_mdb_juca_assumes_in_temer_place_20160405` |
| 7 Apr 2016 | Jucá acting (em exercício or interim) | `br_mdb_juca_styled_presidente_em_exercicio_20160407` |
| 18 Mar 2017 | Lupi re-elected | `br_pdt_lupi_reconducted_xxiii_convention_20170318` |
| 23 Nov 2017 | Jucá styled President during the acting period; convention called (prospective) | `br_mdb_juca_signs_edital_as_presidente_nacional_20171123`, `br_mdb_convention_called_renaming_prospective_20171123` |
| 19 Dec 2017 | convention held; convention votes the renaming PMDB to MDB; Jucá styled President during the acting period | `br_mdb_extraordinary_convention_held_20171219`, `br_mdb_convention_votes_renaming_20171219`, `br_mdb_juca_styled_presidente_nacional_20171219` |
| 31 Jan 2018 | renaming filed with the TSE | `br_mdb_renaming_communicated_to_tse_20180131` |
| 21 Feb 2018 | mandates extended | `br_mdb_executive_extends_mandates_20180221` |
| 22 Feb 2018 | Jucá styled President during the acting period | `br_mdb_juca_styled_presidente_nacional_20180222` |
| 8 Mar 2018 | executive elected at the convention (ACM Neto President) (PFL/DEM, claims only) | `br_pfl_cen_refundacao_elected_convention_acm_neto_20180308` |
| 15 May 2018 | TSE approves the renaming | `br_mdb_tse_approves_name_change_20180515` |
| 18 Mar 2019 | Lupi re-elected | `br_pdt_lupi_reconducted_xxv_convention_20190318` |
| 17 Jun 2019 | Jucá styled President during the acting period | `br_mdb_juca_styled_presidente_do_mdb_20190617` |
| 6 Oct 2019 | Baleia Rossi elected | `br_mdb_convention_elects_executive_baleia_20191006` |
| 17 Oct 2019 | Baleia Rossi styled President in office | `br_mdb_baleia_styled_novo_presidente_20191017`; Baleia Rossi `attested_on` |
| 23 Feb 2021 | mandates extended | `br_mdb_executive_extends_baleia_mandate_20210223` |
| 5 Oct 2021 | convention announced (prospective); ACM Neto styled President in office (PFL/DEM, claims only) | `br_pfl_fusion_convention_scheduled_prospective_20211005`, `br_pfl_acm_neto_styled_presidente_do_dem_20211005` |
| 6 Oct 2021 | fusion approved by the DEM and PSL conventions (month only) | `br_pfl_tse_states_fusion_approved_joint_convention_20211006` |
| 21 Dec 2021 | Lupi styled President in office; convention called (prospective) | `br_pdt_lupi_signs_normative_resolution_003_2021_20211221`, `br_pdt_convention_convoked_for_20220121_20211221`; Lupi `attested_on` |
| 10 Jan 2022 | Lupi styled President (continuation) | `br_pdt_lupi_signs_communique_as_presidente_nacional_20220110` |
| 12 Jan 2022 | ACM Neto styled President in office (PFL/DEM, claims only) | `br_pfl_acm_neto_styled_presidente_nacional_20220112` |
| 21 Jan 2022 | Lupi's convention speech published | `br_pdt_convention_speech_published_20220121` |
| 8 Feb 2022 | TSE session on União Brasil scheduled (prospective); offices of União Brasil named (another organization); registry identity of União Brasil (separate CNPJ); TSE registers União Brasil, formed by the fusion | `br_pfl_tse_session_on_uniao_scheduled_prospective_20220208`, `br_pfl_acm_neto_styled_uniao_secretary_general_20220208`, `br_pfl_tse_sgip_uniao_organ_registry_identity_20220208`, `br_pfl_tse_approves_uniao_registration_fusion_20220208` |
| 2 Feb 2023 | Lupi on leave; André Figueiredo acting (em exercício or interim) | `br_pdt_lupi_takes_leave_of_presidency_20230202`, `br_pdt_figueiredo_announced_presidente_em_exercicio_20230202` |
| 5 Oct 2023 | convention held; Baleia Rossi re-elected; Baleia Rossi elected | `br_mdb_convention_held_20231005`, `br_mdb_baleia_reelected_20231005`, `br_mdb_directorate_elects_executive_by_acclamation_20231005` |
| 20 May 2025 | Lupi returns from leave (executive decision) | `br_pdt_lupi_returns_to_presidency_20250520` |
| 21 May 2025 | Lupi styled President in office | `br_pdt_lupi_styled_presidente_nacional_20250521`; Lupi `attested_on` |
| 16 Jul 2025 | Baleia Rossi styled President in office; mandates extended | `br_mdb_baleia_opens_executive_meeting_as_presidente_nacional_20250716`, `br_mdb_executive_and_directorate_extended_to_20270515_20250716`; Baleia Rossi `attested_on` |
| 30 Mar 2026 | Baleia Rossi styled President (continuation) | `br_mdb_baleia_signs_resolution_1_2026_20260330` |
| 7 Jun 2026 | Baleia Rossi styled President in office | `br_mdb_baleia_signs_decision_as_presidente_nacional_20260607`; Baleia Rossi `attested_on` |
| 8 Jun 2026 | Baleia Rossi styled President (continuation) | `br_mdb_baleia_styled_presidente_nacional_20260608` |
| 15 Jun 2026 | Baleia Rossi styled President (continuation) | `br_mdb_baleia_styled_presidente_20260615` |
| 19 Aug 2026 | Lupi styled President (continuation) | `br_pdt_lupi_styled_presidente_nacional_20260819` |
| 4 Sep 2026 | Lupi styled President in office | `br_pdt_lupi_styled_presidente_nacional_20260904`; Lupi `attested_on` |
| undated (PDT-PRES-01) | undated pages, month or year dates, retrospective lists and registry periods (no structured date) | `br_pdt_brizola_styled_presidente_nacional_199611`, `br_pdt_brizola_signs_tijolaco_35_as_presidente_nacional`, `br_pdt_roster_brizola_presidente_lupi_first_vice_capt20040609`, `br_pdt_history_v_convention_reconducts_brizola_1992`, `br_pdt_history_ix_convention_reconducts_brizola_1999` |
| undated (PDT-PRES-02) | undated pages, month or year dates, retrospective lists and registry periods (no structured date) | `br_pdt_home_links_executive_note_after_succession_20040706`, `br_pdt_roster_lupi_presidente_capt20040722`, `br_pdt_lupi_assumes_automatically_on_brizola_death`, `br_pdt_history_lupi_assumed_interim_executive_meeting_2004` |
| undated (PDT-PRES-03) | undated pages, month or year dates, retrospective lists and registry periods (no structured date) | `br_pdt_convention_convoked_for_20070309`, `br_pdt_tse_sgip_lupi_registered_exercise_20170318_20190318`, `br_pdt_history_xvi_convention_reelects_lupi_2007`, `br_pdt_history_lupi_leave_to_vieira_da_cunha_2008`, `br_pdt_tse_sgip_lupi_registered_exercise_20190318_20220121` |
| undated (PDT-PRES-04) | undated pages, month or year dates, retrospective lists and registry periods (no structured date) | `br_pdt_lupi_leave_2023_recalled` |
| undated (MDB-PRES-01) | undated pages, month or year dates, retrospective lists and registry periods (no structured date) | `br_mdb_jader_styled_presidente_nacional_undated`, `br_mdb_list_jader_mandate_1998_2001`, `br_mdb_list_maguito_mandate_2001` |
| undated (MDB-PRES-02) | undated pages, month or year dates, retrospective lists and registry periods (no structured date) | `br_mdb_temer_styled_presidente_do_pmdb_undated_2001`, `br_mdb_roster_temer_presidente_licenciado_capt2009`, `br_mdb_iris_presidente_em_exercicio_roster_capt2009`, `br_mdb_list_temer_mandate_2001_leave_2009`, `br_mdb_list_iris_interim_2009_2010` |
| undated (MDB-PRES-03) | undated pages, month or year dates, retrospective lists and registry periods (no structured date) | `br_mdb_roster_temer_presidente_licenciado_capt2011`, `br_mdb_raupp_em_exercicio_roster_capt2011`, `br_mdb_tse_sgip_temer_registered_presidente_licenciado_20140716_20160405`, `br_mdb_list_temer_resumes_2010_leave_2010`, `br_mdb_list_raupp_interim_2010_2014`, `br_mdb_list_temer_assumes_2014` |
| undated (MDB-PRES-04) | undated pages, month or year dates, retrospective lists and registry periods (no structured date) | `br_mdb_tse_sgip_juca_registered_presidente_em_exercicio_20160312_20191006`, `br_mdb_roster_temer_presidente_licenciado_capt201901`, `br_mdb_roster_juca_first_vice_em_exercicio_capt201901`, `br_mdb_roster_temer_elected_six_times`, `br_mdb_list_juca_interim_2016` |
| undated (MDB-PRES-05) | undated pages, month or year dates, retrospective lists and registry periods (no structured date) | `br_mdb_tse_sgip_baleia_registered_exercise_20191006_20231006`, `br_mdb_baleia_reconduction_congratulated_20250716` |
| undated (PFL-PRES-02) | undated pages, month or year dates, retrospective lists and registry periods (no structured date) | `br_pfl_site_states_transformation_into_democratas` |
| undated (PFL-PRES-03) | undated pages, month or year dates, retrospective lists and registry periods (no structured date) | `br_pfl_agripino_elected_by_acclamation_reported`, `br_pfl_tse_sgip_agripino_registered_exercise_20151203_20180308` |
| undated (PFL-PRES-04) | undated pages, month or year dates, retrospective lists and registry periods (no structured date) | `br_pfl_tse_sgip_acm_neto_registered_exercise_20180308_20190530`, `br_pfl_tse_sgip_acm_neto_registered_exercise_20190530_20220208`, `br_pfl_tse_sgip_dem_party_record_extinct_by_fusion`, `br_pfl_fusion_approved_joint_convention_october_2021` |

Date conventions follow CLAUDE-C01-10, -17 and -22: `attested_on` is the day of the observed event as the source dates
it; a day printed without a month or year takes them from the item that prints it, and an item that prints no date of
its own gives no year (the PMDB item on the vote of 'terça-feira (04/12)'); a page's archive capture day and its
Last-Modified header are never its date, except where an undated page is plainly generated for its day by a printed
update line (the PDT home pages of 11 April 1997, 21 June 2004 and 6 July 2004); a month, an undated roster, a
retrospective statement and a registry period carry no structured date; no clock time is stored.

## Observations

### PDT-PRES-01 — The President when the period opens, and Leonel Brizola's presidency to his death

Evidence: the PDT's archived site begins in December 1996; no party record of 1990-1996 names the President. A youth
paper's interview reproduced on the site calls 'o ex-governador e presidente nacional do partido, Leonel Brizola' in
November 1996, a month-only date (`br_pdt_brizola_styled_presidente_nacional_199611`). The home page updated '11 de
abril de 97 - 6a. feira - 15:45 hs.' says 'o presidente nacional Leonel Brizola acompanhou todas as intervenções' of a
state party meeting (`br_pdt_brizola_presidente_nacional_home_19970411`); a press item of 26 August 1999 says 'O
presidente do PDT, Leonel Brizola, encerrou seu discurso' (`br_pdt_brizola_styled_presidente_do_pdt_19990826`); his
Tijolaço column, printed '(06.05.2004)', is signed 'Leonel Brizola Presidente Nacional do PDT', but its own text is
written hours before the Chamber's vote on the R$ 260 minimum wage and announces the Encontro Nacional for 'amanhã e
sábado', which places it in early June 2004, so it is undated (`br_pdt_brizola_signs_tijolaco_35_as_presidente_nacional`);
an item of 2 June 2004 reports a meeting of the caucuses 'na tarde de hoje' 'com o Presidente Nacional do Partido, Leonel
Brizola' (`br_pdt_brizola_meets_caucuses_as_presidente_nacional_20040602`); and
the undated roster captured 9 June 2004 lists him President and Carlos Lupi first vice-president
(`br_pdt_roster_brizola_presidente_lupi_first_vice_capt20040609`). The home page updated at 22h15m on 21 June 2004
reports that 'O presidente nacional do PDT' died at 21h29m (`br_pdt_brizola_dies_as_presidente_nacional_20040621`); an
item from Curitiba of 22 June 2004 reports the death 'às 21:20 de ontem'
(`br_pdt_curitiba_reports_death_of_presidente_nacional`). The party's 2019 history says the V Convention of 26 April
1992 'reconduziria Leonel Brizola à Presidência Nacional do PDT, momentos após o falecimento, em 7 de janeiro de 1991,
de Doutel de Andrade', and that the IX Convention of 19 April 1999 re-elected him
(`br_pdt_history_v_convention_reconducts_brizola_1992`, `br_pdt_history_ix_convention_reconducts_brizola_1999`).

Decision: accepted in part. Three holder observations (11 April 1997, 26 August 1999 and 2 June 2004, one per term
found), the last ending on 21 June 2004 on the party's same-day statement of his death in office. No start.

Limits: who held or exercised the office on 1 January 1990 is open; the 2019 history names Doutel de Andrade in
connection with the 1992 recondução without saying he presided. The next day's report prints another time of death.

### PDT-PRES-02 — The succession of 2004

Evidence: the home page updated '06/07/04, 10h17m' heads 'Perfil de Carlos Lupi, presidente nacional do PDT'
(`br_pdt_lupi_presidente_nacional_home_20040706`) and links a 'Nota da Executiva Nacional e das bancadas no Congresso
Nacional' (partido/pdt280604.asp) that was never archived (`br_pdt_home_links_executive_note_after_succession_20040706`).
The undated profile says he 'assume automaticamente a Presidência Nacional do PDT com a morte de Leonel Brizola' as the
first vice-president elected in March 2003 (`br_pdt_lupi_assumes_automatically_on_brizola_death`); the roster captured
22 July 2004 reads 'Leonel Brizola in memoria Presidente Carlos Lupi' (`br_pdt_roster_lupi_presidente_capt20040722`);
the edital of 28 February 2005 is signed 'CARLOS LUPI Presidente Nacional - PDT'
(`br_pdt_lupi_signs_convocation_as_presidente_nacional_20050228`, a continuation). The 2019 history says 'Carlos Lupi
assumia interinamente a Presidência Nacional do PDT na reunião da Executiva Nacional, em 28 de junho de 2004' and that
he was elected on 21 March 2005 (`br_pdt_history_lupi_assumed_interim_executive_meeting_2004`).

Decision: accepted in part. Lupi is a holder observed on 6 July 2004, the first day-dated record, which styles him
national President without 'interino'. No start: the profile prints no day, and the two days on offer (28 June and 21
June 2004) are retrospective. Brizola's death is never his start.

Limits: whether his service from June 2004 to March 2005 was interim is open (the contemporaneous profile says he
assumed automatically; the 2019 history says interim). Contemporaneous records weigh against the interim reading: the
roster captured 22 July 2004 lists him as President with no first vice-president (the one captured 9 June 2004 had
him first vice-president), and the item of 21 March 2005 calls him 'o atual presidente nacional', 'confirmado na presidência'. If
Codex rules the service interim, this observation becomes a claim.

### PDT-PRES-03 — Lupi's presidency, 2005-2022

Evidence: the convention called for 21 March 2005 (`br_pdt_convention_convoked_for_20050321_20050228`) met that day
and, by the same-day report, re-elected Lupi, 'o atual presidente nacional', with an executive 'com mandato até março
de 2007' (`br_pdt_convention_held_20050321`, `br_pdt_lupi_styled_atual_presidente_on_convention_day_20050321`,
`br_pdt_lupi_reelected_convention_20050321`). Resolution 001/07 of 9 February 2007 closes 'CARLOS LUPI Presidente
Nacional' (`br_pdt_lupi_signs_resolution_001_07_as_presidente_nacional_20070209`) and the item calls the convention of 9
March 2007 (`br_pdt_convention_convoked_for_20070309`), which the 2019 history says re-elected him
(`br_pdt_history_xvi_convention_reelects_lupi_2007`). The 2019 history dates his leave to 12 March 2008, with Vieira da
Cunha in charge (`br_pdt_history_lupi_leave_to_vieira_da_cunha_2008`); the convention of Friday 6 March 2009 re-elected
'O ministro Carlos Lupi, presidente licenciado da legenda', and Vieira da Cunha kept command while Lupi stayed away
(`br_pdt_lupi_reelected_while_licenciado_20090306`, `br_pdt_vieira_da_cunha_keeps_command_20090306`). Same-day reports
record his recondução at the conventions of Saturday 18 March 2017 and Monday 18 March 2019
(`br_pdt_lupi_reconducted_xxiii_convention_20170318`, `br_pdt_lupi_reconducted_xxv_convention_20190318`), matching the
closed SGIP organs 70996 and 269142 (registry periods 18/03/2017-18/03/2019 and 18/03/2019-21/01/2022). On 21
December 2021 'A resolução normativa 003/2021 é assinada pelo presidente nacional da legenda, Carlos Lupi'
(`br_pdt_lupi_signs_normative_resolution_003_2021_20211221`); his communiqué of 10 January 2022 moves the convention of
21 January 2022 online (`br_pdt_lupi_signs_communique_as_presidente_nacional_20220110`), and his convention speech of
21 January 2022 is published without a result (`br_pdt_convention_speech_published_20220121`).

Decision: accepted in part. Two holder observations, 9 February 2007 and 21 December 2021. The re-elections, the leave,
the acting service and the registry periods are claims.

Limits: no contemporaneous party record of 2011-2016 was found (the site is barely archived for 2012-2015); the day Lupi resumed after
the leave of 2008-2009 and the result of the convention of 21 January 2022 are open.

### PDT-PRES-04 — 2023-2026: the leave, André Figueiredo's acting service, the return and an attestation before the cutoff

Evidence: an item of 2 February 2023 says that 'o ministro da Previdência Social, Carlos Lupi, se licenciou da
presidência nacional do partido' while he is a minister, and announces André Figueiredo 'como presidente em exercício da
legenda' (`br_pdt_lupi_takes_leave_of_presidency_20230202`, `br_pdt_figueiredo_announced_presidente_em_exercicio_20230202`).
An item of 21 May 2025 says 'Carlos Lupi está de volta à presidência nacional do PDT', decided by the national executive
'nesta terça-feira (20)', and quotes him as 'o presidente nacional do PDT' (`br_pdt_lupi_returns_to_presidency_20250520`,
`br_pdt_lupi_styled_presidente_nacional_20250521`). Items of 19 August 2026 (event printed 'segunda-feira (18)', though
18 August 2026 was a Tuesday) and 4 September 2026 style him 'presidente nacional do PDT'
(`br_pdt_lupi_styled_presidente_nacional_20260819`, `br_pdt_lupi_styled_presidente_nacional_20260904`).

Decision: accepted in part. Lupi is observed on 21 May 2025 and 4 September 2026, three days before the cutoff. The
leave, Figueiredo's acting service and the return are claims.

Limits: the SGIP organ in force (2022-2027) would record the leave and the acting service but is a live record and a
lead; mid-period attestations of the acting service were not sought.

### MDB-PRES-01 — Jader Barbalho's presidency (1998-2001) and its end

Evidence: an undated PMDB page (Last-Modified header 22 October 1998, captured 9 February 1999) reads 'SENADOR JADER
BARBALHO PRESIDENTE NACIONAL DO PMDB.' and 'Eleito em 15 de Setembro de 1998.'
(`br_mdb_jader_styled_presidente_nacional_undated`, `br_mdb_jader_elected_19980915`). The party's retrospective list of
its presidents, captured in January 2019, gives his 'Mandato: 15/09/1998 a 15/05/2001' and Maguito Vilela's 'Mandato:
15/05/2001 a 09/09/2001' (`br_mdb_list_jader_mandate_1998_2001`, `br_mdb_list_maguito_mandate_2001`).

Decision: accepted in part. No day-dated party attestation was found, so Jader Barbalho has no holder; his election is
a claim, never a start. The list prints Maguito Vilela's service as a 'Mandato', like Jader Barbalho's and Temer's, not
as a 'Mandato Interino' like the entries for Iris de Araújo and Valdir Raupp: the party's own record presents him as
President from 15 May to 9 September 2001.

Limits: how and when his presidency ended is open (no pmdb.org.br page was captured between 4 September and 2 December
2001); Maguito Vilela is outside the ten people researched and has no holder. A Folha op-ed
reprinted on the party site (14 December 2000) is news and a lead.

### MDB-PRES-02 — Michel Temer, 2001-2009

Evidence: the retrospective list gives 'Mandato 09/09/2001 e licenciou-se no dia 10/03/2009'
(`br_mdb_list_temer_mandate_2001_leave_2009`). An undated Agência PMDB item on a vote of 'terça-feira (04/12)' says 'O
presidente do PMDB, deputado Michel Temer, também votou contra o projeto do Governo', without printing a year
(`br_mdb_temer_styled_presidente_do_pmdb_undated_2001`). The official note headed 'Brasília, 02 de julho de 2003 NOTA
OFICIAL' names 'o Presidente Nacional do PMDB, Deputado Michel Temer' and is signed 'Michel Temer Presidente do PMDB'
(`br_mdb_temer_signs_official_note_as_presidente_20030702`; located for this packet). The roster captured in November
2009 is headed 'Executiva Nacional (11 DE MARÇO DE 2007)' with 'Presidente: MICHEL TEMER (SP) (Licenciado)' and Iris de
Araújo 'Presidente em exercício' (`br_mdb_roster_temer_presidente_licenciado_capt2009`, undated,
`br_mdb_iris_presidente_em_exercicio_roster_capt2009`). On 10 March 2009 Iris de Araújo 'assumiu interinamente o cargo
de presidente do PMDB', and 'Temer justificou o licenciamento do cargo' (`br_mdb_iris_assumes_interim_presidency_20090310`,
`br_mdb_temer_leave_20090310`; the list's `br_mdb_list_iris_interim_2009_2010`).

Decision: accepted in part. Temer is observed on 2 July 2003. The leave and the interim service are claims.

Limits: his re-elections of 2004 and 2007 were not found as dated items; the 2001 styling has no year.

### MDB-PRES-03 — Temer, 2010-2016

Evidence: a press note of 21 January 2010 is signed 'Michel Temer Presidente licenciado do PMDB' and reports that the
members' meeting of Wednesday 20 January 2010 set the convention for 6 February 2010
(`br_mdb_temer_signs_note_as_presidente_licenciado_20100121`,
`br_mdb_convention_set_for_20100206_prospective_20100120`); the retrospective list says he 'Reassume a Presidência do
Partido em 27/01/2010 e licencia-se em 15/06/2010' (`br_mdb_list_temer_resumes_2010_leave_2010`). The roster captured
in May 2011 prints 'Data da posse: 10 DE MARÇO DE 2010', 'Presidente: MICHEL TEMER (SP) - Licenciado' and 'VALDIR RAUPP
(RO) - EM EXERCÍCIO' (`br_mdb_executive_posse_20100310`, `br_mdb_roster_temer_presidente_licenciado_capt2011`,
`br_mdb_raupp_em_exercicio_roster_capt2011`; the list's
`br_mdb_list_raupp_interim_2010_2014`). The convention of Saturday 2 March 2013 approved 'a recondução de
Michel Temer na presidência do PMDB, para o biênio 2013-2015' (`br_mdb_convention_held_20130302`,
`br_mdb_temer_reelected_20130302`). The list says he 'Assume a direção do Partido em 14/01/2014'
(`br_mdb_list_temer_assumes_2014`); a party item of 14 May 2014 says the executive's meeting was 'comandada pelo
presidente nacional da legenda, senador Valdir Raupp (RO)' (`br_mdb_raupp_styled_presidente_nacional_20140514`); the TSE
registry (organ 70945) lists Temer under the office 'PRESIDENTE LICENCIADO' with an exercise of his own from 16/07/2014
to 05/04/2016, ending on the day of his reported leave
(`br_mdb_tse_sgip_temer_registered_presidente_licenciado_20140716_20160405`). The convention of 12 March 2016 re-elected
him with 96% (`br_mdb_convention_held_20160312`, `br_mdb_temer_reelected_20160312`), and an item of 29 March 2016 styles
'O vice-presidente da República e presidente nacional do PMDB, Michel Temer' (`br_mdb_temer_styled_presidente_nacional_20160329`).

Decision: accepted in part. Temer is observed on 29 March 2016. The posse of 10 March 2010 is a claim (see above); the
leaves, Raupp's acting service and the re-elections are claims.

Limits: whether and when Temer resumed after the leave of June 2010 is unresolved: the list (14 January 2014) and the
item of 14 May 2014 conflict, and the registry names no President before 16 July 2014; the result of the convention of
6 February 2010 was not found.

### MDB-PRES-04 — 2016-2019: Temer's leave, Romero Jucá's service and the renaming

Evidence: the official communiqué of 5 April 2016 says Temer 'licenciou-se hoje (5) da Presidência do Partido' and
'No seu lugar, assume o senador Romero Jucá (RR), por prazo indeterminado' (`br_mdb_temer_leave_20160405`,
`br_mdb_juca_assumes_in_temer_place_20160405`); an item of 7 April 2016 styles 'O presidente nacional do PMDB, em
exercício, senador Romero Jucá' (`br_mdb_juca_styled_presidente_em_exercicio_20160407`). The edital of 23 November 2017,
signed 'ROMERO JUCÁ Presidente Nacional do PMDB', convenes the extraordinary convention of 19 December 2017 on 'o
restabelecimento do nome MOVIMENTO DEMOCRÁTICO BRASILEIRO – MDB' (`br_mdb_juca_signs_edital_as_presidente_nacional_20171123`,
`br_mdb_convention_called_renaming_prospective_20171123`); the convention of that day voted the renaming 325 to 88 with
27 blank (`br_mdb_extraordinary_convention_held_20171219`, `br_mdb_convention_votes_renaming_20171219`,
`br_mdb_juca_styled_presidente_nacional_20171219`); the adoption of the sigla was filed with the TSE on Wednesday 31
January 2018 (`br_mdb_renaming_communicated_to_tse_20180131`); the executive extended the national leadership's
mandates by one year from 2 March 2018 (`br_mdb_executive_extends_mandates_20180221`,
`br_mdb_juca_styled_presidente_nacional_20180222`); and the TSE approved the change of name and sigla unanimously on 15
May 2018 (`br_mdb_tse_approves_name_change_20180515`). The roster captured in January 2019 lists 'Presidente: MICHEL
TEMER (licenciado)' and '1º Vice-Presidente: ROMERO JUCÁ (em exercício)' and says 'Temer foi eleito presidente do MDB por
seis vezes' (`br_mdb_roster_temer_presidente_licenciado_capt201901`, `br_mdb_roster_juca_first_vice_em_exercicio_capt201901`,
`br_mdb_roster_temer_elected_six_times`); the list says Jucá 'Assume a direção interina do Partido em 05/04/2016'
(`br_mdb_list_juca_interim_2016`); the registry records him 'PRESIDENTE EM EXERCÍCIO | PRIMEIRO VICE-PRESIDENTE' from
12/03/2016 to 06/10/2019 (`br_mdb_tse_sgip_juca_registered_presidente_em_exercicio_20160312_20191006`); an item of 17
June 2019 styles 'O presidente do MDB, Romero Jucá' (`br_mdb_juca_styled_presidente_do_mdb_20190617`).

Decision: accepted in part. No holder: Temer stays the President on leave and Jucá's service is acting (see above).
The renaming is recorded as four organization claims and ties the MDB label to the PMDB.

Limits: whether Codex accepts the signed edital of 23 November 2017 as making Jucá a holder is left for a ruling.

### MDB-PRES-05 — Baleia Rossi, 2019 to the cutoff

Evidence: the convention of Sunday 6 October 2019 elected an executive 'que comandará o partido no biênio 2019-2021',
and 'o MDB será presidido pelo deputado Baleia Rossi (SP)' (`br_mdb_convention_elects_executive_baleia_20191006`); the
registry (organ 291052) records him PRESIDENTE from 06/10/2019 to 06/10/2023
(`br_mdb_tse_sgip_baleia_registered_exercise_20191006_20231006`); an item of 17 October 2019 says 'O novo presidente do
MDB, deputado Baleia Rossi (SP) reuniu ontem' the new executive's officers (`br_mdb_baleia_styled_novo_presidente_20191017`).
The executive extended his mandate to October 2022 on 23 February 2021 (`br_mdb_executive_extends_baleia_mandate_20210223`);
the convention of 5 October 2023 renewed it for two years (445 of 448 votes), and the directorate elected the executive
by acclamation, its image table headed by him (`br_mdb_convention_held_20231005`, `br_mdb_baleia_reelected_20231005`,
`br_mdb_directorate_elects_executive_by_acclamation_20231005`). The executive's signed minutes of 16 July 2025 record
that 'o Presidente Nacional do Partido e Deputado Federal Baleia Rossi (MDB-SP) iniciou a reunião' and extended the
executive and directorate to 15 May 2027 (`br_mdb_baleia_opens_executive_meeting_as_presidente_nacional_20250716`,
`br_mdb_executive_and_directorate_extended_to_20270515_20250716`, `br_mdb_baleia_reconduction_congratulated_20250716`).
Resolution MDB nº 1 of 30 March 2026 is signed 'BALEIA ROSSI Presidente Nacional.'
(`br_mdb_baleia_signs_resolution_1_2026_20260330`); a decision of 7 June 2026 closes 'Baleia Rossi Presidente Nacional
do MDB' (`br_mdb_baleia_signs_decision_as_presidente_nacional_20260607`); items of 8 and 15 June 2026 style him
President (`br_mdb_baleia_styled_presidente_nacional_20260608`, `br_mdb_baleia_styled_presidente_20260615`).

Decision: accepted in part. Three holder observations: 17 October 2019, 16 July 2025 and 7 June 2026. The elections,
extensions and continuation stylings are claims.

Limits: no posse day is printed; the extension of 2022 to 2023 was not found; no MDB page after late July 2026 is in the
archive index.

### PFL-PRES-01 — The PFL's President before the renaming: Jorge Bornhausen

Evidence: the PFL site's undated page of its executive, captured 17 October 2000, is headed 'Eleita e empossada na
Convenção de 07.05.99' and pairs 'Presidente' with 'Senador JORGE KONDER BORNHAUSEN - SC'
(`br_pfl_cen_elected_and_invested_bornhausen_presidente_19990507`); an Agência PFL item of 7 March 2003 says 'O
presidente do PFL, senador Jorge Bornhausen (SC), reiterou, hoje, 7,' (`br_pfl_bornhausen_styled_presidente_do_pfl_20030307`);
the convocation notice of 13 March 2007 is signed 'Jorge Bornhausen Presidente'
(`br_pfl_bornhausen_signs_convocation_as_presidente_20070313`).

Decision: claims only (no role). The claims would support observations of 7 March 2003 and 13 March 2007 and a
stated posse of the executive on 7 May 1999 if a PFL/DEM role were ever created.

Limits: the PFL presidents of 1990-1999 (including Hugo Napoleão) are outside the packet.

### PFL-PRES-02 — The renaming of the PFL as Democratas and Rodrigo Maia's presidency

Evidence: the notice of 13 March 2007 convenes the 'Convenção Extraordinária Nacional, a realizar-se no dia 28 de março
de 2007' with a 'proposta de reforma do Estatuto do Partido, que prevê a nova denominação da legenda'
(`br_pfl_convention_called_for_20070328_prospective_20070313`); an undated PFL page captured 29 March 2007 announces the
PFL's 'transformação em DEMOCRATAS' (`br_pfl_site_states_transformation_into_democratas`); Resolution n° 006 of 29 March
2007 is issued by 'A Comissão Provisória Nacional do Democratas' and signed 'DEPUTADO RODRIGO MAIA Presidente Nacional do
Democratas' (`br_pfl_resolution_006_issued_in_name_of_democratas_20070329`,
`br_pfl_maia_signs_resolution_006_as_presidente_nacional_20070329`); Resolution n° 112 of 15 February 2011 is signed
'Deputado Rodrigo Maia Presidente Nacional do Democratas' (`br_pfl_maia_signs_resolution_112_as_presidente_nacional_20110215`),
and an item of 14 March 2011 calls him 'o atual presidente do partido' (`br_pfl_maia_styled_atual_presidente_20110314`).

Decision: claims only. The renaming is an organization claim; Maia's stylings are recorded without a role.

Limits: the convention's own record of 28 March 2007 and the TSE's approval of the name were not found (the SGIP party
catalogue, a lead, gives DEM a registration date of 12/06/2007).

### PFL-PRES-03 — José Agripino's presidency

Evidence: the single slate was agreed 'no dia 16 de fevereiro' (`br_pfl_single_slate_agreement_announced_20110216`); the
item of 14 March 2011 announces the convention of Tuesday 15 March 'na qual o senador José Agripino assumirá a
presidência nacional' (`br_pfl_convention_to_formalize_agripino_prospective_20110314`); an item dated 16 March 2011 is
headed 'Agripino Maia é eleito por aclamação novo presidente do Democratas' (`br_pfl_agripino_elected_by_acclamation_reported`);
Resolution nº 113 of 24 March 2011 is signed 'SENADOR JOSÉ AGRIPINO MAIA Presidente Nacional do Democratas'
(`br_pfl_agripino_signs_resolution_113_as_presidente_nacional_20110324`); the registry (organ 70981) records him
PRESIDENTE from 03/12/2015 to 08/03/2018 (`br_pfl_tse_sgip_agripino_registered_exercise_20151203_20180308`).

Decision: claims only.

Limits: his re-elections of 2011-2015 and the end of his presidency are not researched beyond the registry period.

### PFL-PRES-04 — ACM Neto's presidency and the fusion into União Brasil

Evidence: the Democratas' undated page of its executive, captured 16 October 2018, is headed 'COMISSÃO EXECUTIVA
NACIONAL DE REFUNDAÇÃO DO DEMOCRATAS ELEITA NA CONVENÇÃO NACIONAL DE 08/03/2018' with 'Presidente Nacional: Prefeito ACM
Neto' (`br_pfl_cen_refundacao_elected_convention_acm_neto_20180308`); the registry records him PRESIDENTE from 08/03/2018
to 30/05/2019 and 'MEMBRO NATO | PRESIDENTE' from 30/05/2019 to 08/02/2022 (organs 248141 and 275827). An item of 5
October 2021 announces the convention of Wednesday 6 October 'para votar a fusão com o Partido Social Liberal (PSL)'
and quotes 'o presidente do DEM, ACM Neto' (`br_pfl_fusion_convention_scheduled_prospective_20211005`,
`br_pfl_acm_neto_styled_presidente_do_dem_20211005`); a notice of 12 January 2022 quotes 'O presidente nacional do
Democratas, ACM Neto' (`br_pfl_acm_neto_styled_presidente_nacional_20220112`). An item of 8 February 2022 says the
creation of União Brasil was approved by DEM and PSL 'em convenção conjunta realizada em outubro de 2021', that the TSE
would decide that evening, and calls ACM Neto 'o secretário-geral da legenda' and Luciano Bivar 'o presidente nacional
do União Brasil' (`br_pfl_fusion_approved_joint_convention_october_2021`,
`br_pfl_tse_session_on_uniao_scheduled_prospective_20220208`, `br_pfl_acm_neto_styled_uniao_secretary_general_20220208`);
the TSE's page dates that joint convention to 6 October 2021 (`br_pfl_tse_states_fusion_approved_joint_convention_20211006`),
and the registry's party record attached to the DEM organs names the party 'DEMOCRATAS (extinto por fusão com PSL,
originando o UNIÃO)' (`br_pfl_tse_sgip_dem_party_record_extinct_by_fusion`).
The TSE approved on 8 February 2022 the registration of União Brasil, an 'agremiação política resultante da fusão do
Democratas (DEM) com o Partido Social Liberal (PSL)' (`br_pfl_tse_approves_uniao_registration_fusion_20220208`), and
the registry's first União Brasil organ (405173, from 08/02/2022) carries CNPJ 44.551.496/0001-67
(`br_pfl_tse_sgip_uniao_organ_registry_identity_20220208`).

Decision: claims only; the `UNIÃO` observation is not linked (see above).

Limits: whether ACM Neto's DEM presidency ended with the registration of 8 February 2022 is a registry inference, not a
stated end; the offices of União Brasil are another organization's and are not researched.

## Sources added

All 76 sources are new records, appended after CLAUDE-C01-22's in `brazil.json` (PDT, then PMDB/MDB, then PFL/DEM, each
in date order); no existing source record or extract is edited. The columns give the source id, what it is, the
observations its claims serve and the recorded response identity (bytes and the first and last characters of the
SHA-256; the full values are in each extract and pinned in the test).

| Source ID | What | Observations | Response identity and provenance |
|---|---|---|---|
| `br_pdt_legalidade_interview_199611` | Jornal Legalidade (novembro de 1996, pág 4 e 5) - FHC usa roupagem democrática para servir aos grupos econômicos (interview reproduced on the PDT site) | PDT-PRES-01 | 18,394 bytes, `fb0bf4fc…e21f15`; capture 1997-06-30 |
| `br_pdt_home_19970411` | Home Page do PDT (Brasil), update of 11 April 1997 | PDT-PRES-01 | 25,433 bytes, `03f0b964…04166a`; capture 1997-04-14 |
| `br_pdt_marcha_speech_19990826` | Brizola impede que pare a campanha e pede que o povo se concentre na renúncia (PDT press item, Brasília, 26 de agosto de 1999) | PDT-PRES-01 | 18,015 bytes, `5e4e7c6f…f54d4b`; capture 2000-01-16 |
| `br_pdt_tijolaco_35_2004` | "O epitáfio da esperança" (Tijolaço column of Leonel Brizola, printed "06.05.2004") | PDT-PRES-01 | 25,195 bytes, `cc77652e…7c0df0`; capture 2004-06-15 |
| `br_pdt_salario_minimo_20040602` | PDT fecha questão contra salário mínimo de R$ 260 (PDT item, Brasília, 02/06/04) | PDT-PRES-01 | 13,833 bytes, `e3340fc3…c5dccc`; capture 2004-06-12 |
| `br_pdt_direcao_nacional_capt20040609` | Executiva Nacional (PDT site page quem_e_quem/direcao_nac.asp, Internet Archive capture of 9 June 2004) | PDT-PRES-01 | 14,611 bytes, `709efb8d…c68585`; capture 2004-06-09 |
| `br_pdt_home_20040621` | PDT - Partido Democrático Trabalhista (home page, update of 21/06/04 22h15m) | PDT-PRES-01 | 65,862 bytes, `6434552e…be5c3e`; capture 2004-06-22 |
| `br_pdt_curitiba_perda_20040622` | Brizola: perda insuperável para o PDT (PDT item, Curitiba, 22.06.04) | PDT-PRES-01 | 15,909 bytes, `1b6b81bd…c75dd3`; capture 2004-08-18 |
| `br_pdt_home_20040706` | PDT - Partido Democrático Trabalhista (home page, update of 06/07/04 10h17m) | PDT-PRES-02 | 46,336 bytes, `7430e33a…bce046`; capture 2004-07-07 |
| `br_pdt_direcao_nacional_capt20040722` | Direção Nacional (PDT site page quem_e_quem/direcao_nac.asp, Internet Archive capture of 22 July 2004) | PDT-PRES-02 | 15,559 bytes, `1f29d9dc…1a6f74`; capture 2004-07-22 |
| `br_pdt_perfil_lupi_capt20041011` | Carlos Lupi, presidente nacional do PDT (PDT site profile, Internet Archive capture of 11 October 2004) | PDT-PRES-02 | 19,160 bytes, `4a6e4c43…280c68`; capture 2004-10-11 |
| `br_pdt_edital_convencao_20050228` | Executiva Nacional realiza convenção em março (PDT item, 01.03.05), with the Edital de Convocação and Resolução nº 005/05 | PDT-PRES-02, PDT-PRES-03 | 20,965 bytes, `b3419d17…017db4`; capture 2005-03-05 |
| `br_pdt_convencao_reelege_20050321` | Convenção reelege Lupi para a presidência nacional do PDT (PDT item, Rio de Janeiro, 21/03/2005 - 16h25m) | PDT-PRES-03 | 44,214 bytes, `0813cd63…82d22a`; capture 2005-09-27 |
| `br_pdt_edital_convencao_20070209` | PDT realiza convenção nacional dia 9/03 (PDT item, with Resolução nº 001/07 of 9 February 2007) | PDT-PRES-03 | 20,067 bytes, `4c470275…3cbca9`; capture 2007-03-08 |
| `br_pdt_convencao_reelege_20090306` | Convenção Nacional reelege Lupi Presidente do PDT (PDT site post dated 6 de março de 2009) | PDT-PRES-03 | 72,613 bytes, `e43e2198…642a52`; capture 2021-05-13 |
| `br_pdt_tse_sgip_cen_2017_2019` | SGIP: PDT national organ 70996 (Comissão executiva, vigência 18/03/2017-18/03/2019) with members | PDT-PRES-03 | 14,992 bytes, `39ac6e0c…71572d`; TSE SGIP JSON, closed organ 70996 |
| `br_pdt_xxiii_convencao_20170318` | XXIII Convenção reconduz Lupi e exalta Ciro para presidente (PDT site, 18/03/2017) | PDT-PRES-03 | 63,558 bytes, `853b9be3…3e70ce`; capture 2017-03-18 |
| `br_pdt_historia_convencoes_20190315` | XXV Convenção Nacional do PDT: um resgate da história e das lutas do Trabalhismo (PDT site, 15 March 2019) | PDT-PRES-01, PDT-PRES-02, PDT-PRES-03 | 101,392 bytes, `a23cf8f9…9d3e79`; capture 2019-03-19 |
| `br_pdt_xxv_convencao_20190318` | Diretório Nacional do PDT fecha questão contra reforma da Previdência (PDT site, 18/03/2019) | PDT-PRES-03 | 73,307 bytes, `f6960ed1…dd03a2`; capture 2019-03-20 |
| `br_pdt_tse_sgip_cen_2019_2022` | SGIP: PDT national organ 269142 (Comissão executiva, vigência 18/03/2019-21/01/2022) with members | PDT-PRES-03 | 15,692 bytes, `cde06d86…b21c7d`; TSE SGIP JSON, closed organ 269142 |
| `br_pdt_convencao_hibrida_20211221` | PDT realiza convenção nacional híbrida em 21 janeiro, em Brasília (PDT site, 21/12/2021) | PDT-PRES-03 | 66,378 bytes, `586d61f8…eaf905`; capture 2022-01-20 |
| `br_pdt_convencao_virtual_20220110` | Convenção Nacional do PDT será realizada exclusivamente no modelo virtual (PDT site, 10/01/2022) | PDT-PRES-03 | 69,032 bytes, `c41343a7…6c2fcb`; capture 2022-01-20 |
| `br_pdt_discurso_lupi_20220121` | Discurso de Carlos Lupi na Convenção Nacional do PDT (PDT site, 21/01/2022) | PDT-PRES-03 | 76,754 bytes, `0d13d37f…3a51d2`; capture 2022-01-22 |
| `br_pdt_figueiredo_exercicio_20230202` | André Figueiredo assume a presidência nacional do PDT (PDT site, 02/02/2023) | PDT-PRES-04 | 88,104 bytes, `8fb84321…0d4145`; capture 2023-02-02 |
| `br_pdt_lupi_retorna_20250521` | Carlos Lupi retorna à presidência nacional do PDT (PDT site, 21/05/2025) | PDT-PRES-04 | 73,756 bytes, `f6269943…74ef80`; capture 2025-05-21 |
| `br_pdt_comite_fortaleza_20260819` | Com Carlos Lupi, Elmano, Camilo e Cid, André Figueiredo inaugura Comitê em Fortaleza e mostra força da militância (PDT site, 19/08/2026) | PDT-PRES-04 | 19,286 bytes, `62e503d4…87b690`; capture 2026-08-19, served zstd-encoded (decoded 81,758 bytes) |
| `br_pdt_eleicoes_2026_20260904` | PDT entra forte na disputa de 2026 com candidaturas competitivas em todas as regiões do país (PDT site, 04/09/2026) | PDT-PRES-04 | 24,187 bytes, `99e5090f…0c2d48`; capture 2026-09-05, served zstd-encoded (decoded 95,681 bytes) |
| `br_mdb_jader_presidente_page_1999` | O PMDB NA INTERNET (PMDB site page presid.htm, Internet Archive capture of 9 February 1999) | MDB-PRES-01 | 1,976 bytes, `969e0f67…07efd3`; capture 1999-02-09 |
| `br_mdb_temer_vota_clt_200112` | PMDB vota contra projeto que altera a CLT (Agência PMDB item, undated; capture of 11 January 2002) | MDB-PRES-02 | 10,603 bytes, `30ec40a3…595f82`; capture 2002-01-11 |
| `br_mdb_nota_oficial_roriz_20030702` | NOTA OFICIAL (PMDB national executive, Brasília, 02 de julho de 2003) | MDB-PRES-02 | 12,317 bytes, `f2110d78…563a48`; capture 2003-08-14 |
| `br_mdb_executiva_roster_capt2009` | Executiva Nacional (PMDB site page executiva.php, Internet Archive capture of 8 November 2009) | MDB-PRES-02 | 26,330 bytes, `7155be74…934e98`; capture 2009-11-08 |
| `br_mdb_iris_interina_20090310` | Íris de Araújo é a nova presidente do PMDB (PMDB item, FUG/PMDB, 10 de Março de 2009) | MDB-PRES-02 | 47,549 bytes, `c7a8b570…c686d0`; capture 2010-10-12 |
| `br_mdb_nota_temer_licenciado_20100121` | NOTA À IMPRENSA / Convenção Nacional - Fevereiro (PMDB item, 21 de Janeiro de 2010) | MDB-PRES-03 | 45,487 bytes, `f04896ba…200603`; capture 2010-10-10 |
| `br_mdb_executiva_roster_capt2011` | Executiva Nacional (PMDB site page executiva.php, Internet Archive capture of 21 May 2011) | MDB-PRES-03 | 26,833 bytes, `3c34c2cf…45beda`; capture 2011-05-21 |
| `br_mdb_convencao_2013_20130302` | PMDB elege nova Executiva Nacional para o biênio 2013-2015 (PMDB item, FUG/PMDB, 2 de março de 2013) | MDB-PRES-03 | 24,869 bytes, `045d5b38…61e912`; capture 2013-03-05 |
| `br_mdb_tse_sgip_cen_2013_2019` | SGIP: MDB national organ 70945 (Comissão executiva, vigência 11/03/2013-06/10/2019) with members | MDB-PRES-03, MDB-PRES-04 | 18,952 bytes, `1d00f301…2b2247`; TSE SGIP JSON, closed organ 70945 |
| `br_mdb_raupp_presidente_nacional_20140514` | Em reunião da Executiva Nacional, parlamentares e diretórios reforçam apoio a Michel Temer (PMDB item, FUG/PMDB, 14 de maio de 2014) | MDB-PRES-03 | 29,191 bytes, `20b987ff…c912e1`; capture 2014-05-28 |
| `br_mdb_convencao_2016_20160312` | Com 96% dos votos, Michel Temer é reconduzido à presidência do PMDB (PMDB item, 12 de março de 2016) | MDB-PRES-03 | 26,811 bytes, `8df431b5…8f314a`; capture 2016-03-14 |
| `br_mdb_rompe_alianca_20160329` | PMDB rompe aliança com o PT e o governo federal (PMDB item, FUG/PMDB, 29 de março de 2016) | MDB-PRES-03 | 26,097 bytes, `02b1d59b…f9c85f`; capture 2016-04-09 |
| `br_mdb_comunicado_licenca_temer_20160405` | COMUNICADO OFICIAL (PMDB item, PMDB Nacional, 5 de abril de 2016) | MDB-PRES-04 | 22,944 bytes, `bece8f74…6ba71d`; capture 2016-04-13 |
| `br_mdb_juca_em_exercicio_20160407` | Jucá encaminha a Comissão de Ética do PMDB pedido de expulsão dos ministros Katia Abreu e Celso Pansera (PMDB item, ACS/PMDB, 7 de abril de 2016) | MDB-PRES-04 | 23,911 bytes, `48664939…53d510`; capture 2016-04-21 |
| `br_mdb_edital_convencao_20171123` | Edital de Convocação (PMDB site page, dated 13 de dezembro de 2017; edital of 23 November 2017) | MDB-PRES-04 | 23,935 bytes, `d835f760…bfa9f3`; capture 2017-12-30 |
| `br_mdb_pmdb_muda_sigla_20171219` | PMDB muda a sigla e volta a ser o MDB (PMDB item, PMDB Nacional, 19 de dezembro de 2017) | MDB-PRES-04 | 26,785 bytes, `ab5b523a…69dca0`; capture 2017-12-22 |
| `br_mdb_adocao_sigla_tse_20180201` | Partido comunica adoção da sigla MDB à Justiça Eleitoral (MDB item, Redação MDB, 01 fevereiro 2018) | MDB-PRES-04 | 19,879 bytes, `ac7c03e3…3e4ce6`; capture 2019-01-07 |
| `br_mdb_prorroga_mandatos_20180222` | Executiva do MDB prorroga mandatos da direção nacional do partido (MDB item, Redação MDB, 22 fevereiro 2018) | MDB-PRES-04 | 19,612 bytes, `6e841c66…432df9`; capture 2019-01-06 |
| `br_mdb_tse_name_change_20180515` | Aprovada mudança do nome do Partido do Movimento Democrático Brasileiro (PMDB) (TSE news, 15.05.2018 21:45; Pet 128) | MDB-PRES-04 | 90,320 bytes, `8489553b…8375d9`; capture 2018-05-19 |
| `br_mdb_executiva_roster_capt201901` | Comissão Executiva Nacional (MDB site page, Internet Archive capture of 5 January 2019) | MDB-PRES-04 | 22,963 bytes, `4b2e254c…5c04d3`; capture 2019-01-05 |
| `br_mdb_presidentes_lista_capt201901` | Presidentes do MDB (MDB site page, Internet Archive capture of 5 January 2019) | MDB-PRES-01, MDB-PRES-02, MDB-PRES-03, MDB-PRES-04 | 20,891 bytes, `dc42d807…148183`; capture 2019-01-05 |
| `br_mdb_juca_juventude_20190617` | Romero Jucá, presidente do MDB, participa de encontro da Juventude do partido (MDB item, Redação MDB, 17 junho 2019) | MDB-PRES-04 | 21,843 bytes, `d4f1d320…bef697`; capture 2019-06-19 |
| `br_mdb_eleicao_baleia_20191007` | União e renovação marcam eleição de Baleia Rossi como novo presidente nacional do MDB (MDB item, Redação MDB, 07 outubro 2019) | MDB-PRES-05 | 57,402 bytes, `2df1129a…af6953`; capture 2022-11-01 |
| `br_mdb_tse_sgip_cen_2019_2023` | SGIP: MDB national organ 291052 (Comissão executiva, vigência 06/10/2019-06/10/2023) with members | MDB-PRES-05 | 21,597 bytes, `a2a977c8…945697`; TSE SGIP JSON, closed organ 291052 |
| `br_mdb_baleia_reune_executiva_20191017` | Presidente Baleia Rossi reúne direção da nova Executiva do partido (MDB item, Redação MDB, 17 outubro 2019) | MDB-PRES-05 | 46,414 bytes, `b865cd05…c97eb6`; capture 2022-11-02 |
| `br_mdb_baleia_prorrogacao_20210223` | Baleia Rossi é reconduzido como presidente do MDB até outubro de 2022 (MDB item, Redação MDB, 23 fevereiro 2021) | MDB-PRES-05 | 25,530 bytes, `e8883937…cadb5f`; capture 2021-02-23 |
| `br_mdb_convencao_20231005` | MDB elege nova Comissão Executiva Nacional e novo Diretório Nacional sob influência de voto popular (MDB item, Redação MDB, 05 outubro 2023) | MDB-PRES-05 | 43,330 bytes, `a8c25954…364e1e`; capture 2023-10-14 |
| `br_mdb_ata_diretorio_20231005` | Ata da Reunião do Diretório Nacional do Movimento Democrático Brasileiro - MDB (5 October 2023; signed PDF) | MDB-PRES-05 | 232,963 bytes, `11e40f63…7bef00`; capture 2024-03-01 |
| `br_mdb_ata_executiva_20250716` | Ata da Reunião da Comissão Executiva Nacional do Movimento Democrático Brasileiro - MDB (16 July 2025; signed PDF) | MDB-PRES-05 | 246,382 bytes, `34de51f9…8261cb`; capture 2025-09-15 |
| `br_mdb_executiva_convocada_df_20260608` | Comissão Executiva Nacional é convocada para deliberar sobre requerimento do MDB do Distrito Fderal (MDB item, Redação MDB, junho 8, 2026) | MDB-PRES-05 | 27,805 bytes, `111b30b5…960809`; capture 2026-06-18, served zstd-encoded (decoded 126,473 bytes) |
| `br_mdb_resolucao_blinda_20260615` | MDB blinda candidaturas para 2026 com normas rígidas contra a influência do crime organizado; Leia íntegra da Resolução (MDB item, Redação MDB, junho 15, 2026) | MDB-PRES-05 | 31,429 bytes, `6bfcfc39…ec88e2`; capture 2026-06-18, served zstd-encoded (decoded 136,700 bytes) |
| `br_pfl_site_executiva_nacional_1999` | PFL - Comissão Executiva Nacional, 'Eleita e empossada na Convenção de 07.05.99' (PFL site page orgaos/executiva_nacional.htm, Internet Archive capture of 17 October 2000) | PFL-PRES-01 | 3,808 bytes, `2383d571…0b1e79`; capture 2000-10-17 |
| `br_pfl_agencia_bornhausen_20030307` | PT não apresenta projeto para as reformas e segue política do governo anterior porque não tem proposta de modelo econômico para o país (Agência PFL, 7/3/2003; PFL site news item) | PFL-PRES-01 | 11,708 bytes, `0937d441…9b0327`; capture 2003-03-13 |
| `br_pfl_convencao_2007_edital_20070313` | Convenção Nacional Extraordinária - Edital de Convocação (PFL site page convencao2007.asp; Brasília, 13 de março de 2007) | PFL-PRES-01, PFL-PRES-02 | 16,102 bytes, `58a43687…2476ce`; capture 2007-03-17 |
| `br_pfl_site_democratas_transformation_2007` | :: DEMOCRATAS :: (PFL site page democratas.asp, Internet Archive capture of 29 March 2007) | PFL-PRES-02 | 2,298 bytes, `d5d0f6cf…bc67ef`; capture 2007-03-29 |
| `br_pfl_dem_resolucao_006_20070329` | Resolução n° 006, de 29 de março de 2007 (Democratas site, Resoluções) | PFL-PRES-02 | 42,365 bytes, `8429f426…1a328b`; capture 2011-08-25 |
| `br_pfl_dem_resolucao_112_20110215` | Resolução n° 112, de 15 de fevereiro de 2011 (Democratas site, Resoluções) | PFL-PRES-02 | 46,200 bytes, `458aa245…156edc`; capture 2011-10-07 |
| `br_pfl_dem_convencao_20110314` | Democratas realiza Convenção Nacional e elege nova Executiva (Democratas site, 14 de março de 2011) | PFL-PRES-02, PFL-PRES-03 | 42,578 bytes, `786f897a…cd2b25`; capture 2011-03-18 |
| `br_pfl_dem_agripino_eleito_20110316` | Agripino Maia é eleito por aclamação novo presidente do Democratas (Democratas site, 16 de Março de 2011) | PFL-PRES-03 | 21,728 bytes, `68cc2f88…680101`; capture 2019-01-21 |
| `br_pfl_dem_resolucao_113_20110324` | Resolução nº 113, de 24 de março de 2011 (Democratas site, Resoluções) | PFL-PRES-03 | 44,631 bytes, `487a0eac…b71df6`; capture 2011-10-07 |
| `br_pfl_tse_sgip_dem_cen_2015_2018` | SGIP: DEM national organ 70981 (Comissão executiva, vigência 03/12/2015-08/03/2018) with members | PFL-PRES-03 | 23,088 bytes, `c88585ae…01fd10`; TSE SGIP JSON, closed organ 70981 |
| `br_pfl_dem_cen_refundacao_2018` | COMISSÃO EXECUTIVA NACIONAL \| Democratas (party site page, Internet Archive capture of 16 October 2018) | PFL-PRES-04 | 24,687 bytes, `28d6e082…821efc`; capture 2018-10-16 |
| `br_pfl_tse_sgip_dem_cen_2018_2019` | SGIP: DEM national organ 248141 (Comissão executiva, vigência 08/03/2018-30/05/2019) with members | PFL-PRES-04 | 24,059 bytes, `ae9ce041…95c079`; TSE SGIP JSON, closed organ 248141 |
| `br_pfl_tse_sgip_dem_cen_2019_2022` | SGIP: DEM national organ 275827 (Comissão executiva, vigência 30/05/2019-08/02/2022) with members | PFL-PRES-04 | 24,913 bytes, `d2259050…4bacbf`; TSE SGIP JSON, closed organ 275827 |
| `br_pfl_dem_convencao_fusao_20211005` | Convenção Nacional: DEM vota nesta quarta-feira (6) fusão com PSL (Democratas site, 05 outubro 2021) | PFL-PRES-04 | 61,005 bytes, `6b48db86…4428db`; capture 2021-10-27 |
| `br_pfl_dem_informe_acm_neto_20220112` | Informe: presidente ACM Neto testa positivo para Covid-19 (Democratas site, 12 janeiro 2022) | PFL-PRES-04 | 58,967 bytes, `b76c7473…e92ac1`; capture 2022-01-12 |
| `br_pfl_dem_homologacao_uniao_20220208` | AO VIVO: Homologação do União Brasil em análise no TSE (Democratas site, 08 fevereiro 2022) | PFL-PRES-04 | 60,279 bytes, `744c26ce…0f14af`; capture 2022-02-09 |
| `br_pfl_tse_sgip_uniao_cen_2022_2024` | SGIP: UNIÃO national organ 405173 (Comissão executiva, vigência 08/02/2022-31/05/2024), organ data only | PFL-PRES-04 | 22,515 bytes, `1e5d34f1…74dfa3`; TSE SGIP JSON, closed organ 405173 |
| `br_pfl_tse_uniao_registro_20220208` | TSE aprova registro do partido União Brasil (Tribunal Superior Eleitoral, 08/02/2022 20:35, atualizado em 11/08/2022) | PFL-PRES-04 | 93,606 bytes, `0ebc6d52…2914fe`; capture 2023-03-26 |

The sources are the parties' own records and the Superior Electoral Court's: raw Internet Archive captures of the PDT's
site (www.pdt.org.br, 1997-2026), the PMDB's and MDB's sites (www.pmdb.org.br, pmdb.org.br and www.mdb.org.br,
1999-2026, including two sets of signed minutes as PDFs), the PFL's and Democratas' sites (www.pfl.org.br, www.dem.org.br
and dem.org.br, 2000-2022), two TSE news pages on its own decisions (the MDB renaming, 2018; the União Brasil
registration, 2022), and eight closed national organs in the TSE's SGIP registry. News (Folha, G1 and Agência Estado
reprints, other press) and encyclopaedias are leads only.

Each source has a derived factual extract under [sources/](sources/) in the house format
(`spheres-c01-derived-factual-table/v1`): one row per claim, keyed by `claim_id`, with `observation_id` (the PDT or MDB
observation, or null for the PFL/DEM chain), `review_observation`, `role_id` (`br_pdt_president`, `br_mdb_president` or
null), a normalised `holder_name`, `role_title` (the office, the acting office, the PFL/DEM office or null), `event_kind`,
`attested_on`, the claim's text and locator. Each extract records the original response's byte count and SHA-256, its
content encoding (and decoded identity where the archive serves the capture encoded), a fetch recipe, the stability
check and a provenance note saying that the original is not checked into this repository. Original pages, PDFs and
registry records are not checked in, and no photograph, logo, signature or personal registry field is reproduced: the
SGIP extracts carry only the organ's dates and CNPJ and the President's name, office, exercise dates and status (the
test asserts that no CPF, voter-registration, contact or other personal field is present).

New source types: `primary_party_minutes_pdf_archived` and `primary_electoral_court_news_archived`; the others reuse
CLAUDE-C01-22's `primary_party_web_page_archived` and `primary_electoral_court_party_registry_json`.

## Response identities and stability checks

Every recorded response was downloaded for this packet at least twice on 28 September 2026 (runs T1, T2 and T3), the
first and last at least 30 minutes apart, with the same byte count and SHA-256 each time; the times are in each
extract's `stability_check`. The helpers' own downloads (two each, 30-48 minutes apart) matched as well.

| Responses | Downloads for this packet (UTC, 28 September 2026) | Stability basis |
|---|---|---|
| SGIP records of closed organs (8) | T1 22:00-22:18; T2 22:42-22:43; T3 23:17-23:20 | JSON generated per request but without any timestamp, token or session value; closed organs (Não Vigente); byte-identical in every run |
| PFL/DEM captures (14) | T1 22:09-22:10; T2 22:42-22:43; T3 23:18-23:20 | raw id_ captures, no Content-Encoding; every served body's SHA-1 equals the capture's CDX digest |
| PDT captures (25) | T1 22:33-22:34; T2 22:43-22:46; T3 23:20-23:21 | raw id_ captures; two captures of 2026 are served zstd-encoded (see below); every served body's SHA-1 equals the capture's CDX digest |
| PMDB/MDB captures (29) | T1 22:43-22:45; T3 23:21-23:23 | raw id_ captures; two captures of 2026 are served zstd-encoded; two older captures differ from their CDX digest (see below) |

Reproducibility notes for the reviewer. The Internet Archive serves most raw captures uncompressed to a client that
sends `Accept-Encoding: identity` or no Accept-Encoding header; fetch with curl without `--compressed`. Four captures of
2026 (`br_pdt_comite_fortaleza_20260819`, `br_pdt_eleicoes_2026_20260904`, `br_mdb_executiva_convocada_df_20260608`,
`br_mdb_resolucao_blinda_20260615`) are served with `Content-Encoding: zstd` even to an identity request: their recorded
identity is the zstd body as served (its SHA-1 equals the CDX digest), and the decoded identities are recorded beside
it; do not let the client decode them. Two captures do not match their CDX digest although they were byte-stable: the
1999 page `presid.htm` (an old ARC record; the served length equals the archived original Content-Length) and the 2011
roster `executiva.php` (the original response was chunked). The SGIP records are closed organs and are not expected to
change, but a later annotation by the court would change their bytes; they carry members' personal data, which must
not be copied, and the raw bodies were deleted for this packet after hashing and review. No search or listing page,
live party page, live SGIP organ in force, SGIP party catalogue or cache-busting query is a recorded identity.

## Leads not imported

- The PDT's 'Líderes Históricos' page,
  https://web.archive.org/web/20170118152356id_/http://www.pdt.org.br/index.php/o-pdt/lideres-historicos/ (50,119 bytes,
  capture of 18 January 2017): Lupi assumed the national presidency 'em 21 de junho de 2004, quando Leonel Brizola
  morreu'; an undated retrospective page, conflicting with the 2004 profile and the 2019 history.
- A G1/Agência Estado story reposted on the PDT site,
  https://web.archive.org/web/20190220050508id_/http://www.pdt.org.br/index.php/g1-lupi-diz-que-se-licenciou-para-acalmar-forcas-raivosas/
  (70,804 bytes, `e72cccc7…04fbc`): news; it reports Lupi's leave already on 7 March 2008, before the 2019 history's 12
  March 2008.
- A Tribuna da Imprensa article of 17 February 2001 reproduced on the PDT site (bzcnac17201.htm) and a Folha op-ed by
  Jader Barbalho of 14 December 2000 reprinted on the PMDB site
  (https://web.archive.org/web/20010626225526id_/http://www.pmdb.org.br:80/not_direita.htm): newspaper texts, leads only.
- Undated rosters not needed: the PDT's dirna.htm (1997), the PMDB's executiva_direita.htm (2001; Jader Barbalho
  President, Maguito Vilela first vice-president) and later executive pages.
- The PDT executive's note of 28 June 2004 (partido/pdt280604.asp), linked from the home pages of 30 June and 6 July
  2004: never archived.
- The PDT home page updated 15 June 2004 (https://web.archive.org/web/20040616080719id_/http://www.pdt.org.br:80/;
  60,035 bytes, `a08434ea…c6c49a`): it dates the Encontro Nacional 'nos dias 4 e 5 últimos' (4-5 June 2004), which
  bears only on the dating of Brizola's Tijolaço column; not imported.
- The PMDB item of 27 January 2010 on the executive's meeting (noticias.php?cd=2108): 'o presidente Michel Temer'
  without the office; not re-downloaded.
- The MDB's convention minutes of 5 October 2023 (ATA-DA-REUNIAO-DA-CONVENCAO-NACIONAL-ORDINARIA-...pdf), its roster PDF
  of the executive elected in 2019 (made in 2022) and the landing page of Resolução MDB 01/2026 (resolucao-mdb-01-2026):
  duplicates or retrospective.
- The TSE's SGIP organs in force: PDT idOrgaoPartidario=406194 and idOrgaoPartidario=406570 (from 22/01/2022), MDB
  idOrgaoPartidario=468918 (from 06/10/2023), and UNIÃO 507706; live records that change on each annotation (the PDT
  organ would record Lupi's leave and Figueiredo's acting service). The SGIP organ index (orgaoPartidario/consulta) and
  party catalogue (api/v1/partidos, which dates DEM's registration 12/06/2007, its fusion 08/02/2022 and the MDB's
  change 15/05/2018) are growing listings, used only to find organ ids.
- The Fundação Leonel Brizola - Alberto Pasqualini, cited by the gap ledger for the Lupi succession: not fetched.
- The Chamber's solemn session honouring Brizola (camara.gov.br/Internet/plenario/notas/solene/hv220604.pdf): a tribute,
  not a record of the party office.
- News and encyclopaedias on the PMDB, PDT and DEM leaderships (Folha, Estadão, G1, Poder360, Wikipedia): leads only.

## Sources attempted

- www.tse.jus.br (live): Akamai 'Access Denied' (403); not bypassed; raw captures of the TSE's news pages are used.
- www.mdb.org.br (from late July 2026): Cloudflare challenge (403) on captures; no MDB article after late July 2026 is
  in the archive index.
- The PDT site 2012-2015: only about forty HTML captures; no convention, leave or return item found.
- pmdb.org.br between 4 September and 2 December 2001: no capture, so no party record of the 2001 transition.
- PMDB news ids of February 2010 (noticias.php?cd=2193-2226): archive redirect stubs; the result of the convention of
  6 February 2010 was not found.
- One 1999 PDT home-page capture returned an archive HTTP 500; several first attempts failed with curl exit 7 and
  succeeded on retry. No HTTP 429 was received.

## Checker defects

Two independent checks read every claim, extract row and holder against the stored bodies: one the PDT and PFL/DEM
chains (5 must-fix defects, 15 suggestions), the other the PMDB/MDB chain (8 must-fix defects, 8 suggestions). All
must-fix defects are fixed:

- Brizola's Tijolaço column: its printed '(06.05.2004)' is contradicted by its own text (the R$ 260 minimum-wage vote in
  the Chamber and the Encontro Nacional 'amanhã e sábado', 4-5 June 2004), so the source is renamed
  `br_pdt_tijolaco_35_2004`, its claim is undated, and Brizola's third holder observation is the caucus meeting of 2
  June 2004; the holder's uncertainty no longer says the convention of March 2003 is known only from the 2004 profile
  (the 2019 history dates it 21 March 2003).
- Lupi's 2025 styling: the claim text quotes the paragraph that styles him and the closing 'afirmou Lupi' two paragraphs
  later, as the body has them.
- SGIP: the scope notes' record-creation and alteration dates are checked and corrected against the stored bodies, and
  the raw SGIP bodies with members' personal data are deleted after hashing (the notes say so).
- Temer's and Jucá's registry periods: Temer's 'PRESIDENTE LICENCIADO' period (16/07/2014-05/04/2016) is his own and
  ends on the day of his reported leave; Jucá's runs from the convention of 12 March 2016 under a combined label;
  neither follows the organ's term as first written.
- 'His election of 9 September 2001' becomes the start of his mandate ('Mandato 09/09/2001'); the convention of 6
  February 2010 was set on 20 January 2010 (`br_mdb_convention_set_for_20100206_prospective_20100120`); the 2009 roster
  with Temer 'Licenciado' and Iris de Araújo 'Presidente em exercício' is undated
  (`br_mdb_roster_temer_presidente_licenciado_capt2009`: its heading day identifies the executive but names no event);
  Baleia Rossi's 7 June 2026 uncertainty no longer calls it the latest day-dated MDB record; the PMDB-era sources'
  publisher is the archived pmdb.org.br page, not the MDB site.

Suggestions applied: the retrospective re-election of 9 March 2007; the TSE's statement of the joint fusion convention
of 6 October 2021 and the DEM party record in a recorded registry body; contemporaneous evidence against an interim
reading of Lupi's first months (the roster captured 22 July 2004 and 'confirmado na presidência' in 2005); the 20/06
weekday slip in the report of Brizola's death and its corroboration; the revision date of 4 January 2022 on the December
2021 resolution; 'indicates' for the speech of 21 January 2022; the real reason the November 1996 styling is undated;
one of three named PFL vice-presidents; the prospective 'o novo presidente' of 14 March 2011; 'o novo partido' for União
Brasil; 'no contemporaneous day-dated PDT record'; the Agência PFL page title; the doubled conditional of the 1992
history; the posse of 10 March 2010 split from the undated 'Licenciado' roster status; interim list entries as
`interim_service`; birth dates removed from the list quotations; Maguito Vilela's 'Mandato'; the event days behind the
dated Jucá items (6 April 2016, 21 February 2018) and his teaser styling of 19 June 2019; the context of Baleia Rossi's
2025 'recondução'; the Chamber-presidency remark dropped from Temer's 2009 leave. Lupi's 2025 observation, which the
check suggested Codex rule on, stays with its ruling noted under Integration notes.

## Suggested next work orders

These are proposals for the integrator. They are not created in `work-orders.json`.

- `C01-Brazil-PMDB-002`: the PMDB presidents of 1990-1998 (Ulysses Guimarães, Orestes Quércia, Luiz Henrique da
  Silveira, Paes de Andrade) from Congress diaries and party publications, and the 2001 transition (Jader Barbalho,
  Maguito Vilela, whom the party's list presents as President in 2001, and the start of Temer's mandate on 9 September
  2001).
- `C01-Brazil-PDT-002`: who presided over the PDT on 1 January 1990 (Brizola or Doutel de Andrade), the 28 June 2004
  executive note, Lupi's return after 2008-2009, and the PDT's SGIP organ in force at a fixed date.
- `C01-Brazil-PFL-002`: the PFL presidents of 1990-1999 (including Hugo Napoleão), the convention of 28 March 2007 and
  the TSE's approval of the name Democratas, and whether a PFL/DEM organization observation should be created (only by
  Codex's decision).
- `C01-Brazil-UNIAO-001`: the União Brasil national presidency (2022-2026) as its own organization, if wanted.
- `C01-Brazil-MDB-003`: Temer's re-elections of 2004 and 2007, his returns after 2009 and 2010, and any posse day of
  Baleia Rossi.

## Integration notes (outside this packet's file boundary)

- **Stack, base and claim:** `claude/c01-br-34` is based on `032cd6a3` and **not stacked** on any pending packet (the
  former candidate `claude/c01-source-17` was integrated by Codex as its own scoped commit). Claim commit `c2ff9396`
  holds only the handoff. Integration moved to `44098c5a` during the work (A1 geography and native diagnostic evidence,
  `.gitattributes` and planning files; nothing under C01 research); it was merged cleanly into the branch
  (`e8f38113`) before the packet commits, with no conflict. This packet extends `brazil.json` additively: 76 source records appended after CLAUDE-C01-22's;
  on the MDB and PDT observations, `roles` set to the one new role each and the new ids appended to their `sources` and
  `claim_ids`; one coverage note appended to each of the two observations and one to the packet. No existing source
  record or extract is edited, and no earlier Brazil record changes.
- `research-index.json` is regenerated in a **separate commit**, and it is **the only file shared with other pending
  packets**. New totals against `032cd6a3` (and `44098c5a`, which leaves the index unchanged): 1,647 sources and 4,309 claims (previously 1,571 and 4,182);
  Brazil has one institution, five role observations (previously three), 535 claims and 32 entries pending mapping,
  and its four discovery batches stay 10, 10, 10 and 2 members. Regenerate the index after the other packets merge.
- Pinned tests re-expressed, none loosened and no assertion removed:
  - `test_brazil_research_s10f.py`: counts (entries, sources, claims, roles) are now (32, 248, 535, 5); the 76 new
    sources are pinned to their two hosts and the 2026-09-28 access date and to their order at the end of the packet;
    the guard that only the PT observation has a role becomes an exact pin of the three party roles (MDB, PDT, PT,
    each exactly one role of kind `party_leader`; every other organization none); `mapping_pending` (32) and the
    work-order sizes are unchanged. It imports `NEW_SOURCES` from the new test.
  - `test_brazil_presidents_c01_10.py`: entries and roles (32, 5); the full source list ends with CLAUDE-C01-34's 76; the
    rows loaded after the original sources are exactly CLAUDE-C01-10's, -17's, -22's and -34's claims; the event map
    stays exact for CLAUDE-C01-10's claims; `role_observations` is 5. It imports the new test module.
  - `test_brazil_vice_presidents_c01_17.py`: counts (32, 248, 535, 5); its 11 sources stay pinned at their position;
    the total is pinned with CLAUDE-C01-22's 108 (all `br_pt_`) and CLAUDE-C01-34's 76 (all `br_pdt_`, `br_mdb_` or
    `br_pfl_`); the packet coverage notes are pinned at `[-3]` (vice-presidents), `[-2]` (PT) and `[-1]` (this packet);
    `role_observations` is 5.
  - `test_brazil_pt_presidents_c01_22.py`: counts (32, 248, 535, 5); its 108 sources are pinned at their position
    before this packet's 76; the guard that the PT office is the only `party_leader` role is re-expressed exactly: the
    PT office exists once, on the PT observation, and the only other `party_leader` roles are this packet's two, on the
    MDB and PDT observations; its packet coverage note is pinned at `[-2]`; `role_observations` is 5 and the Brazil
    claim count 535.
- `campaign_census.py --check` fails on the base `032cd6a3` itself, and still on `44098c5a`: integration commit `262d5f61` changed
  `spheres-sim/src/government.rs` without regenerating `census.json`. The only difference is in `census.json`: its
  `source_files` entry for `spheres-sim/src/government.rs` (recorded 848,551 bytes, `0e2dbdef…`, against the current
  849,546 bytes, `3f846b4b…`) and therefore `production_snapshot.stale_inputs` and `all_declared_inputs_current`; every
  other census output matches. It is not fixed here.
- Known failures outside this packet, never fixed here: `tools/avatars/test_certified_gap_ledger.py` errors on this
  packet's new sources ('no pinned attribution') until Codex classifies the packet's commit in `COMMIT_PACKETS` at
  integration; `tools/avatars/test_certified_boundary_matrix.py` (S23) needs `spheres-web/src`, which this sparse
  checkout lacks, and in a full checkout would report the packet as `unclassified_packet` with stale boundary-matrix
  files, so Codex must list the packet and regenerate `docs/campaign-certification/S23/preparation/boundary-matrix/`.
  The gap ledger itself is not touched.
- The atlas (`tools/ui/leadership-research-review.js`) will show the MDB observation with one role and five holders and
  the PDT observation with one role and eight holders; one PDT holder has an end. The PFL/DEM claims are not attached to
  any entry and so are not shown there. No UI code changed.
- New vocabulary: the role kind `party_leader` was already in `ROLE_KINDS`; the event kinds listed in the test
  (`FROM_KINDS`, `UNTIL_KINDS`, `ACTING_KINDS`, `ELECTION_KINDS`, `DEPARTURE_KINDS`, `STYLING_KINDS`,
  `ORGANIZATION_KINDS`, `RETROSPECTIVE_KINDS`) are pinned.
- Rulings for Codex: Brizola's `until` on the party's same-day report of his death in office (fallback: a claim only);
  Lupi's 2004 observation although the 2019 history calls his first months interim; Lupi's 2025 observation on a styling
  inside the report of his return (fallback: observed first on 4 September 2026); the posse of the executive of 10
  March 2010 as a claim rather than Temer's start; Jucá as acting despite the signed edital of 23 November 2017; the
  PFL/DEM chain kept as claims only and the `UNIÃO` observation unlinked.
- `research/README.md`, the C01 README totals and `docs/planning/ai-workstreams.json` are left for the integrator; this
  handoff is self-proposed and not registered there.

## Checks

```text
python -X utf8 tools/avatars/campaign_research.py
python -X utf8 tools/avatars/campaign_research.py --check
python -X utf8 tools/avatars/campaign_census.py --check
python -X utf8 -m unittest discover -s tools/avatars -p "test_brazil*.py"
python -X utf8 -m unittest discover -s tools/avatars -p "test_*research*.py"
python -X utf8 -m unittest discover -s tools/avatars -p "test_campaign*.py"
node --test tools/ui/check_leadership_research_review.cjs
python tools/planning/workboard.py --check
git diff --check
```

Results are recorded in the handoff.
