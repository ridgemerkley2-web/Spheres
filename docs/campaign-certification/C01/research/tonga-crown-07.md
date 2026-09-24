# Tonga Crown 07: reign and regency chronology, 1990-2026

Packet: **CLAUDE-C01-07**. State: **ready_for_review** (not complete).
Owner: Claude. Integrator/reviewer: Codex. Branch `claude/c01-tonga-07`, stacked on
`claude/c01-tonga-04` at `62690e61` (CLAUDE-C01-04, ready for review; that branch has
already merged current integration `04bc99a6`). Research access: 21 September 2026
(local). The historical cutoff stays **7 September 2026**.

This packet reviews eight observations about the Crown of Tonga (`to_crown`, role `to_king`)
in [tonga.json](tonga.json): who held the Crown when the period opens, the regencies recorded
before Taufa'ahau Tupou IV's death, and the accession, proclamation, traditional installation,
coronation and death of each monarch through the cutoff. It adds 27 sources and 44 claims
(seven of them on the reused sources `to_constitution_2020`, `to_constitution_older`,
`to_ipu_2008` and `to_ipu_2010`), three holder observations on the existing `to_king` role, and
a scope note on that role. It adds no organization, institution, role, game mapping, lifespan,
portrait or avatar. The parent scope (C01, C06, S23, WC1 and CP1) remains open.

The research dossier and an independent check were prepared before this packet was written.
Every checker defect is applied below, the major one included (see [Checker defects](#checker-defects)).

## Outcome

| ID | Question | Decision |
|---|---|---|
| TO-CROWN-01 | Who held the Crown when the period opens? | **Accepted:** Taufa'ahau Tupou IV, by royal assents of 12 July and 8 August 1990; holder has no structured start |
| TO-CROWN-02 | Is a regency formally recorded before Tupou IV's death? | **Accepted as observations only:** Prince Regent Tupouto'a (13 Mar 2006) and Princess Regent Pilolevu (1 Jun 2006); no instrument, start or end; not reigns |
| TO-CROWN-03 | When did Tupou IV die, and is the funeral separate? | **Accepted:** 23:34 on 10 Sep 2006 New Zealand time = 00:34 on 11 Sep 2006 Tongan time (Gazette No. 20); funeral 19 Sep and mourning separate |
| TO-CROWN-04 | When did George Tupou V accede, and when was he proclaimed? | **Accepted:** accession by devolution on the death, 11 Sep 2006 (Tongan date); proclamation 11 Sep 2006 as a separate event |
| TO-CROWN-05 | When was George Tupou V crowned; is the traditional installation distinct? | **Accepted:** traditional installation (taumafa kava) 30 Jul 2008; coronation 1 Aug 2008; coronation oath undated |
| TO-CROWN-06 | When did George Tupou V die? | **Accepted:** 20:58 on 18 Mar 2012, Tonga time (15:58 Hong Kong time), Gazette Extraordinary No. 8 |
| TO-CROWN-07 | When did Tupou VI accede, and when was he proclaimed? | **Accepted:** accession by devolution 18 Mar 2012; proclamation 19 Mar 2012 as a separate event; no end |
| TO-CROWN-08 | When was Tupou VI crowned? | **Accepted:** 4 Jul 2015, on a non-ceremonial Tongan public body plus foreign corroboration; no ceremony record; oath undated |

The resulting `to_king` holders (after the existing `to_tupou_vi_2025` attestation, which stays
first and unchanged):

| Holder | `from` | `until` | Basis |
|---|---|---|---|
| Taufa'ahau Tupou IV | null | 2006-09-11 | death (Gazette No. 20); Crown devolves by the decease (Gazette No. 19) |
| George Tupou V | 2006-09-11 | 2012-03-18 | devolution on his predecessor's death; his own death (Gazette Ext. No. 8) and Gazette Ext. No. 9 |
| Tupou VI | 2012-03-18 | null | devolution on his predecessor's death (Gazette Ext. Nos. 8 and 9); no end recorded |

`attested_on` is null for all three because each carries a stated boundary instead. No holder
starts on a proclamation, a traditional installation, a coronation or an oath, and no regent is
a `to_king` holder.

### Date ledger

Each row is a separate dated fact with its own claim. Different dates for a death, a devolution,
a proclamation, an installation, a coronation, a funeral and a mourning period are not
inconsistencies, and none is merged into another, even where two fall on the same Tongan day.

| Date | Event | Claim or field |
|---|---|---|
| 12 Jul 1990 | Tupou IV assents to the Income Tax (Amendment) Act 1990 | `to_crown_assent_tupou_iv_19900712` |
| 8 Aug 1990 | Tupou IV assents to Act 12 of 1990 | `to_crown_assent_tupou_iv_19900808` |
| none (edition text) | The older revised constitution names Tupou IV in clause 31 | `to_crown_const1988_form_tupou_iv_c31` (no date) |
| unstated (on or after 30 Jul 2004) | Alleged participation of "the Regent at the time", Tupouto'a | `to_crown_regent_tupouto_a_2004_pleading` (no date) |
| 13 Mar 2006 | PMO styles Tupouto'a "The Prince Regent" | `to_crown_prince_regent_tupouto_a_20060313` |
| 1 Jun 2006 | Princess Regent Pilolevu opens Parliament | `to_crown_princess_regent_speech_20060601`, `to_crown_princess_regent_opening_20060601_court` |
| 30 Jun 2006 | Assent signed TUPOUTO'A (capacity unstated) | `to_crown_assent_tupouto_a_20060630` |
| 18 Aug 2006 | PMO: succession does not await the coronation oath | `to_crown_succession_not_oath_dependent_2006` |
| 10 Sep 2006, 23:34 NZ time | Tupou IV dies in Auckland (New Zealand calendar date) | `printed_times` of `to_crown_tupou_iv_death_20060911`; `to_crown_tupou_iv_death_20060910_un` |
| 11 Sep 2006, 00:34 Tongan time | The same death on the Tongan calendar; the Crown devolves on George Tupou V | `to_crown_tupou_iv_death_20060911`; Tupou IV `until`, George Tupou V `from` |
| 11 Sep 2006 | Gazette No. 19 proclamation (English and Tongan texts) | `to_crown_gtv_devolution_proclamation_20060911`, `..._to_20060911` |
| 11 Sep - 17 Oct 2006 | State mourning | `to_crown_tupou_iv_mourning_20060911` |
| 12 Sep 2006 | UN: George Tupou V "has succeeded his father" | `to_crown_gtv_king_20060912_un` |
| 13 Sep 2006 (notice), 14 Sep (PMO article) | 19 September appointed a holiday for the State Funeral | `document_date`, `published_date` of `to_pmo_20060914_funeral_holiday` |
| 19 Sep 2006 | Appointed and announced State Funeral day | `to_crown_tupou_iv_state_funeral_holiday_20060919`, `to_crown_tupou_iv_funeral_announced_20060911` |
| "yesterday" of 20 Sep 2006 | Burial at Mala'e Kula reported | `to_crown_tupou_iv_burial_reported_20060920` (no date) |
| "yesterday" of 28 Sep 2006 | Crown Prince title vested on 'Ulukalala Lavaka Ata | `to_crown_crown_prince_tupouto_a_lavaka_2006` (no date) |
| 3 Oct 2006 | Assent signed GEORGE TUPOU V | `to_crown_assent_gtv_20061003` |
| 2 May 2008 | IPU: George Tupou V appoints a Speaker | `to_ipu_2008_gtv_speaker_20080502` |
| 30 Jul 2008 | Traditional installation (taumafa kava): scheduled, held, customary declaration | `to_crown_gtv_taumafa_kava_scheduled_20080730`, `..._held_20080730`, `..._declaration_20080730` |
| 1 Aug 2008 | Coronation of George Tupou V | `to_crown_gtv_coronation_scheduled_20080801`, `..._day_address_20080801`, `..._guests_20080801`, `..._iha_20080801` |
| 18 Mar 2012, 20:58 Tonga time | George Tupou V dies in Hong Kong (15:58 local); the Crown devolves on Tupou VI | `to_crown_gtv_death_20120318`; George Tupou V `until`, Tupou VI `from` |
| 19 Mar 2012 | Gazette Extraordinary No. 9 proclamation | `to_crown_tupou_vi_devolution_proclamation_20120319` |
| 19 Mar - 19 Jun 2012 | State mourning as first declared | `to_crown_gtv_mourning_declared_20120319` |
| 30 Mar 2012 | Mourning revoked and reset to 19-31 March | `to_crown_gtv_mourning_revised_20120330` |
| 25 Jun - 7 Jul 2015 | Coronation import exemptions in force (made 23 June) | `to_crown_tupou_vi_coronation_exemptions_2015` |
| 4 Jul 2015 | Coronation of Tupou VI | `to_crown_tupou_vi_coronation_20150704_nrbt`, `to_crown_tupou_vi_coronation_iha_20150704` |
| 18 Dec 2025 | Latest attestation of Tupou VI (existing) | `to_tupou_vi_2025` |
| unknown | Regency instruments; both coronation oaths; end of Tupou VI's reign | TO-CROWN-02, 05, 08; Tupou VI `until` null |

Date conventions. `attested_on` is the date of the observed event or state as the source dates
it; a page's issue date is `published_date`, and an instrument's own date where the page has no
publication date (an assent, a meeting, a Cabinet understanding) is `document_date`. "Today" in a
dated document is read as that date. A date given only as "yesterday" or "Wednesday this week" is
not converted: those claims carry no structured date, following CLAUDE-C01-04's rule for "the
following day". Death dates are recorded on the Tongan calendar, as Gazette Extraordinary No. 8
records George Tupou V's; the printed local times are kept in `printed_times`.

## Observations

### TO-CROWN-01 — The Crown when the period opens

Evidence: the assent blocks of the Income Tax (Amendment) Act 1990 ("I assent, TAUFA'AHAU TUPOU
IV, 12th July 1990"; `to_crown_assent_tupou_iv_19900712`) and the Act of Constitution of Tonga
(Amendment) Act 1990, Act 12 of 1990 ("8th August, 1990"; `to_crown_assent_tupou_iv_19900808`).
The older revised constitution text names "His Majesty King Taufa'ahau Tupou IV his heirs and
successors" in clause 31 (`to_crown_const1988_form_tupou_iv_c31`). The UN record of
12 September 2006 has Tonga's representative say he became King in 1965 and reigned 41 years.

Decision: accepted. The Tupou IV holder observation has `from` null: his 1965 accession is
year-level context outside the period, and no 1990 date is a start. The two assents are dated
attestations and are cited by the holder.

Limits: the AGO files are re-typeset reproductions, not facsimiles of the gazetted Acts. The
older constitution text is labelled "1988 Revised Edition" but carries amendment notes up to
Act 12 of 1990 and none later, so it is a consolidation as it stood after August 1990, and its
claim has no date (defect D4).

### TO-CROWN-02 — Regencies before Tupou IV's death

Evidence:

- PMO report of 13 March 2006 (`to_crown_prince_regent_tupouto_a_20060313`): the first wreath at
  the Royal Tombs that day was laid by "The Prince Regent, HRH Crown Prince Tupouto'a".
- PMO release of 1 June 2006 (`to_crown_princess_regent_speech_20060601`): the Speech from the
  Throne by the Princess Regent, Princess Salote Mafile'o Pilolevu Tuita, at the Opening of
  Parliament that day.
- Supreme Court, Lasike v Noble Tu'iha'angana, decided 19 June 2006
  (`to_crown_princess_regent_opening_20060601_court`): the Chief Justice, an eyewitness, records
  the 1 June opening by the Princess Regent while the King was in Auckland, treats clause 43 as
  the provision under which she was appointed, construes its words, and holds the opening lawful.
  The same judgment remarks that the Regent has regularly assented to Acts in practice
  (`to_crown_regent_assent_practice_2006_court`).
- The Lord Chamberlain's 2008 profile (`to_crown_gtv_prince_regent_numerous_occasions`): as Crown
  Prince, George Tupou V acted as Prince Regent "on numerous occasions" (undated).
- Clauses 42-43 in the 2020 edition (`to_crown_const2020_prince_regent_c42_43`, now quoted) and
  the identical wording in the older text (`to_crown_const1988_prince_regent_c42_43`).

Decision: accepted as dated regency observations only. The regency claims sit on the `to_crown`
entry, not on the `to_king` role, and no regent is a holder. No regent role was created: the
packet reuses the existing roles, and with no instrument, start or end there is nothing to give a
role holder beyond these observations.

Limits: no Privy Council, Gazette or royal appointment instrument was found for either regency,
and no start or end. The court took the Princess Regent's appointment as given and did not
decide it. Two further items stay unconfirmed and are not regency observations: the Flyniu v Ata
pleading that "the Regent at the time, HRH Crown Prince Tupouto'a" took part in a Privy Council
approval (a party's allegation; the approval date is unstated, so it has no date, defect D6),
and the assent "TUPOUTO'A, 30th June, 2006" (`to_crown_assent_tupouto_a_20060630`), whose
capacity is not printed. Regencies during the reigns of George Tupou V and Tupou VI were not
researched.

### TO-CROWN-03 — Tupou IV's death, funeral and mourning

Evidence: Tonga Government Gazette No. 20 of Monday 11 September 2006, as reproduced on the PMO
website (`to_crown_tupou_iv_death_20060911`). Prime Minister Feleti Vaka'uta Sevele announces
that the death occurred in Auckland "at 11:34 pm on 10th September 2006 (New Zealand time),
12:34 am on 11th September 2006, (Tongan time)". The same notice announces the return to Tonga
on 13 September, lying in state and a State Funeral at Mala'e Kula on 19 September
(`to_crown_tupou_iv_funeral_announced_20060911`), and declares mourning until 17 October
(`to_crown_tupou_iv_mourning_20060911`). The funeral holiday was appointed for 19 September
(`to_crown_tupou_iv_state_funeral_holiday_20060919`), and a 20 September report describes the
burial "yesterday" (`to_crown_tupou_iv_burial_reported_20060920`).

Decision: accepted. The death is recorded with both printed times in `printed_times`, and
`attested_on` is the Tongan date, 2006-09-11, the same convention as for George Tupou V's death.
The UN record's "Sunday, 10 September 2006" (`to_crown_tupou_iv_death_20060910_un`) is kept as
corroboration of the New Zealand calendar date. The funeral, burial and mourning are separate
claims, all at entry level.

Limits: the gazette was read as a PMO web reproduction in an archived capture, not a facsimile.
The two printed times are one instant, consistent with New Zealand standard time (UTC+12) and
Tongan time (UTC+13) in September 2006. The burial report is dated only relatively.

### TO-CROWN-04 — George Tupou V's accession and proclamation

Evidence: Gazette No. 19 of 11 September 2006, English text (`to_crown_gtv_devolution_proclamation_20060911`):
"by whose decease the Crown of the Kingdom of Tonga devolves upon Crown Prince Tupouto'a", and
the proclamation that he "has now become the only lawful and rightful heir to the throne, King
of the Kingdom of Tonga". The Tongan text (`to_crown_gtv_devolution_proclamation_to_20060911`)
says the same in the name of SIAOSI TUPOU V. The PMO's statement of 18 August 2006
(`to_crown_succession_not_oath_dependent_2006`) says succession does not await the coronation
oath. IPU 2010 (`to_ipu_2010_gtv_acceded_sept2006`) and the Lord Chamberlain's profile
(`to_crown_gtv_profile_proclaimed_sept2006`) corroborate September 2006. He is attested as King
at the UN on 12 September 2006 (`to_crown_gtv_king_20060912_un`), by assent on 3 October 2006
(`to_crown_assent_gtv_20061003`) and by IPU's 2 May 2008 Speaker appointment
(`to_ipu_2008_gtv_speaker_20080502`), all before his coronation.

Decision: accepted. The accession by devolution is George Tupou V's `from`, 2006-09-11: the
Tongan date of the death, to which the proclamation's wording ties the transfer of the Crown.
The proclamation is a separate claim with its own date. Both are 11 September on the Tongan
calendar; they are not merged, and the proclamation date is not the reason for the start.

Limits: no accession time is stored. The accession date equals the death date only because the
source says the Crown devolves by the decease. Neither gazette text states the time of death;
that comes from Gazette No. 20. No facsimile of Gazette No. 19 was found.

### TO-CROWN-05 — George Tupou V's traditional installation and coronation

Evidence:

- The PMO programme captured on 31 July 2008 (`to_crown_gtv_taumafa_kava_scheduled_20080730`,
  `to_crown_gtv_coronation_scheduled_20080801`) schedules the "Traditional Installation Ceremony
  ... Taumafa Kava" at Mala'e Pangai on Wednesday 30 July and "THE CORONATION OF HIS MAJESTY KING
  GEORGE TUPOU V" at the Free Wesleyan Centenary Church on Friday 1 August 2008.
- A palace release datelined 30 July 2008 and issued by Tu'ivanuavou, Keeper of the Palace
  Records (`to_crown_gtv_taumafa_kava_held_20080730`), says the installation "took place today
  (July 30th)" at Pangai Lahi. It also reports the talking chief's declaration that from that
  moment the King became King of Tonga, and calls the taumafa kava the "true coronation" to
  Tongans (`to_crown_gtv_taumafa_kava_declaration_20080730`).
- The Prime Minister's Coronation Royal Luncheon address, datelined 1 August 2008, refers to
  "today's Coronation" (`to_crown_gtv_coronation_day_address_20080801`) and to the traditional
  investiture as Tu'i Kanokupolu "On Wednesday this week" (`to_crown_gtv_investiture_address_wednesday`).
- The PMO release of 1 August 2008 on the visiting royal guests (`to_crown_gtv_coronation_guests_20080801`)
  and the Japanese Imperial Household Agency itinerary (`to_crown_gtv_coronation_iha_20080801`).

Decision: accepted. The traditional installation (30 July 2008) and the coronation (1 August
2008) are separate claims on `to_king`; neither is a start date. The customary declaration is
kept because it is exactly the kind of statement that must not become an accession: the
accession followed from the devolution of the Crown in 2006.

Limits: the venue is printed as Mala'e Pangai in the programme and Pangai Lahi in the release;
both are kept. The coronation rests on the schedule plus same-day records; no post-event
narrative of the church ceremony was found. No source dates the clause 34 coronation oath.

### TO-CROWN-06 — George Tupou V's death

Evidence: Gazette Extraordinary No. 8 of 19 March 2012 (`to_crown_gtv_death_20120318`): Prime
Minister Lord Tu'ivakano announces the death in Hong Kong at 15:58 Hong Kong time, "which was
20:58hrs (Tonga time), on 18 March 2012". The same notice declares three months' mourning
(`to_crown_gtv_mourning_declared_20120319`), revoked on 30 March and reset to 19-31 March
(`to_crown_gtv_mourning_revised_20120330`).

Decision: accepted. George Tupou V's `until` is 2012-03-18. The two mourning instruments are
separate claims at entry level.

### TO-CROWN-07 — Tupou VI's accession and proclamation

Evidence: Gazette Extraordinary No. 9 of 19 March 2012 (`to_crown_tupou_vi_devolution_proclamation_20120319`):
"by whose decease the Crown of the Kingdom of Tonga devolves upon Crown Prince Tupouto'a Lavaka",
proclaimed King as Tupou VI, given at Nuku'alofa on 19 March 2012. The PMO article of
28 September 2006 (`to_crown_crown_prince_tupouto_a_lavaka_2006`) links "Crown Prince Tupouto'a
Lavaka" to the former Prince 'Ulukalala Lavaka Ata. Gazette Extraordinary No. 14 attests Tupou VI
as King on 30 March 2012, and the existing `to_tupou_vi_2025` on 18 December 2025.

Decision: accepted. Tupou VI's `from` is 2012-03-18, the death on which the proclamation says the
Crown devolved; the proclamation of 19 March is a separate claim. `until` is null.

Limits: the Crown Prince vesting is dated only "yesterday" and has no structured date. No end of
the reign is recorded before the cutoff, and none is inferred from the last attestation.

### TO-CROWN-08 — Tupou VI's coronation

Evidence: the National Reserve Bank of Tonga's 2014/15 Annual Report
(`to_crown_tupou_vi_coronation_20150704_nrbt`): uncut banknote sheets carry the serial
KT 04.07.15 "to commemorate the Coronation of King Tupou VI on the 4th July 2015". The Imperial
Household Agency's Cabinet understanding of 12 June 2015 and itinerary
(`to_crown_tupou_vi_coronation_iha_20150704`) place the coronation at the Free Wesleyan Centenary
Church on the morning of 4 July. Gazette import exemptions "for the Coronation of His Majesty
Tupou VI", in force 25 June to 7 July 2015 (`to_crown_tupou_vi_coronation_exemptions_2015`), are
consistent context.

Decision: accepted, with the evidence tier labelled in the claims: **a non-ceremonial Tongan
public body plus foreign corroboration** (defect D11). The coronation is a claim on `to_king`
and not a start date.

Limits: no palace, PMO or Gazette ceremony record was found (AGO 2015 gazettes 20, 25, 26 and
27 were checked by the checker and none records it). No source dates the coronation oath.

## Sources added

| Source ID | What | Retained provenance |
|---|---|---|
| `to_act_income_tax_amend_1990` | [Income Tax (Amendment) Act 1990](https://ago.gov.to/cms/images/LEGISLATION/AMENDING/1990/1990-0004/IncomeTaxAmendmentAct1990.pdf), assent 12 Jul 1990 | 35,041 bytes, SHA-256 `25d2c916…85c419`; page 1 viewed |
| `to_act_constitution_amend_1990` | [Act 12 of 1990](https://ago.gov.to/cms/images/LEGISLATION/AMENDING/1990/1990-0012/ActofConstitutionofTongaAmendmentAct1990.pdf), assent 8 Aug 1990 | 32,413 bytes, `4eab11f4…dca90a`; text layer only |
| `to_tlr_2006` | [Tonga Law Reports 2006](https://ago.gov.to/cms/ago-materials/publications/tonga-law-reports.html?download=1560:2006_tlr) (Lasike; Flyniu) | 2,053,972 bytes, `2b4b56d0…4c3aef`; pages 166 and 170 viewed |
| `to_act_income_tax_amend_2006` | [Income Tax (Amendment) Act 2006](https://ago.gov.to/cms/images/LEGISLATION/AMENDING/2006/2006-0002/IncomeTaxAmendmentAct2006.pdf), assent 30 Jun 2006 | 28,627 bytes, `379391bd…934731`; page 1 viewed |
| `to_pmo_20060313_qsc_anniversary` | PMO, Queen Salote College anniversary, 13 Mar 2006 (archived) | 13,490 bytes, `3c22f497…74b3b5`; capture 2006-09-07 |
| `to_pmo_20060601_throne_speech` | PMO, Princess Regent's Speech from the Throne, 1 Jun 2006 (archived) | 23,572 bytes, `2643d47b…223803`; capture 2006-10-05 |
| `to_pmo_20060818_succession` | PMO, "Succession to the Tongan Throne", 18 Aug 2006 (archived) | 11,836 bytes, `e910e5c3…8011b5`; capture 2006-10-18 |
| `to_pmo_20060911_proclamation` | PMO reproduction of Gazette No. 19 (English), 11 Sep 2006 (archived) | 11,706 bytes, `637e1449…f5d378`; capture 2006-10-05 |
| `to_pmo_20060911_death_notice` | PMO reproduction of Gazette No. 20, 11 Sep 2006 (archived) | 11,613 bytes, `0e8ebd4c…64c73e`; capture 2007-05-14; re-download matched |
| `to_pmo_20060911_proclamation_to` | PMO reproduction of Gazette No. 19 (Tongan), 11 Sep 2006 (archived) | 12,662 bytes, `f0252539…a6ec77`; capture 2007-03-12; re-download matched |
| `to_pmo_20060914_funeral_holiday` | PMO, funeral holiday notice of 13 Sep, article of 14 Sep 2006 (archived) | 13,615 bytes, `9947f777…8a9001`; capture 2006-10-17 |
| `to_pmo_20060920_last_tribute` | PMO, burial report, 20 Sep 2006 (archived) | 12,099 bytes, `c9de08e9…0bb993`; capture 2006-11-05 |
| `to_pmo_20060928_crown_prince` | PMO, Crown Prince Tupouto'a Lavaka, 28 Sep 2006 (archived) | 12,821 bytes, `4db64e8e…18b68a`; capture 2006-10-05 |
| `to_act_public_enterprises_amend_2006` | [Public Enterprises (Amendment) Act 2006](https://ago.gov.to/cms/images/LEGISLATION/AMENDING/2006/2006-0003/PublicEnterprisesAmendmentAct2006.pdf), assent 3 Oct 2006 | 55,577 bytes, `dfbd2d28…61ce99`; page 1 viewed |
| `to_pmo_2008_coronation_programme` | PMO coronation programme, 29 Jul - 2 Aug 2008 (archived, undated) | 25,085 bytes, `c574845d…bd053b`; capture 2008-07-31 |
| `to_pmo_20080728_lord_chamberlain_profile` | Lord Chamberlain's profile of the King, 28 Jul 2008 (archived) | 41,693 bytes, `31da340f…35ef03`; capture 2008-08-01 |
| `to_pmo_20080730_taumafa_kava` | Palace release "Sacred Kava and Mats of Power", 30 Jul 2008 (archived) | 27,576 bytes, `8fa1b7c0…a64e75`; capture 2008-08-07; re-download matched |
| `to_pmo_2008_pm_luncheon_address` | PM's Coronation Royal Luncheon address, 1 Aug 2008 (archived) | 32,168 bytes, `68506f37…46e8aa`; capture 2008-08-07 |
| `to_pmo_20080801_royal_visitors` | PMO release on royal visitors, 1 Aug 2008 (archived) | 24,911 bytes, `ad3fd360…244206`; capture 2008-08-07 |
| `to_gazette_ext_8_2012` | [Gazette Extraordinary No. 8 of 2012](https://ago.gov.to/cms/images/LEGISLATION/GAZETTES/2012/2012-0008/GazetteExtraordinaryNo.8of2012.pdf) | 659,563 bytes, `ec50ad5d…ef0b6b`; page 1 viewed |
| `to_gazette_ext_9_2012` | [Gazette Extraordinary No. 9 of 2012](https://ago.gov.to/cms/images/LEGISLATION/GAZETTES/2012/2012-0009/GazetteExtraordinaryNo.9of2012.pdf) | 662,692 bytes, `773fd96d…94d3e9`; page 1 viewed |
| `to_gazette_ext_14_2012` | [Gazette Extraordinary No. 14 of 2012](https://ago.gov.to/cms/images/LEGISLATION/GAZETTES/2012/2012-0014/GazetteExtraordinaryNo.14of2012.pdf) | 650,317 bytes, `fd66a32f…4cad33`; page 1 rendered |
| `to_gazette_supp_4_2015` | [Gazette Supplement Extraordinary No. 4 of 2015](https://ago.gov.to/cms/images/LEGISLATION/GAZETTES/2015/2015-0007/GazetteSupplementExtraordinaryNo.4of2015.pdf) | 11,341 bytes, `fb5fa4fa…e74877`; page 1 viewed |
| `to_nrbt_ar_2015` | [NRBT Annual Report 2014/15](https://www.reservebank.to/data/docs/publications/ar/annual_report_2015_eng.pdf) | 12,583,944 bytes, `fc1b8717…c2041e`; PDF page 30 viewed |
| `to_un_a61_pv1` | [UN General Assembly A/61/PV.1](https://documents.un.org/doc/undoc/gen/n06/520/19/pdf/n0652019.pdf), 12 Sep 2006 | 56,229 bytes, `36a4903f…e23b8e`; page 1 viewed |
| `to_iha_2008_tonga_visit` | [Imperial Household Agency, 2008 visit](https://www.kunaicho.go.jp/activity/gonittei/02/gaikoku/h20tonga/cpv-h20-tonga.html) | 11,320 bytes, `b357802d…d0dc41` |
| `to_iha_2015_tonga_visit` | [Imperial Household Agency, 2015 visit](https://www.kunaicho.go.jp/activity/gonittei/02/gaikoku/h27tonga/cpv-h27-tonga.html) | 10,032 bytes, `9f90055b…9fd884` |

The archived PMO pages are raw Internet Archive captures (`id_` form) of the original
`pmo.gov.to` pages, which are no longer served; each source records the capture URL as `url`
and the original address as `original_url`, and each extract records the capture time. The
three captures located by the checker (Gazette No. 20, the Tongan Gazette No. 19 and the
taumafa kava release) were downloaded again for this packet on 21 September 2026 and matched
byte for byte. "Viewed" pages were rendered and visually checked in the research dossier; the
others were read from the text layer or as HTML text.

Each new source has a checked-in derived factual extract under [sources/](sources/) (four
`tonga-act-*`, one `tonga-tlr-*`, fourteen `tonga-pmo-*`, four `tonga-gazette-*`, one `tonga-nrbt-*`,
one `tonga-un-*` and two `tonga-iha-*`, 27 in all). Every extract
repeats the packet's claims exactly; its own checksum is in the packet, separate from the
original-response hash. Original PDFs, pages and renders are not checked in; no seal, coat of
arms, signature or photograph is republished.

Source types: `primary_legislation_reproduction` and `primary_legislation_print` (AGO Acts),
`primary_court_record` (law report), `primary_government_release_archived`,
`primary_gazette_web_reproduction_archived` and `primary_palace_release_archived` (PMO),
`primary_government_gazette` (AGO gazettes), `primary_central_bank_statutory_report` (NRBT),
`intergovernmental_official_record` (UN) and `foreign_government_official_record` (IHA).

Reused sources. The seven claims on `to_constitution_2020`, `to_constitution_older`, `to_ipu_2008`
and `to_ipu_2010` were read on 21 September 2026; those sources keep their original access dates
and metadata, and each new claim says it was re-read. Constitution downloads: 2020 edition
789,048 bytes, `197f1dca…f004de`; older text 134,165 bytes, `a07b3daf…4cd21f` (fetched without the
packet URL's `zoom_highlight` query). IPU pages: two fresh fetches of each on 21 September 2026
returned HTTP 200 with the same size (12,614 and 17,521 bytes) but a different SHA-256 each time,
so no raw hash is asserted. Removing the single injected `<script>(function(){function c(){`
element (938 bytes), the rule already recorded for IPU 2014, leaves 11,676 bytes with SHA-256
`0da095e75feaa3de5568a652f22f89ae79e974e82de7104e44afc04d508f920a` (2008) and 16,583 bytes with
`af6bcefa59a7e96e520ddaacd5215715e966cb7b613181f777edb64a1864e825` (2010), identical across all
fetches and the checker's copies.

## Leads not imported

- US Department of State, press statement 2006/808 of 11 September 2006,
  [2001-2009.state.gov/r/pa/prs/ps/2006/72034.htm](https://2001-2009.state.gov/r/pa/prs/ps/2006/72034.htm)
  (13,848 bytes, `8875a9b2…96c71a`): Tupou IV died "yesterday" in Auckland. A foreign statement
  dated only relatively to a Washington dateline, superseded by Gazette No. 20. Kept as a lead:
  CLAUDE-C01-04's tests treat `state.gov` as a lead-only host, and that assertion is not loosened.
- PMO release of 12 June 2015 on a coronation holiday
  ([archived capture](https://web.archive.org/web/20150707000509id_/http://www.pmo.gov.to/special-national-holiday-to-honour-their-majesties-coronation/),
  47,299 bytes, `bdb313f8…e20752`): declares 7 July 2015 a holiday, with a programme from 27 June to
  6 July. It gives no coronation day and misnames the King "Tupou IV"; context only.
- NRBT Annual Report 2008/09 ([annual_report_2009_eng.pdf](https://www.reservebank.to/data/docs/publications/ar/annual_report_2009_eng.pdf),
  1,053,749 bytes, `4b37219b…539732`): a T$100 note launched on 25 July and issued on 30 July 2008 to
  mark the coronation; no coronation date.
- 2005 Acts assented "'ULUKALALA LAVAKA ATA, 10th January, 2006", for example the Criminal Offences
  (Amendment) Act 2005 ([AGO](https://ago.gov.to/cms/images/LEGISLATION/AMENDING/2005/2005-0018/CriminalOffencesAmendmentAct2005.pdf),
  29,312 bytes, `de6264a4…a1eacc`), while the Money Laundering and Proceeds of Crime (Amendment)
  Act 2005 (36,711 bytes, `a449fc8c…78c6dc`) is assented "TAUFA'AHAU TUPOU IV, 10th January, 2006". A
  possible regency lead; the conflict is unresolved and no capacity is printed.
- PMO "Program for the State Funeral" PDFs (`artman/uploads/eng__final_king_prog.pdf`,
  `tonga_final_tu_i__pdf.pdf`, linked from archived article 177 of 17 September 2006): no 2006
  capture exists; they may carry more detail on the death and funeral.
- PMO 15 September 2006 Lord Chamberlain release (archived article 174): George Tupou V's
  divestment of commercial interests; not needed.
- PMO 2008 sidebar link, "Address from the Throne of HRH the Princess Regent Opening of the 2008
  Legislative Assembly": a regency under George Tupou V, outside this packet.
- PMO report of 13 March 2006 names "The Acting Prime Minister, Hon. Dr. Feleti Sevele": an
  acting-premiership lead for `to_pm`, not imported here.
- Imperial Household Agency list of foreign visits
  ([gaikoku-h11-20.html](https://www.kunaicho.go.jp/about/gokomu/shinzen/gaikoku/gaikoku-h11-20.html)):
  attendance at Tupou IV's funeral, 18-20 September 2006; redundant.
- Foreign official pages: beehive.govt.nz release on Tupou IV, NZ Hansard of 12 September 2006,
  gg.govt.nz (2008), gg.gov.au (2012 funeral; 2015 coronation), a March 2012 White House
  statement. Corroborative only; not needed or not retrieved.
- IPU 2014 summary: Tupou VI endorsed the Cabinet that took office on 19 January 2015 (already in the
  packet as `to_ipu_2014_royal_endorsement_following_day`); not needed as a reign attestation.
- News and tertiary sources: Matangi Tonga (for example the new Crown Prince, 27 September 2006),
  Talanoa o Tonga (a later Crown Prince regency), Wikipedia, Royal Central and Unofficial Royalty.
  Leads only; the 23:34 New Zealand time they give is now sourced from Gazette No. 20.

## Sources attempted

- Palace sites (`palace.gov.to`, `www.palace.gov.to`, `palaceoffice.gov.to`): DNS did not resolve.
- Ministry of Information royalty page (`www.mic.gov.to/royaltynobility`): connection timed out.
- Live PMO site (`pmo.gov.to`, for example a coronation search): HTTP 403 JavaScript challenge;
  archived captures were used instead.
- AGO gazettes by year for 1990-2009: no rows listed; the 2006 and 2008 gazettes, including the
  Gazette No. 19 and No. 20 facsimiles, are not online. AGO regulations-by-year listings for 2006,
  2008, 2012 and 2015 returned empty tables.
- AGO 2015 gazettes 20, 25, 26 and 27 (checked by the checker): no coronation ceremony record.
- Internet Archive CDX and the funeral programme PDFs: intermittent outages; the English file
  resolves only to a 2024 capture returning 404, and the Tongan file has none.
- Legislative Assembly site search (coronation, regent, Tupou IV) and history page: reachable,
  no reign-chronology record.
- beehive.govt.nz release: a 212-byte stub to curl; not rendered in a browser tab.

## Checker defects

| # | Defect | Outcome |
|---|---|---|
| D1 | Major: Tupou IV's death recorded only on the New Zealand date, with no Tongan record | **Applied**: Gazette No. 20 reproduction added (`to_pmo_20060911_death_notice`); death stored with both printed times and the Tongan date; accession set to 11 Sep 2006; the proclamation kept separate; UN demoted to New Zealand-date corroboration; the US statement demoted further, to a lead (see Leads); dossier unresolved item 3 dropped, item 4 (no facsimile) kept |
| D2 | Taumafa kava rested on a prospective programme | **Applied**: the 30 July 2008 palace release imported as the occurrence record; the programme kept as the schedule; venue naming noted |
| D3 | Luncheon address treated as undated | **Applied**: `published_date` 2008-08-01, dateline in the locator and text, date based on the page, programme as corroboration |
| D4 | Older constitution claim dated 1988 from its label | **Applied**: period dropped; described as an edition text with amendments up to Act 12 of 1990 |
| D5 | "In force for 2004-2006" inferred from a 1990 consolidation | **Applied**: the 1990 claim says only that its wording is identical to 2020; the court's June 2006 application of clause 43 ([2006] Tonga LR 142, PDF page 168) is in the court claim |
| D6 | 2004 pleading given a July 2004 month | **Applied**: no structured date; "on or after 30 July 2004; approval date unstated"; kept as an unconfirmed allegation |
| D7 | Funeral holiday source dated 13 September | **Applied**: `published_date` 2006-09-14, `document_date` 2006-09-13 |
| D8 | Clause 43 paraphrased as a nobles' ballot | **Applied**: the clause wording is quoted |
| D9 | IPU hashes said to be unavailable (403) | **Applied as corrected disclosure**: the checker's raw hashes are not reproducible (per-request script); normalised hashes and the rule are recorded above; the reused source records are unchanged |
| D10 | Tongan Gazette No. 19 text not used | **Applied**: added as corroboration (`to_pmo_20060911_proclamation_to`) after verifying its content and hash |
| D11 | Flag: 2015 coronation rests on non-ceremonial evidence | **Applied**: evidence tier labelled in the NRBT claim and here; a ceremony record remains missing |

Two further consistency changes follow the packet's rules rather than a numbered defect: the
burial and the Crown Prince vesting are dated only "yesterday", so, as with CLAUDE-C01-04's
"following day", they carry no structured date and their IDs no longer claim one
(`to_crown_tupou_iv_burial_reported_20060920`, `to_crown_crown_prince_tupouto_a_lavaka_2006`); and
the profile's proclamation month is a month `period`, not the profile's issue date.

## Suggested next work orders

These are proposals for the integrator. They are not created in `work-orders.json`.

- `C01-Tonga-CROWN-002`: Privy Council or Gazette instruments appointing the 2004-2006 Prince and
  Princess Regents, with their dates; the capacity behind the TUPOUTO'A (30 June 2006) and
  'ULUKALALA LAVAKA ATA (10 January 2006) assents.
- `C01-Tonga-CROWN-003`: regencies under George Tupou V and Tupou VI, starting from the 2008
  Princess Regent throne address.
- `C01-Tonga-CROWN-004`: a palace, PMO or Gazette record of the 4 July 2015 coronation, and any
  record dating either coronation oath (clause 34).
- `C01-Tonga-CROWN-005`: Gazette Nos. 19 and 20 of 2006 in facsimile (National Archives or a
  library holding), and the State Funeral programme PDFs.
- `C01-Tonga-PM-00x`: the acting premiership recorded on 13 March 2006, for `to_pm`.

## Integration notes (outside this packet's file boundary)

- **Stacked:** this branch is based on `claude/c01-tonga-04` at `62690e61` (CLAUDE-C01-04). Merge
  CLAUDE-C01-04 first; against it, this branch is CLAUDE-C01-07 only.
- `research-index.json` is regenerated in a **separate commit**. New totals: 106 sources and
  1,688 claims (previously 79 and 1,644). Tonga role observations stay at 15; organization,
  institution, packet and batch counts are unchanged, and Tonga keeps one open batch,
  `C01-Tonga-DISC-B001`, with nine members.
- `research/README.md`, the C01 README totals and `docs/planning/ai-workstreams.json` are left for
  the integrator. This handoff is self-proposed and not registered there.
- `roles-and-lifecycle.json` records `taufaahau_tupou_iv` with `since` 1965-12-16 from tertiary
  pages, and Crown Prince Tupouto'a as `eligibility_research_required`. This packet supplies
  primary boundaries (11 September 2006; 18 March 2012) that the integrator may use there; that
  file is not edited here.
- Pinned tests: `test_tonga_research_s10g.py` now pins 64 sources and 110 claims (from 37 and 66).
  `test_tonga_dpfi_c01_04.py` pinned every holder's `until` as null and the stated `from` values
  as the two 2021 PMO dates; it now pins the exact set of stated `from` values (adding George
  Tupou V 2006-09-11 and Tupou VI 2012-03-18) and the exact set of stated ends (Tupou IV
  2006-09-11, George Tupou V 2012-03-18), so any other `until` still fails. No assertion was
  removed or loosened.
- New holder and claim field: death claims carry `printed_times` (zone, date and time as
  printed). The validator ignores it; the new test checks it. The atlas shows the three Crown
  holders as "Reported interval" cards; no UI code changed and no browser review was run.

## Checks

```text
python -X utf8 tools/avatars/campaign_research.py
python -X utf8 tools/avatars/campaign_research.py --check
python -X utf8 -m unittest discover -s tools/avatars -p "test_tonga_*.py"
python -X utf8 -m unittest discover -s tools/avatars -p "test_*research*.py"
python -X utf8 -m unittest discover -s tools/avatars -p "test_campaign*.py"
node --test tools/ui/check_leadership_research_review.cjs
python tools/planning/workboard.py --check
git diff --check
```

Results are recorded in the handoff.
