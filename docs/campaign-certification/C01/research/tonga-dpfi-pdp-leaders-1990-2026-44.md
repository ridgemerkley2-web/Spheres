# Tonga opposition parties 44: DPFI/PTOA and PDP leaders, 1990-2026

Packet: **CLAUDE-C01-44**. State: **ready_for_review** (not complete).
Owner: Claude. Integrator/reviewer: Codex. Branch `claude/c01-to-44`; claim commit `fdb71175` on base `509bd289` (then the
head of `codex/campaign-certification`); not stacked on a pending packet. Research access: 30 September 2026 (local; 1 October
UTC), when every recorded response was downloaded at least twice for this packet. The historical cutoff stays **7 September
2026**.

This packet reviews the party offices of the two opposition parties already in [tonga.json](tonga.json) from each party's
founding to the cutoff: `to_dpfi_leader` and `to_dpfi_president` (Democratic Party of the Friendly Islands, `to_dpfi`) and
`to_pdp_leader` (People's Democratic Party, `to_pdp`). It adds 5 sources and 6 claims, all on `to_dpfi` and all claims only:
two retrospective founding statements, a Supreme Court recital of a "Tonga Democratic Party" leader, the Court of Appeal's
description of the "Friendly Islands Democratic Party" as an unincorporated body, and a Prime Minister's Office statement
that PTOA was unregistered. It adds coverage notes on `to_dpfi`, `to_pdp` and the packet. It adds **no holder**, no
organization, institution, role or name observation, and changes no existing source, claim, extract, holder or coverage
entry. The existing holders ('Akilisi Pohiva as leader, attested 27 November 2014, plus the string observation of 2010;
Fatai Helu as President, attested 29 August 2022; the PDP string observation) are unchanged. The parent scope (C01, C06,
S23, WC1 and CP1) remains open.

No additional primary record found by this packet names a later DPFI or PTOA leader, president or chair, or any PDP
officer. The existing primary 2022 Fatai Helu presidency observation remains unchanged. Other proposed names (a
2019-2020 successor, rival 2021 groups, a 2025 chair, the PDP's first president) come only from news or tertiary leads,
listed below.

## Outcome

| ID | Question | Decision |
|---|---|---|
| TO-OPP-01 | When was DPFI founded, and does a founding record name its first leader? | **Unresolved:** the Assembly relays third-party text dating the founding to September 2010 and its member page gives 2010; both retrospective, neither names an office; `lifecycle.from` stays null |
| TO-OPP-02 | 'Akilisi Pohiva as DPFI leader, 2010-2019 | **Claims only:** a Supreme Court recital of 17 Jan 2014 calls him "the leader of the Tonga Democratic Party"; the name differs from DPFI, so no holder (ruling (a): stays a claim, routed to `C01-Tonga-OPP-005`); existing observations unchanged |
| TO-OPP-03 | DPFI/PTOA legal form | **Claims only:** the Court of Appeal (16 Sep 2015) calls the Friendly Islands Democratic Party an unincorporated body; the PMO (17 Mar 2020) says PTOA had not registered and had no legal body or constitution; TO-DPFI-04 stays unresolved |
| TO-OPP-04 | Leader after Pohiva's death in September 2019, and the 2020-2021 groupings | **Unresolved:** no primary record names a leader, interim or acting leader; news leads only |
| TO-OPP-05 | President or chair after 2022 | **Unresolved:** beyond the existing Helu recital, a 2025 chair appears only in news |
| TO-OPP-06 | PDP founding (April 2005) and its first president | **Unresolved:** no PDP or government record found; IPU's existing month-level observation stays a string observation |
| TO-OPP-07 | PDP after 2008 | **Unresolved:** no later officer, dissolution or inactivity record found |

No holder is added to any of the three roles:

| Role | Holder | `attested_on` | `from` | `until` | Basis |
|---|---|---|---|---|---|
| `to_dpfi_leader` | `to_dpfi_pohiva_2010` (string, existing) | | | | IPU 2010, unchanged |
| `to_dpfi_leader` | 'Akilisi Pohiva (existing) | 2014-11-27 | null | null | IPU 2014, unchanged |
| `to_dpfi_president` | Fatai Helu (existing) | 2022-08-29 | null | null | CV 55/2022 recital, unchanged |
| `to_pdp_leader` | `to_pdp_split_fuko` (string, existing) | | | | IPU 2008, unchanged |

### Rulings (decided by Ridge)

Recorded in the checker-fix round as decided by Ridge; Codex may still decide otherwise.

- **(a)** The Supreme Court recital in Pohiva v Tu'ivakano ("He is the leader of the Tonga Democratic Party") stays a
  claim. The record does not name the organization as DPFI or PTOA, or by any name already observed for `to_dpfi`, so
  attaching it to `to_dpfi_leader` would settle an identity question. It is routed to `C01-Tonga-OPP-005`.
- **(b)** No name observation is added for "Tonga Democratic Party", "Friendly Island Democratic Party (FIDP)" or
  "Friendly Islands Democratic Party"; the names are routed to the Tonga reconciliation work order (`C01-Tonga-OPP-005`
  below).
- **(c)** The new `source_type` `legislature_republished_reference_text` is accepted: non-primary, claims only, never
  holder or organization evidence.

### Date ledger

Each row is a separate dated fact with its own claim. A founding statement, a court recital, a description of legal form
and a government statement are never merged, and none dates an office, a lifecycle boundary or a holder.

| Date | Event | Claim or field |
|---|---|---|
| September 2010 (month; retrospective, relayed) | Pohiva "established the Democratic Party of the Friendly Islands" with other HRDM representatives | `to_la_pga_dpfi_established_sept2010` (period only; claims only) |
| 2010 (year; retrospective) | Assembly member page: "Established the Democratic Party of the Friendly Islands in 2010." | `to_la_profile_dpfi_established_2010` (no structured date) |
| 14 Dec 2013 | Assembly news item relaying the PGA text (its publication date) | source `published_date` only |
| 17 Jan 2014 | Supreme Court: "He is the leader of the Tonga Democratic Party." | `to_tlr_pohiva_leader_tonga_democratic_party_20140117` (claim, not holder) |
| 16 Sep 2015 | Court of Appeal: claim brought "on behalf of an unincorporated body, the Friendly Islands Democratic Party" | `to_ca_fidp_unincorporated_body_20150916` |
| 16 Sep 2015 | Court of Appeal: Tongasat's Supreme Court argument that "the party had no funds" | `to_ca_fidp_party_funds_argument_20150916` |
| 17 Mar 2020 | PMO: PTOA "te'eki ke lesisita" (not yet registered); no legal body or constitution | `to_pmo_ptoa_unregistered_statement_20200317` |

## Observations

### TO-OPP-01 — DPFI founding, 2010

Evidence:

- Assembly news item of 14 December 2013 (`to_la_pga_dpfi_established_sept2010`): on Pohiva's Defender of Democracy award it
  quotes the Parliamentarians for Global Action (PGA) forum website, which says that in September 2010 he established the
  Democratic Party of the Friendly Islands with other Human Rights and Democracy Movement People's Representatives to contest
  the 2010 elections. The item names the PGA website as its source, so the text is typed
  `legislature_republished_reference_text`: claims only, never holder evidence (ruling (c) accepts the type as non-primary,
  never holder or organization evidence).
- Assembly member page, captured 4 March 2016 (`to_la_profile_dpfi_established_2010`): "Established the Democratic Party of
  the Friendly Islands in 2010."

Decision: **unresolved**. Both are retrospective, month- or year-level and name no office; establishing the party is not
read as holding the leader's office. No founding statute, launch notice or registration was found, so `lifecycle.from`
stays null (as TO-DPFI-03).

### TO-OPP-02 — 'Akilisi Pohiva as leader, 2010-2019

Evidence: Pohiva v Tu'ivakano & ors (Supreme Court, Scott CJ, AM 20/2013, 17 January 2014), printed in [2014] Tonga LR 9,
paragraph [4] (`to_tlr_pohiva_leader_tonga_democratic_party_20140117`): "He is the leader of the Tonga Democratic Party."

Decision: **claims only**. The recital is dated and present-tense, like the 2022 Helu recital that C01-04 used as a holder,
but it names a "Tonga Democratic Party", a name not otherwise observed for `to_dpfi`. Joining the names would resolve an
identity, which this packet does not do, so the recital is not a `to_dpfi_leader` holder and no name observation is added
(rulings (a) and (b), decided by Ridge: it stays a claim, routed to `C01-Tonga-OPP-005`). The existing observations of
2010 (string) and 27 November 2014 are unchanged; no continuous 2010-2019 term is inferred, and no start, end or death in
office is recorded for the party office.

### TO-OPP-03 — Legal form, 2015 and 2020

Evidence:

- Court of Appeal, AC 9 of 2015, judgment of 16 September 2015 (`to_ca_fidp_unincorporated_body_20150916`,
  `to_ca_fidp_party_funds_argument_20150916`): Mr. Pohiva brought the Tongasat grant-aid claim "on behalf of an
  unincorporated body, the Friendly Islands Democratic Party", accepting personal liability for costs (paragraph [4]);
  Tongasat had argued in the Supreme Court that "the party had no funds" (paragraph [34]). Both paragraphs were read on the
  page images.
- PMO release of 17 March 2020 in Tongan (`to_pmo_ptoa_unregistered_statement_20200317`), replying to a PTOA press release:
  the PTOA party had not yet registered as the law requires and had no legal body ("sino fakalao") or constitution.

Decision: **claims only**. Neither is a registry record: the court describes how a claim was brought, and the PMO makes a
political assertion. TO-DPFI-04 stays unresolved. "Friendly Islands Democratic Party" differs in word order from DPFI and is
not mapped (ruling (b): no name observation; routed to `C01-Tonga-OPP-005`); suing for the party is not an office. The
PMO release names no PTOA officer.

### TO-OPP-04 — Leadership after September 2019

No party record, PMO release, Assembly record, Gazette notice or judgment found names a leader, interim leader or acting
leader of DPFI or PTOA after Pohiva's death. News leads name Semisi Sika as leader in April and October 2020 and describe
rival 2021 candidate groups led by Sika and Siaosi Pohiva. Decision: **unresolved**; no death in the party office is
recorded, because no primary record names the party office together with the day.

### TO-OPP-05 — President or chair after 2022

The existing Helu observation (29 August 2022, TO-DPFI-07) is unchanged. A 2025 news report calls Teisa Pohiva-Cokanasiga
the party's chair; no primary record was found. Decision: **unresolved**.

### TO-OPP-06 — PDP founding and first president, 2005

The existing IPU observation (an April 2005 split under Tesina Fuko's leadership) stays the only record. News and tertiary
leads give a founding on 8 April 2005, Tesina Fuko's election as President at a meeting at 'Atenisi on 15 April 2005 and a
registration on 1 July 2005; none is imported. No PDP record, Assembly page, judgment or Tonga Law Report names a PDP
officer. Decision: **unresolved**; `to_pdp_leader` keeps its string observation and no dict holder is added.

### TO-OPP-07 — PDP after 2008

No later PDP officer, dissolution or inactive period was found in the Assembly's 2011 member pages, the Tonga Law Reports
2005-2020 or the Attorney General's judgment listings. Decision: **unresolved**.

## Sources added

| Source ID | What | Retained provenance |
|---|---|---|
| `to_la_news_pga_award_20131214` | Assembly news item "'Akilisi Pohiva presented 2013 Defender of Democracy awards" (created 14 Dec 2013), relaying the PGA forum website (archived) | 35,057 bytes, `bcaa0769…1918918`; capture 2016-10-27 |
| `to_tlr_2014_pohiva_v_tuivakano` | Tonga Law Reports 2014 volume (AGO download `1552:2014_tlr`): Pohiva v Tu'ivakano & ors, AM 20/2013, 17 Jan 2014 | 1,510,292 bytes, `b302db6f…542916e`; live AGO PDF |
| `to_ca_psa_pohiva_v_kot_20150916` | Court of Appeal, AC 9 of 2015, judgment of 16 Sep 2015 (AGO download `1176`) | 2,668,960 bytes, `53307c4e…314554b`; live AGO PDF |
| `to_la_profile_pohiva_2016` | Assembly member page, Hon. Samiuela 'Akilisi Pohiva (archived) | 33,380 bytes, `84404948…29399b1`; capture 2016-03-04 |
| `to_pmo_20200317_ptoa_reply` | PMO release in Tongan replying to a PTOA press release, 17 Mar 2020 (archived) | 74,793 bytes, `08258419…2998d2d`; capture 2020-07-03 |

Extracts: `sources/tonga-assembly-pga-award-news-20131214-facts.json`, `tonga-tlr-2014-pohiva-leader-20140117-facts.json`,
`tonga-ca-ac9-20150916-facts.json`, `tonga-assembly-profile-pohiva-2016-facts.json` and
`tonga-pmo-ptoa-reply-20200317-facts.json`. Rows carry `observation_id` `to_dpfi`, `role_id` null and a `role_title` saying
the row is not placed on a role, as the Brazil PFL extracts do. `holder_name` is null on every row except the 2014
recital's ("Mr Pohiva"): the other sources name no holder of any office.

## Identities and stability checks

Every recorded response was downloaded three times for this packet: a research download, then two logged rounds at least
30 minutes apart, all matching in byte count and SHA-256 (times in each extract's `stability_check`):

- research downloads: 2026-10-01T02:25:26Z to 2026-10-01T02:48:39Z (UTC);
- first round: 2026-10-01T02:36:44Z to 2026-10-01T02:50:00Z;
- second round: 2026-10-01T03:13:21Z to 2026-10-01T03:22:01Z, 31 to 36 minutes after each source's first round.

- Internet Archive captures were fetched raw (`id_`) with `Accept-Encoding: identity` and curl's default user agent; none
  was returned gzip-encoded, so every identity is the body as received. The archive answered HTTP 429/503/504 at times
  under the parallel load; requests backed off (45 s steps) and were retried, never routed around.
- The two AGO PDFs are stored files from the download manager, served 200 with no redirect, not generated per request;
  the TLR 2006 volume recorded by an earlier packet has kept its identity since 21 September.
- No source is a search page, listing or live page. Discovery used archive CDX listings and AGO category listings, which
  are not recorded as sources.
- Tongan text is quoted as printed with straight apostrophes for the glottal stop; English renderings are this packet's
  reading.

## Leads not imported

- Matangi Tonga, 19 April 2005,
  [Tonga's new People's Democratic Party elects officials](https://matangitonga.to/2005/04/19/tongas-new-peoples-democratic-party-elects-officials):
  Tesina Fuko elected President at a meeting at 'Atenisi on Friday 15 April 2005. News.
- Wikipedia, [People's Democratic Party (Tonga)](https://en.wikipedia.org/wiki/People's_Democratic_Party_(Tonga)): founded 8
  April 2005, first president 15 April 2005, registered 1 July 2005. Tertiary.
- Pacific Islands Report, 23 January 2007, a commentary by Lopeti Senituli
  ([archived](https://web.archive.org/web/20080516135652/http://archives.pireport.org/archive/2007/January/01-23-comm2.htm)):
  names Father Seluini Akauola, Teisina Fuko and Semisi Tapueluelu among "key leaders" of the PDP. Commentary.
- RNZ, [New political group in Tonga breaks away from Democracy Movement](https://www.rnz.co.nz/international/pacific-news/154456/new-political-group-in-tonga-breaks-away-from-democracy-movement)
  (2005; not read). News.
- Kaniva Tonga, December 2018,
  [Pohiva on a successor](https://www.kanivatonga.co.nz/2018/12/pohiva-says-he-has-successor-in-mind-to-take-over-as-democracy-party-leader/),
  and October 2020,
  [a party statement and a "PTOA Party Leader"](https://kanivatonga.co.nz/2020/10/no-one-registered-ptoa-namewe-are-free-to-do-what-we-do-says-movement-leader-after-party-warns-followers-against-inducing-voters/)
  (Semisi Sika). News.
- RNZ, 23 April 2020, [Reports of a rift in Tonga's Democratic Party](https://www.rnz.co.nz/international/pacific-news/414934/reports-of-a-rift-in-tonga-s-democratic-party):
  "the current PTOA leader, Semisi Sika". News.
- Matangi Tonga, 25 April 2020, [party rivalry after 'Akilisi's era](https://matangitonga.to/2020/04/25/party-rivalry-marks-end-akilisis-era):
  body behind a subscription wall, not bypassed.
- Devpolicy, November 2021 ([1](https://devpolicy.org/tonga-election-2021-missing-the-akilisi-pohiva-election-factor-20211116/),
  [2](https://devpolicy.org/all-change-in-tonga-20211119/)): rival 2021 groups led by Sika and Siaosi Pohiva. Commentary.
- Matangi Tonga, 14 August 2025, [PTOA petitions King](https://matangitonga.to/2025/08/14/ptoa-petitions-king-withhold-signature-his-majestys-diplomatic-services-act),
  and Talanoa 'o Tonga the same day ([talanoaotonga.to](https://talanoaotonga.to/diplomatic-overhaul-sparks-ptoa-petition/)):
  Teisa Pohiva-Cokanasiga, "chair" of the party (PMN and Scoop carry the same story). News; the Matangi body is behind a
  subscription wall.
- Assembly member pages captured 3 February 2011:
  [161-sitiveni-halapua](https://web.archive.org/web/20110203112146id_/http://www.parliament.gov.to:80/about-parliament/members/people/161-sitiveni-halapua.html)
  ("a deputy leader of the Democratic Party of the Friendly Islands") and 162-semisi-kioa-lafu-sika ("a member"). Both
  pages say their text was taken from Wikipedia, so they are tertiary text republished on an official page; a deputy-leader
  role is also outside this packet's roles.
- Assembly news item of 16 September 2011,
  [215-friendly-island-democratic-party-candidate-falisi-tupou-wins-tongatapu-9-by-election](https://web.archive.org/web/20161027074916id_/http://www.parliament.gov.to/media-centre/latest-news/latest-news-in-english/215-friendly-island-democratic-party-candidate-falisi-tupou-wins-tongatapu-9-by-election):
  a candidate label ("FIDP"), no office.
- Supreme Court, CV 49/14 ruling of 6 March 2015 (AGO download 1073): Pohiva as "Leader of the Opposition" in the Assembly,
  a parliamentary role, not a party office.

## Sources attempted

- Party sites: `ptoa.org` returns an error page; `ptoa.to`, `temokalati.to`, `dpfi.to` and `pdp.to` do not resolve;
  `dpfi.org` belongs to an unrelated US foundation. No party website, constitution or statement was found.
- IDCPC (International Department of the CPC, `idcpc.org.cn`) site search for Tonga items: only a December 2018 visit (Sika as Deputy Prime
  Minister, a state office) and a 2023 announcement; no party office.
- Tonga Law Reports (Tonga LR) 2005-2020 (all AGO volumes, text layers): only the two passages used (TO-OPP-02, TO-OPP-03 reprint);
  no PDP, PTOA or Temokalati reference.
- AGO judgment listings 2005-2026 (Supreme Court civil and criminal, Court of Appeal), filtered by party and member names,
  and read where relevant: CV 74/21 and CV 73/21 (2022; PTOA supporters only), CV 48/14 rulings of 19 March 2015 (downloads
  1123, 1215) and of 2018 (1620, 1635), CV 49/14 (1073), CV 64/14 (1128), CV 75/14 (995), AC 10/12 (1063), AC 14/15 (1172,
  Kele'a), AC 4/16 (1366), AC 31/15 (1247), AC 15/18 (1721), CV 14/23 (2606), AC 16/23 (2706) and AC 15/08 (625): no party
  office. The CV 48/14 ruling of 17 August 2018 (download=1611) is a 49-page scan without a text layer: pages 1-3 were read;
  its standing analysis was not reviewed. Three 2008 links (556, 557, 580) returned small non-PDF bodies.
- Assembly member pages (16 People's Representatives, 2011; Pohiva, Halapua and Uata, 2016; Pohiva, 2020; Sika, Siaosi
  Pohiva and Tapueluelu, 2021): no party office.
- Archive listings of the Assembly's English news (2011-2026) and the PMO's posts (2019-2021), filtered by party terms: the
  two items imported and items 293 and 295 (2014 Prime Minister nominations; no party label). A listing of the Assembly's
  pre-2009 pages failed repeatedly (HTTP 503/504/429) and was not pursued; the PMO's uploads for September 2019 hold no
  release file.
- IPU legacy summaries for 2008 and 2010: no PDP officer beyond the existing observation.
- Commonwealth Observer Group interim statement (thecommonwealth.org): no party content; no final report was found.
- National Library of New Zealand (natlib) catalogue for the PDP constitution: Incapsula-blocked (C01-04); not retried.
- Matangi Tonga bodies behind a subscription wall: not bypassed.

## Suggested next work orders

These are proposals for the integrator. They are not created in `work-orders.json`.

- `C01-Tonga-OPP-002`: the PDP constitution and rules in MS-Papers-12389-05 (National Library of New Zealand) and any
  Incorporated Societies register entry for the PDP (registered July 2005 per leads), read in person or by a library request.
- `C01-Tonga-OPP-003`: PTOA or DPFI statements signed by an officer (party newspaper, press releases, Facebook-only notices
  that a reviewer can read without logging in), 2010-2025.
- `C01-Tonga-OPP-004`: the standing analysis of the scanned CV 48/14 ruling of 17 August 2018 (download=1611), which may
  describe the party on whose behalf Pohiva sued.
- `C01-Tonga-OPP-005`: a ruling on whether "Tonga Democratic Party" (2014), "Friendly Island Democratic Party" (FIDP;
  Assembly news item of 16 September 2011, singular 'Island' as printed) and "Friendly Islands Democratic Party" (Court
  of Appeal AC 9/2015) name `to_dpfi`. Rulings (a) and (b) route the 2014 recital and these names here.

## Integration notes (outside this packet's file boundary)

- **Base and stack:** claim commit `fdb71175` on base `509bd289`, the head of `codex/campaign-certification` when the packet was
  claimed; not stacked on a pending packet. `codex/campaign-certification` moved to `efc88b85` during the research and was
  merged into the branch before the packet commits (merge `73ae1261`; no conflict, as it touched no file of this packet), so
  the packet diff is measured from `efc88b85`. The merged C06 Tonga cast check (`test_country_cast.py`) passes with this
  packet's `tonga.json`.
- **Batch order:** the user chose to start this research batch (C01-42 to C01-46) before Codex's roadmap line
  "continue existing claims first" was acted on; this packet is one of that batch.
- `research-index.json` is regenerated in a **separate commit**. New totals: 1,976 sources and 4,882 claims across 9 country packets (previously
  1,971 and 4,876 on `efc88b85`), 844 organization and 36 institution observations, 93 open discovery batches; Tonga keeps
  nine entries and now has 325 claims. `research-index.json` is the only file shared
  with the other packets running in parallel (C01-38 to 41 in fixes, C01-42 to 46 in research), so expect an index-only
  conflict and regenerate it on integration.
- `research/README.md`, the C01 README totals and `docs/planning/ai-workstreams.json` are left for the integrator; this handoff
  is self-proposed and not registered there.
- Pinned tests, none loosened and no assertion removed:
  - `test_tonga_research_s10g.py`: totals 212 sources and 325 claims (from 207 and 319), with the ownership comment.
  - `test_tonga_dpfi_c01_04.py`, `test_tonga_deputy_prime_ministers_c01_36.py` and the other `test_tonga_*_c01_*.py` tests
    need no change: no holder, role, name observation or position they pin moves.
- Existing text is not edited: no existing source, claim, extract, holder or coverage entry changes; the new claims and
  sources are appended to `to_dpfi`, and coverage notes are appended to `to_dpfi`, `to_pdp` and the packet.
- New `source_type` value: `legislature_republished_reference_text`, the official-site counterpart of
  `party_republished_reference_text` (third-party text relayed on an Assembly page; claims only). Ruling (c), decided by
  Ridge, accepts it: non-primary, claims only, never holder or organization evidence. New `event_kind` values in
  the extracts: `retrospective_founding_statement`, `court_recital_of_party_office`, `court_description_of_legal_form` and
  `government_statement_on_legal_form`.
- **Census:** `tools/avatars/campaign_census.py --check` passes on the claim base `509bd289` and on this branch after merging `efc88b85`:
  `spheres-sim/src/government.rs` and `docs/campaign-certification/C01/census.json` agree there, so no census failure is
  disclosed. This packet does not touch the census.
- **Checker-fix round:** `holder_name` is set to null in three extract rows (`to_la_pga_dpfi_established_sept2010`,
  `to_la_profile_dpfi_established_2010`, `to_ca_fidp_unincorporated_body_20150916`), whose sources name no holder of any
  office; their three snapshots in `tonga.json` and the pins in `test_tonga_dpfi_pdp_leaders_c01_44.py` are updated (the
  unmapped-name guard also covers ruling (b)'s singular 'Island' names). No claim text, source, holder or coverage entry
  changes. Rulings (a)-(c) are recorded above as decided by Ridge.
- **Integration branch moved (not merged):** `codex/campaign-certification` is at `13367c99`. It holds Codex's held partial
  review of this packet (`214121f9`, receipt `reviews/CLAUDE-C01-44-20261001/`) and Codex's registration of this handoff
  (`e7592644`). Merging it conflicts on `research-index.json` and, add/add, on `docs/planning/ai-handoffs/CLAUDE-C01-44.md`
  (Codex's registration record at the same path), so it is not merged in the fix round and the conflict is reported for a
  decision. The review's prose correction (`9955de17` on `codex/review-c01-44-20261001`, the paragraph on later primary
  records and the 2022 Helu presidency) is applied on this branch by a cherry-pick of that commit.
- **Known failures outside this packet (disclosed, not fixed):**
  - `tools/avatars/test_certified_gap_ledger.py` errors on this packet's new sources ("no pinned attribution") until Codex
    classifies the packet's commit in `COMMIT_PACKETS` at integration.
  - `tools/avatars/test_certified_boundary_matrix.py` (S23) needs `spheres-web/src`, which this sparse checkout lacks; Codex
    must list the packet and regenerate `docs/campaign-certification/S23/preparation/boundary-matrix/` on integration.

## Checks

```text
Run on 1 October 2026 (UTC; 30 September local) from the worktree root with PYTHONDONTWRITEBYTECODE=1, after merging
efc88b85. The sparse checkout was widened by docs/campaign-certification/C06 and S26 (22 small files), which the merged
workboard and Tonga cast checks read.

python -X utf8 tools/avatars/campaign_research.py            # regenerate
python -X utf8 tools/avatars/campaign_research.py --check    # pass: 9 packets, 844 organization and 36 institution
                                                             # observations, 1,976 sources, 4,882 claims, 93 batches
python -X utf8 tools/avatars/campaign_census.py --check      # pass (also on base 509bd289)
python -X utf8 -m unittest discover -s tools/avatars -p "test_country_cast.py" # 31 pass (merged C06 Tonga cast)
python -X utf8 -m unittest discover -s tools/avatars -p "test_tonga*.py"      # 101 pass (11 new)
python -X utf8 -m unittest discover -s tools/avatars -p "test_*research*.py"  # 79 pass
python -X utf8 -m unittest discover -s tools/avatars -p "test_campaign*.py"   # 16 pass, census tests included
node --test tools/ui/check_leadership_research_review.cjs                     # 11 pass
python tools/planning/workboard.py --check                                    # pass (44 markers)
git diff --check -- <this packet's paths>                                     # clean
python -X utf8 D:/spheres-scratch/c01-pipeline/tools/packet_check.py 44       # run on the pushed head; its summary is
                                                             # returned to the pipeline and saved in
                                                             # D:/spheres-scratch/verify/C01-44/packet_check.json

Known failures outside the listed checks (disclosed, not fixed):
test_certified_gap_ledger.py      # error: "Source to_la_news_pga_award_20131214 (Tonga) has no pinned attribution"
test_certified_boundary_matrix.py # error: "Required input is missing: spheres-web/src/person_avatar_assets.rs"

Checker-fix round (same date), from the worktree root with PYTHONDONTWRITEBYTECODE=1, on the fix tree without merging
13367c99 (see Integration notes):

python -X utf8 tools/avatars/campaign_research.py            # regenerate: only the tonga.json checksum changes
python -X utf8 tools/avatars/campaign_research.py --check    # pass: 9 packets, 844 organization and 36 institution
                                                             # observations, 1,976 sources, 4,882 claims, 93 batches
python -X utf8 tools/avatars/campaign_census.py --check      # pass
python -X utf8 -m unittest discover -s tools/avatars -p "test_tonga*.py"      # 101 pass
python -X utf8 -m unittest discover -s tools/avatars -p "test_*research*.py"  # 79 pass
python -X utf8 -m unittest discover -s tools/avatars -p "test_campaign*.py"   # 16 pass
python -X utf8 -m unittest discover -s tools/avatars -p "test_country_cast.py" # 31 pass
node --test tools/ui/check_leadership_research_review.cjs                     # 11 pass
python tools/planning/workboard.py --check                                    # pass (44 markers)
git diff --check                                                              # clean
python -X utf8 D:/spheres-scratch/c01-pipeline/tools/packet_check.py 44 --base origin/codex/campaign-certification --no-fetch
                                                             # run on the pushed fix head; summary returned to the pipeline
The two known failures above are unchanged in the fix round.
```
