# Tonga transition 2021 02: Assembly selection, royal appointment and first Cabinet

Packet: **CLAUDE-C01-02**. State: **ready_for_review** (not complete).
Owner: Claude. Integrator/reviewer: Codex. Branch `claude/c01-tonga-02`, base
`5b46e40a`, claim commit `852510d3`. Research access: 21 September 2026 (local;
direct fetches carry 22 September UTC). The historical cutoff stays **7 September 2026**.

This packet reviews six observations about the 2021 transition from Pohiva
Tu'i'onetoa to Siaosi Sovaleni in [tonga.json](tonga.json). It adds seven sources,
nine claims and two holder observations on existing roles (`to_pm` and
`to_deputy_pm`). It adds no organization, institution, second prime-minister office,
game mapping, lifespan, portrait or avatar. The parent scope (C01, C06, S23, WC1 and
CP1) remains open.

## Outcome

| ID | Question | Decision |
|---|---|---|
| TO-TR21-01 | Assembly procedure before the selection | **Accepted** as procedural records (20 Nov to 15 Dec 2021); none records a result |
| TO-TR21-02 | Assembly selection of Sovaleni on 15 Dec 2021 | **Accepted** on IPU and the PMO release; the Assembly's own result notice was not found |
| TO-TR21-03 | Royal appointment under Clause 50A | **Accepted:** effective 27 Dec 2021, releases dated 28 Dec; holder added from 2021-12-27, end unknown |
| TO-TR21-04 | Signature and presentation of the Royal Warrant | **Unresolved:** no primary date found; the release date is not a signature date |
| TO-TR21-05 | End of Tu'i'onetoa's premiership; any caretaker boundary | **Unresolved:** no primary record found; no end date inferred |
| TO-TR21-06 | First Sovaleni Cabinet | **Accepted:** effective 28 Dec; letters and oaths 29 Dec; Deputy PM holder added from 2021-12-28 |

### Date ledger

Each row is a separate dated fact with its own claim. Different dates for selection,
effect, publication, letters and oath are not inconsistencies, and none is merged into
another.

| Date (2021) | Event | Claim or field |
|---|---|---|
| 20 Nov | Lord Tangi's appointment as interim Speaker takes effect | `to_interim_speaker_tangi_20211120` |
| 30 Nov | Writ of election returned to the King; nominations invited | `to_pm_nominations_invited_20211130` |
| 13 Dec | First nomination received; meeting notice issued | `to_pm_election_meeting_notice_20211213` |
| 14 Dec, 4.30pm | Nominations close with two received | `to_pm_two_nominations_20211214` |
| 15 Dec | Assembly selects Sovaleni | `to_ipu_2021_sovaleni_assembly_selection`, `to_sovaleni_assembly_election_pmo_20211215` |
| 27 Dec | Royal appointment takes effect | `to_sovaleni_royal_appointment_en_20211227`, `to_sovaleni_royal_appointment_to_20211227`; holder `from` |
| 28 Dec | Both appointment releases dated | `published_date` of the two PMO appointment sources |
| 28 Dec | Ministerial appointments take effect | `to_sovaleni_cabinet_effective_20211228`; Deputy PM holder `from` |
| 29 Dec | Letters presented, first Cabinet meeting, oaths; Cabinet release dated | `to_sovaleni_cabinet_letters_oath_20211229`; `published_date` |
| unknown | Warrant signed or presented | TO-TR21-04 |
| unknown | Tu'i'onetoa's premiership ends | TO-TR21-05 |

## Observations

### TO-TR21-01 — Assembly procedure before the selection

Evidence: four Legislative Assembly news notices.

- Created 22 November (`to_assembly_interim_speaker_20211122`): King Tupou VI appointed
  Lord Tangi interim Speaker, effective Saturday 20 November 2021, to lead the process
  that elects the Prime Minister Designate after the 18 November general election. The
  writ had to be returned by 2 December. Invitations follow within 10 days of its return,
  nominations are due within 14 days of it, and a secret-ballot meeting follows within 3
  days of the close.
- Created 1 December (`to_assembly_pm_nominations_20211201`): invitations were issued
  after the writ was returned to the King on Tuesday 30 November. Nominations close at
  4.30pm on Tuesday 14 December.
- Created 13 December (`to_assembly_pm_meeting_20211213`): the Elected Representatives
  meet on Wednesday 15 December at 10am. One nomination had been received, on Monday
  13 December. The Speaker and Deputy Speaker are elected after the Prime Minister result.
- Created 14 December (`to_assembly_pm_nominations_20211214`): two nominations at the
  close, one on 13 December and one on 14 December. Candidates to be revealed on 15 December.

Decision: accepted as procedural records. They are claims on `to_pm` and
`to_prime_minister`, not holder observations. The notices use "elect the Prime
Minister" and "Prime Minister Designate" for the same Assembly stage. The King's
appointment under Clause 50A is a separate act (TO-TR21-03).

Limits: no notice names a candidate or records that the meeting took place. The
creation dates are page metadata; the 1 December invitations are dated only by the
notice's "today". Lord Tangi's appointment is not imported as a Speaker holder
observation, because Assembly offices are outside this packet.

### TO-TR21-02 — Assembly selection on 15 December 2021

Evidence: the existing IPU claim (`to_ipu_2021_sovaleni_assembly_selection`, unchanged)
says the newly elected Assembly chose Sovaleni over 'Aisake Eke on 15 December 2021.
The PMO's English release (second paragraph) independently states that Sovaleni was
"elected as Prime Minister by the Legislative Assembly on the 15th December"
(`to_sovaleni_assembly_election_pmo_20211215`).

Decision: accepted. The selection stays a claim. It is not a holder observation and not
the start of office.

Limits: the Assembly's own notice of the result was not found. In its news sequence the
14 December notice is followed by one created on 4 January 2022, about the King opening
the first session. The Assembly's press-release category lists no 2021 month. Its Daily
Summary of Proceedings is empty. Its online Hansard index lists no sitting between the
minutes of 15 September 2021 and those of 13 January 2022. No vote count is imported.
The PMO sentence begins "Prior to being elected" and ends by calling Sovaleni the 18th
Prime Minister. It is recorded as printed; the ordinal is not used to number a succession.

### TO-TR21-03 — Royal appointment effective 27 December 2021

Evidence:

- English release dated 28 December 2021 (`to_sovaleni_royal_appointment_en_20211227`):
  in accordance with Clause 50A, King Tupou VI appointed the Prime Minister-Designate,
  Hon. Siaosi 'Ofakivahafolau Sovaleni, as Prime Minister "with effect from 27 December, 2021".
- Tongan release dated 'Aho 28 'o Tisema 2021 (`to_sovaleni_royal_appointment_to_20211227`):
  the King appointed Sovaleni, printed with "(Hu'akavameiliku)", as Palēmia on the Fale
  Alea's recommendation under kupu 50A. The premiership is reckoned from the day of
  appointment, 27 Tisema 2021. The English rendering is this packet's own reading.

Both release images were viewed at full size and their bytes hashed (see Sources added).

Decision: accepted. One holder observation was added to the existing `to_pm` role:
"Siaosi 'Ofakivahafolau Sovaleni", `from` 2021-12-27, `until` null, citing both releases
and both appointment claims. It cites neither the Assembly selection nor the Cabinet release.
Hu'akavameiliku is recorded as a name form in the claim text and holder note, not as a
dated title grant.

`attested_on` is null. The atlas shows `attested_on` in place of `from`/`until`, so a
value would hide the stated start. With null it reads "2021-12-27 → Not established".
The 2019 Tu'i'onetoa holder stays event-dated (`attested_on` 2019-10-08, `from` null)
because that release reports an appointment event, not a stated effective start.

Limits: 27 December is the effective date, not a signature or presentation date
(TO-TR21-04). No end date or term is recorded. The Tongan "day of appointment" is read
with the English "with effect from" and creates no separate event. When the chiefly title
Hu'akavameiliku was conferred is not researched. The English release's undated remark on
Sovaleni's earlier portfolios is not imported.

### TO-TR21-04 — Signature and presentation of the warrant

What the primary records say: the English release says the King "has appointed" Sovaleni
with effect from 27 December. The Tongan release counts the office from the day of
appointment, 27 Tisema. Neither records a warrant being signed or presented, or a time or
place. The 28 December release date is publication only.

Lead: a secondary news report says the warrant was presented on 28 December (see Leads
not imported). It was not imported.

Sources attempted: PMO site search for "Sovaleni" (result pages 1 and 2); both PMO
appointment releases; the Assembly's latest-news archive (`start=0` to `460`); its
press-release category; its Hansard index for 2021 and 2022; and its Daily Summary of
Proceedings. No online Tonga Government Gazette was located.

Decision: **unresolved**. Recorded in `to_prime_minister` coverage. No date is inferred.

### TO-TR21-05 — End of Tu'i'onetoa's premiership and any caretaker boundary

Evidence available: the 2019 royal appointment (8 October 2019). The court and IPU
records call Tu'i'onetoa Prime Minister on 1 October and 18 November 2021. IPU calls him
the "outgoing Prime Minister" at the 15 December selection. None records a resignation,
an end date, a caretaker instrument or an interim arrangement.

Decision: **unresolved**. The Tu'i'onetoa holder observation is unchanged (`until` null).
Ending his premiership on 26 or 27 December would be inferred from Sovaleni's start, which
the handoff forbids. "Outgoing" is a description at 15 December, not a boundary.

Sources attempted: those listed under TO-TR21-04. The PMO search was for "Sovaleni" and
might not surface a Tu'i'onetoa or caretaker notice. That search is listed as next work.

### TO-TR21-06 — First Sovaleni Cabinet

Evidence: the PMO release dated 29 December 2021 (two page images):

- `to_sovaleni_cabinet_effective_20211228`: on the Prime Minister's recommendations, the
  King appointed the new Ministers of Government with effect from 28 December 2021. The
  numbered list has twelve members. Item 1 is Sovaleni (Prime Minister; Education and
  Training; Police, Fire Services and Emergency Services; His Majesty's Armed Forces).
  Item 2 is Hon. Poasi Mataele Tei (Deputy Prime Minister; MEIDECC; Public Enterprises).
- `to_sovaleni_cabinet_letters_oath_20211229`: the Prime Minister presented the letters
  of appointment to all Cabinet Ministers, and to the Governor of Ha'apai, "today,
  Wednesday 29th December, 2021". The first Cabinet meeting was held that day, and all
  Cabinet Ministers took their Ministerial oath.

Decision: accepted. A holder observation was added to the existing `to_deputy_pm` role:
Poasi Mataele Tei, `from` 2021-12-28, `until` null, citing only the effective-date claim.
Letters and oath remain a separate claim dated 29 December. The existing 2026 Deputy
Prime Minister observation is unchanged.

Limits: the oath sentence follows two sentences dated "today" and has no date of its own.
It is read as part of the 29 December report. Item 1 lists the premiership among
portfolios effective 28 December. That is not a second start date for the premiership,
which the 28 December releases give as 27 December. Items 3–12 are not imported, and the
`to_ministers` holder list stays empty. Item 8, Viliami Uasike Latu, shares a name with the
2026 Deputy Prime Minister, but no identity or succession link is drawn. The Governor of
Ha'apai is not named. The group photograph on image 2 was not used.

## Sources added

| Source ID | What | Retained provenance |
|---|---|---|
| `to_assembly_interim_speaker_20211122` | [Assembly notice](https://parliament.gov.to/en/media-centre/latest-news/appointment-of-interim-speaker-lord-tangi), created 22 Nov 2021 | Direct fetch, HTTP 200, 55,949 bytes twice; **no hash asserted** (per-request form token; live hit counter) |
| `to_assembly_pm_nominations_20211201` | [Assembly notice](https://parliament.gov.to/en/media-centre/latest-news/period-for-nomination-of-candidates-for-prime-minister-now-open), created 1 Dec 2021 | Direct fetch, 55,879 bytes twice; no hash asserted |
| `to_assembly_pm_meeting_20211213` | [Assembly notice](https://parliament.gov.to/en/media-centre/latest-news/meeting-to-elect-the-prime-minister-to-be-held-on-wednesday-15-december), created 13 Dec 2021 | Direct fetch, 55,539 bytes twice; no hash asserted |
| `to_assembly_pm_nominations_20211214` | [Assembly notice](https://parliament.gov.to/en/media-centre/latest-news/two-prime-minister-candidate-nominations-received), created 14 Dec 2021 | Direct fetch, 54,450 bytes twice; no hash asserted |
| `to_pmo_sovaleni_appointment_20211228` | [PMO English release](https://pmo.gov.to/king-appoints-prime-minister-hon-siaosi-ofakivahafolau-sovaleni/), dated 28 Dec 2021 | Release image 228,292 bytes, SHA-256 `877ef23c…cb61f2`, 1275×1650, viewed; page HTML 76,329 bytes (browser), no hash asserted |
| `to_pmo_sovaleni_appointment_to_20211228` | [PMO Tongan release](https://pmo.gov.to/fakanofo-e-he-ene-afio-a-hon-siaosi-ofakivahafolau-sovaleni-ko-e-palemia-o-tonga/), dated 28 Dec 2021 | Release image 208,565 bytes, SHA-256 `28af517a…c92e2`, viewed; page HTML 76,399 bytes, no hash asserted |
| `to_pmo_cabinet_20211229` | [PMO Cabinet release](https://pmo.gov.to/prime-minister-hon-siaosi-ofakivahafolau-sovaleni-announces-new-cabinet-ministers/), dated 29 Dec 2021 | Image 1: 277,933 bytes, `246495c5…fdabde5`; image 2: 297,845 bytes, `e4c96820…af860fc`; both viewed; page HTML 77,161 bytes, no hash asserted |

Each source has a checked-in derived factual extract under [sources/](sources/) (four
`tonga-assembly-*` and three `tonga-pmo-*` files). The extract's own checksum is in the
packet and is distinct from any original-response hash. Original pages and images are
not checked in; no source artwork, coat of arms or photograph is republished.

PMO pages return HTTP 403 to non-browser requests, so their HTML was read in a browser.
The release images download directly (HTTP 200). A re-request during this review
returned the same byte counts, and the recorded SHA-256 values were recomputed from the
local copies. The PMO extracts record the image as the original response identity and
list every release image. For the two-image Cabinet release, the top-level identity is
image 1.

## Leads not imported

- Matangi Tonga, 28 December 2021,
  [king-appoints-pm-and-cabinet](https://matangitonga.to/2021/12/28/king-appoints-pm-and-cabinet).
  It reports that the King presented the Royal Warrant of Appointment at the Royal Palace
  that morning, with office held from 27 December "for four years". It adds that
  warrants were read and returned to the Legislative Assembly that afternoon. This is a
  secondary account: the presentation date, the four-year term and the Assembly reading
  are not imported. It points to Assembly minutes for 28 December as a primary target.
- The PMO release's "18th Prime Minister" ordinal and its undated list of Sovaleni's
  earlier portfolios.
- Cabinet items 3–12 and the Governor of Ha'apai.
- Lord Tangi as an interim-Speaker holder on `to_speaker`.
- The 2021 releases' identification of Tupou VI as King (a possible `to_king` attestation).

## Suggested next work orders

These are proposals for the integrator. They are not created in `work-orders.json`.

- `C01-Tonga-TR21-003`: Tonga Government Gazette for December 2021 (warrant, signature
  date, instrument number). Resolves TO-TR21-04.
- `C01-Tonga-TR21-004`: Assembly minutes or journal for 15 and 28 December 2021
  (declared result; reading of the warrants). Adds the Assembly-side record for
  TO-TR21-02 and may resolve TO-TR21-04.
- `C01-Tonga-TR21-005`: end of Tu'i'onetoa's premiership. Search PMO for "Tu'i'onetoa"
  and "caretaker", and review any constitutional provision on holding office until a
  successor is appointed, by its effective date. Resolves TO-TR21-05.
- `C01-Tonga-TR21-006`: the other eleven 2021 Cabinet members as dated `to_ministers`
  observations from the same release, without inferring terms.
- The 2019, 2024 and 2025 transitions stay under the earlier `C01-Tonga-REC-002` proposal,
  as separate packets.

## Integration notes (outside this packet's file boundary)

- `research-index.json` is regenerated in a **separate commit** on this branch. New
  totals: 68 sources and 1,622 claims (previously 61 and 1,613). Organization,
  institution, packet and batch counts are unchanged. Tonga still has one open batch,
  `C01-Tonga-DISC-B001`, with nine members.
- `research/README.md` and the S10.g paragraph in `C01/README.md` need the new totals and
  a pointer to this report. They are left for the integrator.
- `test_tonga_research_s10g.py` pins exact totals and was updated: 19→26 sources and
  35→44 claims. Its 2019 holder check now pins both `to_pm` dict holders by name and order.
  It previously unpacked exactly one. No assertion was removed or loosened.
- `test_tonga_reconciliation_c01.py` is unchanged and passes. Its check that no holder
  is named "Siaosi Sovaleni" passes because the holder uses the full printed name.
  That check was about the Assembly selection. The new test carries its intent explicitly:
  no selection or procedure claim is cited by any holder.
- The new holders are the packet's first with a non-null `from`. The atlas labels them
  "Reported interval: 2021-12-27 → Not established" and "Reported interval:
  2021-12-28 → Not established". No UI code changed.
- Access note: `pmo.gov.to` refused all connections during CLAUDE-C01-01. It served
  pages to a browser during this review.

## Checks

```text
python -X utf8 tools/avatars/campaign_research.py --check
python -X utf8 -m unittest discover -s tools/avatars -p "test_tonga_*.py"
python -X utf8 -m unittest discover -s tools/avatars -p "test_campaign*.py"
python -X utf8 -m unittest discover -s tools/avatars -p "test_*research*.py"
node --test tools/ui/check_leadership_research_review.cjs
python tools/planning/workboard.py --check
git diff --check
```
