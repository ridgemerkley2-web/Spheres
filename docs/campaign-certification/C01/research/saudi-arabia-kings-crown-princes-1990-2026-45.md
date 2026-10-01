# Saudi kings and crown princes 1990-2026 45: Saudi primary attestations, and the Allegiance Commission secretary

Packet: **CLAUDE-C01-45**. State: **ready_for_review** (not complete).
Owner: Claude. Integrator/reviewer: Codex. Branch `claude/c01-sa-45`; claim commit `a529ab8d` (pushed) on base `509bd289`;
not stacked. Integration has since moved twice: `a81d2486` was merged at `a342c6f6` and `d0c6676b`
(`codex/campaign-certification`) at `7761abe4`, both before the packet commit. Research, downloads and checks on 30 September 2026 (US Pacific; downloads on 1 October 2026 UTC). The
historical cutoff stays **7 September 2026**.

This packet reviews nine observations, SA-KCP-01 to SA-KCP-09, in [saudi-arabia.json](saudi-arabia.json). It adds Saudi
primary attestations alongside the CLAUDE-C01-06 holders of `sa_king` and `sa_crown_prince` (institution `sa_crown`) and
fills the empty role `sa_succession_secretary` (Secretary General) of `sa_succession_commission` with one holder. It adds
18 sources and 30 claims, one derived extract per source, 17 dated holder observations (7 on `sa_king`, 10 on
`sa_crown_prince`), one holder tenure, a scope note on `sa_succession_secretary` and five coverage items (four on
`sa_crown`, one on `sa_succession_commission`). The eight CLAUDE-C01-06 holders are unchanged: the new observations are
appended after the existing `holder_claims`, and no `attested_on`, `from` or `until` moves. It adds no organization,
institution or role, and touches neither `sa_pm` nor the CLAUDE-C01-25 chair roles. Nine people are named as holders or
observed holders. No game mapping, lifespan, portrait or avatar is added. The parent scope (C01, C06, S23, WC1 and CP1)
remains open.

Sources are Saudi only: the Saudi Press Agency (SPA) portal's news-detail JSON (16 items, read in Arabic and quoted as
printed) and two Internet Archive captures (2000 and 1999) of the Royal Embassy of Saudi Arabia's English releases of
January and February 1996, the earliest Saudi releases online for the period. Foreign records stay leads.

## Outcome

| ID | Question | Decision |
|---|---|---|
| SA-KCP-01 | King Fahd, 1990-2005, in Saudi records | **Accepted in part:** styled King in the Royal Embassy's release of the decree of 1 January 1996, chairing the cabinet on 12 February 1996, and issuing Royal Order A/193 of 25/6/1426 AH (31 July 2005), whose signature block reads 'فهد بن عبدالعزيز' then 'عنه / عبدالله بن عبدالعزيز'. Three observations after his holder; no Saudi record of 1990-1995 was found online |
| SA-KCP-02 | The 1996 delegation of state affairs | **Accepted as claims only:** the decree of 1 January 1996 delegating the Crown Prince 'to undertake the affairs of the state'; its end, King Fahd's message of 21 February 1996 citing 'royal decree A-112 dated 11/30/95' and the reply citing 'A-199 dated 02/21/96'. Acting service, never a boundary |
| SA-KCP-03 | Crown Prince Abdullah before August 2005 | **Accepted in part:** styled Crown Prince on 1 January 1996 and on 30 July 2005. Both observations predate his CLAUDE-C01-06 holder's `attested_on` (2005-08-01), which is kept; whether to re-date that holder is a ruling question |
| SA-KCP-04 | King Abdullah's reign | **Accepted:** the citizens' pledge to him and to Crown Prince Sultan held at Qasr al-Hukm on 3 August 2005 (claim only; it resolves the CLAUDE-C01-06 scheduled pledge); styled King by A/136 (20 October 2006) and A/145 (20 May 2014) |
| SA-KCP-05 | Crown Princes Sultan, Nayef and Salman | **Accepted:** Sultan in A/175 (29 October 2007, with a deputation); Nayef at the Hajj reception of 7 November 2011; Salman on 18 June 2012 (his directive on the pledge, with a scheduled pledge on 3-4/8/1433 AH) and in A/145 (20 May 2014, with a deputation) |
| SA-KCP-06 | King Salman's accession pledge and Crown Prince Muqrin | **Accepted:** the citizens' pledge held after Isha on 23 January 2015 (claim only; it resolves the CLAUDE-C01-06 scheduled pledge); Muqrin styled Crown Prince on 28 April 2015, the day before A/159 |
| SA-KCP-07 | Crown Prince Mohammed bin Nayef | **Accepted:** styled Crown Prince in A/267 (25 July 2015) and A/128 (25 February 2017), each a deputation during the King's absence; King Salman styled King by A/267 |
| SA-KCP-08 | Crown Prince Mohammed bin Salman and King Salman before the cutoff | **Accepted in part:** the pledge to him held at Al-Safa Palace on the evening of 21 June 2017 (claim only); styled Crown Prince and Prime Minister chairing the cabinet on 1 September 2026; King Salman issuing an order reported on 4 September 2026. Continuity to 7 September 2026 is not established |
| SA-KCP-09 | Secretary General of the Allegiance Commission | **Accepted in part:** Royal Order A/136 of 26/9/1427 AH (Royal Court statement, 20 October 2006) appoints 'معالي الأستاذ / خالد بن عبدالعزيز التويجري أمينا عاما لهيئة البيعة' (`attested_on`; no effective day). Article 24 of the law is procedure. No relief, later attestation or successor was found, so `until` is null |

### Holder ledger

The CLAUDE-C01-06 holders are not changed. Each observation below is a claim ID appended, in date order, at the end of the
role's `holder_claims`, after the existing `sa_salman_king_obs_20260813` and `sa_mbs_cp_pm_obs_20260813`.

| Role | Holder (CLAUDE-C01-06 dates kept) | New observations (attested_on) |
|---|---|---|
| `sa_king` | Fahd bin Abdulaziz Al Saud (1990-08-08, -, -) | `sa_fahd_king_obs_19960101` (1996-01-01), `sa_fahd_chairs_cabinet_19960212` (1996-02-12), `sa_fahd_king_order_a193_20050731` (2005-07-31) |
| `sa_king` | Abdullah bin Abdulaziz Al Saud (2005-08-01, -, 2015-01-23) | `sa_abdullah_king_order_a136_20061020` (2006-10-20), `sa_abdullah_king_order_a145_20140520` (2014-05-20) |
| `sa_king` | Salman bin Abdulaziz Al Saud (2015-01-23, -, -) | `sa_salman_king_order_a267_20150725` (2015-07-25), `sa_salman_king_obs_20260904` (2026-09-04) |
| `sa_crown_prince` | Abdullah bin Abdulaziz Al Saud (2005-08-01, -, -) | `sa_abdullah_cp_obs_19960101` (1996-01-01), `sa_abdullah_cp_obs_20050730` (2005-07-30), both before the holder's `attested_on` |
| `sa_crown_prince` | Sultan bin Abdulaziz Al Saud (2005-08-01, -, 2011-10-22) | `sa_sultan_cp_obs_a175_20071029` (2007-10-29) |
| `sa_crown_prince` | Nayef bin Abdulaziz Al Saud (2011-10-27, -, 2012-06-16) | `sa_nayef_cp_obs_20111107` (2011-11-07) |
| `sa_crown_prince` | Salman bin Abdulaziz Al Saud (2012-06-18, -, 2015-01-23) | `sa_salman_cp_obs_20120618` (2012-06-18), `sa_salman_cp_obs_a145_20140520` (2014-05-20) |
| `sa_crown_prince` | Muqrin bin Abdulaziz Al Saud (2015-01-23, -, 2015-04-29) | `sa_muqrin_cp_obs_20150428` (2015-04-28) |
| `sa_crown_prince` | Mohammed bin Nayef bin Abdulaziz Al Saud (2015-04-29, -, 2017-06-21) | `sa_mbn_cp_obs_a267_20150725` (2015-07-25), `sa_mbn_cp_obs_a128_20170225` (2017-02-25) |
| `sa_crown_prince` | Mohammed bin Salman bin Abdulaziz Al Saud (2017-06-21, -, -) | `sa_mbs_cp_obs_20260901` (2026-09-01) |

New tenure on `sa_succession_secretary`: **Khalid bin Abdulaziz Al-Tuwaijri**, `attested_on` 2006-10-20 (A/136, the
Royal Court statement's day), `from` null (no effective day), `until` null (no relief or end stated), claim
`sa_tuwaijri_sg_appointed_a136_20061020`.

Holder names keep the CLAUDE-C01-06 English forms; the Secretary General's is SPA's usual English form without honorific.
Claim text quotes the Arabic as printed, and each extract row with a holder keeps the printed personal name in
`printed_name`. Pledge ceremonies, delegations and deputations of state affairs, their end, the signature on the King's
behalf and the scheduled pledge are institution-level claims on `sa_crown` (`role_id` null), never holder evidence.

### Date ledger

| Date | Event | Claim |
|---|---|---|
| 1 Jan 1996 | King Fahd's decree delegating the Crown Prince to undertake the affairs of state; both styled | `sa_fahd_delegation_19960101`, `sa_fahd_king_obs_19960101`, `sa_abdullah_cp_obs_19960101` |
| 12 Feb 1996 | King Fahd chairs the cabinet ('yesterday evening', release of 13 February 1996) | `sa_fahd_chairs_cabinet_19960212` |
| 21 Feb 1996 | King Fahd's message ending the delegation; A-199 dated '02/21/96' | `sa_fahd_delegation_ended_19960221` |
| 24/6/1426 AH = 30 Jul 2005 | Crown Prince Abdullah receives the Russian ambassador | `sa_abdullah_cp_obs_20050730` |
| 25/6/1426 AH = 31 Jul 2005 | A/193 in King Fahd's name, signed on his behalf | `sa_fahd_king_order_a193_20050731`, `sa_a193_signed_on_behalf_20050731` |
| 28/6/1426 AH = 3 Aug 2005 | Citizens' pledge to King Abdullah and Crown Prince Sultan held 'today' | `sa_citizen_pledge_held_20050803` |
| 26/9/1427 AH; 28/9/1427 = 20 Oct 2006 | A/136 appoints the Secretary General (statement of 20 October); Article 24 | `sa_tuwaijri_sg_appointed_a136_20061020`, `sa_abdullah_king_order_a136_20061020`, `sa_allegiance_law_art24_secretary` (undated) |
| 17/10/1428 AH = 29 Oct 2007 | A/175 deputizes Crown Prince Sultan | `sa_sultan_cp_obs_a175_20071029`, `sa_sultan_deputized_a175_20071029` |
| 11/12/1432 AH = 7 Nov 2011 | Crown Prince Nayef holds the Hajj reception 'today' | `sa_nayef_cp_obs_20111107` |
| 28/7/1433 AH = 18 Jun 2012 | Crown Prince Salman's directive on the pledge; his pledge set for 3-4/8/1433 AH | `sa_salman_cp_obs_20120618`, `sa_salman_cp_pledge_scheduled_20120618` |
| 21/7/1435 AH = 20 May 2014 | A/145 deputizes Crown Prince Salman | `sa_abdullah_king_order_a145_20140520`, `sa_salman_cp_obs_a145_20140520`, `sa_salman_deputized_a145_20140520` |
| 3/4/1436 AH = 23 Jan 2015 | Pledge held after Isha to King Salman and Crown Prince Muqrin | `sa_citizen_pledge_held_20150123` |
| 9/7/1436 AH = 28 Apr 2015 | Crown Prince Muqrin receives the Malaysian Chief of General Staff | `sa_muqrin_cp_obs_20150428` |
| 9/10/1436 AH = 25 Jul 2015 | A/267 deputizes Crown Prince Mohammed bin Nayef | `sa_salman_king_order_a267_20150725`, `sa_mbn_cp_obs_a267_20150725`, `sa_mbn_deputized_a267_20150725` |
| 28/5/1438 AH = 25 Feb 2017 | A/128 deputizes Crown Prince Mohammed bin Nayef | `sa_mbn_cp_obs_a128_20170225`, `sa_mbn_deputized_a128_20170225` |
| 26/9/1438 AH = 21 Jun 2017 | Pledge to Mohammed bin Salman held that evening at Al-Safa Palace | `sa_mbs_pledge_held_20170621` |
| 19/3/1448 AH = 1 Sep 2026 | The Crown Prince and Prime Minister chairs the cabinet 'today' | `sa_mbs_cp_obs_20260901` |
| 22/3/1448 AH = 4 Sep 2026 | SPA reports a royal order issued by King Salman | `sa_salman_king_obs_20260904` |

Hijri and Gregorian dates are as each date line prints them. Every new claim has `attested_on` only (the Article 24 claim
has none); no new `from` or `until` is set anywhere.

## Observations

### SA-KCP-01 — King Fahd, 1990-2005, in Saudi records

Evidence: `sa_fahd_king_obs_19960101` (the Royal Embassy's release of 1 January 1996: 'The Custodian of The Two Holy
Mosques King Fahd Bin Abdulaziz directed the following Royal Decree'); `sa_fahd_chairs_cabinet_19960212` (release of
February 13, 1996: he 'yesterday evening chaired' the cabinet); `sa_fahd_king_order_a193_20050731` (SPA, 31 July 2005:
A/193 of 25/6/1426 AH opens 'نحن فهد بن عبدالعزيز آل سعود ملك المملكة العربية السعودية'); `sa_a193_signed_on_behalf_20050731`
(signature block 'فهد بن عبدالعزيز' / 'عنه / عبدالله بن عبدالعزيز').

Decision: accepted in part. Three dated observations follow his holder; his holder keeps `attested_on` 1990-08-08 (a
foreign record) and a null `until`. The signature on his behalf is a claim only: the order gives no reason, and reading
the signatory as the Crown Prince would be a name match.

Limits: the SPA portal holds dense items only from 2004 (a few 2002-2003 items carry wrong publication stamps), and the
Royal Embassy's releases begin in 1996; no Saudi record of 1990-1995, of his 1982 accession or of a stated day of death was
found.

### SA-KCP-02 — The 1996 delegation of state affairs

Evidence: `sa_fahd_delegation_19960101` (the decree, 'after reviewing article 65 of the basic system of Government',
delegates the Crown Prince 'to undertake the affairs of the state while we enjoy rest and recuperation');
`sa_fahd_delegation_ended_19960221` (item 'February 22' on the February 1996 page: King Fahd's message of 'yesterday',
'at the end of the validity of royal decree A-112 dated 11/30/95', and the reply citing 'your royal decree A-199 dated
02/21/96').

Decision: accepted as claims only. A delegation of the King's functions is acting service; it is not a vacancy and never a
boundary of either office.

Limits: English releases only; the Arabic decrees and their Hijri dates were not read. The February 22 item prints no
year (1996 rests on the page heading and A-199's date), and '11/30/95' is quoted as printed, not reconciled with the
January 1 release. The cabinet session of 12 February 1996 falls inside the delegation as dated; the releases do not
explain it.

### SA-KCP-03 — Crown Prince Abdullah before August 2005

Evidence: `sa_abdullah_cp_obs_19960101` ('Crown Prince His Royal Highness Prince Abdullah Bin Abdulziz, the deputy Premier
and head of the National Guard'); `sa_abdullah_cp_obs_20050730` (SPA, 30 July 2005: 'استقبل صاحب السمو الملكي الأمير عبدالله
بن عبدالعزيز ولي العهد نائب رئيس مجلس الوزراء رئيس الحرس الوطني' ... 'اليوم').

Decision: accepted in part. Both are appended as observations of his existing Crown Prince holder, although they predate
its `attested_on` (2005-08-01). The holder is kept as the scope requires; moving its `attested_on` to 1996-01-01 is a
ruling for Codex.

Limits: his 1982 selection as Crown Prince is outside the sources read.

### SA-KCP-04 — King Abdullah's reign

Evidence: `sa_citizen_pledge_held_20050803` (SPA, 3 August 2005: the King and 'صاحب السمو الملكي الأمير سلطان بن عبدالعزيز
ولي العهد' received the pledge 'في قصر الحكم بالرياض اليوم'); `sa_abdullah_king_order_a136_20061020` and
`sa_abdullah_king_order_a145_20140520` (orders opening 'نحن عبدالله بن عبدالعزيز آل سعود ملك المملكة العربية السعودية').

Decision: accepted. The held pledge resolves CLAUDE-C01-06's open question whether the scheduled pledge of Wednesday 3
August 2005 took place; it is a ceremony and never holder evidence. The two orders are observations of his King holder.

Limits: the pledge item ends 'to follow'; its additions were not read.

### SA-KCP-05 — Crown Princes Sultan, Nayef and Salman

Evidence: `sa_sultan_cp_obs_a175_20071029` and `sa_sultan_deputized_a175_20071029` (A/175 of 17/10/1428 AH: 'صاحب السمو
الملكي الأخ الأمير سلطان بن عبدالعزيز ولي العهد' deputized under Article 66 for the King's absence from that day);
`sa_nayef_cp_obs_20111107` (Mina, 7 November 2011: 'صاحب السمو الملكي الأمير نايف بن عبدالعزيز آل سعود ولي العهد ...' held the
Hajj reception on the King's behalf); `sa_salman_cp_obs_20120618` and `sa_salman_cp_pledge_scheduled_20120618` (18 June
2012: the Crown Prince directs the regional emirs to receive the pledge on his behalf and will receive it at Qasr al-Hukm
on 'السبت والأحد 3 و 4/8/1433هـ'); `sa_salman_cp_obs_a145_20140520` and `sa_salman_deputized_a145_20140520` (A/145).

Decision: accepted. Each styling is an observation of the matching holder; deputations and the scheduled pledge are
claims only.

Limits: no held pledge to Nayef (2011) or to Salman (1433 AH) was found in SPA searches; the 3-4/8/1433 AH days are not
converted.

### SA-KCP-06 — King Salman's accession pledge and Crown Prince Muqrin

Evidence: `sa_citizen_pledge_held_20150123` (SPA, 23 January 2015: King Salman and 'صاحب السمو الملكي الأمير مقرن بن عبدالعزيز
آل سعود ولي العهد نائب رئيس مجلس الوزراء' received the pledge 'بعد صلاة العشاء لهذا اليوم الجمعة 3 ربيع الآخر 1436 هـ' at Qasr
al-Hukm); `sa_muqrin_cp_obs_20150428` (28 April 2015, styled Crown Prince receiving the Malaysian Chief of General Staff
'اليوم').

Decision: accepted. The held pledge resolves the second CLAUDE-C01-06 scheduled pledge and stays a ceremony; the 28 April
item is an observation, never Muqrin's end, which stays A/159.

### SA-KCP-07 — Crown Prince Mohammed bin Nayef

Evidence: `sa_salman_king_order_a267_20150725`, `sa_mbn_cp_obs_a267_20150725` and `sa_mbn_deputized_a267_20150725` (A/267
of 9/10/1436 AH); `sa_mbn_cp_obs_a128_20170225` and `sa_mbn_deputized_a128_20170225` (A/128 of 28/5/1438 AH).

Decision: accepted: two observations of his Crown Prince holder, one of King Salman's, two deputations as claims.

### SA-KCP-08 — Crown Prince Mohammed bin Salman and King Salman before the cutoff

Evidence: `sa_mbs_pledge_held_20170621` (SPA, Makkah 21 June 2017: 'تلقى صاحب السمو الملكي الأمير محمد بن سلمان بن عبدالعزيز
المبايعة وليًا للعهد' ... 'مساء اليوم في قصر الصفا بمكة المكرمة'); `sa_mbs_cp_obs_20260901` (1 September 2026: 'رأس صاحب السمو
الملكي الأمير محمد بن سلمان بن عبدالعزيز آل سعود ولي العهد رئيس مجلس الوزراء' the cabinet session 'اليوم');
`sa_salman_king_obs_20260904` (4 September 2026: 'أصدر خادم الحرمين الشريفين الملك سلمان بن عبدالعزيز آل سعود ... أمرًا ملكيًا').

Decision: accepted in part. The held pledge is a ceremony, kept apart from A/255 and the call; the two 2026 items are point
observations three to six days before the cutoff. Continuity to 7 September 2026 is not established.

Limits: the pledge item was filed at 01:30 Makkah time on 22 June 2017 under the 21 June date line; the printed date line
is kept, as CLAUDE-C01-06 kept A/224's. The 4 September item reports an order without its text or issue day.

### SA-KCP-09 — Secretary General of the Allegiance Commission

Evidence: `sa_tuwaijri_sg_appointed_a136_20061020` (SPA, Makkah 28 Ramadan 1427 / 20 October 2006: Royal Court statement
reproducing A/136 of 26/9/1427 AH, citing Article 24 of the law; item First: 'يعين معالي الأستاذ / خالد بن عبدالعزيز التويجري
أمينا عاما لهيئة البيعة'); `sa_allegiance_law_art24_secretary` (Article 24: 'يعين الملك أمينا عاما للهيئة', with his duties and
a deputy).

Decision: accepted in part. One holder: `attested_on` 2006-10-20, `from` null (no effective day), `until` null.

Limits: SPA searches (Arabic and English, 2006 to the cutoff) found no later styling of anyone as Secretary General of the
Commission. A/56 and A/57 of 3/4/1436 AH (23 January 2015) relieve him of the Royal Court and Royal Guard posts and do
not name the Commission, so they are not an end. The collective oath of 9 December 2007 (CLAUDE-C01-25,
`sa_allegiance_oath_20071209`) names the Secretary General without a name and is not reused.

## Sources added

Source IDs carry the publication date (for the Royal Embassy, the item's date). Each has one extract at
`sources/saudi-arabia-<id without sa_, hyphenated>-facts.json`.

| Source | What | Response bytes | Response SHA-256 |
|---|---|---|---|
| `sa_embassy_fahd_delegates_19960101` | Royal Embassy, January 1996 page, capture 20000930104650 | 62,784 | `383724bf0bb2b23e510f1f459862a4f761d904a51d0ab07ea11eb223617068f3` |
| `sa_embassy_fahd_resumes_19960222` | Royal Embassy, February 1996 page, capture 19991003024900 | 24,905 | `1b3bcaf97bdd44ceabaec4d9760f0e15bf18d065bbea79ba88c4d08bf8d80f8d` |
| `sa_spa_cp_abdullah_obs_20050730` | SPA `835ced88fa` | 2,428 | `538c33ba91ba710ae98dc41467a2f35574eab1d705de86f311b06727628aaf49` |
| `sa_spa_order_a193_20050731` | SPA `3a405a0abd`, A/193 | 2,869 | `6f89b870a9461c4fc0acac7be1fea895f6201f73f6bbd9f0c6827dd07167be5b` |
| `sa_spa_pledge_held_20050803` | SPA `e45e90b20d` | 2,143 | `1c0a88ea99f2da901121a14012499bc1a2bb3d7af0937e85c1b1e453e6c9efb4` |
| `sa_spa_order_a136_20061020` | SPA `f9cb0a7263`, A/136 | 2,420 | `7c4a3614486727eebeb91ee83a6b50d8b64596ae9ba2b44153ab3860e560a4ac` |
| `sa_spa_allegiance_law_art24_20061020` | SPA `76a355d60b`, law Articles 23-25 | 1,971 | `79b60c3a2abfa239dca5cf73fa73dcdd663c8af23f7f022daf76cdf36ef89639` |
| `sa_spa_order_a175_20071029` | SPA `869bc2b6be`, A/175 | 2,647 | `af1798d8e4db08fa31810590ea4add8f7dc002bdd1efc1e39da2953f0b75c47a` |
| `sa_spa_nayef_hajj_reception_20111107` | SPA `c4d376ee3e` | 2,002 | `13dc061370249299b21b43a5e4c74e3f42c5d4342b0243f2ee0cd278ebec0846` |
| `sa_spa_salman_cp_pledge_directive_20120618` | SPA `8c7207b976` | 2,151 | `2bf582b00d7e1235e8645fba089ac1ed411dba43d3baec1fd34d52a73ee61e8a` |
| `sa_spa_order_a145_20140520` | SPA `b913479674`, A/145 | 2,518 | `212b35ec541369eb2f4d4f30b8b5884db60eb45ed1d1af56f61a01b46ba68061` |
| `sa_spa_pledge_held_20150123` | SPA `7fbe706628` | 2,678 | `d8d7885c13a4090690a3fd58a8016f18270d7e34585b693f8b5541624ea371d5` |
| `sa_spa_muqrin_cp_obs_20150428` | SPA `71d56a9352` | 2,983 | `b905fc2cb7901e3d72c7bcc186368369268c1ec4a0e3923f378d66281353f8bd` |
| `sa_spa_order_a267_20150725` | SPA `2807a6725a`, A/267 | 2,327 | `1c718e8b83e7b5964a8417a673912b869eabb96955154b29da6eb1ec82aacad9` |
| `sa_spa_order_a128_20170225` | SPA `49f14049aa`, A/128 | 2,487 | `5df78e647ba99333432e856f0f14b8e10797b3e574b82b7607cbb3d53842f63c` |
| `sa_spa_mbs_pledge_held_20170621` | SPA `16de759e7b` | 2,356 | `f59c465ae43af836a1b9fbb4eb5770e99f0cf0e11ff0af42dc0c7f2f6b6e52d2` |
| `sa_spa_mbs_cabinet_20260901` | SPA `N2666035` | 12,812 | `069441cd3fe0f4f2ffb804c492f5e1f832fddd1b9d800a55768a66b51ba5ffc8` |
| `sa_spa_king_order_judges_20260904` | SPA `N2669204` | 3,328 | `2a7fa39ac3bbb2c69f7f55692f226680701b3608b48898bcaae7c87e6dc26862` |

## Response identities and stability checks

SPA items are recorded as the portal's news-detail API JSON (`https://portalapi.spa.gov.sa/api/v1/news/<id>`), the response
the public page loads; it carries no view counter, session token or request time, unlike the HTML page. The Royal Embassy
pages are raw Internet Archive captures (`id_`) made in 1999 and 2000, served without Content-Encoding. All fetches used
curl with its default User-Agent, no cookies and no Accept-Encoding header. Every response was downloaded at
2026-10-01T02:27:53Z-02:29:13Z and again at least 30 minutes later (times in each extract's `stability_check`); all 18 were
byte-identical. Exploratory reads earlier in the session gave the same byte counts and SHA-256 prefixes. `packet_check.py`
downloads each a third time (see [Checks](#checks)).

## Leads not imported

- SPA's re-send of A/136 with published_at 6 August 2007 under the 2006 date line (`7a34c6edb4`), the re-sent pledge of
  22 June 2017 (`d25612a743`), the 21 June 2017 oath item styling the new Crown Prince (`8b6ca51e47`) and the 'منطقة قصر
  الحكم' feature of 3 August 2005 (`051683c0fa`): duplicates or background.
- The Commission's regulations, fourth addition (A/164, released 8 October 2007, `7f343c89aa` and `5b0cd66f79`): Articles
  11-16 on the Secretary General's duties and ministerial rank. Procedure, and Article 24 of the law suffices.
- Royal Orders on the same man's other offices: A/291 of 6/9/1426 AH (Chief of the Royal Court), A/126 of 5/9/1430 AH
  (extension), A/124 of 24/7/1432 AH (Royal Court merger), A/56 and A/57 of 3/4/1436 AH (relief from the Royal Court and
  Royal Guard): `b2c8842bec`, `09a36df687`, `7a4e708b9b`, `106b9a7f0a`/`4793dc2fe0`, `724439bffa`. Other offices; never
  bound to the Commission role.
- Further deputations: A/174 of 7/11/1429 AH (Sultan, `359e4f88c1`), 3 December 2016 (`f62f808111`) and 27 March 2017
  (`e15afcc823`) (Mohammed bin Nayef); A/124 of 2011 names the Crown Prince without a name.
- The Royal Embassy's 1996 pages' other items (budget session chaired by the Crown Prince on 1 January 1996; cabinet
  sessions of 5 February 1996; King Fahd's thanks of 17 February 1996), and its 1997-2000 releases (`97_spa`-`00_spa`),
  not read.
- SPA portal items stamped 12 October 2002 that carry 2007 date lines (for example `c4c161443f`): publication-stamp
  artefacts, not 2002 records.
- Foreign records and encyclopaedias on Fahd's 1982 accession, the 1995 stroke and the 1996 delegation: leads only, not
  read for claims.

## Sources attempted

- SPA portal search API (`portalapi.spa.gov.sa/api/v1/news/search`): used only to find items, never recorded. It returns
  nothing unless the request carries `Accept-Language` matching the `l` parameter; with it, Arabic and English
  date-restricted searches worked. Requests were spaced; no Cloudflare challenge was met.
- SPA portal coverage: dense from 2004; scattered 2002-2003 items; nothing for 1990-2001. The 1990s therefore rest on the
  Royal Embassy's releases, whose captures begin with the 1996 pages (no 1990-1995 releases are archived).
- Internet Archive: the CDX API answered 503 ("Temporarily Offline") once on 1 October 2026 UTC and worked on a later
  try. On the second download the January 1996 capture answered HTTP 429 three times; the script backed off 10, 30 and 60
  seconds and the fourth request succeeded. Nothing was done to work around the limit.
- Umm al-Qura (`uqn.gov.sa`): its site search works but reaches only items from about 2021 (searches for 'وليا للعهد', 'محمد
  بن نايف' and 'هيئة البيعة' returned 2021-2024 items only), so the gazette texts of the 2005-2017 orders were not found.
  Its listings were not recorded as sources.

## Suggested next work orders

These are proposals for the integrator. They are not created in `work-orders.json`.

- `C01-SaudiArabia-CROWN-001`: Arabic texts of the 1995-1996 decrees (A-112, A-199) and of Fahd's 1982 accession and
  Abdullah's 1982 selection, if an official archive is reachable.
- `C01-SaudiArabia-CROWN-002`: Umm al-Qura gazette issues for A/193, A/136, A/175, A/145, A/267, A/128 and the CLAUDE-C01-06
  orders, once the gazette archive before 2021 is searchable.
- `C01-SaudiArabia-CROWN-003`: the Allegiance Commission's later Secretary General, if any, and the deputy secretary
  general of Article 24.

## Integration notes (outside this packet's file boundary)

- The user chose to start this batch (C01-42 to C01-46) before Codex's roadmap line asking to continue existing claims
  first; this packet is part of that batch.
- Claim commit `a529ab8d` on base `509bd289`; integration merged at `a342c6f6` (`a81d2486`) and `7761abe4` (`d0c6676b`); the
  packet commit and a separate commit regenerating `research-index.json` follow on `claude/c01-sa-45`. Other packets running in parallel (C01-38 to 41 in fixes, C01-42 to 46 in research) edit
  other country files; `research-index.json` is the only shared file, so whichever merges second must regenerate it.
- `research-index.json` new totals: 1,987 sources and 4,904 claims (previously 1,969 and 4,874 at `d0c6676b`). Saudi
  Arabia now has 188 claims (previously 158); its 6 organization and 6 institution observations, 10 role observations, 12
  mapping-pending entries and two open batches are unchanged.
- Pinned tests updated by exact re-expression, none loosened:
  - `test_saudi_executive_c01_06.py`: the totals (103, 158) become (121, 188) in two places; the holder-order assertions on
    `sa_king` and `sa_crown_prince` append the exact lists `C01_45_KING_OBSERVATIONS` and
    `C01_45_CROWN_PRINCE_OBSERVATIONS`; one comment is updated.
  - `test_saudi_shura_allegiance_c01_25.py`: the same totals in two places; its source slice becomes
    `[BASE_SOURCES:-C01_45_SOURCES]` with an exact total; `C01_06_HOLDERS` gains the same appended observations;
    `sa_succession_secretary` leaves `UNTOUCHED` for an exact pin of its new sources, claims and holder
    (`C01_45_SECRETARY`); the coverage check pins the one CLAUDE-C01-45 item after the CLAUDE-C01-25 items
    (`LATER_UNRESOLVED`).
- Known failures outside the suite: `test_certified_gap_ledger.py` reports no pinned attribution for a new packet until
  Codex classifies its commit; `test_certified_boundary_matrix.py` (S23) needs `spheres-web/src`, which the sparse
  checkout lacks. Neither is fixed here, and `docs/campaign-certification/C01/gap-ledger/` is untouched.
- `campaign_census.py --check` passes (exit 0, `"check": true`) both at the merged base and with this packet, so there
  is no census failure to disclose; this packet does not touch `spheres-sim/src/government.rs` or `census.json`.

## Checks

Run from `C:/Users/ridge/Spheres-c01-sa45` with `PYTHONDONTWRITEBYTECODE=1`, after merging `d0c6676b`. The sparse checkout
includes `spheres-sim/src`, `spheres-sim/data` and `spheres-web/data`, so the census runs instead of being skipped. After
the second merge, `workboard.py --check` reported the S26 handoff `docs/campaign-certification/S26/preparation/RECRUITMENT.md`
missing only because the sparse checkout lacked it; `docs/campaign-certification/S26` was added to the sparse checkout
(no file changed) and the check passed.

```text
python -X utf8 tools/avatars/campaign_research.py            # regenerate: 1,987 sources, 4,904 claims, 93 batches
python -X utf8 tools/avatars/campaign_research.py --check    # pass
python -X utf8 tools/avatars/campaign_census.py --check      # exit 0, "check": true
python -X utf8 -m unittest discover -s tools/avatars -p "test_saudi*.py"         # 24 pass (7 new, 9 C01-25, 8 C01-06)
python -X utf8 -m unittest discover -s tools/avatars -p "test_*research*.py"     # 79 pass
python -X utf8 -m unittest discover -s tools/avatars -p "test_campaign*.py"      # 16 pass, census included
node --test tools/ui/check_leadership_research_review.cjs    # 11 pass
python tools/planning/workboard.py --check                   # pass, 44 markers, 49 bounded tasks
git diff --check                                             # clean
```

Outside the suite, as expected: `test_certified_gap_ledger.py` errors with "Source sa_embassy_fahd_delegates_19960101
(SaudiArabia) has no pinned attribution" until Codex classifies this packet's commit, and
`test_certified_boundary_matrix.py` (S23) stops at the missing `spheres-web/src/person_avatar_assets.rs` of the sparse
checkout. Neither is fixed here.

The new test's ten mutations (a successor-derived end, a ceremony as an observation, a deputation on a role, a from from
an order day, a re-dated existing holder, an observation on the wrong tenure, drifting claim text, a misresolved
'yesterday', a changed response identity and an observation on `sa_pm`) each fail.
