# USSR government and President 49: the President and the head of the Union government, dated attestations, 1990-1991

Packet: **CLAUDE-C01-49**. State: **ready_for_review** (not complete).
Owner: Claude. Integrator/reviewer: Codex. Branch `claude/c01-su-49`; claim commit `b2ae720b` on `5ea4f8fc`, the head of
`codex/campaign-certification` when the packet was claimed; **not stacked** on any pending packet. Research access: 1 October 2026
(UTC). The historical cutoff stays **7 September 2026**.

This packet reviews six observations, SU-PRES-01 to SU-PRES-04 and SU-GOV-11 to SU-GOV-12, of two existing roles in
[ussr.json](ussr.json): `su_president` (institution `su_presidency`, kind `head_of_state`) and `su_government_head` (institution
`su_government`, kind `head_of_government`), from 1 January 1990 to 25 December 1991. It adds:

- four dated holder observations of Михаил Сергеевич Горбачев as President: **from 15 March 1990** (the oath and the presiding
  officer's declaration that he had assumed the office) and `attested_on` 22 January, 22 August and 5 October 1991 (signed decrees);
- one dated holder observation of Валентин Сергеевич Павлов as Premier, `attested_on` 22 January 1991 (signed resolution);
- 6 sources and 17 claims, each source with a checked-in derived extract, a scope note on `su_president`, three coverage items on
  `su_presidency`, two on `su_government` and one packet coverage item.

The existing holders keep their places and are unchanged: on `su_president` the two CLAUDE-C01-05 observations ("Mikhail
Gorbachev", 20 March 1990 and 25 December 1991), on `su_government_head` the three CLAUDE-C01-26 observations (Рыжков 12 January and
24 November 1990, Павлов 19 August 1991). The new observations are appended after them in chronological order. No existing source,
claim, extract, holder, role title or kind, scope note or lifecycle is edited; `russia.json` is not touched; no organization,
institution, game mapping, portrait or avatar is added. No holder has an `until`. The parent scope (C01, C06, S23, WC1 and CP1)
remains open.

## Outcome

| ID | Question | Decision |
|---|---|---|
| SU-PRES-01 | The Congress election and the oath, 14-15 March 1990 | **Accepted:** the Congress's official stenographic report records the ballot (14 March, from 'вчера'), the counting commission's result (1329 for, 495 against), the protocol's approval and the election resolution, the oath and the presiding officer's declaration 'Михаил Сергеевич Горбачев вступил в должность Президента Союза Советских Социалистических Республик', all at the sitting of 15 March 1990; holder **from 1990-03-15** |
| SU-PRES-02 | In-office attestations between the oath and the August events, and the latest Soviet signature | **Accepted:** a decree of 22 January 1991 signed 'Президент Союза Советских Социалистических Республик М. ГОРБАЧЕВ' (Pravda No. 20) and decree УП-2668 of 5 October 1991 (Vedomosti No. 41); holders `attested_on` 1991-01-22 and 1991-10-05 |
| SU-PRES-03 | The August 1991 events | **Accepted in part:** the Vice-President's decree of 18 August 1991 assuming the President's duties 'с 19 августа 1991 года', the statement of the 'Soviet leadership' and the acting styling (Izvestia No. 197) are claims only; the Presidium's resolution 2352-I of 21 August 1991 declaring the removal unlawful is a claim; decree УП-2443 of 22 August 1991 attests the office (holder `attested_on` 1991-08-22). No break, resumption day or acting holder |
| SU-PRES-04 | The 25 December 1991 statement | **Not resolved:** no Soviet primary text was found (Pravda No. 302 of 26 December carries only news reports; leads). SURU-TR91-06 stays open; no `until` |
| SU-GOV-11 | The Premier's approval, 14 January 1991 | **Accepted as a claim:** the Supreme Soviet's resolution 'О премьер-министре СССР' ('Утвердить премьер-министром СССР товарища Павлова Валентина Сергеевича', 'Москва, Кремль. 14 января 1991 г.') as printed in Pravda No. 13; the note headed 'Премьер-министр СССР' is a styling claim. No `from` (no effective day) |
| SU-GOV-12 | The Premier's first attestation and the August styling | **Accepted:** the government's resolution of 22 January 1991 signed 'В. ПАВЛОВ / Премьер-министр' (Pravda No. 20); holder `attested_on` 1991-01-22. The statement of 18 August 1991 lists 'Павлов В. С. — Премьер-Министр СССР' (claim). Any acting Premier after 22 August 1991 remains unsourced |

### Holders

`su_president` (existing observations first and unchanged):

| # | Name | `attested_on` | `from` | `until` | Claims |
|---|---|---|---|---|---|
| 0 | Mikhail Gorbachev (CLAUDE-C01-05) | 1990-03-20 | null | null | `su_gorbachev_president_letter_19900320` |
| 1 | Mikhail Gorbachev (CLAUDE-C01-05) | 1991-12-25 | null | null | `su_telcon_gorbachev_title_19911225` |
| 2 | Михаил Сергеевич Горбачев | null | **1990-03-15** | null | `su_snd3p_gorbachev_oath_19900315`, `su_snd3p_assumption_of_office_declared_19900315` |
| 3 | Михаил Сергеевич Горбачев | 1991-01-22 | null | null | `su_pravda20_president_signs_decree_19910122` |
| 4 | Михаил Сергеевич Горбачев | 1991-08-22 | null | null | `su_ved35p_up2443_president_signs_19910822` |
| 5 | Михаил Сергеевич Горбачев | 1991-10-05 | null | null | `su_ved41p_up2668_president_signs_19911005` |

`su_government_head` (existing observations first and unchanged):

| # | Name | `attested_on` | `from` | `until` | Claims |
|---|---|---|---|---|---|
| 0-2 | Рыжков (1990-01-12, 1990-11-24), Павлов (1991-08-19) (CLAUDE-C01-26) | as before | null | null | unchanged |
| 3 | Валентин Сергеевич Павлов | 1991-01-22 | null | null | `su_pravda20_premier_signs_resolution_19910122` |

### Starts and ends

- **From 15 March 1990.** The stenographic report of the seventh sitting (15 March 1990, printed p. 56) records the oath and, at
  once, the presiding officer's words that he 'вступил в должность Президента'. The law's rule that the person elected enters
  office from the moment of the oath is the existing claim `su_first_president_congress_rule_19900314`. This follows the precedent
  of the RSFSR President's oath (CLAUDE-C01-05: Law 1494-I and the stenogram of 10 July 1991 give `from` 1991-07-10). The holder
  cites only the stenogram, and `attested_on` is null, as on that precedent. The election result and resolution of the same day and
  the ballot of 14 March are claims; they are not the start.
- **No other start.** The Premier's approval of 14 January 1991 states no effective day (CLAUDE-C01-37 rule), so it gives no
  `from`; the observation rests on his signature of 22 January. Decrees and resolutions give `attested_on` only.
- **No end.** Nothing in this packet ends either office. The 25 December 1991 statement has no Soviet primary text here; decree
  УП-2443 and the Supreme Soviet's consent remain the existing SU-GOV-06 question.

### Name forms and identities

The new President observations use the full name as the presiding officer prints it ('Михаил Сергеевич Горбачев'); the speaker
line prints 'Горбачев М. С.' and the decrees 'М. ГОРБАЧЕВ'. The CLAUDE-C01-05 observations keep their English 'Mikhail Gorbachev';
the two forms are not reconciled, as CLAUDE-C01-41 did not reconcile its S10.h forms. The Premier is named as decree УП-2443 prints
him ('Павлова Валентина Сергеевича'), matching the CLAUDE-C01-26 holder. People named in rows: Горбачев and Павлов (holders), Янаев
(claimant), and Лукьянов, Осипьян, Бакланов and Шкабардня (presiding officer, counting commission chair and signatories): seven.

### Party office and state office

No claim of this packet is cited by a CPSU role, and no party record supports a state holder. The General Secretary observations of
CLAUDE-C01-41 are untouched.

## Date ledger

| Date | Event | Claim | Effect |
|---|---|---|---|
| 14 Mar 1990 | Ballot for President (from 'вчера' on 15 March) | `su_snd3p_president_ballot_held_19900314` | claim |
| 15 Mar 1990 | Counting commission's result; protocol approved; resolution 'Избрать Президентом' | `su_snd3p_president_vote_result_19900315`, `su_snd3p_protocol_approved_resolution_adopted_19900315` | claims |
| 15 Mar 1990 | Oath; presiding officer declares the assumption of office | `su_snd3p_gorbachev_oath_19900315`, `su_snd3p_assumption_of_office_declared_19900315` | holder **from** |
| 14 Jan 1991 | Supreme Soviet approves the Premier (as printed in Pravda) | `su_pravda13_supreme_soviet_approves_premier_19910114` | claim |
| 15 Jan 1991 | Note headed 'Премьер-министр СССР' (issue date) | `su_pravda13_note_styles_premier_19910115` | claim |
| 22 Jan 1991 | Presidential decree and government resolution signed | `su_pravda20_president_signs_decree_19910122`, `su_pravda20_premier_signs_resolution_19910122` | holders `attested_on` |
| 18 Aug 1991 | Vice-President's decree; 'Soviet leadership' statement; acting styling; Premier listed in the statement | `su_izv197_*` (4) | claims only |
| 21 Aug 1991 | Presidium 2352-I: removal unlawful; Vice-President to revoke his decrees | `su_ved35p_presidium_2352i_*` (2) | claims |
| 22 Aug 1991 | Decree УП-2443 signed | `su_ved35p_up2443_president_signs_19910822` | holder `attested_on` |
| 5 Oct 1991 | Decree УП-2668 signed | `su_ved41p_up2668_president_signs_19911005` | holder `attested_on` |

## Observations

### SU-PRES-01 — The Congress election and the oath, 14-15 March 1990

The stenographic report (volume III, seventh sitting, 'Кремлевский Дворец съездов. 15 марта 1990 года. 10 часов утра', presided
by А. И. Лукьянов) is the Congress's own record. The presiding officer refers to the previous day's ballot; the counting commission's
chairman reports 2245 deputies, 2000 ballots issued, 1878 found, 54 invalid, 1329 for and 495 against, and announces the election;
the protocol is approved (1822, 47, 35) and the resolution adopted; the oath follows and the presiding officer declares the
assumption of office. Each event is its own dated claim. The resolution's signed original (1362-I) was already a role claim of
CLAUDE-C01-26 and is unchanged. The source repeats the bytes of `su_snd3_steno_vol3` (CLAUDE-C01-35) under a new ID.

### SU-PRES-02 — In-office attestations, January and October 1991

The decree of 22 January 1991 on the withdrawal of the 50- and 100-rouble notes, printed in Pravda No. 20, is signed by the
President; decree УП-2668 of 5 October 1991 (Vedomosti No. 41, art. 1164) is the latest dated presidential decree in the latest
scanned issue on the host (Nos. 42-52 have no scans). Both give `attested_on` only.

### SU-PRES-03 — The August 1991 events

Izvestia No. 197 (union edition, 20 August 1991) prints the Vice-President's decree ('вступил в исполнение обязанностей
Президента СССР с 19 августа 1991 года', dated 18 August), the statement of the 'Soviet leadership' (powers passed to the
Vice-President; signed by Янаев, Павлов and Бакланов) and the appeal signed 'И. о. Президента СССР Г. ЯНАЕВ'. They are claims only:
acting service is never a holder or boundary. The Presidium's resolution 2352-I (21 August 1991, Vedomosti No. 35, art. 1001)
declares the removal unlawful and demands that the Vice-President revoke his decrees: claims, not used as a holder observation.
Decree УП-2443 (22 August 1991, art. 1008) attests the office after the events. No break in the office, resumption day or end is
recorded.

### SU-PRES-04 — The 25 December 1991 statement

No Soviet primary text of the statement or of decree УП-3162 was found. Pravda No. 302 (26 December 1991), the only Soviet
newspaper of that day available as a scan, carries news reports ('Горбачев уходит'; the telephone call about the decree on the
command), not the text; Izvestia of 26 December and Vedomosti No. 52 have no reviewable facsimile; the SSSR.SU compilation is a
transcription. The CLAUDE-C01-05 observation of 25 December (US records) stays the latest, with no `until`.

### SU-GOV-11 — The Premier's approval, 14 January 1991

Pravda No. 13 (15 January 1991) prints the Supreme Soviet's resolution approving 'товарища Павлова Валентина Сергеевича' on the
President's submission, signed by А. ЛУКЬЯНОВ and dated 14 January 1991, and a note headed 'Премьер-министр СССР ПАВЛОВ Валентин
Сергеевич'. This is a party newspaper's printing of a state act, on a private Internet Archive upload; it does not reuse the
withdrawn Izvestia identity (SU-GOV-05) and does not supply the act's number or the official gazette. Both rows are claims.

### SU-GOV-12 — The Premier's first attestation and the August styling

Pravda No. 20 (23 January 1991) prints the government's resolution of 22 January 1991 on the same currency measure, headed
'Постановление Кабинета министров СССР от 22 января 1991 г.' and signed 'В. ПАВЛОВ / Премьер-министр / М. ШКАБАРДНЯ' (the printed
block runs the title line between the two names). It gives his earliest observation, `attested_on` 1991-01-22. The paper's heading
'Кабинет министров' is noted, not modelled (SU-GOV-03). The statement of 18 August 1991 lists him as Premier among the committee's
members (claim). No primary record of an acting Premier after 22 August 1991 was found.

## Sources added

6 sources, each with a checked-in derived factual extract under [sources/](sources/) (`ussr-*-facts.json`, LF, format
`spheres-c01-derived-factual-table/v1`, with its own checksum in the packet). Each extract records the recorded response's URL, byte
count and SHA-256, both of this packet's downloads, the attached live file where there is one, the edition and host, the pages read
and one row per claim. Original pages and PDFs are not checked in; no photograph, emblem or signature image is republished.

| Source ID | What | Response identity (bytes, SHA-256) | Claims |
|---|---|---|---|
| `su_snd3_steno_vol3_president` | [Внеочередной третий Съезд народных депутатов СССР. Стенографический отчет. Том III](https://web.archive.org/web/20240903210942id_/https://snd.sssr.su/III/III.pdf) (seventh sitting, 15 March 1990) | IA 20240903210942, 12,922,126, `ae895c02…0b07b345`; live identical | 5 |
| `su_pravda_no13_19910115` | [Правда, № 13 (26461), 15 января 1991 года](https://ia600706.us.archive.org/12/items/199113_8099/%D0%9F%D1%80%D0%B0%D0%B2%D0%B4%D0%B0%2C%201991%20%2C%20%E2%84%96%2013.pdf): resolution 'О премьер-министре СССР' | IA item `199113_8099` (stored file, SHA-1 `5b52dc7a…`), 3,947,088, `84892605…12ea845e` | 2 |
| `su_pravda_no20_19910123` | [Правда, № 20 (26468), 23 января 1991 года](https://ia601904.us.archive.org/17/items/199120_2066/%D0%9F%D1%80%D0%B0%D0%B2%D0%B4%D0%B0%2C%201991%20%2C%20%E2%84%96%2020.pdf): decree and resolution of 22 January 1991 | IA item `199120_2066` (stored file, SHA-1 `93841a36…`), 5,294,185, `a6539752…b91d8a13` | 2 |
| `su_izv_197_19910820` | [Известия Советов народных депутатов СССР, № 197 (23463), 20 августа 1991 года](https://izvestija.sssr.su/1991/197.pdf) (union edition) | live static file, 39,761,395, `ddb564e7…7e63e45e4d`; Last-Modified 2 Dec 2016; no capture exists | 4 |
| `su_ved_1991_35_president` | [Ведомости, 1991, No. 35 (28 August 1991)](https://web.archive.org/web/20211204065955id_/https://vedomosti.sssr.su/1991/35.pdf): arts. 1001, 1008 | IA 20211204065955, 618,372, `5a8c0630…13c55a63`; live identical | 3 |
| `su_ved_1991_41_president` | [Ведомости, 1991, No. 41 (9 October 1991)](https://web.archive.org/web/20240906045835id_/https://vedomosti.sssr.su/1991/41.pdf): art. 1164 | IA 20240906045835, 556,781, `91cf6571…bdff902e`; live identical | 1 |

Hosting, disclosed as CLAUDE-C01-26 and C01-41 did. **The stenogram, Ведомости and Известия** are page-image scans of official
publications served by the non-official SSSR.SU project (the host documented by the CLAUDE-C01-SOURCE-26 review). Three sources
repeat bytes already recorded under other IDs (`su_snd3_steno_vol3`, `su_ved_1991_35`, `su_ved_1991_41`); the earlier extracts stay
unchanged, and `su_ved_1991_41_president` records the pre-cutoff capture where CLAUDE-C01-26 recorded the live file. Izvestia No.
197 has no Internet Archive capture, so its identity is the live static file. **Pravda** scans are Internet Archive items uploaded by
a private account (collection `pravda-newspaper`, May 2025); Pravda was the CPSU Central Committee's organ, so its printing of state
acts is a party newspaper's text of the act, not a state publication. If the private uploads are ruled out, the President's 22
January observation, the Premier's 22 January observation and both SU-GOV-11 claims are lost; if the SSSR.SU host is ruled out,
the `from` 1990-03-15, the 22 August and 5 October observations and the August claims are lost.

## Response identities and stability checks

Every recorded and attached response was downloaded twice by this packet with plain curl (curl's own User-Agent, no
Accept-Encoding header, no cookies, no cache-busting query, no redirect followed): the first pass at 2026-10-01T12:01:50Z-12:03:31Z,
the second at 2026-10-01T12:35:32Z-12:37:23Z; each response's two downloads are 33 to 34 minutes apart, and all 9 pairs
(6 recorded responses and 3 attached live files) returned identical bytes and SHA-256 (times in each
extract's `downloads`). Points a reviewer needs:

- The two Internet Archive stored files: the SHA-1 of each recorded body equals the file SHA-1 in the item's metadata, and the byte
  count equals the recorded size. The recorded URL is the item's primary storage node, as for CLAUDE-C01-41.
- The three SSSR.SU captures are served without Content-Encoding; the base32 SHA-1 of each equals the Internet Archive CDX digest,
  and the live static files (fixed Last-Modified) are byte-identical and attached as `live_file_response`.
- Izvestia No. 197: the CDX index holds no capture of `izvestija.sssr.su/1991/197.pdf`; the live file has a fixed Last-Modified
  (Fri, 02 Dec 2016 01:36:05 GMT) and no text layer.
- Izvestia No. 197 is a 39.8 MB file. A third download by `packet_check.py` on 1 October 2026 stopped at curl's 120-second
  `--max-time` and kept 33,680,543 bytes (`2d85c149ae56…`). The first 33,680,543 bytes of the recorded body hash to exactly that
  value, so it was a truncated transfer of the same file, not a different file. A full re-download at 2026-10-01T12:43:43Z (31.9 s)
  returned the recorded 39,761,395 bytes and `ddb564e7…7e63e45e4d` with the same ETag and Last-Modified, and the recorded identity
  is unchanged. Re-verify it with a time limit long enough for the whole file.
- Every quotation was read from rendered page images (PyMuPDF crops); OCR or text layers were used only to find passages.

## Leads not imported

- **Pravda No. 302 (26 December 1991)**, Internet Archive item `1991302_9265`: front-page news items 'Горбачев уходит' and
  'Судьба ядерной кнопки' (the telephone call of 25 December about a decree on the supreme command). News, not the statement.
  Pravda No. 301 (25 December) announces that he will speak. Not imported.
- **The SSSR.SU compilation 'Избранные документы органов власти СССР по поводу Беловежских соглашений'**
  (`https://sssr.su/1991-12.pdf`, a Word-document transcription of April 2015 citing web copies), which includes the text of the
  25 December address: a transcription, not a facsimile.
- **The Gorbachev Foundation's text of the address (gorby.ru)** and the 1000 Schlüsseldokumente edition (CLAUDE-C01-05 leads):
  not fetched again.
- **Izvestia No. 200 (union edition, 23 August 1991)** (`https://izvestija.sssr.su/1991/200.pdf`, capture 20250718143614): a TASS
  item headed 'ЗАЯВЛЕНИЕ ПРЕЗИДЕНТА СССР' ('В ближайшие сутки Президент приступит к полному исполнению своих обязанностей') states
  no day for the statement, and its report of the Presidium's resolution duplicates the gazette text recorded from Vedomosti No.
  35; the committee's legal conclusion on the Vice-President's decree is commentary. Read, not imported.
- **Izvestia No. 198 (Moscow evening edition, 20 August 1991)**: reports of the coup days and foreign reactions quoting the 'acting
  President'. News, not imported.
- **Pravda No. 76 (17 March 1990)**: a telegram signed 'Президент СССР М. ГОРБАЧЕВ' on the Congress's resolution of 15 March,
  without a date of its own; the stenogram already attests the office from 15 March. Not imported.
- **Encyclopaedias and wikis** on Doguzhiev as acting Premier after 22 August 1991 (CLAUDE-C01-26 leads): no primary record found.

## Sources attempted

- Read and not recorded: Pravda 1990 Nos. 74-77 and 334-358 and 1991 Nos. 11-22 and 280-306 (OCR text of the Internet Archive
  items, for discovery); no signed act of the Chairman of the Council of Ministers appears in Pravda of December 1990, and no
  presidential decree text in Pravda of December 1991.
- `pravo.gov.ru` list pages answer curl's own User-Agent with HTTP 204 (no content); they were not retried with another
  User-Agent. Single document pages answer over HTTP (HTTPS timed out, as for CLAUDE-C01-26); a short sequential scan of
  document numbers 102010317-102010362 (acts of 10-15 January 1991) found RSFSR acts and two Union acts (the Fundamentals of
  legislation on employment, signed 'Президент ... М.ГОРБАЧЕВ', `nd=102010349`, and its enactment resolution, `nd=102010350`),
  not recorded because the decree of 22 January 1991 already attests the office that month; the scan was stopped as too slow
  (10 to 30 seconds per page).
- Internet Archive search for Izvestia, Komsomolskaya Pravda and Rossiyskaya Gazeta of 26-27 December 1991 and for Izvestia of
  January 1991: no items.
- `vedomosti.sssr.su/1990/12/` and `/1990/12.pdf`: 404 (no 1990 scans); `izvestija.sssr.su` holds only August 1991 issues.

## Suggested next work orders

These are proposals for the integrator. They are not created in `work-orders.json`.

- `C01-USSR-PRES-001`: a Soviet primary text of the 25 December 1991 statement and decree УП-3162 (Izvestia or Rossiyskaya Gazeta of
  26-27 December 1991 as an official facsimile, Vedomosti 1991 No. 52 pp. 2058-2060, or GARF copies), which could allow an `until`
  under the resignation ruling.
- `C01-USSR-GOV-001`: an official facsimile of Vedomosti 1991 No. 4 (resolutions 1900-I and 1907-I), to record the approval and the
  pension resolution from the gazette instead of Pravda.
- `C01-USSR-GOV-002`: a primary record of who acted as Premier after 22 August 1991 (Cabinet orders of 22-28 August 1991 or the
  Supreme Soviet stenogram of 28 August).

## Integration notes (outside this packet's file boundary)

- **Stacking.** Not stacked: claim commit `b2ae720b` sits directly on `5ea4f8fc`. `codex/campaign-certification` is fetched again
  before committing and merged if it moved.
- **Shared file.** `research-index.json` is regenerated in a **separate commit** and is the only file this packet shares with the
  parallel packets; if another packet lands first, regenerate the index rather than merging it. USSR's source claims go from 157 to
  174; its entries (7), role observations (8) and `mapping_pending` (7) are unchanged.
- **Existing records changed.** No existing source, claim, extract or holder is edited. This packet appends to the two roles'
  `sources`, `claim_ids` and `holder_claims`, to the two institutions' `sources` and `coverage.unresolved`, adds a `scope_note` to
  `su_president` (it had none) and appends one packet coverage item. The existing notes of the CLAUDE-C01-26 holders ('the latest
  signature of his reviewed'; Павлов's 'It is his only observation') and the `su_government_head` scope note (observations at the
  earliest and latest attestation of each person) are left unchanged; Павлов's new observation is now his earliest.
- **Existing tests updated** (pinned counts, exact sets and guards re-expressed exactly; each appended record is pinned in the new
  test): `test_ussr_research_s10h.py` (totals 68/157 to 74/174, table extracts 58 to 64, the PDF-page set adds the six sources, access
  date 2026-10-01 pinned to positions 68-73, index claims 174, and the "no holder has a from" loop now names the one exception, the
  oath observation with from 1990-03-15); `test_ussr_russia_transition_c01_05.py` (the two C01-05 holders stay first; the four
  appended rest only on this packet's sources); `test_ussr_government_supreme_soviet_c01_26.py` (the two presidency and three
  government holders and the role claim lists stay first; appended ones rest only on this packet's sources; the from-guard names the
  oath observation; the NEVER_BOUNDARY guard checks only that observation's until, because its from (15 March) shares a day with
  the election resolution's claim; coverage lists extended by the two SU-GOV items; totals 74/174);
  `test_ussr_democratic_russia_soyuz_c01_35.py` (its base-holder hash also skips holders of the two state roles resting only on this
  packet's sources; totals); `test_ussr_cpsu_general_secretary_c01_41.py` (positions, totals, and the other-holders digest skips
  holders resting only on this packet's sources).
- **Known failures outside the listed checks** (not fixed here): `tools/avatars/test_certified_gap_ledger.py` reports "no pinned
  attribution" for this packet's sources until Codex classifies its commit; `tools/avatars/test_certified_boundary_matrix.py` (S23)
  needs `spheres-web/src`, absent from the sparse checkout, and Codex regenerates the boundary matrix on integration.
  `docs/campaign-certification/C01/gap-ledger/` is not touched.
- `research/README.md`, the C01 README totals and `docs/planning/ai-workstreams.json` are left for the integrator.
- The new test `test_ussr_government_president_c01_49.py` pins the unchanged existing holders (by hash), the five new holders with
  their claims, every claim's date, kind, observation and role, every row's `holder_name`, every response identity, stored-file
  SHA-1, capture and live file, the extracts against the packet and their snapshots, the three same-bytes sources against the
  earlier records, the separation from party office and `russia.json`, and 16 mutations.

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
python -X utf8 D:/spheres-scratch/c01-pipeline/tools/packet_check.py 49
```

Results are recorded in the handoff's Checks paragraph.
