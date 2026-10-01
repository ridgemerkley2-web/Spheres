# BSP, AAP and NPP national leaders 48: three party offices, 1990-2026

Packet: **CLAUDE-C01-48**. State: **ready_for_review** (not complete).
Owner: Claude. Integrator/reviewer: Codex. Branch `claude/c01-in-48`; claimed at `e2623e3e` on base `5ea4f8fc`
(`codex/campaign-certification`, unmoved when the packet was committed); not stacked on a pending packet. Research
access: 1 October 2026 (UTC); every recorded response was first downloaded that day between 11:39Z and 11:57Z, which is
each source's `accessed_date`, and again at least 30 minutes later (see the identities section). The historical cutoff
stays **7 September 2026**.

This packet adds to [india.json](india.json) one party role on each of three existing recognition observations (rows 1, 2
and 6 of the Election Commission's national-party table of 23 March 2024):

- `in_aap_national_convenor` ("National Convenor of the Aam Aadmi Party") on `in_eci_20240323_np_01`;
- `in_bsp_national_president` ("National President of the Bahujan Samaj Party") on `in_eci_20240323_np_02`;
- `in_npp_national_president` ("National President of the National People's Party") on `in_eci_20240323_np_06`.

All three are kind `party_leader`. The packet adds 19 sources and 24 claims, all about these offices, four holder
observations (one with a stated start), three role scope notes, three observation coverage notes and one note in the
packet coverage. Each observation's identity, recognition row, `unresearched` lifecycle, `reporting_identity_only`
coverage status and empty game mapping are unchanged, and its existing source and claim stay first in its lists. Nothing
that CLAUDE-C01-11, -15, -20, -27, -33 or -40 added is changed and no earlier extract is edited. The party offices and the
`in_prime_minister` and `in_presidency` institutions stay separate both ways, and no Chief Ministership is recorded. The
parent scope (C01, C06, S23, WC1 and CP1) remains open.

## Outcome

| ID | Question | Decision |
|---|---|---|
| AAP-NC-01 | Arvind Kejriwal, first National Convenor (2012-) | **Accepted in part:** observed on 7 October 2013 (the party's NRI release of that date: 'Arvind Kejriwal is AAP’s National Convener'); claim: the party's letter of 29 May 2015 to the Election Commission recalling the first election of office bearers on 25.11.2012 with him as National Convenor |
| AAP-NC-02 | Arvind Kejriwal, term from 27 April 2016 | **Accepted:** from 27 April 2016, the effective day in the National Executive's certified minutes ('W.E.F. 27.04.2016') filed with the Commission on 31 May 2016; later attestations 12 November 2016, 20 January 2017 (Commission catalogue), 18 December 2022, 18 November 2024 and 25 August 2026 are claims |
| BSP-NP-01 | Kanshi Ram, founder (proposed 1990-2003) | **Declined:** no party or official record reviewed attests him in the office on any day from 1 January 1990; claims: the founding recalled for 14 April 1984 and his death recalled as 'the founder President BSP' on 9 October 2006 |
| BSP-NP-02 | Mayawati (2003-) | **Accepted in part:** observed on 16 July 2012 (the party's letter to the Chief Election Commissioner, on her letterhead as National President and signed by her, with the list of office bearers); claims: the successor designation of 15 December 2001, the retrospective selection and assumption of 18 September 2003, the re-election of 27 August 2006, undated styling, the Commission catalogue of 15 April 2019 and the party's press note of 1 September 2021 |
| NPP-NP-01 | Purno Agitok Sangma, founder (2013-2016) | **Declined:** no party or official record reviewed attests him in the office on a stated day; claim: 'Founder President (1947-2016)' on the party's undated leadership page |
| NPP-NP-02 | Conrad K. Sangma (2016-) | **Accepted in part:** observed on 11 July 2020 (the party's press release: the conclave held 'today' 'was attended by the National President Conrad K. Sangma'); claims: elected 'in March 2016' (no day), the retrospective 'onus to lead the party', and his signed notifications as National President of 4 February 2022 and 17 April 2025 |

The resulting holder observations:

| Role | Holder | `attested_on` | `from` | `until` | Basis |
|---|---|---|---|---|---|
| `in_aap_national_convenor` | Arvind Kejriwal | 2013-10-07 | null | null | The party's NRI release datelined 7 October 2013, present-tense styling (raw capture of nri.aamaadmiparty.org) |
| `in_aap_national_convenor` | Arvind Kejriwal | null | 2016-04-27 | null | Certified excerpt of the National Executive's minutes of 27 April 2016, 'W.E.F. 27.04.2016', in the party's letter of 31 May 2016 to the Commission (raw capture of eci.nic.in) |
| `in_bsp_national_president` | Mayawati | 2012-07-16 | null | null | The party's letter of 16-7-2012 to the Chief Election Commissioner and the enclosed list (the Commission's compilation PPS2_01032013, pages 13-14) |
| `in_npp_national_president` | Conrad K. Sangma | 2020-07-11 | null | null | The party's press release of 11 July 2020 (raw capture of nppindia.in) |

### How a start and an end are decided

An effective `from` or `until` requires a source for the actual boundary. The only one is the AAP National Executive's
certified minutes, which state the day the election took effect ('W.E.F. 27.04.2016'); this packet treats that party
instrument like an appointment instrument that states its effective day (see Decisions for Codex). Every other holder is
a dated `attested_on` observation from the earliest same-day in-office record in the party's own papers that names the
holder and styles the office. Recalled elections and selections, retrospective timelines (including the BSP profile's
'Assumed the office' entry for 18 September 2003), stylings, founding and successor recollections, the founder's death
and Election Commission catalogue entries never supply a boundary, and a later term never supplies an earlier term's end.
Catalogue entries and later records are kept as `in_office_continuation_attestation` claims.

### Date ledger

Each row is a separate dated fact with its own claim. "Observed" marks a holder's `attested_on` claim and "start" its
`from` claim; every other claim never feeds a holder.

| Date | Events | Claims |
|---|---|---|
| 14 Apr 1984 | BSP founded by Kanshi Ram (recalled) | `in_bsp_site_kanshi_ram_founded_bsp_19840414` (claim) |
| 15 Dec 2001 | Kanshi Ram declares Mayawati, 'then the lone Vice-President', his heir and successor (recalled) | `in_bsp_site_mayawati_declared_successor_20011215` (claim) |
| 18 Sep 2003 | Party makes Mayawati National President; she 'Assumed the office' (both recalled) | `in_bsp_site_party_made_mayawati_national_president_20030918`, `in_bsp_profile_mayawati_assumed_national_president_20030918` (claims) |
| 27 Aug 2006 | Mayawati re-elected National President (recalled) | `in_bsp_profile_mayawati_reelected_national_president_20060827` (claim) |
| 9 Oct 2006 | Death of 'the founder President BSP', Kanshi Ram (recalled 3 July 2009) | `in_bsp_release_founder_president_kanshi_ram_died_20061009` (claim) |
| 16 Jul 2012 | Mayawati: in office (signed letter and list) | `in_bsp_letter_mayawati_national_president_20120716` (observed) |
| 25 Nov 2012 | AAP's first election of office bearers, Kejriwal National Convenor (recalled 29 May 2015) | `in_aap_letter_first_office_bearer_election_20121125` (claim) |
| 7 Oct 2013 | Kejriwal: in office (NRI release) | `in_aap_nri_kejriwal_national_convener_20131007` (observed) |
| 27 Apr 2016 | Kejriwal elected National Convener with effect from that day | `in_aap_ne_minutes_kejriwal_convener_wef_20160427` (start) |
| 12 Nov 2016 | Kejriwal: in office again (press release) | `in_aap_release_kejriwal_national_convenor_20161112` (claim) |
| 20 Jan 2017 | Commission's order to Kejriwal 'National Convener' (catalogue) | `in_eci_catalogue_kejriwal_national_convener_20170120` (claim) |
| 15 Apr 2019 | Commission's order to Mayawati 'National President' (catalogue) | `in_eci_catalogue_mayawati_national_president_20190415` (claim) |
| 11 Jul 2020 | Conrad K. Sangma: in office (press release) | `in_npp_release_conclave_attended_by_national_president_20200711` (observed) |
| 1 Sep 2021 | Mayawati: in office again (press note) | `in_bsp_press_note_mayawati_national_president_20210901` (claim) |
| 4 Feb 2022 | Conrad K. Sangma: in office again (signed notification) | `in_npp_notification_conrad_sangma_national_president_20220204` (claim) |
| 18 Dec 2022 | Kejriwal: 'Our party national convenor' (post) | `in_aap_post_national_convenor_address_20221218` (claim) |
| 18 Nov 2024 | Kejriwal: in office again (post) | `in_aap_post_kejriwal_national_convenor_approved_20241118` (claim) |
| 17 Apr 2025 | Conrad K. Sangma: in office again (signed notification) | `in_npp_notification_national_committee_conrad_sangma_20250417` (claim) |
| 25 Aug 2026 | Kejriwal: in office again (news post), the last record before the cutoff | `in_aap_news_kejriwal_national_convenor_townhall_20260825` (claim) |
| undated | Mayawati 'At present, National President'; Purno Agitok Sangma 'Founder President (1947-2016)'; Conrad K. Sangma 'onus to lead the party' (March 2016) and elected 'in March 2016' | four claims |

## Observations

### AAP-NC-01 — Arvind Kejriwal, the first term

The party's letter of 29 May 2015 to the Election Commission says the first election of office bearers was held on a
day typed 24.11.2012 and altered by hand to 25; its annexure prints 25.11.2012 and lists him first as National Convenor
(claim, never a start). The earliest dated party record adopted is the NRI release of 7 October 2013, which says 'Arvind
Kejriwal is AAP’s National Convener'. No record states the day this term ended.

### AAP-NC-02 — Arvind Kejriwal, the term from 27 April 2016

The certified excerpt of the National Executive's minutes of 27 April 2016 records his election as National Convener
'W.E.F. 27.04.2016', the packet's only stated start. Later records style him National Convenor (12 November 2016,
18 December 2022, 18 November 2024, 25 August 2026) and the Commission's catalogue names him National Convener (20 January
2017); all are claims. His re-election reported on 10 September 2026 is after the cutoff.

### BSP-NP-01 — Kanshi Ram

The party's 2009 'About The BSP' page recalls that he founded the party on 14 April 1984 and does not call him President.
A party press release of 3 July 2009 recalls the death of 'the founder President BSP' on 9 October 2006. No record
reviewed attests him in the office on any day from 1 January 1990, and the party's pages recall Mayawati's selection on
18 September 2003, so the death is not treated as a death in office. Claims only.

### BSP-NP-02 — Mayawati

The party's 2009 pages recall her designation as heir on 15 December 2001, her selection and assumption on 18 September
2003 and her re-election on 27 August 2006 (claims). The holder observation is her signed letter of 16-7-2012 to the
Chief Election Commissioner on her letterhead as National President, enclosing the list of office bearers. The
Commission's catalogue (15 April 2019) and the party's press note of 1 September 2021 are later claims.

### NPP-NP-01 — Purno Agitok Sangma

The party's undated leadership page heads him 'Founder President (1947-2016)'. No dated party or official record
reviewed attests him in the office, and none states the day of his death with the office. Claims only.

### NPP-NP-02 — Conrad K. Sangma

The party's pages say he was elected National President 'in March 2016' (no day). The holder observation is the press
release of 11 July 2020. His signed notifications of 4 February 2022 and 17 April 2025 (the latter appointing the
National Committee for 'the tenure 2025–2028') are later claims.

## Sources added

AAP (8): the NRI release of 7 October 2013; the party's letters to the Commission of 29 May 2015 and 31 May 2016 (from
the Commission's organisational-election papers on eci.nic.in); the press release of 12 November 2016; the Commission's
catalogue entry for its order of 20 January 2017 (eci.gov.in file 1260); and three WordPress REST records:
archive.aamaadmiparty.org post 773305 (18 December 2022), aamaadmiparty.org post 883198 (18 November 2024) and
aamaadmiparty.org aap_news 904246 (25 August 2026).

BSP (6): the party website's 'About The BSP' page and President's profile (captured 28 and 31 March 2009); its press
release page with the release of 3 July 2009; its letter of 16-7-2012 in the Commission's compilation PPS2_01032013; the
Commission's catalogue entry for its orders of 15 April 2019 (eci.gov.in file 9927); and the press note of 1 September
2021 (bspindia.org/download/pr/).

NPP (5): the party website's 'Leadership & Key People' page (captured 16 November 2019); the press release of 11 July
2020; the 'Conrad Sangma' page (captured 25 February 2021); and the notifications of 4 February 2022 and 17 April 2025
(nppindia.in/wp-content/uploads/).

Each source has one derived extract under [sources/](sources/), named `india-<publisher>-<topic>-<date>-facts.json`.

## Response identities and stability checks

Sixteen sources are raw Internet Archive captures (`id_`) made before the cutoff, used because the live addresses are
gone or blocked: bspindia.org did not resolve, nppindia.in now serves a rebuilt site without these pages, the old
aamaadmiparty.org pages no longer exist, eci.nic.in no longer resolves and eci.gov.in answers curl with HTTP 406. Three
are WordPress REST records from the AAP's own hosts, because the HTML pages of its sites changed between two downloads a
few seconds apart. Every response was downloaded with curl's default User-Agent, without `--compressed` and with no
Accept-Encoding header; none was served with a Content-Encoding.

| Source | Bytes | SHA-256 (first 16) | First / last download (UTC) |
|---|---|---|---|
| `in_aap_nri_release_kejriwal_interact_20131007` | 60785 | `2ee8b421727527d8` | 11:53:24 / 12:28:13 |
| `in_aap_letter_to_eci_org_election_20150529` | 1891611 | `40498164efd55348` | 11:51:33 / 12:28:18 |
| `in_aap_letter_to_eci_new_national_executive_20160531` | 623833 | `708dfdf9cdba85e1` | 11:51:37 / 12:28:22 |
| `in_aap_release_kejriwal_demonetisation_20161112` | 101927 | `75e16c576352f221` | 11:41:04 / 12:28:27 |
| `in_eci_catalogue_order_to_kejriwal_20170120` | 107889 | `8ea555e65c4fcf96` | 11:45:20 / 12:28:31 |
| `in_aap_post_national_council_meeting_20221218` | 76109 | `2c7152d087d15131` | 11:48:09 / 12:28:35 |
| `in_aap_post_shokeen_minister_20241118` | 6125 | `a624a138035190ce` | 11:48:11 / 12:28:40 |
| `in_aap_news_e20_townhall_goa_20260825` | 11586 | `2ec5726cef7e55b6` | 11:48:40 / 12:28:46 |
| `in_bsp_site_about_bsp_2009` | 25135 | `ed631909958c1c7b` | 11:39:26 / 12:28:50 |
| `in_bsp_site_mayawati_profile_2009` | 32452 | `28f7c63518384497` | 11:39:10 / 12:28:54 |
| `in_bsp_press_release_cochin_20090703` | 28694 | `f3a4c8848b2447e8` | 11:39:31 / 12:28:58 |
| `in_bsp_letter_to_cec_office_bearers_20120716` | 2639930 | `a21314ff14a3ad96` | 11:54:22 / 12:29:01 |
| `in_eci_catalogue_order_to_mayawati_20190415` | 97766 | `49bde93e58b6ce4c` | 11:53:27 / 12:29:05 |
| `in_bsp_press_note_lucknow_20210901` | 84146 | `cc7bb6a8f3cdb8c4` | 11:39:56 / 12:29:09 |
| `in_npp_site_leadership_page_2019` | 85879 | `b8fb32fc200fc2f0` | 11:40:19 / 12:29:13 |
| `in_npp_press_release_resolution_20200711` | 117420 | `83af81106ef0b7b2` | 11:40:44 / 12:29:16 |
| `in_npp_site_conrad_sangma_page_2021` | 124635 | `49746d13db3a6318` | 11:40:14 / 12:29:20 |
| `in_npp_notification_manipur_candidates_20220204` | 1071596 | `41996d12d265283d` | 11:56:52 / 12:29:24 |
| `in_npp_notification_national_committee_20250417` | 404254 | `8422b24f623a3785` | 11:56:41 / 12:29:28 |

Each pair matched in byte count and SHA-256; `packet_check.py` downloads every source again.

## Leads not imported

- `aamaadmiparty.org/wp-json/wp/v2/aap_news/905580` (published 10 September 2026): Kejriwal 'unanimously re-elected'
  National Convenor at the 15th National Council meeting 'held on Wednesday' — after the cutoff.
- eci.gov.in file `10347-recognition-of-national-peoples-party` (order of 7 June 2019): the only Internet Archive
  capture of the PDF is truncated at 1,048,576 bytes and unreadable; the catalogue page names no officer.
- eci.gov.in file `7842-national-peoples-party`: also truncated at 1 MiB; its readable first page is an expenditure
  statement signed by a state General Secretary, not the national office.
- bspindia.org `bsp-news-till-sep2009.php`: a news page carrying an item datelined 20 August 2009 that styles Mayawati
  'Bahujan Samaj Party National President and U.P. Chief Minister'; it mixes unattributed government and agency text, so
  it is left as a lead rather than party evidence.
- eci.nic.in `Organisaional_Election_of_the_parties.pdf` (24 MB, 2012) may hold earlier BSP organisational-election
  filings; not downloaded in this pass.
- News reports of Conrad K. Sangma's re-election on 15 March 2025 (Assam Tribune, ANI) and of Kejriwal's 2021
  re-election (Tribune, Deccan Herald): leads only.
- bspindia.org `kumari-mayawati.php` exists in a 2020 version (`/our-president/`) with the same timeline; not added.

## Sources attempted

- eci.gov.in (current site and its `eci-backend` download API): HTTP 406 to curl's default User-Agent; not worked around.
- bspindia.org and loksabha.nic.in: did not resolve on 1 October 2026.
- nppindia.in REST API: the site is now a Next.js application with no WordPress API; archived pages used instead.
- archive.aamaadmiparty.org REST: holds posts only from 2016-2024 and few for 2020-2022; no record of the 2019 or 2021
  organisational elections was found.
- eci.gov.in files 5259 and 11583 (BSP organisational elections 2014, 2014-2019 revised and 2019-2024), 5266 (AAP
  2012-2015 and 2016-2019), 9891 (notice to Ms Mayawati, 11 April 2019) and 1260/9927 (the orders themselves): no PDF
  capture in the Internet Archive.
- eci.nic.in `Bahujan Samaj Party as on 30.10.2014.pdf`: only a redirect capture.

## Suggested next work orders

1. Find a contemporary record attesting Kanshi Ram as BSP President between 1990 and 2003 (Lok Sabha debates of 1991-1998
   in his own words, or Election Commission correspondence) and the party's records of 18 September 2003.
2. Recover the BSP's filings of 2014 and 2019-2024 and the AAP's 2019 and 2021 organisational elections from another
   archive or by request to the Commission.
3. Find a dated NPP or Election Commission record attesting Purno Agitok Sangma as National President (2013-2016) and one
   stating the day of his death with the office.
4. Download and review eci.nic.in `Organisaional_Election_of_the_parties.pdf` (2012) for BSP filings before 2012.

## Decisions for Codex

1. **AAP 'W.E.F.' start.** This packet gives Arvind Kejriwal `from` 2016-04-27 from the party's certified minutes
   ('ELECTED ... AS OFFICE BEARER OF THE PARTY W.E.F. 27.04.2016'), treating a party instrument that states its effective
   day like the appointment instruments of C01-36. The alternative is `attested_on` 2016-04-27 only.
2. **Second AAP holder entry.** The 2016 term is a separate holder entry for the same person, as CLAUDE-C01-27 does for
   repeated BJP presidents; it gives no end to the 2013 observation.
3. **BSP retrospective assumption.** The party's own profile lists '18 September, 2003 : Assumed the office of Bahujan
   Samaj Party's National President'; it is kept as a claim (retrospective list) rather than a start.
4. **Kanshi Ram's death.** The party's 2009 release calls him 'the founder President BSP' and states the day of death; it
   is not treated as a death in office because no record attests him in office and the party's pages place the
   presidency with Mayawati from 2003.
5. **Election Commission catalogue entries** that name the office (orders of 20 January 2017 and 15 April 2019) are kept
   as continuation claims, not holder evidence, because the orders themselves were not read.
6. **ECI-hosted party letters.** The AAP letters of 2015 and 2016 and the BSP letter of 2012, published by the Commission,
   are treated as the parties' own records (`primary_party_record_archived_pdf`); receipt stamps are claims.
7. **NPP observation choice.** The press release of 11 July 2020 (filed under the site's Press Release category) is the
   selected observation; the signed notification of 4 February 2022 is the alternative if a web press release is not
   accepted as holder evidence.
8. **AAP NRI release.** The release of 7 October 2013 on nri.aamaadmiparty.org was issued by the party's volunteers in the
   USA and Canada on the party's own host; it is treated as a party record.
9. **Legacy-font press note.** The BSP press note of 1 September 2021 was read by transliterating its Kruti Dev text
   layer; no page was rendered.

## Integration notes (outside this packet's file boundary)

- `docs/campaign-certification/C01/research-index.json` is regenerated in its own commit (`Regenerate the C01 research
  index for CLAUDE-C01-48`); on a conflict with parallel C01 packets, continue existing claims first and regenerate the
  index from the merged inputs rather than hand-merging it.
- The pinned tests of CLAUDE-C01-11, -15, -20, -27, -33 and -40 and `test_india_research_s10e.py` were re-expressed for
  three more party roles (totals, source order, the party-leader list, coverage-note positions, hosts and access dates);
  none was loosened. A later India packet that appends sources or coverage notes will need the same re-expression.
- `test_certified_gap_ledger.py` reports no pinned attribution for this packet until Codex classifies its commit, and
  `test_certified_boundary_matrix.py` needs `spheres-web/src`, which the sparse checkout lacks.

## Checks

Run in the worktree on 1 October 2026 before committing (base `5ea4f8fc`):

- `python -X utf8 tools/avatars/campaign_research.py` (index regenerated) and `--check`: pass.
- `python -X utf8 tools/avatars/campaign_census.py --check`: pass (it also passed on the unchanged base).
- `python -X utf8 -m unittest discover -s tools/avatars -p 'test_india*.py'`: 84 tests, OK (the new
  `test_india_bsp_aap_npp_leaders_c01_48.py` included; all 19 of its mutations are rejected).
- `-p 'test_*research*.py'`: 79 tests, OK; `-p 'test_campaign*.py'`: 16 tests, OK.
- `node --test tools/ui/check_leadership_research_review.cjs`: 11 pass, 0 fail.
- `python tools/planning/workboard.py --check`: exits 1 with 'Missing task handoff:
  docs/campaign-certification/S26/preparation/RECRUITMENT.md'. The file exists at HEAD but lies outside this worktree's
  sparse checkout, so the check fails identically on the base here; not caused or fixed by this packet.
- `git diff --check`: clean.
- `D:/spheres-scratch/c01-pipeline/tools/packet_check.py 48` is run after the two commits; its summary is returned with
  the handoff.
- Known failures outside the suite: `test_certified_gap_ledger.py` (no pinned attribution for this packet until Codex
  classifies its commit) and `test_certified_boundary_matrix.py` (needs `spheres-web/src`, absent from the sparse
  checkout).
