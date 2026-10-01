# CPI(M) general secretaries 40: the party office, 1990-2026

Packet: **CLAUDE-C01-40**. State: **ready_for_review** (not complete).
Owner: Claude. Integrator/reviewer: Codex. Branch `claude/c01-in-40`; claimed at `c152e35e` on base `02d2c5a2`; not
stacked on a pending packet. After the packet commits (`c70c10eb`, `04019fb3`) the branch merged the moved integration
`79ef97ec` without conflict (merge `14250447`), so it is based on `codex/campaign-certification` at `79ef97ec`. Research access: 30 September 2026 (UTC); every
recorded response was first downloaded on that day, between 23:36:00Z and 23:47:22Z, which is each source's
`accessed_date`, and again at least 30 minutes later (see the identities section). The historical cutoff stays
**7 September 2026**.

This packet adds to [india.json](india.json) one party role, `in_cpm_general_secretary` ("General Secretary of the Communist
Party of India (Marxist)", kind `party_leader`), on the existing recognition observation `in_eci_20240323_np_04` (row 4 of
the Election Commission's national-party table of 23 March 2024). It adds 18 sources and 24 claims, all about the office,
four holder observations (one with a stated end), a role scope note, an observation coverage note and one note in the
packet coverage. The observation's identity, recognition row, `unresearched` lifecycle, `reporting_identity_only` coverage
status and empty game mapping are unchanged, and its existing source and claim stay first in its lists. It changes nothing
that CLAUDE-C01-11, -15, -20, -27 or -33 added: the thirteen prime-minister, eight president, ten Congress-president,
nineteen BJP-president and two Janata Dal-president holders are unchanged, and no existing source or extract is edited. The
party office and the `in_prime_minister` and `in_presidency` institutions stay separate both ways. The parent scope (C01,
C06, S23, WC1 and CP1) remains open.

## Outcome

| ID | Question | Decision |
|---|---|---|
| CPM-GS-01 | EMS Namboodiripad, proposed opening-era General Secretary | **Declined:** no party or official record reviewed attests him in the office on any day from 1 January 1990; the party website's undated caption page (captured 24 April 2001) calls him 'Former General Secretary, CPI(M)' (claim) |
| CPM-GS-02 | Harkishan Singh Surjeet (1992-2005) | **Accepted in part:** observed on 2 March 2004 (People's Democracy of 7 March 2004 prints the letter 'written by CPI(M) general secretary, Harkishan Singh Surjeet' on 'March 2'); claims: the party's 2005 bio-data ('Elected General Secretary ... in 1992 at the 14th Party Congress'), the 16th Congress election recalled for 11 October 1998, his own 'general secretary since 1992', the 'former' styling of 11 April 2005 and his undated handover statement |
| CPM-GS-03 | Prakash Karat (2005-2015) | **Accepted in part:** observed on 11 April 2005 (the 'newly-elected general secretary, Prakash Karat' at the rally concluding the 18th Congress); claims: the 18th Congress election (no day printed), the 'newly elected general secretary' who took the salute at Kozhikode on 9 April 2012, 'general secretary of the CPI(M)' inaugurating the 21st Congress on 14 April 2015 and 'the outgoing general secretary' on 19 April 2015; no end stated |
| CPM-GS-04 | Sitaram Yechury (2015-2024) | **Accepted:** observed on 19 April 2015 (the 'Newly elected general secretary, Sitaram Yechury' at the rally concluding the 21st Congress); **until 12 September 2024** from the Polit Bureau's statement on 'the passing away of Comrade Sitaram Yechury, General Secretary of the Party on September 12, 2024'; claims: the 21st Congress election of 19 April 2015 (names nobody), the Polit Bureau list, the 22nd (22 April 2018) and 23rd (10 April 2022) Congress elections and a retrospective sentence |
| CPM-GS-05 | The interim arrangement after his death | **Claims only:** the Central Committee's decision of 29 September 2024 that Prakash Karat 'will be the coordinator of the Polit Bureau and the Central Committee, as an interim arrangement until the 24th Party Congress', and his speech of 2 April 2025 as 'Coordinator'; never a holder of this office |
| CPM-GS-06 | M. A. Baby (2025-) | **Accepted in part:** observed on 12 May 2025 (the party's release: 'M. A. Baby, General Secretary ... has written a letter today'); claims: the 24th Congress election ('The Central Committee elected MA Baby as the General Secretary', 6 April 2025) and the memo of 24 August 2026 signed as General Secretary, the last record before the cutoff |

The resulting holder observations of `in_cpm_general_secretary`, in date order:

| Holder | `attested_on` | `from` | `until` | Basis |
|---|---|---|---|---|
| Harkishan Singh Surjeet | 2004-03-02 | null | null | People's Democracy, 7 March 2004: the letter 'written by CPI(M) general secretary, Harkishan Singh Surjeet to the Chief Election Commissioner on March 2' (raw Internet Archive capture of the party organ on cpim.org) |
| Prakash Karat | 2005-04-11 | null | null | People's Democracy, 17 April 2005: at the public meeting in New Delhi 'on April 11' Surjeet 'was escorted to the podium by the newly-elected general secretary, Prakash Karat' (raw capture of pd.cpim.org) |
| Sitaram Yechury | 2015-04-19 | null | 2024-09-12 | People's Democracy, 26 April 2015: the 'Newly elected general secretary, Sitaram Yechury, addressing the gathering' at the public meeting 'on April 19' (raw capture of peoplesdemocracy.in); the Polit Bureau's statement of 12 September 2024 (party website record) for the end |
| M. A. Baby | 2025-05-12 | null | null | The party's release of 12 May 2025: 'M. A. Baby, General Secretary of the Communist Party of India (Marxist) has written a letter today to the Prime Minister' (party website record) |

### How a start and an end are decided

The rule of the India party-leader packets (CLAUDE-C01-20, -27 and -33) applies. A holder has `from` only where a source
states the day the office was assumed, and `until` only where a source states the day it ended; otherwise the holder is
dated by `attested_on`, from a same-day in-office attestation in the party's own records that names the holder and styles
the office. No record reviewed states the day any General Secretary assumed the office. The Central Committee's election
of a General Secretary at a Party Congress, as the party records it, is an election claim and never a start (the
CLAUDE-C01-27 practice for organisational elections; see the ruling question below). The one end is Sitaram Yechury's
death in office: the Polit Bureau's own statement names the office and states the day, the ruling applied in CLAUDE-C01-34
(Brizola) and CLAUDE-C01-36 (Ma'afu).

Kept as claims that never feed a holder: elections (each Congress its own claim, dated where the source gives a day), later
attestations of a holder already observed, 'former' and 'outgoing' stylings, an undated handover statement, retrospective
biographies, captions and lists, and the interim coordinator arrangement. A successor's election or observation is never
used as a predecessor's end: Karat is styled 'outgoing' on the day Yechury is observed, and Surjeet 'former' on the day
Karat is observed, and neither has an `until`. Site 'Last modified' stamps, WordPress modification dates and archive
capture dates are never attestation dates; a post's publication date and a weekly issue's date are datelines, and a day
printed in a report's text is the event's day.

### Date ledger

Each row is a separate dated fact with its own claim. "Observed" marks a holder's `attested_on` claim and "end" its `until`
claim; every other claim never feeds a holder.

| Date | Events | Claims |
|---|---|---|
| 11 Oct 1998 | Surjeet: elected by the Central Committee at the 16th Congress (recalled in a 2001 page) | `in_cpim_site_cc_elected_surjeet_gs_recalled_19981011` (claim) |
| 2 Mar 2004 | Surjeet: in office (letter of that day) | `in_pd_surjeet_gs_letter_to_cec_20040302` (observed) |
| 11 Apr 2005 | Surjeet: 'former'; Karat: in office ('newly-elected general secretary') | `in_pd_rally_surjeet_former_gs_20050411` (claim), `in_pd_rally_newly_elected_gs_karat_20050411` (observed) |
| 9 Apr 2012 | Karat: in office again ('newly elected general secretary', 20th Congress) | `in_pd_rally_newly_elected_gs_karat_took_salute_20120409` (claim) |
| 14 Apr 2015 | Karat: in office again (inaugurates the 21st Congress) | `in_pd_karat_gs_inaugurated_21st_congress_20150414` (claim) |
| 19 Apr 2015 | 21st Congress: general secretary elected (names nobody); Karat: 'outgoing'; Yechury: in office | `in_pd_21st_congress_elected_general_secretary_20150419` (claim), `in_pd_rally_karat_outgoing_gs_20150419` (claim), `in_pd_rally_newly_elected_gs_yechury_20150419` (observed) |
| 22 Apr 2018 | Yechury: elected at the 22nd Congress | `in_cpim_cc_elected_yechury_gs_22nd_congress_20180422` (claim) |
| 10 Apr 2022 | Yechury: re-elected at the 23rd Congress | `in_cpim_cc_reelected_yechury_gs_23rd_congress_20220410` (claim) |
| 12 Sep 2024 | Yechury: death in office stated with the office | `in_cpim_pb_yechury_general_secretary_died_20240912` (end) |
| 29 Sep 2024 | Karat: interim coordinator decided | `in_cpim_cc_karat_coordinator_interim_arrangement_20240929` (claim) |
| 2 Apr 2025 | Karat: speech as 'Coordinator' | `in_cpim_karat_styled_coordinator_inaugural_speech_20250402` (claim) |
| 6 Apr 2025 | Baby: elected at the 24th Congress | `in_cpim_cc_elected_baby_gs_24th_congress_20250406` (claim) |
| 12 May 2025 | Baby: in office (letter of that day) | `in_cpim_baby_gs_letter_to_pm_20250512` (observed) |
| 24 Aug 2026 | Baby: in office again (memo) | `in_cpim_baby_gs_memo_census_20260824` (claim) |
| undated | EMS: 'Former General Secretary' caption; Surjeet: elected in 1992 at the 14th Congress (bio-data), 'general secretary since 1992', handover to Karat; Karat: elected at the 18th Congress; Yechury: Polit Bureau list (General Secretary), elected 'at the 21st Congress in 2015' | six claims |

## Observations

### CPM-GS-01 — EMS Namboodiripad

An opening-era holder is not established by this packet. No record reviewed attests EMS in the office on a day from
1 January 1990: the Rajya Sabha debates of 1990-1992 name him once, without the office (discovery only, below), the party's
1990-1991 records are not online, and the Welfare Ministry's resolution of 29 March 1991 (an existing CLAUDE-C01-33 source)
lists him without a designation. The party website's caption page, captured on 24 April 2001, reads 'Former General
Secretary, CPI(M)' and gives the day of his death in 1998; it is retrospective and undated, so he stays in the packet
through that claim only.

### CPM-GS-02 — Harkishan Singh Surjeet

The party's 18th Congress bio-data (captured 14 April 2005) says he was elected General Secretary 'in 1992 at the 14th
Party Congress held at Madras' and 'Continues in this position till date', and his own article of 17 April 2005 speaks of
'the general secretary since 1992'; both give a year, so no start is stored. The party website's 'Party Committees/Structure'
page, written after July 2001, recalls that the Central Committee elected him General Secretary on 11 October 1998 at the
end of the 16th Congress (a re-election claim). He is observed on 2 March 2004, when the party organ prints his letter to
the Chief Election Commissioner as 'CPI(M) general secretary'. On 11 April 2005 the organ calls him 'the former CPI(M)
general secretary', and his article says Prakash Karat 'has taken up the mantle as the new general secretary'; neither gives
the day he left the office, so he has no end.

### CPM-GS-03 — Prakash Karat

People's Democracy of 17 April 2005 reports his unanimous election by the new Central Committee of the 18th Congress without
a day (claim), and, at the public meeting of 11 April 2005, 'the newly-elected general secretary, Prakash Karat' escorting
Surjeet and addressing the crowd (observed). At Kozhikode on 9 April 2012 the 'newly elected general secretary' took the
volunteers' salute (the only 20th Congress record found; a re-election, not a new start). On 14 April 2015 he inaugurated
the 21st Congress as 'general secretary of the CPI(M)', and on 19 April 2015 he was 'the outgoing general secretary'; the
day he left the office is not stated, and Yechury's observation on that day is not used as his end.

### CPM-GS-04 — Sitaram Yechury

The 21st Congress elected 'The new Central Committee, Control Commission, Polit Bureau and general secretary' on 19 April
2015 (the report names nobody; the issue's Polit Bureau list begins '1. Sitaram Yechury (General Secretary)', undated), and
at the concluding public meeting that afternoon the 'Newly elected general secretary, Sitaram Yechury' addressed the
gathering (observed). The party's posts of 22 April 2018 and 10 April 2022 record the Central Committee electing and
re-electing him General Secretary at the 22nd and 23rd Congresses (claims; re-elections are not new starts). The Polit
Bureau's statement of 12 September 2024 records 'the passing away of Comrade Sitaram Yechury, General Secretary of the Party
on September 12, 2024', which gives the end.

### CPM-GS-05 — The interim coordinator arrangement

On 29 September 2024 the party reported that the Central Committee, 'now in session in New Delhi', had decided that Prakash
Karat 'will be the coordinator of the Polit Bureau and the Central Committee, as an interim arrangement until the 24th Party
Congress', because of the death of 'the sitting General Secretary'. On 2 April 2025 his inaugural speech at the 24th
Congress was published as that of the 'Coordinator'. The coordinator is not styled General Secretary; both are claims only.

### CPM-GS-06 — M. A. Baby

The party's post of 6 April 2025 says 'The Central Committee elected MA Baby as the General Secretary' (claim, dated by the
post's dateline). He is observed on 12 May 2025, when the party releases his letter 'written ... today' as General
Secretary, and styled again in the memo of 24 August 2026, the last record before the cutoff. No vacancy, resignation or
interim arrangement is recorded before 7 September 2026.

## Sources added

All 18 are the party's own records: the party website (`cpim.org`) and its weekly organ People's Democracy (on
`pd.cpim.org`, `cpim.org/pd/` and `peoplesdemocracy.in`). No news report, encyclopaedia or history site is recorded.

| Source | Record | Used for |
|---|---|---|
| `in_cpim_site_ems_caption_page_2001` | party website caption page `ems~2.htm`, capture of 24 Apr 2001 | CPM-GS-01 |
| `in_cpim_18th_congress_site_surjeet_biodata_2005` | 18th Congress bio-data of Surjeet, capture of 14 Apr 2005 | CPM-GS-02 |
| `in_cpim_site_party_committees_page_2001` | 'Party Committees/Structure', capture of 25 Sep 2001 | CPM-GS-02 |
| `in_pd_20040307_surjeet_letter_to_cec` | People's Democracy, 7 Mar 2004 (on cpim.org), capture of 28 Sep 2004 | CPM-GS-02 holder |
| `in_pd_20050417_new_polit_bureau` | People's Democracy, 17 Apr 2005, 'New Polit Bureau', capture of 23 Jun 2006 | CPM-GS-03 |
| `in_pd_20050417_rally_concludes_18th_congress` | People's Democracy, 17 Apr 2005, rally report, capture of 26 Apr 2005 | CPM-GS-02, CPM-GS-03 holder |
| `in_pd_20050417_surjeet_break_the_impasse` | People's Democracy, 17 Apr 2005, Surjeet's article, capture of 23 Jun 2006 | CPM-GS-02 |
| `in_pd_20120415_rally_concludes_20th_congress` | People's Democracy, 15 Apr 2012, rally report, capture of 12 Aug 2012 | CPM-GS-03 |
| `in_pd_20150426_join_to_bring_forth_change` | People's Democracy, 26 Apr 2015, 21st Congress report, capture of 29 Apr 2015 | CPM-GS-03, CPM-GS-04 holder |
| `in_pd_20150426_central_committee_elected_21st_congress` | People's Democracy, 26 Apr 2015, lists, capture of 29 Apr 2015 | CPM-GS-04 |
| `in_cpim_22nd_congress_new_cc_elected_20180422` | party post 5567, 22 Apr 2018 | CPM-GS-04 |
| `in_cpim_23rd_congress_new_cc_elected_20220410` | party post 6757, 10 Apr 2022 | CPM-GS-04 |
| `in_cpim_pb_homage_yechury_20240912` | party post 11608, Polit Bureau statement, 12 Sep 2024 | CPM-GS-04 end |
| `in_cpim_cc_karat_coordinator_interim_20240929` | party post 11626, 29 Sep 2024 | CPM-GS-05 |
| `in_cpim_karat_coordinator_inaugural_speech_20250402` | party post 11826, 2 Apr 2025 | CPM-GS-05 |
| `in_cpim_24th_congress_new_cc_elected_20250406` | party post 11931, 6 Apr 2025 | CPM-GS-06 |
| `in_cpim_gs_letter_to_pm_20250512` | party post 11982, 12 May 2025 | CPM-GS-06 holder |
| `in_cpim_memo_census_2027_20260824` | party post 12752, 24 Aug 2026 | CPM-GS-06 |

## Response identities and stability checks

The reviewer re-downloads every recorded response and compares its byte count and SHA-256. Every download used curl with
its default User-Agent, without `--compressed` and with no Accept-Encoding request header; every body was served with no
Content-Encoding, so each identity is the uncompressed body. Each source was downloaded at 23:36-23:47Z on 30 September
2026 and again after 00:22Z on 1 October 2026 (at least 30 minutes apart) with identical bytes; the times are in each
extract's provenance note, and `packet_check.py` downloaded each once more (see Checks).

- **Raw Internet Archive captures (10).** The `id_` form of captures made between 2001 and 2015, each addressed by its exact
  14-digit timestamp (so the archive serves it without a redirect) and replaying the archived bytes unchanged; none is
  served gzip. The live People's Democracy hosts add a per-request Cloudflare script (`pd.cpim.org`, `peoplesdemocracy.in`)
  or no longer serve these paths, so they are not recorded.
- **The party website's WordPress REST records (8).** `https://cpim.org/wp-json/wp/v2/posts/<id>`, the JSON record of each
  post (title, slug, publication and modification dates, body). The HTML page of every post ends with a LiteSpeed cache
  comment stamped with the request time, so two downloads a few seconds apart differ; the JSON record was identical across
  downloads. The current party record reports publication and modification dates before the cutoff (two older posts
  carry a modification stamp of 14 April 2024). These are not independently timestamped historical captures: the current
  record's metadata is the party's reported dateline, not proof that the retrieved text existed unchanged on that date.
  Acceptance is limited to the attributed statements and metadata now served; it does not authenticate every past
  revision or promote a post dateline into an effective term boundary.

## Leads not imported

- The party's release of 10 May 2025 (`https://cpim.org/wp-json/wp/v2/posts/11973`, 'A CPI(M) delegation consisting of
  General Secretary M.A. Baby ... met the three member Election Commission of India today'): two days earlier than the
  observation used, but it prints the name differently from the later records; a lead for the integrator.
- Surjeet's opening speech at the 18th Congress (`cpim.org/18cong/speeches/hks_opening.htm`, capture of 14 April 2005): it
  says 'Today, April 6' but never styles him General Secretary.
- The Human Resource Development resolutions of 14 October 1996 (`E-0298-1996-0190-10568`) and 29 September 1997 list
  'Shri Harkishan Singh Surjeet, Genl.Secy, CPI (M)' among the 'Presidents of all Major National Political Parties'; both are
  existing CLAUDE-C01-33 sources, and adding rows would edit their extracts, so they are left for a later packet (a list
  under a category heading was a claim in CLAUDE-C01-33 in any case).
- People's Democracy of 13 April 2008 (19th Congress, Coimbatore): its index is captured, but the articles
  (`0323_pd/04132008_1.htm`, `04132008_6.htm`) were captured only as HTTP 404 pages.
- Party biographies of the 18th Congress leadership (`cpim.org/18cong/lead/bio-pk.htm`, `bio-sry.htm`) and the party's
  migrated tribute pages to EMS and Surjeet (`/remembering-ems-namboodiripad/`, `/rich-tributes-surjeet/`): retrospective.
- The party's 2015-2026 statements styling the holders already observed (for example the press release of 8 May 2015 by
  'CPI(M) General Secretary Sitaram Yechury', post 4346) and the 2022 inaugural speech of 'General Secretary Comrade Sitaram
  Yechury' (post 6743): later attestations not needed for the observations.

## Sources attempted

- People's Democracy, live: `pd.cpim.org` redirects to `archive.peoplesdemocracy.in`, which did not connect; `pd.cpim.org`
  and `peoplesdemocracy.in` serve every page with a per-request Cloudflare script, so no live page is an identity.
- People's Democracy of 31 March 2002 (17th Congress, Hyderabad): the issue index is captured, but its articles under
  `2002/march31/` have no capture (HTTP 404), so the 17th Congress re-election has no record here.
- The Rajya Sabha debates of January 1990 to December 1992 (discovery only, never an identity): the 3,440 debate files of the
  Secretariat's data-service year listings (as listed by CLAUDE-C01-33) were downloaded, their text layers searched for
  Namboodiripad and Surjeet with 'Secretary', and the files deleted; no passage styles either General Secretary of the
  CPI(M).
- The Parliament Digital Library (`eparlib.sansad.in`, `eparlib.nic.in`): no connection on 30 September 2026.
- The Wayback Machine's CDX index answered HTTP 429 at times; the captures recorded were located when it answered.
- `cpim.org` HTML pages: per-request cache stamp (above); the site's search API was used for discovery only.

No site terms, licences or cookie banners were accepted, no challenge was attempted, no login was used and no User-Agent was
changed.

## Suggested next work orders

1. A party or official record attesting EMS Namboodiripad as General Secretary on a day in 1990-1991 (People's Democracy
   or The Marxist of those years, or the Lok Sabha debates when the Parliament Digital Library is reachable).
2. The day and record of the Central Committee's elections at the 14th (Madras, January 1992), 15th (Chandigarh, 1995),
   17th (Hyderabad, March 2002), 18th (New Delhi, April 2005) and 19th (Coimbatore, April 2008) Congresses, from People's
   Democracy captures or the party's own Congress pages.
3. The HRD resolutions of 1996 and 1997 as Surjeet attestations, if the integrator allows rows to be added to existing
   CLAUDE-C01-33 extracts.
4. Separate roles for other CPI(M) offices (Polit Bureau, parliamentary leaders) only from their own records, never inherited
   from this office.

## Integration notes (outside this packet's file boundary)

- **Batch order.** The user chose to start this batch of C01 research packets (CLAUDE-C01-38 to -41, running in parallel in
  other country files) before Codex's "continue existing claims first" roadmap line; the integrator may sequence them after
  the existing claims.
- **Existing files edited.** `india.json` is edited by appending only: 18 sources after CLAUDE-C01-33's, the role on
  `in_eci_20240323_np_04` (whose `roles` list was empty), the observation's sources, claims and one coverage note after its
  existing entries, and one note at the end of the packet coverage. No existing source or extract is edited.
- **Pinned tests re-expressed, never loosened.** `test_india_research_s10e.py` (85/315/623/6, a fourth organization role in
  observation order, C01-40 hosts and access date, the source order ending with this packet's sources);
  `test_india_prime_ministers_c01_11.py`, `test_india_presidents_c01_15.py`, `test_india_inc_presidents_c01_20.py`,
  `test_india_bjp_presidents_c01_27.py` and `test_india_janata_dal_presidents_c01_33.py` (the source order extended by this
  packet's sources, the party-leader list extended by `in_cpm_general_secretary` between the BJP and Congress roles, six
  roles, 13 coverage notes with the C01-33 note at index 11 and this packet's last, six role observations in the index; the
  C01-11 test's set of other packets' rows and the C01-11/15 disjointness checks also include this role's claims).
- **Shared generated file.** `docs/campaign-certification/C01/research-index.json` is the only file shared with the
  parallel packets; it is regenerated in its own commit and should be regenerated again after merging any of them.
- **Gap ledger.** `tools/avatars/test_certified_gap_ledger.py` reports 'no pinned attribution' for this packet's commits
  until Codex classifies them; disclosed, not fixed; `docs/campaign-certification/C01/gap-ledger/` is untouched.
- **S23 boundary matrix.** `tools/avatars/test_certified_boundary_matrix.py` needs `spheres-web/src`, which this sparse
  checkout lacks; Codex regenerates the matrix on integration (the India packet gains a role).
- **Ruling questions.** (1) Whether a Central Committee election recorded by the party on its day (6 April 2025, 22 April
  2018, 10 April 2022) should give `from`, since the CPI(M) has no separate assumption of charge; this packet keeps
  elections as claims, as CLAUDE-C01-27 did. (2) Whether the party website's WordPress REST record of a post is acceptable
  as the recorded identity where the HTML page differs per request. (3) Whether a 'newly elected general secretary' styling
  at the rally that closes a Congress (11 April 2005, 19 April 2015) is an in-office attestation of that day, as used here.

## Checks

Run from the worktree with `PYTHONDONTWRITEBYTECODE=1` on 1 October 2026 (UTC), before committing on base `02d2c5a2`, and
rerun after merging integration `79ef97ec` (all results unchanged except the census check, below):

```text
python -X utf8 tools/avatars/campaign_research.py
python -X utf8 tools/avatars/campaign_research.py --check
python -X utf8 tools/avatars/campaign_census.py --check
python -X utf8 -m unittest discover -s tools/avatars -p "test_india*.py"
python -X utf8 -m unittest discover -s tools/avatars -p "test_*research*.py"
python -X utf8 -m unittest discover -s tools/avatars -p "test_campaign*.py"
node --test tools/ui/check_leadership_research_review.cjs
python tools/planning/workboard.py --check
git diff --check
python -X utf8 D:/spheres-scratch/c01-pipeline/tools/packet_check.py 40 --no-tests
```

| Check | Result |
|---|---|
| `campaign_research.py`, then `--check` | passed: 9 country packets, 844 organization and 36 institution observations, 1,904 sources, 4,751 claims, 93 open discovery batches |
| `campaign_census.py --check` | passed on `02d2c5a2`; **fails after merging `79ef97ec`, inherited from integration:** 'C01 evidence differs: census.json', because integration commits `434abd50` and `7c6f112c` changed `spheres-sim/src/government.rs` without regenerating `docs/campaign-certification/C01/census.json`, which pins that file's SHA-256 (recorded `3f846b4b...`, now `4b0b82db...` on integration itself); not fixed here (outside the file boundary) |
| `test_india*.py` | 67 passed, including the 10 tests of `test_india_cpim_general_secretaries_c01_40.py` (its mutation test rejects 17 rule cases, a changed claim day and an extract row that disagrees with the packet) |
| `test_*research*.py` | 79 passed |
| `test_campaign*.py` | 16 passed |
| `check_leadership_research_review.cjs` | 11 passed |
| `workboard.py --check` | passed |
| `git diff --check` | clean |
| `packet_check.py 40 --no-tests` (third download of every source) | 18 of 18 sources match their recorded byte count and SHA-256 (the only problem reported was the then-uncommitted worktree); the full run is repeated after pushing |
| `test_certified_gap_ledger.py` (not required; disclosed) | fails until Codex classifies the new commits: 'Source in_cpim_site_ems_caption_page_2001 (India) has no pinned attribution'; not fixed, and the gap ledger is untouched |
| `test_certified_boundary_matrix.py` (S23; not required; disclosed) | cannot pass in this sparse checkout: 'Required input is missing: spheres-web/src/person_avatar_assets.rs'; Codex regenerates the S23 boundary matrix when it merges |
