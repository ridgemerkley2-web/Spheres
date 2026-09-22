# Tonga transition 2024 03: resignation, caretaker service, Assembly selection, royal appointment and first Cabinet

Packet: **CLAUDE-C01-03**. State: **ready_for_review** (not complete).
Owner: Claude. Integrator/reviewer: Codex. Branch `claude/c01-tonga-03`, base
`04bc99a6`, claim commit `bc48be49`. Research access: 21 September 2026 (local;
direct fetches carry 22 September UTC). The historical cutoff stays **7 September 2026**;
every historical observation here falls between 23 November 2024 and 31 January 2025.

This packet reviews six observations about Tonga's 2024-2025 prime-minister transition,
from Siaosi Sovaleni to 'Aisake Valu Eke, and the first Eke Cabinet in
[tonga.json](tonga.json). It adds 17 sources and 27 claims, sets the end of the existing
Sovaleni observation from a stated resignation, and adds three holder observations on
existing roles (`to_pm` and `to_deputy_pm`). It adds no organization, institution, role,
acting-office role, game mapping, lifespan, portrait or avatar, and infers no party
leadership. The 2021 questions TO-TR21-04 and TO-TR21-05 are preserved unchanged. The
parent scope (C01, C06, S23, WC1 and CP1) remains open.

## Outcome

| ID | Question | Decision |
|---|---|---|
| TO-TR24-01 | End of Sovaleni's premiership | **Accepted:** resignation announced "effective immediately" in the House and accepted by the King on 9 Dec 2024; holder `until` 2024-12-09 |
| TO-TR24-02 | Caretaker or acting arrangement | **Accepted as attestation:** Samiu Kuita Vaipulu acting on 24 Dec 2024 and 6 Jan 2025; start, instrument and end **unresolved** |
| TO-TR24-03 | Assembly selection of the Prime Minister Designate | **Accepted:** Eke selected 16-8 on 24 Dec 2024; a claim, not a start of office |
| TO-TR24-04 | Royal appointment of Eke | **Accepted:** Clause 50A(1) appointment on 22 Jan 2025; holder added as an event observation, end unknown |
| TO-TR24-05 | Effectiveness, warrant, publication and oath | **Partly resolved:** posting date and 31 Jan oath recorded; warrant and any separate effective date **unresolved** |
| TO-TR24-06 | First Eke Cabinet | **Accepted:** effective 28 Jan 2025; letters forecast for 30 Jan; oaths 31 Jan; Deputy PM holder from 2025-01-28 |

### Date ledger

Each row is a separate dated fact with its own claim. Different dates for resignation,
acceptance, acting service, selection, appointment, posting, Cabinet effect, letters and
oath are not inconsistencies, and none is merged into another.

| Date | Event | Claim or field |
|---|---|---|
| 23 Nov 2024 | Notice of a no-confidence motion received by the Speaker's Office (context) | `to_vonc_notice_received_20241123` |
| 9 Dec 2024, 1150-1200 block | Sovaleni announces his resignation "effective immediately" | `to_sovaleni_resignation_statement_20241209`; Sovaleni holder `until` |
| 9 Dec 2024, recess before 1400 | King accepts; Palace letter read at 1400-1405 | `to_palace_acceptance_letter_20241209` |
| 9 Dec 2024, 1400-1430 | Motion removed; clause 50A Schedule started; Speaker rules the Cabinet remains with the Deputy PM acting | `to_speaker_vonc_withdrawn_procedure_20241209`, `to_speaker_caretaker_ruling_20241209` |
| 9 Dec 2024 | Members' list prints Samiu Vaipulu as Deputy PM | `to_deputy_pm_vaipulu_listed_20241209`; Vaipulu holder `attested_on` |
| 10 Dec 2024 | Nominations invited; Cabinet makes a statutory proclamation (Gazette SE 22) | `to_pm_nominations_invited_20241210`, `to_cabinet_proclamation_20241210` |
| 18 / 20 Dec 2024 | Nomination 1 (Eke) dated 18 Dec, received 20 Dec at 3:48pm | `to_pm_first_nomination_20241220`, `to_pm_election_nominations_read_20241224` |
| 23 Dec 2024 | Nomination 2 (Lātū) received at noon; nominations close at 4.30pm | `to_pm_two_nominations_20241223` |
| 24 Dec 2024 | Assembly selects Eke 16-8; the Speaker calls on Vaipulu as Acting PM | `to_eke_assembly_selection_20241224`, `to_acting_pm_vaipulu_floor_20241224` |
| 6 Jan 2025 | PMO: caretaker Government under Acting PM Samiu Kuita Vaipulu | `to_caretaker_acting_pm_vaipulu_20250106` |
| 22 Jan 2025 | King appoints Eke under Clause 50A(1) at the Royal Palace | `to_eke_royal_appointment_20250122`; Eke holder `attested_on` |
| 23 Jan 2025 (local) | Appointment release posted on the PMO site (derived, not a claim) | scope note of `to_pmo_eke_appointment_20250122` |
| 24 Jan 2025 | Live PMO release calls Eke Prime Minister | `to_eke_pm_pif_release_20250124` |
| 28 Jan 2025 | New ministers effective; Fusimalohi Deputy PM | `to_eke_cabinet_effective_20250128`; Fusimalohi holder `from` |
| 30 Jan 2025 | Letters of appointment (forecast only) | `to_eke_cabinet_letters_planned_20250128` |
| 31 Jan 2025 | Prime Minister's and ministers' oaths; Vaipulu listed outside the Cabinet | `to_eke_oath_20250131`, `to_assembly_news_oath_20250131`, `to_vaipulu_peoples_rep_listed_20250131` |
| unknown | Start, instrument and end of the acting premiership; revocation of the caretaker ministers | TO-TR24-02 |
| unknown | Warrant signature, presentation or number; any separate effective date | TO-TR24-05 |

## Observations

### TO-TR24-01 — End of Sovaleni's premiership

Evidence: minutes No. 48 of the Legislative Assembly, Monday 9 December 2024
(Tongan-language verbatim record).

- `to_sovaleni_resignation_statement_20241209` (pp.40-41, 1150-1200 blocks): with the
  no-confidence motion on the agenda, Prime Minister Sovaleni tells the House he resigns
  "Fakatatau ki he Konisitūtoné Kupu 50 (2)(c), effective immediately he 'ahó ni". The
  clause is printed as "50 (2)(c)"; the Speaker later reads clause 50(a)(2)(c). The
  printed form is kept.
- `to_palace_acceptance_letter_20241209` (p.42): the Speaker says the matter must be
  completed with His Majesty and adjourns to 2pm. At 1400-1405 he says the recess was
  used to convey it to His Majesty. The Table Clerk then reads a Palace Office letter
  dated 9 December 2024, signed by the Secretary to His Majesty. It says the Palace
  received Hon. Hu'akavameiliku's resignation letter that day and the King graciously
  accepted it.
- `to_speaker_vonc_withdrawn_procedure_20241209` (pp.43, 46, 50): the motion and reply
  are removed unvoted and released as public records. The Speaker reads clause
  50(a)(2)(c) and treats the resignation as confirmed by "tohi tali mei he 'Ene 'Afió"
  (His Majesty's written reply). The clause 50A Schedule then runs to a meeting on 24
  December.

Corroboration: the Assembly's news notice of 9 December
(`to_assembly_news_resignation_accepted_20241209`) and the Speaker's release of 10
December (`to_pm_nominations_invited_20241210`), which dates the vacancy to the
resignation on Monday 9 December 2024.

Decision: accepted. The existing `to_pm` holder "Siaosi 'Ofakivahafolau Sovaleni" now
has `until` 2024-12-09. It also cites the minutes and both resignation claims. Its
sentence "No end date or term is inferred." was replaced rather than contradicted.
The end rests on the stated resignation, not on Eke's later appointment. It is attached
to this holder because it is the resignation of the sitting Prime Minister, named
Hu'akavameiliku in both the 2021 Tongan release and the Palace letter. The spellings
"'Ofakivahafolau" (PMO 2021) and "'Ofa ki Vahafolau" (Assembly 2024) are recorded as
printed and are not the basis of the match.

Limits: the three moments fall on one day and are kept in order: the House statement
(1150-1200), the King's acceptance during the recess, and the reading of the letter
(1400-1405). The clock time of acceptance is not recorded. Neither is the date the
resignation letter was written or signed; the original letter and the Palace letter
are not published. `until` is exact to the day. No term length is inferred. The
no-confidence notice (`to_vonc_notice_received_20241123`) is context only; the motion
was never put to a vote.

### TO-TR24-02 — Caretaker or acting arrangement

Evidence:

- `to_speaker_caretaker_ruling_20241209` (9 Dec, pp.47-50): answering Lord Tu'ivakanō's
  request for an interim Cabinet, the Speaker has section 18(2) of the Government Act
  read. He rules that on a clause 50(a) resignation the Cabinet remains and the Deputy
  Prime Minister acts. He names no one.
- `to_deputy_pm_vaipulu_listed_20241209` (9 Dec, p.2): the members' list prints "Hon.
  Samiu Vaipulu" as Deputy Prime Minister and Minister of Justice and Prisons.
- `to_cabinet_proclamation_20241210`: Gazette Supplement Extraordinary No. 22 shows
  Cabinet exercising a statutory proclamation power on 10 December. It names no Prime
  Minister or Acting Prime Minister.
- `to_acting_pm_addressed_20241224` (24 Dec): the Speaker's, Lātū's and Eke's
  salutations greet the "'Eiki Palēmia Le'ole'o" (Acting Prime Minister). Lātū's also
  greets the acting members of the Cabinet. None of these names the Acting Prime Minister.
- `to_acting_pm_vaipulu_floor_20241224` (24 Dec, p.47, 1320-1325): the Speaker gives the
  floor with "me'a mai Palēmia Le'ole'o". The speech that follows is headed as Vava'u
  15's and carries the speaker label Samiu Vaipulu. The roll on p.3 prints "Samiu Kuita
  Vaipulu - ... Vava'u 15".
- `to_caretaker_acting_pm_vaipulu_20250106` (6 Jan 2025, PMO): Government is in
  "caretaker mode" under the Acting Prime Minister, Hon. Samiu Kuita Vaipulu, and the
  current Cabinet Ministers. It continues until Eke and his new Cabinet are appointed,
  after which the current ministers' appointments will be revoked. A final paragraph
  promises a later announcement of the commencement of the New Government.
- `to_vaipulu_peoples_rep_listed_20250131` (31 Jan 2025, p.2): Vaipulu is listed as
  People's Representative for Vava'u 15, outside the Cabinet list.

Decision: accepted as attestation. Vaipulu is attested as Acting Prime Minister on 24
December 2024 and 6 January 2025. These stay claims on `to_pm` and `to_prime_minister`,
not a `to_pm` holder: the packet has no acting-office role, and a holder there would read
as a substantive premiership. A `to_deputy_pm` holder "Samiu Vaipulu" is added as an
event observation (`attested_on` 2024-12-09), citing only the members' list, whose
printed name it uses.

Limits: **unresolved.** No instrument or start date for the acting premiership was found.
9 December is not inferred from the Speaker's ruling. No end is inferred from 22 or 28
January 2025. The 31 January listing is an upper bound on his deputy premiership, not an
end date. The forecast revocation of the Sovaleni-era ministers is undated, and the
promised commencement announcement was not identified. When Vaipulu became Deputy Prime
Minister, and whether anyone held the office between Poasi Mataele Tei (2021) and him,
is unknown; Tei's observation gains no end. The Government Act was not retrieved;
section 18(2) is taken only as read into the minutes.

Sources attempted: PMO WordPress search ("caretaker", "commencement", "new government",
"Public Announcement", "Acting Prime Minister"). The live PMO post list from 20 January
to 15 February 2025 has only three posts: 24 January, 28 January and 3 February.
The Wayback CDX of pmo.gov.to HTML captured 22 January-15 February 2025 has no
commencement notice. Also checked: the PMO `/2024/12/` archive (404) and the Assembly
news and press-release archives. The acting-premiership observation at the 24 December
meeting (p.47) was found while this packet was being assembled; the source dossier had
called the Acting Prime Minister there unnamed. It was read from the PDF text layer, not
from a rendered page.

### TO-TR24-03 — Assembly selection on 24 December 2024

Evidence: invitations issued on 10 December (`to_pm_nominations_invited_20241210`); the
first nomination received on 20 December (`to_pm_first_nomination_20241220`); two
nominations at the 4.30pm close on 23 December (`to_pm_two_nominations_20241223`). The
minutes of the special meeting of Elected Representatives on 24 December 2024:

- `to_pm_election_nominations_read_20241224`: nomination 1, dated 18 December and received
  20 December at 3:48pm, names 'Aisake Valu 'Eke. Nomination 2, dated and received 23
  December at noon, names Viliami Uasikē Lātū. Proposers and seconders are recorded as
  nomination roles only.
- `to_eke_assembly_selection_20241224` (pp.52-53, 1500-1505): the Speaker says he has
  reported the result to the King, then announces the secret ballot. There were 24 votes,
  a majority being 13: Eke 16, Lātū 8. The Clerk read the result and the Auditor
  General confirmed it.

Corroboration: the Assembly news notice (`to_assembly_news_eke_elected_20241224`), which
says the nation "awaited formal confirmation". The PMO's 22 January release
(`to_eke_assembly_designate_pmo_20250122`) is also attested on 24 December.

Decision: accepted. The clause 50A selection stays a claim; it is not a holder
observation and not the start of office.

Limits: the time of the report to the King is not given. The Assembly news notice of 9
December says the 14 days "starts today 9th of December", while the Speaker's ruling
counts from the next day. The minutes and the 10 December release govern. Nominations,
candidacies, proposers and seconders establish no party office or affiliation.

### TO-TR24-04 — Royal appointment on 22 January 2025

Evidence: `to_eke_royal_appointment_20250122`. The PMO release is datelined "Nuku'alofa,
22 January, 2025". It says King Tupou VI, "today, Wednesday 22 January 2025, at the
Royal Palace, Nuku'alofa", appointed Eke "as the Prime Minister Tonga" (as printed; the
title has "of") under Clause 50A (1). The live post has been removed. Its text is read
from two Wayback captures (23 January and 17 February 2025) with the same wording; the
raw capture hash was reproduced. A live PMO joint release dated and posted 24 January
2025 calls Eke "The Prime Minister of Tonga and Chair of the Pacific Islands Forum"
(`to_eke_pm_pif_release_20250124`). IPU corroborates "January 2025" at month precision
(`to_ipu_2025_transition`).

Decision: accepted. A `to_pm` holder "'Aisake Valu Eke" is added with `attested_on`
2025-01-22 and `from`/`until` null, citing only the appointment claim. This follows the
event-dated 2019 Tu'i'onetoa pattern: the release reports an appointment event, not a
stated "with effect from" date. The 24 January release is a corroborating claim, not a
holder source.

Limits: the four weeks between selection (24 December) and appointment (22 January) are
not explained by any retrieved record. The release's "19th Prime Minister" ordinal is
recorded as printed and not used to number a succession. The 24 January release misprints
its meeting date as "24 February 2025"; that date is not imported. No end of Eke's
premiership is inferred. The packet's existing 18 December 2025 Fakafanua appointment
does not supply one.

### TO-TR24-05 — Effectiveness, warrant, publication and oath

What the primary records say: the release says only that the King "has appointed" Eke
that day. It gives no separate effective date and no signature, presentation, time or
number of a Royal Warrant. It is datelined 22 January but was posted on 23 January local
time. The 15:51 UTC capture on 23 January shows "12 hours ago", which puts publication at
about 03:20-04:20 UTC (16:20-17:20 local, UTC+13). The WordPress media record of the
post's portrait image is dated 2025-01-23T15:42:27 local (02:42:27 UTC). This posting
date is derived, not a claim. The Assembly sitting notice of 29 January
(`to_oath_sitting_notice_20250129`) announced the oaths. On Friday 31 January 2025 Eke
took the clause 83 oath before the Assembly as Prime Minister and Minister of Finance, of
Fisheries and of Prisons (`to_eke_oath_20250131`). The Assembly's news notice of 3
February (`to_assembly_news_oath_20250131`) calls this "the official beginning of their
term in Parliament"; the notice's wording is not read as the start of office.

Decision: **partly resolved.** Publication (derived) and oath are recorded as separate
events. The warrant and any separate effective date are **unresolved**.

Sources attempted: the Attorney General's Office "Gazettes by year" listings for 2024
and 2025 (fetched by POST). These are selective: their numbering has gaps and they list
mostly legislation, so the absence of an appointment notice there rules nothing out.
palace.gov.to does not resolve. archive.ph has no capture of the release. News accounts
(see Leads not imported) are leads only and are not used.

### TO-TR24-06 — First Eke Cabinet

Evidence: the PMO release dated 28 January 2025 (two-page PDF):

- `to_eke_cabinet_effective_20250128`: on the Prime Minister's recommendations, the King
  appointed the new Ministers of Government "with effect from today, Tuesday 28th January
  2025". The numbered list has eleven members. Item 1 is Eke (Prime Minister; Finance;
  Fisheries; Prisons). Item 2 is Hon. Dr. Taniela Likuohihifo Fusimalohi (Deputy Prime
  Minister; Infrastructure; MEIDECC). Item 3 is the Crown Prince (Foreign Affairs; His
  Majesty's Armed Forces).
- `to_eke_cabinet_letters_planned_20250128`: the Prime Minister "will present" the
  letters of appointment on Thursday 30 January 2025.

The minutes of 31 January record the ministers' oaths, Fusimālohi's among them (pp.17-18).

Decision: accepted. A `to_deputy_pm` holder "Taniela Likuohihifo Fusimalohi" (the form the
PMO release prints) is added with `from` 2025-01-28, citing only the effective-date claim.
The minutes' form "Taniela Liku'ohihifo Fusimālohi" is kept in the holder note. Letters and
oath stay separate claims.

Limits: the 30 January presentation of letters is a forecast; no record confirms it. Item
1 is not a second start date for the premiership. Items 4-11 are not imported, and
`to_ministers` stays empty. The end of the caretaker Cabinet is not inferred from 28
January, and neither is the end of Vaipulu's deputy premiership.

## Sources added

| Source ID | What | Retained provenance |
|---|---|---|
| `to_assembly_vonc_notice_20241125` | [Assembly press release](https://parliament.gov.to/en/media-centre/press-releases/office-of-the-lord-speaker-receives-notice-of-intention-to-move-a-motion-for-a-vote-of-no-confidence-in-the-prime-minister), issued 25 Nov 2024 | Direct fetch, 57,203 bytes; **no hash asserted** (per-request form token; live hit counter) |
| `to_assembly_minutes_48_20241209` | [Minutes No. 48](https://parliament.gov.to/en/parliament-business/hansards-debates/3-2024/131-miniti-fika-48-aho-09-o-tisema-2024), 9 Dec 2024 | PDF 404,595 bytes, SHA-256 `9e9cf1dd…c27d5`, reproduced; pp.2, 3, 40-43, 46, 48-50 rendered, p.47 text layer only |
| `to_assembly_resignation_news_20241209` | [Assembly news notice](https://parliament.gov.to/en/media-centre/latest-news/king-tupou-vi-accepts-resignation-of-prime-minister-sovaleni-ahead-of-no-confidence-vote), created 9 Dec 2024 | Direct fetch, 56,637 bytes; no hash asserted |
| `to_assembly_pm_nominations_open_20241210` | [Speaker's release](https://parliament.gov.to/en/media-centre/press-releases/nomination-of-candidates-for-prime-minister-now-open), issued 10 Dec 2024 | Direct fetch, 55,980 bytes; no hash asserted |
| `to_gazette_gse22_20241210` | [Gazette SE No. 22](https://ago.gov.to/cms/images/LEGISLATION/GAZETTES/2024/2024-0047/GazetteSupplementExtraordinaryNo.222024.pdf), 10 Dec 2024 | PDF 273,587 bytes, `f5f830a9…ec6fa5b`, reproduced; text layer read |
| `to_assembly_first_nomination_20241220` | [Lord Speaker's release](https://parliament.gov.to/en/media-centre/press-releases/the-office-of-the-lord-speaker-of-the-legislative-assembly-received-the-first-nomination-for-prime-minister-designate-today), issued 20 Dec 2024 | Direct fetch, 56,828 bytes; no hash asserted |
| `to_assembly_two_nominations_20241223` | [Lord Speaker's release](https://parliament.gov.to/en/media-centre/press-releases/a-total-of-two-prime-minister-candidate-nominations-received-by-the-office-of-the-lord-speaker-of-parliament), issued 23 Dec 2024 | Direct fetch, 56,838 bytes; no hash asserted |
| `to_assembly_minutes_pm_election_20241224` | [Minutes of the PM election](https://parliament.gov.to/en/parliament-business/hansards-debates/3-2024/132-miniti-fili-palemia-aho-24-o-tisema-2024), 24 Dec 2024 | PDF 412,540 bytes, `d798e123…85443347`, reproduced; pp.3, 7-9, 11-13, 52, 53, 55 rendered, pp.19 and 47 text layer only |
| `to_assembly_eke_elected_news_20241224` | [Assembly news notice](https://parliament.gov.to/en/media-centre/latest-news/parliament-elects-hon-dr-aisake-valu-eke-as-tonga-s-new-prime-minister-designate), created 24 Dec 2024 | Direct fetch, 60,909 bytes; no hash asserted |
| `to_pmo_caretaker_announcement_20250106` | [PMO Public Announcement](https://pmo.gov.to/14107-2/) (post now 404), 6 Jan 2025 | Image 1 (English) 137,870 bytes, `db9c580d…4ed9d7d250`; image 2 (Tongan) 160,214 bytes, `682a45b5…6088fa488`; both viewed. Post page from Wayback `20250126154340id_`, 69,871 bytes, `7fce529e…f25f91163` |
| `to_pmo_eke_appointment_20250122` | [PMO appointment release](https://pmo.gov.to/his-majesty-king-tupou-vi-appoints-hon-dr-aisake-valu-eke-as-prime-minister-of-tonga/) (post now 404), dated 22 Jan 2025 | Wayback `20250123155100id_`, 66,444 bytes, `ed2e113e…2690354a`, reproduced; second capture `20250217164854id_`, 75,048 bytes, `a4212306…23837df8` |
| `to_pmo_pif_joint_release_20250124` | [PMO joint media release](https://pmo.gov.to/pacific-islands-forum-secretary-general-meets-new-prime-minister-of-tonga/), 24 Jan 2025 | Page 71,411 bytes, `be3dff70…13c30d76`, same bytes on two in-browser requests; REST record 7,313 bytes; non-browser requests get HTTP 403 |
| `to_pmo_eke_cabinet_20250128` | [PMO Cabinet release](https://pmo.gov.to/prime-minister-hon-dr-aisake-valu-eke-announces-new-cabinet-ministers/), 28 Jan 2025 | PDF 396,595 bytes, `ad766e4d…a318ae134`, reproduced ('-1.pdf' identical); both pages rendered; page HTML 82,136 bytes (browser), no hash asserted |
| `to_assembly_oath_sitting_notice_20250129` | [Assembly news notice](https://parliament.gov.to/en/media-centre/latest-news/parliament-to-sit-of-friday-31st-january), created 29 Jan 2025 | Direct fetch, 54,109 bytes; no hash asserted |
| `to_assembly_minutes_01_20250131` | [Minutes No. 1 of 2025](https://parliament.gov.to/en/parliament-business/hansards-debates/2-2025/111-miniti-fika-01-aho-31-o-sanuali-2025), 31 Jan 2025 | PDF 410,718 bytes, `3ef9f082…38cd320c`, reproduced; pp.2, 7-9, 17 rendered, pp.18 and 23 text layer only |
| `to_assembly_oath_news_20250203` | [Assembly news notice](https://parliament.gov.to/en/media-centre/latest-news/tonga-prime-minister-and-cabinet-ministers-take-oath-of-office-in-parliament), created 3 Feb 2025 | Direct fetch, 70,501 bytes; no hash asserted |
| `to_ipu_2025` | [IPU Parline, November 2025 election](https://data.ipu.org/parliament/TO/TO-LC01/election/TO-LC01-E20251120/) | Direct fetch, 165,854 bytes twice with different hashes (dynamic tokens); no hash asserted; corroboration only |

Each source has a checked-in derived factual extract under [sources/](sources/). The
files are eleven `tonga-assembly-*`, four `tonga-pmo-*`, one `tonga-gazette-*` and one
`tonga-ipu-*`. The extract's own checksum is in the packet and is distinct from any
original-response hash. Where the resource hashed differs from the source URL, the
extract names it in `source_response_url`: the English release image, the Wayback
capture, the release PDF, or the minutes' file page with its download request. Original
pages, PDFs and images are not checked in. No source artwork, coat of arms or photograph
is republished.

The minutes are downloaded by posting the Phoca Download form on each file page. The
file page for minutes No. 48 is served from a page cache with a stale form token, so a
plain re-request can get an empty HTTP 303. Requesting the file page with a
cache-busting query gives a fresh token and the full PDF. All three minutes hashes were
reproduced on 21 September 2026.

## Leads not imported

- Matangi Tonga, 22 January 2025,
  [king-appoints-new-prime-minister-hon-dr-aisake-eke](https://matangitonga.to/2025/01/22/king-appoints-new-prime-minister-hon-dr-aisake-eke):
  reports the appointment at the Royal Palace at 9:53am on 22 January. News; the time
  of day is not imported.
- RNZ, [Tonga PM officially appointed](https://www.rnz.co.nz/news/pacific/539609/tonga-pm-officially-appointed):
  published 23 January 2025 (10:07am); reports the appointment without dating it, so its
  publication date is not an appointment date. News; not imported.
- US News (Reuters wire), 21 January 2025 (US dateline),
  [tongas-king-appoints-eke-prime-minister-after-predecessor-quit](https://www.usnews.com/news/world/articles/2025-01-21/tongas-king-appoints-eke-prime-minister-after-predecessor-quit):
  wire news; not fetched.
- Permanent Mission of Tonga to the UN, [X post](https://x.com/TongaMissionUN/status/1882256344637382798),
  seen only as a search snippet: it agrees with the 22 January appointment. Whether a
  mission's social-media post counts as primary is left to the integrator.
- PMO posts 7313 (event of 7 February 2024), 7553 (24 April 2024) and 7735 (20 May 2024)
  call Samiu Kuita Vaipulu (or "Samiu K Vaipulu") "Acting Prime Minister" during
  Sovaleni's premiership, while Sovaleni was absent. These are separate absence-acting
  spells and must not be merged with the December 2024-January 2025 caretaker
  arrangement. None says "Deputy Prime Minister", but they are leads for the start of
  his deputy premiership.
- PMO page "Former Prime Ministers of Tonga" (`/former-prime-minsiters/`, WordPress
  modified 2026-03-06T21:16:46 local, before the cutoff): year ranges only, undated per
  entry. Its "Tu'i'onetoa (2020-2021)" conflicts with the packet's 8 October 2019
  appointment. Not imported for those reasons.
- Assembly video interviews with Lord Speaker Fakafanua on the PM election (created 23
  December 2024): embedded video with no text; not transcribed.
- Assembly VONC publications (Motion No. 4A-2024 and the Prime Minister's response):
  released as public documents under the 9 December ruling; they concern the motion,
  not the transition. Not downloaded.
- Assembly press releases of 25 and 29 November 2024 (sitting adjourned; tabling
  postponed to 9 December): procedural background only.
- 24 December 2024 minutes, pp.7-8: Sovaleni asks whether three ministers appointed from
  outside the Assembly by the former Prime Minister are still members. A lead for
  caretaker-Cabinet membership; it names no one.
- 9 December 2024 minutes, p.48: Ha'apai 13 cites an earlier acting arrangement after a
  Prime Minister's death. A lead for the 2019 chain, outside this packet.
- The Palace letter and the 22 and 28 January releases as attestations of Tupou VI as
  King: possible `to_king` observations for the separate monarchy packet.
- Cabinet items 4-11 of the 28 January release; the PMO's uncaptioned appointment
  photographs; Wikipedia articles on the people named (tertiary, not read).

## Independent check: defects applied or declined

An independent re-check re-downloaded every hashed source, re-read the Assembly pages
and IPU, and listed 16 defects. All were applied:

1. Acceptance time: the recess bracket (1200-1400) is in the Palace-letter claim and the
   Sovaleni holder; `until` stays at day precision.
2. Role and entry source lists: explicit `sources` and `claim_ids` were added to `to_pm`,
   `to_prime_minister`, `to_cabinet` and `to_deputy_pm`, and the packet validates.
3. The Sovaleni holder's "No end date or term is inferred." was replaced, not appended to.
4. Vaipulu's 31 January 2025 listing outside the Cabinet is a new claim, used as an upper
   bound only.
5. Holder names: "Samiu Vaipulu" and "Taniela Likuohihifo Fusimalohi" use the forms
   printed in the only sources each holder cites.
6. Minutes hashes are recorded as reproduced; the stale-token cause replaces the
   rate-limit guess.
7. The "Former Prime Ministers" page was modified before the cutoff, not after; the
   reasons it is not imported were corrected.
8. The AGO gazette listing is described as selective, and its absence statements are
   weakened. Probing individual Gazette numbers is listed as next work.
9. Locators corrected: p.43 and p.46 time blocks, and the Deputy PM oath on pp.17-18.
   Text-layer-only pages are marked, and the p.2 pairing note names pdftotext.
10. The live 24 January PMO release (post 14141) was fetched and imported as a
    corroborating `to_pm` claim, not a holder source. Its misprint is noted.
11. The derived posting time is given in local time (UTC+13) and UTC.
12. The caretaker announcement's promised commencement notice is in the claim. A search
    was run: the live PMO post list, WordPress search and the Wayback CDX found none.
13. Each PMO and minutes extract names its `source_response_url`.
14. The 2024 absence-acting PMO posts are listed as leads, with a no-merge note.
15. The 9 December news notice's counting discrepancy is in its claim uncertainty.
16. The appointment claim records "as the Prime Minister Tonga" as printed.

Beyond the check: the 24 December minutes (p.47) identify the Acting Prime Minister as
Vaipulu. This is a new claim, `to_acting_pm_vaipulu_floor_20241224`.

## Suggested next work orders

These are proposals for the integrator. They are not created in `work-orders.json`.

- Government Act section 18(2) as in force in December 2024, and any Cabinet, Palace or
  PMO instrument that started or ended the acting arrangement (TO-TR24-02).
- Cabinet or Gazette record revoking the Sovaleni-era ministers and ending the caretaker
  period, and the PMO "commencement of the New Government" announcement (TO-TR24-02/06).
- Royal Warrant appointing Eke: probe individual Gazette and Supplement numbers for
  December 2024 to February 2025, and Palace records, instead of the selective index
  (TO-TR24-05).
- Confirmation that letters of appointment were presented on 30 January 2025.
- Eke Cabinet items 4-11 as dated `to_ministers` observations from the 28 January release.
- Vaipulu's own appointment as Deputy Prime Minister under Sovaleni, which would also
  bound Tei's 2021 observation; the 2024 absence-acting posts are the starting leads.

## Integration notes (outside this packet's file boundary)

- `research-index.json` is regenerated in a **separate commit** on this branch. New
  totals: 85 sources and 1,649 claims (previously 68 and 1,622). Organization,
  institution, packet and batch counts are unchanged. Tonga still has one open batch,
  `C01-Tonga-DISC-B001`, with nine members.
- **Browser CI pin:** `tools/ui/ci-leadership-research-browser.cjs` (lines 62 and 71)
  expects the Sovaleni card to read "Reported interval: 2021-12-27 → Not established".
  With the stated end it now reads "Reported interval: 2021-12-27 → 2024-12-09". Shared
  UI is outside this packet, so the script is unchanged. The integrator will need to
  update that expectation, or keep a separate check, before the browser run passes. The
  new holders read "Observed on 2025-01-22" (Eke), "Observed on 2024-12-09" (Vaipulu) and
  "Reported interval: 2025-01-28 → Not established" (Fusimalohi). No UI code changed.
- `research/README.md` and the S10.g paragraph in `C01/README.md` need the new totals and
  a pointer to this report. They are left for the integrator.
- `test_tonga_research_s10g.py` pins exact totals and was updated: 26→43 sources and
  44→71 claims. Its `to_pm` holder list pin now includes Eke. `test_tonga_transition_c01_02.py`
  pinned the Sovaleni holder with no end and two `to_pm` dict holders. It now pins the
  stated end (2024-12-09), the added resignation claims and source, and the third holder.
  The Sovaleni test was renamed to match what it asserts. No assertion was removed or
  loosened.
- This branch does not contain CLAUDE-C01-04 or CLAUDE-C01-07. It uses none of their
  IDs, and merging them will change the pinned totals again.

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

Results at submission: research index regenerated and `--check` passes (9 packets,
85 sources, 1,649 claims, 92 batches). 39 Tonga tests pass: 28 existing, with
pins updated in two files, and 11 new. The `test_*research*.py` pattern passes 79 tests. Under
`test_campaign*.py`, the 9 `test_campaign_research` tests pass. `test_campaign_census` cannot load in
this sparse worktree because it reads `spheres-sim/data/party_leaders.json`, which is outside the
checkout. The error does not involve this packet's files, and the checkout was not widened. 11 atlas
Node tests pass. The workboard check passes (44 markers). `git diff --check` is clean. The suites
overlap; their counts are not summed as distinct tests.
