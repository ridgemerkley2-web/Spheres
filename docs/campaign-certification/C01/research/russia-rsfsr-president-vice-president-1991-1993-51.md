# RSFSR President and Vice-President 1991-1993 51: dated attestations, the September 1993 conflict and the Vice-President's release

Packet: **CLAUDE-C01-51**. State: **ready_for_review** (not complete).
Owner: Claude. Integrator/reviewer: Codex. Branch `claude/c01-ru-51`; claim commit `4b15a1d7` on `5ea4f8fc`, the head of
`codex/campaign-certification` (fetched again before the result commits; it had not moved). **Not stacked** (the held
CLAUDE-C01-28, which also edits `russia.json`, was not used as a base; this packet's additions are append-only and touch no
party role). Research access: 1 October 2026 (UTC); first download pass 11:38:57Z-11:45:53Z (four cards retried at 11:49:04Z-11:49:22Z), second pass 12:20:31Z-12:23:46Z (one document frame and three cards retried at 12:24:52Z-12:25:08Z). The historical cutoff
stays **7 September 2026**.

This packet reviews six observations, RU-RSP-01 to RU-RSP-06, in [russia.json](russia.json). It extends the two existing
roles `ru_rsfsr_president` and `ru_rsfsr_vice_president` of `ru_rsfsr_presidency` with **nine holder observations**
appended after the CLAUDE-C01-05 holders, which are unchanged, from **sixteen acts on the official legal portal**
(pravo.gov.ru, original editions) carrying **20 claims**. Two people are named as holders: Борис Николаевич Ельцин and
Александр Владимирович Руцкой. Five claims of 21-22 September 1993 (the declared termination of Yeltsin's powers, the declared
acting service and the President's decree rejecting it) are appended to `ru_president`, which gains **no holder**. No organization, institution or role is added;
no existing source, claim, holder, lifecycle or institution-level list is changed; no party office, game mapping, portrait or
avatar is touched. The parent scope (C01, C06, S23, WC1 and CP1) remains open.

## Outcome

| ID | Question | Decision |
|---|---|---|
| RU-RSP-01 | Dated attestations of both offices in 1991 | **Accepted:** decree 8 (19 Jul) and order 98-рп (19 Nov) signed "Президент РСФСР"; the Vice-President's own order 1-рв (29 Jul) signed "Вице-президент РСФСР А. Руцкой" (`attested_on` only). Order 98-рп's assignment of duties names the office only (claim) |
| RU-RSP-02 | Where does the RSFSR styling give way? | **Accepted, decisions open:** decree 316 (26 Dec 1991) is signed "Президент Российской Федерации" (styling claim) while decree 318 of the same day and honours decree 245-н (3 Jan 1992) are still signed "Президент РСФСР" (attestations); the Vice-President signs as "РСФСР" on 5 Dec 1991 (order 8-рв) and as "Российской Федерации" on 16 Jan 1992 (order 1-рв, filed on the only vice-presidency role) |
| RU-RSP-03 | The Vice-President in 1993 | **Accepted:** Supreme Soviet resolution 4825-I (16 Apr 1993) names "вице-президента Российской Федерации ... А. В. Руцкого" (`attested_on`) |
| RU-RSP-04 | 1 September 1993: suspension | **Accepted, one decision open:** decree 1328 names "Вице-президента Российской Федерации Руцкого Александра Владимировича" (`attested_on` 1 Sep 1993) and suspends him from his duties (claim only, not an end); decree 1398 (18 Sep) on how the Vice-President's powers are assigned is procedure |
| RU-RSP-05 | 21-22 September 1993: the "acting President" resolutions | **Claims only:** Presidium resolution 5779-I and Supreme Soviet resolutions 5780-I and 5781-I declare Yeltsin's powers terminated from 20:00 on 21 Sep and exercised by the Vice-President; decree 1410 declares that assumption unlawful. No holder, start or end on any role |
| RU-RSP-06 | 3 October 1993: the release | **Accepted, pending a ruling:** decree 1576 releases him "от должности вице-президента Российской Федерации" and is in force "с момента его подписания" (dated 3 Oct 1993): `until` 1993-10-03 on the 1 Sep observation. Its point 3 (succession to the Chairman of the Council of Ministers) is procedure |

### Holders appended

| # | Role | Name | `attested_on` | `from` | `until` | Basis |
|---|---|---|---|---|---|---|
| 1 | `ru_rsfsr_president` | Борис Николаевич Ельцин | 1991-07-19 | – | – | decree 8 signature |
| 2 | `ru_rsfsr_president` | Борис Николаевич Ельцин | 1991-11-19 | – | – | order 98-рп signature |
| 3 | `ru_rsfsr_president` | Борис Николаевич Ельцин | 1991-12-26 | – | – | decree 318 signature |
| 4 | `ru_rsfsr_president` | Борис Николаевич Ельцин | 1992-01-03 | – | – | decree 245-н signature |
| 5 | `ru_rsfsr_vice_president` | Александр Владимирович Руцкой | 1991-07-29 | – | – | order 1-рв (1991) signature |
| 6 | `ru_rsfsr_vice_president` | Александр Владимирович Руцкой | 1991-12-05 | – | – | order 8-рв signature |
| 7 | `ru_rsfsr_vice_president` | Александр Владимирович Руцкой | 1992-01-16 | – | – | order 1-рв (1992) signature |
| 8 | `ru_rsfsr_vice_president` | Александр Владимирович Руцкой | 1993-04-16 | – | – | resolution 4825-I |
| 9 | `ru_rsfsr_vice_president` | Александр Владимирович Руцкой | 1993-09-01 | – | 1993-10-03 | decree 1328 styling; decree 1576 release |

Names follow the C01-05 holders (resolutions 1595-I and 1596-I); the acts print "Б.Ельцин"/"Б. ЕЛЬЦИН" and "А. Руцкой",
and decree 1328 prints "Руцкого Александра Владимировича". **No `from`** is set: every observation is a signed act or an
official record naming the office. The only `until` is decree 1576's.

## Date ledger

| Day | Event | Claims |
|---|---|---|
| 19 Jul 1991 | Decree 8 signed "Президент РСФСР Б.Ельцин" | `ru_rsfsr_ukaz_8_signed_as_president_rsfsr_19910719` |
| 29 Jul 1991 | Vice-President's order 1-рв signed "Вице-президент РСФСР А. Руцкой" | `ru_rsfsr_vp_rasp_1rv_signed_as_vice_president_19910729` |
| 19 Nov 1991 | Order 98-рп signed "Президент РСФСР Б. ЕЛЬЦИН"; the Vice-President's duties assigned (office only) | `ru_rsfsr_rasp_98rp_*` (two claims) |
| 5 Dec 1991 | Vice-President's order 8-рв, RSFSR title | `ru_rsfsr_vp_rasp_8rv_signed_as_vice_president_19911205` |
| 26 Dec 1991 | Decree 316 signed "Президент Российской Федерации" (claim); decree 318 signed "Президент РСФСР" | `ru_ukaz_316_signed_as_president_rf_19911226`, `ru_rsfsr_ukaz_318_signed_as_president_rsfsr_19911226` |
| 3 Jan 1992 | Decree 245-н signed "Президент РСФСР" | `ru_rsfsr_ukaz_245n_signed_as_president_rsfsr_19920103` |
| 16 Jan 1992 | Vice-President's order 1-рв, new title | `ru_vp_rasp_1rv_signed_as_vice_president_rf_19920116` |
| 16 Apr 1993 | Resolution 4825-I names the Vice-President | `ru_vs_4825i_rutskoi_styled_vice_president_19930416` |
| 1 Sep 1993 | Decree 1328: styling (attestation); suspension from duties (claim) | `ru_ukaz_1328_*` (two claims) |
| 18 Sep 1993 | Decree 1398: the Vice-President's powers only by decree or order (procedure) | `ru_ukaz_1398_vice_president_powers_by_decree_only_19930918` |
| 21 Sep 1993 | Presidium 5779-I: Yeltsin's powers deemed terminated; the Vice-President began exercising them (claims) | `ru_vs_presidium_5779i_*` (two claims) |
| 22 Sep 1993 | Supreme Soviet 5780-I and 5781-I (from 20:00 on 21 Sep); decree 1410 (claims) | `ru_vs_5780i_*`, `ru_vs_5781i_*`, `ru_ukaz_1410_*` |
| 3 Oct 1993 | Decree 1576: release, in force on signature (`until`); succession rule (procedure) | `ru_ukaz_1576_*` (two claims) |

## Observations

### RU-RSP-01 — 1991: both offices under the RSFSR title

Evidence: decree 8 of 19 July 1991 (head of the Vice-President's Secretariat) and order 98-рп of 19 November 1991 (the
Vice-President's duties) are signed "Президент РСФСР"; the Vice-President's own order 1-рв of 29 July 1991 is signed
"Вице-президент РСФСР А. Руцкой". Decision: three dated observations (`attested_on` only; a signed act is never a
boundary, C01-37/C01-38). The appointees and assistants are not recorded. Order 98-рп's point 1 names the office, not a
person, so the assignment of duties is a claim. The August 1991 acts were not reviewed (next work).

### RU-RSP-02 — December 1991 to January 1992: the restyling

Evidence: the portal's listing of 20 December 1991 to 10 January 1992 titles every presidential act "Президента РСФСР" to 25
December; from 26 December both titles appear (RSFSR titles on 26 and 29 December 1991 and 3 January 1992, all others "Президента
Российской Федерации"). The texts read here confirm both on 26 December (decree 316 signed "Президент Российской Федерации Б.Ельцин"; decree 318 signed "Президент РСФСР Б.Ельцин"), and
honours decree 245-н of 3 January 1992 is still signed "Президент РСФСР". The Vice-President's order 8-рв (5 December 1991)
reads "РСФСР" and order 1-рв (16 January 1992) "Вице-президент Российской Федерации". Decision: decrees 318 and 245-н are
observations on `ru_rsfsr_president` because they print its title; decree 316 is a styling claim on that role, not an
observation on either presidential role (CLAUDE-C01-14 left the 1991-1996 `ru_president` question to the integrator,
C01-Russia-PRES-002). The 1992 order is filed on `ru_rsfsr_vice_president`, the only vice-presidency role, with its title
unchanged.

Limits: listing titles were read for the window; texts only for the acts cited. Whether RSFSR-styled signatures continue
after 10 January 1992 was not checked.

### RU-RSP-03 — 16 April 1993: the Supreme Soviet names the Vice-President

Evidence: resolution 4825-I takes note of the information of "вице-президента Российской Федерации, председателя
Межведомственной комиссии Совета безопасности ... А. В. Руцкого". Decision: an official record naming the office and the
person attests the office on its date (C01-33, C01-35). The commission chairmanship is not a role here.

### RU-RSP-04 — 1-18 September 1993: suspension and the rule on assigned powers

Evidence: decree 1328 temporarily suspends "Вице-президента Российской Федерации Руцкого Александра Владимировича" from his
duties, citing an investigation and the absence of assignments, in force on signature; decree 1398 provides that the
Vice-President exercises the President's powers only on a decree or order, and that doing so otherwise is unlawful.
Decision: decree 1328's styling is an attestation (`attested_on` 1 Sep 1993); the suspension is a separate claim and never
an end; decree 1398 is procedure. The Supreme Soviet's appeal against decree 1328 (resolution 5696-I) is a lead.

### RU-RSP-05 — 21-22 September 1993: the "acting President" resolutions

Evidence: Presidium resolution 5779-I (21 Sep) deems Yeltsin's powers terminated from the signing of decree 1400 and
recognises that the Vice-President began exercising them; Supreme Soviet resolution 5780-I (22 Sep) terminates his powers
"с 20 часов 00 минут 21 сентября 1993 года", and 5781-I has the Vice-President exercise them from that moment (its text
prints "статьи 21.6" and "21.11"); decree 1410 (22 Sep) declares the assumption unlawful and the acts issued in the
President's name void. Decision: claims only, all five appended to `ru_president`, the office whose powers they concern (as
CLAUDE-C01-14 filed the 1999-2000 acting service there), which has no 1991-1996 holder; this also keeps the C01-14 rule that
no source is shared between `ru_president` and the RSFSR roles. None is a holder, start or end, and none is used as an
attestation of the vice-presidency.

### RU-RSP-06 — 3 October 1993: the release

Evidence: decree 1576 recites the suspension and the appropriation of powers, releases "Руцкого А.В. от должности
вице-президента Российской Федерации" (point 1), discharges him from military service (point 2, not recorded), passes the
President's powers in a vacancy to the Chairman of the Council of Ministers (point 3, procedure) and enters into force "с
момента его подписания" (point 4); it is dated 3 October 1993. Decision: `until` 1993-10-03 on the 1 September observation,
reading the effect-on-signature clause as stating the effective day, as the accepted CLAUDE-C01-19 did for decrees 861 and
300. Fallback: no `until` (Decisions for Codex). No source reviewed states that the office itself was abolished.

## Sources added

All sixteen are original-edition document frames on the portal, read in full in Russian; each extract is
`docs/campaign-certification/C01/research/sources/russia-ips-<act>-facts.json`.

| Source | Act | nd | Claims | Extract (bytes, SHA-256) |
|---|---|---|---|---|
| `ru_rsfsr_ukaz_8_19910719` | [Decree 8, 19 Jul 1991](https://pravo.gov.ru/proxy/ips/?doc_itself=&nd=102012111&page=1&rdk=0) | 102012111 | 1 | 4,583 `32db2f74c976` |
| `ru_rsfsr_vp_rasp_1rv_19910729` | [Vice-President's order 1-рв, 29 Jul 1991](https://pravo.gov.ru/proxy/ips/?doc_itself=&nd=102012167&page=1&rdk=0) | 102012167 | 1 | 4,464 `41e09bbc2a98` |
| `ru_rsfsr_rasp_98rp_19911119` | [Order 98-рп, 19 Nov 1991](https://pravo.gov.ru/proxy/ips/?doc_itself=&nd=102013152&page=1&rdk=0) | 102013152 | 2 | 5,500 `e09af8004f59` |
| `ru_rsfsr_vp_rasp_8rv_19911205` | [Vice-President's order 8-рв, 5 Dec 1991](https://pravo.gov.ru/proxy/ips/?doc_itself=&nd=102013436&page=1&rdk=0) | 102013436 | 1 | 4,404 `ec941947fe7d` |
| `ru_ukaz_316_19911226` | [Decree 316, 26 Dec 1991](https://pravo.gov.ru/proxy/ips/?doc_itself=&nd=102013793&page=1&rdk=0) | 102013793 | 1 | 4,472 `e47f3e149d3f` |
| `ru_rsfsr_ukaz_318_19911226` | [Decree 318, 26 Dec 1991](https://pravo.gov.ru/proxy/ips/?doc_itself=&nd=102013783&page=1&rdk=0) | 102013783 | 1 | 4,569 `c21d51384ae2` |
| `ru_rsfsr_ukaz_245n_19920103` | [Decree 245-н, 3 Jan 1992](https://pravo.gov.ru/proxy/ips/?doc_itself=&nd=102013951&page=1&rdk=0) | 102013951 | 1 | 4,532 `78d028e431f9` |
| `ru_vp_rasp_1rv_19920116` | [Vice-President's order 1-рв, 16 Jan 1992](https://pravo.gov.ru/proxy/ips/?doc_itself=&nd=102014149&page=1&rdk=0) | 102014149 | 1 | 4,383 `40a8ca08eb53` |
| `ru_vs_res_4825i_19930416` | [Supreme Soviet resolution 4825-I, 16 Apr 1993](https://pravo.gov.ru/proxy/ips/?doc_itself=&nd=102022817&page=1&rdk=0) | 102022817 | 1 | 4,909 `c2a95d8f5bd8` |
| `ru_ukaz_1328_19930901` | [Decree 1328, 1 Sep 1993](https://pravo.gov.ru/proxy/ips/?doc_itself=&nd=102025933&page=1&rdk=0) | 102025933 | 2 | 5,776 `65382ddab61f` |
| `ru_ukaz_1398_19930918` | [Decree 1398, 18 Sep 1993](https://pravo.gov.ru/proxy/ips/?doc_itself=&nd=102026121&page=1&rdk=0) | 102026121 | 1 | 4,774 `a81362b6f21a` |
| `ru_vs_presidium_res_5779i_19930921` | [Presidium resolution 5779-I, 21 Sep 1993](https://pravo.gov.ru/proxy/ips/?doc_itself=&nd=102026142&page=1&rdk=0) | 102026142 | 2 | 5,766 `55b5342428cb` |
| `ru_vs_res_5780i_19930922` | [Supreme Soviet resolution 5780-I, 22 Sep 1993](https://pravo.gov.ru/proxy/ips/?doc_itself=&nd=102026171&page=1&rdk=0) | 102026171 | 1 | 4,595 `0b6bc9be70d6` |
| `ru_vs_res_5781i_19930922` | [Supreme Soviet resolution 5781-I, 22 Sep 1993](https://pravo.gov.ru/proxy/ips/?doc_itself=&nd=102026182&page=1&rdk=0) | 102026182 | 1 | 4,827 `d88aff6cfb8e` |
| `ru_ukaz_1410_19930922` | [Decree 1410, 22 Sep 1993](https://pravo.gov.ru/proxy/ips/?doc_itself=&nd=102026198&page=1&rdk=0) | 102026198 | 1 | 4,855 `496068269c69` |
| `ru_ukaz_1576_19931003` | [Decree 1576, 3 Oct 1993](https://pravo.gov.ru/proxy/ips/?doc_itself=&nd=102026474&page=1&rdk=0) | 102026474 | 2 | 5,677 `cc3fca6f8d8b` |

## Response identities and stability checks

HTTPS to pravo.gov.ru timed out from this environment (port 443), as in CLAUDE-C01-14 and C01-19, so each response was
fetched over HTTP with curl's default User-Agent, no Accept-Encoding header and no `--compressed`; the portal serves
windows-1251 HTML uncompressed. As in those packets, the packet `url` and each extract's `source_url` are the HTTPS form of the same path (the
register accepts only HTTPS, and `test_russia_research_s10h.py` pins `source_url` to the packet URL), while
`source_response_url` is the HTTP URL whose identity is recorded. Each document frame and its date-restricted single-hit card were downloaded twice, at least 30 minutes apart,
with identical bytes and SHA-256.

| Source | Document frame (bytes, SHA-256) | Card (bytes, SHA-256) | Pass 1 (frame / card) | Pass 2 (frame / card) |
|---|---|---|---|---|
| `ru_rsfsr_ukaz_8_19910719` | 23,766 `f5e3acab586f…0da047` | 3,062 `5c3a316d6104` | 11:38:57Z / 11:38:57Z | 12:20:31Z / 12:20:31Z |
| `ru_rsfsr_vp_rasp_1rv_19910729` | 6,964 `d0c0f32fdf8b…8e6656` | 2,897 `ed936312556f` | 11:39:09Z / 11:39:09Z | 12:20:36Z / 12:20:36Z |
| `ru_rsfsr_rasp_98rp_19911119` | 24,378 `edb848b7f433…252746` | 3,051 `56d962e559c3` | 11:39:19Z / 11:39:19Z | 12:20:41Z / 12:20:41Z |
| `ru_rsfsr_vp_rasp_8rv_19911205` | 6,940 `6e73e3505815…aef032` | 2,896 `60c76c34acfe` | 11:39:34Z / 11:49:04Z | 12:20:52Z / 12:20:52Z |
| `ru_ukaz_316_19911226` | 23,808 `07507eb83977…44b1cb` | 3,083 `f7dd214d892a` | 11:40:59Z / 11:49:08Z | 12:21:38Z / 12:21:38Z |
| `ru_rsfsr_ukaz_318_19911226` | 36,815 `7b73e65bb8d6…5a7467` | 3,127 `453a9993cfef` | 11:40:43Z / 11:40:43Z | 12:20:57Z / 12:24:52Z |
| `ru_rsfsr_ukaz_245n_19920103` | 24,066 `57f8bdc42ecb…591d00` | 3,072 `90c450cfe48a` | 11:42:04Z / 11:42:04Z | 12:21:44Z / 12:21:44Z |
| `ru_vp_rasp_1rv_19920116` | 7,014 `07b74b9cb96a…8bab3f` | 2,926 `2f964e3cd2f0` | 11:42:18Z / 11:49:16Z | 12:21:49Z / 12:21:49Z |
| `ru_vs_res_4825i_19930416` | 10,197 `0fd28136d89f…b04c04` | 3,362 `0bc66a0a2293` | 11:43:28Z / 11:49:22Z | 12:21:58Z / 12:21:58Z |
| `ru_ukaz_1328_19930901` | 24,813 `0349671ebb1b…eb9030` | 3,102 `e1cd83157926` | 11:44:26Z / 11:44:26Z | 12:22:02Z / 12:25:02Z |
| `ru_ukaz_1398_19930918` | 24,601 `9449113ef2bc…c8a61a` | 3,133 `4ee6f498c56d` | 11:44:46Z / 11:44:46Z | 12:22:47Z / 12:22:47Z |
| `ru_vs_presidium_res_5779i_19930921` | 8,650 `220cd093eedd…0d9e2c` | 3,073 `602873b91986` | 11:45:53Z / 11:45:53Z | 12:25:08Z / 12:23:46Z |
| `ru_vs_res_5780i_19930922` | 7,886 `371375cc83f2…0c651b` | 2,953 `379fc0c7a871` | 11:44:59Z / 11:44:59Z | 12:22:53Z / 12:22:53Z |
| `ru_vs_res_5781i_19930922` | 7,629 `c1fc0e6c9e1d…f68f6c` | 3,087 `b0a45abcef91` | 11:45:07Z / 11:45:07Z | 12:22:58Z / 12:22:58Z |
| `ru_ukaz_1410_19930922` | 25,150 `a155b8145211…db0cf5` | 3,093 `0d62bc320922` | 11:45:26Z / 11:45:26Z | 12:23:03Z / 12:23:03Z |
| `ru_ukaz_1576_19931003` | 27,293 `b7e2f7813bbb…332e18` | 3,174 `d3b03643b94b` | 11:45:43Z / 11:45:43Z | 12:23:08Z / 12:25:05Z |

Pass times are UTC on 1 October 2026, given as document frame / card. Requests that timed out (four cards in the first pass; three cards and one document frame in the second) were retried; the times shown are those of the successful downloads.

## Leads not imported

- Decree 193 of 26 February 1992 (the Vice-President's assignments: agrarian reform and military-asset sales; downloaded,
  10,069 bytes `971f682dbe06`): names the office only.
- The Constitutional Court's conclusion of 21 September 1993 on decree 1400 (portal nd 608916724; its date-only card lists
  eight acts) and decree 1400 itself concern `ru_president`.
- Supreme Soviet resolution 5696-I of 3 September 1993 (appeal against decree 1328), the presidential orders on the
  Vice-President's 1992 foreign visits (174-рп, 237-рп, 383-рп), his other assistant and adviser orders (2-рв to 7-рв of
  1991; 3-рв of 1992) and the salary resolutions of 1992-1993.
- The acts of the Tenth (Extraordinary) Congress of People's Deputies of 23-24 September 1993 and decree 1452 of 25
  September 1993 (cited by decree 1576).

## Sources attempted

- Portal title searches (title containing "вице-президент", 10 Jul 1991 - 31 Dec 1993, 29 hits; "Руцк*", 1 Jun 1991 - 31
  Dec 1994, 7 hits; "Президент*", 20 Sep - 5 Oct 1993) and date listings of 20-31 December 1991 and 1-10 January 1992 were
  used for discovery only; they are growing listings and are not recorded as sources.
- HTTPS to pravo.gov.ru timed out (port 443); HTTP was used instead. No other host was needed. No source was blocked or
  rate-limited, and no access control was met.

## Suggested next work orders

These are proposals for the integrator. They are not created in `work-orders.json`.

- `C01-Russia-RSP-001`: the August 1991 acts signed by the President and the Vice-President, and the printed Vedomosti and
  Rossiyskaya Gazeta pages cited on the portal cards.
- `C01-Russia-RSP-002`: a ruling on the RSFSR/Russian Federation styling overlap (26 December 1991 to at least 3 January
  1992), together with C01-Russia-PRES-002 (a dated 1992-1993 `ru_president` observation, or one retitled office).
- `C01-Russia-RSP-003`: a source stating whether and when the office of Vice-President was abolished (the 1993
  Constitution has no such office), and the Tenth Congress's acts of 23-24 September 1993 as claims.

## Decisions for Codex

1. **Release as `until` (RU-RSP-06).** Decree 1576 is in force "с момента его подписания" and dated 3 October 1993; it is read
   as stating the effective day (as C01-19 read decrees 861 and 300 for `from`). Under the strict rule of review 1739eccb the
   fallback is `until` null, with the release a dated claim.
2. **Suspension decree as an attestation (RU-RSP-04).** Decree 1328's styling of the suspended Vice-President is an
   attestation (`attested_on` 1993-09-01). Fallback: a claim only, with the end moved to the 16 April 1993 observation.
3. **RSFSR-styled signatures after the renaming (RU-RSP-02).** Decrees 318 (26 Dec 1991) and 245-н (3 Jan 1992) print
   "Президент РСФСР" and are observations on `ru_rsfsr_president`, though after the gap ledger's window for the role
   (to 25 Dec 1991). Fallback: styling claims only.
4. **Decree 316's new title (RU-RSP-02)** is a claim on `ru_rsfsr_president`, not a `ru_president` observation, leaving
   C01-Russia-PRES-002 open.
5. **The Vice-President under the new title (RU-RSP-02 to 06).** The 1992-1993 observations ("Вице-президент Российской
   Федерации") are filed on `ru_rsfsr_vice_president`, whose title is unchanged; no role is added.
6. **September 1993 claims on `ru_president` (RU-RSP-05).** The declared termination, the declared acting service and decree
   1410 are five claims on `ru_president` (claims only, no holder); the alternative is the two RSFSR roles, which would
   share sources with `ru_president` against the C01-14 separation rule.
7. **HTTP identities.** As in C01-14 and C01-19, the recorded identities are those of the HTTP responses
   (`source_response_url`); `source_url` stays the HTTPS packet URL, which `packet_check.py` cannot reach from this
   environment (see Checks).

## Integration notes (outside this packet's file boundary)

- **Not stacked.** Base `5ea4f8fc`; `codex/campaign-certification` was fetched again before the result commits and had not
  moved. CLAUDE-C01-28 (held) also edits `russia.json` on its own branch: this packet appends sources at the end of the
  source list, holders and claims at the end of the three presidency roles' lists, three institution coverage notes and one
  packet note, and touches no party role, so the two diffs should combine without overlap apart from the trailing positions
  of `sources` and `coverage.unresolved`.
- `research-index.json` is regenerated in a **separate commit** (the only file shared with the other parallel C01
  packets). New totals: 2,063 sources and 5,030 claims (previously 2,047 and 5,010); Russia's source claims 322→342, role observations (9) and `mapping_pending` (21) unchanged. If another packet lands first, regenerate the index rather than merging it.
- The gap ledger is not touched. `ru_rsfsr_vice_president` has no applicability `until` in `certified_gap_ledger.py`;
  decree 1576's day could anchor one if the end is accepted. Observations 3-4 of `ru_rsfsr_president` fall after its
  window (to 25 Dec 1991).
- Existing tests updated by exact re-expression, none loosened: `test_russia_research_s10h.py` (totals 177→193 sources and 322→342 claims; C01-46's slice bounded to `[170:177]`; C01-51's 16 legal-portal sources pinned with access date 1 October 2026), `test_ussr_russia_transition_c01_05.py` (the no-end guard admits only decree 1576's stated end on the vice-presidency; the C01-05 holders are the first of five and six), `test_russia_presidents_c01_14.py` (the C01-05 holders followed by this packet's exact nine; `ru_president`'s lists followed by this packet's four sources and five claims; its seven notes at `[-10:-3]`; 342 index claims), `test_russia_heads_of_government_c01_19.py` (presidency holder names; the exact list of ends; totals), `test_russia_duma_faction_heads_c01_46.py` (other-role holder counts 5 and 6; totals; its sources at `[170:177]`) and `test_ussr_government_supreme_soviet_c01_26.py` (outside the pin list but in the test patterns: a third hash pins this packet's nine observations; the base hash is unchanged).
- The new test `test_russia_rsfsr_presidency_c01_51.py` pins the holders with (attested_on, from, until), every claim's
  date, kind, role and observation, every response and card identity, the extracts and snapshots, the separation from
  `ru_president`, the Government and the faction roles, and 26 mutations.

## Checks

Run on 1 October 2026 (UTC) on the packet tree before the commits:

- `campaign_research.py` regenerated and `--check` passes (2,063 sources and 5,030 claims).
- `campaign_census.py --check` passes (exit 0).
- Russia tests (44, 7 of them new), USSR tests (48), research tests (79) and campaign tests (16, census included): OK.
- Atlas Node check (`check_leadership_research_review.cjs`): 11 pass, 0 fail.
- `workboard.py --check`: PASS (44 canonical markers) after adding `docs/campaign-certification/S26` to this worktree's
  sparse checkout (the base references `S26/preparation/RECRUITMENT.md`).
- `git diff --check`: clean.
- The new test's 26 mutations each fail as intended.
- A third HTTP download of every document frame and card (12:35-12:40Z, at least 49 minutes after the first) matched all
  16 sources byte for byte; three requests needed one retry after a timeout.
- `packet_check.py 51` is run after the push; HTTPS to pravo.gov.ru is unreachable from this environment, so its re-fetch
  of each extract's HTTPS `source_url` is expected to fail with HTTP 000 (see Decisions for Codex, item 7). Its summary is
  returned with the submission.

Known failures outside the suite, not fixed: `test_certified_gap_ledger.py` ("Source ru_rsfsr_ukaz_8_19910719 (Russia) has
no pinned attribution" until Codex classifies this commit) and `test_certified_boundary_matrix.py` (needs
`spheres-web/src/person_avatar_assets.rs`, absent from the sparse checkout).
