# PRN / PTC / Agir national presidents 43: one party office across two renamings, 1990-2026

Packet: **CLAUDE-C01-43**. State: **ready_for_review** (not complete).
Owner: Claude. Integrator/reviewer: Codex. Branch `claude/c01-br-43`, based on `509bd289` (current
`codex/campaign-certification`), with the moved integration `229df210` merged cleanly (`803b2627`), **not
stacked** on any pending packet; claim commit `6e6a9d06`. Research access:
30 September to 1 October 2026 (downloads dated 1 October 2026 UTC). The historical cutoff stays **7 September 2026**.

This packet adds one party role to an existing 2024 funding observation in [brazil.json](brazil.json):
`br_agir_president` (Presidente Nacional do Partido da Reconstrução Nacional (PRN) / Partido Trabalhista Cristão (PTC) /
Agir, kind `party_leader`) on the `AGIR` observation (`br_tse_fefc_2024_party_07`). The Superior Electoral Court (TSE)
records the party as one registered party renamed twice: PRN to PTC by its decision of 24 April 2001 (PET nº 341) and
PTC to AGIR by its decision of 31 March 2022, taken in the party's 1989 registration case (RPP nº 51-91.1989). Those
renamings are organization claims and the only basis for treating the office as one office across the names; no other
identity is merged, and the observation's funding identity, unknown lifecycle, empty game mapping and funding record are
unchanged. The Partido Democrático Social (PDS, 1980-1993) has no research observation and is **claims only**: the
court glossary's record of its fusion with the PDC into the PPR is recorded, and no primary record naming a PDS
national president of 1990-1993 was found.

The packet adds 14 sources and 21 claims (twenty on the role, one PDS claim), five holder observations
(all of Daniel Tourinho), one role scope note, one coverage note on the AGIR observation and one packet coverage note.
The `br_presidency` institution and its twelve `br_president` and nine `br_vice_president` holders, CLAUDE-C01-22's
eighteen `br_pt_president` holders and CLAUDE-C01-34's seven `br_pdt_president` and five `br_mdb_president` holders are
unchanged. It adds no organization, institution, game mapping, lifespan, portrait or avatar. The parent scope (C01,
C06, S23, WC1 and CP1) remains open.

**At most ten people.** The office had one holder in every record found: Daniel Tourinho (printed 'Daniel Sampaio
Tourinho', 'DANIEL S. TOURINHO'). No other person is named as a holder or acting holder in a claim. The research was
done in one pass by this session alone (no helpers); every single-quoted passage of an HTML source was checked by
script against the decoded capture, and the three scanned PDFs were read visually page by page.

## Outcome

| Observation | Question | Result |
| --- | --- | --- |
| AGIR-PRES-01 | The PRN when the period opens, its registration and the renaming to PTC (1990-2013) | **Claims only:** the court records the PRN's definitive registration (Res.-TSE nº 16.281, 22 February 1990) and the renaming to PTC (24 April 2001); the party's site republished a registry history naming 'o presidente do PRN, o Sr. Daniel Sampaio Tourinho' in 1997, and an undated 2007 roster lists him as President; no contemporaneous dated party record of 1990-2013 naming the office was found |
| AGIR-PRES-02 | The PTC presidency, 2014-2021 | **Accepted:** Daniel Tourinho observed on 16 May 2014, 25 July 2018, 7 August 2020 and 23 July 2021; the convention of 25 July 2015 (no newly elected President identified), the minutes of 3 July 2018 and the convocation of 19 July 2018 are claims |
| AGIR-PRES-03 | The renaming to Agir and the presidency to the cutoff | **Accepted in part:** the party announced the new name on 1 June 2021 and the court approved it on 31 March 2022; Daniel Tourinho observed on 11 November 2022; an undated 2024 styling and the court's live list (captured 2 August 2026) are claims; no dated party record of 2023-2026 naming the office was found |
| PDS-PRES-01 | The PDS, 1990-1993 | **Claims only:** the court glossary records the fusion of the PDS with the PDC into the PPR (Res.-TSE nº 19.133, 8 June 1993); no primary record naming a PDS national president was found (leads only) |

Holders of `br_agir_president`, in order (name, attested_on, from, until):

| # | Holder | attested_on | from | until | Evidence |
| --- | --- | --- | --- | --- | --- |
| 1 | Daniel Tourinho | 2014-05-16 | - | - | PTC site item of that day: 'O Presidente Nacional do PTC, Daniel Tourinho, confirmando o apoio …' |
| 2 | Daniel Tourinho | 2018-07-25 | - | - | Resolução nº 02/2018, signed 'Brasília, 25 de Julho de 2018.' as 'Presidente Nacional PTC' (scan) |
| 3 | Daniel Tourinho | 2020-08-07 | - | - | Resolução nº 001/2020, signed 'Rio de Janeiro, 07 de Agosto de 2020.' as 'Presidente Nacional do PTC' |
| 4 | Daniel Tourinho | 2021-07-23 | - | - | PTC communiqué signed 'Brasília, 23 de julho de 2021' as 'Presidente Nacional' |
| 5 | Daniel Tourinho | 2022-11-11 | - | - | Agir item of that day: 'Daniel Tourinho (Presidente Nacional)', 'o Pres Nacional Daniel Tourinho' |

### How a start and an end are decided

The packet applies the rule of the integrated Brazil packets (CLAUDE-C01-10, -17, -22 and -34). A holder has `from`
only where a source states the day the office was assumed or took effect, and `until` only where a source states the
day it ended. No record found here states either, so no holder has a start or an end. Every holder is dated by
`attested_on` and cites only in-office attestations made on its own day: three signed acts (two resolutions and a
communiqué) and two party items that print no event day and are dated by their own printed date, as a party
newspaper's issue date may (C01-29, C01-31). The test pins that every cited claim carries exactly the holder's date.

The remaining evidence is retained as claims in this bounded packet: the court's registration, renaming and fusion
decisions, the party's renaming statement, conventions and executive meetings, minutes, convocations, undated rosters and listings,
the registry history republished on the party's site and less explicit office stylings. This is a conservative
selection of five observations, not a rule that an event day invalidates an independently supported office title.
No end is inferred from a renaming.

Three stylings could be read as attestations and are kept as claims:

- **'o Presidente' in the minutes of 3 July 2018.** The signed minutes of the national executive name him only as the
  'Presidente' in national-executive context, without an explicit national-office qualifier. That does not establish
  that he was merely a meeting chair; the explicit national styling of 25 July 2018 dates the selected observation.
- **'Presidente do Diretório Nacional' in the convocation of 19 July 2018.** The packet does not equate the President of
  the National Directorate with the national presidency without a ruling; if Codex does, the convocation is a further
  observation of 19 July 2018 and changes no boundary.
- **'presidente do partido' in an undated 2024 Agir item.** The page prints no date and the relative day ('na última
  quarta-feira (31)') cannot be resolved; it names no national office.

The court's decision days (registration, renamings, fusion) are stored as `attested_on` of organization claims, as
CLAUDE-C01-34 stored the court's renaming of the PMDB; they do not supply holder dates or boundaries in this packet.
Registry listings and rosters without a printed date carry no structured date; a capture date is never a holder date.

### Party office, state office and the PDS

The role holds only `br_agir_` claims and sources; the presidency, the PT, PDT and MDB offices never cite them, and the
role never cites theirs (pinned in the test). The renamings are the court's own records of a change of name of one
registered party; they never merge this observation with any other organization, and the PPR, into which the PDS was
fused, is a different organization. The PDS row has a null observation and role and is never cited.

## Date ledger

| Day | Event | Claims |
| --- | --- | --- |
| 1990-02-22 | TSE: definitive registration of the PRN (Res.-TSE nº 16.281) | `br_agir_tse_glossary_prn_definitive_registration_19900222` |
| 1993-06-08 | TSE: fusion of the PDS with the PDC into the PPR (Res.-TSE nº 19.133); claims only | `br_pds_tse_glossary_fusion_into_ppr_19930608` |
| 2001-04-24 | TSE: PRN renamed PTC (PET nº 341; Res.-TSE nº 20.796) | `br_agir_tse_registry_prn_renamed_ptc_20010424`, `br_agir_tse_glossary_prn_renamed_ptc_20010424` |
| 2014-05-16 | PTC item styling the Presidente Nacional (holder 1) | `br_agir_ptc_tourinho_styled_presidente_nacional_20140516` |
| 2015-07-25 | National convention elects a new directorate and executive; styling on the convention day (claims) | `br_agir_ptc_convention_elects_directorate_20150725`, `br_agir_ptc_tourinho_opens_convention_as_presidente_20150725` |
| 2018-07-03 | Minutes of the national executive: 'o Presidente Daniel Tourinho' (claim) | `br_agir_ptc_executive_minutes_presidente_20180703` |
| 2018-07-19 | Convocation signed as 'Presidente do Diretório Nacional' (claim; convention of 28 July 2018 prospective) | `br_agir_ptc_tourinho_convokes_convention_20180719` |
| 2018-07-25 | Resolução nº 02/2018 signed as Presidente Nacional (holder 2) | `br_agir_ptc_tourinho_signs_resolution_02_2018_20180725` |
| 2020-08-07 | Resolução nº 001/2020 signed as Presidente Nacional (holder 3) | `br_agir_ptc_tourinho_signs_resolution_001_2020_20200807` |
| 2021-06-01 | Party statement of the new name AGIR 36 (claim) | `br_agir_ptc_announces_name_change_20210601` |
| 2021-07-23 | Communiqué signed as Presidente Nacional (holder 4) | `br_agir_ptc_tourinho_signs_communique_20210723` |
| 2022-03-31 | TSE: PTC renamed AGIR (RPP nº 51-91.1989) | `br_agir_tse_registry_ptc_renamed_agir_20220331` |
| 2022-11-10 | Executive meeting led by the Presidente Nacional approves resolution 01/2022 (claim; day from 'ontem') | `br_agir_executive_approves_resolution_01_2022_20221110` |
| 2022-11-11 | Agir item styling the Presidente Nacional (holder 5) | `br_agir_tourinho_styled_presidente_nacional_20221111` |
| undated (AGIR-PRES-01) | undated pages, rosters, listings and retrospective text (no structured date) | `br_agir_history_prn_president_requests_statute_adaptation_1997`, `br_agir_history_prn_requests_renaming_ptc_2000`, `br_agir_ptc_roster_tourinho_presidente_capt20071020` |
| undated (AGIR-PRES-03) | undated pages, rosters, listings and retrospective text (no structured date) | `br_agir_tse_registry_lists_tourinho_presidente_nacional`, `br_agir_plenary_tourinho_styled_presidente_do_partido` |

## Observations

### AGIR-PRES-01 — The PRN, its registration and the renaming to PTC (1990-2013)

The court's glossary entry 'Partido Trabalhista Cristão (2001)' records the party's history in its own resolutions:
'Inicialmente denominado Partido da Juventude', renamed PRN by Res.-TSE nº 15.244 of 11 May 1989, 'Registro definitivo:
Res.-TSE nº 16.281, de 22.2.1990', and renamed PTC by Res.-TSE nº 20.796 of 24 April 2001
(`br_agir_tse_glossary_prn_definitive_registration_19900222`, `br_agir_tse_glossary_prn_renamed_ptc_20010424`). The
court's list of name changes gives the same renaming as row 3, PET nº 341, decided 24/04/2001
(`br_agir_tse_registry_prn_renamed_ptc_20010424`). The PTC site's 'História do Partido' (captured October 2007)
reproduces a registry history written in the court's voice: in 1997 'o presidente do PRN, o Sr. Daniel Sampaio
Tourinho' asked for the adaptation of the statute, approved on 9 December 1997, and in 2000 the PRN asked for the new
name (`br_agir_history_prn_president_requests_statute_adaptation_1997`, `br_agir_history_prn_requests_renaming_ptc_2000`);
it is republished reference text, claims only. An undated roster captured 20 October 2007 lists 'PRESIDENTE: DANIEL
SAMPAIO TOURINHO' (`br_agir_ptc_roster_tourinho_presidente_capt20071020`).

Limits: no contemporaneous dated record of the PRN (1990-2000) or of the PTC before 2014 naming the office was found.
The party's statutes filed with the court (1997, 2000 and later) are charters, procedure only, and were not imported.

### AGIR-PRES-02 — The PTC presidency, 2014-2021

A PTC item dated 16 May 2014 says 'O Presidente Nacional do PTC, Daniel Tourinho, confirmando o apoio à candidatura do
senador Aécio Neves à Presidência da República.' (holder 1). The national convention of Saturday 25 July 2015 'elegeu o
novo Diretório Nacional, a nova Comissão Executiva Nacional e aprovou o novo Estatuto Partidário' and was 'aberto pelo
presidente Nacional do Partido, Daniel Tourinho'; the item names no elected President, so both are claims of the
convention day. The signed minutes of the national executive of 3 July 2018 name 'o Presidente Daniel Tourinho'
(claim); the convocation signed on 19 July 2018 by 'Daniel Sampaio Tourinho' as 'Presidente do Diretório Nacional - PTC'
calls the convention of 28 July 2018 (claim; result not found). Resolução nº 02/2018 is signed 'Brasília, 25 de Julho de
2018.' by 'Daniel Sampaio Tourinho Presidente Nacional PTC' (holder 2). Resolução nº 001/2020 closes 'Rio de Janeiro, 07
de Agosto de 2020. Daniel Sampaio Tourinho Presidente Nacional do PTC' (holder 3). A communiqué closes 'Brasília, 23 de
julho de 2021 Daniel Tourinho Presidente Nacional' (holder 4).

### AGIR-PRES-03 — The renaming to Agir and the presidency to the cutoff

A PTC item dated 1 June 2021, 'PTC agora é AGIR 36!', says 'Foi definido, no último final de semana, a mudança de nome
do Partido Trabalhista Cristão (PTC) para AGIR 36' (`br_agir_ptc_announces_name_change_20210601`); the weekend meant is
not resolved. The court's list of name changes gives row 18, PTC to 'AGIR', 'RPP nº 51-91.1989.6.00.0000', decided
'31/03/2022' (`br_agir_tse_registry_ptc_renamed_agir_20220331`). An Agir item of 11 November 2022 names 'Daniel
Tourinho (Presidente Nacional)' and 'o Pres Nacional Daniel Tourinho' (holder 5) and reports that the executive meeting
'Liderada pelo Presidente Nacional Daniel Tourinho' approved resolution 01/2022 'ontem', resolved from the dateline to
Thursday 10 November 2022 (a meeting claim). An undated Agir item captured 1 March 2024 says 'O presidente do partido,
Daniel Tourinho, elogiou a atuação brilhante de Bria' (claim). The court's live list of registered parties, captured 2
August 2026, gives 'AGIR', 'DEFERIMENTO' '22.2.1990' and 'PRES. NACIONAL' 'DANIEL S. TOURINHO'
(`br_agir_tse_registry_lists_tourinho_presidente_nacional`); a capture date is never a holder date.

Limits: no dated party record of 2023-2026 naming the national office was found. The party's site moved to a
JavaScript site builder in 2024, whose captures carry no dated text naming the office; the 2025 items found are leads.

### PDS-PRES-01 — The PDS, 1990-1993 (claims only)

The court glossary's entry 'Partido Democrático Social (1980)' says 'Fundiu-se com o Partido Democrata Cristão (PDC),
passando a denominar-se Partido Progressista Reformador (PPR) (Res.-TSE nº 19.133, de 8.6.1993)'
(`br_pds_tse_glossary_fusion_into_ppr_19930608`), a claim with no observation, role or holder. No primary record naming
a PDS national president of 1990-1993 was found: the party had no web presence, the court's party pages do not list
the presidents of extinct parties, and the Chamber and Senate biographies read as leads do not print the office. The
encyclopaedia and history leads (Paulo Maluf as President in the early 1990s) are not imported.

## Sources added

| Source | Title | Observation | Identity |
| --- | --- | --- | --- |
| `br_agir_tse_partidos_registrados_capt20260802` | Partidos políticos registrados no TSE (TSE list of registered parties, with its table of name changes) | AGIR-PRES-01, AGIR-PRES-03 | 36,548 bytes, `63222b4c…71bd6a`; raw capture 20260802212048 |
| `br_agir_tse_glossario_partidos_capt20230325` | Partido político (TSE glossary of parties, with registration and name-change resolutions) | AGIR-PRES-01, PDS-PRES-01 | 127,257 bytes, `72ec97fd…bb6ee8`; raw capture 20230325042738 |
| `br_agir_ptc_historia_partido_capt20071011` | História do Partido (PTC site, Partido Trabalhista Cristão - PTC (Antigo PRN)) | AGIR-PRES-01 | 27,308 bytes, `05cd95f5…9b2345`; raw capture 20071011184316 |
| `br_agir_ptc_comissao_executiva_capt20071020` | Comissão Executiva Nacional (PTC site roster) | AGIR-PRES-01 | 23,214 bytes, `2c15b81c…c40d05`; raw capture 20071020032949 |
| `br_agir_ptc_apoio_aecio_20140516` | O Presidente Nacional do PTC Daniel Tourinho confirma apoio à candidatura de Aécio Neves (PTC site, 16 May 2014) | AGIR-PRES-02 | 20,490 bytes, `68b1d9a1…a6e3af`; raw capture 20141008035057 |
| `br_agir_ptc_convencao_aracaju_20150728` | PTC realiza Convenção Nacional em Aracaju (PTC site, 28 July 2015) | AGIR-PRES-02 | 28,704 bytes, `3594488d…c4105d`; raw capture 20160829223324 |
| `br_agir_ptc_ata_executiva_20180703` | Ata da Comissão Executiva Nacional do Partido Trabalhista Cristão, 3 July 2018 (signed minutes, scanned PDF) | AGIR-PRES-02 | 1,130,004 bytes, `a93a97cd…6431fb`; raw capture 20181220141003 |
| `br_agir_ptc_edital_convencao_20180719` | Edital de Convocação, Partido Trabalhista Cristão, 19 July 2018 (scanned PDF) | AGIR-PRES-02 | 322,372 bytes, `fcd6ba94…367c4d`; raw capture 20181220140928 |
| `br_agir_ptc_resolucao_02_2018_20180725` | Resolução nº 02/2018, Partido Trabalhista Cristão, 25 July 2018 (scanned PDF) | AGIR-PRES-02 | 336,809 bytes, `2bf6b2aa…976624`; raw capture 20181220140943 |
| `br_agir_ptc_resolucao_001_2020_20200807` | Critério de distribuição do Fundo Especial de Financiamento de Campanha Eleições 2020 (PTC site, 20 August 2020, with Resolução nº 001/2020 of 7 August 2020) | AGIR-PRES-02 | 30,056 bytes, `448fc9f5…44ea34`; raw capture 20200928205237 |
| `br_agir_ptc_agora_agir36_20210601` | PTC agora é AGIR 36! (PTC site, 1 June 2021) | AGIR-PRES-03 | 37,384 bytes, `29ee50b6…101b85`; raw capture 20210602153422 |
| `br_agir_ptc_comunicado_20210723` | Comunicado (PTC site, 23 July 2021) | AGIR-PRES-02 | 27,090 bytes, `cc13f256…0de9cf`; raw capture 20210728155528 |
| `br_agir_executiva_resolucao_01_2022_20221111` | Comissão Executiva Nacional aprova novas regras partidárias para as eleições de 2024 (Agir site, 11 November 2022) | AGIR-PRES-03 | 14,315 bytes, `f252cd32…d29bf8`; raw capture 20230125002213 |
| `br_agir_plenaria_nacional_capt20240301` | AGIR36 consolida compromisso histórico na defesa dos direitos dos autistas em Plenária Nacional (Agir site, undated) | AGIR-PRES-03 | 127,999 bytes, `406dbbe2…1d01ac`; raw capture 20240301164758 |

## Response identities and stability checks

Every source is a raw Internet Archive capture (id_ form) made before 7 September 2026. Each was downloaded at least
twice from the exact recorded URL with curl, the request header Accept-Encoding: identity, no decoding and no redirects
followed, the first and last downloads at least 30 minutes apart, with identical byte counts and SHA-256; the times are
in each extract's `stability_check`. Two captures are served gzip-encoded even to an identity request; for them the
recorded identity is the encoded body as served and the decoded identity is recorded beside it.

| Source | Bytes | SHA-256 | Encoding | Downloads (UTC) |
| --- | --- | --- | --- | --- |
| `br_agir_tse_partidos_registrados_capt20260802` | 36,548 | `63222b4c603a9653a194e1d21930c543d60b7feb072e91a50515f0436b71bd6a` | gzip (decoded 148,444 bytes, `e2abefa96b04…`) | at 2026-10-01T03:14:17Z and again at 2026-10-01T03:50:12Z |
| `br_agir_tse_glossario_partidos_capt20230325` | 127,257 | `72ec97fd5fe4906d8c8d95f7e134034b211f0da2245adce780e45f5d2dbb6ee8` | identity | at 2026-10-01T03:13:43Z and again at 2026-10-01T03:50:18Z |
| `br_agir_ptc_historia_partido_capt20071011` | 27,308 | `05cd95f56e424e28fccd04f48649a8ede9652666a7bde970f333c8488b9b2345` | identity | at 2026-10-01T02:32:28Z and again at 2026-10-01T03:50:24Z |
| `br_agir_ptc_comissao_executiva_capt20071020` | 23,214 | `2c15b81cab612c24d3ac91e377d2e80e62995952c25818a2ba2e12c38bc40d05` | identity | at 2026-10-01T02:32:21Z and again at 2026-10-01T03:50:30Z |
| `br_agir_ptc_apoio_aecio_20140516` | 20,490 | `68b1d9a12847fc0242912fd9b045a55d98ef78564ffadc0575911e1c71a6e3af` | identity | at 2026-10-01T02:36:04Z and again at 2026-10-01T03:50:36Z |
| `br_agir_ptc_convencao_aracaju_20150728` | 28,704 | `3594488d8e31ac9bc4a3ebc41ed17f04dfd9c008ad249f2e1637a9cd00c4105d` | identity | at 2026-10-01T02:36:11Z and again at 2026-10-01T03:50:41Z |
| `br_agir_ptc_ata_executiva_20180703` | 1,130,004 | `a93a97cdc04946e756b8d2d8c719de4b82389c97c5ded1b0f4260990ec6431fb` | identity | at 2026-10-01T03:11:00Z and again at 2026-10-01T03:50:48Z |
| `br_agir_ptc_edital_convencao_20180719` | 322,372 | `fcd6ba94e8a75b5dbd101fc5ce900d66e287c13c49a0546e66fc6956a1367c4d` | identity | at 2026-10-01T03:10:54Z and again at 2026-10-01T03:50:55Z |
| `br_agir_ptc_resolucao_02_2018_20180725` | 336,809 | `2bf6b2aaa64f1524e104482e0218bf9c2c64f49fb150aaf48c590a121e976624` | identity | at 2026-10-01T03:11:06Z and again at 2026-10-01T03:51:01Z |
| `br_agir_ptc_resolucao_001_2020_20200807` | 30,056 | `448fc9f5c5252d5399015e41ed55131c119421219d2b6f47523dc188db44ea34` | identity | at 2026-10-01T02:45:04Z and again at 2026-10-01T03:51:06Z |
| `br_agir_ptc_agora_agir36_20210601` | 37,384 | `29ee50b675f1ba83a4840956b25a3a509452ee2beb437e44b902d6a81c101b85` | identity | at 2026-10-01T02:45:23Z and again at 2026-10-01T03:51:12Z |
| `br_agir_ptc_comunicado_20210723` | 27,090 | `cc13f2566449dad458244b5c4094ff99c7ba9eea419d92b6d1b8c4b9e80de9cf` | identity | at 2026-10-01T02:46:10Z and again at 2026-10-01T03:51:21Z |
| `br_agir_executiva_resolucao_01_2022_20221111` | 14,315 | `f252cd32737f39cd9fdd0c2330433f499c1eeb8191cf650a6ad2b0f7a5d29bf8` | gzip (decoded 77,862 bytes, `80d14ef5acd3…`) | at 2026-10-01T02:46:20Z and again at 2026-10-01T03:51:26Z |
| `br_agir_plenaria_nacional_capt20240301` | 127,999 | `406dbbe231109e96d2f1af44c753de35295a6f2d37ec86efa8dbccea7d1d01ac` | identity | at 2026-10-01T02:46:46Z and again at 2026-10-01T03:51:32Z |

## Leads not imported

- Encyclopaedias and reference sites: https://pt.wikipedia.org/wiki/Agir_(Brasil), https://pt.wikipedia.org/wiki/Daniel_Tourinho,
  https://pt.wikipedia.org/wiki/Partido_Democr%C3%A1tico_Social (Paulo Maluf as PDS President), https://atlas.fgv.br/verbete/6099 and
  https://atlas.fgv.br/verbete/3217 (PDS and Paulo Maluf; no PDS presidency dates), https://www.politize.com.br/agir/.
- News: https://www.atribunarj.com.br/materia/vamos-ver-se-vai-ter-eleicao-diz-daniel-tourinho-presidente-nacional-do-agir (an
  interview styling him national President), https://www.metropoles.com/sao-paulo/chefe-agir-doacoes-partido, and the
  Manaus municipal council item https://www.cmm.am.gov.br/joao-paulo-janjao-fortalece-dialogo-com-presidente-nacional-do-agir-em-visita-a-sede-do-partido/
  (a 2025 visit to the 'presidente nacional do Agir'; a third party's press item, not a party record), and blog reposts
  of the renaming (luiscardoso.com.br, contraponto.jor.br).
- The TSE's own news item of 31 March 2022,
  https://www.tse.jus.br/comunicacao/noticias/2022/Marco/tse-aprova-alteracao-e-partido-trabalhista-cristao-passa-a-se-chamar-agir:
  www.tse.jus.br refuses curl (Akamai 403), and its archive captures before the cutoff (22 July and 2 September 2022)
  replay the same 403 page; the court's list of name changes is used instead.
- The court's party page https://www.tse.jus.br/partidos/partidos-registrados-no-tse/agir: captures of 2022 and 2024
  replay a 403 page. Older captures of the PTC page (2011, 2017, 2021) list 'PRES. NACIONAL' / 'Presidente Nacional:
  Daniel S. Tourinho' without a date and repeat the registry list; not imported.
- The court's SGIP registry, https://sgip3.tse.jus.br/sgip3-consulta/ and its organ endpoints: on 1 October 2026 every
  request returned the page 'Indisponibilidade de sistema' (unavailable during the election period), so no national
  organ of the PRN, PTC or Agir was read and no registry body was kept.
- The TRE-RO party page (https://www.tre-ro.jus.br/partidos/partidos-politicos/partido-trabalhista-cristao, captures of
  December 2021 and May 2024), which repeats the court's listing and statute dates.
- Party documents found but not imported: the minutes of the 2022 fund resolution (images, 'o Presidente Daniel
  Tourinho' only), the invitation signed 'Daniel Tourinho Presidente Nacional do PTC' on an item of 14 April 2021 and the
  styling 'Presidente Nacional do AGIR36' of 10 September 2021 (continuations of the observations kept), the 2006 site
  news items (unrelated), the 2013 and 2016 'Diretório Nacional' rosters (undated, duplicating the 2007 roster) and the
  party's statutes filed with the court (charters, procedure only).

## Sources attempted

- Internet Archive CDX listings of ptc.org.br, ptc36nacional.com.br, agir36.com.br and the court's party pages
  (discovery only; the CDX service returned HTTP 429 and 503 repeatedly and was retried slowly, never worked around).
- The Agir site's 2024-2025 captures (site-builder pages: leaders, history, news list, a post of 21 June 2024 on a
  meeting with a governor): no dated text naming the office. Two 2024 PDF captures are truncated at 1,048,576 bytes and
  two others are a membership form and an image file.
- The Agir resolution 01/2022 PDF (no capture: 404).
- The SGIP registry (unavailable, see above) and www.tse.jus.br live pages (Akamai 403).

## Suggested next work orders

1. When the court's SGIP registry is available again after the 2026 elections, read the closed national organs of the
   PRN / PTC / Agir (registered terms and the President's registered exercise periods; claims only, personal fields
   hashed and never copied).
2. Look for contemporaneous PRN records of 1990-2000 (the Chamber's and Congress's diaries for the party's notes on its
   bench leadership in 1991, the full text of Res.-TSE nº 20.044 of 9 December 1997 and nº 20.796 of 24 April 2001) to
   date the office in the PRN era.
3. Look for a dated Agir record of 2023-2026 naming the national office (convention editais, resolutions, signed notes).
4. A separate packet for the PDS (1980-1993) and the PPR, with a research observation, if a primary record naming a PDS
   national President can be found.

## Integration notes (outside this packet's file boundary)

- **Roadmap order (author report).** The author described this batch as a user-prioritized addition. That assertion
  is not an instruction or an acceptance ruling. This packet is new research taken from the gap ledger items
  `Brazil/br_prn` and `Brazil/br_pds`, not a continuation of an existing claim; the integrator owns task registration.
- **Research index.** `docs/campaign-certification/C01/research-index.json` is regenerated in its own commit
  (`Regenerate the C01 research index for CLAUDE-C01-43`). It is the only file shared with the parallel packets
  (C01-38 to C01-41 in fixes, C01-42 to C01-46 in research); on integration, regenerate it rather than merging it.
- **Pinned tests.** Five Brazil tests pin totals that this packet changes; each is re-expressed exactly, none loosened:
  `test_brazil_research_s10f.py`, `test_brazil_presidents_c01_10.py`, `test_brazil_vice_presidents_c01_17.py`,
  `test_brazil_pt_presidents_c01_22.py` and `test_brazil_party_presidents_c01_34.py` (entries, sources, claims and roles
  (32, 262, 556, 6); index role observations and claims (6, 556); the exact list of party roles now ends with
  `br_agir_president`; the exact source order now ends with this packet's fourteen sources).
- **Gap ledger.** `test_certified_gap_ledger.py` reports 'no pinned attribution' for this packet until Codex classifies
  its commit; `docs/campaign-certification/C01/gap-ledger/` is not touched. `test_certified_boundary_matrix.py` (S23)
  needs `spheres-web/src`, which the sparse checkout lacks; Codex regenerates the matrix on integration.
- **SGIP outage.** The court's SGIP consultation service returned its election-period outage page on 1 October 2026;
  the twelve SGIP sources recorded by CLAUDE-C01-34 cannot be re-downloaded until it returns.
- **Census.** `campaign_census.py --check` passes on the unchanged base `509bd289` and with this packet, so there is nothing to disclose about `spheres-sim/src/government.rs` and `census.json` for this packet.

## Checks

Run in the worktree on 1 October 2026 (UTC) after merging the moved integration `229df210` (`803b2627`) and before
committing, at least 30 minutes after the first downloads:

- `python tools/avatars/campaign_research.py` then `--check`: pass (9 country packets, 1,985 sources, 4,897 claims).
- `python tools/avatars/campaign_census.py --check`: pass, on the unchanged base `509bd289` (claim commit `6e6a9d06`)
  and again with this packet on the merged integration; nothing to disclose.
- `python -m unittest discover -s tools/avatars -p 'test_brazil*.py'`: 51 tests OK (the new
  `test_brazil_prn_agir_presidents_c01_43.py` and the five re-expressed Brazil tests).
- `-p 'test_*research*.py'`: 79 tests OK; `-p 'test_campaign*.py'`: 16 tests OK.
- `node --test tools/ui/check_leadership_research_review.cjs`: pass (0 failures).
- `python tools/planning/workboard.py --check`: PASS. After the merge it first reported 'Missing task handoff:
  docs/campaign-certification/S26/preparation/RECRUITMENT.md' only because the sparse checkout omitted
  `docs/campaign-certification/S26` (the file is in the merged tree); that directory was added to this worktree's sparse
  checkout and the check passes.
- `git diff --check`: clean.
- Every single-quoted passage of an HTML source claim (62 quotations) was checked by script against the decoded
  capture; the three scanned PDFs were read visually.
- Known failures outside the suite, not fixed here: `test_certified_gap_ledger.py` (setUpClass: 'Source
  br_agir_tse_partidos_registrados_capt20260802 (Brazil) has no pinned attribution', until Codex classifies this
  packet's commit) and `test_certified_boundary_matrix.py` (S23; 'Required input is missing:
  spheres-web/src/person_avatar_assets.rs' in the sparse checkout).
- `D:/spheres-scratch/c01-pipeline/tools/packet_check.py 43` re-downloads every recorded response and reruns this
  suite after the push; its summary is reported with the handoff.
