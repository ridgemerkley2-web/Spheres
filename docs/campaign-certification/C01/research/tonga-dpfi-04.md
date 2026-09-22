# Tonga DPFI 04: PTOA identity, leadership and the 2025 record

Packet: **CLAUDE-C01-04**. State: **ready_for_review** (not complete).
Owner: Claude. Integrator/reviewer: Codex. Branch `claude/c01-tonga-04`, base
`b6767837` (the head of `claude/c01-tonga-02`, CLAUDE-C01-02, still awaiting review;
that branch is based on integration `5b46e40a`). Research access: 21 September 2026
(local). The historical cutoff stays **7 September 2026**.

This packet reviews eight observations about the Democratic Party of the Friendly
Islands (`to_dpfi`), printed in Tongan court records as PTOA, in [tonga.json](tonga.json).
It adds eleven sources and 22 claims, a second holder observation on the existing
`to_dpfi_leader` role, one new party role (`to_dpfi_president`) with one holder
observation, and seven name observations. It adds no organization, institution, game
mapping, lifespan, merger, portrait or avatar. The parent scope (C01, C06, S23, WC1 and
CP1) remains open.

The research dossier and an independent check were prepared before this packet was
written. Every checker defect is applied below (see [Checker defects](#checker-defects)).

## Outcome

| ID | Question | Decision |
|---|---|---|
| TO-DPFI-01 | Does a Tongan primary record expand PTOA and tie it to the Democratic Party name? | **Accepted** as name observations: Supreme Court 2022 and Court of Appeal 2023; not a registered name |
| TO-DPFI-02 | Does a primary record attach the PTOA label to 'Akilisi Pohiva's group before 2019? | **Accepted** as a label co-reference in the 2017 campaign; the identification is contextual |
| TO-DPFI-03 | Is the reported September 2010 founding established by a primary record? | **Unresolved:** news-only; `lifecycle.from` stays null |
| TO-DPFI-04 | What is PTOA/DPFI's registered or incorporated form? | **Unresolved:** none found in the records searched; societies are filed on paper |
| TO-DPFI-05 | Another dated DPFI leader observation for Pohiva? | **Accepted:** IPU 2014 leader holder (IPU-only); the 2014 national-office events are three separate claims |
| TO-DPFI-06 | Is Pohiva's death recorded, and does any record give a party succession? | **Accepted:** death at month precision (September 2019); seat events separate; no party succession recorded |
| TO-DPFI-07 | Who held party offices after September 2019? | **Unresolved:** Fatai Helu recited as President on 29 August 2022 (separate role); party leader unknown |
| TO-DPFI-08 | Did PTOA/DPFI win no seats on 20 November 2025? | **Not accepted:** no primary record labels any 2025 candidate or result with a party |

### Date ledger

Each row is a separate dated fact with its own claim. Different dates for an election,
a selection, an endorsement, a Cabinet, a death, a notice, a win and an oath are not
inconsistencies, and none is merged into another.

| Date | Event | Claim or field |
|---|---|---|
| 25 Nov 2010 | IPU: DPFI formed before the election, led by Pohiva (existing) | `to_dpfi_pohiva_2010` |
| 27 Nov 2014 | IPU: DPFI led by Pohiva at the election | `to_ipu_2014_dpfi_pohiva_leader`; leader holder `attested_on` |
| 29 Dec 2014 | Assembly elects Pohiva Prime Minister | `to_ipu_2014_pohiva_assembly_selection_20141229` |
| "the following day" | King Tupou VI's endorsement (of whom is not explicit) | `to_ipu_2014_royal_endorsement_following_day` (no structured date) |
| 19 Jan 2015 | Twelve-member Cabinet takes office | `to_ipu_2014_cabinet_took_office_20150119` |
| from Aug 2017, end not given | Campaign evidence naming "Mr Pohiva and the PTOA party" | `to_ptoa_pohiva_campaign_2017` (period `from` only) |
| 16 Nov 2017 | IPU: DPFI led by the Prime Minister (existing) | `to_ipu_2017_dpfi_continuity` |
| Sep 2019 | 'Akilisi Pohiva's death (month precision) | `to_pohiva_death_month_2019` |
| 19 Nov 2019 | Notice of the Tongatapu 1 by-election, set for 28 November | `to_tongatapu1_byelection_notice_20191119` |
| Nov 2019 | Siaosi Pohiva wins the by-election by 16 votes | `to_siaosi_pohiva_byelection_win_201911` |
| by 11 May 2020 | Siaosi Pohiva sworn in for Tongatapu 1 | `to_siaosi_pohiva_oath_20200511` |
| 18 Nov 2021 | PTOA fields candidates; Piukala stands in Tongatapu 7 | `to_ptoa_2021_candidates_20211118`; `to_ipu_2021_ptoa_expansion` (existing) |
| 29 Aug 2022 | Ruling: Tongan expansion, Democratic Party, President Helu, objective | four `to_ptoa_*_20220829` claims; President holder `attested_on` |
| 6 Apr 2023 | Court of Appeal: "PTOA or the Democratic Party" | `to_ptoa_democratic_party_20230406` |
| 20 Nov 2025 | Poll; returns carry no party labels | `to_tec_2025_*`, `to_ipu_2025_no_party_result`, `to_fasi_v_sika_poll_result_20251120` |
| 18 May 2026 | CV 49/2025 expanded judgment signed (publication, not an event here) | `published_date` of `to_sc_fasi_v_sika_20260518` |
| 21 Jul 2026 | MCCTIL memo on the paper society register | `to_incsoc_paper_register_20260721` |
| unknown | Founding; legal form; Helu's selection and term; party leader after Sep 2019 | TO-DPFI-03, 04, 07 |

`attested_on` follows the packet's existing convention: it is the date of the observed
state or event as the source dates it (for example, the 1 October 2021 Niutao event in
the 2022 judgments). The date a court or page was issued is `published_date`. A claim
the source dates only by month uses a `period`; a claim dated only relatively gets no
structured date.

## Observations

### TO-DPFI-01 — Tongan expansion of PTOA and the Democratic Party name

Evidence:

- Supreme Court, CV 55 of 2022, ruling of 29 August 2022 (`to_ptoa_tongan_name_20220829`):
  paragraph 3 refers to "the PTOA party, also known as the Democratic Party", and
  footnote 3 expands PTOA as **Paati Temokalati 'a e 'Otu Motu 'Anga'ofa**. The page image
  was read, not only the OCR text.
- Court of Appeal, AC 22 of 2022, judgment of 6 April 2023 (`to_ptoa_democratic_party_20230406`):
  paragraph [1] says the challengers are associated with "a political party known as PTOA
  or the Democratic Party".

Decision: accepted as name observations on `to_dpfi`: "PTOA", "Paati Temokalati 'a e
'Otu Motu 'Anga'ofa" and "Democratic Party" (29 August 2022), and "PTOA" and "Democratic
Party" (6 April 2023). The disjunctive phrase is recorded as two names. This adds court
evidence to the TO-REC-02 provisional grouping; `automatic_merge` stays false and the
`identity_reconciliation` observation list (TO-REC-01 to 05) is unchanged.

Limits: neither court uses the English "Democratic Party of the Friendly Islands". That
link still rests only on IPU 2021's "Democratic Party of the Friendly Islands (PTOA)". The
court text is a background description, not a register entry, founding date or legal
form. AC 22 prints "respondents" for the challengers, though its title names them as
appellants, and dates the election 16 November 2021 where CV 55/2022 and IPU give
18 November. Both are kept as printed.

### TO-DPFI-02 — The PTOA label in the 2017 campaign

Evidence: Supreme Court, CV 23 of 2021, judgment of 28 October 2021
(`to_ptoa_pohiva_campaign_2017`). Paragraph 46 records the plaintiff's evidence that, in
the campaign after Parliament's dissolution in August 2017, "Mr Pohiva and the PTOA
party" made statements about him. Paragraphs 42-44 refer to Prime Minister Samuela
'Akilisi Pohiva, and paragraph 47 dates a broadcast of Mr Pohiva's election speech to
7 October 2017. IPU 2017 (`to_ipu_2017_dpfi_continuity`, existing) describes DPFI at the
same election as led by the Prime Minister.

Decision: accepted as a label co-reference. The PTOA label and the DPFI name both attach
to the Pohiva-led group in 2017. This strengthens, but does not replace, the provisional
grouping.

Limits: only the label fragment is quoted. The statements themselves, which the
plaintiff denied, are not imported, nor is anything else in the judgment. Paragraph 46
says only "Mr Pohiva"; reading him as 'Akilisi Pohiva is contextual. The claim's period
starts in August 2017 (month precision; 1 August is a lower bound) and has no end: the
judgment gives none, and the IPU poll date is not borrowed for it. The 7 October speech
does not mention PTOA. No party office for Pohiva comes from this record, and no name
observation is added for 2017.

### TO-DPFI-03 — Founding

Evidence available: IPU 2010 (`to_dpfi_pohiva_2010`, existing) names DPFI among parties
formed before the 25 November 2010 election. A contemporary RNZI report of a launch in
September 2010 is a news lead (see Leads not imported).

Decision: **unresolved**. No founding statute, launch notice or registration instrument
was found. `lifecycle.from` stays null.

### TO-DPFI-04 — Registered or incorporated form

Evidence:

- CV 55/2022 (`to_ptoa_applicants_individuals_20220829`): the proceeding is brought by two
  individuals, Fatai Helu and Paula Piveni Piukala, not in PTOA's name, and the ruling
  does not describe PTOA as incorporated or registered.
- CV 55/2022 (`to_ptoa_self_described_objective_20220829`): the ruling relays Piukala's
  affidavit as saying PTOA's objective is to promote equality before the law. The court
  adds that the party, by its mandate, is concerned with electing people's
  representatives. The objective is labelled self-description.
- MCCTIL consultation memo of 21 July 2026 (`to_incsoc_paper_register_20260721`): the 1988
  Incorporated Societies Act does not support online filing. The online register opened
  in December 2025 takes company, business-name, licence and foreign-investment filings.
  A society is incorporated by a paper application.

Decision: **unresolved**. No primary record found says PTOA/DPFI is an incorporated
society, registered party or other legal person. Unlike the People's Party
(`to_peoples_party_incorporated_society_2022`), no registry testimony was found for PTOA
in the records searched. Neither incorporation nor its absence is inferred.

Records searched: CV 55/2022, AC 22/2022 and CV 23/2021 (text layer and the pages listed
under Sources added); CV 74/2021, CV 71/2021, AC 17/2022, CV 77/2021, AC 12/2022,
CV 14/2023, AC 16/2023, AC 1/2024 and CV 75/2014 (text layer only; OCR gaps are
possible); CV 49/2025 (image-only, all 13 pages read) and CV 54/2025 (image-only, read
by the checker). Many AGO judgments are image-only, and those not listed were not read.
The MCCTIL online register takes no society filings and its entity search sits behind
reCAPTCHA, so it was not used.

### TO-DPFI-05 — Pohiva as DPFI leader in 2014, and the separate national-office events

Evidence: IPU PARLINE, elections in 2014 (`to_ipu_2014`):

- `to_ipu_2014_dpfi_pohiva_leader`: DPFI, "led by the veteran pro-democracy MP 'Akilisi
  Pohiva", took eight of 17 directly elected seats on 27 November 2014; two elected
  independents later joined it. IPU calls Pohiva the DPFI leader during the campaign.
- `to_ipu_2014_pohiva_assembly_selection_20141229`: on 29 December 2014 the new Assembly
  elected Pohiva Prime Minister.
- `to_ipu_2014_royal_endorsement_following_day`: King Tupou VI's endorsement came "the
  following day".
- `to_ipu_2014_cabinet_took_office_20150119`: the twelve-member Cabinet (five DPFI members,
  one noble, six independents) took office on 19 January 2015.

Decision: accepted. A holder observation is added to `to_dpfi_leader`: "'Akilisi
Pohiva", `attested_on` 2014-11-27, `from` and `until` null. The 2010 holder stays first
and unchanged. The name observation "Democratic Party of the Friendly Islands (DPFI)"
(27 November 2014) fills the gap between the 2010 and 2017 observations. The three
national-office events are separate claims on `to_pm` and `to_prime_minister`, and the
Cabinet claim is also on `to_cabinet`.

Limits: both leader observations are **IPU-only**: interparliamentary summaries citing
Legislative Assembly communications, not Assembly, registry or court records. They are
attestations at elections, not a continuous 2010-2019 term. The endorsement has no
structured date: 30 December 2014 would be derived from relative wording. The sentence
joins the endorsement to the Cabinet's taking office, so whether the King endorsed Pohiva
or the Cabinet is not explicit. No royal instrument was seen, and no `to_pm` holder,
start or end is created for Pohiva. IPU's 2014 party counts (two contesting, one winning)
differ in approach from its later pages; the eight seats are IPU's statement, not a party
total used here.

### TO-DPFI-06 — Death, vacancy, by-election and oath

Evidence: two Legislative Assembly articles.

- Created 19 November 2019 (`to_assembly_byelection_20191119`):
  `to_pohiva_death_month_2019` says 'Akilisi Pohiva, former MP and late Prime Minister,
  "passed away in September this year". `to_tongatapu1_byelection_notice_20191119` says
  the Tongatapu 1 by-election to fill his seat is set for Thursday 28 November, with two
  candidates, Siaosi Pohiva and Dr. Netatua Pelesikoti Taufatofua.
- Created 11 May 2020 (`to_assembly_siaosi_oath_20200511`):
  `to_siaosi_pohiva_byelection_win_201911` says Siaosi Pohiva won the by-election in
  November 2019, 16 votes ahead of "Dr. Netatua Prescott", replacing his late father.
  `to_siaosi_pohiva_oath_20200511` says he has been sworn in for Tongatapu 1.

Decision: accepted as four separate claims: a death (September 2019), a notice
(19 November 2019), a win (November 2019) and an oath (by 11 May 2020). The death is on
`to_dpfi`; the seat events are on `to_legislative_assembly`. No holder is created.

Limits: the death day is not given (12 September 2019 appears only in news). The
scheduled 28 November poll day is not substituted for the win's month, because the
notice predates the poll. The oath day is not given. The opponent's name is printed two
ways and is not reconciled. Neither article records a party-leadership transfer. A seat
succession is not a party succession, and the death is not used as the end of any
office term.

### TO-DPFI-07 — Party offices after September 2019

Evidence: CV 55/2022, paragraph 3 (`to_ptoa_president_helu_20220829`): "Fatai Helu is the
President of the party. Paula Piukala is an executive member." The 2021 candidates
claim (`to_ptoa_2021_candidates_20211118`) records that PTOA fielded candidates in a number
of constituencies, including Piukala in Tongatapu 7.

Decision: **unresolved** overall. A new role `to_dpfi_president` ("President of the party
(PTOA)", kind `other`) holds one observation: Fatai Helu, `attested_on` 2022-08-29, `from`
and `until` null. It is kept separate from `to_dpfi_leader`, as TO-REC-08 kept the People's
Party's Leader and President apart, because no constitution shows whether they are one
office. The party leader after September 2019 remains unknown.

Limits: the President statement is a background recital in a leave ruling. Nothing about
the party's officers was in issue, and no basis is given. It was probably drawn from the
applicants' own filings (Piukala was an applicant), so it carries the weight of party
self-description relayed by the court, not of a finding. Helu's selection date, term and
successor are unknown. "Executive member" is not a named office and is not imported as a
holder. The rival leadership claims of Semisi Sika and Siaosi Pohiva, and 2021 factions,
appear only in news and tertiary leads. IPU 2021's "one of its leaders" for Siaosi Pohiva
remains a non-title. No party office for 2023 to the cutoff was found in the records
searched (listed under TO-DPFI-04).

### TO-DPFI-08 — The 2025 result

Evidence:

- IPU 2025 (`to_ipu_2025_no_party_result`): the Candidates field reads "Not applicable.
  There is no party system or candidates stood as independents." The party-seat fields
  also read "Not applicable", and the narrative names no party.
- Electoral Commission results (`to_tec_2025_no_party_column`): the constituency tables
  have no party or affiliation column. Its candidate list
  (`to_tec_2025_candidate_list_no_party`) gives constituency, candidate and village only.
- Personal poll results (`to_tec_2025_tongatapu1`, `to_tec_2025_tongatapu2`): Tongatapu 1,
  Tevita Fatafehi Puloka first (1,343), Siaosi Vailahi Pohiva second (758); Tongatapu 2,
  Semisi Kioa Lafu Sika first (951), 'Uhilamoelangi Fasi second (764).
- CV 49 of 2025, page 1 (`to_fasi_v_sika_poll_result_20251120`): Fasi and Sika were
  candidates on 20 November 2025, and Sika "was the successful candidate".

Decision: **not accepted**. No primary record found links any 2025 candidate or result to
PTOA/DPFI. The "no seats" statement appears only in commentary and tertiary sources. No
dissolution or inactivity is inferred.

Limits: the primary records agree that Sika won the 20 November 2025 **poll** in
Tongatapu 2, so a tertiary statement that both rival claimants "lost" in 2025 is wrong for
the poll. His seat status after 6 May 2026 is governed by CV 49/2025, whose determination,
findings and allegations are not imported. This packet does not describe Sika as holding
the seat at the cutoff, and no people's-representative holder is created. No source here
identifies the 2019-2020 Siaosi Pohiva with the 2025 candidate Siaosi Vailahi Pohiva, and
no identity is asserted. Personal results carry no party banner, so the IPU, results and
candidate-list claims support only that the 2025 returns record no party. The personal
results and CV 49/2025 are cited on `to_legislative_assembly`, not on `to_dpfi`.

## Sources added

| Source ID | What | Retained provenance |
|---|---|---|
| `to_sc_helu_piukala_v_ec_20220829` | [CV 55/2022 ruling](https://ago.gov.to/cms/judgements/supreme-court-civil/category/206-cv-2022.html?download=2491:fatai-helu-and-paula-piveni-piukala-v-electoral-commission-the-election-of-lord-nuku-cv-55-22-tosc-29-aug-2022-whitten-qc-cj-ruling), 29 Aug 2022 | 348,680 bytes, SHA-256 `a5f84dcf…8681c9`, re-download matched; pages 1, 3, 10 viewed |
| `to_ca_helu_piukala_v_ec_20230406` | [AC 22/2022 judgment](https://ago.gov.to/cms/judgements/court-of-appeal/category/223-ac-2023.html?download=2575:1-fatai-helu-2-paula-piveni-piukala-v-1-the-electoral-commission-2-lord-nuku-ac-22-22-toca-6-apr-2023-randerson-j-white-j-morrison-j-judgment-of-the-court), 6 Apr 2023 | 246,435 bytes, `4148871d…bdfa65`, matched; page 1 viewed |
| `to_sc_tuivakano_v_police_20211028` | [CV 23/2021 judgment](https://ago.gov.to/cms/judgements/supreme-court-civil/category/188-cv-2021.html?download=2294:lord-tuivakano-v-1-police-commissioner-2-attorney-general-3-kingdom-of-tonga-cv-23-21-tosc-28-oct-2021-whitten-qc-cj-judgement), 28 Oct 2021 | 1,288,307 bytes, `af4f8fe0…593c08`, matched; pages 1, 11 viewed |
| `to_ipu_2014` | [IPU PARLINE 2014](https://data.ipu.org/election-summary/HTML/2317_14.htm) | Browser fetch; raw 15,787 bytes, **no raw hash asserted** (per-request Cloudflare script); normalised 14,849 bytes, `c3b08924…0620d7`, with the rule in the extract |
| `to_assembly_byelection_20191119` | [Assembly article](https://parliament.gov.to/en/media-centre/latest-news/tongatapu-1-constituency-to-elect-new-mp-next-thursday), created 19 Nov 2019 | Direct fetch, 54,685 bytes; no hash asserted (form token, hit counter); both hosts serve it |
| `to_assembly_siaosi_oath_20200511` | [Assembly article](https://parliament.gov.to/en/media-centre/latest-news/siaosi-pohiva-sworn-in-as-tongatapu-1-mp), created 11 May 2020 | Direct fetch, 54,069 bytes; no hash asserted; both hosts serve it |
| `to_mcctil_incsoc_consultation_20260721` | [MCCTIL consultation memo](https://www.businessregistries.gov.to/Documentation/TO/Consultation_Memo_Proposed_New_Incorporated_Societies_Act_July_21_2026.pdf), 21 Jul 2026 | 985,683 bytes, `d70f8174…cb757`, matched; page 3 viewed |
| `to_ipu_2025` | [IPU Parline 2025](https://data.ipu.org/parliament/TO/TO-LC01/election/TO-LC01-E20251120/) | Browser fetch, 164,916 bytes every time; no hash asserted (per-request markup) |
| `to_tec_results_2025` | [Electoral Commission results](https://elections.gov.to/wp-content/uploads/2025/11/Tonga-General-Elections-Results-revised-20251.pdf), undated | 8,171,403 bytes, `fe27cd3c…381f88`, matched; image-only, pages 1-2 read, 3-17 as thumbnails |
| `to_tec_home_2025` | [Electoral Commission home page](https://elections.gov.to/), undated | 410,934 bytes, `dcbbc7ea…678677`, matched on re-download (live page) |
| `to_sc_fasi_v_sika_20260518` | [CV 49/2025 judgment and reasons](https://ago.gov.to/cms/judgements/supreme-court-civil/category/247-cv-2026.html?download=3028:uhilamoelangi-fasi-v-semis-kioa-lafu-sika-cv-49-2025-tosc-6-may-2026-j-garlick-kc-judgement-and-reasons), signed 18 May 2026 | 5,984,803 bytes, `8b8a29bc…8f2d9`, matched; image-only; page 1 imported |

Each source has a checked-in derived factual extract under [sources/](sources/) (four
`tonga-sc-*`/`tonga-ca-*` court files, two `tonga-ipu-*`, two `tonga-assembly-*`, two
`tonga-tec-*` and one `tonga-mcctil-*`). Every extract repeats the packet's claims
exactly, and its own checksum is in the packet, separate from the original-response
hash. Original PDFs, pages and renders are not checked in; no seal, coat of arms,
signature or photograph is republished.

Every source carries a `source_type`: `primary_court_record` (the four AGO judgments),
`interparliamentary_election_record` (IPU), `primary_legislature_news_notice` (Assembly),
`primary_electoral_commission_record` (Electoral Commission) and
`primary_registry_consultation_notice` (MCCTIL).

The IPU 2014 hash was re-derived for this packet in its own browser tab. Two raw fetches
gave 15,787 bytes with different hashes. Removing the single injected
`<script>(function(){function c(){…})();</script>` element (938 bytes, before `</body>`)
left 14,849 bytes with the recorded SHA-256 both times. The Assembly articles were
re-fetched from both hosts: `parliament.gov.to` gave 55,119 and 54,475 bytes and
`www.parliament.gov.to` 54,681 and 54,061 bytes, all with HTTP 200 and no redirect. The
original sizes are closest to the `www` host's, and the pages' share links use it. The
packet keeps the `parliament.gov.to` form used by its other Assembly sources.

## Leads not imported

- RNZI via ACE Project, 6 September 2010,
  [another-new-political-party-emerges-in-tonga-as](https://aceproject.org/regions-en/countries-and-territories/TO/news/another-new-political-party-emerges-in-tonga-as):
  a launch in September 2010 (TO-DPFI-03). Contemporary news.
- Wikipedia, [Democratic Party of the Friendly Islands](https://en.wikipedia.org/wiki/Democratic_Party_of_the_Friendly_Islands):
  founded September 2010; no seats in 2025. Tertiary.
- Wikipedia, [Sēmisi Sika](https://en.wikipedia.org/wiki/S%C4%93misi_Sika) and related
  articles, relayed by search summaries: Sika named successor in September 2019, rival
  2021 factions, both claimants losing in 2025. Tertiary; the last point is wrong for the
  2025 poll in Tongatapu 2 (TO-DPFI-08).
- Tonga Independent, 24 November 2025,
  [commentary on PTOA's 2025 result](https://tongaindependent.com/commentaryvoters-reject-ptoa-as-a-new-political-era-emerges/):
  the "no seats" statement. News commentary.
- Kaniva Tonga, November 2021,
  [PTOA after the 2021 losses](https://kanivatonga.co.nz/2021/11/a-sobering-reality-hits-ptoa-after-election-losses-voters-elect-nine-new-independent-faces/);
  December 2018, [Pohiva on a successor](https://www.kanivatonga.co.nz/2018/12/pohiva-says-he-has-successor-in-mind-to-take-over-as-democracy-party-leader/);
  and a Tongan-language article on a 2014 candidate-list dispute
  ([kanivatonga.co.nz](https://kanivatonga.co.nz/ongoongo-faka-tonga/fakamaalaala-isileli-pulu-uhinga-fekihiaki-lisi-kanititeiti-e-paati/),
  read only through a retrieval summary). News.
- Matangi Tonga, 25 April 2020,
  [party rivalry after 'Akilisi's era](https://matangitonga.to/2020/04/25/party-rivalry-marks-end-akilisis-era):
  post-2019 leadership. News; the body was not read.
- ABC News, 12 September 2019,
  [Pohiva's death](https://www.abc.net.au/news/2019-09-12/tongas-prime-minister-akilisi-pohiva-dies/11504638):
  the exact day. News; a PMO or Gazette record is needed.
- International IDEA, August 2015,
  [Tonga in a new political order](https://www.idea.int/sites/default/files/publications/tonga-in-a-new-political-order.pdf).
  Secondary; not read.
- Supreme Court CV 74 of 2021, Piukala v Saulala, 2 May 2022
  ([AGO](https://ago.gov.to/cms/judgements/supreme-court-civil/category/206-cv-2022.html?download=2380:paula-piveni-piukala-v-sione-sangster-saulala-cv-74-21-tosc-2-may-2022-whitten-qc-cj-judgement);
  1,186,757 bytes, `0cf6b608…be2c2`, re-download matched). A primary record, not imported:
  its PTOA mentions (a campaign team, Piukala as PTOA candidate, a poster) sit inside
  disputed evidence and add nothing beyond CV 55/2022 paragraph 3.
- CV 49/2025: its allegations, findings and determination. Only its page 1 poll statement
  is imported.
- Supreme Court CV 54 of 2025 (Finau v Tangimana, Ongo Niua 17), downloaded by the checker
  (`check/cv54_2025.pdf`, 6,166,874 bytes, `054501c9…506fd5c`, image-only). No reference to
  PTOA, DPFI or any party; nothing imported.
- Legislative Assembly [Fun Facts](https://parliament.gov.to/en/about-parliament/fun-facts):
  Pohiva became Prime Minister on 30 December 2014. Undated live page; a `to_pm` lead
  only, which would also conflict with keeping the endorsement date relative.
- MCCTIL [Nonprofit Sector help page](https://www.businessregistries.gov.to/public/help.aspx?cn=NonprofitSector)
  (23,688 bytes; its hash is not reproducible because ASP.NET view-state values change on
  every request). A primary page, excluded: it is undated and was read after the cutoff,
  its "not yet in the online register" reading would be inferred from its wording about
  eventual inclusion, and the dated 21 July 2026 memo already supports paper-only filing.

## Sources attempted

- National Library of New Zealand catalogue search for "Democratic Party of the Friendly
  Islands" ([natlib.govt.nz/records/search](https://natlib.govt.nz/records/search?text=%22Democratic+Party+of+the+Friendly+Islands%22)):
  an Incapsula block to curl and in a browser tab. Bot protection was not bypassed.
- Tonga Business Registries entity search
  ([corp/search.aspx](https://www.businessregistries.gov.to/corp/search.aspx)): behind
  reCAPTCHA; not used. It does not hold societies in any case.
- Prime Minister's Office ([pmo.gov.to](https://pmo.gov.to/)), for a September 2019 death
  statement: HTTP 403 to curl; not tried in a browser.
- IPU Parline by curl: HTTP 403; read in a browser instead. The new Parline pages for 2010
  and 2014 give only summary fields and link to the legacy summaries, which were read.
- US State Department 2022 human rights report on Tonga: HTTP 403 through web retrieval.
  It would be a lead only.
- Legislative Assembly site search for PTOA, Democrats, Fatai Helu, opposition leader and
  Temokalati: no relevant results. Member profiles, for example Semisi Kioa Lafu Sika's,
  carry no party label.
- AGO judgment text layers with no PTOA, DPFI or Temokalati reference: CV 71/2021 and
  AC 17/2022 (Siaosi Pohiva v Puloka), CV 77/2021, AC 12/2022 (Sika v Fasi), CV 14/2023,
  AC 16/2023, AC 1/2024 and CV 75/2014. The AGO 2025 and 2026 category listings appeared
  incomplete through pagination.

## Checker defects

| # | Defect | Outcome |
|---|---|---|
| D1 | TO-DPFI-08 left out CV 49/2025 and overstated the conflict with the tertiary "both lost" claim | **Applied**: CV 49/2025 page 1 added as poll-result corroboration only; the conflict is limited to the poll; its determination is not imported and no seat status at the cutoff is asserted |
| D2 | "PTOA or the Democratic Party" recorded as one name | **Applied**: two name observations, "PTOA" and "Democratic Party" (6 Apr 2023); "PTOA" for 29 Aug 2022 is also recorded |
| D3 | 2017 claim paraphrased the evidence, borrowed an end date and treated the identification as stated | **Applied**: only the label fragment is quoted and the content is described neutrally; period `from` 2017-08-01, `through` null (the validator rejects a month-only "2017-08"); the identification is marked contextual |
| D4 | IPU 2014 hash not reproducible from a raw fetch | **Applied**: raw hash null; normalised bytes, hash and rule recorded; re-derived in this packet's own browser tab |
| D5 | IPU 2014 national-office events collapsed into one date | **Applied**: three claims; the endorsement has no structured date; the sentence's ambiguity about whom the King endorsed is also recorded |
| D6 | Objective claim dated to the filing day | **Applied**: `attested_on` 2022-08-29 and the ID renamed. The checker's "filed on or before 17 August" is not used either: the ruling says only that the 17 August application is supported by the affidavit |
| D7 | MCCTIL nonprofit page given an inferred period and an inferred "not yet included" | **Applied by exclusion**: the page is not imported (see Leads not imported); the dated memo covers the point |
| D8 | MCCTIL nonprofit page hash not reproducible and undisclosed | **Applied by exclusion**: the page is not imported, and the non-reproducibility is disclosed with the lead |
| D9 | IPU 2025 Candidates field misquoted ("and" for "or") | **Applied**: quoted exactly |
| D10 | Helu's office presented as a court attestation without caveat | **Applied**: the claim and holder say it is a background recital, probably from the applicants' filings, carrying the weight of self-description; `from` and `until` null |
| D11 | Absence statements stronger than the search | **Applied**: reworded as "none found in the records searched", with the list under TO-DPFI-04 and in the packet coverage |
| D12 | Sources lacked `source_type`; leader observation rests on IPU only | **Applied**: every source has a `source_type`; the leader claim, holder and TO-DPFI-05 say IPU-only |
| D13 | Mixed meaning of `attested_on` within one source | **Applied**: the convention is stated (event date as the source dates it; issue date in `published_date`), and the 2021 candidates claim ID now carries its event date |
| D14 | No 2014 name observation | **Applied**: "Democratic Party of the Friendly Islands (DPFI)", 27 Nov 2014 |
| D15 | Assembly URLs record the bare host while the copies suggest `www` | **Applied as disclosure**: both hosts were re-fetched and serve the article; the sizes and share links are recorded in each extract; the packet keeps its existing URL form |

The checker also noted that the IPU 2017 and 2021 pages behind the existing claims were
not reopened; they were not reopened for this packet either, and those claims are unchanged.

## Suggested next work orders

These are proposals for the integrator. They are not created in `work-orders.json`.

- `C01-Tonga-DPFI-002`: a paper search of the MCCTIL Incorporated Societies register for
  PTOA, Paati Temokalati 'a e 'Otu Motu 'Anga'ofa and the English name (society number,
  registration date, rules). Resolves TO-DPFI-04, if a record exists.
- `C01-Tonga-DPFI-003`: the PTOA constitution or rules (labelled self-description),
  defining President and Leader and their terms. Would settle whether `to_dpfi_president`
  and `to_dpfi_leader` are one office.
- `C01-Tonga-DPFI-004`: the exact day of 'Akilisi Pohiva's death (PMO release or Gazette);
  the Speaker's vacancy notice or Hansard entry; the Electoral Commission's official
  28 November 2019 by-election result (margin and the opponent's name).
- `C01-Tonga-DPFI-005`: party leadership after September 2019 and Helu's selection, from
  a primary record or labelled self-description. The Matangi Tonga lead may point to one.
- `C01-Tonga-DPFI-006`: any 2025 PTOA candidate list (self-description), and a
  transcription of the Commission's results pages 3-17 without inferring a banner.
  Tongatapu 7 includes Paula Piveni Piukala.
- `C01-Tonga-DPFI-007`: the 2014 royal instrument for Pohiva's premiership (Gazette), for
  `to_pm`, and any appeal from CV 49/2025, for Assembly membership work.
- A founding record for September 2010, if a launch statement or Kalonikali notice exists.

## Integration notes (outside this packet's file boundary)

- **Stacked:** this branch is based on `claude/c01-tonga-02` at `b6767837`. Merge
  CLAUDE-C01-02 first; the diff of this branch against `b6767837` is CLAUDE-C01-04 only.
- `research-index.json` is regenerated in a **separate commit**. New totals: 79 sources and
  1,644 claims (previously 68 and 1,622, including CLAUDE-C01-02). Tonga role observations
  go from 14 to 15. Organization, institution, packet and batch counts are unchanged, and
  Tonga keeps one open batch, `C01-Tonga-DISC-B001`, with nine members.
- `research/README.md`, the C01 README totals and `docs/planning/ai-workstreams.json`
  (which lists handoffs) are left for the integrator. This handoff is self-proposed and not
  yet registered there.
- `test_tonga_research_s10g.py` pins exact totals and was updated: 26 to 37 sources and
  44 to 66 claims. `test_tonga_reconciliation_c01.py` pinned the DPFI leader holder list
  as exactly the 2010 claim; it now pins exactly two entries (the 2010 claim first, then
  the 2014 holder with `from` and `until` null). No assertion was removed or loosened.
- In the atlas, `to_dpfi` shows two offices: "Party leader" (the 2010 claim and "'Akilisi
  Pohiva · Observed on 2014-11-27") and "President of the party (PTOA)" ("Fatai Helu ·
  Observed on 2022-08-29"). No UI code changed; the Node checks pass, and no browser review
  was run.
- CV 49/2025 records findings against a named, sitting representative. Only its page 1
  poll statement is imported. Reviewers should keep it that way.

## Checks

```text
python -X utf8 tools/avatars/campaign_research.py
python -X utf8 tools/avatars/campaign_research.py --check
python -X utf8 -m unittest discover -s tools/avatars -p "test_*research*.py"
python -X utf8 -m unittest discover -s tools/avatars -p "test_tonga_*.py"
python -X utf8 -m unittest discover -s tools/avatars -p "test_campaign*.py"
python -X utf8 -m unittest discover -s tools/avatars -p "test_tonga_dpfi_c01_04.py"
node --test tools/ui/check_leadership_research_review.cjs
python tools/planning/workboard.py --check
git diff --check
```

All passed on 21 September 2026: the exact index regeneration check; 79 research tests,
38 Tonga tests (10 of them new) and 16 campaign tests; 11 atlas Node tests; the workboard
check (44 markers). The new test was also run against four hand-made regressions (a
derived endorsement date, an end date on Helu's office, an inferred Pohiva premiership
and the IPU misquote); it failed on each.
