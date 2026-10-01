# USSR CPSU General Secretary 41: the General Secretary and the Deputy General Secretary of the CPSU Central Committee, 1990-1991

Packet: **CLAUDE-C01-41**. State: **ready_for_review** (not complete).
Owner: Claude. Integrator/reviewer: Codex. Branch `claude/c01-su-41`; claim commit `7778be5b` on `02d2c5a2`, the head of
`codex/campaign-certification` when the packet was claimed; **not stacked** on any pending packet. Research access: 30 September
2026 (UTC; the second download pass ran after midnight UTC, on 1 October, still 30 September local, UTC-7). The historical cutoff
stays **7 September 2026**.

This packet reviews six observations, SU-CPSU-01 to SU-CPSU-06, of the existing organization `su_cpsu` in [ussr.json](ussr.json).
It dates the two existing roles, `su_cpsu_general_secretary` ("General Secretary", kind `party_leader`) and
`su_cpsu_deputy_general_secretary` ("Deputy General Secretary", kind `other`), from 1 January 1990 to the party's end, and adds:

- three dated holder observations of Михаил Сергеевич Горбачев as General Secretary (5 February 1990, 10 July 1990, 22 August
  1991) and two of Владимир Антонович Ивашко as Deputy General Secretary (13 July 1990, 21 August 1991), each with `attested_on`
  only;
- 12 sources and 26 claims, each source with a checked-in derived extract, a scope note on each role and four coverage items on
  `su_cpsu`, and one packet coverage item.

The S10.h month observations of July 1990 (Japan's diplomatic account: "Mikhail Gorbachev" and "Vladimir Ivashkov (source
spelling; identity reconciliation pending)") are unchanged and stay first in each role; the new observations follow in
chronological order. No existing source, claim, extract, holder, role title or kind, or lifecycle is edited; `russia.json` is not
touched; no organization, institution, game mapping, portrait or avatar is added. No holder has a `from` or an `until`. The parent
scope (C01, C06, S23, WC1 and CP1) remains open.

## Outcome

| ID | Question | Decision |
|---|---|---|
| SU-CPSU-01 | The General Secretary in 1990 before the XXVIII Congress | **Accepted:** the Central Committee's communique on the plenum opened 5 Feb 1990 names "Генеральный секретарь ЦК КПСС М. С. Горбачев" as rapporteur (Pravda No. 37); holder `attested_on` 5 Feb 1990 |
| SU-CPSU-02 | The XXVIII Congress election of the General Secretary | **Accepted:** the ballot (Т. Г. Авалиани and М. С. Горбачев), the vote, the result announced and the counting commission's protocol approved, all on 10 Jul 1990, and the elected General Secretary's address (Pravda No. 192); holder `attested_on` 10 Jul 1990; the biography's "вновь избран" is a claim. No `from`: a re-election, and no source states when a term took effect |
| SU-CPSU-03 | The new office of Deputy General Secretary: election and first attestation | **Accepted:** the ballot (А. С. Дудырев, В. А. Ивашко, Е. К. Лигачев) and the vote, 11 Jul 1990 (Pravda No. 193); the result announced and the protocols approved, 12 Jul 1990, with the vote figures in the paper's own report and a biography dating the election to 11 Jul (Pravda No. 194); the Congress's programme commission, formed 13 Jul 1990, lists "Ивашко В. А. — заместитель Генерального секретаря ЦК КПСС" and "Горбачев М. С. — Генеральный секретарь ЦК КПСС" (Pravda No. 195); holder `attested_on` 13 Jul 1990. No `from` |
| SU-CPSU-04 | The latest attestations before the office changed | **Accepted in part:** the Central Committee journal's editorial board names the deputy (signed to press 10 Jul 1991, a claim); the Secretariat's undated statement in Pravda of 22 Aug 1991 names the General Secretary (holder `attested_on` 22 Aug 1991); a TASS report reprinted by Pravda names both at a Central Committee secretary's press conference on 21 Aug 1991 (the deputy's holder `attested_on` 21 Aug 1991; ruling requested) |
| SU-CPSU-05 | The General Secretary's resignation and the deputy's acting service | **Accepted in part:** his own words in the Supreme Soviet on 26 Aug 1991 that he has laid down the duties (no day stated) and another deputy's reference on 3 Sep 1991 are claims; the statement of 24 Aug 1991 itself is known only from leads, so no `until`. The deputy's acting service after 24 Aug 1991 was not found in a primary record (leads only) |
| SU-CPSU-06 | The party's suspension and end | **Accepted in part:** decree УП-2460 on the party's property (24 Aug 1991), the proposal of self-dissolution (26 Aug), the Supreme Soviet's suspension of the party's activity in the USSR (2371-I, 29 Aug), a deputy's statement that the Central Committee has not decided to dissolve itself (3 Sep) and the RSFSR decree ending its activity on RSFSR territory (No. 169, 6 Nov 1991) are organization claims only; no union-level dissolution or lifecycle end is established |

### Holders

`su_cpsu_general_secretary` (existing role; the S10.h observation stays first):

| # | Name | `attested_on` | `from` | `until` | Claim |
|---|---|---|---|---|---|
| 0 | Mikhail Gorbachev (S10.h, July 1990 month window, unchanged) | - | null | null | `su_mofa_gorbachev_general_secretary_199007` |
| 1 | Михаил Сергеевич Горбачев | 1990-02-05 | null | null | `su_pravda37_plenum_report_by_general_secretary_19900205` |
| 2 | Михаил Сергеевич Горбачев | 1990-07-10 | null | null | `su_pravda192_gorbachev_addresses_as_general_secretary_19900710` |
| 3 | Михаил Сергеевич Горбачев | 1991-08-22 | null | null | `su_pravda201_secretariat_statement_names_general_secretary_19910822` |

`su_cpsu_deputy_general_secretary` (existing role; the S10.h observation stays first):

| # | Name | `attested_on` | `from` | `until` | Claim |
|---|---|---|---|---|---|
| 0 | Vladimir Ivashkov (source spelling; identity reconciliation pending) (S10.h, July 1990, unchanged) | - | null | null | `su_mofa_deputy_general_secretary_199007` |
| 1 | Владимир Антонович Ивашко | 1990-07-13 | null | null | `su_pravda195_program_commission_lists_deputy_general_secretary_19900713` |
| 2 | Владимир Антонович Ивашко | 1991-08-21 | null | null | `su_pravda201_ivashko_deputy_general_secretary_flew_to_crimea_19910821` |

### Starts and ends

`from` would need a source stating the day office was assumed or took effect, and `until` one stating the day it ended; none
does. The General Secretary's election of 10 July 1990 re-elected the sitting holder and states no term; the deputy was voted on 11
July and his election was announced and approved on 12 July 1990, and no source says which day, if either, he took office. The
General Secretary's own words of 26 August 1991 ("я сложил с себя обязанности Генерального секретаря ЦК КПСС") state neither the
day of his statement nor an effective day, and the dated statement of 24 August 1991 was found only in leads (a textbook reader
citing Российская газета of 27 August 1991; the Gorbachev Foundation's chronicle), so the ruling for resignations (C01-36, C01-37)
does not apply. No end is inferred from the party's suspension or from the RSFSR decree. The new holders are placed after the
unchanged S10.h observations, which carry a month window and no `attested_on`, so that the existing holders keep their position.

### Name forms and identities

Holders are named as this packet's sources print them in full: Pravda's biographical notes headed "ГОРБАЧЕВ" over "Михаил Сергеевич"
(11 July 1990) and "ИВАШКО" over "Владимир Антонович" (13 July 1990), and "Ивашко Владимира Антоновича" in that day's report (the CLAUDE-C01-19
convention); the records themselves print "М. С. Горбачев", "Горбачев М. С.", "В. А. Ивашко", "Ивашко В. А." and "В. Ивашко". The
S10.h deputy's source spelling "Vladimir Ivashkov" is **not** reconciled with "Владимир Антонович Ивашко": the two stay separate
observations, and the identity question stays in the role's coverage. The CPSU and the RSFSR Communist Party (КП РСФСР, named with
it in the RSFSR decree) are not treated as one identity.

### Party office and state office

The two roles are party offices. Records that name Gorbachev as "Генеральный секретарь ЦК КПСС, Президент СССР" feed only the
party role; no claim or source of this packet feeds `su_president`, `su_supreme_soviet_chair`, `su_government_head` or
`su_congress_deputies`, and no state claim feeds the CPSU roles. The Union and RSFSR state acts on the party (УП-2460, 2371-I, RSFSR
decree No. 169) are organization claims on `su_cpsu`, never holders or lifecycle bounds. Nothing is added to `russia.json`. The
test pins both directions and that every other USSR role and holder is unchanged.

### Date ledger

Each row is a separate dated fact with its own claim; two facts on one day stay two claims.

| Date | Events | Holder field and claims |
|---|---|---|
| 5 Feb 1990 | plenum opens; General Secretary reports | GS holder 1 `attested_on`; `su_pravda37_plenum_report_by_general_secretary_19900205` |
| 10 Jul 1990 | ballot list; vote; result and protocol approved; address as General Secretary; biography "вновь избран" | GS holder 2 `attested_on`; `su_pravda192_general_secretary_ballot_19900710`, `su_pravda192_general_secretary_vote_held_19900710`, `su_pravda192_gorbachev_elected_general_secretary_19900710`, `su_pravda192_gorbachev_addresses_as_general_secretary_19900710`, `su_pravda192_biography_reelected_general_secretary_19900710` |
| 11 Jul 1990 | General Secretary presides; deputy ballot list; deputy vote; biography's election day | `su_pravda193_gorbachev_presides_as_general_secretary_19900711`, `su_pravda193_deputy_general_secretary_ballot_19900711`, `su_pravda193_deputy_general_secretary_vote_held_19900711`, `su_pravda194_biography_elected_deputy_general_secretary_19900711` |
| 12 Jul 1990 | deputy's election announced, protocols approved; vote figures reported | `su_pravda194_ivashko_elected_deputy_general_secretary_19900712`, `su_pravda194_deputy_general_secretary_vote_figures_19900712` |
| 13 Jul 1990 | programme commission formed, listing both officers | DGS holder 1 `attested_on`; `su_pravda195_program_commission_chair_general_secretary_19900713`, `su_pravda195_program_commission_lists_deputy_general_secretary_19900713` |
| 10 Jul 1991 | journal signed to press with the deputy on its board | `su_izv_tsk_1991_08_board_deputy_general_secretary_19910710` |
| 21 Aug 1991 | press conference: plenum with the General Secretary; the deputy flies to the Crimea | DGS holder 2 `attested_on`; `su_pravda201_dzasokhov_refers_to_general_secretary_19910821`, `su_pravda201_ivashko_deputy_general_secretary_flew_to_crimea_19910821` |
| 22 Aug 1991 | the Secretariat's statement (undated; the issue's day) | GS holder 3 `attested_on`; `su_pravda201_secretariat_statement_names_general_secretary_19910822` |
| 24 Aug 1991 | decree УП-2460 on the party's property | `su_ukaz_up2460_cpsu_property_19910824` |
| 26 Aug 1991 | his own words that he has laid down the duties; proposal of self-dissolution | `su_vs26_gorbachev_laid_down_general_secretary_duties_19910826`, `su_vs26_gorbachev_proposed_cc_self_dissolution_19910826` |
| 29 Aug 1991 | the party's activity suspended in the USSR (2371-I, point 7) | `su_vs_2371i_cpsu_activity_suspended_19910829` |
| 3 Sep 1991 | a deputy refers to the resignation; says the Central Committee has not decided to dissolve itself | `su_snd5_medvedev_refers_to_general_secretary_resignation_19910903`, `su_snd5_medvedev_cc_never_decided_self_dissolution_19910903` |
| 6 Nov 1991 | RSFSR decree No. 169: activity ended and structures dissolved on RSFSR territory; property to the state | `su_rsfsr_ukaz_169_cpsu_activity_terminated_19911106`, `su_rsfsr_ukaz_169_cpsu_property_to_state_19911106` |

## Observations

### SU-CPSU-01 — The General Secretary in 1990 before the Congress

Pravda No. 37 (26120) of 6 February 1990, "Орган Центрального Комитета КПСС", prints the Central Committee's communique: "5
февраля 1990 года начал работу очередной Пленум Центрального Комитета КПСС" and "С докладом по этому вопросу на Пленуме выступил
Генеральный секретарь ЦК КПСС М. С. Горбачев." Holder 1, `attested_on` 5 February 1990: the earliest reviewed attestation in the
period, not a start of office. Decision: **accepted**.

### SU-CPSU-02 — The XXVIII Congress election of the General Secretary

The communique on the sitting of 10 July 1990 (Pravda No. 192, 11 July 1990) records the nomination and ballot list ("Т. Г.
Авалиани и М. С. Горбачев"), the counting commission, the vote, the result ("Генеральным секретарем Центрального Комитета
Коммунистической партии Советского Союза избран М. С. Горбачев. Делегаты утвердили протокол счетной комиссии.") and the address
of the "Генеральный секретарь ЦК КПСС, Президент СССР М. С. Горбачев". The biographical note beside it states "10 июля 1990 года на
XXVIII съезде КПСС вновь избран Генеральным секретарем ЦК КПСС". Holder 2, `attested_on` 10 July 1990, rests on the address, not
on the election. Intermediate attestations (presiding on 11 July; chairing the programme commission on 13 July) are claims. The
counting commission's protocol No. 2 and the vote figures are printed only in a transcription of the stenographic report on a
private host (a lead). Decision: **accepted**.

### SU-CPSU-03 — The Deputy General Secretary

The communique on 11 July 1990 (Pravda No. 193) records the nominations for the new office and the ballot list ("А. С. Дудырев, В.
А. Ивашко, Е. К. Лигачев") and the vote at the day's last sitting. The communique on 12 July 1990 (Pravda No. 194) records the
counting commission's report, "Большинством голосов Заместителем Генерального секретаря ЦК КПСС, членом ЦК КПСС избран В. А.
Ивашко", and the protocols approved; the paper's own report gives the figures (3.109 for, 1.309 against; reportage, a claim) and
says the result was of the elections held "накануне"; the biography dates the election to 11 July. The Congress's programme
commission, formed on 13 July 1990 according to that day's communique, lists "Ивашко В. А. — заместитель Генерального секретаря
ЦК КПСС" (Pravda No. 195, p. 2): holder 1, `attested_on` 13 July 1990. Decision: **accepted**; the two election days (vote 11
July, announcement 12 July) are both kept and neither sets `from`.

### SU-CPSU-04 — The latest attestations

Известия ЦК КПСС No. 8 (319), "август 1991", signed to press "10.07.91", lists "В. А. ИВАШКО, заместитель Генерального секретаря
ЦК КПСС" first on its editorial board (a claim, dated by the imprint). Pravda No. 201 of 22 August 1991 prints the Secretariat's
statement, signed "Секретариат ЦК КПСС" and undated, calling for a plenum "с непременным участием Генерального секретаря ЦК КПСС
М. С. Горбачева": General Secretary holder 3, `attested_on` 22 August 1991, the issue's day. The same issue reports that on 21
August 1991 the Central Committee secretary А. Дзасохов said the Secretariat wanted a plenum "с участием Генерального секретаря ЦК
М. С. Горбачева" (a claim) and that "заместитель Генерального секретаря ЦК В. Ивашко вылетел в Крым": deputy holder 2,
`attested_on` 21 August 1991. Decision: **accepted in part**; whether a party organ's report of a Central Committee secretary's
words attests the office is for the integrator (without it, the deputy's latest observation is 13 July 1990 and the journal's
masthead of 10 July 1991 remains a claim).

### SU-CPSU-05 — The resignation and the deputy's acting service

In his report to the Supreme Soviet on 26 August 1991 (bulletin No. 1, printed pp. 31-34), "Горбачев М. С., Президент СССР" says
"Вы знакомы с моим заявлением, в котором я сложил с себя обязанности Генерального секретаря ЦК КПСС и предложил Центральному
Комитету партии самораспуститься" and later "я сложил с себя обязанности Генерального секретаря". On 3 September 1991 the deputy Р.
А. Медведев refers to "отставка товарища Горбачева с поста Генерального секретаря ЦК КПСС". Both are claims and state no day. The
statement of 24 August 1991 ("слагаю соответствующие полномочия" in the leads) was not found in a primary record. The deputy's
acting service as General Secretary from 24 August 1991 is stated only in encyclopaedias; no primary record of it, of a Central
Committee decision on it or of the end of his office was found. Decision: **accepted in part**; no `until`, no acting holder.

### SU-CPSU-06 — The party's suspension and end

Decree УП-2460 of 24 August 1991 (Ведомости 1991 No. 35, Art. 1024) puts the party's property under the Soviets' protection and
provides for staff of "тех партийных комитетов, которые прекращают свою деятельность". On 26 August the General Secretary says he
proposed the Central Committee's self-dissolution. The Supreme Soviet's resolution 2371-I of 29 August 1991, point 7 (Ведомости No.
36, Art. 1038), resolves to "приостановить деятельность КПСС на всей территории СССР". On 3 September Р. А. Медведев states "что
Центральный Комитет никогда не принимал (и я надеюсь не примет) решение о самороспуске". RSFSR decree No. 169 of 6 November 1991
orders "Прекратить на территории РСФСР деятельность КПСС, КП РСФСР, а их организационные структуры распустить" and transfers their
property on RSFSR territory to the state. Decision: **accepted in part**: organization claims only; the lifecycle stays
`documented_at_specific_observations` with no `until`; the RSFSR decree is limited to one republic's territory, and the 1992
Constitutional Court case is after the period.

## Sources added

12 sources, each with a checked-in derived factual extract under [sources/](sources/) (`ussr-*-facts.json`, LF, format
`spheres-c01-derived-factual-table/v1`, with its own checksum in the packet). Each extract records the recorded response's URL,
byte count and SHA-256, both of this packet's downloads, the attached live or original-edition response where there is one, the
edition and host, the pages read and one row per claim (claim_id, observation, role, `holder_name` or null, `persons_named`, role
title, printed title, event kind, date, text, locator). Original pages, PDFs and HTML are not checked in; no photograph, emblem or
signature image is republished.

| Source ID | What | Response identity (bytes, SHA-256) | Claims |
|---|---|---|---|
| `su_pravda_no37_19900206` | [Правда (Орган Центрального Комитета КПСС), № 37 (26120), Вторник, 6 февраля 1990 года: ИНФОРМАЦИОННОЕ СООБЩЕНИЕ о Пленуме Центрального Комитета Коммунистической партии Советского Союза (5 February 1990); page-image scan, Internet Archive stored file](https://ia600709.us.archive.org/2/items/199037_9972/%D0%9F%D1%80%D0%B0%D0%B2%D0%B4%D0%B0%2C%201990%20%2C%20%E2%84%96%2037.pdf) | IA item `199037_9972` (stored file, SHA-1 `d1251189…`), 8,135,736, `537821c7…45175a` | 1 |
| `su_pravda_no192_19900711` | [Правда (Орган Центрального Комитета КПСС), № 192 (26275), Среда, 11 июля 1990 года: ИНФОРМАЦИОННОЕ СООБЩЕНИЕ о ходе XXVIII съезда Коммунистической партии Советского Союза (10 July 1990) and the biographical note headed ГОРБАЧЕВ / Михаил Сергеевич; page-image scan, Internet Archive stored file](https://ia600808.us.archive.org/23/items/1990192_8379/%D0%9F%D1%80%D0%B0%D0%B2%D0%B4%D0%B0%2C%201990%20%2C%20%E2%84%96%20192.pdf) | IA item `1990192_8379` (stored file, SHA-1 `6b90c031…`), 7,723,594, `e9708e2a…e3076c` | 5 |
| `su_pravda_no193_19900712` | [Правда (Орган Центрального Комитета КПСС), № 193 (26276), Четверг, 12 июля 1990 года: ИНФОРМАЦИОННОЕ СООБЩЕНИЕ о ходе XXVIII съезда Коммунистической партии Советского Союза (11 July 1990); page-image scan, Internet Archive stored file](https://ia600708.us.archive.org/3/items/1990193_2350/%D0%9F%D1%80%D0%B0%D0%B2%D0%B4%D0%B0%2C%201990%20%2C%20%E2%84%96%20193.pdf) | IA item `1990193_2350` (stored file, SHA-1 `09e9f05f…`), 7,999,931, `d678165c…8a5376` | 3 |
| `su_pravda_no194_19900713` | [Правда (Орган Центрального Комитета КПСС), № 194 (26277), Пятница, 13 июля 1990 года: ИНФОРМАЦИОННОЕ СООБЩЕНИЕ о ходе XXVIII съезда Коммунистической партии Советского Союза (12 July 1990), the correspondents' report and the biographical note headed ИВАШКО / Владимир Антонович; page-image scan, Internet Archive stored file](https://ia600103.us.archive.org/8/items/1990194_6991/%D0%9F%D1%80%D0%B0%D0%B2%D0%B4%D0%B0%2C%201990%20%2C%20%E2%84%96%20194.pdf) | IA item `1990194_6991` (stored file, SHA-1 `0829f20c…`), 7,671,842, `8e39422b…9ed363` | 3 |
| `su_pravda_no195_19900714` | [Правда (Орган Центрального Комитета КПСС), № 195 (26278), Суббота, 14 июля 1990 года: ИНФОРМАЦИОННОЕ СООБЩЕНИЕ о ходе XXVIII съезда Коммунистической партии Советского Союза (13 July 1990) and the list КОМИССИЯ ПО ПОДГОТОВКЕ ПРОЕКТА ПРОГРАММЫ КПСС; page-image scan, Internet Archive stored file](https://ia600705.us.archive.org/3/items/1990195_8575/%D0%9F%D1%80%D0%B0%D0%B2%D0%B4%D0%B0%2C%201990%20%2C%20%E2%84%96%20195.pdf) | IA item `1990195_8575` (stored file, SHA-1 `d3f7e916…`), 7,038,476, `c3797799…b8cfe8` | 2 |
| `su_izv_tsk_1991_08` | [Известия ЦК КПСС, информационный ежемесячный журнал, № 8 (319), август 1991 (signed to press 10.07.91): editorial board; page-image scan, Internet Archive stored file](https://ia903108.us.archive.org/13/items/B-001-036-100-ALL/B-001-036-100-03.pdf) | IA item `B-001-036-100-ALL` (stored file, SHA-1 `042e2fb9…`), 107,500,579, `3cacb753…424141` | 1 |
| `su_pravda_no201_19910822` | [Правда (Орган Центрального Комитета КПСС), № 201 (26649), Четверг, 22 августа 1991 года: Заявление Секретариата ЦК КПСС and the report Курс на демократизацию общества (21 August 1991); page-image scan, Internet Archive stored file](https://ia601903.us.archive.org/24/items/1991201_1702/%D0%9F%D1%80%D0%B0%D0%B2%D0%B4%D0%B0%2C%201991%20%2C%20%E2%84%96%20201.pdf) | IA item `1991201_1702` (stored file, SHA-1 `7020a496…`), 3,962,723, `3e5f0599…c5b610` | 3 |
| `su_vs_bulletin1_cpsu_19910826` | [Верховный Совет СССР, внеочередная сессия: Бюллетень No. 1 совместного заседания Совета Союза и Совета Национальностей, 26 августа 1991 г. (stenographic bulletin, morning sitting)](https://web.archive.org/web/20250718143607id_/https://sten.vs.sssr.su/12/6/1.pdf) | IA 20250718143607, 2,252,696, `849b9dae…39776a`; live identical | 2 |
| `su_ved_1991_35_cpsu` | [Ведомости Съезда народных депутатов СССР и Верховного Совета СССР, 1991, No. 35 (28 August 1991)](https://web.archive.org/web/20211204065955id_/https://vedomosti.sssr.su/1991/35.pdf) | IA 20211204065955, 618,372, `5a8c0630…c55a63`; live identical | 1 |
| `su_snd5_bulletin3_cpsu_19910903` | [Внеочередной пятый Съезд народных депутатов СССР. Бюллетень No. 3, 3 сентября 1991 г. (stenographic bulletin)](https://web.archive.org/web/20240901234307id_/https://snd.sssr.su/V/3.pdf) | IA 20240901234307, 2,146,995, `8eeae473…d9ac26`; live identical | 2 |
| `su_ved_1991_36_cpsu` | [Ведомости Съезда народных депутатов СССР и Верховного Совета СССР, 1991, No. 36 (4 September 1991)](https://web.archive.org/web/20250820135020id_/https://vedomosti.sssr.su/1991/36.pdf) | IA 20250820135020, 1,303,663, `87abb4c1…075b6f`; live identical | 1 |
| `su_kremlin_rsfsr_ukaz_169_19911106` | [Указ Президента РСФСР от 06.11.1991 г. № 169 «О деятельности КПСС и КП РСФСР»](https://web.archive.org/web/20260830202901id_/http://kremlin.ru/acts/bank/385) | IA 20260830202901, 11,221, `58c2fd44…804f3d`; served gzip, decoded 40,450, `aa81ecac…edf144` | 2 |

Hosting. **Pravda and Известия ЦК КПСС** are the Central Committee's own organs, the party's own records for its offices, but the
scans are Internet Archive items uploaded by private accounts (Pravda: collection `pravda-newspaper`, items added May 2025;
Известия ЦК КПСС: patron collection `nicolai-woodenko-library`, added September 2021); the Internet Archive is not the publisher and
the scans' origin is not documented. Each recorded identity is the item's stored PDF, byte-identical to the SHA-1 and size in the
item's metadata. archive.org's download URL redirects to a storage node that varies between requests, and the pipeline's check
follows no redirects, so each extract records the item's primary storage node as `source_url` and the canonical download URL in
`stored_file`. **The Supreme Soviet and Congress bulletins and Ведомости** are page-image scans of the official publications served
by the non-official SSSR.SU project (the host documented by the CLAUDE-C01-SOURCE-26 review); four sources repeat bytes already
recorded by CLAUDE-C01-26 or CLAUDE-C01-35 under their own IDs (`su_vs_bulletin1_soyuz_19910826`, `su_ved_1991_35`,
`su_snd5_bulletin3_19910903`, `su_ved_1991_36`), each as the same pre-cutoff raw capture under a new ID, so that the earlier
extracts stay unchanged. **RSFSR decree No. 169** is the text on the official site of the President of Russia (kremlin.ru, acts bank
item 385) as a pre-cutoff raw capture, with the original edition on the official legal portal attached. Whether the private-upload
scans and the SSSR.SU host meet the official-facsimile standard is the integrator's decision, as for CLAUDE-C01-26 (C12). If the
Internet Archive user uploads are ruled out, every new holder is lost (all five rest on Pravda) and SU-CPSU-01 to 04 fall to
leads; SU-CPSU-05 and 06 keep their claims from the official Union and RSFSR records. If the SSSR.SU host is ruled out, SU-CPSU-05
keeps no primary claim and SU-CPSU-06 keeps only the RSFSR decree.

## Response identities and stability checks

Every recorded and attached response was downloaded twice by this packet with plain curl (curl's own User-Agent, an explicit
`Accept-Encoding: identity` header, no cookies, no cache-busting query, no redirect followed): the first pass at
2026-09-30T23:36:32Z-23:48:39Z, the second at 2026-10-01T00:19:41Z-00:20:52Z; each response's two downloads are 31 to 43 minutes
apart, and all 17 pairs (12 recorded responses, four live files and one original edition) returned identical bytes and SHA-256 (the times are in each extract's `downloads`). Points a reviewer needs:

- The seven Internet Archive stored files: the SHA-1 of each recorded body equals the file SHA-1 in the item's metadata, and the
  byte count equals the recorded size. The canonical `https://archive.org/download/<item>/<file>` answered with a 302 to two
  different storage nodes on consecutive requests; the recorded storage-node URL answered 200 directly on every request.
- The four SSSR.SU captures are served without Content-Encoding; the base32 SHA-1 of each equals the Internet Archive CDX digest,
  and the live static files (fixed Last-Modified) are byte-identical and attached as `live_file_response`.
- The kremlin.ru capture (20260830202901) is always served gzip-encoded, even to a request without gzip; the extract records the
  served (gzip) identity, `source_response_content_encoding: gzip` and the decoded identity. Its base32 SHA-1 equals the CDX digest.
  The live kremlin.ru did not answer from this environment. The page is the current edition, with the Constitutional Court's notes
  of 30 November 1992 after points 1 and 3; the original edition (pravo.gov.ru, `nd=102012989`, `rdk=0`, over HTTP because the portal's
  HTTPS origin timed out, as for CLAUDE-C01-26) is attached as `original_edition_response` and has the same wording for the quoted
  points and the signature block.
- The scans carry OCR layers (Internet Archive tesseract for the stored files); they were used only to find passages. Every
  quotation was read from rendered page images, and quotation marks, dashes and the gazette's figures ("3.109", "2371—I") are kept
  as printed.

## Leads not imported

- **The stenographic report of the XXVIII Congress** ("XXVIII съезд Коммунистической партии Советского Союза, 2-13 июля 1990 года.
  Стенографический отчет", Politizdat 1991), seen only as an HTML transcription on the private site soveticus5.narod.ru (Internet
  Archive captures 20110316173946 of `xxviii_1.htm`, 1,492,347 bytes, `18e4ccb0…`, and 20110317072959 of `xxviii_2.htm`, 1,452,135
  bytes, `cb298747…`): counting commission protocol No. 2 of 10 July 1990 (Горбачев 3411 for, 1116 against; Авалиани 501 for) with
  Gorbachev's words "Принимаю эти обязанности", and protocol No. 3 (Ивашко 3109 for, 1309 against), dated "от И июля 1990 года" in
  the transcription's OCR. A transcription on a private host with OCR errors, not a facsimile; the live host is behind a network
  warning redirect here. A facsimile of the report (a pirate PDF site was seen and not used) remains to be found.
- **The statement of 24 August 1991**: doc20vek.ru (node 4247, a history site reproducing a 1996 textbook reader that cites
  Российская газета of 27 August 1991), the Gorbachev Foundation's chronicle (gorby.ru, Хроника перестройки), the Yeltsin Center's
  digest "День за днем. 24 августа 1991 года" and rusconstitution.ru. No date or end is taken from them.
- **Encyclopaedias and wikis**: Wikipedia, Ruwiki and Wikiwand (Ивашко as acting General Secretary from 24 August to 6 November
  1991; Gorbachev leaving Шенин in charge in August 1991), bigenc.ru and news retrospectives (РИА Новости, Interfax, aif.ru). Leads
  only.
- **Pravda after 22 August 1991** (Nos. 202-275, no longer the Central Committee's organ after its suspension): the General
  Secretary's press conference of 22 August (No. 202), a commentary quoting the Secretariat's message of 21 August on Ивашко's
  request to meet Горбачев (No. 202) and a retrospective note on the journal's editorial board headed by Горбачев and then Ивашко
  (No. 229). News, not imported.
- **Известия ЦК КПСС 1990 Nos. 8-10** (Internet Archive items B-001-036-088/089/090): the Congress summary "Избранные съездом
  Генеральный секретарь ЦК КПСС М. С. Горбачев и заместитель Генерального секретаря ЦК КПСС В. A. Ивашко одновременно выбраны и
  членами ЦК КПСС" and the July 1990 plenum; read in the OCR text only, not recorded (the Pravda records cover the same days).
- **The CPSU Rules of 13 July 1990** (creating the Deputy General Secretary): procedure only, not imported.
- **RSFSR decrees No. 79 (23 August 1991, suspending the RSFSR Communist Party) and No. 90 (25 August 1991, on the property of the
  CPSU and the RSFSR Communist Party)** on pravo.gov.ru, and the Constitutional Court's judgment 9-П of 30 November 1992: another
  identity or after the period.

## Sources attempted

- Read and not recorded: Pravda 1990 Nos. 1-5, 35-41 and 186-198 and 1991 Nos. 195-275 (OCR text of the Internet Archive items);
  Pravda 1990 No. 196 (the plenum communique of 13-14 July 1990 and the biographies; duplicates of Nos. 194-195); Известия ЦК КПСС
  1991 No. 7 (signed to press 7 June 1991); the USSR Fifth Congress bulletins 1, 2 and 4-7 and the Supreme Soviet bulletin No. 2 of
  26 August 1991 (Ивашко only in the roll-call lists, absent); Ведомости 1991 No. 35-36 beyond the recorded articles.
- The Yeltsin Center archive (search JSON used for discovery only): Fund 7, cases 46-48 ("Документы по событиям 19-21 августа 1991
  г. и приостановлении деятельности КПСС") hold letters and appeals to the RSFSR President, no party record of the offices.
- `soveticus5.narod.ru` live: redirected by a network safe-browsing filter here (not bypassed); only raw captures were read.
- `pravo.gov.ru` over HTTPS: timed out (port 443); the original edition was downloaded over HTTP and is attached, not recorded as
  the identity. `kremlin.ru` live: no answer from this environment.
- Not used: dokumen.pub (a pirate copy of the stenographic report), imwerden.de (Известия ЦК КПСС 1990 Nos. 11-12, a private
  library).

## Suggested next work orders

These are proposals for the integrator. They are not created in `work-orders.json`.

- `C01-USSR-CPSU-001`: an official facsimile of the XXVIII Congress stenographic report (counting commission protocols Nos. 2-3) or
  of "Материалы XXVIII съезда КПСС", to record the protocols and vote figures from the Congress's own record.
- `C01-USSR-CPSU-002`: a primary record of the General Secretary's statement of 24 August 1991 (TASS text in a state gazette or an
  archival copy, e.g. the Presidential Archive or RGANI), which would allow an `until` under the resignation ruling.
- `C01-USSR-CPSU-003`: the Deputy General Secretary's acting service and the Central Committee Secretariat's records after 24 August
  1991 (RGANI fund 4 or the 1992 Constitutional Court case file), and any record of the end of his office.
- `C01-USSR-CPSU-004`: an integrator ruling on Internet Archive items uploaded by private accounts as hosts for party newspapers and
  journals (the Pravda and Известия ЦК КПСС scans), parallel to the SSSR.SU ruling for CLAUDE-C01-26.

## Integration notes (outside this packet's file boundary)

- **Stacking.** Not stacked: claim commit `7778be5b` sits directly on `02d2c5a2`. `codex/campaign-certification` is fetched again
  before committing and merged if it moved.
- **Batch order.** The user chose to start this batch (CLAUDE-C01-38 to C01-41, run in parallel in other country files) before
  Codex's roadmap line "continue existing claims first" was worked through; this packet is one of them.
- **Shared file.** `research-index.json` is regenerated in a **separate commit** and is the only file this packet shares with the
  parallel packets; if another packet lands first, regenerate the index rather than merging it. New totals: 1,898 sources and 4,753 claims (from 1,886 and 4,727); organization observations (844), institution observations (36) and discovery batches (93) are unchanged; USSR's source claims go from 131 to 157, its entries (7), role observations (8) and `mapping_pending` (7) are unchanged.
- **Existing records changed.** No existing source, claim, extract or holder is edited. On `su_cpsu` this packet appends to
  `sources` and `claim_ids`, appends to each role's `sources`, `claim_ids` and `holder_claims` (after the S10.h holder), adds a
  `scope_note` to each role (as CLAUDE-C01-26 did for `su_supreme_soviet_chair`) and appends four coverage items; one packet
  coverage item is appended. Role ids, titles and kinds, the organization's name, jurisdiction and lifecycle are unchanged.
- **Existing tests updated** (pinned counts, exact sets and access dates only; none loosened): `test_ussr_research_s10h.py`: totals (7, 56, 131, 8) → (7, 68, 157, 8); the table-extract count 46 → 58; the PDF-page set adds the 11 scanned PDFs of this packet (`C01_41_PDF_SOURCES`); access dates add 2026-09-30, pinned to positions 56-67, with 2026-09-29 re-pinned to positions 40-55 (`[40:56]`); index source claims 131 → 157; the CPSU roles' source pin is re-expressed exactly: the S10.h source stays first in both roles and every later source is one of this packet's (`C01_41_SOURCES`). `test_ussr_government_supreme_soviet_c01_26.py`: packet totals (68, 157, 7, 8) and index figures (4, 8, 157, 7); its new-source list stays pinned to positions 10-39. `test_ussr_democratic_russia_soyuz_c01_35.py`: its new-source list is pinned to positions 40-55 (`[40:56]`), the packet totals become (68, 157, 7, 8), the source count 56 → 68 and the index figures (3, 4, 8, 157, 7); its holder guard keeps the same SHA-256 and now skips only holders resting on this packet's sources (`C01_41_SOURCES`), which the new test pins in full. `test_ussr_russia_transition_c01_05.py` is unchanged.
- **Rulings requested.** (1) Internet Archive user uploads of Pravda and Известия ЦК КПСС as hosts (all five holders depend on
  them). (2) Whether the TASS-attributed report of a Central Committee secretary's press conference reprinted by Pravda attests the deputy's office on 21 August
  1991. (3) Whether the General Secretary's own words of 26 August 1991, with the statement's date only in leads, should ever give
  `until` 1991-08-24 (this packet sets none). (4) Whether a re-election by the Congress on a stated day should give `from` (this
  packet follows the CLAUDE-C01-26 rule: an election is a claim, not a holder start).
- **Known failures outside the listed checks** (not fixed here): `tools/avatars/test_certified_gap_ledger.py` reports "no pinned
  attribution" for this packet's sources until Codex classifies its commit; `tools/avatars/test_certified_boundary_matrix.py` (S23)
  needs `spheres-web/src`, absent from the sparse checkout, and Codex regenerates the boundary matrix on integration.
  `docs/campaign-certification/C01/gap-ledger/` is not touched.
- `research/README.md`, the C01 README totals and `docs/planning/ai-workstreams.json` are left for the integrator; this handoff is
  not registered there.
- The new test `test_ussr_cpsu_general_secretary_c01_41.py` pins the unchanged S10.h holders, the five new holders with their
  claims, every claim's date, kind, observation and role, every row's `holder_name`, every response identity, stored-file SHA-1,
  capture, live file, decoded identity and pair of downloads, the extracts against the packet and their snapshots, the four
  same-bytes sources against the earlier records, the separation from state offices and `russia.json`, and 19 mutations.

## Checks

```text
python -X utf8 tools/avatars/campaign_research.py
python -X utf8 tools/avatars/campaign_research.py --check
python -X utf8 tools/avatars/campaign_census.py --check
python -X utf8 -m unittest discover -s tools/avatars -p "test_ussr*.py"
python -X utf8 -m unittest discover -s tools/avatars -p "test_russia*.py"
python -X utf8 -m unittest discover -s tools/avatars -p "test_*research*.py"
python -X utf8 -m unittest discover -s tools/avatars -p "test_campaign*.py"
node --test tools/ui/check_leadership_research_review.cjs
python tools/planning/workboard.py --check
git diff --check (this packet's paths)
python -X utf8 D:/spheres-scratch/c01-pipeline/tools/packet_check.py 41
```

Results are recorded in the handoff's Checks paragraph.

## Independent review precision corrections (1 October 2026)

The press-conference article on page 2 of Pravda No. 201 is explicitly signed TASS.
It is attributed reportage of a named party officer, reprinted by the party organ;
it is not a signed party statement or a verbatim conference transcript. The
21 August deputy observation retains that limitation. The Kremlin edition
contains later court qualifications after both points 1 and 3. Both original
1991 clauses were separately checked on the legal portal; no present legal
validity is asserted. A coverage phrase now says attested **on**, rather than
**to**, 22 August, so it does not imply a continuous term.

These corrections do not change claim dates, holder observations, office
boundaries, historical source bytes or the unresolved Ivashkov identity. See
[the independent review](../integrations/CLAUDE-C01-41/review-2026-10-01/README.md).
