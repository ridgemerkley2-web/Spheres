# Saudi executive chronology 03: kings, crown princes and the 2022 prime-minister exception

Packet: **CLAUDE-C01-06**. State: **ready_for_review** (not complete).
Owner: Claude. Integrator/reviewer: Codex. Branch `claude/c01-saudi-03`, base
`857da24d`. Research access: 21 September 2026. The historical cutoff stays
**7 September 2026**; the latest observation recorded is 13 August 2026.

This packet reviews eight observations on Saudi Arabia's kings and crown princes from
1990 to the cutoff in [saudi-arabia.json](saudi-arabia.json), and checks them against the
packet's existing 27 September 2022 prime-minister claims. It adds 22 sources and 33
claims. On existing roles it adds ten dated holder observations (three on `sa_king`,
seven on `sa_crown_prince`) and three claim-ID holders for the 13 August 2026
observation. The existing source `sa_spa_pm_2022` is re-read from a downloaded original;
its three claims are unchanged. The packet adds no organization, institution or role (in
particular no Deputy Crown Prince role), and no game mapping, lifespan, portrait or
avatar. The parent scope (C01, C06, S23, WC1 and CP1) remains open.

## Outcome

| ID | Question | Decision |
|---|---|---|
| SA-EXEC-01 | King Fahd in the 1990 window; his death | **Accepted with limits:** U.S. record of 8 Aug 1990; death announced 1 Aug 2005 with no stated day, so no end date |
| SA-EXEC-02 | Abdullah's 2005 accession; Sultan chosen Crown Prince | **Accepted:** reported by the Royal Court statement dated 1 Aug 2005; no separate pledge date; citizens' pledge only scheduled |
| SA-EXEC-03 | Sultan's death, Nayef's selection (A/224), Nayef's death | **Accepted:** deaths on 22 Oct 2011 and 16 Jun 2012 end both terms; A/224 dated 27 Oct 2011 |
| SA-EXEC-04 | Salman chosen Crown Prince (A/139) | **Accepted:** order of 18 Jun 2012; the term ends at his own accession (SA-EXEC-05) |
| SA-EXEC-05 | Abdullah's death; Salman's and Muqrin's pledges | **Accepted:** all on 23 Jan 2015; the Arabic dateline of 22 Jan is a filing artefact |
| SA-EXEC-06 | April 2015: Muqrin relieved, Mohammed bin Nayef chosen, Deputy Crown Prince | **Accepted:** A/159 and A/160 of 29 Apr 2015; pledge called that morning (06:33) for after Isha, and held that evening |
| SA-EXEC-07 | 21 June 2017: Mohammed bin Nayef relieved, Mohammed bin Salman chosen | **Accepted** on the Arabic original A/255; the 10/6/1436 recital is an error in the order text |
| SA-EXEC-08 | Consistency with the 2022 prime-minister exception; latest pre-cutoff observation | **Accepted:** consistent; 13 Aug 2026 observation added; no continuity inferred |

### Date ledger

Each row is a separate dated claim or field. A death, a pledge, a royal-order selection, a
relief, a scheduled or held ceremony and a release's filing time are not merged, even when
they fall on the same day.

| Date | Event | Claim or field |
|---|---|---|
| 8 Aug 1990 | U.S. Public Papers name Fahd as King | `sa_fahd_king_obs_19900808`; Fahd holder `attested_on` |
| 1 Aug 2005 (26/6/1426) | Royal Court announces Fahd's death; no day or hour of death given | `sa_fahd_death_announced_20050801` |
| 1 Aug 2005 | Abdullah styled Crown Prince, Deputy Premier, National Guard commander | `sa_abdullah_cp_obs_20050801` |
| reported 1 Aug 2005 | Family pledge to Abdullah as King | `sa_abdullah_family_pledge_20050801` |
| reported 1 Aug 2005 | Sultan chosen Crown Prince; family pledge to him | `sa_sultan_cp_selected_20050801` |
| 3 Aug 2005, scheduled | Citizens' pledge (not attested as held) | `sa_citizen_pledge_scheduled_20050803` (dated 1 Aug) |
| 22 Oct 2011 (24/11/1432), dawn | Sultan dies, styled Crown Prince | `sa_sultan_death_20111022`; Sultan `until` |
| 27 Oct 2011 (29/11/1432) | A/224 chooses Nayef; Commission notified | `sa_nayef_cp_order_a224`; Nayef `attested_on` |
| 28 Oct 2011, 00:51 | A/224 release filed (portal field 1432-12-01) | locator and uncertainty only |
| 16 Jun 2012 (26/7/1433) | Nayef dies, styled Crown Prince | `sa_nayef_death_20120616`; Nayef `until` |
| 18 Jun 2012 (28/7/1433) | A/139 chooses Salman | `sa_salman_cp_order_a139`; Salman (Crown Prince) `attested_on` |
| 23 Jan 2015 (3/4/1436), 01:00 | Abdullah dies, styled King | `sa_abdullah_death_20150123`; Abdullah (King) `until` |
| 23 Jan 2015 | Salman receives the pledge as King | `sa_salman_king_pledge_20150123`; Salman (King) `attested_on`, Salman (Crown Prince) `until` |
| 23 Jan 2015 | Muqrin receives the pledge as Crown Prince under Royal Order A/86 | `sa_muqrin_cp_pledge_20150123`; Muqrin `attested_on` |
| 23 Jan 2015, after Isha, scheduled | Citizens' pledge (not attested as held) | `sa_citizen_pledge_scheduled_20150123` |
| 23 Jan 2015, 02:33 | Arabic release filed under a 22 Jan dateline | `sa_abdullah_death_ar_dateline` |
| 29 Apr 2015 (10/7/1436) | A/159 relieves Muqrin at his request; chooses Mohammed bin Nayef | `sa_muqrin_relieved_a159`, `sa_mbn_cp_a159` |
| 29 Apr 2015 (10/7/1436) | A/160 chooses Mohammed bin Salman as Deputy Crown Prince | `sa_mbs_deputy_cp_a160` (no role, no holder) |
| 29 Apr 2015, 06:33 | Pledge called for after Isha that day | `sa_pledge_call_20150429` |
| 29 Apr 2015, evening | Pledge held at Qasr al-Hukm; Muqrin pledges | `sa_pledge_held_20150429` |
| 21 Jun 2017 (26/9/1438) | A/255 relieves Mohammed bin Nayef; chooses Mohammed bin Salman; 31 of 34 | `sa_mbn_relieved_20170621`, `sa_mbs_cp_selected_20170621`, `sa_allegiance_31_of_34_20170621` |
| 21 Jun 2017, 07:05 | Pledge called for after Taraweeh that day | `sa_pledge_call_20170621` |
| 21 Jun 2017 | Mohammed bin Nayef pledges to Mohammed bin Salman "today" | `sa_mbn_pledge_to_mbs_20170621` |
| 27 Sep 2022 | Mohammed bin Salman, "the Crown Prince", appointed Prime Minister (existing) | `sa_mbs_pm_appointment` |
| 13 Aug 2026 | King Salman issues orders; Mohammed bin Salman styled Crown Prince and Prime Minister | `sa_salman_king_obs_20260813`, `sa_mbs_cp_pm_obs_20260813` |
| unknown | Fahd's day of death; start of Abdullah's crown-princeship; family pledge time (2005) | no date recorded |

### Holder ledger

`from` is null for every new holder. No source read uses effective-date wording, so each
holder is event-dated by `attested_on`, following the Tonga convention for an appointment
event. `until` is recorded only where a source states the end: death while styled in the
office, relief by royal order, or the holder's own accession to the throne in the same
statement.

| Role | Holder | `attested_on` | `until` | End evidence |
|---|---|---|---|---|
| `sa_king` | Fahd bin Abdulaziz Al Saud | 1990-08-08 | null | death announced 1 Aug 2005, no day stated |
| `sa_king` | Abdullah bin Abdulaziz Al Saud | 2005-08-01 | 2015-01-23 | death, `sa_abdullah_death_20150123` |
| `sa_king` | Salman bin Abdulaziz Al Saud | 2015-01-23 | null | none; continuity not established |
| `sa_crown_prince` | Abdullah bin Abdulaziz Al Saud | 2005-08-01 | null | accession reported 1 Aug 2005, no separate date |
| `sa_crown_prince` | Sultan bin Abdulaziz Al Saud | 2005-08-01 | 2011-10-22 | death, `sa_sultan_death_20111022` |
| `sa_crown_prince` | Nayef bin Abdulaziz Al Saud | 2011-10-27 | 2012-06-16 | death, `sa_nayef_death_20120616` |
| `sa_crown_prince` | Salman bin Abdulaziz Al Saud | 2012-06-18 | 2015-01-23 | own accession, `sa_salman_king_pledge_20150123` |
| `sa_crown_prince` | Muqrin bin Abdulaziz Al Saud | 2015-01-23 | 2015-04-29 | relief, `sa_muqrin_relieved_a159` |
| `sa_crown_prince` | Mohammed bin Nayef bin Abdulaziz Al Saud | 2015-04-29 | 2017-06-21 | relief, `sa_mbn_relieved_20170621` |
| `sa_crown_prince` | Mohammed bin Salman bin Abdulaziz Al Saud | 2017-06-21 | null | none; continuity not established |

The existing claim-ID holders stay first on their roles: `sa_salman_king_observation`
(`sa_king`) and `sa_mbs_pm_appointment` (`sa_crown_prince`, `sa_pm`). The 13 August 2026
claims are added the same way: `sa_salman_king_obs_20260813` on `sa_king`, and
`sa_mbs_cp_pm_obs_20260813` on `sa_crown_prince` and `sa_pm`.

## Observations

### SA-EXEC-01 — King Fahd in the 1990 window, and his death

Evidence:

- U.S. Public Papers, address of 8 August 1990 (`sa_fahd_king_obs_19900808`). The President
  says that, after consulting with King Fahd, he sent the Secretary of Defense to discuss
  cooperative measures. The editorial note identifies
  "King Fahd bin `Abd al-`Aziz Al Sa`ud of Saudi Arabia".
- Arabic Royal Court statement, dateline Riyadh 26 Jumada II 1426 / 1 August 2005, filed
  11:04 Makkah time (`sa_fahd_death_announced_20050801`). It mourns King Fahd in the name of
  Crown Prince Abdullah, the family and the nation, says death came to him after an illness,
  and sets the funeral prayer for the next day, Tuesday.
- SPA's English release of the same announcement (`sa_abdullah_cp_obs_20050801`) styles
  Abdullah Crown Prince, Deputy Premier and Commander of the National Guard. Its dateline,
  "Riyadh, Aug 1", has no year; the Arabic original carries the full date.
- An SPA biography published the same afternoon (`sa_fahd_mourned_26_6_1426`) equates
  Monday 26/6/1426 AH with 1 August 2005. It is kept only as corroboration of that equivalence.

Decision: accepted. The Fahd holder on `sa_king` is observed on 1990-08-08 and cites the
death announcement, but `until` is null. The statement dates the announcement, not the death.

Limits: the 1990 observation is a foreign official record, read from a presidential-library
transcription rather than the GPO print edition. No Saudi 1990 record was retrieved. Neither
the 1982 accession nor the 1996 temporary delegation to Abdullah is sourced. Continuity
between 1990 and 2005 is not observed.

### SA-EXEC-02 — Abdullah's accession and Sultan's selection, 2005

Evidence: the Arabic Royal Court statement dated 26 Jumada II 1426 / 1 August 2005, filed
11:06 Makkah time, two minutes after the death announcement.

- `sa_abdullah_family_pledge_20050801`: the family pledged allegiance to Crown Prince Abdullah
  as King under Article 5 of the Basic Law.
- `sa_sultan_cp_selected_20050801`: after the pledge, King Abdullah announced Sultan as Crown
  Prince under Article 5, and the family pledged to him.
- `sa_citizen_pledge_scheduled_20050803`: the citizens' pledge was to begin at Qasr al-Hukm,
  Riyadh, after noon prayers "the day after tomorrow, Wednesday", that is 3 August 2005.
- SPA's English release (`sa_family_pledge_en_translation_20050801`) attributes the text to "a
  statement released today". That dates the statement, not the pledge.

Decision: accepted, phrased as reported by the Royal Court statement dated 1 August 2005. The
statement reports the pledge in the past tense and does not say "today", so `attested_on` is
the statement date, not a separately evidenced pledge date. Holders: Abdullah (King) and
Sultan (Crown Prince) observed on 2005-08-01. Abdullah's crown-princeship ended with his
accession, but no separate date is given, so his Crown Prince holder has no `until`.

Limits: no coronation, oath or inauguration is described. The Wednesday citizens' pledge is
scheduled only; the SPA regional reports of 3 August 2005 were not read.

### SA-EXEC-03 — Sultan's death, Nayef's selection and Nayef's death, 2011-2012

Evidence:

- `sa_sultan_death_20111022`: Royal Court statement, Riyadh 24 Dhu al-Qa'da 1432 / 22 October
  2011. Crown Prince Sultan, also Deputy Prime Minister, Minister of Defence and Aviation and
  Inspector General, died at dawn that day, Saturday, outside the Kingdom after an illness.
- `sa_nayef_cp_order_a224`: Royal Order A/224 dated 29/11/1432 AH, dateline 27 October 2011.
  Having notified the chairman and members of the Allegiance Commission, King Abdullah chose
  Nayef as Crown Prince and appointed him Deputy Prime Minister and Minister of Interior.
- `sa_allegiance_law_a135_cited`: the preamble cites A/90 (Basic Law), A/13 (Council of
  Ministers Law), A/135 (Allegiance Commission Law) and item Third of A/135.
- `sa_nayef_death_20120616`: Royal Court statement, Jeddah 26 Rajab 1433 / 16 June 2012. Crown
  Prince Nayef died that day, Saturday, outside the Kingdom; the funeral was set for after
  Maghrib on the Sunday at the Grand Mosque in Makkah.

Decision: accepted. Sultan's holder ends on 2011-10-22 and Nayef's on 2012-06-16, each on the
stated day of death while styled Crown Prince. Nayef's holder is observed on the order date,
2011-10-27.

Limits: A/224 says the Commission was notified and records no vote. The release was filed at
00:51 Makkah time on 28 October 2011. The portal's `date_hijri` field (1432-12-01) reflects
that filing day, not the order date; the printed 29/11/1432 AH, which the dateline equates
with 27 October 2011, stands. The Gregorian dates of the cited instruments were not verified.

### SA-EXEC-04 — Salman's selection as Crown Prince, 2012

Evidence: `sa_salman_cp_order_a139`, Royal Order A/139 dated 28/7/1433 AH, issued that day
according to the dateline (Taif, 18 June 2012). King Abdullah chose Salman as Crown Prince
and appointed him Deputy Prime Minister and Minister of Defence, citing item Third of A/135
and the public interest.

Decision: accepted. Salman's Crown Prince holder is observed on 2012-06-18. Its `until`,
2015-01-23, rests on his own accession in the 23 January 2015 statement (SA-EXEC-05). That
statement then applies the crown-princeship vacancy provision. The end is not taken from
Muqrin's pledge.

Limits: A/139 mentions no Allegiance Commission vote or notification.

### SA-EXEC-05 — Abdullah's death; Salman and Muqrin receive the pledge, 2015

Evidence: SPA's English release dated Riyadh, Rabi II 3, 1436 / 23 January 2015, and its
Arabic original.

- `sa_abdullah_death_20150123`: King Abdullah died at one o'clock that morning, Friday
  3/4/1436 AH.
- `sa_salman_king_pledge_20150123`: Crown Prince Salman received the pledge as King under the
  Basic Law.
- `sa_muqrin_cp_pledge_20150123`: after that pledge, King Salman called for the pledge to
  Muqrin as Crown Prince under item Second of Royal Order A/86 (26/5/1435 AH), the vacancy
  provision, and Muqrin received it. The English release says "Royal Decree"; the Arabic
  original and A/160 say Royal Order.
- `sa_citizen_pledge_scheduled_20150123`: the citizens' pledge was set for after Isha that day
  at Qasr al-Hukm.
- `sa_abdullah_death_ar_dateline`: the Arabic original prints a dateline of 2 Rabi II 1436 /
  22 January 2015. Its portal time, 2015-01-22T23:34:02Z, is 02:34 Makkah time on 23 January,
  and its trailer reads 02:33 Makkah time. It was therefore filed after the 1 a.m. death under
  the previous day's dateline. The body date is used.

Decision: accepted. Abdullah's King holder ends on 2015-01-23 (death). Salman's King holder
and Muqrin's Crown Prince holder are observed on 2015-01-23. The pledge to Salman is not
timed. It follows the 1 a.m. death and precedes the 02:33 filing, so it falls on 23 January.

Limits: no coronation, oath or inauguration is described. The evening citizens' pledge is not
attested as held. A/86 was not read, and its Gregorian date was not verified.

### SA-EXEC-06 — The April 2015 changes

Evidence, all dated 10/7/1436 AH (Wednesday 29 April 2015):

- `sa_muqrin_relieved_a159`: Royal Order A/159, issued that day, cites Muqrin's letter of the
  same date and relieves him at his request of the crown princeship and the Deputy Premiership.
- `sa_mbn_cp_a159`: item Second chooses Mohammed bin Nayef as Crown Prince, Deputy Prime
  Minister, Minister of Interior and Chairman of the Council of Political and Security Affairs.
- `sa_mbs_deputy_cp_a160`: Royal Order A/160, on Mohammed bin Nayef's nomination and with the
  support of the "great majority" of the Commission, chooses Mohammed bin Salman as Deputy
  Crown Prince and Second Deputy Prime Minister. The preamble cites item Fourth of A/86.
- `sa_pledge_call_20150429`: the Royal Court called for the pledge after Isha that day
  (filed 06:33 Makkah time).
- `sa_pledge_held_20150429`: SPA reports, filed 21:45, that Muqrin pledged allegiance that
  evening at Qasr al-Hukm to both, and that princes, the Grand Mufti, scholars, ministers,
  officials and citizens also gave the pledge.

Decision: accepted. Order, relief, call and held ceremony are five separate claims. Muqrin's
holder ends on 2015-04-29 (relief). Mohammed bin Nayef's holder is observed on the order
date, not on the evening pledge. The Deputy Crown Prince order is cited on the `sa_crown`
institution and the Allegiance Commission. It is not a role, holder or Crown Prince claim.

Limits: no Commission vote is stated for the relief or for Mohammed bin Nayef's selection.
The "great majority" in A/160 is unquantified. A/52 was cited but not read.

### SA-EXEC-07 — The 21 June 2017 change

Evidence: the Arabic original of Royal Order A/255 dated 26/9/1438 AH, dateline Makkah 26
Ramadan 1438 / 21 June 2017, filed 06:23 Makkah time.

- `sa_mbn_relieved_20170621`: item First relieves Mohammed bin Nayef of the crown princeship
  and the posts of Deputy Prime Minister and Minister of Interior.
- `sa_mbs_cp_selected_20170621`: item Second chooses Mohammed bin Salman as Crown Prince and
  Deputy Prime Minister, continuing as Minister of Defence.
- `sa_allegiance_31_of_34_20170621`: the recital cites the Commission members' support by a
  great majority, 31 of 34.
- `sa_mbs_deputy_cp_date_discrepancy`: the recital dates A/160 "10/6/1436". SPA's English
  translation (`sa_orders_20170621_en_translation`, which prints no order number) has the same
  date. The 2015 A/160 original, A/159 and the pledge call all read 10/7/1436.
- `sa_pledge_call_20170621`: the pledge was called for after Taraweeh that day at Al-Safa
  Palace, Makkah. Its dateline misprints the year as 1437.
- `sa_mbn_pledge_to_mbs_20170621`: Mohammed bin Nayef pledged allegiance to Mohammed bin
  Salman "today" at Al-Safa Palace; the English translation omits "today".

Decision: accepted, with A/255 as the primary for all three order claims. The 29 April 2015
date of A/160 is kept, and 10/6/1436 is recorded as an error in the 2017 order text as SPA
published it, not an English translation error. Mohammed bin Nayef's holder ends on
2017-06-21 (relief). Mohammed bin Salman's holder is observed on 2017-06-21 and has no end.

Limits: the recital refers to justifications reviewed by the Commission without stating them.
The 31-of-34 count comes from the order, not from a Commission record. The general pledge
that evening is called but not attested as held. The other orders issued that day are not
imported.

### SA-EXEC-08 — The 2022 prime-minister exception and the latest observation

Evidence:

- `sa_spa_pm_2022` (existing, now read from a downloaded original, content hash
  `fdf5daa9…eaef`). All three existing claims match: `sa_mbs_pm_appointment`,
  `sa_king_cabinet_chair_exception` and `sa_salman_king_observation`. The release styles
  Mohammed bin Salman "the Crown Prince" in the Prime Minister order and in the Cabinet list.
  That fits his selection as Crown Prince on 21 June 2017 (SA-EXEC-07).
- `sa_basic_pm_rule` (existing): Article 56 makes the King Prime Minister. This claim rests on
  a search-index read of the official translation, not on a downloaded text. The 2022 order
  names the exception to Article 56 without restating it.
- Before 2022, the crown princes were styled or appointed Deputy Prime Minister: Abdullah in
  2005, and the orders of 2011, 2012, 2015 and 2017. This fits Article 56 until the 2022 exception.
- `sa_salman_king_obs_20260813`, `sa_mbs_cp_pm_obs_20260813`: SPA's English release dated
  Jeddah, 13 August 2026. King Salman issued royal orders that day. The Cabinet was reconstituted
  with its current members under Mohammed bin Salman, styled Crown Prince and Prime Minister.

Decision: consistent and accepted. The 2026 claims are added as claim-ID holders on `sa_king`,
`sa_crown_prince` and `sa_pm`, alongside the existing 2022 holders. No King is added as a
Prime Minister holder by inference from Article 56.

Limits: 13 August 2026 is 25 days before the cutoff. No uninterrupted term is inferred between
the 2017, 2022 and 2026 observations, or through 7 September 2026. The Arabic version of the
2026 release was not downloaded.

## Independent check and defect fixes

An independent checker re-downloaded all 17 dossier sources on 21 September 2026. The
bush41library page matched its recorded hash. Every SPA page matched its recorded byte count
and article-content hash but not its response hash. The checker verified all 29 claims it
reviewed and downloaded six further SPA items (one, 5766455156, a researcher lead). Its
defects were handled as follows.

| Defect | Severity | Handling |
|---|---|---|
| 10/6/1436 called "probably a translation error"; Arabic A/255 exists | high | **Applied.** Added `sa_spa_order_a255_20170621_ar`; claim rewritten as an error in the 2017 order text; 29 Apr 2015 kept |
| 2017 order claims rest on the English release | medium | **Applied.** A/255 is primary for the relief, selection and 31-of-34 claims, with its number and date in each locator; English kept as translation |
| Arabic Fahd death statement exists | medium | **Applied.** Added `sa_spa_fahd_death_20050801_ar` as primary; English kept for Abdullah's styles; no stated date of death, so no `until` |
| Fahd biography is indirect | low | **Applied.** Kept only to corroborate 26/6/1426 = 1 Aug 2005; the Royal Court statement is the holder's evidence |
| 1990 anchor is a foreign record | medium | **Applied in part.** Flagged as a foreign record and a web transcription. Declined: no new search for a Saudi 1990 record or the GPO edition in this packet (both listed as next work) |
| Response-hash cause misstated | low | **Applied.** Extracts attribute the variance to `views_count` and `server_id` and record the article-content hash as the reproducible identity |
| A/86 called "Royal Decree" | low | **Applied.** "Royal Order A/86" in the claim and summary; the English wording is noted |
| 29 Apr 2015 pledge left unresolved | low | **Applied.** Added `sa_pledge_held_20150429`, separate from orders and call |
| 2017 pledge call and Arabic pledge report missing | low (optional) | **Applied.** Added `sa_pledge_call_20170621` (1437 dateline typo noted) and the Arabic original of Mohammed bin Nayef's pledge |
| 2005 family pledge date risks collapsing | low | **Applied.** Phrased as reported by the 1 Aug 2005 statement; no separate pledge date claimed |
| Article 56 reasoning uncited | low | **Applied.** `sa_basic_pm_rule` added to SA-EXEC-08 evidence, noting its search-index basis |
| Arabic 2015 dateline evidence omitted | info | **Applied.** Portal time and 02:33 trailer added to the uncertainty |
| A/224 portal date "differs" | info | **Applied.** Explained as the 28 Oct filing day; printed 29/11/1432 = 27 Oct stands |

## Sources added

| Source ID | What | Retained provenance |
|---|---|---|
| `sa_bush41_address_19900808` | [Public Papers address](https://www.bush41library.gov/digital-research-room/finding-aid/public-papers/address-nation-announcing-deployment-united-states), 8 Aug 1990 | 31,889 bytes, SHA-256 `539f1155…ba1c3`, identical in two downloads |
| `sa_spa_fahd_death_20050801_ar` | [Royal Court statement](https://www.spa.gov.sa/7be193c458) (Arabic; legacy 280265) | 173,207 bytes; content `cc562d7e…5327` |
| `sa_spa_fahd_death_20050801_en` | [English release](https://www.spa.gov.sa/en/35256ba2d1) | 164,267 bytes; content `d1354c57…e976` |
| `sa_spa_allegiance_20050801_ar` | [Royal Court accession statement](https://www.spa.gov.sa/a1252ad00a) (Arabic) | 172,480 bytes; content `7c029335…6eb67`; rendered in a browser |
| `sa_spa_allegiance_20050801_en` | [English release](https://www.spa.gov.sa/280269) | 169,616 bytes; content `7a3a8e9e…3b` |
| `sa_spa_fahd_bio_20050801` | [SPA biography](https://www.spa.gov.sa/280287) (Arabic) | 179,929 bytes; content `f7434392…a6ccd` |
| `sa_spa_sultan_death_20111022` | [Royal Court statement](https://www.spa.gov.sa/936277) | 173,837 bytes; content `656ce67d…6f0` |
| `sa_spa_nayef_cp_20111027` | [Royal Order A/224](https://www.spa.gov.sa/938356) | 174,548 bytes; content `01a8e542…a0a2` |
| `sa_spa_nayef_death_20120616` | [Royal Court statement](https://www.spa.gov.sa/1007895) | 173,562 bytes; content `5c4ef410…8d4` |
| `sa_spa_salman_cp_20120618` | [Royal Order A/139](https://www.spa.gov.sa/1008740) | 175,052 bytes; content `045a57f4…1347` |
| `sa_spa_abdullah_death_20150123_en` | [English release](https://www.spa.gov.sa/1319397) of both statements | 176,591 bytes; content `0d1f4e2c…078c` |
| `sa_spa_abdullah_death_20150123_ar` | [Arabic original](https://www.spa.gov.sa/1319360) | 181,502 bytes; content `152a9b76…7efb63` |
| `sa_spa_order_a159_20150429` | [Royal Order A/159](https://www.spa.gov.sa/1355272) | 182,370 bytes; content `460122e7…fd0dc` |
| `sa_spa_order_a160_20150429` | [Royal Order A/160](https://www.spa.gov.sa/1355273) | 183,113 bytes; content `0bfc8673…159e` |
| `sa_spa_pledge_call_20150429` | [Royal Court pledge call](https://www.spa.gov.sa/cac63786db) | 173,829 bytes; content `66e124f9…54bc` |
| `sa_spa_pledge_held_20150429` | [SPA report of the pledge](https://www.spa.gov.sa/5766455156) | 171,961 bytes; content `a560fe2c…2847` |
| `sa_spa_order_a255_20170621_ar` | [Royal Order A/255](https://www.spa.gov.sa/4058adbaf7) (Arabic; legacy 1641876) | 203,626 bytes; content `b0161db2…c28e` |
| `sa_spa_order_a255_20170621_en` | [English release](https://www.spa.gov.sa/1641911) | 173,485 bytes; content `31ec07b1…2e79`; rendered in a browser |
| `sa_spa_pledge_call_20170621` | [Royal Court pledge call](https://www.spa.gov.sa/2c7ada880e) (legacy 1641892) | 171,509 bytes; content `fdb11dad…7d51` |
| `sa_spa_mbn_pledge_20170621_ar` | [SPA report](https://www.spa.gov.sa/45ccd99bc4) (Arabic; legacy 1641897) | 170,053 bytes; content `35a5c175…176be` |
| `sa_spa_mbn_pledge_20170621_en` | [English release](https://www.spa.gov.sa/1642006) | 168,103 bytes; content `d301cc5f…2c5b` |
| `sa_spa_orders_20260813` | [English release](https://www.spa.gov.sa/en/N2653199) | 166,476 bytes; content `ddac3897…902b6` |
| `sa_spa_pm_2022` (existing) | [Three Royal Orders Issued](https://www.spa.gov.sa/2387811) | 182,652 bytes; content `fdf5daa9…eaef`; three downloads |

Each source has a checked-in derived factual extract under [sources/](sources/) named
`saudi-arabia-*-facts.json`. Its claims are identical to the packet's, and its own checksum
is recorded in the packet separately from any original-response identity. Original pages are
not checked in, and no emblem, photograph or expressive text is republished.

SPA pages embed a live `views_count` and a load-balancer `server_id` in their `__NEXT_DATA__`
JSON. Repeat downloads therefore have different SHA-256 values, usually with the same byte
count. Byte counts also differ between URL forms (A/255: 203,626 at `/4058adbaf7`, 204,437 at
`/1641876`), and they will change when the view count gains a digit. SPA response hashes are
therefore recorded as observations, not asserted (`source_response_sha256` is null). The SHA-256
of the UTF-8 `newsDetails.content` field was identical in every download and is the
reproducible identity (`article_content_sha256`). The researcher downloaded 16 SPA pages and the
checker re-downloaded them. Six SPA items were downloaded only by the checker, and this packet
re-read those local copies: `7be193c458`, `280269`, `5766455156`, `4058adbaf7`, `2c7ada880e`
and `45ccd99bc4`. `published_date` is the Makkah-time publication day from the portal
timestamp. For A/224 the printed dateline (27 Oct) is kept; the release went out at 00:52 on
28 October.

## Leads not imported

- Contemporary news on the 2005, 2011, 2012, 2015 and 2017 transitions (Al Jazeera, NPR,
  MEED, Arab News, Al Riyadh, Okaz, Asharq Al-Awsat, ABC), including commentary on Allegiance
  Council votes. News is a lead only; no vote detail beyond the royal orders' recitals is imported.
- Saudipedia entries on Fahd, Abdullah, Salman and Mohammed bin Salman: an encyclopedia, not
  a record of the events.
- The Bush-Fahd telephone memorandum of 2 August 1990, hosted by nsarchive.gwu.edu (a
  non-government archive). The official Public Papers were used instead.
- The GPO print edition of the 1990 Public Papers on govinfo.gov, which could replace the
  library transcription. Not retrieved.
- The White House statement on Nayef's selection (govinfo.gov, DCPD-201100802): a foreign
  reaction. The SPA primary was used.
- The Ministry of Finance republication of the 23 January 2015 statement
  (mof.gov.sa/mediacenter/news/Pages/23-1-2015.aspx). Official, but it repeats SPA; not fetched.
- SPA regional reports of 3 August 2005 on citizens' pledges (Hail, the Eastern Province and
  missions abroad). Not read; they could show whether the scheduled pledge took place.
- The Arabic version of the 13 August 2026 release (SPA N2653088), listed by the portal and
  not downloaded.
- Fahd's temporary delegation of state affairs to Crown Prince Abdullah, reported for January
  to February 1996. It needs a dated royal record.

## Sources attempted

- `https://my.gov.sa/en/news/4566` (National Portal, April 2015 changes): HTTP 403 (Cloudflare) to curl.
- `https://www.saudiembassy.net/`: HTTPS connection failure; HTTP returned 404.
- `https://www.boe.gov.sa/`, `https://www.mofa.gov.sa/`, `https://www.shura.gov.sa/`: connection
  failed or timed out.
- SPA site search in a browser tab: the researcher did not locate the Arabic originals of the
  2017 order and the 2005 death statement. The checker located both (added above).
- `https://www.uqn.gov.sa/` (Umm al-Qura gazette): reachable, but the issues for these orders
  were not searched.

## Suggested next work orders

These are proposals for the integrator. They are not created in `work-orders.json`.

- `C01-SaudiArabia-EXEC-001`: Umm al-Qura texts of A/224, A/139, A/86, A/52, A/159, A/160 and
  A/255, and of the 2005 and 2015 accession statements, to cross-check the SPA texts and the
  10/6 against 10/7/1436 date of A/160.
- `C01-SaudiArabia-EXEC-002`: a separate Deputy Crown Prince role, if wanted, from the texts
  of A/86 (Muqrin), A/52 (Mohammed bin Nayef) and A/160 (Mohammed bin Salman). Not merged into
  Crown Prince.
- `C01-SaudiArabia-EXEC-003`: a Saudi record of King Fahd in 1990 (an SPA archive item or Umm
  al-Qura issue with an order in his name), and a dated record of the 1996 delegation and its end.
- `C01-SaudiArabia-EXEC-004`: whether the citizens' pledges of 3 August 2005 and 23 January
  2015 were held (SPA reports of those days).
- `C01-SaudiArabia-EXEC-005`: Allegiance Commission records, and the Bureau of Experts texts
  and Gregorian dates of A/90, A/13, A/135, A/86 and A/52.
- `C01-SaudiArabia-EXEC-006`: an official observation closer to 7 September 2026 than
  13 August (for example a Cabinet-session release chaired by the Crown Prince and Prime
  Minister), and the Arabic N2653088.

## Integration notes (outside this packet's file boundary)

- `research-index.json` is regenerated in a **separate commit** on this branch. New totals:
  83 sources and 1,646 claims (previously 61 and 1,613). Organization, institution, role,
  packet and batch counts are unchanged. Saudi Arabia still has two open batches,
  `C01-SaudiArabia-DISC-B001` and `-B002`.
- `research/README.md` and the Saudi paragraph of `C01/README.md` need the new totals and a
  pointer to this report. They are left for the integrator.
- No existing test pins Saudi totals, so no existing test changed. The new
  `test_saudi_executive_c01_06.py` pins them.
- `test_campaign_census.py` cannot run in this sparse worktree, because
  `spheres-sim/data/party_leaders.json` is not checked out. It fails the same way at the base
  commit. It was run in a `git archive` export of its inputs (the `spheres-sim` and
  `spheres-web` data, `tools/avatars` and `C01`) at the result commit, where all 16
  campaign tests passed.
- Atlas display: for a holder with `attested_on`, `observationDate` shows only "Observed on
  <date>" and does not show `until`. The end of each term therefore appears in the holder's
  note and uncertainty lines. It is also structured in `until` for machine readers. No UI code
  changed. Showing a stated `until` next to `attested_on` would be a small atlas follow-up.
- `sa_spa_pm_2022` changed only in its access metadata (`accessed_date` 2026-09-13 →
  2026-09-21, `access_method`), plus a new `scope_note` and `snapshot`. Its claims are
  byte-identical.

## Checks

```text
python -X utf8 tools/avatars/campaign_research.py
python -X utf8 tools/avatars/campaign_research.py --check
python -X utf8 -m unittest discover -s tools/avatars -p "test_*research*.py"
python -X utf8 -m unittest discover -s tools/avatars -p "test_tonga_*.py"
python -X utf8 -m unittest discover -s tools/avatars -p "test_campaign*.py"
python -X utf8 -m unittest discover -s tools/avatars -p "test_saudi_executive_c01_06.py"
node --test tools/ui/check_leadership_research_review.cjs
python tools/planning/workboard.py --check
git diff --check
```
