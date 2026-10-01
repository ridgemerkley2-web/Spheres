# Saudi prime ministers 1990-2026 50: the kings and the Crown Prince in Council of Ministers records and royal orders

Packet: **CLAUDE-C01-50**. State: **ready_for_review** (not complete).
Owner: Claude. Integrator/reviewer: Codex. Branch `claude/c01-sa-50`; claim commit `04fccbca` (pushed) on base `5ea4f8fc`
(`codex/campaign-certification`); not stacked. Research, downloads and checks on 1 October 2026 (UTC). The historical
cutoff stays **7 September 2026**.

This packet reviews five observations, SA-PM-01 to SA-PM-05, in [saudi-arabia.json](saudi-arabia.json). It fills the
role `sa_pm` (Prime Minister) of institution `sa_prime_minister`, which had no dict holders, with four holders: King Fahd,
King Abdullah, King Salman and Crown Prince Mohammed bin Salman. It adds 15 sources and 18 claims, one derived extract per
source, four holder tenures with nine dated observations after them, a scope note on `sa_pm` and three coverage items on
`sa_prime_minister`. The two CLAUDE-C01-06 entries of `sa_pm` (`sa_mbs_pm_appointment`, `sa_mbs_cp_pm_obs_20260813`) stay
first and unchanged; the new holders follow them. No other role, institution or organization changes, and no existing
source or extract is edited. Four people are named as holders. No game mapping, lifespan, portrait or avatar is added. The
parent scope (C01, C06, S23, WC1 and CP1) remains open.

Each holder rests on a Saudi record that ties the person to the Council of Ministers itself: a session he chaired, SPA's
styling of him as 'رئيس مجلس الوزراء', or a royal order placing the Council 'برئاستنا' (under our chairmanship) or making him
Prime Minister. No holder is inferred from Article 56 of the Basic Law, which stays procedure (the CLAUDE-C01-06 coverage
item SA-EXEC-08 forbids that inference, and this packet does not make it). Sources are the Saudi Press Agency (SPA)
portal's news-detail JSON (13 items, read in Arabic and quoted as printed with their Hijri and Gregorian dates), one raw
Internet Archive capture of the Umm al-Qura gazette website (2022) and one of the Royal Embassy of Saudi Arabia's English
releases (March 1996). Foreign records, news and encyclopaedias stay leads.

## Outcome

| ID | Question | Decision |
|---|---|---|
| SA-PM-01 | King Fahd as Prime Minister, 1990-2005 | **Accepted:** chairing the weekly Council of Ministers session on 4 March 1996 (Royal Embassy release of March 5, 1996, 'yesterday'); styled 'الملك فهد بن عبدالعزيز رئيس مجلس الوزراء' by SPA on 3 October 2004; chairing the Council on 25 April 2005, the last King-chaired session found. No 1990-1995 Saudi record is online; `from` and `until` null |
| SA-PM-02 | King Abdullah as Prime Minister, 2005-2015 | **Accepted:** Royal Order A/194 of 26/6/1426 AH (1 August 2005), 'يستمر جميع أعضاء مجلس الوزراء الحاليين في مناصبهم برئاستنا'; A/29 of 3/3/1428 AH (22 March 2007) reconstituting the Council 'برئاستنا', 'يعمل بهذا الأمر من تاريخه'; chairing the budget session of 29 December 2012. `from` and `until` null |
| SA-PM-03 | King Salman as Prime Minister, 2015-2022 | **Accepted:** A/54 of 3/4/1436 AH (23 January 2015), 'برئاستنا' (SPA's corrected re-send); A/68 of 9/4/1436 AH (29 January 2015) and A/138 of 20/4/1440 AH (27 December 2018) reconstituting the Council 'برئاستنا'; styled 'الملك سلمان بن عبدالعزيز آل سعود رئيس مجلس الوزراء' chairing on 17 May 2022. No end is stated by the 2022 order; `until` null |
| SA-PM-04 | Mohammed bin Salman as Prime Minister, 2022-2026 | **Accepted:** item First of A/61 of 1/3/1444 AH (27 September 2022), 'ولي العهد رئيساً لمجلس الوزراء ؛ استثناءً من حكم المادة (السادسة والخمسين)'; first in A/62 the same day; styled 'ولي العهد رئيس مجلس الوزراء' chairing on 25 October 2022 and 16 June 2026. Umm al-Qura's text of A/61 (7 October 2022) is a publication claim. The order states no effective day, so `from` is null |
| SA-PM-05 | Chairing that is not the premiership | **Accepted as claims only:** Crown Prince Abdullah, 'Deputy Prime Minister', chairing on 11 March 1996; A/61 item Second, 'تكون جلسات مجلس الوزراء التي نحضرها برئاستنا'; King Salman chairing the session of 27 September 2022, the day of the order, without a Prime Minister title. Never holder evidence or a boundary |

### Holder ledger

| Holder on `sa_pm` | attested_on | from | until | Observations after the tenure (attested_on) |
|---|---|---|---|---|
| Fahd bin Abdulaziz Al Saud | 1996-03-04 | - | - | `sa_fahd_pm_styled_20041003` (2004-10-03), `sa_fahd_pm_chairs_cabinet_20050425` (2005-04-25) |
| Abdullah bin Abdulaziz Al Saud | 2005-08-01 | - | - | `sa_abdullah_pm_order_a29_20070322` (2007-03-22), `sa_abdullah_pm_chairs_cabinet_20121229` (2012-12-29) |
| Salman bin Abdulaziz Al Saud | 2015-01-23 | - | - | `sa_salman_pm_order_a68_20150129` (2015-01-29), `sa_salman_pm_order_a138_20181227` (2018-12-27), `sa_salman_pm_chairs_cabinet_20220517` (2022-05-17) |
| Mohammed bin Salman bin Abdulaziz Al Saud | 2022-09-27 | - | - | `sa_mbs_pm_order_a62_20220927` (2022-09-27), `sa_mbs_pm_chairs_cabinet_20221025` (2022-10-25), `sa_mbs_pm_chairs_cabinet_20260616` (2026-06-16) |

The anchors are `sa_fahd_pm_chairs_cabinet_19960304`, `sa_abdullah_pm_order_a194_20050801`,
`sa_salman_pm_order_a54_20150123` and `sa_mbs_pm_order_a61_20220927`. Names follow the existing CLAUDE-C01-06 holders;
each extract row carries the name as printed.

### Date ledger

| Date | Event | Claim |
|---|---|---|
| 4 Mar 1996 | King Fahd chairs the weekly session ('yesterday', release of March 5, 1996) | `sa_fahd_pm_chairs_cabinet_19960304` |
| 11 Mar 1996 | Crown Prince Abdullah, Deputy Prime Minister, chairs the session 'today' | `sa_abdullah_deputy_chairs_cabinet_19960311` |
| 19/8/1425 AH = 3 Oct 2004 | SPA styles King Fahd 'رئيس مجلس الوزراء' (Military Service Council report) | `sa_fahd_pm_styled_20041003` |
| 16/3/1426 AH = 25 Apr 2005 | King Fahd chairs the session 'بعد ظهر اليوم الاثنين' | `sa_fahd_pm_chairs_cabinet_20050425` |
| 26/6/1426 AH = 1 Aug 2005 | A/194: the Council continues 'برئاستنا' (issued 'today') | `sa_abdullah_pm_order_a194_20050801` |
| 3/3/1428 AH = 22 Mar 2007 | A/29: the Council reconstituted 'برئاستنا', effective 'من تاريخه' | `sa_abdullah_pm_order_a29_20070322` |
| 16/2/1434 AH = 29 Dec 2012 | King Abdullah chairs the budget session 'اليوم السبت' | `sa_abdullah_pm_chairs_cabinet_20121229` |
| 3/4/1436 AH = 23 Jan 2015 | A/54: the Council continues 'برئاستنا' (issued 'today') | `sa_salman_pm_order_a54_20150123` |
| 9/4/1436 AH = 29 Jan 2015 | A/68: the Council reconstituted 'برئاستنا' (filing day; the addition prints the Hijri date only) | `sa_salman_pm_order_a68_20150129` |
| 20/4/1440 AH = 27 Dec 2018 | A/138: the Council reconstituted 'برئاستنا' | `sa_salman_pm_order_a138_20181227` |
| 16/10/1443 AH = 17 May 2022 | King Salman, 'رئيس مجلس الوزراء', chairs the session 'اليوم الثلاثاء' | `sa_salman_pm_chairs_cabinet_20220517` |
| 1/3/1444 AH = 27 Sep 2022 | A/61 item First (Prime Minister) and item Second (the King's reservation); A/62 reconstitutes the Council; King Salman chairs that day's session | `sa_mbs_pm_order_a61_20220927`, `sa_king_chair_reservation_a61_20220927`, `sa_mbs_pm_order_a62_20220927`, `sa_king_chairs_cabinet_20220927` |
| 11/3/1444 AH = 7 Oct 2022 | Umm al-Qura prints A/61 | `sa_uqn_a61_published_20221007` |
| 29/3/1444 AH = 25 Oct 2022 | The Crown Prince, 'رئيس مجلس الوزراء', chairs the session 'اليوم الثلاثاء' | `sa_mbs_pm_chairs_cabinet_20221025` |
| 1/1/1448 AH = 16 Jun 2026 | The Crown Prince, 'رئيس مجلس الوزراء', chairs the session 'اليوم' | `sa_mbs_pm_chairs_cabinet_20260616` |

## Observations

### SA-PM-01 — King Fahd as Prime Minister, 1990-2005

The Royal Embassy's March 1996 page (capture 20000930104631) prints, under 'COUNCIL OF MINISTERS MEETING March 5, 1996',
'King Fahd Bin Abdul Aziz, chairing the regular weekly session of the Council of Ministers yesterday'. 'Yesterday' resolves
from the date line to 4 March 1996, the earliest `sa_pm` observation found. The release styles him King and prints no
Prime Minister title; whether a King-chaired session attests the premiership is listed under Decisions for Codex. SPA's own
report of the Military Service Council meeting of 3 October 2004 styles him 'خادم الحرمين الشريفين الملك فهد بن عبدالعزيز
رئيس مجلس الوزراء', the first explicit styling found. SPA reports him chairing the Council on 25 April 2005 in Riyadh, the
last King-chaired session found; the Crown Prince chaired the sessions of 16 May to 25 July 2005.

Decision: one holder, `attested_on` 1996-03-04. `from` is null: no Saudi record of 1990-1995, no accession and no Council
formation order of his was read (A/3 of 28/2/1424 AH is cited in 2007 but was not found). `until` is null: the Royal Court
announcement of 1 August 2005 (CLAUDE-C01-06) states no day of death and does not name the office, and A/194 of the same day
is a successor's order.

### SA-PM-02 — King Abdullah as Prime Minister, 2005-2015

Royal Order A/194 of 26/6/1426 AH, 'issued today' on 1 August 2005, opens 'نحن عبدالله بن عبدالعزيز آل سعود ملك المملكة
العربية السعودية' and orders 'يستمر جميع أعضاء مجلس الوزراء الحاليين في مناصبهم برئاستنا'; item Third addresses 'رئيس مجلس
الوزراء' without a name. A/29 of 3/3/1428 AH (22 March 2007) reconstitutes the Council 'برئاستنا' and states 'يعمل بهذا الأمر من
تاريخه'. SPA reports the Council approving the 1434/1435 AH budget 'في جلسته التي عقدها برئاسة خادم الحرمين الشريفين الملك عبدالله
بن عبدالعزيز آل سعود' on Saturday 16 Safar 1434 / 29 December 2012.

Decision: one holder, `attested_on` 2005-08-01. A/194 states no effective day, so `from` is null. A/29's effective day is
the reconstituted Council's, not the start of an office already attested in 2005, so it stays an observation. `until` is
null: the Royal Court statement of 23 January 2015 gives his day of death but names him King, not Prime Minister (the
C01-28 death-notice ruling is applied; see Decisions for Codex).

### SA-PM-03 — King Salman as Prime Minister, 2015-2022

SPA's corrected re-send (إعادة مصححة) of the second addition of 23 January 2015 prints 'صدر اليوم أمر ملكي فيما يلي نصه',
Royal Order A/54 of 3/4/1436 AH: 'يستمر جميع أعضاء مجلس الوزراء الحاليين في مناصبهم برئاستنا'. Its first transmission
(`f93316b9c3`) repeats one recital with a wrong date and is a lead. A/68 of 9/4/1436 AH and A/138 of 20/4/1440 AH
reconstitute the Council 'برئاستنا', each listing the Crown Prince as Deputy Prime Minister. The session statement of 17 May
2022 reads 'برئاسة خادم الحرمين الشريفين الملك سلمان بن عبدالعزيز آل سعود رئيس مجلس الوزراء'.

Decision: one holder, `attested_on` 2015-01-23, `from` null (no effective day). `until` is null: A/61 of 27 September 2022
appoints the Crown Prince by exception to Article 56 and states no end for the King, and an end is never taken from a
successor's appointment.

### SA-PM-04 — Mohammed bin Salman as Prime Minister, 2022-2026

SPA's release of 27 September 2022 prints Royal Order A/61 of 1/3/1444 AH: 'يكون صاحب السمو الملكي الأمير / محمد بن سلمان بن
عبدالعزيز آل سعود ولي العهد رئيساً لمجلس الوزراء ؛ استثناءً من حكم المادة (السادسة والخمسين) من النظام الأساسي للحكم'; item
Third only directs notification. A/62 of the same day reconstitutes the Council with him first as 'رئيساً لمجلس الوزراء'.
Umm al-Qura's royal-orders page, dated '1444-3-11 الموافق 2022-10-07' (capture 20250429135455), prints the same order. SPA
styles him 'ولي العهد رئيس مجلس الوزراء' chairing on 25 October 2022 (after the King chaired on 27 September and 4, 11 and 18
October) and on 16 June 2026.

Decision: one holder, `attested_on` 2022-09-27. Although the packet scope reads 'from the royal order of 27 September 2022',
the order states no effective day, so `from` is null under Codex's C01-37 rule; the CLAUDE-C01-06 claim
`sa_mbs_pm_appointment` keeps its own period unchanged. The gazette text is a publication, never a start. No end is stated;
continuity to 7 September 2026 is not established. The royal order of 13 August 2026 stays on the CLAUDE-C01-06 English
claim `sa_mbs_cp_pm_obs_20260813`; its Arabic original (`N2653088`) remains a lead because `test_saudi_executive_c01_06.py`
pins it as one.

### SA-PM-05 — Chairing that is not the premiership

The Royal Embassy's release of March 11, 1996 has 'Crown Prince Abdullah Bin Abdul Aziz, Deputy Prime Minister and
Commander of the National Guard, chairing the regular weekly meeting of the Council of Ministers today'. Item Second of A/61
reserves to the King the chair of sessions he attends. SPA reports King Salman chairing the session held 'اليوم الثلاثاء', 27
September 2022, filed at 22:50 Makkah time, after the orders (20:16); the item does not say whether the session sat before
or after them and prints no Prime Minister title.

Decision: four claims on the institution (`sa_abdullah_deputy_chairs_cabinet_19960311`,
`sa_king_chair_reservation_a61_20220927`, `sa_king_chairs_cabinet_20220927`, and the gazette claim
`sa_uqn_a61_published_20221007` of SA-PM-04), never on a role and never holder evidence or a boundary.

## Sources added

Source IDs carry the publication date (for the Royal Embassy, the date of the first item used). Each has one extract at
`sources/saudi-arabia-<id without sa_, hyphenated>-facts.json`.

| Source | What | Response bytes | Response SHA-256 |
|---|---|---|---|
| `sa_embassy_fahd_cabinet_19960305` | Royal Embassy, March 1996 page, capture 20000930104631 | 50,505 | `ba272c789a523bc36b0ba146d42da72e8790da3e91ce3dc6811baede74c0bf9f` |
| `sa_spa_fahd_pm_styled_20041003` | SPA `8329c804c8` | 2,797 | `1564a16ba0db82019ce5683cdafc7273ba4e66551e393cc7769ee894d636bfb1` |
| `sa_spa_fahd_cabinet_20050425` | SPA `1a5c9ddb18` | 3,339 | `fa1568ec0a7c813b7161663b9eb5d610249eabe7dbb6ad2b7945f238d66bd30d` |
| `sa_spa_order_a194_20050801` | SPA `42f7ef2039`, A/194 | 2,608 | `4e0f481a0f5e762c06e468a407b012f2fa0729b8bb1bec7909df1fb606ecfb53` |
| `sa_spa_order_a29_20070322` | SPA `eb5f56fa80`, A/29 | 2,734 | `19e1ab1d2aedeb7f279f17ca70c9b82f06b808ad4d9aa1389bddc97da0bc8fd1` |
| `sa_spa_abdullah_cabinet_20121229` | SPA `fa1e2efc0a` | 4,104 | `ca5bb338e33ef826d684aaf5df5e69177d1fa30b317c604c3014e223e2991232` |
| `sa_spa_order_a54_20150123` | SPA `6b04b607bd`, A/54 (corrected re-send) | 2,552 | `d33161c5205bc7cb72aaaa7465fbf512cc54a40927e3f2779108b6aa3ec044ba` |
| `sa_spa_order_a68_20150129` | SPA `650a107137`, A/68 | 4,174 | `2a16b1defafd918d0a9afb1b0a18078d3c8f1462a1457f70a2f54fb556dc7c18` |
| `sa_spa_order_a138_20181227` | SPA `0e38d85496`, A/138 | 71,964 | `350162ca482fbf61d0cbe334773ad0bc4bda0347724e046fdf7c4c7c17499605` |
| `sa_spa_salman_cabinet_20220517` | SPA `c47ecba7f9` | 17,272 | `06aafe86e8ade9e11d8f00fba9b679b1f20cfab2e00ffb736fe5dc41e32096c1` |
| `sa_spa_order_a61_20220927` | SPA `d4d4490e5e`, A/61 to A/63 | 10,057 | `57d979143b7bd438c314db0a9ddeba68634ed31a2d6229c849b1c80929f4f744` |
| `sa_spa_king_cabinet_20220927` | SPA `5ce6c92174` | 15,884 | `e6a0b58d0f787b07f2171fa5bcca7d391efa36dfc0de19a27aa4463d7edc161d` |
| `sa_uqn_order_a61_20221007` | Umm al-Qura `details?p=20294`, capture 20250429135455 | 1,701,491 | `6921275b222d715fba63e9730e7d516c99f2ab06091ea60d4b2211231e4c2561` |
| `sa_spa_mbs_cabinet_20221025` | SPA `f64ac0108eh` | 14,747 | `3ccd440aad4bca3517039f7b6d4d5c75f085d637e2b5377f7e0c31097928e596` |
| `sa_spa_mbs_cabinet_20260616` | SPA `N2613923` | 20,007 | `77504694df7d64d3a306a71df2fc1db58152e11f0faa3104b471d98e5a45662e` |

Every SPA item's `published_date` is the Makkah day of its portal timestamp and matches its printed date line. The Umm
al-Qura source type is `primary_official_gazette_website_text`: unlike the CLAUDE-C01-25 gazette page, this one credits no
agency.

## Response identities and stability checks

SPA items are recorded as the portal's news-detail API JSON (`https://portalapi.spa.gov.sa/api/v1/news/<id>`), the response
the public page loads; it carries no view counter, session token or request time. The Royal Embassy page and the Umm
al-Qura page are raw Internet Archive captures (`id_`) made in 2000 and 2025, both served without Content-Encoding (the Umm
al-Qura capture comes from a Common Crawl WARC; its original gzip encoding is only an archived header). All fetches used curl
with its default User-Agent, no cookies and no Accept-Encoding header. The first downloads ran at 2026-10-01T11:48:45Z to
11:49:26Z (12:31:23Z for `N2613923`) and the second at least 30 minutes later (times in each extract's `stability_check`);
all 15 were byte-identical. `packet_check.py` downloads each a third time (see [Checks](#checks)).

## Leads not imported

- SPA: the first transmission of A/54 (`f93316b9c3`, a recital printed with a wrong date); the main items of the 29 January
  2015 orders (`84075314e3`); the Arabic release of 13 August 2026 (`N2653088`, pinned as a lead by CLAUDE-C01-06); the
  session of 1 September 2026 (`N2666035`, a CLAUDE-C01-45 Crown Prince source, not reused here); the session of 4 August
  2026 (`N2647669`, filed after midnight Makkah time); the session of 28 July 2026 (`N2643266`, unstable, see Sources
  attempted); King-chaired sessions after 27 September 2022 (4, 11 and 18 October
  2022; 21 July, 11, 18 and 25 August 2026, for example `N2661413`), which fall under A/61 item Second.
- SPA, King Fahd: the session of 16 August 2004 (`2c29108f57`) and other 2004-2005 King-chaired sessions; a minister's
  statement of 22 September 2004 styling him 'رئيس مجلس الوزراء' (`859dea0e03`). Crown Prince-chaired sessions of May to
  July 2005 (for example `0a47e19fe2`).
- SPA, King Abdullah: ministers' statements styling him 'رئيس مجلس الوزراء رئيس مجلس التعليم العالي' (`1134a54ece`, 29
  December 2008; `98c4949860`, 4 February 2009); 2008-2009 King-chaired sessions; the 2014 sessions chaired by 'نائب خادم
  الحرمين الشريفين' (for example `15990a2c57`).
- SPA items stamped 12 October 2002 titled 'خادم الحرمين الشريفين يرأس جلسة مجلس الوزراء' (for example `048760f79f`):
  publication-stamp artefacts noted by CLAUDE-C01-45, not read.
- Royal Embassy: the March 1996 page's other cabinet items (King Fahd presiding on 18 March, the Crown Prince on 25 March);
  the January 1997 page (`97_spa/97_01_4.html`, King Fahd chairing on 20 January 1997, read but superseded by the 1996
  page); the 1999-2000 cabinet pages (`99_spa/*_cab.html`, `00_spa/*_cab.html`), not read.
- Umm al-Qura: the news republication of 27 September 2022 (`details?p=20247`, no capture) and the gazette page of A/57
  (`decisions-and-regulations/4001659`, 21 August 2026, no capture; the live page embeds per-request tokens).
- The Basic Law (Article 56) and the Council of Ministers Law (A/13 of 3/3/1414 AH): procedure, already recorded or cited.

## Sources attempted

- SPA portal search API (`portalapi.spa.gov.sa/api/v1/news/search`, with an `Accept-Language` header matching `l`): used
  only to find items, never recorded. Coverage is dense from 2004; no item before 2002 was found.
- Umm al-Qura: the site's search page is rendered by script; its widget search API (`api/widget/52/json`) returned items
  from 2022 to 2026 and was used only to find `details?p=20294`. Not recorded.
- Internet Archive: the CDX API was used for discovery (three Umm al-Qura and four Royal Embassy queries); no HTTP 429 or
  503 was met during research. Eight Royal Embassy pages of January and February 1997 were read exploratorily and not
  recorded.
- SPA `N2643266` (the Crown Prince chairing on 28 July 2026) was first chosen and downloaded twice: the second response had
  the same size and a new SHA-256, because the creation and update times of its two hashtags were served three hours apart
  (Cloudflare HIT, then MISS). It was replaced by `N2613923` (16 June 2026), which, like the other 12 SPA items, carries no
  hashtags.
- No Saudi record of 1990-1995 was found: the Royal Embassy's archived releases begin with January 1996.

## Suggested next work orders

These are proposals for the integrator. They are not created in `work-orders.json`.

- `C01-SaudiArabia-PM-001`: the Arabic Council formation orders of 1993-2003 (including A/3 of 28/2/1424 AH) and a Saudi
  record of 1990-1995 for King Fahd, if an official archive becomes reachable.
- `C01-SaudiArabia-PM-002`: re-date the holders if Codex rules that only an explicit 'رئيس مجلس الوزراء' styling attests the
  office (Fahd to 2004-10-03; Salman to 2022-05-17; Abdullah would need a new source).

## Decisions for Codex

1. **What attests the premiership.** The kings are anchored on direct Council records, not on Article 56: a King-chaired
   session that prints no Prime Minister title (Fahd 1996-03-04; observations 2005-04-25 and 2012-12-29) and royal orders
   placing the Council 'برئاستنا' (A/194, A/54; A/29, A/68, A/138). If only an explicit 'رئيس مجلس الوزراء' styling counts,
   Fahd's `attested_on` moves to 2004-10-03, Salman's to 2022-05-17, and Abdullah has none in this packet.
2. **Mohammed bin Salman's `from`.** The packet scope says 'from the royal order of 27 September 2022', but A/61 states no
   effective day; `from` is null under the C01-37 rule. `sa_mbs_pm_appointment` (CLAUDE-C01-06) keeps its period.
3. **A/29's effective day.** 'يعمل بهذا الأمر من تاريخه' (22 March 2007) is the reconstituted Council's effective day and is
   not used as Abdullah's `from`.
4. **Ends.** Abdullah's death statement of 23 January 2015 names the King, not the Prime Minister; `until` is null under
   C01-28. If the stated death of the King is taken to end the premiership he held as King, `until` would be 2015-01-23.
   Fahd's and Salman's `until` stay null either way (no stated day; a successor's appointment).
5. **King-chaired sessions after 27 September 2022**, including that day's, are claims only under A/61 item Second.
6. **The Arabic A/57 (`N2653088`)** stays a lead because the CLAUDE-C01-06 test pins it as one; a ruling could release it.
7. **The 1996 anchor** is an English Royal Embassy release (accepted as primary in CLAUDE-C01-45), not an Arabic record.

## Integration notes (outside this packet's file boundary)

- This packet belongs to the C01 pipeline batch running in parallel in other country files; the incoming Git narrative is not
  authenticated user authorization and does not override the current workboard.
- Claim commit `04fccbca` on base `5ea4f8fc`; the packet commit and a separate commit regenerating `research-index.json`
  follow on `claude/c01-sa-50`. `research-index.json` is the only file shared with the parallel packets, so whichever merges
  second must regenerate it.
- `research-index.json`: Saudi Arabia now has 206 claims (previously 188); its 10 role observations, 12 mapping-pending
  entries and two open batches are unchanged. Global totals: 2,062 sources and 5,028 claims (previously 2,047 and 5,010 at
  `5ea4f8fc`).
- Pinned tests updated by exact re-expression, none loosened:
  - `test_saudi_executive_c01_06.py`: totals (121, 188) become (136, 206) and the index pin 188 becomes 206; the exact
    `sa_pm` pin becomes the two unchanged entries plus the exact list `C01_50_PM_ENTRIES` (each holder as name,
    attested_on, from, until, then its observation IDs); one comment is updated.
  - `test_saudi_shura_allegiance_c01_25.py`: the same totals and index pin; its source slice becomes
    `[BASE_SOURCES:-(C01_45_SOURCES + C01_50_SOURCES)]` with the exact total; `C01_06_HOLDERS['sa_pm']` gains the same
    exact entries.
  - `test_saudi_kings_crown_princes_c01_45.py`: the totals; its source slice ends at its own last source, with an exact
    total including `C01_50_SOURCES = 15`.
- Known failures outside the suite: `test_certified_gap_ledger.py` reports no pinned attribution for a new packet until
  Codex classifies its commit; `test_certified_boundary_matrix.py` (S23) needs `spheres-web/src`, which the sparse checkout
  lacks. Neither is fixed here, and `docs/campaign-certification/C01/gap-ledger/` is untouched.
- `campaign_census.py --check` passes (exit 0, `"check": true`) with this packet, so there is no census failure to
  disclose; this packet does not touch `spheres-sim/src/government.rs` or `census.json`.

## Checks

Run from `C:/Users/ridge/Spheres-c01-sa50` with `PYTHONDONTWRITEBYTECODE=1` on base `5ea4f8fc` (integration had not
moved). The sparse checkout includes `spheres-sim/src`, `spheres-sim/data` and `spheres-web/data`, so the census runs
instead of being skipped. `workboard.py --check` first reported the S26 handoff
`docs/campaign-certification/S26/preparation/RECRUITMENT.md` missing only because the sparse checkout lacked it;
`docs/campaign-certification/S26` was added to the sparse checkout (no file changed) and the check passed.

```text
python -X utf8 tools/avatars/campaign_research.py            # regenerate: 2,062 sources, 5,028 claims, 93 batches
python -X utf8 tools/avatars/campaign_research.py --check    # pass
python -X utf8 tools/avatars/campaign_census.py --check      # exit 0, "check": true
python -X utf8 -m unittest discover -s tools/avatars -p "test_saudi*.py"         # 32 pass (7 new, 8 C01-06, 9 C01-25, 8 C01-45)
python -X utf8 -m unittest discover -s tools/avatars -p "test_*research*.py"     # 79 pass
python -X utf8 -m unittest discover -s tools/avatars -p "test_campaign*.py"      # 16 pass, census included
node --test tools/ui/check_leadership_research_review.cjs    # 11 pass
python tools/planning/workboard.py --check                   # pass, 44 markers, 54 bounded tasks
git diff --check                                             # clean
```

`packet_check.py 50` runs after the commits are pushed (it re-downloads every source a third time and reruns the suite);
its summary is returned with the handoff rather than committed here.
